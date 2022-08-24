use crate::prelude::{GeneratedGui, GuiRenderer};
use srs2dge_core::{
    buffer::UniformBuffer,
    glam::Mat4,
    main_game_loop::prelude::WindowState,
    prelude::{Frame, Rect},
    shader::Layout,
    target::Target,
    texture::Texture,
    wgpu::TextureView,
};
use srs2dge_presets::{TextShader, Texture2DShader};
use srs2dge_text::glyphs::Glyphs;

//

#[derive(Debug)]
pub struct GuiGraphics {
    pub(crate) ubo: UniformBuffer,
    pub(crate) texture_shader: Texture2DShader,
    pub(crate) text_shader: TextShader,
    pub texture_batcher: GuiRenderer,
    pub text_batcher: GuiRenderer,

    pub(crate) texture: Option<Texture>,
    pub(crate) glyphs: Glyphs,
}

//

impl GuiGraphics {
    pub fn new(target: &Target) -> Self {
        Self {
            ubo: UniformBuffer::new(target, 1),
            texture_shader: Texture2DShader::new(target),
            text_shader: TextShader::new(target),
            texture_batcher: GuiRenderer::default(),
            text_batcher: GuiRenderer::default(),

            texture: None,
            glyphs: Glyphs::new_with_fallback_bytes(
                target,
                Rect::new(512, 512),
                None,
                srs2dge_res::font::ROBOTO,
                Some("GuiGlyphs"),
            )
            .unwrap(),
        }
    }

    pub fn draw<'a>(
        &'a mut self,
        target: &mut Target,
        frame: &mut Frame,
        ws: &WindowState,
        texture: Option<&TextureView>,
    ) -> GeneratedGui<'a> {
        let texture = if let Some(texture) = texture {
            texture
        } else {
            Self::get_texture_inner(&mut self.texture, target)
        };

        Self::upload_ubo(&self.ubo, ws, target, frame);
        Self::generate_inner(
            &self.ubo,
            (&mut self.texture_batcher, &mut self.text_batcher),
            (&self.texture_shader, &self.text_shader),
            target,
            frame,
            (texture, &self.glyphs),
        )
    }

    pub fn get_texture(&mut self, target: &Target) -> &Texture {
        Self::get_texture_inner(&mut self.texture, target)
    }

    // --------------------------
    //  Dumb looking workarounds
    // because partial borrowing
    // is not yet a thing in Rust
    // --------------------------

    fn get_texture_inner<'a>(texture: &'a mut Option<Texture>, target: &Target) -> &'a Texture {
        texture.get_or_insert_with(|| {
            Texture::new_rgba_with(
                target,
                &srs2dge_core::image::load_from_memory(srs2dge_res::texture::EMPTY)
                    .unwrap()
                    .to_rgba8(),
                Some("GuiFallbackRGBATexture"),
            )
        })
    }

    fn upload_ubo<'a>(
        ubo: &'a UniformBuffer,
        ws: &WindowState,
        target: &mut Target,
        frame: &mut Frame,
    ) {
        let mvp = Mat4::orthographic_rh(
            0.0,
            ws.size.width as f32,
            0.0,
            ws.size.height as f32,
            -1.0,
            1.0,
        );
        ubo.upload(target, frame, &[mvp]);
    }

    // partial borrows are not yet possible
    fn generate_inner<'a>(
        ubo: &'a UniformBuffer,
        (texture_batcher, text_batcher): (&'a mut GuiRenderer, &'a mut GuiRenderer),
        (texture_shader, text_shader): (&'a Texture2DShader, &'a TextShader),
        target: &mut Target,
        frame: &mut Frame,
        (texture, glyphs): (&TextureView, &TextureView),
    ) -> GeneratedGui<'a> {
        let (texture_vbo, texture_ibo, texture_indices) = texture_batcher.generate(target, frame);
        let (text_vbo, text_ibo, text_indices) = text_batcher.generate(target, frame);

        GeneratedGui {
            texture_shader,
            texture_vbo,
            texture_ibo,
            texture_indices,
            texture_bindings: texture_shader.bind_group((ubo, texture)),

            text_shader,
            text_vbo,
            text_ibo,
            text_indices,
            text_bindings: text_shader.bind_group((ubo, glyphs)),
        }
    }
}
