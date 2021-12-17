#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case,
         non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(register_tool)]
extern "C" {
    #[no_mangle]
    fn bzero(__s: *mut libc::c_void, __n: u32_0);
    #[no_mangle]
    fn __osGetId(pfs: *mut OSPfs) -> s32;
    #[no_mangle]
    fn __osCheckId(pfs: *mut OSPfs) -> s32;
    #[no_mangle]
    fn __osPfsRWInode(pfs: *mut OSPfs, inode: *mut __OSInode, flag: u8_0,
                      bank: u8_0) -> s32;
    #[no_mangle]
    fn __osPfsSelectBank(pfs: *mut OSPfs, bank: u8_0) -> s32;
    #[no_mangle]
    fn __osContRamRead(ctrlrqueue: *mut OSMesgQueue, channel: s32,
                       addr: u16_0, data: *mut u8_0) -> s32;
    #[no_mangle]
    fn __osContRamWrite(mq: *mut OSMesgQueue, channel: s32, address: u16_0,
                        buffer: *mut u8_0, force: s32) -> s32;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __OSInodeCache {
    pub inode: __OSInode,
    pub bank: u8_0,
    pub map: [u8_0; 256],
}
#[no_mangle]
pub unsafe extern "C" fn osPfsChecker(mut pfs: *mut OSPfs) -> s32 {
    let mut j: s32 = 0;
    let mut ret: s32 = 0;
    let mut next: __OSInodeUnit =
        __OSInodeUnit{inode_t: C2RustUnnamed_0{bank: 0, page: 0,},};
    let mut checkedInode: __OSInode =
        __OSInode{inodePage:
                      [__OSInodeUnit{inode_t:
                                         C2RustUnnamed_0{bank: 0, page: 0,},};
                          128],};
    let mut tempInode: __OSInode =
        __OSInode{inodePage:
                      [__OSInodeUnit{inode_t:
                                         C2RustUnnamed_0{bank: 0, page: 0,},};
                          128],};
    let mut tempDir: __OSDir =
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
    let mut nextNodeInFile: [__OSInodeUnit; 16] =
        [__OSInodeUnit{inode_t: C2RustUnnamed_0{bank: 0, page: 0,},}; 16];
    let mut cache: __OSInodeCache =
        __OSInodeCache{inode:
                           __OSInode{inodePage:
                                         [__OSInodeUnit{inode_t:
                                                            C2RustUnnamed_0{bank:
                                                                                0,
                                                                            page:
                                                                                0,},};
                                             128],},
                       bank: 0,
                       map: [0; 256],};
    let mut fixed: s32 = 0 as libc::c_int;
    let mut bank: u8_0 = 0;
    let mut prevBank: u8_0 = 254 as libc::c_int as u8_0;
    let mut cc: s32 = 0;
    let mut cl: s32 = 0;
    let mut offset: s32 = 0;
    ret = __osCheckId(pfs);
    if ret == 2 as libc::c_int { ret = __osGetId(pfs) }
    if ret != 0 { return ret }
    ret = func_80105788(pfs, &mut cache);
    if ret != 0 as libc::c_int { return ret }
    j = 0 as libc::c_int;
    while j < (*pfs).dir_size {
        ret =
            __osContRamRead((*pfs).queue, (*pfs).channel,
                            ((*pfs).dir_table + j) as u16_0,
                            &mut tempDir as *mut __OSDir as *mut u8_0);
        if ret != 0 as libc::c_int { return ret }
        if tempDir.company_code as libc::c_int != 0 as libc::c_int ||
               tempDir.game_code != 0 as libc::c_int as libc::c_uint {
            if tempDir.company_code as libc::c_int == 0 as libc::c_int ||
                   tempDir.game_code == 0 as libc::c_int as libc::c_uint {
                cc = -(1 as libc::c_int)
            } else {
                next = tempDir.start_page;
                cc = 0 as libc::c_int;
                cl = cc;
                bank = 255 as libc::c_int as u8_0;
                while next.ipage as libc::c_int >= (*pfs).inodeStartPage &&
                          (next.inode_t.bank as libc::c_int) <
                              (*pfs).banks as libc::c_int &&
                          next.inode_t.page as libc::c_int >=
                              0x1 as libc::c_int &&
                          (next.inode_t.page as libc::c_int) <
                              0x80 as libc::c_int {
                    if bank as libc::c_int != next.inode_t.bank as libc::c_int
                       {
                        bank = next.inode_t.bank;
                        if prevBank as libc::c_int != bank as libc::c_int {
                            ret =
                                __osPfsRWInode(pfs, &mut tempInode,
                                               0 as libc::c_int as u8_0,
                                               bank);
                            prevBank = bank
                        }
                        if ret != 0 as libc::c_int && ret != 3 as libc::c_int
                           {
                            return ret
                        }
                    }
                    cc = func_80105A60(pfs, next, &mut cache) - cl;
                    if cc != 0 as libc::c_int { break ; }
                    cl = 1 as libc::c_int;
                    next = tempInode.inodePage[next.inode_t.page as usize]
                }
            }
            if cc != 0 as libc::c_int ||
                   next.ipage as libc::c_int != 1 as libc::c_int {
                bzero(&mut tempDir as *mut __OSDir as *mut libc::c_void,
                      ::std::mem::size_of::<__OSDir>() as libc::c_ulong);
                if (*pfs).activebank as libc::c_int != 0 as libc::c_int {
                    ret = __osPfsSelectBank(pfs, 0 as libc::c_int as u8_0);
                    if ret != 0 as libc::c_int { return ret }
                }
                ret =
                    __osContRamWrite((*pfs).queue, (*pfs).channel,
                                     ((*pfs).dir_table + j) as u16_0,
                                     &mut tempDir as *mut __OSDir as
                                         *mut u8_0, 0 as libc::c_int);
                if ret != 0 as libc::c_int { return ret }
                fixed += 1
            }
        }
        j += 1
    }
    j = 0 as libc::c_int;
    while j < (*pfs).dir_size {
        ret =
            __osContRamRead((*pfs).queue, (*pfs).channel,
                            ((*pfs).dir_table + j) as u16_0,
                            &mut tempDir as *mut __OSDir as *mut u8_0);
        if ret != 0 as libc::c_int { return ret }
        if tempDir.company_code as libc::c_int != 0 as libc::c_int &&
               tempDir.game_code != 0 as libc::c_int as libc::c_uint &&
               tempDir.start_page.ipage as libc::c_int >=
                   (*pfs).inodeStartPage as u16_0 as libc::c_int {
            // cast required
            nextNodeInFile[j as usize].ipage = tempDir.start_page.ipage
        } else {
            nextNodeInFile[j as usize].ipage = 0 as libc::c_int as u16_0
        }
        j += 1
    }
    bank = 0 as libc::c_int as u8_0;
    while (bank as libc::c_int) < (*pfs).banks as libc::c_int {
        ret =
            __osPfsRWInode(pfs, &mut tempInode, 0 as libc::c_int as u8_0,
                           bank);
        if ret != 0 as libc::c_int && ret != 3 as libc::c_int { return ret }
        offset =
            if bank as libc::c_int > 0 as libc::c_int {
                1 as libc::c_int
            } else { (*pfs).inodeStartPage };
        j = 0 as libc::c_int;
        while j < offset {
            checkedInode.inodePage[j as usize].ipage =
                tempInode.inodePage[j as usize].ipage;
            j += 1
        }
        while j < 128 as libc::c_int {
            checkedInode.inodePage[j as usize].ipage =
                3 as libc::c_int as u16_0;
            j += 1
        }
        j = 0 as libc::c_int;
        while j < (*pfs).dir_size {
            while nextNodeInFile[j as usize].inode_t.bank as libc::c_int ==
                      bank as libc::c_int &&
                      nextNodeInFile[j as usize].ipage as libc::c_int >=
                          (*pfs).inodeStartPage as u16_0 as libc::c_int {
                // cast required
                let mut val: u8_0 = 0;
                val = nextNodeInFile[j as usize].inode_t.page;
                checkedInode.inodePage[val as usize] =
                    tempInode.inodePage[val as usize];
                nextNodeInFile[j as usize] =
                    checkedInode.inodePage[val as usize]
            }
            j += 1
        }
        ret =
            __osPfsRWInode(pfs, &mut checkedInode, 1 as libc::c_int as u8_0,
                           bank);
        if ret != 0 as libc::c_int { return ret }
        bank = bank.wrapping_add(1)
    }
    if fixed != 0 as libc::c_int {
        (*pfs).status |= 0x2 as libc::c_int
    } else { (*pfs).status &= !(0x2 as libc::c_int) }
    return 0 as libc::c_int;
}
// Original name: corrupted_init (probably needs better name)
#[no_mangle]
pub unsafe extern "C" fn func_80105788(mut pfs: *mut OSPfs,
                                       mut cache: *mut __OSInodeCache)
 -> s32 {
    let mut i: s32 = 0;
    let mut n: s32 = 0;
    let mut offset: s32 = 0;
    let mut bank: u8_0 = 0;
    let mut tpage: __OSInodeUnit =
        __OSInodeUnit{inode_t: C2RustUnnamed_0{bank: 0, page: 0,},};
    let mut tempInode: __OSInode =
        __OSInode{inodePage:
                      [__OSInodeUnit{inode_t:
                                         C2RustUnnamed_0{bank: 0, page: 0,},};
                          128],};
    let mut ret: s32 = 0;
    i = 0 as libc::c_int;
    while i < 8 as libc::c_int * 32 as libc::c_int {
        (*cache).map[i as usize] = 0 as libc::c_int as u8_0;
        i += 1
    }
    (*cache).bank = 255 as libc::c_int as u8_0;
    bank = 0 as libc::c_int as u8_0;
    while (bank as libc::c_int) < (*pfs).banks as libc::c_int {
        offset =
            if bank as libc::c_int > 0 as libc::c_int {
                1 as libc::c_int
            } else { (*pfs).inodeStartPage };
        ret =
            __osPfsRWInode(pfs, &mut tempInode, 0 as libc::c_int as u8_0,
                           bank);
        if ret != 0 as libc::c_int && ret != 3 as libc::c_int { return ret }
        i = offset;
        while i < 128 as libc::c_int {
            tpage = tempInode.inodePage[i as usize];
            if tpage.ipage as libc::c_int >= (*pfs).inodeStartPage &&
                   tpage.inode_t.bank as libc::c_int != bank as libc::c_int {
                n =
                    (tpage.inode_t.page as libc::c_int & 0x7f as libc::c_int)
                        / (128 as libc::c_int / 32 as libc::c_int) +
                        32 as libc::c_int *
                            (tpage.inode_t.bank as libc::c_int %
                                 8 as libc::c_int);
                (*cache).map[n as usize] =
                    ((*cache).map[n as usize] as libc::c_int |
                         (1 as libc::c_int) <<
                             bank as libc::c_int % 8 as libc::c_int) as u8_0
            }
            i += 1
        }
        bank = bank.wrapping_add(1)
    }
    return 0 as libc::c_int;
}
// original name: corrupted (probably needs a better name)
#[no_mangle]
pub unsafe extern "C" fn func_80105A60(mut pfs: *mut OSPfs,
                                       mut fpage: __OSInodeUnit,
                                       mut cache: *mut __OSInodeCache)
 -> s32 {
    let mut j: s32 = 0;
    let mut n: s32 = 0;
    let mut hit: s32 = 0 as libc::c_int;
    let mut bank: u8_0 = 0;
    let mut offset: s32 = 0;
    let mut ret: s32 = 0 as libc::c_int;
    n =
        fpage.inode_t.page as libc::c_int /
            (128 as libc::c_int / 32 as libc::c_int) +
            32 as libc::c_int *
                (fpage.inode_t.bank as libc::c_int % 8 as libc::c_int);
    bank = 0 as libc::c_int as u8_0;
    while (bank as libc::c_int) < (*pfs).banks as libc::c_int {
        offset =
            if bank as libc::c_int > 0 as libc::c_int {
                1 as libc::c_int
            } else { (*pfs).inodeStartPage };
        if bank as libc::c_int == fpage.inode_t.bank as libc::c_int ||
               (*cache).map[n as usize] as libc::c_int &
                   (1 as libc::c_int) <<
                       bank as libc::c_int % 8 as libc::c_int !=
                   0 as libc::c_int {
            if bank as libc::c_int != (*cache).bank as libc::c_int {
                ret =
                    __osPfsRWInode(pfs, &mut (*cache).inode,
                                   0 as libc::c_int as u8_0, bank);
                if ret != 0 && ret != 3 as libc::c_int { return ret }
                (*cache).bank = bank
            }
            j = offset;
            while hit < 2 as libc::c_int && j < 128 as libc::c_int {
                if (*cache).inode.inodePage[j as usize].ipage as libc::c_int
                       == fpage.ipage as libc::c_int {
                    hit += 1
                }
                j += 1
            }
            if hit >= 2 as libc::c_int { return 2 as libc::c_int }
        }
        bank = bank.wrapping_add(1)
    }
    return hit;
}
