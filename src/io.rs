#![allow(dead_code)]

use core::arch::asm;

#[inline]
pub unsafe fn inb(port: u16) -> u8 {
    let mut ret: u8;
    unsafe { asm!("in al, dx", in("dx") port, out("al") ret) };
    ret
}

#[inline]
pub unsafe fn outb(port: u16, val: u8) {
    unsafe { asm!("out dx, al", in("dx") port, in("al") val) };
}
