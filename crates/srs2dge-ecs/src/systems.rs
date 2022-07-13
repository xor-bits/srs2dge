use legion::{
    systems::{Builder, ParallelRunnable},
    Resources, Schedule,
};
use srs2dge_core::{
    log,
    main_game_loop::{
        report::Reporter,
        update::{UpdateLoop, UpdateRate},
    },
};
use std::{any::type_name, collections::BTreeMap};

use crate::prelude::Time;

//

type SystemCreator = Box<dyn NamedFnMut>;

//

pub trait NamedFnMut {
    fn call(&mut self, builder: &mut Builder);

    fn name(&self) -> &'static str;
}

impl<F> NamedFnMut for F
where
    F: FnMut(&mut Builder),
{
    fn call(&mut self, builder: &mut Builder) {
        (self)(builder);
    }

    fn name(&self) -> &'static str {
        type_name::<Self>()
    }
}

//

#[derive(Default)]
pub struct Systems {
    pub reporter: Reporter,
    pub systems: Vec<SystemCreator>,
    pub internal_systems: BTreeMap<u32, Vec<SystemCreator>>,
}

//

impl Systems {
    pub fn insert<R: ParallelRunnable + 'static, S: FnMut() -> R + 'static>(
        &mut self,
        mut system: S,
    ) {
        self.systems.push(Box::new(move |builder: &mut Builder| {
            builder.add_system(system());
        }));
    }

    /// ### Internally used indices:
    ///
    /// Updates:
    ///  - ..100 : **FREE**
    ///  - 100 : `RigidBody2D`
    ///  - 101..200 : **FREE**
    ///  - 200 : `Sprite`
    ///  - 201.. : **FREE**
    ///
    /// Frames:
    ///  - ..200 : **FREE**
    ///  - 200..202 : `Sprite`
    ///  - 202.. : **FREE**
    pub fn insert_internal<R: ParallelRunnable + 'static, S: FnMut() -> R + 'static>(
        &mut self,
        index: u32,
        mut system: S,
    ) {
        let parallel_systems = self.internal_systems.entry(index).or_default();
        parallel_systems.push(Box::new(move |builder: &mut Builder| {
            builder.add_system(system());
        }));
    }

    pub(crate) fn update(
        &mut self,
        resources: &mut Resources,
        rate: &mut UpdateRate,
        update_loop: &mut UpdateLoop,
        world: &mut legion::World,
    ) -> (f32, bool) {
        let delta_mult = rate.to_interval().as_secs_f32();
        let time = Time { delta_mult };
        let mut builder = Schedule::builder();

        // schedule normal systems
        for system in self.systems.iter_mut() {
            system.call(&mut builder);
            log::trace!("update system {} scheduled", system.name());
        }
        builder.flush();

        // schedule internal systems
        for parallel_systems in self.internal_systems.values_mut() {
            for system in parallel_systems {
                system.call(&mut builder);
                log::trace!("internal update system {} scheduled", system.name());
            }
            builder.flush();
        }

        // insert timers
        let old_rate = resources.remove::<UpdateRate>();
        let old_time = resources.remove::<Time>();
        resources.insert(*rate);
        resources.insert(time);

        // run
        let mut schedule = builder.build();
        let mut updated = false;
        let delta = update_loop.update(|| {
            updated = true;
            let timer = self.reporter.begin();
            schedule.execute(world, resources);
            self.reporter.end(timer);
        });

        // cleanup
        *rate = resources.remove().unwrap();
        if let Some(rate) = old_rate {
            resources.insert(rate);
        }
        if let Some(time) = old_time {
            resources.insert(time);
        }

        (delta * delta_mult, updated)
    }

    pub(crate) fn frame(
        &mut self,
        resources: &mut Resources,
        rate: &mut UpdateRate,
        world: &mut legion::World,
        delta_mult: f32,
    ) {
        let timer = self.reporter.begin();
        let mut builder = Schedule::builder();

        // schedule normal systems
        for system in self.systems.iter_mut() {
            system.call(&mut builder);
            log::trace!("frame system {} scheduled", system.name());
        }
        builder.flush();

        // schedule internal systems
        for parallel_systems in self.internal_systems.values_mut() {
            for system in parallel_systems {
                system.call(&mut builder);
                log::trace!("internal frame system {} scheduled", system.name());
            }
            builder.flush();
        }

        // insert timers
        let old_rate = resources.remove::<UpdateRate>();
        let old_time = resources.remove::<Time>();
        resources.insert(*rate);
        resources.insert(Time { delta_mult });

        builder.build().execute(world, resources);

        // cleanup
        *rate = resources.remove().unwrap();
        if let Some(rate) = old_rate {
            resources.insert(rate);
        }
        if let Some(time) = old_time {
            resources.insert(time);
        }

        self.reporter.end(timer);
    }
}
