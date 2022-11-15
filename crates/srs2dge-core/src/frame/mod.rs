use self::{compute_pass::ComputePass, render_pass::RenderPass};
use crate::{
    color::Color,
    label,
    target::{prelude::BeltPool, surface::Surface},
    texture::{has_render_attachment, Texture},
};
use std::{
    ops::{Deref, DerefMut},
    sync::Arc,
};
use tracing::{debug, debug_span, span::EnteredSpan, warn};
use wgpu::{
    util::StagingBelt, Buffer, BufferAddress, BufferSize, BufferViewMut, CommandEncoder,
    CommandEncoderDescriptor, ComputePassDescriptor, Device, LoadOp, Operations, Queue,
    RenderPassColorAttachment, RenderPassDescriptor, SurfaceTexture, TextureFormat, TextureView,
    TextureViewDescriptor,
};

//

pub mod compute_pass;
pub mod prelude;
pub mod render_pass;

//

pub struct Frame {
    // render target
    main_texture: SafeDrop<SurfaceTexture>,
    main_view: TextureView,
    main_format: TextureFormat,
    main_dim: (u32, u32),

    // command buffer
    encoder: SafeDrop<CommandEncoder>,

    // logical device queue
    queue: Arc<Queue>,

    // render pass clear color
    clear_color: Color,

    // state
    has_primary: bool,

    // tracing
    _span: EnteredSpan,

    // belt
    pub(crate) belt: SafeDrop<StagingBelt>,
    belts: Arc<BeltPool>,
}

//

impl Frame {
    pub fn new(
        device: &Device,
        queue: Arc<Queue>,
        surface: &mut Surface,
        belts: Arc<BeltPool>,
        frame_id: usize,
    ) -> Self {
        let _span = debug_span!("Begin frame", frame_id).entered();

        let main_texture = surface.acquire();
        let main_view = main_texture.texture.create_view(&TextureViewDescriptor {
            label: label!(),
            ..Default::default()
        });
        let main_format = surface.format();
        let main_dim = surface.get_dim();

        let encoder = device.create_command_encoder(&CommandEncoderDescriptor { label: label!() });

        let main_texture = main_texture.into();
        let encoder = encoder.into();
        let belt = belts.recv().into();

        Self {
            main_texture,
            main_view,
            main_format,
            main_dim,

            encoder,

            queue,

            clear_color: Color::CLEAR_COLOR,

            has_primary: false,

            _span,

            belt,
            belts,
        }
    }
}

impl Frame {
    pub fn set_clear_color(&mut self, color: Color) {
        self.clear_color = color;
    }

    pub fn primary_render_pass(&mut self) -> RenderPass<(), (), (), (), false> {
        let span = debug_span!("Primary render pass").entered();

        let pass = self.encoder.begin_render_pass(&RenderPassDescriptor {
            label: label!(),
            color_attachments: &[Some(RenderPassColorAttachment {
                view: &self.main_view,
                resolve_target: None,
                ops: Operations {
                    load: LoadOp::Clear(self.clear_color.into()),
                    store: true,
                },
            })],
            depth_stencil_attachment: None,
        });

        debug!("Created");

        self.has_primary = true;

        RenderPass::new(pass, self.main_format, span)
    }

    pub fn secondary_render_pass<'a, const USAGE: u32>(
        &'a mut self,
        target: &'a Texture<USAGE>,
    ) -> Option<RenderPass<'a, (), (), (), (), false>>
where
        // Rust can't do this yet
        // If<{ has_render_attachment(USAGE) }>: True,
        // { has_render_attachment(USAGE) } = true,
    {
        let span = debug_span!("Secondary render pass").entered();

        if !has_render_attachment(USAGE) {
            debug!("Discarded");
            return None;
        }

        let pass = self.encoder.begin_render_pass(&RenderPassDescriptor {
            label: label!(),
            color_attachments: &[Some(RenderPassColorAttachment {
                view: target,
                resolve_target: None,
                ops: Operations {
                    load: LoadOp::Clear(self.clear_color.into()),
                    store: true,
                },
            })],
            depth_stencil_attachment: None,
        });

        debug!("Created");

        Some(RenderPass::new(pass, target.get_format(), span))
    }

    pub fn compute_pass(&mut self) -> ComputePass {
        let span = debug_span!("Compute pass").entered();

        let pass = self
            .encoder
            .begin_compute_pass(&ComputePassDescriptor { label: label!() });

        debug!("Created");

        ComputePass::new(pass, span)
    }

    pub fn get_dim(&self) -> (u32, u32) {
        self.main_dim
    }

    pub(crate) fn write_buffer(
        &mut self,
        target: &Buffer,
        offset: BufferAddress,
        size: BufferSize,
        device: &Device,
    ) -> BufferViewMut {
        self.belt
            .write_buffer(&mut self.encoder, target, offset, size, device)
    }
}

impl Drop for Frame {
    fn drop(&mut self) {
        if !self.has_primary {
            warn!("Frame is missing its primary render pass");
            self.primary_render_pass();
        }

        debug!("Frame dropped");

        // finish
        self.belt.finish();

        let (encoder, main_texture, belt) = (
            self.encoder.take(),
            self.main_texture.take(),
            self.belt.take(),
        );

        // submit
        self.queue.submit([encoder.finish()]);
        // self.queue.on_submitted_work_done(|| main_texture.present());
        main_texture.present();

        // reset
        self.belts.send(belt);
    }
}

//

pub(crate) struct SafeDrop<T> {
    inner: Option<T>,
}

impl<T> SafeDrop<T> {
    pub fn new(val: T) -> Self {
        Self { inner: Some(val) }
    }

    pub fn take(&mut self) -> T {
        self.inner.take().unwrap()
    }
}

impl<T> From<T> for SafeDrop<T> {
    fn from(val: T) -> Self {
        Self::new(val)
    }
}

impl<T> Deref for SafeDrop<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        self.inner.as_ref().unwrap()
    }
}

impl<T> DerefMut for SafeDrop<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.inner.as_mut().unwrap()
    }
}
