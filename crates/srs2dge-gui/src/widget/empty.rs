use super::{Widget, WidgetBase};
use srs2dge_core::glam::Vec2;

//

pub struct Empty {
    base: WidgetBase,
}

//

impl Widget for Empty {
    fn base(&self) -> WidgetBase {
        self.base
    }
}

impl Empty {
    pub fn new<FSize, FOffset>(parent: &dyn Widget, size: FSize, offset: FOffset) -> Self
    where
        FSize: FnOnce(WidgetBase) -> Vec2,
        FOffset: FnOnce(WidgetBase, Vec2) -> Vec2,
    {
        Self {
            base: WidgetBase::new(parent, size, offset),
        }
    }
}
