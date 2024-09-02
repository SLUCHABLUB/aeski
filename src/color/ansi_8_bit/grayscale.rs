use crate::color::ansi_4_bit::Ansi4Bit;
use crate::color::ansi_8_bit::cube::Cube;
use crate::color::ansi_8_bit::{BACKGROUND, FOREGROUND, SECOND_ARGUMENT};
use crate::color::Color;
use enum_iterator::Sequence;
use image::{Luma, Pixel, Rgb};
use std::io::Write;

const OFFSET: u8 = (Ansi4Bit::CARDINALITY + Cube::CARDINALITY) as u8;

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug, Default)]
pub struct Grayscale {
    /// On the interval [0; 23].
    brightness: u8,
}

impl Grayscale {
    /// Constructs a new `Grayscale` by clamping `brightness` to [0; 23].
    pub fn new(brightness: u8) -> Grayscale {
        Grayscale {
            brightness: brightness.clamp(0, 23),
        }
    }

    /// Returns `None` if the `brightness` isn't in [0; 23].
    pub const fn try_new(brightness: u8) -> Option<Grayscale> {
        if brightness <= 23 {
            Some(Grayscale { brightness })
        } else {
            None
        }
    }

    /// Gets the brightness on the interval [0; 23].
    pub const fn brightness(&self) -> u8 {
        self.brightness
    }
}

impl Sequence for Grayscale {
    const CARDINALITY: usize = 24;

    fn next(&self) -> Option<Self> {
        Self::try_new(self.brightness + 1)
    }

    fn previous(&self) -> Option<Self> {
        Self::try_new(self.brightness.checked_sub(1)?)
    }

    fn first() -> Option<Self> {
        Self::try_new(0)
    }

    fn last() -> Option<Self> {
        Self::try_new(23)
    }
}

impl Color for Grayscale {
    fn to_rgb(&self) -> Rgb<f64> {
        let luma = self.brightness() as f64 / 23.0;
        Luma([luma]).to_rgb()
    }

    fn write_background(&self, mut to: impl Write) -> std::io::Result<()> {
        to.write_all(&[BACKGROUND, SECOND_ARGUMENT, OFFSET + self.brightness])
    }

    fn write_foreground(&self, mut to: impl Write) -> std::io::Result<()> {
        to.write_all(&[FOREGROUND, SECOND_ARGUMENT, OFFSET + self.brightness])
    }
}
