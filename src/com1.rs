pub mod polling;

use core::arch::asm;
use core::sync::atomic::AtomicBool;
use core::sync::atomic::Ordering;

pub const COM1: u16 = 0x3F8;

unsafe fn inb(port_offset: u16) -> u8 {
    let mut ret: u8;
    unsafe { asm!("in al, dx", in("dx") COM1 + port_offset, out("al") ret) };
    ret
}

unsafe fn outb(port_offset: u16, val: u8) {
    unsafe { asm!("out dx, al", in("dx") COM1 + port_offset, in("al") val) };
}

fn init() {
    static INITED: AtomicBool = AtomicBool::new(false);

    if !INITED.swap(true, Ordering::AcqRel) {
        unsafe {
            // Disable interrupts
            outb(1, 0x00);
            // Enable DLAB to set baud rate divisor
            outb(3, 0x80);
            outb(0, 0x03); // Divisor low byte (38400 baud)
            outb(1, 0x00); // Divisor high byte
            // 8 bits, no parity, one stop bit
            outb(3, 0x03);
            // Enable FIFO, clear them, with 14-byte threshold
            outb(2, 0xC7);
            // IRQs enabled, RTS/DSR set
            outb(4, 0x0B);
        }
    }
}

pub fn is_transmit_empty() -> bool {
    unsafe { inb(5) & 0x20 != 0 }
}
