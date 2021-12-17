#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case,
         non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(const_raw_ptr_to_usize_cast, register_tool)]
pub type u8_0 = libc::c_uchar;
pub type u16_0 = libc::c_ushort;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SoundParams {
    pub importance: u8_0,
    pub params: u16_0,
}
#[no_mangle]
pub static mut sEnemyBankParams: [SoundParams; 499] =
    [{
         let mut init =
             SoundParams{importance: 0x18 as libc::c_int as u8_0,
                         params: 0 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x38 as libc::c_int as u8_0,
                         params: 0x1 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x40 as libc::c_int as u8_0,
                         params: 0x1 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0x3 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x38 as libc::c_int as u8_0,
                         params: 0x3 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x40 as libc::c_int as u8_0,
                         params: 0x3 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0x2 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0x3 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0x2 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x20 as libc::c_int as u8_0,
                         params: 0x81 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0x3 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0x3 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x38 as libc::c_int as u8_0,
                         params: 0x3 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0x3 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0x3 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x38 as libc::c_int as u8_0,
                         params: 0x3 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x40 as libc::c_int as u8_0,
                         params: 0x3 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x20 as libc::c_int as u8_0,
                         params: 0x2000 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x28 as libc::c_int as u8_0,
                         params: 0x3 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x28 as libc::c_int as u8_0,
                         params: 0x3 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x20 as libc::c_int as u8_0,
                         params: 0x2 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x28 as libc::c_int as u8_0,
                         params: 0x3 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x38 as libc::c_int as u8_0,
                         params: 0x3 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0x3 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x40 as libc::c_int as u8_0,
                         params: 0x3 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0x3 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0x3 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0x3 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0x3 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0x3 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x40 as libc::c_int as u8_0,
                         params: 0x1 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x18 as libc::c_int as u8_0,
                         params: 0 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x14 as libc::c_int as u8_0,
                         params: 0 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x14 as libc::c_int as u8_0,
                         params: 0 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0x3 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x44 as libc::c_int as u8_0,
                         params: 0x3 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x18 as libc::c_int as u8_0,
                         params: 0 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0x2 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x32 as libc::c_int as u8_0,
                         params: 0x2 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x38 as libc::c_int as u8_0,
                         params: 0x1 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x20 as libc::c_int as u8_0,
                         params: 0 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x40 as libc::c_int as u8_0,
                         params: 0x1 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x18 as libc::c_int as u8_0,
                         params: 0 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x28 as libc::c_int as u8_0,
                         params: 0 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x18 as libc::c_int as u8_0,
                         params: 0 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x38 as libc::c_int as u8_0,
                         params: 0x1 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x40 as libc::c_int as u8_0,
                         params: 0x1 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x14 as libc::c_int as u8_0,
                         params: 0 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x18 as libc::c_int as u8_0,
                         params: 0x80 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x38 as libc::c_int as u8_0,
                         params: 0x2 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x28 as libc::c_int as u8_0,
                         params: 0x1 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x38 as libc::c_int as u8_0,
                         params: 0x1 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x40 as libc::c_int as u8_0,
                         params: 0x1 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x18 as libc::c_int as u8_0,
                         params: 0 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x20 as libc::c_int as u8_0,
                         params: 0 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x20 as libc::c_int as u8_0,
                         params: 0x1 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x40 as libc::c_int as u8_0,
                         params: 0x1 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x38 as libc::c_int as u8_0,
                         params: 0x1 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x40 as libc::c_int as u8_0,
                         params: 0x1 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x38 as libc::c_int as u8_0,
                         params: 0x1 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x20 as libc::c_int as u8_0,
                         params: 0 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x40 as libc::c_int as u8_0,
                         params: 0x1 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x40 as libc::c_int as u8_0,
                         params: 0x1 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0x3 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0x2000 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0x3 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0x3 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0x3 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0x3 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0x3 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0x3 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0x3 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0x3 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x18 as libc::c_int as u8_0,
                         params: 0x3 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0x3 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x28 as libc::c_int as u8_0,
                         params: 0 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x18 as libc::c_int as u8_0,
                         params: 0 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x28 as libc::c_int as u8_0,
                         params: 0 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x38 as libc::c_int as u8_0,
                         params: 0x1 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x40 as libc::c_int as u8_0,
                         params: 0x1 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x28 as libc::c_int as u8_0,
                         params: 0 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x40 as libc::c_int as u8_0,
                         params: 0x1 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x14 as libc::c_int as u8_0,
                         params: 0 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x20 as libc::c_int as u8_0,
                         params: 0 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x40 as libc::c_int as u8_0,
                         params: 0x1 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0x3 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x20 as libc::c_int as u8_0,
                         params: 0 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x38 as libc::c_int as u8_0,
                         params: 0x1 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x20 as libc::c_int as u8_0,
                         params: 0 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x38 as libc::c_int as u8_0,
                         params: 0x1 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x40 as libc::c_int as u8_0,
                         params: 0x1 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x14 as libc::c_int as u8_0,
                         params: 0 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0x3 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x20 as libc::c_int as u8_0,
                         params: 0x1 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x20 as libc::c_int as u8_0,
                         params: 0x1 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0x2 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0x2 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x38 as libc::c_int as u8_0,
                         params: 0x2 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x40 as libc::c_int as u8_0,
                         params: 0x2 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x40 as libc::c_int as u8_0,
                         params: 0x2 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x14 as libc::c_int as u8_0,
                         params: 0x81 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x34 as libc::c_int as u8_0,
                         params: 0 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x40 as libc::c_int as u8_0,
                         params: 0 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x20 as libc::c_int as u8_0,
                         params: 0 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x28 as libc::c_int as u8_0,
                         params: 0 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x28 as libc::c_int as u8_0,
                         params: 0 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x14 as libc::c_int as u8_0,
                         params: 0 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x38 as libc::c_int as u8_0,
                         params: 0x1 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x40 as libc::c_int as u8_0,
                         params: 0x1 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x20 as libc::c_int as u8_0,
                         params: 0 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0x3 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x40 as libc::c_int as u8_0,
                         params: 0x1 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x40 as libc::c_int as u8_0,
                         params: 0x3 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x40 as libc::c_int as u8_0,
                         params: 0x1 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x28 as libc::c_int as u8_0,
                         params: 0x1 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0x3 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x20 as libc::c_int as u8_0,
                         params: 0 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x38 as libc::c_int as u8_0,
                         params: 0x1 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x20 as libc::c_int as u8_0,
                         params: 0 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x20 as libc::c_int as u8_0,
                         params: 0 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x38 as libc::c_int as u8_0,
                         params: 0x2000 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0x3 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0x2000 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x14 as libc::c_int as u8_0,
                         params: 0 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x38 as libc::c_int as u8_0,
                         params: 0x1 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x40 as libc::c_int as u8_0,
                         params: 0x1 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x14 as libc::c_int as u8_0,
                         params: 0 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x20 as libc::c_int as u8_0,
                         params: 0 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x20 as libc::c_int as u8_0,
                         params: 0 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x40 as libc::c_int as u8_0,
                         params: 0x1 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x40 as libc::c_int as u8_0,
                         params: 0x1 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x34 as libc::c_int as u8_0,
                         params: 0x1 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x40 as libc::c_int as u8_0,
                         params: 0x1 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0x3 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0x2 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0x2000 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x20 as libc::c_int as u8_0,
                         params: 0x43 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x20 as libc::c_int as u8_0,
                         params: 0x2 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0x3 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0x3 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0x3 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x38 as libc::c_int as u8_0,
                         params: 0x3 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0x2000 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0x3 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0x2 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0x2000 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0x3 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x38 as libc::c_int as u8_0,
                         params: 0x3 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x40 as libc::c_int as u8_0,
                         params: 0x3 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0x3 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0x3 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0x403 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x38 as libc::c_int as u8_0,
                         params: 0x1 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0x3 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0x3 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x20 as libc::c_int as u8_0,
                         params: 0 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x34 as libc::c_int as u8_0,
                         params: 0 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x18 as libc::c_int as u8_0,
                         params: 0x1 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x20 as libc::c_int as u8_0,
                         params: 0x2000 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0x2000 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x14 as libc::c_int as u8_0,
                         params: 0x3 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x28 as libc::c_int as u8_0,
                         params: 0x3 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x28 as libc::c_int as u8_0,
                         params: 0x3 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x40 as libc::c_int as u8_0,
                         params: 0x3 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0x3 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x20 as libc::c_int as u8_0,
                         params: 0 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x14 as libc::c_int as u8_0,
                         params: 0 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x28 as libc::c_int as u8_0,
                         params: 0 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x28 as libc::c_int as u8_0,
                         params: 0 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x40 as libc::c_int as u8_0,
                         params: 0x1 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x40 as libc::c_int as u8_0,
                         params: 0x1 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x20 as libc::c_int as u8_0,
                         params: 0 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x20 as libc::c_int as u8_0,
                         params: 0 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x14 as libc::c_int as u8_0,
                         params: 0 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x40 as libc::c_int as u8_0,
                         params: 0x1 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0x3 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0x3 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0x3 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0x3 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0x3 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x38 as libc::c_int as u8_0,
                         params: 0x3 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x38 as libc::c_int as u8_0,
                         params: 0x3 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0x3 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0x3 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x40 as libc::c_int as u8_0,
                         params: 0x3 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0x3 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0x3 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x38 as libc::c_int as u8_0,
                         params: 0x1 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x28 as libc::c_int as u8_0,
                         params: 0x3 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x28 as libc::c_int as u8_0,
                         params: 0x83 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x28 as libc::c_int as u8_0,
                         params: 0x82 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0x3 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0x2000 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0x2000 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x38 as libc::c_int as u8_0,
                         params: 0x1 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x20 as libc::c_int as u8_0,
                         params: 0 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x34 as libc::c_int as u8_0,
                         params: 0 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x38 as libc::c_int as u8_0,
                         params: 0x1 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x40 as libc::c_int as u8_0,
                         params: 0x1 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x34 as libc::c_int as u8_0,
                         params: 0x2000 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x20 as libc::c_int as u8_0,
                         params: 0 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x38 as libc::c_int as u8_0,
                         params: 0 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x40 as libc::c_int as u8_0,
                         params: 0x1 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0x3 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0x2 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0x2 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x38 as libc::c_int as u8_0,
                         params: 0x3 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0x3 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x32 as libc::c_int as u8_0,
                         params: 0x3 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x34 as libc::c_int as u8_0,
                         params: 0x3 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x34 as libc::c_int as u8_0,
                         params: 0x3 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0x3 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0x3 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0x3 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x28 as libc::c_int as u8_0,
                         params: 0x82 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x40 as libc::c_int as u8_0,
                         params: 0x3 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x40 as libc::c_int as u8_0,
                         params: 0x3 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0x3 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0x3 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x20 as libc::c_int as u8_0,
                         params: 0 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x20 as libc::c_int as u8_0,
                         params: 0 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x20 as libc::c_int as u8_0,
                         params: 0x80 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0x3 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x18 as libc::c_int as u8_0,
                         params: 0x3 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x34 as libc::c_int as u8_0,
                         params: 0x3 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0x3 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x38 as libc::c_int as u8_0,
                         params: 0x3 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x18 as libc::c_int as u8_0,
                         params: 0x3 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0x2000 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x38 as libc::c_int as u8_0,
                         params: 0x3 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0x3 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x40 as libc::c_int as u8_0,
                         params: 0x3 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x40 as libc::c_int as u8_0,
                         params: 0x2000 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x38 as libc::c_int as u8_0,
                         params: 0x3 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0x2000 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0x3 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0x3 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0x3 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0x3 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0x3 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0x3 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0x2 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0x3 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0x3 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x38 as libc::c_int as u8_0,
                         params: 0x2000 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x38 as libc::c_int as u8_0,
                         params: 0x3 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x38 as libc::c_int as u8_0,
                         params: 0x3 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x38 as libc::c_int as u8_0,
                         params: 0x3 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x38 as libc::c_int as u8_0,
                         params: 0x2000 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x40 as libc::c_int as u8_0,
                         params: 0x2000 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x18 as libc::c_int as u8_0,
                         params: 0 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0x3 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0x2 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x20 as libc::c_int as u8_0,
                         params: 0x2 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x24 as libc::c_int as u8_0,
                         params: 0x3 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x28 as libc::c_int as u8_0,
                         params: 0x3 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0x3 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0x3 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0x3 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x18 as libc::c_int as u8_0,
                         params: 0x1 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x34 as libc::c_int as u8_0,
                         params: 0x3 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0x3 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0x3 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x34 as libc::c_int as u8_0,
                         params: 0x3 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0x3 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0x3 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0x3 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0x1 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x14 as libc::c_int as u8_0,
                         params: 0 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x40 as libc::c_int as u8_0,
                         params: 0x1 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0x1 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0x1 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0x1 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x38 as libc::c_int as u8_0,
                         params: 0x3 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0x3 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0x3 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x38 as libc::c_int as u8_0,
                         params: 0x3 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x38 as libc::c_int as u8_0,
                         params: 0x3 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0x2 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x38 as libc::c_int as u8_0,
                         params: 0x3 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x38 as libc::c_int as u8_0,
                         params: 0x3 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0x83 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x38 as libc::c_int as u8_0,
                         params: 0x3 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0x3 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x34 as libc::c_int as u8_0,
                         params: 0x3 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x20 as libc::c_int as u8_0,
                         params: 0x2 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x34 as libc::c_int as u8_0,
                         params: 0x3 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0x3 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x38 as libc::c_int as u8_0,
                         params: 0x3 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x20 as libc::c_int as u8_0,
                         params: 0x3 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x14 as libc::c_int as u8_0,
                         params: 0 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x20 as libc::c_int as u8_0,
                         params: 0x1 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0x3 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x40 as libc::c_int as u8_0,
                         params: 0x1 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x38 as libc::c_int as u8_0,
                         params: 0x1 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x40 as libc::c_int as u8_0,
                         params: 0x1 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x20 as libc::c_int as u8_0,
                         params: 0 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x20 as libc::c_int as u8_0,
                         params: 0 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0x3 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0x3 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0x3 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0x3 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x28 as libc::c_int as u8_0,
                         params: 0x2 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x38 as libc::c_int as u8_0,
                         params: 0x1 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x28 as libc::c_int as u8_0,
                         params: 0x2 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0x3 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0x3 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x38 as libc::c_int as u8_0,
                         params: 0x3 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x40 as libc::c_int as u8_0,
                         params: 0x3 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x14 as libc::c_int as u8_0,
                         params: 0 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x38 as libc::c_int as u8_0,
                         params: 0x1 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0x1 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x40 as libc::c_int as u8_0,
                         params: 0x1 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x28 as libc::c_int as u8_0,
                         params: 0 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x28 as libc::c_int as u8_0,
                         params: 0 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0x2 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0x3 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0x3 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0x3 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0x3 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0x2000 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x38 as libc::c_int as u8_0,
                         params: 0x3 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x38 as libc::c_int as u8_0,
                         params: 0x3 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0x3 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0x3 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x34 as libc::c_int as u8_0,
                         params: 0x3 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x38 as libc::c_int as u8_0,
                         params: 0x3 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x40 as libc::c_int as u8_0,
                         params: 0x3 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x10 as libc::c_int as u8_0,
                         params: 0 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x34 as libc::c_int as u8_0,
                         params: 0 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x18 as libc::c_int as u8_0,
                         params: 0 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x14 as libc::c_int as u8_0,
                         params: 0 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x34 as libc::c_int as u8_0,
                         params: 0 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x28 as libc::c_int as u8_0,
                         params: 0x1 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x38 as libc::c_int as u8_0,
                         params: 0x1 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x40 as libc::c_int as u8_0,
                         params: 0x1 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x38 as libc::c_int as u8_0,
                         params: 0x3 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x20 as libc::c_int as u8_0,
                         params: 0 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x20 as libc::c_int as u8_0,
                         params: 0x2 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0x2 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0x3 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0x3 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x38 as libc::c_int as u8_0,
                         params: 0x3 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0x3 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x20 as libc::c_int as u8_0,
                         params: 0x2000 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0x3 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x40 as libc::c_int as u8_0,
                         params: 0x1 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x20 as libc::c_int as u8_0,
                         params: 0 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x38 as libc::c_int as u8_0,
                         params: 0x1 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x40 as libc::c_int as u8_0,
                         params: 0x1 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0x2000 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0x3 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0x2 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0x3 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0x3 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x18 as libc::c_int as u8_0,
                         params: 0 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x28 as libc::c_int as u8_0,
                         params: 0 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x34 as libc::c_int as u8_0,
                         params: 0 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x34 as libc::c_int as u8_0,
                         params: 0 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x34 as libc::c_int as u8_0,
                         params: 0 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x38 as libc::c_int as u8_0,
                         params: 0x1 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x40 as libc::c_int as u8_0,
                         params: 0x1 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0x3 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x38 as libc::c_int as u8_0,
                         params: 0x1 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x40 as libc::c_int as u8_0,
                         params: 0x1 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x38 as libc::c_int as u8_0,
                         params: 0x3 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x40 as libc::c_int as u8_0,
                         params: 0x3 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x40 as libc::c_int as u8_0,
                         params: 0x2 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x18 as libc::c_int as u8_0,
                         params: 0 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x44 as libc::c_int as u8_0,
                         params: 0x3 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x34 as libc::c_int as u8_0,
                         params: 0 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x18 as libc::c_int as u8_0,
                         params: 0 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x38 as libc::c_int as u8_0,
                         params: 0x1 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x40 as libc::c_int as u8_0,
                         params: 0x1 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x18 as libc::c_int as u8_0,
                         params: 0x3 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0x3 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x38 as libc::c_int as u8_0,
                         params: 0x3 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x38 as libc::c_int as u8_0,
                         params: 0x3 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x40 as libc::c_int as u8_0,
                         params: 0x3 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0x3 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x36 as libc::c_int as u8_0,
                         params: 0x3 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x34 as libc::c_int as u8_0,
                         params: 0x3 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x28 as libc::c_int as u8_0,
                         params: 0x82 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0x3 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0x3 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0x3 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0x3 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0x3 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0x2000 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0x2000 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0x2000 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0x2000 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x28 as libc::c_int as u8_0,
                         params: 0x3 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0x3 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x40 as libc::c_int as u8_0,
                         params: 0x3 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0x3 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0x3 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x20 as libc::c_int as u8_0,
                         params: 0 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x20 as libc::c_int as u8_0,
                         params: 0x3 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0x3 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0x3 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0x3 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0x3 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x38 as libc::c_int as u8_0,
                         params: 0x3 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x38 as libc::c_int as u8_0,
                         params: 0x3 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x20 as libc::c_int as u8_0,
                         params: 0x3 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0x3 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x44 as libc::c_int as u8_0,
                         params: 0x3 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0x83 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0x3 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0x3 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x34 as libc::c_int as u8_0,
                         params: 0x3 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0x3 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0x3 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x20 as libc::c_int as u8_0,
                         params: 0x2 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0x3 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x20 as libc::c_int as u8_0,
                         params: 0x3 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0x2 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x40 as libc::c_int as u8_0,
                         params: 0x2 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x40 as libc::c_int as u8_0,
                         params: 0x3 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x34 as libc::c_int as u8_0,
                         params: 0x2 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0x3 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0x1 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x34 as libc::c_int as u8_0,
                         params: 0x3 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x24 as libc::c_int as u8_0,
                         params: 0x3 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x34 as libc::c_int as u8_0,
                         params: 0x1 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x20 as libc::c_int as u8_0,
                         params: 0 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0x3 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x8 as libc::c_int as u8_0,
                         params: 0x1 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0x1 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0x3 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x38 as libc::c_int as u8_0,
                         params: 0x3 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x20 as libc::c_int as u8_0,
                         params: 0x2000 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x34 as libc::c_int as u8_0,
                         params: 0x3 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x34 as libc::c_int as u8_0,
                         params: 0x2000 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x8 as libc::c_int as u8_0,
                         params: 0 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x40 as libc::c_int as u8_0,
                         params: 0x3 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x34 as libc::c_int as u8_0,
                         params: 0x3 as libc::c_int as u16_0,};
         init
     }];
