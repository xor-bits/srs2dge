use instant::Instant;
use winit::{dpi::PhysicalPosition, event::WindowEvent, event_loop::ControlFlow};

use srs2dge::prelude::*;

//

struct App {
    target: Target,
    ws: WindowState,
    timer: Instant,
    debug: Gizmos,
}

impl App {
    async fn init(target: &EventLoopTarget) -> Self {
        let engine = Engine::new();
        let target = engine.new_target_default(target).await.unwrap();
        let ws = WindowState::new(&target.get_window().unwrap());
        let timer = Instant::now();
        let mut debug = Gizmos::new(&target);
        debug.set_font_bytes(res::font::FIRA).unwrap();

        Self {
            target,
            ws,
            timer,
            debug,
        }
    }
}

impl Runnable for App {
    fn event(&mut self, event: Event, _: &EventLoopTarget, control: &mut ControlFlow) {
        self.ws.event(&event);

        if let Event::WindowEvent {
            event: WindowEvent::CloseRequested,
            ..
        } = event
        {
            *control = ControlFlow::Exit;
        }
    }

    fn draw(&mut self) {
        let t = self.timer.elapsed().as_secs_f32();

        let r_a = 0.5;
        let r_b = 0.33;
        let r_c = 0.3;
        let r_d = 0.1;

        let a = Vec2::new(t.cos(), t.sin()) * r_a;
        let b = Vec2::new((t * 0.4).cos(), (t * 0.4).sin()) * r_b;
        let c = Vec2::new((t * 4.0).cos(), (t * 4.0).sin()) * r_c;
        let d = Vec2::new((t * 0.8).cos(), (t * 0.8).sin()) * r_d;

        self.debug
            .add_text(GizmosText::new(
                Vec2::new(-0.9, -0.9),
                &self.ws,
                "Do not draw text like this",
                Vec4::X + Vec4::W,
            ))
            .unwrap();

        // lines
        self.debug
            .add_line(GizmosLine::new(a, b, Vec4::X + Vec4::W));
        self.debug
            .add_line(GizmosLine::new(b, c, Vec4::Y + Vec4::W));
        self.debug
            .add_line(GizmosLine::new(c, d, Vec4::Z + Vec4::W));

        // rings
        self.debug
            .add_circle(GizmosCircle::new(Vec2::ZERO, r_a, Vec4::ONE));
        self.debug
            .add_circle(GizmosCircle::new(Vec2::ZERO, r_b, Vec4::ONE));
        self.debug
            .add_circle(GizmosCircle::new(Vec2::ZERO, r_c, Vec4::ONE));
        self.debug
            .add_circle(GizmosCircle::new(Vec2::ZERO, r_d, Vec4::ONE));

        // cursor follower
        (|| {
            // position at cursor
            let middle = self.debug.screen_to_world(&self.ws, self.ws.cursor_pos)?;

            // crude 10px radius
            let radius = self
                .debug
                .screen_to_world(&self.ws, PhysicalPosition::new(10, 0))?
                .x;
            let radius = radius
                - self
                    .debug
                    .screen_to_world(&self.ws, PhysicalPosition::new(0, 0))?
                    .x;

            self.debug.add_circle(GizmosCircle::new(
                middle,
                radius,
                Vec4::new(1.0, 0.4, 0.0, 1.0),
            ));
            self.debug.add_box(GizmosBox::new(
                middle,
                Vec2::new(radius * 1.5, radius * 1.5),
                Vec4::new(1.0, 0.0, 0.4, 1.0),
            ));
            Some(())
        })();

        let mut frame = self.target.get_frame();
        self.debug.prepare(&mut self.target, &mut frame, &self.ws);
        self.debug.draw(frame.primary_render_pass());
        self.target.finish_frame(frame);
    }
}

//

main_app!(async App);
