/* #![feature(duration_consts_float)]
#![feature(const_fn_floating_point_arithmetic)]

#[macro_use]
extern crate glium;

use font_loader::system_fonts::FontPropertyBuilder;
use main_game_loop::{AnyEngine, Event, GameLoop, Runnable};
use glam::{Mat4, Vec3, Vec4};
use glium::{
    index::PrimitiveType,
    texture::CompressedSrgbTexture2d,
    uniforms::{MagnifySamplerFilter, MinifySamplerFilter},
    Blend, DrawParameters, Frame, IndexBuffer, Program, Surface, VertexBuffer,
};
use image::{buffer::ConvertBuffer, RgbaImage};
use srs2dge::{
    packer::{
        glyph::Glyphs,
        packer2d::Rect,
        texture::{TextureAtlasMap, TextureAtlasMapBuilder},
    },
    program::{default_program, DefaultVertex},
    text::{
        self,
        format::{FString, Format, Formatted},
    },
    BuildEngine,
};
use srs2dge::{text::program::text_program, Engine};
use static_res::static_res;
use std::{cell::RefCell, iter::FromIterator, rc::Rc};
use winit::{
    dpi::LogicalSize,
    event::{ElementState, Event as WinitEvent, KeyboardInput, VirtualKeyCode, WindowEvent},
    window::WindowBuilder,
};

//

static_res! { "res/**/
*.{png,ttf}" }

struct FontIds {
    system: usize,
    roboto: usize,
    fira: usize,
}

//

fn if_pressed(event: &Event, keycode: VirtualKeyCode) -> bool {
    matches!(
        event,
        Event::WinitEvent(WinitEvent::WindowEvent {
            event: WindowEvent::KeyboardInput {
                input: KeyboardInput {
                    virtual_keycode: Some(k),
                    state: ElementState::Pressed,
                    ..
                },
                ..
            },
            ..
        }) if *k == keycode
    )
}

//

struct Quad {
    toggle: bool,
    a: f32,

    texture: TextureAtlasMap<u8>,

    vbo: VertexBuffer<DefaultVertex>,
    ibo: IndexBuffer<u8>,

    program: Rc<Program>,
}

impl Quad {
    fn new(gl: &mut GameLoop<Engine>, program: Rc<Program>) -> Self {
        let vbo = VertexBuffer::new(
            gl,
            &[
                DefaultVertex::new(-0.5, -0.5, 1.0, 1.0, 1.0, 1.0, 0.0, 1.0),
                DefaultVertex::new(-0.5, 0.5, 1.0, 0.0, 0.0, 1.0, 0.0, 0.0),
                DefaultVertex::new(0.5, 0.5, 0.0, 1.0, 0.0, 1.0, 1.0, 0.0),
                DefaultVertex::new(0.5, -0.5, 0.0, 0.0, 1.0, 1.0, 1.0, 1.0),
            ],
        )
        .unwrap();

        let ibo =
            IndexBuffer::new(gl, PrimitiveType::TrianglesList, &[0_u8, 1, 2, 0, 2, 3]).unwrap();

        let texture = TextureAtlasMapBuilder::new()
            .with(
                0_u8,
                image::load_from_memory(res::sprite_png).unwrap().to_rgba8(),
            )
            .build(gl);

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
    const fn speed(&self, gl: &GameLoop<Engine>) -> f32 {
        gl.interval.as_secs_f32() * std::f32::consts::PI * 2.0 / 5.0
    }
}

impl Runnable<Engine> for Quad {
    fn update(&mut self, gl: &mut GameLoop<Engine>) {
        self.a += self.speed(gl);
    }

    fn event(&mut self, _: &mut GameLoop<Engine>, event: &Event) {
        if if_pressed(event, VirtualKeyCode::F1) {
            self.toggle = !self.toggle
        }
    }

