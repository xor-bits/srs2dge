use self::quad::QuadMesh;
use crate::{
    buffer::{
        vertex::{DefaultVertex, Vertex},
        IndexBuffer, VertexBuffer,
    },
    target::Target,
    Frame,
};
use std::{
    collections::{BinaryHeap, HashSet},
    marker::PhantomData,
};

//

use serde::{Deserialize, Serialize};
pub use wgpu::PrimitiveTopology;

//

pub mod prelude;
pub mod quad;

//

#[derive(Debug)]
pub struct BatchRenderer<M = QuadMesh, V = DefaultVertex>
where
    M: Mesh<V>,
    V: Vertex + Copy,
{
    vbo: VertexBuffer<V>,
    ibo: IndexBuffer<u32>,
    ibo_regen: bool,

    max: usize,
    modified: HashSet<usize>,
    free: BinaryHeap<usize>,
    used: Vec<M>,

    _p: PhantomData<M>,
}

pub trait Mesh<V> {
    const PRIM: PrimitiveTopology;

    const VERTICES: usize;
    const INDICES: usize;

    type VertexIter: Iterator<Item = V>;
    type IndexIter: Iterator<Item = u32>;

    fn vertices(&self) -> Self::VertexIter;
    // offset in Mesh elements not Vert elements
    fn indices(&self, offset: u32) -> Self::IndexIter;
}

#[derive(Debug, Clone, Copy, Default, Serialize, Deserialize)]
pub struct Idx(usize);

//

impl<M, V> BatchRenderer<M, V>
where
    M: Mesh<V>,
    V: Vertex + Copy,
{
    pub fn new(target: &Target) -> Self {
        Self {
            vbo: VertexBuffer::new(target, 2 * M::VERTICES),
            ibo: IndexBuffer::new(target, 2 * M::INDICES),
            ibo_regen: false,

            max: 2,
            modified: Default::default(),
            free: Default::default(),
            used: Default::default(),

            _p: Default::default(),
        }
    }

    pub fn push_with(&mut self, mesh: M) -> Idx {
        self.ibo_regen = true;
        let spot = if let Some(spot) = self.free.pop() {
            self.used[spot] = mesh;
            spot
        } else {
            let spot = self.used.len();
            self.used.push(mesh);
            spot
        };
        self.max = self.max.max(spot);
        self.modified.insert(spot);
        Idx(spot)
    }

    pub fn push(&mut self) -> Idx
    where
        M: Default,
    {
        self.push_with(Default::default())
    }

    pub fn leak(_: Idx) {}

    pub fn drop(&mut self, idx: Idx) {
        self.ibo_regen = true;
        self.modified.remove(&idx.0);
        self.free.push(idx.0);
    }

    pub fn get(&self, idx: Idx) -> &'_ M {
        &self.used[idx.0]
    }

    pub fn get_mut(&mut self, idx: Idx) -> &'_ mut M {
        self.modified.insert(idx.0);
        &mut self.used[idx.0]
    }

    pub fn generate(
        &mut self,
        target: &mut Target,
        frame: &mut Frame,
    ) -> (&'_ VertexBuffer<V>, &'_ IndexBuffer<u32>) {
        if self.ibo_regen {
            let new_data: Vec<u32> = self
                .used
                .iter()
                .enumerate()
                .flat_map(|(i, m)| m.indices(i as u32))
                .collect();

            // TODO: Copy old data instead of regenerating it
            if self.ibo.capacity() >= new_data.len() {
                self.ibo.upload(target, frame, &new_data);
            } else {
                self.ibo = IndexBuffer::new_with(target, &new_data);
            }
        }

        if !self.modified.is_empty() {
            let new_data: Vec<V> = self.used.iter().flat_map(|s| s.vertices()).collect();

            if self.vbo.capacity() >= new_data.len() {
                self.vbo.upload(target, frame, &new_data);
            } else {
                self.vbo = VertexBuffer::new_with(target, &new_data);
            }

            // TODO: Copy old data instead of regenerating it
            /* if self.max * M::VERTICES >= self.vbo.len() {
                self.vbo = VertexBuffer::new_with(target, &new_data);

                /* let new = VertexBuffer::empty_dynamic(facade, self.max * M::VERTICES * 2).unwrap();
                self.vbo.copy_to(&new).unwrap();
                self.vbo = new; */
            }

            let mut map = self.vbo.map_write();
            for modified in self.modified.drain() {
                for (i, vert) in self.used[modified].vertices().enumerate() {
                    map.set(modified * M::VERTICES + i, vert);
                }
            } */
        }

        (&self.vbo, &self.ibo)
    }
}
