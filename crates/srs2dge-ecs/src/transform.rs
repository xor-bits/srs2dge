use crate::{plugin::Plugin, World};
use specs::{Component, DenseVecStorage};
use srs2dge_core::glam::{Quat, Vec2, Vec3};

//

#[derive(Debug, Clone, Copy, Default, Component)]
pub struct Transform2DComponent {
    pub translation: Vec2,
    pub rotation: f32,
    pub scale: Vec2,
}

#[derive(Debug, Clone, Copy, Default, Component)]
pub struct Transform3DComponent {
    pub translation: Vec3,
    pub rotation: Quat,
    pub scale: Vec3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct TransformPlugin;

//

impl Plugin for TransformPlugin {
    fn build(&self, _: &mut World) {}
}
