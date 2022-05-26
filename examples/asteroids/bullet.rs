use crate::mesh::MultiMesh;
use legion::{system, systems::CommandBuffer, Entity};

use srs2dge::prelude::*;

//

#[derive(Debug, Clone, Copy)]
pub struct Bullet {
    pub idx: Idx,
}

pub struct BulletPlugin;

//

impl Plugin for BulletPlugin {
    fn build(&self, world: &mut World) {
        world.insert_internal_update_system(150, bullet_destroy_system);
        world.insert_internal_frame_system(200, bullet_mesh_system);
    }
}

//

#[system(for_each)]
#[filter(legion::maybe_changed::<Bullet>() | legion::maybe_changed::<Transform2D>())]
fn bullet_mesh(
    bullet: &Bullet,
    transform: &Transform2D,
    #[resource] batcher: &mut BatchRenderer<MultiMesh>,
) {
    if let Some(MultiMesh::Bullet(mesh)) = batcher.get_mut(bullet.idx) {
        mesh.middle = transform.translation;
        mesh.radius = transform.scale.x;
    }
}

#[system(for_each)]
fn bullet_destroy(
    cmd: &mut CommandBuffer,
    entity: &Entity,
    bullet: &Bullet,
    transform: &mut Transform2D,
    #[resource] batcher: &mut BatchRenderer<MultiMesh>,
) {
    if transform.translation.x > 1.1
        || transform.translation.x < -1.1
        || transform.translation.y > 1.1
        || transform.translation.y < -1.1
    {
        batcher.drop(bullet.idx);
        cmd.remove(*entity);
    }
}
