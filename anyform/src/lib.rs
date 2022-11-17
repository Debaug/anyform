use std::{
    borrow::{Borrow, BorrowMut},
    sync::{Arc, Mutex, MutexGuard, RwLock, RwLockReadGuard, RwLockWriteGuard},
};

// TODO: Send & Sync behavior

/// A type exposing shared `&T` access to a contained or referenced value.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Shared<'a, T> {
    Plain(T),
    // Rc(Rc<T>),
    Arc(Arc<T>),
    Ref(&'a T),
}

/// A type exposing shared `&T` access to a possibly unsized value.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum SharedUnsized<'a, T: ?Sized> {
    Box(Box<T>),
    Arc(Arc<T>),
    Ref(&'a T),
}

impl<T> AsRef<T> for Shared<'_, T> {
    fn as_ref(&self) -> &T {
        match self {
            Self::Plain(plain) => plain,
            // Self::Rc(rc) => rc,
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

impl<T: ?Sized> AsRef<T> for SharedUnsized<'_, T> {
    fn as_ref(&self) -> &T {
        match self {
            Self::Box(boxed) => boxed,
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

/// A type that exposes exclusive access to some value, without interior mutability.
///
/// More precisely:
///  - Any shared reference `&Mutable<T>` may be converted to a `&T`;
///  - Any mutable reference `&mut Mutable<T>` may be converted to a `&mut T`.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Mutable<'a, T> {
    Plain(T),
    RefMut(&'a mut T),
}

/// A type that exposes exclusive access to some possibly unsized value, without interior
/// mutability.
///
/// More precisely:
///  - Any shared reference `&MutableUnsized<T>` may be converted to a `&T`;
///  - Any mutable reference `&mut MutableUnsized<T>` may be converted to a `&mut T`.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum MutableUnsized<'a, T: ?Sized> {
    Box(Box<T>),
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

/// A lock providing shared access.
///
/// What is meant by 'lock' is a data structure that provides access to an object through a
/// 'locking' method, which returns an RAII guard instead of a plain reference.
pub enum ReadLock<'a, T> {
    Plain(T),
    Arc(Arc<T>),
    Mutex(Mutex<T>),
    RwLock(RwLock<T>),
    ArcMutex(Arc<Mutex<T>>),
    ArcRwLock(Arc<RwLock<T>>),
    Ref(&'a T),
}

/// A lock providing shared access to a possibly unsized value.
///
/// What is meant by 'lock' is a data structure that provides access to an object through a
/// 'locking' method, which returns an RAII guard instead of a plain reference.
pub enum ReadLockUnsized<'a, T: ?Sized> {
    Box(Box<T>),
    Arc(Arc<T>),
    BoxMutex(Box<Mutex<T>>),
    BoxRwLock(Box<RwLock<T>>),
    ArcMutex(Arc<Mutex<T>>),
    ArcRwLock(Arc<RwLock<T>>),
    Ref(&'a T),
}

/// An RAII guard providing shared `&T` access.
pub enum ReadGuard<'a, T: ?Sized> {
    MutexGuard(MutexGuard<'a, T>),
    RwLockGuard(RwLockReadGuard<'a, T>),
    Ref(&'a T),
}

impl<T> ReadLock<'_, T> {
    /// Locks the `ReadLock`, returning an RAII guard containing a shared `&T` reference.
    pub fn lock(&self) -> ReadGuard<T> {
        match self {
            Self::Plain(plain) => ReadGuard::Ref(plain),
            // Self::Rc(rc) => ReadGuard::Ref(rc),
            // Self::RcRefCell(rc_ref_cell) => ReadGuard::RefGuard(rc_ref_cell.as_ref().borrow()),
            Self::Arc(arc) => ReadGuard::Ref(arc),
            Self::Mutex(mutex) => {
                ReadGuard::MutexGuard(mutex.lock().expect("failed to lock `std::sync::Mutex`"))
            }
            Self::RwLock(rw_lock) => {
                ReadGuard::RwLockGuard(rw_lock.read().expect("failed to lock `std::sync::RwLock`"))
            }
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

impl<T> From<T> for ReadLock<'_, T> {
    fn from(plain: T) -> Self {
        Self::Plain(plain)
    }
}

impl<T> From<Arc<T>> for ReadLock<'_, T> {
    fn from(arc: Arc<T>) -> Self {
        Self::Arc(arc)
    }
}

impl<T> From<Mutex<T>> for ReadLock<'_, T> {
    fn from(mutex: Mutex<T>) -> Self {
        Self::Mutex(mutex)
    }
}

impl<T> From<RwLock<T>> for ReadLock<'_, T> {
    fn from(rw_lock: RwLock<T>) -> Self {
        Self::RwLock(rw_lock)
    }
}

impl<T> From<Arc<Mutex<T>>> for ReadLock<'_, T> {
    fn from(arc_mutex: Arc<Mutex<T>>) -> Self {
        Self::ArcMutex(arc_mutex)
    }
}

impl<T> From<Arc<RwLock<T>>> for ReadLock<'_, T> {
    fn from(arc_rw_lock: Arc<RwLock<T>>) -> Self {
        Self::ArcRwLock(arc_rw_lock)
    }
}

impl<'a, T> From<&'a T> for ReadLock<'a, T> {
    fn from(borrow: &'a T) -> Self {
        Self::Ref(borrow)
    }
}

impl<T> ReadLockUnsized<'_, T> {
    /// Locks the `ReadLockUnsized`, returning an RAII guard containing a shared `&T` reference.
    pub fn lock(&self) -> ReadGuard<T> {
        match self {
            Self::Box(boxed) => ReadGuard::Ref(boxed),
            // Self::Rc(rc) => ReadGuard::Ref(rc),
            // Self::RcRefCell(rc_ref_cell) => ReadGuard::RefGuard(rc_ref_cell.as_ref().borrow()),
            Self::Arc(arc) => ReadGuard::Ref(arc),
            Self::BoxMutex(box_mutex) => {
                ReadGuard::MutexGuard(box_mutex.lock().expect("failed to lock `std::sync::Mutex`"))
            }
            Self::BoxRwLock(rw_lock) => {
                ReadGuard::RwLockGuard(rw_lock.read().expect("failed to lock `std::sync::RwLock`"))
            }
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

impl<T: ?Sized> From<Box<T>> for ReadLockUnsized<'_, T> {
    fn from(boxed: Box<T>) -> Self {
        Self::Box(boxed)
    }
}

impl<T: ?Sized> From<Arc<T>> for ReadLockUnsized<'_, T> {
    fn from(arc: Arc<T>) -> Self {
        Self::Arc(arc)
    }
}

impl<T: ?Sized> From<Box<Mutex<T>>> for ReadLockUnsized<'_, T> {
    fn from(box_mutex: Box<Mutex<T>>) -> Self {
        Self::BoxMutex(box_mutex)
    }
}

