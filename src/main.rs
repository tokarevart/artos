#![no_std]
#![no_main]
#![feature(negative_impls)]
#![allow(clippy::identity_op)]

mod arch;
mod drivers;
mod io;
mod sync;

use crate::drivers::com1;
use crate::drivers::vga;

#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    _ = com1_println!("{info}");
    loop {}
}

#[unsafe(no_mangle)]
extern "C" fn _start() -> ! {
    assert!(com1_println!("Hello COM1!"));

    assert!(vga_println!("Hello VGA!"));
    assert!(vga_println!());
    assert!(vga_println!("Btw, {}", 42));

    loop {
        arch::halt();
    }
}
