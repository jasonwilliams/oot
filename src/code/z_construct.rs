#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case,
         non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(const_raw_ptr_to_usize_cast, register_tool)]
extern "C" {
    #[no_mangle]
    fn DmaMgr_SendRequest1(ram0: *mut libc::c_void, vrom: u32_0, size: u32_0,
                           file: *const libc::c_char, line: s32) -> s32;
    #[no_mangle]
    fn __assert(exp: *const libc::c_char, file: *const libc::c_char,
                line: s32);
    #[no_mangle]
    fn osSyncPrintf(fmt: *const libc::c_char, _: ...);
    #[no_mangle]
    fn Font_LoadOrderedFont(font: *mut Font);
    #[no_mangle]
    fn HealthMeter_Init(globalCtx: *mut GlobalContext);
    #[no_mangle]
    fn Map_Destroy(globalCtx: *mut GlobalContext);
    #[no_mangle]
    fn Map_Init(globalCtx: *mut GlobalContext);
    #[no_mangle]
    fn View_Init(_: *mut View, _: *mut GraphicsContext);
    #[no_mangle]
    fn GameState_Alloc(gameState: *mut GameState, size: size_t,
                       file: *mut libc::c_char, line: s32)
     -> *mut libc::c_void;
    #[no_mangle]
    fn Message_SetTables();
    #[no_mangle]
    static mut gGameInfo: *mut GameInfo;
    #[no_mangle]
    static mut gSaveContext: SaveContext;
    #[no_mangle]
    static mut _icon_item_staticSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut _do_action_staticSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut _parameter_staticSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut _parameter_staticSegmentRomEnd: [u8_0; 0];
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
pub type MtxF_t = [[libc::c_float; 4]; 4];
#[derive(Copy, Clone)]
#[repr(C)]
pub union MtxF {
    pub mf: MtxF_t,
    pub c2rust_unnamed: C2RustUnnamed_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_0 {
    pub xx: libc::c_float,
    pub yx: libc::c_float,
    pub zx: libc::c_float,
    pub wx: libc::c_float,
    pub xy: libc::c_float,
    pub yy: libc::c_float,
    pub zy: libc::c_float,
    pub wy: libc::c_float,
    pub xz: libc::c_float,
    pub yz: libc::c_float,
    pub zz: libc::c_float,
    pub wz: libc::c_float,
    pub xw: libc::c_float,
    pub yw: libc::c_float,
    pub zw: libc::c_float,
    pub ww: libc::c_float,
}
pub type size_t = libc::c_ulong;
pub type OSPri = s32;
pub type OSId = s32;
#[derive(Copy, Clone)]
#[repr(C)]
pub union __OSfp {
    pub f: C2RustUnnamed_1,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_1 {
    pub f_odd: f32_0,
    pub f_even: f32_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __OSThreadContext {
    pub at: u64_0,
    pub v0: u64_0,
    pub v1: u64_0,
    pub a0: u64_0,
    pub a1: u64_0,
    pub a2: u64_0,
    pub a3: u64_0,
    pub t0: u64_0,
    pub t1: u64_0,
    pub t2: u64_0,
    pub t3: u64_0,
    pub t4: u64_0,
    pub t5: u64_0,
    pub t6: u64_0,
    pub t7: u64_0,
    pub s0: u64_0,
    pub s1: u64_0,
    pub s2: u64_0,
    pub s3: u64_0,
    pub s4: u64_0,
    pub s5: u64_0,
    pub s6: u64_0,
    pub s7: u64_0,
    pub t8: u64_0,
    pub t9: u64_0,
    pub gp: u64_0,
    pub sp: u64_0,
    pub s8: u64_0,
    pub ra: u64_0,
    pub lo: u64_0,
    pub hi: u64_0,
    pub sr: u32_0,
    pub pc: u32_0,
    pub cause: u32_0,
    pub badvaddr: u32_0,
    pub rcp: u32_0,
    pub fpcsr: u32_0,
    pub fp0: __OSfp,
    pub fp2: __OSfp,
    pub fp4: __OSfp,
    pub fp6: __OSfp,
    pub fp8: __OSfp,
    pub fp10: __OSfp,
    pub fp12: __OSfp,
    pub fp14: __OSfp,
    pub fp16: __OSfp,
    pub fp18: __OSfp,
    pub fp20: __OSfp,
    pub fp22: __OSfp,
    pub fp24: __OSfp,
    pub fp26: __OSfp,
    pub fp28: __OSfp,
    pub fp30: __OSfp,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __OSThreadprofile {
    pub flag: u32_0,
    pub count: u32_0,
    pub time: u64_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct OSThread {
    pub next: *mut OSThread,
    pub priority: OSPri,
    pub queue: *mut *mut OSThread,
    pub tlnext: *mut OSThread,
    pub state: u16_0,
    pub flags: u16_0,
    pub id: OSId,
    pub fp: s32,
    pub thprof: *mut __OSThreadprofile,
    pub context: __OSThreadContext,
}
pub type OSMesg = *mut libc::c_void;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct OSMesgQueue {
    pub mtqueue: *mut OSThread,
    pub fullqueue: *mut OSThread,
    pub validCount: s32,
    pub first: s32,
    pub msgCount: s32,
    pub msg: *mut OSMesg,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct OSTask_t {
    pub type_0: u32_0,
    pub flags: u32_0,
    pub ucode_boot: *mut u64_0,
    pub ucode_boot_size: u32_0,
    pub ucode: *mut u64_0,
    pub ucode_size: u32_0,
    pub ucode_data: *mut u64_0,
    pub ucode_data_size: u32_0,
    pub dram_stack: *mut u64_0,
    pub dram_stack_size: u32_0,
    pub output_buff: *mut u64_0,
    pub output_buff_size: *mut u64_0,
    pub data_ptr: *mut u64_0,
    pub data_size: u32_0,
    pub yield_data_ptr: *mut u64_0,
    pub yield_data_size: u32_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union OSTask {
    pub t: OSTask_t,
    pub force_structure_alignment: libc::c_longlong,
}
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Vp_t {
    pub vscale: [libc::c_short; 4],
    pub vtrans: [libc::c_short; 4],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union Vp {
    pub vp: Vp_t,
    pub force_structure_alignment: libc::c_longlong,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Light_t {
    pub col: [libc::c_uchar; 3],
    pub pad1: libc::c_char,
    pub colc: [libc::c_uchar; 3],
    pub pad2: libc::c_char,
    pub dir: [libc::c_schar; 3],
    pub pad3: libc::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Ambient_t {
    pub col: [libc::c_uchar; 3],
    pub pad1: libc::c_char,
    pub colc: [libc::c_uchar; 3],
    pub pad2: libc::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union Light {
    pub l: Light_t,
    pub force_structure_alignment: [libc::c_longlong; 2],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union Ambient {
    pub l: Ambient_t,
    pub force_structure_alignment: [libc::c_longlong; 1],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Lightsn {
    pub a: Ambient,
    pub l: [Light; 7],
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
pub struct Vec3us {
    pub x: u16_0,
    pub y: u16_0,
    pub z: u16_0,
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
pub struct Sphere16 {
    pub center: Vec3s,
    pub radius: s16,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Cylinder16 {
    pub radius: s16,
    pub height: s16,
    pub yShift: s16,
    pub pos: Vec3s,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Linef {
    pub a: Vec3f,
    pub b: Vec3f,
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
pub type C2RustUnnamed_2 = libc::c_uint;
pub const SUNSSONG_SPECIAL: C2RustUnnamed_2 = 3;
pub const SUNSSONG_SPEED_TIME: C2RustUnnamed_2 = 2;
pub const SUNSSONG_START: C2RustUnnamed_2 = 1;
pub const SUNSSONG_INACTIVE: C2RustUnnamed_2 = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Color_RGB8 {
    pub r: u8_0,
    pub g: u8_0,
    pub b: u8_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union Color_RGBA8_u32 {
    pub c2rust_unnamed: C2RustUnnamed_3,
    pub rgba: u32_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_3 {
    pub r: u8_0,
    pub g: u8_0,
    pub b: u8_0,
    pub a: u8_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Color_RGBAf {
    pub r: f32_0,
    pub g: f32_0,
    pub b: f32_0,
    pub a: f32_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct LightPoint {
    pub x: s16,
    pub y: s16,
    pub z: s16,
    pub color: [u8_0; 3],
    pub drawGlow: u8_0,
    pub radius: s16,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct LightDirectional {
    pub x: s8,
    pub y: s8,
    pub z: s8,
    pub color: [u8_0; 3],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union LightParams {
    pub point: LightPoint,
    pub dir: LightDirectional,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct LightInfo {
    pub type_0: u8_0,
    pub params: LightParams,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Lights {
    pub numLights: u8_0,
    pub l: Lightsn,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct LightNode {
    pub info: *mut LightInfo,
    pub prev: *mut LightNode,
    pub next: *mut LightNode,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct LightContext {
    pub listHead: *mut LightNode,
    pub ambientColor: [u8_0; 3],
    pub fogColor: [u8_0; 3],
    pub fogNear: s16,
    pub fogFar: s16,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct GlobalContext {
    pub state: GameState,
    pub sceneNum: s16,
    pub sceneConfig: u8_0,
    pub unk_A7: [libc::c_char; 9],
    pub sceneSegment: *mut libc::c_void,
    pub view: View,
    pub mainCamera: Camera,
    pub subCameras: [Camera; 3],
    pub cameraPtrs: [*mut Camera; 4],
    pub activeCamera: s16,
    pub nextCamera: s16,
    pub sequenceCtx: SequenceContext,
    pub lightCtx: LightContext,
    pub frameAdvCtx: FrameAdvanceContext,
    pub colCtx: CollisionContext,
    pub actorCtx: ActorContext,
    pub csCtx: CutsceneContext,
    pub soundSources: [SoundSource; 16],
    pub sramCtx: SramContext,
    pub skyboxCtx: SkyboxContext,
    pub msgCtx: MessageContext,
    pub interfaceCtx: InterfaceContext,
    pub pauseCtx: PauseContext,
    pub gameOverCtx: GameOverContext,
    pub envCtx: EnvironmentContext,
    pub animationCtx: AnimationContext,
    pub objectCtx: ObjectContext,
    pub roomCtx: RoomContext,
    pub transiActorCtx: TransitionActorContext,
    pub playerInit: Option<unsafe extern "C" fn(_: *mut Player,
                                                _: *mut GlobalContext,
                                                _: *mut FlexSkeletonHeader)
                               -> ()>,
    pub playerUpdate: Option<unsafe extern "C" fn(_: *mut Player,
                                                  _: *mut GlobalContext,
                                                  _: *mut Input) -> ()>,
    pub isPlayerDroppingFish: Option<unsafe extern "C" fn(_:
                                                              *mut GlobalContext)
                                         -> s32>,
    pub startPlayerFishing: Option<unsafe extern "C" fn(_: *mut GlobalContext)
                                       -> s32>,
    pub grabPlayer: Option<unsafe extern "C" fn(_: *mut GlobalContext,
                                                _: *mut Player) -> s32>,
    pub startPlayerCutscene: Option<unsafe extern "C" fn(_:
                                                             *mut GlobalContext,
                                                         _: *mut Actor,
                                                         _: s32) -> s32>,
    pub func_11D54: Option<unsafe extern "C" fn(_: *mut Player,
                                                _: *mut GlobalContext) -> ()>,
    pub damagePlayer: Option<unsafe extern "C" fn(_: *mut GlobalContext,
                                                  _: s32) -> s32>,
    pub talkWithPlayer: Option<unsafe extern "C" fn(_: *mut GlobalContext,
                                                    _: *mut Actor) -> ()>,
    pub viewProjectionMtxF: MtxF,
    pub billboardMtxF: MtxF,
    pub billboardMtx: *mut Mtx,
    pub gameplayFrames: u32_0,
    pub linkAgeOnLoad: u8_0,
    pub unk_11DE9: u8_0,
    pub curSpawn: u8_0,
    pub numSetupActors: u8_0,
    pub numRooms: u8_0,
    pub roomList: *mut RomFile,
    pub linkActorEntry: *mut ActorEntry,
    pub setupActorList: *mut ActorEntry,
    pub unk_11DFC: *mut libc::c_void,
    pub setupEntranceList: *mut EntranceEntry,
    pub setupExitList: *mut s16,
    pub setupPathList: *mut Path,
    pub cUpElfMsgs: *mut ElfMessage,
    pub specialEffects: *mut libc::c_void,
    pub skyboxId: u8_0,
    pub sceneLoadFlag: s8,
    pub unk_11E16: s16,
    pub unk_11E18: s16,
    pub nextEntranceIndex: s16,
    pub unk_11E1C: [libc::c_char; 64],
    pub shootingGalleryStatus: s8,
    pub bombchuBowlingStatus: s8,
    pub fadeTransition: u8_0,
    pub colChkCtx: CollisionCheckContext,
    pub envFlags: [u16_0; 20],
    pub pauseBgPreRender: PreRender,
    pub unk_12174: [libc::c_char; 83],
    pub unk_121C7: s8,
    pub transitionCtx: TransitionContext,
    pub unk_12418: [libc::c_char; 3],
    pub transitionMode: u8_0,
    pub transitionFade: TransitionFade,
    pub unk_12428: [libc::c_char; 3],
    pub unk_1242B: u8_0,
    pub loadedScene: *mut SceneTableEntry,
    pub unk_12430: [libc::c_char; 232],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SceneTableEntry {
    pub sceneFile: RomFile,
    pub titleFile: RomFile,
    pub unk_10: u8_0,
    pub config: u8_0,
    pub unk_12: u8_0,
    pub unk_13: u8_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct RomFile {
    pub vromStart: u32_0,
    pub vromEnd: u32_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct TransitionFade {
    pub fadeType: u8_0,
    pub isDone: u8_0,
    pub fadeDirection: u8_0,
    pub fadeColor: Color_RGBA8_u32,
    pub fadeTimer: u16_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct TransitionContext {
    pub c2rust_unnamed: C2RustUnnamed_4,
    pub transitionType: s32,
    pub init: Option<unsafe extern "C" fn(_: *mut libc::c_void)
                         -> *mut libc::c_void>,
    pub destroy: Option<unsafe extern "C" fn(_: *mut libc::c_void) -> ()>,
    pub update: Option<unsafe extern "C" fn(_: *mut libc::c_void, _: s32)
                           -> ()>,
    pub draw: Option<unsafe extern "C" fn(_: *mut libc::c_void,
                                          _: *mut *mut Gfx) -> ()>,
    pub start: Option<unsafe extern "C" fn(_: *mut libc::c_void) -> ()>,
    pub setType: Option<unsafe extern "C" fn(_: *mut libc::c_void, _: s32)
                            -> ()>,
    pub setColor: Option<unsafe extern "C" fn(_: *mut libc::c_void, _: u32_0)
                             -> ()>,
    pub setEnvColor: Option<unsafe extern "C" fn(_: *mut libc::c_void,
                                                 _: u32_0) -> ()>,
    pub isDone: Option<unsafe extern "C" fn(_: *mut libc::c_void) -> s32>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_4 {
    pub fade: TransitionFade,
    pub circle: TransitionCircle,
    pub triforce: TransitionTriforce,
    pub wipe: TransitionWipe,
    pub data: [libc::c_char; 552],
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct TransitionTriforce {
    pub color: Color_RGBA8_u32,
    pub transPos: f32_0,
    pub step: f32_0,
    pub state: s32,
    pub fadeDirection: s32,
    pub projection: Mtx,
    pub frame: s32,
    pub modelView: [[Mtx; 3]; 2],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct TransitionCircle {
    pub color: Color_RGBA8_u32,
    pub envColor: Color_RGBA8_u32,
    pub texX: s32,
    pub texY: s32,
    pub step: s32,
    pub unk_14: u8_0,
    pub typeColor: u8_0,
    pub speed: u8_0,
    pub effect: u8_0,
    pub isDone: u8_0,
    pub frame: u8_0,
    pub normal: u16_0,
    pub projection: Mtx,
    pub lookAt: Mtx,
    pub texture: *mut libc::c_void,
    pub modelView: [[Mtx; 3]; 2],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PreRender {
    pub width: s32,
    pub height: s32,
    pub widthSave: s32,
    pub heightSave: s32,
    pub fbuf: *mut u16_0,
    pub fbufSave: *mut u16_0,
    pub cvgSave: *mut u8_0,
    pub zbuf: *mut u16_0,
    pub zbufSave: *mut u16_0,
    pub ulxSave: s32,
    pub ulySave: s32,
    pub lrxSave: s32,
    pub lrySave: s32,
    pub ulx: s32,
    pub uly: s32,
    pub lrx: s32,
    pub lry: s32,
    pub alloc: ListAlloc,
    pub unk_4C: u32_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ListAlloc {
    pub prev: *mut ListAlloc,
    pub next: *mut ListAlloc,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CollisionCheckContext {
    pub colATCount: s16,
    pub sacFlags: u16_0,
    pub colAT: [*mut Collider; 50],
    pub colACCount: s32,
    pub colAC: [*mut Collider; 60],
    pub colOCCount: s32,
    pub colOC: [*mut Collider; 50],
    pub colLineCount: s32,
    pub colLine: [*mut OcLine; 3],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct OcLine {
    pub line: Linef,
    pub ocFlags: u16_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Collider {
    pub actor: *mut Actor,
    pub at: *mut Actor,
    pub ac: *mut Actor,
    pub oc: *mut Actor,
    pub atFlags: u8_0,
    pub acFlags: u8_0,
    pub ocFlags1: u8_0,
    pub ocFlags2: u8_0,
    pub colType: u8_0,
    pub shape: u8_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Actor {
    pub id: s16,
    pub category: u8_0,
    pub room: s8,
    pub flags: u32_0,
    pub home: PosRot,
    pub params: s16,
    pub objBankIndex: s8,
    pub targetMode: s8,
    pub sfx: u16_0,
    pub world: PosRot,
    pub focus: PosRot,
    pub targetArrowOffset: f32_0,
    pub scale: Vec3f,
    pub velocity: Vec3f,
    pub speedXZ: f32_0,
    pub gravity: f32_0,
    pub minVelocityY: f32_0,
    pub wallPoly: *mut CollisionPoly,
    pub floorPoly: *mut CollisionPoly,
    pub wallBgId: u8_0,
    pub floorBgId: u8_0,
    pub wallYaw: s16,
    pub floorHeight: f32_0,
    pub yDistToWater: f32_0,
    pub bgCheckFlags: u16_0,
    pub yawTowardsPlayer: s16,
    pub xyzDistToPlayerSq: f32_0,
    pub xzDistToPlayer: f32_0,
    pub yDistToPlayer: f32_0,
    pub colChkInfo: CollisionCheckInfo,
    pub shape: ActorShape,
    pub projectedPos: Vec3f,
    pub projectedW: f32_0,
    pub uncullZoneForward: f32_0,
    pub uncullZoneScale: f32_0,
    pub uncullZoneDownward: f32_0,
    pub prevPos: Vec3f,
    pub isTargeted: u8_0,
    pub targetPriority: u8_0,
    pub textId: u16_0,
    pub freezeTimer: u16_0,
    pub colorFilterParams: u16_0,
    pub colorFilterTimer: u8_0,
    pub isDrawn: u8_0,
    pub dropFlag: u8_0,
    pub naviEnemyId: u8_0,
    pub parent: *mut Actor,
    pub child: *mut Actor,
    pub prev: *mut Actor,
    pub next: *mut Actor,
    pub init: ActorFunc,
    pub destroy: ActorFunc,
    pub update: ActorFunc,
    pub draw: ActorFunc,
    pub overlayEntry: *mut ActorOverlay,
    pub dbgPad: [libc::c_char; 16],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ActorOverlay {
    pub vromStart: u32_0,
    pub vromEnd: u32_0,
    pub vramStart: *mut libc::c_void,
    pub vramEnd: *mut libc::c_void,
    pub loadedRamAddr: *mut libc::c_void,
    pub initInfo: *mut ActorInit,
    pub name: *mut libc::c_char,
    pub allocType: u16_0,
    pub numLoaded: s8,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ActorInit {
    pub id: s16,
    pub category: u8_0,
    pub flags: u32_0,
    pub objectId: s16,
    pub instanceSize: u32_0,
    pub init: ActorFunc,
    pub destroy: ActorFunc,
    pub update: ActorFunc,
    pub draw: ActorFunc,
}
pub type ActorFunc
    =
    Option<unsafe extern "C" fn(_: *mut Actor, _: *mut GlobalContext) -> ()>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ActorShape {
    pub rot: Vec3s,
    pub face: s16,
    pub yOffset: f32_0,
    pub shadowDraw: ActorShadowFunc,
    pub shadowScale: f32_0,
    pub shadowAlpha: u8_0,
    pub feetFloorFlags: u8_0,
    pub feetPos: [Vec3f; 2],
}
pub type ActorShadowFunc
    =
    Option<unsafe extern "C" fn(_: *mut Actor, _: *mut Lights,
                                _: *mut GlobalContext) -> ()>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CollisionCheckInfo {
    pub damageTable: *mut DamageTable,
    pub displacement: Vec3f,
    pub cylRadius: s16,
    pub cylHeight: s16,
    pub cylYShift: s16,
    pub mass: u8_0,
    pub health: u8_0,
    pub damage: u8_0,
    pub damageEffect: u8_0,
    pub atHitEffect: u8_0,
    pub acHitEffect: u8_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct DamageTable {
    pub table: [u8_0; 32],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CollisionPoly {
    pub type_0: u16_0,
    pub c2rust_unnamed: C2RustUnnamed_5,
    pub normal: Vec3s,
    pub dist: s16,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_5 {
    pub vtxData: [u16_0; 3],
    pub c2rust_unnamed: C2RustUnnamed_6,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_6 {
    pub flags_vIA: u16_0,
    pub flags_vIB: u16_0,
    pub vIC: u16_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PosRot {
    pub pos: Vec3f,
    pub rot: Vec3s,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ElfMessage {
    pub byte0: u8_0,
    pub byte1: u8_0,
    pub byte2: u8_0,
    pub byte3: u8_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Path {
    pub count: u8_0,
    pub points: *mut Vec3s,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct EntranceEntry {
    pub spawn: u8_0,
    pub room: u8_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ActorEntry {
    pub id: s16,
    pub pos: Vec3s,
    pub rot: Vec3s,
    pub params: s16,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Player {
    pub actor: Actor,
    pub currentTunic: s8,
    pub currentSword: s8,
    pub currentShield: s8,
    pub currentBoots: s8,
    pub heldItemButton: s8,
    pub heldItemActionParam: s8,
    pub heldItemId: u8_0,
    pub prevBoots: s8,
    pub itemActionParam: s8,
    pub unk_155: [libc::c_char; 3],
    pub modelGroup: u8_0,
    pub nextModelGroup: u8_0,
    pub unk_15A: s8,
    pub modelAnimType: u8_0,
    pub leftHandType: u8_0,
    pub rightHandType: u8_0,
    pub sheathType: u8_0,
    pub currentMask: u8_0,
    pub rightHandDLists: *mut *mut Gfx,
    pub leftHandDLists: *mut *mut Gfx,
    pub sheathDLists: *mut *mut Gfx,
    pub waistDLists: *mut *mut Gfx,
    pub giObjectLoading: u8_0,
    pub giObjectDmaRequest: DmaRequest,
    pub giObjectLoadQueue: OSMesgQueue,
    pub giObjectLoadMsg: OSMesg,
    pub giObjectSegment: *mut libc::c_void,
    pub skelAnime: SkelAnime,
    pub jointTable: [Vec3s; 24],
    pub morphTable: [Vec3s; 24],
    pub blendTable: [Vec3s; 24],
    pub unk_3A8: [s16; 2],
    pub heldActor: *mut Actor,
    pub leftHandPos: Vec3f,
    pub unk_3BC: Vec3s,
    pub unk_3C4: *mut Actor,
    pub unk_3C8: Vec3f,
    pub unk_3D4: [libc::c_char; 88],
    pub doorType: s8,
    pub doorDirection: s8,
    pub doorTimer: s16,
    pub doorActor: *mut Actor,
    pub getItemId: s8,
    pub getItemDirection: u16_0,
    pub interactRangeActor: *mut Actor,
    pub mountSide: s8,
    pub unk_43D: [libc::c_char; 3],
    pub rideActor: *mut Actor,
    pub csMode: u8_0,
    pub prevCsMode: u8_0,
    pub unk_446: u8_0,
    pub unk_447: u8_0,
    pub unk_448: *mut Actor,
    pub unk_44C: [libc::c_char; 4],
    pub unk_450: Vec3f,
    pub unk_45C: Vec3f,
    pub unk_468: [libc::c_char; 2],
    pub unk_46A: s16,
    pub unk_46C: s16,
    pub unk_46E: [libc::c_char; 42],
    pub cylinder: ColliderCylinder,
    pub swordQuads: [ColliderQuad; 2],
    pub shieldQuad: ColliderQuad,
    pub unk_664: *mut Actor,
    pub unk_668: [libc::c_char; 4],
    pub unk_66C: s32,
    pub swordEffectIndex: s32,
    pub func_674: PlayerFunc674,
    pub ageProperties: *mut PlayerAgeProperties,
    pub stateFlags1: u32_0,
    pub stateFlags2: u32_0,
    pub unk_684: *mut Actor,
    pub boomerangActor: *mut Actor,
    pub naviActor: *mut Actor,
    pub naviTextId: s16,
    pub stateFlags3: u8_0,
    pub exchangeItemId: s8,
    pub targetActor: *mut Actor,
    pub targetActorDistance: f32_0,
    pub unk_69C: [libc::c_char; 4],
    pub unk_6A0: f32_0,
    pub unk_6A4: f32_0,
    pub unk_6A8: *mut Actor,
    pub unk_6AC: s8,
    pub unk_6AD: u8_0,
    pub unk_6AE: u16_0,
    pub unk_6B0: s16,
    pub unk_6B4: [libc::c_char; 4],
    pub unk_6B6: s16,
    pub unk_6B8: s16,
    pub unk_6BA: s16,
    pub unk_6BC: s16,
    pub unk_6BE: s16,
    pub unk_6C0: s16,
    pub unk_6C2: s16,
    pub unk_6C4: f32_0,
    pub skelAnime2: SkelAnime,
    pub jointTable2: [Vec3s; 24],
    pub morphTable2: [Vec3s; 24],
    pub func_82C: PlayerFunc82C,
    pub unk_830: f32_0,
    pub unk_834: s16,
    pub unk_836: s8,
    pub unk_837: u8_0,
    pub linearVelocity: f32_0,
    pub currentYaw: s16,
    pub targetYaw: s16,
    pub unk_840: u16_0,
    pub swordAnimation: s8,
    pub swordState: s8,
    pub unk_844: s8,
    pub unk_845: u8_0,
    pub unk_846: u8_0,
    pub unk_847: [s8; 4],
    pub unk_84B: [s8; 4],
    pub unk_84F: s8,
    pub unk_850: s16,
    pub unk_854: f32_0,
    pub unk_858: f32_0,
    pub unk_85C: f32_0,
    pub unk_860: s16,
    pub unk_862: s8,
    pub unk_864: f32_0,
    pub unk_868: f32_0,
    pub unk_86C: f32_0,
    pub unk_870: f32_0,
    pub unk_874: f32_0,
    pub unk_878: f32_0,
    pub unk_87C: s16,
    pub unk_87E: s16,
    pub unk_880: f32_0,
    pub wallHeight: f32_0,
    pub wallDistance: f32_0,
    pub unk_88C: u8_0,
    pub unk_88D: u8_0,
    pub unk_88E: u8_0,
    pub unk_88F: u8_0,
    pub unk_890: u8_0,
    pub shockTimer: u8_0,
    pub unk_892: u8_0,
    pub hoverBootsTimer: u8_0,
    pub fallStartHeight: s16,
    pub fallDistance: s16,
    pub unk_898: s16,
    pub unk_89A: s16,
    pub unk_89C: s16,
    pub unk_89E: u16_0,
    pub unk_8A0: u8_0,
    pub unk_8A1: u8_0,
    pub unk_8A2: s16,
    pub unk_8A4: f32_0,
    pub unk_8A8: f32_0,
    pub windSpeed: f32_0,
    pub windDirection: s16,
    pub swordInfo: [WeaponInfo; 3],
    pub bodyPartsPos: [Vec3f; 18],
    pub mf_9E0: MtxF,
    pub shieldMf: MtxF,
    pub isBurning: u8_0,
    pub flameTimers: [u8_0; 18],
    pub unk_A73: u8_0,
    pub func_A74: PlayerFuncA74,
    pub invincibilityTimer: s8,
    pub unk_A79: u8_0,
    pub unk_A7A: u8_0,
    pub unk_A7B: u8_0,
    pub unk_A7C: f32_0,
    pub unk_A80: s16,
    pub unk_A82: u16_0,
    pub unk_A84: s16,
    pub unk_A86: s8,
    pub unk_A87: u8_0,
    pub unk_A88: Vec3f,
}
pub type PlayerFuncA74
    =
    Option<unsafe extern "C" fn(_: *mut GlobalContext, _: *mut Player) -> ()>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct WeaponInfo {
    pub active: s32,
    pub tip: Vec3f,
    pub base: Vec3f,
}
pub type PlayerFunc82C
    =
    Option<unsafe extern "C" fn(_: *mut Player, _: *mut GlobalContext)
               -> s32>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SkelAnime {
    pub limbCount: u8_0,
    pub mode: u8_0,
    pub dListCount: u8_0,
    pub taper: s8,
    pub skeleton: *mut *mut libc::c_void,
    pub animation: *mut libc::c_void,
    pub startFrame: f32_0,
    pub endFrame: f32_0,
    pub animLength: f32_0,
    pub curFrame: f32_0,
    pub playSpeed: f32_0,
    pub jointTable: *mut Vec3s,
    pub morphTable: *mut Vec3s,
    pub morphWeight: f32_0,
    pub morphRate: f32_0,
    pub update: Option<unsafe extern "C" fn() -> s32>,
    pub initFlags: s8,
    pub moveFlags: u8_0,
    pub prevRot: s16,
    pub prevTransl: Vec3s,
    pub baseTransl: Vec3s,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PlayerAgeProperties {
    pub unk_00: f32_0,
    pub unk_04: f32_0,
    pub unk_08: f32_0,
    pub unk_0C: f32_0,
    pub unk_10: f32_0,
    pub unk_14: f32_0,
    pub unk_18: f32_0,
    pub unk_1C: f32_0,
    pub unk_20: f32_0,
    pub unk_24: f32_0,
    pub unk_28: f32_0,
    pub unk_2C: f32_0,
    pub unk_30: f32_0,
    pub unk_34: f32_0,
    pub unk_38: f32_0,
    pub unk_3C: f32_0,
    pub unk_40: f32_0,
    pub unk_44: Vec3s,
    pub unk_4A: [Vec3s; 4],
    pub unk_62: [Vec3s; 4],
    pub unk_7A: [Vec3s; 2],
    pub unk_86: [Vec3s; 2],
    pub unk_92: u16_0,
    pub unk_94: u16_0,
    pub unk_98: *mut LinkAnimationHeader,
    pub unk_9C: *mut LinkAnimationHeader,
    pub unk_A0: *mut LinkAnimationHeader,
    pub unk_A4: *mut LinkAnimationHeader,
    pub unk_A8: *mut LinkAnimationHeader,
    pub unk_AC: [*mut LinkAnimationHeader; 4],
    pub unk_BC: [*mut LinkAnimationHeader; 2],
    pub unk_C4: [*mut LinkAnimationHeader; 2],
    pub unk_CC: [*mut LinkAnimationHeader; 2],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct LinkAnimationHeader {
    pub common: AnimationHeaderCommon,
    pub segment: *mut libc::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AnimationHeaderCommon {
    pub frameCount: s16,
}
pub type PlayerFunc674
    =
    Option<unsafe extern "C" fn(_: *mut Player, _: *mut GlobalContext) -> ()>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ColliderQuad {
    pub base: Collider,
    pub info: ColliderInfo,
    pub dim: ColliderQuadDim,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ColliderQuadDim {
    pub quad: [Vec3f; 4],
    pub dcMid: Vec3s,
    pub baMid: Vec3s,
    pub acDist: f32_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ColliderInfo {
    pub toucher: ColliderTouch,
    pub bumper: ColliderBump,
    pub elemType: u8_0,
    pub toucherFlags: u8_0,
    pub bumperFlags: u8_0,
    pub ocElemFlags: u8_0,
    pub atHit: *mut Collider,
    pub acHit: *mut Collider,
    pub atHitInfo: *mut ColliderInfo,
    pub acHitInfo: *mut ColliderInfo,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ColliderBump {
    pub dmgFlags: u32_0,
    pub effect: u8_0,
    pub defense: u8_0,
    pub hitPos: Vec3s,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ColliderTouch {
    pub dmgFlags: u32_0,
    pub effect: u8_0,
    pub damage: u8_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ColliderCylinder {
    pub base: Collider,
    pub info: ColliderInfo,
    pub dim: Cylinder16,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct DmaRequest {
    pub vromAddr: u32_0,
    pub dramAddr: *mut libc::c_void,
    pub size: u32_0,
    pub filename: *const libc::c_char,
    pub line: s32,
    pub unk_14: s32,
    pub notifyQueue: *mut OSMesgQueue,
    pub notifyMsg: OSMesg,
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
pub struct FlexSkeletonHeader {
    pub sh: SkeletonHeader,
    pub dListCount: u8_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SkeletonHeader {
    pub segment: *mut *mut libc::c_void,
    pub limbCount: u8_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct TransitionActorContext {
    pub numActors: u8_0,
    pub list: *mut TransitionActorEntry,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct TransitionActorEntry {
    pub sides: [C2RustUnnamed_7; 2],
    pub id: s16,
    pub pos: Vec3s,
    pub rotY: s16,
    pub params: s16,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_7 {
    pub room: s8,
    pub effects: s8,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct RoomContext {
    pub curRoom: Room,
    pub prevRoom: Room,
    pub bufPtrs: [*mut libc::c_void; 2],
    pub unk_30: u8_0,
    pub status: s8,
    pub unk_34: *mut libc::c_void,
    pub dmaRequest: DmaRequest,
    pub loadQueue: OSMesgQueue,
    pub loadMsg: OSMesg,
    pub unk_74: [s16; 2],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Room {
    pub num: s8,
    pub unk_01: u8_0,
    pub unk_02: u8_0,
    pub unk_03: u8_0,
    pub echo: s8,
    pub showInvisActors: u8_0,
    pub mesh: *mut Mesh,
    pub segment: *mut libc::c_void,
    pub unk_10: [libc::c_char; 4],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union Mesh {
    pub polygon: Polygon,
    pub polygon0: PolygonType0,
    pub polygon1: PolygonType1,
    pub polygon2: PolygonType2,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PolygonType2 {
    pub type_0: u8_0,
    pub num: u8_0,
    pub start: *mut libc::c_void,
    pub end: *mut libc::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PolygonType1 {
    pub type_0: u8_0,
    pub format: u8_0,
    pub dlist: *mut Gfx,
    pub c2rust_unnamed: C2RustUnnamed_8,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_8 {
    pub single: C2RustUnnamed_10,
    pub multi: C2RustUnnamed_9,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_9 {
    pub count: u8_0,
    pub list: *mut BgImage,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct BgImage {
    pub unk_00: u16_0,
    pub id: u8_0,
    pub source: u32_0,
    pub unk_0C: u32_0,
    pub tlut: u32_0,
    pub width: u16_0,
    pub height: u16_0,
    pub fmt: u8_0,
    pub siz: u8_0,
    pub mode0: u16_0,
    pub tlutCount: u16_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_10 {
    pub source: *mut libc::c_void,
    pub unk_0C: u32_0,
    pub tlut: *mut libc::c_void,
    pub width: u16_0,
    pub height: u16_0,
    pub fmt: u8_0,
    pub siz: u8_0,
    pub mode0: u16_0,
    pub tlutCount: u16_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PolygonType0 {
    pub type_0: u8_0,
    pub num: u8_0,
    pub start: *mut libc::c_void,
    pub end: *mut libc::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Polygon {
    pub type_0: u8_0,
    pub num: u8_0,
    pub start: *mut libc::c_void,
    pub end: *mut libc::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ObjectContext {
    pub spaceStart: *mut libc::c_void,
    pub spaceEnd: *mut libc::c_void,
    pub num: u8_0,
    pub unk_09: u8_0,
    pub mainKeepIndex: u8_0,
    pub subKeepIndex: u8_0,
    pub status: [ObjectStatus; 19],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ObjectStatus {
    pub id: s16,
    pub segment: *mut libc::c_void,
    pub dmaRequest: DmaRequest,
    pub loadQueue: OSMesgQueue,
    pub loadMsg: OSMesg,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AnimationContext {
    pub animationCount: s16,
    pub entries: [AnimationEntry; 50],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AnimationEntry {
    pub type_0: u8_0,
    pub data: AnimationEntryData,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union AnimationEntryData {
    pub load: AnimEntryLoadFrame,
    pub copy: AnimEntryCopyAll,
    pub interp: AnimEntryInterp,
    pub copy1: AnimEntryCopyTrue,
    pub copy0: AnimEntryCopyFalse,
    pub move_0: AnimEntryMoveActor,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AnimEntryMoveActor {
    pub actor: *mut Actor,
    pub skelAnime: *mut SkelAnime,
    pub unk_08: f32_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AnimEntryCopyFalse {
    pub queueFlag: u8_0,
    pub vecCount: u8_0,
    pub dst: *mut Vec3s,
    pub src: *mut Vec3s,
    pub copyFlag: *mut u8_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AnimEntryCopyTrue {
    pub queueFlag: u8_0,
    pub vecCount: u8_0,
    pub dst: *mut Vec3s,
    pub src: *mut Vec3s,
    pub copyFlag: *mut u8_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AnimEntryInterp {
    pub queueFlag: u8_0,
    pub vecCount: u8_0,
    pub base: *mut Vec3s,
    pub mod_0: *mut Vec3s,
    pub weight: f32_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AnimEntryCopyAll {
    pub queueFlag: u8_0,
    pub vecCount: u8_0,
    pub dst: *mut Vec3s,
    pub src: *mut Vec3s,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AnimEntryLoadFrame {
    pub req: DmaRequest,
    pub msgQueue: OSMesgQueue,
    pub msg: OSMesg,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct EnvironmentContext {
    pub unk_00: [libc::c_char; 2],
    pub timeIncrement: u16_0,
    pub sunPos: Vec3f,
    pub skybox1Index: u8_0,
    pub skybox2Index: u8_0,
    pub unk_12: [libc::c_char; 1],
    pub skyboxBlend: u8_0,
    pub unk_14: [libc::c_char; 1],
    pub skyboxDisabled: u8_0,
    pub sunMoonDisabled: u8_0,
    pub unk_17: u8_0,
    pub unk_18: u8_0,
    pub unk_19: u8_0,
    pub unk_1A: u16_0,
    pub unk_1C: [libc::c_char; 2],
    pub indoors: u8_0,
    pub unk_1F: u8_0,
    pub unk_20: u8_0,
    pub unk_21: u8_0,
    pub unk_22: u16_0,
    pub unk_24: u16_0,
    pub unk_26: [libc::c_char; 2],
    pub dirLight1: LightInfo,
    pub dirLight2: LightInfo,
    pub skyboxDmaState: s8,
    pub dmaRequest: DmaRequest,
    pub loadQueue: OSMesgQueue,
    pub loadMsg: OSMesg,
    pub unk_84: f32_0,
    pub unk_88: f32_0,
    pub adjAmbientColor: [s16; 3],
    pub adjLight1Color: [s16; 3],
    pub adjFogColor: [s16; 3],
    pub adjFogNear: s16,
    pub adjFogFar: s16,
    pub unk_A2: [libc::c_char; 6],
    pub windDirection: Vec3s,
    pub windSpeed: f32_0,
    pub numLightSettings: u8_0,
    pub lightSettingsList: *mut EnvLightSettings,
    pub blendIndoorLights: u8_0,
    pub unk_BD: u8_0,
    pub unk_BE: u8_0,
    pub unk_BF: u8_0,
    pub lightSettings: EnvLightSettings,
    pub unk_D6: u16_0,
    pub unk_D8: f32_0,
    pub unk_DC: u8_0,
    pub gloomySkyMode: u8_0,
    pub unk_DE: u8_0,
    pub lightningMode: u8_0,
    pub unk_E0: u8_0,
    pub fillScreen: u8_0,
    pub screenFillColor: [u8_0; 4],
    pub sandstormState: u8_0,
    pub sandstormPrimA: u8_0,
    pub sandstormEnvA: u8_0,
    pub customSkyboxFilter: u8_0,
    pub skyboxFilterColor: [u8_0; 4],
    pub unk_EE: [u8_0; 4],
    pub unk_F2: [u8_0; 4],
    pub unk_F6: [libc::c_char; 6],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct EnvLightSettings {
    pub ambientColor: [u8_0; 3],
    pub light1Dir: [s8; 3],
    pub light1Color: [u8_0; 3],
    pub light2Dir: [s8; 3],
    pub light2Color: [u8_0; 3],
    pub fogColor: [u8_0; 3],
    pub fogNear: s16,
    pub fogFar: s16,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct GameOverContext {
    pub state: u16_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PauseContext {
    pub view: View,
    pub iconItemSegment: *mut u8_0,
    pub iconItem24Segment: *mut u8_0,
    pub iconItemAltSegment: *mut u8_0,
    pub iconItemLangSegment: *mut u8_0,
    pub nameSegment: *mut u8_0,
    pub playerSegment: *mut u8_0,
    pub unk_140: [libc::c_char; 4],
    pub itemPageVtx: *mut Vtx,
    pub equipPageVtx: *mut Vtx,
    pub mapPageVtx: *mut Vtx,
    pub questPageVtx: *mut Vtx,
    pub infoPanelVtx: *mut Vtx,
    pub itemVtx: *mut Vtx,
    pub equipVtx: *mut Vtx,
    pub unk_160: [libc::c_char; 4],
    pub questVtx: *mut Vtx,
    pub cursorVtx: *mut Vtx,
    pub saveVtx: *mut Vtx,
    pub unk_170: [libc::c_char; 36],
    pub ocarinaStaff: *mut OcarinaStaff,
    pub unk_198: [libc::c_char; 32],
    pub loadQueue: OSMesgQueue,
    pub loadMsg: OSMesg,
    pub state: u16_0,
    pub debugState: u16_0,
    pub eye: Vec3f,
    pub unk_1E4: u16_0,
    pub mode: u16_0,
    pub pageIndex: u16_0,
    pub unk_1EA: u16_0,
    pub unk_1EC: u16_0,
    pub unk_1F0: f32_0,
    pub unk_1F4: f32_0,
    pub unk_1F8: f32_0,
    pub unk_1FC: f32_0,
    pub unk_200: f32_0,
    pub unk_204: f32_0,
    pub alpha: u16_0,
    pub offsetY: s16,
    pub unk_20C: [libc::c_char; 8],
    pub stickRelX: s16,
    pub stickRelY: s16,
    pub cursorPoint: [s16; 5],
    pub cursorX: [s16; 5],
    pub cursorY: [s16; 5],
    pub dungeonMapSlot: s16,
    pub cursorSpecialPos: s16,
    pub pageSwitchTimer: s16,
    pub namedItem: u16_0,
    pub cursorItem: [u16_0; 4],
    pub cursorSlot: [u16_0; 4],
    pub equipTargetItem: u16_0,
    pub equipTargetSlot: u16_0,
    pub equipTargetCBtn: u16_0,
    pub equipAnimX: s16,
    pub equipAnimY: s16,
    pub equipAnimAlpha: s16,
    pub infoPanelOffsetY: s16,
    pub nameDisplayTimer: u16_0,
    pub nameColorSet: u16_0,
    pub cursorColorSet: s16,
    pub promptChoice: s16,
    pub ocarinaSongIdx: s16,
    pub worldMapPoints: [u8_0; 20],
    pub tradeQuestLocation: u8_0,
    pub playerSkelAnime: SkelAnime,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct OcarinaStaff {
    pub noteIdx: u8_0,
    pub state: u8_0,
    pub pos: u8_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct View {
    pub magic: s32,
    pub gfxCtx: *mut GraphicsContext,
    pub viewport: Viewport,
    pub fovy: f32_0,
    pub zNear: f32_0,
    pub zFar: f32_0,
    pub scale: f32_0,
    pub eye: Vec3f,
    pub lookAt: Vec3f,
    pub up: Vec3f,
    pub vp: Vp,
    pub projection: Mtx,
    pub viewing: Mtx,
    pub projectionPtr: *mut Mtx,
    pub viewingPtr: *mut Mtx,
    pub unk_E8: Vec3f,
    pub unk_F4: Vec3f,
    pub unk_100: f32_0,
    pub unk_104: Vec3f,
    pub unk_110: Vec3f,
    pub normal: u16_0,
    pub flags: s32,
    pub unk_124: s32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Viewport {
    pub topY: s32,
    pub bottomY: s32,
    pub leftX: s32,
    pub rightX: s32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct GraphicsContext {
    pub polyOpaBuffer: *mut Gfx,
    pub polyXluBuffer: *mut Gfx,
    pub unk_008: [libc::c_char; 8],
    pub overlayBuffer: *mut Gfx,
    pub unk_014: u32_0,
    pub unk_018: [libc::c_char; 32],
    pub msgBuff: [OSMesg; 8],
    pub schedMsgQ: *mut OSMesgQueue,
    pub queue: OSMesgQueue,
    pub unk_074: [libc::c_char; 4],
    pub task: OSScTask,
    pub unk_0D0: [libc::c_char; 224],
    pub workBuffer: *mut Gfx,
    pub work: TwoHeadGfxArena,
    pub unk_01C4: [libc::c_char; 192],
    pub viMode: *mut OSViMode,
    pub unk_0288: [libc::c_char; 32],
    pub overlay: TwoHeadGfxArena,
    pub polyOpa: TwoHeadGfxArena,
    pub polyXlu: TwoHeadGfxArena,
    pub gfxPoolIdx: u32_0,
    pub curFrameBuffer: *mut u16_0,
    pub unk_2E0: [libc::c_char; 4],
    pub viFeatures: u32_0,
    pub fbIdx: s32,
    pub callback: Option<unsafe extern "C" fn(_: *mut GraphicsContext,
                                              _: *mut libc::c_void) -> ()>,
    pub callbackParam: *mut libc::c_void,
    pub xScale: f32_0,
    pub yScale: f32_0,
    pub unk_2FC: [libc::c_char; 4],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct TwoHeadGfxArena {
    pub size: u32_0,
    pub bufp: *mut Gfx,
    pub p: *mut Gfx,
    pub d: *mut Gfx,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct OSScTask {
    pub next: *mut OSScTask,
    pub state: u32_0,
    pub flags: u32_0,
    pub framebuffer: *mut CfbInfo,
    pub list: OSTask,
    pub msgQ: *mut OSMesgQueue,
    pub msg: OSMesg,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CfbInfo {
    pub fb1: *mut u16_0,
    pub swapBuffer: *mut u16_0,
    pub viMode: *mut OSViMode,
    pub features: u32_0,
    pub unk_10: u8_0,
    pub updateRate: s8,
    pub updateRate2: s8,
    pub unk_13: u8_0,
    pub xScale: f32_0,
    pub yScale: f32_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct InterfaceContext {
    pub view: View,
    pub actionVtx: *mut Vtx,
    pub beatingHeartVtx: *mut Vtx,
    pub parameterSegment: *mut u8_0,
    pub doActionSegment: *mut u8_0,
    pub iconItemSegment: *mut u8_0,
    pub mapSegment: *mut u8_0,
    pub mapPalette: [u8_0; 32],
    pub dmaRequest_160: DmaRequest,
    pub dmaRequest_180: DmaRequest,
    pub unk_1A0: [libc::c_char; 32],
    pub loadQueue: OSMesgQueue,
    pub loadMsg: OSMesg,
    pub viewport: Viewport,
    pub unk_1EC: s16,
    pub unk_1EE: u16_0,
    pub unk_1F0: u16_0,
    pub unk_1F4: f32_0,
    pub naviCalling: s16,
    pub unk_1FA: s16,
    pub unk_1FC: s16,
    pub unk_1FE: s16,
    pub unk_200: s16,
    pub beatingHeartPrim: [s16; 3],
    pub beatingHeartEnv: [s16; 3],
    pub heartsPrimR: [s16; 2],
    pub heartsPrimG: [s16; 2],
    pub heartsPrimB: [s16; 2],
    pub heartsEnvR: [s16; 2],
    pub heartsEnvG: [s16; 2],
    pub heartsEnvB: [s16; 2],
    pub unk_226: s16,
    pub unk_228: s16,
    pub unk_22A: s16,
    pub unk_22C: s16,
    pub unk_22E: s16,
    pub unk_230: s16,
    pub counterDigits: [s16; 4],
    pub numHorseBoosts: u8_0,
    pub unk_23C: u16_0,
    pub hbaAmmo: u16_0,
    pub unk_240: u16_0,
    pub unk_242: u16_0,
    pub unk_244: u16_0,
    pub aAlpha: u16_0,
    pub bAlpha: u16_0,
    pub cLeftAlpha: u16_0,
    pub cDownAlpha: u16_0,
    pub cRightAlpha: u16_0,
    pub healthAlpha: u16_0,
    pub magicAlpha: u16_0,
    pub minimapAlpha: u16_0,
    pub startAlpha: s16,
    pub unk_258: s16,
    pub unk_25A: s16,
    pub mapRoomNum: s16,
    pub mapPaletteIndex: s16,
    pub unk_260: u8_0,
    pub unk_261: u8_0,
    pub restrictions: C2RustUnnamed_11,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_11 {
    pub hGauge: u8_0,
    pub bButton: u8_0,
    pub aButton: u8_0,
    pub bottles: u8_0,
    pub tradeItems: u8_0,
    pub hookshot: u8_0,
    pub ocarina: u8_0,
    pub warpSongs: u8_0,
    pub sunsSong: u8_0,
    pub farores: u8_0,
    pub dinsNayrus: u8_0,
    pub all: u8_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct MessageContext {
    pub view: View,
    pub font: Font,
    pub textboxSegment: *mut libc::c_void,
    pub unk_E2B4: [libc::c_char; 4],
    pub ocarinaStaff: *mut OcarinaStaff,
    pub unk_E2BC: [libc::c_char; 60],
    pub textId: u16_0,
    pub choiceTextId: u16_0,
    pub textBoxProperties: u8_0,
    pub textBoxType: u8_0,
    pub textBoxPos: u8_0,
    pub msgLength: s32,
    pub msgMode: u8_0,
    pub unk_E305: [libc::c_char; 1],
    pub msgBufDecoded: [u8_0; 200],
    pub msgBufPos: u16_0,
    pub unk_E3D0: u16_0,
    pub textDrawPos: u16_0,
    pub decodedTextLen: u16_0,
    pub textUnskippable: u16_0,
    pub textPosX: s16,
    pub textPosY: s16,
    pub textColorR: s16,
    pub textColorG: s16,
    pub textColorB: s16,
    pub textColorAlpha: s16,
    pub textboxEndType: u8_0,
    pub choiceIndex: u8_0,
    pub choiceNum: u8_0,
    pub stateTimer: u8_0,
    pub textDelayTimer: u16_0,
    pub textDelay: u16_0,
    pub lastPlayedSong: u16_0,
    pub ocarinaMode: u16_0,
    pub ocarinaAction: u16_0,
    pub unk_E3F2: u16_0,
    pub unk_E3F4: u16_0,
    pub textboxBackgroundIdx: u16_0,
    pub textboxBackgroundForeColorIdx: u8_0,
    pub textboxBackgroundBackColorIdx: u8_0,
    pub textboxBackgroundYOffsetIdx: u8_0,
    pub textboxBackgroundUnkArg: u8_0,
    pub unk_E3FC: [libc::c_char; 2],
    pub textboxColorRed: s16,
    pub textboxColorGreen: s16,
    pub textboxColorBlue: s16,
    pub textboxColorAlphaTarget: s16,
    pub textboxColorAlphaCurrent: s16,
    pub talkActor: *mut Actor,
    pub disableWarpSongs: s16,
    pub unk_E40E: s16,
    pub lastOcaNoteIdx: u8_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Font {
    pub msgOffset: u32_0,
    pub msgLength: u32_0,
    pub charTexBuf: [u8_0; 15360],
    pub iconBuf: [u8_0; 128],
    pub fontBuf: [u8_0; 40960],
    pub c2rust_unnamed: C2RustUnnamed_12,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_12 {
    pub msgBuf: [libc::c_char; 1280],
    pub msgBufWide: [u16_0; 640],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SkyboxContext {
    pub unk_00: [libc::c_char; 296],
    pub staticSegments: [*mut libc::c_void; 2],
    pub palettes: *mut [u16_0; 256],
    pub dListBuf: *mut [Gfx; 150],
    pub unk_138: *mut Gfx,
    pub roomVtx: *mut Vtx,
    pub unk_140: s16,
    pub rot: Vec3f,
    pub unk_150: [libc::c_char; 16],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SramContext {
    pub readBuff: *mut u8_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SoundSource {
    pub countdown: u16_0,
    pub originPos: Vec3f,
    pub relativePos: Vec3f,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CutsceneContext {
    pub unk_00: [libc::c_char; 4],
    pub segment: *mut libc::c_void,
    pub state: u8_0,
    pub unk_0C: f32_0,
    pub frames: u16_0,
    pub unk_12: u16_0,
    pub unk_14: s32,
    pub unk_18: u16_0,
    pub unk_1A: u8_0,
    pub unk_1B: u8_0,
    pub cameraFocus: *mut CutsceneCameraPoint,
    pub cameraPosition: *mut CutsceneCameraPoint,
    pub linkAction: *mut CsCmdActorAction,
    pub npcActions: [*mut CsCmdActorAction; 10],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CsCmdActorAction {
    pub action: u16_0,
    pub startFrame: u16_0,
    pub endFrame: u16_0,
    pub c2rust_unnamed: C2RustUnnamed_13,
    pub startPos: Vec3i,
    pub endPos: Vec3i,
    pub normal: Vec3i,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_13 {
    pub rot: Vec3s,
    pub urot: Vec3us,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CutsceneCameraPoint {
    pub continueFlag: s8,
    pub cameraRoll: s8,
    pub nextPointFrame: u16_0,
    pub viewAngle: f32_0,
    pub pos: Vec3s,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ActorContext {
    pub freezeFlashTimer: u8_0,
    pub unk_01: [libc::c_char; 1],
    pub unk_02: u8_0,
    pub unk_03: u8_0,
    pub unk_04: [libc::c_char; 4],
    pub total: u8_0,
    pub unk_09: [libc::c_char; 3],
    pub actorLists: [ActorListEntry; 12],
    pub targetCtx: TargetContext,
    pub flags: C2RustUnnamed_14,
    pub titleCtx: TitleCardContext,
    pub unk_138: [libc::c_char; 4],
    pub absoluteSpace: *mut libc::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct TitleCardContext {
    pub texture: *mut libc::c_void,
    pub x: s16,
    pub y: s16,
    pub width: u8_0,
    pub height: u8_0,
    pub durationTimer: u8_0,
    pub delayTimer: u8_0,
    pub alpha: s16,
    pub intensity: s16,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_14 {
    pub swch: u32_0,
    pub tempSwch: u32_0,
    pub unk0: u32_0,
    pub unk1: u32_0,
    pub chest: u32_0,
    pub clear: u32_0,
    pub tempClear: u32_0,
    pub collect: u32_0,
    pub tempCollect: u32_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct TargetContext {
    pub naviRefPos: Vec3f,
    pub targetCenterPos: Vec3f,
    pub naviInner: Color_RGBAf,
    pub naviOuter: Color_RGBAf,
    pub arrowPointedActor: *mut Actor,
    pub targetedActor: *mut Actor,
    pub unk_40: f32_0,
    pub unk_44: f32_0,
    pub unk_48: s16,
    pub activeCategory: u8_0,
    pub unk_4B: u8_0,
    pub unk_4C: s8,
    pub unk_4D: [libc::c_char; 3],
    pub arr_50: [TargetContextEntry; 3],
    pub unk_8C: *mut Actor,
    pub bgmEnemy: *mut Actor,
    pub unk_94: *mut Actor,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct TargetContextEntry {
    pub pos: Vec3f,
    pub unk_0C: f32_0,
    pub color: Color_RGB8,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ActorListEntry {
    pub length: s32,
    pub head: *mut Actor,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CollisionContext {
    pub colHeader: *mut CollisionHeader,
    pub minBounds: Vec3f,
    pub maxBounds: Vec3f,
    pub subdivAmount: Vec3i,
    pub subdivLength: Vec3f,
    pub subdivLengthInv: Vec3f,
    pub lookupTbl: *mut StaticLookup,
    pub polyNodes: SSNodeList,
    pub dyna: DynaCollisionContext,
    pub memSize: u32_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct DynaCollisionContext {
    pub bitFlag: u8_0,
    pub bgActors: [BgActor; 50],
    pub bgActorFlags: [u16_0; 50],
    pub polyList: *mut CollisionPoly,
    pub vtxList: *mut Vec3s,
    pub polyNodes: DynaSSNodeList,
    pub polyNodesMax: s32,
    pub polyListMax: s32,
    pub vtxListMax: s32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct DynaSSNodeList {
    pub tbl: *mut SSNode,
    pub count: s32,
    pub max: s32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SSNode {
    pub polyId: s16,
    pub next: u16_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct BgActor {
    pub actor: *mut Actor,
    pub colHeader: *mut CollisionHeader,
    pub dynaLookup: DynaLookup,
    pub vtxStartIndex: u16_0,
    pub prevTransform: ScaleRotPos,
    pub curTransform: ScaleRotPos,
    pub boundingSphere: Sphere16,
    pub minY: f32_0,
    pub maxY: f32_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ScaleRotPos {
    pub scale: Vec3f,
    pub rot: Vec3s,
    pub pos: Vec3f,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct DynaLookup {
    pub polyStartIndex: u16_0,
    pub ceiling: SSList,
    pub wall: SSList,
    pub floor: SSList,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SSList {
    pub head: u16_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CollisionHeader {
    pub minBounds: Vec3s,
    pub maxBounds: Vec3s,
    pub numVertices: u16_0,
    pub vtxList: *mut Vec3s,
    pub numPolygons: u16_0,
    pub polyList: *mut CollisionPoly,
    pub surfaceTypeList: *mut SurfaceType,
    pub cameraDataList: *mut CamData,
    pub numWaterBoxes: u16_0,
    pub waterBoxes: *mut WaterBox,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct WaterBox {
    pub xMin: s16,
    pub ySurface: s16,
    pub zMin: s16,
    pub xLength: s16,
    pub zLength: s16,
    pub properties: u32_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CamData {
    pub cameraSType: u16_0,
    pub numCameras: s16,
    pub camPosData: *mut Vec3s,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SurfaceType {
    pub data: [u32_0; 2],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SSNodeList {
    pub max: u16_0,
    pub count: u16_0,
    pub tbl: *mut SSNode,
    pub polyCheckTbl: *mut u8_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct StaticLookup {
    pub floor: SSList,
    pub wall: SSList,
    pub ceiling: SSList,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct FrameAdvanceContext {
    pub enabled: s32,
    pub timer: s32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SequenceContext {
    pub seqId: u8_0,
    pub natureAmbienceId: u8_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Camera {
    pub paramData: [libc::c_char; 80],
    pub at: Vec3f,
    pub eye: Vec3f,
    pub up: Vec3f,
    pub eyeNext: Vec3f,
    pub skyboxOffset: Vec3f,
    pub globalCtx: *mut GlobalContext,
    pub player: *mut Player,
    pub playerPosRot: PosRot,
    pub target: *mut Actor,
    pub targetPosRot: PosRot,
    pub rUpdateRateInv: f32_0,
    pub pitchUpdateRateInv: f32_0,
    pub yawUpdateRateInv: f32_0,
    pub xzOffsetUpdateRate: f32_0,
    pub yOffsetUpdateRate: f32_0,
    pub fovUpdateRate: f32_0,
    pub xzSpeed: f32_0,
    pub dist: f32_0,
    pub speedRatio: f32_0,
    pub posOffset: Vec3f,
    pub playerPosDelta: Vec3f,
    pub fov: f32_0,
    pub atLERPStepScale: f32_0,
    pub playerGroundY: f32_0,
    pub floorNorm: Vec3f,
    pub waterYPos: f32_0,
    pub waterPrevCamIdx: s32,
    pub waterPrevCamSetting: s32,
    pub waterQuakeId: s32,
    pub data0: *mut libc::c_void,
    pub data1: *mut libc::c_void,
    pub data2: s16,
    pub data3: s16,
    pub uid: s16,
    pub unk_132: [libc::c_char; 2],
    pub inputDir: Vec3s,
    pub camDir: Vec3s,
    pub status: s16,
    pub setting: s16,
    pub mode: s16,
    pub bgCheckId: s16,
    pub camDataIdx: s16,
    pub unk_14A: s16,
    pub unk_14C: s16,
    pub childCamIdx: s16,
    pub unk_150: s16,
    pub unk_152: s16,
    pub prevSetting: s16,
    pub nextCamDataIdx: s16,
    pub nextBGCheckId: s16,
    pub roll: s16,
    pub paramFlags: s16,
    pub animState: s16,
    pub timer: s16,
    pub parentCamIdx: s16,
    pub thisIdx: s16,
    pub prevCamDataIdx: s16,
    pub csId: s16,
    pub unk_16A: s16,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct GameState {
    pub gfxCtx: *mut GraphicsContext,
    pub main: GameStateFunc,
    pub destroy: GameStateFunc,
    pub init: GameStateFunc,
    pub size: u32_0,
    pub input: [Input; 4],
    pub tha: TwoHeadArena,
    pub alloc: GameAlloc,
    pub running: u32_0,
    pub frames: u32_0,
    pub unk_A0: u32_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct GameAlloc {
    pub base: GameAllocEntry,
    pub head: *mut GameAllocEntry,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct GameAllocEntry {
    pub next: *mut GameAllocEntry,
    pub prev: *mut GameAllocEntry,
    pub size: u32_0,
    pub unk_0C: u32_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct TwoHeadArena {
    pub size: u32_0,
    pub bufp: *mut libc::c_void,
    pub head: *mut libc::c_void,
    pub tail: *mut libc::c_void,
}
pub type GameStateFunc
    =
    Option<unsafe extern "C" fn(_: *mut GameState) -> ()>;
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
pub type C2RustUnnamed_15 = libc::c_uint;
pub const OCARINA_MODE_0F: C2RustUnnamed_15 = 15;
pub const OCARINA_MODE_0E: C2RustUnnamed_15 = 14;
pub const OCARINA_MODE_0D: C2RustUnnamed_15 = 13;
pub const OCARINA_MODE_0C: C2RustUnnamed_15 = 12;
pub const OCARINA_MODE_0B: C2RustUnnamed_15 = 11;
pub const OCARINA_MODE_0A: C2RustUnnamed_15 = 10;
pub const OCARINA_MODE_09: C2RustUnnamed_15 = 9;
pub const OCARINA_MODE_08: C2RustUnnamed_15 = 8;
pub const OCARINA_MODE_07: C2RustUnnamed_15 = 7;
pub const OCARINA_MODE_06: C2RustUnnamed_15 = 6;
pub const OCARINA_MODE_05: C2RustUnnamed_15 = 5;
pub const OCARINA_MODE_04: C2RustUnnamed_15 = 4;
pub const OCARINA_MODE_03: C2RustUnnamed_15 = 3;
pub const OCARINA_MODE_02: C2RustUnnamed_15 = 2;
pub const OCARINA_MODE_01: C2RustUnnamed_15 = 1;
pub const OCARINA_MODE_00: C2RustUnnamed_15 = 0;
pub type C2RustUnnamed_16 = libc::c_uint;
pub const LANGUAGE_MAX: C2RustUnnamed_16 = 3;
pub const LANGUAGE_FRA: C2RustUnnamed_16 = 2;
pub const LANGUAGE_GER: C2RustUnnamed_16 = 1;
pub const LANGUAGE_ENG: C2RustUnnamed_16 = 0;
pub type C2RustUnnamed_17 = libc::c_uint;
pub const MSGMODE_PAUSED: C2RustUnnamed_17 = 55;
pub const MSGMODE_TEXT_CLOSING: C2RustUnnamed_17 = 54;
pub const MSGMODE_TEXT_DONE: C2RustUnnamed_17 = 53;
pub const MSGMODE_TEXT_AWAIT_NEXT: C2RustUnnamed_17 = 52;
pub const MSGMODE_FROGS_WAITING: C2RustUnnamed_17 = 51;
pub const MSGMODE_FROGS_PLAYING: C2RustUnnamed_17 = 50;
pub const MSGMODE_FROGS_START: C2RustUnnamed_17 = 49;
pub const MSGMODE_MEMORY_GAME_START_NEXT_ROUND: C2RustUnnamed_17 = 48;
pub const MSGMODE_MEMORY_GAME_ROUND_SUCCESS: C2RustUnnamed_17 = 47;
pub const MSGMODE_MEMORY_GAME_PLAYER_PLAYING: C2RustUnnamed_17 = 46;
pub const MSGMODE_MEMORY_GAME_RIGHT_SKULLKID_WAIT: C2RustUnnamed_17 = 45;
pub const MSGMODE_MEMORY_GAME_RIGHT_SKULLKID_PLAYING: C2RustUnnamed_17 = 44;
pub const MSGMODE_MEMORY_GAME_LEFT_SKULLKID_WAIT: C2RustUnnamed_17 = 43;
pub const MSGMODE_MEMORY_GAME_LEFT_SKULLKID_PLAYING: C2RustUnnamed_17 = 42;
pub const MSGMODE_MEMORY_GAME_START: C2RustUnnamed_17 = 41;
pub const MSGMODE_SCARECROW_PLAYBACK: C2RustUnnamed_17 = 40;
pub const MSGMODE_SCARECROW_RECORDING_DONE: C2RustUnnamed_17 = 39;
pub const MSGMODE_SCARECROW_RECORDING_FAILED: C2RustUnnamed_17 = 38;
pub const MSGMODE_SCARECROW_RECORDING_ONGOING: C2RustUnnamed_17 = 37;
pub const MSGMODE_SCARECROW_RECORDING_START: C2RustUnnamed_17 = 36;
pub const MSGMODE_SCARECROW_LONG_PLAYBACK: C2RustUnnamed_17 = 35;
pub const MSGMODE_SCARECROW_LONG_RECORDING_ONGOING: C2RustUnnamed_17 = 34;
pub const MSGMODE_SCARECROW_LONG_RECORDING_START: C2RustUnnamed_17 = 33;
pub const MSGMODE_UNK_20: C2RustUnnamed_17 = 32;
pub const MSGMODE_OCARINA_AWAIT_INPUT: C2RustUnnamed_17 = 31;
pub const MSGMODE_SONG_PLAYBACK_NOTES_DROP: C2RustUnnamed_17 = 30;
pub const MSGMODE_SONG_PLAYBACK_FAIL: C2RustUnnamed_17 = 29;
pub const MSGMODE_SONG_PLAYBACK_SUCCESS: C2RustUnnamed_17 = 28;
pub const MSGMODE_SONG_PLAYBACK: C2RustUnnamed_17 = 27;
pub const MSGMODE_SONG_DEMONSTRATION_DONE: C2RustUnnamed_17 = 26;
pub const MSGMODE_SONG_DEMONSTRATION: C2RustUnnamed_17 = 25;
pub const MSGMODE_SONG_DEMONSTRATION_SELECT_INSTRUMENT: C2RustUnnamed_17 = 24;
pub const MSGMODE_SONG_PLAYED_ACT: C2RustUnnamed_17 = 23;
pub const MSGMODE_SONG_PLAYED_ACT_BEGIN: C2RustUnnamed_17 = 22;
pub const MSGMODE_DISPLAY_SONG_PLAYED_TEXT: C2RustUnnamed_17 = 21;
pub const MSGMODE_DISPLAY_SONG_PLAYED_TEXT_BEGIN: C2RustUnnamed_17 = 20;
pub const MSGMODE_DISPLAY_SONG_PLAYED: C2RustUnnamed_17 = 19;
pub const MSGMODE_SETUP_DISPLAY_SONG_PLAYED: C2RustUnnamed_17 = 18;
pub const MSGMODE_SONG_PLAYED: C2RustUnnamed_17 = 17;
pub const MSGMODE_OCARINA_NOTES_DROP: C2RustUnnamed_17 = 16;
pub const MSGMODE_OCARINA_FAIL_NO_TEXT: C2RustUnnamed_17 = 15;
pub const MSGMODE_OCARINA_FAIL: C2RustUnnamed_17 = 14;
pub const MSGMODE_OCARINA_CORRECT_PLAYBACK: C2RustUnnamed_17 = 13;
pub const MSGMODE_OCARINA_PLAYING: C2RustUnnamed_17 = 12;
pub const MSGMODE_SONG_PLAYBACK_STARTING: C2RustUnnamed_17 = 11;
pub const MSGMODE_SONG_DEMONSTRATION_STARTING: C2RustUnnamed_17 = 10;
pub const MSGMODE_OCARINA_STARTING: C2RustUnnamed_17 = 9;
pub const MSGMODE_TEXT_DELAYED_BREAK: C2RustUnnamed_17 = 8;
pub const MSGMODE_TEXT_AWAIT_INPUT: C2RustUnnamed_17 = 7;
pub const MSGMODE_TEXT_DISPLAYING: C2RustUnnamed_17 = 6;
pub const MSGMODE_TEXT_CONTINUING: C2RustUnnamed_17 = 5;
pub const MSGMODE_TEXT_NEXT_MSG: C2RustUnnamed_17 = 4;
pub const MSGMODE_TEXT_STARTING: C2RustUnnamed_17 = 3;
pub const MSGMODE_TEXT_BOX_GROWING: C2RustUnnamed_17 = 2;
pub const MSGMODE_TEXT_START: C2RustUnnamed_17 = 1;
pub const MSGMODE_NONE: C2RustUnnamed_17 = 0;
#[no_mangle]
pub unsafe extern "C" fn func_80110990(mut globalCtx: *mut GlobalContext) {
    Map_Destroy(globalCtx);
}
#[no_mangle]
pub unsafe extern "C" fn func_801109B0(mut globalCtx: *mut GlobalContext) {
    let mut interfaceCtx: *mut InterfaceContext =
        &mut (*globalCtx).interfaceCtx;
    let mut parameterSize: u32_0 = 0;
    let mut doActionOffset: u16_0 = 0;
    let mut temp: u8_0 = 0;
    gSaveContext.sunsSongState = SUNSSONG_INACTIVE as libc::c_int as s16;
    gSaveContext.unk_13EA = 0 as libc::c_int as u16_0;
    gSaveContext.unk_13E8 = gSaveContext.unk_13EA;
    View_Init(&mut (*interfaceCtx).view, (*globalCtx).state.gfxCtx);
    (*interfaceCtx).unk_1FC = 0 as libc::c_int as s16;
    (*interfaceCtx).unk_261 = (*interfaceCtx).unk_1FC as u8_0;
    (*interfaceCtx).unk_1FA = (*interfaceCtx).unk_261 as s16;
    (*interfaceCtx).unk_1F0 = 0 as libc::c_int as u16_0;
    (*interfaceCtx).unk_1EE = (*interfaceCtx).unk_1F0;
    (*interfaceCtx).unk_1EC = (*interfaceCtx).unk_1EE as s16;
    (*interfaceCtx).unk_22E = 0 as libc::c_int as s16;
    (*interfaceCtx).unk_230 = 16 as libc::c_int as s16;
    (*interfaceCtx).unk_1F4 = 0.0f32;
    (*interfaceCtx).unk_228 =
        (*gGameInfo).data[(14 as libc::c_int * 6 as libc::c_int *
                               16 as libc::c_int + 95 as libc::c_int) as
                              usize];
    (*interfaceCtx).minimapAlpha = 0 as libc::c_int as u16_0;
    (*interfaceCtx).unk_260 = 0 as libc::c_int as u8_0;
    (*interfaceCtx).magicAlpha = 0 as libc::c_int as u16_0;
    (*interfaceCtx).startAlpha = (*interfaceCtx).magicAlpha as s16;
    (*interfaceCtx).healthAlpha = (*interfaceCtx).startAlpha as u16_0;
    (*interfaceCtx).cRightAlpha = (*interfaceCtx).healthAlpha;
    (*interfaceCtx).cDownAlpha = (*interfaceCtx).cRightAlpha;
    (*interfaceCtx).cLeftAlpha = (*interfaceCtx).cDownAlpha;
    (*interfaceCtx).bAlpha = (*interfaceCtx).cLeftAlpha;
    (*interfaceCtx).aAlpha = (*interfaceCtx).bAlpha;
    (*interfaceCtx).unk_244 = (*interfaceCtx).aAlpha;
    parameterSize =
        (_parameter_staticSegmentRomEnd.as_mut_ptr() as
             u32_0).wrapping_sub(_parameter_staticSegmentRomStart.as_mut_ptr()
                                     as u32_0);
    // "Permanent PARAMETER Segment = %x"
    osSyncPrintf(b"\xe5\xb8\xb8\xe9\xa7\x90\xef\xbc\xb0\xef\xbc\xa1\xef\xbc\xb2\xef\xbc\xa1\xef\xbc\xad\xef\xbc\xa5\xef\xbc\xb4\xef\xbc\xa5\xef\xbc\xb2\xe3\x82\xbb\xe3\x82\xb0\xe3\x83\xa1\xe3\x83\xb3\xe3\x83\x88=%x\n\x00"
                     as *const u8 as *const libc::c_char,
                 parameterSize); // "DO Action Texture Initialization"
    (*interfaceCtx).parameterSegment =
        GameState_Alloc(&mut (*globalCtx).state, parameterSize as size_t,
                        b"../z_construct.c\x00" as *const u8 as
                            *const libc::c_char as *mut libc::c_char,
                        159 as libc::c_int) as *mut u8_0;
    osSyncPrintf(b"parameter->parameterSegment=%x\n\x00" as *const u8 as
                     *const libc::c_char, (*interfaceCtx).parameterSegment);
    if !(*interfaceCtx).parameterSegment.is_null() {
    } else {
        __assert(b"parameter->parameterSegment != NULL\x00" as *const u8 as
                     *const libc::c_char,
                 b"../z_construct.c\x00" as *const u8 as *const libc::c_char,
                 161 as libc::c_int);
    };
    DmaMgr_SendRequest1((*interfaceCtx).parameterSegment as *mut libc::c_void,
                        _parameter_staticSegmentRomStart.as_mut_ptr() as
                            u32_0, parameterSize,
                        b"../z_construct.c\x00" as *const u8 as
                            *const libc::c_char, 162 as libc::c_int);
    (*interfaceCtx).doActionSegment =
        GameState_Alloc(&mut (*globalCtx).state,
                        0x480 as libc::c_int as size_t,
                        b"../z_construct.c\x00" as *const u8 as
                            *const libc::c_char as *mut libc::c_char,
                        166 as libc::c_int) as *mut u8_0;
    osSyncPrintf(b"\xef\xbc\xa4\xef\xbc\xaf\xe3\x82\xa2\xe3\x82\xaf\xe3\x82\xb7\xe3\x83\xa7\xe3\x83\xb3 \xe3\x83\x86\xe3\x82\xaf\xe3\x82\xb9\xe3\x83\x81\xe3\x83\xa3\xe5\x88\x9d\xe6\x9c\x9f=%x\n\x00"
                     as *const u8 as *const libc::c_char,
                 0x480 as libc::c_int);
    osSyncPrintf(b"parameter->do_actionSegment=%x\n\x00" as *const u8 as
                     *const libc::c_char, (*interfaceCtx).doActionSegment);
    if !(*interfaceCtx).doActionSegment.is_null() {
    } else {
        __assert(b"parameter->do_actionSegment != NULL\x00" as *const u8 as
                     *const libc::c_char,
                 b"../z_construct.c\x00" as *const u8 as *const libc::c_char,
                 169 as libc::c_int);
    };
    if gSaveContext.language as libc::c_int == LANGUAGE_ENG as libc::c_int {
        doActionOffset = 0 as libc::c_int as u16_0
    } else if gSaveContext.language as libc::c_int ==
                  LANGUAGE_GER as libc::c_int {
        doActionOffset = 0x2b80 as libc::c_int as u16_0
    } else { doActionOffset = 0x5700 as libc::c_int as u16_0 }
    DmaMgr_SendRequest1((*interfaceCtx).doActionSegment as *mut libc::c_void,
                        (_do_action_staticSegmentRomStart.as_mut_ptr() as
                             u32_0).wrapping_add(doActionOffset as
                                                     libc::c_uint),
                        0x300 as libc::c_int as u32_0,
                        b"../z_construct.c\x00" as *const u8 as
                            *const libc::c_char, 174 as libc::c_int);
    if gSaveContext.language as libc::c_int == LANGUAGE_ENG as libc::c_int {
        doActionOffset = 0x480 as libc::c_int as u16_0
    } else if gSaveContext.language as libc::c_int ==
                  LANGUAGE_GER as libc::c_int {
        doActionOffset = 0x3000 as libc::c_int as u16_0
    } else { doActionOffset = 0x5b80 as libc::c_int as u16_0 }
    DmaMgr_SendRequest1((*interfaceCtx).doActionSegment.offset(0x300 as
                                                                   libc::c_int
                                                                   as isize)
                            as *mut libc::c_void,
                        (_do_action_staticSegmentRomStart.as_mut_ptr() as
                             u32_0).wrapping_add(doActionOffset as
                                                     libc::c_uint),
                        0x180 as libc::c_int as u32_0,
                        b"../z_construct.c\x00" as *const u8 as
                            *const libc::c_char, 178 as libc::c_int);
    (*interfaceCtx).iconItemSegment =
        GameState_Alloc(&mut (*globalCtx).state,
                        0x4000 as libc::c_int as size_t,
                        b"../z_construct.c\x00" as *const u8 as
                            *const libc::c_char as *mut libc::c_char,
                        190 as libc::c_int) as *mut u8_0;
    // "Icon Item Texture Initialization = %x"
    osSyncPrintf(b"\xe3\x82\xa2\xe3\x82\xa4\xe3\x82\xb3\xe3\x83\xb3\xe3\x82\xa2\xe3\x82\xa4\xe3\x83\x86\xe3\x83\xa0 \xe3\x83\x86\xe3\x82\xaf\xe3\x82\xb9\xe3\x83\x81\xe3\x83\xa3\xe5\x88\x9d\xe6\x9c\x9f=%x\n\x00"
                     as *const u8 as *const libc::c_char,
                 0x4000 as libc::c_int);
    osSyncPrintf(b"parameter->icon_itemSegment=%x\n\x00" as *const u8 as
                     *const libc::c_char, (*interfaceCtx).iconItemSegment);
    if !(*interfaceCtx).iconItemSegment.is_null() {
    } else {
        __assert(b"parameter->icon_itemSegment != NULL\x00" as *const u8 as
                     *const libc::c_char,
                 b"../z_construct.c\x00" as *const u8 as *const libc::c_char,
                 193 as libc::c_int);
    };
    osSyncPrintf(b"Register_Item[%x, %x, %x, %x]\n\x00" as *const u8 as
                     *const libc::c_char,
                 gSaveContext.equips.buttonItems[0 as libc::c_int as usize] as
                     libc::c_int,
                 gSaveContext.equips.buttonItems[1 as libc::c_int as usize] as
                     libc::c_int,
                 gSaveContext.equips.buttonItems[2 as libc::c_int as usize] as
                     libc::c_int,
                 gSaveContext.equips.buttonItems[3 as libc::c_int as usize] as
                     libc::c_int);
    if (gSaveContext.equips.buttonItems[0 as libc::c_int as usize] as
            libc::c_int) < 0xf0 as libc::c_int {
        DmaMgr_SendRequest1((*interfaceCtx).iconItemSegment as
                                *mut libc::c_void,
                            _icon_item_staticSegmentRomStart.as_mut_ptr().offset((gSaveContext.equips.buttonItems[0
                                                                                                                      as
                                                                                                                      libc::c_int
                                                                                                                      as
                                                                                                                      usize]
                                                                                      as
                                                                                      libc::c_int
                                                                                      *
                                                                                      0x1000
                                                                                          as
                                                                                          libc::c_int)
                                                                                     as
                                                                                     isize)
                                as u32_0, 0x1000 as libc::c_int as u32_0,
                            b"../z_construct.c\x00" as *const u8 as
                                *const libc::c_char, 198 as libc::c_int);
    } else if gSaveContext.equips.buttonItems[0 as libc::c_int as usize] as
                  libc::c_int != 0xff as libc::c_int {
        DmaMgr_SendRequest1((*interfaceCtx).iconItemSegment as
                                *mut libc::c_void,
                            _icon_item_staticSegmentRomStart.as_mut_ptr().offset((gSaveContext.equips.buttonItems[0
                                                                                                                      as
                                                                                                                      libc::c_int
                                                                                                                      as
                                                                                                                      usize]
                                                                                      as
                                                                                      libc::c_int
                                                                                      *
                                                                                      0x1000
                                                                                          as
                                                                                          libc::c_int)
                                                                                     as
                                                                                     isize)
                                as u32_0, 0x1000 as libc::c_int as u32_0,
                            b"../z_construct.c\x00" as *const u8 as
                                *const libc::c_char, 203 as libc::c_int);
    }
    if (gSaveContext.equips.buttonItems[1 as libc::c_int as usize] as
            libc::c_int) < 0xf0 as libc::c_int {
        DmaMgr_SendRequest1((*interfaceCtx).iconItemSegment.offset(0x1000 as
                                                                       libc::c_int
                                                                       as
                                                                       isize)
                                as *mut libc::c_void,
                            _icon_item_staticSegmentRomStart.as_mut_ptr().offset((gSaveContext.equips.buttonItems[1
                                                                                                                      as
                                                                                                                      libc::c_int
                                                                                                                      as
                                                                                                                      usize]
                                                                                      as
                                                                                      libc::c_int
                                                                                      *
                                                                                      0x1000
                                                                                          as
                                                                                          libc::c_int)
                                                                                     as
                                                                                     isize)
                                as u32_0, 0x1000 as libc::c_int as u32_0,
                            b"../z_construct.c\x00" as *const u8 as
                                *const libc::c_char, 209 as libc::c_int);
    }
    if (gSaveContext.equips.buttonItems[2 as libc::c_int as usize] as
            libc::c_int) < 0xf0 as libc::c_int {
        DmaMgr_SendRequest1((*interfaceCtx).iconItemSegment.offset(0x2000 as
                                                                       libc::c_int
                                                                       as
                                                                       isize)
                                as *mut libc::c_void,
                            _icon_item_staticSegmentRomStart.as_mut_ptr().offset((gSaveContext.equips.buttonItems[2
                                                                                                                      as
                                                                                                                      libc::c_int
                                                                                                                      as
                                                                                                                      usize]
                                                                                      as
                                                                                      libc::c_int
                                                                                      *
                                                                                      0x1000
                                                                                          as
                                                                                          libc::c_int)
                                                                                     as
                                                                                     isize)
                                as u32_0, 0x1000 as libc::c_int as u32_0,
                            b"../z_construct.c\x00" as *const u8 as
                                *const libc::c_char, 214 as libc::c_int);
    }
    if (gSaveContext.equips.buttonItems[3 as libc::c_int as usize] as
            libc::c_int) < 0xf0 as libc::c_int {
        DmaMgr_SendRequest1((*interfaceCtx).iconItemSegment.offset(0x3000 as
                                                                       libc::c_int
                                                                       as
                                                                       isize)
                                as *mut libc::c_void,
                            _icon_item_staticSegmentRomStart.as_mut_ptr().offset((gSaveContext.equips.buttonItems[3
                                                                                                                      as
                                                                                                                      libc::c_int
                                                                                                                      as
                                                                                                                      usize]
                                                                                      as
                                                                                      libc::c_int
                                                                                      *
                                                                                      0x1000
                                                                                          as
                                                                                          libc::c_int)
                                                                                     as
                                                                                     isize)
                                as u32_0, 0x1000 as libc::c_int as u32_0,
                            b"../z_construct.c\x00" as *const u8 as
                                *const libc::c_char, 219 as libc::c_int);
    }
    osSyncPrintf(b"\xef\xbc\xa5\xef\xbc\xb6\xef\xbc\xa5\xef\xbc\xae\xef\xbc\xb4\xef\xbc\x9d%d\n\x00"
                     as *const u8 as *const libc::c_char,
                 gSaveContext.timer1State as libc::c_int);
    if gSaveContext.timer1State as libc::c_int == 4 as libc::c_int ||
           gSaveContext.timer1State as libc::c_int == 8 as libc::c_int ||
           gSaveContext.timer2State as libc::c_int == 4 as libc::c_int ||
           gSaveContext.timer2State as libc::c_int == 10 as libc::c_int {
        osSyncPrintf(b"restart_flag=%d\n\x00" as *const u8 as
                         *const libc::c_char, gSaveContext.respawnFlag);
        if gSaveContext.respawnFlag == -(1 as libc::c_int) ||
               gSaveContext.respawnFlag == 1 as libc::c_int {
            if gSaveContext.timer1State as libc::c_int == 4 as libc::c_int {
                gSaveContext.timer1State = 1 as libc::c_int as s16;
                gSaveContext.timerX[0 as libc::c_int as usize] =
                    140 as libc::c_int as s16;
                gSaveContext.timerY[0 as libc::c_int as usize] =
                    80 as libc::c_int as s16
            }
        }
        if gSaveContext.timer1State as libc::c_int == 4 as libc::c_int ||
               gSaveContext.timer1State as libc::c_int == 8 as libc::c_int {
            temp = 0 as libc::c_int as u8_0
        } else { temp = 1 as libc::c_int as u8_0 }
        gSaveContext.timerX[temp as usize] = 26 as libc::c_int as s16;
        if gSaveContext.healthCapacity as libc::c_int > 0xa0 as libc::c_int {
            gSaveContext.timerY[temp as usize] = 54 as libc::c_int as s16
        } else {
            gSaveContext.timerY[temp as usize] = 46 as libc::c_int as s16
        }
    }
    if gSaveContext.timer1State as libc::c_int >= 11 as libc::c_int &&
           (gSaveContext.timer1State as libc::c_int) < 16 as libc::c_int {
        gSaveContext.timer1State = 0 as libc::c_int as s16;
        // "Timer Stop!!!!!!!!!!!!!!!!!!!!!!"
        osSyncPrintf(b"\xe3\x82\xbf\xe3\x82\xa4\xe3\x83\x9e\xe3\x83\xbc\xe5\x81\x9c\xe6\xad\xa2\xef\xbc\x81\xef\xbc\x81\xef\xbc\x81\xef\xbc\x81\xef\xbc\x81\xef\xbc\x81\xef\xbc\x81\xef\xbc\x81\xef\xbc\x81\xef\xbc\x81\xef\xbc\x81\xef\xbc\x81\xef\xbc\x81\xef\xbc\x81\xef\xbc\x81\xef\xbc\x81\xef\xbc\x81\xef\xbc\x81\xef\xbc\x81\xef\xbc\x81\xef\xbc\x81  = %d\n\x00"
                         as *const u8 as *const libc::c_char,
                     gSaveContext.timer1State as
                         libc::c_int); // "Parameter Area = %x"
    } // "Textbox game_alloc=%x"
    osSyncPrintf(b"\xef\xbc\xb0\xef\xbc\xa1\xef\xbc\xb2\xef\xbc\xa1\xef\xbc\xad\xef\xbc\xa5\xef\xbc\xb4\xef\xbc\xa5\xef\xbc\xb2\xe9\xa0\x98\xe5\x9f\x9f\xef\xbc\x9d%x\n\x00"
                     as *const u8 as *const libc::c_char,
                 parameterSize.wrapping_add(0x5300 as libc::c_int as
                                                libc::c_uint));
    HealthMeter_Init(globalCtx);
    Map_Init(globalCtx);
    (*interfaceCtx).unk_242 = 0 as libc::c_int as u16_0;
    (*interfaceCtx).unk_23C = (*interfaceCtx).unk_242;
    (*gGameInfo).data[(10 as libc::c_int * 6 as libc::c_int *
                           16 as libc::c_int + 70 as libc::c_int +
                           0 as libc::c_int) as usize] =
        160 as libc::c_int as s16;
    (*gGameInfo).data[(10 as libc::c_int * 6 as libc::c_int *
                           16 as libc::c_int + 43 as libc::c_int +
                           0 as libc::c_int) as usize] =
        255 as libc::c_int as s16;
    (*gGameInfo).data[(10 as libc::c_int * 6 as libc::c_int *
                           16 as libc::c_int + 43 as libc::c_int +
                           1 as libc::c_int) as usize] =
        30 as libc::c_int as s16;
    (*gGameInfo).data[(10 as libc::c_int * 6 as libc::c_int *
                           16 as libc::c_int + 43 as libc::c_int +
                           2 as libc::c_int) as usize] =
        30 as libc::c_int as s16;
    (*gGameInfo).data[(10 as libc::c_int * 6 as libc::c_int *
                           16 as libc::c_int + 82 as libc::c_int +
                           0 as libc::c_int) as usize] =
        160 as libc::c_int as s16;
    (*gGameInfo).data[(20 as libc::c_int * 6 as libc::c_int *
                           16 as libc::c_int + 64 as libc::c_int +
                           0 as libc::c_int) as usize] =
        (160 as libc::c_int + 2 as libc::c_int) as s16;
    (*gGameInfo).data[(14 as libc::c_int * 6 as libc::c_int *
                           16 as libc::c_int + 17 as libc::c_int) as usize] =
        186 as libc::c_int as s16;
    (*gGameInfo).data[(14 as libc::c_int * 6 as libc::c_int *
                           16 as libc::c_int + 20 as libc::c_int) as usize] =
        186 as libc::c_int as s16;
    (*gGameInfo).data[(14 as libc::c_int * 6 as libc::c_int *
                           16 as libc::c_int + 22 as libc::c_int +
                           0 as libc::c_int) as usize] =
        0 as libc::c_int as s16;
    (*gGameInfo).data[(14 as libc::c_int * 6 as libc::c_int *
                           16 as libc::c_int + 22 as libc::c_int +
                           1 as libc::c_int) as usize] =
        200 as libc::c_int as s16;
    (*gGameInfo).data[(14 as libc::c_int * 6 as libc::c_int *
                           16 as libc::c_int + 22 as libc::c_int +
                           2 as libc::c_int) as usize] =
        50 as libc::c_int as s16;
}
#[no_mangle]
pub unsafe extern "C" fn Message_Init(mut globalCtx: *mut GlobalContext) {
    let mut msgCtx: *mut MessageContext = &mut (*globalCtx).msgCtx;
    let mut pad: s32 = 0;
    Message_SetTables();
    (*globalCtx).msgCtx.ocarinaMode = OCARINA_MODE_00 as libc::c_int as u16_0;
    (*msgCtx).msgMode = MSGMODE_NONE as libc::c_int as u8_0;
    (*msgCtx).msgLength = 0 as libc::c_int;
    (*msgCtx).textUnskippable = 0 as libc::c_int as u16_0;
    (*msgCtx).ocarinaAction = (*msgCtx).textUnskippable;
    (*msgCtx).choiceIndex = (*msgCtx).ocarinaAction as u8_0;
    (*msgCtx).textboxEndType = (*msgCtx).choiceIndex;
    (*msgCtx).textId = (*msgCtx).textboxEndType as u16_0;
    (*msgCtx).textColorAlpha = 255 as libc::c_int as s16;
    View_Init(&mut (*msgCtx).view, (*globalCtx).state.gfxCtx);
    (*msgCtx).textboxSegment =
        GameState_Alloc(&mut (*globalCtx).state,
                        0x2200 as libc::c_int as size_t,
                        b"../z_construct.c\x00" as *const u8 as
                            *const libc::c_char as *mut libc::c_char,
                        349 as libc::c_int);
    osSyncPrintf(b"message->fukidashiSegment=%x\n\x00" as *const u8 as
                     *const libc::c_char, (*msgCtx).textboxSegment);
    osSyncPrintf(b"\xe5\x90\xb9\xe3\x81\x8d\xe5\x87\xba\xe3\x81\x97game_alloc=%x\n\x00"
                     as *const u8 as *const libc::c_char,
                 0x2200 as libc::c_int);
    if !(*msgCtx).textboxSegment.is_null() {
    } else {
        __assert(b"message->fukidashiSegment != NULL\x00" as *const u8 as
                     *const libc::c_char,
                 b"../z_construct.c\x00" as *const u8 as *const libc::c_char,
                 352 as libc::c_int);
    };
    Font_LoadOrderedFont(&mut (*globalCtx).msgCtx.font);
    (*gGameInfo).data[(6 as libc::c_int * 6 as libc::c_int * 16 as libc::c_int
                           + 31 as libc::c_int) as usize] =
        0 as libc::c_int as s16;
}
#[no_mangle]
pub unsafe extern "C" fn func_80111070() {
    (*gGameInfo).data[(6 as libc::c_int * 6 as libc::c_int * 16 as libc::c_int
                           + 8 as libc::c_int) as usize] =
        10 as libc::c_int as s16;
    (*gGameInfo).data[(6 as libc::c_int * 6 as libc::c_int * 16 as libc::c_int
                           + 14 as libc::c_int) as usize] =
        0 as libc::c_int as s16;
    (*gGameInfo).data[(6 as libc::c_int * 6 as libc::c_int * 16 as libc::c_int
                           + 15 as libc::c_int) as usize] =
        0 as libc::c_int as s16;
    (*gGameInfo).data[(6 as libc::c_int * 6 as libc::c_int * 16 as libc::c_int
                           + 16 as libc::c_int) as usize] =
        0 as libc::c_int as s16;
    (*gGameInfo).data[(6 as libc::c_int * 6 as libc::c_int * 16 as libc::c_int
                           + 17 as libc::c_int) as usize] =
        0 as libc::c_int as s16;
    (*gGameInfo).data[(6 as libc::c_int * 6 as libc::c_int * 16 as libc::c_int
                           + 22 as libc::c_int) as usize] =
        50 as libc::c_int as s16;
    (*gGameInfo).data[(6 as libc::c_int * 6 as libc::c_int * 16 as libc::c_int
                           + 23 as libc::c_int) as usize] =
        0 as libc::c_int as s16;
    (*gGameInfo).data[(6 as libc::c_int * 6 as libc::c_int * 16 as libc::c_int
                           + 24 as libc::c_int) as usize] =
        -(60 as libc::c_int) as s16;
    (*gGameInfo).data[(6 as libc::c_int * 6 as libc::c_int * 16 as libc::c_int
                           + 25 as libc::c_int) as usize] =
        13 as libc::c_int as s16;
    (*gGameInfo).data[(6 as libc::c_int * 6 as libc::c_int * 16 as libc::c_int
                           + 26 as libc::c_int) as usize] =
        15 as libc::c_int as s16;
    (*gGameInfo).data[(6 as libc::c_int * 6 as libc::c_int * 16 as libc::c_int
                           + 27 as libc::c_int) as usize] =
        41 as libc::c_int as s16;
    (*gGameInfo).data[(6 as libc::c_int * 6 as libc::c_int * 16 as libc::c_int
                           + 28 as libc::c_int) as usize] =
        15 as libc::c_int as s16;
    (*gGameInfo).data[(6 as libc::c_int * 6 as libc::c_int * 16 as libc::c_int
                           + 32 as libc::c_int) as usize] =
        265 as libc::c_int as s16;
    (*gGameInfo).data[(6 as libc::c_int * 6 as libc::c_int * 16 as libc::c_int
                           + 33 as libc::c_int) as usize] =
        55 as libc::c_int as s16;
    (*gGameInfo).data[(6 as libc::c_int * 6 as libc::c_int * 16 as libc::c_int
                           + 34 as libc::c_int) as usize] =
        0 as libc::c_int as s16;
    (*gGameInfo).data[(6 as libc::c_int * 6 as libc::c_int * 16 as libc::c_int
                           + 35 as libc::c_int) as usize] =
        20 as libc::c_int as s16;
    (*gGameInfo).data[(6 as libc::c_int * 6 as libc::c_int * 16 as libc::c_int
                           + 36 as libc::c_int) as usize] =
        0 as libc::c_int as s16;
    (*gGameInfo).data[(6 as libc::c_int * 6 as libc::c_int * 16 as libc::c_int
                           + 37 as libc::c_int) as usize] =
        0 as libc::c_int as s16;
    (*gGameInfo).data[(6 as libc::c_int * 6 as libc::c_int * 16 as libc::c_int
                           + 38 as libc::c_int) as usize] =
        0 as libc::c_int as s16;
    (*gGameInfo).data[(6 as libc::c_int * 6 as libc::c_int * 16 as libc::c_int
                           + 40 as libc::c_int) as usize] =
        2 as libc::c_int as s16;
    (*gGameInfo).data[(6 as libc::c_int * 6 as libc::c_int * 16 as libc::c_int
                           + 41 as libc::c_int) as usize] =
        1 as libc::c_int as s16;
    (*gGameInfo).data[(6 as libc::c_int * 6 as libc::c_int * 16 as libc::c_int
                           + 42 as libc::c_int) as usize] =
        2 as libc::c_int as s16;
    (*gGameInfo).data[(6 as libc::c_int * 6 as libc::c_int * 16 as libc::c_int
                           + 43 as libc::c_int) as usize] =
        1 as libc::c_int as s16;
    (*gGameInfo).data[(6 as libc::c_int * 6 as libc::c_int * 16 as libc::c_int
                           + 44 as libc::c_int) as usize] =
        0 as libc::c_int as s16;
    (*gGameInfo).data[(6 as libc::c_int * 6 as libc::c_int * 16 as libc::c_int
                           + 45 as libc::c_int) as usize] =
        236 as libc::c_int as s16;
    (*gGameInfo).data[(6 as libc::c_int * 6 as libc::c_int * 16 as libc::c_int
                           + 46 as libc::c_int) as usize] =
        36 as libc::c_int as s16;
    (*gGameInfo).data[(6 as libc::c_int * 6 as libc::c_int * 16 as libc::c_int
                           + 47 as libc::c_int) as usize] =
        0 as libc::c_int as s16;
    (*gGameInfo).data[(6 as libc::c_int * 6 as libc::c_int * 16 as libc::c_int
                           + 48 as libc::c_int) as usize] =
        -(45 as libc::c_int) as s16;
    (*gGameInfo).data[(6 as libc::c_int * 6 as libc::c_int * 16 as libc::c_int
                           + 49 as libc::c_int) as usize] =
        -(48 as libc::c_int) as s16;
    (*gGameInfo).data[(6 as libc::c_int * 6 as libc::c_int * 16 as libc::c_int
                           + 50 as libc::c_int) as usize] =
        16 as libc::c_int as s16;
    (*gGameInfo).data[(6 as libc::c_int * 6 as libc::c_int * 16 as libc::c_int
                           + 51 as libc::c_int) as usize] =
        22 as libc::c_int as s16;
    (*gGameInfo).data[(6 as libc::c_int * 6 as libc::c_int * 16 as libc::c_int
                           + 52 as libc::c_int) as usize] =
        -(55 as libc::c_int) as s16;
    (*gGameInfo).data[(6 as libc::c_int * 6 as libc::c_int * 16 as libc::c_int
                           + 53 as libc::c_int) as usize] =
        -(53 as libc::c_int) as s16;
    (*gGameInfo).data[(6 as libc::c_int * 6 as libc::c_int * 16 as libc::c_int
                           + 54 as libc::c_int) as usize] =
        43 as libc::c_int as s16;
    (*gGameInfo).data[(6 as libc::c_int * 6 as libc::c_int * 16 as libc::c_int
                           + 55 as libc::c_int) as usize] =
        47 as libc::c_int as s16;
    (*gGameInfo).data[(6 as libc::c_int * 6 as libc::c_int * 16 as libc::c_int
                           + 56 as libc::c_int) as usize] =
        -(33 as libc::c_int) as s16;
    (*gGameInfo).data[(6 as libc::c_int * 6 as libc::c_int * 16 as libc::c_int
                           + 57 as libc::c_int) as usize] =
        -(42 as libc::c_int) as s16;
    (*gGameInfo).data[(6 as libc::c_int * 6 as libc::c_int * 16 as libc::c_int
                           + 58 as libc::c_int) as usize] =
        -(33 as libc::c_int) as s16;
    (*gGameInfo).data[(6 as libc::c_int * 6 as libc::c_int * 16 as libc::c_int
                           + 59 as libc::c_int) as usize] =
        -(37 as libc::c_int) as s16;
    (*gGameInfo).data[(6 as libc::c_int * 6 as libc::c_int * 16 as libc::c_int
                           + 60 as libc::c_int) as usize] =
        14 as libc::c_int as s16;
    (*gGameInfo).data[(6 as libc::c_int * 6 as libc::c_int * 16 as libc::c_int
                           + 61 as libc::c_int) as usize] =
        -(2 as libc::c_int) as s16;
    (*gGameInfo).data[(6 as libc::c_int * 6 as libc::c_int * 16 as libc::c_int
                           + 62 as libc::c_int) as usize] =
        -(2 as libc::c_int) as s16;
    (*gGameInfo).data[(6 as libc::c_int * 6 as libc::c_int * 16 as libc::c_int
                           + 63 as libc::c_int) as usize] =
        -(18 as libc::c_int) as s16;
    (*gGameInfo).data[(6 as libc::c_int * 6 as libc::c_int * 16 as libc::c_int
                           + 64 as libc::c_int) as usize] =
        -(18 as libc::c_int) as s16;
    (*gGameInfo).data[(6 as libc::c_int * 6 as libc::c_int * 16 as libc::c_int
                           + 67 as libc::c_int) as usize] =
        0 as libc::c_int as s16;
    (*gGameInfo).data[(6 as libc::c_int * 6 as libc::c_int * 16 as libc::c_int
                           + 68 as libc::c_int) as usize] =
        0 as libc::c_int as s16;
    (*gGameInfo).data[(6 as libc::c_int * 6 as libc::c_int * 16 as libc::c_int
                           + 69 as libc::c_int) as usize] =
        0 as libc::c_int as s16;
    (*gGameInfo).data[(6 as libc::c_int * 6 as libc::c_int * 16 as libc::c_int
                           + 70 as libc::c_int) as usize] =
        0 as libc::c_int as s16;
    (*gGameInfo).data[(6 as libc::c_int * 6 as libc::c_int * 16 as libc::c_int
                           + 71 as libc::c_int) as usize] =
        -(6 as libc::c_int) as s16;
    (*gGameInfo).data[(6 as libc::c_int * 6 as libc::c_int * 16 as libc::c_int
                           + 72 as libc::c_int) as usize] =
        10 as libc::c_int as s16;
    (*gGameInfo).data[(6 as libc::c_int * 6 as libc::c_int * 16 as libc::c_int
                           + 73 as libc::c_int) as usize] =
        -(8 as libc::c_int) as s16;
    (*gGameInfo).data[(6 as libc::c_int * 6 as libc::c_int * 16 as libc::c_int
                           + 74 as libc::c_int) as usize] =
        8 as libc::c_int as s16;
    (*gGameInfo).data[(6 as libc::c_int * 6 as libc::c_int * 16 as libc::c_int
                           + 75 as libc::c_int) as usize] =
        24 as libc::c_int as s16;
    (*gGameInfo).data[(6 as libc::c_int * 6 as libc::c_int * 16 as libc::c_int
                           + 76 as libc::c_int) as usize] =
        32 as libc::c_int as s16;
    (*gGameInfo).data[(6 as libc::c_int * 6 as libc::c_int * 16 as libc::c_int
                           + 77 as libc::c_int) as usize] =
        0 as libc::c_int as s16;
    (*gGameInfo).data[(6 as libc::c_int * 6 as libc::c_int * 16 as libc::c_int
                           + 78 as libc::c_int) as usize] =
        0 as libc::c_int as s16;
    (*gGameInfo).data[(6 as libc::c_int * 6 as libc::c_int * 16 as libc::c_int
                           + 79 as libc::c_int) as usize] =
        48 as libc::c_int as s16;
    (*gGameInfo).data[(6 as libc::c_int * 6 as libc::c_int * 16 as libc::c_int
                           + 80 as libc::c_int) as usize] =
        450 as libc::c_int as s16;
    (*gGameInfo).data[(6 as libc::c_int * 6 as libc::c_int * 16 as libc::c_int
                           + 81 as libc::c_int) as usize] =
        0 as libc::c_int as s16;
    (*gGameInfo).data[(6 as libc::c_int * 6 as libc::c_int * 16 as libc::c_int
                           + 82 as libc::c_int) as usize] =
        -(15 as libc::c_int) as s16;
    (*gGameInfo).data[(6 as libc::c_int * 6 as libc::c_int * 16 as libc::c_int
                           + 83 as libc::c_int) as usize] =
        500 as libc::c_int as s16;
    (*gGameInfo).data[(6 as libc::c_int * 6 as libc::c_int * 16 as libc::c_int
                           + 84 as libc::c_int) as usize] =
        600 as libc::c_int as s16;
    (*gGameInfo).data[(6 as libc::c_int * 6 as libc::c_int * 16 as libc::c_int
                           + 85 as libc::c_int) as usize] =
        0 as libc::c_int as s16;
    (*gGameInfo).data[(6 as libc::c_int * 6 as libc::c_int * 16 as libc::c_int
                           + 86 as libc::c_int) as usize] =
        -(21 as libc::c_int) as s16;
    (*gGameInfo).data[(6 as libc::c_int * 6 as libc::c_int * 16 as libc::c_int
                           + 87 as libc::c_int) as usize] =
        510 as libc::c_int as s16;
    (*gGameInfo).data[(6 as libc::c_int * 6 as libc::c_int * 16 as libc::c_int
                           + 88 as libc::c_int) as usize] =
        (254 as libc::c_int - 7 as libc::c_int) as s16;
    (*gGameInfo).data[(6 as libc::c_int * 6 as libc::c_int * 16 as libc::c_int
                           + 89 as libc::c_int) as usize] =
        (16 as libc::c_int + 4 as libc::c_int) as s16;
    (*gGameInfo).data[(6 as libc::c_int * 6 as libc::c_int * 16 as libc::c_int
                           + 92 as libc::c_int) as usize] =
        8 as libc::c_int as s16;
    (*gGameInfo).data[(6 as libc::c_int * 6 as libc::c_int * 16 as libc::c_int
                           + 93 as libc::c_int) as usize] =
        6 as libc::c_int as s16;
    (*gGameInfo).data[(6 as libc::c_int * 6 as libc::c_int * 16 as libc::c_int
                           + 94 as libc::c_int) as usize] =
        3 as libc::c_int as s16;
    (*gGameInfo).data[(6 as libc::c_int * 6 as libc::c_int * 16 as libc::c_int
                           + 95 as libc::c_int) as usize] =
        1 as libc::c_int as s16;
    (*gGameInfo).data[(10 as libc::c_int * 6 as libc::c_int *
                           16 as libc::c_int + 0 as libc::c_int +
                           0 as libc::c_int) as usize] =
        0 as libc::c_int as s16;
    (*gGameInfo).data[(10 as libc::c_int * 6 as libc::c_int *
                           16 as libc::c_int + 0 as libc::c_int +
                           1 as libc::c_int) as usize] =
        200 as libc::c_int as s16;
    (*gGameInfo).data[(10 as libc::c_int * 6 as libc::c_int *
                           16 as libc::c_int + 0 as libc::c_int +
                           2 as libc::c_int) as usize] =
        0 as libc::c_int as s16;
    (*gGameInfo).data[(10 as libc::c_int * 6 as libc::c_int *
                           16 as libc::c_int + 9 as libc::c_int) as usize] =
        140 as libc::c_int as s16;
    (*gGameInfo).data[(10 as libc::c_int * 6 as libc::c_int *
                           16 as libc::c_int + 10 as libc::c_int) as usize] =
        200 as libc::c_int as s16;
    (*gGameInfo).data[(10 as libc::c_int * 6 as libc::c_int *
                           16 as libc::c_int + 11 as libc::c_int) as usize] =
        0 as libc::c_int as s16;
    (*gGameInfo).data[(10 as libc::c_int * 6 as libc::c_int *
                           16 as libc::c_int + 12 as libc::c_int) as usize] =
        200 as libc::c_int as s16;
    (*gGameInfo).data[(10 as libc::c_int * 6 as libc::c_int *
                           16 as libc::c_int + 13 as libc::c_int) as usize] =
        0 as libc::c_int as s16;
    (*gGameInfo).data[(10 as libc::c_int * 6 as libc::c_int *
                           16 as libc::c_int + 14 as libc::c_int) as usize] =
        110 as libc::c_int as s16;
    (*gGameInfo).data[(10 as libc::c_int * 6 as libc::c_int *
                           16 as libc::c_int + 15 as libc::c_int) as usize] =
        56 as libc::c_int as s16;
    (*gGameInfo).data[(10 as libc::c_int * 6 as libc::c_int *
                           16 as libc::c_int + 16 as libc::c_int) as usize] =
        1 as libc::c_int as s16;
    (*gGameInfo).data[(10 as libc::c_int * 6 as libc::c_int *
                           16 as libc::c_int + 17 as libc::c_int) as usize] =
        -(50 as libc::c_int) as s16;
    (*gGameInfo).data[(10 as libc::c_int * 6 as libc::c_int *
                           16 as libc::c_int + 18 as libc::c_int) as usize] =
        -(200 as libc::c_int) as s16;
    (*gGameInfo).data[(10 as libc::c_int * 6 as libc::c_int *
                           16 as libc::c_int + 19 as libc::c_int) as usize] =
        0 as libc::c_int as s16;
    (*gGameInfo).data[(10 as libc::c_int * 6 as libc::c_int *
                           16 as libc::c_int + 20 as libc::c_int) as usize] =
        0 as libc::c_int as s16;
    (*gGameInfo).data[(10 as libc::c_int * 6 as libc::c_int *
                           16 as libc::c_int + 21 as libc::c_int) as usize] =
        50 as libc::c_int as s16;
    (*gGameInfo).data[(10 as libc::c_int * 6 as libc::c_int *
                           16 as libc::c_int + 22 as libc::c_int) as usize] =
        -(50 as libc::c_int) as s16;
    (*gGameInfo).data[(10 as libc::c_int * 6 as libc::c_int *
                           16 as libc::c_int + 23 as libc::c_int) as usize] =
        20 as libc::c_int as s16;
    (*gGameInfo).data[(10 as libc::c_int * 6 as libc::c_int *
                           16 as libc::c_int + 24 as libc::c_int) as usize] =
        20 as libc::c_int as s16;
    (*gGameInfo).data[(10 as libc::c_int * 6 as libc::c_int *
                           16 as libc::c_int + 25 as libc::c_int) as usize] =
        4 as libc::c_int as s16;
    (*gGameInfo).data[(10 as libc::c_int * 6 as libc::c_int *
                           16 as libc::c_int + 26 as libc::c_int) as usize] =
        20 as libc::c_int as s16;
    (*gGameInfo).data[(10 as libc::c_int * 6 as libc::c_int *
                           16 as libc::c_int + 27 as libc::c_int) as usize] =
        10 as libc::c_int as s16;
    (*gGameInfo).data[(10 as libc::c_int * 6 as libc::c_int *
                           16 as libc::c_int + 28 as libc::c_int) as usize] =
        20 as libc::c_int as s16;
    (*gGameInfo).data[(10 as libc::c_int * 6 as libc::c_int *
                           16 as libc::c_int + 29 as libc::c_int) as usize] =
        4 as libc::c_int as s16;
    (*gGameInfo).data[(10 as libc::c_int * 6 as libc::c_int *
                           16 as libc::c_int + 30 as libc::c_int) as usize] =
        20 as libc::c_int as s16;
    (*gGameInfo).data[(10 as libc::c_int * 6 as libc::c_int *
                           16 as libc::c_int + 31 as libc::c_int) as usize] =
        10 as libc::c_int as s16;
    (*gGameInfo).data[(10 as libc::c_int * 6 as libc::c_int *
                           16 as libc::c_int + 32 as libc::c_int) as usize] =
        0 as libc::c_int as s16;
    (*gGameInfo).data[(10 as libc::c_int * 6 as libc::c_int *
                           16 as libc::c_int + 33 as libc::c_int) as usize] =
        0 as libc::c_int as s16;
    (*gGameInfo).data[(10 as libc::c_int * 6 as libc::c_int *
                           16 as libc::c_int + 34 as libc::c_int) as usize] =
        0 as libc::c_int as s16;
    (*gGameInfo).data[(10 as libc::c_int * 6 as libc::c_int *
                           16 as libc::c_int + 36 as libc::c_int) as usize] =
        0 as libc::c_int as s16;
    (*gGameInfo).data[(10 as libc::c_int * 6 as libc::c_int *
                           16 as libc::c_int + 37 as libc::c_int) as usize] =
        0 as libc::c_int as s16;
    (*gGameInfo).data[(10 as libc::c_int * 6 as libc::c_int *
                           16 as libc::c_int + 38 as libc::c_int) as usize] =
        0 as libc::c_int as s16;
    (*gGameInfo).data[(10 as libc::c_int * 6 as libc::c_int *
                           16 as libc::c_int + 39 as libc::c_int +
                           0 as libc::c_int) as usize] =
        255 as libc::c_int as s16;
    (*gGameInfo).data[(10 as libc::c_int * 6 as libc::c_int *
                           16 as libc::c_int + 39 as libc::c_int +
                           1 as libc::c_int) as usize] =
        160 as libc::c_int as s16;
    (*gGameInfo).data[(10 as libc::c_int * 6 as libc::c_int *
                           16 as libc::c_int + 39 as libc::c_int +
                           2 as libc::c_int) as usize] =
        0 as libc::c_int as s16;
    (*gGameInfo).data[(10 as libc::c_int * 6 as libc::c_int *
                           16 as libc::c_int + 46 as libc::c_int) as usize] =
        1 as libc::c_int as s16;
    (*gGameInfo).data[(10 as libc::c_int * 6 as libc::c_int *
                           16 as libc::c_int + 47 as libc::c_int) as usize] =
        1 as libc::c_int as s16;
    (*gGameInfo).data[(10 as libc::c_int * 6 as libc::c_int *
                           16 as libc::c_int + 48 as libc::c_int +
                           0 as libc::c_int) as usize] =
        100 as libc::c_int as s16;
    (*gGameInfo).data[(10 as libc::c_int * 6 as libc::c_int *
                           16 as libc::c_int + 48 as libc::c_int +
                           1 as libc::c_int) as usize] =
        89 as libc::c_int as s16;
    (*gGameInfo).data[(10 as libc::c_int * 6 as libc::c_int *
                           16 as libc::c_int + 48 as libc::c_int +
                           2 as libc::c_int) as usize] =
        92 as libc::c_int as s16;
    (*gGameInfo).data[(10 as libc::c_int * 6 as libc::c_int *
                           16 as libc::c_int + 51 as libc::c_int +
                           0 as libc::c_int) as usize] =
        20 as libc::c_int as s16;
    (*gGameInfo).data[(10 as libc::c_int * 6 as libc::c_int *
                           16 as libc::c_int + 51 as libc::c_int +
                           1 as libc::c_int) as usize] =
        20 as libc::c_int as s16;
    (*gGameInfo).data[(10 as libc::c_int * 6 as libc::c_int *
                           16 as libc::c_int + 51 as libc::c_int +
                           2 as libc::c_int) as usize] =
        20 as libc::c_int as s16;
    (*gGameInfo).data[(10 as libc::c_int * 6 as libc::c_int *
                           16 as libc::c_int + 54 as libc::c_int +
                           0 as libc::c_int) as usize] =
        120 as libc::c_int as s16;
    (*gGameInfo).data[(10 as libc::c_int * 6 as libc::c_int *
                           16 as libc::c_int + 54 as libc::c_int +
                           1 as libc::c_int) as usize] =
        119 as libc::c_int as s16;
    (*gGameInfo).data[(10 as libc::c_int * 6 as libc::c_int *
                           16 as libc::c_int + 54 as libc::c_int +
                           2 as libc::c_int) as usize] =
        119 as libc::c_int as s16;
    (*gGameInfo).data[(10 as libc::c_int * 6 as libc::c_int *
                           16 as libc::c_int + 61 as libc::c_int) as usize] =
        1 as libc::c_int as s16;
    (*gGameInfo).data[(10 as libc::c_int * 6 as libc::c_int *
                           16 as libc::c_int + 62 as libc::c_int) as usize] =
        254 as libc::c_int as s16;
    (*gGameInfo).data[(10 as libc::c_int * 6 as libc::c_int *
                           16 as libc::c_int + 63 as libc::c_int) as usize] =
        16 as libc::c_int as s16;
    (*gGameInfo).data[(10 as libc::c_int * 6 as libc::c_int *
                           16 as libc::c_int + 64 as libc::c_int) as usize] =
        20 as libc::c_int as s16;
    (*gGameInfo).data[(10 as libc::c_int * 6 as libc::c_int *
                           16 as libc::c_int + 65 as libc::c_int) as usize] =
        21 as libc::c_int as s16;
    (*gGameInfo).data[(10 as libc::c_int * 6 as libc::c_int *
                           16 as libc::c_int + 66 as libc::c_int) as usize] =
        122 as libc::c_int as s16;
    (*gGameInfo).data[(10 as libc::c_int * 6 as libc::c_int *
                           16 as libc::c_int + 70 as libc::c_int +
                           1 as libc::c_int) as usize] =
        227 as libc::c_int as s16;
    (*gGameInfo).data[(10 as libc::c_int * 6 as libc::c_int *
                           16 as libc::c_int + 70 as libc::c_int +
                           2 as libc::c_int) as usize] =
        249 as libc::c_int as s16;
    (*gGameInfo).data[(10 as libc::c_int * 6 as libc::c_int *
                           16 as libc::c_int + 70 as libc::c_int +
                           3 as libc::c_int) as usize] =
        271 as libc::c_int as s16;
    (*gGameInfo).data[(10 as libc::c_int * 6 as libc::c_int *
                           16 as libc::c_int + 74 as libc::c_int +
                           0 as libc::c_int) as usize] =
        17 as libc::c_int as s16;
    (*gGameInfo).data[(10 as libc::c_int * 6 as libc::c_int *
                           16 as libc::c_int + 74 as libc::c_int +
                           1 as libc::c_int) as usize] =
        18 as libc::c_int as s16;
    (*gGameInfo).data[(10 as libc::c_int * 6 as libc::c_int *
                           16 as libc::c_int + 74 as libc::c_int +
                           2 as libc::c_int) as usize] =
        34 as libc::c_int as s16;
    (*gGameInfo).data[(10 as libc::c_int * 6 as libc::c_int *
                           16 as libc::c_int + 74 as libc::c_int +
                           3 as libc::c_int) as usize] =
        18 as libc::c_int as s16;
    (*gGameInfo).data[(10 as libc::c_int * 6 as libc::c_int *
                           16 as libc::c_int + 78 as libc::c_int +
                           0 as libc::c_int) as usize] =
        575 as libc::c_int as s16;
    (*gGameInfo).data[(10 as libc::c_int * 6 as libc::c_int *
                           16 as libc::c_int + 78 as libc::c_int +
                           1 as libc::c_int) as usize] =
        620 as libc::c_int as s16;
    (*gGameInfo).data[(10 as libc::c_int * 6 as libc::c_int *
                           16 as libc::c_int + 78 as libc::c_int +
                           2 as libc::c_int) as usize] =
        620 as libc::c_int as s16;
    (*gGameInfo).data[(10 as libc::c_int * 6 as libc::c_int *
                           16 as libc::c_int + 78 as libc::c_int +
                           3 as libc::c_int) as usize] =
        620 as libc::c_int as s16;
    (*gGameInfo).data[(10 as libc::c_int * 6 as libc::c_int *
                           16 as libc::c_int + 82 as libc::c_int +
                           1 as libc::c_int) as usize] =
        227 as libc::c_int as s16;
    (*gGameInfo).data[(10 as libc::c_int * 6 as libc::c_int *
                           16 as libc::c_int + 82 as libc::c_int +
                           2 as libc::c_int) as usize] =
        249 as libc::c_int as s16;
    (*gGameInfo).data[(10 as libc::c_int * 6 as libc::c_int *
                           16 as libc::c_int + 82 as libc::c_int +
                           3 as libc::c_int) as usize] =
        271 as libc::c_int as s16;
    (*gGameInfo).data[(10 as libc::c_int * 6 as libc::c_int *
                           16 as libc::c_int + 86 as libc::c_int +
                           0 as libc::c_int) as usize] =
        17 as libc::c_int as s16;
    (*gGameInfo).data[(10 as libc::c_int * 6 as libc::c_int *
                           16 as libc::c_int + 86 as libc::c_int +
                           1 as libc::c_int) as usize] =
        18 as libc::c_int as s16;
    (*gGameInfo).data[(10 as libc::c_int * 6 as libc::c_int *
                           16 as libc::c_int + 86 as libc::c_int +
                           2 as libc::c_int) as usize] =
        34 as libc::c_int as s16;
    (*gGameInfo).data[(10 as libc::c_int * 6 as libc::c_int *
                           16 as libc::c_int + 86 as libc::c_int +
                           3 as libc::c_int) as usize] =
        18 as libc::c_int as s16;
    (*gGameInfo).data[(10 as libc::c_int * 6 as libc::c_int *
                           16 as libc::c_int + 90 as libc::c_int +
                           0 as libc::c_int) as usize] =
        550 as libc::c_int as s16;
    (*gGameInfo).data[(10 as libc::c_int * 6 as libc::c_int *
                           16 as libc::c_int + 90 as libc::c_int +
                           1 as libc::c_int) as usize] =
        680 as libc::c_int as s16;
    (*gGameInfo).data[(10 as libc::c_int * 6 as libc::c_int *
                           16 as libc::c_int + 90 as libc::c_int +
                           2 as libc::c_int) as usize] =
        680 as libc::c_int as s16;
    (*gGameInfo).data[(10 as libc::c_int * 6 as libc::c_int *
                           16 as libc::c_int + 90 as libc::c_int +
                           3 as libc::c_int) as usize] =
        680 as libc::c_int as s16;
    (*gGameInfo).data[(10 as libc::c_int * 6 as libc::c_int *
                           16 as libc::c_int + 94 as libc::c_int) as usize] =
        1 as libc::c_int as s16;
    (*gGameInfo).data[(10 as libc::c_int * 6 as libc::c_int *
                           16 as libc::c_int + 95 as libc::c_int) as usize] =
        0 as libc::c_int as s16;
    (*gGameInfo).data[(14 as libc::c_int * 6 as libc::c_int *
                           16 as libc::c_int + 0 as libc::c_int) as usize] =
        26 as libc::c_int as s16;
    (*gGameInfo).data[(14 as libc::c_int * 6 as libc::c_int *
                           16 as libc::c_int + 1 as libc::c_int) as usize] =
        22 as libc::c_int as s16;
    (*gGameInfo).data[(14 as libc::c_int * 6 as libc::c_int *
                           16 as libc::c_int + 2 as libc::c_int) as usize] =
        -(11 as libc::c_int) as s16;
    (*gGameInfo).data[(14 as libc::c_int * 6 as libc::c_int *
                           16 as libc::c_int + 3 as libc::c_int) as usize] =
        -(4 as libc::c_int) as s16;
    (*gGameInfo).data[(14 as libc::c_int * 6 as libc::c_int *
                           16 as libc::c_int + 4 as libc::c_int) as usize] =
        3 as libc::c_int as s16;
    (*gGameInfo).data[(14 as libc::c_int * 6 as libc::c_int *
                           16 as libc::c_int + 5 as libc::c_int) as usize] =
        0 as libc::c_int as s16;
    (*gGameInfo).data[(14 as libc::c_int * 6 as libc::c_int *
                           16 as libc::c_int + 6 as libc::c_int) as usize] =
        2 as libc::c_int as s16;
    (*gGameInfo).data[(14 as libc::c_int * 6 as libc::c_int *
                           16 as libc::c_int + 7 as libc::c_int) as usize] =
        30 as libc::c_int as s16;
    (*gGameInfo).data[(14 as libc::c_int * 6 as libc::c_int *
                           16 as libc::c_int + 8 as libc::c_int) as usize] =
        10 as libc::c_int as s16;
    (*gGameInfo).data[(14 as libc::c_int * 6 as libc::c_int *
                           16 as libc::c_int + 9 as libc::c_int) as usize] =
        0 as libc::c_int as s16;
    (*gGameInfo).data[(14 as libc::c_int * 6 as libc::c_int *
                           16 as libc::c_int + 10 as libc::c_int) as usize] =
        -(9550 as libc::c_int) as s16;
    (*gGameInfo).data[(14 as libc::c_int * 6 as libc::c_int *
                           16 as libc::c_int + 11 as libc::c_int) as usize] =
        9950 as libc::c_int as s16;
    (*gGameInfo).data[(14 as libc::c_int * 6 as libc::c_int *
                           16 as libc::c_int + 12 as libc::c_int) as usize] =
        68 as libc::c_int as s16;
    (*gGameInfo).data[(14 as libc::c_int * 6 as libc::c_int *
                           16 as libc::c_int + 13 as libc::c_int) as usize] =
        36 as libc::c_int as s16;
    (*gGameInfo).data[(14 as libc::c_int * 6 as libc::c_int *
                           16 as libc::c_int + 14 as libc::c_int) as usize] =
        4 as libc::c_int as s16;
    (*gGameInfo).data[(14 as libc::c_int * 6 as libc::c_int *
                           16 as libc::c_int + 15 as libc::c_int) as usize] =
        1 as libc::c_int as s16;
    (*gGameInfo).data[(14 as libc::c_int * 6 as libc::c_int *
                           16 as libc::c_int + 16 as libc::c_int) as usize] =
        9 as libc::c_int as s16;
    (*gGameInfo).data[(14 as libc::c_int * 6 as libc::c_int *
                           16 as libc::c_int + 18 as libc::c_int) as usize] =
        -(380 as libc::c_int) as s16;
    (*gGameInfo).data[(14 as libc::c_int * 6 as libc::c_int *
                           16 as libc::c_int + 19 as libc::c_int) as usize] =
        9 as libc::c_int as s16;
    (*gGameInfo).data[(14 as libc::c_int * 6 as libc::c_int *
                           16 as libc::c_int + 21 as libc::c_int) as usize] =
        48 as libc::c_int as s16;
    (*gGameInfo).data[(14 as libc::c_int * 6 as libc::c_int *
                           16 as libc::c_int + 25 as libc::c_int) as usize] =
        0 as libc::c_int as s16;
    (*gGameInfo).data[(14 as libc::c_int * 6 as libc::c_int *
                           16 as libc::c_int + 26 as libc::c_int) as usize] =
        0 as libc::c_int as s16;
    (*gGameInfo).data[(14 as libc::c_int * 6 as libc::c_int *
                           16 as libc::c_int + 27 as libc::c_int) as usize] =
        0 as libc::c_int as s16;
    (*gGameInfo).data[(14 as libc::c_int * 6 as libc::c_int *
                           16 as libc::c_int + 28 as libc::c_int) as usize] =
        16 as libc::c_int as s16;
    (*gGameInfo).data[(14 as libc::c_int * 6 as libc::c_int *
                           16 as libc::c_int + 29 as libc::c_int) as usize] =
        50 as libc::c_int as s16;
    (*gGameInfo).data[(14 as libc::c_int * 6 as libc::c_int *
                           16 as libc::c_int + 30 as libc::c_int) as usize] =
        15 as libc::c_int as s16;
    (*gGameInfo).data[(14 as libc::c_int * 6 as libc::c_int *
                           16 as libc::c_int + 31 as libc::c_int) as usize] =
        8 as libc::c_int as s16;
    (*gGameInfo).data[(14 as libc::c_int * 6 as libc::c_int *
                           16 as libc::c_int + 32 as libc::c_int) as usize] =
        4 as libc::c_int as s16;
    (*gGameInfo).data[(14 as libc::c_int * 6 as libc::c_int *
                           16 as libc::c_int + 33 as libc::c_int) as usize] =
        2 as libc::c_int as s16;
    (*gGameInfo).data[(14 as libc::c_int * 6 as libc::c_int *
                           16 as libc::c_int + 34 as libc::c_int) as usize] =
        100 as libc::c_int as s16;
    (*gGameInfo).data[(14 as libc::c_int * 6 as libc::c_int *
                           16 as libc::c_int + 35 as libc::c_int) as usize] =
        7 as libc::c_int as s16;
    (*gGameInfo).data[(14 as libc::c_int * 6 as libc::c_int *
                           16 as libc::c_int + 36 as libc::c_int) as usize] =
        20 as libc::c_int as s16;
    (*gGameInfo).data[(14 as libc::c_int * 6 as libc::c_int *
                           16 as libc::c_int + 37 as libc::c_int) as usize] =
        10 as libc::c_int as s16;
    (*gGameInfo).data[(14 as libc::c_int * 6 as libc::c_int *
                           16 as libc::c_int + 38 as libc::c_int) as usize] =
        2 as libc::c_int as s16;
    (*gGameInfo).data[(14 as libc::c_int * 6 as libc::c_int *
                           16 as libc::c_int + 39 as libc::c_int) as usize] =
        140 as libc::c_int as s16;
    (*gGameInfo).data[(14 as libc::c_int * 6 as libc::c_int *
                           16 as libc::c_int + 40 as libc::c_int) as usize] =
        20 as libc::c_int as s16;
    (*gGameInfo).data[(14 as libc::c_int * 6 as libc::c_int *
                           16 as libc::c_int + 41 as libc::c_int) as usize] =
        300 as libc::c_int as s16;
    (*gGameInfo).data[(14 as libc::c_int * 6 as libc::c_int *
                           16 as libc::c_int + 42 as libc::c_int) as usize] =
        100 as libc::c_int as s16;
    (*gGameInfo).data[(14 as libc::c_int * 6 as libc::c_int *
                           16 as libc::c_int + 43 as libc::c_int) as usize] =
        70 as libc::c_int as s16;
    (*gGameInfo).data[(14 as libc::c_int * 6 as libc::c_int *
                           16 as libc::c_int + 44 as libc::c_int) as usize] =
        50 as libc::c_int as s16;
    (*gGameInfo).data[(14 as libc::c_int * 6 as libc::c_int *
                           16 as libc::c_int + 45 as libc::c_int) as usize] =
        36 as libc::c_int as s16;
    (*gGameInfo).data[(14 as libc::c_int * 6 as libc::c_int *
                           16 as libc::c_int + 46 as libc::c_int) as usize] =
        16 as libc::c_int as s16;
    (*gGameInfo).data[(14 as libc::c_int * 6 as libc::c_int *
                           16 as libc::c_int + 47 as libc::c_int) as usize] =
        8 as libc::c_int as s16;
    (*gGameInfo).data[(14 as libc::c_int * 6 as libc::c_int *
                           16 as libc::c_int + 48 as libc::c_int) as usize] =
        34 as libc::c_int as s16;
    (*gGameInfo).data[(14 as libc::c_int * 6 as libc::c_int *
                           16 as libc::c_int + 49 as libc::c_int) as usize] =
        18 as libc::c_int as s16;
    (*gGameInfo).data[(14 as libc::c_int * 6 as libc::c_int *
                           16 as libc::c_int + 50 as libc::c_int) as usize] =
        42 as libc::c_int as s16;
    (*gGameInfo).data[(14 as libc::c_int * 6 as libc::c_int *
                           16 as libc::c_int + 51 as libc::c_int) as usize] =
        26 as libc::c_int as s16;
    (*gGameInfo).data[(14 as libc::c_int * 6 as libc::c_int *
                           16 as libc::c_int + 52 as libc::c_int) as usize] =
        0 as libc::c_int as s16;
    (*gGameInfo).data[(14 as libc::c_int * 6 as libc::c_int *
                           16 as libc::c_int + 53 as libc::c_int) as usize] =
        1 as libc::c_int as s16;
    (*gGameInfo).data[(14 as libc::c_int * 6 as libc::c_int *
                           16 as libc::c_int + 54 as libc::c_int) as usize] =
        65 as libc::c_int as s16;
    (*gGameInfo).data[(14 as libc::c_int * 6 as libc::c_int *
                           16 as libc::c_int + 55 as libc::c_int) as usize] =
        60 as libc::c_int as s16;
    (*gGameInfo).data[(14 as libc::c_int * 6 as libc::c_int *
                           16 as libc::c_int + 56 as libc::c_int) as usize] =
        16 as libc::c_int as s16;
    (*gGameInfo).data[(14 as libc::c_int * 6 as libc::c_int *
                           16 as libc::c_int + 57 as libc::c_int) as usize] =
        80 as libc::c_int as s16;
    (*gGameInfo).data[(14 as libc::c_int * 6 as libc::c_int *
                           16 as libc::c_int + 58 as libc::c_int) as usize] =
        80 as libc::c_int as s16;
    (*gGameInfo).data[(14 as libc::c_int * 6 as libc::c_int *
                           16 as libc::c_int + 59 as libc::c_int) as usize] =
        12 as libc::c_int as s16;
    (*gGameInfo).data[(14 as libc::c_int * 6 as libc::c_int *
                           16 as libc::c_int + 60 as libc::c_int) as usize] =
        1 as libc::c_int as s16;
    (*gGameInfo).data[(14 as libc::c_int * 6 as libc::c_int *
                           16 as libc::c_int + 61 as libc::c_int) as usize] =
        3 as libc::c_int as s16;
    (*gGameInfo).data[(14 as libc::c_int * 6 as libc::c_int *
                           16 as libc::c_int + 62 as libc::c_int) as usize] =
        0 as libc::c_int as s16;
    (*gGameInfo).data[(14 as libc::c_int * 6 as libc::c_int *
                           16 as libc::c_int + 63 as libc::c_int) as usize] =
        100 as libc::c_int as s16;
    (*gGameInfo).data[(14 as libc::c_int * 6 as libc::c_int *
                           16 as libc::c_int + 64 as libc::c_int) as usize] =
        158 as libc::c_int as s16;
    (*gGameInfo).data[(14 as libc::c_int * 6 as libc::c_int *
                           16 as libc::c_int + 65 as libc::c_int) as usize] =
        102 as libc::c_int as s16;
    (*gGameInfo).data[(14 as libc::c_int * 6 as libc::c_int *
                           16 as libc::c_int + 66 as libc::c_int) as usize] =
        48 as libc::c_int as s16;
    (*gGameInfo).data[(14 as libc::c_int * 6 as libc::c_int *
                           16 as libc::c_int + 67 as libc::c_int +
                           0 as libc::c_int) as usize] =
        54 as libc::c_int as s16;
    (*gGameInfo).data[(14 as libc::c_int * 6 as libc::c_int *
                           16 as libc::c_int + 67 as libc::c_int +
                           1 as libc::c_int) as usize] =
        70 as libc::c_int as s16;
    (*gGameInfo).data[(14 as libc::c_int * 6 as libc::c_int *
                           16 as libc::c_int + 67 as libc::c_int +
                           2 as libc::c_int) as usize] =
        86 as libc::c_int as s16;
    (*gGameInfo).data[(14 as libc::c_int * 6 as libc::c_int *
                           16 as libc::c_int + 70 as libc::c_int) as usize] =
        -(300 as libc::c_int) as s16;
    (*gGameInfo).data[(14 as libc::c_int * 6 as libc::c_int *
                           16 as libc::c_int + 71 as libc::c_int) as usize] =
        0 as libc::c_int as s16;
    (*gGameInfo).data[(14 as libc::c_int * 6 as libc::c_int *
                           16 as libc::c_int + 72 as libc::c_int) as usize] =
        54 as libc::c_int as s16;
    (*gGameInfo).data[(14 as libc::c_int * 6 as libc::c_int *
                           16 as libc::c_int + 73 as libc::c_int) as usize] =
        48 as libc::c_int as s16;
    (*gGameInfo).data[(14 as libc::c_int * 6 as libc::c_int *
                           16 as libc::c_int + 74 as libc::c_int) as usize] =
        128 as libc::c_int as s16;
    (*gGameInfo).data[(14 as libc::c_int * 6 as libc::c_int *
                           16 as libc::c_int + 75 as libc::c_int) as usize] =
        64 as libc::c_int as s16;
    (*gGameInfo).data[(14 as libc::c_int * 6 as libc::c_int *
                           16 as libc::c_int + 76 as libc::c_int) as usize] =
        2048 as libc::c_int as s16;
    (*gGameInfo).data[(14 as libc::c_int * 6 as libc::c_int *
                           16 as libc::c_int + 77 as libc::c_int) as usize] =
        512 as libc::c_int as s16;
    (*gGameInfo).data[(14 as libc::c_int * 6 as libc::c_int *
                           16 as libc::c_int + 78 as libc::c_int) as usize] =
        96 as libc::c_int as s16;
    (*gGameInfo).data[(14 as libc::c_int * 6 as libc::c_int *
                           16 as libc::c_int + 79 as libc::c_int) as usize] =
        98 as libc::c_int as s16;
    (*gGameInfo).data[(14 as libc::c_int * 6 as libc::c_int *
                           16 as libc::c_int + 80 as libc::c_int) as usize] =
        0 as libc::c_int as s16;
    (*gGameInfo).data[(14 as libc::c_int * 6 as libc::c_int *
                           16 as libc::c_int + 81 as libc::c_int) as usize] =
        50 as libc::c_int as s16;
    (*gGameInfo).data[(14 as libc::c_int * 6 as libc::c_int *
                           16 as libc::c_int + 82 as libc::c_int) as usize] =
        25 as libc::c_int as s16;
    (*gGameInfo).data[(14 as libc::c_int * 6 as libc::c_int *
                           16 as libc::c_int + 83 as libc::c_int) as usize] =
        100 as libc::c_int as s16;
    (*gGameInfo).data[(14 as libc::c_int * 6 as libc::c_int *
                           16 as libc::c_int + 84 as libc::c_int) as usize] =
        100 as libc::c_int as s16;
    (*gGameInfo).data[(14 as libc::c_int * 6 as libc::c_int *
                           16 as libc::c_int + 85 as libc::c_int) as usize] =
        0 as libc::c_int as s16;
    (*gGameInfo).data[(14 as libc::c_int * 6 as libc::c_int *
                           16 as libc::c_int + 86 as libc::c_int) as usize] =
        0 as libc::c_int as s16;
    (*gGameInfo).data[(14 as libc::c_int * 6 as libc::c_int *
                           16 as libc::c_int + 87 as libc::c_int) as usize] =
        0 as libc::c_int as s16;
    (*gGameInfo).data[(14 as libc::c_int * 6 as libc::c_int *
                           16 as libc::c_int + 88 as libc::c_int) as usize] =
        -(50 as libc::c_int) as s16;
    (*gGameInfo).data[(14 as libc::c_int * 6 as libc::c_int *
                           16 as libc::c_int + 89 as libc::c_int) as usize] =
        -(100 as libc::c_int) as s16;
    (*gGameInfo).data[(14 as libc::c_int * 6 as libc::c_int *
                           16 as libc::c_int + 90 as libc::c_int) as usize] =
        -(500 as libc::c_int) as s16;
    (*gGameInfo).data[(14 as libc::c_int * 6 as libc::c_int *
                           16 as libc::c_int + 91 as libc::c_int) as usize] =
        0 as libc::c_int as s16;
    (*gGameInfo).data[(14 as libc::c_int * 6 as libc::c_int *
                           16 as libc::c_int + 92 as libc::c_int) as usize] =
        100 as libc::c_int as s16;
    (*gGameInfo).data[(14 as libc::c_int * 6 as libc::c_int *
                           16 as libc::c_int + 93 as libc::c_int) as usize] =
        100 as libc::c_int as s16;
    (*gGameInfo).data[(14 as libc::c_int * 6 as libc::c_int *
                           16 as libc::c_int + 94 as libc::c_int) as usize] =
        160 as libc::c_int as s16;
    (*gGameInfo).data[(14 as libc::c_int * 6 as libc::c_int *
                           16 as libc::c_int + 95 as libc::c_int) as usize] =
        200 as libc::c_int as s16;
    (*gGameInfo).data[(18 as libc::c_int * 6 as libc::c_int *
                           16 as libc::c_int + 2 as libc::c_int) as usize] =
        -(6080 as libc::c_int) as s16;
    (*gGameInfo).data[(18 as libc::c_int * 6 as libc::c_int *
                           16 as libc::c_int + 3 as libc::c_int) as usize] =
        9355 as libc::c_int as s16;
    (*gGameInfo).data[(18 as libc::c_int * 6 as libc::c_int *
                           16 as libc::c_int + 4 as libc::c_int) as usize] =
        8 as libc::c_int as s16;
    (*gGameInfo).data[(18 as libc::c_int * 6 as libc::c_int *
                           16 as libc::c_int + 5 as libc::c_int) as usize] =
        3 as libc::c_int as s16;
    (*gGameInfo).data[(18 as libc::c_int * 6 as libc::c_int *
                           16 as libc::c_int + 6 as libc::c_int) as usize] =
        8 as libc::c_int as s16;
    (*gGameInfo).data[(18 as libc::c_int * 6 as libc::c_int *
                           16 as libc::c_int + 7 as libc::c_int) as usize] =
        0 as libc::c_int as s16;
    (*gGameInfo).data[(18 as libc::c_int * 6 as libc::c_int *
                           16 as libc::c_int + 8 as libc::c_int) as usize] =
        100 as libc::c_int as s16;
    (*gGameInfo).data[(18 as libc::c_int * 6 as libc::c_int *
                           16 as libc::c_int + 9 as libc::c_int) as usize] =
        109 as libc::c_int as s16;
    (*gGameInfo).data[(18 as libc::c_int * 6 as libc::c_int *
                           16 as libc::c_int + 10 as libc::c_int) as usize] =
        151 as libc::c_int as s16;
    (*gGameInfo).data[(18 as libc::c_int * 6 as libc::c_int *
                           16 as libc::c_int + 11 as libc::c_int) as usize] =
        148 as libc::c_int as s16;
    (*gGameInfo).data[(18 as libc::c_int * 6 as libc::c_int *
                           16 as libc::c_int + 12 as libc::c_int) as usize] =
        23 as libc::c_int as s16;
    (*gGameInfo).data[(18 as libc::c_int * 6 as libc::c_int *
                           16 as libc::c_int + 13 as libc::c_int) as usize] =
        22 as libc::c_int as s16;
    (*gGameInfo).data[(18 as libc::c_int * 6 as libc::c_int *
                           16 as libc::c_int + 14 as libc::c_int) as usize] =
        -(380 as libc::c_int) as s16;
    (*gGameInfo).data[(18 as libc::c_int * 6 as libc::c_int *
                           16 as libc::c_int + 15 as libc::c_int) as usize] =
        -(350 as libc::c_int) as s16;
    (*gGameInfo).data[(18 as libc::c_int * 6 as libc::c_int *
                           16 as libc::c_int + 16 as libc::c_int) as usize] =
        -(175 as libc::c_int) as s16;
    (*gGameInfo).data[(18 as libc::c_int * 6 as libc::c_int *
                           16 as libc::c_int + 17 as libc::c_int) as usize] =
        155 as libc::c_int as s16;
    (*gGameInfo).data[(18 as libc::c_int * 6 as libc::c_int *
                           16 as libc::c_int + 18 as libc::c_int) as usize] =
        10 as libc::c_int as s16;
    (*gGameInfo).data[(18 as libc::c_int * 6 as libc::c_int *
                           16 as libc::c_int + 19 as libc::c_int) as usize] =
        10 as libc::c_int as s16;
    (*gGameInfo).data[(18 as libc::c_int * 6 as libc::c_int *
                           16 as libc::c_int + 20 as libc::c_int) as usize] =
        -(50 as libc::c_int) as s16;
    (*gGameInfo).data[(18 as libc::c_int * 6 as libc::c_int *
                           16 as libc::c_int + 21 as libc::c_int) as usize] =
        -(54 as libc::c_int) as s16;
    (*gGameInfo).data[(18 as libc::c_int * 6 as libc::c_int *
                           16 as libc::c_int + 22 as libc::c_int) as usize] =
        -(32 as libc::c_int) as s16;
    (*gGameInfo).data[(18 as libc::c_int * 6 as libc::c_int *
                           16 as libc::c_int + 23 as libc::c_int) as usize] =
        -(38 as libc::c_int) as s16;
    (*gGameInfo).data[(18 as libc::c_int * 6 as libc::c_int *
                           16 as libc::c_int + 24 as libc::c_int) as usize] =
        -(36 as libc::c_int) as s16;
    (*gGameInfo).data[(18 as libc::c_int * 6 as libc::c_int *
                           16 as libc::c_int + 25 as libc::c_int) as usize] =
        40 as libc::c_int as s16;
    (*gGameInfo).data[(18 as libc::c_int * 6 as libc::c_int *
                           16 as libc::c_int + 26 as libc::c_int) as usize] =
        -(40 as libc::c_int) as s16;
    (*gGameInfo).data[(18 as libc::c_int * 6 as libc::c_int *
                           16 as libc::c_int + 27 as libc::c_int) as usize] =
        0 as libc::c_int as s16;
    (*gGameInfo).data[(18 as libc::c_int * 6 as libc::c_int *
                           16 as libc::c_int + 28 as libc::c_int) as usize] =
        0 as libc::c_int as s16;
    (*gGameInfo).data[(18 as libc::c_int * 6 as libc::c_int *
                           16 as libc::c_int + 29 as libc::c_int) as usize] =
        238 as libc::c_int as s16;
    (*gGameInfo).data[(18 as libc::c_int * 6 as libc::c_int *
                           16 as libc::c_int + 30 as libc::c_int) as usize] =
        164 as libc::c_int as s16;
    (*gGameInfo).data[(18 as libc::c_int * 6 as libc::c_int *
                           16 as libc::c_int + 31 as libc::c_int) as usize] =
        0 as libc::c_int as s16;
    (*gGameInfo).data[(18 as libc::c_int * 6 as libc::c_int *
                           16 as libc::c_int + 32 as libc::c_int) as usize] =
        122 as libc::c_int as s16;
    (*gGameInfo).data[(18 as libc::c_int * 6 as libc::c_int *
                           16 as libc::c_int + 33 as libc::c_int) as usize] =
        60 as libc::c_int as s16;
    (*gGameInfo).data[(18 as libc::c_int * 6 as libc::c_int *
                           16 as libc::c_int + 35 as libc::c_int) as usize] =
        0 as libc::c_int as s16;
    (*gGameInfo).data[(18 as libc::c_int * 6 as libc::c_int *
                           16 as libc::c_int + 36 as libc::c_int) as usize] =
        0 as libc::c_int as s16;
    (*gGameInfo).data[(18 as libc::c_int * 6 as libc::c_int *
                           16 as libc::c_int + 37 as libc::c_int) as usize] =
        100 as libc::c_int as s16;
    (*gGameInfo).data[(18 as libc::c_int * 6 as libc::c_int *
                           16 as libc::c_int + 38 as libc::c_int) as usize] =
        99 as libc::c_int as s16;
    (*gGameInfo).data[(18 as libc::c_int * 6 as libc::c_int *
                           16 as libc::c_int + 39 as libc::c_int) as usize] =
        109 as libc::c_int as s16;
    (*gGameInfo).data[(18 as libc::c_int * 6 as libc::c_int *
                           16 as libc::c_int + 40 as libc::c_int +
                           0 as libc::c_int) as usize] =
        (160 as libc::c_int - 9 as libc::c_int) as s16;
    (*gGameInfo).data[(18 as libc::c_int * 6 as libc::c_int *
                           16 as libc::c_int + 40 as libc::c_int +
                           1 as libc::c_int) as usize] =
        (160 as libc::c_int - 11 as libc::c_int) as s16;
    (*gGameInfo).data[(18 as libc::c_int * 6 as libc::c_int *
                           16 as libc::c_int + 40 as libc::c_int +
                           2 as libc::c_int) as usize] =
        (160 as libc::c_int - 12 as libc::c_int) as s16;
    (*gGameInfo).data[(18 as libc::c_int * 6 as libc::c_int *
                           16 as libc::c_int + 43 as libc::c_int +
                           0 as libc::c_int) as usize] =
        (17 as libc::c_int + 6 as libc::c_int) as s16;
    (*gGameInfo).data[(18 as libc::c_int * 6 as libc::c_int *
                           16 as libc::c_int + 43 as libc::c_int +
                           1 as libc::c_int) as usize] =
        (17 as libc::c_int + 5 as libc::c_int) as s16;
    (*gGameInfo).data[(18 as libc::c_int * 6 as libc::c_int *
                           16 as libc::c_int + 43 as libc::c_int +
                           2 as libc::c_int) as usize] =
        (17 as libc::c_int + 5 as libc::c_int) as s16;
    (*gGameInfo).data[(18 as libc::c_int * 6 as libc::c_int *
                           16 as libc::c_int + 46 as libc::c_int) as usize] =
        -(380 as libc::c_int) as s16;
    (*gGameInfo).data[(18 as libc::c_int * 6 as libc::c_int *
                           16 as libc::c_int + 47 as libc::c_int) as usize] =
        -(360 as libc::c_int) as s16;
    (*gGameInfo).data[(18 as libc::c_int * 6 as libc::c_int *
                           16 as libc::c_int + 48 as libc::c_int) as usize] =
        -(350 as libc::c_int) as s16;
    (*gGameInfo).data[(18 as libc::c_int * 6 as libc::c_int *
                           16 as libc::c_int + 49 as libc::c_int) as usize] =
        -(48 as libc::c_int) as s16;
    (*gGameInfo).data[(18 as libc::c_int * 6 as libc::c_int *
                           16 as libc::c_int + 50 as libc::c_int) as usize] =
        16 as libc::c_int as s16;
    (*gGameInfo).data[(18 as libc::c_int * 6 as libc::c_int *
                           16 as libc::c_int + 51 as libc::c_int) as usize] =
        -(62 as libc::c_int) as s16;
    (*gGameInfo).data[(18 as libc::c_int * 6 as libc::c_int *
                           16 as libc::c_int + 52 as libc::c_int) as usize] =
        22 as libc::c_int as s16;
    (*gGameInfo).data[(18 as libc::c_int * 6 as libc::c_int *
                           16 as libc::c_int + 53 as libc::c_int) as usize] =
        -(84 as libc::c_int) as s16;
    (*gGameInfo).data[(18 as libc::c_int * 6 as libc::c_int *
                           16 as libc::c_int + 54 as libc::c_int) as usize] =
        20 as libc::c_int as s16;
    (*gGameInfo).data[(18 as libc::c_int * 6 as libc::c_int *
                           16 as libc::c_int + 55 as libc::c_int) as usize] =
        -(53 as libc::c_int) as s16;
    (*gGameInfo).data[(18 as libc::c_int * 6 as libc::c_int *
                           16 as libc::c_int + 56 as libc::c_int) as usize] =
        40 as libc::c_int as s16;
    (*gGameInfo).data[(18 as libc::c_int * 6 as libc::c_int *
                           16 as libc::c_int + 57 as libc::c_int) as usize] =
        -(64 as libc::c_int) as s16;
    (*gGameInfo).data[(18 as libc::c_int * 6 as libc::c_int *
                           16 as libc::c_int + 58 as libc::c_int) as usize] =
        47 as libc::c_int as s16;
    (*gGameInfo).data[(18 as libc::c_int * 6 as libc::c_int *
                           16 as libc::c_int + 59 as libc::c_int) as usize] =
        -(84 as libc::c_int) as s16;
    (*gGameInfo).data[(18 as libc::c_int * 6 as libc::c_int *
                           16 as libc::c_int + 60 as libc::c_int) as usize] =
        44 as libc::c_int as s16;
    (*gGameInfo).data[(18 as libc::c_int * 6 as libc::c_int *
                           16 as libc::c_int + 61 as libc::c_int) as usize] =
        -(42 as libc::c_int) as s16;
    (*gGameInfo).data[(18 as libc::c_int * 6 as libc::c_int *
                           16 as libc::c_int + 62 as libc::c_int) as usize] =
        32 as libc::c_int as s16;
    (*gGameInfo).data[(18 as libc::c_int * 6 as libc::c_int *
                           16 as libc::c_int + 63 as libc::c_int) as usize] =
        -(45 as libc::c_int) as s16;
    (*gGameInfo).data[(18 as libc::c_int * 6 as libc::c_int *
                           16 as libc::c_int + 64 as libc::c_int) as usize] =
        -(37 as libc::c_int) as s16;
    (*gGameInfo).data[(18 as libc::c_int * 6 as libc::c_int *
                           16 as libc::c_int + 65 as libc::c_int) as usize] =
        30 as libc::c_int as s16;
    (*gGameInfo).data[(18 as libc::c_int * 6 as libc::c_int *
                           16 as libc::c_int + 66 as libc::c_int) as usize] =
        -(50 as libc::c_int) as s16;
    (*gGameInfo).data[(18 as libc::c_int * 6 as libc::c_int *
                           16 as libc::c_int + 68 as libc::c_int) as usize] =
        204 as libc::c_int as s16;
    (*gGameInfo).data[(18 as libc::c_int * 6 as libc::c_int *
                           16 as libc::c_int + 69 as libc::c_int) as usize] =
        140 as libc::c_int as s16;
    (*gGameInfo).data[(18 as libc::c_int * 6 as libc::c_int *
                           16 as libc::c_int + 87 as libc::c_int) as usize] =
        80 as libc::c_int as s16;
    (*gGameInfo).data[(18 as libc::c_int * 6 as libc::c_int *
                           16 as libc::c_int + 88 as libc::c_int) as usize] =
        70 as libc::c_int as s16;
    (*gGameInfo).data[(18 as libc::c_int * 6 as libc::c_int *
                           16 as libc::c_int + 89 as libc::c_int) as usize] =
        40 as libc::c_int as s16;
    (*gGameInfo).data[(18 as libc::c_int * 6 as libc::c_int *
                           16 as libc::c_int + 90 as libc::c_int) as usize] =
        320 as libc::c_int as s16;
    (*gGameInfo).data[(18 as libc::c_int * 6 as libc::c_int *
                           16 as libc::c_int + 91 as libc::c_int) as usize] =
        40 as libc::c_int as s16;
    (*gGameInfo).data[(18 as libc::c_int * 6 as libc::c_int *
                           16 as libc::c_int + 92 as libc::c_int) as usize] =
        3 as libc::c_int as s16;
    (*gGameInfo).data[(18 as libc::c_int * 6 as libc::c_int *
                           16 as libc::c_int + 93 as libc::c_int) as usize] =
        6 as libc::c_int as s16;
    (*gGameInfo).data[(18 as libc::c_int * 6 as libc::c_int *
                           16 as libc::c_int + 94 as libc::c_int) as usize] =
        3 as libc::c_int as s16;
    (*gGameInfo).data[(18 as libc::c_int * 6 as libc::c_int *
                           16 as libc::c_int + 95 as libc::c_int) as usize] =
        6 as libc::c_int as s16;
    if gSaveContext.gameMode == 0 as libc::c_int {
        (*gGameInfo).data[(20 as libc::c_int * 6 as libc::c_int *
                               16 as libc::c_int + 0 as libc::c_int) as usize]
            = 52 as libc::c_int as s16;
        (*gGameInfo).data[(20 as libc::c_int * 6 as libc::c_int *
                               16 as libc::c_int + 1 as libc::c_int) as usize]
            = 36 as libc::c_int as s16;
        (*gGameInfo).data[(20 as libc::c_int * 6 as libc::c_int *
                               16 as libc::c_int + 2 as libc::c_int) as usize]
            = 214 as libc::c_int as s16;
        (*gGameInfo).data[(20 as libc::c_int * 6 as libc::c_int *
                               16 as libc::c_int + 3 as libc::c_int) as usize]
            = 76 as libc::c_int as s16;
        (*gGameInfo).data[(20 as libc::c_int * 6 as libc::c_int *
                               16 as libc::c_int + 4 as libc::c_int) as usize]
            = 304 as libc::c_int as s16;
        (*gGameInfo).data[(20 as libc::c_int * 6 as libc::c_int *
                               16 as libc::c_int + 5 as libc::c_int) as usize]
            = 430 as libc::c_int as s16;
        (*gGameInfo).data[(20 as libc::c_int * 6 as libc::c_int *
                               16 as libc::c_int + 6 as libc::c_int) as usize]
            = 1 as libc::c_int as s16;
        (*gGameInfo).data[(20 as libc::c_int * 6 as libc::c_int *
                               16 as libc::c_int + 7 as libc::c_int) as usize]
            = 78 as libc::c_int as s16;
        (*gGameInfo).data[(20 as libc::c_int * 6 as libc::c_int *
                               16 as libc::c_int + 8 as libc::c_int) as usize]
            = 166 as libc::c_int as s16;
        (*gGameInfo).data[(20 as libc::c_int * 6 as libc::c_int *
                               16 as libc::c_int + 9 as libc::c_int) as usize]
            = 40 as libc::c_int as s16;
        (*gGameInfo).data[(20 as libc::c_int * 6 as libc::c_int *
                               16 as libc::c_int + 14 as libc::c_int) as
                              usize] = 32 as libc::c_int as s16;
        (*gGameInfo).data[(20 as libc::c_int * 6 as libc::c_int *
                               16 as libc::c_int + 15 as libc::c_int) as
                              usize] = 32 as libc::c_int as s16;
        (*gGameInfo).data[(20 as libc::c_int * 6 as libc::c_int *
                               16 as libc::c_int + 16 as libc::c_int) as
                              usize] = 110 as libc::c_int as s16;
        (*gGameInfo).data[(20 as libc::c_int * 6 as libc::c_int *
                               16 as libc::c_int + 17 as libc::c_int) as
                              usize] = -(740 as libc::c_int) as s16;
        (*gGameInfo).data[(20 as libc::c_int * 6 as libc::c_int *
                               16 as libc::c_int + 18 as libc::c_int +
                               0 as libc::c_int) as usize] =
            0 as libc::c_int as s16;
        (*gGameInfo).data[(20 as libc::c_int * 6 as libc::c_int *
                               16 as libc::c_int + 18 as libc::c_int +
                               1 as libc::c_int) as usize] =
            255 as libc::c_int as s16;
        (*gGameInfo).data[(20 as libc::c_int * 6 as libc::c_int *
                               16 as libc::c_int + 18 as libc::c_int +
                               2 as libc::c_int) as usize] =
            255 as libc::c_int as s16
    }
    (*gGameInfo).data[(20 as libc::c_int * 6 as libc::c_int *
                           16 as libc::c_int + 21 as libc::c_int) as usize] =
        0 as libc::c_int as s16;
    (*gGameInfo).data[(20 as libc::c_int * 6 as libc::c_int *
                           16 as libc::c_int + 22 as libc::c_int) as usize] =
        0 as libc::c_int as s16;
    (*gGameInfo).data[(20 as libc::c_int * 6 as libc::c_int *
                           16 as libc::c_int + 23 as libc::c_int) as usize] =
        0 as libc::c_int as s16;
    (*gGameInfo).data[(20 as libc::c_int * 6 as libc::c_int *
                           16 as libc::c_int + 24 as libc::c_int) as usize] =
        0 as libc::c_int as s16;
    (*gGameInfo).data[(20 as libc::c_int * 6 as libc::c_int *
                           16 as libc::c_int + 25 as libc::c_int) as usize] =
        0 as libc::c_int as s16;
    (*gGameInfo).data[(20 as libc::c_int * 6 as libc::c_int *
                           16 as libc::c_int + 26 as libc::c_int) as usize] =
        0 as libc::c_int as s16;
    (*gGameInfo).data[(20 as libc::c_int * 6 as libc::c_int *
                           16 as libc::c_int + 27 as libc::c_int) as usize] =
        0 as libc::c_int as s16;
    (*gGameInfo).data[(20 as libc::c_int * 6 as libc::c_int *
                           16 as libc::c_int + 28 as libc::c_int) as usize] =
        98 as libc::c_int as s16;
    (*gGameInfo).data[(20 as libc::c_int * 6 as libc::c_int *
                           16 as libc::c_int + 29 as libc::c_int) as usize] =
        18 as libc::c_int as s16;
    (*gGameInfo).data[(20 as libc::c_int * 6 as libc::c_int *
                           16 as libc::c_int + 30 as libc::c_int) as usize] =
        0 as libc::c_int as s16;
    (*gGameInfo).data[(20 as libc::c_int * 6 as libc::c_int *
                           16 as libc::c_int + 31 as libc::c_int) as usize] =
        0 as libc::c_int as s16;
    (*gGameInfo).data[(20 as libc::c_int * 6 as libc::c_int *
                           16 as libc::c_int + 32 as libc::c_int) as usize] =
        0 as libc::c_int as s16;
    (*gGameInfo).data[(20 as libc::c_int * 6 as libc::c_int *
                           16 as libc::c_int + 33 as libc::c_int) as usize] =
        70 as libc::c_int as s16;
    (*gGameInfo).data[(20 as libc::c_int * 6 as libc::c_int *
                           16 as libc::c_int + 34 as libc::c_int) as usize] =
        255 as libc::c_int as s16;
    (*gGameInfo).data[(20 as libc::c_int * 6 as libc::c_int *
                           16 as libc::c_int + 35 as libc::c_int) as usize] =
        80 as libc::c_int as s16;
    (*gGameInfo).data[(20 as libc::c_int * 6 as libc::c_int *
                           16 as libc::c_int + 36 as libc::c_int) as usize] =
        70 as libc::c_int as s16;
    (*gGameInfo).data[(20 as libc::c_int * 6 as libc::c_int *
                           16 as libc::c_int + 37 as libc::c_int) as usize] =
        255 as libc::c_int as s16;
    (*gGameInfo).data[(20 as libc::c_int * 6 as libc::c_int *
                           16 as libc::c_int + 38 as libc::c_int) as usize] =
        80 as libc::c_int as s16;
    (*gGameInfo).data[(20 as libc::c_int * 6 as libc::c_int *
                           16 as libc::c_int + 40 as libc::c_int) as usize] =
        9 as libc::c_int as s16;
    (*gGameInfo).data[(20 as libc::c_int * 6 as libc::c_int *
                           16 as libc::c_int + 42 as libc::c_int) as usize] =
        250 as libc::c_int as s16;
    (*gGameInfo).data[(20 as libc::c_int * 6 as libc::c_int *
                           16 as libc::c_int + 43 as libc::c_int) as usize] =
        440 as libc::c_int as s16;
    (*gGameInfo).data[(20 as libc::c_int * 6 as libc::c_int *
                           16 as libc::c_int + 44 as libc::c_int) as usize] =
        10 as libc::c_int as s16;
    (*gGameInfo).data[(20 as libc::c_int * 6 as libc::c_int *
                           16 as libc::c_int + 45 as libc::c_int +
                           0 as libc::c_int) as usize] =
        190 as libc::c_int as s16;
    (*gGameInfo).data[(20 as libc::c_int * 6 as libc::c_int *
                           16 as libc::c_int + 45 as libc::c_int +
                           1 as libc::c_int) as usize] =
        184 as libc::c_int as s16;
    (*gGameInfo).data[(20 as libc::c_int * 6 as libc::c_int *
                           16 as libc::c_int + 45 as libc::c_int +
                           2 as libc::c_int) as usize] =
        176 as libc::c_int as s16;
    (*gGameInfo).data[(20 as libc::c_int * 6 as libc::c_int *
                           16 as libc::c_int + 45 as libc::c_int +
                           3 as libc::c_int) as usize] =
        172 as libc::c_int as s16;
    (*gGameInfo).data[(20 as libc::c_int * 6 as libc::c_int *
                           16 as libc::c_int + 45 as libc::c_int +
                           4 as libc::c_int) as usize] =
        170 as libc::c_int as s16;
    (*gGameInfo).data[(20 as libc::c_int * 6 as libc::c_int *
                           16 as libc::c_int + 50 as libc::c_int) as usize] =
        30 as libc::c_int as s16;
    (*gGameInfo).data[(20 as libc::c_int * 6 as libc::c_int *
                           16 as libc::c_int + 51 as libc::c_int) as usize] =
        0 as libc::c_int as s16;
    (*gGameInfo).data[(20 as libc::c_int * 6 as libc::c_int *
                           16 as libc::c_int + 52 as libc::c_int) as usize] =
        -(16 as libc::c_int) as s16;
    (*gGameInfo).data[(20 as libc::c_int * 6 as libc::c_int *
                           16 as libc::c_int + 53 as libc::c_int) as usize] =
        230 as libc::c_int as s16;
    (*gGameInfo).data[(20 as libc::c_int * 6 as libc::c_int *
                           16 as libc::c_int + 54 as libc::c_int) as usize] =
        230 as libc::c_int as s16;
    (*gGameInfo).data[(20 as libc::c_int * 6 as libc::c_int *
                           16 as libc::c_int + 55 as libc::c_int) as usize] =
        120 as libc::c_int as s16;
    (*gGameInfo).data[(20 as libc::c_int * 6 as libc::c_int *
                           16 as libc::c_int + 56 as libc::c_int) as usize] =
        -(720 as libc::c_int) as s16;
    (*gGameInfo).data[(20 as libc::c_int * 6 as libc::c_int *
                           16 as libc::c_int + 57 as libc::c_int) as usize] =
        255 as libc::c_int as s16;
    (*gGameInfo).data[(20 as libc::c_int * 6 as libc::c_int *
                           16 as libc::c_int + 58 as libc::c_int) as usize] =
        255 as libc::c_int as s16;
    (*gGameInfo).data[(20 as libc::c_int * 6 as libc::c_int *
                           16 as libc::c_int + 59 as libc::c_int) as usize] =
        255 as libc::c_int as s16;
    (*gGameInfo).data[(20 as libc::c_int * 6 as libc::c_int *
                           16 as libc::c_int + 60 as libc::c_int) as usize] =
        20 as libc::c_int as s16;
    (*gGameInfo).data[(20 as libc::c_int * 6 as libc::c_int *
                           16 as libc::c_int + 61 as libc::c_int) as usize] =
        100 as libc::c_int as s16;
    (*gGameInfo).data[(20 as libc::c_int * 6 as libc::c_int *
                           16 as libc::c_int + 62 as libc::c_int) as usize] =
        0 as libc::c_int as s16;
    (*gGameInfo).data[(20 as libc::c_int * 6 as libc::c_int *
                           16 as libc::c_int + 63 as libc::c_int) as usize] =
        10 as libc::c_int as s16;
    (*gGameInfo).data[(20 as libc::c_int * 6 as libc::c_int *
                           16 as libc::c_int + 64 as libc::c_int +
                           1 as libc::c_int) as usize] =
        (227 as libc::c_int + 1 as libc::c_int) as s16;
    (*gGameInfo).data[(20 as libc::c_int * 6 as libc::c_int *
                           16 as libc::c_int + 64 as libc::c_int +
                           2 as libc::c_int) as usize] =
        (249 as libc::c_int + 1 as libc::c_int) as s16;
    (*gGameInfo).data[(20 as libc::c_int * 6 as libc::c_int *
                           16 as libc::c_int + 64 as libc::c_int +
                           3 as libc::c_int) as usize] =
        (271 as libc::c_int + 1 as libc::c_int) as s16;
    (*gGameInfo).data[(20 as libc::c_int * 6 as libc::c_int *
                           16 as libc::c_int + 68 as libc::c_int +
                           0 as libc::c_int) as usize] =
        (17 as libc::c_int + 18 as libc::c_int) as s16;
    (*gGameInfo).data[(20 as libc::c_int * 6 as libc::c_int *
                           16 as libc::c_int + 68 as libc::c_int +
                           1 as libc::c_int) as usize] =
        (18 as libc::c_int + 17 as libc::c_int) as s16;
    (*gGameInfo).data[(20 as libc::c_int * 6 as libc::c_int *
                           16 as libc::c_int + 68 as libc::c_int +
                           2 as libc::c_int) as usize] =
        (34 as libc::c_int + 17 as libc::c_int) as s16;
    (*gGameInfo).data[(20 as libc::c_int * 6 as libc::c_int *
                           16 as libc::c_int + 68 as libc::c_int +
                           3 as libc::c_int) as usize] =
        (18 as libc::c_int + 17 as libc::c_int) as s16;
    (*gGameInfo).data[(20 as libc::c_int * 6 as libc::c_int *
                           16 as libc::c_int + 72 as libc::c_int) as usize] =
        0 as libc::c_int as s16;
    (*gGameInfo).data[(20 as libc::c_int * 6 as libc::c_int *
                           16 as libc::c_int + 73 as libc::c_int) as usize] =
        0 as libc::c_int as s16;
    (*gGameInfo).data[(20 as libc::c_int * 6 as libc::c_int *
                           16 as libc::c_int + 74 as libc::c_int) as usize] =
        0 as libc::c_int as s16;
    (*gGameInfo).data[(20 as libc::c_int * 6 as libc::c_int *
                           16 as libc::c_int + 75 as libc::c_int) as usize] =
        0 as libc::c_int as s16;
    (*gGameInfo).data[(20 as libc::c_int * 6 as libc::c_int *
                           16 as libc::c_int + 76 as libc::c_int +
                           0 as libc::c_int) as usize] =
        30 as libc::c_int as s16;
    (*gGameInfo).data[(20 as libc::c_int * 6 as libc::c_int *
                           16 as libc::c_int + 76 as libc::c_int +
                           1 as libc::c_int) as usize] =
        24 as libc::c_int as s16;
    (*gGameInfo).data[(20 as libc::c_int * 6 as libc::c_int *
                           16 as libc::c_int + 76 as libc::c_int +
                           2 as libc::c_int) as usize] =
        24 as libc::c_int as s16;
    (*gGameInfo).data[(20 as libc::c_int * 6 as libc::c_int *
                           16 as libc::c_int + 76 as libc::c_int +
                           3 as libc::c_int) as usize] =
        24 as libc::c_int as s16;
    (*gGameInfo).data[(20 as libc::c_int * 6 as libc::c_int *
                           16 as libc::c_int + 80 as libc::c_int +
                           0 as libc::c_int) as usize] =
        29 as libc::c_int as s16;
    (*gGameInfo).data[(20 as libc::c_int * 6 as libc::c_int *
                           16 as libc::c_int + 80 as libc::c_int +
                           1 as libc::c_int) as usize] =
        27 as libc::c_int as s16;
    (*gGameInfo).data[(20 as libc::c_int * 6 as libc::c_int *
                           16 as libc::c_int + 80 as libc::c_int +
                           2 as libc::c_int) as usize] =
        27 as libc::c_int as s16;
    (*gGameInfo).data[(20 as libc::c_int * 6 as libc::c_int *
                           16 as libc::c_int + 80 as libc::c_int +
                           3 as libc::c_int) as usize] =
        27 as libc::c_int as s16;
    (*gGameInfo).data[(20 as libc::c_int * 6 as libc::c_int *
                           16 as libc::c_int + 84 as libc::c_int) as usize] =
        0 as libc::c_int as s16;
    (*gGameInfo).data[(20 as libc::c_int * 6 as libc::c_int *
                           16 as libc::c_int + 85 as libc::c_int) as usize] =
        50 as libc::c_int as s16;
    (*gGameInfo).data[(20 as libc::c_int * 6 as libc::c_int *
                           16 as libc::c_int + 86 as libc::c_int) as usize] =
        0 as libc::c_int as s16;
    (*gGameInfo).data[(20 as libc::c_int * 6 as libc::c_int *
                           16 as libc::c_int + 87 as libc::c_int) as usize] =
        64 as libc::c_int as s16;
    (*gGameInfo).data[(20 as libc::c_int * 6 as libc::c_int *
                           16 as libc::c_int + 88 as libc::c_int) as usize] =
        66 as libc::c_int as s16;
    (*gGameInfo).data[(20 as libc::c_int * 6 as libc::c_int *
                           16 as libc::c_int + 89 as libc::c_int) as usize] =
        0 as libc::c_int as s16;
    (*gGameInfo).data[(20 as libc::c_int * 6 as libc::c_int *
                           16 as libc::c_int + 90 as libc::c_int) as usize] =
        126 as libc::c_int as s16;
    (*gGameInfo).data[(20 as libc::c_int * 6 as libc::c_int *
                           16 as libc::c_int + 91 as libc::c_int) as usize] =
        124 as libc::c_int as s16;
    (*gGameInfo).data[(20 as libc::c_int * 6 as libc::c_int *
                           16 as libc::c_int + 92 as libc::c_int) as usize] =
        -(63 as libc::c_int) as s16;
}
#[no_mangle]
pub unsafe extern "C" fn func_80112098(mut globalCtx: *mut GlobalContext) {
    func_80111070();
}
