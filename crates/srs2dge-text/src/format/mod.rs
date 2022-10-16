use self::{iter::FormatChars, part::FormatStringPart};
use srs2dge_core::color::Color;

//

pub mod iter;
pub mod part;
pub mod prelude;

//

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct FormatChar {
    pub character: char,
    pub format: Format,
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct FormatString<'a> {
    parts: Vec<FormatStringPart<'a>>,
    init: Format,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Format {
    pub color: Color,
    pub font: usize,
    pub px: f32,
}

//

impl<'a> FormatString<'a> {
    pub fn builder() -> Self {
        Self::default()
    }

    pub fn to_static(self) -> FormatString<'static> {
        FormatString {
            parts: self
                .parts
                .into_iter()
                .map(|part| part.to_static())
                .collect(),
            init: self.init,
        }
    }

    pub fn with<P: Into<FormatStringPart<'a>>>(self, part: P) -> Self {
        self.with_part(part.into())
    }

    pub fn with_part(mut self, part: FormatStringPart<'a>) -> Self {
        self.add_part(part);
        self
    }

    pub fn with_init(mut self, init: Format) -> Self {
        self.init(init);
        self
    }

    pub fn add<P: Into<FormatStringPart<'a>>>(&mut self, part: P) {
        self.add_part(part.into());
    }

    pub fn add_part(&mut self, part: FormatStringPart<'a>) {
        self.parts.push(part);
    }

    pub fn init(&mut self, init: Format) {
        self.init = init;
    }

    pub fn chars(&self) -> FormatChars {
        self.into()
    }
}

impl Default for Format {
    fn default() -> Self {
        Self {
            color: Color::BLACK,
            font: 0,
            px: 18.0,
        }
    }
}

impl Format {
    pub fn new(color: Color, font: usize, px: f32) -> Self {
        Self { color, font, px }
    }
}

impl<'a> FromIterator<FormatStringPart<'a>> for FormatString<'a> {
    fn from_iter<T: IntoIterator<Item = FormatStringPart<'a>>>(iter: T) -> Self {
        Self {
            parts: iter.into_iter().collect(),
            ..Default::default()
        }
    }
}

impl<'a> From<&'a str> for FormatString<'a> {
    fn from(val: &'a str) -> Self {
        Self {
            parts: vec![val.into()],
            ..Default::default()
        }
    }
}

impl From<String> for FormatString<'static> {
    fn from(val: String) -> Self {
        Self {
            parts: vec![val.into()],
            ..Default::default()
        }
    }
}
