use crate::prelude::{GuiCalc, WidgetBase};
use srs2dge_core::glam::Vec2;

//

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct Max<T: GuiCalc, U: GuiCalc> {
    pub a: T,
    pub b: U,
}

//

pub trait IntoMax<U>
where
    Self: GuiCalc + Sized,
    U: GuiCalc,
{
    fn max(self, other: U) -> Max<Self, U> {
        Max { a: self, b: other }
    }
}

//

impl<T: GuiCalc, U: GuiCalc> IntoMax<U> for T {}

impl<T: GuiCalc, U: GuiCalc> GuiCalc for Max<T, U> {
    fn reduce(&self, refs: &(WidgetBase, Vec2)) -> Vec2 {
        self.a.reduce(refs).max(self.b.reduce(refs))
    }
}
