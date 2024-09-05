use crate::cell::AsciiCell;
use crate::color::Color;
use crate::font::Font;
use image::{DynamicImage, Pixel, Rgb, SubImage};
use std::io::Write;
use crate::color::util::average_color;

const BACKGROUND: u8 = 48;
const FOREGROUND: u8 = 38;
const SECOND_ARGUMENT: u8 = 2;

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub struct Ansi24Bit {
    r: u8,
    g: u8,
    b: u8,
}

impl Ansi24Bit {
    #[must_use]
    pub const fn new(r: u8, g: u8, b: u8) -> Ansi24Bit {
        Ansi24Bit { r, g, b }
    }
}

impl From<Rgb<u8>> for Ansi24Bit {
    fn from(Rgb([r, g, b]): Rgb<u8>) -> Self {
        Ansi24Bit::new(r, g, b)
    }
}

impl From<Ansi24Bit> for Rgb<u8> {
    fn from(color: Ansi24Bit) -> Self {
        Rgb([color.r, color.g, color.b])
    }
}

impl Color for Ansi24Bit {
    fn to_rgb(&self) -> Rgb<u8> {
        Rgb::from(*self)
    }
    fn from_rgb(color: Rgb<u8>) -> Self {
        color.into()
    }

    fn write_background(&self, mut to: impl Write) -> std::io::Result<()> {
        to.write_all(&[BACKGROUND, SECOND_ARGUMENT, self.r, self.g, self.b])
    }

    fn write_foreground(&self, mut to: impl Write) -> std::io::Result<()> {
        to.write_all(&[FOREGROUND, SECOND_ARGUMENT, self.r, self.g, self.b])
    }

    // TODO: Base the character on the alpha.
    fn new_cell<G: AsRef<[char]>>(view: SubImage<&DynamicImage>, _font: &Font<G>) -> AsciiCell<Self> {
        let color = average_color(*view).to_rgb();
        AsciiCell {
            background: color.into(),
            foreground: None,
        }
    }
}
