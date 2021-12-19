use super::{format::FString, pos_iter::CharPositionIter};
use crate::packer::glyph::Glyphs;
use fontdue::Font;
use glium::{backend::Facade, Program};
use image::RgbaImage;

#[derive(Debug, Clone, Copy)]
pub struct Vertex {
    vi_position: [f32; 2],
    vi_color: [f32; 3],
    vi_uv: [f32; 2],
}
glium::implement_vertex!(Vertex, vi_position, vi_color, vi_uv);

pub fn text<'s, F>(string: F, glyphs: &mut Glyphs, px: f32) -> Vec<Vertex>
where
    F: Into<&'s FString>,
{
    let string: &FString = string.into();

    for (c, _) in string.chars() {
        glyphs.queue(c, px as u16);
    }
    glyphs.flush();

    CharPositionIter::new(string, glyphs.font(), px)
        .flat_map(|c| {
            let glyph = glyphs.get_indexed(c.index, px as u16).unwrap();
            [
                Vertex {
                    vi_position: [c.x as f32, c.y as f32],
                    vi_color: c.format.color.to_array(),
                    vi_uv: [glyph.top_left.x, glyph.top_left.y],
                },
                Vertex {
                    vi_position: [c.x as f32, c.y as f32 + c.height as f32],
                    vi_color: c.format.color.to_array(),
                    vi_uv: [glyph.top_left.x, glyph.bottom_right.y],
                },
                Vertex {
                    vi_position: [c.x as f32 + c.width as f32, c.y as f32 + c.height as f32],
                    vi_color: c.format.color.to_array(),
                    vi_uv: [glyph.bottom_right.x, glyph.bottom_right.y],
                },
                Vertex {
                    vi_position: [c.x as f32 + c.width as f32, c.y as f32],
                    vi_color: c.format.color.to_array(),
                    vi_uv: [glyph.bottom_right.x, glyph.top_left.y],
                },
            ]
        })
        .collect()
}

pub fn baked_text<'s, F>(string: F, font: &Font, px: f32) -> Option<RgbaImage>
where
    F: Into<&'s FString> + Clone,
{
    let string: &FString = string.into();

    // text bounding box
    let mut x_min = 0;
    let mut x_max = 0;
    let mut y_min = 0;
    let mut y_max = 0;

    for c in CharPositionIter::new(string, font, px) {
        x_min = x_min.min(c.x);
        y_min = y_min.min(c.y);

        x_max = x_max.max(c.x + c.width as i32);
        y_max = y_max.max(c.y + c.height as i32);
    }
    let width = (x_max - x_min).max(0) as usize;
    let height = (y_max - y_min).max(0) as usize;

    let mut text = vec![0; width * height * 4];
    for c in CharPositionIter::new(string, font, px) {
        let (metrics, bitmap) = font.rasterize_indexed(c.index, px);

        for (index, pixel) in bitmap.iter().enumerate() {
            let x = (c.x - x_min) as usize + index % metrics.width;
            let y = (c.y - y_min) as usize + index / metrics.width;
            let index = (x + y * width) * 4;
            let a = *pixel as f32 / 255.0;

            let output_r = &mut text[index];
            *output_r =
                (*output_r as f32 * (1.0 - a) + (255.0 * c.format.color.x) as f32 * a) as u8;
            let output_g = &mut text[index + 1];
            *output_g =
                (*output_g as f32 * (1.0 - a) + (255.0 * c.format.color.y) as f32 * a) as u8;
            let output_b = &mut text[index + 2];
            *output_b =
                (*output_b as f32 * (1.0 - a) + (255.0 * c.format.color.z) as f32 * a) as u8;
            let output_a = &mut text[index + 3];
            *output_a = (*output_a).max(*pixel);
        }
    }

    RgbaImage::from_raw(width as u32, height as u32, text)
}

pub fn text_program<F: Facade>(facade: &F) -> Program {
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
                    o_color.rgb = fi_color;
					o_color.a = texture(sprite, fi_uv).r;
                }"
        }
    )
    .unwrap()
}
