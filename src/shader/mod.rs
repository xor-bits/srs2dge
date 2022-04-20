use self::builder::ShaderBuilder;
use crate::frame::render_pass::RenderPass;
use wgpu::{PipelineLayout, RenderPipeline, TextureFormat};

//

pub mod builder;
pub mod module;
pub mod presets;

//

pub struct Shader {
    pipeline: RenderPipeline,
    layout: PipelineLayout,

    format: TextureFormat,
}

//

impl Shader {
    pub fn builder<'s>() -> ShaderBuilder<'s, false, false, false, false> {
        ShaderBuilder::<false, false, false, false>::new()
    }

    pub(crate) fn bind<'a, const B: bool>(&'a self, render_pass: &mut RenderPass<'a, B>) {
        let _ = &self.layout;

        if render_pass.format != self.format {
            panic!("Shader output incompatible with this render target");
        } else {
            render_pass.inner.set_pipeline(&self.pipeline);
        }
    }
}
