#![no_std]
#![no_main]

use core::fmt::Write;

mod boot;
mod io;
mod d_log;
mod start;

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo)->!{
    loop {}
}

pub fn kmain() -> !{
    let mut logger = d_log::DebugLog::default();

    logger.write_str("hello kernel!\n");


    loop{}
}
