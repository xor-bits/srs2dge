use instant::Instant;
use std::sync::Arc;
use winit::window::Window;

use glam::*;
use main_game_loop::prelude::*;
use srs2dge::prelude::*;

//

struct App {
    target: Target,
    timer: Instant,
}

impl App {
    async fn new(target: &EventLoopTarget) -> Self {
        let engine = Engine::new();
        let target = engine
            .new_target(Arc::new(Window::new(target).unwrap()))
            .await;
        Self {
            target,
            timer: Instant::now(),
        }
    }

    fn event(&mut self) {
        let t = self.timer.elapsed().as_secs_f32();
        const PHASE_OFFS: f32 = 2.0 / 3.0 * std::f32::consts::PI;
        let phase_a = t;
        let phase_b = phase_a + PHASE_OFFS;
        let phase_c = phase_b + PHASE_OFFS;
        let a = phase_a.sin() * 0.5 + 0.5;
        let b = phase_b.sin() * 0.5 + 0.5;
        let c = phase_c.sin() * 0.5 + 0.5;
        let c = Vec4::new(a, b, c, 1.0);

        let mut frame = self.target.get_frame();
        frame.set_clear_color(c);
        frame.main_render_pass();
        self.target.finish_frame(frame);
    }
}

async fn run() {
    let target = EventLoop::new();
    let mut app = App::new(&target).await;
    target.run(move |_, _, _| {
        app.event();
    });
}

fn main() {
    init_log();
    as_async(run());
}
