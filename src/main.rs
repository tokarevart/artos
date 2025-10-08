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
    unsafe { outl(port, exit_code as u32) };
}

pub trait Testable {
    fn run(&self) -> ();
}

impl<T: Fn()> Testable for T {
    fn run(&self) {
        com1_print!("{}...\t", core::any::type_name::<T>());
        self();
        com1_println!("[ok]");
    }
}

#[cfg(test)]
pub fn test_runner(tests: &[&dyn Testable]) {
    com1_println!("Running {} tests", tests.len());

    for test in tests {
        test.run();
    }

    exit_qemu(QemuExitCode::Success);
}

mod arch;
mod drivers;
mod io;
mod sync;

use crate::drivers::vga;
use crate::io::outl;

#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    _ = com1_try_println!("{info}");

    #[cfg(test)]
    exit_qemu(QemuExitCode::Failure);

    loop {
        arch::halt();
    }
}

#[unsafe(no_mangle)]
extern "C" fn _start() -> ! {
    #[cfg(test)]
    test_main();
    #[cfg(not(test))]
    main();

    loop {
        arch::halt();
    }
}

fn main() {
    com1_println!("Hello COM1!");

    vga_println!("Hello VGA!");
    vga_println!();
    vga_println!("Btw, {}", 42);
}

#[test_case]
fn trivial_assertion() {
    assert_eq!(0, 1);
}
