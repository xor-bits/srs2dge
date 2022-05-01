use bytemuck::{Pod, Zeroable};
use glam::{Vec2, Vec4};

//

#[derive(Debug, Clone, Copy, PartialEq, Default, Zeroable, Pod)]
#[repr(C)]
pub struct DefaultVertex {
    pos: Vec2,
    uv: Vec2,
    col: Vec4,
}

//

impl DefaultVertex {
    pub fn new(pos: Vec2, col: Vec4, uv: Vec2) -> Self {
        Self { pos, uv, col }
    }

    pub fn from_arrays(pos: [f32; 2], col: [f32; 4], uv: [f32; 2]) -> Self {
        Self {
            pos: pos.into(),
            uv: uv.into(),
            col: col.into(),
        }
    }

    pub fn pos(&self) -> Vec2 {
        self.pos
    }

    pub fn uv(&self) -> Vec2 {
        self.uv
    }

    pub fn col(&self) -> Vec4 {
        self.col
    }
}

//

pub trait Vertex: Pod {}

//

impl<T> Vertex for T where T: Pod {}