impl<T: ?Sized> From<Box<RwLock<T>>> for ReadLockUnsized<'_, T> {
    fn from(box_rw_lock: Box<RwLock<T>>) -> Self {
        Self::BoxRwLock(box_rw_lock)
    }
}

impl<T: ?Sized> From<Arc<Mutex<T>>> for ReadLockUnsized<'_, T> {
    fn from(arc_mutex: Arc<Mutex<T>>) -> Self {
        Self::ArcMutex(arc_mutex)
    }
}

impl<T: ?Sized> From<Arc<RwLock<T>>> for ReadLockUnsized<'_, T> {
    fn from(arc_rw_lock: Arc<RwLock<T>>) -> Self {
        Self::ArcRwLock(arc_rw_lock)
    }
}

impl<'a, T: ?Sized> From<&'a T> for ReadLockUnsized<'a, T> {
    fn from(borrow: &'a T) -> Self {
        Self::Ref(borrow)
    }
}

/// A lock providing mutable access through interior mutability.
///
/// What is meant by 'lock' is a data structure that provides access to an object through a
/// 'locking' method, which returns an RAII guard instead of a plain reference.
pub enum WriteLock<T> {
    Mutex(Mutex<T>),
    RwLock(RwLock<T>),
    ArcMutex(Arc<Mutex<T>>),
    ArcRwLock(Arc<RwLock<T>>),
}

