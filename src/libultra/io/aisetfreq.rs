#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case,
         non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(register_tool)]
extern "C" {
    #[no_mangle]
    static mut osViClock: s32;
}
pub type u8_0 = libc::c_uchar;
pub type s32 = libc::c_int;
pub type u32_0 = libc::c_uint;
pub type f32_0 = libc::c_float;
#[no_mangle]
pub unsafe extern "C" fn osAiSetFrequency(mut frequency: u32_0) -> s32 {
    let mut bitrate: u8_0 = 0;
    let mut dacRateF: f32_0 =
        osViClock as f32_0 / frequency as libc::c_float + 0.5f32;
    let mut dacRate: u32_0 = dacRateF as u32_0;
    if dacRate < 132 as libc::c_int as libc::c_uint {
        return -(1 as libc::c_int)
    }
    bitrate = dacRate.wrapping_div(66 as libc::c_int as libc::c_uint) as u8_0;
    if bitrate as libc::c_int > 16 as libc::c_int {
        bitrate = 16 as libc::c_int as u8_0
    }
    ::std::ptr::write_volatile((0x4500010 as libc::c_int as libc::c_uint |
                                    0xa0000000 as libc::c_uint) as *mut u32_0,
                               dacRate.wrapping_sub(1 as libc::c_int as
                                                        libc::c_uint));
    ::std::ptr::write_volatile((0x4500014 as libc::c_int as libc::c_uint |
                                    0xa0000000 as libc::c_uint) as *mut u32_0,
                               (bitrate as libc::c_int - 1 as libc::c_int) as
                                   u32_0);
    return osViClock / dacRate as s32;
}
