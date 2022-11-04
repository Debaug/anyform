use std::{
    borrow::{Borrow, BorrowMut},
    cell::{Ref, RefCell},
    rc::Rc,
    sync::{Arc, Mutex, MutexGuard, RwLock, RwLockReadGuard},
};

/// A type exposing shared `&T` access to a contained or referenced value.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Shared<'a, T> {
    Plain(T),
    Rc(Rc<T>),
    Arc(Arc<T>),
    Ref(&'a T),
}

impl<T> AsRef<T> for Shared<'_, T> {
    fn as_ref(&self) -> &T {
        match self {
            Self::Plain(plain) => plain,
            Self::Rc(rc) => rc,
            Self::Arc(arc) => arc,
            Self::Ref(borrow) => borrow,
        }
    }
}

impl<T> Borrow<T> for Shared<'_, T> {
    fn borrow(&self) -> &T {
        self.as_ref()
    }
}

impl<T> From<T> for Shared<'_, T> {
    fn from(plain: T) -> Self {
        Self::Plain(plain)
    }
}

impl<T> From<Rc<T>> for Shared<'_, T> {
    fn from(rc: Rc<T>) -> Self {
        Self::Rc(rc)
    }
}

impl<T> From<Arc<T>> for Shared<'_, T> {
    fn from(arc: Arc<T>) -> Self {
        Self::Arc(arc)
    }
}

impl<'a, T> From<&'a T> for Shared<'a, T> {
    fn from(borrow: &'a T) -> Self {
        Self::Ref(borrow)
    }
}

/// A type that exposes exclusive access to some value, without interior mutability.
///
/// More precisely:
///  - Any shared reference `&Mut<T>` may be converted to a `&T`;
///  - Any mutable reference `&mut Mut<T>` may be converted to a `&mut T`.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Mutable<'a, T> {
    Plain(T),
    RefMut(&'a mut T),
}

impl<T> AsRef<T> for Mutable<'_, T> {
    fn as_ref(&self) -> &T {
        match self {
            Self::Plain(plain) => plain,
            Self::RefMut(ref_mut) => ref_mut,
        }
    }
}

impl<T> AsMut<T> for Mutable<'_, T> {
    fn as_mut(&mut self) -> &mut T {
        match self {
            Self::Plain(plain) => plain,
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

impl<'a, T> From<&'a mut T> for Mutable<'a, T> {
    fn from(ref_mut: &'a mut T) -> Self {
        Self::RefMut(ref_mut)
    }
}

/// A type exposing shared `&T` access to a contained or referenced value, whose size is not known.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum SharedUnsized<'a, T: ?Sized> {
    Box(Box<T>),
    Rc(Rc<T>),
    Arc(Arc<T>),
    Ref(&'a T),
}

impl<T: ?Sized> AsRef<T> for SharedUnsized<'_, T> {
    fn as_ref(&self) -> &T {
        match self {
            Self::Box(boxed) => boxed,
            Self::Rc(rc) => rc,
            Self::Arc(arc) => arc,
            Self::Ref(borrow) => borrow,
        }
    }
}

impl<T: ?Sized> Borrow<T> for SharedUnsized<'_, T> {
    fn borrow(&self) -> &T {
        self.as_ref()
    }
}

impl<T: ?Sized> From<Box<T>> for SharedUnsized<'_, T> {
    fn from(boxed: Box<T>) -> Self {
        Self::Box(boxed)
    }
}

impl<T: ?Sized> From<Rc<T>> for SharedUnsized<'_, T> {
    fn from(rc: Rc<T>) -> Self {
        Self::Rc(rc)
    }
}

impl<T: ?Sized> From<Arc<T>> for SharedUnsized<'_, T> {
    fn from(arc: Arc<T>) -> Self {
        Self::Arc(arc)
    }
}

impl<'a, T: ?Sized> From<&'a T> for SharedUnsized<'a, T> {
    fn from(borrow: &'a T) -> Self {
        Self::Ref(borrow)
    }
}

/// A type that exposes exclusive access to some value whose size is not known, without interior
/// mutability.
///
/// More precisely:
///  - Any shared reference `&Mut<T>` may be converted to a `&T`;
///  - Any mutable reference `&mut Mut<T>` may be converted to a `&mut T`.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum MutableUnsized<'a, T: ?Sized> {
    Box(Box<T>),
    RefMut(&'a mut T),
}

impl<T: ?Sized> AsRef<T> for MutableUnsized<'_, T> {
    fn as_ref(&self) -> &T {
        match self {
            Self::Box(boxed) => boxed,
            Self::RefMut(ref_mut) => ref_mut,
        }
    }
}

impl<T: ?Sized> AsMut<T> for MutableUnsized<'_, T> {
    fn as_mut(&mut self) -> &mut T {
        match self {
            Self::Box(boxed) => boxed,
            Self::RefMut(ref_mut) => ref_mut,
        }
    }
}

impl<T: ?Sized> Borrow<T> for MutableUnsized<'_, T> {
    fn borrow(&self) -> &T {
        self.as_ref()
    }
}

impl<T: ?Sized> BorrowMut<T> for MutableUnsized<'_, T> {
    fn borrow_mut(&mut self) -> &mut T {
        self.as_mut()
    }
}

impl<T: ?Sized> From<Box<T>> for MutableUnsized<'_, T> {
    fn from(boxed: Box<T>) -> Self {
        Self::Box(boxed)
    }
}

impl<'a, T: ?Sized> From<&'a mut T> for MutableUnsized<'a, T> {
    fn from(ref_mut: &'a mut T) -> Self {
        Self::RefMut(ref_mut)
    }
}

pub enum ReadLock<'a, T> {
    Plain(T),
    Box(Box<T>),
    Rc(Rc<T>),
    RcRefCell(Rc<RefCell<T>>),
    Arc(Arc<T>),
    ArcMutex(Arc<Mutex<T>>),
    ArcRwLock(Arc<RwLock<T>>),
    Ref(&'a T),
}

pub enum ReadLockGuard<'a, T> {
    RefGuard(Ref<'a, T>),
    MutexGuard(MutexGuard<'a, T>),
    RwLockGuard(RwLockReadGuard<'a, T>),
    Ref(&'a T),
}

impl<T> ReadLock<'_, T> {
    pub fn lock(&self) -> ReadLockGuard<'_, T> {
        match self {
            Self::Plain(plain) => ReadLockGuard::Ref(plain),
            Self::Box(boxed) => ReadLockGuard::Ref(boxed),
            Self::Rc(rc) => ReadLockGuard::Ref(rc),
            Self::RcRefCell(rc_ref_cell) => ReadLockGuard::RefGuard(rc_ref_cell.as_ref().borrow()),
            Self::Arc(arc) => ReadLockGuard::Ref(arc),
            Self::ArcMutex(arc_mutex) => ReadLockGuard::MutexGuard(
                arc_mutex.lock().expect("failed to lock `std::sync::Mutex`"),
            ),
            Self::ArcRwLock(arc_rw_lock) => ReadLockGuard::RwLockGuard(
                arc_rw_lock
                    .as_ref()
                    .read()
                    .expect("failed to lock `std::sync::RwLock`"),
            ),
            Self::Ref(borrow) => ReadLockGuard::Ref(borrow),
        }
    }
}

pub enum WriteLock<'a, T> {
    Plain(T),
    Box(Box<T>),
    RcRefCell(Rc<RefCell<T>>),
    ArcMutex(Arc<Mutex<T>>),
    ArcRwLock(Arc<RwLock<T>>),
    RefMut(&'a mut T),
}