#[no_mangle]
pub static mut sPlayerBankParams: [SoundParams; 224] =
    [{
         let mut init =
             SoundParams{importance: 0x20 as libc::c_int as u8_0,
                         params: 0x480 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x20 as libc::c_int as u8_0,
                         params: 0x480 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x20 as libc::c_int as u8_0,
                         params: 0x480 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x20 as libc::c_int as u8_0,
                         params: 0x480 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x20 as libc::c_int as u8_0,
                         params: 0x440 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x20 as libc::c_int as u8_0,
                         params: 0x440 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x20 as libc::c_int as u8_0,
                         params: 0x440 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x20 as libc::c_int as u8_0,
                         params: 0x440 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x20 as libc::c_int as u8_0,
                         params: 0x480 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x20 as libc::c_int as u8_0,
                         params: 0x440 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x20 as libc::c_int as u8_0,
                         params: 0x480 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x20 as libc::c_int as u8_0,
                         params: 0x400 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x20 as libc::c_int as u8_0,
                         params: 0x400 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x20 as libc::c_int as u8_0,
                         params: 0x400 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x20 as libc::c_int as u8_0,
                         params: 0x400 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x20 as libc::c_int as u8_0,
                         params: 0x400 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0x400 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0x400 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0x400 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0x400 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0x400 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0x400 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0x400 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0x400 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0x400 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0x400 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0x400 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0x400 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0x400 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0x400 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0x400 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0x400 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x40 as libc::c_int as u8_0,
                         params: 0x440 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x40 as libc::c_int as u8_0,
                         params: 0x440 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x40 as libc::c_int as u8_0,
                         params: 0x440 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x40 as libc::c_int as u8_0,
                         params: 0x440 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x40 as libc::c_int as u8_0,
                         params: 0x440 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x40 as libc::c_int as u8_0,
                         params: 0x440 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x40 as libc::c_int as u8_0,
                         params: 0x440 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x40 as libc::c_int as u8_0,
                         params: 0x440 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x40 as libc::c_int as u8_0,
                         params: 0x440 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x40 as libc::c_int as u8_0,
                         params: 0x440 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x40 as libc::c_int as u8_0,
                         params: 0x440 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x40 as libc::c_int as u8_0,
                         params: 0x440 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x40 as libc::c_int as u8_0,
                         params: 0x440 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x40 as libc::c_int as u8_0,
                         params: 0x440 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x40 as libc::c_int as u8_0,
                         params: 0x440 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x40 as libc::c_int as u8_0,
                         params: 0x440 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0x80 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0x400 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0x400 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0x400 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0x40 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0x40 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0x80 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0x400 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0x400 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x40 as libc::c_int as u8_0,
                         params: 0x400 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0x400 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0x402 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0x400 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0x40 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0x40 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0x40 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0x40 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0x40 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0x40 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0x40 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0x40 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0x40 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0x40 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0x40 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0x40 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x80 as libc::c_int as u8_0,
                         params: 0 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x80 as libc::c_int as u8_0,
                         params: 0 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x80 as libc::c_int as u8_0,
                         params: 0 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x80 as libc::c_int as u8_0,
                         params: 0 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x80 as libc::c_int as u8_0,
                         params: 0 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x80 as libc::c_int as u8_0,
                         params: 0 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x80 as libc::c_int as u8_0,
                         params: 0 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x80 as libc::c_int as u8_0,
                         params: 0 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x80 as libc::c_int as u8_0,
                         params: 0 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x80 as libc::c_int as u8_0,
                         params: 0 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x80 as libc::c_int as u8_0,
                         params: 0 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x80 as libc::c_int as u8_0,
                         params: 0 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x80 as libc::c_int as u8_0,
                         params: 0 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x80 as libc::c_int as u8_0,
                         params: 0 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x80 as libc::c_int as u8_0,
                         params: 0 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x80 as libc::c_int as u8_0,
                         params: 0 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0x400 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0x400 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0x3 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0x80 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0x3 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0x1 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0x2000 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0xc00 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0x400 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0x400 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0x400 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x20 as libc::c_int as u8_0,
                         params: 0x80 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x20 as libc::c_int as u8_0,
                         params: 0x80 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x20 as libc::c_int as u8_0,
                         params: 0x80 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x20 as libc::c_int as u8_0,
                         params: 0x80 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x20 as libc::c_int as u8_0,
                         params: 0x40 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x20 as libc::c_int as u8_0,
                         params: 0x40 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x20 as libc::c_int as u8_0,
                         params: 0x40 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x20 as libc::c_int as u8_0,
                         params: 0x40 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x20 as libc::c_int as u8_0,
                         params: 0x80 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x20 as libc::c_int as u8_0,
                         params: 0x80 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x20 as libc::c_int as u8_0,
                         params: 0x80 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x20 as libc::c_int as u8_0,
                         params: 0 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x20 as libc::c_int as u8_0,
                         params: 0 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x20 as libc::c_int as u8_0,
                         params: 0 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x20 as libc::c_int as u8_0,
                         params: 0 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x20 as libc::c_int as u8_0,
                         params: 0 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x40 as libc::c_int as u8_0,
                         params: 0 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x40 as libc::c_int as u8_0,
                         params: 0 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x40 as libc::c_int as u8_0,
                         params: 0 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x40 as libc::c_int as u8_0,
                         params: 0 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x40 as libc::c_int as u8_0,
                         params: 0 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x40 as libc::c_int as u8_0,
                         params: 0 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x40 as libc::c_int as u8_0,
                         params: 0 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x40 as libc::c_int as u8_0,
                         params: 0 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x40 as libc::c_int as u8_0,
                         params: 0 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x40 as libc::c_int as u8_0,
                         params: 0 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x40 as libc::c_int as u8_0,
                         params: 0 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x40 as libc::c_int as u8_0,
                         params: 0 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x40 as libc::c_int as u8_0,
                         params: 0 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x40 as libc::c_int as u8_0,
                         params: 0 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x40 as libc::c_int as u8_0,
                         params: 0 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x40 as libc::c_int as u8_0,
                         params: 0 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0x440 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0x440 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0x440 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0x440 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0x440 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0x440 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0x440 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0x440 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0x440 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0x440 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0x440 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0x440 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0x440 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0x440 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0x440 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0x440 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0xc00 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0x80 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x60 as libc::c_int as u8_0,
                         params: 0x2 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0x800 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0 as libc::c_int as u16_0,};
         init
     }];
