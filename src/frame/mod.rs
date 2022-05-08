use self::{compute_pass::ComputePass, render_pass::RenderPass};
use crate::{label, target::surface::Surface};
use glam::Vec4;
use std::sync::Arc;
use wgpu::{
    util::StagingBelt, Buffer, BufferAddress, BufferSize, BufferViewMut, Color, CommandEncoder,
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

    encoder: Option<CommandEncoder>,

    queue: Arc<Queue>,

    clear_color: Vec4,

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

        let encoder = device.create_command_encoder(&CommandEncoderDescriptor { label: label!() });

        let main_texture = Some(main_texture);
        let encoder = Some(encoder);

        Self {
            main_texture,
            main_view,
            main_format,

            encoder,

            queue,

            clear_color: Vec4::new(0.1, 0.1, 0.1, 1.0),

            belt,
        }
    }
}

impl Frame {
    pub fn encoder(&mut self) -> &mut CommandEncoder {
        self.encoder.as_mut().expect("Frame was dropped")
    }

    pub fn set_clear_color(&mut self, color: Vec4) {
        self.clear_color = color;
    }

    pub fn main_render_pass(&mut self) -> RenderPass<(), (), (), (), false> {
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
                        load: LoadOp::Clear(Color {
                            r: self.clear_color.x as _,
                            g: self.clear_color.y as _,
                            b: self.clear_color.z as _,
                            a: self.clear_color.w as _,
                        }),
                        store: true,
                    },
                }],
                depth_stencil_attachment: None,
            });

        RenderPass::new(pass, self.main_format)
    }

    pub fn compute_pass(&mut self) -> ComputePass {
        let pass = self
            .encoder
            .as_mut()
            .expect("Frame was dropped")
            .begin_compute_pass(&ComputePassDescriptor { label: label!() });

        ComputePass::new(pass)
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
