use num_rational::Ratio;
use num_traits::float::FloatCore;
use num_traits::{NumCast, One, Zero};

#[derive(Copy, Clone, PartialEq, Debug)]
pub struct Font<G> {
    /// A gradient of characters from least to most coverage.
    gradient: G,
    /// A number on the interval (0; 1] which represents the coverage of last char in `gradient`.
    /// 0 represents no coverage (a non-rendered character like space).
    /// 1 represents full coverage.
    max_coverage: Ratio<usize>,
    /// The font width divided by the font height.
    aspect_ratio: Ratio<usize>,
}

impl<G: AsRef<[char]>> Font<G> {
    /// Tries to construct a new `Font` object.
    ///
    /// # Errors
    /// If the gradient is empty
    /// `max_coverage` is not on the interval [0; 1],
    /// or `aspect_ratio` is 0
    /// `Err(gradient)` is returned.
    pub fn new(
        gradient: G,
        max_coverage: Ratio<usize>,
        aspect_ratio: Ratio<usize>,
    ) -> Result<Font<G>, G> {
        if gradient.as_ref().is_empty() || Ratio::one() < max_coverage || aspect_ratio.is_zero() {
            return Err(gradient);
        }

        Ok(Font {
            gradient,
            max_coverage,
            aspect_ratio,
        })
    }

    pub fn new_float<F: FloatCore + NumCast>(
        gradient: G,
        max_coverage: F,
        aspect_ratio: F,
    ) -> Result<Font<G>, G> {
        let (Some(max_coverage), Some(aspect_ratio)) = (
            Ratio::approximate_float_unsigned(max_coverage),
            Ratio::approximate_float_unsigned(aspect_ratio),
        ) else {
            return Err(gradient);
        };

        Self::new(gradient, max_coverage, aspect_ratio)
    }

    /// See field documentation.
    pub fn gradient(&self) -> &[char] {
        self.gradient.as_ref()
    }

    /// See field documentation.
    pub fn max_coverage(&self) -> Ratio<usize> {
        self.max_coverage
    }

    /// See field documentation.
    pub fn aspect_ratio(&self) -> Ratio<usize> {
        self.aspect_ratio
    }

    /// Gets the coverage of the char at the specified index.
    /// If the index is out of bounds, the maximum coverage is returned.
    pub fn coverage(&self, index: usize) -> Ratio<usize> {
        if index >= self.gradient().len() {
            return self.max_coverage;
        }

        if self.gradient().len() == 1 {
            return 0.into();
        }

        // [0; 1]
        let t = Ratio::new(index, self.gradient().len() - 1);

        t * self.max_coverage
    }
}
