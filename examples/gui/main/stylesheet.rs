use srs2dge::prelude::*;

//

pub fn styles(tex0: TexturePosition, tex1: TexturePosition) -> StyleSheet<'static> {
    stylesheet! {
        => {
            texture: tex0,
        }
        "fill_0" => {
            color: Color::CHARTREUSE,
        }
        "fill_1" => {
            color: Color::DARK_GREY,
        }
        "fill_2" => {
            color: Color::GREY,
        }
        "text_box" => {
            size: Size::Max(Vec2::new(400.0, 120.0)),
            offset: Offset::Centered,
        }
        "text_box_text" => {
            text_align: TextAlign::centered(),
        }
        "texture" => {
            color: Color::WHITE,
            texture: tex1,
        }
        "bordered" => {
            size: Size::borders(3.0),
            offset: Offset::borders(3.0),
        }
    }
}
