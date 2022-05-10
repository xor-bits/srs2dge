use crate::{
    buffer::{index::Index, IndexBuffer, Vertex, VertexBuffer},
    shader::Shader,
};
use std::{marker::PhantomData, ops::Range};
use wgpu::{BindGroup, TextureFormat};

//

pub struct RenderPass<'e, Sv = (), Bv = (), Si = (), Bi = (), const PIPELINE_BOUND: bool = false> {
    pub(crate) inner: wgpu::RenderPass<'e>,
    pub(crate) format: TextureFormat,

    _p: PhantomData<(Sv, Bv, Si, Bi)>,
}

//

impl<'e, Sv, Bv, Si, Bi, const PIPELINE_BOUND: bool>
    RenderPass<'e, Sv, Bv, Si, Bi, PIPELINE_BOUND>
{
    pub fn bind_vbo<'b, T>(
        mut self,
        buffer: &'b VertexBuffer<T>,
        // slot: u32,
    ) -> RenderPass<'e, Sv, T, Si, Bi, PIPELINE_BOUND>
    where
        'b: 'e,
        T: Vertex + 'static,
    {
        self.inner
            .set_vertex_buffer(0, buffer.inner().slice(..));
        self.pass()
    }

    pub fn bind_ibo<'b, T>(
        mut self,
        buffer: &'b IndexBuffer<T>,
    ) -> RenderPass<'e, Sv, Bv, Si, T, PIPELINE_BOUND>
    where
        'b: 'e,
        T: Index + 'static,
    {
        self.inner
            .set_index_buffer(buffer.inner().slice(..), T::FORMAT);
        self.pass()
    }

    pub fn bind_shader<'s, V, I>(
        mut self,
        shader: &'s Shader<V, I>,
    ) -> RenderPass<'e, V, Bv, I, Bi, true>
    where
        's: 'e,
        V: Vertex + 'static,
        I: Index + 'static,
    {
        if self.format != shader.format {
            panic!("Shader output incompatible with this render target");
        } else {
            self.inner.set_pipeline(&shader.pipeline);
        }
        self.pass()
    }

    pub fn bind_group<'g>(mut self, bind_group: &'g BindGroup) -> Self
    where
        'g: 'e,
    {
        self.inner.set_bind_group(0, bind_group, &[]);
        self.pass()
    }

    pub fn done(self) -> RenderPass<'e> {
        self.pass()
    }

    pub(crate) fn new(inner: wgpu::RenderPass<'e>, format: TextureFormat) -> Self {
        Self {
            inner,
            format,
            _p: PhantomData::default(),
        }
    }

    fn pass<Svn, Bvn, Sin, Bin, const N: bool>(self) -> RenderPass<'e, Svn, Bvn, Sin, Bin, N> {
        RenderPass {
            inner: self.inner,
            format: self.format,
            _p: PhantomData::default(),
        }
    }
}

// implement for all renderpasses where buffers match shaders and shader is bound
impl<'e, V, I> RenderPass<'e, V, V, I, I, true> {
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
