use winit::event_loop::ControlFlow;

use srs2dge::prelude::*;

//

struct App {
    target: Target,

    ws: WindowState,

    texture: Texture,

    vbo: VertexBuffer,
    ibo: IndexBuffer,
    ubo: UniformBuffer<Mat4>,
    shader: Texture2DShader,
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

        let quad = QuadMesh {
            pos: Vec2::new(-1.0, -1.0),
            size: Vec2::new(2.0, 2.0),
            col: Vec4::new(1.0, 1.0, 1.0, 1.0),
            tex: TexturePosition::default(),
        };
        let vbo = VertexBuffer::new_with(&target, &quad.vertices().collect::<Box<_>>());
        let ibo = IndexBuffer::new_with(&target, &quad.indices(0).collect::<Box<_>>());
        let ubo = UniformBuffer::new(&target, 1);
        let shader = Texture2DShader::new(&target);

        Self {
            target,

            ws,

            texture,

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
            &[Mat4::orthographic_lh(
                -2.0 * self.ws.aspect,
                2.0 * self.ws.aspect,
                -2.0,
                2.0,
                -100.0,
                100.0,
            )],
        );

        frame
            .primary_render_pass()
            .bind_vbo(&self.vbo)
            .bind_ibo(&self.ibo)
            .bind_group(&self.shader.bind_group((&self.ubo, &self.texture)))
            .bind_shader(&self.shader)
            .draw_indexed(0..4, 0, 0..1);

        self.target.finish_frame(frame);
    }
}

//

main_app!(async App);
