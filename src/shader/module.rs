use crate::{label, Engine};
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
    pub fn new_wgsl_source(engine: &Engine, source: &str) -> Result<Arc<Self>, String> {
        /* let validated =
            naga::front::wgsl::parse_str(source).map_err(|err| err.emit_to_string(source))?;

        log::debug!("validated: {validated:#?}");
        for (h, v) in validated.global_variables.iter() {
            log::debug!("handle: {h:?}, var: {v:?}");
        } */

        Ok(Self::from_module(engine.device.create_shader_module(
            &ShaderModuleDescriptor {
                label: label!(),
                source: ShaderSource::Wgsl(Cow::Borrowed(source)),
            },
        )))
    }

    #[cfg(feature = "glsl")]
    pub fn new_glsl_source(engine: &Engine, source: ShaderSource) -> Arc<Self> {
        Self::from_module(engine.device.create_shader_module(&ShaderModuleDescriptor {
            label: label!(),
            source,
        }))
    }

    fn from_module(inner: wgpu::ShaderModule) -> Arc<Self> {
        Arc::new(Self { inner })
    }
}
