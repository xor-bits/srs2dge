use crate::{label, target::Target, Frame};
use bytemuck::Pod;
use std::{marker::PhantomData, mem, num::NonZeroU64, ops::RangeBounds};
use wgpu::{
    util::{BufferInitDescriptor, DeviceExt},
    BufferAddress, BufferDescriptor, BufferUsages,
};

//

pub use index::*;
pub use indirect::*;
pub use uniform::*;
pub use vertex::*;

//

pub mod index;
pub mod indirect;
pub mod prelude;
pub mod uniform;
pub mod vertex;

//

#[derive(Debug)]
pub struct Buffer<T, const USAGE: u32> {
    buffer: wgpu::Buffer,
    elements: usize,
    _p: PhantomData<T>,
}

#[derive(Debug, Clone, Copy)]
pub struct BufferSlice<'b, T, const USAGE: u32> {
    slice: wgpu::BufferSlice<'b>,
    _p: PhantomData<T>,
}

//

impl<T, const USAGE: u32> Buffer<T, USAGE>
where
    T: Pod,
{
    pub fn new_with(target: &Target, data: &[T]) -> Self {
        let buffer = target.device.create_buffer_init(&BufferInitDescriptor {
            label: label!(),
            usage: BufferUsages::from_bits_truncate(USAGE),
            contents: bytemuck::cast_slice(data),
        });

        Self::with_buffer(buffer, data.len())
    }

    pub fn new_single(target: &Target, data: T) -> Self {
        let buffer = target.device.create_buffer_init(&BufferInitDescriptor {
            label: label!(),
            usage: BufferUsages::from_bits_truncate(USAGE),
            contents: bytemuck::cast_slice(&[data]),
        });

        Self::with_buffer(buffer, 1)
    }

    pub fn upload(&self, target: &mut Target, frame: &mut Frame, new_data: &[T]) {
        let mut mapping = frame.write_buffer(
            &self.buffer,
            0,
            NonZeroU64::new(Self::size_of(self.elements) as _).unwrap(),
            &target.device,
        );

        let new_data = bytemuck::cast_slice(new_data);
        mapping[..new_data.len()].copy_from_slice(new_data);
    }
}

impl<T, const USAGE: u32> Buffer<T, USAGE>
where
    T: Pod,
{
    pub fn new(target: &Target, elements: usize) -> Self {
        let buffer = target.device.create_buffer(&BufferDescriptor {
            label: label!(),
            size: Self::size_of(elements) as _,
            usage: BufferUsages::from_bits_truncate(USAGE),
            mapped_at_creation: false,
        });

        Self::with_buffer(buffer, elements)
    }

    pub fn inner(&self) -> &wgpu::Buffer {
        &self.buffer
    }

    pub fn capacity(&self) -> usize {
        self.elements
    }

    pub fn slice<S>(&self, range: S) -> BufferSlice<T, USAGE>
    where
        S: RangeBounds<BufferAddress>,
    {
        BufferSlice {
            slice: self.buffer.slice(range),
            _p: PhantomData::default(),
        }
    }

    fn with_buffer(buffer: wgpu::Buffer, elements: usize) -> Self {
        Self {
            buffer,
            elements,
            _p: Default::default(),
        }
    }

    fn size_of(elements: usize) -> usize {
        mem::size_of::<T>() * elements
    }
}

impl<'b, T, const USAGE: u32> BufferSlice<'b, T, USAGE> {
    pub fn inner(&self) -> wgpu::BufferSlice {
        self.slice
    }
}

//

pub trait AsBufferSlice<T, const USAGE: u32> {
    fn as_slice(&self) -> BufferSlice<T, USAGE>;
}

//

impl<T, const USAGE: u32> AsBufferSlice<T, USAGE> for Buffer<T, USAGE>
where
    T: Pod,
{
    fn as_slice(&self) -> BufferSlice<T, USAGE> {
        self.slice(..)
    }
}

impl<'b, T, const USAGE: u32> AsBufferSlice<T, USAGE> for BufferSlice<'b, T, USAGE>
where
    T: Pod,
{
    fn as_slice(&self) -> Self {
        *self
    }
}
