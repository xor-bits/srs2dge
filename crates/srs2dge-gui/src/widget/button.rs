use super::{Widget, WidgetBase, WidgetBaseBuilder};
use crate::{gui::Gui, impl_base_widget, impl_base_widget_builder_methods};
use srs2dge_core::glam::Vec2;

//

type W = Button;
type Wb<'g> = ButtonBuilder<'g>;

//

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Button {
    base: WidgetBase,
    clicked: bool,
}

#[derive(Debug, Default)]
pub struct ButtonBuilder<'g> {
    base: WidgetBaseBuilder,
    gui: Option<&'g mut Gui>,
}

//

impl W {
    pub fn builder<'g>() -> Wb<'g> {
        Wb::default()
    }

    pub fn clicked(&self) -> bool {
        self.clicked
    }
}

impl<'g> Wb<'g> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_gui(mut self, gui: &'g mut Gui) -> Self {
        self.gui = Some(gui);
        self
    }

    pub fn build(self) -> W {
        let Self { base, gui } = self;

        let base = base.build();

        let clicked = gui.map(|gui| gui.clicked(base)).unwrap_or(false);

        W { base, clicked }
    }
}

//

impl_base_widget! { base W }
impl_base_widget_builder_methods! { base Wb <'g> }
