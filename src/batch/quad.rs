use crate::program::DefaultVertex;

use super::Mesh;
use glam::{Vec2, Vec2Swizzles, Vec3, Vec3Swizzles, Vec4, Vec4Swizzles};
use glium::index::PrimitiveType;
use std::array::IntoIter;

//

#[derive(Debug, Clone, Copy, Default)]
pub struct QuadMesh {
    pub pos: Vec2,
    pub size: Vec2,
    pub col: Vec4,
}

#[derive(Debug, Clone, Copy, Default)]
pub struct IsoQuadMesh {
    pub pos: Vec2,
    pub size: Vec2,
    pub col: Vec4,
}

//

impl Mesh<DefaultVertex> for QuadMesh {
    const PRIM: PrimitiveType = PrimitiveType::TriangleStrip;

    const VERTICES: usize = 4;
    const INDICES: usize = 5;

    type VertexIter = IntoIter<DefaultVertex, 4>;
    type IndexIter = IntoIter<u32, 5>;

    fn vertices(&self) -> Self::VertexIter {
        let p = Vec4::new(
            self.pos.x,
            self.pos.y,
            self.pos.x + self.size.x,
            self.pos.y + self.size.y,
        );
        let c = Vec2::new(0.0, 1.0);
        IntoIterator::into_iter([
            DefaultVertex::from_vecs(p.xy(), self.col, c.xx()),
            DefaultVertex::from_vecs(p.xw(), self.col, c.xy()),
            DefaultVertex::from_vecs(p.zy(), self.col, c.yx()),
            DefaultVertex::from_vecs(p.zw(), self.col, c.yy()),
        ])
    }

    fn indices(&self, offset: u32) -> Self::IndexIter {
        IntoIterator::into_iter([
            offset * 4,
            offset * 4 + 1,
            offset * 4 + 2,
            offset * 4 + 3,
            !0,
        ])
    }
}

impl Mesh<DefaultVertex> for IsoQuadMesh {
    const PRIM: PrimitiveType = PrimitiveType::TriangleStrip;

    const VERTICES: usize = 4;
    const INDICES: usize = 5;

    type VertexIter = IntoIter<DefaultVertex, 4>;
    type IndexIter = IntoIter<u32, 5>;

    fn vertices(&self) -> Self::VertexIter {
        let c = Vec3::new(0.0, 0.5, 1.0);
        IntoIterator::into_iter([
            DefaultVertex::from_vecs(self.pos + self.size * c.xy(), self.col, c.xx()),
            DefaultVertex::from_vecs(self.pos + self.size * c.yz(), self.col, c.xz()),
            DefaultVertex::from_vecs(self.pos + self.size * c.yx(), self.col, c.zx()),
            DefaultVertex::from_vecs(self.pos + self.size * c.zy(), self.col, c.zz()),
        ])
    }

    fn indices(&self, offset: u32) -> Self::IndexIter {
        IntoIterator::into_iter([offset, offset + 1, offset + 2, offset + 3, !0])
    }
}
