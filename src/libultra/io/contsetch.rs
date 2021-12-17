#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case,
         non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(register_tool)]
extern "C" {
    #[no_mangle]
    fn __osSiGetAccess();
    #[no_mangle]
    fn __osSiRelAccess();
    #[no_mangle]
    static mut __osContLastPoll: u8_0;
    #[no_mangle]
    static mut __osMaxControllers: u8_0;
}
pub type u8_0 = libc::c_uchar;
pub type s32 = libc::c_int;
/*
 * s32 osContSetCh(u8 ch)
 * This function specifies the number of devices for the functions to access when those functions access to multiple
 * direct SI devices.
 */
#[no_mangle]
pub unsafe extern "C" fn osContSetCh(mut ch: u8_0) -> s32 {
    __osSiGetAccess();
    if ch as libc::c_int > 4 as libc::c_int {
        __osMaxControllers = 4 as libc::c_int as u8_0
    } else { __osMaxControllers = ch }
    __osContLastPoll = -(2 as libc::c_int) as u8_0;
    __osSiRelAccess();
    return 0 as libc::c_int;
}
