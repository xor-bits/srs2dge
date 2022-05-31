use super::{Widget, WidgetBase};
use crate::Gui;
use srs2dge_core::{color::Color, glam::Vec2, prelude::QuadMesh};

//

pub struct Fill {
    base: WidgetBase,
}

//

impl Widget for Fill {
    fn base(&self) -> WidgetBase {
        self.base
    }
}

impl Fill {
    pub fn new<FSize, FOffset>(
        parent: &dyn Widget,
        size: FSize,
        offset: FOffset,
        col: Color,
        gui: &mut Gui,
    ) -> Self
    where
        FSize: FnOnce(WidgetBase) -> Vec2,
        FOffset: FnOnce(WidgetBase, Vec2) -> Vec2,
    {
        let base = WidgetBase::new(parent, size, offset);

        gui.batcher.push_with(QuadMesh {
            pos: base.offset + 0.5 * base.size,
            size: base.size,
            col,
            tex: Default::default(),
        });

        Self { base }
    }
}
