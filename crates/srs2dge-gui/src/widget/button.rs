use super::{
    base::{WidgetBase, WidgetBaseBuilder},
    Widget,
};
use crate::{gui::Gui, impl_base_widget, impl_base_widget_builder_methods};
use srs2dge_core::glam::Vec2;

//

type W = Button;
type Wb = ButtonBuilder;

//

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Button {
    base: WidgetBase,
    clicked: bool,
}

#[derive(Debug, Default)]
pub struct ButtonBuilder {
    base: WidgetBaseBuilder,
}

//

impl W {
    pub fn builder() -> Wb {
        Wb::default()
    }

    pub fn clicked(&self) -> bool {
        self.clicked
    }
}

impl Wb {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn build(self, gui: &mut Gui) -> W {
        let Self { base } = self;

        let base = base.build();

        let clicked = gui.clicked(base);

        W { base, clicked }
    }
}

//

impl_base_widget! { base W }
impl_base_widget_builder_methods! { base Wb  }
