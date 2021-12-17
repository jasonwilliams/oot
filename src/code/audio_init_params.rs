#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case,
         non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(register_tool)]
pub type s16 = libc::c_short;
pub type u32_0 = libc::c_uint;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AudioContextInitSizes {
    pub heapSize: u32_0,
    pub initPoolSize: u32_0,
    pub permanentPoolSize: u32_0,
}
#[no_mangle]
pub static mut D_8014A6C0: [s16; 2] =
    [0x1c00 as libc::c_int as s16, 0x30 as libc::c_int as s16];
#[no_mangle]
pub static mut D_8014A6C4: AudioContextInitSizes =
    {
        let mut init =
            AudioContextInitSizes{heapSize: 0x37f00 as libc::c_int as u32_0,
                                  initPoolSize:
                                      0xe0e0 as libc::c_int as u32_0,
                                  permanentPoolSize:
                                      0xbce0 as libc::c_int as u32_0,};
        init
    };
