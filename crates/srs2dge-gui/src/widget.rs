use crate::{
    prelude::{Gui, GuiEvent, GuiGraphics, GuiLayout, WidgetLayout},
    style::{LayoutStyle, Style, StyleSheet},
};
use srs2dge_core::target::Target;
use std::any::Any;
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

pub trait Widget {
    /// Subwidgets are processed **before** the
    /// parent i.e. in backwards direction.
    #[allow(unused_variables)]
    fn event(
        &mut self,
        parent_layout: WidgetLayout,
        gui_layout: &mut GuiLayout,
        event: &mut GuiEvent,
    ) -> Result<(), taffy::Error> {
        Ok(())
    }

    /// Subwidgets are processed **after** the
    /// parent i.e. in forward direction.
    #[allow(unused_variables)]
    fn draw(
        &mut self,
        parent_layout: WidgetLayout,
        gui_layout: &mut GuiLayout,
        draw: &mut GuiDraw,
    ) -> Result<(), taffy::Error> {
        Ok(())
    }

    /// returns a ref to the WidgetCore of this widget
    fn core(&self) -> &WidgetCore;

    fn as_any(&self) -> &dyn Any;

    fn as_mut_any(&mut self) -> &mut dyn Any;

    fn node(&self) -> Node {
        self.core().node
    }
}

pub trait WidgetBuilder: Sized {
    fn build(
        gui: &mut Gui,
        style: Style,
        stylesheet: &StyleSheet,
        children: &[Node],
    ) -> Result<Self, taffy::Error>;
}

pub trait WidgetEventHandler {
    #[allow(unused_variables)]
    fn event_handler(
        &mut self,
        widget_layout: WidgetLayout,
        gui_layout: &mut GuiLayout,
        event: &mut GuiEvent,
    ) -> Result<(), taffy::Error> {
        Ok(())
    }
}

pub trait WidgetDrawHandler {
    #[allow(unused_variables)]
    fn draw_handler(
        &mut self,
        widget_layout: WidgetLayout,
        gui_layout: &mut GuiLayout,
        draw: &mut GuiDraw,
    ) -> Result<(), taffy::Error> {
        Ok(())
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
