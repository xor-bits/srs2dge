use srs2dge_core::{glam::Vec2, main_game_loop::prelude::WindowState};

//

pub mod empty;
pub mod fill;
pub mod prelude;
pub mod root;

//

pub trait Widget {
    fn base(&self) -> WidgetBase;
}

//

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct WidgetBase {
    pub size: Vec2,
    pub offset: Vec2,
}

//

impl WidgetBase {
    pub fn new_root(ws: &WindowState) -> Self {
        Self {
            size: Vec2::new(ws.size.width as f32, ws.size.height as f32),
            offset: Vec2::ZERO,
        }
    }

    pub fn new<FSize, FOffset>(parent: &dyn Widget, size: FSize, offset: FOffset) -> Self
    where
        FSize: FnOnce(WidgetBase) -> Vec2,
        FOffset: FnOnce(WidgetBase, Vec2) -> Vec2,
    {
        let base = parent.base();
        let size = size(base);
        let offset = offset(base, size);
        Self { size, offset }
    }
}

//

#[inline]
pub fn inherit_size(base: WidgetBase) -> Vec2 {
    base.size
}

#[inline]
pub fn inherit_offset(base: WidgetBase, _: Vec2) -> Vec2 {
    base.offset
}

#[inline]
pub fn align(base: WidgetBase, size: Vec2, side: Vec2) -> Vec2 {
    base.offset + (base.size - size) * side
}
