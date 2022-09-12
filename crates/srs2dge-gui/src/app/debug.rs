use super::App;
use crate::prelude::{GuiLayout, Widget, WidgetLayout};
use srs2dge_core::{
    glam::Mat4,
    main_game_loop::event::Event,
    prelude::{Color, Frame, RenderPass, VirtualKeyCode},
    target::Target,
    winit::event::{ElementState, KeyboardInput, WindowEvent},
};
use std::fmt::Write;

//

pub struct AppDebug {
    #[cfg(feature = "gizmos")]
    gizmo: bool,
    #[cfg(feature = "gizmos")]
    gizmos: srs2dge_gizmos::Gizmos,
}

struct DebugTree<'a> {
    layout: &'a GuiLayout,
    this: &'a dyn Widget,
    depth: usize,
}

//

impl AppDebug {
    pub fn new(target: &Target) -> Self {
        Self {
            #[cfg(feature = "gizmos")]
            gizmo: cfg!(debug_assertions),
            #[cfg(feature = "gizmos")]
            gizmos: srs2dge_gizmos::Gizmos::new(target),
        }
    }
}

impl<'a> DebugTree<'a> {
    fn new(layout: &'a GuiLayout, widget: &'a dyn Widget, depth: usize) -> Self {
        Self {
            layout,
            this: widget,
            depth,
        }
    }
}

impl<T: Widget> App<T> {
    pub fn run_debug_events(&mut self, event: &Event<'static>) {
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
        match pressed {
            Some(VirtualKeyCode::F22) => {
                log::debug!("F22 pressed, printing debug gui tree");

                impl<'a> DebugTree<'a> {
                    fn print_debug_tree(self, buf: &mut String, parent_layout: WidgetLayout) {
                        let spaces = " ".repeat(self.depth * 2);
                        let name = self.this.name();
                        let layout = match self.layout.get(self.this) {
                            Ok(o) => o,
                            _ => return,
                        }
                        .to_absolute(parent_layout);

                        writeln!(
                            buf,
                            "{spaces}(s:{},o:{}) = \"{name}\"",
                            layout.size, layout.offset
                        )
                        .unwrap();

                        for widget in self.this.subwidgets() {
                            Self::new(self.layout, widget, self.depth + 1)
                                .print_debug_tree(buf, layout);
                        }
                    }
                }

                if log::log_enabled!(log::Level::Debug) {
                    let mut buf = String::new();
                    DebugTree::new(&self.gui.layout(), &self.root, 0)
                        .print_debug_tree(&mut buf, Default::default());
                    log::debug!("Done:\n{buf}",);
                }
            }
            #[cfg(feature = "gizmos")]
            Some(VirtualKeyCode::F21) => {
                self.debug.gizmo = !self.debug.gizmo;
                log::debug!(
                    "F21 pressed, gui debug gizmo toggled {}",
                    if self.debug.gizmo { "on" } else { "off" }
                );
            }
            _ => (),
        }
    }

    pub fn run_debug_pre_draw(&mut self, frame: &mut Frame) {
        #[cfg(feature = "gizmos")]
        {
            use srs2dge_gizmos::prelude::*;
            impl<'a> DebugTree<'a> {
                fn debug_gizmo(self, gizmos: &mut Gizmos, parent_layout: WidgetLayout) {
                    let layout = match self.layout.get(self.this) {
                        Ok(o) => o,
                        _ => return,
                    }
                    .to_absolute(parent_layout);

                    let colors = [
                        Color::AZURE,
                        Color::BLUE,
                        Color::CHARTREUSE,
                        Color::CYAN,
                        Color::GREEN,
                        Color::MAGENTA,
                        Color::MINT,
                        Color::ORANGE,
                        Color::RED,
                        Color::ROSE,
                        Color::VIOLET,
                        Color::YELLOW,
                    ];

                    gizmos.add_box(GizmosBox::new(
                        layout.offset + 0.5 * layout.size,
                        0.5 * layout.size,
                        colors[self.depth % colors.len()],
                    ));

                    for widget in self.this.subwidgets() {
                        Self::new(self.layout, widget, self.depth + 1).debug_gizmo(gizmos, layout);
                    }
                }
            }

            if self.debug.gizmo {
                DebugTree::new(self.gui.layout(), &self.root, 0)
                    .debug_gizmo(&mut self.debug.gizmos, Default::default());
                self.debug.gizmos.set_mvp(Mat4::orthographic_rh(
                    0.0,
                    self.gui.window_state().size.width as f32,
                    0.0,
                    self.gui.window_state().size.height as f32,
                    -1.0,
                    1.0,
                ));
                self.debug
                    .gizmos
                    .prepare(&mut self.target, frame, self.gui.window_state());
            }
        }
    }
}

impl AppDebug {
    pub fn run_debug_draw<'a>(&'a self, render_pass: RenderPass<'a>) -> RenderPass<'a> {
        #[cfg(feature = "gizmos")]
        if self.gizmo {
            return self.gizmos.draw(render_pass);
        };

        render_pass
    }
}