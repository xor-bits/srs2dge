use srs2dge::prelude::*;

//

struct App {
    target: Target,

    gui: Gui,

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

            texture,
        }
    }
}

impl Runnable for App {
    fn event(&mut self, event: Event, _: &EventLoopTarget, control: &mut ControlFlow) {
        self.gui.event(&event);

        if self.gui.window_state().should_close {
            *control = ControlFlow::Exit;
        }
    }

    fn draw(&mut self) {
        let mut frame = self.target.get_frame();

        let root = self.gui.root();
        const BORDER: f32 = 8.0;

        let v_split = |base: WidgetBase| {
            (base.size * Vec2::new(0.5, 1.0) - BORDER * Vec2::ONE * 2.0).max(Vec2::ZERO)
        };

        let left = Fill::builder()
            .with_parent(&root)
            .with_size(v_split)
            .with_offset(|base, _| base.offset + BORDER * Vec2::ONE)
            .with_color(Color::AZURE)
            .with_texture(self.texture.get(&0).unwrap())
            .with_gui(&mut self.gui)
            .build();

        let size = |base: WidgetBase| {
            (base.size * 0.5) // half the size of parent
                .min(Vec2::ONE * 200.0) // 200x200 px at max
                .force_ratio_with_x(1.0) // modify x to force square
                .min(base.size) // size of parent at max
                .force_ratio_with_y(1.0) // modify y to force square
        };

        let _top = Fill::builder()
            .with_parent(&left)
            .with_size(size)
            .with_offset(|base, size| align(base, size, Vec2::new(0.5, 1.0))) // x center, y top
            .with_color(Color::WHITE)
            .with_texture(self.texture.get(&1).unwrap())
            .with_gui(&mut self.gui)
            .build();
        let _bottom = Fill::builder()
            .with_parent(&left)
            .with_size(size)
            .with_offset(|base, size| align(base, size, Vec2::new(0.5, 0.0))) // x center, y bottom
            .with_color(Color::CHARTREUSE)
            .with_texture(self.texture.get(&0).unwrap())
            .with_gui(&mut self.gui)
            .build();

        let right = Grid::builder()
            .with_parent(&root)
            .with_size(v_split)
            .with_offset(|base, size| base.offset + base.size - size - BORDER * Vec2::ONE)
            .build();

        for (i, base) in right.enumerate() {
            Fill::builder()
                .with_base(base)
                .with_size(|base| (base.size - Vec2::ONE * 4.0).max(Vec2::ZERO))
                .with_offset(|base, _| base.offset + Vec2::ONE)
                .with_color(Color::new_mono((i as f32 / 9.0).powf(2.2)))
                .with_texture(self.texture.get(&0).unwrap())
                .with_gui(&mut self.gui)
                .build();
        }

        let gui = self
            .gui
            .generate_with(&mut self.target, &mut frame, &self.texture);
        frame.primary_render_pass().draw_gui(&gui);

        self.target.finish_frame(frame);
    }
}

//

main_app!(async App);
