#![feature(duration_consts_float)]
#![feature(const_fn_floating_point_arithmetic)]

#[macro_use]
extern crate glium;

use font_loader::system_fonts::FontPropertyBuilder;
use glam::{Mat4, Vec3, Vec4};
use glium::{
    index::PrimitiveType,
    texture::CompressedSrgbTexture2d,
    uniforms::{MagnifySamplerFilter, MinifySamplerFilter},
    Blend, DrawParameters, Frame, IndexBuffer, Program, Surface, VertexBuffer,
};
use image::{buffer::ConvertBuffer, ImageFormat, RgbaImage};
use srs2dge::{
    packer::{glyph::Glyphs, packer2d::Rect},
    program::{default_program, DefaultVertex},
    runnable::Runnable,
    text::{
        self,
        format::{FString, Format, Formatted},
    },
};
use srs2dge::{text::program::text_program, Engine};
use static_res::static_res;
use std::{cell::RefCell, io::Cursor, iter::FromIterator, rc::Rc};
use winit::event::{ElementState, Event, KeyboardInput, VirtualKeyCode, WindowEvent};

//

static_res! { "res/**/*.{png,ttf}" }

struct FontIds {
    system: usize,
    roboto: usize,
    fira: usize,
}

//

fn if_pressed(event: &Event<()>, keycode: VirtualKeyCode) -> bool {
    matches!(
        event,
        Event::WindowEvent {
            event: WindowEvent::KeyboardInput {
                input: KeyboardInput {
                    virtual_keycode: Some(k),
                    state: ElementState::Pressed,
                    ..
                },
                ..
            },
            ..
        } if *k == keycode
    )
}

//

struct Quad {
    toggle: bool,
    a: f32,

    texture: CompressedSrgbTexture2d,

    vbo: VertexBuffer<DefaultVertex>,
    ibo: IndexBuffer<u8>,

    program: Rc<Program>,
}

impl Quad {
    fn new(engine: &Engine, program: Rc<Program>) -> Self {
        let vbo = VertexBuffer::new(
            engine,
            &[
                DefaultVertex::new(-0.5, -0.5, 1.0, 1.0, 1.0, 0.0, 1.0),
                DefaultVertex::new(-0.5, 0.5, 1.0, 0.0, 0.0, 0.0, 0.0),
                DefaultVertex::new(0.5, 0.5, 0.0, 1.0, 0.0, 1.0, 0.0),
                DefaultVertex::new(0.5, -0.5, 0.0, 0.0, 1.0, 1.0, 1.0),
            ],
        )
        .unwrap();

        let ibo =
            IndexBuffer::new(engine, PrimitiveType::TrianglesList, &[0_u8, 1, 2, 0, 2, 3]).unwrap();

        let img = image::load(Cursor::new(res::sprite_png), ImageFormat::Png)
            .unwrap()
            .to_rgba8();
        let dim = img.dimensions();
        let texture = glium::texture::RawImage2d::from_raw_rgba_reversed(&img, dim);
        let texture = glium::texture::CompressedSrgbTexture2d::new(engine, texture).unwrap();

        Self {
            toggle: true,
            a: 0.0,

            texture,

            vbo,
            ibo,

            program,
        }
    }

    #[inline]
    const fn speed(&self, engine: &Engine) -> f32 {
        engine.interval.as_secs_f32() * std::f32::consts::PI * 2.0 / 5.0
    }
}

impl Runnable for Quad {
    fn update(&mut self, engine: &Engine) {
        self.a += self.speed(engine);
    }

    fn event(&mut self, _: &Engine, event: &Event<()>) {
        if if_pressed(event, VirtualKeyCode::F1) {
            self.toggle = !self.toggle
        }
    }

    fn draw(&mut self, engine: &Engine, frame: &mut Frame, delta: f32) {
        if !self.toggle {
            return;
        }

        let params = DrawParameters {
            blend: Blend::alpha_blending(),
            ..Default::default()
        };

        let ubo = uniform! {
            mat: (Mat4::from_diagonal(Vec4::new(1.0, engine.aspect, 1.0, 1.0)) * Mat4::from_rotation_z(self.a + self.speed(engine) * delta)).to_cols_array_2d(),
            sprite: self.texture
                .sampled()
                .minify_filter(MinifySamplerFilter::Nearest)
                .magnify_filter(MagnifySamplerFilter::Nearest),
        };

        frame
            .draw(&self.vbo, &self.ibo, &self.program, &ubo, &params)
            .unwrap();
    }
}

struct CpuText {
    toggle: bool,

    texture: CompressedSrgbTexture2d,

    vbo: VertexBuffer<DefaultVertex>,
    ibo: IndexBuffer<u8>,

    program: Rc<Program>,
}

