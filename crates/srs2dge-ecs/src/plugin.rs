use crate::{prelude::RigidBody2DPlugin, sprite::SpritePlugin, World};
use srs2dge_core::{
    batch::BatchRenderer, buffer::DefaultVertex, prelude::QuadMesh, target::Target,
};
use std::fmt::Debug;

//

pub trait Plugin {
    fn build(&self, world: &mut World);
}

//

#[derive(Clone, Copy)]
pub struct DefaultClientPlugins<'a>(pub &'a Target);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct DefaultServerPlugins;

/// Required for rendering
#[derive(Clone, Copy)]
pub struct FramePlugin<'a>(pub &'a Target);

//

impl<'a> Plugin for DefaultClientPlugins<'a> {
    fn build(&self, world: &mut World) {
        world.add_plugin(FramePlugin(self.0));
        world.add_plugin(SpritePlugin);
        world.add_plugin(RigidBody2DPlugin);
    }
}

impl<'a> Debug for DefaultClientPlugins<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("DefaultClientPlugins").finish()
    }
}

impl Plugin for DefaultServerPlugins {
    fn build(&self, world: &mut World) {
        world.add_plugin(RigidBody2DPlugin);
    }
}

impl<'a> Plugin for FramePlugin<'a> {
    fn build(&self, world: &mut World) {
        world
            .resources
            .insert(BatchRenderer::<QuadMesh, DefaultVertex>::new(self.0));
        world.frame_plugin = true;
    }
}

impl<'a> Debug for FramePlugin<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("FramePlugin").finish()
    }
}
