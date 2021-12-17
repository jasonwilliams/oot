#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case,
         non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(register_tool)]
pub type s8 = libc::c_schar;
pub type u8_0 = libc::c_uchar;
pub type u16_0 = libc::c_ushort;
pub type s32 = libc::c_int;
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
pub struct FrameAdvanceContext {
    pub enabled: s32,
    pub timer: s32,
}
#[no_mangle]
pub unsafe extern "C" fn FrameAdvance_Init(mut frameAdvCtx:
                                               *mut FrameAdvanceContext) {
    (*frameAdvCtx).timer = 0 as libc::c_int;
    (*frameAdvCtx).enabled = 0 as libc::c_int;
}
/* *
 * Frame advance allows you to advance through the game one frame at a time on command.
 * To enable, hold R and press Dpad Down on the specified controller.
 * To advance a frame, hold Z and press R.
 * Holding Z and R will advance a frame every half second.
 *
 * This function returns true when frame advance is not active (game will run normally)
 */
#[no_mangle]
pub unsafe extern "C" fn FrameAdvance_Update(mut frameAdvCtx:
                                                 *mut FrameAdvanceContext,
                                             mut input: *mut Input) -> s32 {
    if !((*input).cur.button as libc::c_int | !(0x10 as libc::c_int)) ==
           0 as libc::c_int &&
           !((*input).press.button as libc::c_int | !(0x400 as libc::c_int))
               == 0 as libc::c_int {
        (*frameAdvCtx).enabled = ((*frameAdvCtx).enabled == 0) as libc::c_int
    }
    if (*frameAdvCtx).enabled == 0 ||
           !((*input).cur.button as libc::c_int | !(0x2000 as libc::c_int)) ==
               0 as libc::c_int &&
               (!((*input).press.button as libc::c_int |
                      !(0x10 as libc::c_int)) == 0 as libc::c_int ||
                    !((*input).cur.button as libc::c_int |
                          !(0x10 as libc::c_int)) == 0 as libc::c_int &&
                        {
                            (*frameAdvCtx).timer += 1;
                            ((*frameAdvCtx).timer) >= 9 as libc::c_int
                        }) {
        (*frameAdvCtx).timer = 0 as libc::c_int;
        return 1 as libc::c_int
    }
    return 0 as libc::c_int;
}
