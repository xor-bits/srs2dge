use instant::Instant;
use winit::{dpi::PhysicalPosition, event::WindowEvent};

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
        let target = Engine::new().new_target_default(target).await.unwrap();
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

    async fn event(&mut self, event: Event<'_>, _: &EventLoopTarget, control: &mut ControlFlow) {
        self.ws.event(&event);

        self.target.event(&event);

        if let Event::WindowEvent {
            event: WindowEvent::CloseRequested,
            ..
        } = event
        {
            *control = ControlFlow::Exit;
        }
    }

    async fn draw(&mut self) {
        let t = self.timer.elapsed().as_secs_f32();

        let r_a = Vec2::ONE * 0.5;
        let r_b = Vec2::ONE * 0.33;
        let r_c = Vec2::ONE * 0.3;
        let r_d = Vec2::ONE * 0.1;

        let a = Vec2::new(t.cos(), t.sin()) * r_a;
        let b = Vec2::new((t * 0.4).cos(), (t * 0.4).sin()) * r_b;
        let c = Vec2::new((t * 4.0).cos(), (t * 4.0).sin()) * r_c;
        let d = Vec2::new((t * 0.8).cos(), (t * 0.8).sin()) * r_d;

        self.debug
            .add_text(GizmosText::new(
                Vec2::new(-0.3, -0.9),
                &self.ws,
                "Do not draw text like this",
                Color::RED,
            ))
            .unwrap();

        // lines
        self.debug.add_line(GizmosLine::new(a, b, Color::RED));
        self.debug.add_line(GizmosLine::new(b, c, Color::GREEN));
        self.debug.add_line(GizmosLine::new(c, d, Color::BLUE));

        // rings
        self.debug
            .add_circle(GizmosCircle::new(Vec2::ZERO, r_a, Color::WHITE));
        self.debug
            .add_circle(GizmosCircle::new(Vec2::ZERO, r_b, Color::WHITE));
        self.debug
            .add_circle(GizmosCircle::new(Vec2::ZERO, r_c, Color::WHITE));
        self.debug
            .add_circle(GizmosCircle::new(Vec2::ZERO, r_d, Color::WHITE));

        // cursor follower
        (|| {
            let mvp = self.debug.mvp(&self.ws);

            // position at cursor
            let middle = Gizmos::screen_to_world(mvp, &self.ws, self.ws.cursor_pos)?;

            // crude 10px radius
            let radius = Gizmos::screen_to_world(mvp, &self.ws, PhysicalPosition::new(10, 10))?;
            let radius =
                radius - Gizmos::screen_to_world(mvp, &self.ws, PhysicalPosition::new(0, 0))?;

            self.debug
                .add_circle(GizmosCircle::new(middle, radius, Color::ORANGE));
            self.debug.add_box(GizmosBox::new(
                middle,
                Vec2::ONE * radius * 1.5,
                Color::ROSE,
            ));
            Some(())
        })();

        let mut frame = self.target.get_frame();
        self.debug.prepare(&mut self.target, &mut frame, &self.ws);
        self.debug.draw(frame.primary_render_pass());
    }
}

//

fn main() {
    app!(App);
}
