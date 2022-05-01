use crate::{
    buffer::{index::ty::Index, IndexBuffer, VertexBuffer},
    shader::Shader,
};
use bytemuck::Pod;
use std::ops::Range;
use wgpu::{BindGroup, TextureFormat};

//

pub struct RenderPass<'e, const PIPELINE_BOUND: bool> {
    pub(crate) inner: wgpu::RenderPass<'e>,
    pub(crate) format: TextureFormat,
}

//

impl<'e, const PIPELINE_BOUND: bool> RenderPass<'e, PIPELINE_BOUND> {
    pub fn bind_vbo<'b: 'e, T: Pod + 'static>(
        mut self,
        buffer: &'b VertexBuffer<T>,
        slot: u32,
    ) -> Self {
        self.inner
            .set_vertex_buffer(slot, buffer.get_buffer().slice(..));
        self
    }

    pub fn bind_ibo<'b: 'e, T: Index + 'static>(mut self, buffer: &'b IndexBuffer<T>) -> Self {
        self.inner
            .set_index_buffer(buffer.get_buffer().slice(..), T::FORMAT);
        self
    }

    pub fn bind_shader<'s: 'e>(mut self, shader: &'s Shader) -> RenderPass<'e, true> {
        shader.bind(&mut self);
        Self::pass(self)
    }

    pub fn bind_group<'g: 'e>(mut self, bind_group: &'g BindGroup) -> Self {
        self.inner.set_bind_group(0, bind_group, &[]);
        self
    }

    pub(crate) fn new(inner: wgpu::RenderPass<'e>, format: TextureFormat) -> Self {
        Self { inner, format }
    }

    fn pass<const N: bool>(self) -> RenderPass<'e, N> {
        let RenderPass { inner, format } = self;
        RenderPass { inner, format }
    }
}

impl<'e> RenderPass<'e, true> {
    pub fn draw(mut self, vertices: Range<u32>, instances: Range<u32>) -> Self {
        self.inner.draw(vertices, instances);
        self
    }

    pub fn draw_indexed(
        mut self,
        indices: Range<u32>,
        base_vertex: i32,
        instances: Range<u32>,
    ) -> Self {
        self.inner.draw_indexed(indices, base_vertex, instances);
        self
    }

    // TODO:
    /* pub fn draw_indirect(&mut self, vertices: Range<u32>, instances: Range<u32>) {
        self.inner.draw_indirect(vertices, instances)
    }

    pub fn draw_indirect_indexed(&mut self, vertices: Range<u32>, instances: Range<u32>) {
        self.inner.draw(vertices, instances)
    } */
}
