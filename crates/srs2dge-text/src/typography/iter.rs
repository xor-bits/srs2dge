use fontdue::LineMetrics;

use super::config::{TextConfig, YOrigin};
use crate::{
    glyphs::fonts::Fonts,
    prelude::{Format, FormatChar, FormatChars},
};

//

pub struct TextChars<'s> {
    // iterator for characters and their 'formats'
    chars: FormatChars<'s>,

    // all available fonts
    fonts: &'s Fonts,

    // text config
    config: TextConfig,

    pub(crate) x_origin: i32,
    pub(crate) y_origin: i32,

    // kerning
    last_char: Option<char>,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct TextChar {
    pub character: char,
    // character codepoint
    pub index: u16,
    // formatting
    pub format: Format,
    // top left
    pub x: i32,
    pub y: i32,
    // size
    pub width: u32,
    pub height: u32,
}

//

impl<'s> TextChars<'s> {
    pub fn new(chars: FormatChars<'s>, fonts: &'s Fonts, config: TextConfig) -> Self {
        let mut result = Self {
            chars,
            fonts,
            config,
            x_origin: 0,
            y_origin: 0,
            last_char: None,
        };

        result.init_origin(result.init());

        result
    }

    pub fn init(&self) -> Format {
        self.chars.init()
    }

    fn init_origin(&mut self, format: Format) {
        let line = self.line_metrics(format);

        let y_origin_offset = match self.config.y_origin_line {
            YOrigin::Baseline => 0.0,
            YOrigin::Descender => line.descent,
            YOrigin::Ascender => line.ascent,
            YOrigin::Mean => 0.5 * (line.descent + line.ascent),
        };

        self.x_origin = self.config.x_origin;
        self.y_origin = self.config.y_origin - y_origin_offset as i32;
    }

    pub(crate) fn new_line(&mut self, format: Format) {
        // TODO: new_line_size should not ignore lines with multiple fonts and px

        let line = self.line_metrics(format);

        self.x_origin = self.config.x_origin;
        self.y_origin -= line.new_line_size as i32;
    }

    fn line_metrics(&self, format: Format) -> LineMetrics {
        self.fonts
            .get_font(format.font)
            .horizontal_line_metrics(format.px)
            .unwrap_or_else(|| todo!())
    }
}

impl<'s> Iterator for TextChars<'s> {
    type Item = TextChar;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let FormatChar { character, format } = self.chars.next()?;
            let Format { font, px, .. } = format;
            let font = self.fonts.get_font(font);

            match character {
                '\n' => {
                    // TODO: text direction
                    // new lines
                    // TODO: font.vertical_line_metrics(px)
                    self.new_line(format);

                    self.last_char = None;
                }
                '\t' => {
                    // TODO: text direction
                    // align to `tab_width` space chars
                    let width = font.metrics(' ', px, self.config.sdf).advance_width.floor()
                        * self.config.tab_width as f32;

                    // `floor + 1` because `ceil` would do nothing if it divides evenly
                    self.x_origin = ((self.x_origin as f32 / width).floor() * width + width) as i32;

                    self.last_char = None;
                }
                _ => {
                    let index = font.lookup_glyph_index(character);
                    let metrics = font.metrics_indexed(index, px, self.config.sdf);
                    let x = self.x_origin + metrics.xmin;
                    let y = self.y_origin + metrics.ymin;
                    let width = metrics.width as _;
                    let height = metrics.height as _;
                    let area = width * height;

                    let kern = self
                        .last_char
                        .and_then(|last_c| font.horizontal_kern(last_c, character, px))
                        .unwrap_or(0.0) as i32;
                    self.x_origin += metrics.advance_width as i32 + kern;

                    /* match self.config.dir {
                        TextDirection::Right => {
                            let kern = self
                                .last_char
                                .and_then(|last_c| font.horizontal_kern(last_c, character, px))
                                .unwrap_or(0.0) as i32;
                            self.x_origin += metrics.advance_width as i32 + kern
                        }
                        TextDirection::Down => self.y_origin += metrics.advance_height as i32,
                    } */
                    /* let kern = self
                        .last_char
                        .and_then(|last_c| font.horizontal_kern(last_c, character, px))
                        .unwrap_or(0.0) as i32;
                    let xo = metrics.advance_width as i32 + kern;

                    let x = match self.config.x_origin_point {
                        XOrigin::Left => {
                            let x = self.x_origin + metrics.xmin;
                            self.x_origin += xo;
                            x
                        }
                        XOrigin::Right => {
                            self.x_origin -= xo;
                            self.x_origin + metrics.xmin
                        }
                        _ => todo!(),
                    }; */

                    self.last_char = Some(character);

                    // skip glyphs that have 0 area
                    if area == 0 {
                        continue;
                    }

                    return Some(TextChar {
                        character,
                        index,
                        format,
                        x,
                        y,
                        width,
                        height,
                    });
                }
            }
        }
    }
}
