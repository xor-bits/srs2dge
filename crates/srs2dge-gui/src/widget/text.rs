use super::{
    base::{WidgetBase, WidgetBaseBuilder},
    Widget,
};
use crate::{
    gui::{
        geom::{GuiGeom, GuiQuad},
        Gui,
    },
    impl_base_widget, impl_base_widget_builder_methods,
};
use srs2dge_core::glam::Vec2;

//

type W = Text;
type Wb<'g> = TextBuilder<'g>;

//

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Text {
    base: WidgetBase,
}

#[derive(Debug)]
pub struct TextBuilder<'g> {
    base: WidgetBaseBuilder,
    gui: Option<&'g mut Gui>,
}

//

impl W {
    pub fn builder<'g>() -> Wb<'g> {
        Wb::default()
    }
}

impl<'g> Default for Wb<'g> {
    fn default() -> Self {
        Self {
            base: Default::default(),
            px: 18.0,
            text: Default::default(), // FString::from_string("Text Label"),
            gui: Default::default(),
        }
    }
}

impl<'g> Wb<'g> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_px(mut self, px: f32) -> Self {
        self.px = px;
        self
    }

    pub fn with_text(mut self, text: FString) -> Self {
        self.text = text;
        self
    }

    pub fn with_gui(mut self, gui: &'g mut Gui) -> Self {
        self.gui = Some(gui);
        self
    }

    pub fn build(self) -> W {
        let Self {
            base,
            px,
            text,
            gui,
        } = self;

        let base = base.build();

        if let Some(gui) = gui {
            let iter = CharPositionIter::new(&text, gui.glyphs(), px, true);

            for x in iter {
                x.x;
            }

            text.chars();
        }

        W { base }
    }
}

//

impl_base_widget! { base W }
impl_base_widget_builder_methods! { base Wb <'g> }
