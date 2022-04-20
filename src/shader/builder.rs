use super::{module::ShaderModule, Shader};
use crate::{label, Engine};
use wgpu::{
    BlendState, ColorTargetState, ColorWrites, FragmentState, FrontFace, MultisampleState,
    PipelineLayoutDescriptor, PolygonMode, PrimitiveState, PrimitiveTopology,
    RenderPipelineDescriptor, TextureFormat, VertexAttribute, VertexBufferLayout, VertexFormat,
    VertexState, VertexStepMode,
};

//

pub struct ShaderBuilder<'s, const VS: bool, const FS: bool, const FMT: bool, const L: bool> {
    vert: Option<(&'s ShaderModule, &'s str)>,
    frag: Option<(&'s ShaderModule, &'s str)>,
    format: Option<TextureFormat>,
    layout: Option<PipelineLayoutDescriptor<'s>>,
    // bind_layout: Vec<BindGroupLayoutDescriptor<'s>>,
}

//

impl<'s, const VS: bool, const FS: bool, const FMT: bool, const L: bool>
    ShaderBuilder<'s, VS, FS, FMT, L>
{
    pub fn new() -> ShaderBuilder<'s, false, false, false, false> {
        ShaderBuilder {
            vert: None,
            frag: None,
            format: None,
            layout: None,
            // bind_layout: vec![],
        }
    }

    fn pass<const VSN: bool, const FSN: bool, const FMTN: bool, const LN: bool>(
        self,
    ) -> ShaderBuilder<'s, VSN, FSN, FMTN, LN> {
        ShaderBuilder {
            vert: self.vert,
            frag: self.frag,
            format: self.format,
            layout: self.layout,
            // bind_layout: self.bind_layout,
        }
    }

    pub fn with_vertex<'n: 's>(
        self,
        module: &'n ShaderModule,
        entry: &'n str,
    ) -> ShaderBuilder<'s, true, FS, FMT, L> {
        ShaderBuilder {
            vert: Some((module, entry)),
            ..self.pass()
        }
    }

    pub fn with_fragment<'n: 's>(
        self,
        module: &'n ShaderModule,
        entry: &'n str,
    ) -> ShaderBuilder<'s, VS, true, FMT, L> {
        ShaderBuilder {
            frag: Some((module, entry)),
            ..self.pass()
        }
    }

    pub fn with_format(self, format: TextureFormat) -> ShaderBuilder<'s, VS, FS, true, L> {
        ShaderBuilder {
            format: Some(format),
            ..self.pass()
        }
    }

    /* pub(crate) fn with_bind_layout<'l: 's>(self, layout: BindGroupLayoutDescriptor<'l>) -> Self {
        self.bind_layout.push(layout);
        self
    } */

    // TODO: allow custom shaders
    pub(crate) fn with_layout<'l: 's>(
        self,
        layout: PipelineLayoutDescriptor<'l>,
    ) -> ShaderBuilder<'s, VS, FS, FMT, true> {
        ShaderBuilder {
            layout: Some(layout),
            ..self.pass()
        }
    }
}

impl<'s> ShaderBuilder<'s, true, true, true, true> {
    pub fn build(self, engine: &Engine) -> Shader {
        let (vert_mod, vert_entry) = self.vert.unwrap();
        let (frag_mod, frag_entry) = self.frag.unwrap();
        let format = self.format.unwrap();

        let layout = engine.device.create_pipeline_layout(&self.layout.unwrap());

        let pipeline = engine
            .device
            .create_render_pipeline(&RenderPipelineDescriptor {
                label: label!(),
                layout: Some(&layout),
                vertex: VertexState {
                    module: &vert_mod.inner,
                    entry_point: vert_entry,
                    buffers: &[VertexBufferLayout {
                        array_stride: 24,
                        step_mode: VertexStepMode::Vertex,
                        attributes: &[
                            VertexAttribute {
                                format: VertexFormat::Float32x2,
                                offset: 0,
                                shader_location: 0,
                            },
                            VertexAttribute {
                                format: VertexFormat::Float32x4,
                                offset: 8,
                                shader_location: 1,
                            },
                        ],
                    }],
                },
                primitive: PrimitiveState {
                    topology: PrimitiveTopology::TriangleList,
                    strip_index_format: None,
                    front_face: FrontFace::Ccw,
                    cull_mode: None,
                    unclipped_depth: false,
                    polygon_mode: PolygonMode::Fill,
                    conservative: false,
                },
                depth_stencil: None,
                multisample: MultisampleState::default(),
                fragment: Some(FragmentState {
                    module: &frag_mod.inner,
                    entry_point: frag_entry,
                    targets: &[ColorTargetState {
                        format,
                        blend: Some(BlendState::ALPHA_BLENDING),
                        write_mask: ColorWrites::ALL,
                    }],
                }),
                multiview: None,
            });

        Shader {
            pipeline,
            layout,

            format,
        }
    }
}
