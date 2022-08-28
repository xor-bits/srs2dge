use srs2dge_core::prelude::{Color, TexturePosition};
use std::{
    borrow::Cow,
    collections::HashMap,
    sync::atomic::{AtomicBool, Ordering},
};

//

pub use taffy::{
    self,
    prelude::{Node, Number, Rect as LayoutRect, Size},
    style::{
        AlignContent, AlignItems, AlignSelf, Dimension, Display, FlexDirection, FlexWrap,
        JustifyContent, PositionType,
    },
};

//

#[macro_export]
macro_rules! stylesheet {
    ($($rule_name:ident => {
        $($container_name:ident . $field_name:ident : $field_val:expr),* $(,)?
    })*) => {{
        let mut stylesheet = StyleSheet::new();
        $(
            let mut style = Style::default();
            $(style . $container_name . $field_name = Some($field_val);)*
            stylesheet.insert(stringify!($rule_name), style);
        )*

        stylesheet
    }};
}

#[macro_export]
macro_rules! stylesheet_entry {
    ($rule_name:ident => {
        $($field_name:ident : $field_val:expr),*
    }) => {};
}

//

/// This struct mirrors the [`taffy::prelude::Style`]
/// but fields are optional.
/// None fields get the default value after processing.
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct LayoutStyle {
    /// What layout strategy should be used?
    pub display: Option<Display>,
    /// What should the `position` value of this struct use as a base offset?
    pub position_type: Option<PositionType>,
    /// Which direction does the main axis flow in?
    pub flex_direction: Option<FlexDirection>,
    /// Should elements wrap, or stay in a single line?
    pub flex_wrap: Option<FlexWrap>,
    /// How should items be aligned relative to the cross axis?
    pub align_items: Option<AlignItems>,
    /// Should this item violate the cross axis alignment specified by its parent's [`AlignItems`]?
    pub align_self: Option<AlignSelf>,
    /// How should content contained within this item be aligned relative to the cross axis?
    pub align_content: Option<AlignContent>,
    /// How should items be aligned relative to the main axis?
    pub justify_content: Option<JustifyContent>,
    /// How should the position of this element be tweaked relative to the layout defined?
    pub position: Option<LayoutRect<Dimension>>,
    /// How large should the margin be on each side?
    pub margin: Option<LayoutRect<Dimension>>,
    /// How large should the padding be on each side?
    pub padding: Option<LayoutRect<Dimension>>,
    /// How large should the border be on each side?
    pub border: Option<LayoutRect<Dimension>>,
    /// The relative rate at which this item grows when it is expanding to fill space
    ///
    /// 1.0 is the default value, and this value must be positive.
    pub flex_grow: Option<f32>,
    /// The relative rate at which this item shrinks when it is contracting to fit into space
    ///
    /// 1.0 is the default value, and this value must be positive.
    pub flex_shrink: Option<f32>,
    /// Sets the initial main axis size of the item
    pub flex_basis: Option<Dimension>,
    /// Sets the initial size of the item
    pub size: Option<Size<Dimension>>,
    /// Controls the minimum size of the item
    pub min_size: Option<Size<Dimension>>,
    /// Controls the maximum size of the item
    pub max_size: Option<Size<Dimension>>,
    /// Sets the preferred aspect ratio for the item
    ///
    /// The ratio is calculated as width divided by height.
    pub aspect_ratio: Option<Number>,
}

/// Common style settings for widgets
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct WidgetStyle {
    pub color: Option<Color>,
    pub background_color: Option<Color>,
    pub texture: Option<TexturePosition>,
}

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Style {
    pub widget: WidgetStyle,
    pub layout: LayoutStyle,
}

#[derive(Debug, Default)]
pub struct StyleSheet<'a> {
    map: HashMap<Cow<'a, str>, (AtomicBool, Style)>,
}

//

pub trait MergeStyles: Sized {
    fn merge(self, other: Self) -> Self;
}

//

impl Style {
    pub fn merge_from_styles(self, styles: &StyleSheet, name: &str) -> Self {
        if let Some(other) = styles.get(name) {
            self.merge(*other)
        } else {
            log::warn!("StyleSheet has no style for the name '{name}', using None as fallback.");
            self
        }
    }
}

