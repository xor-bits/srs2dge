use glam::Vec2;

pub mod glyph;
pub mod packer2d;
pub mod texture;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct TexturePosition {
    pub top_left: Vec2,
    pub bottom_right: Vec2,
}
