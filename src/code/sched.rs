#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case,
         non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(const_raw_ptr_to_usize_cast, register_tool)]
extern "C" {
    #[no_mangle]
    fn ViConfig_UpdateVi(mode: u32_0);
    #[no_mangle]
    fn ViConfig_UpdateBlack();
    #[no_mangle]
    fn __assert(exp: *const libc::c_char, file: *const libc::c_char,
                line: s32);
    #[no_mangle]
    fn osSyncPrintf(fmt: *const libc::c_char, _: ...);
    #[no_mangle]
    fn LogUtils_CheckValidPointer(exp: *const libc::c_char,
                                  ptr: *mut libc::c_void,
                                  file: *const libc::c_char, line: s32);
    #[no_mangle]
    fn LogUtils_LogThreadId(name: *const libc::c_char, line: s32);
    #[no_mangle]
    fn osSendMesg(mq: *mut OSMesgQueue, mesg: OSMesg, flag: s32) -> s32;
    #[no_mangle]
    fn osRecvMesg(mq: *mut OSMesgQueue, msg: *mut OSMesg, flag: s32) -> s32;
    #[no_mangle]
    fn bzero(__s: *mut libc::c_void, __n: u32_0);
    #[no_mangle]
    fn osCreateThread(thread: *mut OSThread, id: OSId,
                      entry:
                          Option<unsafe extern "C" fn(_: *mut libc::c_void)
                                     -> ()>, arg: *mut libc::c_void,
                      sp: *mut libc::c_void, pri: OSPri);
    #[no_mangle]
    fn osViGetNextFramebuffer() -> *mut libc::c_void;
    #[no_mangle]
    fn osSetEventMesg(e: OSEvent, mq: *mut OSMesgQueue, msg: OSMesg);
    #[no_mangle]
    fn osCreateMesgQueue(mq: *mut OSMesgQueue, msg: *mut OSMesg, count: s32);
    #[no_mangle]
    fn osViSwapBuffer(vaddr: *mut libc::c_void);
    #[no_mangle]
    fn osGetTime() -> OSTime;
    #[no_mangle]
    fn osViSetSpecialFeatures(func: u32_0);
    #[no_mangle]
    fn osStartThread(thread: *mut OSThread);
    #[no_mangle]
    fn IrqMgr_AddClient(this: *mut IrqMgr, c: *mut IrqMgrClient,
                        msgQ: *mut OSMesgQueue);
    #[no_mangle]
    fn Fault_SetFB(_: *mut libc::c_void, _: u16_0, _: u16_0);
    #[no_mangle]
    fn func_800FBFD8();
    #[no_mangle]
    static mut gGameInfo: *mut GameInfo;
    #[no_mangle]
    static mut gScreenWidth: s32;
    #[no_mangle]
    fn osViGetCurrentFramebuffer() -> *mut u32_0;
    #[no_mangle]
    static mut gIrqMgrResetStatus: vu32;
    #[no_mangle]
    fn osSpTaskYield();
    #[no_mangle]
    fn osSpTaskStartGo(task: *mut OSTask);
    #[no_mangle]
    fn osSpTaskLoad(task: *mut OSTask);
    #[no_mangle]
    fn osWritebackDCacheAll();
    #[no_mangle]
    fn osSpTaskYielded(task: *mut OSTask) -> OSYieldResult;
    #[no_mangle]
    static mut gRSPOtherTotalTime: OSTime;
    #[no_mangle]
    static mut gRSPGFXTotalTime: OSTime;
    #[no_mangle]
    static mut gRSPAudioTotalTime: OSTime;
    #[no_mangle]
    static mut gRDPTotalTime: OSTime;
}
pub type s8 = libc::c_schar;
pub type u8_0 = libc::c_uchar;
pub type s16 = libc::c_short;
pub type u16_0 = libc::c_ushort;
pub type s32 = libc::c_int;
pub type u32_0 = libc::c_uint;
pub type u64_0 = libc::c_ulonglong;
pub type vu32 = u32_0;
pub type vs32 = s32;
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
pub type OSEvent = u32_0;
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
pub struct OSTimer {
    pub next: *mut OSTimer,
    pub prev: *mut OSTimer,
    pub interval: OSTime,
    pub value: OSTime,
    pub mq: *mut OSMesgQueue,
    pub msg: OSMesg,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct OSTask_t {
    pub type_0: u32_0,
    pub flags: u32_0,
    pub ucode_boot: *mut u64_0,
    pub ucode_boot_size: u32_0,
    pub ucode: *mut u64_0,
    pub ucode_size: u32_0,
    pub ucode_data: *mut u64_0,
    pub ucode_data_size: u32_0,
    pub dram_stack: *mut u64_0,
    pub dram_stack_size: u32_0,
    pub output_buff: *mut u64_0,
    pub output_buff_size: *mut u64_0,
    pub data_ptr: *mut u64_0,
    pub data_size: u32_0,
    pub yield_data_ptr: *mut u64_0,
    pub yield_data_size: u32_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union OSTask {
    pub t: OSTask_t,
    pub force_structure_alignment: libc::c_longlong,
}
pub type OSYieldResult = u32_0;
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
pub struct OSScTask {
    pub next: *mut OSScTask,
    pub state: u32_0,
    pub flags: u32_0,
    pub framebuffer: *mut CfbInfo,
    pub list: OSTask,
    pub msgQ: *mut OSMesgQueue,
    pub msg: OSMesg,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CfbInfo {
    pub fb1: *mut u16_0,
    pub swapBuffer: *mut u16_0,
    pub viMode: *mut OSViMode,
    pub features: u32_0,
    pub unk_10: u8_0,
    pub updateRate: s8,
    pub updateRate2: s8,
    pub unk_13: u8_0,
    pub xScale: f32_0,
    pub yScale: f32_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct GameInfo {
    pub regPage: s32,
    pub regGroup: s32,
    pub regCur: s32,
    pub dpadLast: s32,
    pub repeat: s32,
    pub data: [s16; 2784],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct OSScMsg {
    pub type_0: s16,
    pub misc: [libc::c_char; 30],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct IrqMgrClient {
    pub prev: *mut IrqMgrClient,
    pub queue: *mut OSMesgQueue,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct IrqMgr {
    pub retraceMsg: OSScMsg,
    pub prenmiMsg: OSScMsg,
    pub nmiMsg: OSScMsg,
    pub queue: OSMesgQueue,
    pub msgBuf: [OSMesg; 8],
    pub thread: OSThread,
    pub clients: *mut IrqMgrClient,
    pub resetStatus: u8_0,
    pub resetTime: OSTime,
    pub timer: OSTimer,
    pub retraceTime: OSTime,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SchedContext {
    pub interruptQ: OSMesgQueue,
    pub intBuf: [OSMesg; 8],
    pub cmdQ: OSMesgQueue,
    pub cmdMsgBuf: [OSMesg; 8],
    pub thread: OSThread,
    pub audioListHead: *mut OSScTask,
    pub gfxListHead: *mut OSScTask,
    pub audioListTail: *mut OSScTask,
    pub gfxListTail: *mut OSScTask,
    pub curRSPTask: *mut OSScTask,
    pub curRDPTask: *mut OSScTask,
    pub retraceCnt: s32,
    pub doAudio: s32,
    pub curBuf: *mut CfbInfo,
    pub pendingSwapBuf1: *mut CfbInfo,
    pub pendingSwapBuf2: *mut CfbInfo,
    pub unk_24C: s32,
    pub irqClient: IrqMgrClient,
}
// data
#[no_mangle]
pub static mut sLogScheduler: vs32 = 0 as libc::c_int;
// bss
#[no_mangle]
pub static mut sRSPGFXStartTime: OSTime = 0;
#[no_mangle]
pub static mut sRSPAudioStartTime: OSTime = 0;
#[no_mangle]
pub static mut sRSPOtherStartTime: OSTime = 0;
#[no_mangle]
pub static mut sRDPStartTime: OSTime = 0;
#[no_mangle]
pub unsafe extern "C" fn Sched_SwapFrameBuffer(mut cfbInfo: *mut CfbInfo) {
    let mut width: u16_0 = 0;
    LogUtils_CheckValidPointer(b"cfbinfo->swapbuffer\x00" as *const u8 as
                                   *const libc::c_char,
                               (*cfbInfo).swapBuffer as *mut libc::c_void,
                               b"../sched.c\x00" as *const u8 as
                                   *const libc::c_char, 340 as libc::c_int);
    if !(*cfbInfo).swapBuffer.is_null() {
        osViSwapBuffer((*cfbInfo).swapBuffer as *mut libc::c_void);
        (*cfbInfo).updateRate2 = (*cfbInfo).updateRate;
        if sLogScheduler != 0 {
            osSyncPrintf(b"osViSwapBuffer %08x %08x %08x\n\x00" as *const u8
                             as *const libc::c_char,
                         osViGetCurrentFramebuffer(),
                         osViGetNextFramebuffer(),
                         if !cfbInfo.is_null() {
                             (*cfbInfo).swapBuffer
                         } else { 0 as *mut u16_0 });
        }
        width =
            if !(*cfbInfo).viMode.is_null() {
                (*(*cfbInfo).viMode).comRegs.width
            } else { gScreenWidth as u32_0 } as u16_0;
        Fault_SetFB((*cfbInfo).swapBuffer as *mut libc::c_void, width,
                    0x10 as libc::c_int as u16_0);
        if (*gGameInfo).data[(21 as libc::c_int * 6 as libc::c_int *
                                  16 as libc::c_int + 80 as libc::c_int) as
                                 usize] as libc::c_int == 0xd as libc::c_int
               &&
               (*gGameInfo).data[(21 as libc::c_int * 6 as libc::c_int *
                                      16 as libc::c_int + 95 as libc::c_int)
                                     as usize] as libc::c_int !=
                   0xd as libc::c_int {
            (*gGameInfo).data[(21 as libc::c_int * 6 as libc::c_int *
                                   16 as libc::c_int + 81 as libc::c_int) as
                                  usize] = 0 as libc::c_int as s16;
            (*gGameInfo).data[(21 as libc::c_int * 6 as libc::c_int *
                                   16 as libc::c_int + 82 as libc::c_int) as
                                  usize] = 0 as libc::c_int as s16;
            (*gGameInfo).data[(21 as libc::c_int * 6 as libc::c_int *
                                   16 as libc::c_int + 83 as libc::c_int) as
                                  usize] = 1 as libc::c_int as s16;
            (*gGameInfo).data[(21 as libc::c_int * 6 as libc::c_int *
                                   16 as libc::c_int + 84 as libc::c_int) as
                                  usize] = 0 as libc::c_int as s16;
            (*gGameInfo).data[(21 as libc::c_int * 6 as libc::c_int *
                                   16 as libc::c_int + 85 as libc::c_int) as
                                  usize] = 1 as libc::c_int as s16;
            (*gGameInfo).data[(21 as libc::c_int * 6 as libc::c_int *
                                   16 as libc::c_int + 86 as libc::c_int) as
                                  usize] = 0 as libc::c_int as s16;
            (*gGameInfo).data[(21 as libc::c_int * 6 as libc::c_int *
                                   16 as libc::c_int + 87 as libc::c_int) as
                                  usize] = 0 as libc::c_int as s16;
            (*gGameInfo).data[(21 as libc::c_int * 6 as libc::c_int *
                                   16 as libc::c_int + 88 as libc::c_int) as
                                  usize] = 0 as libc::c_int as s16;
            (*gGameInfo).data[(21 as libc::c_int * 6 as libc::c_int *
                                   16 as libc::c_int + 89 as libc::c_int) as
                                  usize] = 0 as libc::c_int as s16;
            (*gGameInfo).data[(21 as libc::c_int * 6 as libc::c_int *
                                   16 as libc::c_int + 90 as libc::c_int) as
                                  usize] = 0 as libc::c_int as s16;
            (*gGameInfo).data[(21 as libc::c_int * 6 as libc::c_int *
                                   16 as libc::c_int + 91 as libc::c_int) as
                                  usize] = 0 as libc::c_int as s16;
            (*gGameInfo).data[(21 as libc::c_int * 6 as libc::c_int *
                                   16 as libc::c_int + 92 as libc::c_int) as
                                  usize] = 0 as libc::c_int as s16;
            (*gGameInfo).data[(21 as libc::c_int * 6 as libc::c_int *
                                   16 as libc::c_int + 93 as libc::c_int) as
                                  usize] = 0 as libc::c_int as s16;
            (*gGameInfo).data[(21 as libc::c_int * 6 as libc::c_int *
                                   16 as libc::c_int + 94 as libc::c_int) as
                                  usize] = 0 as libc::c_int as s16;
            (*gGameInfo).data[(21 as libc::c_int * 6 as libc::c_int *
                                   16 as libc::c_int + 95 as libc::c_int) as
                                  usize] = 0xd as libc::c_int as s16
        }
        if (*gGameInfo).data[(21 as libc::c_int * 6 as libc::c_int *
                                  16 as libc::c_int + 80 as libc::c_int) as
                                 usize] as libc::c_int == 0xd as libc::c_int
               &&
               (*gGameInfo).data[(21 as libc::c_int * 6 as libc::c_int *
                                      16 as libc::c_int + 81 as libc::c_int)
                                     as usize] as libc::c_int ==
                   2 as libc::c_int {
            osViSetSpecialFeatures(if (*gGameInfo).data[(21 as libc::c_int *
                                                             6 as libc::c_int
                                                             *
                                                             16 as libc::c_int
                                                             +
                                                             82 as
                                                                 libc::c_int)
                                                            as usize] as
                                          libc::c_int != 0 as libc::c_int {
                                       0x1 as libc::c_int
                                   } else { 0x2 as libc::c_int } as u32_0);
            osViSetSpecialFeatures(if (*gGameInfo).data[(21 as libc::c_int *
                                                             6 as libc::c_int
                                                             *
                                                             16 as libc::c_int
                                                             +
                                                             83 as
                                                                 libc::c_int)
                                                            as usize] as
                                          libc::c_int != 0 as libc::c_int {
                                       0x40 as libc::c_int
                                   } else { 0x80 as libc::c_int } as u32_0);
            osViSetSpecialFeatures(if (*gGameInfo).data[(21 as libc::c_int *
                                                             6 as libc::c_int
                                                             *
                                                             16 as libc::c_int
                                                             +
                                                             84 as
                                                                 libc::c_int)
                                                            as usize] as
                                          libc::c_int != 0 as libc::c_int {
                                       0x4 as libc::c_int
                                   } else { 0x8 as libc::c_int } as u32_0);
            osViSetSpecialFeatures(if (*gGameInfo).data[(21 as libc::c_int *
                                                             6 as libc::c_int
                                                             *
                                                             16 as libc::c_int
                                                             +
                                                             85 as
                                                                 libc::c_int)
                                                            as usize] as
                                          libc::c_int != 0 as libc::c_int {
                                       0x10 as libc::c_int
                                   } else { 0x20 as libc::c_int } as u32_0);
        }
    }
    (*cfbInfo).unk_10 = 0 as libc::c_int as u8_0;
}
#[no_mangle]
pub unsafe extern "C" fn func_800C84E4(mut sc: *mut SchedContext,
                                       mut cfbInfo: *mut CfbInfo) {
    if (*sc).unk_24C != 0 as libc::c_int {
        (*sc).unk_24C = 0 as libc::c_int;
        if gIrqMgrResetStatus == 0 as libc::c_int as libc::c_uint {
            ViConfig_UpdateVi(0 as libc::c_int as u32_0);
        }
    }
    Sched_SwapFrameBuffer(cfbInfo);
}
#[no_mangle]
pub unsafe extern "C" fn Sched_HandleReset(mut sc: *mut SchedContext) {
    let mut now: OSTime = 0;
    if !(*sc).curRSPTask.is_null() {
        now = osGetTime();
        if (*(*sc).curRSPTask).framebuffer.is_null() {
            LogUtils_LogThreadId(b"../sched.c\x00" as *const u8 as
                                     *const libc::c_char, 421 as libc::c_int);
            osSyncPrintf(b"(((u64)(now - audio_rsp_start_time)*(1000000LL/15625LL))/((62500000LL*3/4)/15625LL)) = %lld\n\x00"
                             as *const u8 as *const libc::c_char,
                         now.wrapping_sub(sRSPAudioStartTime).wrapping_mul((1000000
                                                                                as
                                                                                libc::c_longlong
                                                                                /
                                                                                15625
                                                                                    as
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
                                                                                                                    15625
                                                                                                                        as
                                                                                                                        libc::c_longlong)
                                                                                                                   as
                                                                                                                   libc::c_ulonglong));
        } else if now.wrapping_sub(sRSPGFXStartTime).wrapping_mul((1000000 as
                                                                       libc::c_longlong
                                                                       /
                                                                       15625
                                                                           as
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
                                                                                                           15625
                                                                                                               as
                                                                                                               libc::c_longlong)
                                                                                                          as
                                                                                                          libc::c_ulonglong)
                      > 1000000 as libc::c_int as libc::c_ulonglong ||
                      now.wrapping_sub(sRDPStartTime).wrapping_mul((1000000 as
                                                                        libc::c_longlong
                                                                        /
                                                                        15625
                                                                            as
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
                                                                                                            15625
                                                                                                                as
                                                                                                                libc::c_longlong)
                                                                                                           as
                                                                                                           libc::c_ulonglong)
                          > 1000000 as libc::c_int as libc::c_ulonglong {
            func_800FBFD8();
            if !(*sc).curRSPTask.is_null() {
                LogUtils_LogThreadId(b"../sched.c\x00" as *const u8 as
                                         *const libc::c_char,
                                     427 as libc::c_int);
                osSyncPrintf(b"(((u64)(now - graph_rsp_start_time)*(1000000LL/15625LL))/((62500000LL*3/4)/15625LL)) = %lld\n\x00"
                                 as *const u8 as *const libc::c_char,
                             now.wrapping_sub(sRSPGFXStartTime).wrapping_mul((1000000
                                                                                  as
                                                                                  libc::c_longlong
                                                                                  /
                                                                                  15625
                                                                                      as
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
                                                                                                                      15625
                                                                                                                          as
                                                                                                                          libc::c_longlong)
                                                                                                                     as
                                                                                                                     libc::c_ulonglong));
                osSendMesg(&mut (*sc).interruptQ,
                           667 as libc::c_int as OSMesg, 0 as libc::c_int);
            }
            if !(*sc).curRDPTask.is_null() {
                LogUtils_LogThreadId(b"../sched.c\x00" as *const u8 as
                                         *const libc::c_char,
                                     431 as libc::c_int);
                osSyncPrintf(b"(((u64)(now - rdp_start_time)*(1000000LL/15625LL))/((62500000LL*3/4)/15625LL)) = %lld\n\x00"
                                 as *const u8 as *const libc::c_char,
                             now.wrapping_sub(sRDPStartTime).wrapping_mul((1000000
                                                                               as
                                                                               libc::c_longlong
                                                                               /
                                                                               15625
                                                                                   as
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
                                                                                                                   15625
                                                                                                                       as
                                                                                                                       libc::c_longlong)
                                                                                                                  as
                                                                                                                  libc::c_ulonglong));
                osSendMesg(&mut (*sc).interruptQ,
                           668 as libc::c_int as OSMesg, 0 as libc::c_int);
            }
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn Sched_HandleStart(mut sc: *mut SchedContext) {
    ViConfig_UpdateVi(1 as libc::c_int as u32_0);
}
#[no_mangle]
pub unsafe extern "C" fn Sched_QueueTask(mut sc: *mut SchedContext,
                                         mut task: *mut OSScTask) {
    let mut type_0: s32 = (*task).list.t.type_0 as s32;
    if type_0 == 2 as libc::c_int || type_0 == 1 as libc::c_int ||
           type_0 == 4 as libc::c_int || type_0 == 0 as libc::c_int {
    } else {
        __assert(b"(type == M_AUDTASK) || (type == M_GFXTASK) || (type == M_NJPEGTASK) || (type == M_NULTASK)\x00"
                     as *const u8 as *const libc::c_char,
                 b"../sched.c\x00" as *const u8 as *const libc::c_char,
                 463 as libc::c_int);
    };
    if type_0 == 2 as libc::c_int {
        if sLogScheduler != 0 {
            // "You have entered an audio task"
            osSyncPrintf(b"\xe3\x82\xaa\xe3\x83\xbc\xe3\x83\x87\xe3\x82\xa3\xe3\x82\xaa\xe3\x82\xbf\xe3\x82\xb9\xe3\x82\xaf\xe3\x82\x92\xe3\x82\xa8\xe3\x83\xb3\xe3\x83\x88\xe3\x83\xaa\xe3\x81\x97\xe3\x81\xbe\xe3\x81\x97\xe3\x81\x9f\n\x00"
                             as *const u8 as *const libc::c_char);
        }
        if !(*sc).audioListTail.is_null() {
            (*(*sc).audioListTail).next = task
        } else { (*sc).audioListHead = task }
        (*sc).audioListTail = task;
        (*sc).doAudio = 1 as libc::c_int
    } else {
        if sLogScheduler != 0 {
            osSyncPrintf(b"\xe3\x82\xb0\xe3\x83\xa9\xe3\x83\x95\xe3\x82\xbf\xe3\x82\xb9\xe3\x82\xaf\xe3\x82\x92\xe3\x82\xa8\xe3\x83\xb3\xe3\x83\x88\xe3\x83\xaa\xe3\x81\x97\xe3\x81\xbe\xe3\x81\x97\xe3\x81\x9f\n\x00"
                             as *const u8 as *const libc::c_char);
            // "Entered graph task"
        }
        if !(*sc).gfxListTail.is_null() {
            (*(*sc).gfxListTail).next = task
        } else { (*sc).gfxListHead = task }
        (*sc).gfxListTail = task
    }
    (*task).next = 0 as *mut OSScTask;
    (*task).state =
        (*task).flags &
            (0x1 as libc::c_int | 0x2 as libc::c_int) as libc::c_uint;
}
#[no_mangle]
pub unsafe extern "C" fn Sched_Yield(mut sc: *mut SchedContext) {
    if (*(*sc).curRSPTask).state & 0x10 as libc::c_int as libc::c_uint == 0 {
        if (*(*sc).curRSPTask).list.t.type_0 !=
               2 as libc::c_int as libc::c_uint {
        } else {
            __assert(b"sc->curRSPTask->list.t.type != M_AUDTASK\x00" as
                         *const u8 as *const libc::c_char,
                     b"../sched.c\x00" as *const u8 as *const libc::c_char,
                     496 as libc::c_int);
        };
        (*(*sc).curRSPTask).state |= 0x10 as libc::c_int as libc::c_uint;
        osSpTaskYield();
        if sLogScheduler != 0 {
            osSyncPrintf(b"%08d:osSpTaskYield\n\x00" as *const u8 as
                             *const libc::c_char,
                         osGetTime().wrapping_mul((1000000 as libc::c_longlong
                                                       /
                                                       15625 as
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
                                                                                           15625
                                                                                               as
                                                                                               libc::c_longlong)
                                                                                          as
                                                                                          libc::c_ulonglong)
                             as u32_0);
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn func_800C89D4(mut sc: *mut SchedContext,
                                       mut task: *mut OSScTask)
 -> *mut OSScTask {
    if task.is_null() { return 0 as *mut OSScTask }
    if !(*sc).pendingSwapBuf1.is_null() { return 0 as *mut OSScTask }
    if !(*sc).pendingSwapBuf2.is_null() { return 0 as *mut OSScTask }
    if (if !(*sc).pendingSwapBuf2.is_null() {
            (*(*sc).pendingSwapBuf2).swapBuffer
        } else { 0 as *mut u16_0 }) == (*(*task).framebuffer).fb1 {
        return 0 as *mut OSScTask
    }
    if (if !(*sc).pendingSwapBuf1.is_null() {
            (*(*sc).pendingSwapBuf1).swapBuffer
        } else { 0 as *mut u16_0 }) == (*(*task).framebuffer).fb1 {
        return 0 as *mut OSScTask
    }
    if osViGetCurrentFramebuffer() == (*(*task).framebuffer).fb1 as *mut u32_0
       {
        return 0 as *mut OSScTask
    }
    return task;
}
#[no_mangle]
pub unsafe extern "C" fn Sched_Schedule(mut sc: *mut SchedContext,
                                        mut sp: *mut *mut OSScTask,
                                        mut dp: *mut *mut OSScTask,
                                        mut state: s32) -> s32 {
    let mut ret: s32 = state;
    let mut gfxTask: *mut OSScTask = (*sc).gfxListHead;
    let mut audioTask: *mut OSScTask = (*sc).audioListHead;
    if (*sc).doAudio != 0 && ret & 0x2 as libc::c_int != 0 {
        *sp = audioTask;
        ret &= !(0x2 as libc::c_int);
        (*sc).doAudio = 0 as libc::c_int;
        (*sc).audioListHead = (*(*sc).audioListHead).next;
        if (*sc).audioListHead.is_null() {
            (*sc).audioListTail = 0 as *mut OSScTask
        }
    } else if !gfxTask.is_null() {
        if (*gfxTask).state & 0x20 as libc::c_int as libc::c_uint != 0 ||
               (*gfxTask).flags & 0x1 as libc::c_int as libc::c_uint == 0 {
            if ret & 0x2 as libc::c_int != 0 {
                *sp = gfxTask;
                ret &= !(0x2 as libc::c_int);
                (*sc).gfxListHead = (*(*sc).gfxListHead).next;
                if (*sc).gfxListHead.is_null() {
                    (*sc).gfxListTail = 0 as *mut OSScTask
                }
            }
        } else if ret == 0x2 as libc::c_int | 0x1 as libc::c_int {
            if (*gfxTask).framebuffer.is_null() ||
                   !func_800C89D4(sc, gfxTask).is_null() {
                *dp = gfxTask;
                *sp = *dp;
                ret &= !(0x2 as libc::c_int | 0x1 as libc::c_int);
                (*sc).gfxListHead = (*(*sc).gfxListHead).next;
                if (*sc).gfxListHead.is_null() {
                    (*sc).gfxListTail = 0 as *mut OSScTask
                }
            }
        }
    }
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn func_800C8BC4(mut sc: *mut SchedContext,
                                       mut task: *mut OSScTask) {
    if (*sc).pendingSwapBuf1.is_null() {
        (*sc).pendingSwapBuf1 = (*task).framebuffer;
        LogUtils_CheckValidPointer(b"sc->pending_swapbuffer1\x00" as *const u8
                                       as *const libc::c_char,
                                   (*sc).pendingSwapBuf1 as *mut libc::c_void,
                                   b"../sched.c\x00" as *const u8 as
                                       *const libc::c_char,
                                   618 as libc::c_int);
        if (*sc).curBuf.is_null() ||
               ((*(*sc).curBuf).updateRate2 as libc::c_int) < 1 as libc::c_int
           {
            func_800C84E4(sc, (*task).framebuffer);
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn Sched_IsComplete(mut sc: *mut SchedContext,
                                          mut task: *mut OSScTask) -> u32_0 {
    if (*task).state &
           (0x1 as libc::c_int | 0x2 as libc::c_int) as libc::c_uint == 0 {
        if !(*task).msgQ.is_null() {
            osSendMesg((*task).msgQ, (*task).msg, 1 as libc::c_int);
        }
        if (*task).flags & 0x40 as libc::c_int as libc::c_uint != 0 {
            func_800C8BC4(sc, task);
        }
        return 1 as libc::c_int as u32_0
    }
    return 0 as libc::c_int as u32_0;
}
#[no_mangle]
pub unsafe extern "C" fn Sched_RunTask(mut sc: *mut SchedContext,
                                       mut spTask: *mut OSScTask,
                                       mut dpTask: *mut OSScTask) {
    if (*sc).curRSPTask.is_null() {
    } else {
        __assert(b"sc->curRSPTask == NULL\x00" as *const u8 as
                     *const libc::c_char,
                 b"../sched.c\x00" as *const u8 as *const libc::c_char,
                 663 as libc::c_int);
    };
    if !spTask.is_null() {
        if (*spTask).list.t.type_0 == 0 as libc::c_int as libc::c_uint {
            if (*spTask).flags & 0x2 as libc::c_int as libc::c_uint != 0 {
                (*spTask).state &= !(0x2 as libc::c_int) as libc::c_uint;
                (*sc).curRSPTask = 0 as *mut OSScTask
            }
            if (*spTask).flags & 0x1 as libc::c_int as libc::c_uint != 0 {
                (*spTask).state &= !(0x1 as libc::c_int) as libc::c_uint;
                (*sc).curRDPTask = 0 as *mut OSScTask
            }
            Sched_IsComplete(sc, spTask);
            return
        }
        (*spTask).state &=
            !(0x10 as libc::c_int | 0x20 as libc::c_int) as libc::c_uint;
        osWritebackDCacheAll();
        osSpTaskLoad(&mut (*spTask).list);
        if (*spTask).list.t.type_0 == 2 as libc::c_int as libc::c_uint {
            sRSPAudioStartTime = osGetTime()
        } else if (*spTask).list.t.type_0 == 1 as libc::c_int as libc::c_uint
         {
            sRSPGFXStartTime = osGetTime()
        } else { sRSPOtherStartTime = osGetTime() }
        osSpTaskStartGo(&mut (*spTask).list);
        if sLogScheduler != 0 {
            osSyncPrintf(b"%08d:osSpTaskStartGo(%08x) %s\n\x00" as *const u8
                             as *const libc::c_char,
                         osGetTime().wrapping_mul((1000000 as libc::c_longlong
                                                       /
                                                       15625 as
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
                                                                                           15625
                                                                                               as
                                                                                               libc::c_longlong)
                                                                                          as
                                                                                          libc::c_ulonglong)
                             as u32_0, &mut (*spTask).list as *mut OSTask,
                         if (*spTask).list.t.type_0 ==
                                2 as libc::c_int as libc::c_uint {
                             b"AUDIO\x00" as *const u8 as *const libc::c_char
                         } else if (*spTask).list.t.type_0 ==
                                       1 as libc::c_int as libc::c_uint {
                             b"GRAPH\x00" as *const u8 as *const libc::c_char
                         } else {
                             b"OTHER\x00" as *const u8 as *const libc::c_char
                         });
        }
        (*sc).curRSPTask = spTask;
        if spTask == dpTask && (*sc).curRDPTask.is_null() {
            (*sc).curRDPTask = dpTask;
            sRDPStartTime = sRSPGFXStartTime
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn Sched_HandleEntry(mut sc: *mut SchedContext) {
    let mut nextRSP: *mut OSScTask = 0 as *mut OSScTask;
    let mut nextRDP: *mut OSScTask = 0 as *mut OSScTask;
    let mut state: s32 = 0;
    let mut msg: OSMesg = 0 as *mut libc::c_void;
    while osRecvMesg(&mut (*sc).cmdQ, &mut msg, 0 as libc::c_int) !=
              -(1 as libc::c_int) {
        Sched_QueueTask(sc, msg as *mut OSScTask);
    }
    if (*sc).doAudio != 0 as libc::c_int && !(*sc).curRSPTask.is_null() {
        if sLogScheduler != 0 {
            osSyncPrintf(b"[YIELD B]\x00" as *const u8 as
                             *const libc::c_char);
        }
        Sched_Yield(sc);
        return
    }
    state =
        ((*sc).curRSPTask == 0 as *mut OSScTask) as libc::c_int *
            2 as libc::c_int |
            ((*sc).curRDPTask == 0 as *mut OSScTask) as libc::c_int;
    if Sched_Schedule(sc, &mut nextRSP, &mut nextRDP, state) != state {
        Sched_RunTask(sc, nextRSP, nextRDP);
    }
    if sLogScheduler != 0 {
        osSyncPrintf(b"EN sc:%08x sp:%08x dp:%08x state:%x\n\x00" as *const u8
                         as *const libc::c_char, sc, nextRSP, nextRDP, state);
    };
}
#[no_mangle]
pub unsafe extern "C" fn Sched_HandleRetrace(mut sc: *mut SchedContext) {
    if sLogScheduler != 0 {
        osSyncPrintf(b"%08d:scHandleRetrace %08x\n\x00" as *const u8 as
                         *const libc::c_char,
                     osGetTime().wrapping_mul((1000000 as libc::c_longlong /
                                                   15625 as libc::c_longlong)
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
                                                                                       15625
                                                                                           as
                                                                                           libc::c_longlong)
                                                                                      as
                                                                                      libc::c_ulonglong)
                         as u32_0, osViGetCurrentFramebuffer());
    }
    ViConfig_UpdateBlack();
    (*sc).retraceCnt += 1;
    if osViGetCurrentFramebuffer() ==
           (if !(*sc).pendingSwapBuf1.is_null() {
                (*(*sc).pendingSwapBuf1).swapBuffer
            } else { 0 as *mut u16_0 }) as *mut u32_0 {
        if !(*sc).curBuf.is_null() {
            (*(*sc).curBuf).unk_10 = 0 as libc::c_int as u8_0
        }
        if !(*sc).pendingSwapBuf1.is_null() {
            (*(*sc).pendingSwapBuf1).unk_10 = 0 as libc::c_int as u8_0
        }
        (*sc).curBuf = (*sc).pendingSwapBuf1;
        (*sc).pendingSwapBuf1 = (*sc).pendingSwapBuf2;
        (*sc).pendingSwapBuf2 = 0 as *mut CfbInfo
    }
    if !(*sc).curBuf.is_null() {
        if (*(*sc).curBuf).updateRate2 as libc::c_int > 0 as libc::c_int {
            (*(*sc).curBuf).updateRate2 -= 1
        }
        if (*(*sc).curBuf).updateRate2 as libc::c_int <= 0 as libc::c_int &&
               !(*sc).pendingSwapBuf1.is_null() {
            func_800C84E4(sc, (*sc).pendingSwapBuf1);
        }
    }
    if sLogScheduler != 0 {
        osSyncPrintf(b"%08x %08x %08x %d\n\x00" as *const u8 as
                         *const libc::c_char, osViGetCurrentFramebuffer(),
                     osViGetNextFramebuffer(),
                     if !(*sc).pendingSwapBuf1.is_null() {
                         (*(*sc).pendingSwapBuf1).swapBuffer
                     } else { 0 as *mut u16_0 },
                     if !(*sc).curBuf.is_null() {
                         (*(*sc).curBuf).updateRate2 as libc::c_int
                     } else { 0 as libc::c_int });
    }
    Sched_HandleEntry(sc);
}
#[no_mangle]
pub unsafe extern "C" fn Sched_HandleRSPDone(mut sc: *mut SchedContext) {
    let mut curRSPTask: *mut OSScTask = 0 as *mut OSScTask;
    let mut nextRSP: *mut OSScTask = 0 as *mut OSScTask;
    let mut nextRDP: *mut OSScTask = 0 as *mut OSScTask;
    let mut state: s32 = 0;
    if !(*sc).curRSPTask.is_null() {
    } else {
        __assert(b"sc->curRSPTask\x00" as *const u8 as *const libc::c_char,
                 b"../sched.c\x00" as *const u8 as *const libc::c_char,
                 819 as libc::c_int);
    };
    if (*(*sc).curRSPTask).list.t.type_0 == 2 as libc::c_int as libc::c_uint {
        ::std::ptr::write_volatile(&mut gRSPAudioTotalTime as *mut OSTime,
                                   (::std::ptr::read_volatile::<OSTime>(&gRSPAudioTotalTime
                                                                            as
                                                                            *const OSTime)
                                        as
                                        libc::c_ulonglong).wrapping_add(osGetTime().wrapping_sub(sRSPAudioStartTime))
                                       as OSTime as OSTime)
    } else if (*(*sc).curRSPTask).list.t.type_0 ==
                  1 as libc::c_int as libc::c_uint {
        ::std::ptr::write_volatile(&mut gRSPGFXTotalTime as *mut OSTime,
                                   (::std::ptr::read_volatile::<OSTime>(&gRSPGFXTotalTime
                                                                            as
                                                                            *const OSTime)
                                        as
                                        libc::c_ulonglong).wrapping_add(osGetTime().wrapping_sub(sRSPGFXStartTime))
                                       as OSTime as OSTime)
    } else {
        ::std::ptr::write_volatile(&mut gRSPOtherTotalTime as *mut OSTime,
                                   (::std::ptr::read_volatile::<OSTime>(&gRSPOtherTotalTime
                                                                            as
                                                                            *const OSTime)
                                        as
                                        libc::c_ulonglong).wrapping_add(osGetTime().wrapping_sub(sRSPOtherStartTime))
                                       as OSTime as OSTime)
    }
    curRSPTask = (*sc).curRSPTask;
    (*sc).curRSPTask = 0 as *mut OSScTask;
    if sLogScheduler != 0 {
        osSyncPrintf(b"RSP DONE %d %d\x00" as *const u8 as
                         *const libc::c_char,
                     (*curRSPTask).state &
                         0x10 as libc::c_int as libc::c_uint,
                     osSpTaskYielded(&mut (*curRSPTask).list));
    }
    if (*curRSPTask).state & 0x10 as libc::c_int as libc::c_uint != 0 &&
           osSpTaskYielded(&mut (*curRSPTask).list) != 0 {
        if sLogScheduler != 0 {
            osSyncPrintf(b"[YIELDED]\n\x00" as *const u8 as
                             *const libc::c_char);
        }
        (*curRSPTask).state |= 0x20 as libc::c_int as libc::c_uint;
        (*curRSPTask).next = (*sc).gfxListHead;
        (*sc).gfxListHead = curRSPTask;
        if (*sc).gfxListTail.is_null() { (*sc).gfxListTail = curRSPTask }
    } else {
        if sLogScheduler != 0 {
            osSyncPrintf(b"[NOT YIELDED]\n\x00" as *const u8 as
                             *const libc::c_char);
        }
        (*curRSPTask).state &= !(0x2 as libc::c_int) as libc::c_uint;
        Sched_IsComplete(sc, curRSPTask);
    }
    state =
        (((*sc).curRSPTask == 0 as *mut libc::c_void as *mut OSScTask) as
             libc::c_int) << 1 as libc::c_int |
            ((*sc).curRDPTask == 0 as *mut libc::c_void as *mut OSScTask) as
                libc::c_int;
    if Sched_Schedule(sc, &mut nextRSP, &mut nextRDP, state) != state {
        Sched_RunTask(sc, nextRSP, nextRDP);
    }
    if sLogScheduler != 0 {
        osSyncPrintf(b"SP sc:%08x sp:%08x dp:%08x state:%x\n\x00" as *const u8
                         as *const libc::c_char, sc, nextRSP, nextRDP, state);
    };
}
#[no_mangle]
pub unsafe extern "C" fn Sched_HandleRDPDone(mut sc: *mut SchedContext) {
    let mut curTask: *mut OSScTask = 0 as *mut OSScTask;
    let mut nextRSP: *mut OSScTask = 0 as *mut OSScTask;
    let mut nextRDP: *mut OSScTask = 0 as *mut OSScTask;
    let mut state: s32 = 0;
    ::std::ptr::write_volatile(&mut gRDPTotalTime as *mut OSTime,
                               osGetTime().wrapping_sub(sRDPStartTime));
    if !(*sc).curRDPTask.is_null() {
    } else {
        __assert(b"sc->curRDPTask\x00" as *const u8 as *const libc::c_char,
                 b"../sched.c\x00" as *const u8 as *const libc::c_char,
                 878 as libc::c_int);
    };
    if (*(*sc).curRDPTask).list.t.type_0 == 1 as libc::c_int as libc::c_uint {
    } else {
        __assert(b"sc->curRDPTask->list.t.type == M_GFXTASK\x00" as *const u8
                     as *const libc::c_char,
                 b"../sched.c\x00" as *const u8 as *const libc::c_char,
                 879 as libc::c_int);
    };
    curTask = (*sc).curRDPTask;
    (*sc).curRDPTask = 0 as *mut OSScTask;
    (*curTask).state &= !(0x1 as libc::c_int) as libc::c_uint;
    Sched_IsComplete(sc, curTask);
    state =
        (((*sc).curRSPTask == 0 as *mut libc::c_void as *mut OSScTask) as
             libc::c_int) << 1 as libc::c_int |
            ((*sc).curRDPTask == 0 as *mut libc::c_void as *mut OSScTask) as
                libc::c_int;
    if Sched_Schedule(sc, &mut nextRSP, &mut nextRDP, state) != state {
        Sched_RunTask(sc, nextRSP, nextRDP);
    }
    if sLogScheduler != 0 {
        osSyncPrintf(b"DP sc:%08x sp:%08x dp:%08x state:%x\n\x00" as *const u8
                         as *const libc::c_char, sc, nextRSP, nextRDP, state);
    };
}
#[no_mangle]
pub unsafe extern "C" fn Sched_SendEntryMsg(mut sc: *mut SchedContext) {
    if sLogScheduler != 0 {
        osSyncPrintf(b"osScKickEntryMsg\n\x00" as *const u8 as
                         *const libc::c_char);
    }
    osSendMesg(&mut (*sc).interruptQ, 670 as libc::c_int as OSMesg,
               1 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn Sched_ThreadEntry(mut arg: *mut libc::c_void) {
    let mut msg: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut sc: *mut SchedContext = arg as *mut SchedContext;
    msg = 0 as *mut libc::c_void;
    loop  {
        if sLogScheduler != 0 {
            // "%08d: standby"
            osSyncPrintf(b"%08d:\xe5\xbe\x85\xe6\xa9\x9f\xe4\xb8\xad\n\x00" as
                             *const u8 as *const libc::c_char,
                         osGetTime().wrapping_mul((1000000 as libc::c_longlong
                                                       /
                                                       15625 as
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
                                                                                           15625
                                                                                               as
                                                                                               libc::c_longlong)
                                                                                          as
                                                                                          libc::c_ulonglong)
                             as u32_0);
        }
        osRecvMesg(&mut (*sc).interruptQ, &mut msg, 1 as libc::c_int);
        match msg as s32 {
            670 => {
                if sLogScheduler != 0 {
                    osSyncPrintf(b"%08d:ENTRY_MSG\n\x00" as *const u8 as
                                     *const libc::c_char,
                                 osGetTime().wrapping_mul((1000000 as
                                                               libc::c_longlong
                                                               /
                                                               15625 as
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
                                                                                                   15625
                                                                                                       as
                                                                                                       libc::c_longlong)
                                                                                                  as
                                                                                                  libc::c_ulonglong)
                                     as u32_0);
                }
                Sched_HandleEntry(sc);
            }
            667 => {
                if sLogScheduler != 0 {
                    osSyncPrintf(b"%08d:RSP_DONE_MSG\n\x00" as *const u8 as
                                     *const libc::c_char,
                                 osGetTime().wrapping_mul((1000000 as
                                                               libc::c_longlong
                                                               /
                                                               15625 as
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
                                                                                                   15625
                                                                                                       as
                                                                                                       libc::c_longlong)
                                                                                                  as
                                                                                                  libc::c_ulonglong)
                                     as u32_0);
                }
                Sched_HandleRSPDone(sc);
            }
            668 => {
                if sLogScheduler != 0 {
                    osSyncPrintf(b"%08d:RDP_DONE_MSG\n\x00" as *const u8 as
                                     *const libc::c_char,
                                 osGetTime().wrapping_mul((1000000 as
                                                               libc::c_longlong
                                                               /
                                                               15625 as
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
                                                                                                   15625
                                                                                                       as
                                                                                                       libc::c_longlong)
                                                                                                  as
                                                                                                  libc::c_ulonglong)
                                     as u32_0);
                }
                Sched_HandleRDPDone(sc);
            }
            _ => {
                match (*(msg as *mut OSScMsg)).type_0 as libc::c_int {
                    1 => { Sched_HandleRetrace(sc); }
                    4 => { Sched_HandleReset(sc); }
                    3 => { Sched_HandleStart(sc); }
                    _ => { }
                }
            }
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn Sched_Init(mut sc: *mut SchedContext,
                                    mut stack: *mut libc::c_void,
                                    mut priority: OSPri, mut arg3: s32,
                                    mut arg4: s32, mut irqMgr: *mut IrqMgr) {
    bzero(sc as *mut libc::c_void,
          ::std::mem::size_of::<SchedContext>() as libc::c_ulong);
    (*sc).unk_24C = 1 as libc::c_int;
    osCreateMesgQueue(&mut (*sc).interruptQ, (*sc).intBuf.as_mut_ptr(),
                      8 as libc::c_int);
    osCreateMesgQueue(&mut (*sc).cmdQ, (*sc).cmdMsgBuf.as_mut_ptr(),
                      8 as libc::c_int);
    osSetEventMesg(4 as libc::c_int as OSEvent, &mut (*sc).interruptQ,
                   667 as libc::c_int as OSMesg);
    osSetEventMesg(9 as libc::c_int as OSEvent, &mut (*sc).interruptQ,
                   668 as libc::c_int as OSMesg);
    IrqMgr_AddClient(irqMgr, &mut (*sc).irqClient, &mut (*sc).interruptQ);
    osCreateThread(&mut (*sc).thread, 5 as libc::c_int,
                   Some(Sched_ThreadEntry as
                            unsafe extern "C" fn(_: *mut libc::c_void) -> ()),
                   sc as *mut libc::c_void, stack, priority);
    osStartThread(&mut (*sc).thread);
}
