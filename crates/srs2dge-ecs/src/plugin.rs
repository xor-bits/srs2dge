use crate::{prelude::RigidBody2DPlugin, sprite::SpritePlugin, World};

//

pub trait Plugin {
    fn build(&self, world: &mut World);
}

//

pub struct DefaultPlugins;

//

impl Plugin for DefaultPlugins {
    fn build(&self, world: &mut World) {
        world.add_plugin(SpritePlugin);
        world.add_plugin(RigidBody2DPlugin);
    }
}
