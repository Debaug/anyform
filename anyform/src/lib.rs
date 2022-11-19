//! Utilities for accessing objects of any form.
//!
//! The philosophy promoted in this crate is that you shouldn't think of a particular value as how
//! it is *stored*, but what you can *do with it*.
//!
//! # Shared access
//!
//! If you only intend to have shared `&T` access to a value, the [`Shared`] and [`SharedUnsized`]
//! might be good options. For example:
//!
//! ```
//! use anyform::Shared;
//! let hello = Shared::Plain("Hi :D".to_owned());
//! assert_eq!(hello.as_ref(), "Hi :D");
//! ```
//!
//! # Mutable access
//!
//! The [`Mutable`] and [`MutableUnsized`] types provide two kinds of access: `&T` from a
//! `&Mutable<T>` and `&mut T` from `&mut Mutable<T>`. For example:
//!
//! ```
//! use anyform::Mutable;
//! let mut hello = Mutable::Plain("Hi".to_owned());
//! hello.as_mut().push_str(" :D");
//! assert_eq!(hello.as_ref(), "Hi :D");
//! ```
//!
//! # Locks
//!
//! Locks are data structures that provide shared of mutable access to a contained value, checking
//! the borrow rules at runtime instead of compile-time. As such, they provide a 'locking'
//! mechanism that returns an RAII guard which, in turn, references the contained value.
//!
//! The most well-known lock is probably the [mutex](std::sync::Mutex), but [`std`] also exposes
//! [`RwLock`](std::sync::RwLock) and [`RefCell`](std::cell::RefCell) which each work slightly
//! differently.
//!
//! Example:
//!
//! ```
//! use anyform::Lock;
//! use std::{sync::{mpsc, Arc, Mutex}, thread};
//!
//! let counter = Arc::new(Mutex::new(0));
//! let (tx, rx) = mpsc::channel();
//! // Each thread will increment the counter using a mutex.
//! for _ in 0..10 {
//!     let counter = Lock::ArcMutex(Arc::clone(&counter));
//!     let tx = tx.clone();
//!     thread::spawn(move || {
//!         let mut counter = counter.lock_mut();
//!         *counter.as_mut() += 1;
//!         // Let the main thread know when we're done.
//!         if *counter.as_mut() == 10 {
//!             tx.send(()).unwrap();
//!         }
//!     });
//! }
//! rx.recv().unwrap();
//! // The main thread has recieved a `()`, we know the calculation is complete.
//! assert_eq!(*counter.lock().unwrap(), 10);
//! ```
//!
//! # Note on Thread Safety
//!
//! One thing you may have noticed while browsing through this crate is it currently lacks support
//! for non-thread-safe constructs, like [`Rc`](std::rc::Rc) and [`RefCell`](std::cell::RefCell).
//! Because of this, every pointer `anyform` exposes implements both [`Send`] and [`Sync`]. The
//! guards only implement [`Sync`] though, as they must unlock their corresponding lock on the same
//! thread it was locked from.

use std::sync::{Arc, Mutex, MutexGuard, RwLock, RwLockReadGuard, RwLockWriteGuard};

// TODO: Send & Sync behavior

/// A type exposing shared `&T` access to a contained or referenced value.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Shared<'a, T> {
    Plain(T),
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
            Self::Arc(arc) => arc,
            Self::Ref(borrow) => borrow,
        }
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

/// An RAII guard providing shared `&T` access.
pub enum ReadGuard<'a, T: ?Sized> {
    MutexGuard(MutexGuard<'a, T>),
    RwLockReadGuard(RwLockReadGuard<'a, T>),
    RwLockWriteGuard(RwLockWriteGuard<'a, T>),
    Ref(&'a T),
}

impl<'a, T> From<MutexGuard<'a, T>> for ReadGuard<'a, T> {
    fn from(guard: MutexGuard<'a, T>) -> Self {
        Self::MutexGuard(guard)
    }
}

impl<'a, T> From<RwLockReadGuard<'a, T>> for ReadGuard<'a, T> {
    fn from(guard: RwLockReadGuard<'a, T>) -> Self {
        Self::RwLockReadGuard(guard)
    }
}

impl<'a, T> From<RwLockWriteGuard<'a, T>> for ReadGuard<'a, T> {
    fn from(guard: RwLockWriteGuard<'a, T>) -> Self {
        Self::RwLockWriteGuard(guard)
    }
}

impl<'a, T> From<&'a T> for ReadGuard<'a, T> {
    fn from(borrow: &'a T) -> Self {
        Self::Ref(borrow)
    }
}

impl<T> AsRef<T> for ReadGuard<'_, T> {
    fn as_ref(&self) -> &T {
        match self {
            Self::MutexGuard(guard) => &guard,
            Self::RwLockReadGuard(guard) => &guard,
            Self::RwLockWriteGuard(guard) => &guard,
            Self::Ref(borrow) => borrow,
        }
    }
}

/// A lock providing shared access.
///
/// What is meant by 'lock' is a data structure that provides access to an object through a
/// 'locking' method, which returns an RAII guard instead of a plain reference.
#[derive(Debug)]
pub enum ReadLock<'a, T> {
    Plain(T),
    Arc(Arc<T>),
    ArcMutex(Arc<Mutex<T>>),
    ArcRwLock(Arc<RwLock<T>>),
    Ref(&'a T),
}

