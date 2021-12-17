#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case,
         non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(const_raw_ptr_to_usize_cast, register_tool)]
extern "C" {
    #[no_mangle]
    fn __osPiGetAccess();
    #[no_mangle]
    fn __osPiRelAccess();
    #[no_mangle]
    fn bzero(__s: *mut libc::c_void, __n: u32_0);
    #[no_mangle]
    fn __osDisableInt() -> s32;
    #[no_mangle]
    fn __osRestoreInt(_: s32);
    #[no_mangle]
    static mut __osPiTable: *mut OSPiHandle;
}
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
#[no_mangle]
pub static mut __CartRomHandle: OSPiHandle =
    OSPiHandle{next: 0 as *const OSPiHandle as *mut OSPiHandle,
               type_0: 0,
               latency: 0,
               pageSize: 0,
               relDuration: 0,
               pulse: 0,
               domain: 0,
               baseAddress: 0,
               speed: 0,
               transferInfo:
                   __OSTranxInfo{cmdType: 0,
                                 transferMode: 0,
                                 blockNum: 0,
                                 sectorNum: 0,
                                 devAddr: 0,
                                 bmCtlShadow: 0,
                                 seqCtlShadow: 0,
                                 block:
                                     [__OSBlockInfo{errStatus: 0,
                                                    dramAddr:
                                                        0 as
                                                            *const libc::c_void
                                                            as
                                                            *mut libc::c_void,
                                                    C2Addr:
                                                        0 as
                                                            *const libc::c_void
                                                            as
                                                            *mut libc::c_void,
                                                    sectorSize: 0,
                                                    C1ErrNum: 0,
                                                    C1ErrSector: [0; 4],};
                                         2],},};
#[no_mangle]
pub unsafe extern "C" fn osCartRomInit() -> *mut OSPiHandle {
    let mut a: u32_0 = 0;
    let mut status: s32 = 0;
    let mut prevInt: u32_0 = 0;
    let mut lastLatency: u32_0 = 0;
    let mut lastPageSize: u32_0 = 0;
    let mut lastRelDuration: u32_0 = 0;
    let mut lastPulse: u32_0 = 0;
    static mut D_8000AF10: u32_0 = 1 as libc::c_int as u32_0;
    __osPiGetAccess();
    if D_8000AF10 == 0 { __osPiRelAccess(); return &mut __CartRomHandle }
    D_8000AF10 = 0 as libc::c_int as u32_0;
    __CartRomHandle.type_0 = 0 as libc::c_int as u8_0;
    __CartRomHandle.baseAddress = 0xb0000000 as libc::c_uint;
    __CartRomHandle.domain = 0 as libc::c_int as u8_0;
    __CartRomHandle.speed = 0 as libc::c_int as u32_0;
    bzero(&mut __CartRomHandle.transferInfo as *mut __OSTranxInfo as
              *mut libc::c_void,
          ::std::mem::size_of::<__OSTranxInfo>() as libc::c_ulong);
    loop  {
        status =
            *((0x4600010 as libc::c_int as libc::c_uint |
                   0xa0000000 as libc::c_uint) as *mut u32_0) as s32;
        if !(status & (0x1 as libc::c_int | 0x2 as libc::c_int) != 0) {
            break ;
        }
    }
    lastLatency =
        *((0x4600014 as libc::c_int as libc::c_uint |
               0xa0000000 as libc::c_uint) as *mut u32_0);
    lastPageSize =
        *((0x460001c as libc::c_int as libc::c_uint |
               0xa0000000 as libc::c_uint) as *mut u32_0);
    lastRelDuration =
        *((0x4600020 as libc::c_int as libc::c_uint |
               0xa0000000 as libc::c_uint) as *mut u32_0);
    lastPulse =
        *((0x4600018 as libc::c_int as libc::c_uint |
               0xa0000000 as libc::c_uint) as *mut u32_0);
    ::std::ptr::write_volatile((0x4600014 as libc::c_int as libc::c_uint |
                                    0xa0000000 as libc::c_uint) as *mut u32_0,
                               0xff as libc::c_int as u32_0);
    ::std::ptr::write_volatile((0x460001c as libc::c_int as libc::c_uint |
                                    0xa0000000 as libc::c_uint) as *mut u32_0,
                               0 as libc::c_int as u32_0);
    ::std::ptr::write_volatile((0x4600020 as libc::c_int as libc::c_uint |
                                    0xa0000000 as libc::c_uint) as *mut u32_0,
                               3 as libc::c_int as u32_0);
    ::std::ptr::write_volatile((0x4600018 as libc::c_int as libc::c_uint |
                                    0xa0000000 as libc::c_uint) as *mut u32_0,
                               0xff as libc::c_int as u32_0);
    a =
        *((__CartRomHandle.baseAddress | 0xa0000000 as libc::c_uint) as
              *mut u32_0);
    __CartRomHandle.latency =
        (a & 0xff as libc::c_int as libc::c_uint) as u8_0;
    __CartRomHandle.pageSize =
        (a >> 0x10 as libc::c_int & 0xf as libc::c_int as libc::c_uint) as
            u8_0;
    __CartRomHandle.relDuration =
        (a >> 0x14 as libc::c_int & 0xf as libc::c_int as libc::c_uint) as
            u8_0;
    __CartRomHandle.pulse =
        (a >> 8 as libc::c_int & 0xff as libc::c_int as libc::c_uint) as u8_0;
    ::std::ptr::write_volatile((0x4600014 as libc::c_int as libc::c_uint |
                                    0xa0000000 as libc::c_uint) as *mut u32_0,
                               lastLatency);
    ::std::ptr::write_volatile((0x460001c as libc::c_int as libc::c_uint |
                                    0xa0000000 as libc::c_uint) as *mut u32_0,
                               lastPageSize);
    ::std::ptr::write_volatile((0x4600020 as libc::c_int as libc::c_uint |
                                    0xa0000000 as libc::c_uint) as *mut u32_0,
                               lastRelDuration);
    ::std::ptr::write_volatile((0x4600018 as libc::c_int as libc::c_uint |
                                    0xa0000000 as libc::c_uint) as *mut u32_0,
                               lastPulse);
    prevInt = __osDisableInt() as u32_0;
    __CartRomHandle.next = __osPiTable;
    __osPiTable = &mut __CartRomHandle;
    __osRestoreInt(prevInt as s32);
    __osPiRelAccess();
    return &mut __CartRomHandle;
}
