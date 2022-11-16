use std::{
    borrow::{Borrow, BorrowMut},
    cell::{Ref, RefCell, RefMut},
    rc::Rc,
    sync::{Arc, Mutex, MutexGuard, RwLock, RwLockReadGuard, RwLockWriteGuard},
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
///  - Any shared reference `&Exclusive<T>` may be converted to a `&T`;
///  - Any mutable reference `&mut Exclusive<T>` may be converted to a `&mut T`.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Exclusive<'a, T> {
    Plain(T),
    RefMut(&'a mut T),
}

impl<T> AsRef<T> for Exclusive<'_, T> {
    fn as_ref(&self) -> &T {
        match self {
            Self::Plain(plain) => plain,
            Self::RefMut(ref_mut) => ref_mut,
        }
    }
}

impl<T> AsMut<T> for Exclusive<'_, T> {
    fn as_mut(&mut self) -> &mut T {
        match self {
            Self::Plain(plain) => plain,
            Self::RefMut(ref_mut) => ref_mut,
        }
    }
}

impl<T> Borrow<T> for Exclusive<'_, T> {
    fn borrow(&self) -> &T {
        self.as_ref()
    }
}

impl<T> BorrowMut<T> for Exclusive<'_, T> {
    fn borrow_mut(&mut self) -> &mut T {
        self.as_mut()
    }
}

impl<T> From<T> for Exclusive<'_, T> {
    fn from(plain: T) -> Self {
        Self::Plain(plain)
    }
}

impl<'a, T> From<&'a mut T> for Exclusive<'a, T> {
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
///  - Any shared reference `&ExclusiveUnsized<T>` may be converted to a `&T`;
///  - Any mutable reference `&mut ExclusiveUnsized<T>` may be converted to a `&mut T`.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ExclusiveUnsized<'a, T: ?Sized> {
    Box(Box<T>),
    RefMut(&'a mut T),
}

impl<T: ?Sized> AsRef<T> for ExclusiveUnsized<'_, T> {
    fn as_ref(&self) -> &T {
        match self {
            Self::Box(boxed) => boxed,
            Self::RefMut(ref_mut) => ref_mut,
        }
    }
}

impl<T: ?Sized> AsMut<T> for ExclusiveUnsized<'_, T> {
    fn as_mut(&mut self) -> &mut T {
        match self {
            Self::Box(boxed) => boxed,
            Self::RefMut(ref_mut) => ref_mut,
        }
    }
}

impl<T: ?Sized> Borrow<T> for ExclusiveUnsized<'_, T> {
    fn borrow(&self) -> &T {
        self.as_ref()
    }
}

impl<T: ?Sized> BorrowMut<T> for ExclusiveUnsized<'_, T> {
    fn borrow_mut(&mut self) -> &mut T {
        self.as_mut()
    }
}

impl<T: ?Sized> From<Box<T>> for ExclusiveUnsized<'_, T> {
    fn from(boxed: Box<T>) -> Self {
        Self::Box(boxed)
    }
}

impl<'a, T: ?Sized> From<&'a mut T> for ExclusiveUnsized<'a, T> {
    fn from(ref_mut: &'a mut T) -> Self {
        Self::RefMut(ref_mut)
    }
}

/// A lock providing shared access.
///
/// What is meant by 'lock' is a data structure that provides access to an object through a
/// 'locking' method, which returns an RAII guard instead of a plain reference.
pub enum ReadLock<'a, T> {
    Plain(T),
    Rc(Rc<T>),
    RcRefCell(Rc<RefCell<T>>),
    Arc(Arc<T>),
    ArcMutex(Arc<Mutex<T>>),
    ArcRwLock(Arc<RwLock<T>>),
    Ref(&'a T),
}

