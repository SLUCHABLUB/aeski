// TODO: add aspect ratio
#[derive(Copy, Clone, PartialEq, Debug)]
pub struct Font<G> {
    /// A gradient of characters from least to most coverage.
    gradient: G,
    /// A number on the interval (0; 1] which represents the coverage of last char in `gradient`.
    /// 0 represents no coverage (a non-rendered character like space).
    /// 1 represents full coverage.
    max_coverage: f64,
    /// The font width divided by the font height.
    aspect_ratio: f64,
}

impl<G: AsRef<[char]>> Font<G> {
    /// Tries to construct a new `Font` object.
    ///
    /// # Errors
    /// If the gradient is empty
    /// `max_coverage` is not on the interval [0; 1],
    /// or `aspect_ratio` is <= 0
    /// `Err(gradient)` is returned.
    pub fn new(gradient: G, max_coverage: f64, aspect_ratio: f64) -> Result<Font<G>, G> {
        if gradient.as_ref().is_empty()
            || !(0.0 < max_coverage && max_coverage <= 1.0)
            || aspect_ratio <= 0.0
        {
            return Err(gradient);
        }

        Ok(Font {
            gradient,
            max_coverage,
            aspect_ratio,
        })
    }

    /// See field documentation.
    pub fn gradient(&self) -> &[char] {
        self.gradient.as_ref()
    }

    /// See field documentation.
    pub fn max_coverage(&self) -> f64 {
        self.max_coverage
    }

    /// See field documentation.
    pub fn aspect_ratio(&self) -> f64 {
        self.aspect_ratio
    }

    /// Gets the coverage of the char at the specified index.
    /// If the index is out of bounds, the maximum coverage is returned.
    pub fn coverage(&self, index: usize) -> f64 {
        if index >= self.gradient().len() {
            return self.max_coverage;
        }

        if self.gradient().len() == 1 {
            return 0.0;
        }

        // [0; 1]
        let t = index as f64 / (self.gradient().len() - 1) as f64;

        t * self.max_coverage
    }
}
