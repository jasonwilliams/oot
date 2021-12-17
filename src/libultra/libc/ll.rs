#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case,
         non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(register_tool)]
pub type u16_0 = libc::c_ushort;
pub type s64 = libc::c_longlong;
pub type u64_0 = libc::c_ulonglong;
#[no_mangle]
pub unsafe extern "C" fn __ull_rshift(mut l: u64_0, mut r: s64) -> s64 {
    return (l >> r) as s64;
}
#[no_mangle]
pub unsafe extern "C" fn __ull_rem(mut l: u64_0, mut r: u64_0) -> u64_0 {
    return l.wrapping_rem(r);
}
#[no_mangle]
pub unsafe extern "C" fn __ull_div(mut l: u64_0, mut r: u64_0) -> u64_0 {
    return l.wrapping_div(r);
}
#[no_mangle]
pub unsafe extern "C" fn __ll_lshift(mut l: s64, mut r: s64) -> s64 {
    return l << r;
}
#[no_mangle]
pub unsafe extern "C" fn __ll_rem(mut l: s64, mut r: u64_0) -> s64 {
    return (l as libc::c_ulonglong).wrapping_rem(r) as s64;
}
#[no_mangle]
pub unsafe extern "C" fn __ll_div(mut l: s64, mut r: s64) -> s64 {
    return l / r;
}
#[no_mangle]
pub unsafe extern "C" fn __ll_mul(mut l: s64, mut r: s64) -> s64 {
    return l * r;
}
#[no_mangle]
pub unsafe extern "C" fn __ull_divremi(mut quotient: *mut u64_0,
                                       mut remainder: *mut u64_0,
                                       mut dividend: u64_0,
                                       mut divisor: u16_0) {
    *quotient = dividend.wrapping_div(divisor as libc::c_ulonglong);
    *remainder = dividend.wrapping_rem(divisor as libc::c_ulonglong);
}
#[no_mangle]
pub unsafe extern "C" fn __ll_mod(mut l: s64, mut r: s64) -> s64 {
    let mut remainder: s64 = l % r;
    if remainder < 0 as libc::c_int as libc::c_longlong &&
           r > 0 as libc::c_int as libc::c_longlong ||
           remainder > 0 as libc::c_int as libc::c_longlong &&
               r < 0 as libc::c_int as libc::c_longlong {
        remainder += r
    }
    return remainder;
}
#[no_mangle]
pub unsafe extern "C" fn __ll_rshift(mut l: s64, mut r: s64) -> s64 {
    return l >> r;
}
