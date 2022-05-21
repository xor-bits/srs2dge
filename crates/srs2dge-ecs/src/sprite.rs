use crate::{plugin::Plugin, transform::Transform2DComponent, SystemType, World};
use specs::{Component, DenseVecStorage, Join, System, Write, WriteStorage};
use srs2dge_core::{
    batch::BatchRenderer,
    glam::Vec4,
    prelude::{Idx, QuadMesh, TexturePosition},
};

#[derive(Debug, Clone, Copy, Default, Component)]
pub struct SpriteComponent {
    sprite: TexturePosition,
    color: Vec4,
    idx: Option<Idx>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct SpritePlugin;

struct SpriteToQuadSystem;

//

impl Plugin for SpritePlugin {
    fn build(&self, world: &mut World) {
        world.insert_system();
    }
}

impl<'a> System<'a> for SpriteToQuadSystem {
    type SystemData = (
        WriteStorage<'a, SpriteComponent>,
        WriteStorage<'a, Transform2DComponent>,
        Write<'a, Option<BatchRenderer>>,
    );

    fn run(&mut self, (mut sprite, mut transform, mut batcher): Self::SystemData) {
        let batcher = batcher.as_mut().unwrap();
        for (sprite, transform) in (&mut sprite, &mut transform).join() {
            if let Some(idx) = sprite.idx {
                let mesh = batcher.get(idx);
                if mesh.pos != transform.translation
                    || mesh.size != transform.scale
                    || mesh.col != sprite.color
                    || mesh.tex != sprite.sprite
                {
                    let mesh = batcher.get_mut(idx);
                    mesh.pos = transform.translation;
                    mesh.size = transform.scale;
                    mesh.col = sprite.color;
                    mesh.tex = sprite.sprite;
                }
            } else {
                sprite.idx = Some(batcher.push_with(QuadMesh {
                    pos: transform.translation,
                    size: transform.scale,
                    col: sprite.color,
                    tex: sprite.sprite,
                }));
            }
        }
    }
}
