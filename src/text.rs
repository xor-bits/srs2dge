use fontdue::Font;
use glium::{backend::Facade, Program};
use image::GrayImage;
use std::str::Chars;

use crate::packer::glyph::Glyphs;

#[derive(Debug, Clone, Copy)]
pub struct Vertex {
    vi_position: [f32; 2],
    vi_uv: [f32; 2],
}
glium::implement_vertex!(Vertex, vi_position, vi_uv);

#[derive(Debug, Clone)]
pub struct CharPositionIterator<'s> {
    chars: Chars<'s>,
    font: &'s Font,

    px: f32,

    x_origin: i32,
    y_origin: i32,
    last_c: Option<char>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct CharPosition {
    pub index: u16,
    pub x: i32,
    pub y: i32,
    pub width: u32,
    pub height: u32,
}

impl<'s> CharPositionIterator<'s> {
    pub fn new(s: &'s str, font: &'s Font, px: f32) -> Self {
        Self {
            chars: s.chars(),
            font,

            px,

            x_origin: 0,
            y_origin: px as i32,
            last_c: None,
        }
    }
}

impl<'s> Iterator for CharPositionIterator<'s> {
    type Item = CharPosition;

    fn next(&mut self) -> Option<Self::Item> {
        let mut c;
        loop {
            c = self.chars.next()?;

            match c {
                '\n' => {
                    self.x_origin = 0;
                    self.y_origin += (self.px * 1.4) as i32;
                    self.last_c = None;
                }
                '\t' => {
                    let w = self.px * 2.0;
                    self.x_origin = ((self.x_origin as f32 / w).floor() * w + w) as i32;
                }
                _ => break,
            }
        }

        let index = self.font.lookup_glyph_index(c);
        let metrics = self.font.metrics_indexed(index, self.px);

        let rect = CharPosition {
            index,
            x: self.x_origin + metrics.xmin,
            y: self.y_origin - metrics.height as i32 - metrics.ymin,
            width: metrics.width as _,
            height: metrics.height as _,
        };

        self.x_origin += metrics.advance_width as i32
            + self
                .last_c
                .and_then(|last_c| self.font.horizontal_kern(last_c, c, self.px))
                .unwrap_or(0.0) as i32;
        // self.y_origin -= metrics.advance_height as i32;
        self.last_c = Some(c);

        Some(rect)
    }
}

pub fn text(s: &str, glyphs: &mut Glyphs, px: f32) -> Vec<Vertex> {
    for c in s.chars() {
        glyphs.queue(c, px as u16);
    }
    glyphs.flush();

    CharPositionIterator::new(s, glyphs.font(), px)
        .flat_map(|c| {
            let glyph = glyphs.get_indexed(c.index, px as u16).unwrap();
            [
                Vertex {
                    vi_position: [c.x as f32, c.y as f32],
                    vi_uv: [glyph.top_left.x, glyph.top_left.y],
                },
                Vertex {
                    vi_position: [c.x as f32, c.y as f32 + c.height as f32],
                    vi_uv: [glyph.top_left.x, glyph.bottom_right.y],
                },
                Vertex {
                    vi_position: [c.x as f32 + c.width as f32, c.y as f32 + c.height as f32],
                    vi_uv: [glyph.bottom_right.x, glyph.bottom_right.y],
                },
                Vertex {
                    vi_position: [c.x as f32 + c.width as f32, c.y as f32],
                    vi_uv: [glyph.bottom_right.x, glyph.top_left.y],
                },
            ]
        })
        .collect()
}

pub fn baked_text(s: &str, font: &Font, px: f32) -> Option<GrayImage> {
    let chars = CharPositionIterator::new(s, font, px);

    // text bounding box
    let mut x_min = 0;
    let mut x_max = 0;
    let mut y_min = 0;
    let mut y_max = 0;

    for c in chars.clone() {
        x_min = x_min.min(c.x);
        y_min = y_min.min(c.y);

        x_max = x_max.max(c.x + c.width as i32);
        y_max = y_max.max(c.y + c.height as i32);
    }
    let width = (x_max - x_min).max(0) as usize;
    let height = (y_max - y_min).max(0) as usize;

    let mut text = vec![0; width * height];
    for c in chars {
        let (metrics, bitmap) = font.rasterize_indexed(c.index, px);

        for (index, pixel) in bitmap.iter().enumerate() {
            let x = index % metrics.width;
            let y = index / metrics.width;
            let output =
                &mut text[((c.x - x_min) as usize + x) + ((c.y - y_min) as usize + y) * width];
            *output = (*output).max(*pixel);
        }
    }

    GrayImage::from_raw(width as u32, height as u32, text)
}

pub fn text_program<F: Facade>(facade: &F) -> Program {
    glium::program!(facade,
        140 => {
            vertex: "#version 140
                in vec2 vi_position;
                in vec2 vi_uv;

                uniform mat4 mat;

                out vec2 fi_uv;

                void main() {
                    gl_Position = mat * vec4(vi_position, 0.0, 1.0) * vec4(1.0, -1.0, 1.0, 1.0);
                    fi_uv = vi_uv;
                }",
            fragment: "#version 140
                in vec2 fi_uv;

                uniform sampler2D sprite;

                out vec4 o_color;

                void main() {
                    o_color.rgb = vec3(1.0);
					o_color.a = texture(sprite, fi_uv).r;
                }"
        }
    )
    .unwrap()
}
