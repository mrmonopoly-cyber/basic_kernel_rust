#![no_std]
#![no_main]

mod boot;
mod start;

use::log::info;

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo)->!{
    loop {}
}

pub fn kmain() -> !{
    use boot::multiboot_v1::log::*;
    let _vga = VGALog::initialize(VgaColor::LightGrey, VgaColor::Black);

    _vga.write_string("hello");

    loop{}
}
