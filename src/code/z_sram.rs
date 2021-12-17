#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case,
         non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(const_raw_ptr_to_usize_cast, register_tool)]
extern "C" {
    #[no_mangle]
    fn __assert(exp: *const libc::c_char, file: *const libc::c_char,
                line: s32);
    #[no_mangle]
    fn osSyncPrintf(fmt: *const libc::c_char, _: ...);
    #[no_mangle]
    fn bzero(__s: *mut libc::c_void, __n: u32_0);
    #[no_mangle]
    fn MemCopy(dest: *mut libc::c_void, src: *mut libc::c_void, size: s32)
     -> *mut libc::c_void;
    #[no_mangle]
    fn Inventory_ChangeEquipment(equipment: s16, value: u16_0);
    #[no_mangle]
    static mut gSaveContext: SaveContext;
    #[no_mangle]
    static mut gSpoilingItemReverts: [s16; 3];
    #[no_mangle]
    static mut gSpoilingItems: [s16; 3];
    #[no_mangle]
    static mut gItemSlots: [u8_0; 56];
    #[no_mangle]
    static mut gScarecrowSpawnSongPtr: *mut u8_0;
    #[no_mangle]
    static mut gScarecrowCustomSongPtr: *mut OcarinaNote;
    #[no_mangle]
    fn SsSram_ReadWrite(addr: u32_0, dramAddr: *mut libc::c_void,
                        size: size_t, direction: s32);
    #[no_mangle]
    fn func_800F6700(outputMode: s8);
    #[no_mangle]
    fn GameState_Alloc(gameState: *mut GameState, size: size_t,
                       file: *mut libc::c_char, line: s32)
     -> *mut libc::c_void;
    #[no_mangle]
    static mut gBitFlags: [u32_0; 32];
    #[no_mangle]
    static mut gEquipShifts: [u8_0; 4];
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
    pub c2rust_unnamed: C2RustUnnamed_2,
    pub rgba: u32_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_2 {
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
    pub c2rust_unnamed: C2RustUnnamed_3,
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
pub union C2RustUnnamed_3 {
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
    pub c2rust_unnamed: C2RustUnnamed_4,
    pub normal: Vec3s,
    pub dist: s16,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_4 {
    pub vtxData: [u16_0; 3],
    pub c2rust_unnamed: C2RustUnnamed_5,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_5 {
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
    pub sides: [C2RustUnnamed_6; 2],
    pub id: s16,
    pub pos: Vec3s,
    pub rotY: s16,
    pub params: s16,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_6 {
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
    pub c2rust_unnamed: C2RustUnnamed_7,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_7 {
    pub single: C2RustUnnamed_9,
    pub multi: C2RustUnnamed_8,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_8 {
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
pub struct C2RustUnnamed_9 {
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
    pub restrictions: C2RustUnnamed_10,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_10 {
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
    pub c2rust_unnamed: C2RustUnnamed_11,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_11 {
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
    pub c2rust_unnamed: C2RustUnnamed_12,
    pub startPos: Vec3i,
    pub endPos: Vec3i,
    pub normal: Vec3i,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_12 {
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
    pub flags: C2RustUnnamed_13,
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
pub struct C2RustUnnamed_13 {
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
pub struct OcarinaNote {
    pub noteIdx: u8_0,
    pub unk_01: u8_0,
    pub unk_02: u16_0,
    pub volume: u8_0,
    pub vibrato: u8_0,
    pub tone: s8,
    pub semitone: u8_0,
}
pub type C2RustUnnamed_14 = libc::c_uint;
pub const SCENE_ID_MAX: C2RustUnnamed_14 = 110;
pub const SCENE_TESTROOM: C2RustUnnamed_14 = 109;
pub const SCENE_SASATEST: C2RustUnnamed_14 = 108;
pub const SCENE_HAIRAL_NIWA2: C2RustUnnamed_14 = 107;
pub const SCENE_SUTARU: C2RustUnnamed_14 = 106;
pub const SCENE_SYOTES2: C2RustUnnamed_14 = 105;
pub const SCENE_SYOTES: C2RustUnnamed_14 = 104;
pub const SCENE_DEPTH_TEST: C2RustUnnamed_14 = 103;
pub const SCENE_BESITU: C2RustUnnamed_14 = 102;
pub const SCENE_TEST01: C2RustUnnamed_14 = 101;
pub const SCENE_GANON_TOU: C2RustUnnamed_14 = 100;
pub const SCENE_SPOT20: C2RustUnnamed_14 = 99;
pub const SCENE_SPOT18: C2RustUnnamed_14 = 98;
pub const SCENE_SPOT17: C2RustUnnamed_14 = 97;
pub const SCENE_SPOT16: C2RustUnnamed_14 = 96;
pub const SCENE_SPOT15: C2RustUnnamed_14 = 95;
pub const SCENE_SPOT13: C2RustUnnamed_14 = 94;
pub const SCENE_SPOT12: C2RustUnnamed_14 = 93;
pub const SCENE_SPOT11: C2RustUnnamed_14 = 92;
pub const SCENE_SPOT10: C2RustUnnamed_14 = 91;
pub const SCENE_SPOT09: C2RustUnnamed_14 = 90;
pub const SCENE_SPOT08: C2RustUnnamed_14 = 89;
pub const SCENE_SPOT07: C2RustUnnamed_14 = 88;
pub const SCENE_SPOT06: C2RustUnnamed_14 = 87;
pub const SCENE_SPOT05: C2RustUnnamed_14 = 86;
pub const SCENE_SPOT04: C2RustUnnamed_14 = 85;
pub const SCENE_SPOT03: C2RustUnnamed_14 = 84;
pub const SCENE_SPOT02: C2RustUnnamed_14 = 83;
pub const SCENE_SPOT01: C2RustUnnamed_14 = 82;
pub const SCENE_SPOT00: C2RustUnnamed_14 = 81;
pub const SCENE_KINSUTA: C2RustUnnamed_14 = 80;
pub const SCENE_GANON_DEMO: C2RustUnnamed_14 = 79;
pub const SCENE_MAHOUYA: C2RustUnnamed_14 = 78;
pub const SCENE_MIHARIGOYA: C2RustUnnamed_14 = 77;
pub const SCENE_SOUKO: C2RustUnnamed_14 = 76;
pub const SCENE_BOWLING: C2RustUnnamed_14 = 75;
pub const SCENE_NAKANIWA: C2RustUnnamed_14 = 74;
pub const SCENE_TURIBORI: C2RustUnnamed_14 = 73;
pub const SCENE_HAKASITARELAY: C2RustUnnamed_14 = 72;
pub const SCENE_HIRAL_DEMO: C2RustUnnamed_14 = 71;
pub const SCENE_HAIRAL_NIWA_N: C2RustUnnamed_14 = 70;
pub const SCENE_HAIRAL_NIWA: C2RustUnnamed_14 = 69;
pub const SCENE_KENJYANOMA: C2RustUnnamed_14 = 68;
pub const SCENE_TOKINOMA: C2RustUnnamed_14 = 67;
pub const SCENE_SYATEKIJYOU: C2RustUnnamed_14 = 66;
pub const SCENE_HAKAANA_OUKE: C2RustUnnamed_14 = 65;
pub const SCENE_HAKAANA2: C2RustUnnamed_14 = 64;
pub const SCENE_HAKAANA: C2RustUnnamed_14 = 63;
pub const SCENE_KAKUSIANA: C2RustUnnamed_14 = 62;
pub const SCENE_YOUSEI_IZUMI_YOKO: C2RustUnnamed_14 = 61;
pub const SCENE_YOUSEI_IZUMI_TATE: C2RustUnnamed_14 = 60;
pub const SCENE_DAIYOUSEI_IZUMI: C2RustUnnamed_14 = 59;
pub const SCENE_HUT: C2RustUnnamed_14 = 58;
pub const SCENE_TENT: C2RustUnnamed_14 = 57;
pub const SCENE_HYLIA_LABO: C2RustUnnamed_14 = 56;
pub const SCENE_LABO: C2RustUnnamed_14 = 55;
pub const SCENE_MALON_STABLE: C2RustUnnamed_14 = 54;
pub const SCENE_IMPA: C2RustUnnamed_14 = 53;
pub const SCENE_LINK_HOME: C2RustUnnamed_14 = 52;
pub const SCENE_FACE_SHOP: C2RustUnnamed_14 = 51;
pub const SCENE_NIGHT_SHOP: C2RustUnnamed_14 = 50;
pub const SCENE_ALLEY_SHOP: C2RustUnnamed_14 = 49;
pub const SCENE_DRAG: C2RustUnnamed_14 = 48;
pub const SCENE_ZOORA: C2RustUnnamed_14 = 47;
pub const SCENE_GOLON: C2RustUnnamed_14 = 46;
pub const SCENE_KOKIRI_SHOP: C2RustUnnamed_14 = 45;
pub const SCENE_SHOP1: C2RustUnnamed_14 = 44;
pub const SCENE_KAKARIKO3: C2RustUnnamed_14 = 43;
pub const SCENE_KAKARIKO: C2RustUnnamed_14 = 42;
pub const SCENE_KOKIRI_HOME5: C2RustUnnamed_14 = 41;
pub const SCENE_KOKIRI_HOME4: C2RustUnnamed_14 = 40;
pub const SCENE_KOKIRI_HOME3: C2RustUnnamed_14 = 39;
pub const SCENE_KOKIRI_HOME: C2RustUnnamed_14 = 38;
pub const SCENE_SHRINE_R: C2RustUnnamed_14 = 37;
pub const SCENE_SHRINE_N: C2RustUnnamed_14 = 36;
pub const SCENE_SHRINE: C2RustUnnamed_14 = 35;
pub const SCENE_MARKET_RUINS: C2RustUnnamed_14 = 34;
pub const SCENE_MARKET_NIGHT: C2RustUnnamed_14 = 33;
pub const SCENE_MARKET_DAY: C2RustUnnamed_14 = 32;
pub const SCENE_MARKET_ALLEY_N: C2RustUnnamed_14 = 31;
pub const SCENE_MARKET_ALLEY: C2RustUnnamed_14 = 30;
pub const SCENE_ENRUI: C2RustUnnamed_14 = 29;
pub const SCENE_ENTRA_N: C2RustUnnamed_14 = 28;
pub const SCENE_ENTRA: C2RustUnnamed_14 = 27;
pub const SCENE_GANON_FINAL: C2RustUnnamed_14 = 26;
pub const SCENE_GANON_BOSS: C2RustUnnamed_14 = 25;
pub const SCENE_HAKADAN_BS: C2RustUnnamed_14 = 24;
pub const SCENE_JYASINBOSS: C2RustUnnamed_14 = 23;
pub const SCENE_MIZUSIN_BS: C2RustUnnamed_14 = 22;
pub const SCENE_FIRE_BS: C2RustUnnamed_14 = 21;
pub const SCENE_MORIBOSSROOM: C2RustUnnamed_14 = 20;
pub const SCENE_BDAN_BOSS: C2RustUnnamed_14 = 19;
pub const SCENE_DDAN_BOSS: C2RustUnnamed_14 = 18;
pub const SCENE_YDAN_BOSS: C2RustUnnamed_14 = 17;
pub const SCENE_TAKARAYA: C2RustUnnamed_14 = 16;
pub const SCENE_GANONTIKA_SONOGO: C2RustUnnamed_14 = 15;
pub const SCENE_GANON_SONOGO: C2RustUnnamed_14 = 14;
pub const SCENE_GANONTIKA: C2RustUnnamed_14 = 13;
pub const SCENE_GERUDOWAY: C2RustUnnamed_14 = 12;
pub const SCENE_MEN: C2RustUnnamed_14 = 11;
pub const SCENE_GANON: C2RustUnnamed_14 = 10;
pub const SCENE_ICE_DOUKUTO: C2RustUnnamed_14 = 9;
pub const SCENE_HAKADANCH: C2RustUnnamed_14 = 8;
pub const SCENE_HAKADAN: C2RustUnnamed_14 = 7;
pub const SCENE_JYASINZOU: C2RustUnnamed_14 = 6;
pub const SCENE_MIZUSIN: C2RustUnnamed_14 = 5;
pub const SCENE_HIDAN: C2RustUnnamed_14 = 4;
pub const SCENE_BMORI1: C2RustUnnamed_14 = 3;
pub const SCENE_BDAN: C2RustUnnamed_14 = 2;
pub const SCENE_DDAN: C2RustUnnamed_14 = 1;
pub const SCENE_YDAN: C2RustUnnamed_14 = 0;
pub type C2RustUnnamed_15 = libc::c_uint;
pub const EQUIP_BOOTS: C2RustUnnamed_15 = 3;
pub const EQUIP_TUNIC: C2RustUnnamed_15 = 2;
pub const EQUIP_SHIELD: C2RustUnnamed_15 = 1;
pub const EQUIP_SWORD: C2RustUnnamed_15 = 0;
pub type C2RustUnnamed_16 = libc::c_uint;
pub const QUEST_HEART_PIECE: C2RustUnnamed_16 = 24;
pub const QUEST_SKULL_TOKEN: C2RustUnnamed_16 = 23;
pub const QUEST_GERUDO_CARD: C2RustUnnamed_16 = 22;
pub const QUEST_STONE_OF_AGONY: C2RustUnnamed_16 = 21;
pub const QUEST_ZORA_SAPPHIRE: C2RustUnnamed_16 = 20;
pub const QUEST_GORON_RUBY: C2RustUnnamed_16 = 19;
pub const QUEST_KOKIRI_EMERALD: C2RustUnnamed_16 = 18;
pub const QUEST_SONG_STORMS: C2RustUnnamed_16 = 17;
pub const QUEST_SONG_TIME: C2RustUnnamed_16 = 16;
pub const QUEST_SONG_SUN: C2RustUnnamed_16 = 15;
pub const QUEST_SONG_SARIA: C2RustUnnamed_16 = 14;
pub const QUEST_SONG_EPONA: C2RustUnnamed_16 = 13;
pub const QUEST_SONG_LULLABY: C2RustUnnamed_16 = 12;
pub const QUEST_SONG_PRELUDE: C2RustUnnamed_16 = 11;
pub const QUEST_SONG_NOCTURNE: C2RustUnnamed_16 = 10;
pub const QUEST_SONG_REQUIEM: C2RustUnnamed_16 = 9;
pub const QUEST_SONG_SERENADE: C2RustUnnamed_16 = 8;
pub const QUEST_SONG_BOLERO: C2RustUnnamed_16 = 7;
pub const QUEST_SONG_MINUET: C2RustUnnamed_16 = 6;
pub const QUEST_MEDALLION_LIGHT: C2RustUnnamed_16 = 5;
pub const QUEST_MEDALLION_SHADOW: C2RustUnnamed_16 = 4;
pub const QUEST_MEDALLION_SPIRIT: C2RustUnnamed_16 = 3;
pub const QUEST_MEDALLION_WATER: C2RustUnnamed_16 = 2;
pub const QUEST_MEDALLION_FIRE: C2RustUnnamed_16 = 1;
pub const QUEST_MEDALLION_FOREST: C2RustUnnamed_16 = 0;
pub type C2RustUnnamed_17 = libc::c_uint;
pub const SLOT_NONE: C2RustUnnamed_17 = 255;
pub const SLOT_TRADE_CHILD: C2RustUnnamed_17 = 23;
pub const SLOT_TRADE_ADULT: C2RustUnnamed_17 = 22;
pub const SLOT_BOTTLE_4: C2RustUnnamed_17 = 21;
pub const SLOT_BOTTLE_3: C2RustUnnamed_17 = 20;
pub const SLOT_BOTTLE_2: C2RustUnnamed_17 = 19;
pub const SLOT_BOTTLE_1: C2RustUnnamed_17 = 18;
pub const SLOT_NAYRUS_LOVE: C2RustUnnamed_17 = 17;
pub const SLOT_ARROW_LIGHT: C2RustUnnamed_17 = 16;
pub const SLOT_HAMMER: C2RustUnnamed_17 = 15;
pub const SLOT_BEAN: C2RustUnnamed_17 = 14;
pub const SLOT_LENS: C2RustUnnamed_17 = 13;
pub const SLOT_BOOMERANG: C2RustUnnamed_17 = 12;
pub const SLOT_FARORES_WIND: C2RustUnnamed_17 = 11;
pub const SLOT_ARROW_ICE: C2RustUnnamed_17 = 10;
pub const SLOT_HOOKSHOT: C2RustUnnamed_17 = 9;
pub const SLOT_BOMBCHU: C2RustUnnamed_17 = 8;
pub const SLOT_OCARINA: C2RustUnnamed_17 = 7;
pub const SLOT_SLINGSHOT: C2RustUnnamed_17 = 6;
pub const SLOT_DINS_FIRE: C2RustUnnamed_17 = 5;
pub const SLOT_ARROW_FIRE: C2RustUnnamed_17 = 4;
pub const SLOT_BOW: C2RustUnnamed_17 = 3;
pub const SLOT_BOMB: C2RustUnnamed_17 = 2;
pub const SLOT_NUT: C2RustUnnamed_17 = 1;
pub const SLOT_STICK: C2RustUnnamed_17 = 0;
pub type C2RustUnnamed_18 = libc::c_uint;
pub const ITEM_NONE: C2RustUnnamed_18 = 255;
pub const ITEM_NONE_FE: C2RustUnnamed_18 = 254;
pub const ITEM_LAST_USED: C2RustUnnamed_18 = 252;
pub const ITEM_NUT_UPGRADE_40: C2RustUnnamed_18 = 155;
pub const ITEM_NUT_UPGRADE_30: C2RustUnnamed_18 = 154;
pub const ITEM_STICK_UPGRADE_30: C2RustUnnamed_18 = 153;
pub const ITEM_STICK_UPGRADE_20: C2RustUnnamed_18 = 152;
pub const ITEM_BOMBCHUS_20: C2RustUnnamed_18 = 151;
pub const ITEM_BOMBCHUS_5: C2RustUnnamed_18 = 150;
pub const ITEM_SEEDS_30: C2RustUnnamed_18 = 149;
pub const ITEM_ARROWS_LARGE: C2RustUnnamed_18 = 148;
pub const ITEM_ARROWS_MEDIUM: C2RustUnnamed_18 = 147;
pub const ITEM_ARROWS_SMALL: C2RustUnnamed_18 = 146;
pub const ITEM_BOMBS_30: C2RustUnnamed_18 = 145;
pub const ITEM_BOMBS_20: C2RustUnnamed_18 = 144;
pub const ITEM_BOMBS_10: C2RustUnnamed_18 = 143;
pub const ITEM_BOMBS_5: C2RustUnnamed_18 = 142;
pub const ITEM_NUTS_10: C2RustUnnamed_18 = 141;
pub const ITEM_NUTS_5: C2RustUnnamed_18 = 140;
pub const ITEM_STICKS_10: C2RustUnnamed_18 = 139;
pub const ITEM_STICKS_5: C2RustUnnamed_18 = 138;
pub const ITEM_INVALID_8: C2RustUnnamed_18 = 137;
pub const ITEM_RUPEE_GOLD: C2RustUnnamed_18 = 136;
pub const ITEM_RUPEE_PURPLE: C2RustUnnamed_18 = 135;
pub const ITEM_RUPEE_RED: C2RustUnnamed_18 = 134;
pub const ITEM_RUPEE_BLUE: C2RustUnnamed_18 = 133;
pub const ITEM_RUPEE_GREEN: C2RustUnnamed_18 = 132;
pub const ITEM_HEART: C2RustUnnamed_18 = 131;
pub const ITEM_MILK: C2RustUnnamed_18 = 130;
pub const ITEM_INVALID_7: C2RustUnnamed_18 = 129;
pub const ITEM_INVALID_6: C2RustUnnamed_18 = 128;
pub const ITEM_INVALID_5: C2RustUnnamed_18 = 127;
pub const ITEM_INVALID_4: C2RustUnnamed_18 = 126;
pub const ITEM_INVALID_3: C2RustUnnamed_18 = 125;
pub const ITEM_INVALID_2: C2RustUnnamed_18 = 124;
pub const ITEM_INVALID_1: C2RustUnnamed_18 = 123;
pub const ITEM_HEART_PIECE_2: C2RustUnnamed_18 = 122;
pub const ITEM_MAGIC_LARGE: C2RustUnnamed_18 = 121;
pub const ITEM_MAGIC_SMALL: C2RustUnnamed_18 = 120;
pub const ITEM_KEY_SMALL: C2RustUnnamed_18 = 119;
pub const ITEM_DUNGEON_MAP: C2RustUnnamed_18 = 118;
pub const ITEM_COMPASS: C2RustUnnamed_18 = 117;
pub const ITEM_KEY_BOSS: C2RustUnnamed_18 = 116;
pub const ITEM_HEART_PIECE: C2RustUnnamed_18 = 115;
pub const ITEM_HEART_CONTAINER: C2RustUnnamed_18 = 114;
pub const ITEM_SKULL_TOKEN: C2RustUnnamed_18 = 113;
pub const ITEM_GERUDO_CARD: C2RustUnnamed_18 = 112;
pub const ITEM_STONE_OF_AGONY: C2RustUnnamed_18 = 111;
pub const ITEM_ZORA_SAPPHIRE: C2RustUnnamed_18 = 110;
pub const ITEM_GORON_RUBY: C2RustUnnamed_18 = 109;
pub const ITEM_KOKIRI_EMERALD: C2RustUnnamed_18 = 108;
pub const ITEM_MEDALLION_LIGHT: C2RustUnnamed_18 = 107;
pub const ITEM_MEDALLION_SHADOW: C2RustUnnamed_18 = 106;
pub const ITEM_MEDALLION_SPIRIT: C2RustUnnamed_18 = 105;
pub const ITEM_MEDALLION_WATER: C2RustUnnamed_18 = 104;
pub const ITEM_MEDALLION_FIRE: C2RustUnnamed_18 = 103;
pub const ITEM_MEDALLION_FOREST: C2RustUnnamed_18 = 102;
pub const ITEM_SONG_STORMS: C2RustUnnamed_18 = 101;
pub const ITEM_SONG_TIME: C2RustUnnamed_18 = 100;
pub const ITEM_SONG_SUN: C2RustUnnamed_18 = 99;
pub const ITEM_SONG_SARIA: C2RustUnnamed_18 = 98;
pub const ITEM_SONG_EPONA: C2RustUnnamed_18 = 97;
pub const ITEM_SONG_LULLABY: C2RustUnnamed_18 = 96;
pub const ITEM_SONG_PRELUDE: C2RustUnnamed_18 = 95;
pub const ITEM_SONG_NOCTURNE: C2RustUnnamed_18 = 94;
pub const ITEM_SONG_REQUIEM: C2RustUnnamed_18 = 93;
pub const ITEM_SONG_SERENADE: C2RustUnnamed_18 = 92;
pub const ITEM_SONG_BOLERO: C2RustUnnamed_18 = 91;
pub const ITEM_SONG_MINUET: C2RustUnnamed_18 = 90;
pub const ITEM_FISHING_POLE: C2RustUnnamed_18 = 89;
pub const ITEM_SEEDS: C2RustUnnamed_18 = 88;
pub const ITEM_WALLET_GIANT: C2RustUnnamed_18 = 87;
pub const ITEM_WALLET_ADULT: C2RustUnnamed_18 = 86;
pub const ITEM_SWORD_KNIFE: C2RustUnnamed_18 = 85;
pub const ITEM_SCALE_GOLDEN: C2RustUnnamed_18 = 84;
pub const ITEM_SCALE_SILVER: C2RustUnnamed_18 = 83;
pub const ITEM_GAUNTLETS_GOLD: C2RustUnnamed_18 = 82;
pub const ITEM_GAUNTLETS_SILVER: C2RustUnnamed_18 = 81;
pub const ITEM_BRACELET: C2RustUnnamed_18 = 80;
pub const ITEM_BOMB_BAG_40: C2RustUnnamed_18 = 79;
pub const ITEM_BOMB_BAG_30: C2RustUnnamed_18 = 78;
pub const ITEM_BOMB_BAG_20: C2RustUnnamed_18 = 77;
pub const ITEM_QUIVER_50: C2RustUnnamed_18 = 76;
pub const ITEM_QUIVER_40: C2RustUnnamed_18 = 75;
pub const ITEM_QUIVER_30: C2RustUnnamed_18 = 74;
pub const ITEM_BULLET_BAG_50: C2RustUnnamed_18 = 73;
pub const ITEM_BULLET_BAG_40: C2RustUnnamed_18 = 72;
pub const ITEM_BULLET_BAG_30: C2RustUnnamed_18 = 71;
pub const ITEM_BOOTS_HOVER: C2RustUnnamed_18 = 70;
pub const ITEM_BOOTS_IRON: C2RustUnnamed_18 = 69;
pub const ITEM_BOOTS_KOKIRI: C2RustUnnamed_18 = 68;
pub const ITEM_TUNIC_ZORA: C2RustUnnamed_18 = 67;
pub const ITEM_TUNIC_GORON: C2RustUnnamed_18 = 66;
pub const ITEM_TUNIC_KOKIRI: C2RustUnnamed_18 = 65;
pub const ITEM_SHIELD_MIRROR: C2RustUnnamed_18 = 64;
pub const ITEM_SHIELD_HYLIAN: C2RustUnnamed_18 = 63;
pub const ITEM_SHIELD_DEKU: C2RustUnnamed_18 = 62;
pub const ITEM_SWORD_BGS: C2RustUnnamed_18 = 61;
pub const ITEM_SWORD_MASTER: C2RustUnnamed_18 = 60;
pub const ITEM_SWORD_KOKIRI: C2RustUnnamed_18 = 59;
pub const ITEM_BOW_ARROW_LIGHT: C2RustUnnamed_18 = 58;
pub const ITEM_BOW_ARROW_ICE: C2RustUnnamed_18 = 57;
pub const ITEM_BOW_ARROW_FIRE: C2RustUnnamed_18 = 56;
pub const ITEM_CLAIM_CHECK: C2RustUnnamed_18 = 55;
pub const ITEM_EYEDROPS: C2RustUnnamed_18 = 54;
pub const ITEM_FROG: C2RustUnnamed_18 = 53;
pub const ITEM_PRESCRIPTION: C2RustUnnamed_18 = 52;
pub const ITEM_SWORD_BROKEN: C2RustUnnamed_18 = 51;
pub const ITEM_SAW: C2RustUnnamed_18 = 50;
pub const ITEM_ODD_POTION: C2RustUnnamed_18 = 49;
pub const ITEM_ODD_MUSHROOM: C2RustUnnamed_18 = 48;
pub const ITEM_COJIRO: C2RustUnnamed_18 = 47;
pub const ITEM_POCKET_CUCCO: C2RustUnnamed_18 = 46;
pub const ITEM_POCKET_EGG: C2RustUnnamed_18 = 45;
pub const ITEM_SOLD_OUT: C2RustUnnamed_18 = 44;
pub const ITEM_MASK_TRUTH: C2RustUnnamed_18 = 43;
pub const ITEM_MASK_GERUDO: C2RustUnnamed_18 = 42;
pub const ITEM_MASK_ZORA: C2RustUnnamed_18 = 41;
pub const ITEM_MASK_GORON: C2RustUnnamed_18 = 40;
pub const ITEM_MASK_BUNNY: C2RustUnnamed_18 = 39;
pub const ITEM_MASK_SPOOKY: C2RustUnnamed_18 = 38;
pub const ITEM_MASK_SKULL: C2RustUnnamed_18 = 37;
pub const ITEM_MASK_KEATON: C2RustUnnamed_18 = 36;
pub const ITEM_LETTER_ZELDA: C2RustUnnamed_18 = 35;
pub const ITEM_CHICKEN: C2RustUnnamed_18 = 34;
pub const ITEM_WEIRD_EGG: C2RustUnnamed_18 = 33;
pub const ITEM_POE: C2RustUnnamed_18 = 32;
pub const ITEM_MILK_HALF: C2RustUnnamed_18 = 31;
pub const ITEM_BIG_POE: C2RustUnnamed_18 = 30;
pub const ITEM_BUG: C2RustUnnamed_18 = 29;
pub const ITEM_BLUE_FIRE: C2RustUnnamed_18 = 28;
pub const ITEM_LETTER_RUTO: C2RustUnnamed_18 = 27;
pub const ITEM_MILK_BOTTLE: C2RustUnnamed_18 = 26;
pub const ITEM_FISH: C2RustUnnamed_18 = 25;
pub const ITEM_FAIRY: C2RustUnnamed_18 = 24;
pub const ITEM_POTION_BLUE: C2RustUnnamed_18 = 23;
pub const ITEM_POTION_GREEN: C2RustUnnamed_18 = 22;
pub const ITEM_POTION_RED: C2RustUnnamed_18 = 21;
pub const ITEM_BOTTLE: C2RustUnnamed_18 = 20;
pub const ITEM_NAYRUS_LOVE: C2RustUnnamed_18 = 19;
pub const ITEM_ARROW_LIGHT: C2RustUnnamed_18 = 18;
pub const ITEM_HAMMER: C2RustUnnamed_18 = 17;
pub const ITEM_BEAN: C2RustUnnamed_18 = 16;
pub const ITEM_LENS: C2RustUnnamed_18 = 15;
pub const ITEM_BOOMERANG: C2RustUnnamed_18 = 14;
pub const ITEM_FARORES_WIND: C2RustUnnamed_18 = 13;
pub const ITEM_ARROW_ICE: C2RustUnnamed_18 = 12;
pub const ITEM_LONGSHOT: C2RustUnnamed_18 = 11;
pub const ITEM_HOOKSHOT: C2RustUnnamed_18 = 10;
pub const ITEM_BOMBCHU: C2RustUnnamed_18 = 9;
pub const ITEM_OCARINA_TIME: C2RustUnnamed_18 = 8;
pub const ITEM_OCARINA_FAIRY: C2RustUnnamed_18 = 7;
pub const ITEM_SLINGSHOT: C2RustUnnamed_18 = 6;
pub const ITEM_DINS_FIRE: C2RustUnnamed_18 = 5;
pub const ITEM_ARROW_FIRE: C2RustUnnamed_18 = 4;
pub const ITEM_BOW: C2RustUnnamed_18 = 3;
pub const ITEM_BOMB: C2RustUnnamed_18 = 2;
pub const ITEM_NUT: C2RustUnnamed_18 = 1;
pub const ITEM_STICK: C2RustUnnamed_18 = 0;
pub type C2RustUnnamed_19 = libc::c_uint;
pub const LANGUAGE_MAX: C2RustUnnamed_19 = 3;
pub const LANGUAGE_FRA: C2RustUnnamed_19 = 2;
pub const LANGUAGE_GER: C2RustUnnamed_19 = 1;
pub const LANGUAGE_ENG: C2RustUnnamed_19 = 0;
pub type C2RustUnnamed_20 = libc::c_uint;
pub const SRAM_HEADER_MAGIC: C2RustUnnamed_20 = 3;
pub const SRAM_HEADER_LANGUAGE: C2RustUnnamed_20 = 2;
pub const SRAM_HEADER_ZTARGET: C2RustUnnamed_20 = 1;
pub const SRAM_HEADER_SOUND: C2RustUnnamed_20 = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct FileChooseContext {
    pub state: GameState,
    pub windowVtx: *mut Vtx,
    pub staticSegment: *mut u8_0,
    pub parameterSegment: *mut u8_0,
    pub unk_B0: [libc::c_char; 8],
    pub view: View,
    pub sramCtx: SramContext,
    pub unk_1E4: [libc::c_char; 4],
    pub skyboxCtx: SkyboxContext,
    pub msgCtx: MessageContext,
    pub font: Font,
    pub envCtx: EnvironmentContext,
    pub unk_1C9E4: [libc::c_char; 4],
    pub windowContentVtx: *mut Vtx,
    pub keyboardVtx: *mut Vtx,
    pub nameEntryVtx: *mut Vtx,
    pub n64ddFlag: u8_0,
    pub deaths: [u16_0; 3],
    pub fileNames: [[u8_0; 8]; 3],
    pub healthCapacities: [u16_0; 3],
    pub questItems: [u32_0; 3],
    pub n64ddFlags: [s16; 3],
    pub defense: [s8; 3],
    pub health: [u16_0; 3],
    pub buttonIndex: s16,
    pub confirmButtonIndex: s16,
    pub menuMode: s16,
    pub configMode: s16,
    pub prevConfigMode: s16,
    pub nextConfigMode: s16,
    pub selectMode: s16,
    pub selectedFileIndex: s16,
    pub unk_1CA48: [libc::c_char; 2],
    pub fileNamesY: [s16; 3],
    pub actionTimer: s16,
    pub buttonYOffsets: [s16; 6],
    pub copyDestFileIndex: s16,
    pub warningLabel: s16,
    pub warningButtonIndex: s16,
    pub titleLabel: s16,
    pub nextTitleLabel: s16,
    pub windowColor: [s16; 3],
    pub titleAlpha: [s16; 2],
    pub windowAlpha: s16,
    pub fileButtonAlpha: [s16; 3],
    pub nameBoxAlpha: [s16; 3],
    pub nameAlpha: [s16; 3],
    pub connectorAlpha: [s16; 3],
    pub fileInfoAlpha: [s16; 3],
    pub actionButtonAlpha: [s16; 2],
    pub confirmButtonAlpha: [s16; 2],
    pub optionButtonAlpha: s16,
    pub nameEntryBoxAlpha: s16,
    pub controlsAlpha: s16,
    pub emptyFileTextAlpha: s16,
    pub highlightColor: [s16; 4],
    pub highlightPulseDir: s16,
    pub unk_1CAAC: s16,
    pub confirmButtonTexIndices: [s16; 2],
    pub inputTimerX: s16,
    pub inputTimerY: s16,
    pub stickXDir: s16,
    pub stickYDir: s16,
    pub stickRelX: s16,
    pub stickRelY: s16,
    pub nameEntryBoxPosX: s16,
    pub windowPosX: s16,
    pub windowRot: f32_0,
    pub kbdButton: s16,
    pub charPage: s16,
    pub charBgAlpha: s16,
    pub charIndex: s16,
    pub kbdX: s16,
    pub kbdY: s16,
    pub newFileNameCharCount: s16,
    pub unk_1CAD6: [s16; 5],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SavePlayerData {
    pub newf: [libc::c_char; 6],
    pub deaths: s16,
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
    pub unk_1F: u8_0,
    pub doubleMagic: u8_0,
    pub doubleDefense: u8_0,
    pub bgsFlag: u8_0,
    pub ocarinaGameRoundNum: u8_0,
    pub childEquips: ItemEquips,
    pub adultEquips: ItemEquips,
    pub unk_38: u32_0,
    pub unk_3C: [libc::c_char; 14],
    pub savedSceneNum: s16,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SaveInfo {
    pub playerData: SavePlayerData,
    pub equips: ItemEquips,
    pub inventory: Inventory,
    pub sceneFlags: [SavedSceneFlags; 124],
    pub fw: FaroresWindData,
    pub unk_E70: [libc::c_char; 16],
    pub gsFlags: [s32; 6],
    pub unk_E98: [libc::c_char; 16],
    pub horseRaceRecord: s32,
    pub unk_EAC: [libc::c_char; 12],
    pub eventChkInf: [u16_0; 14],
    pub itemGetInf: [u16_0; 4],
    pub infTable: [u16_0; 30],
    pub unk_F18: [libc::c_char; 4],
    pub worldMapAreaData: u32_0,
    pub unk_F20: [libc::c_char; 4],
    pub scarecrowCustomSongSet: u8_0,
    pub scarecrowCustomSong: [u8_0; 864],
    pub unk_1285: [libc::c_char; 36],
    pub scarecrowSpawnSongSet: u8_0,
    pub scarecrowSpawnSong: [u8_0; 128],
    pub unk_132A: [libc::c_char; 2],
    pub horseData: HorseData,
    pub checksum: u16_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Save {
    pub entranceIndex: s32,
    pub linkAge: s32,
    pub cutsceneIndex: s32,
    pub dayTime: u16_0,
    pub nightFlag: s32,
    pub totalDays: s32,
    pub unk_18: s32,
    pub info: SaveInfo,
}
// Initialized in run_static_initializers
#[no_mangle]
pub static mut gSramSlotOffsets: [u16_0; 6] = [0; 6];
static mut sZeldaMagic: [libc::c_char; 12] =
    ['\u{0}' as i32 as libc::c_char, '\u{0}' as i32 as libc::c_char,
     '\u{0}' as i32 as libc::c_char, -104i32 as libc::c_char,
     '\t' as i32 as libc::c_char, '\u{10}' as i32 as libc::c_char,
     '!' as i32 as libc::c_char, 'Z' as i32 as libc::c_char,
     'E' as i32 as libc::c_char, 'L' as i32 as libc::c_char,
     'D' as i32 as libc::c_char, 'A' as i32 as libc::c_char];
static mut sNewSavePlayerData: SavePlayerData =
    {
        let mut init =
            SavePlayerData{newf:
                               ['\u{0}' as i32 as libc::c_char,
                                '\u{0}' as i32 as libc::c_char,
                                '\u{0}' as i32 as libc::c_char,
                                '\u{0}' as i32 as libc::c_char,
                                '\u{0}' as i32 as libc::c_char,
                                '\u{0}' as i32 as libc::c_char],
                           deaths: 0 as libc::c_int as s16,
                           playerName:
                               [0x3e as libc::c_int as libc::c_char,
                                0x3e as libc::c_int as libc::c_char,
                                0x3e as libc::c_int as libc::c_char,
                                0x3e as libc::c_int as libc::c_char,
                                0x3e as libc::c_int as libc::c_char,
                                0x3e as libc::c_int as libc::c_char,
                                0x3e as libc::c_int as libc::c_char,
                                0x3e as libc::c_int as libc::c_char],
                           n64ddFlag: 0 as libc::c_int as s16,
                           healthCapacity: 0x30 as libc::c_int as s16,
                           health: 0x30 as libc::c_int as s16,
                           magicLevel: 0 as libc::c_int as s8,
                           magic: 0x30 as libc::c_int as s8,
                           rupees: 0 as libc::c_int as s16,
                           swordHealth: 0 as libc::c_int as u16_0,
                           naviTimer: 0 as libc::c_int as u16_0,
                           magicAcquired: 0 as libc::c_int as u8_0,
                           unk_1F: 0 as libc::c_int as u8_0,
                           doubleMagic: 0 as libc::c_int as u8_0,
                           doubleDefense: 0 as libc::c_int as u8_0,
                           bgsFlag: 0 as libc::c_int as u8_0,
                           ocarinaGameRoundNum: 0 as libc::c_int as u8_0,
                           childEquips:
                               {
                                   let mut init =
                                       ItemEquips{buttonItems:
                                                      [ITEM_NONE as
                                                           libc::c_int as
                                                           u8_0,
                                                       ITEM_NONE as
                                                           libc::c_int as
                                                           u8_0,
                                                       ITEM_NONE as
                                                           libc::c_int as
                                                           u8_0,
                                                       ITEM_NONE as
                                                           libc::c_int as
                                                           u8_0],
                                                  cButtonSlots:
                                                      [SLOT_NONE as
                                                           libc::c_int as
                                                           u8_0,
                                                       SLOT_NONE as
                                                           libc::c_int as
                                                           u8_0,
                                                       SLOT_NONE as
                                                           libc::c_int as
                                                           u8_0],
                                                  equipment:
                                                      0 as libc::c_int as
                                                          u16_0,};
                                   init
                               },
                           adultEquips:
                               {
                                   let mut init =
                                       ItemEquips{buttonItems:
                                                      [ITEM_NONE as
                                                           libc::c_int as
                                                           u8_0,
                                                       ITEM_NONE as
                                                           libc::c_int as
                                                           u8_0,
                                                       ITEM_NONE as
                                                           libc::c_int as
                                                           u8_0,
                                                       ITEM_NONE as
                                                           libc::c_int as
                                                           u8_0],
                                                  cButtonSlots:
                                                      [SLOT_NONE as
                                                           libc::c_int as
                                                           u8_0,
                                                       SLOT_NONE as
                                                           libc::c_int as
                                                           u8_0,
                                                       SLOT_NONE as
                                                           libc::c_int as
                                                           u8_0],
                                                  equipment:
                                                      0 as libc::c_int as
                                                          u16_0,};
                                   init
                               },
                           unk_38: 0 as libc::c_int as u32_0,
                           unk_3C:
                               [0 as libc::c_int as libc::c_char,
                                0 as libc::c_int as libc::c_char,
                                0 as libc::c_int as libc::c_char,
                                0 as libc::c_int as libc::c_char,
                                0 as libc::c_int as libc::c_char,
                                0 as libc::c_int as libc::c_char,
                                0 as libc::c_int as libc::c_char,
                                0 as libc::c_int as libc::c_char,
                                0 as libc::c_int as libc::c_char,
                                0 as libc::c_int as libc::c_char,
                                0 as libc::c_int as libc::c_char,
                                0 as libc::c_int as libc::c_char,
                                0 as libc::c_int as libc::c_char,
                                0 as libc::c_int as libc::c_char],
                           savedSceneNum: 0x34 as libc::c_int as s16,};
        init
    };
static mut sNewSaveEquips: ItemEquips =
    {
        let mut init =
            ItemEquips{buttonItems:
                           [ITEM_NONE as libc::c_int as u8_0,
                            ITEM_NONE as libc::c_int as u8_0,
                            ITEM_NONE as libc::c_int as u8_0,
                            ITEM_NONE as libc::c_int as u8_0],
                       cButtonSlots:
                           [SLOT_NONE as libc::c_int as u8_0,
                            SLOT_NONE as libc::c_int as u8_0,
                            SLOT_NONE as libc::c_int as u8_0],
                       equipment: 0x1100 as libc::c_int as u16_0,};
        init
    };
static mut sNewSaveInventory: Inventory =
    {
        let mut init =
            Inventory{items:
                          [0xff as libc::c_int as u8_0,
                           0xff as libc::c_int as u8_0,
                           0xff as libc::c_int as u8_0,
                           0xff as libc::c_int as u8_0,
                           0xff as libc::c_int as u8_0,
                           0xff as libc::c_int as u8_0,
                           0xff as libc::c_int as u8_0,
                           0xff as libc::c_int as u8_0,
                           0xff as libc::c_int as u8_0,
                           0xff as libc::c_int as u8_0,
                           0xff as libc::c_int as u8_0,
                           0xff as libc::c_int as u8_0,
                           0xff as libc::c_int as u8_0,
                           0xff as libc::c_int as u8_0,
                           0xff as libc::c_int as u8_0,
                           0xff as libc::c_int as u8_0,
                           0xff as libc::c_int as u8_0,
                           0xff as libc::c_int as u8_0,
                           0xff as libc::c_int as u8_0,
                           0xff as libc::c_int as u8_0,
                           0xff as libc::c_int as u8_0,
                           0xff as libc::c_int as u8_0,
                           0xff as libc::c_int as u8_0,
                           0xff as libc::c_int as u8_0],
                      ammo:
                          [0 as libc::c_int as s8, 0 as libc::c_int as s8,
                           0 as libc::c_int as s8, 0 as libc::c_int as s8,
                           0 as libc::c_int as s8, 0 as libc::c_int as s8,
                           0 as libc::c_int as s8, 0 as libc::c_int as s8,
                           0 as libc::c_int as s8, 0 as libc::c_int as s8,
                           0 as libc::c_int as s8, 0 as libc::c_int as s8,
                           0 as libc::c_int as s8, 0 as libc::c_int as s8,
                           0 as libc::c_int as s8, 0 as libc::c_int as s8],
                      equipment: 0x1100 as libc::c_int as u16_0,
                      upgrades: 0 as libc::c_int as u32_0,
                      questItems: 0 as libc::c_int as u32_0,
                      dungeonItems:
                          [0 as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
                           0 as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
                           0 as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
                           0 as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
                           0 as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
                           0 as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
                           0 as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
                           0 as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
                           0 as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
                           0 as libc::c_int as u8_0,
                           0 as libc::c_int as u8_0],
                      dungeonKeys:
                          [0xff as libc::c_int as s8,
                           0xff as libc::c_int as s8,
                           0xff as libc::c_int as s8,
                           0xff as libc::c_int as s8,
                           0xff as libc::c_int as s8,
                           0xff as libc::c_int as s8,
                           0xff as libc::c_int as s8,
                           0xff as libc::c_int as s8,
                           0xff as libc::c_int as s8,
                           0xff as libc::c_int as s8,
                           0xff as libc::c_int as s8,
                           0xff as libc::c_int as s8,
                           0xff as libc::c_int as s8,
                           0xff as libc::c_int as s8,
                           0xff as libc::c_int as s8,
                           0xff as libc::c_int as s8,
                           0xff as libc::c_int as s8,
                           0xff as libc::c_int as s8,
                           0xff as libc::c_int as s8],
                      defenseHearts: 0 as libc::c_int as s8,
                      gsTokens: 0 as libc::c_int as s16,};
        init
    };
static mut sNewSaveChecksum: u16_0 = 0 as libc::c_int as u16_0;
/* *
 *  Initialize new save.
 *  This save has an empty inventory with 3 hearts and single magic.
 */
#[no_mangle]
pub unsafe extern "C" fn Sram_InitNewSave() {
    let mut temp: *mut SaveContext = &mut gSaveContext;
    bzero(&mut gSaveContext.newf as *mut [libc::c_char; 6] as *mut SaveInfo as
              *mut libc::c_void,
          ::std::mem::size_of::<SaveInfo>() as libc::c_ulong);
    gSaveContext.totalDays = 0 as libc::c_int;
    gSaveContext.bgsDayCount = 0 as libc::c_int;
    *(&mut gSaveContext.newf as *mut [libc::c_char; 6] as *mut SavePlayerData)
        = sNewSavePlayerData;
    gSaveContext.equips = sNewSaveEquips;
    gSaveContext.inventory = sNewSaveInventory;
    (*temp).checksum = sNewSaveChecksum;
    gSaveContext.horseData.scene = SCENE_SPOT00 as libc::c_int as s16;
    gSaveContext.horseData.pos.x = -(1840 as libc::c_int) as s16;
    gSaveContext.horseData.pos.y = 72 as libc::c_int as s16;
    gSaveContext.horseData.pos.z = 5497 as libc::c_int as s16;
    gSaveContext.horseData.angle = -(0x6ad9 as libc::c_int) as s16;
    gSaveContext.magicLevel = 0 as libc::c_int as s8;
    gSaveContext.infTable[29 as libc::c_int as usize] =
        1 as libc::c_int as u16_0;
    gSaveContext.sceneFlags[5 as libc::c_int as usize].swch =
        0x40000000 as libc::c_int as u32_0;
}
static mut sDebugSavePlayerData: SavePlayerData =
    {
        let mut init =
            SavePlayerData{newf:
                               ['Z' as i32 as libc::c_char,
                                'E' as i32 as libc::c_char,
                                'L' as i32 as libc::c_char,
                                'D' as i32 as libc::c_char,
                                'A' as i32 as libc::c_char,
                                'Z' as i32 as libc::c_char],
                           deaths: 0 as libc::c_int as s16,
                           playerName:
                               [0x15 as libc::c_int as libc::c_char,
                                0x12 as libc::c_int as libc::c_char,
                                0x17 as libc::c_int as libc::c_char,
                                0x14 as libc::c_int as libc::c_char,
                                0x3e as libc::c_int as libc::c_char,
                                0x3e as libc::c_int as libc::c_char,
                                0x3e as libc::c_int as libc::c_char,
                                0x3e as libc::c_int as libc::c_char],
                           n64ddFlag: 0 as libc::c_int as s16,
                           healthCapacity: 0xe0 as libc::c_int as s16,
                           health: 0xe0 as libc::c_int as s16,
                           magicLevel: 0 as libc::c_int as s8,
                           magic: 0x30 as libc::c_int as s8,
                           rupees: 150 as libc::c_int as s16,
                           swordHealth: 8 as libc::c_int as u16_0,
                           naviTimer: 0 as libc::c_int as u16_0,
                           magicAcquired: 1 as libc::c_int as u8_0,
                           unk_1F: 0 as libc::c_int as u8_0,
                           doubleMagic: 0 as libc::c_int as u8_0,
                           doubleDefense: 0 as libc::c_int as u8_0,
                           bgsFlag: 0 as libc::c_int as u8_0,
                           ocarinaGameRoundNum: 0 as libc::c_int as u8_0,
                           childEquips:
                               {
                                   let mut init =
                                       ItemEquips{buttonItems:
                                                      [ITEM_NONE as
                                                           libc::c_int as
                                                           u8_0,
                                                       ITEM_NONE as
                                                           libc::c_int as
                                                           u8_0,
                                                       ITEM_NONE as
                                                           libc::c_int as
                                                           u8_0,
                                                       ITEM_NONE as
                                                           libc::c_int as
                                                           u8_0],
                                                  cButtonSlots:
                                                      [SLOT_NONE as
                                                           libc::c_int as
                                                           u8_0,
                                                       SLOT_NONE as
                                                           libc::c_int as
                                                           u8_0,
                                                       SLOT_NONE as
                                                           libc::c_int as
                                                           u8_0],
                                                  equipment:
                                                      0 as libc::c_int as
                                                          u16_0,};
                                   init
                               },
                           adultEquips:
                               {
                                   let mut init =
                                       ItemEquips{buttonItems:
                                                      [ITEM_NONE as
                                                           libc::c_int as
                                                           u8_0,
                                                       ITEM_NONE as
                                                           libc::c_int as
                                                           u8_0,
                                                       ITEM_NONE as
                                                           libc::c_int as
                                                           u8_0,
                                                       ITEM_NONE as
                                                           libc::c_int as
                                                           u8_0],
                                                  cButtonSlots:
                                                      [SLOT_NONE as
                                                           libc::c_int as
                                                           u8_0,
                                                       SLOT_NONE as
                                                           libc::c_int as
                                                           u8_0,
                                                       SLOT_NONE as
                                                           libc::c_int as
                                                           u8_0],
                                                  equipment:
                                                      0 as libc::c_int as
                                                          u16_0,};
                                   init
                               },
                           unk_38: 0 as libc::c_int as u32_0,
                           unk_3C:
                               [0 as libc::c_int as libc::c_char,
                                0 as libc::c_int as libc::c_char,
                                0 as libc::c_int as libc::c_char,
                                0 as libc::c_int as libc::c_char,
                                0 as libc::c_int as libc::c_char,
                                0 as libc::c_int as libc::c_char,
                                0 as libc::c_int as libc::c_char,
                                0 as libc::c_int as libc::c_char,
                                0 as libc::c_int as libc::c_char,
                                0 as libc::c_int as libc::c_char,
                                0 as libc::c_int as libc::c_char,
                                0 as libc::c_int as libc::c_char,
                                0 as libc::c_int as libc::c_char,
                                0 as libc::c_int as libc::c_char],
                           savedSceneNum: 0x51 as libc::c_int as s16,};
        init
    };
static mut sDebugSaveEquips: ItemEquips =
    {
        let mut init =
            ItemEquips{buttonItems:
                           [ITEM_SWORD_MASTER as libc::c_int as u8_0,
                            ITEM_BOW as libc::c_int as u8_0,
                            ITEM_BOMB as libc::c_int as u8_0,
                            ITEM_OCARINA_FAIRY as libc::c_int as u8_0],
                       cButtonSlots:
                           [SLOT_BOW as libc::c_int as u8_0,
                            SLOT_BOMB as libc::c_int as u8_0,
                            SLOT_OCARINA as libc::c_int as u8_0],
                       equipment: 0x1122 as libc::c_int as u16_0,};
        init
    };
static mut sDebugSaveInventory: Inventory =
    {
        let mut init =
            Inventory{items:
                          [ITEM_STICK as libc::c_int as u8_0,
                           ITEM_NUT as libc::c_int as u8_0,
                           ITEM_BOMB as libc::c_int as u8_0,
                           ITEM_BOW as libc::c_int as u8_0,
                           ITEM_ARROW_FIRE as libc::c_int as u8_0,
                           ITEM_DINS_FIRE as libc::c_int as u8_0,
                           ITEM_SLINGSHOT as libc::c_int as u8_0,
                           ITEM_OCARINA_FAIRY as libc::c_int as u8_0,
                           ITEM_BOMBCHU as libc::c_int as u8_0,
                           ITEM_HOOKSHOT as libc::c_int as u8_0,
                           ITEM_ARROW_ICE as libc::c_int as u8_0,
                           ITEM_FARORES_WIND as libc::c_int as u8_0,
                           ITEM_BOOMERANG as libc::c_int as u8_0,
                           ITEM_LENS as libc::c_int as u8_0,
                           ITEM_BEAN as libc::c_int as u8_0,
                           ITEM_HAMMER as libc::c_int as u8_0,
                           ITEM_ARROW_LIGHT as libc::c_int as u8_0,
                           ITEM_NAYRUS_LOVE as libc::c_int as u8_0,
                           ITEM_BOTTLE as libc::c_int as u8_0,
                           ITEM_POTION_RED as libc::c_int as u8_0,
                           ITEM_POTION_GREEN as libc::c_int as u8_0,
                           ITEM_POTION_BLUE as libc::c_int as u8_0,
                           ITEM_POCKET_EGG as libc::c_int as u8_0,
                           ITEM_WEIRD_EGG as libc::c_int as u8_0],
                      ammo:
                          [50 as libc::c_int as s8, 50 as libc::c_int as s8,
                           10 as libc::c_int as s8, 30 as libc::c_int as s8,
                           1 as libc::c_int as s8, 1 as libc::c_int as s8,
                           30 as libc::c_int as s8, 1 as libc::c_int as s8,
                           50 as libc::c_int as s8, 1 as libc::c_int as s8,
                           1 as libc::c_int as s8, 1 as libc::c_int as s8,
                           1 as libc::c_int as s8, 1 as libc::c_int as s8,
                           1 as libc::c_int as s8, 1 as libc::c_int as s8],
                      equipment: 0x7777 as libc::c_int as u16_0,
                      upgrades: 0x125249 as libc::c_int as u32_0,
                      questItems: 0x1e3ffff as libc::c_int as u32_0,
                      dungeonItems:
                          [7 as libc::c_int as u8_0, 7 as libc::c_int as u8_0,
                           7 as libc::c_int as u8_0, 7 as libc::c_int as u8_0,
                           7 as libc::c_int as u8_0, 7 as libc::c_int as u8_0,
                           7 as libc::c_int as u8_0, 7 as libc::c_int as u8_0,
                           7 as libc::c_int as u8_0, 7 as libc::c_int as u8_0,
                           0 as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
                           0 as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
                           0 as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
                           0 as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
                           0 as libc::c_int as u8_0,
                           0 as libc::c_int as u8_0],
                      dungeonKeys:
                          [8 as libc::c_int as s8, 8 as libc::c_int as s8,
                           8 as libc::c_int as s8, 8 as libc::c_int as s8,
                           8 as libc::c_int as s8, 8 as libc::c_int as s8,
                           8 as libc::c_int as s8, 8 as libc::c_int as s8,
                           8 as libc::c_int as s8, 8 as libc::c_int as s8,
                           8 as libc::c_int as s8, 8 as libc::c_int as s8,
                           8 as libc::c_int as s8, 8 as libc::c_int as s8,
                           8 as libc::c_int as s8, 8 as libc::c_int as s8,
                           8 as libc::c_int as s8, 8 as libc::c_int as s8,
                           8 as libc::c_int as s8],
                      defenseHearts: 0 as libc::c_int as s8,
                      gsTokens: 0 as libc::c_int as s16,};
        init
    };
static mut sDebugSaveChecksum: u16_0 = 0 as libc::c_int as u16_0;
/* *
 *  Initialize debug save. This is also used on the Title Screen
 *  This save has a mostly full inventory with 10 hearts and single magic.
 *
 *  Some noteable flags that are set:
 *  Showed Mido sword/shield, met Deku Tree, Deku Tree mouth opened,
 *  used blue warp in Gohmas room, Zelda fled castle, light arrow cutscene watched,
 *  and set water level in Water Temple to lowest level.
 */
#[no_mangle]
pub unsafe extern "C" fn Sram_InitDebugSave() {
    let mut temp: *mut SaveContext = &mut gSaveContext;
    bzero(&mut gSaveContext.newf as *mut [libc::c_char; 6] as *mut SaveInfo as
              *mut libc::c_void,
          ::std::mem::size_of::<SaveInfo>() as libc::c_ulong);
    gSaveContext.totalDays = 0 as libc::c_int;
    gSaveContext.bgsDayCount = 0 as libc::c_int;
    *(&mut gSaveContext.newf as *mut [libc::c_char; 6] as *mut SavePlayerData)
        = sDebugSavePlayerData;
    gSaveContext.equips = sDebugSaveEquips;
    gSaveContext.inventory = sDebugSaveInventory;
    (*temp).checksum = sDebugSaveChecksum;
    gSaveContext.horseData.scene = SCENE_SPOT00 as libc::c_int as s16;
    gSaveContext.horseData.pos.x = -(1840 as libc::c_int) as s16;
    gSaveContext.horseData.pos.y = 72 as libc::c_int as s16;
    gSaveContext.horseData.pos.z = 5497 as libc::c_int as s16;
    gSaveContext.horseData.angle = -(0x6ad9 as libc::c_int) as s16;
    gSaveContext.infTable[0 as libc::c_int as usize] =
        (gSaveContext.infTable[0 as libc::c_int as usize] as libc::c_int |
             0x5009 as libc::c_int) as u16_0;
    gSaveContext.eventChkInf[0 as libc::c_int as usize] =
        (gSaveContext.eventChkInf[0 as libc::c_int as usize] as libc::c_int |
             0x123f as libc::c_int) as u16_0;
    gSaveContext.eventChkInf[8 as libc::c_int as usize] =
        (gSaveContext.eventChkInf[8 as libc::c_int as usize] as libc::c_int |
             1 as libc::c_int) as u16_0;
    gSaveContext.eventChkInf[12 as libc::c_int as usize] =
        (gSaveContext.eventChkInf[12 as libc::c_int as usize] as libc::c_int |
             0x10 as libc::c_int) as u16_0;
    if (if !(gSaveContext.linkAge == 0 as libc::c_int) {
            5 as libc::c_int
        } else { 17 as libc::c_int }) == 5 as libc::c_int {
        gSaveContext.equips.buttonItems[0 as libc::c_int as usize] =
            ITEM_SWORD_KOKIRI as libc::c_int as u8_0;
        Inventory_ChangeEquipment(EQUIP_SWORD as libc::c_int as s16,
                                  1 as libc::c_int as u16_0);
        if gSaveContext.fileNum == 0xff as libc::c_int {
            gSaveContext.equips.buttonItems[1 as libc::c_int as usize] =
                ITEM_SLINGSHOT as libc::c_int as u8_0;
            gSaveContext.equips.cButtonSlots[0 as libc::c_int as usize] =
                SLOT_SLINGSHOT as libc::c_int as u8_0;
            Inventory_ChangeEquipment(EQUIP_SHIELD as libc::c_int as s16,
                                      1 as libc::c_int as u16_0);
        }
    }
    gSaveContext.entranceIndex = 0xcd as libc::c_int;
    gSaveContext.magicLevel = 0 as libc::c_int as s8;
    gSaveContext.sceneFlags[5 as libc::c_int as usize].swch =
        0x40000000 as libc::c_int as u32_0;
}
/* *
 *  Copy save currently on the buffer to Save Context and complete various tasks to open the save.
 *  This includes:
 *  - Set proper entrance depending on where the game was saved
 *  - If health is less than 3 hearts, give 3 hearts
 *  - If either scarecrow song is set, copy them from save context to the proper location
 *  - Handle a case where the player saved and quit after zelda cutscene but didnt get the song
 *  - Give and equip master sword if player is adult and doesnt have kokiri sword (bug?)
 *  - Revert any trade items that spoil
 */
#[no_mangle]
pub unsafe extern "C" fn Sram_OpenSave(mut sramCtx: *mut SramContext) {
    static mut dungeonEntrances: [s16; 16] =
        [0 as libc::c_int as s16, 0x4 as libc::c_int as s16,
         0x28 as libc::c_int as s16, 0x169 as libc::c_int as s16,
         0x165 as libc::c_int as s16, 0x10 as libc::c_int as s16,
         0x82 as libc::c_int as s16, 0x37 as libc::c_int as s16,
         0x98 as libc::c_int as s16, 0x88 as libc::c_int as s16,
         0x41b as libc::c_int as s16, 0x8 as libc::c_int as s16,
         0x486 as libc::c_int as s16, 0x467 as libc::c_int as s16,
         0x179 as libc::c_int as s16,
         0x56c as libc::c_int as s16]; // "Create personal file"
    let mut i: u16_0 = 0; // "Point="
    let mut j: u16_0 = 0;
    let mut ptr: *mut u8_0 = 0 as *mut u8_0;
    osSyncPrintf(b"\xe5\x80\x8b\xe4\xba\xba\xef\xbc\xa6\xef\xbd\x89\xef\xbd\x8c\xef\xbd\x85\xe4\xbd\x9c\xe6\x88\x90\n\x00"
                     as *const u8 as *const libc::c_char);
    i = gSramSlotOffsets[gSaveContext.fileNum as usize];
    osSyncPrintf(b"\xe3\x81\xbd\xe3\x81\x84\xe3\x82\x93\xe3\x81\xa8\xef\xbc\x9d%x(%d)\n\x00"
                     as *const u8 as *const libc::c_char, i as libc::c_int,
                 gSaveContext.fileNum);
    MemCopy(&mut gSaveContext as *mut SaveContext as *mut libc::c_void,
            (*sramCtx).readBuff.offset(i as libc::c_int as isize) as
                *mut libc::c_void,
            ::std::mem::size_of::<Save>() as libc::c_ulong as s32);
    osSyncPrintf(b"\x1b[33m\x00" as *const u8 as *const libc::c_char);
    osSyncPrintf(b"SCENE_DATA_ID = %d   SceneNo = %d\n\x00" as *const u8 as
                     *const libc::c_char,
                 gSaveContext.savedSceneNum as libc::c_int,
                 gSaveContext.entranceIndex);
    match gSaveContext.savedSceneNum as libc::c_int {
        0 | 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 => {
            gSaveContext.entranceIndex =
                dungeonEntrances[gSaveContext.savedSceneNum as usize] as s32
        }
        17 => { gSaveContext.entranceIndex = 0 as libc::c_int }
        18 => { gSaveContext.entranceIndex = 4 as libc::c_int }
        19 => { gSaveContext.entranceIndex = 0x28 as libc::c_int }
        20 => { gSaveContext.entranceIndex = 0x169 as libc::c_int }
        21 => { gSaveContext.entranceIndex = 0x165 as libc::c_int }
        22 => { gSaveContext.entranceIndex = 0x10 as libc::c_int }
        23 => { gSaveContext.entranceIndex = 0x82 as libc::c_int }
        24 => { gSaveContext.entranceIndex = 0x37 as libc::c_int }
        14 | 15 | 25 | 26 | 79 => {
            gSaveContext.entranceIndex = 0x41b as libc::c_int
        }
        _ => {
            if gSaveContext.savedSceneNum as libc::c_int !=
                   SCENE_LINK_HOME as libc::c_int {
                gSaveContext.entranceIndex =
                    if (if !(gSaveContext.linkAge == 0 as libc::c_int) {
                            5 as libc::c_int
                        } else { 17 as libc::c_int }) == 5 as libc::c_int {
                        0xbb as libc::c_int
                    } else { 0x5f4 as libc::c_int }
            } else { gSaveContext.entranceIndex = 0xbb as libc::c_int }
        }
    }
    osSyncPrintf(b"scene_no = %d\n\x00" as *const u8 as *const libc::c_char,
                 gSaveContext.entranceIndex);
    osSyncPrintf(b"\x1b[m\x00" as *const u8 as *const libc::c_char);
    if (gSaveContext.health as libc::c_int) < 0x30 as libc::c_int {
        gSaveContext.health = 0x30 as libc::c_int as s16
    }
    if gSaveContext.scarecrowCustomSongSet != 0 {
        osSyncPrintf(b"\x1b[34m\x00" as *const u8 as *const libc::c_char);
        osSyncPrintf(b"\n====================================================================\n\x00"
                         as *const u8 as *const libc::c_char);
        MemCopy(gScarecrowCustomSongPtr as *mut libc::c_void,
                gSaveContext.scarecrowCustomSong.as_mut_ptr() as
                    *mut libc::c_void,
                ::std::mem::size_of::<[u8_0; 864]>() as libc::c_ulong as s32);
        ptr = gScarecrowCustomSongPtr as *mut u8_0;
        i = 0 as libc::c_int as u16_0;
        while (i as libc::c_int) <
                  (::std::mem::size_of::<[u8_0; 864]>() as
                       libc::c_ulong).wrapping_div(::std::mem::size_of::<u8_0>()
                                                       as libc::c_ulong) as
                      s32 {
            osSyncPrintf(b"%d, \x00" as *const u8 as *const libc::c_char,
                         *ptr as libc::c_int);
            i = i.wrapping_add(1);
            ptr = ptr.offset(1)
        }
        osSyncPrintf(b"\n====================================================================\n\x00"
                         as *const u8 as *const libc::c_char);
        osSyncPrintf(b"\x1b[m\x00" as *const u8 as *const libc::c_char);
    }
    if gSaveContext.scarecrowSpawnSongSet != 0 {
        osSyncPrintf(b"\x1b[32m\x00" as *const u8 as *const libc::c_char);
        osSyncPrintf(b"\n====================================================================\n\x00"
                         as *const u8 as *const libc::c_char);
        MemCopy(gScarecrowSpawnSongPtr as *mut libc::c_void,
                gSaveContext.scarecrowSpawnSong.as_mut_ptr() as
                    *mut libc::c_void,
                ::std::mem::size_of::<[u8_0; 128]>() as libc::c_ulong as s32);
        ptr = gScarecrowSpawnSongPtr;
        i = 0 as libc::c_int as u16_0;
        while (i as libc::c_int) <
                  (::std::mem::size_of::<[u8_0; 128]>() as
                       libc::c_ulong).wrapping_div(::std::mem::size_of::<u8_0>()
                                                       as libc::c_ulong) as
                      s32 {
            osSyncPrintf(b"%d, \x00" as *const u8 as *const libc::c_char,
                         *ptr as libc::c_int);
            i = i.wrapping_add(1);
            ptr = ptr.offset(1)
        }
        osSyncPrintf(b"\n====================================================================\n\x00"
                         as *const u8 as *const libc::c_char);
        osSyncPrintf(b"\x1b[m\x00" as *const u8 as *const libc::c_char);
    }
    // if zelda cutscene has been watched but lullaby was not obtained, restore cutscene and take away letter
    if gSaveContext.eventChkInf[4 as libc::c_int as usize] as libc::c_int &
           1 as libc::c_int != 0 &&
           gBitFlags[QUEST_SONG_LULLABY as libc::c_int as usize] &
               gSaveContext.inventory.questItems == 0 {
        i =
            (gSaveContext.eventChkInf[4 as libc::c_int as usize] as
                 libc::c_int & !(1 as libc::c_int)) as u16_0;
        gSaveContext.eventChkInf[4 as libc::c_int as usize] = i;
        gSaveContext.inventory.items[gItemSlots[ITEM_LETTER_ZELDA as
                                                    libc::c_int as usize] as
                                         usize] =
            ITEM_CHICKEN as libc::c_int as u8_0;
        j = 1 as libc::c_int as u16_0;
        while (j as libc::c_int) < 4 as libc::c_int {
            if gSaveContext.equips.buttonItems[j as usize] as libc::c_int ==
                   ITEM_LETTER_ZELDA as libc::c_int {
                gSaveContext.equips.buttonItems[j as usize] =
                    ITEM_CHICKEN as libc::c_int as u8_0
            }
            j = j.wrapping_add(1)
        }
    }
    // check for owning kokiri sword.. to restore master sword? bug or debug feature?
    if (if !(gSaveContext.linkAge == 0 as libc::c_int) {
            5 as libc::c_int
        } else { 17 as libc::c_int }) == 17 as libc::c_int &&
           gBitFlags[1 as libc::c_int as usize] <<
               gEquipShifts[EQUIP_SWORD as libc::c_int as usize] as
                   libc::c_int &
               gSaveContext.inventory.equipment as libc::c_uint == 0 {
        gSaveContext.inventory.equipment =
            (gSaveContext.inventory.equipment as libc::c_uint |
                 gBitFlags[1 as libc::c_int as usize] <<
                     gEquipShifts[EQUIP_SWORD as libc::c_int as usize] as
                         libc::c_int) as u16_0;
        gSaveContext.equips.buttonItems[0 as libc::c_int as usize] =
            ITEM_SWORD_MASTER as libc::c_int as u8_0;
        gSaveContext.equips.equipment =
            (gSaveContext.equips.equipment as libc::c_int &
                 !(0xf as libc::c_int)) as u16_0;
        gSaveContext.equips.equipment =
            (gSaveContext.equips.equipment as libc::c_int | 2 as libc::c_int)
                as u16_0
    }
    i = 0 as libc::c_int as u16_0;
    while (i as libc::c_int) <
              (::std::mem::size_of::<[s16; 3]>() as
                   libc::c_ulong).wrapping_div(::std::mem::size_of::<s16>() as
                                                   libc::c_ulong) as s32 {
        if gSaveContext.inventory.items[gItemSlots[ITEM_POCKET_EGG as
                                                       libc::c_int as usize]
                                            as usize] as libc::c_int ==
               gSpoilingItems[i as usize] as libc::c_int {
            gSaveContext.inventory.items[gItemSlots[gSpoilingItemReverts[i as
                                                                             usize]
                                                        as usize] as usize] =
                gSpoilingItemReverts[i as usize] as u8_0;
            j = 1 as libc::c_int as u16_0;
            while (j as libc::c_int) < 4 as libc::c_int {
                if gSaveContext.equips.buttonItems[j as usize] as libc::c_int
                       == gSpoilingItems[i as usize] as libc::c_int {
                    gSaveContext.equips.buttonItems[j as usize] =
                        gSpoilingItemReverts[i as usize] as u8_0
                }
                j = j.wrapping_add(1)
            }
        }
        i = i.wrapping_add(1)
    }
    gSaveContext.magicLevel = 0 as libc::c_int as s8;
}
/* *
 *  Write the contents of the Save Context to a main and backup slot in SRAM.
 *  Note: The whole Save Context is written even though only the `save` substruct is read back later
 */
#[no_mangle]
pub unsafe extern "C" fn Sram_WriteSave(mut sramCtx: *mut SramContext) {
    let mut offset: u16_0 = 0;
    let mut checksum: u16_0 = 0;
    let mut j: u16_0 = 0;
    let mut ptr: *mut u16_0 = 0 as *mut u16_0;
    gSaveContext.checksum = 0 as libc::c_int as u16_0;
    ptr = &mut gSaveContext as *mut SaveContext as *mut u16_0;
    checksum = 0 as libc::c_int as u16_0;
    j = 0 as libc::c_int as u16_0;
    offset = 0 as libc::c_int as u16_0;
    while (offset as libc::c_uint) <
              (::std::mem::size_of::<Save>() as
                   libc::c_ulong).wrapping_div(2 as libc::c_int as
                                                   libc::c_uint) {
        j = j.wrapping_add(1);
        if j as libc::c_int == 0x20 as libc::c_int {
            j = 0 as libc::c_int as u16_0
        }
        let fresh0 = ptr;
        ptr = ptr.offset(1);
        checksum =
            (checksum as libc::c_int + *fresh0 as libc::c_int) as u16_0;
        offset = offset.wrapping_add(1)
    }
    gSaveContext.checksum = checksum;
    ptr = &mut gSaveContext as *mut SaveContext as *mut u16_0;
    checksum = 0 as libc::c_int as u16_0;
    offset = 0 as libc::c_int as u16_0;
    while (offset as libc::c_uint) <
              (::std::mem::size_of::<Save>() as
                   libc::c_ulong).wrapping_div(2 as libc::c_int as
                                                   libc::c_uint) {
        j = j.wrapping_add(1);
        if j as libc::c_int == 0x20 as libc::c_int {
            j = 0 as libc::c_int as u16_0
        }
        let fresh1 = ptr;
        ptr = ptr.offset(1);
        checksum =
            (checksum as libc::c_int + *fresh1 as libc::c_int) as u16_0;
        offset = offset.wrapping_add(1)
    }
    offset = gSramSlotOffsets[gSaveContext.fileNum as usize];
    SsSram_ReadWrite(((0xa8000000 as libc::c_uint as
                           *mut libc::c_char).offset(-(0xa0000000 as
                                                           libc::c_uint as
                                                           isize)) as
                          u32_0).wrapping_add(offset as libc::c_uint),
                     &mut gSaveContext as *mut SaveContext as
                         *mut libc::c_void,
                     (::std::mem::size_of::<SaveContext>() as
                          libc::c_ulong).wrapping_add(0x28 as libc::c_int as
                                                          libc::c_uint) as
                         size_t, 1 as libc::c_int);
    ptr = &mut gSaveContext as *mut SaveContext as *mut u16_0;
    checksum = 0 as libc::c_int as u16_0;
    offset = 0 as libc::c_int as u16_0;
    while (offset as libc::c_uint) <
              (::std::mem::size_of::<Save>() as
                   libc::c_ulong).wrapping_div(2 as libc::c_int as
                                                   libc::c_uint) {
        j = j.wrapping_add(1);
        if j as libc::c_int == 0x20 as libc::c_int {
            j = 0 as libc::c_int as u16_0
        }
        let fresh2 = ptr;
        ptr = ptr.offset(1);
        checksum =
            (checksum as libc::c_int + *fresh2 as libc::c_int) as u16_0;
        offset = offset.wrapping_add(1)
    }
    offset =
        gSramSlotOffsets[(gSaveContext.fileNum + 3 as libc::c_int) as usize];
    SsSram_ReadWrite(((0xa8000000 as libc::c_uint as
                           *mut libc::c_char).offset(-(0xa0000000 as
                                                           libc::c_uint as
                                                           isize)) as
                          u32_0).wrapping_add(offset as libc::c_uint),
                     &mut gSaveContext as *mut SaveContext as
                         *mut libc::c_void,
                     (::std::mem::size_of::<SaveContext>() as
                          libc::c_ulong).wrapping_add(0x28 as libc::c_int as
                                                          libc::c_uint) as
                         size_t, 1 as libc::c_int);
}
/* *
 *  For all 3 slots, verify that the checksum is correct. If corrupted, attempt to load a backup save.
 *  If backup is also corrupted, default to a new save (or debug save for slot 0 on debug rom).
 *
 *  After verifying all 3 saves, pass relevant data to File Select to be displayed.
 */
#[no_mangle]
pub unsafe extern "C" fn Sram_VerifyAndLoadAllSaves(mut fileChooseCtx:
                                                        *mut FileChooseContext,
                                                    mut sramCtx:
                                                        *mut SramContext) {
    let mut i: u16_0 = 0;
    let mut newChecksum: u16_0 = 0;
    let mut slotNum: u16_0 = 0;
    let mut offset: u16_0 = 0;
    let mut j: u16_0 = 0;
    let mut oldChecksum: u16_0 = 0;
    let mut ptr: *mut u16_0 = 0 as *mut u16_0;
    let mut dayTime: u16_0 = 0;
    osSyncPrintf(b"\xef\xbc\xb3\xef\xbc\xb2\xef\xbc\xa1\xef\xbc\xad \xef\xbc\xb3\xef\xbc\xb4\xef\xbc\xa1\xef\xbc\xb2\xef\xbc\xb4\xe2\x94\x80\xef\xbc\xac\xef\xbc\xaf\xef\xbc\xa1\xef\xbc\xa4\n\x00"
                     as *const u8 as *const libc::c_char);
    bzero((*sramCtx).readBuff as *mut libc::c_void,
          0x8000 as libc::c_int as u32_0);
    SsSram_ReadWrite((0xa8000000 as libc::c_uint as
                          *mut libc::c_char).offset(-(0xa0000000 as
                                                          libc::c_uint as
                                                          isize)) as u32_0,
                     (*sramCtx).readBuff as *mut libc::c_void,
                     0x8000 as libc::c_int as size_t, 0 as libc::c_int);
    dayTime = gSaveContext.dayTime;
    slotNum = 0 as libc::c_int as u16_0;
    while (slotNum as libc::c_int) < 3 as libc::c_int {
        offset = gSramSlotOffsets[slotNum as usize];
        osSyncPrintf(b"\xe3\x81\xbd\xe3\x81\x84\xe3\x82\x93\xe3\x81\xa8\xef\xbc\x9d%x(%d)    SAVE_MAX=%d\n\x00"
                         as *const u8 as *const libc::c_char,
                     offset as libc::c_int, gSaveContext.fileNum,
                     ::std::mem::size_of::<Save>() as libc::c_ulong);
        MemCopy(&mut gSaveContext as *mut SaveContext as *mut libc::c_void,
                (*sramCtx).readBuff.offset(offset as libc::c_int as isize) as
                    *mut libc::c_void,
                ::std::mem::size_of::<Save>() as libc::c_ulong as s32);
        oldChecksum = gSaveContext.checksum;
        gSaveContext.checksum = 0 as libc::c_int as u16_0;
        ptr = &mut gSaveContext as *mut SaveContext as *mut u16_0;
        osSyncPrintf(b"\n\xef\xbc\x9d\xef\xbc\x9d\xef\xbc\x9d\xef\xbc\x9d\xef\xbc\x9d\xef\xbc\x9d\xef\xbc\x9d\xef\xbc\x9d\xef\xbc\x9d\xef\xbc\x9d\xef\xbc\x9d\xef\xbc\x9d\xef\xbc\x9d  \xef\xbc\xb3\xef\xbc\x88%d\xef\xbc\x89 \xef\xbc\x9d\xef\xbc\x9d\xef\xbc\x9d\xef\xbc\x9d\xef\xbc\x9d\xef\xbc\x9d\xef\xbc\x9d\xef\xbc\x9d\xef\xbc\x9d\xef\xbc\x9d\xef\xbc\x9d\xef\xbc\x9d\xef\xbc\x9d\n\x00"
                         as *const u8 as *const libc::c_char,
                     slotNum as libc::c_int);
        j = 0 as libc::c_int as u16_0;
        newChecksum = j;
        i = newChecksum;
        while (i as libc::c_uint) <
                  (::std::mem::size_of::<Save>() as
                       libc::c_ulong).wrapping_div(2 as libc::c_int as
                                                       libc::c_uint) {
            let fresh3 = ptr;
            ptr = ptr.offset(1);
            newChecksum =
                (newChecksum as libc::c_int + *fresh3 as libc::c_int) as
                    u16_0;
            i = i.wrapping_add(1);
            offset = (offset as libc::c_int + 2 as libc::c_int) as u16_0
        }
        // "SAVE checksum calculation"
        osSyncPrintf(b"\n\xef\xbc\xb3\xef\xbc\xa1\xef\xbc\xb6\xef\xbc\xa5\xe3\x83\x81\xe3\x82\xa7\xe3\x83\x83\xe3\x82\xaf\xe3\x82\xb5\xe3\x83\xa0\xe8\xa8\x88\xe7\xae\x97  j=%x  mmm=%x  \x00"
                         as *const u8 as *const libc::c_char,
                     newChecksum as libc::c_int, oldChecksum as libc::c_int);
        if newChecksum as libc::c_int != oldChecksum as libc::c_int {
            // checksum didnt match, try backup save
            osSyncPrintf(b"\xef\xbc\xa5\xef\xbc\xb2\xef\xbc\xb2\xef\xbc\xaf\xef\xbc\xb2\xef\xbc\x81\xef\xbc\x81\xef\xbc\x81 \xef\xbc\x9d %x(%d)\n\x00"
                             as *const u8 as *const libc::c_char,
                         gSramSlotOffsets[slotNum as usize] as libc::c_int,
                         slotNum as libc::c_int);
            offset =
                gSramSlotOffsets[(slotNum as libc::c_int + 3 as libc::c_int)
                                     as usize];
            MemCopy(&mut gSaveContext as *mut SaveContext as
                        *mut libc::c_void,
                    (*sramCtx).readBuff.offset(offset as libc::c_int as isize)
                        as *mut libc::c_void,
                    ::std::mem::size_of::<Save>() as libc::c_ulong as s32);
            oldChecksum = gSaveContext.checksum;
            gSaveContext.checksum = 0 as libc::c_int as u16_0;
            ptr = &mut gSaveContext as *mut SaveContext as *mut u16_0;
            osSyncPrintf(b"================= \xef\xbc\xa2\xef\xbc\xa1\xef\xbc\xa3\xef\xbc\xab\xe2\x94\x80\xef\xbc\xb5\xef\xbc\xb0 ========================\n\x00"
                             as *const u8 as *const libc::c_char);
            j = 0 as libc::c_int as u16_0;
            newChecksum = j;
            i = newChecksum;
            while (i as libc::c_uint) <
                      (::std::mem::size_of::<Save>() as
                           libc::c_ulong).wrapping_div(2 as libc::c_int as
                                                           libc::c_uint) {
                let fresh4 = ptr;
                ptr = ptr.offset(1);
                newChecksum =
                    (newChecksum as libc::c_int + *fresh4 as libc::c_int) as
                        u16_0;
                i = i.wrapping_add(1);
                offset = (offset as libc::c_int + 2 as libc::c_int) as u16_0
            }
            // "(B) SAVE checksum calculation"
            osSyncPrintf(b"\n\xef\xbc\x88\xef\xbc\xa2\xef\xbc\x89\xef\xbc\xb3\xef\xbc\xa1\xef\xbc\xb6\xef\xbc\xa5\xe3\x83\x81\xe3\x82\xa7\xe3\x83\x83\xe3\x82\xaf\xe3\x82\xb5\xe3\x83\xa0\xe8\xa8\x88\xe7\xae\x97  j=%x  mmm=%x  \x00"
                             as *const u8 as *const libc::c_char,
                         newChecksum as libc::c_int,
                         oldChecksum as libc::c_int);
            if newChecksum as libc::c_int != oldChecksum as libc::c_int {
                // backup save didnt work, make new save
                osSyncPrintf(b"\xef\xbc\xa5\xef\xbc\xb2\xef\xbc\xb2\xef\xbc\xaf\xef\xbc\xb2\xef\xbc\x81\xef\xbc\x81\xef\xbc\x81 \xef\xbc\x9d %x(%d+3)\n\x00"
                                 as *const u8 as *const libc::c_char,
                             gSramSlotOffsets[(slotNum as libc::c_int +
                                                   3 as libc::c_int) as usize]
                                 as libc::c_int, slotNum as libc::c_int);
                bzero(&mut gSaveContext.entranceIndex as *mut s32 as
                          *mut libc::c_void,
                      ::std::mem::size_of::<s32>() as libc::c_ulong);
                bzero(&mut gSaveContext.linkAge as *mut s32 as
                          *mut libc::c_void,
                      ::std::mem::size_of::<s32>() as libc::c_ulong);
                bzero(&mut gSaveContext.cutsceneIndex as *mut s32 as
                          *mut libc::c_void,
                      ::std::mem::size_of::<s32>() as libc::c_ulong);
                // note that gSaveContext.dayTime is not actually the sizeof(s32)
                bzero(&mut gSaveContext.dayTime as *mut u16_0 as
                          *mut libc::c_void,
                      ::std::mem::size_of::<s32>() as libc::c_ulong);
                bzero(&mut gSaveContext.nightFlag as *mut s32 as
                          *mut libc::c_void,
                      ::std::mem::size_of::<s32>() as libc::c_ulong);
                bzero(&mut gSaveContext.totalDays as *mut s32 as
                          *mut libc::c_void,
                      ::std::mem::size_of::<s32>() as libc::c_ulong);
                bzero(&mut gSaveContext.bgsDayCount as *mut s32 as
                          *mut libc::c_void,
                      ::std::mem::size_of::<s32>() as libc::c_ulong);
                if slotNum == 0 {
                    Sram_InitDebugSave();
                    gSaveContext.newf[0 as libc::c_int as usize] =
                        'Z' as i32 as libc::c_char;
                    gSaveContext.newf[1 as libc::c_int as usize] =
                        'E' as i32 as libc::c_char;
                    gSaveContext.newf[2 as libc::c_int as usize] =
                        'L' as i32 as libc::c_char;
                    gSaveContext.newf[3 as libc::c_int as usize] =
                        'D' as i32 as libc::c_char;
                    gSaveContext.newf[4 as libc::c_int as usize] =
                        'A' as i32 as libc::c_char;
                    gSaveContext.newf[5 as libc::c_int as usize] =
                        'Z' as i32 as libc::c_char;
                    osSyncPrintf(b"newf=%x,%x,%x,%x,%x,%x\n\x00" as *const u8
                                     as *const libc::c_char,
                                 gSaveContext.newf[0 as libc::c_int as usize]
                                     as libc::c_int,
                                 gSaveContext.newf[1 as libc::c_int as usize]
                                     as libc::c_int,
                                 gSaveContext.newf[2 as libc::c_int as usize]
                                     as libc::c_int,
                                 gSaveContext.newf[3 as libc::c_int as usize]
                                     as libc::c_int,
                                 gSaveContext.newf[4 as libc::c_int as usize]
                                     as libc::c_int,
                                 gSaveContext.newf[5 as libc::c_int as usize]
                                     as libc::c_int);
                } else { Sram_InitNewSave(); }
                ptr = &mut gSaveContext as *mut SaveContext as *mut u16_0;
                osSyncPrintf(b"\n--------------------------------------------------------------\n\x00"
                                 as *const u8 as *const libc::c_char);
                j = 0 as libc::c_int as u16_0;
                newChecksum = j;
                i = newChecksum;
                while (i as libc::c_uint) <
                          (::std::mem::size_of::<Save>() as
                               libc::c_ulong).wrapping_div(2 as libc::c_int as
                                                               libc::c_uint) {
                    osSyncPrintf(b"%x \x00" as *const u8 as
                                     *const libc::c_char,
                                 *ptr as libc::c_int);
                    j = j.wrapping_add(1);
                    if j as libc::c_int == 0x20 as libc::c_int {
                        osSyncPrintf(b"\n\x00" as *const u8 as
                                         *const libc::c_char);
                        j = 0 as libc::c_int as u16_0
                    }
                    let fresh5 = ptr;
                    ptr = ptr.offset(1);
                    newChecksum =
                        (newChecksum as libc::c_int + *fresh5 as libc::c_int)
                            as u16_0;
                    i = i.wrapping_add(1)
                }
                gSaveContext.checksum = newChecksum;
                osSyncPrintf(b"\nCheck_Sum=%x(%x)\n\x00" as *const u8 as
                                 *const libc::c_char,
                             gSaveContext.checksum as libc::c_int,
                             newChecksum as libc::c_int);
                i =
                    gSramSlotOffsets[(slotNum as libc::c_int +
                                          3 as libc::c_int) as usize];
                SsSram_ReadWrite(((0xa8000000 as libc::c_uint as
                                       *mut libc::c_char).offset(-(0xa0000000
                                                                       as
                                                                       libc::c_uint
                                                                       as
                                                                       isize))
                                      as
                                      u32_0).wrapping_add(i as libc::c_uint),
                                 &mut gSaveContext as *mut SaveContext as
                                     *mut libc::c_void,
                                 (::std::mem::size_of::<SaveContext>() as
                                      libc::c_ulong).wrapping_add(0x28 as
                                                                      libc::c_int
                                                                      as
                                                                      libc::c_uint)
                                     as size_t, 1 as libc::c_int);
                osSyncPrintf(b"????#%x,%x,%x,%x,%x,%x\n\x00" as *const u8 as
                                 *const libc::c_char,
                             gSaveContext.newf[0 as libc::c_int as usize] as
                                 libc::c_int,
                             gSaveContext.newf[1 as libc::c_int as usize] as
                                 libc::c_int,
                             gSaveContext.newf[2 as libc::c_int as usize] as
                                 libc::c_int,
                             gSaveContext.newf[3 as libc::c_int as usize] as
                                 libc::c_int,
                             gSaveContext.newf[4 as libc::c_int as usize] as
                                 libc::c_int,
                             gSaveContext.newf[5 as libc::c_int as usize] as
                                 libc::c_int);
                osSyncPrintf(b"\n\xe3\x81\xbd\xe3\x81\x84\xe3\x82\x93\xe3\x81\xa8\xef\xbc\x9d%x(%d+3)  check_sum=%x(%x)\n\x00"
                                 as *const u8 as *const libc::c_char,
                             i as libc::c_int, slotNum as libc::c_int,
                             gSaveContext.checksum as libc::c_int,
                             newChecksum as libc::c_int);
            }
            i = gSramSlotOffsets[slotNum as usize];
            SsSram_ReadWrite(((0xa8000000 as libc::c_uint as
                                   *mut libc::c_char).offset(-(0xa0000000 as
                                                                   libc::c_uint
                                                                   as isize))
                                  as u32_0).wrapping_add(i as libc::c_uint),
                             &mut gSaveContext as *mut SaveContext as
                                 *mut libc::c_void,
                             (::std::mem::size_of::<SaveContext>() as
                                  libc::c_ulong).wrapping_add(0x28 as
                                                                  libc::c_int
                                                                  as
                                                                  libc::c_uint)
                                 as size_t, 1 as libc::c_int);
            osSyncPrintf(b"\xe3\x81\xbd\xe3\x81\x84\xe3\x82\x93\xe3\x81\xa8\xef\xbc\x9d%x(%d)  check_sum=%x(%x)\n\x00"
                             as *const u8 as *const libc::c_char,
                         i as libc::c_int, slotNum as libc::c_int,
                         gSaveContext.checksum as libc::c_int,
                         newChecksum as libc::c_int);
        } else {
            osSyncPrintf(b"\n\xef\xbc\xb3\xef\xbc\xa1\xef\xbc\xb6\xef\xbc\xa5\xe3\x83\x87\xe3\x83\xbc\xe3\x82\xbf \xef\xbc\xaf\xef\xbc\xab\xef\xbc\x81\xef\xbc\x81\xef\xbc\x81\xef\xbc\x81\n\x00"
                             as *const u8 as *const libc::c_char);
            // "SAVE data OK! ! ! !"
        } // "Checksum = %x"
        slotNum = slotNum.wrapping_add(1)
    } // "SAVE end"
    bzero((*sramCtx).readBuff as *mut libc::c_void,
          0x8000 as libc::c_int as u32_0);
    SsSram_ReadWrite((0xa8000000 as libc::c_uint as
                          *mut libc::c_char).offset(-(0xa0000000 as
                                                          libc::c_uint as
                                                          isize)) as u32_0,
                     (*sramCtx).readBuff as *mut libc::c_void,
                     0x8000 as libc::c_int as size_t, 0 as libc::c_int);
    gSaveContext.dayTime = dayTime;
    osSyncPrintf(b"SAVECT=%x, NAME=%x, LIFE=%x, ITEM=%x,  64DD=%x,  HEART=%x\n\x00"
                     as *const u8 as *const libc::c_char,
                 &mut (*(0 as *mut SaveContext)).deaths as *mut u16_0 as
                     size_t,
                 &mut (*(0 as *mut SaveContext)).playerName as
                     *mut [libc::c_char; 8] as size_t,
                 &mut (*(0 as *mut SaveContext)).healthCapacity as *mut s16 as
                     size_t,
                 &mut (*(0 as *mut SaveContext)).inventory.questItems as
                     *mut u32_0 as size_t,
                 &mut (*(0 as *mut SaveContext)).n64ddFlag as *mut s16 as
                     size_t,
                 &mut (*(0 as *mut SaveContext)).inventory.defenseHearts as
                     *mut s8 as size_t);
    MemCopy(&mut *(*fileChooseCtx).deaths.as_mut_ptr().offset(0 as libc::c_int
                                                                  as isize) as
                *mut u16_0 as *mut libc::c_void,
            (*sramCtx).readBuff.offset(((0x10 as libc::c_int +
                                             0x10 as libc::c_int) as
                                            libc::c_uint).wrapping_add((0 as
                                                                            libc::c_int
                                                                            as
                                                                            libc::c_uint).wrapping_mul((::std::mem::size_of::<SaveContext>()
                                                                                                            as
                                                                                                            libc::c_ulong).wrapping_add(0x28
                                                                                                                                            as
                                                                                                                                            libc::c_int
                                                                                                                                            as
                                                                                                                                            libc::c_uint)))
                                           as
                                           isize).offset(&mut (*(0 as
                                                                     *mut SaveContext)).deaths
                                                             as *mut u16_0 as
                                                             size_t as isize)
                as *mut libc::c_void,
            ::std::mem::size_of::<u16_0>() as libc::c_ulong as s32);
    MemCopy(&mut *(*fileChooseCtx).deaths.as_mut_ptr().offset(1 as libc::c_int
                                                                  as isize) as
                *mut u16_0 as *mut libc::c_void,
            (*sramCtx).readBuff.offset(((0x10 as libc::c_int +
                                             0x10 as libc::c_int) as
                                            libc::c_uint).wrapping_add((1 as
                                                                            libc::c_int
                                                                            as
                                                                            libc::c_uint).wrapping_mul((::std::mem::size_of::<SaveContext>()
                                                                                                            as
                                                                                                            libc::c_ulong).wrapping_add(0x28
                                                                                                                                            as
                                                                                                                                            libc::c_int
                                                                                                                                            as
                                                                                                                                            libc::c_uint)))
                                           as
                                           isize).offset(&mut (*(0 as
                                                                     *mut SaveContext)).deaths
                                                             as *mut u16_0 as
                                                             size_t as isize)
                as *mut libc::c_void,
            ::std::mem::size_of::<u16_0>() as libc::c_ulong as s32);
    MemCopy(&mut *(*fileChooseCtx).deaths.as_mut_ptr().offset(2 as libc::c_int
                                                                  as isize) as
                *mut u16_0 as *mut libc::c_void,
            (*sramCtx).readBuff.offset(((0x10 as libc::c_int +
                                             0x10 as libc::c_int) as
                                            libc::c_uint).wrapping_add((2 as
                                                                            libc::c_int
                                                                            as
                                                                            libc::c_uint).wrapping_mul((::std::mem::size_of::<SaveContext>()
                                                                                                            as
                                                                                                            libc::c_ulong).wrapping_add(0x28
                                                                                                                                            as
                                                                                                                                            libc::c_int
                                                                                                                                            as
                                                                                                                                            libc::c_uint)))
                                           as
                                           isize).offset(&mut (*(0 as
                                                                     *mut SaveContext)).deaths
                                                             as *mut u16_0 as
                                                             size_t as isize)
                as *mut libc::c_void,
            ::std::mem::size_of::<u16_0>() as libc::c_ulong as s32);
    MemCopy(&mut *(*fileChooseCtx).fileNames.as_mut_ptr().offset(0 as
                                                                     libc::c_int
                                                                     as isize)
                as *mut [u8_0; 8] as *mut libc::c_void,
            (*sramCtx).readBuff.offset(((0x10 as libc::c_int +
                                             0x10 as libc::c_int) as
                                            libc::c_uint).wrapping_add((0 as
                                                                            libc::c_int
                                                                            as
                                                                            libc::c_uint).wrapping_mul((::std::mem::size_of::<SaveContext>()
                                                                                                            as
                                                                                                            libc::c_ulong).wrapping_add(0x28
                                                                                                                                            as
                                                                                                                                            libc::c_int
                                                                                                                                            as
                                                                                                                                            libc::c_uint)))
                                           as
                                           isize).offset(&mut (*(0 as
                                                                     *mut SaveContext)).playerName
                                                             as
                                                             *mut [libc::c_char; 8]
                                                             as size_t as
                                                             isize) as
                *mut libc::c_void,
            ::std::mem::size_of::<[u8_0; 8]>() as libc::c_ulong as s32);
    MemCopy(&mut *(*fileChooseCtx).fileNames.as_mut_ptr().offset(1 as
                                                                     libc::c_int
                                                                     as isize)
                as *mut [u8_0; 8] as *mut libc::c_void,
            (*sramCtx).readBuff.offset(((0x10 as libc::c_int +
                                             0x10 as libc::c_int) as
                                            libc::c_uint).wrapping_add((1 as
                                                                            libc::c_int
                                                                            as
                                                                            libc::c_uint).wrapping_mul((::std::mem::size_of::<SaveContext>()
                                                                                                            as
                                                                                                            libc::c_ulong).wrapping_add(0x28
                                                                                                                                            as
                                                                                                                                            libc::c_int
                                                                                                                                            as
                                                                                                                                            libc::c_uint)))
                                           as
                                           isize).offset(&mut (*(0 as
                                                                     *mut SaveContext)).playerName
                                                             as
                                                             *mut [libc::c_char; 8]
                                                             as size_t as
                                                             isize) as
                *mut libc::c_void,
            ::std::mem::size_of::<[u8_0; 8]>() as libc::c_ulong as s32);
    MemCopy(&mut *(*fileChooseCtx).fileNames.as_mut_ptr().offset(2 as
                                                                     libc::c_int
                                                                     as isize)
                as *mut [u8_0; 8] as *mut libc::c_void,
            (*sramCtx).readBuff.offset(((0x10 as libc::c_int +
                                             0x10 as libc::c_int) as
                                            libc::c_uint).wrapping_add((2 as
                                                                            libc::c_int
                                                                            as
                                                                            libc::c_uint).wrapping_mul((::std::mem::size_of::<SaveContext>()
                                                                                                            as
                                                                                                            libc::c_ulong).wrapping_add(0x28
                                                                                                                                            as
                                                                                                                                            libc::c_int
                                                                                                                                            as
                                                                                                                                            libc::c_uint)))
                                           as
                                           isize).offset(&mut (*(0 as
                                                                     *mut SaveContext)).playerName
                                                             as
                                                             *mut [libc::c_char; 8]
                                                             as size_t as
                                                             isize) as
                *mut libc::c_void,
            ::std::mem::size_of::<[u8_0; 8]>() as libc::c_ulong as s32);
    MemCopy(&mut *(*fileChooseCtx).healthCapacities.as_mut_ptr().offset(0 as
                                                                            libc::c_int
                                                                            as
                                                                            isize)
                as *mut u16_0 as *mut libc::c_void,
            (*sramCtx).readBuff.offset(((0x10 as libc::c_int +
                                             0x10 as libc::c_int) as
                                            libc::c_uint).wrapping_add((0 as
                                                                            libc::c_int
                                                                            as
                                                                            libc::c_uint).wrapping_mul((::std::mem::size_of::<SaveContext>()
                                                                                                            as
                                                                                                            libc::c_ulong).wrapping_add(0x28
                                                                                                                                            as
                                                                                                                                            libc::c_int
                                                                                                                                            as
                                                                                                                                            libc::c_uint)))
                                           as
                                           isize).offset(&mut (*(0 as
                                                                     *mut SaveContext)).healthCapacity
                                                             as *mut s16 as
                                                             size_t as isize)
                as *mut libc::c_void,
            ::std::mem::size_of::<u16_0>() as libc::c_ulong as s32);
    MemCopy(&mut *(*fileChooseCtx).healthCapacities.as_mut_ptr().offset(1 as
                                                                            libc::c_int
                                                                            as
                                                                            isize)
                as *mut u16_0 as *mut libc::c_void,
            (*sramCtx).readBuff.offset(((0x10 as libc::c_int +
                                             0x10 as libc::c_int) as
                                            libc::c_uint).wrapping_add((1 as
                                                                            libc::c_int
                                                                            as
                                                                            libc::c_uint).wrapping_mul((::std::mem::size_of::<SaveContext>()
                                                                                                            as
                                                                                                            libc::c_ulong).wrapping_add(0x28
                                                                                                                                            as
                                                                                                                                            libc::c_int
                                                                                                                                            as
                                                                                                                                            libc::c_uint)))
                                           as
                                           isize).offset(&mut (*(0 as
                                                                     *mut SaveContext)).healthCapacity
                                                             as *mut s16 as
                                                             size_t as isize)
                as *mut libc::c_void,
            ::std::mem::size_of::<u16_0>() as libc::c_ulong as s32);
    MemCopy(&mut *(*fileChooseCtx).healthCapacities.as_mut_ptr().offset(2 as
                                                                            libc::c_int
                                                                            as
                                                                            isize)
                as *mut u16_0 as *mut libc::c_void,
            (*sramCtx).readBuff.offset(((0x10 as libc::c_int +
                                             0x10 as libc::c_int) as
                                            libc::c_uint).wrapping_add((2 as
                                                                            libc::c_int
                                                                            as
                                                                            libc::c_uint).wrapping_mul((::std::mem::size_of::<SaveContext>()
                                                                                                            as
                                                                                                            libc::c_ulong).wrapping_add(0x28
                                                                                                                                            as
                                                                                                                                            libc::c_int
                                                                                                                                            as
                                                                                                                                            libc::c_uint)))
                                           as
                                           isize).offset(&mut (*(0 as
                                                                     *mut SaveContext)).healthCapacity
                                                             as *mut s16 as
                                                             size_t as isize)
                as *mut libc::c_void,
            ::std::mem::size_of::<u16_0>() as libc::c_ulong as s32);
    MemCopy(&mut *(*fileChooseCtx).questItems.as_mut_ptr().offset(0 as
                                                                      libc::c_int
                                                                      as
                                                                      isize)
                as *mut u32_0 as *mut libc::c_void,
            (*sramCtx).readBuff.offset(((0x10 as libc::c_int +
                                             0x10 as libc::c_int) as
                                            libc::c_uint).wrapping_add((0 as
                                                                            libc::c_int
                                                                            as
                                                                            libc::c_uint).wrapping_mul((::std::mem::size_of::<SaveContext>()
                                                                                                            as
                                                                                                            libc::c_ulong).wrapping_add(0x28
                                                                                                                                            as
                                                                                                                                            libc::c_int
                                                                                                                                            as
                                                                                                                                            libc::c_uint)))
                                           as
                                           isize).offset(&mut (*(0 as
                                                                     *mut SaveContext)).inventory.questItems
                                                             as *mut u32_0 as
                                                             size_t as isize)
                as *mut libc::c_void,
            ::std::mem::size_of::<u32_0>() as libc::c_ulong as s32);
    MemCopy(&mut *(*fileChooseCtx).questItems.as_mut_ptr().offset(1 as
                                                                      libc::c_int
                                                                      as
                                                                      isize)
                as *mut u32_0 as *mut libc::c_void,
            (*sramCtx).readBuff.offset(((0x10 as libc::c_int +
                                             0x10 as libc::c_int) as
                                            libc::c_uint).wrapping_add((1 as
                                                                            libc::c_int
                                                                            as
                                                                            libc::c_uint).wrapping_mul((::std::mem::size_of::<SaveContext>()
                                                                                                            as
                                                                                                            libc::c_ulong).wrapping_add(0x28
                                                                                                                                            as
                                                                                                                                            libc::c_int
                                                                                                                                            as
                                                                                                                                            libc::c_uint)))
                                           as
                                           isize).offset(&mut (*(0 as
                                                                     *mut SaveContext)).inventory.questItems
                                                             as *mut u32_0 as
                                                             size_t as isize)
                as *mut libc::c_void,
            ::std::mem::size_of::<u32_0>() as libc::c_ulong as s32);
    MemCopy(&mut *(*fileChooseCtx).questItems.as_mut_ptr().offset(2 as
                                                                      libc::c_int
                                                                      as
                                                                      isize)
                as *mut u32_0 as *mut libc::c_void,
            (*sramCtx).readBuff.offset(((0x10 as libc::c_int +
                                             0x10 as libc::c_int) as
                                            libc::c_uint).wrapping_add((2 as
                                                                            libc::c_int
                                                                            as
                                                                            libc::c_uint).wrapping_mul((::std::mem::size_of::<SaveContext>()
                                                                                                            as
                                                                                                            libc::c_ulong).wrapping_add(0x28
                                                                                                                                            as
                                                                                                                                            libc::c_int
                                                                                                                                            as
                                                                                                                                            libc::c_uint)))
                                           as
                                           isize).offset(&mut (*(0 as
                                                                     *mut SaveContext)).inventory.questItems
                                                             as *mut u32_0 as
                                                             size_t as isize)
                as *mut libc::c_void,
            ::std::mem::size_of::<u32_0>() as libc::c_ulong as s32);
    MemCopy(&mut *(*fileChooseCtx).n64ddFlags.as_mut_ptr().offset(0 as
                                                                      libc::c_int
                                                                      as
                                                                      isize)
                as *mut s16 as *mut libc::c_void,
            (*sramCtx).readBuff.offset(((0x10 as libc::c_int +
                                             0x10 as libc::c_int) as
                                            libc::c_uint).wrapping_add((0 as
                                                                            libc::c_int
                                                                            as
                                                                            libc::c_uint).wrapping_mul((::std::mem::size_of::<SaveContext>()
                                                                                                            as
                                                                                                            libc::c_ulong).wrapping_add(0x28
                                                                                                                                            as
                                                                                                                                            libc::c_int
                                                                                                                                            as
                                                                                                                                            libc::c_uint)))
                                           as
                                           isize).offset(&mut (*(0 as
                                                                     *mut SaveContext)).n64ddFlag
                                                             as *mut s16 as
                                                             size_t as isize)
                as *mut libc::c_void,
            ::std::mem::size_of::<s16>() as libc::c_ulong as s32);
    MemCopy(&mut *(*fileChooseCtx).n64ddFlags.as_mut_ptr().offset(1 as
                                                                      libc::c_int
                                                                      as
                                                                      isize)
                as *mut s16 as *mut libc::c_void,
            (*sramCtx).readBuff.offset(((0x10 as libc::c_int +
                                             0x10 as libc::c_int) as
                                            libc::c_uint).wrapping_add((1 as
                                                                            libc::c_int
                                                                            as
                                                                            libc::c_uint).wrapping_mul((::std::mem::size_of::<SaveContext>()
                                                                                                            as
                                                                                                            libc::c_ulong).wrapping_add(0x28
                                                                                                                                            as
                                                                                                                                            libc::c_int
                                                                                                                                            as
                                                                                                                                            libc::c_uint)))
                                           as
                                           isize).offset(&mut (*(0 as
                                                                     *mut SaveContext)).n64ddFlag
                                                             as *mut s16 as
                                                             size_t as isize)
                as *mut libc::c_void,
            ::std::mem::size_of::<s16>() as libc::c_ulong as s32);
    MemCopy(&mut *(*fileChooseCtx).n64ddFlags.as_mut_ptr().offset(2 as
                                                                      libc::c_int
                                                                      as
                                                                      isize)
                as *mut s16 as *mut libc::c_void,
            (*sramCtx).readBuff.offset(((0x10 as libc::c_int +
                                             0x10 as libc::c_int) as
                                            libc::c_uint).wrapping_add((2 as
                                                                            libc::c_int
                                                                            as
                                                                            libc::c_uint).wrapping_mul((::std::mem::size_of::<SaveContext>()
                                                                                                            as
                                                                                                            libc::c_ulong).wrapping_add(0x28
                                                                                                                                            as
                                                                                                                                            libc::c_int
                                                                                                                                            as
                                                                                                                                            libc::c_uint)))
                                           as
                                           isize).offset(&mut (*(0 as
                                                                     *mut SaveContext)).n64ddFlag
                                                             as *mut s16 as
                                                             size_t as isize)
                as *mut libc::c_void,
            ::std::mem::size_of::<s16>() as libc::c_ulong as s32);
    MemCopy(&mut *(*fileChooseCtx).defense.as_mut_ptr().offset(0 as
                                                                   libc::c_int
                                                                   as isize)
                as *mut s8 as *mut libc::c_void,
            (*sramCtx).readBuff.offset(((0x10 as libc::c_int +
                                             0x10 as libc::c_int) as
                                            libc::c_uint).wrapping_add((0 as
                                                                            libc::c_int
                                                                            as
                                                                            libc::c_uint).wrapping_mul((::std::mem::size_of::<SaveContext>()
                                                                                                            as
                                                                                                            libc::c_ulong).wrapping_add(0x28
                                                                                                                                            as
                                                                                                                                            libc::c_int
                                                                                                                                            as
                                                                                                                                            libc::c_uint)))
                                           as
                                           isize).offset(&mut (*(0 as
                                                                     *mut SaveContext)).inventory.defenseHearts
                                                             as *mut s8 as
                                                             size_t as isize)
                as *mut libc::c_void,
            ::std::mem::size_of::<s8>() as libc::c_ulong as s32);
    MemCopy(&mut *(*fileChooseCtx).defense.as_mut_ptr().offset(1 as
                                                                   libc::c_int
                                                                   as isize)
                as *mut s8 as *mut libc::c_void,
            (*sramCtx).readBuff.offset(((0x10 as libc::c_int +
                                             0x10 as libc::c_int) as
                                            libc::c_uint).wrapping_add((1 as
                                                                            libc::c_int
                                                                            as
                                                                            libc::c_uint).wrapping_mul((::std::mem::size_of::<SaveContext>()
                                                                                                            as
                                                                                                            libc::c_ulong).wrapping_add(0x28
                                                                                                                                            as
                                                                                                                                            libc::c_int
                                                                                                                                            as
                                                                                                                                            libc::c_uint)))
                                           as
                                           isize).offset(&mut (*(0 as
                                                                     *mut SaveContext)).inventory.defenseHearts
                                                             as *mut s8 as
                                                             size_t as isize)
                as *mut libc::c_void,
            ::std::mem::size_of::<s8>() as libc::c_ulong as s32);
    MemCopy(&mut *(*fileChooseCtx).defense.as_mut_ptr().offset(2 as
                                                                   libc::c_int
                                                                   as isize)
                as *mut s8 as *mut libc::c_void,
            (*sramCtx).readBuff.offset(((0x10 as libc::c_int +
                                             0x10 as libc::c_int) as
                                            libc::c_uint).wrapping_add((2 as
                                                                            libc::c_int
                                                                            as
                                                                            libc::c_uint).wrapping_mul((::std::mem::size_of::<SaveContext>()
                                                                                                            as
                                                                                                            libc::c_ulong).wrapping_add(0x28
                                                                                                                                            as
                                                                                                                                            libc::c_int
                                                                                                                                            as
                                                                                                                                            libc::c_uint)))
                                           as
                                           isize).offset(&mut (*(0 as
                                                                     *mut SaveContext)).inventory.defenseHearts
                                                             as *mut s8 as
                                                             size_t as isize)
                as *mut libc::c_void,
            ::std::mem::size_of::<s8>() as libc::c_ulong as s32);
    MemCopy(&mut *(*fileChooseCtx).health.as_mut_ptr().offset(0 as libc::c_int
                                                                  as isize) as
                *mut u16_0 as *mut libc::c_void,
            (*sramCtx).readBuff.offset(((0x10 as libc::c_int +
                                             0x10 as libc::c_int) as
                                            libc::c_uint).wrapping_add((0 as
                                                                            libc::c_int
                                                                            as
                                                                            libc::c_uint).wrapping_mul((::std::mem::size_of::<SaveContext>()
                                                                                                            as
                                                                                                            libc::c_ulong).wrapping_add(0x28
                                                                                                                                            as
                                                                                                                                            libc::c_int
                                                                                                                                            as
                                                                                                                                            libc::c_uint)))
                                           as
                                           isize).offset(&mut (*(0 as
                                                                     *mut SaveContext)).health
                                                             as *mut s16 as
                                                             size_t as isize)
                as *mut libc::c_void,
            ::std::mem::size_of::<u16_0>() as libc::c_ulong as s32);
    MemCopy(&mut *(*fileChooseCtx).health.as_mut_ptr().offset(1 as libc::c_int
                                                                  as isize) as
                *mut u16_0 as *mut libc::c_void,
            (*sramCtx).readBuff.offset(((0x10 as libc::c_int +
                                             0x10 as libc::c_int) as
                                            libc::c_uint).wrapping_add((1 as
                                                                            libc::c_int
                                                                            as
                                                                            libc::c_uint).wrapping_mul((::std::mem::size_of::<SaveContext>()
                                                                                                            as
                                                                                                            libc::c_ulong).wrapping_add(0x28
                                                                                                                                            as
                                                                                                                                            libc::c_int
                                                                                                                                            as
                                                                                                                                            libc::c_uint)))
                                           as
                                           isize).offset(&mut (*(0 as
                                                                     *mut SaveContext)).health
                                                             as *mut s16 as
                                                             size_t as isize)
                as *mut libc::c_void,
            ::std::mem::size_of::<u16_0>() as libc::c_ulong as s32);
    MemCopy(&mut *(*fileChooseCtx).health.as_mut_ptr().offset(2 as libc::c_int
                                                                  as isize) as
                *mut u16_0 as *mut libc::c_void,
            (*sramCtx).readBuff.offset(((0x10 as libc::c_int +
                                             0x10 as libc::c_int) as
                                            libc::c_uint).wrapping_add((2 as
                                                                            libc::c_int
                                                                            as
                                                                            libc::c_uint).wrapping_mul((::std::mem::size_of::<SaveContext>()
                                                                                                            as
                                                                                                            libc::c_ulong).wrapping_add(0x28
                                                                                                                                            as
                                                                                                                                            libc::c_int
                                                                                                                                            as
                                                                                                                                            libc::c_uint)))
                                           as
                                           isize).offset(&mut (*(0 as
                                                                     *mut SaveContext)).health
                                                             as *mut s16 as
                                                             size_t as isize)
                as *mut libc::c_void,
            ::std::mem::size_of::<u16_0>() as libc::c_ulong as s32);
    osSyncPrintf(b"f_64dd=%d, %d, %d\n\x00" as *const u8 as
                     *const libc::c_char,
                 (*fileChooseCtx).n64ddFlags[0 as libc::c_int as usize] as
                     libc::c_int,
                 (*fileChooseCtx).n64ddFlags[1 as libc::c_int as usize] as
                     libc::c_int,
                 (*fileChooseCtx).n64ddFlags[2 as libc::c_int as usize] as
                     libc::c_int);
    osSyncPrintf(b"heart_status=%d, %d, %d\n\x00" as *const u8 as
                     *const libc::c_char,
                 (*fileChooseCtx).defense[0 as libc::c_int as usize] as
                     libc::c_int,
                 (*fileChooseCtx).defense[1 as libc::c_int as usize] as
                     libc::c_int,
                 (*fileChooseCtx).defense[2 as libc::c_int as usize] as
                     libc::c_int);
    osSyncPrintf(b"now_life=%d, %d, %d\n\x00" as *const u8 as
                     *const libc::c_char,
                 (*fileChooseCtx).health[0 as libc::c_int as usize] as
                     libc::c_int,
                 (*fileChooseCtx).health[1 as libc::c_int as usize] as
                     libc::c_int,
                 (*fileChooseCtx).health[2 as libc::c_int as usize] as
                     libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn Sram_InitSave(mut fileChooseCtx:
                                           *mut FileChooseContext,
                                       mut sramCtx: *mut SramContext) {
    let mut offset: u16_0 = 0;
    let mut j: u16_0 = 0;
    let mut ptr: *mut u16_0 = 0 as *mut u16_0;
    let mut checksum: u16_0 = 0;
    if (*fileChooseCtx).buttonIndex as libc::c_int != 0 as libc::c_int {
        Sram_InitNewSave();
    } else { Sram_InitDebugSave(); }
    gSaveContext.entranceIndex = 0xbb as libc::c_int;
    gSaveContext.linkAge = 1 as libc::c_int;
    gSaveContext.dayTime = 0x6aab as libc::c_int as u16_0;
    gSaveContext.cutsceneIndex = 0xfff1 as libc::c_int;
    if (*fileChooseCtx).buttonIndex as libc::c_int == 0 as libc::c_int {
        gSaveContext.cutsceneIndex = 0 as libc::c_int
    }
    offset = 0 as libc::c_int as u16_0;
    while (offset as libc::c_int) < 8 as libc::c_int {
        gSaveContext.playerName[offset as usize] =
            (*fileChooseCtx).fileNames[(*fileChooseCtx).buttonIndex as
                                           usize][offset as usize] as
                libc::c_char;
        offset = offset.wrapping_add(1)
    }
    gSaveContext.newf[0 as libc::c_int as usize] = 'Z' as i32 as libc::c_char;
    gSaveContext.newf[1 as libc::c_int as usize] = 'E' as i32 as libc::c_char;
    gSaveContext.newf[2 as libc::c_int as usize] = 'L' as i32 as libc::c_char;
    gSaveContext.newf[3 as libc::c_int as usize] = 'D' as i32 as libc::c_char;
    gSaveContext.newf[4 as libc::c_int as usize] = 'A' as i32 as libc::c_char;
    gSaveContext.newf[5 as libc::c_int as usize] = 'Z' as i32 as libc::c_char;
    gSaveContext.n64ddFlag = (*fileChooseCtx).n64ddFlag as s16;
    osSyncPrintf(b"\xef\xbc\x96\xef\xbc\x94\xef\xbc\xa4\xef\xbc\xa4\xe3\x83\x95\xe3\x83\xa9\xe3\x82\xb0=%d\n\x00"
                     as *const u8 as *const libc::c_char,
                 (*fileChooseCtx).n64ddFlag as libc::c_int);
    osSyncPrintf(b"newf=%x,%x,%x,%x,%x,%x\n\x00" as *const u8 as
                     *const libc::c_char,
                 gSaveContext.newf[0 as libc::c_int as usize] as libc::c_int,
                 gSaveContext.newf[1 as libc::c_int as usize] as libc::c_int,
                 gSaveContext.newf[2 as libc::c_int as usize] as libc::c_int,
                 gSaveContext.newf[3 as libc::c_int as usize] as libc::c_int,
                 gSaveContext.newf[4 as libc::c_int as usize] as libc::c_int,
                 gSaveContext.newf[5 as libc::c_int as usize] as libc::c_int);
    osSyncPrintf(b"\n$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$\n\x00"
                     as *const u8 as *const libc::c_char);
    ptr = &mut gSaveContext as *mut SaveContext as *mut u16_0;
    j = 0 as libc::c_int as u16_0;
    checksum = 0 as libc::c_int as u16_0;
    offset = 0 as libc::c_int as u16_0;
    while (offset as libc::c_uint) <
              (::std::mem::size_of::<Save>() as
                   libc::c_ulong).wrapping_div(2 as libc::c_int as
                                                   libc::c_uint) {
        osSyncPrintf(b"%x \x00" as *const u8 as *const libc::c_char,
                     *ptr as libc::c_int);
        let fresh6 = ptr;
        ptr = ptr.offset(1);
        checksum =
            (checksum as libc::c_int + *fresh6 as libc::c_int) as u16_0;
        j = j.wrapping_add(1);
        if j as libc::c_int == 0x20 as libc::c_int {
            osSyncPrintf(b"\n\x00" as *const u8 as *const libc::c_char);
            j = 0 as libc::c_int as u16_0
        }
        offset = offset.wrapping_add(1)
    }
    gSaveContext.checksum = checksum;
    osSyncPrintf(b"\n\xe3\x83\x81\xe3\x82\xa7\xe3\x83\x83\xe3\x82\xaf\xe3\x82\xb5\xe3\x83\xa0\xef\xbc\x9d%x\n\x00"
                     as *const u8 as *const libc::c_char,
                 gSaveContext.checksum as libc::c_int);
    offset = gSramSlotOffsets[gSaveContext.fileNum as usize];
    osSyncPrintf(b"I=%x no=%d\n\x00" as *const u8 as *const libc::c_char,
                 offset as libc::c_int, gSaveContext.fileNum);
    MemCopy((*sramCtx).readBuff.offset(offset as libc::c_int as isize) as
                *mut libc::c_void,
            &mut gSaveContext as *mut SaveContext as *mut libc::c_void,
            ::std::mem::size_of::<Save>() as libc::c_ulong as s32);
    offset =
        gSramSlotOffsets[(gSaveContext.fileNum + 3 as libc::c_int) as usize];
    osSyncPrintf(b"I=%x no=%d\n\x00" as *const u8 as *const libc::c_char,
                 offset as libc::c_int,
                 gSaveContext.fileNum + 3 as libc::c_int);
    MemCopy((*sramCtx).readBuff.offset(offset as libc::c_int as isize) as
                *mut libc::c_void,
            &mut gSaveContext as *mut SaveContext as *mut libc::c_void,
            ::std::mem::size_of::<Save>() as libc::c_ulong as s32);
    SsSram_ReadWrite((0xa8000000 as libc::c_uint as
                          *mut libc::c_char).offset(-(0xa0000000 as
                                                          libc::c_uint as
                                                          isize)) as u32_0,
                     (*sramCtx).readBuff as *mut libc::c_void,
                     0x8000 as libc::c_int as size_t, 1 as libc::c_int);
    osSyncPrintf(b"\xef\xbc\xb3\xef\xbc\xa1\xef\xbc\xb6\xef\xbc\xa5\xe7\xb5\x82\xe4\xba\x86\n\x00"
                     as *const u8 as *const libc::c_char);
    osSyncPrintf(b"z_common_data.file_no = %d\n\x00" as *const u8 as
                     *const libc::c_char, gSaveContext.fileNum);
    osSyncPrintf(b"SAVECT=%x, NAME=%x, LIFE=%x, ITEM=%x,  SAVE_64DD=%x\n\x00"
                     as *const u8 as *const libc::c_char,
                 &mut (*(0 as *mut SaveContext)).deaths as *mut u16_0 as
                     size_t,
                 &mut (*(0 as *mut SaveContext)).playerName as
                     *mut [libc::c_char; 8] as size_t,
                 &mut (*(0 as *mut SaveContext)).healthCapacity as *mut s16 as
                     size_t,
                 &mut (*(0 as *mut SaveContext)).inventory.questItems as
                     *mut u32_0 as size_t,
                 &mut (*(0 as *mut SaveContext)).n64ddFlag as *mut s16 as
                     size_t);
    j = gSramSlotOffsets[gSaveContext.fileNum as usize];
    MemCopy(&mut *(*fileChooseCtx).deaths.as_mut_ptr().offset(gSaveContext.fileNum
                                                                  as isize) as
                *mut u16_0 as *mut libc::c_void,
            (*sramCtx).readBuff.offset(j as libc::c_int as
                                           isize).offset(&mut (*(0 as
                                                                     *mut SaveContext)).deaths
                                                             as *mut u16_0 as
                                                             size_t as isize)
                as *mut libc::c_void,
            ::std::mem::size_of::<u16_0>() as libc::c_ulong as s32);
    MemCopy(&mut *(*fileChooseCtx).fileNames.as_mut_ptr().offset(gSaveContext.fileNum
                                                                     as isize)
                as *mut [u8_0; 8] as *mut libc::c_void,
            (*sramCtx).readBuff.offset(j as libc::c_int as
                                           isize).offset(&mut (*(0 as
                                                                     *mut SaveContext)).playerName
                                                             as
                                                             *mut [libc::c_char; 8]
                                                             as size_t as
                                                             isize) as
                *mut libc::c_void,
            ::std::mem::size_of::<[u8_0; 8]>() as libc::c_ulong as s32);
    MemCopy(&mut *(*fileChooseCtx).healthCapacities.as_mut_ptr().offset(gSaveContext.fileNum
                                                                            as
                                                                            isize)
                as *mut u16_0 as *mut libc::c_void,
            (*sramCtx).readBuff.offset(j as libc::c_int as
                                           isize).offset(&mut (*(0 as
                                                                     *mut SaveContext)).healthCapacity
                                                             as *mut s16 as
                                                             size_t as isize)
                as *mut libc::c_void,
            ::std::mem::size_of::<u16_0>() as libc::c_ulong as s32);
    MemCopy(&mut *(*fileChooseCtx).questItems.as_mut_ptr().offset(gSaveContext.fileNum
                                                                      as
                                                                      isize)
                as *mut u32_0 as *mut libc::c_void,
            (*sramCtx).readBuff.offset(j as libc::c_int as
                                           isize).offset(&mut (*(0 as
                                                                     *mut SaveContext)).inventory.questItems
                                                             as *mut u32_0 as
                                                             size_t as isize)
                as *mut libc::c_void,
            ::std::mem::size_of::<u32_0>() as libc::c_ulong as s32);
    MemCopy(&mut *(*fileChooseCtx).n64ddFlags.as_mut_ptr().offset(gSaveContext.fileNum
                                                                      as
                                                                      isize)
                as *mut s16 as *mut libc::c_void,
            (*sramCtx).readBuff.offset(j as libc::c_int as
                                           isize).offset(&mut (*(0 as
                                                                     *mut SaveContext)).n64ddFlag
                                                             as *mut s16 as
                                                             size_t as isize)
                as *mut libc::c_void,
            ::std::mem::size_of::<s16>() as libc::c_ulong as s32);
    MemCopy(&mut *(*fileChooseCtx).defense.as_mut_ptr().offset(gSaveContext.fileNum
                                                                   as isize)
                as *mut s8 as *mut libc::c_void,
            (*sramCtx).readBuff.offset(j as libc::c_int as
                                           isize).offset(&mut (*(0 as
                                                                     *mut SaveContext)).inventory.defenseHearts
                                                             as *mut s8 as
                                                             size_t as isize)
                as *mut libc::c_void,
            ::std::mem::size_of::<s8>() as libc::c_ulong as s32);
    MemCopy(&mut *(*fileChooseCtx).health.as_mut_ptr().offset(gSaveContext.fileNum
                                                                  as isize) as
                *mut u16_0 as *mut libc::c_void,
            (*sramCtx).readBuff.offset(j as libc::c_int as
                                           isize).offset(&mut (*(0 as
                                                                     *mut SaveContext)).health
                                                             as *mut s16 as
                                                             size_t as isize)
                as *mut libc::c_void,
            ::std::mem::size_of::<u16_0>() as libc::c_ulong as s32);
    osSyncPrintf(b"f_64dd[%d]=%d\n\x00" as *const u8 as *const libc::c_char,
                 gSaveContext.fileNum,
                 (*fileChooseCtx).n64ddFlags[gSaveContext.fileNum as usize] as
                     libc::c_int);
    osSyncPrintf(b"heart_status[%d]=%d\n\x00" as *const u8 as
                     *const libc::c_char, gSaveContext.fileNum,
                 (*fileChooseCtx).defense[gSaveContext.fileNum as usize] as
                     libc::c_int);
    osSyncPrintf(b"now_life[%d]=%d\n\x00" as *const u8 as *const libc::c_char,
                 gSaveContext.fileNum,
                 (*fileChooseCtx).health[gSaveContext.fileNum as usize] as
                     libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn Sram_EraseSave(mut fileChooseCtx:
                                            *mut FileChooseContext,
                                        mut sramCtx: *mut SramContext) {
    let mut offset: s32 = 0;
    Sram_InitNewSave();
    offset =
        gSramSlotOffsets[(*fileChooseCtx).selectedFileIndex as usize] as s32;
    MemCopy((*sramCtx).readBuff.offset(offset as isize) as *mut libc::c_void,
            &mut gSaveContext as *mut SaveContext as *mut libc::c_void,
            ::std::mem::size_of::<Save>() as libc::c_ulong as s32);
    SsSram_ReadWrite(((0xa8000000 as libc::c_uint as
                           *mut libc::c_char).offset(-(0xa0000000 as
                                                           libc::c_uint as
                                                           isize)) as
                          u32_0).wrapping_add(offset as libc::c_uint),
                     &mut gSaveContext as *mut SaveContext as
                         *mut libc::c_void,
                     (::std::mem::size_of::<SaveContext>() as
                          libc::c_ulong).wrapping_add(0x28 as libc::c_int as
                                                          libc::c_uint) as
                         size_t, 1 as libc::c_int);
    MemCopy(&mut *(*fileChooseCtx).n64ddFlags.as_mut_ptr().offset((*fileChooseCtx).selectedFileIndex
                                                                      as
                                                                      isize)
                as *mut s16 as *mut libc::c_void,
            (*sramCtx).readBuff.offset(offset as
                                           isize).offset(&mut (*(0 as
                                                                     *mut SaveContext)).n64ddFlag
                                                             as *mut s16 as
                                                             size_t as isize)
                as *mut libc::c_void,
            ::std::mem::size_of::<s16>() as libc::c_ulong as s32);
    offset =
        gSramSlotOffsets[((*fileChooseCtx).selectedFileIndex as libc::c_int +
                              3 as libc::c_int) as usize] as s32;
    MemCopy((*sramCtx).readBuff.offset(offset as isize) as *mut libc::c_void,
            &mut gSaveContext as *mut SaveContext as *mut libc::c_void,
            ::std::mem::size_of::<Save>() as libc::c_ulong as s32);
    SsSram_ReadWrite(((0xa8000000 as libc::c_uint as
                           *mut libc::c_char).offset(-(0xa0000000 as
                                                           libc::c_uint as
                                                           isize)) as
                          u32_0).wrapping_add(offset as libc::c_uint),
                     &mut gSaveContext as *mut SaveContext as
                         *mut libc::c_void,
                     (::std::mem::size_of::<SaveContext>() as
                          libc::c_ulong).wrapping_add(0x28 as libc::c_int as
                                                          libc::c_uint) as
                         size_t, 1 as libc::c_int);
    osSyncPrintf(b"\xef\xbc\xa3\xef\xbc\xac\xef\xbc\xa5\xef\xbc\xa1\xef\xbc\xb2\xe7\xb5\x82\xe4\xba\x86\n\x00"
                     as *const u8 as *const libc::c_char);
}
#[no_mangle]
pub unsafe extern "C" fn Sram_CopySave(mut fileChooseCtx:
                                           *mut FileChooseContext,
                                       mut sramCtx: *mut SramContext) {
    let mut offset: s32 = 0;
    osSyncPrintf(b"\xef\xbc\xb2\xef\xbc\xa5\xef\xbc\xa1\xef\xbc\xa4=%d(%x)  \xef\xbc\xa3\xef\xbc\xaf\xef\xbc\xb0\xef\xbc\xb9=%d(%x)\n\x00"
                     as *const u8 as *const libc::c_char,
                 (*fileChooseCtx).selectedFileIndex as libc::c_int,
                 gSramSlotOffsets[(*fileChooseCtx).selectedFileIndex as usize]
                     as libc::c_int,
                 (*fileChooseCtx).copyDestFileIndex as libc::c_int,
                 gSramSlotOffsets[(*fileChooseCtx).copyDestFileIndex as usize]
                     as libc::c_int);
    offset =
        gSramSlotOffsets[(*fileChooseCtx).selectedFileIndex as usize] as s32;
    MemCopy(&mut gSaveContext as *mut SaveContext as *mut libc::c_void,
            (*sramCtx).readBuff.offset(offset as isize) as *mut libc::c_void,
            ::std::mem::size_of::<Save>() as libc::c_ulong as s32);
    offset =
        gSramSlotOffsets[(*fileChooseCtx).copyDestFileIndex as usize] as s32;
    MemCopy((*sramCtx).readBuff.offset(offset as isize) as *mut libc::c_void,
            &mut gSaveContext as *mut SaveContext as *mut libc::c_void,
            ::std::mem::size_of::<Save>() as libc::c_ulong as s32);
    offset =
        gSramSlotOffsets[((*fileChooseCtx).copyDestFileIndex as libc::c_int +
                              3 as libc::c_int) as usize] as s32;
    MemCopy((*sramCtx).readBuff.offset(offset as isize) as *mut libc::c_void,
            &mut gSaveContext as *mut SaveContext as *mut libc::c_void,
            ::std::mem::size_of::<Save>() as libc::c_ulong as s32);
    SsSram_ReadWrite((0xa8000000 as libc::c_uint as
                          *mut libc::c_char).offset(-(0xa0000000 as
                                                          libc::c_uint as
                                                          isize)) as u32_0,
                     (*sramCtx).readBuff as *mut libc::c_void,
                     0x8000 as libc::c_int as size_t, 1 as libc::c_int);
    offset =
        gSramSlotOffsets[(*fileChooseCtx).copyDestFileIndex as usize] as s32;
    MemCopy(&mut *(*fileChooseCtx).deaths.as_mut_ptr().offset((*fileChooseCtx).copyDestFileIndex
                                                                  as isize) as
                *mut u16_0 as *mut libc::c_void,
            (*sramCtx).readBuff.offset(offset as
                                           isize).offset(&mut (*(0 as
                                                                     *mut SaveContext)).deaths
                                                             as *mut u16_0 as
                                                             size_t as isize)
                as *mut libc::c_void,
            ::std::mem::size_of::<u16_0>() as libc::c_ulong as s32);
    MemCopy(&mut *(*fileChooseCtx).fileNames.as_mut_ptr().offset((*fileChooseCtx).copyDestFileIndex
                                                                     as isize)
                as *mut [u8_0; 8] as *mut libc::c_void,
            (*sramCtx).readBuff.offset(offset as
                                           isize).offset(&mut (*(0 as
                                                                     *mut SaveContext)).playerName
                                                             as
                                                             *mut [libc::c_char; 8]
                                                             as size_t as
                                                             isize) as
                *mut libc::c_void,
            ::std::mem::size_of::<[u8_0; 8]>() as libc::c_ulong as s32);
    MemCopy(&mut *(*fileChooseCtx).healthCapacities.as_mut_ptr().offset((*fileChooseCtx).copyDestFileIndex
                                                                            as
                                                                            isize)
                as *mut u16_0 as *mut libc::c_void,
            (*sramCtx).readBuff.offset(offset as
                                           isize).offset(&mut (*(0 as
                                                                     *mut SaveContext)).healthCapacity
                                                             as *mut s16 as
                                                             size_t as isize)
                as *mut libc::c_void,
            ::std::mem::size_of::<u16_0>() as libc::c_ulong as s32);
    MemCopy(&mut *(*fileChooseCtx).questItems.as_mut_ptr().offset((*fileChooseCtx).copyDestFileIndex
                                                                      as
                                                                      isize)
                as *mut u32_0 as *mut libc::c_void,
            (*sramCtx).readBuff.offset(offset as
                                           isize).offset(&mut (*(0 as
                                                                     *mut SaveContext)).inventory.questItems
                                                             as *mut u32_0 as
                                                             size_t as isize)
                as *mut libc::c_void,
            ::std::mem::size_of::<u32_0>() as libc::c_ulong as s32);
    MemCopy(&mut *(*fileChooseCtx).n64ddFlags.as_mut_ptr().offset((*fileChooseCtx).copyDestFileIndex
                                                                      as
                                                                      isize)
                as *mut s16 as *mut libc::c_void,
            (*sramCtx).readBuff.offset(offset as
                                           isize).offset(&mut (*(0 as
                                                                     *mut SaveContext)).n64ddFlag
                                                             as *mut s16 as
                                                             size_t as isize)
                as *mut libc::c_void,
            ::std::mem::size_of::<s16>() as libc::c_ulong as s32);
    MemCopy(&mut *(*fileChooseCtx).defense.as_mut_ptr().offset((*fileChooseCtx).copyDestFileIndex
                                                                   as isize)
                as *mut s8 as *mut libc::c_void,
            (*sramCtx).readBuff.offset(offset as
                                           isize).offset(&mut (*(0 as
                                                                     *mut SaveContext)).inventory.defenseHearts
                                                             as *mut s8 as
                                                             size_t as isize)
                as *mut libc::c_void,
            ::std::mem::size_of::<s8>() as libc::c_ulong as s32);
    MemCopy(&mut *(*fileChooseCtx).health.as_mut_ptr().offset((*fileChooseCtx).copyDestFileIndex
                                                                  as isize) as
                *mut u16_0 as *mut libc::c_void,
            (*sramCtx).readBuff.offset(offset as
                                           isize).offset(&mut (*(0 as
                                                                     *mut SaveContext)).health
                                                             as *mut s16 as
                                                             size_t as isize)
                as *mut libc::c_void,
            ::std::mem::size_of::<u16_0>() as libc::c_ulong as s32);
    osSyncPrintf(b"f_64dd[%d]=%d\n\x00" as *const u8 as *const libc::c_char,
                 gSaveContext.fileNum,
                 (*fileChooseCtx).n64ddFlags[gSaveContext.fileNum as usize] as
                     libc::c_int);
    osSyncPrintf(b"heart_status[%d]=%d\n\x00" as *const u8 as
                     *const libc::c_char, gSaveContext.fileNum,
                 (*fileChooseCtx).defense[gSaveContext.fileNum as usize] as
                     libc::c_int);
    osSyncPrintf(b"\xef\xbc\xa3\xef\xbc\xaf\xef\xbc\xb0\xef\xbc\xb9\xe7\xb5\x82\xe4\xba\x86\n\x00"
                     as *const u8 as *const libc::c_char);
    // "Copy end"
}
/* *
 *  Write the first 16 bytes of the read buffer to the SRAM header
 */
#[no_mangle]
pub unsafe extern "C" fn Sram_WriteSramHeader(mut sramCtx: *mut SramContext) {
    SsSram_ReadWrite((0xa8000000 as libc::c_uint as
                          *mut libc::c_char).offset(-(0xa0000000 as
                                                          libc::c_uint as
                                                          isize)) as u32_0,
                     (*sramCtx).readBuff as *mut libc::c_void,
                     0x10 as libc::c_int as size_t,
                     1 as libc::c_int); // "SRAM destruction! ! ! ! ! !"
}
#[no_mangle]
pub unsafe extern "C" fn Sram_InitSram(mut gameState: *mut GameState,
                                       mut sramCtx: *mut SramContext) {
    let mut i: u16_0 = 0;
    osSyncPrintf(b"sram_initialize( Game *game, Sram *sram )\n\x00" as
                     *const u8 as *const libc::c_char);
    SsSram_ReadWrite((0xa8000000 as libc::c_uint as
                          *mut libc::c_char).offset(-(0xa0000000 as
                                                          libc::c_uint as
                                                          isize)) as u32_0,
                     (*sramCtx).readBuff as *mut libc::c_void,
                     0x8000 as libc::c_int as size_t, 0 as libc::c_int);
    i = 0 as libc::c_int as u16_0;
    while (i as libc::c_uint) <
              (::std::mem::size_of::<[libc::c_char; 12]>() as
                   libc::c_ulong).wrapping_div(::std::mem::size_of::<libc::c_char>()
                                                   as
                                                   libc::c_ulong).wrapping_sub(3
                                                                                   as
                                                                                   libc::c_int
                                                                                   as
                                                                                   libc::c_uint)
          {
        if sZeldaMagic[(i as libc::c_int + SRAM_HEADER_MAGIC as libc::c_int)
                           as usize] as libc::c_int !=
               *(*sramCtx).readBuff.offset((i as libc::c_int +
                                                SRAM_HEADER_MAGIC as
                                                    libc::c_int) as isize) as
                   libc::c_int {
            osSyncPrintf(b"\xef\xbc\xb3\xef\xbc\xb2\xef\xbc\xa1\xef\xbc\xad\xe7\xa0\xb4\xe5\xa3\x8a\xef\xbc\x81\xef\xbc\x81\xef\xbc\x81\xef\xbc\x81\xef\xbc\x81\xef\xbc\x81\n\x00"
                             as *const u8 as *const libc::c_char);
            gSaveContext.language =
                *(*sramCtx).readBuff.offset(SRAM_HEADER_LANGUAGE as
                                                libc::c_int as isize);
            MemCopy((*sramCtx).readBuff as *mut libc::c_void,
                    sZeldaMagic.as_mut_ptr() as *mut libc::c_void,
                    ::std::mem::size_of::<[libc::c_char; 12]>() as
                        libc::c_ulong as s32);
            *(*sramCtx).readBuff.offset(SRAM_HEADER_LANGUAGE as libc::c_int as
                                            isize) = gSaveContext.language;
            Sram_WriteSramHeader(sramCtx);
        }
        i = i.wrapping_add(1)
    }
    gSaveContext.audioSetting =
        (*(*sramCtx).readBuff.offset(SRAM_HEADER_SOUND as libc::c_int as
                                         isize) as libc::c_int &
             3 as libc::c_int) as u8_0;
    gSaveContext.zTargetSetting =
        (*(*sramCtx).readBuff.offset(SRAM_HEADER_ZTARGET as libc::c_int as
                                         isize) as libc::c_int &
             1 as libc::c_int) as u8_0;
    gSaveContext.language =
        *(*sramCtx).readBuff.offset(SRAM_HEADER_LANGUAGE as libc::c_int as
                                        isize);
    if gSaveContext.language as libc::c_int >= LANGUAGE_MAX as libc::c_int {
        gSaveContext.language = LANGUAGE_ENG as libc::c_int as u8_0;
        *(*sramCtx).readBuff.offset(SRAM_HEADER_LANGUAGE as libc::c_int as
                                        isize) = gSaveContext.language;
        Sram_WriteSramHeader(sramCtx);
    }
    if (*gameState).input[2 as libc::c_int as usize].cur.button as libc::c_int
           & 0x100 as libc::c_int != 0 as libc::c_int {
        bzero((*sramCtx).readBuff as *mut libc::c_void,
              0x8000 as libc::c_int as u32_0);
        i = 0 as libc::c_int as u16_0;
        while (i as libc::c_uint) <
                  (::std::mem::size_of::<Save>() as
                       libc::c_ulong).wrapping_div(2 as libc::c_int as
                                                       libc::c_uint) {
            *(*sramCtx).readBuff.offset(i as isize) = i as u8_0;
            i = i.wrapping_add(1)
        }
        SsSram_ReadWrite((0xa8000000 as libc::c_uint as
                              *mut libc::c_char).offset(-(0xa0000000 as
                                                              libc::c_uint as
                                                              isize)) as
                             u32_0, (*sramCtx).readBuff as *mut libc::c_void,
                         0x8000 as libc::c_int as size_t, 1 as libc::c_int);
        osSyncPrintf(b"\xef\xbc\xb3\xef\xbc\xb2\xef\xbc\xa1\xef\xbc\xad\xe7\xa0\xb4\xe5\xa3\x8a\xef\xbc\x81\xef\xbc\x81\xef\xbc\x81\xef\xbc\x81\xef\xbc\x81\xef\xbc\x81\n\x00"
                         as *const u8 as *const libc::c_char);
        // "SRAM destruction! ! ! ! ! !"
    }
    // "GOOD! GOOD! Size = %d + %d = %d"
    osSyncPrintf(b"\xef\xbc\xa7\xef\xbc\xaf\xef\xbc\xaf\xef\xbc\xa4\xef\xbc\x81\xef\xbc\xa7\xef\xbc\xaf\xef\xbc\xaf\xef\xbc\xa4\xef\xbc\x81 \xe3\x82\xb5\xe3\x82\xa4\xe3\x82\xba\xef\xbc\x9d%d + %d \xef\xbc\x9d %d\n\x00"
                     as *const u8 as *const libc::c_char,
                 ::std::mem::size_of::<SaveInfo>() as libc::c_ulong,
                 4 as libc::c_int,
                 (::std::mem::size_of::<SaveInfo>() as
                      libc::c_ulong).wrapping_add(4 as libc::c_int as
                                                      libc::c_uint));
    osSyncPrintf(b"\x1b[34m\x00" as *const u8 as *const libc::c_char);
    osSyncPrintf(b"Na_SetSoundOutputMode = %d\n\x00" as *const u8 as
                     *const libc::c_char,
                 gSaveContext.audioSetting as libc::c_int);
    osSyncPrintf(b"Na_SetSoundOutputMode = %d\n\x00" as *const u8 as
                     *const libc::c_char,
                 gSaveContext.audioSetting as libc::c_int);
    osSyncPrintf(b"Na_SetSoundOutputMode = %d\n\x00" as *const u8 as
                     *const libc::c_char,
                 gSaveContext.audioSetting as libc::c_int);
    osSyncPrintf(b"\x1b[m\x00" as *const u8 as *const libc::c_char);
    func_800F6700(gSaveContext.audioSetting as s8);
}
#[no_mangle]
pub unsafe extern "C" fn Sram_Alloc(mut gameState: *mut GameState,
                                    mut sramCtx: *mut SramContext) {
    (*sramCtx).readBuff =
        GameState_Alloc(gameState, 0x8000 as libc::c_int as size_t,
                        b"../z_sram.c\x00" as *const u8 as *const libc::c_char
                            as *mut libc::c_char, 1294 as libc::c_int) as
            *mut u8_0;
    if !(*sramCtx).readBuff.is_null() {
    } else {
        __assert(b"sram->read_buff != NULL\x00" as *const u8 as
                     *const libc::c_char,
                 b"../z_sram.c\x00" as *const u8 as *const libc::c_char,
                 1295 as libc::c_int);
    };
}
#[no_mangle]
pub unsafe extern "C" fn Sram_Init(mut globalCtx: *mut GlobalContext,
                                   mut sramCtx: *mut SramContext) {
}
unsafe extern "C" fn run_static_initializers() {
    gSramSlotOffsets =
        [((0x10 as libc::c_int + 0x10 as libc::c_int) as
              libc::c_uint).wrapping_add((0 as libc::c_int as
                                              libc::c_uint).wrapping_mul((::std::mem::size_of::<SaveContext>()
                                                                              as
                                                                              libc::c_ulong).wrapping_add(0x28
                                                                                                              as
                                                                                                              libc::c_int
                                                                                                              as
                                                                                                              libc::c_uint)))
             as u16_0,
         ((0x10 as libc::c_int + 0x10 as libc::c_int) as
              libc::c_uint).wrapping_add((1 as libc::c_int as
                                              libc::c_uint).wrapping_mul((::std::mem::size_of::<SaveContext>()
                                                                              as
                                                                              libc::c_ulong).wrapping_add(0x28
                                                                                                              as
                                                                                                              libc::c_int
                                                                                                              as
                                                                                                              libc::c_uint)))
             as u16_0,
         ((0x10 as libc::c_int + 0x10 as libc::c_int) as
              libc::c_uint).wrapping_add((2 as libc::c_int as
                                              libc::c_uint).wrapping_mul((::std::mem::size_of::<SaveContext>()
                                                                              as
                                                                              libc::c_ulong).wrapping_add(0x28
                                                                                                              as
                                                                                                              libc::c_int
                                                                                                              as
                                                                                                              libc::c_uint)))
             as u16_0,
         ((0x10 as libc::c_int + 0x10 as libc::c_int) as
              libc::c_uint).wrapping_add((3 as libc::c_int as
                                              libc::c_uint).wrapping_mul((::std::mem::size_of::<SaveContext>()
                                                                              as
                                                                              libc::c_ulong).wrapping_add(0x28
                                                                                                              as
                                                                                                              libc::c_int
                                                                                                              as
                                                                                                              libc::c_uint)))
             as u16_0,
         ((0x10 as libc::c_int + 0x10 as libc::c_int) as
              libc::c_uint).wrapping_add((4 as libc::c_int as
                                              libc::c_uint).wrapping_mul((::std::mem::size_of::<SaveContext>()
                                                                              as
                                                                              libc::c_ulong).wrapping_add(0x28
                                                                                                              as
                                                                                                              libc::c_int
                                                                                                              as
                                                                                                              libc::c_uint)))
             as u16_0,
         ((0x10 as libc::c_int + 0x10 as libc::c_int) as
              libc::c_uint).wrapping_add((5 as libc::c_int as
                                              libc::c_uint).wrapping_mul((::std::mem::size_of::<SaveContext>()
                                                                              as
                                                                              libc::c_ulong).wrapping_add(0x28
                                                                                                              as
                                                                                                              libc::c_int
                                                                                                              as
                                                                                                              libc::c_uint)))
             as u16_0]
}
#[used]
#[cfg_attr(target_os = "linux", link_section = ".init_array")]
#[cfg_attr(target_os = "windows", link_section = ".CRT$XIB")]
#[cfg_attr(target_os = "macos", link_section = "__DATA,__mod_init_func")]
static INIT_ARRAY: [unsafe extern "C" fn(); 1] = [run_static_initializers];
