use legion::system;
use rand::Rng;
use winit::event_loop::ControlFlow;

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

#[cfg_attr(target_arch = "wasm32", system(for_each))]
#[cfg_attr(not(target_arch = "wasm32"), system(par_for_each))]
fn random_movement(body: &mut RigidBody2D) {
    let mut rng = rand::thread_rng();
    if (0.9..=1.0).contains(&rng.gen()) {
        // 10% chance to stop moving
        body.linear_velocity = Vec2::ZERO;
    }
    if (0.0..=0.005).contains(&rng.gen()) {
        // 0.5% chance to start moving
        let movement = Vec2::new(rng.gen_range(-1.0..=1.0), rng.gen_range(-1.0..=1.0));
        body.linear_velocity = movement;
    }
}

//

impl App {
    async fn init(target: &EventLoopTarget) -> Self {
        let engine = Engine::new();
        let target = engine.new_target_default(target).await.unwrap();

        let ws = WindowState::new(&target.get_window().unwrap());

        let frame_report = Reporter::new();

        let texture = Texture::new_rgba_with(
            &target,
            &image::load_from_memory(res::texture::RUST)
                .unwrap()
                .to_rgba8(),
        );

        let ubo = UniformBuffer::new(&target, 1);
        let shader = Texture2DShader::new(&target);

        let mut world = World::new(&target).with_plugin(DefaultPlugins);
        world.insert_update_system(random_movement_system);
        for _ in 0..1_000_000
        /* _000 */
        {
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
}

impl Runnable for App {
    fn event(&mut self, event: Event, _: &EventLoopTarget, control: &mut ControlFlow) {
        self.ws.event(&event);

        if self.ws.should_close {
            *control = ControlFlow::Exit;
        }
    }

    fn draw(&mut self) {
        self.world.run();

        let (ecs_update, ecs_frame) = self.world.reporters();
        if ecs_frame.should_report() {
            log::info!(
                "{}",
                Reporter::report_all(
                    "ECS Perf report",
                    [
                        ("ECS Updates", ecs_update),
                        ("ECS Frames", ecs_frame),
                        ("Frames", &mut self.frame_report)
                    ]
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

        let (vbo, ibo) = self
            .world
            .get_batcher_mut()
            .generate(&mut self.target, &mut frame);

        frame
            .primary_render_pass()
            .bind_vbo(vbo)
            .bind_ibo(ibo)
            .bind_group(&self.shader.bind_group((&self.ubo, &self.texture)))
            .bind_shader(&self.shader)
            .draw_indexed(0..ibo.capacity() as _, 0, 0..1);

        self.target.finish_frame(frame);
        self.frame_report.end(timer);
    }
}

//

main_app!(async App);
