use super::vert::GizmosVertex;
use crate::prelude::Mesh;
use glam::{Vec2, Vec3, Vec4};
use std::array::IntoIter;
use wgpu::PrimitiveTopology;

//

pub struct GizmosLine {
    pub from: Vec2,
    pub to: Vec2,
}

//

impl GizmosLine {
    pub fn new(from: Vec2, to: Vec2) -> Self {
        Self { from, to }
    }
}

impl Mesh<GizmosVertex> for GizmosLine {
    const PRIM: PrimitiveTopology = PrimitiveTopology::TriangleList;

    const VERTICES: usize = 2;
    const INDICES: usize = 2;

    type VertexIter = IntoIter<GizmosVertex, 2>;
    type IndexIter = IntoIter<u32, 2>;

    fn vertices(&self) -> Self::VertexIter {
        [
            GizmosVertex {
                pos: self.from,
                col: Vec3::new(1.0, 1.0, 1.0),
                ..Default::default()
            },
            GizmosVertex {
                pos: self.to,
                col: Vec3::new(1.0, 1.0, 1.0),
                ..Default::default()
            },
        ]
        .into_iter()
    }

    fn indices(&self, offset: u32) -> Self::IndexIter {
        let offset = offset * Self::VERTICES as u32;
        IntoIterator::into_iter([offset, offset + 1])
    }
}
