use super::geom::GuiGeom;
use srs2dge_core::{
    batch::Mesh,
    buffer::{IndexBuffer, VertexBuffer},
    prelude::Frame,
    target::Target,
};

//

#[derive(Debug, Default)]
pub struct GuiRenderer {
    pub vbo: Option<VertexBuffer>,
    pub ibo: Option<IndexBuffer>,

    pub geometry: Vec<GuiGeom>,
}

//

impl GuiRenderer {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn push_with(&mut self, geometry: GuiGeom) {
        self.geometry.push(geometry)
    }

    pub fn generate(
        &mut self,
        target: &mut Target,
        frame: &mut Frame,
    ) -> (&VertexBuffer, &IndexBuffer, u32) {
        let mut vertices = vec![];
        let mut indices = vec![];

        let mut i = 0;
        for geom in self.geometry.drain(..) {
            vertices.extend(geom.vertices());
            indices.extend(geom.indices(i));
            i += geom.index_step();
        }

        let vbo_cap = self.vbo.as_ref().map(|vbo| vbo.capacity());
        let ibo_cap = self.ibo.as_ref().map(|ibo| ibo.capacity());
        if vbo_cap < Some(vertices.len()) {
            self.vbo = Some(VertexBuffer::new(target, vertices.len() * 2));
        }
        if ibo_cap < Some(indices.len()) {
            self.ibo = Some(IndexBuffer::new(target, indices.len() * 2));
        }

        let vbo = self.vbo.as_mut().unwrap();
        let ibo = self.ibo.as_mut().unwrap();

        vbo.upload(target, frame, &vertices);
        ibo.upload(target, frame, &indices);

        (vbo, ibo, indices.len() as _)
    }
}
