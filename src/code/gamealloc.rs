#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case,
         non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(register_tool)]
extern "C" {
    #[no_mangle]
    fn osSyncPrintf(fmt: *const libc::c_char, _: ...);
    #[no_mangle]
    fn LogUtils_CheckNullPointer(exp: *const libc::c_char,
                                 ptr: *mut libc::c_void,
                                 file: *const libc::c_char, line: s32);
    #[no_mangle]
    fn SystemArena_MallocDebug(size: u32_0, file: *const libc::c_char,
                               line: s32) -> *mut libc::c_void;
    #[no_mangle]
    fn SystemArena_FreeDebug(ptr: *mut libc::c_void,
                             file: *const libc::c_char, line: s32);
}
pub type s32 = libc::c_int;
pub type u32_0 = libc::c_uint;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct GameAlloc {
    pub base: GameAllocEntry,
    pub head: *mut GameAllocEntry,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct GameAllocEntry {
    pub next: *mut GameAllocEntry,
    pub prev: *mut GameAllocEntry,
    pub size: u32_0,
    pub unk_0C: u32_0,
}
#[no_mangle]
pub unsafe extern "C" fn GameAlloc_Log(mut this: *mut GameAlloc) {
    let mut iter: *mut GameAllocEntry = 0 as *mut GameAllocEntry;
    osSyncPrintf(b"this = %08x\n\x00" as *const u8 as *const libc::c_char,
                 this);
    iter = (*this).base.next;
    while iter != &mut (*this).base as *mut GameAllocEntry {
        osSyncPrintf(b"ptr = %08x size = %d\n\x00" as *const u8 as
                         *const libc::c_char, iter, (*iter).size);
        iter = (*iter).next
    };
}
#[no_mangle]
pub unsafe extern "C" fn GameAlloc_MallocDebug(mut this: *mut GameAlloc,
                                               mut size: u32_0,
                                               mut file: *const libc::c_char,
                                               mut line: s32)
 -> *mut libc::c_void {
    let mut ptr: *mut GameAllocEntry =
        SystemArena_MallocDebug(size.wrapping_add(::std::mem::size_of::<GameAllocEntry>()
                                                      as libc::c_ulong), file,
                                line) as *mut GameAllocEntry;
    if !ptr.is_null() {
        (*ptr).size = size;
        (*ptr).prev = (*this).head;
        (*(*this).head).next = ptr;
        (*this).head = ptr;
        (*ptr).next = &mut (*this).base;
        (*this).base.prev = (*this).head;
        return ptr.offset(1 as libc::c_int as isize) as *mut libc::c_void
    } else { return 0 as *mut libc::c_void };
}
#[no_mangle]
pub unsafe extern "C" fn GameAlloc_Malloc(mut this: *mut GameAlloc,
                                          mut size: u32_0)
 -> *mut libc::c_void {
    let mut ptr: *mut GameAllocEntry =
        SystemArena_MallocDebug(size.wrapping_add(::std::mem::size_of::<GameAllocEntry>()
                                                      as libc::c_ulong),
                                b"../gamealloc.c\x00" as *const u8 as
                                    *const libc::c_char, 93 as libc::c_int) as
            *mut GameAllocEntry;
    if !ptr.is_null() {
        (*ptr).size = size;
        (*ptr).prev = (*this).head;
        (*(*this).head).next = ptr;
        (*this).head = ptr;
        (*ptr).next = &mut (*this).base;
        (*this).base.prev = (*this).head;
        return ptr.offset(1 as libc::c_int as isize) as *mut libc::c_void
    } else { return 0 as *mut libc::c_void };
}
#[no_mangle]
pub unsafe extern "C" fn GameAlloc_Free(mut this: *mut GameAlloc,
                                        mut data: *mut libc::c_void) {
    let mut ptr: *mut GameAllocEntry = 0 as *mut GameAllocEntry;
    if !data.is_null() {
        ptr =
            &mut *(data as
                       *mut GameAllocEntry).offset(-(1 as libc::c_int) as
                                                       isize) as
                *mut GameAllocEntry;
        LogUtils_CheckNullPointer(b"ptr->prev\x00" as *const u8 as
                                      *const libc::c_char,
                                  (*ptr).prev as *mut libc::c_void,
                                  b"../gamealloc.c\x00" as *const u8 as
                                      *const libc::c_char,
                                  120 as libc::c_int);
        LogUtils_CheckNullPointer(b"ptr->next\x00" as *const u8 as
                                      *const libc::c_char,
                                  (*ptr).next as *mut libc::c_void,
                                  b"../gamealloc.c\x00" as *const u8 as
                                      *const libc::c_char,
                                  121 as libc::c_int);
        (*(*ptr).prev).next = (*ptr).next;
        (*(*ptr).next).prev = (*ptr).prev;
        (*this).head = (*this).base.prev;
        SystemArena_FreeDebug(ptr as *mut libc::c_void,
                              b"../gamealloc.c\x00" as *const u8 as
                                  *const libc::c_char, 125 as libc::c_int);
    };
}
#[no_mangle]
pub unsafe extern "C" fn GameAlloc_Cleanup(mut this: *mut GameAlloc) {
    let mut next: *mut GameAllocEntry = (*this).base.next;
    let mut cur: *mut GameAllocEntry = 0 as *mut GameAllocEntry;
    while &mut (*this).base as *mut GameAllocEntry != next {
        cur = next;
        next = (*next).next;
        SystemArena_FreeDebug(cur as *mut libc::c_void,
                              b"../gamealloc.c\x00" as *const u8 as
                                  *const libc::c_char, 145 as libc::c_int);
    }
    (*this).head = &mut (*this).base;
    (*this).base.next = &mut (*this).base;
    (*this).base.prev = &mut (*this).base;
}
#[no_mangle]
pub unsafe extern "C" fn GameAlloc_Init(mut this: *mut GameAlloc) {
    (*this).head = &mut (*this).base;
    (*this).base.next = &mut (*this).base;
    (*this).base.prev = &mut (*this).base;
}
