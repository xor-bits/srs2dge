#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Rect {
    pub width: u32,
    pub height: u32,
}

impl Rect {
    #[inline]
    pub const fn new(width: u32, height: u32) -> Self {
        Self { width, height }
    }

    #[inline]
    pub const fn positioned(self, x: u32, y: u32) -> PositionedRect {
        PositionedRect {
            x,
            y,
            width: self.width,
            height: self.height,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PositionedRect {
    pub x: u32,
    pub y: u32,

    pub width: u32,
    pub height: u32,
}

impl PositionedRect {
    #[inline]
    pub const fn new(x: u32, y: u32, width: u32, height: u32) -> Self {
        Self {
            x,
            y,
            width,
            height,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Packer {
    rect: Rect,
    rows: Vec<PositionedRect>,
    bottom: PositionedRect,
}

impl Packer {
    #[inline]
    pub const fn new(rect: Rect) -> Self {
        let rows = vec![];
        let bottom = rect.positioned(0, 0);
        Self { rect, rows, bottom }
    }

    pub fn push(&mut self, rect: Rect) -> Option<PositionedRect> {
        if rect.width == 0 || rect.height == 0 {
            return None;
        }

        // find a spot this new rectangle can fit into with minimal wasted space
        let (index, &row, _) = match self
            .rows
            .iter()
            .enumerate()
            .filter(|(_, row)| row.width >= rect.width && row.height >= rect.height)
            .map(|(index, row)| (index, row, row.height - rect.height))
            .min_by_key(|(_, _, wasted)| *wasted)
        {
            Some(s) => s,
            None => return self.push_row(rect),
        };

        // try pushing a new row if about to waste way too much
        if 2 * row.height > 3 * rect.height && self.can_push_row(rect) {
            match self.push_row(rect) {
                None => {}
                some => return some,
            }
        }

        // free space gets split into 1 or 2 new areas
        // 1 if the rectangle fits perfectly into the required space
        // 2 otherwise
        match row.width == rect.width {
            // width is the same
            // +-------+
            // | alloc |
            // +-------+
            true => {
                // pushed rectangle consumes the whole row
                self.rows.remove(index);
                Some(rect.positioned(row.x, row.y))
            }

            // +-----------+-----+
            // |   alloc   |     |
            // |-----------|  A  |
            // | discarded |     |
            // +-----------+-----+
            false => {
                let a = PositionedRect::new(
                    row.x + rect.width,
                    row.y,
                    row.width - rect.width,
                    row.height,
                );
                *self.rows.get_mut(index).unwrap() = a;
                Some(rect.positioned(row.x, row.y))
            }
        }
    }

    #[inline]
    const fn can_push_row(&self, rect: Rect) -> bool {
        self.bottom.height >= rect.height && self.bottom.width >= rect.width
    }

    #[inline]
    fn push_row(&mut self, rect: Rect) -> Option<PositionedRect> {
        self.bottom.height = self.bottom.height.checked_sub(rect.height)?;

        self.rows.push(PositionedRect::new(
            self.bottom.x + rect.width,
            self.bottom.y,
            self.bottom.width.checked_sub(rect.width)?,
            rect.height,
        ));

        self.bottom.y += rect.height;

        Some(rect.positioned(self.bottom.x, self.bottom.y - rect.height))
    }

    #[inline]
    pub fn remove() {}
}

#[cfg(test)]
mod test {
    use std::fs;

    use super::{Packer, Rect};
    use image::{Pixel, Rgba, RgbaImage};
    use rand::Rng;

    macro_rules! gen_test {
        ($packer:expr, $w:expr, $h:expr => $x:expr, $y:expr) => {
            let rect = Rect::new($w, $h);
            assert_eq! { $packer.push(rect), Some(rect.positioned($x, $y)) }
        };
        ($packer:expr, $w:expr, $h:expr) => {
            assert_eq! { $packer.push(Rect::new($w, $h)), None }
        };
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
    pub fn test_push_sort() {
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
        gen_test! { packer, 0, 0 };

        let mut packer = Packer::new(Rect::new(0, 0));
        gen_test! { packer, 10, 10 };

        let mut packer = Packer::new(Rect::new(0, 0));
        gen_test! { packer, 0, 0 };
    }

    /* #[test]
    pub fn test_remove() {
        let mut packer = Packer::new(Rect::new(200, 200));
        assert_eq!(packer.push(Rect::new(200, 100)), Some((0, 0)));
        assert_eq!(packer.push(Rect::new(200, 100)), Some((0, 100)));
        assert_eq!(packer.push(Rect::new(10, 10)), None);
        packer.remove(PositionedRect::new(0, 0, Rect::new(200, 100)));
        assert_eq!(packer.push(Rect::new(200, 100)), Some((0, 0)));
        assert_eq!(packer.push(Rect::new(10, 10)), None);
    }

    #[test]
    pub fn test_multi_remove() {
        for _ in 0..100 {
            let mut rng = rand::thread_rng();
            let mut packer = Packer::new(Rect::new(2000, 2000));
            for _ in 0..100 {
                packer.push(Rect::new(rng.gen_range(0..3000), rng.gen_range(0..3000)));
            }
            println!("{:?}", packer.space);
            packer.remove(PositionedRect::new(0, 0, Rect::new(2000, 2000)));
            println!("{:?}", packer.space);

            assert!(packer
                .space
                .windows(2)
                .all(|space| space[0].y <= space[1].y));

            assert_eq!(packer.space.len(), 1);
        }
    } */

    #[test]
    pub fn test_image() {
        const PACKER_RECT: Rect = Rect::new(500, 500);
        const PACKER_AREA: u32 = PACKER_RECT.width * PACKER_RECT.height;

        let mut img = RgbaImage::new(500, 500);
        let mut packer = Packer::new(PACKER_RECT);
        let mut rng = rand::thread_rng();

        fs::create_dir_all("packer/").unwrap();

        let rects: Vec<_> = (0..100)
            .map(|_| {
                (
                    Rgba::from_channels(
                        rng.gen_range(100..255),
                        rng.gen_range(100..255),
                        rng.gen_range(100..255),
                        rng.gen_range(100..255),
                    ),
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
            "packer/random.txt",
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
        img.save("packer/random.png").unwrap();
    }

    #[test]
    pub fn test_image_sorted() {
        const PACKER_RECT: Rect = Rect::new(500, 500);
        const PACKER_AREA: u32 = PACKER_RECT.width * PACKER_RECT.height;

        let mut img = RgbaImage::new(500, 500);
        let mut packer = Packer::new(PACKER_RECT);
        let mut rng = rand::thread_rng();

        fs::create_dir_all("packer/").unwrap();

        let mut rects: Vec<_> = (0..100)
            .map(|_| {
                (
                    Rgba::from_channels(
                        rng.gen_range(100..255),
                        rng.gen_range(100..255),
                        rng.gen_range(100..255),
                        rng.gen_range(100..255),
                    ),
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
            "packer/sorted.txt",
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
        img.save("packer/sorted.png").unwrap();
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
