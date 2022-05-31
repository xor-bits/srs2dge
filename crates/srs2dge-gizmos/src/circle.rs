use srs2dge_core::{
    color::Color,
    glam::{Mat4, Vec2},
    prelude::{
        DefaultVertex, Frame, IndexBuffer, Layout, Mesh, RenderPass, UniformBuffer, VertexBuffer,
    },
    target::Target,
    wgpu::{BindGroup, PrimitiveTopology},
};
use srs2dge_presets::LineShader;
use std::f32::consts::PI;

//

const RES: u32 = 50;

//

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct GizmosCircle {
    pub middle: Vec2,
    pub radius: Vec2,
    pub col: Color,
}

//

impl GizmosCircle {
    pub fn new(middle: Vec2, radius: Vec2, col: Color) -> Self {
        Self {
            middle,
            radius,
            col,
        }
    }
}

impl Mesh<DefaultVertex> for GizmosCircle {
    const PRIM: PrimitiveTopology = PrimitiveTopology::LineStrip;

    // TODO : #![feature(type_alias_impl_trait)]
    type VertexIter = GizmosCircleVertexIter;
    type IndexIter = GizmosCircleIndexIter;

    fn vertices(&self) -> Self::VertexIter {
        GizmosCircleVertexIter {
            col: self.col,
            middle: self.middle,
            radius: self.radius,
            i: 0,
        }
    }

    fn indices(&self, offset: u32) -> Self::IndexIter {
        GizmosCircleIndexIter { i: 0, offset }
    }

    fn index_step(&self) -> u32 {
        RES
    }
}

//

pub struct GizmosCircleVertexIter {
    col: Color,
    middle: Vec2,
    radius: Vec2,
    i: u32,
}

impl Iterator for GizmosCircleVertexIter {
    type Item = DefaultVertex;

    fn next(&mut self) -> Option<Self::Item> {
        if self.i >= RES {
            return None;
        }

        let i = self.i;
        self.i += 1;
        let v = i as f32 / RES as f32 * 2.0 * PI;

        Some(DefaultVertex::new(
            self.middle + self.radius * Vec2::new(v.cos(), v.sin()),
            self.col,
            Vec2::ZERO,
        ))
    }
}

pub struct GizmosCircleIndexIter {
    i: u32,
    offset: u32,
}

impl Iterator for GizmosCircleIndexIter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        let i = self.i;
        self.i += 1;

        if i == RES {
            Some(self.offset)
        } else if i == RES + 1 {
            Some(!0)
        } else if i >= RES + 2 {
            None
        } else {
            Some(i + self.offset)
        }
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
        let mut i = 0;
        let ibo_data: Vec<u32> = self
            .circles
            .drain(..)
            .flat_map(|line| {
                let offset = i;
                i += line.index_step();
                line.indices(offset)
            })
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
