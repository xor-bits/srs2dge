use srs2dge::prelude::*;

//

#[derive(Debug, Clone, Copy, PartialEq, Widget)]
#[gui(builder)]
pub struct Root {
    #[gui(style = "div")]
    middle: Middle,

    #[gui(inherit, style = "root")]
    bg: Fill,
}

#[derive(Debug, Clone, Copy, PartialEq, Widget)]
#[gui(builder)]
pub struct Middle {
    #[gui(style = "status")]
    status: LoadingStatus,

    #[gui(core)]
    core: WidgetCore,
}

#[derive(Debug, Clone, Copy, PartialEq, Widget)]
#[gui(builder)]
pub struct LoadingStatus {
    #[gui(style = "bar")]
    bar: LoadingBar,

    #[gui(core)]
    core: WidgetCore,
}

#[derive(Debug, Clone, Copy, PartialEq, Widget)]
#[gui(builder)]
pub struct LoadingBar {
    #[gui(style = "bar_fill")]
    progress: Fill,

    #[gui(inherit, style = "bar_bg")]
    bg: Fill,
}

//

fn gui_main(target: &Target, gui: &mut Gui) -> (TextureAtlasMap<u8>, Root) {
    let texture = TextureAtlasMap::builder()
        .with_bytes(0, res::texture::EMPTY)
        .unwrap()
        .with_label(Some("Atlas".to_string()))
        .build(target);

    let tex = texture.get(&0).unwrap_or_default();

    let styles = stylesheet!(
        "root" => {
            layout.size: Size {
                width: Dimension::Percent(1.0),
                height: Dimension::Percent(1.0),
            },
            // layout.justify_content: JustifyContent::Center,
            layout.justify_content: JustifyContent::FlexStart,
            layout.align_content: AlignContent::Stretch,
            layout.align_items: AlignItems::Stretch,
            widget.color: Color::BLACK,
            widget.texture: tex,
        }
        "status" => {
            layout.flex_direction: FlexDirection::Column,
            layout.align_content: AlignContent::Stretch,
            layout.align_items: AlignItems::Stretch,
        }
        "bar" => {
            layout.min_size: Size {
                width: Dimension::Points(50.0),
                height: Dimension::Points(25.0),
            },
            layout.max_size: Size {
                width: Dimension::Points(500.0),
                height: Dimension::Points(25.0),
            },
            layout.flex_shrink: 0.0,
            layout.flex_grow: 1.0,
            layout.flex_basis: Dimension::Points(500.0),
            layout.margin: LayoutRect {
                start: Dimension::Points(20.0),
                end: Dimension::Points(20.0),
                ..Default::default()
            },
            // layout.flex_shrink: 1.0,
            // layout.flex_basis: Dimension::Points(500.0),
            //layout.max_size: Size {
            //    width: Dimension::Points(500.0),
            //    height: Dimension::Points(25.0)
            //},
        }
        "bar_bg" => {
            widget.color: Color::WHITE,
            widget.texture: tex,
        }
        "bar_fill" => {
            layout.margin: LayoutRect {
                start: Dimension::Points(3.0),
                end: Dimension::Points(3.0),
                top: Dimension::Points(3.0),
                bottom: Dimension::Points(3.0),
            },
            layout.flex_grow: 1.0,
            widget.color: Color::RED,
            widget.texture: tex,
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
