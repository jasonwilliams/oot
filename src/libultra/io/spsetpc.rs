#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case,
         non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(register_tool)]
pub type s32 = libc::c_int;
pub type u32_0 = libc::c_uint;
#[no_mangle]
pub unsafe extern "C" fn __osSpSetPc(mut pc: *mut libc::c_void) -> s32 {
    let mut spStatus: u32_0 =
        *((0x4040010 as libc::c_int as libc::c_uint |
               0xa0000000 as libc::c_uint) as *mut u32_0);
    if spStatus & 0x1 as libc::c_int as libc::c_uint == 0 {
        return -(1 as libc::c_int)
    } else {
        let ref mut fresh0 =
            *((0x4080000 as libc::c_int as libc::c_uint |
                   0xa0000000 as libc::c_uint) as *mut *mut libc::c_void);
        *fresh0 = pc as *mut libc::c_void
    }
    return 0 as libc::c_int;
}
