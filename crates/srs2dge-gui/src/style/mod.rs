use std::{
    borrow::Cow,
    collections::HashMap,
    sync::atomic::{AtomicBool, Ordering},
};

use self::layout::{Offset, Size};
use srs2dge_core::prelude::{Color, TexturePosition};

//

pub mod layout;
pub mod prelude;

//

/// Common style settings for widgets
#[derive(Debug, Clone, Default)]
pub struct Style {
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

#[derive(Debug, Default)]
pub struct StyleSheet<'a> {
    map: HashMap<Cow<'a, str>, (AtomicBool, Style)>,
}

//

impl Style {
    pub fn from_styles(style: &str, styles: &StyleSheet) -> Self {
        if let Some(style) = styles.get(style) {
            style.clone()
        } else {
            log::warn!("Style '{style}' not found");
            Self::default()
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
