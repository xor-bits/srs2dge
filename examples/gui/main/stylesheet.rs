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
            color: Color::AZURE,
        }
        "fill_2" => {
            color: Color::ORANGE,
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
