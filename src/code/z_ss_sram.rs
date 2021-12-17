#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case,
         non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(const_raw_ptr_to_usize_cast, register_tool)]
extern "C" {
    #[no_mangle]
    fn osSyncPrintf(fmt: *const libc::c_char, _: ...);
    #[no_mangle]
    fn osRecvMesg(mq: *mut OSMesgQueue, msg: *mut OSMesg, flag: s32) -> s32;
    #[no_mangle]
    fn bzero(__s: *mut libc::c_void, __n: u32_0);
    #[no_mangle]
    fn osWritebackDCache(vaddr: *mut libc::c_void, nbytes: s32);
    #[no_mangle]
    fn osEPiStartDma(handle: *mut OSPiHandle, mb: *mut OSIoMesg,
                     direction: s32) -> s32;
    #[no_mangle]
    fn osCreateMesgQueue(mq: *mut OSMesgQueue, msg: *mut OSMesg, count: s32);
    #[no_mangle]
    fn osInvalDCache(vaddr: *mut libc::c_void, nbytes: s32);
    #[no_mangle]
    fn __osDisableInt() -> s32;
    #[no_mangle]
    fn __osRestoreInt(_: s32);
    #[no_mangle]
    static mut __osPiTable: *mut OSPiHandle;
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
pub struct SsSramContext {
    pub piHandle: OSPiHandle,
    pub ioMesg: OSIoMesg,
    pub mesgQ: OSMesgQueue,
}
// size = 0xA4
#[no_mangle]
pub static mut sSsSramContext: SsSramContext =
    {
        let mut init =
            SsSramContext{piHandle:
                              {
                                  let mut init =
                                      OSPiHandle{next:
                                                     0 as *const OSPiHandle as
                                                         *mut OSPiHandle,
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
                                                                   transferMode:
                                                                       0,
                                                                   blockNum:
                                                                       0,
                                                                   sectorNum:
                                                                       0,
                                                                   devAddr: 0,
                                                                   bmCtlShadow:
                                                                       0,
                                                                   seqCtlShadow:
                                                                       0,
                                                                   block:
                                                                       [__OSBlockInfo{errStatus:
                                                                                          0,
                                                                                      dramAddr:
                                                                                          0
                                                                                              as
                                                                                              *const libc::c_void
                                                                                              as
                                                                                              *mut libc::c_void,
                                                                                      C2Addr:
                                                                                          0
                                                                                              as
                                                                                              *const libc::c_void
                                                                                              as
                                                                                              *mut libc::c_void,
                                                                                      sectorSize:
                                                                                          0,
                                                                                      C1ErrNum:
                                                                                          0,
                                                                                      C1ErrSector:
                                                                                          [0;
                                                                                              4],};
                                                                           2],},};
                                  init
                              },
                          ioMesg:
                              OSIoMesg{hdr:
                                           OSIoMesgHdr{type_0: 0,
                                                       pri: 0,
                                                       status: 0,
                                                       retQueue:
                                                           0 as
                                                               *const OSMesgQueue
                                                               as
                                                               *mut OSMesgQueue,},
                                       dramAddr:
                                           0 as *const libc::c_void as
                                               *mut libc::c_void,
                                       devAddr: 0,
                                       size: 0,
                                       piHandle:
                                           0 as *const OSPiHandle as
                                               *mut OSPiHandle,},
                          mesgQ:
                              OSMesgQueue{mtqueue:
                                              0 as *const OSThread as
                                                  *mut OSThread,
                                          fullqueue:
                                              0 as *const OSThread as
                                                  *mut OSThread,
                                          validCount: 0,
                                          first: 0,
                                          msgCount: 0,
                                          msg:
                                              0 as *const OSMesg as
                                                  *mut OSMesg,},};
        init
    };
#[no_mangle]
pub unsafe extern "C" fn SsSram_Init(mut addr: u32_0, mut handleType: u8_0,
                                     mut handleDomain: u8_0,
                                     mut handleLatency: u8_0,
                                     mut handlePageSize: u8_0,
                                     mut handleRelDuration: u8_0,
                                     mut handlePulse: u8_0,
                                     mut handleSpeed: u32_0) {
    let mut prevInt: u32_0 = 0;
    let mut handle: *mut OSPiHandle = &mut sSsSramContext.piHandle;
    if addr.wrapping_add(0xa0000000 as libc::c_uint) as *mut libc::c_void as
           u32_0 != (*handle).baseAddress {
        sSsSramContext.piHandle.type_0 = handleType;
        (*handle).baseAddress =
            addr.wrapping_add(0xa0000000 as libc::c_uint) as *mut libc::c_void
                as u32_0;
        sSsSramContext.piHandle.latency = handleLatency;
        sSsSramContext.piHandle.pulse = handlePulse;
        sSsSramContext.piHandle.pageSize = handlePageSize;
        sSsSramContext.piHandle.relDuration = handleRelDuration;
        sSsSramContext.piHandle.domain = handleDomain;
        sSsSramContext.piHandle.speed = handleSpeed;
        bzero(&mut sSsSramContext.piHandle.transferInfo as *mut __OSTranxInfo
                  as *mut libc::c_void,
              ::std::mem::size_of::<__OSTranxInfo>() as libc::c_ulong);
        prevInt = __osDisableInt() as u32_0;
        sSsSramContext.piHandle.next = __osPiTable;
        __osPiTable = &mut sSsSramContext.piHandle;
        __osRestoreInt(prevInt as s32);
        sSsSramContext.ioMesg.hdr.pri = 0 as libc::c_int as u8_0;
        sSsSramContext.ioMesg.hdr.retQueue = &mut sSsSramContext.mesgQ;
        sSsSramContext.ioMesg.devAddr = addr
    };
}
#[no_mangle]
pub unsafe extern "C" fn SsSram_Dma(mut dramAddr: *mut libc::c_void,
                                    mut size: size_t, mut direction: s32) {
    let mut mesg: OSMesg = 0 as *mut libc::c_void;
    osCreateMesgQueue(&mut sSsSramContext.mesgQ, &mut mesg, 1 as libc::c_int);
    sSsSramContext.ioMesg.dramAddr = dramAddr;
    sSsSramContext.ioMesg.size = size;
    osWritebackDCache(dramAddr, size as s32);
    osEPiStartDma(&mut sSsSramContext.piHandle, &mut sSsSramContext.ioMesg,
                  direction);
    osRecvMesg(&mut sSsSramContext.mesgQ, &mut mesg, 1 as libc::c_int);
    osInvalDCache(dramAddr, size as s32);
}
#[no_mangle]
pub unsafe extern "C" fn SsSram_ReadWrite(mut addr: u32_0,
                                          mut dramAddr: *mut libc::c_void,
                                          mut size: size_t,
                                          mut direction: s32) {
    osSyncPrintf(b"ssSRAMReadWrite:%08x %08x %08x %d\n\x00" as *const u8 as
                     *const libc::c_char, addr, dramAddr, size, direction);
    SsSram_Init(addr, 3 as libc::c_int as u8_0, 1 as libc::c_int as u8_0,
                5 as libc::c_int as u8_0, 0xd as libc::c_int as u8_0,
                2 as libc::c_int as u8_0, 0xc as libc::c_int as u8_0,
                0 as libc::c_int as u32_0);
    SsSram_Dma(dramAddr, size, direction);
}
