use srs2dge::prelude::*;

//

const BORDER: f32 = 4.0;

//

struct App {
    target: Target,

    reporter: Reporter,

    gui: Gui,
    col: Color,
    drag: DragZoneState,

    texture: TextureAtlasMap<u8>,
}

//

impl App {
    async fn init(target: &EventLoopTarget) -> Self {
        let engine = Engine::new();
        let target = engine.new_target_default(target).await.unwrap();

        let gui = Gui::new(&target);

        let texture = TextureAtlasMap::builder()
            .with_bytes(0, res::texture::EMPTY)
            .unwrap()
            .with_bytes(1, res::texture::SPRITE)
            .unwrap()
            .build(&target);

        Self {
            target,

            reporter: Reporter::new(),

            gui,
            col: Color::AZURE,
            drag: Default::default(),

            texture,
        }
    }
}

impl Runnable for App {
    fn event(&mut self, event: Event, _: &EventLoopTarget, control: &mut ControlFlow) {
        let event = match event.to_static() {
            Some(some) => some,
            None => return,
        };

        self.gui.event(event);

        if self.gui.window_state().should_close {
            *control = ControlFlow::Exit;
        }
    }

    fn draw(&mut self) {
        let timer = self.reporter.begin();

        let tex0 = self.texture.get(&0).unwrap_or_default();
        let tex1 = self.texture.get(&1).unwrap_or_default();

        let mut frame = self.target.get_frame();

        let root = self.gui.root();
        let mut split = Grid::builder()
            .with_parent(&root)
            .with_size(&border_size(BORDER))
            .with_offset(&border_offset(BORDER))
            .with_columns(2)
            .with_rows(1)
            .build();
        // left panel
        left_panel(
            split.next().unwrap_or_default(),
            &mut self.gui,
            self.col,
            tex0,
            tex1,
        );
        // right panel
        right_panel(
            split.next().unwrap_or_default(),
            &mut self.gui,
            &mut self.col,
            &self.target,
            tex0,
        );
        // floating panel
        float_panel(
            &mut self.gui,
            &mut self.drag,
            &self.target,
            tex0,
            self.reporter.last_string(),
        );

        // drawing
        let gui = self
            .gui
            .generate_with(&mut self.target, &mut frame, &self.texture);
        frame.primary_render_pass().draw_gui(&gui);

        self.target.finish_frame(frame);

        self.reporter.end(timer);
        if self.reporter.should_report() {
            self.reporter.reset();
        }
    }
}

fn left_panel(
    root: WidgetBase,
    gui: &mut Gui,
    col: Color,
    tex0: TexturePosition,
    tex1: TexturePosition,
) {
    // background
    let left = Fill::builder()
        .with_base(root)
        .with_size(&border_size(BORDER))
        .with_offset(&border_offset(BORDER))
        .with_color(col)
        .with_texture(tex0)
        .build(gui);
    // top and bottom quad base
    let quad_base_size = (BaseSize * Const(Vec2::splat(0.5))) // half the size of parent
        .min(Const(Vec2::splat(200.0))) // 200x200 px at max
        .force_ratio_with_x(1.0) // modify x to force square
        .min(BaseSize) // size of parent at max
        .force_ratio_with_y(1.0); // modify y to force square
    let quad_base = Fill::builder()
        .with_parent(&left)
        .with_size(&quad_base_size);
    // top quad
    quad_base
        .with_offset(&align(Vec2::new(0.5, 1.0))) // x center, y top
        .with_color(Color::WHITE)
        .with_texture(tex1)
        .build(gui);
    // bottom quad
    quad_base
        .with_offset(&align(Vec2::new(0.5, 0.0))) // x center, y bottom
        .with_color(Color::CHARTREUSE)
        .with_texture(tex0)
        .build(gui);
}

fn right_panel(
    root: WidgetBase,
    gui: &mut Gui,
    col: &mut Color,
    target: &Target,
    tex0: TexturePosition,
) {
    // background
    let right = Fill::builder()
        .with_base(root)
        .with_size(&border_size(BORDER))
        .with_offset(&border_offset(BORDER))
        .with_color(Color::ROSE)
        .with_texture(tex0)
        .build(gui);

    // 3x3 grid
    for (i, base) in Grid::builder().with_parent(&right).build().enumerate() {
        let button_col = Color::new_mono((i as f32 / 8.0).powf(2.2));
        let hovered = gui.hovered(base).next().is_some();
        let border = if hovered { 2.0 } else { 4.0 };
        let text_col = button_col.foreground();
        let text_px = if hovered { 20.0 } else { 18.0 };

        // buttons
        let button = Button::builder()
            .with_size(&border_size(border))
            .with_offset(&border_offset(border))
            .with_base(base)
            .build(gui);

        // button backgrounds
        let fill = Fill::builder()
            .with_parent(&button)
            .with_color(button_col)
            .with_texture(tex0)
            .build(gui);

        // button texts
        Text::builder()
            .with_parent(&fill)
            .with_text(
                FormatString::builder()
                    .with(text_col)
                    .with(text_px)
                    .with(format!("{button_col:#}")),
            )
            .build(gui, target);

        // set left side bg color to match
        // the color of the clicked button
        if button.clicked() {
            *col = button_col;
        }
    }
}

fn float_panel(
    gui: &mut Gui,
    state: &mut DragZoneState,
    target: &Target,
    tex0: TexturePosition,
    (ft, fps): (String, String),
) {
    // draggable tile/window/frame/whatever
    let drag = state.get();
    let tile = Fill::builder()
        .with_size(&Const(Vec2::new(200.0, 300.0)))
        .with_offset(&Const(Vec2::splat(10.0) + drag))
        .with_color(Color::MINT)
        .with_texture(tex0)
        .build(gui);

    // top and bottom draggers
    let drag_base_size = BaseSize * Const(Vec2::splat(0.25));
    let drag_base = DragZone::builder()
        .with_parent(&tile)
        .with_size(&drag_base_size);
    let drag_0 = drag_base.build(gui, state);
    let drag_1 = drag_base.with_offset(&align(Vec2::ONE)).build(gui, state);
    let drag_fill = Fill::builder()
        .with_color(Color::new_mono_a(0.0, 0.2))
        .with_texture(tex0);
    let _drag_0_fill = drag_fill.with_parent(&drag_0).build(gui);
    let _drag_1_fill = drag_fill.with_parent(&drag_1).build(gui);

    // perf text
    let _text = Text::builder()
        .with_parent(&tile)
        .with_text(FormatString::from_iter([
            Color::MINT.foreground().into(),
            20.0.into(),
            format!("FPS: {fps}\n").into(),
            16.0.into(),
            format!("Frametime: {ft}").into(),
        ]))
        .build(gui, target);
}

//

main_app!(async App);
