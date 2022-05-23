use legion::{component, system, world::SubWorld, IntoQuery};
use serde::{Deserialize, Serialize};
use srs2dge::{prelude::*, winit::event::VirtualKeyCode};

//

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
pub struct Player {
    can_jump: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
pub struct Collider;

#[derive(Debug, Clone, Copy, PartialEq, Default, Serialize, Deserialize)]
pub struct CollisionResolver(pub Vec2);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct CustomPlugin;

//

impl Plugin for CustomPlugin {
    fn build(&self, world: &mut World) {
        world.insert_internal_update_system(50, player_system);
        world.insert_internal_update_system(105, collider_system);
        world.insert_internal_update_system(106, collision_resolution_system);
        world.insert_internal_update_system(107, player_reposition_system);
    }
}

#[cfg_attr(target_arch = "wasm32", system(for_each))]
#[cfg_attr(not(target_arch = "wasm32"), system(par_for_each))]
#[filter(component::<Player>())]
fn player_reposition(transform: &mut Transform2D, body: &mut RigidBody2D) {
    if transform.translation.y <= -1.0 {
        transform.translation = Vec2::ZERO;
        *body = RigidBody2D::default();
    }
}

#[cfg_attr(target_arch = "wasm32", system(for_each))]
#[cfg_attr(not(target_arch = "wasm32"), system(par_for_each))]
fn player(
    player: &mut Player,
    body: &mut RigidBody2D,
    #[resource] input_kb: &KeyboardState,
    #[resource] input_gp: &GamepadState,
) {
    // gravity
    body.linear_velocity += Vec2::new(0.0, -0.1);

    // movement
    if input_kb.pressed(VirtualKeyCode::A) {
        body.linear_velocity.x -= 0.5;
    } else if let Some(gamepad) = input_gp.gamepads().next() {
        body.linear_velocity.x += input_gp.axis_value(GamepadAxisInput {
            axis: GamepadAxis::LeftStickX,
            gamepad,
        }) * 0.5;
    }
    if input_kb.pressed(VirtualKeyCode::D) {
        body.linear_velocity.x += 0.5;
    }
    if player.can_jump
        && input_kb.just_pressed(VirtualKeyCode::W) | input_kb.just_pressed(VirtualKeyCode::Space)
    {
        body.linear_velocity.y += 3.0;
        player.can_jump = false;
    } else if let Some(gamepad) = input_gp.gamepads().next() {
        if player.can_jump
            && input_gp.just_pressed(GamepadButtonInput {
                gamepad,
                button: GamepadButton::South,
            })
        {
            body.linear_velocity.y += 3.0;
            player.can_jump = false;
        }
    }

    // dampening
    body.linear_velocity.x *= 0.8;
    body.linear_velocity.y *= 0.95;
}

#[system]
#[write_component(Player)]
#[write_component(RigidBody2D)]
#[write_component(CollisionResolver)]
#[read_component(Transform2D)]
#[read_component(Collider)]
fn collider(world: &mut SubWorld) {
    type QueryA<'a> = (
        &'a mut Player,
        &'a mut RigidBody2D,
        &'a mut CollisionResolver,
        &'a Transform2D,
        &'a Collider,
    );
    type QueryB<'a> = (&'a Transform2D, &'a Collider);

    let mut movable_colliders = QueryA::query();

    let (mut movable, all) = world.split_for_query(&movable_colliders);
    movable_colliders.for_each_mut(&mut movable, |query| {
        // unwrap it here to give it a type
        // just to fix a (bug/optimization)? in rust-analyzer
        let (player, body, res, a, _): QueryA = query;
        QueryB::query()
            .filter(!component::<RigidBody2D>())
            .for_each(&all, |query| {
                // same as above
                let (b, _): QueryB = query;

                let a_min = a.translation + res.0 - a.scale * 0.5;
                let a_max = a.translation + res.0 + a.scale * 0.5;
                let b_min = b.translation - b.scale * 0.5;
                let b_max = b.translation + b.scale * 0.5;
                if aabb(a_min, a_max, b_min, b_max) {
                    player.can_jump = true;
                    res.0 += aabb_res(a_min, a_max, b_min, b_max);
                    if res.0.x.abs() >= f32::EPSILON {
                        // top at wall
                        body.linear_velocity.x = 0.0;
                        // slide walls
                        body.linear_velocity.y = body.linear_velocity.y.max(0.0);
                    }
                    if res.0.y.abs() >= f32::EPSILON {
                        // stop at floor/ceiling
                        body.linear_velocity.y = 0.0;
                    }
                }
            });
    })
}

#[cfg_attr(target_arch = "wasm32", system(for_each))]
#[cfg_attr(not(target_arch = "wasm32"), system(par_for_each))]
fn collision_resolution(transform: &mut Transform2D, res: &mut CollisionResolver) {
    transform.translation += res.0;
    res.0 = Vec2::ZERO;
}

fn aabb(a_min: Vec2, a_max: Vec2, b_min: Vec2, b_max: Vec2) -> bool {
    (a_min.x < b_max.x && a_max.x > b_min.x) && (a_min.y < b_max.y && a_max.y > b_min.y)
}

fn aabb_res(a_min: Vec2, a_max: Vec2, b_min: Vec2, b_max: Vec2) -> Vec2 {
    let xo_1 = a_max.x - b_min.x;
    let yo_1 = a_max.y - b_min.y;
    let xo_2 = a_min.x - b_max.x;
    let yo_2 = a_min.y - b_max.y;
    let smallest = |v: f32, a: &[f32]| -> bool { a.iter().all(|x| v.abs() < x.abs()) };

    if smallest(xo_1, &[yo_1, xo_2, yo_2]) {
        Vec2::new(-xo_1, 0.0)
    } else if smallest(yo_1, &[xo_1, xo_2, yo_2]) {
        Vec2::new(0.0, -yo_1)
    } else if smallest(xo_2, &[xo_1, yo_1, yo_2]) {
        Vec2::new(-xo_2, 0.0)
    } else if smallest(yo_2, &[xo_1, yo_1, xo_2]) {
        Vec2::new(0.0, -yo_2)
    } else {
        Vec2::ZERO
    }
}
