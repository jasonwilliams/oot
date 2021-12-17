#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case,
         non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(const_raw_ptr_to_usize_cast, register_tool)]
extern "C" {
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
    fn osSetThreadPri(thread: *mut OSThread, pri: OSPri);
    #[no_mangle]
    fn osGetThreadPri(thread: *mut OSThread) -> OSPri;
    #[no_mangle]
    fn __osTimerServicesInit();
    #[no_mangle]
    fn __osTimerInterrupt();
    #[no_mangle]
    fn osGetCount() -> u32_0;
    #[no_mangle]
    fn __osDisableInt() -> s32;
    #[no_mangle]
    fn __osRestoreInt(_: s32);
    #[no_mangle]
    fn __osViInit();
    #[no_mangle]
    fn __osViSwapContext();
    #[no_mangle]
    fn __osViGetCurrentContext() -> *mut OSViContext;
    #[no_mangle]
    fn osStartThread(thread: *mut OSThread);
    #[no_mangle]
    static mut __osViIntrCount: u32_0;
    #[no_mangle]
    static mut __osBaseCounter: u32_0;
    #[no_mangle]
    static mut __osCurrentTime: OSTime;
}
pub type u8_0 = libc::c_uchar;
pub type u16_0 = libc::c_ushort;
pub type s32 = libc::c_int;
pub type u32_0 = libc::c_uint;
pub type u64_0 = libc::c_ulonglong;
pub type f32_0 = libc::c_float;
pub type size_t = libc::c_ulong;
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
pub struct OSViCommonRegs {
    pub ctrl: u32_0,
    pub width: u32_0,
    pub burst: u32_0,
    pub vSync: u32_0,
    pub hSync: u32_0,
    pub leap: u32_0,
    pub hStart: u32_0,
    pub xScale: u32_0,
    pub vCurrent: u32_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct OSViFieldRegs {
    pub origin: u32_0,
    pub yScale: u32_0,
    pub vStart: u32_0,
    pub vBurst: u32_0,
    pub vIntr: u32_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct OSViMode {
    pub type_0: u8_0,
    pub comRegs: OSViCommonRegs,
    pub fldRegs: [OSViFieldRegs; 2],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __OSViScale {
    pub factor: f32_0,
    pub offset: u16_0,
    pub scale: u32_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct OSViContext {
    pub state: u16_0,
    pub retraceCount: u16_0,
    pub buffer: *mut libc::c_void,
    pub modep: *mut OSViMode,
    pub features: u32_0,
    pub mq: *mut OSMesgQueue,
    pub msg: *mut OSMesg,
    pub x: __OSViScale,
    pub y: __OSViScale,
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
pub struct OSIoMesgHdr {
    pub type_0: u16_0,
    pub pri: u8_0,
    pub status: u8_0,
    pub retQueue: *mut OSMesgQueue,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct OSIoMesg {
    pub hdr: OSIoMesgHdr,
    pub dramAddr: *mut libc::c_void,
    pub devAddr: u32_0,
    pub size: size_t,
    pub piHandle: *mut OSPiHandle,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct OSMgrArgs {
    pub initialized: u32_0,
    pub mgrThread: *mut OSThread,
    pub cmdQueue: *mut OSMesgQueue,
    pub eventQueue: *mut OSMesgQueue,
    pub acccessQueue: *mut OSMesgQueue,
    pub piDmaCallback: Option<unsafe extern "C" fn(_: s32, _: u32_0,
                                                   _: *mut libc::c_void,
                                                   _: size_t) -> s32>,
    pub epiDmaCallback: Option<unsafe extern "C" fn(_: *mut OSPiHandle,
                                                    _: s32, _: u32_0,
                                                    _: *mut libc::c_void,
                                                    _: size_t) -> s32>,
}
#[no_mangle]
pub static mut viThread: OSThread =
    OSThread{next: 0 as *const OSThread as *mut OSThread,
             priority: 0,
             queue: 0 as *const *mut OSThread as *mut *mut OSThread,
             tlnext: 0 as *const OSThread as *mut OSThread,
             state: 0,
             flags: 0,
             id: 0,
             fp: 0,
             thprof: 0 as *const __OSThreadprofile as *mut __OSThreadprofile,
             context:
                 __OSThreadContext{at: 0,
                                   v0: 0,
                                   v1: 0,
                                   a0: 0,
                                   a1: 0,
                                   a2: 0,
                                   a3: 0,
                                   t0: 0,
                                   t1: 0,
                                   t2: 0,
                                   t3: 0,
                                   t4: 0,
                                   t5: 0,
                                   t6: 0,
                                   t7: 0,
                                   s0: 0,
                                   s1: 0,
                                   s2: 0,
                                   s3: 0,
                                   s4: 0,
                                   s5: 0,
                                   s6: 0,
                                   s7: 0,
                                   t8: 0,
                                   t9: 0,
                                   gp: 0,
                                   sp: 0,
                                   s8: 0,
                                   ra: 0,
                                   lo: 0,
                                   hi: 0,
                                   sr: 0,
                                   pc: 0,
                                   cause: 0,
                                   badvaddr: 0,
                                   rcp: 0,
                                   fpcsr: 0,
                                   fp0:
                                       __OSfp{f:
                                                  C2RustUnnamed{f_odd: 0.,
                                                                f_even:
                                                                    0.,},},
                                   fp2:
                                       __OSfp{f:
                                                  C2RustUnnamed{f_odd: 0.,
                                                                f_even:
                                                                    0.,},},
                                   fp4:
                                       __OSfp{f:
                                                  C2RustUnnamed{f_odd: 0.,
                                                                f_even:
                                                                    0.,},},
                                   fp6:
                                       __OSfp{f:
                                                  C2RustUnnamed{f_odd: 0.,
                                                                f_even:
                                                                    0.,},},
                                   fp8:
                                       __OSfp{f:
                                                  C2RustUnnamed{f_odd: 0.,
                                                                f_even:
                                                                    0.,},},
                                   fp10:
                                       __OSfp{f:
                                                  C2RustUnnamed{f_odd: 0.,
                                                                f_even:
                                                                    0.,},},
                                   fp12:
                                       __OSfp{f:
                                                  C2RustUnnamed{f_odd: 0.,
                                                                f_even:
                                                                    0.,},},
                                   fp14:
                                       __OSfp{f:
                                                  C2RustUnnamed{f_odd: 0.,
                                                                f_even:
                                                                    0.,},},
                                   fp16:
                                       __OSfp{f:
                                                  C2RustUnnamed{f_odd: 0.,
                                                                f_even:
                                                                    0.,},},
                                   fp18:
                                       __OSfp{f:
                                                  C2RustUnnamed{f_odd: 0.,
                                                                f_even:
                                                                    0.,},},
                                   fp20:
                                       __OSfp{f:
                                                  C2RustUnnamed{f_odd: 0.,
                                                                f_even:
                                                                    0.,},},
                                   fp22:
                                       __OSfp{f:
                                                  C2RustUnnamed{f_odd: 0.,
                                                                f_even:
                                                                    0.,},},
                                   fp24:
                                       __OSfp{f:
                                                  C2RustUnnamed{f_odd: 0.,
                                                                f_even:
                                                                    0.,},},
                                   fp26:
                                       __OSfp{f:
                                                  C2RustUnnamed{f_odd: 0.,
                                                                f_even:
                                                                    0.,},},
                                   fp28:
                                       __OSfp{f:
                                                  C2RustUnnamed{f_odd: 0.,
                                                                f_even:
                                                                    0.,},},
                                   fp30:
                                       __OSfp{f:
                                                  C2RustUnnamed{f_odd: 0.,
                                                                f_even:
                                                                    0.,},},},};
#[no_mangle]
pub static mut viThreadStack: [u8_0; 4096] = [0; 4096];
#[no_mangle]
pub static mut viEventQueue: OSMesgQueue =
    OSMesgQueue{mtqueue: 0 as *const OSThread as *mut OSThread,
                fullqueue: 0 as *const OSThread as *mut OSThread,
                validCount: 0,
                first: 0,
                msgCount: 0,
                msg: 0 as *const OSMesg as *mut OSMesg,};
#[no_mangle]
pub static mut viEventBuf: [OSMesg; 6] =
    [0 as *const libc::c_void as *mut libc::c_void; 6];
#[no_mangle]
pub static mut viRetraceMsg: OSIoMesg =
    OSIoMesg{hdr:
                 OSIoMesgHdr{type_0: 0,
                             pri: 0,
                             status: 0,
                             retQueue:
                                 0 as *const OSMesgQueue as
                                     *mut OSMesgQueue,},
             dramAddr: 0 as *const libc::c_void as *mut libc::c_void,
             devAddr: 0,
             size: 0,
             piHandle: 0 as *const OSPiHandle as *mut OSPiHandle,};
#[no_mangle]
pub static mut viCounterMsg: OSIoMesg =
    OSIoMesg{hdr:
                 OSIoMesgHdr{type_0: 0,
                             pri: 0,
                             status: 0,
                             retQueue:
                                 0 as *const OSMesgQueue as
                                     *mut OSMesgQueue,},
             dramAddr: 0 as *const libc::c_void as *mut libc::c_void,
             devAddr: 0,
             size: 0,
             piHandle: 0 as *const OSPiHandle as *mut OSPiHandle,};
#[no_mangle]
pub static mut __osViDevMgr: OSMgrArgs =
    {
        let mut init =
            OSMgrArgs{initialized: 0 as libc::c_int as u32_0,
                      mgrThread: 0 as *const OSThread as *mut OSThread,
                      cmdQueue: 0 as *const OSMesgQueue as *mut OSMesgQueue,
                      eventQueue: 0 as *const OSMesgQueue as *mut OSMesgQueue,
                      acccessQueue:
                          0 as *const OSMesgQueue as *mut OSMesgQueue,
                      piDmaCallback: None,
                      epiDmaCallback: None,}; // always 0
        init
    };
#[no_mangle]
pub static mut __additional_scanline: u32_0 = 0 as libc::c_int as u32_0;
#[no_mangle]
pub unsafe extern "C" fn osCreateViManager(mut pri: OSPri) {
    let mut prevInt: u32_0 = 0;
    let mut newPri: OSPri = 0;
    let mut currentPri: OSPri = 0;
    if __osViDevMgr.initialized == 0 {
        __osTimerServicesInit();
        __additional_scanline = 0 as libc::c_int as u32_0;
        osCreateMesgQueue(&mut viEventQueue, viEventBuf.as_mut_ptr(),
                          5 as libc::c_int);
        viRetraceMsg.hdr.type_0 = 13 as libc::c_int as u16_0;
        viRetraceMsg.hdr.pri = 0 as libc::c_int as u8_0;
        viRetraceMsg.hdr.retQueue = 0 as *mut OSMesgQueue;
        viCounterMsg.hdr.type_0 = 14 as libc::c_int as u16_0;
        viCounterMsg.hdr.pri = 0 as libc::c_int as u8_0;
        viCounterMsg.hdr.retQueue = 0 as *mut OSMesgQueue;
        osSetEventMesg(7 as libc::c_int as OSEvent, &mut viEventQueue,
                       &mut viRetraceMsg as *mut OSIoMesg as OSMesg);
        osSetEventMesg(3 as libc::c_int as OSEvent, &mut viEventQueue,
                       &mut viCounterMsg as *mut OSIoMesg as OSMesg);
        newPri = -(1 as libc::c_int);
        currentPri = osGetThreadPri(0 as *mut OSThread);
        if currentPri < pri {
            newPri = currentPri;
            osSetThreadPri(0 as *mut OSThread, pri);
        }
        prevInt = __osDisableInt() as u32_0;
        __osViDevMgr.initialized = 1 as libc::c_int as u32_0;
        __osViDevMgr.mgrThread = &mut viThread;
        __osViDevMgr.cmdQueue = &mut viEventQueue;
        __osViDevMgr.eventQueue = &mut viEventQueue;
        __osViDevMgr.acccessQueue = 0 as *mut OSMesgQueue;
        __osViDevMgr.piDmaCallback = None;
        __osViDevMgr.epiDmaCallback = None;
        osCreateThread(&mut viThread, 0 as libc::c_int,
                       Some(viMgrMain as
                                unsafe extern "C" fn(_: *mut libc::c_void)
                                    -> ()),
                       &mut __osViDevMgr as *mut OSMgrArgs as
                           *mut libc::c_void,
                       viThreadStack.as_mut_ptr().offset(::std::mem::size_of::<[u8_0; 4096]>()
                                                             as libc::c_ulong
                                                             as isize) as
                           *mut libc::c_void, pri);
        __osViInit();
        osStartThread(&mut viThread);
        __osRestoreInt(prevInt as s32);
        if newPri != -(1 as libc::c_int) {
            osSetThreadPri(0 as *mut OSThread, newPri);
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn viMgrMain(mut vargs: *mut libc::c_void) {
    static mut viRetrace: u16_0 = 0;
    let mut args: *mut OSMgrArgs = 0 as *mut OSMgrArgs;
    let mut addTime: u32_0 = 0;
    let mut mesg: *mut OSIoMesg = 0 as *mut OSIoMesg;
    let mut temp: u32_0 = 0 as libc::c_int as u32_0;
    viRetrace = (*__osViGetCurrentContext()).retraceCount;
    if viRetrace as libc::c_int == 0 as libc::c_int {
        viRetrace = 1 as libc::c_int as u16_0
    }
    args = vargs as *mut OSMgrArgs;
    loop  {
        osRecvMesg((*args).eventQueue,
                   &mut mesg as *mut *mut OSIoMesg as OSMesg as *mut OSMesg,
                   1 as libc::c_int);
        match (*mesg).hdr.type_0 as libc::c_int {
            13 => {
                __osViSwapContext();
                viRetrace = viRetrace.wrapping_sub(1);
                if viRetrace == 0 {
                    let mut ctx: *mut OSViContext = __osViGetCurrentContext();
                    if !(*ctx).mq.is_null() {
                        osSendMesg((*ctx).mq, (*ctx).msg as OSMesg,
                                   0 as libc::c_int);
                    }
                    viRetrace = (*ctx).retraceCount
                }
                __osViIntrCount = __osViIntrCount.wrapping_add(1);
                // block optimized out since temp is always 0,
                // but it changes register allocation and ordering for __osCurrentTime
                if temp != 0 as libc::c_int as libc::c_uint {
                    addTime = osGetCount();
                    __osCurrentTime = addTime as OSTime;
                    temp = 0 as libc::c_int as u32_0
                }
                addTime = __osBaseCounter;
                __osBaseCounter = osGetCount();
                addTime = __osBaseCounter.wrapping_sub(addTime);
                __osCurrentTime =
                    __osCurrentTime.wrapping_add(addTime as libc::c_ulonglong)
            }
            14 => { __osTimerInterrupt(); }
            _ => { }
        }
    };
}
