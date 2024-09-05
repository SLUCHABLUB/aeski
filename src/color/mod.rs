pub mod ansi_24_bit;
pub mod ansi_3_bit;
pub mod ansi_4_bit;
pub mod ansi_8_bit;
pub mod colorless;
mod util;
mod variants;

use crate::cell::{AsciiCell, Foreground};
use image::{DynamicImage, Rgb, SubImage};
use std::io::Write;

use crate::color::util::{interpolate, square_distance};
use crate::font::Font;

// TODO: Add compact colors.
pub trait Color: Copy {
    #[must_use]
    fn to_rgb(&self) -> Rgb<u8>;
    /// Approximate `color` using `Self`.
    #[must_use]
    fn from_rgb(color: Rgb<u8>) -> Self;

    /// Writes the ansi `SGR` parameters to color the background.
    /// # Errors
    /// If writing fails.
    fn write_background(&self, to: impl Write) -> std::io::Result<()>;
    /// Writes the ansi `SGR` parameters to color the character.
    /// # Errors
    //  If writing fails.
    fn write_foreground(&self, to: impl Write) -> std::io::Result<()>;

    // TODO: Take alpha channel into consideration.
    // TODO: Replace `color: Rgb<u8>` with `pixels: SubImage`.
    /// Creates a new `AsciiCell` with `Self` as the color type.
    /// `color` represents the color to approximate.
    /// `max_coverage` is how much coverage the last character in `gradient` provides.
    #[must_use]
    fn new_cell<G: AsRef<[char]>>(view: SubImage<&DynamicImage>, font: &Font<G>) -> AsciiCell<Self>
    where
        Self: Sized;
}

pub(super) fn default_new_cell<C: Color + Default, G: AsRef<[char]>>(
    colors: &[C],
    color: Rgb<u8>,
    font: &Font<G>,
) -> AsciiCell<C> {
    let mut closest = u32::MAX;
    let mut background = C::default();
    let mut foreground = C::default();
    let mut character = ' ';

    // Cache coverages
    let mut coverages = Vec::with_capacity(font.gradient().len());
    for i in 0..font.gradient().len() {
        coverages.push(font.coverage(i))
    }
    // Using a slice improves performance
    let coverages = coverages.as_slice();

    // Go through all possible color-color-character combinations and find the closest
    for from in colors {
        let from_rgb = from.to_rgb();

        for to in colors {
            let to_rgb = to.to_rgb();

            for (index, char) in font.gradient().iter().enumerate() {
                let interpolation_parameter = coverages[index];

                let interpolation = interpolate(from_rgb, to_rgb, interpolation_parameter);

                let distance = square_distance(color, interpolation);

                if distance < closest {
                    closest = distance;
                    background = *from;
                    foreground = *to;
                    character = *char;
                }
            }
        }
    }

    let foreground = (character != ' ').then_some(Foreground {
        color: foreground,
        character,
    });

    AsciiCell {
        background,
        foreground,
    }
}

pub(super) fn default_from_rgb<C: Color + Default>(colors: &[C], color: Rgb<u8>) -> C {
    colors
        .iter()
        .copied()
        .min_by_key(|c| square_distance(c.to_rgb(), color))
        .unwrap_or_default()
}
