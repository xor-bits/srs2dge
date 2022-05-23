use bytemuck::{Pod, Zeroable};
use rand::{distributions::Standard, prelude::Distribution, Rng};
use serde::{Deserialize, Serialize};
use std::{
    fmt::{self, Debug, Display},
    num::ParseIntError,
    ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign},
    str::FromStr,
};

//

/// ### Color
///
/// ```
/// # use srs2dge_core::color::Color;
/// # use std::str::FromStr;
/// let color: Color = Color::new_rgb(1.0, 0.5, 0.0);
/// println!("{color} {color:#}"); // '#ff7f00ff rgba[255, 127, 0, 255]'
/// assert_eq!(u32::from(color), 0xff7f00ff);
///
/// let color: Color = Color::ORANGE;
/// println!("{color} {color:#}"); // '#ff7f00ff rgba[255, 127, 0, 255]'
/// assert_eq!(u32::from(color), 0xff7f00ff);
///
/// assert_eq!(Color::YELLOW, Color::from(0xffff00ff));
/// assert_eq!(Color::YELLOW, Color::from_str("#ffff00ff").unwrap());
/// assert_eq!(Color::YELLOW, Color::from_str("ffff00ff").unwrap());
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize, Zeroable, Pod)]
#[repr(C)]
pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum HexColorError {
    EmptyStr,
    InvalidHexFormat(ParseIntError),
}

//

impl Default for Color {
    fn default() -> Self {
        Self::WHITE
    }
}

impl Color {
    pub const WHITE: Self = Self::new_mono(1.0);
    pub const LIGHT_GREY: Self = Self::new_mono(0.75);
    pub const LIGHT_GRAY: Self = Self::GREY;
    pub const GREY: Self = Self::new_mono(0.5);
    pub const GRAY: Self = Self::GREY;
    pub const DARK_GREY: Self = Self::new_mono(0.25);
    pub const DARK_GRAY: Self = Self::GREY;
    pub const BLACK: Self = Self::new_mono(0.0);

    pub const RED: Self = Self::new_rgb(1.0, 0.0, 0.0);
    pub const GREEN: Self = Self::new_rgb(0.0, 1.0, 0.0);
    pub const BLUE: Self = Self::new_rgb(0.0, 0.0, 1.0);

    pub const YELLOW: Self = Self::new_rgb(1.0, 1.0, 0.0);
    pub const CYAN: Self = Self::new_rgb(0.0, 1.0, 1.0);
    pub const MAGENTA: Self = Self::new_rgb(1.0, 0.0, 1.0);

    pub const ORANGE: Self = Self::new_rgb(1.0, 0.5, 0.0);
    pub const MINT: Self = Self::new_rgb(0.0, 1.0, 0.5); // or spring green
    pub const ROSE: Self = Self::new_rgb(1.0, 0.0, 0.5); // or hot pink

    pub const CHARTREUSE: Self = Self::new_rgb(0.5, 1.0, 0.0);
    pub const AZURE: Self = Self::new_rgb(0.0, 0.5, 1.0); // or spring green
    pub const VIOLET: Self = Self::new_rgb(0.5, 0.0, 1.0); // or hot pink

    pub const CLEAR_COLOR: Self = Self::new_rgb(0.05, 0.06, 0.07);

    #[inline]
    pub const fn new(r: f32, g: f32, b: f32, a: f32) -> Self {
        Self::new_rgba(r, g, b, a)
    }

    #[inline]
    pub const fn new_rgba(r: f32, g: f32, b: f32, a: f32) -> Self {
        Self { r, g, b, a }
    }

    #[inline]
    pub const fn new_rgb(r: f32, g: f32, b: f32) -> Self {
        Self { r, g, b, a: 1.0 }
    }

    #[inline]
    pub const fn new_mono(val: f32) -> Self {
        Self {
            r: val,
            g: val,
            b: val,
            a: 1.0,
        }
    }

    #[inline]
    pub const fn into_u32(self) -> u32 {
        u32::from_be_bytes([
            0,
            (self.r * 255.0) as u8,
            (self.g * 255.0) as u8,
            (self.b * 255.0) as u8,
        ])
    }

    #[inline]
    pub const fn into_u32_alpha(self) -> u32 {
        u32::from_be_bytes([
            (self.r * 255.0) as u8,
            (self.g * 255.0) as u8,
            (self.b * 255.0) as u8,
            (self.a * 255.0) as u8,
        ])
    }

