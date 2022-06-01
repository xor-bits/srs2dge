use srs2dge_core::{
    buffer::{DefaultIndex, DefaultVertex, Index, UniformBuffer},
    glam::Mat4,
    label,
    shader::{module::ShaderModule, Layout, Shader},
    target::Target,
    wgpu::{
        BindGroup, BindGroupDescriptor, BindGroupEntry, BindGroupLayout, BindGroupLayoutDescriptor,
        BindGroupLayoutEntry, BindingType, BufferBindingType, Device, PipelineLayoutDescriptor,
        PrimitiveTopology, ShaderStages,
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
pub struct LineShader<I = DefaultIndex>
where
    I: Index,
{
    inner: Internal<I>,
    layout: BindGroupLayout,
    device: Arc<Device>,
}

impl<I> LineShader<I>
where
    I: Index,
{
    pub fn new(target: &Target, strip: bool) -> Self {
        let module =
            ShaderModule::new_wgsl_source(target, Cow::Borrowed(srs2dge_res::shader::COLORED_2D))
                .unwrap_or_else(|err| panic!("{err}"));

        let layout = Self::bind_group_layout(&target.get_device());

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
                .with_topology(if strip {
                    PrimitiveTopology::LineStrip
                } else {
                    PrimitiveTopology::LineList
                })
                .build(target),
            layout,

            device: target.get_device(),
        }
    }
}

impl<'a, I> Layout<'a> for LineShader<I>
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

impl<I> Deref for LineShader<I>
where
    I: Index,
{
    type Target = Internal<I>;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl<I> DerefMut for LineShader<I>
where
    I: Index,
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
