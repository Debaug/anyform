use std::{borrow::{Borrow, BorrowMut}, rc::Rc, sync::{Arc, RwLock, Mutex}, cell::RefCell};

/// A type representing fully owned data.
/// 
/// More precisely:
///  - Any shared reference `&Owned<T>` can be transformed into `&T`;
///  - Any mutable reference `&mut Owned<T>` can be transformed into `&mut T`;
///  - The contained value can be infallibly retrieved, consuming the `Owned` object.
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

/// A type exposing shared `&T` access to a contained or referenced value.
pub enum Immut<'a, T> {
    Plain(T),
    Box(Box<T>),
    Rc(Rc<T>),
    Arc(Arc<T>),
    Ref(&'a T),
}

impl<T> AsRef<T> for Immut<'_, T> {
    fn as_ref(&self) -> &T {
        match self {
            Self::Plain(plain) => plain,
            Self::Box(boxed) => boxed,
            Self::Rc(rc) => rc,
            Self::Arc(arc) => arc,
            Self::Ref(borrow) => borrow,  
        }
    }
}

impl<T> Borrow<T> for Immut<'_, T> {
    fn borrow(&self) -> &T {
        self.as_ref()
    }
}

impl<T> From<T> for Immut<'_, T> {
    fn from(plain: T) -> Self {
        Self::Plain(plain)
    }
}

impl<T> From<Box<T>> for Immut<'_, T> {
    fn from(boxed: Box<T>) -> Self {
        Self::Box(boxed)
    }
}

impl<T> From<Rc<T>> for Immut<'_, T> {
    fn from(rc: Rc<T>) -> Self {
        Self::Rc(rc)
    }
}

impl<T> From<Arc<T>> for Immut<'_, T> {
    fn from(arc: Arc<T>) -> Self {
        Self::Arc(arc)
    }
}

impl<'a, T> From<&'a T> for Immut<'a, T> {
    fn from(borrow: &'a T) -> Self {
        Self::Ref(borrow)
    }
}

/// A type that exposes exclusive access to a contained value, without interior mutability.
/// 
/// More precisely:
///  - Any shared reference `&Mut<T>` may be converted to a `&T`;
///  - Any mutable reference `&mut Mut<T>` may be converted to a `&mut T`.
pub enum Mutable<'a, T> {
    Plain(T),
    Box(Box<T>),
    RefMut(&'a mut T),
}

impl<T> AsRef<T> for Mutable<'_, T> {
    fn as_ref(&self) -> &T {
        match self {
            Self::Plain(plain) => plain,
            Self::Box(boxed) => boxed,
            Self::RefMut(ref_mut) => ref_mut,
        }
    }
}

impl<T> AsMut<T> for Mutable<'_, T> {
    fn as_mut(&mut self) -> &mut T {
        match self {
            Self::Plain(plain) => plain,
            Self::Box(boxed) => boxed,
            Self::RefMut(ref_mut) => ref_mut,
        }
    }
}

impl<T> Borrow<T> for Mutable<'_, T> {
    fn borrow(&self) -> &T {
        self.as_ref()
    }
}

impl<T> BorrowMut<T> for Mutable<'_, T> {
    fn borrow_mut(&mut self) -> &mut T {
        self.as_mut()
    }
}


impl<T> From<T> for Mutable<'_, T> {
    fn from(plain: T) -> Self {
        Self::Plain(plain)
    }
}

impl<T> From<Box<T>> for Mutable<'_, T> {
    fn from(boxed: Box<T>) -> Self {
        Self::Box(boxed)
    }
}

impl<'a, T> From<&'a mut T> for Mutable<'a, T> {
    fn from(ref_mut: &'a mut T) -> Self {
        Self::RefMut(ref_mut)
    }
}
