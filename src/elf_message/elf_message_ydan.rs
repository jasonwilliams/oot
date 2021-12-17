#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case,
         non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(register_tool)]
pub type u8_0 = libc::c_uchar;
pub type u32_0 = libc::c_uint;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ElfMessage {
    pub byte0: u8_0,
    pub byte1: u8_0,
    pub byte2: u8_0,
    pub byte3: u8_0,
}
#[no_mangle]
pub static mut gDungeonNaviMsgs: [ElfMessage; 1] =
    [{
         let mut init =
             ElfMessage{byte0:
                            ((7 as libc::c_int as u32_0 &
                                  (((0x1 as libc::c_int) << 3 as libc::c_int)
                                       - 1 as libc::c_int) as libc::c_uint) <<
                                 5 as libc::c_int |
                                 (0 as libc::c_int as u32_0 &
                                      (((0x1 as libc::c_int) <<
                                            4 as libc::c_int) -
                                           1 as libc::c_int) as libc::c_uint)
                                     << 1 as libc::c_int |
                                 (0 as libc::c_int as u32_0 &
                                      (((0x1 as libc::c_int) <<
                                            1 as libc::c_int) -
                                           1 as libc::c_int) as libc::c_uint)
                                     << 0 as libc::c_int) as u8_0,
                        byte1:
                            ((0 as libc::c_int as u32_0 &
                                  (((0x1 as libc::c_int) << 8 as libc::c_int)
                                       - 1 as libc::c_int) as libc::c_uint) <<
                                 0 as libc::c_int) as u8_0,
                        byte2:
                            ((0x5f as libc::c_int as u32_0 &
                                  (((0x1 as libc::c_int) << 8 as libc::c_int)
                                       - 1 as libc::c_int) as libc::c_uint) <<
                                 0 as libc::c_int) as u8_0,
                        byte3:
                            ((0 as libc::c_int as u32_0 &
                                  (((0x1 as libc::c_int) << 8 as libc::c_int)
                                       - 1 as libc::c_int) as libc::c_uint) <<
                                 0 as libc::c_int) as u8_0,};
         init
     }];
