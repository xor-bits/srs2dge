use fontdue::Font;

use crate::packer::glyph::Glyphs;

use super::format::{FString, Format};

pub struct CharPositionIter<'s> {
    chars: Box<dyn Iterator<Item = (char, Format)> + 's>,
    glyphs: &'s Glyphs,
    font: &'s Font,

    px: f32,

    x_origin: i32,
    y_origin: i32,
    last_c: Option<char>,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CharPosition {
    pub index: u16,
    pub format: Format,
    pub x: i32,
    pub y: i32,
    pub width: u32,
    pub height: u32,
}

impl<'s> CharPositionIter<'s> {
    pub fn new(string: &'s FString, glyphs: &'s Glyphs, px: f32) -> Self {
        Self {
            chars: Box::new(string.chars()),
            glyphs,
            font: glyphs.font(0).unwrap(),

            px,

            x_origin: 0,
            y_origin: px as i32,
            last_c: None,
        }
    }
}

impl<'s> Iterator for CharPositionIter<'s> {
    type Item = CharPosition;

    fn next(&mut self) -> Option<Self::Item> {
        let (mut c, mut format);
        loop {
            (c, format) = self.chars.next()?;
            self.font = self.glyphs.font(format.font).unwrap();

            match c {
                '\n' => {
                    self.x_origin = 0;
                    self.y_origin += (self.px * 1.4) as i32;
                    self.last_c = None;
                }
                '\t' => {
                    //panic!("px{} tab{}", self.px, self.tab);
                    let w = self.font.metrics(' ', self.px).advance_width.floor() * 4.0;
                    self.x_origin = ((self.x_origin as f32 / w).floor() * w + w) as i32;
                    self.last_c = None;
                }
                _ => break,
            }
        }

        let index = self.font.lookup_glyph_index(c);
        let metrics = self.font.metrics_indexed(index, self.px);

        let rect = CharPosition {
            index,
            format,
            x: self.x_origin + metrics.xmin,
            y: self.y_origin - metrics.height as i32 - metrics.ymin,
            width: metrics.width as _,
            height: metrics.height as _,
        };

        self.x_origin += metrics.advance_width as i32
            + self
                .last_c
                .and_then(|last_c| self.font.horizontal_kern(last_c, c, self.px))
                .unwrap_or(0.0) as i32;
        // self.y_origin -= metrics.advance_height as i32;
        self.last_c = Some(c);

        Some(rect)
    }
}
