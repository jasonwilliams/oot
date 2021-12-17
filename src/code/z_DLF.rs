#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case,
         non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(register_tool)]
extern "C" {
    #[no_mangle]
    fn osSyncPrintf(fmt: *const libc::c_char, _: ...);
    #[no_mangle]
    fn Overlay_AllocateAndLoad(vRomStart: u32_0, vRomEnd: u32_0,
                               vRamStart: *mut libc::c_void,
                               vRamEnd: *mut libc::c_void)
     -> *mut libc::c_void;
    #[no_mangle]
    fn SystemArena_FreeDebug(ptr: *mut libc::c_void,
                             file: *const libc::c_char, line: s32);
}
pub type s32 = libc::c_int;
pub type u32_0 = libc::c_uint;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct GameStateOverlay {
    pub loadedRamAddr: *mut libc::c_void,
    pub vromStart: u32_0,
    pub vromEnd: u32_0,
    pub vramStart: *mut libc::c_void,
    pub vramEnd: *mut libc::c_void,
    pub unk_14: *mut libc::c_void,
    pub init: *mut libc::c_void,
    pub destroy: *mut libc::c_void,
    pub unk_20: *mut libc::c_void,
    pub unk_24: *mut libc::c_void,
    pub unk_28: s32,
    pub instanceSize: u32_0,
}
#[no_mangle]
pub unsafe extern "C" fn Overlay_LoadGameState(mut overlayEntry:
                                                   *mut GameStateOverlay) {
    if !(*overlayEntry).loadedRamAddr.is_null() {
        osSyncPrintf(b"\xe6\x97\xa2\xe3\x81\xab\xe3\x83\xaa\xe3\x83\xb3\xe3\x82\xaf\xe3\x81\x95\xe3\x82\x8c\xe3\x81\xa6\xe3\x81\x84\xe3\x81\xbe\xe3\x81\x99\n\x00"
                         as *const u8 as
                         *const libc::c_char); // "Already linked"
        return
    } // "Loading failed"
    if (*overlayEntry).vramStart.is_null() {
        (*overlayEntry).unk_28 = 0 as libc::c_int
    } else {
        (*overlayEntry).loadedRamAddr =
            Overlay_AllocateAndLoad((*overlayEntry).vromStart,
                                    (*overlayEntry).vromEnd,
                                    (*overlayEntry).vramStart,
                                    (*overlayEntry).vramEnd);
        if (*overlayEntry).loadedRamAddr.is_null() {
            osSyncPrintf(b"\xe3\x83\xad\xe3\x83\xbc\xe3\x83\x89\xe3\x81\xab\xe5\xa4\xb1\xe6\x95\x97\xe3\x81\x97\xe3\x81\xbe\xe3\x81\x97\xe3\x81\x9f\n\x00"
                             as *const u8 as *const libc::c_char);
            return
        }
        osSyncPrintf(b"\x1b[32m\x00" as *const u8 as *const libc::c_char);
        osSyncPrintf(b"OVL(d):Seg:%08x-%08x Ram:%08x-%08x Off:%08x %s\n\x00"
                         as *const u8 as *const libc::c_char,
                     (*overlayEntry).vramStart, (*overlayEntry).vramEnd,
                     (*overlayEntry).loadedRamAddr,
                     ((*overlayEntry).loadedRamAddr as
                          u32_0).wrapping_add((*overlayEntry).vramEnd as
                                                  u32_0).wrapping_sub((*overlayEntry).vramStart
                                                                          as
                                                                          u32_0),
                     ((*overlayEntry).vramStart as
                          u32_0).wrapping_sub((*overlayEntry).loadedRamAddr as
                                                  u32_0),
                     b"\x00" as *const u8 as *const libc::c_char);
        osSyncPrintf(b"\x1b[m\x00" as *const u8 as *const libc::c_char);
        if !(*overlayEntry).unk_14.is_null() {
            (*overlayEntry).unk_14 =
                ((*overlayEntry).unk_14 as
                     u32_0).wrapping_sub(((*overlayEntry).vramStart as
                                              u32_0).wrapping_sub((*overlayEntry).loadedRamAddr
                                                                      as
                                                                      u32_0)
                                             as s32 as libc::c_uint) as
                    *mut libc::c_void
        } else { (*overlayEntry).unk_14 = 0 as *mut libc::c_void }
        if !(*overlayEntry).init.is_null() {
            (*overlayEntry).init =
                ((*overlayEntry).init as
                     u32_0).wrapping_sub(((*overlayEntry).vramStart as
                                              u32_0).wrapping_sub((*overlayEntry).loadedRamAddr
                                                                      as
                                                                      u32_0)
                                             as s32 as libc::c_uint) as
                    *mut libc::c_void
        } else { (*overlayEntry).init = 0 as *mut libc::c_void }
        if !(*overlayEntry).destroy.is_null() {
            (*overlayEntry).destroy =
                ((*overlayEntry).destroy as
                     u32_0).wrapping_sub(((*overlayEntry).vramStart as
                                              u32_0).wrapping_sub((*overlayEntry).loadedRamAddr
                                                                      as
                                                                      u32_0)
                                             as s32 as libc::c_uint) as
                    *mut libc::c_void
        } else { (*overlayEntry).destroy = 0 as *mut libc::c_void }
        if !(*overlayEntry).unk_20.is_null() {
            (*overlayEntry).unk_20 =
                ((*overlayEntry).unk_20 as
                     u32_0).wrapping_sub(((*overlayEntry).vramStart as
                                              u32_0).wrapping_sub((*overlayEntry).loadedRamAddr
                                                                      as
                                                                      u32_0)
                                             as s32 as libc::c_uint) as
                    *mut libc::c_void
        } else { (*overlayEntry).unk_20 = 0 as *mut libc::c_void }
        if !(*overlayEntry).unk_24.is_null() {
            (*overlayEntry).unk_24 =
                ((*overlayEntry).unk_24 as
                     u32_0).wrapping_sub(((*overlayEntry).vramStart as
                                              u32_0).wrapping_sub((*overlayEntry).loadedRamAddr
                                                                      as
                                                                      u32_0)
                                             as s32 as libc::c_uint) as
                    *mut libc::c_void
        } else { (*overlayEntry).unk_24 = 0 as *mut libc::c_void }
        (*overlayEntry).unk_28 = 0 as libc::c_int
    };
}
#[no_mangle]
pub unsafe extern "C" fn Overlay_FreeGameState(mut overlayEntry:
                                                   *mut GameStateOverlay) {
    if !(*overlayEntry).loadedRamAddr.is_null() {
        let mut temp: s32 =
            if (*overlayEntry).unk_28 != 0 as libc::c_int {
                -(1 as libc::c_int)
            } else { 0 as libc::c_int };
        if temp == 0 as libc::c_int {
            if !(*overlayEntry).unk_14.is_null() {
                (*overlayEntry).unk_14 =
                    ((*overlayEntry).unk_14 as
                         u32_0).wrapping_add(((*overlayEntry).vramStart as
                                                  u32_0).wrapping_sub((*overlayEntry).loadedRamAddr
                                                                          as
                                                                          u32_0)
                                                 as s32 as libc::c_uint) as
                        *mut libc::c_void
            } else { (*overlayEntry).unk_14 = 0 as *mut libc::c_void }
            if !(*overlayEntry).init.is_null() {
                (*overlayEntry).init =
                    ((*overlayEntry).init as
                         u32_0).wrapping_add(((*overlayEntry).vramStart as
                                                  u32_0).wrapping_sub((*overlayEntry).loadedRamAddr
                                                                          as
                                                                          u32_0)
                                                 as s32 as libc::c_uint) as
                        *mut libc::c_void
            } else { (*overlayEntry).init = 0 as *mut libc::c_void }
            if !(*overlayEntry).destroy.is_null() {
                (*overlayEntry).destroy =
                    ((*overlayEntry).destroy as
                         u32_0).wrapping_add(((*overlayEntry).vramStart as
                                                  u32_0).wrapping_sub((*overlayEntry).loadedRamAddr
                                                                          as
                                                                          u32_0)
                                                 as s32 as libc::c_uint) as
                        *mut libc::c_void
            } else { (*overlayEntry).destroy = 0 as *mut libc::c_void }
            if !(*overlayEntry).unk_20.is_null() {
                (*overlayEntry).unk_20 =
                    ((*overlayEntry).unk_20 as
                         u32_0).wrapping_add(((*overlayEntry).vramStart as
                                                  u32_0).wrapping_sub((*overlayEntry).loadedRamAddr
                                                                          as
                                                                          u32_0)
                                                 as s32 as libc::c_uint) as
                        *mut libc::c_void
            } else { (*overlayEntry).unk_20 = 0 as *mut libc::c_void }
            if !(*overlayEntry).unk_24.is_null() {
                (*overlayEntry).unk_24 =
                    ((*overlayEntry).unk_24 as
                         u32_0).wrapping_add(((*overlayEntry).vramStart as
                                                  u32_0).wrapping_sub((*overlayEntry).loadedRamAddr
                                                                          as
                                                                          u32_0)
                                                 as s32 as libc::c_uint) as
                        *mut libc::c_void
            } else { (*overlayEntry).unk_24 = 0 as *mut libc::c_void }
            SystemArena_FreeDebug((*overlayEntry).loadedRamAddr,
                                  b"../z_DLF.c\x00" as *const u8 as
                                      *const libc::c_char,
                                  149 as libc::c_int);
            (*overlayEntry).loadedRamAddr = 0 as *mut libc::c_void
        }
    };
}
