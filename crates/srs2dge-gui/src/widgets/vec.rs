use core::slice;

use taffy::prelude::Node;

use crate::{
    prelude::{Gui, GuiDraw, GuiEvent, GuiLayout, Widget, WidgetBuilder, WidgetCore, WidgetLayout},
    style::{Style, StyleSheet},
};

//

/// Widget for holding a dynamic number of
/// widgets of type `T`
#[derive(Debug, Clone, PartialEq)]
pub struct WidgetVec<T> {
    vec: Vec<T>,

    core: WidgetCore,
}

//

impl<T: Widget + 'static> WidgetVec<T> {
    pub fn new(gui: &mut Gui, style: Style, children: &[Node]) -> Result<Self, taffy::Error> {
        Ok(Self {
            vec: vec![],
            core: WidgetCore::new(gui, style.layout, children)?,
        })
    }

    pub fn new_with(gui: &mut Gui, style: Style, children: Vec<T>) -> Result<Self, taffy::Error> {
        let children_nodes = children.iter().map(|w| w.node()).collect::<Vec<_>>();
        Ok(Self {
            vec: children,
            core: WidgetCore::new(gui, style.layout, &children_nodes)?,
        })
    }

    pub fn push(&mut self, gui: &mut Gui, widget: T) -> Result<(), taffy::Error> {
        gui.layout_mut().add_child(self.node(), widget.node())?;
        self.vec.push(widget);
        Ok(())
    }

    pub fn pop(&mut self, gui: &mut Gui) -> Result<Option<T>, taffy::Error> {
        let widget = match self.vec.pop() {
            Some(w) => w,
            None => return Ok(None),
        };
        gui.layout_mut().remove_child(self.node(), widget.node())?;
        Ok(Some(widget))
    }

    pub fn insert(&mut self, gui: &mut Gui, index: usize, widget: T) -> Result<(), taffy::Error> {
        let mut children = gui.layout_mut().children(self.node())?;
        children.insert(index, widget.node());
        gui.layout_mut().set_children(self.node(), &children)?;

        self.vec.insert(index, widget);
        Ok(())
    }

    pub fn remove(&mut self, gui: &mut Gui, index: usize) -> Result<T, taffy::Error> {
        let mut children = gui.layout_mut().children(self.node())?;
        children.remove(index);
        gui.layout_mut().set_children(self.node(), &children)?;

        let widget = self.vec.remove(index);
        Ok(widget)
    }

    pub fn extend<I: IntoIterator<Item = T>>(
        &mut self,
        gui: &mut Gui,
        iter: I,
    ) -> Result<(), taffy::Error> {
        for widget in iter.into_iter() {
            self.push(gui, widget)?;
        }
        Ok(())
    }

    pub fn iter(&self) -> slice::Iter<'_, T> {
        self.vec.iter()
    }

    pub fn iter_mut(&mut self) -> slice::IterMut<'_, T> {
        self.vec.iter_mut()
    }

    pub fn get(&self, index: usize) -> Option<&T> {
        self.vec.get(index)
    }

    pub fn get_mut(&mut self, index: usize) -> Option<&mut T> {
        self.vec.get_mut(index)
    }
}

impl<T: Widget + 'static> Widget for WidgetVec<T> {
    fn event(
        &mut self,
        parent_layout: WidgetLayout,
        gui_layout: &mut GuiLayout,
        event: &mut GuiEvent,
    ) -> Result<(), taffy::Error> {
        let layout = gui_layout.get(self)?.to_absolute(parent_layout);
        for subwidget in self.vec.iter_mut() {
            subwidget.event(layout, gui_layout, event)?;
        }
        Ok(())
    }

    fn draw(
        &mut self,
        parent_layout: WidgetLayout,
        gui_layout: &mut GuiLayout,
        draw: &mut GuiDraw,
    ) -> Result<(), taffy::Error> {
        let layout = gui_layout.get(self)?.to_absolute(parent_layout);
        for subwidget in self.vec.iter_mut() {
            subwidget.draw(layout, gui_layout, draw)?;
        }
        Ok(())
    }

    fn core(&self) -> &WidgetCore {
        &self.core
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn as_mut_any(&mut self) -> &mut dyn std::any::Any {
        self
    }
}

impl<T: Widget + 'static> WidgetBuilder for WidgetVec<T> {
    fn build(
        gui: &mut Gui,
        style: Style,
        _: &StyleSheet,
        children: &[Node],
    ) -> Result<Self, taffy::Error> {
        Self::new(gui, style, children)
    }
}
