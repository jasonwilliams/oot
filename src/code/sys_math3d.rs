#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case,
         non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(register_tool)]
extern "C" {
    #[no_mangle]
    fn fabsf(f: f32_0) -> f32_0;
    #[no_mangle]
    fn sqrtf(f: f32_0) -> f32_0;
    #[no_mangle]
    fn sqrt(d: f64_0) -> f64_0;
    #[no_mangle]
    fn osSyncPrintf(fmt: *const libc::c_char, _: ...);
    #[no_mangle]
    fn Math_CosS(angle: s16) -> f32_0;
    #[no_mangle]
    fn Math_SinS(angle: s16) -> f32_0;
    #[no_mangle]
    fn Math_Vec3f_Copy(dest: *mut Vec3f, src: *mut Vec3f);
    #[no_mangle]
    fn Math_Vec3s_ToVec3f(dest: *mut Vec3f, src: *mut Vec3s);
    #[no_mangle]
    fn Math_Vec3f_Diff(a: *mut Vec3f, b: *mut Vec3f, dest: *mut Vec3f);
    #[no_mangle]
    fn Math_Vec3f_DistXYZ(a: *mut Vec3f, b: *mut Vec3f) -> f32_0;
}
pub type s8 = libc::c_schar;
pub type u8_0 = libc::c_uchar;
pub type s16 = libc::c_short;
pub type u16_0 = libc::c_ushort;
pub type s32 = libc::c_int;
pub type u32_0 = libc::c_uint;
pub type u64_0 = libc::c_ulonglong;
pub type f32_0 = libc::c_float;
pub type f64_0 = libc::c_double;
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
pub struct Spheref {
    pub center: Vec3f,
    pub radius: f32_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Plane {
    pub normal: Vec3f,
    pub originDist: f32_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct TriNorm {
    pub vtx: [Vec3f; 3],
    pub plane: Plane,
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
pub struct Cylinderf {
    pub radius: f32_0,
    pub height: f32_0,
    pub yShift: f32_0,
    pub pos: Vec3f,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct InfiniteLine {
    pub point: Vec3f,
    pub dir: Vec3f,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Linef {
    pub a: Vec3f,
    pub b: Vec3f,
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
/* *
 * Creates an infinite line along the intersection of the plane defined from `planeAA`x + `planeAB`y + `planeAB`z +
 * `planeADist` = 0 and `planeBA`x + `planeBB`y + `planeBC`z + `planeBDist` = 0, and finds the closest point on that
 * intersection to the line segment `linePointA and linePointB`, outputs the intersection to `closestPoint`
 */
#[no_mangle]
pub unsafe extern "C" fn Math3D_PlaneVsLineSegClosestPoint(mut planeAA: f32_0,
                                                           mut planeAB: f32_0,
                                                           mut planeAC: f32_0,
                                                           mut planeADist:
                                                               f32_0,
                                                           mut planeBA: f32_0,
                                                           mut planeBB: f32_0,
                                                           mut planeBC: f32_0,
                                                           mut planeBDist:
                                                               f32_0,
                                                           mut linePointA:
                                                               *mut Vec3f,
                                                           mut linePointB:
                                                               *mut Vec3f,
                                                           mut closestPoint:
                                                               *mut Vec3f)
 -> s32 {
    static mut planeIntersectLine: InfiniteLine =
        InfiniteLine{point: Vec3f{x: 0., y: 0., z: 0.,},
                     dir: Vec3f{x: 0., y: 0., z: 0.,},}; // unused
    static mut planeIntersectSeg: Linef =
        Linef{a: Vec3f{x: 0., y: 0., z: 0.,},
              b: Vec3f{x: 0., y: 0., z: 0.,},};
    let mut sp34: Vec3f = Vec3f{x: 0., y: 0., z: 0.,};
    if Math3D_PlaneVsPlaneNewLine(planeAA, planeAB, planeAC, planeADist,
                                  planeBA, planeBB, planeBC, planeBDist,
                                  &mut planeIntersectLine) == 0 {
        // The planes are parallel
        return 0 as libc::c_int
    }
    // create a line segment on the plane.
    Math_Vec3f_Copy(&mut planeIntersectSeg.a, &mut planeIntersectLine.point);
    planeIntersectSeg.b.x =
        planeIntersectLine.dir.x * 100.0f32 + planeIntersectLine.point.x;
    planeIntersectSeg.b.y =
        planeIntersectLine.dir.y * 100.0f32 + planeIntersectLine.point.y;
    planeIntersectSeg.b.z =
        planeIntersectLine.dir.z * 100.0f32 + planeIntersectLine.point.z;
    // closestPoint is a point on planeIntersect, sp34 is a point on linePointA, linePointB
    if Math3D_LineVsLineClosestTwoPoints(&mut planeIntersectSeg.a,
                                         &mut planeIntersectSeg.b, linePointA,
                                         linePointB, closestPoint, &mut sp34)
           == 0 {
        return 0 as libc::c_int
    }
    return 1 as libc::c_int;
}
/* *
 * Finds the two points on lines A and B where the lines are closest together.
 */
#[no_mangle]
pub unsafe extern "C" fn Math3D_LineVsLineClosestTwoPoints(mut lineAPointA:
                                                               *mut Vec3f,
                                                           mut lineAPointB:
                                                               *mut Vec3f,
                                                           mut lineBPointA:
                                                               *mut Vec3f,
                                                           mut lineBPointB:
                                                               *mut Vec3f,
                                                           mut lineAClosestToB:
                                                               *mut Vec3f,
                                                           mut lineBClosestToA:
                                                               *mut Vec3f)
 -> s32 {
    let mut sqMag: f32_0 = 0.;
    let mut scaleB: f32_0 = 0.;
    let mut lineAx: f32_0 = 0.;
    let mut lineAy: f32_0 = 0.;
    let mut lineAz: f32_0 = 0.;
    let mut lineBx: f32_0 = 0.;
    let mut lineBy: f32_0 = 0.;
    let mut lineBz: f32_0 = 0.;
    let mut compAAlongB: f32_0 = 0.;
    let mut compBAAlongB: f32_0 = 0.;
    let mut lineAPerpB: Vec3f = Vec3f{x: 0., y: 0., z: 0.,};
    let mut lineBAPerpB: Vec3f = Vec3f{x: 0., y: 0., z: 0.,};
    let mut tA: f32_0 = 0.;
    let mut tB: f32_0 = 0.;
    lineAx = (*lineAPointB).x - (*lineAPointA).x;
    lineAy = (*lineAPointB).y - (*lineAPointA).y;
    lineAz = (*lineAPointB).z - (*lineAPointA).z;
    lineBx = (*lineBPointB).x - (*lineBPointA).x;
    lineBy = (*lineBPointB).y - (*lineBPointA).y;
    lineBz = (*lineBPointB).z - (*lineBPointA).z;
    sqMag = lineBx * lineBx + lineBy * lineBy + lineBz * lineBz;
    if fabsf(sqMag) < 0.008f32 { return 0 as libc::c_int }
    scaleB = 1.0f32 / sqMag;
    compAAlongB =
        (lineAx * lineBx + lineAy * lineBy + lineAz * lineBz) * scaleB;
    compBAAlongB =
        (lineBx * ((*lineAPointA).x - (*lineBPointA).x) +
             lineBy * ((*lineAPointA).y - (*lineBPointA).y) +
             lineBz * ((*lineAPointA).z - (*lineBPointA).z)) * scaleB;
    lineAPerpB.x = lineAx - lineBx * compAAlongB;
    lineAPerpB.y = lineAy - lineBy * compAAlongB;
    lineAPerpB.z = lineAz - lineBz * compAAlongB;
    sqMag =
        lineAPerpB.x * lineAPerpB.x + lineAPerpB.y * lineAPerpB.y +
            lineAPerpB.z * lineAPerpB.z;
    if fabsf(sqMag) < 0.008f32 { return 0 as libc::c_int }
    lineBAPerpB.x =
        (*lineAPointA).x - (*lineBPointA).x - lineBx * compBAAlongB;
    lineBAPerpB.y =
        (*lineAPointA).y - (*lineBPointA).y - lineBy * compBAAlongB;
    lineBAPerpB.z =
        (*lineAPointA).z - (*lineBPointA).z - lineBz * compBAAlongB;
    tA =
        -(lineAPerpB.x * lineBAPerpB.x + lineAPerpB.y * lineBAPerpB.y +
              lineAPerpB.z * lineBAPerpB.z) / sqMag;
    (*lineAClosestToB).x = lineAx * tA + (*lineAPointA).x;
    (*lineAClosestToB).y = lineAy * tA + (*lineAPointA).y;
    (*lineAClosestToB).z = lineAz * tA + (*lineAPointA).z;
    tB = compAAlongB * tA + compBAAlongB;
    (*lineBClosestToA).x = lineBx * tB + (*lineBPointA).x;
    (*lineBClosestToA).y = lineBy * tB + (*lineBPointA).y;
    (*lineBClosestToA).z = lineBz * tB + (*lineBPointA).z;
    return 1 as libc::c_int;
}
/* *
 * Determines the closest point on the line `line` to `pos`, by forming a line perpendicular from
 * `point` to `line` closest point is placed in `closestPoint`
 */
#[no_mangle]
pub unsafe extern "C" fn Math3D_LineClosestToPoint(mut line: *mut Linef,
                                                   mut pos: *mut Vec3f,
                                                   mut closestPoint:
                                                       *mut Vec3f) {
    let mut dirVectorSize: f32_0 = 0.;
    let mut t: f32_0 = 0.;
    dirVectorSize = Math3D_Vec3fMagnitudeSq(&mut (*line).b);
    if fabsf(dirVectorSize) < 0.008f32 {
        osSyncPrintf(b"\x1b[43;30m\x00" as *const u8 as *const libc::c_char);
        // "Math3D_lineVsPosSuisenCross(): No straight line length"
        osSyncPrintf(b"Math3D_lineVsPosSuisenCross():\xe7\x9b\xb4\xe7\xb7\x9a\xe3\x81\xae\xe9\x95\xb7\xe3\x81\x95\xe3\x81\x8c\xe3\x81\x82\xe3\x82\x8a\xe3\x81\xbe\xe3\x81\x9b\xe3\x82\x93\n\x00"
                         as *const u8 as
                         *const libc::c_char); // "Returns cross = pos."
        osSyncPrintf(b"cross = pos \xe3\x82\x92\xe8\xbf\x94\xe3\x81\x97\xe3\x81\xbe\xe3\x81\x99\xe3\x80\x82\n\x00"
                         as *const u8 as *const libc::c_char);
        osSyncPrintf(b"\x1b[m\x00" as *const u8 as *const libc::c_char);
        Math_Vec3f_Copy(closestPoint, pos);
    }
    t =
        (((*pos).x - (*line).a.x) * (*line).b.x +
             ((*pos).y - (*line).a.y) * (*line).b.y +
             ((*pos).z - (*line).a.z) * (*line).b.z) / dirVectorSize;
    (*closestPoint).x = (*line).b.x * t + (*line).a.x;
    (*closestPoint).y = (*line).b.y * t + (*line).a.y;
    (*closestPoint).z = (*line).b.z * t + (*line).a.z;
}
#[no_mangle]
pub unsafe extern "C" fn Math3D_FindPointOnPlaneIntersect(mut planeAAxis1Norm:
                                                              f32_0,
                                                          mut planeAAxis2Norm:
                                                              f32_0,
                                                          mut planeBAxis1Norm:
                                                              f32_0,
                                                          mut planeBAxis2Norm:
                                                              f32_0,
                                                          mut axis3Direction:
                                                              f32_0,
                                                          mut planeADist:
                                                              f32_0,
                                                          mut planeBDist:
                                                              f32_0,
                                                          mut axis1Point:
                                                              *mut f32_0,
                                                          mut axis2Point:
                                                              *mut f32_0) {
    *axis1Point =
        (planeAAxis2Norm * planeBDist - planeBAxis2Norm * planeADist) /
            axis3Direction;
    *axis2Point =
        (planeBAxis1Norm * planeADist - planeAAxis1Norm * planeBDist) /
            axis3Direction;
}
/* *
 * Creates a line between the intersections of two planes defined from `planeAA`x + `planeAB`y + `planeAC`z +
 * `planeADist` = 0 and `planeBA`x + `planeBB`y + `planeBC`z + `planeBDist` = 0, and outputs the line to `intersect`.
 * Returns false if the planes are parallel.
 */
#[no_mangle]
pub unsafe extern "C" fn Math3D_PlaneVsPlaneNewLine(mut planeAA: f32_0,
                                                    mut planeAB: f32_0,
                                                    mut planeAC: f32_0,
                                                    mut planeADist: f32_0,
                                                    mut planeBA: f32_0,
                                                    mut planeBB: f32_0,
                                                    mut planeBC: f32_0,
                                                    mut planeBDist: f32_0,
                                                    mut intersect:
                                                        *mut InfiniteLine)
 -> s32 {
    let mut pad: [libc::c_char; 4] = [0; 4];
    let mut planeANormal: Vec3f = Vec3f{x: 0., y: 0., z: 0.,};
    let mut planeBNormal: Vec3f = Vec3f{x: 0., y: 0., z: 0.,};
    let mut dirX: f32_0 = 0.;
    let mut dirY: f32_0 = 0.;
    let mut dirZ: f32_0 = 0.;
    planeANormal.x = planeAA;
    planeANormal.y = planeAB;
    planeANormal.z = planeAC;
    planeBNormal.x = planeBA;
    planeBNormal.y = planeBB;
    planeBNormal.z = planeBC;
    Math3D_Vec3f_Cross(&mut planeANormal, &mut planeBNormal,
                       &mut (*intersect).dir);
    if fabsf((*intersect).dir.x) < 0.008f32 &&
           fabsf((*intersect).dir.y) < 0.008f32 &&
           fabsf((*intersect).dir.z) < 0.008f32 {
        // planes are parallel
        return 0 as libc::c_int
    }
    dirX = fabsf((*intersect).dir.x);
    dirY = fabsf((*intersect).dir.y);
    dirZ = fabsf((*intersect).dir.z);
    if dirX >= dirY && dirX >= dirZ {
        Math3D_FindPointOnPlaneIntersect(planeAB, planeAC, planeBB, planeBC,
                                         (*intersect).dir.x, planeADist,
                                         planeBDist,
                                         &mut (*intersect).point.y,
                                         &mut (*intersect).point.z);
        (*intersect).point.x = 0.0f32
    } else if dirY >= dirX && dirY >= dirZ {
        Math3D_FindPointOnPlaneIntersect(planeAC, planeAA, planeBC, planeBA,
                                         (*intersect).dir.y, planeADist,
                                         planeBDist,
                                         &mut (*intersect).point.z,
                                         &mut (*intersect).point.x);
        (*intersect).point.y = 0.0f32
    } else {
        Math3D_FindPointOnPlaneIntersect(planeAA, planeAB, planeBA, planeBB,
                                         (*intersect).dir.z, planeADist,
                                         planeBDist,
                                         &mut (*intersect).point.x,
                                         &mut (*intersect).point.y);
        (*intersect).point.z = 0.0f32
    }
    return 1 as libc::c_int;
}
/* *
 * Gets the closest point on the line formed from the intersection of of the planes defined from
 * `planeAA`x + `planeAB`y + `planeAC`z + `planeADist` = 0 and
 * `planeBA`x + `planeBB`y + `planeBC`z + `planeBDist` = 0
 * the point on the intersection line closest to `point` is placed in `closestPoint`
 * returns false if the planes are parallel.
 */
#[no_mangle]
pub unsafe extern "C" fn Math3D_PlaneVsPlaneVsLineClosestPoint(mut planeAA:
                                                                   f32_0,
                                                               mut planeAB:
                                                                   f32_0,
                                                               mut planeAC:
                                                                   f32_0,
                                                               mut planeADist:
                                                                   f32_0,
                                                               mut planeBA:
                                                                   f32_0,
                                                               mut planeBB:
                                                                   f32_0,
                                                               mut planeBC:
                                                                   f32_0,
                                                               mut planeBDist:
                                                                   f32_0,
                                                               mut point:
                                                                   *mut Vec3f,
                                                               mut closestPoint:
                                                                   *mut Vec3f)
 -> s32 {
    static mut planeIntersect: Linef =
        Linef{a: Vec3f{x: 0., y: 0., z: 0.,},
              b: Vec3f{x: 0., y: 0., z: 0.,},};
    if Math3D_PlaneVsPlaneNewLine(planeAA, planeAB, planeAC, planeADist,
                                  planeBA, planeBB, planeBC, planeBDist,
                                  &mut planeIntersect as *mut Linef as
                                      *mut InfiniteLine) == 0 {
        return 0 as libc::c_int
    }
    Math3D_LineClosestToPoint(&mut planeIntersect, point, closestPoint);
    return 1 as libc::c_int;
}
/* *
 * Finds a point on the line from starting point `v0`, and directional vector `dir`
 * which is `dist` length from the starting point.  Result is placed in `ret`
 */
#[no_mangle]
pub unsafe extern "C" fn Math3D_PointOnInfiniteLine(mut v0: *mut Vec3f,
                                                    mut dir: *mut Vec3f,
                                                    mut dist: f32_0,
                                                    mut ret: *mut Vec3f) {
    (*ret).x = (*dir).x * dist + (*v0).x;
    (*ret).y = (*dir).y * dist + (*v0).y;
    (*ret).z = (*dir).z * dist + (*v0).z;
}
/* *
 * Splits the line segment from end points `v0` and `v1`, and splits that segment
 * by `ratio` of `v0`:`v1`, places the resulting point on the line in `ret`
 */
#[no_mangle]
pub unsafe extern "C" fn Math3D_LineSplitRatio(mut v0: *mut Vec3f,
                                               mut v1: *mut Vec3f,
                                               mut ratio: f32_0,
                                               mut ret: *mut Vec3f) {
    let mut diff: Vec3f = Vec3f{x: 0., y: 0., z: 0.,};
    Math_Vec3f_Diff(v1, v0, &mut diff);
    Math3D_PointOnInfiniteLine(v0, &mut diff, ratio, ret);
}
/* *
 * Calculates the cosine between vectors `a` and `b`
 */
#[no_mangle]
pub unsafe extern "C" fn Math3D_Cos(mut a: *mut Vec3f, mut b: *mut Vec3f)
 -> f32_0 {
    let mut ret: f32_0 = 0.;
    Math3D_CosOut(a, b, &mut ret);
    return ret;
}
/* *
 * Calculates the cosine between bectors `a` and `b` and places the result in `ret`
 * returns true if the cosine cannot be calculated because the product of the magnitudes is zero
 */
#[no_mangle]
pub unsafe extern "C" fn Math3D_CosOut(mut a: *mut Vec3f, mut b: *mut Vec3f,
                                       mut dst: *mut f32_0) -> s32 {
    let mut magProduct: f32_0 = 0.;
    magProduct = Math3D_Vec3fMagnitude(a) * Math3D_Vec3fMagnitude(b);
    if fabsf(magProduct) < 0.008f32 { *dst = 0.0f32; return 1 as libc::c_int }
    *dst = ((*a).x * (*b).x + (*a).y * (*b).y + (*a).z * (*b).z) / magProduct;
    return 0 as libc::c_int;
}
/* *
 * Reflects vector `vec` across the normal vector `normal`, reflection vector is placed in
 * `reflVec`
 */
#[no_mangle]
pub unsafe extern "C" fn Math3D_Vec3fReflect(mut vec: *mut Vec3f,
                                             mut normal: *mut Vec3f,
                                             mut reflVec: *mut Vec3f) {
    let mut normScaleY: f32_0 = 0.;
    let mut negVec: Vec3f = Vec3f{x: 0., y: 0., z: 0.,};
    let mut normScaleZ: f32_0 = 0.;
    let mut normScaleX: f32_0 = 0.;
    let mut vecDotNorm: f32_0 = 0.;
    negVec.x = (*vec).x * -1.0f32;
    negVec.y = (*vec).y * -1.0f32;
    negVec.z = (*vec).z * -1.0f32;
    vecDotNorm = Math3D_Cos(&mut negVec, normal);
    normScaleX = (*normal).x * vecDotNorm;
    normScaleY = (*normal).y * vecDotNorm;
    normScaleZ = (*normal).z * vecDotNorm;
    (*reflVec).x = normScaleX + (*vec).x + (normScaleX + (*vec).x) + negVec.x;
    (*reflVec).y = normScaleY + (*vec).y + (normScaleY + (*vec).y) + negVec.y;
    (*reflVec).z = normScaleZ + (*vec).z + (normScaleZ + (*vec).z) + negVec.z;
}
/* *
 * Checks if the point (`x`,`y`) is contained within the square formed from (`upperLeftX`,`upperLeftY`) to
 * (`lowerRightX`,`lowerRightY`)
 */
#[no_mangle]
pub unsafe extern "C" fn Math3D_PointInSquare2D(mut upperLeftX: f32_0,
                                                mut lowerRightX: f32_0,
                                                mut upperLeftY: f32_0,
                                                mut lowerRightY: f32_0,
                                                mut x: f32_0, mut y: f32_0)
 -> s32 {
    if x >= upperLeftX && x <= lowerRightX && y >= upperLeftY &&
           y <= lowerRightY {
        return 1 as libc::c_int
    }
    return 0 as libc::c_int;
}
/* *
 * Checks if the square formed around the circle with center (`centerX`,`centerY`) with radius `radius`
 * touches any portion of the square formed around the triangle with vertices (`x0`,`y0`), (`x1`,`y1`),
 * and (`x2`,`y2`)
 */
#[no_mangle]
pub unsafe extern "C" fn Math3D_CirSquareVsTriSquare(mut x0: f32_0,
                                                     mut y0: f32_0,
                                                     mut x1: f32_0,
                                                     mut y1: f32_0,
                                                     mut x2: f32_0,
                                                     mut y2: f32_0,
                                                     mut centerX: f32_0,
                                                     mut centerY: f32_0,
                                                     mut radius: f32_0)
 -> s32 {
    let mut minX: f32_0 = 0.;
    let mut maxX: f32_0 = 0.;
    let mut minY: f32_0 = 0.;
    let mut maxY: f32_0 = 0.;
    maxX = x0;
    minX = maxX;
    maxY = y0;
    minY = maxY;
    if x1 < minX { minX = x1 } else if maxX < x1 { maxX = x1 }
    if y1 < minY { minY = y1 } else if maxY < y1 { maxY = y1 }
    if x2 < minX { minX = x2 } else if maxX < x2 { maxX = x2 }
    if y2 < minY { minY = y2 } else if maxY < y2 { maxY = y2 }
    if minX - radius <= centerX && maxX + radius >= centerX &&
           minY - radius <= centerY && maxY + radius >= centerY {
        return 1 as libc::c_int
    }
    return 0 as libc::c_int;
}
/* *
 * Checks if the cube formed around the triangle formed from `v0`, `v1`, and `v2`
 * has any portion touching the cube formed around the sphere with center `center`
 * and radius of `radius`
 */
#[no_mangle]
pub unsafe extern "C" fn Math3D_SphCubeVsTriCube(mut v0: *mut Vec3f,
                                                 mut v1: *mut Vec3f,
                                                 mut v2: *mut Vec3f,
                                                 mut center: *mut Vec3f,
                                                 mut radius: f32_0) -> s32 {
    let mut minX: f32_0 = 0.;
    let mut maxX: f32_0 = 0.;
    let mut minY: f32_0 = 0.;
    let mut maxY: f32_0 = 0.;
    let mut minZ: f32_0 = 0.;
    let mut maxZ: f32_0 = 0.;
    maxX = (*v0).x;
    minX = maxX;
    maxY = (*v0).y;
    minY = maxY;
    maxZ = (*v0).z;
    minZ = maxZ;
    if (*v1).x < minX {
        minX = (*v1).x
    } else if maxX < (*v1).x { maxX = (*v1).x }
    if (*v1).y < minY {
        minY = (*v1).y
    } else if maxY < (*v1).y { maxY = (*v1).y }
    if (*v1).z < minZ {
        minZ = (*v1).z
    } else if maxZ < (*v1).z { maxZ = (*v1).z }
    if (*v2).x < minX {
        minX = (*v2).x
    } else if maxX < (*v2).x { maxX = (*v2).x }
    if (*v2).y < minY {
        minY = (*v2).y
    } else if maxY < (*v2).y { maxY = (*v2).y }
    if (*v2).z < minZ {
        minZ = (*v2).z
    } else if maxZ < (*v2).z { maxZ = (*v2).z }
    if (*center).x >= minX - radius && (*center).x <= maxX + radius &&
           (*center).y >= minY - radius && (*center).y <= maxY + radius &&
           (*center).z >= minZ - radius && (*center).z <= maxZ + radius {
        return 1 as libc::c_int
    }
    return 0 as libc::c_int;
}
/* *
 * Returns the distance squared between `a` and `b` on a single axis
 */
#[no_mangle]
pub unsafe extern "C" fn Math3D_Dist1DSq(mut a: f32_0, mut b: f32_0)
 -> f32_0 {
    return a * a + b * b;
}
/* *
 * Returns the distance between `a` and `b` on a single axis
 */
#[no_mangle]
pub unsafe extern "C" fn Math3D_Dist1D(mut a: f32_0, mut b: f32_0) -> f32_0 {
    return sqrtf(Math3D_Dist1DSq(a, b));
}
/* *
 * Returns the distance squared between (`x0`,`y0`) and (`x1`,`x2`)
 */
#[no_mangle]
pub unsafe extern "C" fn Math3D_Dist2DSq(mut x0: f32_0, mut y0: f32_0,
                                         mut x1: f32_0, mut y1: f32_0)
 -> f32_0 {
    return Math3D_Dist1DSq(x0 - x1, y0 - y1);
}
/* *
 * Returns the distance between points (`x0`,`y0`) and (`x1`,`y1`)
 */
#[no_mangle]
pub unsafe extern "C" fn Math3D_Dist2D(mut x0: f32_0, mut y0: f32_0,
                                       mut x1: f32_0, mut y1: f32_0)
 -> f32_0 {
    return sqrtf(Math3D_Dist2DSq(x0, y0, x1, y1));
}
/* *
 * Returns the magntiude (length) squared of `vec`
 */
#[no_mangle]
pub unsafe extern "C" fn Math3D_Vec3fMagnitudeSq(mut vec: *mut Vec3f)
 -> f32_0 {
    return (*vec).x * (*vec).x + (*vec).y * (*vec).y + (*vec).z * (*vec).z;
}
/* *
 * Returns the magnitude(length) of `vec`
 */
#[no_mangle]
pub unsafe extern "C" fn Math3D_Vec3fMagnitude(mut vec: *mut Vec3f) -> f32_0 {
    return sqrt(Math3D_Vec3fMagnitudeSq(vec) as f64_0) as f32_0;
}
/* *
 * Returns the distance between `a` and `b` squared.
 */
#[no_mangle]
pub unsafe extern "C" fn Math3D_Vec3fDistSq(mut a: *mut Vec3f,
                                            mut b: *mut Vec3f) -> f32_0 {
    let mut diff: Vec3f = Vec3f{x: 0., y: 0., z: 0.,};
    Math_Vec3f_Diff(a, b, &mut diff);
    return Math3D_Vec3fMagnitudeSq(&mut diff);
}
/*
 * Calculates the distance between points `a` and `b`
 */
#[no_mangle]
pub unsafe extern "C" fn Math3D_Vec3f_DistXYZ(mut a: *mut Vec3f,
                                              mut b: *mut Vec3f) -> f32_0 {
    return Math_Vec3f_DistXYZ(a, b);
}
/*
 * Calculates the distance between `a` and `b`.
 */
#[no_mangle]
pub unsafe extern "C" fn Math3D_DistXYZ16toF(mut a: *mut Vec3s,
                                             mut b: *mut Vec3f) -> f32_0 {
    let mut diff: Vec3f = Vec3f{x: 0., y: 0., z: 0.,};
    diff.x = (*a).x as libc::c_int as libc::c_float - (*b).x;
    diff.y = (*a).y as libc::c_int as libc::c_float - (*b).y;
    diff.z = (*a).z as libc::c_int as libc::c_float - (*b).z;
    return Math3D_Vec3fMagnitude(&mut diff);
}
/* *
 * Gets the Z portion of the cross product of vectors `a - (`dx`,`dy`,z) and `b` - (`dx`,`dy`,z)
 */
#[no_mangle]
pub unsafe extern "C" fn Math3D_Vec3fDiff_CrossZ(mut a: *mut Vec3f,
                                                 mut b: *mut Vec3f,
                                                 mut dx: f32_0, mut dy: f32_0)
 -> f32_0 {
    return ((*a).x - dx) * ((*b).y - dy) - ((*a).y - dy) * ((*b).x - dx);
}
/* *
 * Gets the X portion of the cross product of vectors `a - (x,`dy`,`dz`) and `b` - (x,`dy`,`dz`)
 */
#[no_mangle]
pub unsafe extern "C" fn Math3D_Vec3fDiff_CrossX(mut a: *mut Vec3f,
                                                 mut b: *mut Vec3f,
                                                 mut dy: f32_0, mut dz: f32_0)
 -> f32_0 {
    return ((*a).y - dy) * ((*b).z - dz) - ((*a).z - dz) * ((*b).y - dy);
}
/* *
 * Gets the Y portion of the cross product of vectors `a - (`dx`,y,`dz`) and `b` - (`dx`,y,`dz`)
 */
#[no_mangle]
pub unsafe extern "C" fn Math3D_Vec3fDiff_CrossY(mut a: *mut Vec3f,
                                                 mut b: *mut Vec3f,
                                                 mut dz: f32_0, mut dx: f32_0)
 -> f32_0 {
    return ((*a).z - dz) * ((*b).x - dx) - ((*a).x - dx) * ((*b).z - dz);
}
/* *
 * Gets the Cross Product of vectors `a` and `b` and places the result in `ret`
 */
#[no_mangle]
pub unsafe extern "C" fn Math3D_Vec3f_Cross(mut a: *mut Vec3f,
                                            mut b: *mut Vec3f,
                                            mut ret: *mut Vec3f) {
    (*ret).x = (*a).y * (*b).z - (*a).z * (*b).y;
    (*ret).y = (*a).z * (*b).x - (*a).x * (*b).z;
    (*ret).z = (*a).x * (*b).y - (*a).y * (*b).x;
}
/*
 * Calculates the normal vector to a surface with sides `vb` - `va` and `vc` - `va`
 * outputs the normal to `normal`
 */
#[no_mangle]
pub unsafe extern "C" fn Math3D_SurfaceNorm(mut va: *mut Vec3f,
                                            mut vb: *mut Vec3f,
                                            mut vc: *mut Vec3f,
                                            mut normal: *mut Vec3f) {
    static mut abDiff: Vec3f = Vec3f{x: 0., y: 0., z: 0.,};
    static mut acDiff: Vec3f = Vec3f{x: 0., y: 0., z: 0.,};
    Math_Vec3f_Diff(vb, va, &mut abDiff);
    Math_Vec3f_Diff(vc, va, &mut acDiff);
    Math3D_Vec3f_Cross(&mut abDiff, &mut acDiff, normal);
}
/* *
 * Creates flags relative to the faces of a cube.
 */
#[no_mangle]
pub unsafe extern "C" fn Math3D_PointRelativeToCubeFaces(mut point:
                                                             *mut Vec3f,
                                                         mut min: *mut Vec3f,
                                                         mut max: *mut Vec3f)
 -> s32 {
    let mut ret: s32 = 0 as libc::c_int;
    if (*point).x > (*max).x { ret = 1 as libc::c_int }
    if (*point).x < (*min).x { ret |= 2 as libc::c_int }
    if (*point).y > (*max).y { ret |= 4 as libc::c_int }
    if (*point).y < (*min).y { ret |= 8 as libc::c_int }
    if (*point).z > (*max).z { ret |= 0x10 as libc::c_int }
    if (*point).z < (*min).z { ret |= 0x20 as libc::c_int }
    return ret;
}
/* *
 * Creates flags of `point` relative to the edges of a cube
 */
#[no_mangle]
pub unsafe extern "C" fn Math3D_PointRelativeToCubeEdges(mut point:
                                                             *mut Vec3f,
                                                         mut min: *mut Vec3f,
                                                         mut max: *mut Vec3f)
 -> s32 {
    let mut ret: s32 = 0 as libc::c_int;
    if -(*min).x + (*max).y < -(*point).x + (*point).y {
        ret |= 1 as libc::c_int
    }
    if -(*point).x + (*point).y < -(*max).x + (*min).y {
        ret |= 2 as libc::c_int
    }
    if (*max).x + (*max).y < (*point).x + (*point).y {
        ret |= 4 as libc::c_int
    }
    if (*point).x + (*point).y < (*min).x + (*min).y {
        ret |= 8 as libc::c_int
    }
    if -(*min).z + (*max).y < -(*point).z + (*point).y {
        ret |= 0x10 as libc::c_int
    }
    if -(*point).z + (*point).y < -(*max).z + (*min).y {
        ret |= 0x20 as libc::c_int
    }
    if (*max).z + (*max).y < (*point).z + (*point).y {
        ret |= 0x40 as libc::c_int
    }
    if (*point).z + (*point).y < (*min).z + (*min).y {
        ret |= 0x80 as libc::c_int
    }
    if -(*min).z + (*max).x < -(*point).z + (*point).x {
        ret |= 0x100 as libc::c_int
    }
    if -(*point).z + (*point).x < -(*max).z + (*min).x {
        ret |= 0x200 as libc::c_int
    }
    if (*max).z + (*max).x < (*point).z + (*point).x {
        ret |= 0x400 as libc::c_int
    }
    if (*point).z + (*point).x < (*min).z + (*min).x {
        ret |= 0x800 as libc::c_int
    }
    return ret;
}
/* *
 * Creates flags for `point` relative to the vertices of a cube
 */
#[no_mangle]
pub unsafe extern "C" fn Math3D_PointRelativeToCubeVertices(mut point:
                                                                *mut Vec3f,
                                                            mut min:
                                                                *mut Vec3f,
                                                            mut max:
                                                                *mut Vec3f)
 -> s32 {
    let mut ret: s32 = 0 as libc::c_int;
    if (*max).x + (*max).y + (*max).z < (*point).x + (*point).y + (*point).z {
        ret = 1 as libc::c_int
    }
    if -(*min).x + (*max).y + (*max).z < -(*point).x + (*point).y + (*point).z
       {
        ret |= 2 as libc::c_int
    }
    if -(*min).x + (*max).y - (*min).z < -(*point).x + (*point).y - (*point).z
       {
        ret |= 4 as libc::c_int
    }
    if (*max).x + (*max).y - (*min).z < (*point).x + (*point).y - (*point).z {
        ret |= 8 as libc::c_int
    }
    if (*max).x - (*min).y + (*max).z < (*point).x - (*point).y + (*point).z {
        ret |= 0x10 as libc::c_int
    }
    // ! @bug: The next 2 conditions are the same check.
    if -(*min).x - (*min).y + (*max).z < -(*point).x - (*point).y + (*point).z
       {
        ret |= 0x20 as libc::c_int
    }
    if -(*min).x - (*min).y + (*max).z < -(*point).x - (*point).y + (*point).z
       {
        ret |= 0x40 as libc::c_int
    }
    if -(*min).x - (*min).y - (*min).z < -(*point).x - (*point).y - (*point).z
       {
        ret |= 0x80 as libc::c_int
    }
    return ret;
}
/* *
 * Checks if a line segment with endpoints `a` and `b` intersect a cube
 */
#[no_mangle]
pub unsafe extern "C" fn Math3D_LineVsCube(mut min: *mut Vec3f,
                                           mut max: *mut Vec3f,
                                           mut a: *mut Vec3f,
                                           mut b: *mut Vec3f) -> s32 {
    static mut triVtx0: Vec3f = Vec3f{x: 0., y: 0., z: 0.,};
    static mut triVtx1: Vec3f = Vec3f{x: 0., y: 0., z: 0.,};
    static mut triVtx2: Vec3f = Vec3f{x: 0., y: 0., z: 0.,};
    static mut intersectPoint: Vec3f = Vec3f{x: 0., y: 0., z: 0.,};
    let mut flags: [s32; 2] = [0; 2];
    flags[1 as libc::c_int as usize] = 0 as libc::c_int;
    flags[0 as libc::c_int as usize] = flags[1 as libc::c_int as usize];
    flags[0 as libc::c_int as usize] =
        Math3D_PointRelativeToCubeFaces(a, min, max);
    if flags[0 as libc::c_int as usize] == 0 { return 1 as libc::c_int }
    flags[1 as libc::c_int as usize] =
        Math3D_PointRelativeToCubeFaces(b, min, max);
    if flags[1 as libc::c_int as usize] == 0 { return 1 as libc::c_int }
    if flags[0 as libc::c_int as usize] & flags[1 as libc::c_int as usize] !=
           0 {
        return 0 as libc::c_int
    }
    flags[0 as libc::c_int as usize] |=
        Math3D_PointRelativeToCubeEdges(a, min, max) << 8 as libc::c_int;
    flags[1 as libc::c_int as usize] |=
        Math3D_PointRelativeToCubeEdges(b, min, max) << 8 as libc::c_int;
    if flags[0 as libc::c_int as usize] & flags[1 as libc::c_int as usize] !=
           0 {
        return 0 as libc::c_int
    }
    flags[0 as libc::c_int as usize] |=
        Math3D_PointRelativeToCubeVertices(a, min, max) <<
            0x18 as libc::c_int;
    flags[1 as libc::c_int as usize] |=
        Math3D_PointRelativeToCubeVertices(b, min, max) <<
            0x18 as libc::c_int;
    if flags[0 as libc::c_int as usize] & flags[1 as libc::c_int as usize] !=
           0 {
        return 0 as libc::c_int
    }
    // face 1
    triVtx0.x = (*min).x;
    triVtx0.y = (*min).y;
    triVtx0.z = (*min).z;
    triVtx1.x = (*min).x;
    triVtx1.y = (*min).y;
    triVtx1.z = (*max).z;
    triVtx2.x = (*min).x;
    triVtx2.y = (*max).y;
    triVtx2.z = (*max).z;
    if Math3D_TriLineIntersect(&mut triVtx0, &mut triVtx1, &mut triVtx2,
                               -1.0f32, 0.0f32, 0.0f32, (*min).x, a, b,
                               &mut intersectPoint, 0 as libc::c_int) != 0 {
        return 1 as libc::c_int
    }
    triVtx0.x = (*min).x;
    triVtx0.y = (*min).y;
    triVtx0.z = (*min).z;
    triVtx1.x = (*min).x;
    triVtx1.y = (*max).y;
    triVtx1.z = (*max).z;
    triVtx2.x = (*min).x;
    triVtx2.y = (*max).y;
    triVtx2.z = (*min).z;
    if Math3D_TriLineIntersect(&mut triVtx0, &mut triVtx1, &mut triVtx2,
                               -1.0f32, 0.0f32, 0.0f32, (*min).x, a, b,
                               &mut intersectPoint, 0 as libc::c_int) != 0 {
        return 1 as libc::c_int
    }
    // face 2
    triVtx0.x = (*min).x;
    triVtx0.y = (*max).y;
    triVtx0.z = (*max).z;
    triVtx1.x = (*min).x;
    triVtx1.y = (*min).y;
    triVtx1.z = (*max).z;
    triVtx2.x = (*max).x;
    triVtx2.y = (*max).y;
    triVtx2.z = (*max).z;
    if Math3D_TriLineIntersect(&mut triVtx0, &mut triVtx1, &mut triVtx2,
                               0.0f32, 0.0f32, 1.0f32, -(*max).z, a, b,
                               &mut intersectPoint, 0 as libc::c_int) != 0 {
        return 1 as libc::c_int
    }
    triVtx0.x = (*max).x;
    triVtx0.y = (*max).y;
    triVtx0.z = (*max).z;
    triVtx1.x = (*min).x;
    triVtx1.y = (*min).y;
    triVtx1.z = (*max).z;
    triVtx2.x = (*max).x;
    // ! @bug trVtx1.y should be triVtx2.y, prevents a tri on the cube from being checked.
    triVtx1.y = (*min).y;
    triVtx2.z = (*max).z;
    if Math3D_TriLineIntersect(&mut triVtx0, &mut triVtx1, &mut triVtx2,
                               0.0f32, 0.0f32, 1.0f32, -(*max).z, a, b,
                               &mut intersectPoint, 0 as libc::c_int) != 0 {
        return 1 as libc::c_int
    }
    // face 3
    triVtx0.x = (*max).x;
    triVtx0.y = (*max).y;
    triVtx0.z = (*max).z;
    triVtx1.x = (*min).x;
    triVtx1.y = (*max).y;
    triVtx1.z = (*min).z;
    triVtx2.x = (*min).x;
    triVtx2.y = (*max).y;
    triVtx2.z = (*max).z;
    if Math3D_TriLineIntersect(&mut triVtx0, &mut triVtx1, &mut triVtx2,
                               0.0f32, 1.0f32, 0.0f32, -(*max).y, a, b,
                               &mut intersectPoint, 0 as libc::c_int) != 0 {
        return 1 as libc::c_int
    }
    triVtx0.x = (*max).x;
    triVtx0.y = (*max).y;
    triVtx0.z = (*max).z;
    triVtx1.x = (*max).x;
    triVtx1.y = (*max).y;
    triVtx1.z = (*min).z;
    triVtx2.x = (*min).x;
    triVtx2.y = (*max).y;
    triVtx2.z = (*min).z;
    if Math3D_TriLineIntersect(&mut triVtx0, &mut triVtx1, &mut triVtx2,
                               0.0f32, 1.0f32, 0.0f32, -(*max).y, a, b,
                               &mut intersectPoint, 0 as libc::c_int) != 0 {
        return 1 as libc::c_int
    }
    // face 4
    triVtx0.x = (*min).x;
    triVtx0.y = (*min).y;
    triVtx0.z = (*min).z;
    triVtx1.x = (*min).x;
    triVtx1.y = (*max).y;
    triVtx1.z = (*min).z;
    triVtx2.x = (*max).x;
    triVtx2.y = (*max).y;
    triVtx2.z = (*min).z;
    if Math3D_TriLineIntersect(&mut triVtx0, &mut triVtx1, &mut triVtx2,
                               0.0f32, 0.0f32, -1.0f32, (*min).z, a, b,
                               &mut intersectPoint, 0 as libc::c_int) != 0 {
        return 1 as libc::c_int
    }
    triVtx0.x = (*min).x;
    triVtx0.y = (*min).y;
    triVtx0.z = (*min).z;
    triVtx1.x = (*max).x;
    triVtx1.y = (*max).y;
    triVtx1.z = (*min).z;
    triVtx2.x = (*max).x;
    triVtx2.y = (*min).y;
    triVtx2.z = (*min).z;
    // face 5
    if Math3D_TriLineIntersect(&mut triVtx0, &mut triVtx1, &mut triVtx2,
                               0.0f32, 0.0f32, -1.0f32, (*min).z, a, b,
                               &mut intersectPoint, 0 as libc::c_int) != 0 {
        return 1 as libc::c_int
    }
    triVtx0.x = (*min).x;
    triVtx0.y = (*min).y;
    triVtx0.z = (*min).z;
    triVtx1.x = (*max).x;
    triVtx1.y = (*min).y;
    triVtx1.z = (*min).z;
    triVtx2.x = (*max).x;
    triVtx2.y = (*min).y;
    triVtx2.z = (*max).z;
    if Math3D_TriLineIntersect(&mut triVtx0, &mut triVtx1, &mut triVtx2,
                               0.0f32, -1.0f32, 0.0f32, (*min).y, a, b,
                               &mut intersectPoint, 0 as libc::c_int) != 0 {
        return 1 as libc::c_int
    }
    triVtx0.x = (*min).x;
    triVtx0.y = (*min).y;
    triVtx0.z = (*min).z;
    triVtx1.x = (*max).x;
    triVtx1.y = (*min).y;
    triVtx1.z = (*max).z;
    triVtx2.x = (*min).x;
    triVtx2.y = (*min).y;
    triVtx2.z = (*max).z;
    // face 6
    if Math3D_TriLineIntersect(&mut triVtx0, &mut triVtx1, &mut triVtx2,
                               0.0f32, -1.0f32, 0.0f32, (*min).y, a, b,
                               &mut intersectPoint, 0 as libc::c_int) != 0 {
        return 1 as libc::c_int
    }
    triVtx0.x = (*max).x;
    triVtx0.y = (*max).y;
    triVtx0.z = (*max).z;
    triVtx1.x = (*max).x;
    triVtx1.y = (*min).y;
    triVtx1.z = (*min).z;
    triVtx2.x = (*max).x;
    triVtx2.y = (*max).y;
    triVtx2.z = (*min).z;
    if Math3D_TriLineIntersect(&mut triVtx0, &mut triVtx1, &mut triVtx2,
                               1.0f32, 0.0f32, 0.0f32, -(*max).x, a, b,
                               &mut intersectPoint, 0 as libc::c_int) != 0 {
        return 1 as libc::c_int
    }
    triVtx0.x = (*max).x;
    triVtx0.y = (*max).y;
    triVtx0.z = (*max).z;
    triVtx1.x = (*max).x;
    triVtx1.y = (*min).y;
    triVtx1.z = (*max).z;
    triVtx2.x = (*max).x;
    triVtx2.y = (*min).y;
    triVtx2.z = (*min).z;
    if Math3D_TriLineIntersect(&mut triVtx0, &mut triVtx1, &mut triVtx2,
                               1.0f32, 0.0f32, 0.0f32, -(*max).x, a, b,
                               &mut intersectPoint, 0 as libc::c_int) != 0 {
        return 1 as libc::c_int
    }
    return 0 as libc::c_int;
}
/* *
 * Checks if a line segment with endpoints `a` and `b` intersect a cube
 */
#[no_mangle]
pub unsafe extern "C" fn Math3D_LineVsCubeShort(mut min: *mut Vec3s,
                                                mut max: *mut Vec3s,
                                                mut a: *mut Vec3s,
                                                mut b: *mut Vec3s) -> s32 {
    static mut minF: Vec3f = Vec3f{x: 0., y: 0., z: 0.,};
    static mut maxF: Vec3f = Vec3f{x: 0., y: 0., z: 0.,};
    static mut aF: Vec3f = Vec3f{x: 0., y: 0., z: 0.,};
    static mut bF: Vec3f = Vec3f{x: 0., y: 0., z: 0.,};
    minF.x = (*min).x as f32_0;
    minF.y = (*min).y as f32_0;
    minF.z = (*min).z as f32_0;
    maxF.x = (*max).x as f32_0;
    maxF.y = (*max).y as f32_0;
    maxF.z = (*max).z as f32_0;
    aF.x = (*a).x as f32_0;
    aF.y = (*a).y as f32_0;
    aF.z = (*a).z as f32_0;
    bF.x = (*b).x as f32_0;
    bF.y = (*b).y as f32_0;
    bF.z = (*b).z as f32_0;
    return Math3D_LineVsCube(&mut minF, &mut maxF, &mut aF, &mut bF);
}
/* *
 * Rotates the xz plane around the y axis `angle` degrees.
 * outputs the plane equation `a``pointOnPlane->x` + 0y + `c``pointOnPlane->z`+`d` = 0
 */
#[no_mangle]
pub unsafe extern "C" fn Math3D_RotateXZPlane(mut pointOnPlane: *mut Vec3f,
                                              mut angle: s16,
                                              mut a: *mut f32_0,
                                              mut c: *mut f32_0,
                                              mut d: *mut f32_0) {
    *a = Math_SinS(angle) * 32767.0f32;
    *c = Math_CosS(angle) * 32767.0f32;
    *d = -(*a * (*pointOnPlane).x + *c * (*pointOnPlane).z);
}
/*
 * Defines a plane from verticies `va`, `vb`, and `vc`.  Normal components are output to
 * `nx`, `ny`, and `nz`.  Distance from the origin is output to `originDist`
 * Satisifes the plane equation NxVx + NyVy + NzVz + D = 0
 */
#[no_mangle]
pub unsafe extern "C" fn Math3D_DefPlane(mut va: *mut Vec3f,
                                         mut vb: *mut Vec3f,
                                         mut vc: *mut Vec3f,
                                         mut nx: *mut f32_0,
                                         mut ny: *mut f32_0,
                                         mut nz: *mut f32_0,
                                         mut originDist: *mut f32_0) {
    static mut normal: Vec3f = Vec3f{x: 0., y: 0., z: 0.,};
    let mut normMagnitude: f32_0 = 0.;
    let mut normMagInv: f32_0 = 0.;
    Math3D_SurfaceNorm(va, vb, vc, &mut normal);
    normMagnitude =
        sqrtf(normal.x * normal.x + normal.y * normal.y +
                  normal.z * normal.z);
    if !(fabsf(normMagnitude) < 0.008f32) {
        normMagInv = 1.0f32 / normMagnitude;
        *nx = normal.x * normMagInv;
        *ny = normal.y * normMagInv;
        *nz = normal.z * normMagInv;
        *originDist = -(*nx * (*va).x + *ny * (*va).y + *nz * (*va).z)
    } else { *originDist = 0.0f32; *nz = 0.0f32; *ny = 0.0f32; *nx = 0.0f32 };
}
/*
 * Returns the answer to the plane equation with elements specified by arguments.
 */
#[no_mangle]
pub unsafe extern "C" fn Math3D_Planef(mut nx: f32_0, mut ny: f32_0,
                                       mut nz: f32_0, mut originDist: f32_0,
                                       mut pointOnPlane: *mut Vec3f)
 -> f32_0 {
    return nx * (*pointOnPlane).x + ny * (*pointOnPlane).y +
               nz * (*pointOnPlane).z + originDist;
}
/*
 * Returns the answer to the plane equation
 */
#[no_mangle]
pub unsafe extern "C" fn Math3D_Plane(mut plane: *mut Plane,
                                      mut pointOnPlane: *mut Vec3f) -> f32_0 {
    return (*plane).normal.x * (*pointOnPlane).x +
               (*plane).normal.y * (*pointOnPlane).y +
               (*plane).normal.z * (*pointOnPlane).z + (*plane).originDist;
}
/*
 * Calculates the absolute distance from a point `p` to the plane defined as
 * `nx`, `ny`, `nz`, and `originDist`
 */
#[no_mangle]
pub unsafe extern "C" fn Math3D_UDistPlaneToPos(mut nx: f32_0, mut ny: f32_0,
                                                mut nz: f32_0,
                                                mut originDist: f32_0,
                                                mut p: *mut Vec3f) -> f32_0 {
    if fabsf(sqrtf(nx * nx + ny * ny + nz * nz)) < 0.008f32 {
        osSyncPrintf(b"\x1b[43;30m\x00" as *const u8 as *const libc::c_char);
        // "Math3DLengthPlaneAndPos(): Normal size is near zero %f %f %f"
        osSyncPrintf(b"Math3DLengthPlaneAndPos():\xe6\xb3\x95\xe7\xb7\x9asize \xe3\x81\x8c\xe3\x82\xbc\xe3\x83\xad\xe8\xbf\x91\xe3\x81\x84\xe3\x81\xa7\xe3\x81\x99%f %f %f\n\x00"
                         as *const u8 as *const libc::c_char,
                     nx as libc::c_double, ny as libc::c_double,
                     nz as libc::c_double);
        osSyncPrintf(b"\x1b[m\x00" as *const u8 as *const libc::c_char);
        return 0.0f32
    }
    return fabsf(Math3D_DistPlaneToPos(nx, ny, nz, originDist, p));
}
/*
 * Calculates the signed distance from a point `p` to a plane defined as
 * `nx`, `ny`, `nz`, and `originDist`
 */
#[no_mangle]
pub unsafe extern "C" fn Math3D_DistPlaneToPos(mut nx: f32_0, mut ny: f32_0,
                                               mut nz: f32_0,
                                               mut originDist: f32_0,
                                               mut p: *mut Vec3f) -> f32_0 {
    let mut normMagnitude: f32_0 = 0.;
    normMagnitude = sqrtf(nx * nx + ny * ny + nz * nz);
    if fabsf(normMagnitude) < 0.008f32 {
        osSyncPrintf(b"\x1b[43;30m\x00" as *const u8 as *const libc::c_char);
        // "Math3DSignedLengthPlaneAndPos(): Normal size is close to zero %f %f %f"
        osSyncPrintf(b"Math3DSignedLengthPlaneAndPos():\xe6\xb3\x95\xe7\xb7\x9asize \xe3\x81\x8c\xe3\x82\xbc\xe3\x83\xad\xe8\xbf\x91\xe3\x81\x84\xe3\x81\xa7\xe3\x81\x99%f %f %f\n\x00"
                         as *const u8 as *const libc::c_char,
                     nx as libc::c_double, ny as libc::c_double,
                     nz as libc::c_double);
        osSyncPrintf(b"\x1b[m\x00" as *const u8 as *const libc::c_char);
        return 0.0f32
    }
    return Math3D_Planef(nx, ny, nz, originDist, p) / normMagnitude;
}
/* *
 * Checks if the point defined by (`z`,`x`) is within distance of the triangle defined from `v0`,`v1`, and `v2`
 */
#[no_mangle]
pub unsafe extern "C" fn Math3D_TriChkPointParaYImpl(mut v0: *mut Vec3f,
                                                     mut v1: *mut Vec3f,
                                                     mut v2: *mut Vec3f,
                                                     mut z: f32_0,
                                                     mut x: f32_0,
                                                     mut detMax: f32_0,
                                                     mut chkDist: f32_0,
                                                     mut ny: f32_0) -> s32 {
    let mut detv0v1: f32_0 = 0.;
    let mut detv1v2: f32_0 = 0.;
    let mut detv2v0: f32_0 = 0.;
    let mut distToEdgeSq: f32_0 = 0.;
    let mut chkDistSq: f32_0 = 0.;
    // first check if the point is within range of the triangle.
    if Math3D_CirSquareVsTriSquare((*v0).z, (*v0).x, (*v1).z, (*v1).x,
                                   (*v2).z, (*v2).x, z, x, chkDist) == 0 {
        return 0 as libc::c_int
    }
    // check if the point is within `chkDist` units of any vertex of the triangle.
    chkDistSq = chkDist * chkDist;
    if ((*v0).z - z) * ((*v0).z - z) + ((*v0).x - x) * ((*v0).x - x) <
           chkDistSq ||
           ((*v1).z - z) * ((*v1).z - z) + ((*v1).x - x) * ((*v1).x - x) <
               chkDistSq ||
           ((*v2).z - z) * ((*v2).z - z) + ((*v2).x - x) * ((*v2).x - x) <
               chkDistSq {
        return 1 as libc::c_int
    }
    // Calculate the determinant of each face of the triangle to the point.
    // If all the of determinants are within abs(`detMax`), return true.
    detv0v1 = ((*v0).z - z) * ((*v1).x - x) - ((*v0).x - x) * ((*v1).z - z);
    detv1v2 = ((*v1).z - z) * ((*v2).x - x) - ((*v1).x - x) * ((*v2).z - z);
    detv2v0 = ((*v2).z - z) * ((*v0).x - x) - ((*v2).x - x) * ((*v0).z - z);
    if detMax >= detv0v1 && detMax >= detv1v2 && detMax >= detv2v0 ||
           -detMax <= detv0v1 && -detMax <= detv1v2 && -detMax <= detv2v0 {
        return 1 as libc::c_int
    }
    if fabsf(ny) > 0.5f32 {
        // Do a check on each face of the triangle, if the point is within `chkDist` units return true.
        if Math3D_PointDistToLine2D(z, x, (*v0).z, (*v0).x, (*v1).z, (*v1).x,
                                    &mut distToEdgeSq) != 0 &&
               distToEdgeSq < chkDistSq {
            return 1 as libc::c_int
        }
        if Math3D_PointDistToLine2D(z, x, (*v1).z, (*v1).x, (*v2).z, (*v2).x,
                                    &mut distToEdgeSq) != 0 &&
               distToEdgeSq < chkDistSq {
            return 1 as libc::c_int
        }
        if Math3D_PointDistToLine2D(z, x, (*v2).z, (*v2).x, (*v0).z, (*v0).x,
                                    &mut distToEdgeSq) != 0 &&
               distToEdgeSq < chkDistSq {
            return 1 as libc::c_int
        }
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn Math3D_TriChkPointParaYDeterminate(mut v0:
                                                                *mut Vec3f,
                                                            mut v1:
                                                                *mut Vec3f,
                                                            mut v2:
                                                                *mut Vec3f,
                                                            mut z: f32_0,
                                                            mut x: f32_0,
                                                            mut detMax: f32_0,
                                                            mut ny: f32_0)
 -> s32 {
    return Math3D_TriChkPointParaYImpl(v0, v1, v2, z, x, detMax, 1.0f32, ny);
}
#[no_mangle]
pub unsafe extern "C" fn Math3D_TriChkPointParaYSlopedY(mut v0: *mut Vec3f,
                                                        mut v1: *mut Vec3f,
                                                        mut v2: *mut Vec3f,
                                                        mut z: f32_0,
                                                        mut x: f32_0) -> s32 {
    return Math3D_TriChkPointParaYImpl(v0, v1, v2, z, x, 300.0f32, 1.0f32,
                                       0.6f32);
}
/* *
 * Performs the triangle and point check parallel to the Y axis, outputs the y coordinate of the point to `yIntersect`
 */
#[no_mangle]
pub unsafe extern "C" fn Math3D_TriChkPointParaYIntersectDist(mut v0:
                                                                  *mut Vec3f,
                                                              mut v1:
                                                                  *mut Vec3f,
                                                              mut v2:
                                                                  *mut Vec3f,
                                                              mut nx: f32_0,
                                                              mut ny: f32_0,
                                                              mut nz: f32_0,
                                                              mut originDist:
                                                                  f32_0,
                                                              mut z: f32_0,
                                                              mut x: f32_0,
                                                              mut yIntersect:
                                                                  *mut f32_0,
                                                              mut chkDist:
                                                                  f32_0)
 -> s32 {
    if fabsf(ny) < 0.008f32 { return 0 as libc::c_int }
    if Math3D_TriChkPointParaYImpl(v0, v1, v2, z, x, 300.0f32, chkDist, ny) !=
           0 {
        *yIntersect = (-nx * x - nz * z - originDist) / ny;
        return 1 as libc::c_int
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn Math3D_TriChkPointParaYIntersectInsideTri(mut v0:
                                                                       *mut Vec3f,
                                                                   mut v1:
                                                                       *mut Vec3f,
                                                                   mut v2:
                                                                       *mut Vec3f,
                                                                   mut nx:
                                                                       f32_0,
                                                                   mut ny:
                                                                       f32_0,
                                                                   mut nz:
                                                                       f32_0,
                                                                   mut originDist:
                                                                       f32_0,
                                                                   mut z:
                                                                       f32_0,
                                                                   mut x:
                                                                       f32_0,
                                                                   mut yIntersect:
                                                                       *mut f32_0,
                                                                   mut chkDist:
                                                                       f32_0)
 -> s32 {
    if fabsf(ny) < 0.008f32 { return 0 as libc::c_int }
    if Math3D_TriChkPointParaYImpl(v0, v1, v2, z, x, 0.0f32, chkDist, ny) != 0
       {
        *yIntersect = (-nx * x - nz * z - originDist) / ny;
        return 1 as libc::c_int
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn Math3D_TriChkPointParaY(mut v0: *mut Vec3f,
                                                 mut v1: *mut Vec3f,
                                                 mut v2: *mut Vec3f,
                                                 mut ny: f32_0, mut z: f32_0,
                                                 mut x: f32_0) -> s32 {
    if fabsf(ny) < 0.008f32 { return 0 as libc::c_int }
    if Math3D_TriChkPointParaYImpl(v0, v1, v2, z, x, 300.0f32, 1.0f32, ny) !=
           0 {
        return 1 as libc::c_int
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn Math3D_TriChkLineSegParaYIntersect(mut v0:
                                                                *mut Vec3f,
                                                            mut v1:
                                                                *mut Vec3f,
                                                            mut v2:
                                                                *mut Vec3f,
                                                            mut nx: f32_0,
                                                            mut ny: f32_0,
                                                            mut nz: f32_0,
                                                            mut originDist:
                                                                f32_0,
                                                            mut z: f32_0,
                                                            mut x: f32_0,
                                                            mut yIntersect:
                                                                *mut f32_0,
                                                            mut y0: f32_0,
                                                            mut y1: f32_0)
 -> s32 {
    let mut pointADist: f32_0 = 0.;
    let mut pointBDist: f32_0 = 0.;
    let mut planePos: Vec3f = Vec3f{x: 0., y: 0., z: 0.,};
    if fabsf(ny) < 0.008f32 { return 0 as libc::c_int }
    planePos.x = x;
    planePos.y = y0;
    planePos.z = z;
    pointADist = Math3D_Planef(nx, ny, nz, originDist, &mut planePos);
    planePos.y = y1;
    pointBDist = Math3D_Planef(nx, ny, nz, originDist, &mut planePos);
    if pointADist > 0.0f32 && pointBDist > 0.0f32 ||
           pointADist < 0.0f32 && pointBDist < 0.0f32 {
        return 0 as libc::c_int
    }
    if Math3D_TriChkPointParaYImpl(v0, v1, v2, z, x, 300.0f32, 1.0f32, ny) !=
           0 {
        *yIntersect = (-nx * x - nz * z - originDist) / ny;
        return 1 as libc::c_int
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn Math3D_TriChkPointParaYDist(mut v0: *mut Vec3f,
                                                     mut v1: *mut Vec3f,
                                                     mut v2: *mut Vec3f,
                                                     mut plane: *mut Plane,
                                                     mut z: f32_0,
                                                     mut x: f32_0,
                                                     mut chkDist: f32_0)
 -> s32 {
    if fabsf((*plane).normal.y) < 0.008f32 { return 0 as libc::c_int }
    if Math3D_TriChkPointParaYImpl(v0, v1, v2, z, x, 0.0f32, chkDist,
                                   (*plane).normal.y) != 0 {
        return 1 as libc::c_int
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn Math3D_TriChkPointParaXImpl(mut v0: *mut Vec3f,
                                                     mut v1: *mut Vec3f,
                                                     mut v2: *mut Vec3f,
                                                     mut y: f32_0,
                                                     mut z: f32_0,
                                                     mut detMax: f32_0,
                                                     mut chkDist: f32_0,
                                                     mut nx: f32_0) -> s32 {
    let mut detv0v1: f32_0 = 0.;
    let mut detv1v2: f32_0 = 0.;
    let mut detv2v0: f32_0 = 0.;
    let mut distToEdgeSq: f32_0 = 0.;
    let mut chkDistSq: f32_0 = 0.;
    if Math3D_CirSquareVsTriSquare((*v0).y, (*v0).z, (*v1).y, (*v1).z,
                                   (*v2).y, (*v2).z, y, z, chkDist) == 0 {
        return 0 as libc::c_int
    }
    chkDistSq = chkDist * chkDist;
    if ((*v0).y - y) * ((*v0).y - y) + ((*v0).z - z) * ((*v0).z - z) <
           chkDistSq ||
           ((*v1).y - y) * ((*v1).y - y) + ((*v1).z - z) * ((*v1).z - z) <
               chkDistSq ||
           ((*v2).y - y) * ((*v2).y - y) + ((*v2).z - z) * ((*v2).z - z) <
               chkDistSq {
        return 1 as libc::c_int
    }
    detv0v1 = ((*v0).y - y) * ((*v1).z - z) - ((*v0).z - z) * ((*v1).y - y);
    detv1v2 = ((*v1).y - y) * ((*v2).z - z) - ((*v1).z - z) * ((*v2).y - y);
    detv2v0 = ((*v2).y - y) * ((*v0).z - z) - ((*v2).z - z) * ((*v0).y - y);
    if detv0v1 <= detMax && detv1v2 <= detMax && detv2v0 <= detMax ||
           -detMax <= detv0v1 && -detMax <= detv1v2 && -detMax <= detv2v0 {
        return 1 as libc::c_int
    }
    if fabsf(nx) > 0.5f32 {
        if Math3D_PointDistToLine2D(y, z, (*v0).y, (*v0).z, (*v1).y, (*v1).z,
                                    &mut distToEdgeSq) != 0 &&
               distToEdgeSq < chkDistSq {
            return 1 as libc::c_int
        }
        if Math3D_PointDistToLine2D(y, z, (*v1).y, (*v1).z, (*v2).y, (*v2).z,
                                    &mut distToEdgeSq) != 0 &&
               distToEdgeSq < chkDistSq {
            return 1 as libc::c_int
        }
        if Math3D_PointDistToLine2D(y, z, (*v2).y, (*v2).z, (*v0).y, (*v0).z,
                                    &mut distToEdgeSq) != 0 &&
               distToEdgeSq < chkDistSq {
            return 1 as libc::c_int
        }
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn Math3D_TriChkPointParaXDeterminate(mut v0:
                                                                *mut Vec3f,
                                                            mut v1:
                                                                *mut Vec3f,
                                                            mut v2:
                                                                *mut Vec3f,
                                                            mut y: f32_0,
                                                            mut z: f32_0,
                                                            mut detMax: f32_0,
                                                            mut nx: f32_0)
 -> s32 {
    return Math3D_TriChkPointParaXImpl(v0, v1, v2, y, z, detMax, 1.0f32, nx);
}
#[no_mangle]
pub unsafe extern "C" fn Math3D_TriChkPointParaXIntersect(mut v0: *mut Vec3f,
                                                          mut v1: *mut Vec3f,
                                                          mut v2: *mut Vec3f,
                                                          mut nx: f32_0,
                                                          mut ny: f32_0,
                                                          mut nz: f32_0,
                                                          mut originDist:
                                                              f32_0,
                                                          mut y: f32_0,
                                                          mut z: f32_0,
                                                          mut xIntersect:
                                                              *mut f32_0)
 -> s32 {
    if fabsf(nx) < 0.008f32 { return 0 as libc::c_int }
    if Math3D_TriChkPointParaXImpl(v0, v1, v2, y, z, 300.0f32, 1.0f32, nx) !=
           0 {
        *xIntersect = (-ny * y - nz * z - originDist) / nx;
        return 1 as libc::c_int
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn Math3D_TriChkPointParaX(mut v0: *mut Vec3f,
                                                 mut v1: *mut Vec3f,
                                                 mut v2: *mut Vec3f,
                                                 mut nx: f32_0, mut y: f32_0,
                                                 mut z: f32_0) -> s32 {
    if fabsf(nx) < 0.008f32 { return 0 as libc::c_int }
    if Math3D_TriChkPointParaXImpl(v0, v1, v2, y, z, 300.0f32, 1.0f32, nx) !=
           0 {
        return 1 as libc::c_int
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn Math3D_TriChkLineSegParaXIntersect(mut v0:
                                                                *mut Vec3f,
                                                            mut v1:
                                                                *mut Vec3f,
                                                            mut v2:
                                                                *mut Vec3f,
                                                            mut nx: f32_0,
                                                            mut ny: f32_0,
                                                            mut nz: f32_0,
                                                            mut originDist:
                                                                f32_0,
                                                            mut y: f32_0,
                                                            mut z: f32_0,
                                                            mut xIntersect:
                                                                *mut f32_0,
                                                            mut x0: f32_0,
                                                            mut x1: f32_0)
 -> s32 {
    static mut planePos: Vec3f = Vec3f{x: 0., y: 0., z: 0.,};
    let mut pointADist: f32_0 = 0.;
    let mut pointBDist: f32_0 = 0.;
    if fabsf(nx) < 0.008f32 { return 0 as libc::c_int }
    planePos.x = x0;
    planePos.y = y;
    planePos.z = z;
    pointADist = Math3D_Planef(nx, ny, nz, originDist, &mut planePos);
    planePos.x = x1;
    pointBDist = Math3D_Planef(nx, ny, nz, originDist, &mut planePos);
    if pointADist > 0.0f32 && pointBDist > 0.0f32 ||
           pointADist < 0.0f32 && pointBDist < 0.0f32 {
        return 0 as libc::c_int
    }
    if Math3D_TriChkPointParaXImpl(v0, v1, v2, y, z, 300.0f32, 1.0f32, nx) !=
           0 {
        *xIntersect = (-ny * y - nz * z - originDist) / nx;
        return 1 as libc::c_int
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn Math3D_TriChkPointParaXDist(mut v0: *mut Vec3f,
                                                     mut v1: *mut Vec3f,
                                                     mut v2: *mut Vec3f,
                                                     mut plane: *mut Plane,
                                                     mut y: f32_0,
                                                     mut z: f32_0,
                                                     mut chkDist: f32_0)
 -> s32 {
    if fabsf((*plane).normal.x) < 0.008f32 { return 0 as libc::c_int }
    if Math3D_TriChkPointParaXImpl(v0, v1, v2, y, z, 0.0f32, chkDist,
                                   (*plane).normal.x) != 0 {
        return 1 as libc::c_int
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn Math3D_TriChkPointParaZImpl(mut v0: *mut Vec3f,
                                                     mut v1: *mut Vec3f,
                                                     mut v2: *mut Vec3f,
                                                     mut x: f32_0,
                                                     mut y: f32_0,
                                                     mut detMax: f32_0,
                                                     mut chkDist: f32_0,
                                                     mut nz: f32_0) -> s32 {
    let mut detv0v1: f32_0 = 0.;
    let mut detv1v2: f32_0 = 0.;
    let mut detv2v0: f32_0 = 0.;
    let mut distToEdgeSq: f32_0 = 0.;
    let mut chkDistSq: f32_0 = 0.;
    if Math3D_CirSquareVsTriSquare((*v0).x, (*v0).y, (*v1).x, (*v1).y,
                                   (*v2).x, (*v2).y, x, y, chkDist) == 0 {
        return 0 as libc::c_int
    }
    chkDistSq = chkDist * chkDist;
    if (x - (*v0).x) * (x - (*v0).x) + (y - (*v0).y) * (y - (*v0).y) <
           chkDistSq ||
           (x - (*v1).x) * (x - (*v1).x) + (y - (*v1).y) * (y - (*v1).y) <
               chkDistSq ||
           (x - (*v2).x) * (x - (*v2).x) + (y - (*v2).y) * (y - (*v2).y) <
               chkDistSq {
        // Distance from any vertex to a point is less than chkDist
        return 1 as libc::c_int
    }
    detv0v1 = ((*v0).x - x) * ((*v1).y - y) - ((*v0).y - y) * ((*v1).x - x);
    detv1v2 = ((*v1).x - x) * ((*v2).y - y) - ((*v1).y - y) * ((*v2).x - x);
    detv2v0 = ((*v2).x - x) * ((*v0).y - y) - ((*v2).y - y) * ((*v0).x - x);
    if detMax >= detv0v1 && detMax >= detv1v2 && detMax >= detv2v0 ||
           -detMax <= detv0v1 && -detMax <= detv1v2 && -detMax <= detv2v0 {
        return 1 as libc::c_int
    }
    if fabsf(nz) > 0.5f32 {
        if Math3D_PointDistToLine2D(x, y, (*v0).x, (*v0).y, (*v1).x, (*v1).y,
                                    &mut distToEdgeSq) != 0 &&
               distToEdgeSq < chkDistSq {
            return 1 as libc::c_int
        }
        if Math3D_PointDistToLine2D(x, y, (*v1).x, (*v1).y, (*v2).x, (*v2).y,
                                    &mut distToEdgeSq) != 0 &&
               distToEdgeSq < chkDistSq {
            return 1 as libc::c_int
        }
        if Math3D_PointDistToLine2D(x, y, (*v2).x, (*v2).y, (*v0).x, (*v0).y,
                                    &mut distToEdgeSq) != 0 &&
               distToEdgeSq < chkDistSq {
            return 1 as libc::c_int
        }
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn Math3D_TriChkPointParaZDeterminate(mut v0:
                                                                *mut Vec3f,
                                                            mut v1:
                                                                *mut Vec3f,
                                                            mut v2:
                                                                *mut Vec3f,
                                                            mut x: f32_0,
                                                            mut y: f32_0,
                                                            mut detMax: f32_0,
                                                            mut nz: f32_0)
 -> s32 {
    return Math3D_TriChkPointParaZImpl(v0, v1, v2, x, y, detMax, 1.0f32, nz);
}
#[no_mangle]
pub unsafe extern "C" fn Math3D_TriChkPointParaZIntersect(mut v0: *mut Vec3f,
                                                          mut v1: *mut Vec3f,
                                                          mut v2: *mut Vec3f,
                                                          mut nx: f32_0,
                                                          mut ny: f32_0,
                                                          mut nz: f32_0,
                                                          mut originDist:
                                                              f32_0,
                                                          mut x: f32_0,
                                                          mut y: f32_0,
                                                          mut zIntersect:
                                                              *mut f32_0)
 -> s32 {
    if fabsf(nz) < 0.008f32 { return 0 as libc::c_int }
    if Math3D_TriChkPointParaZImpl(v0, v1, v2, x, y, 300.0f32, 1.0f32, nz) !=
           0 {
        *zIntersect = (-nx * x - ny * y - originDist) / nz;
        return 1 as libc::c_int
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn Math3D_TriChkPointParaZ(mut v0: *mut Vec3f,
                                                 mut v1: *mut Vec3f,
                                                 mut v2: *mut Vec3f,
                                                 mut nz: f32_0, mut x: f32_0,
                                                 mut y: f32_0) -> s32 {
    if fabsf(nz) < 0.008f32 { return 0 as libc::c_int }
    if Math3D_TriChkPointParaZImpl(v0, v1, v2, x, y, 300.0f32, 1.0f32, nz) !=
           0 {
        return 1 as libc::c_int
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn Math3D_TriChkLineSegParaZIntersect(mut v0:
                                                                *mut Vec3f,
                                                            mut v1:
                                                                *mut Vec3f,
                                                            mut v2:
                                                                *mut Vec3f,
                                                            mut nx: f32_0,
                                                            mut ny: f32_0,
                                                            mut nz: f32_0,
                                                            mut originDist:
                                                                f32_0,
                                                            mut x: f32_0,
                                                            mut y: f32_0,
                                                            mut zIntersect:
                                                                *mut f32_0,
                                                            mut z0: f32_0,
                                                            mut z1: f32_0)
 -> s32 {
    static mut planePos: Vec3f = Vec3f{x: 0., y: 0., z: 0.,};
    let mut pointADist: f32_0 = 0.;
    let mut pointBDist: f32_0 = 0.;
    if fabsf(nz) < 0.008f32 { return 0 as libc::c_int }
    planePos.x = x;
    planePos.y = y;
    planePos.z = z0;
    pointADist = Math3D_Planef(nx, ny, nz, originDist, &mut planePos);
    planePos.z = z1;
    pointBDist = Math3D_Planef(nx, ny, nz, originDist, &mut planePos);
    if pointADist > 0.0f32 && pointBDist > 0.0f32 ||
           pointADist < 0.0f32 && pointBDist < 0.0f32 {
        // points on the line segment are on the same side of the plane
        return 0 as libc::c_int
    }
    if Math3D_TriChkPointParaZImpl(v0, v1, v2, x, y, 300.0f32, 1.0f32, nz) !=
           0 {
        *zIntersect = (-nx * x - ny * y - originDist) / nz;
        return 1 as libc::c_int
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn Math3D_TriChkLineSegParaZDist(mut v0: *mut Vec3f,
                                                       mut v1: *mut Vec3f,
                                                       mut v2: *mut Vec3f,
                                                       mut plane: *mut Plane,
                                                       mut x: f32_0,
                                                       mut y: f32_0,
                                                       mut chkDist: f32_0)
 -> s32 {
    if fabsf((*plane).normal.z) < 0.008f32 { return 0 as libc::c_int }
    if Math3D_TriChkPointParaZImpl(v0, v1, v2, x, y, 0.0f32, chkDist,
                                   (*plane).normal.z) != 0 {
        return 1 as libc::c_int
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn Math3D_LineSegFindPlaneIntersect(mut pointADist:
                                                              f32_0,
                                                          mut pointBDist:
                                                              f32_0,
                                                          mut pointA:
                                                              *mut Vec3f,
                                                          mut pointB:
                                                              *mut Vec3f,
                                                          mut intersect:
                                                              *mut Vec3f)
 -> s32 {
    let mut distDiff: f32_0 = 0.;
    distDiff = pointADist - pointBDist;
    if fabsf(distDiff) < 0.008f32 {
        // both points lie on the plane.
        *intersect = *pointB;
        return 0 as libc::c_int
    }
    if pointADist == 0.0f32 {
        // pointA is on the plane
        *intersect = *pointA
    } else if pointBDist == 0.0f32 {
        // pointB is on the plane
        *intersect = *pointB
    } else {
        // place the point at the intersection point.
        Math3D_LineSplitRatio(pointA, pointB, pointADist / distDiff,
                              intersect);
    }
    return 1 as libc::c_int;
}
/* *
 * Determines if the line segement from `linePointA` to `linePointB` crosses the plane
 * from `nx` + `ny` + `nz` + `originDist` = 0.  If fromFront is set, then detection will only
 * be true if point A crosses from the front of the plane
 */
#[no_mangle]
pub unsafe extern "C" fn Math3D_LineSegVsPlane(mut nx: f32_0, mut ny: f32_0,
                                               mut nz: f32_0,
                                               mut originDist: f32_0,
                                               mut linePointA: *mut Vec3f,
                                               mut linePointB: *mut Vec3f,
                                               mut intersect: *mut Vec3f,
                                               mut fromFront: s32) -> s32 {
    let mut pointADist: f32_0 = 0.;
    let mut pointBDist: f32_0 = 0.;
    pointADist = Math3D_Planef(nx, ny, nz, originDist, linePointA);
    pointBDist = Math3D_Planef(nx, ny, nz, originDist, linePointB);
    if pointADist * pointBDist > 0.0f32 {
        *intersect = *linePointB;
        return 0 as libc::c_int
    }
    if fromFront != 0 && pointADist < 0.0f32 && pointBDist > 0.0f32 {
        *intersect = *linePointB;
        return 0 as libc::c_int
    }
    return Math3D_LineSegFindPlaneIntersect(pointADist, pointBDist,
                                            linePointA, linePointB,
                                            intersect);
}
/*
 * Determines if the line formed by `linePiontA` and `linePointB` intersect with Triangle formed from
 * vertices `v0`, `v1`, and `v2` with normal vector `nx`, `ny`, and `nz` with plane distance from origin
 * `originDist` Outputs the intersection point at to `intersect`
 * Returns 1 if the line intersects with the triangle, 0 otherwise
 */
#[no_mangle]
pub unsafe extern "C" fn Math3D_TriLineIntersect(mut v0: *mut Vec3f,
                                                 mut v1: *mut Vec3f,
                                                 mut v2: *mut Vec3f,
                                                 mut nx: f32_0, mut ny: f32_0,
                                                 mut nz: f32_0,
                                                 mut originDist: f32_0,
                                                 mut linePointA: *mut Vec3f,
                                                 mut linePointB: *mut Vec3f,
                                                 mut intersect: *mut Vec3f,
                                                 mut fromFront: s32) -> s32 {
    if Math3D_LineSegVsPlane(nx, ny, nz, originDist, linePointA, linePointB,
                             intersect, fromFront) == 0 {
        return 0 as libc::c_int
    }
    if (nx == 0.0f32 ||
            Math3D_TriChkPointParaX(v0, v1, v2, nx, (*intersect).y,
                                    (*intersect).z) != 0) &&
           (ny == 0.0f32 ||
                Math3D_TriChkPointParaY(v0, v1, v2, ny, (*intersect).z,
                                        (*intersect).x) != 0) &&
           (nz == 0.0f32 ||
                Math3D_TriChkPointParaZ(v0, v1, v2, nz, (*intersect).x,
                                        (*intersect).y) != 0) {
        return 1 as libc::c_int
    }
    *intersect = *linePointB;
    return 0 as libc::c_int;
}
/*
 * Creates a TriNorm output to `tri`, and calculates the normal vector and plane from vertices
 * `va`, `vb`, and `vc`
 */
#[no_mangle]
pub unsafe extern "C" fn Math3D_TriNorm(mut tri: *mut TriNorm,
                                        mut va: *mut Vec3f,
                                        mut vb: *mut Vec3f,
                                        mut vc: *mut Vec3f) {
    (*tri).vtx[0 as libc::c_int as usize] = *va;
    (*tri).vtx[1 as libc::c_int as usize] = *vb;
    (*tri).vtx[2 as libc::c_int as usize] = *vc;
    Math3D_DefPlane(va, vb, vc, &mut (*tri).plane.normal.x,
                    &mut (*tri).plane.normal.y, &mut (*tri).plane.normal.z,
                    &mut (*tri).plane.originDist);
}
/*
 * Determines if point `point` lies within `sphere`
 */
#[no_mangle]
pub unsafe extern "C" fn Math3D_PointInSph(mut sphere: *mut Sphere16,
                                           mut point: *mut Vec3f) -> s32 {
    if Math3D_DistXYZ16toF(&mut (*sphere).center, point) <
           (*sphere).radius as libc::c_int as libc::c_float {
        return 1 as libc::c_int
    }
    return 0 as libc::c_int;
}
/* *
 * Determines the distance from point (`x0`,`y0`) to the line fromed from (`x1`,`y1`) and (`x2`,`y2`)
 * Distance squared is output to `lineLenSq`, returns true if the point perpendicular from (`x0`,`y0`)
 * is contained within the segement between (`x1`,`y1`) and (`x2`,`y2`)
 */
#[no_mangle]
pub unsafe extern "C" fn Math3D_PointDistToLine2D(mut x0: f32_0,
                                                  mut y0: f32_0,
                                                  mut x1: f32_0,
                                                  mut y1: f32_0,
                                                  mut x2: f32_0,
                                                  mut y2: f32_0,
                                                  mut lineLenSq: *mut f32_0)
 -> s32 {
    static mut perpendicularPoint: Vec3f = Vec3f{x: 0., y: 0., z: 0.,};
    let mut perpendicularRatio: f32_0 = 0.;
    let mut xDiff: f32_0 = 0.;
    let mut distSq: f32_0 = 0.;
    let mut yDiff: f32_0 = 0.;
    let mut ret: s32 = 0 as libc::c_int;
    xDiff = x2 - x1;
    yDiff = y2 - y1;
    distSq = xDiff * xDiff + yDiff * yDiff;
    if fabsf(distSq) < 0.008f32 {
        *lineLenSq = 0.0f32;
        return 0 as libc::c_int
    }
    perpendicularRatio = ((x0 - x1) * xDiff + (y0 - y1) * yDiff) / distSq;
    if perpendicularRatio >= 0.0f32 && perpendicularRatio <= 1.0f32 {
        ret = 1 as libc::c_int
    }
    perpendicularPoint.x = xDiff * perpendicularRatio + x1;
    perpendicularPoint.y = yDiff * perpendicularRatio + y1;
    *lineLenSq =
        (perpendicularPoint.x - x0) * (perpendicularPoint.x - x0) +
            (perpendicularPoint.y - y0) * (perpendicularPoint.y - y0);
    return ret;
}
/* *
 * Determines if the line `line` is touching the sphere `sphere` at any point in the line.
 */
#[no_mangle]
pub unsafe extern "C" fn Math3D_LineVsSph(mut sphere: *mut Sphere16,
                                          mut line: *mut Linef) -> s32 {
    static mut sphLinePerpendicularPoint: Vec3f = Vec3f{x: 0., y: 0., z: 0.,};
    let mut lineDiff: Vec3f = Vec3f{x: 0., y: 0., z: 0.,};
    let mut temp_f0_2: f32_0 = 0.;
    let mut lineLenSq: f32_0 = 0.;
    if Math3D_PointInSph(sphere, &mut (*line).a) != 0 ||
           Math3D_PointInSph(sphere, &mut (*line).b) != 0 {
        // either point of the line is in the sphere.
        return 1 as libc::c_int
    }
    lineDiff.x = (*line).b.x - (*line).a.x;
    lineDiff.y = (*line).b.y - (*line).a.y;
    lineDiff.z = (*line).b.z - (*line).a.z;
    lineLenSq =
        lineDiff.x * lineDiff.x + lineDiff.y * lineDiff.y +
            lineDiff.z * lineDiff.z;
    if fabsf(lineLenSq) < 0.008f32 {
        // line length is "0"
        return 0 as libc::c_int
    }
    temp_f0_2 =
        (((*sphere).center.x as libc::c_int as libc::c_float - (*line).a.x) *
             lineDiff.x +
             ((*sphere).center.y as libc::c_int as libc::c_float -
                  (*line).a.y) * lineDiff.y +
             ((*sphere).center.z as libc::c_int as libc::c_float -
                  (*line).a.z) * lineDiff.z) / lineLenSq;
    if temp_f0_2 < 0.0f32 || temp_f0_2 > 1.0f32 { return 0 as libc::c_int }
    sphLinePerpendicularPoint.x = lineDiff.x * temp_f0_2 + (*line).a.x;
    sphLinePerpendicularPoint.y = lineDiff.y * temp_f0_2 + (*line).a.y;
    sphLinePerpendicularPoint.z = lineDiff.z * temp_f0_2 + (*line).a.z;
    if (sphLinePerpendicularPoint.x -
            (*sphere).center.x as libc::c_int as libc::c_float) *
           (sphLinePerpendicularPoint.x -
                (*sphere).center.x as libc::c_int as libc::c_float) +
           (sphLinePerpendicularPoint.y -
                (*sphere).center.y as libc::c_int as libc::c_float) *
               (sphLinePerpendicularPoint.y -
                    (*sphere).center.y as libc::c_int as libc::c_float) +
           (sphLinePerpendicularPoint.z -
                (*sphere).center.z as libc::c_int as libc::c_float) *
               (sphLinePerpendicularPoint.z -
                    (*sphere).center.z as libc::c_int as libc::c_float) <=
           (*sphere).radius as f32_0 * (*sphere).radius as f32_0 {
        return 1 as libc::c_int
    }
    return 0 as libc::c_int;
}
/* *
 * Gets the surface point of `sphere` intersecting with `tri` generated from the line formed from the
 * sphere's surface to the midpoint of the line formed from the first two vertices of the tri
 */
#[no_mangle]
pub unsafe extern "C" fn Math3D_GetSphVsTriIntersectPoint(mut sphere:
                                                              *mut Sphere16,
                                                          mut tri:
                                                              *mut TriNorm,
                                                          mut intersectPoint:
                                                              *mut Vec3f) {
    static mut v0v1Center: Vec3f = Vec3f{x: 0., y: 0., z: 0.,};
    static mut sphereCenter: Vec3f = Vec3f{x: 0., y: 0., z: 0.,};
    let mut dist: f32_0 = 0.;
    let mut splitRatio: f32_0 = 0.;
    v0v1Center.x =
        ((*tri).vtx[0 as libc::c_int as usize].x +
             (*tri).vtx[1 as libc::c_int as usize].x) * 0.5f32;
    v0v1Center.y =
        ((*tri).vtx[0 as libc::c_int as usize].y +
             (*tri).vtx[1 as libc::c_int as usize].y) * 0.5f32;
    v0v1Center.z =
        ((*tri).vtx[0 as libc::c_int as usize].z +
             (*tri).vtx[1 as libc::c_int as usize].z) * 0.5f32;
    sphereCenter.x = (*sphere).center.x as f32_0;
    sphereCenter.y = (*sphere).center.y as f32_0;
    sphereCenter.z = (*sphere).center.z as f32_0;
    dist = Math3D_Vec3f_DistXYZ(&mut v0v1Center, &mut sphereCenter);
    // Distance from the sphere's center to the center of the line formed from v0->v1
    if fabsf(dist) < 0.008f32 {
        (*intersectPoint).x = sphereCenter.x;
        (*intersectPoint).y = sphereCenter.y;
        (*intersectPoint).z = sphereCenter.z;
        return
    }
    splitRatio = (*sphere).radius as libc::c_int as libc::c_float / dist;
    Math3D_LineSplitRatio(&mut sphereCenter, &mut v0v1Center, splitRatio,
                          intersectPoint);
}
/* *
 * Determines if `sphere` and `tri` and touching, and outputs the intersection point to `intersectPoint`
 */
#[no_mangle]
pub unsafe extern "C" fn Math3D_TriVsSphIntersect(mut sphere: *mut Sphere16,
                                                  mut tri: *mut TriNorm,
                                                  mut intersectPoint:
                                                      *mut Vec3f) -> s32 {
    static mut triTestLine: Linef =
        Linef{a: Vec3f{x: 0., y: 0., z: 0.,},
              b: Vec3f{x: 0., y: 0., z: 0.,},};
    static mut sphereCenter: Vec3f = Vec3f{x: 0., y: 0., z: 0.,};
    static mut sphPlanePos: Vec3f = Vec3f{x: 0., y: 0., z: 0.,};
    let mut radius: f32_0 = 0.;
    let mut nx: f32_0 = 0.;
    let mut ny: f32_0 = 0.;
    let mut nz: f32_0 = 0.;
    let mut planeDist: f32_0 = 0.;
    sphereCenter.x = (*sphere).center.x as f32_0;
    sphereCenter.y = (*sphere).center.y as f32_0;
    sphereCenter.z = (*sphere).center.z as f32_0;
    radius = (*sphere).radius as f32_0;
    if Math3D_SphCubeVsTriCube(&mut *(*tri).vtx.as_mut_ptr().offset(0 as
                                                                        libc::c_int
                                                                        as
                                                                        isize),
                               &mut *(*tri).vtx.as_mut_ptr().offset(1 as
                                                                        libc::c_int
                                                                        as
                                                                        isize),
                               &mut *(*tri).vtx.as_mut_ptr().offset(2 as
                                                                        libc::c_int
                                                                        as
                                                                        isize),
                               &mut sphereCenter, radius) == 0 {
        return 0 as libc::c_int
    }
    planeDist =
        Math3D_UDistPlaneToPos((*tri).plane.normal.x, (*tri).plane.normal.y,
                               (*tri).plane.normal.z, (*tri).plane.originDist,
                               &mut sphereCenter);
    if radius < planeDist {
        // the point that lies within the plane of the triangle which is perpendicular to the sphere's center is more
        // than the radius of the sphere, the plane never crosses the sphere.
        return 0 as libc::c_int
    }
    // tests if any of the edges of the triangle are intersecting the sphere
    triTestLine.a = (*tri).vtx[0 as libc::c_int as usize];
    triTestLine.b = (*tri).vtx[1 as libc::c_int as usize];
    if Math3D_LineVsSph(sphere, &mut triTestLine) != 0 {
        Math3D_GetSphVsTriIntersectPoint(sphere, tri, intersectPoint);
        return 1 as libc::c_int
    }
    triTestLine.a = (*tri).vtx[1 as libc::c_int as usize];
    triTestLine.b = (*tri).vtx[2 as libc::c_int as usize];
    if Math3D_LineVsSph(sphere, &mut triTestLine) != 0 {
        Math3D_GetSphVsTriIntersectPoint(sphere, tri, intersectPoint);
        return 1 as libc::c_int
    }
    triTestLine.a = (*tri).vtx[2 as libc::c_int as usize];
    triTestLine.b = (*tri).vtx[0 as libc::c_int as usize];
    if Math3D_LineVsSph(sphere, &mut triTestLine) != 0 {
        Math3D_GetSphVsTriIntersectPoint(sphere, tri, intersectPoint);
        return 1 as libc::c_int
    }
    nx = (*tri).plane.normal.x * planeDist;
    ny = (*tri).plane.normal.y * planeDist;
    nz = (*tri).plane.normal.z * planeDist;
    if Math3D_Planef((*tri).plane.normal.x, (*tri).plane.normal.y,
                     (*tri).plane.normal.z, (*tri).plane.originDist,
                     &mut sphereCenter) > 0.0f32 {
        sphPlanePos.x = sphereCenter.x - nx;
        sphPlanePos.y = sphereCenter.y - ny;
        sphPlanePos.z = sphereCenter.z - nz
    } else {
        sphPlanePos.x = sphereCenter.x + nx;
        sphPlanePos.y = sphereCenter.y + ny;
        sphPlanePos.z = sphereCenter.z + nz
    }
    if fabsf((*tri).plane.normal.y) > 0.5f32 {
        if Math3D_TriChkPointParaYDeterminate(&mut *(*tri).vtx.as_mut_ptr().offset(0
                                                                                       as
                                                                                       libc::c_int
                                                                                       as
                                                                                       isize),
                                              &mut *(*tri).vtx.as_mut_ptr().offset(1
                                                                                       as
                                                                                       libc::c_int
                                                                                       as
                                                                                       isize),
                                              &mut *(*tri).vtx.as_mut_ptr().offset(2
                                                                                       as
                                                                                       libc::c_int
                                                                                       as
                                                                                       isize),
                                              sphPlanePos.z, sphPlanePos.x,
                                              0.0f32, (*tri).plane.normal.y)
               != 0 {
            Math3D_GetSphVsTriIntersectPoint(sphere, tri, intersectPoint);
            return 1 as libc::c_int
        }
    } else if fabsf((*tri).plane.normal.x) > 0.5f32 {
        if Math3D_TriChkPointParaXDeterminate(&mut *(*tri).vtx.as_mut_ptr().offset(0
                                                                                       as
                                                                                       libc::c_int
                                                                                       as
                                                                                       isize),
                                              &mut *(*tri).vtx.as_mut_ptr().offset(1
                                                                                       as
                                                                                       libc::c_int
                                                                                       as
                                                                                       isize),
                                              &mut *(*tri).vtx.as_mut_ptr().offset(2
                                                                                       as
                                                                                       libc::c_int
                                                                                       as
                                                                                       isize),
                                              sphPlanePos.y, sphPlanePos.z,
                                              0.0f32, (*tri).plane.normal.x)
               != 0 {
            Math3D_GetSphVsTriIntersectPoint(sphere, tri, intersectPoint);
            return 1 as libc::c_int
        }
    } else if Math3D_TriChkPointParaZDeterminate(&mut *(*tri).vtx.as_mut_ptr().offset(0
                                                                                          as
                                                                                          libc::c_int
                                                                                          as
                                                                                          isize),
                                                 &mut *(*tri).vtx.as_mut_ptr().offset(1
                                                                                          as
                                                                                          libc::c_int
                                                                                          as
                                                                                          isize),
                                                 &mut *(*tri).vtx.as_mut_ptr().offset(2
                                                                                          as
                                                                                          libc::c_int
                                                                                          as
                                                                                          isize),
                                                 sphPlanePos.x, sphPlanePos.y,
                                                 0.0f32,
                                                 (*tri).plane.normal.z) != 0 {
        Math3D_GetSphVsTriIntersectPoint(sphere, tri, intersectPoint);
        return 1 as libc::c_int
    }
    return 0 as libc::c_int;
}
/*
 * Checks if point `point` is within cylinder `cyl`
 * Returns 1 if the point is inside the cylinder, 0 otherwise.
 */
#[no_mangle]
pub unsafe extern "C" fn Math3D_PointInCyl(mut cyl: *mut Cylinder16,
                                           mut point: *mut Vec3f) -> s32 {
    let mut bottom: f32_0 = 0.;
    let mut top: f32_0 = 0.;
    let mut x: f32_0 = 0.;
    let mut z: f32_0 = 0.;
    x = (*cyl).pos.x as libc::c_int as libc::c_float - (*point).x;
    z = (*cyl).pos.z as libc::c_int as libc::c_float - (*point).z;
    bottom =
        (*cyl).pos.y as f32_0 + (*cyl).yShift as libc::c_int as libc::c_float;
    top = (*cyl).height as libc::c_int as libc::c_float + bottom;
    if x * x + z * z <
           ((*cyl).radius as libc::c_int * (*cyl).radius as libc::c_int) as
               libc::c_float && bottom < (*point).y && (*point).y < top {
        return 1 as libc::c_int
    } else { return 0 as libc::c_int };
}
#[no_mangle]
pub unsafe extern "C" fn Math3D_CylVsLineSeg(mut cyl: *mut Cylinder16,
                                             mut linePointA: *mut Vec3f,
                                             mut linePointB: *mut Vec3f,
                                             mut intersectA: *mut Vec3f,
                                             mut intersectB: *mut Vec3f)
 -> s32 {
    let mut cylToPtA: Vec3f = Vec3f{x: 0., y: 0., z: 0.,};
    let mut cylToPtB: Vec3f = Vec3f{x: 0., y: 0., z: 0.,};
    let mut ptAToPtB: Vec3f = Vec3f{x: 0., y: 0., z: 0.,};
    let mut fracA: f32_0 = 0.;
    let mut fracB: f32_0 = 0.;
    let mut fracBase: f32_0 = 0.;
    let mut zero: f32_0 = 0.0f32;
    let mut pad: f32_0 = 0.;
    let mut cylRadiusSq: f32_0 = 0.;
    let mut radSqDiff: f32_0 = 0.;
    let mut distCent2: f32_0 = 0.;
    let mut dot2AB: f32_0 = 0.;
    let mut sideIntA: s32 = 0;
    let mut sideIntB: s32 = 0;
    let mut intBeyondA: s32 = 0;
    let mut intBeyondB: s32 = 0;
    let mut intFlags: s32 = 0 as libc::c_int;
    let mut intPts: [Vec3f; 4] = [Vec3f{x: 0., y: 0., z: 0.,}; 4];
    let mut count: s32 = 0;
    let mut i: s32 = 0;
    if Math3D_PointInCyl(cyl, linePointA) != 0 &&
           Math3D_PointInCyl(cyl, linePointB) != 0 {
        // both points are in the cylinder
        *intersectA = *linePointA;
        *intersectB = *linePointB;
        return 2 as libc::c_int
    }
    cylToPtA.x =
        (*linePointA).x - (*cyl).pos.x as libc::c_int as libc::c_float;
    cylToPtA.y =
        (*linePointA).y - (*cyl).pos.y as libc::c_int as libc::c_float -
            (*cyl).yShift as libc::c_int as libc::c_float;
    cylToPtA.z =
        (*linePointA).z - (*cyl).pos.z as libc::c_int as libc::c_float;
    cylToPtB.x =
        (*linePointB).x - (*cyl).pos.x as libc::c_int as libc::c_float;
    cylToPtB.y =
        (*linePointB).y - (*cyl).pos.y as libc::c_int as libc::c_float -
            (*cyl).yShift as libc::c_int as libc::c_float;
    cylToPtB.z =
        (*linePointB).z - (*cyl).pos.z as libc::c_int as libc::c_float;
    Math_Vec3f_Diff(&mut cylToPtB, &mut cylToPtA, &mut ptAToPtB);
    cylRadiusSq =
        ((*cyl).radius as libc::c_int * (*cyl).radius as libc::c_int) as
            f32_0;
    /* *
     * This section checks for intersections with the cylinder's base and top
     */
    if !(fabsf(ptAToPtB.y) < 0.008f32) {
        // fraction of length along AB to reach y = 0
        fracBase = -cylToPtA.y / ptAToPtB.y;
        if 0.0f32 <= fracBase && fracBase <= 1.0f32 {
            let mut baseIntX: f32_0 = ptAToPtB.x * fracBase + cylToPtA.x;
            let mut baseIntZ: f32_0 = ptAToPtB.z * fracBase + cylToPtA.z;
            if baseIntX * baseIntX + baseIntZ * baseIntZ < cylRadiusSq {
                // adds base intersection point to intPts and sets its flag
                intPts[0 as libc::c_int as usize].x =
                    (*cyl).pos.x as libc::c_int as libc::c_float + baseIntX;
                intPts[0 as libc::c_int as usize].y =
                    (*cyl).pos.y as f32_0 +
                        (*cyl).yShift as libc::c_int as libc::c_float;
                intPts[0 as libc::c_int as usize].z =
                    (*cyl).pos.z as libc::c_int as libc::c_float + baseIntZ;
                intFlags |= 1 as libc::c_int
            }
        }
        // fraction of length along AB to reach y = cyl->height
        fracA =
            ((*cyl).height as libc::c_int as libc::c_float - cylToPtA.y) /
                ptAToPtB.y;
        if 0.0f32 <= fracA && fracA <= 1.0f32 {
            let mut topIntX: f32_0 = ptAToPtB.x * fracA + cylToPtA.x;
            let mut topIntZ: f32_0 = ptAToPtB.z * fracA + cylToPtA.z;
            if topIntX * topIntX + topIntZ * topIntZ < cylRadiusSq {
                // adds top intersection point to intPts and sets its flag
                intPts[1 as libc::c_int as usize].x =
                    (*cyl).pos.x as libc::c_int as libc::c_float + topIntX;
                intPts[1 as libc::c_int as usize].y =
                    (*cyl).pos.y as f32_0 +
                        (*cyl).yShift as libc::c_int as libc::c_float +
                        (*cyl).height as libc::c_int as libc::c_float;
                intPts[1 as libc::c_int as usize].z =
                    (*cyl).pos.z as libc::c_int as libc::c_float + topIntZ;
                intFlags |= 2 as libc::c_int
            }
        }
    }
    /* *
     * This section finds the points of intersection of the infinite line containing AB with the side of the infinite
     * cylinder containing cyl. Intersection points beyond the bounds of the segment and cylinder are filtered out
     * afterward.
     */
    radSqDiff =
        cylToPtA.x * cylToPtA.x + cylToPtA.z * cylToPtA.z - cylRadiusSq;
    if !(fabsf(2.0f32 * (ptAToPtB.x * ptAToPtB.x + ptAToPtB.z * ptAToPtB.z)) <
             0.008f32) {
        dot2AB = 2.0f32 * (ptAToPtB.x * cylToPtA.x + ptAToPtB.z * cylToPtA.z);
        if dot2AB * dot2AB <
               4.0f32 * (ptAToPtB.x * ptAToPtB.x + ptAToPtB.z * ptAToPtB.z) *
                   radSqDiff {
            // Line's closest xz-approach is outside cylinder. No intersections.
            return 0 as libc::c_int
        }
        if dot2AB * dot2AB -
               4.0f32 * (ptAToPtB.x * ptAToPtB.x + ptAToPtB.z * ptAToPtB.z) *
                   radSqDiff > zero {
            sideIntB = 1 as libc::c_int;
            sideIntA = sideIntB
        } else {
            // Line is tangent in xz-plane. At most 1 side intersection.
            sideIntA = 1 as libc::c_int;
            sideIntB = 0 as libc::c_int
        }
        distCent2 =
            sqrtf(dot2AB * dot2AB -
                      4.0f32 *
                          (ptAToPtB.x * ptAToPtB.x + ptAToPtB.z * ptAToPtB.z)
                          * radSqDiff);
        if sideIntA == 1 as libc::c_int {
            // fraction of length along AB for side intersection closer to A
            fracA =
                (distCent2 - dot2AB) /
                    (2.0f32 *
                         (ptAToPtB.x * ptAToPtB.x + ptAToPtB.z * ptAToPtB.z))
        }
        if sideIntB == 1 as libc::c_int {
            // fraction of length along AB for side intersection closer to B
            fracB =
                (-dot2AB - distCent2) /
                    (2.0f32 *
                         (ptAToPtB.x * ptAToPtB.x + ptAToPtB.z * ptAToPtB.z))
        }
    } else if !(fabsf(2.0f32 *
                          (ptAToPtB.x * cylToPtA.x + ptAToPtB.z * cylToPtA.z))
                    < 0.008f32) {
        // Used if the line segment is nearly vertical. Unclear what it's calculating.
        fracA =
            -radSqDiff /
                (2.0f32 *
                     (ptAToPtB.x * cylToPtA.x + ptAToPtB.z * cylToPtA.z));
        sideIntA = 1 as libc::c_int;
        sideIntB = 0 as libc::c_int
    } else { return 0 as libc::c_int }
    // checks for intersection points outside the bounds of the segment
    if sideIntB == 0 {
        if fracA < 0.0f32 || 1.0f32 < fracA { return 0 as libc::c_int }
    } else {
        intBeyondA = (fracA < 0.0f32 || 1.0f32 < fracA) as libc::c_int;
        intBeyondB = (fracB < 0.0f32 || 1.0f32 < fracB) as libc::c_int;
        if intBeyondA != 0 && intBeyondB != 0 { return 0 as libc::c_int }
        if intBeyondA != 0 { sideIntA = 0 as libc::c_int }
        if intBeyondB != 0 { sideIntB = 0 as libc::c_int }
    }
    // checks for intersection points outside the bounds of the cylinder
    if sideIntA == 1 as libc::c_int &&
           (fracA * ptAToPtB.y + cylToPtA.y < 0.0f32 ||
                ((*cyl).height as libc::c_int as libc::c_float) <
                    fracA * ptAToPtB.y + cylToPtA.y) {
        sideIntA = 0 as libc::c_int
    }
    if sideIntB == 1 as libc::c_int &&
           (fracB * ptAToPtB.y + cylToPtA.y < 0.0f32 ||
                ((*cyl).height as libc::c_int as libc::c_float) <
                    fracB * ptAToPtB.y + cylToPtA.y) {
        sideIntB = 0 as libc::c_int
    }
    if sideIntA == 0 as libc::c_int && sideIntB == 0 as libc::c_int {
        return 0 as libc::c_int
    }
    // Adds intersection points to intPts and sets side A and side B flags
    if sideIntA == 1 as libc::c_int && sideIntB == 1 as libc::c_int {
        intPts[2 as libc::c_int as usize].x =
            fracA * ptAToPtB.x + cylToPtA.x +
                (*cyl).pos.x as libc::c_int as libc::c_float;
        intPts[2 as libc::c_int as usize].y =
            fracA * ptAToPtB.y + cylToPtA.y +
                (*cyl).pos.y as libc::c_int as libc::c_float +
                (*cyl).yShift as libc::c_int as libc::c_float;
        intPts[2 as libc::c_int as usize].z =
            fracA * ptAToPtB.z + cylToPtA.z +
                (*cyl).pos.z as libc::c_int as libc::c_float;
        intFlags |= 4 as libc::c_int;
        intPts[3 as libc::c_int as usize].x =
            fracB * ptAToPtB.x + cylToPtA.x +
                (*cyl).pos.x as libc::c_int as libc::c_float;
        intPts[3 as libc::c_int as usize].y =
            fracB * ptAToPtB.y + cylToPtA.y +
                (*cyl).pos.y as libc::c_int as libc::c_float +
                (*cyl).yShift as libc::c_int as libc::c_float;
        intPts[3 as libc::c_int as usize].z =
            fracB * ptAToPtB.z + cylToPtA.z +
                (*cyl).pos.z as libc::c_int as libc::c_float;
        intFlags |= 8 as libc::c_int
    } else if sideIntA == 1 as libc::c_int {
        intPts[2 as libc::c_int as usize].x =
            fracA * ptAToPtB.x + cylToPtA.x +
                (*cyl).pos.x as libc::c_int as libc::c_float;
        intPts[2 as libc::c_int as usize].y =
            fracA * ptAToPtB.y + cylToPtA.y +
                (*cyl).pos.y as libc::c_int as libc::c_float +
                (*cyl).yShift as libc::c_int as libc::c_float;
        intPts[2 as libc::c_int as usize].z =
            fracA * ptAToPtB.z + cylToPtA.z +
                (*cyl).pos.z as libc::c_int as libc::c_float;
        intFlags |= 4 as libc::c_int
    } else if sideIntB == 1 as libc::c_int {
        intPts[2 as libc::c_int as usize].x =
            fracB * ptAToPtB.x + cylToPtA.x +
                (*cyl).pos.x as libc::c_int as libc::c_float;
        intPts[2 as libc::c_int as usize].y =
            fracB * ptAToPtB.y + cylToPtA.y +
                (*cyl).pos.y as libc::c_int as libc::c_float +
                (*cyl).yShift as libc::c_int as libc::c_float;
        intPts[2 as libc::c_int as usize].z =
            fracB * ptAToPtB.z + cylToPtA.z +
                (*cyl).pos.z as libc::c_int as libc::c_float;
        intFlags |= 4 as libc::c_int
    }
    /* *
     * Places the found intersection points into intersectA and intersectB. IntersectA is always closer to point A
     */
    count = 0 as libc::c_int;
    i = 0 as libc::c_int;
    while i < 4 as libc::c_int {
        if intFlags & (1 as libc::c_int) << i != 0 {
            if count == 0 as libc::c_int {
                *intersectA = intPts[i as usize]
            } else if count == 1 as libc::c_int {
                if Math3D_Vec3fDistSq(intersectA, linePointA) <
                       Math3D_Vec3fDistSq(intersectA,
                                          &mut *intPts.as_mut_ptr().offset(i
                                                                               as
                                                                               isize))
                   {
                    *intersectB = intPts[i as usize]
                } else {
                    *intersectB = *intersectA;
                    *intersectA = intPts[i as usize]
                }
                break ;
            }
            count += 1
        }
        i += 1
    }
    return count;
}
/*
 * Determines if `cyl` and `tri` are touching.  The point of intersection
 * is placed in `intersect` Returns 1 if they are touching, 0 otherwise.
 */
#[no_mangle]
pub unsafe extern "C" fn Math3D_CylTriVsIntersect(mut cyl: *mut Cylinder16,
                                                  mut tri: *mut TriNorm,
                                                  mut intersect: *mut Vec3f)
 -> s32 {
    static mut topSphere: Sphere16 =
        Sphere16{center: Vec3s{x: 0, y: 0, z: 0,}, radius: 0,};
    static mut bottomSphere: Sphere16 =
        Sphere16{center: Vec3s{x: 0, y: 0, z: 0,}, radius: 0,};
    static mut cylIntersectA: Vec3f = Vec3f{x: 0., y: 0., z: 0.,};
    static mut cylIntersectB: Vec3f = Vec3f{x: 0., y: 0., z: 0.,};
    let mut yIntersect: f32_0 = 0.;
    let mut cylTop: f32_0 = 0.;
    let mut cylBottom: f32_0 = 0.;
    let mut minDistSq: f32_0 = 0.;
    let mut radiusTodistFromCylYIntersectTov0v1: f32_0 = 0.;
    let mut distFromPointAToIntersectASq: f32_0 = 0.;
    let mut cylIntersectCenter: Vec3f = Vec3f{x: 0., y: 0., z: 0.,};
    let mut midpointv0v1: Vec3f = Vec3f{x: 0., y: 0., z: 0.,};
    let mut diffMidpointIntersect: Vec3f = Vec3f{x: 0., y: 0., z: 0.,};
    let mut distFromCylYIntersectTov0v1: f32_0 = 0.;
    let mut pad: s32 = 0;
    cylBottom =
        (*cyl).pos.y as f32_0 + (*cyl).yShift as libc::c_int as libc::c_float;
    cylTop = (*cyl).height as libc::c_int as libc::c_float + cylBottom;
    if (*tri).vtx[0 as libc::c_int as usize].y < cylBottom &&
           (*tri).vtx[1 as libc::c_int as usize].y < cylBottom &&
           (*tri).vtx[2 as libc::c_int as usize].y < cylBottom ||
           cylTop < (*tri).vtx[0 as libc::c_int as usize].y &&
               cylTop < (*tri).vtx[1 as libc::c_int as usize].y &&
               cylTop < (*tri).vtx[2 as libc::c_int as usize].y {
        // If all of the verticies are below or all of the verticies are above the cylinder.
        return 0 as libc::c_int
    }
    minDistSq = 1.0e38f32;
    if Math3D_CylVsLineSeg(cyl,
                           &mut *(*tri).vtx.as_mut_ptr().offset(0 as
                                                                    libc::c_int
                                                                    as isize),
                           &mut *(*tri).vtx.as_mut_ptr().offset(1 as
                                                                    libc::c_int
                                                                    as isize),
                           &mut cylIntersectA, &mut cylIntersectB) != 0 {
        distFromPointAToIntersectASq =
            Math3D_Vec3fDistSq(&mut cylIntersectA,
                               &mut *(*tri).vtx.as_mut_ptr().offset(0 as
                                                                        libc::c_int
                                                                        as
                                                                        isize));
        minDistSq = distFromPointAToIntersectASq;
        *intersect = cylIntersectA
    }
    if Math3D_CylVsLineSeg(cyl,
                           &mut *(*tri).vtx.as_mut_ptr().offset(2 as
                                                                    libc::c_int
                                                                    as isize),
                           &mut *(*tri).vtx.as_mut_ptr().offset(1 as
                                                                    libc::c_int
                                                                    as isize),
                           &mut cylIntersectA, &mut cylIntersectB) != 0 {
        distFromPointAToIntersectASq =
            Math3D_Vec3fDistSq(&mut cylIntersectA,
                               &mut *(*tri).vtx.as_mut_ptr().offset(2 as
                                                                        libc::c_int
                                                                        as
                                                                        isize));
        if distFromPointAToIntersectASq < minDistSq {
            *intersect = cylIntersectA;
            minDistSq = distFromPointAToIntersectASq
        }
    }
    if Math3D_CylVsLineSeg(cyl,
                           &mut *(*tri).vtx.as_mut_ptr().offset(0 as
                                                                    libc::c_int
                                                                    as isize),
                           &mut *(*tri).vtx.as_mut_ptr().offset(2 as
                                                                    libc::c_int
                                                                    as isize),
                           &mut cylIntersectA, &mut cylIntersectB) != 0 {
        distFromPointAToIntersectASq =
            Math3D_Vec3fDistSq(&mut cylIntersectA,
                               &mut *(*tri).vtx.as_mut_ptr().offset(0 as
                                                                        libc::c_int
                                                                        as
                                                                        isize));
        if distFromPointAToIntersectASq < minDistSq {
            *intersect = cylIntersectA;
            minDistSq = distFromPointAToIntersectASq
        }
    }
    if minDistSq != 1.0e38f32 { return 1 as libc::c_int }
    if Math3D_TriChkLineSegParaYIntersect(&mut *(*tri).vtx.as_mut_ptr().offset(0
                                                                                   as
                                                                                   libc::c_int
                                                                                   as
                                                                                   isize),
                                          &mut *(*tri).vtx.as_mut_ptr().offset(1
                                                                                   as
                                                                                   libc::c_int
                                                                                   as
                                                                                   isize),
                                          &mut *(*tri).vtx.as_mut_ptr().offset(2
                                                                                   as
                                                                                   libc::c_int
                                                                                   as
                                                                                   isize),
                                          (*tri).plane.normal.x,
                                          (*tri).plane.normal.y,
                                          (*tri).plane.normal.z,
                                          (*tri).plane.originDist,
                                          (*cyl).pos.z as f32_0,
                                          (*cyl).pos.x as f32_0,
                                          &mut yIntersect, cylBottom, cylTop)
           != 0 {
        cylIntersectCenter.x = (*cyl).pos.x as f32_0;
        cylIntersectCenter.y = yIntersect;
        cylIntersectCenter.z = (*cyl).pos.z as f32_0;
        midpointv0v1.x =
            ((*tri).vtx[0 as libc::c_int as usize].x +
                 (*tri).vtx[1 as libc::c_int as usize].x) * 0.5f32;
        midpointv0v1.y =
            ((*tri).vtx[0 as libc::c_int as usize].y +
                 (*tri).vtx[1 as libc::c_int as usize].y) * 0.5f32;
        midpointv0v1.z =
            ((*tri).vtx[0 as libc::c_int as usize].z +
                 (*tri).vtx[1 as libc::c_int as usize].z) * 0.5f32;
        Math_Vec3f_Diff(&mut midpointv0v1, &mut cylIntersectCenter,
                        &mut diffMidpointIntersect);
        distFromCylYIntersectTov0v1 =
            sqrtf(diffMidpointIntersect.x * diffMidpointIntersect.x +
                      diffMidpointIntersect.z * diffMidpointIntersect.z);
        if fabsf(distFromCylYIntersectTov0v1) < 0.008f32 {
            Math_Vec3f_Copy(intersect, &mut midpointv0v1);
            return 1 as libc::c_int
        }
        radiusTodistFromCylYIntersectTov0v1 =
            (*cyl).radius as libc::c_int as libc::c_float /
                distFromCylYIntersectTov0v1;
        Math3D_PointOnInfiniteLine(&mut cylIntersectCenter,
                                   &mut diffMidpointIntersect,
                                   radiusTodistFromCylYIntersectTov0v1,
                                   intersect);
        return 1 as libc::c_int
    }
    bottomSphere.center.x = (*cyl).pos.x;
    topSphere.center.x = bottomSphere.center.x;
    bottomSphere.center.z = (*cyl).pos.z;
    topSphere.center.z = bottomSphere.center.z;
    topSphere.center.y = cylTop as s16;
    bottomSphere.center.y = cylBottom as s16;
    bottomSphere.radius = (*cyl).radius;
    topSphere.radius = bottomSphere.radius;
    if Math3D_TriVsSphIntersect(&mut topSphere, tri, intersect) != 0 ||
           Math3D_TriVsSphIntersect(&mut bottomSphere, tri, intersect) != 0 {
        return 1 as libc::c_int
    }
    return 0 as libc::c_int;
}
/*
 * Determines if `cyl` and `tri` are touching.
 */
#[no_mangle]
pub unsafe extern "C" fn Math3D_CylVsTri(mut cyl: *mut Cylinder16,
                                         mut tri: *mut TriNorm) -> s32 {
    let mut intersect: Vec3f = Vec3f{x: 0., y: 0., z: 0.,};
    return Math3D_CylTriVsIntersect(cyl, tri, &mut intersect);
}
/*
 * Deteremines if two spheres are touching.
 */
#[no_mangle]
pub unsafe extern "C" fn Math3D_SphVsSph(mut sphereA: *mut Sphere16,
                                         mut sphereB: *mut Sphere16) -> s32 {
    let mut overlapSize: f32_0 = 0.;
    return Math3D_SphVsSphOverlap(sphereA, sphereB, &mut overlapSize);
}
/*
 * Determines if two spheres are touching.  The amount that they're overlapping is placed in `overlapSize`
 */
#[no_mangle]
pub unsafe extern "C" fn Math3D_SphVsSphOverlap(mut sphereA: *mut Sphere16,
                                                mut sphereB: *mut Sphere16,
                                                mut overlapSize: *mut f32_0)
 -> s32 {
    let mut centerDist: f32_0 = 0.;
    return Math3D_SphVsSphOverlapCenter(sphereA, sphereB, overlapSize,
                                        &mut centerDist);
}
/*
 * Determines if two spheres are touching  The distance from the centers is placed in `centerDist`,
 * and the amount that they're overlapping is placed in `overlapSize`
 */
#[no_mangle]
pub unsafe extern "C" fn Math3D_SphVsSphOverlapCenter(mut sphereA:
                                                          *mut Sphere16,
                                                      mut sphereB:
                                                          *mut Sphere16,
                                                      mut overlapSize:
                                                          *mut f32_0,
                                                      mut centerDist:
                                                          *mut f32_0) -> s32 {
    let mut diff: Vec3f = Vec3f{x: 0., y: 0., z: 0.,};
    diff.x = (*sphereA).center.x as f32_0 - (*sphereB).center.x as f32_0;
    diff.y = (*sphereA).center.y as f32_0 - (*sphereB).center.y as f32_0;
    diff.z = (*sphereA).center.z as f32_0 - (*sphereB).center.z as f32_0;
    *centerDist =
        sqrt((diff.x * diff.x + diff.y * diff.y + diff.z * diff.z) as f64_0)
            as f32_0;
    *overlapSize =
        (*sphereA).radius as f32_0 + (*sphereB).radius as f32_0 - *centerDist;
    if *overlapSize > 0.008f32 { return 1 as libc::c_int }
    *overlapSize = 0.0f32;
    return 0 as libc::c_int;
}
/* *
 * Checks if `sph` and `cyl` are touching, output the amount of overlap to `overlapSize`
 */
#[no_mangle]
pub unsafe extern "C" fn Math3D_SphVsCylOverlapDist(mut sph: *mut Sphere16,
                                                    mut cyl: *mut Cylinder16,
                                                    mut overlapSize:
                                                        *mut f32_0) -> s32 {
    let mut centerDist: f32_0 = 0.;
    return Math3D_SphVsCylOverlapCenterDist(sph, cyl, overlapSize,
                                            &mut centerDist);
}
/* *
 * Checks if `sph` and `cyl` are touching, output the xz distance of the centers to `centerDist`, and the amount of
 * overlap to `overlapSize`
 */
#[no_mangle]
pub unsafe extern "C" fn Math3D_SphVsCylOverlapCenterDist(mut sph:
                                                              *mut Sphere16,
                                                          mut cyl:
                                                              *mut Cylinder16,
                                                          mut overlapSize:
                                                              *mut f32_0,
                                                          mut centerDist:
                                                              *mut f32_0)
 -> s32 {
    static mut cylf: Cylinderf =
        Cylinderf{radius: 0.,
                  height: 0.,
                  yShift: 0.,
                  pos: Vec3f{x: 0., y: 0., z: 0.,},};
    static mut sphf: Spheref =
        Spheref{center: Vec3f{x: 0., y: 0., z: 0.,}, radius: 0.,};
    let mut x: f32_0 = 0.;
    let mut z: f32_0 = 0.;
    let mut combinedRadius: f32_0 = 0.;
    let mut cylBottom: f32_0 = 0.;
    let mut cylTop: f32_0 = 0.;
    let mut sphBottom: f32_0 = 0.;
    let mut sphTop: f32_0 = 0.;
    if (*sph).radius as libc::c_int <= 0 as libc::c_int ||
           (*cyl).radius as libc::c_int <= 0 as libc::c_int {
        // either radius is 0
        return 0 as libc::c_int
    }
    sphf.center.y = (*sph).center.y as f32_0;
    sphf.radius = (*sph).radius as f32_0;
    cylf.pos.y = (*cyl).pos.y as f32_0;
    cylf.yShift = (*cyl).yShift as f32_0;
    cylf.height = (*cyl).height as f32_0;
    x =
        (*sph).center.x as f32_0 -
            (*cyl).pos.x as libc::c_int as libc::c_float;
    z =
        (*sph).center.z as f32_0 -
            (*cyl).pos.z as libc::c_int as libc::c_float;
    combinedRadius =
        (*sph).radius as f32_0 +
            (*cyl).radius as libc::c_int as libc::c_float;
    *centerDist = sqrtf(x * x + z * z);
    if combinedRadius < *centerDist {
        // if the combined radii is less than the distance to the centers, they cannot be touching.
        return 0 as libc::c_int
    }
    cylBottom = cylf.pos.y + cylf.yShift;
    cylTop = cylBottom + cylf.height;
    sphBottom = sphf.center.y - sphf.radius;
    sphTop = sphf.center.y + sphf.radius;
    if sphTop >= cylBottom && sphBottom <= cylTop {
        // if the cylinder and sphere are intersecting on the xz plane, check if they're intersecting on
        // the y axis.
        *overlapSize = combinedRadius - *centerDist;
        return 1 as libc::c_int
    }
    return 0 as libc::c_int;
}
/*
 * returns 1 if cylinder `ca` is outside cylinder `cb`.
 * Sets `deadSpace` to the mininum space between the cylinders not occupied by the other.
 */
#[no_mangle]
pub unsafe extern "C" fn Math3D_CylOutsideCyl(mut ca: *mut Cylinder16,
                                              mut cb: *mut Cylinder16,
                                              mut deadSpace: *mut f32_0)
 -> s32 {
    let mut xzDist: f32_0 = 0.;
    return Math3D_CylOutsideCylDist(ca, cb, deadSpace, &mut xzDist);
}
/*
 * returns 1 if cylinder `ca` is outside cylinder `cb`.
 * Sets `xzDist` to the xz distance between the centers of the cylinders.
 * Sets `deadSpace` to the mininum space between the cylinders not occupied by the other.
 */
#[no_mangle]
pub unsafe extern "C" fn Math3D_CylOutsideCylDist(mut ca: *mut Cylinder16,
                                                  mut cb: *mut Cylinder16,
                                                  mut deadSpace: *mut f32_0,
                                                  mut xzDist: *mut f32_0)
 -> s32 {
    static mut caf: Cylinderf =
        Cylinderf{radius: 0.,
                  height: 0.,
                  yShift: 0.,
                  pos: Vec3f{x: 0., y: 0., z: 0.,},};
    static mut cbf: Cylinderf =
        Cylinderf{radius: 0.,
                  height: 0.,
                  yShift: 0.,
                  pos: Vec3f{x: 0., y: 0., z: 0.,},};
    Math_Vec3s_ToVec3f(&mut caf.pos, &mut (*ca).pos);
    caf.radius = (*ca).radius as f32_0;
    caf.yShift = (*ca).yShift as f32_0;
    caf.height = (*ca).height as f32_0;
    Math_Vec3s_ToVec3f(&mut cbf.pos, &mut (*cb).pos);
    cbf.radius = (*cb).radius as f32_0;
    cbf.yShift = (*cb).yShift as f32_0;
    cbf.height = (*cb).height as f32_0;
    *xzDist =
        sqrtf((caf.pos.x - cbf.pos.x) * (caf.pos.x - cbf.pos.x) +
                  (caf.pos.z - cbf.pos.z) * (caf.pos.z - cbf.pos.z));
    // The combined radix are within the xz distance
    if caf.radius + cbf.radius < *xzDist { return 0 as libc::c_int }
    // top of ca < bottom of cb or top of cb < bottom of ca
    if caf.pos.y + caf.yShift + caf.height < cbf.pos.y + cbf.yShift ||
           cbf.pos.y + cbf.yShift + cbf.height < caf.pos.y + caf.yShift {
        return 0 as libc::c_int
    }
    *deadSpace = caf.radius + cbf.radius - *xzDist;
    return 1 as libc::c_int;
}
/*
 * Determines if triangle `ta` intersects with triangle `tb` the point of
 * intersection is output to `intersect.
 * Returns 1 is the triangles intersect, 0 otherwise
 */
#[no_mangle]
pub unsafe extern "C" fn Math3D_TriVsTriIntersect(mut ta: *mut TriNorm,
                                                  mut tb: *mut TriNorm,
                                                  mut intersect: *mut Vec3f)
 -> s32 {
    let mut dist0: f32_0 = 0.;
    let mut dist1: f32_0 = 0.;
    let mut dist2: f32_0 = 0.;
    dist0 =
        Math3D_Plane(&mut (*ta).plane,
                     &mut *(*tb).vtx.as_mut_ptr().offset(0 as libc::c_int as
                                                             isize));
    dist1 =
        Math3D_Plane(&mut (*ta).plane,
                     &mut *(*tb).vtx.as_mut_ptr().offset(1 as libc::c_int as
                                                             isize));
    dist2 =
        Math3D_Plane(&mut (*ta).plane,
                     &mut *(*tb).vtx.as_mut_ptr().offset(2 as libc::c_int as
                                                             isize));
    if dist0 > 0.0f32 && dist1 > 0.0f32 && dist2 > 0.0f32 ||
           dist0 < 0.0f32 && dist1 < 0.0f32 && dist2 < 0.0f32 {
        return 0 as libc::c_int
    }
    dist0 =
        Math3D_Plane(&mut (*tb).plane,
                     &mut *(*ta).vtx.as_mut_ptr().offset(0 as libc::c_int as
                                                             isize));
    dist1 =
        Math3D_Plane(&mut (*tb).plane,
                     &mut *(*ta).vtx.as_mut_ptr().offset(1 as libc::c_int as
                                                             isize));
    dist2 =
        Math3D_Plane(&mut (*tb).plane,
                     &mut *(*ta).vtx.as_mut_ptr().offset(2 as libc::c_int as
                                                             isize));
    if dist0 > 0.0f32 && dist1 > 0.0f32 && dist2 > 0.0f32 ||
           dist0 < 0.0f32 && dist1 < 0.0f32 && dist2 < 0.0f32 {
        return 0 as libc::c_int
    }
    if Math3D_TriLineIntersect(&mut *(*tb).vtx.as_mut_ptr().offset(0 as
                                                                       libc::c_int
                                                                       as
                                                                       isize),
                               &mut *(*tb).vtx.as_mut_ptr().offset(1 as
                                                                       libc::c_int
                                                                       as
                                                                       isize),
                               &mut *(*tb).vtx.as_mut_ptr().offset(2 as
                                                                       libc::c_int
                                                                       as
                                                                       isize),
                               (*tb).plane.normal.x, (*tb).plane.normal.y,
                               (*tb).plane.normal.z, (*tb).plane.originDist,
                               &mut *(*ta).vtx.as_mut_ptr().offset(0 as
                                                                       libc::c_int
                                                                       as
                                                                       isize),
                               &mut *(*ta).vtx.as_mut_ptr().offset(1 as
                                                                       libc::c_int
                                                                       as
                                                                       isize),
                               intersect, 0 as libc::c_int) != 0 {
        return 1 as libc::c_int
    }
    if Math3D_TriLineIntersect(&mut *(*tb).vtx.as_mut_ptr().offset(0 as
                                                                       libc::c_int
                                                                       as
                                                                       isize),
                               &mut *(*tb).vtx.as_mut_ptr().offset(1 as
                                                                       libc::c_int
                                                                       as
                                                                       isize),
                               &mut *(*tb).vtx.as_mut_ptr().offset(2 as
                                                                       libc::c_int
                                                                       as
                                                                       isize),
                               (*tb).plane.normal.x, (*tb).plane.normal.y,
                               (*tb).plane.normal.z, (*tb).plane.originDist,
                               &mut *(*ta).vtx.as_mut_ptr().offset(1 as
                                                                       libc::c_int
                                                                       as
                                                                       isize),
                               &mut *(*ta).vtx.as_mut_ptr().offset(2 as
                                                                       libc::c_int
                                                                       as
                                                                       isize),
                               intersect, 0 as libc::c_int) != 0 {
        return 1 as libc::c_int
    }
    if Math3D_TriLineIntersect(&mut *(*tb).vtx.as_mut_ptr().offset(0 as
                                                                       libc::c_int
                                                                       as
                                                                       isize),
                               &mut *(*tb).vtx.as_mut_ptr().offset(1 as
                                                                       libc::c_int
                                                                       as
                                                                       isize),
                               &mut *(*tb).vtx.as_mut_ptr().offset(2 as
                                                                       libc::c_int
                                                                       as
                                                                       isize),
                               (*tb).plane.normal.x, (*tb).plane.normal.y,
                               (*tb).plane.normal.z, (*tb).plane.originDist,
                               &mut *(*ta).vtx.as_mut_ptr().offset(2 as
                                                                       libc::c_int
                                                                       as
                                                                       isize),
                               &mut *(*ta).vtx.as_mut_ptr().offset(0 as
                                                                       libc::c_int
                                                                       as
                                                                       isize),
                               intersect, 0 as libc::c_int) != 0 {
        return 1 as libc::c_int
    }
    if Math3D_TriLineIntersect(&mut *(*ta).vtx.as_mut_ptr().offset(0 as
                                                                       libc::c_int
                                                                       as
                                                                       isize),
                               &mut *(*ta).vtx.as_mut_ptr().offset(1 as
                                                                       libc::c_int
                                                                       as
                                                                       isize),
                               &mut *(*ta).vtx.as_mut_ptr().offset(2 as
                                                                       libc::c_int
                                                                       as
                                                                       isize),
                               (*ta).plane.normal.x, (*ta).plane.normal.y,
                               (*ta).plane.normal.z, (*ta).plane.originDist,
                               &mut *(*tb).vtx.as_mut_ptr().offset(0 as
                                                                       libc::c_int
                                                                       as
                                                                       isize),
                               &mut *(*tb).vtx.as_mut_ptr().offset(1 as
                                                                       libc::c_int
                                                                       as
                                                                       isize),
                               intersect, 0 as libc::c_int) ==
           1 as libc::c_int {
        return 1 as libc::c_int
    }
    if Math3D_TriLineIntersect(&mut *(*ta).vtx.as_mut_ptr().offset(0 as
                                                                       libc::c_int
                                                                       as
                                                                       isize),
                               &mut *(*ta).vtx.as_mut_ptr().offset(1 as
                                                                       libc::c_int
                                                                       as
                                                                       isize),
                               &mut *(*ta).vtx.as_mut_ptr().offset(2 as
                                                                       libc::c_int
                                                                       as
                                                                       isize),
                               (*ta).plane.normal.x, (*ta).plane.normal.y,
                               (*ta).plane.normal.z, (*ta).plane.originDist,
                               &mut *(*tb).vtx.as_mut_ptr().offset(1 as
                                                                       libc::c_int
                                                                       as
                                                                       isize),
                               &mut *(*tb).vtx.as_mut_ptr().offset(2 as
                                                                       libc::c_int
                                                                       as
                                                                       isize),
                               intersect, 0 as libc::c_int) ==
           1 as libc::c_int {
        return 1 as libc::c_int
    }
    if Math3D_TriLineIntersect(&mut *(*ta).vtx.as_mut_ptr().offset(0 as
                                                                       libc::c_int
                                                                       as
                                                                       isize),
                               &mut *(*ta).vtx.as_mut_ptr().offset(1 as
                                                                       libc::c_int
                                                                       as
                                                                       isize),
                               &mut *(*ta).vtx.as_mut_ptr().offset(2 as
                                                                       libc::c_int
                                                                       as
                                                                       isize),
                               (*ta).plane.normal.x, (*ta).plane.normal.y,
                               (*ta).plane.normal.z, (*ta).plane.originDist,
                               &mut *(*tb).vtx.as_mut_ptr().offset(2 as
                                                                       libc::c_int
                                                                       as
                                                                       isize),
                               &mut *(*tb).vtx.as_mut_ptr().offset(0 as
                                                                       libc::c_int
                                                                       as
                                                                       isize),
                               intersect, 0 as libc::c_int) ==
           1 as libc::c_int {
        return 1 as libc::c_int
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn Math3D_XZInSphere(mut sphere: *mut Sphere16,
                                           mut x: f32_0, mut z: f32_0)
 -> s32 {
    let mut xDiff: f32_0 = 0.;
    let mut zDiff: f32_0 = 0.;
    xDiff = (*sphere).center.x as libc::c_int as libc::c_float - x;
    zDiff = (*sphere).center.z as libc::c_int as libc::c_float - z;
    if xDiff * xDiff + zDiff * zDiff <=
           ((*sphere).radius as libc::c_int * (*sphere).radius as libc::c_int)
               as libc::c_float {
        return 1 as libc::c_int
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn Math3D_XYInSphere(mut sphere: *mut Sphere16,
                                           mut x: f32_0, mut y: f32_0)
 -> s32 {
    let mut xDiff: f32_0 = 0.;
    let mut yDiff: f32_0 = 0.;
    xDiff = (*sphere).center.x as libc::c_int as libc::c_float - x;
    yDiff = (*sphere).center.y as libc::c_int as libc::c_float - y;
    if xDiff * xDiff + yDiff * yDiff <=
           ((*sphere).radius as libc::c_int * (*sphere).radius as libc::c_int)
               as libc::c_float {
        return 1 as libc::c_int
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn Math3D_YZInSphere(mut sphere: *mut Sphere16,
                                           mut y: f32_0, mut z: f32_0)
 -> s32 {
    let mut yDiff: f32_0 = 0.;
    let mut zDiff: f32_0 = 0.;
    yDiff = (*sphere).center.y as libc::c_int as libc::c_float - y;
    zDiff = (*sphere).center.z as libc::c_int as libc::c_float - z;
    if yDiff * yDiff + zDiff * zDiff <=
           ((*sphere).radius as libc::c_int * (*sphere).radius as libc::c_int)
               as libc::c_float {
        return 1 as libc::c_int
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn Math3D_DrawSphere(mut globalCtx: *mut GlobalContext,
                                           mut sph: *mut Sphere16) {
}
#[no_mangle]
pub unsafe extern "C" fn Math3D_DrawCylinder(mut globalCtx:
                                                 *mut GlobalContext,
                                             mut cyl: *mut Cylinder16) {
}
