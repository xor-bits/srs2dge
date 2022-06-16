use super::{
    base::{WidgetBase, WidgetBaseBuilder},
    Widget,
};
use crate::{
    gui::{geom::GuiGeom, Gui},
    impl_base_widget, impl_base_widget_builder_methods,
};
use srs2dge_core::{
    color::Color,
    glam::Vec2,
    prelude::{QuadMesh, TexturePosition},
};

//

type W = Fill;
type Wb = FillBuilder;

//

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Fill {
    base: WidgetBase,
}

#[derive(Debug, Default)]
pub struct FillBuilder {
    base: WidgetBaseBuilder,
    col: Color,
    tex: TexturePosition,
}

//

impl W {
    pub fn builder() -> Wb {
        Wb::default()
    }
}

impl Wb {
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

    pub fn build(self, gui: &mut Gui) -> W {
        let Self { base, col, tex } = self;

        let base = base.build();

        gui.texture_batcher
            .push_with(GuiGeom::Quad(QuadMesh::new_top_left(
                base.offset,
                base.size,
                col,
                tex,
            )));

        W { base }
    }
}

//

impl_base_widget! { base W }
impl_base_widget_builder_methods! { base Wb  }
