use crate::cell::AsciiCell;
use crate::color::Color;
use crate::font::Font;
use crate::sgr::SelectGraphicRendition;
use image::{DynamicImage, GenericImageView};
use num_rational::Ratio;
use rayon::iter::IndexedParallelIterator;
use rayon::iter::IntoParallelIterator;
use rayon::iter::ParallelIterator;
use std::cmp::Ordering;
use std::fmt::{Display, Formatter};

#[derive(Clone, Eq, PartialEq, Hash, Debug)]
pub struct AsciiImage<Color> {
    width: usize,
    cells: Vec<AsciiCell<Color>>,
}

impl<C> AsciiImage<C> {
    #[must_use]
    pub fn new() -> Self {
        AsciiImage::default()
    }

    /// Collects an iterator of cells into an image with the provided dimensions.
    ///
    /// # Errors
    /// If the number of cells isn't `width * height`
    /// the cells are collected into a `Vec` and returned.
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
    #[must_use]
    pub const fn width(&self) -> usize {
        self.width
    }

    /// The height of the image in characters
    #[must_use]
    pub fn height(&self) -> usize {
        self.cells.len() / self.width
    }

    /// All the cells that make up the image.
    /// Going from right to left, wrapping from top to bottom.
    #[must_use]
    pub fn cells(&self) -> &[AsciiCell<C>] {
        &self.cells
    }

    /// All the cells that make up the image.
    /// Going from right to left, wrapping from top to bottom.
    pub fn cells_mut(&mut self) -> &mut [AsciiCell<C>] {
        &mut self.cells
    }
}

// TODO: Add serial versions for non Send colors.
impl<C: Color + Send> AsciiImage<C> {
    /// Converts the image to ascii using the image's dimensions.
    /// However, the aspect ratio is kept by scaling one of the
    /// dimensions down using the font's aspect ratio.
    pub fn from_image<G: AsRef<[char]> + Send + Sync>(
        image: &DynamicImage,
        font: &Font<G>,
    ) -> Self {
        match font.aspect_ratio().cmp(&Ratio::from(1)) {
            // font height > font width => downsample height
            Ordering::Less => Self::from_image_with_width(image, font, image.width()),
            // font height = font width => don't downsample
            Ordering::Equal => Self::from_image_with_dimensions(image, font, image.width(), image.height()),
            // font height < font width => downsample width
            Ordering::Greater => Self::from_image_with_height(image, font, image.height()),
        }
    }

    pub fn from_image_with_width<G: AsRef<[char]> + Send + Sync>(
        image: &DynamicImage,
        font: &Font<G>,
        width: u32,
    ) -> Self {
        let scaling_factor = Ratio::new(width, image.width());
        let height = (scaling_factor * image.height() * font.aspect_ratio())
            .round()
            .to_integer();
        Self::from_image_with_dimensions(image, font, width, height)
    }

    pub fn from_image_with_height<G: AsRef<[char]> + Send + Sync>(
        image: &DynamicImage,
        font: &Font<G>,
        height: u32,
    ) -> Self {
        let scaling_factor = Ratio::new(height, image.height());
        let width = (scaling_factor * image.width() / font.aspect_ratio())
            .round()
            .to_integer();
        Self::from_image_with_dimensions(image, font, width, height)
    }

    pub fn from_image_with_dimensions<G: AsRef<[char]> + Send + Sync>(
        image: &DynamicImage,
        font: &Font<G>,
        width: u32,
        height: u32,
    ) -> Self {
        let area = width * height;

        if area == 0 {
            return AsciiImage::new()
        }

        let mut cells = Vec::with_capacity(area as usize);

        (0..area)
            .into_par_iter()
            .map(|index| {
                let char_x = index % width;
                let char_y = index / width;

                let x = char_x * image.width() / width;
                let y = char_y * image.height() / height;

                let width = (char_x + 1) * image.width() / width - x;
                let height = (char_y + 1) * image.height() / height - y;

                let view = image.view(x, y, width.max(1), height.max(1));

                C::new_cell(view, font)
            })
            .collect_into_vec(&mut cells);

        let width = width as usize;
        AsciiImage { width, cells }
    }
}

impl<C: Color + PartialEq> AsciiImage<C> {
    // TODO: Use `Formatter` and make public.
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

    #[must_use]
    pub fn line(&self, y: usize) -> Option<String> {
        if self.height() <= y {
            return None;
        }

        let mut line = String::new();
        // Only fails if writing fails, which it won't (unless we're out of memory)
        self.fmt_line(&mut line, y).unwrap();

        Some(line)
    }

    #[must_use]
    pub fn lines(&self) -> Vec<String> {
        (0..self.height()).map(|y| self.line(y).unwrap()).collect()
    }
}

// Manually implemented since deriving would impose `C: Default` (a bug)
impl<C> Default for AsciiImage<C> {
    fn default() -> Self {
        AsciiImage {
            width: 0,
            cells: vec![],
        }
    }
}

impl<C: Color + PartialEq> Display for AsciiImage<C> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.lines().join("\n"))
    }
}
