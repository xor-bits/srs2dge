use main_game_loop::{
    as_async,
    event::{EventLoop, EventLoopTarget},
    init_log,
};
use srs2dge::{target::Target, Engine};
use std::sync::Arc;
use winit::window::Window;

//

struct App {
    target: Target,
}

impl App {
    async fn new(target: &EventLoopTarget) -> Self {
        let mut engine = Engine::new();
        let target = engine
            .new_target(Arc::new(Window::new(target).unwrap()))
            .await;
        Self { target }
    }

    fn event(&mut self) {
        let mut frame = self.target.get_frame();
        let _ = frame.main_render_pass();
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
