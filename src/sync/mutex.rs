#![allow(dead_code)]

use core::cell::UnsafeCell;
use core::ops::Deref;
use core::ops::DerefMut;
use core::ptr::NonNull;
use core::sync::atomic::AtomicBool;
use core::sync::atomic::Ordering;

#[derive(Debug)]
pub struct LazyMutex<T, F: Fn()> {
    mutex: Mutex<T>,
    on_first_lock: F,
}

impl<T, F: Fn()> LazyMutex<T, F> {
    pub const fn new(t: T, f: F) -> Self {
        Self {
            mutex: Mutex::new(t),
            on_first_lock: f,
        }
    }

    /// Non-blocking attempt to acquire the lock.
    /// Returns `Some(MutexGuard)` on success, `None` on failure.
    #[inline]
    pub fn try_lock(&self) -> Option<MutexGuard<'_, T>> {
        static INITED: AtomicBool = AtomicBool::new(false);

        self.mutex.try_lock().inspect(|_| {
            if INITED
                .compare_exchange(false, true, Ordering::Acquire, Ordering::Relaxed)
                .is_ok()
            {
                (self.on_first_lock)();
            }
        })
    }
}

#[derive(Debug)]
pub struct MutexOnLock<T, F: Fn(&T) -> bool> {
    mutex: Mutex<T>,
    on_lock: F,
}

impl<T, F: Fn(&T) -> bool> MutexOnLock<T, F> {
    pub const fn new(t: T, on_lock: F) -> Self {
        Self {
            mutex: Mutex::new(t),
            on_lock,
        }
    }

    /// Non-blocking attempt to acquire the lock.
    /// Returns `Some(MutexGuard)` on success, `None` on failure or if `on_lock`
    /// function returns `false`.
    #[inline]
    pub fn try_lock(&self) -> Option<MutexGuard<'_, T>> {
        self.mutex
            .try_lock()
            .and_then(|x| (self.on_lock)(&x).then_some(x))
    }
}

#[derive(Default, Debug)]
pub struct Mutex<T: ?Sized> {
    is_locked: AtomicBool,
    data: UnsafeCell<T>,
}

impl<T> Mutex<T> {
    pub const fn new(t: T) -> Self {
        Self {
            is_locked: AtomicBool::new(false),
            data: UnsafeCell::new(t),
        }
    }
}

impl<T: ?Sized> Mutex<T> {
    /// Non-blocking attempt to acquire the lock.
    /// Returns `Some(MutexGuard)` on success, `None` on failure.
    #[inline]
    pub fn try_lock(&self) -> Option<MutexGuard<'_, T>> {
        match self
            .is_locked
            .compare_exchange(false, true, Ordering::Acquire, Ordering::Relaxed)
        {
            Ok(_) => Some(MutexGuard { mutex: self }),
            Err(_) => None,
        }
    }
}

unsafe impl<T: ?Sized + Send> Send for Mutex<T> {}
unsafe impl<T: ?Sized + Send> Sync for Mutex<T> {}

#[derive(Debug)]
pub struct MutexGuard<'a, T: ?Sized> {
    mutex: &'a Mutex<T>,
}

unsafe impl<T: ?Sized + Send> Send for MutexGuard<'_, T> {}
unsafe impl<T: ?Sized + Sync> Sync for MutexGuard<'_, T> {}

impl<T: ?Sized> Deref for MutexGuard<'_, T> {
    type Target = T;

    #[inline]
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.mutex.data.get() }
    }
}

impl<T: ?Sized> DerefMut for MutexGuard<'_, T> {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut *self.mutex.data.get() }
    }
}

impl<T: ?Sized> Drop for MutexGuard<'_, T> {
    #[inline]
    fn drop(&mut self) {
        self.mutex.is_locked.store(false, Ordering::Release);
    }
}

#[derive(Debug)]
pub struct MutexPtr<T: ?Sized> {
    is_locked: AtomicBool,
    ptr: *mut T,
}

impl<T> MutexPtr<T> {
    pub const unsafe fn new(ptr: *mut T) -> Self {
        Self {
            is_locked: AtomicBool::new(false),
            ptr,
        }
    }
}

impl<T: ?Sized> MutexPtr<T> {
    /// Non-blocking attempt to acquire the lock.
    /// Returns `Some(MutexGuardPtr)` on success, `None` on failure.
    #[inline]
    pub fn try_lock(&self) -> Option<MutexPtrGuard<'_, T>> {
        match self
            .is_locked
            .compare_exchange(false, true, Ordering::Acquire, Ordering::Relaxed)
        {
            Ok(_) => Some(MutexPtrGuard {
                is_locked: &self.is_locked,
                ptr: self.ptr,
            }),
            Err(_) => None,
        }
    }
}

unsafe impl<T: ?Sized + Send> Send for MutexPtr<T> {}
unsafe impl<T: ?Sized + Send> Sync for MutexPtr<T> {}

#[derive(Debug)]
pub struct MutexPtrGuard<'a, T: ?Sized> {
    is_locked: &'a AtomicBool,
    pub ptr: *mut T,
}

unsafe impl<T: ?Sized + Send> Send for MutexPtrGuard<'_, T> {}
unsafe impl<T: ?Sized + Sync> Sync for MutexPtrGuard<'_, T> {}

impl<T: ?Sized> Drop for MutexPtrGuard<'_, T> {
    #[inline]
    fn drop(&mut self) {
        self.is_locked.store(false, Ordering::Release);
    }
}
