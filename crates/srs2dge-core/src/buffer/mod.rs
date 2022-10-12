use crate::{
    prelude::{label, Frame, Target},
    unwrap_or_return,
};
use bytemuck::Pod;
use std::{borrow::Borrow, marker::PhantomData, mem, num::NonZeroU64, ops::RangeBounds};
use wgpu::{
    util::{BufferInitDescriptor, DeviceExt},
    BufferAddress, BufferDescriptor, BufferUsages, BufferViewMut,
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

    pub fn upload_single(&self, target: &mut Target, frame: &mut Frame, new_data: &T) {
        self.upload(target, frame, std::slice::from_ref(new_data))
    }

    pub fn upload(&self, target: &mut Target, frame: &mut Frame, new_data: &[T]) {
        self.upload_at(target, frame, 0, new_data);
    }

    pub fn upload_at(&self, target: &mut Target, frame: &mut Frame, offset: u64, new_data: &[T]) {
        let mut map = unwrap_or_return!(self.map(target, frame, offset, new_data.len() as _));

        let new_data = bytemuck::cast_slice(new_data);
        map[..].copy_from_slice(new_data);
    }

    pub fn upload_iter<I, R>(
        &self,
        target: &mut Target,
        frame: &mut Frame,
        offset: u64,
        count: u64,
        new_data: I,
    ) where
        I: IntoIterator<Item = R>,
        R: Borrow<T>,
    {
        let mut map = unwrap_or_return!(self.map(target, frame, offset, count));

        // copy
        let mut map = &mut map[..];
        for from in new_data.into_iter() {
            let x = bytemuck::bytes_of(from.borrow());
            let (copy, split) = map.split_at_mut(x.len());
            copy.copy_from_slice(x);
            map = split;

            if map.is_empty() {
                break;
            }
        }
    }

    fn map<'f>(
        &self,
        target: &mut Target,
        frame: &'f mut Frame,
        offset: u64,
        count: u64,
    ) -> Option<BufferViewMut<'f>> {
        // check count
        if count == 0 {
            None
        } else {
            let offset = Self::size_of(offset as _) as _;
            let size = NonZeroU64::new(Self::size_of(count as _) as _).unwrap();
            // map
            Some(frame.write_buffer(&self.buffer, offset, size, &target.device))
        }
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
