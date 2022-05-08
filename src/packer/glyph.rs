use crate::{
    prelude::{Packer, Rect, Target, TexturePosition},
    texture::Texture,
};
use fontsdf::Font;
use image::GrayImage;
use std::{collections::HashMap, ops::Deref};
use wgpu::TextureUsages;

#[cfg(feature = "font_loader")]
use font_loader::system_fonts::{self, FontProperty};

//

type GlyphHash = usize;

//

const USAGE: u32 = TextureUsages::TEXTURE_BINDING.bits() | TextureUsages::COPY_DST.bits();

//

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
struct Glyph {
    index: u16,
    scale: u16,
    font: usize,
}

pub struct Glyphs {
    texture: Texture<USAGE>,
    packer: Packer,

    fonts: Vec<Font>,
    glyphs: HashMap<Glyph, (GlyphHash, TexturePosition)>,
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
    pub fn new(target: &Target, dim: Rect, sdf: Option<u16>) -> Self {
        let texture = Texture::new_grey(target, dim);
        let packer = Packer::new(dim);

        let fonts = Default::default();
        let glyphs = Default::default();
        let queue = Default::default();

        let mut result = Self {
            texture,
            packer,

            fonts,
            glyphs,
            sdf,

            queue,
        };

        result
            .add_font_bytes(include_bytes!("../../res/font/roboto/font.ttf"))
            .unwrap();

        result
    }

    /// Add a font to this glyph map
    ///
    /// returns a handle to it
    ///
    /// this handle is used to format text
    pub fn add_font(&mut self, font: Font) -> usize {
        let id = self.fonts.len();
        self.fonts.push(font);
        id
    }

    /// Add a font to this glyph map
    ///
    /// returns a handle to it
    ///
    /// this handle is used to format text
    pub fn add_font_bytes(&mut self, font: &[u8]) -> Result<usize, &'static str> {
        Ok(self.add_font(Font::from_bytes(font)?))
    }

    /// Add a font to this glyph map
    ///
    /// returns a handle to it
    ///
    /// this handle is used to format text
    #[cfg(feature = "font_loader")]
    pub fn add_font_property(&mut self, font: FontProperty) -> Result<usize, &'static str> {
        self.add_font_bytes(&system_fonts::get(&font).ok_or("Font not found")?.0[..])
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
            index: self.fonts[font].lookup_glyph_index(c),
            scale,
            font,
        });
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

            let (metrics, data) = self.fonts[queued.font].rasterize_indexed(
                queued.index,
                queued.scale as _,
                self.is_sdf(),
            );

            let rect = self
                .packer
                .push(Rect::new(metrics.width as _, metrics.height as _))
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
            index: self.fonts[font].lookup_glyph_index(c),
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

    pub fn get_font(&self, font: usize) -> Option<&'_ Font> {
        self.fonts.get(font)
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
