use super::{Widget, WidgetLayout};
use crate::{
    gui::geom::GuiGeom,
    prelude::{GuiGraphics, GuiValue},
};
use srs2dge_core::{
    color::Color,
    prelude::{QuadMesh, TexturePosition},
    target::Target,
};
use std::{any::Any, borrow::Cow};

//

#[derive(Debug, Clone, Default)]
pub struct Fill {
    col: GuiValue<Color>,
    tex: GuiValue<TexturePosition>,
}

//

impl Fill {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_texture<T: Into<GuiValue<TexturePosition>>>(mut self, tex: T) -> Self {
        self.set_texture(tex);
        self
    }

    pub fn with_color<T: Into<GuiValue<Color>>>(mut self, col: T) -> Self {
        self.set_color(col);
        self
    }

    pub fn set_texture<T: Into<GuiValue<TexturePosition>>>(&mut self, tex: T) {
        self.tex = tex.into()
    }

    pub fn set_color<T: Into<GuiValue<Color>>>(&mut self, col: T) {
        self.col = col.into()
    }

    pub fn get_texture(&mut self) -> Cow<TexturePosition> {
        self.tex.get()
    }

    pub fn get_color(&mut self) -> Cow<Color> {
        self.col.get()
    }
}

impl<T> Widget<T> for Fill {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn draw(&mut self, gui: &mut GuiGraphics, _: &Target, layout: WidgetLayout) {
        gui.texture_batcher
            .push_with(GuiGeom::Quad(QuadMesh::new_top_left(
                layout.offset,
                layout.size,
                *self.get_color(),
                *self.get_texture(),
            )));
    }
}
