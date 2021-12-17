#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case,
         non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(const_raw_ptr_to_usize_cast, register_tool)]
extern "C" {
    #[no_mangle]
    fn osSyncPrintf(fmt: *const libc::c_char, _: ...);
    #[no_mangle]
    fn LogUtils_LogThreadId(name: *const libc::c_char, line: s32);
    #[no_mangle]
    static mut osViModeFpalLan1: OSViMode;
    #[no_mangle]
    static mut osViModePalLan1: OSViMode;
    #[no_mangle]
    static mut gGameInfo: *mut GameInfo;
    #[no_mangle]
    static mut osTvType: u32_0;
    #[no_mangle]
    static mut osViModeNtscLan1: OSViMode;
    #[no_mangle]
    static mut gScreenHeight: s32;
    #[no_mangle]
    static mut gScreenWidth: s32;
}
pub type s8 = libc::c_schar;
pub type u8_0 = libc::c_uchar;
pub type s16 = libc::c_short;
pub type u16_0 = libc::c_ushort;
pub type s32 = libc::c_int;
pub type u32_0 = libc::c_uint;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct OSViCommonRegs {
    pub ctrl: u32_0,
    pub width: u32_0,
    pub burst: u32_0,
    pub vSync: u32_0,
    pub hSync: u32_0,
    pub leap: u32_0,
    pub hStart: u32_0,
    pub xScale: u32_0,
    pub vCurrent: u32_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct OSViFieldRegs {
    pub origin: u32_0,
    pub yScale: u32_0,
    pub vStart: u32_0,
    pub vBurst: u32_0,
    pub vIntr: u32_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct OSViMode {
    pub type_0: u8_0,
    pub comRegs: OSViCommonRegs,
    pub fldRegs: [OSViFieldRegs; 2],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct OSContPad {
    pub button: u16_0,
    pub stick_x: s8,
    pub stick_y: s8,
    pub errno: u8_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Input {
    pub cur: OSContPad,
    pub prev: OSContPad,
    pub press: OSContPad,
    pub rel: OSContPad,
}
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ViMode {
    pub customViMode: OSViMode,
    pub viHeight: s32,
    pub viWidth: s32,
    pub unk_58: s32,
    pub unk_5C: s32,
    pub unk_60: s32,
    pub unk_64: s32,
    pub viModeBase: s32,
    pub viTvType: s32,
    pub unk_70: u32_0,
    pub unk_74: u32_0,
    pub unk_78: u32_0,
    pub unk_7C: u32_0,
    pub viFeatures: u32_0,
    pub unk_84: u32_0,
}
#[no_mangle]
pub unsafe extern "C" fn ViMode_LogPrint(mut osViMode: *mut OSViMode) {
    LogUtils_LogThreadId(b"../z_vimode.c\x00" as *const u8 as
                             *const libc::c_char, 87 as libc::c_int);
    osSyncPrintf(b"osvimodep = %08x\n\x00" as *const u8 as
                     *const libc::c_char, osViMode);
    LogUtils_LogThreadId(b"../z_vimode.c\x00" as *const u8 as
                             *const libc::c_char, 88 as libc::c_int);
    osSyncPrintf(b"osvimodep->comRegs.ctrl = %08x\n\x00" as *const u8 as
                     *const libc::c_char, (*osViMode).comRegs.ctrl);
    LogUtils_LogThreadId(b"../z_vimode.c\x00" as *const u8 as
                             *const libc::c_char, 89 as libc::c_int);
    osSyncPrintf(b"osvimodep->comRegs.width = %08x\n\x00" as *const u8 as
                     *const libc::c_char, (*osViMode).comRegs.width);
    LogUtils_LogThreadId(b"../z_vimode.c\x00" as *const u8 as
                             *const libc::c_char, 90 as libc::c_int);
    osSyncPrintf(b"osvimodep->comRegs.burst = %08x\n\x00" as *const u8 as
                     *const libc::c_char, (*osViMode).comRegs.burst);
    LogUtils_LogThreadId(b"../z_vimode.c\x00" as *const u8 as
                             *const libc::c_char, 91 as libc::c_int);
    osSyncPrintf(b"osvimodep->comRegs.vSync = %08x\n\x00" as *const u8 as
                     *const libc::c_char, (*osViMode).comRegs.vSync);
    LogUtils_LogThreadId(b"../z_vimode.c\x00" as *const u8 as
                             *const libc::c_char, 92 as libc::c_int);
    osSyncPrintf(b"osvimodep->comRegs.hSync = %08x\n\x00" as *const u8 as
                     *const libc::c_char, (*osViMode).comRegs.hSync);
    LogUtils_LogThreadId(b"../z_vimode.c\x00" as *const u8 as
                             *const libc::c_char, 93 as libc::c_int);
    osSyncPrintf(b"osvimodep->comRegs.leap = %08x\n\x00" as *const u8 as
                     *const libc::c_char, (*osViMode).comRegs.leap);
    LogUtils_LogThreadId(b"../z_vimode.c\x00" as *const u8 as
                             *const libc::c_char, 94 as libc::c_int);
    osSyncPrintf(b"osvimodep->comRegs.hStart = %08x\n\x00" as *const u8 as
                     *const libc::c_char, (*osViMode).comRegs.hStart);
    LogUtils_LogThreadId(b"../z_vimode.c\x00" as *const u8 as
                             *const libc::c_char, 95 as libc::c_int);
    osSyncPrintf(b"osvimodep->comRegs.xScale = %08x\n\x00" as *const u8 as
                     *const libc::c_char, (*osViMode).comRegs.xScale);
    LogUtils_LogThreadId(b"../z_vimode.c\x00" as *const u8 as
                             *const libc::c_char, 96 as libc::c_int);
    osSyncPrintf(b"osvimodep->fldRegs[0].vStart = %08x\n\x00" as *const u8 as
                     *const libc::c_char,
                 (*osViMode).fldRegs[0 as libc::c_int as usize].vStart);
    LogUtils_LogThreadId(b"../z_vimode.c\x00" as *const u8 as
                             *const libc::c_char, 97 as libc::c_int);
    osSyncPrintf(b"osvimodep->fldRegs[0].vBurst = %08x\n\x00" as *const u8 as
                     *const libc::c_char,
                 (*osViMode).fldRegs[0 as libc::c_int as usize].vBurst);
    LogUtils_LogThreadId(b"../z_vimode.c\x00" as *const u8 as
                             *const libc::c_char, 98 as libc::c_int);
    osSyncPrintf(b"osvimodep->fldRegs[0].origin = %08x\n\x00" as *const u8 as
                     *const libc::c_char,
                 (*osViMode).fldRegs[0 as libc::c_int as usize].origin);
    LogUtils_LogThreadId(b"../z_vimode.c\x00" as *const u8 as
                             *const libc::c_char, 99 as libc::c_int);
    osSyncPrintf(b"osvimodep->fldRegs[0].yScale = %08x\n\x00" as *const u8 as
                     *const libc::c_char,
                 (*osViMode).fldRegs[0 as libc::c_int as usize].yScale);
    LogUtils_LogThreadId(b"../z_vimode.c\x00" as *const u8 as
                             *const libc::c_char, 100 as libc::c_int);
    osSyncPrintf(b"osvimodep->fldRegs[0].vIntr = %08x\n\x00" as *const u8 as
                     *const libc::c_char,
                 (*osViMode).fldRegs[0 as libc::c_int as usize].vIntr);
    LogUtils_LogThreadId(b"../z_vimode.c\x00" as *const u8 as
                             *const libc::c_char, 101 as libc::c_int);
    osSyncPrintf(b"osvimodep->fldRegs[1].vStart = %08x\n\x00" as *const u8 as
                     *const libc::c_char,
                 (*osViMode).fldRegs[1 as libc::c_int as usize].vStart);
    LogUtils_LogThreadId(b"../z_vimode.c\x00" as *const u8 as
                             *const libc::c_char, 102 as libc::c_int);
    osSyncPrintf(b"osvimodep->fldRegs[1].vBurst = %08x\n\x00" as *const u8 as
                     *const libc::c_char,
                 (*osViMode).fldRegs[1 as libc::c_int as usize].vBurst);
    LogUtils_LogThreadId(b"../z_vimode.c\x00" as *const u8 as
                             *const libc::c_char, 103 as libc::c_int);
    osSyncPrintf(b"osvimodep->fldRegs[1].origin = %08x\n\x00" as *const u8 as
                     *const libc::c_char,
                 (*osViMode).fldRegs[1 as libc::c_int as usize].origin);
    LogUtils_LogThreadId(b"../z_vimode.c\x00" as *const u8 as
                             *const libc::c_char, 104 as libc::c_int);
    osSyncPrintf(b"osvimodep->fldRegs[1].yScale = %08x\n\x00" as *const u8 as
                     *const libc::c_char,
                 (*osViMode).fldRegs[1 as libc::c_int as usize].yScale);
    LogUtils_LogThreadId(b"../z_vimode.c\x00" as *const u8 as
                             *const libc::c_char, 105 as libc::c_int);
    osSyncPrintf(b"osvimodep->fldRegs[1].vIntr = %08x\n\x00" as *const u8 as
                     *const libc::c_char,
                 (*osViMode).fldRegs[1 as libc::c_int as usize].vIntr);
}
// This function configures the custom VI mode (`viMode.customViMode`) based on the other flags in `viMode`.
#[no_mangle]
pub unsafe extern "C" fn ViMode_Configure(mut viMode: *mut ViMode,
                                          mut mode: s32, mut type_0: s32,
                                          mut unk_70: s32, mut unk_74: s32,
                                          mut unk_78: s32, mut unk_7C: s32,
                                          mut width: s32, mut height: s32,
                                          mut unk_left: s32,
                                          mut unk_right: s32,
                                          mut unk_top: s32,
                                          mut unk_bottom: s32) {
    let mut not_70: s32 = 0;
    let mut not_74: s32 = 0;
    let mut not_78: s32 = 0;
    let mut not_7C: s32 = 0;
    let mut cond_4C: s32 = 0;
    let mut cond_48: s32 = 0;
    let mut cond_44: s32 = 0;
    let mut cond_40: s32 = 0;
    let mut cond_3C: s32 = 0;
    let mut cond_38: s32 = 0;
    let mut cond_34: s32 = 0;
    let mut yScaleLo: s32 = 0;
    let mut yScaleHi0: s32 = 0;
    let mut yScaleHi1: s32 = 0;
    not_70 = (unk_70 == 0) as libc::c_int;
    not_74 = (unk_74 == 0) as libc::c_int;
    not_78 = (unk_78 == 0) as libc::c_int;
    not_7C = (unk_7C == 0) as libc::c_int;
    cond_4C = (not_70 != 0 && not_78 != 0) as libc::c_int;
    cond_48 = (not_70 != 0 && unk_78 != 0) as libc::c_int;
    cond_44 = (unk_70 != 0 && unk_78 != 0) as libc::c_int;
    cond_40 = (unk_70 != 0 && not_78 != 0) as libc::c_int;
    cond_3C =
        (unk_70 != 0 && not_74 != 0 && unk_78 != 0 && unk_7C != 0) as
            libc::c_int;
    cond_38 =
        (unk_70 != 0 && unk_74 != 0 && unk_78 != 0 && not_7C != 0) as
            libc::c_int;
    cond_34 =
        (not_70 != 0 && unk_74 != 0 && unk_78 != 0 && not_7C != 0) as
            libc::c_int;
    unk_top &= !(1 as libc::c_int);
    unk_bottom &= !(1 as libc::c_int);
    yScaleLo =
        (if cond_4C != 0 { 2 as libc::c_int } else { 1 as libc::c_int }) *
            ((height << 11 as libc::c_int) /
                 (240 as libc::c_int * 2 as libc::c_int + unk_bottom -
                      unk_top) /
                 (if unk_70 != 0 {
                      1 as libc::c_int
                  } else { 2 as libc::c_int }));
    yScaleHi0 =
        if not_78 != 0 {
            if cond_40 != 0 {
                0x1000000 as libc::c_int
            } else { 0x2000000 as libc::c_int }
        } else { 0 as libc::c_int };
    yScaleHi1 =
        if not_78 != 0 {
            if cond_40 != 0 {
                0x3000000 as libc::c_int
            } else { 0x2000000 as libc::c_int }
        } else { 0 as libc::c_int };
    (*viMode).customViMode.type_0 = mode as u8_0;
    (*viMode).customViMode.comRegs.ctrl =
        (0x2000 as libc::c_int | 0x1000 as libc::c_int | 0x8 as libc::c_int |
             0x4 as libc::c_int |
             (if cond_44 == 0 {
                  0x40 as libc::c_int
              } else { 0 as libc::c_int }) |
             (if not_74 != 0 {
                  0x10 as libc::c_int
              } else { 0 as libc::c_int }) |
             (if not_7C != 0 {
                  (0x2 as libc::c_int) | 0x1 as libc::c_int
              } else { 0x2 as libc::c_int })) as u32_0;
    if cond_3C != 0 {
        (*viMode).customViMode.comRegs.ctrl |=
            0x100 as libc::c_int as libc::c_uint
    } else if cond_38 | cond_34 != 0 {
        (*viMode).customViMode.comRegs.ctrl |=
            0x300 as libc::c_int as libc::c_uint
    } else if unk_74 != 0 {
        (*viMode).customViMode.comRegs.ctrl |=
            0x200 as libc::c_int as libc::c_uint
    } else {
        (*viMode).customViMode.comRegs.ctrl |=
            0 as libc::c_int as libc::c_uint
    }
    (*viMode).customViMode.comRegs.width =
        (width *
             (if cond_48 != 0 { 2 as libc::c_int } else { 1 as libc::c_int }))
            as u32_0;
    if type_0 == 1 as libc::c_int {
        (*viMode).customViMode.comRegs.burst =
            0x3e52239 as libc::c_int as u32_0;
        (*viMode).customViMode.comRegs.vSync = 0x20c as libc::c_int as u32_0;
        (*viMode).customViMode.comRegs.hSync = 0xc15 as libc::c_int as u32_0;
        (*viMode).customViMode.comRegs.leap =
            0xc150c15 as libc::c_int as u32_0;
        (*viMode).customViMode.comRegs.hStart =
            0x6c02ec as libc::c_int as u32_0;
        (*viMode).customViMode.fldRegs[0 as libc::c_int as usize].vStart =
            0x2501ff as libc::c_int as u32_0;
        (*viMode).customViMode.fldRegs[0 as libc::c_int as usize].vBurst =
            0xe0204 as libc::c_int as u32_0
    } else if type_0 == 0 as libc::c_int {
        (*viMode).customViMode.comRegs.burst =
            0x404233a as libc::c_int as u32_0;
        (*viMode).customViMode.comRegs.vSync = 0x270 as libc::c_int as u32_0;
        (*viMode).customViMode.comRegs.hSync =
            0x150c69 as libc::c_int as u32_0;
        (*viMode).customViMode.comRegs.leap =
            0xc6f0c6e as libc::c_int as u32_0;
        (*viMode).customViMode.comRegs.hStart =
            0x800300 as libc::c_int as u32_0;
        (*viMode).customViMode.fldRegs[0 as libc::c_int as usize].vStart =
            0x5f0239 as libc::c_int as u32_0;
        (*viMode).customViMode.fldRegs[0 as libc::c_int as usize].vBurst =
            0x9026b as libc::c_int as u32_0
    } else if type_0 == 2 as libc::c_int {
        (*viMode).customViMode.comRegs.burst =
            0x4651e39 as libc::c_int as u32_0;
        (*viMode).customViMode.comRegs.vSync = 0x20c as libc::c_int as u32_0;
        (*viMode).customViMode.comRegs.hSync = 0xc10 as libc::c_int as u32_0;
        (*viMode).customViMode.comRegs.leap =
            0xc1c0c1c as libc::c_int as u32_0;
        (*viMode).customViMode.comRegs.hStart =
            0x6c02ec as libc::c_int as u32_0;
        (*viMode).customViMode.fldRegs[0 as libc::c_int as usize].vStart =
            0x2501ff as libc::c_int as u32_0;
        (*viMode).customViMode.fldRegs[0 as libc::c_int as usize].vBurst =
            0xe0204 as libc::c_int as u32_0
    }
    (*viMode).customViMode.fldRegs[1 as libc::c_int as usize].vStart =
        (*viMode).customViMode.fldRegs[0 as libc::c_int as usize].vStart;
    (*viMode).customViMode.comRegs.hStart =
        ((*viMode).customViMode.comRegs.hStart as
             libc::c_uint).wrapping_add(((unk_left << 16 as libc::c_int) +
                                             unk_right as s16 as libc::c_int)
                                            as libc::c_uint) as u32_0 as
            u32_0;
    (*viMode).customViMode.fldRegs[0 as libc::c_int as usize].vStart =
        ((*viMode).customViMode.fldRegs[0 as libc::c_int as usize].vStart as
             libc::c_uint).wrapping_add(((unk_top << 16 as libc::c_int) +
                                             unk_bottom as s16 as libc::c_int)
                                            as libc::c_uint) as u32_0 as
            u32_0;
    (*viMode).customViMode.fldRegs[1 as libc::c_int as usize].vStart =
        ((*viMode).customViMode.fldRegs[1 as libc::c_int as usize].vStart as
             libc::c_uint).wrapping_add(((unk_top << 16 as libc::c_int) +
                                             unk_bottom as s16 as libc::c_int)
                                            as libc::c_uint) as u32_0 as
            u32_0;
    (*viMode).customViMode.fldRegs[1 as libc::c_int as usize].vBurst =
        (*viMode).customViMode.fldRegs[0 as libc::c_int as usize].vBurst;
    if cond_44 != 0 {
        (*viMode).customViMode.comRegs.vSync =
            (*viMode).customViMode.comRegs.vSync.wrapping_add(1);
        if type_0 == 2 as libc::c_int {
            (*viMode).customViMode.comRegs.hSync =
                ((*viMode).customViMode.comRegs.hSync as
                     libc::c_uint).wrapping_add(0x40001 as libc::c_int as
                                                    libc::c_uint) as u32_0 as
                    u32_0
        }
        if type_0 == 2 as libc::c_int {
            (*viMode).customViMode.comRegs.leap =
                ((*viMode).customViMode.comRegs.leap as
                     libc::c_uint).wrapping_add(0xfffcfffe as libc::c_uint) as
                    u32_0 as u32_0
        }
    } else {
        (*viMode).customViMode.fldRegs[0 as libc::c_int as usize].vStart =
            ((*viMode).customViMode.fldRegs[0 as libc::c_int as usize].vStart
                 as libc::c_uint).wrapping_add(0xfffdfffe as libc::c_uint) as
                u32_0 as u32_0;
        if type_0 == 2 as libc::c_int {
            (*viMode).customViMode.fldRegs[0 as libc::c_int as usize].vBurst =
                ((*viMode).customViMode.fldRegs[0 as libc::c_int as
                                                    usize].vBurst as
                     libc::c_uint).wrapping_add(0xfffcfffe as libc::c_uint) as
                    u32_0 as u32_0
        }
        if type_0 == 0 as libc::c_int {
            (*viMode).customViMode.fldRegs[1 as libc::c_int as usize].vBurst =
                ((*viMode).customViMode.fldRegs[1 as libc::c_int as
                                                    usize].vBurst as
                     libc::c_uint).wrapping_add(0x2fffe as libc::c_int as
                                                    libc::c_uint) as u32_0 as
                    u32_0
        }
    }
    (*viMode).customViMode.comRegs.xScale =
        ((width << 10 as libc::c_int) /
             (320 as libc::c_int * 2 as libc::c_int + unk_right - unk_left))
            as u32_0;
    (*viMode).customViMode.comRegs.vCurrent = 0 as libc::c_int as u32_0;
    (*viMode).customViMode.fldRegs[0 as libc::c_int as usize].origin =
        (width * 2 as libc::c_int *
             (if unk_7C != 0 { 1 as libc::c_int } else { 2 as libc::c_int }))
            as u32_0;
    (*viMode).customViMode.fldRegs[1 as libc::c_int as usize].origin =
        (width * 2 as libc::c_int *
             (if unk_7C != 0 { 1 as libc::c_int } else { 2 as libc::c_int }) *
             (if unk_70 != 0 { 1 as libc::c_int } else { 2 as libc::c_int }))
            as u32_0;
    (*viMode).customViMode.fldRegs[0 as libc::c_int as usize].yScale =
        (yScaleLo | yScaleHi0) as u32_0;
    (*viMode).customViMode.fldRegs[1 as libc::c_int as usize].yScale =
        (yScaleLo | yScaleHi1) as u32_0;
    (*viMode).customViMode.fldRegs[0 as libc::c_int as usize].vIntr =
        2 as libc::c_int as u32_0;
    (*viMode).customViMode.fldRegs[1 as libc::c_int as usize].vIntr =
        2 as libc::c_int as u32_0;
}
#[no_mangle]
pub unsafe extern "C" fn ViMode_Save(mut viMode: *mut ViMode) {
    (*gGameInfo).data[(1 as libc::c_int * 6 as libc::c_int * 16 as libc::c_int
                           + 48 as libc::c_int) as usize] =
        (*viMode).viModeBase as s16;
    (*gGameInfo).data[(1 as libc::c_int * 6 as libc::c_int * 16 as libc::c_int
                           + 49 as libc::c_int) as usize] =
        (*viMode).viWidth as s16;
    (*gGameInfo).data[(1 as libc::c_int * 6 as libc::c_int * 16 as libc::c_int
                           + 50 as libc::c_int) as usize] =
        (*viMode).viHeight as s16;
    (*gGameInfo).data[(1 as libc::c_int * 6 as libc::c_int * 16 as libc::c_int
                           + 51 as libc::c_int) as usize] =
        (*viMode).unk_64 as s16;
    (*gGameInfo).data[(1 as libc::c_int * 6 as libc::c_int * 16 as libc::c_int
                           + 52 as libc::c_int) as usize] =
        (*viMode).unk_60 as s16;
    (*gGameInfo).data[(1 as libc::c_int * 6 as libc::c_int * 16 as libc::c_int
                           + 53 as libc::c_int) as usize] =
        (*viMode).unk_5C as s16;
    (*gGameInfo).data[(1 as libc::c_int * 6 as libc::c_int * 16 as libc::c_int
                           + 54 as libc::c_int) as usize] =
        (*viMode).unk_58 as s16;
    if (*gGameInfo).data[(1 as libc::c_int * 6 as libc::c_int *
                              16 as libc::c_int + 58 as libc::c_int) as usize]
           as libc::c_int == 1 as libc::c_int {
        (*gGameInfo).data[(1 as libc::c_int * 6 as libc::c_int *
                               16 as libc::c_int + 58 as libc::c_int) as
                              usize] = 0 as libc::c_int as s16;
        match (*gGameInfo).data[(1 as libc::c_int * 6 as libc::c_int *
                                     16 as libc::c_int + 59 as libc::c_int) as
                                    usize] as libc::c_int {
            1 => {
                osSyncPrintf(b"osViModePalLan1\n\x00" as *const u8 as
                                 *const libc::c_char);
                ViMode_LogPrint(&mut osViModePalLan1);
            }
            2 => {
                osSyncPrintf(b"osViModeFpalLan1\n\x00" as *const u8 as
                                 *const libc::c_char);
                ViMode_LogPrint(&mut osViModeFpalLan1);
            }
            _ => {
                osSyncPrintf(b"Custom\n\x00" as *const u8 as
                                 *const libc::c_char);
                ViMode_LogPrint(&mut (*viMode).customViMode);
            }
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn ViMode_Load(mut viMode: *mut ViMode) {
    if (*gGameInfo).data[(1 as libc::c_int * 6 as libc::c_int *
                              16 as libc::c_int + 49 as libc::c_int) as usize]
           as libc::c_int & !(3 as libc::c_int) == 1 as libc::c_int {
        (*gGameInfo).data[(1 as libc::c_int * 6 as libc::c_int *
                               16 as libc::c_int + 49 as libc::c_int) as
                              usize] =
            ((*gGameInfo).data[(1 as libc::c_int * 6 as libc::c_int *
                                    16 as libc::c_int + 49 as libc::c_int) as
                                   usize] as libc::c_int + 4 as libc::c_int)
                as s16
    }
    (*viMode).viModeBase =
        (*gGameInfo).data[(1 as libc::c_int * 6 as libc::c_int *
                               16 as libc::c_int + 48 as libc::c_int) as
                              usize] as s32;
    (*viMode).viWidth =
        (*gGameInfo).data[(1 as libc::c_int * 6 as libc::c_int *
                               16 as libc::c_int + 49 as libc::c_int) as
                              usize] as libc::c_int & !(3 as libc::c_int);
    (*viMode).viHeight =
        (*gGameInfo).data[(1 as libc::c_int * 6 as libc::c_int *
                               16 as libc::c_int + 50 as libc::c_int) as
                              usize] as s32;
    (*viMode).unk_64 =
        (*gGameInfo).data[(1 as libc::c_int * 6 as libc::c_int *
                               16 as libc::c_int + 51 as libc::c_int) as
                              usize] as s32;
    (*viMode).unk_60 =
        (*gGameInfo).data[(1 as libc::c_int * 6 as libc::c_int *
                               16 as libc::c_int + 52 as libc::c_int) as
                              usize] as s32;
    (*viMode).unk_5C =
        (*gGameInfo).data[(1 as libc::c_int * 6 as libc::c_int *
                               16 as libc::c_int + 53 as libc::c_int) as
                              usize] as s32;
    (*viMode).unk_58 =
        (*gGameInfo).data[(1 as libc::c_int * 6 as libc::c_int *
                               16 as libc::c_int + 54 as libc::c_int) as
                              usize] as s32;
}
#[no_mangle]
pub unsafe extern "C" fn ViMode_Init(mut viMode: *mut ViMode) {
    (*viMode).viModeBase = 0 as libc::c_int;
    (*viMode).viWidth = 320 as libc::c_int;
    (*viMode).viHeight = 240 as libc::c_int;
    (*viMode).unk_5C = 0 as libc::c_int;
    (*viMode).unk_58 = 0 as libc::c_int;
    (*viMode).unk_64 = 0 as libc::c_int;
    (*viMode).unk_60 = 0 as libc::c_int;
    (*viMode).viFeatures =
        (0x40 as libc::c_int | 0x2 as libc::c_int) as u32_0;
    (*viMode).viTvType = osTvType as s32;
    (*viMode).unk_7C = 1 as libc::c_int as u32_0;
    (*viMode).unk_78 = 1 as libc::c_int as u32_0;
    (*viMode).unk_74 = 0 as libc::c_int as u32_0;
    (*viMode).unk_70 = 1 as libc::c_int as u32_0;
    ViMode_Save(viMode);
}
#[no_mangle]
pub unsafe extern "C" fn ViMode_Destroy(mut viMode: *mut ViMode) { }
#[no_mangle]
pub unsafe extern "C" fn ViMode_ConfigureFeatures(mut viMode: *mut ViMode,
                                                  mut viFeatures: s32) {
    let mut ctrl: u32_0 = (*viMode).customViMode.comRegs.ctrl;
    if viFeatures & 0x1 as libc::c_int != 0 {
        ctrl |= 0x8 as libc::c_int as libc::c_uint
    }
    if viFeatures & 0x2 as libc::c_int != 0 {
        ctrl &= !(0x8 as libc::c_int) as libc::c_uint
    }
    if viFeatures & 0x4 as libc::c_int != 0 {
        ctrl |= 0x4 as libc::c_int as libc::c_uint
    }
    if viFeatures & 0x8 as libc::c_int != 0 {
        ctrl &= !(0x4 as libc::c_int) as libc::c_uint
    }
    if viFeatures & 0x10 as libc::c_int != 0 {
        ctrl |= 0x10 as libc::c_int as libc::c_uint
    }
    if viFeatures & 0x20 as libc::c_int != 0 {
        ctrl &= !(0x10 as libc::c_int) as libc::c_uint
    }
    (*viMode).customViMode.comRegs.ctrl = ctrl;
}
// This function uses controller input (C buttons + D pad) to reconfigure the custom VI mode
#[no_mangle]
pub unsafe extern "C" fn ViMode_Update(mut viMode: *mut ViMode,
                                       mut input: *mut Input) {
    ViMode_Load(viMode);
    if (*viMode).viModeBase == 1 as libc::c_int ||
           (*viMode).viModeBase == 2 as libc::c_int ||
           (*viMode).viModeBase == 3 as libc::c_int {
        gScreenWidth = (*viMode).viWidth;
        gScreenHeight = (*viMode).viHeight;
        if !((*input).cur.button as libc::c_int |
                 !(0x1000 as libc::c_int | 0x8 as libc::c_int |
                       0x1 as libc::c_int)) == 0 as libc::c_int {
            ViMode_Init(viMode);
        }
        if !((*input).cur.button as libc::c_int | !(0x8 as libc::c_int)) ==
               0 as libc::c_int {
            if !((*input).cur.button as libc::c_int | !(0x800 as libc::c_int))
                   == 0 as libc::c_int {
                (*viMode).unk_64 -= 1
            }
            if !((*input).cur.button as libc::c_int | !(0x400 as libc::c_int))
                   == 0 as libc::c_int {
                (*viMode).unk_64 += 1
            }
            if !((*input).cur.button as libc::c_int | !(0x200 as libc::c_int))
                   == 0 as libc::c_int {
                (*viMode).unk_5C -= 1
            }
            if !((*input).cur.button as libc::c_int | !(0x100 as libc::c_int))
                   == 0 as libc::c_int {
                (*viMode).unk_5C += 1
            }
        }
        if !((*input).cur.button as libc::c_int | !(0x1 as libc::c_int)) ==
               0 as libc::c_int {
            if !((*input).cur.button as libc::c_int | !(0x800 as libc::c_int))
                   == 0 as libc::c_int {
                (*viMode).unk_60 -= 1
            }
            if !((*input).cur.button as libc::c_int | !(0x400 as libc::c_int))
                   == 0 as libc::c_int {
                (*viMode).unk_60 += 1
            }
            if !((*input).cur.button as libc::c_int | !(0x200 as libc::c_int))
                   == 0 as libc::c_int {
                (*viMode).unk_58 -= 1
            }
            if !((*input).cur.button as libc::c_int | !(0x100 as libc::c_int))
                   == 0 as libc::c_int {
                (*viMode).unk_58 += 1
            }
        }
        if !((*input).cur.button as libc::c_int | !(0x4 as libc::c_int)) ==
               0 as libc::c_int {
            if !((*input).press.button as libc::c_int |
                     !(0x800 as libc::c_int)) == 0 as libc::c_int {
                (*viMode).unk_70 =
                    ((*viMode).unk_70 == 0) as libc::c_int as u32_0
            }
            if !((*input).press.button as libc::c_int |
                     !(0x400 as libc::c_int)) == 0 as libc::c_int {
                (*viMode).unk_74 =
                    ((*viMode).unk_74 == 0) as libc::c_int as u32_0
            }
            if !((*input).press.button as libc::c_int |
                     !(0x200 as libc::c_int)) == 0 as libc::c_int {
                (*viMode).unk_78 =
                    ((*viMode).unk_78 == 0) as libc::c_int as u32_0
            }
            if !((*input).press.button as libc::c_int |
                     !(0x100 as libc::c_int)) == 0 as libc::c_int {
                (*viMode).unk_7C =
                    ((*viMode).unk_7C == 0) as libc::c_int as u32_0
            }
        }
        if (*viMode).viModeBase >= 2 as libc::c_int {
            if (*viMode).unk_5C < -(16 as libc::c_int) {
                (*viMode).unk_5C = -(16 as libc::c_int)
            }
            if (*viMode).unk_64 < -(50 as libc::c_int) {
                (*viMode).unk_64 = -(50 as libc::c_int)
            }
            if (*viMode).unk_58 > 16 as libc::c_int {
                (*viMode).unk_58 = 16 as libc::c_int
            }
            if (*viMode).unk_60 > 50 as libc::c_int {
                (*viMode).unk_60 = 50 as libc::c_int
            }
        } else {
            if (*viMode).unk_5C < 0 as libc::c_int {
                (*viMode).unk_5C = 0 as libc::c_int
            }
            if (*viMode).unk_64 < 0 as libc::c_int {
                (*viMode).unk_64 = 0 as libc::c_int
            }
            if (*viMode).unk_58 > 0 as libc::c_int {
                (*viMode).unk_58 = 0 as libc::c_int
            }
            if (*viMode).unk_60 > 0 as libc::c_int {
                (*viMode).unk_60 = 0 as libc::c_int
            }
        }
        ViMode_Configure(viMode, 28 as libc::c_int, osTvType as s32,
                         (*viMode).unk_70 as s32, (*viMode).unk_74 as s32,
                         (*viMode).unk_78 as s32, (*viMode).unk_7C as s32,
                         (*viMode).viWidth, (*viMode).viHeight,
                         (*viMode).unk_5C, (*viMode).unk_58, (*viMode).unk_64,
                         (*viMode).unk_60);
        ViMode_ConfigureFeatures(viMode, (*viMode).viFeatures as s32);
        if (*viMode).viModeBase == 3 as libc::c_int {
            ViMode_LogPrint(&mut osViModeNtscLan1);
            ViMode_LogPrint(&mut (*viMode).customViMode);
            (*viMode).viModeBase = 2 as libc::c_int
        }
    }
    ViMode_Save(viMode);
}
