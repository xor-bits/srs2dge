//! 2D Texture packer with ability to reuse areas

use super::rect::{PositionedRect, Rect};
use integer_sqrt::IntegerSquareRoot;

//

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
struct Space {
    x: u32,
    width: u32,
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
struct Row {
    y: u32,
    height: u32,

    free_spaces: Vec<Space>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Packer {
    rect: Rect,
    rows: Vec<Row>,
    bottom: PositionedRect,
    pub padding: u8,
}

impl Default for Packer {
    fn default() -> Self {
        Self::from_side(0)
    }
}

impl Packer {
    /// Creates a Packer with where the area is a rectangle.
    pub const fn new(rect: Rect) -> Self {
        let rows = vec![];
        let bottom = rect.positioned(0, 0);
        Self {
            rect,
            rows,
            bottom,
            padding: 0,
        }
    }

    /// Creates a Packer with where the area is a square.
    ///
    /// Side length will be `<side>`.
    pub const fn from_side(side: u32) -> Self {
        Self::new(Rect {
            height: side,
            width: side,
        })
    }

    /// Creates a Packer with where the area is a square.
    ///
    /// Side length will be `ceil(sqrt(<area>))`.
    pub fn from_area(area: u32) -> Self {
        Self::from_side(area.integer_sqrt())
    }

    pub fn with_padding(mut self, padding: u8) -> Self {
        self.padding = padding;
        self
    }

    pub fn area(&self) -> Rect {
        self.rect
    }

    /// The resulting rect will be the sums the rectangles (**sides**, not area)
    ///
    /// ```text
    /// +----------+-------+
    /// | original | new   |
    /// +----------+-------+
    /// | new      | alloc |
    /// +----------+-------+
    /// ```
    pub fn alloc_more(&mut self, rect: Rect) {
        // expand the area
        self.rect.width += rect.width;
        self.rect.height += rect.height;
        self.bottom.width += rect.width;
        self.bottom.height += rect.height;

        // add width to all rows
        if rect.width != 0 {
            for row in self.rows.iter_mut() {
                if let Some(last) = row.free_spaces.last_mut() {
                    // last free space
                    last.width += rect.width;
                } else {
                    // add one free space, because there weren't any
                    let x = self.rect.width - rect.width;
                    let width = rect.width;
                    row.free_spaces.push(Space { x, width });
                }
            }
        }
    }

    /// Expands each side length to be the next power of two.
    ///
    /// ex:
    ///  - 30x35 -> 32x64
    ///  - 32x35 -> 64x64
    pub fn next_pow2(&mut self) {
        let width = Self::u32_next_pow2(self.rect.width) - self.rect.width;
        let height = Self::u32_next_pow2(self.rect.height) - self.rect.height;
        self.alloc_more(Rect { width, height })
    }

    /// Expands each side length to be the next power of two of the biggest side.
    ///
    /// Optimal for GPU textures.
    ///
    /// ex:
    ///  - 30x35 -> 35x35 -> 64x64
    ///  - 32x32 -> 64x64
    pub fn next_pow2_square(&mut self) {
        let side = self.rect.width.max(self.rect.height);
        let width = Self::u32_next_pow2(side) - side;
        let height = width;
        self.alloc_more(Rect { width, height })
    }

    fn u32_next_pow2(mut i: u32) -> u32 {
        // if > 1
        //   no effects
        // if = 1
        //   result will be 1 anyways
        // if = 0
        //   wraps to u32::MAX and wraps back to 0 when adding 1
        // i = i.wrapping_sub(1);

        // set all bits to one to the LEFT from most significant **1** bit
        i |= i >> 1;
        i |= i >> 2;
        i |= i >> 4;
        i |= i >> 8;
        i |= i >> 16;

        // add 1 to convert from ex: 0x00011111 to 0x00100000
        // because after setting bits to 1 from LEFT, the result
        // is `the_next_power_of_two - 1`
        i = i.wrapping_add(1);

        i
    }

