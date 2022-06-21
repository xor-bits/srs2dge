use super::{
    base::{WidgetBase, WidgetBaseBuilder},
    Widget, WidgetBuilder,
};
use crate::gui::Gui;
use srs2dge_core::glam::Vec2;

//

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct DragZone {
    base: WidgetBase,
}

impl Widget for DragZone {
    fn base(&self) -> WidgetBase {
        self.base
    }
}

impl DragZone {
    pub fn builder<'a>() -> DragZoneBuilder<'a> {
        DragZoneBuilder::new()
    }
}

//

#[derive(Debug, Clone, Copy, Default)]
pub struct DragZoneBuilder<'a> {
    base: WidgetBaseBuilder<'a>,
}

//

impl<'a> DragZoneBuilder<'a> {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn build(self, gui: &mut Gui, state: &mut DragZoneState) -> DragZone {
        let Self { base } = self;

        let base = base.build();

        if let Some((area_before_dragging, (_, initial, now))) =
            state.dragging.and_then(|area_before_dragging| {
                Some((
                    area_before_dragging,
                    gui.dragged(area_before_dragging).next()?,
                ))
            })
        {
            // continue dragging
            state.dragging = Some(area_before_dragging);
            state.drag = now - initial;
        } else if let Some((_, initial, now)) = gui.dragged(base).next() {
            // begin dragging
            state.dragging = Some(base);
            state.drag = now - initial;
        } else {
            // release
            state.dragging = None;
            state.current += state.drag;
            state.drag = Vec2::ZERO;
        };

        DragZone { base }
    }
}

impl<'a> WidgetBuilder<'a> for DragZoneBuilder<'a> {
    fn inner(&self) -> &WidgetBaseBuilder<'a> {
        &self.base
    }

    fn inner_mut(&mut self) -> &mut WidgetBaseBuilder<'a> {
        &mut self.base
    }
}

//

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct DragZoneState {
    pub dragging: Option<WidgetBase>,
    pub drag: Vec2,
    pub current: Vec2,
}

impl DragZoneState {
    pub fn get(&self) -> Vec2 {
        self.current + self.drag
    }
}
