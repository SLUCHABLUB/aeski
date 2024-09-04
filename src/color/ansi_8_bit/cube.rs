use crate::cell::AsciiCell;
use crate::color::ansi_8_bit::cube_coordinate::CubeCoordinate;
use crate::color::ansi_8_bit::{BACKGROUND, FOREGROUND, SECOND_ARGUMENT};
use crate::color::variants::CUBE;
use crate::color::{default_from_rgb, default_new_cell, Color};
use crate::font::Font;
use image::Rgb;
use num_rational::Ratio;
use std::io::Write;

const OFFSET: u8 = 16;

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub struct Cube {
    pub r: CubeCoordinate,
    pub g: CubeCoordinate,
    pub b: CubeCoordinate,
}

impl Cube {
    pub const fn new(r: u8, g: u8, b: u8) -> Self {
        Cube {
            r: CubeCoordinate::new(r),
            g: CubeCoordinate::new(g),
            b: CubeCoordinate::new(b),
        }
    }
}

impl Color for Cube {
    fn to_rgb(&self) -> Rgb<u8> {
        Rgb([
            *Ratio::new(self.r.get(), 5).round().numer(),
            *Ratio::new(self.g.get(), 5).round().numer(),
            *Ratio::new(self.b.get(), 5).round().numer(),
        ])
    }

    fn from_rgb(color: Rgb<u8>) -> Self {
        default_from_rgb(&CUBE, color)
    }

    fn write_background(&self, mut to: impl Write) -> std::io::Result<()> {
        to.write_all(&[
            BACKGROUND,
            SECOND_ARGUMENT,
            OFFSET + 36 * self.r.get() + 6 * self.g.get() + self.b.get(),
        ])
    }

    fn write_foreground(&self, mut to: impl Write) -> std::io::Result<()> {
        to.write_all(&[
            FOREGROUND,
            SECOND_ARGUMENT,
            OFFSET + 36 * self.r.get() + 6 * self.g.get() + self.b.get(),
        ])
    }

    fn new_cell<G: AsRef<[char]>>(color: Rgb<u8>, font: &Font<G>) -> AsciiCell<Self> {
        default_new_cell(&CUBE, color, font)
    }
}
