use super::{Widget, WidgetBase};
use srs2dge_core::main_game_loop::prelude::WindowState;

//

pub struct Root {
    base: WidgetBase,
}

//

impl Widget for Root {
    fn base(&self) -> WidgetBase {
        self.base
    }
}

impl Root {
    pub(crate) fn new(ws: &WindowState) -> Self {
        Self {
            base: WidgetBase::new_root(ws),
        }
    }
}
