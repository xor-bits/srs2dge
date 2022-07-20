use super::{Widget, WidgetLayout};
use crate::prelude::{GuiEvent, GuiGraphics, WidgetBase};
use srs2dge_core::{glam::Vec2, target::Target};
use std::any::Any;

//

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Grid {
    pub cols: usize,
    pub rows: usize,
}

//

impl Grid {
    pub const fn new() -> Self {
        Self::with(3, 3)
    }

    pub const fn with(rows: usize, cols: usize) -> Self {
        Self { cols, rows }
    }

    pub const fn with_columns(mut self, cols: usize) -> Self {
        self.cols = cols;
        self
    }

    pub const fn with_rows(mut self, rows: usize) -> Self {
        self.rows = rows;
        self
    }

    pub fn new_grid<const ROWS: usize, const COLS: usize, T>(
        grid: [[WidgetBase<T>; COLS]; ROWS],
    ) -> WidgetBase<T> {
        Self::with(ROWS, COLS).into_widget_with(grid.into_iter().flatten())
    }

    pub fn new_row<const COLS: usize, T>(grid: [WidgetBase<T>; COLS]) -> WidgetBase<T> {
        Self::with(1, COLS).into_widget_with(grid)
    }

    fn layout_grid_init(&self, layout: WidgetLayout) -> WidgetLayout {
        WidgetLayout {
            size: layout.size / Vec2::new(self.cols as f32, self.rows as f32),
            offset: layout.offset,
        }
    }

    fn layout_grid_nth(&self, init: WidgetLayout, n: usize) -> WidgetLayout {
        let x = (n % self.cols) as f32;
        let y = (n / self.cols) as f32;

        WidgetLayout {
            size: init.size,
            offset: init.offset + init.size * Vec2::new(x, y),
        }
    }
}

impl<T> Widget<T> for Grid {
    fn event_recurse(
        &mut self,
        subwidgets: &mut [WidgetBase<T>],
        state: &mut T,
        layout: WidgetLayout,
        mut event: GuiEvent,
    ) -> GuiEvent {
        let init = self.layout_grid_init(layout);

        for (n, widget) in subwidgets.iter_mut().enumerate().rev() {
            event = widget.event(state, self.layout_grid_nth(init, n), event);
        }

        event
    }

    fn draw_recurse(
        &mut self,
        subwidgets: &mut [WidgetBase<T>],
        gui: &mut GuiGraphics,
        target: &Target,
        layout: WidgetLayout,
    ) {
        let init = self.layout_grid_init(layout);

        for (n, widget) in subwidgets.iter_mut().enumerate() {
            widget.draw(gui, target, self.layout_grid_nth(init, n));
        }
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl Default for Grid {
    fn default() -> Self {
        Self::new()
    }
}
