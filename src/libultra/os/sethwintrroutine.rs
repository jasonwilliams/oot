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
    static mut __osHwIntTable: [__osHwInt; 0];
}
pub type s32 = libc::c_int;
pub type u32_0 = libc::c_uint;
pub type OSHWIntr = u32_0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __osHwInt {
    pub callback: *mut libc::c_void,
    pub sp: *mut libc::c_void,
}
#[no_mangle]
pub unsafe extern "C" fn __osSetHWIntrRoutine(mut intr: OSHWIntr,
                                              mut callback:
                                                  Option<unsafe extern "C" fn()
                                                             -> s32>,
                                              mut sp: *mut libc::c_void) {
    let mut prevInt: u32_0 = __osDisableInt() as u32_0;
    let ref mut fresh0 =
        (*__osHwIntTable.as_mut_ptr().offset(intr as isize)).callback;
    *fresh0 =
        ::std::mem::transmute::<Option<unsafe extern "C" fn() -> s32>,
                                *mut libc::c_void>(callback);
    let ref mut fresh1 =
        (*__osHwIntTable.as_mut_ptr().offset(intr as isize)).sp;
    *fresh1 = sp;
    __osRestoreInt(prevInt as s32);
}
