use super::rect::{PositionedRect, Rect};
use glam::Vec2;
use serde::{Deserialize, Serialize};

//

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize, Default)]
pub struct TexturePosition {
    pub top_left: Vec2,
    pub bottom_right: Vec2,
}

//

impl TexturePosition {
    pub fn new(area: Rect, pos: PositionedRect) -> Self {
        let (w, h) = (area.width as f32, area.height as f32);
        let top_left = Vec2::new(pos.x as f32 / w, pos.y as f32 / h);
        let bottom_right = Vec2::new(pos.width as f32 / w, pos.height as f32 / h) + top_left;

        Self {
            top_left,
            bottom_right,
        }
    }
}
