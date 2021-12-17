#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case,
         non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(register_tool)]
extern "C" {
    #[no_mangle]
    static mut __osCurrentHandle: [*mut OSPiHandle; 0];
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
pub unsafe extern "C" fn __osEPiRawReadIo(mut handle: *mut OSPiHandle,
                                          mut devAddr: u32_0,
                                          mut data: *mut u32_0) -> s32 {
    let mut status: s32 = 0;
    let mut curHandle: *mut OSPiHandle = 0 as *mut OSPiHandle;
    loop  {
        status =
            *((0x4600010 as libc::c_int as libc::c_uint |
                   0xa0000000 as libc::c_uint) as *mut u32_0) as s32;
        if !(status & (0x1 as libc::c_int | 0x2 as libc::c_int) != 0) {
            break ;
        }
    }
    if (**__osCurrentHandle.as_mut_ptr().offset((*handle).domain as
                                                    isize)).type_0 as
           libc::c_int != (*handle).type_0 as libc::c_int {
        curHandle =
            *__osCurrentHandle.as_mut_ptr().offset((*handle).domain as isize);
        if (*handle).domain as libc::c_int == 0 as libc::c_int {
            if (*curHandle).latency as libc::c_int !=
                   (*handle).latency as libc::c_int {
                ::std::ptr::write_volatile((0x4600014 as libc::c_int as
                                                libc::c_uint |
                                                0xa0000000 as libc::c_uint) as
                                               *mut u32_0,
                                           (*handle).latency as u32_0)
            }
            if (*curHandle).pageSize as libc::c_int !=
                   (*handle).pageSize as libc::c_int {
                ::std::ptr::write_volatile((0x460001c as libc::c_int as
                                                libc::c_uint |
                                                0xa0000000 as libc::c_uint) as
                                               *mut u32_0,
                                           (*handle).pageSize as u32_0)
            }
            if (*curHandle).relDuration as libc::c_int !=
                   (*handle).relDuration as libc::c_int {
                ::std::ptr::write_volatile((0x4600020 as libc::c_int as
                                                libc::c_uint |
                                                0xa0000000 as libc::c_uint) as
                                               *mut u32_0,
                                           (*handle).relDuration as u32_0)
            }
            if (*curHandle).pulse as libc::c_int !=
                   (*handle).pulse as libc::c_int {
                ::std::ptr::write_volatile((0x4600018 as libc::c_int as
                                                libc::c_uint |
                                                0xa0000000 as libc::c_uint) as
                                               *mut u32_0,
                                           (*handle).pulse as u32_0)
            }
        } else {
            if (*curHandle).latency as libc::c_int !=
                   (*handle).latency as libc::c_int {
                ::std::ptr::write_volatile((0x4600024 as libc::c_int as
                                                libc::c_uint |
                                                0xa0000000 as libc::c_uint) as
                                               *mut u32_0,
                                           (*handle).latency as u32_0)
            }
            if (*curHandle).pageSize as libc::c_int !=
                   (*handle).pageSize as libc::c_int {
                ::std::ptr::write_volatile((0x460002c as libc::c_int as
                                                libc::c_uint |
                                                0xa0000000 as libc::c_uint) as
                                               *mut u32_0,
                                           (*handle).pageSize as u32_0)
            }
            if (*curHandle).relDuration as libc::c_int !=
                   (*handle).relDuration as libc::c_int {
                ::std::ptr::write_volatile((0x4600030 as libc::c_int as
                                                libc::c_uint |
                                                0xa0000000 as libc::c_uint) as
                                               *mut u32_0,
                                           (*handle).relDuration as u32_0)
            }
            if (*curHandle).pulse as libc::c_int !=
                   (*handle).pulse as libc::c_int {
                ::std::ptr::write_volatile((0x4600028 as libc::c_int as
                                                libc::c_uint |
                                                0xa0000000 as libc::c_uint) as
                                               *mut u32_0,
                                           (*handle).pulse as u32_0)
            }
        }
        (*curHandle).type_0 = (*handle).type_0;
        (*curHandle).latency = (*handle).latency;
        (*curHandle).pageSize = (*handle).pageSize;
        (*curHandle).relDuration = (*handle).relDuration;
        (*curHandle).pulse = (*handle).pulse
    }
    *data =
        *(((*handle).baseAddress | devAddr | 0 as libc::c_int as libc::c_uint
               | 0xa0000000 as libc::c_uint) as *mut u32_0);
    return 0 as libc::c_int;
}
