use srs2dge_core::{
    batch::mesh::Mesh,
    buffer::{DefaultIndex, DefaultVertex},
    prelude::QuadMesh,
    wgpu::PrimitiveTopology,
};

//

#[derive(Debug, Clone, Copy)]
pub enum GuiGeom {
    Quad(QuadMesh),
}

//

impl Mesh for GuiGeom {
    const PRIM: PrimitiveTopology = PrimitiveTopology::TriangleStrip;

    type VertexIter = IterEnum<<QuadMesh as Mesh>::VertexIter, DefaultVertex>;
    type IndexIter = IterEnum<<QuadMesh as Mesh>::IndexIter, DefaultIndex>;

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
