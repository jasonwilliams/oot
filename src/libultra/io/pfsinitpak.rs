#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case,
         non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(register_tool)]
extern "C" {
    #[no_mangle]
    fn bcmp(__sl: *mut libc::c_void, __s2: *mut libc::c_void, __n: u32_0)
     -> u32_0;
    #[no_mangle]
    fn bcopy(__src: *mut libc::c_void, __dest: *mut libc::c_void, __n: u32_0)
     -> *mut libc::c_void;
    #[no_mangle]
    fn __osSiGetAccess();
    #[no_mangle]
    fn __osSiRelAccess();
    #[no_mangle]
    fn __osPfsGetStatus(queue: *mut OSMesgQueue, channel: s32) -> s32;
    #[no_mangle]
    fn __osIdCheckSum(ptr: *mut u16_0, csum: *mut u16_0, icsum: *mut u16_0)
     -> s32;
    #[no_mangle]
    fn __osRepairPackId(pfs: *mut OSPfs, badid: *mut __OSPackId,
                        newid: *mut __OSPackId) -> s32;
    #[no_mangle]
    fn __osCheckPackId(pfs: *mut OSPfs, check: *mut __OSPackId) -> s32;
    #[no_mangle]
    fn __osPfsSelectBank(pfs: *mut OSPfs, bank: u8_0) -> s32;
    #[no_mangle]
    fn osPfsChecker(pfs: *mut OSPfs) -> s32;
    #[no_mangle]
    fn __osContRamRead(ctrlrqueue: *mut OSMesgQueue, channel: s32,
                       addr: u16_0, data: *mut u8_0) -> s32;
    #[no_mangle]
    fn __osContRamWrite(mq: *mut OSMesgQueue, channel: s32, address: u16_0,
                        buffer: *mut u8_0, force: s32) -> s32;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __OSPackId {
    pub repaired: u32_0,
    pub random: u32_0,
    pub serialMid: u64_0,
    pub serialLow: u64_0,
    pub deviceid: u16_0,
    pub banks: u8_0,
    pub version: u8_0,
    pub checksum: u16_0,
    pub invertedChecksum: u16_0,
}
#[no_mangle]
pub unsafe extern "C" fn osPfsInitPak(mut queue: *mut OSMesgQueue,
                                      mut pfs: *mut OSPfs, mut channel: s32)
 -> s32 {
    let mut ret: s32 = 0;
    let mut sum: u16_0 = 0;
    let mut isum: u16_0 = 0;
    let mut temp: [u8_0; 32] = [0; 32];
    let mut id: *mut __OSPackId = 0 as *mut __OSPackId;
    let mut newid: __OSPackId =
        __OSPackId{repaired: 0,
                   random: 0,
                   serialMid: 0,
                   serialLow: 0,
                   deviceid: 0,
                   banks: 0,
                   version: 0,
                   checksum: 0,
                   invertedChecksum: 0,};
    __osSiGetAccess();
    ret = __osPfsGetStatus(queue, channel);
    __osSiRelAccess();
    if ret != 0 as libc::c_int { return ret }
    (*pfs).queue = queue;
    (*pfs).channel = channel;
    (*pfs).status = 0 as libc::c_int;
    ret = __osPfsCheckRamArea(pfs);
    if ret != 0 as libc::c_int { return ret }
    ret = __osPfsSelectBank(pfs, 0 as libc::c_int as u8_0);
    if ret != 0 as libc::c_int { return ret }
    ret =
        __osContRamRead((*pfs).queue, (*pfs).channel,
                        1 as libc::c_int as u16_0, temp.as_mut_ptr());
    if ret != 0 as libc::c_int { return ret }
    __osIdCheckSum(temp.as_mut_ptr() as *mut u16_0, &mut sum, &mut isum);
    id = temp.as_mut_ptr() as *mut __OSPackId;
    if (*id).checksum as libc::c_int != sum as libc::c_int ||
           (*id).invertedChecksum as libc::c_int != isum as libc::c_int {
        ret = __osCheckPackId(pfs, id);
        if ret != 0 as libc::c_int {
            (*pfs).status |= 0x4 as libc::c_int;
            return ret
        }
    }
    if (*id).deviceid as libc::c_int & 0x1 as libc::c_int == 0 as libc::c_int
       {
        ret = __osRepairPackId(pfs, id, &mut newid);
        if ret != 0 {
            if ret == 10 as libc::c_int {
                (*pfs).status |= 0x4 as libc::c_int
            }
            return ret
        }
        id = &mut newid;
        if (*id).deviceid as libc::c_int & 0x1 as libc::c_int ==
               0 as libc::c_int {
            return 11 as libc::c_int
        }
    }
    bcopy(id as *mut libc::c_void,
          (*pfs).id.as_mut_ptr() as *mut libc::c_void,
          32 as libc::c_int as u32_0);
    (*pfs).version = (*id).version as s32;
    (*pfs).banks = (*id).banks;
    (*pfs).inodeStartPage =
        1 as libc::c_int + 2 as libc::c_int +
            2 as libc::c_int * (*pfs).banks as libc::c_int;
    (*pfs).dir_size = 2 as libc::c_int * 8 as libc::c_int;
    (*pfs).inode_table = 1 as libc::c_int * 8 as libc::c_int;
    (*pfs).minode_table =
        (1 as libc::c_int + (*pfs).banks as libc::c_int) * 8 as libc::c_int;
    (*pfs).dir_table =
        (*pfs).minode_table + (*pfs).banks as libc::c_int * 8 as libc::c_int;
    ret =
        __osContRamRead((*pfs).queue, (*pfs).channel,
                        7 as libc::c_int as u16_0, (*pfs).label.as_mut_ptr());
    if ret != 0 as libc::c_int { return ret }
    ret = osPfsChecker(pfs);
    (*pfs).status |= 0x1 as libc::c_int;
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn __osPfsCheckRamArea(mut pfs: *mut OSPfs) -> s32 {
    let mut i: s32 = 0 as libc::c_int;
    let mut ret: s32 = 0 as libc::c_int;
    let mut temp1: [u8_0; 32] = [0; 32];
    let mut temp2: [u8_0; 32] = [0; 32];
    let mut saveReg: [u8_0; 32] = [0; 32];
    ret = __osPfsSelectBank(pfs, 0 as libc::c_int as u8_0);
    if ret != 0 as libc::c_int { return ret }
    ret =
        __osContRamRead((*pfs).queue, (*pfs).channel,
                        0 as libc::c_int as u16_0, saveReg.as_mut_ptr());
    if ret != 0 as libc::c_int { return ret }
    i = 0 as libc::c_int;
    while i < 32 as libc::c_int { temp1[i as usize] = i as u8_0; i += 1 }
    ret =
        __osContRamWrite((*pfs).queue, (*pfs).channel,
                         0 as libc::c_int as u16_0, temp1.as_mut_ptr(),
                         0 as libc::c_int);
    if ret != 0 as libc::c_int { return ret }
    ret =
        __osContRamRead((*pfs).queue, (*pfs).channel,
                        0 as libc::c_int as u16_0, temp2.as_mut_ptr());
    if ret != 0 as libc::c_int { return ret }
    if bcmp(temp1.as_mut_ptr() as *mut libc::c_void,
            temp2.as_mut_ptr() as *mut libc::c_void,
            32 as libc::c_int as u32_0) != 0 as libc::c_int as libc::c_uint {
        return 11 as libc::c_int
    }
    return __osContRamWrite((*pfs).queue, (*pfs).channel,
                            0 as libc::c_int as u16_0, saveReg.as_mut_ptr(),
                            0 as libc::c_int);
}
