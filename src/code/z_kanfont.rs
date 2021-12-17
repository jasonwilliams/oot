#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case,
         non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(ptr_wrapping_offset_from, register_tool)]
extern "C" {
    #[no_mangle]
    fn DmaMgr_SendRequest1(ram0: *mut libc::c_void, vrom: u32_0, size: u32_0,
                           file: *const libc::c_char, line: s32) -> s32;
    #[no_mangle]
    fn osSyncPrintf(fmt: *const libc::c_char, _: ...);
    #[no_mangle]
    static mut _nes_font_staticSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut _message_staticSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut _nes_message_data_staticSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut _nes_message_data_staticSegmentStart: [u8_0; 0];
    #[no_mangle]
    static _message_0xFFFC_nes: [libc::c_char; 0];
    #[no_mangle]
    static _message_0xFFFD_nes: [libc::c_char; 0];
}
pub type u8_0 = libc::c_uchar;
pub type u16_0 = libc::c_ushort;
pub type s32 = libc::c_int;
pub type u32_0 = libc::c_uint;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Font {
    pub msgOffset: u32_0,
    pub msgLength: u32_0,
    pub charTexBuf: [u8_0; 15360],
    pub iconBuf: [u8_0; 128],
    pub fontBuf: [u8_0; 40960],
    pub c2rust_unnamed: C2RustUnnamed,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
    pub msgBuf: [libc::c_char; 1280],
    pub msgBufWide: [u16_0; 640],
}
#[no_mangle]
pub unsafe extern "C" fn func_8006EE50(mut font: *mut Font, mut arg1: u16_0,
                                       mut arg2: u16_0) {
}
/* *
 * Loads a texture from nes_font_static for the requested `character` into the character texture buffer
 * at `codePointIndex`. The value of `character` is the ASCII codepoint subtract ' '/0x20.
 */
#[no_mangle]
pub unsafe extern "C" fn Font_LoadChar(mut font: *mut Font,
                                       mut character: u8_0,
                                       mut codePointIndex: u16_0) {
    DmaMgr_SendRequest1(&mut *(*font).charTexBuf.as_mut_ptr().offset(codePointIndex
                                                                         as
                                                                         isize)
                            as *mut u8_0 as *mut libc::c_void,
                        &mut *_nes_font_staticSegmentRomStart.as_mut_ptr().offset((character
                                                                                       as
                                                                                       libc::c_int
                                                                                       *
                                                                                       (16
                                                                                            as
                                                                                            libc::c_int
                                                                                            *
                                                                                            16
                                                                                                as
                                                                                                libc::c_int
                                                                                            /
                                                                                            2
                                                                                                as
                                                                                                libc::c_int))
                                                                                      as
                                                                                      isize)
                            as *mut u8_0 as u32_0,
                        (16 as libc::c_int * 16 as libc::c_int /
                             2 as libc::c_int) as u32_0,
                        b"../z_kanfont.c\x00" as *const u8 as
                            *const libc::c_char, 93 as libc::c_int);
}
/* *
 * Loads a message box icon from message_static, such as the ending triangle/square or choice arrow into the
 * icon buffer.
 * The different icons are given in the MessageBoxIcon enum.
 */
#[no_mangle]
pub unsafe extern "C" fn Font_LoadMessageBoxIcon(mut font: *mut Font,
                                                 mut icon: u16_0) {
    DmaMgr_SendRequest1((*font).iconBuf.as_mut_ptr() as *mut libc::c_void,
                        &mut *_message_staticSegmentRomStart.as_mut_ptr().offset((4
                                                                                      as
                                                                                      libc::c_int
                                                                                      *
                                                                                      0x1000
                                                                                          as
                                                                                          libc::c_int
                                                                                      +
                                                                                      icon
                                                                                          as
                                                                                          libc::c_int
                                                                                          *
                                                                                          (16
                                                                                               as
                                                                                               libc::c_int
                                                                                               *
                                                                                               16
                                                                                                   as
                                                                                                   libc::c_int
                                                                                               /
                                                                                               2
                                                                                                   as
                                                                                                   libc::c_int))
                                                                                     as
                                                                                     isize)
                            as *mut u8_0 as u32_0,
                        (16 as libc::c_int * 16 as libc::c_int /
                             2 as libc::c_int) as u32_0,
                        b"../z_kanfont.c\x00" as *const u8 as
                            *const libc::c_char, 100 as libc::c_int);
}
/* *
 * Loads a full set of character textures based on their ordering in the message with text id 0xFFFC into
 * the font buffer.
 */
