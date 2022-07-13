use crate::{collider::Collider, mesh::MultiMesh};
use instant::{Duration, Instant};
use legion::{system, systems::CommandBuffer, world::SubWorld, Query};

use rand::Rng;
use srs2dge::prelude::*;

//

#[derive(Debug, Clone, Copy)]
pub struct Asteroid {
    pub size: Size,
    pub idx: Idx,
}

#[derive(Debug, Clone, Copy)]
pub enum Size {
    Large,
    Medium,
    Small,
}

struct Timeout {
    deadline: Option<Instant>,
}

pub struct AsteroidPlugin;

//

impl Plugin for AsteroidPlugin {
    fn build(&self, world: &mut World) {
        world
            .updates
            .insert_internal(10, asteroid_spawner_timeout_system);
        world.updates.insert_internal(11, asteroid_spawner_system);
        world.frames.insert_internal(200, asteroid_mesh_system);
        world.resources.insert(Timeout {
            deadline: Some(Instant::now() + Duration::from_millis(1500)),
        });
    }
}

//

pub fn spawn_asteroid(
    size: Size,
    pos: Vec2,
    l_vel: Vec2,
    a_vel: f32,
    batcher: &mut BatchRenderer<MultiMesh>,
) -> (Transform2D, RigidBody2D, Asteroid, Collider) {
    (
        Transform2D {
            translation: pos,
            scale: Vec2::ONE
                * match size {
                    Size::Large => 0.1,
                    Size::Medium => 0.05,
                    Size::Small => 0.025,
                },
            ..Transform2D::default()
        },
        RigidBody2D {
            linear_velocity: l_vel,
            angular_velocity: a_vel,
        },
        Asteroid {
            size,
            idx: batcher.push_with(MultiMesh::Asteroid(Default::default())),
        },
        Collider,
    )
}

#[system(for_each)]
#[filter(legion::maybe_changed::<Asteroid>() | legion::maybe_changed::<Transform2D>())]
fn asteroid_mesh(
    asteroid: &Asteroid,
    transform: &Transform2D,
    #[resource] batcher: &mut BatchRenderer<MultiMesh>,
) {
    if let Some(MultiMesh::Asteroid(mesh)) = batcher.get_mut(asteroid.idx) {
        mesh.lerp_transform = *transform;
    }
}

#[system]
#[read_component(Asteroid)]
fn asteroid_spawner_timeout(
    world: &mut SubWorld,
    q: &mut Query<&Asteroid>,
    #[resource] timeout: &mut Timeout,
) {
    if q.iter(world).count() == 0 && timeout.deadline.is_none() {
        timeout.deadline = Some(Instant::now() + Duration::from_secs(2));
    }
}

#[system]
#[read_component(Asteroid)]
fn asteroid_spawner(
    cmd: &mut CommandBuffer,
    #[resource] timeout: &mut Timeout,
    #[resource] batcher: &mut BatchRenderer<MultiMesh>,
) {
    if let Some(false) | None = timeout.deadline.as_ref().map(|i| Instant::now() >= *i) {
        return;
    }

    timeout.deadline = None;

    let mut rng = rand::thread_rng();
    for _ in 0..20 {
        cmd.push(spawn_asteroid(
            Size::Large,
            Mat2::from_angle(rng.gen_range(0.0..2.0 * std::f32::consts::PI)) * Vec2::X,
            Mat2::from_angle(rng.gen_range(0.0..2.0 * std::f32::consts::PI)) * Vec2::X * 0.3,
            rng.gen_range(-1.0..1.0),
            batcher,
        ));
    }
}
