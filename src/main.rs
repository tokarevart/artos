#![no_std]
#![no_main]
#![allow(clippy::identity_op)]

mod com1;

use core::hint;

use crate::com1::polling::Serial;

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}

const VGA: usize = 0xb8000;

static HELLO_VGA: &str = "Hello VGA!";
static HELLO_COM1: &str = "Hello COM1!\n";

#[unsafe(no_mangle)]
extern "C" fn _start() -> ! {
    let serial = Serial::new();
    serial.write_str(HELLO_COM1);

    let vga_buffer = VGA as *mut u8;

    for (i, byte) in HELLO_VGA.bytes().enumerate() {
        unsafe {
            *vga_buffer.add(i * 2) = byte;
            *vga_buffer.add(i * 2 + 1) = 0xb;
        }
    }

    loop {
        hint::spin_loop();
    }
}
