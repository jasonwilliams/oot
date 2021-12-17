#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case,
         non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(register_tool)]
extern "C" {
    #[no_mangle]
    fn __osSiDeviceBusy() -> s32;
}
pub type s32 = libc::c_int;
pub type u32_0 = libc::c_uint;
#[no_mangle]
pub unsafe extern "C" fn __osSiRawReadIo(mut devAddr: *mut libc::c_void,
                                         mut dst: *mut u32_0) -> s32 {
    if __osSiDeviceBusy() != 0 { return -(1 as libc::c_int) }
    *dst = *((devAddr as u32_0 | 0xa0000000 as libc::c_uint) as *mut u32_0);
    return 0 as libc::c_int;
}
