#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case,
         non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(register_tool)]
extern "C" {
    #[no_mangle]
    fn fabsf(f: f32_0) -> f32_0;
}
pub type f32_0 = libc::c_float;
#[no_mangle]
pub unsafe extern "C" fn absf(mut a: f32_0) -> f32_0 { return fabsf(a); }
