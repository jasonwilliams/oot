#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case,
         non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(register_tool)]
extern "C" {
    #[no_mangle]
    fn __osSpSetStatus(status: u32_0);
}
pub type u32_0 = libc::c_uint;
#[no_mangle]
pub unsafe extern "C" fn osSpTaskYield() {
    __osSpSetStatus(0x400 as libc::c_int as u32_0);
}
