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
//! use anyform::SharedUnsized;
//! let hello = SharedUnsized::<str>::Ref("Hi :D");
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
//! let mut hello = Mutable::<String>::Plain("Hi".to_owned());
//! hello.push_str(" :D");
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
//!         *counter += 1;
//!         // Let the main thread know when we're done.
//!         if *counter == 10 {
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

use std::{
    borrow::{Borrow, BorrowMut},
    ops::{Deref, DerefMut},
    sync::{Arc, Mutex, MutexGuard, RwLock, RwLockReadGuard, RwLockWriteGuard},
};

// TODO: Send & Sync behavior

/// A type exposing shared `&T` access to a contained or referenced value.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Shared<'a, T, B: ?Sized = T>
where
    T: Borrow<B>,
{
    Plain(T),
    Arc(Arc<T>),
    Ref(&'a B),
}

impl<T, B: ?Sized> AsRef<B> for Shared<'_, T, B>
where
    T: Borrow<B>,
{
    fn as_ref(&self) -> &B {
        match self {
            Self::Plain(plain) => plain.borrow(),
            Self::Arc(arc) => arc.as_ref().borrow(),
            Self::Ref(borrow) => borrow,
        }
    }
}

impl<T, B: ?Sized> Borrow<B> for Shared<'_, T, B>
where
    T: Borrow<B>,
{
    fn borrow(&self) -> &B {
        self.as_ref()
    }
}

impl<T, B: ?Sized> Deref for Shared<'_, T, B>
where
    T: Borrow<B>,
{
    type Target = B;
    fn deref(&self) -> &Self::Target {
        self.as_ref()
    }
}

/// A type exposing shared `&T` access to a possibly unsized value.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum SharedUnsized<'a, T: ?Sized, B: ?Sized = T>
where
    T: Borrow<B>,
{
    Box(Box<T>),
    Arc(Arc<T>),
    Ref(&'a B),
}

impl<T: ?Sized, B: ?Sized> AsRef<B> for SharedUnsized<'_, T, B>
where
    T: Borrow<B>,
{
    fn as_ref(&self) -> &B {
        match self {
            Self::Box(boxed) => boxed.as_ref().borrow(),
            Self::Arc(arc) => arc.as_ref().borrow(),
            Self::Ref(borrow) => borrow,
        }
    }
}

impl<T: ?Sized, B: ?Sized> Borrow<B> for SharedUnsized<'_, T, B>
where
    T: Borrow<B>,
{
    fn borrow(&self) -> &B {
        self.as_ref()
    }
}

impl<T: ?Sized, B: ?Sized> Deref for SharedUnsized<'_, T, B>
where
    T: Borrow<B>,
{
    type Target = B;
    fn deref(&self) -> &Self::Target {
        self.as_ref()
    }
}

/// A type that exposes exclusive access to some value, without interior mutability.
///
/// More precisely:
///  - Any shared reference `&Mutable<T>` may be converted to a `&T`;
///  - Any mutable reference `&mut Mutable<T>` may be converted to a `&mut T`.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Mutable<'a, T, B: ?Sized = T>
where
    T: BorrowMut<B>,
{
    Plain(T),
    RefMut(&'a mut B),
}

impl<T, B: ?Sized> AsRef<B> for Mutable<'_, T, B>
where
    T: BorrowMut<B>,
{
    fn as_ref(&self) -> &B {
        match self {
            Self::Plain(plain) => plain.borrow(),
            Self::RefMut(ref_mut) => ref_mut,
        }
    }
}

impl<T, B: ?Sized> Borrow<B> for Mutable<'_, T, B>
where
    T: BorrowMut<B>,
{
    fn borrow(&self) -> &B {
        self.as_ref()
    }
}

impl<T, B: ?Sized> Deref for Mutable<'_, T, B>
where
    T: BorrowMut<B>,
{
    type Target = B;
    fn deref(&self) -> &Self::Target {
        self.as_ref()
    }
}

impl<T, B: ?Sized> AsMut<B> for Mutable<'_, T, B>
where
    T: BorrowMut<B>,
{
    fn as_mut(&mut self) -> &mut B {
        match self {
            Self::Plain(plain) => plain.borrow_mut(),
            Self::RefMut(ref_mut) => ref_mut,
        }
    }
}

impl<T, B: ?Sized> BorrowMut<B> for Mutable<'_, T, B>
where
    T: BorrowMut<B>,
{
    fn borrow_mut(&mut self) -> &mut B {
        self.as_mut()
    }
}

impl<T, B: ?Sized> DerefMut for Mutable<'_, T, B>
where
    T: BorrowMut<B>,
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.as_mut()
    }
}
/// A type that exposes exclusive access to some possibly unsized value, without interior
/// mutability.
///
/// More precisely:
///  - Any shared reference `&MutableUnsized<T>` may be converted to a `&T`;
///  - Any mutable reference `&mut MutableUnsized<T>` may be converted to a `&mut T`.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum MutableUnsized<'a, T: ?Sized, B: ?Sized = T>
where
    T: BorrowMut<B>,
{
    Box(Box<T>),
    RefMut(&'a mut B),
}

impl<T: ?Sized, B: ?Sized> AsRef<B> for MutableUnsized<'_, T, B>
where
    T: BorrowMut<B>,
{
    fn as_ref(&self) -> &B {
        match self {
            Self::Box(boxed) => boxed.as_ref().borrow(),
            Self::RefMut(ref_mut) => ref_mut,
        }
    }
}

impl<T: ?Sized, B: ?Sized> Borrow<B> for MutableUnsized<'_, T, B>
where
    T: BorrowMut<B>,
{
    fn borrow(&self) -> &B {
        self.as_ref()
    }
}

impl<T: ?Sized, B: ?Sized> Deref for MutableUnsized<'_, T, B>
where
    T: BorrowMut<B>,
{
    type Target = B;
    fn deref(&self) -> &Self::Target {
        self.as_ref()
    }
}

impl<T: ?Sized, B: ?Sized> AsMut<B> for MutableUnsized<'_, T, B>
where
    T: BorrowMut<B>,
{
    fn as_mut(&mut self) -> &mut B {
        match self {
            Self::Box(boxed) => boxed.as_mut().borrow_mut(),
            Self::RefMut(ref_mut) => ref_mut,
        }
    }
}

impl<T: ?Sized, B: ?Sized> BorrowMut<B> for MutableUnsized<'_, T, B>
where
    T: BorrowMut<B>,
{
    fn borrow_mut(&mut self) -> &mut B {
        self.as_mut()
    }
}

impl<T: ?Sized, B: ?Sized> DerefMut for MutableUnsized<'_, T, B>
where
    T: BorrowMut<B>,
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.as_mut()
    }
}

