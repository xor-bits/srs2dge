use super::Buffer;
use wgpu::BufferUsages;

//

pub type UniformBuffer<T> =
    Buffer<T, { BufferUsages::UNIFORM.bits() | BufferUsages::COPY_DST.bits() }>;
