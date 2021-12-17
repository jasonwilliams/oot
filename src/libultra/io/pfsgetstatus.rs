#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case,
         non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(register_tool)]
extern "C" {
    #[no_mangle]
    fn osRecvMesg(mq: *mut OSMesgQueue, msg: *mut OSMesg, flag: s32) -> s32;
    #[no_mangle]
    fn __osSiRawStartDma(dir: s32, addr: *mut libc::c_void) -> s32;
    #[no_mangle]
    static mut __osContLastPoll: u8_0;
    #[no_mangle]
    static mut __osPfsInodeCacheBank: u8_0;
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
pub struct __OSContRequestHeaderAligned {
    pub txsize: u8_0,
    pub rxsize: u8_0,
    pub poll: u8_0,
    pub typeh: u8_0,
    pub typel: u8_0,
    pub status: u8_0,
}
#[no_mangle]
pub static mut gPifMempakBuf: OSPifRam = OSPifRam{ram: [0; 15], status: 0,};
#[no_mangle]
pub unsafe extern "C" fn __osPfsGetStatus(mut queue: *mut OSMesgQueue,
                                          mut channel: s32) -> s32 {
    let mut ret: s32 = 0 as libc::c_int;
    let mut msg: OSMesg = 0 as *mut libc::c_void;
    let mut data: OSContStatus =
        OSContStatus{type_0: 0, status: 0, errno: 0,};
    __osPfsInodeCacheBank = 250 as libc::c_int as u8_0;
    __osPfsRequestOneChannel(channel, 0 as libc::c_int as u8_0);
    ret =
        __osSiRawStartDma(1 as libc::c_int,
                          &mut gPifMempakBuf as *mut OSPifRam as
                              *mut libc::c_void);
    osRecvMesg(queue, &mut msg, 1 as libc::c_int);
    ret =
        __osSiRawStartDma(0 as libc::c_int,
                          &mut gPifMempakBuf as *mut OSPifRam as
                              *mut libc::c_void);
    osRecvMesg(queue, &mut msg, 1 as libc::c_int);
    __osPfsGetOneChannelData(channel, &mut data);
    if data.status as libc::c_int & 0x1 as libc::c_int != 0 as libc::c_int &&
           data.status as libc::c_int & 0x2 as libc::c_int != 0 as libc::c_int
       {
        return 2 as libc::c_int
    } else {
        if data.errno as libc::c_int != 0 ||
               data.status as libc::c_int & 0x1 as libc::c_int ==
                   0 as libc::c_int {
            return 1 as libc::c_int
        } else {
            if data.status as libc::c_int & 0x4 as libc::c_int !=
                   0 as libc::c_int {
                return 0x4 as libc::c_int
            }
        }
    }
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn __osPfsRequestOneChannel(mut channel: s32,
                                                  mut poll: u8_0) {
    let mut bufptr: *mut u8_0 = 0 as *mut u8_0;
    let mut req: __OSContRequestHeaderAligned =
        __OSContRequestHeaderAligned{txsize: 0,
                                     rxsize: 0,
                                     poll: 0,
                                     typeh: 0,
                                     typel: 0,
                                     status: 0,};
    let mut idx: s32 = 0;
    __osContLastPoll = 0xfe as libc::c_int as u8_0;
    gPifMempakBuf.status = 1 as libc::c_int as u32_0;
    bufptr = &mut gPifMempakBuf as *mut OSPifRam as *mut u8_0;
    req.txsize = 1 as libc::c_int as u8_0;
    req.rxsize = 3 as libc::c_int as u8_0;
    req.poll = poll;
    req.typeh = 0xff as libc::c_int as u8_0;
    req.typel = 0xff as libc::c_int as u8_0;
    req.status = 0xff as libc::c_int as u8_0;
    idx = 0 as libc::c_int;
    while idx < channel {
        let fresh0 = bufptr;
        bufptr = bufptr.offset(1);
        *fresh0 = 0 as libc::c_int as u8_0;
        idx += 1
    }
    *(bufptr as *mut __OSContRequestHeaderAligned) = req;
    bufptr =
        bufptr.offset(::std::mem::size_of::<__OSContRequestHeaderAligned>() as
                          libc::c_ulong as isize);
    *bufptr = 0xfe as libc::c_int as u8_0;
}
#[no_mangle]
pub unsafe extern "C" fn __osPfsGetOneChannelData(mut channel: s32,
                                                  mut contData:
                                                      *mut OSContStatus) {
    let mut bufptr: *mut u8_0 =
        &mut gPifMempakBuf as *mut OSPifRam as *mut u8_0;
    let mut req: __OSContRequestHeaderAligned =
        __OSContRequestHeaderAligned{txsize: 0,
                                     rxsize: 0,
                                     poll: 0,
                                     typeh: 0,
                                     typel: 0,
                                     status: 0,};
    let mut idx: s32 = 0;
    idx = 0 as libc::c_int;
    while idx < channel { bufptr = bufptr.offset(1); idx += 1 }
    req = *(bufptr as *mut __OSContRequestHeaderAligned);
    (*contData).errno =
        ((req.rxsize as libc::c_int & 0xc0 as libc::c_int) >>
             4 as libc::c_int) as u8_0;
    if (*contData).errno != 0 { return }
    (*contData).type_0 =
        ((req.typel as libc::c_int) << 8 as libc::c_int |
             req.typeh as libc::c_int) as u16_0;
    (*contData).status = req.status;
}
