#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case,
         non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(const_raw_ptr_to_usize_cast, register_tool)]
extern "C" {
    #[no_mangle]
    fn osSyncPrintf(fmt: *const libc::c_char, _: ...);
    #[no_mangle]
    fn bzero(__s: *mut libc::c_void, __n: u32_0);
    #[no_mangle]
    fn __osMallocInit(arena: *mut Arena, start: *mut libc::c_void,
                      size: u32_0);
    #[no_mangle]
    fn __osMallocCleanup(arena: *mut Arena);
    #[no_mangle]
    fn __osMallocIsInitalized(arena: *mut Arena) -> u8_0;
    #[no_mangle]
    fn __osMallocDebug(arena: *mut Arena, size: u32_0,
                       file: *const libc::c_char, line: s32)
     -> *mut libc::c_void;
    #[no_mangle]
    fn __osMallocRDebug(arena: *mut Arena, size: u32_0,
                        file: *const libc::c_char, line: s32)
     -> *mut libc::c_void;
    #[no_mangle]
    fn __osMalloc(arena: *mut Arena, size: u32_0) -> *mut libc::c_void;
    #[no_mangle]
    fn __osMallocR(arena: *mut Arena, size: u32_0) -> *mut libc::c_void;
    #[no_mangle]
    fn __osFree(arena: *mut Arena, ptr: *mut libc::c_void);
    #[no_mangle]
    fn __osFreeDebug(arena: *mut Arena, ptr: *mut libc::c_void,
                     file: *const libc::c_char, line: s32);
    #[no_mangle]
    fn __osRealloc(arena: *mut Arena, ptr: *mut libc::c_void, newSize: u32_0)
     -> *mut libc::c_void;
    #[no_mangle]
    fn __osReallocDebug(arena: *mut Arena, ptr: *mut libc::c_void,
                        newSize: u32_0, file: *const libc::c_char, line: s32)
     -> *mut libc::c_void;
    #[no_mangle]
    fn ArenaImpl_GetSizes(arena: *mut Arena, outMaxFree: *mut u32_0,
                          outFree: *mut u32_0, outAlloc: *mut u32_0);
    #[no_mangle]
    fn __osDisplayArena(arena: *mut Arena);
    #[no_mangle]
    fn __osCheckArena(arena: *mut Arena) -> u32_0;
}
pub type u8_0 = libc::c_uchar;
pub type s16 = libc::c_short;
pub type u16_0 = libc::c_ushort;
pub type s32 = libc::c_int;
pub type u32_0 = libc::c_uint;
pub type u64_0 = libc::c_ulonglong;
pub type f32_0 = libc::c_float;
pub type OSPri = s32;
pub type OSId = s32;
#[derive(Copy, Clone)]
#[repr(C)]
pub union __OSfp {
    pub f: C2RustUnnamed,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed {
    pub f_odd: f32_0,
    pub f_even: f32_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __OSThreadContext {
    pub at: u64_0,
    pub v0: u64_0,
    pub v1: u64_0,
    pub a0: u64_0,
    pub a1: u64_0,
    pub a2: u64_0,
    pub a3: u64_0,
    pub t0: u64_0,
    pub t1: u64_0,
    pub t2: u64_0,
    pub t3: u64_0,
    pub t4: u64_0,
    pub t5: u64_0,
    pub t6: u64_0,
    pub t7: u64_0,
    pub s0: u64_0,
    pub s1: u64_0,
    pub s2: u64_0,
    pub s3: u64_0,
    pub s4: u64_0,
    pub s5: u64_0,
    pub s6: u64_0,
    pub s7: u64_0,
    pub t8: u64_0,
    pub t9: u64_0,
    pub gp: u64_0,
    pub sp: u64_0,
    pub s8: u64_0,
    pub ra: u64_0,
    pub lo: u64_0,
    pub hi: u64_0,
    pub sr: u32_0,
    pub pc: u32_0,
    pub cause: u32_0,
    pub badvaddr: u32_0,
    pub rcp: u32_0,
    pub fpcsr: u32_0,
    pub fp0: __OSfp,
    pub fp2: __OSfp,
    pub fp4: __OSfp,
    pub fp6: __OSfp,
    pub fp8: __OSfp,
    pub fp10: __OSfp,
    pub fp12: __OSfp,
    pub fp14: __OSfp,
    pub fp16: __OSfp,
    pub fp18: __OSfp,
    pub fp20: __OSfp,
    pub fp22: __OSfp,
    pub fp24: __OSfp,
    pub fp26: __OSfp,
    pub fp28: __OSfp,
    pub fp30: __OSfp,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __OSThreadprofile {
    pub flag: u32_0,
    pub count: u32_0,
    pub time: u64_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct OSThread {
    pub next: *mut OSThread,
    pub priority: OSPri,
    pub queue: *mut *mut OSThread,
    pub tlnext: *mut OSThread,
    pub state: u16_0,
    pub flags: u16_0,
    pub id: OSId,
    pub fp: s32,
    pub thprof: *mut __OSThreadprofile,
    pub context: __OSThreadContext,
}
pub type OSMesg = *mut libc::c_void;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct OSMesgQueue {
    pub mtqueue: *mut OSThread,
    pub fullqueue: *mut OSThread,
    pub validCount: s32,
    pub first: s32,
    pub msgCount: s32,
    pub msg: *mut OSMesg,
}
pub type OSTime = u64_0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ArenaNode {
    pub magic: s16,
    pub isFree: s16,
    pub size: u32_0,
    pub next: *mut ArenaNode,
    pub prev: *mut ArenaNode,
    pub filename: *const libc::c_char,
    pub line: s32,
    pub threadId: OSId,
    pub arena: *mut Arena,
    pub time: OSTime,
    pub unk_28: [u8_0; 8],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Arena {
    pub head: *mut ArenaNode,
    pub start: *mut libc::c_void,
    pub lock: OSMesgQueue,
    pub unk_20: u8_0,
    pub isInit: u8_0,
    pub flag: u8_0,
}
#[no_mangle]
pub static mut gSystemArenaLogSeverity: s32 = 0 as libc::c_int;
#[no_mangle]
pub static mut gSystemArena: Arena =
    Arena{head: 0 as *const ArenaNode as *mut ArenaNode,
          start: 0 as *const libc::c_void as *mut libc::c_void,
          lock:
              OSMesgQueue{mtqueue: 0 as *const OSThread as *mut OSThread,
                          fullqueue: 0 as *const OSThread as *mut OSThread,
                          validCount: 0,
                          first: 0,
                          msgCount: 0,
                          msg: 0 as *const OSMesg as *mut OSMesg,},
          unk_20: 0,
          isInit: 0,
          flag: 0,};
#[no_mangle]
pub unsafe extern "C" fn SystemArena_CheckPointer(mut ptr: *mut libc::c_void,
                                                  mut size: u32_0,
                                                  mut name:
                                                      *const libc::c_char,
                                                  mut action:
                                                      *const libc::c_char) {
    if ptr.is_null() {
        if gSystemArenaLogSeverity >= 2 as libc::c_int {
            // "%s: %u bytes %s failed\n"
            osSyncPrintf(b"%s: %u \xe3\x83\x90\xe3\x82\xa4\xe3\x83\x88\xe3\x81\xae%s\xe3\x81\xab\xe5\xa4\xb1\xe6\x95\x97\xe3\x81\x97\xe3\x81\xbe\xe3\x81\x97\xe3\x81\x9f\n\x00"
                             as *const u8 as *const libc::c_char, name, size,
                         action);
            __osDisplayArena(&mut gSystemArena);
            return
        }
    } else if gSystemArenaLogSeverity >= 3 as libc::c_int {
        // "%s: %u bytes %s succeeded\n"
        osSyncPrintf(b"%s: %u \xe3\x83\x90\xe3\x82\xa4\xe3\x83\x88\xe3\x81\xae%s\xe3\x81\xab\xe6\x88\x90\xe5\x8a\x9f\xe3\x81\x97\xe3\x81\xbe\xe3\x81\x97\xe3\x81\x9f\n\x00"
                         as *const u8 as *const libc::c_char, name, size,
                     action); // "Secure"
    }; // "Secure"
}
#[no_mangle]
pub unsafe extern "C" fn SystemArena_Malloc(mut size: u32_0)
 -> *mut libc::c_void {
    let mut ptr: *mut libc::c_void =
        __osMalloc(&mut gSystemArena, size); // "Secure"
    SystemArena_CheckPointer(ptr, size,
                             b"malloc\x00" as *const u8 as
                                 *const libc::c_char,
                             b"\xe7\xa2\xba\xe4\xbf\x9d\x00" as *const u8 as
                                 *const libc::c_char); // "Secure"
    return ptr; // "Re-securing"
}
#[no_mangle]
pub unsafe extern "C" fn SystemArena_MallocDebug(mut size: u32_0,
                                                 mut file:
                                                     *const libc::c_char,
                                                 mut line: s32)
 -> *mut libc::c_void {
    let mut ptr: *mut libc::c_void =
        __osMallocDebug(&mut gSystemArena, size, file, line); // "Re-securing"
    SystemArena_CheckPointer(ptr, size,
                             b"malloc_DEBUG\x00" as *const u8 as
                                 *const libc::c_char,
                             b"\xe7\xa2\xba\xe4\xbf\x9d\x00" as *const u8 as
                                 *const libc::c_char); // "System heap display"
    return ptr;
}
#[no_mangle]
pub unsafe extern "C" fn SystemArena_MallocR(mut size: u32_0)
 -> *mut libc::c_void {
    let mut ptr: *mut libc::c_void = __osMallocR(&mut gSystemArena, size);
    SystemArena_CheckPointer(ptr, size,
                             b"malloc_r\x00" as *const u8 as
                                 *const libc::c_char,
                             b"\xe7\xa2\xba\xe4\xbf\x9d\x00" as *const u8 as
                                 *const libc::c_char);
    return ptr;
}
#[no_mangle]
pub unsafe extern "C" fn SystemArena_MallocRDebug(mut size: u32_0,
                                                  mut file:
                                                      *const libc::c_char,
                                                  mut line: s32)
 -> *mut libc::c_void {
    let mut ptr: *mut libc::c_void =
        __osMallocRDebug(&mut gSystemArena, size, file, line);
    SystemArena_CheckPointer(ptr, size,
                             b"malloc_r_DEBUG\x00" as *const u8 as
                                 *const libc::c_char,
                             b"\xe7\xa2\xba\xe4\xbf\x9d\x00" as *const u8 as
                                 *const libc::c_char);
    return ptr;
}
#[no_mangle]
pub unsafe extern "C" fn SystemArena_Realloc(mut ptr: *mut libc::c_void,
                                             mut newSize: u32_0)
 -> *mut libc::c_void {
    ptr = __osRealloc(&mut gSystemArena, ptr, newSize);
    SystemArena_CheckPointer(ptr, newSize,
                             b"realloc\x00" as *const u8 as
                                 *const libc::c_char,
                             b"\xe5\x86\x8d\xe7\xa2\xba\xe4\xbf\x9d\x00" as
                                 *const u8 as *const libc::c_char);
    return ptr;
}
#[no_mangle]
pub unsafe extern "C" fn SystemArena_ReallocDebug(mut ptr: *mut libc::c_void,
                                                  mut newSize: u32_0,
                                                  mut file:
                                                      *const libc::c_char,
                                                  mut line: s32)
 -> *mut libc::c_void {
    ptr = __osReallocDebug(&mut gSystemArena, ptr, newSize, file, line);
    SystemArena_CheckPointer(ptr, newSize,
                             b"realloc_DEBUG\x00" as *const u8 as
                                 *const libc::c_char,
                             b"\xe5\x86\x8d\xe7\xa2\xba\xe4\xbf\x9d\x00" as
                                 *const u8 as *const libc::c_char);
    return ptr;
}
#[no_mangle]
pub unsafe extern "C" fn SystemArena_Free(mut ptr: *mut libc::c_void) {
    __osFree(&mut gSystemArena, ptr);
}
#[no_mangle]
pub unsafe extern "C" fn SystemArena_FreeDebug(mut ptr: *mut libc::c_void,
                                               mut file: *const libc::c_char,
                                               mut line: s32) {
    __osFreeDebug(&mut gSystemArena, ptr, file, line);
}
#[no_mangle]
pub unsafe extern "C" fn SystemArena_Calloc(mut num: u32_0, mut size: u32_0)
 -> *mut libc::c_void {
    let mut ret: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut n: u32_0 = num.wrapping_mul(size);
    ret = __osMalloc(&mut gSystemArena, n);
    if !ret.is_null() { bzero(ret, n); }
    SystemArena_CheckPointer(ret, n,
                             b"calloc\x00" as *const u8 as
                                 *const libc::c_char,
                             b"\xe7\xa2\xba\xe4\xbf\x9d\x00" as *const u8 as
                                 *const libc::c_char);
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn SystemArena_Display() {
    osSyncPrintf(b"\xe3\x82\xb7\xe3\x82\xb9\xe3\x83\x86\xe3\x83\xa0\xe3\x83\x92\xe3\x83\xbc\xe3\x83\x97\xe8\xa1\xa8\xe7\xa4\xba\n\x00"
                     as *const u8 as *const libc::c_char);
    __osDisplayArena(&mut gSystemArena);
}
#[no_mangle]
pub unsafe extern "C" fn SystemArena_GetSizes(mut outMaxFree: *mut u32_0,
                                              mut outFree: *mut u32_0,
                                              mut outAlloc: *mut u32_0) {
    ArenaImpl_GetSizes(&mut gSystemArena, outMaxFree, outFree, outAlloc);
}
#[no_mangle]
pub unsafe extern "C" fn SystemArena_Check() {
    __osCheckArena(&mut gSystemArena);
}
#[no_mangle]
pub unsafe extern "C" fn SystemArena_Init(mut start: *mut libc::c_void,
                                          mut size: u32_0) {
    gSystemArenaLogSeverity = 0 as libc::c_int;
    __osMallocInit(&mut gSystemArena, start, size);
}
#[no_mangle]
pub unsafe extern "C" fn SystemArena_Cleanup() {
    gSystemArenaLogSeverity = 0 as libc::c_int;
    __osMallocCleanup(&mut gSystemArena);
}
#[no_mangle]
pub unsafe extern "C" fn SystemArena_IsInitalized() -> u8_0 {
    return __osMallocIsInitalized(&mut gSystemArena);
}
