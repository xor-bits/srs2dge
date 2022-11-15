use srs2dge::prelude::*;

//

mod stylesheet;

//

struct App {
    target: Target,
    reporter: Reporter,
    texture: TextureAtlasMap<u8>,

    gui: Gui,
    root: Root,
}

#[derive(Debug, Clone, Widget)]
struct Root {
    #[gui(style = "bordered fill_0")]
    left_panel: Fill,
    // #[gui(style = "bordered fill_1")]
    // right_panel: Fill,
    text_box: TextBox,

    #[gui(core, style = "root")]
    core: WidgetCore,
}

#[derive(Debug, Clone, Widget)]
struct TextBox {
    #[gui(style = "fill_1")]
    bg_border: Fill,
    #[gui(style = "bordered fill_2")]
    bg: Fill,
    #[gui(style = "text_box_text")]
    text: Text<'static>,

    #[gui(core, style = "text_box")]
    core: WidgetCore,
}

//

impl App {
    async fn init(target: &EventLoopTarget) -> Self {
        let target = Engine::new().new_target_default(target).await.unwrap();

        // gui base
        let gui = Gui::new(&target);

        // main texture atlas
        let texture = TextureAtlasMap::builder()
            .with_bytes(0, res::texture::EMPTY)
            .unwrap()
            .with_bytes(1, res::texture::SPRITE)
            .unwrap()
            .with_label(Some("Atlas".to_string()))
            .build(&target);

        // style sheet setup
        let tex0 = texture.get(&0).unwrap_or_default();
        let tex1 = texture.get(&1).unwrap_or_default();
        let stylesheet = stylesheet::styles(tex0, tex1);

        // root widget (and all of its subwidgets)
        let mut root: Root = WidgetBuilder::build(StyleRef::default(), &stylesheet);
        root.text_box.text.text(
            FormatString::builder()
                .with_init(Format::new(Color::BLACK, 0, 28.0))
                .with(format!("test text j|^ {}", env!("CARGO_PKG_NAME"))),
        );

        // warn about unused style sheet entries
        for name in stylesheet.check_unused() {
            tracing::warn!("Unused style: '{name}'");
        }

        Self {
            target,
            reporter: Reporter::new(),
            texture,

            gui,

            root,
        }
    }

    async fn event(&mut self, event: Event<'_>, _: &EventLoopTarget, control: &mut ControlFlow) {
        self.target.event(&event);

        let event = match event.to_static() {
            Some(some) => some,
            None => return,
        };

        self.gui.event(&mut self.root, event);

        if self.gui.window_state().should_close {
            *control = ControlFlow::Exit;
        }
    }

    async fn draw(&mut self) {
        let timer = self.reporter.begin();

        let mut frame = self.target.get_frame();

        // draw gui
        let gui = self
            .gui
            .draw_with(&mut self.root, &mut self.target, &mut frame, &self.texture);
        frame.primary_render_pass().draw_gui(&gui);

        self.reporter.end(timer);
        if self.reporter.should_report() {
            self.reporter.reset();
        }
    }
}

//

fn main() {
    app!(App);
}
