#![feature(drain_filter)]
#![feature(type_alias_impl_trait)]
// #![feature(iter_partition_in_place)]

//

use game_loop::AnyEngine;
use glium::{
    backend::{Context, Facade},
    glutin::ContextBuilder,
    Display, Frame,
};
use std::{cell::Ref, rc::Rc};
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

    /* pub fn run(mut self, mut app: impl Runnable<Self> + 'static) -> ! {
        log::debug!("Initialization took: {:?}", self.init_timer.elapsed());

        let mut previous = Instant::now();
        let mut lag = Duration::from_secs_f64(0.0);

        self.facade.gl_window().window().set_visible(true);

        self.event_loop
            .take()
            .unwrap()
            .run(move |event, _, control| {
                *control = if self.stop.load(Ordering::Relaxed) {
                    ControlFlow::Exit
                } else {
                    ControlFlow::Poll
                };

                match &event {
                    Event::WindowEvent {
                        event: WindowEvent::CursorEntered { .. },
                        ..
                    } => self.cursor_in = true,
                    Event::WindowEvent {
                        event: WindowEvent::CursorLeft { .. },
                        ..
                    } => self.cursor_in = false,
                    Event::WindowEvent {
                        event: WindowEvent::CursorMoved { position, .. },
                        ..
                    } => {
                        self.cursor_pos = *position;
                    }
                    Event::WindowEvent {
                        event: WindowEvent::Resized(s),
                        ..
                    } => {
                        self.size = (s.width as f32, s.height as f32);
                        let s = s.to_logical::<f32>(self.scale_factor);
                        self.aspect = s.width / s.height;
                    }
                    Event::RedrawRequested(_) => {
                        // main game loop source:
                        //  - https://gameprogrammingpatterns.com/game-loop.html
                        let elapsed = previous.elapsed();
                        previous = Instant::now();
                        lag += elapsed;

                        // updates
                        while lag >= self.interval {
                            let timer = self.update_reporter.begin();
                            app.update(&self);
                            self.update_reporter.end(timer);
                            lag -= self.interval;
                        }

                        // frames
                        let timer = self.frame_reporter.begin();
                        {
                            let mut frame = self.facade.draw();
                            app.draw(
                                &self,
                                &mut frame,
                                lag.as_secs_f32() / self.interval.as_secs_f32(),
                            );
                            frame.finish().unwrap();
                        }
                        let should_report = self.frame_reporter.end(timer);

                        // reports
                        if should_report {
                            let int = self.frame_reporter.report_interval();
                            let (u_int, u_per_sec) = self.update_reporter.last_string();
                            let (f_int, f_per_sec) = self.frame_reporter.last_string();

                            #[cfg(debug_assertions)]
                            const DEBUG: &str = "debug build";
                            #[cfg(not(debug_assertions))]
                            const DEBUG: &str = "release build";

                            log::debug!(
                                "Report ({:?})({})\n        per second @ time per\nUPDATES: {:>9} @ {}\nFRAMES: {:>10} @ {}",
                                int,
                                DEBUG,
                                u_per_sec,
                                u_int,
                                f_per_sec,
                                f_int
                            );
                        }

                        return;
                    }
                    Event::MainEventsCleared => {
                        self.facade.gl_window().window().request_redraw();
                    }
                    _ => {}
                }

                app.event(&self, &event);
            })
    } */
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

    fn get_window(&self) -> Ref<'_, Window> {
        Ref::map(self.facade.gl_window(), |d| d.window())
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
