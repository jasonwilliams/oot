#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case,
         non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(const_raw_ptr_to_usize_cast, register_tool)]
extern "C" {
    #[no_mangle]
    fn DmaMgr_DmaHandler(pihandle: *mut OSPiHandle, mb: *mut OSIoMesg,
                         direction: s32) -> s32;
    #[no_mangle]
    fn osSyncPrintf(fmt: *const libc::c_char, _: ...);
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
    fn osCreateMesgQueue(mq: *mut OSMesgQueue, msg: *mut OSMesg, count: s32);
    #[no_mangle]
    fn osGetTime() -> OSTime;
    #[no_mangle]
    fn osStartThread(thread: *mut OSThread);
    #[no_mangle]
    fn Sched_SendEntryMsg(sc: *mut SchedContext);
    #[no_mangle]
    fn IrqMgr_AddClient(this: *mut IrqMgr, c: *mut IrqMgrClient,
                        msgQ: *mut OSMesgQueue);
    #[no_mangle]
    fn AudioLoad_SetDmaHandler(callback: DmaHandler);
    #[no_mangle]
    fn func_800E4FE0() -> *mut AudioTask;
    #[no_mangle]
    fn Audio_PreNMI();
    #[no_mangle]
    fn Audio_Init();
    #[no_mangle]
    fn Audio_InitSound();
    #[no_mangle]
    static mut D_8016A550: OSTime;
    #[no_mangle]
    static mut D_8016A558: OSTime;
    #[no_mangle]
    static mut gGameInfo: *mut GameInfo;
}
pub type s8 = libc::c_schar;
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
pub struct __OSBlockInfo {
    pub errStatus: u32_0,
    pub dramAddr: *mut libc::c_void,
    pub C2Addr: *mut libc::c_void,
    pub sectorSize: u32_0,
    pub C1ErrNum: u32_0,
    pub C1ErrSector: [u32_0; 4],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __OSTranxInfo {
    pub cmdType: u32_0,
    pub transferMode: u16_0,
    pub blockNum: u16_0,
    pub sectorNum: s32,
    pub devAddr: u32_0,
    pub bmCtlShadow: u32_0,
    pub seqCtlShadow: u32_0,
    pub block: [__OSBlockInfo; 2],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct OSPiHandle {
    pub next: *mut OSPiHandle,
    pub type_0: u8_0,
    pub latency: u8_0,
    pub pageSize: u8_0,
    pub relDuration: u8_0,
    pub pulse: u8_0,
    pub domain: u8_0,
    pub baseAddress: u32_0,
    pub speed: u32_0,
    pub transferInfo: __OSTranxInfo,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct OSIoMesgHdr {
    pub type_0: u16_0,
    pub pri: u8_0,
    pub status: u8_0,
    pub retQueue: *mut OSMesgQueue,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct OSIoMesg {
    pub hdr: OSIoMesgHdr,
    pub dramAddr: *mut libc::c_void,
    pub devAddr: u32_0,
    pub size: size_t,
    pub piHandle: *mut OSPiHandle,
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
pub type DmaHandler
    =
    Option<unsafe extern "C" fn(_: *mut OSPiHandle, _: *mut OSIoMesg, _: s32)
               -> s32>;
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
#[no_mangle]
pub unsafe extern "C" fn func_800C3C80(mut audioMgr: *mut AudioMgr) {
    let mut task: *mut AudioTask = 0 as *mut AudioTask;
    task = (*audioMgr).rspTask;
    if !(*(*audioMgr).rspTask).taskQueue.is_null() {
        osSendMesg((*task).taskQueue, 0 as *mut libc::c_void,
                   1 as libc::c_int);
    };
}
#[no_mangle]
pub unsafe extern "C" fn AudioMgr_HandleRetrace(mut audioMgr: *mut AudioMgr) {
    let mut rspTask: *mut AudioTask = 0 as *mut AudioTask;
    if (*gGameInfo).data[(1 as libc::c_int * 6 as libc::c_int *
                              16 as libc::c_int + 20 as libc::c_int) as usize]
           as libc::c_int > 0 as libc::c_int {
        (*audioMgr).rspTask = 0 as *mut AudioTask
    }
    if !(*audioMgr).rspTask.is_null() {
        (*audioMgr).audioTask.next = 0 as *mut OSScTask;
        (*audioMgr).audioTask.flags = 2 as libc::c_int as u32_0;
        (*audioMgr).audioTask.framebuffer = 0 as *mut CfbInfo;
        (*audioMgr).audioTask.list = (*(*audioMgr).rspTask).task;
        (*audioMgr).audioTask.msgQ = &mut (*audioMgr).unk_AC;
        (*audioMgr).audioTask.msg = 0 as *mut libc::c_void;
        osSendMesg(&mut (*(*audioMgr).sched).cmdQ,
                   &mut (*audioMgr).audioTask as *mut OSScTask as OSMesg,
                   1 as libc::c_int);
        Sched_SendEntryMsg((*audioMgr).sched);
    }
    ::std::ptr::write_volatile(&mut D_8016A550 as *mut OSTime, osGetTime());
    if (*gGameInfo).data[(1 as libc::c_int * 6 as libc::c_int *
                              16 as libc::c_int + 20 as libc::c_int) as usize]
           as libc::c_int >= 2 as libc::c_int {
        rspTask = 0 as *mut AudioTask
    } else { rspTask = func_800E4FE0() }
    ::std::ptr::write_volatile(&mut D_8016A558 as *mut OSTime,
                               (::std::ptr::read_volatile::<OSTime>(&D_8016A558
                                                                        as
                                                                        *const OSTime)
                                    as
                                    libc::c_ulonglong).wrapping_add(osGetTime().wrapping_sub(D_8016A550))
                                   as OSTime as OSTime);
    ::std::ptr::write_volatile(&mut D_8016A550 as *mut OSTime,
                               0 as libc::c_int as OSTime);
    if !(*audioMgr).rspTask.is_null() {
        osRecvMesg(&mut (*audioMgr).unk_AC, 0 as *mut OSMesg,
                   1 as libc::c_int);
        func_800C3C80(audioMgr);
    }
    (*audioMgr).rspTask = rspTask;
}
#[no_mangle]
pub unsafe extern "C" fn AudioMgr_HandlePRENMI(mut audioMgr: *mut AudioMgr) {
    // "Audio manager received OS_SC_PRE_NMI_MSG"
    osSyncPrintf(b"\xe3\x82\xaa\xe3\x83\xbc\xe3\x83\x87\xe3\x82\xa3\xe3\x82\xaa\xe3\x83\x9e\xe3\x83\x8d\xe3\x83\xbc\xe3\x82\xb8\xe3\x83\xa3\xe3\x81\x8c OS_SC_PRE_NMI_MSG \xe3\x82\x92\xe5\x8f\x97\xe3\x81\x91\xe5\x8f\x96\xe3\x82\x8a\xe3\x81\xbe\xe3\x81\x97\xe3\x81\x9f\n\x00"
                     as *const u8 as
                     *const libc::c_char); // "Start running audio manager thread"
    Audio_PreNMI();
}
#[no_mangle]
pub unsafe extern "C" fn AudioMgr_ThreadEntry(mut arg0: *mut libc::c_void) {
    let mut audioMgr: *mut AudioMgr = arg0 as *mut AudioMgr;
    let mut irqClient: IrqMgrClient =
        IrqMgrClient{prev: 0 as *mut IrqMgrClient,
                     queue: 0 as *mut OSMesgQueue,};
    let mut msg: *mut s16 = 0 as *mut s16;
    osSyncPrintf(b"\xe3\x82\xaa\xe3\x83\xbc\xe3\x83\x87\xe3\x82\xa3\xe3\x82\xaa\xe3\x83\x9e\xe3\x83\x8d\xe3\x83\xbc\xe3\x82\xb8\xe3\x83\xa3\xe3\x82\xb9\xe3\x83\xac\xe3\x83\x83\xe3\x83\x89\xe5\xae\x9f\xe8\xa1\x8c\xe9\x96\x8b\xe5\xa7\x8b\n\x00"
                     as *const u8 as *const libc::c_char);
    Audio_Init();
    AudioLoad_SetDmaHandler(Some(DmaMgr_DmaHandler as
                                     unsafe extern "C" fn(_: *mut OSPiHandle,
                                                          _: *mut OSIoMesg,
                                                          _: s32) -> s32));
    Audio_InitSound();
    osSendMesg(&mut (*audioMgr).unk_C8, 0 as *mut libc::c_void,
               1 as libc::c_int);
    IrqMgr_AddClient((*audioMgr).irqMgr, &mut irqClient,
                     &mut (*audioMgr).unk_74);
    loop  {
        osRecvMesg(&mut (*audioMgr).unk_74,
                   &mut msg as *mut *mut s16 as *mut OSMesg,
                   1 as libc::c_int);
        match *msg as libc::c_int {
            1 => {
                AudioMgr_HandleRetrace(audioMgr);
                while (*audioMgr).unk_74.validCount != 0 as libc::c_int {
                    osRecvMesg(&mut (*audioMgr).unk_74,
                               &mut msg as *mut *mut s16 as *mut OSMesg,
                               1 as libc::c_int);
                    match *msg as libc::c_int {
                        4 => { AudioMgr_HandlePRENMI(audioMgr); }
                        1 | _ => { }
                    }
                }
            }
            4 => { AudioMgr_HandlePRENMI(audioMgr); }
            _ => { }
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn AudioMgr_Unlock(mut audioMgr: *mut AudioMgr) {
    osRecvMesg(&mut (*audioMgr).unk_C8, 0 as *mut OSMesg, 1 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn AudioMgr_Init(mut audioMgr: *mut AudioMgr,
                                       mut stack: *mut libc::c_void,
                                       mut pri: OSPri, mut id: OSId,
                                       mut sched: *mut SchedContext,
                                       mut irqMgr: *mut IrqMgr) {
    bzero(audioMgr as *mut libc::c_void,
          ::std::mem::size_of::<AudioMgr>() as libc::c_ulong);
    (*audioMgr).sched = sched;
    (*audioMgr).irqMgr = irqMgr;
    (*audioMgr).rspTask = 0 as *mut AudioTask;
    osCreateMesgQueue(&mut (*audioMgr).unk_AC, &mut (*audioMgr).unk_C4,
                      1 as libc::c_int);
    osCreateMesgQueue(&mut (*audioMgr).unk_74, &mut (*audioMgr).unk_8C,
                      8 as libc::c_int);
    osCreateMesgQueue(&mut (*audioMgr).unk_C8, &mut (*audioMgr).unk_E0,
                      1 as libc::c_int);
    osSendMesg(&mut (*audioMgr).unk_AC, 0 as *mut libc::c_void,
               1 as libc::c_int);
    osCreateThread(&mut (*audioMgr).unk_E8, id,
                   Some(AudioMgr_ThreadEntry as
                            unsafe extern "C" fn(_: *mut libc::c_void) -> ()),
                   audioMgr as *mut libc::c_void, stack, pri);
    osStartThread(&mut (*audioMgr).unk_E8);
}
