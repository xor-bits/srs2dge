use super::{
    base::{WidgetBase, WidgetBaseBuilder},
    Widget, WidgetBuilder,
};

//

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Empty {
    base: WidgetBase,
}

impl Widget for Empty {
    fn base(&self) -> WidgetBase {
        self.base
    }
}

impl Empty {
    pub fn builder<'a>() -> EmptyBuilder<'a> {
        EmptyBuilder::new()
    }
}

//

#[derive(Debug, Clone, Copy, Default)]
pub struct EmptyBuilder<'a> {
    base: WidgetBaseBuilder<'a>,
}

//

impl<'a> EmptyBuilder<'a> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn build(self) -> Empty {
        let Self { base } = self;

        let base = base.build();

        Empty { base }
    }
}
impl<'a> WidgetBuilder<'a> for EmptyBuilder<'a> {
    fn inner(&self) -> &WidgetBaseBuilder<'a> {
        &self.base
    }

    fn inner_mut(&mut self) -> &mut WidgetBaseBuilder<'a> {
        &mut self.base
    }
}
