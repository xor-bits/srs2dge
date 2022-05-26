use srs2dge_core::{
    color::Color,
    glam::{Mat4, Vec2},
    prelude::{DefaultVertex, IndexBuffer, Layout, Mesh, RenderPass, UniformBuffer, VertexBuffer},
    target::Target,
    wgpu::{BindGroup, PrimitiveTopology},
    Frame,
};
use srs2dge_presets::LineShader;
use std::{f32::consts::PI, ops::Rem};

//

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct GizmosBox {
    pub middle: Vec2,
    pub radius: Vec2,
    pub col: Color,
}

//

impl GizmosBox {
    pub fn new(middle: Vec2, radius: Vec2, col: Color) -> Self {
        Self {
            middle,
            radius,
            col,
        }
    }
}

impl Mesh<DefaultVertex> for GizmosBox {
    const PRIM: PrimitiveTopology = PrimitiveTopology::LineStrip;

    // TODO : #![feature(type_alias_impl_trait)]
    type VertexIter = GizmosBoxVertexIter;
    type IndexIter = GizmosBoxIndexIter;

    fn vertices(&self) -> Self::VertexIter {
        GizmosBoxVertexIter {
            col: self.col,
            middle: self.middle,
            radius: self.radius,
            i: 0,
        }
    }

    fn indices(&self, offset: u32) -> Self::IndexIter {
        GizmosBoxIndexIter { i: 0, offset }
    }

    fn index_step(&self) -> u32 {
        4
    }
}

//

pub struct GizmosBoxVertexIter {
    col: Color,
    middle: Vec2,
    radius: Vec2,
    i: i32,
}

impl Iterator for GizmosBoxVertexIter {
    type Item = DefaultVertex;

    fn next(&mut self) -> Option<Self::Item> {
        if self.i >= 4 {
            return None;
        }

        let i = self.i;
        self.i += 1;
        let v = i as f32 / 4.0 * 2.0 * PI;

        Some(DefaultVertex::new(
            self.middle + self.radius * Vec2::new(square_cos(v), square_sin(v)),
            self.col,
            Vec2::ZERO,
        ))
    }
}

pub struct GizmosBoxIndexIter {
    i: u32,
    offset: u32,
}

impl Iterator for GizmosBoxIndexIter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        let i = self.i;
        self.i += 1;

        if i == 4 {
            Some(self.offset)
        } else if i == 5 {
            Some(!0)
        } else if i >= 6 {
            None
        } else {
            Some(i + self.offset)
        }
    }
}

// https://www.desmos.com/calculator/ceexqanvmb
fn square_cos(f: f32) -> f32 {
    square_sin(f + 0.5 * PI)
}

fn square_sin(f: f32) -> f32 {
    1.0 - (f / PI).floor().rem(2.0) * 2.0
}

//

pub(super) struct GizmosBoxes {
    boxes: Vec<GizmosBox>,

    vbo: VertexBuffer,
    ibo: IndexBuffer,
    ibo_len: u32,
    shader: LineShader,
    bind_group: BindGroup,
}

//

impl GizmosBoxes {
    pub fn new(target: &Target, ubo: &UniformBuffer<Mat4>) -> Self {
        let shader = LineShader::new(target, true);
        let bind_group = shader.bind_group(ubo);

        Self {
            boxes: vec![],

            vbo: VertexBuffer::new(target, 4),
            ibo: IndexBuffer::new(target, 6),
            ibo_len: 0,
            shader,
            bind_group,
        }
    }

    pub fn push(&mut self, r#box: GizmosBox) {
        self.boxes.push(r#box);
    }

    pub fn prepare(&mut self, target: &mut Target, frame: &mut Frame) {
        let vbo_data: Vec<DefaultVertex> =
            self.boxes.iter().flat_map(|line| line.vertices()).collect();
        let mut i = 0;
        let ibo_data: Vec<u32> = self
            .boxes
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
