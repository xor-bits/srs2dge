use std::sync::Arc;

use srs2dge::{prelude::*, text::config::TextConfig};

//

struct App {
    target: Target,
    ws: WindowState,
    is: KeyboardState,

    update_loop: Option<UpdateLoop>,
    reporter: Reporter,

    texture_shader: Texture2DShader,
    text_shader: TextShader,

    quad: Quad,
    text: Text,
    dyn_text: DynText,
    fonts: FontIds,
}

struct Quad {
    vbo: VertexBuffer,
    ibo: IndexBuffer,
    ubo: UniformBuffer<Mat4>,
    texture: TextureAtlasMap<u8>,
    a: f32,
    speed: f32,
}

struct Text {
    vbo: VertexBuffer,
    ibo: IndexBuffer,
    ubo: UniformBuffer<Mat4>,
    glyphs: Glyphs,
}

struct DynText {
    vbo: VertexBuffer,
    ibo: IndexBuffer,
}

struct FontIds {
    // system: usize,
    roboto: usize,
    fira: usize,
}

//

impl App {
    async fn init(target: &EventLoopTarget) -> Self {
        let engine = Engine::new();

        let window = Arc::new(
            WindowBuilder::new()
                .with_visible(false)
                .build(target)
                .unwrap(),
        );
        let mut target = engine.new_target(window.clone()).await;
        target.set_vsync(false);

        let reporter = Reporter::new();

        let update_rate = UpdateRate::PerSecond(60);
        let update_loop = UpdateLoop::new(update_rate);

        let ws = WindowState::new(&window);
        let is = KeyboardState::new();

        let texture_shader = Texture2DShader::new(&target);
        let text_shader = TextShader::new(&target);

        let vbo = VertexBuffer::new_with(
            &target,
            &[
                DefaultVertex::from_arrays([-0.5, -0.5], [1.0, 1.0, 1.0, 1.0], [0.0, 1.0]),
                DefaultVertex::from_arrays([-0.5, 0.5], [1.0, 0.0, 0.0, 1.0], [0.0, 0.0]),
                DefaultVertex::from_arrays([0.5, 0.5], [0.0, 1.0, 0.0, 1.0], [1.0, 0.0]),
                DefaultVertex::from_arrays([0.5, -0.5], [0.0, 0.0, 1.0, 1.0], [1.0, 1.0]),
            ],
        );

        let ibo = IndexBuffer::new_with(&target, &[0_u32, 1, 2, 0, 2, 3]);

        let ubo = UniformBuffer::new_single(
            &target,
            Mat4::from_diagonal(Vec4::new(1.0, ws.aspect, 1.0, 1.0)) * Mat4::from_rotation_z(0.0),
        );

        let texture = TextureAtlasMapBuilder::new()
            .with(
                0,
                image::load_from_memory(res::texture::SPRITE)
                    .unwrap()
                    .to_rgba8(),
            )
            .build(&target);

        // 2pi radians per 5 seconds
        let speed = std::f32::consts::PI * 2.0 * update_rate.to_interval().as_secs_f32() / 5.0;

        let quad = Quad {
            vbo,
            ibo,
            ubo,
            texture,
            a: 0.0,
            speed,
        };

        let mut glyphs =
            Glyphs::new_with_fallback_bytes(&target, Rect::new(512, 512), None, res::font::ROBOTO)
                .unwrap();

        let fonts = FontIds {
            roboto: 0,
            fira: glyphs.fonts_mut().add_font_bytes(res::font::FIRA).unwrap(),
        };

        // lines are not required to be on their own
        // `with` calls, it just makes it look nicer
        let text = FormatString::builder()
            // default config
            .with(Color::WHITE)
            .with(fonts.roboto)
            .with(18.0)
            // text
            .with("All of ASCII:\n")
            .with((0..64_u8).map(char::from).collect::<String>())
            .with("\n")
            .with((64..128_u8).map(char::from).collect::<String>())
            .with("\n")
            .with((128..192_u8).map(char::from).collect::<String>())
            .with("\n")
            .with((192..=255_u8).map(char::from).collect::<String>())
            .with("\n")
            .with("Random Unicode: \u{0416} \u{0409}\n")
            .with("|\t|\t|\t|\t|\t|\ttabs\n")
            .with("may be|\t|\t|\tworking\n")
            // code text
            .with(Color::ORANGE)
            .with(fonts.fira)
            .with("fn ")
            .with(Color::new(0.1, 0.1, 1.0, 1.0))
            .with("main")
            .with(Color::WHITE)
            .with("() {\n\t")
            .with(Color::new(0.1, 0.1, 1.0, 1.0))
            .with("println!")
            .with(Color::WHITE)
            .with("(")
            .with(Color::new(0.1, 1.0, 0.1, 1.0))
            .with(r#""Hello World!""#)
            .with(Color::WHITE)
            .with(");\n}\n\n")
            // text
            .with(Color::WHITE)
            .with(fonts.roboto)
            .with("\tTAB\n")
            .with("\t\tWIDTH\n")
            .with("    IS\n")
            .with("        4 spaces\n")
            .with(Color::WHITE)
            .with(fonts.fira)
            .with("It\n")
            .with("\tis\n")
            .with("\t\tmore\n")
            .with("\t\t\tclear\n")
            .with("\t\t\t\twith\n")
            .with("\t\t\t\t\tmonospace\n")
            .with("\t\t\t\t\t\tfonts\n");

        let (vbo, ibo) = vbo::text(
            &target,
            text.chars(),
            &mut glyphs,
            TextConfig {
                x_origin: 50,
                y_origin: -50,
                ..Default::default()
            },
        )
        .unwrap();
        let (vbo, ibo) = (
            VertexBuffer::new_with(&target, &vbo),
            IndexBuffer::new_with(&target, &ibo),
        );

        let ubo = UniformBuffer::new_single(
            &target,
            Mat4::orthographic_rh(
                0.0,
                ws.size.width as _,
                -(ws.size.height as f32),
                0.0,
                -100.0,
                100.0,
            ),
        );

        let text = Text {
            vbo,
            ibo,
            ubo,
            glyphs,
        };

        let vbo = VertexBuffer::new(&target, 400);
        let ibo = IndexBuffer::new_with(
            &target,
            &(0..100)
                .flat_map(|i| [i * 4, i * 4 + 1, i * 4 + 2, i * 4, i * 4 + 2, i * 4 + 3])
                .collect::<Vec<_>>(),
        );

        let dyn_text = DynText { vbo, ibo };

        Self {
            target,
            ws,
            is,

            update_loop: Some(update_loop),
            reporter,

            texture_shader,
            text_shader,

            quad,
            text,
            dyn_text,
            fonts,
        }
    }

    fn update(&mut self) {
        let mut delta = 0.0;
        if self.is.pressed(VirtualKeyCode::Left) {
            delta -= 1.0;
        }
        if self.is.pressed(VirtualKeyCode::Right) {
            delta += 1.0;
        }

        self.quad.speed -= delta * UpdateRate::PerSecond(60).to_interval().as_secs_f32() / 2.0;
        self.quad.a += self.quad.speed;
    }
}

impl Runnable for App {
    fn draw(&mut self) {
        // update
        let mut update_loop = self.update_loop.take().unwrap();
        let delta = update_loop.update(|| {
            self.update();
        });
        self.update_loop = Some(update_loop);

        // draw
        let timer = self.reporter.begin();
        let mut frame = self.target.get_frame();

        let (frametime, fps) = self.reporter.last_string();
        // another way to make format strings
        let text =
            /* FormatString::from_iter([
                format!("AVG frametime: {}\nAVG FPS: {}", frametime, fps).into()
            ]) */
            // or in this case just this:
            FormatString::from(format!("AVG frametime: {}\nAVG FPS: {}", frametime, fps))
            .with_init(Format {
                color: Color::WHITE,
                font: self.fonts.roboto,
                px: 18.0,
            });
        let (vertices, indices) = vbo::text(
            &self.target,
            text.chars(),
            &mut self.text.glyphs,
            TextConfig {
                x_origin: 500,
                y_origin: -50,
                x_origin_point: XOrigin::Right,
                ..Default::default()
            },
        )
        .unwrap();
        self.dyn_text.vbo.upload(
            &mut self.target,
            &mut frame,
            &vertices[..self.dyn_text.vbo.capacity().min(vertices.len())],
        );
        self.dyn_text.ibo.upload(
            &mut self.target,
            &mut frame,
            &indices[..self.dyn_text.ibo.capacity().min(indices.len())],
        );
        self.quad.ubo.upload(
            &mut self.target,
            &mut frame,
            &[
                Mat4::from_diagonal(Vec4::new(1.0, self.ws.aspect, 1.0, 1.0))
                    * Mat4::from_rotation_z(self.quad.a + self.quad.speed * delta),
            ],
        );
        self.text.ubo.upload(
            &mut self.target,
            &mut frame,
            &[Mat4::orthographic_rh(
                0.0,
                self.ws.size.width as _,
                -(self.ws.size.height as f32),
                0.0,
                -100.0,
                100.0,
            )],
        );

        frame
            .primary_render_pass()
            // quad draw
            .bind_ibo(&self.quad.ibo)
            .bind_vbo(&self.quad.vbo)
            .bind_group(
                &self
                    .texture_shader
                    .bind_group((&self.quad.ubo, &self.quad.texture)),
            )
            .bind_shader(&self.texture_shader)
            .draw_indexed(0..6, 0, 0..1)
            // static text draw
            .bind_ibo(&self.text.ibo)
            .bind_vbo(&self.text.vbo)
            .bind_group(
                &self
                    .text_shader
                    .bind_group((&self.text.ubo, &self.text.glyphs)),
            )
            .bind_shader(&self.text_shader)
            .draw_indexed(0..self.text.ibo.capacity() as _, 0, 0..1)
            // dynamic text draw
            .bind_ibo(&self.dyn_text.ibo)
            .bind_vbo(&self.dyn_text.vbo)
            .bind_group(
                &self
                    .text_shader
                    .bind_group((&self.text.ubo, &self.text.glyphs)),
            )
            .bind_shader(&self.text_shader)
            .draw_indexed(0..indices.len() as _, 0, 0..1);

        self.target.finish_frame(frame);
        self.reporter.end(timer);

        // perf report
        if self.reporter.should_report() {
            let report = Reporter::report_all("3.0s", [("FRAME", &mut self.reporter)]);
            log::debug!("\n{}", report,);
        }
    }

    fn event(&mut self, event: Event<'_>, _: &EventLoopTarget, control: &mut ControlFlow) {
        *control = ControlFlow::Poll;
        self.ws.event(&event);
        self.is.event(&event);

        if self.ws.should_close {
            *control = ControlFlow::Exit;
        }
    }
}

//

main_app!(async App);
