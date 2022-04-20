use crate::{label, Engine};
use bytemuck::Pod;
use std::{marker::PhantomData, mem};
use wgpu::{
    util::{BufferInitDescriptor, DeviceExt},
    BindingResource, Buffer, BufferDescriptor, BufferUsages,
};

//

pub struct UniformBuffer<T> {
    buffer: Buffer,
    _p: PhantomData<T>,
}

//

impl<T> UniformBuffer<T> {
    pub fn new(engine: &Engine, elements: u32) -> Self {
        let buffer = engine.device.create_buffer(&BufferDescriptor {
            label: label!(),
            size: mem::size_of::<T>() as u64 * elements as u64,
            usage: BufferUsages::UNIFORM,
            mapped_at_creation: false,
        });

        Self::with_buffer(buffer)
    }

    pub(crate) fn bind(&self) -> BindingResource {
        BindingResource::Buffer(self.buffer.as_entire_buffer_binding())
    }

    fn with_buffer(buffer: Buffer) -> Self {
        Self {
            buffer,
            _p: Default::default(),
        }
    }
}

impl<T> UniformBuffer<T>
where
    T: Pod,
{
    pub fn new_with(engine: &Engine, data: &[T]) -> Self {
        let buffer = engine.device.create_buffer_init(&BufferInitDescriptor {
            label: label!(),
            usage: BufferUsages::UNIFORM,
            contents: bytemuck::cast_slice(data),
        });

        Self::with_buffer(buffer)
    }
}
