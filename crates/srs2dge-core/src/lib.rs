// #![feature(generic_associated_types)]
// #![feature(type_alias_impl_trait)]
// #![feature(drain_filter)]
// #![feature(const_fn_floating_point_arithmetic)]
// #![feature(generic_const_exprs)]

//

use cfg_if::cfg_if;
use main_game_loop::event::EventLoopTarget;
use std::sync::{Arc, RwLock};
use target::Target;
use tokio::runtime::{Builder, Runtime};
use wgpu::{util::backend_bits_from_env, Adapter, Backends, Device, Instance, Queue};
use winit::{
    error::OsError,
    window::{Window, WindowBuilder},
};

//

pub use bytemuck;
pub use colorful;
pub use glam;
pub use image;
pub use main_game_loop;
pub use naga;
pub use serde;
pub use wgpu;
pub use winit;

//

pub mod batch;
pub mod buffer;
pub mod color;
pub mod frame;
pub mod packer;
pub mod prelude;
pub mod shader;
pub mod target;
pub mod texture;
pub mod util;

//

pub type DeviceStorage = Arc<RwLock<Vec<(Arc<Adapter>, Arc<Device>, Arc<Queue>)>>>;

//

#[macro_export]
macro_rules! app {
    ($app:tt) => {
        $crate::app!($app::init, $app::event, $app::draw)
    };

    ($init:expr, $event:expr, $draw:expr) => {
        $crate::init_log();

        let rt = $crate::init_tokio();

        let target = EventLoop::new();
        let mut app = rt.block_on($init(&target));

        target.run(move |e, t, c| {
            rt.block_on(async {
                if should_draw(&e) {
                    ($draw(&mut app)).await;
                }

                ($event(&mut app, e, t, c)).await;
            })
        });
    };
}

pub fn init_tokio() -> Runtime {
    #[cfg(not(target_arch = "wasm32"))]
    let mut builder = Builder::new_multi_thread();
    #[cfg(target_arch = "wasm32")]
    let mut builder = Builder::new_current_thread();

    let runtime = builder
        .enable_all()
        .build()
        .expect("Failed to start a tokio runtime");

    tracing::debug!("Tokio runtime started");

    runtime
}

pub fn init_log() {
    #[allow(unused_assignments)]
    let mut plat = "unknown";

    cfg_if! {
        if #[cfg(target_os = "android")] {
            use android_logger::{init_once, Config, FilterBuilder};

            init_once(
                Config::default().with_min_level(Level::Debug).with_filter(
                    FilterBuilder::new()
                        .parse("debug,winit=info,wgpu_core=info,wgpu_hal=info,naga=info,gilrs=info")
                        .build(),
                ),
            );

            plat = "android";
        } else if #[cfg(target_arch = "wasm32")] {
            std::panic::set_hook(Box::new(console_error_panic_hook::hook));
            console_log::init_with_level(tracing::Level::Debug).unwrap();

            plat = "wasm32";
        } else {
            tracing_subscriber::fmt::init();

            plat = "default";
        }
    };

    tracing::debug!(plat, "Logger initialized");
}

//

pub struct Engine {
    instance: Arc<Instance>,

    device_storage: DeviceStorage,
}

//

impl Default for Engine {
    fn default() -> Self {
        Self {
            instance: Self::make_instance(),

            device_storage: Default::default(),
        }
    }
}

impl Engine {
    pub fn new() -> Self {
        Self::default()
    }

    /// Create a new render target that is bound to a window
    pub async fn new_target(&self, window: Arc<Window>) -> Target {
        self.new_target_element_id(window, None).await
    }

    /// Create a new render target that is bound to a window
    ///
    /// Also construct the window's canvas and append it to an
    /// element with id `canvas_div_id`
    pub async fn new_target_element_id(
        &self,
        window: Arc<Window>,
        canvas_div_id: Option<&str>,
    ) -> Target {
        let _ = canvas_div_id;
        #[cfg(target_arch = "wasm32")]
        {
            use winit::platform::web::WindowExtWebSys;

            let win = web_sys::window().unwrap();
            let doc = win.document().unwrap();

            canvas_div_id
                .and_then(|id| doc.get_element_by_id(canvas_div_id).ok())
                .unwrap_or_else(|| doc.body().expect("Failing to read the DOM"))
                .append_child(&web_sys::Element::from(window.canvas()))
                .unwrap();
        }

        Target::new(self.instance.clone(), window, self.device_storage.clone()).await
    }

    /// Create a new window and a render target
    pub async fn new_target_default(&self, target: &EventLoopTarget) -> Result<Target, OsError> {
        Ok(self
            .new_target(Arc::new(
                WindowBuilder::new()
                    .with_visible(false)
                    .with_title(env!("CARGO_PKG_NAME"))
                    .build(target)?,
            ))
            .await)
    }

    /// Create a new render target that doesn't require a window
    pub async fn new_target_headless(&self) -> Target {
        Target::new_headless(self.instance.clone(), self.device_storage.clone()).await
    }

    /// returns the wgpu instance
    pub fn get_instance(&self) -> Arc<Instance> {
        self.instance.clone()
    }

    /// returns the device storage handled by this engine's
    /// wgpu instance
    pub fn get_device_storage(&self) -> DeviceStorage {
        self.device_storage.clone()
    }

    fn make_instance() -> Arc<Instance> {
        // backend_bits_from_env:
        // "vulkan" | "vk"          => Backends::VULKAN,
        // "dx12"   | "d3d12"       => Backends::DX12,
        // "dx11"   | "d3d11"       => Backends::DX11,
        // "metal"  | "mtl"         => Backends::METAL,
        // "opengl" | "gles" | "gl" => Backends::GL,
        // "webgpu"                 => Backends::BROWSER_WEBGPU,
        let mut backend = backend_bits_from_env();
        if let Some(backend) = &backend {
            tracing::info!("Using overwritten backend: {backend:?}");
        }

        // force default to vulkan if renderdoc was detected
        //
        // renderdoc freaks out if wgpu starts running both opengl and vulkan code
        if backend.is_none()
            && std::env::vars().map(|(name, _)| name).any(|name| {
                matches!(
                    name.as_str(),
                    "ENABLE_VULKAN_RENDERDOC_CAPTURE"
                        | "RENDERDOC_CAPFILE"
                        | "RENDERDOC_CAPOPTS"
                        | "RENDERDOC_DEBUG_LOG_FILE"
                )
            })
        {
            tracing::warn!("RenderDoc environment detected. Forcing Vulkan. You can force a specific WGPU backend with the environment value `WGPU_BACKEND`");
            backend = Some(Backends::VULKAN);
        }

        tracing::info!("Selected backend(s): {backend:?}");

        Arc::new(Instance::new(backend.unwrap_or(Backends::all())))
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
