#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case,
         non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(register_tool)]
extern "C" {
    #[no_mangle]
    fn __osDisableInt() -> s32;
    #[no_mangle]
    fn __osRestoreInt(_: s32);
    #[no_mangle]
    static mut __OSGlobalIntMask: OSHWIntr;
}
pub type s32 = libc::c_int;
pub type u32_0 = libc::c_uint;
pub type OSHWIntr = u32_0;
#[no_mangle]
pub unsafe extern "C" fn __osSetGlobalIntMask(mut mask: OSHWIntr) {
    let mut prevInt: u32_0 = __osDisableInt() as u32_0;
    __OSGlobalIntMask |= mask;
    __osRestoreInt(prevInt as s32);
}
