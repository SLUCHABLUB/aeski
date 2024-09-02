use crate::cell::{AsciiCell, Foreground};
use crate::color::{Color, Sequence};
use crate::font::Font;
use image::{Luma, Pixel, Rgb};
use std::io::Write;

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default, Sequence)]
pub struct Colorless;

impl Color for Colorless {
    fn to_rgb(&self) -> Rgb<f64> {
        Rgb([0.0; 3])
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

    fn new_cell<G: AsRef<[char]>>(color: Rgb<u8>, font: &Font<G>) -> AsciiCell<Self> {
        let Luma([luma]) = color.to_luma();
        let luma = luma as f64 / 255.0;

        let non_quantized_index = (luma / font.max_coverage()).clamp(0.0, 1.0);

        let index = (non_quantized_index * (font.gradient().len() - 1) as f64).round() as usize;

        AsciiCell {
            background: Colorless,
            foreground: Some(Foreground {
                color: Colorless,
                character: font.gradient()[index],
            }),
        }
    }
}
