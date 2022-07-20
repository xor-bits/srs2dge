use self::pointer::PointerState;
use srs2dge_core::winit::event::KeyboardInput;

//

pub mod pointer;
pub mod prelude;

//

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct GuiEvent {
    pub ty: GuiEventTy,
    pub blocked: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub enum GuiEventTy {
    /// Pointer events
    ///
    /// - cursors
    /// - pens
    /// - touches
    Pointer(PointerState),

    /// keyboard *button* input
    Key(KeyboardInput),

    /// keyboard *text* input
    Text(char),

    /// All events cleared
    #[default]
    Cleared,
}

//

impl GuiEvent {
    pub fn new(ty: GuiEventTy) -> Self {
        Self { ty, blocked: false }
    }
}
