#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case,
         non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(register_tool)]
extern "C" {
    #[no_mangle]
    fn osSyncPrintf(fmt: *const libc::c_char, _: ...);
    #[no_mangle]
    fn StackCheck_Check(entry: *mut StackEntry) -> u32_0;
    #[no_mangle]
    fn LogUtils_CheckNullPointer(exp: *const libc::c_char,
                                 ptr: *mut libc::c_void,
                                 file: *const libc::c_char, line: s32);
    #[no_mangle]
    fn osSendMesg(mq: *mut OSMesgQueue, mesg: OSMesg, flag: s32) -> s32;
    #[no_mangle]
    fn osRecvMesg(mq: *mut OSMesgQueue, msg: *mut OSMesg, flag: s32) -> s32;
    #[no_mangle]
    fn osCreateThread(thread: *mut OSThread, id: OSId,
                      entry:
                          Option<unsafe extern "C" fn(_: *mut libc::c_void)
                                     -> ()>, arg: *mut libc::c_void,
                      sp: *mut libc::c_void, pri: OSPri);
    #[no_mangle]
    fn osSetEventMesg(e: OSEvent, mq: *mut OSMesgQueue, msg: OSMesg);
    #[no_mangle]
    fn osCreateMesgQueue(mq: *mut OSMesgQueue, msg: *mut OSMesg, count: s32);
    #[no_mangle]
    fn osGetTime() -> OSTime;
    #[no_mangle]
    fn osStartThread(thread: *mut OSThread);
    #[no_mangle]
    fn osSetIntMask(_: OSIntMask) -> OSIntMask;
    #[no_mangle]
    fn osSetTimer(timer: *mut OSTimer, countdown: OSTime, interval: OSTime,
                  mq: *mut OSMesgQueue, msg: OSMesg) -> s32;
    #[no_mangle]
    fn osAfterPreNMI() -> s32;
    #[no_mangle]
    fn osViSetEvent(mq: *mut OSMesgQueue, m: OSMesg, retraceCount: u32_0);
}
pub type u8_0 = libc::c_uchar;
pub type s16 = libc::c_short;
pub type u16_0 = libc::c_ushort;
pub type s32 = libc::c_int;
pub type u32_0 = libc::c_uint;
pub type u64_0 = libc::c_ulonglong;
pub type vu32 = u32_0;
pub type f32_0 = libc::c_float;
pub type OSIntMask = u32_0;
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
pub type OSEvent = u32_0;
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
pub struct StackEntry {
    pub next: *mut StackEntry,
    pub prev: *mut StackEntry,
    pub head: u32_0,
    pub tail: u32_0,
    pub initValue: u32_0,
    pub minSpace: s32,
    pub name: *const libc::c_char,
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
#[no_mangle]
pub static mut gIrqMgrResetStatus: vu32 = 0 as libc::c_int as u32_0;
#[no_mangle]
pub static mut sIrqMgrResetTime: OSTime = 0 as libc::c_int as OSTime;
#[no_mangle]
pub static mut gIrqMgrRetraceTime: OSTime = 0 as libc::c_int as OSTime;
#[no_mangle]
pub static mut sIrqMgrRetraceCount: u32_0 = 0 as libc::c_int as u32_0;
#[no_mangle]
pub unsafe extern "C" fn IrqMgr_AddClient(mut this: *mut IrqMgr,
                                          mut c: *mut IrqMgrClient,
                                          mut msgQ: *mut OSMesgQueue) {
    let mut prevInt: OSIntMask = 0;
    LogUtils_CheckNullPointer(b"this\x00" as *const u8 as *const libc::c_char,
                              this as *mut libc::c_void,
                              b"../irqmgr.c\x00" as *const u8 as
                                  *const libc::c_char, 96 as libc::c_int);
    LogUtils_CheckNullPointer(b"c\x00" as *const u8 as *const libc::c_char,
                              c as *mut libc::c_void,
                              b"../irqmgr.c\x00" as *const u8 as
                                  *const libc::c_char, 97 as libc::c_int);
    LogUtils_CheckNullPointer(b"msgQ\x00" as *const u8 as *const libc::c_char,
                              msgQ as *mut libc::c_void,
                              b"../irqmgr.c\x00" as *const u8 as
                                  *const libc::c_char, 98 as libc::c_int);
    prevInt = osSetIntMask(1 as libc::c_int as OSIntMask);
    (*c).queue = msgQ;
    (*c).prev = (*this).clients;
    (*this).clients = c;
    osSetIntMask(prevInt);
    if (*this).resetStatus as libc::c_int > 0 as libc::c_int {
        osSendMesg((*c).queue,
                   &mut (*this).prenmiMsg as *mut OSScMsg as OSMesg,
                   0 as libc::c_int);
    }
    if (*this).resetStatus as libc::c_int >= 2 as libc::c_int {
        osSendMesg((*c).queue, &mut (*this).nmiMsg as *mut OSScMsg as OSMesg,
                   0 as libc::c_int);
    };
}
#[no_mangle]
pub unsafe extern "C" fn IrqMgr_RemoveClient(mut this: *mut IrqMgr,
                                             mut c: *mut IrqMgrClient) {
    let mut iter: *mut IrqMgrClient = (*this).clients;
    let mut lastIter: *mut IrqMgrClient = 0 as *mut IrqMgrClient;
    let mut prevInt: OSIntMask = 0;
    LogUtils_CheckNullPointer(b"this\x00" as *const u8 as *const libc::c_char,
                              this as *mut libc::c_void,
                              b"../irqmgr.c\x00" as *const u8 as
                                  *const libc::c_char, 129 as libc::c_int);
    LogUtils_CheckNullPointer(b"c\x00" as *const u8 as *const libc::c_char,
                              c as *mut libc::c_void,
                              b"../irqmgr.c\x00" as *const u8 as
                                  *const libc::c_char, 130 as libc::c_int);
    prevInt = osSetIntMask(1 as libc::c_int as OSIntMask);
    while !iter.is_null() {
        if iter == c {
            if !lastIter.is_null() {
                (*lastIter).prev = (*c).prev
            } else { (*this).clients = (*c).prev }
            break ;
        } else { lastIter = iter; iter = (*iter).prev }
    }
    osSetIntMask(prevInt);
}
#[no_mangle]
pub unsafe extern "C" fn IrqMgr_SendMesgForClient(mut this: *mut IrqMgr,
                                                  mut msg: OSMesg) {
    let mut iter: *mut IrqMgrClient = (*this).clients;
    while !iter.is_null() {
        if (*(*iter).queue).validCount >= (*(*iter).queue).msgCount {
            // "irqmgr_SendMesgForClient: Message queue is overflowing mq=%08x cnt=%d"
            osSyncPrintf(b"\x1b[41;37mirqmgr_SendMesgForClient:\xe3\x83\xa1\xe3\x83\x83\xe3\x82\xbb\xe3\x83\xbc\xe3\x82\xb8\xe3\x82\xad\xe3\x83\xa5\xe3\x83\xbc\xe3\x81\x8c\xe3\x81\x82\xe3\x81\xb5\xe3\x82\x8c\xe3\x81\xa6\xe3\x81\x84\xe3\x81\xbe\xe3\x81\x99 mq=%08x cnt=%d\n\x1b[m\x00"
                             as *const u8 as *const libc::c_char,
                         (*iter).queue, (*(*iter).queue).validCount);
        } else { osSendMesg((*iter).queue, msg, 0 as libc::c_int); }
        iter = (*iter).prev
    };
}
#[no_mangle]
pub unsafe extern "C" fn IrqMgr_JamMesgForClient(mut this: *mut IrqMgr,
                                                 mut msg: OSMesg) {
    let mut iter: *mut IrqMgrClient = (*this).clients;
    while !iter.is_null() {
        if (*(*iter).queue).validCount >= (*(*iter).queue).msgCount {
            // "irqmgr_JamMesgForClient: Message queue is overflowing mq=%08x cnt=%d"
            osSyncPrintf(b"\x1b[41;37mirqmgr_JamMesgForClient:\xe3\x83\xa1\xe3\x83\x83\xe3\x82\xbb\xe3\x83\xbc\xe3\x82\xb8\xe3\x82\xad\xe3\x83\xa5\xe3\x83\xbc\xe3\x81\x8c\xe3\x81\x82\xe3\x81\xb5\xe3\x82\x8c\xe3\x81\xa6\xe3\x81\x84\xe3\x81\xbe\xe3\x81\x99 mq=%08x cnt=%d\n\x1b[m\x00"
                             as *const u8 as *const libc::c_char,
                         (*iter).queue, (*(*iter).queue).validCount);
        } else {
            // mistake? the function's name suggests it would use osJamMesg
            osSendMesg((*iter).queue, msg,
                       0 as libc::c_int); // required to match
        } // "0.5 seconds after PRENMI"
        iter = (*iter).prev
    };
}
#[no_mangle]
pub unsafe extern "C" fn IrqMgr_HandlePreNMI(mut this: *mut IrqMgr) {
    let mut temp: u64_0 = 1 as libc::c_int as u64_0;
    ::std::ptr::write_volatile(&mut gIrqMgrResetStatus as *mut vu32,
                               temp as u32_0);
    (*this).resetStatus = 1 as libc::c_int as u8_0;
    (*this).resetTime = osGetTime();
    ::std::ptr::write_volatile(&mut sIrqMgrResetTime as *mut OSTime,
                               (*this).resetTime);
    osSetTimer(&mut (*this).timer,
               (450000 as libc::c_int as
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
                                                                                libc::c_ulonglong),
               0 as libc::c_ulonglong, &mut (*this).queue,
               671 as libc::c_int as OSMesg);
    IrqMgr_JamMesgForClient(this,
                            &mut (*this).prenmiMsg as *mut OSScMsg as OSMesg);
}
#[no_mangle]
pub unsafe extern "C" fn IrqMgr_CheckStack() {
    osSyncPrintf(b"irqmgr.c: PRENMI\xe3\x81\x8b\xe3\x82\x890.5\xe7\xa7\x92\xe7\xb5\x8c\xe9\x81\x8e\n\x00"
                     as *const u8 as *const libc::c_char);
    if StackCheck_Check(0 as *mut StackEntry) ==
           0 as libc::c_int as libc::c_uint {
        osSyncPrintf(b"\xe3\x82\xb9\xe3\x82\xbf\xe3\x83\x83\xe3\x82\xaf\xe3\x81\xaf\xe5\xa4\xa7\xe4\xb8\x88\xe5\xa4\xab\xe3\x81\xbf\xe3\x81\x9f\xe3\x81\x84\xe3\x81\xa7\xe3\x81\x99\n\x00"
                         as *const u8 as *const libc::c_char);
        // "The stack looks ok"
    } else {
        osSyncPrintf(b"%c\x00" as *const u8 as *const libc::c_char,
                     7 as libc::c_int);
        osSyncPrintf(b"\x1b[31m\x00" as *const u8 as *const libc::c_char);
        // "Stack overflow or dangerous"
        osSyncPrintf(b"\xe3\x82\xb9\xe3\x82\xbf\xe3\x83\x83\xe3\x82\xaf\xe3\x81\x8c\xe3\x82\xaa\xe3\x83\xbc\xe3\x83\x90\xe3\x83\xbc\xe3\x83\x95\xe3\x83\xad\xe3\x83\xbc\xe3\x81\x97\xe3\x81\x9f\xe3\x81\x8b\xe5\x8d\xb1\xe9\x99\xba\xe3\x81\xaa\xe7\x8a\xb6\xe6\x85\x8b\xe3\x81\xa7\xe3\x81\x99\n\x00"
                         as *const u8 as *const libc::c_char);
        // "Increase stack size early or don't consume stack"
        osSyncPrintf(b"\xe6\x97\xa9\xe3\x80\x85\xe3\x81\xab\xe3\x82\xb9\xe3\x82\xbf\xe3\x83\x83\xe3\x82\xaf\xe3\x82\xb5\xe3\x82\xa4\xe3\x82\xba\xe3\x82\x92\xe5\xa2\x97\xe3\x82\x84\xe3\x81\x99\xe3\x81\x8b\xe3\x80\x81\xe3\x82\xb9\xe3\x82\xbf\xe3\x83\x83\xe3\x82\xaf\xe3\x82\x92\xe6\xb6\x88\xe8\xb2\xbb\xe3\x81\x97\xe3\x81\xaa\xe3\x81\x84\xe3\x82\x88\xe3\x81\x86\xe3\x81\xab\xe3\x81\x97\xe3\x81\xa6\xe3\x81\x8f\xe3\x81\xa0\xe3\x81\x95\xe3\x81\x84\n\x00"
                         as *const u8 as
                         *const libc::c_char); // required to match
        osSyncPrintf(b"\x1b[m\x00" as *const u8 as *const libc::c_char);
    };
}
#[no_mangle]
pub unsafe extern "C" fn IrqMgr_HandlePRENMI450(mut this: *mut IrqMgr) {
    let mut temp: u64_0 = 2 as libc::c_int as u64_0;
    ::std::ptr::write_volatile(&mut gIrqMgrResetStatus as *mut vu32,
                               temp as u32_0);
    (*this).resetStatus = 2 as libc::c_int as u8_0;
    osSetTimer(&mut (*this).timer,
               (30000 as libc::c_int as
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
                                                                                libc::c_ulonglong),
               0 as libc::c_ulonglong, &mut (*this).queue,
               672 as libc::c_int as OSMesg);
    IrqMgr_SendMesgForClient(this,
                             &mut (*this).nmiMsg as *mut OSScMsg as OSMesg);
}
#[no_mangle]
pub unsafe extern "C" fn IrqMgr_HandlePRENMI480(mut this: *mut IrqMgr) {
    let mut ret: u32_0 = 0;
    osSetTimer(&mut (*this).timer,
               (20000 as libc::c_int as
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
                                                                                libc::c_ulonglong),
               0 as libc::c_ulonglong, &mut (*this).queue,
               673 as libc::c_int as OSMesg);
    ret = osAfterPreNMI() as u32_0;
    if ret != 0 {
        // "osAfterPreNMI returned %d !?"
        osSyncPrintf(b"osAfterPreNMI\xe3\x81\x8c %d \xe3\x82\x92\xe8\xbf\x94\xe3\x81\x97\xe3\x81\xbe\xe3\x81\x97\xe3\x81\x9f\xef\xbc\x81\xef\xbc\x9f\n\x00"
                         as *const u8 as *const libc::c_char,
                     ret); // "Start IRQ manager thread execution"
        osSetTimer(&mut (*this).timer,
                   (1000 as libc::c_int as
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
                                                                                    libc::c_ulonglong),
                   0 as libc::c_ulonglong, &mut (*this).queue,
                   672 as libc::c_int as OSMesg);
    };
}
#[no_mangle]
pub unsafe extern "C" fn IrqMgr_HandlePRENMI500(mut this: *mut IrqMgr) {
    IrqMgr_CheckStack();
}
#[no_mangle]
pub unsafe extern "C" fn IrqMgr_HandleRetrace(mut this: *mut IrqMgr) {
    if gIrqMgrRetraceTime == 0 as libc::c_ulonglong {
        if (*this).retraceTime == 0 as libc::c_int as libc::c_ulonglong {
            (*this).retraceTime = osGetTime()
        } else {
            ::std::ptr::write_volatile(&mut gIrqMgrRetraceTime as *mut OSTime,
                                       osGetTime().wrapping_sub((*this).retraceTime))
        }
    }
    sIrqMgrRetraceCount = sIrqMgrRetraceCount.wrapping_add(1);
    IrqMgr_SendMesgForClient(this,
                             &mut (*this).retraceMsg as *mut OSScMsg as
                                 OSMesg);
}
#[no_mangle]
pub unsafe extern "C" fn IrqMgr_ThreadEntry(mut arg0: *mut libc::c_void) {
    let mut msg: OSMesg = 0 as *mut libc::c_void;
    let mut this: *mut IrqMgr = arg0 as *mut IrqMgr;
    let mut exit: u8_0 = 0;
    msg = 0 as OSMesg;
    osSyncPrintf(b"\xef\xbc\xa9\xef\xbc\xb2\xef\xbc\xb1\xe3\x83\x9e\xe3\x83\x8d\xe3\x83\xbc\xe3\x82\xb8\xe3\x83\xa3\xe3\x82\xb9\xe3\x83\xac\xe3\x83\x83\xe3\x83\x89\xe5\xae\x9f\xe8\xa1\x8c\xe9\x96\x8b\xe5\xa7\x8b\n\x00"
                     as *const u8 as *const libc::c_char);
    exit = 0 as libc::c_int as u8_0;
    while exit == 0 {
        osRecvMesg(&mut (*this).queue, &mut msg, 1 as libc::c_int);
        match msg as u32_0 {
            666 => { IrqMgr_HandleRetrace(this); }
            669 => {
                osSyncPrintf(b"PRE_NMI_MSG\n\x00" as *const u8 as
                                 *const libc::c_char);
                // "Scheduler: Receives PRE_NMI message"
                osSyncPrintf(b"\xe3\x82\xb9\xe3\x82\xb1\xe3\x82\xb8\xe3\x83\xa5\xe3\x83\xbc\xe3\x83\xa9\xef\xbc\x9aPRE_NMI\xe3\x83\xa1\xe3\x83\x83\xe3\x82\xbb\xe3\x83\xbc\xe3\x82\xb8\xe3\x82\x92\xe5\x8f\x97\xe4\xbf\xa1\n\x00"
                                 as *const u8 as *const libc::c_char);
                IrqMgr_HandlePreNMI(this);
            }
            671 => {
                osSyncPrintf(b"PRENMI450_MSG\n\x00" as *const u8 as
                                 *const libc::c_char);
                // "Scheduler: Receives PRENMI450 message"
                osSyncPrintf(b"\xe3\x82\xb9\xe3\x82\xb1\xe3\x82\xb8\xe3\x83\xa5\xe3\x83\xbc\xe3\x83\xa9\xef\xbc\x9aPRENMI450\xe3\x83\xa1\xe3\x83\x83\xe3\x82\xbb\xe3\x83\xbc\xe3\x82\xb8\xe3\x82\x92\xe5\x8f\x97\xe4\xbf\xa1\n\x00"
                                 as *const u8 as *const libc::c_char);
                IrqMgr_HandlePRENMI450(this);
            }
            672 => {
                osSyncPrintf(b"PRENMI480_MSG\n\x00" as *const u8 as
                                 *const libc::c_char);
                // "Scheduler: Receives PRENMI480 message"
                osSyncPrintf(b"\xe3\x82\xb9\xe3\x82\xb1\xe3\x82\xb8\xe3\x83\xa5\xe3\x83\xbc\xe3\x83\xa9\xef\xbc\x9aPRENMI480\xe3\x83\xa1\xe3\x83\x83\xe3\x82\xbb\xe3\x83\xbc\xe3\x82\xb8\xe3\x82\x92\xe5\x8f\x97\xe4\xbf\xa1\n\x00"
                                 as *const u8 as *const libc::c_char);
                IrqMgr_HandlePRENMI480(this);
            }
            673 => {
                osSyncPrintf(b"PRENMI500_MSG\n\x00" as *const u8 as
                                 *const libc::c_char);
                // "Scheduler: Receives PRENMI500 message"
                osSyncPrintf(b"\xe3\x82\xb9\xe3\x82\xb1\xe3\x82\xb8\xe3\x83\xa5\xe3\x83\xbc\xe3\x83\xa9\xef\xbc\x9aPRENMI500\xe3\x83\xa1\xe3\x83\x83\xe3\x82\xbb\xe3\x83\xbc\xe3\x82\xb8\xe3\x82\x92\xe5\x8f\x97\xe4\xbf\xa1\n\x00"
                                 as *const u8 as *const libc::c_char);
                exit = 1 as libc::c_int as u8_0;
                IrqMgr_HandlePRENMI500(this);
            }
            _ => {
                // "Unexpected message received"
                osSyncPrintf(b"irqmgr.c:\xe4\xba\x88\xe6\x9c\x9f\xe3\x81\x97\xe3\x81\xaa\xe3\x81\x84\xe3\x83\xa1\xe3\x83\x83\xe3\x82\xbb\xe3\x83\xbc\xe3\x82\xb8\xe3\x82\x92\xe5\x8f\x97\xe3\x81\x91\xe5\x8f\x96\xe3\x82\x8a\xe3\x81\xbe\xe3\x81\x97\xe3\x81\x9f(%08x)\n\x00"
                                 as *const u8 as *const libc::c_char, msg);
            }
        }
    }
    osSyncPrintf(b"\xef\xbc\xa9\xef\xbc\xb2\xef\xbc\xb1\xe3\x83\x9e\xe3\x83\x8d\xe3\x83\xbc\xe3\x82\xb8\xe3\x83\xa3\xe3\x82\xb9\xe3\x83\xac\xe3\x83\x83\xe3\x83\x89\xe5\xae\x9f\xe8\xa1\x8c\xe7\xb5\x82\xe4\xba\x86\n\x00"
                     as *const u8 as *const libc::c_char);
    // "End of IRQ manager thread execution"
}
#[no_mangle]
pub unsafe extern "C" fn IrqMgr_Init(mut this: *mut IrqMgr,
                                     mut stack: *mut libc::c_void,
                                     mut pri: OSPri, mut retraceCount: u8_0) {
    LogUtils_CheckNullPointer(b"this\x00" as *const u8 as *const libc::c_char,
                              this as *mut libc::c_void,
                              b"../irqmgr.c\x00" as *const u8 as
                                  *const libc::c_char, 346 as libc::c_int);
    LogUtils_CheckNullPointer(b"stack\x00" as *const u8 as
                                  *const libc::c_char, stack,
                              b"../irqmgr.c\x00" as *const u8 as
                                  *const libc::c_char, 347 as libc::c_int);
    (*this).clients = 0 as *mut IrqMgrClient;
    (*this).retraceMsg.type_0 = 1 as libc::c_int as s16;
    (*this).prenmiMsg.type_0 = 4 as libc::c_int as s16;
    (*this).nmiMsg.type_0 = 3 as libc::c_int as s16;
    (*this).resetStatus = 0 as libc::c_int as u8_0;
    (*this).resetTime = 0 as libc::c_int as OSTime;
    osCreateMesgQueue(&mut (*this).queue, (*this).msgBuf.as_mut_ptr(),
                      (::std::mem::size_of::<[OSMesg; 8]>() as
                           libc::c_ulong).wrapping_div(::std::mem::size_of::<OSMesg>()
                                                           as libc::c_ulong)
                          as s32);
    osSetEventMesg(14 as libc::c_int as OSEvent, &mut (*this).queue,
                   669 as libc::c_int as OSMesg);
    osViSetEvent(&mut (*this).queue, 666 as libc::c_int as OSMesg,
                 retraceCount as u32_0);
    osCreateThread(&mut (*this).thread, 0x13 as libc::c_int,
                   Some(IrqMgr_ThreadEntry as
                            unsafe extern "C" fn(_: *mut libc::c_void) -> ()),
                   this as *mut libc::c_void, stack, pri);
    osStartThread(&mut (*this).thread);
}
