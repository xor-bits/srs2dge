use super::{
    base::{WidgetBase, WidgetBaseBuilder},
    Widget,
};
use crate::{
    gui::Gui,
    impl_base, impl_base_widget,
    prelude::{BaseOffset, BaseSize, GuiCalc},
};
use srs2dge_core::glam::Vec2;

//

type W = DragZone;
type Wb<T, U> = DragZoneBuilder<T, U>;
type Ws = DragZoneState;

//

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct DragZone {
    base: WidgetBase,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct DragZoneBuilder<T, U> {
    base: WidgetBaseBuilder<T, U>,
}

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct DragZoneState {
    pub dragging: Option<WidgetBase>,
    pub drag: Vec2,
    pub current: Vec2,
}

//

impl_base! {}

impl W {
    pub fn builder() -> Wb<BaseSize, BaseOffset> {
        Wb {
            base: WidgetBaseBuilder::new(),
        }
    }
}

impl Default for Wb<BaseSize, BaseOffset> {
    fn default() -> Self {
        Self {
            base: Default::default(),
        }
    }
}

impl Wb<BaseSize, BaseOffset> {
    pub fn new() -> Self {
        Self::default()
    }
}

impl<T, U> Wb<T, U> {
    impl_base_widget! { => }
}

impl<T, U> Wb<T, U>
where
    T: GuiCalc,
    U: GuiCalc,
{
    pub fn build(self, gui: &mut Gui, state: &mut Ws) -> W {
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

        W { base }
    }
}

impl Ws {
    pub fn get(&self) -> Vec2 {
        self.current + self.drag
    }
}
