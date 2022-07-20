use srs2dge_core::target::Target;

use super::{Widget, WidgetLayout};
use crate::prelude::{GuiCalcOffset, GuiCalcSize, GuiEvent, GuiGraphics, GuiValue};
use core::fmt;
use std::{any::type_name, fmt::Debug};

//

pub struct WidgetBase<T = ()> {
    widget: Box<dyn Widget<T>>,
    name: &'static str,
    subwidgets: Vec<WidgetBase<T>>,

    pub size: GuiValue<GuiCalcSize>,
    pub offset: GuiValue<GuiCalcOffset>,
    // layout: WidgetLayout,
}

//

impl<T> Debug for WidgetBase<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("WidgetBase")
            .field(
                "widget",
                &WidgetDebug {
                    name: self.name,
                    widget: self.widget.as_ref(),
                },
            )
            .field("subwidgets", &self.subwidgets)
            .finish()
    }
}

impl<T> WidgetBase<T> {
    pub fn new<W>(widget: W) -> Self
    where
        W: Widget<T> + 'static,
    {
        Self::new_with(widget, vec![])
    }

    pub fn new_with<W, I>(widget: W, subwidgets: I) -> Self
    where
        W: Widget<T> + 'static,
        I: IntoIterator<Item = WidgetBase<T>>,
    {
        Self::new_with_vec(widget, subwidgets.into_iter().collect())
    }

    pub fn new_with_vec<W>(widget: W, subwidgets: Vec<WidgetBase<T>>) -> Self
    where
        W: Widget<T> + 'static,
    {
        Self {
            widget: Box::new(widget),
            name: type_name::<W>(),
            subwidgets,

            size: Default::default(),
            offset: Default::default(),
            // layout: None,
        }
    }

    pub fn push(&mut self, subwidget: WidgetBase<T>) {
        self.subwidgets.push(subwidget)
    }

    pub fn extend<I: IntoIterator<Item = WidgetBase<T>>>(&mut self, subwidgets: I) {
        self.subwidgets.extend(subwidgets)
    }

    pub fn try_as<W>(&self) -> Option<&W>
    where
        W: Widget<T> + 'static,
    {
        self.widget.as_any().downcast_ref()
    }

    pub fn calculate_layout(&mut self, parent_layout: WidgetLayout) -> WidgetLayout {
        let layout = self
            .widget
            .layout(parent_layout, &self.size.get(), &self.offset.get());
        // layout.size = layout.size.floor();
        // layout.offset = layout.offset.floor();
        layout
    }

    pub fn event(
        &mut self,
        state: &mut T,
        parent_layout: WidgetLayout,
        event: GuiEvent,
    ) -> GuiEvent {
        let layout = self.calculate_layout(parent_layout);
        self.widget
            .event_recurse(&mut self.subwidgets[..], state, layout, event)
    }

    pub fn draw(&mut self, gui: &mut GuiGraphics, target: &Target, parent_layout: WidgetLayout) {
        let layout = self.calculate_layout(parent_layout);
        self.widget
            .draw_recurse(&mut self.subwidgets[..], gui, target, layout)
    }

    pub fn with_size<C: Into<GuiValue<GuiCalcSize>>>(mut self, size: C) -> Self {
        self.size = size.into();
        self
    }

    pub fn with_offset<C: Into<GuiValue<GuiCalcOffset>>>(mut self, offset: C) -> Self {
        self.offset = offset.into();
        self
    }
}

struct WidgetDebug<'a, T> {
    name: &'static str,
    widget: &'a dyn Widget<T>,
}

impl<'a, T> Debug for WidgetDebug<'a, T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.widget.debug(self.name, f)
    }
}
