use rand::Rng;
use specs::{
    prelude::ParallelIterator, Builder, Component, DenseVecStorage, DispatcherBuilder, Join,
    ParJoin, System, World, WorldExt, Write, WriteStorage,
};
use std::sync::Arc;
use winit::event_loop::ControlFlow;

use srs2dge::prelude::*;

//

struct App {
    target: Target,

    ws: WindowState,

    texture: Texture,

    ubo: UniformBuffer<Mat4>,
    shader: Texture2DShader,

    world: Arc<World>,
}

#[derive(Debug, Clone, Copy, Default, Component)]
struct RigidBody2DComponent {
    velocity: Vec2,
    acceleration: Vec2,
}

struct RigidBody2DSystem {}

struct RandomMovementSystem;
impl<'a> System<'a> for RandomMovementSystem {
    type SystemData = WriteStorage<'a, Transform2DComponent>;

    fn run(&mut self, mut transform: Self::SystemData) {
        (&mut transform).par_join().for_each(|transform| {
            let mut rng = rand::thread_rng();
            let f: f32 = rng.gen();
            if (0.9..=0.95).contains(&f) {
                // 5% chance
                let movement = Vec2::new(rng.gen_range(-0.1..0.1), rng.gen_range(-0.1..0.1));
                transform.translation += movement;
            }
            if (0.95..=1.0).contains(&f) {
                // 5% chance
                let movement = rng.gen_range(-0.1..0.1);
                transform.rotation += movement; // TODO:
            }
        });
    }
}

//

impl App {
    async fn init(target: &EventLoopTarget) -> Self {
        let engine = Engine::new();
        let target = engine.new_target_default(target).await.unwrap();

        let ws = WindowState::new(&target.get_window().unwrap());

        let texture = Texture::new_rgba_with(
            &target,
            &image::load_from_memory(res::texture::RUST)
                .unwrap()
                .to_rgba8(),
        );

        let batcher = BatchRenderer::<QuadMesh, DefaultVertex>::new(&target);
        let ubo = UniformBuffer::new(&target, 1);
        let shader = Texture2DShader::new(&target);

        let mut world = World::new();
        world.register::<Transform2DComponent>();
        world.register::<SpriteComponent>();
        world.insert(Some(batcher));

        let mut rng = rand::thread_rng();
        for _ in 0..100 {
            world
                .create_entity()
                .with(RigidBody2DComponent::default())
                .with(Transform2DComponent {
                    translation: Vec2::new(rng.gen_range(-0.8..0.8), rng.gen_range(-0.8..0.8)),
                    scale: Vec2::new(0.1, 0.1),
                    ..Default::default()
                })
                .with(SpriteComponent {
                    sprite: Default::default(),
                    color: Vec4::ONE,
                    idx: None,
                })
                .build();
        }

        let world = Arc::new(world);

        Self {
            target,

            ws,

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
        let mut dispatcher = DispatcherBuilder::new()
            .with(RandomMovementSystem, "RandomMovementSystem", &[])
            .with_barrier()
            .with(SpriteToQuadSystem, "SpriteToQuadSystem", &[])
            .build_async(self.world.clone());
        dispatcher.dispatch();

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

        dispatcher.wait();
        let mut batcher = self.world.write_resource::<Option<BatchRenderer>>();
        let (vbo, ibo) = batcher
            .as_mut()
            .unwrap()
            .generate(&mut self.target, &mut frame);

        frame
            .primary_render_pass()
            .bind_vbo(vbo)
            .bind_ibo(ibo)
            .bind_group(&self.shader.bind_group((&self.ubo, &self.texture)))
            .bind_shader(&self.shader)
            .draw_indexed(0..ibo.capacity() as _, 0, 0..1);

        self.target.finish_frame(frame);
    }
}

//

main_app!(async App);