#[no_mangle]
pub static mut sItemBankParams: [SoundParams; 80] =
    [{
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0x8040 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0x40 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0x440 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0x440 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x60 as libc::c_int as u8_0,
                         params: 0x83 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0x440 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x80 as libc::c_int as u8_0,
                         params: 0x43 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0x40 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0x400 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0x401 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x50 as libc::c_int as u8_0,
                         params: 0 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x90 as libc::c_int as u8_0,
                         params: 0x2 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x50 as libc::c_int as u8_0,
                         params: 0x2 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0x400 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x40 as libc::c_int as u8_0,
                         params: 0x2 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0x40 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x34 as libc::c_int as u8_0,
                         params: 0 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x40 as libc::c_int as u8_0,
                         params: 0 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0x80 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0x40 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0x400 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x20 as libc::c_int as u8_0,
                         params: 0x400 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0x400 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0x400 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x60 as libc::c_int as u8_0,
                         params: 0x43 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0x1 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0x401 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0xa0 as libc::c_int as u8_0,
                         params: 0x2 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0xa0 as libc::c_int as u8_0,
                         params: 0x2 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0x400 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x60 as libc::c_int as u8_0,
                         params: 0 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x60 as libc::c_int as u8_0,
                         params: 0 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x60 as libc::c_int as u8_0,
                         params: 0 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0x400 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x60 as libc::c_int as u8_0,
                         params: 0x81 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x60 as libc::c_int as u8_0,
                         params: 0x8003 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x60 as libc::c_int as u8_0,
                         params: 0x8003 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x60 as libc::c_int as u8_0,
                         params: 0x8003 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0x4000 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0x4000 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0x40 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x80 as libc::c_int as u8_0,
                         params: 0x3 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x80 as libc::c_int as u8_0,
                         params: 0x3 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0x3 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0x40 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x80 as libc::c_int as u8_0,
                         params: 0 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0 as libc::c_int as u16_0,};
         init
     }];
