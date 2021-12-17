#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case,
         non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(register_tool)]
pub type u32_0 = libc::c_uint;
#[no_mangle]
pub unsafe extern "C" fn __osSpSetStatus(mut status: u32_0) {
    ::std::ptr::write_volatile((0x4040010 as libc::c_int as libc::c_uint |
                                    0xa0000000 as libc::c_uint) as *mut u32_0,
                               status);
}
