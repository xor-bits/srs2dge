use crate::prelude::{
    Const, GuiCalc, GuiCalcAdd, GuiCalcMul, GuiCalcOffset, GuiCalcSize, WidgetLayout,
};
use srs2dge_core::{color::Color, glam::Vec2, prelude::TexturePosition};
use srs2dge_text::prelude::{TextAlign, TextConfig, TextDirection};
use std::sync::Arc;

//

pub trait Lerp {
    fn get(&self, other: &Self, i: f32) -> Self
    where
        Self: Clone,
    {
        if i >= 0.5 {
            self.clone()
        } else {
            other.clone()
        }
    }
}

//

impl Lerp for u8 {
    fn get(&self, other: &Self, i: f32) -> Self {
        (*self as f32 * (1.0 - i) + *other as f32 * i) as u8
    }
}

impl Lerp for f32 {
    fn get(&self, other: &Self, i: f32) -> Self {
        self * (1.0 - i) + other * i
    }
}

impl Lerp for f64 {
    fn get(&self, other: &Self, i: f32) -> Self {
        self * (1.0 - i as f64) + other * i as f64
    }
}

impl Lerp for Vec2 {
    fn get(&self, other: &Self, i: f32) -> Self {
        self.lerp(*other, i)
    }
}

impl Lerp for Color {
    fn get(&self, other: &Self, i: f32) -> Self {
        self.lerp(*other, i)
    }
}

impl Lerp for TexturePosition {}

impl Lerp for GuiCalcSize {
    fn get(&self, other: &Self, i: f32) -> Self {
        GuiCalcSize::Other(Arc::new(GuiCalcAdd {
            lhs: GuiCalcMul {
                lhs: GuiCalcSizeWrap(self.clone()),
                rhs: Const(Vec2::splat(1.0 - i)),
            },
            rhs: GuiCalcMul {
                lhs: GuiCalcSizeWrap(other.clone()),
                rhs: Const(Vec2::splat(i)),
            },
        }))
    }
}

impl Lerp for GuiCalcOffset {
    fn get(&self, other: &Self, i: f32) -> Self {
        GuiCalcOffset::Other(Arc::new(GuiCalcAdd {
            lhs: GuiCalcMul {
                lhs: GuiCalcOffsetWrap(self.clone()),
                rhs: Const(Vec2::splat(1.0 - i)),
            },
            rhs: GuiCalcMul {
                lhs: GuiCalcOffsetWrap(other.clone()),
                rhs: Const(Vec2::splat(i)),
            },
        }))
    }
}

impl Lerp for TextConfig {
    fn get(&self, other: &Self, i: f32) -> Self
    where
        Self: Clone,
    {
        Self {
            x_origin: self.x_origin.get(&other.x_origin, i),
            y_origin: self.y_origin.get(&other.y_origin, i),
            align: self.align.get(&other.align, i),
            scale: self.scale.get(&other.scale, i),
            dir: self.dir.get(&other.dir, i),
            tab_width: self.tab_width.get(&other.tab_width, i),
            line_gap: self.line_gap.get(&other.line_gap, i),
            sdf: self.sdf.get(&other.sdf, i),
        }
    }
}

impl Lerp for TextAlign {}
impl Lerp for TextDirection {}
impl Lerp for Option<f32> {
    fn get(&self, other: &Self, i: f32) -> Self
    where
        Self: Clone,
    {
        if let (Some(a), Some(b)) = (self, other) {
            return Some(a.get(b, i));
        }

        if i >= 0.5 {
            *self
        } else {
            *other
        }
    }
}
impl Lerp for bool {}

//

struct GuiCalcSizeWrap(GuiCalcSize);

impl GuiCalc for GuiCalcSizeWrap {
    fn reduce(&self, refs: &(WidgetLayout, Vec2)) -> Vec2 {
        self.0.reduce(refs)
    }
}

struct GuiCalcOffsetWrap(GuiCalcOffset);

impl GuiCalc for GuiCalcOffsetWrap {
    fn reduce(&self, refs: &(WidgetLayout, Vec2)) -> Vec2 {
        self.0.reduce(refs)
    }
}
