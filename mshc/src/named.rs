//! Traits for things with a name.
//! 
//! # [Named]
//! 
//! A trait for querying name of something.
//! 
//! # [NamedMut]
//! 
//! A trait for mutating something's name.
//! 
/// Name query trait.
pub trait Named {
    /// Get the thing's name.
    fn name<'a>(&'a self) -> &'a str;
}

/// Name mutation trait.
pub trait NamedMut {
    /// Set the name.
    fn set_name(&mut self, name: &str) -> &mut Self;
}
