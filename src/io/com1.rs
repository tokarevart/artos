use core::fmt::Write;

use crate::drivers::uart_16550::Writer;
use crate::sync::LazyLock;
use crate::sync::Mutex;

pub const COM1: u16 = 0x3F8;

pub static WRITER: Mutex<LazyLock<Writer>> = Mutex::new(LazyLock::new(|| {
    let writer = Writer { io_base: COM1 };
    unsafe { writer.init() };
    writer
}));

#[macro_export]
macro_rules! com1_println {
    () => {
        $crate::com1_print!("\n")
    };
    ($($arg:tt)*) => {
        $crate::com1_print!("{}\n", ::core::format_args!($($arg)*))
    };
}

#[macro_export]
macro_rules! com1_try_println {
    () => {
        $crate::com1_try_print!("\n")
    };
    ($($arg:tt)*) => {
        $crate::com1_try_print!("{}\n", ::core::format_args!($($arg)*))
    };
}

#[macro_export]
macro_rules! com1_print {
    ($($arg:tt)*) => {
        $crate::io::com1::print(::core::format_args!($($arg)*))
    };
}

#[macro_export]
macro_rules! com1_try_print {
    ($($arg:tt)*) => {
        $crate::io::com1::try_print(::core::format_args!($($arg)*))
    };
}

pub fn print(args: core::fmt::Arguments) {
    assert!(try_print(args));
}

#[must_use]
pub fn try_print(args: core::fmt::Arguments) -> bool {
    struct WriteWrapper<'a>(&'a mut Writer);

    impl Write for WriteWrapper<'_> {
        fn write_str(&mut self, s: &str) -> core::fmt::Result {
            unsafe { self.0.write_str(s) }
        }
    }

    if let Some(mut x) = WRITER.try_lock() {
        WriteWrapper(&mut x).write_fmt(args).unwrap();
        true
    } else {
        false
    }
}
