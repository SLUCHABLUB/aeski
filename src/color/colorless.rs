use crate::cell::{AsciiCell, Foreground};
use crate::color::Color;
use crate::font::Font;
use image::{DynamicImage, Luma, Pixel, Rgb, SubImage};
use num_rational::Ratio;
use std::io::Write;
use crate::color::util::average_color;

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub struct Colorless;

impl Color for Colorless {
    fn to_rgb(&self) -> Rgb<u8> {
        Rgb([0; 3])
    }

    fn from_rgb(_: Rgb<u8>) -> Self {
        Colorless
    }

    fn write_background(&self, _: impl Write) -> std::io::Result<()> {
        Ok(())
    }

    fn write_foreground(&self, _: impl Write) -> std::io::Result<()> {
        Ok(())
    }

    fn new_cell<G: AsRef<[char]>>(view: SubImage<&DynamicImage>, font: &Font<G>) -> AsciiCell<Self> {
        let Luma([luma]) = average_color(*view).to_luma();
        let luma = Ratio::new(luma as usize, u8::MAX as usize);

        let index = (luma * (font.gradient().len() - 1)).round().to_integer();

        AsciiCell {
            background: Colorless,
            foreground: Some(Foreground {
                color: Colorless,
                character: font.gradient()[index],
            }),
        }
    }
}
