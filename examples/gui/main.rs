use srs2dge::{
    gui::gui::{generated::DrawGeneratedGui, Gui},
    prelude::*,
    winit::event::WindowEvent,
};

//

struct App {
    target: Target,

    gui: Gui,
    col: Color,

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
        let mut split = Grid::builder()
            .with_parent(&root)
            .with_size(|base| border_size(base, BORDER))
            .with_offset(|base, _| border_offset(base, BORDER))
            .with_columns(2)
            .with_rows(1)
            // .with_size(|base| border_size(base, 8.0))
            // .with_offset(offset)
            .build();

        let left = Fill::builder()
            .with_base(split.next().unwrap())
            .with_size(|base| border_size(base, BORDER))
            .with_offset(|base, _| border_offset(base, BORDER))
            .with_color(self.col)
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
            .with_base(split.next().unwrap())
            .with_size(|base| border_size(base, BORDER))
            .with_offset(|base, _| border_offset(base, BORDER))
            .build();

        for (i, base) in right.enumerate() {
            let col = Color::new_mono((i as f32 / 8.0).powf(2.2));
            let button = if self.gui.hovered(base) {
                Button::builder()
                    .with_size(|base| border_size(base, 2.0))
                    .with_offset(|base, _| border_offset(base, 2.0))
            } else {
                Button::builder()
                    .with_size(|base| border_size(base, 4.0))
                    .with_offset(|base, _| border_offset(base, 4.0))
            }
            .with_base(base)
            .with_gui(&mut self.gui)
            .build();

            Fill::builder()
                .with_parent(&button)
                .with_color(col)
                .with_texture(self.texture.get(&0).unwrap())
                .with_gui(&mut self.gui)
                .build();

            if button.clicked() {
                self.col = col;
            }
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
