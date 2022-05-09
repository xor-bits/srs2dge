use super::{layout::AutoLayout, module::ShaderModule, Shader};
use crate::{
    buffer::{
        index::{DefaultIndex, Index},
        vertex::{DefaultVertex, Vertex},
    },
    label,
    target::Target,
};
use std::marker::PhantomData;
use wgpu::{
    BlendState, ColorTargetState, ColorWrites, FragmentState, FrontFace, MultisampleState,
    PipelineLayoutDescriptor, PolygonMode, PrimitiveState, PrimitiveTopology,
    RenderPipelineDescriptor, TextureFormat, VertexState,
};

//

pub struct ShaderBuilder<
    's,
    V = DefaultVertex,
    I = DefaultIndex,
    const VS: bool = false,
    const FS: bool = false,
    const FMT: bool = false,
> {
    pub(crate) vert: Option<(&'s ShaderModule<'s>, &'s str)>,
    pub(crate) frag: Option<(&'s ShaderModule<'s>, &'s str)>,
    format: Option<TextureFormat>,
    layout: Option<PipelineLayoutDescriptor<'s>>,
    topology: PrimitiveTopology,
    label: Option<&'s str>,

    _p: PhantomData<(V, I)>,
}

//

impl<'s, V, I, const VS: bool, const FS: bool, const FMT: bool> Default
    for ShaderBuilder<'s, V, I, VS, FS, FMT>
{
    fn default() -> Self {
        Self {
            vert: None,
            frag: None,
            format: None,
            layout: None,
            topology: PrimitiveTopology::TriangleStrip,
            label: label!(),

            _p: PhantomData::default(),
        }
    }
}

impl<'s, V, I, const VS: bool, const FS: bool, const FMT: bool>
    ShaderBuilder<'s, V, I, VS, FS, FMT>
{
    pub fn new() -> Self {
        Self::default()
    }

    fn pass<Vn, In, const VSN: bool, const FSN: bool, const FMTN: bool>(
        self,
    ) -> ShaderBuilder<'s, Vn, In, VSN, FSN, FMTN> {
        ShaderBuilder {
            vert: self.vert,
            frag: self.frag,
            format: self.format,
            layout: self.layout,
            topology: self.topology,
            label: self.label,

            _p: PhantomData::default(),
        }
    }

    pub fn with_vertex<'n: 's>(
        self,
        module: &'n ShaderModule,
        entry: &'n str,
    ) -> ShaderBuilder<'s, V, I, true, FS, FMT> {
        ShaderBuilder {
            vert: Some((module, entry)),
            ..self.pass()
        }
    }

    pub fn with_fragment<'n: 's>(
        self,
        module: &'n ShaderModule,
        entry: &'n str,
    ) -> ShaderBuilder<'s, V, I, VS, true, FMT> {
        ShaderBuilder {
            frag: Some((module, entry)),
            ..self.pass()
        }
    }

    pub fn with_format(self, format: TextureFormat) -> ShaderBuilder<'s, V, I, VS, FS, true> {
        ShaderBuilder {
            format: Some(format),
            ..self.pass()
        }
    }

    pub fn with_vertex_format<Vn>(self) -> ShaderBuilder<'s, Vn, I, VS, FS, true> {
        ShaderBuilder { ..self.pass() }
    }

    pub fn with_index_format<In>(self) -> ShaderBuilder<'s, V, In, VS, FS, true> {
        ShaderBuilder { ..self.pass() }
    }

    pub fn with_topology(mut self, topology: PrimitiveTopology) -> Self {
        self.topology = topology;
        self
    }

    pub fn with_label<'n: 's>(mut self, label: Option<&'n str>) -> Self {
        self.label = label;
        self
    }

    pub fn with_baked_layout<'l: 's>(
        self,
        layout: PipelineLayoutDescriptor<'l>,
    ) -> ShaderBuilder<'s, V, I, VS, FS, FMT> {
        ShaderBuilder {
            layout: Some(layout),
            ..self.pass()
        }
    }
}

impl<'s, V, I> ShaderBuilder<'s, V, I, true, true, true>
where
    V: Vertex,
    I: Index,
{
    pub fn build(self, target: &Target) -> Shader<V, I> {
        let (vert_mod, vert_entry) = self.vert.unwrap();
        let (frag_mod, frag_entry) = self.frag.unwrap();
        let format = self.format.unwrap();

        let layout = match self.layout {
            Some(l) => target.device.create_pipeline_layout(&l),
            None => {
                let a = AutoLayout::new(target, (vert_mod, vert_entry), (frag_mod, frag_entry));
                let a = a.get();
                target.device.create_pipeline_layout(&a.get())
            }
        };

        let pipeline = target
            .device
            .create_render_pipeline(&RenderPipelineDescriptor {
                label: self.label,
                layout: Some(&layout),
                vertex: VertexState {
                    module: &vert_mod.inner,
                    entry_point: vert_entry,
                    buffers: V::LAYOUT,
                },
                primitive: PrimitiveState {
                    topology: self.topology,
                    strip_index_format: Some(I::FORMAT),
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
            format,

            _p: PhantomData::default(),
        }
    }
}
