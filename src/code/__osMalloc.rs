#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case,
         non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(const_raw_ptr_to_usize_cast, register_tool)]
extern "C" {
    #[no_mangle]
    fn osSyncPrintf(fmt: *const libc::c_char, _: ...);
    #[no_mangle]
    fn osSendMesg(mq: *mut OSMesgQueue, mesg: OSMesg, flag: s32) -> s32;
    #[no_mangle]
    fn osRecvMesg(mq: *mut OSMesgQueue, msg: *mut OSMesg, flag: s32) -> s32;
    #[no_mangle]
    fn bzero(__s: *mut libc::c_void, __n: u32_0);
    #[no_mangle]
    fn osGetThreadId(thread: *mut OSThread) -> OSId;
    #[no_mangle]
    fn osCreateMesgQueue(mq: *mut OSMesgQueue, msg: *mut OSMesg, count: s32);
    #[no_mangle]
    fn osGetTime() -> OSTime;
    #[no_mangle]
    fn bcopy(__src: *mut libc::c_void, __dest: *mut libc::c_void, __n: u32_0)
     -> *mut libc::c_void;
    #[no_mangle]
    fn FaultDrawer_SetFontColor(_: u16_0);
    #[no_mangle]
    fn FaultDrawer_Printf(_: *const libc::c_char, _: ...);
    #[no_mangle]
    fn func_80106860(ptr: *mut libc::c_void, val: s32, size: size_t)
     -> *mut libc::c_void;
    #[no_mangle]
    fn func_801068B0(dst: *mut libc::c_void, src: *mut libc::c_void,
                     size: size_t) -> *mut libc::c_void;
}
pub type u8_0 = libc::c_uchar;
pub type s16 = libc::c_short;
pub type u16_0 = libc::c_ushort;
pub type s32 = libc::c_int;
pub type u32_0 = libc::c_uint;
pub type u64_0 = libc::c_ulonglong;
pub type f32_0 = libc::c_float;
pub type size_t = libc::c_ulong;
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
pub static mut sArenaLockMsg: OSMesg =
    0 as *const libc::c_void as *mut libc::c_void;
