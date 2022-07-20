use crate::prelude::{GuiCalc, WidgetLayout};
use srs2dge_core::glam::Vec2;

//

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct Var(u16);

impl GuiCalc for Var {
    fn reduce(&self, _: &(WidgetLayout, Vec2)) -> Vec2 {
        todo!()
    }
}
