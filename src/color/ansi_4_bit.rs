use crate::color::ansi_3_bit::{Ansi3Bit, BACKGROUND, FOREGROUND};
use crate::color::{Color, Sequence};
use image::{Pixel, Rgb};
use std::io::Write;

const BRIGHT_OFFSET: u8 = 60;

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug, Sequence, Default)]
pub struct Ansi4Bit {
    pub is_bright: bool,
    pub color: Ansi3Bit,
}

impl Color for Ansi4Bit {
    fn to_rgb(&self) -> Rgb<f64> {
        const THIRD: f64 = 1.0 / 3.0;
        const TWO_THIRDS: f64 = 2.0 / 3.0;

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
            } => color.to_rgb().map(|chanel| chanel / 2.0),
        }
    }

    fn write_background(&self, mut to: impl Write) -> std::io::Result<()> {
        to.write_all(&[self.color as u8 + BACKGROUND + u8::from(self.is_bright) * BRIGHT_OFFSET])
    }

    fn write_foreground(&self, mut to: impl Write) -> std::io::Result<()> {
        to.write_all(&[self.color as u8 + FOREGROUND + u8::from(self.is_bright) * BRIGHT_OFFSET])
    }
}
