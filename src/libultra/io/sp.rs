#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case,
         non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(register_tool)]
pub type u32_0 = libc::c_uint;
#[no_mangle]
pub unsafe extern "C" fn __osSpDeviceBusy() -> u32_0 {
    let mut status: u32_0 =
        *((0x4040010 as libc::c_int as libc::c_uint |
               0xa0000000 as libc::c_uint) as *mut u32_0);
    if status &
           (0x4 as libc::c_int | 0x8 as libc::c_int | 0x10 as libc::c_int) as
               libc::c_uint != 0 {
        return 1 as libc::c_int as u32_0
    }
    return 0 as libc::c_int as u32_0;
}
