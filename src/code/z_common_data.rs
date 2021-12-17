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
pub type s16 = libc::c_short;
pub type u16_0 = libc::c_ushort;
pub type s32 = libc::c_int;
pub type u32_0 = libc::c_uint;
pub type f32_0 = libc::c_float;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Vec3f {
    pub x: f32_0,
    pub y: f32_0,
    pub z: f32_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Vec3s {
    pub x: s16,
    pub y: s16,
    pub z: s16,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Vec3i {
    pub x: s32,
    pub y: s32,
    pub z: s32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ItemEquips {
    pub buttonItems: [u8_0; 4],
    pub cButtonSlots: [u8_0; 3],
    pub equipment: u16_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Inventory {
    pub items: [u8_0; 24],
    pub ammo: [s8; 16],
    pub equipment: u16_0,
    pub upgrades: u32_0,
    pub questItems: u32_0,
    pub dungeonItems: [u8_0; 20],
    pub dungeonKeys: [s8; 19],
    pub defenseHearts: s8,
    pub gsTokens: s16,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SavedSceneFlags {
    pub chest: u32_0,
    pub swch: u32_0,
    pub clear: u32_0,
    pub collect: u32_0,
    pub unk: u32_0,
    pub rooms: u32_0,
    pub floors: u32_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct HorseData {
    pub scene: s16,
    pub pos: Vec3s,
    pub angle: s16,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct RespawnData {
    pub pos: Vec3f,
    pub yaw: s16,
    pub playerParams: s16,
    pub entranceIndex: s16,
    pub roomIndex: u8_0,
    pub data: s8,
    pub tempSwchFlags: u32_0,
    pub tempCollectFlags: u32_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct FaroresWindData {
    pub pos: Vec3i,
    pub yaw: s32,
    pub playerParams: s32,
    pub entranceIndex: s32,
    pub roomIndex: s32,
    pub set: s32,
    pub tempSwchFlags: s32,
    pub tempCollectFlags: s32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SaveContext {
    pub entranceIndex: s32,
    pub linkAge: s32,
    pub cutsceneIndex: s32,
    pub dayTime: u16_0,
    pub nightFlag: s32,
    pub totalDays: s32,
    pub bgsDayCount: s32,
    pub newf: [libc::c_char; 6],
    pub deaths: u16_0,
    pub playerName: [libc::c_char; 8],
    pub n64ddFlag: s16,
    pub healthCapacity: s16,
    pub health: s16,
    pub magicLevel: s8,
    pub magic: s8,
    pub rupees: s16,
    pub swordHealth: u16_0,
    pub naviTimer: u16_0,
    pub magicAcquired: u8_0,
    pub unk_3B: [libc::c_char; 1],
    pub doubleMagic: u8_0,
    pub doubleDefense: u8_0,
    pub bgsFlag: u8_0,
    pub ocarinaGameRoundNum: u8_0,
    pub childEquips: ItemEquips,
    pub adultEquips: ItemEquips,
    pub unk_54: u32_0,
    pub unk_58: [libc::c_char; 14],
    pub savedSceneNum: s16,
    pub equips: ItemEquips,
    pub inventory: Inventory,
    pub sceneFlags: [SavedSceneFlags; 124],
    pub fw: FaroresWindData,
    pub unk_E8C: [libc::c_char; 16],
    pub gsFlags: [s32; 6],
    pub unk_EB4: [libc::c_char; 4],
    pub highScores: [s32; 7],
    pub eventChkInf: [u16_0; 14],
    pub itemGetInf: [u16_0; 4],
    pub infTable: [u16_0; 30],
    pub unk_F34: [libc::c_char; 4],
    pub worldMapAreaData: u32_0,
    pub unk_F3C: [libc::c_char; 4],
    pub scarecrowCustomSongSet: u8_0,
    pub scarecrowCustomSong: [u8_0; 864],
    pub unk_12A1: [libc::c_char; 36],
    pub scarecrowSpawnSongSet: u8_0,
    pub scarecrowSpawnSong: [u8_0; 128],
    pub unk_1346: [libc::c_char; 2],
    pub horseData: HorseData,
    pub checksum: u16_0,
    pub fileNum: s32,
    pub unk_1358: [libc::c_char; 4],
    pub gameMode: s32,
    pub sceneSetupIndex: s32,
    pub respawnFlag: s32,
    pub respawn: [RespawnData; 3],
    pub entranceSpeed: f32_0,
    pub entranceSound: u16_0,
    pub unk_13C2: [libc::c_char; 1],
    pub unk_13C3: u8_0,
    pub dogParams: s16,
    pub textTriggerFlags: u8_0,
    pub showTitleCard: u8_0,
    pub nayrusLoveTimer: s16,
    pub unk_13CA: [libc::c_char; 2],
    pub rupeeAccumulator: s16,
    pub timer1State: s16,
    pub timer1Value: s16,
    pub timer2State: s16,
    pub timer2Value: s16,
    pub timerX: [s16; 2],
    pub timerY: [s16; 2],
    pub unk_13DE: [libc::c_char; 2],
    pub seqId: u8_0,
    pub natureAmbienceId: u8_0,
    pub buttonStatus: [u8_0; 5],
    pub unk_13E7: u8_0,
    pub unk_13E8: u16_0,
    pub unk_13EA: u16_0,
    pub unk_13EC: u16_0,
    pub unk_13EE: u16_0,
    pub unk_13F0: s16,
    pub unk_13F2: s16,
    pub unk_13F4: s16,
    pub unk_13F6: s16,
    pub unk_13F8: s16,
    pub eventInf: [u16_0; 4],
    pub mapIndex: u16_0,
    pub minigameState: u16_0,
    pub minigameScore: u16_0,
    pub unk_1408: [libc::c_char; 1],
    pub language: u8_0,
    pub audioSetting: u8_0,
    pub unk_140B: [libc::c_char; 1],
    pub zTargetSetting: u8_0,
    pub forcedSeqId: u16_0,
    pub unk_1410: u8_0,
    pub unk_1411: [libc::c_char; 1],
    pub nextCutsceneIndex: u16_0,
    pub cutsceneTrigger: u8_0,
    pub chamberCutsceneNum: u8_0,
    pub nextDayTime: u16_0,
    pub fadeDuration: u8_0,
    pub unk_1419: u8_0,
    pub skyboxTime: u16_0,
    pub dogIsLost: u8_0,
    pub nextTransition: u8_0,
    pub unk_141E: [libc::c_char; 2],
    pub worldMapArea: s16,
    pub sunsSongState: s16,
    pub healthAccumulator: s16,
}
#[no_mangle]
pub static mut gSaveContext: SaveContext =
    SaveContext{entranceIndex: 0,
                linkAge: 0,
                cutsceneIndex: 0,
                dayTime: 0,
                nightFlag: 0,
                totalDays: 0,
                bgsDayCount: 0,
                newf: [0; 6],
                deaths: 0,
                playerName: [0; 8],
                n64ddFlag: 0,
                healthCapacity: 0,
                health: 0,
                magicLevel: 0,
                magic: 0,
                rupees: 0,
                swordHealth: 0,
                naviTimer: 0,
                magicAcquired: 0,
                unk_3B: [0; 1],
                doubleMagic: 0,
                doubleDefense: 0,
                bgsFlag: 0,
                ocarinaGameRoundNum: 0,
                childEquips:
                    ItemEquips{buttonItems: [0; 4],
                               cButtonSlots: [0; 3],
                               equipment: 0,},
                adultEquips:
                    ItemEquips{buttonItems: [0; 4],
                               cButtonSlots: [0; 3],
                               equipment: 0,},
                unk_54: 0,
                unk_58: [0; 14],
                savedSceneNum: 0,
                equips:
                    ItemEquips{buttonItems: [0; 4],
                               cButtonSlots: [0; 3],
                               equipment: 0,},
                inventory:
                    Inventory{items: [0; 24],
                              ammo: [0; 16],
                              equipment: 0,
                              upgrades: 0,
                              questItems: 0,
                              dungeonItems: [0; 20],
                              dungeonKeys: [0; 19],
                              defenseHearts: 0,
                              gsTokens: 0,},
                sceneFlags:
                    [SavedSceneFlags{chest: 0,
                                     swch: 0,
                                     clear: 0,
                                     collect: 0,
                                     unk: 0,
                                     rooms: 0,
                                     floors: 0,}; 124],
                fw:
                    FaroresWindData{pos: Vec3i{x: 0, y: 0, z: 0,},
                                    yaw: 0,
                                    playerParams: 0,
                                    entranceIndex: 0,
                                    roomIndex: 0,
                                    set: 0,
                                    tempSwchFlags: 0,
                                    tempCollectFlags: 0,},
                unk_E8C: [0; 16],
                gsFlags: [0; 6],
                unk_EB4: [0; 4],
                highScores: [0; 7],
                eventChkInf: [0; 14],
                itemGetInf: [0; 4],
                infTable: [0; 30],
                unk_F34: [0; 4],
                worldMapAreaData: 0,
                unk_F3C: [0; 4],
                scarecrowCustomSongSet: 0,
                scarecrowCustomSong: [0; 864],
                unk_12A1: [0; 36],
                scarecrowSpawnSongSet: 0,
                scarecrowSpawnSong: [0; 128],
                unk_1346: [0; 2],
                horseData:
                    HorseData{scene: 0,
                              pos: Vec3s{x: 0, y: 0, z: 0,},
                              angle: 0,},
                checksum: 0,
                fileNum: 0,
                unk_1358: [0; 4],
                gameMode: 0,
                sceneSetupIndex: 0,
                respawnFlag: 0,
                respawn:
                    [RespawnData{pos: Vec3f{x: 0., y: 0., z: 0.,},
                                 yaw: 0,
                                 playerParams: 0,
                                 entranceIndex: 0,
                                 roomIndex: 0,
                                 data: 0,
                                 tempSwchFlags: 0,
                                 tempCollectFlags: 0,}; 3],
                entranceSpeed: 0.,
                entranceSound: 0,
                unk_13C2: [0; 1],
                unk_13C3: 0,
                dogParams: 0,
                textTriggerFlags: 0,
                showTitleCard: 0,
                nayrusLoveTimer: 0,
                unk_13CA: [0; 2],
                rupeeAccumulator: 0,
                timer1State: 0,
                timer1Value: 0,
                timer2State: 0,
                timer2Value: 0,
                timerX: [0; 2],
                timerY: [0; 2],
                unk_13DE: [0; 2],
                seqId: 0,
                natureAmbienceId: 0,
                buttonStatus: [0; 5],
                unk_13E7: 0,
                unk_13E8: 0,
                unk_13EA: 0,
                unk_13EC: 0,
                unk_13EE: 0,
                unk_13F0: 0,
                unk_13F2: 0,
                unk_13F4: 0,
                unk_13F6: 0,
                unk_13F8: 0,
                eventInf: [0; 4],
                mapIndex: 0,
                minigameState: 0,
                minigameScore: 0,
                unk_1408: [0; 1],
                language: 0,
                audioSetting: 0,
                unk_140B: [0; 1],
                zTargetSetting: 0,
                forcedSeqId: 0,
                unk_1410: 0,
                unk_1411: [0; 1],
                nextCutsceneIndex: 0,
                cutsceneTrigger: 0,
                chamberCutsceneNum: 0,
                nextDayTime: 0,
                fadeDuration: 0,
                unk_1419: 0,
                skyboxTime: 0,
                dogIsLost: 0,
                nextTransition: 0,
                unk_141E: [0; 2],
                worldMapArea: 0,
                sunsSongState: 0,
                healthAccumulator: 0,};
#[no_mangle]
pub static mut D_8015FA88: u32_0 = 0;
#[no_mangle]
pub static mut D_8015FA8C: u32_0 = 0;
#[no_mangle]
pub unsafe extern "C" fn SaveContext_Init() {
    bzero(&mut gSaveContext as *mut SaveContext as *mut libc::c_void,
          ::std::mem::size_of::<SaveContext>() as libc::c_ulong);
    D_8015FA88 = 0 as libc::c_int as u32_0;
    D_8015FA8C = 0 as libc::c_int as u32_0;
    gSaveContext.seqId = 0xffff as libc::c_int as u8_0;
    gSaveContext.natureAmbienceId = 0xff as libc::c_int as u8_0;
    gSaveContext.forcedSeqId = 0 as libc::c_int as u16_0;
    gSaveContext.nextCutsceneIndex = 0xffef as libc::c_int as u16_0;
    gSaveContext.cutsceneTrigger = 0 as libc::c_int as u8_0;
    gSaveContext.chamberCutsceneNum = 0 as libc::c_int as u8_0;
    gSaveContext.nextDayTime = 0xffff as libc::c_int as u16_0;
    gSaveContext.skyboxTime = 0 as libc::c_int as u16_0;
    gSaveContext.dogIsLost = 1 as libc::c_int as u8_0;
    gSaveContext.nextTransition = 0xff as libc::c_int as u8_0;
    gSaveContext.unk_13EE = 50 as libc::c_int as u16_0;
}
