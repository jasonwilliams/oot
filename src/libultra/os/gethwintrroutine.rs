#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case,
         non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(register_tool)]
extern "C" {
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
pub unsafe extern "C" fn __osGetHWIntrRoutine(mut intr: OSHWIntr,
                                              mut callbackOut:
                                                  *mut Option<unsafe extern "C" fn()
                                                                  -> s32>,
                                              mut spOut:
                                                  *mut *mut libc::c_void) {
    *callbackOut =
        ::std::mem::transmute::<*mut libc::c_void,
                                Option<unsafe extern "C" fn()
                                           ->
                                               s32>>((*__osHwIntTable.as_mut_ptr().offset(intr
                                                                                              as
                                                                                              isize)).callback);
    *spOut = (*__osHwIntTable.as_mut_ptr().offset(intr as isize)).sp;
}
