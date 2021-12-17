#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case,
         non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(register_tool)]
extern "C" {
    #[no_mangle]
    fn osSyncPrintf(fmt: *const libc::c_char, _: ...);
    #[no_mangle]
    fn LogUtils_HungupThread(name: *const libc::c_char, line: s32);
    #[no_mangle]
    static mut osMemSize: u32_0;
}
pub type s32 = libc::c_int;
pub type u32_0 = libc::c_uint;
#[no_mangle]
pub static mut sSysCfbFbPtr: [u32_0; 2] = [0; 2];
#[no_mangle]
pub static mut sSysCfbEnd: u32_0 = 0;
#[no_mangle]
pub unsafe extern "C" fn SysCfb_Init(mut n64dd: s32) {
    let mut screenSize: u32_0 = 0;
    let mut tmpFbEnd: u32_0 = 0;
    if osMemSize >= 0x800000 as libc::c_int as libc::c_uint {
        // "8MB or more memory is installed"
        osSyncPrintf(b"\xef\xbc\x98\xef\xbc\xad\xe3\x83\x90\xe3\x82\xa4\xe3\x83\x88\xe4\xbb\xa5\xe4\xb8\x8a\xe3\x81\xae\xe3\x83\xa1\xe3\x83\xa2\xe3\x83\xaa\xe3\x81\x8c\xe6\x90\xad\xe8\xbc\x89\xe3\x81\x95\xe3\x82\x8c\xe3\x81\xa6\xe3\x81\x84\xe3\x81\xbe\xe3\x81\x99\n\x00"
                         as *const u8 as
                         *const libc::c_char); // "RAM 8M mode (N64DD compatible)"
        tmpFbEnd = 0x8044be80 as libc::c_uint;
        if n64dd == 1 as libc::c_int {
            osSyncPrintf(b"RAM 8M mode (N64DD\xe5\xaf\xbe\xe5\xbf\x9c)\n\x00"
                             as *const u8 as *const libc::c_char);
            sSysCfbEnd = 0x805fb000 as libc::c_uint
        } else {
            // "The margin for this version is %dK bytes"
            osSyncPrintf(b"\xe3\x81\x93\xe3\x81\xae\xe3\x83\x90\xe3\x83\xbc\xe3\x82\xb8\xe3\x83\xa7\xe3\x83\xb3\xe3\x81\xae\xe3\x83\x9e\xe3\x83\xbc\xe3\x82\xb8\xe3\x83\xb3\xe3\x81\xaf %dK \xe3\x83\x90\xe3\x82\xa4\xe3\x83\x88\xe3\x81\xa7\xe3\x81\x99\n\x00"
                             as *const u8 as *const libc::c_char,
                         0x4bc00 as libc::c_int / 1024 as libc::c_int);
            sSysCfbEnd = tmpFbEnd
        }
    } else if osMemSize >= 0x400000 as libc::c_int as libc::c_uint {
        osSyncPrintf(b"RAM4M mode\n\x00" as *const u8 as *const libc::c_char);
        sSysCfbEnd = 0x80400000 as libc::c_uint
    } else {
        LogUtils_HungupThread(b"../sys_cfb.c\x00" as *const u8 as
                                  *const libc::c_char, 354 as libc::c_int);
    }
    screenSize = (320 as libc::c_int * 240 as libc::c_int) as u32_0;
    sSysCfbEnd &= !(0x3f as libc::c_int) as libc::c_uint;
    // "The final address used by the system is %08x"
    osSyncPrintf(b"\xe3\x82\xb7\xe3\x82\xb9\xe3\x83\x86\xe3\x83\xa0\xe3\x81\x8c\xe4\xbd\xbf\xe7\x94\xa8\xe3\x81\x99\xe3\x82\x8b\xe6\x9c\x80\xe7\xb5\x82\xe3\x82\xa2\xe3\x83\x89\xe3\x83\xac\xe3\x82\xb9\xe3\x81\xaf %08x \xe3\x81\xa7\xe3\x81\x99\n\x00"
                     as *const u8 as *const libc::c_char, sSysCfbEnd);
    sSysCfbFbPtr[0 as libc::c_int as usize] =
        sSysCfbEnd.wrapping_sub(screenSize.wrapping_mul(4 as libc::c_int as
                                                            libc::c_uint));
    sSysCfbFbPtr[1 as libc::c_int as usize] =
        sSysCfbEnd.wrapping_sub(screenSize.wrapping_mul(2 as libc::c_int as
                                                            libc::c_uint));
    // "Frame buffer addresses are %08x and %08x"
    osSyncPrintf(b"\xe3\x83\x95\xe3\x83\xac\xe3\x83\xbc\xe3\x83\xa0\xe3\x83\x90\xe3\x83\x83\xe3\x83\x95\xe3\x82\xa1\xe3\x81\xae\xe3\x82\xa2\xe3\x83\x89\xe3\x83\xac\xe3\x82\xb9\xe3\x81\xaf %08x \xe3\x81\xa8 %08x \xe3\x81\xa7\xe3\x81\x99\n\x00"
                     as *const u8 as *const libc::c_char,
                 sSysCfbFbPtr[0 as libc::c_int as usize],
                 sSysCfbFbPtr[1 as libc::c_int as usize]);
}
#[no_mangle]
pub unsafe extern "C" fn SysCfb_Reset() {
    sSysCfbFbPtr[0 as libc::c_int as usize] = 0 as libc::c_int as u32_0;
    sSysCfbFbPtr[1 as libc::c_int as usize] = 0 as libc::c_int as u32_0;
    sSysCfbEnd = 0 as libc::c_int as u32_0;
}
#[no_mangle]
pub unsafe extern "C" fn SysCfb_GetFbPtr(mut idx: s32) -> u32_0 {
    if idx < 2 as libc::c_int { return sSysCfbFbPtr[idx as usize] }
    return 0 as libc::c_int as u32_0;
}
#[no_mangle]
pub unsafe extern "C" fn SysCfb_GetFbEnd() -> u32_0 { return sSysCfbEnd; }
