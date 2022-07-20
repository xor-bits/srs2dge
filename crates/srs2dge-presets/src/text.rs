use crate::Texture2DShader;
use srs2dge_core::{
    buffer::{DefaultIndex, Index},
    shader::module::ShaderModule,
    target::Target,
};
use std::{
    borrow::Cow,
    ops::{Deref, DerefMut},
};

//

type Internal<I> = Texture2DShader<true, I>;

//

#[derive(Debug)]
pub struct TextShader<I = DefaultIndex>
where
    I: Index,
{
    inner: Internal<I>,
}

impl<I> TextShader<I>
where
    I: Index,
{
    pub fn new(target: &Target) -> Self {
        let module = Self::built_in(target);
        Self {
            inner: Internal::new_custom(target, &module, "vs_main", &module, "fs_main"),
        }
    }

    pub fn new_custom_vert(
        target: &Target,
        module: &ShaderModule,
        entry: &str,
    ) -> Result<Self, String> {
        target.catch_error(|target| Self {
            inner: Internal::new_custom(target, module, entry, &Self::built_in(target), "fs_main"),
        })
    }

    pub fn new_custom_frag(
        target: &Target,
        module: &ShaderModule,
        entry: &str,
    ) -> Result<Self, String> {
        target.catch_error(|target| Self {
            inner: Internal::new_custom(target, &Self::built_in(target), "vs_main", module, entry),
        })
    }

    pub fn built_in(target: &Target) -> ShaderModule {
        ShaderModule::new_wgsl_source(target, Cow::Borrowed(srs2dge_res::shader::TEXT))
            .unwrap_or_else(|err| panic!("Built in shader compilation failed: {err}"))
    }
}

impl<I> Deref for TextShader<I>
where
    I: Index,
{
    type Target = Internal<I>;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl<I> DerefMut for TextShader<I>
where
    I: Index,
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
