use logic::{Board, Move};
use std::sync::Arc;
use winit::dpi::PhysicalSize;

use saveload::*;
use srs2dge::prelude::*;

//

mod logic;
mod saveload;
mod tetromino;

//

struct App {
    target: Target,

    ws: WindowState,
    ks: KeyboardState,
    gs: GamepadState,
    ul: Option<UpdateLoop>,

    batcher: BatchRenderer,
    world_ubo: UniformBuffer,
    shader: Colored2DShader,

    glyphs: Glyphs,
    vbos: VertexBuffer,
    ibos: IndexBuffer,
    screen_ubo: UniformBuffer<SdfUniform>,
    text_shader: SdfShader,

    board: Board,
    score: usize,
    highscore: usize,
    game_over: bool,
}

//

impl App {
    async fn init(target: &EventLoopTarget) -> Self {
        let engine = Engine::new();
        let target = engine
            .new_target(Arc::new(
                WindowBuilder::new()
                    .with_visible(false)
                    .with_inner_size(PhysicalSize::new(300, 600))
                    .build(target)
                    .unwrap(),
            ))
            .await;

        let ws = WindowState::new(&target.get_window().unwrap());
        let ks = KeyboardState::new();
        let gs = GamepadState::new();
        let ul = Some(UpdateLoop::new(UpdateRate::PerMinute(60)));

        let mut batcher = BatchRenderer::new(&target);
        let world_ubo = UniformBuffer::new(&target, 1);
        let shader = Colored2DShader::new(&target);

        let glyphs = Glyphs::new_with_fallback_bytes(
            &target,
            Rect::new(1024, 1024),
            Some(32),
            res::font::FIRA,
            None,
        )
        .unwrap();
        let screen_ubo = UniformBuffer::new(&target, 1);
        let text_shader = SdfShader::new(&target);

        let vbos = VertexBuffer::new(&target, 0);
        let ibos = IndexBuffer::new(&target, 0);

        let board = Board::new(&mut batcher, &mut rand::thread_rng());

        let highscore = load_highscore();

        Self {
            target,

            ws,
            ks,
            gs,
            ul,

            batcher,
            world_ubo,
            shader,

            glyphs,
            vbos,
            ibos,
            screen_ubo,
            text_shader,

            board,
            score: 999,
            highscore,
            game_over: false,
        }
    }

    async fn event(&mut self, event: Event<'_>, _: &EventLoopTarget, control: &mut ControlFlow) {
        self.ws.event(&event);
        self.ks.event(&event);
        self.gs.event(&event);

        if self.ws.should_close {
            *control = ControlFlow::Exit;

            save_highscore(self.highscore);
        }
    }

