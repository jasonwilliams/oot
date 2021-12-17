#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case,
         non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(register_tool)]
extern "C" {
    #[no_mangle]
    static mut D_80009320: [u8_0; 0];
    #[no_mangle]
    static mut D_800093F0: [u8_0; 0];
    #[no_mangle]
    static mut D_80155F50: [u8_0; 0];
    #[no_mangle]
    static mut D_80157580: [u8_0; 0];
}
pub type u8_0 = libc::c_uchar;
pub type u32_0 = libc::c_uint;
// Initialized in run_static_initializers
#[no_mangle]
pub static mut D_8012DBA0: u32_0 = 0;
// Initialized in run_static_initializers
#[no_mangle]
pub static mut D_8012DBA4: u32_0 = 0;
#[no_mangle]
pub unsafe extern "C" fn SysUcode_GetUCodeBoot() -> u32_0 {
    return &mut D_80009320 as *mut [u8_0; 0] as u32_0;
}
#[no_mangle]
pub unsafe extern "C" fn SysUcode_GetUCodeBootSize() -> u32_0 {
    return (&mut D_800093F0 as *mut [u8_0; 0] as
                u32_0).wrapping_sub(&mut D_80009320 as *mut [u8_0; 0] as
                                        u32_0);
}
#[no_mangle]
pub unsafe extern "C" fn SysUcode_GetUCode() -> u32_0 { return D_8012DBA0; }
#[no_mangle]
pub unsafe extern "C" fn SysUcode_GetUCodeData() -> u32_0 {
    return D_8012DBA4;
}
unsafe extern "C" fn run_static_initializers() {
    D_8012DBA0 = &mut D_80155F50 as *mut [u8_0; 0] as u32_0;
    D_8012DBA4 = &mut D_80157580 as *mut [u8_0; 0] as u32_0
}
#[used]
#[cfg_attr(target_os = "linux", link_section = ".init_array")]
#[cfg_attr(target_os = "windows", link_section = ".CRT$XIB")]
#[cfg_attr(target_os = "macos", link_section = "__DATA,__mod_init_func")]
static INIT_ARRAY: [unsafe extern "C" fn(); 1] = [run_static_initializers];
