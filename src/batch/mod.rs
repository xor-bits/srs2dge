use crate::Engine;
use glium::{index::PrimitiveType, IndexBuffer, Vertex, VertexBuffer};
use std::{
    collections::{BinaryHeap, HashSet},
    marker::PhantomData,
};

//

pub mod quad;

//

#[derive(Debug)]
pub struct BatchRenderer<V, M>
where
    V: Vertex + Copy,
    M: Mesh<V> + Default,
{
    vbo: VertexBuffer<V>,
    ibo: IndexBuffer<u32>,
    ibo_regen: bool,

    max: usize,
    modified: HashSet<usize>,
    free: BinaryHeap<usize>,
    used: Vec<Option<M>>,

    _p: PhantomData<M>,
}

pub trait Mesh<V> {
    const PRIM: PrimitiveType;

    const VERTICES: usize;
    const INDICES: usize;

    type VertexIter: Iterator<Item = V>;
    type IndexIter: Iterator<Item = u32>;

    fn vertices(&self) -> Self::VertexIter;
    // offset in Mesh elements not Vert elements
    fn indices(&self, offset: u32) -> Self::IndexIter;
}

pub struct Idx(usize);

//

impl<V, M> BatchRenderer<V, M>
where
    V: Vertex + Copy,
    M: Mesh<V> + Default,
{
    pub fn new(engine: &Engine) -> Self {
        Self {
            vbo: VertexBuffer::empty_dynamic(engine, 2 * M::VERTICES).unwrap(),
            ibo: IndexBuffer::empty_dynamic(engine, M::PRIM, 2 * M::INDICES).unwrap(),
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

    pub fn push(&mut self) -> Idx {
        self.push_with(Default::default())
    }

    pub fn leak(_: Idx) {}

    pub fn drop(&mut self, idx: Idx) {
        self.ibo_regen = true;
        self.modified.insert(idx.0);
        self.used[idx.0] = None;
        self.free.push(idx.0);
    }

    pub fn get(&self, idx: Idx) -> &'_ M {
        self.used[idx.0].as_ref().unwrap()
    }

    pub fn get_mut(&mut self, idx: Idx) -> &'_ mut M {
        self.modified.insert(idx.0);
        self.used[idx.0].as_mut().unwrap()
    }

    pub fn draw(&mut self, engine: &Engine) -> (&'_ VertexBuffer<V>, &'_ IndexBuffer<u32>) {
        if self.ibo_regen {
            let ibo: Vec<u32> = self
                .used
                .iter()
                .filter_map(|mesh| mesh.as_ref())
                .enumerate()
                .flat_map(|(i, m)| m.indices(i as u32))
                .collect();

            if let Some(map) = self.ibo.slice_mut(..ibo.len()) {
                map.write(&ibo);
            } else {
                self.ibo = IndexBuffer::dynamic(engine, M::PRIM, &ibo).unwrap();
            }
        }

        if !self.modified.is_empty() {
            if self.max * M::VERTICES >= self.vbo.len() {
                let new = VertexBuffer::empty_dynamic(engine, self.vbo.len() * 2).unwrap();
                self.vbo.copy_to(&new).unwrap();
                self.vbo = new;
            }

            let mut map = self.vbo.map_write();
            for modified in self.modified.drain() {
                for (i, vert) in self.used[modified].as_ref().unwrap().vertices().enumerate() {
                    map.set(modified * M::VERTICES + i, vert);
                }
            }
        }

        (&self.vbo, &self.ibo)
    }
}