impl CpuText {
    fn new(
        engine: &Engine,
        text: &FString,
        glyphs: Rc<RefCell<Glyphs>>,
        program: Rc<Program>,
    ) -> Self {
        let img: RgbaImage = text::vbo::baked_text(text, &glyphs.borrow(), 18.0)
            .unwrap()
            .convert();
        let dim = img.dimensions();
        let texture = glium::texture::RawImage2d::from_raw_rgba_reversed(&img, dim);
        let texture = glium::texture::CompressedSrgbTexture2d::new(engine, texture).unwrap();

        let vbo = VertexBuffer::new(
            engine,
            &[
                DefaultVertex::from_arrays([300.0, 0.0], [1.0, 1.0, 1.0], [0.0, 1.0]),
                DefaultVertex::from_arrays([300.0, dim.1 as f32], [1.0, 1.0, 1.0], [0.0, 0.0]),
                DefaultVertex::from_arrays(
                    [300.0 + dim.0 as f32, dim.1 as f32],
                    [1.0, 1.0, 1.0],
                    [1.0, 0.0],
                ),
                DefaultVertex::from_arrays(
                    [300.0 + dim.0 as f32, 0.0],
                    [1.0, 1.0, 1.0],
                    [1.0, 1.0],
                ),
            ],
        )
        .unwrap();

        let ibo =
            IndexBuffer::new(engine, PrimitiveType::TrianglesList, &[0_u8, 1, 2, 0, 2, 3]).unwrap();

        Self {
            toggle: true,

            texture,

            vbo,
            ibo,

            program,
        }
    }
}

impl Runnable for CpuText {
    fn event(&mut self, _: &Engine, event: &Event<()>) {
        if if_pressed(event, VirtualKeyCode::F2) {
            self.toggle = !self.toggle
        }
    }

    fn draw(&mut self, engine: &Engine, frame: &mut Frame, _: f32) {
        if !self.toggle {
            return;
        }

        let params = DrawParameters {
            blend: Blend::alpha_blending(),
            ..Default::default()
        };

        let ubo = uniform! {
            mat: Mat4::orthographic_rh_gl(0.0, engine.size.0, 0.0, engine.size.1, -1.0, 1.0).to_cols_array_2d(),
            sprite: self.texture
                .sampled()
                .minify_filter(MinifySamplerFilter::Nearest)
                .magnify_filter(MagnifySamplerFilter::Nearest)
        };

        frame
            .draw(&self.vbo, &self.ibo, &self.program, &ubo, &params)
            .unwrap();
    }
}

struct DynText {
    toggle: bool,

    vbo: VertexBuffer<DefaultVertex>,
    ibo: IndexBuffer<u16>,

    fonts: Rc<FontIds>,
    glyphs: Rc<RefCell<Glyphs>>,
    program: Rc<Program>,
}

impl DynText {
    const MAX_CHARS: usize = 500;

    fn new(
        engine: &Engine,
        fonts: Rc<FontIds>,
        glyphs: Rc<RefCell<Glyphs>>,
        program: Rc<Program>,
    ) -> Self {
        let vbo = VertexBuffer::empty_dynamic(engine, Self::MAX_CHARS * 4).unwrap();

        let indices = (0..Self::MAX_CHARS as u16)
            .flat_map(|i| [i * 4, i * 4 + 1, i * 4 + 2, i * 4, i * 4 + 2, i * 4 + 3])
            .collect::<Vec<u16>>();
        let ibo = IndexBuffer::new(engine, PrimitiveType::TrianglesList, &indices[..]).unwrap();

        Self {
            toggle: true,

            vbo,
            ibo,

            fonts,
            glyphs,
            program,
        }
    }
}

impl Runnable for DynText {
    fn event(&mut self, _: &Engine, event: &Event<()>) {
        if if_pressed(event, VirtualKeyCode::F4) {
            self.toggle = !self.toggle
        }
    }

    fn draw(&mut self, engine: &Engine, frame: &mut Frame, _: f32) {
        if !self.toggle {
            return;
        }

        let params = DrawParameters {
            blend: Blend::alpha_blending(),
            ..Default::default()
        };

        let (frametime, fps) = engine.reporter.last_string();

        let text = format!("AVG frametime: {}\nAVG FPS: {}", frametime, fps)
            .default()
            .font(self.fonts.system)
            .color(0.0, 0.0, 0.0)
            .into();
        let mut glyphs = self.glyphs.borrow_mut();
        let mut vertices = text::vbo::text(&text, &mut glyphs, 18.0, 500.0, 0.0);
        vertices.truncate(Self::MAX_CHARS * 4);

        let charc = vertices.len().min(Self::MAX_CHARS * 4) / 4;

        self.vbo.slice(0..charc * 4).unwrap().write(&vertices);

        let ubo = uniform! {
            mat: Mat4::orthographic_rh_gl(0.0, engine.size.0, 0.0, engine.size.1, -1.0, 1.0).to_cols_array_2d(),
            sprite: glyphs
                .sampled()
                .minify_filter(MinifySamplerFilter::Nearest)
                .magnify_filter(MagnifySamplerFilter::Nearest)
        };

        frame
            .draw(
                self.vbo.slice(0..charc * 4).unwrap(),
                self.ibo.slice(0..charc * 6).unwrap(),
                &self.program,
                &ubo,
                &params,
            )
            .unwrap();
    }
}

struct GpuText {
    toggle: bool,

    vbo: VertexBuffer<DefaultVertex>,
    ibo: IndexBuffer<u32>,

