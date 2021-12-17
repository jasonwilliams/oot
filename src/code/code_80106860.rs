#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case,
         non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(register_tool)]
pub type u8_0 = libc::c_uchar;
pub type s32 = libc::c_int;
pub type size_t = libc::c_ulong;
// memset used in __osMalloc, z_quake, z_view, and z_camera
#[no_mangle]
pub unsafe extern "C" fn func_80106860(mut ptr: *mut libc::c_void,
                                       mut val: s32, mut size: size_t)
 -> *mut libc::c_void {
    let mut sp4: *mut u8_0 = ptr as *mut u8_0;
    let mut a3: s32 = 0;
    let fresh0 = size;
    size = size.wrapping_sub(1);
    a3 = fresh0 as s32;
    while a3 != 0 as libc::c_int {
        let fresh1 = sp4;
        sp4 = sp4.offset(1);
        *fresh1 = val as u8_0;
        let fresh2 = size;
        size = size.wrapping_sub(1);
        a3 = fresh2 as s32
    }
    return ptr;
}
