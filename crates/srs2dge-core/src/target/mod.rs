use self::{
    belt::Belt,
    catcher::Catcher,
    surface::{ISurface, Surface},
};
use crate::{label, prelude::Frame, DeviceStorage};
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
    init: bool,
}

//

impl Target {
    pub async fn new(
        instance: Arc<Instance>,
        window: Arc<Window>,
        device_storage: DeviceStorage,
    ) -> Self {
        // create a surface that is compatible with both the window and the instance
        let surface = ISurface::new(window, instance.clone());

        // create a device and a queue for it
        let (adapter, device, queue) =
            Self::new_with_opt(instance, Some(&surface), device_storage).await;

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
            init: true,
        }
    }

    pub async fn new_headless(instance: Arc<Instance>, device_storage: DeviceStorage) -> Self {
        let (_, device, queue) = Self::new_with_opt(instance, None, device_storage).await;

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
            init: true,
        }
    }

    async fn new_with_opt(
        instance: Arc<Instance>,
        surface: Option<&wgpu::Surface>,
        device_storage: DeviceStorage,
    ) -> (Arc<Adapter>, Arc<Device>, Arc<Queue>) {
        // 'borrow' a device and a queue if this surface is compatible with any previous ones
        // or create new if there were none
        if let Some(pre_existing) = Self::try_borrow_device(surface, device_storage.clone()) {
            // borrow
            pre_existing
        } else {
            // create
            // get a GPU
            let adapter = Self::make_adapter(surface, &instance).await;

            // print out some info about the selected GPU
            Self::debug_report(&adapter);

            // create a logical device and a queue for it
            let (device, queue) = Self::make_device(&adapter).await;

            // push to the device storage
            if let Ok(mut write) = device_storage.write() {
                write.push((adapter.clone(), device.clone(), queue.clone()));
            }

            (adapter, device, queue)
        }
    }

    /// check if objects created with `self` target
    /// can be used with the `other` target
    pub fn compatible_with(&self, other: &Target) -> bool {
        Arc::ptr_eq(&self.device, &other.device) && Arc::ptr_eq(&self.queue, &other.queue)
    }

    #[must_use]
    pub fn get_frame(&mut self) -> Frame {
        if self.active {
            panic!("Earlier frame was not finished before starting a new one");
        }

        if self.init {
            self.init = false;
            self.get_window().unwrap().set_visible(true);
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

    pub fn set_vsync(&mut self, on: bool) {
        if let Some(s) = self.surface.as_mut() {
            s.set_vsync(on);
        }
    }

    pub fn get_vsync(&self) -> Option<bool> {
        self.surface.as_ref().map(|s| s.get_vsync())
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

    fn try_borrow_device(
        compatible_surface: Option<&wgpu::Surface>,
        device_storage: DeviceStorage,
    ) -> Option<(Arc<Adapter>, Arc<Device>, Arc<Queue>)> {
        device_storage
            .read()
            .ok()?
            .iter()
            .find(|(adapter, _, _)| {
                if let Some(surface) = compatible_surface {
                    adapter.is_surface_supported(surface)
                } else {
                    true
                }
            })
            .cloned()
    }

    async fn make_adapter(
        compatible_surface: Option<&wgpu::Surface>,
        instance: &Instance,
    ) -> Arc<Adapter> {
        let options = RequestAdapterOptionsBase {
            power_preference: power_preference_from_env()
                .unwrap_or(PowerPreference::HighPerformance),
            compatible_surface,
            ..Default::default()
        };
        Arc::new(
            instance
                .request_adapter(&options)
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
                        // max_texture_dimension_2d: 16384,
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
