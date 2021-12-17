#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case,
         non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(register_tool)]
extern "C" {
    #[no_mangle]
    fn osSyncPrintf(fmt: *const libc::c_char, _: ...);
    #[no_mangle]
    fn Overlay_Load(vRomStart: u32_0, vRomEnd: u32_0,
                    vRamStart: *mut libc::c_void, vRamEnd: *mut libc::c_void,
                    allocatedVRamAddress: *mut libc::c_void) -> s32;
    #[no_mangle]
    fn SystemArena_MallocRDebug(size: u32_0, file: *const libc::c_char,
                                line: s32) -> *mut libc::c_void;
    #[no_mangle]
    static mut gOverlayLogSeverity: s32;
}
pub type s32 = libc::c_int;
pub type u32_0 = libc::c_uint;
#[no_mangle]
pub unsafe extern "C" fn Overlay_AllocateAndLoad(mut vRomStart: u32_0,
                                                 mut vRomEnd: u32_0,
                                                 mut vRamStart:
                                                     *mut libc::c_void,
                                                 mut vRamEnd:
                                                     *mut libc::c_void)
 -> *mut libc::c_void {
    let mut allocatedVRamAddr: *mut libc::c_void =
        SystemArena_MallocRDebug((vRamEnd as s32 - vRamStart as s32) as u32_0,
                                 b"../loadfragment2.c\x00" as *const u8 as
                                     *const libc::c_char, 31 as libc::c_int);
    if gOverlayLogSeverity >= 3 as libc::c_int {
        osSyncPrintf(b"OVL:SPEC(%08x-%08x) REAL(%08x-%08x) OFFSET(%08x)\n\x00"
                         as *const u8 as *const libc::c_char, vRamStart,
                     vRamEnd, allocatedVRamAddr,
                     (vRamEnd as
                          u32_0).wrapping_sub(vRamStart as
                                                  u32_0).wrapping_add(allocatedVRamAddr
                                                                          as
                                                                          u32_0),
                     (vRamStart as
                          u32_0).wrapping_sub(allocatedVRamAddr as u32_0));
    }
    if !allocatedVRamAddr.is_null() {
        Overlay_Load(vRomStart, vRomEnd, vRamStart, vRamEnd,
                     allocatedVRamAddr);
    }
    return allocatedVRamAddr;
}
