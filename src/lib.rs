#![feature(drain_filter)]
#![feature(type_alias_impl_trait)]

use glium::{backend::Facade, glutin::ContextBuilder, Display};
use report::Reporter;
use runnable::Runnable;
use std::{
    sync::atomic::{AtomicBool, Ordering},
    time::{Duration, Instant},
};
use winit::{
    dpi::{LogicalSize, PhysicalPosition},
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

#[macro_use]
pub extern crate glium;
pub extern crate glam;
pub extern crate winit;

pub mod batch;
pub mod packer;
pub mod program;
pub mod report;
pub mod runnable;
pub mod text;

pub struct Engine {
    pub facade: Display,

    event_loop: Option<EventLoop<()>>,
    stop: AtomicBool,
    init_timer: Instant,

    //
    pub frame_reporter: Reporter,
    pub update_reporter: Reporter,

    // window size
    pub size: (f32, f32),

    // window aspect ratio
    pub aspect: f32,

    // is cursor inside the window?
    pub cursor_in: bool,

    // cursor position
    pub cursor_pos: PhysicalPosition<f64>,

    // window scaling factor
    pub scale_factor: f64,

    // update interval
    pub interval: Duration,
}

impl Engine {
    pub fn init() -> Self {
        let init_timer = Instant::now();

        let event_loop = EventLoop::new();
        let window_builder = WindowBuilder::new()
            .with_inner_size(LogicalSize::new(600_i32, 600_i32))
            .with_visible(false)
            .with_title("Title");
        let context = ContextBuilder::new()
            // .with_vsync(true)
            .build_windowed(window_builder, &event_loop)
            .unwrap();
        let scale_factor = context.window().scale_factor();
        let facade = Display::from_gl_window(context).unwrap();

        log::debug!("OpenGL Vendor: {}", facade.get_opengl_vendor_string());
        log::debug!("OpenGL Renderer: {}", facade.get_opengl_renderer_string());
        log::debug!("OpenGL Version: {}", facade.get_opengl_version_string());

        let frame_reporter = Reporter::new_with_interval(Duration::from_secs_f32(5.0));
        let update_reporter = Reporter::new_with_interval(Duration::from_secs_f32(5.0));

        let size = facade
            .gl_window()
            .window()
            .inner_size()
            .to_logical(scale_factor);
        let size = (size.width, size.height);
        let aspect = size.0 / size.1;
        let interval = Duration::from_secs_f64(1.0 / 60.0);
        let stop = AtomicBool::new(false);
        let event_loop = Some(event_loop);
        let cursor_in = false;
        let cursor_pos = Default::default();

        Self {
            facade,
            event_loop,
            frame_reporter,
            update_reporter,
            stop,
            init_timer,

            size,
            aspect,
            cursor_in,
            cursor_pos,
            scale_factor,
            interval,
        }
    }

    pub fn stop(&self) {
        self.stop.store(true, Ordering::Relaxed)
    }

    pub fn run(mut self, mut app: impl Runnable + 'static) -> ! {
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
    }
}

impl Facade for Engine {
    fn get_context(&self) -> &std::rc::Rc<glium::backend::Context> {
        self.facade.get_context()
    }
}
