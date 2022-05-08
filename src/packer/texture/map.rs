use crate::prelude::{
    PositionedRect, Rect, Target, TextureAtlas, TextureAtlasBuilder, TextureAtlasFile,
    TexturePosition,
};
use image::RgbaImage;
use serde::{Deserialize, Serialize};
use std::{
    collections::{BinaryHeap, HashMap},
    hash::Hash,
    ops::Deref,
};

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

#[derive(Debug, Clone)]
pub struct TextureAtlasMapBuilder<K> {
    // side length limit
    limit: u16,

    images: BinaryHeap<SortBySize<K>>,
}

#[derive(Debug)]
pub struct TextureAtlasMap<K>
where
    K: Eq + Hash + Clone,
{
    inner: TextureAtlas,
    map: HashMap<K, TexturePosition>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TextureAtlasMapFile<K>
where
    K: Eq + Hash + Clone,
{
    inner: TextureAtlasFile,
    map: HashMap<K, TexturePosition>,
}

impl<K> Default for TextureAtlasMapBuilder<K> {
    fn default() -> Self {
        Self {
            images: Default::default(),
            limit: u16::MAX,
        }
    }
}

impl<K> TextureAtlasMapBuilder<K> {
    pub fn new() -> Self {
        Self::default()
    }

    /// side length limit
    pub fn with_limit(mut self, limit: u16) -> Self {
        self.limit = limit;
        self
    }

    pub fn with(mut self, key: K, image: RgbaImage) -> Self {
        self.insert(key, image);
        self
    }

    pub fn insert(&mut self, key: K, image: RgbaImage) {
        self.images.push(SortBySize { key, image });
    }
}

impl<K> TextureAtlasMapBuilder<K>
where
    K: Eq + Hash + Clone,
{
    pub fn build(mut self, target: &Target) -> TextureAtlasMap<K> {
        let mut builder = TextureAtlasBuilder::new();
        let mut images = vec![];

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

        let inner = builder.build(target, iter.into_iter());
        let size = inner.dim();
        let map = map
            .into_iter()
            .map(|(key, rect)| (key, TexturePosition::new(size, rect)))
            .collect();

        TextureAtlasMap { inner, map }
    }
}

impl<K> TextureAtlasMap<K>
where
    K: Eq + Hash + Clone,
{
    pub async fn convert(&self, target: &Target) -> TextureAtlasMapFile<K> {
        let inner = self.inner.convert(target).await;
        let map = self.map.clone();

        TextureAtlasMapFile { inner, map }
    }

    pub fn get(&self, key: &K) -> Option<&TexturePosition> {
        self.map.get(key)
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

impl<K> TextureAtlasMapFile<K>
where
    K: Eq + Hash + Clone,
{
    pub fn convert(&self, target: &Target) -> TextureAtlasMap<K> {
        let inner = self.inner.convert(target);
        let map = self.map.clone();

        TextureAtlasMap { inner, map }
    }
}
