use crate::cell::AsciiCell;
use crate::color::ansi_8_bit::{BACKGROUND, FOREGROUND, SECOND_ARGUMENT};
use crate::color::variants::GRAYSCALE;
use crate::color::{default_new_cell, Color};
use crate::font::Font;
use image::{DynamicImage, Luma, Pixel, Rgb, SubImage};
use num_rational::Ratio;
use std::io::Write;
use crate::color::util::average_color;

const OFFSET: u8 = 232;

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug, Default)]
pub struct Grayscale {
    /// On the interval [0; 23].
    brightness: u8,
}

impl Grayscale {
    /// Constructs a new `Grayscale` by clamping `brightness` to [0; 23].
    #[must_use]
    pub const fn new(mut brightness: u8) -> Grayscale {
        if 23 < brightness {
            brightness = 23;
        }

        Grayscale { brightness }
    }

    /// Returns `None` if the `brightness` isn't in [0; 23].
    #[must_use]
    pub const fn try_new(brightness: u8) -> Option<Grayscale> {
        if brightness <= 23 {
            Some(Grayscale { brightness })
        } else {
            None
        }
    }

    /// Gets the brightness on the interval [0; 23].
    #[must_use]
    pub const fn brightness(&self) -> u8 {
        self.brightness
    }
}

impl Color for Grayscale {
    fn to_rgb(&self) -> Rgb<u8> {
        Luma([Ratio::new(self.brightness() as usize * 255, 23)
            .round()
            .to_integer()
            .try_into()
            .unwrap_or(u8::MAX)])
        .to_rgb()
    }

    fn from_rgb(color: Rgb<u8>) -> Self {
        let Luma([luma]) = color.to_luma();

        Grayscale::new(
            Ratio::new(luma as usize * 23, 255)
                .round()
                .to_integer()
                .try_into()
                .unwrap_or(23),
        )
    }

    fn write_background(&self, mut to: impl Write) -> std::io::Result<()> {
        to.write_all(&[BACKGROUND, SECOND_ARGUMENT, OFFSET + self.brightness])
    }

    fn write_foreground(&self, mut to: impl Write) -> std::io::Result<()> {
        to.write_all(&[FOREGROUND, SECOND_ARGUMENT, OFFSET + self.brightness])
    }

    fn new_cell<G: AsRef<[char]>>(view: SubImage<&DynamicImage>, font: &Font<G>) -> AsciiCell<Self> {
        default_new_cell(&GRAYSCALE, average_color(*view).to_rgb(), font)
    }
}