/// An RAII guard providing shared `&T` access.
pub enum ReadGuard<'a, T: ?Sized, B: ?Sized = T>
where
    T: Borrow<B>,
{
    MutexGuard(MutexGuard<'a, T>),
    RwLockReadGuard(RwLockReadGuard<'a, T>),
    RwLockWriteGuard(RwLockWriteGuard<'a, T>),
    Ref(&'a B),
}

impl<T: ?Sized, B: ?Sized> AsRef<B> for ReadGuard<'_, T, B>
where
    T: Borrow<B>,
{
    fn as_ref(&self) -> &B {
        match self {
            Self::MutexGuard(guard) => (**guard).borrow(),
            Self::RwLockReadGuard(guard) => (**guard).borrow(),
            Self::RwLockWriteGuard(guard) => (**guard).borrow(),
            Self::Ref(borrow) => borrow,
        }
    }
}

impl<T: ?Sized, B: ?Sized> Borrow<B> for ReadGuard<'_, T, B>
where
    T: Borrow<B>,
{
    fn borrow(&self) -> &B {
        self.as_ref()
    }
}

impl<T: ?Sized, B: ?Sized> Deref for ReadGuard<'_, T, B>
where
    T: Borrow<B>,
{
    type Target = B;
    fn deref(&self) -> &Self::Target {
        self.as_ref()
    }
}

/// An RAII guard providing mutable `&mut T` access.
pub enum WriteGuard<'a, T: ?Sized, B: ?Sized = T>
where
    T: BorrowMut<B>,
{
    MutexGuard(MutexGuard<'a, T>),
    RwLockWriteGuard(RwLockWriteGuard<'a, T>),
    RefMut(&'a mut B),
}

impl<T: ?Sized, B: ?Sized> AsRef<B> for WriteGuard<'_, T, B>
where
    T: BorrowMut<B>,
{
    fn as_ref(&self) -> &B {
        match self {
            Self::MutexGuard(guard) => (**guard).borrow(),
            Self::RwLockWriteGuard(guard) => (**guard).borrow(),
            Self::RefMut(ref_mut) => ref_mut,
        }
    }
}

impl<T: ?Sized, B: ?Sized> Borrow<B> for WriteGuard<'_, T, B>
where
    T: BorrowMut<B>,
{
    fn borrow(&self) -> &B {
        self.as_ref()
    }
}

impl<T: ?Sized, B: ?Sized> Deref for WriteGuard<'_, T, B>
where
    T: BorrowMut<B>,
{
    type Target = B;
    fn deref(&self) -> &Self::Target {
        self.as_ref()
    }
}

impl<T: ?Sized, B: ?Sized> AsMut<B> for WriteGuard<'_, T, B>
where
    T: BorrowMut<B>,
{
    fn as_mut(&mut self) -> &mut B {
        match self {
            Self::MutexGuard(guard) => (**guard).borrow_mut(),
            Self::RwLockWriteGuard(guard) => (**guard).borrow_mut(),
            Self::RefMut(ref_mut) => ref_mut,
        }
    }
}

impl<T: ?Sized, B: ?Sized> BorrowMut<B> for WriteGuard<'_, T, B>
where
    T: BorrowMut<B>,
{
    fn borrow_mut(&mut self) -> &mut B {
        self.as_mut()
    }
}

impl<T: ?Sized, B: ?Sized> DerefMut for WriteGuard<'_, T, B>
where
    T: BorrowMut<B>,
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.as_mut()
    }
}

/// A lock providing shared access.
///
/// What is meant by 'lock' is a data structure that provides access to an object through a
/// 'locking' method, which returns an RAII guard instead of a plain reference.
#[derive(Debug)]
pub enum ReadLock<'a, T, B: ?Sized = T>
where
    T: Borrow<B>,
{
    Plain(T),
    Arc(Arc<T>),
    ArcMutex(Arc<Mutex<T>>),
    ArcRwLock(Arc<RwLock<T>>),
    Ref(&'a B),
}

impl<T, B: ?Sized> ReadLock<'_, T, B>
where
    T: Borrow<B>,
{
    /// Locks the `ReadLock`, returning an RAII guard containing a shared `&T` reference.
    pub fn lock(&self) -> ReadGuard<T, B> {
        match self {
            Self::Plain(plain) => ReadGuard::Ref(plain.borrow()),
            Self::Arc(arc) => ReadGuard::Ref(arc.as_ref().borrow()),
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

/// A lock providing shared access to a possibly unsized value.
///
/// What is meant by 'lock' is a data structure that provides access to an object through a
/// 'locking' method, which returns an RAII guard instead of a plain reference.
#[derive(Debug)]
pub enum ReadLockUnsized<'a, T: ?Sized, B: ?Sized = T>
where
    T: Borrow<B>,
{
    Box(Box<T>),
    Arc(Arc<T>),
    ArcMutex(Arc<Mutex<T>>),
    ArcRwLock(Arc<RwLock<T>>),
    Ref(&'a B),
}

impl<T: ?Sized, B: ?Sized> ReadLockUnsized<'_, T, B>
where
    T: Borrow<B>,
{
    /// Locks the `ReadLockUnsized`, returning an RAII guard containing a shared `&T` reference.
    pub fn lock(&self) -> ReadGuard<T, B> {
        match self {
            Self::Box(boxed) => ReadGuard::Ref(boxed.as_ref().borrow()),
            Self::Arc(arc) => ReadGuard::Ref(arc.as_ref().borrow()),
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
