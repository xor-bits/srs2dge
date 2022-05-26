use crate::{
    asteroid::{spawn_asteroid, Asteroid, Size},
    bullet::Bullet,
    mesh::MultiMesh,
};
use legion::{component, system, systems::CommandBuffer, world::SubWorld, Entity, IntoQuery};

use srs2dge::prelude::*;

//

#[derive(Debug, Clone, Copy)]
pub struct Collider;

#[derive(Debug, Clone, Copy)]
pub struct ColliderPlugin;

//

impl Plugin for ColliderPlugin {
    fn build(&self, world: &mut World) {
        world.insert_internal_update_system(150, split_collided_asteroids_system);
    }
}

#[system]
#[read_component(Entity)]
#[read_component(Bullet)]
#[read_component(Asteroid)]
#[read_component(Transform2D)]
#[read_component(RigidBody2D)]
#[read_component(Collider)]
fn split_collided_asteroids(
    cmd: &mut CommandBuffer,
    world: &SubWorld,
    #[resource] batcher: &mut BatchRenderer<MultiMesh>,
) {
    for q in <(Entity, &Bullet, &Transform2D, &Collider)>::query()
        .filter(component::<Bullet>())
        .iter(world)
    {
        let (other_entity, bullet, other_transform, _): (
            &Entity,
            &Bullet,
            &Transform2D,
            &Collider,
        ) = q;

        for q in <(Entity, &Asteroid, &Transform2D, &RigidBody2D, &Collider)>::query().iter(world) {
            let (entity, asteroid, transform, body, _): (
                &Entity,
                &Asteroid,
                &Transform2D,
                &RigidBody2D,
                &Collider,
            ) = q;

            if entity == other_entity {
                continue;
            }

            if (transform.translation - other_transform.translation).length_squared()
                <= (transform.scale.x + other_transform.scale.x).powi(2)
            {
                cmd.remove(*entity);
                batcher.drop(bullet.idx);
                cmd.remove(*other_entity);
                batcher.drop(asteroid.idx);

                match asteroid.size {
                    Size::Large => {
                        cmd.push(spawn_asteroid(
                            Size::Medium,
                            transform.translation,
                            body.linear_velocity,
                            body.angular_velocity,
                            batcher,
                        ));
                        cmd.push(spawn_asteroid(
                            Size::Medium,
                            transform.translation,
                            -body.linear_velocity,
                            -body.angular_velocity,
                            batcher,
                        ));
                    }
                    Size::Medium => {
                        cmd.push(spawn_asteroid(
                            Size::Small,
                            transform.translation,
                            body.linear_velocity,
                            body.angular_velocity,
                            batcher,
                        ));
                        cmd.push(spawn_asteroid(
                            Size::Small,
                            transform.translation,
                            -body.linear_velocity,
                            -body.angular_velocity,
                            batcher,
                        ));
                    }
                    Size::Small => {}
                }

                break;
            }
        }
    }
}
