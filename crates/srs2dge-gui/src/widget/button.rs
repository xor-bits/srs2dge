use super::{Widget, WidgetBuilder};
use crate::{
    gui::Gui,
    prelude::{WidgetBase, WidgetBaseBuilder},
};

//

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Button {
    base: WidgetBase,
    clicked: bool,
}

impl Widget for Button {
    fn base(&self) -> WidgetBase {
        self.base
    }
}

impl Button {
    pub fn builder<'a>() -> ButtonBuilder<'a> {
        ButtonBuilder::new()
    }

    pub fn clicked(&self) -> bool {
        self.clicked
    }
}

//

#[derive(Debug, Clone, Copy, Default)]
pub struct ButtonBuilder<'a> {
    base: WidgetBaseBuilder<'a>,
}

//

impl<'a> ButtonBuilder<'a> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn build(self, gui: &mut Gui) -> Button {
        let Self { base } = self;

        let base = base.build();

        let clicked = gui.clicked(base).next().is_some();

        Button { base, clicked }
    }
}

impl<'a> WidgetBuilder<'a> for ButtonBuilder<'a> {
    fn inner(&self) -> &WidgetBaseBuilder<'a> {
        &self.base
    }

    fn inner_mut(&mut self) -> &mut WidgetBaseBuilder<'a> {
        &mut self.base
    }
}
