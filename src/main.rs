#![no_std]
#![no_main]

use core::fmt::Write;
mod boot;
mod start;

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo)->!{
    loop {}
}

pub fn kmain() -> !{
    use boot::multiboot_v1::log::*;
    let mut vga = VGALog::initialize(VgaColor::LightGrey, VgaColor::Black);

    write!(vga, "hello kernel").unwrap();

    loop{}
}
