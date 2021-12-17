#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case,
         non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(register_tool)]
pub type u8_0 = libc::c_uchar;
pub type s32 = libc::c_int;
pub type size_t = libc::c_ulong;
// memmove used in __osMalloc.c
#[no_mangle]
pub unsafe extern "C" fn func_801068B0(mut dst: *mut libc::c_void,
                                       mut src: *mut libc::c_void,
                                       mut size: size_t)
 -> *mut libc::c_void {
    let mut spC: *mut u8_0 = dst as *mut u8_0;
    let mut sp8: *mut u8_0 = src as *mut u8_0;
    let mut a3: s32 = 0;
    if spC == sp8 { return dst }
    if spC < sp8 {
        let fresh0 = size;
        size = size.wrapping_sub(1);
        a3 = fresh0 as s32;
        while a3 != 0 as libc::c_int {
            let fresh1 = sp8;
            sp8 = sp8.offset(1);
            let fresh2 = spC;
            spC = spC.offset(1);
            *fresh2 = *fresh1;
            let fresh3 = size;
            size = size.wrapping_sub(1);
            a3 = fresh3 as s32
        }
    } else {
        spC =
            spC.offset(size.wrapping_sub(1 as libc::c_int as libc::c_ulong) as
                           isize);
        sp8 =
            sp8.offset(size.wrapping_sub(1 as libc::c_int as libc::c_ulong) as
                           isize);
        let fresh4 = size;
        size = size.wrapping_sub(1);
        a3 = fresh4 as s32;
        while a3 != 0 as libc::c_int {
            let fresh5 = sp8;
            sp8 = sp8.offset(-1);
            let fresh6 = spC;
            spC = spC.offset(-1);
            *fresh6 = *fresh5;
            let fresh7 = size;
            size = size.wrapping_sub(1);
            a3 = fresh7 as s32
        }
    }
    return dst;
}
