#![no_std]
#![no_main]
#![feature(negative_impls)]
#![allow(clippy::identity_op)]

mod arch;
mod drivers;
mod io;
mod sync;

use core::fmt::Write;

use crate::drivers::com1;
use crate::drivers::vga;

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}

#[unsafe(no_mangle)]
extern "C" fn _start() -> ! {
    let mut serial = com1::Writer::new();
    serial.write_str("Hello COM1!\n").unwrap();

    assert!(vga_println!("Hello VGA!"));
    assert!(vga_println!());
    assert!(vga_println!("Btw, {}", 42));

    loop {
        arch::halt();
    }
}
