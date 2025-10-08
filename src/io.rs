#![allow(dead_code)]

pub mod com1;

use core::arch::asm;

#[inline]
pub unsafe fn inb(port: u16) -> u8 {
    let mut ret;
    unsafe {
        asm!("in al, dx", out("al") ret, in("dx") port, options(nomem, nostack, preserves_flags))
    };
    ret
}

#[inline]
pub unsafe fn outb(port: u16, val: u8) {
    unsafe {
        asm!("out dx, al", in("dx") port, in("al") val, options(nomem, nostack, preserves_flags))
    };
}

#[inline]
pub unsafe fn inw(port: u16) -> u16 {
    let mut ret;
    unsafe {
        asm!("in ax, dx", out("ax") ret, in("dx") port, options(nomem, nostack, preserves_flags))
    };
    ret
}

#[inline]
pub unsafe fn outw(port: u16, val: u16) {
    unsafe {
        asm!("out dx, ax", in("dx") port, in("ax") val, options(nomem, nostack, preserves_flags))
    };
}

#[inline]
pub unsafe fn inl(port: u16) -> u32 {
    let mut ret;
    unsafe {
        asm!("in eax, dx", out("eax") ret, in("dx") port, options(nomem, nostack, preserves_flags))
    };
    ret
}

#[inline]
pub unsafe fn outl(port: u16, val: u32) {
    unsafe {
        asm!("out dx, eax", in("dx") port, in("eax") val, options(nomem, nostack, preserves_flags))
    };
}
