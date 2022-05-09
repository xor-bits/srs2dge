use crate::prelude::Vertex;
use bytemuck::{Pod, Zeroable};
use glam::{Vec2, Vec3, Vec4};
use wgpu::{VertexAttribute, VertexBufferLayout, VertexFormat, VertexStepMode};

//

#[derive(Debug, Clone, Copy, PartialEq, Default, Zeroable, Pod)]
#[repr(C)]
pub struct GizmosVertex {
    pub pos: Vec2,
    pub col: Vec3,
}

//

impl Vertex for GizmosVertex {
    const LAYOUT: &'static [VertexBufferLayout<'static>] = &[VertexBufferLayout {
        array_stride: std::mem::size_of::<Self>() as u64,
        step_mode: VertexStepMode::Vertex,
        attributes: &[
            VertexAttribute {
                format: VertexFormat::Float16x2,
                offset: 0,
                shader_location: 0,
            },
            VertexAttribute {
                format: VertexFormat::Float16x4,
                offset: std::mem::size_of::<Vec2>() as u64,
                shader_location: 1,
            },
        ],
    }];
}
