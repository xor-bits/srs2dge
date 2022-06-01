use self::{generated::GeneratedGui, renderer::GuiRenderer};
use crate::{
    event::PointerState,
    prelude::{Root, WidgetBase},
};
use srs2dge_core::{
    buffer::UniformBuffer,
    glam::Mat4,
    main_game_loop::{event::Event, prelude::WindowState},
    prelude::{Frame, Rect},
    shader::Layout,
    target::Target,
    texture::Texture,
    wgpu::TextureView,
    winit::event::{DeviceId, ElementState, MouseButton, WindowEvent},
};
use srs2dge_presets::{SdfShader, Texture2DShader};
use srs2dge_text::glyphs::Glyphs;
use std::collections::HashMap;

//

pub mod generated;
pub mod geom;
pub mod prelude;
pub mod renderer;

//

#[derive(Debug)]
pub struct Gui {
    ws: WindowState,

    ubo: UniformBuffer<Mat4>,
    texture_shader: Texture2DShader,
    text_shader: SdfShader,
    pub texture_batcher: GuiRenderer,
    pub text_batcher: GuiRenderer,

    texture: Option<Texture>,
    glyphs: Glyphs,

    pointers: HashMap<DeviceId, PointerState>,
}

//

impl Gui {
    pub fn new(target: &Target) -> Self {
        let ubo = UniformBuffer::new(target, 1);

        let texture_shader = Texture2DShader::new(target);
        let text_shader = SdfShader::new(target);

        Self {
            ws: WindowState::new(&target.get_window().unwrap()), // TODO: allow headless

            ubo,
            texture_shader,
            text_shader,
            texture_batcher: GuiRenderer::default(),
            text_batcher: GuiRenderer::default(),

            texture: None,
            glyphs: Glyphs::new_bytes(
                target,
                Rect::new(512, 512),
                Some(32),
                srs2dge_res::font::ROBOTO,
            )
            .unwrap(),

            pointers: Default::default(),
        }
    }

    /// default texture if custom one is not provided
    pub fn texture(&mut self, target: &Target) -> &Texture {
        Self::texture_inner(&mut self.texture, target)
    }

    /// handle gui events
    pub fn event(&mut self, event: Event<'static>) {
        self.ws.event(&event);

        match event {
            Event::WindowEvent { window_id, event } if Some(window_id) == self.ws.id => match event
            {
                WindowEvent::CursorMoved {
                    device_id,
                    position,
                    ..
                } => self
                    .pointers
                    .entry(device_id)
                    .or_default()
                    .moved_physical(position, &self.ws),
                WindowEvent::MouseInput {
                    device_id,
                    state: ElementState::Pressed,
                    button: MouseButton::Left,
                    ..
                } => self.pointers.entry(device_id).or_default().pressed(),
                WindowEvent::MouseInput {
                    device_id,
                    state: ElementState::Released,
                    button: MouseButton::Left,
                    ..
                } => self.pointers.entry(device_id).or_default().released(),
                _ => {}
            },
            _ => {}
        }
    }

    /// root widget
    ///
    /// start widget tree from this
    pub fn root(&mut self) -> Root {
        Root::new(&self.ws)
    }

    /// generate drawable geometry
    pub fn generate(&mut self, target: &mut Target, frame: &mut Frame) -> GeneratedGui {
        self.update_pointers();
        let texture = Self::texture_inner(&mut self.texture, target);
        Self::upload_ubo(&self.ubo, &self.ws, target, frame);
        Self::generate_inner(
            &self.ubo,
            (&mut self.texture_batcher, &mut self.text_batcher),
            (&self.texture_shader, &self.text_shader),
            target,
            frame,
            (texture, &self.glyphs),
        )
    }

    /// generate drawable geometry with custom texture
    pub fn generate_with(
        &mut self,
        target: &mut Target,
        frame: &mut Frame,
        texture: &TextureView,
    ) -> GeneratedGui {
        self.update_pointers();
        Self::upload_ubo(&self.ubo, &self.ws, target, frame);
        Self::generate_inner(
            &self.ubo,
            (&mut self.texture_batcher, &mut self.text_batcher),
            (&self.texture_shader, &self.text_shader),
            target,
            frame,
            (texture, &self.glyphs),
        )
    }

    /// gui internal `WindowState`
    pub fn window_state(&self) -> &WindowState {
        &self.ws
    }

    /// iterator to all cursors, touch points, ...
    pub fn pointers(&self) -> impl Iterator<Item = PointerState> + '_ {
        self.pointers.values().copied()
    }

    /// one of the pointers was pressed and released in this area
    pub fn clicked(&self, area: WidgetBase) -> bool {
        self.pointers().any(|pointer| match pointer {
            PointerState::Release { initial, now } => {
                let bl = area.offset;
                let tr = area.offset + area.size;

                (bl.x..tr.x).contains(&initial.x)
                    && (bl.y..tr.y).contains(&initial.y)
                    && (bl.x..tr.x).contains(&now.x)
                    && (bl.y..tr.y).contains(&now.y)
            }
            _ => false,
        })
    }

    /// one of the pointers is in this area
    pub fn hovered(&self, area: WidgetBase) -> bool {
        self.pointers().any(|pointer| {
            let now = match pointer {
                PointerState::Release { now, .. } => now,
                PointerState::Hover { now } => now,
                PointerState::Hold { now, .. } => now,
            };

            let bl = area.offset;
            let tr = area.offset + area.size;

            (bl.x..tr.x).contains(&now.x) && (bl.y..tr.y).contains(&now.y)
        })
    }

    /// clear pointer `released` states
    fn update_pointers(&mut self) {
        self.pointers
            .values_mut()
            .for_each(|pointer| pointer.update());
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

    fn upload_ubo<'a>(
        ubo: &'a UniformBuffer<Mat4>,
        ws: &WindowState,
        target: &mut Target,
        frame: &mut Frame,
    ) {
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
    }

    // partial borrows are not yet possible
    fn generate_inner<'a>(
        ubo: &'a UniformBuffer<Mat4>,
        (texture_batcher, text_batcher): (&'a mut GuiRenderer, &'a mut GuiRenderer),
        (texture_shader, text_shader): (&'a Texture2DShader, &'a SdfShader),
        target: &mut Target,
        frame: &mut Frame,
        (texture, glyphs): (&TextureView, &TextureView),
    ) -> GeneratedGui<'a> {
        let (texture_vbo, texture_ibo, texture_indices) = texture_batcher.generate(target, frame);
        let (text_vbo, text_ibo, text_indices) = text_batcher.generate(target, frame);

        GeneratedGui {
            texture_vbo,
            texture_ibo,
            texture_indices,

            texture_shader,
            texture_bindings: texture_shader.bind_group((ubo, texture)),

            text_vbo,
            text_ibo,
            text_indices,

            text_shader,
            text_bindings: text_shader.bind_group((ubo, glyphs)),
        }
    }
}
