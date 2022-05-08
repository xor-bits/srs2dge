use std::sync::Arc;
use winit::{event_loop::ControlFlow, window::Window};

use glam::*;
use main_game_loop::prelude::*;
use srs2dge::prelude::*;

//

struct App {
    target: Target,
    left: Idx,
    right: Idx,
    left_a: Idx,
    left_b: Idx,
    left_c: Idx,
    left_d: Idx,
    right_a: Idx,
    right_b: Idx,
    right_c: Idx,
    right_d: Idx,
    batcher: BatchRenderer,

    shader: Colored2DShader,
    ubo: UniformBuffer<Mat4>,

    input: InputState,
    window: WindowState,
}

//

impl App {
    async fn init(target: &EventLoopTarget) -> Self {
        let engine = Engine::new();
        let window = Arc::new(Window::new(target).unwrap());
        let target = engine.new_target(window.clone()).await;

        let mut batcher = BatchRenderer::new(&target);
        let left = batcher.push_with(QuadMesh {
            pos: Vec2::new(-0.5, 0.0),
            size: Vec2::new(0.1, 0.1),
            col: Vec4::new(1.0, 1.0, 1.0, 1.0),
            tex: Default::default(),
        });
        let right = batcher.push_with(QuadMesh {
            pos: Vec2::new(0.5, 0.0),
            size: Vec2::new(0.1, 0.1),
            col: Vec4::new(1.0, 1.0, 1.0, 1.0),
            tex: Default::default(),
        });

        let left_a = batcher.push_with(QuadMesh {
            pos: Vec2::new(-0.75, 0.0),
            size: Vec2::new(0.05, 0.05),
            col: Vec4::new(1.0, 1.0, 1.0, 1.0),
            tex: Default::default(),
        });
        let left_b = batcher.push_with(QuadMesh {
            pos: Vec2::new(-0.75, 0.1),
            size: Vec2::new(0.05, 0.05),
            col: Vec4::new(1.0, 1.0, 1.0, 1.0),
            tex: Default::default(),
        });
        let left_c = batcher.push_with(QuadMesh {
            pos: Vec2::new(-0.7, 0.05),
            size: Vec2::new(0.05, 0.05),
            col: Vec4::new(1.0, 1.0, 1.0, 1.0),
            tex: Default::default(),
        });
        let left_d = batcher.push_with(QuadMesh {
            pos: Vec2::new(-0.8, 0.05),
            size: Vec2::new(0.05, 0.05),
            col: Vec4::new(1.0, 1.0, 1.0, 1.0),
            tex: Default::default(),
        });

        let right_a = batcher.push_with(QuadMesh {
            pos: Vec2::new(0.75, 0.0),
            size: Vec2::new(0.05, 0.05),
            col: Vec4::new(1.0, 1.0, 1.0, 1.0),
            tex: Default::default(),
        });
        let right_b = batcher.push_with(QuadMesh {
            pos: Vec2::new(0.75, 0.1),
            size: Vec2::new(0.05, 0.05),
            col: Vec4::new(1.0, 1.0, 1.0, 1.0),
            tex: Default::default(),
        });
        let right_c = batcher.push_with(QuadMesh {
            pos: Vec2::new(0.7, 0.05),
            size: Vec2::new(0.05, 0.05),
            col: Vec4::new(1.0, 1.0, 1.0, 1.0),
            tex: Default::default(),
        });
        let right_d = batcher.push_with(QuadMesh {
            pos: Vec2::new(0.8, 0.05),
            size: Vec2::new(0.05, 0.05),
            col: Vec4::new(1.0, 1.0, 1.0, 1.0),
            tex: Default::default(),
        });

        let shader = Colored2DShader::new(&target);

        let input = InputState::new();
        let window = WindowState::new(&window);

        let ubo = UniformBuffer::new_single(
            &target,
            Mat4::orthographic_rh(-window.aspect, window.aspect, -1.0, 1.0, -100.0, 100.0),
        );

        Self {
            target,

            left,
            right,
            left_a,
            left_b,
            left_c,
            left_d,
            right_a,
            right_b,
            right_c,
            right_d,
            batcher,

            shader,
            ubo,

            input,
            window,
        }
    }

    fn event(&mut self, event: Event, control: &mut ControlFlow) {
        *control = ControlFlow::Poll;
        self.input.event(&event);
        self.window.event(&event);

        if self.window.should_close {
            *control = ControlFlow::Exit;
            return;
        }

        let color = |active: bool| -> Vec4 {
            if active {
                Vec4::new(1.0, 0.0, 0.0, 1.0)
            } else {
                Vec4::new(0.0, 0.0, 1.0, 1.0)
            }
        };

        let move_axis = self.input.get_axis(InputAxis::Move, 0);
        let left = self.batcher.get_mut(self.left);
        left.pos = move_axis / 4.0 + Vec2::new(-0.5, -0.5);
        left.col = color(move_axis.length_squared() <= 0.1_f32.powi(2));

        let look_axis = self.input.get_axis(InputAxis::Look, 0);
        let right = self.batcher.get_mut(self.right);
        right.pos = look_axis / 4.0 + Vec2::new(0.5, -0.5);
        right.col = color(look_axis.length_squared() <= 0.1_f32.powi(2));

        self.batcher.get_mut(self.left_a).col =
            color(self.input.get_input(Input::RollDown, 0).triggered());
        self.batcher.get_mut(self.left_b).col =
            color(self.input.get_input(Input::RollUp, 0).triggered());
        self.batcher.get_mut(self.left_c).col =
            color(self.input.get_input(Input::RollRight, 0).triggered());
        self.batcher.get_mut(self.left_d).col =
            color(self.input.get_input(Input::RollLeft, 0).triggered());

        self.batcher.get_mut(self.right_a).col =
            color(self.input.get_input(Input::Jump, 0).triggered());
        self.batcher.get_mut(self.right_b).col =
            color(self.input.get_input(Input::Inventory, 0).triggered());
        self.batcher.get_mut(self.right_c).col =
            color(self.input.get_input(Input::Reload, 0).triggered());
        self.batcher.get_mut(self.right_d).col =
            color(self.input.get_input(Input::Crouch, 0).triggered());

        if let Event::MainEventsCleared = event {
            self.draw();
        }
    }

    fn draw(&mut self) {
        let mut frame = self.target.get_frame();

        let (vbo, ibo) = self.batcher.generate(&mut self.target, &mut frame);

        frame
            .main_render_pass()
            .bind_vbo(vbo)
            .bind_ibo(ibo)
            .bind_group(&self.shader.bind_group(&self.ubo))
            .bind_shader(&self.shader)
            .draw_indexed(0..ibo.capacity() as _, 0, 0..1);

        self.target.finish_frame(frame);
    }
}

//

async fn run() {
    let target = EventLoop::new();
    let mut app = App::init(&target).await;
    target.run(move |e, _, c| {
        app.event(e, c);
    });
}

fn main() {
    init_log();
    as_async(run());
}
