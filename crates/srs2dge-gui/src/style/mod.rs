use self::layout::{Offset, Size};
use srs2dge_core::prelude::{Color, TexturePosition};

//

pub mod layout;
pub mod prelude;

//

/// Common style settings for widgets
#[derive(Debug, Clone, PartialEq, Default)]
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
