#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case,
         non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(register_tool)]
pub type s64 = libc::c_longlong;
pub type u64_0 = libc::c_ulonglong;
pub type f32_0 = libc::c_float;
pub type f64_0 = libc::c_double;
#[no_mangle]
pub unsafe extern "C" fn __d_to_ll(mut d: f64_0) -> s64 { return d as s64; }
#[no_mangle]
pub unsafe extern "C" fn __f_to_ll(mut f: f32_0) -> s64 { return f as s64; }
#[no_mangle]
pub unsafe extern "C" fn __d_to_ull(mut d: f64_0) -> u64_0 {
    return d as u64_0;
}
#[no_mangle]
pub unsafe extern "C" fn __f_to_ull(mut f: f32_0) -> u64_0 {
    return f as u64_0;
}
#[no_mangle]
pub unsafe extern "C" fn __ll_to_d(mut l: s64) -> f64_0 { return l as f64_0; }
#[no_mangle]
pub unsafe extern "C" fn __ll_to_f(mut l: s64) -> f32_0 { return l as f32_0; }
#[no_mangle]
pub unsafe extern "C" fn __ull_to_d(mut l: u64_0) -> f64_0 {
    return l as f64_0;
}
#[no_mangle]
pub unsafe extern "C" fn __ull_to_f(mut l: u64_0) -> f32_0 {
    return l as f32_0;
}
