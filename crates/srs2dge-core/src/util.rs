use glam::{Vec2, Vec3, Vec4};
use std::ops::Range;
use wgpu::PresentMode;

//

pub trait RemapRange
where
    Self: Sized,
{
    /// remap value from range `from` to range `to`
    ///
    /// ```
    /// # use srs2dge_core::util::RemapRange;
    /// assert_eq!(0.4.remap(0.0..1.0, 0.0..10.0), 4.0);
    /// assert_eq!(5.0.remap(0.0..10.0, 10.0..20.0), 15.0);
    /// ```
    fn remap(self, from: Range<Self>, to: Range<Self>) -> Self;
}

macro_rules! impl_remap_range {
    ($ty:tt) => {
        impl RemapRange for $ty {
            fn remap(self, from: Range<Self>, to: Range<Self>) -> Self {
                to.start + (self - from.start) * (to.end - to.start) / (from.end - from.start)
            }
        }
    };
}

impl_remap_range! { i8 }
impl_remap_range! { i16 }
impl_remap_range! { i32 }
impl_remap_range! { i64 }
impl_remap_range! { isize }

impl_remap_range! { u8 }
impl_remap_range! { u16 }
impl_remap_range! { u32 }
impl_remap_range! { u64 }
impl_remap_range! { usize }

impl_remap_range! { f32 }
impl_remap_range! { f64 }

impl_remap_range! { Vec2 }
impl_remap_range! { Vec3 }
impl_remap_range! { Vec4 }

//

pub trait ForceAspectRatio {
    fn force_ratio_with_x(self, ratio: f32) -> Self;
    fn force_ratio_with_y(self, ratio: f32) -> Self;
}

//

impl ForceAspectRatio for Vec2 {
    fn force_ratio_with_x(mut self, ratio: f32) -> Self {
        self.x = self.y * ratio;
        self
    }

    fn force_ratio_with_y(mut self, ratio: f32) -> Self {
        self.y = self.x / ratio;
        self
    }
}

//

pub fn present_mode_from_env() -> Option<PresentMode> {
    std::env::var("PRESENT_MODE")
        .as_deref()
        .map(str::to_lowercase)
        .ok()
        .as_deref()
        .and_then(present_mode_from_str)
}

pub fn present_mode_from_str(string: &str) -> Option<PresentMode> {
    match string.to_lowercase().trim() {
        "mailbox" | "mail" | "sync" | "mb" | "m" | "s" => Some(PresentMode::Mailbox),
        "fifo" | "f" => Some(PresentMode::Fifo),
        "immediate" | "nosync" | "im" | "i" => Some(PresentMode::Immediate),
        b => {
            log::warn!("unknown present mode string '{}'", b);
            None
        }
    }
}

//

#[macro_export]
macro_rules! unwrap_or_return {
    ($e:expr) => {
        match $e {
            Some(__some) => __some,
            None => return,
        }
    };
}
