#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case,
         non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(register_tool)]
extern "C" {
    #[no_mangle]
    fn osVirtualToPhysical(vaddr: *mut libc::c_void) -> u32_0;
    #[no_mangle]
    static mut osRomBase: u32_0;
}
pub type s32 = libc::c_int;
pub type u32_0 = libc::c_uint;
pub type size_t = libc::c_ulong;
#[no_mangle]
pub unsafe extern "C" fn __osPiRawStartDma(mut dir: s32, mut cartAddr: u32_0,
                                           mut dramAddr: *mut libc::c_void,
                                           mut size: size_t) -> s32 {
    let mut status: s32 =
        *((0x4600010 as libc::c_int as libc::c_uint |
               0xa0000000 as libc::c_uint) as *mut u32_0) as s32;
    while status & (0x1 as libc::c_int | 0x2 as libc::c_int) != 0 {
        status =
            *((0x4600010 as libc::c_int as libc::c_uint |
                   0xa0000000 as libc::c_uint) as *mut u32_0) as s32
    }
    let ref mut fresh0 =
        *((0x4600000 as libc::c_int as libc::c_uint |
               0xa0000000 as libc::c_uint) as *mut *mut libc::c_void);
    *fresh0 =
        osVirtualToPhysical(dramAddr) as *mut libc::c_void as
            *mut libc::c_void;
    let ref mut fresh1 =
        *((0x4600004 as libc::c_int as libc::c_uint |
               0xa0000000 as libc::c_uint) as *mut *mut libc::c_void);
    *fresh1 =
        ((osRomBase | cartAddr) & 0x1fffffff as libc::c_int as libc::c_uint)
            as *mut libc::c_void as *mut libc::c_void;
    match dir {
        0 => {
            ::std::ptr::write_volatile((0x460000c as libc::c_int as
                                            libc::c_uint |
                                            0xa0000000 as libc::c_uint) as
                                           *mut u32_0,
                                       size.wrapping_sub(1 as libc::c_int as
                                                             libc::c_ulong) as
                                           u32_0)
        }
        1 => {
            ::std::ptr::write_volatile((0x4600008 as libc::c_int as
                                            libc::c_uint |
                                            0xa0000000 as libc::c_uint) as
                                           *mut u32_0,
                                       size.wrapping_sub(1 as libc::c_int as
                                                             libc::c_ulong) as
                                           u32_0)
        }
        _ => { return -(1 as libc::c_int) }
    }
    return 0 as libc::c_int;
}
