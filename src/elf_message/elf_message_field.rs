#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case,
         non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(register_tool)]
pub type u8_0 = libc::c_uchar;
pub type u32_0 = libc::c_uint;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ElfMessage {
    pub byte0: u8_0,
    pub byte1: u8_0,
    pub byte2: u8_0,
    pub byte3: u8_0,
}
pub type C2RustUnnamed = libc::c_uint;
pub const ITEM_NONE: C2RustUnnamed = 255;
pub const ITEM_NONE_FE: C2RustUnnamed = 254;
pub const ITEM_LAST_USED: C2RustUnnamed = 252;
pub const ITEM_NUT_UPGRADE_40: C2RustUnnamed = 155;
pub const ITEM_NUT_UPGRADE_30: C2RustUnnamed = 154;
pub const ITEM_STICK_UPGRADE_30: C2RustUnnamed = 153;
pub const ITEM_STICK_UPGRADE_20: C2RustUnnamed = 152;
pub const ITEM_BOMBCHUS_20: C2RustUnnamed = 151;
pub const ITEM_BOMBCHUS_5: C2RustUnnamed = 150;
pub const ITEM_SEEDS_30: C2RustUnnamed = 149;
pub const ITEM_ARROWS_LARGE: C2RustUnnamed = 148;
pub const ITEM_ARROWS_MEDIUM: C2RustUnnamed = 147;
pub const ITEM_ARROWS_SMALL: C2RustUnnamed = 146;
pub const ITEM_BOMBS_30: C2RustUnnamed = 145;
pub const ITEM_BOMBS_20: C2RustUnnamed = 144;
pub const ITEM_BOMBS_10: C2RustUnnamed = 143;
pub const ITEM_BOMBS_5: C2RustUnnamed = 142;
pub const ITEM_NUTS_10: C2RustUnnamed = 141;
pub const ITEM_NUTS_5: C2RustUnnamed = 140;
pub const ITEM_STICKS_10: C2RustUnnamed = 139;
pub const ITEM_STICKS_5: C2RustUnnamed = 138;
pub const ITEM_INVALID_8: C2RustUnnamed = 137;
pub const ITEM_RUPEE_GOLD: C2RustUnnamed = 136;
pub const ITEM_RUPEE_PURPLE: C2RustUnnamed = 135;
pub const ITEM_RUPEE_RED: C2RustUnnamed = 134;
pub const ITEM_RUPEE_BLUE: C2RustUnnamed = 133;
pub const ITEM_RUPEE_GREEN: C2RustUnnamed = 132;
pub const ITEM_HEART: C2RustUnnamed = 131;
pub const ITEM_MILK: C2RustUnnamed = 130;
pub const ITEM_INVALID_7: C2RustUnnamed = 129;
pub const ITEM_INVALID_6: C2RustUnnamed = 128;
pub const ITEM_INVALID_5: C2RustUnnamed = 127;
pub const ITEM_INVALID_4: C2RustUnnamed = 126;
pub const ITEM_INVALID_3: C2RustUnnamed = 125;
pub const ITEM_INVALID_2: C2RustUnnamed = 124;
pub const ITEM_INVALID_1: C2RustUnnamed = 123;
pub const ITEM_HEART_PIECE_2: C2RustUnnamed = 122;
pub const ITEM_MAGIC_LARGE: C2RustUnnamed = 121;
pub const ITEM_MAGIC_SMALL: C2RustUnnamed = 120;
pub const ITEM_KEY_SMALL: C2RustUnnamed = 119;
pub const ITEM_DUNGEON_MAP: C2RustUnnamed = 118;
pub const ITEM_COMPASS: C2RustUnnamed = 117;
pub const ITEM_KEY_BOSS: C2RustUnnamed = 116;
pub const ITEM_HEART_PIECE: C2RustUnnamed = 115;
pub const ITEM_HEART_CONTAINER: C2RustUnnamed = 114;
pub const ITEM_SKULL_TOKEN: C2RustUnnamed = 113;
pub const ITEM_GERUDO_CARD: C2RustUnnamed = 112;
pub const ITEM_STONE_OF_AGONY: C2RustUnnamed = 111;
pub const ITEM_ZORA_SAPPHIRE: C2RustUnnamed = 110;
pub const ITEM_GORON_RUBY: C2RustUnnamed = 109;
pub const ITEM_KOKIRI_EMERALD: C2RustUnnamed = 108;
pub const ITEM_MEDALLION_LIGHT: C2RustUnnamed = 107;
pub const ITEM_MEDALLION_SHADOW: C2RustUnnamed = 106;
pub const ITEM_MEDALLION_SPIRIT: C2RustUnnamed = 105;
pub const ITEM_MEDALLION_WATER: C2RustUnnamed = 104;
pub const ITEM_MEDALLION_FIRE: C2RustUnnamed = 103;
pub const ITEM_MEDALLION_FOREST: C2RustUnnamed = 102;
pub const ITEM_SONG_STORMS: C2RustUnnamed = 101;
pub const ITEM_SONG_TIME: C2RustUnnamed = 100;
pub const ITEM_SONG_SUN: C2RustUnnamed = 99;
pub const ITEM_SONG_SARIA: C2RustUnnamed = 98;
pub const ITEM_SONG_EPONA: C2RustUnnamed = 97;
pub const ITEM_SONG_LULLABY: C2RustUnnamed = 96;
pub const ITEM_SONG_PRELUDE: C2RustUnnamed = 95;
pub const ITEM_SONG_NOCTURNE: C2RustUnnamed = 94;
pub const ITEM_SONG_REQUIEM: C2RustUnnamed = 93;
pub const ITEM_SONG_SERENADE: C2RustUnnamed = 92;
pub const ITEM_SONG_BOLERO: C2RustUnnamed = 91;
pub const ITEM_SONG_MINUET: C2RustUnnamed = 90;
pub const ITEM_FISHING_POLE: C2RustUnnamed = 89;
pub const ITEM_SEEDS: C2RustUnnamed = 88;
pub const ITEM_WALLET_GIANT: C2RustUnnamed = 87;
pub const ITEM_WALLET_ADULT: C2RustUnnamed = 86;
pub const ITEM_SWORD_KNIFE: C2RustUnnamed = 85;
pub const ITEM_SCALE_GOLDEN: C2RustUnnamed = 84;
pub const ITEM_SCALE_SILVER: C2RustUnnamed = 83;
pub const ITEM_GAUNTLETS_GOLD: C2RustUnnamed = 82;
pub const ITEM_GAUNTLETS_SILVER: C2RustUnnamed = 81;
pub const ITEM_BRACELET: C2RustUnnamed = 80;
pub const ITEM_BOMB_BAG_40: C2RustUnnamed = 79;
pub const ITEM_BOMB_BAG_30: C2RustUnnamed = 78;
pub const ITEM_BOMB_BAG_20: C2RustUnnamed = 77;
pub const ITEM_QUIVER_50: C2RustUnnamed = 76;
pub const ITEM_QUIVER_40: C2RustUnnamed = 75;
pub const ITEM_QUIVER_30: C2RustUnnamed = 74;
pub const ITEM_BULLET_BAG_50: C2RustUnnamed = 73;
pub const ITEM_BULLET_BAG_40: C2RustUnnamed = 72;
pub const ITEM_BULLET_BAG_30: C2RustUnnamed = 71;
pub const ITEM_BOOTS_HOVER: C2RustUnnamed = 70;
pub const ITEM_BOOTS_IRON: C2RustUnnamed = 69;
pub const ITEM_BOOTS_KOKIRI: C2RustUnnamed = 68;
pub const ITEM_TUNIC_ZORA: C2RustUnnamed = 67;
pub const ITEM_TUNIC_GORON: C2RustUnnamed = 66;
pub const ITEM_TUNIC_KOKIRI: C2RustUnnamed = 65;
pub const ITEM_SHIELD_MIRROR: C2RustUnnamed = 64;
pub const ITEM_SHIELD_HYLIAN: C2RustUnnamed = 63;
pub const ITEM_SHIELD_DEKU: C2RustUnnamed = 62;
pub const ITEM_SWORD_BGS: C2RustUnnamed = 61;
pub const ITEM_SWORD_MASTER: C2RustUnnamed = 60;
pub const ITEM_SWORD_KOKIRI: C2RustUnnamed = 59;
pub const ITEM_BOW_ARROW_LIGHT: C2RustUnnamed = 58;
pub const ITEM_BOW_ARROW_ICE: C2RustUnnamed = 57;
pub const ITEM_BOW_ARROW_FIRE: C2RustUnnamed = 56;
pub const ITEM_CLAIM_CHECK: C2RustUnnamed = 55;
pub const ITEM_EYEDROPS: C2RustUnnamed = 54;
pub const ITEM_FROG: C2RustUnnamed = 53;
pub const ITEM_PRESCRIPTION: C2RustUnnamed = 52;
pub const ITEM_SWORD_BROKEN: C2RustUnnamed = 51;
pub const ITEM_SAW: C2RustUnnamed = 50;
pub const ITEM_ODD_POTION: C2RustUnnamed = 49;
pub const ITEM_ODD_MUSHROOM: C2RustUnnamed = 48;
pub const ITEM_COJIRO: C2RustUnnamed = 47;
pub const ITEM_POCKET_CUCCO: C2RustUnnamed = 46;
pub const ITEM_POCKET_EGG: C2RustUnnamed = 45;
pub const ITEM_SOLD_OUT: C2RustUnnamed = 44;
pub const ITEM_MASK_TRUTH: C2RustUnnamed = 43;
pub const ITEM_MASK_GERUDO: C2RustUnnamed = 42;
pub const ITEM_MASK_ZORA: C2RustUnnamed = 41;
pub const ITEM_MASK_GORON: C2RustUnnamed = 40;
pub const ITEM_MASK_BUNNY: C2RustUnnamed = 39;
pub const ITEM_MASK_SPOOKY: C2RustUnnamed = 38;
pub const ITEM_MASK_SKULL: C2RustUnnamed = 37;
pub const ITEM_MASK_KEATON: C2RustUnnamed = 36;
pub const ITEM_LETTER_ZELDA: C2RustUnnamed = 35;
pub const ITEM_CHICKEN: C2RustUnnamed = 34;
pub const ITEM_WEIRD_EGG: C2RustUnnamed = 33;
pub const ITEM_POE: C2RustUnnamed = 32;
pub const ITEM_MILK_HALF: C2RustUnnamed = 31;
pub const ITEM_BIG_POE: C2RustUnnamed = 30;
pub const ITEM_BUG: C2RustUnnamed = 29;
pub const ITEM_BLUE_FIRE: C2RustUnnamed = 28;
pub const ITEM_LETTER_RUTO: C2RustUnnamed = 27;
pub const ITEM_MILK_BOTTLE: C2RustUnnamed = 26;
pub const ITEM_FISH: C2RustUnnamed = 25;
pub const ITEM_FAIRY: C2RustUnnamed = 24;
pub const ITEM_POTION_BLUE: C2RustUnnamed = 23;
pub const ITEM_POTION_GREEN: C2RustUnnamed = 22;
pub const ITEM_POTION_RED: C2RustUnnamed = 21;
pub const ITEM_BOTTLE: C2RustUnnamed = 20;
pub const ITEM_NAYRUS_LOVE: C2RustUnnamed = 19;
pub const ITEM_ARROW_LIGHT: C2RustUnnamed = 18;
pub const ITEM_HAMMER: C2RustUnnamed = 17;
pub const ITEM_BEAN: C2RustUnnamed = 16;
pub const ITEM_LENS: C2RustUnnamed = 15;
pub const ITEM_BOOMERANG: C2RustUnnamed = 14;
pub const ITEM_FARORES_WIND: C2RustUnnamed = 13;
pub const ITEM_ARROW_ICE: C2RustUnnamed = 12;
pub const ITEM_LONGSHOT: C2RustUnnamed = 11;
pub const ITEM_HOOKSHOT: C2RustUnnamed = 10;
pub const ITEM_BOMBCHU: C2RustUnnamed = 9;
pub const ITEM_OCARINA_TIME: C2RustUnnamed = 8;
pub const ITEM_OCARINA_FAIRY: C2RustUnnamed = 7;
pub const ITEM_SLINGSHOT: C2RustUnnamed = 6;
pub const ITEM_DINS_FIRE: C2RustUnnamed = 5;
pub const ITEM_ARROW_FIRE: C2RustUnnamed = 4;
pub const ITEM_BOW: C2RustUnnamed = 3;
pub const ITEM_BOMB: C2RustUnnamed = 2;
pub const ITEM_NUT: C2RustUnnamed = 1;
pub const ITEM_STICK: C2RustUnnamed = 0;
#[no_mangle]
pub static mut gOverworldNaviMsgs: [ElfMessage; 28] =
    [{
         let mut init =
             ElfMessage{byte0:
                            ((0 as libc::c_int as u32_0 &
                                  (((0x1 as libc::c_int) << 3 as libc::c_int)
                                       - 1 as libc::c_int) as libc::c_uint) <<
                                 5 as libc::c_int |
                                 (0 as libc::c_int as u32_0 &
                                      (((0x1 as libc::c_int) <<
                                            4 as libc::c_int) -
                                           1 as libc::c_int) as libc::c_uint)
                                     << 1 as libc::c_int |
                                 (0 as libc::c_int as u32_0 &
                                      (((0x1 as libc::c_int) <<
                                            1 as libc::c_int) -
                                           1 as libc::c_int) as libc::c_uint)
                                     << 0 as libc::c_int) as u8_0,
                        byte1:
                            ((0x5 as libc::c_int as u32_0 &
                                  (((0x1 as libc::c_int) << 8 as libc::c_int)
                                       - 1 as libc::c_int) as libc::c_uint) <<
                                 0 as libc::c_int) as u8_0,
                        byte2:
                            ((0x40 as libc::c_int as u32_0 &
                                  (((0x1 as libc::c_int) << 8 as libc::c_int)
                                       - 1 as libc::c_int) as libc::c_uint) <<
                                 0 as libc::c_int) as u8_0,
                        byte3:
                            ((0 as libc::c_int as u32_0 &
                                  (((0x1 as libc::c_int) << 8 as libc::c_int)
                                       - 1 as libc::c_int) as libc::c_uint) <<
                                 0 as libc::c_int) as u8_0,};
         init
     },
     {
         let mut init =
             ElfMessage{byte0:
                            ((0 as libc::c_int as u32_0 &
                                  (((0x1 as libc::c_int) << 3 as libc::c_int)
                                       - 1 as libc::c_int) as libc::c_uint) <<
                                 5 as libc::c_int |
                                 (0 as libc::c_int as u32_0 &
                                      (((0x1 as libc::c_int) <<
                                            4 as libc::c_int) -
                                           1 as libc::c_int) as libc::c_uint)
                                     << 1 as libc::c_int |
                                 (0 as libc::c_int as u32_0 &
                                      (((0x1 as libc::c_int) <<
                                            1 as libc::c_int) -
                                           1 as libc::c_int) as libc::c_uint)
                                     << 0 as libc::c_int) as u8_0,
                        byte1:
                            ((0x9 as libc::c_int as u32_0 &
                                  (((0x1 as libc::c_int) << 8 as libc::c_int)
                                       - 1 as libc::c_int) as libc::c_uint) <<
                                 0 as libc::c_int) as u8_0,
                        byte2:
                            ((0x41 as libc::c_int as u32_0 &
                                  (((0x1 as libc::c_int) << 8 as libc::c_int)
                                       - 1 as libc::c_int) as libc::c_uint) <<
                                 0 as libc::c_int) as u8_0,
                        byte3:
                            ((0 as libc::c_int as u32_0 &
                                  (((0x1 as libc::c_int) << 8 as libc::c_int)
                                       - 1 as libc::c_int) as libc::c_uint) <<
                                 0 as libc::c_int) as u8_0,};
         init
     },
     {
         let mut init =
             ElfMessage{byte0:
                            ((0 as libc::c_int as u32_0 &
                                  (((0x1 as libc::c_int) << 3 as libc::c_int)
                                       - 1 as libc::c_int) as libc::c_uint) <<
                                 5 as libc::c_int |
                                 (0 as libc::c_int as u32_0 &
                                      (((0x1 as libc::c_int) <<
                                            4 as libc::c_int) -
                                           1 as libc::c_int) as libc::c_uint)
                                     << 1 as libc::c_int |
                                 (0 as libc::c_int as u32_0 &
                                      (((0x1 as libc::c_int) <<
                                            1 as libc::c_int) -
                                           1 as libc::c_int) as libc::c_uint)
                                     << 0 as libc::c_int) as u8_0,
                        byte1:
                            ((0x12 as libc::c_int as u32_0 &
                                  (((0x1 as libc::c_int) << 8 as libc::c_int)
                                       - 1 as libc::c_int) as libc::c_uint) <<
                                 0 as libc::c_int) as u8_0,
                        byte2:
                            ((0x42 as libc::c_int as u32_0 &
                                  (((0x1 as libc::c_int) << 8 as libc::c_int)
                                       - 1 as libc::c_int) as libc::c_uint) <<
                                 0 as libc::c_int) as u8_0,
                        byte3:
                            ((0 as libc::c_int as u32_0 &
                                  (((0x1 as libc::c_int) << 8 as libc::c_int)
                                       - 1 as libc::c_int) as libc::c_uint) <<
                                 0 as libc::c_int) as u8_0,};
         init
     },
     {
         let mut init =
             ElfMessage{byte0:
                            ((0 as libc::c_int as u32_0 &
                                  (((0x1 as libc::c_int) << 3 as libc::c_int)
                                       - 1 as libc::c_int) as libc::c_uint) <<
                                 5 as libc::c_int |
                                 (0 as libc::c_int as u32_0 &
                                      (((0x1 as libc::c_int) <<
                                            4 as libc::c_int) -
                                           1 as libc::c_int) as libc::c_uint)
                                     << 1 as libc::c_int |
                                 (0 as libc::c_int as u32_0 &
                                      (((0x1 as libc::c_int) <<
                                            1 as libc::c_int) -
                                           1 as libc::c_int) as libc::c_uint)
                                     << 0 as libc::c_int) as u8_0,
                        byte1:
                            ((0x14 as libc::c_int as u32_0 &
                                  (((0x1 as libc::c_int) << 8 as libc::c_int)
                                       - 1 as libc::c_int) as libc::c_uint) <<
                                 0 as libc::c_int) as u8_0,
                        byte2:
                            ((0x43 as libc::c_int as u32_0 &
                                  (((0x1 as libc::c_int) << 8 as libc::c_int)
                                       - 1 as libc::c_int) as libc::c_uint) <<
                                 0 as libc::c_int) as u8_0,
                        byte3:
                            ((0 as libc::c_int as u32_0 &
                                  (((0x1 as libc::c_int) << 8 as libc::c_int)
                                       - 1 as libc::c_int) as libc::c_uint) <<
                                 0 as libc::c_int) as u8_0,};
         init
     },
     {
         let mut init =
             ElfMessage{byte0:
                            ((0 as libc::c_int as u32_0 &
                                  (((0x1 as libc::c_int) << 3 as libc::c_int)
                                       - 1 as libc::c_int) as libc::c_uint) <<
                                 5 as libc::c_int |
                                 (0 as libc::c_int as u32_0 &
                                      (((0x1 as libc::c_int) <<
                                            4 as libc::c_int) -
                                           1 as libc::c_int) as libc::c_uint)
                                     << 1 as libc::c_int |
                                 (0 as libc::c_int as u32_0 &
                                      (((0x1 as libc::c_int) <<
                                            1 as libc::c_int) -
                                           1 as libc::c_int) as libc::c_uint)
                                     << 0 as libc::c_int) as u8_0,
                        byte1:
                            ((0x40 as libc::c_int as u32_0 &
                                  (((0x1 as libc::c_int) << 8 as libc::c_int)
                                       - 1 as libc::c_int) as libc::c_uint) <<
                                 0 as libc::c_int) as u8_0,
                        byte2:
                            ((0x44 as libc::c_int as u32_0 &
                                  (((0x1 as libc::c_int) << 8 as libc::c_int)
                                       - 1 as libc::c_int) as libc::c_uint) <<
                                 0 as libc::c_int) as u8_0,
                        byte3:
                            ((0 as libc::c_int as u32_0 &
                                  (((0x1 as libc::c_int) << 8 as libc::c_int)
                                       - 1 as libc::c_int) as libc::c_uint) <<
                                 0 as libc::c_int) as u8_0,};
         init
     },
     {
         let mut init =
             ElfMessage{byte0:
                            ((0 as libc::c_int as u32_0 &
                                  (((0x1 as libc::c_int) << 3 as libc::c_int)
                                       - 1 as libc::c_int) as libc::c_uint) <<
                                 5 as libc::c_int |
                                 (3 as libc::c_int as u32_0 &
                                      (((0x1 as libc::c_int) <<
                                            4 as libc::c_int) -
                                           1 as libc::c_int) as libc::c_uint)
                                     << 1 as libc::c_int |
                                 (0 as libc::c_int as u32_0 &
                                      (((0x1 as libc::c_int) <<
                                            1 as libc::c_int) -
                                           1 as libc::c_int) as libc::c_uint)
                                     << 0 as libc::c_int) as u8_0,
                        byte1:
                            ((2 as libc::c_int as u32_0 &
                                  (((0x1 as libc::c_int) << 4 as libc::c_int)
                                       - 1 as libc::c_int) as libc::c_uint) <<
                                 4 as libc::c_int |
                                 (0 as libc::c_int as u32_0 &
                                      (((0x1 as libc::c_int) <<
                                            4 as libc::c_int) -
                                           1 as libc::c_int) as libc::c_uint)
                                     << 0 as libc::c_int) as u8_0,
                        byte2:
                            ((0x45 as libc::c_int as u32_0 &
                                  (((0x1 as libc::c_int) << 8 as libc::c_int)
                                       - 1 as libc::c_int) as libc::c_uint) <<
                                 0 as libc::c_int) as u8_0,
                        byte3:
                            ((ITEM_SONG_SARIA as libc::c_int as u32_0 &
                                  (((0x1 as libc::c_int) << 8 as libc::c_int)
                                       - 1 as libc::c_int) as libc::c_uint) <<
                                 0 as libc::c_int) as u8_0,};
         init
     },
     {
         let mut init =
             ElfMessage{byte0:
                            ((0 as libc::c_int as u32_0 &
                                  (((0x1 as libc::c_int) << 3 as libc::c_int)
                                       - 1 as libc::c_int) as libc::c_uint) <<
                                 5 as libc::c_int |
                                 (3 as libc::c_int as u32_0 &
                                      (((0x1 as libc::c_int) <<
                                            4 as libc::c_int) -
                                           1 as libc::c_int) as libc::c_uint)
                                     << 1 as libc::c_int |
                                 (1 as libc::c_int as u32_0 &
                                      (((0x1 as libc::c_int) <<
                                            1 as libc::c_int) -
                                           1 as libc::c_int) as libc::c_uint)
                                     << 0 as libc::c_int) as u8_0,
                        byte1:
                            ((0 as libc::c_int as u32_0 &
                                  (((0x1 as libc::c_int) << 4 as libc::c_int)
                                       - 1 as libc::c_int) as libc::c_uint) <<
                                 4 as libc::c_int |
                                 (0 as libc::c_int as u32_0 &
                                      (((0x1 as libc::c_int) <<
                                            4 as libc::c_int) -
                                           1 as libc::c_int) as libc::c_uint)
                                     << 0 as libc::c_int) as u8_0,
                        byte2:
                            ((0x46 as libc::c_int as u32_0 &
                                  (((0x1 as libc::c_int) << 8 as libc::c_int)
                                       - 1 as libc::c_int) as libc::c_uint) <<
                                 0 as libc::c_int) as u8_0,
                        byte3:
                            ((0 as libc::c_int as u32_0 &
                                  (((0x1 as libc::c_int) << 8 as libc::c_int)
                                       - 1 as libc::c_int) as libc::c_uint) <<
                                 0 as libc::c_int) as u8_0,};
         init
     },
     {
         let mut init =
             ElfMessage{byte0:
                            ((0 as libc::c_int as u32_0 &
                                  (((0x1 as libc::c_int) << 3 as libc::c_int)
                                       - 1 as libc::c_int) as libc::c_uint) <<
                                 5 as libc::c_int |
                                 (0 as libc::c_int as u32_0 &
                                      (((0x1 as libc::c_int) <<
                                            4 as libc::c_int) -
                                           1 as libc::c_int) as libc::c_uint)
                                     << 1 as libc::c_int |
                                 (0 as libc::c_int as u32_0 &
                                      (((0x1 as libc::c_int) <<
                                            1 as libc::c_int) -
                                           1 as libc::c_int) as libc::c_uint)
                                     << 0 as libc::c_int) as u8_0,
                        byte1:
                            ((0x25 as libc::c_int as u32_0 &
                                  (((0x1 as libc::c_int) << 8 as libc::c_int)
                                       - 1 as libc::c_int) as libc::c_uint) <<
                                 0 as libc::c_int) as u8_0,
                        byte2:
                            ((0x47 as libc::c_int as u32_0 &
                                  (((0x1 as libc::c_int) << 8 as libc::c_int)
                                       - 1 as libc::c_int) as libc::c_uint) <<
                                 0 as libc::c_int) as u8_0,
                        byte3:
                            ((0 as libc::c_int as u32_0 &
                                  (((0x1 as libc::c_int) << 8 as libc::c_int)
                                       - 1 as libc::c_int) as libc::c_uint) <<
                                 0 as libc::c_int) as u8_0,};
         init
     },
     {
         let mut init =
             ElfMessage{byte0:
                            ((0 as libc::c_int as u32_0 &
                                  (((0x1 as libc::c_int) << 3 as libc::c_int)
                                       - 1 as libc::c_int) as libc::c_uint) <<
                                 5 as libc::c_int |
                                 (3 as libc::c_int as u32_0 &
                                      (((0x1 as libc::c_int) <<
                                            4 as libc::c_int) -
                                           1 as libc::c_int) as libc::c_uint)
                                     << 1 as libc::c_int |
                                 (0 as libc::c_int as u32_0 &
                                      (((0x1 as libc::c_int) <<
                                            1 as libc::c_int) -
                                           1 as libc::c_int) as libc::c_uint)
                                     << 0 as libc::c_int) as u8_0,
                        byte1:
                            ((4 as libc::c_int as u32_0 &
                                  (((0x1 as libc::c_int) << 4 as libc::c_int)
                                       - 1 as libc::c_int) as libc::c_uint) <<
                                 4 as libc::c_int |
                                 (0 as libc::c_int as u32_0 &
                                      (((0x1 as libc::c_int) <<
                                            4 as libc::c_int) -
                                           1 as libc::c_int) as libc::c_uint)
                                     << 0 as libc::c_int) as u8_0,
                        byte2:
                            ((0x48 as libc::c_int as u32_0 &
                                  (((0x1 as libc::c_int) << 8 as libc::c_int)
                                       - 1 as libc::c_int) as libc::c_uint) <<
                                 0 as libc::c_int) as u8_0,
                        byte3:
                            ((0 as libc::c_int as u32_0 &
                                  (((0x1 as libc::c_int) << 8 as libc::c_int)
                                       - 1 as libc::c_int) as libc::c_uint) <<
                                 0 as libc::c_int) as u8_0,};
         init
     },
     {
         let mut init =
             ElfMessage{byte0:
                            ((0 as libc::c_int as u32_0 &
                                  (((0x1 as libc::c_int) << 3 as libc::c_int)
                                       - 1 as libc::c_int) as libc::c_uint) <<
                                 5 as libc::c_int |
                                 (0 as libc::c_int as u32_0 &
                                      (((0x1 as libc::c_int) <<
                                            4 as libc::c_int) -
                                           1 as libc::c_int) as libc::c_uint)
                                     << 1 as libc::c_int |
                                 (0 as libc::c_int as u32_0 &
                                      (((0x1 as libc::c_int) <<
                                            1 as libc::c_int) -
                                           1 as libc::c_int) as libc::c_uint)
                                     << 0 as libc::c_int) as u8_0,
                        byte1:
                            ((0x33 as libc::c_int as u32_0 &
                                  (((0x1 as libc::c_int) << 8 as libc::c_int)
                                       - 1 as libc::c_int) as libc::c_uint) <<
                                 0 as libc::c_int) as u8_0,
                        byte2:
                            ((0x49 as libc::c_int as u32_0 &
                                  (((0x1 as libc::c_int) << 8 as libc::c_int)
                                       - 1 as libc::c_int) as libc::c_uint) <<
                                 0 as libc::c_int) as u8_0,
                        byte3:
                            ((0 as libc::c_int as u32_0 &
                                  (((0x1 as libc::c_int) << 8 as libc::c_int)
                                       - 1 as libc::c_int) as libc::c_uint) <<
                                 0 as libc::c_int) as u8_0,};
         init
     },
     {
         let mut init =
             ElfMessage{byte0:
                            ((0 as libc::c_int as u32_0 &
                                  (((0x1 as libc::c_int) << 3 as libc::c_int)
                                       - 1 as libc::c_int) as libc::c_uint) <<
                                 5 as libc::c_int |
                                 (0 as libc::c_int as u32_0 &
                                      (((0x1 as libc::c_int) <<
                                            4 as libc::c_int) -
                                           1 as libc::c_int) as libc::c_uint)
                                     << 1 as libc::c_int |
                                 (0 as libc::c_int as u32_0 &
                                      (((0x1 as libc::c_int) <<
                                            1 as libc::c_int) -
                                           1 as libc::c_int) as libc::c_uint)
                                     << 0 as libc::c_int) as u8_0,
                        byte1:
                            ((0x37 as libc::c_int as u32_0 &
                                  (((0x1 as libc::c_int) << 8 as libc::c_int)
                                       - 1 as libc::c_int) as libc::c_uint) <<
                                 0 as libc::c_int) as u8_0,
                        byte2:
                            ((0x4a as libc::c_int as u32_0 &
                                  (((0x1 as libc::c_int) << 8 as libc::c_int)
                                       - 1 as libc::c_int) as libc::c_uint) <<
                                 0 as libc::c_int) as u8_0,
                        byte3:
                            ((0 as libc::c_int as u32_0 &
                                  (((0x1 as libc::c_int) << 8 as libc::c_int)
                                       - 1 as libc::c_int) as libc::c_uint) <<
                                 0 as libc::c_int) as u8_0,};
         init
     },
     {
         let mut init =
             ElfMessage{byte0:
                            ((0 as libc::c_int as u32_0 &
                                  (((0x1 as libc::c_int) << 3 as libc::c_int)
                                       - 1 as libc::c_int) as libc::c_uint) <<
                                 5 as libc::c_int |
                                 (0 as libc::c_int as u32_0 &
                                      (((0x1 as libc::c_int) <<
                                            4 as libc::c_int) -
                                           1 as libc::c_int) as libc::c_uint)
                                     << 1 as libc::c_int |
                                 (0 as libc::c_int as u32_0 &
                                      (((0x1 as libc::c_int) <<
                                            1 as libc::c_int) -
                                           1 as libc::c_int) as libc::c_uint)
                                     << 0 as libc::c_int) as u8_0,
                        byte1:
                            ((0x80 as libc::c_int as u32_0 &
                                  (((0x1 as libc::c_int) << 8 as libc::c_int)
                                       - 1 as libc::c_int) as libc::c_uint) <<
                                 0 as libc::c_int) as u8_0,
                        byte2:
                            ((0x4b as libc::c_int as u32_0 &
                                  (((0x1 as libc::c_int) << 8 as libc::c_int)
                                       - 1 as libc::c_int) as libc::c_uint) <<
                                 0 as libc::c_int) as u8_0,
                        byte3:
                            ((0 as libc::c_int as u32_0 &
                                  (((0x1 as libc::c_int) << 8 as libc::c_int)
                                       - 1 as libc::c_int) as libc::c_uint) <<
                                 0 as libc::c_int) as u8_0,};
         init
     },
     {
         let mut init =
             ElfMessage{byte0:
                            ((0 as libc::c_int as u32_0 &
                                  (((0x1 as libc::c_int) << 3 as libc::c_int)
                                       - 1 as libc::c_int) as libc::c_uint) <<
                                 5 as libc::c_int |
                                 (0 as libc::c_int as u32_0 &
                                      (((0x1 as libc::c_int) <<
                                            4 as libc::c_int) -
                                           1 as libc::c_int) as libc::c_uint)
                                     << 1 as libc::c_int |
                                 (0 as libc::c_int as u32_0 &
                                      (((0x1 as libc::c_int) <<
                                            1 as libc::c_int) -
                                           1 as libc::c_int) as libc::c_uint)
                                     << 0 as libc::c_int) as u8_0,
                        byte1:
                            ((0x43 as libc::c_int as u32_0 &
                                  (((0x1 as libc::c_int) << 8 as libc::c_int)
                                       - 1 as libc::c_int) as libc::c_uint) <<
                                 0 as libc::c_int) as u8_0,
                        byte2:
                            ((0x4c as libc::c_int as u32_0 &
                                  (((0x1 as libc::c_int) << 8 as libc::c_int)
                                       - 1 as libc::c_int) as libc::c_uint) <<
                                 0 as libc::c_int) as u8_0,
                        byte3:
                            ((0 as libc::c_int as u32_0 &
                                  (((0x1 as libc::c_int) << 8 as libc::c_int)
                                       - 1 as libc::c_int) as libc::c_uint) <<
                                 0 as libc::c_int) as u8_0,};
         init
     },
     {
         let mut init =
             ElfMessage{byte0:
                            ((0 as libc::c_int as u32_0 &
                                  (((0x1 as libc::c_int) << 3 as libc::c_int)
                                       - 1 as libc::c_int) as libc::c_uint) <<
                                 5 as libc::c_int |
                                 (0 as libc::c_int as u32_0 &
                                      (((0x1 as libc::c_int) <<
                                            4 as libc::c_int) -
                                           1 as libc::c_int) as libc::c_uint)
                                     << 1 as libc::c_int |
                                 (0 as libc::c_int as u32_0 &
                                      (((0x1 as libc::c_int) <<
                                            1 as libc::c_int) -
                                           1 as libc::c_int) as libc::c_uint)
                                     << 0 as libc::c_int) as u8_0,
                        byte1:
                            ((0x45 as libc::c_int as u32_0 &
                                  (((0x1 as libc::c_int) << 8 as libc::c_int)
                                       - 1 as libc::c_int) as libc::c_uint) <<
                                 0 as libc::c_int) as u8_0,
                        byte2:
                            ((0x4d as libc::c_int as u32_0 &
                                  (((0x1 as libc::c_int) << 8 as libc::c_int)
                                       - 1 as libc::c_int) as libc::c_uint) <<
                                 0 as libc::c_int) as u8_0,
                        byte3:
                            ((0 as libc::c_int as u32_0 &
                                  (((0x1 as libc::c_int) << 8 as libc::c_int)
                                       - 1 as libc::c_int) as libc::c_uint) <<
                                 0 as libc::c_int) as u8_0,};
         init
     },
     {
         let mut init =
             ElfMessage{byte0:
                            ((0 as libc::c_int as u32_0 &
                                  (((0x1 as libc::c_int) << 3 as libc::c_int)
                                       - 1 as libc::c_int) as libc::c_uint) <<
                                 5 as libc::c_int |
                                 (2 as libc::c_int as u32_0 &
                                      (((0x1 as libc::c_int) <<
                                            4 as libc::c_int) -
                                           1 as libc::c_int) as libc::c_uint)
                                     << 1 as libc::c_int |
                                 (1 as libc::c_int as u32_0 &
                                      (((0x1 as libc::c_int) <<
                                            1 as libc::c_int) -
                                           1 as libc::c_int) as libc::c_uint)
                                     << 0 as libc::c_int) as u8_0,
                        byte1:
                            ((ITEM_HOOKSHOT as libc::c_int as u32_0 &
                                  (((0x1 as libc::c_int) << 8 as libc::c_int)
                                       - 1 as libc::c_int) as libc::c_uint) <<
                                 0 as libc::c_int) as u8_0,
                        byte2:
                            ((0x4e as libc::c_int as u32_0 &
                                  (((0x1 as libc::c_int) << 8 as libc::c_int)
                                       - 1 as libc::c_int) as libc::c_uint) <<
                                 0 as libc::c_int) as u8_0,
                        byte3:
                            ((ITEM_NONE as libc::c_int as u32_0 &
                                  (((0x1 as libc::c_int) << 8 as libc::c_int)
                                       - 1 as libc::c_int) as libc::c_uint) <<
                                 0 as libc::c_int) as u8_0,};
         init
     },
     {
         let mut init =
             ElfMessage{byte0:
                            ((0 as libc::c_int as u32_0 &
                                  (((0x1 as libc::c_int) << 3 as libc::c_int)
                                       - 1 as libc::c_int) as libc::c_uint) <<
                                 5 as libc::c_int |
                                 (3 as libc::c_int as u32_0 &
                                      (((0x1 as libc::c_int) <<
                                            4 as libc::c_int) -
                                           1 as libc::c_int) as libc::c_uint)
                                     << 1 as libc::c_int |
                                 (0 as libc::c_int as u32_0 &
                                      (((0x1 as libc::c_int) <<
                                            1 as libc::c_int) -
                                           1 as libc::c_int) as libc::c_uint)
                                     << 0 as libc::c_int) as u8_0,
                        byte1:
                            ((3 as libc::c_int as u32_0 &
                                  (((0x1 as libc::c_int) << 4 as libc::c_int)
                                       - 1 as libc::c_int) as libc::c_uint) <<
                                 4 as libc::c_int |
                                 (0 as libc::c_int as u32_0 &
                                      (((0x1 as libc::c_int) <<
                                            4 as libc::c_int) -
                                           1 as libc::c_int) as libc::c_uint)
                                     << 0 as libc::c_int) as u8_0,
                        byte2:
                            ((0x50 as libc::c_int as u32_0 &
                                  (((0x1 as libc::c_int) << 8 as libc::c_int)
                                       - 1 as libc::c_int) as libc::c_uint) <<
                                 0 as libc::c_int) as u8_0,
                        byte3:
                            ((ITEM_MEDALLION_FOREST as libc::c_int as u32_0 &
                                  (((0x1 as libc::c_int) << 8 as libc::c_int)
                                       - 1 as libc::c_int) as libc::c_uint) <<
                                 0 as libc::c_int) as u8_0,};
         init
     },
     {
         let mut init =
             ElfMessage{byte0:
                            ((0 as libc::c_int as u32_0 &
                                  (((0x1 as libc::c_int) << 3 as libc::c_int)
                                       - 1 as libc::c_int) as libc::c_uint) <<
                                 5 as libc::c_int |
                                 (3 as libc::c_int as u32_0 &
                                      (((0x1 as libc::c_int) <<
                                            4 as libc::c_int) -
                                           1 as libc::c_int) as libc::c_uint)
                                     << 1 as libc::c_int |
                                 (0 as libc::c_int as u32_0 &
                                      (((0x1 as libc::c_int) <<
                                            1 as libc::c_int) -
                                           1 as libc::c_int) as libc::c_uint)
                                     << 0 as libc::c_int) as u8_0,
                        byte1:
                            ((3 as libc::c_int as u32_0 &
                                  (((0x1 as libc::c_int) << 4 as libc::c_int)
                                       - 1 as libc::c_int) as libc::c_uint) <<
                                 4 as libc::c_int |
                                 (0 as libc::c_int as u32_0 &
                                      (((0x1 as libc::c_int) <<
                                            4 as libc::c_int) -
                                           1 as libc::c_int) as libc::c_uint)
                                     << 0 as libc::c_int) as u8_0,
                        byte2:
                            ((0x51 as libc::c_int as u32_0 &
                                  (((0x1 as libc::c_int) << 8 as libc::c_int)
                                       - 1 as libc::c_int) as libc::c_uint) <<
                                 0 as libc::c_int) as u8_0,
                        byte3:
                            ((ITEM_MEDALLION_FIRE as libc::c_int as u32_0 &
                                  (((0x1 as libc::c_int) << 8 as libc::c_int)
                                       - 1 as libc::c_int) as libc::c_uint) <<
                                 0 as libc::c_int) as u8_0,};
         init
     },
     {
         let mut init =
             ElfMessage{byte0:
                            ((0 as libc::c_int as u32_0 &
                                  (((0x1 as libc::c_int) << 3 as libc::c_int)
                                       - 1 as libc::c_int) as libc::c_uint) <<
                                 5 as libc::c_int |
                                 (3 as libc::c_int as u32_0 &
                                      (((0x1 as libc::c_int) <<
                                            4 as libc::c_int) -
                                           1 as libc::c_int) as libc::c_uint)
                                     << 1 as libc::c_int |
                                 (0 as libc::c_int as u32_0 &
                                      (((0x1 as libc::c_int) <<
                                            1 as libc::c_int) -
                                           1 as libc::c_int) as libc::c_uint)
                                     << 0 as libc::c_int) as u8_0,
                        byte1:
                            ((1 as libc::c_int as u32_0 &
                                  (((0x1 as libc::c_int) << 4 as libc::c_int)
                                       - 1 as libc::c_int) as libc::c_uint) <<
                                 4 as libc::c_int |
                                 (0 as libc::c_int as u32_0 &
                                      (((0x1 as libc::c_int) <<
                                            4 as libc::c_int) -
                                           1 as libc::c_int) as libc::c_uint)
                                     << 0 as libc::c_int) as u8_0,
                        byte2:
                            ((0x52 as libc::c_int as u32_0 &
                                  (((0x1 as libc::c_int) << 8 as libc::c_int)
                                       - 1 as libc::c_int) as libc::c_uint) <<
                                 0 as libc::c_int) as u8_0,
                        byte3:
                            ((ITEM_BOOTS_IRON as libc::c_int as u32_0 &
                                  (((0x1 as libc::c_int) << 8 as libc::c_int)
                                       - 1 as libc::c_int) as libc::c_uint) <<
                                 0 as libc::c_int) as u8_0,};
         init
     },
     {
         let mut init =
             ElfMessage{byte0:
                            ((0 as libc::c_int as u32_0 &
                                  (((0x1 as libc::c_int) << 3 as libc::c_int)
                                       - 1 as libc::c_int) as libc::c_uint) <<
                                 5 as libc::c_int |
                                 (3 as libc::c_int as u32_0 &
                                      (((0x1 as libc::c_int) <<
                                            4 as libc::c_int) -
                                           1 as libc::c_int) as libc::c_uint)
                                     << 1 as libc::c_int |
                                 (0 as libc::c_int as u32_0 &
                                      (((0x1 as libc::c_int) <<
                                            1 as libc::c_int) -
                                           1 as libc::c_int) as libc::c_uint)
                                     << 0 as libc::c_int) as u8_0,
                        byte1:
                            ((3 as libc::c_int as u32_0 &
                                  (((0x1 as libc::c_int) << 4 as libc::c_int)
                                       - 1 as libc::c_int) as libc::c_uint) <<
                                 4 as libc::c_int |
                                 (0 as libc::c_int as u32_0 &
                                      (((0x1 as libc::c_int) <<
                                            4 as libc::c_int) -
                                           1 as libc::c_int) as libc::c_uint)
                                     << 0 as libc::c_int) as u8_0,
                        byte2:
                            ((0x53 as libc::c_int as u32_0 &
                                  (((0x1 as libc::c_int) << 8 as libc::c_int)
                                       - 1 as libc::c_int) as libc::c_uint) <<
                                 0 as libc::c_int) as u8_0,
                        byte3:
                            ((ITEM_MEDALLION_WATER as libc::c_int as u32_0 &
                                  (((0x1 as libc::c_int) << 8 as libc::c_int)
                                       - 1 as libc::c_int) as libc::c_uint) <<
                                 0 as libc::c_int) as u8_0,};
         init
     },
     {
         let mut init =
             ElfMessage{byte0:
                            ((0 as libc::c_int as u32_0 &
                                  (((0x1 as libc::c_int) << 3 as libc::c_int)
                                       - 1 as libc::c_int) as libc::c_uint) <<
                                 5 as libc::c_int |
                                 (0 as libc::c_int as u32_0 &
                                      (((0x1 as libc::c_int) <<
                                            4 as libc::c_int) -
                                           1 as libc::c_int) as libc::c_uint)
                                     << 1 as libc::c_int |
                                 (0 as libc::c_int as u32_0 &
                                      (((0x1 as libc::c_int) <<
                                            1 as libc::c_int) -
                                           1 as libc::c_int) as libc::c_uint)
                                     << 0 as libc::c_int) as u8_0,
                        byte1:
                            ((0xaa as libc::c_int as u32_0 &
                                  (((0x1 as libc::c_int) << 8 as libc::c_int)
                                       - 1 as libc::c_int) as libc::c_uint) <<
                                 0 as libc::c_int) as u8_0,
                        byte2:
                            ((0x54 as libc::c_int as u32_0 &
                                  (((0x1 as libc::c_int) << 8 as libc::c_int)
                                       - 1 as libc::c_int) as libc::c_uint) <<
                                 0 as libc::c_int) as u8_0,
                        byte3:
                            ((0 as libc::c_int as u32_0 &
                                  (((0x1 as libc::c_int) << 8 as libc::c_int)
                                       - 1 as libc::c_int) as libc::c_uint) <<
                                 0 as libc::c_int) as u8_0,};
         init
     },
     {
         let mut init =
             ElfMessage{byte0:
                            ((0 as libc::c_int as u32_0 &
                                  (((0x1 as libc::c_int) << 3 as libc::c_int)
                                       - 1 as libc::c_int) as libc::c_uint) <<
                                 5 as libc::c_int |
                                 (2 as libc::c_int as u32_0 &
                                      (((0x1 as libc::c_int) <<
                                            4 as libc::c_int) -
                                           1 as libc::c_int) as libc::c_uint)
                                     << 1 as libc::c_int |
                                 (1 as libc::c_int as u32_0 &
                                      (((0x1 as libc::c_int) <<
                                            1 as libc::c_int) -
                                           1 as libc::c_int) as libc::c_uint)
                                     << 0 as libc::c_int) as u8_0,
                        byte1:
                            ((ITEM_LENS as libc::c_int as u32_0 &
                                  (((0x1 as libc::c_int) << 8 as libc::c_int)
                                       - 1 as libc::c_int) as libc::c_uint) <<
                                 0 as libc::c_int) as u8_0,
                        byte2:
                            ((0x55 as libc::c_int as u32_0 &
                                  (((0x1 as libc::c_int) << 8 as libc::c_int)
                                       - 1 as libc::c_int) as libc::c_uint) <<
                                 0 as libc::c_int) as u8_0,
                        byte3:
                            ((ITEM_NONE as libc::c_int as u32_0 &
                                  (((0x1 as libc::c_int) << 8 as libc::c_int)
                                       - 1 as libc::c_int) as libc::c_uint) <<
                                 0 as libc::c_int) as u8_0,};
         init
     },
     {
         let mut init =
             ElfMessage{byte0:
                            ((0 as libc::c_int as u32_0 &
                                  (((0x1 as libc::c_int) << 3 as libc::c_int)
                                       - 1 as libc::c_int) as libc::c_uint) <<
                                 5 as libc::c_int |
                                 (3 as libc::c_int as u32_0 &
                                      (((0x1 as libc::c_int) <<
                                            4 as libc::c_int) -
                                           1 as libc::c_int) as libc::c_uint)
                                     << 1 as libc::c_int |
                                 (0 as libc::c_int as u32_0 &
                                      (((0x1 as libc::c_int) <<
                                            1 as libc::c_int) -
                                           1 as libc::c_int) as libc::c_uint)
                                     << 0 as libc::c_int) as u8_0,
                        byte1:
                            ((3 as libc::c_int as u32_0 &
                                  (((0x1 as libc::c_int) << 4 as libc::c_int)
                                       - 1 as libc::c_int) as libc::c_uint) <<
                                 4 as libc::c_int |
                                 (0 as libc::c_int as u32_0 &
                                      (((0x1 as libc::c_int) <<
                                            4 as libc::c_int) -
                                           1 as libc::c_int) as libc::c_uint)
                                     << 0 as libc::c_int) as u8_0,
                        byte2:
                            ((0x57 as libc::c_int as u32_0 &
                                  (((0x1 as libc::c_int) << 8 as libc::c_int)
                                       - 1 as libc::c_int) as libc::c_uint) <<
                                 0 as libc::c_int) as u8_0,
                        byte3:
                            ((ITEM_MEDALLION_SHADOW as libc::c_int as u32_0 &
                                  (((0x1 as libc::c_int) << 8 as libc::c_int)
                                       - 1 as libc::c_int) as libc::c_uint) <<
                                 0 as libc::c_int) as u8_0,};
         init
     },
     {
         let mut init =
             ElfMessage{byte0:
                            ((0 as libc::c_int as u32_0 &
                                  (((0x1 as libc::c_int) << 3 as libc::c_int)
                                       - 1 as libc::c_int) as libc::c_uint) <<
                                 5 as libc::c_int |
                                 (3 as libc::c_int as u32_0 &
                                      (((0x1 as libc::c_int) <<
                                            4 as libc::c_int) -
                                           1 as libc::c_int) as libc::c_uint)
                                     << 1 as libc::c_int |
                                 (0 as libc::c_int as u32_0 &
                                      (((0x1 as libc::c_int) <<
                                            1 as libc::c_int) -
                                           1 as libc::c_int) as libc::c_uint)
                                     << 0 as libc::c_int) as u8_0,
                        byte1:
                            ((2 as libc::c_int as u32_0 &
                                  (((0x1 as libc::c_int) << 4 as libc::c_int)
                                       - 1 as libc::c_int) as libc::c_uint) <<
                                 4 as libc::c_int |
                                 (0 as libc::c_int as u32_0 &
                                      (((0x1 as libc::c_int) <<
                                            4 as libc::c_int) -
                                           1 as libc::c_int) as libc::c_uint)
                                     << 0 as libc::c_int) as u8_0,
                        byte2:
                            ((0x58 as libc::c_int as u32_0 &
                                  (((0x1 as libc::c_int) << 8 as libc::c_int)
                                       - 1 as libc::c_int) as libc::c_uint) <<
                                 0 as libc::c_int) as u8_0,
                        byte3:
                            ((ITEM_SONG_REQUIEM as libc::c_int as u32_0 &
                                  (((0x1 as libc::c_int) << 8 as libc::c_int)
                                       - 1 as libc::c_int) as libc::c_uint) <<
                                 0 as libc::c_int) as u8_0,};
         init
     },
     {
         let mut init =
             ElfMessage{byte0:
                            ((0 as libc::c_int as u32_0 &
                                  (((0x1 as libc::c_int) << 3 as libc::c_int)
                                       - 1 as libc::c_int) as libc::c_uint) <<
                                 5 as libc::c_int |
                                 (3 as libc::c_int as u32_0 &
                                      (((0x1 as libc::c_int) <<
                                            4 as libc::c_int) -
                                           1 as libc::c_int) as libc::c_uint)
                                     << 1 as libc::c_int |
                                 (1 as libc::c_int as u32_0 &
                                      (((0x1 as libc::c_int) <<
                                            1 as libc::c_int) -
                                           1 as libc::c_int) as libc::c_uint)
                                     << 0 as libc::c_int) as u8_0,
                        byte1:
                            ((0 as libc::c_int as u32_0 &
                                  (((0x1 as libc::c_int) << 4 as libc::c_int)
                                       - 1 as libc::c_int) as libc::c_uint) <<
                                 4 as libc::c_int |
                                 (1 as libc::c_int as u32_0 &
                                      (((0x1 as libc::c_int) <<
                                            4 as libc::c_int) -
                                           1 as libc::c_int) as libc::c_uint)
                                     << 0 as libc::c_int) as u8_0,
                        byte2:
                            ((0x56 as libc::c_int as u32_0 &
                                  (((0x1 as libc::c_int) << 8 as libc::c_int)
                                       - 1 as libc::c_int) as libc::c_uint) <<
                                 0 as libc::c_int) as u8_0,
                        byte3:
                            ((0 as libc::c_int as u32_0 &
                                  (((0x1 as libc::c_int) << 8 as libc::c_int)
                                       - 1 as libc::c_int) as libc::c_uint) <<
                                 0 as libc::c_int) as u8_0,};
         init
     },
     {
         let mut init =
             ElfMessage{byte0:
                            ((0 as libc::c_int as u32_0 &
                                  (((0x1 as libc::c_int) << 3 as libc::c_int)
                                       - 1 as libc::c_int) as libc::c_uint) <<
                                 5 as libc::c_int |
                                 (3 as libc::c_int as u32_0 &
                                      (((0x1 as libc::c_int) <<
                                            4 as libc::c_int) -
                                           1 as libc::c_int) as libc::c_uint)
                                     << 1 as libc::c_int |
                                 (0 as libc::c_int as u32_0 &
                                      (((0x1 as libc::c_int) <<
                                            1 as libc::c_int) -
                                           1 as libc::c_int) as libc::c_uint)
                                     << 0 as libc::c_int) as u8_0,
                        byte1:
                            ((3 as libc::c_int as u32_0 &
                                  (((0x1 as libc::c_int) << 4 as libc::c_int)
                                       - 1 as libc::c_int) as libc::c_uint) <<
                                 4 as libc::c_int |
                                 (0 as libc::c_int as u32_0 &
                                      (((0x1 as libc::c_int) <<
                                            4 as libc::c_int) -
                                           1 as libc::c_int) as libc::c_uint)
                                     << 0 as libc::c_int) as u8_0,
                        byte2:
                            ((0x5a as libc::c_int as u32_0 &
                                  (((0x1 as libc::c_int) << 8 as libc::c_int)
                                       - 1 as libc::c_int) as libc::c_uint) <<
                                 0 as libc::c_int) as u8_0,
                        byte3:
                            ((ITEM_MEDALLION_SPIRIT as libc::c_int as u32_0 &
                                  (((0x1 as libc::c_int) << 8 as libc::c_int)
                                       - 1 as libc::c_int) as libc::c_uint) <<
                                 0 as libc::c_int) as u8_0,};
         init
     },
     {
         let mut init =
             ElfMessage{byte0:
                            ((0 as libc::c_int as u32_0 &
                                  (((0x1 as libc::c_int) << 3 as libc::c_int)
                                       - 1 as libc::c_int) as libc::c_uint) <<
                                 5 as libc::c_int |
                                 (2 as libc::c_int as u32_0 &
                                      (((0x1 as libc::c_int) <<
                                            4 as libc::c_int) -
                                           1 as libc::c_int) as libc::c_uint)
                                     << 1 as libc::c_int |
                                 (1 as libc::c_int as u32_0 &
                                      (((0x1 as libc::c_int) <<
                                            1 as libc::c_int) -
                                           1 as libc::c_int) as libc::c_uint)
                                     << 0 as libc::c_int) as u8_0,
                        byte1:
                            ((ITEM_ARROW_LIGHT as libc::c_int as u32_0 &
                                  (((0x1 as libc::c_int) << 8 as libc::c_int)
                                       - 1 as libc::c_int) as libc::c_uint) <<
                                 0 as libc::c_int) as u8_0,
                        byte2:
                            ((0x5b as libc::c_int as u32_0 &
                                  (((0x1 as libc::c_int) << 8 as libc::c_int)
                                       - 1 as libc::c_int) as libc::c_uint) <<
                                 0 as libc::c_int) as u8_0,
                        byte3:
                            ((ITEM_NONE as libc::c_int as u32_0 &
                                  (((0x1 as libc::c_int) << 8 as libc::c_int)
                                       - 1 as libc::c_int) as libc::c_uint) <<
                                 0 as libc::c_int) as u8_0,};
         init
     },
     {
         let mut init =
             ElfMessage{byte0:
                            ((0 as libc::c_int as u32_0 &
                                  (((0x1 as libc::c_int) << 3 as libc::c_int)
                                       - 1 as libc::c_int) as libc::c_uint) <<
                                 5 as libc::c_int |
                                 (0 as libc::c_int as u32_0 &
                                      (((0x1 as libc::c_int) <<
                                            4 as libc::c_int) -
                                           1 as libc::c_int) as libc::c_uint)
                                     << 1 as libc::c_int |
                                 (0 as libc::c_int as u32_0 &
                                      (((0x1 as libc::c_int) <<
                                            1 as libc::c_int) -
                                           1 as libc::c_int) as libc::c_uint)
                                     << 0 as libc::c_int) as u8_0,
                        byte1:
                            ((0xc3 as libc::c_int as u32_0 &
                                  (((0x1 as libc::c_int) << 8 as libc::c_int)
                                       - 1 as libc::c_int) as libc::c_uint) <<
                                 0 as libc::c_int) as u8_0,
                        byte2:
                            ((0x5c as libc::c_int as u32_0 &
                                  (((0x1 as libc::c_int) << 8 as libc::c_int)
                                       - 1 as libc::c_int) as libc::c_uint) <<
                                 0 as libc::c_int) as u8_0,
                        byte3:
                            ((0 as libc::c_int as u32_0 &
                                  (((0x1 as libc::c_int) << 8 as libc::c_int)
                                       - 1 as libc::c_int) as libc::c_uint) <<
                                 0 as libc::c_int) as u8_0,};
         init
     },
     {
         let mut init =
             ElfMessage{byte0:
                            ((7 as libc::c_int as u32_0 &
                                  (((0x1 as libc::c_int) << 3 as libc::c_int)
                                       - 1 as libc::c_int) as libc::c_uint) <<
                                 5 as libc::c_int |
                                 (0 as libc::c_int as u32_0 &
                                      (((0x1 as libc::c_int) <<
                                            4 as libc::c_int) -
                                           1 as libc::c_int) as libc::c_uint)
                                     << 1 as libc::c_int |
                                 (0 as libc::c_int as u32_0 &
                                      (((0x1 as libc::c_int) <<
                                            1 as libc::c_int) -
                                           1 as libc::c_int) as libc::c_uint)
                                     << 0 as libc::c_int) as u8_0,
                        byte1:
                            ((0 as libc::c_int as u32_0 &
                                  (((0x1 as libc::c_int) << 8 as libc::c_int)
                                       - 1 as libc::c_int) as libc::c_uint) <<
                                 0 as libc::c_int) as u8_0,
                        byte2:
                            ((0x5f as libc::c_int as u32_0 &
                                  (((0x1 as libc::c_int) << 8 as libc::c_int)
                                       - 1 as libc::c_int) as libc::c_uint) <<
                                 0 as libc::c_int) as u8_0,
                        byte3:
                            ((0 as libc::c_int as u32_0 &
                                  (((0x1 as libc::c_int) << 8 as libc::c_int)
                                       - 1 as libc::c_int) as libc::c_uint) <<
                                 0 as libc::c_int) as u8_0,};
         init
     }];
