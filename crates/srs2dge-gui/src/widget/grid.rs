use super::{
    base::{WidgetBase, WidgetBaseBuilder},
    Widget,
};
use crate::{
    impl_base, impl_base_widget,
    prelude::{BaseOffset, BaseSize, GuiCalc},
};

//

type W = Grid;
type Wb<T, U> = GridBuilder<T, U>;

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

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct GridBuilder<T, U> {
    base: WidgetBaseBuilder<T, U>,

    cols: usize,
    rows: usize,
    // border: Vec2,
    // margin: Vec2,
}

//

impl_base! {}

impl W {
    pub fn builder() -> Wb<BaseSize, BaseOffset> {
        Wb::new()
    }

    /* pub fn elements<F: FnOnce(WidgetBase) -> Vec2>(&mut self, size: F) -> WidgetBase {
        GridRow { base }
    } */
}

impl Default for Wb<BaseSize, BaseOffset> {
    fn default() -> Self {
        Self {
            base: Default::default(),
            cols: 3,
            rows: 3,
        }
    }
}

impl Wb<BaseSize, BaseOffset> {
    pub fn new() -> Self {
        Self::default()
    }
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

impl<T, U> Wb<T, U> {
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

    impl_base_widget! { cols, rows => }
}

impl<T, U> Wb<T, U>
where
    T: GuiCalc,
    U: GuiCalc,
{
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
