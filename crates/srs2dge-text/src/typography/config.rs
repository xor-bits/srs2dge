/// Text rendering configuration
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct TextConfig {
    /// initial x position, defaults to `0`
    pub x_origin: i32,

    /// initial y position, defaults to `0`
    pub y_origin: i32,

    // TODO:
    /// see [`XOrigin`], defaults to `XOrigin::Left`
    ///
    /// WORK IN PROGRESS
    pub x_origin_point: XOrigin,

    /// see [`YOrigin`], defaults to `YOrigin::Baseline`
    pub y_origin_line: YOrigin,

    // TODO:
    /// see [`TextDirection`], text direction, defaults to `Right`
    ///
    /// WORK IN PROGRESS
    pub dir: TextDirection,

    /// tab width in `' '` _("space")_ characters, defaults to `4`
    pub tab_width: u8,

    /// line gap, `None` uses what the font suggests, defaults to `None`
    pub line_gap: Option<f32>,

    /// sdf or simple raster, defaults to `true`
    pub sdf: bool,
}

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

/// Text horizontal? X alignment
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum XOrigin {
    /// Write text to right of this line
    Left,

    /// Write text equally to both left and right of this line
    Middle,

    /// Write text equally to both left and right of this line
    ///
    /// Faster than [`XOrigin::Middle`] but correct only for
    /// monospace fonts.
    FastMiddle,

    /// Write text to left of this line
    Right,
}

/// Text vertical? Y alignment
///
/// Y line that the `y_origin` in [`TextConfig`] points to
///
/// For image examples: https://upload.wikimedia.org/wikipedia/commons/thumb/3/39/Typography_Line_Terms.svg/1920px-Typography_Line_Terms.svg.png
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum YOrigin {
    /// Baseline height line
    ///
    /// Letter bottom point (ignoring descenders)
    ///
    /// see: https://en.wikipedia.org/wiki/Ascender_(typography)#/media/File:Typography_Line_Terms.svg
    ///
    /// Lowest point of for example for `'o'` and `'h'` but not for `'p'`
    Baseline,

    /// Descender height line
    ///
    /// Letter bottom point
    ///
    /// Use for aligning text from its bottom
    ///
    /// see: https://en.wikipedia.org/wiki/Ascender_(typography)#/media/File:Typography_Line_Terms.svg
    ///
    /// Lowest point for example for `'p'` and `'q'` but not for `''`
    Descender,

    /// Ascender height line
    ///
    /// Letter top point
    ///
    /// see: https://en.wikipedia.org/wiki/Ascender_(typography)#/media/File:Typography_Line_Terms.svg
    ///
    /// Highest point for example for `'h'` and `'l'` but not for `'y'`
    Ascender,

    /// Mean line
    ///
    /// Middle point of [`YOrigin::Ascender`] and [`YOrigin::Descender`]
    ///
    /// Use for centering text
    ///
    /// see: https://en.wikipedia.org/wiki/Ascender_(typography)#/media/File:Typography_Line_Terms.svg
    Mean,
    /* /// Typographic CAP HEIGHT
    ///
    /// Capital letter top point
    ///
    /// Use for aligning text from its top
    ///
    /// see: https://en.wikipedia.org/wiki/Ascender_(typography)#/media/File:Typography_Line_Terms.svg
    ///
    /// Highest point for example for `'W'` and `'S'` but not for `'b'`
    Cap, */

    /* /// Typographic X-Height
    ///
    /// Letter top point (ignoring ascenders)
    ///
    /// see: https://en.wikipedia.org/wiki/Ascender_(typography)#/media/File:Typography_Line_Terms.svg
    ///
    /// Highest point for example for `'a'` and `'g'` but not for `'i'`
    X, */
}

//

impl Default for TextConfig {
    fn default() -> Self {
        Self {
            x_origin: Default::default(),
            y_origin: Default::default(),
            x_origin_point: Default::default(),
            y_origin_line: Default::default(),
            dir: Default::default(),
            tab_width: 4,
            line_gap: None,
            sdf: true,
        }
    }
}

impl Default for TextDirection {
    fn default() -> Self {
        TextDirection::Right
    }
}

impl Default for XOrigin {
    fn default() -> Self {
        Self::Left
    }
}

impl Default for YOrigin {
    fn default() -> Self {
        Self::Baseline
    }
}
