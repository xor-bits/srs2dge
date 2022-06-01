use core::fmt;
use srs2dge_core::{glam::Vec2, main_game_loop::prelude::WindowState};
use std::fmt::Debug;

//

pub mod button;
pub mod empty;
pub mod fill;
pub mod grid;
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

pub struct WidgetBaseBuilder {
    pub parent: WidgetBase,
    pub size: Box<dyn FnOnce(WidgetBase) -> Vec2>,
    pub offset: Box<dyn FnOnce(WidgetBase, Vec2) -> Vec2>,
}

//

impl Debug for WidgetBaseBuilder {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("WidgetBaseBuilder")
            .field("parent", &self.parent)
            .finish()
    }
}

impl Default for WidgetBaseBuilder {
    fn default() -> Self {
        Self {
            parent: WidgetBase::default(),
            size: Box::new(inherit_size),
            offset: Box::new(inherit_offset),
        }
    }
}

impl WidgetBaseBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_base(mut self, base: WidgetBase) -> Self {
        self.parent = base;
        self
    }

    pub fn with_parent(mut self, parent: &dyn Widget) -> Self {
        self.parent = parent.base();
        self
    }

    pub fn with_size_boxed(mut self, size: Box<dyn FnOnce(WidgetBase) -> Vec2>) -> Self {
        self.size = size;
        self
    }

    pub fn with_size<F: FnOnce(WidgetBase) -> Vec2 + 'static>(self, size: F) -> Self {
        self.with_size_boxed(Box::new(size))
    }

    pub fn with_offset_boxed(mut self, offset: Box<dyn FnOnce(WidgetBase, Vec2) -> Vec2>) -> Self {
        self.offset = offset;
        self
    }

    pub fn with_offset<F: FnOnce(WidgetBase, Vec2) -> Vec2 + 'static>(self, offset: F) -> Self {
        self.with_offset_boxed(Box::new(offset))
    }

    pub fn build(self) -> WidgetBase {
        let size = (self.size)(self.parent);
        let offset = (self.offset)(self.parent, size);
        WidgetBase { size, offset }
    }
}

impl WidgetBase {
    pub fn new_root(ws: &WindowState) -> Self {
        Self {
            size: Vec2::new(ws.size.width as f32, ws.size.height as f32),
            offset: Vec2::ZERO,
        }
    }

    pub fn new() -> Self {
        Self::builder().build()
    }

    pub fn builder() -> WidgetBaseBuilder {
        WidgetBaseBuilder::new()
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

#[inline]
pub fn border_size(base: WidgetBase, px: f32) -> Vec2 {
    (base.size - Vec2::ONE * 2.0 * px).max(Vec2::ZERO)
}

#[inline]
pub fn border_offset(base: WidgetBase, px: f32) -> Vec2 {
    base.offset + Vec2::ONE * px
}

//

#[macro_export]
macro_rules! impl_base_widget {
    ($base:ident $ty:tt) => {
        impl Widget for $ty {
            fn base(&self) -> WidgetBase {
                self.$base
            }
        }
    };
}

#[macro_export]
macro_rules! impl_base_widget_builder_methods {
    ($base:ident $ty:tt $($generics:tt)*) => {
        impl $($generics)* $ty $($generics)* {
            pub fn with_base(mut self, base: WidgetBase) -> Self {
                self.$base = self.$base.with_base(base);
                self
            }

            pub fn with_parent(mut self, parent: &dyn Widget) -> Self {
                self.$base = self.$base.with_parent(parent);
                self
            }

            pub fn with_size_boxed(mut self, size: Box<dyn FnOnce(WidgetBase) -> Vec2>) -> Self {
                self.$base = self.$base.with_size_boxed(size);
                self
            }

            pub fn with_size<F: FnOnce(WidgetBase) -> Vec2 + 'static>(mut self, size: F) -> Self {
                self.$base = self.$base.with_size(size);
                self
            }

            pub fn with_offset_boxed(
                mut self,
                offset: Box<dyn FnOnce(WidgetBase, Vec2) -> Vec2>,
            ) -> Self {
                self.$base = self.$base.with_offset_boxed(offset);
                self
            }

            pub fn with_offset<F: FnOnce(WidgetBase, Vec2) -> Vec2 + 'static>(
                mut self,
                offset: F,
            ) -> Self {
                self.$base = self.$base.with_offset(offset);
                self
            }
        }
    };
}
