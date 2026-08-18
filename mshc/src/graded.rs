//! Grade of things.
//! 
//! What a [Grade] *means* depends on the things' context itself.
//! 
#[cfg(all(feature = "grade-a2e", not(feature = "grade-f-to-sss")))]
pub mod a2e;
#[cfg(all(not(feature = "grade-a2e"), feature = "grade-f-to-sss"))]
pub mod f_to_sss;
