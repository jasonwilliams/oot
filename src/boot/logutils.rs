#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case,
         non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(register_tool)]
extern "C" {
    #[no_mangle]
    fn osSyncPrintf(fmt: *const libc::c_char, _: ...);
    #[no_mangle]
    fn osGetThreadId(thread: *mut OSThread) -> OSId;
    #[no_mangle]
    fn Fault_AddHungupAndCrash(_: *const libc::c_char, _: u32_0);
    #[no_mangle]
    static mut osMemSize: u32_0;
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
#[no_mangle]
pub unsafe extern "C" fn LogUtils_CheckFloatRange(mut exp:
                                                      *const libc::c_char,
                                                  mut line: s32,
                                                  mut valueName:
                                                      *const libc::c_char,
                                                  mut value: f32_0,
                                                  mut minName:
                                                      *const libc::c_char,
                                                  mut min: f32_0,
                                                  mut maxName:
                                                      *const libc::c_char,
                                                  mut max: f32_0) -> f32_0 {
    if value < min || max < value {
        osSyncPrintf(b"%s %d: range error %s(%f) < %s(%f) < %s(%f)\n\x00" as
                         *const u8 as *const libc::c_char, exp, line, minName,
                     min as libc::c_double, valueName,
                     value as libc::c_double, maxName, max as libc::c_double);
    }
    return value;
}
#[no_mangle]
pub unsafe extern "C" fn LogUtils_CheckIntRange(mut exp: *const libc::c_char,
                                                mut line: s32,
                                                mut valueName:
                                                    *const libc::c_char,
                                                mut value: s32,
                                                mut minName:
                                                    *const libc::c_char,
                                                mut min: s32,
                                                mut maxName:
                                                    *const libc::c_char,
                                                mut max: s32) -> s32 {
    if value < min || max < value {
        osSyncPrintf(b"%s %d: range error %s(%d) < %s(%d) < %s(%d)\n\x00" as
                         *const u8 as *const libc::c_char, exp, line, minName,
                     min, valueName, value, maxName, max);
    }
    return value;
}
#[no_mangle]
pub unsafe extern "C" fn LogUtils_LogHexDump(mut ptr: *mut libc::c_void,
                                             mut size0: s32) {
    let mut addr: *mut u8_0 = ptr as *mut u8_0;
    let mut size: s32 = size0;
    let mut rest: s32 = 0;
    let mut i: s32 = 0;
    let mut off: u32_0 = 0;
    osSyncPrintf(b"dump(%08x, %u)\n\x00" as *const u8 as *const libc::c_char,
                 addr, size);
    osSyncPrintf(b"address  off  +0 +1 +2 +3 +4 +5 +6 +7 +8 +9 +a +b +c +d +e +f   0123456789abcdef\n\x00"
                     as *const u8 as *const libc::c_char);
    off = 0 as libc::c_int as u32_0;
    while size > 0 as libc::c_int {
        osSyncPrintf(b"%08x %04x\x00" as *const u8 as *const libc::c_char,
                     addr, off);
        rest =
            if size < 0x10 as libc::c_int {
                size
            } else { 0x10 as libc::c_int };
        i = 0 as libc::c_int;
        loop  {
            if i < rest {
                osSyncPrintf(b" %02x\x00" as *const u8 as *const libc::c_char,
                             *addr.offset(i as isize) as libc::c_int);
            } else {
                osSyncPrintf(b"   \x00" as *const u8 as *const libc::c_char);
            }
            i += 1;
            if i > 0xf as libc::c_int { break ; }
        }
        osSyncPrintf(b"   \x00" as *const u8 as *const libc::c_char);
        i = 0 as libc::c_int;
        loop  {
            if i < rest {
                let mut a: u8_0 = *addr.offset(i as isize);
                osSyncPrintf(b"%c\x00" as *const u8 as *const libc::c_char,
                             if a as libc::c_int >= 0x20 as libc::c_int &&
                                    (a as libc::c_int) < 0x7f as libc::c_int {
                                 a as libc::c_int
                             } else { '.' as i32 });
            } else {
                osSyncPrintf(b" \x00" as *const u8 as *const libc::c_char);
            }
            i += 1;
            if i > 0xf as libc::c_int { break ; }
        }
        osSyncPrintf(b"\n\x00" as *const u8 as *const libc::c_char);
        size -= rest;
        addr = addr.offset(rest as isize);
        off =
            (off as libc::c_uint).wrapping_add(rest as libc::c_uint) as u32_0
                as u32_0
    };
}
#[no_mangle]
pub unsafe extern "C" fn LogUtils_LogPointer(mut value: s32, mut max: u32_0,
                                             mut ptr: *mut libc::c_void,
                                             mut name: *const libc::c_char,
                                             mut file: *const libc::c_char,
                                             mut line: s32) {
    osSyncPrintf(b"\x1b[41;37m%s %d %s[%d] max=%u ptr=%08x\n\x1b[m\x00" as
                     *const u8 as *const libc::c_char, file, line, name,
                 value, max, ptr);
}
#[no_mangle]
pub unsafe extern "C" fn LogUtils_CheckBoundary(mut name: *const libc::c_char,
                                                mut value: s32, mut unk: s32,
                                                mut file: *const libc::c_char,
                                                mut line: s32) {
    let mut mask: u32_0 = (unk - 1 as libc::c_int) as u32_0;
    if value as libc::c_uint & mask != 0 {
        osSyncPrintf(b"\x1b[41;37m%s %d:%s(%08x) \xe3\x81\xaf \xe3\x83\x90\xe3\x82\xa6\xe3\x83\xb3\xe3\x83\x80\xe3\x83\xaa(%d)\xe9\x81\x95\xe5\x8f\x8d\xe3\x81\xa7\xe3\x81\x99\n\x1b[m\x00"
                         as *const u8 as *const libc::c_char, file, line,
                     name, value, unk);
    };
}
#[no_mangle]
pub unsafe extern "C" fn LogUtils_CheckNullPointer(mut exp:
                                                       *const libc::c_char,
                                                   mut ptr: *mut libc::c_void,
                                                   mut file:
                                                       *const libc::c_char,
                                                   mut line: s32) {
    if ptr.is_null() {
        osSyncPrintf(b"\x1b[41;37m%s %d:%s \xe3\x81\xaf \xe3\x81\xaf\xe3\x83\x8c\xe3\x83\xab\xe3\x83\x9d\xe3\x82\xa4\xe3\x83\xb3\xe3\x82\xbf\xe3\x81\xa7\xe3\x81\x99\n\x1b[m\x00"
                         as *const u8 as *const libc::c_char, file, line,
                     exp);
    };
}
#[no_mangle]
pub unsafe extern "C" fn LogUtils_CheckValidPointer(mut exp:
                                                        *const libc::c_char,
                                                    mut ptr:
                                                        *mut libc::c_void,
                                                    mut file:
                                                        *const libc::c_char,
                                                    mut line: s32) {
    if ptr.is_null() || (ptr as u32_0) < 0x80000000 as libc::c_uint ||
           (0x80000000 as libc::c_uint).wrapping_add(osMemSize) <=
               ptr as u32_0 {
        osSyncPrintf(b"\x1b[41;37m%s %d:\xe3\x83\x9d\xe3\x82\xa4\xe3\x83\xb3\xe3\x82\xbf %s(%08x) \xe3\x81\x8c\xe7\x95\xb0\xe5\xb8\xb8\xe3\x81\xa7\xe3\x81\x99\n\x1b[m\x00"
                         as *const u8 as *const libc::c_char, file, line, exp,
                     ptr);
    };
}
#[no_mangle]
pub unsafe extern "C" fn LogUtils_LogThreadId(mut name: *const libc::c_char,
                                              mut line: s32) {
    osSyncPrintf(b"<%d %s %d>\x00" as *const u8 as *const libc::c_char,
                 osGetThreadId(0 as *mut OSThread), name, line);
}
#[no_mangle]
pub unsafe extern "C" fn LogUtils_HungupThread(mut name: *const libc::c_char,
                                               mut line: s32) {
    osSyncPrintf(b"*** HungUp in thread %d, [%s:%d] ***\n\x00" as *const u8 as
                     *const libc::c_char, osGetThreadId(0 as *mut OSThread),
                 name, line);
    Fault_AddHungupAndCrash(name, line as u32_0);
}
#[no_mangle]
pub unsafe extern "C" fn LogUtils_ResetHungup() {
    osSyncPrintf(b"*** Reset ***\n\x00" as *const u8 as *const libc::c_char);
    Fault_AddHungupAndCrash(b"Reset\x00" as *const u8 as *const libc::c_char,
                            0 as libc::c_int as u32_0);
}