    glyphs: Rc<RefCell<Glyphs>>,
    program: Rc<Program>,
}

impl GpuText {
    fn new(
        engine: &Engine,
        text: &FString,
        glyphs: Rc<RefCell<Glyphs>>,
        program: Rc<Program>,
    ) -> Self {
        let vertices = text::vbo::text(text, &mut glyphs.borrow_mut(), 18.0, 0.0, 0.0);
        let vbo = VertexBuffer::new(engine, &vertices[..]).unwrap();
        let indices = (0..(vbo.len() / 4) as u32)
            .flat_map(|i| [i * 4, i * 4 + 1, i * 4 + 2, i * 4, i * 4 + 2, i * 4 + 3])
            .collect::<Vec<u32>>();
        let ibo = IndexBuffer::new(engine, PrimitiveType::TrianglesList, &indices[..]).unwrap();

        Self {
            toggle: true,

            vbo,
            ibo,

            glyphs,
            program,
        }
    }
}

impl Runnable for GpuText {
    fn event(&mut self, _: &Engine, event: &Event<()>) {
        if if_pressed(event, VirtualKeyCode::F3) {
            self.toggle = !self.toggle
        }
    }

    fn draw(&mut self, engine: &Engine, frame: &mut Frame, _: f32) {
        if !self.toggle {
            return;
        }

        let params = DrawParameters {
            blend: Blend::alpha_blending(),
            ..Default::default()
        };

        let glyphs = self.glyphs.borrow();
        let ubo = uniform! {
            mat: Mat4::orthographic_rh_gl(0.0, engine.size.0, 0.0, engine.size.1, -1.0, 1.0).to_cols_array_2d(),
            sprite: glyphs
                .sampled()
                .minify_filter(MinifySamplerFilter::Nearest)
                .magnify_filter(MagnifySamplerFilter::Nearest)
        };

        frame
            .draw(&self.vbo, &self.ibo, &self.program, &ubo, &params)
            .unwrap();
    }
}

struct App {
    quad: Quad,
    cpu_text: CpuText,
    gpu_text: GpuText,
    dyn_text: DynText,
}

impl Runnable for App {
    fn update(&mut self, engine: &Engine) {
        self.quad.update(engine);
        self.cpu_text.update(engine);
        self.gpu_text.update(engine);
        self.dyn_text.update(engine);
    }

    fn event(&mut self, engine: &Engine, event: &Event<()>) {
        match event {
            Event::WindowEvent {
                event:
                    WindowEvent::KeyboardInput {
                        input:
                            KeyboardInput {
                                virtual_keycode: Some(VirtualKeyCode::Escape),
                                state: ElementState::Pressed,
                                ..
                            },
                        ..
                    },
                ..
            }
            | Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => engine.stop(),
            _ => {}
        }

        self.quad.event(engine, event);
        self.cpu_text.event(engine, event);
        self.gpu_text.event(engine, event);
        self.dyn_text.event(engine, event);
    }

    fn draw(&mut self, engine: &Engine, frame: &mut Frame, delta: f32) {
        let cc = Vec4::new(0.2, 0.22, 0.24, 1.0);
        frame.clear_color_srgb(cc.x, cc.y, cc.z, cc.w);

        // DRAW CUBE

        self.quad.draw(engine, frame, delta);

        // DRAW CPU TEXT

        self.cpu_text.draw(engine, frame, delta);

        // DRAW GPU TEXT

        self.gpu_text.draw(engine, frame, delta);

        // DRAW DYNAMIC TEXT

        self.dyn_text.draw(engine, frame, delta);
    }
}

pub fn main() {
    env_logger::init();

    let engine = Engine::init();

    // DEFAULT SHADER

    let default_program = Rc::new(default_program(&engine));
    let text_program = Rc::new(text_program(&engine));

    // TEXT SETUP

    let mut glyphs = Glyphs::new(&engine, Rect::new(512, 512)).unwrap();

    let fonts = Rc::new(FontIds {
        system: glyphs
            .add_font_property(FontPropertyBuilder::new().italic().build())
            .unwrap(),
        roboto: glyphs.add_font_bytes(res::roboto::font_ttf).unwrap(),
        fira: glyphs.add_font_bytes(res::fira::font_ttf).unwrap(),
    });

    let glyphs = Rc::new(RefCell::new(glyphs));

    let mut text = FString::from_iter([
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
    text.set_default_format(Format {
        color: Vec3::new(1.0, 1.0, 1.0),
        font: fonts.roboto,
    });

    // APP

    let quad = Quad::new(&engine, default_program.clone());
    let cpu_text = CpuText::new(&engine, &text, glyphs.clone(), default_program);
    let gpu_text = GpuText::new(&engine, &text, glyphs.clone(), text_program.clone());
    let dyn_text = DynText::new(&engine, fonts, glyphs, text_program);

    let app = App {
        quad,
        cpu_text,
        gpu_text,
        dyn_text,
    };

    // all initialization is done before the window is shown
    // this delay makes it more obvious
    std::thread::sleep(std::time::Duration::from_secs_f32(3.0));

    engine.run(app);
}
