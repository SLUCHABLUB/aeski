use aeski::color::ansi_4_bit::Ansi4Bit;
use aeski::font::Font;
use aeski::image::AsciiImage;
use image::ImageReader;
use std::error::Error;
use std::io::{stdin, stdout, Write};

const GRADIENT: [char; 10] = [' ', '.', ':', '-', '=', '+', '*', '#', '%', '@'];
const COVERAGE: f64 = 0.3;
const ASPECT_RATIO: f64 = 0.5;

fn input(prompt: &str) -> std::io::Result<String> {
    print!("{prompt}");
    stdout().flush()?;

    let mut buffer = String::new();
    stdin().read_line(&mut buffer)?;
    Ok(buffer)
}

fn main() -> Result<(), Box<dyn Error>> {
    let font = Font::new_float(GRADIENT, COVERAGE, ASPECT_RATIO).unwrap();

    let image_path = input("Path to image: ")?;
    let image = ImageReader::open(image_path.trim())?.decode()?;

    let width = input("Width in characters: ")?.trim().parse()?;
    let ascii_image = AsciiImage::<Ansi4Bit>::from_image_with_width(&image, &font, width);

    println!("{ascii_image}");

    Ok(())
}
