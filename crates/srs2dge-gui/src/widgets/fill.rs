use crate::{
    gui::geom::GuiGeom,
    prelude::{GuiDraw, Ref, Style, StyleSheet, Widget, WidgetBuilder, WidgetCore},
};
use srs2dge_core::prelude::QuadMesh;
use std::any::{type_name, Any};

//

#[derive(Debug, Clone, Default)]
pub struct Fill {
    core: WidgetCore,
}

//

impl Fill {
    pub fn new() -> Self {
        Self {
            core: WidgetCore::default(),
        }
    }

    pub fn with_style(mut self, style: Style<Ref>) -> Self {
        self.core.style = style.into();
        self
    }
}

impl Widget for Fill {
    fn draw(&mut self, draw: &mut GuiDraw) {
        // tracing::debug!("fill {layout:?}");

        draw.graphics
            .texture_batcher
            .push_with(GuiGeom::Quad(QuadMesh::new_top_left(
                self.core.layout.offset,
                self.core.layout.size,
                self.core.style.color,
                self.core.style.texture,
            )));
    }

    fn name(&self) -> &'static str {
        type_name::<Self>()
    }

    fn core(&self) -> &WidgetCore {
        &self.core
    }

    fn core_mut(&mut self) -> &mut WidgetCore {
        &mut self.core
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_mut_any(&mut self) -> &mut dyn Any {
        self
    }
}

impl WidgetBuilder for Fill {
    fn build(style: Style<Ref>, _: &StyleSheet) -> Self {
        Self::new().with_style(style)
    }
}
