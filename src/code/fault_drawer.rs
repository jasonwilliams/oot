#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case,
         non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(c_variadic, const_raw_ptr_to_usize_cast, register_tool)]
extern "C" {
    #[no_mangle]
    fn osSyncPrintf(fmt: *const libc::c_char, _: ...);
    #[no_mangle]
    fn osWritebackDCache(vaddr: *mut libc::c_void, nbytes: s32);
    #[no_mangle]
    fn _Printf(_: PrintCallback, arg: *mut libc::c_void,
               fmt: *const libc::c_char, ap: __builtin_va_list) -> s32;
    #[no_mangle]
    fn bcopy(__src: *mut libc::c_void, __dest: *mut libc::c_void, __n: u32_0)
     -> *mut libc::c_void;
    #[no_mangle]
    fn osWritebackDCacheAll();
    #[no_mangle]
    static mut osMemSize: u32_0;
}
pub type __builtin_va_list = *mut libc::c_char;
pub type s8 = libc::c_schar;
pub type u8_0 = libc::c_uchar;
pub type u16_0 = libc::c_ushort;
pub type s32 = libc::c_int;
pub type u32_0 = libc::c_uint;
pub type PrintCallback
    =
    Option<unsafe extern "C" fn(_: *mut libc::c_void, _: *const libc::c_char,
                                _: u32_0) -> *mut libc::c_void>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct FaultDrawer {
    pub fb: *mut u16_0,
    pub w: u16_0,
    pub h: u16_0,
    pub yStart: u16_0,
    pub yEnd: u16_0,
    pub xStart: u16_0,
    pub xEnd: u16_0,
    pub foreColor: u16_0,
    pub backColor: u16_0,
    pub cursorX: u16_0,
    pub cursorY: u16_0,
    pub fontData: *const u32_0,
    pub charW: u8_0,
    pub charH: u8_0,
    pub charWPad: s8,
    pub charHPad: s8,
    pub printColors: [u16_0; 10],
    pub escCode: u8_0,
    pub osSyncPrintfEnabled: u8_0,
    pub inputCallback: Option<unsafe extern "C" fn() -> ()>,
}
// rodata
#[no_mangle]
pub static mut sFaultDrawerFont: [u32_0; 256] =
    [0xdffd00 as libc::c_int as u32_0, 0xaeeffa0 as libc::c_int as u32_0,
     0xdf22dd0 as libc::c_int as u32_0, 0x6611dc0 as libc::c_int as u32_0,
     0x1122dd0 as libc::c_int as u32_0, 0x6719900 as libc::c_int as u32_0,
     0x11eed10 as libc::c_int as u32_0, 0x77ef700 as libc::c_int as u32_0,
     0x1562990 as libc::c_int as u32_0, 0x5589760 as libc::c_int as u32_0,
     0xdd22990 as libc::c_int as u32_0, 0x5599770 as libc::c_int as u32_0,
     0x4dffd40 as libc::c_int as u32_0, 0x26ef700 as libc::c_int as u32_0,
     0 as libc::c_int as u32_0, 0 as libc::c_int as u32_0,
     0x8bffb00 as libc::c_int as u32_0, 0xeffffc0 as libc::c_int as u32_0,
     0xbf00fb0 as libc::c_int as u32_0, 0xff00330 as libc::c_int as u32_0,
     0xff00ff0 as libc::c_int as u32_0, 0xff00220 as libc::c_int as u32_0,
     0xcfbbf60 as libc::c_int as u32_0, 0xffcce20 as libc::c_int as u32_0,
     0xdd44ff0 as libc::c_int as u32_0, 0xff00220 as libc::c_int as u32_0,
     0xff00ff0 as libc::c_int as u32_0, 0xff00330 as libc::c_int as u32_0,
     0xcfbbf40 as libc::c_int as u32_0, 0xef77740 as libc::c_int as u32_0,
     0 as libc::c_int as u32_0, 0 as libc::c_int as u32_0,
     0xdffd00 as libc::c_int as u32_0, 0xaeeffa0 as libc::c_int as u32_0,
     0xdf22dd0 as libc::c_int as u32_0, 0x6611dc0 as libc::c_int as u32_0,
     0x1122dd0 as libc::c_int as u32_0, 0x6719900 as libc::c_int as u32_0,
     0x11eed10 as libc::c_int as u32_0, 0x77ef700 as libc::c_int as u32_0,
     0x1562990 as libc::c_int as u32_0, 0x5589760 as libc::c_int as u32_0,
     0xdd22990 as libc::c_int as u32_0, 0x5599770 as libc::c_int as u32_0,
     0x4dffd40 as libc::c_int as u32_0, 0x26ef700 as libc::c_int as u32_0,
     0 as libc::c_int as u32_0, 0 as libc::c_int as u32_0,
     0x8bffb00 as libc::c_int as u32_0, 0xde000 as libc::c_int as u32_0,
     0xbf00fb0 as libc::c_int as u32_0, 0x5de600 as libc::c_int as u32_0,
     0xff00ff0 as libc::c_int as u32_0, 0x55cc660 as libc::c_int as u32_0,
     0xcfbbf60 as libc::c_int as u32_0, 0x773ff377 as libc::c_int as u32_0,
     0xdd44ff0 as libc::c_int as u32_0, 0xbb3ff3bb as libc::c_uint,
     0xff00ff0 as libc::c_int as u32_0, 0x99ccaa0 as libc::c_int as u32_0,
     0xcfbbf40 as libc::c_int as u32_0, 0x9dea00 as libc::c_int as u32_0,
     0 as libc::c_int as u32_0, 0xde000 as libc::c_int as u32_0,
     0x4c22c40 as libc::c_int as u32_0, 0x28d5020 as libc::c_int as u32_0,
     0xccaacc0 as libc::c_int as u32_0, 0x21f91710 as libc::c_int as u32_0,
     0x4c22c40 as libc::c_int as u32_0, 0x12493400 as libc::c_int as u32_0,
     0x820800 as libc::c_int as u32_0, 0x1975110 as libc::c_int as u32_0,
     0x88a8880 as libc::c_int as u32_0, 0x4615241 as libc::c_int as u32_0,
     0x800800 as libc::c_int as u32_0, 0x43117530 as libc::c_int as u32_0,
     0xa20800 as libc::c_int as u32_0, 0x60055600 as libc::c_int as u32_0,
     0 as libc::c_int as u32_0, 0x4400040 as libc::c_int as u32_0,
     0x221100 as libc::c_int as u32_0, 0x80 as libc::c_int as u32_0,
     0xfb000 as libc::c_int as u32_0, 0x880 as libc::c_int as u32_0,
     0x40da400 as libc::c_int as u32_0, 0x8800 as libc::c_int as u32_0,
     0x8cde880 as libc::c_int as u32_0, 0x22aa220 as libc::c_int as u32_0,
     0x8cde880 as libc::c_int as u32_0, 0x2aa2220 as libc::c_int as u32_0,
     0x40da400 as libc::c_int as u32_0, 0xcd10000 as libc::c_int as u32_0,
     0xfb000 as libc::c_int as u32_0, 0x8c510000 as libc::c_uint,
     0x221100 as libc::c_int as u32_0, 0x81100000 as libc::c_uint,
     0xdffd00 as libc::c_int as u32_0, 0xaeeffa0 as libc::c_int as u32_0,
     0xdf22dd0 as libc::c_int as u32_0, 0x6611dc0 as libc::c_int as u32_0,
     0x1122dd0 as libc::c_int as u32_0, 0x6719900 as libc::c_int as u32_0,
     0x11eed10 as libc::c_int as u32_0, 0x77ef700 as libc::c_int as u32_0,
     0x1562990 as libc::c_int as u32_0, 0x5589760 as libc::c_int as u32_0,
     0xdd22990 as libc::c_int as u32_0, 0x5599770 as libc::c_int as u32_0,
     0x4dffd40 as libc::c_int as u32_0, 0x26ef700 as libc::c_int as u32_0,
     0 as libc::c_int as u32_0, 0 as libc::c_int as u32_0,
     0x333300 as libc::c_int as u32_0, 0x4489980 as libc::c_int as u32_0,
     0x33cc330 as libc::c_int as u32_0, 0xcd1088 as libc::c_int as u32_0,
     0x33cc330 as libc::c_int as u32_0, 0x2bf62a8 as libc::c_int as u32_0,
     0x333320 as libc::c_int as u32_0, 0x1104c80 as libc::c_int as u32_0,
     0x1100330 as libc::c_int as u32_0, 0x15c800 as libc::c_int as u32_0,
     0x33cc330 as libc::c_int as u32_0, 0x2673220 as libc::c_int as u32_0,
     0x3ff300 as libc::c_int as u32_0, 0x4409900 as libc::c_int as u32_0,
     0x880000 as libc::c_int as u32_0, 0 as libc::c_int as u32_0,
     0x5dffd10 as libc::c_int as u32_0, 0x7ffff60 as libc::c_int as u32_0,
     0x1ce00ec1 as libc::c_int as u32_0, 0xff00990 as libc::c_int as u32_0,
     0x1ee11661 as libc::c_int as u32_0, 0xff00110 as libc::c_int as u32_0,
     0x1ef45621 as libc::c_int as u32_0, 0xff66710 as libc::c_int as u32_0,
     0x1ef23661 as libc::c_int as u32_0, 0xff08990 as libc::c_int as u32_0,
     0x1ef10fe1 as libc::c_int as u32_0, 0xff00990 as libc::c_int as u32_0,
     0x16ecce21 as libc::c_int as u32_0, 0x7fbbb20 as libc::c_int as u32_0,
     0x1111110 as libc::c_int as u32_0, 0 as libc::c_int as u32_0,
     0x9b66fd0 as libc::c_int as u32_0, 0x27d88e60 as libc::c_int as u32_0,
     0x992ed10 as libc::c_int as u32_0, 0x2ff02ee0 as libc::c_int as u32_0,
     0x99ae510 as libc::c_int as u32_0, 0x2ff62ee0 as libc::c_int as u32_0,
     0x99b7510 as libc::c_int as u32_0, 0x2fd64ee0 as libc::c_int as u32_0,
     0xddae510 as libc::c_int as u32_0, 0x2fd04ee0 as libc::c_int as u32_0,
     0xdd2ed10 as libc::c_int as u32_0, 0x2fd00ee0 as libc::c_int as u32_0,
     0x9f66f90 as libc::c_int as u32_0, 0x27d99f70 as libc::c_int as u32_0,
     0 as libc::c_int as u32_0, 0 as libc::c_int as u32_0,
     0x7ffff00 as libc::c_int as u32_0, 0x8f711ff0 as libc::c_uint,
     0x2fd00ff0 as libc::c_int as u32_0, 0x8f711ff0 as libc::c_uint,
     0x2fd00770 as libc::c_int as u32_0, 0x8e611ee0 as libc::c_uint,
     0x27dddf60 as libc::c_int as u32_0, 0x8e691ee0 as libc::c_uint,
     0x27764aa0 as libc::c_int as u32_0, 0x8ee99ee0 as libc::c_uint,
     0x2fd06e80 as libc::c_int as u32_0, 0x8ae7fea0 as libc::c_uint,
     0x7fa8e60 as libc::c_int as u32_0, 0x88277a80 as libc::c_uint,
     0 as libc::c_int as u32_0, 0 as libc::c_int as u32_0,
     0x77ccff0 as libc::c_int as u32_0, 0x13266011 as libc::c_int as u32_0,
     0x77ccff0 as libc::c_int as u32_0, 0x3766510 as libc::c_int as u32_0,
     0x239d720 as libc::c_int as u32_0, 0x4533540 as libc::c_int as u32_0,
     0x2ff200 as libc::c_int as u32_0, 0x1133110 as libc::c_int as u32_0,
     0x5fb100 as libc::c_int as u32_0, 0x33000 as libc::c_int as u32_0,
     0x55ee550 as libc::c_int as u32_0, 0x1133110 as libc::c_int as u32_0,
     0x55eedd0 as libc::c_int as u32_0, 0x2233000 as libc::c_int as u32_0,
     0x88880 as libc::c_int as u32_0, 0x8aabb888 as libc::c_uint,
     0x1100 as libc::c_int as u32_0, 0x44510 as libc::c_int as u32_0,
     0x4623320 as libc::c_int as u32_0, 0x440110 as libc::c_int as u32_0,
     0x4c89aa0 as libc::c_int as u32_0, 0xeeab10 as libc::c_int as u32_0,
     0xce66720 as libc::c_int as u32_0, 0xef55fb0 as libc::c_int as u32_0,
     0xee00660 as libc::c_int as u32_0, 0xbf62b90 as libc::c_int as u32_0,
     0xee00660 as libc::c_int as u32_0, 0x3fc8990 as libc::c_int as u32_0,
     0x4eeeea0 as libc::c_int as u32_0, 0x773bb0 as libc::c_int as u32_0,
     0 as libc::c_int as u32_0, 0x8888800 as libc::c_int as u32_0,
     0x9900000 as libc::c_int as u32_0, 0x111000 as libc::c_int as u32_0,
     0x9922440 as libc::c_int as u32_0, 0x11000 as libc::c_int as u32_0,
     0x9908800 as libc::c_int as u32_0, 0x26efde20 as libc::c_int as u32_0,
     0x99bb540 as libc::c_int as u32_0, 0x2ec33ce2 as libc::c_int as u32_0,
     0xd9a2550 as libc::c_int as u32_0, 0x2ec33ce2 as libc::c_int as u32_0,
     0xddaa550 as libc::c_int as u32_0, 0x2ec33ce2 as libc::c_int as u32_0,
     0x9d6ed10 as libc::c_int as u32_0, 0x26cbbc62 as libc::c_int as u32_0,
     0 as libc::c_int as u32_0, 0 as libc::c_int as u32_0,
     0 as libc::c_int as u32_0, 0 as libc::c_int as u32_0,
     0 as libc::c_int as u32_0, 0x11000 as libc::c_int as u32_0,
     0x5fbffe0 as libc::c_int as u32_0, 0x8e6116e8 as libc::c_uint,
     0xff40330 as libc::c_int as u32_0, 0x8f7117f8 as libc::c_uint,
     0x7fc8b30 as libc::c_int as u32_0, 0x8e6996e8 as libc::c_uint,
     0x5733ba0 as libc::c_int as u32_0, 0x8a6dd6a8 as libc::c_uint,
     0xdd88a20 as libc::c_int as u32_0, 0x8a779b2 as libc::c_int as u32_0,
     0x1100220 as libc::c_int as u32_0, 0 as libc::c_int as u32_0,
     0x80 as libc::c_int as u32_0, 0x8a011000 as libc::c_uint,
     0x800 as libc::c_int as u32_0, 0x80a11000 as libc::c_uint,
     0x7744f70 as libc::c_int as u32_0, 0x80a99000 as libc::c_uint,
     0x231df20 as libc::c_int as u32_0, 0x84e60004 as libc::c_uint,
     0x27da20 as libc::c_int as u32_0, 0xc8aa4c40 as libc::c_uint,
     0x573b20 as libc::c_int as u32_0, 0xa11800 as libc::c_int as u32_0,
     0x5546f50 as libc::c_int as u32_0, 0xa99800 as libc::c_int as u32_0,
     0x2222080 as libc::c_int as u32_0, 0x2001888 as libc::c_int as u32_0];
