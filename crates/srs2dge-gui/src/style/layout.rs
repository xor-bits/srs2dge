use super::{Baked, Style};
use srs2dge_core::{glam::Vec2, main_game_loop::state::window::WindowState};
use std::{any::Any, fmt::Debug};

//

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct WidgetLayout {
    pub size: Vec2,
    pub offset: Vec2,
}

#[derive(Debug, Clone, Default)]
pub enum Size {
    /// Default
    ///
    /// Same size as parent
    #[default]
    Inherit,

    /// 100% of the parent size
    /// - all borders
    Border {
        left: f32,
        right: f32,
        top: f32,
        bottom: f32,
    },

    /// 100% of the parent size clamped
    Max(Vec2),

    /* VerticalStack {
        stretch: f32,
    },

    HorizontalStack {
        stretch: f32,
    }, */
    /// Fixed pixel size
    ///
    /// relative to the parent
    PointsRel(Vec2),

    /// Fixed pixel size
    ///
    /// absolute pixel size
    PointsAbs(Vec2),

    /// Custom size calculator
    Calc(Box<dyn Calc<Size>>),
}

#[derive(Debug, Clone, Default)]
pub enum Offset {
    /// Default
    ///
    /// Same offset as parent
    #[default]
    Inherit,

    /// Parent offset + borders
    Border {
        left: f32,
        right: f32,
        top: f32,
        bottom: f32,
    },

    Centered,

    /// Fixed pixel offset
    ///
    /// relative to the parent
    PointsRel(Vec2),

    /// Fixed pixel offset
    ///
    /// absolute positioning
    PointsAbs(Vec2),

    /// Custom offset calculator
    Calc(Box<dyn Calc<Offset>>),
}

/* #[derive(PartialEq, Eq, PartialOrd, Ord, Hash)]
enum StretchId {
    Vertical(u8),
    Horizontal(u8),
} */

//

impl WidgetLayout {
    pub fn from_ws(ws: &WindowState) -> Self {
        Self {
            size: Vec2::new(ws.size.width as f32, ws.size.height as f32),
            offset: Vec2::ZERO,
        }
    }

    pub fn calc(self, size: &mut Size, offset: &mut Offset) -> Self {
        let size = size.calc(self);
        let offset = offset.calc(self, size);
        Self { size, offset }
    }

    pub fn calc_with_style(self, style: &mut Style<Baked>) -> Self {
        Self::calc(self, &mut style.size, &mut style.offset)
    }

    pub fn split_vertical(self, stretch: &'_ [f32]) -> impl Iterator<Item = Self> + '_ {
        let stretch_sum_inv = 1.0 / stretch.iter().copied().sum::<f32>();

        let mut y = 0.0;
        stretch.iter().copied().map(move |s| {
            let size_y = self.size.y * s * stretch_sum_inv;
            let off_y = y;
            y += size_y;
            Self {
                size: Vec2::new(self.size.x, size_y),
                offset: Vec2::new(self.offset.x, off_y),
            }
        })
    }

    pub fn split_horizontal(self, stretch: &'_ [f32]) -> impl Iterator<Item = Self> + '_ {
        let stretch_sum_inv = 1.0 / stretch.iter().copied().sum::<f32>();

        let mut x = 0.0;
        stretch.iter().copied().map(move |s| {
            let size_x = self.size.x * s * stretch_sum_inv;
            let off_x = x;
            x += size_x;
            Self {
                size: Vec2::new(size_x, self.size.y),
                offset: Vec2::new(off_x, self.offset.y),
            }
        })
    }
}

impl Size {
    pub fn borders(px: f32) -> Self {
        Self::Border {
            left: px,
            right: px,
            top: px,
            bottom: px,
        }
    }

    pub fn calc(&mut self, parent: WidgetLayout) -> Vec2 {
        match self {
            Self::Inherit => parent.size,
            Self::Border {
                left,
                right,
                top,
                bottom,
            } => parent.size - Vec2::new(*left + *right, *top + *bottom),
            Self::Max(c) => parent.size.min(*c),
            /* Self::VerticalStack { stretch } => todo!(),
            Self::HorizontalStack { stretch } => todo!(), */
            Self::PointsRel(c) => parent.size + *c,
            Self::PointsAbs(c) => *c,
            Self::Calc(c) => c.call((parent,)),
        }
        .floor()
    }
}

impl Offset {
    pub fn borders(px: f32) -> Self {
        Self::Border {
            left: px,
            right: px,
            top: px,
            bottom: px,
        }
    }

    pub fn calc(&mut self, parent: WidgetLayout, self_size: Vec2) -> Vec2 {
        match self {
            Self::Inherit => parent.offset,
            Self::Border { left, bottom, .. } => parent.offset + Vec2::new(*left, *bottom),
            Self::Centered => parent.offset + (parent.size - self_size) * 0.5,
            Self::PointsRel(c) => parent.offset + *c,
            Self::PointsAbs(c) => *c,
            Self::Calc(c) => c.call((parent, self_size)),
        }
        .floor()
    }
}

//

pub trait Calc<T: CalcInputs>: Any + 'static {
    fn call(&mut self, inputs: T::Inputs) -> Vec2;

    fn dyn_clone(&self) -> Box<dyn Calc<T>>;
}

pub trait CalcInputs {
    type Inputs;
}

impl<T: CalcInputs + 'static> Clone for Box<dyn Calc<T>> {
    fn clone(&self) -> Self {
        self.dyn_clone()
    }
}

impl<T: CalcInputs + 'static> Debug for Box<dyn Calc<T>> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("Custom").finish()
    }
}

impl<T: Clone + FnMut(WidgetLayout) -> Vec2 + 'static> Calc<Size> for T {
    fn call(&mut self, (parent,): <Size as CalcInputs>::Inputs) -> Vec2 {
        (self)(parent)
    }

    fn dyn_clone(&self) -> Box<dyn Calc<Size>> {
        Box::new(self.clone()) as _
    }
}

impl CalcInputs for Size {
    type Inputs = (WidgetLayout,);
}

impl<T: Clone + FnMut(WidgetLayout, Vec2) -> Vec2 + 'static> Calc<Offset> for T {
    fn call(&mut self, (parent, self_size): <Offset as CalcInputs>::Inputs) -> Vec2 {
        (self)(parent, self_size)
    }

    fn dyn_clone(&self) -> Box<dyn Calc<Offset>> {
        Box::new(self.clone()) as _
    }
}

impl CalcInputs for Offset {
    type Inputs = (WidgetLayout, Vec2);
}
