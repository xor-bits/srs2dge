use super::{line::GizmosLine, Gizmos};
use srs2dge_core::{
    color::Color, glam::Vec2, main_game_loop::prelude::WindowState, winit::dpi::PhysicalPosition,
};
use srs2dge_text::fontsdf::math::Line;

//

#[derive(Debug, Clone, Copy)]
pub struct GizmosText<'s> {
    origin: Vec2,
    ws: &'s WindowState,
    text: &'s str,
    col: Color,
}

//

impl<'s> GizmosText<'s> {
    pub fn new(origin: Vec2, ws: &'s WindowState, text: &'s str, col: Color) -> Self {
        Self {
            origin,
            ws,
            text,
            col,
        }
    }

    pub fn lines(self, base: &mut Gizmos) -> Result<(), &'static str> {
        let font = base.font.as_ref().ok_or("No font set")?;

        let mvp = base.mvp(self.ws);
        let px = Gizmos::screen_to_world(mvp, self.ws, PhysicalPosition::new(28, 0))
            .ok_or("Invalid projection")?
            .x;
        let px = px
            - Gizmos::screen_to_world(mvp, self.ws, PhysicalPosition::new(0, 0))
                .ok_or("Invalid projection")?
                .x;
        let sf = font.scale_factor(px);
        let sf = Vec2::new(sf, -sf);

        let col = self.col;
        let mut origin = self.origin;

        self.text
            .chars()
            .flat_map(|character| {
                let index = font.lookup_glyph_index(character);

                let metrics = font.metrics_indexed(index, px, false);

                let iter =
                    font.geometry_indexed(index)
                        .0
                        .iter_lines()
                        .map(move |Line { from, to }| GizmosLine {
                            from: from * sf + origin,
                            to: to * sf + origin,
                            col,
                        });

                origin += Vec2::new(metrics.advance_width, 0.0);

                iter
            })
            .collect::<Vec<_>>()
            .into_iter()
            .for_each(|line| base.add_line(line));

        Ok(())
    }
}
