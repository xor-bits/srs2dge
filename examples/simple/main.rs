use srs2dge::prelude::*;

//

struct App {
    target: Target,
}

//

impl App {
    async fn init(target: &EventLoopTarget) -> Self {
        let window = std::sync::Arc::new(WindowBuilder::new().build(target).unwrap());
        let target = Engine::new().new_target(window).await;

        Self { target }
    }

    async fn event(&mut self, e: Event<'_>, _: &EventLoopTarget, c: &mut ControlFlow) {
        self.target.event(&e);

        if let Event::WindowEvent {
            event: WindowEvent::CloseRequested,
            ..
        } = e
        {
            *c = ControlFlow::Exit;
        }
    }

    async fn draw(&mut self) {
        let t = instant::now() as f32 / 1000.0;
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
    }
}

//

fn main() {
    app!(App);
}
