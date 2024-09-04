use aeski::color::ansi_4_bit::Ansi4Bit;
use aeski::font::Font;
use aeski::image::AsciiImage;
use criterion::{criterion_group, criterion_main, Criterion};
use image::ImageFormat::Jpeg;
use image::ImageReader;
use num_rational::Ratio;
use std::hint::black_box;
use std::io::Cursor;

const GRADIENT: [char; 10] = [' ', '.', ':', '-', '=', '+', '*', '#', '%', '@'];
const COVERAGE: Ratio<usize> = Ratio::new_raw(3, 10);
const ASPECT_RATIO: Ratio<usize> = Ratio::new_raw(1, 2);

const WIDTH: usize = 100;

fn mona_lisa_4_bit(criterion: &mut Criterion) {
    let data = Cursor::new(include_bytes!("Mona Lisa.jpg"));
    let image = ImageReader::with_format(data, Jpeg).decode().unwrap();
    let font = Font::new(GRADIENT, COVERAGE, ASPECT_RATIO).unwrap();

    criterion.bench_function("Mona Lisa", |bencher| {
        bencher.iter(|| {
            AsciiImage::<Ansi4Bit>::from_image_with_width(
                black_box(&image),
                black_box(&font),
                black_box(WIDTH),
            )
        })
    });
}

criterion_group!(benches, mona_lisa_4_bit);
criterion_main!(benches);
