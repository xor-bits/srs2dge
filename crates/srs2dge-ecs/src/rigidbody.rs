use crate::{plugin::Plugin, time::Time, transform::Transform2D, World};
use serde::{Deserialize, Serialize};
use srs2dge_core::glam::{Quat, Vec2, Vec3};

//

#[derive(Debug, Clone, Copy, Default, Serialize, Deserialize)]
pub struct RigidBody2D {
    pub linear_velocity: Vec2,
    pub angular_velocity: f32,
}

#[derive(Debug, Clone, Copy, Default, Serialize, Deserialize)]
pub struct RigidBody3D {
    pub linear_velocity: Vec3,
    pub angular_velocity: Quat,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct RigidBody2DPlugin;

//

impl Plugin for RigidBody2DPlugin {
    fn build(&self, world: &mut World) {
        world.updates.insert_internal(100, update_system);
    }
}

//

#[cfg_attr(
    any(target_arch = "wasm32", not(feature = "parallel")),
    legion::system(for_each)
)]
#[cfg_attr(
    all(not(target_arch = "wasm32"), feature = "parallel"),
    legion::system(par_for_each)
)]
fn update(rigidbody: &RigidBody2D, transform: &mut Transform2D, #[resource] time: &Time) {
    // println!("update rigidbody");
    transform.translation += rigidbody.linear_velocity * time.delta_mult();
    transform.rotation += rigidbody.angular_velocity * time.delta_mult();
}