/// A lock providing mutable access to a possibly unsized value.
///
/// What is meant by 'lock' is a data structure that provides access to an object through a
/// 'locking' method, which returns an RAII guard instead of a plain reference.
pub enum WriteLockUnsized<T: ?Sized> {
    BoxMutex(Box<Mutex<T>>),
    BoxRwLock(Box<RwLock<T>>),
    ArcMutex(Arc<Mutex<T>>),
    ArcRwLock(Arc<RwLock<T>>),
}

/// An RAII guard providing mutable `&mut T` access.
pub enum WriteGuard<'a, T: ?Sized> {
    MutexGuard(MutexGuard<'a, T>),
    RwLockGuard(RwLockWriteGuard<'a, T>),
}

impl<T> WriteLock<T> {
    /// Locks the `WriteLock`, returning an RAII guard containing a mutable `&mut T` reference.
    pub fn lock(&self) -> WriteGuard<T> {
        match self {
            // Self::RefCell(ref_cell) => WriteGuard::RefMutGuard(ref_cell.borrow_mut()),
            // Self::RcRefCell(ref_cell) => WriteGuard::RefMutGuard(ref_cell.as_ref().borrow_mut()),
            Self::Mutex(mutex) => {
                WriteGuard::MutexGuard(mutex.lock().expect("failed to lock `std::sync::Mutex`"))
            }
            Self::RwLock(rw_lock) => WriteGuard::RwLockGuard(
                rw_lock.write().expect("failed to lock `std::sync::RwLock`"),
            ),
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

impl<T> From<Mutex<T>> for WriteLock<T> {
    fn from(mutex: Mutex<T>) -> Self {
        Self::Mutex(mutex)
    }
}

impl<T> From<RwLock<T>> for WriteLock<T> {
    fn from(rw_lock: RwLock<T>) -> Self {
        Self::RwLock(rw_lock)
    }
}

impl<T> From<Arc<Mutex<T>>> for WriteLock<T> {
    fn from(arc_mutex: Arc<Mutex<T>>) -> Self {
        Self::ArcMutex(arc_mutex)
    }
}

impl<T> From<Arc<RwLock<T>>> for WriteLock<T> {
    fn from(arc_rw_lock: Arc<RwLock<T>>) -> Self {
        Self::ArcRwLock(arc_rw_lock)
    }
}

impl<T> WriteLockUnsized<T> {
    /// Locks the `WriteLockUnsized`, returning an RAII guard containing a mutable `&mut T` reference.
    pub fn lock(&self) -> WriteGuard<T> {
        match self {
            // Self::RefCell(ref_cell) => WriteGuard::RefMutGuard(ref_cell.borrow_mut()),
            // Self::RcRefCell(ref_cell) => WriteGuard::RefMutGuard(ref_cell.as_ref().borrow_mut()),
            Self::BoxMutex(box_mutex) => {
                WriteGuard::MutexGuard(box_mutex.lock().expect("failed to lock `std::sync::Mutex`"))
            }
            Self::BoxRwLock(box_rw_lock) => WriteGuard::RwLockGuard(
                box_rw_lock
                    .write()
                    .expect("failed to lock `std::sync::RwLock`"),
            ),
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

impl<T: ?Sized> From<Box<Mutex<T>>> for WriteLockUnsized<T> {
    fn from(box_mutex: Box<Mutex<T>>) -> Self {
        Self::BoxMutex(box_mutex)
    }
}

impl<T: ?Sized> From<Box<RwLock<T>>> for WriteLockUnsized<T> {
    fn from(box_rw_lock: Box<RwLock<T>>) -> Self {
        Self::BoxRwLock(box_rw_lock)
    }
}

impl<T: ?Sized> From<Arc<Mutex<T>>> for WriteLockUnsized<T> {
    fn from(arc_mutex: Arc<Mutex<T>>) -> Self {
        Self::ArcMutex(arc_mutex)
    }
}

impl<T: ?Sized> From<Arc<RwLock<T>>> for WriteLockUnsized<T> {
    fn from(arc_rw_lock: Arc<RwLock<T>>) -> Self {
        Self::ArcRwLock(arc_rw_lock)
    }
}

/// A lock providing shared and mutable access through interior mutability.
///
/// What is meant by 'lock' is a data structure that provides access to an object through a
/// 'locking' method, which returns an RAII guard instead of a plain reference.
pub enum Lock<T> {
    Mutex(Mutex<T>),
    RwLock(RwLock<T>),
    ArcMutex(Arc<Mutex<T>>),
    ArcRwLock(Arc<RwLock<T>>),
}

/// A lock providing shared and mutable access to a possibly unsized value through interior
/// mutability.
///
/// What is meant by 'lock' is a data structure that provides access to an object through a
/// 'locking' method, which returns an RAII guard instead of a plain reference.
pub enum LockUnsized<T: ?Sized> {
    BoxMutex(Box<Mutex<T>>),
    BoxRwLock(Box<RwLock<T>>),
    ArcMutex(Arc<Mutex<T>>),
    ArcRwLock(Arc<RwLock<T>>),
}

impl<T> Lock<T> {
    /// Locks the `Lock`, returning an RAII guard containing a shared `&T` reference.
    pub fn lock(&self) -> ReadGuard<T> {
        match self {
            Self::Mutex(mutex) => {
                ReadGuard::MutexGuard(mutex.lock().expect("failed to lock `std::sync::Mutex`"))
            }
            Self::RwLock(rw_lock) => {
                ReadGuard::RwLockGuard(rw_lock.read().expect("failed to lock `std::sync::RwLock`"))
            }
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
            Self::Mutex(mutex) => {
                WriteGuard::MutexGuard(mutex.lock().expect("failed to lock `std::sync::Mutex`"))
            }
            Self::RwLock(rw_lock) => WriteGuard::RwLockGuard(
                rw_lock.write().expect("failed to lock `std::sync::RwLock`"),
            ),
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

impl<T> From<Mutex<T>> for Lock<T> {
    fn from(mutex: Mutex<T>) -> Self {
        Self::Mutex(mutex)
    }
}

impl<T> From<RwLock<T>> for Lock<T> {
    fn from(rw_lock: RwLock<T>) -> Self {
        Self::RwLock(rw_lock)
    }
}

impl<T> From<Arc<Mutex<T>>> for Lock<T> {
    fn from(arc_mutex: Arc<Mutex<T>>) -> Self {
        Self::ArcMutex(arc_mutex)
    }
}

impl<T> From<Arc<RwLock<T>>> for Lock<T> {
    fn from(arc_rw_lock: Arc<RwLock<T>>) -> Self {
        Self::ArcRwLock(arc_rw_lock)
    }
}

impl<T: ?Sized> LockUnsized<T> {
    /// Locks the `LockUnsized`, returning an RAII guard containing a shared `&T` reference.
    pub fn lock(&self) -> ReadGuard<T> {
        match self {
            Self::BoxMutex(box_mutex) => {
                ReadGuard::MutexGuard(box_mutex.lock().expect("failed to lock `std::sync::Mutex`"))
            }

            Self::BoxRwLock(box_rw_lock) => ReadGuard::RwLockGuard(
                box_rw_lock
                    .read()
                    .expect("failed to lock `std::sync::RwLock`"),
            ),
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
            Self::BoxMutex(box_mutex) => {
                WriteGuard::MutexGuard(box_mutex.lock().expect("failed to lock `std::sync::Mutex`"))
            }
            Self::BoxRwLock(box_rw_lock) => WriteGuard::RwLockGuard(
                box_rw_lock
                    .write()
                    .expect("failed to lock `std::sync::RwLock`"),
            ),
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

impl<T: ?Sized> From<Box<Mutex<T>>> for LockUnsized<T> {
    fn from(box_mutex: Box<Mutex<T>>) -> Self {
        Self::BoxMutex(box_mutex)
    }
}

impl<T: ?Sized> From<Box<RwLock<T>>> for LockUnsized<T> {
    fn from(box_rw_lock: Box<RwLock<T>>) -> Self {
        Self::BoxRwLock(box_rw_lock)
    }
}

impl<T: ?Sized> From<Arc<Mutex<T>>> for LockUnsized<T> {
    fn from(arc_mutex: Arc<Mutex<T>>) -> Self {
        Self::ArcMutex(arc_mutex)
    }
}

impl<T: ?Sized> From<Arc<RwLock<T>>> for LockUnsized<T> {
    fn from(arc_rw_lock: Arc<RwLock<T>>) -> Self {
        Self::ArcRwLock(arc_rw_lock)
    }
}
