use self::debug::AppDebug;
use crate::prelude::{DrawGeneratedGui, Gui, Widget};
use srs2dge_core::{
    main_game_loop::{
        as_async, event::Event, event::EventLoopTarget, prelude::EventLoop, report::Reporter,
        runnable::Runnable, state::window::WindowState,
    },
    prelude::{ControlFlow, TextureAtlasMap},
    target::Target,
    Engine,
};

//

mod debug;

//

struct App<Root: Widget> {
    target: Target,
    reporter: Reporter,
    texture: TextureAtlasMap<u8>,

    gui: Gui,
    root: Root,

    debug: AppDebug,
}

//

impl<Root: Widget> App<Root> {
    async fn init<F: FnOnce(&Target, &mut Gui) -> (TextureAtlasMap<u8>, Root)>(
        target: &EventLoopTarget,
        f: F,
    ) -> Self {
        let engine = Engine::new();
        let target = engine.new_target_default(target).await.unwrap();

        let mut gui = Gui::new(&target);

        let (texture, root) = f(&target, &mut gui);

        let debug = AppDebug::new(&target);

        Self {
            target,
            reporter: Reporter::new(),
            texture,

            gui,
            root,

            debug,
        }
    }
}

impl<Root: Widget> Runnable for App<Root> {
    fn event(&mut self, event: Event, _: &EventLoopTarget, control: &mut ControlFlow) {
        let event = match event.to_static() {
            Some(some) => some,
            None => return,
        };

        self.run_debug_events(&event);

        self.gui.event(&mut self.root, event).unwrap();

        if self.gui.window_state().should_close {
            *control = ControlFlow::Exit;
        }
    }

    fn draw(&mut self) {
        let timer = self.reporter.begin();

        let mut frame = self.target.get_frame();

        // drawing
        self.run_debug_pre_draw(&mut frame);
        let gui = self
            .gui
            .draw_with(&mut self.root, &mut self.target, &mut frame, &self.texture)
            .unwrap();
        let rp = frame.primary_render_pass().draw_gui(&gui);
        self.debug.run_debug_draw(rp);

        self.target.finish_frame(frame);

        self.reporter.end(timer);
        if self.reporter.should_report() {
            self.reporter.reset();
        }
    }
}

pub fn run_gui_app<
    Root: Widget + 'static,
    F: FnOnce(&Target, &mut Gui) -> (TextureAtlasMap<u8>, Root) + 'static,
>(
    f: F,
) {
    as_async(run_gui_app_async(f));
}

pub async fn run_gui_app_async<
    Root: Widget + 'static,
    F: FnOnce(&Target, &mut Gui) -> (TextureAtlasMap<u8>, Root),
>(
    f: F,
) {
    let target = EventLoop::new();
    let app = App::<Root>::init(&target, f).await;
    target.runnable(app);
}
