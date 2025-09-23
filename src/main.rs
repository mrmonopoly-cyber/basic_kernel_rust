#![no_std]
#![no_main]

mod boot;

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo)->!{
    loop {}
}

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> !{
    loop{}
}
