#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case,
         non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(register_tool)]
extern "C" {
    #[no_mangle]
    fn osRecvMesg(mq: *mut OSMesgQueue, msg: *mut OSMesg, flag: s32) -> s32;
    #[no_mangle]
    fn bcopy(__src: *mut libc::c_void, __dest: *mut libc::c_void, __n: u32_0)
     -> *mut libc::c_void;
    #[no_mangle]
    fn __osSiGetAccess();
    #[no_mangle]
    fn __osSiRelAccess();
    #[no_mangle]
    fn __osSiRawStartDma(dir: s32, addr: *mut libc::c_void) -> s32;
    #[no_mangle]
    fn __osPfsGetStatus(queue: *mut OSMesgQueue, channel: s32) -> s32;
    #[no_mangle]
    static mut gPifMempakBuf: OSPifRam;
    #[no_mangle]
    fn __osContDataCrc(data: *mut u8_0) -> u8_0;
    #[no_mangle]
    fn __osContAddressCrc(addr: u16_0) -> u8_0;
    #[no_mangle]
    static mut __osPfsLastChannel: s32;
    #[no_mangle]
    static mut __osContLastPoll: u8_0;
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
pub struct __OSContRamHeader {
    pub unk_00: u8_0,
    pub txsize: u8_0,
    pub rxsize: u8_0,
    pub poll: u8_0,
    pub hi: u8_0,
    pub lo: u8_0,
    pub data: [u8_0; 32],
    pub datacrc: u8_0,
}
#[no_mangle]
pub unsafe extern "C" fn __osContRamWrite(mut mq: *mut OSMesgQueue,
                                          mut channel: s32,
                                          mut address: u16_0,
                                          mut buffer: *mut u8_0,
                                          mut force: s32) -> s32 {
    let mut ret: s32 = 0 as libc::c_int;
    let mut i: s32 = 0;
    let mut ptr: *mut u8_0 = 0 as *mut u8_0;
    let mut retry: s32 = 2 as libc::c_int;
    let mut crc: u8_0 = 0;
    if force != 1 as libc::c_int &&
           (address as libc::c_int) < 7 as libc::c_int &&
           address as libc::c_int != 0 as libc::c_int {
        return 0 as libc::c_int
    }
    __osSiGetAccess();
    loop  {
        ptr = &mut gPifMempakBuf as *mut OSPifRam as *mut u8_0;
        if __osContLastPoll as libc::c_int != 3 as libc::c_int ||
               __osPfsLastChannel != channel {
            __osContLastPoll = 3 as libc::c_int as u8_0;
            __osPfsLastChannel = channel;
            // clang-format off
            i = 0 as libc::c_int;
            while i < channel {
                let fresh0 = ptr;
                ptr = ptr.offset(1);
                *fresh0 = 0 as libc::c_int as u8_0;
                i += 1
            }
            // clang-format on
            gPifMempakBuf.status = 1 as libc::c_int as u32_0;
            (*(ptr as *mut __OSContRamHeader)).unk_00 =
                0xff as libc::c_int as u8_0;
            (*(ptr as *mut __OSContRamHeader)).txsize =
                35 as libc::c_int as u8_0;
            (*(ptr as *mut __OSContRamHeader)).rxsize =
                1 as libc::c_int as u8_0;
            (*(ptr as *mut __OSContRamHeader)).poll =
                3 as libc::c_int as u8_0;
            (*(ptr as *mut __OSContRamHeader)).datacrc =
                0xff as libc::c_int as u8_0;
            *ptr.offset(::std::mem::size_of::<__OSContRamHeader>() as
                            libc::c_ulong as isize) =
                0xfe as libc::c_int as u8_0
        } else { ptr = ptr.offset(channel as isize) }
        (*(ptr as *mut __OSContRamHeader)).hi =
            (address as libc::c_int >> 3 as libc::c_int) as u8_0;
        (*(ptr as *mut __OSContRamHeader)).lo =
            ((address as libc::c_int) << 5 as libc::c_int |
                 __osContAddressCrc(address) as libc::c_int) as u8_0;
        bcopy(buffer as *mut libc::c_void,
              (*(ptr as *mut __OSContRamHeader)).data.as_mut_ptr() as
                  *mut libc::c_void, 32 as libc::c_int as u32_0);
        ret =
            __osSiRawStartDma(1 as libc::c_int,
                              &mut gPifMempakBuf as *mut OSPifRam as
                                  *mut libc::c_void);
        crc = __osContDataCrc(buffer);
        osRecvMesg(mq, 0 as *mut libc::c_void as *mut OSMesg,
                   1 as libc::c_int);
        ret =
            __osSiRawStartDma(0 as libc::c_int,
                              &mut gPifMempakBuf as *mut OSPifRam as
                                  *mut libc::c_void);
        osRecvMesg(mq, 0 as *mut libc::c_void as *mut OSMesg,
                   1 as libc::c_int);
        ret =
            ((*(ptr as *mut __OSContRamHeader)).rxsize as libc::c_int &
                 0xc0 as libc::c_int) >> 4 as libc::c_int;
        if ret == 0 {
            if crc as libc::c_int !=
                   (*(ptr as *mut __OSContRamHeader)).datacrc as libc::c_int {
                ret = __osPfsGetStatus(mq, channel);
                if ret != 0 { break ; }
                ret = 0x4 as libc::c_int
            }
        } else { ret = 1 as libc::c_int }
        if !(ret == 0x4 as libc::c_int &&
                 {
                     let fresh1 = retry;
                     retry = retry - 1;
                     (fresh1) >= 0 as libc::c_int
                 }) {
            break ;
        }
    }
    __osSiRelAccess();
    return ret;
}
