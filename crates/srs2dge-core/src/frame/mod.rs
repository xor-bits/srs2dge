use self::{compute_pass::ComputePass, render_pass::RenderPass};
use crate::{
    color::Color,
    label,
    target::surface::Surface,
    texture::{has_render_attachment, Texture},
};
use std::sync::Arc;
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
    main_texture: Option<SurfaceTexture>,
    main_view: TextureView,
    main_format: TextureFormat,
    main_dim: (u32, u32),

    encoder: Option<CommandEncoder>,

    queue: Arc<Queue>,

    clear_color: Color,

    pub(crate) belt: StagingBelt,
}

//

impl Frame {
    pub fn new(
        device: &Device,
        queue: Arc<Queue>,
        surface: &mut Surface,
        belt: StagingBelt,
    ) -> Self {
        let main_texture = surface.acquire();
        let main_view = main_texture.texture.create_view(&TextureViewDescriptor {
            label: label!(),
            ..Default::default()
        });
        let main_format = surface.format();
        let main_dim = surface.get_dim();

        let encoder = device.create_command_encoder(&CommandEncoderDescriptor { label: label!() });

        let main_texture = Some(main_texture);
        let encoder = Some(encoder);

        Self {
            main_texture,
            main_view,
            main_format,
            main_dim,

            encoder,

            queue,

            clear_color: Color::CLEAR_COLOR,

            belt,
        }
    }
}

impl Frame {
    pub fn encoder(&mut self) -> &mut CommandEncoder {
        self.encoder.as_mut().expect("Frame was dropped")
    }

    pub fn set_clear_color(&mut self, color: Color) {
        self.clear_color = color;
    }

    pub fn primary_render_pass(&mut self) -> RenderPass<(), (), (), (), false> {
        let pass = self
            .encoder
            .as_mut()
            .expect("Frame was dropped")
            .begin_render_pass(&RenderPassDescriptor {
                label: label!(),
                color_attachments: &[RenderPassColorAttachment {
                    view: &self.main_view,
                    resolve_target: None,
                    ops: Operations {
                        load: LoadOp::Clear(self.clear_color.into()),
                        store: true,
                    },
                }],
                depth_stencil_attachment: None,
            });

        RenderPass::new(pass, self.main_format)
    }

    pub fn secondary_render_pass<'a, const USAGE: u32>(
        &'a mut self,
        target: &'a Texture<USAGE>,
    ) -> Option<RenderPass<'a, (), (), (), (), false>>
// where // Rust can't do this yet
//     If<{ has_render_attachment(USAGE) }>: True,
    {
        if !has_render_attachment(USAGE) {
            return None;
        }

        let pass = self
            .encoder
            .as_mut()
            .expect("Frame was dropped")
            .begin_render_pass(&RenderPassDescriptor {
                label: label!(),
                color_attachments: &[RenderPassColorAttachment {
                    view: target,
                    resolve_target: None,
                    ops: Operations {
                        load: LoadOp::Clear(self.clear_color.into()),
                        store: true,
                    },
                }],
                depth_stencil_attachment: None,
            });

        Some(RenderPass::new(pass, target.get_format()))
    }

    pub fn compute_pass(&mut self) -> ComputePass {
        let pass = self
            .encoder
            .as_mut()
            .expect("Frame was dropped")
            .begin_compute_pass(&ComputePassDescriptor { label: label!() });

        ComputePass::new(pass)
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
        self.belt.write_buffer(
            self.encoder.as_mut().expect("Frame was dropped"),
            target,
            offset,
            size,
            device,
        )
    }

    pub(crate) fn finish(mut self) -> StagingBelt {
        self.belt.finish();
        self.queue.submit([self
            .encoder
            .take()
            .expect("Frame was dropped twice")
            .finish()]);
        self.main_texture
            .take()
            .expect("Frame was dropped twice")
            .present();
        self.belt
    }
}
