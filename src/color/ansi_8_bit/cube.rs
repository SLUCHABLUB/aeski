use crate::cell::AsciiCell;
use crate::color::ansi_8_bit::cube_coordinate::CubeCoordinate;
use crate::color::ansi_8_bit::{BACKGROUND, FOREGROUND, SECOND_ARGUMENT};
use crate::color::variants::CUBE;
use crate::color::{default_new_cell, Color};
use crate::font::Font;
use image::{DynamicImage, Pixel, Rgb, SubImage};
use std::io::Write;
use rounded_div::RoundedDiv;
use crate::color::util::average_color;

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
            self.r.get() * 51,
            self.g.get() * 51,
            self.b.get() * 51,
        ])
    }

    fn from_rgb(color: Rgb<u8>) -> Self {
        Cube {
            r: CubeCoordinate::new(color.0[0].rounded_div(51)),
            g: CubeCoordinate::new(color.0[1].rounded_div(51)),
            b: CubeCoordinate::new(color.0[2].rounded_div(51)),
        }
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

    fn new_cell<G: AsRef<[char]>>(view: SubImage<&DynamicImage>, font: &Font<G>) -> AsciiCell<Self> {
        default_new_cell(&CUBE, average_color(*view).to_rgb(), font)
    }
}
