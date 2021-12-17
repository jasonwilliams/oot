#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case,
         non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(const_transmute, register_tool)]
#[no_mangle]
pub static mut gBuildTeam: [libc::c_char; 14] =
    unsafe {
        *::std::mem::transmute::<&[u8; 14],
                                 &[libc::c_char; 14]>(b"zelda@srd022j\x00")
    };
#[no_mangle]
pub static mut gBuildDate: [libc::c_char; 18] =
    unsafe {
        *::std::mem::transmute::<&[u8; 18],
                                 &[libc::c_char; 18]>(b"03-02-21 00:16:31\x00")
    };
#[no_mangle]
pub static mut gBuildMakeOption: [libc::c_char; 1] =
    unsafe {
        *::std::mem::transmute::<&[u8; 1], &[libc::c_char; 1]>(b"\x00")
    };
