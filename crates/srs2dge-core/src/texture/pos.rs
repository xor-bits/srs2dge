use crate::prelude::{PositionedRect, Rect};
use glam::{Vec2, Vec4};
use serde::{Deserialize, Serialize};

//

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
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

    pub const fn to_vec4(self) -> Vec4 {
        Vec4::from_array([
            self.top_left.x,
            self.top_left.y,
            self.bottom_right.x,
            self.bottom_right.y,
        ])
    }
}

impl Default for TexturePosition {
    fn default() -> Self {
        Self {
            top_left: Vec2::new(0.0, 0.0),
            bottom_right: Vec2::new(1.0, 1.0),
        }
    }
}
