use self::{
    belt::BeltPool,
    catcher::Catcher,
    poll::PollThread,
    surface::{ISurface, Surface},
};
use crate::{label, prelude::Frame, DeviceStorage};
use colorful::Colorful;
use main_game_loop::event::Event;
use std::{
    future::Future,
    sync::{
        atomic::{AtomicUsize, Ordering},
        Arc,
    },
};
use wgpu::{
    util::power_preference_from_env, Adapter, Device, DeviceDescriptor, Features, Instance, Limits,
    PowerPreference, Queue, RequestAdapterOptionsBase, TextureFormat,
};
use winit::{event::WindowEvent, window::Window};

//

pub mod prelude;
pub mod surface;

//

mod belt;
mod catcher;
mod poll;

//

/// This handles the gpu logical device instance
pub struct Target {
    pub(crate) device: Arc<Device>,
    pub(crate) queue: Arc<Queue>,

    pub(crate) surface: Option<Surface>,
    pub(crate) belts: Arc<BeltPool>,
    catcher: Catcher,

    // tracing
    frame_id: AtomicUsize,

    // thread to poll the device
    _poll: PollThread,
}

//

impl Target {
    /// Create a new render target that is bound to a window
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

        Self::new_finish(device, queue, surface)
    }

    /// Create a new render target that doesn't require a window
    pub async fn new_headless(instance: Arc<Instance>, device_storage: DeviceStorage) -> Self {
        let (_, device, queue) = Self::new_with_opt(instance, None, device_storage).await;

        Self::new_finish(device, queue, None)
    }

    /// check if objects created with `self` target
    /// can be used with the `other` target
    ///
    /// this just checks if both [`Target`]s share their
    /// logical devices
    pub fn compatible_with(&self, other: &Target) -> bool {
        Arc::ptr_eq(&self.device, &other.device) && Arc::ptr_eq(&self.queue, &other.queue)
    }

    /// reconfigures the swapchain if the window is
    /// resized
    ///
    /// calling this in the event function is not
    /// often needed, but it is recommended
    ///
    /// wayland requires calling this (idk why)
    ///
    /// [`Self::resized`] is an alternative to this
    pub fn event(&mut self, event: &Event) {
        let Some(window) = self.get_window() else {
            return;
        };

        if let Event::WindowEvent {
            window_id,
            event: WindowEvent::Resized(_),
        } = event
        {
            if *window_id == window.id() {
                self.resized();
            }
        }
    }

    /// reconfigures the swapchain if the window is
    /// resized
    ///
    /// calling this in the event function is not
    /// often needed, but it is recommended
    ///
    /// wayland requires calling this (idk why)
    ///
    /// [`Self::event`] is an alternative to this
    pub fn resized(&mut self) {
        let Some(surface) = &mut self.surface else {
            return
        };

        surface.configure();
    }

    /// start rendering a new frame
    ///
    /// the first frame sets the window visible
    ///
    /// # Panics
    ///
    /// This function panics if used in headless mode. FIXME:
    #[must_use]
    pub fn get_frame(&mut self) -> Frame {
        let frame_id = self.frame_id.fetch_add(1, Ordering::Relaxed);

        // first frame sets the window visible
        if frame_id == 0 {
            if let Some(window) = self.get_window() {
                window.set_visible(true);
            }
        }

        Frame::new(
            &self.device,
            self.queue.clone(),
            self.surface.as_mut().expect("TODO: Draw in headless mode"),
            self.belts.clone(),
            frame_id,
        )
    }

    /// make the first frame NOT automatically set the window visible
    pub fn no_auto_visible(&self) {
        self.frame_id.fetch_add(1, Ordering::SeqCst);
    }

    /// finish the frame
    #[deprecated]
    pub fn finish_frame(&mut self, _: Frame) {}

    /// set vertical sync preference
    ///
    /// does nothing with headless targets
    pub fn set_vsync(&mut self, on: bool) {
        if let Some(s) = self.surface.as_mut() {
            s.set_vsync(on);
        }
    }

    /// get the current vertical sync preference
    ///
    /// not possible with headless targets
    pub fn get_vsync(&self) -> Option<bool> {
        self.surface.as_ref().map(|s| s.get_vsync())
    }

    /// get the window bound to this render target
    ///
    /// not possible with headless targets
    pub fn get_window(&self) -> Option<Arc<Window>> {
        self.surface.as_ref().map(|surface| surface.get_window())
    }

    /// get the texture format this target prefers
    ///
    /// [`TextureFormat::Rgba8Unorm`] is returned with headless targets
    pub fn get_format(&self) -> TextureFormat {
        self.surface
            .as_ref()
            .map(|surface| surface.format())
            .unwrap_or(TextureFormat::Rgba8Unorm)
    }

    /// get the logical device
    pub fn get_device(&self) -> Arc<Device> {
        self.device.clone()
    }

    /// run something while listening for wgpu errors
    pub fn catch_error<T, F: FnOnce(&Self) -> T>(&self, f: F) -> Result<T, String> {
        Catcher::catch_error(self, f)
    }

    /// run something and await on it while listening for wgpu errors
    pub async fn catch_error_async<T, Fut, F>(&self, f: F) -> Result<T, String>
    where
        F: FnOnce(&Self) -> Fut,
        Fut: Future<Output = T>,
    {
        Catcher::catch_error_async(self, f).await
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
        if tracing::enabled!(tracing::Level::DEBUG) {
            let gpu_info = adapter.get_info();
            let api = format!("{:?}", gpu_info.backend).red();
            let name = gpu_info.name.blue();
            let ty = format!("{:?}", gpu_info.device_type).green();

            tracing::debug!("GPU API: {api}");
            tracing::debug!("GPU: {name} ({ty})");
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

    fn new_finish(device: Arc<Device>, queue: Arc<Queue>, surface: Option<Surface>) -> Self {
        // create a belt for fast data uploading
        let belts = Arc::new(BeltPool::new());

        // create a catcher to catch non fatal errors
        // for example: shader compilation errors
        let catcher = Catcher::new(&device);

        // create a poll thread to allow wgpu wait operations to work
        let _poll = PollThread::new(device.clone());

        Self {
            device,
            queue,

            surface,
            belts,
            catcher,

            frame_id: AtomicUsize::new(0),

            _poll,
        }
    }
}
