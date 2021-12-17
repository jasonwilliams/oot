#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case,
         non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(register_tool)]
extern "C" {
    #[no_mangle]
    fn osRecvMesg(mq: *mut OSMesgQueue, msg: *mut OSMesg, flag: s32) -> s32;
    #[no_mangle]
    fn osCreateMesgQueue(mq: *mut OSMesgQueue, msg: *mut OSMesg, count: s32);
    #[no_mangle]
    fn osSetTimer(timer: *mut OSTimer, countdown: OSTime, interval: OSTime,
                  mq: *mut OSMesgQueue, msg: OSMesg) -> s32;
}
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
pub struct OSTimer {
    pub next: *mut OSTimer,
    pub prev: *mut OSTimer,
    pub interval: OSTime,
    pub value: OSTime,
    pub mq: *mut OSMesgQueue,
    pub msg: OSMesg,
}
#[no_mangle]
pub unsafe extern "C" fn Sleep_Cycles(mut cycles: OSTime) {
    let mut mq: OSMesgQueue =
        OSMesgQueue{mtqueue: 0 as *mut OSThread,
                    fullqueue: 0 as *mut OSThread,
                    validCount: 0,
                    first: 0,
                    msgCount: 0,
                    msg: 0 as *mut OSMesg,};
    let mut msg: OSMesg = 0 as *mut libc::c_void;
    let mut timer: OSTimer =
        OSTimer{next: 0 as *mut OSTimer,
                prev: 0 as *mut OSTimer,
                interval: 0,
                value: 0,
                mq: 0 as *mut OSMesgQueue,
                msg: 0 as *mut libc::c_void,};
    osCreateMesgQueue(&mut mq, &mut msg, 1 as libc::c_int);
    osSetTimer(&mut timer, cycles, 0 as libc::c_int as OSTime, &mut mq,
               0 as *mut libc::c_void);
    osRecvMesg(&mut mq, 0 as *mut OSMesg, 1 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn Sleep_Nsec(mut nsec: u32_0) {
    Sleep_Cycles((nsec as
                      u64_0).wrapping_mul((62500000 as libc::c_longlong *
                                               3 as libc::c_int as
                                                   libc::c_longlong /
                                               4 as libc::c_int as
                                                   libc::c_longlong /
                                               15625000 as libc::c_longlong)
                                              as
                                              libc::c_ulonglong).wrapping_div((1000000000
                                                                                   as
                                                                                   libc::c_longlong
                                                                                   /
                                                                                   15625000
                                                                                       as
                                                                                       libc::c_longlong)
                                                                                  as
                                                                                  libc::c_ulonglong));
}
#[no_mangle]
pub unsafe extern "C" fn Sleep_Usec(mut usec: u32_0) {
    Sleep_Cycles((usec as
                      u64_0).wrapping_mul((62500000 as libc::c_longlong *
                                               3 as libc::c_int as
                                                   libc::c_longlong /
                                               4 as libc::c_int as
                                                   libc::c_longlong /
                                               15625 as libc::c_longlong) as
                                              libc::c_ulonglong).wrapping_div((1000000
                                                                                   as
                                                                                   libc::c_longlong
                                                                                   /
                                                                                   15625
                                                                                       as
                                                                                       libc::c_longlong)
                                                                                  as
                                                                                  libc::c_ulonglong));
}
// originally "msleep"
#[no_mangle]
pub unsafe extern "C" fn Sleep_Msec(mut ms: u32_0) {
    Sleep_Cycles(((ms as libc::c_longlong *
                       (62500000 as libc::c_longlong *
                            3 as libc::c_int as libc::c_longlong /
                            4 as libc::c_int as libc::c_longlong)) as
                      libc::c_ulonglong).wrapping_div(1000 as
                                                          libc::c_ulonglong));
}
#[no_mangle]
pub unsafe extern "C" fn Sleep_Sec(mut sec: u32_0) {
    Sleep_Cycles((sec as libc::c_longlong *
                      (62500000 as libc::c_longlong *
                           3 as libc::c_int as libc::c_longlong /
                           4 as libc::c_int as libc::c_longlong)) as OSTime);
}
