#![no_std]
#![no_main]

mod boot;
mod start;

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo)->!{
    loop {}
}

pub fn kmain() -> !{
    loop{}
}
