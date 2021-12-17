#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case,
         non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(register_tool)]
pub type u8_0 = libc::c_uchar;
pub type s32 = libc::c_int;
#[no_mangle]
pub unsafe extern "C" fn MemCopy(mut dest: *mut libc::c_void,
                                 mut src: *mut libc::c_void, mut size: s32)
 -> *mut libc::c_void {
    let mut destu: *mut u8_0 = dest as *mut u8_0;
    let mut srcu: *mut u8_0 = src as *mut u8_0;
    while size > 0 as libc::c_int {
        let fresh0 = srcu;
        srcu = srcu.offset(1);
        let fresh1 = destu;
        destu = destu.offset(1);
        *fresh1 = *fresh0;
        size -= 1
    }
    return dest;
}
#[no_mangle]
pub unsafe extern "C" fn MemSet(mut dest: *mut libc::c_void, mut val: s32,
                                mut size: s32) -> *mut libc::c_void {
    let mut destu: *mut u8_0 = dest as *mut u8_0;
    let mut s: s32 = size;
    while s > 0 as libc::c_int {
        let fresh2 = destu;
        destu = destu.offset(1);
        *fresh2 = val as u8_0;
        s -= 1
    }
    return dest;
}
