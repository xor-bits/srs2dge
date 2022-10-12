use super::{config::TextConfig, line::LineTextChars, prelude::TextBoundingBox, TextChar};
use crate::{glyphs::fonts::Fonts, prelude::FormatChar};
use std::{fmt::Debug, iter::TakeWhile};

//

// TODO: whole text box alignment
#[derive(Debug, Clone)]
pub struct TextChars<'s, I>
where
    I: Iterator<Item = FormatChar> + Clone,
{
    // iterator for characters and their 'formats'
    chars: I,

    // all available fonts
    fonts: &'s Fonts,

    // text config
    pub config: TextConfig,

    current_line: LineTextChars<'s, TakeWhile<I, NotNewline>>,
    new_line: NewLineResult,
}

//

impl<'s, I> TextChars<'s, I>
where
    I: Iterator<Item = FormatChar> + Clone,
{
    pub fn new(mut chars: I, fonts: &'s Fonts, config: TextConfig) -> Self {
        let current_line = LineTextChars::new(
            chars.clone().take_while::<NotNewline>(not_newline),
            fonts,
            config,
        );

        let new_line = skip_until_newline(&mut chars, fonts);

        Self {
            chars,
            fonts,
            config,
            current_line,
            new_line,
        }
    }

    pub fn bounding_box(mut self) -> TextBoundingBox {
        // TODO: optimize for monospaced fonts

        let mut result = self.current_line.bounding_box();

        loop {
            if self.new_line.empty {
                break;
            }

            result = result.union(
                LineTextChars::new(
                    self.chars.clone().take_while::<NotNewline>(not_newline),
                    self.fonts,
                    self.config,
                )
                .bounding_box(),
            );
            self.new_line = skip_until_newline(&mut self.chars, self.fonts);
        }

        result
    }
}

impl<'s, I> Iterator for TextChars<'s, I>
where
    I: Iterator<Item = FormatChar> + Clone,
{
    type Item = TextChar;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if let Some(next) = self.current_line.next() {
                return Some(next);
            }

            self.config.y_origin -= self.new_line.max_ascender - self.new_line.min_descender;
            if self.new_line.empty {
                return None;
            }

            self.current_line = LineTextChars::new(
                self.chars.clone().take_while::<NotNewline>(not_newline),
                self.fonts,
                self.config,
            );
            self.new_line = skip_until_newline(&mut self.chars, self.fonts);
        }
    }
}

//

#[derive(Debug, Clone, Copy)]
struct NewLineResult {
    empty: bool,
    max_ascender: f32,
    min_descender: f32,
}

/// returns bool telling if the iterator is empty
fn skip_until_newline<I>(chars: &mut I, fonts: &Fonts) -> NewLineResult
where
    I: Iterator<Item = FormatChar>,
{
    let mut max_ascender = std::f32::MIN;
    let mut min_descender = std::f32::MAX;
    for s in chars.by_ref() {
        let line = fonts
            .get_font(s.format.font)
            .inner()
            .horizontal_line_metrics(s.format.px)
            .unwrap();
        max_ascender = max_ascender.max(line.ascent);
        min_descender = min_descender.min(line.descent);

        if s.character == '\n' {
            return NewLineResult {
                empty: false,
                max_ascender,
                min_descender,
            };
        }
    }

    NewLineResult {
        empty: true,
        max_ascender,
        min_descender,
    }
}

fn not_newline(c: &FormatChar) -> bool {
    c.character != '\n'
}

type NotNewline = fn(&FormatChar) -> bool;
