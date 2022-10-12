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
    config: TextConfig,

    // kerning
    last_char: Option<char>,

    // not the X startter program
    x_init: f32,
}

//

impl<'a, I> LineTextChars<'a, I>
where
    I: Iterator<Item = FormatChar> + Clone,
{
    pub fn new(chars: I, fonts: &'a Fonts, config: TextConfig) -> Self {
        let mut non_aligned = Self {
            chars,
            fonts,
            config,
            last_char: None,
            x_init: config.x_origin,
        };

        let mut cursor_x = config.x_origin;
        let mut cursor_y = config.y_origin;

        // move the whole area if the alignment requires to
        let bounding_box = Lazy::new(|| non_aligned.clone().bounding_box_max());
        match config.align.x {
            XOrigin::Middle => {
                cursor_x -= bounding_box.0.width / 2.0;
            }
            XOrigin::Right => {
                // TODO: write from right to left as an optimization
                cursor_x -= bounding_box.0.width;
            }
            XOrigin::Left => {}
        }
        match config.align.y {
            YOrigin::Top => cursor_y -= bounding_box.2,
            YOrigin::Bottom => cursor_y -= bounding_box.1,
            YOrigin::Middle => cursor_y -= (bounding_box.1 + bounding_box.2) * 0.5,
            YOrigin::Baseline => {}
        }

        // fix the non aligned iterator
        non_aligned.x_init = cursor_x.floor();
        non_aligned.config.x_origin = cursor_x.floor();
        non_aligned.config.y_origin = cursor_y.floor();

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
            let line = font.inner().horizontal_line_metrics(c.format.px).unwrap();

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

                    // round upwards to nearest multiple of width
                    //
                    // `floor + 1` because `ceil` would do nothing if it divides evenly
                    self.config.x_origin = ((self.config.x_origin - self.x_init) / width).floor()
                        * width
                        + width
                        + self.x_init;

                    self.last_char = None;
                }
                _ => {
                    let index = font.lookup_glyph_index(character);
                    let metrics = font.metrics_indexed(index, px, self.config.sdf);
                    let x = self.config.x_origin.round() /* as i32 */ + metrics.xmin as f32;
                    let y = self.config.y_origin.round() /* as i32 */ + metrics.ymin as f32;
                    let width = metrics.width as _;
                    let height = metrics.height as _;
                    let area = width * height;

                    match self.config.dir {
                        TextDirection::Right => {
                            let kern = self
                                .last_char
                                .and_then(|last_c| {
                                    font.inner().horizontal_kern(last_c, character, px)
                                })
                                .unwrap_or(0.0);
                            self.config.x_origin += metrics.advance_width + kern;
                            // println!("{} {}", metrics.advance_width, kern);
                        }
                        TextDirection::Down => {
                            self.config.y_origin += metrics.advance_height;
                        }
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
