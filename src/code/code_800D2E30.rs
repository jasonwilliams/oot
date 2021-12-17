#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case,
         non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(const_raw_ptr_to_usize_cast, register_tool)]
extern "C" {
    #[no_mangle]
    fn bzero(__s: *mut libc::c_void, __n: u32_0);
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct UnkRumbleStruct {
    pub rumbleEnable: [u8_0; 4],
    pub unk_04: [u8_0; 64],
    pub unk_44: [u8_0; 64],
    pub unk_84: [u8_0; 64],
    pub unk_C4: [u8_0; 64],
    pub unk_104: u8_0,
    pub unk_105: u8_0,
    pub unk_106: u16_0,
    pub unk_108: u16_0,
    pub unk_10A: u8_0,
    pub unk_10B: u8_0,
    pub unk_10C: u8_0,
    pub unk_10D: u8_0,
}
#[no_mangle]
pub unsafe extern "C" fn func_800D2E30(mut arg0: *mut UnkRumbleStruct) {
    static mut D_8012DBB0: u8_0 = 1 as libc::c_int as u8_0;
    let mut i: s32 = 0;
    let mut unk_a3: s32 = 0;
    let mut index: s32 = -(1 as libc::c_int);
    i = 0 as libc::c_int;
    while i < 4 as libc::c_int {
        (*arg0).rumbleEnable[i as usize] = 0 as libc::c_int as u8_0;
        i += 1
    }
    if (*arg0).unk_105 as libc::c_int == 0 as libc::c_int {
        if D_8012DBB0 as libc::c_int != 0 as libc::c_int {
            i = 0 as libc::c_int;
            while i < 4 as libc::c_int {
                gPadMgr.pakType[i as usize] = 0 as libc::c_int as u8_0;
                i += 1
            }
        }
        D_8012DBB0 = (*arg0).unk_105;
        return
    }
    D_8012DBB0 = (*arg0).unk_105;
    if (*arg0).unk_104 as libc::c_int == 2 as libc::c_int {
        i = 0 as libc::c_int;
        while i < 4 as libc::c_int {
            gPadMgr.pakType[i as usize] = 0 as libc::c_int as u8_0;
            i += 1
        }
        i = 0 as libc::c_int;
        while i < 0x40 as libc::c_int {
            (*arg0).unk_C4[i as usize] = 0 as libc::c_int as u8_0;
            (*arg0).unk_84[i as usize] = 0 as libc::c_int as u8_0;
            (*arg0).unk_44[i as usize] = 0 as libc::c_int as u8_0;
            (*arg0).unk_04[i as usize] = 0 as libc::c_int as u8_0;
            i += 1
        }
        (*arg0).unk_10D = 0 as libc::c_int as u8_0;
        (*arg0).unk_10C = (*arg0).unk_10D;
        (*arg0).unk_10B = (*arg0).unk_10C;
        (*arg0).unk_10A = (*arg0).unk_10B;
        (*arg0).unk_108 = (*arg0).unk_10A as u16_0;
        (*arg0).unk_106 = (*arg0).unk_108;
        (*arg0).unk_104 = 1 as libc::c_int as u8_0
    }
    if (*arg0).unk_104 as libc::c_int != 0 as libc::c_int {
        i = 0 as libc::c_int;
        while i < 0x40 as libc::c_int {
            if (*arg0).unk_04[i as usize] as libc::c_int != 0 as libc::c_int {
                if (*arg0).unk_44[i as usize] as libc::c_int >
                       0 as libc::c_int {
                    (*arg0).unk_44[i as usize] =
                        (*arg0).unk_44[i as usize].wrapping_sub(1)
                } else {
                    unk_a3 =
                        (*arg0).unk_04[i as usize] as libc::c_int -
                            (*arg0).unk_84[i as usize] as libc::c_int;
                    if unk_a3 > 0 as libc::c_int {
                        (*arg0).unk_04[i as usize] = unk_a3 as u8_0
                    } else {
                        (*arg0).unk_04[i as usize] = 0 as libc::c_int as u8_0
                    }
                }
                unk_a3 =
                    (*arg0).unk_C4[i as usize] as libc::c_int +
                        (*arg0).unk_04[i as usize] as libc::c_int;
                (*arg0).unk_C4[i as usize] = unk_a3 as u8_0;
                if index == -(1 as libc::c_int) {
                    index = i;
                    (*arg0).rumbleEnable[0 as libc::c_int as usize] =
                        (unk_a3 >= 0x100 as libc::c_int) as libc::c_int as
                            u8_0
                } else if ((*arg0).unk_04[index as usize] as libc::c_int) <
                              (*arg0).unk_04[i as usize] as libc::c_int {
                    index = i;
                    (*arg0).rumbleEnable[0 as libc::c_int as usize] =
                        (unk_a3 >= 0x100 as libc::c_int) as libc::c_int as
                            u8_0
                }
            }
            i += 1
        }
        if (*arg0).unk_10A as libc::c_int != 0 as libc::c_int {
            if (*arg0).unk_10B as libc::c_int > 0 as libc::c_int {
                (*arg0).unk_10B = (*arg0).unk_10B.wrapping_sub(1)
            } else {
                unk_a3 =
                    (*arg0).unk_10A as libc::c_int -
                        (*arg0).unk_10C as libc::c_int;
                if unk_a3 > 0 as libc::c_int {
                    (*arg0).unk_10A = unk_a3 as u8_0
                } else { (*arg0).unk_10A = 0 as libc::c_int as u8_0 }
            }
            unk_a3 =
                (*arg0).unk_10D as libc::c_int +
                    (*arg0).unk_10A as libc::c_int;
            (*arg0).unk_10D = unk_a3 as u8_0;
            (*arg0).rumbleEnable[0 as libc::c_int as usize] =
                (unk_a3 >= 0x100 as libc::c_int) as libc::c_int as u8_0
        }
        if (*arg0).unk_10A as libc::c_int != 0 as libc::c_int {
            unk_a3 = (*arg0).unk_10A as s32
        } else if index == -(1 as libc::c_int) {
            unk_a3 = 0 as libc::c_int
        } else { unk_a3 = (*arg0).unk_04[index as usize] as s32 }
        if unk_a3 == 0 as libc::c_int {
            (*arg0).unk_108 = (*arg0).unk_108.wrapping_add(1);
            if (*arg0).unk_108 as libc::c_int >= 6 as libc::c_int {
                (*arg0).unk_106 = 0 as libc::c_int as u16_0;
                (*arg0).unk_108 = 5 as libc::c_int as u16_0
            }
        } else {
            (*arg0).unk_108 = 0 as libc::c_int as u16_0;
            (*arg0).unk_106 = (*arg0).unk_106.wrapping_add(1);
            if (*arg0).unk_106 as libc::c_int >= 0x1c21 as libc::c_int {
                (*arg0).unk_104 = 0 as libc::c_int as u8_0
            }
        }
    } else {
        i = 0 as libc::c_int;
        while i < 0x40 as libc::c_int {
            (*arg0).unk_C4[i as usize] = 0 as libc::c_int as u8_0;
            (*arg0).unk_84[i as usize] = 0 as libc::c_int as u8_0;
            (*arg0).unk_44[i as usize] = 0 as libc::c_int as u8_0;
            (*arg0).unk_04[i as usize] = 0 as libc::c_int as u8_0;
            i += 1
        }
        (*arg0).unk_10D = 0 as libc::c_int as u8_0;
        (*arg0).unk_10C = (*arg0).unk_10D;
        (*arg0).unk_10B = (*arg0).unk_10C;
        (*arg0).unk_10A = (*arg0).unk_10B;
        (*arg0).unk_108 = (*arg0).unk_10A as u16_0;
        (*arg0).unk_106 = (*arg0).unk_108
    };
}
#[no_mangle]
pub unsafe extern "C" fn func_800D3140(mut arg0: *mut UnkRumbleStruct) {
    bzero(arg0 as *mut libc::c_void,
          ::std::mem::size_of::<UnkRumbleStruct>() as libc::c_ulong);
    (*arg0).unk_104 = 2 as libc::c_int as u8_0;
    (*arg0).unk_105 = 1 as libc::c_int as u8_0;
}
#[no_mangle]
pub unsafe extern "C" fn func_800D3178(mut arg0: *mut UnkRumbleStruct) {
    bzero(arg0 as *mut libc::c_void,
          ::std::mem::size_of::<UnkRumbleStruct>() as libc::c_ulong);
}
