use super::Buffer;
use wgpu::BufferUsages;

//

pub use ty::*;

//

pub mod ty;

//

pub type IndexBuffer<T = DefaultIndex> =
    Buffer<T, { BufferUsages::INDEX.bits() | BufferUsages::COPY_DST.bits() }>;