//! Grade of things.
//! 
//! What a [Grade] *means* depends on the things' context itself.
//! 
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
