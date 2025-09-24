#![no_std]
#![no_main]

mod boot;
mod start;

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo)->!{
    loop {}
}

pub fn kmain() -> !{
    use boot::multiboot_v1::log::*;
    let mut vga = VGALog::initialize(VgaColor::LightGrey, VgaColor::Black);

    vga.write_string("hello kernel");

    loop{}
}
