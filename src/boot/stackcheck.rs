#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case,
         non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(const_raw_ptr_to_usize_cast, register_tool)]
extern "C" {
    #[no_mangle]
    fn osSyncPrintf(fmt: *const libc::c_char, _: ...);
    #[no_mangle]
    fn LogUtils_LogHexDump(ptr: *mut libc::c_void, size0: s32);
}
pub type s32 = libc::c_int;
pub type u32_0 = libc::c_uint;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct StackEntry {
    pub next: *mut StackEntry,
    pub prev: *mut StackEntry,
    pub head: u32_0,
    pub tail: u32_0,
    pub initValue: u32_0,
    pub minSpace: s32,
    pub name: *const libc::c_char,
}
pub type StackStatus = libc::c_uint;
pub const STACK_STATUS_OVERFLOW: StackStatus = 2;
pub const STACK_STATUS_WARNING: StackStatus = 1;
pub const STACK_STATUS_OK: StackStatus = 0;
#[no_mangle]
pub static mut sStackInfoListStart: *mut StackEntry =
    0 as *const StackEntry as *mut StackEntry;
#[no_mangle]
pub static mut sStackInfoListEnd: *mut StackEntry =
    0 as *const StackEntry as *mut StackEntry;
#[no_mangle]
pub unsafe extern "C" fn StackCheck_Init(mut entry: *mut StackEntry,
                                         mut stackTop: *mut libc::c_void,
                                         mut stackBottom: *mut libc::c_void,
                                         mut initValue: u32_0,
                                         mut minSpace: s32,
                                         mut name: *const libc::c_char) {
    let mut iter: *mut StackEntry = 0 as *mut StackEntry;
    let mut addr: *mut u32_0 = 0 as *mut u32_0;
    if entry.is_null() {
        sStackInfoListStart = 0 as *mut StackEntry
    } else {
        (*entry).head = stackTop as u32_0;
        (*entry).tail = stackBottom as u32_0;
        (*entry).initValue = initValue;
        (*entry).minSpace = minSpace;
        (*entry).name = name;
        iter = sStackInfoListStart;
        while !iter.is_null() {
            if iter == entry {
                osSyncPrintf(b"\x1b[41;37mstackcheck_init: %08x \xe3\x81\xaf\xe6\x97\xa2\xe3\x81\xab\xe3\x83\xaa\xe3\x82\xb9\xe3\x83\x88\xe4\xb8\xad\xe3\x81\xab\xe3\x81\x82\xe3\x82\x8b\n\x1b[m\x00"
                                 as *const u8 as *const libc::c_char, entry);
                return
            }
            iter = (*iter).next
        }
        (*entry).prev = sStackInfoListEnd;
        (*entry).next = 0 as *mut StackEntry;
        if !sStackInfoListEnd.is_null() { (*sStackInfoListEnd).next = entry }
        sStackInfoListEnd = entry;
        if sStackInfoListStart.is_null() { sStackInfoListStart = entry }
        if (*entry).minSpace != -(1 as libc::c_int) {
            addr = (*entry).head as *mut u32_0;
            while (addr as u32_0) < (*entry).tail {
                let fresh0 = addr;
                addr = addr.offset(1);
                *fresh0 = (*entry).initValue
            }
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn StackCheck_Cleanup(mut entry: *mut StackEntry) {
    let mut inconsistency: u32_0 = 0 as libc::c_int as u32_0;
    if (*entry).prev.is_null() {
        if entry == sStackInfoListStart {
            sStackInfoListStart = (*entry).next
        } else { inconsistency = 1 as libc::c_int as u32_0 }
    } else { (*(*entry).prev).next = (*entry).next }
    if (*entry).next.is_null() {
        if entry == sStackInfoListEnd {
            sStackInfoListEnd = (*entry).prev
        } else { inconsistency = 1 as libc::c_int as u32_0 }
    }
    if inconsistency != 0 {
        osSyncPrintf(b"\x1b[41;37mstackcheck_cleanup: %08x \xe3\x83\xaa\xe3\x82\xb9\xe3\x83\x88\xe4\xb8\x8d\xe6\x95\xb4\xe5\x90\x88\xe3\x81\xa7\xe3\x81\x99\n\x1b[m\x00"
                         as *const u8 as *const libc::c_char, entry);
    };
}
#[no_mangle]
pub unsafe extern "C" fn StackCheck_GetState(mut entry: *mut StackEntry)
 -> StackStatus {
    let mut last: *mut u32_0 = 0 as *mut u32_0;
    let mut used: u32_0 = 0;
    let mut free: u32_0 = 0;
    let mut ret: s32 = 0;
    last = (*entry).head as *mut u32_0;
    while (last as u32_0) < (*entry).tail {
        if (*entry).initValue != *last { break ; }
        last = last.offset(1)
    }
    used = (*entry).tail.wrapping_sub(last as u32_0);
    free = (last as u32_0).wrapping_sub((*entry).head);
    if free == 0 as libc::c_int as libc::c_uint {
        ret = STACK_STATUS_OVERFLOW as libc::c_int;
        osSyncPrintf(b"\x1b[31m\x00" as *const u8 as *const libc::c_char);
    } else if free < (*entry).minSpace as u32_0 &&
                  (*entry).minSpace != -(1 as libc::c_int) {
        ret = STACK_STATUS_WARNING as libc::c_int;
        osSyncPrintf(b"\x1b[33m\x00" as *const u8 as *const libc::c_char);
    } else {
        osSyncPrintf(b"\x1b[32m\x00" as *const u8 as *const libc::c_char);
        ret = STACK_STATUS_OK as libc::c_int
    }
    osSyncPrintf(b"head=%08x tail=%08x last=%08x used=%08x free=%08x [%s]\n\x00"
                     as *const u8 as *const libc::c_char, (*entry).head,
                 (*entry).tail, last, used, free,
                 if !(*entry).name.is_null() {
                     (*entry).name
                 } else {
                     b"(null)\x00" as *const u8 as *const libc::c_char
                 });
    osSyncPrintf(b"\x1b[m\x00" as *const u8 as *const libc::c_char);
    if ret != STACK_STATUS_OK as libc::c_int {
        LogUtils_LogHexDump((*entry).head as *mut libc::c_void,
                            (*entry).tail.wrapping_sub((*entry).head) as s32);
    }
    return ret as StackStatus;
}
#[no_mangle]
pub unsafe extern "C" fn StackCheck_CheckAll() -> u32_0 {
    let mut ret: u32_0 = 0 as libc::c_int as u32_0;
    let mut iter: *mut StackEntry = sStackInfoListStart;
    while !iter.is_null() {
        let mut state: u32_0 = StackCheck_GetState(iter) as u32_0;
        if state != STACK_STATUS_OK as libc::c_int as libc::c_uint {
            ret = 1 as libc::c_int as u32_0
        }
        iter = (*iter).next
    }
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn StackCheck_Check(mut entry: *mut StackEntry)
 -> u32_0 {
    if entry.is_null() {
        return StackCheck_CheckAll()
    } else { return StackCheck_GetState(entry) as u32_0 };
}
