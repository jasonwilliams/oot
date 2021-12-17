#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case,
         non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(register_tool)]
extern "C" {
    #[no_mangle]
    fn osGetCount() -> u32_0;
    #[no_mangle]
    fn __osDisableInt() -> s32;
    #[no_mangle]
    fn __osRestoreInt(_: s32);
    #[no_mangle]
    static mut __osBaseCounter: u32_0;
    #[no_mangle]
    static mut __osCurrentTime: OSTime;
}
pub type s32 = libc::c_int;
pub type u32_0 = libc::c_uint;
pub type u64_0 = libc::c_ulonglong;
pub type OSTime = u64_0;
#[no_mangle]
pub unsafe extern "C" fn osGetTime() -> OSTime {
    let mut count: u32_0 = 0;
    let mut base: u32_0 = 0;
    let mut t: OSTime = 0;
    let mut prevInt: u32_0 = __osDisableInt() as u32_0;
    count = osGetCount();
    base = count.wrapping_sub(__osBaseCounter);
    t = __osCurrentTime;
    __osRestoreInt(prevInt as s32);
    return (base as libc::c_ulonglong).wrapping_add(t);
}
