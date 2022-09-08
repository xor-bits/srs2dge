use srs2dge::prelude::*;

//

#[derive(Debug, Clone, Copy, PartialEq, Widget)]
#[gui(builder)]
pub struct Root {
    #[gui(style = "container")]
    items: WidgetArray<Item, 6>,

    #[gui(inherit)]
    bg: Fill,
}

#[derive(Debug, Clone, Copy, PartialEq, Widget)]
#[gui(builder)]
pub struct Item {
    #[gui(inherit)]
    #[gui(style = "item")]
    bg: Fill,
}

//

fn gui_main(target: &Target, gui: &mut Gui) -> (TextureAtlasMap<u8>, Root) {
    let texture = TextureAtlasMap::builder()
        .with_bytes(0, res::texture::EMPTY)
        .unwrap()
        .with_bytes(1, res::texture::SPRITE)
        .unwrap()
        .with_label(Some("Atlas".to_string()))
        .build(target);

    let tex0 = texture.get(&0).unwrap_or_default();
    let tex1 = texture.get(&1).unwrap_or_default();

    let styles = stylesheet!(
        "root" => {
            widget.color: Color::ORANGE,
            widget.texture: tex0,
        }
        "container" => {
            layout.margin: LayoutRect {
                start: Dimension::Points(5.0),
                end: Dimension::Points(5.0),
                top: Dimension::Points(5.0),
                bottom: Dimension::Points(5.0)
            },
            layout.size: Size {
                width: Dimension::Auto,
                height: Dimension::Auto
            },
            layout.align_items: AlignItems::FlexStart,
            // layout.flex_wrap: FlexWrap::Wrap,
        }
        "item" => {
            layout.margin: LayoutRect {
                start: Dimension::Points(5.0),
                end: Dimension::Points(5.0),
                top: Dimension::Points(5.0),
                bottom: Dimension::Points(5.0)
            },
            // layout.flex_shrink: 1.0,
            layout.flex_grow: 1.0,
            layout.flex_basis: Dimension::Points(1.0),
            widget.color: Color::AZURE,
            widget.texture: tex0,
        }
        "max_size" => {
            layout.size: Size {
                width: Dimension::Percent(1.0),
                height: Dimension::Percent(1.0)
            },
        }
    );

    let root: Root = Root::build(
        gui,
        Style::from_styles(&styles, ["max_size", "root"]),
        &styles,
        &[],
    )
    .unwrap();

    (texture, root)
}

fn main() {
    init_log();
    run_gui_app(gui_main);
}
