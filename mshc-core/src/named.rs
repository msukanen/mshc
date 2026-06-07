//! [Named] trait for things with a name.

pub trait Named {
    fn name<'a>(&'a self) -> &'a str;
}
