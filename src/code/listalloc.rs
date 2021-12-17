#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case,
         non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(register_tool)]
extern "C" {
    #[no_mangle]
    fn SystemArena_MallocDebug(size: u32_0, file: *const libc::c_char,
                               line: s32) -> *mut libc::c_void;
    #[no_mangle]
    fn SystemArena_FreeDebug(ptr: *mut libc::c_void,
                             file: *const libc::c_char, line: s32);
}
pub type u8_0 = libc::c_uchar;
pub type s32 = libc::c_int;
pub type u32_0 = libc::c_uint;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ListAlloc {
    pub prev: *mut ListAlloc,
    pub next: *mut ListAlloc,
}
#[no_mangle]
pub unsafe extern "C" fn ListAlloc_Init(mut this: *mut ListAlloc)
 -> *mut ListAlloc {
    (*this).prev = 0 as *mut ListAlloc;
    (*this).next = 0 as *mut ListAlloc;
    return this;
}
#[no_mangle]
pub unsafe extern "C" fn ListAlloc_Alloc(mut this: *mut ListAlloc,
                                         mut size: u32_0)
 -> *mut libc::c_void {
    let mut ptr: *mut ListAlloc =
        SystemArena_MallocDebug(size.wrapping_add(::std::mem::size_of::<ListAlloc>()
                                                      as libc::c_ulong),
                                b"../listalloc.c\x00" as *const u8 as
                                    *const libc::c_char, 40 as libc::c_int) as
            *mut ListAlloc;
    let mut next: *mut ListAlloc = 0 as *mut ListAlloc;
    if ptr.is_null() { return 0 as *mut libc::c_void }
    next = (*this).next;
    if !next.is_null() { (*next).next = ptr }
    (*ptr).prev = next;
    (*ptr).next = 0 as *mut ListAlloc;
    (*this).next = ptr;
    if (*this).prev.is_null() { (*this).prev = ptr }
    return (ptr as
                *mut u8_0).offset(::std::mem::size_of::<ListAlloc>() as
                                      libc::c_ulong as isize) as
               *mut libc::c_void;
}
#[no_mangle]
pub unsafe extern "C" fn ListAlloc_Free(mut this: *mut ListAlloc,
                                        mut data: *mut libc::c_void) {
    let mut ptr: *mut ListAlloc =
        &mut *(data as *mut ListAlloc).offset(-(1 as libc::c_int) as isize) as
            *mut ListAlloc;
    if !(*ptr).prev.is_null() { (*(*ptr).prev).next = (*ptr).next }
    if !(*ptr).next.is_null() { (*(*ptr).next).prev = (*ptr).prev }
    if (*this).prev == ptr { (*this).prev = (*ptr).next }
    if (*this).next == ptr { (*this).next = (*ptr).prev }
    SystemArena_FreeDebug(ptr as *mut libc::c_void,
                          b"../listalloc.c\x00" as *const u8 as
                              *const libc::c_char, 72 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn ListAlloc_FreeAll(mut this: *mut ListAlloc) {
    let mut iter: *mut ListAlloc = (*this).prev;
    while !iter.is_null() {
        ListAlloc_Free(this,
                       (iter as
                            *mut u8_0).offset(::std::mem::size_of::<ListAlloc>()
                                                  as libc::c_ulong as isize)
                           as *mut libc::c_void);
        iter = (*this).prev
    };
}
