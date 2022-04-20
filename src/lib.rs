#![feature(drain_filter)]
#![feature(type_alias_impl_trait)]
#![feature(generic_associated_types)]
// #![feature(adt_const_params)]
// #![feature(trait_alias)]
// #![feature(iter_partition_in_place)]

//

use colorful::Colorful;
use futures::executor::block_on;
use std::sync::Arc;
use surface::{ISurface, Surface};
use wgpu::{
    util::{backend_bits_from_env, power_preference_from_env},
    Backends, Device, DeviceDescriptor, Features, Instance, Limits, PowerPreference, Queue,
    RequestAdapterOptionsBase,
};
use winit::{
    event_loop::EventLoop,
    window::{Window, WindowBuilder},
};

//

pub use frame::Frame;
pub use game_loop::{AnyEngine, Runnable};
// pub trait Runnable = game_loop::Runnable<Engine>;
pub type GameLoop = game_loop::GameLoop<Engine>;

//

pub extern crate glam;
pub extern crate winit;

//

// pub mod batch;
// pub mod packer;
// pub mod program;
// pub mod text;
pub mod buffer;
pub mod frame;
pub mod prelude;
pub mod shader;

//

mod surface;

//

pub struct Engine {
    surface: Surface,
    window: Arc<Window>,
    device: Arc<Device>,
    queue: Arc<Queue>,

    event_loop: Option<EventLoop<()>>,
}

//

impl Engine {
    pub fn new(wb: WindowBuilder) -> Self {
        let event_loop = EventLoop::new();
        let window_builder = wb.with_visible(false);
        let window = Arc::new(window_builder.build(&event_loop).unwrap());

        let backend = backend_bits_from_env().unwrap_or(Backends::VULKAN /* all() */);
        let instance = Arc::new(Instance::new(backend));
        let surface = ISurface::new(window.clone(), instance.clone());
        let adapter = Arc::new(
            block_on(instance.request_adapter(&RequestAdapterOptionsBase {
                power_preference:
                    power_preference_from_env().unwrap_or(PowerPreference::HighPerformance),
                force_fallback_adapter: false,
                compatible_surface: Some(&surface),
            }))
            .expect("No suitable GPUs"),
        );

        if log::log_enabled!(log::Level::Debug) {
            let gpu_info = adapter.get_info();
            let api = format!("{:?}", gpu_info.backend).red();
            let name = gpu_info.name.blue();
            let ty = format!("{:?}", gpu_info.device_type).green();

            log::debug!("GPU API: {api}");
            log::debug!("GPU: {name} ({ty})");
        }

        let (device, queue) = block_on(adapter.request_device(
            &DeviceDescriptor {
                label: label!(),
                features: Features::empty(),
                limits: Limits::downlevel_webgl2_defaults(),
            },
            None,
        ))
        .unwrap();
        let (device, queue) = (Arc::new(device), Arc::new(queue));

        /* device.on_uncaptured_error(|err| match err {
            wgpu::Error::OutOfMemory { source } => panic!("Out of memory: {source}"),
            wgpu::Error::Validation {
                source,
                description,
            } => panic!("Validation error: {source}: {description}"),
        }); */

        let surface = surface.complete(&adapter, device.clone());

        let event_loop = Some(event_loop);

        Self {
            surface,
            window,
            device,
            queue,

            event_loop,
        }
    }
}

//

impl AnyEngine for Engine {
    type Frame = Frame;

    fn get_frame(&mut self) -> Self::Frame {
        Self::Frame::new(&self.device, self.queue.clone(), &mut self.surface)
    }

    fn finish_frame(&mut self, _: Self::Frame) {
        // drop frame
    }

    fn get_window(&self) -> &Window {
        &self.window
    }

    fn take_event_loop(&mut self) -> EventLoop<()> {
        self.event_loop.take().unwrap()
    }
}

//

pub trait BuildEngine {
    fn build_engine(self) -> Engine;
}

impl BuildEngine for WindowBuilder {
    fn build_engine(self) -> Engine {
        Engine::new(self)
    }
}

//

// shamelessly stolen from: https://github.com/popzxc/stdext-rs
#[macro_export]
macro_rules! function_name {
    () => {{
        // Okay, this is ugly, I get it. However, this is the best we can get on a stable rust.
        fn f() {}
        fn type_name_of<T>(_: T) -> &'static str {
            std::any::type_name::<T>()
        }
        let name = type_name_of(f);
        // `3` is the length of the `::f`.
        &name[..name.len() - 3]
    }};
}

#[macro_export]
macro_rules! label {
    () => {
        Some($crate::function_name!())
    };
}
