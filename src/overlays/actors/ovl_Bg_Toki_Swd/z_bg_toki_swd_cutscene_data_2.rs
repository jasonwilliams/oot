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
pub type C2RustUnnamed_0 = libc::c_uint;
pub const ZELDAS_COURTYARD_RECEIVE_LETTER: C2RustUnnamed_0 = 119;
pub const GANON_BATTLE_TOWER_COLLAPSE: C2RustUnnamed_0 = 118;
pub const HYRULE_FIELD_SKY: C2RustUnnamed_0 = 117;
pub const DESERT_COLOSSUS_SPIRIT_BLUE_WARP_2: C2RustUnnamed_0 = 116;
pub const HYRULE_FIELD_AFTER_IMPA_ESCORT: C2RustUnnamed_0 = 115;
pub const HYRULE_FIELD_INTRO: C2RustUnnamed_0 = 114;
pub const GANONS_CASTLE_DISPEL_BARRIER_IF_CONDITIONS: C2RustUnnamed_0 = 113;
pub const GANONS_CASTLE_AFTER_SPIRIT_TRIAL: C2RustUnnamed_0 = 112;
pub const GANONS_CASTLE_AFTER_LIGHT_TRIAL: C2RustUnnamed_0 = 111;
pub const GANONS_CASTLE_AFTER_FIRE_TRIAL: C2RustUnnamed_0 = 110;
pub const GANONS_CASTLE_AFTER_SHADOW_TRIAL: C2RustUnnamed_0 = 109;
pub const GANONS_CASTLE_AFTER_WATER_TRIAL: C2RustUnnamed_0 = 108;
pub const GANONS_CASTLE_AFTER_FOREST_TRIAL: C2RustUnnamed_0 = 107;
pub const ROYAL_FAMILYS_TOMB_SUNS_SONG: C2RustUnnamed_0 = 106;
pub const GRAVEYARD_SUNS_SONG: C2RustUnnamed_0 = 105;
pub const SPIRIT_TEMPLE_BOSS_TITLE_SCREEN: C2RustUnnamed_0 = 104;
pub const HYRULE_FIELD_TITLE_SCREEN: C2RustUnnamed_0 = 103;
pub const TEMPLE_OF_TIME_FRONT_OF_PEDESTAL: C2RustUnnamed_0 = 102;
pub const DESERT_COLOSSUS_AFTER_SILVER_GAUNTLETS: C2RustUnnamed_0 = 101;
pub const KOKIRI_FOREST_AFTER_FOREST_BLUE_WARP: C2RustUnnamed_0 = 100;
pub const SACRED_FOREST_MEADOW_AFTER_FOREST_BLUE_WARP: C2RustUnnamed_0 = 99;
pub const DEATH_MOUNTAIN_CRATER_AFTER_FIRE_BLUE_WARP: C2RustUnnamed_0 = 98;
pub const GRAVEYARD_AFTER_SHADOW_BLUE_WARP: C2RustUnnamed_0 = 97;
pub const DESERT_COLOSSUS_SPIRIT_BLUE_WARP: C2RustUnnamed_0 = 96;
pub const CONDITIONAL_DESTINATION: C2RustUnnamed_0 = 95;
pub const LON_LON_RANCH_NO_CS_EPONAS_SONG: C2RustUnnamed_0 = 94;
pub const LON_LON_RANCH_NO_CS_15: C2RustUnnamed_0 = 93;
pub const LON_LON_RANCH_NO_CS_14: C2RustUnnamed_0 = 92;
pub const LON_LON_RANCH_NO_CS_13: C2RustUnnamed_0 = 91;
pub const LON_LON_RANCH_NO_CS_12: C2RustUnnamed_0 = 90;
pub const LON_LON_RANCH_NO_CS_11: C2RustUnnamed_0 = 89;
pub const LON_LON_RANCH_NO_CS_10: C2RustUnnamed_0 = 88;
pub const LON_LON_RANCH_NO_CS_9: C2RustUnnamed_0 = 87;
pub const LON_LON_RANCH_NO_CS_8: C2RustUnnamed_0 = 86;
pub const LON_LON_RANCH_NO_CS_7: C2RustUnnamed_0 = 85;
pub const LON_LON_RANCH_NO_CS_6: C2RustUnnamed_0 = 84;
pub const LON_LON_RANCH_NO_CS_5: C2RustUnnamed_0 = 83;
pub const LON_LON_RANCH_NO_CS_4: C2RustUnnamed_0 = 82;
pub const LON_LON_RANCH_NO_CS_3: C2RustUnnamed_0 = 81;
pub const LON_LON_RANCH_NO_CS_2: C2RustUnnamed_0 = 80;
pub const LON_LON_RANCH_NO_CS_1: C2RustUnnamed_0 = 79;
pub const LON_LON_RANCH_CREDITS_6: C2RustUnnamed_0 = 78;
pub const LON_LON_RANCH_CREDITS_5: C2RustUnnamed_0 = 77;
pub const LON_LON_RANCH_CREDITS_4: C2RustUnnamed_0 = 76;
pub const LON_LON_RANCH_CREDITS_3: C2RustUnnamed_0 = 75;
pub const LON_LON_RANCH_CREDITS_2: C2RustUnnamed_0 = 74;
pub const LON_LON_RANCH_CREDITS_1_2: C2RustUnnamed_0 = 73;
pub const ZELDAS_COURTYARD_CREDITS: C2RustUnnamed_0 = 72;
pub const TEMPLE_OF_TIME_CREDITS: C2RustUnnamed_0 = 71;
pub const DEATH_MOUNTAIN_TRAIL_CREDITS_2: C2RustUnnamed_0 = 70;
pub const KOKIRI_FOREST_POST_FOREST_MEDALLION: C2RustUnnamed_0 = 69;
pub const CUTSCENE_MAP_FIRE: C2RustUnnamed_0 = 68;
pub const HTRULE_FIELD_UNUSED_ENTRANCE: C2RustUnnamed_0 = 67;
pub const KAKARIKO_VILLAGE_AFTER_TRAIL_OWL: C2RustUnnamed_0 = 66;
pub const LON_LON_RANCH_CREDITS_1: C2RustUnnamed_0 = 65;
pub const HYRULE_FIELD_CREDITS: C2RustUnnamed_0 = 64;
pub const KOKIRI_FOREST_CREDITS_2: C2RustUnnamed_0 = 63;
pub const KOKIRI_FOREST_CREDITS_1: C2RustUnnamed_0 = 62;
pub const ZORAS_DOMAIN_CREDITS: C2RustUnnamed_0 = 61;
pub const ZORAS_FOUNTAIN_CREDITS: C2RustUnnamed_0 = 60;
pub const LAKE_HYLIA_CREDITS: C2RustUnnamed_0 = 59;
pub const GORON_CITY_CREDITS: C2RustUnnamed_0 = 58;
pub const DEATH_MOUNTAIN_TRAIL_CREDITS_1: C2RustUnnamed_0 = 57;
pub const KAKARIKO_VILLAGE_CREDITS: C2RustUnnamed_0 = 56;
pub const GERUDO_FORTRESS_CREDITS: C2RustUnnamed_0 = 55;
pub const GERUDO_VALLEY_CREDITS: C2RustUnnamed_0 = 54;
pub const HYRULE_FIELD_AFTER_SONG_OF_TIME: C2RustUnnamed_0 = 53;
pub const TEMPLE_OF_TIME_SONG_OF_TIME: C2RustUnnamed_0 = 52;
pub const HYRULE_FIELD_IMPA_ESCORT_CS: C2RustUnnamed_0 = 51;
pub const KAKARIKO_VILLAGE_AFTER_NOCTURNE: C2RustUnnamed_0 = 50;
pub const TEMPLE_OF_TIME_AFTER_LIGHT_ARROWS: C2RustUnnamed_0 = 49;
pub const DESERT_COLOSSUS_AFTER_REQUIEM: C2RustUnnamed_0 = 48;
pub const KAKARIKO_VILLAGE_NOCTURNE_PART_2: C2RustUnnamed_0 = 47;
pub const TEMPLE_OF_TIME_AFTER_USE_MS_FIRST_2: C2RustUnnamed_0 = 46;
pub const INVALID_DESTINATION_2D: C2RustUnnamed_0 = 45;
pub const TEMPLE_OF_TIME_AFTER_DOOR_OF_TIME_OPENS: C2RustUnnamed_0 = 44;
pub const WINDMILL_AFTER_DRAIN_WELL: C2RustUnnamed_0 = 43;
pub const KAKARIKO_VILLAGE_DRAIN_WELL: C2RustUnnamed_0 = 42;
pub const LAKE_HYLIA_AFTER_BLUE_WARP: C2RustUnnamed_0 = 41;
pub const TEMPLE_OF_TIME_GET_LIGHT_ARROWS: C2RustUnnamed_0 = 40;
pub const TEMPLE_OF_TIME_ZELDA_REVEAL: C2RustUnnamed_0 = 39;
pub const CUTSCENE_MAP_SHEIKAH_LEGEND: C2RustUnnamed_0 = 38;
pub const INVALID_DESTINATION_25: C2RustUnnamed_0 = 37;
pub const INVALID_DESTINATION_24: C2RustUnnamed_0 = 36;
pub const HYRULE_FIELD_INTRO_ZELDA_ESCAPE: C2RustUnnamed_0 = 35;
pub const CUTSCENE_MAP_GANON_AFTER_USE_MS: C2RustUnnamed_0 = 34;
pub const HYRULE_FIELD_AFTER_LAKE_HYLIA_OWL: C2RustUnnamed_0 = 33;
pub const HYRULE_FIELD_FLASHBACK: C2RustUnnamed_0 = 32;
pub const CHAMBER_OF_SAGES_WATER_MEDALLION: C2RustUnnamed_0 = 31;
pub const CHAMBER_OF_SAGES_FIRE_MEDALLION: C2RustUnnamed_0 = 30;
pub const CHAMBER_OF_SAGES_FOREST_MEDALLION: C2RustUnnamed_0 = 29;
pub const TEMPLE_OF_TIME_ZORAS_SAPPHIRE_2: C2RustUnnamed_0 = 28;
pub const TEMPLE_OF_TIME_GORON_RUBY_2: C2RustUnnamed_0 = 27;
pub const TEMPLE_OF_TIME_KOKIRI_EMERALD_2: C2RustUnnamed_0 = 26;
pub const CHAMBER_OF_SAGES_LIGHT_MEDALLION: C2RustUnnamed_0 = 25;
pub const JABU_JABU_INTRO: C2RustUnnamed_0 = 24;
pub const CUTSCENE_MAP_CURSE_YOU: C2RustUnnamed_0 = 23;
pub const DESERT_COLOSSUS_REQUIEM: C2RustUnnamed_0 = 22;
pub const LAKE_HYLIA_WATER_RISES: C2RustUnnamed_0 = 21;
pub const INVALID_DESTINATION_14: C2RustUnnamed_0 = 20;
pub const DEATH_MOUNTAIN_TRAIL_AFTER_INTRO: C2RustUnnamed_0 = 19;
pub const TEMPLE_OF_TIME_AFTER_USE_MS_FIRST: C2RustUnnamed_0 = 18;
pub const TEMPLE_OF_TIME_ZORAS_SAPPHIRE: C2RustUnnamed_0 = 17;
pub const TEMPLE_OF_TIME_GORON_RUBY: C2RustUnnamed_0 = 16;
pub const TEMPLE_OF_TIME_KOKIRI_EMERALD: C2RustUnnamed_0 = 15;
pub const KOKIRI_FOREST_AFTER_KOKIRI_EMERALD: C2RustUnnamed_0 = 14;
pub const ZORAS_FOUNTAIN_AFTER_ZORAS_SAPPHIRE: C2RustUnnamed_0 = 13;
pub const DEATH_MOUNTAIN_TRAIL_AFTER_GORON_RUBY: C2RustUnnamed_0 = 12;
pub const KOKIRI_FOREST_INTRO: C2RustUnnamed_0 = 11;
pub const LINKS_HOUSE_INTRO: C2RustUnnamed_0 = 10;
pub const GERUDO_VALLEY_DIN_2: C2RustUnnamed_0 = 9;
pub const TEMPLE_OF_TIME_AFTER_USE_MS: C2RustUnnamed_0 = 8;
pub const KOKIRI_FOREST_RECEIVE_KOKIRI_EMERALD: C2RustUnnamed_0 = 7;
pub const CUTSCENE_MAP_TRIFORCE_CREATION: C2RustUnnamed_0 = 6;
pub const KOKIRI_FOREST_FARORE: C2RustUnnamed_0 = 5;
pub const DEATH_MOUNTAIN_TRAIL_NAYRU: C2RustUnnamed_0 = 4;
pub const GERUDO_VALLEY_DIN: C2RustUnnamed_0 = 3;
pub const CUTSCENE_MAP_THREE_GODESSES_POST_DEKU_TREE: C2RustUnnamed_0 = 2;
pub const CUTSCENE_MAP_GANON_HORSE: C2RustUnnamed_0 = 1;
pub const INVALID_DESTINATION_0: C2RustUnnamed_0 = 0;
// clang-format off
#[no_mangle]
pub static mut D_808BB7A0: [CutsceneData; 378] =
    [CutsceneData{i: 9 as libc::c_int,}, CutsceneData{i: 368 as libc::c_int,},
     CutsceneData{i: CS_CMD_CAM_EYE as libc::c_int,},
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
                      ((126 as libc::c_int as u32_0 &
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
     CutsceneData{f: 60.0f32,},
     CutsceneData{i:
                      ((-(1 as libc::c_int) as u32_0 &
                            (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           16 as libc::c_int |
                           (101 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{i:
                      ((-(110 as libc::c_int) as u32_0 &
                            (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           16 as libc::c_int |
                           (0x616d as libc::c_int as u32_0 &
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
     CutsceneData{f: 40.0f32,},
     CutsceneData{i:
                      ((-(1 as libc::c_int) as u32_0 &
                            (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           16 as libc::c_int |
                           (101 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{i:
                      ((-(110 as libc::c_int) as u32_0 &
                            (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           16 as libc::c_int |
                           (0x6964 as libc::c_int as u32_0 &
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
     CutsceneData{f: 40.0f32,},
     CutsceneData{i:
                      ((-(1 as libc::c_int) as u32_0 &
                            (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           16 as libc::c_int |
                           (101 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{i:
                      ((-(110 as libc::c_int) as u32_0 &
                            (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           16 as libc::c_int |
                           (0xfb as libc::c_int as u32_0 &
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
     CutsceneData{f: 40.0f32,},
     CutsceneData{i:
                      ((-(1 as libc::c_int) as u32_0 &
                            (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           16 as libc::c_int |
                           (101 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{i:
                      ((-(110 as libc::c_int) as u32_0 &
                            (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           16 as libc::c_int |
                           (0x111 as libc::c_int as u32_0 &
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
     CutsceneData{f: 40.0f32,},
     CutsceneData{i:
                      ((-(1 as libc::c_int) as u32_0 &
                            (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           16 as libc::c_int |
                           (101 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{i:
                      ((-(109 as libc::c_int) as u32_0 &
                            (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           16 as libc::c_int |
                           (0x12d as libc::c_int as u32_0 &
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
     CutsceneData{f: 40.0f32,},
     CutsceneData{i:
                      ((-(1 as libc::c_int) as u32_0 &
                            (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           16 as libc::c_int |
                           (71 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{i:
                      ((-(110 as libc::c_int) as u32_0 &
                            (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           16 as libc::c_int |
                           (0x33 as libc::c_int as u32_0 &
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
     CutsceneData{f: 40.0f32,},
     CutsceneData{i:
                      ((-(1 as libc::c_int) as u32_0 &
                            (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           16 as libc::c_int |
                           (72 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{i:
                      ((-(110 as libc::c_int) as u32_0 &
                            (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           16 as libc::c_int |
                           (0xfb as libc::c_int as u32_0 &
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
     CutsceneData{f: 40.0f32,},
     CutsceneData{i:
                      ((-(1 as libc::c_int) as u32_0 &
                            (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           16 as libc::c_int |
                           (72 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{i:
                      ((-(110 as libc::c_int) as u32_0 &
                            (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           16 as libc::c_int |
                           (0xfb as libc::c_int as u32_0 &
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
     CutsceneData{f: 40.0f32,},
     CutsceneData{i:
                      ((-(1 as libc::c_int) as u32_0 &
                            (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           16 as libc::c_int |
                           (72 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{i:
                      ((-(110 as libc::c_int) as u32_0 &
                            (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           16 as libc::c_int |
                           (0x2f68 as libc::c_int as u32_0 &
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
     CutsceneData{f: 40.0f32,},
     CutsceneData{i:
                      ((-(1 as libc::c_int) as u32_0 &
                            (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           16 as libc::c_int |
                           (72 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{i:
                      ((-(110 as libc::c_int) as u32_0 &
                            (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           16 as libc::c_int |
                           (0x612f as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{i: CS_CMD_CAM_EYE as libc::c_int,},
     CutsceneData{i:
                      ((0x1 as libc::c_int as u32_0 &
                            (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           16 as libc::c_int |
                           (58 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{i:
                      ((339 as libc::c_int as u32_0 &
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
     CutsceneData{f: 60.000004f32,},
     CutsceneData{i:
                      ((-(9 as libc::c_int) as u32_0 &
                            (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           16 as libc::c_int |
                           (106 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{i:
                      ((-(3 as libc::c_int) as u32_0 &
                            (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           16 as libc::c_int |
                           (0xa1bc as libc::c_int as u32_0 &
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
                      ((-(9 as libc::c_int) as u32_0 &
                            (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           16 as libc::c_int |
                           (106 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{i:
                      ((-(3 as libc::c_int) as u32_0 &
                            (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           16 as libc::c_int |
                           (0xa5f3 as libc::c_int as u32_0 &
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
                      ((-(9 as libc::c_int) as u32_0 &
                            (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           16 as libc::c_int |
                           (106 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{i:
                      ((-(3 as libc::c_int) as u32_0 &
                            (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           16 as libc::c_int |
                           (0xa5eb as libc::c_int as u32_0 &
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
                      ((-(9 as libc::c_int) as u32_0 &
                            (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           16 as libc::c_int |
                           (106 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{i:
                      ((-(3 as libc::c_int) as u32_0 &
                            (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           16 as libc::c_int |
                           (0xa5a2 as libc::c_int as u32_0 &
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
                      ((-(67 as libc::c_int) as u32_0 &
                            (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           16 as libc::c_int |
                           (108 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{i:
                      ((-(53 as libc::c_int) as u32_0 &
                            (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           16 as libc::c_int |
                           (0xbaee as libc::c_int as u32_0 &
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
                      ((-(67 as libc::c_int) as u32_0 &
                            (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           16 as libc::c_int |
                           (108 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{i:
                      ((-(53 as libc::c_int) as u32_0 &
                            (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           16 as libc::c_int |
                           (0xc9b8 as libc::c_int as u32_0 &
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
                      ((-(67 as libc::c_int) as u32_0 &
                            (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           16 as libc::c_int |
                           (108 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{i:
                      ((-(53 as libc::c_int) as u32_0 &
                            (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           16 as libc::c_int |
                           (0x20 as libc::c_int as u32_0 &
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
                      ((-(67 as libc::c_int) as u32_0 &
                            (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           16 as libc::c_int |
                           (108 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{i:
                      ((-(53 as libc::c_int) as u32_0 &
                            (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           16 as libc::c_int |
                           (0xa5a4 as libc::c_int as u32_0 &
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
                      ((-(67 as libc::c_int) as u32_0 &
                            (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           16 as libc::c_int |
                           (108 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{i:
                      ((-(53 as libc::c_int) as u32_0 &
                            (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           16 as libc::c_int |
                           (0x20 as libc::c_int as u32_0 &
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
                      ((-(83 as libc::c_int) as u32_0 &
                            (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           16 as libc::c_int |
                           (102 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{i:
                      ((-(42 as libc::c_int) as u32_0 &
                            (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           16 as libc::c_int |
                           (0xa5e1 as libc::c_int as u32_0 &
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
                      ((-(116 as libc::c_int) as u32_0 &
                            (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           16 as libc::c_int |
                           (81 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{i:
                      ((-(19 as libc::c_int) as u32_0 &
                            (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           16 as libc::c_int |
                           (0xa5d0 as libc::c_int as u32_0 &
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
                      ((-(118 as libc::c_int) as u32_0 &
                            (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           16 as libc::c_int |
                           (81 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{i:
                      ((54 as libc::c_int as u32_0 &
                            (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           16 as libc::c_int |
                           (0x20 as libc::c_int as u32_0 &
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
                      ((-(106 as libc::c_int) as u32_0 &
                            (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           16 as libc::c_int |
                           (78 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{i:
                      ((116 as libc::c_int as u32_0 &
                            (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           16 as libc::c_int |
                           (0xa5a6 as libc::c_int as u32_0 &
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
                      ((-(27 as libc::c_int) as u32_0 &
                            (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           16 as libc::c_int |
                           (75 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{i:
                      ((226 as libc::c_int as u32_0 &
                            (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           16 as libc::c_int |
                           (0xa5e5 as libc::c_int as u32_0 &
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
                      ((121 as libc::c_int as u32_0 &
                            (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           16 as libc::c_int |
                           (82 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{i:
                      ((312 as libc::c_int as u32_0 &
                            (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           16 as libc::c_int |
                           (0xc0ae as libc::c_int as u32_0 &
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
                      ((296 as libc::c_int as u32_0 &
                            (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           16 as libc::c_int |
                           (76 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{i:
                      ((374 as libc::c_int as u32_0 &
                            (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           16 as libc::c_int |
                           (0xa5e5 as libc::c_int as u32_0 &
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
                      ((565 as libc::c_int as u32_0 &
                            (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           16 as libc::c_int |
                           (80 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{i:
                      ((257 as libc::c_int as u32_0 &
                            (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           16 as libc::c_int |
                           (0x20 as libc::c_int as u32_0 &
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
                      ((614 as libc::c_int as u32_0 &
                            (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           16 as libc::c_int |
                           (80 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{i:
                      ((-(1 as libc::c_int) as u32_0 &
                            (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           16 as libc::c_int |
                           (0xa5eb as libc::c_int as u32_0 &
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
                      ((594 as libc::c_int as u32_0 &
                            (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           16 as libc::c_int |
                           (80 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{i:
                      ((-(145 as libc::c_int) as u32_0 &
                            (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           16 as libc::c_int |
                           (0xa5e1 as libc::c_int as u32_0 &
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
                      ((475 as libc::c_int as u32_0 &
                            (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           16 as libc::c_int |
                           (80 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{i:
                      ((-(380 as libc::c_int) as u32_0 &
                            (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           16 as libc::c_int |
                           (0x20 as libc::c_int as u32_0 &
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
                      ((323 as libc::c_int as u32_0 &
                            (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           16 as libc::c_int |
                           (80 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{i:
                      ((-(513 as libc::c_int) as u32_0 &
                            (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           16 as libc::c_int |
                           (0xa5e1 as libc::c_int as u32_0 &
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
                      ((192 as libc::c_int as u32_0 &
                            (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           16 as libc::c_int |
                           (80 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{i:
                      ((-(574 as libc::c_int) as u32_0 &
                            (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           16 as libc::c_int |
                           (0xa5d0 as libc::c_int as u32_0 &
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
                      ((-(45 as libc::c_int) as u32_0 &
                            (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           16 as libc::c_int |
                           (80 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{i:
                      ((-(604 as libc::c_int) as u32_0 &
                            (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           16 as libc::c_int |
                           (0x20 as libc::c_int as u32_0 &
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
                      ((-(142 as libc::c_int) as u32_0 &
                            (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           16 as libc::c_int |
                           (80 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{i:
                      ((-(589 as libc::c_int) as u32_0 &
                            (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           16 as libc::c_int |
                           (0xa5a6 as libc::c_int as u32_0 &
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
                      ((-(228 as libc::c_int) as u32_0 &
                            (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           16 as libc::c_int |
                           (80 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{i:
                      ((-(562 as libc::c_int) as u32_0 &
                            (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           16 as libc::c_int |
                           (0xa5e5 as libc::c_int as u32_0 &
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
                      ((-(376 as libc::c_int) as u32_0 &
                            (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           16 as libc::c_int |
                           (93 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{i:
                      ((-(477 as libc::c_int) as u32_0 &
                            (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           16 as libc::c_int |
                           (0xc0ae as libc::c_int as u32_0 &
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
                      ((-(584 as libc::c_int) as u32_0 &
                            (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           16 as libc::c_int |
                           (95 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{i:
                      ((-(183 as libc::c_int) as u32_0 &
                            (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           16 as libc::c_int |
                           (0xa5e5 as libc::c_int as u32_0 &
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
                      ((-(613 as libc::c_int) as u32_0 &
                            (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           16 as libc::c_int |
                           (95 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{i:
                      ((-(34 as libc::c_int) as u32_0 &
                            (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           16 as libc::c_int |
                           (0x20 as libc::c_int as u32_0 &
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
     CutsceneData{f: 60.0f32,},
     CutsceneData{i:
                      ((-(595 as libc::c_int) as u32_0 &
                            (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           16 as libc::c_int |
                           (95 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{i:
                      ((161 as libc::c_int as u32_0 &
                            (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           16 as libc::c_int |
                           (0xa5eb as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{i: CS_CMD_CAM_AT as libc::c_int,},
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
                      ((155 as libc::c_int as u32_0 &
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
                           (5 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{f: 40.4f32,},
     CutsceneData{i:
                      ((-(1 as libc::c_int) as u32_0 &
                            (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           16 as libc::c_int |
                           (111 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{i:
                      ((5 as libc::c_int as u32_0 &
                            (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           16 as libc::c_int |
                           (0xa1bc as libc::c_int as u32_0 &
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
                           (5 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{f: 40.0f32,},
     CutsceneData{i:
                      ((-(1 as libc::c_int) as u32_0 &
                            (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           16 as libc::c_int |
                           (111 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{i:
                      ((5 as libc::c_int as u32_0 &
                            (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           16 as libc::c_int |
                           (0xa5f3 as libc::c_int as u32_0 &
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
                           (5 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{f: 40.0f32,},
     CutsceneData{i:
                      ((-(1 as libc::c_int) as u32_0 &
                            (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           16 as libc::c_int |
                           (111 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{i:
                      ((5 as libc::c_int as u32_0 &
                            (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           16 as libc::c_int |
                           (0xa5eb as libc::c_int as u32_0 &
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
                           (15 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{f: 40.0f32,},
     CutsceneData{i:
                      ((-(1 as libc::c_int) as u32_0 &
                            (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           16 as libc::c_int |
                           (111 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{i:
                      ((5 as libc::c_int as u32_0 &
                            (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           16 as libc::c_int |
                           (0xa5a2 as libc::c_int as u32_0 &
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
                           (15 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{f: 40.0f32,},
     CutsceneData{i:
                      ((-(1 as libc::c_int) as u32_0 &
                            (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           16 as libc::c_int |
                           (111 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{i:
                      ((5 as libc::c_int as u32_0 &
                            (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           16 as libc::c_int |
                           (0xbaee as libc::c_int as u32_0 &
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
                           (15 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{f: 40.0f32,},
     CutsceneData{i:
                      ((-(1 as libc::c_int) as u32_0 &
                            (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           16 as libc::c_int |
                           (104 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{i:
                      ((8 as libc::c_int as u32_0 &
                            (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           16 as libc::c_int |
                           (0xc9b8 as libc::c_int as u32_0 &
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
                           (5 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{f: 40.0f32,},
     CutsceneData{i:
                      ((-(1 as libc::c_int) as u32_0 &
                            (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           16 as libc::c_int |
                           (104 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{i:
                      ((8 as libc::c_int as u32_0 &
                            (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           16 as libc::c_int |
                           (0x20 as libc::c_int as u32_0 &
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
     CutsceneData{f: 40.0f32,},
     CutsceneData{i:
                      ((-(1 as libc::c_int) as u32_0 &
                            (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           16 as libc::c_int |
                           (104 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{i:
                      ((8 as libc::c_int as u32_0 &
                            (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           16 as libc::c_int |
                           (0xa5a4 as libc::c_int as u32_0 &
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
     CutsceneData{f: 40.0f32,},
     CutsceneData{i:
                      ((-(1 as libc::c_int) as u32_0 &
                            (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           16 as libc::c_int |
                           (104 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{i:
                      ((8 as libc::c_int as u32_0 &
                            (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           16 as libc::c_int |
                           (0x20 as libc::c_int as u32_0 &
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
     CutsceneData{f: 40.0f32,},
     CutsceneData{i:
                      ((-(1 as libc::c_int) as u32_0 &
                            (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           16 as libc::c_int |
                           (104 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{i:
                      ((8 as libc::c_int as u32_0 &
                            (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           16 as libc::c_int |
                           (0xa5e1 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{i: CS_CMD_CAM_AT as libc::c_int,},
     CutsceneData{i:
                      ((0x1 as libc::c_int as u32_0 &
                            (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           16 as libc::c_int |
                           (58 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{i:
                      ((368 as libc::c_int as u32_0 &
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
                           (5 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{f: 60.000004f32,},
     CutsceneData{i:
                      ((0 as libc::c_int as u32_0 &
                            (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           16 as libc::c_int |
                           (120 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{i:
                      ((12 as libc::c_int as u32_0 &
                            (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           16 as libc::c_int |
                           (0xa1bc as libc::c_int as u32_0 &
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
                           (5 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{f: 60.0f32,},
     CutsceneData{i:
                      ((0 as libc::c_int as u32_0 &
                            (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           16 as libc::c_int |
                           (120 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{i:
                      ((12 as libc::c_int as u32_0 &
                            (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           16 as libc::c_int |
                           (0xa5f3 as libc::c_int as u32_0 &
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
                           (5 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{f: 60.0f32,},
     CutsceneData{i:
                      ((0 as libc::c_int as u32_0 &
                            (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           16 as libc::c_int |
                           (120 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{i:
                      ((12 as libc::c_int as u32_0 &
                            (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           16 as libc::c_int |
                           (0xa5eb as libc::c_int as u32_0 &
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
                           (10 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{f: 60.0f32,},
     CutsceneData{i:
                      ((0 as libc::c_int as u32_0 &
                            (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           16 as libc::c_int |
                           (120 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{i:
                      ((12 as libc::c_int as u32_0 &
                            (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           16 as libc::c_int |
                           (0xa5a2 as libc::c_int as u32_0 &
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
                           (5 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{f: 60.0f32,},
     CutsceneData{i:
                      ((-(1 as libc::c_int) as u32_0 &
                            (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           16 as libc::c_int |
                           (95 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{i:
                      ((11 as libc::c_int as u32_0 &
                            (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           16 as libc::c_int |
                           (0xbaee as libc::c_int as u32_0 &
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
                           (5 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{f: 60.0f32,},
     CutsceneData{i:
                      ((-(1 as libc::c_int) as u32_0 &
                            (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           16 as libc::c_int |
                           (95 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{i:
                      ((11 as libc::c_int as u32_0 &
                            (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           16 as libc::c_int |
                           (0xc9b8 as libc::c_int as u32_0 &
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
                           (5 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{f: 60.0f32,},
     CutsceneData{i:
                      ((-(1 as libc::c_int) as u32_0 &
                            (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           16 as libc::c_int |
                           (95 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{i:
                      ((11 as libc::c_int as u32_0 &
                            (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           16 as libc::c_int |
                           (0x20 as libc::c_int as u32_0 &
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
                           (5 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{f: 60.0f32,},
     CutsceneData{i:
                      ((-(1 as libc::c_int) as u32_0 &
                            (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           16 as libc::c_int |
                           (95 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{i:
                      ((11 as libc::c_int as u32_0 &
                            (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           16 as libc::c_int |
                           (0xa5a4 as libc::c_int as u32_0 &
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
                           (5 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{f: 60.0f32,},
     CutsceneData{i:
                      ((-(1 as libc::c_int) as u32_0 &
                            (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           16 as libc::c_int |
                           (95 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{i:
                      ((11 as libc::c_int as u32_0 &
                            (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           16 as libc::c_int |
                           (0x20 as libc::c_int as u32_0 &
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
                           (10 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{f: 60.0f32,},
     CutsceneData{i:
                      ((-(3 as libc::c_int) as u32_0 &
                            (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           16 as libc::c_int |
                           (95 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{i:
                      ((8 as libc::c_int as u32_0 &
                            (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           16 as libc::c_int |
                           (0xa5e1 as libc::c_int as u32_0 &
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
                           (10 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{f: 60.0f32,},
     CutsceneData{i:
                      ((-(3 as libc::c_int) as u32_0 &
                            (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           16 as libc::c_int |
                           (95 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{i:
                      ((8 as libc::c_int as u32_0 &
                            (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           16 as libc::c_int |
                           (0xa5d0 as libc::c_int as u32_0 &
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
                           (10 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{f: 60.0f32,},
     CutsceneData{i:
                      ((-(3 as libc::c_int) as u32_0 &
                            (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           16 as libc::c_int |
                           (95 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{i:
                      ((8 as libc::c_int as u32_0 &
                            (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           16 as libc::c_int |
                           (0x20 as libc::c_int as u32_0 &
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
                           (10 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{f: 60.0f32,},
     CutsceneData{i:
                      ((-(3 as libc::c_int) as u32_0 &
                            (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           16 as libc::c_int |
                           (95 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{i:
                      ((8 as libc::c_int as u32_0 &
                            (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           16 as libc::c_int |
                           (0xa5a6 as libc::c_int as u32_0 &
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
                           (10 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{f: 60.0f32,},
     CutsceneData{i:
                      ((-(3 as libc::c_int) as u32_0 &
                            (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           16 as libc::c_int |
                           (95 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{i:
                      ((8 as libc::c_int as u32_0 &
                            (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           16 as libc::c_int |
                           (0xa5e5 as libc::c_int as u32_0 &
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
                           (10 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{f: 60.0f32,},
     CutsceneData{i:
                      ((-(3 as libc::c_int) as u32_0 &
                            (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           16 as libc::c_int |
                           (95 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{i:
                      ((8 as libc::c_int as u32_0 &
                            (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           16 as libc::c_int |
                           (0xc0ae as libc::c_int as u32_0 &
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
                           (10 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{f: 60.0f32,},
     CutsceneData{i:
                      ((-(3 as libc::c_int) as u32_0 &
                            (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           16 as libc::c_int |
                           (95 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{i:
                      ((8 as libc::c_int as u32_0 &
                            (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           16 as libc::c_int |
                           (0xa5e5 as libc::c_int as u32_0 &
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
                           (10 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{f: 60.0f32,},
     CutsceneData{i:
                      ((-(3 as libc::c_int) as u32_0 &
                            (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           16 as libc::c_int |
                           (95 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{i:
                      ((8 as libc::c_int as u32_0 &
                            (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           16 as libc::c_int |
                           (0x20 as libc::c_int as u32_0 &
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
                           (10 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{f: 60.0f32,},
     CutsceneData{i:
                      ((-(3 as libc::c_int) as u32_0 &
                            (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           16 as libc::c_int |
                           (95 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{i:
                      ((8 as libc::c_int as u32_0 &
                            (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           16 as libc::c_int |
                           (0xa5eb as libc::c_int as u32_0 &
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
                           (10 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{f: 60.0f32,},
     CutsceneData{i:
                      ((-(3 as libc::c_int) as u32_0 &
                            (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           16 as libc::c_int |
                           (95 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{i:
                      ((8 as libc::c_int as u32_0 &
                            (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           16 as libc::c_int |
                           (0xa5e1 as libc::c_int as u32_0 &
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
                           (10 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{f: 60.0f32,},
     CutsceneData{i:
                      ((-(3 as libc::c_int) as u32_0 &
                            (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           16 as libc::c_int |
                           (95 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{i:
                      ((8 as libc::c_int as u32_0 &
                            (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           16 as libc::c_int |
                           (0x20 as libc::c_int as u32_0 &
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
                           (15 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{f: 60.0f32,},
     CutsceneData{i:
                      ((-(3 as libc::c_int) as u32_0 &
                            (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           16 as libc::c_int |
                           (95 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{i:
                      ((8 as libc::c_int as u32_0 &
                            (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           16 as libc::c_int |
                           (0xa5e1 as libc::c_int as u32_0 &
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
                           (15 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{f: 60.0f32,},
     CutsceneData{i:
                      ((-(3 as libc::c_int) as u32_0 &
                            (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           16 as libc::c_int |
                           (95 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{i:
                      ((8 as libc::c_int as u32_0 &
                            (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           16 as libc::c_int |
                           (0xa5d0 as libc::c_int as u32_0 &
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
                           (15 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{f: 60.0f32,},
     CutsceneData{i:
                      ((-(3 as libc::c_int) as u32_0 &
                            (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           16 as libc::c_int |
                           (95 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{i:
                      ((8 as libc::c_int as u32_0 &
                            (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           16 as libc::c_int |
                           (0x20 as libc::c_int as u32_0 &
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
                           (15 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{f: 60.0f32,},
     CutsceneData{i:
                      ((-(3 as libc::c_int) as u32_0 &
                            (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           16 as libc::c_int |
                           (95 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{i:
                      ((8 as libc::c_int as u32_0 &
                            (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           16 as libc::c_int |
                           (0xa5a6 as libc::c_int as u32_0 &
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
                           (15 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{f: 60.0f32,},
     CutsceneData{i:
                      ((-(3 as libc::c_int) as u32_0 &
                            (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           16 as libc::c_int |
                           (95 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{i:
                      ((8 as libc::c_int as u32_0 &
                            (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           16 as libc::c_int |
                           (0xa5e5 as libc::c_int as u32_0 &
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
                           (15 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{f: 60.0f32,},
     CutsceneData{i:
                      ((-(3 as libc::c_int) as u32_0 &
                            (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           16 as libc::c_int |
                           (95 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{i:
                      ((8 as libc::c_int as u32_0 &
                            (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           16 as libc::c_int |
                           (0xc0ae as libc::c_int as u32_0 &
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
                           (15 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{f: 60.0f32,},
     CutsceneData{i:
                      ((-(3 as libc::c_int) as u32_0 &
                            (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           16 as libc::c_int |
                           (95 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{i:
                      ((8 as libc::c_int as u32_0 &
                            (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           16 as libc::c_int |
                           (0xa5e5 as libc::c_int as u32_0 &
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
                           (15 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{f: 60.0f32,},
     CutsceneData{i:
                      ((-(3 as libc::c_int) as u32_0 &
                            (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           16 as libc::c_int |
                           (95 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{i:
                      ((8 as libc::c_int as u32_0 &
                            (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           16 as libc::c_int |
                           (0x20 as libc::c_int as u32_0 &
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
     CutsceneData{f: 60.0f32,},
     CutsceneData{i:
                      ((-(3 as libc::c_int) as u32_0 &
                            (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           16 as libc::c_int |
                           (95 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{i:
                      ((8 as libc::c_int as u32_0 &
                            (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           16 as libc::c_int |
                           (0xa5eb as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{i: CS_CMD_SET_PLAYER_ACTION as libc::c_int,},
     CutsceneData{i: 1 as libc::c_int,},
     CutsceneData{i:
                      ((0xc as libc::c_int as u32_0 &
                            (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           16 as libc::c_int |
                           (0 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{i:
                      ((180 as libc::c_int as u32_0 &
                            (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           16 as libc::c_int |
                           (0 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{i:
                      ((0 as libc::c_int as u32_0 &
                            (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           16 as libc::c_int |
                           (0 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{i: 0 as libc::c_int,}, CutsceneData{i: 28 as libc::c_int,},
     CutsceneData{i: -(10 as libc::c_int),},
     CutsceneData{i: 0 as libc::c_int,},
     CutsceneData{i: -(14 as libc::c_int),},
     CutsceneData{i: 9 as libc::c_int,}, CutsceneData{f: 0.0f32,},
     CutsceneData{f: -0.23333333f32,}, CutsceneData{f: 0.0f32,},
     CutsceneData{i: CS_CMD_SET_LIGHTING as libc::c_int,},
     CutsceneData{i: 1 as libc::c_int,},
     CutsceneData{i:
                      ((0x2 as libc::c_int as u32_0 &
                            (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           16 as libc::c_int |
                           (90 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{i:
                      ((91 as libc::c_int as u32_0 &
                            (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           16 as libc::c_int |
                           (0 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{i: 0 as libc::c_int,}, CutsceneData{i: 0 as libc::c_int,},
     CutsceneData{i: 0xfffffff2 as libc::c_uint as s32,},
     CutsceneData{i: 0x24 as libc::c_int,},
     CutsceneData{i: 0 as libc::c_int,},
     CutsceneData{i: 0xfffffff2 as libc::c_uint as s32,},
     CutsceneData{i: 0x24 as libc::c_int,},
     CutsceneData{i: 0 as libc::c_int,}, CutsceneData{i: 0 as libc::c_int,},
     CutsceneData{i: 0 as libc::c_int,},
     CutsceneData{i: CS_CMD_MISC as libc::c_int,},
     CutsceneData{i: 1 as libc::c_int,},
     CutsceneData{i:
                      ((0xa as libc::c_int as u32_0 &
                            (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           16 as libc::c_int |
                           (90 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{i:
                      ((91 as libc::c_int as u32_0 &
                            (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           16 as libc::c_int |
                           (0 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{i: 0 as libc::c_int,}, CutsceneData{i: 0 as libc::c_int,},
     CutsceneData{i: 0xfffffff8 as libc::c_uint as s32,},
     CutsceneData{i: 0xffffffdd as libc::c_uint as s32,},
     CutsceneData{i: 0 as libc::c_int,},
     CutsceneData{i: 0xfffffff8 as libc::c_uint as s32,},
     CutsceneData{i: 0xffffffdd as libc::c_uint as s32,},
     CutsceneData{i: 0 as libc::c_int,}, CutsceneData{i: 0 as libc::c_int,},
     CutsceneData{i: 0 as libc::c_int,},
     CutsceneData{i: CS_CMD_SCENE_TRANS_FX as libc::c_int,},
     CutsceneData{i: 0x1 as libc::c_int,},
     CutsceneData{i:
                      ((0x1 as libc::c_int as u32_0 &
                            (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           16 as libc::c_int |
                           (190 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{i:
                      ((210 as libc::c_int as u32_0 &
                            (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           16 as libc::c_int |
                           (210 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{i: CS_CMD_TERMINATOR as libc::c_int,},
     CutsceneData{i: 0x1 as libc::c_int,},
     CutsceneData{i:
                      ((TEMPLE_OF_TIME_AFTER_USE_MS as libc::c_int as u32_0 &
                            (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           16 as libc::c_int |
                           (210 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{i:
                      ((211 as libc::c_int as u32_0 &
                            (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           16 as libc::c_int |
                           (211 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{i: 0xffffffff as libc::c_uint as s32,},
     CutsceneData{i: 0 as libc::c_int,}];
// clang-format on
