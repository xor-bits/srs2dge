use std::sync::Arc;
use target::Target;
use wgpu::{util::backend_bits_from_env, Backends, Instance};
use winit::window::Window;

//

pub use frame::Frame;

//

pub extern crate glam;
pub extern crate winit;

//

pub mod batch;
pub mod buffer;
pub mod frame;
pub mod packer;
pub mod prelude;
pub mod shader;
pub mod target;
pub mod text;

//

pub struct Engine {
    instance: Arc<Instance>,
}

//

impl Default for Engine {
    fn default() -> Self {
        Self {
            instance: Self::make_instance(),
        }
    }
}

impl Engine {
    pub fn new() -> Self {
        Self::default()
    }

    pub async fn new_target(&mut self, window: Arc<Window>) -> Target {
        #[cfg(target_arch = "wasm32")]
        {
            use winit::platform::web::WindowExtWebSys;
            web_sys::window()
                .and_then(|win| win.document())
                .and_then(|doc| doc.body())
                .and_then(|body| {
                    body.append_child(&web_sys::Element::from(window.canvas()))
                        .ok()
                })
                .expect("couldn't append canvas to document body");
        }

        Target::new(self.instance.clone(), window).await
    }

    fn make_instance() -> Arc<Instance> {
        // renderdoc
        // let backend = Backends::VULKAN;

        // default
        let backend = Backends::all();

        // webgl
        // let backend = Backends::GL;

        let backend = backend_bits_from_env().unwrap_or(backend);
        Arc::new(Instance::new(backend))
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
