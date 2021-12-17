#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case,
         non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(register_tool)]
extern "C" {
    #[no_mangle]
    fn __osSpSetPc(pc: *mut libc::c_void) -> s32;
}
pub type s32 = libc::c_int;
#[no_mangle]
pub unsafe extern "C" fn osAfterPreNMI() -> s32 {
    return __osSpSetPc(0 as *mut libc::c_void);
}
