use crate::tetromino::Tetromino;
use rand::Rng;

use srs2dge::prelude::*;

//

#[derive(Debug, Clone, Copy, Default)]
pub struct Board {
    board: [[Tile; 10]; 20],
    tetromino: Tetromino,
    background: Idx,
    score: usize,
    game_over: bool,
}

#[derive(Debug, Clone, Copy, Default)]
pub struct Tile {
    idx: Idx,
    state: TileState,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TileState {
    Moving,
    Stopped,
    Empty,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Move {
    Drop,
    Down,
    Left,
    Right,
    RotateCW,
    RotateCCW,
}

//

impl Default for TileState {
    fn default() -> Self {
        Self::Empty
    }
}

impl Default for Move {
    fn default() -> Self {
        Self::Down
    }
}

impl Board {
    pub fn new<R: Rng>(batcher: &mut BatchRenderer, rng: &mut R) -> Self {
        let mut board = [[Tile::default(); 10]; 20];
        let tetromino = Tetromino::new(rng);
        let background = batcher.push_with(QuadMesh {
            pos: Vec2::ZERO,
            size: Vec2::new(1.0, 2.0),
            col: tetromino.color(),
            tex: TexturePosition::default(),
        });

        // init board
        board
            .iter_mut()
            .enumerate()
            .flat_map(|(y, row)| {
                row.iter_mut()
                    .enumerate()
                    .map(move |(x, tile)| (x, y, tile))
            })
            .for_each(|(x, y, tile)| tile.init(batcher, x, y));

        let mut res = Self {
            board,
            tetromino,
            background,
            score: 0,
            game_over: false,
        };

        res.draw(batcher);

        res
    }

    fn check_collision(&self, m: Move) -> bool {
        !self.tetromino.apply(m).iter().all(|(x, y)| {
            (0..10).contains(&x)
                && (0..20).contains(&y)
                && self.board[y as usize][x as usize].state != TileState::Stopped
        })
    }

    fn clear_tetromino(&mut self, batcher: &mut BatchRenderer) {
        self.tetromino.iter().for_each(|(x, y)| {
            self.board[y as usize][x as usize].reset(batcher);
        });
    }

    fn print_tetromino(&mut self, batcher: &mut BatchRenderer) {
        self.tetromino.iter().for_each(|(x, y)| {
            self.board[y as usize][x as usize].tetromino(batcher, self.tetromino.color());
        });
    }

    fn move_tetromino(&mut self, m: Move) -> bool {
        if self.check_collision(m) {
            return true;
        }

        self.tetromino = self.tetromino.apply(m);

        false
    }

    fn clear_rows(&mut self, batcher: &mut BatchRenderer) {
        let mut rows = 0;
        for y in 0..20 {
            let mut full = true;
            for x in 0..10 {
                if self.board[y][x].state != TileState::Stopped {
                    full = false;
                }
            }
            if full && y != 0 {
                rows += 1;
                for y in (1..=y).rev() {
                    for x in 0..10 {
                        self.board[y][x].state = self.board[y - 1][x].state;
                        batcher.get_mut(self.board[y][x].idx).unwrap().col =
                            batcher.get(self.board[y - 1][x].idx).unwrap().col;
                    }
                }
            }
        }

        self.score += match rows {
            0 => 0,
            1 => 100,
            2 => 300,
            3 => 500,
            4 => 800,
            _ => unreachable!(),
        };
    }

    fn draw(&mut self, batcher: &mut BatchRenderer) {
        // draw tetromino + shadow
        self.print_tetromino(batcher);
        let mut copy = *self;
        while !copy.move_tetromino(Move::Down) {}
        for (x, y) in copy.tetromino.iter() {
            batcher
                .get_mut(self.board[y as usize][x as usize].idx)
                .unwrap()
                .size = Vec2::ONE / 11.0;
        }
    }

    pub fn update(&mut self, batcher: &mut BatchRenderer, m: Move) {
        if self.game_over {
            return;
        }

        self.clear_tetromino(batcher);
        for y in 0..20 {
            for x in 0..10 {
                if batcher.get(self.board[y][x].idx).unwrap().size.x <= 0.1 {
                    batcher.get_mut(self.board[y][x].idx).unwrap().size = Vec2::ONE / 10.0;
                }
            }
        }

        // move piece
        let tetromino_dropped = if m == Move::Drop {
            while !self.move_tetromino(Move::Down) {
                self.score += 2;
            }
            true
        } else {
            if m == Move::Down {
                self.score += 1;
            }
            self.move_tetromino(m) && m == Move::Down
        };

        // stop the old piece
        if tetromino_dropped {
            self.print_tetromino(batcher);
            self.tetromino.iter().for_each(|(x, y)| {
                self.board[y as usize][x as usize].state = TileState::Stopped;
            });
        };

        // clear full rows
        self.clear_rows(batcher);

        // new piece
        if tetromino_dropped {
            self.tetromino = Tetromino::new(&mut rand::thread_rng());
            for (x, y) in self.tetromino.iter() {
                self.game_over |= self.board[y as usize][x as usize].state == TileState::Stopped;
            }
            if self.game_over {
                return;
            }
            batcher.get_mut(self.background).unwrap().col = self.tetromino.color();
        }

        self.draw(batcher);
    }

    pub fn score(&self) -> usize {
        self.score
    }

    pub fn game_over(&self) -> bool {
        self.game_over
    }
}

impl Tile {
    pub fn init(&mut self, batcher: &mut BatchRenderer, x: usize, y: usize) {
        self.idx = batcher.push_with(QuadMesh {
            pos: Vec2::new(x as f32 - 4.5, y as f32 - 9.5) / 10.0,
            size: Vec2::ONE / 10.0,
            col: Color::BLACK,
            tex: TexturePosition::default(),
        });
    }

    pub fn reset(&mut self, batcher: &mut BatchRenderer) {
        self.state = TileState::Empty;
        batcher.get_mut(self.idx).unwrap().col = Color::BLACK;
    }

    pub fn tetromino(&mut self, batcher: &mut BatchRenderer, color: Color) {
        self.state = TileState::Moving;
        batcher.get_mut(self.idx).unwrap().col = color;
    }
}
