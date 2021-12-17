#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case,
         non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(const_raw_ptr_to_usize_cast, register_tool)]
extern "C" {
    #[no_mangle]
    fn __osExceptionPreamble();
    #[no_mangle]
    fn bzero(__s: *mut libc::c_void, __n: u32_0);
    #[no_mangle]
    fn __osSetSR(_: u32_0);
    #[no_mangle]
    fn __osGetSR() -> u32_0;
    #[no_mangle]
    fn osWritebackDCache(vaddr: *mut libc::c_void, nbytes: s32);
    #[no_mangle]
    fn __osSiRawReadIo(devAddr: *mut libc::c_void, dst: *mut u32_0) -> s32;
    #[no_mangle]
    fn osUnmapTLBAll();
    #[no_mangle]
    fn osInvalICache(vaddr: *mut libc::c_void, nbytes: s32);
    #[no_mangle]
    fn __osSetFpcCsr(_: u32_0);
    #[no_mangle]
    fn osMapTLBRdb();
    #[no_mangle]
    fn __osGetCause() -> u32_0;
    #[no_mangle]
    fn __osSiRawWriteIo(devAddr: *mut libc::c_void, val: u32_0) -> s32;
    #[no_mangle]
    fn __osSetWatchLo(_: u32_0);
    #[no_mangle]
    static mut __Dom1SpeedParam: OSPiHandle;
    #[no_mangle]
    static mut __Dom2SpeedParam: OSPiHandle;
    #[no_mangle]
    static mut osResetType: u32_0;
    #[no_mangle]
    static mut osAppNmiBuffer: [u8_0; 64];
    #[no_mangle]
    static mut osTvType: u32_0;
}
pub type u8_0 = libc::c_uchar;
pub type u16_0 = libc::c_ushort;
pub type s32 = libc::c_int;
pub type u32_0 = libc::c_uint;
pub type u64_0 = libc::c_ulonglong;
pub type OSHWIntr = u32_0;
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
pub struct struct_exceptionPreamble {
    pub ins_00: u32_0,
    pub ins_04: u32_0,
    pub ins_08: u32_0,
    pub ins_0C: u32_0,
}
#[no_mangle]
pub static mut osClockRate: u64_0 = 62500000 as libc::c_longlong as u64_0;
#[no_mangle]
pub static mut osViClock: s32 = 48681812 as libc::c_int;
#[no_mangle]
pub static mut __osShutdown: u32_0 = 0 as libc::c_int as u32_0;
#[no_mangle]
pub static mut __OSGlobalIntMask: OSHWIntr =
    0x3fff01 as libc::c_int as OSHWIntr;
