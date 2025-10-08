#![allow(dead_code)]

use crate::arch;
use crate::io::inb;
use crate::io::outb;

#[derive(Hash, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Debug)]
pub struct Writer {
    pub io_base: u16,
}

impl Writer {
    pub unsafe fn init(self) {
        unsafe {
            // Disable interrupts
            outb(self.io_base + 1, 0x00);

            // Enable DLAB to set baud rate divisor
            // LCR: bit7 = DLAB
            outb(self.io_base + 3, 0x80);

            // Divisor low byte (38400 baud) and high byte
            outb(self.io_base + 0, 0x03);
            outb(self.io_base + 1, 0x00);

            // Clear DLAB, set 8 data bits, no parity, 1 stop bit
            outb(self.io_base + 3, 0x03);

            // Enable FIFO, clear queues, set 14-byte threshold
            // FCR (write): bit0 = enable, bit1 = clear RX, bit2 = clear TX,
            // bits6-7 = trigger
            outb(self.io_base + 2, 0xC7);

            // Set modem control: assert DTR+RTS and set OUT2 so UART can raise IRQ to PIC
            // MCR bits: bit0=DTR, bit1=RTS, bit3=OUT1, bit4=OUT2
            outb(self.io_base + 4, 0x0B);

            // Enable interrupts
            // IER: bit0 = enable received data available interrupt
            outb(self.io_base + 1, 0x01);
        }
    }

    pub unsafe fn write_byte(self, byte: u8) {
        while !self.is_transmit_empty() {
            arch::halt();
        }

        unsafe { outb(self.io_base, byte) };
    }

    pub unsafe fn write_str(self, s: &str) -> core::fmt::Result {
        for b in s.bytes() {
            unsafe { self.write_byte(b) };
        }

        Ok(())
    }

    pub fn is_transmit_empty(self) -> bool {
        unsafe { inb(self.io_base + 5) & 0x20 != 0 }
    }
}
