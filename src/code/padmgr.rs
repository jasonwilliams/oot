#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case,
         non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(const_raw_ptr_to_usize_cast, register_tool)]
extern "C" {
    #[no_mangle]
    fn osSyncPrintf(fmt: *const libc::c_char, _: ...);
    #[no_mangle]
    fn LogUtils_CheckNullPointer(exp: *const libc::c_char,
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
    fn osGetThreadId(thread: *mut OSThread) -> OSId;
    #[no_mangle]
    fn osCreateMesgQueue(mq: *mut OSMesgQueue, msg: *mut OSMesg, count: s32);
    #[no_mangle]
    fn osGetTime() -> OSTime;
    #[no_mangle]
    fn osStartThread(thread: *mut OSThread);
    #[no_mangle]
    fn IrqMgr_AddClient(this: *mut IrqMgr, c: *mut IrqMgrClient,
                        msgQ: *mut OSMesgQueue);
    #[no_mangle]
    fn IrqMgr_RemoveClient(this: *mut IrqMgr, c: *mut IrqMgrClient);
    #[no_mangle]
    fn Fault_AddHungupAndCrash(_: *const libc::c_char, _: u32_0);
    #[no_mangle]
    fn PadUtils_UpdateRelXY(input: *mut Input);
    #[no_mangle]
    fn PadSetup_Init(mq: *mut OSMesgQueue, outMask: *mut u8_0,
                     status: *mut OSContStatus) -> s32;
    #[no_mangle]
    fn osMotorInit(ctrlrqueue: *mut OSMesgQueue, pfs: *mut OSPfs,
                   channel: s32) -> s32;
    #[no_mangle]
    fn __osMotorAccess(pfs: *mut OSPfs, vibrate: u32_0) -> s32;
    #[no_mangle]
    static mut gFaultStruct: FaultThreadStruct;
    #[no_mangle]
    fn osContGetQuery(data: *mut OSContStatus);
    #[no_mangle]
    fn osContStartQuery(mq: *mut OSMesgQueue) -> s32;
    #[no_mangle]
    fn osContGetReadData(pad: *mut OSContPad);
    #[no_mangle]
    fn osContStartReadData(mq: *mut OSMesgQueue) -> s32;
    #[no_mangle]
    fn osContSetCh(ch: u8_0) -> s32;
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
pub struct FaultClient {
    pub next: *mut FaultClient,
    pub callback: u32_0,
    pub param1: u32_0,
    pub param2: u32_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct FaultAddrConvClient {
    pub next: *mut FaultAddrConvClient,
    pub callback: u32_0,
    pub param: u32_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct FaultThreadStruct {
    pub thread: OSThread,
    pub unk_1B0: [u8_0; 1536],
    pub queue: OSMesgQueue,
    pub msg: OSMesg,
    pub exitDebugger: u8_0,
    pub msgId: u8_0,
    pub faultHandlerEnabled: u8_0,
    pub faultActive: u8_0,
    pub faultedThread: *mut OSThread,
    pub padCallback: Option<unsafe extern "C" fn(_: *mut Input) -> ()>,
    pub clients: *mut FaultClient,
    pub addrConvClients: *mut FaultAddrConvClient,
    pub unk_7E0: [u8_0; 4],
    pub padInput: Input,
    pub colors: [u16_0; 36],
    pub fb: *mut libc::c_void,
    pub currClientThreadSp: u32_0,
    pub unk_84C: [u8_0; 4],
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
#[no_mangle]
pub static mut D_8012D280: s32 = 1 as libc::c_int;
#[no_mangle]
pub unsafe extern "C" fn PadMgr_LockSerialMesgQueue(mut padMgr: *mut PadMgr)
 -> *mut OSMesgQueue {
    let mut ctrlrQ: *mut OSMesgQueue = 0 as *mut OSMesgQueue;
    if D_8012D280 > 2 as libc::c_int {
        // "serialMsgQ Waiting for lock"
        osSyncPrintf(b"%2d %d serialMsgQ\xe3\x83\xad\xe3\x83\x83\xe3\x82\xaf\xe5\xbe\x85\xe3\x81\xa1         %08x %08x          %08x\n\x00"
                         as *const u8 as *const libc::c_char,
                     osGetThreadId(0 as *mut OSThread),
                     (*padMgr).serialMsgQ.validCount, padMgr,
                     &mut (*padMgr).serialMsgQ as *mut OSMesgQueue,
                     &mut ctrlrQ as *mut *mut OSMesgQueue);
    }
    osRecvMesg(&mut (*padMgr).serialMsgQ,
               &mut ctrlrQ as *mut *mut OSMesgQueue as OSMesg as *mut OSMesg,
               1 as libc::c_int);
    if D_8012D280 > 2 as libc::c_int {
        // "serialMsgQ Locked"
        osSyncPrintf(b"%2d %d serialMsgQ\xe3\x82\x92\xe3\x83\xad\xe3\x83\x83\xe3\x82\xaf\xe3\x81\x97\xe3\x81\xbe\xe3\x81\x97\xe3\x81\x9f                     %08x\n\x00"
                         as *const u8 as *const libc::c_char,
                     osGetThreadId(0 as *mut OSThread),
                     (*padMgr).serialMsgQ.validCount, ctrlrQ);
    }
    return ctrlrQ;
}
#[no_mangle]
pub unsafe extern "C" fn PadMgr_UnlockSerialMesgQueue(mut padMgr: *mut PadMgr,
                                                      mut ctrlrQ:
                                                          *mut OSMesgQueue) {
    if D_8012D280 > 2 as libc::c_int {
        // "serialMsgQ Unlock"
        osSyncPrintf(b"%2d %d serialMsgQ\xe3\x83\xad\xe3\x83\x83\xe3\x82\xaf\xe8\xa7\xa3\xe9\x99\xa4\xe3\x81\x97\xe3\x81\xbe\xe3\x81\x99   %08x %08x %08x\n\x00"
                         as *const u8 as *const libc::c_char,
                     osGetThreadId(0 as *mut OSThread),
                     (*padMgr).serialMsgQ.validCount, padMgr,
                     &mut (*padMgr).serialMsgQ as *mut OSMesgQueue, ctrlrQ);
    }
    osSendMesg(&mut (*padMgr).serialMsgQ, ctrlrQ as OSMesg, 1 as libc::c_int);
    if D_8012D280 > 2 as libc::c_int {
        // "serialMsgQ Unlocked"
        osSyncPrintf(b"%2d %d serialMsgQ\xe3\x83\xad\xe3\x83\x83\xe3\x82\xaf\xe8\xa7\xa3\xe9\x99\xa4\xe3\x81\x97\xe3\x81\xbe\xe3\x81\x97\xe3\x81\x9f %08x %08x %08x\n\x00"
                         as *const u8 as *const libc::c_char,
                     osGetThreadId(0 as *mut OSThread),
                     (*padMgr).serialMsgQ.validCount, padMgr,
                     &mut (*padMgr).serialMsgQ as *mut OSMesgQueue, ctrlrQ);
    };
}
#[no_mangle]
pub unsafe extern "C" fn PadMgr_LockPadData(mut padMgr: *mut PadMgr) {
    osRecvMesg(&mut (*padMgr).lockMsgQ, 0 as *mut OSMesg, 1 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn PadMgr_UnlockPadData(mut padMgr: *mut PadMgr) {
    osSendMesg(&mut (*padMgr).lockMsgQ, 0 as *mut libc::c_void,
               1 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn PadMgr_RumbleControl(mut padMgr: *mut PadMgr) {
    static mut errcnt: u32_0 = 0 as libc::c_int as u32_0;
    static mut frame: u32_0 = 0;
    let mut temp: s32 = 1 as libc::c_int;
    let mut triedRumbleComm: s32 = 0;
    let mut ctrlrQ: *mut OSMesgQueue = PadMgr_LockSerialMesgQueue(padMgr);
    let mut var4: s32 = 0;
    let mut i: s32 = 0;
    triedRumbleComm = 0 as libc::c_int;
    i = 0 as libc::c_int;
    while i < 4 as libc::c_int {
        if (*padMgr).ctrlrIsConnected[i as usize] != 0 {
            if (*padMgr).padStatus[i as usize].status as libc::c_int &
                   1 as libc::c_int != 0 {
                if (*padMgr).pakType[i as usize] as libc::c_int == temp {
                    if (*padMgr).rumbleEnable[i as usize] as libc::c_int !=
                           0 as libc::c_int {
                        if ((*padMgr).rumbleCounter[i as usize] as
                                libc::c_int) < 3 as libc::c_int {
                            // clang-format off
                            osSyncPrintf(b"\x1b[33m\x00" as *const u8 as
                                             *const libc::c_char);
                            // clang-format on
                            // "Vibration pack jumble jumble"?
                            osSyncPrintf(b"padmgr: %d\xe3\x82\xb3\xe3\x83\xb3: %s\n\x00"
                                             as *const u8 as
                                             *const libc::c_char,
                                         i + 1 as libc::c_int,
                                         b"\xe6\x8c\xaf\xe5\x8b\x95\xe3\x83\x91\xe3\x83\x83\xe3\x82\xaf \xe3\x81\xb6\xe3\x82\x8b\xe3\x81\xb6\xe3\x82\x8b\xe3\x81\xb6\xe3\x82\x8b\xe3\x81\xb6\xe3\x82\x8b\x00"
                                             as *const u8 as
                                             *const libc::c_char);
                            osSyncPrintf(b"\x1b[m\x00" as *const u8 as
                                             *const libc::c_char);
                            if __osMotorAccess(&mut *(*padMgr).pfs.as_mut_ptr().offset(i
                                                                                           as
                                                                                           isize),
                                               temp as u32_0) !=
                                   0 as libc::c_int {
                                (*padMgr).pakType[i as usize] =
                                    0 as libc::c_int as u8_0;
                                osSyncPrintf(b"\x1b[33m\x00" as *const u8 as
                                                 *const libc::c_char);
                                // "A communication error has occurred with the vibration pack"
                                osSyncPrintf(b"padmgr: %d\xe3\x82\xb3\xe3\x83\xb3: %s\n\x00"
                                                 as *const u8 as
                                                 *const libc::c_char,
                                             i + 1 as libc::c_int,
                                             b"\xe6\x8c\xaf\xe5\x8b\x95\xe3\x83\x91\xe3\x83\x83\xe3\x82\xaf\xe3\x81\xa7\xe9\x80\x9a\xe4\xbf\xa1\xe3\x82\xa8\xe3\x83\xa9\xe3\x83\xbc\xe3\x81\x8c\xe7\x99\xba\xe7\x94\x9f\xe3\x81\x97\xe3\x81\xbe\xe3\x81\x97\xe3\x81\x9f\x00"
                                                 as *const u8 as
                                                 *const libc::c_char);
                                osSyncPrintf(b"\x1b[m\x00" as *const u8 as
                                                 *const libc::c_char);
                            } else {
                                (*padMgr).rumbleCounter[i as usize] =
                                    3 as libc::c_int as u8_0
                            }
                            triedRumbleComm = 1 as libc::c_int
                        }
                    } else if (*padMgr).rumbleCounter[i as usize] as
                                  libc::c_int != 0 as libc::c_int {
                        // clang-format off
                        osSyncPrintf(b"\x1b[33m\x00" as *const u8 as
                                         *const libc::c_char);
                        // clang-format on
                        // "Stop vibration pack"
                        osSyncPrintf(b"padmgr: %d\xe3\x82\xb3\xe3\x83\xb3: %s\n\x00"
                                         as *const u8 as *const libc::c_char,
                                     i + 1 as libc::c_int,
                                     b"\xe6\x8c\xaf\xe5\x8b\x95\xe3\x83\x91\xe3\x83\x83\xe3\x82\xaf \xe5\x81\x9c\xe6\xad\xa2\x00"
                                         as *const u8 as *const libc::c_char);
                        osSyncPrintf(b"\x1b[m\x00" as *const u8 as
                                         *const libc::c_char);
                        if __osMotorAccess(&mut *(*padMgr).pfs.as_mut_ptr().offset(i
                                                                                       as
                                                                                       isize),
                                           0 as libc::c_int as u32_0) !=
                               0 as libc::c_int {
                            (*padMgr).pakType[i as usize] =
                                0 as libc::c_int as u8_0;
                            osSyncPrintf(b"\x1b[33m\x00" as *const u8 as
                                             *const libc::c_char);
                            // "A communication error has occurred with the vibration pack"
                            osSyncPrintf(b"padmgr: %d\xe3\x82\xb3\xe3\x83\xb3: %s\n\x00"
                                             as *const u8 as
                                             *const libc::c_char,
                                         i + 1 as libc::c_int,
                                         b"\xe6\x8c\xaf\xe5\x8b\x95\xe3\x83\x91\xe3\x83\x83\xe3\x82\xaf\xe3\x81\xa7\xe9\x80\x9a\xe4\xbf\xa1\xe3\x82\xa8\xe3\x83\xa9\xe3\x83\xbc\xe3\x81\x8c\xe7\x99\xba\xe7\x94\x9f\xe3\x81\x97\xe3\x81\xbe\xe3\x81\x97\xe3\x81\x9f\x00"
                                             as *const u8 as
                                             *const libc::c_char);
                            osSyncPrintf(b"\x1b[m\x00" as *const u8 as
                                             *const libc::c_char);
                        } else {
                            (*padMgr).rumbleCounter[i as usize] =
                                (*padMgr).rumbleCounter[i as
                                                            usize].wrapping_sub(1)
                        }
                        triedRumbleComm = 1 as libc::c_int
                    }
                }
            } else if (*padMgr).pakType[i as usize] as libc::c_int !=
                          0 as libc::c_int {
                if (*padMgr).pakType[i as usize] as libc::c_int ==
                       1 as libc::c_int {
                    osSyncPrintf(b"\x1b[33m\x00" as *const u8 as
                                     *const libc::c_char);
                    // "It seems that a vibration pack was pulled out"
                    osSyncPrintf(b"padmgr: %d\xe3\x82\xb3\xe3\x83\xb3: %s\n\x00"
                                     as *const u8 as *const libc::c_char,
                                 i + 1 as libc::c_int,
                                 b"\xe6\x8c\xaf\xe5\x8b\x95\xe3\x83\x91\xe3\x83\x83\xe3\x82\xaf\xe3\x81\x8c\xe6\x8a\x9c\xe3\x81\x8b\xe3\x82\x8c\xe3\x81\x9f\xe3\x82\x88\xe3\x81\x86\xe3\x81\xa7\xe3\x81\x99\x00"
                                     as *const u8 as *const libc::c_char);
                    osSyncPrintf(b"\x1b[m\x00" as *const u8 as
                                     *const libc::c_char);
                    (*padMgr).pakType[i as usize] = 0 as libc::c_int as u8_0
                } else {
                    osSyncPrintf(b"\x1b[33m\x00" as *const u8 as
                                     *const libc::c_char);
                    // "It seems that a controller pack that is not a vibration pack was pulled out"
                    osSyncPrintf(b"padmgr: %d\xe3\x82\xb3\xe3\x83\xb3: %s\n\x00"
                                     as *const u8 as *const libc::c_char,
                                 i + 1 as libc::c_int,
                                 b"\xe6\x8c\xaf\xe5\x8b\x95\xe3\x83\x91\xe3\x83\x83\xe3\x82\xaf\xe3\x81\xa7\xe3\x81\xaf\xe3\x81\xaa\xe3\x81\x84\xe3\x82\xb3\xe3\x83\xb3\xe3\x83\x88\xe3\x83\xad\xe3\x83\xbc\xe3\x83\xa9\xe3\x83\x91\xe3\x83\x83\xe3\x82\xaf\xe3\x81\x8c\xe6\x8a\x9c\xe3\x81\x8b\xe3\x82\x8c\xe3\x81\x9f\xe3\x82\x88\xe3\x81\x86\xe3\x81\xa7\xe3\x81\x99\x00"
                                     as *const u8 as *const libc::c_char);
                    osSyncPrintf(b"\x1b[m\x00" as *const u8 as
                                     *const libc::c_char);
                    (*padMgr).pakType[i as usize] = 0 as libc::c_int as u8_0
                }
            }
        }
        i += 1
    }
    if triedRumbleComm == 0 {
        i = frame.wrapping_rem(4 as libc::c_int as libc::c_uint) as s32;
        if (*padMgr).ctrlrIsConnected[i as usize] as libc::c_int != 0 &&
               (*padMgr).padStatus[i as usize].status as libc::c_int &
                   1 as libc::c_int != 0 &&
               (*padMgr).pakType[i as usize] as libc::c_int !=
                   1 as libc::c_int {
            var4 =
                osMotorInit(ctrlrQ,
                            &mut *(*padMgr).pfs.as_mut_ptr().offset(i as
                                                                        isize),
                            i);
            if var4 == 0 as libc::c_int {
                (*padMgr).pakType[i as usize] = 1 as libc::c_int as u8_0;
                __osMotorAccess(&mut *(*padMgr).pfs.as_mut_ptr().offset(i as
                                                                            isize),
                                1 as libc::c_int as u32_0);
                __osMotorAccess(&mut *(*padMgr).pfs.as_mut_ptr().offset(i as
                                                                            isize),
                                0 as libc::c_int as u32_0);
                osSyncPrintf(b"\x1b[33m\x00" as *const u8 as
                                 *const libc::c_char);
                // "Recognized vibration pack"
                osSyncPrintf(b"padmgr: %d\xe3\x82\xb3\xe3\x83\xb3: %s\n\x00"
                                 as *const u8 as *const libc::c_char,
                             i + 1 as libc::c_int,
                             b"\xe6\x8c\xaf\xe5\x8b\x95\xe3\x83\x91\xe3\x83\x83\xe3\x82\xaf\xe3\x82\x92\xe8\xaa\x8d\xe8\xad\x98\xe3\x81\x97\xe3\x81\xbe\xe3\x81\x97\xe3\x81\x9f\x00"
                                 as *const u8 as *const libc::c_char);
                osSyncPrintf(b"\x1b[m\x00" as *const u8 as
                                 *const libc::c_char);
            } else if var4 == 11 as libc::c_int {
                (*padMgr).pakType[i as usize] = 2 as libc::c_int as u8_0
            } else if var4 == 4 as libc::c_int {
                LogUtils_LogThreadId(b"../padmgr.c\x00" as *const u8 as
                                         *const libc::c_char,
                                     282 as libc::c_int);
                errcnt = errcnt.wrapping_add(1);
                osSyncPrintf(b"++errcnt = %d\n\x00" as *const u8 as
                                 *const libc::c_char, errcnt);
                osSyncPrintf(b"\x1b[33m\x00" as *const u8 as
                                 *const libc::c_char);
                // "Controller pack communication error"
                osSyncPrintf(b"padmgr: %d\xe3\x82\xb3\xe3\x83\xb3: %s\n\x00"
                                 as *const u8 as *const libc::c_char,
                             i + 1 as libc::c_int,
                             b"\xe3\x82\xb3\xe3\x83\xb3\xe3\x83\x88\xe3\x83\xad\xe3\x83\xbc\xe3\x83\xa9\xe3\x83\x91\xe3\x83\x83\xe3\x82\xaf\xe3\x81\xae\xe9\x80\x9a\xe4\xbf\xa1\xe3\x82\xa8\xe3\x83\xa9\xe3\x83\xbc\x00"
                                 as *const u8 as *const libc::c_char);
                osSyncPrintf(b"\x1b[m\x00" as *const u8 as
                                 *const libc::c_char);
            }
        }
    }
    frame = frame.wrapping_add(1);
    PadMgr_UnlockSerialMesgQueue(padMgr, ctrlrQ);
}
#[no_mangle]
pub unsafe extern "C" fn PadMgr_RumbleStop(mut padMgr: *mut PadMgr) {
    let mut i: s32 = 0;
    let mut ctrlrQ: *mut OSMesgQueue = PadMgr_LockSerialMesgQueue(padMgr);
    i = 0 as libc::c_int;
    while i < 4 as libc::c_int {
        if osMotorInit(ctrlrQ,
                       &mut *(*padMgr).pfs.as_mut_ptr().offset(i as isize), i)
               == 0 as libc::c_int {
            if gFaultStruct.msgId as libc::c_int == 0 as libc::c_int &&
                   (*padMgr).rumbleOnFrames as libc::c_int != 0 as libc::c_int
               {
                osSyncPrintf(b"\x1b[33m\x00" as *const u8 as
                                 *const libc::c_char);
                // "Stop vibration pack"
                osSyncPrintf(b"padmgr: %d\xe3\x82\xb3\xe3\x83\xb3: %s\n\x00"
                                 as *const u8 as *const libc::c_char,
                             i + 1 as libc::c_int,
                             b"\xe6\x8c\xaf\xe5\x8b\x95\xe3\x83\x91\xe3\x83\x83\xe3\x82\xaf \xe5\x81\x9c\xe6\xad\xa2\x00"
                                 as *const u8 as
                                 *const libc::c_char); // original name
                osSyncPrintf(b"\x1b[m\x00" as *const u8 as
                                 *const libc::c_char);
            }
            __osMotorAccess(&mut *(*padMgr).pfs.as_mut_ptr().offset(i as
                                                                        isize),
                            0 as libc::c_int as u32_0);
        }
        i += 1
    }
    PadMgr_UnlockSerialMesgQueue(padMgr, ctrlrQ);
}
#[no_mangle]
pub unsafe extern "C" fn PadMgr_RumbleReset(mut padMgr: *mut PadMgr) {
    ::std::ptr::write_volatile(&mut (*padMgr).rumbleOffFrames as *mut vu8,
                               3 as libc::c_int as u8_0);
}
#[no_mangle]
pub unsafe extern "C" fn PadMgr_RumbleSetSingle(mut padMgr: *mut PadMgr,
                                                mut ctrlr: u32_0,
                                                mut rumble: u32_0) {
    ::std::ptr::write_volatile(&mut (*padMgr).rumbleEnable[ctrlr as usize] as
                                   *mut vu8, rumble as u8_0);
    ::std::ptr::write_volatile(&mut (*padMgr).rumbleOnFrames as *mut vu8,
                               240 as libc::c_int as u8_0);
}
#[no_mangle]
pub unsafe extern "C" fn PadMgr_RumbleSet(mut padMgr: *mut PadMgr,
                                          mut ctrlrRumbles: *mut u8_0) {
    let mut i: s32 = 0;
    i = 0 as libc::c_int;
    while i < 4 as libc::c_int {
        ::std::ptr::write_volatile(&mut (*padMgr).rumbleEnable[i as usize] as
                                       *mut vu8,
                                   *ctrlrRumbles.offset(i as isize));
        i += 1
    }
    ::std::ptr::write_volatile(&mut (*padMgr).rumbleOnFrames as *mut vu8,
                               240 as libc::c_int as u8_0);
}
#[no_mangle]
pub unsafe extern "C" fn PadMgr_ProcessInputs(mut padMgr: *mut PadMgr) {
    let mut i: s32 = 0;
    let mut input: *mut Input = 0 as *mut Input;
    let mut padnow1: *mut OSContPad = 0 as *mut OSContPad;
    let mut buttonDiff: s32 = 0;
    PadMgr_LockPadData(padMgr);
    input =
        &mut *(*padMgr).inputs.as_mut_ptr().offset(0 as libc::c_int as isize)
            as *mut Input;
    padnow1 =
        &mut *(*padMgr).pads.as_mut_ptr().offset(0 as libc::c_int as isize) as
            *mut OSContPad;
    i = 0 as libc::c_int;
    while i < (*padMgr).nControllers as libc::c_int {
        (*input).prev = (*input).cur;
        // Necessary to match
        match (*padnow1).errno as libc::c_int {
            0 => {
                (*input).cur = *padnow1; // "Recognized"
                if (*padMgr).ctrlrIsConnected[i as usize] == 0 {
                    (*padMgr).ctrlrIsConnected[i as usize] =
                        1 as libc::c_int as u8_0;
                    osSyncPrintf(b"\x1b[33m\x00" as *const u8 as
                                     *const libc::c_char);
                    osSyncPrintf(b"padmgr: %d\xe3\x82\xb3\xe3\x83\xb3: %s\n\x00"
                                     as *const u8 as *const libc::c_char,
                                 i + 1 as libc::c_int,
                                 b"\xe8\xaa\x8d\xe8\xad\x98\xe3\x81\x97\xe3\x81\xbe\xe3\x81\x97\xe3\x81\x9f\x00"
                                     as *const u8 as *const libc::c_char);
                    osSyncPrintf(b"\x1b[m\x00" as *const u8 as
                                     *const libc::c_char);
                }
            }
            4 => {
                (*input).cur = (*input).prev;
                LogUtils_LogThreadId(b"../padmgr.c\x00" as *const u8 as
                                         *const libc::c_char,
                                     380 as libc::c_int);
                osSyncPrintf(b"this->Key_switch[i] = %d\n\x00" as *const u8 as
                                 *const libc::c_char,
                             (*padMgr).ctrlrIsConnected[i as usize] as
                                 libc::c_int);
                osSyncPrintf(b"\x1b[33m\x00" as *const u8 as
                                 *const libc::c_char);
                // "Overrun error occurred"
                osSyncPrintf(b"padmgr: %d\xe3\x82\xb3\xe3\x83\xb3: %s\n\x00"
                                 as *const u8 as *const libc::c_char,
                             i + 1 as libc::c_int,
                             b"\xe3\x82\xaa\xe3\x83\xbc\xe3\x83\x90\xe3\x83\xbc\xe3\x83\xa9\xe3\x83\xb3\xe3\x82\xa8\xe3\x83\xa9\xe3\x83\xbc\xe3\x81\x8c\xe7\x99\xba\xe7\x94\x9f\x00"
                                 as *const u8 as *const libc::c_char);
                osSyncPrintf(b"\x1b[m\x00" as *const u8 as
                                 *const libc::c_char);
            }
            8 => {
                (*input).cur.button = 0 as libc::c_int as u16_0;
                (*input).cur.stick_x = 0 as libc::c_int as s8;
                (*input).cur.stick_y = 0 as libc::c_int as s8;
                (*input).cur.errno = (*padnow1).errno;
                if (*padMgr).ctrlrIsConnected[i as usize] != 0 {
                    (*padMgr).ctrlrIsConnected[i as usize] =
                        0 as libc::c_int as u8_0;
                    (*padMgr).pakType[i as usize] = 0 as libc::c_int as u8_0;
                    (*padMgr).rumbleCounter[i as usize] =
                        0xff as libc::c_int as u8_0;
                    osSyncPrintf(b"\x1b[33m\x00" as *const u8 as
                                     *const libc::c_char);
                    // "Do not respond"?
                    osSyncPrintf(b"padmgr: %d\xe3\x82\xb3\xe3\x83\xb3: %s\n\x00"
                                     as *const u8 as *const libc::c_char,
                                 i + 1 as libc::c_int,
                                 b"\xe5\xbf\x9c\xe7\xad\x94\xe3\x81\x97\xe3\x81\xbe\xe3\x81\x9b\xe3\x82\x93\x00"
                                     as *const u8 as *const libc::c_char);
                    osSyncPrintf(b"\x1b[m\x00" as *const u8 as
                                     *const libc::c_char);
                }
            }
            _ => {
                LogUtils_LogThreadId(b"../padmgr.c\x00" as *const u8 as
                                         *const libc::c_char,
                                     396 as libc::c_int);
                osSyncPrintf(b"padnow1->errno = %x\n\x00" as *const u8 as
                                 *const libc::c_char,
                             (*padnow1).errno as libc::c_int);
                Fault_AddHungupAndCrash(b"../padmgr.c\x00" as *const u8 as
                                            *const libc::c_char,
                                        397 as libc::c_int as u32_0);
            }
        }
        buttonDiff =
            (*input).prev.button as libc::c_int ^
                (*input).cur.button as libc::c_int;
        (*input).press.button =
            ((*input).press.button as libc::c_int |
                 (buttonDiff & (*input).cur.button as libc::c_int) as u16_0 as
                     libc::c_int) as u16_0;
        (*input).rel.button =
            ((*input).rel.button as libc::c_int |
                 (buttonDiff & (*input).prev.button as libc::c_int) as u16_0
                     as libc::c_int) as u16_0;
        PadUtils_UpdateRelXY(input);
        (*input).press.stick_x =
            ((*input).press.stick_x as libc::c_int +
                 ((*input).cur.stick_x as libc::c_int -
                      (*input).prev.stick_x as libc::c_int) as s8 as
                     libc::c_int) as s8;
        (*input).press.stick_y =
            ((*input).press.stick_y as libc::c_int +
                 ((*input).cur.stick_y as libc::c_int -
                      (*input).prev.stick_y as libc::c_int) as s8 as
                     libc::c_int) as s8;
        i += 1;
        input = input.offset(1);
        padnow1 = padnow1.offset(1)
    }
    PadMgr_UnlockPadData(padMgr);
}
#[no_mangle]
pub unsafe extern "C" fn PadMgr_HandleRetraceMsg(mut padMgr: *mut PadMgr) {
    let mut i: s32 = 0;
    let mut queue: *mut OSMesgQueue = PadMgr_LockSerialMesgQueue(padMgr);
    let mut mask: u32_0 = 0;
    osContStartReadData(queue);
    if (*padMgr).retraceCallback.is_some() {
        (*padMgr).retraceCallback.expect("non-null function pointer")(padMgr,
                                                                      (*padMgr).retraceCallbackValue
                                                                          as
                                                                          s32);
    }
    osRecvMesg(queue, 0 as *mut OSMesg, 1 as libc::c_int);
    osContGetReadData((*padMgr).pads.as_mut_ptr());
    if (*padMgr).preNMIShutdown != 0 {
        bzero((*padMgr).pads.as_mut_ptr() as *mut libc::c_void,
              ::std::mem::size_of::<[OSContPad; 4]>() as libc::c_ulong);
    }
    PadMgr_ProcessInputs(padMgr);
    osContStartQuery(queue);
    osRecvMesg(queue, 0 as *mut OSMesg, 1 as libc::c_int);
    osContGetQuery((*padMgr).padStatus.as_mut_ptr());
    PadMgr_UnlockSerialMesgQueue(padMgr, queue);
    mask = 0 as libc::c_int as u32_0;
    i = 0 as libc::c_int;
    while i < 4 as libc::c_int {
        if (*padMgr).padStatus[i as usize].errno as libc::c_int ==
               0 as libc::c_int {
            if (*padMgr).padStatus[i as usize].type_0 as libc::c_int ==
                   0x5 as libc::c_int {
                mask |= ((1 as libc::c_int) << i) as libc::c_uint
            } else {
                LogUtils_LogThreadId(b"../padmgr.c\x00" as *const u8 as
                                         *const libc::c_char,
                                     458 as libc::c_int);
                osSyncPrintf(b"this->pad_status[i].type = %x\n\x00" as
                                 *const u8 as *const libc::c_char,
                             (*padMgr).padStatus[i as usize].type_0 as
                                 libc::c_int);
                // "An unknown type of controller is connected"
                osSyncPrintf(b"\xe7\x9f\xa5\xe3\x82\x89\xe3\x81\xaa\xe3\x81\x84\xe7\xa8\xae\xe9\xa1\x9e\xe3\x81\xae\xe3\x82\xb3\xe3\x83\xb3\xe3\x83\x88\xe3\x83\xad\xe3\x83\xbc\xe3\x83\xa9\xe3\x81\x8c\xe6\x8e\xa5\xe7\xb6\x9a\xe3\x81\x95\xe3\x82\x8c\xe3\x81\xa6\xe3\x81\x84\xe3\x81\xbe\xe3\x81\x99\n\x00"
                                 as *const u8 as
                                 *const libc::c_char); // "Controller thread execution start"
            }
        }
        i += 1
    }
    ::std::ptr::write_volatile(&mut (*padMgr).validCtrlrsMask as *mut vu8,
                               mask as u8_0);
    if gFaultStruct.msgId != 0 {
        PadMgr_RumbleStop(padMgr);
    } else if (*padMgr).rumbleOffFrames as libc::c_int > 0 as libc::c_int {
        ::std::ptr::write_volatile(&mut (*padMgr).rumbleOffFrames as *mut vu8,
                                   ::std::ptr::read_volatile::<vu8>(&(*padMgr).rumbleOffFrames
                                                                        as
                                                                        *const vu8).wrapping_sub(1));
        PadMgr_RumbleStop(padMgr);
    } else if (*padMgr).rumbleOnFrames as libc::c_int == 0 as libc::c_int {
        PadMgr_RumbleStop(padMgr);
    } else if (*padMgr).preNMIShutdown == 0 {
        PadMgr_RumbleControl(padMgr);
        ::std::ptr::write_volatile(&mut (*padMgr).rumbleOnFrames as *mut vu8,
                                   ::std::ptr::read_volatile::<vu8>(&(*padMgr).rumbleOnFrames
                                                                        as
                                                                        *const vu8).wrapping_sub(1))
    };
}
#[no_mangle]
pub unsafe extern "C" fn PadMgr_HandlePreNMI(mut padMgr: *mut PadMgr) {
    osSyncPrintf(b"padmgr_HandlePreNMI()\n\x00" as *const u8 as
                     *const libc::c_char);
    (*padMgr).preNMIShutdown = 1 as libc::c_int as u8_0;
    PadMgr_RumbleReset(padMgr);
}
#[no_mangle]
pub unsafe extern "C" fn PadMgr_RequestPadData(mut padMgr: *mut PadMgr,
                                               mut inputs: *mut Input,
                                               mut mode: s32) {
    let mut i: s32 = 0;
    let mut ogInput: *mut Input = 0 as *mut Input;
    let mut newInput: *mut Input = 0 as *mut Input;
    let mut buttonDiff: s32 = 0;
    PadMgr_LockPadData(padMgr);
    ogInput =
        &mut *(*padMgr).inputs.as_mut_ptr().offset(0 as libc::c_int as isize)
            as *mut Input;
    newInput = &mut *inputs.offset(0 as libc::c_int as isize) as *mut Input;
    i = 0 as libc::c_int;
    while i < 4 as libc::c_int {
        if mode != 0 as libc::c_int {
            *newInput = *ogInput;
            (*ogInput).press.button = 0 as libc::c_int as u16_0;
            (*ogInput).press.stick_x = 0 as libc::c_int as s8;
            (*ogInput).press.stick_y = 0 as libc::c_int as s8;
            (*ogInput).rel.button = 0 as libc::c_int as u16_0
        } else {
            (*newInput).prev = (*newInput).cur;
            (*newInput).cur = (*ogInput).cur;
            buttonDiff =
                (*newInput).prev.button as libc::c_int ^
                    (*newInput).cur.button as libc::c_int;
            (*newInput).press.button =
                ((*newInput).cur.button as libc::c_int & buttonDiff) as u16_0;
            (*newInput).rel.button =
                ((*newInput).prev.button as libc::c_int & buttonDiff) as
                    u16_0;
            PadUtils_UpdateRelXY(newInput);
            (*newInput).press.stick_x =
                ((*newInput).press.stick_x as libc::c_int +
                     ((*newInput).cur.stick_x as libc::c_int -
                          (*newInput).prev.stick_x as libc::c_int) as s8 as
                         libc::c_int) as s8;
            (*newInput).press.stick_y =
                ((*newInput).press.stick_y as libc::c_int +
                     ((*newInput).cur.stick_y as libc::c_int -
                          (*newInput).prev.stick_y as libc::c_int) as s8 as
                         libc::c_int) as s8
        }
        ogInput = ogInput.offset(1);
        newInput = newInput.offset(1);
        i += 1
    }
    PadMgr_UnlockPadData(padMgr);
}
#[no_mangle]
pub unsafe extern "C" fn PadMgr_ThreadEntry(mut padMgr: *mut PadMgr) {
    let mut mesg: *mut s16 = 0 as *mut s16;
    let mut exit: s32 = 0;
    osSyncPrintf(b"\xe3\x82\xb3\xe3\x83\xb3\xe3\x83\x88\xe3\x83\xad\xe3\x83\xbc\xe3\x83\xa9\xe3\x82\xb9\xe3\x83\xac\xe3\x83\x83\xe3\x83\x89\xe5\xae\x9f\xe8\xa1\x8c\xe9\x96\x8b\xe5\xa7\x8b\n\x00"
                     as *const u8 as *const libc::c_char);
    exit = 0 as libc::c_int;
    while exit == 0 {
        if D_8012D280 > 2 as libc::c_int &&
               (*padMgr).interruptMsgQ.validCount == 0 as libc::c_int {
            // "Waiting for controller thread event"
            osSyncPrintf(b"\xe3\x82\xb3\xe3\x83\xb3\xe3\x83\x88\xe3\x83\xad\xe3\x83\xbc\xe3\x83\xa9\xe3\x82\xb9\xe3\x83\xac\xe3\x83\x83\xe3\x83\x89\xe3\x82\xa4\xe3\x83\x99\xe3\x83\xb3\xe3\x83\x88\xe5\xbe\x85\xe3\x81\xa1 %lld\n\x00"
                             as *const u8 as *const libc::c_char,
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
                                                                                          libc::c_ulonglong));
        }
        osRecvMesg(&mut (*padMgr).interruptMsgQ,
                   &mut mesg as *mut *mut s16 as OSMesg as *mut OSMesg,
                   1 as libc::c_int);
        LogUtils_CheckNullPointer(b"msg\x00" as *const u8 as
                                      *const libc::c_char,
                                  mesg as *mut libc::c_void,
                                  b"../padmgr.c\x00" as *const u8 as
                                      *const libc::c_char,
                                  563 as libc::c_int);
        match *mesg as libc::c_int {
            1 => {
                if D_8012D280 > 2 as libc::c_int {
                    osSyncPrintf(b"padmgr_HandleRetraceMsg START %lld\n\x00"
                                     as *const u8 as *const libc::c_char,
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
                                                                                                  libc::c_ulonglong));
                }
                PadMgr_HandleRetraceMsg(padMgr);
                if D_8012D280 > 2 as libc::c_int {
                    osSyncPrintf(b"padmgr_HandleRetraceMsg END   %lld\n\x00"
                                     as *const u8 as *const libc::c_char,
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
                                                                                                  libc::c_ulonglong));
                }
            }
            4 => { PadMgr_HandlePreNMI(padMgr); }
            3 => { exit = 1 as libc::c_int }
            _ => { }
        }
    }
    IrqMgr_RemoveClient((*padMgr).irqMgr, &mut (*padMgr).irqClient);
    osSyncPrintf(b"\xe3\x82\xb3\xe3\x83\xb3\xe3\x83\x88\xe3\x83\xad\xe3\x83\xbc\xe3\x83\xa9\xe3\x82\xb9\xe3\x83\xac\xe3\x83\x83\xe3\x83\x89\xe5\xae\x9f\xe8\xa1\x8c\xe7\xb5\x82\xe4\xba\x86\n\x00"
                     as *const u8 as *const libc::c_char);
    // "Controller thread execution end"
}
#[no_mangle]
pub unsafe extern "C" fn PadMgr_Init(mut padMgr: *mut PadMgr,
                                     mut siIntMsgQ: *mut OSMesgQueue,
                                     mut irqMgr: *mut IrqMgr, mut id: OSId,
                                     mut priority: OSPri,
                                     mut stack: *mut libc::c_void) {
    osSyncPrintf(b"\xe3\x83\x91\xe3\x83\x83\xe3\x83\x89\xe3\x83\x9e\xe3\x83\x8d\xe3\x83\xbc\xe3\x82\xb8\xe3\x83\xa3\xe4\xbd\x9c\xe6\x88\x90 padmgr_Create()\n\x00"
                     as *const u8 as
                     *const libc::c_char); // "Pad Manager creation"
    bzero(padMgr as *mut libc::c_void,
          ::std::mem::size_of::<PadMgr>() as libc::c_ulong);
    (*padMgr).irqMgr = irqMgr;
    osCreateMesgQueue(&mut (*padMgr).interruptMsgQ,
                      (*padMgr).interruptMsgBuf.as_mut_ptr(),
                      4 as libc::c_int);
    IrqMgr_AddClient((*padMgr).irqMgr, &mut (*padMgr).irqClient,
                     &mut (*padMgr).interruptMsgQ);
    osCreateMesgQueue(&mut (*padMgr).serialMsgQ,
                      (*padMgr).serialMsgBuf.as_mut_ptr(), 1 as libc::c_int);
    PadMgr_UnlockSerialMesgQueue(padMgr, siIntMsgQ);
    osCreateMesgQueue(&mut (*padMgr).lockMsgQ,
                      (*padMgr).lockMsgBuf.as_mut_ptr(), 1 as libc::c_int);
    PadMgr_UnlockPadData(padMgr);
    PadSetup_Init(siIntMsgQ,
                  &mut (*padMgr).validCtrlrsMask as *mut vu8 as *mut u8_0,
                  (*padMgr).padStatus.as_mut_ptr());
    (*padMgr).nControllers = 4 as libc::c_int as u8_0;
    osContSetCh((*padMgr).nControllers);
    osCreateThread(&mut (*padMgr).thread, id,
                   ::std::mem::transmute::<Option<unsafe extern "C" fn(_:
                                                                           *mut PadMgr)
                                                      -> ()>,
                                           Option<unsafe extern "C" fn(_:
                                                                           *mut libc::c_void)
                                                      ->
                                                          ()>>(Some(PadMgr_ThreadEntry
                                                                        as
                                                                        unsafe extern "C" fn(_:
                                                                                                 *mut PadMgr)
                                                                            ->
                                                                                ())),
                   padMgr as *mut libc::c_void, stack, priority);
    osStartThread(&mut (*padMgr).thread);
}
