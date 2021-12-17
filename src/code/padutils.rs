#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case,
         non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(register_tool)]
extern "C" {
    #[no_mangle]
    fn bzero(__s: *mut libc::c_void, __n: u32_0);
}
pub type s8 = libc::c_schar;
pub type u8_0 = libc::c_uchar;
pub type u16_0 = libc::c_ushort;
pub type s32 = libc::c_int;
pub type u32_0 = libc::c_uint;
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
#[no_mangle]
pub unsafe extern "C" fn PadUtils_Init(mut input: *mut Input) {
    bzero(input as *mut libc::c_void,
          ::std::mem::size_of::<Input>() as libc::c_ulong);
}
#[no_mangle]
pub unsafe extern "C" fn func_800FCB70() { }
#[no_mangle]
pub unsafe extern "C" fn PadUtils_ResetPressRel(mut input: *mut Input) {
    (*input).press.button = 0 as libc::c_int as u16_0;
    (*input).rel.button = 0 as libc::c_int as u16_0;
}
#[no_mangle]
pub unsafe extern "C" fn PadUtils_CheckCurExact(mut input: *mut Input,
                                                mut value: u16_0) -> u32_0 {
    return (value as libc::c_int == (*input).cur.button as libc::c_int) as
               libc::c_int as u32_0;
}
#[no_mangle]
pub unsafe extern "C" fn PadUtils_CheckCur(mut input: *mut Input,
                                           mut key: u16_0) -> u32_0 {
    return (key as libc::c_int ==
                (*input).cur.button as libc::c_int & key as libc::c_int) as
               libc::c_int as u32_0;
}
#[no_mangle]
pub unsafe extern "C" fn PadUtils_CheckPressed(mut input: *mut Input,
                                               mut key: u16_0) -> u32_0 {
    return (key as libc::c_int ==
                (*input).press.button as libc::c_int & key as libc::c_int) as
               libc::c_int as u32_0;
}
#[no_mangle]
pub unsafe extern "C" fn PadUtils_CheckReleased(mut input: *mut Input,
                                                mut key: u16_0) -> u32_0 {
    return (key as libc::c_int ==
                (*input).rel.button as libc::c_int & key as libc::c_int) as
               libc::c_int as u32_0;
}
#[no_mangle]
pub unsafe extern "C" fn PadUtils_GetCurButton(mut input: *mut Input)
 -> u16_0 {
    return (*input).cur.button;
}
#[no_mangle]
pub unsafe extern "C" fn PadUtils_GetPressButton(mut input: *mut Input)
 -> u16_0 {
    return (*input).press.button;
}
#[no_mangle]
pub unsafe extern "C" fn PadUtils_GetCurX(mut input: *mut Input) -> s8 {
    return (*input).cur.stick_x;
}
#[no_mangle]
pub unsafe extern "C" fn PadUtils_GetCurY(mut input: *mut Input) -> s8 {
    return (*input).cur.stick_y;
}
#[no_mangle]
pub unsafe extern "C" fn PadUtils_SetRelXY(mut input: *mut Input, mut x: s32,
                                           mut y: s32) {
    (*input).rel.stick_x = x as s8;
    (*input).rel.stick_y = y as s8;
}
#[no_mangle]
pub unsafe extern "C" fn PadUtils_GetRelXImpl(mut input: *mut Input) -> s8 {
    return (*input).rel.stick_x;
}
#[no_mangle]
pub unsafe extern "C" fn PadUtils_GetRelYImpl(mut input: *mut Input) -> s8 {
    return (*input).rel.stick_y;
}
#[no_mangle]
pub unsafe extern "C" fn PadUtils_GetRelX(mut input: *mut Input) -> s8 {
    return PadUtils_GetRelXImpl(input);
}
#[no_mangle]
pub unsafe extern "C" fn PadUtils_GetRelY(mut input: *mut Input) -> s8 {
    return PadUtils_GetRelYImpl(input);
}
#[no_mangle]
pub unsafe extern "C" fn PadUtils_UpdateRelXY(mut input: *mut Input) {
    let mut curX: s32 = PadUtils_GetCurX(input) as s32;
    let mut curY: s32 = PadUtils_GetCurY(input) as s32;
    let mut relX: s32 = 0;
    let mut relY: s32 = 0;
    if curX > 7 as libc::c_int {
        relX =
            if curX < 0x43 as libc::c_int {
                (curX) - 7 as libc::c_int
            } else { (0x43 as libc::c_int) - 7 as libc::c_int }
    } else if curX < -(7 as libc::c_int) {
        relX =
            if curX > -(0x43 as libc::c_int) {
                (curX) + 7 as libc::c_int
            } else { (-(0x43 as libc::c_int)) + 7 as libc::c_int }
    } else { relX = 0 as libc::c_int }
    if curY > 7 as libc::c_int {
        relY =
            if curY < 0x43 as libc::c_int {
                (curY) - 7 as libc::c_int
            } else { (0x43 as libc::c_int) - 7 as libc::c_int }
    } else if curY < -(7 as libc::c_int) {
        relY =
            if curY > -(0x43 as libc::c_int) {
                (curY) + 7 as libc::c_int
            } else { (-(0x43 as libc::c_int)) + 7 as libc::c_int }
    } else { relY = 0 as libc::c_int }
    PadUtils_SetRelXY(input, relX, relY);
}
