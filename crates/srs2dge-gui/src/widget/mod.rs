use self::prelude::WidgetBase;
use crate::prelude::{GuiCalcOffset, GuiCalcSize, GuiEvent, GuiGraphics};
use core::fmt;
use srs2dge_core::{glam::Vec2, main_game_loop::prelude::WindowState, target::Target};
use std::any::Any;

//

pub mod base;
pub mod drag_zone;
pub mod empty;
pub mod fill;
pub mod grid;
pub mod prelude;
pub mod root;
pub mod text;
pub mod trigger;

//

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct WidgetLayout {
    pub size: Vec2,
    pub offset: Vec2,
}

impl WidgetLayout {
    pub fn from_window_state(ws: &WindowState) -> Self {
        let size = ws.size;
        Self {
            size: Vec2::new(size.width as _, size.height as _),
            offset: Vec2::ZERO,
        }
    }

    pub fn contains(&self, point: Vec2) -> bool {
        self.offset.cmple(point).all() && point.cmplt(self.offset + self.size).all()
    }
}

//

pub trait Widget<T = ()> {
    fn debug(&self, name: &'static str, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(name)
    }

    fn into_widget(self) -> WidgetBase<T>
    where
        Self: Sized + 'static,
    {
        WidgetBase::new(self)
    }

    fn into_widget_with<I>(self, subwidgets: I) -> WidgetBase<T>
    where
        Self: Sized + 'static,
        I: IntoIterator<Item = WidgetBase<T>>,
    {
        WidgetBase::new_with(self, subwidgets)
    }

    fn into_widget_with_vec(self, subwidgets: Vec<WidgetBase<T>>) -> WidgetBase<T>
    where
        Self: Sized + 'static,
    {
        WidgetBase::new_with_vec(self, subwidgets)
    }

    fn layout(
        &mut self,
        parent: WidgetLayout,
        size: &GuiCalcSize,
        offset: &GuiCalcOffset,
    ) -> WidgetLayout {
        let mut layout = WidgetLayout::default();
        layout.size = size.reduce(&(parent, Vec2::ZERO));
        layout.offset = offset.reduce(&(parent, layout.size));
        layout
    }

    #[allow(unused_variables)]
    fn event(&mut self, state: &mut T, layout: WidgetLayout, event: GuiEvent) -> GuiEvent {
        event
    }

    fn event_recurse(
        &mut self,
        subwidgets: &mut [WidgetBase<T>],
        state: &mut T,
        layout: WidgetLayout,
        mut event: GuiEvent,
    ) -> GuiEvent {
        for widget in subwidgets.iter_mut().rev() {
            event = widget.event(state, layout, event);
        }

        // log::debug!("event: {}", self.name);
        self.event(state, layout, event)
    }

    #[allow(unused_variables)]
    fn draw(&mut self, gui: &mut GuiGraphics, target: &Target, layout: WidgetLayout) {}

    fn draw_recurse(
        &mut self,
        subwidgets: &mut [WidgetBase<T>],
        gui: &mut GuiGraphics,
        target: &Target,
        layout: WidgetLayout,
    ) {
        // log::debug!("draw: {}", self.name);
        self.draw(gui, target, layout);

        for widget in subwidgets {
            widget.draw(gui, target, layout);
        }
    }

    fn as_any(&self) -> &dyn Any;
}
