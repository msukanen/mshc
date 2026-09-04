use std::ops::{Add, Sub};

use serde::{Deserialize, Serialize};

/// Grading system, from crap to extra-super-grand…
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Deserialize, Serialize)]
#[repr(u8)]
pub enum Grade {
    F,
    E,
    D,
    C, // CCDung: A
    B, // CCDung: B
    A, // CCDung: C
    S,
    SS,
    SSS,
}

impl Default for Grade {
    #[inline(always)]
    fn default() -> Self {
        Self::C
    }
}

impl Grade {
    /// Get next higher [Grade] (if possible).
    #[inline]
    pub const fn next(&self) -> Self {
        match self {
            Self::F => Self::E,
            Self::E => Self::D,
            Self::D => Self::C,
            Self::C => Self::B,
            Self::B => Self::A,
            Self::A => Self::S,
            Self::S => Self::SS,
            Self::SS  |
            Self::SSS => Self::SSS
        }
    }

    /// Get next lower [Grade] (if possible).
    #[inline]
    pub const fn prev(&self) -> Self {
        match self {
            Self::F |
            Self::E => Self::F,
            Self::D => Self::E,
            Self::C => Self::D,
            Self::B => Self::C,
            Self::A => Self::B,
            Self::S => Self::A,
            Self::SS => Self::S,
            Self::SSS => Self::SS
        }
    }

    #[inline(always)]
    const fn idx_v(v: i64) -> usize {
        if v < 0 { 0 }
        else if v >= GRADES.len() as i64 { GRADES.len() - 1 }
        else { v as usize }
    }

    #[inline(always)]
    pub const fn add(self, rhs: i32) -> Self {
        let v = self as i64 + rhs as i64;
        GRADES[Self::idx_v(v)]
    }

    #[inline(always)]
    pub const fn sub(self, rhs: i32) -> Self {
        let v = self as i64 - rhs as i64;
        GRADES[Self::idx_v(v)]
    }

    pub const fn floor() -> Self { Self::F }
    pub const fn ceil() -> Self { Self::SSS }
}

const GRADES: [Grade; 9] = [
    Grade::F, Grade::E, Grade::D,
    Grade::C, Grade::B, Grade::A,
    Grade::S, Grade::SS, Grade::SSS,
];

impl Add<i32> for Grade {
    type Output = Self;
    fn add(self, rhs: i32) -> Self::Output {
        Self::add(self, rhs)
    }
}

impl Sub<i32> for Grade {
    type Output = Self;
    fn sub(self, rhs: i32) -> Self::Output {
        Self::sub(self, rhs)
    }
}

#[cfg(test)]
mod grade_tests {
    use super::*;

    #[test]
    fn grade_sss_plus_x_is_sss() {
        let sss = Grade::SSS;
        let sss_plus = sss + 1;
        assert_eq!(sss, sss_plus);
    }

    #[test]
    fn grade_f_minus_x_is_f() {
        let f = Grade::F;
        let f_minus = f - 1;
        assert_eq!(f, f_minus);
    }

    #[test]
    fn grade_slide_works() {
        let mut grade = Grade::F;
        for x in 0..GRADES.len() {
            assert_eq!(grade, GRADES[x]);
            grade = grade + 1;
        }
    }

    #[test]
    fn grade_next_prev_agree_with_slide() {
        for x in 0..GRADES.len() - 1 {
            assert_eq!(GRADES[x].next(), GRADES[x + 1]);
            assert_eq!(GRADES[x + 1].prev(), GRADES[x]);
        }
    }
}
