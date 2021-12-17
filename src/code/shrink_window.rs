#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case,
         non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(const_raw_ptr_to_usize_cast, register_tool)]
extern "C" {
    #[no_mangle]
    fn osSyncPrintf(fmt: *const libc::c_char, _: ...);
    #[no_mangle]
    static mut gGameInfo: *mut GameInfo;
}
pub type s16 = libc::c_short;
pub type s32 = libc::c_int;
pub type u32_0 = libc::c_uint;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct GameInfo {
    pub regPage: s32,
    pub regGroup: s32,
    pub regCur: s32,
    pub dpadLast: s32,
    pub repeat: s32,
    pub data: [s16; 2784],
}
#[no_mangle]
pub static mut D_8012CED0: s32 = 0 as libc::c_int;
#[no_mangle]
pub static mut sShrinkWindowVal: s32 = 0 as libc::c_int;
#[no_mangle]
pub static mut sShrinkWindowCurrentVal: s32 = 0 as libc::c_int;
#[no_mangle]
pub unsafe extern "C" fn ShrinkWindow_SetVal(mut value: s32) {
    if (*gGameInfo).data[(21 as libc::c_int * 6 as libc::c_int *
                              16 as libc::c_int + 80 as libc::c_int) as usize]
           as libc::c_int == 0x13 as libc::c_int &&
           (*gGameInfo).data[(21 as libc::c_int * 6 as libc::c_int *
                                  16 as libc::c_int + 81 as libc::c_int) as
                                 usize] as libc::c_int == 1 as libc::c_int {
        osSyncPrintf(b"shrink_window_setval(%d)\n\x00" as *const u8 as
                         *const libc::c_char, value);
    }
    sShrinkWindowVal = value;
}
#[no_mangle]
pub unsafe extern "C" fn ShrinkWindow_GetVal() -> u32_0 {
    return sShrinkWindowVal as u32_0;
}
#[no_mangle]
pub unsafe extern "C" fn ShrinkWindow_SetCurrentVal(mut currentVal: s32) {
    if (*gGameInfo).data[(21 as libc::c_int * 6 as libc::c_int *
                              16 as libc::c_int + 80 as libc::c_int) as usize]
           as libc::c_int == 0x13 as libc::c_int &&
           (*gGameInfo).data[(21 as libc::c_int * 6 as libc::c_int *
                                  16 as libc::c_int + 81 as libc::c_int) as
                                 usize] as libc::c_int == 1 as libc::c_int {
        osSyncPrintf(b"shrink_window_setnowval(%d)\n\x00" as *const u8 as
                         *const libc::c_char, currentVal);
    }
    sShrinkWindowCurrentVal = currentVal;
}
#[no_mangle]
pub unsafe extern "C" fn ShrinkWindow_GetCurrentVal() -> u32_0 {
    return sShrinkWindowCurrentVal as u32_0;
}
#[no_mangle]
pub unsafe extern "C" fn ShrinkWindow_Init() {
    if (*gGameInfo).data[(21 as libc::c_int * 6 as libc::c_int *
                              16 as libc::c_int + 80 as libc::c_int) as usize]
           as libc::c_int == 0x13 as libc::c_int &&
           (*gGameInfo).data[(21 as libc::c_int * 6 as libc::c_int *
                                  16 as libc::c_int + 81 as libc::c_int) as
                                 usize] as libc::c_int == 1 as libc::c_int {
        osSyncPrintf(b"shrink_window_init()\n\x00" as *const u8 as
                         *const libc::c_char);
    }
    D_8012CED0 = 0 as libc::c_int;
    sShrinkWindowVal = 0 as libc::c_int;
    sShrinkWindowCurrentVal = 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn ShrinkWindow_Destroy() {
    if (*gGameInfo).data[(21 as libc::c_int * 6 as libc::c_int *
                              16 as libc::c_int + 80 as libc::c_int) as usize]
           as libc::c_int == 0x13 as libc::c_int &&
           (*gGameInfo).data[(21 as libc::c_int * 6 as libc::c_int *
                                  16 as libc::c_int + 81 as libc::c_int) as
                                 usize] as libc::c_int == 1 as libc::c_int {
        osSyncPrintf(b"shrink_window_cleanup()\n\x00" as *const u8 as
                         *const libc::c_char);
    }
    sShrinkWindowCurrentVal = 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn ShrinkWindow_Update(mut updateRate: s32) {
    let mut off: s32 = 0;
    if updateRate == 3 as libc::c_int {
        off = 10 as libc::c_int
    } else { off = 30 as libc::c_int / updateRate }
    if sShrinkWindowCurrentVal < sShrinkWindowVal {
        if D_8012CED0 != 1 as libc::c_int { D_8012CED0 = 1 as libc::c_int }
        if sShrinkWindowCurrentVal + off < sShrinkWindowVal {
            sShrinkWindowCurrentVal += off
        } else { sShrinkWindowCurrentVal = sShrinkWindowVal }
    } else if sShrinkWindowVal < sShrinkWindowCurrentVal {
        if D_8012CED0 != 2 as libc::c_int { D_8012CED0 = 2 as libc::c_int }
        if sShrinkWindowVal < sShrinkWindowCurrentVal - off {
            sShrinkWindowCurrentVal -= off
        } else { sShrinkWindowCurrentVal = sShrinkWindowVal }
    } else { D_8012CED0 = 0 as libc::c_int }
    if (*gGameInfo).data[(21 as libc::c_int * 6 as libc::c_int *
                              16 as libc::c_int + 80 as libc::c_int) as usize]
           as libc::c_int == 0x13 as libc::c_int {
        if (*gGameInfo).data[(21 as libc::c_int * 6 as libc::c_int *
                                  16 as libc::c_int + 94 as libc::c_int) as
                                 usize] as libc::c_int != 0x13 as libc::c_int
           {
            (*gGameInfo).data[(21 as libc::c_int * 6 as libc::c_int *
                                   16 as libc::c_int + 94 as libc::c_int) as
                                  usize] = 0x13 as libc::c_int as s16;
            (*gGameInfo).data[(21 as libc::c_int * 6 as libc::c_int *
                                   16 as libc::c_int + 81 as libc::c_int) as
                                  usize] = 0 as libc::c_int as s16;
            (*gGameInfo).data[(21 as libc::c_int * 6 as libc::c_int *
                                   16 as libc::c_int + 82 as libc::c_int) as
                                  usize] = 0 as libc::c_int as s16;
            (*gGameInfo).data[(21 as libc::c_int * 6 as libc::c_int *
                                   16 as libc::c_int + 83 as libc::c_int) as
                                  usize] = 0 as libc::c_int as s16;
            (*gGameInfo).data[(21 as libc::c_int * 6 as libc::c_int *
                                   16 as libc::c_int + 84 as libc::c_int) as
                                  usize] = 0 as libc::c_int as s16;
            (*gGameInfo).data[(21 as libc::c_int * 6 as libc::c_int *
                                   16 as libc::c_int + 85 as libc::c_int) as
                                  usize] = 0 as libc::c_int as s16;
            (*gGameInfo).data[(21 as libc::c_int * 6 as libc::c_int *
                                   16 as libc::c_int + 86 as libc::c_int) as
                                  usize] = 0 as libc::c_int as s16;
            (*gGameInfo).data[(21 as libc::c_int * 6 as libc::c_int *
                                   16 as libc::c_int + 87 as libc::c_int) as
                                  usize] = 0 as libc::c_int as s16;
            (*gGameInfo).data[(21 as libc::c_int * 6 as libc::c_int *
                                   16 as libc::c_int + 88 as libc::c_int) as
                                  usize] = 0 as libc::c_int as s16;
            (*gGameInfo).data[(21 as libc::c_int * 6 as libc::c_int *
                                   16 as libc::c_int + 89 as libc::c_int) as
                                  usize] = 0 as libc::c_int as s16
        }
        (*gGameInfo).data[(21 as libc::c_int * 6 as libc::c_int *
                               16 as libc::c_int + 83 as libc::c_int) as
                              usize] = D_8012CED0 as s16;
        (*gGameInfo).data[(21 as libc::c_int * 6 as libc::c_int *
                               16 as libc::c_int + 84 as libc::c_int) as
                              usize] = sShrinkWindowCurrentVal as s16;
        (*gGameInfo).data[(21 as libc::c_int * 6 as libc::c_int *
                               16 as libc::c_int + 85 as libc::c_int) as
                              usize] = sShrinkWindowVal as s16;
        (*gGameInfo).data[(21 as libc::c_int * 6 as libc::c_int *
                               16 as libc::c_int + 86 as libc::c_int) as
                              usize] = off as s16
    };
}
