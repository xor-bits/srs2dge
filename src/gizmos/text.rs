use super::{line::GizmosLine, Gizmos};
use fontsdf::math::Line;
use glam::{Vec2, Vec4};
use main_game_loop::prelude::WindowState;
use winit::dpi::PhysicalPosition;

//

pub struct GizmosText<'s> {
    origin: Vec2,
    ws: &'s WindowState,
    text: &'s str,
    col: Vec4,
}

//

impl<'s> GizmosText<'s> {
    pub fn new(origin: Vec2, ws: &'s WindowState, text: &'s str, col: Vec4) -> Self {
        Self {
            origin,
            ws,
            text,
            col,
        }
    }

    pub fn lines(self, base: &mut Gizmos) -> Option<()> {
        let px = base
            .screen_to_world(self.ws, PhysicalPosition::new(28, 0))?
            .x;
        let px = px
            - base
                .screen_to_world(self.ws, PhysicalPosition::new(0, 0))?
                .x;
        let sf = base.font.scale_factor(px);
        let sf = Vec2::new(sf, -sf);

        let col = self.col;
        let mut origin = self.origin;

        for character in self.text.chars() {
            let index = base.font.lookup_glyph_index(character);

            let metrics = base.font.metrics_indexed(index, px, false);

            base.font
                .geometry_indexed(index)
                .0
                .iter_lines()
                .map(move |Line { from, to }| GizmosLine {
                    from: from * sf + origin,
                    to: to * sf + origin,
                    col,
                })
                .collect::<Vec<_>>()
                .into_iter()
                .for_each(|line| base.add_line(line));

            origin += Vec2::new(metrics.advance_width, 0.0);
        }

        Some(())
    }
}
