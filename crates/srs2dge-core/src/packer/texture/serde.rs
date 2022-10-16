use super::{TextureAtlas, TextureAtlasMap};
use crate::{
    packer::prelude::Rect,
    prelude::Target,
    texture::{pos::TexturePosition, serde::SerializeableTexture},
};
use image::RgbaImage;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, hash::Hash};

//

/// A texture atlas handler.
///
/// This is headless, and is serializeable.
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct SerializeableTextureAtlas {
    inner: SerializeableTexture,
}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct SerializeableTextureAtlasMap<K>
where
    K: Eq + Hash + Clone,
{
    inner: SerializeableTextureAtlas,
    map: HashMap<K, TexturePosition>,
}

//

impl SerializeableTextureAtlas {
    pub fn new(image: RgbaImage, label: Option<String>) -> Self {
        Self {
            inner: SerializeableTexture::new(image, label),
        }
    }

    pub fn split(self) -> (RgbaImage, Option<String>) {
        self.inner.split()
    }

    pub async fn download(from: &TextureAtlas, target: &Target) -> Self {
        from.download(target).await
    }

    pub fn upload(&self, target: &Target) -> TextureAtlas {
        let texture = self.inner.upload(target);
        let label = self.inner.label.clone();
        TextureAtlas { texture, label }
    }

    pub fn get_dim(&self) -> Rect {
        self.inner.get_dim()
    }
}

impl<K> SerializeableTextureAtlasMap<K>
where
    K: Eq + Hash + Clone,
{
    pub fn new(image: RgbaImage, map: HashMap<K, TexturePosition>, label: Option<String>) -> Self {
        Self {
            inner: SerializeableTextureAtlas::new(image, label),
            map,
        }
    }

    pub async fn download(from: &TextureAtlasMap<K>, target: &Target) -> Self {
        from.download(target).await
    }

    pub fn upload(&self, target: &Target) -> TextureAtlasMap<K> {
        let inner = self.inner.upload(target);
        let map = self.map.clone();
        TextureAtlasMap { inner, map }
    }
}

impl TextureAtlas {
    pub async fn download(&self, target: &Target) -> SerializeableTextureAtlas {
        let inner = self.texture.download(target, self.label.clone()).await;
        SerializeableTextureAtlas { inner }
    }

    pub fn upload(from: &SerializeableTextureAtlas, target: &Target) -> Self {
        from.upload(target)
    }
}

impl<K> TextureAtlasMap<K>
where
    K: Eq + Hash + Clone,
{
    pub async fn download(&self, target: &Target) -> SerializeableTextureAtlasMap<K> {
        let inner = self.inner.download(target).await;
        let map = self.map.clone();
        SerializeableTextureAtlasMap { inner, map }
    }

    pub fn upload(from: &SerializeableTextureAtlasMap<K>, target: &Target) -> Self {
        from.upload(target)
    }
}
