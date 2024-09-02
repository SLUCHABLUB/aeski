use crate::cell::AsciiCell;
use crate::color::Color;
use crate::font::Font;
use crate::sgr::SelectGraphicRendition;
use image::imageops::Nearest;
use image::{DynamicImage, GenericImageView, Pixel};
use itertools::iproduct;
use std::fmt::{Display, Formatter};

#[derive(Clone, Eq, PartialEq, Hash, Debug, Default)]
pub struct AsciiImage<Color> {
    width: usize,
    cells: Vec<AsciiCell<Color>>,
}

impl<C> AsciiImage<C> {
    pub fn new() -> usize {
        Default::default()
    }

    pub fn from_cells<Cells>(
        cells: Cells,
        width: usize,
        height: usize,
    ) -> Result<Self, Vec<AsciiCell<C>>>
    where
        Cells: IntoIterator<Item = AsciiCell<C>>,
    {
        let cells: Vec<_> = cells.into_iter().collect();

        if cells.len() == width * height {
            Ok(AsciiImage { width, cells })
        } else {
            Err(cells)
        }
    }

    /// The width of the image in characters
    pub const fn width(&self) -> usize {
        self.width
    }

    /// The height of the image in characters
    pub fn height(&self) -> usize {
        self.cells.len() / self.width
    }

    /// All the cells that make up the image.
    /// Going from right to left, wrapping from top to bottom.
    pub fn cells(&self) -> &[AsciiCell<C>] {
        &self.cells
    }

    /// All the cells that make up the image.
    /// Going from right to left, wrapping from top to bottom.
    pub fn cells_mut(&mut self) -> &mut [AsciiCell<C>] {
        &mut self.cells
    }
}

impl<C: Color> AsciiImage<C> {
    // TODO: optimize
    pub fn from_image<G: AsRef<[char]>>(
        image: DynamicImage,
        width: usize,
        height: usize,
        font: &Font<G>,
    ) -> Self {
        let image = image.resize_exact(width as _, height as _, Nearest);

        let cells = iproduct!(0..height, 0..width)
            .map(|(y, x)| {
                let pixel = image.get_pixel(x as u32, y as u32).to_rgb();

                C::new_cell(pixel, font)
            })
            .collect();

        AsciiImage { width, cells }
    }

    // TODO: use `Formatter` and make pub
    pub(crate) fn fmt_line(&self, f: &mut String, y: usize) -> std::fmt::Result {
        if self.height() <= y {
            return Ok(());
        }

        let start = self.width * y;
        let end = start + self.width;

        let mut previous = None;

        for cell in &self.cells[start..end] {
            cell.fmt_with_previous(f, previous)?;

            let previous = previous.get_or_insert(*cell);
            previous.background = cell.background;
            previous.foreground = cell.foreground.or(previous.foreground);
        }

        // Reset the graphic rendition at the end of the line.
        SelectGraphicRendition::new(f).write_zero();

        Ok(())
    }

    pub fn line(&self, y: usize) -> Option<String> {
        if self.height() <= y {
            return None;
        }

        let mut line = String::new();
        // Only fails if writing fails, which it won't (unless we're out of memory)
        self.fmt_line(&mut line, y).unwrap();

        Some(line)
    }

    pub fn lines(&self) -> Vec<String> {
        (0..self.height()).map(|y| self.line(y).unwrap()).collect()
    }
}

impl<C: Color> Display for AsciiImage<C> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.lines().join("\n"))
    }
}
