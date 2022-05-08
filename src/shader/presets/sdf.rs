use crate::{
    buffer::{DefaultIndex, DefaultVertex, Index, UniformBuffer},
    label,
    prelude::Shader,
    shader::{module::ShaderModule, Layout},
    target::Target,
};
use glam::Mat4;
use std::{
    borrow::Cow,
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

const SHADER_SOURCE: &str = include_str!("sdf.wgsl");

//

type Internal<I> = Shader<DefaultVertex, I>;

//

pub struct SdfShader<I = DefaultIndex>
where
    I: Index,
{
    inner: Internal<I>,
    layout: BindGroupLayout,
    sampler: Sampler,

    device: Arc<Device>,
}

impl<I> SdfShader<I>
where
    I: Index,
{
    pub fn new(target: &Target) -> Self {
        let module = ShaderModule::new_wgsl_source(target, Cow::Borrowed(SHADER_SOURCE))
            .unwrap_or_else(|err| panic!("{err}"));

        let layout = Self::bind_group_layout(&target.device);

        let sampler = target.device.create_sampler(&SamplerDescriptor {
            label: label!(),
            address_mode_u: AddressMode::ClampToEdge,
            address_mode_v: AddressMode::ClampToEdge,
            address_mode_w: AddressMode::ClampToEdge,
            mag_filter: FilterMode::Linear, // important!
            min_filter: FilterMode::Linear,
            mipmap_filter: FilterMode::Linear,
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
                .with_format(target.get_format())
                .with_baked_layout(PipelineLayoutDescriptor {
                    label: label!(),
                    bind_group_layouts: &[&layout],
                    push_constant_ranges: &[],
                })
                .with_label(label!())
                .build(target),
            layout,
            sampler,

            device: target.device.clone(),
        }
    }
}

impl<I> Layout for SdfShader<I>
where
    I: Index,
{
    type Bindings<'a> = (&'a UniformBuffer<Mat4>, &'a TextureView);

    fn bind_group_layout(device: &Device) -> BindGroupLayout {
        device.create_bind_group_layout(&BindGroupLayoutDescriptor {
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
                        sample_type: TextureSampleType::Float { filterable: true }, // again, very important!
                        view_dimension: TextureViewDimension::D2,
                        multisampled: false,
                    },
                    count: None,
                },
                BindGroupLayoutEntry {
                    binding: 2,
                    visibility: ShaderStages::FRAGMENT,
                    ty: BindingType::Sampler(SamplerBindingType::Filtering), // yet again, super important!
                    count: None,
                },
            ],
        })
    }

    fn bind_group(&self, (uniform, texture): Self::Bindings<'_>) -> BindGroup {
        self.device.create_bind_group(&BindGroupDescriptor {
            label: label!(),
            layout: &self.layout,
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

impl<I> Deref for SdfShader<I>
where
    I: Index,
{
    type Target = Internal<I>;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl<I> DerefMut for SdfShader<I>
where
    I: Index,
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}