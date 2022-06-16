use srs2dge_core::{
    buffer::{DefaultIndex, DefaultVertex, IndexBuffer, VertexBuffer},
    prelude::RenderPass,
    shader::Shader,
    wgpu::BindGroup,
};

//

pub struct GeneratedGui<'a> {
    pub texture_shader: &'a Shader<DefaultVertex, DefaultIndex>,
    pub texture_vbo: &'a VertexBuffer<DefaultVertex>,
    pub texture_ibo: &'a IndexBuffer<u32>,
    pub texture_indices: u32,
    pub texture_bindings: BindGroup,

    pub text_shader: &'a Shader<DefaultVertex, DefaultIndex>,
    pub text_vbo: &'a VertexBuffer<DefaultVertex>,
    pub text_ibo: &'a IndexBuffer<u32>,
    pub text_indices: u32,
    pub text_bindings: BindGroup,
}

//

pub trait DrawGeneratedGui<'e> {
    fn draw_gui(self, generated: &'e GeneratedGui<'e>) -> RenderPass<'e>;
}

//

impl<'e, Sv, Bv, Si, Bi, const PIPELINE_BOUND: bool> DrawGeneratedGui<'e>
    for RenderPass<'e, Sv, Bv, Si, Bi, PIPELINE_BOUND>
{
    fn draw_gui(self, g: &'e GeneratedGui<'e>) -> RenderPass<'e> {
        let mut pass = self.done();

        // texture pass
        if g.texture_indices != 0 {
            pass = pass
                .bind_vbo(g.texture_vbo)
                .bind_ibo(g.texture_ibo)
                .bind_group(&g.texture_bindings)
                .bind_shader(g.texture_shader)
                .draw_indexed(0..g.texture_indices, 0, 0..1)
                .done();
        }

        // text pass
        if g.text_indices != 0 {
            pass = pass
                .bind_vbo(g.text_vbo)
                .bind_ibo(g.text_ibo)
                .bind_group(&g.text_bindings)
                .bind_shader(g.text_shader)
                .draw_indexed(0..g.text_indices, 0, 0..1)
                .done();
        }

        // return cleared
        pass
    }
}
