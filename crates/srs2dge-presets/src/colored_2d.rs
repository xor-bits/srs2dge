use srs2dge_core::{
    buffer::{DefaultIndex, DefaultVertex, Index, UniformBuffer},
    glam::Mat4,
    label,
    shader::{module::ShaderModule, Layout, Shader},
    target::Target,
    wgpu::{
        BindGroup, BindGroupDescriptor, BindGroupEntry, BindGroupLayout, BindGroupLayoutDescriptor,
        BindGroupLayoutEntry, BindingType, BufferBindingType, Device, PipelineLayoutDescriptor,
        ShaderStages,
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
pub struct Colored2DShader<I = DefaultIndex>
where
    I: Index,
{
    inner: Internal<I>,
    layout: BindGroupLayout,
    device: Arc<Device>,
}

impl<I> Colored2DShader<I>
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
        ShaderModule::new_wgsl_source(target, Cow::Borrowed(srs2dge_res::shader::COLORED_2D))
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

            device: target.get_device(),
        }
    }
}

impl<'a, I> Layout<'a> for Colored2DShader<I>
where
    I: Index,
{
    type Bindings = &'a UniformBuffer<Mat4>;

    fn bind_group_layout(device: &Device) -> BindGroupLayout {
        device.create_bind_group_layout(&BindGroupLayoutDescriptor {
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
        })
    }

    fn bind_group(&self, bindings: Self::Bindings) -> BindGroup {
        self.device.create_bind_group(&BindGroupDescriptor {
            label: label!(),
            layout: &self.layout,
            entries: &[BindGroupEntry {
                binding: 0,
                resource: bindings.inner().as_entire_binding(),
            }],
        })
    }
}

impl<I> Deref for Colored2DShader<I>
where
    I: Index,
{
    type Target = Internal<I>;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl<I> DerefMut for Colored2DShader<I>
where
    I: Index,
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
