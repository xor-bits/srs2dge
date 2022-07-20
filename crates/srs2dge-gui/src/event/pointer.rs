use srs2dge_core::{
    glam::Vec2,
    main_game_loop::prelude::WindowState,
    winit::{dpi::PhysicalPosition, event::MouseButton},
};

//

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PointerState {
    Hover {
        now: Vec2,
    },
    Press {
        button: MouseButton,
        now: Vec2,
    },
    Hold {
        button: MouseButton,
        initial: Vec2,
        now: Vec2,
    },
    Release {
        button: MouseButton,
        initial: Vec2,
        now: Vec2,
    },
}

//

impl Default for PointerState {
    fn default() -> Self {
        Self::Hover { now: Vec2::ZERO }
    }
}

impl PointerState {
    pub fn moved_physical(&mut self, now: PhysicalPosition<f64>, ws: &WindowState) {
        self.moved(Vec2::new(
            now.x as f32,
            ws.size.height as f32 - now.y as f32,
        ))
    }

    pub fn moved(&mut self, now: Vec2) {
        *self = match *self {
            Self::Hover { .. } => Self::Hover { now },
            Self::Press { button, now: old } => Self::Hold {
                button,
                initial: old,
                now,
            },
            Self::Hold {
                button, initial, ..
            } => Self::Hold {
                button,
                initial,
                now,
            },
            Self::Release { .. } => Self::Hover { now },
        }
    }

    pub fn pressed(&mut self, button: MouseButton) {
        *self = match *self {
            Self::Hover { now } => Self::Press { button, now },
            Self::Press { now, .. } => Self::Press { button, now },
            Self::Hold { initial, now, .. } => Self::Hold {
                button,
                initial,
                now,
            },
            Self::Release { initial, now, .. } => Self::Hold {
                button,
                initial,
                now,
            },
        }
    }

    pub fn released(&mut self, button: MouseButton) {
        *self = match *self {
            Self::Hover { now } => Self::Release {
                button,
                initial: now,
                now,
            },
            Self::Press { now, .. } => Self::Release {
                button,
                initial: now,
                now,
            },
            Self::Hold { initial, now, .. } => Self::Release {
                button,
                initial,
                now,
            },
            Self::Release { initial, now, .. } => Self::Release {
                button,
                initial,
                now,
            },
        }
    }

    pub fn update(&mut self) {
        *self = match *self {
            Self::Hover { now } => Self::Hover { now },
            Self::Press { button, now } => Self::Hold {
                button,
                initial: now,
                now,
            },
            Self::Hold {
                button,
                initial,
                now,
            } => Self::Hold {
                button,
                initial,
                now,
            },
            Self::Release { now, .. } => Self::Hover { now },
        }
    }
}
