#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case,
         non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(register_tool)]
extern "C" {
    #[no_mangle]
    fn __osPfsRWInode(pfs: *mut OSPfs, inode: *mut __OSInode, flag: u8_0,
                      bank: u8_0) -> s32;
    #[no_mangle]
    fn __osPfsGetStatus(queue: *mut OSMesgQueue, channel: s32) -> s32;
    #[no_mangle]
    fn __osContRamWrite(mq: *mut OSMesgQueue, channel: s32, address: u16_0,
                        buffer: *mut u8_0, force: s32) -> s32;
    #[no_mangle]
    fn __osPfsSelectBank(pfs: *mut OSPfs, bank: u8_0) -> s32;
    #[no_mangle]
    fn __osContRamRead(ctrlrqueue: *mut OSMesgQueue, channel: s32,
                       addr: u16_0, data: *mut u8_0) -> s32;
    #[no_mangle]
    fn __osCheckId(pfs: *mut OSPfs) -> s32;
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
pub static mut __osPfsInodeCache: __OSInode =
    __OSInode{inodePage:
                  [__OSInodeUnit{inode_t:
                                     C2RustUnnamed_0{bank: 0, page: 0,},};
                      128],};
#[no_mangle]
pub unsafe extern "C" fn __osPfsGetNextPage(mut pfs: *mut OSPfs,
                                            mut bank: *mut u8_0,
                                            mut inode: *mut __OSInode,
                                            mut page: *mut __OSInodeUnit)
 -> s32 {
    let mut ret: s32 = 0;
    if (*page).inode_t.bank as libc::c_int != *bank as libc::c_int {
        *bank = (*page).inode_t.bank;
        ret = __osPfsRWInode(pfs, inode, 0 as libc::c_int as u8_0, *bank);
        if ret != 0 as libc::c_int { return ret }
    }
    *page = (*inode).inodePage[(*page).inode_t.page as usize];
    if !((*page).ipage as libc::c_int >= (*pfs).inodeStartPage &&
             ((*page).inode_t.bank as libc::c_int) <
                 (*pfs).banks as libc::c_int &&
             (*page).inode_t.page as libc::c_int >= 0x1 as libc::c_int &&
             ((*page).inode_t.page as libc::c_int) < 0x80 as libc::c_int) {
        if (*page).ipage as libc::c_int == 1 as libc::c_int {
            return 5 as libc::c_int
        }
        return 3 as libc::c_int
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn osPfsReadWriteFile(mut pfs: *mut OSPfs,
                                            mut fileNo: s32, mut flag: u8_0,
                                            mut offset: s32, mut size: s32,
                                            mut data: *mut u8_0) -> s32 {
    let mut ret: s32 = 0;
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
    let mut inode: __OSInode =
        __OSInode{inodePage:
                      [__OSInodeUnit{inode_t:
                                         C2RustUnnamed_0{bank: 0, page: 0,},};
                          128],};
    let mut curPage: __OSInodeUnit =
        __OSInodeUnit{inode_t: C2RustUnnamed_0{bank: 0, page: 0,},};
    let mut curBlock: s32 = 0;
    let mut blockSize: s32 = 0;
    let mut buffer: *mut u8_0 = 0 as *mut u8_0;
    let mut bank: u8_0 = 0;
    let mut blockno: u16_0 = 0;
    if fileNo >= (*pfs).dir_size || fileNo < 0 as libc::c_int {
        return 5 as libc::c_int
    }
    if size <= 0 as libc::c_int ||
           size % 32 as libc::c_int != 0 as libc::c_int {
        return 5 as libc::c_int
    }
    if offset < 0 as libc::c_int ||
           offset % 32 as libc::c_int != 0 as libc::c_int {
        return 5 as libc::c_int
    }
    if (*pfs).status & 0x1 as libc::c_int == 0 { return 5 as libc::c_int }
    if __osCheckId(pfs) == 2 as libc::c_int { return 2 as libc::c_int }
    if (*pfs).activebank as libc::c_int != 0 as libc::c_int &&
           {
               ret = __osPfsSelectBank(pfs, 0 as libc::c_int as u8_0);
               (ret) != 0 as libc::c_int
           } {
        return ret
    }
    ret =
        __osContRamRead((*pfs).queue, (*pfs).channel,
                        ((*pfs).dir_table + fileNo) as u16_0,
                        &mut dir as *mut __OSDir as *mut u8_0);
    if ret != 0 as libc::c_int { return ret }
    if dir.company_code as libc::c_int == 0 as libc::c_int ||
           dir.game_code == 0 as libc::c_int as libc::c_uint {
        return 5 as libc::c_int
    }
    if !(dir.start_page.ipage as libc::c_int >= (*pfs).inodeStartPage &&
             (dir.start_page.inode_t.bank as libc::c_int) <
                 (*pfs).banks as libc::c_int &&
             dir.start_page.inode_t.page as libc::c_int >= 0x1 as libc::c_int
             &&
             (dir.start_page.inode_t.page as libc::c_int) <
                 0x80 as libc::c_int) {
        if dir.start_page.ipage as libc::c_int == 1 as libc::c_int {
            return 5 as libc::c_int
        }
        return 3 as libc::c_int
    }
    if flag as libc::c_int == 0 as libc::c_int &&
           dir.status as libc::c_int & 2 as libc::c_int == 0 as libc::c_int {
        return 6 as libc::c_int
    }
    bank = 255 as libc::c_int as u8_0;
    curBlock = offset / 32 as libc::c_int;
    curPage = dir.start_page;
    while curBlock >= 8 as libc::c_int {
        ret = __osPfsGetNextPage(pfs, &mut bank, &mut inode, &mut curPage);
        if ret != 0 as libc::c_int { return ret }
        curBlock -= 8 as libc::c_int
    }
    blockSize = size / 32 as libc::c_int;
    buffer = data;
    while blockSize > 0 as libc::c_int {
        if curBlock == 8 as libc::c_int {
            ret =
                __osPfsGetNextPage(pfs, &mut bank, &mut inode, &mut curPage);
            if ret != 0 as libc::c_int { return ret }
            curBlock = 0 as libc::c_int
        }
        if (*pfs).activebank as libc::c_int !=
               curPage.inode_t.bank as libc::c_int &&
               {
                   ret = __osPfsSelectBank(pfs, curPage.inode_t.bank);
                   (ret) != 0 as libc::c_int
               } {
            return ret
        }
        blockno =
            (curPage.inode_t.page as libc::c_int * 8 as libc::c_int +
                 curBlock) as u16_0;
        if flag as libc::c_int == 0 as libc::c_int {
            ret =
                __osContRamRead((*pfs).queue, (*pfs).channel, blockno, buffer)
        } else {
            ret =
                __osContRamWrite((*pfs).queue, (*pfs).channel, blockno,
                                 buffer, 0 as libc::c_int)
        }
        if ret != 0 as libc::c_int { return ret }
        buffer = buffer.offset(32 as libc::c_int as isize);
        curBlock += 1;
        blockSize -= 1
    }
    if flag as libc::c_int == 1 as libc::c_int &&
           dir.status as libc::c_int & 2 as libc::c_int == 0 {
        dir.status = (dir.status as libc::c_int | 2 as libc::c_int) as u8_0;
        if (*pfs).activebank as libc::c_int != 0 as libc::c_int &&
               {
                   ret = __osPfsSelectBank(pfs, 0 as libc::c_int as u8_0);
                   (ret) != 0 as libc::c_int
               } {
            return ret
        }
        ret =
            __osContRamWrite((*pfs).queue, (*pfs).channel,
                             ((*pfs).dir_table + fileNo) as u16_0,
                             &mut dir as *mut __OSDir as *mut u8_0,
                             0 as libc::c_int);
        if ret != 0 as libc::c_int { return ret }
    }
    return __osPfsGetStatus((*pfs).queue, (*pfs).channel);
}
