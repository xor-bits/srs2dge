#![feature(type_name_of_val)]

use std::{
    collections::BTreeMap,
    ops::{Deref, DerefMut},
};

use legion::{
    systems::{Builder, ParallelRunnable},
    Resources, Schedule,
};
use plugin::Plugin;
use prelude::time::Time;
use srs2dge_core::{
    batch::BatchRenderer,
    main_game_loop::{
        report::Reporter,
        update::{UpdateLoop, UpdateRate},
    },
    target::Target,
};

//

pub extern crate legion;

//

pub mod plugin;
pub mod prelude;
pub mod rigidbody;
pub mod sprite;
pub mod time;
pub mod transform;

//

type SystemCreator = Box<dyn FnMut(&mut Builder) -> &mut Builder>;

//

pub struct World {
    world: legion::World,

    batcher: Option<BatchRenderer>,

    update_loop: UpdateLoop,
    update_rate: UpdateRate,

    update_reporter: Reporter,
    frame_reporter: Reporter,

    update_systems: Vec<SystemCreator>,
    frame_systems: Vec<SystemCreator>,
    internal_update_systems: BTreeMap<u32, Vec<SystemCreator>>,
    internal_frame_systems: BTreeMap<u32, Vec<SystemCreator>>,
}

//

impl World {
    pub fn new(target: &Target) -> Self {
        let world = legion::World::new(Default::default());

        let batcher = Some(BatchRenderer::new(target));

        let update_rate = UpdateRate::PerSecond(60);
        let update_loop = UpdateLoop::new(update_rate);

        Self {
            world,

            batcher,

            update_loop,
            update_rate,

            update_reporter: Reporter::new(),
            frame_reporter: Reporter::new(),

            update_systems: Default::default(),
            frame_systems: Default::default(),
            internal_update_systems: Default::default(),
            internal_frame_systems: Default::default(),
        }
    }

    pub fn with_rate(mut self, rate: UpdateRate) -> Self {
        self.update_rate = rate;
        self.update_loop = UpdateLoop::new(rate);
        self
    }

    pub fn with_plugin(mut self, plugin: impl Plugin) -> Self {
        self.add_plugin(plugin);
        self
    }

    pub fn add_plugin(&mut self, plugin: impl Plugin) {
        plugin.build(self);
    }

    pub fn insert_update_system<R: ParallelRunnable + 'static, S: FnMut() -> R + 'static>(
        &mut self,
        mut system: S,
    ) {
        self.update_systems
            .push(Box::new(move |builder| builder.add_system(system())));
    }

    pub fn insert_frame_system<R: ParallelRunnable + 'static, S: FnMut() -> R + 'static>(
        &mut self,
        mut system: S,
    ) {
        self.frame_systems
            .push(Box::new(move |builder| builder.add_system(system())));
    }

    pub fn insert_internal_update_system<
        R: ParallelRunnable + 'static,
        S: FnMut() -> R + 'static,
    >(
        &mut self,
        index: u32,
        mut system: S,
    ) {
        let parallel_systems = self.internal_update_systems.entry(index).or_default();
        parallel_systems.push(Box::new(move |builder| builder.add_system(system())));
    }

    pub fn insert_internal_frame_system<
        R: ParallelRunnable + 'static,
        S: FnMut() -> R + 'static,
    >(
        &mut self,
        index: u32,
        mut system: S,
    ) {
        let parallel_systems = self.internal_frame_systems.entry(index).or_default();
        parallel_systems.push(Box::new(move |builder| builder.add_system(system())));
    }

    pub fn get_batcher(&self) -> &BatchRenderer {
        self.batcher.as_ref().unwrap()
    }

    pub fn get_batcher_mut(&mut self) -> &mut BatchRenderer {
        self.batcher.as_mut().unwrap()
    }

    /// returns a bool that is true if update systems ran
    pub fn run_with(&mut self, update_resources: Resources, frame_resources: Resources) -> bool {
        let old_update_rate = self.update_rate;

        // update(s)
        let (delta_seconds, updated) = self.update(update_resources);

        // frame
        self.frame(delta_seconds, frame_resources);

        // update rate modified
        if self.update_rate != old_update_rate {
            self.update_loop = UpdateLoop::new(self.update_rate);
        }

        updated
    }

    /// returns a bool that is true if update systems ran
    pub fn run(&mut self) -> bool {
        self.run_with(Resources::default(), Resources::default())
    }

    pub fn reporters(&mut self) -> (&mut Reporter, &mut Reporter) {
        (&mut self.update_reporter, &mut self.frame_reporter)
    }

    fn update(&mut self, update_resources: Resources) -> (f32, bool) {
        let delta_mult = self.update_rate.to_interval().as_secs_f32();
        let time = Time { delta_mult };

        let mut builder = Schedule::builder();

        for system in self.update_systems.iter_mut() {
            system(&mut builder);
        }

        builder.flush();

        for parallel_systems in self.internal_update_systems.values_mut() {
            for system in parallel_systems {
                system(&mut builder);
            }
            builder.flush();
        }

        let mut resources = Resources::default();
        resources.insert(self.update_rate);
        resources.insert(time);
        resources.merge(update_resources);

        let mut schedule = builder.build();
        let mut updated = false;
        let delta = self.update_loop.update(|| {
            updated = true;
            let timer = self.update_reporter.begin();
            schedule.execute(&mut self.world, &mut resources);
            self.update_reporter.end(timer);
        });

        self.update_rate = resources.remove().unwrap();

        (delta * delta_mult, updated)
    }

    fn frame(&mut self, delta_mult: f32, frame_resources: Resources) {
        let timer = self.frame_reporter.begin();
        let mut builder = Schedule::builder();

        for system in self.frame_systems.iter_mut() {
            system(&mut builder);
        }

        builder.flush();

        for parallel_systems in self.internal_frame_systems.values_mut() {
            for system in parallel_systems {
                system(&mut builder);
            }
            builder.flush();
        }

        let mut resources = Resources::default();
        resources.merge(frame_resources);
        resources.insert(self.update_rate);
        resources.insert(self.batcher.take().unwrap());
        resources.insert(Time { delta_mult });

        builder.build().execute(&mut self.world, &mut resources);

        self.batcher = Some(resources.remove().unwrap());
        self.update_rate = resources.remove().unwrap();
        self.frame_reporter.end(timer);
    }
}

impl Deref for World {
    type Target = legion::World;

    fn deref(&self) -> &Self::Target {
        &self.world
    }
}

impl DerefMut for World {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.world
    }
}
