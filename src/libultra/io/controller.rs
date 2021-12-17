#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case,
         non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(const_raw_ptr_to_usize_cast, register_tool)]
extern "C" {
    #[no_mangle]
    fn osRecvMesg(mq: *mut OSMesgQueue, msg: *mut OSMesg, flag: s32) -> s32;
    #[no_mangle]
    fn osCreateMesgQueue(mq: *mut OSMesgQueue, msg: *mut OSMesg, count: s32);
    #[no_mangle]
    fn osGetTime() -> OSTime;
    #[no_mangle]
    fn __osSiCreateAccessQueue();
    #[no_mangle]
    fn __osSiRawStartDma(dir: s32, addr: *mut libc::c_void) -> s32;
    #[no_mangle]
    fn osSetTimer(timer: *mut OSTimer, countdown: OSTime, interval: OSTime,
                  mq: *mut OSMesgQueue, msg: OSMesg) -> s32;
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
pub struct OSPifRam {
    pub ram: [u32_0; 15],
    pub status: u32_0,
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
pub struct __OSContRequestHeader {
    pub align: u8_0,
    pub txsize: u8_0,
    pub rxsize: u8_0,
    pub poll: u8_0,
    pub typeh: u8_0,
    pub typel: u8_0,
    pub status: u8_0,
    pub align1: u8_0,
}
#[no_mangle]
pub static mut __osPifInternalBuff: OSPifRam =
    OSPifRam{ram: [0; 15], status: 0,};
#[no_mangle]
pub static mut __osContLastPoll: u8_0 = 0;
#[no_mangle]
pub static mut __osMaxControllers: u8_0 = 0;
// always 4
#[no_mangle]
pub static mut __osEepromTimer: OSTimer =
    OSTimer{next: 0 as *const OSTimer as *mut OSTimer,
            prev: 0 as *const OSTimer as *mut OSTimer,
            interval: 0,
            value: 0,
            mq: 0 as *const OSMesgQueue as *mut OSMesgQueue,
            msg: 0 as *const libc::c_void as *mut libc::c_void,};
#[no_mangle]
pub static mut __osEepromTimerMsgQ: OSMesgQueue =
    OSMesgQueue{mtqueue: 0 as *const OSThread as *mut OSThread,
                fullqueue: 0 as *const OSThread as *mut OSThread,
                validCount: 0,
                first: 0,
                msgCount: 0,
                msg: 0 as *const OSMesg as *mut OSMesg,};
#[no_mangle]
pub static mut __osEepromTimerMsg: OSMesg =
    0 as *const libc::c_void as *mut libc::c_void;
#[no_mangle]
pub static mut gOSContInitialized: u32_0 = 0 as libc::c_int as u32_0;
#[no_mangle]
pub unsafe extern "C" fn osContInit(mut mq: *mut OSMesgQueue,
                                    mut ctlBitfield: *mut u8_0,
                                    mut status: *mut OSContStatus) -> s32 {
    let mut mesg: OSMesg = 0 as *mut libc::c_void;
    let mut ret: s32 = 0 as libc::c_int;
    let mut currentTime: OSTime = 0;
    let mut timer: OSTimer =
        OSTimer{next: 0 as *const OSTimer as *mut OSTimer,
                prev: 0 as *const OSTimer as *mut OSTimer,
                interval: 0,
                value: 0,
                mq: 0 as *const OSMesgQueue as *mut OSMesgQueue,
                msg: 0 as *const libc::c_void as *mut libc::c_void,};
    let mut timerqueue: OSMesgQueue =
        OSMesgQueue{mtqueue: 0 as *const OSThread as *mut OSThread,
                    fullqueue: 0 as *const OSThread as *mut OSThread,
                    validCount: 0,
                    first: 0,
                    msgCount: 0,
                    msg: 0 as *const OSMesg as *mut OSMesg,};
    if gOSContInitialized != 0 { return 0 as libc::c_int }
    gOSContInitialized = 1 as libc::c_int as u32_0;
    currentTime = osGetTime();
    if (500000 as libc::c_int as
            u64_0).wrapping_mul((62500000 as libc::c_longlong *
                                     3 as libc::c_int as libc::c_longlong /
                                     4 as libc::c_int as libc::c_longlong /
                                     15625 as libc::c_longlong) as
                                    libc::c_ulonglong).wrapping_div((1000000
                                                                         as
                                                                         libc::c_longlong
                                                                         /
                                                                         15625
                                                                             as
                                                                             libc::c_longlong)
                                                                        as
                                                                        libc::c_ulonglong)
           > currentTime {
        osCreateMesgQueue(&mut timerqueue, &mut mesg, 1 as libc::c_int);
        osSetTimer(&mut timer,
                   (500000 as libc::c_int as
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
                                                                                    libc::c_ulonglong).wrapping_sub(currentTime),
                   0 as libc::c_int as OSTime, &mut timerqueue,
                   &mut mesg as *mut OSMesg as OSMesg);
        osRecvMesg(&mut timerqueue, &mut mesg, 1 as libc::c_int);
    }
    __osMaxControllers = 4 as libc::c_int as u8_0;
    __osPackRequestData(0 as libc::c_int as u8_0);
    ret =
        __osSiRawStartDma(1 as libc::c_int,
                          &mut __osPifInternalBuff as *mut OSPifRam as
                              *mut libc::c_void);
    osRecvMesg(mq, &mut mesg, 1 as libc::c_int);
    ret =
        __osSiRawStartDma(0 as libc::c_int,
                          &mut __osPifInternalBuff as *mut OSPifRam as
                              *mut libc::c_void);
    osRecvMesg(mq, &mut mesg, 1 as libc::c_int);
    __osContGetInitData(ctlBitfield, status);
    __osContLastPoll = 0 as libc::c_int as u8_0;
    __osSiCreateAccessQueue();
    osCreateMesgQueue(&mut __osEepromTimerMsgQ, &mut __osEepromTimerMsg,
                      1 as libc::c_int);
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn __osContGetInitData(mut ctlBitfield: *mut u8_0,
                                             mut status: *mut OSContStatus) {
    let mut bufptr: *mut u8_0 = 0 as *mut u8_0;
    let mut req: __OSContRequestHeader =
        __OSContRequestHeader{align: 0,
                              txsize: 0,
                              rxsize: 0,
                              poll: 0,
                              typeh: 0,
                              typel: 0,
                              status: 0,
                              align1: 0,};
    let mut i: s32 = 0;
    let mut bitfieldTemp: u8_0 = 0 as libc::c_int as u8_0;
    bufptr = &mut __osPifInternalBuff as *mut OSPifRam as *mut u8_0;
    i = 0 as libc::c_int;
    while i < __osMaxControllers as libc::c_int {
        req = *(bufptr as *mut __OSContRequestHeader);
        (*status).errno =
            ((req.rxsize as libc::c_int & 0xc0 as libc::c_int) >>
                 4 as libc::c_int) as u8_0;
        if !((*status).errno != 0) {
            (*status).type_0 =
                ((req.typel as libc::c_int) << 8 as libc::c_int |
                     req.typeh as libc::c_int) as u16_0;
            (*status).status = req.status;
            bitfieldTemp =
                (bitfieldTemp as libc::c_int | (1 as libc::c_int) << i) as
                    u8_0
        }
        i += 1;
        bufptr =
            bufptr.offset(::std::mem::size_of::<__OSContRequestHeader>() as
                              libc::c_ulong as isize);
        status = status.offset(1)
    }
    *ctlBitfield = bitfieldTemp;
}
#[no_mangle]
pub unsafe extern "C" fn __osPackRequestData(mut poll: u8_0) {
    let mut bufptr: *mut u8_0 = 0 as *mut u8_0;
    let mut req: __OSContRequestHeader =
        __OSContRequestHeader{align: 0,
                              txsize: 0,
                              rxsize: 0,
                              poll: 0,
                              typeh: 0,
                              typel: 0,
                              status: 0,
                              align1: 0,};
    let mut i: s32 = 0;
    i = 0 as libc::c_int;
    while i < 0xf as libc::c_int {
        __osPifInternalBuff.ram[i as usize] = 0 as libc::c_int as u32_0;
        i += 1
    }
    __osPifInternalBuff.status = 1 as libc::c_int as u32_0;
    bufptr = &mut __osPifInternalBuff as *mut OSPifRam as *mut u8_0;
    req.align = 0xff as libc::c_int as u8_0;
    req.txsize = 1 as libc::c_int as u8_0;
    req.rxsize = 3 as libc::c_int as u8_0;
    req.poll = poll;
    req.typeh = 0xff as libc::c_int as u8_0;
    req.typel = 0xff as libc::c_int as u8_0;
    req.status = 0xff as libc::c_int as u8_0;
    req.align1 = 0xff as libc::c_int as u8_0;
    i = 0 as libc::c_int;
    while i < __osMaxControllers as libc::c_int {
        *(bufptr as *mut __OSContRequestHeader) = req;
        bufptr =
            bufptr.offset(::std::mem::size_of::<__OSContRequestHeader>() as
                              libc::c_ulong as isize);
        i += 1
    }
    *bufptr = 254 as libc::c_int as u8_0;
}
