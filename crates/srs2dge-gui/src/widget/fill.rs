use super::{
    base::{WidgetBase, WidgetBaseBuilder},
    Widget,
};
use crate::{
    gui::{geom::GuiGeom, Gui},
    impl_base, impl_base_widget,
    prelude::{BaseOffset, BaseSize, GuiCalc},
};
use srs2dge_core::{
    color::Color,
    prelude::{QuadMesh, TexturePosition},
};

//

type W = Fill;
type Wb<T, U> = FillBuilder<T, U>;

//

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Fill {
    base: WidgetBase,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct FillBuilder<T, U> {
    base: WidgetBaseBuilder<T, U>,
    col: Color,
    tex: TexturePosition,
}

//

impl_base! {}

impl W {
    pub fn builder() -> Wb<BaseSize, BaseOffset> {
        Wb::new()
    }
}

impl Default for Wb<BaseSize, BaseOffset> {
    fn default() -> Self {
        Self {
            base: Default::default(),
            col: Default::default(),
            tex: Default::default(),
        }
    }
}

impl Wb<BaseSize, BaseOffset> {
    pub fn new() -> Self {
        Self::default()
    }
}

impl<T, U> Wb<T, U> {
    pub fn with_texture(mut self, tex: TexturePosition) -> Self {
        self.tex = tex;
        self
    }

    pub fn with_color(mut self, col: Color) -> Self {
        self.col = col;
        self
    }

    impl_base_widget! { col, tex =>  }
}

impl<T, U> Wb<T, U>
where
    T: GuiCalc,
    U: GuiCalc,
{
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
