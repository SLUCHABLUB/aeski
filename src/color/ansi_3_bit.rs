use crate::color::Color;
use enum_iterator::Sequence;
use image::Rgb;
use std::io::Write;

pub(super) const BACKGROUND: u8 = 40;
pub(super) const FOREGROUND: u8 = 30;

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug, Default, Sequence)]
pub enum Ansi3Bit {
    #[default]
    Black,
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
    White,
}

impl Color for Ansi3Bit {
    fn to_rgb(&self) -> Rgb<f64> {
        Rgb(match self {
            Ansi3Bit::Black => [0.0; 3],
            Ansi3Bit::Red => [1.0, 0.0, 0.0],
            Ansi3Bit::Green => [0.0, 1.0, 0.0],
            Ansi3Bit::Yellow => [1.0, 1.0, 0.0],
            Ansi3Bit::Blue => [0.0, 0.0, 1.0],
            Ansi3Bit::Magenta => [1.0, 0.0, 1.0],
            Ansi3Bit::Cyan => [0.0, 1.0, 1.0],
            Ansi3Bit::White => [1.0; 3],
        })
    }

    fn write_background(&self, mut to: impl Write) -> std::io::Result<()> {
        to.write_all(&[BACKGROUND + *self as u8])
    }

    fn write_foreground(&self, mut to: impl Write) -> std::io::Result<()> {
        to.write_all(&[FOREGROUND + *self as u8])
    }
}