// Initialized in run_static_initializers
#[no_mangle]
pub static mut sFaultDrawerDefault: FaultDrawer =
    FaultDrawer{fb: 0 as *const u16_0 as *mut u16_0,
                w: 0,
                h: 0,
                yStart: 0,
                yEnd: 0,
                xStart: 0,
                xEnd: 0,
                foreColor: 0,
                backColor: 0,
                cursorX: 0,
                cursorY: 0,
                fontData: 0 as *const u32_0,
                charW: 0,
                charH: 0,
                charWPad: 0,
                charHPad: 0,
                printColors: [0; 10],
                escCode: 0,
                osSyncPrintfEnabled: 0,
                inputCallback: None,};
#[no_mangle]
pub static mut sFaultDrawerStruct: FaultDrawer =
    FaultDrawer{fb: 0 as *const u16_0 as *mut u16_0,
                w: 0,
                h: 0,
                yStart: 0,
                yEnd: 0,
                xStart: 0,
                xEnd: 0,
                foreColor: 0,
                backColor: 0,
                cursorX: 0,
                cursorY: 0,
                fontData: 0 as *const u32_0,
                charW: 0,
                charH: 0,
                charWPad: 0,
                charHPad: 0,
                printColors: [0; 10],
                escCode: 0,
                osSyncPrintfEnabled: 0,
                inputCallback: None,};
