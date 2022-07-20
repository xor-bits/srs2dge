use super::{
    packer2d::Packer,
    rect::{PositionedRect, Rect},
};
use crate::{target::Target, texture::Texture};
use image::RgbaImage;
use rapid_qoi::{Colors, Qoi};
use serde::{de::Visitor, Deserialize, Deserializer, Serialize, Serializer};
use std::ops::Deref;
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
}

#[derive(Debug)]
pub struct TextureAtlas {
    texture: Texture<USAGE>,
}

#[derive(Debug, Clone)]
pub struct TextureAtlasFile {
    image: RgbaImage,
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

impl Default for TextureAtlasBuilder {
    fn default() -> Self {
        let mut packer = Packer::default();
        let padding = 2;
        packer.padding = padding;
        Self {
            packer,
            limit: u16::MAX,
            padding,
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

    pub fn push(&mut self, rect: Rect) -> Option<PositionedRect> {
        self.packer.push_until(rect, self.limit)
    }

    pub fn build<I, R>(self, target: &Target, iter: I) -> TextureAtlas
    where
        R: Reference<RgbaImage>,
        I: IntoIterator<Item = (R, PositionedRect)>,
    {
        let dim = self.packer.area();

        // combine all images into one
        let mut combined = RgbaImage::new(dim.width, dim.height);
        for (image, pos) in iter {
            for (xo, yo, pixel) in image.reference().enumerate_pixels() {
                combined.put_pixel(pos.x + xo, pos.y + yo, *pixel);
            }
        }

        let texture = Texture::new_rgba_with(target, &combined);

        TextureAtlas { texture }
    }
}

impl TextureAtlas {
    pub fn builder() -> TextureAtlasBuilder {
        TextureAtlasBuilder::new()
    }

    pub async fn convert(&self, target: &Target) -> TextureAtlasFile {
        let image = self.texture.read(target).await.into_rgba8();
        TextureAtlasFile { image }
    }
}

impl Deref for TextureAtlas {
    type Target = Texture<USAGE>;

    fn deref(&self) -> &Self::Target {
        &self.texture
    }
}

impl TextureAtlasFile {
    pub fn convert(&self, target: &Target) -> TextureAtlas {
        let texture = Texture::new_rgba_with(target, &self.image);
        TextureAtlas { texture }
    }
}

impl Serialize for TextureAtlasFile {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let qoi = Qoi {
            width: self.image.width(),
            height: self.image.height(),
            colors: Colors::Rgba,
        };

        let image = qoi
            .encode_alloc(self.image.as_raw())
            .map_err(serde::ser::Error::custom)?;

        serializer.serialize_bytes(&image)
    }
}

impl<'de> Deserialize<'de> for TextureAtlasFile {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let image = deserializer.deserialize_byte_buf(V)?;

        struct V;

        impl<'de> Visitor<'de> for V {
            type Value = Vec<u8>;

            fn expecting(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                write!(f, "byte array")
            }

            fn visit_bytes<E>(self, v: &[u8]) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(v.to_owned())
            }

            fn visit_byte_buf<E>(self, v: Vec<u8>) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(v)
            }
        }

        let (qoi, image) = Qoi::decode_alloc(&image).map_err(serde::de::Error::custom)?;

        Ok(Self {
            image: RgbaImage::from_raw(qoi.width, qoi.height, image).unwrap(),
        })
    }
}