/// An RAII guard providing mutable `&mut T` access.
pub enum WriteGuard<'a, T: ?Sized> {
    MutexGuard(MutexGuard<'a, T>),
    RwLockWriteGuard(RwLockWriteGuard<'a, T>),
    RefMut(&'a mut T),
}

impl<'a, T> From<MutexGuard<'a, T>> for WriteGuard<'a, T> {
    fn from(guard: MutexGuard<'a, T>) -> Self {
        Self::MutexGuard(guard)
    }
}

impl<'a, T> From<RwLockWriteGuard<'a, T>> for WriteGuard<'a, T> {
    fn from(guard: RwLockWriteGuard<'a, T>) -> Self {
        Self::RwLockWriteGuard(guard)
    }
}

impl<'a, T> From<&'a mut T> for WriteGuard<'a, T> {
    fn from(ref_mut: &'a mut T) -> Self {
        Self::RefMut(ref_mut)
    }
}

impl<T> AsMut<T> for WriteGuard<'_, T> {
    fn as_mut(&mut self) -> &mut T {
        match self {
            Self::MutexGuard(guard) => guard,
            Self::RwLockWriteGuard(guard) => guard,
            Self::RefMut(ref_mut) => ref_mut,
        }
    }
}

/// A lock providing shared access to a possibly unsized value.
///
/// What is meant by 'lock' is a data structure that provides access to an object through a
/// 'locking' method, which returns an RAII guard instead of a plain reference.
#[derive(Debug)]
pub enum ReadLockUnsized<'a, T: ?Sized> {
    Box(Box<T>),
    Arc(Arc<T>),
    ArcMutex(Arc<Mutex<T>>),
    ArcRwLock(Arc<RwLock<T>>),
    Ref(&'a T),
}

impl<T> ReadLock<'_, T> {
    /// Locks the `ReadLock`, returning an RAII guard containing a shared `&T` reference.
    pub fn lock(&self) -> ReadGuard<T> {
        match self {
            Self::Plain(plain) => ReadGuard::Ref(plain),
            Self::Arc(arc) => ReadGuard::Ref(arc),
            Self::ArcMutex(arc_mutex) => {
                ReadGuard::MutexGuard(arc_mutex.lock().expect("failed to lock `std::sync::Mutex`"))
            }
            Self::ArcRwLock(arc_rw_lock) => ReadGuard::RwLockReadGuard(
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
            Self::Arc(arc) => ReadGuard::Ref(arc),
            Self::ArcMutex(arc_mutex) => {
                ReadGuard::MutexGuard(arc_mutex.lock().expect("failed to lock `std::sync::Mutex`"))
            }
            Self::ArcRwLock(arc_rw_lock) => ReadGuard::RwLockReadGuard(
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

/// A lock providing shared and mutable access through interior mutability.
///
/// What is meant by 'lock' is a data structure that provides access to an object through a
/// 'locking' method, which returns an RAII guard instead of a plain reference.
#[derive(Debug)]
pub enum Lock<T> {
    Mutex(Mutex<T>),
    RwLock(RwLock<T>),
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
            Self::RwLock(rw_lock) => ReadGuard::RwLockReadGuard(
                rw_lock.read().expect("failed to lock `std::sync::RwLock`"),
            ),
            Self::ArcMutex(arc_mutex) => {
                ReadGuard::MutexGuard(arc_mutex.lock().expect("failed to lock `std::sync::Mutex`"))
            }
            Self::ArcRwLock(arc_rw_lock) => ReadGuard::RwLockReadGuard(
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
            Self::RwLock(rw_lock) => WriteGuard::RwLockWriteGuard(
                rw_lock.write().expect("failed to lock `std::sync::RwLock`"),
            ),
            Self::ArcMutex(arc_mutex) => {
                WriteGuard::MutexGuard(arc_mutex.lock().expect("failed to lock `std::sync::Mutex`"))
            }
            Self::ArcRwLock(arc_rw_lock) => WriteGuard::RwLockWriteGuard(
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

/// A lock providing shared and mutable access to a possibly unsized value through interior
/// mutability.
///
/// What is meant by 'lock' is a data structure that provides access to an object through a
/// 'locking' method, which returns an RAII guard instead of a plain reference.
#[derive(Debug)]
pub enum LockUnsized<T: ?Sized> {
    BoxMutex(Box<Mutex<T>>),
    BoxRwLock(Box<RwLock<T>>),
    ArcMutex(Arc<Mutex<T>>),
    ArcRwLock(Arc<RwLock<T>>),
}

impl<T: ?Sized> LockUnsized<T> {
    /// Locks the `LockUnsized`, returning an RAII guard containing a shared `&T` reference.
    pub fn lock(&self) -> ReadGuard<T> {
        match self {
            Self::BoxMutex(box_mutex) => {
                ReadGuard::MutexGuard(box_mutex.lock().expect("failed to lock `std::sync::Mutex`"))
            }

            Self::BoxRwLock(box_rw_lock) => ReadGuard::RwLockReadGuard(
                box_rw_lock
                    .read()
                    .expect("failed to lock `std::sync::RwLock`"),
            ),
            Self::ArcMutex(arc_mutex) => {
                ReadGuard::MutexGuard(arc_mutex.lock().expect("failed to lock `std::sync::Mutex`"))
            }
            Self::ArcRwLock(arc_rw_lock) => ReadGuard::RwLockReadGuard(
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
            Self::BoxRwLock(box_rw_lock) => WriteGuard::RwLockWriteGuard(
                box_rw_lock
                    .write()
                    .expect("failed to lock `std::sync::RwLock`"),
            ),
            Self::ArcMutex(arc_mutex) => {
                WriteGuard::MutexGuard(arc_mutex.lock().expect("failed to lock `std::sync::Mutex`"))
            }
            Self::ArcRwLock(arc_rw_lock) => WriteGuard::RwLockWriteGuard(
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
