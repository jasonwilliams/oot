#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case,
         non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(register_tool)]
extern "C" {
    #[no_mangle]
    fn osRecvMesg(mq: *mut OSMesgQueue, msg: *mut OSMesg, flag: s32) -> s32;
    #[no_mangle]
    fn osContGetQuery(data: *mut OSContStatus);
    #[no_mangle]
    fn osContStartQuery(mq: *mut OSMesgQueue) -> s32;
    #[no_mangle]
    fn osContInit(mq: *mut OSMesgQueue, ctl_present_bitfield: *mut u8_0,
                  status: *mut OSContStatus) -> s32;
}
pub type u8_0 = libc::c_uchar;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct OSContStatus {
    pub type_0: u16_0,
    pub status: u8_0,
    pub errno: u8_0,
}
#[no_mangle]
pub unsafe extern "C" fn PadSetup_Init(mut mq: *mut OSMesgQueue,
                                       mut outMask: *mut u8_0,
                                       mut status: *mut OSContStatus) -> s32 {
    let mut ret: s32 = 0;
    let mut i: s32 = 0;
    *outMask = 0xff as libc::c_int as u8_0;
    ret = osContInit(mq, outMask, status);
    if ret != 0 as libc::c_int { return ret }
    if *outMask as libc::c_int == 0xff as libc::c_int {
        if osContStartQuery(mq) != 0 as libc::c_int {
            return 1 as libc::c_int
        }
        osRecvMesg(mq, 0 as *mut OSMesg, 1 as libc::c_int);
        osContGetQuery(status);
        *outMask = 0 as libc::c_int as u8_0;
        i = 0 as libc::c_int;
        while i < 4 as libc::c_int {
            match (*status.offset(i as isize)).errno as libc::c_int {
                0 => {
                    if (*status.offset(i as isize)).type_0 as libc::c_int ==
                           0x5 as libc::c_int {
                        *outMask =
                            (*outMask as libc::c_int |
                                 (1 as libc::c_int) << i) as u8_0
                    }
                }
                _ => { }
            }
            i += 1
        }
    }
    return 0 as libc::c_int;
}
