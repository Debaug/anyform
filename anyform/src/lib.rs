use std::borrow::{Borrow, BorrowMut};

/// A type exposing representing fully owned data, i. e.:
///  - Any shared reference `&Owned<T>` can be transformed into `&T`;
///  - Any mutable reference `&mut Owned<T>` can be transformed into `&mut T`.
/// With [`std`]/[`alloc`], these exact restrictions exist with two types: either `T` itself, or [`Box`]`<T>`.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Owned<T> {
    Plain(T),
    Box(Box<T>),
}

impl<T> AsRef<T> for Owned<T> {
    fn as_ref(&self) -> &T {
        match self {
            Self::Plain(plain) => plain,
            Self::Box(boxed) => boxed,
        }
    }
}

impl<T> AsMut<T> for Owned<T> {
    fn as_mut(&mut self) -> &mut T {
        match self {
            Self::Plain(plain) => plain,
            Self::Box(boxed) => boxed,
        }
    }
}

impl<T> Borrow<T> for Owned<T> {
    fn borrow(&self) -> &T {
        self.as_ref()
    }
}

impl<T> BorrowMut<T> for Owned<T> {
    fn borrow_mut(&mut self) -> &mut T {
        self.as_mut()
    }
}

impl<T> From<T> for Owned<T> {
    fn from(plain: T) -> Self {
        Self::Plain(plain)
    }
}

impl<T> From<Box<T>> for Owned<T> {
    fn from(boxed: Box<T>) -> Self {
        Self::Box(boxed)
    }
}

impl<T> Owned<T> {
    /// Converts `self` into `T`, moving the containing value to the stack if `self` is
    /// `Owned::Box(_)`.
    pub fn into_plain(self) -> T {
        match self {
            Self::Plain(plain) => plain,
            Self::Box(boxed) => *boxed,
        }
    }

    /// Converts `self` into `Box<T>`, moving the containing value to the heap if `self` is
    /// `Owned::Plain(_)`.
    pub fn into_box(self) -> Box<T> {
        match self {
            Self::Plain(plain) => Box::new(plain),
            Self::Box(boxed) => boxed,
        }
    }
}
