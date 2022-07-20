use super::{Widget, WidgetLayout};
use crate::{
    gui::geom::GuiGeom,
    prelude::{GuiGraphics, GuiValue},
};
use srs2dge_core::{glam::Vec2, prelude::QuadMesh, target::Target};
use srs2dge_text::prelude::{
    FormatChar, FormatString, TextAlign, TextChar, TextChars, TextConfig, XOrigin, YOrigin,
};
use std::any::Any;

//

#[derive(Debug, Clone)]
pub struct Text<'a> {
    text: FormatString<'a>,
    config: GuiValue<TextConfig>,
}

impl<'a> Text<'a> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_text<S: Into<FormatString<'a>>>(mut self, text: S) -> Self {
        self.text = text.into();
        self
    }

    pub fn with_config<T: Into<GuiValue<TextConfig>>>(mut self, config: T) -> Self {
        self.config = config.into();
        self
    }

    pub fn default_config() -> TextConfig {
        TextConfig {
            align: TextAlign::centered(),
            ..Default::default()
        }
    }
}

impl<'a, T> Widget<T> for Text<'a>
where
    Self: 'static,
{
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn draw(&mut self, gui: &mut GuiGraphics, target: &Target, layout: WidgetLayout) {
        let mut config = *self.config.get();
        config.sdf = gui.glyphs.is_sdf();
        config.x_origin = layout.offset.x
            + match config.align.x {
                XOrigin::Left => 0.0,
                XOrigin::Middle => layout.size.x * 0.5,
                XOrigin::Right => layout.size.x,
            };
        config.y_origin = layout.offset.y
            + match config.align.y {
                YOrigin::Baseline => layout.size.y,
                YOrigin::Top => layout.size.y,
                YOrigin::Bottom => 0.0,
                YOrigin::Middle => layout.size.y * 0.5,
            };

        // queue glyphs
        for FormatChar { character, format } in self.text.chars() {
            gui.glyphs.queue(character, format.px as _, format.font);
        }
        gui.glyphs.flush(target).unwrap();

        // generate text quads
        for TextChar {
            index,
            format,
            x,
            y,
            width,
            height,
            ..
        } in TextChars::new(self.text.chars(), gui.glyphs.fonts(), config)
        {
            let tex = gui
                .glyphs
                .get_indexed(index, format.px as _, format.font)
                .unwrap();

            if let Some(quad) = QuadMesh::new_top_left(
                Vec2::new(x, y),
                Vec2::new(width as _, height as _),
                format.color,
                tex,
            )
            .clip(layout.offset, layout.offset + layout.size)
            {
                gui.text_batcher.push_with(GuiGeom::Quad(quad));
            }
        }
    }
}

impl<'a> Default for Text<'a> {
    fn default() -> Self {
        Self {
            text: Default::default(),
            config: GuiValue::Owned(Self::default_config()),
        }
    }
}
