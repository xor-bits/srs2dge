use crate::{
    color::Color,
    prelude::{DefaultVertex, Mesh, TexturePosition},
    util::RemapRange,
};
use glam::{Vec2, Vec3, Vec3Swizzles, Vec4, Vec4Swizzles};
use std::array::IntoIter;
use wgpu::PrimitiveTopology;

//

#[derive(Debug, Clone, Copy, Default)]
pub struct QuadMesh {
    pub pos: Vec2,
    pub size: Vec2,
    pub col: Color,
    pub tex: TexturePosition,
}

#[derive(Debug, Clone, Copy, Default)]
pub struct IsoQuadMesh {
    pub pos: Vec2,
    pub size: Vec2,
    pub col: Color,
    // TODO:
    // pub tex: TexturePosition,
}

//

impl QuadMesh {
    pub fn new_top_left(pos: Vec2, size: Vec2, col: Color, tex: TexturePosition) -> Self {
        Self {
            pos,
            size,
            col,
            tex,
        }
    }

    pub fn new_centered(pos: Vec2, size: Vec2, col: Color, tex: TexturePosition) -> Self {
        Self {
            pos: pos - size * 0.5,
            size,
            col,
            tex,
        }
    }

    /// remove those quads that are outside of the bounds
    ///
    /// and 'cut' those quads that are touching the bounds
    pub fn clip(self, bounds_min: Vec2, bounds_max: Vec2) -> Option<Self> {
        // discard quads outside of the bounding box
        if (self.pos + self.size).cmplt(bounds_min).any() || self.pos.cmpge(bounds_max).any() {
            return None;
        }

        // stretch (squash actually) quads that
        // clip with the bounding box
        //  - left & bottom clamp
        let offset_clamp = self.pos.max(bounds_min);
        let mut size = self.pos - offset_clamp + self.size;
        let pos = offset_clamp;
        //  - right & top clamp
        size = (pos + size).min(bounds_max) - pos;

        // texture position correction
        // to not stretch textures when
        // clamping the quads
        let tex = TexturePosition {
            top_left: pos.remap(
                self.pos..self.pos + self.size,
                self.tex.top_left..self.tex.bottom_right,
            ),
            bottom_right: (pos + size).remap(
                self.pos..self.pos + self.size,
                self.tex.top_left..self.tex.bottom_right,
            ),
        };

        Some(Self::new_top_left(pos, size, self.col, tex))
    }
}

impl Mesh<DefaultVertex> for QuadMesh {
    const PRIM: PrimitiveTopology = PrimitiveTopology::TriangleStrip;

    type VertexIter = IntoIter<DefaultVertex, 4>;
    type IndexIter = IntoIter<u32, 5>;

    fn vertices(&self) -> Self::VertexIter {
        let top_left = self.pos;
        let bottom_right = self.pos + self.size;
        let p = Vec4::new(top_left.x, top_left.y, bottom_right.x, bottom_right.y);
        let c = Vec4::new(
            self.tex.top_left.x,
            self.tex.bottom_right.y,
            self.tex.bottom_right.x,
            self.tex.top_left.y,
        );
        IntoIterator::into_iter([
            DefaultVertex::new(p.xy(), self.col, c.xy()),
            DefaultVertex::new(p.xw(), self.col, c.xw()),
            DefaultVertex::new(p.zy(), self.col, c.zy()),
            DefaultVertex::new(p.zw(), self.col, c.zw()),
        ])
    }

    fn indices(&self, offset: u32) -> Self::IndexIter {
        IntoIterator::into_iter([offset, offset + 1, offset + 2, offset + 3, !0])
    }

    fn index_step(&self) -> u32 {
        4
    }
}

impl Mesh<DefaultVertex> for IsoQuadMesh {
    const PRIM: PrimitiveTopology = PrimitiveTopology::TriangleStrip;

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
        IntoIterator::into_iter([offset, offset + 1, offset + 2, offset + 3, !0])
    }

    fn index_step(&self) -> u32 {
        4
    }
}

//
