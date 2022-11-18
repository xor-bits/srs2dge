use crate::{label, target::Target};
use std::borrow::Cow;
use tracing::{debug, trace};
use wgpu::ShaderModuleDescriptor;

//

pub use naga::{FastHashMap, ShaderStage};
pub use wgpu::ShaderSource;

//

pub struct ShaderModule<'a> {
    pub(crate) inner: wgpu::ShaderModule,
    pub(crate) source: ShaderSource<'a>,
}

//

impl<'a> ShaderModule<'a> {
    pub fn new_wgsl_source(target: &Target, source: Cow<'a, str>) -> Result<Self, String> {
        Self::new(target, ShaderSource::Wgsl(source))
    }

    #[cfg(feature = "glsl")]
    pub fn new_glsl_source(
        target: &Target,
        source: Cow<'a, str>,
        stage: ShaderStage,
        defines: FastHashMap<String, String>,
    ) -> Result<Self, String> {
        Self::new(
            target,
            ShaderSource::Glsl {
                shader: source,
                stage,
                defines,
            },
        )
    }

    #[cfg(feature = "spirv")]
    pub fn new_spirv_source(target: &Target, source: &'a [u32]) -> Result<Self, String> {
        Self::new(target, ShaderSource::SpirV(Cow::Borrowed(source)))
    }

    pub fn new(target: &Target, source: ShaderSource<'a>) -> Result<Self, String> {
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
        descriptor: ShaderModuleDescriptor<'a>,
    ) -> Result<Self, String> {
        let source = match &descriptor.source {
            #[cfg(feature = "spirv")]
            ShaderSource::SpirV(spv) => ShaderSource::SpirV(spv.clone()),
            #[cfg(feature = "glsl")]
            ShaderSource::Glsl {
                shader,
                stage,
                defines,
            } => ShaderSource::Glsl {
                shader: shader.clone(),
                stage: stage.clone(),
                defines: defines.clone(),
            },
            ShaderSource::Wgsl(source) => ShaderSource::Wgsl(source.clone()),
            _ => todo!(),
        };

        let module = naga::front::wgsl::parse_str(match &source {
            ShaderSource::Wgsl(source) => source.as_ref(),
            _ => todo!(),
        })
        .unwrap();

        debug!("Module parsed to {module:?}");

        Ok(Self {
            source,
            inner: target.catch_error(|engine| engine.device.create_shader_module(descriptor))?,
        })
    }
}
