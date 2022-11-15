use srs2dge::prelude::*;

//

struct App {
    target: Target,

    ws: WindowState,

    frame_report: Reporter,

    texture: Texture,

    ubo: UniformBuffer<Mat4>,
    shader: Texture2DShader,

    world: World,
}

#[cfg_attr(
    any(target_arch = "wasm32", not(feature = "parallel-ecs")),
    legion::system(for_each)
)]
#[cfg_attr(
    not(any(target_arch = "wasm32", not(feature = "parallel-ecs"))),
    legion::system(par_for_each)
)]
fn random_movement(body: &mut RigidBody2D) {
    if (0.9..=1.0).contains(&fastrand::f32()) {
        // 10% chance to stop moving
        body.linear_velocity = Vec2::ZERO;
    }
    if (0.0..=0.005).contains(&fastrand::f32()) {
        // 0.5% chance to start moving
        let movement = Vec2::new(fastrand::f32() * 2.0 - 1.0, fastrand::f32() * 2.0 - 1.0);
        body.linear_velocity = movement;
    }
}

//

impl App {
    async fn init(target: &EventLoopTarget) -> Self {
        let target = Engine::new().new_target_default(target).await.unwrap();

        let ws = WindowState::new(&target.get_window().unwrap());

        let frame_report = Reporter::new();

        let texture = Texture::new_rgba_with(
            &target,
            &image::load_from_memory(res::texture::RUST)
                .unwrap()
                .to_rgba8(),
            None,
        );

        let ubo = UniformBuffer::new(&target, 1);
        let shader = Texture2DShader::new(&target);

        let mut world = World::new().with_plugin(DefaultClientPlugins(&target));
        world.updates.insert(random_movement_system);
        for _ in 0..100_000 {
            world.push((
                RigidBody2D::default(),
                Transform2D {
                    scale: Vec2::new(0.05, 0.05),
                    ..Default::default()
                },
                Sprite {
                    sprite: Default::default(),
                    color: Color::WHITE,
                    ..Default::default()
                },
            ));
        }

        Self {
            target,

            ws,

            frame_report,

            texture,

            ubo,
            shader,
            world,
        }
    }

    async fn event(&mut self, event: Event<'_>, _: &EventLoopTarget, control: &mut ControlFlow) {
        self.ws.event(&event);

        self.target.event(&event);

        if self.ws.should_close {
            *control = ControlFlow::Exit;
        }
    }

    async fn draw(&mut self) {
        self.world.run();

        if self.frame_report.should_report() {
            tracing::info!(
                "{}",
                Reporter::report_all(
                    "ECS Perf report",
                    self.world
                        .reporters()
                        .chain([("Frames", &mut self.frame_report)])
                )
            )
        }

        let timer = self.frame_report.begin();
        let mut frame = self.target.get_frame();

        self.ubo.upload(
            &mut self.target,
            &mut frame,
            &[Mat4::orthographic_lh(
                -2.0 * self.ws.aspect,
                2.0 * self.ws.aspect,
                -2.0,
                2.0,
                -100.0,
                100.0,
            )],
        );

        let mut batcher = self.world.get_batcher_mut();
        let (vbo, ibo, i) = batcher.generate(&mut self.target, &mut frame);

        frame
            .primary_render_pass()
            .bind_vbo(vbo)
            .bind_ibo(ibo)
            .bind_group(&self.shader.bind_group((&self.ubo, &self.texture)))
            .bind_shader(&self.shader)
            .draw_indexed(0..i, 0, 0..1);

        self.frame_report.end(timer);
    }
}

//

fn main() {
    app!(App);
}
