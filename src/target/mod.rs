use self::{
    belt::Belt,
    catcher::Catcher,
    surface::{ISurface, Surface},
};
use crate::{label, Frame};
use colorful::Colorful;
use std::sync::Arc;
use wgpu::{
    util::power_preference_from_env, Adapter, Device, DeviceDescriptor, Features, Instance, Limits,
    PowerPreference, Queue, RequestAdapterOptionsBase, TextureFormat,
};
use winit::window::Window;

//

pub mod prelude;
pub mod surface;

//

mod belt;
mod catcher;

//

pub struct Target {
    pub(crate) device: Arc<Device>,
    pub(crate) queue: Arc<Queue>,

    pub(crate) surface: Option<Surface>,
    pub(crate) belt: Belt,
    catcher: Catcher,

    active: bool,
}

//

impl Target {
    pub async fn new(instance: Arc<Instance>, window: Arc<Window>) -> Self {
        // create a surface that is compatible with both the window and the instance
        let surface = ISurface::new(window, instance.clone());

        // get a GPU
        let adapter = Self::make_adapter(Some(&surface), &instance).await;

        // print out some info about the selected GPU
        Self::debug_report(&adapter);

        // create a logical device and a queue for it
        let (device, queue) = Self::make_device(&adapter).await;

        // complete the surface (ready for rendering)
        let surface = Some(surface.complete(&adapter, device.clone()));

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

    pub async fn new_headless(instance: Arc<Instance>) -> Self {
        // get a GPU
        let adapter = Self::make_adapter(None, &instance).await;

        // print out some info about the selected GPU
        Self::debug_report(&adapter);

        // create a logical device and a queue for it
        let (device, queue) = Self::make_device(&adapter).await;

        // create a belt for fast data uploading
        let belt = Belt::new(device.clone());

        // create a catcher to catch non fatal errors
        // for example: shader compilation errors
        let catcher = Catcher::new(&device);

        Self {
            device,
            queue,

            surface: None,
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
            self.surface.as_mut().expect("TODO: Draw in headless mode"),
            self.belt.get(),
        )
    }

    pub fn finish_frame(&mut self, frame: Frame) {
        self.belt.set(frame.finish())
    }

    pub fn get_window(&self) -> Option<Arc<Window>> {
        self.surface.as_ref().map(|surface| surface.get_window())
    }

    pub fn get_format(&self) -> TextureFormat {
        self.surface
            .as_ref()
            .map(|surface| surface.format())
            .unwrap_or(TextureFormat::Rgba8Unorm)
    }

    pub fn get_device(&self) -> Arc<Device> {
        self.device.clone()
    }

    pub fn catch_error<T, F: FnOnce(&Self) -> T>(&self, f: F) -> Result<T, String> {
        Catcher::catch_error(self, f)
    }

    async fn make_adapter(
        compatible_surface: Option<&wgpu::Surface>,
        instance: &Instance,
    ) -> Arc<Adapter> {
        Arc::new(
            instance
                .request_adapter(&RequestAdapterOptionsBase {
                    power_preference: power_preference_from_env()
                        .unwrap_or(PowerPreference::HighPerformance),
                    force_fallback_adapter: false,
                    compatible_surface,
                })
                .await
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

    async fn make_device(adapter: &Adapter) -> (Arc<Device>, Arc<Queue>) {
        let (device, queue) = adapter
            .request_device(
                &DeviceDescriptor {
                    label: label!(),
                    features: Features::empty(),
                    limits: Limits {
                        max_texture_dimension_2d: 16384,
                        ..Limits::downlevel_webgl2_defaults()
                    },
                },
                None,
            )
            .await
            .unwrap();
        (Arc::new(device), Arc::new(queue))
    }
}
