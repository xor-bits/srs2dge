use bytemuck::{Pod, Zeroable};
use glam::{Mat4, Vec2, Vec3};

//

#[derive(Debug, Clone, Copy, PartialEq, Default, Zeroable, Pod)]
#[repr(C)]
pub struct Camera {
    mat: Mat4,
}

//

impl Camera {
    pub fn new_look_at(camera_pos: Vec3, look_at: Vec3) -> Self {
        Self::new_look_at_orient(camera_pos, look_at, Vec3::Y)
    }

    pub fn new_look_at_orient(camera_pos: Vec3, look_at: Vec3, up: Vec3) -> Self {
        Self {
            mat: Mat4::look_at_rh(camera_pos, look_at, up),
        }
    }

    pub fn new_ortho_2d(camera_pos: Vec2, aspect_ratio: f32, zoom: f32, angle: f32) -> Self {
        let v = Mat4::from_rotation_z(angle)
            * Mat4::from_translation(Vec3::new(camera_pos.x, camera_pos.y, 0.0));
        let p = Mat4::orthographic_rh(
            -aspect_ratio * zoom,
            aspect_ratio * zoom,
            zoom,
            -zoom,
            -100.0,
            100.0,
        );

        Self { mat: v * p }
    }

    pub fn new_from(mat: Mat4) -> Self {
        Self { mat }
    }
}
