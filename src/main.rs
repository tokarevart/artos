#![no_std]
#![no_main]
#![allow(clippy::identity_op)]

use core::arch::asm;
use core::hint;

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}

const VGA: usize = 0xb8000;
const COM1: u16 = 0x3F8;

unsafe fn outb(port: u16, val: u8) {
    unsafe { asm!("out dx, al", in("dx") port, in("al") val) };
}

unsafe fn inb(port: u16) -> u8 {
    let mut ret: u8;
    unsafe { asm!("in al, dx", in("dx") port, out("al") ret) };
    ret
}

fn serial_init() {
    unsafe {
        // Disable interrupts
        outb(COM1 + 1, 0x00);
        // Enable DLAB (set baud rate divisor)
        outb(COM1 + 3, 0x80);
        outb(COM1 + 0, 0x03); // Divisor low byte (38400 baud)
        outb(COM1 + 1, 0x00); // Divisor high byte
        // 8 bits, no parity, one stop bit
        outb(COM1 + 3, 0x03);
        // Enable FIFO, clear them, with 14-byte threshold
        outb(COM1 + 2, 0xC7);
        // IRQs enabled, RTS/DSR set
        outb(COM1 + 4, 0x0B);
    }
}

fn serial_is_transmit_empty() -> bool {
    unsafe { inb(COM1 + 5) & 0x20 != 0 }
}

fn serial_write_byte(byte: u8) {
    while !serial_is_transmit_empty() {}
    unsafe { outb(COM1, byte) };
}

fn serial_write_string(s: &str) {
    for b in s.bytes() {
        serial_write_byte(b);
    }
}

static HELLO: &[u8] = b"Hello World!";

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    serial_init();
    serial_write_string("Hello from serial!\n");

    let vga_buffer = VGA as *mut u8;

    for (i, &byte) in HELLO.iter().enumerate() {
        unsafe {
            *vga_buffer.add(i * 2) = byte;
            *vga_buffer.add(i * 2 + 1) = 0xb;
        }
    }

    loop {
        hint::spin_loop();
    }
}
