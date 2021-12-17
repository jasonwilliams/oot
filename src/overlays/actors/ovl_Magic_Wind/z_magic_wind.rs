#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case,
         non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(const_raw_ptr_to_usize_cast, const_transmute, register_tool)]
extern "C" {
    #[no_mangle]
    fn osSyncPrintf(fmt: *const libc::c_char, _: ...);
    #[no_mangle]
    fn LogUtils_LogThreadId(name: *const libc::c_char, line: s32);
    #[no_mangle]
    fn Actor_Kill(actor: *mut Actor);
    #[no_mangle]
    fn func_8002F7DC(actor: *mut Actor, sfxId: u16_0);
    #[no_mangle]
    fn SkelCurve_Init(globalCtx: *mut GlobalContext,
                      skelCurve: *mut SkelAnimeCurve,
                      limbListSeg: *mut SkelCurveLimbList,
                      transUpdIdx: *mut TransformUpdateIndex) -> s32;
    #[no_mangle]
    fn SkelCurve_Destroy(globalCtx: *mut GlobalContext,
                         skelCurve: *mut SkelAnimeCurve);
    #[no_mangle]
    fn SkelCurve_SetAnim(skelCurve: *mut SkelAnimeCurve,
                         transUpdIdx: *mut TransformUpdateIndex, arg2: f32_0,
                         animFinalFrame: f32_0, animCurFrame: f32_0,
                         animSpeed: f32_0);
    #[no_mangle]
    fn SkelCurve_Update(globalCtx: *mut GlobalContext,
                        skelCurve: *mut SkelAnimeCurve) -> s32;
    #[no_mangle]
    fn SkelCurve_Draw(actor: *mut Actor, globalCtx: *mut GlobalContext,
                      skelCurve: *mut SkelAnimeCurve,
                      overrideLimbDraw: OverrideCurveLimbDraw,
                      postLimbDraw: PostCurveLimbDraw, lod: s32,
                      data: *mut libc::c_void);
    #[no_mangle]
    fn func_800876C8(globalCtx: *mut GlobalContext);
    #[no_mangle]
    fn Gfx_CallSetupDL(gfx: *mut Gfx, i: u32_0) -> *mut Gfx;
    #[no_mangle]
    fn Gfx_TwoTexScroll(gfxCtx: *mut GraphicsContext, tile1: s32, x1: u32_0,
                        y1: u32_0, width1: s32, height1: s32, tile2: s32,
                        x2: u32_0, y2: u32_0, width2: s32, height2: s32)
     -> *mut Gfx;
    #[no_mangle]
    fn Graph_OpenDisps(dispRefs: *mut *mut Gfx, gfxCtx: *mut GraphicsContext,
                       file: *const libc::c_char, line: s32);
    #[no_mangle]
    fn Graph_CloseDisps(dispRefs: *mut *mut Gfx, gfxCtx: *mut GraphicsContext,
                        file: *const libc::c_char, line: s32);
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
pub struct TransformData {
    pub unk_00: u16_0,
    pub unk_02: s16,
    pub unk_04: s16,
    pub unk_06: s16,
    pub unk_08: f32_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct TransformUpdateIndex {
    pub refIndex: *mut u8_0,
    pub transformData: *mut TransformData,
    pub copyValues: *mut s16,
    pub unk_0C: s16,
    pub unk_0E: s16,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SkelCurveLimb {
    pub firstChildIdx: u8_0,
    pub nextLimbIdx: u8_0,
    pub dList: [*mut Gfx; 2],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SkelCurveLimbList {
    pub limbs: *mut *mut SkelCurveLimb,
    pub limbCount: u8_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct LimbTransform {
    pub scale: Vec3s,
    pub rot: Vec3s,
    pub pos: Vec3s,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SkelAnimeCurve {
    pub limbCount: u8_0,
    pub limbList: *mut *mut SkelCurveLimb,
    pub transUpdIdx: *mut TransformUpdateIndex,
    pub unk_0C: f32_0,
    pub animFinalFrame: f32_0,
    pub animSpeed: f32_0,
    pub animCurFrame: f32_0,
    pub transforms: *mut LimbTransform,
}
pub type OverrideCurveLimbDraw
    =
    Option<unsafe extern "C" fn(_: *mut GlobalContext, _: *mut SkelAnimeCurve,
                                _: s32, _: *mut libc::c_void) -> s32>;
pub type PostCurveLimbDraw
    =
    Option<unsafe extern "C" fn(_: *mut GlobalContext, _: *mut SkelAnimeCurve,
                                _: s32, _: *mut libc::c_void) -> ()>;
pub type C2RustUnnamed_14 = libc::c_uint;
pub const ACTORCAT_CHEST: C2RustUnnamed_14 = 11;
pub const ACTORCAT_DOOR: C2RustUnnamed_14 = 10;
pub const ACTORCAT_BOSS: C2RustUnnamed_14 = 9;
pub const ACTORCAT_MISC: C2RustUnnamed_14 = 8;
pub const ACTORCAT_ITEMACTION: C2RustUnnamed_14 = 7;
pub const ACTORCAT_PROP: C2RustUnnamed_14 = 6;
pub const ACTORCAT_ENEMY: C2RustUnnamed_14 = 5;
pub const ACTORCAT_NPC: C2RustUnnamed_14 = 4;
pub const ACTORCAT_EXPLOSIVE: C2RustUnnamed_14 = 3;
pub const ACTORCAT_PLAYER: C2RustUnnamed_14 = 2;
pub const ACTORCAT_BG: C2RustUnnamed_14 = 1;
pub const ACTORCAT_SWITCH: C2RustUnnamed_14 = 0;
pub type C2RustUnnamed_15 = libc::c_uint;
pub const ACTOR_ID_MAX: C2RustUnnamed_15 = 471;
pub const ACTOR_OBJ_WARP2BLOCK: C2RustUnnamed_15 = 470;
pub const ACTOR_BG_JYA_BLOCK: C2RustUnnamed_15 = 469;
pub const ACTOR_EN_MM2: C2RustUnnamed_15 = 468;
pub const ACTOR_EN_ZL4: C2RustUnnamed_15 = 467;
pub const ACTOR_OBJ_HAMISHI: C2RustUnnamed_15 = 466;
pub const ACTOR_OBJ_TIMEBLOCK: C2RustUnnamed_15 = 465;
pub const ACTOR_EN_GE3: C2RustUnnamed_15 = 464;
pub const ACTOR_OBJ_MAKEKINSUTA: C2RustUnnamed_15 = 463;
pub const ACTOR_EN_ZO: C2RustUnnamed_15 = 462;
pub const ACTOR_BG_MENKURI_NISEKABE: C2RustUnnamed_15 = 461;
pub const ACTOR_EN_EG: C2RustUnnamed_15 = 460;
pub const ACTOR_OCEFF_WIPE4: C2RustUnnamed_15 = 459;
pub const ACTOR_EN_KAKASI3: C2RustUnnamed_15 = 458;
pub const ACTOR_EN_KAKASI2: C2RustUnnamed_15 = 457;
pub const ACTOR_BG_ICE_SHUTTER: C2RustUnnamed_15 = 456;
pub const ACTOR_BG_ICE_TURARA: C2RustUnnamed_15 = 455;
pub const ACTOR_EN_COW: C2RustUnnamed_15 = 454;
pub const ACTOR_EN_MA3: C2RustUnnamed_15 = 453;
pub const ACTOR_BG_SPOT18_SHUTTER: C2RustUnnamed_15 = 452;
pub const ACTOR_BG_SPOT18_FUTA: C2RustUnnamed_15 = 451;
pub const ACTOR_BG_SPOT11_OASIS: C2RustUnnamed_15 = 450;
pub const ACTOR_DOOR_KILLER: C2RustUnnamed_15 = 449;
pub const ACTOR_EN_CROW: C2RustUnnamed_15 = 448;
pub const ACTOR_EN_PO_DESERT: C2RustUnnamed_15 = 447;
pub const ACTOR_EN_WALL_TUBO: C2RustUnnamed_15 = 446;
pub const ACTOR_BG_BOWL_WALL: C2RustUnnamed_15 = 445;
pub const ACTOR_EN_DAIKU_KAKARIKO: C2RustUnnamed_15 = 444;
pub const ACTOR_BG_MIZU_SHUTTER: C2RustUnnamed_15 = 443;
pub const ACTOR_BG_MIZU_BWALL: C2RustUnnamed_15 = 442;
pub const ACTOR_EN_GS: C2RustUnnamed_15 = 441;
pub const ACTOR_EN_GB: C2RustUnnamed_15 = 440;
pub const ACTOR_BG_GND_ICEBLOCK: C2RustUnnamed_15 = 439;
pub const ACTOR_BG_GND_NISEKABE: C2RustUnnamed_15 = 438;
pub const ACTOR_BG_GND_SOULMEIRO: C2RustUnnamed_15 = 437;
pub const ACTOR_BG_GND_DARKMEIRO: C2RustUnnamed_15 = 436;
pub const ACTOR_BG_GND_FIREMEIRO: C2RustUnnamed_15 = 435;
pub const ACTOR_DEMO_GEFF: C2RustUnnamed_15 = 434;
pub const ACTOR_DEMO_GJ: C2RustUnnamed_15 = 433;
pub const ACTOR_EN_SKB: C2RustUnnamed_15 = 432;
pub const ACTOR_EN_WF: C2RustUnnamed_15 = 431;
pub const ACTOR_EN_GO2: C2RustUnnamed_15 = 430;
pub const ACTOR_EN_MU: C2RustUnnamed_15 = 429;
pub const ACTOR_EN_TG: C2RustUnnamed_15 = 428;
pub const ACTOR_OBJ_MURE3: C2RustUnnamed_15 = 427;
pub const ACTOR_UNSET_1AA: C2RustUnnamed_15 = 426;
pub const ACTOR_BG_SPOT17_BAKUDANKABE: C2RustUnnamed_15 = 425;
pub const ACTOR_BG_SPOT08_BAKUDANKABE: C2RustUnnamed_15 = 424;
pub const ACTOR_DEMO_KEKKAI: C2RustUnnamed_15 = 423;
pub const ACTOR_EN_HS2: C2RustUnnamed_15 = 422;
pub const ACTOR_BG_BOM_GUARD: C2RustUnnamed_15 = 421;
pub const ACTOR_EN_GUEST: C2RustUnnamed_15 = 420;
pub const ACTOR_EN_DNT_NOMAL: C2RustUnnamed_15 = 419;
pub const ACTOR_EN_DNT_JIJI: C2RustUnnamed_15 = 418;
pub const ACTOR_EN_DNT_DEMO: C2RustUnnamed_15 = 417;
pub const ACTOR_OBJ_KIBAKO2: C2RustUnnamed_15 = 416;
pub const ACTOR_BG_SPOT11_BAKUDANKABE: C2RustUnnamed_15 = 415;
pub const ACTOR_OBJ_COMB: C2RustUnnamed_15 = 414;
pub const ACTOR_BG_SPOT01_OBJECTS2: C2RustUnnamed_15 = 413;
pub const ACTOR_EN_SI: C2RustUnnamed_15 = 412;
pub const ACTOR_EN_DOG: C2RustUnnamed_15 = 411;
pub const ACTOR_EN_NIW_GIRL: C2RustUnnamed_15 = 410;
pub const ACTOR_OCEFF_WIPE3: C2RustUnnamed_15 = 409;
pub const ACTOR_OCEFF_WIPE2: C2RustUnnamed_15 = 408;
pub const ACTOR_EN_GELDB: C2RustUnnamed_15 = 407;
pub const ACTOR_EN_IT: C2RustUnnamed_15 = 406;
pub const ACTOR_EN_SHOPNUTS: C2RustUnnamed_15 = 405;
pub const ACTOR_BG_SPOT00_BREAK: C2RustUnnamed_15 = 404;
pub const ACTOR_EN_NUTSBALL: C2RustUnnamed_15 = 403;
pub const ACTOR_EN_HINTNUTS: C2RustUnnamed_15 = 402;
pub const ACTOR_BG_SPOT12_SAKU: C2RustUnnamed_15 = 401;
pub const ACTOR_BG_SPOT12_GATE: C2RustUnnamed_15 = 400;
pub const ACTOR_BG_JYA_HAHENIRON: C2RustUnnamed_15 = 399;
pub const ACTOR_BG_JYA_1FLIFT: C2RustUnnamed_15 = 398;
pub const ACTOR_BG_SPOT05_SOKO: C2RustUnnamed_15 = 397;
pub const ACTOR_EN_WEIYER: C2RustUnnamed_15 = 396;
pub const ACTOR_OCEFF_STORM: C2RustUnnamed_15 = 395;
pub const ACTOR_OCEFF_WIPE: C2RustUnnamed_15 = 394;
pub const ACTOR_EN_STH: C2RustUnnamed_15 = 393;
pub const ACTOR_EN_SSH: C2RustUnnamed_15 = 392;
pub const ACTOR_OBJ_ROOMTIMER: C2RustUnnamed_15 = 391;
pub const ACTOR_EN_GE2: C2RustUnnamed_15 = 390;
pub const ACTOR_EN_WONDER_TALK2: C2RustUnnamed_15 = 389;
pub const ACTOR_EN_DY_EXTRA: C2RustUnnamed_15 = 388;
pub const ACTOR_SHOT_SUN: C2RustUnnamed_15 = 387;
pub const ACTOR_DEMO_EC: C2RustUnnamed_15 = 386;
pub const ACTOR_EN_TORCH: C2RustUnnamed_15 = 385;
pub const ACTOR_UNSET_180: C2RustUnnamed_15 = 384;
pub const ACTOR_END_TITLE: C2RustUnnamed_15 = 383;
pub const ACTOR_OCEFF_SPOT: C2RustUnnamed_15 = 382;
pub const ACTOR_OBJ_MAKEOSHIHIKI: C2RustUnnamed_15 = 381;
pub const ACTOR_EN_TAKARA_MAN: C2RustUnnamed_15 = 380;
pub const ACTOR_EN_KAKASI: C2RustUnnamed_15 = 379;
pub const ACTOR_BOSS_GANON2: C2RustUnnamed_15 = 378;
pub const ACTOR_EN_ZL3: C2RustUnnamed_15 = 377;
pub const ACTOR_EN_HEISHI4: C2RustUnnamed_15 = 376;
pub const ACTOR_BG_ZG: C2RustUnnamed_15 = 375;
pub const ACTOR_EFC_ERUPC: C2RustUnnamed_15 = 374;
pub const ACTOR_EN_PO_FIELD: C2RustUnnamed_15 = 373;
pub const ACTOR_DEMO_GT: C2RustUnnamed_15 = 372;
pub const ACTOR_ELF_MSG2: C2RustUnnamed_15 = 371;
pub const ACTOR_DOOR_GERUDO: C2RustUnnamed_15 = 370;
pub const ACTOR_EN_MAG: C2RustUnnamed_15 = 369;
pub const ACTOR_EN_OKARINA_EFFECT: C2RustUnnamed_15 = 368;
pub const ACTOR_EN_GANON_MANT: C2RustUnnamed_15 = 367;
pub const ACTOR_EN_HY: C2RustUnnamed_15 = 366;
pub const ACTOR_EN_MD: C2RustUnnamed_15 = 365;
pub const ACTOR_EN_CS: C2RustUnnamed_15 = 364;
pub const ACTOR_EN_JSJUTAN: C2RustUnnamed_15 = 363;
pub const ACTOR_EN_JS: C2RustUnnamed_15 = 362;
pub const ACTOR_BG_JYA_IRONOBJ: C2RustUnnamed_15 = 361;
pub const ACTOR_EN_EX_ITEM: C2RustUnnamed_15 = 360;
pub const ACTOR_EN_ANI: C2RustUnnamed_15 = 359;
pub const ACTOR_BG_SST_FLOOR: C2RustUnnamed_15 = 358;
pub const ACTOR_EN_WEATHER_TAG: C2RustUnnamed_15 = 357;
pub const ACTOR_EN_KZ: C2RustUnnamed_15 = 356;
pub const ACTOR_EN_KO: C2RustUnnamed_15 = 355;
pub const ACTOR_EN_MM: C2RustUnnamed_15 = 354;
pub const ACTOR_UNSET_161: C2RustUnnamed_15 = 353;
pub const ACTOR_EN_STREAM: C2RustUnnamed_15 = 352;
pub const ACTOR_EN_SIOFUKI: C2RustUnnamed_15 = 351;
pub const ACTOR_EN_GANON_ORGAN: C2RustUnnamed_15 = 350;
pub const ACTOR_UNSET_15D: C2RustUnnamed_15 = 349;
pub const ACTOR_BG_SPOT18_BASKET: C2RustUnnamed_15 = 348;
pub const ACTOR_BG_JYA_BOMBIWA: C2RustUnnamed_15 = 347;
pub const ACTOR_BG_JYA_AMISHUTTER: C2RustUnnamed_15 = 346;
pub const ACTOR_BG_JYA_BOMBCHUIWA: C2RustUnnamed_15 = 345;
pub const ACTOR_BG_JYA_BIGMIRROR: C2RustUnnamed_15 = 344;
pub const ACTOR_BG_JYA_LIFT: C2RustUnnamed_15 = 343;
pub const ACTOR_BG_JYA_MEGAMI: C2RustUnnamed_15 = 342;
pub const ACTOR_EN_CHANGER: C2RustUnnamed_15 = 341;
pub const ACTOR_UNSET_154: C2RustUnnamed_15 = 340;
pub const ACTOR_EN_FU: C2RustUnnamed_15 = 339;
pub const ACTOR_EN_GO: C2RustUnnamed_15 = 338;
pub const ACTOR_OBJ_MURE2: C2RustUnnamed_15 = 337;
pub const ACTOR_OBJ_LIGHTSWITCH: C2RustUnnamed_15 = 336;
pub const ACTOR_OBJ_HANA: C2RustUnnamed_15 = 335;
pub const ACTOR_EN_ISHI: C2RustUnnamed_15 = 334;
pub const ACTOR_EN_OWL: C2RustUnnamed_15 = 333;
pub const ACTOR_EN_BOM_BOWL_PIT: C2RustUnnamed_15 = 332;
pub const ACTOR_EN_BOM_BOWL_MAN: C2RustUnnamed_15 = 331;
pub const ACTOR_EN_MK: C2RustUnnamed_15 = 330;
pub const ACTOR_EN_DS: C2RustUnnamed_15 = 329;
pub const ACTOR_BG_GJYO_BRIDGE: C2RustUnnamed_15 = 328;
pub const ACTOR_EN_WONDER_TALK: C2RustUnnamed_15 = 327;
pub const ACTOR_EN_SA: C2RustUnnamed_15 = 326;
pub const ACTOR_BG_SPOT01_IDOSOKO: C2RustUnnamed_15 = 325;
pub const ACTOR_EN_ATTACK_NIW: C2RustUnnamed_15 = 324;
pub const ACTOR_EN_SYATEKI_NIW: C2RustUnnamed_15 = 323;
pub const ACTOR_EN_HEISHI3: C2RustUnnamed_15 = 322;
pub const ACTOR_EN_KANBAN: C2RustUnnamed_15 = 321;
pub const ACTOR_BG_INGATE: C2RustUnnamed_15 = 320;
pub const ACTOR_EN_HS: C2RustUnnamed_15 = 319;
pub const ACTOR_EN_MS: C2RustUnnamed_15 = 318;
pub const ACTOR_EN_GM: C2RustUnnamed_15 = 317;
pub const ACTOR_EN_NIW_LADY: C2RustUnnamed_15 = 316;
pub const ACTOR_EN_CLEAR_TAG: C2RustUnnamed_15 = 315;
pub const ACTOR_EN_SDA: C2RustUnnamed_15 = 314;
pub const ACTOR_OBJ_BLOCKSTOP: C2RustUnnamed_15 = 313;
pub const ACTOR_EN_GE1: C2RustUnnamed_15 = 312;
pub const ACTOR_ITEM_INBOX: C2RustUnnamed_15 = 311;
pub const ACTOR_EN_BLKOBJ: C2RustUnnamed_15 = 310;
pub const ACTOR_EN_NWC: C2RustUnnamed_15 = 309;
pub const ACTOR_UNSET_134: C2RustUnnamed_15 = 308;
pub const ACTOR_EN_DAIKU: C2RustUnnamed_15 = 307;
pub const ACTOR_EN_TORYO: C2RustUnnamed_15 = 306;
pub const ACTOR_EN_EX_RUPPY: C2RustUnnamed_15 = 305;
pub const ACTOR_EN_GOROIWA: C2RustUnnamed_15 = 304;
pub const ACTOR_EN_YABUSAME_MARK: C2RustUnnamed_15 = 303;
pub const ACTOR_EN_OKARINA_TAG: C2RustUnnamed_15 = 302;
pub const ACTOR_OBJ_HSBLOCK: C2RustUnnamed_15 = 301;
pub const ACTOR_OBJ_LIFT: C2RustUnnamed_15 = 300;
pub const ACTOR_OBJ_ELEVATOR: C2RustUnnamed_15 = 299;
pub const ACTOR_OBJ_SWITCH: C2RustUnnamed_15 = 298;
pub const ACTOR_UNSET_129: C2RustUnnamed_15 = 297;
pub const ACTOR_UNSET_128: C2RustUnnamed_15 = 296;
pub const ACTOR_OBJ_BOMBIWA: C2RustUnnamed_15 = 295;
pub const ACTOR_OBJ_BEAN: C2RustUnnamed_15 = 294;
pub const ACTOR_EN_KUSA: C2RustUnnamed_15 = 293;
pub const ACTOR_EN_DIVING_GAME: C2RustUnnamed_15 = 292;
pub const ACTOR_BG_RELAY_OBJECTS: C2RustUnnamed_15 = 291;
pub const ACTOR_EN_PO_RELAY: C2RustUnnamed_15 = 290;
pub const ACTOR_EN_FZ: C2RustUnnamed_15 = 289;
pub const ACTOR_BG_SPOT07_TAKI: C2RustUnnamed_15 = 288;
pub const ACTOR_BG_SPOT03_TAKI: C2RustUnnamed_15 = 287;
pub const ACTOR_OBJ_ICE_POLY: C2RustUnnamed_15 = 286;
pub const ACTOR_EN_TUBO_TRAP: C2RustUnnamed_15 = 285;
pub const ACTOR_EN_HONOTRAP: C2RustUnnamed_15 = 284;
pub const ACTOR_ELF_MSG: C2RustUnnamed_15 = 283;
pub const ACTOR_EN_DNS: C2RustUnnamed_15 = 282;
pub const ACTOR_DEMO_SHD: C2RustUnnamed_15 = 281;
pub const ACTOR_DEMO_EXT: C2RustUnnamed_15 = 280;
pub const ACTOR_EN_G_SWITCH: C2RustUnnamed_15 = 279;
pub const ACTOR_EN_SKJNEEDLE: C2RustUnnamed_15 = 278;
pub const ACTOR_EN_SKJ: C2RustUnnamed_15 = 277;
pub const ACTOR_DEMO_IK: C2RustUnnamed_15 = 276;
pub const ACTOR_EN_IK: C2RustUnnamed_15 = 275;
pub const ACTOR_EN_WONDER_ITEM: C2RustUnnamed_15 = 274;
pub const ACTOR_OBJ_TSUBO: C2RustUnnamed_15 = 273;
pub const ACTOR_OBJ_KIBAKO: C2RustUnnamed_15 = 272;
pub const ACTOR_ITEM_ETCETERA: C2RustUnnamed_15 = 271;
pub const ACTOR_UNSET_10E: C2RustUnnamed_15 = 270;
pub const ACTOR_UNSET_10D: C2RustUnnamed_15 = 269;
pub const ACTOR_ARROW_LIGHT: C2RustUnnamed_15 = 268;
pub const ACTOR_ARROW_ICE: C2RustUnnamed_15 = 267;
pub const ACTOR_ARROW_FIRE: C2RustUnnamed_15 = 266;
pub const ACTOR_UNSET_109: C2RustUnnamed_15 = 265;
pub const ACTOR_BG_UMAJUMP: C2RustUnnamed_15 = 264;
pub const ACTOR_BG_SPOT15_RRBOX: C2RustUnnamed_15 = 263;
pub const ACTOR_BG_GANON_OTYUKA: C2RustUnnamed_15 = 262;
pub const ACTOR_BG_PO_SYOKUDAI: C2RustUnnamed_15 = 261;
pub const ACTOR_BG_SPOT01_IDOMIZU: C2RustUnnamed_15 = 260;
pub const ACTOR_BG_SPOT01_IDOHASHIRA: C2RustUnnamed_15 = 259;
pub const ACTOR_BG_SPOT01_FUSYA: C2RustUnnamed_15 = 258;
pub const ACTOR_EFF_DUST: C2RustUnnamed_15 = 257;
pub const ACTOR_BG_GATE_SHUTTER: C2RustUnnamed_15 = 256;
pub const ACTOR_OBJ_OSHIHIKI: C2RustUnnamed_15 = 255;
pub const ACTOR_FISHING: C2RustUnnamed_15 = 254;
pub const ACTOR_BG_JYA_KANAAMI: C2RustUnnamed_15 = 253;
pub const ACTOR_BG_JYA_COBRA: C2RustUnnamed_15 = 252;
pub const ACTOR_UNSET_FB: C2RustUnnamed_15 = 251;
pub const ACTOR_BG_JYA_ZURERUKABE: C2RustUnnamed_15 = 250;
pub const ACTOR_BG_JYA_GOROIWA: C2RustUnnamed_15 = 249;
pub const ACTOR_BG_SPOT15_SAKU: C2RustUnnamed_15 = 248;
pub const ACTOR_BG_HAKA_GATE: C2RustUnnamed_15 = 247;
pub const ACTOR_EN_ANUBICE_TAG: C2RustUnnamed_15 = 246;
pub const ACTOR_DEMO_6K: C2RustUnnamed_15 = 245;
pub const ACTOR_MAGIC_DARK: C2RustUnnamed_15 = 244;
pub const ACTOR_UNSET_F3: C2RustUnnamed_15 = 243;
pub const ACTOR_UNSET_F2: C2RustUnnamed_15 = 242;
pub const ACTOR_ITEM_OCARINA: C2RustUnnamed_15 = 241;
pub const ACTOR_EN_ICE_HONO: C2RustUnnamed_15 = 240;
pub const ACTOR_BG_ICE_SHELTER: C2RustUnnamed_15 = 239;
pub const ACTOR_ITEM_SHIELD: C2RustUnnamed_15 = 238;
pub const ACTOR_EN_FR: C2RustUnnamed_15 = 237;
pub const ACTOR_EN_NY: C2RustUnnamed_15 = 236;
pub const ACTOR_UNSET_EB: C2RustUnnamed_15 = 235;
pub const ACTOR_UNSET_EA: C2RustUnnamed_15 = 234;
pub const ACTOR_BOSS_SST: C2RustUnnamed_15 = 233;
pub const ACTOR_BOSS_GANON: C2RustUnnamed_15 = 232;
pub const ACTOR_EN_MA1: C2RustUnnamed_15 = 231;
pub const ACTOR_BG_BDAN_SWITCH: C2RustUnnamed_15 = 230;
pub const ACTOR_BG_SPOT16_DOUGHNUT: C2RustUnnamed_15 = 229;
pub const ACTOR_BG_MORI_IDOMIZU: C2RustUnnamed_15 = 228;
pub const ACTOR_BG_MORI_HASHIRA4: C2RustUnnamed_15 = 227;
pub const ACTOR_BG_MORI_HASHIGO: C2RustUnnamed_15 = 226;
pub const ACTOR_EN_ANUBICE_FIRE: C2RustUnnamed_15 = 225;
pub const ACTOR_EN_ANUBICE: C2RustUnnamed_15 = 224;
pub const ACTOR_EN_BX: C2RustUnnamed_15 = 223;
pub const ACTOR_EN_BA: C2RustUnnamed_15 = 222;
pub const ACTOR_EN_RR: C2RustUnnamed_15 = 221;
pub const ACTOR_BOSS_TW: C2RustUnnamed_15 = 220;
pub const ACTOR_EN_HORSE_GAME_CHECK: C2RustUnnamed_15 = 219;
pub const ACTOR_EN_BOM_CHU: C2RustUnnamed_15 = 218;
pub const ACTOR_EN_MA2: C2RustUnnamed_15 = 217;
pub const ACTOR_UNSET_D8: C2RustUnnamed_15 = 216;
pub const ACTOR_BG_HAKA_WATER: C2RustUnnamed_15 = 215;
pub const ACTOR_BG_ICE_OBJECTS: C2RustUnnamed_15 = 214;
pub const ACTOR_BG_SPOT06_OBJECTS: C2RustUnnamed_15 = 213;
pub const ACTOR_BG_MIZU_UZU: C2RustUnnamed_15 = 212;
pub const ACTOR_OBJ_DEKUJR: C2RustUnnamed_15 = 211;
pub const ACTOR_EN_RU2: C2RustUnnamed_15 = 210;
pub const ACTOR_BG_SPOT08_ICEBLOCK: C2RustUnnamed_15 = 209;
pub const ACTOR_BG_BOMBWALL: C2RustUnnamed_15 = 208;
pub const ACTOR_BG_HIDAN_KOWARERUKABE: C2RustUnnamed_15 = 207;
pub const ACTOR_UNSET_CE: C2RustUnnamed_15 = 206;
pub const ACTOR_BG_SPOT16_BOMBSTONE: C2RustUnnamed_15 = 205;
pub const ACTOR_EN_TR: C2RustUnnamed_15 = 204;
pub const ACTOR_EN_IN: C2RustUnnamed_15 = 203;
pub const ACTOR_DEMO_GO: C2RustUnnamed_15 = 202;
pub const ACTOR_DEMO_SA: C2RustUnnamed_15 = 201;
pub const ACTOR_BG_BDAN_OBJECTS: C2RustUnnamed_15 = 200;
pub const ACTOR_EN_KAREBABA: C2RustUnnamed_15 = 199;
pub const ACTOR_EN_BIGOKUTA: C2RustUnnamed_15 = 198;
pub const ACTOR_EN_SB: C2RustUnnamed_15 = 197;
pub const ACTOR_BOSS_MO: C2RustUnnamed_15 = 196;
pub const ACTOR_EN_NB: C2RustUnnamed_15 = 195;
pub const ACTOR_EN_TANA: C2RustUnnamed_15 = 194;
pub const ACTOR_EN_SYATEKI_MAN: C2RustUnnamed_15 = 193;
pub const ACTOR_EN_SYATEKI_ITM: C2RustUnnamed_15 = 192;
pub const ACTOR_BG_SPOT17_FUNEN: C2RustUnnamed_15 = 191;
pub const ACTOR_BG_HAKA_ZOU: C2RustUnnamed_15 = 190;
pub const ACTOR_BG_HAKA_HUTA: C2RustUnnamed_15 = 189;
pub const ACTOR_BG_HAKA_TRAP: C2RustUnnamed_15 = 188;
pub const ACTOR_BG_HAKA_TUBO: C2RustUnnamed_15 = 187;
pub const ACTOR_BOSS_VA: C2RustUnnamed_15 = 186;
pub const ACTOR_BG_SPOT18_OBJ: C2RustUnnamed_15 = 185;
pub const ACTOR_BG_SPOT09_OBJ: C2RustUnnamed_15 = 184;
pub const ACTOR_MIR_RAY: C2RustUnnamed_15 = 183;
pub const ACTOR_EN_BROB: C2RustUnnamed_15 = 182;
pub const ACTOR_EN_FIRE_ROCK: C2RustUnnamed_15 = 181;
pub const ACTOR_EN_ENCOUNT2: C2RustUnnamed_15 = 180;
pub const ACTOR_EN_HEISHI2: C2RustUnnamed_15 = 179;
pub const ACTOR_UNSET_B2: C2RustUnnamed_15 = 178;
pub const ACTOR_BG_HAKA_SGAMI: C2RustUnnamed_15 = 177;
pub const ACTOR_BG_HAKA_SHIP: C2RustUnnamed_15 = 176;
pub const ACTOR_BG_HAKA_MEGANEBG: C2RustUnnamed_15 = 175;
pub const ACTOR_BG_HAKA_MEGANE: C2RustUnnamed_15 = 174;
pub const ACTOR_EN_VB_BALL: C2RustUnnamed_15 = 173;
pub const ACTOR_BG_VB_SIMA: C2RustUnnamed_15 = 172;
pub const ACTOR_EN_FW: C2RustUnnamed_15 = 171;
pub const ACTOR_DEMO_TRE_LGT: C2RustUnnamed_15 = 170;
pub const ACTOR_DEMO_IM: C2RustUnnamed_15 = 169;
pub const ACTOR_DEMO_DU: C2RustUnnamed_15 = 168;
pub const ACTOR_EN_ENCOUNT1: C2RustUnnamed_15 = 167;
pub const ACTOR_EN_RL: C2RustUnnamed_15 = 166;
pub const ACTOR_EN_DHA: C2RustUnnamed_15 = 165;
pub const ACTOR_EN_DH: C2RustUnnamed_15 = 164;
pub const ACTOR_EN_FD_FIRE: C2RustUnnamed_15 = 163;
pub const ACTOR_BOSS_FD2: C2RustUnnamed_15 = 162;
pub const ACTOR_EN_RU1: C2RustUnnamed_15 = 161;
pub const ACTOR_UNSET_A0: C2RustUnnamed_15 = 160;
pub const ACTOR_MAGIC_FIRE: C2RustUnnamed_15 = 159;
pub const ACTOR_MAGIC_WIND: C2RustUnnamed_15 = 158;
pub const ACTOR_BG_HAKA: C2RustUnnamed_15 = 157;
pub const ACTOR_BG_SPOT02_OBJECTS: C2RustUnnamed_15 = 156;
pub const ACTOR_DOOR_ANA: C2RustUnnamed_15 = 155;
pub const ACTOR_EN_HORSE_LINK_CHILD: C2RustUnnamed_15 = 154;
pub const ACTOR_EN_FD: C2RustUnnamed_15 = 153;
pub const ACTOR_EN_DU: C2RustUnnamed_15 = 152;
pub const ACTOR_OBJECT_KANKYO: C2RustUnnamed_15 = 151;
pub const ACTOR_BOSS_FD: C2RustUnnamed_15 = 150;
pub const ACTOR_EN_SW: C2RustUnnamed_15 = 149;
pub const ACTOR_OBJ_MURE: C2RustUnnamed_15 = 148;
pub const ACTOR_BG_PO_EVENT: C2RustUnnamed_15 = 147;
pub const ACTOR_BG_HEAVY_BLOCK: C2RustUnnamed_15 = 146;
pub const ACTOR_EN_PO_SISTERS: C2RustUnnamed_15 = 145;
pub const ACTOR_EN_RD: C2RustUnnamed_15 = 144;
pub const ACTOR_EN_HEISHI1: C2RustUnnamed_15 = 143;
pub const ACTOR_EN_FLOORMAS: C2RustUnnamed_15 = 142;
pub const ACTOR_BG_HIDAN_FWBIG: C2RustUnnamed_15 = 141;
pub const ACTOR_DEMO_KANKYO: C2RustUnnamed_15 = 140;
pub const ACTOR_DEMO_EFFECT: C2RustUnnamed_15 = 139;
pub const ACTOR_EN_VM: C2RustUnnamed_15 = 138;
pub const ACTOR_BG_MORI_RAKKATENJO: C2RustUnnamed_15 = 137;
pub const ACTOR_BG_MORI_KAITENKABE: C2RustUnnamed_15 = 136;
pub const ACTOR_BG_MORI_ELEVATOR: C2RustUnnamed_15 = 135;
pub const ACTOR_BG_MORI_BIGST: C2RustUnnamed_15 = 134;
pub const ACTOR_EN_TK: C2RustUnnamed_15 = 133;
pub const ACTOR_EN_TA: C2RustUnnamed_15 = 132;
pub const ACTOR_UNSET_83: C2RustUnnamed_15 = 131;
pub const ACTOR_EN_VASE: C2RustUnnamed_15 = 130;
pub const ACTOR_EN_AROW_TRAP: C2RustUnnamed_15 = 129;
pub const ACTOR_EN_TRAP: C2RustUnnamed_15 = 128;
pub const ACTOR_UNSET_7F: C2RustUnnamed_15 = 127;
pub const ACTOR_UNSET_7E: C2RustUnnamed_15 = 126;
pub const ACTOR_EN_PU_BOX: C2RustUnnamed_15 = 125;
pub const ACTOR_EN_LIGHTBOX: C2RustUnnamed_15 = 124;
pub const ACTOR_UNSET_7B: C2RustUnnamed_15 = 123;
pub const ACTOR_UNSET_7A: C2RustUnnamed_15 = 122;
pub const ACTOR_UNSET_79: C2RustUnnamed_15 = 121;
pub const ACTOR_UNSET_78: C2RustUnnamed_15 = 120;
pub const ACTOR_EN_WOOD02: C2RustUnnamed_15 = 119;
pub const ACTOR_UNSET_76: C2RustUnnamed_15 = 118;
pub const ACTOR_UNSET_75: C2RustUnnamed_15 = 117;
pub const ACTOR_UNSET_74: C2RustUnnamed_15 = 116;
pub const ACTOR_UNSET_73: C2RustUnnamed_15 = 115;
pub const ACTOR_EN_BIRD: C2RustUnnamed_15 = 114;
pub const ACTOR_BG_HIDAN_HAMSTEP: C2RustUnnamed_15 = 113;
pub const ACTOR_DOOR_TOKI: C2RustUnnamed_15 = 112;
pub const ACTOR_BG_HIDAN_KOUSI: C2RustUnnamed_15 = 111;
pub const ACTOR_BG_MJIN: C2RustUnnamed_15 = 110;
pub const ACTOR_EN_FHG_FIRE: C2RustUnnamed_15 = 109;
pub const ACTOR_BG_TOKI_SWD: C2RustUnnamed_15 = 108;
pub const ACTOR_EN_YUKABYUN: C2RustUnnamed_15 = 107;
pub const ACTOR_BG_TOKI_HIKARI: C2RustUnnamed_15 = 106;
pub const ACTOR_EN_BB: C2RustUnnamed_15 = 105;
pub const ACTOR_BG_MORI_HINERI: C2RustUnnamed_15 = 104;
pub const ACTOR_EN_FHG: C2RustUnnamed_15 = 103;
pub const ACTOR_ARMS_HOOK: C2RustUnnamed_15 = 102;
pub const ACTOR_BG_MIZU_WATER: C2RustUnnamed_15 = 101;
pub const ACTOR_BG_MIZU_MOVEBG: C2RustUnnamed_15 = 100;
pub const ACTOR_EN_VALI: C2RustUnnamed_15 = 99;
pub const ACTOR_BG_MENKURI_EYE: C2RustUnnamed_15 = 98;
pub const ACTOR_BG_MENKURI_KAITEN: C2RustUnnamed_15 = 97;
pub const ACTOR_EN_DEKUNUTS: C2RustUnnamed_15 = 96;
pub const ACTOR_ITEM_B_HEART: C2RustUnnamed_15 = 95;
pub const ACTOR_OBJ_SYOKUDAI: C2RustUnnamed_15 = 94;
pub const ACTOR_DOOR_WARP1: C2RustUnnamed_15 = 93;
pub const ACTOR_BG_DDAN_KD: C2RustUnnamed_15 = 92;
pub const ACTOR_EN_HORSE_ZELDA: C2RustUnnamed_15 = 91;
pub const ACTOR_EN_JJ: C2RustUnnamed_15 = 90;
pub const ACTOR_BG_BREAKWALL: C2RustUnnamed_15 = 89;
pub const ACTOR_BG_DDAN_JD: C2RustUnnamed_15 = 88;
pub const ACTOR_EN_M_THUNDER: C2RustUnnamed_15 = 87;
pub const ACTOR_EN_M_FIRE1: C2RustUnnamed_15 = 86;
pub const ACTOR_EN_DEKUBABA: C2RustUnnamed_15 = 85;
pub const ACTOR_EN_AM: C2RustUnnamed_15 = 84;
pub const ACTOR_UNSET_53: C2RustUnnamed_15 = 83;
pub const ACTOR_BOSS_GANONDROF: C2RustUnnamed_15 = 82;
pub const ACTOR_BG_YDAN_MARUTA: C2RustUnnamed_15 = 81;
pub const ACTOR_BG_YDAN_HASI: C2RustUnnamed_15 = 80;
pub const ACTOR_EN_OE2: C2RustUnnamed_15 = 79;
pub const ACTOR_BG_HIDAN_FSLIFT: C2RustUnnamed_15 = 78;
pub const ACTOR_EN_ZL2: C2RustUnnamed_15 = 77;
pub const ACTOR_EN_BOMBF: C2RustUnnamed_15 = 76;
pub const ACTOR_EN_MB: C2RustUnnamed_15 = 75;
pub const ACTOR_BG_SPOT00_HANEBASI: C2RustUnnamed_15 = 74;
pub const ACTOR_BG_HIDAN_CURTAIN: C2RustUnnamed_15 = 73;
pub const ACTOR_EN_XC: C2RustUnnamed_15 = 72;
pub const ACTOR_BG_HIDAN_SYOKU: C2RustUnnamed_15 = 71;
pub const ACTOR_BG_HIDAN_SIMA: C2RustUnnamed_15 = 70;
pub const ACTOR_BG_HIDAN_SEKIZOU: C2RustUnnamed_15 = 69;
pub const ACTOR_BG_HIDAN_RSEKIZOU: C2RustUnnamed_15 = 68;
pub const ACTOR_BG_HIDAN_ROCK: C2RustUnnamed_15 = 67;
pub const ACTOR_EN_HORSE_GANON: C2RustUnnamed_15 = 66;
pub const ACTOR_BG_HIDAN_HROCK: C2RustUnnamed_15 = 65;
pub const ACTOR_BG_HIDAN_DALM: C2RustUnnamed_15 = 64;
pub const ACTOR_BG_DODOAGO: C2RustUnnamed_15 = 63;
pub const ACTOR_BG_TREEMOUTH: C2RustUnnamed_15 = 62;
pub const ACTOR_EN_OSSAN: C2RustUnnamed_15 = 61;
pub const ACTOR_EN_HORSE_NORMAL: C2RustUnnamed_15 = 60;
pub const ACTOR_EN_RIVER_SOUND: C2RustUnnamed_15 = 59;
pub const ACTOR_EN_EIYER: C2RustUnnamed_15 = 58;
pub const ACTOR_EN_A_OBJ: C2RustUnnamed_15 = 57;
pub const ACTOR_EN_BW: C2RustUnnamed_15 = 56;
pub const ACTOR_EN_ST: C2RustUnnamed_15 = 55;
pub const ACTOR_UNSET_36: C2RustUnnamed_15 = 54;
pub const ACTOR_EN_TP: C2RustUnnamed_15 = 53;
pub const ACTOR_EN_BILI: C2RustUnnamed_15 = 52;
pub const ACTOR_EN_TORCH2: C2RustUnnamed_15 = 51;
pub const ACTOR_EN_BOOM: C2RustUnnamed_15 = 50;
pub const ACTOR_UNSET_31: C2RustUnnamed_15 = 49;
pub const ACTOR_EN_BDFIRE: C2RustUnnamed_15 = 48;
pub const ACTOR_EN_DODOJR: C2RustUnnamed_15 = 47;
pub const ACTOR_DOOR_SHUTTER: C2RustUnnamed_15 = 46;
pub const ACTOR_EN_BUBBLE: C2RustUnnamed_15 = 45;
pub const ACTOR_BG_PUSHBOX: C2RustUnnamed_15 = 44;
pub const ACTOR_EN_GOMA: C2RustUnnamed_15 = 43;
pub const ACTOR_EN_VIEWER: C2RustUnnamed_15 = 42;
pub const ACTOR_EN_ZL1: C2RustUnnamed_15 = 41;
pub const ACTOR_BOSS_GOMA: C2RustUnnamed_15 = 40;
pub const ACTOR_BOSS_DODONGO: C2RustUnnamed_15 = 39;
pub const ACTOR_EN_HATA: C2RustUnnamed_15 = 38;
pub const ACTOR_EN_ZF: C2RustUnnamed_15 = 37;
pub const ACTOR_EN_SCENE_CHANGE: C2RustUnnamed_15 = 36;
pub const ACTOR_EN_HOLL: C2RustUnnamed_15 = 35;
pub const ACTOR_UNSET_22: C2RustUnnamed_15 = 34;
pub const ACTOR_EN_FISH: C2RustUnnamed_15 = 33;
pub const ACTOR_EN_INSECT: C2RustUnnamed_15 = 32;
pub const ACTOR_UNSET_1F: C2RustUnnamed_15 = 31;
pub const ACTOR_EN_BUTTE: C2RustUnnamed_15 = 30;
pub const ACTOR_EN_PEEHAT: C2RustUnnamed_15 = 29;
pub const ACTOR_EN_REEBA: C2RustUnnamed_15 = 28;
pub const ACTOR_EN_TITE: C2RustUnnamed_15 = 27;
pub const ACTOR_UNSET_1A: C2RustUnnamed_15 = 26;
pub const ACTOR_EN_NIW: C2RustUnnamed_15 = 25;
pub const ACTOR_EN_ELF: C2RustUnnamed_15 = 24;
pub const ACTOR_UNSET_17: C2RustUnnamed_15 = 23;
pub const ACTOR_EN_ARROW: C2RustUnnamed_15 = 22;
pub const ACTOR_EN_ITEM00: C2RustUnnamed_15 = 21;
pub const ACTOR_EN_HORSE: C2RustUnnamed_15 = 20;
pub const ACTOR_EN_FIREFLY: C2RustUnnamed_15 = 19;
pub const ACTOR_EN_DODONGO: C2RustUnnamed_15 = 18;
pub const ACTOR_EN_WALLMAS: C2RustUnnamed_15 = 17;
pub const ACTOR_EN_BOM: C2RustUnnamed_15 = 16;
pub const ACTOR_BG_YDAN_SP: C2RustUnnamed_15 = 15;
pub const ACTOR_EN_OKUTA: C2RustUnnamed_15 = 14;
pub const ACTOR_EN_POH: C2RustUnnamed_15 = 13;
pub const ACTOR_BG_HIDAN_FIREWALL: C2RustUnnamed_15 = 12;
pub const ACTOR_BG_DY_YOSEIZO: C2RustUnnamed_15 = 11;
pub const ACTOR_EN_BOX: C2RustUnnamed_15 = 10;
pub const ACTOR_EN_DOOR: C2RustUnnamed_15 = 9;
pub const ACTOR_EN_LIGHT: C2RustUnnamed_15 = 8;
pub const ACTOR_EN_PART: C2RustUnnamed_15 = 7;
pub const ACTOR_UNSET_6: C2RustUnnamed_15 = 6;
pub const ACTOR_UNSET_5: C2RustUnnamed_15 = 5;
pub const ACTOR_EN_GIRLA: C2RustUnnamed_15 = 4;
pub const ACTOR_UNSET_3: C2RustUnnamed_15 = 3;
pub const ACTOR_EN_TEST: C2RustUnnamed_15 = 2;
pub const ACTOR_UNSET_1: C2RustUnnamed_15 = 1;
pub const ACTOR_PLAYER: C2RustUnnamed_15 = 0;
pub type C2RustUnnamed_16 = libc::c_uint;
pub const OBJECT_ID_MAX: C2RustUnnamed_16 = 402;
pub const OBJECT_ZL4: C2RustUnnamed_16 = 401;
pub const OBJECT_TIMEBLOCK: C2RustUnnamed_16 = 400;
pub const OBJECT_OUKE_HAKA: C2RustUnnamed_16 = 399;
pub const OBJECT_DOOR_KILLER: C2RustUnnamed_16 = 398;
pub const OBJECT_GI_SWORD_1: C2RustUnnamed_16 = 397;
pub const OBJECT_COB: C2RustUnnamed_16 = 396;
pub const OBJECT_COW: C2RustUnnamed_16 = 395;
pub const OBJECT_BWALL: C2RustUnnamed_16 = 394;
pub const OBJECT_PS: C2RustUnnamed_16 = 393;
pub const OBJECT_GS: C2RustUnnamed_16 = 392;
pub const OBJECT_HAKA_DOOR: C2RustUnnamed_16 = 391;
pub const OBJECT_GEFF: C2RustUnnamed_16 = 390;
pub const OBJECT_GJ: C2RustUnnamed_16 = 389;
pub const OBJECT_SKB: C2RustUnnamed_16 = 388;
pub const OBJECT_WF: C2RustUnnamed_16 = 387;
pub const OBJECT_MU: C2RustUnnamed_16 = 386;
pub const OBJECT_SPOT01_MATOYAB: C2RustUnnamed_16 = 385;
pub const OBJECT_SPOT01_MATOYA: C2RustUnnamed_16 = 384;
pub const OBJECT_GI_RUPY: C2RustUnnamed_16 = 383;
pub const OBJECT_GANON_ANIME3: C2RustUnnamed_16 = 382;
pub const OBJECT_GANON_ANIME2: C2RustUnnamed_16 = 381;
pub const OBJECT_GANON_ANIME1: C2RustUnnamed_16 = 380;
pub const OBJECT_GI_DEKUPOUCH: C2RustUnnamed_16 = 379;
pub const OBJECT_EFC_DOUGHNUT: C2RustUnnamed_16 = 378;
pub const OBJECT_DEMO_KEKKAI: C2RustUnnamed_16 = 377;
pub const OBJECT_BOWL: C2RustUnnamed_16 = 376;
pub const OBJECT_GI_SOUL: C2RustUnnamed_16 = 375;
pub const OBJECT_GI_GHOST: C2RustUnnamed_16 = 374;
pub const OBJECT_GI_BUTTERFLY: C2RustUnnamed_16 = 373;
pub const OBJECT_GI_INSECT: C2RustUnnamed_16 = 372;
pub const OBJECT_GI_FIRE: C2RustUnnamed_16 = 371;
pub const OBJECT_DNK: C2RustUnnamed_16 = 370;
pub const OBJECT_DNS: C2RustUnnamed_16 = 369;
pub const OBJECT_KIBAKO2: C2RustUnnamed_16 = 368;
pub const OBJECT_SPOT11_OBJ: C2RustUnnamed_16 = 367;
pub const OBJECT_UNSET_16E: C2RustUnnamed_16 = 366;
pub const OBJECT_JYA_DOOR: C2RustUnnamed_16 = 365;
pub const OBJECT_JYA_IRON: C2RustUnnamed_16 = 364;
pub const OBJECT_DOG: C2RustUnnamed_16 = 363;
pub const OBJECT_GR: C2RustUnnamed_16 = 362;
pub const OBJECT_GELDB: C2RustUnnamed_16 = 361;
pub const OBJECT_SHOPNUTS: C2RustUnnamed_16 = 360;
pub const OBJECT_GLA: C2RustUnnamed_16 = 359;
pub const OBJECT_SPOT00_BREAK: C2RustUnnamed_16 = 358;
pub const OBJECT_RS: C2RustUnnamed_16 = 357;
pub const OBJECT_HINTNUTS: C2RustUnnamed_16 = 356;
pub const OBJECT_BOMBIWA: C2RustUnnamed_16 = 355;
pub const OBJECT_SPOT12_OBJ: C2RustUnnamed_16 = 354;
pub const OBJECT_SPOT05_OBJECTS: C2RustUnnamed_16 = 353;
pub const OBJECT_BG: C2RustUnnamed_16 = 352;
pub const OBJECT_BIGOKUTA: C2RustUnnamed_16 = 351;
pub const OBJECT_SSH: C2RustUnnamed_16 = 350;
pub const OBJECT_GI_GODDESS: C2RustUnnamed_16 = 349;
pub const OBJECT_GI_SUTARU: C2RustUnnamed_16 = 348;
pub const OBJECT_FISH: C2RustUnnamed_16 = 347;
pub const OBJECT_EC: C2RustUnnamed_16 = 346;
pub const OBJECT_DS2: C2RustUnnamed_16 = 345;
pub const OBJECT_GI_M_ARROW: C2RustUnnamed_16 = 344;
pub const OBJECT_GI_HOVERBOOTS: C2RustUnnamed_16 = 343;
pub const OBJECT_ZG: C2RustUnnamed_16 = 342;
pub const OBJECT_TS: C2RustUnnamed_16 = 341;
pub const OBJECT_KA: C2RustUnnamed_16 = 340;
pub const OBJECT_GANON2: C2RustUnnamed_16 = 339;
pub const OBJECT_GI_GERUDOMASK: C2RustUnnamed_16 = 338;
pub const OBJECT_GI_ZORAMASK: C2RustUnnamed_16 = 337;
pub const OBJECT_GI_GOLONMASK: C2RustUnnamed_16 = 336;
pub const OBJECT_ZL2_ANIME2: C2RustUnnamed_16 = 335;
pub const OBJECT_ZL2_ANIME1: C2RustUnnamed_16 = 334;
pub const OBJECT_EFC_ERUPC: C2RustUnnamed_16 = 333;
pub const OBJECT_GT: C2RustUnnamed_16 = 332;
pub const OBJECT_DOOR_GERUDO: C2RustUnnamed_16 = 331;
pub const OBJECT_MAG: C2RustUnnamed_16 = 330;
pub const OBJECT_GI_FROG: C2RustUnnamed_16 = 329;
pub const OBJECT_GI_SOLDOUT: C2RustUnnamed_16 = 328;
pub const OBJECT_GI_BRACELET: C2RustUnnamed_16 = 327;
pub const OBJECT_GI_PRESCRIPTION: C2RustUnnamed_16 = 326;
pub const OBJECT_CS: C2RustUnnamed_16 = 325;
pub const OBJECT_JS: C2RustUnnamed_16 = 324;
pub const OBJECT_GI_BROKENSWORD: C2RustUnnamed_16 = 323;
pub const OBJECT_GI_TICKETSTONE: C2RustUnnamed_16 = 322;
pub const OBJECT_GI_MUSHROOM: C2RustUnnamed_16 = 321;
pub const OBJECT_GI_POWDER: C2RustUnnamed_16 = 320;
pub const OBJECT_GI_EYE_LOTION: C2RustUnnamed_16 = 319;
pub const OBJECT_OS: C2RustUnnamed_16 = 318;
pub const OBJECT_FA: C2RustUnnamed_16 = 317;
pub const OBJECT_MM: C2RustUnnamed_16 = 316;
pub const OBJECT_STREAM: C2RustUnnamed_16 = 315;
pub const OBJECT_SIOFUKI: C2RustUnnamed_16 = 314;
pub const OBJECT_GANON_OBJECTS: C2RustUnnamed_16 = 313;
pub const OBJECT_GI_TRUTH_MASK: C2RustUnnamed_16 = 312;
pub const OBJECT_GI_RABIT_MASK: C2RustUnnamed_16 = 311;
pub const OBJECT_GI_SKJ_MASK: C2RustUnnamed_16 = 310;
pub const OBJECT_GI_REDEAD_MASK: C2RustUnnamed_16 = 309;
pub const OBJECT_GI_KI_TAN_MASK: C2RustUnnamed_16 = 308;
pub const OBJECT_FU: C2RustUnnamed_16 = 307;
pub const OBJECT_MK: C2RustUnnamed_16 = 306;
pub const OBJECT_OWL: C2RustUnnamed_16 = 305;
pub const OBJECT_GJYO_OBJECTS: C2RustUnnamed_16 = 304;
pub const OBJECT_KANBAN: C2RustUnnamed_16 = 303;
pub const OBJECT_GI_COIN: C2RustUnnamed_16 = 302;
pub const OBJECT_GI_GLOVES: C2RustUnnamed_16 = 301;
pub const OBJECT_TSUBO: C2RustUnnamed_16 = 300;
pub const OBJECT_KUSA: C2RustUnnamed_16 = 299;
pub const OBJECT_LIGHTSWITCH: C2RustUnnamed_16 = 298;
pub const OBJECT_INGATE: C2RustUnnamed_16 = 297;
pub const OBJECT_HS: C2RustUnnamed_16 = 296;
pub const OBJECT_MS: C2RustUnnamed_16 = 295;
pub const OBJECT_GM: C2RustUnnamed_16 = 294;
pub const OBJECT_BLKOBJ: C2RustUnnamed_16 = 293;
pub const OBJECT_NWC: C2RustUnnamed_16 = 292;
pub const OBJECT_UNSET_123: C2RustUnnamed_16 = 291;
pub const OBJECT_DAIKU: C2RustUnnamed_16 = 290;
pub const OBJECT_TORYO: C2RustUnnamed_16 = 289;
pub const OBJECT_UNSET_120: C2RustUnnamed_16 = 288;
pub const OBJECT_GOROIWA: C2RustUnnamed_16 = 287;
pub const OBJECT_MAMENOKI: C2RustUnnamed_16 = 286;
pub const OBJECT_D_LIFT: C2RustUnnamed_16 = 285;
pub const OBJECT_D_HSBLOCK: C2RustUnnamed_16 = 284;
pub const OBJECT_D_ELEVATOR: C2RustUnnamed_16 = 283;
pub const OBJECT_GND_MAGIC: C2RustUnnamed_16 = 282;
pub const OBJECT_GI_SEED: C2RustUnnamed_16 = 281;
pub const OBJECT_GI_BOOTS_2: C2RustUnnamed_16 = 280;
pub const OBJECT_YABUSAME_POINT: C2RustUnnamed_16 = 279;
pub const OBJECT_GE1: C2RustUnnamed_16 = 278;
pub const OBJECT_BOB: C2RustUnnamed_16 = 277;
pub const OBJECT_FZ: C2RustUnnamed_16 = 276;
pub const OBJECT_SPOT07_OBJECT: C2RustUnnamed_16 = 275;
pub const OBJECT_SPOT03_OBJECT: C2RustUnnamed_16 = 274;
pub const OBJECT_BOJ: C2RustUnnamed_16 = 273;
pub const OBJECT_ANE: C2RustUnnamed_16 = 272;
pub const OBJECT_DS: C2RustUnnamed_16 = 271;
pub const OBJECT_GI_OCARINA_0: C2RustUnnamed_16 = 270;
pub const OBJECT_BBA: C2RustUnnamed_16 = 269;
pub const OBJECT_BJI: C2RustUnnamed_16 = 268;
pub const OBJECT_GI_BOTTLE_LETTER: C2RustUnnamed_16 = 267;
pub const OBJECT_SKJ: C2RustUnnamed_16 = 266;
pub const OBJECT_GI_NIWATORI: C2RustUnnamed_16 = 265;
pub const OBJECT_CNE: C2RustUnnamed_16 = 264;
pub const OBJECT_AHG: C2RustUnnamed_16 = 263;
pub const OBJECT_IK: C2RustUnnamed_16 = 262;
pub const OBJECT_AOB: C2RustUnnamed_16 = 261;
pub const OBJECT_MASTERZOORA: C2RustUnnamed_16 = 260;
pub const OBJECT_MASTERGOLON: C2RustUnnamed_16 = 259;
pub const OBJECT_MASTERKOKIRIHEAD: C2RustUnnamed_16 = 258;
pub const OBJECT_MASTERKOKIRI: C2RustUnnamed_16 = 257;
pub const OBJECT_UMAJUMP: C2RustUnnamed_16 = 256;
pub const OBJECT_KZ: C2RustUnnamed_16 = 255;
pub const OBJECT_ZO: C2RustUnnamed_16 = 254;
pub const OBJECT_KW1: C2RustUnnamed_16 = 253;
pub const OBJECT_KM1: C2RustUnnamed_16 = 252;
pub const OBJECT_MD: C2RustUnnamed_16 = 251;
pub const OBJECT_MD_UNUSED: C2RustUnnamed_16 = 250;
pub const OBJECT_SPOT01_OBJECTS: C2RustUnnamed_16 = 249;
pub const OBJECT_GI_LONGSWORD: C2RustUnnamed_16 = 248;
pub const OBJECT_GI_GRASS: C2RustUnnamed_16 = 247;
pub const OBJECT_GI_HAMMER: C2RustUnnamed_16 = 246;
pub const OBJECT_GI_SAW: C2RustUnnamed_16 = 245;
pub const OBJECT_GI_FISH: C2RustUnnamed_16 = 244;
pub const OBJECT_GI_BEAN: C2RustUnnamed_16 = 243;
pub const OBJECT_GI_CLOTHES: C2RustUnnamed_16 = 242;
pub const OBJECT_JYA_OBJ: C2RustUnnamed_16 = 241;
pub const OBJECT_SPOT15_OBJ: C2RustUnnamed_16 = 240;
pub const OBJECT_GI_LETTER: C2RustUnnamed_16 = 239;
pub const OBJECT_GI_SHIELD_3: C2RustUnnamed_16 = 238;
pub const OBJECT_DEMO_6K: C2RustUnnamed_16 = 237;
pub const OBJECT_ANI: C2RustUnnamed_16 = 236;
pub const OBJECT_GI_LIQUID: C2RustUnnamed_16 = 235;
pub const OBJECT_GI_GLASSES: C2RustUnnamed_16 = 234;
pub const OBJECT_GI_BOW: C2RustUnnamed_16 = 233;
pub const OBJECT_GI_BOOMERANG: C2RustUnnamed_16 = 232;
pub const OBJECT_GI_PACHINKO: C2RustUnnamed_16 = 231;
pub const OBJECT_FR: C2RustUnnamed_16 = 230;
pub const OBJECT_NY: C2RustUnnamed_16 = 229;
pub const OBJECT_UNSET_E4: C2RustUnnamed_16 = 228;
pub const OBJECT_NY_UNUSED: C2RustUnnamed_16 = 227;
pub const OBJECT_SST: C2RustUnnamed_16 = 226;
pub const OBJECT_GANON: C2RustUnnamed_16 = 225;
pub const OBJECT_MA1: C2RustUnnamed_16 = 224;
pub const OBJECT_GI_MILK: C2RustUnnamed_16 = 223;
pub const OBJECT_GI_OCARINA: C2RustUnnamed_16 = 222;
pub const OBJECT_GI_HOOKSHOT: C2RustUnnamed_16 = 221;
pub const OBJECT_GI_SHIELD_2: C2RustUnnamed_16 = 220;
pub const OBJECT_GI_SCALE: C2RustUnnamed_16 = 219;
pub const OBJECT_GI_EGG: C2RustUnnamed_16 = 218;
pub const OBJECT_GI_BOMB_2: C2RustUnnamed_16 = 217;
pub const OBJECT_GI_ARROW: C2RustUnnamed_16 = 216;
pub const OBJECT_GI_GERUDO: C2RustUnnamed_16 = 215;
pub const OBJECT_ANUBICE: C2RustUnnamed_16 = 214;
pub const OBJECT_BXA: C2RustUnnamed_16 = 213;
pub const OBJECT_RR: C2RustUnnamed_16 = 212;
pub const OBJECT_TW: C2RustUnnamed_16 = 211;
pub const OBJECT_HNI: C2RustUnnamed_16 = 210;
pub const OBJECT_GI_PURSE: C2RustUnnamed_16 = 209;
pub const OBJECT_MA2: C2RustUnnamed_16 = 208;
pub const OBJECT_OF1S: C2RustUnnamed_16 = 207;
pub const OBJECT_GI_BOMB_1: C2RustUnnamed_16 = 206;
pub const OBJECT_GI_MAGICPOT: C2RustUnnamed_16 = 205;
pub const OBJECT_DEKUJR: C2RustUnnamed_16 = 204;
pub const OBJECT_GI_SHIELD_1: C2RustUnnamed_16 = 203;
pub const OBJECT_RU2: C2RustUnnamed_16 = 202;
pub const OBJECT_OF1D_MAP: C2RustUnnamed_16 = 201;
pub const OBJECT_GI_MAP: C2RustUnnamed_16 = 200;
pub const OBJECT_GI_STICK: C2RustUnnamed_16 = 199;
pub const OBJECT_GI_BOTTLE: C2RustUnnamed_16 = 198;
pub const OBJECT_OS_ANIME: C2RustUnnamed_16 = 197;
pub const OBJECT_OE4S: C2RustUnnamed_16 = 196;
pub const OBJECT_OE1S: C2RustUnnamed_16 = 195;
pub const OBJECT_SPOT16_OBJ: C2RustUnnamed_16 = 194;
pub const OBJECT_TR: C2RustUnnamed_16 = 193;
pub const OBJECT_IN: C2RustUnnamed_16 = 192;
pub const OBJECT_GI_BOMBPOUCH: C2RustUnnamed_16 = 191;
pub const OBJECT_GI_ARROWCASE: C2RustUnnamed_16 = 190;
pub const OBJECT_GI_HEARTS: C2RustUnnamed_16 = 189;
pub const OBJECT_SA: C2RustUnnamed_16 = 188;
pub const OBJECT_GI_NUTS: C2RustUnnamed_16 = 187;
pub const OBJECT_GI_MEDAL: C2RustUnnamed_16 = 186;
pub const OBJECT_GI_BOSSKEY: C2RustUnnamed_16 = 185;
pub const OBJECT_GI_COMPASS: C2RustUnnamed_16 = 184;
pub const OBJECT_GI_HEART: C2RustUnnamed_16 = 183;
pub const OBJECT_GI_MELODY: C2RustUnnamed_16 = 182;
pub const OBJECT_SB: C2RustUnnamed_16 = 181;
pub const OBJECT_MO: C2RustUnnamed_16 = 180;
pub const OBJECT_NB: C2RustUnnamed_16 = 179;
pub const OBJECT_SHOP_DUNGEN: C2RustUnnamed_16 = 178;
pub const OBJECT_SPOT17_OBJ: C2RustUnnamed_16 = 177;
pub const OBJECT_BDOOR: C2RustUnnamed_16 = 176;
pub const OBJECT_SPOT18_OBJ: C2RustUnnamed_16 = 175;
pub const OBJECT_SPOT09_OBJ: C2RustUnnamed_16 = 174;
pub const OBJECT_GI_JEWEL: C2RustUnnamed_16 = 173;
pub const OBJECT_BROB: C2RustUnnamed_16 = 172;
pub const OBJECT_MIR_RAY: C2RustUnnamed_16 = 171;
pub const OBJECT_GI_KEY: C2RustUnnamed_16 = 170;
pub const OBJECT_DEMO_TRE_LGT: C2RustUnnamed_16 = 169;
pub const OBJECT_EFC_TW: C2RustUnnamed_16 = 168;
pub const OBJECT_RL: C2RustUnnamed_16 = 167;
pub const OBJECT_DH: C2RustUnnamed_16 = 166;
pub const OBJECT_FD2: C2RustUnnamed_16 = 165;
pub const OBJECT_SYOKUDAI: C2RustUnnamed_16 = 164;
pub const OBJECT_RU1: C2RustUnnamed_16 = 163;
pub const OBJECT_HAKA: C2RustUnnamed_16 = 162;
pub const OBJECT_SPOT02_OBJECTS: C2RustUnnamed_16 = 161;
pub const OBJECT_HORSE_LINK_CHILD: C2RustUnnamed_16 = 160;
pub const OBJECT_MEDAL: C2RustUnnamed_16 = 159;
pub const OBJECT_FW: C2RustUnnamed_16 = 158;
pub const OBJECT_DU: C2RustUnnamed_16 = 157;
pub const OBJECT_FD: C2RustUnnamed_16 = 156;
pub const OBJECT_GNDD: C2RustUnnamed_16 = 155;
pub const OBJECT_HEAVY_OBJECT: C2RustUnnamed_16 = 154;
pub const OBJECT_PO_SISTERS: C2RustUnnamed_16 = 153;
pub const OBJECT_RD: C2RustUnnamed_16 = 152;
pub const OBJECT_SD: C2RustUnnamed_16 = 151;
pub const OBJECT_BDAN_OBJECTS: C2RustUnnamed_16 = 150;
pub const OBJECT_TRIFORCE_SPOT: C2RustUnnamed_16 = 149;
pub const OBJECT_LIGHT_RING: C2RustUnnamed_16 = 148;
pub const OBJECT_GOD_LGT: C2RustUnnamed_16 = 147;
pub const OBJECT_EFC_STAR_FIELD: C2RustUnnamed_16 = 146;
pub const OBJECT_EFC_LGT_SHOWER: C2RustUnnamed_16 = 145;
pub const OBJECT_EFC_FLASH: C2RustUnnamed_16 = 144;
pub const OBJECT_EFC_FIRE_BALL: C2RustUnnamed_16 = 143;
pub const OBJECT_EFC_CRYSTAL_LIGHT: C2RustUnnamed_16 = 142;
pub const OBJECT_HAKACH_OBJECTS: C2RustUnnamed_16 = 141;
pub const OBJECT_BV: C2RustUnnamed_16 = 140;
pub const OBJECT_VM: C2RustUnnamed_16 = 139;
pub const OBJECT_XC: C2RustUnnamed_16 = 138;
pub const OBJECT_TK: C2RustUnnamed_16 = 137;
pub const OBJECT_TA: C2RustUnnamed_16 = 136;
pub const OBJECT_IM: C2RustUnnamed_16 = 135;
pub const OBJECT_VASE: C2RustUnnamed_16 = 134;
pub const OBJECT_TRAP: C2RustUnnamed_16 = 133;
pub const OBJECT_UNSET_84: C2RustUnnamed_16 = 132;
pub const OBJECT_UNSET_83: C2RustUnnamed_16 = 131;
pub const OBJECT_PU_BOX: C2RustUnnamed_16 = 130;
pub const OBJECT_LIGHTBOX: C2RustUnnamed_16 = 129;
pub const OBJECT_UNSET_80: C2RustUnnamed_16 = 128;
pub const OBJECT_UNSET_7F: C2RustUnnamed_16 = 127;
pub const OBJECT_UNSET_7E: C2RustUnnamed_16 = 126;
pub const OBJECT_UNSET_7D: C2RustUnnamed_16 = 125;
pub const OBJECT_WOOD02: C2RustUnnamed_16 = 124;
pub const OBJECT_UNSET_7B: C2RustUnnamed_16 = 123;
pub const OBJECT_UNSET_7A: C2RustUnnamed_16 = 122;
pub const OBJECT_UNSET_79: C2RustUnnamed_16 = 121;
pub const OBJECT_UNSET_78: C2RustUnnamed_16 = 120;
pub const OBJECT_BIRD: C2RustUnnamed_16 = 119;
pub const OBJECT_HATA: C2RustUnnamed_16 = 118;
pub const OBJECT_WARP2: C2RustUnnamed_16 = 117;
pub const OBJECT_SPOT08_OBJ: C2RustUnnamed_16 = 116;
pub const OBJECT_MORI_TEX: C2RustUnnamed_16 = 115;
pub const OBJECT_MORI_OBJECTS: C2RustUnnamed_16 = 114;
pub const OBJECT_MORI_HINERI2A: C2RustUnnamed_16 = 113;
pub const OBJECT_MORI_HINERI2: C2RustUnnamed_16 = 112;
pub const OBJECT_MORI_HINERI1A: C2RustUnnamed_16 = 111;
pub const OBJECT_PO_COMPOSER: C2RustUnnamed_16 = 110;
pub const OBJECT_PO_FIELD: C2RustUnnamed_16 = 109;
pub const OBJECT_RELAY_OBJECTS: C2RustUnnamed_16 = 108;
pub const OBJECT_ICE_OBJECTS: C2RustUnnamed_16 = 107;
pub const OBJECT_SPOT06_OBJECTS: C2RustUnnamed_16 = 106;
pub const OBJECT_HAKA_OBJECTS: C2RustUnnamed_16 = 105;
pub const OBJECT_MJIN_OKA: C2RustUnnamed_16 = 104;
pub const OBJECT_MJIN_WIND: C2RustUnnamed_16 = 103;
pub const OBJECT_MJIN_SOUL: C2RustUnnamed_16 = 102;
pub const OBJECT_MJIN_ICE: C2RustUnnamed_16 = 101;
pub const OBJECT_MJIN_FLAME: C2RustUnnamed_16 = 100;
pub const OBJECT_MJIN_DARK: C2RustUnnamed_16 = 99;
pub const OBJECT_MJIN_FLASH: C2RustUnnamed_16 = 98;
pub const OBJECT_MJIN: C2RustUnnamed_16 = 97;
pub const OBJECT_ZL2: C2RustUnnamed_16 = 96;
pub const OBJECT_YUKABYUN: C2RustUnnamed_16 = 95;
pub const OBJECT_TOKI_OBJECTS: C2RustUnnamed_16 = 94;
pub const OBJECT_BB: C2RustUnnamed_16 = 93;
pub const OBJECT_MORI_HINERI1: C2RustUnnamed_16 = 92;
pub const OBJECT_OSSAN: C2RustUnnamed_16 = 91;
pub const OBJECT_FHG: C2RustUnnamed_16 = 90;
pub const OBJECT_MIZU_OBJECTS: C2RustUnnamed_16 = 89;
pub const OBJECT_OA11: C2RustUnnamed_16 = 88;
pub const OBJECT_OA10: C2RustUnnamed_16 = 87;
pub const OBJECT_VALI: C2RustUnnamed_16 = 86;
pub const OBJECT_OE12: C2RustUnnamed_16 = 85;
pub const OBJECT_OE11: C2RustUnnamed_16 = 84;
pub const OBJECT_OE10: C2RustUnnamed_16 = 83;
pub const OBJECT_OE9: C2RustUnnamed_16 = 82;
pub const OBJECT_OE8: C2RustUnnamed_16 = 81;
pub const OBJECT_OE7: C2RustUnnamed_16 = 80;
pub const OBJECT_OE6: C2RustUnnamed_16 = 79;
pub const OBJECT_OE5: C2RustUnnamed_16 = 78;
pub const OBJECT_MENKURI_OBJECTS: C2RustUnnamed_16 = 77;
pub const OBJECT_OE4: C2RustUnnamed_16 = 76;
pub const OBJECT_OE3: C2RustUnnamed_16 = 75;
pub const OBJECT_DEKUNUTS: C2RustUnnamed_16 = 74;
pub const OBJECT_B_HEART: C2RustUnnamed_16 = 73;
pub const OBJECT_WARP1: C2RustUnnamed_16 = 72;
pub const OBJECT_OPENING_DEMO1: C2RustUnnamed_16 = 71;
pub const OBJECT_HORSE_ZELDA: C2RustUnnamed_16 = 70;
pub const OBJECT_OB4: C2RustUnnamed_16 = 69;
pub const OBJECT_OB3: C2RustUnnamed_16 = 68;
pub const OBJECT_OB2: C2RustUnnamed_16 = 67;
pub const OBJECT_OA9: C2RustUnnamed_16 = 66;
pub const OBJECT_OA8: C2RustUnnamed_16 = 65;
pub const OBJECT_JJ: C2RustUnnamed_16 = 64;
pub const OBJECT_OA7: C2RustUnnamed_16 = 63;
pub const OBJECT_OA6: C2RustUnnamed_16 = 62;
pub const OBJECT_OA5: C2RustUnnamed_16 = 61;
pub const OBJECT_OA4: C2RustUnnamed_16 = 60;
pub const OBJECT_OA3: C2RustUnnamed_16 = 59;
pub const OBJECT_UNSET_3A: C2RustUnnamed_16 = 58;
pub const OBJECT_DEKUBABA: C2RustUnnamed_16 = 57;
pub const OBJECT_AM: C2RustUnnamed_16 = 56;
pub const OBJECT_GND: C2RustUnnamed_16 = 55;
pub const OBJECT_YDAN_OBJECTS: C2RustUnnamed_16 = 54;
pub const OBJECT_OE2: C2RustUnnamed_16 = 53;
pub const OBJECT_OE_ANIME: C2RustUnnamed_16 = 52;
pub const OBJECT_OE1: C2RustUnnamed_16 = 51;
pub const OBJECT_SK2: C2RustUnnamed_16 = 50;
pub const OBJECT_BOMBF: C2RustUnnamed_16 = 49;
pub const OBJECT_MB: C2RustUnnamed_16 = 48;
pub const OBJECT_SPOT00_OBJECTS: C2RustUnnamed_16 = 47;
pub const OBJECT_OA2: C2RustUnnamed_16 = 46;
pub const OBJECT_HORSE_GANON: C2RustUnnamed_16 = 45;
pub const OBJECT_HIDAN_OBJECTS: C2RustUnnamed_16 = 44;
pub const OBJECT_DDAN_OBJECTS: C2RustUnnamed_16 = 43;
pub const OBJECT_SPOT04_OBJECTS: C2RustUnnamed_16 = 42;
pub const OBJECT_O_ANIME: C2RustUnnamed_16 = 41;
pub const OBJECT_OB1: C2RustUnnamed_16 = 40;
pub const OBJECT_HORSE_NORMAL: C2RustUnnamed_16 = 39;
pub const OBJECT_EI: C2RustUnnamed_16 = 38;
pub const OBJECT_BW: C2RustUnnamed_16 = 37;
pub const OBJECT_ST: C2RustUnnamed_16 = 36;
pub const OBJECT_OA1: C2RustUnnamed_16 = 35;
pub const OBJECT_TP: C2RustUnnamed_16 = 34;
pub const OBJECT_BL: C2RustUnnamed_16 = 33;
pub const OBJECT_TORCH2: C2RustUnnamed_16 = 32;
pub const OBJECT_DODOJR: C2RustUnnamed_16 = 31;
pub const OBJECT_GOL: C2RustUnnamed_16 = 30;
pub const OBJECT_ZL1: C2RustUnnamed_16 = 29;
pub const OBJECT_GOMA: C2RustUnnamed_16 = 28;
pub const OBJECT_ZF: C2RustUnnamed_16 = 27;
pub const OBJECT_HORSE: C2RustUnnamed_16 = 26;
pub const OBJECT_KINGDODONGO: C2RustUnnamed_16 = 25;
pub const OBJECT_PEEHAT: C2RustUnnamed_16 = 24;
pub const OBJECT_REEBA: C2RustUnnamed_16 = 23;
pub const OBJECT_TITE: C2RustUnnamed_16 = 22;
pub const OBJECT_LINK_CHILD: C2RustUnnamed_16 = 21;
pub const OBJECT_LINK_BOY: C2RustUnnamed_16 = 20;
pub const OBJECT_NIW: C2RustUnnamed_16 = 19;
pub const OBJECT_BUBBLE: C2RustUnnamed_16 = 18;
pub const OBJECT_UNSET_11: C2RustUnnamed_16 = 17;
pub const OBJECT_UNSET_10: C2RustUnnamed_16 = 16;
pub const OBJECT_FIRE: C2RustUnnamed_16 = 15;
pub const OBJECT_BOX: C2RustUnnamed_16 = 14;
pub const OBJECT_FIREFLY: C2RustUnnamed_16 = 13;
pub const OBJECT_DODONGO: C2RustUnnamed_16 = 12;
pub const OBJECT_WALLMASTER: C2RustUnnamed_16 = 11;
pub const OBJECT_DY_OBJ: C2RustUnnamed_16 = 10;
pub const OBJECT_POH: C2RustUnnamed_16 = 9;
pub const OBJECT_CROW: C2RustUnnamed_16 = 8;
pub const OBJECT_OKUTA: C2RustUnnamed_16 = 7;
pub const OBJECT_HUMAN: C2RustUnnamed_16 = 6;
pub const OBJECT_UNSET_5: C2RustUnnamed_16 = 5;
pub const OBJECT_UNSET_4: C2RustUnnamed_16 = 4;
pub const OBJECT_GAMEPLAY_DANGEON_KEEP: C2RustUnnamed_16 = 3;
pub const OBJECT_GAMEPLAY_FIELD_KEEP: C2RustUnnamed_16 = 2;
pub const OBJECT_GAMEPLAY_KEEP: C2RustUnnamed_16 = 1;
pub const OBJECT_INVALID: C2RustUnnamed_16 = 0;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct MagicWind {
    pub actor: Actor,
    pub skelCurve: SkelAnimeCurve,
    pub timer: s16,
    pub actionFunc: MagicWindFunc,
}
pub type MagicWindFunc
    =
    Option<unsafe extern "C" fn(_: *mut MagicWind, _: *mut GlobalContext)
               -> ()>;
#[no_mangle]
pub static mut Magic_Wind_InitVars: ActorInit =
    unsafe {
        {
            let mut init =
                ActorInit{id: ACTOR_MAGIC_WIND as libc::c_int as s16,
                          category:
                              ACTORCAT_ITEMACTION as libc::c_int as u8_0,
                          flags:
                              ((1 as libc::c_int) << 4 as libc::c_int |
                                   (1 as libc::c_int) << 25 as libc::c_int) as
                                  u32_0,
                          objectId:
                              OBJECT_GAMEPLAY_KEEP as libc::c_int as s16,
                          instanceSize:
                              ::std::mem::size_of::<MagicWind>() as
                                  libc::c_ulong,
                          init:
                              ::std::mem::transmute::<Option<unsafe extern "C" fn(_:
                                                                                      *mut Actor,
                                                                                  _:
                                                                                      *mut GlobalContext)
                                                                 -> ()>,
                                                      ActorFunc>(Some(MagicWind_Init
                                                                          as
                                                                          unsafe extern "C" fn(_:
                                                                                                   *mut Actor,
                                                                                               _:
                                                                                                   *mut GlobalContext)
                                                                              ->
                                                                                  ())),
                          destroy:
                              ::std::mem::transmute::<Option<unsafe extern "C" fn(_:
                                                                                      *mut Actor,
                                                                                  _:
                                                                                      *mut GlobalContext)
                                                                 -> ()>,
                                                      ActorFunc>(Some(MagicWind_Destroy
                                                                          as
                                                                          unsafe extern "C" fn(_:
                                                                                                   *mut Actor,
                                                                                               _:
                                                                                                   *mut GlobalContext)
                                                                              ->
                                                                                  ())),
                          update:
                              ::std::mem::transmute::<Option<unsafe extern "C" fn(_:
                                                                                      *mut Actor,
                                                                                  _:
                                                                                      *mut GlobalContext)
                                                                 -> ()>,
                                                      ActorFunc>(Some(MagicWind_Update
                                                                          as
                                                                          unsafe extern "C" fn(_:
                                                                                                   *mut Actor,
                                                                                               _:
                                                                                                   *mut GlobalContext)
                                                                              ->
                                                                                  ())),
                          draw:
                              ::std::mem::transmute::<Option<unsafe extern "C" fn(_:
                                                                                      *mut Actor,
                                                                                  _:
                                                                                      *mut GlobalContext)
                                                                 -> ()>,
                                                      ActorFunc>(Some(MagicWind_Draw
                                                                          as
                                                                          unsafe extern "C" fn(_:
                                                                                                   *mut Actor,
                                                                                               _:
                                                                                                   *mut GlobalContext)
                                                                              ->
                                                                                  ())),};
            init
        }
    };
static mut sAnim: TransformUpdateIndex =
    unsafe {
        {
            let mut init =
                TransformUpdateIndex{refIndex:
                                         ovl_Magic_WindCurveAnime_Ref_001BD0.as_ptr()
                                             as *mut _,
                                     transformData:
                                         ovl_Magic_WindCurveAnime_TransformData_001C20.as_ptr()
                                             as *mut _,
                                     copyValues:
                                         ovl_Magic_WindCurveAnime_Copy_001BEC.as_ptr()
                                             as *mut _,
                                     unk_0C: 1 as libc::c_int as s16,
                                     unk_0E: 60 as libc::c_int as s16,};
            init
        }
    };
static mut ovl_Magic_WindCurveAnime_Copy_001BEC: [s16; 25] =
    [1024 as libc::c_int as s16, 1024 as libc::c_int as s16,
     1024 as libc::c_int as s16, 0 as libc::c_int as s16,
     0 as libc::c_int as s16, 0 as libc::c_int as s16,
     0 as libc::c_int as s16, 0 as libc::c_int as s16,
     0 as libc::c_int as s16, 512 as libc::c_int as s16,
     512 as libc::c_int as s16, 0 as libc::c_int as s16,
     0 as libc::c_int as s16, 0 as libc::c_int as s16,
     0 as libc::c_int as s16, 0 as libc::c_int as s16,
     0 as libc::c_int as s16, 717 as libc::c_int as s16,
     717 as libc::c_int as s16, 0 as libc::c_int as s16,
     0 as libc::c_int as s16, 0 as libc::c_int as s16,
     0 as libc::c_int as s16, 0 as libc::c_int as s16,
     0 as libc::c_int as s16];
static mut ovl_Magic_WindCurveAnime_TransformData_001C20: [TransformData; 4] =
    [{
         let mut init =
             TransformData{unk_00: 0xc as libc::c_int as u16_0,
                           unk_02: 0x1 as libc::c_int as s16,
                           unk_04: 1 as libc::c_int as s16,
                           unk_06: 1 as libc::c_int as s16,
                           unk_08: 0.000000f32,};
         init
     },
     {
         let mut init =
             TransformData{unk_00: 0x14 as libc::c_int as u16_0,
                           unk_02: 0x3c as libc::c_int as s16,
                           unk_04: 0 as libc::c_int as s16,
                           unk_06: 0 as libc::c_int as s16,
                           unk_08: 1.500000f32,};
         init
     },
     {
         let mut init =
             TransformData{unk_00: 0xc as libc::c_int as u16_0,
                           unk_02: 0x1 as libc::c_int as s16,
                           unk_04: 1 as libc::c_int as s16,
                           unk_06: 1 as libc::c_int as s16,
                           unk_08: 0.000000f32,};
         init
     },
     {
         let mut init =
             TransformData{unk_00: 0x14 as libc::c_int as u16_0,
                           unk_02: 0x3c as libc::c_int as s16,
                           unk_04: 0 as libc::c_int as s16,
                           unk_06: 0 as libc::c_int as s16,
                           unk_08: 1.000000f32,};
         init
     }];
static mut ovl_Magic_WindCurveAnime_Ref_001BD0: [u8_0; 27] =
    [0 as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
     0 as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
     0 as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
     0 as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
     0 as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
     0x2 as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
     0 as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
     0 as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
     0 as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
     0 as libc::c_int as u8_0, 0x2 as libc::c_int as u8_0,
     0 as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
     0 as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
     0 as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
     0 as libc::c_int as u8_0];
static mut sRootLimb: SkelCurveLimb =
    {
        let mut init =
            SkelCurveLimb{firstChildIdx: 0x1 as libc::c_int as u8_0,
                          nextLimbIdx: 0xff as libc::c_int as u8_0,
                          dList:
                              [0 as *const Gfx as *mut Gfx,
                               0 as *const Gfx as *mut Gfx],};
        init
    };
// Initialized in run_static_initializers
static mut sInnerCylinderDL: [Gfx; 33] =
    [Gfx{words: Gwords{w0: 0, w1: 0,},}; 33];
static mut sInnerCylinderLimb: SkelCurveLimb =
    unsafe {
        {
            let mut init =
                SkelCurveLimb{firstChildIdx: 0xff as libc::c_int as u8_0,
                              nextLimbIdx: 0x2 as libc::c_int as u8_0,
                              dList:
                                  [0 as *const Gfx as *mut Gfx,
                                   sInnerCylinderDL.as_ptr() as *mut _],};
            init
        }
    };
static mut sTex: [u64_0; 512] =
    [0x61928436682a0ae as libc::c_longlong as u64_0,
     0xa393765632120000 as libc::c_ulonglong,
     0x31525395781adbc as libc::c_longlong as u64_0,
     0xa47b5d4533202438 as libc::c_ulonglong,
     0x331f264f718cb2c9 as libc::c_longlong as u64_0,
     0xc7ae875733241100 as libc::c_ulonglong,
     0x51c3b5e7d9093 as libc::c_longlong as u64_0,
     0x85767881703e1401 as libc::c_ulonglong,
     0x1627334f6c84969a as libc::c_longlong as u64_0,
     0x9b8c704f27090000 as libc::c_ulonglong,
     0x51529436896b9b7 as libc::c_longlong as u64_0,
     0x996e4c342a202631 as libc::c_ulonglong,
     0x2b202f4d668fb5c0 as libc::c_longlong as u64_0,
     0xb4976b46311e0700 as libc::c_ulonglong,
     0xc274c6b8c9e9a as libc::c_longlong as u64_0,
     0x8b7a7a7c66421e0b as libc::c_ulonglong,
     0x27303e617c8f97a0 as libc::c_longlong as u64_0,
     0xa2876f512a0c0000 as libc::c_ulonglong,
     0x91d355983acc0b5 as libc::c_longlong as u64_0,
     0x8e5e3e2e28252427 as libc::c_ulonglong,
     0x2628394d6ca0b8b3 as libc::c_longlong as u64_0,
     0xa07a58402e130000 as libc::c_ulonglong,
     0x61b3b5d84a9b2ad as libc::c_longlong as u64_0,
     0x998b8f876743251c as libc::c_ulonglong,
     0x2d3a4f738d9da8ab as libc::c_longlong as u64_0,
     0x9f85725b36160300 as libc::c_ulonglong,
     0x11294872a0c6c7ac as libc::c_longlong as u64_0,
     0x8052362827231b1e as libc::c_ulonglong,
     0x242e3a507ba6b3a6 as libc::c_longlong as u64_0,
     0x8f684d3924080000 as libc::c_ulonglong,
     0x112d4f73a2c1c8bd as libc::c_longlong as u64_0,
     0xafb2b49565402b25 as libc::c_ulonglong,
     0x30485e7d96a9b4ac as libc::c_longlong as u64_0,
     0x9b8a79603a1b0b0a as libc::c_ulonglong,
     0x1a355c8bbcd6c9a0 as libc::c_longlong as u64_0,
     0x734d2f1e201d1119 as libc::c_longlong as u64_0,
     0x2831385383a2a99d as libc::c_longlong as u64_0,
     0x815b42301d020006 as libc::c_ulonglong,
     0x1d3c618dbad8dbc7 as libc::c_longlong as u64_0,
     0xc7cec18c5c443527 as libc::c_ulonglong,
     0x3a586c849bafb4ae as libc::c_longlong as u64_0,
     0xa39178522e171114 as libc::c_ulonglong,
     0x264872a4d4dfc799 as libc::c_longlong as u64_0,
     0x7253321c1f1b192a as libc::c_longlong as u64_0,
     0x393c46638aa0a396 as libc::c_longlong as u64_0,
     0x73513d301c020010 as libc::c_longlong as u64_0,
     0x2a4d79a6d5efe6d5 as libc::c_longlong as u64_0,
     0xd0c8ab77574f3f2e as libc::c_ulonglong,
     0x446377879fadb3b7 as libc::c_longlong as u64_0,
     0xaf9064391c11141f as libc::c_ulonglong,
     0x385c8abfe8e7c69e as libc::c_longlong as u64_0,
     0x7f6448312b313f4c as libc::c_longlong as u64_0,
     0x4e53668096a4a38e as libc::c_longlong as u64_0,
     0x664f473b1f06061c as libc::c_longlong as u64_0,
     0x3a6393c2effceddd as libc::c_longlong as u64_0,
     0xc2a485645a5a4338 as libc::c_ulonglong,
     0x48667a8aa0aab2bc as libc::c_longlong as u64_0,
     0xab784521100d192f as libc::c_ulonglong,
     0x4d72a3d7f7edccae as libc::c_longlong as u64_0,
     0x947f69535263655e as libc::c_ulonglong,
     0x5c698097a4aaa180 as libc::c_longlong as u64_0,
     0x605b5e4c270d122b as libc::c_longlong as u64_0,
     0x4e7babdefffdecd2 as libc::c_longlong as u64_0,
     0xa57a696266614540 as libc::c_ulonglong,
     0x4d62798da3abb3b0 as libc::c_longlong as u64_0,
     0x895228120a102543 as libc::c_ulonglong,
     0x638cbeeafff7dec4 as libc::c_longlong as u64_0,
     0xb09f8b7c8b896f62 as libc::c_ulonglong,
     0x66778ca3b2af9973 as libc::c_longlong as u64_0,
     0x67737258331c233d as libc::c_longlong as u64_0,
     0x6591c4f2fff5ddba as libc::c_longlong as u64_0,
     0x8a666e7979664a47 as libc::c_ulonglong,
     0x57657d95aab5b394 as libc::c_longlong as u64_0,
     0x5c2e160b0b1b3959 as libc::c_longlong as u64_0,
     0x7da6d5f9fffff5e0 as libc::c_longlong as u64_0,
     0xccb8acaba98d6f66 as libc::c_ulonglong,
     0x71869daebcae8e76 as libc::c_longlong as u64_0,
     0x808a7c6044333852 as libc::c_ulonglong,
     0x7baaddfeffe7c9a4 as libc::c_longlong as u64_0,
     0x7f76909b866b5250 as libc::c_longlong as u64_0,
     0x63768ba1b9c0aa75 as libc::c_longlong as u64_0,
     0x3a191212192e5075 as libc::c_longlong as u64_0,
     0x9bc1e7fffffffff5 as libc::c_ulonglong,
     0xdfccc7b69a7d6769 as libc::c_ulonglong,
     0x85a0b0c0c1a88c87 as libc::c_ulonglong,
     0x99967b655b52526b as libc::c_ulonglong,
     0x90c2ecfffbddbda1 as libc::c_ulonglong,
     0x909bb2a98268615e as libc::c_ulonglong,
     0x6b819eb4cac49d5c as libc::c_longlong as u64_0,
     0x28151c2730476e96 as libc::c_longlong as u64_0,
     0xb8d7f2fbf9fafff9 as libc::c_ulonglong,
     0xe7d6bf9e7f675d75 as libc::c_ulonglong,
     0xa1b0beccc0a3949b as libc::c_ulonglong,
     0xa896787078737180 as libc::c_ulonglong,
     0xa6d6f1f6f2ddc4b7 as libc::c_ulonglong,
     0xb4bdb9a0796d6f6b as libc::c_ulonglong,
     0x6d87accad5c08e4f as libc::c_longlong as u64_0,
     0x2823313f4c6993b5 as libc::c_longlong as u64_0,
     0xcee3edeae5e6ecf1 as libc::c_ulonglong,
     0xe5cdaa8a6f5e678d as libc::c_ulonglong,
     0xb2b7c8d1bfa499a1 as libc::c_ulonglong,
     0xa48e7d86958f8794 as libc::c_ulonglong,
     0xbddfe6e7e9e1ddd6 as libc::c_ulonglong,
     0xd0c8ac987b767569 as libc::c_ulonglong,
     0x6e8eb7dddbb98454 as libc::c_longlong as u64_0,
     0x39394553688db6cb as libc::c_longlong as u64_0,
     0xd6dfded7d3d4dee2 as libc::c_ulonglong,
     0xd7bd9f80696c87a3 as libc::c_ulonglong,
     0xbac4d4d6c2a99693 as libc::c_ulonglong,
     0x90878d9eaaa096a6 as libc::c_ulonglong,
     0xc9d8d1d2dce5e9e1 as libc::c_ulonglong,
     0xd4bda0948277675e as libc::c_ulonglong,
     0x7195c7ebdcb08769 as libc::c_longlong as u64_0,
     0x544e516384aed0d1 as libc::c_longlong as u64_0,
     0xd0d3cec9c7c7ced0 as libc::c_ulonglong,
     0xc7af96796f829fb5 as libc::c_ulonglong,
     0xc7d7e3ddc8a98776 as libc::c_ulonglong,
     0x77899fb0b4a5a0b2 as libc::c_longlong as u64_0,
     0xc7c8b8bccad8dbd8 as libc::c_ulonglong,
     0xc8ac978676625159 as libc::c_ulonglong,
     0x73a1dff7d3a89586 as libc::c_longlong as u64_0,
     0x735c59719fc8d3ca as libc::c_longlong as u64_0,
     0xc9cac2bab8b7babf as libc::c_ulonglong,
     0xb7a390767b94aac3 as libc::c_ulonglong,
     0xd3e1f0e2c6a27358 as libc::c_ulonglong,
     0x658cabb7b3a5a9b4 as libc::c_longlong as u64_0,
     0xbcafa0a4b3c1d0d1 as libc::c_ulonglong,
     0xb79985705340475a as libc::c_ulonglong,
     0x7bb8f6f6c8a3a09f as libc::c_longlong as u64_0,
     0x85656385b6cecbc9 as libc::c_ulonglong,
     0xccc4b0a2a1a4aab2 as libc::c_ulonglong,
     0xad9d8e7989a5bccb as libc::c_ulonglong,
     0xcedef2debd966146 as libc::c_ulonglong,
     0x608daeb3aeabb4b5 as libc::c_longlong as u64_0,
     0xaa90868da0bad2ca as libc::c_ulonglong,
     0xa482705b3630465c as libc::c_ulonglong,
     0x8dd2fcecc4a8a5a4 as libc::c_ulonglong,
     0x8b71759ec1cccacd as libc::c_ulonglong,
     0xcdb691808593a0ab as libc::c_ulonglong,
     0xaa9a908597b7c6bd as libc::c_ulonglong,
     0xb9d6ebd8b4865a45 as libc::c_ulonglong,
     0x628da6abb2b9bfb3 as libc::c_longlong as u64_0,
     0x8e726e7da2cad0b5 as libc::c_ulonglong,
     0x8e7065503834435d as libc::c_ulonglong,
     0xa0e0f4e6caaea39a as libc::c_ulonglong,
     0x8d8390b2c9cccccd as libc::c_ulonglong,
     0xbf986b5d6f85a1b2 as libc::c_ulonglong,
     0xac9b958f9faea597 as libc::c_ulonglong,
     0xa5cee5d4ac836356 as libc::c_ulonglong,
     0x6b8c9eacbfcac39f as libc::c_longlong as u64_0,
     0x705a6286bfd7bd9a as libc::c_longlong as u64_0,
     0x806d65544c3f4063 as libc::c_ulonglong,
     0xadddeae2ceac9a93 as libc::c_ulonglong,
     0x909aafc1ccc8c8bf as libc::c_ulonglong,
     0x9e7048476384b0b9 as libc::c_ulonglong,
     0xad9a918e8d7d6f76 as libc::c_ulonglong,
     0x99c4ded4af8d7370 as libc::c_ulonglong,
     0x7c91a2b7cbd0b07f as libc::c_longlong as u64_0,
     0x5a526daadccaa18d as libc::c_longlong as u64_0,
     0x7d716f685c404271 as libc::c_longlong as u64_0,
     0xb6d8e2dbc49f9594 as libc::c_ulonglong,
     0x98b3c9c9c3beb8a1 as libc::c_ulonglong,
     0x764d333d6696c0b6 as libc::c_longlong as u64_0,
     0xa597827760424665 as libc::c_ulonglong,
     0x91bdd5d9bd9e8d8b as libc::c_ulonglong,
     0x97a6b4c6cdba9066 as libc::c_ulonglong,
     0x525d8ccbd7ad938a as libc::c_longlong as u64_0,
     0x80767b7b5b3c4a80 as libc::c_ulonglong,
     0xc2dfd9c6a08e9a9e as libc::c_ulonglong,
     0xa4c7d4c2b7b0a280 as libc::c_ulonglong,
     0x52342e477fabbfac as libc::c_longlong as u64_0,
     0x947c5e4e301d3560 as libc::c_ulonglong,
     0x8db6ceddcdb6a8a4 as libc::c_ulonglong,
     0xb4c4cccdbc9a765a as libc::c_ulonglong,
     0x5773a8cabf9e8f89 as libc::c_longlong as u64_0,
     0x7d7b81714a3b588d as libc::c_longlong as u64_0,
     0xd0e1c9997287a9b0 as libc::c_ulonglong,
     0xb3d1d2b7aca28c65 as libc::c_ulonglong,
     0x3b293c6c9bb4b09b as libc::c_longlong as u64_0,
     0x724c382a1511335d as libc::c_longlong as u64_0,
     0x8ab3c7dbe2d0beb6 as libc::c_ulonglong,
     0xceded8c3a2816556 as libc::c_ulonglong,
     0x5e85a5adae9b8d86 as libc::c_longlong as u64_0,
     0x827d7051363b6399 as libc::c_ulonglong,
     0xd0d1a1645889b7bd as libc::c_ulonglong,
     0xbad0c9b0a2957952 as libc::c_ulonglong,
     0x2f2f5e91a5a99a79 as libc::c_longlong as u64_0,
     0x4c2d251c0f123059 as libc::c_longlong as u64_0,
     0x87abbfdcf5e3c9c0 as libc::c_ulonglong,
     0xe1e7d3b38f71594f as libc::c_ulonglong,
     0x5e808d95a3978683 as libc::c_longlong as u64_0,
     0x856f5039263b6fa2 as libc::c_ulonglong,
     0xb9a06746548cb6b9 as libc::c_ulonglong,
     0xc1d0bfa999846342 as libc::c_ulonglong,
     0x304479999a938058 as libc::c_longlong as u64_0,
     0x3827231a12162552 as libc::c_longlong as u64_0,
     0x7b9bb8dbfeebd0ca as libc::c_longlong as u64_0,
     0xdfd6c4a482644a42 as libc::c_ulonglong,
     0x546c778696897977 as libc::c_longlong as u64_0,
     0x755b392820477fa3 as libc::c_longlong as u64_0,
     0x8d5a3d455c85a3b2 as libc::c_ulonglong,
     0xced1b3a08f714e3a as libc::c_ulonglong,
     0x3e567a837c78664e as libc::c_longlong as u64_0,
     0x3d2d201a1c202345 as libc::c_longlong as u64_0,
     0x678aafd3f6e7d7cd as libc::c_longlong as u64_0,
     0xc8bcae9972563a31 as libc::c_ulonglong,
     0x45586c7b82756662 as libc::c_longlong as u64_0,
     0x584c35242d5e8b9d as libc::c_longlong as u64_0,
     0x60292d4f5d708db5 as libc::c_longlong as u64_0,
     0xdcc4a89a835d4042 as libc::c_ulonglong,
     0x4c5965605d605450 as libc::c_longlong as u64_0,
     0x47321c1a2d3e3433 as libc::c_longlong as u64_0,
     0x5179a4c3e3e1d6c3 as libc::c_longlong as u64_0,
     0xab9d978b6244271f as libc::c_ulonglong,
     0x3449616c6c645c50 as libc::c_longlong as u64_0,
     0x4443433a4a75908e as libc::c_longlong as u64_0,
     0x48172851505a8cbc as libc::c_longlong as u64_0,
     0xd3b3a0987750464f as libc::c_ulonglong,
     0x4e4f4c474c4f4d4d as libc::c_longlong as u64_0,
     0x46321b2650664a2a as libc::c_longlong as u64_0,
     0x42739db2d4d9cbb2 as libc::c_longlong as u64_0,
     0x8c7f847a52321b10 as libc::c_ulonglong,
     0x233b4c53595d5b48 as libc::c_longlong as u64_0,
     0x404c5d666b818d80 as libc::c_longlong as u64_0,
     0x421624443b5894b3 as libc::c_longlong as u64_0,
     0xc1aaa2946f545456 as libc::c_ulonglong,
     0x473d3d3c4345474a as libc::c_longlong as u64_0,
     0x45362d527c795329 as libc::c_longlong as u64_0,
     0x3f7399abc7cdbd98 as libc::c_longlong as u64_0,
     0x706c736b47271408 as libc::c_longlong as u64_0,
     0x112832394d5d5a4f as libc::c_longlong as u64_0,
     0x51688d8e80818571 as libc::c_longlong as u64_0,
     0x38131c2f33628fa6 as libc::c_longlong as u64_0,
     0xbdb0a88c77645e56 as libc::c_ulonglong,
     0x3f2f2e313f43454d as libc::c_longlong as u64_0,
     0x4e506b999b7d5a2d as libc::c_longlong as u64_0,
     0x3f719babbcc0a380 as libc::c_longlong as u64_0,
     0x5d5e635c442a1c0b as libc::c_longlong as u64_0,
     0xb19212d44606367 as libc::c_longlong as u64_0,
     0x7197b49d8479735b as libc::c_longlong as u64_0,
     0x21101724346286a3 as libc::c_longlong as u64_0,
     0xc2bda88d86756b58 as libc::c_ulonglong,
     0x3c211524424c4e57 as libc::c_longlong as u64_0,
     0x658ac3cbab846031 as libc::c_longlong as u64_0,
     0x3e6e98adb3b28c71 as libc::c_longlong as u64_0,
     0x50545a584d3e2f1e as libc::c_longlong as u64_0,
     0x15151e2f45627a87 as libc::c_longlong as u64_0,
     0xa0bdaa8d7b69523c as libc::c_ulonglong,
     0x11151e1e325b82a4 as libc::c_longlong as u64_0,
     0xc8bea5979183795d as libc::c_ulonglong,
     0x3c1a0f254c606165 as libc::c_longlong as u64_0,
     0x8ccaf7deb98d6436 as libc::c_ulonglong,
     0x3b6690aaa89f8162 as libc::c_longlong as u64_0,
     0x46505a6461524235 as libc::c_longlong as u64_0,
     0x251920384f688dac as libc::c_longlong as u64_0,
     0xc3ab776b674e301d as libc::c_ulonglong,
     0x1423251732587ca3 as libc::c_longlong as u64_0,
     0xc0b3a19a978d8260 as libc::c_ulonglong,
     0x3c1e1e395c7a7682 as libc::c_longlong as u64_0,
     0xbae5ffebc294683d as libc::c_ulonglong,
     0x325c829e9b8c6948 as libc::c_longlong as u64_0,
     0x44566e817663574a as libc::c_longlong as u64_0,
     0x2f1f2a445b78a0c6 as libc::c_longlong as u64_0,
     0xaf724242442e110a as libc::c_ulonglong,
     0x212a21133253799a as libc::c_longlong as u64_0,
     0xb0a596979d91835c as libc::c_ulonglong,
     0x4333394f739090ac as libc::c_longlong as u64_0,
     0xd3e6fff4c4986b40 as libc::c_ulonglong,
     0x284f6f8e8b6c3930 as libc::c_longlong as u64_0,
     0x4e6c939b87796f56 as libc::c_longlong as u64_0,
     0x312638516887b2b3 as libc::c_longlong as u64_0,
     0x7b471f191e0f000f as libc::c_longlong as u64_0,
     0x2a27160b3051778c as libc::c_longlong as u64_0,
     0xa0988e96a2997f5e as libc::c_ulonglong,
     0x545051618a9eaec8 as libc::c_longlong as u64_0,
     0xd9f0fff9c7936744 as libc::c_ulonglong,
     0x1c3c607f6e361435 as libc::c_longlong as u64_0,
     0x6787a8a29d967a4d as libc::c_longlong as u64_0,
     0x2d2e45617696aa87 as libc::c_longlong as u64_0,
     0x5d39120003000d21 as libc::c_longlong as u64_0,
     0x271908062e527180 as libc::c_longlong as u64_0,
     0x95918f9aaaa0806b as libc::c_ulonglong,
     0x6f6b657097abc3d4 as libc::c_longlong as u64_0,
     0xe9fefffbc2896644 as libc::c_ulonglong,
     0xd2b5064380f1d58 as libc::c_longlong as u64_0,
     0x84949fa0b3a26938 as libc::c_ulonglong,
     0x2939586f819d9577 as libc::c_longlong as u64_0,
     0x5d3f170000112d35 as libc::c_longlong as u64_0,
     0x170501062f526b7a as libc::c_longlong as u64_0,
     0x8f9499a4afa38b80 as libc::c_ulonglong,
     0x857d757fa5bad1e5 as libc::c_ulonglong,
     0xfcfffff6ba8c6c39 as libc::c_ulonglong,
     0x61c3b330c154279 as libc::c_longlong as u64_0,
     0x918c95a9b88f4f2f as libc::c_ulonglong,
     0x344f6e7b899a9181 as libc::c_longlong as u64_0,
     0x684a23061a3b4d3d as libc::c_longlong as u64_0,
     0x700050b2e506b7b as libc::c_longlong as u64_0,
     0x8f9aa5abada8a296 as libc::c_ulonglong,
     0x8c898190b4c3def4 as libc::c_ulonglong,
     0xffffffefc09e6b29 as libc::c_ulonglong,
     0x211190a11345b7f as libc::c_longlong as u64_0,
     0x8c879eb3a070463a as libc::c_ulonglong,
     0x517083858ba1a18f as libc::c_longlong as u64_0,
     0x6e503129516c6033 as libc::c_longlong as u64_0,
     0x807101229507381 as libc::c_longlong as u64_0,
     0x93a4a9a9acb9b898 as libc::c_ulonglong,
     0x8c8c8da2b9c8e3f6 as libc::c_ulonglong,
     0xfffff3f4d1a25c17 as libc::c_ulonglong,
     0x106020d30455e70 as libc::c_longlong as u64_0,
     0x818ea8a97f634f59 as libc::c_ulonglong,
     0x7d97978d90b2b491 as libc::c_longlong as u64_0,
     0x6c5854668f895e27 as libc::c_longlong as u64_0,
     0x2324211629597d85 as libc::c_longlong as u64_0,
     0x97a8a0a4b8ccbd93 as libc::c_ulonglong,
     0x8c8c95a6bacee3fa as libc::c_ulonglong,
     0xfffaf5ffd594480c as libc::c_ulonglong,
     0x92a44445b66 as libc::c_longlong as u64_0,
     0x819ba49372676682 as libc::c_ulonglong,
     0xabb5a598a0beb58b as libc::c_ulonglong,
     0x6c6c8fadb68e5a2e as libc::c_longlong as u64_0,
     0x4f49311e34648287 as libc::c_longlong as u64_0,
     0x94a099adc8d1b999 as libc::c_ulonglong,
     0x8d8d91a2b9d4ebfd as libc::c_ulonglong,
     0xfaf5ffffc67d3e11 as libc::c_ulonglong,
     0x50d213a44445c69 as libc::c_longlong as u64_0,
     0x8ba29a8b787885aa as libc::c_ulonglong,
     0xc7c0afa8adb7aa85 as libc::c_ulonglong,
     0x7794cee1c08f654f as libc::c_longlong as u64_0,
     0x7b623e2f486d7d81 as libc::c_longlong as u64_0,
     0x8f969fbccccebfa8 as libc::c_ulonglong,
     0x9387899ebadcf2f4 as libc::c_ulonglong,
     0xecf5fffab0734830 as libc::c_ulonglong,
     0x313c3d3940475e72 as libc::c_longlong as u64_0,
     0x96a098938990a4c0 as libc::c_ulonglong,
     0xc8c2b8b4adaaa28a as libc::c_ulonglong,
     0x97ccfcf6c49d8782 as libc::c_ulonglong,
     0x94684e495d72707c as libc::c_ulonglong,
     0x909ab2c3cdd6cdb4 as libc::c_ulonglong,
     0x978387a3c6e3efe9 as libc::c_ulonglong,
     0xdfefffe7a87f6768 as libc::c_ulonglong,
     0x706b53393f495e7f as libc::c_longlong as u64_0,
     0x9ba0a29f9aa9b3bd as libc::c_ulonglong,
     0xbfbdbcb7a5a2a4a4 as libc::c_ulonglong,
     0xcbfbfffbd4beb7af as libc::c_ulonglong,
     0x976c626772766d80 as libc::c_ulonglong,
     0x95abc1cad9e7d7b9 as libc::c_ulonglong,
     0x9a8791b2d2e5e7db as libc::c_ulonglong,
     0xd1e7fee0b49d97a0 as libc::c_ulonglong,
     0x997b5d3b3f4a638e as libc::c_ulonglong,
     0x9faaaea6a8b2adb3 as libc::c_ulonglong,
     0xb4b6beb6a2a2b8d0 as libc::c_ulonglong,
     0xfbfffffeece5dfc8 as libc::c_ulonglong,
     0x96787d877d7c7b8b as libc::c_ulonglong,
     0x9fb3c9dbeff1dbba as libc::c_ulonglong,
     0x9f91a2c3d9dcd3bf as libc::c_ulonglong,
     0xc0defbecd5c9c6bd as libc::c_ulonglong,
     0xa2795b383c54769e as libc::c_ulonglong,
     0xa5b7b8acacada8ac as libc::c_ulonglong,
     0xadb6beb4a5afd7f0 as libc::c_ulonglong,
     0xfffffffbfcfff3ca as libc::c_ulonglong,
     0x9e909d9681868a9d as libc::c_ulonglong,
     0xaab7d4f2fff5d9ba as libc::c_ulonglong,
     0xa4a1b7d5d6bda69f as libc::c_ulonglong,
     0xafd0effbfaeddfc1 as libc::c_ulonglong,
     0x9b705132446e94a6 as libc::c_ulonglong,
     0xa9bfc1b3aca8a5a9 as libc::c_ulonglong,
     0xafbcbeb7b5c8e0f2 as libc::c_ulonglong,
     0xfffff3eaf7fff3c7 as libc::c_ulonglong,
     0xb0b0ab8a818991a2 as libc::c_ulonglong,
     0xacc6e5fffff7d9bd as libc::c_ulonglong,
     0xadb3ccd8b9867681 as libc::c_ulonglong,
     0x9dbcdbf9fff7e0ba as libc::c_ulonglong,
     0x8e6545346194a49f as libc::c_ulonglong,
     0xa6c4ccc0afa9aab2 as libc::c_ulonglong,
     0xbec0bfc2cbcdccde as libc::c_ulonglong,
     0xf3ead2d5e5f1eac7 as libc::c_ulonglong,
     0xc8c09d79787f8a93 as libc::c_ulonglong,
     0xa5d2f5fffff7d9c3 as libc::c_ulonglong,
     0xbcc4cdba83525268 as libc::c_ulonglong,
     0x85a4c9ebfbedd7ae as libc::c_ulonglong,
     0x8059404887a9938f as libc::c_ulonglong,
     0xa5ceded0bab3bac7 as libc::c_ulonglong,
     0xcbc2c3cdceb8b0c2 as libc::c_ulonglong,
     0xcbbdb7c1d2e0e1cc as libc::c_ulonglong,
     0xcaab846c66697280 as libc::c_ulonglong,
     0xa1d5fffffff6ddce as libc::c_ulonglong,
     0xcac2ac86502e364f as libc::c_ulonglong,
     0x6e94bde1f3e1cc9f as libc::c_longlong as u64_0,
     0x7054506e98977c89 as libc::c_longlong as u64_0,
     0xade0f0dccbc8d4d9 as libc::c_ulonglong,
     0xd0c7c8c8b89a9398 as libc::c_ulonglong,
     0x979aa1b3c7d5dcd0 as libc::c_ulonglong,
     0xaa816b5a51515b76 as libc::c_ulonglong,
     0xa2d5fcfffff6e5d9 as libc::c_ulonglong,
     0xc7a5805830161c36 as libc::c_ulonglong,
     0x5c8ab8dcecd9c08d as libc::c_longlong as u64_0,
     0x67606e86857a7b90 as libc::c_longlong as u64_0,
     0xbff4fdeae1e3e8df as libc::c_ulonglong,
     0xd2cbc3b49a7f706b as libc::c_ulonglong,
     0x718394adc2d0d2c4 as libc::c_longlong as u64_0,
     0x7b5b4a3e383a4d73 as libc::c_longlong as u64_0,
     0xa4d1effffff9ead7 as libc::c_ulonglong,
     0xaf855e3c1f090825 as libc::c_ulonglong,
     0x5184b5d7e7d1af80 as libc::c_longlong as u64_0,
     0x676e857f6b7089a6 as libc::c_longlong as u64_0,
     0xd9fffffbfafbefdd as libc::c_ulonglong,
     0xd4cbb4987f655050 as libc::c_ulonglong,
     0x5d7391adc1cac0a3 as libc::c_longlong as u64_0,
     0x583f2f211b274676 as libc::c_longlong as u64_0,
     0xa4c6e3fffffbe8c4 as libc::c_ulonglong,
     0x97704d2e1502001b as libc::c_ulonglong,
     0x4e83b3d4dcc2a27a as libc::c_longlong as u64_0,
     0x6c7b877066789bc4 as libc::c_longlong as u64_0,
     0xf1ffffffffffebd6 as libc::c_ulonglong,
     0xd0c1a17c62483d47 as libc::c_ulonglong,
     0x577396aec0bfa67b as libc::c_longlong as u64_0,
     0x3f2d1f130b1a4376 as libc::c_longlong as u64_0,
     0x9fc1e3fffff7dbae as libc::c_ulonglong,
     0x846142260f000017 as libc::c_ulonglong,
     0x4d84b6d0ccb39879 as libc::c_longlong as u64_0,
     0x75838273728bb8e1 as libc::c_longlong as u64_0,
     0xfffffffffffbe0c8 as libc::c_ulonglong,
     0xc1af895b3f313544 as libc::c_ulonglong,
     0x5a7da0b3baad865c as libc::c_longlong as u64_0,
     0x3123190d08134071 as libc::c_longlong as u64_0,
     0x9fc9f3ffffefc89a as libc::c_ulonglong,
     0x735339210c02061c as libc::c_longlong as u64_0,
     0x518bb7c7bca28e7f as libc::c_longlong as u64_0,
     0x7c85858486a5d6f4 as libc::c_longlong as u64_0,
     0xffffffffffefcbb2 as libc::c_ulonglong,
     0xac96663520253147 as libc::c_ulonglong,
     0x6791b3b6af946948 as libc::c_longlong as u64_0,
     0x2a1f130908193e73 as libc::c_longlong as u64_0,
     0xade3fffffddeb287 as libc::c_ulonglong,
     0x6448321f0c081427 as libc::c_longlong as u64_0,
     0x5b8fb3bcaa8f8782 as libc::c_longlong as u64_0,
     0x81869198a0c3ebff as libc::c_ulonglong,
     0xfffffffffeddad96 as libc::c_ulonglong,
     0x8e713e170f1d3252 as libc::c_ulonglong,
     0x7caac3b5a07c563c as libc::c_longlong as u64_0,
     0x281c0b050c214583 as libc::c_longlong as u64_0,
     0xcbfffffff2ca9b73 as libc::c_ulonglong,
     0x543d2d1c1016283a as libc::c_longlong as u64_0,
     0x6494afaf9b828281 as libc::c_longlong as u64_0,
     0x82899daabcdffaff as libc::c_ulonglong,
     0xfffffffff1c18f76 as libc::c_ulonglong,
     0x69481d05061a3b65 as libc::c_longlong as u64_0,
     0x95bec7ab8e694633 as libc::c_ulonglong,
     0x28150001132f57a1 as libc::c_longlong as u64_0,
     0xedffffffdfb28360 as libc::c_ulonglong,
     0x4635271d1c293c49 as libc::c_longlong as u64_0,
     0x6e97a9a58e797a7b as libc::c_longlong as u64_0,
     0x7c89a5bdd4effcff as libc::c_longlong as u64_0,
     0xffffffffdba47758 as libc::c_ulonglong,
     0x44250800021e4a7c as libc::c_longlong as u64_0,
     0xa9c7bc9d7c5a3b2f as libc::c_ulonglong,
     0x270c00021a3f73c3 as libc::c_longlong as u64_0,
     0xfefffff4c9976b4e as libc::c_ulonglong,
     0x3b2d23252d3c4857 as libc::c_longlong as u64_0,
     0x7c9aa69b81716d70 as libc::c_longlong as u64_0,
     0x7385b2cedce5f3ff as libc::c_longlong as u64_0,
     0xffffffefbd8b623e as libc::c_ulonglong,
     0x260f0000072b5e91 as libc::c_longlong as u64_0,
     0xb4c0ac8d6d4e3832 as libc::c_ulonglong,
     0x26030006215294d6 as libc::c_longlong as u64_0,
     0xffffffe1ae7b563f as libc::c_ulonglong,
     0x2e2324303d475067 as libc::c_longlong as u64_0,
     0x8ca3a68f73645e65 as libc::c_ulonglong,
     0x6d87bacdc7ceebff as libc::c_longlong as u64_0,
     0xfffffed3a1774e2b as libc::c_ulonglong,
     0x13020000103a729f as libc::c_longlong as u64_0,
     0xb3b49d7b604a433e as libc::c_ulonglong,
     0x2000000a2a66a4d9 as libc::c_longlong as u64_0,
     0xfffff4cb93654532 as libc::c_ulonglong,
     0x211b283a464d5a78 as libc::c_longlong as u64_0,
     0x9eafa2816454525d as libc::c_ulonglong,
     0x6f8db3b0a8bfe3ff as libc::c_longlong as u64_0,
     0xffffe9b88d673f1d as libc::c_ulonglong,
     0x70000021d4882a0 as libc::c_longlong as u64_0,
     0xadab8d6f59575a48 as libc::c_ulonglong,
     0x1100000d3572aade as libc::c_longlong as u64_0,
     0xffffe0ac79513424 as libc::c_ulonglong,
     0x16162a3d4750668b as libc::c_longlong as u64_0,
     0xafb49d7152484553 as libc::c_ulonglong,
     0x6e838f878fb3d9fb as libc::c_longlong as u64_0,
     0xfff6cca07d5d3210 as libc::c_ulonglong,
     0xd2a5a869e as libc::c_longlong as u64_0,
     0xa69d7d6562706e45 as libc::c_ulonglong,
     0x3153d77afdd as libc::c_longlong as u64_0,
     0xf0e6ba865e392314 as libc::c_ulonglong,
     0xc142a3c4654729e as libc::c_longlong as u64_0,
     0xb6af8c63493a3547 as libc::c_ulonglong,
     0x5b6161687fa6d0e9 as libc::c_longlong as u64_0,
     0xedd7ac89714a2107 as libc::c_ulonglong,
     0x51a3a62879a as libc::c_longlong as u64_0,
     0x97876d66767d6026 as libc::c_ulonglong,
     0x10b254a7fb0d3 as libc::c_longlong as u64_0,
     0xdbc1956b492a1406 as libc::c_ulonglong,
     0x7142a39465e83ad as libc::c_longlong as u64_0,
     0xb79f755d492d293b as libc::c_ulonglong,
     0x463e3c5476a0c1d5 as libc::c_longlong as u64_0,
     0xd7bf9b7c5c311705 as libc::c_ulonglong,
     0xc2645698b91 as libc::c_longlong as u64_0,
     0x8b786976856d3002 as libc::c_ulonglong,
     0x91a355d85a9c6 as libc::c_longlong as u64_0,
     0xbfa1805e3d1f0700 as libc::c_ulonglong,
     0x51627354c6d99b9 as libc::c_longlong as u64_0,
     0xb28a685842252439 as libc::c_ulonglong,
     0x3c28294e7798b5cc as libc::c_longlong as u64_0,
     0xd0b7987040261505 as libc::c_ulonglong,
     0x2143051788c8f as libc::c_longlong as u64_0,
     0x86737282834e1200 as libc::c_ulonglong];
// Initialized in run_static_initializers
static mut sOuterCylinderDL: [Gfx; 33] =
    [Gfx{words: Gwords{w0: 0, w1: 0,},}; 33];
static mut sOuterCylinderLimb: SkelCurveLimb =
    unsafe {
        {
            let mut init =
                SkelCurveLimb{firstChildIdx: 0xff as libc::c_int as u8_0,
                              nextLimbIdx: 0xff as libc::c_int as u8_0,
                              dList:
                                  [0 as *const Gfx as *mut Gfx,
                                   sOuterCylinderDL.as_ptr() as *mut _],};
            init
        }
    };
static mut sSkelLimbs: [*mut SkelCurveLimb; 3] =
    unsafe {
        [&sRootLimb as *const SkelCurveLimb as *mut SkelCurveLimb,
         &sInnerCylinderLimb as *const SkelCurveLimb as *mut SkelCurveLimb,
         &sOuterCylinderLimb as *const SkelCurveLimb as *mut SkelCurveLimb]
    };
static mut sSkel: SkelCurveLimbList =
    unsafe {
        {
            let mut init =
                SkelCurveLimbList{limbs: sSkelLimbs.as_ptr() as *mut _,
                                  limbCount: 3 as libc::c_int as u8_0,};
            init
        }
    };
static mut sCylinderVtx: [Vtx; 36] =
    [Vtx{v:
             {
                 let mut init =
                     Vtx_t{ob:
                               [0 as libc::c_int as libc::c_short,
                                0 as libc::c_int as libc::c_short,
                                -(6000 as libc::c_int) as libc::c_short],
                           flag: 0 as libc::c_int as libc::c_ushort,
                           tc:
                               [1024 as libc::c_int as libc::c_short,
                                2048 as libc::c_int as libc::c_short],
                           cn:
                               [255 as libc::c_int as libc::c_uchar,
                                255 as libc::c_int as libc::c_uchar,
                                255 as libc::c_int as libc::c_uchar,
                                255 as libc::c_int as libc::c_uchar],};
                 init
             },},
     Vtx{v:
             {
                 let mut init =
                     Vtx_t{ob:
                               [-(4243 as libc::c_int) as libc::c_short,
                                12800 as libc::c_int as libc::c_short,
                                -(4243 as libc::c_int) as libc::c_short],
                           flag: 0 as libc::c_int as libc::c_ushort,
                           tc:
                               [1280 as libc::c_int as libc::c_short,
                                0 as libc::c_int as libc::c_short],
                           cn:
                               [255 as libc::c_int as libc::c_uchar,
                                255 as libc::c_int as libc::c_uchar,
                                255 as libc::c_int as libc::c_uchar,
                                0 as libc::c_int as libc::c_uchar],};
                 init
             },},
     Vtx{v:
             {
                 let mut init =
                     Vtx_t{ob:
                               [0 as libc::c_int as libc::c_short,
                                12800 as libc::c_int as libc::c_short,
                                -(6000 as libc::c_int) as libc::c_short],
                           flag: 0 as libc::c_int as libc::c_ushort,
                           tc:
                               [1024 as libc::c_int as libc::c_short,
                                0 as libc::c_int as libc::c_short],
                           cn:
                               [255 as libc::c_int as libc::c_uchar,
                                255 as libc::c_int as libc::c_uchar,
                                255 as libc::c_int as libc::c_uchar,
                                0 as libc::c_int as libc::c_uchar],};
                 init
             },},
     Vtx{v:
             {
                 let mut init =
                     Vtx_t{ob:
                               [-(4243 as libc::c_int) as libc::c_short,
                                0 as libc::c_int as libc::c_short,
                                -(4243 as libc::c_int) as libc::c_short],
                           flag: 0 as libc::c_int as libc::c_ushort,
                           tc:
                               [1280 as libc::c_int as libc::c_short,
                                2048 as libc::c_int as libc::c_short],
                           cn:
                               [255 as libc::c_int as libc::c_uchar,
                                255 as libc::c_int as libc::c_uchar,
                                255 as libc::c_int as libc::c_uchar,
                                255 as libc::c_int as libc::c_uchar],};
                 init
             },},
     Vtx{v:
             {
                 let mut init =
                     Vtx_t{ob:
                               [0 as libc::c_int as libc::c_short,
                                0 as libc::c_int as libc::c_short,
                                6000 as libc::c_int as libc::c_short],
                           flag: 0 as libc::c_int as libc::c_ushort,
                           tc:
                               [0 as libc::c_int as libc::c_short,
                                2048 as libc::c_int as libc::c_short],
                           cn:
                               [255 as libc::c_int as libc::c_uchar,
                                255 as libc::c_int as libc::c_uchar,
                                255 as libc::c_int as libc::c_uchar,
                                255 as libc::c_int as libc::c_uchar],};
                 init
             },},
     Vtx{v:
             {
                 let mut init =
                     Vtx_t{ob:
                               [4243 as libc::c_int as libc::c_short,
                                12800 as libc::c_int as libc::c_short,
                                4243 as libc::c_int as libc::c_short],
                           flag: 0 as libc::c_int as libc::c_ushort,
                           tc:
                               [256 as libc::c_int as libc::c_short,
                                0 as libc::c_int as libc::c_short],
                           cn:
                               [255 as libc::c_int as libc::c_uchar,
                                255 as libc::c_int as libc::c_uchar,
                                255 as libc::c_int as libc::c_uchar,
                                0 as libc::c_int as libc::c_uchar],};
                 init
             },},
     Vtx{v:
             {
                 let mut init =
                     Vtx_t{ob:
                               [0 as libc::c_int as libc::c_short,
                                12800 as libc::c_int as libc::c_short,
                                6000 as libc::c_int as libc::c_short],
                           flag: 0 as libc::c_int as libc::c_ushort,
                           tc:
                               [0 as libc::c_int as libc::c_short,
                                0 as libc::c_int as libc::c_short],
                           cn:
                               [255 as libc::c_int as libc::c_uchar,
                                255 as libc::c_int as libc::c_uchar,
                                255 as libc::c_int as libc::c_uchar,
                                0 as libc::c_int as libc::c_uchar],};
                 init
             },},
     Vtx{v:
             {
                 let mut init =
                     Vtx_t{ob:
                               [4243 as libc::c_int as libc::c_short,
                                0 as libc::c_int as libc::c_short,
                                4243 as libc::c_int as libc::c_short],
                           flag: 0 as libc::c_int as libc::c_ushort,
                           tc:
                               [256 as libc::c_int as libc::c_short,
                                2048 as libc::c_int as libc::c_short],
                           cn:
                               [255 as libc::c_int as libc::c_uchar,
                                255 as libc::c_int as libc::c_uchar,
                                255 as libc::c_int as libc::c_uchar,
                                255 as libc::c_int as libc::c_uchar],};
                 init
             },},
     Vtx{v:
             {
                 let mut init =
                     Vtx_t{ob:
                               [-(6000 as libc::c_int) as libc::c_short,
                                12800 as libc::c_int as libc::c_short,
                                0 as libc::c_int as libc::c_short],
                           flag: 0 as libc::c_int as libc::c_ushort,
                           tc:
                               [1536 as libc::c_int as libc::c_short,
                                0 as libc::c_int as libc::c_short],
                           cn:
                               [255 as libc::c_int as libc::c_uchar,
                                255 as libc::c_int as libc::c_uchar,
                                255 as libc::c_int as libc::c_uchar,
                                0 as libc::c_int as libc::c_uchar],};
                 init
             },},
     Vtx{v:
             {
                 let mut init =
                     Vtx_t{ob:
                               [-(6000 as libc::c_int) as libc::c_short,
                                0 as libc::c_int as libc::c_short,
                                0 as libc::c_int as libc::c_short],
                           flag: 0 as libc::c_int as libc::c_ushort,
                           tc:
                               [1536 as libc::c_int as libc::c_short,
                                2048 as libc::c_int as libc::c_short],
                           cn:
                               [255 as libc::c_int as libc::c_uchar,
                                255 as libc::c_int as libc::c_uchar,
                                255 as libc::c_int as libc::c_uchar,
                                255 as libc::c_int as libc::c_uchar],};
                 init
             },},
     Vtx{v:
             {
                 let mut init =
                     Vtx_t{ob:
                               [4243 as libc::c_int as libc::c_short,
                                0 as libc::c_int as libc::c_short,
                                -(4243 as libc::c_int) as libc::c_short],
                           flag: 0 as libc::c_int as libc::c_ushort,
                           tc:
                               [768 as libc::c_int as libc::c_short,
                                2048 as libc::c_int as libc::c_short],
                           cn:
                               [255 as libc::c_int as libc::c_uchar,
                                255 as libc::c_int as libc::c_uchar,
                                255 as libc::c_int as libc::c_uchar,
                                255 as libc::c_int as libc::c_uchar],};
                 init
             },},
     Vtx{v:
             {
                 let mut init =
                     Vtx_t{ob:
                               [4243 as libc::c_int as libc::c_short,
                                12800 as libc::c_int as libc::c_short,
                                -(4243 as libc::c_int) as libc::c_short],
                           flag: 0 as libc::c_int as libc::c_ushort,
                           tc:
                               [768 as libc::c_int as libc::c_short,
                                0 as libc::c_int as libc::c_short],
                           cn:
                               [255 as libc::c_int as libc::c_uchar,
                                255 as libc::c_int as libc::c_uchar,
                                255 as libc::c_int as libc::c_uchar,
                                0 as libc::c_int as libc::c_uchar],};
                 init
             },},
     Vtx{v:
             {
                 let mut init =
                     Vtx_t{ob:
                               [-(4243 as libc::c_int) as libc::c_short,
                                12800 as libc::c_int as libc::c_short,
                                4243 as libc::c_int as libc::c_short],
                           flag: 0 as libc::c_int as libc::c_ushort,
                           tc:
                               [1792 as libc::c_int as libc::c_short,
                                0 as libc::c_int as libc::c_short],
                           cn:
                               [255 as libc::c_int as libc::c_uchar,
                                255 as libc::c_int as libc::c_uchar,
                                255 as libc::c_int as libc::c_uchar,
                                0 as libc::c_int as libc::c_uchar],};
                 init
             },},
     Vtx{v:
             {
                 let mut init =
                     Vtx_t{ob:
                               [-(4243 as libc::c_int) as libc::c_short,
                                0 as libc::c_int as libc::c_short,
                                4243 as libc::c_int as libc::c_short],
                           flag: 0 as libc::c_int as libc::c_ushort,
                           tc:
                               [1792 as libc::c_int as libc::c_short,
                                2048 as libc::c_int as libc::c_short],
                           cn:
                               [255 as libc::c_int as libc::c_uchar,
                                255 as libc::c_int as libc::c_uchar,
                                255 as libc::c_int as libc::c_uchar,
                                255 as libc::c_int as libc::c_uchar],};
                 init
             },},
     Vtx{v:
             {
                 let mut init =
                     Vtx_t{ob:
                               [6000 as libc::c_int as libc::c_short,
                                12800 as libc::c_int as libc::c_short,
                                0 as libc::c_int as libc::c_short],
                           flag: 0 as libc::c_int as libc::c_ushort,
                           tc:
                               [512 as libc::c_int as libc::c_short,
                                0 as libc::c_int as libc::c_short],
                           cn:
                               [255 as libc::c_int as libc::c_uchar,
                                255 as libc::c_int as libc::c_uchar,
                                255 as libc::c_int as libc::c_uchar,
                                0 as libc::c_int as libc::c_uchar],};
                 init
             },},
     Vtx{v:
             {
                 let mut init =
                     Vtx_t{ob:
                               [6000 as libc::c_int as libc::c_short,
                                0 as libc::c_int as libc::c_short,
                                0 as libc::c_int as libc::c_short],
                           flag: 0 as libc::c_int as libc::c_ushort,
                           tc:
                               [512 as libc::c_int as libc::c_short,
                                2048 as libc::c_int as libc::c_short],
                           cn:
                               [255 as libc::c_int as libc::c_uchar,
                                255 as libc::c_int as libc::c_uchar,
                                255 as libc::c_int as libc::c_uchar,
                                255 as libc::c_int as libc::c_uchar],};
                 init
             },},
     Vtx{v:
             {
                 let mut init =
                     Vtx_t{ob:
                               [0 as libc::c_int as libc::c_short,
                                12800 as libc::c_int as libc::c_short,
                                6000 as libc::c_int as libc::c_short],
                           flag: 0 as libc::c_int as libc::c_ushort,
                           tc:
                               [2048 as libc::c_int as libc::c_short,
                                0 as libc::c_int as libc::c_short],
                           cn:
                               [255 as libc::c_int as libc::c_uchar,
                                255 as libc::c_int as libc::c_uchar,
                                255 as libc::c_int as libc::c_uchar,
                                0 as libc::c_int as libc::c_uchar],};
                 init
             },},
     Vtx{v:
             {
                 let mut init =
                     Vtx_t{ob:
                               [0 as libc::c_int as libc::c_short,
                                0 as libc::c_int as libc::c_short,
                                6000 as libc::c_int as libc::c_short],
                           flag: 0 as libc::c_int as libc::c_ushort,
                           tc:
                               [2048 as libc::c_int as libc::c_short,
                                2048 as libc::c_int as libc::c_short],
                           cn:
                               [255 as libc::c_int as libc::c_uchar,
                                255 as libc::c_int as libc::c_uchar,
                                255 as libc::c_int as libc::c_uchar,
                                255 as libc::c_int as libc::c_uchar],};
                 init
             },},
     Vtx{v:
             {
                 let mut init =
                     Vtx_t{ob:
                               [0 as libc::c_int as libc::c_short,
                                0 as libc::c_int as libc::c_short,
                                -(6000 as libc::c_int) as libc::c_short],
                           flag: 0 as libc::c_int as libc::c_ushort,
                           tc:
                               [1024 as libc::c_int as libc::c_short,
                                2048 as libc::c_int as libc::c_short],
                           cn:
                               [255 as libc::c_int as libc::c_uchar,
                                255 as libc::c_int as libc::c_uchar,
                                255 as libc::c_int as libc::c_uchar,
                                255 as libc::c_int as libc::c_uchar],};
                 init
             },},
     Vtx{v:
             {
                 let mut init =
                     Vtx_t{ob:
                               [-(4243 as libc::c_int) as libc::c_short,
                                12800 as libc::c_int as libc::c_short,
                                -(4243 as libc::c_int) as libc::c_short],
                           flag: 0 as libc::c_int as libc::c_ushort,
                           tc:
                               [1280 as libc::c_int as libc::c_short,
                                0 as libc::c_int as libc::c_short],
                           cn:
                               [255 as libc::c_int as libc::c_uchar,
                                255 as libc::c_int as libc::c_uchar,
                                255 as libc::c_int as libc::c_uchar,
                                0 as libc::c_int as libc::c_uchar],};
                 init
             },},
     Vtx{v:
             {
                 let mut init =
                     Vtx_t{ob:
                               [0 as libc::c_int as libc::c_short,
                                12800 as libc::c_int as libc::c_short,
                                -(6000 as libc::c_int) as libc::c_short],
                           flag: 0 as libc::c_int as libc::c_ushort,
                           tc:
                               [1024 as libc::c_int as libc::c_short,
                                0 as libc::c_int as libc::c_short],
                           cn:
                               [255 as libc::c_int as libc::c_uchar,
                                255 as libc::c_int as libc::c_uchar,
                                255 as libc::c_int as libc::c_uchar,
                                0 as libc::c_int as libc::c_uchar],};
                 init
             },},
     Vtx{v:
             {
                 let mut init =
                     Vtx_t{ob:
                               [-(4243 as libc::c_int) as libc::c_short,
                                0 as libc::c_int as libc::c_short,
                                -(4243 as libc::c_int) as libc::c_short],
                           flag: 0 as libc::c_int as libc::c_ushort,
                           tc:
                               [1280 as libc::c_int as libc::c_short,
                                2048 as libc::c_int as libc::c_short],
                           cn:
                               [255 as libc::c_int as libc::c_uchar,
                                255 as libc::c_int as libc::c_uchar,
                                255 as libc::c_int as libc::c_uchar,
                                255 as libc::c_int as libc::c_uchar],};
                 init
             },},
     Vtx{v:
             {
                 let mut init =
                     Vtx_t{ob:
                               [0 as libc::c_int as libc::c_short,
                                0 as libc::c_int as libc::c_short,
                                6000 as libc::c_int as libc::c_short],
                           flag: 0 as libc::c_int as libc::c_ushort,
                           tc:
                               [0 as libc::c_int as libc::c_short,
                                2048 as libc::c_int as libc::c_short],
                           cn:
                               [255 as libc::c_int as libc::c_uchar,
                                255 as libc::c_int as libc::c_uchar,
                                255 as libc::c_int as libc::c_uchar,
                                255 as libc::c_int as libc::c_uchar],};
                 init
             },},
     Vtx{v:
             {
                 let mut init =
                     Vtx_t{ob:
                               [4243 as libc::c_int as libc::c_short,
                                12800 as libc::c_int as libc::c_short,
                                4243 as libc::c_int as libc::c_short],
                           flag: 0 as libc::c_int as libc::c_ushort,
                           tc:
                               [256 as libc::c_int as libc::c_short,
                                0 as libc::c_int as libc::c_short],
                           cn:
                               [255 as libc::c_int as libc::c_uchar,
                                255 as libc::c_int as libc::c_uchar,
                                255 as libc::c_int as libc::c_uchar,
                                0 as libc::c_int as libc::c_uchar],};
                 init
             },},
     Vtx{v:
             {
                 let mut init =
                     Vtx_t{ob:
                               [0 as libc::c_int as libc::c_short,
                                12800 as libc::c_int as libc::c_short,
                                6000 as libc::c_int as libc::c_short],
                           flag: 0 as libc::c_int as libc::c_ushort,
                           tc:
                               [0 as libc::c_int as libc::c_short,
                                0 as libc::c_int as libc::c_short],
                           cn:
                               [255 as libc::c_int as libc::c_uchar,
                                255 as libc::c_int as libc::c_uchar,
                                255 as libc::c_int as libc::c_uchar,
                                0 as libc::c_int as libc::c_uchar],};
                 init
             },},
     Vtx{v:
             {
                 let mut init =
                     Vtx_t{ob:
                               [4243 as libc::c_int as libc::c_short,
                                0 as libc::c_int as libc::c_short,
                                4243 as libc::c_int as libc::c_short],
                           flag: 0 as libc::c_int as libc::c_ushort,
                           tc:
                               [256 as libc::c_int as libc::c_short,
                                2048 as libc::c_int as libc::c_short],
                           cn:
                               [255 as libc::c_int as libc::c_uchar,
                                255 as libc::c_int as libc::c_uchar,
                                255 as libc::c_int as libc::c_uchar,
                                255 as libc::c_int as libc::c_uchar],};
                 init
             },},
     Vtx{v:
             {
                 let mut init =
                     Vtx_t{ob:
                               [-(6000 as libc::c_int) as libc::c_short,
                                12800 as libc::c_int as libc::c_short,
                                0 as libc::c_int as libc::c_short],
                           flag: 0 as libc::c_int as libc::c_ushort,
                           tc:
                               [1536 as libc::c_int as libc::c_short,
                                0 as libc::c_int as libc::c_short],
                           cn:
                               [255 as libc::c_int as libc::c_uchar,
                                255 as libc::c_int as libc::c_uchar,
                                255 as libc::c_int as libc::c_uchar,
                                0 as libc::c_int as libc::c_uchar],};
                 init
             },},
     Vtx{v:
             {
                 let mut init =
                     Vtx_t{ob:
                               [-(6000 as libc::c_int) as libc::c_short,
                                0 as libc::c_int as libc::c_short,
                                0 as libc::c_int as libc::c_short],
                           flag: 0 as libc::c_int as libc::c_ushort,
                           tc:
                               [1536 as libc::c_int as libc::c_short,
                                2048 as libc::c_int as libc::c_short],
                           cn:
                               [255 as libc::c_int as libc::c_uchar,
                                255 as libc::c_int as libc::c_uchar,
                                255 as libc::c_int as libc::c_uchar,
                                255 as libc::c_int as libc::c_uchar],};
                 init
             },},
     Vtx{v:
             {
                 let mut init =
                     Vtx_t{ob:
                               [4243 as libc::c_int as libc::c_short,
                                0 as libc::c_int as libc::c_short,
                                -(4243 as libc::c_int) as libc::c_short],
                           flag: 0 as libc::c_int as libc::c_ushort,
                           tc:
                               [768 as libc::c_int as libc::c_short,
                                2048 as libc::c_int as libc::c_short],
                           cn:
                               [255 as libc::c_int as libc::c_uchar,
                                255 as libc::c_int as libc::c_uchar,
                                255 as libc::c_int as libc::c_uchar,
                                255 as libc::c_int as libc::c_uchar],};
                 init
             },},
     Vtx{v:
             {
                 let mut init =
                     Vtx_t{ob:
                               [4243 as libc::c_int as libc::c_short,
                                12800 as libc::c_int as libc::c_short,
                                -(4243 as libc::c_int) as libc::c_short],
                           flag: 0 as libc::c_int as libc::c_ushort,
                           tc:
                               [768 as libc::c_int as libc::c_short,
                                0 as libc::c_int as libc::c_short],
                           cn:
                               [255 as libc::c_int as libc::c_uchar,
                                255 as libc::c_int as libc::c_uchar,
                                255 as libc::c_int as libc::c_uchar,
                                0 as libc::c_int as libc::c_uchar],};
                 init
             },},
     Vtx{v:
             {
                 let mut init =
                     Vtx_t{ob:
                               [-(4243 as libc::c_int) as libc::c_short,
                                12800 as libc::c_int as libc::c_short,
                                4243 as libc::c_int as libc::c_short],
                           flag: 0 as libc::c_int as libc::c_ushort,
                           tc:
                               [1792 as libc::c_int as libc::c_short,
                                0 as libc::c_int as libc::c_short],
                           cn:
                               [255 as libc::c_int as libc::c_uchar,
                                255 as libc::c_int as libc::c_uchar,
                                255 as libc::c_int as libc::c_uchar,
                                0 as libc::c_int as libc::c_uchar],};
                 init
             },},
     Vtx{v:
             {
                 let mut init =
                     Vtx_t{ob:
                               [-(4243 as libc::c_int) as libc::c_short,
                                0 as libc::c_int as libc::c_short,
                                4243 as libc::c_int as libc::c_short],
                           flag: 0 as libc::c_int as libc::c_ushort,
                           tc:
                               [1792 as libc::c_int as libc::c_short,
                                2048 as libc::c_int as libc::c_short],
                           cn:
                               [255 as libc::c_int as libc::c_uchar,
                                255 as libc::c_int as libc::c_uchar,
                                255 as libc::c_int as libc::c_uchar,
                                255 as libc::c_int as libc::c_uchar],};
                 init
             },},
     Vtx{v:
             {
                 let mut init =
                     Vtx_t{ob:
                               [6000 as libc::c_int as libc::c_short,
                                12800 as libc::c_int as libc::c_short,
                                0 as libc::c_int as libc::c_short],
                           flag: 0 as libc::c_int as libc::c_ushort,
                           tc:
                               [512 as libc::c_int as libc::c_short,
                                0 as libc::c_int as libc::c_short],
                           cn:
                               [255 as libc::c_int as libc::c_uchar,
                                255 as libc::c_int as libc::c_uchar,
                                255 as libc::c_int as libc::c_uchar,
                                0 as libc::c_int as libc::c_uchar],};
                 init
             },},
     Vtx{v:
             {
                 let mut init =
                     Vtx_t{ob:
                               [6000 as libc::c_int as libc::c_short,
                                0 as libc::c_int as libc::c_short,
                                0 as libc::c_int as libc::c_short],
                           flag: 0 as libc::c_int as libc::c_ushort,
                           tc:
                               [512 as libc::c_int as libc::c_short,
                                2048 as libc::c_int as libc::c_short],
                           cn:
                               [255 as libc::c_int as libc::c_uchar,
                                255 as libc::c_int as libc::c_uchar,
                                255 as libc::c_int as libc::c_uchar,
                                255 as libc::c_int as libc::c_uchar],};
                 init
             },},
     Vtx{v:
             {
                 let mut init =
                     Vtx_t{ob:
                               [0 as libc::c_int as libc::c_short,
                                12800 as libc::c_int as libc::c_short,
                                6000 as libc::c_int as libc::c_short],
                           flag: 0 as libc::c_int as libc::c_ushort,
                           tc:
                               [2048 as libc::c_int as libc::c_short,
                                0 as libc::c_int as libc::c_short],
                           cn:
                               [255 as libc::c_int as libc::c_uchar,
                                255 as libc::c_int as libc::c_uchar,
                                255 as libc::c_int as libc::c_uchar,
                                0 as libc::c_int as libc::c_uchar],};
                 init
             },},
     Vtx{v:
             {
                 let mut init =
                     Vtx_t{ob:
                               [0 as libc::c_int as libc::c_short,
                                0 as libc::c_int as libc::c_short,
                                6000 as libc::c_int as libc::c_short],
                           flag: 0 as libc::c_int as libc::c_ushort,
                           tc:
                               [2048 as libc::c_int as libc::c_short,
                                2048 as libc::c_int as libc::c_short],
                           cn:
                               [255 as libc::c_int as libc::c_uchar,
                                255 as libc::c_int as libc::c_uchar,
                                255 as libc::c_int as libc::c_uchar,
                                255 as libc::c_int as libc::c_uchar],};
                 init
             },}];
static mut sAlphaUpdVals: [u8_0; 18] =
    [0 as libc::c_int as u8_0, 0x3 as libc::c_int as u8_0,
     0x4 as libc::c_int as u8_0, 0x7 as libc::c_int as u8_0,
     0x9 as libc::c_int as u8_0, 0xa as libc::c_int as u8_0,
     0xd as libc::c_int as u8_0, 0xf as libc::c_int as u8_0,
     0x11 as libc::c_int as u8_0, 0x12 as libc::c_int as u8_0,
     0x15 as libc::c_int as u8_0, 0x16 as libc::c_int as u8_0,
     0x19 as libc::c_int as u8_0, 0x1b as libc::c_int as u8_0,
     0x1c as libc::c_int as u8_0, 0x1f as libc::c_int as u8_0,
     0x21 as libc::c_int as u8_0, 0x23 as libc::c_int as u8_0];
#[no_mangle]
pub unsafe extern "C" fn MagicWind_SetupAction(mut this: *mut MagicWind,
                                               mut actionFunc:
                                                   MagicWindFunc) {
    (*this).actionFunc = actionFunc;
}
#[no_mangle]
pub unsafe extern "C" fn MagicWind_Init(mut thisx: *mut Actor,
                                        mut globalCtx: *mut GlobalContext) {
    let mut this: *mut MagicWind = thisx as *mut MagicWind;
    let mut player: *mut Player =
        (*globalCtx).actorCtx.actorLists[ACTORCAT_PLAYER as libc::c_int as
                                             usize].head as *mut Player;
    if SkelCurve_Init(globalCtx, &mut (*this).skelCurve, &mut sSkel,
                      &mut sAnim) == 0 as libc::c_int {
        // "Magic_Wind_Actor_ct (): Construct failed"
        osSyncPrintf(b"Magic_Wind_Actor_ct():\xe3\x82\xb3\xe3\x83\xb3\xe3\x82\xb9\xe3\x83\x88\xe3\x83\xa9\xe3\x82\xaf\xe3\x83\x88\xe5\xa4\xb1\xe6\x95\x97\n\x00"
                         as *const u8 as *const libc::c_char);
    }
    (*this).actor.room = -(1 as libc::c_int) as s8;
    match (*this).actor.params as libc::c_int {
        0 => {
            SkelCurve_SetAnim(&mut (*this).skelCurve, &mut sAnim, 0.0f32,
                              60.0f32, 0.0f32, 1.0f32);
            (*this).timer = 29 as libc::c_int as s16;
            MagicWind_SetupAction(this,
                                  Some(MagicWind_WaitForTimer as
                                           unsafe extern "C" fn(_:
                                                                    *mut MagicWind,
                                                                _:
                                                                    *mut GlobalContext)
                                               -> ()));
        }
        1 => {
            SkelCurve_SetAnim(&mut (*this).skelCurve, &mut sAnim, 60.0f32,
                              0.0f32, 60.0f32, -1.0f32);
            MagicWind_SetupAction(this,
                                  Some(MagicWind_Shrink as
                                           unsafe extern "C" fn(_:
                                                                    *mut MagicWind,
                                                                _:
                                                                    *mut GlobalContext)
                                               -> ()));
            // "Means start"
            LogUtils_LogThreadId(b"../z_magic_wind.c\x00" as *const u8 as
                                     *const libc::c_char, 486 as libc::c_int);
            osSyncPrintf(b"\"\xe8\xa1\xa8\xe7\xa4\xba\xe9\x96\x8b\xe5\xa7\x8b\" = %s\n\x00"
                             as *const u8 as *const libc::c_char,
                         b"\xe8\xa1\xa8\xe7\xa4\xba\xe9\x96\x8b\xe5\xa7\x8b\x00"
                             as *const u8 as *const libc::c_char);
            func_8002F7DC(&mut (*player).actor,
                          0x87b as libc::c_int as u16_0);
        }
        _ => { }
    };
}
#[no_mangle]
pub unsafe extern "C" fn MagicWind_Destroy(mut thisx: *mut Actor,
                                           mut globalCtx:
                                               *mut GlobalContext) {
    let mut this: *mut MagicWind = thisx as *mut MagicWind;
    SkelCurve_Destroy(globalCtx, &mut (*this).skelCurve);
    func_800876C8(globalCtx);
    // "wipe out"
    LogUtils_LogThreadId(b"../z_magic_wind.c\x00" as *const u8 as
                             *const libc::c_char, 505 as libc::c_int);
    osSyncPrintf(b"\"\xe6\xb6\x88\xe6\xbb\x85\" = %s\n\x00" as *const u8 as
                     *const libc::c_char,
                 b"\xe6\xb6\x88\xe6\xbb\x85\x00" as *const u8 as
                     *const libc::c_char);
}
#[no_mangle]
pub unsafe extern "C" fn MagicWind_UpdateAlpha(mut alpha: f32_0) {
    let mut i: s32 = 0;
    i = 0 as libc::c_int;
    while i <
              (::std::mem::size_of::<[u8_0; 18]>() as
                   libc::c_ulong).wrapping_div(::std::mem::size_of::<u8_0>()
                                                   as libc::c_ulong) as s32 {
        sCylinderVtx[sAlphaUpdVals[i as usize] as usize].n.a =
            (alpha * 255.0f32) as libc::c_uchar;
        i += 1
    };
}
#[no_mangle]
pub unsafe extern "C" fn MagicWind_WaitForTimer(mut this: *mut MagicWind,
                                                mut globalCtx:
                                                    *mut GlobalContext) {
    let mut player: *mut Player =
        (*globalCtx).actorCtx.actorLists[ACTORCAT_PLAYER as libc::c_int as
                                             usize].head as *mut Player;
    if (*this).timer as libc::c_int > 0 as libc::c_int {
        (*this).timer -= 1;
        return
    }
    // "Means start"
    LogUtils_LogThreadId(b"../z_magic_wind.c\x00" as *const u8 as
                             *const libc::c_char, 539 as libc::c_int);
    osSyncPrintf(b"\"\xe8\xa1\xa8\xe7\xa4\xba\xe9\x96\x8b\xe5\xa7\x8b\" = %s\n\x00"
                     as *const u8 as *const libc::c_char,
                 b"\xe8\xa1\xa8\xe7\xa4\xba\xe9\x96\x8b\xe5\xa7\x8b\x00" as
                     *const u8 as *const libc::c_char);
    func_8002F7DC(&mut (*player).actor, 0x87a as libc::c_int as u16_0);
    MagicWind_UpdateAlpha(1.0f32);
    MagicWind_SetupAction(this,
                          Some(MagicWind_Grow as
                                   unsafe extern "C" fn(_: *mut MagicWind,
                                                        _: *mut GlobalContext)
                                       -> ()));
    SkelCurve_Update(globalCtx, &mut (*this).skelCurve);
}
#[no_mangle]
pub unsafe extern "C" fn MagicWind_Grow(mut this: *mut MagicWind,
                                        mut globalCtx: *mut GlobalContext) {
    if SkelCurve_Update(globalCtx, &mut (*this).skelCurve) != 0 {
        MagicWind_SetupAction(this,
                              Some(MagicWind_WaitAtFullSize as
                                       unsafe extern "C" fn(_: *mut MagicWind,
                                                            _:
                                                                *mut GlobalContext)
                                           -> ()));
        (*this).timer = 50 as libc::c_int as s16
    };
}
#[no_mangle]
pub unsafe extern "C" fn MagicWind_WaitAtFullSize(mut this: *mut MagicWind,
                                                  mut globalCtx:
                                                      *mut GlobalContext) {
    if (*this).timer as libc::c_int > 0 as libc::c_int {
        (*this).timer -= 1
    } else {
        MagicWind_SetupAction(this,
                              Some(MagicWind_FadeOut as
                                       unsafe extern "C" fn(_: *mut MagicWind,
                                                            _:
                                                                *mut GlobalContext)
                                           -> ()));
        (*this).timer = 30 as libc::c_int as s16
    };
}
#[no_mangle]
pub unsafe extern "C" fn MagicWind_FadeOut(mut this: *mut MagicWind,
                                           mut globalCtx:
                                               *mut GlobalContext) {
    if (*this).timer as libc::c_int > 0 as libc::c_int {
        MagicWind_UpdateAlpha((*this).timer as f32_0 * (1.0f32 / 30.0f32));
        (*this).timer -= 1
    } else { Actor_Kill(&mut (*this).actor); };
}
#[no_mangle]
pub unsafe extern "C" fn MagicWind_Shrink(mut this: *mut MagicWind,
                                          mut globalCtx: *mut GlobalContext) {
    if SkelCurve_Update(globalCtx, &mut (*this).skelCurve) != 0 {
        Actor_Kill(&mut (*this).actor);
    };
}
#[no_mangle]
pub unsafe extern "C" fn MagicWind_Update(mut thisx: *mut Actor,
                                          mut globalCtx: *mut GlobalContext) {
    let mut this: *mut MagicWind = thisx as *mut MagicWind;
    if (*globalCtx).msgCtx.msgMode as libc::c_int ==
           MSGMODE_OCARINA_CORRECT_PLAYBACK as libc::c_int ||
           (*globalCtx).msgCtx.msgMode as libc::c_int ==
               MSGMODE_SONG_PLAYED as libc::c_int {
        Actor_Kill(thisx);
        return
    }
    (*this).actionFunc.expect("non-null function pointer")(this, globalCtx);
}
#[no_mangle]
pub unsafe extern "C" fn MagicWind_OverrideLimbDraw(mut globalCtx:
                                                        *mut GlobalContext,
                                                    mut skelCurve:
                                                        *mut SkelAnimeCurve,
                                                    mut limbIndex: s32,
                                                    mut thisx:
                                                        *mut libc::c_void)
 -> s32 {
    let mut this: *mut MagicWind = thisx as *mut MagicWind;
    let mut __gfxCtx: *mut GraphicsContext = 0 as *mut GraphicsContext;
    let mut dispRefs: [*mut Gfx; 4] = [0 as *mut Gfx; 4];
    __gfxCtx = (*globalCtx).state.gfxCtx;
    Graph_OpenDisps(dispRefs.as_mut_ptr(), (*globalCtx).state.gfxCtx,
                    b"../z_magic_wind.c\x00" as *const u8 as
                        *const libc::c_char, 615 as libc::c_int);
    if limbIndex == 1 as libc::c_int {
        let fresh0 = (*__gfxCtx).polyXlu.p;
        (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
        let mut _g: *mut Gfx = fresh0;
        (*_g).words.w0 =
            (0xdb as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 24 as libc::c_int
                |
                (0x6 as libc::c_int as u32_0 &
                     (((0x1 as libc::c_int) << 8 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    16 as libc::c_int |
                ((8 as libc::c_int * 4 as libc::c_int) as u32_0 &
                     (((0x1 as libc::c_int) << 16 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    0 as libc::c_int;
        (*_g).words.w1 =
            Gfx_TwoTexScroll((*globalCtx).state.gfxCtx, 0 as libc::c_int,
                             (*globalCtx).state.frames.wrapping_mul(9 as
                                                                        libc::c_int
                                                                        as
                                                                        libc::c_uint)
                                 & 0xff as libc::c_int as libc::c_uint,
                             (0xff as libc::c_int as
                                  libc::c_uint).wrapping_sub((*globalCtx).state.frames.wrapping_mul(0xf
                                                                                                        as
                                                                                                        libc::c_int
                                                                                                        as
                                                                                                        libc::c_uint)
                                                                 &
                                                                 0xff as
                                                                     libc::c_int
                                                                     as
                                                                     libc::c_uint),
                             0x40 as libc::c_int, 0x40 as libc::c_int,
                             1 as libc::c_int,
                             (*globalCtx).state.frames.wrapping_mul(0xf as
                                                                        libc::c_int
                                                                        as
                                                                        libc::c_uint)
                                 & 0xff as libc::c_int as libc::c_uint,
                             (0xff as libc::c_int as
                                  libc::c_uint).wrapping_sub((*globalCtx).state.frames.wrapping_mul(0x1e
                                                                                                        as
                                                                                                        libc::c_int
                                                                                                        as
                                                                                                        libc::c_uint)
                                                                 &
                                                                 0xff as
                                                                     libc::c_int
                                                                     as
                                                                     libc::c_uint),
                             0x40 as libc::c_int, 0x40 as libc::c_int) as
                libc::c_uint
    } else if limbIndex == 2 as libc::c_int {
        let fresh1 = (*__gfxCtx).polyXlu.p;
        (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
        let mut _g_0: *mut Gfx = fresh1;
        (*_g_0).words.w0 =
            (0xdb as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 24 as libc::c_int
                |
                (0x6 as libc::c_int as u32_0 &
                     (((0x1 as libc::c_int) << 8 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    16 as libc::c_int |
                ((9 as libc::c_int * 4 as libc::c_int) as u32_0 &
                     (((0x1 as libc::c_int) << 16 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    0 as libc::c_int;
        (*_g_0).words.w1 =
            Gfx_TwoTexScroll((*globalCtx).state.gfxCtx, 0 as libc::c_int,
                             (*globalCtx).state.frames.wrapping_mul(3 as
                                                                        libc::c_int
                                                                        as
                                                                        libc::c_uint)
                                 & 0xff as libc::c_int as libc::c_uint,
                             (0xff as libc::c_int as
                                  libc::c_uint).wrapping_sub((*globalCtx).state.frames.wrapping_mul(5
                                                                                                        as
                                                                                                        libc::c_int
                                                                                                        as
                                                                                                        libc::c_uint)
                                                                 &
                                                                 0xff as
                                                                     libc::c_int
                                                                     as
                                                                     libc::c_uint),
                             0x40 as libc::c_int, 0x40 as libc::c_int,
                             1 as libc::c_int,
                             (*globalCtx).state.frames.wrapping_mul(6 as
                                                                        libc::c_int
                                                                        as
                                                                        libc::c_uint)
                                 & 0xff as libc::c_int as libc::c_uint,
                             (0xff as libc::c_int as
                                  libc::c_uint).wrapping_sub((*globalCtx).state.frames.wrapping_mul(0xa
                                                                                                        as
                                                                                                        libc::c_int
                                                                                                        as
                                                                                                        libc::c_uint)
                                                                 &
                                                                 0xff as
                                                                     libc::c_int
                                                                     as
                                                                     libc::c_uint),
                             0x40 as libc::c_int, 0x40 as libc::c_int) as
                libc::c_uint
    }
    Graph_CloseDisps(dispRefs.as_mut_ptr(), (*globalCtx).state.gfxCtx,
                     b"../z_magic_wind.c\x00" as *const u8 as
                         *const libc::c_char, 646 as libc::c_int);
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn MagicWind_Draw(mut thisx: *mut Actor,
                                        mut globalCtx: *mut GlobalContext) {
    let mut gfxCtx: *mut GraphicsContext = (*globalCtx).state.gfxCtx;
    let mut this: *mut MagicWind = thisx as *mut MagicWind;
    let mut __gfxCtx: *mut GraphicsContext = 0 as *mut GraphicsContext;
    let mut dispRefs: [*mut Gfx; 4] = [0 as *mut Gfx; 4];
    __gfxCtx = gfxCtx;
    Graph_OpenDisps(dispRefs.as_mut_ptr(), gfxCtx,
                    b"../z_magic_wind.c\x00" as *const u8 as
                        *const libc::c_char, 661 as libc::c_int);
    if (*this).actionFunc !=
           Some(MagicWind_WaitForTimer as
                    unsafe extern "C" fn(_: *mut MagicWind,
                                         _: *mut GlobalContext) -> ()) {
        (*__gfxCtx).polyXlu.p =
            Gfx_CallSetupDL((*__gfxCtx).polyXlu.p,
                            25 as libc::c_int as u32_0);
        SkelCurve_Draw(thisx, globalCtx, &mut (*this).skelCurve,
                       Some(MagicWind_OverrideLimbDraw as
                                unsafe extern "C" fn(_: *mut GlobalContext,
                                                     _: *mut SkelAnimeCurve,
                                                     _: s32,
                                                     _: *mut libc::c_void)
                                    -> s32), None, 1 as libc::c_int,
                       0 as *mut libc::c_void);
    }
    Graph_CloseDisps(dispRefs.as_mut_ptr(), gfxCtx,
                     b"../z_magic_wind.c\x00" as *const u8 as
                         *const libc::c_char, 673 as libc::c_int);
}
unsafe extern "C" fn run_static_initializers() {
    sInnerCylinderDL =
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
                                w1: sTex.as_mut_ptr() as libc::c_uint,};
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
                                        ((0 as libc::c_int | 0 as libc::c_int)
                                             as u32_0 &
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
                                        (0 as libc::c_int as u32_0 &
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
                                                   1 as libc::c_int >>
                                                   1 as libc::c_int) -
                                                  1 as libc::c_int) <
                                                 2047 as libc::c_int {
                                              (64 as libc::c_int *
                                                   64 as libc::c_int +
                                                   1 as libc::c_int >>
                                                   1 as libc::c_int) -
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
                                                       64 as libc::c_int *
                                                           1 as libc::c_int /
                                                           8 as libc::c_int {
                                                    1 as libc::c_int
                                                } else {
                                                    (64 as libc::c_int *
                                                         1 as libc::c_int) /
                                                        8 as libc::c_int
                                                }) - 1 as libc::c_int) /
                                              (if 1 as libc::c_int >
                                                      64 as libc::c_int *
                                                          1 as libc::c_int /
                                                          8 as libc::c_int {
                                                   1 as libc::c_int
                                               } else {
                                                   (64 as libc::c_int *
                                                        1 as libc::c_int) /
                                                       8 as libc::c_int
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
                                        (1 as libc::c_int as u32_0 &
                                             (((0x1 as libc::c_int) <<
                                                   2 as libc::c_int) -
                                                  1 as libc::c_int) as
                                                 libc::c_uint) <<
                                            19 as libc::c_int |
                                        ((64 as libc::c_int * 1 as libc::c_int
                                              + 7 as libc::c_int >>
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
                                        ((0 as libc::c_int | 0 as libc::c_int)
                                             as u32_0 &
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
                                        (0 as libc::c_int as u32_0 &
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
                                w1: sTex.as_mut_ptr() as libc::c_uint,};
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
                                        ((0 as libc::c_int | 0 as libc::c_int)
                                             as u32_0 &
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
                                        (14 as libc::c_int as u32_0 &
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
                                        (14 as libc::c_int as u32_0 &
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
                                                   1 as libc::c_int >>
                                                   1 as libc::c_int) -
                                                  1 as libc::c_int) <
                                                 2047 as libc::c_int {
                                              (64 as libc::c_int *
                                                   64 as libc::c_int +
                                                   1 as libc::c_int >>
                                                   1 as libc::c_int) -
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
                                                       64 as libc::c_int *
                                                           1 as libc::c_int /
                                                           8 as libc::c_int {
                                                    1 as libc::c_int
                                                } else {
                                                    (64 as libc::c_int *
                                                         1 as libc::c_int) /
                                                        8 as libc::c_int
                                                }) - 1 as libc::c_int) /
                                              (if 1 as libc::c_int >
                                                      64 as libc::c_int *
                                                          1 as libc::c_int /
                                                          8 as libc::c_int {
                                                   1 as libc::c_int
                                               } else {
                                                   (64 as libc::c_int *
                                                        1 as libc::c_int) /
                                                       8 as libc::c_int
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
                                        (1 as libc::c_int as u32_0 &
                                             (((0x1 as libc::c_int) <<
                                                   2 as libc::c_int) -
                                                  1 as libc::c_int) as
                                                 libc::c_uint) <<
                                            19 as libc::c_int |
                                        ((64 as libc::c_int * 1 as libc::c_int
                                              + 7 as libc::c_int >>
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
                                        ((0 as libc::c_int | 0 as libc::c_int)
                                             as u32_0 &
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
                                        (14 as libc::c_int as u32_0 &
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
                                        (14 as libc::c_int as u32_0 &
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
                                              ((3 as libc::c_int as u32_0 &
                                                    (((0x1 as libc::c_int) <<
                                                          4 as libc::c_int) -
                                                         1 as libc::c_int) as
                                                        libc::c_uint) <<
                                                   5 as libc::c_int |
                                                   (0 as libc::c_int as u32_0
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
                                    (3 as libc::c_int as u32_0 &
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
                                        ((5 as libc::c_int as u32_0 &
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
                                             (4 as libc::c_int as u32_0 &
                                                  (((0x1 as libc::c_int) <<
                                                        3 as libc::c_int) -
                                                       1 as libc::c_int) as
                                                      libc::c_uint) <<
                                                 18 as libc::c_int |
                                             (5 as libc::c_int as u32_0 &
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
                                    (0xe2 as libc::c_int as u32_0 &
                                         (((0x1 as libc::c_int) <<
                                               8 as libc::c_int) -
                                              1 as libc::c_int) as
                                             libc::c_uint) <<
                                        24 as libc::c_int |
                                        ((32 as libc::c_int - 3 as libc::c_int
                                              - 29 as libc::c_int) as u32_0 &
                                             (((0x1 as libc::c_int) <<
                                                   8 as libc::c_int) -
                                                  1 as libc::c_int) as
                                                 libc::c_uint) <<
                                            8 as libc::c_int |
                                        ((29 as libc::c_int -
                                              1 as libc::c_int) as u32_0 &
                                             (((0x1 as libc::c_int) <<
                                                   8 as libc::c_int) -
                                                  1 as libc::c_int) as
                                                 libc::c_uint) <<
                                            0 as libc::c_int,
                                w1:
                                    ((0 as libc::c_int) << 30 as libc::c_int |
                                         (3 as libc::c_int) <<
                                             26 as libc::c_int |
                                         (0 as libc::c_int) <<
                                             22 as libc::c_int |
                                         (2 as libc::c_int) <<
                                             18 as libc::c_int |
                                         (0x8 as libc::c_int |
                                              0x10 as libc::c_int |
                                              0x40 as libc::c_int |
                                              0x100 as libc::c_int |
                                              0x80 as libc::c_int |
                                              0x4000 as libc::c_int |
                                              0x800 as libc::c_int |
                                              (0 as libc::c_int) <<
                                                  28 as libc::c_int |
                                              (0 as libc::c_int) <<
                                                  24 as libc::c_int |
                                              (1 as libc::c_int) <<
                                                  20 as libc::c_int |
                                              (0 as libc::c_int) <<
                                                  16 as libc::c_int)) as
                                        libc::c_uint,};
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
                                        (!((0x400 as libc::c_int |
                                                0x10000 as libc::c_int |
                                                0x20000 as libc::c_int |
                                                0x40000 as libc::c_int |
                                                0x80000 as libc::c_int) as
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
                                    (0xfa as libc::c_int as u32_0 &
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
                                            8 as libc::c_int |
                                        (0x80 as libc::c_int as u32_0 &
                                             (((0x1 as libc::c_int) <<
                                                   8 as libc::c_int) -
                                                  1 as libc::c_int) as
                                                 libc::c_uint) <<
                                            0 as libc::c_int,
                                w1:
                                    (255 as libc::c_int as u32_0 &
                                         (((0x1 as libc::c_int) <<
                                               8 as libc::c_int) -
                                              1 as libc::c_int) as
                                             libc::c_uint) <<
                                        24 as libc::c_int |
                                        (255 as libc::c_int as u32_0 &
                                             (((0x1 as libc::c_int) <<
                                                   8 as libc::c_int) -
                                                  1 as libc::c_int) as
                                                 libc::c_uint) <<
                                            16 as libc::c_int |
                                        (170 as libc::c_int as u32_0 &
                                             (((0x1 as libc::c_int) <<
                                                   8 as libc::c_int) -
                                                  1 as libc::c_int) as
                                                 libc::c_uint) <<
                                            8 as libc::c_int |
                                        (255 as libc::c_int as u32_0 &
                                             (((0x1 as libc::c_int) <<
                                                   8 as libc::c_int) -
                                                  1 as libc::c_int) as
                                                 libc::c_uint) <<
                                            0 as libc::c_int,};
                     init
                 },},
         Gfx{words:
                 {
                     let mut init =
                         Gwords{w0:
                                    (0xfb as libc::c_int as u32_0 &
                                         (((0x1 as libc::c_int) <<
                                               8 as libc::c_int) -
                                              1 as libc::c_int) as
                                             libc::c_uint) <<
                                        24 as libc::c_int,
                                w1:
                                    (150 as libc::c_int as u32_0 &
                                         (((0x1 as libc::c_int) <<
                                               8 as libc::c_int) -
                                              1 as libc::c_int) as
                                             libc::c_uint) <<
                                        24 as libc::c_int |
                                        (255 as libc::c_int as u32_0 &
                                             (((0x1 as libc::c_int) <<
                                                   8 as libc::c_int) -
                                                  1 as libc::c_int) as
                                                 libc::c_uint) <<
                                            16 as libc::c_int |
                                        (0 as libc::c_int as u32_0 &
                                             (((0x1 as libc::c_int) <<
                                                   8 as libc::c_int) -
                                                  1 as libc::c_int) as
                                                 libc::c_uint) <<
                                            8 as libc::c_int |
                                        (0 as libc::c_int as u32_0 &
                                             (((0x1 as libc::c_int) <<
                                                   8 as libc::c_int) -
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
                                        (18 as libc::c_int as u32_0 &
                                             (((0x1 as libc::c_int) <<
                                                   8 as libc::c_int) -
                                                  1 as libc::c_int) as
                                                 libc::c_uint) <<
                                            12 as libc::c_int |
                                        ((0 as libc::c_int +
                                              18 as libc::c_int) as u32_0 &
                                             (((0x1 as libc::c_int) <<
                                                   7 as libc::c_int) -
                                                  1 as libc::c_int) as
                                                 libc::c_uint) <<
                                            1 as libc::c_int,
                                w1:
                                    sCylinderVtx.as_mut_ptr() as
                                        libc::c_uint,};
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
                                        (((0 as libc::c_int *
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
                                            ((1 as libc::c_int *
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
                                             ((1 as libc::c_int *
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
                                    } else {
                                        (((1 as libc::c_int *
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
                                             (((4 as libc::c_int *
                                                    2 as libc::c_int) as u32_0
                                                   &
                                                   (((0x1 as libc::c_int) <<
                                                         8 as libc::c_int) -
                                                        1 as libc::c_int) as
                                                       libc::c_uint) <<
                                                  16 as libc::c_int |
                                                  ((5 as libc::c_int *
                                                        2 as libc::c_int) as
                                                       u32_0 &
                                                       (((0x1 as libc::c_int)
                                                             <<
                                                             8 as libc::c_int)
                                                            -
                                                            1 as libc::c_int)
                                                           as libc::c_uint) <<
                                                      8 as libc::c_int) |
                                                 ((6 as libc::c_int *
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
                                                  (((5 as libc::c_int *
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
                                                           <<
                                                           8 as libc::c_int) |
                                                      ((4 as libc::c_int *
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
                                                       ((4 as libc::c_int *
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
                                              })
                                         }),
                                w1:
                                    if 0 as libc::c_int == 0 as libc::c_int {
                                        (((4 as libc::c_int *
                                               2 as libc::c_int) as u32_0 &
                                              (((0x1 as libc::c_int) <<
                                                    8 as libc::c_int) -
                                                   1 as libc::c_int) as
                                                  libc::c_uint) <<
                                             16 as libc::c_int |
                                             ((7 as libc::c_int *
                                                   2 as libc::c_int) as u32_0
                                                  &
                                                  (((0x1 as libc::c_int) <<
                                                        8 as libc::c_int) -
                                                       1 as libc::c_int) as
                                                      libc::c_uint) <<
                                                 8 as libc::c_int) |
                                            ((5 as libc::c_int *
                                                  2 as libc::c_int) as u32_0 &
                                                 (((0x1 as libc::c_int) <<
                                                       8 as libc::c_int) -
                                                      1 as libc::c_int) as
                                                     libc::c_uint) <<
                                                0 as libc::c_int
                                    } else if 0 as libc::c_int ==
                                                  1 as libc::c_int {
                                        (((7 as libc::c_int *
                                               2 as libc::c_int) as u32_0 &
                                              (((0x1 as libc::c_int) <<
                                                    8 as libc::c_int) -
                                                   1 as libc::c_int) as
                                                  libc::c_uint) <<
                                             16 as libc::c_int |
                                             ((5 as libc::c_int *
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
                                    } else {
                                        (((5 as libc::c_int *
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
                                            ((7 as libc::c_int *
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
                                             (((3 as libc::c_int *
                                                    2 as libc::c_int) as u32_0
                                                   &
                                                   (((0x1 as libc::c_int) <<
                                                         8 as libc::c_int) -
                                                        1 as libc::c_int) as
                                                       libc::c_uint) <<
                                                  16 as libc::c_int |
                                                  ((8 as libc::c_int *
                                                        2 as libc::c_int) as
                                                       u32_0 &
                                                       (((0x1 as libc::c_int)
                                                             <<
                                                             8 as libc::c_int)
                                                            -
                                                            1 as libc::c_int)
                                                           as libc::c_uint) <<
                                                      8 as libc::c_int) |
                                                 ((1 as libc::c_int *
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
                                                  (((8 as libc::c_int *
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
                                                           <<
                                                           8 as libc::c_int) |
                                                      ((3 as libc::c_int *
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
                                                       ((3 as libc::c_int *
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
                                              })
                                         }),
                                w1:
                                    if 0 as libc::c_int == 0 as libc::c_int {
                                        (((3 as libc::c_int *
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
                                            ((8 as libc::c_int *
                                                  2 as libc::c_int) as u32_0 &
                                                 (((0x1 as libc::c_int) <<
                                                       8 as libc::c_int) -
                                                      1 as libc::c_int) as
                                                     libc::c_uint) <<
                                                0 as libc::c_int
                                    } else if 0 as libc::c_int ==
                                                  1 as libc::c_int {
                                        (((9 as libc::c_int *
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
                                            ((3 as libc::c_int *
                                                  2 as libc::c_int) as u32_0 &
                                                 (((0x1 as libc::c_int) <<
                                                       8 as libc::c_int) -
                                                      1 as libc::c_int) as
                                                     libc::c_uint) <<
                                                0 as libc::c_int
                                    } else {
                                        (((8 as libc::c_int *
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
                                            ((9 as libc::c_int *
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
                                             (((10 as libc::c_int *
                                                    2 as libc::c_int) as u32_0
                                                   &
                                                   (((0x1 as libc::c_int) <<
                                                         8 as libc::c_int) -
                                                        1 as libc::c_int) as
                                                       libc::c_uint) <<
                                                  16 as libc::c_int |
                                                  ((2 as libc::c_int *
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
                                                           <<
                                                           8 as libc::c_int) |
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
                                             ((0 as libc::c_int *
                                                   2 as libc::c_int) as u32_0
                                                  &
                                                  (((0x1 as libc::c_int) <<
                                                        8 as libc::c_int) -
                                                       1 as libc::c_int) as
                                                      libc::c_uint) <<
                                                 8 as libc::c_int) |
                                            ((2 as libc::c_int *
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
                                             ((2 as libc::c_int *
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
                                        (((2 as libc::c_int *
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
                                    (0x6 as libc::c_int as u32_0 &
                                         (((0x1 as libc::c_int) <<
                                               8 as libc::c_int) -
                                              1 as libc::c_int) as
                                             libc::c_uint) <<
                                        24 as libc::c_int |
                                        (if 0 as libc::c_int ==
                                                0 as libc::c_int {
                                             (((9 as libc::c_int *
                                                    2 as libc::c_int) as u32_0
                                                   &
                                                   (((0x1 as libc::c_int) <<
                                                         8 as libc::c_int) -
                                                        1 as libc::c_int) as
                                                       libc::c_uint) <<
                                                  16 as libc::c_int |
                                                  ((12 as libc::c_int *
                                                        2 as libc::c_int) as
                                                       u32_0 &
                                                       (((0x1 as libc::c_int)
                                                             <<
                                                             8 as libc::c_int)
                                                            -
                                                            1 as libc::c_int)
                                                           as libc::c_uint) <<
                                                      8 as libc::c_int) |
                                                 ((8 as libc::c_int *
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
                                                  (((12 as libc::c_int *
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
                                                      ((9 as libc::c_int *
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
                                                  (((8 as libc::c_int *
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
                                                       ((9 as libc::c_int *
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
                                              })
                                         }),
                                w1:
                                    if 0 as libc::c_int == 0 as libc::c_int {
                                        (((9 as libc::c_int *
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
                                            ((12 as libc::c_int *
                                                  2 as libc::c_int) as u32_0 &
                                                 (((0x1 as libc::c_int) <<
                                                       8 as libc::c_int) -
                                                      1 as libc::c_int) as
                                                     libc::c_uint) <<
                                                0 as libc::c_int
                                    } else if 0 as libc::c_int ==
                                                  1 as libc::c_int {
                                        (((13 as libc::c_int *
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
                                            ((9 as libc::c_int *
                                                  2 as libc::c_int) as u32_0 &
                                                 (((0x1 as libc::c_int) <<
                                                       8 as libc::c_int) -
                                                      1 as libc::c_int) as
                                                     libc::c_uint) <<
                                                0 as libc::c_int
                                    } else {
                                        (((12 as libc::c_int *
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
                                            ((13 as libc::c_int *
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
                                             (((7 as libc::c_int *
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
                                                 ((5 as libc::c_int *
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
                                                          << 0 as libc::c_int
                                              } else {
                                                  (((5 as libc::c_int *
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
                                        (((7 as libc::c_int *
                                               2 as libc::c_int) as u32_0 &
                                              (((0x1 as libc::c_int) <<
                                                    8 as libc::c_int) -
                                                   1 as libc::c_int) as
                                                  libc::c_uint) <<
                                             16 as libc::c_int |
                                             ((15 as libc::c_int *
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
                                    } else if 0 as libc::c_int ==
                                                  1 as libc::c_int {
                                        (((15 as libc::c_int *
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
                                            ((7 as libc::c_int *
                                                  2 as libc::c_int) as u32_0 &
                                                 (((0x1 as libc::c_int) <<
                                                       8 as libc::c_int) -
                                                      1 as libc::c_int) as
                                                     libc::c_uint) <<
                                                0 as libc::c_int
                                    } else {
                                        (((14 as libc::c_int *
                                               2 as libc::c_int) as u32_0 &
                                              (((0x1 as libc::c_int) <<
                                                    8 as libc::c_int) -
                                                   1 as libc::c_int) as
                                                  libc::c_uint) <<
                                             16 as libc::c_int |
                                             ((7 as libc::c_int *
                                                   2 as libc::c_int) as u32_0
                                                  &
                                                  (((0x1 as libc::c_int) <<
                                                        8 as libc::c_int) -
                                                       1 as libc::c_int) as
                                                      libc::c_uint) <<
                                                 8 as libc::c_int) |
                                            ((15 as libc::c_int *
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
                                             (((15 as libc::c_int *
                                                    2 as libc::c_int) as u32_0
                                                   &
                                                   (((0x1 as libc::c_int) <<
                                                         8 as libc::c_int) -
                                                        1 as libc::c_int) as
                                                       libc::c_uint) <<
                                                  16 as libc::c_int |
                                                  ((11 as libc::c_int *
                                                        2 as libc::c_int) as
                                                       u32_0 &
                                                       (((0x1 as libc::c_int)
                                                             <<
                                                             8 as libc::c_int)
                                                            -
                                                            1 as libc::c_int)
                                                           as libc::c_uint) <<
                                                      8 as libc::c_int) |
                                                 ((14 as libc::c_int *
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
                                                           <<
                                                           8 as libc::c_int) |
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
                                                          << 0 as libc::c_int
                                              } else {
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
                                                          << 0 as libc::c_int
                                              })
                                         }),
                                w1:
                                    if 0 as libc::c_int == 0 as libc::c_int {
                                        (((15 as libc::c_int *
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
                                            ((11 as libc::c_int *
                                                  2 as libc::c_int) as u32_0 &
                                                 (((0x1 as libc::c_int) <<
                                                       8 as libc::c_int) -
                                                      1 as libc::c_int) as
                                                     libc::c_uint) <<
                                                0 as libc::c_int
                                    } else if 0 as libc::c_int ==
                                                  1 as libc::c_int {
                                        (((10 as libc::c_int *
                                               2 as libc::c_int) as u32_0 &
                                              (((0x1 as libc::c_int) <<
                                                    8 as libc::c_int) -
                                                   1 as libc::c_int) as
                                                  libc::c_uint) <<
                                             16 as libc::c_int |
                                             ((11 as libc::c_int *
                                                   2 as libc::c_int) as u32_0
                                                  &
                                                  (((0x1 as libc::c_int) <<
                                                        8 as libc::c_int) -
                                                       1 as libc::c_int) as
                                                      libc::c_uint) <<
                                                 8 as libc::c_int) |
                                            ((15 as libc::c_int *
                                                  2 as libc::c_int) as u32_0 &
                                                 (((0x1 as libc::c_int) <<
                                                       8 as libc::c_int) -
                                                      1 as libc::c_int) as
                                                     libc::c_uint) <<
                                                0 as libc::c_int
                                    } else {
                                        (((11 as libc::c_int *
                                               2 as libc::c_int) as u32_0 &
                                              (((0x1 as libc::c_int) <<
                                                    8 as libc::c_int) -
                                                   1 as libc::c_int) as
                                                  libc::c_uint) <<
                                             16 as libc::c_int |
                                             ((15 as libc::c_int *
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
                                             (((13 as libc::c_int *
                                                    2 as libc::c_int) as u32_0
                                                   &
                                                   (((0x1 as libc::c_int) <<
                                                         8 as libc::c_int) -
                                                        1 as libc::c_int) as
                                                       libc::c_uint) <<
                                                  16 as libc::c_int |
                                                  ((16 as libc::c_int *
                                                        2 as libc::c_int) as
                                                       u32_0 &
                                                       (((0x1 as libc::c_int)
                                                             <<
                                                             8 as libc::c_int)
                                                            -
                                                            1 as libc::c_int)
                                                           as libc::c_uint) <<
                                                      8 as libc::c_int) |
                                                 ((12 as libc::c_int *
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
                                                  (((16 as libc::c_int *
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
                                                      ((13 as libc::c_int *
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
                                                  (((12 as libc::c_int *
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
                                                       ((13 as libc::c_int *
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
                                              })
                                         }),
                                w1:
                                    if 0 as libc::c_int == 0 as libc::c_int {
                                        (((13 as libc::c_int *
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
                                            ((16 as libc::c_int *
                                                  2 as libc::c_int) as u32_0 &
                                                 (((0x1 as libc::c_int) <<
                                                       8 as libc::c_int) -
                                                      1 as libc::c_int) as
                                                     libc::c_uint) <<
                                                0 as libc::c_int
                                    } else if 0 as libc::c_int ==
                                                  1 as libc::c_int {
                                        (((17 as libc::c_int *
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
                                            ((13 as libc::c_int *
                                                  2 as libc::c_int) as u32_0 &
                                                 (((0x1 as libc::c_int) <<
                                                       8 as libc::c_int) -
                                                      1 as libc::c_int) as
                                                     libc::c_uint) <<
                                                0 as libc::c_int
                                    } else {
                                        (((16 as libc::c_int *
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
                                            ((17 as libc::c_int *
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
                 },}];
    sOuterCylinderDL =
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
                                w1: sTex.as_mut_ptr() as libc::c_uint,};
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
                                        ((0 as libc::c_int | 0 as libc::c_int)
                                             as u32_0 &
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
                                        (0 as libc::c_int as u32_0 &
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
                                                   1 as libc::c_int >>
                                                   1 as libc::c_int) -
                                                  1 as libc::c_int) <
                                                 2047 as libc::c_int {
                                              (64 as libc::c_int *
                                                   64 as libc::c_int +
                                                   1 as libc::c_int >>
                                                   1 as libc::c_int) -
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
                                                       64 as libc::c_int *
                                                           1 as libc::c_int /
                                                           8 as libc::c_int {
                                                    1 as libc::c_int
                                                } else {
                                                    (64 as libc::c_int *
                                                         1 as libc::c_int) /
                                                        8 as libc::c_int
                                                }) - 1 as libc::c_int) /
                                              (if 1 as libc::c_int >
                                                      64 as libc::c_int *
                                                          1 as libc::c_int /
                                                          8 as libc::c_int {
                                                   1 as libc::c_int
                                               } else {
                                                   (64 as libc::c_int *
                                                        1 as libc::c_int) /
                                                       8 as libc::c_int
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
                                        (1 as libc::c_int as u32_0 &
                                             (((0x1 as libc::c_int) <<
                                                   2 as libc::c_int) -
                                                  1 as libc::c_int) as
                                                 libc::c_uint) <<
                                            19 as libc::c_int |
                                        ((64 as libc::c_int * 1 as libc::c_int
                                              + 7 as libc::c_int >>
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
                                        ((0 as libc::c_int | 0 as libc::c_int)
                                             as u32_0 &
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
                                        (0 as libc::c_int as u32_0 &
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
                                w1: sTex.as_mut_ptr() as libc::c_uint,};
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
                                        ((0 as libc::c_int | 0 as libc::c_int)
                                             as u32_0 &
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
                                        (15 as libc::c_int as u32_0 &
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
                                        (15 as libc::c_int as u32_0 &
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
                                                   1 as libc::c_int >>
                                                   1 as libc::c_int) -
                                                  1 as libc::c_int) <
                                                 2047 as libc::c_int {
                                              (64 as libc::c_int *
                                                   64 as libc::c_int +
                                                   1 as libc::c_int >>
                                                   1 as libc::c_int) -
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
                                                       64 as libc::c_int *
                                                           1 as libc::c_int /
                                                           8 as libc::c_int {
                                                    1 as libc::c_int
                                                } else {
                                                    (64 as libc::c_int *
                                                         1 as libc::c_int) /
                                                        8 as libc::c_int
                                                }) - 1 as libc::c_int) /
                                              (if 1 as libc::c_int >
                                                      64 as libc::c_int *
                                                          1 as libc::c_int /
                                                          8 as libc::c_int {
                                                   1 as libc::c_int
                                               } else {
                                                   (64 as libc::c_int *
                                                        1 as libc::c_int) /
                                                       8 as libc::c_int
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
                                        (1 as libc::c_int as u32_0 &
                                             (((0x1 as libc::c_int) <<
                                                   2 as libc::c_int) -
                                                  1 as libc::c_int) as
                                                 libc::c_uint) <<
                                            19 as libc::c_int |
                                        ((64 as libc::c_int * 1 as libc::c_int
                                              + 7 as libc::c_int >>
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
                                        ((0 as libc::c_int | 0 as libc::c_int)
                                             as u32_0 &
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
                                        (15 as libc::c_int as u32_0 &
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
                                        (15 as libc::c_int as u32_0 &
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
                                              ((3 as libc::c_int as u32_0 &
                                                    (((0x1 as libc::c_int) <<
                                                          4 as libc::c_int) -
                                                         1 as libc::c_int) as
                                                        libc::c_uint) <<
                                                   5 as libc::c_int |
                                                   (0 as libc::c_int as u32_0
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
                                    (3 as libc::c_int as u32_0 &
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
                                        ((5 as libc::c_int as u32_0 &
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
                                             (4 as libc::c_int as u32_0 &
                                                  (((0x1 as libc::c_int) <<
                                                        3 as libc::c_int) -
                                                       1 as libc::c_int) as
                                                      libc::c_uint) <<
                                                 18 as libc::c_int |
                                             (5 as libc::c_int as u32_0 &
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
                                    (0xe2 as libc::c_int as u32_0 &
                                         (((0x1 as libc::c_int) <<
                                               8 as libc::c_int) -
                                              1 as libc::c_int) as
                                             libc::c_uint) <<
                                        24 as libc::c_int |
                                        ((32 as libc::c_int - 3 as libc::c_int
                                              - 29 as libc::c_int) as u32_0 &
                                             (((0x1 as libc::c_int) <<
                                                   8 as libc::c_int) -
                                                  1 as libc::c_int) as
                                                 libc::c_uint) <<
                                            8 as libc::c_int |
                                        ((29 as libc::c_int -
                                              1 as libc::c_int) as u32_0 &
                                             (((0x1 as libc::c_int) <<
                                                   8 as libc::c_int) -
                                                  1 as libc::c_int) as
                                                 libc::c_uint) <<
                                            0 as libc::c_int,
                                w1:
                                    ((0 as libc::c_int) << 30 as libc::c_int |
                                         (3 as libc::c_int) <<
                                             26 as libc::c_int |
                                         (0 as libc::c_int) <<
                                             22 as libc::c_int |
                                         (2 as libc::c_int) <<
                                             18 as libc::c_int |
                                         (0x8 as libc::c_int |
                                              0x10 as libc::c_int |
                                              0x40 as libc::c_int |
                                              0x100 as libc::c_int |
                                              0x80 as libc::c_int |
                                              0x4000 as libc::c_int |
                                              0x800 as libc::c_int |
                                              (0 as libc::c_int) <<
                                                  28 as libc::c_int |
                                              (0 as libc::c_int) <<
                                                  24 as libc::c_int |
                                              (1 as libc::c_int) <<
                                                  20 as libc::c_int |
                                              (0 as libc::c_int) <<
                                                  16 as libc::c_int)) as
                                        libc::c_uint,};
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
                                        (!((0x400 as libc::c_int |
                                                0x10000 as libc::c_int |
                                                0x20000 as libc::c_int |
                                                0x40000 as libc::c_int |
                                                0x80000 as libc::c_int) as
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
                                    (0xfa as libc::c_int as u32_0 &
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
                                            8 as libc::c_int |
                                        (0x80 as libc::c_int as u32_0 &
                                             (((0x1 as libc::c_int) <<
                                                   8 as libc::c_int) -
                                                  1 as libc::c_int) as
                                                 libc::c_uint) <<
                                            0 as libc::c_int,
                                w1:
                                    (255 as libc::c_int as u32_0 &
                                         (((0x1 as libc::c_int) <<
                                               8 as libc::c_int) -
                                              1 as libc::c_int) as
                                             libc::c_uint) <<
                                        24 as libc::c_int |
                                        (255 as libc::c_int as u32_0 &
                                             (((0x1 as libc::c_int) <<
                                                   8 as libc::c_int) -
                                                  1 as libc::c_int) as
                                                 libc::c_uint) <<
                                            16 as libc::c_int |
                                        (170 as libc::c_int as u32_0 &
                                             (((0x1 as libc::c_int) <<
                                                   8 as libc::c_int) -
                                                  1 as libc::c_int) as
                                                 libc::c_uint) <<
                                            8 as libc::c_int |
                                        (255 as libc::c_int as u32_0 &
                                             (((0x1 as libc::c_int) <<
                                                   8 as libc::c_int) -
                                                  1 as libc::c_int) as
                                                 libc::c_uint) <<
                                            0 as libc::c_int,};
                     init
                 },},
         Gfx{words:
                 {
                     let mut init =
                         Gwords{w0:
                                    (0xfb as libc::c_int as u32_0 &
                                         (((0x1 as libc::c_int) <<
                                               8 as libc::c_int) -
                                              1 as libc::c_int) as
                                             libc::c_uint) <<
                                        24 as libc::c_int,
                                w1:
                                    (0 as libc::c_int as u32_0 &
                                         (((0x1 as libc::c_int) <<
                                               8 as libc::c_int) -
                                              1 as libc::c_int) as
                                             libc::c_uint) <<
                                        24 as libc::c_int |
                                        (150 as libc::c_int as u32_0 &
                                             (((0x1 as libc::c_int) <<
                                                   8 as libc::c_int) -
                                                  1 as libc::c_int) as
                                                 libc::c_uint) <<
                                            16 as libc::c_int |
                                        (0 as libc::c_int as u32_0 &
                                             (((0x1 as libc::c_int) <<
                                                   8 as libc::c_int) -
                                                  1 as libc::c_int) as
                                                 libc::c_uint) <<
                                            8 as libc::c_int |
                                        (0 as libc::c_int as u32_0 &
                                             (((0x1 as libc::c_int) <<
                                                   8 as libc::c_int) -
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
                                    0x9000000 as libc::c_int as
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
                                        (18 as libc::c_int as u32_0 &
                                             (((0x1 as libc::c_int) <<
                                                   8 as libc::c_int) -
                                                  1 as libc::c_int) as
                                                 libc::c_uint) <<
                                            12 as libc::c_int |
                                        ((0 as libc::c_int +
                                              18 as libc::c_int) as u32_0 &
                                             (((0x1 as libc::c_int) <<
                                                   7 as libc::c_int) -
                                                  1 as libc::c_int) as
                                                 libc::c_uint) <<
                                            1 as libc::c_int,
                                w1:
                                    &mut *sCylinderVtx.as_mut_ptr().offset(18
                                                                               as
                                                                               libc::c_int
                                                                               as
                                                                               isize)
                                        as *mut Vtx as libc::c_uint,};
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
                                        (((0 as libc::c_int *
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
                                            ((1 as libc::c_int *
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
                                             ((1 as libc::c_int *
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
                                    } else {
                                        (((1 as libc::c_int *
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
                                             (((4 as libc::c_int *
                                                    2 as libc::c_int) as u32_0
                                                   &
                                                   (((0x1 as libc::c_int) <<
                                                         8 as libc::c_int) -
                                                        1 as libc::c_int) as
                                                       libc::c_uint) <<
                                                  16 as libc::c_int |
                                                  ((5 as libc::c_int *
                                                        2 as libc::c_int) as
                                                       u32_0 &
                                                       (((0x1 as libc::c_int)
                                                             <<
                                                             8 as libc::c_int)
                                                            -
                                                            1 as libc::c_int)
                                                           as libc::c_uint) <<
                                                      8 as libc::c_int) |
                                                 ((6 as libc::c_int *
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
                                                  (((5 as libc::c_int *
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
                                                           <<
                                                           8 as libc::c_int) |
                                                      ((4 as libc::c_int *
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
                                                       ((4 as libc::c_int *
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
                                              })
                                         }),
                                w1:
                                    if 0 as libc::c_int == 0 as libc::c_int {
                                        (((4 as libc::c_int *
                                               2 as libc::c_int) as u32_0 &
                                              (((0x1 as libc::c_int) <<
                                                    8 as libc::c_int) -
                                                   1 as libc::c_int) as
                                                  libc::c_uint) <<
                                             16 as libc::c_int |
                                             ((7 as libc::c_int *
                                                   2 as libc::c_int) as u32_0
                                                  &
                                                  (((0x1 as libc::c_int) <<
                                                        8 as libc::c_int) -
                                                       1 as libc::c_int) as
                                                      libc::c_uint) <<
                                                 8 as libc::c_int) |
                                            ((5 as libc::c_int *
                                                  2 as libc::c_int) as u32_0 &
                                                 (((0x1 as libc::c_int) <<
                                                       8 as libc::c_int) -
                                                      1 as libc::c_int) as
                                                     libc::c_uint) <<
                                                0 as libc::c_int
                                    } else if 0 as libc::c_int ==
                                                  1 as libc::c_int {
                                        (((7 as libc::c_int *
                                               2 as libc::c_int) as u32_0 &
                                              (((0x1 as libc::c_int) <<
                                                    8 as libc::c_int) -
                                                   1 as libc::c_int) as
                                                  libc::c_uint) <<
                                             16 as libc::c_int |
                                             ((5 as libc::c_int *
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
                                    } else {
                                        (((5 as libc::c_int *
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
                                            ((7 as libc::c_int *
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
                                             (((3 as libc::c_int *
                                                    2 as libc::c_int) as u32_0
                                                   &
                                                   (((0x1 as libc::c_int) <<
                                                         8 as libc::c_int) -
                                                        1 as libc::c_int) as
                                                       libc::c_uint) <<
                                                  16 as libc::c_int |
                                                  ((8 as libc::c_int *
                                                        2 as libc::c_int) as
                                                       u32_0 &
                                                       (((0x1 as libc::c_int)
                                                             <<
                                                             8 as libc::c_int)
                                                            -
                                                            1 as libc::c_int)
                                                           as libc::c_uint) <<
                                                      8 as libc::c_int) |
                                                 ((1 as libc::c_int *
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
                                                  (((8 as libc::c_int *
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
                                                           <<
                                                           8 as libc::c_int) |
                                                      ((3 as libc::c_int *
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
                                                       ((3 as libc::c_int *
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
                                              })
                                         }),
                                w1:
                                    if 0 as libc::c_int == 0 as libc::c_int {
                                        (((3 as libc::c_int *
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
                                            ((8 as libc::c_int *
                                                  2 as libc::c_int) as u32_0 &
                                                 (((0x1 as libc::c_int) <<
                                                       8 as libc::c_int) -
                                                      1 as libc::c_int) as
                                                     libc::c_uint) <<
                                                0 as libc::c_int
                                    } else if 0 as libc::c_int ==
                                                  1 as libc::c_int {
                                        (((9 as libc::c_int *
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
                                            ((3 as libc::c_int *
                                                  2 as libc::c_int) as u32_0 &
                                                 (((0x1 as libc::c_int) <<
                                                       8 as libc::c_int) -
                                                      1 as libc::c_int) as
                                                     libc::c_uint) <<
                                                0 as libc::c_int
                                    } else {
                                        (((8 as libc::c_int *
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
                                            ((9 as libc::c_int *
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
                                             (((10 as libc::c_int *
                                                    2 as libc::c_int) as u32_0
                                                   &
                                                   (((0x1 as libc::c_int) <<
                                                         8 as libc::c_int) -
                                                        1 as libc::c_int) as
                                                       libc::c_uint) <<
                                                  16 as libc::c_int |
                                                  ((2 as libc::c_int *
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
                                                           <<
                                                           8 as libc::c_int) |
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
                                             ((0 as libc::c_int *
                                                   2 as libc::c_int) as u32_0
                                                  &
                                                  (((0x1 as libc::c_int) <<
                                                        8 as libc::c_int) -
                                                       1 as libc::c_int) as
                                                      libc::c_uint) <<
                                                 8 as libc::c_int) |
                                            ((2 as libc::c_int *
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
                                             ((2 as libc::c_int *
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
                                        (((2 as libc::c_int *
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
                                    (0x6 as libc::c_int as u32_0 &
                                         (((0x1 as libc::c_int) <<
                                               8 as libc::c_int) -
                                              1 as libc::c_int) as
                                             libc::c_uint) <<
                                        24 as libc::c_int |
                                        (if 0 as libc::c_int ==
                                                0 as libc::c_int {
                                             (((9 as libc::c_int *
                                                    2 as libc::c_int) as u32_0
                                                   &
                                                   (((0x1 as libc::c_int) <<
                                                         8 as libc::c_int) -
                                                        1 as libc::c_int) as
                                                       libc::c_uint) <<
                                                  16 as libc::c_int |
                                                  ((12 as libc::c_int *
                                                        2 as libc::c_int) as
                                                       u32_0 &
                                                       (((0x1 as libc::c_int)
                                                             <<
                                                             8 as libc::c_int)
                                                            -
                                                            1 as libc::c_int)
                                                           as libc::c_uint) <<
                                                      8 as libc::c_int) |
                                                 ((8 as libc::c_int *
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
                                                  (((12 as libc::c_int *
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
                                                      ((9 as libc::c_int *
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
                                                  (((8 as libc::c_int *
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
                                                       ((9 as libc::c_int *
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
                                              })
                                         }),
                                w1:
                                    if 0 as libc::c_int == 0 as libc::c_int {
                                        (((9 as libc::c_int *
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
                                            ((12 as libc::c_int *
                                                  2 as libc::c_int) as u32_0 &
                                                 (((0x1 as libc::c_int) <<
                                                       8 as libc::c_int) -
                                                      1 as libc::c_int) as
                                                     libc::c_uint) <<
                                                0 as libc::c_int
                                    } else if 0 as libc::c_int ==
                                                  1 as libc::c_int {
                                        (((13 as libc::c_int *
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
                                            ((9 as libc::c_int *
                                                  2 as libc::c_int) as u32_0 &
                                                 (((0x1 as libc::c_int) <<
                                                       8 as libc::c_int) -
                                                      1 as libc::c_int) as
                                                     libc::c_uint) <<
                                                0 as libc::c_int
                                    } else {
                                        (((12 as libc::c_int *
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
                                            ((13 as libc::c_int *
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
                                             (((7 as libc::c_int *
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
                                                 ((5 as libc::c_int *
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
                                                          << 0 as libc::c_int
                                              } else {
                                                  (((5 as libc::c_int *
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
                                        (((7 as libc::c_int *
                                               2 as libc::c_int) as u32_0 &
                                              (((0x1 as libc::c_int) <<
                                                    8 as libc::c_int) -
                                                   1 as libc::c_int) as
                                                  libc::c_uint) <<
                                             16 as libc::c_int |
                                             ((15 as libc::c_int *
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
                                    } else if 0 as libc::c_int ==
                                                  1 as libc::c_int {
                                        (((15 as libc::c_int *
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
                                            ((7 as libc::c_int *
                                                  2 as libc::c_int) as u32_0 &
                                                 (((0x1 as libc::c_int) <<
                                                       8 as libc::c_int) -
                                                      1 as libc::c_int) as
                                                     libc::c_uint) <<
                                                0 as libc::c_int
                                    } else {
                                        (((14 as libc::c_int *
                                               2 as libc::c_int) as u32_0 &
                                              (((0x1 as libc::c_int) <<
                                                    8 as libc::c_int) -
                                                   1 as libc::c_int) as
                                                  libc::c_uint) <<
                                             16 as libc::c_int |
                                             ((7 as libc::c_int *
                                                   2 as libc::c_int) as u32_0
                                                  &
                                                  (((0x1 as libc::c_int) <<
                                                        8 as libc::c_int) -
                                                       1 as libc::c_int) as
                                                      libc::c_uint) <<
                                                 8 as libc::c_int) |
                                            ((15 as libc::c_int *
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
                                             (((15 as libc::c_int *
                                                    2 as libc::c_int) as u32_0
                                                   &
                                                   (((0x1 as libc::c_int) <<
                                                         8 as libc::c_int) -
                                                        1 as libc::c_int) as
                                                       libc::c_uint) <<
                                                  16 as libc::c_int |
                                                  ((11 as libc::c_int *
                                                        2 as libc::c_int) as
                                                       u32_0 &
                                                       (((0x1 as libc::c_int)
                                                             <<
                                                             8 as libc::c_int)
                                                            -
                                                            1 as libc::c_int)
                                                           as libc::c_uint) <<
                                                      8 as libc::c_int) |
                                                 ((14 as libc::c_int *
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
                                                           <<
                                                           8 as libc::c_int) |
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
                                                          << 0 as libc::c_int
                                              } else {
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
                                                          << 0 as libc::c_int
                                              })
                                         }),
                                w1:
                                    if 0 as libc::c_int == 0 as libc::c_int {
                                        (((15 as libc::c_int *
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
                                            ((11 as libc::c_int *
                                                  2 as libc::c_int) as u32_0 &
                                                 (((0x1 as libc::c_int) <<
                                                       8 as libc::c_int) -
                                                      1 as libc::c_int) as
                                                     libc::c_uint) <<
                                                0 as libc::c_int
                                    } else if 0 as libc::c_int ==
                                                  1 as libc::c_int {
                                        (((10 as libc::c_int *
                                               2 as libc::c_int) as u32_0 &
                                              (((0x1 as libc::c_int) <<
                                                    8 as libc::c_int) -
                                                   1 as libc::c_int) as
                                                  libc::c_uint) <<
                                             16 as libc::c_int |
                                             ((11 as libc::c_int *
                                                   2 as libc::c_int) as u32_0
                                                  &
                                                  (((0x1 as libc::c_int) <<
                                                        8 as libc::c_int) -
                                                       1 as libc::c_int) as
                                                      libc::c_uint) <<
                                                 8 as libc::c_int) |
                                            ((15 as libc::c_int *
                                                  2 as libc::c_int) as u32_0 &
                                                 (((0x1 as libc::c_int) <<
                                                       8 as libc::c_int) -
                                                      1 as libc::c_int) as
                                                     libc::c_uint) <<
                                                0 as libc::c_int
                                    } else {
                                        (((11 as libc::c_int *
                                               2 as libc::c_int) as u32_0 &
                                              (((0x1 as libc::c_int) <<
                                                    8 as libc::c_int) -
                                                   1 as libc::c_int) as
                                                  libc::c_uint) <<
                                             16 as libc::c_int |
                                             ((15 as libc::c_int *
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
                                             (((13 as libc::c_int *
                                                    2 as libc::c_int) as u32_0
                                                   &
                                                   (((0x1 as libc::c_int) <<
                                                         8 as libc::c_int) -
                                                        1 as libc::c_int) as
                                                       libc::c_uint) <<
                                                  16 as libc::c_int |
                                                  ((16 as libc::c_int *
                                                        2 as libc::c_int) as
                                                       u32_0 &
                                                       (((0x1 as libc::c_int)
                                                             <<
                                                             8 as libc::c_int)
                                                            -
                                                            1 as libc::c_int)
                                                           as libc::c_uint) <<
                                                      8 as libc::c_int) |
                                                 ((12 as libc::c_int *
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
                                                  (((16 as libc::c_int *
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
                                                      ((13 as libc::c_int *
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
                                                  (((12 as libc::c_int *
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
                                                       ((13 as libc::c_int *
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
                                              })
                                         }),
                                w1:
                                    if 0 as libc::c_int == 0 as libc::c_int {
                                        (((13 as libc::c_int *
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
                                            ((16 as libc::c_int *
                                                  2 as libc::c_int) as u32_0 &
                                                 (((0x1 as libc::c_int) <<
                                                       8 as libc::c_int) -
                                                      1 as libc::c_int) as
                                                     libc::c_uint) <<
                                                0 as libc::c_int
                                    } else if 0 as libc::c_int ==
                                                  1 as libc::c_int {
                                        (((17 as libc::c_int *
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
                                            ((13 as libc::c_int *
                                                  2 as libc::c_int) as u32_0 &
                                                 (((0x1 as libc::c_int) <<
                                                       8 as libc::c_int) -
                                                      1 as libc::c_int) as
                                                     libc::c_uint) <<
                                                0 as libc::c_int
                                    } else {
                                        (((16 as libc::c_int *
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
                                            ((17 as libc::c_int *
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
