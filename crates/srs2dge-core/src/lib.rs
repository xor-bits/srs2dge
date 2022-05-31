// #![feature(generic_associated_types)]
// #![feature(type_alias_impl_trait)]
// #![feature(drain_filter)]
// #![feature(const_fn_floating_point_arithmetic)]
// #![feature(generic_const_exprs)]

//

use main_game_loop::event::EventLoopTarget;
use std::sync::{Arc, RwLock};
use target::Target;
use wgpu::{util::backend_bits_from_env, Adapter, Backends, Device, Instance, Queue};
use winit::{
    error::OsError,
    window::{Window, WindowBuilder},
};

//

pub use colorful;
pub use log;

pub use glam;
pub use main_game_loop;
pub use naga;
pub use wgpu;
pub use winit;

pub use image;
pub use rapid_qoi;

pub use bytemuck;
pub use rand;
pub use serde;

pub use integer_sqrt;

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

//

pub type DeviceStorage = Arc<RwLock<Vec<(Arc<Adapter>, Arc<Device>, Arc<Queue>)>>>;

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

    pub async fn new_target(&self, window: Arc<Window>) -> Target {
        #[cfg(target_arch = "wasm32")]
        {
            use winit::platform::web::WindowExtWebSys;

            let win = web_sys::window().unwrap();
            let doc = win.document().unwrap();

            doc.body()
                .unwrap()
                .append_child(&web_sys::Element::from(window.canvas()))
                .unwrap();
        }

        Target::new(self.instance.clone(), window, self.device_storage.clone()).await
    }

    pub async fn new_target_element_id(&self, window: Arc<Window>, canvas_div_id: &str) -> Target {
        let _ = canvas_div_id;
        #[cfg(target_arch = "wasm32")]
        {
            use winit::platform::web::WindowExtWebSys;

            let win = web_sys::window().unwrap();
            let doc = win.document().unwrap();

            doc.get_element_by_id(canvas_div_id)
                .unwrap()
                .append_child(&web_sys::Element::from(window.canvas()))
                .unwrap();
        }

        Target::new(self.instance.clone(), window, self.device_storage.clone()).await
    }

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

    pub async fn new_target_headless(&self) -> Target {
        Target::new_headless(self.instance.clone(), self.device_storage.clone()).await
    }

    fn make_instance() -> Arc<Instance> {
        // detect renderdoc
        #[cfg(not(target_arch = "wasm32"))]
        let renderdoc = std::env::vars().map(|(name, _)| name).any(|name| {
            matches!(
                name.as_str(),
                "ENABLE_VULKAN_RENDERDOC_CAPTURE"
                    | "RENDERDOC_CAPFILE"
                    | "RENDERDOC_CAPOPTS"
                    | "RENDERDOC_DEBUG_LOG_FILE"
            )
        });
        #[cfg(target_arch = "wasm32")]
        let renderdoc = false;

        // force default to vulkan if renderdoc was detected
        let default = if renderdoc {
            Backends::VULKAN
        } else {
            Backends::all()
        };

        Arc::new(Instance::new(backend_bits_from_env().unwrap_or(default)))
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
