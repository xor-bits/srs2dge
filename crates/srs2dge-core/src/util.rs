use glam::Vec2;

pub trait ForceAspectRatio {
    fn force_ratio_with_x(self, ratio: f32) -> Self;
    fn force_ratio_with_y(self, ratio: f32) -> Self;
}

//

impl ForceAspectRatio for Vec2 {
    fn force_ratio_with_x(mut self, ratio: f32) -> Self {
        self.x = self.y * ratio;
        self
    }

    fn force_ratio_with_y(mut self, ratio: f32) -> Self {
        self.y = self.x / ratio;
        self
    }
}
