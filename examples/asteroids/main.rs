use asteroid::AsteroidPlugin;
use bullet::BulletPlugin;
use collider::ColliderPlugin;
use mesh::MultiMesh;
use player::{Player, PlayerPlugin};
use winit::dpi::PhysicalPosition;

use srs2dge::prelude::*;

//

mod asteroid;
mod bullet;
mod collider;
mod mesh;
mod player;

//

struct App {
    target: Target,

    ws: WindowState,
    ks: KeyboardState,
    gs: GamepadState,
    old_cursor_pos: PhysicalPosition<f64>,

    batcher: Option<BatchRenderer<MultiMesh, DefaultVertex>>,
    shader: LineShader,
    ubo: UniformBuffer<Mat4>,

    world: World,
}

#[derive(Debug, Clone, Copy)]
struct Settings {
    direction_relative_movement: bool,
    free_movement: bool,
    easy_rotation: bool,
}

//

impl App {
    pub async fn init(target: &EventLoopTarget) -> Self {
        let engine = Engine::new();
        let target = engine.new_target_default(target).await.unwrap();

        let ws = WindowState::new(&target.get_window().unwrap());
        let ks = KeyboardState::new();
        let gs = GamepadState::new();
        let old_cursor_pos = ws.cursor_pos;

        let mut batcher = Some(BatchRenderer::new(&target));
        let shader = LineShader::new(&target, true);
        let ubo = UniformBuffer::new(&target, 1);

        let mut world = World::new()
            .with_plugin(DefaultClientPlugins(&target))
            .with_plugin(PlayerPlugin)
            .with_plugin(AsteroidPlugin)
            .with_plugin(ColliderPlugin)
            .with_plugin(BulletPlugin);
        world.push((
            Transform2D {
                scale: Vec2::ONE * 0.05,
                ..Transform2D::default()
            },
            RigidBody2D::default(),
            Player {
                idx: batcher
                    .as_mut()
                    .unwrap()
                    .push_with(MultiMesh::Player(Default::default())),
            },
        ));

        /* let settings = Settings {
            direction_relative_movement: true,
            free_movement: false,
            easy_rotation: false,
        }; */
        let settings = Settings {
            direction_relative_movement: false,
            free_movement: true,
            easy_rotation: true,
        };

        world.resources.insert(settings);
        world.resources.insert(settings);

        Self {
            target,

            ws,
            ks,
            gs,
            old_cursor_pos,

            batcher,
            shader,
            ubo,

            world,
        }
    }
}

impl Runnable for App {
    fn event(&mut self, event: Event, _: &EventLoopTarget, control: &mut ControlFlow) {
        self.ws.event(&event);
        self.ks.event(&event);
        self.gs.event(&event);

        if self.ws.should_close {
            *control = ControlFlow::Exit;
        }
    }

    fn draw(&mut self) {
        let mvp = Mat4::orthographic_rh(-self.ws.aspect, self.ws.aspect, -1.0, 1.0, -1.0, 1.0);

        // update
        let cursor = if self.ws.cursor_pos != self.old_cursor_pos {
            self.old_cursor_pos = self.ws.cursor_pos;
            Gizmos::screen_to_world(mvp, &self.ws, self.ws.cursor_pos)
        } else {
            None
        };
        self.world.resources.insert(cursor);
        self.world.resources.insert(self.ks.clone());
        self.world.resources.insert(self.gs.clone());
        self.world.resources.insert(self.batcher.take().unwrap());
        if self.world.run() {
            self.ks.clear();
            self.gs.clear();
        }
        self.batcher = Some(self.world.resources.remove().unwrap());

        // draw
        let mut frame = self.target.get_frame();
        frame.set_clear_color(Color::BLACK);

        self.ubo.upload(&mut self.target, &mut frame, &[mvp]);

        let (vbo, ibo, i) = self
            .batcher
            .as_mut()
            .unwrap()
            .generate(&mut self.target, &mut frame);

        frame
            .primary_render_pass()
            .bind_vbo(vbo)
            .bind_ibo(ibo)
            .bind_group(&self.shader.bind_group(&self.ubo))
            .bind_shader(&self.shader)
            .draw_indexed(0..i, 0, 0..1);
        self.target.finish_frame(frame);
    }
}

//

main_app!(async App);
