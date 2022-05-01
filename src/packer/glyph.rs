use super::{packer2d::Packer, pos::TexturePosition, rect::Rect};
use crate::{label, target::Target};
use fontdue::{Font, FontSettings};
use std::{collections::HashMap, num::NonZeroU32, ops::Deref};
use wgpu::{
    ImageCopyTexture, ImageDataLayout, Origin3d, Texture, TextureAspect, TextureDescriptor,
    TextureDimension, TextureFormat, TextureUsages, TextureView,
};

#[cfg(feature = "font_loader")]
use font_loader::system_fonts::{self, FontProperty};

//

type GlyphHash = usize;

//

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
struct Glyph {
    index: u16,
    scale: u16,
    font: usize,
}

pub struct Glyphs {
    texture: Texture,
    view: TextureView,
    packer: Packer,

    fonts: Vec<Font>,
    glyphs: HashMap<Glyph, (GlyphHash, TexturePosition)>,

    queue: Vec<Glyph>,
}

//

impl Glyphs {
    pub fn new(target: &Target, rect: Rect) -> Self {
        let texture = target.device.create_texture(&TextureDescriptor {
            label: label!(),
            size: rect.into(),
            mip_level_count: 1,
            sample_count: 1,
            dimension: TextureDimension::D2,
            format: TextureFormat::R8Unorm,
            usage: TextureUsages::TEXTURE_BINDING | TextureUsages::COPY_DST,
        });
        let view = texture.create_view(&Default::default());
        let packer = Packer::new(rect);

        let fonts = Default::default();
        let glyphs = Default::default();
        let queue = Default::default();

        Self {
            texture,
            view,
            packer,

            fonts,
            glyphs,

            queue,
        }
    }

    pub fn add_font(&mut self, font: Font) -> usize {
        let id = self.fonts.len();
        self.fonts.push(font);
        id
    }

    pub fn add_font_bytes(&mut self, font: &[u8]) -> Result<usize, &'static str> {
        Ok(self.add_font(Font::from_bytes(font, FontSettings::default())?))
    }

    #[cfg(feature = "font_loader")]
    pub fn add_font_property(&mut self, font: FontProperty) -> Result<usize, &'static str> {
        self.add_font_bytes(&system_fonts::get(&font).ok_or("Font not found")?.0[..])
    }

    pub fn queue(&mut self, c: char, scale: u16, font: usize) {
        self.queue.push(Glyph {
            index: self.fonts[font].lookup_glyph_index(c),
            scale,
            font,
        });
    }

    pub fn flush(&mut self, target: &Target) {
        let mut tmp_queue = vec![];
        std::mem::swap(&mut tmp_queue, &mut self.queue);

        for queued in tmp_queue.drain(..) {
            if self.get_glyph(&queued).is_some() {
                continue;
            }

            let (metrics, data) =
                self.fonts[queued.font].rasterize_indexed(queued.index, queued.scale as _);

            let rect = self
                .packer
                .push(Rect::new(metrics.width as _, metrics.height as _))
                .unwrap();

            let dim = self.packer.area();

            self.glyphs
                .insert(queued, (0, TexturePosition::new(dim, rect)));

            // write the glyph texture into the texture pack
            // unless the texture has 0 area

            if metrics.width == 0 || metrics.height == 0 {
                continue;
            }

            target.queue.write_texture(
                ImageCopyTexture {
                    texture: &self.texture,
                    mip_level: 0,
                    origin: Origin3d {
                        x: rect.x,
                        y: rect.y,
                        z: 0,
                    },
                    aspect: TextureAspect::All,
                },
                &data,
                ImageDataLayout {
                    offset: 0,
                    bytes_per_row: Some(NonZeroU32::new(rect.width).unwrap()),
                    rows_per_image: Some(NonZeroU32::new(rect.height).unwrap()),
                },
                rect.rect().into(),
            );
        }
    }

    fn get_glyph(&self, glyph: &Glyph) -> Option<TexturePosition> {
        Some(self.glyphs.get(glyph)?.1)
    }

    pub fn get(&self, c: char, scale: u16, font: usize) -> Option<TexturePosition> {
        self.get_glyph(&Glyph {
            index: self.fonts[font].lookup_glyph_index(c),
            scale,
            font,
        })
    }

    pub fn get_indexed(&self, index: u16, scale: u16, font: usize) -> Option<TexturePosition> {
        self.get_glyph(&Glyph { index, scale, font })
    }

    pub fn font(&self, font: usize) -> Option<&'_ Font> {
        self.fonts.get(font)
    }
}

//

impl Deref for Glyphs {
    type Target = TextureView;

    fn deref(&self) -> &Self::Target {
        &self.view
    }
}
