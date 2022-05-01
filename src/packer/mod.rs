//

pub mod glyph;
pub mod packer2d;
pub mod pos;
pub mod rect;
pub mod texture;

//

mod dim;

//

/* pub struct Texture {
    texture: wgpu::Texture,
    dim: Rect,
} */

//

/* impl Texture {
    pub fn new(target: &Target, dim: Rect) -> Self {
        let texture = target.device.create_texture(&TextureDescriptor {
            label: label!(),
            size: dim.into(),
            mip_level_count: 1,
            sample_count: 1,
            dimension: TextureDimension::D2,
            format: TextureFormat::Rgba8UnormSrgb,
            usage: TextureUsages::TEXTURE_BINDING,
        });
        Self { texture, dim }
    }
} */
