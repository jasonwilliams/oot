#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case,
         non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(const_raw_ptr_to_usize_cast, register_tool)]
extern "C" {
    #[no_mangle]
    fn __osSetTimerIntr(time: OSTime);
    #[no_mangle]
    fn __osInsertTimer(timer: *mut OSTimer) -> OSTime;
    #[no_mangle]
    fn osGetCount() -> u32_0;
    #[no_mangle]
    fn __osDisableInt() -> s32;
    #[no_mangle]
    fn __osRestoreInt(_: s32);
    #[no_mangle]
    static mut __osTimerList: *mut OSTimer;
    #[no_mangle]
    static mut __osTimerCounter: u32_0;
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
pub unsafe extern "C" fn osSetTimer(mut timer: *mut OSTimer,
                                    mut countdown: OSTime,
                                    mut interval: OSTime,
                                    mut mq: *mut OSMesgQueue, mut msg: OSMesg)
 -> s32 {
    let mut time: OSTime = 0; // suppresses set but unused warning
    let mut next: *mut OSTimer = 0 as *mut OSTimer;
    let mut count: u32_0 = 0;
    let mut value: u32_0 = 0;
    let mut prevInt: u32_0 = 0;
    (*timer).next = 0 as *mut OSTimer;
    (*timer).prev = 0 as *mut OSTimer;
    (*timer).interval = interval;
    if countdown != 0 as libc::c_int as libc::c_ulonglong {
        (*timer).value = countdown
    } else { (*timer).value = interval }
    (*timer).mq = mq;
    (*timer).msg = msg;
    prevInt = __osDisableInt() as u32_0;
    if (*__osTimerList).next != __osTimerList {
        next = (*__osTimerList).next;
        count = osGetCount();
        value = count.wrapping_sub(__osTimerCounter);
        if (value as libc::c_ulonglong) < (*next).value {
            (*next).value =
                ((*next).value as
                     libc::c_ulonglong).wrapping_sub(value as
                                                         libc::c_ulonglong) as
                    OSTime as OSTime
        } else { (*next).value = 1 as libc::c_int as OSTime }
    }
    time = __osInsertTimer(timer);
    __osSetTimerIntr((*(*__osTimerList).next).value);
    __osRestoreInt(prevInt as s32);
    (time) != 0;
    return 0 as libc::c_int;
}
