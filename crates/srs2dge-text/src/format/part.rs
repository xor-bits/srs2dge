use srs2dge_core::color::Color;

//

#[derive(Debug, Clone, PartialEq)]
pub enum FormatStringPart<'a> {
    String(String),
    Str(&'a str),
    // Char(char),
    Color(Color),
    Font(usize),
    Px(f32),
    Reset,
}

pub type Reset = ();

//

impl<'a> FormatStringPart<'a> {
    pub fn to_static(self) -> FormatStringPart<'static> {
        match self {
            FormatStringPart::String(v) => FormatStringPart::String(v),
            FormatStringPart::Str(v) => FormatStringPart::String(v.to_owned()),
            // FormatStringPart::Char(v) => FormatStringPart::Char(v),
            FormatStringPart::Color(v) => FormatStringPart::Color(v),
            FormatStringPart::Font(v) => FormatStringPart::Font(v),
            FormatStringPart::Px(v) => FormatStringPart::Px(v),
            FormatStringPart::Reset => FormatStringPart::Reset,
        }
    }
}

impl From<String> for FormatStringPart<'static> {
    fn from(val: String) -> Self {
        FormatStringPart::String(val)
    }
}

impl<'a> From<&'a str> for FormatStringPart<'a> {
    fn from(val: &'a str) -> Self {
        FormatStringPart::Str(val)
    }
}

/* impl From<char> for FormatStringPart<'static> {
    fn from(val: char) -> Self {
        FormatStringPart::Char(val)
    }
} */

impl From<Color> for FormatStringPart<'static> {
    fn from(val: Color) -> Self {
        FormatStringPart::Color(val)
    }
}

impl From<usize> for FormatStringPart<'static> {
    fn from(val: usize) -> Self {
        FormatStringPart::Font(val)
    }
}

impl From<f32> for FormatStringPart<'static> {
    fn from(val: f32) -> Self {
        FormatStringPart::Px(val)
    }
}

impl From<()> for FormatStringPart<'static> {
    fn from(_: ()) -> Self {
        FormatStringPart::Reset
    }
}
