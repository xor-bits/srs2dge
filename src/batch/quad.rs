use crate::prelude::{DefaultVertex, Mesh, TexturePosition};
use glam::{Vec2, Vec3, Vec3Swizzles, Vec4, Vec4Swizzles};
use std::array::IntoIter;
use wgpu::PrimitiveTopology;

//

#[derive(Debug, Clone, Copy, Default)]
pub struct QuadMesh {
    pub pos: Vec2,
    pub size: Vec2,
    pub col: Vec4,
    pub tex: TexturePosition,
}

#[derive(Debug, Clone, Copy, Default)]
pub struct IsoQuadMesh {
    pub pos: Vec2,
    pub size: Vec2,
    pub col: Vec4,
    // TODO:
    // pub tex: TexturePosition,
}

//

impl Mesh<DefaultVertex> for QuadMesh {
    const PRIM: PrimitiveTopology = PrimitiveTopology::TriangleStrip;

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
        let c = Vec4::new(
            self.tex.top_left.x,
            self.tex.top_left.y,
            self.tex.bottom_right.x,
            self.tex.bottom_right.y,
        );
        IntoIterator::into_iter([
            DefaultVertex::new(p.xy(), self.col, c.xy()),
            DefaultVertex::new(p.xw(), self.col, c.xw()),
            DefaultVertex::new(p.zy(), self.col, c.zy()),
            DefaultVertex::new(p.zw(), self.col, c.zw()),
        ])
    }

    fn indices(&self, offset: u32) -> Self::IndexIter {
        let offset = offset * 4;
        IntoIterator::into_iter([offset, offset + 1, offset + 2, offset + 3, !0])
        // webgpu doesn't seem to support primitive restart
        /* IntoIterator::into_iter([
            offset,
            offset + 1,
            offset + 2,
            offset + 2,
            offset + 1,
            offset + 3,
        ]) */
    }
}

impl Mesh<DefaultVertex> for IsoQuadMesh {
    const PRIM: PrimitiveTopology = PrimitiveTopology::TriangleStrip;

    const VERTICES: usize = 4;
    const INDICES: usize = 5;

    type VertexIter = IntoIter<DefaultVertex, 4>;
    type IndexIter = IntoIter<u32, 5>;

    fn vertices(&self) -> Self::VertexIter {
        let c = Vec3::new(0.0, 0.5, 1.0);
        IntoIterator::into_iter([
            DefaultVertex::new(self.pos + self.size * c.xy(), self.col, c.xx()),
            DefaultVertex::new(self.pos + self.size * c.yz(), self.col, c.xz()),
            DefaultVertex::new(self.pos + self.size * c.yx(), self.col, c.zx()),
            DefaultVertex::new(self.pos + self.size * c.zy(), self.col, c.zz()),
        ])
    }

    fn indices(&self, offset: u32) -> Self::IndexIter {
        let offset = offset * 4;
        IntoIterator::into_iter([offset, offset + 1, offset + 2, offset + 3, !0])
        // webgpu doesn't seem to support primitive restart
        /* IntoIterator::into_iter([
            offset,
            offset + 1,
            offset + 2,
            offset + 2,
            offset + 1,
            offset + 3,
        ]) */
    }
}
