#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case,
         non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(const_raw_ptr_to_usize_cast, register_tool)]
extern "C" {
    #[no_mangle]
    fn osSyncPrintf(fmt: *const libc::c_char, _: ...);
    #[no_mangle]
    fn LogUtils_HungupThread(name: *const libc::c_char, line: s32);
    #[no_mangle]
    fn osEPiReadIo(handle: *mut OSPiHandle, devAddr: u32_0, data: *mut u32_0)
     -> s32;
    #[no_mangle]
    static mut gPadMgr: PadMgr;
    #[no_mangle]
    static mut gCartHandle: *mut OSPiHandle;
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
pub struct LocaleCartInfo {
    pub name: [libc::c_char; 24],
    pub mediaFormat: u32_0,
    pub c2rust_unnamed: C2RustUnnamed_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_0 {
    pub c2rust_unnamed: C2RustUnnamed_1,
    pub regionInfo: u32_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_1 {
    pub cartId: u16_0,
    pub countryCode: u8_0,
    pub version: u8_0,
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
pub static mut gCurrentRegion: u32_0 = 0 as libc::c_int as u32_0;
#[no_mangle]
pub static mut sCartInfo: LocaleCartInfo =
    LocaleCartInfo{name: [0; 24],
                   mediaFormat: 0,
                   c2rust_unnamed:
                       C2RustUnnamed_0{c2rust_unnamed:
                                           C2RustUnnamed_1{cartId: 0,
                                                           countryCode: 0,
                                                           version: 0,},},};
#[no_mangle]
pub unsafe extern "C" fn Locale_Init() {
    osEPiReadIo(gCartHandle, 0x38 as libc::c_int as u32_0,
                &mut sCartInfo.mediaFormat);
    osEPiReadIo(gCartHandle, 0x3c as libc::c_int as u32_0,
                &mut sCartInfo.c2rust_unnamed.regionInfo);
    match sCartInfo.c2rust_unnamed.c2rust_unnamed.countryCode as libc::c_int {
        74 => {
            // "NTSC-U (North America)"
            gCurrentRegion = 1 as libc::c_int as u32_0
        }
        69 => {
            // "NTSC-J (Japan)"
            gCurrentRegion = 2 as libc::c_int as u32_0
        }
        80 => {
            // "PAL (Europe)"
            gCurrentRegion = 3 as libc::c_int as u32_0
        }
        _ => {
            osSyncPrintf(b"\x1b[41;37m\x00" as *const u8 as
                             *const libc::c_char);
            osSyncPrintf(b"z_locale_init: \xe6\x97\xa5\xe6\x9c\xac\xe7\x94\xa8\xe3\x81\x8b\xe3\x82\xa2\xe3\x83\xa1\xe3\x83\xaa\xe3\x82\xab\xe7\x94\xa8\xe3\x81\x8b\xe5\x88\xa4\xe5\x88\xa5\xe3\x81\xa7\xe3\x81\x8d\xe3\x81\xbe\xe3\x81\x9b\xe3\x82\x93\n\x00"
                             as *const u8 as *const libc::c_char);
            LogUtils_HungupThread(b"../z_locale.c\x00" as *const u8 as
                                      *const libc::c_char,
                                  118 as libc::c_int);
            osSyncPrintf(b"\x1b[m\x00" as *const u8 as *const libc::c_char);
        }
    }
    osSyncPrintf(b"z_locale_init:\xe6\x97\xa5\xe6\x9c\xac\xe7\x94\xa8\xe3\x81\x8b\xe3\x82\xa2\xe3\x83\xa1\xe3\x83\xaa\xe3\x82\xab\xe7\x94\xa8\xe3\x81\x8b\xef\xbc\x93\xe3\x82\xb3\xe3\x83\xb3\xe3\x81\xa7\xe5\x88\xa4\xe6\x96\xad\xe3\x81\x95\xe3\x81\x9b\xe3\x82\x8b\n\x00"
                     as *const u8 as *const libc::c_char);
}
#[no_mangle]
pub unsafe extern "C" fn Locale_ResetRegion() {
    gCurrentRegion = 0 as libc::c_int as u32_0;
}
#[no_mangle]
pub unsafe extern "C" fn func_80001F48() -> u32_0 {
    if gCurrentRegion == 3 as libc::c_int as libc::c_uint {
        return 0 as libc::c_int as u32_0
    }
    if gPadMgr.validCtrlrsMask as libc::c_int & 4 as libc::c_int != 0 {
        return 0 as libc::c_int as u32_0
    }
    return 1 as libc::c_int as u32_0;
}
#[no_mangle]
pub unsafe extern "C" fn func_80001F8C() -> u32_0 {
    if gCurrentRegion == 3 as libc::c_int as libc::c_uint {
        return 0 as libc::c_int as u32_0
    }
    if gPadMgr.validCtrlrsMask as libc::c_int & 4 as libc::c_int != 0 {
        return 1 as libc::c_int as u32_0
    }
    return 0 as libc::c_int as u32_0;
}
// This function appears to be unused?
#[no_mangle]
pub unsafe extern "C" fn Locale_IsRegionNative() -> u32_0 {
    return (gCurrentRegion == 3 as libc::c_int as libc::c_uint) as libc::c_int
               as u32_0;
}