#[no_mangle]
pub static mut __osMalloc_FreeBlockTest_Enable: u32_0 = 0;
#[no_mangle]
pub unsafe extern "C" fn ArenaImpl_GetFillAllocBlock(mut arena: *mut Arena)
 -> u32_0 {
    return ((*arena).flag as libc::c_int &
                (1 as libc::c_int) << 0 as libc::c_int != 0 as libc::c_int) as
               libc::c_int as u32_0; // memset
}
#[no_mangle]
pub unsafe extern "C" fn ArenaImpl_GetFillFreeBlock(mut arena: *mut Arena)
 -> u32_0 {
    return ((*arena).flag as libc::c_int &
                (1 as libc::c_int) << 1 as libc::c_int != 0 as libc::c_int) as
               libc::c_int as u32_0; // memset
}
#[no_mangle]
pub unsafe extern "C" fn ArenaImpl_GetCheckFreeBlock(mut arena: *mut Arena)
 -> u32_0 {
    return ((*arena).flag as libc::c_int &
                (1 as libc::c_int) << 2 as libc::c_int != 0 as libc::c_int) as
               libc::c_int as u32_0;
}
#[no_mangle]
pub unsafe extern "C" fn ArenaImpl_SetFillAllocBlock(mut arena: *mut Arena) {
    (*arena).flag =
        ((*arena).flag as libc::c_int |
             (1 as libc::c_int) << 0 as libc::c_int) as u8_0;
}
#[no_mangle]
pub unsafe extern "C" fn ArenaImpl_SetFillFreeBlock(mut arena: *mut Arena) {
    (*arena).flag =
        ((*arena).flag as libc::c_int |
             (1 as libc::c_int) << 1 as libc::c_int) as u8_0;
}
#[no_mangle]
pub unsafe extern "C" fn ArenaImpl_SetCheckFreeBlock(mut arena: *mut Arena) {
    (*arena).flag =
        ((*arena).flag as libc::c_int |
             (1 as libc::c_int) << 2 as libc::c_int) as u8_0;
}
#[no_mangle]
pub unsafe extern "C" fn ArenaImpl_UnsetFillAllocBlock(mut arena:
                                                           *mut Arena) {
    (*arena).flag =
        ((*arena).flag as libc::c_int &
             !((1 as libc::c_int) << 0 as libc::c_int)) as u8_0;
}
#[no_mangle]
pub unsafe extern "C" fn ArenaImpl_UnsetFillFreeBlock(mut arena: *mut Arena) {
    (*arena).flag =
        ((*arena).flag as libc::c_int &
             !((1 as libc::c_int) << 1 as libc::c_int)) as u8_0;
}
#[no_mangle]
pub unsafe extern "C" fn ArenaImpl_UnsetCheckFreeBlock(mut arena:
                                                           *mut Arena) {
    (*arena).flag =
        ((*arena).flag as libc::c_int &
             !((1 as libc::c_int) << 2 as libc::c_int)) as u8_0;
}
#[no_mangle]
pub unsafe extern "C" fn ArenaImpl_SetDebugInfo(mut node: *mut ArenaNode,
                                                mut file: *const libc::c_char,
                                                mut line: s32,
                                                mut arena: *mut Arena) {
    (*node).filename = file;
    (*node).line = line;
    (*node).threadId = osGetThreadId(0 as *mut OSThread);
    (*node).arena = arena;
    (*node).time = osGetTime();
}
#[no_mangle]
pub unsafe extern "C" fn ArenaImpl_LockInit(mut arena: *mut Arena) {
    osCreateMesgQueue(&mut (*arena).lock, &mut sArenaLockMsg,
                      1 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn ArenaImpl_Lock(mut arena: *mut Arena) {
    osSendMesg(&mut (*arena).lock, 0 as *mut libc::c_void, 1 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn ArenaImpl_Unlock(mut arena: *mut Arena) {
    osRecvMesg(&mut (*arena).lock, 0 as *mut OSMesg, 1 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn ArenaImpl_GetNextBlock(mut node: *mut ArenaNode)
 -> *mut ArenaNode {
    let mut next: *mut ArenaNode = (*node).next;
    if !next.is_null() &&
           (next.is_null() ||
                (*next).magic as libc::c_int != 0x7373 as libc::c_int) {
        osSyncPrintf(b"\x1b[41;37m\xe7\xb7\x8a\xe6\x80\xa5\xe4\xba\x8b\xe6\x85\x8b\xef\xbc\x81\xe3\x83\xa1\xe3\x83\xa2\xe3\x83\xaa\xe3\x83\xaa\xe3\x83\xbc\xe3\x82\xaf\xe7\x99\xba\xe8\xa6\x8b\xef\xbc\x81 (block=%08x)\n\x1b[m\x00"
                         as *const u8 as *const libc::c_char, next);
        next = 0 as *mut ArenaNode;
        (*node).next = 0 as *mut ArenaNode
    }
    return next;
}
#[no_mangle]
pub unsafe extern "C" fn ArenaImpl_GetPrevBlock(mut node: *mut ArenaNode)
 -> *mut ArenaNode {
    let mut prev: *mut ArenaNode = (*node).prev;
    if !prev.is_null() &&
           (prev.is_null() ||
                (*prev).magic as libc::c_int != 0x7373 as libc::c_int) {
        osSyncPrintf(b"\x1b[41;37m\xe7\xb7\x8a\xe6\x80\xa5\xe4\xba\x8b\xe6\x85\x8b\xef\xbc\x81\xe3\x83\xa1\xe3\x83\xa2\xe3\x83\xaa\xe3\x83\xaa\xe3\x83\xbc\xe3\x82\xaf\xe7\x99\xba\xe8\xa6\x8b\xef\xbc\x81 (block=%08x)\n\x1b[m\x00"
                         as *const u8 as *const libc::c_char, prev);
        prev = 0 as *mut ArenaNode;
        (*node).prev = 0 as *mut ArenaNode
    }
    return prev;
}
#[no_mangle]
pub unsafe extern "C" fn ArenaImpl_GetLastBlock(mut arena: *mut Arena)
 -> *mut ArenaNode {
    let mut last: *mut ArenaNode = 0 as *mut ArenaNode;
    let mut iter: *mut ArenaNode = 0 as *mut ArenaNode;
    if !arena.is_null() && !(*arena).head.is_null() &&
           (*(*arena).head).magic as libc::c_int == 0x7373 as libc::c_int {
        iter = (*arena).head;
        while !iter.is_null() {
            last = iter;
            iter = ArenaImpl_GetNextBlock(iter)
        }
    }
    return last;
}
#[no_mangle]
pub unsafe extern "C" fn __osMallocInit(mut arena: *mut Arena,
                                        mut start: *mut libc::c_void,
                                        mut size: u32_0) {
    bzero(arena as *mut libc::c_void,
          ::std::mem::size_of::<Arena>() as libc::c_ulong);
    ArenaImpl_LockInit(arena);
    __osMallocAddBlock(arena, start, size as s32);
    (*arena).isInit = 1 as libc::c_int as u8_0;
}
#[no_mangle]
pub unsafe extern "C" fn __osMallocAddBlock(mut arena: *mut Arena,
                                            mut start: *mut libc::c_void,
                                            mut size: s32) {
    let mut diff: s32 = 0;
    let mut size2: s32 = 0;
    let mut firstNode: *mut ArenaNode = 0 as *mut ArenaNode;
    let mut lastNode: *mut ArenaNode = 0 as *mut ArenaNode;
    if !start.is_null() {
        firstNode =
            ((start as u32_0).wrapping_add(0xf as libc::c_int as libc::c_uint)
                 & !(0xf as libc::c_int) as libc::c_uint) as *mut ArenaNode;
        diff = firstNode as s32 - start as s32;
        size2 = size - diff & !(0xf as libc::c_int);
        if size2 > ::std::mem::size_of::<ArenaNode>() as libc::c_ulong as s32
           {
            func_80106860(firstNode as *mut libc::c_void, 0xab as libc::c_int,
                          size2 as size_t);
            (*firstNode).next = 0 as *mut ArenaNode;
            (*firstNode).prev = 0 as *mut ArenaNode;
            (*firstNode).size =
                (size2 as
                     libc::c_uint).wrapping_sub(::std::mem::size_of::<ArenaNode>()
                                                    as libc::c_ulong);
            (*firstNode).isFree = 1 as libc::c_int as s16;
            (*firstNode).magic = 0x7373 as libc::c_int as s16;
            ArenaImpl_Lock(arena);
            lastNode = ArenaImpl_GetLastBlock(arena);
            if lastNode.is_null() {
                (*arena).head = firstNode;
                (*arena).start = start
            } else {
                (*firstNode).prev = lastNode;
                (*lastNode).next = firstNode
            }
            ArenaImpl_Unlock(arena);
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn ArenaImpl_RemoveAllBlocks(mut arena: *mut Arena) {
    let mut iter: *mut ArenaNode = 0 as *mut ArenaNode;
    let mut next: *mut ArenaNode = 0 as *mut ArenaNode;
    ArenaImpl_Lock(arena);
    iter = (*arena).head;
    while !iter.is_null() {
        next = ArenaImpl_GetNextBlock(iter);
        func_80106860(iter as *mut libc::c_void, 0xab as libc::c_int,
                      (*iter).size.wrapping_add(::std::mem::size_of::<ArenaNode>()
                                                    as libc::c_ulong) as
                          size_t);
        iter = next
    }
    ArenaImpl_Unlock(arena);
}
#[no_mangle]
pub unsafe extern "C" fn __osMallocCleanup(mut arena: *mut Arena) {
    ArenaImpl_RemoveAllBlocks(arena);
    bzero(arena as *mut libc::c_void,
          ::std::mem::size_of::<Arena>() as libc::c_ulong);
}
#[no_mangle]
pub unsafe extern "C" fn __osMallocIsInitalized(mut arena: *mut Arena)
 -> u8_0 {
    return (*arena).isInit;
}
#[no_mangle]
pub unsafe extern "C" fn __osMalloc_FreeBlockTest(mut arena: *mut Arena,
                                                  mut node: *mut ArenaNode) {
    let mut node2: *mut ArenaNode = node;
    let mut start: *mut u32_0 = 0 as *mut u32_0;
    let mut end: *mut u32_0 = 0 as *mut u32_0;
    let mut iter: *mut u32_0 = 0 as *mut u32_0;
    if __osMalloc_FreeBlockTest_Enable != 0 {
        start =
            (node as
                 u32_0).wrapping_add(::std::mem::size_of::<ArenaNode>() as
                                         libc::c_ulong) as *mut u32_0;
        end = (start as u32_0).wrapping_add((*node2).size) as *mut u32_0;
        iter = start;
        while iter < end {
            if *iter != 0xabababab as libc::c_uint &&
                   *iter != 0xefefefef as libc::c_uint {
                osSyncPrintf(b"\x1b[41;37m\xe7\xb7\x8a\xe6\x80\xa5\xe4\xba\x8b\xe6\x85\x8b\xef\xbc\x81\xe3\x83\xa1\xe3\x83\xa2\xe3\x83\xaa\xe3\x83\xaa\xe3\x83\xbc\xe3\x82\xaf\xe6\xa4\x9c\xe5\x87\xba\xef\xbc\x81 (block=%08x s=%08x e=%08x p=%08x)\n\x1b[m\x00"
                                 as *const u8 as *const libc::c_char, node,
                             start, end, iter);
                __osDisplayArena(arena);
                return
            }
            iter = iter.offset(1)
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn __osMalloc_NoLockDebug(mut arena: *mut Arena,
                                                mut size: u32_0,
                                                mut file: *const libc::c_char,
                                                mut line: s32)
 -> *mut libc::c_void {
    let mut iter: *mut ArenaNode = 0 as *mut ArenaNode;
    let mut blockSize: u32_0 = 0;
    let mut newNode: *mut ArenaNode = 0 as *mut ArenaNode;
    let mut alloc: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut next: *mut ArenaNode = 0 as *mut ArenaNode;
    iter = (*arena).head;
    size =
        size.wrapping_add(0xf as libc::c_int as libc::c_uint) &
            !(0xf as libc::c_int) as libc::c_uint;
    blockSize =
        (size.wrapping_add(0xf as libc::c_int as libc::c_uint) &
             !(0xf as libc::c_int) as
                 libc::c_uint).wrapping_add(::std::mem::size_of::<ArenaNode>()
                                                as libc::c_ulong);
    while !iter.is_null() {
        if (*iter).isFree as libc::c_int != 0 && (*iter).size >= size {
            if (*arena).flag as libc::c_int &
                   (1 as libc::c_int) << 2 as libc::c_int != 0 {
                __osMalloc_FreeBlockTest(arena, iter);
            }
            if blockSize < (*iter).size {
                newNode =
                    (iter as u32_0).wrapping_add(blockSize) as *mut ArenaNode;
                (*newNode).next = ArenaImpl_GetNextBlock(iter);
                (*newNode).prev = iter;
                (*newNode).size = (*iter).size.wrapping_sub(blockSize);
                (*newNode).isFree = 1 as libc::c_int as s16;
                (*newNode).magic = 0x7373 as libc::c_int as s16;
                (*iter).next = newNode;
                (*iter).size = size;
                next = ArenaImpl_GetNextBlock(newNode);
                if !next.is_null() { (*next).prev = newNode }
            }
            (*iter).isFree = 0 as libc::c_int as s16;
            ArenaImpl_SetDebugInfo(iter, file, line, arena);
            alloc =
                (iter as
                     u32_0).wrapping_add(::std::mem::size_of::<ArenaNode>() as
                                             libc::c_ulong) as
                    *mut libc::c_void;
            if (*arena).flag as libc::c_int &
                   (1 as libc::c_int) << 0 as libc::c_int != 0 {
                func_80106860(alloc, 0xcd as libc::c_int, size as size_t);
            }
            break ;
        } else { iter = ArenaImpl_GetNextBlock(iter) }
    }
    return alloc;
}
#[no_mangle]
pub unsafe extern "C" fn __osMallocDebug(mut arena: *mut Arena,
                                         mut size: u32_0,
                                         mut file: *const libc::c_char,
                                         mut line: s32) -> *mut libc::c_void {
    let mut alloc: *mut libc::c_void = 0 as *mut libc::c_void;
    ArenaImpl_Lock(arena);
    alloc = __osMalloc_NoLockDebug(arena, size, file, line);
    ArenaImpl_Unlock(arena);
    return alloc;
}
#[no_mangle]
pub unsafe extern "C" fn __osMallocRDebug(mut arena: *mut Arena,
                                          mut size: u32_0,
                                          mut file: *const libc::c_char,
                                          mut line: s32)
 -> *mut libc::c_void {
    let mut iter: *mut ArenaNode = 0 as *mut ArenaNode;
    let mut newNode: *mut ArenaNode = 0 as *mut ArenaNode;
    let mut blockSize: u32_0 = 0;
    let mut next: *mut ArenaNode = 0 as *mut ArenaNode;
    let mut allocR: *mut libc::c_void = 0 as *mut libc::c_void;
    size =
        size.wrapping_add(0xf as libc::c_int as libc::c_uint) &
            !(0xf as libc::c_int) as libc::c_uint;
    ArenaImpl_Lock(arena);
    iter = ArenaImpl_GetLastBlock(arena);
    while !iter.is_null() {
        if (*iter).isFree as libc::c_int != 0 && (*iter).size >= size {
            if (*arena).flag as libc::c_int &
                   (1 as libc::c_int) << 2 as libc::c_int != 0 {
                __osMalloc_FreeBlockTest(arena, iter);
            }
            blockSize =
                (size.wrapping_add(0xf as libc::c_int as libc::c_uint) &
                     !(0xf as libc::c_int) as
                         libc::c_uint).wrapping_add(::std::mem::size_of::<ArenaNode>()
                                                        as libc::c_ulong);
            if blockSize < (*iter).size {
                newNode =
                    (iter as
                         u32_0).wrapping_add((*iter).size.wrapping_sub(size))
                        as *mut ArenaNode;
                (*newNode).next = ArenaImpl_GetNextBlock(iter);
                (*newNode).prev = iter;
                (*newNode).size = size;
                (*newNode).magic = 0x7373 as libc::c_int as s16;
                (*iter).next = newNode;
                (*iter).size =
                    ((*iter).size as libc::c_uint).wrapping_sub(blockSize) as
                        u32_0 as u32_0;
                next = ArenaImpl_GetNextBlock(newNode);
                if !next.is_null() { (*next).prev = newNode }
                iter = newNode
            }
            (*iter).isFree = 0 as libc::c_int as s16;
            ArenaImpl_SetDebugInfo(iter, file, line, arena);
            allocR =
                (iter as
                     u32_0).wrapping_add(::std::mem::size_of::<ArenaNode>() as
                                             libc::c_ulong) as
                    *mut libc::c_void;
            if (*arena).flag as libc::c_int &
                   (1 as libc::c_int) << 0 as libc::c_int != 0 {
                func_80106860(allocR, 0xcd as libc::c_int, size as size_t);
            }
            break ;
        } else { iter = ArenaImpl_GetPrevBlock(iter) }
    }
    ArenaImpl_Unlock(arena);
    return allocR;
}
#[no_mangle]
pub unsafe extern "C" fn __osMalloc_NoLock(mut arena: *mut Arena,
                                           mut size: u32_0)
 -> *mut libc::c_void {
    let mut iter: *mut ArenaNode = 0 as *mut ArenaNode;
    let mut blockSize: u32_0 = 0;
    let mut newNode: *mut ArenaNode = 0 as *mut ArenaNode;
    let mut alloc: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut next: *mut ArenaNode = 0 as *mut ArenaNode;
    iter = (*arena).head;
    size =
        size.wrapping_add(0xf as libc::c_int as libc::c_uint) &
            !(0xf as libc::c_int) as libc::c_uint;
    blockSize =
        (size.wrapping_add(0xf as libc::c_int as libc::c_uint) &
             !(0xf as libc::c_int) as
                 libc::c_uint).wrapping_add(::std::mem::size_of::<ArenaNode>()
                                                as libc::c_ulong);
    while !iter.is_null() {
        if (*iter).isFree as libc::c_int != 0 && (*iter).size >= size {
            if (*arena).flag as libc::c_int &
                   (1 as libc::c_int) << 2 as libc::c_int != 0 {
                __osMalloc_FreeBlockTest(arena, iter);
            }
            if blockSize < (*iter).size {
                newNode =
                    (iter as u32_0).wrapping_add(blockSize) as *mut ArenaNode;
                (*newNode).next = ArenaImpl_GetNextBlock(iter);
                (*newNode).prev = iter;
                (*newNode).size = (*iter).size.wrapping_sub(blockSize);
                (*newNode).isFree = 1 as libc::c_int as s16;
                (*newNode).magic = 0x7373 as libc::c_int as s16;
                (*iter).next = newNode;
                (*iter).size = size;
                next = ArenaImpl_GetNextBlock(newNode);
                if !next.is_null() { (*next).prev = newNode }
            }
            (*iter).isFree = 0 as libc::c_int as s16;
            ArenaImpl_SetDebugInfo(iter, 0 as *const libc::c_char,
                                   0 as libc::c_int, arena);
            alloc =
                (iter as
                     u32_0).wrapping_add(::std::mem::size_of::<ArenaNode>() as
                                             libc::c_ulong) as
                    *mut libc::c_void;
            if (*arena).flag as libc::c_int &
                   (1 as libc::c_int) << 0 as libc::c_int != 0 {
                func_80106860(alloc, 0xcd as libc::c_int, size as size_t);
            }
            break ;
        } else { iter = ArenaImpl_GetNextBlock(iter) }
    }
    return alloc;
}
#[no_mangle]
pub unsafe extern "C" fn __osMalloc(mut arena: *mut Arena, mut size: u32_0)
 -> *mut libc::c_void {
    let mut alloc: *mut libc::c_void = 0 as *mut libc::c_void;
    ArenaImpl_Lock(arena);
    alloc = __osMalloc_NoLock(arena, size);
    ArenaImpl_Unlock(arena);
    return alloc;
}
#[no_mangle]
pub unsafe extern "C" fn __osMallocR(mut arena: *mut Arena, mut size: u32_0)
 -> *mut libc::c_void {
    let mut iter: *mut ArenaNode = 0 as *mut ArenaNode;
    let mut newNode: *mut ArenaNode = 0 as *mut ArenaNode;
    let mut blockSize: u32_0 = 0;
    let mut next: *mut ArenaNode = 0 as *mut ArenaNode;
    let mut alloc: *mut libc::c_void = 0 as *mut libc::c_void;
    size =
        size.wrapping_add(0xf as libc::c_int as libc::c_uint) &
            !(0xf as libc::c_int) as libc::c_uint;
    ArenaImpl_Lock(arena);
    iter = ArenaImpl_GetLastBlock(arena);
    while !iter.is_null() {
        if (*iter).isFree as libc::c_int != 0 && (*iter).size >= size {
            if (*arena).flag as libc::c_int &
                   (1 as libc::c_int) << 2 as libc::c_int != 0 {
                __osMalloc_FreeBlockTest(arena, iter);
            }
            blockSize =
                (size.wrapping_add(0xf as libc::c_int as libc::c_uint) &
                     !(0xf as libc::c_int) as
                         libc::c_uint).wrapping_add(::std::mem::size_of::<ArenaNode>()
                                                        as libc::c_ulong);
            if blockSize < (*iter).size {
                newNode =
                    (iter as
                         u32_0).wrapping_add((*iter).size.wrapping_sub(size))
                        as *mut ArenaNode;
                (*newNode).next = ArenaImpl_GetNextBlock(iter);
                (*newNode).prev = iter;
                (*newNode).size = size;
                (*newNode).magic = 0x7373 as libc::c_int as s16;
                (*iter).next = newNode;
                (*iter).size =
                    ((*iter).size as libc::c_uint).wrapping_sub(blockSize) as
                        u32_0 as u32_0;
                next = ArenaImpl_GetNextBlock(newNode);
                if !next.is_null() { (*next).prev = newNode }
                iter = newNode
            }
            (*iter).isFree = 0 as libc::c_int as s16;
            ArenaImpl_SetDebugInfo(iter, 0 as *const libc::c_char,
                                   0 as libc::c_int, arena);
            alloc =
                (iter as
                     u32_0).wrapping_add(::std::mem::size_of::<ArenaNode>() as
                                             libc::c_ulong) as
                    *mut libc::c_void;
            if (*arena).flag as libc::c_int &
                   (1 as libc::c_int) << 0 as libc::c_int != 0 {
                func_80106860(alloc, 0xcd as libc::c_int, size as size_t);
            }
            break ;
        } else { iter = ArenaImpl_GetPrevBlock(iter) }
    }
    ArenaImpl_Unlock(arena);
    return alloc;
}
#[no_mangle]
pub unsafe extern "C" fn __osFree_NoLock(mut arena: *mut Arena,
                                         mut ptr: *mut libc::c_void) {
    let mut node: *mut ArenaNode = 0 as *mut ArenaNode;
    let mut next: *mut ArenaNode = 0 as *mut ArenaNode;
    let mut prev: *mut ArenaNode = 0 as *mut ArenaNode;
    let mut newNext: *mut ArenaNode = 0 as *mut ArenaNode;
    if ptr.is_null() { return }
    node =
        (ptr as
             u32_0).wrapping_sub(::std::mem::size_of::<ArenaNode>() as
                                     libc::c_ulong) as *mut ArenaNode;
    if node.is_null() || (*node).magic as libc::c_int != 0x7373 as libc::c_int
       {
        // "__osFree: Unauthorized release (%08x)"
        osSyncPrintf(b"\x1b[41;37m__osFree:\xe4\xb8\x8d\xe6\xad\xa3\xe8\xa7\xa3\xe6\x94\xbe(%08x)\n\x1b[m\x00"
                         as *const u8 as *const libc::c_char,
                     ptr); // "__osFree: Double release (%08x)"
        return
    }
    if (*node).isFree != 0 {
        osSyncPrintf(b"\x1b[41;37m__osFree:\xe4\xba\x8c\xe9\x87\x8d\xe8\xa7\xa3\xe6\x94\xbe(%08x)\n\x1b[m\x00"
                         as *const u8 as *const libc::c_char, ptr);
        return
    }
    if arena != (*node).arena && !arena.is_null() {
        // "__osFree:Tried to release in a different way than when it was secured (%08x:%08x)"
        osSyncPrintf(b"\x1b[41;37m__osFree:\xe7\xa2\xba\xe4\xbf\x9d\xe6\x99\x82\xe3\x81\xa8\xe9\x81\x95\xe3\x81\x86\xe6\x96\xb9\xe6\xb3\x95\xe3\x81\xa7\xe8\xa7\xa3\xe6\x94\xbe\xe3\x81\x97\xe3\x82\x88\xe3\x81\x86\xe3\x81\xa8\xe3\x81\x97\xe3\x81\x9f (%08x:%08x)\n\x1b[m\x00"
                         as *const u8 as *const libc::c_char, arena,
                     (*node).arena);
        return
    }
    next = ArenaImpl_GetNextBlock(node);
    prev = ArenaImpl_GetPrevBlock(node);
    (*node).isFree = 1 as libc::c_int as s16;
    ArenaImpl_SetDebugInfo(node, 0 as *const libc::c_char, 0 as libc::c_int,
                           arena);
    if (*arena).flag as libc::c_int & (1 as libc::c_int) << 1 as libc::c_int
           != 0 {
        func_80106860((node as
                           u32_0).wrapping_add(::std::mem::size_of::<ArenaNode>()
                                                   as libc::c_ulong) as
                          *mut libc::c_void, 0xef as libc::c_int,
                      (*node).size as size_t);
    }
    newNext = next;
    if next as u32_0 ==
           (node as
                u32_0).wrapping_add(::std::mem::size_of::<ArenaNode>() as
                                        libc::c_ulong).wrapping_add((*node).size)
           && (*next).isFree as libc::c_int != 0 {
        newNext = ArenaImpl_GetNextBlock(next);
        if !newNext.is_null() { (*newNext).prev = node }
        (*node).size =
            ((*node).size as
                 libc::c_uint).wrapping_add((*next).size.wrapping_add(::std::mem::size_of::<ArenaNode>()
                                                                          as
                                                                          libc::c_ulong))
                as u32_0 as u32_0;
        if (*arena).flag as libc::c_int &
               (1 as libc::c_int) << 1 as libc::c_int != 0 {
            func_80106860(next as *mut libc::c_void, 0xef as libc::c_int,
                          ::std::mem::size_of::<ArenaNode>() as libc::c_ulong
                              as size_t);
        }
        (*node).next = newNext;
        next = newNext
    }
    if !prev.is_null() && (*prev).isFree as libc::c_int != 0 &&
           node as u32_0 ==
               (prev as
                    u32_0).wrapping_add(::std::mem::size_of::<ArenaNode>() as
                                            libc::c_ulong).wrapping_add((*prev).size)
       {
        if !next.is_null() { (*next).prev = prev }
        (*prev).next = next;
        (*prev).size =
            ((*prev).size as
                 libc::c_uint).wrapping_add((*node).size.wrapping_add(::std::mem::size_of::<ArenaNode>()
                                                                          as
                                                                          libc::c_ulong))
                as u32_0 as u32_0;
        if (*arena).flag as libc::c_int &
               (1 as libc::c_int) << 1 as libc::c_int != 0 {
            func_80106860(node as *mut libc::c_void, 0xef as libc::c_int,
                          ::std::mem::size_of::<ArenaNode>() as libc::c_ulong
                              as size_t);
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn __osFree(mut arena: *mut Arena,
                                  mut ptr: *mut libc::c_void) {
    ArenaImpl_Lock(arena);
    __osFree_NoLock(arena, ptr);
    ArenaImpl_Unlock(arena);
}
#[no_mangle]
pub unsafe extern "C" fn __osFree_NoLockDebug(mut arena: *mut Arena,
                                              mut ptr: *mut libc::c_void,
                                              mut file: *const libc::c_char,
                                              mut line: s32) {
    let mut node: *mut ArenaNode = 0 as *mut ArenaNode;
    let mut next: *mut ArenaNode = 0 as *mut ArenaNode;
    let mut prev: *mut ArenaNode = 0 as *mut ArenaNode;
    let mut newNext: *mut ArenaNode = 0 as *mut ArenaNode;
    if ptr.is_null() { return }
    node =
        (ptr as
             u32_0).wrapping_sub(::std::mem::size_of::<ArenaNode>() as
                                     libc::c_ulong) as *mut ArenaNode;
    if node.is_null() || (*node).magic as libc::c_int != 0x7373 as libc::c_int
       {
        // "__osFree: Unauthorized release (%08x)"
        osSyncPrintf(b"\x1b[41;37m__osFree:\xe4\xb8\x8d\xe6\xad\xa3\xe8\xa7\xa3\xe6\x94\xbe(%08x) [%s:%d ]\n\x1b[m\x00"
                         as *const u8 as *const libc::c_char, ptr, file,
                     line);
        return
    }
    if (*node).isFree != 0 {
        // "__osFree: Double release (%08x)"
        osSyncPrintf(b"\x1b[41;37m__osFree:\xe4\xba\x8c\xe9\x87\x8d\xe8\xa7\xa3\xe6\x94\xbe(%08x) [%s:%d ]\n\x1b[m\x00"
                         as *const u8 as *const libc::c_char, ptr, file,
                     line);
        return
    }
    if arena != (*node).arena && !arena.is_null() {
        // "__osFree:Tried to release in a different way than when it was secured (%08x:%08x)"
        osSyncPrintf(b"\x1b[41;37m__osFree:\xe7\xa2\xba\xe4\xbf\x9d\xe6\x99\x82\xe3\x81\xa8\xe9\x81\x95\xe3\x81\x86\xe6\x96\xb9\xe6\xb3\x95\xe3\x81\xa7\xe8\xa7\xa3\xe6\x94\xbe\xe3\x81\x97\xe3\x82\x88\xe3\x81\x86\xe3\x81\xa8\xe3\x81\x97\xe3\x81\x9f (%08x:%08x)\n\x1b[m\x00"
                         as *const u8 as *const libc::c_char, arena,
                     (*node).arena);
        return
    }
    next = ArenaImpl_GetNextBlock(node);
    prev = ArenaImpl_GetPrevBlock(node);
    (*node).isFree = 1 as libc::c_int as s16;
    ArenaImpl_SetDebugInfo(node, file, line, arena);
    if (*arena).flag as libc::c_int & (1 as libc::c_int) << 1 as libc::c_int
           != 0 {
        func_80106860((node as
                           u32_0).wrapping_add(::std::mem::size_of::<ArenaNode>()
                                                   as libc::c_ulong) as
                          *mut libc::c_void, 0xef as libc::c_int,
                      (*node).size as size_t);
    }
    newNext = (*node).next;
    if next as u32_0 ==
           (node as
                u32_0).wrapping_add(::std::mem::size_of::<ArenaNode>() as
                                        libc::c_ulong).wrapping_add((*node).size)
           && (*next).isFree as libc::c_int != 0 {
        newNext = ArenaImpl_GetNextBlock(next);
        if !newNext.is_null() { (*newNext).prev = node }
        (*node).size =
            ((*node).size as
                 libc::c_uint).wrapping_add((*next).size.wrapping_add(::std::mem::size_of::<ArenaNode>()
                                                                          as
                                                                          libc::c_ulong))
                as u32_0 as u32_0;
        if (*arena).flag as libc::c_int &
               (1 as libc::c_int) << 1 as libc::c_int != 0 {
            func_80106860(next as *mut libc::c_void, 0xef as libc::c_int,
                          ::std::mem::size_of::<ArenaNode>() as libc::c_ulong
                              as size_t);
        }
        (*node).next = newNext;
        next = newNext
    }
    if !prev.is_null() && (*prev).isFree as libc::c_int != 0 &&
           node as u32_0 ==
               (prev as
                    u32_0).wrapping_add(::std::mem::size_of::<ArenaNode>() as
                                            libc::c_ulong).wrapping_add((*prev).size)
       {
        if !next.is_null() { (*next).prev = prev }
        (*prev).next = next;
        (*prev).size =
            ((*prev).size as
                 libc::c_uint).wrapping_add((*node).size.wrapping_add(::std::mem::size_of::<ArenaNode>()
                                                                          as
                                                                          libc::c_ulong))
                as u32_0 as u32_0;
        if (*arena).flag as libc::c_int &
               (1 as libc::c_int) << 1 as libc::c_int != 0 {
            func_80106860(node as *mut libc::c_void, 0xef as libc::c_int,
                          ::std::mem::size_of::<ArenaNode>() as libc::c_ulong
                              as size_t);
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn __osFreeDebug(mut arena: *mut Arena,
                                       mut ptr: *mut libc::c_void,
                                       mut file: *const libc::c_char,
                                       mut line: s32) {
    ArenaImpl_Lock(arena);
    __osFree_NoLockDebug(arena, ptr, file, line);
    ArenaImpl_Unlock(arena);
}
#[no_mangle]
pub unsafe extern "C" fn __osRealloc(mut arena: *mut Arena,
                                     mut ptr: *mut libc::c_void,
                                     mut newSize: u32_0)
 -> *mut libc::c_void {
    let mut newAlloc: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut node: *mut ArenaNode = 0 as *mut ArenaNode;
    let mut next: *mut ArenaNode = 0 as *mut ArenaNode;
    let mut newNext: *mut ArenaNode = 0 as *mut ArenaNode;
    let mut overNext: *mut ArenaNode = 0 as *mut ArenaNode;
    let mut newNext2: *mut ArenaNode = 0 as *mut ArenaNode;
    let mut next2: *mut ArenaNode = 0 as *mut ArenaNode;
    let mut sizeDiff: u32_0 = 0;
    let mut overNext2: *mut ArenaNode = 0 as *mut ArenaNode;
    let mut localCopy: ArenaNode =
        ArenaNode{magic: 0,
                  isFree: 0,
                  size: 0,
                  next: 0 as *mut ArenaNode,
                  prev: 0 as *mut ArenaNode,
                  filename: 0 as *const libc::c_char,
                  line: 0,
                  threadId: 0,
                  arena: 0 as *mut Arena,
                  time: 0,
                  unk_28: [0; 8],};
    let mut blockSize: u32_0 = 0;
    let mut pad: s32 = 0;
    newSize =
        newSize.wrapping_add(0xf as libc::c_int as libc::c_uint) &
            !(0xf as libc::c_int) as libc::c_uint;
    osSyncPrintf(b"__osRealloc(%08x, %d)\n\x00" as *const u8 as
                     *const libc::c_char, ptr, newSize);
    ArenaImpl_Lock(arena);
    if ptr.is_null() {
        ptr = __osMalloc_NoLock(arena, newSize)
    } else if newSize == 0 as libc::c_int as libc::c_uint {
        __osFree_NoLock(arena, ptr);
        ptr = 0 as *mut libc::c_void
    } else {
        node =
            (ptr as
                 u32_0).wrapping_sub(::std::mem::size_of::<ArenaNode>() as
                                         libc::c_ulong) as *mut ArenaNode;
        if newSize == (*node).size {
            // "Does nothing because the memory block size does not change"
            osSyncPrintf(b"\xe3\x83\xa1\xe3\x83\xa2\xe3\x83\xaa\xe3\x83\x96\xe3\x83\xad\xe3\x83\x83\xe3\x82\xaf\xe3\x82\xb5\xe3\x82\xa4\xe3\x82\xba\xe3\x81\x8c\xe5\xa4\x89\xe3\x82\x8f\xe3\x82\x89\xe3\x81\xaa\xe3\x81\x84\xe3\x81\x9f\xe3\x82\x81\xe3\x81\xaa\xe3\x81\xab\xe3\x82\x82\xe3\x81\x97\xe3\x81\xbe\xe3\x81\x9b\xe3\x82\x93\n\x00"
                             as *const u8 as *const libc::c_char);
        } else if (*node).size < newSize {
            next = ArenaImpl_GetNextBlock(node);
            sizeDiff = newSize.wrapping_sub((*node).size);
            if next as u32_0 ==
                   (node as
                        u32_0).wrapping_add((*node).size).wrapping_add(::std::mem::size_of::<ArenaNode>()
                                                                           as
                                                                           libc::c_ulong)
                   && (*next).isFree as libc::c_int != 0 &&
                   (*next).size >= sizeDiff {
                // "Merge because there is a free block after the current memory block"
                osSyncPrintf(b"\xe7\x8f\xbe\xe3\x83\xa1\xe3\x83\xa2\xe3\x83\xaa\xe3\x83\x96\xe3\x83\xad\xe3\x83\x83\xe3\x82\xaf\xe3\x81\xae\xe5\xbe\x8c\xe3\x82\x8d\xe3\x81\xab\xe3\x83\x95\xe3\x83\xaa\xe3\x83\xbc\xe3\x83\x96\xe3\x83\xad\xe3\x83\x83\xe3\x82\xaf\xe3\x81\x8c\xe3\x81\x82\xe3\x82\x8b\xe3\x81\xae\xe3\x81\xa7\xe7\xb5\x90\xe5\x90\x88\xe3\x81\x97\xe3\x81\xbe\xe3\x81\x99\n\x00"
                                 as *const u8 as *const libc::c_char);
                (*next).size =
                    ((*next).size as libc::c_uint).wrapping_sub(sizeDiff) as
                        u32_0 as u32_0;
                overNext = ArenaImpl_GetNextBlock(next);
                newNext =
                    (next as u32_0).wrapping_add(sizeDiff) as *mut ArenaNode;
                if !overNext.is_null() { (*overNext).prev = newNext }
                (*node).next = newNext;
                (*node).size = newSize;
                func_801068B0(newNext as *mut libc::c_void,
                              next as *mut libc::c_void,
                              ::std::mem::size_of::<ArenaNode>() as
                                  libc::c_ulong as size_t);
                // memcpy
            } else {
                // "Allocate a new memory block and move the contents"
                osSyncPrintf(b"\xe6\x96\xb0\xe3\x81\x9f\xe3\x81\xab\xe3\x83\xa1\xe3\x83\xa2\xe3\x83\xaa\xe3\x83\x96\xe3\x83\xad\xe3\x83\x83\xe3\x82\xaf\xe3\x82\x92\xe7\xa2\xba\xe4\xbf\x9d\xe3\x81\x97\xe3\x81\xa6\xe5\x86\x85\xe5\xae\xb9\xe3\x82\x92\xe7\xa7\xbb\xe5\x8b\x95\xe3\x81\x97\xe3\x81\xbe\xe3\x81\x99\n\x00"
                                 as *const u8 as *const libc::c_char);
                newAlloc = __osMalloc_NoLock(arena, newSize);
                if !newAlloc.is_null() {
                    bcopy(ptr, newAlloc, (*node).size);
                    __osFree_NoLock(arena, ptr);
                }
                ptr = newAlloc
            }
        } else if newSize < (*node).size {
            next2 = ArenaImpl_GetNextBlock(node);
            if !next2.is_null() && (*next2).isFree as libc::c_int != 0 {
                blockSize =
                    (newSize.wrapping_add(0xf as libc::c_int as libc::c_uint)
                         &
                         !(0xf as libc::c_int) as
                             libc::c_uint).wrapping_add(::std::mem::size_of::<ArenaNode>()
                                                            as libc::c_ulong);
                // "Increased free block behind current memory block"
                osSyncPrintf(b"\xe7\x8f\xbe\xe3\x83\xa1\xe3\x83\xa2\xe3\x83\xaa\xe3\x83\x96\xe3\x83\xad\xe3\x83\x83\xe3\x82\xaf\xe3\x81\xae\xe5\xbe\x8c\xe3\x82\x8d\xe3\x81\xae\xe3\x83\x95\xe3\x83\xaa\xe3\x83\xbc\xe3\x83\x96\xe3\x83\xad\xe3\x83\x83\xe3\x82\xaf\xe3\x82\x92\xe5\xa4\xa7\xe3\x81\x8d\xe3\x81\x8f\xe3\x81\x97\xe3\x81\xbe\xe3\x81\x97\xe3\x81\x9f\n\x00"
                                 as *const u8 as *const libc::c_char);
                newNext2 =
                    (node as u32_0).wrapping_add(blockSize) as *mut ArenaNode;
                localCopy = *next2;
                *newNext2 = localCopy;
                (*newNext2).size =
                    ((*newNext2).size as
                         libc::c_uint).wrapping_add((*node).size.wrapping_sub(newSize))
                        as u32_0 as u32_0;
                (*node).next = newNext2;
                (*node).size = newSize;
                overNext2 = ArenaImpl_GetNextBlock(newNext2);
                if !overNext2.is_null() { (*overNext2).prev = newNext2 }
            } else if newSize.wrapping_add(::std::mem::size_of::<ArenaNode>()
                                               as libc::c_ulong) <
                          (*node).size {
                blockSize =
                    (newSize.wrapping_add(0xf as libc::c_int as libc::c_uint)
                         &
                         !(0xf as libc::c_int) as
                             libc::c_uint).wrapping_add(::std::mem::size_of::<ArenaNode>()
                                                            as libc::c_ulong);
                // "Generated because there is no free block after the current memory block"
                osSyncPrintf(b"\xe7\x8f\xbe\xe3\x83\xa1\xe3\x83\xa2\xe3\x83\xaa\xe3\x83\x96\xe3\x83\xad\xe3\x83\x83\xe3\x82\xaf\xe3\x81\xae\xe5\xbe\x8c\xe3\x82\x8d\xe3\x81\xab\xe3\x83\x95\xe3\x83\xaa\xe3\x83\xbc\xe3\x83\x96\xe3\x83\xad\xe3\x83\x83\xe3\x82\xaf\xe3\x81\x8c\xe3\x81\xaa\xe3\x81\x84\xe3\x81\xae\xe3\x81\xa7\xe7\x94\x9f\xe6\x88\x90\xe3\x81\x97\xe3\x81\xbe\xe3\x81\x99\n\x00"
                                 as *const u8 as *const libc::c_char);
                newNext2 =
                    (node as u32_0).wrapping_add(blockSize) as *mut ArenaNode;
                (*newNext2).next = ArenaImpl_GetNextBlock(node);
                (*newNext2).prev = node;
                (*newNext2).size = (*node).size.wrapping_sub(blockSize);
                (*newNext2).isFree = 1 as libc::c_int as s16;
                (*newNext2).magic = 0x7373 as libc::c_int as s16;
                (*node).next = newNext2;
                (*node).size = newSize;
                overNext2 = ArenaImpl_GetNextBlock(newNext2);
                if !overNext2.is_null() { (*overNext2).prev = newNext2 }
            } else {
                // "There is no room to generate free blocks"
                osSyncPrintf(b"\xe3\x83\x95\xe3\x83\xaa\xe3\x83\xbc\xe3\x83\x96\xe3\x83\xad\xe3\x83\x83\xe3\x82\xaf\xe7\x94\x9f\xe6\x88\x90\xe3\x81\x99\xe3\x82\x8b\xe3\x81\xa0\xe3\x81\x91\xe3\x81\xae\xe7\xa9\xba\xe3\x81\x8d\xe3\x81\x8c\xe3\x81\x82\xe3\x82\x8a\xe3\x81\xbe\xe3\x81\x9b\xe3\x82\x93\n\x00"
                                 as *const u8 as
                                 *const libc::c_char); // "Arena is not initalized"
                ptr = 0 as *mut libc::c_void
            }
        }
    } // "Arena contents (0x%08x)"
    ArenaImpl_Unlock(arena);
    return ptr;
}
#[no_mangle]
pub unsafe extern "C" fn __osReallocDebug(mut arena: *mut Arena,
                                          mut ptr: *mut libc::c_void,
                                          mut newSize: u32_0,
                                          mut file: *const libc::c_char,
                                          mut line: s32)
 -> *mut libc::c_void {
    return __osRealloc(arena, ptr, newSize);
}
#[no_mangle]
pub unsafe extern "C" fn ArenaImpl_GetSizes(mut arena: *mut Arena,
                                            mut outMaxFree: *mut u32_0,
                                            mut outFree: *mut u32_0,
                                            mut outAlloc: *mut u32_0) {
    let mut iter: *mut ArenaNode = 0 as *mut ArenaNode;
    ArenaImpl_Lock(arena);
    *outMaxFree = 0 as libc::c_int as u32_0;
    *outFree = 0 as libc::c_int as u32_0;
    *outAlloc = 0 as libc::c_int as u32_0;
    iter = (*arena).head;
    while !iter.is_null() {
        if (*iter).isFree != 0 {
            *outFree =
                (*outFree as libc::c_uint).wrapping_add((*iter).size) as u32_0
                    as u32_0;
            if *outMaxFree < (*iter).size { *outMaxFree = (*iter).size }
        } else {
            *outAlloc =
                (*outAlloc as libc::c_uint).wrapping_add((*iter).size) as
                    u32_0 as u32_0
        }
        iter = ArenaImpl_GetNextBlock(iter)
    }
    ArenaImpl_Unlock(arena);
}
#[no_mangle]
pub unsafe extern "C" fn __osDisplayArena(mut arena: *mut Arena) {
    let mut freeSize: u32_0 = 0;
    let mut allocatedSize: u32_0 = 0;
    let mut maxFree: u32_0 = 0;
    let mut iter: *mut ArenaNode = 0 as *mut ArenaNode;
    let mut next: *mut ArenaNode = 0 as *mut ArenaNode;
    if __osMallocIsInitalized(arena) == 0 {
        osSyncPrintf(b"\xe3\x82\xa2\xe3\x83\xaa\xe3\x83\xbc\xe3\x83\x8a\xe3\x81\xaf\xe5\x88\x9d\xe6\x9c\x9f\xe5\x8c\x96\xe3\x81\x95\xe3\x82\x8c\xe3\x81\xa6\xe3\x81\x84\xe3\x81\xbe\xe3\x81\x9b\xe3\x82\x93\n\x00"
                         as *const u8 as *const libc::c_char);
        return
    }
    ArenaImpl_Lock(arena);
    maxFree = 0 as libc::c_int as u32_0;
    freeSize = 0 as libc::c_int as u32_0;
    allocatedSize = 0 as libc::c_int as u32_0;
    osSyncPrintf(b"\xe3\x82\xa2\xe3\x83\xaa\xe3\x83\xbc\xe3\x83\x8a\xe3\x81\xae\xe5\x86\x85\xe5\xae\xb9 (0x%08x)\n\x00"
                     as *const u8 as *const libc::c_char, arena);
    // "Memory node range status size [time s ms us ns: TID: src: line]"
    osSyncPrintf(b"\xe3\x83\xa1\xe3\x83\xa2\xe3\x83\xaa\xe3\x83\x96\xe3\x83\xad\xe3\x83\x83\xe3\x82\xaf\xe7\xaf\x84\xe5\x9b\xb2 status \xe3\x82\xb5\xe3\x82\xa4\xe3\x82\xba  [\xe6\x99\x82\xe5\x88\xbb  s ms us ns: TID:src:\xe8\xa1\x8c]\n\x00"
                     as *const u8 as *const libc::c_char);
    iter = (*arena).head;
    while !iter.is_null() {
        if !iter.is_null() &&
               (*iter).magic as libc::c_int == 0x7373 as libc::c_int {
            next = (*iter).next;
            osSyncPrintf(b"%08x-%08x%c %s %08x\x00" as *const u8 as
                             *const libc::c_char, iter,
                         (iter as
                              u32_0).wrapping_add(::std::mem::size_of::<ArenaNode>()
                                                      as
                                                      libc::c_ulong).wrapping_add((*iter).size),
                         if next.is_null() {
                             '$' as i32
                         } else if iter != (*next).prev {
                             '!' as i32
                         } else { ' ' as i32 },
                         if (*iter).isFree as libc::c_int != 0 {
                             b"\xe7\xa9\xba\xe3\x81\x8d\x00" as *const u8 as
                                 *const libc::c_char
                         } else {
                             b"\xe7\xa2\xba\xe4\xbf\x9d\x00" as *const u8 as
                                 *const libc::c_char
                         }, (*iter).size);
            if (*iter).isFree == 0 {
                osSyncPrintf(b" [%016llu:%2d:%s:%d]\x00" as *const u8 as
                                 *const libc::c_char,
                             (*iter).time.wrapping_mul((1000000000 as
                                                            libc::c_longlong /
                                                            15625000 as
                                                                libc::c_longlong)
                                                           as
                                                           libc::c_ulonglong).wrapping_div((62500000
                                                                                                as
                                                                                                libc::c_longlong
                                                                                                *
                                                                                                3
                                                                                                    as
                                                                                                    libc::c_int
                                                                                                    as
                                                                                                    libc::c_longlong
                                                                                                /
                                                                                                4
                                                                                                    as
                                                                                                    libc::c_int
                                                                                                    as
                                                                                                    libc::c_longlong
                                                                                                /
                                                                                                15625000
                                                                                                    as
                                                                                                    libc::c_longlong)
                                                                                               as
                                                                                               libc::c_ulonglong),
                             (*iter).threadId,
                             if !(*iter).filename.is_null() {
                                 (*iter).filename
                             } else {
                                 b"**NULL**\x00" as *const u8 as
                                     *const libc::c_char
                             }, (*iter).line);
            }
            osSyncPrintf(b"\n\x00" as *const u8 as *const libc::c_char);
            if (*iter).isFree != 0 {
                freeSize =
                    (freeSize as libc::c_uint).wrapping_add((*iter).size) as
                        u32_0 as u32_0;
                if maxFree < (*iter).size { maxFree = (*iter).size }
            } else {
                allocatedSize =
                    (allocatedSize as libc::c_uint).wrapping_add((*iter).size)
                        as u32_0 as u32_0
            }
        } else {
            osSyncPrintf(b"%08x Block Invalid\n\x00" as *const u8 as
                             *const libc::c_char, iter);
            next = 0 as *mut ArenaNode
        }
        iter = next
    }
    // "Total reserved node size 0x%08x bytes"
    osSyncPrintf(b"\xe7\xa2\xba\xe4\xbf\x9d\xe3\x83\x96\xe3\x83\xad\xe3\x83\x83\xe3\x82\xaf\xe3\x82\xb5\xe3\x82\xa4\xe3\x82\xba\xe3\x81\xae\xe5\x90\x88\xe8\xa8\x88 0x%08x \xe3\x83\x90\xe3\x82\xa4\xe3\x83\x88\n\x00"
                     as *const u8 as *const libc::c_char, allocatedSize);
    // "Total free node size 0x%08x bytes"
    osSyncPrintf(b"\xe7\xa9\xba\xe3\x81\x8d\xe3\x83\x96\xe3\x83\xad\xe3\x83\x83\xe3\x82\xaf\xe3\x82\xb5\xe3\x82\xa4\xe3\x82\xba\xe3\x81\xae\xe5\x90\x88\xe8\xa8\x88 0x%08x \xe3\x83\x90\xe3\x82\xa4\xe3\x83\x88\n\x00"
                     as *const u8 as *const libc::c_char, freeSize);
    // "Maximum free node size 0x%08x bytes"
    osSyncPrintf(b"\xe6\x9c\x80\xe5\xa4\xa7\xe7\xa9\xba\xe3\x81\x8d\xe3\x83\x96\xe3\x83\xad\xe3\x83\x83\xe3\x82\xaf\xe3\x82\xb5\xe3\x82\xa4\xe3\x82\xba   0x%08x \xe3\x83\x90\xe3\x82\xa4\xe3\x83\x88\n\x00"
                     as *const u8 as *const libc::c_char, maxFree);
    ArenaImpl_Unlock(arena);
}
#[no_mangle]
pub unsafe extern "C" fn ArenaImpl_FaultClient(mut arena: *mut Arena) {
    let mut freeSize: u32_0 = 0;
    let mut allocatedSize: u32_0 = 0;
    let mut maxFree: u32_0 = 0;
    let mut iter: *mut ArenaNode = 0 as *mut ArenaNode;
    let mut next: *mut ArenaNode = 0 as *mut ArenaNode;
    FaultDrawer_Printf(b"ARENA INFO (0x%08x)\n\x00" as *const u8 as
                           *const libc::c_char, arena);
    if __osMallocIsInitalized(arena) == 0 {
        FaultDrawer_Printf(b"Arena is uninitalized\n\x00" as *const u8 as
                               *const libc::c_char, arena);
        return
    }
    maxFree = 0 as libc::c_int as u32_0;
    freeSize = 0 as libc::c_int as u32_0;
    allocatedSize = 0 as libc::c_int as u32_0;
    FaultDrawer_Printf(b"Memory Block Region status size\n\x00" as *const u8
                           as *const libc::c_char);
    iter = (*arena).head;
    while !iter.is_null() {
        if !iter.is_null() &&
               (*iter).magic as libc::c_int == 0x7373 as libc::c_int {
            next = (*iter).next;
            FaultDrawer_Printf(b"%08x-%08x%c %s %08x\x00" as *const u8 as
                                   *const libc::c_char, iter,
                               (iter as
                                    u32_0).wrapping_add(::std::mem::size_of::<ArenaNode>()
                                                            as
                                                            libc::c_ulong).wrapping_add((*iter).size),
                               if next.is_null() {
                                   '$' as i32
                               } else if iter != (*next).prev {
                                   '!' as i32
                               } else { ' ' as i32 },
                               if (*iter).isFree as libc::c_int != 0 {
                                   b"F\x00" as *const u8 as
                                       *const libc::c_char
                               } else {
                                   b"A\x00" as *const u8 as
                                       *const libc::c_char
                               }, (*iter).size);
            FaultDrawer_Printf(b"\n\x00" as *const u8 as *const libc::c_char);
            if (*iter).isFree != 0 {
                freeSize =
                    (freeSize as libc::c_uint).wrapping_add((*iter).size) as
                        u32_0 as u32_0;
                if maxFree < (*iter).size { maxFree = (*iter).size }
            } else {
                allocatedSize =
                    (allocatedSize as libc::c_uint).wrapping_add((*iter).size)
                        as u32_0 as u32_0
            }
        } else {
            FaultDrawer_SetFontColor(0xf801 as libc::c_int as u16_0);
            FaultDrawer_Printf(b"%08x Block Invalid\n\x00" as *const u8 as
                                   *const libc::c_char, iter);
            next = 0 as *mut ArenaNode
        }
        iter = next
    }
    FaultDrawer_SetFontColor(0x7f1 as libc::c_int as u16_0);
    FaultDrawer_Printf(b"Total Alloc Block Size  %08x\n\x00" as *const u8 as
                           *const libc::c_char, allocatedSize);
    FaultDrawer_Printf(b"Total Free Block Size   %08x\n\x00" as *const u8 as
                           *const libc::c_char, freeSize);
    FaultDrawer_Printf(b"Largest Free Block Size %08x\n\x00" as *const u8 as
                           *const libc::c_char, maxFree);
}
#[no_mangle]
pub unsafe extern "C" fn __osCheckArena(mut arena: *mut Arena) -> u32_0 {
    let mut iter: *mut ArenaNode = 0 as *mut ArenaNode;
    let mut error: u32_0 = 0 as libc::c_int as u32_0;
    ArenaImpl_Lock(arena);
    // "Checking the contents of the arena. . ． (%08x)"
    osSyncPrintf(b"\xe3\x82\xa2\xe3\x83\xaa\xe3\x83\xbc\xe3\x83\x8a\xe3\x81\xae\xe5\x86\x85\xe5\xae\xb9\xe3\x82\x92\xe3\x83\x81\xe3\x82\xa7\xe3\x83\x83\xe3\x82\xaf\xe3\x81\x97\xe3\x81\xa6\xe3\x81\x84\xe3\x81\xbe\xe3\x81\x99\xef\xbc\x8e\xef\xbc\x8e\xef\xbc\x8e (%08x)\n\x00"
                     as *const u8 as *const libc::c_char, arena);
    iter = (*arena).head;
    while !iter.is_null() {
        if !iter.is_null() &&
               (*iter).magic as libc::c_int == 0x7373 as libc::c_int {
            // "Oops!! (%08x %08x)"
            osSyncPrintf(b"\x1b[41;37m\xe3\x81\x8a\xe3\x81\x8a\xe3\x81\xa3\xe3\x81\xa8\xef\xbc\x81\xef\xbc\x81 (%08x %08x)\n\x1b[m\x00"
                             as *const u8 as *const libc::c_char, iter,
                         (*iter).magic as libc::c_int);
            error = 1 as libc::c_int as u32_0;
            break ;
        } else { iter = ArenaImpl_GetNextBlock(iter) }
    }
    if error == 0 as libc::c_int as libc::c_uint {
        osSyncPrintf(b"\xe3\x82\xa2\xe3\x83\xaa\xe3\x83\xbc\xe3\x83\x8a\xe3\x81\xaf\xe3\x81\xbe\xe3\x81\xa0\xe3\x80\x81\xe3\x81\x84\xe3\x81\x91\xe3\x81\x9d\xe3\x81\x86\xe3\x81\xa7\xe3\x81\x99\n\x00"
                         as *const u8 as *const libc::c_char);
        // "The arena is still going well"
    }
    ArenaImpl_Unlock(arena);
    return error;
}
#[no_mangle]
pub unsafe extern "C" fn func_800FF334(mut arena: *mut Arena) -> u8_0 {
    return (*arena).unk_20;
}
