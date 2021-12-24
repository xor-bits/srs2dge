use glam::{Vec2, Vec3};
use glium::{backend::Facade, Program};

#[derive(Debug, Clone, Copy)]
pub struct DefaultVertex {
    vi_position: [f32; 2],
    vi_color: [f32; 3],
    vi_uv: [f32; 2],
}

impl DefaultVertex {
    pub fn new(x: f32, y: f32, r: f32, g: f32, b: f32, u: f32, v: f32) -> Self {
        Self {
            vi_position: [x, y],
            vi_color: [r, g, b],
            vi_uv: [u, v],
        }
    }

    pub fn from_arrays(pos: [f32; 2], col: [f32; 3], uv: [f32; 2]) -> Self {
        Self {
            vi_position: pos,
            vi_color: col,
            vi_uv: uv,
        }
    }

    pub fn from_vecs(pos: Vec2, col: Vec3, uv: Vec2) -> Self {
        Self::from_arrays(pos.to_array(), col.to_array(), uv.to_array())
    }

    pub fn pos(&self) -> Vec2 {
        Vec2::from_slice(&self.vi_position)
    }

    pub fn col(&self) -> Vec3 {
        Vec3::from_slice(&self.vi_color)
    }

    pub fn uv(&self) -> Vec2 {
        Vec2::from_slice(&self.vi_uv)
    }
}

glium::implement_vertex!(DefaultVertex, vi_position, vi_color, vi_uv);

pub fn default_program<F>(facade: &F) -> Program
where
    F: Facade,
{
    glium::program!(facade,
        140 => {
            vertex: "#version 140
                in vec2 vi_position;
                in vec3 vi_color;
                in vec2 vi_uv;

                uniform mat4 mat;

                out vec3 fi_color;
                out vec2 fi_uv;

                void main() {
                    gl_Position = mat * vec4(vi_position, 0.0, 1.0) * vec4(1.0, -1.0, 1.0, 1.0);
                    fi_color = vi_color;
                    fi_uv = vi_uv;
                }",
            fragment: "#version 140
                in vec3 fi_color;
                in vec2 fi_uv;

                uniform sampler2D sprite;

                out vec4 o_color;

                void main() {
                    o_color = vec4(fi_color, 1.0) * texture(sprite, fi_uv);
                }"
        }
    )
    .unwrap_or_else(|err| panic!("Default program failed to compile: {}", err))
}
