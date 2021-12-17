#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case,
         non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(const_raw_ptr_to_usize_cast, register_tool)]
extern "C" {
    #[no_mangle]
    fn __osPiCreateAccessQueue();
    #[no_mangle]
    fn osCreateThread(thread: *mut OSThread, id: OSId,
                      entry:
                          Option<unsafe extern "C" fn(_: *mut libc::c_void)
                                     -> ()>, arg: *mut libc::c_void,
                      sp: *mut libc::c_void, pri: OSPri);
    #[no_mangle]
    fn __osDevMgrMain(arg: *mut libc::c_void);
    #[no_mangle]
    fn __osPiRawStartDma(dir: s32, cartAddr: u32_0,
                         dramAddr: *mut libc::c_void, size: size_t) -> s32;
    #[no_mangle]
    fn osSetEventMesg(e: OSEvent, mq: *mut OSMesgQueue, msg: OSMesg);
    #[no_mangle]
    fn osCreateMesgQueue(mq: *mut OSMesgQueue, msg: *mut OSMesg, count: s32);
    #[no_mangle]
    fn osSetThreadPri(thread: *mut OSThread, pri: OSPri);
    #[no_mangle]
    fn osGetThreadPri(thread: *mut OSThread) -> OSPri;
    #[no_mangle]
    fn __osEPiRawStartDma(handle: *mut OSPiHandle, direction: s32,
                          cartAddr: u32_0, dramAddr: *mut libc::c_void,
                          size: size_t) -> s32;
    #[no_mangle]
    fn __osDisableInt() -> s32;
    #[no_mangle]
    fn __osRestoreInt(_: s32);
    #[no_mangle]
    fn osStartThread(thread: *mut OSThread);
    #[no_mangle]
    static mut __osPiAccessQueueEnabled: u32_0;
    #[no_mangle]
    static mut __osPiAccessQueue: OSMesgQueue;
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
pub static mut __osPiDevMgr: OSMgrArgs =
    {
        let mut init =
            OSMgrArgs{initialized: 0 as libc::c_int as u32_0,
                      mgrThread: 0 as *const OSThread as *mut OSThread,
                      cmdQueue: 0 as *const OSMesgQueue as *mut OSMesgQueue,
                      eventQueue: 0 as *const OSMesgQueue as *mut OSMesgQueue,
                      acccessQueue:
                          0 as *const OSMesgQueue as *mut OSMesgQueue,
                      piDmaCallback: None,
                      epiDmaCallback: None,};
        init
    };
#[no_mangle]
pub static mut __Dom1SpeedParam: OSPiHandle =
    OSPiHandle{next: 0 as *const OSPiHandle as *mut OSPiHandle,
               type_0: 0,
               latency: 0,
               pageSize: 0,
               relDuration: 0,
               pulse: 0,
               domain: 0,
               baseAddress: 0,
               speed: 0,
               transferInfo:
                   __OSTranxInfo{cmdType: 0,
                                 transferMode: 0,
                                 blockNum: 0,
                                 sectorNum: 0,
                                 devAddr: 0,
                                 bmCtlShadow: 0,
                                 seqCtlShadow: 0,
                                 block:
                                     [__OSBlockInfo{errStatus: 0,
                                                    dramAddr:
                                                        0 as
                                                            *const libc::c_void
                                                            as
                                                            *mut libc::c_void,
                                                    C2Addr:
                                                        0 as
                                                            *const libc::c_void
                                                            as
                                                            *mut libc::c_void,
                                                    sectorSize: 0,
                                                    C1ErrNum: 0,
                                                    C1ErrSector: [0; 4],};
                                         2],},};
#[no_mangle]
pub static mut __Dom2SpeedParam: OSPiHandle =
    OSPiHandle{next: 0 as *const OSPiHandle as *mut OSPiHandle,
               type_0: 0,
               latency: 0,
               pageSize: 0,
               relDuration: 0,
               pulse: 0,
               domain: 0,
               baseAddress: 0,
               speed: 0,
               transferInfo:
                   __OSTranxInfo{cmdType: 0,
                                 transferMode: 0,
                                 blockNum: 0,
                                 sectorNum: 0,
                                 devAddr: 0,
                                 bmCtlShadow: 0,
                                 seqCtlShadow: 0,
                                 block:
                                     [__OSBlockInfo{errStatus: 0,
                                                    dramAddr:
                                                        0 as
                                                            *const libc::c_void
                                                            as
                                                            *mut libc::c_void,
                                                    C2Addr:
                                                        0 as
                                                            *const libc::c_void
                                                            as
                                                            *mut libc::c_void,
                                                    sectorSize: 0,
                                                    C1ErrNum: 0,
                                                    C1ErrSector: [0; 4],};
                                         2],},};
#[no_mangle]
pub static mut piThread: OSThread =
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
pub static mut piStackThread: [u8_0; 4096] = [0; 4096];
#[no_mangle]
pub static mut piEventQueue: OSMesgQueue =
    OSMesgQueue{mtqueue: 0 as *const OSThread as *mut OSThread,
                fullqueue: 0 as *const OSThread as *mut OSThread,
                validCount: 0,
                first: 0,
                msgCount: 0,
                msg: 0 as *const OSMesg as *mut OSMesg,};
#[no_mangle]
pub static mut piEventBuf: [OSMesg; 2] =
    [0 as *const libc::c_void as *mut libc::c_void; 2];
#[no_mangle]
pub static mut __osThreadSave: OSThread =
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
pub static mut __osPiTable: *mut OSPiHandle =
    0 as *const OSPiHandle as *mut OSPiHandle;
#[no_mangle]
pub static mut __osCurrentHandle: [*mut OSPiHandle; 2] =
    unsafe {
        [&__Dom1SpeedParam as *const OSPiHandle as *mut OSPiHandle,
         &__Dom2SpeedParam as *const OSPiHandle as *mut OSPiHandle]
    };
#[no_mangle]
pub unsafe extern "C" fn osCreatePiManager(mut pri: OSPri,
                                           mut cmdQ: *mut OSMesgQueue,
                                           mut cmdBuf: *mut OSMesg,
                                           mut cmdMsgCnt: s32) {
    let mut prevInt: u32_0 = 0;
    let mut newPri: OSPri = 0;
    let mut currentPri: OSPri = 0;
    if __osPiDevMgr.initialized == 0 {
        osCreateMesgQueue(cmdQ, cmdBuf, cmdMsgCnt);
        osCreateMesgQueue(&mut piEventQueue, piEventBuf.as_mut_ptr(),
                          1 as libc::c_int);
        if __osPiAccessQueueEnabled == 0 { __osPiCreateAccessQueue(); }
        osSetEventMesg(8 as libc::c_int as OSEvent, &mut piEventQueue,
                       0x22222222 as libc::c_int as OSMesg);
        newPri = -(1 as libc::c_int);
        currentPri = osGetThreadPri(0 as *mut OSThread);
        if currentPri < pri {
            newPri = currentPri;
            osSetThreadPri(0 as *mut OSThread, pri);
        }
        prevInt = __osDisableInt() as u32_0;
        __osPiDevMgr.initialized = 1 as libc::c_int as u32_0;
        __osPiDevMgr.cmdQueue = cmdQ;
        __osPiDevMgr.mgrThread = &mut piThread;
        __osPiDevMgr.eventQueue = &mut piEventQueue;
        __osPiDevMgr.acccessQueue = &mut __osPiAccessQueue;
        __osPiDevMgr.piDmaCallback =
            Some(__osPiRawStartDma as
                     unsafe extern "C" fn(_: s32, _: u32_0,
                                          _: *mut libc::c_void, _: size_t)
                         -> s32);
        __osPiDevMgr.epiDmaCallback =
            Some(__osEPiRawStartDma as
                     unsafe extern "C" fn(_: *mut OSPiHandle, _: s32,
                                          _: u32_0, _: *mut libc::c_void,
                                          _: size_t) -> s32);
        osCreateThread(&mut piThread, 0 as libc::c_int,
                       Some(__osDevMgrMain as
                                unsafe extern "C" fn(_: *mut libc::c_void)
                                    -> ()),
                       &mut __osPiDevMgr as *mut OSMgrArgs as
                           *mut libc::c_void,
                       piStackThread.as_mut_ptr().offset(::std::mem::size_of::<[u8_0; 4096]>()
                                                             as libc::c_ulong
                                                             as isize) as
                           *mut libc::c_void, pri);
        osStartThread(&mut piThread);
        __osRestoreInt(prevInt as s32);
        if newPri != -(1 as libc::c_int) {
            osSetThreadPri(0 as *mut OSThread, newPri);
        }
    };
}
