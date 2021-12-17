#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case,
         non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(c_variadic, const_raw_ptr_to_usize_cast, register_tool)]
extern "C" {
    #[no_mangle]
    fn _Printf(_: PrintCallback, arg: *mut libc::c_void,
               fmt: *const libc::c_char, ap: __builtin_va_list) -> s32;
    #[no_mangle]
    fn osEPiReadIo(handle: *mut OSPiHandle, devAddr: u32_0, data: *mut u32_0)
     -> s32;
    #[no_mangle]
    fn osCartRomInit() -> *mut OSPiHandle;
    #[no_mangle]
    fn osEPiWriteIo(handle: *mut OSPiHandle, devAddr: u32_0, data: u32_0)
     -> s32;
}
pub type __builtin_va_list = *mut libc::c_char;
pub type u8_0 = libc::c_uchar;
pub type u16_0 = libc::c_ushort;
pub type s32 = libc::c_int;
pub type u32_0 = libc::c_uint;
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
pub type PrintCallback
    =
    Option<unsafe extern "C" fn(_: *mut libc::c_void, _: *const libc::c_char,
                                _: u32_0) -> *mut libc::c_void>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ISVDbg {
    pub magic: u32_0,
    pub get: u32_0,
    pub unk_08: [u8_0; 12],
    pub put: u32_0,
    pub unk_18: [u8_0; 8],
    pub data: [u8_0; 65504],
}
#[no_mangle]
pub static mut sISVHandle: *mut OSPiHandle =
    0 as *const OSPiHandle as *mut OSPiHandle;
#[no_mangle]
pub unsafe extern "C" fn isPrintfInit() {
    sISVHandle = osCartRomInit();
    osEPiWriteIo(sISVHandle,
                 &mut (*(0xb3ff0000 as libc::c_uint as *mut ISVDbg)).put as
                     *mut u32_0 as u32_0, 0 as libc::c_int as u32_0);
    osEPiWriteIo(sISVHandle,
                 &mut (*(0xb3ff0000 as libc::c_uint as *mut ISVDbg)).get as
                     *mut u32_0 as u32_0, 0 as libc::c_int as u32_0);
    osEPiWriteIo(sISVHandle,
                 &mut (*(0xb3ff0000 as libc::c_uint as *mut ISVDbg)).magic as
                     *mut u32_0 as u32_0,
                 (('I' as i32) << 24 as libc::c_int |
                      ('S' as i32) << 16 as libc::c_int |
                      ('6' as i32) << 8 as libc::c_int |
                      ('4' as i32) << 0 as libc::c_int) as u32_0);
}
#[no_mangle]
pub unsafe extern "C" fn osSyncPrintfUnused(mut fmt: *const libc::c_char,
                                            mut args: ...) {
    let mut args_0: __builtin_va_list = 0 as *mut libc::c_char;
    args_0 = args.clone();
    _Printf(Some(is_proutSyncPrintf as
                     unsafe extern "C" fn(_: *mut libc::c_void,
                                          _: *const libc::c_char, _: u32_0)
                         -> *mut libc::c_void), 0 as *mut libc::c_void, fmt,
            args_0);
}
#[no_mangle]
pub unsafe extern "C" fn osSyncPrintf(mut fmt: *const libc::c_char,
                                      mut args: ...) {
    let mut args_0: __builtin_va_list = 0 as *mut libc::c_char;
    args_0 = args.clone();
    _Printf(Some(is_proutSyncPrintf as
                     unsafe extern "C" fn(_: *mut libc::c_void,
                                          _: *const libc::c_char, _: u32_0)
                         -> *mut libc::c_void), 0 as *mut libc::c_void, fmt,
            args_0);
}
// assumption
#[no_mangle]
pub unsafe extern "C" fn rmonPrintf(mut fmt: *const libc::c_char,
                                    mut args: ...) {
    let mut args_0: __builtin_va_list = 0 as *mut libc::c_char;
    args_0 = args.clone();
    _Printf(Some(is_proutSyncPrintf as
                     unsafe extern "C" fn(_: *mut libc::c_void,
                                          _: *const libc::c_char, _: u32_0)
                         -> *mut libc::c_void), 0 as *mut libc::c_void, fmt,
            args_0);
}
#[no_mangle]
pub unsafe extern "C" fn is_proutSyncPrintf(mut arg: *mut libc::c_void,
                                            mut str: *const libc::c_char,
                                            mut count: u32_0)
 -> *mut libc::c_void {
    let mut data: u32_0 = 0;
    let mut pos: s32 = 0;
    let mut start: s32 = 0;
    let mut end: s32 = 0;
    osEPiReadIo(sISVHandle,
                &mut (*(0xb3ff0000 as libc::c_uint as *mut ISVDbg)).magic as
                    *mut u32_0 as u32_0, &mut data);
    if data !=
           (('I' as i32) << 24 as libc::c_int |
                ('S' as i32) << 16 as libc::c_int |
                ('6' as i32) << 8 as libc::c_int |
                ('4' as i32) << 0 as libc::c_int) as u32_0 {
        return 1 as libc::c_int as *mut libc::c_void
    }
    osEPiReadIo(sISVHandle,
                &mut (*(0xb3ff0000 as libc::c_uint as *mut ISVDbg)).get as
                    *mut u32_0 as u32_0, &mut data);
    pos = data as s32;
    osEPiReadIo(sISVHandle,
                &mut (*(0xb3ff0000 as libc::c_uint as *mut ISVDbg)).put as
                    *mut u32_0 as u32_0, &mut data);
    start = data as s32;
    end = (start as libc::c_uint).wrapping_add(count) as s32;
    if end >= 0xffe0 as libc::c_int {
        end -= 0xffe0 as libc::c_int;
        if pos < end || start < pos {
            return 1 as libc::c_int as *mut libc::c_void
        }
    } else if start < pos && pos < end {
        return 1 as libc::c_int as *mut libc::c_void
    }
    while count != 0 {
        let mut addr: u32_0 =
            (&mut (*(0xb3ff0000 as libc::c_uint as *mut ISVDbg)).data as
                 *mut [u8_0; 65504] as
                 u32_0).wrapping_add((start & 0xffffffc as libc::c_int) as
                                         libc::c_uint);
        let mut shift: s32 =
            (3 as libc::c_int - (start & 3 as libc::c_int)) *
                8 as libc::c_int;
        if *str != 0 {
            osEPiReadIo(sISVHandle, addr, &mut data);
            osEPiWriteIo(sISVHandle, addr,
                         ((*str as libc::c_int) << shift) as libc::c_uint |
                             data &
                                 !((0xff as libc::c_int) << shift) as
                                     libc::c_uint);
            start += 1;
            if start >= 0xffe0 as libc::c_int {
                start -= 0xffe0 as libc::c_int
            }
        }
        count = count.wrapping_sub(1);
        str = str.offset(1)
    }
    osEPiWriteIo(sISVHandle,
                 &mut (*(0xb3ff0000 as libc::c_uint as *mut ISVDbg)).put as
                     *mut u32_0 as u32_0, start as u32_0);
    return 1 as libc::c_int as *mut libc::c_void;
}
#[no_mangle]
pub unsafe extern "C" fn func_80002384(mut exp: *const libc::c_char,
                                       mut file: *const libc::c_char,
                                       mut line: u32_0) {
    osSyncPrintf(b"File:%s Line:%d  %s \n\x00" as *const u8 as
                     *const libc::c_char, file, line, exp);
    loop  { };
}
