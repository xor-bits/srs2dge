use prelude::Root;
use srs2dge_core::{
    batch::BatchRenderer,
    buffer::{DefaultVertex, IndexBuffer, UniformBuffer, VertexBuffer},
    glam::Mat4,
    main_game_loop::{event::Event, prelude::WindowState},
    prelude::{Frame, RenderPass},
    shader::Layout,
    target::Target,
    texture::Texture,
    wgpu::{BindGroup, TextureView},
};
use srs2dge_presets::Texture2DShader;

// pub mod expr;
pub mod prelude;
pub mod widget;

//

#[derive(Debug)]
pub struct Gui {
    pub batcher: BatchRenderer,

    ws: WindowState,

    ubo: UniformBuffer<Mat4>,
    shader: Texture2DShader,

    texture: Option<Texture>,
}

pub struct GeneratedGui<'a> {
    vbo: &'a VertexBuffer<DefaultVertex>,
    ibo: &'a IndexBuffer<u32>,
    indices: u32,

    shader: &'a Texture2DShader,
    bindings: BindGroup,
}

pub trait DrawGeneratedGui<'e> {
    fn draw_gui(self, generated: &'e GeneratedGui<'e>) -> RenderPass<'e>;
}

//

impl Gui {
    pub fn new(target: &Target) -> Self {
        let ubo = UniformBuffer::new(target, 1);

        let shader = Texture2DShader::new(target);

        Self {
            batcher: BatchRenderer::new(target),

            ws: WindowState::new(&target.get_window().unwrap()), // TODO: allow headless

            ubo,
            shader,

            texture: None,
        }
    }

    pub fn texture(&mut self, target: &Target) -> &Texture {
        Self::texture_inner(&mut self.texture, target)
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
        let texture = Self::texture_inner(&mut self.texture, target);
        Self::generate_inner(
            &self.ubo,
            &self.ws,
            &mut self.batcher,
            &self.shader,
            target,
            frame,
            texture,
        )
    }

    pub fn generate_with(
        &mut self,
        target: &mut Target,
        frame: &mut Frame,
        texture: &TextureView,
    ) -> GeneratedGui {
        Self::generate_inner(
            &self.ubo,
            &self.ws,
            &mut self.batcher,
            &self.shader,
            target,
            frame,
            texture,
        )
    }

    fn texture_inner<'a>(texture: &'a mut Option<Texture>, target: &Target) -> &'a Texture {
        if texture.is_none() {
            *texture = Some(Texture::new_rgba_with(
                target,
                &srs2dge_core::image::load_from_memory(srs2dge_res::texture::EMPTY)
                    .unwrap()
                    .to_rgba8(),
            ));
        }

        texture.as_ref().unwrap()
    }

    fn generate_inner<'a>(
        ubo: &'a UniformBuffer<Mat4>,
        ws: &WindowState,
        batcher: &'a mut BatchRenderer,
        shader: &'a Texture2DShader,
        target: &mut Target,
        frame: &mut Frame,
        texture: &TextureView,
    ) -> GeneratedGui<'a> {
        ubo.upload(
            target,
            frame,
            &[Mat4::orthographic_rh(
                0.0,
                ws.size.width as f32,
                0.0,
                ws.size.height as f32,
                -1.0,
                1.0,
            )],
        );

        let (vbo, ibo, indices) = batcher.generate(target, frame);

        GeneratedGui {
            vbo,
            ibo,
            indices,
            shader,
            bindings: shader.bind_group((ubo, texture)),
        }
    }
}

impl<'e, Sv, Bv, Si, Bi, const PIPELINE_BOUND: bool> DrawGeneratedGui<'e>
    for RenderPass<'e, Sv, Bv, Si, Bi, PIPELINE_BOUND>
{
    fn draw_gui(self, g: &'e GeneratedGui<'e>) -> RenderPass<'e> {
        self.bind_vbo(g.vbo)
            .bind_ibo(g.ibo)
            .bind_group(&g.bindings)
            .bind_shader(g.shader)
            .draw_indexed(0..g.indices, 0, 0..1)
            .done()
    }
}
