// #![feature(let_else)]
// #![feature(drain_filter)]
// #![feature(destructuring_assignment)]
// #![feature(test)]

use crate::report::Reporter;
use fontdue::{Font, Metrics};
use glam::{Mat4, Vec4};
use glium::{
    glutin::ContextBuilder,
    index::PrimitiveType,
    uniforms::{MagnifySamplerFilter, MinifySamplerFilter},
    Blend, DrawParameters, IndexBuffer, Surface, VertexBuffer,
};
use image::{buffer::ConvertBuffer, imageops::flip_vertical_in_place, GrayImage, Luma};
use image::{ImageFormat, RgbaImage};
use rusttype::{Point, Scale};
use static_res::static_res;
use std::{io::Cursor, time::Duration};
use winit::{
    dpi::LogicalSize,
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

#[macro_use]
extern crate glium;

mod packer;
mod report;

static_res! { "res/**/*.{png,ttf}" }

#[derive(Debug, Clone, Copy)]
struct Vertex {
    vi_position: [f32; 2],
    vi_color: [f32; 3],
    vi_uv: [f32; 2],
}
glium::implement_vertex!(Vertex, vi_position, vi_color, vi_uv);

fn character(c: char, font: &Font, px: f32) -> (Metrics, Vec<u8>) {
    font.rasterize(c, px)
}

fn text(s: &str, font: &Font, px: f32) -> Option<GrayImage> {
    let chars: Vec<(char, (Metrics, Vec<u8>))> =
        s.chars().map(|c| (c, character(c, font, px))).collect();

    // text bounding box
    let mut x_min = 0;
    let mut x_max = 0;
    let mut y_min = 0;
    let mut y_max = 0;
    let mut x_origin = 0;
    let mut y_origin = 0;
    let mut last_c = None;
    for (c, (metrics, _)) in chars.iter() {
        if *c == '\n' {
            x_origin = 0;
            y_origin -= (px * 1.4) as i32;
            last_c = None;
            continue;
        };

        x_min = x_min.min(x_origin + metrics.xmin);
        y_min = y_min.min(y_origin + metrics.ymin);

        x_max = x_max.max(x_origin + metrics.xmin + metrics.width as i32);
        y_max = y_max.max(y_origin + metrics.ymin + metrics.height as i32);

        x_origin += metrics.advance_width as i32
            + last_c
                .and_then(|last_c| font.horizontal_kern(last_c, *c, px))
                .unwrap_or(0.0) as i32;
        y_origin += metrics.advance_height as i32;
        last_c = Some(*c);
    }
    let width = (x_max - x_min).max(0) as usize;
    let height = (y_max - y_min).max(0) as usize;

    x_origin = 0;
    y_origin = 0;
    let mut text = vec![0; width * height];
    for (c, (metrics, bitmap)) in chars.iter() {
        if *c == '\n' {
            x_origin = 0;
            y_origin -= (px * 1.4) as i32;
            last_c = None;
            continue;
        };

        for (index, pixel) in bitmap.iter().enumerate() {
            let x = x_origin + metrics.xmin - x_min + (index % metrics.width) as i32;
            let y = y_origin + metrics.ymin - y_min
                + (metrics.height - 1 - (index / metrics.width)) as i32;
            text[x as usize + y as usize * width] = *pixel;
        }

        x_origin += metrics.advance_width as i32
            + last_c
                .and_then(|last_c| font.horizontal_kern(last_c, *c, px))
                .unwrap_or(0.0) as i32;
        y_origin += metrics.advance_height as i32;
        last_c = Some(*c);
    }

    GrayImage::from_raw(width as u32, height as u32, text)
}

fn main() {
    env_logger::init();

    let font = rusttype::Font::try_from_bytes(res::Roboto_Regular_ttf).unwrap();
    let mut cache = rusttype::gpu_cache::CacheBuilder::default()
        .dimensions(2048, 2048)
        .build();
    let mut image = GrayImage::new(2048, 2048);
    for c in '\u{0}'..'\u{800}' {
        cache.queue_glyph(
            0,
            font.glyph(c)
                .scaled(Scale::uniform(64.0))
                .positioned(Point::default()),
        );
        cache
            .cache_queued(|rect, data| {
                let mut i = 0;
                for y in rect.min.y..rect.max.y {
                    for x in rect.min.x..rect.max.x {
                        image.put_pixel(x as u32, y as u32, Luma { 0: [data[i]] });
                        i += 1;
                    }
                }
            })
            .unwrap();
    }
    image.save("path.png").unwrap();
    return;

    let font = Font::from_bytes(res::Roboto_Regular_ttf, Default::default()).unwrap();

    /* let mut area = 0;
    for c in 0..font.glyph_count() {
        let c = font.metrics_indexed(c, 32.0);
        area += c.width * c.height;
    }
    let side = (area as f64 * 1.2).sqrt() as i32;
    let mut packer = Packer::new(Config {
        width: side,
        height: side,
        border_padding: 0,
        rectangle_padding: 0,
    });
    let mut image = GrayImage::new(side as u32, side as u32);
    for c in 0..font.glyph_count() {
        let (rect, c) = font.rasterize_indexed(c, 32.0);
        let Some(rect) = packer.pack(rect.width as i32, rect.height as i32, false) else {
            continue;
        };
        let mut i = 0;
        for y in rect.y..rect.y + rect.height {
            for x in rect.x..rect.x + rect.width {
                image.put_pixel(x as u32, y as u32, Luma { 0: [c[i]] });
                i += 1;
            }
        }
    }
    image.save("path.png").unwrap();
    panic! {"{}", font.glyph_count()}; */

    let mut text_img: RgbaImage = text(
        "∫|∫x dx + 'test text j'\u{FF1B}\\/\"\n\\VAW//\n readability\n  line height",
        &font,
        64.0,
    )
    .unwrap()
    .convert();

    text_img.pixels_mut().for_each(|px| {
        px.0[3] = px.0[0];
        px.0[0] = !0;
        px.0[1] = !0;
        px.0[2] = !0;
    });

    flip_vertical_in_place(&mut text_img);

    // RENDERING

    let event_loop = EventLoop::new();
    let window_builder = WindowBuilder::new()
        .with_inner_size(LogicalSize::new(600, 600))
        .with_title("Title");
    let context = ContextBuilder::new()
        .with_vsync(true)
        .build_windowed(window_builder, &event_loop)
        .unwrap();
    let scale_factor = context.window().scale_factor();
    let display = glium::Display::from_gl_window(context).unwrap();

    log::debug!("OpenGL Vendor: {}", display.get_opengl_vendor_string());
    log::debug!("OpenGL Renderer: {}", display.get_opengl_renderer_string());
    log::debug!("OpenGL Version: {}", display.get_opengl_version_string());

    let shader = glium::program!(&display,
        140 => {
            vertex: "
                #version 140

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
            fragment: "
                #version 140

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

    let text_dim = text_img.dimensions();
    let text_texture = glium::texture::RawImage2d::from_raw_rgba_reversed(&text_img, text_dim);
    let text_texture =
        glium::texture::CompressedSrgbTexture2d::new(&display, text_texture).unwrap();

    let text_vbo = VertexBuffer::new(
        &display,
        &[
            Vertex {
                vi_position: [0.0, 0.0],
                vi_color: [1.0, 1.0, 1.0],
                vi_uv: [0.0, 1.0],
            },
            Vertex {
                vi_position: [0.0, text_dim.1 as f32],
                vi_color: [1.0, 1.0, 1.0],
                vi_uv: [0.0, 0.0],
            },
            Vertex {
                vi_position: [text_dim.0 as f32, text_dim.1 as f32],
                vi_color: [1.0, 1.0, 1.0],
                vi_uv: [1.0, 0.0],
            },
            Vertex {
                vi_position: [text_dim.0 as f32, 0.0],
                vi_color: [1.0, 1.0, 1.0],
                vi_uv: [1.0, 1.0],
            },
        ],
    )
    .unwrap();

    let mut a = 0.0;
    let mut aspect = 0.0;
    let mut size = (0.0, 0.0);
    let mut reporter = Reporter::new_with_interval(Duration::from_millis(500));

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
                a += reporter.last().as_secs_f32() * std::f32::consts::PI * 2.0 / 5.0;

                let ubo = uniform! {
                    mat: (Mat4::from_diagonal(Vec4::new(1.0, aspect, 1.0, 1.0)) * Mat4::from_rotation_z(a)).to_cols_array_2d(),
                    sprite: texture
                        .sampled()
                        .anisotropy(4)
                        .minify_filter(MinifySamplerFilter::Nearest)
                        .magnify_filter(MagnifySamplerFilter::Nearest),
                };

                let params = DrawParameters {
                    blend: Blend::alpha_blending(),
                    ..Default::default()
                };

                let mut frame = display.draw();
                let cc = Vec4::new(0.2, 0.22, 0.24, 1.0);
                frame.clear_color_srgb(cc.x, cc.y, cc.z, cc.w);
                frame.draw(&vbo, &ibo, &shader, &ubo, &params).unwrap();

                let ubo = uniform! {
                    mat: Mat4::orthographic_rh_gl(0.0, size.0, 0.0, size.1, -1.0, 1.0).to_cols_array_2d(),
                    sprite: text_texture
                        .sampled()
                        .anisotropy(4)
                        .minify_filter(MinifySamplerFilter::Nearest)
                        .magnify_filter(MagnifySamplerFilter::Nearest)
                };

                frame.draw(&text_vbo, &ibo, &shader, &ubo, &params).unwrap();

                frame.finish().unwrap();
            };
            reporter.end(timer);
            reporter.report_maybe();
        }
        Event::WindowEvent {
            event: WindowEvent::CloseRequested,
            ..
        } => *control = ControlFlow::Exit,
        _ => {}
    }
});
}
