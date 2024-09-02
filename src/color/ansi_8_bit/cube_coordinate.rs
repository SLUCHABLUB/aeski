use enum_iterator::Sequence;
use std::fmt::{Debug, Display, Formatter};

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Default)]
#[repr(transparent)]
pub struct CubeCoordinate {
    /// On the interval [0; 5].
    coordinate: u8,
}

impl CubeCoordinate {
    /// Clamps the argument.
    pub fn new(value: u8) -> CubeCoordinate {
        CubeCoordinate {
            coordinate: value.clamp(0, 5),
        }
    }

    /// Returns `None` if the argument is too large.
    pub const fn try_new(value: u8) -> Option<CubeCoordinate> {
        if value <= 5 {
            Some(CubeCoordinate { coordinate: value })
        } else {
            None
        }
    }

    /// Converts `self` to `u8`.
    pub const fn get(&self) -> u8 {
        self.coordinate
    }
}

impl Debug for CubeCoordinate {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.coordinate)
    }
}

impl Display for CubeCoordinate {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.coordinate)
    }
}

impl Sequence for CubeCoordinate {
    const CARDINALITY: usize = 6;

    fn next(&self) -> Option<Self> {
        Self::try_new(self.coordinate + 1)
    }

    fn previous(&self) -> Option<Self> {
        Self::try_new(self.coordinate.checked_sub(1)?)
    }

    fn first() -> Option<Self> {
        Some(Self::new(0))
    }

    fn last() -> Option<Self> {
        Some(Self::new(5))
    }
}
