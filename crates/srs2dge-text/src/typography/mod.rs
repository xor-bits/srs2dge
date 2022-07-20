use crate::prelude::Format;

//

pub mod bounds;
pub mod config;
pub mod iter;
pub mod line;
pub mod prelude;

//

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct TextChar {
    pub character: char,
    // character codepoint
    pub index: u16,
    // formatting
    pub format: Format,
    // top left
    pub x: f32,
    pub y: f32,
    // size
    pub width: u32,
    pub height: u32,
}
