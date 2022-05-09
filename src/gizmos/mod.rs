//! Immediate mode rendering

//

use glam::Mat4;

use self::{line::GizmosLine, vert::GizmosVertex};
use crate::{
    prelude::{IndexBuffer, Layout, LineShader, Mesh, RenderPass, UniformBuffer, VertexBuffer},
    target::Target,
    Frame,
};

//

pub mod line;
pub mod vert;

//

pub struct Gizmos {
    lines: Vec<GizmosLine>,

    line_vbo: VertexBuffer<GizmosVertex>,
    line_ibo: IndexBuffer,
    line_ibo_len: u32,
    line_shader: LineShader<GizmosVertex>,
}

//

impl Gizmos {
    pub fn new(target: &Target) -> Self {
        Self {
            lines: vec![],

            line_vbo: VertexBuffer::new(target, 80),
            line_ibo: IndexBuffer::new(target, 80),
            line_ibo_len: 0,
            line_shader: LineShader::new(target),
        }
    }

    pub fn add_line(&mut self, line: GizmosLine) {
        self.lines.push(line);
    }

    pub fn prepare(&mut self, target: &mut Target, frame: &mut Frame) {
        let vbo_data: Vec<GizmosVertex> =
            self.lines.iter().flat_map(|line| line.vertices()).collect();
        let ibo_data: Vec<u32> = self
            .lines
            .iter()
            .enumerate()
            .flat_map(|(i, line)| line.indices(i as _))
            .collect();

        self.line_vbo.upload(target, frame, &vbo_data);
        self.line_ibo.upload(target, frame, &ibo_data);
        self.line_ibo_len = ibo_data.len() as _;
    }

    pub fn draw<'e, Sv, Bv, Si, Bi, const PIPELINE_BOUND: bool>(
        &self,
        render_pass: RenderPass<'e, Sv, Bv, Si, Bi, PIPELINE_BOUND>,
        ubo: &'e UniformBuffer<Mat4>,
    ) -> RenderPass<'e, GizmosVertex, GizmosVertex, u32, u32, true> {
        render_pass
            .bind_ibo(&self.line_ibo)
            .bind_vbo(&self.line_vbo)
            .bind_group(&self.line_shader.bind_group(ubo))
            .bind_shader(&self.line_shader)
    }
}
