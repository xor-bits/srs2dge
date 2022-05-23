use rand::Rng;
use winit::event_loop::ControlFlow;

use srs2dge::{prelude::*, winit::event::VirtualKeyCode};

//

const SHAPES: &[([Shape; 4], Color)] = &[
    (
        [
            // O
            [(0, 0), (1, 0), (0, 1), (1, 1)],
            [(0, 0), (1, 0), (0, 1), (1, 1)],
            [(0, 0), (1, 0), (0, 1), (1, 1)],
            [(0, 0), (1, 0), (0, 1), (1, 1)],
        ],
        Color::YELLOW,
    ),
    (
        [
            // I
            [(1, -1), (1, 0), (1, 1), (1, 2)],
            [(-1, 1), (0, 1), (1, 1), (2, 1)],
            [(2, -1), (2, 0), (2, 1), (2, 2)],
            [(-1, 2), (0, 2), (1, 2), (2, 2)],
        ],
        Color::CYAN,
    ),
    (
        [
            // Y
            [(1, 0), (0, 1), (1, 1), (2, 1)],
            [(-1, 1), (0, 1), (1, 1), (2, 1)],
            [(2, -1), (2, 0), (2, 1), (2, 2)],
            [(-1, 2), (0, 2), (1, 2), (2, 2)],
        ],
        Color::MAGENTA,
    ),
];

//

struct App {
    target: Target,

    ws: WindowState,
    kb: KeyboardState,
    ul: Option<UpdateLoop>,

    texture: Texture,

    grid: [[Tile; 10]; 20],
    active: Shape,
    color: Color,

    batcher: BatchRenderer,
    ubo: UniformBuffer<Mat4>,
    shader: Colored2DShader,
}

#[derive(Debug, Clone, Copy, Default)]
struct Tile {
    idx: Idx,
    state: TileState,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum TileState {
    Moving,
    Stopped,
    Empty,
}

type Shape = [(i8, i8); 4];

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Direction {
    Down,
    Left,
    Right,
    // RotateL,
    // RotateR,
}

//

impl Direction {
    fn to_offs(self) -> (i8, i8) {
        match self {
            Direction::Down => (0, 1),
            Direction::Left => (-1, 0),
            Direction::Right => (1, 0),
        }
    }
}

//

impl Default for TileState {
    fn default() -> Self {
        Self::Empty
    }
}

//

impl App {
    async fn init(target: &EventLoopTarget) -> Self {
        let engine = Engine::new();
        let target = engine.new_target_default(target).await.unwrap();

        let ws = WindowState::new(&target.get_window().unwrap());
        let kb = KeyboardState::new();
        let ul = Some(UpdateLoop::new(UpdateRate::PerSecond(5)));

        let texture = Texture::new_rgba_with(
            &target,
            &image::load_from_memory(res::texture::EMPTY)
                .unwrap()
                .to_rgba8(),
        );

        let mut batcher = BatchRenderer::new(&target);
        let ubo = UniformBuffer::new(&target, 1);
        let shader = Colored2DShader::new(&target);

        let mut grid = [[Tile::default(); 10]; 20];
        let mut rng = rand::thread_rng();
        for (x, y, tile) in grid.iter_mut().enumerate().flat_map(|(y, row)| {
            row.iter_mut()
                .enumerate()
                .map(move |(x, tile)| (x, y, tile))
        }) {
            tile.idx = batcher.push_with(QuadMesh {
                pos: Vec2::new(x as f32 - 4.5, y as f32 - 9.5) / 9.5,
                size: Vec2::ONE / 9.8,
                col: Color::DARK_GRAY,
                tex: TexturePosition::default(),
            });
        }

        let mut active = SHAPES[0].0[0];
        let mut color = SHAPES[0].1;
        for (x, y) in active.iter_mut() {
            *x += 1;
            *y += 1;
        }

        let mut res = Self {
            target,

            ws,
            kb,
            ul,

            texture,

            batcher,
            ubo,
            shader,

            grid,
            active,
            color,
        };

        // res.spawn(0, 5, 1);

        res
    }

    fn spawn(&mut self, id: usize, x: i8, y: i8) {
        let shape = SHAPES[id].0[1];
        let color = SHAPES[id].1;
        for (xo, yo) in shape {
            let tile = &mut self.grid[(y + yo) as usize][(x + xo) as usize];
            self.batcher.get_mut(tile.idx).col = color;
            tile.state = TileState::Moving;
        }
    }