    /// Push a rectangle into this packer.
    ///
    /// The optimal way is to push the tallest rectangles first.
    pub fn push(&mut self, rect: Rect) -> Option<PositionedRect> {
        if rect.width == 0 || rect.height == 0 {
            return Some(rect.positioned(0, 0));
        }

        let pad = self.padding as u32;
        if rect.width + pad > self.rect.width || rect.height + pad > self.rect.height {
            return None;
        }

        // find a spot where this new rectangle can fit (while wasting as little space as possible)
        let (row, col, score) = match self
            .rows
            .iter()
            .enumerate()
            .filter(|(_, row)| row.height >= rect.height + pad)
            .flat_map(|(index_row, row)| {
                row.free_spaces
                    .iter()
                    .enumerate()
                    .filter(|(_, row)| row.width >= rect.width + pad)
                    .map(move |(index_col, _)| (index_row, index_col))
            })
            .map(|(row, col)| (row, col, self.rows[row].height - rect.height - pad))
            .min_by_key(|(_, _, wasted)| *wasted)
        {
            Some(s) => s,
            None => return self.push_row(rect),
        };

        // try pushing a new row if about to waste way too much
        if score > rect.height + pad && self.can_push_row(rect) {
            match self.push_row(rect) {
                None => {}
                some => return some,
            }
        }

        let (x, y, w, l) = {
            let row = &self.rows[row];
            let space = &row.free_spaces[col];
            (space.x, row.y, space.width, row.free_spaces.len())
        };

        // free space gets split into 1 or 2 new areas
        // 1 if the rectangle fits perfectly into the required space
        // 2 otherwise
        match (w == rect.width + pad, l) {
            // width is the same
            // +--+-------+--+
            // |//| free  |//|
            // +--+-------+--+
            //   \/ \/ \/
            // +--+-------+--+
            // |//| alloc |//|
            // +--+-------+--+
            (true, 1) => {
                // pushed rectangle consumes the whole row
                self.rows[row].free_spaces.remove(col);
            }

            // width is the same
            // +------+--+-------+--+------+
            // | free |//| free  |//| free |
            // +------+--+-------+--+------+
            //   \/ \/ \/
            // +------+--+-------+--+------+
            // | free |//| alloc |//| free |
            // +------+--+-------+--+------+
            (true, _) => {
                // pushed rectangle consumes the whole free space
                self.rows[row].free_spaces.remove(col);
            }

            // +------+--+----------------+--+
            // | free |//|      free      |//|
            // +------+--+----------------+--+
            //   \/ \/ \/
            // +------+--+---------+------+--+
            // | free |//|  alloc  | free |//|
            // +------+--+---------+------+--+
            (false, _) => {
                let a = Space {
                    x: x + rect.width + pad,
                    width: w - rect.width - pad,
                };
                self.rows[row].free_spaces[col] = a;
            }
        }

        let rect = rect.positioned(x, y);

        // assert!(rect.x + rect.width <= self.rect.width && rect.y + rect.height <= self.rect.height);

        Some(rect)
    }

    /// Repeatedly pushes a rect until it succeeds.
    /// With each fail, it expands the area.
    ///
    /// It will stop if any side length reaches this limit.
    pub fn push_until(&mut self, rect: Rect, limit: u16) -> Option<PositionedRect> {
        loop {
            match self.push(rect) {
                Some(pos) => return Some(pos),
                None => {
                    let lim = limit as u32;
                    if self.rect.width >= lim || self.rect.height >= lim {
                        return None;
                    }
                    self.next_pow2_square();
                }
            }
        }
    }

    #[inline]
    const fn can_push_row(&self, rect: Rect) -> bool {
        let pad = self.padding as u32;
        self.bottom.height >= rect.height + pad && self.bottom.width >= rect.width + pad
    }

    #[inline]
    fn push_row(&mut self, rect: Rect) -> Option<PositionedRect> {
        let pad = self.padding as u32;
        let width = self.bottom.width.checked_sub(rect.width + pad)?;
        self.bottom.height = self.bottom.height.checked_sub(rect.height + pad)?;

        let free_spaces = if width != 0 {
            vec![Space {
                x: self.bottom.x + rect.width + pad,
                width,
            }]
        } else {
            vec![]
        };

        self.rows.push(Row {
            y: self.bottom.y,
            height: rect.height + pad,

            free_spaces,
        });

        self.bottom.y += rect.height + pad;

        Some(rect.positioned(self.bottom.x, self.bottom.y - rect.height - pad))
    }

