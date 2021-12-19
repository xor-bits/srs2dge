// #![feature(let_else)]
// #![feature(destructuring_assignment)]
// #![feature(test)]
#![feature(drain_filter)]
#![feature(int_abs_diff)]
#![feature(type_alias_impl_trait)]

use crate::{
    packer::{glyph::Glyphs, packer2d::Rect},
    report::Reporter,
    text::format::{FString, Formatted},
};
use fontdue::{Font, FontSettings};
use glam::{Mat4, Vec4};
use glium::{
    glutin::ContextBuilder,
    index::PrimitiveType,
    uniforms::{MagnifySamplerFilter, MinifySamplerFilter},
    Blend, DrawParameters, IndexBuffer, Surface, VertexBuffer,
};
use image::{buffer::ConvertBuffer, ImageFormat, RgbaImage};
use static_res::static_res;
use std::{io::Cursor, time::Duration};
use winit::{
    dpi::LogicalSize,
    event::{ElementState, Event, KeyboardInput, VirtualKeyCode, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

#[macro_use]
extern crate glium;

mod packer;
mod report;
mod text;

static_res! { "res/**/*.{png,ttf}" }

#[derive(Debug, Clone, Copy)]
struct Vertex {
    vi_position: [f32; 2],
    vi_color: [f32; 3],
    vi_uv: [f32; 2],
}
glium::implement_vertex!(Vertex, vi_position, vi_color, vi_uv);

fn main() {
    env_logger::init();

    // SETUP

    let event_loop = EventLoop::new();
    let window_builder = WindowBuilder::new()
        .with_inner_size(LogicalSize::new(600_i32, 600_i32))
        .with_title("Title");
    let context = ContextBuilder::new()
        // .with_vsync(true)
        .build_windowed(window_builder, &event_loop)
        .unwrap();
    let scale_factor = context.window().scale_factor();
    let display = glium::Display::from_gl_window(context).unwrap();

    log::debug!("OpenGL Vendor: {}", display.get_opengl_vendor_string());
    log::debug!("OpenGL Renderer: {}", display.get_opengl_renderer_string());
    log::debug!("OpenGL Version: {}", display.get_opengl_version_string());

    // DEFAULT SHADER

    let shader = glium::program!(&display,
        140 => {
            vertex: "#version 140
                in vec2 vi_position;
                in vec3 vi_color;
                in vec2 vi_uv;

                uniform mat4 mat;

                out vec3 fi_color;
                out vec2 fi_uv;

                void main() {
                    gl_Position = mat * vec4(vi_position, 0.0, 1.0) * vec4(1.0, -1.0, 1.0, 1.0);
                    fi_color = vi_color;
                    fi_uv = vi_uv;
                }",
            fragment: "#version 140
                in vec3 fi_color;
                in vec2 fi_uv;

                uniform sampler2D sprite;

                out vec4 o_color;

                void main() {
                    o_color = vec4(fi_color, 1.0) * texture(sprite, fi_uv);
                }"
        }
    )
    .unwrap();

    // SPINNING QUAD

    let vbo = VertexBuffer::new(
        &display,
        &[
            Vertex {
                vi_position: [-0.5, -0.5],
                vi_color: [1.0, 1.0, 1.0],
                vi_uv: [0.0, 1.0],
            },
            Vertex {
                vi_position: [-0.5, 0.5],
                vi_color: [1.0, 0.0, 0.0],
                vi_uv: [0.0, 0.0],
            },
            Vertex {
                vi_position: [0.5, 0.5],
                vi_color: [0.0, 1.0, 0.0],
                vi_uv: [1.0, 0.0],
            },
            Vertex {
                vi_position: [0.5, -0.5],
                vi_color: [0.0, 0.0, 1.0],
                vi_uv: [1.0, 1.0],
            },
        ],
    )
    .unwrap();

    let ibo = IndexBuffer::new(
        &display,
        PrimitiveType::TrianglesList,
        &[0_u8, 1, 2, 0, 2, 3],
    )
    .unwrap();

    let img = image::load(Cursor::new(res::sprite_png), ImageFormat::Png)
        .unwrap()
        .to_rgba8();
    let dim = img.dimensions();
    let texture = glium::texture::RawImage2d::from_raw_rgba_reversed(&img, dim);
    let texture = glium::texture::CompressedSrgbTexture2d::new(&display, texture).unwrap();

    // TEXT SETUP

    let text = FString::from_iter([
        "∫|∫x dx + 'test text j'\u{FF1B}\\/\"\n\\VAW//\n\treadability\n\t\tline height\n\t\t\tnewline\n54is9\taligned\n\n".formatted(),
        "yy̆y\n".formatted(),
        "\u{FF1B}\n".formatted(),
        "fn ".colored(1.0, 0.5, 0.0),
        "main".colored(0.1, 0.1, 1.0),
        "() {\n\t".formatted(),
        "println!".colored(0.1, 0.1, 1.0),
        "(".formatted(),
        "\"Hello World!\"".colored(0.1, 1.0, 0.1),
        ");\n}\n\n".formatted(),
        "\tTAB\n".formatted(),
        "\t\tWIDTH\n".formatted(),
        "----IS\n".formatted(),
        "--------4\n".formatted()
    ]);

    // CPU RENDERED TEXT

    let font = Font::from_bytes(res::roboto::font_ttf, FontSettings::default()).unwrap();

    let text1_img: RgbaImage = text::vbo::baked_text(&text, &font, 18.0).unwrap().convert();
    let text1_dim = text1_img.dimensions();
    let text1_texture = glium::texture::RawImage2d::from_raw_rgba_reversed(&text1_img, text1_dim);
    let text1_texture =
        glium::texture::CompressedSrgbTexture2d::new(&display, text1_texture).unwrap();

    let text1_vbo = VertexBuffer::new(
        &display,
        &[
            Vertex {
                vi_position: [300.0, 0.0],
                vi_color: [1.0, 1.0, 1.0],
                vi_uv: [0.0, 1.0],
            },
            Vertex {
                vi_position: [300.0, text1_dim.1 as f32],
                vi_color: [1.0, 1.0, 1.0],
                vi_uv: [0.0, 0.0],
            },
            Vertex {
                vi_position: [300.0 + text1_dim.0 as f32, text1_dim.1 as f32],
                vi_color: [1.0, 1.0, 1.0],
                vi_uv: [1.0, 0.0],
            },
            Vertex {
                vi_position: [300.0 + text1_dim.0 as f32, 0.0],
                vi_color: [1.0, 1.0, 1.0],
                vi_uv: [1.0, 1.0],
            },
        ],
    )
    .unwrap();

    // GPU RENDERED TEXT

    let font = Font::from_bytes(res::fira::font_ttf, FontSettings::default()).unwrap();

    let mut glyphs = Glyphs::new(&display, font, Rect::new(512, 512)).unwrap();
    for c in text.as_str().chars() {
        glyphs.queue(c, 18);
    }
    glyphs.flush();

    let vertices = text::vbo::text(&text, &mut glyphs, 18.0, 0.0, 0.0);
    let text2_vbo = VertexBuffer::new(&display, &vertices[..]).unwrap();
    let indices = (0..(text2_vbo.len() / 4) as u32)
        .flat_map(|i| [i * 4, i * 4 + 1, i * 4 + 2, i * 4, i * 4 + 2, i * 4 + 3])
        .collect::<Vec<u32>>();
    let text2_ibo = IndexBuffer::new(&display, PrimitiveType::TrianglesList, &indices[..]).unwrap();
    let text2_program = text::vbo::text_program(&display);

    // MAIN LOOP

    let mut a = 0.0;
    let mut aspect = 0.0;
    let mut size = (0.0, 0.0);
    let mut reporter = Reporter::new_with_interval(Duration::from_millis(500));

    let (mut gpu_toggle, mut cpu_toggle, mut quad_toggle) = (true, true, true);

    event_loop.run(move |event, _, control| {
        *control = ControlFlow::Poll;

        match event {
            Event::WindowEvent {
                event: WindowEvent::Resized(s),
                ..
            } => {
                size = (s.width as f32, s.height as f32);
                let s = s.to_logical::<f32>(scale_factor);
                aspect = s.width / s.height;
            }
            Event::RedrawEventsCleared => {
                let timer = reporter.begin();
                {
                    let mut frame = display.draw();
                    let cc = Vec4::new(0.2, 0.22, 0.24, 1.0);
                    frame.clear_color_srgb(cc.x, cc.y, cc.z, cc.w);
    
                    let params = DrawParameters {
                        blend: Blend::alpha_blending(),
                        ..Default::default()
                    };

                    // DRAW CUBE

                    a += reporter.last().0.as_secs_f32() * std::f32::consts::PI * 2.0 / 5.0;
                    if quad_toggle {
                        let ubo = uniform! {
                            mat: (Mat4::from_diagonal(Vec4::new(1.0, aspect, 1.0, 1.0)) * Mat4::from_rotation_z(a)).to_cols_array_2d(),
                            sprite: texture
                                .sampled()
                                .minify_filter(MinifySamplerFilter::Nearest)
                                .magnify_filter(MagnifySamplerFilter::Nearest),
                        };
    
                        frame.draw(&vbo, &ibo, &shader, &ubo, &params).unwrap();
                    }

                    // DRAW CPU TEXT

                    if cpu_toggle {
                        let ubo = uniform! {
                            mat: Mat4::orthographic_rh_gl(0.0, size.0, 0.0, size.1, -1.0, 1.0).to_cols_array_2d(),
                            sprite: text1_texture
                                .sampled()
                                .minify_filter(MinifySamplerFilter::Nearest)
                                .magnify_filter(MagnifySamplerFilter::Nearest)
                        };
                        frame.draw(&text1_vbo, &ibo, &shader, &ubo, &params).unwrap();
                    }

                    // DRAW DYNAMIC TEXT

                    let mut text = FString::new();
                    text += format!("AVG frametime: {:?}\nAVG FPS: {:?}", reporter.last().0, reporter.last().1).formatted();
                    let vertices = text::vbo::text(&text, &mut glyphs, 18.0, 500.0, 0.0);
                    
                    let ubo = uniform! {
                        mat: Mat4::orthographic_rh_gl(0.0, size.0, 0.0, size.1, -1.0, 1.0).to_cols_array_2d(),
                        sprite: glyphs
                            .sampled()
                            .minify_filter(MinifySamplerFilter::Nearest)
                            .magnify_filter(MagnifySamplerFilter::Nearest)
                    };

                    let vbo = VertexBuffer::new(&display, &vertices[..]).unwrap();
                    let indices = (0..(text2_vbo.len() / 4) as u32)
                        .flat_map(|i| [i * 4, i * 4 + 1, i * 4 + 2, i * 4, i * 4 + 2, i * 4 + 3])
                        .collect::<Vec<u32>>();
                    let ibo = IndexBuffer::new(&display, PrimitiveType::TrianglesList, &indices[..]).unwrap();
                    frame.draw(&vbo, &ibo, &text2_program, &ubo, &params).unwrap();

                    // DRAW GPU TEXT

                    if gpu_toggle {
                        frame.draw(&text2_vbo, &text2_ibo, &text2_program, &ubo, &params).unwrap();
                    }

                    frame.finish().unwrap();
                };
                reporter.end(timer);
                reporter.report_maybe();
            }
            Event::WindowEvent {
                event: WindowEvent::KeyboardInput{
                    input: KeyboardInput {
                        virtual_keycode: Some(keycode),
                        state: ElementState::Pressed,
                        ..
                    },
                    ..
                },
                ..
            } => match keycode {
                VirtualKeyCode::Escape => *control = ControlFlow::Exit,
                VirtualKeyCode::F1 => gpu_toggle = !gpu_toggle,
                VirtualKeyCode::F2 => cpu_toggle = !cpu_toggle,
                VirtualKeyCode::F3 => quad_toggle = !quad_toggle,
                _ => {}
            },
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => *control = ControlFlow::Exit,
            _ => {}
        }
    });
}
