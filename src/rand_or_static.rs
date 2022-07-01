use rand::Rng;
use std::ops::Range;

/// A random value or a static value
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum RandOrStatic {
    /// A random value in the given range, [start, end).
    Rand { start: usize, end: usize },

    /// A static value, as given.
    Static { value: usize },
}

impl RandOrStatic {
    /// Generate a num if rng using the given rng,
    /// or return the static value.
    pub(crate) fn generate_num<R>(self, mut rng: R) -> usize
    where
        R: Rng,
    {
        match self {
            Self::Rand { start, end } => rng.gen_range(start..end),
            Self::Static { value } => value,
        }
    }
}

impl From<Range<usize>> for RandOrStatic {
    fn from(range: Range<usize>) -> Self {
        RandOrStatic::Rand {
            start: range.start,
            end: range.end,
        }
    }
}

impl From<(usize, usize)> for RandOrStatic {
    fn from((start, end): (usize, usize)) -> Self {
        RandOrStatic::Rand { start, end }
    }
}

impl From<usize> for RandOrStatic {
    fn from(value: usize) -> Self {
        RandOrStatic::Static { value }
    }
}
