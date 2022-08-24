use super::{Widget, WidgetLayout};
use crate::prelude::{GuiEvent, GuiEventTy, PointerState};
use srs2dge_core::{glam::Vec2, winit::event::MouseButton};
use std::{any::Any, fmt::Debug};

//

#[derive(Clone)]
pub struct DragZone<T> {
    blocking: bool,
    button: Option<MouseButton>,
    dragging: bool,

    on_drag: Vec<Handler<T>>,
    on_release: Vec<Handler<T>>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum DragZoneFilter {
    /// Pointer is dragging from the initial area
    #[default]
    OnDrag,

    /// A dragging pointer was released
    OnRelease,
}

pub type Handler<T> = Box<dyn FnMut(&mut T, Vec2)>;

pub trait HandlerB<I, O> {
    fn call(&mut self, i: I) -> O;

    fn clone(&self) -> Box<dyn HandlerB<I, O>>;
}

impl<A, O, F: FnMut(A) -> O + Clone + 'static> HandlerB<(A,), O> for F {
    fn call(&mut self, i: (A,)) -> O {
        (self)(i.0)
    }
    fn clone(&self) -> Box<dyn HandlerB<(A,), O>> {
        Box::new(self.clone())
    }
}

impl<A, B, O, F: FnMut(A, B) -> O + Clone + 'static> HandlerB<(A, B), O> for F {
    fn call(&mut self, i: (A, B)) -> O {
        (self)(i.0, i.1)
    }
    fn clone(&self) -> Box<dyn HandlerB<(A, B), O>> {
        Box::new(self.clone())
    }
}

//

impl<T> DragZone<T> {
    pub fn new() -> Self {
        Self {
            blocking: true,
            button: Some(MouseButton::Left),
            dragging: false,

            on_drag: vec![],
            on_release: vec![],
        }
    }

    pub fn with_blocking(mut self, blocking: bool) -> Self {
        self.blocking = blocking;
        self
    }

    pub fn with_button(mut self, button: Option<MouseButton>) -> Self {
        self.button = button;
        self
    }

    pub fn with_handler<F: FnMut(&mut T, Vec2) + 'static>(
        mut self,
        filter: DragZoneFilter,
        handler: F,
    ) -> Self {
        match filter {
            DragZoneFilter::OnDrag => &mut self.on_drag,
            DragZoneFilter::OnRelease => &mut self.on_release,
        }
        .push(Box::new(handler));
        self
    }
}

impl<T: 'static> Widget<T> for DragZone<T> {
    fn event(&mut self, state: &mut T, layout: WidgetLayout, mut event: GuiEvent) -> GuiEvent {
        match &event.ty {
            GuiEventTy::Pointer(PointerState::Hold {
                button,
                initial,
                now,
                ..
            }) => {
                let check = self.button.map(|btn| btn == *button).unwrap_or(false)
                    && (layout.contains(*initial) || self.dragging);
                self.dragging = check;

                if check {
                    let delta = *now - *initial;
                    for handler in self.on_drag.iter_mut() {
                        (handler)(state, delta);
                    }

                    event.blocked |= self.blocking;
                }
            }
            GuiEventTy::Pointer(PointerState::Release {
                button,
                initial,
                now,
            }) => {
                if self.dragging && self.button.map(|btn| btn == *button).unwrap_or(false) {
                    let delta = *now - *initial;
                    for handler in self.on_release.iter_mut() {
                        (handler)(state, delta);
                    }

                    self.dragging = false;
                }

                event.blocked |= self.blocking && layout.contains(*now);
            }
            GuiEventTy::Pointer(PointerState::Press { now, .. }) => {
                event.blocked |= self.blocking && layout.contains(*now);
            }
            GuiEventTy::Pointer(PointerState::Hover { now }) => {
                event.blocked |= self.blocking && layout.contains(*now);
            }
            _ => {}
        }

        event
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn clone_widget(&self) -> Box<dyn Widget<T>> {
        Box::new(self.clone())
    }
}

impl<T> Debug for DragZone<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("DragZone")
            .field("blocking", &self.blocking)
            .field("button", &self.button)
            .field("on_drag", &self.on_drag.len())
            .finish()
    }
}

impl<T> Default for DragZone<T> {
    fn default() -> Self {
        Self::new()
    }
}
