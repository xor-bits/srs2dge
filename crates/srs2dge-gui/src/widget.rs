use crate::prelude::{Baked, GuiEvent, GuiGraphics, Ref, Style, StyleSheet, WidgetLayout};
use srs2dge_core::target::Target;
use std::{any::Any, borrow::Cow};

//

#[derive(Debug, Clone, Default)]
pub struct WidgetCore {
    pub style: Style<Baked>,
    pub layout: WidgetLayout,
}

pub struct GuiDraw<'a> {
    pub graphics: &'a mut GuiGraphics,
    pub target: &'a mut Target,
}

//

#[allow(unused_variables)]
pub trait Widget {
    /// Subwidgets are processed **before** the
    /// parent i.e. in backwards direction.
    fn event(&mut self, event: &mut GuiEvent) {}

    /// Subwidgets are processed **after** the
    /// parent i.e. in forward direction.
    fn draw(&mut self, draw: &mut GuiDraw) {}

    /// Subwidgets are processed **after** the
    /// parent i.e. in forward firection.
    fn layout(&mut self, parent_layout: WidgetLayout) {
        self.gen_layout(parent_layout);
    }

    /// General but potentially expensive
    /// iterable of subwidgets.
    fn subwidgets(&self) -> Cow<'_, [&'_ dyn Widget]> {
        Cow::Borrowed(&[])
    }

    /// The number of subwidgets
    fn len(&self) -> usize {
        0
    }

    /// Get subwidget with its index
    fn get(&self, index: usize) -> Option<&dyn Widget> {
        None
    }

    /// Get mutable subwidget with
    /// its index
    fn get_mut(&mut self, index: usize) -> Option<&mut dyn Widget> {
        None
    }

    /// returns a ref to the WidgetCore of this widget
    fn core(&self) -> &WidgetCore;

    /// returns a mut ref to the WidgetCore of this widget
    fn core_mut(&mut self) -> &mut WidgetCore;

    fn name(&self) -> &'static str {
        "GenericWidget"
    }

    fn as_any(&self) -> &dyn Any;

    fn as_mut_any(&mut self) -> &mut dyn Any;

    fn style(&self) -> &Style<Baked> {
        &self.core().style
    }

    fn style_mut(&mut self) -> &mut Style<Baked> {
        &mut self.core_mut().style
    }

    fn gen_layout(&mut self, parent_layout: WidgetLayout) {
        self.core_mut().layout = parent_layout.calc_with_style(self.style_mut());
    }
}

pub trait WidgetBuilder: Sized {
    fn build<'a>(style: Style<Ref<'a>>, stylesheet: &'a StyleSheet<'a>) -> Self;
}

#[allow(unused_variables)]
pub trait WidgetEventHandler {
    fn event_handler(&mut self, layout: WidgetLayout, event: &mut GuiEvent) {}
}

#[allow(unused_variables)]
pub trait WidgetDrawHandler {
    fn draw_handler(&mut self, layout: WidgetLayout, draw: &mut GuiDraw) {}
}

//

impl WidgetCore {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn new_root() -> Self {
        Self::new()
    }

    pub fn with_style(mut self, style: Style<Ref>) -> Self {
        self.style = style.into();
        self
    }
}