#[no_mangle]
pub static mut D_8016B6C0: [libc::c_char; 32] = [0; 32];
#[no_mangle]
pub unsafe extern "C" fn FaultDrawer_SetOsSyncPrintfEnabled(mut enabled:
                                                                u32_0) {
    sFaultDrawerStruct.osSyncPrintfEnabled = enabled as u8_0;
}
#[no_mangle]
pub unsafe extern "C" fn FaultDrawer_DrawRecImpl(mut xStart: s32,
                                                 mut yStart: s32,
                                                 mut xEnd: s32, mut yEnd: s32,
                                                 mut color: u16_0) {
    let mut fb: *mut u16_0 = 0 as *mut u16_0;
    let mut x: s32 = 0;
    let mut y: s32 = 0;
    let mut xDiff: s32 = sFaultDrawerStruct.w as libc::c_int - xStart;
    let mut yDiff: s32 = sFaultDrawerStruct.h as libc::c_int - yStart;
    let mut xSize: s32 = xEnd - xStart + 1 as libc::c_int;
    let mut ySize: s32 = yEnd - yStart + 1 as libc::c_int;
    if xDiff > 0 as libc::c_int && yDiff > 0 as libc::c_int {
        if xDiff < xSize { xSize = xDiff }
        if yDiff < ySize { ySize = yDiff }
        fb =
            sFaultDrawerStruct.fb.offset((sFaultDrawerStruct.w as libc::c_int
                                              * yStart) as
                                             isize).offset(xStart as isize);
        y = 0 as libc::c_int;
        while y < ySize {
            x = 0 as libc::c_int;
            while x < xSize {
                let fresh0 = fb;
                fb = fb.offset(1);
                *fresh0 = color;
                x += 1
            }
            fb =
                fb.offset((sFaultDrawerStruct.w as libc::c_int - xSize) as
                              isize);
            y += 1
        }
        osWritebackDCacheAll();
    };
}
#[no_mangle]
pub unsafe extern "C" fn FaultDrawer_DrawChar(mut c: libc::c_char) {
    let mut fb: *mut u16_0 = 0 as *mut u16_0;
    let mut x: s32 = 0;
    let mut y: s32 = 0;
    let mut dataPtr: *const u32_0 = 0 as *const u32_0;
    let mut data: u32_0 = 0;
    let mut cursorX: s32 = sFaultDrawerStruct.cursorX as s32;
    let mut cursorY: s32 = sFaultDrawerStruct.cursorY as s32;
    let mut fontData: *mut *const u32_0 = &mut sFaultDrawerStruct.fontData;
    let mut shift: s32 = c as libc::c_int % 4 as libc::c_int;
    dataPtr =
        &*(*fontData.offset(0 as libc::c_int as
                                isize)).offset((c as libc::c_int /
                                                    8 as libc::c_int *
                                                    16 as libc::c_int +
                                                    ((c as libc::c_int &
                                                          4 as libc::c_int) >>
                                                         2 as libc::c_int)) as
                                                   isize) as *const u32_0;
    fb =
        sFaultDrawerStruct.fb.offset((sFaultDrawerStruct.w as libc::c_int *
                                          cursorY) as
                                         isize).offset(cursorX as isize);
    if sFaultDrawerStruct.xStart as libc::c_int <= cursorX &&
           sFaultDrawerStruct.charW as libc::c_int + cursorX -
               1 as libc::c_int <= sFaultDrawerStruct.xEnd as libc::c_int &&
           sFaultDrawerStruct.yStart as libc::c_int <= cursorY &&
           sFaultDrawerStruct.charH as libc::c_int + cursorY -
               1 as libc::c_int <= sFaultDrawerStruct.yEnd as libc::c_int {
        y = 0 as libc::c_int;
        while y < sFaultDrawerStruct.charH as libc::c_int {
            let mut mask: u32_0 =
                ((0x10000000 as libc::c_int) << shift) as u32_0;
            data = *dataPtr;
            x = 0 as libc::c_int;
            while x < sFaultDrawerStruct.charW as libc::c_int {
                if mask & data != 0 {
                    *fb.offset(x as isize) = sFaultDrawerStruct.foreColor
                } else if sFaultDrawerStruct.backColor as libc::c_int &
                              1 as libc::c_int != 0 {
                    *fb.offset(x as isize) = sFaultDrawerStruct.backColor
                }
                mask >>= 4 as libc::c_int;
                x += 1
            }
            fb = fb.offset(sFaultDrawerStruct.w as libc::c_int as isize);
            dataPtr = dataPtr.offset(2 as libc::c_int as isize);
            y += 1
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn FaultDrawer_ColorToPrintColor(mut color: u16_0)
 -> s32 {
    let mut i: s32 = 0;
    i = 0 as libc::c_int;
    while i < 10 as libc::c_int {
        if color as libc::c_int ==
               sFaultDrawerStruct.printColors[i as usize] as libc::c_int {
            return i
        }
        i += 1
    }
    return -(1 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn FaultDrawer_UpdatePrintColor() {
    let mut idx: s32 = 0;
    if sFaultDrawerStruct.osSyncPrintfEnabled != 0 {
        osSyncPrintf(b"\x1b[m\x00" as *const u8 as *const libc::c_char);
        idx = FaultDrawer_ColorToPrintColor(sFaultDrawerStruct.foreColor);
        if idx >= 0 as libc::c_int && idx < 8 as libc::c_int {
            osSyncPrintf(b"\x1b[3%dm\x00" as *const u8 as *const libc::c_char,
                         idx);
        }
        idx = FaultDrawer_ColorToPrintColor(sFaultDrawerStruct.backColor);
        if idx >= 0 as libc::c_int && idx < 8 as libc::c_int {
            osSyncPrintf(b"\x1b[4%dm\x00" as *const u8 as *const libc::c_char,
                         idx);
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn FaultDrawer_SetForeColor(mut color: u16_0) {
    sFaultDrawerStruct.foreColor = color;
    FaultDrawer_UpdatePrintColor();
}
#[no_mangle]
pub unsafe extern "C" fn FaultDrawer_SetBackColor(mut color: u16_0) {
    sFaultDrawerStruct.backColor = color;
    FaultDrawer_UpdatePrintColor();
}
#[no_mangle]
pub unsafe extern "C" fn FaultDrawer_SetFontColor(mut color: u16_0) {
    FaultDrawer_SetForeColor((color as libc::c_int | 1 as libc::c_int) as
                                 u16_0);
    // force alpha to be set
}
#[no_mangle]
pub unsafe extern "C" fn FaultDrawer_SetCharPad(mut padW: s8, mut padH: s8) {
    sFaultDrawerStruct.charWPad = padW;
    sFaultDrawerStruct.charHPad = padH;
}
#[no_mangle]
pub unsafe extern "C" fn FaultDrawer_SetCursor(mut x: s32, mut y: s32) {
    if sFaultDrawerStruct.osSyncPrintfEnabled != 0 {
        osSyncPrintf(b"\x1b[%d;%dH\x00" as *const u8 as *const libc::c_char,
                     (y - sFaultDrawerStruct.yStart as libc::c_int) /
                         (sFaultDrawerStruct.charH as libc::c_int +
                              sFaultDrawerStruct.charHPad as libc::c_int),
                     (x - sFaultDrawerStruct.xStart as libc::c_int) /
                         (sFaultDrawerStruct.charW as libc::c_int +
                              sFaultDrawerStruct.charWPad as libc::c_int));
    }
    sFaultDrawerStruct.cursorX = x as u16_0;
    sFaultDrawerStruct.cursorY = y as u16_0;
}
#[no_mangle]
pub unsafe extern "C" fn FaultDrawer_FillScreen() {
    if sFaultDrawerStruct.osSyncPrintfEnabled != 0 {
        osSyncPrintf(b"\x1b[2J\x00" as *const u8 as *const libc::c_char);
    }
    FaultDrawer_DrawRecImpl(sFaultDrawerStruct.xStart as s32,
                            sFaultDrawerStruct.yStart as s32,
                            sFaultDrawerStruct.xEnd as s32,
                            sFaultDrawerStruct.yEnd as s32,
                            (sFaultDrawerStruct.backColor as libc::c_int |
                                 1 as libc::c_int) as u16_0);
    FaultDrawer_SetCursor(sFaultDrawerStruct.xStart as s32,
                          sFaultDrawerStruct.yStart as s32);
}
#[no_mangle]
pub unsafe extern "C" fn FaultDrawer_FormatStringFunc(mut arg:
                                                          *mut libc::c_void,
                                                      mut str:
                                                          *const libc::c_char,
                                                      mut count: u32_0)
 -> *mut libc::c_void {
    while count != 0 as libc::c_int as libc::c_uint {
        let mut curXStart: s32 = 0;
        let mut curXEnd: s32 = 0;
        if sFaultDrawerStruct.escCode != 0 {
            sFaultDrawerStruct.escCode = 0 as libc::c_int as u8_0;
            if *str as libc::c_int > 0x30 as libc::c_int &&
                   (*str as libc::c_int) < 0x3a as libc::c_int {
                FaultDrawer_SetForeColor(sFaultDrawerStruct.printColors[(*str
                                                                             as
                                                                             libc::c_int
                                                                             -
                                                                             0x30
                                                                                 as
                                                                                 libc::c_int)
                                                                            as
                                                                            usize]);
            }
            curXStart = sFaultDrawerStruct.cursorX as s32;
            curXEnd =
                sFaultDrawerStruct.xEnd as libc::c_int -
                    sFaultDrawerStruct.charW as libc::c_int
        } else {
            match *str as libc::c_int {
                10 => {
                    if sFaultDrawerStruct.osSyncPrintfEnabled != 0 {
                        osSyncPrintf(b"\n\x00" as *const u8 as
                                         *const libc::c_char);
                    }
                    sFaultDrawerStruct.cursorX = sFaultDrawerStruct.w;
                    curXStart = sFaultDrawerStruct.cursorX as s32;
                    curXEnd =
                        sFaultDrawerStruct.xEnd as libc::c_int -
                            sFaultDrawerStruct.charW as libc::c_int
                }
                26 => {
                    sFaultDrawerStruct.escCode = 1 as libc::c_int as u8_0;
                    curXStart = sFaultDrawerStruct.cursorX as s32;
                    curXEnd =
                        sFaultDrawerStruct.xEnd as libc::c_int -
                            sFaultDrawerStruct.charW as libc::c_int
                }
                _ => {
                    if sFaultDrawerStruct.osSyncPrintfEnabled != 0 {
                        osSyncPrintf(b"%c\x00" as *const u8 as
                                         *const libc::c_char,
                                     *str as libc::c_int);
                    }
                    FaultDrawer_DrawChar(*str);
                    sFaultDrawerStruct.cursorX =
                        (sFaultDrawerStruct.cursorX as libc::c_int +
                             (sFaultDrawerStruct.charW as libc::c_int +
                                  sFaultDrawerStruct.charWPad as libc::c_int))
                            as u16_0;
                    curXStart = sFaultDrawerStruct.cursorX as s32;
                    curXEnd =
                        sFaultDrawerStruct.xEnd as libc::c_int -
                            sFaultDrawerStruct.charW as libc::c_int
                }
            }
        }
        if curXEnd <= curXStart {
            sFaultDrawerStruct.cursorX = sFaultDrawerStruct.xStart;
            sFaultDrawerStruct.cursorY =
                (sFaultDrawerStruct.cursorY as libc::c_int +
                     (sFaultDrawerStruct.charH as libc::c_int +
                          sFaultDrawerStruct.charHPad as libc::c_int)) as
                    u16_0;
            if sFaultDrawerStruct.yEnd as libc::c_int -
                   sFaultDrawerStruct.charH as libc::c_int <=
                   sFaultDrawerStruct.cursorY as libc::c_int {
                if sFaultDrawerStruct.inputCallback.is_some() {
                    ::std::mem::transmute::<_,
                                            fn()>(sFaultDrawerStruct.inputCallback.expect("non-null function pointer"))();
                    FaultDrawer_FillScreen();
                }
                sFaultDrawerStruct.cursorY = sFaultDrawerStruct.yStart
            }
        }
        count = count.wrapping_sub(1);
        str = str.offset(1)
    }
    osWritebackDCacheAll();
    return arg;
}
#[no_mangle]
pub unsafe extern "C" fn FaultDrawer_VPrintf(mut str: *const libc::c_char,
                                             mut args: *mut libc::c_char) {
    // va_list
    _Printf(Some(FaultDrawer_FormatStringFunc as
                     unsafe extern "C" fn(_: *mut libc::c_void,
                                          _: *const libc::c_char, _: u32_0)
                         -> *mut libc::c_void),
            &mut sFaultDrawerStruct as *mut FaultDrawer as *mut libc::c_char
                as *mut libc::c_void, str, args);
}
#[no_mangle]
pub unsafe extern "C" fn FaultDrawer_Printf(mut fmt: *const libc::c_char,
                                            mut args: ...) {
    let mut args_0: __builtin_va_list = 0 as *mut libc::c_char;
    args_0 = args.clone();
    FaultDrawer_VPrintf(fmt, args_0);
}
#[no_mangle]
pub unsafe extern "C" fn FaultDrawer_DrawText(mut x: s32, mut y: s32,
                                              mut fmt: *const libc::c_char,
                                              mut args: ...) {
    let mut args_0: __builtin_va_list = 0 as *mut libc::c_char;
    args_0 = args.clone();
    FaultDrawer_SetCursor(x, y);
    FaultDrawer_VPrintf(fmt, args_0);
}
#[no_mangle]
pub unsafe extern "C" fn FaultDrawer_SetDrawerFB(mut fb: *mut libc::c_void,
                                                 mut w: u16_0, mut h: u16_0) {
    sFaultDrawerStruct.fb = fb as *mut u16_0;
    sFaultDrawerStruct.w = w;
    sFaultDrawerStruct.h = h;
}
#[no_mangle]
pub unsafe extern "C" fn FaultDrawer_SetInputCallback(mut callback:
                                                          Option<unsafe extern "C" fn()
                                                                     -> ()>) {
    sFaultDrawerStruct.inputCallback = callback;
}
#[no_mangle]
pub unsafe extern "C" fn FaultDrawer_WritebackFBDCache() {
    osWritebackDCache(sFaultDrawerStruct.fb as *mut libc::c_void,
                      sFaultDrawerStruct.w as libc::c_int *
                          sFaultDrawerStruct.h as libc::c_int *
                          2 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn FaultDrawer_SetDefault() {
    bcopy(&mut sFaultDrawerDefault as *mut FaultDrawer as *mut libc::c_void,
          &mut sFaultDrawerStruct as *mut FaultDrawer as *mut libc::c_void,
          ::std::mem::size_of::<FaultDrawer>() as libc::c_ulong);
    sFaultDrawerStruct.fb =
        (osMemSize |
             0x80000000 as
                 libc::c_uint).wrapping_sub(::std::mem::size_of::<[[u16_0; 320]; 240]>()
                                                as libc::c_ulong) as
            *mut u16_0;
}
unsafe extern "C" fn run_static_initializers() {
    sFaultDrawerDefault =
        {
            let mut init =
                FaultDrawer{fb:
                                (0x80400000 as
                                     libc::c_uint).wrapping_sub(::std::mem::size_of::<[[u16_0; 320]; 240]>()
                                                                    as
                                                                    libc::c_ulong)
                                    as *mut u16_0,
                            w: 320 as libc::c_int as u16_0,
                            h: 240 as libc::c_int as u16_0,
                            yStart: 16 as libc::c_int as u16_0,
                            yEnd: 223 as libc::c_int as u16_0,
                            xStart: 22 as libc::c_int as u16_0,
                            xEnd: 297 as libc::c_int as u16_0,
                            foreColor:
                                ((255 as libc::c_int) << 8 as libc::c_int &
                                     0xf800 as libc::c_int |
                                     (255 as libc::c_int) << 3 as libc::c_int
                                         & 0x7c0 as libc::c_int |
                                     255 as libc::c_int >> 2 as libc::c_int &
                                         0x3e as libc::c_int |
                                     255 as libc::c_int & 0x1 as libc::c_int)
                                    as u16_0,
                            backColor:
                                ((0 as libc::c_int) << 8 as libc::c_int &
                                     0xf800 as libc::c_int |
                                     (0 as libc::c_int) << 3 as libc::c_int &
                                         0x7c0 as libc::c_int |
                                     0 as libc::c_int >> 2 as libc::c_int &
                                         0x3e as libc::c_int |
                                     0 as libc::c_int & 0x1 as libc::c_int) as
                                    u16_0,
                            cursorX: 22 as libc::c_int as u16_0,
                            cursorY: 16 as libc::c_int as u16_0,
                            fontData: sFaultDrawerFont.as_ptr(),
                            charW: 8 as libc::c_int as u8_0,
                            charH: 8 as libc::c_int as u8_0,
                            charWPad: 0 as libc::c_int as s8,
                            charHPad: 0 as libc::c_int as s8,
                            printColors:
                                [((0 as libc::c_int) << 8 as libc::c_int &
                                      0xf800 as libc::c_int |
                                      (0 as libc::c_int) << 3 as libc::c_int &
                                          0x7c0 as libc::c_int |
                                      0 as libc::c_int >> 2 as libc::c_int &
                                          0x3e as libc::c_int |
                                      1 as libc::c_int & 0x1 as libc::c_int)
                                     as u16_0,
                                 ((255 as libc::c_int) << 8 as libc::c_int &
                                      0xf800 as libc::c_int |
                                      (0 as libc::c_int) << 3 as libc::c_int &
                                          0x7c0 as libc::c_int |
                                      0 as libc::c_int >> 2 as libc::c_int &
                                          0x3e as libc::c_int |
                                      1 as libc::c_int & 0x1 as libc::c_int)
                                     as u16_0,
                                 ((0 as libc::c_int) << 8 as libc::c_int &
                                      0xf800 as libc::c_int |
                                      (255 as libc::c_int) << 3 as libc::c_int
                                          & 0x7c0 as libc::c_int |
                                      0 as libc::c_int >> 2 as libc::c_int &
                                          0x3e as libc::c_int |
                                      1 as libc::c_int & 0x1 as libc::c_int)
                                     as u16_0,
                                 ((255 as libc::c_int) << 8 as libc::c_int &
                                      0xf800 as libc::c_int |
                                      (255 as libc::c_int) << 3 as libc::c_int
                                          & 0x7c0 as libc::c_int |
                                      0 as libc::c_int >> 2 as libc::c_int &
                                          0x3e as libc::c_int |
                                      1 as libc::c_int & 0x1 as libc::c_int)
                                     as u16_0,
                                 ((0 as libc::c_int) << 8 as libc::c_int &
                                      0xf800 as libc::c_int |
                                      (0 as libc::c_int) << 3 as libc::c_int &
                                          0x7c0 as libc::c_int |
                                      255 as libc::c_int >> 2 as libc::c_int &
                                          0x3e as libc::c_int |
                                      1 as libc::c_int & 0x1 as libc::c_int)
                                     as u16_0,
                                 ((255 as libc::c_int) << 8 as libc::c_int &
                                      0xf800 as libc::c_int |
                                      (0 as libc::c_int) << 3 as libc::c_int &
                                          0x7c0 as libc::c_int |
                                      255 as libc::c_int >> 2 as libc::c_int &
                                          0x3e as libc::c_int |
                                      1 as libc::c_int & 0x1 as libc::c_int)
                                     as u16_0,
                                 ((0 as libc::c_int) << 8 as libc::c_int &
                                      0xf800 as libc::c_int |
                                      (255 as libc::c_int) << 3 as libc::c_int
                                          & 0x7c0 as libc::c_int |
                                      255 as libc::c_int >> 2 as libc::c_int &
                                          0x3e as libc::c_int |
                                      1 as libc::c_int & 0x1 as libc::c_int)
                                     as u16_0,
                                 ((255 as libc::c_int) << 8 as libc::c_int &
                                      0xf800 as libc::c_int |
                                      (255 as libc::c_int) << 3 as libc::c_int
                                          & 0x7c0 as libc::c_int |
                                      255 as libc::c_int >> 2 as libc::c_int &
                                          0x3e as libc::c_int |
                                      1 as libc::c_int & 0x1 as libc::c_int)
                                     as u16_0,
                                 ((120 as libc::c_int) << 8 as libc::c_int &
                                      0xf800 as libc::c_int |
                                      (120 as libc::c_int) << 3 as libc::c_int
                                          & 0x7c0 as libc::c_int |
                                      120 as libc::c_int >> 2 as libc::c_int &
                                          0x3e as libc::c_int |
                                      1 as libc::c_int & 0x1 as libc::c_int)
                                     as u16_0,
                                 ((176 as libc::c_int) << 8 as libc::c_int &
                                      0xf800 as libc::c_int |
                                      (176 as libc::c_int) << 3 as libc::c_int
                                          & 0x7c0 as libc::c_int |
                                      176 as libc::c_int >> 2 as libc::c_int &
                                          0x3e as libc::c_int |
                                      1 as libc::c_int & 0x1 as libc::c_int)
                                     as u16_0],
                            escCode: 0 as libc::c_int as u8_0,
                            osSyncPrintfEnabled: 0 as libc::c_int as u8_0,
                            inputCallback: None,};
            init
        }
}
#[used]
#[cfg_attr(target_os = "linux", link_section = ".init_array")]
#[cfg_attr(target_os = "windows", link_section = ".CRT$XIB")]
#[cfg_attr(target_os = "macos", link_section = "__DATA,__mod_init_func")]
static INIT_ARRAY: [unsafe extern "C" fn(); 1] = [run_static_initializers];
