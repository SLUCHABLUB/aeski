use crate::color::ansi_4_bit::Ansi4Bit;
use crate::color::ansi_8_bit::cube_coordinate::CubeCoordinate;
use crate::color::ansi_8_bit::{BACKGROUND, FOREGROUND, SECOND_ARGUMENT};
use crate::color::Color;
use enum_iterator::Sequence;
use image::Rgb;
use std::io::Write;

const OFFSET: u8 = Ansi4Bit::CARDINALITY as u8;

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default, Sequence)]
pub struct Cube {
    r: CubeCoordinate,
    g: CubeCoordinate,
    b: CubeCoordinate,
}

impl Color for Cube {
    fn to_rgb(&self) -> Rgb<f64> {
        Rgb([
            self.r.get() as f64 / 5.0,
            self.g.get() as f64 / 5.0,
            self.b.get() as f64 / 5.0,
        ])
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
}
