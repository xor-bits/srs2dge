use crate::prelude::Widget;
use srs2dge_core::glam::Vec2;
use std::{
    fmt::Debug,
    ops::{Deref, DerefMut},
};

//

#[derive(Default)]
pub struct GuiLayout {
    pub stretch: Taffy,
    pub height: f32,
}

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct WidgetLayout {
    pub offset: Vec2,
    pub size: Vec2,
}

//

impl GuiLayout {
    pub fn get<T: Widget + ?Sized>(&self, widget: &T) -> Result<WidgetLayout, taffy::Error> {
        let layout = self.stretch.layout(widget.node())?;

        Ok(WidgetLayout {
            offset: Vec2::new(
                layout.location.x,
                layout.location.y,
                // self.height - layout.location.y - layout.size.height,
            ),
            size: Vec2::new(layout.size.width, layout.size.height),
        })
    }

    // pub fn get_dyn(&self, widget: &dyn Widget) -> Result<WidgetLayout, taffy::Error> {}
}

impl WidgetLayout {
    pub fn to_absolute(mut self, parent_layout: Self) -> Self {
        self.offset += parent_layout.offset;
        self
    }
}

impl Debug for GuiLayout {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Layout").field("stretch", &"??").finish()
    }
}

impl Deref for GuiLayout {
    type Target = Taffy;

    fn deref(&self) -> &Self::Target {
        &self.stretch
    }
}

impl DerefMut for GuiLayout {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.stretch
    }
}
