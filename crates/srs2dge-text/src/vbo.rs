use crate::{
    glyphs::{fonts::Fonts, Glyphs},
    prelude::{FormatChar, FormatChars},
    typography::{config::TextConfig, prelude::TextChars},
};
use srs2dge_core::{glam::Vec2, image::RgbaImage, prelude::QuadMesh, target::Target};

//

/// overwrites config sdf with the value from glyphs
pub fn text<'i>(
    target: &Target,
    chars: FormatChars<'i>,
    glyphs: &'i mut Glyphs,
    mut config: TextConfig,
) -> Result<impl Iterator<Item = QuadMesh> + 'i, &'static str> {
    config.sdf = glyphs.is_sdf();

    // setup glyphs

    for FormatChar { character, format } in chars.clone() {
        glyphs.queue(character, format.px as _, format.font);
    }
    glyphs.flush(target)?;

    // gen quads
    Ok(TextChars::new(chars, glyphs.fonts(), config).map(|c| {
        let tex = glyphs
            .get_indexed(c.index, c.format.px as _, c.format.font)
            .unwrap();

        // TODO: mesh anchoring
        let size = Vec2::new(c.width as _, c.height as _);
        QuadMesh {
            pos: Vec2::new(c.x as _, c.y as _) + 0.5 * size,
            size,
            col: c.format.color,
            tex,
        }
    }))
}

pub fn baked_text(_: FormatChars, _: &Fonts, _: TextConfig) -> Option<RgbaImage> {
    todo!()
    /* let TextBoundingBox {
        x,
        y,
        width,
        height,
    } = TextChars::new(chars.clone(), fonts, config).bounding_box();
    let width = width as usize;
    let height = height as usize;

    let mut text = vec![0; width * height * 4];
    for c in TextChars::new(chars, fonts, config) {
        let (metrics, bitmap) =
            fonts
                .get_font(c.format.font)
                .rasterize_indexed(c.index, c.format.px, false);

        for (index, pixel) in bitmap.iter().enumerate() {
            let x = (c.x - x) as usize + index % metrics.width;
            let y = (c.y - y) as usize + index / metrics.width;
            let index = (x + y * width) * 4;
            let a = *pixel as f32 / 255.0;

            let output_r = &mut text[index];
            *output_r = (*output_r as f32 * (1.0 - a) + (255.0 * c.format.color.r) * a) as u8;
            let output_g = &mut text[index + 1];
            *output_g = (*output_g as f32 * (1.0 - a) + (255.0 * c.format.color.g) * a) as u8;
            let output_b = &mut text[index + 2];
            *output_b = (*output_b as f32 * (1.0 - a) + (255.0 * c.format.color.b) * a) as u8;
            let output_a = &mut text[index + 3];
            *output_a = (*output_a).max(*pixel);
        }
    }

    RgbaImage::from_raw(width as u32, height as u32, text) */
}
