/* #![feature(bench_black_box)]

use instant::Instant;
use srs2dge::prelude::*;
use std::hint::black_box;

fn main() {
    let fonts = Fonts::new_bytes(res::font::ROBOTO).unwrap();

    let text = FormatString::from_iter([
        Color::RED.into(),
        "red ".into(),
        Color::GREEN.into(),
        "green ".into(),
        Color::BLUE.into(),
        "blue".into(),
    ])
    .with_init(Format {
        px: 80.0,
        ..Default::default()
    });
    let config = TextConfig::default();

    // it runs 1M times per 0.5 seconds on my pc?
    // I didn't expect it to be this fast
    // I most likely made this benchmark incorrectly this is just so unexpectedly fast
    // perf improvement: 5x
    let i = Instant::now();
    for _ in 0..1_000_000 {
        TextChars::new(text.chars(), &fonts, config)
            .map(black_box)
            .for_each(|_| {});
    }
    println!("iter elapsed: {:?}", i.elapsed());

    // and this is even faster
    // running 10M times per 0.7 seconds??
    // perf improvement: 2x
    let i = Instant::now();
    for _ in 0..10_000_000 {
        black_box(
            FormatString::from_iter([
                Color::RED.into(),
                "red ".into(),
                Color::GREEN.into(),
                "green ".into(),
                Color::BLUE.into(),
                "blue".into(),
            ])
            .with_init(Format {
                px: 80.0,
                ..Default::default()
            }),
        );
    }
    println!("iter elapsed: {:?}", i.elapsed());
}
 */
fn main() {}
