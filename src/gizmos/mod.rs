//! Immediate mode rendering
//!
//! Easy/quick but slow rendering
//!
//! Usage:
//!
//! create `Gizmos`
//!
//! while render:
//!     add all sorts of gizmos
//!     prepare
//!     draw

use self::{
    circle::{GizmosCircle, GizmosCircles},
    line::{GizmosLine, GizmosLines},
    r#box::{GizmosBox, GizmosBoxes},
    text::GizmosText,
};
use crate::{
    prelude::{RenderPass, UniformBuffer},
    target::Target,
    Frame,
};
use fontsdf::Font;
use glam::{Mat4, Vec2, Vec4, Vec4Swizzles};
use main_game_loop::prelude::WindowState;
use winit::dpi::{PhysicalPosition, Pixel};

//

pub mod r#box;
pub mod circle;
pub mod line;
pub mod text;

//

pub struct Gizmos {
    mat: Option<Mat4>,
    current_mat: Mat4,
    ubo: UniformBuffer<Mat4>,
    font: Font,

    lines: GizmosLines,
    circles: GizmosCircles,
    boxes: GizmosBoxes,
}

//

impl Gizmos {
    /// position ranges are
    /// - x: -aspect..aspect
    /// - y: -1.0..1.0
    pub fn new(target: &Target) -> Self {
        let current_mat = Mat4::IDENTITY;
        let ubo = UniformBuffer::new_single(target, current_mat);
        let font = Font::from_bytes(include_bytes!("../../res/font/fira/font.ttf")).unwrap();

        let lines = GizmosLines::new(target, &ubo);
        let circles = GizmosCircles::new(target, &ubo);
        let boxes = GizmosBoxes::new(target, &ubo);

        Self {
            mat: None,
            current_mat,
            ubo,
            font,

            lines,
            circles,
            boxes,
        }
    }

    #[inline(always)]
    pub fn add_line(&mut self, line: GizmosLine) {
        self.lines.push(line);
    }

    #[inline(always)]
    pub fn add_circle(&mut self, circle: GizmosCircle) {
        self.circles.push(circle);
    }

    #[inline(always)]
    pub fn add_box(&mut self, r#box: GizmosBox) {
        self.boxes.push(r#box);
    }

    /// Just use proper text rendering
    #[inline(always)]
    pub fn add_text(&mut self, text: GizmosText) -> Option<()> {
        text.lines(self)
    }

    /// Override the Model View Projection (MVP) matrix
    /// for the next draw
    ///
    /// Defaults to:
    /// `Mat4::orthographic_rh(-aspect, aspect, 1.0, -1.0, -100.0, 100.0)`
    /// if not set
    pub fn set_mvp(&mut self, mvp: Mat4) {
        self.mat = Some(mvp);
    }

    /// Returns the current mvp matrix
    pub fn mvp(&self, ws: &WindowState) -> Mat4 {
        if let Some(mat) = self.mat {
            // custom
            mat
        } else {
            // default
            Mat4::orthographic_rh(-ws.aspect, ws.aspect, 1.0, -1.0, -100.0, 100.0)
        }
    }

    /// Return the inverse of the current mvp matrix
    pub fn inverse_mvp(&self, ws: &WindowState) -> Option<Mat4> {
        let mvp = self.mvp(ws);
        if mvp.determinant() != 0.0 {
            Some(mvp.inverse())
        } else {
            None
        }
    }

    /// A slow way to convert screen space coordinates
    /// to world space coordinates
    pub fn screen_to_world<T>(&self, ws: &WindowState, pos: PhysicalPosition<T>) -> Option<Vec2>
    where
        T: Pixel,
    {
        let screen_pos = pos.cast::<f32>();
        let render_area_pos = Vec4::new(
            screen_pos.x / ws.size.width as f32 * 2.0 - 1.0,
            1.0 - screen_pos.y / ws.size.height as f32 * 2.0,
            0.0,
            1.0,
        );
        let world_pos = self.inverse_mvp(ws)? * render_area_pos;
        Some(world_pos.xy())
    }

    /// Prepares the Gizmos for drawing
    ///
    /// It clears all submitted gizmos and
    /// uploads them to their VBOs/IBOs
    pub fn prepare(&mut self, target: &mut Target, frame: &mut Frame, ws: &WindowState) {
        // take the optional custom or default matrix
        let mat = self.mvp(ws);
        self.mat = None;
        // don't waste time uploading if the matrix is the same
        if mat != self.current_mat {
            // upload the matrix
            self.ubo.upload(target, frame, &[mat]);
            self.current_mat = mat;
        }

        // actual gizmos
        self.lines.prepare(target, frame);
        self.circles.prepare(target, frame);
        self.boxes.prepare(target, frame);
    }

    /// Ideally
    /// Preparing just once is ok, but really something meant to be done
    pub fn draw<'e, Sv, Bv, Si, Bi, const PIPELINE_BOUND: bool>(
        &'e self,
        render_pass: RenderPass<'e, Sv, Bv, Si, Bi, PIPELINE_BOUND>,
    ) -> RenderPass<'e> {
        self.boxes
            .draw(self.circles.draw(self.lines.draw(render_pass)))
    }
}