    fn check_collision(&self, dir: Direction) -> bool {
        let (xo, yo) = dir.to_offs();
        for (x, y) in self.active.iter().copied() {
            if y + yo < 0
                || y + yo >= 20
                || x + xo < 0
                || x + xo >= 10
                || self.grid[(y + yo) as usize][(x + xo) as usize].state == TileState::Stopped
            {
                return true;
            }
        }

        false
    }

    fn move_shape(&mut self, dir: Direction) -> bool {
        if self.check_collision(dir) {
            return true;
        }

        let (xo, yo) = dir.to_offs();
        for (x, y) in self.active.iter().copied() {
            let x = x as usize;
            let y = y as usize;
            self.grid[y][x].state = TileState::Empty;
            let t = self.batcher.get_mut(self.grid[y][x].idx);
            t.col = Color::DARK_GRAY;
        }
        for (x, y) in self.active.iter().copied() {
            let x = (x + xo) as usize;
            let y = (y + yo) as usize;
            self.grid[y][x].state = TileState::Moving;
            self.batcher.get_mut(self.grid[y][x].idx).col = self.color;
        }
        for (x, y) in self.active.iter_mut() {
            *x += xo;
            *y += yo;
        }

        false
    }

    fn update(&mut self, dir: Direction) -> bool {
        // move piece
        let new = if self.move_shape(dir) && dir == Direction::Down {
            // new piece
            for (x, y) in self.active.iter().copied() {
                let x = x as usize;
                let y = y as usize;
                self.grid[y][x].state = TileState::Stopped;
            }

            let id = rand::thread_rng().gen_range(0..SHAPES.len());
            // self.spawn(id, 5, 1);
            self.active = SHAPES[id].0[0];
            self.color = SHAPES[id].1;
            for (x, y) in self.active.iter_mut() {
                *x += 1;
                *y += 1;
            }

            true
        } else {
            false
        };

        // clear rows
        for y in 0..20 {
            let mut full = true;
            for x in 0..10 {
                if self.grid[y][x].state != TileState::Stopped {
                    full = false;
                }
            }
            if full {
                if y != 0 {
                    for y in (1..=y).rev() {
                        for x in 0..10 {
                            self.grid[y][x].state = self.grid[y - 1][x].state;
                            self.batcher.get_mut(self.grid[y][x].idx).col =
                                self.batcher.get(self.grid[y - 1][x].idx).col;
                        }
                    }
                }
                /* for x in 0..10 {
                    self.grid[y][x].state = TileState::Empty;
                    self.batcher.get_mut(self.grid[y][x].idx).col = Color::DARK_GRAY;
                } */
            }
        }

        new
    }
}

impl Runnable for App {
    fn event(&mut self, event: Event, _: &EventLoopTarget, control: &mut ControlFlow) {
        self.ws.event(&event);
        self.kb.event(&event);

        if self.ws.should_close {
            *control = ControlFlow::Exit;
        }
    }

    fn draw(&mut self) {
        let mut rng = rand::thread_rng();
        if self.kb.just_pressed(VirtualKeyCode::Left) || self.kb.just_pressed(VirtualKeyCode::A) {
            self.update(Direction::Left);
        }
        if self.kb.just_pressed(VirtualKeyCode::Right) || self.kb.just_pressed(VirtualKeyCode::D) {
            self.update(Direction::Right);
        }
        if self.kb.just_pressed(VirtualKeyCode::Space)
            || self.kb.just_pressed(VirtualKeyCode::Return)
        {
            while !self.update(Direction::Down) {}
        }
        self.kb.clear();

        let mut ul = self.ul.take().unwrap();
        ul.update(|| {
            self.update(Direction::Down);
        });
        self.ul = Some(ul);

        let mut frame = self.target.get_frame();

        self.ubo.upload(
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

        let (vbo, ibo) = self.batcher.generate(&mut self.target, &mut frame);

        frame
            .primary_render_pass()
            .bind_vbo(vbo)
            .bind_ibo(ibo)
            .bind_group(&self.shader.bind_group(&self.ubo))
            .bind_shader(&self.shader)
            .draw_indexed(0..ibo.capacity() as _, 0, 0..1);

        self.target.finish_frame(frame);
    }
}

//

main_app!(async App);
