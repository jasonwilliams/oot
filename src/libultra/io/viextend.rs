#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case,
         non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(register_tool)]
extern "C" {
    #[no_mangle]
    static mut __additional_scanline: u32_0;
}
pub type u32_0 = libc::c_uint;
#[no_mangle]
pub unsafe extern "C" fn osViExtendVStart(mut arg0: u32_0) {
    __additional_scanline = arg0;
}
