use bytemuck::Pod;
use std::{marker::PhantomData, mem, num::NonZeroU64};
use wgpu::{
    util::{BufferInitDescriptor, DeviceExt},
    BufferDescriptor, BufferUsages,
};

//

pub use index::*;
pub use indirect::*;
pub use uniform::*;
pub use vertex::*;

use crate::{label, target::Target, Frame};

//

pub mod index;
pub mod indirect;
pub mod uniform;
pub mod vertex;

//

#[derive(Debug)]
pub struct Buffer<T, const USAGE: u32>
where
    T: Pod,
{
    buffer: wgpu::Buffer,
    elements: usize,
    _p: PhantomData<T>,
}

//

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

    pub(crate) fn get_buffer(&self) -> &wgpu::Buffer {
        &self.buffer
    }

    pub fn capacity(&self) -> usize {
        self.elements
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
