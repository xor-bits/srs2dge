use crate::prelude::GuiCalc;

use self::{base::WidgetBase, prelude::WidgetBaseBuilder};

//

pub mod base;
pub mod button;
pub mod drag_zone;
pub mod empty;
pub mod fill;
pub mod grid;
pub mod prelude;
pub mod root;
pub mod text;

//

pub trait Widget {
    fn base(&self) -> WidgetBase;
}

pub trait WidgetBuilder<'a>
where
    Self: Sized + 'a,
{
    fn inner(&self) -> &WidgetBaseBuilder<'a>;

    fn inner_mut(&mut self) -> &mut WidgetBaseBuilder<'a>;

    fn with_parent(mut self, parent: &dyn Widget) -> Self {
        self.inner_mut().base = parent.base();
        self
    }

    fn with_base(mut self, base: WidgetBase) -> Self {
        self.inner_mut().base = base;
        self
    }

    fn with_size(mut self, size: &'a dyn GuiCalc) -> Self {
        self.inner_mut().size = size;
        self
    }

    fn with_offset(mut self, offset: &'a dyn GuiCalc) -> Self {
        self.inner_mut().offset = offset;
        self
    }
}