#[no_mangle]
pub static mut sEnvBankParams: [SoundParams; 248] =
    [{
         let mut init =
             SoundParams{importance: 0x70 as libc::c_int as u8_0,
                         params: 0x640 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x80 as libc::c_int as u8_0,
                         params: 0x40 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0x40 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0x40 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x40 as libc::c_int as u8_0,
                         params: 0x40 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0x480 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x38 as libc::c_int as u8_0,
                         params: 0x2 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0x40 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0x40 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x80 as libc::c_int as u8_0,
                         params: 0x2 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0xa0 as libc::c_int as u8_0,
                         params: 0x3 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0x3 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0x3 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0x3 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0x3 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0x2 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0x40 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0x40 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x60 as libc::c_int as u8_0,
                         params: 0 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0x3 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0x82 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x40 as libc::c_int as u8_0,
                         params: 0 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x38 as libc::c_int as u8_0,
                         params: 0 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x28 as libc::c_int as u8_0,
                         params: 0 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x60 as libc::c_int as u8_0,
                         params: 0 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x70 as libc::c_int as u8_0,
                         params: 0x3 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0x3 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0xa0 as libc::c_int as u8_0,
                         params: 0x2008 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x20 as libc::c_int as u8_0,
                         params: 0x2 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0x800 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0x8800 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0x8000 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0x2 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0x400 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0x80 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x60 as libc::c_int as u8_0,
                         params: 0x42 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x10 as libc::c_int as u8_0,
                         params: 0 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0xa0 as libc::c_int as u8_0,
                         params: 0x3 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0x10 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0x3 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0x2 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0xa0 as libc::c_int as u8_0,
                         params: 0x3 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0x400 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0x400 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x70 as libc::c_int as u8_0,
                         params: 0x13 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x60 as libc::c_int as u8_0,
                         params: 0x8000 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0x8000 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0x2003 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0x2010 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0x1 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x40 as libc::c_int as u8_0,
                         params: 0 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0xc2 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x70 as libc::c_int as u8_0,
                         params: 0x2 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x60 as libc::c_int as u8_0,
                         params: 0x2 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x60 as libc::c_int as u8_0,
                         params: 0x1 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0x2 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x90 as libc::c_int as u8_0,
                         params: 0x3 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x90 as libc::c_int as u8_0,
                         params: 0x3 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0x2 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0x3800 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0x3 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0x3 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0x803 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0x3 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0x2 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0x2 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0x2 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0x2 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x40 as libc::c_int as u8_0,
                         params: 0 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x1c as libc::c_int as u8_0,
                         params: 0 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x60 as libc::c_int as u8_0,
                         params: 0x200 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0x800 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0x3 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0x3 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0x3 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0x2000 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0x2000 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0x3 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0x80 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0x800 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0x800 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0x2 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0x3 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x20 as libc::c_int as u8_0,
                         params: 0x3 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0x8000 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0x2 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0x3 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x60 as libc::c_int as u8_0,
                         params: 0x3 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0x80 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0x2000 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0x1 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0x2 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0x3 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0xa0 as libc::c_int as u8_0,
                         params: 0x3 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0xc0 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0x2 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0x2 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0x3 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0x3 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0x4083 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x80 as libc::c_int as u8_0,
                         params: 0 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x60 as libc::c_int as u8_0,
                         params: 0 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x90 as libc::c_int as u8_0,
                         params: 0 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x60 as libc::c_int as u8_0,
                         params: 0xc3 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0x3 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0x3 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0xa0 as libc::c_int as u8_0,
                         params: 0x800 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0x2 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0x3 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x20 as libc::c_int as u8_0,
                         params: 0 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0 as libc::c_int as u16_0,};
         init
     }];
