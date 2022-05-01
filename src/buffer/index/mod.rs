use self::ty::DefaultIndex;

use super::Buffer;
use wgpu::BufferUsages;

//

pub mod ty;

//

pub type IndexBuffer<T = DefaultIndex> =
    Buffer<T, { BufferUsages::INDEX.bits() | BufferUsages::COPY_DST.bits() }>;

/* #[derive(Debug)]
pub struct IndexBuffer<T>
where
    T: Index,
{
    buffer: Buffer,
    elements: usize,
    _p: PhantomData<T>,
}

//

impl<T> IndexBuffer<T>
where
    T: Index,
{
    pub fn new(engine: &Engine, elements: usize) -> Self {
        let buffer = engine.device.create_buffer(&BufferDescriptor {
            label: label!(),
            size: Self::size_of(elements) as _,
            usage: BufferUsages::INDEX | BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        Self::with_buffer(buffer, elements)
    }

    pub fn new_with(engine: &Engine, data: &[T]) -> Self {
        let buffer = engine.device.create_buffer_init(&BufferInitDescriptor {
            label: label!(),
            usage: BufferUsages::INDEX,
            contents: bytemuck::cast_slice(data),
        });

        Self::with_buffer(buffer, data.len())
    }

    pub fn upload(&self, engine: &Engine, frame: &mut Frame, new_data: &[T]) {
        let encoder = frame.encoder();

        let mapping = engine.belt.write_buffer(
            encoder,
            &self.buffer,
            0,
            NonZeroU64::new(Self::size_of(self.elements) as _).unwrap(),
            &engine.device,
        );

        mapping.copy_from_slice(bytemuck::cast_slice(new_data));
    }

    pub(crate) fn bind<'a>(&'a self, render_pass: &mut RenderPass<'a>) {
        render_pass.set_index_buffer(self.buffer.slice(..), T::FORMAT);
    }

    pub fn capacity(&self) -> usize {
        self.elements
    }

    fn with_buffer(buffer: Buffer, elements: usize) -> Self {
        Self {
            buffer,
            elements,
            _p: Default::default(),
        }
    }

    fn size_of(elements: usize) -> usize {
        mem::size_of::<T>() * elements
    }
} */
