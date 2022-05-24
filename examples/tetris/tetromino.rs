use rand::Rng;
use srs2dge::color::Color;

use crate::logic::Move;

//

const F: bool = false;
const T: bool = true;
const TETROMINOS: [&[&[bool]]; 7] = [
    &[
        //
        &[F, T, F],
        &[F, T, F],
        &[F, T, F],
        &[F, T, F],
    ],
    &[
        //
        &[T, T],
        &[T, T],
    ],
    &[
        //
        &[F, T],
        &[T, T],
        &[T, F],
    ],
    &[
        //
        &[T, F],
        &[T, T],
        &[F, T],
    ],
    &[
        //
        &[F, T, F],
        &[T, T, F],
        &[F, T, F],
    ],
    &[
        //
        &[T, T, F],
        &[F, T, F],
        &[F, T, F],
    ],
    &[
        //
        &[F, T, T],
        &[F, T, F],
        &[F, T, F],
    ],
];
const TETROMINO_COLORS: [Color; 7] = [
    Color::CYAN,
    Color::YELLOW,
    Color::RED,
    Color::GREEN,
    Color::MAGENTA,
    Color::ORANGE,
    Color::AZURE,
];

//

#[derive(Debug, Clone, Copy, Default)]
pub struct Tetromino {
    x: i8,
    y: i8,

    ty: u8,
    rotation: u8,
}

//

impl Tetromino {
    pub fn new<R: Rng>(rng: &mut R) -> Self {
        Self {
            x: 0,
            y: 0,

            ty: rng.gen_range(0..TETROMINOS.len() as u8),
            rotation: 0,
        }
    }

    #[must_use]
    pub fn rotate_cw(mut self) -> Self {
        self.rotation = (self.rotation + 1) % 4;
        self
    }

    #[must_use]
    pub fn rotate_ccw(mut self) -> Self {
        self.rotation = (self.rotation + 3) % 4;
        self
    }

    #[must_use]
    pub fn apply(mut self, m: Move) -> Self {
        match m {
            Move::Drop => unreachable!(),
            Move::Down => self.y += 1,
            Move::Left => self.x -= 1,
            Move::Right => self.x += 1,
            Move::RotateCW => return self.rotate_cw(),
            Move::RotateCCW => return self.rotate_ccw(),
        }
        self
    }

    pub fn iter(&self) -> impl Iterator<Item = (i8, i8)> {
        let xo = self.x;
        let yo = self.y;
        let ty = self.ty;
        let rotation = self.rotation;
        let mut w = Self::width(ty);
        let mut h = Self::height(ty);
        if rotation == 1 || rotation == 3 {
            std::mem::swap(&mut w, &mut h);
        }
        (0..w)
            .flat_map(move |y| (0..h).map(move |x| (x, y)))
            .filter(move |&(x, y)| Self::get(ty, rotation, x, y))
            .map(move |(x, y)| (xo + x as i8, yo + y as i8))
    }

    pub fn color(&self) -> Color {
        TETROMINO_COLORS[self.ty as usize]
    }

    fn width(ty: u8) -> usize {
        TETROMINOS[ty as usize].len()
    }

    fn height(ty: u8) -> usize {
        TETROMINOS[ty as usize][0].len()
    }

    fn get(ty: u8, rotation: u8, x: usize, y: usize) -> bool {
        match rotation {
            0 => TETROMINOS[ty as usize][y][x],
            1 => TETROMINOS[ty as usize][Self::width(ty) - 1 - x][y],
            2 => TETROMINOS[ty as usize][Self::width(ty) - 1 - y][Self::height(ty) - 1 - x],
            3 => TETROMINOS[ty as usize][x][Self::height(ty) - 1 - y],
            _ => unreachable!(),
        }
    }
}
