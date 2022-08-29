use srs2dge::prelude::*;

//

#[derive(Debug, Clone, Copy, PartialEq, Widget)]
#[gui(builder)]
pub struct Root {
    #[gui(inherit)]
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
            layout.size: Size {
                width: Dimension::Percent(1.0),
                height: Dimension::Percent(1.0)
            },
            widget.color: Color::ORANGE,
            widget.texture: tex0,
        }
    );

    let root: Root = Root::build(gui, Style::from_styles(&styles, ["root"]), &styles, &[]).unwrap();

    (texture, root)
}

fn main() {
    init_log();
    run_gui_app(gui_main);
}