impl<'a> StyleSheet<'a> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn insert<N: Into<Cow<'a, str>>, V: Into<Style>>(&mut self, name: N, value: V) {
        self.map
            .insert(name.into(), (AtomicBool::new(false), value.into()));
    }

    pub fn get(&self, name: &str) -> Option<&Style> {
        self.map.get(name).map(|(used, s)| {
            used.store(true, Ordering::SeqCst);
            s
        })
    }

    pub fn check_unused(&self) -> impl Iterator<Item = &str> + '_ {
        self.map
            .iter()
            .filter(|(_, (used, _))| !used.load(Ordering::SeqCst))
            .map(|(name, _)| name.as_ref())
    }

    pub fn reset_unused(&self) {
        self.map
            .values()
            .for_each(|(used, _)| used.store(false, Ordering::SeqCst))
    }
}

impl LayoutStyle {
    pub fn convert(self) -> taffy::prelude::Style {
        let default = taffy::prelude::Style::default();
        taffy::prelude::Style {
            display: self.display.unwrap_or(default.display),
            position_type: self.position_type.unwrap_or(default.position_type),
            flex_direction: self.flex_direction.unwrap_or(default.flex_direction),
            flex_wrap: self.flex_wrap.unwrap_or(default.flex_wrap),
            align_items: self.align_items.unwrap_or(default.align_items),
            align_self: self.align_self.unwrap_or(default.align_self),
            align_content: self.align_content.unwrap_or(default.align_content),
            justify_content: self.justify_content.unwrap_or(default.justify_content),
            position: self.position.unwrap_or(default.position),
            margin: self.margin.unwrap_or(default.margin),
            padding: self.padding.unwrap_or(default.padding),
            border: self.border.unwrap_or(default.border),
            flex_grow: self.flex_grow.unwrap_or(default.flex_grow),
            flex_shrink: self.flex_shrink.unwrap_or(default.flex_shrink),
            flex_basis: self.flex_basis.unwrap_or(default.flex_basis),
            size: self.size.unwrap_or(default.size),
            min_size: self.min_size.unwrap_or(default.min_size),
            max_size: self.max_size.unwrap_or(default.max_size),
            aspect_ratio: self.aspect_ratio.unwrap_or(default.aspect_ratio),
        }
    }
}

impl MergeStyles for Style {
    fn merge(self, other: Self) -> Self {
        Self {
            widget: self.widget.merge(other.widget),
            layout: self.layout.merge(other.layout),
        }
    }
}

impl MergeStyles for LayoutStyle {
    fn merge(self, other: Self) -> Self {
        Self {
            display: self.display.or(other.display),
            position_type: self.position_type.or(other.position_type),
            flex_direction: self.flex_direction.or(other.flex_direction),
            flex_wrap: self.flex_wrap.or(other.flex_wrap),
            align_items: self.align_items.or(other.align_items),
            align_self: self.align_self.or(other.align_self),
            align_content: self.align_content.or(other.align_content),
            justify_content: self.justify_content.or(other.justify_content),
            position: self.position.or(other.position),
            margin: self.margin.or(other.margin),
            padding: self.padding.or(other.padding),
            border: self.border.or(other.border),
            flex_grow: self.flex_grow.or(other.flex_grow),
            flex_shrink: self.flex_shrink.or(other.flex_shrink),
            flex_basis: self.flex_basis.or(other.flex_basis),
            size: self.size.or(other.size),
            min_size: self.min_size.or(other.min_size),
            max_size: self.max_size.or(other.max_size),
            aspect_ratio: self.aspect_ratio.or(other.aspect_ratio),
        }
    }
}

impl MergeStyles for WidgetStyle {
    fn merge(self, other: Self) -> Self {
        Self {
            color: self.color.or(other.color),
            background_color: self.background_color.or(other.background_color),
            texture: self.texture.or(other.texture),
        }
    }
}

// impl Deref for StyleSheet {
//     type Target = HashMap<String, Style>;

//     fn deref(&self) -> &Self::Target {
//         &self.map
//     }
// }

// impl DerefMut for StyleSheet {
//     fn deref_mut(&mut self) -> &mut Self::Target {
//         &mut self.map
//     }
// }

impl From<LayoutStyle> for Style {
    fn from(layout: LayoutStyle) -> Self {
        Self {
            layout,
            ..Default::default()
        }
    }
}

impl From<WidgetStyle> for Style {
    fn from(widget: WidgetStyle) -> Self {
        Self {
            widget,
            ..Default::default()
        }
    }
}
