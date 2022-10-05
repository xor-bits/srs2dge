use std::{
    borrow::Cow,
    collections::HashMap,
    mem::swap,
    sync::atomic::{AtomicBool, Ordering},
};

use self::{
    layout::{Offset, Size},
    merge::MergeStyles,
};
use srs2dge_core::prelude::{Color, TexturePosition};

//

pub mod layout;
pub mod r#macro;
pub mod merge;
pub mod prelude;

//

/// Style with baked in values
///
/// Common style settings for widgets
#[derive(Debug, Clone, Default)]
pub struct BakedStyle {
    // visual styling
    /// Foreground colors
    ///
    /// Like text color or fill color
    pub color: Color,
    /// Texture position
    ///
    /// Used by fill
    ///
    /// TODO: Texture manager
    pub texture: TexturePosition,

    // layout styling
    /// Size calculator of the widget
    pub size: Size,
    /// Offset calculator of the widget
    pub offset: Offset,
}

/// Style with optional fields
/// to allow merging
///
/// Common style settings for widgets
#[derive(Debug, Clone, Default)]
pub struct Style {
    // visual styling
    /// Foreground colors
    ///
    /// Like text color or fill color
    pub color: Option<Color>,
    /// Texture position
    ///
    /// Used by fill
    ///
    /// TODO: Texture manager
    pub texture: Option<TexturePosition>,

    // layout styling
    /// Size calculator of the widget
    pub size: Option<Size>,
    /// Offset calculator of the widget
    pub offset: Option<Offset>,
    /*/// Allocated size with multiple
    /// subwidgets is:
    /// `stretch / stretch_sum * size`
    ///
    /// Offset is also affected
    pub stretch: Option<f32>,*/
}

/// Ref to a mergeable style [`MStyle`]
///
/// Common style settings for widgets
#[derive(Debug, Clone, Default)]
pub struct StyleRef<'a> {
    // visual styling
    /// Foreground colors
    ///
    /// Like text color or fill color
    pub color: Option<&'a Color>,
    /// Texture position
    ///
    /// Used by fill
    ///
    /// TODO: Texture manager
    pub texture: Option<&'a TexturePosition>,

    // layout styling
    /// Size calculator of the widget
    pub size: Option<&'a Size>,
    /// Offset calculator of the widget
    pub offset: Option<&'a Offset>,
}

#[derive(Debug, Default)]
pub struct StyleSheet<'a> {
    map: HashMap<Cow<'a, str>, (AtomicBool, Style)>,

    default: Style,
}

//

impl Style {
    pub fn as_ref(&self) -> StyleRef {
        StyleRef {
            color: self.color.as_ref(),
            texture: self.texture.as_ref(),
            size: self.size.as_ref(),
            offset: self.offset.as_ref(),
        }
    }

    pub fn finalize(self) -> BakedStyle {
        BakedStyle {
            color: self.color.unwrap_or_default(),
            texture: self.texture.unwrap_or_default(),
            size: self.size.unwrap_or_default(),
            offset: self.offset.unwrap_or_default(),
        }
    }
}

impl<'a> StyleRef<'a> {
    /// styles is a whitespace separated list of styles
    /// in which the latter ones are preferred
    ///
    /// example `style: &str` : "* fill_widget yellow"
    pub fn from_styles(style: &str, styles: &'a StyleSheet) -> Self {
        style
            .split_whitespace()
            .map(|style| (style, styles.get(style)))
            .inspect(|(style, s)| {
                if s.is_none() {
                    log::warn!("Style '{style}' not found");
                }
            })
            .filter_map(|(_, s)| s)
            .fold(Self::default(), |collected, style| collected.merge(style))
    }

    pub fn finalize(self) -> BakedStyle {
        BakedStyle {
            color: self.color.copied().unwrap_or_default(),
            texture: self.texture.copied().unwrap_or_default(),
            size: self.size.cloned().unwrap_or_default(),
            offset: self.offset.cloned().unwrap_or_default(),
        }
    }
}

impl<'a> StyleSheet<'a> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn take_default(&mut self) -> Style {
        self.set_default(Style::default())
    }

    pub fn set_default(&mut self, mut default: Style) -> Style {
        swap(&mut default, &mut self.default);
        default
    }

    pub fn get_default(&self) -> StyleRef {
        self.default.as_ref()
    }

    /// insert a style to this stylesheet
    ///
    /// mark it as unused
    pub fn insert<N: Into<Cow<'a, str>>, V: Into<Style>>(&mut self, name: N, value: V) {
        self.map
            .insert(name.into(), (AtomicBool::new(false), value.into()));
    }

    /// get one style from this stylesheet
    ///
    /// mark it as used
    pub fn get(&self, name: &str) -> Option<StyleRef> {
        self.map.get(name).map(|(used, s)| {
            used.store(true, Ordering::SeqCst);
            self.get_default().merge(s.as_ref())
        })
    }

    /// returns an iterator of
    /// all style names that
    /// are marked as unused
    pub fn check_unused(&self) -> impl Iterator<Item = &str> + '_ {
        self.map
            .iter()
            .filter(|(_, (used, _))| !used.load(Ordering::SeqCst))
            .map(|(name, _)| name.as_ref())
    }

    /// mark all styles
    /// as unused
    ///
    /// used with
    /// [`Self::check_unused`]
    pub fn reset_unused(&self) {
        self.map
            .values()
            .for_each(|(used, _)| used.store(false, Ordering::SeqCst))
    }
}

impl From<Style> for BakedStyle {
    fn from(v: Style) -> Self {
        v.finalize()
    }
}

impl<'a> From<StyleRef<'a>> for BakedStyle {
    fn from(v: StyleRef) -> Self {
        v.finalize()
    }
}