/// An RAII guard providing shared `&T` access.
pub enum ReadGuard<'a, T> {
    RefGuard(Ref<'a, T>),
    MutexGuard(MutexGuard<'a, T>),
    RwLockGuard(RwLockReadGuard<'a, T>),
    Ref(&'a T),
}

impl<T> ReadLock<'_, T> {
    /// Locks the `ReadLock`, returning an RAII guard containing a shared `&T` reference.
    pub fn lock(&self) -> ReadGuard<T> {
        match self {
            Self::Plain(plain) => ReadGuard::Ref(plain),
            Self::Rc(rc) => ReadGuard::Ref(rc),
            Self::RcRefCell(rc_ref_cell) => ReadGuard::RefGuard(rc_ref_cell.as_ref().borrow()),
            Self::Arc(arc) => ReadGuard::Ref(arc),
            Self::ArcMutex(arc_mutex) => {
                ReadGuard::MutexGuard(arc_mutex.lock().expect("failed to lock `std::sync::Mutex`"))
            }
            Self::ArcRwLock(arc_rw_lock) => ReadGuard::RwLockGuard(
                arc_rw_lock
                    .read()
                    .expect("failed to lock `std::sync::RwLock`"),
            ),
            Self::Ref(borrow) => ReadGuard::Ref(borrow),
        }
    }
}

/// A lock providing mutable access.
///
/// What is meant by 'lock' is a data structure that provides access to an object through a
/// 'locking' method, which returns an RAII guard instead of a plain reference.
pub enum WriteLock<T> {
    RefCell(RefCell<T>),
    RcRefCell(Rc<RefCell<T>>),
    ArcMutex(Arc<Mutex<T>>),
    ArcRwLock(Arc<RwLock<T>>),
}

pub enum WriteGuard<'a, T> {
    RefMutGuard(RefMut<'a, T>),
    MutexGuard(MutexGuard<'a, T>),
    RwLockGuard(RwLockWriteGuard<'a, T>),
}

impl<T> WriteLock<T> {
    /// Locks the `WriteLock`, returning an RAII guard containing a mutable `&mut T` reference.
    pub fn lock(&self) -> WriteGuard<T> {
        match self {
            Self::RefCell(ref_cell) => WriteGuard::RefMutGuard(ref_cell.borrow_mut()),
            Self::RcRefCell(ref_cell) => WriteGuard::RefMutGuard(ref_cell.as_ref().borrow_mut()),
            Self::ArcMutex(mutex) => {
                WriteGuard::MutexGuard(mutex.lock().expect("failed to lock `std::sync::Mutex`"))
            }
            Self::ArcRwLock(rw_lock) => WriteGuard::RwLockGuard(
                rw_lock.write().expect("failed to lock `std::sync::RwLock`"),
            ),
        }
    }
}

/// A lock providing shared and mutable access.
///
/// What is meant by 'lock' is a data structure that provides access to an object through a
/// 'locking' method, which returns an RAII guard instead of a plain reference.
pub enum Lock<T> {
    RefCell(RefCell<T>),
    RcRefCell(Rc<RefCell<T>>),
    ArcMutex(Arc<Mutex<T>>),
    ArcRwLock(Arc<RwLock<T>>),
}

impl<T> Lock<T> {
    /// Locks the `Lock`, returning an RAII guard containing a shared `&T` reference.
    pub fn lock(&self) -> ReadGuard<T> {
        match self {
            Self::RefCell(ref_cell) => ReadGuard::RefGuard(ref_cell.borrow()),
            Self::RcRefCell(rc_ref_cell) => ReadGuard::RefGuard(rc_ref_cell.as_ref().borrow()),
            Self::ArcMutex(arc_mutex) => {
                ReadGuard::MutexGuard(arc_mutex.lock().expect("failed to lock `std::sync::Mutex`"))
            }
            Self::ArcRwLock(arc_rw_lock) => ReadGuard::RwLockGuard(
                arc_rw_lock
                    .read()
                    .expect("failed to lock `std::sync::RwLock`"),
            ),
        }
    }

    /// Locks the `Lock`, returning an RAII guard containing a mutable `&mut T` reference.
    pub fn lock_mut(&self) -> WriteGuard<T> {
        match self {
            Self::RefCell(ref_cell) => WriteGuard::RefMutGuard(ref_cell.borrow_mut()),
            Self::RcRefCell(rc_ref_cell) => {
                WriteGuard::RefMutGuard(rc_ref_cell.as_ref().borrow_mut())
            }
            Self::ArcMutex(arc_mutex) => {
                WriteGuard::MutexGuard(arc_mutex.lock().expect("failed to lock `std::sync::Mutex`"))
            }
            Self::ArcRwLock(arc_rw_lock) => WriteGuard::RwLockGuard(
                arc_rw_lock
                    .write()
                    .expect("failed to lock `std::sync::RwLock`"),
            ),
        }
    }
}
