use crate::World;

pub trait Plugin {
    fn build(&self, world: &mut World);
}

//

pub struct DefaultPlugins;

//

impl Plugin for DefaultPlugins {
    fn build(&self, _: &mut World) {}
}
