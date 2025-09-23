macro_rules! STACK_SIZE {
    () => {
        16384
    };
}

#[unsafe(link_section = ".bss")]
#[used]
pub static STACK : [u8;STACK_SIZE!()] = [0;STACK_SIZE!()];

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> !{

    unsafe{
        ::core::arch::asm!(
            "mov {0:e}, esp",
            in(reg) (&STACK as *const u8 as usize) + STACK.len(),
            );
    };

    crate::kmain();
}
