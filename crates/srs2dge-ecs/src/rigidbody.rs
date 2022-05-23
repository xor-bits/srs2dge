use crate::{plugin::Plugin, time::Time, transform::Transform2D, World};
use legion::system;
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
        world.insert_internal_update_system(100, update_system);
    }
}

//

#[cfg_attr(target_arch = "wasm32", system(for_each))]
#[cfg_attr(not(target_arch = "wasm32"), system(par_for_each))]
fn update(rigidbody: &RigidBody2D, transform: &mut Transform2D, #[resource] time: &Time) {
    // println!("update rigidbody");
    transform.translation += rigidbody.linear_velocity * time.delta_mult();
    transform.rotation += rigidbody.angular_velocity * time.delta_mult();
}
