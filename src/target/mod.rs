use self::{
    belt::Belt,
    catcher::Catcher,
    surface::{ISurface, Surface},
};
use crate::{label, Frame};
use colorful::Colorful;
use futures::executor::block_on;
use std::sync::Arc;
use wgpu::{
    util::power_preference_from_env, Adapter, Device, DeviceDescriptor, Features, Instance, Limits,
    PowerPreference, Queue, RequestAdapterOptionsBase,
};
use winit::window::Window;

//

pub mod surface;

//

mod belt;
mod catcher;

//

pub struct Target {
    pub(crate) device: Arc<Device>,
    pub(crate) queue: Arc<Queue>,

    pub(crate) surface: Surface,
    pub(crate) belt: Belt,
    catcher: Catcher,

    active: bool,
}

//

impl Target {
    pub fn new(instance: Arc<Instance>, window: Arc<Window>) -> Self {
        // create a surface that is compatible with both the window and the instance
        let surface = ISurface::new(window, instance.clone());

        // get a GPU
        let adapter = Self::make_adapter(&surface, &instance);

        // print out some info about the selected GPU
        Self::debug_report(&adapter);

        // create a logical device and a queue for it
        let (device, queue) = Self::make_device(&adapter);

        // complete the surface (ready for rendering)
        let surface = surface.complete(&adapter, device.clone());

        // create a belt for fast data uploading
        let belt = Belt::new(device.clone());

        // create a catcher to catch non fatal errors
        // for example: shader compilation errors
        let catcher = Catcher::new(&device);

        Self {
            device,
            queue,

            surface,
            belt,
            catcher,

            active: false,
        }
    }

    #[must_use]
    pub fn get_frame(&mut self) -> Frame {
        if self.active {
            panic!("Earlier frame was not finished before starting a new one");
        }

        Frame::new(
            &self.device,
            self.queue.clone(),
            &mut self.surface,
            self.belt.get(),
        )
    }

    pub fn finish_frame(&mut self, frame: Frame) {
        self.belt.set(frame.finish());
    }

    pub fn get_window(&self) -> Arc<Window> {
        self.surface.get_window()
    }

    pub(crate) fn catch_error<T, F: FnOnce(&Self) -> T>(&self, f: F) -> Result<T, String> {
        Catcher::catch_error(self, f)
    }

    fn make_adapter(surface: &ISurface, instance: &Instance) -> Arc<Adapter> {
        Arc::new(
            block_on(instance.request_adapter(&RequestAdapterOptionsBase {
                power_preference:
                    power_preference_from_env().unwrap_or(PowerPreference::HighPerformance),
                force_fallback_adapter: false,
                compatible_surface: Some(surface),
            }))
            .expect("No suitable GPUs"),
        )
    }

    fn debug_report(adapter: &Adapter) {
        if log::log_enabled!(log::Level::Debug) {
            let gpu_info = adapter.get_info();
            let api = format!("{:?}", gpu_info.backend).red();
            let name = gpu_info.name.blue();
            let ty = format!("{:?}", gpu_info.device_type).green();

            log::debug!("GPU API: {api}");
            log::debug!("GPU: {name} ({ty})");
        }
    }

    fn make_device(adapter: &Adapter) -> (Arc<Device>, Arc<Queue>) {
        let (device, queue) = block_on(adapter.request_device(
            &DeviceDescriptor {
                label: label!(),
                features: Features::empty(),
                limits: Limits::downlevel_webgl2_defaults(),
            },
            None,
        ))
        .unwrap();
        (Arc::new(device), Arc::new(queue))
    }
}
