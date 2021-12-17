#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case,
         non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(register_tool)]
pub type u32_0 = libc::c_uint;
pub type vu32 = u32_0;
#[no_mangle]
pub unsafe extern "C" fn osDpSetStatus(mut status: u32_0) {
    let ref mut fresh0 =
        *((0xa4100000 as
               libc::c_uint).wrapping_add(0xc as libc::c_int as libc::c_uint)
              as *mut vu32);
    ::std::ptr::write_volatile(fresh0, status);
}
