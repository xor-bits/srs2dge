use crate::prelude::{GuiCalc, WidgetBase};
use srs2dge_core::glam::Vec2;

//

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct BaseSize;

impl GuiCalc for BaseSize {
    fn reduce(self, base: WidgetBase, _: Vec2) -> Vec2 {
        base.size
    }
}

//

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct BaseOffset;

impl GuiCalc for BaseOffset {
    fn reduce(self, base: WidgetBase, _: Vec2) -> Vec2 {
        base.offset
    }
}

//

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct SelfSize;

impl GuiCalc for SelfSize {
    fn reduce(self, _: WidgetBase, self_size: Vec2) -> Vec2 {
        self_size
    }
}
