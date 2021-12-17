#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case,
         non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(const_raw_ptr_to_usize_cast, register_tool)]
extern "C" {
    #[no_mangle]
    fn osSyncPrintf(fmt: *const libc::c_char, _: ...);
    #[no_mangle]
    fn Sleep_Msec(ms: u32_0);
    #[no_mangle]
    static mut gPadMgr: PadMgr;
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
pub static mut gIsCtrlr2Valid: u32_0 = 0 as libc::c_int as u32_0;
#[no_mangle]
pub unsafe extern "C" fn func_800D31A0() {
    osSyncPrintf(b"\x1b[31m\n**** Freeze!! ****\n\x1b[m\x00" as *const u8 as
                     *const libc::c_char);
    loop  { Sleep_Msec(1000 as libc::c_int as u32_0); };
}
#[no_mangle]
pub unsafe extern "C" fn func_800D31F0() {
    gIsCtrlr2Valid =
        (gPadMgr.validCtrlrsMask as libc::c_int & 2 as libc::c_int !=
             0 as libc::c_int) as libc::c_int as u32_0;
}
#[no_mangle]
pub unsafe extern "C" fn func_800D3210() {
    gIsCtrlr2Valid = 0 as libc::c_int as u32_0;
}
