#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case,
         non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(c_variadic, register_tool)]
extern "C" {
    #[no_mangle]
    fn _Printf(_: PrintCallback, arg: *mut libc::c_void,
               fmt: *const libc::c_char, ap: __builtin_va_list) -> s32;
}
pub type __builtin_va_list = *mut libc::c_char;
pub type s32 = libc::c_int;
pub type u32_0 = libc::c_uint;
pub type PrintCallback
    =
    Option<unsafe extern "C" fn(_: *mut libc::c_void, _: *const libc::c_char,
                                _: u32_0) -> *mut libc::c_void>;
#[no_mangle]
pub unsafe extern "C" fn PrintUtils_VPrintf(mut pfn: *mut PrintCallback,
                                            mut fmt: *const libc::c_char,
                                            mut args: __builtin_va_list)
 -> s32 {
    return _Printf(*pfn, pfn as *mut libc::c_void, fmt, args);
}
#[no_mangle]
pub unsafe extern "C" fn PrintUtils_Printf(mut pfn: *mut PrintCallback,
                                           mut fmt: *const libc::c_char,
                                           mut args: ...) -> s32 {
    let mut ret: s32 = 0;
    let mut args_0: __builtin_va_list = 0 as *mut libc::c_char;
    args_0 = args.clone();
    ret = PrintUtils_VPrintf(pfn, fmt, args_0);
    return ret;
}
