use srs2dge::prelude::*;

//

pub fn styles(tex0: TexturePosition, tex1: TexturePosition) -> StyleSheet<'static> {
    stylesheet! {
        "fill_0" => {
            widget.color: Color::CHARTREUSE,
            widget.texture: tex0,
        }
        "fill_1" => {
            widget.color: Color::AZURE,
            widget.texture: tex0,
        }
        "fill_2" => {
            widget.color: Color::WHITE,
            widget.texture: tex1,
        }
        "fill_3" => {
            widget.color: Color::ORANGE,
            widget.texture: tex0,
        }
        "texture" => {
            widget.color: Color::WHITE,
            widget.texture: tex1,
        }
        "bordered" => {
            layout.margin: LayoutRect {
                start: Dimension::Points(3.0),
                end: Dimension::Points(3.0),
                top: Dimension::Points(3.0),
                bottom: Dimension::Points(3.0)
            },
        }
        "flex_item" => {
            layout.flex_shrink: 1.0,
            layout.flex_grow: 1.0,
            layout.flex_basis: Dimension::Points(5.0),
        }
        "float_item" => {
            layout.position_type: PositionType::Absolute,
            layout.position: LayoutRect {
                start: Dimension::Points(0.0),
                end: Dimension::Auto,
                top: Dimension::Points(0.0),
                bottom: Dimension::Auto,
            },
            layout.size: Size {
                width: Dimension::Points(350.0),
                height: Dimension::Points(350.0),
            },
        }
        "max_size" => {
            layout.size: Size {
                width: Dimension::Percent(1.0),
                height: Dimension::Percent(1.0),
            },
        }
        "split_block_lower" => {
            layout.size: Size {
                width: Dimension::Auto,
                height: Dimension::Points(150.0),
            },
            layout.justify_content: JustifyContent::Center,
        }
        "split_block_upper" => {
            layout.size: Size {
                width: Dimension::Auto,
                height: Dimension::Auto
            },
            layout.aspect_ratio: Number::Defined(1.0),
        }
        "left_split" => {
            layout.flex_direction: FlexDirection::Column,
            layout.justify_content: JustifyContent::SpaceBetween,
            // layout.align_items: AlignItems::Center,
        }
        "button" => {
            // layout.flex_shrink: 1.0,
        }
        "buttons" => {
            // layout.size: Size {
            //     width: Dimension::Percent(1.0),
            //     height: Dimension::Percent(1.0)
            // },
            layout.flex_wrap: FlexWrap::WrapReverse,
            layout.justify_content: JustifyContent::FlexStart,
            layout.align_content: AlignContent::FlexStart,
            // layout.justify_content: JustifyContent::SpaceBetween,
            // layout.align_items: AlignItems::Center,
        }
        "button_fill" => {
            layout.min_size: Size {
                width: Dimension::Points(100.0),
                height: Dimension::Points(100.0)
            },
            layout.max_size: Size {
                width: Dimension::Points(250.0),
                height: Dimension::Points(250.0)
            },
            layout.flex_shrink: 1.0,
            layout.flex_grow: 1.0,
        }
    }
}
