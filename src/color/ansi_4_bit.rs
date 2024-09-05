use crate::cell::AsciiCell;
use crate::color::ansi_3_bit::{Ansi3Bit, BACKGROUND, FOREGROUND};
use crate::color::variants::ANSI_4_BIT;
use crate::color::{default_from_rgb, default_new_cell, Color};
use crate::font::Font;
use image::{DynamicImage, Pixel, Rgb, SubImage};
use std::io::Write;
use crate::color::util::average_color;

const BRIGHT_OFFSET: u8 = 60;

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug, Default)]
pub struct Ansi4Bit {
    pub is_bright: bool,
    pub color: Ansi3Bit,
}

impl Ansi4Bit {
    pub const fn new_non_bright(color: Ansi3Bit) -> Self {
        Ansi4Bit {
            is_bright: false,
            color,
        }
    }

    pub const fn new_bright(color: Ansi3Bit) -> Self {
        Ansi4Bit {
            is_bright: true,
            color,
        }
    }
}

impl Color for Ansi4Bit {
    fn to_rgb(&self) -> Rgb<u8> {
        const FULL: u8 = 255;
        const THIRD: u8 = FULL / 3;
        const TWO_THIRDS: u8 = 2 * THIRD;

        match self {
            Ansi4Bit {
                is_bright: true,
                color: Ansi3Bit::Black,
            } => Rgb([THIRD; 3]),
            Ansi4Bit {
                is_bright: false,
                color: Ansi3Bit::White,
            } => Rgb([TWO_THIRDS; 3]),
            Ansi4Bit {
                is_bright: true,
                color,
            } => color.to_rgb(),
            Ansi4Bit {
                is_bright: false,
                color,
            } => color.to_rgb().map(|chanel| chanel / 2),
        }
    }

    fn from_rgb(color: Rgb<u8>) -> Self {
        default_from_rgb(&ANSI_4_BIT, color)
    }

    fn write_background(&self, mut to: impl Write) -> std::io::Result<()> {
        to.write_all(&[self.color as u8 + BACKGROUND + u8::from(self.is_bright) * BRIGHT_OFFSET])
    }

    fn write_foreground(&self, mut to: impl Write) -> std::io::Result<()> {
        to.write_all(&[self.color as u8 + FOREGROUND + u8::from(self.is_bright) * BRIGHT_OFFSET])
    }

    fn new_cell<G: AsRef<[char]>>(view: SubImage<&DynamicImage>, font: &Font<G>) -> AsciiCell<Self> {
        default_new_cell(&ANSI_4_BIT, average_color(*view).to_rgb(), font)
    }
}
