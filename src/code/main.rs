#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case,
         non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(const_raw_ptr_to_usize_cast, register_tool)]
extern "C" {
    #[no_mangle]
    fn osSyncPrintf(fmt: *const libc::c_char, _: ...);
    #[no_mangle]
    fn StackCheck_Init(entry: *mut StackEntry, stackTop: *mut libc::c_void,
                       stackBottom: *mut libc::c_void, initValue: u32_0,
                       minSpace: s32, name: *const libc::c_char);
    #[no_mangle]
    fn osRecvMesg(mq: *mut OSMesgQueue, msg: *mut OSMesg, flag: s32) -> s32;
    #[no_mangle]
    fn osDestroyThread(thread: *mut OSThread);
    #[no_mangle]
    fn osCreateThread(thread: *mut OSThread, id: OSId,
                      entry:
                          Option<unsafe extern "C" fn(_: *mut libc::c_void)
                                     -> ()>, arg: *mut libc::c_void,
                      sp: *mut libc::c_void, pri: OSPri);
    #[no_mangle]
    fn osSetEventMesg(e: OSEvent, mq: *mut OSMesgQueue, msg: OSMesg);
    #[no_mangle]
    fn osCreateMesgQueue(mq: *mut OSMesgQueue, msg: *mut OSMesg, count: s32);
    #[no_mangle]
    fn osSetThreadPri(thread: *mut OSThread, pri: OSPri);
    #[no_mangle]
    fn osStartThread(thread: *mut OSThread);
    #[no_mangle]
    fn func_800636C0();
    #[no_mangle]
    fn PreNmiBuff_Init(this: *mut PreNmiBuff);
    #[no_mangle]
    fn PreNmiBuff_SetReset(this: *mut PreNmiBuff);
    #[no_mangle]
    fn AudioMgr_Unlock(audioMgr: *mut AudioMgr);
    #[no_mangle]
    fn AudioMgr_Init(audioMgr: *mut AudioMgr, stack: *mut libc::c_void,
                     pri: OSPri, id: OSId, sched: *mut SchedContext,
                     irqMgr: *mut IrqMgr);
    #[no_mangle]
    fn Graph_ThreadEntry(_: *mut libc::c_void);
    #[no_mangle]
    fn PadMgr_Init(padmgr: *mut PadMgr, siIntMsgQ: *mut OSMesgQueue,
                   irqMgr: *mut IrqMgr, id: OSId, priority: OSPri,
                   stack: *mut libc::c_void);
    #[no_mangle]
    fn Sched_Init(sc: *mut SchedContext, stack: *mut libc::c_void,
                  priority: OSPri, arg3: s32, arg4: s32, irqMgr: *mut IrqMgr);
    #[no_mangle]
    fn SysCfb_Init(n64dd: s32);
    #[no_mangle]
    fn SysCfb_GetFbPtr(idx: s32) -> u32_0;
    #[no_mangle]
    fn SysCfb_GetFbEnd() -> u32_0;
    #[no_mangle]
    fn IrqMgr_AddClient(this: *mut IrqMgr, c: *mut IrqMgrClient,
                        msgQ: *mut OSMesgQueue);
    #[no_mangle]
    fn IrqMgr_Init(this: *mut IrqMgr, stack: *mut libc::c_void, pri: OSPri,
                   retraceCount: u8_0);
    #[no_mangle]
    fn DebugArena_Init(start: *mut libc::c_void, size: u32_0);
    #[no_mangle]
    fn Fault_Init();
    #[no_mangle]
    fn func_800FBFD8();
    #[no_mangle]
    fn SystemHeap_Init(start: *mut libc::c_void, size: u32_0);
    #[no_mangle]
    fn SystemArena_MallocDebug(size: u32_0, file: *const libc::c_char,
                               line: s32) -> *mut libc::c_void;
    #[no_mangle]
    static mut gSystemHeap: [u8_0; 0];
    #[no_mangle]
    static mut D_80013960: u8_0;
    #[no_mangle]
    static mut gGameInfo: *mut GameInfo;
    #[no_mangle]
    static mut osMemSize: u32_0;
    #[no_mangle]
    static mut osAppNmiBuffer: [u8_0; 64];
}
pub type s8 = libc::c_schar;
pub type u8_0 = libc::c_uchar;
pub type s16 = libc::c_short;
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
pub struct OSContStatus {
    pub type_0: u16_0,
    pub status: u8_0,
    pub errno: u8_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct OSContPad {
    pub button: u16_0,
    pub stick_x: s8,
    pub stick_y: s8,
    pub errno: u8_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct OSPfs {
    pub status: s32,
    pub queue: *mut OSMesgQueue,
    pub channel: s32,
    pub id: [u8_0; 32],
    pub label: [u8_0; 32],
    pub version: s32,
    pub dir_size: s32,
    pub inode_table: s32,
    pub minode_table: s32,
    pub dir_table: s32,
    pub inodeStartPage: s32,
    pub banks: u8_0,
    pub activebank: u8_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Input {
    pub cur: OSContPad,
    pub prev: OSContPad,
    pub press: OSContPad,
    pub rel: OSContPad,
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
pub struct AudioTask {
    pub task: OSTask,
    pub taskQueue: *mut OSMesgQueue,
    pub unk_44: *mut libc::c_void,
    pub unk_48: [libc::c_char; 8],
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
pub struct StackEntry {
    pub next: *mut StackEntry,
    pub prev: *mut StackEntry,
    pub head: u32_0,
    pub tail: u32_0,
    pub initValue: u32_0,
    pub minSpace: s32,
    pub name: *const libc::c_char,
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
pub struct PadMgr {
    pub padStatus: [OSContStatus; 4],
    pub serialMsgBuf: [OSMesg; 1],
    pub lockMsgBuf: [OSMesg; 1],
    pub interruptMsgBuf: [OSMesg; 4],
    pub serialMsgQ: OSMesgQueue,
    pub lockMsgQ: OSMesgQueue,
    pub interruptMsgQ: OSMesgQueue,
    pub irqClient: IrqMgrClient,
    pub irqMgr: *mut IrqMgr,
    pub thread: OSThread,
    pub inputs: [Input; 4],
    pub pads: [OSContPad; 4],
    pub validCtrlrsMask: vu8,
    pub nControllers: u8_0,
    pub ctrlrIsConnected: [u8_0; 4],
    pub pakType: [u8_0; 4],
    pub rumbleEnable: [vu8; 4],
    pub rumbleCounter: [u8_0; 4],
    pub pfs: [OSPfs; 4],
    pub rumbleOffFrames: vu8,
    pub rumbleOnFrames: vu8,
    pub preNMIShutdown: u8_0,
    pub retraceCallback: Option<unsafe extern "C" fn(_: *mut PadMgr, _: s32)
                                    -> ()>,
    pub retraceCallbackValue: u32_0,
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AudioMgr {
    pub irqMgr: *mut IrqMgr,
    pub sched: *mut SchedContext,
    pub audioTask: OSScTask,
    pub unk_60: [libc::c_char; 16],
    pub rspTask: *mut AudioTask,
    pub unk_74: OSMesgQueue,
    pub unk_8C: OSMesg,
    pub unk_90: OSMesgQueue,
    pub unk_A8: OSMesg,
    pub unk_AC: OSMesgQueue,
    pub unk_C4: OSMesg,
    pub unk_C8: OSMesgQueue,
    pub unk_E0: OSMesg,
    pub unk_E4: [libc::c_char; 4],
    pub unk_E8: OSThread,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PreNmiBuff {
    pub resetting: u32_0,
    pub resetCount: u32_0,
    pub duration: OSTime,
    pub resetTime: OSTime,
}
#[no_mangle]
pub static mut gScreenWidth: s32 = 320 as libc::c_int;
#[no_mangle]
pub static mut gScreenHeight: s32 = 240 as libc::c_int;
#[no_mangle]
pub static mut gSystemHeapSize: u32_0 = 0 as libc::c_int as u32_0;
#[no_mangle]
pub static mut gAppNmiBufferPtr: *mut PreNmiBuff =
    0 as *const PreNmiBuff as *mut PreNmiBuff;
#[no_mangle]
pub static mut gSchedContext: SchedContext =
    SchedContext{interruptQ:
                     OSMesgQueue{mtqueue:
                                     0 as *const OSThread as *mut OSThread,
                                 fullqueue:
                                     0 as *const OSThread as *mut OSThread,
                                 validCount: 0,
                                 first: 0,
                                 msgCount: 0,
                                 msg: 0 as *const OSMesg as *mut OSMesg,},
                 intBuf: [0 as *const libc::c_void as *mut libc::c_void; 8],
                 cmdQ:
                     OSMesgQueue{mtqueue:
                                     0 as *const OSThread as *mut OSThread,
                                 fullqueue:
                                     0 as *const OSThread as *mut OSThread,
                                 validCount: 0,
                                 first: 0,
                                 msgCount: 0,
                                 msg: 0 as *const OSMesg as *mut OSMesg,},
                 cmdMsgBuf:
                     [0 as *const libc::c_void as *mut libc::c_void; 8],
                 thread:
                     OSThread{next: 0 as *const OSThread as *mut OSThread,
                              priority: 0,
                              queue:
                                  0 as *const *mut OSThread as
                                      *mut *mut OSThread,
                              tlnext: 0 as *const OSThread as *mut OSThread,
                              state: 0,
                              flags: 0,
                              id: 0,
                              fp: 0,
                              thprof:
                                  0 as *const __OSThreadprofile as
                                      *mut __OSThreadprofile,
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
                                                                   C2RustUnnamed{f_odd:
                                                                                     0.,
                                                                                 f_even:
                                                                                     0.,},},
                                                    fp2:
                                                        __OSfp{f:
                                                                   C2RustUnnamed{f_odd:
                                                                                     0.,
                                                                                 f_even:
                                                                                     0.,},},
                                                    fp4:
                                                        __OSfp{f:
                                                                   C2RustUnnamed{f_odd:
                                                                                     0.,
                                                                                 f_even:
                                                                                     0.,},},
                                                    fp6:
                                                        __OSfp{f:
                                                                   C2RustUnnamed{f_odd:
                                                                                     0.,
                                                                                 f_even:
                                                                                     0.,},},
                                                    fp8:
                                                        __OSfp{f:
                                                                   C2RustUnnamed{f_odd:
                                                                                     0.,
                                                                                 f_even:
                                                                                     0.,},},
                                                    fp10:
                                                        __OSfp{f:
                                                                   C2RustUnnamed{f_odd:
                                                                                     0.,
                                                                                 f_even:
                                                                                     0.,},},
                                                    fp12:
                                                        __OSfp{f:
                                                                   C2RustUnnamed{f_odd:
                                                                                     0.,
                                                                                 f_even:
                                                                                     0.,},},
                                                    fp14:
                                                        __OSfp{f:
                                                                   C2RustUnnamed{f_odd:
                                                                                     0.,
                                                                                 f_even:
                                                                                     0.,},},
                                                    fp16:
                                                        __OSfp{f:
                                                                   C2RustUnnamed{f_odd:
                                                                                     0.,
                                                                                 f_even:
                                                                                     0.,},},
                                                    fp18:
                                                        __OSfp{f:
                                                                   C2RustUnnamed{f_odd:
                                                                                     0.,
                                                                                 f_even:
                                                                                     0.,},},
                                                    fp20:
                                                        __OSfp{f:
                                                                   C2RustUnnamed{f_odd:
                                                                                     0.,
                                                                                 f_even:
                                                                                     0.,},},
                                                    fp22:
                                                        __OSfp{f:
                                                                   C2RustUnnamed{f_odd:
                                                                                     0.,
                                                                                 f_even:
                                                                                     0.,},},
                                                    fp24:
                                                        __OSfp{f:
                                                                   C2RustUnnamed{f_odd:
                                                                                     0.,
                                                                                 f_even:
                                                                                     0.,},},
                                                    fp26:
                                                        __OSfp{f:
                                                                   C2RustUnnamed{f_odd:
                                                                                     0.,
                                                                                 f_even:
                                                                                     0.,},},
                                                    fp28:
                                                        __OSfp{f:
                                                                   C2RustUnnamed{f_odd:
                                                                                     0.,
                                                                                 f_even:
                                                                                     0.,},},
                                                    fp30:
                                                        __OSfp{f:
                                                                   C2RustUnnamed{f_odd:
                                                                                     0.,
                                                                                 f_even:
                                                                                     0.,},},},},
                 audioListHead: 0 as *const OSScTask as *mut OSScTask,
                 gfxListHead: 0 as *const OSScTask as *mut OSScTask,
                 audioListTail: 0 as *const OSScTask as *mut OSScTask,
                 gfxListTail: 0 as *const OSScTask as *mut OSScTask,
                 curRSPTask: 0 as *const OSScTask as *mut OSScTask,
                 curRDPTask: 0 as *const OSScTask as *mut OSScTask,
                 retraceCnt: 0,
                 doAudio: 0,
                 curBuf: 0 as *const CfbInfo as *mut CfbInfo,
                 pendingSwapBuf1: 0 as *const CfbInfo as *mut CfbInfo,
                 pendingSwapBuf2: 0 as *const CfbInfo as *mut CfbInfo,
                 unk_24C: 0,
                 irqClient:
                     IrqMgrClient{prev:
                                      0 as *const IrqMgrClient as
                                          *mut IrqMgrClient,
                                  queue:
                                      0 as *const OSMesgQueue as
                                          *mut OSMesgQueue,},};
#[no_mangle]
pub static mut gPadMgr: PadMgr =
    PadMgr{padStatus: [OSContStatus{type_0: 0, status: 0, errno: 0,}; 4],
           serialMsgBuf: [0 as *const libc::c_void as *mut libc::c_void; 1],
           lockMsgBuf: [0 as *const libc::c_void as *mut libc::c_void; 1],
           interruptMsgBuf:
               [0 as *const libc::c_void as *mut libc::c_void; 4],
           serialMsgQ:
               OSMesgQueue{mtqueue: 0 as *const OSThread as *mut OSThread,
                           fullqueue: 0 as *const OSThread as *mut OSThread,
                           validCount: 0,
                           first: 0,
                           msgCount: 0,
                           msg: 0 as *const OSMesg as *mut OSMesg,},
           lockMsgQ:
               OSMesgQueue{mtqueue: 0 as *const OSThread as *mut OSThread,
                           fullqueue: 0 as *const OSThread as *mut OSThread,
                           validCount: 0,
                           first: 0,
                           msgCount: 0,
                           msg: 0 as *const OSMesg as *mut OSMesg,},
           interruptMsgQ:
               OSMesgQueue{mtqueue: 0 as *const OSThread as *mut OSThread,
                           fullqueue: 0 as *const OSThread as *mut OSThread,
                           validCount: 0,
                           first: 0,
                           msgCount: 0,
                           msg: 0 as *const OSMesg as *mut OSMesg,},
           irqClient:
               IrqMgrClient{prev:
                                0 as *const IrqMgrClient as *mut IrqMgrClient,
                            queue:
                                0 as *const OSMesgQueue as *mut OSMesgQueue,},
           irqMgr: 0 as *const IrqMgr as *mut IrqMgr,
           thread:
               OSThread{next: 0 as *const OSThread as *mut OSThread,
                        priority: 0,
                        queue:
                            0 as *const *mut OSThread as *mut *mut OSThread,
                        tlnext: 0 as *const OSThread as *mut OSThread,
                        state: 0,
                        flags: 0,
                        id: 0,
                        fp: 0,
                        thprof:
                            0 as *const __OSThreadprofile as
                                *mut __OSThreadprofile,
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
                                                             C2RustUnnamed{f_odd:
                                                                               0.,
                                                                           f_even:
                                                                               0.,},},
                                              fp2:
                                                  __OSfp{f:
                                                             C2RustUnnamed{f_odd:
                                                                               0.,
                                                                           f_even:
                                                                               0.,},},
                                              fp4:
                                                  __OSfp{f:
                                                             C2RustUnnamed{f_odd:
                                                                               0.,
                                                                           f_even:
                                                                               0.,},},
                                              fp6:
                                                  __OSfp{f:
                                                             C2RustUnnamed{f_odd:
                                                                               0.,
                                                                           f_even:
                                                                               0.,},},
                                              fp8:
                                                  __OSfp{f:
                                                             C2RustUnnamed{f_odd:
                                                                               0.,
                                                                           f_even:
                                                                               0.,},},
                                              fp10:
                                                  __OSfp{f:
                                                             C2RustUnnamed{f_odd:
                                                                               0.,
                                                                           f_even:
                                                                               0.,},},
                                              fp12:
                                                  __OSfp{f:
                                                             C2RustUnnamed{f_odd:
                                                                               0.,
                                                                           f_even:
                                                                               0.,},},
                                              fp14:
                                                  __OSfp{f:
                                                             C2RustUnnamed{f_odd:
                                                                               0.,
                                                                           f_even:
                                                                               0.,},},
                                              fp16:
                                                  __OSfp{f:
                                                             C2RustUnnamed{f_odd:
                                                                               0.,
                                                                           f_even:
                                                                               0.,},},
                                              fp18:
                                                  __OSfp{f:
                                                             C2RustUnnamed{f_odd:
                                                                               0.,
                                                                           f_even:
                                                                               0.,},},
                                              fp20:
                                                  __OSfp{f:
                                                             C2RustUnnamed{f_odd:
                                                                               0.,
                                                                           f_even:
                                                                               0.,},},
                                              fp22:
                                                  __OSfp{f:
                                                             C2RustUnnamed{f_odd:
                                                                               0.,
                                                                           f_even:
                                                                               0.,},},
                                              fp24:
                                                  __OSfp{f:
                                                             C2RustUnnamed{f_odd:
                                                                               0.,
                                                                           f_even:
                                                                               0.,},},
                                              fp26:
                                                  __OSfp{f:
                                                             C2RustUnnamed{f_odd:
                                                                               0.,
                                                                           f_even:
                                                                               0.,},},
                                              fp28:
                                                  __OSfp{f:
                                                             C2RustUnnamed{f_odd:
                                                                               0.,
                                                                           f_even:
                                                                               0.,},},
                                              fp30:
                                                  __OSfp{f:
                                                             C2RustUnnamed{f_odd:
                                                                               0.,
                                                                           f_even:
                                                                               0.,},},},},
           inputs:
               [Input{cur:
                          OSContPad{button: 0,
                                    stick_x: 0,
                                    stick_y: 0,
                                    errno: 0,},
                      prev:
                          OSContPad{button: 0,
                                    stick_x: 0,
                                    stick_y: 0,
                                    errno: 0,},
                      press:
                          OSContPad{button: 0,
                                    stick_x: 0,
                                    stick_y: 0,
                                    errno: 0,},
                      rel:
                          OSContPad{button: 0,
                                    stick_x: 0,
                                    stick_y: 0,
                                    errno: 0,},}; 4],
           pads: [OSContPad{button: 0, stick_x: 0, stick_y: 0, errno: 0,}; 4],
           validCtrlrsMask: 0,
           nControllers: 0,
           ctrlrIsConnected: [0; 4],
           pakType: [0; 4],
           rumbleEnable: [0; 4],
           rumbleCounter: [0; 4],
           pfs:
               [OSPfs{status: 0,
                      queue: 0 as *const OSMesgQueue as *mut OSMesgQueue,
                      channel: 0,
                      id: [0; 32],
                      label: [0; 32],
                      version: 0,
                      dir_size: 0,
                      inode_table: 0,
                      minode_table: 0,
                      dir_table: 0,
                      inodeStartPage: 0,
                      banks: 0,
                      activebank: 0,}; 4],
           rumbleOffFrames: 0,
           rumbleOnFrames: 0,
           preNMIShutdown: 0,
           retraceCallback: None,
           retraceCallbackValue: 0,};
#[no_mangle]
pub static mut gIrqMgr: IrqMgr =
    IrqMgr{retraceMsg: OSScMsg{type_0: 0, misc: [0; 30],},
           prenmiMsg: OSScMsg{type_0: 0, misc: [0; 30],},
           nmiMsg: OSScMsg{type_0: 0, misc: [0; 30],},
           queue:
               OSMesgQueue{mtqueue: 0 as *const OSThread as *mut OSThread,
                           fullqueue: 0 as *const OSThread as *mut OSThread,
                           validCount: 0,
                           first: 0,
                           msgCount: 0,
                           msg: 0 as *const OSMesg as *mut OSMesg,},
           msgBuf: [0 as *const libc::c_void as *mut libc::c_void; 8],
           thread:
               OSThread{next: 0 as *const OSThread as *mut OSThread,
                        priority: 0,
                        queue:
                            0 as *const *mut OSThread as *mut *mut OSThread,
                        tlnext: 0 as *const OSThread as *mut OSThread,
                        state: 0,
                        flags: 0,
                        id: 0,
                        fp: 0,
                        thprof:
                            0 as *const __OSThreadprofile as
                                *mut __OSThreadprofile,
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
                                                             C2RustUnnamed{f_odd:
                                                                               0.,
                                                                           f_even:
                                                                               0.,},},
                                              fp2:
                                                  __OSfp{f:
                                                             C2RustUnnamed{f_odd:
                                                                               0.,
                                                                           f_even:
                                                                               0.,},},
                                              fp4:
                                                  __OSfp{f:
                                                             C2RustUnnamed{f_odd:
                                                                               0.,
                                                                           f_even:
                                                                               0.,},},
                                              fp6:
                                                  __OSfp{f:
                                                             C2RustUnnamed{f_odd:
                                                                               0.,
                                                                           f_even:
                                                                               0.,},},
                                              fp8:
                                                  __OSfp{f:
                                                             C2RustUnnamed{f_odd:
                                                                               0.,
                                                                           f_even:
                                                                               0.,},},
                                              fp10:
                                                  __OSfp{f:
                                                             C2RustUnnamed{f_odd:
                                                                               0.,
                                                                           f_even:
                                                                               0.,},},
                                              fp12:
                                                  __OSfp{f:
                                                             C2RustUnnamed{f_odd:
                                                                               0.,
                                                                           f_even:
                                                                               0.,},},
                                              fp14:
                                                  __OSfp{f:
                                                             C2RustUnnamed{f_odd:
                                                                               0.,
                                                                           f_even:
                                                                               0.,},},
                                              fp16:
                                                  __OSfp{f:
                                                             C2RustUnnamed{f_odd:
                                                                               0.,
                                                                           f_even:
                                                                               0.,},},
                                              fp18:
                                                  __OSfp{f:
                                                             C2RustUnnamed{f_odd:
                                                                               0.,
                                                                           f_even:
                                                                               0.,},},
                                              fp20:
                                                  __OSfp{f:
                                                             C2RustUnnamed{f_odd:
                                                                               0.,
                                                                           f_even:
                                                                               0.,},},
                                              fp22:
                                                  __OSfp{f:
                                                             C2RustUnnamed{f_odd:
                                                                               0.,
                                                                           f_even:
                                                                               0.,},},
                                              fp24:
                                                  __OSfp{f:
                                                             C2RustUnnamed{f_odd:
                                                                               0.,
                                                                           f_even:
                                                                               0.,},},
                                              fp26:
                                                  __OSfp{f:
                                                             C2RustUnnamed{f_odd:
                                                                               0.,
                                                                           f_even:
                                                                               0.,},},
                                              fp28:
                                                  __OSfp{f:
                                                             C2RustUnnamed{f_odd:
                                                                               0.,
                                                                           f_even:
                                                                               0.,},},
                                              fp30:
                                                  __OSfp{f:
                                                             C2RustUnnamed{f_odd:
                                                                               0.,
                                                                           f_even:
                                                                               0.,},},},},
           clients: 0 as *const IrqMgrClient as *mut IrqMgrClient,
           resetStatus: 0,
           resetTime: 0,
           timer:
               OSTimer{next: 0 as *const OSTimer as *mut OSTimer,
                       prev: 0 as *const OSTimer as *mut OSTimer,
                       interval: 0,
                       value: 0,
                       mq: 0 as *const OSMesgQueue as *mut OSMesgQueue,
                       msg: 0 as *const libc::c_void as *mut libc::c_void,},
           retraceTime: 0,};
#[no_mangle]
pub static mut gSegments: [u32_0; 16] = [0; 16];
#[no_mangle]
pub static mut sGraphThread: OSThread =
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
pub static mut sGraphStack: [u8_0; 6144] = [0; 6144];
#[no_mangle]
pub static mut sSchedStack: [u8_0; 1536] = [0; 1536];
#[no_mangle]
pub static mut sAudioStack: [u8_0; 2048] = [0; 2048];
#[no_mangle]
pub static mut sPadMgrStack: [u8_0; 1280] = [0; 1280];
#[no_mangle]
pub static mut sIrqMgrStack: [u8_0; 1280] = [0; 1280];
#[no_mangle]
pub static mut sGraphStackInfo: StackEntry =
    StackEntry{next: 0 as *const StackEntry as *mut StackEntry,
               prev: 0 as *const StackEntry as *mut StackEntry,
               head: 0,
               tail: 0,
               initValue: 0,
               minSpace: 0,
               name: 0 as *const libc::c_char,};
#[no_mangle]
pub static mut sSchedStackInfo: StackEntry =
    StackEntry{next: 0 as *const StackEntry as *mut StackEntry,
               prev: 0 as *const StackEntry as *mut StackEntry,
               head: 0,
               tail: 0,
               initValue: 0,
               minSpace: 0,
               name: 0 as *const libc::c_char,};
#[no_mangle]
pub static mut sAudioStackInfo: StackEntry =
    StackEntry{next: 0 as *const StackEntry as *mut StackEntry,
               prev: 0 as *const StackEntry as *mut StackEntry,
               head: 0,
               tail: 0,
               initValue: 0,
               minSpace: 0,
               name: 0 as *const libc::c_char,};
#[no_mangle]
pub static mut sPadMgrStackInfo: StackEntry =
    StackEntry{next: 0 as *const StackEntry as *mut StackEntry,
               prev: 0 as *const StackEntry as *mut StackEntry,
               head: 0,
               tail: 0,
               initValue: 0,
               minSpace: 0,
               name: 0 as *const libc::c_char,};
#[no_mangle]
pub static mut sIrqMgrStackInfo: StackEntry =
    StackEntry{next: 0 as *const StackEntry as *mut StackEntry,
               prev: 0 as *const StackEntry as *mut StackEntry,
               head: 0,
               tail: 0,
               initValue: 0,
               minSpace: 0,
               name: 0 as *const libc::c_char,};
#[no_mangle]
pub static mut gAudioMgr: AudioMgr =
    AudioMgr{irqMgr: 0 as *const IrqMgr as *mut IrqMgr,
             sched: 0 as *const SchedContext as *mut SchedContext,
             audioTask:
                 OSScTask{next: 0 as *const OSScTask as *mut OSScTask,
                          state: 0,
                          flags: 0,
                          framebuffer: 0 as *const CfbInfo as *mut CfbInfo,
                          list:
                              OSTask{t:
                                         OSTask_t{type_0: 0,
                                                  flags: 0,
                                                  ucode_boot:
                                                      0 as *const u64_0 as
                                                          *mut u64_0,
                                                  ucode_boot_size: 0,
                                                  ucode:
                                                      0 as *const u64_0 as
                                                          *mut u64_0,
                                                  ucode_size: 0,
                                                  ucode_data:
                                                      0 as *const u64_0 as
                                                          *mut u64_0,
                                                  ucode_data_size: 0,
                                                  dram_stack:
                                                      0 as *const u64_0 as
                                                          *mut u64_0,
                                                  dram_stack_size: 0,
                                                  output_buff:
                                                      0 as *const u64_0 as
                                                          *mut u64_0,
                                                  output_buff_size:
                                                      0 as *const u64_0 as
                                                          *mut u64_0,
                                                  data_ptr:
                                                      0 as *const u64_0 as
                                                          *mut u64_0,
                                                  data_size: 0,
                                                  yield_data_ptr:
                                                      0 as *const u64_0 as
                                                          *mut u64_0,
                                                  yield_data_size: 0,},},
                          msgQ: 0 as *const OSMesgQueue as *mut OSMesgQueue,
                          msg:
                              0 as *const libc::c_void as *mut libc::c_void,},
             unk_60: [0; 16],
             rspTask: 0 as *const AudioTask as *mut AudioTask,
             unk_74:
                 OSMesgQueue{mtqueue: 0 as *const OSThread as *mut OSThread,
                             fullqueue: 0 as *const OSThread as *mut OSThread,
                             validCount: 0,
                             first: 0,
                             msgCount: 0,
                             msg: 0 as *const OSMesg as *mut OSMesg,},
             unk_8C: 0 as *const libc::c_void as *mut libc::c_void,
             unk_90:
                 OSMesgQueue{mtqueue: 0 as *const OSThread as *mut OSThread,
                             fullqueue: 0 as *const OSThread as *mut OSThread,
                             validCount: 0,
                             first: 0,
                             msgCount: 0,
                             msg: 0 as *const OSMesg as *mut OSMesg,},
             unk_A8: 0 as *const libc::c_void as *mut libc::c_void,
             unk_AC:
                 OSMesgQueue{mtqueue: 0 as *const OSThread as *mut OSThread,
                             fullqueue: 0 as *const OSThread as *mut OSThread,
                             validCount: 0,
                             first: 0,
                             msgCount: 0,
                             msg: 0 as *const OSMesg as *mut OSMesg,},
             unk_C4: 0 as *const libc::c_void as *mut libc::c_void,
             unk_C8:
                 OSMesgQueue{mtqueue: 0 as *const OSThread as *mut OSThread,
                             fullqueue: 0 as *const OSThread as *mut OSThread,
                             validCount: 0,
                             first: 0,
                             msgCount: 0,
                             msg: 0 as *const OSMesg as *mut OSMesg,},
             unk_E0: 0 as *const libc::c_void as *mut libc::c_void,
             unk_E4: [0; 4],
             unk_E8:
                 OSThread{next: 0 as *const OSThread as *mut OSThread,
                          priority: 0,
                          queue:
                              0 as *const *mut OSThread as *mut *mut OSThread,
                          tlnext: 0 as *const OSThread as *mut OSThread,
                          state: 0,
                          flags: 0,
                          id: 0,
                          fp: 0,
                          thprof:
                              0 as *const __OSThreadprofile as
                                  *mut __OSThreadprofile,
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
                                                               C2RustUnnamed{f_odd:
                                                                                 0.,
                                                                             f_even:
                                                                                 0.,},},
                                                fp2:
                                                    __OSfp{f:
                                                               C2RustUnnamed{f_odd:
                                                                                 0.,
                                                                             f_even:
                                                                                 0.,},},
                                                fp4:
                                                    __OSfp{f:
                                                               C2RustUnnamed{f_odd:
                                                                                 0.,
                                                                             f_even:
                                                                                 0.,},},
                                                fp6:
                                                    __OSfp{f:
                                                               C2RustUnnamed{f_odd:
                                                                                 0.,
                                                                             f_even:
                                                                                 0.,},},
                                                fp8:
                                                    __OSfp{f:
                                                               C2RustUnnamed{f_odd:
                                                                                 0.,
                                                                             f_even:
                                                                                 0.,},},
                                                fp10:
                                                    __OSfp{f:
                                                               C2RustUnnamed{f_odd:
                                                                                 0.,
                                                                             f_even:
                                                                                 0.,},},
                                                fp12:
                                                    __OSfp{f:
                                                               C2RustUnnamed{f_odd:
                                                                                 0.,
                                                                             f_even:
                                                                                 0.,},},
                                                fp14:
                                                    __OSfp{f:
                                                               C2RustUnnamed{f_odd:
                                                                                 0.,
                                                                             f_even:
                                                                                 0.,},},
                                                fp16:
                                                    __OSfp{f:
                                                               C2RustUnnamed{f_odd:
                                                                                 0.,
                                                                             f_even:
                                                                                 0.,},},
                                                fp18:
                                                    __OSfp{f:
                                                               C2RustUnnamed{f_odd:
                                                                                 0.,
                                                                             f_even:
                                                                                 0.,},},
                                                fp20:
                                                    __OSfp{f:
                                                               C2RustUnnamed{f_odd:
                                                                                 0.,
                                                                             f_even:
                                                                                 0.,},},
                                                fp22:
                                                    __OSfp{f:
                                                               C2RustUnnamed{f_odd:
                                                                                 0.,
                                                                             f_even:
                                                                                 0.,},},
                                                fp24:
                                                    __OSfp{f:
                                                               C2RustUnnamed{f_odd:
                                                                                 0.,
                                                                             f_even:
                                                                                 0.,},},
                                                fp26:
                                                    __OSfp{f:
                                                               C2RustUnnamed{f_odd:
                                                                                 0.,
                                                                             f_even:
                                                                                 0.,},},
                                                fp28:
                                                    __OSfp{f:
                                                               C2RustUnnamed{f_odd:
                                                                                 0.,
                                                                             f_even:
                                                                                 0.,},},
                                                fp30:
                                                    __OSfp{f:
                                                               C2RustUnnamed{f_odd:
                                                                                 0.,
                                                                             f_even:
                                                                                 0.,},},},},};
