use super::{module::ShaderModule, Shader};
use crate::{buffer::UniformBuffer, label, target::Target};
use bytemuck::Pod;
use std::{
    ops::{Deref, DerefMut},
    sync::Arc,
};
use wgpu::{
    BindGroup, BindGroupDescriptor, BindGroupEntry, BindGroupLayout, BindGroupLayoutDescriptor,
    BindGroupLayoutEntry, BindingType, BufferBindingType, Device, PipelineLayoutDescriptor,
    ShaderStages,
};

//

static_res::static_res! {
    "res/**/*.{wgsl}"
}

//

pub struct Colored2DShader {
    inner: Shader,
    bind_layout: BindGroupLayout,

    device: Arc<Device>,
}

impl Colored2DShader {
    pub fn new(target: &Target) -> Self {
        let module = ShaderModule::new_wgsl_source(
            target,
            std::str::from_utf8(res::shader::colored_2d_wgsl).unwrap(),
        )
        .unwrap_or_else(|err| panic!("{err}"));

        let bind_layout = target
            .device
            .create_bind_group_layout(&BindGroupLayoutDescriptor {
                label: label!(),
                entries: &[BindGroupLayoutEntry {
                    binding: 0,
                    visibility: ShaderStages::VERTEX,
                    ty: BindingType::Buffer {
                        ty: BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                }],
            });

        Colored2DShader {
            inner: Shader::builder()
                .with_vertex(&module, "vs_main")
                .with_fragment(&module, "fs_main")
                .with_format(target.surface.format())
                .with_layout(PipelineLayoutDescriptor {
                    label: label!(),
                    bind_group_layouts: &[&bind_layout],
                    push_constant_ranges: &[],
                })
                .build(target),
            bind_layout,

            device: target.device.clone(),
        }
    }

    pub fn bind_group<T: Pod>(&self, uniform: &UniformBuffer<T>) -> BindGroup {
        self.device.create_bind_group(&BindGroupDescriptor {
            label: label!(),
            layout: &self.bind_layout,
            entries: &[BindGroupEntry {
                binding: 0,
                resource: uniform.get_buffer().as_entire_binding(),
            }],
        })
    }
}

impl Deref for Colored2DShader {
    type Target = Shader;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl DerefMut for Colored2DShader {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
