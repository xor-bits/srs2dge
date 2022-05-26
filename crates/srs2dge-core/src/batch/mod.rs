use self::quad::QuadMesh;
use crate::{
    buffer::{
        vertex::{DefaultVertex, Vertex},
        IndexBuffer, VertexBuffer,
    },
    target::Target,
    Frame,
};
use serde::{Deserialize, Serialize};
use std::{
    collections::{BinaryHeap, HashSet},
    marker::PhantomData,
};

//

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
    ibo_len: u32,

    max: usize,
    modified: HashSet<usize>,
    free: BinaryHeap<usize>,
    used: Vec<Option<M>>,

    _p: PhantomData<M>,
}

pub trait Mesh<V> {
    const PRIM: PrimitiveTopology;

    type VertexIter: Iterator<Item = V>;
    type IndexIter: Iterator<Item = u32>;

    fn vertices(&self) -> Self::VertexIter;
    fn indices(&self, offset: u32) -> Self::IndexIter;
    fn index_step(&self) -> u32;
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
            vbo: VertexBuffer::new(target, 0),
            ibo: IndexBuffer::new(target, 0),
            ibo_regen: false,
            ibo_len: 0,

            max: 0,
            modified: Default::default(),
            free: Default::default(),
            used: Default::default(),

            _p: Default::default(),
        }
    }

    pub fn push_with(&mut self, mesh: M) -> Idx {
        self.ibo_regen = true;
        let spot = if let Some(spot) = self.free.pop() {
            self.used[spot] = Some(mesh);
            spot
        } else {
            let spot = self.used.len();
            self.used.push(Some(mesh));
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

    pub fn drop(&mut self, idx: Idx) {
        self.ibo_regen = true;
        if let Some(m) = self.used.get_mut(idx.0) {
            *m = None;
        }
        self.modified.insert(idx.0);
        self.free.push(idx.0);
    }

    pub fn get(&self, idx: Idx) -> Option<&M> {
        if let Some(Some(mesh)) = self.used.get(idx.0) {
            Some(mesh)
        } else {
            None
        }
    }

    pub fn get_mut(&mut self, idx: Idx) -> Option<&mut M> {
        if let Some(Some(mesh)) = self.used.get_mut(idx.0) {
            self.modified.insert(idx.0);
            Some(mesh)
        } else {
            None
        }
    }

    pub fn generate(
        &mut self,
        target: &mut Target,
        frame: &mut Frame,
    ) -> (&'_ VertexBuffer<V>, &'_ IndexBuffer<u32>, u32) {
        if self.ibo_regen {
            let mut i = 0;
            let new_data: Vec<u32> = self
                .used
                .iter()
                .filter_map(|m| m.as_ref())
                .flat_map(|m| {
                    let offset = i;
                    i += m.index_step();
                    m.indices(offset)
                })
                .collect();
            self.ibo_len = new_data.len() as _;

            // TODO: Copy old data instead of regenerating it
            if self.ibo.capacity() >= new_data.len() {
                self.ibo.upload(target, frame, &new_data);
            } else {
                self.ibo = IndexBuffer::new_with(target, &new_data);
            }
        }

        if !self.modified.is_empty() {
            let new_data: Vec<V> = self
                .used
                .iter()
                .filter_map(|m| m.as_ref())
                .flat_map(|s| s.vertices())
                .collect();

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

        (&self.vbo, &self.ibo, self.ibo_len)
    }
}
