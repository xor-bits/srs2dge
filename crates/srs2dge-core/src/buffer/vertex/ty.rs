use crate::color::Color;
use bytemuck::{Pod, Zeroable};
use glam::Vec2;
use wgpu::{VertexAttribute, VertexBufferLayout, VertexFormat, VertexStepMode};

//

#[derive(Debug, Clone, Copy, PartialEq, Default, Zeroable, Pod)]
#[repr(C)]
pub struct DefaultVertex {
    pos: Vec2,
    uv: Vec2,
    col: Color,
}

//

pub trait Vertex: Pod {
    const LAYOUT: &'static [VertexBufferLayout<'static>];
}

//

impl DefaultVertex {
    pub fn new(pos: Vec2, col: Color, uv: Vec2) -> Self {
        Self { pos, uv, col }
    }

    pub fn from_arrays(pos: [f32; 2], col: [f32; 4], uv: [f32; 2]) -> Self {
        Self {
            pos: pos.into(),
            uv: uv.into(),
            col: col.into(),
        }
    }

    pub fn pos(&self) -> Vec2 {
        self.pos
    }

    pub fn uv(&self) -> Vec2 {
        self.uv
    }

    pub fn col(&self) -> Color {
        self.col
    }
}

impl Vertex for DefaultVertex {
    const LAYOUT: &'static [VertexBufferLayout<'static>] = &[VertexBufferLayout {
        array_stride: 32,
        step_mode: VertexStepMode::Vertex,
        attributes: &[
            VertexAttribute {
                format: VertexFormat::Float32x2,
                offset: 0,
                shader_location: 0,
            },
            VertexAttribute {
                format: VertexFormat::Float32x2,
                offset: 8,
                shader_location: 1,
            },
            VertexAttribute {
                format: VertexFormat::Float32x4,
                offset: 16,
                shader_location: 2,
            },
        ],
    }];
}

//

impl Vertex for () {
    const LAYOUT: &'static [VertexBufferLayout<'static>] = &[];
}

// impl<T> Vertex for T where T: Pod {}