    #[inline]
    fn aabb_1d(x1: u32, x2: u32, w1: u32, w2: u32) -> bool {
        // 2 * (x1 + w1 / 2).abs_diff(x2 + w1 / 2) < w1 + w2
        x2 <= x1 + w1 && x2 + w2 > x1
    }

    #[inline]
    fn remove_at_line(row: &mut Row, rect: &PositionedRect) -> bool {
        if !Self::aabb_1d(row.y, rect.y, row.height, rect.height) {
            return false;
        }

        // row.free_spaces
        //     .drain_filter(|col| Self::aabb_1d(col.x, rect.x, col.width, rect.width));
        let mut tmp = Default::default();
        std::mem::swap(&mut tmp, &mut row.free_spaces);
        row.free_spaces = tmp
            .into_iter()
            .partition(|col| Self::aabb_1d(col.x, rect.x, col.width, rect.width))
            .1;

        if row.free_spaces.is_empty() {
            true
        } else {
            row.free_spaces.push(Space {
                x: rect.x,
                width: rect.width,
            });
            false
        }
    }

    /// Remove all quads that collide with `rect`.
    pub fn remove(&mut self, rect: PositionedRect) {
        let last_merged = self
            .rows
            .iter_mut()
            .enumerate()
            .filter_map(|(index, row)| {
                if Self::remove_at_line(row, &rect) {
                    None
                } else {
                    Some(index)
                }
            })
            .last()
            .unwrap_or(0);

        self.rows.drain(last_merged..);

        if let Some(last) = self.rows.last() {
            self.bottom.y = last.y;
            self.bottom.height = self.rect.height - last.y - last.height;
        } else {
            self.bottom.y = 0;
            self.bottom.height = self.rect.height;
        }
    }
}

#[cfg(test)]
mod test {
    use super::{Packer, Rect};
    use image::{Rgba, RgbaImage};
    use rand::Rng;
    use std::fs;

    macro_rules! gen_test {
        ($packer:expr, $w:expr, $h:expr => $x:expr, $y:expr) => {
            let rect = Rect::new($w, $h);
            assert_eq! { $packer.push(rect), Some(rect.positioned($x, $y)) }
        };
        ($packer:expr, $w:expr, $h:expr) => {
            assert_eq! { $packer.push(Rect::new($w, $h)), None }
        };
        ($packer:expr, $w:expr, $h:expr ; $x:expr, $y:expr) => {
            $packer.remove(Rect::new($w, $h).positioned($x, $y));
        };
    }

    #[test]
    fn next_pow2_square_test() {
        assert_eq!(Packer::u32_next_pow2(8), 16);
        assert_eq!(Packer::u32_next_pow2(7), 8);
        assert_eq!(Packer::u32_next_pow2(6), 8);
        assert_eq!(Packer::u32_next_pow2(5), 8);
        assert_eq!(Packer::u32_next_pow2(4), 8);
        assert_eq!(Packer::u32_next_pow2(3), 4);
        assert_eq!(Packer::u32_next_pow2(2), 4);
        assert_eq!(Packer::u32_next_pow2(1), 2);
        assert_eq!(Packer::u32_next_pow2(0), 1);
    }

    #[test]
    pub fn test_push_vertical() {
        let mut packer = Packer::new(Rect::new(200, 200));
        gen_test! { packer, 200, 100 => 0, 0 };
        gen_test! { packer, 200, 100 => 0, 100 };
        gen_test! { packer, 200, 100 };
    }

    #[test]
    pub fn test_push_horizontal() {
        let mut packer = Packer::new(Rect::new(200, 200));
        gen_test! { packer, 100, 200 => 0, 0 };
        gen_test! { packer, 100, 200 => 100, 0 };
        gen_test! { packer, 100, 200 };
    }

    #[test]
    pub fn test_push_grid() {
        let mut packer = Packer::new(Rect::new(20, 20));
        gen_test! { packer, 10, 10 => 0, 0 };
        gen_test! { packer, 10, 10 => 10, 0 };
        gen_test! { packer, 10, 10 => 0, 10 };
        gen_test! { packer, 10, 10 => 10, 10 };
        gen_test! { packer, 10, 10 };
    }

