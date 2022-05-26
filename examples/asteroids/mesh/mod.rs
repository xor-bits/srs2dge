use self::{asteroid::AsteroidMesh, player::PlayerMesh};

use srs2dge::prelude::*;

//

pub mod asteroid;
pub mod player;

//

#[derive(Debug, Clone, Copy)]
pub enum MultiMesh {
    Player(PlayerMesh),
    Bullet(GizmosCircle),
    Asteroid(AsteroidMesh),
}

//

impl Mesh<DefaultVertex> for MultiMesh {
    const PRIM: PrimitiveTopology = PrimitiveTopology::LineStrip;

    type VertexIter = IterEnum<
        <PlayerMesh as Mesh<DefaultVertex>>::VertexIter,
        <GizmosCircle as Mesh<DefaultVertex>>::VertexIter,
        <AsteroidMesh as Mesh<DefaultVertex>>::VertexIter,
        <<PlayerMesh as Mesh<DefaultVertex>>::VertexIter as Iterator>::Item,
    >;
    type IndexIter = IterEnum<
        <PlayerMesh as Mesh<DefaultVertex>>::IndexIter,
        <GizmosCircle as Mesh<DefaultVertex>>::IndexIter,
        <AsteroidMesh as Mesh<DefaultVertex>>::IndexIter,
        <<PlayerMesh as Mesh<DefaultVertex>>::IndexIter as Iterator>::Item,
    >;

    fn vertices(&self) -> Self::VertexIter {
        match self {
            MultiMesh::Player(mesh) => IterEnum::A(mesh.vertices()),
            MultiMesh::Bullet(mesh) => IterEnum::B(mesh.vertices()),
            MultiMesh::Asteroid(mesh) => IterEnum::C(mesh.vertices()),
        }
    }

    fn indices(&self, offset: u32) -> Self::IndexIter {
        match self {
            MultiMesh::Player(mesh) => IterEnum::A(mesh.indices(offset)),
            MultiMesh::Bullet(mesh) => IterEnum::B(mesh.indices(offset)),
            MultiMesh::Asteroid(mesh) => IterEnum::C(mesh.indices(offset)),
        }
    }

    fn index_step(&self) -> u32 {
        match self {
            MultiMesh::Player(mesh) => mesh.index_step(),
            MultiMesh::Bullet(mesh) => mesh.index_step(),
            MultiMesh::Asteroid(mesh) => mesh.index_step(),
        }
    }
}

//

pub enum IterEnum<A, B, C, I>
where
    A: Iterator<Item = I>,
    B: Iterator<Item = I>,
    C: Iterator<Item = I>,
{
    A(A),
    B(B),
    C(C),
}

impl<A, B, C, I> Iterator for IterEnum<A, B, C, I>
where
    A: Iterator<Item = I>,
    B: Iterator<Item = I>,
    C: Iterator<Item = I>,
{
    type Item = I;

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            IterEnum::A(a) => a.next(),
            IterEnum::B(b) => b.next(),
            IterEnum::C(c) => c.next(),
        }
    }
}
