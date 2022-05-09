use std::f32::consts::PI;

use crate::{
    prelude::{
        DefaultVertex, IndexBuffer, Layout, LineShader, Mesh, RenderPass, UniformBuffer,
        VertexBuffer,
    },
    target::Target,
    Frame,
};
use glam::{Mat4, Vec2, Vec4};
use wgpu::{BindGroup, PrimitiveTopology};

//

const RES: u32 = 50;

//

pub struct GizmosCircle {
    pub middle: Vec2,
    pub radius: f32,
    pub col: Vec4,
}

//

impl GizmosCircle {
    pub fn new(middle: Vec2, radius: f32, col: Vec4) -> Self {
        Self {
            middle,
            radius,
            col,
        }
    }
}

impl Mesh<DefaultVertex> for GizmosCircle {
    const PRIM: PrimitiveTopology = PrimitiveTopology::LineStrip;

    const VERTICES: usize = RES as usize;
    const INDICES: usize = RES as usize + 2;

    type VertexIter = impl Iterator<Item = DefaultVertex>;
    type IndexIter = impl Iterator<Item = u32>;

    fn vertices(&self) -> Self::VertexIter {
        let col = self.col;
        let middle = self.middle;
        let radius = self.radius;
        (0..RES)
            .map(|i| i as f32 / RES as f32 * 2.0 * PI)
            .map(move |v| {
                DefaultVertex::new(
                    middle + radius * Vec2::new(v.cos(), v.sin()),
                    col,
                    Vec2::ZERO,
                )
            })
    }

    fn indices(&self, offset: u32) -> Self::IndexIter {
        let offset = offset * Self::VERTICES as u32;
        (0..RES).chain([0]).map(move |i| i + offset).chain([!0])
    }
}

//

pub(super) struct GizmosCircles {
    circles: Vec<GizmosCircle>,

    vbo: VertexBuffer,
    ibo: IndexBuffer,
    ibo_len: u32,
    shader: LineShader,
    bind_group: BindGroup,
}

//

impl GizmosCircles {
    pub fn new(target: &Target, ubo: &UniformBuffer<Mat4>) -> Self {
        let shader = LineShader::new(target, true);
        let bind_group = shader.bind_group(ubo);

        Self {
            circles: vec![],

            vbo: VertexBuffer::new(target, RES as usize),
            ibo: IndexBuffer::new(target, RES as usize + 2),
            ibo_len: 0,
            shader,
            bind_group,
        }
    }

    pub fn push(&mut self, circle: GizmosCircle) {
        self.circles.push(circle);
    }

    pub fn prepare(&mut self, target: &mut Target, frame: &mut Frame) {
        let vbo_data: Vec<DefaultVertex> = self
            .circles
            .iter()
            .flat_map(|line| line.vertices())
            .collect();
        let ibo_data: Vec<u32> = self
            .circles
            .drain(..)
            .enumerate()
            .flat_map(|(i, line)| line.indices(i as _))
            .collect();

        if self.vbo.capacity() < vbo_data.len() {
            self.vbo = VertexBuffer::new(target, vbo_data.len() * 2);
        }
        if self.ibo.capacity() < ibo_data.len() {
            self.ibo = IndexBuffer::new(target, ibo_data.len() * 2);
        }
        self.vbo.upload(target, frame, &vbo_data);
        self.ibo.upload(target, frame, &ibo_data);
        self.ibo_len = ibo_data.len() as _;
    }

    pub fn draw<'e, Sv, Bv, Si, Bi, const PIPELINE_BOUND: bool>(
        &'e self,
        render_pass: RenderPass<'e, Sv, Bv, Si, Bi, PIPELINE_BOUND>,
    ) -> RenderPass<'e> {
        render_pass
            .bind_ibo(&self.ibo)
            .bind_vbo(&self.vbo)
            .bind_group(&self.bind_group)
            .bind_shader(&self.shader)
            .draw_indexed(0..self.ibo_len, 0, 0..1)
            .done()
    }
}
