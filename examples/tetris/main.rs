use logic::{Board, Move};
use std::{
    fs::File,
    io::{Read, Write},
};
use winit::event_loop::ControlFlow;

use srs2dge::{prelude::*, winit::event::VirtualKeyCode};

//

mod logic;
mod tetromino;

//

struct App {
    target: Target,

    ws: WindowState,
    kb: KeyboardState,
    ul: Option<UpdateLoop>,

    batcher: BatchRenderer,
    world_ubo: UniformBuffer<Mat4>,
    shader: Colored2DShader,

    glyphs: Glyphs,
    vbos: Vec<VertexBuffer>,
    ibos: Vec<IndexBuffer>,
    screen_ubo: UniformBuffer<Mat4>,
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
        let target = engine.new_target_default(target).await.unwrap();

        let ws = WindowState::new(&target.get_window().unwrap());
        let kb = KeyboardState::new();
        let ul = Some(UpdateLoop::new(UpdateRate::PerMinute(60)));

        let mut batcher = BatchRenderer::new(&target);
        let world_ubo = UniformBuffer::new(&target, 1);
        let shader = Colored2DShader::new(&target);

        let glyphs =
            Glyphs::new_bytes(&target, Rect::new(1024, 1024), Some(32), res::font::FIRA).unwrap();
        let screen_ubo = UniformBuffer::new(&target, 1);
        let text_shader = SdfShader::new(&target);

        let vbos = vec![VertexBuffer::new(&target, 0)];
        let ibos = vec![IndexBuffer::new(&target, 0)];

        let board = Board::new(&mut batcher, &mut rand::thread_rng());

        let highscore = (|| -> Result<usize, ()> {
            let mut highscore_file = File::options()
                .read(true)
                .open("highscore")
                .map_err(|_| ())?;
            let mut buf = String::new();
            highscore_file.read_to_string(&mut buf).map_err(|_| ())?;
            ron::from_str(&buf).map_err(|_| ())
        })()
        .unwrap_or(0);

        Self {
            target,

            ws,
            kb,
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
}

impl Runnable for App {
    fn event(&mut self, event: Event, _: &EventLoopTarget, control: &mut ControlFlow) {
        self.ws.event(&event);
        self.kb.event(&event);

        if self.ws.should_close {
            *control = ControlFlow::Exit;

            if let Err(err) = (|| -> Result<_, String> {
                let mut highscore_file = File::options()
                    .create(true)
                    .write(true)
                    .open("highscore")
                    .map_err(|err| err.to_string())?;
                write!(
                    highscore_file,
                    "{}",
                    ron::to_string(&self.highscore).map_err(|err| err.to_string())?
                )
                .map_err(|err| err.to_string())?;
                Ok(())
            })() {
                log::warn!("Failed to write highscore file: {err}");
            }
        }
    }

