use crate::{
    buffer::UniformBuffer, label, prelude::Shader, shader::module::ShaderModule, target::Target,
};
use bytemuck::Pod;
use std::{
    ops::{Deref, DerefMut},
    sync::Arc,
};
use wgpu::{
    AddressMode, BindGroup, BindGroupDescriptor, BindGroupEntry, BindGroupLayout,
    BindGroupLayoutDescriptor, BindGroupLayoutEntry, BindingResource, BindingType,
    BufferBindingType, Device, FilterMode, PipelineLayoutDescriptor, Sampler, SamplerBindingType,
    SamplerDescriptor, ShaderStages, TextureSampleType, TextureView, TextureViewDimension,
};

//

const SHADER_SOURCE: &str = include_str!("texture_2d.wgsl");

//

pub struct Texture2DShader {
    inner: Shader,
    bind_layout: BindGroupLayout,
    sampler: Sampler,

    device: Arc<Device>,
}

impl Texture2DShader {
    pub fn new(target: &Target) -> Self {
        let module = ShaderModule::new_wgsl_source(target, SHADER_SOURCE)
            .unwrap_or_else(|err| panic!("{err}"));

        let bind_layout = target
            .device
            .create_bind_group_layout(&BindGroupLayoutDescriptor {
                label: label!(),
                entries: &[
                    BindGroupLayoutEntry {
                        binding: 0,
                        visibility: ShaderStages::VERTEX,
                        ty: BindingType::Buffer {
                            ty: BufferBindingType::Uniform,
                            has_dynamic_offset: false,
                            min_binding_size: None,
                        },
                        count: None,
                    },
                    BindGroupLayoutEntry {
                        binding: 1,
                        visibility: ShaderStages::FRAGMENT,
                        ty: BindingType::Texture {
                            sample_type: TextureSampleType::Float { filterable: false },
                            view_dimension: TextureViewDimension::D2,
                            multisampled: false,
                        },
                        count: None,
                    },
                    BindGroupLayoutEntry {
                        binding: 2,
                        visibility: ShaderStages::FRAGMENT,
                        ty: BindingType::Sampler(SamplerBindingType::NonFiltering),
                        count: None,
                    },
                ],
            });

        let sampler = target.device.create_sampler(&SamplerDescriptor {
            label: label!(),
            address_mode_u: AddressMode::ClampToEdge,
            address_mode_v: AddressMode::ClampToEdge,
            address_mode_w: AddressMode::ClampToEdge,
            mag_filter: FilterMode::Nearest,
            min_filter: FilterMode::Nearest,
            mipmap_filter: FilterMode::Nearest,
            lod_min_clamp: 0.0,
            lod_max_clamp: f32::MAX,
            compare: None,
            anisotropy_clamp: None,
            border_color: None,
        });

        Self {
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
            sampler,

            device: target.device.clone(),
        }
    }

    pub fn bind_group<T: Pod>(
        &self,
        uniform: &UniformBuffer<T>,
        texture: &TextureView,
    ) -> BindGroup {
        self.device.create_bind_group(&BindGroupDescriptor {
            label: label!(),
            layout: &self.bind_layout,
            entries: &[
                BindGroupEntry {
                    binding: 0,
                    resource: uniform.get_buffer().as_entire_binding(),
                },
                BindGroupEntry {
                    binding: 1,
                    resource: BindingResource::TextureView(texture),
                },
                BindGroupEntry {
                    binding: 2,
                    resource: BindingResource::Sampler(&self.sampler),
                },
            ],
        })
    }
}

impl Deref for Texture2DShader {
    type Target = Shader;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl DerefMut for Texture2DShader {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
