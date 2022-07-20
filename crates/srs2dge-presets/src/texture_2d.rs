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

#[derive(Debug)]
pub struct Texture2DShader<const FILTER: bool = false, I = DefaultIndex>
where
    I: Index,
{
    inner: Internal<I>,
    layout: BindGroupLayout,
    sampler: Sampler,

    device: Arc<Device>,
}

impl<const FILTER: bool, I> Texture2DShader<FILTER, I>
where
    I: Index,
{
    pub fn new(target: &Target) -> Self {
        let module = Self::built_in(target);
        Self::new_custom(target, &module, "vs_main", &module, "fs_main")
    }

    pub fn new_custom_vert(
        target: &Target,
        module: &ShaderModule,
        entry: &str,
    ) -> Result<Self, String> {
        target.catch_error(|target| {
            Self::new_custom(target, module, entry, &Self::built_in(target), "fs_main")
        })
    }

    pub fn new_custom_frag(
        target: &Target,
        module: &ShaderModule,
        entry: &str,
    ) -> Result<Self, String> {
        target.catch_error(|target| {
            Self::new_custom(target, &Self::built_in(target), "vs_main", module, entry)
        })
    }

    pub fn built_in(target: &Target) -> ShaderModule {
        ShaderModule::new_wgsl_source(target, Cow::Borrowed(srs2dge_res::shader::TEXTURE_2D))
            .unwrap_or_else(|err| panic!("Built in shader compilation failed: {err}"))
    }

    pub fn new_custom(
        target: &Target,
        vert_module: &ShaderModule,
        vert_entry: &str,
        frag_module: &ShaderModule,
        frag_entry: &str,
    ) -> Self {
        let layout = Self::bind_group_layout(&target.get_device());

        let filter = if FILTER {
            FilterMode::Linear
        } else {
            FilterMode::Nearest
        };
        let sampler = target.get_device().create_sampler(&SamplerDescriptor {
            label: label!(),
            address_mode_u: AddressMode::ClampToEdge,
            address_mode_v: AddressMode::ClampToEdge,
            address_mode_w: AddressMode::ClampToEdge,
            mag_filter: filter,
            min_filter: filter,
            mipmap_filter: filter,
            lod_min_clamp: 0.0,
            lod_max_clamp: f32::MAX,
            compare: None,
            anisotropy_clamp: None,
            border_color: None,
        });

        Self {
            inner: Shader::builder()
                .with_vertex(vert_module, vert_entry)
                .with_fragment(frag_module, frag_entry)
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

impl<'a, const FILTER: bool, I> Layout<'a> for Texture2DShader<FILTER, I>
where
    I: Index,
{
    type Bindings = (&'a UniformBuffer<Mat4>, &'a TextureView);

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
                        sample_type: TextureSampleType::Float { filterable: FILTER },
                        view_dimension: TextureViewDimension::D2,
                        multisampled: false,
                    },
                    count: None,
                },
                BindGroupLayoutEntry {
                    binding: 2,
                    visibility: ShaderStages::FRAGMENT,
                    ty: BindingType::Sampler(if FILTER {
                        SamplerBindingType::Filtering
                    } else {
                        SamplerBindingType::NonFiltering
                    }),
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

impl<const FILTER: bool, I> Deref for Texture2DShader<FILTER, I>
where
    I: Index,
{
    type Target = Internal<I>;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl<const FILTER: bool, I> DerefMut for Texture2DShader<FILTER, I>
where
    I: Index,
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
