#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case,
         non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(register_tool)]
extern "C" {
    #[no_mangle]
    fn osSendMesg(mq: *mut OSMesgQueue, mesg: OSMesg, flag: s32) -> s32;
    #[no_mangle]
    fn osRecvMesg(mq: *mut OSMesgQueue, msg: *mut OSMesg, flag: s32) -> s32;
    #[no_mangle]
    fn __osEPiRawReadIo(handle: *mut OSPiHandle, devAddr: u32_0,
                        data: *mut u32_0) -> s32;
    #[no_mangle]
    fn osYieldThread();
    #[no_mangle]
    fn __osEPiRawWriteIo(handle: *mut OSPiHandle, devAddr: u32_0, data: u32_0)
     -> s32;
    #[no_mangle]
    fn __osResetGlobalIntMask(mask: OSHWIntr);
    #[no_mangle]
    fn __osSetGlobalIntMask(mask: OSHWIntr);
}
pub type u8_0 = libc::c_uchar;
pub type u16_0 = libc::c_ushort;
pub type s32 = libc::c_int;
pub type u32_0 = libc::c_uint;
pub type u64_0 = libc::c_ulonglong;
pub type f32_0 = libc::c_float;
pub type size_t = libc::c_ulong;
pub type OSHWIntr = u32_0;
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
pub unsafe extern "C" fn __osDevMgrMain(mut arg: *mut libc::c_void) {
    let mut ioMesg: *mut OSIoMesg = 0 as *mut OSIoMesg;
    let mut sp70: OSMesg = 0 as *mut libc::c_void;
    let mut sp6C: OSMesg = 0 as *mut libc::c_void;
    let mut arg0: *mut OSMgrArgs = arg as *mut OSMgrArgs;
    let mut transfer: *mut __OSTranxInfo = 0 as *mut __OSTranxInfo;
    let mut block: *mut __OSBlockInfo = 0 as *mut __OSBlockInfo;
    let mut phi_s2: s32 = 0;
    let mut phi_s0: s32 = 0;
    let mut sp54: u32_0 = 0;
    ioMesg = 0 as *mut OSIoMesg;
    loop  {
        osRecvMesg((*arg0).cmdQueue,
                   &mut ioMesg as *mut *mut OSIoMesg as OSMesg as *mut OSMesg,
                   1 as libc::c_int);
        if !(*ioMesg).piHandle.is_null() &&
               (*(*ioMesg).piHandle).type_0 as libc::c_int == 2 as libc::c_int
               &&
               ((*(*ioMesg).piHandle).transferInfo.cmdType ==
                    0 as libc::c_int as libc::c_uint ||
                    (*(*ioMesg).piHandle).transferInfo.cmdType ==
                        1 as libc::c_int as libc::c_uint) {
            transfer = &mut (*(*ioMesg).piHandle).transferInfo;
            block =
                &mut *(*transfer).block.as_mut_ptr().offset((*transfer).blockNum
                                                                as isize) as
                    *mut __OSBlockInfo;
            (*transfer).sectorNum = -(1 as libc::c_int);
            if (*transfer).transferMode as libc::c_int != 3 as libc::c_int {
                (*block).dramAddr =
                    ((*block).dramAddr as
                         u32_0).wrapping_sub((*block).sectorSize) as
                        *mut libc::c_void
            }
            phi_s2 =
                if (*transfer).transferMode as libc::c_int == 2 as libc::c_int
                       &&
                       (*(*ioMesg).piHandle).transferInfo.cmdType ==
                           0 as libc::c_int as libc::c_uint {
                    1 as libc::c_int
                } else { 0 as libc::c_int };
            osRecvMesg((*arg0).acccessQueue, &mut sp6C, 1 as libc::c_int);
            __osResetGlobalIntMask(0x100401 as libc::c_int as OSHWIntr);
            __osEPiRawWriteIo((*ioMesg).piHandle,
                              0x5000510 as libc::c_int as u32_0,
                              (*transfer).bmCtlShadow |
                                  0x80000000 as libc::c_uint);
            loop  {
                osRecvMesg((*arg0).eventQueue, &mut sp70, 1 as libc::c_int);
                transfer = &mut (*(*ioMesg).piHandle).transferInfo;
                block =
                    &mut *(*transfer).block.as_mut_ptr().offset((*transfer).blockNum
                                                                    as isize)
                        as *mut __OSBlockInfo;
                if (*block).errStatus == 0x1d as libc::c_int as libc::c_uint {
                    __osEPiRawWriteIo((*ioMesg).piHandle,
                                      0x5000510 as libc::c_int as u32_0,
                                      (*transfer).bmCtlShadow |
                                          0x10000000 as libc::c_int as
                                              libc::c_uint);
                    __osEPiRawWriteIo((*ioMesg).piHandle,
                                      0x5000510 as libc::c_int as u32_0,
                                      (*transfer).bmCtlShadow);
                    __osEPiRawReadIo((*ioMesg).piHandle,
                                     0x5000508 as libc::c_int as u32_0,
                                     &mut sp54);
                    if sp54 & 0x2000000 as libc::c_int as libc::c_uint != 0 {
                        __osEPiRawWriteIo((*ioMesg).piHandle,
                                          0x5000510 as libc::c_int as u32_0,
                                          (*transfer).bmCtlShadow |
                                              0x1000000 as libc::c_int as
                                                  libc::c_uint);
                    }
                    (*block).errStatus = 4 as libc::c_int as u32_0;
                    ::std::ptr::write_volatile((0x4600010 as libc::c_int as
                                                    libc::c_uint |
                                                    0xa0000000 as
                                                        libc::c_uint) as
                                                   *mut u32_0,
                                               0x2 as libc::c_int as u32_0);
                    __osSetGlobalIntMask(0x100c01 as libc::c_int as OSHWIntr);
                }
                osSendMesg((*ioMesg).hdr.retQueue, ioMesg as OSMesg,
                           0 as libc::c_int);
                if phi_s2 != 1 as libc::c_int ||
                       (*(*ioMesg).piHandle).transferInfo.block[0 as
                                                                    libc::c_int
                                                                    as
                                                                    usize].errStatus
                           != 0 as libc::c_int as libc::c_uint {
                    break ;
                }
                phi_s2 = 0 as libc::c_int
            }
            osSendMesg((*arg0).acccessQueue, 0 as OSMesg, 0 as libc::c_int);
            if (*(*ioMesg).piHandle).transferInfo.blockNum as libc::c_int ==
                   1 as libc::c_int {
                osYieldThread();
            }
        } else {
            match (*ioMesg).hdr.type_0 as libc::c_int {
                11 => {
                    osRecvMesg((*arg0).acccessQueue, &mut sp6C,
                               1 as libc::c_int);
                    phi_s0 =
                        (*arg0).piDmaCallback.expect("non-null function pointer")(0
                                                                                      as
                                                                                      libc::c_int,
                                                                                  (*ioMesg).devAddr,
                                                                                  (*ioMesg).dramAddr,
                                                                                  (*ioMesg).size)
                }
                12 => {
                    osRecvMesg((*arg0).acccessQueue, &mut sp6C,
                               1 as libc::c_int);
                    phi_s0 =
                        (*arg0).piDmaCallback.expect("non-null function pointer")(1
                                                                                      as
                                                                                      libc::c_int,
                                                                                  (*ioMesg).devAddr,
                                                                                  (*ioMesg).dramAddr,
                                                                                  (*ioMesg).size)
                }
                15 => {
                    osRecvMesg((*arg0).acccessQueue, &mut sp6C,
                               1 as libc::c_int);
                    phi_s0 =
                        (*arg0).epiDmaCallback.expect("non-null function pointer")((*ioMesg).piHandle,
                                                                                   0
                                                                                       as
                                                                                       libc::c_int,
                                                                                   (*ioMesg).devAddr,
                                                                                   (*ioMesg).dramAddr,
                                                                                   (*ioMesg).size)
                }
                16 => {
                    osRecvMesg((*arg0).acccessQueue, &mut sp6C,
                               1 as libc::c_int);
                    phi_s0 =
                        (*arg0).epiDmaCallback.expect("non-null function pointer")((*ioMesg).piHandle,
                                                                                   1
                                                                                       as
                                                                                       libc::c_int,
                                                                                   (*ioMesg).devAddr,
                                                                                   (*ioMesg).dramAddr,
                                                                                   (*ioMesg).size)
                }
                10 => {
                    osSendMesg((*ioMesg).hdr.retQueue, ioMesg as OSMesg,
                               0 as libc::c_int);
                    phi_s0 = -(1 as libc::c_int)
                }
                _ => { phi_s0 = -(1 as libc::c_int) }
            }
            if phi_s0 == 0 as libc::c_int {
                osRecvMesg((*arg0).eventQueue, &mut sp70, 1 as libc::c_int);
                osSendMesg((*ioMesg).hdr.retQueue, ioMesg as OSMesg,
                           0 as libc::c_int);
                osSendMesg((*arg0).acccessQueue, 0 as *mut libc::c_void,
                           0 as libc::c_int);
            }
        }
    };
}
