#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case,
         non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(register_tool)]
pub type s32 = libc::c_int;
#[no_mangle]
pub static mut gOverlayLogSeverity: s32 = 2 as libc::c_int;
