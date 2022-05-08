use crate::{
    label,
    prelude::{PositionedRect, Rect},
    target::Target,
};
use image::{DynamicImage, GrayImage, RgbaImage};
use std::{mem, num::NonZeroU32, ops::Deref};
use wgpu::{
    util::DeviceExt, BufferDescriptor, BufferUsages, CommandEncoderDescriptor, Extent3d,
    ImageCopyBuffer, ImageCopyTexture, ImageDataLayout, MapMode, Origin3d, TextureAspect,
    TextureDescriptor, TextureDimension, TextureFormat, TextureUsages, TextureView,
};

//

pub mod pos;
pub mod prelude;

//

const DEFAULT_USAGE: u32 = TextureUsages::TEXTURE_BINDING.bits();

//

#[derive(Debug)]
pub struct Texture<const USAGE: u32 = DEFAULT_USAGE> {
    texture: wgpu::Texture,
    format: TextureFormat,
    view: TextureView,
    dim: Rect,
}

//

impl<const USAGE: u32> Texture<USAGE> {
    pub fn new(target: &Target, format: TextureFormat, dim: Rect) -> Self {
        Self::new_inner(target, format, dim, None)
    }

    pub fn new_rgba(target: &Target, dim: Rect) -> Self {
        Self::new_inner(target, TextureFormat::Rgba8Unorm, dim, None)
    }

    pub fn new_rgba_with(target: &Target, data: &RgbaImage) -> Self {
        Self::new_inner(
            target,
            TextureFormat::Rgba8Unorm,
            Rect::from(data.dimensions()),
            Some(data.as_raw()),
        )
    }

    pub fn new_grey(target: &Target, dim: Rect) -> Self {
        Self::new_inner(target, TextureFormat::R8Unorm, dim, None)
    }

    pub fn new_grey_with(target: &Target, data: &GrayImage) -> Self {
        Self::new_inner(
            target,
            TextureFormat::R8Unorm,
            Rect::from(data.dimensions()),
            Some(data.as_raw()),
        )
    }

    pub fn dim(&self) -> Rect {
        self.dim
    }

    pub fn write(
        &self,
        target: &Target,
        spot: PositionedRect,
        image: DynamicImage,
    ) -> Result<(), &'static str> {
        if spot.width != image.width() || spot.height != image.height() {
            return Err("Image dimensions do not match the spot dimension");
        }

        if spot.x + spot.width > self.dim.width || spot.y + spot.height > self.dim.height {
            return Err("Spot out of the texture's bounds");
        }

        const INVALID_FORMAT: &str = "Image format doesn't match with the texture format";
        let image = match self.format {
            TextureFormat::Rgba8Unorm | TextureFormat::Rgba8UnormSrgb => {
                image.as_rgba8().ok_or(INVALID_FORMAT)?.as_raw()
            }
            TextureFormat::R8Unorm => image.as_luma8().ok_or(INVALID_FORMAT)?.as_raw(),
            _ => unimplemented!(),
        };

        target.queue.write_texture(
            ImageCopyTexture {
                texture: &self.texture,
                mip_level: 0,
                origin: Origin3d {
                    x: spot.x,
                    y: spot.y,
                    z: 0,
                },
                aspect: TextureAspect::All,
            },
            image,
            ImageDataLayout {
                offset: 0,
                bytes_per_row: Some(NonZeroU32::new(spot.width).unwrap()),
                rows_per_image: Some(NonZeroU32::new(spot.height).unwrap()),
            },
            spot.rect().into(),
        );

        Ok(())
    }

    pub async fn read(&self, target: &Target) -> RgbaImage {
        let dim = BufferDimensions::new(self.dim.width as _, self.dim.height as _);

        // cache these buffers
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

        RgbaImage::from_raw(dim.width as _, dim.height as _, bytes).unwrap()
    }

    fn new_inner(target: &Target, format: TextureFormat, dim: Rect, data: Option<&[u8]>) -> Self {
        let desc = TextureDescriptor {
            label: label!(),
            size: dim.into(),
            mip_level_count: 1,
            sample_count: 1,
            dimension: TextureDimension::D2,
            format,
            usage: TextureUsages::from_bits_truncate(USAGE),
        };
        let texture = match data {
            None => target.device.create_texture(&desc),
            Some(data) => target
                .device
                .create_texture_with_data(&target.queue, &desc, data),
        };
        let view = texture.create_view(&Default::default());

        Self {
            texture,
            format,
            view,
            dim,
        }
    }
}

impl<const USAGE: u32> Deref for Texture<USAGE> {
    type Target = TextureView;

    fn deref(&self) -> &Self::Target {
        &self.view
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
