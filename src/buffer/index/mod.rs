use super::{Buffer, BufferSlice};
use wgpu::BufferUsages;

//

pub use ty::*;

//

pub mod ty;

//

const USAGE: u32 = BufferUsages::INDEX.bits() | BufferUsages::COPY_DST.bits();

//

pub type IndexBuffer<T = DefaultIndex> = Buffer<T, USAGE>;
pub type IndexBufferSlice<'b, T = DefaultIndex> = BufferSlice<'b, T, USAGE>;
