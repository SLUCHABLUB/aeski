pub mod ansi_24_bit;
pub mod ansi_3_bit;
pub mod ansi_4_bit;
pub mod ansi_8_bit;
pub mod colorless;
mod util;

use crate::cell::{AsciiCell, Foreground};
use enum_iterator::all;
use image::Rgb;
use std::fmt::Debug;
use std::io::Write;

use crate::color::util::{float, interpolate, square_distance};
use crate::font::Font;
pub use enum_iterator::Sequence;
use itertools::iproduct;

// TODO: compact colors
pub trait Color: Copy + Debug + Default + PartialEq + Sequence {
    #[must_use]
    fn to_rgb(&self) -> Rgb<f64>;
    /// Approximate `color` using `Self`.
    #[must_use]
    fn from_rgb(color: Rgb<u8>) -> Self {
        default_from_rgb(color)
    }

    /// Writes the ansi `SGR` parameters to color the background.
    /// # Errors
    /// If writing fails.
    fn write_background(&self, to: impl Write) -> std::io::Result<()>;
    /// Writes the ansi `SGR` parameters to color the character.
    /// # Errors
    //  If writing fails.
    fn write_foreground(&self, to: impl Write) -> std::io::Result<()>;

    // TODO: take alpha channel into consideration
    // TODO: replace `color: Rgb<u8>` with `pixels: SubImage`
    /// Creates a new `AsciiCell` with `Self` as the color type.
    /// `color` represents the color to approximate.
    /// `max_coverage` is how much coverage the last character in `gradient` provides.
    #[must_use]
    fn new_cell<G: AsRef<[char]>>(color: Rgb<u8>, font: &Font<G>) -> AsciiCell<Self> {
        let target = float(color);

        // Go through all possible color-color-character combinations and find the closest
        let mut options: Vec<_> = iproduct!(all::<Self>(), all::<Self>())
            .flat_map(|(from, to)| {
                let from_rgb = from.to_rgb();
                let to_rgb = to.to_rgb();

                font.gradient()
                    .iter()
                    .enumerate()
                    .map(move |(index, char)| {
                        let interpolation_parameter = font.coverage(index);

                        let interpolation = interpolate(from_rgb, to_rgb, interpolation_parameter);

                        let distance = square_distance(target, interpolation);

                        (from, to, distance, *char)
                    })
            })
            .collect();
        options
            .sort_by(|(_, _, distance0, _), (_, _, distance1, _)| distance0.total_cmp(distance1));

        if options.is_empty() {
            return AsciiCell::default();
        }

        let (_, _, closest, _) = options[0];
        let options: Vec<_> = options
            .into_iter()
            .take_while(|(_, _, distance, _)| *distance == closest)
            .collect();

        // TODO: make a better choice from `options` (random?)
        let (background, color, _, character) = options[0];

        AsciiCell {
            background,
            foreground: Some(Foreground { color, character }),
        }
    }
}

pub(super) fn default_from_rgb<C: Color>(color: Rgb<u8>) -> C {
    all::<C>()
        .min_by(|self0, self1| {
            let color_f64 = float(color);
            let dist0 = square_distance(self0.to_rgb(), color_f64);
            let dist1 = square_distance(self1.to_rgb(), color_f64);
            dist0.total_cmp(&dist1)
        })
        .unwrap_or_default()
}
