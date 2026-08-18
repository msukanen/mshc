use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, Copy, Deserialize, Serialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Grade {
    A,
    B,
    C,
    D,
    E,
}

impl Default for Grade {
    #[inline(always)]
    fn default() -> Self {
        Self::C
    }
}

impl From<Grade> for u8 {
    #[inline]
    fn from(grade: Grade) -> Self {
        use Grade::*;
        match grade {
            E => 5,
            D => 4,
            C => 3,
            B => 2,
            A => 1,
        }
    }
}

impl From<Grade> for i8 {
    #[inline]
    fn from(grade: Grade) -> Self {
        // [Grade] will safely fit in `i8`.
        u8::from(grade) as i8
    }
}

impl Grade {
    #[inline]
    pub const fn next(&self) -> Self {
        use Grade::*;
        match self {
            A => B,
            B => C,
            C => D,
            _ => E
        }
    }

    #[inline]
    pub const fn prev(&self) -> Self {
        use Grade::*;
        match self {
            E => D,
            D => C,
            C => B,
            _ => A
        }
    }

    #[inline(always)] pub const fn floor() -> Self { Self::A }
    #[inline(always)] pub const fn ceil() -> Self { Self::E }
}
