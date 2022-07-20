use super::{
    config::TextConfig,
    prelude::{TextBoundingBox, TextDirection, XOrigin, YOrigin},
    TextChar,
};
use crate::{
    glyphs::fonts::Fonts,
    prelude::{Format, FormatChar},
};
use once_cell::unsync::Lazy;

//

/// This text iter does **not** care about newlines
///
/// This is for separate lines only
#[derive(Debug, Clone, Copy)]
pub struct LineTextChars<'a, I>
where
    I: Iterator<Item = FormatChar> + Clone,
{
    // iterator for characters and their 'formats'
    chars: I,

    // all available fonts
    fonts: &'a Fonts,

    // text config
    pub config: TextConfig,

    // kerning
    last_char: Option<char>,
}

//

impl<'a, I> LineTextChars<'a, I>
where
    I: Iterator<Item = FormatChar> + Clone,
{
    pub fn new(chars: I, fonts: &'a Fonts, mut config: TextConfig) -> Self {
        let mut non_aligned = Self {
            chars,
            fonts,
            config,
            last_char: None,
        };

        // move the whole area if the alignment requires to
        let bounding_box = Lazy::new(|| non_aligned.clone().bounding_box_max());
        match config.align.x {
            XOrigin::Middle => {
                config.x_origin -= bounding_box.0.width / 2.0;
            }
            XOrigin::Right => {
                // TODO: write from right to left as an optimization
                config.x_origin -= bounding_box.0.width;
            }
            XOrigin::Left => {}
        }
        match config.align.y {
            YOrigin::Top => config.y_origin -= bounding_box.2,
            YOrigin::Bottom => config.y_origin -= bounding_box.1,
            YOrigin::Middle => config.y_origin -= (bounding_box.1 + bounding_box.2) * 0.5,
            YOrigin::Baseline => {}
        }
        non_aligned.config = config;

        non_aligned
    }

    pub fn bounding_box(self) -> TextBoundingBox {
        self.bounding_box_max().0
    }

    fn bounding_box_max(mut self) -> (TextBoundingBox, f32, f32) {
        let mut result = TextBoundingBox {
            x: self.config.x_origin,
            y: self.config.y_origin,
            width: 0.0,
            height: 0.0,
        };
        let mut min = f32::MAX;
        let mut max = f32::MIN;

        while let Some(c) = self.next() {
            let font = self.fonts.get_font(c.format.font);
            let line = font.horizontal_line_metrics(c.format.px).unwrap();

            result = result.union(TextBoundingBox {
                x: self.config.x_origin,
                y: self.config.y_origin,
                width: 0.0,
                height: line.ascent - line.descent,
            });

            min = min.min(line.descent);
            max = max.max(line.ascent);
        }

        (result, min, max)
    }
}

impl<'a, I> Iterator for LineTextChars<'a, I>
where
    I: Iterator<Item = FormatChar> + Clone,
{
    type Item = TextChar;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let FormatChar { character, format } = self.chars.next()?;
            let Format { font, mut px, .. } = format;
            let font = self.fonts.get_font(font);
            px = (px * self.config.scale).round();

            match character {
                '\t' => {
                    // TODO: text direction
                    // align to `tab_width` space chars
                    let width = font.metrics(' ', px, self.config.sdf).advance_width
                        * self.config.tab_width as f32;

                    // `floor + 1` because `ceil` would do nothing if it divides evenly
                    self.config.x_origin = (self.config.x_origin / width).floor() * width + width;

                    self.last_char = None;
                }
                _ => {
                    let index = font.lookup_glyph_index(character);
                    let metrics = font.metrics_indexed(index, px, self.config.sdf);
                    let x = self.config.x_origin /* as i32 */ + metrics.xmin as f32;
                    let y = self.config.y_origin /* as i32 */ + metrics.ymin as f32;
                    let width = metrics.width as _;
                    let height = metrics.height as _;
                    let area = width * height;

                    match self.config.dir {
                        TextDirection::Right => {
                            let kern = self
                                .last_char
                                .and_then(|last_c| font.horizontal_kern(last_c, character, px))
                                .unwrap_or(0.0);
                            self.config.x_origin += metrics.advance_width + kern
                        }
                        TextDirection::Down => self.config.y_origin += metrics.advance_height,
                    };

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
            } // end match
        } // end loop
    }
}
