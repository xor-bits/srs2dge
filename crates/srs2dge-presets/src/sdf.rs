use bytemuck::{Pod, Zeroable};
use srs2dge_core::{
    buffer::{DefaultIndex, DefaultVertex, Index, UniformBuffer},
    glam::Mat4,
    label,
    shader::{module::ShaderModule, Layout, Shader},
    target::Target,
    wgpu::{
        AddressMode, BindGroup, BindGroupDescriptor, BindGroupEntry, BindGroupLayout,
        BindGroupLayoutDescriptor, BindGroupLayoutEntry, BindingResource, BindingType,
        BufferBindingType, Device, FilterMode, PipelineLayoutDescriptor, Sampler,
        SamplerBindingType, SamplerDescriptor, ShaderStages, TextureSampleType, TextureView,
        TextureViewDimension,
    },
};
use std::{
    borrow::Cow,
    ops::{Deref, DerefMut},
    sync::Arc,
};

//

type Internal<I> = Shader<DefaultVertex, I>;

//

#[derive(Debug, Clone, Copy, PartialEq, Zeroable, Pod)]
#[repr(C, align(16))]
pub struct SdfUniform {
    pub mvp: [[f32; 4]; 4],
    pub weight: f32,
    pub anti_alias: f32,
    pub border: f32,
    pub _pad: f32,
}

impl Default for SdfUniform {
    fn default() -> Self {
        Self {
            mvp: Mat4::IDENTITY.to_cols_array_2d(),
            weight: 0.0,
            anti_alias: 0.01,
            border: 0.15,
            _pad: 0.0,
        }
    }
}

impl SdfUniform {
    pub fn new(mvp: Mat4, weight: f32, anti_alias: f32, border: f32) -> Self {
        Self {
            mvp: mvp.to_cols_array_2d(),
            weight,
            anti_alias,
            border,
            _pad: 0.0,
        }
    }

    pub fn new_defaults(mvp: Mat4) -> Self {
        Self {
            mvp: mvp.to_cols_array_2d(),
            ..Default::default()
        }
    }

    pub fn get_mvp(&self) -> Mat4 {
        Mat4::from_cols_array_2d(&self.mvp)
    }

    pub fn set_mvp(&mut self, mvp: Mat4) {
        self.mvp = mvp.to_cols_array_2d();
    }
}

#[derive(Debug)]
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
        let module = ShaderModule::new_wgsl_source(target, Cow::Borrowed(srs2dge_res::shader::SDF))
            .unwrap_or_else(|err| panic!("{err}"));

        let layout = Self::bind_group_layout(&target.get_device());

        let sampler = target.get_device().create_sampler(&SamplerDescriptor {
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

            device: target.get_device(),
        }
    }
}

impl<'a, I> Layout<'a> for SdfShader<I>
where
    I: Index,
{
    type Bindings = (&'a UniformBuffer<SdfUniform>, &'a TextureView);

    fn bind_group_layout(device: &Device) -> BindGroupLayout {
        device.create_bind_group_layout(&BindGroupLayoutDescriptor {
            label: label!(),
            entries: &[
                BindGroupLayoutEntry {
                    binding: 0,
                    visibility: ShaderStages::VERTEX | ShaderStages::FRAGMENT,
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

    fn bind_group(&self, (uniform, texture): Self::Bindings) -> BindGroup {
        self.device.create_bind_group(&BindGroupDescriptor {
            label: label!(),
            layout: &self.layout,
            entries: &[
                BindGroupEntry {
                    binding: 0,
                    resource: uniform.inner().as_entire_binding(),
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
