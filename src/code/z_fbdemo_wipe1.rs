#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case,
         non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(register_tool)]
extern "C" {
    #[no_mangle]
    fn bzero(__s: *mut libc::c_void, __n: u32_0);
    #[no_mangle]
    fn Gfx_BranchTexScroll(gfxp: *mut *mut Gfx, x: u32_0, y: u32_0,
                           width: s32, height: s32) -> *mut Gfx;
    #[no_mangle]
    fn guLookAt(_: *mut Mtx, xEye: f32_0, yEye: f32_0, zEye: f32_0,
                xAt: f32_0, yAt: f32_0, zAt: f32_0, xUp: f32_0, yUp: f32_0,
                zUp: f32_0);
    #[no_mangle]
    fn guPerspective(m: *mut Mtx, perspNorm: *mut u16_0, fovy: f32_0,
                     aspect: f32_0, near: f32_0, far: f32_0, scale: f32_0);
    #[no_mangle]
    static mut gSaveContext: SaveContext;
    #[no_mangle]
    fn guTranslate(m: *mut Mtx, x: f32_0, y: f32_0, z: f32_0);
    #[no_mangle]
    fn guRotate(_: *mut Mtx, angle: f32_0, x: f32_0, y: f32_0, z: f32_0);
    #[no_mangle]
    fn guScale(m: *mut Mtx, x: f32_0, y: f32_0, z: f32_0);
}
pub type s8 = libc::c_schar;
pub type u8_0 = libc::c_uchar;
pub type s16 = libc::c_short;
pub type u16_0 = libc::c_ushort;
pub type s32 = libc::c_int;
pub type u32_0 = libc::c_uint;
pub type u64_0 = libc::c_ulonglong;
pub type f32_0 = libc::c_float;
pub type Mtx_t = [[libc::c_long; 4]; 4];
#[derive(Copy, Clone)]
#[repr(C)]
pub union Mtx {
    pub m: Mtx_t,
    pub c2rust_unnamed: C2RustUnnamed,
    pub forc_structure_alignment: libc::c_longlong,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed {
    pub intPart: [[u16_0; 4]; 4],
    pub fracPart: [[u16_0; 4]; 4],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Vtx_t {
    pub ob: [libc::c_short; 3],
    pub flag: libc::c_ushort,
    pub tc: [libc::c_short; 2],
    pub cn: [libc::c_uchar; 4],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Vtx_tn {
    pub ob: [libc::c_short; 3],
    pub flag: libc::c_ushort,
    pub tc: [libc::c_short; 2],
    pub n: [libc::c_schar; 3],
    pub a: libc::c_uchar,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union Vtx {
    pub v: Vtx_t,
    pub n: Vtx_tn,
    pub force_structure_alignment: libc::c_longlong,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Tri {
    pub flag: libc::c_uchar,
    pub v: [libc::c_uchar; 3],
}
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct Gdma {
    #[bitfield(name = "cmd", ty = "libc::c_int", bits = "0..=7")]
    #[bitfield(name = "par", ty = "libc::c_uint", bits = "8..=15")]
    #[bitfield(name = "len", ty = "libc::c_uint", bits = "16..=31")]
    pub cmd_par_len: [u8; 4],
    pub addr: libc::c_uint,
}
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct Gtri {
    #[bitfield(name = "cmd", ty = "libc::c_int", bits = "0..=7")]
    #[bitfield(name = "pad", ty = "libc::c_int", bits = "8..=31")]
    pub cmd_pad: [u8; 4],
    pub tri: Tri,
}
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct Gpopmtx {
    #[bitfield(name = "cmd", ty = "libc::c_int", bits = "0..=7")]
    #[bitfield(name = "pad1", ty = "libc::c_int", bits = "8..=31")]
    #[bitfield(name = "pad2", ty = "libc::c_int", bits = "32..=55")]
    #[bitfield(name = "param", ty = "libc::c_uchar", bits = "56..=63")]
    pub cmd_pad1_pad2_param: [u8; 8],
}
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct Gsegment {
    #[bitfield(name = "cmd", ty = "libc::c_int", bits = "0..=7")]
    #[bitfield(name = "pad0", ty = "libc::c_int", bits = "8..=15")]
    #[bitfield(name = "mw_index", ty = "libc::c_int", bits = "16..=23")]
    #[bitfield(name = "number", ty = "libc::c_int", bits = "24..=31")]
    #[bitfield(name = "pad1", ty = "libc::c_int", bits = "32..=39")]
    #[bitfield(name = "base", ty = "libc::c_int", bits = "40..=63")]
    pub cmd_pad0_mw_index_number_pad1_base: [u8; 8],
}
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct GsetothermodeL {
    #[bitfield(name = "cmd", ty = "libc::c_int", bits = "0..=7")]
    #[bitfield(name = "pad0", ty = "libc::c_int", bits = "8..=15")]
    #[bitfield(name = "sft", ty = "libc::c_int", bits = "16..=23")]
    #[bitfield(name = "len", ty = "libc::c_int", bits = "24..=31")]
    #[bitfield(name = "data", ty = "libc::c_uint", bits = "32..=63")]
    pub cmd_pad0_sft_len_data: [u8; 8],
}
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct GsetothermodeH {
    #[bitfield(name = "cmd", ty = "libc::c_int", bits = "0..=7")]
    #[bitfield(name = "pad0", ty = "libc::c_int", bits = "8..=15")]
    #[bitfield(name = "sft", ty = "libc::c_int", bits = "16..=23")]
    #[bitfield(name = "len", ty = "libc::c_int", bits = "24..=31")]
    #[bitfield(name = "data", ty = "libc::c_uint", bits = "32..=63")]
    pub cmd_pad0_sft_len_data: [u8; 8],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Gtexture {
    pub cmd: libc::c_uchar,
    pub lodscale: libc::c_uchar,
    pub tile: libc::c_uchar,
    pub on: libc::c_uchar,
    pub s: libc::c_ushort,
    pub t: libc::c_ushort,
}
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct Gline3D {
    #[bitfield(name = "cmd", ty = "libc::c_int", bits = "0..=7")]
    #[bitfield(name = "pad", ty = "libc::c_int", bits = "8..=31")]
    pub cmd_pad: [u8; 4],
    pub line: Tri,
}
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct Gperspnorm {
    #[bitfield(name = "cmd", ty = "libc::c_int", bits = "0..=7")]
    #[bitfield(name = "pad1", ty = "libc::c_int", bits = "8..=31")]
    pub cmd_pad1: [u8; 4],
    pub pad2: libc::c_short,
    pub scale: libc::c_short,
}
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct Gsetimg {
    #[bitfield(name = "cmd", ty = "libc::c_int", bits = "0..=7")]
    #[bitfield(name = "fmt", ty = "libc::c_uint", bits = "8..=10")]
    #[bitfield(name = "siz", ty = "libc::c_uint", bits = "11..=12")]
    #[bitfield(name = "pad", ty = "libc::c_uint", bits = "13..=19")]
    #[bitfield(name = "wd", ty = "libc::c_uint", bits = "20..=31")]
    pub cmd_fmt_siz_pad_wd: [u8; 4],
    pub dram: libc::c_uint,
}
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct Gsetcombine {
    #[bitfield(name = "cmd", ty = "libc::c_int", bits = "0..=7")]
    #[bitfield(name = "muxs0", ty = "libc::c_uint", bits = "8..=31")]
    #[bitfield(name = "muxs1", ty = "libc::c_uint", bits = "32..=63")]
    pub cmd_muxs0_muxs1: [u8; 8],
}
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct Gsetcolor {
    #[bitfield(name = "cmd", ty = "libc::c_int", bits = "0..=7")]
    pub cmd: [u8; 1],
    pub pad: libc::c_uchar,
    pub prim_min_level: libc::c_uchar,
    pub prim_level: libc::c_uchar,
    pub color: libc::c_ulong,
}
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct Gfillrect {
    #[bitfield(name = "cmd", ty = "libc::c_int", bits = "0..=7")]
    #[bitfield(name = "x0", ty = "libc::c_int", bits = "8..=17")]
    #[bitfield(name = "x0frac", ty = "libc::c_int", bits = "18..=19")]
    #[bitfield(name = "y0", ty = "libc::c_int", bits = "20..=29")]
    #[bitfield(name = "y0frac", ty = "libc::c_int", bits = "30..=31")]
    #[bitfield(name = "pad", ty = "libc::c_uint", bits = "32..=39")]
    #[bitfield(name = "x1", ty = "libc::c_int", bits = "40..=49")]
    #[bitfield(name = "x1frac", ty = "libc::c_int", bits = "50..=51")]
    #[bitfield(name = "y1", ty = "libc::c_int", bits = "52..=61")]
    #[bitfield(name = "y1frac", ty = "libc::c_int", bits = "62..=63")]
    pub cmd_x0_x0frac_y0_y0frac_pad_x1_x1frac_y1_y1frac: [u8; 8],
}
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct Gsettile {
    #[bitfield(name = "cmd", ty = "libc::c_int", bits = "0..=7")]
    #[bitfield(name = "fmt", ty = "libc::c_uint", bits = "8..=10")]
    #[bitfield(name = "siz", ty = "libc::c_uint", bits = "11..=12")]
    #[bitfield(name = "pad0", ty = "libc::c_uint", bits = "13..=13")]
    #[bitfield(name = "line", ty = "libc::c_uint", bits = "14..=22")]
    #[bitfield(name = "tmem", ty = "libc::c_uint", bits = "23..=31")]
    #[bitfield(name = "pad1", ty = "libc::c_uint", bits = "32..=36")]
    #[bitfield(name = "tile", ty = "libc::c_uint", bits = "37..=39")]
    #[bitfield(name = "palette", ty = "libc::c_uint", bits = "40..=43")]
    #[bitfield(name = "ct", ty = "libc::c_uint", bits = "44..=44")]
    #[bitfield(name = "mt", ty = "libc::c_uint", bits = "45..=45")]
    #[bitfield(name = "maskt", ty = "libc::c_uint", bits = "46..=49")]
    #[bitfield(name = "shiftt", ty = "libc::c_uint", bits = "50..=53")]
    #[bitfield(name = "cs", ty = "libc::c_uint", bits = "54..=54")]
    #[bitfield(name = "ms", ty = "libc::c_uint", bits = "55..=55")]
    #[bitfield(name = "masks", ty = "libc::c_uint", bits = "56..=59")]
    #[bitfield(name = "shifts", ty = "libc::c_uint", bits = "60..=63")]
    pub cmd_fmt_siz_pad0_line_tmem_pad1_tile_palette_ct_mt_maskt_shiftt_cs_ms_masks_shifts: [u8; 8],
}
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct Gloadtile {
    #[bitfield(name = "cmd", ty = "libc::c_int", bits = "0..=7")]
    #[bitfield(name = "sl", ty = "libc::c_uint", bits = "8..=19")]
    #[bitfield(name = "tl", ty = "libc::c_uint", bits = "20..=31")]
    #[bitfield(name = "pad", ty = "libc::c_int", bits = "32..=36")]
    #[bitfield(name = "tile", ty = "libc::c_uint", bits = "37..=39")]
    #[bitfield(name = "sh", ty = "libc::c_uint", bits = "40..=51")]
    #[bitfield(name = "th", ty = "libc::c_uint", bits = "52..=63")]
    pub cmd_sl_tl_pad_tile_sh_th: [u8; 8],
}
pub type Gsettilesize = Gloadtile;
pub type Gloadtlut = Gloadtile;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Gwords {
    pub w0: libc::c_uint,
    pub w1: libc::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union Gfx {
    pub words: Gwords,
    pub dma: Gdma,
    pub tri: Gtri,
    pub line: Gline3D,
    pub popmtx: Gpopmtx,
    pub segment: Gsegment,
    pub setothermodeH: GsetothermodeH,
    pub setothermodeL: GsetothermodeL,
    pub texture: Gtexture,
    pub perspnorm: Gperspnorm,
    pub setimg: Gsetimg,
    pub setcombine: Gsetcombine,
    pub setcolor: Gsetcolor,
    pub fillrect: Gfillrect,
    pub settile: Gsettile,
    pub loadtile: Gloadtile,
    pub settilesize: Gsettilesize,
    pub loadtlut: Gloadtlut,
    pub force_structure_alignment: libc::c_longlong,
}
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
#[derive(Copy, Clone)]
#[repr(C)]
pub union Color_RGBA8_u32 {
    pub c2rust_unnamed: C2RustUnnamed_0,
    pub rgba: u32_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_0 {
    pub r: u8_0,
    pub g: u8_0,
    pub b: u8_0,
    pub a: u8_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct TransitionWipe {
    pub color: Color_RGBA8_u32,
    pub envColor: Color_RGBA8_u32,
    pub direction: u8_0,
    pub frame: u8_0,
    pub isDone: u8_0,
    pub texX: u16_0,
    pub texY: u16_0,
    pub normal: u16_0,
    pub projection: Mtx,
    pub lookAt: Mtx,
    pub modelView: [[Mtx; 3]; 2],
}
#[no_mangle]
pub static mut sWipe1Vtx: [Vtx; 25] =
    [Vtx{v:
             {
                 let mut init =
                     Vtx_t{ob:
                               [-(1299 as libc::c_int) as libc::c_short,
                                750 as libc::c_int as libc::c_short,
                                0 as libc::c_int as libc::c_short],
                           flag: 0 as libc::c_int as libc::c_ushort,
                           tc:
                               [13653 as libc::c_int as libc::c_short,
                                2048 as libc::c_int as libc::c_short],
                           cn:
                               [173 as libc::c_int as libc::c_uchar,
                                48 as libc::c_int as libc::c_uchar,
                                184 as libc::c_int as libc::c_uchar,
                                255 as libc::c_int as libc::c_uchar],};
                 init
             },},
     Vtx{v:
             {
                 let mut init =
                     Vtx_t{ob:
                               [-(750 as libc::c_int) as libc::c_short,
                                1299 as libc::c_int as libc::c_short,
                                0 as libc::c_int as libc::c_short],
                           flag: 0 as libc::c_int as libc::c_ushort,
                           tc:
                               [15019 as libc::c_int as libc::c_short,
                                2048 as libc::c_int as libc::c_short],
                           cn:
                               [208 as libc::c_int as libc::c_uchar,
                                83 as libc::c_int as libc::c_uchar,
                                184 as libc::c_int as libc::c_uchar,
                                255 as libc::c_int as libc::c_uchar],};
                 init
             },},
     Vtx{v:
             {
                 let mut init =
                     Vtx_t{ob:
                               [0 as libc::c_int as libc::c_short,
                                0 as libc::c_int as libc::c_short,
                                -(500 as libc::c_int) as libc::c_short],
                           flag: 0 as libc::c_int as libc::c_ushort,
                           tc:
                               [14336 as libc::c_int as libc::c_short,
                                0 as libc::c_int as libc::c_short],
                           cn:
                               [206 as libc::c_int as libc::c_uchar,
                                206 as libc::c_int as libc::c_uchar,
                                160 as libc::c_int as libc::c_uchar,
                                255 as libc::c_int as libc::c_uchar],};
                 init
             },},
     Vtx{v:
             {
                 let mut init =
                     Vtx_t{ob:
                               [0 as libc::c_int as libc::c_short,
                                1500 as libc::c_int as libc::c_short,
                                0 as libc::c_int as libc::c_short],
                           flag: 0 as libc::c_int as libc::c_ushort,
                           tc:
                               [16384 as libc::c_int as libc::c_short,
                                2048 as libc::c_int as libc::c_short],
                           cn:
                               [0 as libc::c_int as libc::c_uchar,
                                96 as libc::c_int as libc::c_uchar,
                                184 as libc::c_int as libc::c_uchar,
                                255 as libc::c_int as libc::c_uchar],};
                 init
             },},
     Vtx{v:
             {
                 let mut init =
                     Vtx_t{ob:
                               [0 as libc::c_int as libc::c_short,
                                0 as libc::c_int as libc::c_short,
                                -(500 as libc::c_int) as libc::c_short],
                           flag: 0 as libc::c_int as libc::c_ushort,
                           tc:
                               [15701 as libc::c_int as libc::c_short,
                                0 as libc::c_int as libc::c_short],
                           cn:
                               [61 as libc::c_int as libc::c_uchar,
                                61 as libc::c_int as libc::c_uchar,
                                173 as libc::c_int as libc::c_uchar,
                                255 as libc::c_int as libc::c_uchar],};
                 init
             },},
     Vtx{v:
             {
                 let mut init =
                     Vtx_t{ob:
                               [0 as libc::c_int as libc::c_short,
                                1500 as libc::c_int as libc::c_short,
                                0 as libc::c_int as libc::c_short],
                           flag: 0 as libc::c_int as libc::c_ushort,
                           tc:
                               [0 as libc::c_int as libc::c_short,
                                2048 as libc::c_int as libc::c_short],
                           cn:
                               [0 as libc::c_int as libc::c_uchar,
                                96 as libc::c_int as libc::c_uchar,
                                184 as libc::c_int as libc::c_uchar,
                                255 as libc::c_int as libc::c_uchar],};
                 init
             },},
     Vtx{v:
             {
                 let mut init =
                     Vtx_t{ob:
                               [750 as libc::c_int as libc::c_short,
                                1299 as libc::c_int as libc::c_short,
                                0 as libc::c_int as libc::c_short],
                           flag: 0 as libc::c_int as libc::c_ushort,
                           tc:
                               [1365 as libc::c_int as libc::c_short,
                                2048 as libc::c_int as libc::c_short],
                           cn:
                               [48 as libc::c_int as libc::c_uchar,
                                83 as libc::c_int as libc::c_uchar,
                                184 as libc::c_int as libc::c_uchar,
                                255 as libc::c_int as libc::c_uchar],};
                 init
             },},
     Vtx{v:
             {
                 let mut init =
                     Vtx_t{ob:
                               [0 as libc::c_int as libc::c_short,
                                0 as libc::c_int as libc::c_short,
                                -(500 as libc::c_int) as libc::c_short],
                           flag: 0 as libc::c_int as libc::c_ushort,
                           tc:
                               [683 as libc::c_int as libc::c_short,
                                0 as libc::c_int as libc::c_short],
                           cn:
                               [61 as libc::c_int as libc::c_uchar,
                                61 as libc::c_int as libc::c_uchar,
                                173 as libc::c_int as libc::c_uchar,
                                255 as libc::c_int as libc::c_uchar],};
                 init
             },},
     Vtx{v:
             {
                 let mut init =
                     Vtx_t{ob:
                               [1299 as libc::c_int as libc::c_short,
                                750 as libc::c_int as libc::c_short,
                                0 as libc::c_int as libc::c_short],
                           flag: 0 as libc::c_int as libc::c_ushort,
                           tc:
                               [2731 as libc::c_int as libc::c_short,
                                2048 as libc::c_int as libc::c_short],
                           cn:
                               [83 as libc::c_int as libc::c_uchar,
                                48 as libc::c_int as libc::c_uchar,
                                184 as libc::c_int as libc::c_uchar,
                                255 as libc::c_int as libc::c_uchar],};
                 init
             },},
     Vtx{v:
             {
                 let mut init =
                     Vtx_t{ob:
                               [0 as libc::c_int as libc::c_short,
                                0 as libc::c_int as libc::c_short,
                                -(500 as libc::c_int) as libc::c_short],
                           flag: 0 as libc::c_int as libc::c_ushort,
                           tc:
                               [2048 as libc::c_int as libc::c_short,
                                0 as libc::c_int as libc::c_short],
                           cn:
                               [61 as libc::c_int as libc::c_uchar,
                                61 as libc::c_int as libc::c_uchar,
                                173 as libc::c_int as libc::c_uchar,
                                255 as libc::c_int as libc::c_uchar],};
                 init
             },},
     Vtx{v:
             {
                 let mut init =
                     Vtx_t{ob:
                               [1500 as libc::c_int as libc::c_short,
                                0 as libc::c_int as libc::c_short,
                                0 as libc::c_int as libc::c_short],
                           flag: 0 as libc::c_int as libc::c_ushort,
                           tc:
                               [4096 as libc::c_int as libc::c_short,
                                2048 as libc::c_int as libc::c_short],
                           cn:
                               [96 as libc::c_int as libc::c_uchar,
                                0 as libc::c_int as libc::c_uchar,
                                184 as libc::c_int as libc::c_uchar,
                                255 as libc::c_int as libc::c_uchar],};
                 init
             },},
     Vtx{v:
             {
                 let mut init =
                     Vtx_t{ob:
                               [0 as libc::c_int as libc::c_short,
                                0 as libc::c_int as libc::c_short,
                                -(500 as libc::c_int) as libc::c_short],
                           flag: 0 as libc::c_int as libc::c_ushort,
                           tc:
                               [3413 as libc::c_int as libc::c_short,
                                0 as libc::c_int as libc::c_short],
                           cn:
                               [61 as libc::c_int as libc::c_uchar,
                                61 as libc::c_int as libc::c_uchar,
                                173 as libc::c_int as libc::c_uchar,
                                255 as libc::c_int as libc::c_uchar],};
                 init
             },},
     Vtx{v:
             {
                 let mut init =
                     Vtx_t{ob:
                               [1299 as libc::c_int as libc::c_short,
                                -(750 as libc::c_int) as libc::c_short,
                                0 as libc::c_int as libc::c_short],
                           flag: 0 as libc::c_int as libc::c_ushort,
                           tc:
                               [5461 as libc::c_int as libc::c_short,
                                2048 as libc::c_int as libc::c_short],
                           cn:
                               [83 as libc::c_int as libc::c_uchar,
                                208 as libc::c_int as libc::c_uchar,
                                184 as libc::c_int as libc::c_uchar,
                                255 as libc::c_int as libc::c_uchar],};
                 init
             },},
     Vtx{v:
             {
                 let mut init =
                     Vtx_t{ob:
                               [0 as libc::c_int as libc::c_short,
                                0 as libc::c_int as libc::c_short,
                                -(500 as libc::c_int) as libc::c_short],
                           flag: 0 as libc::c_int as libc::c_ushort,
                           tc:
                               [4779 as libc::c_int as libc::c_short,
                                0 as libc::c_int as libc::c_short],
                           cn:
                               [61 as libc::c_int as libc::c_uchar,
                                61 as libc::c_int as libc::c_uchar,
                                173 as libc::c_int as libc::c_uchar,
                                255 as libc::c_int as libc::c_uchar],};
                 init
             },},
     Vtx{v:
             {
                 let mut init =
                     Vtx_t{ob:
                               [750 as libc::c_int as libc::c_short,
                                -(1299 as libc::c_int) as libc::c_short,
                                0 as libc::c_int as libc::c_short],
                           flag: 0 as libc::c_int as libc::c_ushort,
                           tc:
                               [6827 as libc::c_int as libc::c_short,
                                2048 as libc::c_int as libc::c_short],
                           cn:
                               [48 as libc::c_int as libc::c_uchar,
                                173 as libc::c_int as libc::c_uchar,
                                184 as libc::c_int as libc::c_uchar,
                                255 as libc::c_int as libc::c_uchar],};
                 init
             },},
     Vtx{v:
             {
                 let mut init =
                     Vtx_t{ob:
                               [0 as libc::c_int as libc::c_short,
                                0 as libc::c_int as libc::c_short,
                                -(500 as libc::c_int) as libc::c_short],
                           flag: 0 as libc::c_int as libc::c_ushort,
                           tc:
                               [6144 as libc::c_int as libc::c_short,
                                0 as libc::c_int as libc::c_short],
                           cn:
                               [206 as libc::c_int as libc::c_uchar,
                                206 as libc::c_int as libc::c_uchar,
                                160 as libc::c_int as libc::c_uchar,
                                255 as libc::c_int as libc::c_uchar],};
                 init
             },},
     Vtx{v:
             {
                 let mut init =
                     Vtx_t{ob:
                               [0 as libc::c_int as libc::c_short,
                                -(1500 as libc::c_int) as libc::c_short,
                                0 as libc::c_int as libc::c_short],
                           flag: 0 as libc::c_int as libc::c_ushort,
                           tc:
                               [8192 as libc::c_int as libc::c_short,
                                2048 as libc::c_int as libc::c_short],
                           cn:
                               [0 as libc::c_int as libc::c_uchar,
                                160 as libc::c_int as libc::c_uchar,
                                184 as libc::c_int as libc::c_uchar,
                                255 as libc::c_int as libc::c_uchar],};
                 init
             },},
     Vtx{v:
             {
                 let mut init =
                     Vtx_t{ob:
                               [0 as libc::c_int as libc::c_short,
                                0 as libc::c_int as libc::c_short,
                                -(500 as libc::c_int) as libc::c_short],
                           flag: 0 as libc::c_int as libc::c_ushort,
                           tc:
                               [7509 as libc::c_int as libc::c_short,
                                0 as libc::c_int as libc::c_short],
                           cn:
                               [206 as libc::c_int as libc::c_uchar,
                                206 as libc::c_int as libc::c_uchar,
                                160 as libc::c_int as libc::c_uchar,
                                255 as libc::c_int as libc::c_uchar],};
                 init
             },},
     Vtx{v:
             {
                 let mut init =
                     Vtx_t{ob:
                               [-(750 as libc::c_int) as libc::c_short,
                                -(1299 as libc::c_int) as libc::c_short,
                                0 as libc::c_int as libc::c_short],
                           flag: 0 as libc::c_int as libc::c_ushort,
                           tc:
                               [9557 as libc::c_int as libc::c_short,
                                2048 as libc::c_int as libc::c_short],
                           cn:
                               [208 as libc::c_int as libc::c_uchar,
                                173 as libc::c_int as libc::c_uchar,
                                184 as libc::c_int as libc::c_uchar,
                                255 as libc::c_int as libc::c_uchar],};
                 init
             },},
     Vtx{v:
             {
                 let mut init =
                     Vtx_t{ob:
                               [0 as libc::c_int as libc::c_short,
                                0 as libc::c_int as libc::c_short,
                                -(500 as libc::c_int) as libc::c_short],
                           flag: 0 as libc::c_int as libc::c_ushort,
                           tc:
                               [8875 as libc::c_int as libc::c_short,
                                0 as libc::c_int as libc::c_short],
                           cn:
                               [206 as libc::c_int as libc::c_uchar,
                                206 as libc::c_int as libc::c_uchar,
                                160 as libc::c_int as libc::c_uchar,
                                255 as libc::c_int as libc::c_uchar],};
                 init
             },},
     Vtx{v:
             {
                 let mut init =
                     Vtx_t{ob:
                               [-(1299 as libc::c_int) as libc::c_short,
                                -(750 as libc::c_int) as libc::c_short,
                                0 as libc::c_int as libc::c_short],
                           flag: 0 as libc::c_int as libc::c_ushort,
                           tc:
                               [10923 as libc::c_int as libc::c_short,
                                2048 as libc::c_int as libc::c_short],
                           cn:
                               [173 as libc::c_int as libc::c_uchar,
                                208 as libc::c_int as libc::c_uchar,
                                184 as libc::c_int as libc::c_uchar,
                                255 as libc::c_int as libc::c_uchar],};
                 init
             },},
     Vtx{v:
             {
                 let mut init =
                     Vtx_t{ob:
                               [0 as libc::c_int as libc::c_short,
                                0 as libc::c_int as libc::c_short,
                                -(500 as libc::c_int) as libc::c_short],
                           flag: 0 as libc::c_int as libc::c_ushort,
                           tc:
                               [10240 as libc::c_int as libc::c_short,
                                0 as libc::c_int as libc::c_short],
                           cn:
                               [206 as libc::c_int as libc::c_uchar,
                                206 as libc::c_int as libc::c_uchar,
                                160 as libc::c_int as libc::c_uchar,
                                255 as libc::c_int as libc::c_uchar],};
                 init
             },},
     Vtx{v:
             {
                 let mut init =
                     Vtx_t{ob:
                               [-(1500 as libc::c_int) as libc::c_short,
                                0 as libc::c_int as libc::c_short,
                                0 as libc::c_int as libc::c_short],
                           flag: 0 as libc::c_int as libc::c_ushort,
                           tc:
                               [12288 as libc::c_int as libc::c_short,
                                2048 as libc::c_int as libc::c_short],
                           cn:
                               [160 as libc::c_int as libc::c_uchar,
                                0 as libc::c_int as libc::c_uchar,
                                184 as libc::c_int as libc::c_uchar,
                                255 as libc::c_int as libc::c_uchar],};
                 init
             },},
     Vtx{v:
             {
                 let mut init =
                     Vtx_t{ob:
                               [0 as libc::c_int as libc::c_short,
                                0 as libc::c_int as libc::c_short,
                                -(500 as libc::c_int) as libc::c_short],
                           flag: 0 as libc::c_int as libc::c_ushort,
                           tc:
                               [11605 as libc::c_int as libc::c_short,
                                0 as libc::c_int as libc::c_short],
                           cn:
                               [206 as libc::c_int as libc::c_uchar,
                                206 as libc::c_int as libc::c_uchar,
                                160 as libc::c_int as libc::c_uchar,
                                255 as libc::c_int as libc::c_uchar],};
                 init
             },},
     Vtx{v:
             {
                 let mut init =
                     Vtx_t{ob:
                               [0 as libc::c_int as libc::c_short,
                                0 as libc::c_int as libc::c_short,
                                -(500 as libc::c_int) as libc::c_short],
                           flag: 0 as libc::c_int as libc::c_ushort,
                           tc:
                               [12971 as libc::c_int as libc::c_short,
                                0 as libc::c_int as libc::c_short],
                           cn:
                               [206 as libc::c_int as libc::c_uchar,
                                206 as libc::c_int as libc::c_uchar,
                                160 as libc::c_int as libc::c_uchar,
                                255 as libc::c_int as libc::c_uchar],};
                 init
             },}];
#[no_mangle]
pub static mut sWipe1Tex: [u64_0; 256] =
    [0 as libc::c_int as u64_0, 0 as libc::c_int as u64_0,
     0 as libc::c_int as u64_0, 0 as libc::c_int as u64_0,
     0 as libc::c_int as u64_0, 0 as libc::c_int as u64_0,
     0 as libc::c_int as u64_0, 0 as libc::c_int as u64_0,
     0 as libc::c_int as u64_0, 0 as libc::c_int as u64_0,
     0 as libc::c_int as u64_0, 0 as libc::c_int as u64_0,
     0 as libc::c_int as u64_0, 0 as libc::c_int as u64_0,
     0 as libc::c_int as u64_0, 0 as libc::c_int as u64_0,
     0 as libc::c_int as u64_0, 0 as libc::c_int as u64_0,
     0 as libc::c_int as u64_0, 0 as libc::c_int as u64_0,
     0 as libc::c_int as u64_0, 0 as libc::c_int as u64_0,
     0 as libc::c_int as u64_0, 0 as libc::c_int as u64_0,
     0 as libc::c_int as u64_0, 0 as libc::c_int as u64_0,
     0 as libc::c_int as u64_0, 0 as libc::c_int as u64_0,
     0 as libc::c_int as u64_0, 0 as libc::c_int as u64_0,
     0 as libc::c_int as u64_0, 0 as libc::c_int as u64_0,
     0 as libc::c_int as u64_0, 0 as libc::c_int as u64_0,
     0 as libc::c_int as u64_0, 0 as libc::c_int as u64_0,
     0 as libc::c_int as u64_0, 0 as libc::c_int as u64_0,
     0 as libc::c_int as u64_0, 0 as libc::c_int as u64_0,
     0 as libc::c_int as u64_0, 0 as libc::c_int as u64_0,
     0 as libc::c_int as u64_0, 0 as libc::c_int as u64_0,
     0 as libc::c_int as u64_0, 0 as libc::c_int as u64_0,
     0 as libc::c_int as u64_0, 0 as libc::c_int as u64_0,
     0 as libc::c_int as u64_0, 0 as libc::c_int as u64_0,
     0 as libc::c_int as u64_0, 0 as libc::c_int as u64_0,
     0 as libc::c_int as u64_0, 0 as libc::c_int as u64_0,
     0 as libc::c_int as u64_0, 0 as libc::c_int as u64_0,
     0 as libc::c_int as u64_0, 0 as libc::c_int as u64_0,
     0 as libc::c_int as u64_0, 0x1000 as libc::c_int as u64_0,
     0x100000000000000 as libc::c_longlong as u64_0,
     0 as libc::c_int as u64_0, 0 as libc::c_int as u64_0,
     0x1000 as libc::c_int as u64_0,
     0x100000000000000 as libc::c_longlong as u64_0,
     0 as libc::c_int as u64_0, 0 as libc::c_int as u64_0,
     0x1000 as libc::c_int as u64_0,
     0x1200000000000000 as libc::c_longlong as u64_0,
     0 as libc::c_int as u64_0, 0 as libc::c_int as u64_0,
     0x2001 as libc::c_int as u64_0,
     0x2300000000000000 as libc::c_longlong as u64_0,
     0 as libc::c_int as u64_0, 0 as libc::c_int as u64_0,
     0x12001 as libc::c_int as u64_0,
     0x2410000000000010 as libc::c_longlong as u64_0,
     0 as libc::c_int as u64_0, 0 as libc::c_int as u64_0,
     0x13012 as libc::c_int as u64_0,
     0x3410000000000010 as libc::c_longlong as u64_0,
     0x100000000000000 as libc::c_longlong as u64_0,
     0 as libc::c_int as u64_0, 0x13123 as libc::c_int as u64_0,
     0x4521100000010011 as libc::c_longlong as u64_0,
     0x100000000000000 as libc::c_longlong as u64_0,
     0x10 as libc::c_int as u64_0, 0x14134 as libc::c_int as u64_0,
     0x5631100000010021 as libc::c_longlong as u64_0,
     0x200000000000000 as libc::c_longlong as u64_0,
     0x10 as libc::c_int as u64_0,
     0x100000010024235 as libc::c_longlong as u64_0,
     0x6632200000020032 as libc::c_longlong as u64_0,
     0x1310000000000000 as libc::c_longlong as u64_0,
     0x20 as libc::c_int as u64_0,
     0x100000010025345 as libc::c_longlong as u64_0,
     0x6742200000020132 as libc::c_longlong as u64_0,
     0x2310000000000000 as libc::c_longlong as u64_0,
     0x110000000000030 as libc::c_longlong as u64_0,
     0x1200100010036456 as libc::c_longlong as u64_0,
     0x7753300100131242 as libc::c_longlong as u64_0,
     0x2420100000000010 as libc::c_longlong as u64_0,
     0x110000000000131 as libc::c_longlong as u64_0,
     0x1300110010036467 as libc::c_longlong as u64_0,
     0x7754410100141253 as libc::c_longlong as u64_0,
     0x3520100000000010 as libc::c_longlong as u64_0,
     0x110000000000241 as libc::c_longlong as u64_0,
     0x2411210020147567 as libc::c_longlong as u64_0,
     0x7764511200242353 as libc::c_longlong as u64_0,
     0x4630100000000020 as libc::c_longlong as u64_0,
     0x110000000000252 as libc::c_longlong as u64_0,
     0x3411310120157677 as libc::c_longlong as u64_0,
     0x8765521300253464 as libc::c_ulonglong,
     0x5631200000000131 as libc::c_longlong as u64_0,
     0x210000000100352 as libc::c_longlong as u64_0,
     0x3522410221267777 as libc::c_longlong as u64_0,
     0x8775621311363565 as libc::c_ulonglong,
     0x5741300010000131 as libc::c_longlong as u64_0,
     0x1210100000100462 as libc::c_longlong as u64_0,
     0x4623510221367778 as libc::c_longlong as u64_0,
     0x8876632411464675 as libc::c_ulonglong,
     0x6752400110000242 as libc::c_longlong as u64_0,
     0x1310200100211563 as libc::c_longlong as u64_0,
     0x5634511331478878 as libc::c_longlong as u64_0,
     0x8877642422565676 as libc::c_ulonglong,
     0x7753410210100353 as libc::c_longlong as u64_0,
     0x2321310101311674 as libc::c_longlong as u64_0,
     0x6635621432478888 as libc::c_longlong as u64_0,
     0x8887753422575776 as libc::c_ulonglong,
     0x7754510310110463 as libc::c_longlong as u64_0,
     0x3422310201422674 as libc::c_longlong as u64_0,
     0x6746622542578988 as libc::c_longlong as u64_0,
     0x8898753533676777 as libc::c_ulonglong,
     0x7765510310211464 as libc::c_longlong as u64_0,
     0x4533411212523775 as libc::c_longlong as u64_0,
     0x7756723653678988 as libc::c_longlong as u64_0,
     0x89a8764544676787 as libc::c_ulonglong,
     0x7776621421321575 as libc::c_longlong as u64_0,
     0x5543521313634775 as libc::c_longlong as u64_0,
     0x7757734654788a88 as libc::c_longlong as u64_0,
     0x9aa8775655776788 as libc::c_ulonglong,
     0x8876722521422675 as libc::c_ulonglong,
     0x6654622423645776 as libc::c_longlong as u64_0,
     0x7767745765789b89 as libc::c_longlong as u64_0,
     0x9bb8875655777888 as libc::c_ulonglong,
     0x8887732521433676 as libc::c_ulonglong,
     0x6765633524755777 as libc::c_longlong as u64_0,
     0x7777756765789b89 as libc::c_longlong as u64_0,
     0xabc8876766777898 as libc::c_ulonglong,
     0x8898743632543776 as libc::c_ulonglong,
     0x7766734535756777 as libc::c_longlong as u64_0,
     0x887775676678ac9a as libc::c_ulonglong,
     0xbcc88767667778a9 as libc::c_ulonglong,
     0x8898754633654777 as libc::c_ulonglong,
     0x7777745646767787 as libc::c_longlong as u64_0,
     0x898876777678bd9b as libc::c_ulonglong,
     0xcdd89877767878aa as libc::c_ulonglong,
     0x8998765644665777 as libc::c_ulonglong,
     0x7777756746777888 as libc::c_longlong as u64_0,
     0x898887788778cd9c as libc::c_ulonglong,
     0xded99877777888ba as libc::c_ulonglong,
     0x99a8766644766788 as libc::c_ulonglong,
     0x8888757756777899 as libc::c_ulonglong,
     0x898888888788cd9d as libc::c_ulonglong,
     0xeed9a888878888cb as libc::c_ulonglong,
     0x9aa8776755777788 as libc::c_ulonglong,
     0x88887677677888aa as libc::c_ulonglong,
     0x8a9889889888ddad as libc::c_ulonglong,
     0xfedab988988988dc as libc::c_ulonglong,
     0xabb8887756777789 as libc::c_ulonglong,
     0x88987677678888aa as libc::c_ulonglong,
     0x8aa89a88a889ddae as libc::c_ulonglong,
     0xffdac988988988dd as libc::c_ulonglong,
     0xacb8887766787899 as libc::c_ulonglong,
     0x88987777778988bb as libc::c_ulonglong,
     0x9ba89b88b88aedbe as libc::c_ulonglong,
     0xffdbda88a88a89ed as libc::c_ulonglong,
     0xbdc889876789889a as libc::c_ulonglong,
     0x88a88777778a88cb as libc::c_ulonglong,
     0x9cb8ac88c98afdcf as libc::c_ulonglong,
     0xffeceb89b89b89ed as libc::c_ulonglong,
     0xcec89a87778988ab as libc::c_ulonglong,
     0x89a88788788b89dc as libc::c_ulonglong,
     0x9cc9bc88da8bfecf as libc::c_ulonglong,
     0xffeceb8ac9ac9aee as libc::c_ulonglong,
     0xced89a87778a88bc as libc::c_ulonglong,
     0x9ab89888788b8aed as libc::c_ulonglong,
     0x9dc9cd89eb8cfedf as libc::c_ulonglong,
     0xfffdfc9bd9bd9afe as libc::c_ulonglong,
     0xdfe9ab88778a88cd as libc::c_ulonglong,
     0x9bc8a988889c9aed as libc::c_ulonglong,
     0xadd9de89eb9cfeef as libc::c_ulonglong,
     0xfffefd9bd9bdabfe as libc::c_ulonglong,
     0xdfe9bc88888b98dd as libc::c_ulonglong,
     0xabc9aa8888ad9bed as libc::c_ulonglong,
     0xbee9de99ec9dfeff as libc::c_ulonglong,
     0xfffffdace9ceacfe as libc::c_ulonglong,
     0xdfe9cc89888c99ee as libc::c_ulonglong,
     0xacc9bb8888ae9bfd as libc::c_ulonglong,
     0xbfe9ee99ec9dffff as libc::c_ulonglong,
     0xfffffdade9debdfe as libc::c_ulonglong,
     0xefeadd99889da9ee as libc::c_ulonglong,
     0xaddacc8988beacfe as libc::c_ulonglong,
     0xcfeaeeaaed9effff as libc::c_ulonglong,
     0xfffffebeeadfcefe as libc::c_ulonglong,
     0xefebed9a88adaafe as libc::c_ulonglong,
     0xbedadc9998cebdfe as libc::c_ulonglong,
     0xdfeaffabfdaeffff as libc::c_ulonglong,
     0xfffffecedaefcefe as libc::c_ulonglong,
     0xfffced9a88bebbfe as libc::c_ulonglong,
     0xbedbedaaa9dfbdfd as libc::c_ulonglong,
     0xefebffbcfebeffff as libc::c_ulonglong,
     0xfffffedfebefdfff as libc::c_ulonglong,
     0xfffdfc9b99cebcfe as libc::c_ulonglong,
     0xcfecfebbb9efcefd as libc::c_ulonglong,
     0xefecffcdfecefeff as libc::c_ulonglong,
     0xfffffedfecefdfff as libc::c_ulonglong,
     0xfffdfcacaacecdfe as libc::c_ulonglong,
     0xdfecfeccbaefdeed as libc::c_ulonglong,
     0xffedffddeecfeeff as libc::c_ulonglong,
     0xfffffeefecefefff as libc::c_ulonglong,
     0xfffefcbcabddddfe as libc::c_ulonglong,
     0xdfedfecdcbffeeec as libc::c_ulonglong,
     0xffddffeeeedfeeff as libc::c_ulonglong,
     0xfffffeefedffefff as libc::c_ulonglong,
     0xfffffccdaceddefe as libc::c_ulonglong,
     0xeffefededcffefec as libc::c_ulonglong,
     0xffdeffffeeefeeff as libc::c_ulonglong,
     0xffffffffeeffffff as libc::c_ulonglong,
     0xfffffccdadfddffe as libc::c_ulonglong,
     0xeffffeeeedffffec as libc::c_ulonglong,
     0xffdffeffefffefff as libc::c_ulonglong,
     0xffffffffeefeffff as libc::c_ulonglong,
     0xfffffcddbefdefff as libc::c_ulonglong,
     0xfffffeefedffffeb as libc::c_ulonglong,
     0xffdffeffefffefff as libc::c_ulonglong,
     0xffffeffffffeffff as libc::c_ulonglong,
     0xfffffdeebefdefff as libc::c_ulonglong,
     0xfffffeffeeffffeb as libc::c_ulonglong,
     0xffdffeffefffefff as libc::c_ulonglong,
     0xffffeffffffdffff as libc::c_ulonglong,
     0xfffffdfebffdffff as libc::c_ulonglong,
     0xfffffeefeefffffc as libc::c_ulonglong,
     0xffeffeffefffefff as libc::c_ulonglong,
     0xffffeffffffdffff as libc::c_ulonglong,
     0xfffffeffaffeffff as libc::c_ulonglong,
     0xffeffeefeefffffc as libc::c_ulonglong,
     0xffeffeffefffefff as libc::c_ulonglong,
     0xffffdffffffcffff as libc::c_ulonglong,
     0xffffffffaffeffff as libc::c_ulonglong,
     0xffeffeefeefffffc as libc::c_ulonglong,
     0xffeffeffffffffff as libc::c_ulonglong,
     0xffffdffffffcffff as libc::c_ulonglong,
     0xfffffffeaefeffff as libc::c_ulonglong,
     0xffefffefedfffffc as libc::c_ulonglong,
     0xfffffeffffffffff as libc::c_ulonglong];
// Initialized in run_static_initializers
#[no_mangle]
pub static mut sWipeDList: [Gfx; 31] =
    [Gfx{words: Gwords{w0: 0, w1: 0,},}; 31];
// unused.
#[no_mangle]
pub static mut sWipeSyncDList: [Gfx; 2] =
    [Gfx{words:
             {
                 let mut init =
                     Gwords{w0:
                                (0xe7 as libc::c_int as u32_0 &
                                     (((0x1 as libc::c_int) <<
                                           8 as libc::c_int) -
                                          1 as libc::c_int) as libc::c_uint)
                                    << 24 as libc::c_int,
                            w1: 0 as libc::c_int as libc::c_uint,};
                 init
             },},
     Gfx{words:
             {
                 let mut init =
                     Gwords{w0:
                                (0xdf as libc::c_int as u32_0 &
                                     (((0x1 as libc::c_int) <<
                                           8 as libc::c_int) -
                                          1 as libc::c_int) as libc::c_uint)
                                    << 24 as libc::c_int,
                            w1: 0 as libc::c_int as libc::c_uint,};
                 init
             },}];
#[no_mangle]
pub unsafe extern "C" fn TransitionWipe_Start(mut thisx: *mut libc::c_void) {
    let mut this: *mut TransitionWipe = thisx as *mut TransitionWipe;
    (*this).isDone = 0 as libc::c_int as u8_0;
    if (*this).direction != 0 {
        (*this).texY = 0x14d as libc::c_int as u16_0
    } else { (*this).texY = 0x264 as libc::c_int as u16_0 }
    guPerspective(&mut (*this).projection, &mut (*this).normal, 60.0f32,
                  (4.0f64 / 3.0f32 as libc::c_double) as f32_0, 10.0f32,
                  12800.0f32, 1.0f32);
    guLookAt(&mut (*this).lookAt, 0.0f32, 0.0f32, 400.0f32, 0.0f32, 0.0f32,
             0.0f32, 0.0f32, 1.0f32, 0.0f32);
}
#[no_mangle]
pub unsafe extern "C" fn TransitionWipe_Init(mut thisx: *mut libc::c_void)
 -> *mut libc::c_void {
    let mut this: *mut TransitionWipe = thisx as *mut TransitionWipe;
    bzero(this as *mut libc::c_void,
          ::std::mem::size_of::<TransitionWipe>() as libc::c_ulong);
    return this as *mut libc::c_void;
}
#[no_mangle]
pub unsafe extern "C" fn TransitionWipe_Destroy(mut thisx:
                                                    *mut libc::c_void) {
}
#[no_mangle]
pub unsafe extern "C" fn TransitionWipe_Update(mut thisx: *mut libc::c_void,
                                               mut updateRate: s32) {
    let mut this: *mut TransitionWipe = thisx as *mut TransitionWipe;
    let mut unk1419: u8_0 = 0;
    if (*this).direction as libc::c_int != 0 as libc::c_int {
        unk1419 = gSaveContext.unk_1419;
        (*this).texY =
            ((*this).texY as libc::c_int +
                 unk1419 as libc::c_int * 3 as libc::c_int / updateRate) as
                u16_0;
        if (*this).texY as libc::c_int >= 0x264 as libc::c_int {
            (*this).texY = 0x264 as libc::c_int as u16_0;
            (*this).isDone = 1 as libc::c_int as u8_0
        }
    } else {
        unk1419 = gSaveContext.unk_1419;
        (*this).texY =
            ((*this).texY as libc::c_int -
                 unk1419 as libc::c_int * 3 as libc::c_int / updateRate) as
                u16_0;
        if ((*this).texY as libc::c_int) < 0x14e as libc::c_int {
            (*this).texY = 0x14d as libc::c_int as u16_0;
            (*this).isDone = 1 as libc::c_int as u8_0
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn TransitionWipe_Draw(mut thisx: *mut libc::c_void,
                                             mut gfxP: *mut *mut Gfx) {
    let mut gfx: *mut Gfx = *gfxP;
    let mut modelView: *mut Mtx = 0 as *mut Mtx;
    let mut this: *mut TransitionWipe = thisx as *mut TransitionWipe;
    let mut pad: [s32; 4] = [0; 4];
    let mut tex: *mut Gfx = 0 as *mut Gfx;
    modelView = (*this).modelView[(*this).frame as usize].as_mut_ptr();
    (*this).frame = ((*this).frame as libc::c_int ^ 1 as libc::c_int) as u8_0;
    guScale(&mut *modelView.offset(0 as libc::c_int as isize), 0.56f32,
            0.56f32, 1.0f32);
    guRotate(&mut *modelView.offset(1 as libc::c_int as isize), 0.0f32,
             0.0f32, 0.0f32, 1.0f32);
    guTranslate(&mut *modelView.offset(2 as libc::c_int as isize), 0.0f32,
                0.0f32, 0.0f32);
    let fresh0 = gfx;
    gfx = gfx.offset(1);
    let mut _g: *mut Gfx = fresh0;
    (*_g).words.w0 =
        (0xe7 as libc::c_int as u32_0 &
             (((0x1 as libc::c_int) << 8 as libc::c_int) - 1 as libc::c_int)
                 as libc::c_uint) << 24 as libc::c_int;
    (*_g).words.w1 = 0 as libc::c_int as libc::c_uint;
    tex =
        Gfx_BranchTexScroll(&mut gfx, (*this).texX as u32_0,
                            (*this).texY as u32_0, 0 as libc::c_int,
                            0 as libc::c_int);
    let fresh1 = gfx;
    gfx = gfx.offset(1);
    let mut _g_0: *mut Gfx = fresh1;
    (*_g_0).words.w0 =
        (0xdb as libc::c_int as u32_0 &
             (((0x1 as libc::c_int) << 8 as libc::c_int) - 1 as libc::c_int)
                 as libc::c_uint) << 24 as libc::c_int |
            (0x6 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 16 as libc::c_int
            |
            ((8 as libc::c_int * 4 as libc::c_int) as u32_0 &
                 (((0x1 as libc::c_int) << 16 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 0 as libc::c_int;
    (*_g_0).words.w1 = tex as libc::c_uint;
    let fresh2 = gfx;
    gfx = gfx.offset(1);
    let mut _g_1: *mut Gfx = fresh2;
    (*_g_1).words.w0 =
        (0xfa as libc::c_int as u32_0 &
             (((0x1 as libc::c_int) << 8 as libc::c_int) - 1 as libc::c_int)
                 as libc::c_uint) << 24 as libc::c_int |
            (0 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 8 as libc::c_int |
            (0x80 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 0 as libc::c_int;
    (*_g_1).words.w1 =
        ((*this).color.c2rust_unnamed.r as u32_0 &
             (((0x1 as libc::c_int) << 8 as libc::c_int) - 1 as libc::c_int)
                 as libc::c_uint) << 24 as libc::c_int |
            ((*this).color.c2rust_unnamed.g as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 16 as libc::c_int
            |
            ((*this).color.c2rust_unnamed.b as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 8 as libc::c_int |
            (255 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 0 as libc::c_int;
    let fresh3 = gfx;
    gfx = gfx.offset(1);
    let mut _g_2: *mut Gfx = fresh3;
    (*_g_2).words.w0 =
        (0xda as libc::c_int as u32_0 &
             (((0x1 as libc::c_int) << 8 as libc::c_int) - 1 as libc::c_int)
                 as libc::c_uint) << 24 as libc::c_int |
            ((::std::mem::size_of::<Mtx>() as
                  libc::c_ulong).wrapping_sub(1 as libc::c_int as
                                                  libc::c_uint).wrapping_div(8
                                                                                 as
                                                                                 libc::c_int
                                                                                 as
                                                                                 libc::c_uint)
                 &
                 (((0x1 as libc::c_int) << 5 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 19 as libc::c_int
            |
            ((0 as libc::c_int / 8 as libc::c_int) as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 8 as libc::c_int |
            (((0x2 as libc::c_int | 0x4 as libc::c_int) ^ 0x1 as libc::c_int)
                 as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 0 as libc::c_int;
    (*_g_2).words.w1 = &mut (*this).projection as *mut Mtx as libc::c_uint;
    let fresh4 = gfx;
    gfx = gfx.offset(1);
    let mut _g_3: *mut Gfx = fresh4;
    (*_g_3).words.w0 =
        (0xdb as libc::c_int as u32_0 &
             (((0x1 as libc::c_int) << 8 as libc::c_int) - 1 as libc::c_int)
                 as libc::c_uint) << 24 as libc::c_int |
            (0xe as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 16 as libc::c_int
            |
            (0 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 16 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 0 as libc::c_int;
    (*_g_3).words.w1 = (*this).normal as libc::c_uint;
    let fresh5 = gfx;
    gfx = gfx.offset(1);
    let mut _g_4: *mut Gfx = fresh5;
    (*_g_4).words.w0 =
        (0xda as libc::c_int as u32_0 &
             (((0x1 as libc::c_int) << 8 as libc::c_int) - 1 as libc::c_int)
                 as libc::c_uint) << 24 as libc::c_int |
            ((::std::mem::size_of::<Mtx>() as
                  libc::c_ulong).wrapping_sub(1 as libc::c_int as
                                                  libc::c_uint).wrapping_div(8
                                                                                 as
                                                                                 libc::c_int
                                                                                 as
                                                                                 libc::c_uint)
                 &
                 (((0x1 as libc::c_int) << 5 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 19 as libc::c_int
            |
            ((0 as libc::c_int / 8 as libc::c_int) as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 8 as libc::c_int |
            (((0 as libc::c_int | 0x4 as libc::c_int) ^ 0x1 as libc::c_int) as
                 u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 0 as libc::c_int;
    (*_g_4).words.w1 = &mut (*this).lookAt as *mut Mtx as libc::c_uint;
    let fresh6 = gfx;
    gfx = gfx.offset(1);
    let mut _g_5: *mut Gfx = fresh6;
    (*_g_5).words.w0 =
        (0xda as libc::c_int as u32_0 &
             (((0x1 as libc::c_int) << 8 as libc::c_int) - 1 as libc::c_int)
                 as libc::c_uint) << 24 as libc::c_int |
            ((::std::mem::size_of::<Mtx>() as
                  libc::c_ulong).wrapping_sub(1 as libc::c_int as
                                                  libc::c_uint).wrapping_div(8
                                                                                 as
                                                                                 libc::c_int
                                                                                 as
                                                                                 libc::c_uint)
                 &
                 (((0x1 as libc::c_int) << 5 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 19 as libc::c_int
            |
            ((0 as libc::c_int / 8 as libc::c_int) as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 8 as libc::c_int |
            (((0 as libc::c_int | 0x2 as libc::c_int | 0 as libc::c_int) ^
                  0x1 as libc::c_int) as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 0 as libc::c_int;
    (*_g_5).words.w1 =
        &mut *modelView.offset(0 as libc::c_int as isize) as *mut Mtx as
            libc::c_uint;
    let fresh7 = gfx;
    gfx = gfx.offset(1);
    let mut _g_6: *mut Gfx = fresh7;
    (*_g_6).words.w0 =
        (0xda as libc::c_int as u32_0 &
             (((0x1 as libc::c_int) << 8 as libc::c_int) - 1 as libc::c_int)
                 as libc::c_uint) << 24 as libc::c_int |
            ((::std::mem::size_of::<Mtx>() as
                  libc::c_ulong).wrapping_sub(1 as libc::c_int as
                                                  libc::c_uint).wrapping_div(8
                                                                                 as
                                                                                 libc::c_int
                                                                                 as
                                                                                 libc::c_uint)
                 &
                 (((0x1 as libc::c_int) << 5 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 19 as libc::c_int
            |
            ((0 as libc::c_int / 8 as libc::c_int) as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 8 as libc::c_int |
            (((0 as libc::c_int | 0 as libc::c_int | 0 as libc::c_int) ^
                  0x1 as libc::c_int) as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 0 as libc::c_int;
    (*_g_6).words.w1 =
        &mut *modelView.offset(1 as libc::c_int as isize) as *mut Mtx as
            libc::c_uint;
    let fresh8 = gfx;
    gfx = gfx.offset(1);
    let mut _g_7: *mut Gfx = fresh8;
    (*_g_7).words.w0 =
        (0xda as libc::c_int as u32_0 &
             (((0x1 as libc::c_int) << 8 as libc::c_int) - 1 as libc::c_int)
                 as libc::c_uint) << 24 as libc::c_int |
            ((::std::mem::size_of::<Mtx>() as
                  libc::c_ulong).wrapping_sub(1 as libc::c_int as
                                                  libc::c_uint).wrapping_div(8
                                                                                 as
                                                                                 libc::c_int
                                                                                 as
                                                                                 libc::c_uint)
                 &
                 (((0x1 as libc::c_int) << 5 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 19 as libc::c_int
            |
            ((0 as libc::c_int / 8 as libc::c_int) as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 8 as libc::c_int |
            (((0 as libc::c_int | 0 as libc::c_int | 0 as libc::c_int) ^
                  0x1 as libc::c_int) as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 0 as libc::c_int;
    (*_g_7).words.w1 =
        &mut *modelView.offset(2 as libc::c_int as isize) as *mut Mtx as
            libc::c_uint;
    let fresh9 = gfx;
    gfx = gfx.offset(1);
    let mut _g_8: *mut Gfx = fresh9;
    (*_g_8).words.w0 =
        (0xde as libc::c_int as u32_0 &
             (((0x1 as libc::c_int) << 8 as libc::c_int) - 1 as libc::c_int)
                 as libc::c_uint) << 24 as libc::c_int |
            (0 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 16 as libc::c_int
            |
            (0 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 16 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 0 as libc::c_int;
    (*_g_8).words.w1 = sWipeDList.as_mut_ptr() as libc::c_uint;
    let fresh10 = gfx;
    gfx = gfx.offset(1);
    let mut _g_9: *mut Gfx = fresh10;
    (*_g_9).words.w0 =
        (0xe7 as libc::c_int as u32_0 &
             (((0x1 as libc::c_int) << 8 as libc::c_int) - 1 as libc::c_int)
                 as libc::c_uint) << 24 as libc::c_int;
    (*_g_9).words.w1 = 0 as libc::c_int as libc::c_uint;
    *gfxP = gfx;
}
#[no_mangle]
pub unsafe extern "C" fn TransitionWipe_IsDone(mut thisx: *mut libc::c_void)
 -> s32 {
    let mut this: *mut TransitionWipe = thisx as *mut TransitionWipe;
    return (*this).isDone as s32;
}
#[no_mangle]
pub unsafe extern "C" fn TransitionWipe_SetType(mut thisx: *mut libc::c_void,
                                                mut type_0: s32) {
    let mut this: *mut TransitionWipe = thisx as *mut TransitionWipe;
    if type_0 == 1 as libc::c_int {
        (*this).direction = 1 as libc::c_int as u8_0
    } else { (*this).direction = 0 as libc::c_int as u8_0 }
    if (*this).direction as libc::c_int != 0 as libc::c_int {
        (*this).texY = 0x14d as libc::c_int as u16_0
    } else { (*this).texY = 0x264 as libc::c_int as u16_0 };
}
#[no_mangle]
pub unsafe extern "C" fn TransitionWipe_SetColor(mut thisx: *mut libc::c_void,
                                                 mut color: u32_0) {
    let mut this: *mut TransitionWipe = thisx as *mut TransitionWipe;
    (*this).color.rgba = color;
}
#[no_mangle]
pub unsafe extern "C" fn TransitionWipe_SetEnvColor(mut thisx:
                                                        *mut libc::c_void,
                                                    mut color: u32_0) {
    let mut this: *mut TransitionWipe = thisx as *mut TransitionWipe;
    (*this).envColor.rgba = color;
}
unsafe extern "C" fn run_static_initializers() {
    sWipeDList =
        [Gfx{words:
                 {
                     let mut init =
                         Gwords{w0:
                                    (0xe7 as libc::c_int as u32_0 &
                                         (((0x1 as libc::c_int) <<
                                               8 as libc::c_int) -
                                              1 as libc::c_int) as
                                             libc::c_uint) <<
                                        24 as libc::c_int,
                                w1: 0 as libc::c_int as libc::c_uint,};
                     init
                 },},
         Gfx{words:
                 {
                     let mut init =
                         Gwords{w0:
                                    (0xd9 as libc::c_int as u32_0 &
                                         (((0x1 as libc::c_int) <<
                                               8 as libc::c_int) -
                                              1 as libc::c_int) as
                                             libc::c_uint) <<
                                        24 as libc::c_int |
                                        (!((0x1 as libc::c_int |
                                                0x4 as libc::c_int |
                                                0x600 as libc::c_int |
                                                0x10000 as libc::c_int |
                                                0x20000 as libc::c_int |
                                                0x40000 as libc::c_int |
                                                0x80000 as libc::c_int |
                                                0x100000 as libc::c_int |
                                                0x200000 as libc::c_int) as
                                               u32_0) &
                                             (((0x1 as libc::c_int) <<
                                                   24 as libc::c_int) -
                                                  1 as libc::c_int) as
                                                 libc::c_uint) <<
                                            0 as libc::c_int,
                                w1: 0 as libc::c_int as u32_0,};
                     init
                 },},
         Gfx{words:
                 {
                     let mut init =
                         Gwords{w0:
                                    (0xd9 as libc::c_int as u32_0 &
                                         (((0x1 as libc::c_int) <<
                                               8 as libc::c_int) -
                                              1 as libc::c_int) as
                                             libc::c_uint) <<
                                        24 as libc::c_int |
                                        (!(0 as libc::c_int as u32_0) &
                                             (((0x1 as libc::c_int) <<
                                                   24 as libc::c_int) -
                                                  1 as libc::c_int) as
                                                 libc::c_uint) <<
                                            0 as libc::c_int,
                                w1:
                                    (0x1 as libc::c_int | 0x4 as libc::c_int |
                                         0x200000 as libc::c_int) as u32_0,};
                     init
                 },},
         Gfx{words:
                 {
                     let mut init =
                         Gwords{w0:
                                    (0xef as libc::c_int as u32_0 &
                                         (((0x1 as libc::c_int) <<
                                               8 as libc::c_int) -
                                              1 as libc::c_int) as
                                             libc::c_uint) <<
                                        24 as libc::c_int |
                                        (((3 as libc::c_int) <<
                                              4 as libc::c_int |
                                              (0 as libc::c_int) <<
                                                  6 as libc::c_int |
                                              (0 as libc::c_int) <<
                                                  8 as libc::c_int |
                                              (6 as libc::c_int) <<
                                                  9 as libc::c_int |
                                              (2 as libc::c_int) <<
                                                  12 as libc::c_int |
                                              (0 as libc::c_int) <<
                                                  14 as libc::c_int |
                                              (0 as libc::c_int) <<
                                                  16 as libc::c_int |
                                              (0 as libc::c_int) <<
                                                  17 as libc::c_int |
                                              (1 as libc::c_int) <<
                                                  19 as libc::c_int |
                                              (1 as libc::c_int) <<
                                                  20 as libc::c_int |
                                              (1 as libc::c_int) <<
                                                  23 as libc::c_int) as u32_0
                                             &
                                             (((0x1 as libc::c_int) <<
                                                   24 as libc::c_int) -
                                                  1 as libc::c_int) as
                                                 libc::c_uint) <<
                                            0 as libc::c_int,
                                w1:
                                    ((0 as libc::c_int) << 0 as libc::c_int |
                                         (1 as libc::c_int) <<
                                             2 as libc::c_int |
                                         (0 as libc::c_int) <<
                                             30 as libc::c_int |
                                         (3 as libc::c_int) <<
                                             26 as libc::c_int |
                                         (0 as libc::c_int) <<
                                             22 as libc::c_int |
                                         (2 as libc::c_int) <<
                                             18 as libc::c_int |
                                         0x8 as libc::c_int |
                                         0x10 as libc::c_int |
                                         0x20 as libc::c_int |
                                         0x40 as libc::c_int |
                                         0 as libc::c_int |
                                         0x1000 as libc::c_int |
                                         0x2000 as libc::c_int |
                                         0 as libc::c_int | 0 as libc::c_int |
                                         (0 as libc::c_int) <<
                                             28 as libc::c_int |
                                         (0 as libc::c_int) <<
                                             24 as libc::c_int |
                                         (1 as libc::c_int) <<
                                             20 as libc::c_int |
                                         (1 as libc::c_int) <<
                                             16 as libc::c_int) as
                                        libc::c_uint,};
                     init
                 },},
         Gfx{words:
                 {
                     let mut init =
                         Gwords{w0:
                                    (0xfc as libc::c_int as u32_0 &
                                         (((0x1 as libc::c_int) <<
                                               8 as libc::c_int) -
                                              1 as libc::c_int) as
                                             libc::c_uint) <<
                                        24 as libc::c_int |
                                        (((2 as libc::c_int as u32_0 &
                                               (((0x1 as libc::c_int) <<
                                                     4 as libc::c_int) -
                                                    1 as libc::c_int) as
                                                   libc::c_uint) <<
                                              20 as libc::c_int |
                                              (14 as libc::c_int as u32_0 &
                                                   (((0x1 as libc::c_int) <<
                                                         5 as libc::c_int) -
                                                        1 as libc::c_int) as
                                                       libc::c_uint) <<
                                                  15 as libc::c_int |
                                              (2 as libc::c_int as u32_0 &
                                                   (((0x1 as libc::c_int) <<
                                                         3 as libc::c_int) -
                                                        1 as libc::c_int) as
                                                       libc::c_uint) <<
                                                  12 as libc::c_int |
                                              (6 as libc::c_int as u32_0 &
                                                   (((0x1 as libc::c_int) <<
                                                         3 as libc::c_int) -
                                                        1 as libc::c_int) as
                                                       libc::c_uint) <<
                                                  9 as libc::c_int |
                                              ((0 as libc::c_int as u32_0 &
                                                    (((0x1 as libc::c_int) <<
                                                          4 as libc::c_int) -
                                                         1 as libc::c_int) as
                                                        libc::c_uint) <<
                                                   5 as libc::c_int |
                                                   (3 as libc::c_int as u32_0
                                                        &
                                                        (((0x1 as libc::c_int)
                                                              <<
                                                              5 as
                                                                  libc::c_int)
                                                             -
                                                             1 as libc::c_int)
                                                            as libc::c_uint)
                                                       << 0 as libc::c_int)) &
                                             (((0x1 as libc::c_int) <<
                                                   24 as libc::c_int) -
                                                  1 as libc::c_int) as
                                                 libc::c_uint) <<
                                            0 as libc::c_int,
                                w1:
                                    (1 as libc::c_int as u32_0 &
                                         (((0x1 as libc::c_int) <<
                                               4 as libc::c_int) -
                                              1 as libc::c_int) as
                                             libc::c_uint) <<
                                        28 as libc::c_int |
                                        (1 as libc::c_int as u32_0 &
                                             (((0x1 as libc::c_int) <<
                                                   3 as libc::c_int) -
                                                  1 as libc::c_int) as
                                                 libc::c_uint) <<
                                            15 as libc::c_int |
                                        (1 as libc::c_int as u32_0 &
                                             (((0x1 as libc::c_int) <<
                                                   3 as libc::c_int) -
                                                  1 as libc::c_int) as
                                                 libc::c_uint) <<
                                            12 as libc::c_int |
                                        (1 as libc::c_int as u32_0 &
                                             (((0x1 as libc::c_int) <<
                                                   3 as libc::c_int) -
                                                  1 as libc::c_int) as
                                                 libc::c_uint) <<
                                            9 as libc::c_int |
                                        ((31 as libc::c_int as u32_0 &
                                              (((0x1 as libc::c_int) <<
                                                    4 as libc::c_int) -
                                                   1 as libc::c_int) as
                                                  libc::c_uint) <<
                                             24 as libc::c_int |
                                             (0 as libc::c_int as u32_0 &
                                                  (((0x1 as libc::c_int) <<
                                                        3 as libc::c_int) -
                                                       1 as libc::c_int) as
                                                      libc::c_uint) <<
                                                 21 as libc::c_int |
                                             (3 as libc::c_int as u32_0 &
                                                  (((0x1 as libc::c_int) <<
                                                        3 as libc::c_int) -
                                                       1 as libc::c_int) as
                                                      libc::c_uint) <<
                                                 18 as libc::c_int |
                                             (31 as libc::c_int as u32_0 &
                                                  (((0x1 as libc::c_int) <<
                                                        3 as libc::c_int) -
                                                       1 as libc::c_int) as
                                                      libc::c_uint) <<
                                                 6 as libc::c_int |
                                             (7 as libc::c_int as u32_0 &
                                                  (((0x1 as libc::c_int) <<
                                                        3 as libc::c_int) -
                                                       1 as libc::c_int) as
                                                      libc::c_uint) <<
                                                 3 as libc::c_int |
                                             (7 as libc::c_int as u32_0 &
                                                  (((0x1 as libc::c_int) <<
                                                        3 as libc::c_int) -
                                                       1 as libc::c_int) as
                                                      libc::c_uint) <<
                                                 0 as libc::c_int),};
                     init
                 },},
         Gfx{words:
                 {
                     let mut init =
                         Gwords{w0:
                                    (0xee as libc::c_int as u32_0 &
                                         (((0x1 as libc::c_int) <<
                                               8 as libc::c_int) -
                                              1 as libc::c_int) as
                                             libc::c_uint) <<
                                        24 as libc::c_int,
                                w1:
                                    (0 as libc::c_int as u32_0 &
                                         (((0x1 as libc::c_int) <<
                                               16 as libc::c_int) -
                                              1 as libc::c_int) as
                                             libc::c_uint) <<
                                        16 as libc::c_int |
                                        (0 as libc::c_int as u32_0 &
                                             (((0x1 as libc::c_int) <<
                                                   16 as libc::c_int) -
                                                  1 as libc::c_int) as
                                                 libc::c_uint) <<
                                            0 as libc::c_int,};
                     init
                 },},
         Gfx{words:
                 {
                     let mut init =
                         Gwords{w0:
                                    (0xfd as libc::c_int as u32_0 &
                                         (((0x1 as libc::c_int) <<
                                               8 as libc::c_int) -
                                              1 as libc::c_int) as
                                             libc::c_uint) <<
                                        24 as libc::c_int |
                                        (4 as libc::c_int as u32_0 &
                                             (((0x1 as libc::c_int) <<
                                                   3 as libc::c_int) -
                                                  1 as libc::c_int) as
                                                 libc::c_uint) <<
                                            21 as libc::c_int |
                                        (2 as libc::c_int as u32_0 &
                                             (((0x1 as libc::c_int) <<
                                                   2 as libc::c_int) -
                                                  1 as libc::c_int) as
                                                 libc::c_uint) <<
                                            19 as libc::c_int |
                                        ((1 as libc::c_int - 1 as libc::c_int)
                                             as u32_0 &
                                             (((0x1 as libc::c_int) <<
                                                   12 as libc::c_int) -
                                                  1 as libc::c_int) as
                                                 libc::c_uint) <<
                                            0 as libc::c_int,
                                w1: sWipe1Tex.as_mut_ptr() as libc::c_uint,};
                     init
                 },},
         Gfx{words:
                 {
                     let mut init =
                         Gwords{w0:
                                    (0xf5 as libc::c_int as u32_0 &
                                         (((0x1 as libc::c_int) <<
                                               8 as libc::c_int) -
                                              1 as libc::c_int) as
                                             libc::c_uint) <<
                                        24 as libc::c_int |
                                        (4 as libc::c_int as u32_0 &
                                             (((0x1 as libc::c_int) <<
                                                   3 as libc::c_int) -
                                                  1 as libc::c_int) as
                                                 libc::c_uint) <<
                                            21 as libc::c_int |
                                        (2 as libc::c_int as u32_0 &
                                             (((0x1 as libc::c_int) <<
                                                   2 as libc::c_int) -
                                                  1 as libc::c_int) as
                                                 libc::c_uint) <<
                                            19 as libc::c_int |
                                        (0 as libc::c_int as u32_0 &
                                             (((0x1 as libc::c_int) <<
                                                   9 as libc::c_int) -
                                                  1 as libc::c_int) as
                                                 libc::c_uint) <<
                                            9 as libc::c_int |
                                        (0 as libc::c_int as u32_0 &
                                             (((0x1 as libc::c_int) <<
                                                   9 as libc::c_int) -
                                                  1 as libc::c_int) as
                                                 libc::c_uint) <<
                                            0 as libc::c_int,
                                w1:
                                    (7 as libc::c_int as u32_0 &
                                         (((0x1 as libc::c_int) <<
                                               3 as libc::c_int) -
                                              1 as libc::c_int) as
                                             libc::c_uint) <<
                                        24 as libc::c_int |
                                        (0 as libc::c_int as u32_0 &
                                             (((0x1 as libc::c_int) <<
                                                   4 as libc::c_int) -
                                                  1 as libc::c_int) as
                                                 libc::c_uint) <<
                                            20 as libc::c_int |
                                        ((0x1 as libc::c_int |
                                              0 as libc::c_int) as u32_0 &
                                             (((0x1 as libc::c_int) <<
                                                   2 as libc::c_int) -
                                                  1 as libc::c_int) as
                                                 libc::c_uint) <<
                                            18 as libc::c_int |
                                        (6 as libc::c_int as u32_0 &
                                             (((0x1 as libc::c_int) <<
                                                   4 as libc::c_int) -
                                                  1 as libc::c_int) as
                                                 libc::c_uint) <<
                                            14 as libc::c_int |
                                        (0 as libc::c_int as u32_0 &
                                             (((0x1 as libc::c_int) <<
                                                   4 as libc::c_int) -
                                                  1 as libc::c_int) as
                                                 libc::c_uint) <<
                                            10 as libc::c_int |
                                        ((0 as libc::c_int | 0 as libc::c_int)
                                             as u32_0 &
                                             (((0x1 as libc::c_int) <<
                                                   2 as libc::c_int) -
                                                  1 as libc::c_int) as
                                                 libc::c_uint) <<
                                            8 as libc::c_int |
                                        (6 as libc::c_int as u32_0 &
                                             (((0x1 as libc::c_int) <<
                                                   4 as libc::c_int) -
                                                  1 as libc::c_int) as
                                                 libc::c_uint) <<
                                            4 as libc::c_int |
                                        (11 as libc::c_int as u32_0 &
                                             (((0x1 as libc::c_int) <<
                                                   4 as libc::c_int) -
                                                  1 as libc::c_int) as
                                                 libc::c_uint) <<
                                            0 as libc::c_int,};
                     init
                 },},
         Gfx{words:
                 {
                     let mut init =
                         Gwords{w0:
                                    (0xe6 as libc::c_int as u32_0 &
                                         (((0x1 as libc::c_int) <<
                                               8 as libc::c_int) -
                                              1 as libc::c_int) as
                                             libc::c_uint) <<
                                        24 as libc::c_int,
                                w1: 0 as libc::c_int as libc::c_uint,};
                     init
                 },},
         Gfx{words:
                 {
                     let mut init =
                         Gwords{w0:
                                    (0xf3 as libc::c_int as u32_0 &
                                         (((0x1 as libc::c_int) <<
                                               8 as libc::c_int) -
                                              1 as libc::c_int) as
                                             libc::c_uint) <<
                                        24 as libc::c_int |
                                        (0 as libc::c_int as u32_0 &
                                             (((0x1 as libc::c_int) <<
                                                   12 as libc::c_int) -
                                                  1 as libc::c_int) as
                                                 libc::c_uint) <<
                                            12 as libc::c_int |
                                        (0 as libc::c_int as u32_0 &
                                             (((0x1 as libc::c_int) <<
                                                   12 as libc::c_int) -
                                                  1 as libc::c_int) as
                                                 libc::c_uint) <<
                                            0 as libc::c_int,
                                w1:
                                    (7 as libc::c_int as u32_0 &
                                         (((0x1 as libc::c_int) <<
                                               3 as libc::c_int) -
                                              1 as libc::c_int) as
                                             libc::c_uint) <<
                                        24 as libc::c_int |
                                        ((if ((64 as libc::c_int *
                                                   64 as libc::c_int +
                                                   3 as libc::c_int >>
                                                   2 as libc::c_int) -
                                                  1 as libc::c_int) <
                                                 2047 as libc::c_int {
                                              (64 as libc::c_int *
                                                   64 as libc::c_int +
                                                   3 as libc::c_int >>
                                                   2 as libc::c_int) -
                                                  1 as libc::c_int
                                          } else { 2047 as libc::c_int }) as
                                             u32_0 &
                                             (((0x1 as libc::c_int) <<
                                                   12 as libc::c_int) -
                                                  1 as libc::c_int) as
                                                 libc::c_uint) <<
                                            12 as libc::c_int |
                                        (((((1 as libc::c_int) <<
                                                11 as libc::c_int) +
                                               (if 1 as libc::c_int >
                                                       64 as libc::c_int /
                                                           16 as libc::c_int {
                                                    1 as libc::c_int
                                                } else {
                                                    (64 as libc::c_int) /
                                                        16 as libc::c_int
                                                }) - 1 as libc::c_int) /
                                              (if 1 as libc::c_int >
                                                      64 as libc::c_int /
                                                          16 as libc::c_int {
                                                   1 as libc::c_int
                                               } else {
                                                   (64 as libc::c_int) /
                                                       16 as libc::c_int
                                               })) as u32_0 &
                                             (((0x1 as libc::c_int) <<
                                                   12 as libc::c_int) -
                                                  1 as libc::c_int) as
                                                 libc::c_uint) <<
                                            0 as libc::c_int,};
                     init
                 },},
         Gfx{words:
                 {
                     let mut init =
                         Gwords{w0:
                                    (0xe7 as libc::c_int as u32_0 &
                                         (((0x1 as libc::c_int) <<
                                               8 as libc::c_int) -
                                              1 as libc::c_int) as
                                             libc::c_uint) <<
                                        24 as libc::c_int,
                                w1: 0 as libc::c_int as libc::c_uint,};
                     init
                 },},
         Gfx{words:
                 {
                     let mut init =
                         Gwords{w0:
                                    (0xf5 as libc::c_int as u32_0 &
                                         (((0x1 as libc::c_int) <<
                                               8 as libc::c_int) -
                                              1 as libc::c_int) as
                                             libc::c_uint) <<
                                        24 as libc::c_int |
                                        (4 as libc::c_int as u32_0 &
                                             (((0x1 as libc::c_int) <<
                                                   3 as libc::c_int) -
                                                  1 as libc::c_int) as
                                                 libc::c_uint) <<
                                            21 as libc::c_int |
                                        (0 as libc::c_int as u32_0 &
                                             (((0x1 as libc::c_int) <<
                                                   2 as libc::c_int) -
                                                  1 as libc::c_int) as
                                                 libc::c_uint) <<
                                            19 as libc::c_int |
                                        (((64 as libc::c_int >>
                                               1 as libc::c_int) +
                                              7 as libc::c_int >>
                                              3 as libc::c_int) as u32_0 &
                                             (((0x1 as libc::c_int) <<
                                                   9 as libc::c_int) -
                                                  1 as libc::c_int) as
                                                 libc::c_uint) <<
                                            9 as libc::c_int |
                                        (0 as libc::c_int as u32_0 &
                                             (((0x1 as libc::c_int) <<
                                                   9 as libc::c_int) -
                                                  1 as libc::c_int) as
                                                 libc::c_uint) <<
                                            0 as libc::c_int,
                                w1:
                                    (0 as libc::c_int as u32_0 &
                                         (((0x1 as libc::c_int) <<
                                               3 as libc::c_int) -
                                              1 as libc::c_int) as
                                             libc::c_uint) <<
                                        24 as libc::c_int |
                                        (0 as libc::c_int as u32_0 &
                                             (((0x1 as libc::c_int) <<
                                                   4 as libc::c_int) -
                                                  1 as libc::c_int) as
                                                 libc::c_uint) <<
                                            20 as libc::c_int |
                                        ((0x1 as libc::c_int |
                                              0 as libc::c_int) as u32_0 &
                                             (((0x1 as libc::c_int) <<
                                                   2 as libc::c_int) -
                                                  1 as libc::c_int) as
                                                 libc::c_uint) <<
                                            18 as libc::c_int |
                                        (6 as libc::c_int as u32_0 &
                                             (((0x1 as libc::c_int) <<
                                                   4 as libc::c_int) -
                                                  1 as libc::c_int) as
                                                 libc::c_uint) <<
                                            14 as libc::c_int |
                                        (0 as libc::c_int as u32_0 &
                                             (((0x1 as libc::c_int) <<
                                                   4 as libc::c_int) -
                                                  1 as libc::c_int) as
                                                 libc::c_uint) <<
                                            10 as libc::c_int |
                                        ((0 as libc::c_int | 0 as libc::c_int)
                                             as u32_0 &
                                             (((0x1 as libc::c_int) <<
                                                   2 as libc::c_int) -
                                                  1 as libc::c_int) as
                                                 libc::c_uint) <<
                                            8 as libc::c_int |
                                        (6 as libc::c_int as u32_0 &
                                             (((0x1 as libc::c_int) <<
                                                   4 as libc::c_int) -
                                                  1 as libc::c_int) as
                                                 libc::c_uint) <<
                                            4 as libc::c_int |
                                        (11 as libc::c_int as u32_0 &
                                             (((0x1 as libc::c_int) <<
                                                   4 as libc::c_int) -
                                                  1 as libc::c_int) as
                                                 libc::c_uint) <<
                                            0 as libc::c_int,};
                     init
                 },},
         Gfx{words:
                 {
                     let mut init =
                         Gwords{w0:
                                    (0xf2 as libc::c_int as u32_0 &
                                         (((0x1 as libc::c_int) <<
                                               8 as libc::c_int) -
                                              1 as libc::c_int) as
                                             libc::c_uint) <<
                                        24 as libc::c_int |
                                        (0 as libc::c_int as u32_0 &
                                             (((0x1 as libc::c_int) <<
                                                   12 as libc::c_int) -
                                                  1 as libc::c_int) as
                                                 libc::c_uint) <<
                                            12 as libc::c_int |
                                        (0 as libc::c_int as u32_0 &
                                             (((0x1 as libc::c_int) <<
                                                   12 as libc::c_int) -
                                                  1 as libc::c_int) as
                                                 libc::c_uint) <<
                                            0 as libc::c_int,
                                w1:
                                    (0 as libc::c_int as u32_0 &
                                         (((0x1 as libc::c_int) <<
                                               3 as libc::c_int) -
                                              1 as libc::c_int) as
                                             libc::c_uint) <<
                                        24 as libc::c_int |
                                        (((64 as libc::c_int -
                                               1 as libc::c_int) <<
                                              2 as libc::c_int) as u32_0 &
                                             (((0x1 as libc::c_int) <<
                                                   12 as libc::c_int) -
                                                  1 as libc::c_int) as
                                                 libc::c_uint) <<
                                            12 as libc::c_int |
                                        (((64 as libc::c_int -
                                               1 as libc::c_int) <<
                                              2 as libc::c_int) as u32_0 &
                                             (((0x1 as libc::c_int) <<
                                                   12 as libc::c_int) -
                                                  1 as libc::c_int) as
                                                 libc::c_uint) <<
                                            0 as libc::c_int,};
                     init
                 },},
         Gfx{words:
                 {
                     let mut init =
                         Gwords{w0:
                                    (0xfd as libc::c_int as u32_0 &
                                         (((0x1 as libc::c_int) <<
                                               8 as libc::c_int) -
                                              1 as libc::c_int) as
                                             libc::c_uint) <<
                                        24 as libc::c_int |
                                        (4 as libc::c_int as u32_0 &
                                             (((0x1 as libc::c_int) <<
                                                   3 as libc::c_int) -
                                                  1 as libc::c_int) as
                                                 libc::c_uint) <<
                                            21 as libc::c_int |
                                        (2 as libc::c_int as u32_0 &
                                             (((0x1 as libc::c_int) <<
                                                   2 as libc::c_int) -
                                                  1 as libc::c_int) as
                                                 libc::c_uint) <<
                                            19 as libc::c_int |
                                        ((1 as libc::c_int - 1 as libc::c_int)
                                             as u32_0 &
                                             (((0x1 as libc::c_int) <<
                                                   12 as libc::c_int) -
                                                  1 as libc::c_int) as
                                                 libc::c_uint) <<
                                            0 as libc::c_int,
                                w1: sWipe1Tex.as_mut_ptr() as libc::c_uint,};
                     init
                 },},
         Gfx{words:
                 {
                     let mut init =
                         Gwords{w0:
                                    (0xf5 as libc::c_int as u32_0 &
                                         (((0x1 as libc::c_int) <<
                                               8 as libc::c_int) -
                                              1 as libc::c_int) as
                                             libc::c_uint) <<
                                        24 as libc::c_int |
                                        (4 as libc::c_int as u32_0 &
                                             (((0x1 as libc::c_int) <<
                                                   3 as libc::c_int) -
                                                  1 as libc::c_int) as
                                                 libc::c_uint) <<
                                            21 as libc::c_int |
                                        (2 as libc::c_int as u32_0 &
                                             (((0x1 as libc::c_int) <<
                                                   2 as libc::c_int) -
                                                  1 as libc::c_int) as
                                                 libc::c_uint) <<
                                            19 as libc::c_int |
                                        (0 as libc::c_int as u32_0 &
                                             (((0x1 as libc::c_int) <<
                                                   9 as libc::c_int) -
                                                  1 as libc::c_int) as
                                                 libc::c_uint) <<
                                            9 as libc::c_int |
                                        (0x100 as libc::c_int as u32_0 &
                                             (((0x1 as libc::c_int) <<
                                                   9 as libc::c_int) -
                                                  1 as libc::c_int) as
                                                 libc::c_uint) <<
                                            0 as libc::c_int,
                                w1:
                                    (7 as libc::c_int as u32_0 &
                                         (((0x1 as libc::c_int) <<
                                               3 as libc::c_int) -
                                              1 as libc::c_int) as
                                             libc::c_uint) <<
                                        24 as libc::c_int |
                                        (0 as libc::c_int as u32_0 &
                                             (((0x1 as libc::c_int) <<
                                                   4 as libc::c_int) -
                                                  1 as libc::c_int) as
                                                 libc::c_uint) <<
                                            20 as libc::c_int |
                                        ((0x1 as libc::c_int |
                                              0 as libc::c_int) as u32_0 &
                                             (((0x1 as libc::c_int) <<
                                                   2 as libc::c_int) -
                                                  1 as libc::c_int) as
                                                 libc::c_uint) <<
                                            18 as libc::c_int |
                                        (6 as libc::c_int as u32_0 &
                                             (((0x1 as libc::c_int) <<
                                                   4 as libc::c_int) -
                                                  1 as libc::c_int) as
                                                 libc::c_uint) <<
                                            14 as libc::c_int |
                                        (1 as libc::c_int as u32_0 &
                                             (((0x1 as libc::c_int) <<
                                                   4 as libc::c_int) -
                                                  1 as libc::c_int) as
                                                 libc::c_uint) <<
                                            10 as libc::c_int |
                                        ((0 as libc::c_int | 0 as libc::c_int)
                                             as u32_0 &
                                             (((0x1 as libc::c_int) <<
                                                   2 as libc::c_int) -
                                                  1 as libc::c_int) as
                                                 libc::c_uint) <<
                                            8 as libc::c_int |
                                        (6 as libc::c_int as u32_0 &
                                             (((0x1 as libc::c_int) <<
                                                   4 as libc::c_int) -
                                                  1 as libc::c_int) as
                                                 libc::c_uint) <<
                                            4 as libc::c_int |
                                        (11 as libc::c_int as u32_0 &
                                             (((0x1 as libc::c_int) <<
                                                   4 as libc::c_int) -
                                                  1 as libc::c_int) as
                                                 libc::c_uint) <<
                                            0 as libc::c_int,};
                     init
                 },},
         Gfx{words:
                 {
                     let mut init =
                         Gwords{w0:
                                    (0xe6 as libc::c_int as u32_0 &
                                         (((0x1 as libc::c_int) <<
                                               8 as libc::c_int) -
                                              1 as libc::c_int) as
                                             libc::c_uint) <<
                                        24 as libc::c_int,
                                w1: 0 as libc::c_int as libc::c_uint,};
                     init
                 },},
         Gfx{words:
                 {
                     let mut init =
                         Gwords{w0:
                                    (0xf3 as libc::c_int as u32_0 &
                                         (((0x1 as libc::c_int) <<
                                               8 as libc::c_int) -
                                              1 as libc::c_int) as
                                             libc::c_uint) <<
                                        24 as libc::c_int |
                                        (0 as libc::c_int as u32_0 &
                                             (((0x1 as libc::c_int) <<
                                                   12 as libc::c_int) -
                                                  1 as libc::c_int) as
                                                 libc::c_uint) <<
                                            12 as libc::c_int |
                                        (0 as libc::c_int as u32_0 &
                                             (((0x1 as libc::c_int) <<
                                                   12 as libc::c_int) -
                                                  1 as libc::c_int) as
                                                 libc::c_uint) <<
                                            0 as libc::c_int,
                                w1:
                                    (7 as libc::c_int as u32_0 &
                                         (((0x1 as libc::c_int) <<
                                               3 as libc::c_int) -
                                              1 as libc::c_int) as
                                             libc::c_uint) <<
                                        24 as libc::c_int |
                                        ((if ((64 as libc::c_int *
                                                   64 as libc::c_int +
                                                   3 as libc::c_int >>
                                                   2 as libc::c_int) -
                                                  1 as libc::c_int) <
                                                 2047 as libc::c_int {
                                              (64 as libc::c_int *
                                                   64 as libc::c_int +
                                                   3 as libc::c_int >>
                                                   2 as libc::c_int) -
                                                  1 as libc::c_int
                                          } else { 2047 as libc::c_int }) as
                                             u32_0 &
                                             (((0x1 as libc::c_int) <<
                                                   12 as libc::c_int) -
                                                  1 as libc::c_int) as
                                                 libc::c_uint) <<
                                            12 as libc::c_int |
                                        (((((1 as libc::c_int) <<
                                                11 as libc::c_int) +
                                               (if 1 as libc::c_int >
                                                       64 as libc::c_int /
                                                           16 as libc::c_int {
                                                    1 as libc::c_int
                                                } else {
                                                    (64 as libc::c_int) /
                                                        16 as libc::c_int
                                                }) - 1 as libc::c_int) /
                                              (if 1 as libc::c_int >
                                                      64 as libc::c_int /
                                                          16 as libc::c_int {
                                                   1 as libc::c_int
                                               } else {
                                                   (64 as libc::c_int) /
                                                       16 as libc::c_int
                                               })) as u32_0 &
                                             (((0x1 as libc::c_int) <<
                                                   12 as libc::c_int) -
                                                  1 as libc::c_int) as
                                                 libc::c_uint) <<
                                            0 as libc::c_int,};
                     init
                 },},
         Gfx{words:
                 {
                     let mut init =
                         Gwords{w0:
                                    (0xe7 as libc::c_int as u32_0 &
                                         (((0x1 as libc::c_int) <<
                                               8 as libc::c_int) -
                                              1 as libc::c_int) as
                                             libc::c_uint) <<
                                        24 as libc::c_int,
                                w1: 0 as libc::c_int as libc::c_uint,};
                     init
                 },},
         Gfx{words:
                 {
                     let mut init =
                         Gwords{w0:
                                    (0xf5 as libc::c_int as u32_0 &
                                         (((0x1 as libc::c_int) <<
                                               8 as libc::c_int) -
                                              1 as libc::c_int) as
                                             libc::c_uint) <<
                                        24 as libc::c_int |
                                        (4 as libc::c_int as u32_0 &
                                             (((0x1 as libc::c_int) <<
                                                   3 as libc::c_int) -
                                                  1 as libc::c_int) as
                                                 libc::c_uint) <<
                                            21 as libc::c_int |
                                        (0 as libc::c_int as u32_0 &
                                             (((0x1 as libc::c_int) <<
                                                   2 as libc::c_int) -
                                                  1 as libc::c_int) as
                                                 libc::c_uint) <<
                                            19 as libc::c_int |
                                        (((64 as libc::c_int >>
                                               1 as libc::c_int) +
                                              7 as libc::c_int >>
                                              3 as libc::c_int) as u32_0 &
                                             (((0x1 as libc::c_int) <<
                                                   9 as libc::c_int) -
                                                  1 as libc::c_int) as
                                                 libc::c_uint) <<
                                            9 as libc::c_int |
                                        (0x100 as libc::c_int as u32_0 &
                                             (((0x1 as libc::c_int) <<
                                                   9 as libc::c_int) -
                                                  1 as libc::c_int) as
                                                 libc::c_uint) <<
                                            0 as libc::c_int,
                                w1:
                                    (1 as libc::c_int as u32_0 &
                                         (((0x1 as libc::c_int) <<
                                               3 as libc::c_int) -
                                              1 as libc::c_int) as
                                             libc::c_uint) <<
                                        24 as libc::c_int |
                                        (0 as libc::c_int as u32_0 &
                                             (((0x1 as libc::c_int) <<
                                                   4 as libc::c_int) -
                                                  1 as libc::c_int) as
                                                 libc::c_uint) <<
                                            20 as libc::c_int |
                                        ((0x1 as libc::c_int |
                                              0 as libc::c_int) as u32_0 &
                                             (((0x1 as libc::c_int) <<
                                                   2 as libc::c_int) -
                                                  1 as libc::c_int) as
                                                 libc::c_uint) <<
                                            18 as libc::c_int |
                                        (6 as libc::c_int as u32_0 &
                                             (((0x1 as libc::c_int) <<
                                                   4 as libc::c_int) -
                                                  1 as libc::c_int) as
                                                 libc::c_uint) <<
                                            14 as libc::c_int |
                                        (1 as libc::c_int as u32_0 &
                                             (((0x1 as libc::c_int) <<
                                                   4 as libc::c_int) -
                                                  1 as libc::c_int) as
                                                 libc::c_uint) <<
                                            10 as libc::c_int |
                                        ((0 as libc::c_int | 0 as libc::c_int)
                                             as u32_0 &
                                             (((0x1 as libc::c_int) <<
                                                   2 as libc::c_int) -
                                                  1 as libc::c_int) as
                                                 libc::c_uint) <<
                                            8 as libc::c_int |
                                        (6 as libc::c_int as u32_0 &
                                             (((0x1 as libc::c_int) <<
                                                   4 as libc::c_int) -
                                                  1 as libc::c_int) as
                                                 libc::c_uint) <<
                                            4 as libc::c_int |
                                        (11 as libc::c_int as u32_0 &
                                             (((0x1 as libc::c_int) <<
                                                   4 as libc::c_int) -
                                                  1 as libc::c_int) as
                                                 libc::c_uint) <<
                                            0 as libc::c_int,};
                     init
                 },},
         Gfx{words:
                 {
                     let mut init =
                         Gwords{w0:
                                    (0xf2 as libc::c_int as u32_0 &
                                         (((0x1 as libc::c_int) <<
                                               8 as libc::c_int) -
                                              1 as libc::c_int) as
                                             libc::c_uint) <<
                                        24 as libc::c_int |
                                        (0 as libc::c_int as u32_0 &
                                             (((0x1 as libc::c_int) <<
                                                   12 as libc::c_int) -
                                                  1 as libc::c_int) as
                                                 libc::c_uint) <<
                                            12 as libc::c_int |
                                        (0 as libc::c_int as u32_0 &
                                             (((0x1 as libc::c_int) <<
                                                   12 as libc::c_int) -
                                                  1 as libc::c_int) as
                                                 libc::c_uint) <<
                                            0 as libc::c_int,
                                w1:
                                    (1 as libc::c_int as u32_0 &
                                         (((0x1 as libc::c_int) <<
                                               3 as libc::c_int) -
                                              1 as libc::c_int) as
                                             libc::c_uint) <<
                                        24 as libc::c_int |
                                        (((64 as libc::c_int -
                                               1 as libc::c_int) <<
                                              2 as libc::c_int) as u32_0 &
                                             (((0x1 as libc::c_int) <<
                                                   12 as libc::c_int) -
                                                  1 as libc::c_int) as
                                                 libc::c_uint) <<
                                            12 as libc::c_int |
                                        (((64 as libc::c_int -
                                               1 as libc::c_int) <<
                                              2 as libc::c_int) as u32_0 &
                                             (((0x1 as libc::c_int) <<
                                                   12 as libc::c_int) -
                                                  1 as libc::c_int) as
                                                 libc::c_uint) <<
                                            0 as libc::c_int,};
                     init
                 },},
         Gfx{words:
                 {
                     let mut init =
                         Gwords{w0:
                                    (0xe3 as libc::c_int as u32_0 &
                                         (((0x1 as libc::c_int) <<
                                               8 as libc::c_int) -
                                              1 as libc::c_int) as
                                             libc::c_uint) <<
                                        24 as libc::c_int |
                                        ((32 as libc::c_int -
                                              14 as libc::c_int -
                                              2 as libc::c_int) as u32_0 &
                                             (((0x1 as libc::c_int) <<
                                                   8 as libc::c_int) -
                                                  1 as libc::c_int) as
                                                 libc::c_uint) <<
                                            8 as libc::c_int |
                                        ((2 as libc::c_int - 1 as libc::c_int)
                                             as u32_0 &
                                             (((0x1 as libc::c_int) <<
                                                   8 as libc::c_int) -
                                                  1 as libc::c_int) as
                                                 libc::c_uint) <<
                                            0 as libc::c_int,
                                w1:
                                    ((0 as libc::c_int) << 14 as libc::c_int)
                                        as libc::c_uint,};
                     init
                 },},
         Gfx{words:
                 {
                     let mut init =
                         Gwords{w0:
                                    (0xd7 as libc::c_int as u32_0 &
                                         (((0x1 as libc::c_int) <<
                                               8 as libc::c_int) -
                                              1 as libc::c_int) as
                                             libc::c_uint) <<
                                        24 as libc::c_int |
                                        (0 as libc::c_int as u32_0 &
                                             (((0x1 as libc::c_int) <<
                                                   8 as libc::c_int) -
                                                  1 as libc::c_int) as
                                                 libc::c_uint) <<
                                            16 as libc::c_int |
                                        (0 as libc::c_int as u32_0 &
                                             (((0x1 as libc::c_int) <<
                                                   3 as libc::c_int) -
                                                  1 as libc::c_int) as
                                                 libc::c_uint) <<
                                            11 as libc::c_int |
                                        (0 as libc::c_int as u32_0 &
                                             (((0x1 as libc::c_int) <<
                                                   3 as libc::c_int) -
                                                  1 as libc::c_int) as
                                                 libc::c_uint) <<
                                            8 as libc::c_int |
                                        (1 as libc::c_int as u32_0 &
                                             (((0x1 as libc::c_int) <<
                                                   7 as libc::c_int) -
                                                  1 as libc::c_int) as
                                                 libc::c_uint) <<
                                            1 as libc::c_int,
                                w1:
                                    (0xffff as libc::c_int as u32_0 &
                                         (((0x1 as libc::c_int) <<
                                               16 as libc::c_int) -
                                              1 as libc::c_int) as
                                             libc::c_uint) <<
                                        16 as libc::c_int |
                                        (0xffff as libc::c_int as u32_0 &
                                             (((0x1 as libc::c_int) <<
                                                   16 as libc::c_int) -
                                                  1 as libc::c_int) as
                                                 libc::c_uint) <<
                                            0 as libc::c_int,};
                     init
                 },},
         Gfx{words:
                 {
                     let mut init =
                         Gwords{w0:
                                    (0xde as libc::c_int as u32_0 &
                                         (((0x1 as libc::c_int) <<
                                               8 as libc::c_int) -
                                              1 as libc::c_int) as
                                             libc::c_uint) <<
                                        24 as libc::c_int |
                                        (0 as libc::c_int as u32_0 &
                                             (((0x1 as libc::c_int) <<
                                                   8 as libc::c_int) -
                                                  1 as libc::c_int) as
                                                 libc::c_uint) <<
                                            16 as libc::c_int |
                                        (0 as libc::c_int as u32_0 &
                                             (((0x1 as libc::c_int) <<
                                                   16 as libc::c_int) -
                                                  1 as libc::c_int) as
                                                 libc::c_uint) <<
                                            0 as libc::c_int,
                                w1:
                                    0x8000000 as libc::c_int as
                                        libc::c_uint,};
                     init
                 },},
         Gfx{words:
                 {
                     let mut init =
                         Gwords{w0:
                                    (0x1 as libc::c_int as u32_0 &
                                         (((0x1 as libc::c_int) <<
                                               8 as libc::c_int) -
                                              1 as libc::c_int) as
                                             libc::c_uint) <<
                                        24 as libc::c_int |
                                        (25 as libc::c_int as u32_0 &
                                             (((0x1 as libc::c_int) <<
                                                   8 as libc::c_int) -
                                                  1 as libc::c_int) as
                                                 libc::c_uint) <<
                                            12 as libc::c_int |
                                        ((0 as libc::c_int +
                                              25 as libc::c_int) as u32_0 &
                                             (((0x1 as libc::c_int) <<
                                                   7 as libc::c_int) -
                                                  1 as libc::c_int) as
                                                 libc::c_uint) <<
                                            1 as libc::c_int,
                                w1: sWipe1Vtx.as_mut_ptr() as libc::c_uint,};
                     init
                 },},
         Gfx{words:
                 {
                     let mut init =
                         Gwords{w0:
                                    (0x6 as libc::c_int as u32_0 &
                                         (((0x1 as libc::c_int) <<
                                               8 as libc::c_int) -
                                              1 as libc::c_int) as
                                             libc::c_uint) <<
                                        24 as libc::c_int |
                                        (if 0 as libc::c_int ==
                                                0 as libc::c_int {
                                             (((0 as libc::c_int *
                                                    2 as libc::c_int) as u32_0
                                                   &
                                                   (((0x1 as libc::c_int) <<
                                                         8 as libc::c_int) -
                                                        1 as libc::c_int) as
                                                       libc::c_uint) <<
                                                  16 as libc::c_int |
                                                  ((1 as libc::c_int *
                                                        2 as libc::c_int) as
                                                       u32_0 &
                                                       (((0x1 as libc::c_int)
                                                             <<
                                                             8 as libc::c_int)
                                                            -
                                                            1 as libc::c_int)
                                                           as libc::c_uint) <<
                                                      8 as libc::c_int) |
                                                 ((2 as libc::c_int *
                                                       2 as libc::c_int) as
                                                      u32_0 &
                                                      (((0x1 as libc::c_int)
                                                            <<
                                                            8 as libc::c_int)
                                                           - 1 as libc::c_int)
                                                          as libc::c_uint) <<
                                                     0 as libc::c_int
                                         } else {
                                             (if 0 as libc::c_int ==
                                                     1 as libc::c_int {
                                                  (((1 as libc::c_int *
                                                         2 as libc::c_int) as
                                                        u32_0 &
                                                        (((0x1 as libc::c_int)
                                                              <<
                                                              8 as
                                                                  libc::c_int)
                                                             -
                                                             1 as libc::c_int)
                                                            as libc::c_uint)
                                                       << 16 as libc::c_int |
                                                       ((2 as libc::c_int *
                                                             2 as libc::c_int)
                                                            as u32_0 &
                                                            (((0x1 as
                                                                   libc::c_int)
                                                                  <<
                                                                  8 as
                                                                      libc::c_int)
                                                                 -
                                                                 1 as
                                                                     libc::c_int)
                                                                as
                                                                libc::c_uint)
                                                           <<
                                                           8 as libc::c_int) |
                                                      ((0 as libc::c_int *
                                                            2 as libc::c_int)
                                                           as u32_0 &
                                                           (((0x1 as
                                                                  libc::c_int)
                                                                 <<
                                                                 8 as
                                                                     libc::c_int)
                                                                -
                                                                1 as
                                                                    libc::c_int)
                                                               as
                                                               libc::c_uint)
                                                          << 0 as libc::c_int
                                              } else {
                                                  (((2 as libc::c_int *
                                                         2 as libc::c_int) as
                                                        u32_0 &
                                                        (((0x1 as libc::c_int)
                                                              <<
                                                              8 as
                                                                  libc::c_int)
                                                             -
                                                             1 as libc::c_int)
                                                            as libc::c_uint)
                                                       << 16 as libc::c_int |
                                                       ((0 as libc::c_int *
                                                             2 as libc::c_int)
                                                            as u32_0 &
                                                            (((0x1 as
                                                                   libc::c_int)
                                                                  <<
                                                                  8 as
                                                                      libc::c_int)
                                                                 -
                                                                 1 as
                                                                     libc::c_int)
                                                                as
                                                                libc::c_uint)
                                                           <<
                                                           8 as libc::c_int) |
                                                      ((1 as libc::c_int *
                                                            2 as libc::c_int)
                                                           as u32_0 &
                                                           (((0x1 as
                                                                  libc::c_int)
                                                                 <<
                                                                 8 as
                                                                     libc::c_int)
                                                                -
                                                                1 as
                                                                    libc::c_int)
                                                               as
                                                               libc::c_uint)
                                                          << 0 as libc::c_int
                                              })
                                         }),
                                w1:
                                    if 0 as libc::c_int == 0 as libc::c_int {
                                        (((1 as libc::c_int *
                                               2 as libc::c_int) as u32_0 &
                                              (((0x1 as libc::c_int) <<
                                                    8 as libc::c_int) -
                                                   1 as libc::c_int) as
                                                  libc::c_uint) <<
                                             16 as libc::c_int |
                                             ((3 as libc::c_int *
                                                   2 as libc::c_int) as u32_0
                                                  &
                                                  (((0x1 as libc::c_int) <<
                                                        8 as libc::c_int) -
                                                       1 as libc::c_int) as
                                                      libc::c_uint) <<
                                                 8 as libc::c_int) |
                                            ((4 as libc::c_int *
                                                  2 as libc::c_int) as u32_0 &
                                                 (((0x1 as libc::c_int) <<
                                                       8 as libc::c_int) -
                                                      1 as libc::c_int) as
                                                     libc::c_uint) <<
                                                0 as libc::c_int
                                    } else if 0 as libc::c_int ==
                                                  1 as libc::c_int {
                                        (((3 as libc::c_int *
                                               2 as libc::c_int) as u32_0 &
                                              (((0x1 as libc::c_int) <<
                                                    8 as libc::c_int) -
                                                   1 as libc::c_int) as
                                                  libc::c_uint) <<
                                             16 as libc::c_int |
                                             ((4 as libc::c_int *
                                                   2 as libc::c_int) as u32_0
                                                  &
                                                  (((0x1 as libc::c_int) <<
                                                        8 as libc::c_int) -
                                                       1 as libc::c_int) as
                                                      libc::c_uint) <<
                                                 8 as libc::c_int) |
                                            ((1 as libc::c_int *
                                                  2 as libc::c_int) as u32_0 &
                                                 (((0x1 as libc::c_int) <<
                                                       8 as libc::c_int) -
                                                      1 as libc::c_int) as
                                                     libc::c_uint) <<
                                                0 as libc::c_int
                                    } else {
                                        (((4 as libc::c_int *
                                               2 as libc::c_int) as u32_0 &
                                              (((0x1 as libc::c_int) <<
                                                    8 as libc::c_int) -
                                                   1 as libc::c_int) as
                                                  libc::c_uint) <<
                                             16 as libc::c_int |
                                             ((1 as libc::c_int *
                                                   2 as libc::c_int) as u32_0
                                                  &
                                                  (((0x1 as libc::c_int) <<
                                                        8 as libc::c_int) -
                                                       1 as libc::c_int) as
                                                      libc::c_uint) <<
                                                 8 as libc::c_int) |
                                            ((3 as libc::c_int *
                                                  2 as libc::c_int) as u32_0 &
                                                 (((0x1 as libc::c_int) <<
                                                       8 as libc::c_int) -
                                                      1 as libc::c_int) as
                                                     libc::c_uint) <<
                                                0 as libc::c_int
                                    },};
                     init
                 },},
         Gfx{words:
                 {
                     let mut init =
                         Gwords{w0:
                                    (0x6 as libc::c_int as u32_0 &
                                         (((0x1 as libc::c_int) <<
                                               8 as libc::c_int) -
                                              1 as libc::c_int) as
                                             libc::c_uint) <<
                                        24 as libc::c_int |
                                        (if 0 as libc::c_int ==
                                                0 as libc::c_int {
                                             (((5 as libc::c_int *
                                                    2 as libc::c_int) as u32_0
                                                   &
                                                   (((0x1 as libc::c_int) <<
                                                         8 as libc::c_int) -
                                                        1 as libc::c_int) as
                                                       libc::c_uint) <<
                                                  16 as libc::c_int |
                                                  ((6 as libc::c_int *
                                                        2 as libc::c_int) as
                                                       u32_0 &
                                                       (((0x1 as libc::c_int)
                                                             <<
                                                             8 as libc::c_int)
                                                            -
                                                            1 as libc::c_int)
                                                           as libc::c_uint) <<
                                                      8 as libc::c_int) |
                                                 ((7 as libc::c_int *
                                                       2 as libc::c_int) as
                                                      u32_0 &
                                                      (((0x1 as libc::c_int)
                                                            <<
                                                            8 as libc::c_int)
                                                           - 1 as libc::c_int)
                                                          as libc::c_uint) <<
                                                     0 as libc::c_int
                                         } else {
                                             (if 0 as libc::c_int ==
                                                     1 as libc::c_int {
                                                  (((6 as libc::c_int *
                                                         2 as libc::c_int) as
                                                        u32_0 &
                                                        (((0x1 as libc::c_int)
                                                              <<
                                                              8 as
                                                                  libc::c_int)
                                                             -
                                                             1 as libc::c_int)
                                                            as libc::c_uint)
                                                       << 16 as libc::c_int |
                                                       ((7 as libc::c_int *
                                                             2 as libc::c_int)
                                                            as u32_0 &
                                                            (((0x1 as
                                                                   libc::c_int)
                                                                  <<
                                                                  8 as
                                                                      libc::c_int)
                                                                 -
                                                                 1 as
                                                                     libc::c_int)
                                                                as
                                                                libc::c_uint)
                                                           <<
                                                           8 as libc::c_int) |
                                                      ((5 as libc::c_int *
                                                            2 as libc::c_int)
                                                           as u32_0 &
                                                           (((0x1 as
                                                                  libc::c_int)
                                                                 <<
                                                                 8 as
                                                                     libc::c_int)
                                                                -
                                                                1 as
                                                                    libc::c_int)
                                                               as
                                                               libc::c_uint)
                                                          << 0 as libc::c_int
                                              } else {
                                                  (((7 as libc::c_int *
                                                         2 as libc::c_int) as
                                                        u32_0 &
                                                        (((0x1 as libc::c_int)
                                                              <<
                                                              8 as
                                                                  libc::c_int)
                                                             -
                                                             1 as libc::c_int)
                                                            as libc::c_uint)
                                                       << 16 as libc::c_int |
                                                       ((5 as libc::c_int *
                                                             2 as libc::c_int)
                                                            as u32_0 &
                                                            (((0x1 as
                                                                   libc::c_int)
                                                                  <<
                                                                  8 as
                                                                      libc::c_int)
                                                                 -
                                                                 1 as
                                                                     libc::c_int)
                                                                as
                                                                libc::c_uint)
                                                           <<
                                                           8 as libc::c_int) |
                                                      ((6 as libc::c_int *
                                                            2 as libc::c_int)
                                                           as u32_0 &
                                                           (((0x1 as
                                                                  libc::c_int)
                                                                 <<
                                                                 8 as
                                                                     libc::c_int)
                                                                -
                                                                1 as
                                                                    libc::c_int)
                                                               as
                                                               libc::c_uint)
                                                          << 0 as libc::c_int
                                              })
                                         }),
                                w1:
                                    if 0 as libc::c_int == 0 as libc::c_int {
                                        (((6 as libc::c_int *
                                               2 as libc::c_int) as u32_0 &
                                              (((0x1 as libc::c_int) <<
                                                    8 as libc::c_int) -
                                                   1 as libc::c_int) as
                                                  libc::c_uint) <<
                                             16 as libc::c_int |
                                             ((8 as libc::c_int *
                                                   2 as libc::c_int) as u32_0
                                                  &
                                                  (((0x1 as libc::c_int) <<
                                                        8 as libc::c_int) -
                                                       1 as libc::c_int) as
                                                      libc::c_uint) <<
                                                 8 as libc::c_int) |
                                            ((9 as libc::c_int *
                                                  2 as libc::c_int) as u32_0 &
                                                 (((0x1 as libc::c_int) <<
                                                       8 as libc::c_int) -
                                                      1 as libc::c_int) as
                                                     libc::c_uint) <<
                                                0 as libc::c_int
                                    } else if 0 as libc::c_int ==
                                                  1 as libc::c_int {
                                        (((8 as libc::c_int *
                                               2 as libc::c_int) as u32_0 &
                                              (((0x1 as libc::c_int) <<
                                                    8 as libc::c_int) -
                                                   1 as libc::c_int) as
                                                  libc::c_uint) <<
                                             16 as libc::c_int |
                                             ((9 as libc::c_int *
                                                   2 as libc::c_int) as u32_0
                                                  &
                                                  (((0x1 as libc::c_int) <<
                                                        8 as libc::c_int) -
                                                       1 as libc::c_int) as
                                                      libc::c_uint) <<
                                                 8 as libc::c_int) |
                                            ((6 as libc::c_int *
                                                  2 as libc::c_int) as u32_0 &
                                                 (((0x1 as libc::c_int) <<
                                                       8 as libc::c_int) -
                                                      1 as libc::c_int) as
                                                     libc::c_uint) <<
                                                0 as libc::c_int
                                    } else {
                                        (((9 as libc::c_int *
                                               2 as libc::c_int) as u32_0 &
                                              (((0x1 as libc::c_int) <<
                                                    8 as libc::c_int) -
                                                   1 as libc::c_int) as
                                                  libc::c_uint) <<
                                             16 as libc::c_int |
                                             ((6 as libc::c_int *
                                                   2 as libc::c_int) as u32_0
                                                  &
                                                  (((0x1 as libc::c_int) <<
                                                        8 as libc::c_int) -
                                                       1 as libc::c_int) as
                                                      libc::c_uint) <<
                                                 8 as libc::c_int) |
                                            ((8 as libc::c_int *
                                                  2 as libc::c_int) as u32_0 &
                                                 (((0x1 as libc::c_int) <<
                                                       8 as libc::c_int) -
                                                      1 as libc::c_int) as
                                                     libc::c_uint) <<
                                                0 as libc::c_int
                                    },};
                     init
                 },},
         Gfx{words:
                 {
                     let mut init =
                         Gwords{w0:
                                    (0x6 as libc::c_int as u32_0 &
                                         (((0x1 as libc::c_int) <<
                                               8 as libc::c_int) -
                                              1 as libc::c_int) as
                                             libc::c_uint) <<
                                        24 as libc::c_int |
                                        (if 0 as libc::c_int ==
                                                0 as libc::c_int {
                                             (((8 as libc::c_int *
                                                    2 as libc::c_int) as u32_0
                                                   &
                                                   (((0x1 as libc::c_int) <<
                                                         8 as libc::c_int) -
                                                        1 as libc::c_int) as
                                                       libc::c_uint) <<
                                                  16 as libc::c_int |
                                                  ((10 as libc::c_int *
                                                        2 as libc::c_int) as
                                                       u32_0 &
                                                       (((0x1 as libc::c_int)
                                                             <<
                                                             8 as libc::c_int)
                                                            -
                                                            1 as libc::c_int)
                                                           as libc::c_uint) <<
                                                      8 as libc::c_int) |
                                                 ((11 as libc::c_int *
                                                       2 as libc::c_int) as
                                                      u32_0 &
                                                      (((0x1 as libc::c_int)
                                                            <<
                                                            8 as libc::c_int)
                                                           - 1 as libc::c_int)
                                                          as libc::c_uint) <<
                                                     0 as libc::c_int
                                         } else {
                                             (if 0 as libc::c_int ==
                                                     1 as libc::c_int {
                                                  (((10 as libc::c_int *
                                                         2 as libc::c_int) as
                                                        u32_0 &
                                                        (((0x1 as libc::c_int)
                                                              <<
                                                              8 as
                                                                  libc::c_int)
                                                             -
                                                             1 as libc::c_int)
                                                            as libc::c_uint)
                                                       << 16 as libc::c_int |
                                                       ((11 as libc::c_int *
                                                             2 as libc::c_int)
                                                            as u32_0 &
                                                            (((0x1 as
                                                                   libc::c_int)
                                                                  <<
                                                                  8 as
                                                                      libc::c_int)
                                                                 -
                                                                 1 as
                                                                     libc::c_int)
                                                                as
                                                                libc::c_uint)
                                                           <<
                                                           8 as libc::c_int) |
                                                      ((8 as libc::c_int *
                                                            2 as libc::c_int)
                                                           as u32_0 &
                                                           (((0x1 as
                                                                  libc::c_int)
                                                                 <<
                                                                 8 as
                                                                     libc::c_int)
                                                                -
                                                                1 as
                                                                    libc::c_int)
                                                               as
                                                               libc::c_uint)
                                                          << 0 as libc::c_int
                                              } else {
                                                  (((11 as libc::c_int *
                                                         2 as libc::c_int) as
                                                        u32_0 &
                                                        (((0x1 as libc::c_int)
                                                              <<
                                                              8 as
                                                                  libc::c_int)
                                                             -
                                                             1 as libc::c_int)
                                                            as libc::c_uint)
                                                       << 16 as libc::c_int |
                                                       ((8 as libc::c_int *
                                                             2 as libc::c_int)
                                                            as u32_0 &
                                                            (((0x1 as
                                                                   libc::c_int)
                                                                  <<
                                                                  8 as
                                                                      libc::c_int)
                                                                 -
                                                                 1 as
                                                                     libc::c_int)
                                                                as
                                                                libc::c_uint)
                                                           <<
                                                           8 as libc::c_int) |
                                                      ((10 as libc::c_int *
                                                            2 as libc::c_int)
                                                           as u32_0 &
                                                           (((0x1 as
                                                                  libc::c_int)
                                                                 <<
                                                                 8 as
                                                                     libc::c_int)
                                                                -
                                                                1 as
                                                                    libc::c_int)
                                                               as
                                                               libc::c_uint)
                                                          << 0 as libc::c_int
                                              })
                                         }),
                                w1:
                                    if 0 as libc::c_int == 0 as libc::c_int {
                                        (((10 as libc::c_int *
                                               2 as libc::c_int) as u32_0 &
                                              (((0x1 as libc::c_int) <<
                                                    8 as libc::c_int) -
                                                   1 as libc::c_int) as
                                                  libc::c_uint) <<
                                             16 as libc::c_int |
                                             ((12 as libc::c_int *
                                                   2 as libc::c_int) as u32_0
                                                  &
                                                  (((0x1 as libc::c_int) <<
                                                        8 as libc::c_int) -
                                                       1 as libc::c_int) as
                                                      libc::c_uint) <<
                                                 8 as libc::c_int) |
                                            ((13 as libc::c_int *
                                                  2 as libc::c_int) as u32_0 &
                                                 (((0x1 as libc::c_int) <<
                                                       8 as libc::c_int) -
                                                      1 as libc::c_int) as
                                                     libc::c_uint) <<
                                                0 as libc::c_int
                                    } else if 0 as libc::c_int ==
                                                  1 as libc::c_int {
                                        (((12 as libc::c_int *
                                               2 as libc::c_int) as u32_0 &
                                              (((0x1 as libc::c_int) <<
                                                    8 as libc::c_int) -
                                                   1 as libc::c_int) as
                                                  libc::c_uint) <<
                                             16 as libc::c_int |
                                             ((13 as libc::c_int *
                                                   2 as libc::c_int) as u32_0
                                                  &
                                                  (((0x1 as libc::c_int) <<
                                                        8 as libc::c_int) -
                                                       1 as libc::c_int) as
                                                      libc::c_uint) <<
                                                 8 as libc::c_int) |
                                            ((10 as libc::c_int *
                                                  2 as libc::c_int) as u32_0 &
                                                 (((0x1 as libc::c_int) <<
                                                       8 as libc::c_int) -
                                                      1 as libc::c_int) as
                                                     libc::c_uint) <<
                                                0 as libc::c_int
                                    } else {
                                        (((13 as libc::c_int *
                                               2 as libc::c_int) as u32_0 &
                                              (((0x1 as libc::c_int) <<
                                                    8 as libc::c_int) -
                                                   1 as libc::c_int) as
                                                  libc::c_uint) <<
                                             16 as libc::c_int |
                                             ((10 as libc::c_int *
                                                   2 as libc::c_int) as u32_0
                                                  &
                                                  (((0x1 as libc::c_int) <<
                                                        8 as libc::c_int) -
                                                       1 as libc::c_int) as
                                                      libc::c_uint) <<
                                                 8 as libc::c_int) |
                                            ((12 as libc::c_int *
                                                  2 as libc::c_int) as u32_0 &
                                                 (((0x1 as libc::c_int) <<
                                                       8 as libc::c_int) -
                                                      1 as libc::c_int) as
                                                     libc::c_uint) <<
                                                0 as libc::c_int
                                    },};
                     init
                 },},
         Gfx{words:
                 {
                     let mut init =
                         Gwords{w0:
                                    (0x6 as libc::c_int as u32_0 &
                                         (((0x1 as libc::c_int) <<
                                               8 as libc::c_int) -
                                              1 as libc::c_int) as
                                             libc::c_uint) <<
                                        24 as libc::c_int |
                                        (if 0 as libc::c_int ==
                                                0 as libc::c_int {
                                             (((12 as libc::c_int *
                                                    2 as libc::c_int) as u32_0
                                                   &
                                                   (((0x1 as libc::c_int) <<
                                                         8 as libc::c_int) -
                                                        1 as libc::c_int) as
                                                       libc::c_uint) <<
                                                  16 as libc::c_int |
                                                  ((14 as libc::c_int *
                                                        2 as libc::c_int) as
                                                       u32_0 &
                                                       (((0x1 as libc::c_int)
                                                             <<
                                                             8 as libc::c_int)
                                                            -
                                                            1 as libc::c_int)
                                                           as libc::c_uint) <<
                                                      8 as libc::c_int) |
                                                 ((15 as libc::c_int *
                                                       2 as libc::c_int) as
                                                      u32_0 &
                                                      (((0x1 as libc::c_int)
                                                            <<
                                                            8 as libc::c_int)
                                                           - 1 as libc::c_int)
                                                          as libc::c_uint) <<
                                                     0 as libc::c_int
                                         } else {
                                             (if 0 as libc::c_int ==
                                                     1 as libc::c_int {
                                                  (((14 as libc::c_int *
                                                         2 as libc::c_int) as
                                                        u32_0 &
                                                        (((0x1 as libc::c_int)
                                                              <<
                                                              8 as
                                                                  libc::c_int)
                                                             -
                                                             1 as libc::c_int)
                                                            as libc::c_uint)
                                                       << 16 as libc::c_int |
                                                       ((15 as libc::c_int *
                                                             2 as libc::c_int)
                                                            as u32_0 &
                                                            (((0x1 as
                                                                   libc::c_int)
                                                                  <<
                                                                  8 as
                                                                      libc::c_int)
                                                                 -
                                                                 1 as
                                                                     libc::c_int)
                                                                as
                                                                libc::c_uint)
                                                           <<
                                                           8 as libc::c_int) |
                                                      ((12 as libc::c_int *
                                                            2 as libc::c_int)
                                                           as u32_0 &
                                                           (((0x1 as
                                                                  libc::c_int)
                                                                 <<
                                                                 8 as
                                                                     libc::c_int)
                                                                -
                                                                1 as
                                                                    libc::c_int)
                                                               as
                                                               libc::c_uint)
                                                          << 0 as libc::c_int
                                              } else {
                                                  (((15 as libc::c_int *
                                                         2 as libc::c_int) as
                                                        u32_0 &
                                                        (((0x1 as libc::c_int)
                                                              <<
                                                              8 as
                                                                  libc::c_int)
                                                             -
                                                             1 as libc::c_int)
                                                            as libc::c_uint)
                                                       << 16 as libc::c_int |
                                                       ((12 as libc::c_int *
                                                             2 as libc::c_int)
                                                            as u32_0 &
                                                            (((0x1 as
                                                                   libc::c_int)
                                                                  <<
                                                                  8 as
                                                                      libc::c_int)
                                                                 -
                                                                 1 as
                                                                     libc::c_int)
                                                                as
                                                                libc::c_uint)
                                                           <<
                                                           8 as libc::c_int) |
                                                      ((14 as libc::c_int *
                                                            2 as libc::c_int)
                                                           as u32_0 &
                                                           (((0x1 as
                                                                  libc::c_int)
                                                                 <<
                                                                 8 as
                                                                     libc::c_int)
                                                                -
                                                                1 as
                                                                    libc::c_int)
                                                               as
                                                               libc::c_uint)
                                                          << 0 as libc::c_int
                                              })
                                         }),
                                w1:
                                    if 0 as libc::c_int == 0 as libc::c_int {
                                        (((14 as libc::c_int *
                                               2 as libc::c_int) as u32_0 &
                                              (((0x1 as libc::c_int) <<
                                                    8 as libc::c_int) -
                                                   1 as libc::c_int) as
                                                  libc::c_uint) <<
                                             16 as libc::c_int |
                                             ((16 as libc::c_int *
                                                   2 as libc::c_int) as u32_0
                                                  &
                                                  (((0x1 as libc::c_int) <<
                                                        8 as libc::c_int) -
                                                       1 as libc::c_int) as
                                                      libc::c_uint) <<
                                                 8 as libc::c_int) |
                                            ((17 as libc::c_int *
                                                  2 as libc::c_int) as u32_0 &
                                                 (((0x1 as libc::c_int) <<
                                                       8 as libc::c_int) -
                                                      1 as libc::c_int) as
                                                     libc::c_uint) <<
                                                0 as libc::c_int
                                    } else if 0 as libc::c_int ==
                                                  1 as libc::c_int {
                                        (((16 as libc::c_int *
                                               2 as libc::c_int) as u32_0 &
                                              (((0x1 as libc::c_int) <<
                                                    8 as libc::c_int) -
                                                   1 as libc::c_int) as
                                                  libc::c_uint) <<
                                             16 as libc::c_int |
                                             ((17 as libc::c_int *
                                                   2 as libc::c_int) as u32_0
                                                  &
                                                  (((0x1 as libc::c_int) <<
                                                        8 as libc::c_int) -
                                                       1 as libc::c_int) as
                                                      libc::c_uint) <<
                                                 8 as libc::c_int) |
                                            ((14 as libc::c_int *
                                                  2 as libc::c_int) as u32_0 &
                                                 (((0x1 as libc::c_int) <<
                                                       8 as libc::c_int) -
                                                      1 as libc::c_int) as
                                                     libc::c_uint) <<
                                                0 as libc::c_int
                                    } else {
                                        (((17 as libc::c_int *
                                               2 as libc::c_int) as u32_0 &
                                              (((0x1 as libc::c_int) <<
                                                    8 as libc::c_int) -
                                                   1 as libc::c_int) as
                                                  libc::c_uint) <<
                                             16 as libc::c_int |
                                             ((14 as libc::c_int *
                                                   2 as libc::c_int) as u32_0
                                                  &
                                                  (((0x1 as libc::c_int) <<
                                                        8 as libc::c_int) -
                                                       1 as libc::c_int) as
                                                      libc::c_uint) <<
                                                 8 as libc::c_int) |
                                            ((16 as libc::c_int *
                                                  2 as libc::c_int) as u32_0 &
                                                 (((0x1 as libc::c_int) <<
                                                       8 as libc::c_int) -
                                                      1 as libc::c_int) as
                                                     libc::c_uint) <<
                                                0 as libc::c_int
                                    },};
                     init
                 },},
         Gfx{words:
                 {
                     let mut init =
                         Gwords{w0:
                                    (0x6 as libc::c_int as u32_0 &
                                         (((0x1 as libc::c_int) <<
                                               8 as libc::c_int) -
                                              1 as libc::c_int) as
                                             libc::c_uint) <<
                                        24 as libc::c_int |
                                        (if 0 as libc::c_int ==
                                                0 as libc::c_int {
                                             (((16 as libc::c_int *
                                                    2 as libc::c_int) as u32_0
                                                   &
                                                   (((0x1 as libc::c_int) <<
                                                         8 as libc::c_int) -
                                                        1 as libc::c_int) as
                                                       libc::c_uint) <<
                                                  16 as libc::c_int |
                                                  ((18 as libc::c_int *
                                                        2 as libc::c_int) as
                                                       u32_0 &
                                                       (((0x1 as libc::c_int)
                                                             <<
                                                             8 as libc::c_int)
                                                            -
                                                            1 as libc::c_int)
                                                           as libc::c_uint) <<
                                                      8 as libc::c_int) |
                                                 ((19 as libc::c_int *
                                                       2 as libc::c_int) as
                                                      u32_0 &
                                                      (((0x1 as libc::c_int)
                                                            <<
                                                            8 as libc::c_int)
                                                           - 1 as libc::c_int)
                                                          as libc::c_uint) <<
                                                     0 as libc::c_int
                                         } else {
                                             (if 0 as libc::c_int ==
                                                     1 as libc::c_int {
                                                  (((18 as libc::c_int *
                                                         2 as libc::c_int) as
                                                        u32_0 &
                                                        (((0x1 as libc::c_int)
                                                              <<
                                                              8 as
                                                                  libc::c_int)
                                                             -
                                                             1 as libc::c_int)
                                                            as libc::c_uint)
                                                       << 16 as libc::c_int |
                                                       ((19 as libc::c_int *
                                                             2 as libc::c_int)
                                                            as u32_0 &
                                                            (((0x1 as
                                                                   libc::c_int)
                                                                  <<
                                                                  8 as
                                                                      libc::c_int)
                                                                 -
                                                                 1 as
                                                                     libc::c_int)
                                                                as
                                                                libc::c_uint)
                                                           <<
                                                           8 as libc::c_int) |
                                                      ((16 as libc::c_int *
                                                            2 as libc::c_int)
                                                           as u32_0 &
                                                           (((0x1 as
                                                                  libc::c_int)
                                                                 <<
                                                                 8 as
                                                                     libc::c_int)
                                                                -
                                                                1 as
                                                                    libc::c_int)
                                                               as
                                                               libc::c_uint)
                                                          << 0 as libc::c_int
                                              } else {
                                                  (((19 as libc::c_int *
                                                         2 as libc::c_int) as
                                                        u32_0 &
                                                        (((0x1 as libc::c_int)
                                                              <<
                                                              8 as
                                                                  libc::c_int)
                                                             -
                                                             1 as libc::c_int)
                                                            as libc::c_uint)
                                                       << 16 as libc::c_int |
                                                       ((16 as libc::c_int *
                                                             2 as libc::c_int)
                                                            as u32_0 &
                                                            (((0x1 as
                                                                   libc::c_int)
                                                                  <<
                                                                  8 as
                                                                      libc::c_int)
                                                                 -
                                                                 1 as
                                                                     libc::c_int)
                                                                as
                                                                libc::c_uint)
                                                           <<
                                                           8 as libc::c_int) |
                                                      ((18 as libc::c_int *
                                                            2 as libc::c_int)
                                                           as u32_0 &
                                                           (((0x1 as
                                                                  libc::c_int)
                                                                 <<
                                                                 8 as
                                                                     libc::c_int)
                                                                -
                                                                1 as
                                                                    libc::c_int)
                                                               as
                                                               libc::c_uint)
                                                          << 0 as libc::c_int
                                              })
                                         }),
                                w1:
                                    if 0 as libc::c_int == 0 as libc::c_int {
                                        (((18 as libc::c_int *
                                               2 as libc::c_int) as u32_0 &
                                              (((0x1 as libc::c_int) <<
                                                    8 as libc::c_int) -
                                                   1 as libc::c_int) as
                                                  libc::c_uint) <<
                                             16 as libc::c_int |
                                             ((20 as libc::c_int *
                                                   2 as libc::c_int) as u32_0
                                                  &
                                                  (((0x1 as libc::c_int) <<
                                                        8 as libc::c_int) -
                                                       1 as libc::c_int) as
                                                      libc::c_uint) <<
                                                 8 as libc::c_int) |
                                            ((21 as libc::c_int *
                                                  2 as libc::c_int) as u32_0 &
                                                 (((0x1 as libc::c_int) <<
                                                       8 as libc::c_int) -
                                                      1 as libc::c_int) as
                                                     libc::c_uint) <<
                                                0 as libc::c_int
                                    } else if 0 as libc::c_int ==
                                                  1 as libc::c_int {
                                        (((20 as libc::c_int *
                                               2 as libc::c_int) as u32_0 &
                                              (((0x1 as libc::c_int) <<
                                                    8 as libc::c_int) -
                                                   1 as libc::c_int) as
                                                  libc::c_uint) <<
                                             16 as libc::c_int |
                                             ((21 as libc::c_int *
                                                   2 as libc::c_int) as u32_0
                                                  &
                                                  (((0x1 as libc::c_int) <<
                                                        8 as libc::c_int) -
                                                       1 as libc::c_int) as
                                                      libc::c_uint) <<
                                                 8 as libc::c_int) |
                                            ((18 as libc::c_int *
                                                  2 as libc::c_int) as u32_0 &
                                                 (((0x1 as libc::c_int) <<
                                                       8 as libc::c_int) -
                                                      1 as libc::c_int) as
                                                     libc::c_uint) <<
                                                0 as libc::c_int
                                    } else {
                                        (((21 as libc::c_int *
                                               2 as libc::c_int) as u32_0 &
                                              (((0x1 as libc::c_int) <<
                                                    8 as libc::c_int) -
                                                   1 as libc::c_int) as
                                                  libc::c_uint) <<
                                             16 as libc::c_int |
                                             ((18 as libc::c_int *
                                                   2 as libc::c_int) as u32_0
                                                  &
                                                  (((0x1 as libc::c_int) <<
                                                        8 as libc::c_int) -
                                                       1 as libc::c_int) as
                                                      libc::c_uint) <<
                                                 8 as libc::c_int) |
                                            ((20 as libc::c_int *
                                                  2 as libc::c_int) as u32_0 &
                                                 (((0x1 as libc::c_int) <<
                                                       8 as libc::c_int) -
                                                      1 as libc::c_int) as
                                                     libc::c_uint) <<
                                                0 as libc::c_int
                                    },};
                     init
                 },},
         Gfx{words:
                 {
                     let mut init =
                         Gwords{w0:
                                    (0x6 as libc::c_int as u32_0 &
                                         (((0x1 as libc::c_int) <<
                                               8 as libc::c_int) -
                                              1 as libc::c_int) as
                                             libc::c_uint) <<
                                        24 as libc::c_int |
                                        (if 0 as libc::c_int ==
                                                0 as libc::c_int {
                                             (((20 as libc::c_int *
                                                    2 as libc::c_int) as u32_0
                                                   &
                                                   (((0x1 as libc::c_int) <<
                                                         8 as libc::c_int) -
                                                        1 as libc::c_int) as
                                                       libc::c_uint) <<
                                                  16 as libc::c_int |
                                                  ((22 as libc::c_int *
                                                        2 as libc::c_int) as
                                                       u32_0 &
                                                       (((0x1 as libc::c_int)
                                                             <<
                                                             8 as libc::c_int)
                                                            -
                                                            1 as libc::c_int)
                                                           as libc::c_uint) <<
                                                      8 as libc::c_int) |
                                                 ((23 as libc::c_int *
                                                       2 as libc::c_int) as
                                                      u32_0 &
                                                      (((0x1 as libc::c_int)
                                                            <<
                                                            8 as libc::c_int)
                                                           - 1 as libc::c_int)
                                                          as libc::c_uint) <<
                                                     0 as libc::c_int
                                         } else {
                                             (if 0 as libc::c_int ==
                                                     1 as libc::c_int {
                                                  (((22 as libc::c_int *
                                                         2 as libc::c_int) as
                                                        u32_0 &
                                                        (((0x1 as libc::c_int)
                                                              <<
                                                              8 as
                                                                  libc::c_int)
                                                             -
                                                             1 as libc::c_int)
                                                            as libc::c_uint)
                                                       << 16 as libc::c_int |
                                                       ((23 as libc::c_int *
                                                             2 as libc::c_int)
                                                            as u32_0 &
                                                            (((0x1 as
                                                                   libc::c_int)
                                                                  <<
                                                                  8 as
                                                                      libc::c_int)
                                                                 -
                                                                 1 as
                                                                     libc::c_int)
                                                                as
                                                                libc::c_uint)
                                                           <<
                                                           8 as libc::c_int) |
                                                      ((20 as libc::c_int *
                                                            2 as libc::c_int)
                                                           as u32_0 &
                                                           (((0x1 as
                                                                  libc::c_int)
                                                                 <<
                                                                 8 as
                                                                     libc::c_int)
                                                                -
                                                                1 as
                                                                    libc::c_int)
                                                               as
                                                               libc::c_uint)
                                                          << 0 as libc::c_int
                                              } else {
                                                  (((23 as libc::c_int *
                                                         2 as libc::c_int) as
                                                        u32_0 &
                                                        (((0x1 as libc::c_int)
                                                              <<
                                                              8 as
                                                                  libc::c_int)
                                                             -
                                                             1 as libc::c_int)
                                                            as libc::c_uint)
                                                       << 16 as libc::c_int |
                                                       ((20 as libc::c_int *
                                                             2 as libc::c_int)
                                                            as u32_0 &
                                                            (((0x1 as
                                                                   libc::c_int)
                                                                  <<
                                                                  8 as
                                                                      libc::c_int)
                                                                 -
                                                                 1 as
                                                                     libc::c_int)
                                                                as
                                                                libc::c_uint)
                                                           <<
                                                           8 as libc::c_int) |
                                                      ((22 as libc::c_int *
                                                            2 as libc::c_int)
                                                           as u32_0 &
                                                           (((0x1 as
                                                                  libc::c_int)
                                                                 <<
                                                                 8 as
                                                                     libc::c_int)
                                                                -
                                                                1 as
                                                                    libc::c_int)
                                                               as
                                                               libc::c_uint)
                                                          << 0 as libc::c_int
                                              })
                                         }),
                                w1:
                                    if 0 as libc::c_int == 0 as libc::c_int {
                                        (((22 as libc::c_int *
                                               2 as libc::c_int) as u32_0 &
                                              (((0x1 as libc::c_int) <<
                                                    8 as libc::c_int) -
                                                   1 as libc::c_int) as
                                                  libc::c_uint) <<
                                             16 as libc::c_int |
                                             ((0 as libc::c_int *
                                                   2 as libc::c_int) as u32_0
                                                  &
                                                  (((0x1 as libc::c_int) <<
                                                        8 as libc::c_int) -
                                                       1 as libc::c_int) as
                                                      libc::c_uint) <<
                                                 8 as libc::c_int) |
                                            ((24 as libc::c_int *
                                                  2 as libc::c_int) as u32_0 &
                                                 (((0x1 as libc::c_int) <<
                                                       8 as libc::c_int) -
                                                      1 as libc::c_int) as
                                                     libc::c_uint) <<
                                                0 as libc::c_int
                                    } else if 0 as libc::c_int ==
                                                  1 as libc::c_int {
                                        (((0 as libc::c_int *
                                               2 as libc::c_int) as u32_0 &
                                              (((0x1 as libc::c_int) <<
                                                    8 as libc::c_int) -
                                                   1 as libc::c_int) as
                                                  libc::c_uint) <<
                                             16 as libc::c_int |
                                             ((24 as libc::c_int *
                                                   2 as libc::c_int) as u32_0
                                                  &
                                                  (((0x1 as libc::c_int) <<
                                                        8 as libc::c_int) -
                                                       1 as libc::c_int) as
                                                      libc::c_uint) <<
                                                 8 as libc::c_int) |
                                            ((22 as libc::c_int *
                                                  2 as libc::c_int) as u32_0 &
                                                 (((0x1 as libc::c_int) <<
                                                       8 as libc::c_int) -
                                                      1 as libc::c_int) as
                                                     libc::c_uint) <<
                                                0 as libc::c_int
                                    } else {
                                        (((24 as libc::c_int *
                                               2 as libc::c_int) as u32_0 &
                                              (((0x1 as libc::c_int) <<
                                                    8 as libc::c_int) -
                                                   1 as libc::c_int) as
                                                  libc::c_uint) <<
                                             16 as libc::c_int |
                                             ((22 as libc::c_int *
                                                   2 as libc::c_int) as u32_0
                                                  &
                                                  (((0x1 as libc::c_int) <<
                                                        8 as libc::c_int) -
                                                       1 as libc::c_int) as
                                                      libc::c_uint) <<
                                                 8 as libc::c_int) |
                                            ((0 as libc::c_int *
                                                  2 as libc::c_int) as u32_0 &
                                                 (((0x1 as libc::c_int) <<
                                                       8 as libc::c_int) -
                                                      1 as libc::c_int) as
                                                     libc::c_uint) <<
                                                0 as libc::c_int
                                    },};
                     init
                 },},
         Gfx{words:
                 {
                     let mut init =
                         Gwords{w0:
                                    (0xdf as libc::c_int as u32_0 &
                                         (((0x1 as libc::c_int) <<
                                               8 as libc::c_int) -
                                              1 as libc::c_int) as
                                             libc::c_uint) <<
                                        24 as libc::c_int,
                                w1: 0 as libc::c_int as libc::c_uint,};
                     init
                 },}]
}
#[used]
#[cfg_attr(target_os = "linux", link_section = ".init_array")]
#[cfg_attr(target_os = "windows", link_section = ".CRT$XIB")]
#[cfg_attr(target_os = "macos", link_section = "__DATA,__mod_init_func")]
static INIT_ARRAY: [unsafe extern "C" fn(); 1] = [run_static_initializers];
