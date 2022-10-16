use crate::prelude::{GuiDraw, GuiGeom, Ref, Style, StyleSheet, Widget, WidgetBuilder, WidgetCore};
use srs2dge_core::glam::Vec2;
use srs2dge_text::prelude::{FormatString, TextConfig};
use std::any::{type_name, Any};

//

#[derive(Debug, Clone, Default)]
pub struct Text<'a> {
    text: FormatString<'a>,
    config: TextConfig,
    core: WidgetCore,
}

//

impl<'a> Text<'a> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_style(mut self, style: Style<Ref>) -> Self {
        self.core.style = style.into();
        self
    }

    pub fn text(&mut self, text: FormatString<'a>) -> &mut Self {
        self.text = text;
        self
    }

    pub fn config(&mut self, config: TextConfig) -> &mut Self {
        self.config = config;
        self
    }
}

impl Widget for Text<'static> {
    fn draw(&mut self, draw: &mut GuiDraw) {
        // println!("{:?}", self.core.style);
        self.config.align = self.core.style.text_align;
        let align = self.config.align.to_vec2();
        let align = Vec2::new(align.x, 1.0 - align.y);

        let [x, y] = (self.core.layout.offset + self.core.layout.size * align).to_array();

        self.config.x_origin = x;
        self.config.y_origin = y;
        self.config.sdf = draw.graphics.glyphs.is_sdf();

        let glyph_quads = match srs2dge_text::vbo::text(
            &draw.target,
            self.text.chars(),
            &mut draw.graphics.glyphs,
            self.config,
        ) {
            Ok(glyph_quads) => glyph_quads,
            Err(err) => {
                log::warn!("Text widget draw error: {err}");
                return;
            }
        };

        glyph_quads.into_iter().for_each(|q| {
            draw.graphics.text_batcher.push_with(GuiGeom::Quad(q));
        });
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

impl<'a> WidgetBuilder for Text<'a> {
    fn build(style: Style<Ref>, _: &StyleSheet) -> Self {
        Self::new().with_style(style)
    }
}
