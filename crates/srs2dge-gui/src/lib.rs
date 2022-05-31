use prelude::Root;
use srs2dge_core::{
    batch::BatchRenderer,
    buffer::{DefaultVertex, IndexBuffer, UniformBuffer, VertexBuffer},
    glam::Mat4,
    main_game_loop::{event::Event, prelude::WindowState},
    prelude::{Frame, RenderPass},
    shader::Layout,
    target::Target,
    wgpu::BindGroup,
};
use srs2dge_presets::Colored2DShader;

// pub mod expr;
pub mod prelude;
pub mod widget;

//

pub struct Gui {
    pub batcher: BatchRenderer,

    ws: WindowState,

    ubo: UniformBuffer<Mat4>,
    color_shader: Colored2DShader,
    color_shader_bindings: BindGroup,
}

pub struct GeneratedGui<'a> {
    vbo: &'a VertexBuffer<DefaultVertex>,
    ibo: &'a IndexBuffer<u32>,
    indices: u32,

    shader: &'a Colored2DShader,
    bindings: &'a BindGroup,
}

pub trait DrawGeneratedGui<'e> {
    fn draw_gui(self, generated: GeneratedGui<'e>) -> RenderPass<'e>;
}

//

impl Gui {
    pub fn new(target: &Target) -> Self {
        let ubo = UniformBuffer::new(target, 1);

        let color_shader = Colored2DShader::new(target);
        let color_shader_bindings = color_shader.bind_group(&ubo);

        Self {
            batcher: BatchRenderer::new(target),

            ws: WindowState::new(&target.get_window().unwrap()), // TODO: allow headless

            ubo,
            color_shader,
            color_shader_bindings,
        }
    }

    pub fn event(&mut self, event: &Event) {
        self.ws.event(event);
    }

    pub fn window_state(&self) -> &WindowState {
        &self.ws
    }

    pub fn root(&mut self) -> Root {
        self.batcher.clear();
        Root::new(&self.ws)
    }

    pub fn generate(&mut self, target: &mut Target, frame: &mut Frame) -> GeneratedGui {
        self.ubo.upload(
            target,
            frame,
            &[Mat4::orthographic_rh(
                0.0,
                self.ws.size.width as f32,
                0.0,
                self.ws.size.height as f32,
                -1.0,
                1.0,
            )],
        );
        let (vbo, ibo, indices) = self.batcher.generate(target, frame);
        GeneratedGui {
            vbo,
            ibo,
            indices,
            shader: &self.color_shader,
            bindings: &self.color_shader_bindings,
        }
    }
}

impl<'e, Sv, Bv, Si, Bi, const PIPELINE_BOUND: bool> DrawGeneratedGui<'e>
    for RenderPass<'e, Sv, Bv, Si, Bi, PIPELINE_BOUND>
{
    fn draw_gui(self, g: GeneratedGui<'e>) -> RenderPass<'e> {
        self.bind_vbo(g.vbo)
            .bind_ibo(g.ibo)
            .bind_group(g.bindings)
            .bind_shader(g.shader)
            .draw_indexed(0..g.indices, 0, 0..1)
            .done()
    }
}
