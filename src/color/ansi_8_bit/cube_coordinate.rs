use std::fmt::{Debug, Display, Formatter};

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Default)]
#[repr(transparent)]
pub struct CubeCoordinate {
    /// On the interval [0; 5].
    coordinate: u8,
}

impl CubeCoordinate {
    /// Clamps the argument.
    #[must_use]
    pub const fn new(mut coordinate: u8) -> CubeCoordinate {
        if 5 < coordinate {
            coordinate = 5;
        }

        CubeCoordinate { coordinate }
    }

    /// Returns `None` if the argument is too large.
    #[must_use]
    pub const fn try_new(value: u8) -> Option<CubeCoordinate> {
        if value <= 5 {
            Some(CubeCoordinate { coordinate: value })
        } else {
            None
        }
    }

    /// Converts `self` to `u8`.
    #[must_use]
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
