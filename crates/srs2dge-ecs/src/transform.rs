use serde::{Deserialize, Serialize};
use srs2dge_core::glam::{Quat, Vec2, Vec3};

//

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Transform2D {
    pub translation: Vec2,
    pub rotation: f32,
    pub scale: Vec2,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Transform3D {
    pub translation: Vec3,
    pub rotation: Quat,
    pub scale: Vec3,
}

//

impl Default for Transform2D {
    fn default() -> Self {
        Self {
            translation: Vec2::ZERO,
            rotation: 0.0,
            scale: Vec2::ONE,
        }
    }
}

impl Default for Transform3D {
    fn default() -> Self {
        Self {
            translation: Vec3::ZERO,
            rotation: Quat::IDENTITY,
            scale: Vec3::ONE,
        }
    }
}
