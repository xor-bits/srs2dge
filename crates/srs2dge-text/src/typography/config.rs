// ---------------
// COMBINED CONFIG
// ---------------

/// Text rendering configuration
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct TextConfig {
    /// initial x position, defaults to `0`
    pub x_origin: f32,

    /// initial y position, defaults to `0`
    pub y_origin: f32,

    /// text alignment
    pub align: TextAlign,

    // TODO:
    /// see [`TextDirection`], text direction, defaults to `Right`
    ///
    /// WORK IN PROGRESS
    pub dir: TextDirection,

    /// maximum tab width in `' '` _("space")_ characters, defaults to `4`
    pub tab_width: u8,

    /// line gap, `None` uses what the font suggests, defaults to `None`
    pub line_gap: Option<f32>,

    /// sdf or simple raster, defaults to `true`
    pub sdf: bool,
}

impl Default for TextConfig {
    fn default() -> Self {
        Self {
            x_origin: Default::default(),
            y_origin: Default::default(),
            align: Default::default(),
            dir: Default::default(),
            tab_width: 4,
            line_gap: None,
            sdf: true,
        }
    }
}

// ------------------------------------------
// COMBINED (TEXT ALIGNMENT / ORIGIN OFFSETS)
// ------------------------------------------

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct TextAlign {
    /// see [`XOrigin`], defaults to `XOrigin::Left`
    pub x: XOrigin,

    /// see [`YOrigin`], defaults to `YOrigin::Baseline`
    pub y: YOrigin,
}

macro_rules! impl_text_align {
    ($name:ident: ($xt:tt, $yt:tt)) => {
        #[inline]
        pub const fn $name() -> Self {
            Self {
                x: XOrigin::$xt,
                y: YOrigin::$yt,
            }
        }
    };
}

impl TextAlign {
    impl_text_align! { top_left: (Left, Top) }
    impl_text_align! { top: (Middle, Top) }
    impl_text_align! { top_right: (Right, Top) }

    impl_text_align! { left: (Left, Middle) }
    impl_text_align! { centered: (Middle, Middle) }
    impl_text_align! { right: (Right, Middle) }

    impl_text_align! { bottom_left: (Left, Bottom) }
    impl_text_align! { bottom: (Middle, Bottom) }
    impl_text_align! { bottom_right: (Right, Bottom) }

    impl_text_align! { base_left: (Left, Baseline) }
    impl_text_align! { base: (Middle, Baseline) }
    impl_text_align! { base_right: (Right, Baseline) }

    impl_text_align! { baseline_left: (Left, Baseline) }
    impl_text_align! { baseline: (Middle, Baseline) }
    impl_text_align! { baseline_right: (Right, Baseline) }
}

// --------------------
// TEXT WRITE DIRECTION
// --------------------

/// Text rendering direction
///
/// Horizontal / Vertical text
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TextDirection {
    /// Horizontal text
    Right,

    /// Vertical text
    Down,
}

impl Default for TextDirection {
    fn default() -> Self {
        TextDirection::Right
    }
}

// -------------------------------
// TEXT ALIGNMENT / ORIGIN OFFSETS
// -------------------------------

/// Text X alignment
///
/// X line that the `x` in [`TextAlign`] points to
///
/// Defaults to [`XOrigin::Left`]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum XOrigin {
    /// Write text to right from this line
    ///
    /// Fastest
    // +-------------+
    // | ~~~~        |
    // | ~~~~~~~     |
    // | ~~~~~~~~~   |
    // | ~~~~~~      |
    // | ~~~~~~~~    |
    // | ~~~~~~      |
    // +-------------+
    Left,

    /// Write text equally to both left and right from this line
    // +-------------+
    // |    ~~~~     |
    // |   ~~~~~~~   |
    // |  ~~~~~~~~~  |
    // |   ~~~~~~    |
    // |  ~~~~~~~~   |
    // |   ~~~~~~    |
    // +-------------+
    Middle,

    /// Write text to left from this line
    // +-------------+
    // |        ~~~~ |
    // |     ~~~~~~~ |
    // |   ~~~~~~~~~ |
    // |      ~~~~~~ |
    // |    ~~~~~~~~ |
    // |      ~~~~~~ |
    // +-------------+
    Right,
}

impl Default for XOrigin {
    fn default() -> Self {
        Self::Left
    }
}

/// Text Y alignment
///
/// Y line that the `y` in [`TextAlign`] points to
///
/// Defaults to [`YOrigin::Baseline`]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum YOrigin {
    /// Write text down from this line
    ///
    /// First line uses this as the baseline
    ///
    /// Fastest
    // +-------------+
    // | ~~~~        |
    // | ~~~~~~~     |
    // | ~~~~~~~~~   |
    // | ~~~~~~      |
    // | ~~~~~~~~    |
    // | ~~~~~~      |
    // |             |
    // |             |
    // +-------------+
    Baseline,

    /// Write text down from this line
    // +-------------+
    // | ~~~~        |
    // | ~~~~~~~     |
    // | ~~~~~~~~~   |
    // | ~~~~~~      |
    // | ~~~~~~~~    |
    // | ~~~~~~      |
    // |             |
    // |             |
    // +-------------+
    Top,

    /// Write text up from this line
    // +-------------+
    // |             |
    // |             |
    // | ~~~~        |
    // | ~~~~~~~     |
    // | ~~~~~~~~~   |
    // | ~~~~~~      |
    // | ~~~~~~~~    |
    // | ~~~~~~      |
    // +-------------+
    Bottom,

    /// Write text equally both down and up from this line
    // +-------------+
    // |             |
    // | ~~~~        |
    // | ~~~~~~~     |
    // | ~~~~~~~~~   |
    // | ~~~~~~      |
    // | ~~~~~~~~    |
    // | ~~~~~~      |
    // |             |
    // +-------------+
    Middle,
}

impl Default for YOrigin {
    fn default() -> Self {
        Self::Baseline
    }
}