    #[inline]
    pub fn random<R: Rng + ?Sized>(rng: &mut R) -> Self {
        Color::new_rgb(rng.gen(), rng.gen(), rng.gen())
    }

    #[inline]
    pub fn random_bright<R: Rng + ?Sized>(rng: &mut R) -> Self {
        let phase_a: f32 = rng.gen_range(0.0..2.0 * std::f32::consts::PI);
        const PHASE_OFFS: f32 = 2.0 / 3.0 * std::f32::consts::PI;
        let phase_b = phase_a + PHASE_OFFS;
        let phase_c = phase_b + PHASE_OFFS;
        let a = phase_a.sin() * 0.5 + 0.5;
        let b = phase_b.sin() * 0.5 + 0.5;
        let c = phase_c.sin() * 0.5 + 0.5;
        Color::new_rgb(a, b, c)
    }
}

impl Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if f.alternate() {
            write!(f, "{:06x}", self.into_u32())
        } else {
            write!(f, "{:08x}", self.into_u32_alpha())
        }
    }
}

impl Add<f32> for Color {
    type Output = Color;

    fn add(mut self, rhs: f32) -> Self::Output {
        self.r += rhs;
        self.g += rhs;
        self.b += rhs;
        self.a += rhs;
        self
    }
}

impl Sub<f32> for Color {
    type Output = Color;

    fn sub(mut self, rhs: f32) -> Self::Output {
        self.r -= rhs;
        self.g -= rhs;
        self.b -= rhs;
        self.a -= rhs;
        self
    }
}

impl Mul<f32> for Color {
    type Output = Color;

    fn mul(mut self, rhs: f32) -> Self::Output {
        self.r *= rhs;
        self.g *= rhs;
        self.b *= rhs;
        self.a *= rhs;
        self
    }
}

impl Div<f32> for Color {
    type Output = Color;

    fn div(mut self, rhs: f32) -> Self::Output {
        self.r /= rhs;
        self.g /= rhs;
        self.b /= rhs;
        self.a /= rhs;
        self
    }
}

impl AddAssign<f32> for Color {
    fn add_assign(&mut self, rhs: f32) {
        *self = self.add(rhs);
    }
}

impl SubAssign<f32> for Color {
    fn sub_assign(&mut self, rhs: f32) {
        *self = self.sub(rhs);
    }
}

impl MulAssign<f32> for Color {
    fn mul_assign(&mut self, rhs: f32) {
        *self = self.mul(rhs);
    }
}

impl DivAssign<f32> for Color {
    fn div_assign(&mut self, rhs: f32) {
        *self = self.div(rhs);
    }
}

impl Distribution<Color> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Color {
        Color::random(rng)
    }
}

// ---------
// From/Into
// ---------

impl FromStr for Color {
    type Err = HexColorError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let split = s.split('#');
        let last: &str = split.last().ok_or(Self::Err::EmptyStr)?;

        let hex_number = u32::from_str_radix(last, 16).map_err(Self::Err::InvalidHexFormat)?;
        Ok(Self::from(hex_number))
    }
}

impl From<u32> for Color {
    fn from(hex: u32) -> Self {
        Self::from(hex.to_be_bytes())
    }
}

impl From<[u8; 4]> for Color {
    fn from(arr: [u8; 4]) -> Self {
        Self {
            r: arr[0] as f32 / 255.0,
            g: arr[1] as f32 / 255.0,
            b: arr[2] as f32 / 255.0,
            a: arr[3] as f32 / 255.0,
        }
    }
}

impl From<[u8; 3]> for Color {
    fn from(arr: [u8; 3]) -> Self {
        Self {
            r: arr[0] as f32 / 255.0,
            g: arr[1] as f32 / 255.0,
            b: arr[2] as f32 / 255.0,
            a: 1.0,
        }
    }
}

impl From<[f32; 4]> for Color {
    fn from(arr: [f32; 4]) -> Self {
        Self {
            r: arr[0],
            g: arr[1],
            b: arr[2],
            a: arr[3],
        }
    }
}

impl From<[f32; 3]> for Color {
    fn from(arr: [f32; 3]) -> Self {
        Self {
            r: arr[0],
            g: arr[1],
            b: arr[2],
            a: 1.0,
        }
    }
}

impl From<Color> for u32 {
    fn from(col: Color) -> Self {
        col.into_u32()
    }
}

impl From<Color> for wgpu::Color {
    fn from(color: Color) -> Self {
        Self {
            r: color.r as _,
            g: color.g as _,
            b: color.b as _,
            a: color.a as _,
        }
    }
}
