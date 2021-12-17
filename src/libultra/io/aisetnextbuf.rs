#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case,
         non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(register_tool)]
pub type u8_0 = libc::c_uchar;
pub type s32 = libc::c_int;
pub type u32_0 = libc::c_uint;
// ! Note that this is not the same as the original libultra
// ! osAiSetNextBuffer, see comments in the function
#[no_mangle]
pub unsafe extern "C" fn osAiSetNextBuffer(mut buf: *mut libc::c_void,
                                           mut size: u32_0) -> s32 {
    static mut D_80130500: u8_0 = 0 as libc::c_int as u8_0;
    let mut bufAdjusted: u32_0 = buf as u32_0;
    let mut status: s32 = 0;
    if D_80130500 != 0 {
        bufAdjusted =
            (buf as u32_0).wrapping_sub(0x2000 as libc::c_int as libc::c_uint)
    }
    if (buf as u32_0).wrapping_add(size) &
           0x1fff as libc::c_int as libc::c_uint ==
           0 as libc::c_int as libc::c_uint {
        D_80130500 = 1 as libc::c_int as u8_0
    } else { D_80130500 = 0 as libc::c_int as u8_0 }
    // Originally a call to __osAiDeviceBusy
    status =
        *((0x450000c as libc::c_int as libc::c_uint |
               0xa0000000 as libc::c_uint) as *mut s32);
    if status & (1 as libc::c_int) << 31 as libc::c_int != 0 {
        return -(1 as libc::c_int)
    }
    // OS_K0_TO_PHYSICAL replaces osVirtualToPhysical, this replacement
    // assumes that only KSEG0 addresses are given
    ::std::ptr::write_volatile((0x4500000 as libc::c_int as libc::c_uint |
                                    0xa0000000 as libc::c_uint) as *mut u32_0,
                               (bufAdjusted as
                                    *mut libc::c_char).offset(-(0x80000000 as
                                                                    libc::c_uint
                                                                    as isize))
                                   as u32_0);
    ::std::ptr::write_volatile((0x4500004 as libc::c_int as libc::c_uint |
                                    0xa0000000 as libc::c_uint) as *mut u32_0,
                               size);
    return 0 as libc::c_int;
}
