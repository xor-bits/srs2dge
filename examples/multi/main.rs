use instant::Instant;
use srs2dge::winit::event::WindowEvent;

use srs2dge::prelude::*;

//

struct App {
    targets: [Target; 2],
    timer: Instant,
}

//

impl App {
    async fn init(target: &EventLoopTarget) -> Self {
        let engine = Engine::new();
        let targets = [
            engine.new_target_default(target).await.unwrap(),
            engine.new_target_default(target).await.unwrap(),
        ];
        let timer = Instant::now();

        if targets[0].compatible_with(&targets[1]) {
            tracing::info!("Targets are compatible");
        } else {
            tracing::info!("Targets are NOT compatible");
        }

        Self { targets, timer }
    }

    async fn event(&mut self, e: Event<'_>, _: &EventLoopTarget, c: &mut ControlFlow) {
        self.targets.iter_mut().for_each(|t| t.event(&e));

        if let Event::WindowEvent {
            event: WindowEvent::CloseRequested,
            ..
        } = e
        {
            *c = ControlFlow::Exit;
        }
    }

    async fn draw(&mut self) {
        let t = self.timer.elapsed().as_secs_f32();
        const PHASE_OFFS: f32 = 2.0 / 3.0 * std::f32::consts::PI;
        let phase_a = t;
        let phase_b = phase_a + PHASE_OFFS;
        let phase_c = phase_b + PHASE_OFFS;
        let a = phase_a.sin() * 0.5 + 0.5;
        let b = phase_b.sin() * 0.5 + 0.5;
        let c = phase_c.sin() * 0.5 + 0.5;
        let c = Color::new(a, b, c, 1.0);

        for (i, target) in self.targets.iter_mut().enumerate() {
            let mut frame = target.get_frame();
            frame.set_clear_color(c / (i + 1) as f32);
            frame.primary_render_pass();
        }
    }
}

//

fn main() {
    app!(App);
}