    fn draw(&mut self, gl: &mut GameLoop<Engine>, frame: &mut Frame, delta: f32) {
        if !self.toggle {
            return;
        }

        let params = DrawParameters {
            blend: Blend::alpha_blending(),
            ..Default::default()
        };

        let ubo = uniform! {
            mat: (Mat4::from_diagonal(Vec4::new(1.0, gl.aspect, 1.0, 1.0)) * Mat4::from_rotation_z(self.a + self.speed(gl) * delta)).to_cols_array_2d(),
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
        gl: &mut GameLoop<Engine>,
        text: &FString,
        glyphs: Rc<RefCell<Glyphs>>,
        program: Rc<Program>,
    ) -> Self {
        let img: RgbaImage = text::vbo::baked_text(text, &glyphs.borrow(), 18.0)
            .unwrap()
            .convert();
        let dim = img.dimensions();
        let texture = glium::texture::RawImage2d::from_raw_rgba_reversed(&img, dim);
        let texture = glium::texture::CompressedSrgbTexture2d::new(gl, texture).unwrap();

        let vbo = VertexBuffer::new(
            gl,
            &[
                DefaultVertex::from_arrays([300.0, 0.0], [1.0, 1.0, 1.0, 1.0], [0.0, 1.0]),
                DefaultVertex::from_arrays([300.0, dim.1 as f32], [1.0, 1.0, 1.0, 1.0], [0.0, 0.0]),
                DefaultVertex::from_arrays(
                    [300.0 + dim.0 as f32, dim.1 as f32],
                    [1.0, 1.0, 1.0, 1.0],
                    [1.0, 0.0],
                ),
                DefaultVertex::from_arrays(
                    [300.0 + dim.0 as f32, 0.0],
                    [1.0, 1.0, 1.0, 1.0],
                    [1.0, 1.0],
                ),
            ],
        )
        .unwrap();

        let ibo =
            IndexBuffer::new(gl, PrimitiveType::TrianglesList, &[0_u8, 1, 2, 0, 2, 3]).unwrap();

        Self {
            toggle: true,

            texture,

            vbo,
            ibo,

            program,
        }
    }
}

impl Runnable<Engine> for CpuText {
    fn event(&mut self, _: &mut GameLoop<Engine>, event: &Event) {
        if if_pressed(event, VirtualKeyCode::F2) {
            self.toggle = !self.toggle
        }
    }

