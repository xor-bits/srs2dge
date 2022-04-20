#![feature(drain_filter)]
#![feature(type_alias_impl_trait)]
#![feature(generic_associated_types)]
// #![feature(iter_partition_in_place)]

//

use glium::{
    backend::{Context, Facade},
    glutin::ContextBuilder,
    Display, Frame,
};
use main_game_loop::AnyEngine;
use std::rc::Rc;
use winit::{
    event_loop::EventLoop,
    window::{Window, WindowBuilder},
};

//

#[macro_use]
pub extern crate glium;
pub extern crate glam;
pub extern crate winit;

//

pub mod batch;
pub mod packer;
pub mod program;
pub mod text;

//

pub struct Engine {
    pub facade: Display,
    event_loop: Option<EventLoop<()>>,
}

//

impl Engine {
    pub fn new(wb: WindowBuilder) -> Self {
        let event_loop = EventLoop::new();
        let window_builder = wb.with_visible(false);
        let context = ContextBuilder::new()
            // .with_vsync(true)
            .build_windowed(window_builder, &event_loop)
            .unwrap();
        let facade = Display::from_gl_window(context).unwrap();

        log::debug!("OpenGL Vendor: {}", facade.get_opengl_vendor_string());
        log::debug!("OpenGL Renderer: {}", facade.get_opengl_renderer_string());
        log::debug!("OpenGL Version: {}", facade.get_opengl_version_string());

        let event_loop = Some(event_loop);

        Self { facade, event_loop }
    }
}

//

impl AnyEngine for Engine {
    type Frame = Frame;

    fn get_frame(&mut self) -> Self::Frame {
        self.facade.draw()
    }

    fn finish_frame(&mut self, frame: Self::Frame) {
        frame.finish().unwrap();
    }

    fn use_window<F, T>(&self, f: F) -> T
    where
        F: FnOnce(&Window) -> T,
    {
        f(self.facade.gl_window().window())
    }

    fn take_event_loop(&mut self) -> EventLoop<()> {
        self.event_loop.take().unwrap()
    }
}

impl Facade for Engine {
    fn get_context(&self) -> &Rc<Context> {
        self.facade.get_context()
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
