use wgpu::Extent3d;

//

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Rect {
    pub width: u32,
    pub height: u32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct PositionedRect {
    pub x: u32,
    pub y: u32,

    pub width: u32,
    pub height: u32,
}

//

impl Rect {
    #[inline]
    pub const fn new(width: u32, height: u32) -> Self {
        Self { width, height }
    }

    #[inline]
    pub const fn positioned(self, x: u32, y: u32) -> PositionedRect {
        PositionedRect {
            x,
            y,
            width: self.width,
            height: self.height,
        }
    }
}

impl PositionedRect {
    #[inline]
    pub const fn new(x: u32, y: u32, width: u32, height: u32) -> Self {
        Self {
            x,
            y,
            width,
            height,
        }
    }

    #[inline]
    pub const fn rect(self) -> Rect {
        Rect {
            width: self.width,
            height: self.height,
        }
    }
}

//

impl From<Rect> for (u32, u32) {
    fn from(rect: Rect) -> Self {
        (rect.width, rect.height)
    }
}

impl From<Rect> for (usize, usize) {
    fn from(rect: Rect) -> Self {
        (rect.width as _, rect.height as _)
    }
}

impl From<(u32, u32)> for Rect {
    fn from((width, height): (u32, u32)) -> Self {
        Self { width, height }
    }
}

impl From<(usize, usize)> for Rect {
    fn from((width, height): (usize, usize)) -> Self {
        Self {
            width: width as _,
            height: height as _,
        }
    }
}

impl From<Rect> for Extent3d {
    fn from(rect: Rect) -> Self {
        Self {
            width: rect.width,
            height: rect.height,
            depth_or_array_layers: 1,
        }
    }
}
