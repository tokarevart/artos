use crate::com1::init;
use crate::com1::is_transmit_empty;
use crate::com1::outb;

#[derive(Clone, Copy, Debug)]
pub struct Serial(());

impl Serial {
    pub fn new() -> Self {
        init();
        Self(())
    }

    pub fn write_str(self, s: &str) {
        for b in s.bytes() {
            self.write_byte(b);
        }
    }

    pub fn write_byte(self, byte: u8) {
        while !is_transmit_empty() {}
        unsafe { outb(0, byte) };
    }
}

impl Default for Serial {
    fn default() -> Self {
        Self::new()
    }
}
