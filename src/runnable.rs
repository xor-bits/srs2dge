use crate::Engine;
use glium::Frame;
pub use winit;
use winit::event::Event;

pub trait Runnable {
    #[allow(unused_variables)]
    fn update(&mut self, engine: &Engine) {}

    #[allow(unused_variables)]
    fn event(&mut self, engine: &Engine, event: &Event<()>) {}

    #[allow(unused_variables)]
    fn draw(&self, engine: &Engine, frame: &mut Frame, delta: f32) {}
}
