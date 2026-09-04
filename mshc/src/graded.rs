//! Grade of things.
//! 
//! What a [Grade] *means* depends on the things' context itself.
//! 

#[cfg(all(feature = "grade-f-to-sss", not(feature = "grade-a2e")))]
pub mod f_to_sss;
#[cfg(all(feature = "grade-f-to-sss", not(feature = "grade-a2e")))]
pub use f_to_sss as grade;
#[cfg(feature = "grade-a2e")]
pub mod a2e;
#[cfg(feature = "grade-a2e")]
pub use a2e as grade;

pub trait Graded {
    fn grade(&self) -> super::grade::Grade;
}

pub trait GradeMut {
    fn set_grade(&mut self, grade: super::grade::Grade);
}