#[no_mangle]
pub unsafe extern "C" fn Font_LoadOrderedFont(mut font: *mut Font) {
    let mut len: s32 = 0;
    let mut jj: s32 = 0;
    let mut fontStatic: s32 = 0;
    let mut fontBuf: u32_0 = 0;
    let mut codePointIndex: s32 = 0;
    let mut fontBufIndex: s32 = 0;
    let mut offset: s32 = 0;
    (*font).msgOffset =
        _message_0xFFFC_nes.as_ptr().wrapping_offset_from(_nes_message_data_staticSegmentStart.as_mut_ptr()
                                                              as
                                                              *const libc::c_char)
            as libc::c_int as u32_0;
    (*font).msgLength =
        _message_0xFFFD_nes.as_ptr().wrapping_offset_from(_message_0xFFFC_nes.as_ptr())
            as libc::c_int as u32_0;
    len = (*font).msgLength as s32;
    DmaMgr_SendRequest1((*font).c2rust_unnamed.msgBuf.as_mut_ptr() as
                            *mut libc::c_void,
                        &mut *_nes_message_data_staticSegmentRomStart.as_mut_ptr().offset((*font).msgOffset
                                                                                              as
                                                                                              isize)
                            as *mut u8_0 as u32_0, len as u32_0,
                        b"../z_kanfont.c\x00" as *const u8 as
                            *const libc::c_char, 122 as libc::c_int);
    jj = len;
    osSyncPrintf(b"msg_data=%x,  msg_data0=%x   jj=%x\n\x00" as *const u8 as
                     *const libc::c_char, (*font).msgOffset,
                 (*font).msgLength, jj);
    len = jj;
    fontBufIndex = 0 as libc::c_int;
    codePointIndex = 0 as libc::c_int;
    while (*font).c2rust_unnamed.msgBuf[codePointIndex as usize] as
              libc::c_int != 0x2 as libc::c_int {
        if codePointIndex > len {
            osSyncPrintf(b"\xef\xbc\xa5\xef\xbc\xb2\xef\xbc\xb2\xef\xbc\xaf\xef\xbc\xb2\xef\xbc\x81\xef\xbc\x81  \xe3\x82\xa8\xe3\x83\xa9\xe3\x83\xbc\xef\xbc\x81\xef\xbc\x81\xef\xbc\x81  error\xe2\x94\x80\xe2\x94\x80\xe2\x94\x80\xef\xbc\x81\xef\xbc\x81\xef\xbc\x81\xef\xbc\x81\n\x00"
                             as *const u8 as *const libc::c_char);
            return
        }
        if (*font).c2rust_unnamed.msgBuf[codePointIndex as usize] as
               libc::c_int != 0x1 as libc::c_int {
            fontBuf =
                (*font).fontBuf.as_mut_ptr().offset((fontBufIndex *
                                                         8 as libc::c_int) as
                                                        isize) as u32_0;
            fontStatic = _nes_font_staticSegmentRomStart.as_mut_ptr() as s32;
            osSyncPrintf(b"nes_mes_buf[%d]=%d\n\x00" as *const u8 as
                             *const libc::c_char, codePointIndex,
                         (*font).c2rust_unnamed.msgBuf[codePointIndex as
                                                           usize] as
                             libc::c_int);
            offset =
                ((*font).c2rust_unnamed.msgBuf[codePointIndex as usize] as
                     libc::c_int - ' ' as i32) *
                    (16 as libc::c_int * 16 as libc::c_int /
                         2 as libc::c_int);
            DmaMgr_SendRequest1(fontBuf as *mut libc::c_void,
                                (fontStatic + offset) as u32_0,
                                (16 as libc::c_int * 16 as libc::c_int /
                                     2 as libc::c_int) as u32_0,
                                b"../z_kanfont.c\x00" as *const u8 as
                                    *const libc::c_char, 134 as libc::c_int);
            fontBufIndex +=
                16 as libc::c_int * 16 as libc::c_int / 2 as libc::c_int /
                    8 as libc::c_int
        }
        codePointIndex += 1
    };
}
