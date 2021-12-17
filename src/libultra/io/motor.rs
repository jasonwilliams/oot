#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case,
         non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(register_tool)]
extern "C" {
    #[no_mangle]
    fn osRecvMesg(mq: *mut OSMesgQueue, msg: *mut OSMesg, flag: s32) -> s32;
    #[no_mangle]
    fn __osPfsSelectBank(pfs: *mut OSPfs, bank: u8_0) -> s32;
    #[no_mangle]
    fn __osContRamRead(ctrlrqueue: *mut OSMesgQueue, channel: s32,
                       addr: u16_0, data: *mut u8_0) -> s32;
    #[no_mangle]
    fn __osContAddressCrc(addr: u16_0) -> u8_0;
    #[no_mangle]
    fn __osSiGetAccess();
    #[no_mangle]
    static mut __osContLastPoll: u8_0;
    #[no_mangle]
    fn __osSiRawStartDma(dir: s32, addr: *mut libc::c_void) -> s32;
    #[no_mangle]
    fn __osSiRelAccess();
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct OSPfs {
    pub status: s32,
    pub queue: *mut OSMesgQueue,
    pub channel: s32,
    pub id: [u8_0; 32],
    pub label: [u8_0; 32],
    pub version: s32,
    pub dir_size: s32,
    pub inode_table: s32,
    pub minode_table: s32,
    pub dir_table: s32,
    pub inodeStartPage: s32,
    pub banks: u8_0,
    pub activebank: u8_0,
}
#[no_mangle]
pub static mut osPifBuffers: [OSPifRam; 4] =
    [OSPifRam{ram: [0; 15], status: 0,}; 4];
#[no_mangle]
pub unsafe extern "C" fn __osMotorAccess(mut pfs: *mut OSPfs,
                                         mut vibrate: u32_0) -> s32 {
    let mut i: s32 = 0; // write mempak
    let mut ret: s32 = 0;
    let mut buf: *mut u8_0 =
        &mut *osPifBuffers.as_mut_ptr().offset((*pfs).channel as isize) as
            *mut OSPifRam as *mut u8_0;
    if (*pfs).status & 8 as libc::c_int == 0 { return 5 as libc::c_int }
    __osSiGetAccess();
    osPifBuffers[(*pfs).channel as usize].status = 1 as libc::c_int as u32_0;
    buf = buf.offset((*pfs).channel as isize);
    i = 0 as libc::c_int;
    while i < 32 as libc::c_int {
        (*(buf as *mut __OSContRamHeader)).data[i as usize] = vibrate as u8_0;
        i += 1
    }
    __osContLastPoll = 0xfe as libc::c_int as u8_0;
    __osSiRawStartDma(1 as libc::c_int,
                      &mut *osPifBuffers.as_mut_ptr().offset((*pfs).channel as
                                                                 isize) as
                          *mut OSPifRam as *mut libc::c_void);
    osRecvMesg((*pfs).queue, 0 as *mut OSMesg, 1 as libc::c_int);
    __osSiRawStartDma(0 as libc::c_int,
                      &mut *osPifBuffers.as_mut_ptr().offset((*pfs).channel as
                                                                 isize) as
                          *mut OSPifRam as *mut libc::c_void);
    osRecvMesg((*pfs).queue, 0 as *mut OSMesg, 1 as libc::c_int);
    ret =
        (*(buf as *mut __OSContRamHeader)).rxsize as libc::c_int &
            0xc0 as libc::c_int;
    if ret == 0 {
        if vibrate == 0 {
            if (*(buf as *mut __OSContRamHeader)).datacrc as libc::c_int !=
                   0 as libc::c_int {
                ret = 0x4 as libc::c_int
            }
        } else if (*(buf as *mut __OSContRamHeader)).datacrc as libc::c_int !=
                      0xeb as libc::c_int {
            ret = 0x4 as libc::c_int
        }
    }
    __osSiRelAccess();
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn _MakeMotorData(mut channel: s32,
                                        mut buf: *mut OSPifRam) {
    let mut bufptr: *mut u8_0 = buf as *mut u8_0;
    let mut mempakwr: __OSContRamHeader =
        __OSContRamHeader{unk_00: 0,
                          txsize: 0,
                          rxsize: 0,
                          poll: 0,
                          hi: 0,
                          lo: 0,
                          data: [0; 32],
                          datacrc: 0,};
    let mut i: s32 = 0;
    mempakwr.unk_00 = 0xff as libc::c_int as u8_0;
    mempakwr.txsize = 0x23 as libc::c_int as u8_0;
    mempakwr.rxsize = 1 as libc::c_int as u8_0;
    mempakwr.poll = 3 as libc::c_int as u8_0;
    mempakwr.hi = (0x600 as libc::c_int >> 3 as libc::c_int) as u8_0;
    mempakwr.lo =
        (__osContAddressCrc(0x600 as libc::c_int as u16_0) as libc::c_int |
             (0x600 as libc::c_int) << 5 as libc::c_int) as u8_0;
    if channel != 0 as libc::c_int {
        i = 0 as libc::c_int;
        while i < channel {
            let fresh0 = bufptr;
            bufptr = bufptr.offset(1);
            *fresh0 = 0 as libc::c_int as u8_0;
            i += 1
        }
    }
    *(bufptr as *mut __OSContRamHeader) = mempakwr;
    bufptr =
        bufptr.offset(::std::mem::size_of::<__OSContRamHeader>() as
                          libc::c_ulong as isize);
    *bufptr = 0xfe as libc::c_int as u8_0;
}
#[no_mangle]
pub unsafe extern "C" fn osMotorInit(mut ctrlrqueue: *mut OSMesgQueue,
                                     mut pfs: *mut OSPfs, mut channel: s32)
 -> s32 {
    let mut ret: s32 = 0;
    let mut sp24: [u8_0; 32] = [0; 32];
    (*pfs).queue = ctrlrqueue;
    (*pfs).channel = channel;
    (*pfs).activebank = 0xff as libc::c_int as u8_0;
    (*pfs).status = 0 as libc::c_int;
    ret = __osPfsSelectBank(pfs, 0xfe as libc::c_int as u8_0);
    if ret == 2 as libc::c_int {
        ret = __osPfsSelectBank(pfs, 0x80 as libc::c_int as u8_0)
    }
    if ret != 0 as libc::c_int { return ret }
    ret =
        __osContRamRead(ctrlrqueue, channel, 0x400 as libc::c_int as u16_0,
                        sp24.as_mut_ptr());
    if ret == 2 as libc::c_int {
        ret = 4 as libc::c_int
        // "Controller pack communication error"
    }
    if ret != 0 as libc::c_int { return ret }
    if sp24[(32 as libc::c_int - 1 as libc::c_int) as usize] as libc::c_int ==
           0xfe as libc::c_int {
        return 0xb as libc::c_int
    }
    ret = __osPfsSelectBank(pfs, 0x80 as libc::c_int as u8_0);
    if ret == 2 as libc::c_int {
        ret = 4 as libc::c_int
        // "Controller pack communication error"
    }
    if ret != 0 as libc::c_int { return ret }
    ret =
        __osContRamRead(ctrlrqueue, channel, 0x400 as libc::c_int as u16_0,
                        sp24.as_mut_ptr());
    if ret == 2 as libc::c_int {
        ret = 4 as libc::c_int
        // "Controller pack communication error"
    }
    if ret != 0 as libc::c_int { return ret }
    if sp24[(32 as libc::c_int - 1 as libc::c_int) as usize] as libc::c_int !=
           0x80 as libc::c_int {
        return 0xb as libc::c_int
    }
    if (*pfs).status & 0x8 as libc::c_int == 0 as libc::c_int {
        _MakeMotorData(channel,
                       &mut *osPifBuffers.as_mut_ptr().offset(channel as
                                                                  isize));
    }
    (*pfs).status = 0x8 as libc::c_int;
    return 0 as libc::c_int;
    // "Recognized rumble pak"
}
