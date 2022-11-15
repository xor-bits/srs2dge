use self::debug::AppDebug;
use crate::prelude::{DrawGeneratedGui, Gui, Widget};
use srs2dge_core::{
    main_game_loop::{
        event::Event, event::EventLoopTarget, prelude::EventLoop, report::Reporter, should_draw,
    },
    prelude::{ControlFlow, TextureAtlasMap},
    target::Target,
    Engine,
};

//

mod debug;

//

struct App<Root: Widget, Fupd: FnMut(&mut Root)> {
    target: Target,
    reporter: Reporter,
    texture: TextureAtlasMap<u8>,

    gui: Gui,
    root: Root,
    upd: Fupd,

    debug: AppDebug,
}

//

impl<Root: Widget, Fu: FnMut(&mut Root)> App<Root, Fu> {
    async fn init<Fi: FnOnce(&Target, &mut Gui) -> (TextureAtlasMap<u8>, Root)>(
        target: &EventLoopTarget,
        fi: Fi,
        fu: Fu,
    ) -> Self {
        let engine = Engine::new();
        let mut target = engine.new_target_default(target).await.unwrap();
        target.set_vsync(false);

        let mut gui = Gui::new(&target);

        let (texture, root) = fi(&target, &mut gui);

        let debug = AppDebug::new(&target);

        Self {
            target,
            reporter: Reporter::new(),
            texture,

            gui,
            root,
            upd: fu,

            debug,
        }
    }

    async fn event(&mut self, event: Event<'_>, _: &EventLoopTarget, control: &mut ControlFlow) {
        self.target.event(&event);

        let event = match event.to_static() {
            Some(some) => some,
            None => return,
        };

        self.run_debug_events(&event);

        self.gui.event(&mut self.root, event);

        if self.gui.window_state().should_close {
            *control = ControlFlow::Exit;
        }
    }

    async fn draw(&mut self) {
        let timer = self.reporter.begin();

        let mut frame = self.target.get_frame();

        // drawing
        (self.upd)(&mut self.root);
        self.run_debug_pre_draw(&mut frame);
        let gui = self
            .gui
            .draw_with(&mut self.root, &mut self.target, &mut frame, &self.texture);
        let rp = frame.primary_render_pass().draw_gui(&gui);
        self.debug.run_debug_draw(rp);

        self.reporter.end(timer);
        if self.reporter.should_report() {
            tracing::info!(
                "{}",
                Reporter::report_all("GUI testbed perf report", [("FPS", &mut self.reporter)])
            );
        }
    }
}

pub fn gui_app<
    Root: Widget + 'static,
    Fi: FnOnce(&Target, &mut Gui) -> (TextureAtlasMap<u8>, Root),
    Fu: FnMut(&mut Root) + 'static,
>(
    fi: Fi,
    fu: Fu,
) {
    srs2dge_core::app!(
        |t| App::<Root, Fu>::init(t, fi, fu),
        App::<Root, Fu>::event,
        App::<Root, Fu>::draw
    );
}
