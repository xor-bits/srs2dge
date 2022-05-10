use super::{Buffer, BufferSlice};
use wgpu::BufferUsages;

//

const USAGE: u32 = BufferUsages::UNIFORM.bits() | BufferUsages::COPY_DST.bits();

//

pub type UniformBuffer<T> = Buffer<T, USAGE>;
pub type UniformBufferSlice<'b, T> = BufferSlice<'b, T, USAGE>;
