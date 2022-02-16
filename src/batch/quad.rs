use super::Mesh;
use glam::{Vec2, Vec4, Vec4Swizzles};
use glium::index::PrimitiveType;
use std::array::IntoIter;

#[derive(Debug, Clone, Copy, Default)]
pub struct QuadMesh {
    pub pos: Vec2,
    pub size: Vec2,
    pub id: u32,
    pub isometric: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Vert {
    pos: [f32; 2],
    uv: [f32; 2],
    id: u32,
}

implement_vertex!(Vert, pos, uv, id);

//

impl Mesh<Vert> for QuadMesh {
    const PRIM: PrimitiveType = PrimitiveType::TriangleStrip;

    const VERTICES: usize = 4;
    const INDICES: usize = 5;

    type VertexIter = IntoIter<Vert, 4>;
    type IndexIter = IntoIter<u32, 5>;

    fn vertices(&self) -> Self::VertexIter {
        if self.isometric {
            let id = self.id;
            IntoIterator::into_iter([
                Vert {
                    pos: (self.pos + self.size * Vec2::new(0.0, 0.5)).to_array(),
                    uv: [0.0, 0.0],
                    id,
                },
                Vert {
                    pos: (self.pos + self.size * Vec2::new(0.5, 1.0)).to_array(),
                    uv: [0.0, 1.0],
                    id,
                },
                Vert {
                    pos: (self.pos + self.size * Vec2::new(0.5, 0.0)).to_array(),
                    uv: [1.0, 0.0],
                    id,
                },
                Vert {
                    pos: (self.pos + self.size * Vec2::new(1.0, 0.5)).to_array(),
                    uv: [1.0, 1.0],
                    id,
                },
            ])
        } else {
            let p = Vec4::new(
                self.pos.x,
                self.pos.y,
                self.pos.x + self.size.x,
                self.pos.y + self.size.y,
            );
            let id = self.id;
            IntoIterator::into_iter([
                Vert {
                    pos: p.xy().to_array(),
                    uv: [0.0, 0.0],
                    id,
                },
                Vert {
                    pos: p.xw().to_array(),
                    uv: [0.0, 1.0],
                    id,
                },
                Vert {
                    pos: p.zy().to_array(),
                    uv: [1.0, 0.0],
                    id,
                },
                Vert {
                    pos: p.zw().to_array(),
                    uv: [1.0, 1.0],
                    id,
                },
            ])
        }
    }

    fn indices(&self, offset: u32) -> Self::IndexIter {
        IntoIterator::into_iter([offset, offset + 1, offset + 2, offset + 3, !0])
    }
}
