#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case,
         non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(ptr_wrapping_offset_from, register_tool)]
pub type u8_0 = libc::c_uchar;
pub type s32 = libc::c_int;
pub type u32_0 = libc::c_uint;
#[no_mangle]
pub unsafe extern "C" fn strchr(mut str: *const libc::c_char, mut ch: s32)
 -> *const libc::c_char {
    let mut c: u8_0 = ch as u8_0;
    while *str as libc::c_int != c as libc::c_int {
        if *str as libc::c_int == 0 as libc::c_int {
            return 0 as *const libc::c_char
        }
        str = str.offset(1)
    }
    return str;
}
#[no_mangle]
pub unsafe extern "C" fn strlen(mut str: *const libc::c_char) -> u32_0 {
    let mut ptr: *const libc::c_char = str;
    while *ptr != 0 { ptr = ptr.offset(1) }
    return ptr.wrapping_offset_from(str) as libc::c_int as u32_0;
}
#[no_mangle]
pub unsafe extern "C" fn memcpy(mut dst: *mut libc::c_void,
                                mut src: *const libc::c_void, mut size: u32_0)
 -> *mut libc::c_void {
    let mut _dst: *mut u8_0 = dst as *mut u8_0;
    let mut _src: *const u8_0 = src as *const u8_0;
    while size > 0 as libc::c_int as libc::c_uint {
        let fresh0 = _src;
        _src = _src.offset(1);
        let fresh1 = _dst;
        _dst = _dst.offset(1);
        *fresh1 = *fresh0;
        size = size.wrapping_sub(1)
    }
    return dst;
}
