pub mod log;

#[allow(unused,clippy::upper_case_acronyms)]
#[repr(u32)]
enum ArchTypes{
    I386ProtectedMode =0,
    MIPS = 4,
    
}

#[allow(unused)]
#[repr(align(64),C)]
pub struct MultiBootV2<const N:usize>{
    magic: u32,
    arch: u32,
    header_length: u32,
    checksum: u32,
    tags: [MultiBootV2Tag;N]
}

#[allow(unused)]
#[repr(align(64),C)]
pub struct MultiBootV2Tag{
    t_type: u16,
    t_flags: u16,
    t_size: u32,
}

const END: MultiBootV2Tag = MultiBootV2Tag{t_type:0,t_flags:0,t_size:8};
const MAGIC_V2: u32 = 0xE85250D6;
const TAG_NUM: usize = 1;

#[unsafe(link_section = ".multiboot")]
#[used]
pub static MULTIBOOT_HEADER: MultiBootV2<TAG_NUM>= MultiBootV2{
    magic: MAGIC_V2,
    arch: ArchTypes::I386ProtectedMode as u32,
    header_length: core::mem::size_of::<MultiBootV2<TAG_NUM>>() as u32,
    checksum: (MAGIC_V2 + ArchTypes::I386ProtectedMode as u32).wrapping_neg(),
    tags : [END]
};
