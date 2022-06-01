use srs2dge_core::{
    glam::Vec2, main_game_loop::prelude::WindowState, winit::dpi::PhysicalPosition,
};

//

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PointerState {
    Hover { now: Vec2 },
    Hold { initial: Vec2, now: Vec2 },
    Release { initial: Vec2, now: Vec2 },
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
            Self::Hold { initial, .. } => Self::Hold { initial, now },
            Self::Release { .. } => Self::Hover { now },
        }
    }

    pub fn pressed(&mut self) {
        *self = match *self {
            Self::Hover { now } => Self::Hold { initial: now, now },
            Self::Hold { initial, now } => Self::Hold { initial, now },
            Self::Release { initial, now } => Self::Hold { initial, now },
        }
    }

    pub fn released(&mut self) {
        *self = match *self {
            Self::Hover { now } => Self::Release { initial: now, now },
            Self::Hold { initial, now } => Self::Release { initial, now },
            Self::Release { initial, now } => Self::Release { initial, now },
        }
    }

    pub fn update(&mut self) {
        *self = match *self {
            Self::Hover { now } => Self::Hover { now },
            Self::Hold { initial, now } => Self::Hold { initial, now },
            Self::Release { now, .. } => Self::Hover { now },
        }
    }
}
