#![feature(generic_associated_types)]
#![feature(type_alias_impl_trait)]

//

use main_game_loop::event::EventLoopTarget;
use std::sync::Arc;
use target::Target;
use wgpu::{util::backend_bits_from_env, Backends, Instance};
use winit::{
    error::OsError,
    window::{Window, WindowBuilder},
};

//

pub use frame::Frame;

//

pub extern crate bytemuck;
pub extern crate glam;
pub extern crate naga;
pub extern crate wgpu;
pub extern crate winit;

//

pub mod batch;
pub mod buffer;
pub mod frame;
pub mod gizmos;
pub mod packer;
pub mod prelude;
pub mod shader;
pub mod target;
pub mod text;
pub mod texture;

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

        Target::new(self.instance.clone(), window).await
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

        Target::new(self.instance.clone(), window).await
    }

    pub async fn new_target_default(&self, target: &EventLoopTarget) -> Result<Target, OsError> {
        Ok(self
            .new_target(Arc::new(
                WindowBuilder::new()
                    .with_title(env!("CARGO_PKG_NAME"))
                    .build(target)?,
            ))
            .await)
    }

    pub async fn new_target_headless(&self) -> Target {
        Target::new_headless(self.instance.clone()).await
    }

    fn make_instance() -> Arc<Instance> {
        // renderdoc
        // let default = Backends::VULKAN;

        // any
        let default = Backends::all();

        // webgl
        // let default = Backends::GL;

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
