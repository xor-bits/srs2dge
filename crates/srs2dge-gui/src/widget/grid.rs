use super::{Widget, WidgetBase, WidgetBaseBuilder};
use crate::{impl_base_widget, impl_base_widget_builder_methods};
use srs2dge_core::glam::Vec2;

//

type W = Grid;
type Wb = GridBuilder;

//

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Grid {
    base: WidgetBase,

    cols: usize,
    rows: usize,
    // border: Vec2,
    // margin: Vec2,
    i: usize,
}

#[derive(Debug)]
pub struct GridBuilder {
    base: WidgetBaseBuilder,

    cols: usize,
    rows: usize,
    // border: Vec2,
    // margin: Vec2,
}

//

impl W {
    pub fn builder() -> Wb {
        Wb::default()
    }

    /* pub fn elements<F: FnOnce(WidgetBase) -> Vec2>(&mut self, size: F) -> WidgetBase {
        GridRow { base }
    } */
}

impl Iterator for W {
    type Item = WidgetBase;

    fn next(&mut self) -> Option<Self::Item> {
        let x = self.i % self.cols;
        let y = self.i / self.cols;

        if y == self.rows {
            // wrapped around
            return None;
        }

        // next pos
        self.i += 1;

        let mut base = self.base;
        base.size.x /= self.cols as f32;
        base.size.y /= self.rows as f32;
        base.offset.x += base.size.x * x as f32;
        base.offset.y += base.size.y * y as f32;

        Some(base)
    }
}

impl Default for Wb {
    fn default() -> Self {
        Self {
            base: Default::default(),
            cols: 3,
            rows: 3,
            // border: Default::default(),
            // margin: Default::default(),
        }
    }
}

impl Wb {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_columns(mut self, cols: usize) -> Self {
        self.cols = cols;
        self
    }

    pub fn with_rows(mut self, rows: usize) -> Self {
        self.rows = rows;
        self
    }

    /* pub fn with_border(mut self, border: Vec2) -> Self {
        self.border = border;
        self
    } */

    /* pub fn with_margin(mut self, margin: Vec2) -> Self {
        self.margin = margin;
        self
    } */

    pub fn build(self) -> W {
        let Self {
            base,
            cols,
            rows,
            // border,
            // margin,
        } = self;

        let base = base.build();

        W {
            base,
            cols,
            rows,
            // border,
            // margin,
            i: 0,
        }
    }
}

//

impl_base_widget! { base W }
impl_base_widget_builder_methods! { base Wb }
