use srs2dge_core::glam::Vec3;
use std::{borrow::Borrow, collections::BTreeMap, iter::FromIterator, ops::AddAssign};

//

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NewSetting<T>
where
    T: Copy,
{
    // Leave this setting untouched
    Leave,

    // Get the default setting
    Default,

    // Set new setting
    New(T),
}

impl<T> Default for NewSetting<T>
where
    T: Copy,
{
    fn default() -> Self {
        Self::Default
    }
}

impl<T> NewSetting<T>
where
    T: Copy,
{
    #[must_use]
    pub fn merge(lhs: Self, rhs: Self) -> Self {
        match &rhs {
            Self::Leave => lhs,
            _ => rhs,
        }
    }

    pub fn merge_default(self, default: T) -> T {
        match &self {
            Self::Leave => unreachable!(),
            Self::Default => default,
            Self::New(new) => *new,
        }
    }
}

//

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Format {
    pub color: Vec3,
    pub font: usize,
}

//

impl Default for Format {
    fn default() -> Self {
        Self {
            color: Vec3::new(1.0, 1.0, 1.0),
            font: 0,
        }
    }
}

//

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct NewFormat {
    pub color: NewSetting<Vec3>,
    pub font: NewSetting<usize>,
}

//

impl NewFormat {
    #[must_use]
    pub fn leave() -> Self {
        Self {
            color: NewSetting::Leave,
            font: NewSetting::Leave,
        }
    }

    #[must_use]
    pub fn new(r: f32, g: f32, b: f32, font: usize) -> Self {
        Self {
            color: NewSetting::New(Vec3::new(r, g, b)),
            font: NewSetting::New(font),
        }
    }

    pub fn merge(lhs: Self, rhs: Self) -> Self {
        Self {
            color: NewSetting::merge(lhs.color, rhs.color),
            font: NewSetting::merge(lhs.font, rhs.font),
        }
    }

    pub fn merge_default(self, default: Format) -> Format {
        Format {
            color: NewSetting::merge_default(self.color, default.color),
            font: NewSetting::merge_default(self.font, default.font),
        }
    }
}

//

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct FormatPair<T> {
    pub format: NewFormat,
    pub other: T,
}

//

impl<T> FormatPair<T> {
    #[must_use]
    pub fn color(mut self, r: f32, g: f32, b: f32) -> Self {
        self.format.color = NewSetting::New(Vec3::new(r, g, b));
        self
    }

    #[must_use]
    pub fn font(mut self, font: usize) -> Self {
        self.format.font = NewSetting::New(font);
        self
    }
}

//

pub trait Formatted
where
    Self: Sized,
{
    fn leave(&self) -> FormatPair<&'_ str>;
    fn default(&self) -> FormatPair<&'_ str>;
    fn formatted(&self, format: NewFormat) -> FormatPair<&'_ str>;
}

//

impl Formatted for &str {
    fn leave(&self) -> FormatPair<&'_ str> {
        Self::formatted(self, NewFormat::leave())
    }

    fn default(&self) -> FormatPair<&'_ str> {
        Self::formatted(self, NewFormat::default())
    }

    fn formatted(&self, format: NewFormat) -> FormatPair<&'_ str> {
        FormatPair {
            format,
            other: self,
        }
    }
}

//

impl Formatted for String {
    fn leave(&self) -> FormatPair<&'_ str> {
        Self::formatted(self, NewFormat::leave())
    }

    fn default(&self) -> FormatPair<&'_ str> {
        Self::formatted(self, NewFormat::default())
    }

    fn formatted(&self, format: NewFormat) -> FormatPair<&'_ str> {
        FormatPair {
            format,
            other: self.as_str(),
        }
    }
}

// FORMAT STRING

#[derive(Debug, Clone, PartialEq, Default)]
pub struct FString {
    string: String,
    format: BTreeMap<usize, NewFormat>,

    default_format: Format,
}

//

impl<'s, S> FromIterator<FormatPair<S>> for FString
where
    S: Into<&'s str> + 's,
{
    fn from_iter<T: IntoIterator<Item = FormatPair<S>>>(iter: T) -> Self {
        let mut string = String::new();
        let format = BTreeMap::from_iter(iter.into_iter().map(|s| {
            let _str = s.other.into();
            let len = string.len();
            string.push_str(_str);
            (len, s.format)
        }));

        Self {
            string,
            format,
            default_format: Format::default(),
        }
    }
}

//

impl FString {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn from_string<S>(string: S) -> Self
    where
        S: Into<String>,
    {
        Self {
            string: string.into(),
            format: BTreeMap::new(),
            default_format: Format::default(),
        }
    }

    pub fn from_formatted<S>(string: FormatPair<S>) -> Self
    where
        S: Into<String>,
    {
        Self {
            string: string.other.into(),
            format: BTreeMap::from_iter([(0, string.format)]),
            default_format: Format::default(),
        }
    }

    pub fn from_slice<'s, S>(formatted: &'s [FormatPair<S>]) -> Self
    where
        &'s str: From<&'s S>,
    {
        Self::from_iter(formatted.iter().map(|s| FormatPair::<&'s str> {
            format: s.format,
            other: s.other.borrow().into(),
        }))
    }

    pub fn insert_format(&mut self, index: usize, format: NewFormat) {
        self.format.insert(index, format);
    }

    pub fn chars(&self) -> impl Iterator<Item = (char, Format)> + '_ {
        let mut format = NewFormat::default();
        self.string.char_indices().map(move |(index, c)| {
            if let Some(new_format) = self.format.get(&index) {
                format = NewFormat::merge(format, *new_format);
            }
            (c, format.merge_default(self.default_format))
        })
    }

    pub fn as_str(&self) -> &'_ str {
        self.string.as_str()
    }

    pub fn set_default_format(&mut self, format: Format) {
        self.default_format = format;
    }

    pub fn get_default_format(&self) -> Format {
        self.default_format
    }
}

impl<S> From<S> for FString
where
    S: Into<String>,
{
    fn from(string: S) -> Self {
        Self::from_string(string)
    }
}

impl<S> From<FormatPair<S>> for FString
where
    S: Into<String>,
{
    fn from(string: FormatPair<S>) -> Self {
        Self::from_formatted(string)
    }
}

impl AddAssign for FString {
    fn add_assign(&mut self, rhs: Self) {
        let len = self.string.len();
        self.format
            .extend(rhs.format.iter().map(|(i, f)| (i + len, *f)));
        self.string.push_str(rhs.string.as_str());
    }
}

impl<'s, S> AddAssign<FormatPair<S>> for FString
where
    S: Into<&'s str>,
{
    fn add_assign(&mut self, rhs: FormatPair<S>) {
        let len = self.string.len();
        self.format.insert(len, rhs.format);
        self.string.push_str(rhs.other.into());
    }
}

impl<'s, S> AddAssign<S> for FString
where
    S: Into<&'s str>,
{
    fn add_assign(&mut self, rhs: S) {
        self.add_assign(rhs.into().default());
    }
}
