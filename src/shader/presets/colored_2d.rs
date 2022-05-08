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
    BindGroup, BindGroupDescriptor, BindGroupEntry, BindGroupLayout, BindGroupLayoutDescriptor,
    BindGroupLayoutEntry, BindingType, BufferBindingType, Device, PipelineLayoutDescriptor,
    ShaderStages,
};

//

const SHADER_SOURCE: &str = include_str!("colored_2d.wgsl");

//

type Internal<I> = Shader<DefaultVertex, I>;

//

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
        let module = ShaderModule::new_wgsl_source(target, Cow::Borrowed(SHADER_SOURCE))
            .unwrap_or_else(|err| panic!("{err}"));

        let layout = Self::bind_group_layout(&target.device);

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

            device: target.device.clone(),
        }
    }
}

impl<I> Layout for Colored2DShader<I>
where
    I: Index,
{
    type Bindings<'a> = &'a UniformBuffer<Mat4>;

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

    fn bind_group(&self, bindings: Self::Bindings<'_>) -> BindGroup {
        self.device.create_bind_group(&BindGroupDescriptor {
            label: label!(),
            layout: &self.layout,
            entries: &[BindGroupEntry {
                binding: 0,
                resource: bindings.get_buffer().as_entire_binding(),
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
