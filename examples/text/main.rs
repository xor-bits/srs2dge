use srs2dge::winit::dpi::PhysicalSize;
use std::sync::Arc;

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
        // setup text
        let text = FormatString::from_iter([
            Color::RED.into(),
            "red ".into(),
            Color::GREEN.into(),
            "green ".into(),
            Color::BLUE.into(),
            "blue".into(),
        ])
        .with_init(Format {
            px: 80.0,
            ..Default::default()
        });
        let config = TextConfig {
            align: TextAlign::bottom_left(),
            ..Default::default()
        };
        let fonts = Fonts::new_bytes(res::font::ROBOTO).unwrap();
        let bb = TextChars::new(text.chars(), &fonts, config).bounding_box();
        log::debug!("{bb:?}");

        // engine
        let engine = Engine::new();
        let target = engine
            .new_target(Arc::new(
                WindowBuilder::new()
                    .with_visible(false)
                    .with_inner_size(PhysicalSize::new(bb.width, bb.height))
                    .build(target)
                    .unwrap(),
            ))
            .await;

        let ws = WindowState::new(&target.get_window().unwrap());

        let mut glyphs = Glyphs::new_with_fallback_bytes(
            &target,
            Rect::new(128, 156),
            None,
            res::font::ROBOTO,
            None,
        )
        .unwrap();

        let (v, i) = vbo::text(&target, text.chars(), &mut glyphs, config)
            .unwrap()
            .collect_mesh();
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

    async fn event(&mut self, event: Event<'_>, _: &EventLoopTarget, control: &mut ControlFlow) {
        self.ws.event(&event);

        if self.ws.should_close {
            *control = ControlFlow::Exit;
        }
    }

    async fn draw(&mut self) {
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
            .bind_vbo(&self.vbo)
            .bind_ibo(&self.ibo)
            .bind_group(&self.shader.bind_group((&self.ubo, &self.glyphs)))
            .bind_shader(&self.shader)
            .draw_indexed(0..self.ibo.capacity() as _, 0, 0..1);

        self.target.finish_frame(frame);
    }
}

//

fn main() {
    app!(App);
}
