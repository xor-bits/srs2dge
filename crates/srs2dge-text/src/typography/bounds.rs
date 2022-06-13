#[derive(Debug, Clone, Copy, PartialEq)]
pub struct TextBoundingBox {
    /// top left point
    pub x: f32,

    /// top left point
    pub y: f32,

    /// left to right for [`TextDirection::Right`]
    /// top to bottom [`TextDirection::Down`]
    pub width: f32,

    /// top to bottom for [`TextDirection::Down`]
    /// left to right [`TextDirection::Right`]
    pub height: f32,
}

//

impl TextBoundingBox {
    pub fn union(self, other: Self) -> Self {
        let x = self.x.min(other.x);
        let y = self.y.min(other.y);
        Self {
            x,
            y,
            width: (self.x + self.width).max(other.x + other.width) - x,
            height: (self.y + self.height).max(other.y + other.height) - y,
        }
    }
}
