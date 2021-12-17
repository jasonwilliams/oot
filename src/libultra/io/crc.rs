#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case,
         non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(register_tool)]
pub type u8_0 = libc::c_uchar;
pub type u16_0 = libc::c_ushort;
pub type s32 = libc::c_int;
pub type u32_0 = libc::c_uint;
// Valid addr up to 0x7FF
// It's the address of a block of 0x20 bytes in the mempak
// So that means the whole mempak has a 16-bit address space
#[no_mangle]
pub unsafe extern "C" fn __osContAddressCrc(mut addr: u16_0) -> u8_0 {
    let mut addr32: u32_0 = addr as u32_0;
    let mut ret: u32_0 = 0 as libc::c_int as u32_0;
    let mut bit: u32_0 = 0;
    let mut i: s32 = 0;
    bit = 0x400 as libc::c_int as u32_0;
    while bit != 0 {
        ret <<= 1 as libc::c_int;
        if addr32 & bit != 0 {
            if ret & 0x20 as libc::c_int as libc::c_uint != 0 {
                ret ^= 0x14 as libc::c_int as libc::c_uint
            } else { ret = ret.wrapping_add(1) }
        } else if ret & 0x20 as libc::c_int as libc::c_uint != 0 {
            ret ^= 0x15 as libc::c_int as libc::c_uint
        }
        bit >>= 1 as libc::c_int
    }
    i = 0 as libc::c_int;
    while i < 5 as libc::c_int {
        ret <<= 1 as libc::c_int;
        if ret & 0x20 as libc::c_int as libc::c_uint != 0 {
            ret ^= 0x15 as libc::c_int as libc::c_uint
        }
        i += 1
    }
    return (ret & 0x1f as libc::c_int as libc::c_uint) as u8_0;
}
#[no_mangle]
pub unsafe extern "C" fn __osContDataCrc(mut data: *mut u8_0) -> u8_0 {
    let mut ret: s32 = 0 as libc::c_int;
    let mut bit: u32_0 = 0;
    let mut byte: u32_0 = 0;
    byte = 0x20 as libc::c_int as u32_0;
    while byte != 0 {
        bit = 0x80 as libc::c_int as u32_0;
        while bit != 0 {
            ret <<= 1 as libc::c_int;
            if *data as libc::c_uint & bit != 0 as libc::c_int as libc::c_uint
               {
                if ret & 0x100 as libc::c_int != 0 as libc::c_int {
                    ret ^= 0x84 as libc::c_int
                } else { ret += 1 }
            } else if ret & 0x100 as libc::c_int != 0 {
                ret ^= 0x85 as libc::c_int
            }
            bit >>= 1 as libc::c_int
        }
        byte = byte.wrapping_sub(1);
        data = data.offset(1)
    }
    loop  {
        ret <<= 1 as libc::c_int;
        if ret & 0x100 as libc::c_int != 0 { ret ^= 0x85 as libc::c_int }
        byte = byte.wrapping_add(1);
        if !(byte < 8 as libc::c_uint) { break ; }
    }
    return ret as u8_0;
}
