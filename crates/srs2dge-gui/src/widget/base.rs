use super::WidgetBuilder;
use crate::prelude::{GuiCalc, INHERIT_OFFSET, INHERIT_SIZE};
use srs2dge_core::{glam::Vec2, main_game_loop::prelude::WindowState};
use std::fmt::Debug;

//

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct WidgetBase {
    pub size: Vec2,
    pub offset: Vec2,
}

impl WidgetBase {
    pub fn new_root(ws: &WindowState) -> Self {
        Self {
            size: Vec2::new(ws.size.width as f32, ws.size.height as f32),
            offset: Vec2::ZERO,
        }
    }

    pub fn new() -> Self {
        Self::builder().build()
    }

    pub fn builder<'a>() -> WidgetBaseBuilder<'a> {
        WidgetBaseBuilder::new()
    }
}

//

#[derive(Clone, Copy)]
pub struct WidgetBaseBuilder<'a> {
    pub base: WidgetBase,
    pub size: &'a dyn GuiCalc,
    pub offset: &'a dyn GuiCalc,
}

//

impl<'a> Debug for WidgetBaseBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("WidgetBaseBuilder")
            .field("base", &self.base)
            .finish()
    }
}

impl<'a> Default for WidgetBaseBuilder<'a> {
    fn default() -> Self {
        Self {
            base: Default::default(),
            size: &INHERIT_SIZE,
            offset: &INHERIT_OFFSET,
        }
    }
}

impl<'a> WidgetBuilder<'a> for WidgetBaseBuilder<'a> {
    fn inner(&self) -> &WidgetBaseBuilder<'a> {
        self
    }

    fn inner_mut(&mut self) -> &mut WidgetBaseBuilder<'a> {
        self
    }
}

impl<'a> WidgetBaseBuilder<'a> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn build(self) -> WidgetBase {
        let size = self.size.reduce(&(self.base, Vec2::ZERO));
        let offset = self.offset.reduce(&(self.base, size));
        WidgetBase { size, offset }
    }
}
