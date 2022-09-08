use std::any::type_name;

use crate::prelude::*;

//

/// Widget for holding a dynamic number of
/// widgets of type `T`
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct WidgetArray<T, const LEN: usize> {
    array: [T; LEN],

    core: WidgetCore,
}

//

impl<T: WidgetBuilder + Widget + 'static, const LEN: usize> WidgetArray<T, LEN> {
    pub fn new(
        gui: &mut Gui,
        style: Style,
        stylesheet: &StyleSheet,
        children: &[Node],
    ) -> Result<Self, taffy::Error> {
        // TODO: array_try_map
        // let widgets = [()].try_map(|_|T::build(gui, style, stylesheet, children))?;
        let widgets: [T; LEN] = (0..LEN)
            .map(|_| T::build(gui, Style::default(), stylesheet, children))
            .collect::<Result<Vec<_>, _>>()?
            .try_into()
            .map_err(|_| ())
            .expect("0..LEN should have the length of LEN");

        Self::new_with(gui, style, children, widgets)
    }
}

impl<T: Widget + 'static, const LEN: usize> WidgetArray<T, LEN> {
    pub fn new_with(
        gui: &mut Gui,
        style: Style,
        children: &[Node],
        widgets: [T; LEN],
    ) -> Result<Self, taffy::Error> {
        let array = widgets;
        let core = WidgetCore::new(gui, style.layout, children)?;

        for widget in array.iter() {
            gui.layout_mut().add_child(core.node, widget.node())?;
        }

        Ok(Self { array, core })
    }
}

impl<T: Widget + 'static, const LEN: usize> Widget for WidgetArray<T, LEN> {
    fn event(
        &mut self,
        parent_layout: WidgetLayout,
        gui_layout: &mut GuiLayout,
        event: &mut GuiEvent,
    ) -> Result<(), taffy::Error> {
        let layout = gui_layout.get(self)?.to_absolute(parent_layout);
        for subwidget in self.array.iter_mut().rev() {
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
        for subwidget in self.array.iter_mut() {
            subwidget.draw(layout, gui_layout, draw)?;
        }
        Ok(())
    }

    fn subwidgets(&self) -> Vec<&dyn Widget> {
        self.array.iter().map(|w| w as &dyn Widget).collect()
    }

    fn name(&self) -> &'static str {
        type_name::<Self>()
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

impl<T: Widget + WidgetBuilder + 'static, const LEN: usize> WidgetBuilder for WidgetArray<T, LEN> {
    fn build(
        gui: &mut Gui,
        style: Style,
        stylesheet: &StyleSheet,
        children: &[Node],
    ) -> Result<Self, taffy::Error> {
        Self::new(gui, style, stylesheet, children)
    }
}
