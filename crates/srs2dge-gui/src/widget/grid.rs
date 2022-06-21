use super::{
    base::{WidgetBase, WidgetBaseBuilder},
    Widget, WidgetBuilder,
};

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

impl Widget for Grid {
    fn base(&self) -> WidgetBase {
        self.base
    }
}

impl Grid {
    pub fn builder<'a>() -> GridBuilder<'a> {
        GridBuilder::new()
    }

    /* pub fn elements<F: FnOnce(WidgetBase) -> Vec2>(&mut self, size: F) -> WidgetBase {
        GridRow { base }
    } */
}

impl Iterator for Grid {
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

//

#[derive(Debug, Clone, Copy)]
pub struct GridBuilder<'a> {
    base: WidgetBaseBuilder<'a>,

    cols: usize,
    rows: usize,
    // border: Vec2,
    // margin: Vec2,
}

//

impl<'a> GridBuilder<'a> {
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

    pub fn build(self) -> Grid {
        let Self {
            base,
            cols,
            rows,
            // border,
            // margin,
        } = self;

        let base = base.build();

        Grid {
            base,
            cols,
            rows,
            // border,
            // margin,
            i: 0,
        }
    }
}

impl<'a> Default for GridBuilder<'a> {
    fn default() -> Self {
        Self {
            base: Default::default(),
            cols: 3,
            rows: 3,
        }
    }
}

impl<'a> WidgetBuilder<'a> for GridBuilder<'a> {
    fn inner(&self) -> &WidgetBaseBuilder<'a> {
        &self.base
    }

    fn inner_mut(&mut self) -> &mut WidgetBaseBuilder<'a> {
        &mut self.base
    }
}