    fn draw(&mut self) {
        // manual moves
        if self.kb.just_pressed(VirtualKeyCode::Left) || self.kb.just_pressed(VirtualKeyCode::A) {
            self.board.update(&mut self.batcher, Move::Left);
        }
        if self.kb.just_pressed(VirtualKeyCode::Right) || self.kb.just_pressed(VirtualKeyCode::D) {
            self.board.update(&mut self.batcher, Move::Right);
        }
        if self.kb.just_pressed(VirtualKeyCode::Q) {
            self.board.update(&mut self.batcher, Move::RotateCCW);
        }
        if self.kb.just_pressed(VirtualKeyCode::W)
            || self.kb.just_pressed(VirtualKeyCode::E)
            || self.kb.just_pressed(VirtualKeyCode::Up)
        {
            self.board.update(&mut self.batcher, Move::RotateCW);
        }
        if self.kb.just_pressed(VirtualKeyCode::Down) || self.kb.just_pressed(VirtualKeyCode::S) {
            self.board.update(&mut self.batcher, Move::Down);
        }
        if self.kb.just_pressed(VirtualKeyCode::Space)
            || self.kb.just_pressed(VirtualKeyCode::Return)
        {
            self.board.update(&mut self.batcher, Move::Drop);
        }
        if self.game_over && self.kb.just_pressed(VirtualKeyCode::R) {
            let mut rng = rand::thread_rng();
            self.board = Board::new(&mut self.batcher, &mut rng);
            self.score = 999;
            self.game_over = false;
        }
        self.kb.clear();

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

                self.vbos.clear();
                self.ibos.clear();

                let s = FString::from_iter(["Game Over".default()]);
                let (v, i) = vbo::text(
                    &self.target,
                    &s,
                    &mut self.glyphs,
                    64.0,
                    Vec2::new(0.0, -200.0),
                    Some(Vec2::new(0.5, 0.0)),
                )
                .unwrap();
                self.vbos.push(VertexBuffer::new_with(&self.target, &v));
                self.ibos.push(IndexBuffer::new_with(&self.target, &i));

                let text = format!("Score: {score}");
                let s = FString::from_iter([text.default()]);
                let (v, i) = vbo::text(
                    &self.target,
                    &s,
                    &mut self.glyphs,
                    32.0,
                    Vec2::new(0.0, -264.0),
                    Some(Vec2::new(0.5, 0.0)),
                )
                .unwrap();
                self.vbos.push(VertexBuffer::new_with(&self.target, &v));
                self.ibos.push(IndexBuffer::new_with(&self.target, &i));

                let text = format!("Highscore: {highscore}");
                let s = FString::from_iter([text.default()]);
                let (v, i) = vbo::text(
                    &self.target,
                    &s,
                    &mut self.glyphs,
                    32.0,
                    Vec2::new(0.0, -296.0),
                    Some(Vec2::new(0.5, 0.0)),
                )
                .unwrap();
                self.vbos.push(VertexBuffer::new_with(&self.target, &v));
                self.ibos.push(IndexBuffer::new_with(&self.target, &i));

                let s = FString::from_iter(["Press R to restart".default()]);
                let (v, i) = vbo::text(
                    &self.target,
                    &s,
                    &mut self.glyphs,
                    24.0,
                    Vec2::new(0.0, -324.0),
                    Some(Vec2::new(0.5, 0.0)),
                )
                .unwrap();
                self.vbos.push(VertexBuffer::new_with(&self.target, &v));
                self.ibos.push(IndexBuffer::new_with(&self.target, &i));
            }
        } else {
            let score = self.board.score();
            if score != self.score {
                // score updated
                self.score = score;
                // increase speed
                self.ul = Some(UpdateLoop::new(UpdateRate::PerMinute(
                    60 + (self.score / 20) as u32,
                )));

                let text = format!("Score: {score}");
                let s = FString::from_iter([text.default()]);
                let (v, i) = vbo::text(
                    &self.target,
                    &s,
                    &mut self.glyphs,
                    32.0,
                    Vec2::new(0.0, -32.0),
                    Some(Vec2::new(0.5, 0.0)),
                )
                .unwrap();
                self.vbos = vec![VertexBuffer::new_with(&self.target, &v)];
                self.ibos = vec![IndexBuffer::new_with(&self.target, &i)];
            }
        }

        // draw
        let mut frame = self.target.get_frame();

        self.world_ubo.upload(
            &mut self.target,
            &mut frame,
            &[Mat4::orthographic_lh(
                -1.2 * self.ws.aspect,
                1.2 * self.ws.aspect,
                1.2,
                -1.2,
                -10.0,
                10.0,
            )],
        );
        self.screen_ubo.upload(
            &mut self.target,
            &mut frame,
            &[Mat4::orthographic_lh(
                -(self.ws.size.width as f32) * 0.5,
                self.ws.size.width as f32 * 0.5,
                -(self.ws.size.height as f32) * 1.0,
                0.0, /* self.ws.size.height as f32 * 0.5 */
                -10.0,
                10.0,
            )],
        );

        let (vbo, ibo) = self.batcher.generate(&mut self.target, &mut frame);

        let bg_a = self.shader.bind_group(&self.world_ubo);
        let bg_b = self
            .text_shader
            .bind_group((&self.screen_ubo, &self.glyphs));
        let mut pass = frame
            .primary_render_pass()
            .bind_vbo(vbo)
            .bind_ibo(ibo)
            .bind_group(&bg_a)
            .bind_shader(&self.shader)
            .draw_indexed(0..ibo.capacity() as _, 0, 0..1)
            .done();

        for (vbo, ibo) in self.vbos.iter().zip(self.ibos.iter()) {
            pass = pass
                .bind_vbo(vbo)
                .bind_ibo(ibo)
                .bind_group(&bg_b)
                .bind_shader(&self.text_shader)
                .draw_indexed(0..ibo.capacity() as _, 0, 0..1)
                .done()
        }

        drop(pass);

        self.target.finish_frame(frame);
    }
}

//

main_app!(async App);
