use std::sync::Arc;
use winit::{event_loop::ControlFlow, window::Window};

use glam::*;
use main_game_loop::prelude::*;
use srs2dge::prelude::*;

//

static_res::static_res! { "res/**/*.{png,ttf}" }

//

struct App {
    target: Target,
    ws: WindowState,
    is: InputState,

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
    async fn new(target: &EventLoopTarget) -> Self {
        let engine = Engine::new();

        let window = Arc::new(Window::new(target).unwrap());
        let target = engine.new_target(window.clone()).await;

        let reporter = Reporter::new();

        let update_rate = UpdateRate::PerSecond(60);
        let update_loop = UpdateLoop::new(update_rate);

        let ws = WindowState::new(&window);
        let is = InputState::new();

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
                image::load_from_memory(res::texture::sprite_png)
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

        let mut glyphs = Glyphs::new(&target, Rect::new(128, 128), None);

        let fonts = FontIds {
            roboto: glyphs.add_font_bytes(res::font::roboto::font_ttf).unwrap(),
            fira: glyphs.add_font_bytes(res::font::fira::font_ttf).unwrap(),
        };

        let mut t = FString::from_iter([
            "∫|∫x dx + 'test text j'\u{FF1B}\\/\"\n\\VAW//\n\treadability\n\t\tline height\n\t\t\tnewline\n54is9\taligned\n\n".default(),
            "yy̆y\n".default(),
            "\u{FF1B}\n".default(),
            "fn ".default().color(1.0, 0.5, 0.0).font(fonts.fira),
            "main".leave().color(0.1, 0.1, 1.0),
            "() {\n\t".leave().color(1.0, 1.0, 1.0),
            "println!".leave().color(0.1, 0.1, 1.0),
            "(".leave().color(1.0, 1.0, 1.0),
            "\"Hello World!\"".leave().color(0.1, 1.0, 0.1),
            ");\n}\n\n".leave().color(1.0, 1.0, 1.0),
            "\tTAB\n".default(),
            "\t\tWIDTH\n".default(),
            "----IS\n".default(),
            "--------4\n".default()
        ]);
        t.set_default_format(Format {
            color: Vec3::new(1.0, 1.0, 1.0),
            font: fonts.roboto,
        });

        let (vbo, ibo) = vbo::text(&target, &t, &mut glyphs, 18.0, 100.0, -50.0).unwrap();
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
        self.quad.speed -= self.is.get_axis(InputAxis::Look, 0).x
            * UpdateRate::PerSecond(60).to_interval().as_secs_f32()
            / 2.0;
        self.quad.a += self.quad.speed;
    }

    fn draw(&mut self, delta: f32) {
        let mut frame = self.target.get_frame();

        let (frametime, fps) = self.reporter.last_string();
        let t = format!("AVG frametime: {}\nAVG FPS: {}", frametime, fps)
            .default()
            .font(self.fonts.roboto)
            .color(0.0, 0.0, 0.0)
            .into();
        let (vertices, indices) =
            vbo::text(&self.target, &t, &mut self.text.glyphs, 18.0, 500.0, -50.0).unwrap();
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
            .draw_indexed(0..self.dyn_text.ibo.capacity() as _, 0, 0..1);

        self.target.finish_frame(frame);
    }

    fn event(&mut self, event: Event<'_>, control: &mut ControlFlow) {
        *control = ControlFlow::Poll;
        self.ws.event(&event);
        self.is.event(&event);

        if self.ws.should_close {
            *control = ControlFlow::Exit;
            return;
        }

        if let Event::RedrawEventsCleared = event {
            let mut update_loop = self.update_loop.take().unwrap();
            let delta = update_loop.update(|| {
                self.update();
            });
            self.update_loop = Some(update_loop);

            let timer = self.reporter.begin();
            self.draw(delta);
            self.reporter.end(timer);

            if self.reporter.should_report() {
                let report = Reporter::report_all("3.0s", [("FRAME", &mut self.reporter)]);
                log::debug!("\n{}", report,);
            }
        }
    }
}

//

async fn run() {
    let target = EventLoop::new();
    let mut app = App::new(&target).await;
    target.run(move |event, _, control| {
        app.event(event, control);
    });
}

fn main() {
    // init_log();
    as_async(run());
}
