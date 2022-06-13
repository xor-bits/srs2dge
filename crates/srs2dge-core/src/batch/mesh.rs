use super::prelude::{DefaultVertex, Vertex};
use wgpu::PrimitiveTopology;

//

pub trait Mesh<V = DefaultVertex> {
    const PRIM: PrimitiveTopology;

    type VertexIter: Iterator<Item = V>;
    type IndexIter: Iterator<Item = u32>;

    fn vertices(&self) -> Self::VertexIter;
    fn indices(&self, offset: u32) -> Self::IndexIter;
    fn index_step(&self) -> u32;
}

pub trait CollectMeshIter<V> {
    fn collect_mesh(self) -> (Vec<V>, Vec<u32>);
}

//

// hehe VIM
impl<V, I, M> CollectMeshIter<V> for I
where
    V: Vertex,
    I: Iterator<Item = M>,
    M: Mesh<V>,
{
    fn collect_mesh(self) -> (Vec<V>, Vec<u32>) {
        collect_mesh_iter(self)
    }
}

pub fn collect_mesh_iter<V, I, M>(iter: I) -> (Vec<V>, Vec<u32>)
where
    V: Vertex,
    I: Iterator<Item = M>,
    M: Mesh<V>,
{
    let mut i = 0;
    let mut vertices = Vec::new();
    let mut indices = Vec::new();

    for mesh in iter {
        let offset = i;
        i += mesh.index_step();

        vertices.extend(mesh.vertices());
        indices.extend(mesh.indices(offset))
    }

    (vertices, indices)
}
