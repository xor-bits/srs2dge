use main_game_loop::event::EventLoop;
use main_game_loop::{engine::AnyEngine, event::EventReceiver};
use std::sync::Arc;
use std::thread;
use target::Target;
use wgpu::{util::backend_bits_from_env, Backends, Instance};
use winit::error::OsError;
use winit::window::{Window, WindowBuilder};

//

pub use frame::Frame;

//

pub extern crate glam;
pub extern crate winit;

//

// pub mod packer;
// pub mod program;
// pub mod text;
pub mod batch;
pub mod buffer;
pub mod frame;
pub mod prelude;
pub mod shader;
pub mod target;

//

pub struct Engine {
    instance: Arc<Instance>,
    event_receiver: Option<EventReceiver>,
}

//

impl Default for Engine {
    fn default() -> Self {
        Self {
            instance: Self::make_instance(),
            event_receiver: Default::default(),
        }
    }
}

impl Engine {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn new_target(&mut self) -> Result<Target, OsError> {
        self.new_target_from_builder(WindowBuilder::new())
    }

    pub fn new_target_from(&mut self, window: Arc<Window>) -> Target {
        Target::new(self.instance.clone(), window)
    }

    pub fn new_target_from_builder(&mut self, builder: WindowBuilder) -> Result<Target, OsError> {
        let window = Arc::new(self.create_window(builder)?);
        Ok(self.new_target_from(window))
    }

    fn make_instance() -> Arc<Instance> {
        // renderdoc
        let backend = Backends::VULKAN;

        // default
        // let backend = Backends::all();

        let backend = backend_bits_from_env().unwrap_or(backend);
        Arc::new(Instance::new(backend))
    }
}

//

impl AnyEngine for Engine {
    fn run<F>(mut self, f: F) -> !
    where
        F: FnOnce(Self) + Send + 'static,
    {
        let (sender, receiver) = EventLoop::new();
        self.event_receiver = Some(receiver);
        thread::spawn(move || f(self));
        sender.run();
    }

    fn event_receiver(&mut self) -> &mut EventReceiver {
        self.event_receiver.as_mut().expect("GameLoop not running")
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
