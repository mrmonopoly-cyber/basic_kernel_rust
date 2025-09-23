#![allow(unused)]
#[repr(align(16),C)]
pub struct MultiBootV1{
    magic : u32,
    flags : u32,
    checksum : u32,
}

const V_ALIGN : u32 = 1<<0;
const V_MEMINFO : u32 = 1<<1;
const V_FLAGS : u32 = V_ALIGN | V_MEMINFO;
const V_MAGIC : u32 = 0x1BADB002;
const V_CHECKSUM : u32 = (V_MAGIC + V_FLAGS).wrapping_neg();

#[unsafe(link_section = ".multiboot")]
#[used]
pub static MULTIBOOT_HEADER : MultiBootV1 = MultiBootV1{
    magic : V_MAGIC,
    flags : V_FLAGS,
    checksum : V_CHECKSUM,
};
