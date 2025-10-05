use core::arch::asm;

/// Halts the CPU until the next interrupt occurs.
#[inline(always)]
pub fn halt() {
    unsafe { asm!("hlt", options(nomem, nostack, preserves_flags)) }
}
