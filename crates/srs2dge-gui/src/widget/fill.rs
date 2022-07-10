use super::{
    base::{WidgetBase, WidgetBaseBuilder},
    Widget, WidgetBuilder,
};
use crate::gui::{geom::GuiGeom, Gui};
use srs2dge_core::{
    color::Color,
    prelude::{QuadMesh, TexturePosition},
};

//

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Fill {
    base: WidgetBase,
}

impl Widget for Fill {
    fn base(&self) -> WidgetBase {
        self.base
    }
}

impl<'a> Fill {
    pub fn builder() -> FillBuilder<'a> {
        FillBuilder::new()
    }
}

//

#[derive(Debug, Clone, Copy, Default)]
pub struct FillBuilder<'a> {
    base: WidgetBaseBuilder<'a>,
    col: Color,
    tex: TexturePosition,
}

impl<'a> FillBuilder<'a> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_texture(mut self, tex: TexturePosition) -> Self {
        self.tex = tex;
        self
    }

    pub fn with_color(mut self, col: Color) -> Self {
        self.col = col;
        self
    }

    pub fn build(self, gui: &mut Gui) -> Fill {
        let Self { base, col, tex } = self;

        let base = base.build();

        gui.texture_batcher
            .push_with(GuiGeom::Quad(QuadMesh::new_top_left(
                base.offset,
                base.size,
                col,
                tex,
            )));

        Fill { base }
    }
}

impl<'a> WidgetBuilder<'a> for FillBuilder<'a> {
    fn inner(&self) -> &WidgetBaseBuilder<'a> {
        &self.base
    }

    fn inner_mut(&mut self) -> &mut WidgetBaseBuilder<'a> {
        &mut self.base
    }
}
