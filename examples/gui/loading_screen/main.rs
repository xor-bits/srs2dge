use std::time::Instant;

use srs2dge::prelude::*;

//

#[derive(Debug, Clone, Widget)]
pub struct Root {
    middle: Middle,

    #[gui(inherit, style = "root")]
    bg: Fill,
}

#[derive(Debug, Clone, Widget)]
pub struct Middle {
    status: LoadingStatus,

    #[gui(core, style = "div")]
    core: WidgetCore,
}

#[derive(Debug, Clone, Widget)]
pub struct LoadingStatus {
    #[gui(style = "bar")]
    bar: LoadingBar,

    #[gui(core, style = "status")]
    core: WidgetCore,
}

#[derive(Debug, Clone, Widget)]
pub struct LoadingBar {
    #[gui(style = "bar_fill")]
    progress: Fill,

    #[gui(inherit, style = "bar_bg")]
    bg: Fill,
}

//

fn gui_main(target: &Target, _: &mut Gui) -> (TextureAtlasMap<u8>, Root) {
    let texture = TextureAtlasMap::builder()
        .with_bytes(0, res::texture::EMPTY)
        .unwrap()
        .with_label(Some("Atlas".to_string()))
        .build(target);

    let tex = texture.get(&0).unwrap_or_default();
    let now = Instant::now();

    let mut styles = StyleSheet::new();
    styles.set_default(Style {
        texture: Some(tex),
        ..Default::default()
    });
    styles.insert(
        "root",
        Style {
            color: Some(Color::BLACK),
            ..Default::default()
        },
    );
    styles.insert(
        "div",
        Style {
            size: Some(Size::Calc(Box::new(|parent: WidgetLayout| {
                Vec2::new((parent.size.x - 50.0).min(500.0).max(0.0), parent.size.y)
            }))),
            offset: Some(Offset::Calc(Box::new(
                |parent: WidgetLayout, size: Vec2| {
                    Vec2::new(
                        parent.offset.x + (parent.size.x - size.x) * 0.5,
                        parent.offset.y,
                    )
                },
            ))),
            ..Default::default()
        },
    );
    styles.insert(
        "status",
        Style {
            size: Some(Size::Calc(Box::new(|parent: WidgetLayout| {
                Vec2::new(parent.size.x, parent.size.y.min(25.0).max(0.0))
            }))),
            offset: Some(Offset::Calc(Box::new(
                |parent: WidgetLayout, size: Vec2| {
                    Vec2::new(
                        parent.offset.x,
                        parent.offset.y + (parent.size.y - size.y) * 0.5,
                    )
                },
            ))),
            ..Default::default()
        },
    );
    styles.insert(
        "bar",
        Style {
            color: Some(Color::RED),
            ..Default::default()
        },
    );
    styles.insert(
        "bar_fill",
        Style {
            color: Some(Color::WHITE),
            size: Some(Size::Calc(Box::new(move |parent: WidgetLayout| {
                Vec2::new(
                    (parent.size.x - 6.0)
                        * ((Instant::now() - now).as_secs_f32() * 0.2)
                            .min(1.0)
                            .max(0.0),
                    parent.size.y - 6.0,
                )
            }))),
            offset: Some(Offset::borders(3.0)),
            ..Default::default()
        },
    );

    let root: Root = WidgetBuilder::build(StyleRef::from_styles("root", &styles), &styles);

    for unused in styles.check_unused() {
        log::warn!("Unused style '{unused}'")
    }

    (texture, root)
}

fn gui_upd(_: &mut Root) {}

fn main() {
    init_log();
    run_gui_app(gui_main, gui_upd);
}
