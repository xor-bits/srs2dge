use super::SerializeableTextureAtlasMap;
use crate::prelude::{
    PositionedRect, Rect, Target, TextureAtlas, TextureAtlasBuilder, TexturePosition,
};
use image::{load_from_memory, ImageResult, RgbaImage};
use std::{
    collections::{BinaryHeap, HashMap},
    hash::Hash,
    ops::Deref,
};

//

/// A builder for the mapped texture atlas.
///
/// A helper struct to find [`TexturePosition`]:s
/// stored in this texture atlas by their keys.
#[derive(Debug, Clone)]
pub struct TextureAtlasMapBuilder<K> {
    /// texture side length limit
    limit: u16,

    /// a padding for each texture
    padding: u8,

    /// all images before combining
    images: BinaryHeap<SortBySize<K>>,

    /// optional label used for debugging
    label: Option<String>,
}

/// A texture atlas map handler.
///
/// This is on the GPU and is not serializeable.
#[derive(Debug)]
pub struct TextureAtlasMap<K>
where
    K: Eq + Hash + Clone,
{
    pub(super) inner: TextureAtlas,
    pub(super) map: HashMap<K, TexturePosition>,
}

//

impl<K> Default for TextureAtlasMapBuilder<K> {
    fn default() -> Self {
        Self {
            images: Default::default(),
            limit: u16::MAX,
            padding: 2,
            label: None,
        }
    }
}

impl<K> TextureAtlasMapBuilder<K> {
    pub fn new() -> Self {
        Self::default()
    }

    /// texture side length limit
    pub fn with_limit(mut self, limit: u16) -> Self {
        self.limit = limit;
        self
    }

    /// a padding for each texture
    pub fn with_padding(mut self, padding: u8) -> Self {
        self.padding = padding;
        self
    }

    /// a label for the resulting texture
    ///
    /// used in debugging
    pub fn with_label(mut self, label: Option<String>) -> Self {
        self.label = label;
        self
    }

    /// insert a new image and its key
    /// to this texture atlas map builder
    pub fn with(mut self, key: K, image: RgbaImage) -> Self {
        self.insert(key, image);
        self
    }

    /// insert a new image and its key
    /// to this texture atlas map builder
    pub fn with_bytes(self, key: K, image_bytes: &[u8]) -> ImageResult<Self> {
        Ok(self.with(key, load_from_memory(image_bytes)?.to_rgba8()))
    }

    /// insert a new image and its key
    /// to this texture atlas map builder
    pub fn insert(&mut self, key: K, image: RgbaImage) {
        self.images.push(SortBySize { key, image });
    }
}

impl<K> TextureAtlasMapBuilder<K>
where
    K: Eq + Hash + Clone,
{
    /// build the texture atlas map headlessly
    pub fn build_serializeable(mut self) -> SerializeableTextureAtlasMap<K> {
        let mut builder = TextureAtlasBuilder::new()
            .with_padding(self.padding)
            .with_limit(self.limit)
            .with_label(self.label);
        let mut images = vec![];

        // sort images by size for (more) optimal packing
        while let Some(SortBySize { key, image }) = self.images.pop() {
            let (width, height) = image.dimensions();

            let v = builder
                .push(Rect { width, height })
                .expect("Texture atlas limit reached");
            images.push((key, v, image));
        }

        type Map<K> = Vec<(K, PositionedRect)>;
        type Iter = Vec<(RgbaImage, PositionedRect)>;
        let (map, iter): (Map<K>, Iter) = images
            .into_iter()
            .map(|(key, pos, img)| ((key, pos), (img, pos)))
            .unzip();

        let inner = builder.build_serializeable(iter);
        let size = inner.get_dim();
        let map = map
            .into_iter()
            .map(|(key, rect)| (key, TexturePosition::new(size, rect)))
            .collect();
        let (image, label) = inner.split();

        SerializeableTextureAtlasMap::new(image, map, label)
    }

    /// build the texture atlas map and
    /// upload it to the GPU
    pub fn build(self, target: &Target) -> TextureAtlasMap<K> {
        self.build_serializeable().upload(target)
    }
}

impl<K> TextureAtlasMap<K>
where
    K: Eq + Hash + Clone,
{
    pub fn builder() -> TextureAtlasMapBuilder<K> {
        TextureAtlasMapBuilder::new()
    }

    pub fn get(&self, key: &K) -> Option<TexturePosition> {
        self.map.get(key).copied()
    }
}

impl<K> Deref for TextureAtlasMap<K>
where
    K: Eq + Hash + Clone,
{
    type Target = TextureAtlas;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

//

#[derive(Debug, Clone)]
struct SortBySize<K> {
    key: K,
    image: RgbaImage,
}

impl<K> SortBySize<K> {
    fn size(&self) -> u64 {
        let (w, h) = self.image.dimensions();
        w as u64 * h as u64
    }
}

impl<K> PartialEq for SortBySize<K> {
    fn eq(&self, other: &Self) -> bool {
        self.size() == other.size()
    }
}

impl<K> Eq for SortBySize<K> {}

impl<K> PartialOrd for SortBySize<K> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.size().partial_cmp(&other.size())
    }
}

impl<K> Ord for SortBySize<K> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.size().cmp(&other.size())
    }
}
