use super::{Buffer, BufferSlice};
use glam::Mat4;
use wgpu::BufferUsages;

//

const USAGE: u32 = BufferUsages::UNIFORM.bits() | BufferUsages::COPY_DST.bits();

//

pub type UniformBuffer<T = Mat4> = Buffer<T, USAGE>;
pub type UniformBufferSlice<'b, T = Mat4> = BufferSlice<'b, T, USAGE>;
