use super::{Buffer, BufferSlice};
use wgpu::BufferUsages;

//

pub use ty::*;

//

pub mod ty;

//

const USAGE: u32 = BufferUsages::VERTEX.bits() | BufferUsages::COPY_DST.bits();

//

pub type VertexBuffer<T = DefaultVertex> = Buffer<T, USAGE>;
pub type VertexBufferSlice<'b, T> = BufferSlice<'b, T, USAGE>;
