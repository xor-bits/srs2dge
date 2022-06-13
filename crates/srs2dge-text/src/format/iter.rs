use crate::prelude::FormatString;

use super::part::FormatStringPart;
use super::{Format, FormatChar};
use std::slice::Iter;
use std::str::Chars;

//

// TODO:
// pub trait FormatChars = Iterator<Item = FormatChar>;

//

#[derive(Debug, Clone)]
pub struct FormatChars<'a> {
    parts: Iter<'a, FormatStringPart<'a>>,
    current: Option<Chars<'a>>,
    format: Format,
    init: Format,
}

//

impl<'a> FormatChars<'a> {
    pub fn init(&self) -> Format {
        self.init
    }

    pub fn chars_only(&self) -> impl Iterator<Item = char> + '_ {
        self.current
            .iter()
            .flat_map(|c| c.to_owned())
            .chain(self.parts.to_owned().flat_map(|part| match part {
                FormatStringPart::String(s) => s.chars(),
                FormatStringPart::Str(s) => s.chars(),
                _ => "".chars(),
            }))
    }
}

impl<'a> From<&'a str> for FormatChars<'a> {
    fn from(val: &'a str) -> Self {
        Self {
            parts: [].iter(),
            current: Some(val.chars()),
            format: Format::default(),
            init: Format::default(),
        }
    }
}

impl<'a> From<&'a FormatString<'a>> for FormatChars<'a> {
    fn from(val: &'a FormatString<'a>) -> Self {
        Self {
            parts: val.parts.iter(),
            current: None,
            format: val.init,
            init: val.init,
        }
    }
}

impl<'a> Iterator for FormatChars<'a> {
    type Item = FormatChar;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if let Some(current) = self.current.as_mut() {
                if let Some(character) = current.next() {
                    let format = self.format;
                    return Some(FormatChar { character, format });
                }
            }

            match self.parts.next()? {
                FormatStringPart::String(s) => self.current = Some(s.chars()),
                FormatStringPart::Str(s) => self.current = Some(s.chars()),
                FormatStringPart::Color(color) => self.format.color = *color,
                FormatStringPart::Font(font) => self.format.font = *font,
                FormatStringPart::Px(px) => self.format.px = *px,
                FormatStringPart::Reset => self.format = self.init,
            }
        }
    }
}
