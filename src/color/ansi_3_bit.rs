use crate::cell::AsciiCell;
use crate::color::variants::ANSI_3_BIT;
use crate::color::{default_from_rgb, default_new_cell, Color};
use crate::font::Font;
use image::{DynamicImage, Pixel, Rgb, SubImage};
use std::io::Write;
use crate::color::util::average_color;

pub(super) const BACKGROUND: u8 = 40;
pub(super) const FOREGROUND: u8 = 30;

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug, Default)]
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
    fn to_rgb(&self) -> Rgb<u8> {
        const FULL: u8 = 255;

        Rgb(match self {
            Ansi3Bit::Black => [0; 3],
            Ansi3Bit::Red => [FULL, 0, 0],
            Ansi3Bit::Green => [0, FULL, 0],
            Ansi3Bit::Yellow => [FULL, FULL, 0],
            Ansi3Bit::Blue => [0, 0, FULL],
            Ansi3Bit::Magenta => [FULL, 0, FULL],
            Ansi3Bit::Cyan => [0, FULL, FULL],
            Ansi3Bit::White => [FULL; 3],
        })
    }

    fn from_rgb(color: Rgb<u8>) -> Self {
        default_from_rgb(&ANSI_3_BIT, color)
    }

    fn write_background(&self, mut to: impl Write) -> std::io::Result<()> {
        to.write_all(&[BACKGROUND + *self as u8])
    }

    fn write_foreground(&self, mut to: impl Write) -> std::io::Result<()> {
        to.write_all(&[FOREGROUND + *self as u8])
    }

    fn new_cell<G: AsRef<[char]>>(view: SubImage<&DynamicImage>, font: &Font<G>) -> AsciiCell<Self> {
        default_new_cell(&ANSI_3_BIT, average_color(*view).to_rgb(), font)
    }
}
