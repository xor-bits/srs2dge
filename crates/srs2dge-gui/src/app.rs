use crate::prelude::{DrawGeneratedGui, Gui, GuiLayout, Widget};
use srs2dge_core::{
    main_game_loop::{
        as_async, event::Event, event::EventLoopTarget, prelude::EventLoop, report::Reporter,
        runnable::Runnable,
    },
    prelude::{ControlFlow, TextureAtlasMap, VirtualKeyCode},
    target::Target,
    winit::event::{ElementState, KeyboardInput, WindowEvent},
    Engine,
};

//

struct App<Root: Widget> {
    target: Target,
    reporter: Reporter,
    texture: TextureAtlasMap<u8>,

    gui: Gui,
    root: Root,
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

        Self {
            target,
            reporter: Reporter::new(),
            texture,

            gui,

            root,
        }
    }
}

impl<Root: Widget> Runnable for App<Root> {
    fn event(&mut self, event: Event, _: &EventLoopTarget, control: &mut ControlFlow) {
        let event = match event.to_static() {
            Some(some) => some,
            None => return,
        };

        // debug keys
        let pressed = if let Event::WindowEvent {
            event:
                WindowEvent::KeyboardInput {
                    input:
                        KeyboardInput {
                            state: ElementState::Pressed,
                            virtual_keycode,
                            ..
                        },
                    ..
                },
            ..
        } = &event
        {
            *virtual_keycode
        } else {
            None
        };
        if let Some(VirtualKeyCode::F22) = pressed {
            log::debug!("F22 pressed, printing debug gui tree");
            struct DebugTree<'a> {
                layout: &'a GuiLayout,
                this: &'a dyn Widget,
                depth: usize,
            }
            impl<'a> DebugTree<'a> {
                fn new(layout: &'a GuiLayout, widget: &'a dyn Widget, depth: usize) -> Self {
                    Self {
                        layout,
                        this: widget,
                        depth,
                    }
                }

                fn print(self, buf: &mut String) {
                    use std::fmt::Write;
                    let spaces = " ".repeat(self.depth * 2);
                    let name = self.this.name();
                    let layout = self.layout.get(self.this);
                    let layout = match layout {
                        Ok(layout) => format!("Ok(o: {}, s: {})", layout.offset, layout.size),
                        Err(err) => format!("Err({err})"),
                    };
                    writeln!(buf, "{spaces}{layout} = \"{name}\"").unwrap();

                    for widget in self.this.subwidgets() {
                        Self::new(self.layout, widget, self.depth + 1).print(buf);
                    }
                }
            }

            if log::log_enabled!(log::Level::Debug) {
                let mut buf = String::new();
                DebugTree::new(&self.gui.layout(), &self.root, 0).print(&mut buf);
                log::debug!("Done:\n{buf}",);
            }
        }

        self.gui.event(&mut self.root, event).unwrap();

        if self.gui.window_state().should_close {
            *control = ControlFlow::Exit;
        }
    }

    fn draw(&mut self) {
        let timer = self.reporter.begin();

        let mut frame = self.target.get_frame();

        // drawing
        let gui = self
            .gui
            .draw_with(&mut self.root, &mut self.target, &mut frame, &self.texture)
            .unwrap();
        frame.primary_render_pass().draw_gui(&gui);

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
