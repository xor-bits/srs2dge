use super::{
    packer2d::Packer,
    rect::{PositionedRect, Rect},
};
use crate::{
    target::Target,
    texture::{serde::SerializeableTexture, Texture},
};
use image::RgbaImage;
use serde::{Deserialize, Serialize};
use std::{any::type_name, ops::Deref};
use wgpu::TextureUsages;

//

pub use map::*;

//

mod map;

//

const USAGE: u32 = TextureUsages::TEXTURE_BINDING.bits()
    | TextureUsages::COPY_SRC.bits()
    | TextureUsages::COPY_DST.bits();

//

#[derive(Debug, Clone)]
pub struct TextureAtlasBuilder {
    packer: Packer,

    // side length limit
    limit: u16,

    padding: u8,

    label: Option<String>,
}

#[derive(Debug)]
pub struct TextureAtlas {
    texture: Texture<USAGE>,
    label: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct SerializeableTextureAtlas {
    inner: SerializeableTexture,
}

//

pub trait Reference<T> {
    fn reference(&self) -> &'_ T;
}

//

impl<T> Reference<T> for T {
    fn reference(&self) -> &'_ T {
        self
    }
}

impl<T> Reference<T> for &T {
    fn reference(&self) -> &'_ T {
        self
    }
}

impl<'a> Default for TextureAtlasBuilder {
    fn default() -> Self {
        let mut packer = Packer::default();
        let padding = 2;
        packer.padding = padding;
        Self {
            packer,
            limit: u16::MAX,
            padding,
            label: None,
        }
    }
}

impl TextureAtlasBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    /* pub fn new_sized(width: u32, height: u32) -> Self {
        Self {
            packer: Packer::new(Rect { width, height }),
            ..Default::default()
        }
    } */

    /// side length limit
    pub fn with_limit(mut self, limit: u16) -> Self {
        self.limit = limit;
        self
    }

    /// texture padding
    pub fn with_padding(mut self, padding: u8) -> Self {
        self.padding = padding;
        self.packer.padding = padding;
        self
    }

    pub fn with_label(mut self, label: Option<String>) -> Self {
        self.label = label;
        self
    }

    pub fn push(&mut self, rect: Rect) -> Option<PositionedRect> {
        self.packer.push_until(rect, self.limit)
    }

    pub fn build<I, R>(self, target: &Target, iter: I) -> TextureAtlas
    where
        R: Reference<RgbaImage>,
        I: IntoIterator<Item = (R, PositionedRect)>,
    {
        let dim = self.packer.area();
        let label = self.label;

        // combine all images into one
        let mut combined = RgbaImage::new(dim.width, dim.height);
        for (image, pos) in iter {
            for (xo, yo, pixel) in image.reference().enumerate_pixels() {
                combined.put_pixel(pos.x + xo, pos.y + yo, *pixel);
            }
        }

        let texture = Texture::new_rgba_with(
            target,
            &combined,
            Some(
                label
                    .as_ref()
                    .map(|s| s.as_str())
                    .unwrap_or(type_name::<Self>()),
            ),
        );

        TextureAtlas { texture, label }
    }
}

impl TextureAtlas {
    pub fn builder() -> TextureAtlasBuilder {
        TextureAtlasBuilder::new()
    }

    pub async fn download(&self, target: &Target) -> SerializeableTextureAtlas {
        let inner = self.texture.download(target, self.label.clone()).await;
        SerializeableTextureAtlas { inner }
    }

    pub fn upload(from: &SerializeableTextureAtlas, target: &Target) -> Self {
        from.upload(target)
    }
}

impl SerializeableTextureAtlas {
    pub async fn download(from: &TextureAtlas, target: &Target) -> Self {
        from.download(target).await
    }

    pub fn upload(&self, target: &Target) -> TextureAtlas {
        let texture = self.inner.upload(target);
        let label = self.inner.label.clone();
        TextureAtlas { texture, label }
    }
}

impl Deref for TextureAtlas {
    type Target = Texture<USAGE>;

    fn deref(&self) -> &Self::Target {
        &self.texture
    }
}
