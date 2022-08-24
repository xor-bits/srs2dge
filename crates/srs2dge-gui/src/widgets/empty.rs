use super::Widget;
use std::any::Any;

//

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Empty;

//

impl Empty {
    pub fn new() -> Self {
        Self::default()
    }
}

impl<T> Widget<T> for Empty {
    fn as_any(&self) -> &dyn Any {
        self
    }
}
