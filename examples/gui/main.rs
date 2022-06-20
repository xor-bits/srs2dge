use srs2dge::prelude::*;

//

struct App {
    target: Target,

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
        let mut frame = self.target.get_frame();

        let root = self.gui.root();

        const BORDER: f32 = 4.0;

        // Left and right sides of the whole gui
        let mut split = Grid::builder()
            .with_parent(&root)
            .with_size(border_size(BORDER))
            .with_offset(border_offset(BORDER))
            .with_columns(2)
            .with_rows(1)
            // .with_size(|base| border_size(base, 8.0))
            // .with_offset(offset)
            .build();

        // left
        // background
        let left = Fill::builder()
            .with_base(split.next().unwrap())
            .with_size(border_size(BORDER))
            .with_offset(border_offset(BORDER))
            .with_color(self.col)
            .with_texture(self.texture.get(&0).unwrap())
            .build(&mut self.gui);
        // top and bottom quads
        let quad_base = Fill::builder().with_parent(&left).with_size(
            (BaseSize * Const(Vec2::splat(0.5))) // half the size of parent
                .min(Const(Vec2::splat(200.0))) // 200x200 px at max
                .force_ratio_with_x(1.0) // modify x to force square
                .min(BaseSize) // size of parent at max
                .force_ratio_with_y(1.0), // modify y to force square
        );
        let _top_quad = quad_base
            .with_offset(align(Vec2::new(0.5, 1.0))) // x center, y top
            .with_color(Color::WHITE)
            .with_texture(self.texture.get(&1).unwrap())
            .build(&mut self.gui);
        let _bottom_quad = quad_base
            .with_offset(align(Vec2::new(0.5, 0.0))) // x center, y bottom
            .with_color(Color::CHARTREUSE)
            .with_texture(self.texture.get(&0).unwrap())
            .build(&mut self.gui);

        // right
        // background
        let right = Fill::builder()
            .with_base(split.next().unwrap())
            .with_size(border_size(BORDER))
            .with_offset(border_offset(BORDER))
            .with_color(Color::ROSE)
            .with_texture(self.texture.get(&0).unwrap())
            .build(&mut self.gui);

        // 3x3 grid
        for (i, base) in Grid::builder().with_parent(&right).build().enumerate() {
            let col = Color::new_mono((i as f32 / 8.0).powf(2.2));
            let hovered = self.gui.hovered(base).next().is_some();
            let border = if hovered { 2.0 } else { 4.0 };
            let text_col = if (col.r + col.g + col.b) / 3.0 >= 0.35 {
                Color::BLACK
            } else {
                Color::WHITE
            };
            let text_px = if hovered { 20.0 } else { 18.0 };

            // buttons
            let button = Button::builder()
                .with_size(border_size(border))
                .with_base(base)
                .build(&mut self.gui);

            // button backgrounds
            let fill = Fill::builder()
                .with_parent(&button)
                .with_color(col)
                .with_texture(self.texture.get(&0).unwrap())
                .build(&mut self.gui);

            // button texts
            Text::builder()
                .with_parent(&fill)
                .with_text(
                    FormatString::builder()
                        .with(text_col)
                        .with(text_px)
                        .with(format!("{col:#}")),
                )
                .build(&mut self.gui, &self.target);

            // set left side bg color to match
            // the color of the clicked button
            if button.clicked() {
                self.col = col;
            }
        }

        // draggable tile/window/frame/whatever
        let drag = self.drag.get();
        let tile = Fill::builder()
            .with_size(Const(Vec2::new(200.0, 300.0)))
            .with_offset(Const(Vec2::splat(10.0) + drag))
            .with_color(Color::MINT)
            .with_texture(self.texture.get(&0).unwrap())
            .build(&mut self.gui);

        // top and bottom draggers
        let drag_base = DragZone::builder()
            .with_parent(&tile)
            .with_size(BaseSize * Const(Vec2::new(1.0, 0.25)));
        let drag_0 = drag_base.build(&mut self.gui, &mut self.drag);
        let drag_1 = drag_base
            .with_offset(align(Vec2::ONE))
            .build(&mut self.gui, &mut self.drag);
        let drag_fill = Fill::builder()
            .with_color(Color::new_mono_a(0.0, 0.2))
            .with_texture(self.texture.get(&0).unwrap());
        let _drag_0_fill = drag_fill.with_parent(&drag_0).build(&mut self.gui);
        let _drag_1_fill = drag_fill.with_parent(&drag_1).build(&mut self.gui);

        let gui = self
            .gui
            .generate_with(&mut self.target, &mut frame, &self.texture);
        frame.primary_render_pass().draw_gui(&gui);

        self.target.finish_frame(frame);
    }
}

//

main_app!(async App);
