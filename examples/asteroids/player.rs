use crate::{bullet::Bullet, collider::Collider, mesh::MultiMesh, Settings};
use legion::{system, systems::CommandBuffer};

use srs2dge::prelude::*;

//

#[derive(Debug, Clone, Copy)]
pub struct Player {
    pub idx: Idx,
}

pub struct PlayerPlugin;

//

impl Plugin for PlayerPlugin {
    fn build(&self, world: &mut World) {
        world.updates.insert_internal(50, player_movement_system);
        world.updates.insert_internal(50, player_shoot_system);
        world.updates.insert_internal(150, map_wrapping_system);
        world.frames.insert_internal(200, player_mesh_system);
    }
}

//

#[system(for_each)]
#[filter(legion::maybe_changed::<Player>() | legion::maybe_changed::<Transform2D>())]
fn player_mesh(
    player: &Player,
    transform: &Transform2D,
    #[resource] batcher: &mut BatchRenderer<MultiMesh>,
) {
    if let Some(MultiMesh::Player(mesh)) = batcher.get_mut(player.idx) {
        mesh.lerp_transform = *transform
    }
}

#[cfg_attr(target_arch = "wasm32", system(for_each))]
#[cfg_attr(not(target_arch = "wasm32"), system(par_for_each))]
#[filter(legion::component::<Player>())]
fn player_movement(
    body: &mut RigidBody2D,
    transform: &mut Transform2D,
    #[resource] ks: &KeyboardState,
    #[resource] gs: &GamepadState,
    #[resource] settings: &Settings,
    #[resource] cursor: &Option<Vec2>,
) {
    let mat = Mat2::from_scale_angle(
        Vec2::ONE * 0.1,
        if settings.direction_relative_movement {
            transform.rotation
        } else {
            0.0
        },
    );

    // movement
    if ks.pressed(VirtualKeyCode::W) {
        // M&K
        body.linear_velocity += mat * Vec2::new(0.0, 1.0);
    } else {
        // Gamepad
        let mut o = Vec2::new(
            gs.axis_value_first(GamepadAxis::LeftStickX).unwrap_or(0.0),
            gs.axis_value_first(GamepadAxis::LeftStickY).unwrap_or(0.0),
        );
        if !settings.free_movement {
            o.y = o.y.max(0.0);
            o.x = 0.0;
        }
        body.linear_velocity += mat * o;
    }
    if settings.free_movement {
        // M&K
        if ks.pressed(VirtualKeyCode::A) {
            body.linear_velocity += mat * Vec2::new(-1.0, 0.0);
        }
        if ks.pressed(VirtualKeyCode::D) {
            body.linear_velocity += mat * Vec2::new(1.0, 0.0);
        }
        if ks.pressed(VirtualKeyCode::S) {
            body.linear_velocity += mat * Vec2::new(0.0, -1.0);
        }
    }
    body.linear_velocity *= 0.98;

    // rotation
    if settings.easy_rotation {
        let direction = if let Some(cursor) = cursor {
            // M&K
            Some(*cursor - transform.translation)
        } else {
            // Gamepad
            let o = Vec2::new(
                gs.axis_value_first(GamepadAxis::RightStickX).unwrap_or(0.0),
                gs.axis_value_first(GamepadAxis::RightStickY).unwrap_or(0.0),
            );
            if o.length_squared() >= 0.2 {
                Some(o)
            } else {
                None
            }
        };

        if let Some(direction) = direction {
            transform.rotation = direction.y.atan2(direction.x) - std::f32::consts::FRAC_PI_2;
        }
    } else {
        // M&K
        body.angular_velocity = 0.0;
        if ks.pressed(VirtualKeyCode::Left) {
            body.angular_velocity += 5.0;
        }
        if ks.pressed(VirtualKeyCode::Right) {
            body.angular_velocity -= 5.0;
        }

        // Gamepad
        body.angular_velocity -= 5.0 * gs.axis_value_first(GamepadAxis::LeftStickX).unwrap_or(0.0);
    }
}

#[system(for_each)]
#[filter(legion::component::<Player>())]
fn player_shoot(
    cmd: &mut CommandBuffer,
    transform: &Transform2D,
    #[resource] ks: &KeyboardState,
    #[resource] gs: &GamepadState,
    #[resource] batcher: &mut BatchRenderer<MultiMesh>,
) {
    if ks.just_pressed(VirtualKeyCode::Space)
        || gs.gamepads().next().map(|gamepad| {
            gs.just_pressed(GamepadButtonInput {
                gamepad,
                button: GamepadButton::RightTrigger,
            })
        }) == Some(true)
    {
        cmd.push((
            Transform2D {
                translation: transform.translation,
                scale: Vec2::ONE * 0.01,
                ..Default::default()
            },
            RigidBody2D {
                linear_velocity: Mat2::from_angle(transform.rotation) * Vec2::new(0.0, 3.0),
                ..RigidBody2D::default()
            },
            Bullet {
                idx: batcher.push_with(MultiMesh::Bullet(GizmosCircle::new(
                    Vec2::default(),
                    Vec2::ZERO,
                    Color::WHITE,
                ))),
            },
            Collider,
        ));
    }
}

#[cfg_attr(target_arch = "wasm32", system(for_each))]
#[cfg_attr(not(target_arch = "wasm32"), system(par_for_each))]
#[filter(!legion::component::<Bullet>())]
fn map_wrapping(transform: &mut Transform2D) {
    if transform.translation.x > 1.1 {
        transform.translation.x = -1.1;
    }
    if transform.translation.x < -1.1 {
        transform.translation.x = 1.1;
    }
    if transform.translation.y > 1.1 {
        transform.translation.y = -1.1;
    }
    if transform.translation.y < -1.1 {
        transform.translation.y = 1.1;
    }
}
