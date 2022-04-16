use glium::{backend::Facade, Program};

pub fn text_program<F>(facade: &F) -> Program
where
    F: Facade,
{
    glium::program!(facade,
        140 => {
            vertex: "#version 140
                in vec2 vi_position;
                in vec4 vi_color;
                in vec2 vi_uv;

                uniform mat4 mat;

                out vec4 fi_color;
                out vec2 fi_uv;

                void main() {
                    gl_Position = mat * vec4(vi_position, 0.0, 1.0) * vec4(1.0, -1.0, 1.0, 1.0);
                    fi_color = vi_color;
                    fi_uv = vi_uv;
                }",
            fragment: "#version 140
                in vec4 fi_color;
                in vec2 fi_uv;

                uniform sampler2D sprite;

                out vec4 o_color;

                void main() {
                    o_color = fi_color;
					o_color.a = texture(sprite, fi_uv).r;
                }"
        }
    )
    .unwrap_or_else(|err| panic!("Default program failed to compile: {}", err))
}
