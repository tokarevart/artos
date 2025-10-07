#![allow(clippy::identity_op)]
#![no_std]
#![no_main]
#![feature(negative_impls)]
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "test_main"]

#[derive(Hash, PartialOrd, Ord, PartialEq, Eq, Clone, Copy, Debug)]
#[repr(u32)]
pub enum QemuExitCode {
    Success = 0x10,
    Failure = 0x11,
}

pub fn exit_qemu(exit_code: QemuExitCode) {
    let port = 0xf4;
    unsafe {
        outl(port, exit_code as u32);
    }
}

#[cfg(test)]
pub fn test_runner(tests: &[&dyn Fn()]) {
    com1_println!("Running {} tests", tests.len());

    for test in tests {
        test();
    }

    exit_qemu(QemuExitCode::Success);
}

mod arch;
mod drivers;
mod io;
mod sync;

use crate::drivers::uart_16550;
use crate::drivers::vga;
use crate::io::outl;

#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    _ = com1_println!("{info}");
    loop {}
}

#[unsafe(no_mangle)]
extern "C" fn _start() -> ! {
    com1_println!("Hello COM1!");

    vga_println!("Hello VGA!");
    vga_println!();
    vga_println!("Btw, {}", 42);

    #[cfg(test)]
    test_main();

    loop {
        arch::halt();
    }
}

#[test_case]
fn trivial_assertion() {
    com1_print!("trivial assertion... ");
    assert_eq!(1, 1);
    com1_println!("[ok]");
}
