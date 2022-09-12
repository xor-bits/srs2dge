use srs2dge_core::glam::Vec2;
use std::{any::Any, fmt::Debug};

//

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Layout {
    pub size: Vec2,
    pub offset: Vec2,
}

#[derive(Debug, Clone, PartialEq, Default)]
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

    /// Fixed pixel size
    Points(Vec2),

    /// Custom size calculator
    Calc(Box<dyn Calc<Size>>),
}

#[derive(Debug, Clone, PartialEq, Default)]
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

    /// Custom offset calculator
    Calc(Box<dyn Calc<Offset>>),
}

//

impl Size {
    pub fn calc(&mut self, parent: Layout) -> Vec2 {
        match self {
            Size::Inherit => parent.size,
            &Self::Border {
                left,
                right,
                top,
                bottom,
            } => parent.size - Vec2::new(left + right, top + bottom),
            Size::Calc(c) => c.call((parent,)),
        }
    }
}

impl Offset {
    pub fn calc(&mut self, parent: Layout, self_size: Vec2) -> Vec2 {
        match self {
            Offset::Inherit => parent.offset,
            &Self::Border {
                left,
                right,
                top,
                bottom,
            } => parent.offset + Vec2::new(left, bottom),
            Offset::Calc(c) => c.call((parent, self_size)),
        }
    }
}

//

pub trait Calc<T: CalcInputs>: Debug + Any {
    fn call(&mut self, inputs: T::Inputs) -> Vec2;

    fn dyn_clone(&self) -> Box<dyn Calc<T>>;

    fn dyn_eq(&self, other: &dyn Calc<T>) -> bool;

    fn as_any(&self) -> &dyn Any {
        &self
    }
}

pub trait CalcInputs {
    type Inputs;
}

impl<T: CalcInputs> PartialEq for dyn Calc<T> {
    fn eq(&self, other: &Self) -> bool {
        self.dyn_eq(other)
    }
}

impl<T: CalcInputs> Clone for Box<dyn Calc<T>> {
    fn clone(&self) -> Self {
        self.dyn_clone()
    }
}

impl<T: Debug + Clone + PartialEq + FnMut(Layout) -> Vec2 + 'static> Calc<Size> for T {
    fn call(&mut self, (parent,): <Size as CalcInputs>::Inputs) -> Vec2 {
        (self)(parent)
    }

    fn dyn_clone(&self) -> Box<dyn Calc<Size>> {
        Box::new(self.clone()) as _
    }

    fn dyn_eq(&self, other: &dyn Calc<Size>) -> bool {
        if let Some(other) = other.as_any().downcast_ref::<T>() {
            self.eq(other)
        } else {
            false
        }
    }
}

impl CalcInputs for Size {
    type Inputs = (Layout,);
}

impl<T: Debug + Clone + PartialEq + FnMut(Layout, Vec2) -> Vec2 + 'static> Calc<Offset> for T {
    fn call(&mut self, (parent, self_size): <Offset as CalcInputs>::Inputs) -> Vec2 {
        (self)(parent, self_size)
    }

    fn dyn_clone(&self) -> Box<dyn Calc<Offset>> {
        Box::new(self.clone()) as _
    }

    fn dyn_eq(&self, other: &dyn Calc<Offset>) -> bool {
        if let Some(other) = other.as_any().downcast_ref::<T>() {
            self.eq(other)
        } else {
            false
        }
    }
}

impl CalcInputs for Offset {
    type Inputs = (Layout, Vec2);
}
