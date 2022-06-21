use crate::prelude::{GuiCalc, WidgetBase};
use srs2dge_core::glam::Vec2;

//

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct Min<T: GuiCalc, U: GuiCalc> {
    pub a: T,
    pub b: U,
}

//

pub trait IntoMin<U>
where
    Self: GuiCalc + Sized,
    U: GuiCalc,
{
    fn min(self, other: U) -> Min<Self, U> {
        Min { a: self, b: other }
    }
}

//

impl<T: GuiCalc, U: GuiCalc> IntoMin<U> for T {}

impl<T: GuiCalc, U: GuiCalc> GuiCalc for Min<T, U> {
    fn reduce(&self, refs: &(WidgetBase, Vec2)) -> Vec2 {
        self.a.reduce(refs).min(self.b.reduce(refs))
    }
}
