use crate::prelude::{Widget, WidgetCore};
use std::any::Any;

//

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Root {
    pub core: WidgetCore,
}

//

impl Widget for Root {
    fn core(&self) -> &WidgetCore {
        &self.core
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_mut_any(&mut self) -> &mut dyn Any {
        self
    }
}
