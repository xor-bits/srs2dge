use self::prelude::{
    base::{BaseOffset, BaseSize, SelfSize},
    IntoMax,
};
use crate::prelude::{Const, WidgetLayout};
use srs2dge_core::glam::Vec2;
use std::sync::Arc;

//

pub mod base;
pub mod constant;
pub mod max;
pub mod min;
pub mod ops;
pub mod prelude;
pub mod ratio;
pub mod var;

//

pub trait GuiCalc {
    /// `base` has the size and the offset of the parent widget
    ///
    /// `self_size` is the size calculated for this widget
    /// `Vec2::ZERO` if it is currently being calculated
    fn reduce(&self, refs: &(WidgetLayout, Vec2)) -> Vec2;
}

//

pub const INHERIT_SIZE: BaseSize = BaseSize;
pub const INHERIT_OFFSET: BaseOffset = BaseOffset;

pub const fn inherit_size() -> BaseSize {
    INHERIT_SIZE
}

pub const fn inherit_offset() -> BaseOffset {
    INHERIT_OFFSET
}

pub fn align(side: Vec2) -> impl GuiCalc {
    BaseOffset + (BaseSize - SelfSize) * Const(side)
}

pub fn border_size(px: f32) -> impl GuiCalc {
    (BaseSize - Const(Vec2::splat(2.0 * px))).max(Const(Vec2::ZERO))
}

pub fn border_offset(px: f32) -> impl GuiCalc {
    BaseOffset + Const(Vec2::splat(px))
}

//

/// Prefer constructing `Base` or `Const`
/// variants directly instead of using `From`
#[derive(Default, Clone)]
pub enum GuiCalcSize {
    /// Default for size
    #[default]
    Base,

    /// Fast root widget
    Const(Const),

    /// Custom sizes
    Other(Arc<dyn GuiCalc>),
}

/// Prefer constructing `Base` or `Const`
/// variants directly instead of using `From`
#[derive(Default, Clone)]
pub enum GuiCalcOffset {
    /// Default for offset
    #[default]
    Base,

    /// Fast root widget
    Const(Const),

    /// Custom offsets
    Other(Arc<dyn GuiCalc>),
}

impl GuiCalcSize {
    pub fn reduce(&self, refs: &(WidgetLayout, Vec2)) -> Vec2 {
        match self {
            Self::Base => BaseSize.reduce(refs),
            Self::Const(v) => v.reduce(refs),
            Self::Other(v) => v.reduce(refs),
        }
    }
}

impl GuiCalcOffset {
    pub fn reduce(&self, refs: &(WidgetLayout, Vec2)) -> Vec2 {
        match self {
            Self::Base => BaseOffset.reduce(refs),
            Self::Const(v) => v.reduce(refs),
            Self::Other(v) => v.reduce(refs),
        }
    }
}

impl From<Arc<dyn GuiCalc>> for GuiCalcSize {
    fn from(o: Arc<dyn GuiCalc>) -> Self {
        Self::Other(o)
    }
}

impl From<Arc<dyn GuiCalc>> for GuiCalcOffset {
    fn from(o: Arc<dyn GuiCalc>) -> Self {
        Self::Other(o)
    }
}

impl<T: GuiCalc + 'static> From<T> for GuiCalcSize {
    fn from(o: T) -> Self {
        Self::Other(Arc::new(o))
    }
}

impl<T: GuiCalc + 'static> From<T> for GuiCalcOffset {
    fn from(o: T) -> Self {
        Self::Other(Arc::new(o))
    }
}
