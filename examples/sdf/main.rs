use srs2dge::{image, winit::dpi::PhysicalSize};
use std::sync::Arc;

use srs2dge::prelude::*;

//

struct App {
    target: Target,

    ws: WindowState,

    glyphs: Glyphs,
    texture: Texture,

    rust_vbo: VertexBuffer,
    rust_ibo: IndexBuffer,
    text_vbo: VertexBuffer,
    text_ibo: IndexBuffer,
    ubo: UniformBuffer<Mat4>,
    sdf_shader: SdfShader,
}

//

impl App {
    async fn init(target: &EventLoopTarget) -> Self {
        let engine = Engine::new();
        let target = engine
            .new_target(Arc::new(
                WindowBuilder::new()
                    .with_visible(false)
                    .with_inner_size(PhysicalSize::new(560, 350))
                    .build(target)
                    .unwrap(),
            ))
            .await;

        let ws = WindowState::new(&target.get_window().unwrap());

        let mut glyphs =
            Glyphs::new_bytes(&target, Rect::new(128, 128), Some(48), res::font::ROBOTO).unwrap();
        let texture = Texture::new_grey_with(
            &target,
            &image::load_from_memory(res::texture::SPRITE)
                .unwrap()
                .to_luma8(),
        );

        let (v, i) = text(
            &target,
            &FString::from_iter([
                "red".default().color(1.0, 0.0, 0.0),
                "green".default().color(0.0, 1.0, 0.0),
                "blue".default().color(0.0, 0.0, 1.0),
            ]),
            &mut glyphs,
            80.0, // note this rendered size is a whole a lot bigger than the actual glyph resolution in the memory
            Vec2::ONE * 50.0,
            None,
        )
        .unwrap();
        let _ = glyphs.read(&target).await.save("target/sdf_glyphs.png");
        let text_vbo = VertexBuffer::new_with(&target, &v);
        let text_ibo = IndexBuffer::new_with(&target, &i);
        let ubo = UniformBuffer::new(&target, 1);
        let sdf_shader = SdfShader::new(&target);

        let quad = QuadMesh {
            pos: Vec2::new(200.0, 200.0),
            size: Vec2::new(150.0, 150.0),
            col: Color::WHITE,
            tex: TexturePosition::default(),
        };
        let rust_vbo = VertexBuffer::new_with(&target, &quad.vertices().collect::<Box<_>>());
        let rust_ibo = IndexBuffer::new_with(&target, &quad.indices(0).collect::<Box<_>>());

        Self {
            target,

            ws,

            glyphs,
            texture,

            rust_vbo,
            rust_ibo,
            text_vbo,
            text_ibo,
            ubo,
            sdf_shader,
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
        self.target.get_window().unwrap().set_visible(true);

        let mut frame = self.target.get_frame();

        self.ubo.upload(
            &mut self.target,
            &mut frame,
            &[Mat4::orthographic_rh(
                0.0,
                self.ws.size.width as _,
                0.0,
                self.ws.size.height as _,
                -100.0,
                100.0,
            )],
        );

        frame
            .primary_render_pass()
            .bind_vbo(&self.text_vbo)
            .bind_ibo(&self.text_ibo)
            .bind_group(&self.sdf_shader.bind_group((&self.ubo, &self.glyphs)))
            .bind_shader(&self.sdf_shader)
            .draw_indexed(0..self.text_ibo.capacity() as _, 0, 0..1)
            .bind_vbo(&self.rust_vbo)
            .bind_ibo(&self.rust_ibo)
            .bind_group(&self.sdf_shader.bind_group((&self.ubo, &self.texture)))
            .draw_indexed(0..self.rust_ibo.capacity() as _, 0, 0..1);

        self.target.finish_frame(frame);
    }
}

//

main_app!(async App);
