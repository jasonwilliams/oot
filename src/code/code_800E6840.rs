#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case,
         non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(register_tool)]
extern "C" {
    #[no_mangle]
    fn osWritebackDCache(vaddr: *mut libc::c_void, nbytes: s32);
    #[no_mangle]
    fn osInvalDCache(vaddr: *mut libc::c_void, nbytes: s32);
    #[no_mangle]
    fn osSetIntMask(_: OSIntMask) -> OSIntMask;
}
pub type s32 = libc::c_int;
pub type u32_0 = libc::c_uint;
pub type OSIntMask = u32_0;
#[no_mangle]
pub unsafe extern "C" fn Audio_InvalDCache(mut buf: *mut libc::c_void,
                                           mut size: s32) {
    let mut prevMask: OSIntMask = osSetIntMask(1 as libc::c_int as OSIntMask);
    osInvalDCache(buf, size);
    osSetIntMask(prevMask);
}
#[no_mangle]
pub unsafe extern "C" fn Audio_WritebackDCache(mut buf: *mut libc::c_void,
                                               mut size: s32) {
    let mut prevMask: OSIntMask = osSetIntMask(1 as libc::c_int as OSIntMask);
    osWritebackDCache(buf, size);
    osSetIntMask(prevMask);
}
