#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case,
         non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(register_tool)]
extern "C" {
    #[no_mangle]
    fn sins(_: u16_0) -> s16;
    #[no_mangle]
    fn coss(_: u16_0) -> s16;
}
pub type s16 = libc::c_short;
pub type u16_0 = libc::c_ushort;
pub type s32 = libc::c_int;
pub type u32_0 = libc::c_uint;
pub type f32_0 = libc::c_float;
#[no_mangle]
pub static mut sFactorialTbl: [f32_0; 13] =
    [1.0f32, 1.0f32, 2.0f32, 6.0f32, 24.0f32, 120.0f32, 720.0f32, 5040.0f32,
     40320.0f32, 362880.0f32, 3628800.0f32, 39916800.0f32, 479001600.0f32];
#[no_mangle]
pub unsafe extern "C" fn Math_FactorialF(mut n: f32_0) -> f32_0 {
    let mut ret: f32_0 = 1.0f32;
    let mut i: s32 = 0;
    i = n as s32;
    while i > 1 as libc::c_int { ret *= i as libc::c_float; i -= 1 }
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn Math_Factorial(mut n: s32) -> f32_0 {
    let mut ret: f32_0 = 0.;
    let mut i: s32 = 0;
    if n as u32_0 > 12 as libc::c_uint {
        ret = sFactorialTbl[12 as libc::c_int as usize];
        i = 13 as libc::c_int;
        while i <= n { ret *= i as libc::c_float; i += 1 }
    } else { ret = sFactorialTbl[n as usize] }
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn Math_PowF(mut base: f32_0, mut exp: s32) -> f32_0 {
    let mut ret: f32_0 = 1.0f32;
    while exp > 0 as libc::c_int { exp -= 1; ret *= base }
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn Math_SinF(mut angle: f32_0) -> f32_0 {
    return sins((angle * (32767.0f32 / 3.14159265358979323846f32)) as s16 as
                    u16_0) as libc::c_int as libc::c_float *
               (1.0f32 / 32767.0f32);
}
#[no_mangle]
pub unsafe extern "C" fn Math_CosF(mut angle: f32_0) -> f32_0 {
    return coss((angle * (32767.0f32 / 3.14159265358979323846f32)) as s16 as
                    u16_0) as libc::c_int as libc::c_float *
               (1.0f32 / 32767.0f32);
}
