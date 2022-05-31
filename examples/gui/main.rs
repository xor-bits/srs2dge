use srs2dge::prelude::*;

//

struct App {
    target: Target,

    gui: Gui,
}

//

impl App {
    async fn init(target: &EventLoopTarget) -> Self {
        let engine = Engine::new();
        let target = engine.new_target_default(target).await.unwrap();

        let gui = Gui::new(&target);

        Self { target, gui }
    }
}

impl Runnable for App {
    fn event(&mut self, event: Event, _: &EventLoopTarget, control: &mut ControlFlow) {
        self.gui.event(&event);

        if self.gui.window_state().should_close {
            *control = ControlFlow::Exit;
        }
    }

    fn draw(&mut self) {
        let mut frame = self.target.get_frame();

        let root = self.gui.root();
        const BORDER: f32 = 8.0;
        let fill = Fill::new(
            &root,
            |base| (base.size - BORDER * Vec2::ONE * 2.0).max(Vec2::ZERO),
            |base, _| base.offset + BORDER * Vec2::ONE,
            Color::AZURE,
            &mut self.gui,
        );
        Fill::new(
            &fill,
            |base| (base.size * 0.5).min(Vec2::ONE * 200.0), // half the size of `fill` and 200x200 px at max
            |base, size| align(base, size, Vec2::new(0.5, 1.0)), // x center, y top
            Color::CHARTREUSE,
            &mut self.gui,
        );

        let gui = self.gui.generate(&mut self.target, &mut frame);
        frame.primary_render_pass().draw_gui(gui);

        self.target.finish_frame(frame);
    }
}

//

main_app!(async App);
