#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case,
         non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(register_tool)]
pub type s32 = libc::c_int;
pub type s64 = libc::c_longlong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct lldiv_t {
    pub quot: s64,
    pub rem: s64,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ldiv_t {
    pub quot: s32,
    pub rem: s32,
}
#[no_mangle]
pub unsafe extern "C" fn ldiv(mut num: s32, mut denom: s32) -> ldiv_t {
    let mut ret: ldiv_t = ldiv_t{quot: 0, rem: 0,};
    ret.quot = num / denom;
    ret.rem = num - denom * ret.quot;
    if ret.quot < 0 as libc::c_int && ret.rem > 0 as libc::c_int {
        ret.quot += 1;
        ret.rem -= denom
    }
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn lldiv(mut num: s64, mut denom: s64) -> lldiv_t {
    let mut ret: lldiv_t = lldiv_t{quot: 0, rem: 0,};
    ret.quot = num / denom;
    ret.rem = num - denom * ret.quot;
    if ret.quot < 0 as libc::c_int as libc::c_longlong &&
           ret.rem > 0 as libc::c_int as libc::c_longlong {
        ret.quot += 1;
        ret.rem -= denom
    }
    return ret;
}
