#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case,
         non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(register_tool)]
extern "C" {
    #[no_mangle]
    fn bzero(__s: *mut libc::c_void, __n: u32_0);
    #[no_mangle]
    fn __osPfsRWInode(pfs: *mut OSPfs, inode: *mut __OSInode, flag: u8_0,
                      bank: u8_0) -> s32;
    #[no_mangle]
    fn osPfsFindFile(pfs: *mut OSPfs, companyCode: u16_0, gameCode: u32_0,
                     gameName: *mut u8_0, extName: *mut u8_0,
                     fileNo: *mut s32) -> s32;
    #[no_mangle]
    fn __osContRamWrite(mq: *mut OSMesgQueue, channel: s32, address: u16_0,
                        buffer: *mut u8_0, force: s32) -> s32;
    #[no_mangle]
    fn __osContRamRead(ctrlrqueue: *mut OSMesgQueue, channel: s32,
                       addr: u16_0, data: *mut u8_0) -> s32;
    #[no_mangle]
    fn __osPfsSelectBank(pfs: *mut OSPfs, bank: u8_0) -> s32;
}
pub type s8 = libc::c_schar;
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
pub union __OSInodeUnit {
    pub inode_t: C2RustUnnamed_0,
    pub ipage: u16_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_0 {
    pub bank: u8_0,
    pub page: u8_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __OSInode {
    pub inodePage: [__OSInodeUnit; 128],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __OSDir {
    pub game_code: u32_0,
    pub company_code: u16_0,
    pub start_page: __OSInodeUnit,
    pub status: u8_0,
    pub reserved: s8,
    pub data_sum: u16_0,
    pub ext_name: [u8_0; 4],
    pub game_name: [u8_0; 16],
}
#[no_mangle]
pub unsafe extern "C" fn osPfsDeleteFile(mut pfs: *mut OSPfs,
                                         mut companyCode: u16_0,
                                         mut gameCode: u32_0,
                                         mut gameName: *mut u8_0,
                                         mut extName: *mut u8_0) -> s32 {
    let mut file_no: s32 = 0;
    let mut ret: s32 = 0;
    let mut inode: __OSInode =
        __OSInode{inodePage:
                      [__OSInodeUnit{inode_t:
                                         C2RustUnnamed_0{bank: 0, page: 0,},};
                          128],};
    let mut dir: __OSDir =
        __OSDir{game_code: 0,
                company_code: 0,
                start_page:
                    __OSInodeUnit{inode_t:
                                      C2RustUnnamed_0{bank: 0, page: 0,},},
                status: 0,
                reserved: 0,
                data_sum: 0,
                ext_name: [0; 4],
                game_name: [0; 16],};
    let mut last_page: __OSInodeUnit =
        __OSInodeUnit{inode_t: C2RustUnnamed_0{bank: 0, page: 0,},};
    let mut startpage: u8_0 = 0;
    let mut bank: u8_0 = 0;
    if companyCode as libc::c_int == 0 as libc::c_int ||
           gameCode == 0 as libc::c_int as libc::c_uint {
        return 5 as libc::c_int
    }
    ret =
        osPfsFindFile(pfs, companyCode, gameCode, gameName, extName,
                      &mut file_no);
    if ret != 0 as libc::c_int { return ret }
    if (*pfs).activebank as libc::c_int != 0 as libc::c_int &&
           {
               ret = __osPfsSelectBank(pfs, 0 as libc::c_int as u8_0);
               (ret) != 0 as libc::c_int
           } {
        return ret
    }
    ret =
        __osContRamRead((*pfs).queue, (*pfs).channel,
                        ((*pfs).dir_table + file_no) as u16_0,
                        &mut dir as *mut __OSDir as *mut u8_0);
    if ret != 0 as libc::c_int { return ret }
    startpage = dir.start_page.inode_t.page;
    bank = dir.start_page.inode_t.bank;
    while (bank as libc::c_int) < (*pfs).banks as libc::c_int {
        ret = __osPfsRWInode(pfs, &mut inode, 0 as libc::c_int as u8_0, bank);
        if ret != 0 as libc::c_int { return ret }
        ret =
            __osPfsReleasePages(pfs, &mut inode, startpage, bank,
                                &mut last_page);
        if ret != 0 as libc::c_int { return ret }
        ret = __osPfsRWInode(pfs, &mut inode, 1 as libc::c_int as u8_0, bank);
        if ret != 0 as libc::c_int { return ret }
        if last_page.ipage as libc::c_int == 1 as libc::c_int { break ; }
        bank = last_page.inode_t.bank;
        startpage = last_page.inode_t.page
    }
    if bank as libc::c_int >= (*pfs).banks as libc::c_int {
        return 3 as libc::c_int
    }
    bzero(&mut dir as *mut __OSDir as *mut libc::c_void,
          ::std::mem::size_of::<__OSDir>() as libc::c_ulong);
    ret =
        __osContRamWrite((*pfs).queue, (*pfs).channel,
                         ((*pfs).dir_table + file_no) as u16_0,
                         &mut dir as *mut __OSDir as *mut u8_0,
                         0 as libc::c_int);
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn __osPfsReleasePages(mut pfs: *mut OSPfs,
                                             mut inode: *mut __OSInode,
                                             mut initialPage: u8_0,
                                             mut bank: u8_0,
                                             mut finalPage:
                                                 *mut __OSInodeUnit) -> s32 {
    let mut next: __OSInodeUnit =
        __OSInodeUnit{inode_t: C2RustUnnamed_0{bank: 0, page: 0,},};
    let mut prev: __OSInodeUnit =
        __OSInodeUnit{inode_t: C2RustUnnamed_0{bank: 0, page: 0,},};
    let mut ret: s32 = 0 as libc::c_int;
    next.ipage =
        (((bank as libc::c_int) << 8 as libc::c_int) +
             initialPage as libc::c_int) as u16_0;
    loop  {
        prev = next;
        next = (*inode).inodePage[next.inode_t.page as usize];
        (*inode).inodePage[prev.inode_t.page as usize].ipage =
            3 as libc::c_int as u16_0;
        if !(next.ipage as libc::c_int >= (*pfs).inodeStartPage &&
                 next.inode_t.bank as libc::c_int == bank as libc::c_int) {
            break ;
        }
    }
    *finalPage = next;
    return ret;
}
