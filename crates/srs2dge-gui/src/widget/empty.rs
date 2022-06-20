use super::{
    base::{WidgetBase, WidgetBaseBuilder},
    Widget,
};
use crate::{
    impl_base, impl_base_widget,
    prelude::{BaseOffset, BaseSize, GuiCalc},
};

//

type W = Empty;
type Wb<T, U> = EmptyBuilder<T, U>;

//

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Empty {
    base: WidgetBase,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct EmptyBuilder<T, U> {
    base: WidgetBaseBuilder<T, U>,
}

//

impl_base! {}

impl W {
    pub fn builder() -> Wb<BaseSize, BaseOffset> {
        Wb::new()
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
    pub fn build(self) -> W {
        let Self { base } = self;

        let base = base.build();

        W { base }
    }
}
