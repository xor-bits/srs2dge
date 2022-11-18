use self::builder::ShaderBuilder;
use crate::buffer::{index::Index, vertex::Vertex};
use std::marker::PhantomData;
use wgpu::{BindGroup, BindGroupLayout, Device, RenderPipeline, TextureFormat};

//

//pub mod builder;
//pub mod layout;
//pub mod module;
//pub mod prelude;

//

/// Vertex shader stage
pub struct GraphicsShader<I: ShaderIO, O: ShaderIO, B> {
    _p: PhantomData<(I, O, B)>,
}

impl<I: ShaderIO, O: ShaderIO, B> GraphicsShader<I, O, B> {
    pub fn new<VO: ShaderIO>(vert: Vert<I, VO>, frag: Frag<VO, O>) -> Self {
        Self {
            _p: Default::default(),
        }
    }
}

pub struct Vert<I: ShaderIO, O: ShaderIO> {
    _p: PhantomData<(I, O)>,
}

pub struct Frag<I: ShaderIO, O: ShaderIO> {
    _p: PhantomData<(I, O)>,
}

impl<I: ShaderIO, O: ShaderIO> Vert<I, O> {
    pub fn new(module: &str, entry: &str) -> Self {
        Self {
            _p: Default::default(),
        }
    }
}

impl<I: ShaderIO, O: ShaderIO> Frag<I, O> {
    pub fn new(module: &str, entry: &str) -> Self {
        Self {
            _p: Default::default(),
        }
    }
}

impl<I: ShaderIO, O: ShaderIO> Module for Vert<I, O> {
    type Input = I;
    type Output = O;
}

impl<I: ShaderIO, O: ShaderIO> Module for Frag<I, O> {
    type Input = I;
    type Output = O;
}

pub trait ShaderIO {}

pub trait Module {
    type Input: ShaderIO;
    type Output: ShaderIO;
}

pub trait Bindings {}

pub trait Binding {}

pub struct UniformBufferBinding;

pub struct TextureBinding;

pub struct SamplerBinding;

pub struct Ignored;

//

#[cfg(test)]
mod tests {
    use glam::{Vec2, Vec4};
    use wgpu::ShaderStages;

    use crate::{
        batch::DefaultVertex,
        color::Color,
        shader::{Frag, GraphicsShader, ShaderIO, Vert},
    };

    use super::UniformBufferBinding;

    fn main() {
        struct Intermediate {
            col: Vec4,
            uv: Vec2,
        }

        impl ShaderIO for Intermediate {}
        impl ShaderIO for DefaultVertex {}
        impl ShaderIO for Color {}

        let vert: Vert<DefaultVertex, Intermediate> = Vert::new("module", "vs_main");

        let frag: Frag<Color, Color> = Frag::new("module", "fs_main");

        let pipeline: GraphicsShader<DefaultVertex, Color, ()> = GraphicsShader::new(vert, frag);

        ShaderStages::VERTEX;
    }
}