    async fn draw(&mut self) {
        // manual moves
        if self.ks.just_pressed(VirtualKeyCode::Left)
            || self.ks.just_pressed(VirtualKeyCode::A)
            || self.gs.just_pressed_first(GamepadButton::DPadLeft) == Some(true)
        {
            self.board.update(&mut self.batcher, Move::Left);
        }
        if self.ks.just_pressed(VirtualKeyCode::Right)
            || self.ks.just_pressed(VirtualKeyCode::D)
            || self.gs.just_pressed_first(GamepadButton::DPadRight) == Some(true)
        {
            self.board.update(&mut self.batcher, Move::Right);
        }
        if self.ks.just_pressed(VirtualKeyCode::Q)
            || self.gs.just_pressed_first(GamepadButton::LeftTrigger) == Some(true)
            || self.gs.just_pressed_first(GamepadButton::LeftTrigger2) == Some(true)
        {
            self.board.update(&mut self.batcher, Move::RotateCCW);
        }
        if self.ks.just_pressed(VirtualKeyCode::W)
            || self.ks.just_pressed(VirtualKeyCode::E)
            || self.ks.just_pressed(VirtualKeyCode::Up)
            || self.gs.just_pressed_first(GamepadButton::DPadUp) == Some(true)
            || self.gs.just_pressed_first(GamepadButton::RightTrigger) == Some(true)
            || self.gs.just_pressed_first(GamepadButton::RightTrigger2) == Some(true)
        {
            self.board.update(&mut self.batcher, Move::RotateCW);
        }
        if self.ks.just_pressed(VirtualKeyCode::Down)
            || self.ks.just_pressed(VirtualKeyCode::S)
            || self.gs.just_pressed_first(GamepadButton::DPadDown) == Some(true)
        {
            self.board.update(&mut self.batcher, Move::Down);
        }
        if self.ks.just_pressed(VirtualKeyCode::Space)
            || self.ks.just_pressed(VirtualKeyCode::Return)
            || self.gs.just_pressed_first(GamepadButton::South) == Some(true)
        {
            self.board.update(&mut self.batcher, Move::Drop);
        }
        if self.game_over && self.ks.just_pressed(VirtualKeyCode::R)
            || self.gs.just_pressed_first(GamepadButton::North) == Some(true)
        {
            let mut rng = rand::thread_rng();
            self.board = Board::new(&mut self.batcher, &mut rng);
            self.score = 999;
            self.game_over = false;
        }
        self.ks.clear();
        self.gs.clear();

        // tick moves
        let mut ul = self.ul.take().unwrap();
        ul.update(|| {
            self.board.update(&mut self.batcher, Move::Down);
        });
        self.ul = Some(ul);

        if self.board.game_over() {
            let just_happened = self.board.game_over() != self.game_over;
            self.game_over = true;

            if just_happened {
                let score = self.board.score();
                self.highscore = self.highscore.max(score);
                let highscore = self.highscore;

                // TODO: automatic centering for multiple lines

                let text = FormatString::from_iter([
                    "Game Over".into(),
                    24.0.into(),
                    format!("\nScore: {score}").into(),
                    format!("\nHighscore: {highscore}").into(),
                    18.0.into(),
                    "\nPress R/Δ/Y to restart".into(),
                ])
                .with_init(Format {
                    color: Color::WHITE,
                    font: 0,
                    px: 48.0,
                });
                let (v, i) = vbo::text(
                    &self.target,
                    text.chars(),
                    &mut self.glyphs,
                    TextConfig {
                        x_origin: 0.0,
                        y_origin: -5.0,
                        align: TextAlign::top(),
                        ..Default::default()
                    },
                )
                .unwrap()
                .collect_mesh();
                self.vbos = VertexBuffer::new_with(&self.target, &v);
                self.ibos = IndexBuffer::new_with(&self.target, &i);
            }
        } else {
            let score = self.board.score();
            if score != self.score {
                // score updated
                self.score = score;
                // increase speed
                self.ul = Some(UpdateLoop::new(UpdateRate::PerMinute(
                    60 + (self.score / 50) as u32,
                )));

                let text = FormatString::from(format!("Score: {score}")).with_init(Format {
                    color: Color::WHITE,
                    font: 0,
                    px: 32.0,
                });
                let (v, i) = vbo::text(
                    &self.target,
                    text.chars(),
                    &mut self.glyphs,
                    TextConfig {
                        x_origin: 5.0,
                        y_origin: -5.0,
                        align: TextAlign::top(),
                        ..Default::default()
                    },
                )
                .unwrap()
                .collect_mesh();
                self.vbos = VertexBuffer::new_with(&self.target, &v);
                self.ibos = IndexBuffer::new_with(&self.target, &i);
            }
        }

        // draw
        let mut frame = self.target.get_frame();

        self.world_ubo.upload(
            &mut self.target,
            &mut frame,
            &[Mat4::orthographic_lh(
                -1.0 * self.ws.aspect,
                1.0 * self.ws.aspect,
                1.0,
                -1.0,
                -10.0,
                10.0,
            )],
        );
        self.screen_ubo.upload(
            &mut self.target,
            &mut frame,
            &[SdfUniform::new_defaults(Mat4::orthographic_lh(
                self.ws.size.width as f32 * -0.5,
                self.ws.size.width as f32 * 0.5,
                -(self.ws.size.height as f32) * 1.0,
                0.0,
                -10.0,
                10.0,
            ))],
        );

        let (vbo, ibo, i) = self.batcher.generate(&mut self.target, &mut frame);

        frame
            .primary_render_pass()
            .bind_vbo(vbo)
            .bind_ibo(ibo)
            .bind_group(&self.shader.bind_group(&self.world_ubo))
            .bind_shader(&self.shader)
            .draw_indexed(0..i, 0, 0..1)
            .bind_vbo(&self.vbos)
            .bind_ibo(&self.ibos)
            .bind_group(
                &self
                    .text_shader
                    .bind_group((&self.screen_ubo, &self.glyphs)),
            )
            .bind_shader(&self.text_shader)
            .draw_indexed(0..self.ibos.capacity() as _, 0, 0..1);

        self.target.finish_frame(frame);
    }
}

//

fn main() {
    app!(App);
}
