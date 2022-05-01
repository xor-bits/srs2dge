use super::{
    packer2d::Packer,
    rect::{PositionedRect, Rect},
};
use crate::{label, target::Target};
use image::RgbaImage;
use rapid_qoi::{Colors, Qoi};
use serde::{de::Visitor, Deserialize, Deserializer, Serialize, Serializer};
use std::{mem, num::NonZeroU32, ops::Deref};
use wgpu::{
    util::DeviceExt, BufferDescriptor, BufferUsages, CommandEncoderDescriptor, Extent3d,
    ImageCopyBuffer, ImageDataLayout, MapMode, Texture, TextureDescriptor, TextureDimension,
    TextureFormat, TextureUsages, TextureView, TextureViewDescriptor,
};

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
    texture: Texture,
    view: TextureView,
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

    pub fn build<I, R>(self, target: &Target, iter: I) -> TextureAtlas
    where
        R: Reference<RgbaImage>,
        I: Iterator<Item = (R, PositionedRect)>,
    {
        let dim = self.packer.area();

        // combine all images into one
        let mut combined = RgbaImage::new(dim.width, dim.height);
        for (image, pos) in iter {
            for (xo, yo, pixel) in image.reference().enumerate_pixels() {
                combined.put_pixel(pos.x + xo, pos.y + yo, *pixel);
            }
        }

        let texture = target.device.create_texture_with_data(
            &target.queue,
            &TextureDescriptor {
                label: label!(),
                size: Extent3d {
                    width: dim.width,
                    height: dim.height,
                    depth_or_array_layers: 1,
                },
                mip_level_count: 1,
                sample_count: 1,
                dimension: TextureDimension::D2,
                format: TextureFormat::Rgba8UnormSrgb,
                usage: TextureUsages::TEXTURE_BINDING
                    | TextureUsages::RENDER_ATTACHMENT
                    | TextureUsages::COPY_SRC,
            },
            combined.as_raw(),
        );

        let view = texture.create_view(&TextureViewDescriptor::default());

        TextureAtlas { texture, view, dim }
    }
}

impl TextureAtlas {
    pub async fn convert(&self, target: &Target) -> TextureAtlasFile {
        let dim = BufferDimensions::new(self.dim.width as _, self.dim.height as _);
        let read_buffer = target.device.create_buffer(&BufferDescriptor {
            label: label!(),
            size: (dim.padded_bytes_per_row * dim.height) as u64,
            usage: BufferUsages::MAP_READ | BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        let mut encoder = target
            .device
            .create_command_encoder(&CommandEncoderDescriptor { label: label!() });

        encoder.copy_texture_to_buffer(
            self.texture.as_image_copy(),
            ImageCopyBuffer {
                buffer: &read_buffer,
                layout: ImageDataLayout {
                    offset: 0,
                    bytes_per_row: Some(NonZeroU32::new(dim.padded_bytes_per_row as _).unwrap()),
                    rows_per_image: None,
                },
            },
            Extent3d {
                width: self.dim.width,
                height: self.dim.height,
                depth_or_array_layers: 1,
            },
        );

        target.queue.submit([encoder.finish()]);

        let range = read_buffer.slice(..);
        range.map_async(MapMode::Read).await.unwrap();

        let bytes = range
            .get_mapped_range()
            .chunks(dim.padded_bytes_per_row)
            .flat_map(|s| &s[..dim.unpadded_bytes_per_row])
            .copied()
            .collect();

        let image = RgbaImage::from_raw(dim.width as _, dim.height as _, bytes).unwrap();

        TextureAtlasFile { image }
    }

    pub fn dimensions(&self) -> Rect {
        self.dim
    }
}

impl Deref for TextureAtlas {
    type Target = TextureView;

    fn deref(&self) -> &Self::Target {
        &self.view
    }
}

impl TextureAtlasFile {
    pub fn convert(&self, target: &Target) -> TextureAtlas {
        let dim: Rect = self.image.dimensions().into();
        let texture = target.device.create_texture_with_data(
            &target.queue,
            &TextureDescriptor {
                label: label!(),
                size: dim.into(),
                mip_level_count: 1,
                sample_count: 1,
                dimension: TextureDimension::D2,
                format: TextureFormat::Rgba8UnormSrgb,
                usage: TextureUsages::TEXTURE_BINDING
                    | TextureUsages::RENDER_ATTACHMENT
                    | TextureUsages::COPY_SRC,
            },
            self.image.as_raw(),
        );

        let view = texture.create_view(&TextureViewDescriptor::default());

        TextureAtlas { texture, view, dim }
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
        let image = deserializer.deserialize_bytes(V)?;

        struct V;

        impl<'de> Visitor<'de> for V {
            type Value = &'de [u8];

            fn expecting(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                write!(f, "expected byte array")
            }
        }

        let (qoi, image) = Qoi::decode_alloc(image).map_err(serde::de::Error::custom)?;

        Ok(Self {
            image: RgbaImage::from_raw(qoi.width, qoi.height, image).unwrap(),
        })
    }
}

//

struct BufferDimensions {
    width: usize,
    height: usize,
    unpadded_bytes_per_row: usize,
    padded_bytes_per_row: usize,
}

impl BufferDimensions {
    fn new(width: usize, height: usize) -> Self {
        let unpadded_bytes_per_row = width * mem::size_of::<u32>();
        let align = wgpu::COPY_BYTES_PER_ROW_ALIGNMENT as usize;
        let padded_bytes_per_row_padding = (align - unpadded_bytes_per_row % align) % align;
        let padded_bytes_per_row = unpadded_bytes_per_row + padded_bytes_per_row_padding;
        Self {
            width,
            height,
            unpadded_bytes_per_row,
            padded_bytes_per_row,
        }
    }
}
