use srs2dge_core::{
    color::Color,
    glam::{Mat4, Vec2},
    prelude::{
        DefaultVertex, Frame, IndexBuffer, Layout, Mesh, RenderPass, UniformBuffer, VertexBuffer,
    },
    target::Target,
    wgpu::{BindGroup, PrimitiveTopology},
};
use srs2dge_presets::LineShader;
use std::array::IntoIter;

//

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct GizmosLine {
    pub from: Vec2,
    pub to: Vec2,
    pub col: Color,
}

//

impl GizmosLine {
    pub fn new(from: Vec2, to: Vec2, col: Color) -> Self {
        Self { from, to, col }
    }
}

impl Mesh<DefaultVertex> for GizmosLine {
    const PRIM: PrimitiveTopology = PrimitiveTopology::LineList;

    type VertexIter = IntoIter<DefaultVertex, 2>;
    type IndexIter = IntoIter<u32, 2>;

    fn vertices(&self) -> Self::VertexIter {
        [
            DefaultVertex::new(self.from, self.col, Vec2::ZERO),
            DefaultVertex::new(self.to, self.col, Vec2::ZERO),
        ]
        .into_iter()
    }

    fn indices(&self, offset: u32) -> Self::IndexIter {
        IntoIterator::into_iter([offset, offset + 1])
    }

    fn index_step(&self) -> u32 {
        2
    }
}

//

pub(super) struct GizmosLines {
    lines: Vec<GizmosLine>,

    vbo: VertexBuffer,
    ibo: IndexBuffer,
    ibo_len: u32,
    shader: LineShader,
    bind_group: BindGroup,
}

//

impl GizmosLines {
    pub fn new(target: &Target, ubo: &UniformBuffer<Mat4>) -> Self {
        let shader = LineShader::new(target, false);
        let bind_group = shader.bind_group(ubo);

        Self {
            lines: vec![],

            vbo: VertexBuffer::new(target, 2),
            ibo: IndexBuffer::new(target, 2),
            ibo_len: 0,
            shader,
            bind_group,
        }
    }

    #[inline(always)]
    pub fn push(&mut self, line: GizmosLine) {
        self.lines.push(line);
    }

    pub fn prepare(&mut self, target: &mut Target, frame: &mut Frame) {
        let vbo_data: Vec<DefaultVertex> =
            self.lines.iter().flat_map(|line| line.vertices()).collect();
        let mut i = 0;
        let ibo_data: Vec<u32> = self
            .lines
            .drain(..)
            .flat_map(|line| {
                let offset = i;
                i += line.index_step();
                line.indices(offset)
            })
            .collect();

        if self.vbo.capacity() < vbo_data.len() {
            self.vbo = VertexBuffer::new(target, vbo_data.len() * 2);
        }
        if self.ibo.capacity() < ibo_data.len() {
            self.ibo = IndexBuffer::new(target, ibo_data.len() * 2);
        }
        self.vbo.upload(target, frame, &vbo_data);
        self.ibo.upload(target, frame, &ibo_data);
        self.ibo_len = ibo_data.len() as _;
    }

    pub fn draw<'e, Sv, Bv, Si, Bi, const PIPELINE_BOUND: bool>(
        &'e self,
        render_pass: RenderPass<'e, Sv, Bv, Si, Bi, PIPELINE_BOUND>,
    ) -> RenderPass<'e> {
        render_pass
            .bind_ibo(&self.ibo)
            .bind_vbo(&self.vbo)
            .bind_group(&self.bind_group)
            .bind_shader(&self.shader)
            .draw_indexed(0..self.ibo_len, 0, 0..1)
            .done()
    }
}
