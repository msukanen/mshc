use std::ops::{Add, Sub};

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

macro_rules! impl_grade_stuff {
    ([$($t:tt),+]) => { paste::paste! {
        $(
            impl From<GradeA2E> for [<u $t>] {
                #[inline]
                fn from(grade: GradeA2E) -> Self {
                    u8::from(grade) as [<u $t>]
                }
            }
        )+
    }};
}
// baseline u8/i8 have been defined separately
impl_grade_stuff!([16,32,64,128,size]);

pub trait Graded {
    fn grade(&self) -> GradeA2E;
}

pub trait GradeMut {
    fn set_grade(&mut self, grade: GradeA2E);
}

impl Add<i32> for Grade {
    type Output = Self;
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
    type Output = Self;
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
