#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case,
         non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(register_tool)]
pub type s32 = libc::c_int;
pub type u32_0 = libc::c_uint;
#[no_mangle]
pub unsafe extern "C" fn __osSiDeviceBusy() -> s32 {
    let mut status: u32_0 =
        *((0x4800018 as libc::c_int as libc::c_uint |
               0xa0000000 as libc::c_uint) as *mut u32_0);
    if status & (0x1 as libc::c_int | 0x2 as libc::c_int) as libc::c_uint != 0
       {
        return 1 as libc::c_int
    } else { return 0 as libc::c_int };
}