#[no_mangle]
pub static mut sSiIntMsgQ: OSMesgQueue =
    OSMesgQueue{mtqueue: 0 as *const OSThread as *mut OSThread,
                fullqueue: 0 as *const OSThread as *mut OSThread,
                validCount: 0,
                first: 0,
                msgCount: 0,
                msg: 0 as *const OSMesg as *mut OSMesg,};
#[no_mangle]
pub static mut sSiIntMsgBuf: [OSMesg; 1] =
    [0 as *const libc::c_void as *mut libc::c_void; 1];
#[no_mangle]
pub unsafe extern "C" fn Main_LogSystemHeap() {
    osSyncPrintf(b"\x1b[32m\x00" as *const u8 as *const libc::c_char);
    // "System heap size% 08x (% dKB) Start address% 08x"
    osSyncPrintf(b"\xe3\x82\xb7\xe3\x82\xb9\xe3\x83\x86\xe3\x83\xa0\xe3\x83\x92\xe3\x83\xbc\xe3\x83\x97\xe3\x82\xb5\xe3\x82\xa4\xe3\x82\xba %08x(%dKB) \xe9\x96\x8b\xe5\xa7\x8b\xe3\x82\xa2\xe3\x83\x89\xe3\x83\xac\xe3\x82\xb9 %08x\n\x00"
                     as *const u8 as *const libc::c_char, gSystemHeapSize,
                 gSystemHeapSize.wrapping_div(1024 as libc::c_int as
                                                  libc::c_uint),
                 gSystemHeap.as_mut_ptr()); // "Start running"
    osSyncPrintf(b"\x1b[m\x00" as *const u8 as *const libc::c_char);
}
#[no_mangle]
pub unsafe extern "C" fn Main(mut arg: *mut libc::c_void) {
    let mut irqClient: IrqMgrClient =
        IrqMgrClient{prev: 0 as *const IrqMgrClient as *mut IrqMgrClient,
                     queue: 0 as *const OSMesgQueue as *mut OSMesgQueue,};
    let mut irqMgrMsgQ: OSMesgQueue =
        OSMesgQueue{mtqueue: 0 as *const OSThread as *mut OSThread,
                    fullqueue: 0 as *const OSThread as *mut OSThread,
                    validCount: 0,
                    first: 0,
                    msgCount: 0,
                    msg: 0 as *const OSMesg as *mut OSMesg,};
    let mut irqMgrMsgBuf: [OSMesg; 60] = [0 as *mut libc::c_void; 60];
    let mut sysHeap: u32_0 = 0;
    let mut fb: u32_0 = 0;
    let mut debugHeap: s32 = 0;
    let mut debugHeapSize: s32 = 0;
    let mut msg: *mut s16 = 0 as *mut s16;
    osSyncPrintf(b"mainproc \xe5\xae\x9f\xe8\xa1\x8c\xe9\x96\x8b\xe5\xa7\x8b\n\x00"
                     as *const u8 as *const libc::c_char);
    gScreenWidth = 320 as libc::c_int;
    gScreenHeight = 240 as libc::c_int;
    gAppNmiBufferPtr = osAppNmiBuffer.as_mut_ptr() as *mut PreNmiBuff;
    PreNmiBuff_Init(gAppNmiBufferPtr);
    Fault_Init();
    SysCfb_Init(0 as libc::c_int);
    sysHeap = gSystemHeap.as_mut_ptr() as u32_0;
    fb = SysCfb_GetFbPtr(0 as libc::c_int);
    gSystemHeapSize = fb.wrapping_sub(sysHeap);
    // "System heap initalization"
    osSyncPrintf(b"\xe3\x82\xb7\xe3\x82\xb9\xe3\x83\x86\xe3\x83\xa0\xe3\x83\x92\xe3\x83\xbc\xe3\x83\x97\xe5\x88\x9d\xe6\x9c\x9f\xe5\x8c\x96 %08x-%08x %08x\n\x00"
                     as *const u8 as *const libc::c_char, sysHeap, fb,
                 gSystemHeapSize); // initializes the system heap
    SystemHeap_Init(sysHeap as *mut libc::c_void,
                    gSystemHeapSize); // "Initialize the task scheduler"
    if osMemSize >= 0x800000 as libc::c_int as libc::c_uint {
        debugHeap = SysCfb_GetFbEnd() as s32; // "Looks like it's been reset"
        debugHeapSize =
            (0x80600000 as
                 libc::c_uint).wrapping_sub(debugHeap as libc::c_uint) as s32
    } else {
        debugHeapSize = 0x400 as libc::c_int; // "Cleanup"
        debugHeap =
            SystemArena_MallocDebug(debugHeapSize as u32_0,
                                    b"../main.c\x00" as *const u8 as
                                        *const libc::c_char,
                                    565 as libc::c_int) as s32
    }
    osSyncPrintf(b"debug_InitArena(%08x, %08x)\n\x00" as *const u8 as
                     *const libc::c_char, debugHeap, debugHeapSize);
    DebugArena_Init(debugHeap as *mut libc::c_void, debugHeapSize as u32_0);
    func_800636C0();
    (*gGameInfo).data[(1 as libc::c_int * 6 as libc::c_int * 16 as libc::c_int
                           + 0 as libc::c_int) as usize] =
        0 as libc::c_int as s16;
    osCreateMesgQueue(&mut sSiIntMsgQ, sSiIntMsgBuf.as_mut_ptr(),
                      1 as libc::c_int);
    osSetEventMesg(5 as libc::c_int as OSEvent, &mut sSiIntMsgQ, 0 as OSMesg);
    Main_LogSystemHeap();
    osCreateMesgQueue(&mut irqMgrMsgQ, irqMgrMsgBuf.as_mut_ptr(),
                      0x3c as libc::c_int);
    StackCheck_Init(&mut sIrqMgrStackInfo,
                    sIrqMgrStack.as_mut_ptr() as *mut libc::c_void,
                    sIrqMgrStack.as_mut_ptr().offset(::std::mem::size_of::<[u8_0; 1280]>()
                                                         as libc::c_ulong as
                                                         isize) as
                        *mut libc::c_void, 0 as libc::c_int as u32_0,
                    0x100 as libc::c_int,
                    b"irqmgr\x00" as *const u8 as *const libc::c_char);
    IrqMgr_Init(&mut gIrqMgr,
                &mut sGraphStackInfo as *mut StackEntry as *mut libc::c_void,
                17 as libc::c_int, 1 as libc::c_int as u8_0);
    osSyncPrintf(b"\xe3\x82\xbf\xe3\x82\xb9\xe3\x82\xaf\xe3\x82\xb9\xe3\x82\xb1\xe3\x82\xb8\xe3\x83\xa5\xe3\x83\xbc\xe3\x83\xa9\xe3\x81\xae\xe5\x88\x9d\xe6\x9c\x9f\xe5\x8c\x96\n\x00"
                     as *const u8 as *const libc::c_char);
    StackCheck_Init(&mut sSchedStackInfo,
                    sSchedStack.as_mut_ptr() as *mut libc::c_void,
                    sSchedStack.as_mut_ptr().offset(::std::mem::size_of::<[u8_0; 1536]>()
                                                        as libc::c_ulong as
                                                        isize) as
                        *mut libc::c_void, 0 as libc::c_int as u32_0,
                    0x100 as libc::c_int,
                    b"sched\x00" as *const u8 as *const libc::c_char);
    Sched_Init(&mut gSchedContext,
               &mut sAudioStack as *mut [u8_0; 2048] as *mut libc::c_void,
               15 as libc::c_int, D_80013960 as s32, 1 as libc::c_int,
               &mut gIrqMgr);
    IrqMgr_AddClient(&mut gIrqMgr, &mut irqClient, &mut irqMgrMsgQ);
    StackCheck_Init(&mut sAudioStackInfo,
                    sAudioStack.as_mut_ptr() as *mut libc::c_void,
                    sAudioStack.as_mut_ptr().offset(::std::mem::size_of::<[u8_0; 2048]>()
                                                        as libc::c_ulong as
                                                        isize) as
                        *mut libc::c_void, 0 as libc::c_int as u32_0,
                    0x100 as libc::c_int,
                    b"audio\x00" as *const u8 as *const libc::c_char);
    AudioMgr_Init(&mut gAudioMgr,
                  sAudioStack.as_mut_ptr().offset(::std::mem::size_of::<[u8_0; 2048]>()
                                                      as libc::c_ulong as
                                                      isize) as
                      *mut libc::c_void, 12 as libc::c_int,
                  0xa as libc::c_int, &mut gSchedContext, &mut gIrqMgr);
    StackCheck_Init(&mut sPadMgrStackInfo,
                    sPadMgrStack.as_mut_ptr() as *mut libc::c_void,
                    sPadMgrStack.as_mut_ptr().offset(::std::mem::size_of::<[u8_0; 1280]>()
                                                         as libc::c_ulong as
                                                         isize) as
                        *mut libc::c_void, 0 as libc::c_int as u32_0,
                    0x100 as libc::c_int,
                    b"padmgr\x00" as *const u8 as *const libc::c_char);
    PadMgr_Init(&mut gPadMgr, &mut sSiIntMsgQ, &mut gIrqMgr, 7 as libc::c_int,
                14 as libc::c_int,
                &mut sIrqMgrStack as *mut [u8_0; 1280] as *mut libc::c_void);
    AudioMgr_Unlock(&mut gAudioMgr);
    StackCheck_Init(&mut sGraphStackInfo,
                    sGraphStack.as_mut_ptr() as *mut libc::c_void,
                    sGraphStack.as_mut_ptr().offset(::std::mem::size_of::<[u8_0; 6144]>()
                                                        as libc::c_ulong as
                                                        isize) as
                        *mut libc::c_void, 0 as libc::c_int as u32_0,
                    0x100 as libc::c_int,
                    b"graph\x00" as *const u8 as *const libc::c_char);
    osCreateThread(&mut sGraphThread, 4 as libc::c_int,
                   Some(Graph_ThreadEntry as
                            unsafe extern "C" fn(_: *mut libc::c_void) -> ()),
                   arg,
                   sGraphStack.as_mut_ptr().offset(::std::mem::size_of::<[u8_0; 6144]>()
                                                       as libc::c_ulong as
                                                       isize) as
                       *mut libc::c_void, 11 as libc::c_int);
    osStartThread(&mut sGraphThread);
    osSetThreadPri(0 as *mut OSThread, 15 as libc::c_int);
    loop  {
        msg = 0 as *mut s16;
        osRecvMesg(&mut irqMgrMsgQ,
                   &mut msg as *mut *mut s16 as OSMesg as *mut OSMesg,
                   1 as libc::c_int);
        if msg.is_null() { break ; }
        if *msg as libc::c_int == 4 as libc::c_int {
            osSyncPrintf(b"main.c: \xe3\x83\xaa\xe3\x82\xbb\xe3\x83\x83\xe3\x83\x88\xe3\x81\x95\xe3\x82\x8c\xe3\x81\x9f\xe3\x81\xbf\xe3\x81\x9f\xe3\x81\x84\xe3\x81\xa0\xe3\x82\x88\n\x00"
                             as *const u8 as *const libc::c_char);
            PreNmiBuff_SetReset(gAppNmiBufferPtr);
        }
    }
    osSyncPrintf(b"mainproc \xe5\xbe\x8c\xe5\xa7\x8b\xe6\x9c\xab\n\x00" as
                     *const u8 as *const libc::c_char);
    osDestroyThread(&mut sGraphThread);
    func_800FBFD8();
    osSyncPrintf(b"mainproc \xe5\xae\x9f\xe8\xa1\x8c\xe7\xb5\x82\xe4\xba\x86\n\x00"
                     as *const u8 as *const libc::c_char);
    // "End of execution"
}
