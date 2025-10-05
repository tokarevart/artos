use core::fmt::Write;
use core::sync::atomic::AtomicBool;
use core::sync::atomic::Ordering;

use crate::arch;
use crate::io::inb;
use crate::io::outb;

pub const COM1: u16 = 0x3F8;

fn init() {
    static INITED: AtomicBool = AtomicBool::new(false);

    if !INITED.swap(true, Ordering::AcqRel) {
        unsafe {
            // Disable interrupts
            outb(COM1 + 1, 0x00);
            // Enable DLAB to set baud rate divisor
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
}

pub fn is_transmit_empty() -> bool {
    unsafe { inb(COM1 + 5) & 0x20 != 0 }
}

#[derive(Clone, Copy, Debug)]
pub struct Writer(());

impl Writer {
    pub fn new() -> Self {
        init();
        Self(())
    }

    pub fn write_byte(&mut self, byte: u8) {
        while !is_transmit_empty() {
            arch::halt();
        }

        unsafe { outb(COM1 + 0, byte) };
    }
}

impl Default for Writer {
    fn default() -> Self {
        Self::new()
    }
}

impl Write for Writer {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        for b in s.bytes() {
            self.write_byte(b);
        }

        Ok(())
    }
}
