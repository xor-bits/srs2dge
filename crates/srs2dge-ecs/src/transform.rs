use serde::{Deserialize, Serialize};
use srs2dge_core::glam::{Quat, Vec2, Vec3};

//

#[derive(Debug, Clone, Copy, Default, Serialize, Deserialize)]
pub struct Transform2D {
    pub translation: Vec2,
    pub rotation: f32,
    pub scale: Vec2,
}

#[derive(Debug, Clone, Copy, Default, Serialize, Deserialize)]
pub struct Transform3D {
    pub translation: Vec3,
    pub rotation: Quat,
    pub scale: Vec3,
}
