use core::cell::UnsafeCell;
use core::mem::ManuallyDrop;
use core::mem::MaybeUninit;
use core::ops::Deref;
use core::ops::DerefMut;
use core::sync::atomic::AtomicBool;
use core::sync::atomic::Ordering;

union Data<T, F> {
    data: ManuallyDrop<T>,
    init: ManuallyDrop<F>,
}

#[derive(Debug)]
pub struct LazyLock<T, F = fn() -> T> {
    is_inited: AtomicBool,
    data: UnsafeCell<Data<T, F>>,
}

unsafe impl<T: Sync + Send, F: Send> Sync for LazyLock<T, F> {}

impl<T, F: FnOnce() -> T> LazyLock<T, F> {
    pub const fn new(f: F) -> Self {
        Self {
            is_inited: AtomicBool::new(false),
            data: UnsafeCell::new(Data {
                init: ManuallyDrop::new(f),
            }),
        }
    }

    pub fn try_init(&self) -> bool {
        if self
            .is_inited
            .compare_exchange(false, true, Ordering::Acquire, Ordering::Relaxed)
            .is_ok()
        {
            let t = unsafe { ManuallyDrop::take(&mut (*self.data.get()).init) }();
            let data = ManuallyDrop::new(t);
            unsafe { *self.data.get() = Data { data } };

            true
        } else {
            false
        }
    }
}

impl<T, F: Fn() -> T> Deref for LazyLock<T, F> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        _ = self.try_init();
        unsafe { &(*self.data.get()).data }
    }
}

impl<T, F: Fn() -> T> DerefMut for LazyLock<T, F> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        _ = self.try_init();
        unsafe { &mut (*self.data.get()).data }
    }
}
