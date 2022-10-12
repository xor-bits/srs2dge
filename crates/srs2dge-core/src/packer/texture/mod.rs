use self::serde::SerializeableTextureAtlas;
use super::{
    packer2d::Packer,
    rect::{PositionedRect, Rect},
};
use crate::{target::Target, texture::Texture};
use image::RgbaImage;
use std::{borrow::Borrow, ops::Deref};
use wgpu::TextureUsages;

//

pub use self::serde::*;
pub use map::*;

//

mod map;
mod serde;

//

const USAGE: u32 = TextureUsages::TEXTURE_BINDING.bits()
    | TextureUsages::COPY_SRC.bits()
    | TextureUsages::COPY_DST.bits();

//

/// A builder for the texture atlas.
#[derive(Debug, Clone)]
pub struct TextureAtlasBuilder {
    /// the inner packer to pack
    /// texture ([`Rect`]:s) with
    packer: Packer,

    /// texture side length limit
    limit: u16,

    /// a padding for each texture
    padding: u8,

    /// optional label used for debugging
    label: Option<String>,
}

/// A texture atlas handler.
///
/// This is on the GPU and is not serializeable.
#[derive(Debug)]
pub struct TextureAtlas {
    texture: Texture<USAGE>,
    label: Option<String>,
}

//

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

    /// texture side length limit
    pub fn with_limit(mut self, limit: u16) -> Self {
        self.limit = limit;
        self
    }

    /// a padding for each texture
    pub fn with_padding(mut self, padding: u8) -> Self {
        self.padding = padding;
        self.packer.padding = padding;
        self
    }

    /// a label for the resulting texture
    ///
    /// used in debugging
    pub fn with_label(mut self, label: Option<String>) -> Self {
        self.label = label;
        self
    }

    /// push a new image (its size)
    /// to this texture atlas builder
    pub fn push(&mut self, rect: Rect) -> Option<PositionedRect> {
        self.packer.push_until(rect, self.limit)
    }

    /// build the texture atlas headlessly
    pub fn build_serializeable<I, R>(self, iter: I) -> SerializeableTextureAtlas
    where
        R: Borrow<RgbaImage>,
        I: IntoIterator<Item = (R, PositionedRect)>,
    {
        let dim = self.packer.area();
        let label = self.label;

        // combine all images into one
        let mut combined = RgbaImage::new(dim.width, dim.height);
        for (image, pos) in iter {
            for (xo, yo, pixel) in image.borrow().enumerate_pixels() {
                combined.put_pixel(pos.x + xo, pos.y + yo, *pixel);
            }
        }

        SerializeableTextureAtlas::new(combined, label)
    }

    /// build the texture atlas and upload
    /// it to the GPU
    pub fn build<I, R>(self, target: &Target, iter: I) -> TextureAtlas
    where
        R: Borrow<RgbaImage>,
        I: IntoIterator<Item = (R, PositionedRect)>,
    {
        self.build_serializeable(iter).upload(&target)
    }
}

impl TextureAtlas {
    pub fn builder() -> TextureAtlasBuilder {
        TextureAtlasBuilder::new()
    }
}

impl Deref for TextureAtlas {
    type Target = Texture<USAGE>;

    fn deref(&self) -> &Self::Target {
        &self.texture
    }
}
