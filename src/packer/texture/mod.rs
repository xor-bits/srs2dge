use super::packer2d::{Packer, PositionedRect, Rect};
use glium::{backend::Facade, texture::RawImage2d, uniforms::Sampler, Texture2d};
use image::RgbaImage;
use rapid_qoi::{Colors, Qoi};
use serde::{de::Visitor, Deserialize, Deserializer, Serialize, Serializer};

//

pub use map::*;

//

mod map;

//

#[derive(Debug, Clone)]
pub struct TextureAtlasBuilder {
    packer: Packer,

    // side length limit
    limit: u16,
}

#[derive(Debug)]
pub struct TextureAtlas {
    texture: Texture2d,
    dim: Rect,
}

#[derive(Debug, Clone)]
pub struct TextureAtlasFile {
    image: RgbaImage,
}

pub trait Reference<T> {
    fn reference(&self) -> &'_ T;
}

impl<T> Reference<T> for T {
    fn reference(&self) -> &'_ T {
        self
    }
}

impl<T> Reference<T> for &T {
    fn reference(&self) -> &'_ T {
        *self
    }
}

impl Default for TextureAtlasBuilder {
    fn default() -> Self {
        Self {
            packer: Default::default(),
            limit: u16::MAX,
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

    pub fn push(&mut self, rect: Rect) -> Option<PositionedRect> {
        self.packer.push_until(rect, self.limit)
    }

    pub fn build<'i, I, R, F>(self, facade: &F, iter: I) -> TextureAtlas
    where
        R: Reference<RgbaImage>,
        I: Iterator<Item = (R, PositionedRect)>,
        F: Facade,
    {
        let dim = self.packer.area();

        // combine all images into one
        let mut combined = RgbaImage::new(dim.width, dim.height);
        for (image, pos) in iter {
            for (xo, yo, pixel) in image.reference().enumerate_pixels() {
                combined.put_pixel(pos.x + xo, pos.y + yo, *pixel);
            }
        }

        let image = RawImage2d::from_raw_rgba_reversed(combined.as_raw(), combined.dimensions());
        let texture = Texture2d::new(facade, image).unwrap();

        TextureAtlas { texture, dim }
    }
}

impl TextureAtlas {
    pub fn convert(&self) -> TextureAtlasFile {
        let image: RawImage2d<u8> = self.texture.read();
        let image =
            RgbaImage::from_raw(image.width, image.height, image.data.into_owned()).unwrap();

        TextureAtlasFile { image }
    }

    pub fn sampled(&self) -> Sampler<'_, Texture2d> {
        self.texture.sampled()
    }

    pub fn dimensions(&self) -> Rect {
        self.dim
    }
}

impl TextureAtlasFile {
    pub fn convert<F>(&self, facade: &F) -> TextureAtlas
    where
        F: Facade,
    {
        let dim = self.image.dimensions();
        let image = RawImage2d::from_raw_rgba_reversed(self.image.as_raw(), dim);
        let texture = Texture2d::new(facade, image).unwrap();
        let (width, height) = dim;
        let dim = Rect { width, height };

        TextureAtlas { texture, dim }
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
            .map_err(|err| serde::ser::Error::custom(err))?;

        serializer.serialize_bytes(&image)
    }
}

impl<'de> Deserialize<'de> for TextureAtlasFile {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let image = deserializer.deserialize_bytes(V)?;

        struct V;

        impl<'de> Visitor<'de> for V {
            type Value = &'de [u8];

            fn expecting(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                write!(f, "expected byte array")
            }
        }

        let (qoi, image) =
            Qoi::decode_alloc(&image).map_err(|err| serde::de::Error::custom(err))?;

        Ok(Self {
            image: RgbaImage::from_raw(qoi.width, qoi.height, image).unwrap(),
        })
    }
}
