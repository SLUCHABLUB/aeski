use crate::color::ansi_4_bit::Ansi4Bit;
use crate::color::{default_from_rgb, Color};
use cube::Cube;
use enum_iterator::Sequence;
use grayscale::Grayscale;
use image::{Luma, Pixel, Rgb};
use std::io::Write;

pub mod cube;
pub mod cube_coordinate;
pub mod grayscale;

const BACKGROUND: u8 = 48;
const FOREGROUND: u8 = 38;
const SECOND_ARGUMENT: u8 = 5;

/// Represents an 8-bit ansi color.
/// Can either be a 4-bit ansi color,
/// a 6×6×6 cube color,
/// or a grayscale value on the interval [0; 23].
#[derive(Copy, Clone, Eq, Hash, Debug, Sequence)]
pub enum Ansi8Bit {
    Ansi4Bit(Ansi4Bit),
    Cube(Cube),
    Grayscale(Grayscale),
}

impl Default for Ansi8Bit {
    fn default() -> Self {
        Ansi8Bit::Ansi4Bit(Ansi4Bit::default())
    }
}

impl PartialEq for Ansi8Bit {
    fn eq(&self, other: &Self) -> bool {
        // TODO: avoid conversion here
        self.to_rgb() == other.to_rgb()
    }
}

impl Color for Ansi8Bit {
    fn to_rgb(&self) -> Rgb<f64> {
        match self {
            Ansi8Bit::Ansi4Bit(color) => color.to_rgb(),
            Ansi8Bit::Cube(color) => color.to_rgb(),
            Ansi8Bit::Grayscale(color) => color.to_rgb(),
        }
    }

    fn from_rgb(color: Rgb<u8>) -> Self {
        let luma = color.to_luma();

        if luma.to_rgb() == color {
            let Luma([luma]) = luma;
            let grayscale = (luma as f64 / 256.0 * 23.0).round() as u8;

            return Ansi8Bit::Grayscale(Grayscale::new(grayscale)).into();
        }

        default_from_rgb(color)
    }

    fn write_background(&self, to: impl Write) -> std::io::Result<()> {
        match self {
            Ansi8Bit::Ansi4Bit(color) => color.write_background(to),
            Ansi8Bit::Cube(color) => color.write_background(to),
            Ansi8Bit::Grayscale(color) => color.write_background(to),
        }
    }

    fn write_foreground(&self, to: impl Write) -> std::io::Result<()> {
        match self {
            Ansi8Bit::Ansi4Bit(color) => color.write_foreground(to),
            Ansi8Bit::Cube(color) => color.write_foreground(to),
            Ansi8Bit::Grayscale(color) => color.write_foreground(to),
        }
    }
}
