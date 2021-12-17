#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case,
         non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(register_tool)]
pub type s8 = libc::c_schar;
pub type s16 = libc::c_short;
pub type s32 = libc::c_int;
pub type u32_0 = libc::c_uint;
pub type f32_0 = libc::c_float;
pub type C2RustUnnamed = libc::c_uint;
pub const CS_CMD_END: C2RustUnnamed = 65535;
pub const CS_CMD_TERMINATOR: C2RustUnnamed = 1000;
pub const CS_CMD_SETTIME: C2RustUnnamed = 140;
pub const CS_CMD_FADEBGM: C2RustUnnamed = 124;
pub const CS_CMD_STOPBGM: C2RustUnnamed = 87;
pub const CS_CMD_PLAYBGM: C2RustUnnamed = 86;
pub const CS_CMD_NOP: C2RustUnnamed = 11;
pub const CS_CMD_SCENE_TRANS_FX: C2RustUnnamed = 45;
pub const CS_CMD_SET_ACTOR_ACTION_10: C2RustUnnamed = 143;
pub const CS_CMD_SET_ACTOR_ACTION_9: C2RustUnnamed = 62;
pub const CS_CMD_SET_ACTOR_ACTION_8: C2RustUnnamed = 49;
pub const CS_CMD_SET_ACTOR_ACTION_7: C2RustUnnamed = 31;
pub const CS_CMD_SET_ACTOR_ACTION_6: C2RustUnnamed = 44;
pub const CS_CMD_SET_ACTOR_ACTION_5: C2RustUnnamed = 30;
pub const CS_CMD_SET_ACTOR_ACTION_4: C2RustUnnamed = 29;
pub const CS_CMD_SET_ACTOR_ACTION_3: C2RustUnnamed = 25;
pub const CS_CMD_SET_ACTOR_ACTION_2: C2RustUnnamed = 14;
pub const CS_CMD_SET_ACTOR_ACTION_1: C2RustUnnamed = 15;
pub const CS_CMD_SET_PLAYER_ACTION: C2RustUnnamed = 10;
pub const CS_CMD_TEXTBOX: C2RustUnnamed = 19;
pub const CS_CMD_09: C2RustUnnamed = 9;
pub const CS_CMD_08: C2RustUnnamed = 8;
pub const CS_CMD_07: C2RustUnnamed = 7;
pub const CS_CMD_CAM_AT_REL_TO_PLAYER: C2RustUnnamed = 6;
pub const CS_CMD_CAM_EYE_REL_TO_PLAYER: C2RustUnnamed = 5;
pub const CS_CMD_SET_LIGHTING: C2RustUnnamed = 4;
pub const CS_CMD_MISC: C2RustUnnamed = 3;
pub const CS_CMD_CAM_AT: C2RustUnnamed = 2;
pub const CS_CMD_CAM_EYE: C2RustUnnamed = 1;
pub const CS_CMD_00: C2RustUnnamed = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub union CutsceneData {
    pub i: s32,
    pub f: f32_0,
    pub s: [s16; 2],
    pub b: [s8; 4],
}
// clang-format off
#[no_mangle]
pub static mut gAdultWarpInToTCS: [CutsceneData; 66] =
    [CutsceneData{i: 2 as libc::c_int,}, CutsceneData{i: 118 as libc::c_int,},
     CutsceneData{i: CS_CMD_CAM_EYE_REL_TO_PLAYER as libc::c_int,},
     CutsceneData{i:
                      ((0x1 as libc::c_int as u32_0 &
                            (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           16 as libc::c_int |
                           (0 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{i:
                      ((89 as libc::c_int as u32_0 &
                            (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           16 as libc::c_int |
                           (0 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{i:
                      ((0 as libc::c_int as u32_0 &
                            (((0x1 as libc::c_int) << 8 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           24 as libc::c_int |
                           (0 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 8 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               16 as libc::c_int |
                           (0 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{f: 71.32476f32,},
     CutsceneData{i:
                      ((53 as libc::c_int as u32_0 &
                            (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           16 as libc::c_int |
                           (53 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{i:
                      ((40 as libc::c_int as u32_0 &
                            (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           16 as libc::c_int |
                           (0x10f as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{i:
                      ((0 as libc::c_int as u32_0 &
                            (((0x1 as libc::c_int) << 8 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           24 as libc::c_int |
                           (0 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 8 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               16 as libc::c_int |
                           (0 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{f: 60.0f32,},
     CutsceneData{i:
                      ((53 as libc::c_int as u32_0 &
                            (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           16 as libc::c_int |
                           (53 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{i:
                      ((40 as libc::c_int as u32_0 &
                            (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           16 as libc::c_int |
                           (0x120 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{i:
                      ((0 as libc::c_int as u32_0 &
                            (((0x1 as libc::c_int) << 8 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           24 as libc::c_int |
                           (0 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 8 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               16 as libc::c_int |
                           (0 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{f: 60.0f32,},
     CutsceneData{i:
                      ((53 as libc::c_int as u32_0 &
                            (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           16 as libc::c_int |
                           (53 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{i:
                      ((40 as libc::c_int as u32_0 &
                            (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           16 as libc::c_int |
                           (0x131 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{i:
                      ((0 as libc::c_int as u32_0 &
                            (((0x1 as libc::c_int) << 8 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           24 as libc::c_int |
                           (0 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 8 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               16 as libc::c_int |
                           (0 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{f: 70.599915f32,},
     CutsceneData{i:
                      ((58 as libc::c_int as u32_0 &
                            (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           16 as libc::c_int |
                           (102 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{i:
                      ((48 as libc::c_int as u32_0 &
                            (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           16 as libc::c_int |
                           (0x1f4 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{i:
                      ((0 as libc::c_int as u32_0 &
                            (((0x1 as libc::c_int) << 8 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           24 as libc::c_int |
                           (0 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 8 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               16 as libc::c_int |
                           (0 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{f: 70.599915f32,},
     CutsceneData{i:
                      ((58 as libc::c_int as u32_0 &
                            (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           16 as libc::c_int |
                           (102 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{i:
                      ((48 as libc::c_int as u32_0 &
                            (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           16 as libc::c_int |
                           (0x1f6 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{i:
                      ((0 as libc::c_int as u32_0 &
                            (((0x1 as libc::c_int) << 8 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           24 as libc::c_int |
                           (0 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 8 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               16 as libc::c_int |
                           (0 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{f: 70.599915f32,},
     CutsceneData{i:
                      ((58 as libc::c_int as u32_0 &
                            (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           16 as libc::c_int |
                           (102 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{i:
                      ((48 as libc::c_int as u32_0 &
                            (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           16 as libc::c_int |
                           (0x207 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{i:
                      ((-(1 as libc::c_int) as u32_0 &
                            (((0x1 as libc::c_int) << 8 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           24 as libc::c_int |
                           (0 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 8 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               16 as libc::c_int |
                           (0 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{f: 70.599915f32,},
     CutsceneData{i:
                      ((58 as libc::c_int as u32_0 &
                            (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           16 as libc::c_int |
                           (102 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{i:
                      ((48 as libc::c_int as u32_0 &
                            (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           16 as libc::c_int |
                           (0x47 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{i: CS_CMD_CAM_AT_REL_TO_PLAYER as libc::c_int,},
     CutsceneData{i:
                      ((0x1 as libc::c_int as u32_0 &
                            (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           16 as libc::c_int |
                           (0 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{i:
                      ((118 as libc::c_int as u32_0 &
                            (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           16 as libc::c_int |
                           (0 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{i:
                      ((0 as libc::c_int as u32_0 &
                            (((0x1 as libc::c_int) << 8 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           24 as libc::c_int |
                           (0 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 8 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               16 as libc::c_int |
                           (7 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{f: 70.52477f32,},
     CutsceneData{i:
                      ((11 as libc::c_int as u32_0 &
                            (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           16 as libc::c_int |
                           (29 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{i:
                      ((10 as libc::c_int as u32_0 &
                            (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           16 as libc::c_int |
                           (0x10f as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{i:
                      ((0 as libc::c_int as u32_0 &
                            (((0x1 as libc::c_int) << 8 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           24 as libc::c_int |
                           (0 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 8 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               16 as libc::c_int |
                           (7 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{f: 70.39992f32,},
     CutsceneData{i:
                      ((11 as libc::c_int as u32_0 &
                            (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           16 as libc::c_int |
                           (29 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{i:
                      ((10 as libc::c_int as u32_0 &
                            (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           16 as libc::c_int |
                           (0x120 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{i:
                      ((0 as libc::c_int as u32_0 &
                            (((0x1 as libc::c_int) << 8 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           24 as libc::c_int |
                           (0 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 8 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               16 as libc::c_int |
                           (7 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{f: 70.599915f32,},
     CutsceneData{i:
                      ((11 as libc::c_int as u32_0 &
                            (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           16 as libc::c_int |
                           (29 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{i:
                      ((10 as libc::c_int as u32_0 &
                            (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           16 as libc::c_int |
                           (0x131 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{i:
                      ((0 as libc::c_int as u32_0 &
                            (((0x1 as libc::c_int) << 8 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           24 as libc::c_int |
                           (0 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 8 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               16 as libc::c_int |
                           (7 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{f: 70.599915f32,},
     CutsceneData{i:
                      ((29 as libc::c_int as u32_0 &
                            (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           16 as libc::c_int |
                           (71 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{i:
                      ((25 as libc::c_int as u32_0 &
                            (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           16 as libc::c_int |
                           (0x1f4 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{i:
                      ((0 as libc::c_int as u32_0 &
                            (((0x1 as libc::c_int) << 8 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           24 as libc::c_int |
                           (0 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 8 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               16 as libc::c_int |
                           (30 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{f: 70.599915f32,},
     CutsceneData{i:
                      ((29 as libc::c_int as u32_0 &
                            (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           16 as libc::c_int |
                           (71 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{i:
                      ((25 as libc::c_int as u32_0 &
                            (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           16 as libc::c_int |
                           (0x1f6 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{i:
                      ((0 as libc::c_int as u32_0 &
                            (((0x1 as libc::c_int) << 8 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           24 as libc::c_int |
                           (0 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 8 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               16 as libc::c_int |
                           (30 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{f: 70.599915f32,},
     CutsceneData{i:
                      ((29 as libc::c_int as u32_0 &
                            (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           16 as libc::c_int |
                           (71 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{i:
                      ((25 as libc::c_int as u32_0 &
                            (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           16 as libc::c_int |
                           (0x207 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{i:
                      ((-(1 as libc::c_int) as u32_0 &
                            (((0x1 as libc::c_int) << 8 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           24 as libc::c_int |
                           (0 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 8 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               16 as libc::c_int |
                           (30 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{f: 70.599915f32,},
     CutsceneData{i:
                      ((29 as libc::c_int as u32_0 &
                            (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           16 as libc::c_int |
                           (71 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{i:
                      ((25 as libc::c_int as u32_0 &
                            (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           16 as libc::c_int |
                           (0x47 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{i: 0xffffffff as libc::c_uint as s32,},
     CutsceneData{i: 0 as libc::c_int,}];
// clang-format on
