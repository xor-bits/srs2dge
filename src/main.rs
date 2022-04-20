#![feature(explicit_generic_args_with_impl_trait)]

//

use srs2dge::{prelude::*, shader::presets::Colored2DShader};

//

struct App {
    vbo: VertexBuffer<[f32; 6]>,
    ubo: UniformBuffer<[[f32; 4]; 4]>,
    shader: Colored2DShader,
}

impl Runnable<Engine> for App {
    fn init(gl: &mut GameLoop) -> Self {
        Self {
            vbo: VertexBuffer::new_with(
                gl,
                &[
                    [-0.5, -0.5, 1.0, 0.0, 0.0, 1.0],
                    [0.5, -0.5, 0.0, 1.0, 0.0, 1.0],
                    [0.0, 0.5, 0.0, 0.0, 1.0, 1.0],
                ],
            ),
            ubo: UniformBuffer::new_with(
                gl,
                &[[
                    [1.0, 0.0, 0.0, 0.0],
                    [0.0, 1.0, 0.0, 0.0],
                    [0.0, 0.0, 1.0, 0.0],
                    [0.0, 0.0, 0.0, 1.0],
                ]],
            ),
            shader: Colored2DShader::new(gl),
        }
    }

    fn draw(&mut self, _: &mut GameLoop, frame: &mut Frame, _: f32) {
        frame
            .main_render_pass()
            .bind_vbo(&self.vbo, 0)
            .bind_group(&self.shader.bind_group(&self.ubo))
            .bind_shader(&self.shader)
            .draw(0..3, 0..1);
    }
}

//

fn main() {
    env_logger::init();

    WindowBuilder::new()
        .build_engine()
        .build_game_loop()
        .run::<App>();
}
