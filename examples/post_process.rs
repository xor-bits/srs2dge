use glam::*;
use main_game_loop::prelude::*;
use srs2dge::prelude::*;
use winit::event_loop::ControlFlow;

//

const POST_PROCESSOR: &str = include_str!("post_process.wgsl");

//

struct App {
    target: Target,
    secondary_target: RenderTargetTexture,

    ws: WindowState,

    // logo
    texture: Texture,

    // logo quad and screen quad
    vbo: VertexBuffer,
    ibo: IndexBuffer,

    // drawing to texture
    ubo: UniformBuffer<Mat4>,
    shader: Texture2DShader,

    // drawing to screen
    identity_ubo: UniformBuffer<Mat4>,
    custom_shader: Texture2DShader,
}

//

impl App {
    async fn init(target: &EventLoopTarget) -> Self {
        let engine = Engine::new();
        let target = engine.new_target_default(target).await.unwrap();

        let ws = WindowState::new(&target.get_window().unwrap());
        let secondary_target = RenderTargetTexture::new_format(
            &target,
            Rect::new(ws.size.width, ws.size.height),
            target.get_format(), // secondary target format set to same as primary target format to be able to use a single pipeline
        );

        let texture = Texture::new_rgba_with(
            &target,
            &image::load_from_memory(include_bytes!("../res/texture/rust.png"))
                .unwrap()
                .to_rgba8(),
        );

        let quad_a = QuadMesh {
            pos: Vec2::new(-0.5, -0.5), // bottom left
            size: Vec2::new(1.0, 1.0),
            col: Vec4::new(1.0, 1.0, 1.0, 1.0),
            tex: TexturePosition::default(),
        };
        let quad_b = QuadMesh {
            pos: Vec2::new(0.0, 0.0),
            size: Vec2::new(2.0, 2.0),
            col: Vec4::new(1.0, 1.0, 1.0, 1.0),
            tex: TexturePosition::default(),
        };
        let vbo = VertexBuffer::new_with(
            &target,
            &[quad_a, quad_b]
                .into_iter()
                .flat_map(|q| q.vertices())
                .collect::<Box<_>>(),
        );
        let ibo = IndexBuffer::new_with(
            &target,
            &[quad_a, quad_b]
                .into_iter()
                .enumerate()
                .flat_map(|(i, q)| q.indices(i as _))
                .collect::<Box<_>>(),
        );
        let ubo = UniformBuffer::new(&target, 1);
        let shader = Texture2DShader::new(&target);

        let identity_ubo = UniformBuffer::new_single(&target, Mat4::IDENTITY);
        let custom_frag = ShaderModule::new_wgsl_source(&target, POST_PROCESSOR.into())
            .expect("Custom module compilation failed");
        let custom_shader = Texture2DShader::new_custom_frag(&target, &custom_frag, "main")
            .expect("Custom module incompatible");

        Self {
            target,
            secondary_target,

            ws,

            texture,

            vbo,
            ibo,
            ubo,
            shader,

            identity_ubo,
            custom_shader,
        }
    }
}

impl Runnable for App {
    fn event(&mut self, event: Event, _: &EventLoopTarget, control: &mut ControlFlow) {
        let old = self.ws.size;
        self.ws.event(&event);
        let changed = self.ws.size != old;

        if self.ws.should_close {
            *control = ControlFlow::Exit;
        }

        if changed {
            self.secondary_target = RenderTargetTexture::new_format(
                &self.target,
                self.ws.size.into(),
                self.target.get_format(),
            );
        }
    }

    fn draw(&mut self) {
        let mut frame = self.target.get_frame();

        self.ubo.upload(
            &mut self.target,
            &mut frame,
            &[Mat4::orthographic_rh(
                -self.ws.aspect,
                self.ws.aspect,
                -1.0,
                1.0,
                -100.0,
                100.0,
            )],
        );

        frame
            .secondary_render_pass(&self.secondary_target)
            .unwrap()
            .bind_vbo(&self.vbo)
            .bind_ibo(&self.ibo)
            .bind_group(&self.shader.bind_group((&self.ubo, &self.texture)))
            .bind_shader(&self.shader)
            .draw_indexed(0..4, 0, 0..1);

        frame
            .primary_render_pass()
            .bind_vbo(&self.vbo)
            .bind_ibo(&self.ibo)
            .bind_group(
                &self
                    .shader
                    .bind_group((&self.identity_ubo, &self.secondary_target)),
            )
            .bind_shader(&self.custom_shader)
            .draw_indexed(5..9, 0, 0..1);

        self.target.finish_frame(frame);
    }
}

//

main_app!(async App);
