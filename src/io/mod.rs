//Write a byte on a port
pub fn putb(port:u16, byte: u8) {
    unsafe{
        ::core::arch::asm!("out dx, al",
            in("dx") port,
            in("al") byte,
            options(preserves_flags, nostack, nomem));
    }
}
