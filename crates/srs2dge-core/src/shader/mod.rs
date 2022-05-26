use self::builder::ShaderBuilder;
use crate::buffer::{index::Index, vertex::Vertex};
use std::marker::PhantomData;
use wgpu::{BindGroup, BindGroupLayout, Device, RenderPipeline, TextureFormat};

//

pub mod builder;
pub mod layout;
pub mod module;
pub mod prelude;

//

pub struct Shader<V, I>
where
    V: Vertex,
    I: Index,
{
    pub(crate) pipeline: RenderPipeline,
    pub(crate) format: TextureFormat,

    _p: PhantomData<(V, I)>,
}

//

pub trait Layout<'a> {
    type Bindings;

    fn bind_group_layout(device: &Device) -> BindGroupLayout;
    fn bind_group(&self, bindings: Self::Bindings) -> BindGroup;
}

//

impl<V, I> Shader<V, I>
where
    V: Vertex,
    I: Index,
{
    pub fn builder<'s>() -> ShaderBuilder<'s, V, I> {
        ShaderBuilder::<'s, V, I>::new()
    }
}
