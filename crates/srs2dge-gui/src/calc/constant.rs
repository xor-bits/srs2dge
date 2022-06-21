use crate::prelude::{GuiCalc, WidgetBase};
use srs2dge_core::glam::Vec2;

//

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Const(pub Vec2);

impl GuiCalc for Const {
    fn reduce(&self, _: &(WidgetBase, Vec2)) -> Vec2 {
        self.0
    }
}

impl From<Vec2> for Const {
    fn from(val: Vec2) -> Self {
        Self(val)
    }
}
