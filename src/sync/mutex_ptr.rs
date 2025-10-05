use core::ops::Deref;
use core::ops::DerefMut;
use core::ptr::NonNull;

use crate::sync::Mutex;
use crate::sync::MutexGuard;

#[derive(Debug)]
pub struct MutexPtr<T: ?Sized>(Mutex<NonNull<T>>);

impl<T> MutexPtr<T> {
    pub const unsafe fn new(t: NonNull<T>) -> Self {
        Self(Mutex::new(t))
    }
}

impl<T: ?Sized> MutexPtr<T> {
    /// Non-blocking attempt to acquire the lock.
    /// Returns `Some(MutexGuard)` on success, `None` on failure.
    #[inline]
    pub fn try_lock(&self) -> Option<MutexPtrGuard<'_, T>> {
        self.0.try_lock().map(MutexPtrGuard)
    }
}

unsafe impl<T: ?Sized + Send> Send for MutexPtr<T> {}
unsafe impl<T: ?Sized + Send> Sync for MutexPtr<T> {}

#[derive(Debug)]
pub struct MutexPtrGuard<'a, T: ?Sized>(MutexGuard<'a, NonNull<T>>);

unsafe impl<T: ?Sized + Sync> Send for MutexPtrGuard<'_, T> {}
unsafe impl<T: ?Sized + Sync> Sync for MutexPtrGuard<'_, T> {}

impl<T: ?Sized> Deref for MutexPtrGuard<'_, T> {
    type Target = T;

    #[inline]
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0.deref().as_ptr() }
    }
}

impl<T: ?Sized> DerefMut for MutexPtrGuard<'_, T> {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut *self.0.deref().as_ptr() }
    }
}