    #[test]
    pub fn test_push_mixed() {
        let mut packer = Packer::new(Rect::new(20, 20));
        gen_test! { packer, 20, 10 => 0, 0 };
        gen_test! { packer, 10, 10 => 0, 10 };
        gen_test! { packer, 10, 10 => 10, 10 };
        gen_test! { packer, 10, 10 };

        let mut packer = Packer::new(Rect::new(30, 20));
        gen_test! { packer, 20, 10 => 0, 0 };
        gen_test! { packer, 10, 10 => 20, 0 };
        gen_test! { packer, 10, 10 => 0, 10 };
        gen_test! { packer, 10, 10 => 10, 10 };
        gen_test! { packer, 20, 10 };
    }

    #[test]
    pub fn test_push_fuzz() {
        for _ in 0..100 {
            let mut rng = rand::thread_rng();
            let mut packer = Packer::new(Rect::new(2000, 2000));
            for _ in 0..100 {
                packer.push(Rect::new(rng.gen_range(0..3000), rng.gen_range(0..3000)));
            }
        }
    }

    #[test]
    pub fn test_push_empty() {
        let mut packer = Packer::new(Rect::new(100, 100));
        gen_test! { packer, 110, 110 };
        gen_test! { packer, 0, 0 => 0, 0 };

        let mut packer = Packer::new(Rect::new(0, 0));
        gen_test! { packer, 10, 10 };

        let mut packer = Packer::new(Rect::new(0, 0));
        gen_test! { packer, 0, 0 => 0, 0 };
    }

    #[test]
    pub fn test_remove() {
        let mut packer = Packer::new(Rect::new(200, 200));
        gen_test! { packer, 200, 100 => 0, 0 };
        gen_test! { packer, 200, 100 => 0, 100 };
        gen_test! { packer, 200, 100 };

        gen_test! { packer, 200, 100 ; 0, 0 };

        gen_test! { packer, 200, 100 => 0, 0 };
        gen_test! { packer, 10, 10 };
    }

    #[test]
    pub fn test_multi_remove() {
        for _ in 0..100 {
            let mut rng = rand::thread_rng();
            let mut packer = Packer::new(Rect::new(2000, 2000));
            for _ in 0..100 {
                let push = packer.push(Rect::new(rng.gen_range(0..3000), rng.gen_range(0..3000)));
                let some = push.is_some();
                if let Some(rect) = push {
                    packer.remove(rect);
                }

                assert_eq!(
                    packer,
                    Packer::new(Rect::new(2000, 2000)),
                    "was some {} ",
                    some
                );
            }
        }
    }

    #[test]
    pub fn test_multi_remove_all() {
        for _ in 0..100 {
            let mut rng = rand::thread_rng();
            let mut packer = Packer::new(Rect::new(2000, 2000));
            for _ in 0..100 {
                packer.push(Rect::new(rng.gen_range(0..3000), rng.gen_range(0..3000)));
            }
            packer.remove(Rect::new(2000, 2000).positioned(0, 0));

            assert_eq!(packer, Packer::new(Rect::new(2000, 2000)));
        }
    }

    const TARGET_DIR: &str = "/tmp/srs2dge-test/";
    const TARGET_DIR_RT: &str = "/tmp/srs2dge-test/random.txt";
    const TARGET_DIR_RP: &str = "/tmp/srs2dge-test/random.png";
    const TARGET_DIR_ST: &str = "/tmp/srs2dge-test/sorted.txt";
    const TARGET_DIR_SP: &str = "/tmp/srs2dge-test/sorted.png";

