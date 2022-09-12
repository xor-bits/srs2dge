use crate::{
    prelude::{Gui, GuiEvent, GuiGraphics, GuiLayout, WidgetLayout},
    style::{LayoutStyle, Style, StyleSheet},
};
use srs2dge_core::target::Target;
use std::{any::Any, borrow::Cow};
use taffy::{
    prelude::{Node, Size},
    style::Dimension,
};

//

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct WidgetCore {
    pub node: Node,
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
    fn event(
        &mut self,
        parent_layout: WidgetLayout,
        gui_layout: &mut GuiLayout,
        event: &mut GuiEvent,
    ) {
    }

    /// Subwidgets are processed **after** the
    /// parent i.e. in forward direction.
    fn draw(
        &mut self,
        parent_layout: WidgetLayout,
        gui_layout: &mut GuiLayout,
        draw: &mut GuiDraw,
    ) {
    }

    /// General but potentially expensive
    /// iterable of the subwidgets.
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

    fn name(&self) -> &'static str {
        "GenericWidget"
    }

    fn as_any(&self) -> &dyn Any;

    fn as_mut_any(&mut self) -> &mut dyn Any;

    fn node(&self) -> Node {
        self.core().node
    }
}

pub trait WidgetBuilder: Sized {
    fn build(gui: &mut Gui, style: Style, stylesheet: &StyleSheet, children: &[Node]) -> Self;
}

#[allow(unused_variables)]
pub trait WidgetEventHandler {
    fn event_handler(
        &mut self,
        widget_layout: WidgetLayout,
        gui_layout: &mut GuiLayout,
        event: &mut GuiEvent,
    ) {
    }
}

#[allow(unused_variables)]
pub trait WidgetDrawHandler {
    fn draw_handler(
        &mut self,
        widget_layout: WidgetLayout,
        gui_layout: &mut GuiLayout,
        draw: &mut GuiDraw,
    ) {
    }
}

//

impl WidgetCore {
    /// WidgetCore constructor for normal widgets
    ///
    /// [`taffy::prelude::Node`] is aquired from a widget with:
    ///
    /// ```ignore
    /// let _: Node = subwidget.node();
    /// ```
    pub fn new(gui: &mut Gui, style: LayoutStyle, children: &[Node]) -> Result<Self, taffy::Error> {
        Ok(Self {
            node: gui.layout_mut().new_node(style.convert(), children)?,
        })
    }

    /// WidgetCore constructor for root widgets
    ///
    /// [`taffy::prelude::Node`] is aquired from a widget with:
    ///
    /// ```ignore
    /// let _: Node = subwidget.node();
    /// ```
    pub fn new_root(gui: &mut Gui, children: &[Node]) -> Result<Self, taffy::Error> {
        Self::new(gui, Self::root_style().layout, children)
    }

    pub fn root_style() -> Style {
        Style {
            layout: LayoutStyle {
                size: Some(Size {
                    width: Dimension::Percent(1.0),
                    height: Dimension::Percent(1.0),
                }),
                ..Default::default()
            },
            ..Default::default()
        }
    }
}
