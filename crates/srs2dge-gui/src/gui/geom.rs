use srs2dge_core::{
    batch::mesh::Mesh,
    buffer::{DefaultIndex, DefaultVertex},
    color::Color,
    glam::{Vec2, Vec4, Vec4Swizzles},
    prelude::TexturePosition,
    wgpu::PrimitiveTopology,
};
use std::array::IntoIter;

//

#[derive(Debug, Clone, Copy, Default)]
pub struct GuiQuad {
    pub pos: Vec2,
    pub size: Vec2,
    pub col: Color,
    pub tex: TexturePosition,
}

#[derive(Debug, Clone, Copy)]
pub enum GuiGeom {
    Quad(GuiQuad),
}

//

impl Mesh<DefaultVertex> for GuiQuad {
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

impl Mesh for GuiGeom {
    const PRIM: PrimitiveTopology = PrimitiveTopology::TriangleStrip;

    type VertexIter = IterEnum<<GuiQuad as Mesh>::VertexIter, DefaultVertex>;
    type IndexIter = IterEnum<<GuiQuad as Mesh>::IndexIter, DefaultIndex>;

    fn vertices(&self) -> Self::VertexIter {
        match self {
            GuiGeom::Quad(m) => IterEnum::A(m.vertices()),
        }
    }

    fn indices(&self, offset: u32) -> Self::IndexIter {
        match self {
            GuiGeom::Quad(m) => IterEnum::A(m.indices(offset)),
        }
    }

    fn index_step(&self) -> u32 {
        match self {
            GuiGeom::Quad(m) => m.index_step(),
        }
    }
}

//

pub enum IterEnum<A, /* B, C, */ I>
where
    A: Iterator<Item = I>,
    // B: Iterator<Item = I>,
    // C: Iterator<Item = I>,
{
    A(A),
    // B(B),
    // C(C),
}

impl<A, /* B, C, */ I> Iterator for IterEnum<A, /* B, C, */ I>
where
    A: Iterator<Item = I>,
    // B: Iterator<Item = I>,
    // C: Iterator<Item = I>,
{
    type Item = I;

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            IterEnum::A(a) => a.next(),
            // IterEnum::B(b) => b.next(),
            // IterEnum::C(c) => c.next(),
        }
    }
}
