use crate::{
    gui::geom::GuiGeom,
    prelude::{Gui, GuiDraw, GuiLayout, Widget, WidgetBuilder, WidgetCore, WidgetLayout},
    style::{Style, StyleSheet},
};
use srs2dge_core::{
    color::Color,
    prelude::{QuadMesh, TexturePosition},
};
use std::any::{type_name, Any};
use taffy::prelude::Node;

//

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Fill {
    pub col: Color,
    pub tex: TexturePosition,

    core: WidgetCore,
}

//

impl Fill {
    pub fn new(gui: &mut Gui, style: Style, children: &[Node]) -> Result<Self, taffy::Error> {
        Ok(Self {
            col: style.widget.color.unwrap_or_default(),
            tex: style.widget.texture.unwrap_or_default(),
            core: WidgetCore::new(gui, style.layout, children)?,
        })
    }

    pub fn with_texture(mut self, tex: TexturePosition) -> Self {
        self.tex = tex;
        self
    }

    pub fn with_color(mut self, col: Color) -> Self {
        self.col = col;
        self
    }
}

impl Widget for Fill {
    fn draw(
        &mut self,
        parent_layout: WidgetLayout,
        gui_layout: &mut GuiLayout,
        draw: &mut GuiDraw,
    ) -> Result<(), taffy::Error> {
        let layout = gui_layout.get(self)?.to_absolute(parent_layout);

        // log::debug!("fill {layout:?}");

        draw.graphics
            .texture_batcher
            .push_with(GuiGeom::Quad(QuadMesh::new_top_left(
                layout.offset,
                layout.size,
                self.col,
                self.tex,
            )));

        Ok(())
    }

    fn name(&self) -> &'static str {
        type_name::<Self>()
    }

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

impl WidgetBuilder for Fill {
    fn build(
        gui: &mut Gui,
        style: Style,
        _: &StyleSheet,
        children: &[Node],
    ) -> Result<Self, taffy::Error> {
        Self::new(gui, style, children)
    }
}
