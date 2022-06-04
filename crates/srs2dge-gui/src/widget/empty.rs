use super::{
    base::{WidgetBase, WidgetBaseBuilder},
    Widget,
};
use crate::{impl_base_widget, impl_base_widget_builder_methods};
use srs2dge_core::glam::Vec2;

//

type W = Empty;
type Wb = EmptyBuilder;

//

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Empty {
    base: WidgetBase,
}

#[derive(Debug, Default)]
pub struct EmptyBuilder {
    base: WidgetBaseBuilder,
}

//

impl W {
    pub fn builder() -> Wb {
        Wb::default()
    }
}

impl Wb {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn build(self) -> W {
        let Self { base } = self;

        let base = base.build();

        W { base }
    }
}

//

impl_base_widget! { base W }
impl_base_widget_builder_methods! { base Wb }
