use std::{
    borrow::Cow,
    collections::HashMap,
    fmt::Debug,
    marker::PhantomData,
    mem::swap,
    sync::atomic::{AtomicBool, Ordering},
};

use self::{
    layout::{Offset, Size},
    merge::MergeStyles,
};
use srs2dge_core::prelude::{Color, TexturePosition};
use srs2dge_text::prelude::TextAlign;

//

pub mod layout;
pub mod r#macro;
pub mod merge;
pub mod prelude;

//

pub trait StyleField<T>: Debug + Clone + Default {
    type Type: Debug + Clone + Default;
}

#[derive(Debug, Clone, Default)]
pub struct Baked;
#[derive(Debug, Clone, Default)]
pub struct Mergeable;
#[derive(Debug, Clone, Default)]
pub struct Ref<'a>(PhantomData<&'a ()>);

impl<T: Debug + Clone + Default> StyleField<T> for Baked {
    type Type = T;
}

impl<T: Debug + Clone + Default> StyleField<T> for Mergeable {
    type Type = Option<T>;
}

impl<'a, T: Debug + Clone + Default + 'a> StyleField<T> for Ref<'a> {
    type Type = Option<&'a T>;
}

#[derive(Debug, Clone, Default)]
pub struct Style<T = Mergeable>
where
    T: StyleField<Color>,
    T: StyleField<TexturePosition>,
    T: StyleField<Size>,
    T: StyleField<Offset>,
    T: StyleField<TextAlign>,
{
    // visual styling
    /// Foreground colors
    ///
    /// Like text color or fill color
    pub color: <T as StyleField<Color>>::Type,
    /// Texture position
    ///
    /// Used by fill
    ///
    /// TODO: Texture manager
    pub texture: <T as StyleField<TexturePosition>>::Type,

    // layout styling
    /// Size calculator of the widget
    pub size: <T as StyleField<Size>>::Type,
    /// Offset calculator of the widget
    pub offset: <T as StyleField<Offset>>::Type,
    /// text alignment
    pub text_align: <T as StyleField<TextAlign>>::Type,
}

pub type BakedStyle = Style<Baked>;
pub type StyleRef<'a> = Style<Ref<'a>>;

#[derive(Debug, Default)]
pub struct StyleSheet<'a> {
    map: HashMap<Cow<'a, str>, (AtomicBool, Style)>,

    default: Style,
}

//

impl Style {
    pub fn as_ref(&self) -> Style<Ref> {
        Style {
            color: self.color.as_ref(),
            texture: self.texture.as_ref(),
            size: self.size.as_ref(),
            offset: self.offset.as_ref(),
            text_align: self.text_align.as_ref(),
        }
    }

    pub fn finalize(self) -> Style<Baked> {
        Style {
            color: self.color.unwrap_or_default(),
            texture: self.texture.unwrap_or_default(),
            size: self.size.unwrap_or_default(),
            offset: self.offset.unwrap_or_default(),
            text_align: self.text_align.unwrap_or_default(),
        }
    }
}

impl<'a> Style<Ref<'a>> {
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
            .fold(styles.get_default(), |collected, style| {
                collected.merge(style)
            })
    }

    pub fn finalize(self) -> Style<Baked> {
        Style {
            color: self.color.copied().unwrap_or_default(),
            texture: self.texture.copied().unwrap_or_default(),
            size: self.size.cloned().unwrap_or_default(),
            offset: self.offset.cloned().unwrap_or_default(),
            text_align: self.text_align.copied().unwrap_or_default(),
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

    pub fn get_default(&self) -> Style<Ref> {
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
    pub fn get(&self, name: &str) -> Option<Style<Ref>> {
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

impl From<Style> for Style<Baked> {
    fn from(v: Style) -> Self {
        v.finalize()
    }
}

impl<'a> From<Style<Ref<'a>>> for Style<Baked> {
    fn from(v: Style<Ref>) -> Self {
        v.finalize()
    }
}
