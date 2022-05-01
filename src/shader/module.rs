use crate::{label, target::Target};
use std::{borrow::Cow, sync::Arc};
use wgpu::ShaderModuleDescriptor;

//

#[cfg(not(feature = "glsl"))]
use wgpu::ShaderSource;
#[cfg(feature = "glsl")]
pub use {naga::ShaderStage, wgpu::ShaderSource};

//

pub struct ShaderModule {
    pub(crate) inner: wgpu::ShaderModule,
}

//

impl ShaderModule {
    pub fn new_wgsl_source(target: &Target, source: &str) -> Result<Arc<Self>, String> {
        Self::from_descriptor(
            target,
            ShaderModuleDescriptor {
                label: label!(),
                source: ShaderSource::Wgsl(Cow::Borrowed(source)),
            },
        )
    }

    #[cfg(feature = "glsl")]
    pub fn new_glsl_source(target: &Target, source: ShaderSource) -> Result<Arc<Self>, String> {
        Self::from_descriptor(
            target,
            ShaderModuleDescriptor {
                label: label!(),
                source,
            },
        )
    }

    fn from_descriptor(
        target: &Target,
        descriptor: ShaderModuleDescriptor,
    ) -> Result<Arc<Self>, String> {
        Ok(Self::from_module(target.catch_error(|engine| {
            engine.device.create_shader_module(&descriptor)
        })?))
    }

    fn from_module(inner: wgpu::ShaderModule) -> Arc<Self> {
        Arc::new(Self { inner })
    }
}
