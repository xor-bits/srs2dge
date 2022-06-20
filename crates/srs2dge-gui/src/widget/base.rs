use super::Widget;
use crate::prelude::{inherit_offset, inherit_size, BaseOffset, BaseSize, GuiCalc};
use srs2dge_core::{glam::Vec2, main_game_loop::prelude::WindowState};
use std::fmt::Debug;

//

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct WidgetBase {
    pub size: Vec2,
    pub offset: Vec2,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct WidgetBaseBuilder<T, U> {
    pub base: WidgetBase,
    pub size: T,
    pub offset: U,
}

//

impl Default for WidgetBaseBuilder<BaseSize, BaseOffset> {
    fn default() -> Self {
        Self {
            base: Default::default(),
            size: inherit_size(),
            offset: inherit_offset(),
        }
    }
}

impl WidgetBaseBuilder<BaseSize, BaseOffset> {
    pub fn new() -> Self {
        Self::default()
    }
}

impl<T, U> WidgetBaseBuilder<T, U> {
    pub fn with_parent(mut self, parent: &dyn Widget) -> Self {
        self.base = parent.base();
        self
    }

    pub fn with_base(mut self, base: WidgetBase) -> Self {
        self.base = base;
        self
    }

    pub fn with_size<Tn>(self, size: Tn) -> WidgetBaseBuilder<Tn, U> {
        let Self { base, offset, .. } = self;
        WidgetBaseBuilder { base, size, offset }
    }

    pub fn with_offset<Un>(self, offset: Un) -> WidgetBaseBuilder<T, Un> {
        let Self { base, size, .. } = self;
        WidgetBaseBuilder { base, size, offset }
    }
}

impl<T, U> WidgetBaseBuilder<T, U>
where
    T: GuiCalc,
    U: GuiCalc,
{
    pub fn build(self) -> WidgetBase {
        let base = self.base;

        let size = self.size.reduce(base, Vec2::ZERO);
        let offset = self.offset.reduce(base, size);

        WidgetBase { size, offset }
    }
}

impl WidgetBase {
    pub fn new_root(ws: &WindowState) -> Self {
        Self {
            size: Vec2::new(ws.size.width as f32, ws.size.height as f32),
            offset: Vec2::ZERO,
        }
    }

    pub fn new() -> Self {
        Self::builder().build()
    }

    pub fn builder() -> WidgetBaseBuilder<BaseSize, BaseOffset> {
        WidgetBaseBuilder::new()
    }
}

//

#[macro_export]
macro_rules! impl_base {
    () => {
        impl Widget for W {
            fn base(&self) -> WidgetBase {
                self.base
            }
        }
    };
}

#[macro_export]
macro_rules! impl_base_widget {
    ($($fields:ident),* => $($generics:tt),*) => {
        pub fn with_parent(mut self, parent: &dyn Widget) -> Self {
            self.base = self.base.with_parent(parent);
            self
        }

        pub fn with_base(mut self, base: WidgetBase) -> Self {
            self.base = self.base.with_base(base);
            self
        }

        pub fn with_size<Tn>(self, size: Tn) -> Wb<$($generics,)* Tn, U> {
            let Self { $($fields,)* base } = self;
            Wb {
                $($fields,)*
                base: base.with_size(size),
            }
        }

        pub fn with_offset<Un>(self, offset: Un) -> Wb<$($generics,)* T, Un> {
            let Self { $($fields,)* base } = self;
            Wb {
                $($fields,)*
                base: base.with_offset(offset),
            }
        }
    };
}
