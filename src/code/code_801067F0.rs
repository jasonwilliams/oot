#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case,
         non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(register_tool)]
pub type s32 = libc::c_int;
pub type f32_0 = libc::c_float;
// fmodf?
#[no_mangle]
pub unsafe extern "C" fn func_801067F0(mut arg0: f32_0, mut arg1: f32_0)
 -> f32_0 {
    let mut sp4: s32 = 0;
    if arg1 == 0.0f32 { return 0.0f32 }
    sp4 = (arg0 / arg1) as s32;
    return arg0 - sp4 as libc::c_float * arg1;
}
