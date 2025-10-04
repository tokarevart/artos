#![no_std]
#![no_main]
#![allow(clippy::identity_op)]

mod cpu;
mod drivers;
mod io;

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

    let mut vga_writer = vga::Writer {
        position: 0,
        color_code: vga::ColorCode::new(vga::Color::Yellow, vga::Color::Black),
        buffer: vga::buffer(),
    };

    vga_writer
        .write_str("Hello VGA!\n\nYou're so pretty today!")
        .unwrap();

    loop {
        cpu::halt();
    }
}
