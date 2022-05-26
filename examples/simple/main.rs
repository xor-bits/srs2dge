use instant::Instant;

use srs2dge::prelude::*;

//

struct App {
    target: Target,
    timer: Instant,
}

//

impl App {
    async fn init(target: &EventLoopTarget) -> Self {
        let engine = Engine::new();
        let target = engine.new_target_default(target).await.unwrap();
        let timer = Instant::now();

        Self { target, timer }
    }
}

impl Runnable for App {
    fn event(&mut self, _: Event, _: &EventLoopTarget, _: &mut ControlFlow) {}

    fn draw(&mut self) {
        let t = self.timer.elapsed().as_secs_f32();
        const PHASE_OFFS: f32 = 2.0 / 3.0 * std::f32::consts::PI;
        let phase_a = t;
        let phase_b = phase_a + PHASE_OFFS;
        let phase_c = phase_b + PHASE_OFFS;
        let a = phase_a.sin() * 0.5 + 0.5;
        let b = phase_b.sin() * 0.5 + 0.5;
        let c = phase_c.sin() * 0.5 + 0.5;
        let c = Color::new(a, b, c, 1.0);

        let mut frame = self.target.get_frame();
        frame.set_clear_color(c);
        frame.primary_render_pass();
        self.target.finish_frame(frame);
    }
}

//

main_app!(async App);
