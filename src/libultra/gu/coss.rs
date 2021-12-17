#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case,
         non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(register_tool)]
extern "C" {
    #[no_mangle]
    fn sins(_: u16_0) -> s16;
}
pub type s16 = libc::c_short;
pub type u16_0 = libc::c_ushort;
#[no_mangle]
pub unsafe extern "C" fn coss(mut angle: u16_0) -> s16 {
    return sins((angle as libc::c_int + 0x4000 as libc::c_int) as u16_0);
}
