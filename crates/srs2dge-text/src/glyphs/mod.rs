use crate::prelude::FormatChar;

use self::fonts::Fonts;
use fontsdf::Font;
use srs2dge_core::{
    image::GrayImage,
    prelude::{Packer, Rect, TexturePosition},
    target::Target,
    texture::Texture,
    wgpu::TextureUsages,
};
use std::{any::type_name, collections::HashMap, ops::Deref};

//

pub mod fonts;
pub mod prelude;

//

const USAGE: u32 = TextureUsages::TEXTURE_BINDING.bits()
    | TextureUsages::COPY_DST.bits()
    | TextureUsages::COPY_SRC.bits();

//

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
struct Glyph {
    index: u16,
    scale: u16,
    font: usize,
}

#[derive(Debug)]
pub struct Glyphs {
    texture: Texture<USAGE>,
    packer: Packer,

    fonts: Fonts,
    glyphs: HashMap<Glyph, (usize, TexturePosition)>,
    sdf: Option<u16>,

    queue: Vec<Glyph>,
}

//

impl Glyphs {
    /// Creates a dynamic glyph atlas map thingy.
    ///
    /// It uploads used glyphs to its texture
    /// and _**in the future**, it will replace less
    /// used glyphs when no room is available_.
    ///
    /// `sdf` enables or disables the optional SDF
    /// renderer mode. SDF rendering requires the
    /// `SdfShader` instead of `TextShader`.
    pub fn new(
        target: &Target,
        dim: Rect,
        sdf: Option<u16>,
        fonts: Fonts,
        label: Option<&str>,
    ) -> Self {
        Self {
            texture: Texture::new_grey(target, dim, Some(label.unwrap_or(type_name::<Self>()))),
            packer: Packer::new(dim).with_padding(2),

            fonts,
            glyphs: Default::default(),
            sdf,

            queue: Default::default(),
        }
    }

    /// Creates a dynamic glyph atlas map thingy.
    ///
    /// It uploads used glyphs to its texture
    /// and _**in the future**, it will replace less
    /// used glyphs when no room is available_.
    ///
    /// `sdf` enables or disables the optional SDF
    /// renderer mode. SDF rendering requires the
    /// `SdfShader` instead of `TextShader`.
    pub fn new_with_fallback(
        target: &Target,
        dim: Rect,
        sdf: Option<u16>,
        fallback: Font,
        label: Option<&str>,
    ) -> Self {
        Self::new(target, dim, sdf, Fonts::new(fallback), label)
    }

    /// Creates a dynamic glyph atlas map thingy.
    ///
    /// It uploads used glyphs to its texture
    /// and _**in the future**, it will replace less
    /// used glyphs when no room is available_.
    ///
    /// `sdf` enables or disables the optional SDF
    /// renderer mode. SDF rendering requires the
    /// `SdfShader` instead of `TextShader`.
    pub fn new_with_fallback_bytes(
        target: &Target,
        dim: Rect,
        sdf: Option<u16>,
        fallback_bytes: &[u8],
        label: Option<&str>,
    ) -> Result<Self, &'static str> {
        Ok(Self::new(
            target,
            dim,
            sdf,
            Fonts::new_bytes(fallback_bytes)?,
            label,
        ))
    }

    /// Inner font map for this glyph map
    ///
    /// Used for retrieving fonts
    pub fn fonts(&self) -> &Fonts {
        &self.fonts
    }

    /// Inner font map for this glyph map
    ///
    /// Used for adding and retrieving fonts
    pub fn fonts_mut(&mut self) -> &mut Fonts {
        &mut self.fonts
    }

    /// Queues a glyph to be available
    /// after the next flush happens
    ///
    /// scale is ignored with sdf glyph maps
    pub fn queue(&mut self, c: char, scale: u16, font: usize) {
        let scale = if let Some(scale) = self.sdf {
            scale
        } else {
            scale
        };

        self.queue.push(Glyph {
            index: self.fonts.get_font(font).lookup_glyph_index(c),
            scale,
            font,
        });
    }

    pub fn queue_all<I: IntoIterator<Item = FormatChar>>(&mut self, i: I) {
        self.queue
            .extend(i.into_iter().map(|FormatChar { character, format }| {
                let scale = if let Some(scale) = self.sdf {
                    scale
                } else {
                    format.px as _
                };

                Glyph {
                    index: self
                        .fonts
                        .get_font(format.font)
                        .lookup_glyph_index(character),
                    scale,
                    font: format.font,
                }
            }));
    }

    /// Generates and uploads all queued
    /// glyphs to the gpu texture
    pub fn flush(&mut self, target: &Target) -> Result<(), &'static str> {
        let mut tmp_queue = vec![];
        std::mem::swap(&mut tmp_queue, &mut self.queue);

        for queued in tmp_queue.drain(..) {
            if self.get_glyph(&queued).is_some() {
                continue;
            }

            let (metrics, data) = self.fonts.get_font(queued.font).rasterize_indexed(
                queued.index,
                queued.scale as _,
                self.is_sdf(),
            );

            let rect = self
                .packer
                .push(Rect::new(metrics.width as u32, metrics.height as u32))
                .ok_or("Out of space")?;

            let dim = self.packer.area();

            self.glyphs
                .insert(queued, (0, TexturePosition::new(dim, rect)));

            // write the glyph texture into the texture pack
            // unless the texture has 0 area

            if metrics.width == 0 || metrics.height == 0 {
                continue;
            }

            let image = GrayImage::from_raw(rect.width, rect.height, data).unwrap();
            self.texture.write(target, rect, image.into()).unwrap();
        }

        Ok(())
    }

    /// Get a position to a glyph
    ///
    /// Guaranteed to be available if and
    /// only if this exact glyph was queued
    /// for the previous flush.
    ///
    /// scale is ignored with sdf glyph maps
    pub fn get(&self, c: char, scale: u16, font: usize) -> Option<TexturePosition> {
        let scale = if let Some(scale) = self.sdf {
            scale
        } else {
            scale
        };

        self.get_glyph(&Glyph {
            index: self.fonts.get_font(font).lookup_glyph_index(c),
            scale,
            font,
        })
    }

    /// Get a position to a glyph
    ///
    /// Guaranteed to be available if and
    /// only if this exact glyph was queued
    /// for the previous flush.
    ///
    /// scale is ignored with sdf glyph maps
    pub fn get_indexed(&self, index: u16, scale: u16, font: usize) -> Option<TexturePosition> {
        let scale = if let Some(scale) = self.sdf {
            scale
        } else {
            scale
        };

        self.get_glyph(&Glyph { index, scale, font })
    }

    pub fn is_sdf(&self) -> bool {
        self.sdf.is_some()
    }

    pub fn sdf_resolution(&self) -> Option<u16> {
        self.sdf
    }

    fn get_glyph(&self, glyph: &Glyph) -> Option<TexturePosition> {
        Some(self.glyphs.get(glyph)?.1)
    }
}

//

impl Deref for Glyphs {
    type Target = Texture<USAGE>;

    fn deref(&self) -> &Self::Target {
        &self.texture
    }
}
