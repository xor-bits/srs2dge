#![feature(type_name_of_val)]

use std::collections::BTreeMap;

use plugin::Plugin;
use specs::{DispatcherBuilder, WorldExt};
use srs2dge_core::{
    batch::BatchRenderer, buffer::DefaultVertex, main_game_loop::update::UpdateRate,
    prelude::QuadMesh, target::Target,
};
use system::System;

//

pub mod plugin;
pub mod sprite;
pub mod system;
pub mod transform;

//

pub struct World {
    world: specs::World,

    systems: Vec<Box<dyn System>>,
    internal_systems: BTreeMap<u32, Box<dyn System>>,
    // dispatcher: AsyncDispatcher<'static, Arc<RwLock<specs::World>>>,
    // pre_update_1_systems: HashMap<UpdateRate, Vec<Box<dyn System>>>,
    // pre_update_2_systems: HashMap<UpdateRate, Vec<Box<dyn System>>>,
    // post_update_systems: HashMap<UpdateRate, Vec<Box<dyn System>>>,
    // pre_frame_1_systems: Vec<Box<dyn System>>,
    // pre_frame_2_systems: Vec<Box<dyn System>>,
    // post_frame_systems: Vec<Box<dyn System>>,
    // modified: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SystemType {
    PreUpdate1(UpdateRate),
    PreUpdate2(UpdateRate),
    PostUpdate(UpdateRate),
    PreFrame1,
    PreFrame2,
    PostFrame,
}

//

impl World {
    pub fn new(target: &Target) -> Self {
        let mut world = specs::World::new(); //Arc::new(RwLock::new(specs::World::new()));
        world.insert(BatchRenderer::<QuadMesh, DefaultVertex>::new(target));

        // let dispatcher = DispatcherBuilder::new().build_async(world.clone());
        // let dispatcher_builder = DispatcherBuilder::new();

        Self {
            world,

            systems: Default::default(),
            internal_systems: Default::default(),
            // dispatcher,
            // pre_update_1_systems: Default::default(),
            // pre_update_2_systems: Default::default(),
            // post_update_systems: Default::default(),
            // pre_frame_1_systems: Default::default(),
            // pre_frame_2_systems: Default::default(),
            // post_frame_systems: Default::default(),
            // modified: false,
        }
    }

    pub fn with_plugin(mut self, plugin: impl Plugin) -> Self {
        self.add_plugin(plugin);
        self
    }

    pub fn add_plugin(&mut self, plugin: impl Plugin) {
        plugin.build(self);
    }

    /* pub fn insert_system<S>(&mut self, ty: SystemType, system: S)
    where
        S: System + 'static,
    {
        let s = Box::new(system);
        match ty {
            SystemType::PreUpdate1(rate) => {
                self.pre_update_1_systems.entry(rate).or_default().push(s)
            }
            SystemType::PreUpdate2(rate) => {
                self.pre_update_1_systems.entry(rate).or_default().push(s)
            }
            SystemType::PostUpdate(rate) => {
                self.pre_update_1_systems.entry(rate).or_default().push(s)
            }
            SystemType::PreFrame1 => self.pre_frame_1_systems.push(s),
            SystemType::PreFrame2 => self.pre_frame_2_systems.push(s),
            SystemType::PostFrame => self.post_frame_systems.push(s),
        }
        // self.modified = true;
    }

    pub fn remove_system<S>(_: SystemType, _: S)
    where
        S: System,
    {
        todo!()
    } */

    pub fn insert_system<S: System + 'static>(&mut self, system: S) {
        self.systems.push(Box::new(system))
    }

    pub fn insert_internal_system<S: System + 'static>(&mut self, index: u32, system: S) {
        if self
            .internal_systems
            .insert(index, Box::new(system))
            .is_some()
        {
            panic!("Internal system index already used. Is this plugin already set?");
        }
    }

    pub fn run(&mut self) {
        let mut builder = DispatcherBuilder::new();

        for system in self.systems.iter() {
            system.add_system(&mut builder);
        }

        builder.add_barrier();

        for system in self.internal_systems.values() {
            system.add_system(&mut builder);
        }

        builder.build().dispatch(&self.world);
    }

    pub fn create_entity(&mut self) {}
}
