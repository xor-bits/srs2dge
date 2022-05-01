use self::ty::DefaultVertex;

use super::Buffer;
use wgpu::BufferUsages;

//

pub mod ty;

//

pub type VertexBuffer<T = DefaultVertex> =
    Buffer<T, { BufferUsages::VERTEX.bits() | BufferUsages::COPY_DST.bits() }>;

/* #[derive(Debug)]
pub struct VertexBuffer<T> {
    buffer: Buffer,
    _p: PhantomData<T>,
}

//

impl<T> VertexBuffer<T> {
    pub fn new(engine: &Engine, elements: usize) -> Self {
        let buffer = engine.device.create_buffer(&BufferDescriptor {
            label: label!(),
            size: mem::size_of::<T>() as u64 * elements as u64,
            usage: BufferUsages::VERTEX,
            mapped_at_creation: false,
        });

        Self::with_buffer(buffer)
    }

    pub(crate) fn bind<'a>(&'a self, render_pass: &mut RenderPass<'a>, slot: u32) {
        render_pass.set_vertex_buffer(slot, self.buffer.slice(..));
    }

    fn with_buffer(buffer: Buffer) -> Self {
        Self {
            buffer,
            _p: Default::default(),
        }
    }
}

impl<T> VertexBuffer<T>
where
    T: Pod,
{
    pub fn new_with(engine: &Engine, data: &[T]) -> Self {
        let buffer = engine.device.create_buffer_init(&BufferInitDescriptor {
            label: label!(),
            usage: BufferUsages::VERTEX,
            contents: bytemuck::cast_slice(data),
        });

        Self::with_buffer(buffer)
    }
} */
