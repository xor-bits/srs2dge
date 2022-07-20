use super::Widget;
use std::any::Any;

//

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Root;

//

impl<T> Widget<T> for Root {
    fn as_any(&self) -> &dyn Any {
        self
    }
}
