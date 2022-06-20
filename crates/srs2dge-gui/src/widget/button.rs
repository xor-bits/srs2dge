use super::{
    base::{WidgetBase, WidgetBaseBuilder},
    Widget,
};
use crate::{
    gui::Gui,
    impl_base, impl_base_widget,
    prelude::{BaseOffset, BaseSize, GuiCalc},
};

//

type W = Button;
type Wb<T, U> = ButtonBuilder<T, U>;

//

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Button {
    base: WidgetBase,
    clicked: bool,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ButtonBuilder<T, U> {
    base: WidgetBaseBuilder<T, U>,
}

//

impl_base! {}

impl W {
    pub fn builder() -> Wb<BaseSize, BaseOffset> {
        Wb::new()
    }

    pub fn clicked(&self) -> bool {
        self.clicked
    }
}

impl Default for Wb<BaseSize, BaseOffset> {
    fn default() -> Self {
        Self {
            base: Default::default(),
        }
    }
}

impl Wb<BaseSize, BaseOffset> {
    pub fn new() -> Self {
        Self::default()
    }
}

impl<T, U> Wb<T, U> {
    impl_base_widget! { => }
}

impl<T, U> Wb<T, U>
where
    T: GuiCalc,
    U: GuiCalc,
{
    pub fn build(self, gui: &mut Gui) -> W {
        let Self { base } = self;

        let base = base.build();

        let clicked = gui.clicked(base).next().is_some();

        W { base, clicked }
    }
}
