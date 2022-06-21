use super::{
    base::{WidgetBase, WidgetBaseBuilder},
    Widget, WidgetBuilder,
};
use crate::gui::{geom::GuiGeom, Gui};
use srs2dge_core::{glam::Vec2, prelude::QuadMesh, target::Target};
use srs2dge_text::prelude::{
    FormatChar, FormatString, TextAlign, TextChar, TextChars, TextConfig, XOrigin, YOrigin,
};

//

#[derive(Debug, Clone, PartialEq, Default)]
pub struct Text {
    base: WidgetBase,
}

impl Widget for Text {
    fn base(&self) -> WidgetBase {
        self.base
    }
}

impl Text {
    pub fn builder<'a>() -> TextBuilder<'a> {
        TextBuilder::new()
    }
}

//

#[derive(Debug, Clone)]
pub struct TextBuilder<'a> {
    base: WidgetBaseBuilder<'a>,
    text: FormatString<'a>,
    config: TextConfig,
}

impl<'a> TextBuilder<'a> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_text<S: Into<FormatString<'a>>>(mut self, text: S) -> Self {
        self.text = text.into();
        self
    }

    pub fn with_config(mut self, config: TextConfig) -> Self {
        self.config = config;
        self
    }

    pub fn build(self, gui: &mut Gui, target: &Target) -> Text {
        let Self {
            base,
            text,
            mut config,
        } = self;
        let glyphs = &mut gui.glyphs;
        config.sdf = glyphs.is_sdf();

        // base widget
        let base = base.build();

        config.x_origin = base.offset.x
            + match config.align.x {
                XOrigin::Left => 0.0,
                XOrigin::Middle => base.size.x * 0.5,
                XOrigin::Right => base.size.x,
            };
        config.y_origin = base.offset.y
            + match config.align.y {
                YOrigin::Baseline => base.size.y,
                YOrigin::Top => base.size.y,
                YOrigin::Bottom => 0.0,
                YOrigin::Middle => base.size.y * 0.5,
            };

        // queue glyphs
        for FormatChar { character, format } in text.chars() {
            glyphs.queue(character, format.px as _, format.font);
        }
        glyphs.flush(target).unwrap();

        // generate text quads
        for TextChar {
            index,
            format,
            x,
            y,
            width,
            height,
            ..
        } in TextChars::new(text.chars(), glyphs.fonts(), config)
        {
            let tex = glyphs
                .get_indexed(index, format.px as _, format.font)
                .unwrap();

            if let Some(quad) = QuadMesh::new_top_left(
                Vec2::new(x as _, y as _),
                Vec2::new(width as _, height as _),
                format.color,
                tex,
            )
            .clip(base.offset, base.offset + base.size)
            {
                gui.text_batcher.push_with(GuiGeom::Quad(quad));
            }
        }

        Text { base }
    }
}

impl<'a> Default for TextBuilder<'a> {
    fn default() -> Self {
        Self {
            base: Default::default(),
            text: Default::default(),
            config: TextConfig {
                align: TextAlign::centered(),
                ..Default::default()
            },
        }
    }
}

impl<'a> WidgetBuilder<'a> for TextBuilder<'a> {
    fn inner(&self) -> &WidgetBaseBuilder<'a> {
        &self.base
    }

    fn inner_mut(&mut self) -> &mut WidgetBaseBuilder<'a> {
        &mut self.base
    }
}