#[no_mangle]
pub static mut D_800145C0: u32_0 = 0;
#[no_mangle]
pub unsafe extern "C" fn __createSpeedParam() {
    __Dom1SpeedParam.type_0 = 7 as libc::c_int as u8_0; // TLB miss
    __Dom1SpeedParam.latency =
        *((0x4600014 as libc::c_int as libc::c_uint |
               0xa0000000 as libc::c_uint) as *mut u32_0) as
            u8_0; // XTLB miss
    __Dom1SpeedParam.pulse =
        *((0x4600018 as libc::c_int as libc::c_uint |
               0xa0000000 as libc::c_uint) as *mut u32_0) as
            u8_0; // cache errors
    __Dom1SpeedParam.pageSize =
        *((0x460001c as libc::c_int as libc::c_uint |
               0xa0000000 as libc::c_uint) as *mut u32_0) as
            u8_0; // general exceptions
    __Dom1SpeedParam.relDuration =
        *((0x4600020 as libc::c_int as libc::c_uint |
               0xa0000000 as libc::c_uint) as *mut u32_0) as u8_0;
    __Dom2SpeedParam.type_0 = 7 as libc::c_int as u8_0;
    __Dom2SpeedParam.latency =
        *((0x4600024 as libc::c_int as libc::c_uint |
               0xa0000000 as libc::c_uint) as *mut u32_0) as u8_0;
    __Dom2SpeedParam.pulse =
        *((0x4600028 as libc::c_int as libc::c_uint |
               0xa0000000 as libc::c_uint) as *mut u32_0) as u8_0;
    __Dom2SpeedParam.pageSize =
        *((0x460002c as libc::c_int as libc::c_uint |
               0xa0000000 as libc::c_uint) as *mut u32_0) as u8_0;
    __Dom2SpeedParam.relDuration =
        *((0x4600030 as libc::c_int as libc::c_uint |
               0xa0000000 as libc::c_uint) as *mut u32_0) as u8_0;
}
#[no_mangle]
pub unsafe extern "C" fn __osInitialize_common() {
    let mut sp2C: u32_0 = 0;
    D_800145C0 = 1 as libc::c_int as u32_0;
    __osSetSR(__osGetSR() | 0x20000000 as libc::c_int as libc::c_uint);
    __osSetFpcCsr((0x1000000 as libc::c_int | 0x800 as libc::c_int) as u32_0);
    __osSetWatchLo(0x4900000 as libc::c_int as u32_0);
    while __osSiRawReadIo((0x1fc007c0 as libc::c_int + 0x3c as libc::c_int) as
                              *mut libc::c_void, &mut sp2C) != 0 {
    }
    while __osSiRawWriteIo((0x1fc007c0 as libc::c_int + 0x3c as libc::c_int)
                               as *mut libc::c_void,
                           sp2C | 8 as libc::c_int as libc::c_uint) != 0 {
    }
    *(0x80000000 as libc::c_uint as *mut struct_exceptionPreamble) =
        *::std::mem::transmute::<Option<unsafe extern "C" fn() -> ()>,
                                 *mut struct_exceptionPreamble>(Some(::std::mem::transmute::<unsafe extern "C" fn()
                                                                                                 ->
                                                                                                     (),
                                                                                             unsafe extern "C" fn()
                                                                                                 ->
                                                                                                     ()>(__osExceptionPreamble)));
    *((0x80000000 as
           libc::c_uint).wrapping_add(0x80 as libc::c_int as libc::c_uint) as
          *mut struct_exceptionPreamble) =
        *::std::mem::transmute::<Option<unsafe extern "C" fn() -> ()>,
                                 *mut struct_exceptionPreamble>(Some(::std::mem::transmute::<unsafe extern "C" fn()
                                                                                                 ->
                                                                                                     (),
                                                                                             unsafe extern "C" fn()
                                                                                                 ->
                                                                                                     ()>(__osExceptionPreamble)));
    *((0x80000000 as
           libc::c_uint).wrapping_add(0x100 as libc::c_int as libc::c_uint) as
          *mut struct_exceptionPreamble) =
        *::std::mem::transmute::<Option<unsafe extern "C" fn() -> ()>,
                                 *mut struct_exceptionPreamble>(Some(::std::mem::transmute::<unsafe extern "C" fn()
                                                                                                 ->
                                                                                                     (),
                                                                                             unsafe extern "C" fn()
                                                                                                 ->
                                                                                                     ()>(__osExceptionPreamble)));
    *((0x80000000 as
           libc::c_uint).wrapping_add(0x180 as libc::c_int as libc::c_uint) as
          *mut struct_exceptionPreamble) =
        *::std::mem::transmute::<Option<unsafe extern "C" fn() -> ()>,
                                 *mut struct_exceptionPreamble>(Some(::std::mem::transmute::<unsafe extern "C" fn()
                                                                                                 ->
                                                                                                     (),
                                                                                             unsafe extern "C" fn()
                                                                                                 ->
                                                                                                     ()>(__osExceptionPreamble)));
    osWritebackDCache(0x80000000 as libc::c_uint as *mut libc::c_void,
                      (0x80000000 as
                           libc::c_uint).wrapping_add(0x180 as libc::c_int as
                                                          libc::c_uint).wrapping_sub(0x80000000
                                                                                         as
                                                                                         libc::c_uint).wrapping_add(::std::mem::size_of::<struct_exceptionPreamble>()
                                                                                                                        as
                                                                                                                        libc::c_ulong)
                          as s32);
    osInvalICache(0x80000000 as libc::c_uint as *mut libc::c_void,
                  (0x80000000 as
                       libc::c_uint).wrapping_add(0x180 as libc::c_int as
                                                      libc::c_uint).wrapping_sub(0x80000000
                                                                                     as
                                                                                     libc::c_uint).wrapping_add(::std::mem::size_of::<struct_exceptionPreamble>()
                                                                                                                    as
                                                                                                                    libc::c_ulong)
                      as s32);
    __createSpeedParam();
    osUnmapTLBAll();
    osMapTLBRdb();
    osClockRate =
        osClockRate.wrapping_mul(3 as libc::c_longlong as
                                     libc::c_ulonglong).wrapping_div(4 as
                                                                         libc::c_ulonglong);
    if osResetType == 0 {
        bzero(osAppNmiBuffer.as_mut_ptr() as *mut libc::c_void,
              ::std::mem::size_of::<[u8_0; 64]>() as libc::c_ulong);
    }
    if osTvType == 0 as libc::c_int as libc::c_uint {
        osViClock = 49656530 as libc::c_int
    } else if osTvType == 2 as libc::c_int as libc::c_uint {
        osViClock = 48628316 as libc::c_int
    } else { osViClock = 48681812 as libc::c_int }
    // Wait until there are no RCP interrupts
    if __osGetCause() & 0x1000 as libc::c_int as libc::c_uint != 0 {
        loop  { }
    }
    ::std::ptr::write_volatile((0x4500008 as libc::c_int as libc::c_uint |
                                    0xa0000000 as libc::c_uint) as *mut u32_0,
                               1 as libc::c_int as u32_0);
    ::std::ptr::write_volatile((0x4500010 as libc::c_int as libc::c_uint |
                                    0xa0000000 as libc::c_uint) as *mut u32_0,
                               0x3fff as libc::c_int as u32_0);
    ::std::ptr::write_volatile((0x4500014 as libc::c_int as libc::c_uint |
                                    0xa0000000 as libc::c_uint) as *mut u32_0,
                               0xf as libc::c_int as u32_0);
}
#[no_mangle]
pub unsafe extern "C" fn __osInitialize_autodetect() { }
