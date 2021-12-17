#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case,
         non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(const_raw_ptr_to_usize_cast, ptr_wrapping_offset_from,
           register_tool)]
extern "C" {
    #[no_mangle]
    fn ViConfig_UpdateVi(mode: u32_0);
    #[no_mangle]
    fn DmaMgr_Init();
    #[no_mangle]
    fn DmaMgr_SendRequest1(ram0: *mut libc::c_void, vrom: u32_0, size: u32_0,
                           file: *const libc::c_char, line: s32) -> s32;
    #[no_mangle]
    fn osSyncPrintf(fmt: *const libc::c_char, _: ...);
    #[no_mangle]
    fn StackCheck_Init(entry: *mut StackEntry, stackTop: *mut libc::c_void,
                       stackBottom: *mut libc::c_void, initValue: u32_0,
                       minSpace: s32, name: *const libc::c_char);
    #[no_mangle]
    fn bzero(__s: *mut libc::c_void, __n: u32_0);
    #[no_mangle]
    fn osCreateThread(thread: *mut OSThread, id: OSId,
                      entry:
                          Option<unsafe extern "C" fn(_: *mut libc::c_void)
                                     -> ()>, arg: *mut libc::c_void,
                      sp: *mut libc::c_void, pri: OSPri);
    #[no_mangle]
    fn osCreatePiManager(pri: OSPri, cmdQ: *mut OSMesgQueue,
                         cmdBuf: *mut OSMesg, cmdMsgCnt: s32);
    #[no_mangle]
    fn osViBlack(active: u8_0);
    #[no_mangle]
    fn osViSetMode(mode: *mut OSViMode);
    #[no_mangle]
    fn osSetThreadPri(thread: *mut OSThread, pri: OSPri);
    #[no_mangle]
    fn osViSwapBuffer(vaddr: *mut libc::c_void);
    #[no_mangle]
    fn osGetTime() -> OSTime;
    #[no_mangle]
    fn osCreateViManager(pri: OSPri);
    #[no_mangle]
    fn osStartThread(thread: *mut OSThread);
    #[no_mangle]
    fn Main(arg: *mut libc::c_void);
    #[no_mangle]
    static mut gBuildTeam: [u8_0; 0];
    #[no_mangle]
    static mut gBuildDate: [u8_0; 0];
    #[no_mangle]
    static mut gBuildMakeOption: [u8_0; 0];
    #[no_mangle]
    static mut osMemSize: u32_0;
    #[no_mangle]
    static mut _bootSegmentEnd: [u8_0; 0];
    #[no_mangle]
    static mut gSystemHeap: [u8_0; 0];
    #[no_mangle]
    static mut gAudioHeap: [u8_0; 229376];
    #[no_mangle]
    static mut osTvType: u32_0;
    #[no_mangle]
    static mut osViModeNtscLan1: OSViMode;
    #[no_mangle]
    static mut osViModeMpalLan1: OSViMode;
    #[no_mangle]
    static mut osViModeFpalLan1: OSViMode;
    #[no_mangle]
    static mut _codeSegmentStart: [u8_0; 0];
    #[no_mangle]
    static mut _codeSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _codeSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut _codeSegmentBssEnd: [u8_0; 0];
    #[no_mangle]
    static mut _codeSegmentBssStart: [u8_0; 0];
}
pub type s8 = libc::c_schar;
pub type u8_0 = libc::c_uchar;
pub type u16_0 = libc::c_ushort;
pub type s32 = libc::c_int;
pub type u32_0 = libc::c_uint;
pub type u64_0 = libc::c_ulonglong;
pub type vu8 = u8_0;
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
pub struct OSViCommonRegs {
    pub ctrl: u32_0,
    pub width: u32_0,
    pub burst: u32_0,
    pub vSync: u32_0,
    pub hSync: u32_0,
    pub leap: u32_0,
    pub hStart: u32_0,
    pub xScale: u32_0,
    pub vCurrent: u32_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct OSViFieldRegs {
    pub origin: u32_0,
    pub yScale: u32_0,
    pub vStart: u32_0,
    pub vBurst: u32_0,
    pub vIntr: u32_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct OSViMode {
    pub type_0: u8_0,
    pub comRegs: OSViCommonRegs,
    pub fldRegs: [OSViFieldRegs; 2],
}
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
#[no_mangle]
pub static mut gMainThread: OSThread =
    OSThread{next: 0 as *const OSThread as *mut OSThread,
             priority: 0,
             queue: 0 as *const *mut OSThread as *mut *mut OSThread,
             tlnext: 0 as *const OSThread as *mut OSThread,
             state: 0,
             flags: 0,
             id: 0,
             fp: 0,
             thprof: 0 as *const __OSThreadprofile as *mut __OSThreadprofile,
             context:
                 __OSThreadContext{at: 0,
                                   v0: 0,
                                   v1: 0,
                                   a0: 0,
                                   a1: 0,
                                   a2: 0,
                                   a3: 0,
                                   t0: 0,
                                   t1: 0,
                                   t2: 0,
                                   t3: 0,
                                   t4: 0,
                                   t5: 0,
                                   t6: 0,
                                   t7: 0,
                                   s0: 0,
                                   s1: 0,
                                   s2: 0,
                                   s3: 0,
                                   s4: 0,
                                   s5: 0,
                                   s6: 0,
                                   s7: 0,
                                   t8: 0,
                                   t9: 0,
                                   gp: 0,
                                   sp: 0,
                                   s8: 0,
                                   ra: 0,
                                   lo: 0,
                                   hi: 0,
                                   sr: 0,
                                   pc: 0,
                                   cause: 0,
                                   badvaddr: 0,
                                   rcp: 0,
                                   fpcsr: 0,
                                   fp0:
                                       __OSfp{f:
                                                  C2RustUnnamed{f_odd: 0.,
                                                                f_even:
                                                                    0.,},},
                                   fp2:
                                       __OSfp{f:
                                                  C2RustUnnamed{f_odd: 0.,
                                                                f_even:
                                                                    0.,},},
                                   fp4:
                                       __OSfp{f:
                                                  C2RustUnnamed{f_odd: 0.,
                                                                f_even:
                                                                    0.,},},
                                   fp6:
                                       __OSfp{f:
                                                  C2RustUnnamed{f_odd: 0.,
                                                                f_even:
                                                                    0.,},},
                                   fp8:
                                       __OSfp{f:
                                                  C2RustUnnamed{f_odd: 0.,
                                                                f_even:
                                                                    0.,},},
                                   fp10:
                                       __OSfp{f:
                                                  C2RustUnnamed{f_odd: 0.,
                                                                f_even:
                                                                    0.,},},
                                   fp12:
                                       __OSfp{f:
                                                  C2RustUnnamed{f_odd: 0.,
                                                                f_even:
                                                                    0.,},},
                                   fp14:
                                       __OSfp{f:
                                                  C2RustUnnamed{f_odd: 0.,
                                                                f_even:
                                                                    0.,},},
                                   fp16:
                                       __OSfp{f:
                                                  C2RustUnnamed{f_odd: 0.,
                                                                f_even:
                                                                    0.,},},
                                   fp18:
                                       __OSfp{f:
                                                  C2RustUnnamed{f_odd: 0.,
                                                                f_even:
                                                                    0.,},},
                                   fp20:
                                       __OSfp{f:
                                                  C2RustUnnamed{f_odd: 0.,
                                                                f_even:
                                                                    0.,},},
                                   fp22:
                                       __OSfp{f:
                                                  C2RustUnnamed{f_odd: 0.,
                                                                f_even:
                                                                    0.,},},
                                   fp24:
                                       __OSfp{f:
                                                  C2RustUnnamed{f_odd: 0.,
                                                                f_even:
                                                                    0.,},},
                                   fp26:
                                       __OSfp{f:
                                                  C2RustUnnamed{f_odd: 0.,
                                                                f_even:
                                                                    0.,},},
                                   fp28:
                                       __OSfp{f:
                                                  C2RustUnnamed{f_odd: 0.,
                                                                f_even:
                                                                    0.,},},
                                   fp30:
                                       __OSfp{f:
                                                  C2RustUnnamed{f_odd: 0.,
                                                                f_even:
                                                                    0.,},},},};
#[no_mangle]
pub static mut sMainStack: [u8_0; 2304] = [0; 2304];
#[no_mangle]
pub static mut sMainStackInfo: StackEntry =
    StackEntry{next: 0 as *const StackEntry as *mut StackEntry,
               prev: 0 as *const StackEntry as *mut StackEntry,
               head: 0,
               tail: 0,
               initValue: 0,
               minSpace: 0,
               name: 0 as *const libc::c_char,};
#[no_mangle]
pub static mut sPiMgrCmdBuff: [OSMesg; 50] =
    [0 as *const libc::c_void as *mut libc::c_void; 50];
#[no_mangle]
pub static mut gPiMgrCmdQ: OSMesgQueue =
    OSMesgQueue{mtqueue: 0 as *const OSThread as *mut OSThread,
                fullqueue: 0 as *const OSThread as *mut OSThread,
                validCount: 0,
                first: 0,
                msgCount: 0,
                msg: 0 as *const OSMesg as *mut OSMesg,};
#[no_mangle]
pub static mut gViConfigMode: OSViMode =
    OSViMode{type_0: 0,
             comRegs:
                 OSViCommonRegs{ctrl: 0,
                                width: 0,
                                burst: 0,
                                vSync: 0,
                                hSync: 0,
                                leap: 0,
                                hStart: 0,
                                xScale: 0,
                                vCurrent: 0,},
             fldRegs:
                 [OSViFieldRegs{origin: 0,
                                yScale: 0,
                                vStart: 0,
                                vBurst: 0,
                                vIntr: 0,}; 2],};
#[no_mangle]
pub static mut D_80013960: u8_0 = 0;
#[no_mangle]
pub static mut D_80009430: s8 = 1 as libc::c_int as s8;
#[no_mangle]
pub static mut gViConfigUseDefault: vu8 = 1 as libc::c_int as u8_0;
#[no_mangle]
pub static mut gViConfigAdditionalScanLines: u8_0 = 0 as libc::c_int as u8_0;
#[no_mangle]
pub static mut gViConfigFeatures: u32_0 =
    (0x40 as libc::c_int | 0x2 as libc::c_int) as u32_0;
#[no_mangle]
pub static mut gViConfigXScale: f32_0 = 1.0f64 as f32_0;
#[no_mangle]
pub static mut gViConfigYScale: f32_0 = 1.0f64 as f32_0;
#[no_mangle]
pub unsafe extern "C" fn Main_ThreadEntry(mut arg: *mut libc::c_void) {
    let mut time: OSTime =
        0; // ! @bug Invalid vram address (probably intended to be 0x803DA800)
    osSyncPrintf(b"mainx \xe5\xae\x9f\xe8\xa1\x8c\xe9\x96\x8b\xe5\xa7\x8b\n\x00"
                     as *const u8 as *const libc::c_char);
    DmaMgr_Init();
    osSyncPrintf(b"code\xe3\x82\xbb\xe3\x82\xb0\xe3\x83\xa1\xe3\x83\xb3\xe3\x83\x88\xe3\x83\xad\xe3\x83\xbc\xe3\x83\x89\xe4\xb8\xad...\x00"
                     as *const u8 as *const libc::c_char);
    time = osGetTime();
    DmaMgr_SendRequest1(_codeSegmentStart.as_mut_ptr() as *mut libc::c_void,
                        _codeSegmentRomStart.as_mut_ptr() as u32_0,
                        _codeSegmentRomEnd.as_mut_ptr().wrapping_offset_from(_codeSegmentRomStart.as_mut_ptr())
                            as libc::c_int as u32_0,
                        b"../idle.c\x00" as *const u8 as *const libc::c_char,
                        238 as libc::c_int);
    time =
        (time as libc::c_ulonglong).wrapping_sub(osGetTime()) as OSTime as
            OSTime;
    osSyncPrintf(b"\rcode\xe3\x82\xbb\xe3\x82\xb0\xe3\x83\xa1\xe3\x83\xb3\xe3\x83\x88\xe3\x83\xad\xe3\x83\xbc\xe3\x83\x89\xe4\xb8\xad...\xe5\xae\x8c\xe4\xba\x86\n\x00"
                     as *const u8 as *const libc::c_char);
    osSyncPrintf(b"\xe8\xbb\xa2\xe9\x80\x81\xe6\x99\x82\xe9\x96\x93 %6.3f\n\x00"
                     as *const u8 as *const libc::c_char);
    bzero(_codeSegmentBssStart.as_mut_ptr() as *mut libc::c_void,
          _codeSegmentBssEnd.as_mut_ptr().wrapping_offset_from(_codeSegmentBssStart.as_mut_ptr())
              as libc::c_int as u32_0);
    osSyncPrintf(b"code\xe3\x82\xbb\xe3\x82\xb0\xe3\x83\xa1\xe3\x83\xb3\xe3\x83\x88BSS\xe3\x82\xaf\xe3\x83\xaa\xe3\x82\xa2\xe5\xae\x8c\xe4\xba\x86\n\x00"
                     as *const u8 as *const libc::c_char);
    Main(arg);
    osSyncPrintf(b"mainx \xe5\xae\x9f\xe8\xa1\x8c\xe7\xb5\x82\xe4\xba\x86\n\x00"
                     as *const u8 as *const libc::c_char);
}
#[no_mangle]
pub unsafe extern "C" fn Idle_ThreadEntry(mut arg: *mut libc::c_void) {
    osSyncPrintf(b"\xe3\x82\xa2\xe3\x82\xa4\xe3\x83\x89\xe3\x83\xab\xe3\x82\xb9\xe3\x83\xac\xe3\x83\x83\xe3\x83\x89(idleproc)\xe5\xae\x9f\xe8\xa1\x8c\xe9\x96\x8b\xe5\xa7\x8b\n\x00"
                     as *const u8 as *const libc::c_char);
    osSyncPrintf(b"\xe4\xbd\x9c\xe8\xa3\xbd\xe8\x80\x85    : %s\n\x00" as
                     *const u8 as *const libc::c_char,
                 gBuildTeam.as_mut_ptr());
    osSyncPrintf(b"\xe4\xbd\x9c\xe6\x88\x90\xe6\x97\xa5\xe6\x99\x82  : %s\n\x00"
                     as *const u8 as *const libc::c_char,
                 gBuildDate.as_mut_ptr());
    osSyncPrintf(b"MAKEOPTION: %s\n\x00" as *const u8 as *const libc::c_char,
                 gBuildMakeOption.as_mut_ptr());
    osSyncPrintf(b"\x1b[32m\x00" as *const u8 as *const libc::c_char);
    osSyncPrintf(b"\xef\xbc\xb2\xef\xbc\xa1\xef\xbc\xad\xe3\x82\xb5\xe3\x82\xa4\xe3\x82\xba\xe3\x81\xaf %d \xe3\x82\xad\xe3\x83\xad\xe3\x83\x90\xe3\x82\xa4\xe3\x83\x88\xe3\x81\xa7\xe3\x81\x99(osMemSize/osGetMemSize)\n\x00"
                     as *const u8 as *const libc::c_char,
                 osMemSize as s32 / 1024 as libc::c_int);
    osSyncPrintf(b"_bootSegmentEnd(%08x) \xe4\xbb\xa5\xe9\x99\x8d\xe3\x81\xae\xef\xbc\xb2\xef\xbc\xa1\xef\xbc\xad\xe9\xa0\x98\xe5\x9f\x9f\xe3\x81\xaf\xe3\x82\xaf\xe3\x83\xaa\xe3\x82\xa2\xe3\x81\x95\xe3\x82\x8c\xe3\x81\xbe\xe3\x81\x97\xe3\x81\x9f(boot)\n\x00"
                     as *const u8 as *const libc::c_char,
                 _bootSegmentEnd.as_mut_ptr());
    osSyncPrintf(b"\xef\xbc\xba\xe3\x83\x90\xe3\x83\x83\xe3\x83\x95\xe3\x82\xa1\xe3\x81\xae\xe3\x82\xb5\xe3\x82\xa4\xe3\x82\xba\xe3\x81\xaf %d \xe3\x82\xad\xe3\x83\xad\xe3\x83\x90\xe3\x82\xa4\xe3\x83\x88\xe3\x81\xa7\xe3\x81\x99\n\x00"
                     as *const u8 as *const libc::c_char,
                 0x96 as libc::c_int);
    osSyncPrintf(b"\xe3\x83\x80\xe3\x82\xa4\xe3\x83\x8a\xe3\x83\x9f\xe3\x83\x83\xe3\x82\xaf\xe3\x83\x90\xe3\x83\x83\xe3\x83\x95\xe3\x82\xa1\xe3\x81\xae\xe3\x82\xb5\xe3\x82\xa4\xe3\x82\xba\xe3\x81\xaf %d \xe3\x82\xad\xe3\x83\xad\xe3\x83\x90\xe3\x82\xa4\xe3\x83\x88\xe3\x81\xa7\xe3\x81\x99\n\x00"
                     as *const u8 as *const libc::c_char,
                 0x92 as libc::c_int);
    osSyncPrintf(b"\xef\xbc\xa6\xef\xbc\xa9\xef\xbc\xa6\xef\xbc\xaf\xe3\x83\x90\xe3\x83\x83\xe3\x83\x95\xe3\x82\xa1\xe3\x81\xae\xe3\x82\xb5\xe3\x82\xa4\xe3\x82\xba\xe3\x81\xaf %d \xe3\x82\xad\xe3\x83\xad\xe3\x83\x90\xe3\x82\xa4\xe3\x83\x88\xe3\x81\xa7\xe3\x81\x99\n\x00"
                     as *const u8 as *const libc::c_char,
                 0x60 as libc::c_int);
    osSyncPrintf(b"\xef\xbc\xb9\xef\xbc\xa9\xef\xbc\xa5\xef\xbc\xac\xef\xbc\xa4\xe3\x83\x90\xe3\x83\x83\xe3\x83\x95\xe3\x82\xa1\xe3\x81\xae\xe3\x82\xb5\xe3\x82\xa4\xe3\x82\xba\xe3\x81\xaf %d \xe3\x82\xad\xe3\x83\xad\xe3\x83\x90\xe3\x82\xa4\xe3\x83\x88\xe3\x81\xa7\xe3\x81\x99\n\x00"
                     as *const u8 as *const libc::c_char, 3 as libc::c_int);
    osSyncPrintf(b"\xe3\x82\xaa\xe3\x83\xbc\xe3\x83\x87\xe3\x82\xa3\xe3\x82\xaa\xe3\x83\x92\xe3\x83\xbc\xe3\x83\x97\xe3\x81\xae\xe3\x82\xb5\xe3\x82\xa4\xe3\x82\xba\xe3\x81\xaf %d \xe3\x82\xad\xe3\x83\xad\xe3\x83\x90\xe3\x82\xa4\xe3\x83\x88\xe3\x81\xa7\xe3\x81\x99\n\x00"
                     as *const u8 as *const libc::c_char,
                 (gSystemHeap.as_mut_ptr() as s32 -
                      gAudioHeap.as_mut_ptr() as s32) / 1024 as libc::c_int);
    osSyncPrintf(b"\x1b[m\x00" as *const u8 as *const libc::c_char);
    osCreateViManager(254 as libc::c_int);
    gViConfigFeatures = (0x2 as libc::c_int | 0x40 as libc::c_int) as u32_0;
    gViConfigXScale = 1.0f32;
    gViConfigYScale = 1.0f32;
    match osTvType {
        1 => {
            D_80013960 = 2 as libc::c_int as u8_0;
            gViConfigMode = osViModeNtscLan1
        }
        2 => {
            D_80013960 = 0x1e as libc::c_int as u8_0;
            gViConfigMode = osViModeMpalLan1
        }
        0 => {
            D_80013960 = 0x2c as libc::c_int as u8_0;
            gViConfigMode = osViModeFpalLan1;
            gViConfigYScale = 0.833f32
        }
        _ => { }
    }
    D_80009430 = 1 as libc::c_int as s8;
    osViSetMode(&mut gViConfigMode);
    ViConfig_UpdateVi(1 as libc::c_int as u32_0);
    osViBlack(1 as libc::c_int as u8_0);
    osViSwapBuffer(0x803da80 as libc::c_int as *mut libc::c_void);
    osCreatePiManager(150 as libc::c_int, &mut gPiMgrCmdQ,
                      sPiMgrCmdBuff.as_mut_ptr(), 50 as libc::c_int);
    StackCheck_Init(&mut sMainStackInfo,
                    sMainStack.as_mut_ptr() as *mut libc::c_void,
                    sMainStack.as_mut_ptr().offset(::std::mem::size_of::<[u8_0; 2304]>()
                                                       as libc::c_ulong as
                                                       isize) as
                        *mut libc::c_void, 0 as libc::c_int as u32_0,
                    0x400 as libc::c_int,
                    b"main\x00" as *const u8 as *const libc::c_char);
    osCreateThread(&mut gMainThread, 3 as libc::c_int,
                   Some(Main_ThreadEntry as
                            unsafe extern "C" fn(_: *mut libc::c_void) -> ()),
                   arg,
                   sMainStack.as_mut_ptr().offset(::std::mem::size_of::<[u8_0; 2304]>()
                                                      as libc::c_ulong as
                                                      isize) as
                       *mut libc::c_void, 10 as libc::c_int);
    osStartThread(&mut gMainThread);
    osSetThreadPri(0 as *mut OSThread, 0 as libc::c_int);
    loop  { };
}