    fn draw(&mut self, gl: &mut GameLoop<Engine>, frame: &mut Frame, _: f32) {
        if !self.toggle {
            return;
        }

        let params = DrawParameters {
            blend: Blend::alpha_blending(),
            ..Default::default()
        };

        let ubo = uniform! {
            mat: Mat4::orthographic_rh_gl(0.0, gl.size.0, 0.0, gl.size.1, -1.0, 1.0).to_cols_array_2d(),
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
        gl: &mut GameLoop<Engine>,
        fonts: Rc<FontIds>,
        glyphs: Rc<RefCell<Glyphs>>,
        program: Rc<Program>,
    ) -> Self {
        let vbo = VertexBuffer::empty_dynamic(gl, Self::MAX_CHARS * 4).unwrap();

        let indices = (0..Self::MAX_CHARS as u16)
            .flat_map(|i| [i * 4, i * 4 + 1, i * 4 + 2, i * 4, i * 4 + 2, i * 4 + 3])
            .collect::<Vec<u16>>();
        let ibo = IndexBuffer::new(gl, PrimitiveType::TrianglesList, &indices[..]).unwrap();

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

impl Runnable<Engine> for DynText {
    fn event(&mut self, _: &mut GameLoop<Engine>, event: &Event) {
        if if_pressed(event, VirtualKeyCode::F4) {
            self.toggle = !self.toggle
        }
    }

    fn draw(&mut self, gl: &mut GameLoop<Engine>, frame: &mut Frame, _: f32) {
        if !self.toggle {
            return;
        }

        let params = DrawParameters {
            blend: Blend::alpha_blending(),
            ..Default::default()
        };

        let (frametime, fps) = gl.frame_reporter.last_string();

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
            mat: Mat4::orthographic_rh_gl(0.0, gl.size.0, 0.0, gl.size.1, -1.0, 1.0).to_cols_array_2d(),
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
        gl: &mut GameLoop<Engine>,
        text: &FString,
        glyphs: Rc<RefCell<Glyphs>>,
        program: Rc<Program>,
    ) -> Self {
        let vertices = text::vbo::text(text, &mut glyphs.borrow_mut(), 18.0, 0.0, 0.0);
        let vbo = VertexBuffer::new(gl, &vertices[..]).unwrap();
        let indices = (0..(vbo.len() / 4) as u32)
            .flat_map(|i| [i * 4, i * 4 + 1, i * 4 + 2, i * 4, i * 4 + 2, i * 4 + 3])
            .collect::<Vec<u32>>();
        let ibo = IndexBuffer::new(gl, PrimitiveType::TrianglesList, &indices[..]).unwrap();

        Self {
            toggle: true,

            vbo,
            ibo,

            glyphs,
            program,
        }
    }
}

impl Runnable<Engine> for GpuText {
    fn event(&mut self, _: &mut GameLoop<Engine>, event: &Event) {
        if if_pressed(event, VirtualKeyCode::F3) {
            self.toggle = !self.toggle
        }
    }

    fn draw(&mut self, gl: &mut GameLoop<Engine>, frame: &mut Frame, _: f32) {
        if !self.toggle {
            return;
        }

        let params = DrawParameters {
            blend: Blend::alpha_blending(),
            ..Default::default()
        };

        let glyphs = self.glyphs.borrow();
        let ubo = uniform! {
            mat: Mat4::orthographic_rh_gl(0.0, gl.size.0, 0.0, gl.size.1, -1.0, 1.0).to_cols_array_2d(),
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

impl Runnable<Engine> for App {
    fn update(&mut self, gl: &mut GameLoop<Engine>) {
        self.quad.update(gl);
        self.cpu_text.update(gl);
        self.gpu_text.update(gl);
        self.dyn_text.update(gl);
    }

    fn event(&mut self, gl: &mut GameLoop<Engine>, event: &Event) {
        if let Event::WinitEvent(
            WinitEvent::WindowEvent {
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
            | WinitEvent::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            },
        ) = event
        {
            gl.stop()
        }

        self.quad.event(gl, event);
        self.cpu_text.event(gl, event);
        self.gpu_text.event(gl, event);
        self.dyn_text.event(gl, event);
    }

    fn draw(&mut self, gl: &mut GameLoop<Engine>, frame: &mut Frame, delta: f32) {
        let cc = Vec4::new(0.2, 0.22, 0.24, 1.0);
        frame.clear_color_srgb(cc.x, cc.y, cc.z, cc.w);

        // DRAW CUBE

        self.quad.draw(gl, frame, delta);

        // DRAW CPU TEXT

        self.cpu_text.draw(gl, frame, delta);

        // DRAW GPU TEXT

        self.gpu_text.draw(gl, frame, delta);

        // DRAW DYNAMIC TEXT

        self.dyn_text.draw(gl, frame, delta);
    }
}

pub fn main() {
    env_logger::init();

    let mut gl = WindowBuilder::new()
        .with_title("Main")
        .with_inner_size(LogicalSize::new(400_u16, 400_u16))
        .build_engine()
        .build_main_game_loop();

    // DEFAULT SHADER

    let default_program = Rc::new(default_program(&gl.engine));
    let text_program = Rc::new(text_program(&gl.engine));

    // TEXT SETUP

    let mut glyphs = Glyphs::new(&gl.engine, Rect::new(512, 512)).unwrap();

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

    let quad = Quad::new(&mut gl, default_program.clone());
    let cpu_text = CpuText::new(&mut gl, &text, glyphs.clone(), default_program);
    let gpu_text = GpuText::new(&mut gl, &text, glyphs.clone(), text_program.clone());
    let dyn_text = DynText::new(&mut gl, fonts, glyphs, text_program);

    let app = App {
        quad,
        cpu_text,
        gpu_text,
        dyn_text,
    };

    gl.run(app)
}
*/
