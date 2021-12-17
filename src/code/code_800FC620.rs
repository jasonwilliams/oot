#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case,
         non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(const_raw_ptr_to_usize_cast, register_tool)]
extern "C" {
    #[no_mangle]
    fn DmaMgr_SendRequest0(ram: u32_0, vrom: u32_0, size: u32_0) -> s32;
    #[no_mangle]
    fn osSyncPrintf(fmt: *const libc::c_char, _: ...);
    #[no_mangle]
    fn bzero(__s: *mut libc::c_void, __n: u32_0);
    #[no_mangle]
    fn osWritebackDCache(vaddr: *mut libc::c_void, nbytes: s32);
    #[no_mangle]
    fn osInvalICache(vaddr: *mut libc::c_void, nbytes: s32);
    #[no_mangle]
    fn Overlay_Relocate(allocatedVRamAddress: *mut libc::c_void,
                        overlayInfo: *mut OverlayRelocationSection,
                        vRamAddress: *mut libc::c_void);
    #[no_mangle]
    fn SystemArena_Init(start: *mut libc::c_void, size: u32_0);
    #[no_mangle]
    fn __osMallocDebug(arena: *mut Arena, size: u32_0,
                       file: *const libc::c_char, line: s32)
     -> *mut libc::c_void;
    #[no_mangle]
    fn __osFree(arena: *mut Arena, ptr: *mut libc::c_void);
    #[no_mangle]
    static mut gOverlayLogSeverity: s32;
    #[no_mangle]
    static mut gSystemArena: Arena;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct OverlayRelocationSection {
    pub textSize: u32_0,
    pub dataSize: u32_0,
    pub rodataSize: u32_0,
    pub bssSize: u32_0,
    pub nRelocations: u32_0,
    pub relocations: [u32_0; 1],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct InitFunc {
    pub nextOffset: s32,
    pub func: Option<unsafe extern "C" fn() -> ()>,
}
pub type arg3_800FC868
    =
    Option<unsafe extern "C" fn(_: *mut libc::c_void) -> ()>;
pub type arg3_800FC8D8
    =
    Option<unsafe extern "C" fn(_: *mut libc::c_void, _: u32_0) -> ()>;
pub type arg3_800FC948
    =
    Option<unsafe extern "C" fn(_: *mut libc::c_void, _: u32_0, _: u32_0,
                                _: u32_0, _: u32_0, _: u32_0, _: u32_0,
                                _: u32_0, _: u32_0) -> ()>;
pub type arg3_800FCA18
    =
    Option<unsafe extern "C" fn(_: *mut libc::c_void, _: u32_0) -> ()>;
// .data
#[no_mangle]
pub static mut sInitFuncs: *mut libc::c_void =
    0 as *const libc::c_void as *mut libc::c_void;
#[no_mangle]
pub static mut sNew: [libc::c_char; 3] =
    ['n' as i32 as libc::c_char, 'e' as i32 as libc::c_char,
     'w' as i32 as libc::c_char];
#[no_mangle]
pub static mut D_80134488: [libc::c_char; 24] =
    [0 as libc::c_int as libc::c_char, 0 as libc::c_int as libc::c_char,
     0 as libc::c_int as libc::c_char, 0 as libc::c_int as libc::c_char,
     0 as libc::c_int as libc::c_char, 0 as libc::c_int as libc::c_char,
     0 as libc::c_int as libc::c_char, 0 as libc::c_int as libc::c_char,
     0x7f as libc::c_int as libc::c_char, 0x80 as libc::c_int as libc::c_char,
     0 as libc::c_int as libc::c_char, 0 as libc::c_int as libc::c_char,
     0xff as libc::c_int as libc::c_char, 0x80 as libc::c_int as libc::c_char,
     0 as libc::c_int as libc::c_char, 0 as libc::c_int as libc::c_char,
     0 as libc::c_int as libc::c_char, 0 as libc::c_int as libc::c_char,
     0 as libc::c_int as libc::c_char, 0 as libc::c_int as libc::c_char,
     0x80 as libc::c_int as libc::c_char, 0 as libc::c_int as libc::c_char,
     0 as libc::c_int as libc::c_char, 0 as libc::c_int as libc::c_char];
#[no_mangle]
pub unsafe extern "C" fn Overlay_Load(mut vRomStart: u32_0,
                                      mut vRomEnd: u32_0,
                                      mut vRamStart: *mut libc::c_void,
                                      mut vRamEnd: *mut libc::c_void,
                                      mut allocatedVRamAddr:
                                          *mut libc::c_void) -> s32 {
    let mut pad: s32 = 0;
    let mut end: u32_0 = 0;
    let mut bssSize: u32_0 = 0;
    let mut ovl: *mut OverlayRelocationSection =
        0 as *mut OverlayRelocationSection;
    let mut relocCnt: u32_0 = 0;
    let mut ovlOffset: u32_0 = 0;
    let mut size: u32_0 = 0;
    if gOverlayLogSeverity >= 3 as libc::c_int {
        // "Start loading dynamic link function"
        osSyncPrintf(b"\n\xe3\x83\x80\xe3\x82\xa4\xe3\x83\x8a\xe3\x83\x9f\xe3\x83\x83\xe3\x82\xaf\xe3\x83\xaa\xe3\x83\xb3\xe3\x82\xaf\xe3\x83\x95\xe3\x82\xa1\xe3\x83\xb3\xe3\x82\xaf\xe3\x82\xb7\xe3\x83\xa7\xe3\x83\xb3\xe3\x81\xae\xe3\x83\xad\xe3\x83\xbc\xe3\x83\x89\xe3\x82\x92\xe9\x96\x8b\xe5\xa7\x8b\xe3\x81\x97\xe3\x81\xbe\xe3\x81\x99\n\x00"
                         as *const u8 as *const libc::c_char);
    }
    if gOverlayLogSeverity >= 3 as libc::c_int {
        size = vRomEnd.wrapping_sub(vRomStart);
        // "DMA transfer of TEXT, DATA, RODATA + rel (%08x-%08x)"
        osSyncPrintf(b"TEXT,DATA,RODATA+rel\xe3\x82\x92\xef\xbc\xa4\xef\xbc\xad\xef\xbc\xa1\xe8\xbb\xa2\xe9\x80\x81\xe3\x81\x97\xe3\x81\xbe\xe3\x81\x99(%08x-%08x)\n\x00"
                         as *const u8 as *const libc::c_char,
                     allocatedVRamAddr,
                     (allocatedVRamAddr as u32_0).wrapping_add(size));
    }
    size = vRomEnd.wrapping_sub(vRomStart);
    end = (allocatedVRamAddr as u32_0).wrapping_add(size);
    DmaMgr_SendRequest0(allocatedVRamAddr as u32_0, vRomStart, size);
    ovlOffset =
        *(end as *mut s32).offset(-(1 as libc::c_int) as isize) as u32_0;
    ovl = end.wrapping_sub(ovlOffset) as *mut OverlayRelocationSection;
    if gOverlayLogSeverity >= 3 as libc::c_int {
        osSyncPrintf(b"TEXT(%08x), DATA(%08x), RODATA(%08x), BSS(%08x)\n\x00"
                         as *const u8 as *const libc::c_char, (*ovl).textSize,
                     (*ovl).dataSize, (*ovl).rodataSize, (*ovl).bssSize);
    }
    if gOverlayLogSeverity >= 3 as libc::c_int {
        osSyncPrintf(b"\xe3\x83\xaa\xe3\x83\xad\xe3\x82\xb1\xe3\x83\xbc\xe3\x82\xb7\xe3\x83\xa7\xe3\x83\xb3\xe3\x81\x97\xe3\x81\xbe\xe3\x81\x99\n\x00"
                         as *const u8 as *const libc::c_char);
        // "Relocate"
    }
    Overlay_Relocate(allocatedVRamAddr, ovl, vRamStart);
    bssSize = (*ovl).bssSize;
    if bssSize != 0 as libc::c_int as libc::c_uint {
        if gOverlayLogSeverity >= 3 as libc::c_int {
            // "Clear BSS area (% 08x-% 08x)"
            osSyncPrintf(b"BSS\xe9\xa0\x98\xe5\x9f\x9f\xe3\x82\x92\xe3\x82\xaf\xe3\x83\xaa\xe3\x82\xa2\xe3\x81\x97\xe3\x81\xbe\xe3\x81\x99(%08x-%08x)\n\x00"
                             as *const u8 as *const libc::c_char, end,
                         end.wrapping_add((*ovl).bssSize));
        }
        size = (*ovl).bssSize;
        bssSize = size;
        bzero(end as *mut libc::c_void, bssSize);
        relocCnt = (*ovl).nRelocations
        // suppresses set but unused warning
    }
    size =
        (&mut *(*ovl).relocations.as_mut_ptr().offset((*ovl).nRelocations as
                                                          isize) as *mut u32_0
             as u32_0).wrapping_sub(ovl as u32_0);
    if gOverlayLogSeverity >= 3 as libc::c_int {
        // "Clear REL area (%08x-%08x)"
        osSyncPrintf(b"REL\xe9\xa0\x98\xe5\x9f\x9f\xe3\x82\x92\xe3\x82\xaf\xe3\x83\xaa\xe3\x82\xa2\xe3\x81\x97\xe3\x81\xbe\xe3\x81\x99(%08x-%08x)\n\x00"
                         as *const u8 as *const libc::c_char, ovl,
                     (ovl as u32_0).wrapping_add(size));
    }
    bzero(ovl as *mut libc::c_void, size);
    size = (vRamEnd as u32_0).wrapping_sub(vRamStart as u32_0);
    osWritebackDCache(allocatedVRamAddr, size as s32);
    osInvalICache(allocatedVRamAddr, size as s32);
    if gOverlayLogSeverity >= 3 as libc::c_int {
        // "Finish loading dynamic link function"
        osSyncPrintf(b"\xe3\x83\x80\xe3\x82\xa4\xe3\x83\x8a\xe3\x83\x9f\xe3\x83\x83\xe3\x82\xaf\xe3\x83\xaa\xe3\x83\xb3\xe3\x82\xaf\xe3\x83\x95\xe3\x82\xa1\xe3\x83\xb3\xe3\x82\xaf\xe3\x82\xb7\xe3\x83\xa7\xe3\x83\xb3\xe3\x81\xae\xe3\x83\xad\xe3\x83\xbc\xe3\x83\x89\xe3\x82\x92\xe7\xb5\x82\xe4\xba\x86\xe3\x81\x97\xe3\x81\xbe\xe3\x81\x99\n\n\x00"
                         as *const u8 as *const libc::c_char);
    }
    return size as s32;
}
// possibly some kind of new() function
#[no_mangle]
pub unsafe extern "C" fn func_800FC800(mut size: u32_0) -> *mut libc::c_void {
    if size == 0 as libc::c_int as libc::c_uint {
        size = 1 as libc::c_int as u32_0
    }
    return __osMallocDebug(&mut gSystemArena, size, sNew.as_mut_ptr(),
                           0 as libc::c_int);
}
// possible some kind of delete() function
#[no_mangle]
pub unsafe extern "C" fn func_800FC83C(mut ptr: *mut libc::c_void) {
    if !ptr.is_null() { __osFree(&mut gSystemArena, ptr); };
}
#[no_mangle]
pub unsafe extern "C" fn func_800FC868(mut blk: *mut libc::c_void,
                                       mut nBlk: u32_0, mut blkSize: u32_0,
                                       mut arg3: arg3_800FC868) {
    let mut pos: u32_0 = 0;
    pos = blk as u32_0;
    while pos < (blk as u32_0).wrapping_add(nBlk.wrapping_mul(blkSize)) {
        arg3.expect("non-null function pointer")(pos as *mut libc::c_void);
        pos = pos.wrapping_add(blkSize & !(0 as libc::c_int) as libc::c_uint)
    };
}
#[no_mangle]
pub unsafe extern "C" fn func_800FC8D8(mut blk: *mut libc::c_void,
                                       mut nBlk: u32_0, mut blkSize: s32,
                                       mut arg3: arg3_800FC8D8) {
    let mut pos: u32_0 = 0;
    pos = blk as u32_0;
    while pos <
              (blk as
                   u32_0).wrapping_add(nBlk.wrapping_mul(blkSize as
                                                             libc::c_uint)) {
        arg3.expect("non-null function pointer")(pos as *mut libc::c_void,
                                                 2 as libc::c_int as u32_0);
        pos =
            pos.wrapping_add((blkSize & !(0 as libc::c_int)) as libc::c_uint)
    };
}
#[no_mangle]
pub unsafe extern "C" fn func_800FC948(mut blk: *mut libc::c_void,
                                       mut nBlk: u32_0, mut blkSize: u32_0,
                                       mut arg3: arg3_800FC948)
 -> *mut libc::c_void {
    let mut pos: u32_0 = 0;
    if blk.is_null() { blk = func_800FC800(nBlk.wrapping_mul(blkSize)) }
    if !blk.is_null() && arg3.is_some() {
        pos = blk as u32_0;
        while pos < (blk as u32_0).wrapping_add(nBlk.wrapping_mul(blkSize)) {
            arg3.expect("non-null function pointer")(pos as *mut libc::c_void,
                                                     0 as libc::c_int as
                                                         u32_0,
                                                     0 as libc::c_int as
                                                         u32_0,
                                                     0 as libc::c_int as
                                                         u32_0,
                                                     0 as libc::c_int as
                                                         u32_0,
                                                     0 as libc::c_int as
                                                         u32_0,
                                                     0 as libc::c_int as
                                                         u32_0,
                                                     0 as libc::c_int as
                                                         u32_0,
                                                     0 as libc::c_int as
                                                         u32_0);
            pos =
                pos.wrapping_add(blkSize &
                                     !(0 as libc::c_int) as libc::c_uint)
        }
    }
    return blk;
}
#[no_mangle]
pub unsafe extern "C" fn func_800FCA18(mut blk: *mut libc::c_void,
                                       mut nBlk: u32_0, mut blkSize: u32_0,
                                       mut arg3: arg3_800FCA18,
                                       mut arg4: s32) {
    let mut pos: u32_0 = 0;
    let mut end: u32_0 = 0;
    let mut masked_arg2: s32 = 0;
    if blk.is_null() { return }
    if arg3.is_some() {
        end = blk as u32_0;
        masked_arg2 = (blkSize & !(0 as libc::c_int) as libc::c_uint) as s32;
        pos = end.wrapping_add(nBlk.wrapping_mul(blkSize));
        (masked_arg2) != 0;
        while pos > end {
            pos =
                (pos as
                     libc::c_uint).wrapping_sub(masked_arg2 as libc::c_uint)
                    as u32_0 as u32_0;
            arg3.expect("non-null function pointer")(pos as *mut libc::c_void,
                                                     2 as libc::c_int as
                                                         u32_0);
        }
        (masked_arg2) == 0;
    }
    if arg4 != 0 as libc::c_int { func_800FC83C(blk); };
}
#[no_mangle]
pub unsafe extern "C" fn func_800FCB34() {
    let mut initFunc: *mut InitFunc =
        &mut sInitFuncs as *mut *mut libc::c_void as *mut InitFunc;
    let mut nextOffset: u32_0 = (*initFunc).nextOffset as u32_0;
    let mut prev: *mut InitFunc = 0 as *mut InitFunc;
    while nextOffset != 0 as libc::c_int as libc::c_uint {
        initFunc =
            (initFunc as s32 as libc::c_uint).wrapping_add(nextOffset) as
                *mut InitFunc;
        if (*initFunc).func.is_some() {
            Some((*initFunc).func.expect("non-null function pointer")).expect("non-null function pointer")();
        }
        nextOffset = (*initFunc).nextOffset as u32_0;
        (*initFunc).nextOffset = prev as s32;
        prev = initFunc
    }
    sInitFuncs = prev as *mut libc::c_void;
}
#[no_mangle]
pub unsafe extern "C" fn SystemHeap_Init(mut start: *mut libc::c_void,
                                         mut size: u32_0) {
    SystemArena_Init(start, size);
    func_800FCB34();
}
