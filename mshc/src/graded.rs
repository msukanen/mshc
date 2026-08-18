//! Grade of things.
//! 
//! What a [Grade] *means* depends on the things' context itself.
//! 

use std::ops::{Add, Sub};
#[cfg(all(feature = "grade-f-to-sss", not(feature = "grade-a2e")))]
pub mod f_to_sss;
#[cfg(all(feature = "grade-f-to-sss", not(feature = "grade-a2e")))]
pub use f_to_sss as grade;
#[cfg(feature = "grade-a2e")]
pub mod a2e;
#[cfg(feature = "grade-a2e")]
pub use a2e as grade;

macro_rules! impl_grade_stuff {
    ([$($t:tt),+]) => { paste::paste! {
        $(
            impl From<grade::Grade> for [<u $t>] {
                #[inline]
                fn from(grade: grade::Grade) -> Self {
                    u8::from(grade) as [<u $t>]
                }
            }
        )+
    }};
}
// baseline u8/i8 have been defined separately
impl_grade_stuff!([16,32,64,128,size]);

pub trait Graded {
    fn grade(&self) -> super::grade::Grade;
}

pub trait GradeMut {
    fn set_grade(&mut self, grade: super::grade::Grade);
}

impl Add<i32> for grade::Grade {
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

impl Sub<i32> for grade::Grade {
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
