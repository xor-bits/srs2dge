use std::{collections::BTreeMap, iter::FromIterator, ops::AddAssign};

use glam::Vec3;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Format {
    pub color: Vec3,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct FormatPair<T> {
    pub format: Format,
    pub other: T,
}

impl Default for Format {
    fn default() -> Self {
        Self {
            color: Vec3::new(1.0, 1.0, 1.0),
        }
    }
}

pub trait Formatted
where
    Self: Sized,
{
    fn formatted_with(&self, format: Format) -> FormatPair<&'_ str>;

    fn formatted(&self) -> FormatPair<&'_ str>;
    fn colored(&self, r: f32, g: f32, b: f32) -> FormatPair<&'_ str>;
}

impl Formatted for &str {
    fn formatted_with(&self, format: Format) -> FormatPair<&'_ str> {
        FormatPair {
            format,
            other: self,
        }
    }

    fn formatted(&self) -> FormatPair<&'_ str> {
        Self::formatted_with(self, Format::default())
    }

    fn colored(&self, r: f32, g: f32, b: f32) -> FormatPair<&'_ str> {
        Self::formatted_with(
            self,
            Format {
                color: Vec3::new(r, g, b),
            },
        )
    }
}

impl Formatted for String {
    fn formatted_with(&self, format: Format) -> FormatPair<&'_ str> {
        FormatPair {
            format,
            other: self.as_str(),
        }
    }

    fn formatted(&self) -> FormatPair<&'_ str> {
        Self::formatted_with(self, Format::default())
    }

    fn colored(&self, r: f32, g: f32, b: f32) -> FormatPair<&'_ str> {
        Self::formatted_with(
            self,
            Format {
                color: Vec3::new(r, g, b),
            },
        )
    }
}

// FORMAT STRING

pub struct FString {
    string: String,
    format: BTreeMap<usize, Format>,
}

impl FString {
    pub fn new() -> Self {
        Self {
            string: String::new(),
            format: BTreeMap::new(),
        }
    }

    pub fn from_str<S>(string: S) -> Self
    where
        S: Into<String>,
    {
        Self {
            string: string.into(),
            format: BTreeMap::new(),
        }
    }

    pub fn from_formatted<S>(string: FormatPair<S>) -> Self
    where
        S: Into<String>,
    {
        Self {
            string: string.other.into(),
            format: BTreeMap::from_iter([(0, string.format)]),
        }
    }

    pub fn from_slice<'s, S>(formatted: &'s [FormatPair<S>]) -> Self
    where
        &'s str: From<&'s S>,
    {
        Self::from_iter(formatted.iter().map(|s| FormatPair::<&'s str> {
            format: s.format,
            other: (&s.other).into(),
        }))
    }

    pub fn from_iter<'s, S, I>(formatted: I) -> Self
    where
        I: IntoIterator<Item = FormatPair<S>>,
        S: Into<&'s str> + 's,
    {
        let mut string = String::new();
        let format = BTreeMap::from_iter(formatted.into_iter().map(|s| {
            let _str = s.other.into();
            let len = string.len();
            string.push_str(_str);
            (len, s.format)
        }));

        Self { string, format }
    }

    pub fn insert_format(&mut self, index: usize, format: Format) {
        self.format.insert(index, format);
    }

    pub fn chars(&self) -> impl Iterator<Item = (char, Format)> + '_ {
        let mut format = Format::default();
        self.string.char_indices().map(move |(index, c)| {
            if let Some(new_format) = self.format.get(&index) {
                format = *new_format;
            }
            (c, format)
        })
    }

    pub fn as_str(&self) -> &'_ str {
        self.string.as_str()
    }
}

impl<S> From<S> for FString
where
    S: Into<String>,
{
    fn from(string: S) -> Self {
        Self::from_str(string)
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
        self.add_assign(rhs.into().formatted());
    }
}