    #[test]
    pub fn test_image() {
        const PACKER_RECT: Rect = Rect::new(500, 500);
        const PACKER_AREA: u32 = PACKER_RECT.width * PACKER_RECT.height;

        let mut img = RgbaImage::new(500, 500);
        let mut packer = Packer::new(PACKER_RECT);
        let mut rng = rand::thread_rng();
        fs::create_dir_all(TARGET_DIR).unwrap();

        let rects: Vec<_> = (0..100)
            .map(|_| {
                (
                    Rgba([
                        rng.gen_range(100..255),
                        rng.gen_range(100..255),
                        rng.gen_range(100..255),
                        rng.gen_range(100..255_u8),
                    ]),
                    Rect::new(rng.gen_range(5..100), rng.gen_range(5..100)),
                )
            })
            .filter_map(|(color, rect)| Some((color, packer.push(rect)?)))
            .collect();

        // area %
        let used_area = rects
            .iter()
            .fold(0, |acc, (_, rect)| acc + rect.width * rect.height);
        fs::write(
            TARGET_DIR_RT,
            format!("Area%:{}", used_area as f64 / PACKER_AREA as f64 * 100.0),
        )
        .unwrap();

        // packer image
        for (color, rect) in rects.into_iter() {
            for y in rect.y..rect.y + rect.height {
                for x in rect.x..rect.x + rect.width {
                    img.put_pixel(x, y, color);
                }
            }
        }
        img.save(TARGET_DIR_RP).unwrap();
    }

    #[test]
    pub fn test_image_sorted() {
        const PACKER_RECT: Rect = Rect::new(500, 500);
        const PACKER_AREA: u32 = PACKER_RECT.width * PACKER_RECT.height;

        let mut img = RgbaImage::new(500, 500);
        let mut packer = Packer::new(PACKER_RECT);
        let mut rng = rand::thread_rng();

        fs::create_dir_all(TARGET_DIR).unwrap();

        let mut rects: Vec<_> = (0..100)
            .map(|_| {
                (
                    Rgba([
                        rng.gen_range(100..255),
                        rng.gen_range(100..255),
                        rng.gen_range(100..255),
                        rng.gen_range(100..255_u8),
                    ]),
                    Rect::new(rng.gen_range(5..100), rng.gen_range(5..100)),
                )
            })
            .collect();

        rects.sort_unstable_by_key(|(_, rect)| rect.height);
        rects.reverse();

        let rects: Vec<_> = rects
            .into_iter()
            .filter_map(|(color, rect)| Some((color, packer.push(rect)?)))
            .collect();

        // area %
        let used_area = rects
            .iter()
            .fold(0, |acc, (_, rect)| acc + rect.width * rect.height);
        fs::write(
            TARGET_DIR_ST,
            format!("Area%:{}", used_area as f64 / PACKER_AREA as f64 * 100.0),
        )
        .unwrap();

        // packer image
        for (color, rect) in rects.into_iter() {
            for y in rect.y..rect.y + rect.height {
                for x in rect.x..rect.x + rect.width {
                    img.put_pixel(x, y, color);
                }
            }
        }
        img.save(TARGET_DIR_SP).unwrap();
    }

    #[test]
    pub fn test_push_alloc_more() {
        let mut packer = Packer::new(Rect::new(20, 20));
        gen_test! { packer, 10, 10 => 0, 0 };
        gen_test! { packer, 10, 10 => 10, 0 };
        gen_test! { packer, 10, 10 => 0, 10 };
        gen_test! { packer, 10, 10 => 10, 10 };
        gen_test! { packer, 10, 10 };
        packer.alloc_more(Rect::new(10, 0));
        gen_test! { packer, 10, 10 => 20, 0 };
        gen_test! { packer, 10, 10 => 20, 10 };
        gen_test! { packer, 10, 10 };
    }

    #[test]
    pub fn test_padding() {
        let mut packer = Packer::new(Rect::new(20, 20)).with_padding(2);
        gen_test! { packer, 8, 8 => 0, 0 };
        gen_test! { packer, 8, 8 => 10, 0 };
        gen_test! { packer, 8, 8 => 0, 10 };
        gen_test! { packer, 8, 8 => 10, 10 };
        gen_test! { packer, 8, 8 };
        gen_test! { packer, 1, 1 };

        let mut packer = Packer::new(Rect::new(8, 8)).with_padding(2);
        gen_test! { packer, 1, 1 => 0, 0 };
        gen_test! { packer, 1, 1 => 3, 0 };
    }

    /* #[bench]
    pub fn bench_packing(bencher: &mut Bencher) {
        let mut packer = Packer::new(Rect {
            width: 500,
            height: 500,
        })
        .unwrap();
        let mut rng = rand::thread_rng();

        bencher.iter(|| {
            packer.push(Rect {
                width: rng.gen_range(5..100),
                height: rng.gen_range(5..100),
            })
        });
    } */
}
