use super::{Widget, WidgetLayout};
use crate::prelude::{GuiEvent, GuiEventTy, PointerState};
use srs2dge_core::winit::event::MouseButton;
use std::{any::Any, fmt::Debug};

//

pub struct Trigger<T> {
    blocking: bool,

    hovering: bool,

    on_hover: Vec<Handler<T>>,
    on_enter: Vec<Handler<T>>,
    on_exit: Vec<Handler<T>>,
    on_click: Vec<(Option<MouseButton>, Handler<T>)>,
    on_press: Vec<(Option<MouseButton>, Handler<T>)>,
    on_hold: Vec<(Option<MouseButton>, Handler<T>)>,
    on_release: Vec<(Option<MouseButton>, Handler<T>)>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum TriggerFilter {
    /// Pointer is hovering in the area
    OnHover,

    /// Pointer entered the area
    OnEnter,

    /// Pointer left the area
    OnExit,

    /// Any button was pressed and released
    /// in the area
    #[default]
    OnClick,

    /// Any button was pressed in the area
    OnPress,

    /// Any button was held down in the area
    ///
    /// The pointer may not still be in the area
    OnHold,

    /// Any button was released in the area
    ///
    /// The button may not have been pressed
    /// in the area
    OnRelease,

    /// The button was pressed and released
    /// in the area
    OnClickButton(MouseButton),

    /// The button was pressed in the area
    OnPressButton(MouseButton),

    /// The button was held down in the area
    ///
    /// The pointer may not still be in the area
    OnHoldButton(MouseButton),

    /// The button was released in the area
    ///
    /// The button may not have been pressed
    /// in the area
    OnReleaseButton(MouseButton),
}

pub type Handler<T> = Box<dyn FnMut(&mut T)>;

//

impl<T> Trigger<T> {
    pub fn new() -> Self {
        Self {
            blocking: true,

            hovering: false,

            on_hover: vec![],
            on_enter: vec![],
            on_exit: vec![],
            on_click: vec![],
            on_press: vec![],
            on_hold: vec![],
            on_release: vec![],
        }
    }

    pub fn with_blocking(mut self, blocking: bool) -> Self {
        self.blocking = blocking;
        self
    }

    pub fn with_handler<F: FnMut(&mut T) + 'static>(
        mut self,
        filter: TriggerFilter,
        handler: F,
    ) -> Self {
        let handler = Box::new(handler);

        match filter {
            TriggerFilter::OnHover => self.on_hover.push(handler),
            TriggerFilter::OnEnter => self.on_enter.push(handler),
            TriggerFilter::OnExit => self.on_exit.push(handler),
            TriggerFilter::OnClick => self.on_click.push((None, handler)),
            TriggerFilter::OnPress => self.on_press.push((None, handler)),
            TriggerFilter::OnHold => self.on_hold.push((None, handler)),
            TriggerFilter::OnRelease => self.on_release.push((None, handler)),
            TriggerFilter::OnClickButton(btn) => self.on_click.push((Some(btn), handler)),
            TriggerFilter::OnPressButton(btn) => self.on_press.push((Some(btn), handler)),
            TriggerFilter::OnHoldButton(btn) => self.on_hold.push((Some(btn), handler)),
            TriggerFilter::OnReleaseButton(btn) => self.on_release.push((Some(btn), handler)),
        }

        self
    }

    fn filter_iter<'a, I, Item>(
        iter: I,
        button: &'a MouseButton,
    ) -> impl Iterator<Item = &'a mut Item> + 'a
    where
        I: Iterator<Item = &'a mut (Option<MouseButton>, Item)> + 'a,
        Item: 'a,
    {
        iter.filter_map(move |(filter, handler)| match filter {
            Some(filter) if filter == button => Some(handler),
            None => Some(handler),
            _ => None,
        })
    }
}

impl<T: 'static> Widget<T> for Trigger<T> {
    fn event(&mut self, state: &mut T, layout: WidgetLayout, mut event: GuiEvent) -> GuiEvent {
        match &event.ty {
            GuiEventTy::Pointer(PointerState::Hover { now }) => {
                if layout.contains(*now) && !event.blocked {
                    // check if just changed
                    if !self.hovering {
                        // run handlers
                        self.hovering = true;
                        for handler in self.on_enter.iter_mut() {
                            (handler)(state)
                        }
                    }

                    for handler in self.on_hover.iter_mut() {
                        (handler)(state)
                    }

                    event.blocked = self.blocking;
                } else {
                    // check if just changed
                    if self.hovering {
                        self.hovering = false;
                        // run handlers
                        for handler in self.on_exit.iter_mut() {
                            (handler)(state)
                        }
                    }
                }
            }
            GuiEventTy::Pointer(PointerState::Press { button, now }) => {
                if layout.contains(*now) && !event.blocked {
                    Self::filter_iter(self.on_press.iter_mut(), button)
                        .for_each(|handler| (handler)(state));

                    event.blocked = self.blocking;
                }
            }
            GuiEventTy::Pointer(PointerState::Hold {
                button, initial, ..
            }) => {
                if layout.contains(*initial) && !event.blocked
                /* && layout.contains(*now) */
                {
                    Self::filter_iter(self.on_hold.iter_mut(), button)
                        .for_each(|handler| (handler)(state));

                    event.blocked = self.blocking;
                }
            }
            GuiEventTy::Pointer(PointerState::Release {
                button,
                initial,
                now,
            }) => {
                if layout.contains(*now) && !event.blocked {
                    Self::filter_iter(self.on_release.iter_mut(), button)
                        .for_each(|handler| (handler)(state));

                    if layout.contains(*initial) {
                        Self::filter_iter(self.on_click.iter_mut(), button)
                            .for_each(|handler| (handler)(state));
                    }

                    event.blocked = self.blocking;
                }
            }
            _ => {}
        }
        event
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl<T> Debug for Trigger<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Trigger")
            .field("blocking", &self.blocking)
            .field("on_hover", &self.on_hover.len())
            .field("on_click", &self.on_click.len())
            .field("on_press", &self.on_press.len())
            .field("on_hold", &self.on_hold.len())
            .field("on_release", &self.on_release.len())
            .finish()
    }
}

impl<T> Default for Trigger<T> {
    fn default() -> Self {
        Self::new()
    }
}