#[no_mangle]
pub static mut sSystemBankParams: [SoundParams; 72] =
    [{
         let mut init =
             SoundParams{importance: 0xc0 as libc::c_int as u8_0,
                         params: 0 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0xc0 as libc::c_int as u8_0,
                         params: 0 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0xb0 as libc::c_int as u8_0,
                         params: 0x20 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x50 as libc::c_int as u8_0,
                         params: 0 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0x20 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x20 as libc::c_int as u8_0,
                         params: 0 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x28 as libc::c_int as u8_0,
                         params: 0 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x18 as libc::c_int as u8_0,
                         params: 0 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x2c as libc::c_int as u8_0,
                         params: 0 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x2c as libc::c_int as u8_0,
                         params: 0 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x20 as libc::c_int as u8_0,
                         params: 0 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x20 as libc::c_int as u8_0,
                         params: 0 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x60 as libc::c_int as u8_0,
                         params: 0 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0 as libc::c_int as u16_0,};
         init
     }];
#[no_mangle]
pub static mut sOcarinaBankParams: [SoundParams; 8] =
    [{
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0x20 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0x642 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0 as libc::c_int as u16_0,};
         init
     }];
#[no_mangle]
pub static mut sVoiceBankParams: [SoundParams; 128] =
    [{
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0x402 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0x402 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0x402 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x20 as libc::c_int as u8_0,
                         params: 0x402 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0x402 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0x402 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0x402 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0x402 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0x402 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0x442 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0x442 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0x402 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0x402 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x50 as libc::c_int as u8_0,
                         params: 0x402 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0x402 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0x402 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0x402 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0x402 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0x402 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0x402 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0x402 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0x402 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0x402 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0x402 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0x482 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0x402 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0x402 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0x402 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0x402 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x80 as libc::c_int as u8_0,
                         params: 0x402 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0x402 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0x402 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0x402 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0x402 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0x402 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x20 as libc::c_int as u8_0,
                         params: 0x402 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0x402 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0x402 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0x402 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0x402 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0x402 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0x442 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0x442 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0x402 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0x402 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x50 as libc::c_int as u8_0,
                         params: 0x402 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0x402 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0x402 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0x402 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0x402 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0x402 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0x402 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0x402 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0x402 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0x402 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0x402 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0x481 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0x402 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0x402 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0x402 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0x402 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0x402 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0x402 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0x402 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x60 as libc::c_int as u8_0,
                         params: 0x20 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0x20 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0x20 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x60 as libc::c_int as u8_0,
                         params: 0 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0x8041 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0x8041 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0x8041 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0x8041 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0x8041 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0x8041 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0x8041 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0x8041 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0x8041 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0x8041 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0x8041 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0x8041 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0x8041 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0x8041 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0x8041 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0x8041 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0x8041 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0x8041 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0x8041 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0x8041 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0x8041 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0x8041 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0x8041 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0x8041 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0x8041 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0x8041 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0x8041 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0x8041 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0x8041 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0x8041 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0x8041 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0x8041 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0x8041 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0x8041 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0x8041 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0x8041 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0x8041 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0x8043 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0x8043 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0x8043 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0x8041 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0x8041 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0x8041 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0x8041 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0x8041 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0x8041 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0x8041 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             SoundParams{importance: 0x30 as libc::c_int as u8_0,
                         params: 0x8041 as libc::c_int as u16_0,};
         init
     }];
#[no_mangle]
pub static mut gSoundParams: [*mut SoundParams; 7] =
    unsafe {
        [sPlayerBankParams.as_ptr() as *mut _,
         sItemBankParams.as_ptr() as *mut _,
         sEnvBankParams.as_ptr() as *mut _,
         sEnemyBankParams.as_ptr() as *mut _,
         sSystemBankParams.as_ptr() as *mut _,
         sOcarinaBankParams.as_ptr() as *mut _,
         sVoiceBankParams.as_ptr() as *mut _]
    };
