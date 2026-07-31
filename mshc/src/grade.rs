//! Grade of things.
//! 
//! What a [Grade] *means* depends on the things' context itself.
//! 
use std::ops::{Add, Sub};

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
}

impl From<Grade> for u32 {
    #[inline]
    fn from(grade: Grade) -> Self {
        u8::from(grade) as u32
    }
}

pub trait Graded {
    fn grade(&self) -> Grade;
}

pub trait GradeMut {
    fn set_grade(&mut self, grade: Grade);
}

impl Add<i32> for Grade {
    type Output = Grade;
    fn add(self, rhs: i32) -> Self::Output {
        let rev = rhs < 0;
        let mut res = self;
        for _ in 0..rhs.abs() {
            if rev { res = res.prev() }
            else   { res = res.next() }
        }
        res
    }
}

impl Sub<i32> for Grade {
    type Output = Grade;
    fn sub(self, rhs: i32) -> Self::Output {
        let rev = rhs < 0;
        let mut res = self;
        for _ in 0..rhs.abs() {
            if rev { res = res.next() }
            else   { res = res.prev() }
        }
        res
    }
}
