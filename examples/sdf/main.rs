use srs2dge::winit::dpi::PhysicalSize;
use std::sync::Arc;

use srs2dge::prelude::*;

//

struct App {
    target: Target,

    ws: WindowState,

    glyphs: Glyphs,

    vbo: VertexBuffer,
    ibo: IndexBuffer,
    ubo: UniformBuffer<SdfUniform>,
    sdf_shader: SdfShader,
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
            px: 80.0, // note this rendered size is a whole a lot bigger than the actual glyph resolution in the memory
            ..Default::default()
        });
        let config = TextConfig {
            x_origin: 0,
            y_origin: 0,
            y_origin_line: YOrigin::Descender,
            ..Default::default()
        };
        let fonts = Fonts::new_bytes(res::font::ROBOTO).unwrap();
        let bb = TextChars::new(text.chars(), &fonts, config).bounding_box_typo();
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

        let mut glyphs = Glyphs::new(&target, Rect::new(130, 130), Some(48), fonts);

        let (v, i) = vbo::text(&target, text.chars(), &mut glyphs, config).unwrap();
        let vbo = VertexBuffer::new_with(&target, &v);
        let ibo = IndexBuffer::new_with(&target, &i);
        let ubo = UniformBuffer::new(&target, 1);
        let sdf_shader = SdfShader::new(&target);

        Self {
            target,

            ws,

            glyphs,

            vbo,
            ibo,
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
            &[SdfUniform::new_defaults(Mat4::orthographic_rh(
                0.0,
                self.ws.size.width as _,
                0.0,
                self.ws.size.height as _,
                -100.0,
                100.0,
            ))],
        );

        frame
            .primary_render_pass()
            .bind_vbo(&self.vbo)
            .bind_ibo(&self.ibo)
            .bind_group(&self.sdf_shader.bind_group((&self.ubo, &self.glyphs)))
            .bind_shader(&self.sdf_shader)
            .draw_indexed(0..self.ibo.capacity() as _, 0, 0..1);

        self.target.finish_frame(frame);
    }
}

//

main_app!(async App);
