#[macro_use]
extern crate glium;

use crate::report::Reporter;
use glam::{Mat2, Vec2};
use glium::{
    glutin::ContextBuilder,
    index::{NoIndices, PrimitiveType},
    uniforms::{EmptyUniforms, UniformBuffer},
    Surface, VertexBuffer,
};
use std::time::Duration;
use winit::{
    dpi::LogicalSize,
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

mod report;

#[derive(Debug, Clone, Copy)]
struct Vertex {
    vi_position: [f32; 2],
    vi_color: [f32; 3],
}
glium::implement_vertex!(Vertex, vi_position, vi_color);

#[derive(Debug, Clone, Copy)]
struct Ubo {
    nat: Mat2,
}

fn main() {
    env_logger::init();

    let event_loop = EventLoop::new();
    let window_builder = WindowBuilder::new()
        .with_inner_size(LogicalSize::new(600, 600))
        .with_title("Title");
    let context = ContextBuilder::new()
        .build_windowed(window_builder, &event_loop)
        .unwrap();
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

                uniform mat2 mat;

                out vec3 fi_color;

                void main() {
                    gl_Position = vec4(mat * vi_position, 0.0, 1.0);
                    fi_color = vi_color;
                }
            ",
            fragment: "
                #version 140

                in vec3 fi_color;

                out vec4 o_color;

                void main() {
                    o_color = vec4(fi_color, 1.0);
                }
            "
        }
    )
    .unwrap();

    let vbo = VertexBuffer::new(
        &display,
        &[
            Vertex {
                vi_position: [-0.5, -0.5],
                vi_color: [1.0, 0.0, 0.0],
            },
            Vertex {
                vi_position: [0.0, 0.5],
                vi_color: [0.0, 1.0, 0.0],
            },
            Vertex {
                vi_position: [0.5, -0.5],
                vi_color: [0.0, 0.0, 1.0],
            },
        ],
    )
    .unwrap();

    let ibo = NoIndices(PrimitiveType::TrianglesList);
    let ubo = UniformBuffer::new(&display, Mat2::IDENTITY).unwrap();
    // let ubo = EmptyUniforms;

    let mut a = 0.0;

    let mut reporter = Reporter::new_with_interval(Duration::from_millis(500));

    event_loop.run(move |event, _, control| {
        *control = ControlFlow::Poll;

        match event {
            Event::RedrawEventsCleared => {
                let timer = reporter.begin();
                {
                    a += 0.001;
                    let ubo = uniform! {
                        mat: Mat2::from_angle(a).to_cols_array_2d()
                    };

                    let mut frame = display.draw();
                    frame.clear_color(0.2, 0.3, 0.4, 1.0);
                    frame
                        .draw(&vbo, &ibo, &shader, &ubo, &Default::default())
                        .unwrap();
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
