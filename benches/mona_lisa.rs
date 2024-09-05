use aeski::color::ansi_4_bit::Ansi4Bit;
use aeski::font::Font;
use aeski::image::AsciiImage;
use criterion::{criterion_group, criterion_main, Criterion};
use image::ImageFormat::Jpeg;
use image::ImageReader;
use std::hint::black_box;
use std::io::Cursor;
use aeski::color::ansi_8_bit::Ansi8Bit;

const GRADIENT: [char; 10] = [' ', '.', ':', '-', '=', '+', '*', '#', '%', '@'];
const COVERAGE: f64 = 0.3;
const ASPECT_RATIO: f64 = 1.0 / 2.0;

const WIDTH: u32 = 100;

fn mona_lisa_4_bit(criterion: &mut Criterion) {
    let data = Cursor::new(include_bytes!("Mona Lisa.jpg"));
    let image = ImageReader::with_format(data, Jpeg).decode().unwrap();
    let font = Font::new_float(GRADIENT, COVERAGE, ASPECT_RATIO).unwrap();

    criterion.bench_function("4 bit", |bencher| {
        bencher.iter(|| {
            AsciiImage::<Ansi4Bit>::from_image_with_width(
                black_box(&image),
                black_box(&font),
                black_box(WIDTH),
            )
        })
    });

    criterion.bench_function("8 bit", |bencher| {
        bencher.iter(|| {
            AsciiImage::<Ansi8Bit>::from_image_with_width(
                black_box(&image),
                black_box(&font),
                black_box(WIDTH),
            )
        })
    });
}

criterion_group!(benches, mona_lisa_4_bit);
criterion_main!(benches);
