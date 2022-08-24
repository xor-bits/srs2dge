use crate::{
    gui::geom::GuiGeom,
    prelude::{Gui, GuiDraw, GuiLayout, Widget, WidgetBuilder, WidgetCore, WidgetLayout},
};
use srs2dge_core::{
    color::Color,
    prelude::{QuadMesh, TexturePosition},
};
use std::any::Any;
use taffy::{prelude::Node, style::Style};

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
            col: Default::default(),
            tex: Default::default(),
            core: WidgetCore::new(gui, style, children)?,
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
    fn build(gui: &mut Gui, style: Style) -> Result<Self, taffy::Error> {
        Ok(Self {
            col: Default::default(),
            tex: Default::default(),
            core: WidgetCore::new(gui, style, &[])?,
        })
    }
}
