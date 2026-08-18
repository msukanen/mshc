use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, Copy, Deserialize, Serialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Grade {
    F,
    E,
    D,
    C,
    B,
    A,
    S,
    SS,
    SSS,
}

impl Default for Grade {
    #[inline(always)]
    fn default() -> Self {
        Self::D
    }
}

impl From<Grade> for u8 {
    #[inline]
    fn from(grade: Grade) -> Self {
        use Grade::*;
        match grade {
            F => 0,
            E => 1,
            D => 2,
            C => 3,
            B => 4,
            A => 5,
            S => 6,
            SS => 7,
            SSS => 8,
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
            F => E,
            E => D,
            D => C,
            C => B,
            B => A,
            A => S,
            S => SS,
            _ => SSS,
        }
    }

    #[inline]
    pub const fn prev(&self) -> Self {
        use Grade::*;
        match self {
            F|E => F,
            D => E,
            C => D,
            B => C,
            A => B,
            S => A,
            SS => S,
            SSS => SS,
        }
    }

    #[inline(always)] pub const fn floor() -> Self { Self::F }
    #[inline(always)] pub const fn ceil() -> Self { Self::SSS }
}
