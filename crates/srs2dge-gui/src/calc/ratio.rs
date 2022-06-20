use crate::prelude::{GuiCalc, WidgetBase};
use srs2dge_core::{glam::Vec2, util::ForceAspectRatio};

//

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Default)]
pub struct Ratio<T: GuiCalc> {
    pub val: T,
    pub ratio: f32,
    pub with_x: bool,
}

//

pub trait IntoRatio
where
    Self: GuiCalc + Sized,
{
    fn force_ratio_with_x(self, ratio: f32) -> Ratio<Self> {
        Ratio {
            val: self,
            ratio,
            with_x: true,
        }
    }

    fn force_ratio_with_y(self, ratio: f32) -> Ratio<Self> {
        Ratio {
            val: self,
            ratio,
            with_x: false,
        }
    }
}

//

impl<T: GuiCalc> IntoRatio for T {}

impl<T: GuiCalc> GuiCalc for Ratio<T> {
    fn reduce(self, base: WidgetBase, self_size: Vec2) -> Vec2 {
        let mut val = self.val.reduce(base, self_size);
        if self.with_x {
            val = val.force_ratio_with_x(self.ratio);
        } else {
            val = val.force_ratio_with_y(self.ratio);
        }
        val
    }
}
