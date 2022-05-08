use super::Buffer;
use wgpu::BufferUsages;

//

pub use ty::*;

//

pub mod ty;

//

pub type VertexBuffer<T = DefaultVertex> =
    Buffer<T, { BufferUsages::VERTEX.bits() | BufferUsages::COPY_DST.bits() }>;
