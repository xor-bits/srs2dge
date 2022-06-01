use super::{Widget, WidgetBase, WidgetBaseBuilder};
use crate::{
    gui::{
        geom::{GuiGeom, GuiQuad},
        Gui,
    },
    impl_base_widget, impl_base_widget_builder_methods,
};
use srs2dge_core::{color::Color, glam::Vec2, prelude::TexturePosition};

//

type W = Fill;
type Wb<'g> = FillBuilder<'g>;

//

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Fill {
    base: WidgetBase,
}

#[derive(Debug, Default)]
pub struct FillBuilder<'g> {
    base: WidgetBaseBuilder,
    col: Color,
    tex: TexturePosition,
    gui: Option<&'g mut Gui>,
}

//

impl W {
    pub fn builder<'g>() -> Wb<'g> {
        Wb::default()
    }
}

impl<'g> Wb<'g> {
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

    pub fn with_gui(mut self, gui: &'g mut Gui) -> Self {
        self.gui = Some(gui);
        self
    }

    pub fn build(self) -> W {
        let Self {
            base,
            col,
            tex,
            gui,
        } = self;

        let base = base.build();

        if let Some(gui) = gui {
            gui.texture_batcher.push_with(GuiGeom::Quad(GuiQuad {
                pos: base.offset,
                size: base.size,
                col,
                tex,
            }));
        }

        W { base }
    }
}

//

impl_base_widget! { base W }
impl_base_widget_builder_methods! { base Wb <'g> }
