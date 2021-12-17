#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case,
         non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(const_raw_ptr_to_usize_cast, register_tool)]
extern "C" {
    #[no_mangle]
    fn osSendMesg(mq: *mut OSMesgQueue, mesg: OSMesg, flag: s32) -> s32;
    #[no_mangle]
    fn osGetCount() -> u32_0;
    #[no_mangle]
    fn __osSetCompare(_: u32_0);
    #[no_mangle]
    fn __osDisableInt() -> s32;
    #[no_mangle]
    fn __osRestoreInt(_: s32);
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
pub static mut __osBaseTimer: OSTimer =
    OSTimer{next: 0 as *const OSTimer as *mut OSTimer,
            prev: 0 as *const OSTimer as *mut OSTimer,
            interval: 0,
            value: 0,
            mq: 0 as *const OSMesgQueue as *mut OSMesgQueue,
            msg: 0 as *const libc::c_void as *mut libc::c_void,};
#[no_mangle]
pub static mut __osCurrentTime: OSTime = 0;
#[no_mangle]
pub static mut __osBaseCounter: u32_0 = 0;
#[no_mangle]
pub static mut __osViIntrCount: u32_0 = 0;
#[no_mangle]
pub static mut __osTimerCounter: u32_0 = 0;
#[no_mangle]
pub static mut __osTimerList: *mut OSTimer =
    unsafe { &__osBaseTimer as *const OSTimer as *mut OSTimer };
#[no_mangle]
pub unsafe extern "C" fn __osTimerServicesInit() {
    __osCurrentTime = 0 as libc::c_int as OSTime;
    __osBaseCounter = 0 as libc::c_int as u32_0;
    __osViIntrCount = 0 as libc::c_int as u32_0;
    (*__osTimerList).prev = __osTimerList;
    (*__osTimerList).next = (*__osTimerList).prev;
    (*__osTimerList).value = 0 as libc::c_int as OSTime;
    (*__osTimerList).interval = (*__osTimerList).value;
    (*__osTimerList).mq = 0 as *mut OSMesgQueue;
    (*__osTimerList).msg = 0 as *mut libc::c_void;
}
#[no_mangle]
pub unsafe extern "C" fn __osTimerInterrupt() {
    let mut timer: *mut OSTimer = 0 as *mut OSTimer;
    let mut sp20: u32_0 = 0;
    let mut sp1c: u32_0 = 0;
    if (*__osTimerList).next == __osTimerList { return }
    loop  {
        timer = (*__osTimerList).next;
        if timer == __osTimerList {
            __osSetCompare(0 as libc::c_int as u32_0);
            __osTimerCounter = 0 as libc::c_int as u32_0;
            break ;
        } else {
            sp20 = osGetCount();
            sp1c = sp20.wrapping_sub(__osTimerCounter);
            __osTimerCounter = sp20;
            if (sp1c as libc::c_ulonglong) < (*timer).value {
                (*timer).value =
                    ((*timer).value as
                         libc::c_ulonglong).wrapping_sub(sp1c as
                                                             libc::c_ulonglong)
                        as OSTime as OSTime;
                __osSetTimerIntr((*timer).value);
                break ;
            } else {
                (*(*timer).prev).next = (*timer).next;
                (*(*timer).next).prev = (*timer).prev;
                (*timer).next = 0 as *mut OSTimer;
                (*timer).prev = 0 as *mut OSTimer;
                if !(*timer).mq.is_null() {
                    osSendMesg((*timer).mq, (*timer).msg, 0 as libc::c_int);
                }
                if (*timer).interval != 0 as libc::c_int as libc::c_ulonglong
                   {
                    (*timer).value = (*timer).interval;
                    __osInsertTimer(timer);
                }
            }
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn __osSetTimerIntr(mut time: OSTime) {
    let mut newTime: OSTime = 0;
    let mut prevInt: u32_0 = 0;
    if time < 468 as libc::c_int as libc::c_ulonglong {
        time = 468 as libc::c_int as OSTime
    }
    prevInt = __osDisableInt() as u32_0;
    __osTimerCounter = osGetCount();
    newTime = time.wrapping_add(__osTimerCounter as libc::c_ulonglong);
    __osSetCompare(newTime as u32_0);
    __osRestoreInt(prevInt as s32);
}
#[no_mangle]
pub unsafe extern "C" fn __osInsertTimer(mut timer: *mut OSTimer) -> OSTime {
    let mut nextTimer: *mut OSTimer = 0 as *mut OSTimer;
    let mut timerValue: u64_0 = 0;
    let mut prevInt: u32_0 = __osDisableInt() as u32_0;
    nextTimer = (*__osTimerList).next;
    timerValue = (*timer).value;
    while nextTimer != __osTimerList && timerValue > (*nextTimer).value {
        timerValue =
            (timerValue as libc::c_ulonglong).wrapping_sub((*nextTimer).value)
                as u64_0 as u64_0;
        nextTimer = (*nextTimer).next
    }
    (*timer).value = timerValue;
    if nextTimer != __osTimerList {
        (*nextTimer).value =
            ((*nextTimer).value as libc::c_ulonglong).wrapping_sub(timerValue)
                as OSTime as OSTime
    }
    (*timer).next = nextTimer;
    (*timer).prev = (*nextTimer).prev;
    (*(*nextTimer).prev).next = timer;
    (*nextTimer).prev = timer;
    __osRestoreInt(prevInt as s32);
    return timerValue;
}
