use std::sync::Arc;
use winit::{dpi::PhysicalSize, event_loop::ControlFlow, window::WindowBuilder};

use glam::*;
use main_game_loop::prelude::*;
use srs2dge::prelude::*;

//

struct App {
    // this is the main render
    // target, the main window
    target: Target,

    // this is used to keep track
    // of the window size, aspect
    // ratio and etc.
    ws: WindowState,

    // glyphs are allocated here
    glyphs: Glyphs,

    // triangle points generated
    // by `text`
    vbo: VertexBuffer,
    // indices to earlier `vbo`
    // to generate those triangles
    ibo: IndexBuffer,
    // used to send a model
    // view projection matrix
    // MVP matrix to the shader
    //
    // can send any type that
    // implements `bytemuck::Pod`
    ubo: UniformBuffer<Mat4>,
    // GPU program used to draw
    //
    // this is a preset that only
    // uses the texture for
    // color output alpha values
    shader: TextShader,
}

//

impl App {
    async fn init(target: &EventLoopTarget) -> Self {
        let engine = Engine::new();
        let target = engine
            .new_target(Arc::new(
                WindowBuilder::new()
                    .with_inner_size(PhysicalSize::new(570, 210))
                    .build(target)
                    .unwrap(),
            ))
            .await;

        let ws = WindowState::new(&target.get_window().unwrap());

        let mut glyphs = Glyphs::new(&target, Rect::new(256, 256), None);

        let (v, i) = text(
            &target,
            &FString::from_iter([
                "red".default().color(1.0, 0.0, 0.0),
                "green".default().color(0.0, 1.0, 0.0),
                "blue".default().color(0.0, 0.0, 1.0),
            ]),
            &mut glyphs,
            80.0,
            50.0,
            50.0,
        )
        .unwrap();
        let vbo = VertexBuffer::new_with(&target, &v);
        let ibo = IndexBuffer::new_with(&target, &i);
        let ubo = UniformBuffer::new(&target, 1);
        let shader = TextShader::new(&target);

        Self {
            target,

            ws,

            glyphs,

            vbo,
            ibo,
            ubo,
            shader,
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
        let mut frame = self.target.get_frame();

        self.ubo.upload(
            &mut self.target,
            &mut frame,
            &[Mat4::orthographic_rh(
                0.0,
                self.ws.size.width as _,
                self.ws.size.height as _,
                0.0,
                -100.0,
                100.0,
            )],
        );

        frame
            .main_render_pass()
            // .bind_vbo(&self.vbo)
            // .bind_ibo(&self.ibo)
            // .bind_group(&self.text_shader.bind_group((&self.ubo, &self.glyphs)))
            // .bind_shader(&self.text_shader)
            // .draw_indexed(0..self.ibo.capacity() as _, 0, 0..1)
            .bind_vbo(&self.vbo)
            .bind_ibo(&self.ibo)
            .bind_group(&self.shader.bind_group((&self.ubo, &self.glyphs)))
            .bind_shader(&self.shader)
            .draw_indexed(0..self.ibo.capacity() as _, 0, 0..1);

        self.target.finish_frame(frame);
    }
}

//

main_app!(async App);
