#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case,
         non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(const_transmute, register_tool)]
extern "C" {
    #[no_mangle]
    fn Actor_Kill(actor: *mut Actor);
    #[no_mangle]
    fn Actor_SetScale(actor: *mut Actor, scale: f32_0);
    #[no_mangle]
    fn func_8002F7DC(actor: *mut Actor, sfxId: u16_0);
    #[no_mangle]
    fn Collider_InitCylinder(globalCtx: *mut GlobalContext,
                             collider: *mut ColliderCylinder) -> s32;
    #[no_mangle]
    fn Collider_SetCylinder(globalCtx: *mut GlobalContext,
                            collider: *mut ColliderCylinder,
                            actor: *mut Actor, src: *mut ColliderCylinderInit)
     -> s32;
    #[no_mangle]
    fn CollisionCheck_SetAT(globalCtx: *mut GlobalContext,
                            colChkCtx: *mut CollisionCheckContext,
                            collider: *mut Collider) -> s32;
    #[no_mangle]
    fn Collider_UpdateCylinder(actor: *mut Actor,
                               collider: *mut ColliderCylinder);
    #[no_mangle]
    fn Math_StepToF(pValue: *mut f32_0, target: f32_0, step: f32_0) -> s32;
    #[no_mangle]
    fn Actor_ProcessInitChain(actor: *mut Actor,
                              initChain: *mut InitChainEntry);
    #[no_mangle]
    fn Math_SmoothStepToF(pValue: *mut f32_0, target: f32_0, fraction: f32_0,
                          step: f32_0, minStep: f32_0) -> f32_0;
    #[no_mangle]
    fn func_800876C8(globalCtx: *mut GlobalContext);
    #[no_mangle]
    fn func_800937C0(gfx: *mut Gfx) -> *mut Gfx;
    #[no_mangle]
    fn func_80093D84(gfxCtx: *mut GraphicsContext);
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
    #[no_mangle]
    fn Matrix_Scale(x: f32_0, y: f32_0, z: f32_0, mode: u8_0);
    #[no_mangle]
    fn Matrix_NewMtx(gfxCtx: *mut GraphicsContext, file: *mut libc::c_char,
                     line: s32) -> *mut Mtx;
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
pub struct ColliderInit {
    pub colType: u8_0,
    pub atFlags: u8_0,
    pub acFlags: u8_0,
    pub ocFlags1: u8_0,
    pub ocFlags2: u8_0,
    pub shape: u8_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ColliderBumpInit {
    pub dmgFlags: u32_0,
    pub effect: u8_0,
    pub defense: u8_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ColliderInfoInit {
    pub elemType: u8_0,
    pub toucher: ColliderTouch,
    pub bumper: ColliderBumpInit,
    pub toucherFlags: u8_0,
    pub bumperFlags: u8_0,
    pub ocElemFlags: u8_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ColliderCylinderInit {
    pub base: ColliderInit,
    pub info: ColliderInfoInit,
    pub dim: Cylinder16,
}
pub type C2RustUnnamed_14 = libc::c_uint;
pub const COLTYPE_TREE: C2RustUnnamed_14 = 13;
pub const COLTYPE_HARD: C2RustUnnamed_14 = 12;
pub const COLTYPE_WOOD: C2RustUnnamed_14 = 11;
pub const COLTYPE_NONE: C2RustUnnamed_14 = 10;
pub const COLTYPE_METAL: C2RustUnnamed_14 = 9;
pub const COLTYPE_HIT8: C2RustUnnamed_14 = 8;
pub const COLTYPE_HIT7: C2RustUnnamed_14 = 7;
pub const COLTYPE_HIT6: C2RustUnnamed_14 = 6;
pub const COLTYPE_HIT5: C2RustUnnamed_14 = 5;
pub const COLTYPE_HIT4: C2RustUnnamed_14 = 4;
pub const COLTYPE_HIT3: C2RustUnnamed_14 = 3;
pub const COLTYPE_HIT2: C2RustUnnamed_14 = 2;
pub const COLTYPE_HIT1: C2RustUnnamed_14 = 1;
pub const COLTYPE_HIT0: C2RustUnnamed_14 = 0;
pub type C2RustUnnamed_15 = libc::c_uint;
pub const COLSHAPE_INVALID: C2RustUnnamed_15 = 4;
pub const COLSHAPE_QUAD: C2RustUnnamed_15 = 3;
pub const COLSHAPE_TRIS: C2RustUnnamed_15 = 2;
pub const COLSHAPE_CYLINDER: C2RustUnnamed_15 = 1;
pub const COLSHAPE_JNTSPH: C2RustUnnamed_15 = 0;
pub type C2RustUnnamed_16 = libc::c_uint;
pub const ELEMTYPE_UNK7: C2RustUnnamed_16 = 7;
pub const ELEMTYPE_UNK6: C2RustUnnamed_16 = 6;
pub const ELEMTYPE_UNK5: C2RustUnnamed_16 = 5;
pub const ELEMTYPE_UNK4: C2RustUnnamed_16 = 4;
pub const ELEMTYPE_UNK3: C2RustUnnamed_16 = 3;
pub const ELEMTYPE_UNK2: C2RustUnnamed_16 = 2;
pub const ELEMTYPE_UNK1: C2RustUnnamed_16 = 1;
pub const ELEMTYPE_UNK0: C2RustUnnamed_16 = 0;
pub type C2RustUnnamed_17 = libc::c_uint;
pub const ACTORCAT_CHEST: C2RustUnnamed_17 = 11;
pub const ACTORCAT_DOOR: C2RustUnnamed_17 = 10;
pub const ACTORCAT_BOSS: C2RustUnnamed_17 = 9;
pub const ACTORCAT_MISC: C2RustUnnamed_17 = 8;
pub const ACTORCAT_ITEMACTION: C2RustUnnamed_17 = 7;
pub const ACTORCAT_PROP: C2RustUnnamed_17 = 6;
pub const ACTORCAT_ENEMY: C2RustUnnamed_17 = 5;
pub const ACTORCAT_NPC: C2RustUnnamed_17 = 4;
pub const ACTORCAT_EXPLOSIVE: C2RustUnnamed_17 = 3;
pub const ACTORCAT_PLAYER: C2RustUnnamed_17 = 2;
pub const ACTORCAT_BG: C2RustUnnamed_17 = 1;
pub const ACTORCAT_SWITCH: C2RustUnnamed_17 = 0;
pub type C2RustUnnamed_18 = libc::c_uint;
pub const ACTOR_ID_MAX: C2RustUnnamed_18 = 471;
pub const ACTOR_OBJ_WARP2BLOCK: C2RustUnnamed_18 = 470;
pub const ACTOR_BG_JYA_BLOCK: C2RustUnnamed_18 = 469;
pub const ACTOR_EN_MM2: C2RustUnnamed_18 = 468;
pub const ACTOR_EN_ZL4: C2RustUnnamed_18 = 467;
pub const ACTOR_OBJ_HAMISHI: C2RustUnnamed_18 = 466;
pub const ACTOR_OBJ_TIMEBLOCK: C2RustUnnamed_18 = 465;
pub const ACTOR_EN_GE3: C2RustUnnamed_18 = 464;
pub const ACTOR_OBJ_MAKEKINSUTA: C2RustUnnamed_18 = 463;
pub const ACTOR_EN_ZO: C2RustUnnamed_18 = 462;
pub const ACTOR_BG_MENKURI_NISEKABE: C2RustUnnamed_18 = 461;
pub const ACTOR_EN_EG: C2RustUnnamed_18 = 460;
pub const ACTOR_OCEFF_WIPE4: C2RustUnnamed_18 = 459;
pub const ACTOR_EN_KAKASI3: C2RustUnnamed_18 = 458;
pub const ACTOR_EN_KAKASI2: C2RustUnnamed_18 = 457;
pub const ACTOR_BG_ICE_SHUTTER: C2RustUnnamed_18 = 456;
pub const ACTOR_BG_ICE_TURARA: C2RustUnnamed_18 = 455;
pub const ACTOR_EN_COW: C2RustUnnamed_18 = 454;
pub const ACTOR_EN_MA3: C2RustUnnamed_18 = 453;
pub const ACTOR_BG_SPOT18_SHUTTER: C2RustUnnamed_18 = 452;
pub const ACTOR_BG_SPOT18_FUTA: C2RustUnnamed_18 = 451;
pub const ACTOR_BG_SPOT11_OASIS: C2RustUnnamed_18 = 450;
pub const ACTOR_DOOR_KILLER: C2RustUnnamed_18 = 449;
pub const ACTOR_EN_CROW: C2RustUnnamed_18 = 448;
pub const ACTOR_EN_PO_DESERT: C2RustUnnamed_18 = 447;
pub const ACTOR_EN_WALL_TUBO: C2RustUnnamed_18 = 446;
pub const ACTOR_BG_BOWL_WALL: C2RustUnnamed_18 = 445;
pub const ACTOR_EN_DAIKU_KAKARIKO: C2RustUnnamed_18 = 444;
pub const ACTOR_BG_MIZU_SHUTTER: C2RustUnnamed_18 = 443;
pub const ACTOR_BG_MIZU_BWALL: C2RustUnnamed_18 = 442;
pub const ACTOR_EN_GS: C2RustUnnamed_18 = 441;
pub const ACTOR_EN_GB: C2RustUnnamed_18 = 440;
pub const ACTOR_BG_GND_ICEBLOCK: C2RustUnnamed_18 = 439;
pub const ACTOR_BG_GND_NISEKABE: C2RustUnnamed_18 = 438;
pub const ACTOR_BG_GND_SOULMEIRO: C2RustUnnamed_18 = 437;
pub const ACTOR_BG_GND_DARKMEIRO: C2RustUnnamed_18 = 436;
pub const ACTOR_BG_GND_FIREMEIRO: C2RustUnnamed_18 = 435;
pub const ACTOR_DEMO_GEFF: C2RustUnnamed_18 = 434;
pub const ACTOR_DEMO_GJ: C2RustUnnamed_18 = 433;
pub const ACTOR_EN_SKB: C2RustUnnamed_18 = 432;
pub const ACTOR_EN_WF: C2RustUnnamed_18 = 431;
pub const ACTOR_EN_GO2: C2RustUnnamed_18 = 430;
pub const ACTOR_EN_MU: C2RustUnnamed_18 = 429;
pub const ACTOR_EN_TG: C2RustUnnamed_18 = 428;
pub const ACTOR_OBJ_MURE3: C2RustUnnamed_18 = 427;
pub const ACTOR_UNSET_1AA: C2RustUnnamed_18 = 426;
pub const ACTOR_BG_SPOT17_BAKUDANKABE: C2RustUnnamed_18 = 425;
pub const ACTOR_BG_SPOT08_BAKUDANKABE: C2RustUnnamed_18 = 424;
pub const ACTOR_DEMO_KEKKAI: C2RustUnnamed_18 = 423;
pub const ACTOR_EN_HS2: C2RustUnnamed_18 = 422;
pub const ACTOR_BG_BOM_GUARD: C2RustUnnamed_18 = 421;
pub const ACTOR_EN_GUEST: C2RustUnnamed_18 = 420;
pub const ACTOR_EN_DNT_NOMAL: C2RustUnnamed_18 = 419;
pub const ACTOR_EN_DNT_JIJI: C2RustUnnamed_18 = 418;
pub const ACTOR_EN_DNT_DEMO: C2RustUnnamed_18 = 417;
pub const ACTOR_OBJ_KIBAKO2: C2RustUnnamed_18 = 416;
pub const ACTOR_BG_SPOT11_BAKUDANKABE: C2RustUnnamed_18 = 415;
pub const ACTOR_OBJ_COMB: C2RustUnnamed_18 = 414;
pub const ACTOR_BG_SPOT01_OBJECTS2: C2RustUnnamed_18 = 413;
pub const ACTOR_EN_SI: C2RustUnnamed_18 = 412;
pub const ACTOR_EN_DOG: C2RustUnnamed_18 = 411;
pub const ACTOR_EN_NIW_GIRL: C2RustUnnamed_18 = 410;
pub const ACTOR_OCEFF_WIPE3: C2RustUnnamed_18 = 409;
pub const ACTOR_OCEFF_WIPE2: C2RustUnnamed_18 = 408;
pub const ACTOR_EN_GELDB: C2RustUnnamed_18 = 407;
pub const ACTOR_EN_IT: C2RustUnnamed_18 = 406;
pub const ACTOR_EN_SHOPNUTS: C2RustUnnamed_18 = 405;
pub const ACTOR_BG_SPOT00_BREAK: C2RustUnnamed_18 = 404;
pub const ACTOR_EN_NUTSBALL: C2RustUnnamed_18 = 403;
pub const ACTOR_EN_HINTNUTS: C2RustUnnamed_18 = 402;
pub const ACTOR_BG_SPOT12_SAKU: C2RustUnnamed_18 = 401;
pub const ACTOR_BG_SPOT12_GATE: C2RustUnnamed_18 = 400;
pub const ACTOR_BG_JYA_HAHENIRON: C2RustUnnamed_18 = 399;
pub const ACTOR_BG_JYA_1FLIFT: C2RustUnnamed_18 = 398;
pub const ACTOR_BG_SPOT05_SOKO: C2RustUnnamed_18 = 397;
pub const ACTOR_EN_WEIYER: C2RustUnnamed_18 = 396;
pub const ACTOR_OCEFF_STORM: C2RustUnnamed_18 = 395;
pub const ACTOR_OCEFF_WIPE: C2RustUnnamed_18 = 394;
pub const ACTOR_EN_STH: C2RustUnnamed_18 = 393;
pub const ACTOR_EN_SSH: C2RustUnnamed_18 = 392;
pub const ACTOR_OBJ_ROOMTIMER: C2RustUnnamed_18 = 391;
pub const ACTOR_EN_GE2: C2RustUnnamed_18 = 390;
pub const ACTOR_EN_WONDER_TALK2: C2RustUnnamed_18 = 389;
pub const ACTOR_EN_DY_EXTRA: C2RustUnnamed_18 = 388;
pub const ACTOR_SHOT_SUN: C2RustUnnamed_18 = 387;
pub const ACTOR_DEMO_EC: C2RustUnnamed_18 = 386;
pub const ACTOR_EN_TORCH: C2RustUnnamed_18 = 385;
pub const ACTOR_UNSET_180: C2RustUnnamed_18 = 384;
pub const ACTOR_END_TITLE: C2RustUnnamed_18 = 383;
pub const ACTOR_OCEFF_SPOT: C2RustUnnamed_18 = 382;
pub const ACTOR_OBJ_MAKEOSHIHIKI: C2RustUnnamed_18 = 381;
pub const ACTOR_EN_TAKARA_MAN: C2RustUnnamed_18 = 380;
pub const ACTOR_EN_KAKASI: C2RustUnnamed_18 = 379;
pub const ACTOR_BOSS_GANON2: C2RustUnnamed_18 = 378;
pub const ACTOR_EN_ZL3: C2RustUnnamed_18 = 377;
pub const ACTOR_EN_HEISHI4: C2RustUnnamed_18 = 376;
pub const ACTOR_BG_ZG: C2RustUnnamed_18 = 375;
pub const ACTOR_EFC_ERUPC: C2RustUnnamed_18 = 374;
pub const ACTOR_EN_PO_FIELD: C2RustUnnamed_18 = 373;
pub const ACTOR_DEMO_GT: C2RustUnnamed_18 = 372;
pub const ACTOR_ELF_MSG2: C2RustUnnamed_18 = 371;
pub const ACTOR_DOOR_GERUDO: C2RustUnnamed_18 = 370;
pub const ACTOR_EN_MAG: C2RustUnnamed_18 = 369;
pub const ACTOR_EN_OKARINA_EFFECT: C2RustUnnamed_18 = 368;
pub const ACTOR_EN_GANON_MANT: C2RustUnnamed_18 = 367;
pub const ACTOR_EN_HY: C2RustUnnamed_18 = 366;
pub const ACTOR_EN_MD: C2RustUnnamed_18 = 365;
pub const ACTOR_EN_CS: C2RustUnnamed_18 = 364;
pub const ACTOR_EN_JSJUTAN: C2RustUnnamed_18 = 363;
pub const ACTOR_EN_JS: C2RustUnnamed_18 = 362;
pub const ACTOR_BG_JYA_IRONOBJ: C2RustUnnamed_18 = 361;
pub const ACTOR_EN_EX_ITEM: C2RustUnnamed_18 = 360;
pub const ACTOR_EN_ANI: C2RustUnnamed_18 = 359;
pub const ACTOR_BG_SST_FLOOR: C2RustUnnamed_18 = 358;
pub const ACTOR_EN_WEATHER_TAG: C2RustUnnamed_18 = 357;
pub const ACTOR_EN_KZ: C2RustUnnamed_18 = 356;
pub const ACTOR_EN_KO: C2RustUnnamed_18 = 355;
pub const ACTOR_EN_MM: C2RustUnnamed_18 = 354;
pub const ACTOR_UNSET_161: C2RustUnnamed_18 = 353;
pub const ACTOR_EN_STREAM: C2RustUnnamed_18 = 352;
pub const ACTOR_EN_SIOFUKI: C2RustUnnamed_18 = 351;
pub const ACTOR_EN_GANON_ORGAN: C2RustUnnamed_18 = 350;
pub const ACTOR_UNSET_15D: C2RustUnnamed_18 = 349;
pub const ACTOR_BG_SPOT18_BASKET: C2RustUnnamed_18 = 348;
pub const ACTOR_BG_JYA_BOMBIWA: C2RustUnnamed_18 = 347;
pub const ACTOR_BG_JYA_AMISHUTTER: C2RustUnnamed_18 = 346;
pub const ACTOR_BG_JYA_BOMBCHUIWA: C2RustUnnamed_18 = 345;
pub const ACTOR_BG_JYA_BIGMIRROR: C2RustUnnamed_18 = 344;
pub const ACTOR_BG_JYA_LIFT: C2RustUnnamed_18 = 343;
pub const ACTOR_BG_JYA_MEGAMI: C2RustUnnamed_18 = 342;
pub const ACTOR_EN_CHANGER: C2RustUnnamed_18 = 341;
pub const ACTOR_UNSET_154: C2RustUnnamed_18 = 340;
pub const ACTOR_EN_FU: C2RustUnnamed_18 = 339;
pub const ACTOR_EN_GO: C2RustUnnamed_18 = 338;
pub const ACTOR_OBJ_MURE2: C2RustUnnamed_18 = 337;
pub const ACTOR_OBJ_LIGHTSWITCH: C2RustUnnamed_18 = 336;
pub const ACTOR_OBJ_HANA: C2RustUnnamed_18 = 335;
pub const ACTOR_EN_ISHI: C2RustUnnamed_18 = 334;
pub const ACTOR_EN_OWL: C2RustUnnamed_18 = 333;
pub const ACTOR_EN_BOM_BOWL_PIT: C2RustUnnamed_18 = 332;
pub const ACTOR_EN_BOM_BOWL_MAN: C2RustUnnamed_18 = 331;
pub const ACTOR_EN_MK: C2RustUnnamed_18 = 330;
pub const ACTOR_EN_DS: C2RustUnnamed_18 = 329;
pub const ACTOR_BG_GJYO_BRIDGE: C2RustUnnamed_18 = 328;
pub const ACTOR_EN_WONDER_TALK: C2RustUnnamed_18 = 327;
pub const ACTOR_EN_SA: C2RustUnnamed_18 = 326;
pub const ACTOR_BG_SPOT01_IDOSOKO: C2RustUnnamed_18 = 325;
pub const ACTOR_EN_ATTACK_NIW: C2RustUnnamed_18 = 324;
pub const ACTOR_EN_SYATEKI_NIW: C2RustUnnamed_18 = 323;
pub const ACTOR_EN_HEISHI3: C2RustUnnamed_18 = 322;
pub const ACTOR_EN_KANBAN: C2RustUnnamed_18 = 321;
pub const ACTOR_BG_INGATE: C2RustUnnamed_18 = 320;
pub const ACTOR_EN_HS: C2RustUnnamed_18 = 319;
pub const ACTOR_EN_MS: C2RustUnnamed_18 = 318;
pub const ACTOR_EN_GM: C2RustUnnamed_18 = 317;
pub const ACTOR_EN_NIW_LADY: C2RustUnnamed_18 = 316;
pub const ACTOR_EN_CLEAR_TAG: C2RustUnnamed_18 = 315;
pub const ACTOR_EN_SDA: C2RustUnnamed_18 = 314;
pub const ACTOR_OBJ_BLOCKSTOP: C2RustUnnamed_18 = 313;
pub const ACTOR_EN_GE1: C2RustUnnamed_18 = 312;
pub const ACTOR_ITEM_INBOX: C2RustUnnamed_18 = 311;
pub const ACTOR_EN_BLKOBJ: C2RustUnnamed_18 = 310;
pub const ACTOR_EN_NWC: C2RustUnnamed_18 = 309;
pub const ACTOR_UNSET_134: C2RustUnnamed_18 = 308;
pub const ACTOR_EN_DAIKU: C2RustUnnamed_18 = 307;
pub const ACTOR_EN_TORYO: C2RustUnnamed_18 = 306;
pub const ACTOR_EN_EX_RUPPY: C2RustUnnamed_18 = 305;
pub const ACTOR_EN_GOROIWA: C2RustUnnamed_18 = 304;
pub const ACTOR_EN_YABUSAME_MARK: C2RustUnnamed_18 = 303;
pub const ACTOR_EN_OKARINA_TAG: C2RustUnnamed_18 = 302;
pub const ACTOR_OBJ_HSBLOCK: C2RustUnnamed_18 = 301;
pub const ACTOR_OBJ_LIFT: C2RustUnnamed_18 = 300;
pub const ACTOR_OBJ_ELEVATOR: C2RustUnnamed_18 = 299;
pub const ACTOR_OBJ_SWITCH: C2RustUnnamed_18 = 298;
pub const ACTOR_UNSET_129: C2RustUnnamed_18 = 297;
pub const ACTOR_UNSET_128: C2RustUnnamed_18 = 296;
pub const ACTOR_OBJ_BOMBIWA: C2RustUnnamed_18 = 295;
pub const ACTOR_OBJ_BEAN: C2RustUnnamed_18 = 294;
pub const ACTOR_EN_KUSA: C2RustUnnamed_18 = 293;
pub const ACTOR_EN_DIVING_GAME: C2RustUnnamed_18 = 292;
pub const ACTOR_BG_RELAY_OBJECTS: C2RustUnnamed_18 = 291;
pub const ACTOR_EN_PO_RELAY: C2RustUnnamed_18 = 290;
pub const ACTOR_EN_FZ: C2RustUnnamed_18 = 289;
pub const ACTOR_BG_SPOT07_TAKI: C2RustUnnamed_18 = 288;
pub const ACTOR_BG_SPOT03_TAKI: C2RustUnnamed_18 = 287;
pub const ACTOR_OBJ_ICE_POLY: C2RustUnnamed_18 = 286;
pub const ACTOR_EN_TUBO_TRAP: C2RustUnnamed_18 = 285;
pub const ACTOR_EN_HONOTRAP: C2RustUnnamed_18 = 284;
pub const ACTOR_ELF_MSG: C2RustUnnamed_18 = 283;
pub const ACTOR_EN_DNS: C2RustUnnamed_18 = 282;
pub const ACTOR_DEMO_SHD: C2RustUnnamed_18 = 281;
pub const ACTOR_DEMO_EXT: C2RustUnnamed_18 = 280;
pub const ACTOR_EN_G_SWITCH: C2RustUnnamed_18 = 279;
pub const ACTOR_EN_SKJNEEDLE: C2RustUnnamed_18 = 278;
pub const ACTOR_EN_SKJ: C2RustUnnamed_18 = 277;
pub const ACTOR_DEMO_IK: C2RustUnnamed_18 = 276;
pub const ACTOR_EN_IK: C2RustUnnamed_18 = 275;
pub const ACTOR_EN_WONDER_ITEM: C2RustUnnamed_18 = 274;
pub const ACTOR_OBJ_TSUBO: C2RustUnnamed_18 = 273;
pub const ACTOR_OBJ_KIBAKO: C2RustUnnamed_18 = 272;
pub const ACTOR_ITEM_ETCETERA: C2RustUnnamed_18 = 271;
pub const ACTOR_UNSET_10E: C2RustUnnamed_18 = 270;
pub const ACTOR_UNSET_10D: C2RustUnnamed_18 = 269;
pub const ACTOR_ARROW_LIGHT: C2RustUnnamed_18 = 268;
pub const ACTOR_ARROW_ICE: C2RustUnnamed_18 = 267;
pub const ACTOR_ARROW_FIRE: C2RustUnnamed_18 = 266;
pub const ACTOR_UNSET_109: C2RustUnnamed_18 = 265;
pub const ACTOR_BG_UMAJUMP: C2RustUnnamed_18 = 264;
pub const ACTOR_BG_SPOT15_RRBOX: C2RustUnnamed_18 = 263;
pub const ACTOR_BG_GANON_OTYUKA: C2RustUnnamed_18 = 262;
pub const ACTOR_BG_PO_SYOKUDAI: C2RustUnnamed_18 = 261;
pub const ACTOR_BG_SPOT01_IDOMIZU: C2RustUnnamed_18 = 260;
pub const ACTOR_BG_SPOT01_IDOHASHIRA: C2RustUnnamed_18 = 259;
pub const ACTOR_BG_SPOT01_FUSYA: C2RustUnnamed_18 = 258;
pub const ACTOR_EFF_DUST: C2RustUnnamed_18 = 257;
pub const ACTOR_BG_GATE_SHUTTER: C2RustUnnamed_18 = 256;
pub const ACTOR_OBJ_OSHIHIKI: C2RustUnnamed_18 = 255;
pub const ACTOR_FISHING: C2RustUnnamed_18 = 254;
pub const ACTOR_BG_JYA_KANAAMI: C2RustUnnamed_18 = 253;
pub const ACTOR_BG_JYA_COBRA: C2RustUnnamed_18 = 252;
pub const ACTOR_UNSET_FB: C2RustUnnamed_18 = 251;
pub const ACTOR_BG_JYA_ZURERUKABE: C2RustUnnamed_18 = 250;
pub const ACTOR_BG_JYA_GOROIWA: C2RustUnnamed_18 = 249;
pub const ACTOR_BG_SPOT15_SAKU: C2RustUnnamed_18 = 248;
pub const ACTOR_BG_HAKA_GATE: C2RustUnnamed_18 = 247;
pub const ACTOR_EN_ANUBICE_TAG: C2RustUnnamed_18 = 246;
pub const ACTOR_DEMO_6K: C2RustUnnamed_18 = 245;
pub const ACTOR_MAGIC_DARK: C2RustUnnamed_18 = 244;
pub const ACTOR_UNSET_F3: C2RustUnnamed_18 = 243;
pub const ACTOR_UNSET_F2: C2RustUnnamed_18 = 242;
pub const ACTOR_ITEM_OCARINA: C2RustUnnamed_18 = 241;
pub const ACTOR_EN_ICE_HONO: C2RustUnnamed_18 = 240;
pub const ACTOR_BG_ICE_SHELTER: C2RustUnnamed_18 = 239;
pub const ACTOR_ITEM_SHIELD: C2RustUnnamed_18 = 238;
pub const ACTOR_EN_FR: C2RustUnnamed_18 = 237;
pub const ACTOR_EN_NY: C2RustUnnamed_18 = 236;
pub const ACTOR_UNSET_EB: C2RustUnnamed_18 = 235;
pub const ACTOR_UNSET_EA: C2RustUnnamed_18 = 234;
pub const ACTOR_BOSS_SST: C2RustUnnamed_18 = 233;
pub const ACTOR_BOSS_GANON: C2RustUnnamed_18 = 232;
pub const ACTOR_EN_MA1: C2RustUnnamed_18 = 231;
pub const ACTOR_BG_BDAN_SWITCH: C2RustUnnamed_18 = 230;
pub const ACTOR_BG_SPOT16_DOUGHNUT: C2RustUnnamed_18 = 229;
pub const ACTOR_BG_MORI_IDOMIZU: C2RustUnnamed_18 = 228;
pub const ACTOR_BG_MORI_HASHIRA4: C2RustUnnamed_18 = 227;
pub const ACTOR_BG_MORI_HASHIGO: C2RustUnnamed_18 = 226;
pub const ACTOR_EN_ANUBICE_FIRE: C2RustUnnamed_18 = 225;
pub const ACTOR_EN_ANUBICE: C2RustUnnamed_18 = 224;
pub const ACTOR_EN_BX: C2RustUnnamed_18 = 223;
pub const ACTOR_EN_BA: C2RustUnnamed_18 = 222;
pub const ACTOR_EN_RR: C2RustUnnamed_18 = 221;
pub const ACTOR_BOSS_TW: C2RustUnnamed_18 = 220;
pub const ACTOR_EN_HORSE_GAME_CHECK: C2RustUnnamed_18 = 219;
pub const ACTOR_EN_BOM_CHU: C2RustUnnamed_18 = 218;
pub const ACTOR_EN_MA2: C2RustUnnamed_18 = 217;
pub const ACTOR_UNSET_D8: C2RustUnnamed_18 = 216;
pub const ACTOR_BG_HAKA_WATER: C2RustUnnamed_18 = 215;
pub const ACTOR_BG_ICE_OBJECTS: C2RustUnnamed_18 = 214;
pub const ACTOR_BG_SPOT06_OBJECTS: C2RustUnnamed_18 = 213;
pub const ACTOR_BG_MIZU_UZU: C2RustUnnamed_18 = 212;
pub const ACTOR_OBJ_DEKUJR: C2RustUnnamed_18 = 211;
pub const ACTOR_EN_RU2: C2RustUnnamed_18 = 210;
pub const ACTOR_BG_SPOT08_ICEBLOCK: C2RustUnnamed_18 = 209;
pub const ACTOR_BG_BOMBWALL: C2RustUnnamed_18 = 208;
pub const ACTOR_BG_HIDAN_KOWARERUKABE: C2RustUnnamed_18 = 207;
pub const ACTOR_UNSET_CE: C2RustUnnamed_18 = 206;
pub const ACTOR_BG_SPOT16_BOMBSTONE: C2RustUnnamed_18 = 205;
pub const ACTOR_EN_TR: C2RustUnnamed_18 = 204;
pub const ACTOR_EN_IN: C2RustUnnamed_18 = 203;
pub const ACTOR_DEMO_GO: C2RustUnnamed_18 = 202;
pub const ACTOR_DEMO_SA: C2RustUnnamed_18 = 201;
pub const ACTOR_BG_BDAN_OBJECTS: C2RustUnnamed_18 = 200;
pub const ACTOR_EN_KAREBABA: C2RustUnnamed_18 = 199;
pub const ACTOR_EN_BIGOKUTA: C2RustUnnamed_18 = 198;
pub const ACTOR_EN_SB: C2RustUnnamed_18 = 197;
pub const ACTOR_BOSS_MO: C2RustUnnamed_18 = 196;
pub const ACTOR_EN_NB: C2RustUnnamed_18 = 195;
pub const ACTOR_EN_TANA: C2RustUnnamed_18 = 194;
pub const ACTOR_EN_SYATEKI_MAN: C2RustUnnamed_18 = 193;
pub const ACTOR_EN_SYATEKI_ITM: C2RustUnnamed_18 = 192;
pub const ACTOR_BG_SPOT17_FUNEN: C2RustUnnamed_18 = 191;
pub const ACTOR_BG_HAKA_ZOU: C2RustUnnamed_18 = 190;
pub const ACTOR_BG_HAKA_HUTA: C2RustUnnamed_18 = 189;
pub const ACTOR_BG_HAKA_TRAP: C2RustUnnamed_18 = 188;
pub const ACTOR_BG_HAKA_TUBO: C2RustUnnamed_18 = 187;
pub const ACTOR_BOSS_VA: C2RustUnnamed_18 = 186;
pub const ACTOR_BG_SPOT18_OBJ: C2RustUnnamed_18 = 185;
pub const ACTOR_BG_SPOT09_OBJ: C2RustUnnamed_18 = 184;
pub const ACTOR_MIR_RAY: C2RustUnnamed_18 = 183;
pub const ACTOR_EN_BROB: C2RustUnnamed_18 = 182;
pub const ACTOR_EN_FIRE_ROCK: C2RustUnnamed_18 = 181;
pub const ACTOR_EN_ENCOUNT2: C2RustUnnamed_18 = 180;
pub const ACTOR_EN_HEISHI2: C2RustUnnamed_18 = 179;
pub const ACTOR_UNSET_B2: C2RustUnnamed_18 = 178;
pub const ACTOR_BG_HAKA_SGAMI: C2RustUnnamed_18 = 177;
pub const ACTOR_BG_HAKA_SHIP: C2RustUnnamed_18 = 176;
pub const ACTOR_BG_HAKA_MEGANEBG: C2RustUnnamed_18 = 175;
pub const ACTOR_BG_HAKA_MEGANE: C2RustUnnamed_18 = 174;
pub const ACTOR_EN_VB_BALL: C2RustUnnamed_18 = 173;
pub const ACTOR_BG_VB_SIMA: C2RustUnnamed_18 = 172;
pub const ACTOR_EN_FW: C2RustUnnamed_18 = 171;
pub const ACTOR_DEMO_TRE_LGT: C2RustUnnamed_18 = 170;
pub const ACTOR_DEMO_IM: C2RustUnnamed_18 = 169;
pub const ACTOR_DEMO_DU: C2RustUnnamed_18 = 168;
pub const ACTOR_EN_ENCOUNT1: C2RustUnnamed_18 = 167;
pub const ACTOR_EN_RL: C2RustUnnamed_18 = 166;
pub const ACTOR_EN_DHA: C2RustUnnamed_18 = 165;
pub const ACTOR_EN_DH: C2RustUnnamed_18 = 164;
pub const ACTOR_EN_FD_FIRE: C2RustUnnamed_18 = 163;
pub const ACTOR_BOSS_FD2: C2RustUnnamed_18 = 162;
pub const ACTOR_EN_RU1: C2RustUnnamed_18 = 161;
pub const ACTOR_UNSET_A0: C2RustUnnamed_18 = 160;
pub const ACTOR_MAGIC_FIRE: C2RustUnnamed_18 = 159;
pub const ACTOR_MAGIC_WIND: C2RustUnnamed_18 = 158;
pub const ACTOR_BG_HAKA: C2RustUnnamed_18 = 157;
pub const ACTOR_BG_SPOT02_OBJECTS: C2RustUnnamed_18 = 156;
pub const ACTOR_DOOR_ANA: C2RustUnnamed_18 = 155;
pub const ACTOR_EN_HORSE_LINK_CHILD: C2RustUnnamed_18 = 154;
pub const ACTOR_EN_FD: C2RustUnnamed_18 = 153;
pub const ACTOR_EN_DU: C2RustUnnamed_18 = 152;
pub const ACTOR_OBJECT_KANKYO: C2RustUnnamed_18 = 151;
pub const ACTOR_BOSS_FD: C2RustUnnamed_18 = 150;
pub const ACTOR_EN_SW: C2RustUnnamed_18 = 149;
pub const ACTOR_OBJ_MURE: C2RustUnnamed_18 = 148;
pub const ACTOR_BG_PO_EVENT: C2RustUnnamed_18 = 147;
pub const ACTOR_BG_HEAVY_BLOCK: C2RustUnnamed_18 = 146;
pub const ACTOR_EN_PO_SISTERS: C2RustUnnamed_18 = 145;
pub const ACTOR_EN_RD: C2RustUnnamed_18 = 144;
pub const ACTOR_EN_HEISHI1: C2RustUnnamed_18 = 143;
pub const ACTOR_EN_FLOORMAS: C2RustUnnamed_18 = 142;
pub const ACTOR_BG_HIDAN_FWBIG: C2RustUnnamed_18 = 141;
pub const ACTOR_DEMO_KANKYO: C2RustUnnamed_18 = 140;
pub const ACTOR_DEMO_EFFECT: C2RustUnnamed_18 = 139;
pub const ACTOR_EN_VM: C2RustUnnamed_18 = 138;
pub const ACTOR_BG_MORI_RAKKATENJO: C2RustUnnamed_18 = 137;
pub const ACTOR_BG_MORI_KAITENKABE: C2RustUnnamed_18 = 136;
pub const ACTOR_BG_MORI_ELEVATOR: C2RustUnnamed_18 = 135;
pub const ACTOR_BG_MORI_BIGST: C2RustUnnamed_18 = 134;
pub const ACTOR_EN_TK: C2RustUnnamed_18 = 133;
pub const ACTOR_EN_TA: C2RustUnnamed_18 = 132;
pub const ACTOR_UNSET_83: C2RustUnnamed_18 = 131;
pub const ACTOR_EN_VASE: C2RustUnnamed_18 = 130;
pub const ACTOR_EN_AROW_TRAP: C2RustUnnamed_18 = 129;
pub const ACTOR_EN_TRAP: C2RustUnnamed_18 = 128;
pub const ACTOR_UNSET_7F: C2RustUnnamed_18 = 127;
pub const ACTOR_UNSET_7E: C2RustUnnamed_18 = 126;
pub const ACTOR_EN_PU_BOX: C2RustUnnamed_18 = 125;
pub const ACTOR_EN_LIGHTBOX: C2RustUnnamed_18 = 124;
pub const ACTOR_UNSET_7B: C2RustUnnamed_18 = 123;
pub const ACTOR_UNSET_7A: C2RustUnnamed_18 = 122;
pub const ACTOR_UNSET_79: C2RustUnnamed_18 = 121;
pub const ACTOR_UNSET_78: C2RustUnnamed_18 = 120;
pub const ACTOR_EN_WOOD02: C2RustUnnamed_18 = 119;
pub const ACTOR_UNSET_76: C2RustUnnamed_18 = 118;
pub const ACTOR_UNSET_75: C2RustUnnamed_18 = 117;
pub const ACTOR_UNSET_74: C2RustUnnamed_18 = 116;
pub const ACTOR_UNSET_73: C2RustUnnamed_18 = 115;
pub const ACTOR_EN_BIRD: C2RustUnnamed_18 = 114;
pub const ACTOR_BG_HIDAN_HAMSTEP: C2RustUnnamed_18 = 113;
pub const ACTOR_DOOR_TOKI: C2RustUnnamed_18 = 112;
pub const ACTOR_BG_HIDAN_KOUSI: C2RustUnnamed_18 = 111;
pub const ACTOR_BG_MJIN: C2RustUnnamed_18 = 110;
pub const ACTOR_EN_FHG_FIRE: C2RustUnnamed_18 = 109;
pub const ACTOR_BG_TOKI_SWD: C2RustUnnamed_18 = 108;
pub const ACTOR_EN_YUKABYUN: C2RustUnnamed_18 = 107;
pub const ACTOR_BG_TOKI_HIKARI: C2RustUnnamed_18 = 106;
pub const ACTOR_EN_BB: C2RustUnnamed_18 = 105;
pub const ACTOR_BG_MORI_HINERI: C2RustUnnamed_18 = 104;
pub const ACTOR_EN_FHG: C2RustUnnamed_18 = 103;
pub const ACTOR_ARMS_HOOK: C2RustUnnamed_18 = 102;
pub const ACTOR_BG_MIZU_WATER: C2RustUnnamed_18 = 101;
pub const ACTOR_BG_MIZU_MOVEBG: C2RustUnnamed_18 = 100;
pub const ACTOR_EN_VALI: C2RustUnnamed_18 = 99;
pub const ACTOR_BG_MENKURI_EYE: C2RustUnnamed_18 = 98;
pub const ACTOR_BG_MENKURI_KAITEN: C2RustUnnamed_18 = 97;
pub const ACTOR_EN_DEKUNUTS: C2RustUnnamed_18 = 96;
pub const ACTOR_ITEM_B_HEART: C2RustUnnamed_18 = 95;
pub const ACTOR_OBJ_SYOKUDAI: C2RustUnnamed_18 = 94;
pub const ACTOR_DOOR_WARP1: C2RustUnnamed_18 = 93;
pub const ACTOR_BG_DDAN_KD: C2RustUnnamed_18 = 92;
pub const ACTOR_EN_HORSE_ZELDA: C2RustUnnamed_18 = 91;
pub const ACTOR_EN_JJ: C2RustUnnamed_18 = 90;
pub const ACTOR_BG_BREAKWALL: C2RustUnnamed_18 = 89;
pub const ACTOR_BG_DDAN_JD: C2RustUnnamed_18 = 88;
pub const ACTOR_EN_M_THUNDER: C2RustUnnamed_18 = 87;
pub const ACTOR_EN_M_FIRE1: C2RustUnnamed_18 = 86;
pub const ACTOR_EN_DEKUBABA: C2RustUnnamed_18 = 85;
pub const ACTOR_EN_AM: C2RustUnnamed_18 = 84;
pub const ACTOR_UNSET_53: C2RustUnnamed_18 = 83;
pub const ACTOR_BOSS_GANONDROF: C2RustUnnamed_18 = 82;
pub const ACTOR_BG_YDAN_MARUTA: C2RustUnnamed_18 = 81;
pub const ACTOR_BG_YDAN_HASI: C2RustUnnamed_18 = 80;
pub const ACTOR_EN_OE2: C2RustUnnamed_18 = 79;
pub const ACTOR_BG_HIDAN_FSLIFT: C2RustUnnamed_18 = 78;
pub const ACTOR_EN_ZL2: C2RustUnnamed_18 = 77;
pub const ACTOR_EN_BOMBF: C2RustUnnamed_18 = 76;
pub const ACTOR_EN_MB: C2RustUnnamed_18 = 75;
pub const ACTOR_BG_SPOT00_HANEBASI: C2RustUnnamed_18 = 74;
pub const ACTOR_BG_HIDAN_CURTAIN: C2RustUnnamed_18 = 73;
pub const ACTOR_EN_XC: C2RustUnnamed_18 = 72;
pub const ACTOR_BG_HIDAN_SYOKU: C2RustUnnamed_18 = 71;
pub const ACTOR_BG_HIDAN_SIMA: C2RustUnnamed_18 = 70;
pub const ACTOR_BG_HIDAN_SEKIZOU: C2RustUnnamed_18 = 69;
pub const ACTOR_BG_HIDAN_RSEKIZOU: C2RustUnnamed_18 = 68;
pub const ACTOR_BG_HIDAN_ROCK: C2RustUnnamed_18 = 67;
pub const ACTOR_EN_HORSE_GANON: C2RustUnnamed_18 = 66;
pub const ACTOR_BG_HIDAN_HROCK: C2RustUnnamed_18 = 65;
pub const ACTOR_BG_HIDAN_DALM: C2RustUnnamed_18 = 64;
pub const ACTOR_BG_DODOAGO: C2RustUnnamed_18 = 63;
pub const ACTOR_BG_TREEMOUTH: C2RustUnnamed_18 = 62;
pub const ACTOR_EN_OSSAN: C2RustUnnamed_18 = 61;
pub const ACTOR_EN_HORSE_NORMAL: C2RustUnnamed_18 = 60;
pub const ACTOR_EN_RIVER_SOUND: C2RustUnnamed_18 = 59;
pub const ACTOR_EN_EIYER: C2RustUnnamed_18 = 58;
pub const ACTOR_EN_A_OBJ: C2RustUnnamed_18 = 57;
pub const ACTOR_EN_BW: C2RustUnnamed_18 = 56;
pub const ACTOR_EN_ST: C2RustUnnamed_18 = 55;
pub const ACTOR_UNSET_36: C2RustUnnamed_18 = 54;
pub const ACTOR_EN_TP: C2RustUnnamed_18 = 53;
pub const ACTOR_EN_BILI: C2RustUnnamed_18 = 52;
pub const ACTOR_EN_TORCH2: C2RustUnnamed_18 = 51;
pub const ACTOR_EN_BOOM: C2RustUnnamed_18 = 50;
pub const ACTOR_UNSET_31: C2RustUnnamed_18 = 49;
pub const ACTOR_EN_BDFIRE: C2RustUnnamed_18 = 48;
pub const ACTOR_EN_DODOJR: C2RustUnnamed_18 = 47;
pub const ACTOR_DOOR_SHUTTER: C2RustUnnamed_18 = 46;
pub const ACTOR_EN_BUBBLE: C2RustUnnamed_18 = 45;
pub const ACTOR_BG_PUSHBOX: C2RustUnnamed_18 = 44;
pub const ACTOR_EN_GOMA: C2RustUnnamed_18 = 43;
pub const ACTOR_EN_VIEWER: C2RustUnnamed_18 = 42;
pub const ACTOR_EN_ZL1: C2RustUnnamed_18 = 41;
pub const ACTOR_BOSS_GOMA: C2RustUnnamed_18 = 40;
pub const ACTOR_BOSS_DODONGO: C2RustUnnamed_18 = 39;
pub const ACTOR_EN_HATA: C2RustUnnamed_18 = 38;
pub const ACTOR_EN_ZF: C2RustUnnamed_18 = 37;
pub const ACTOR_EN_SCENE_CHANGE: C2RustUnnamed_18 = 36;
pub const ACTOR_EN_HOLL: C2RustUnnamed_18 = 35;
pub const ACTOR_UNSET_22: C2RustUnnamed_18 = 34;
pub const ACTOR_EN_FISH: C2RustUnnamed_18 = 33;
pub const ACTOR_EN_INSECT: C2RustUnnamed_18 = 32;
pub const ACTOR_UNSET_1F: C2RustUnnamed_18 = 31;
pub const ACTOR_EN_BUTTE: C2RustUnnamed_18 = 30;
pub const ACTOR_EN_PEEHAT: C2RustUnnamed_18 = 29;
pub const ACTOR_EN_REEBA: C2RustUnnamed_18 = 28;
pub const ACTOR_EN_TITE: C2RustUnnamed_18 = 27;
pub const ACTOR_UNSET_1A: C2RustUnnamed_18 = 26;
pub const ACTOR_EN_NIW: C2RustUnnamed_18 = 25;
pub const ACTOR_EN_ELF: C2RustUnnamed_18 = 24;
pub const ACTOR_UNSET_17: C2RustUnnamed_18 = 23;
pub const ACTOR_EN_ARROW: C2RustUnnamed_18 = 22;
pub const ACTOR_EN_ITEM00: C2RustUnnamed_18 = 21;
pub const ACTOR_EN_HORSE: C2RustUnnamed_18 = 20;
pub const ACTOR_EN_FIREFLY: C2RustUnnamed_18 = 19;
pub const ACTOR_EN_DODONGO: C2RustUnnamed_18 = 18;
pub const ACTOR_EN_WALLMAS: C2RustUnnamed_18 = 17;
pub const ACTOR_EN_BOM: C2RustUnnamed_18 = 16;
pub const ACTOR_BG_YDAN_SP: C2RustUnnamed_18 = 15;
pub const ACTOR_EN_OKUTA: C2RustUnnamed_18 = 14;
pub const ACTOR_EN_POH: C2RustUnnamed_18 = 13;
pub const ACTOR_BG_HIDAN_FIREWALL: C2RustUnnamed_18 = 12;
pub const ACTOR_BG_DY_YOSEIZO: C2RustUnnamed_18 = 11;
pub const ACTOR_EN_BOX: C2RustUnnamed_18 = 10;
pub const ACTOR_EN_DOOR: C2RustUnnamed_18 = 9;
pub const ACTOR_EN_LIGHT: C2RustUnnamed_18 = 8;
pub const ACTOR_EN_PART: C2RustUnnamed_18 = 7;
pub const ACTOR_UNSET_6: C2RustUnnamed_18 = 6;
pub const ACTOR_UNSET_5: C2RustUnnamed_18 = 5;
pub const ACTOR_EN_GIRLA: C2RustUnnamed_18 = 4;
pub const ACTOR_UNSET_3: C2RustUnnamed_18 = 3;
pub const ACTOR_EN_TEST: C2RustUnnamed_18 = 2;
pub const ACTOR_UNSET_1: C2RustUnnamed_18 = 1;
pub const ACTOR_PLAYER: C2RustUnnamed_18 = 0;
pub type C2RustUnnamed_19 = libc::c_uint;
pub const OBJECT_ID_MAX: C2RustUnnamed_19 = 402;
pub const OBJECT_ZL4: C2RustUnnamed_19 = 401;
pub const OBJECT_TIMEBLOCK: C2RustUnnamed_19 = 400;
pub const OBJECT_OUKE_HAKA: C2RustUnnamed_19 = 399;
pub const OBJECT_DOOR_KILLER: C2RustUnnamed_19 = 398;
pub const OBJECT_GI_SWORD_1: C2RustUnnamed_19 = 397;
pub const OBJECT_COB: C2RustUnnamed_19 = 396;
pub const OBJECT_COW: C2RustUnnamed_19 = 395;
pub const OBJECT_BWALL: C2RustUnnamed_19 = 394;
pub const OBJECT_PS: C2RustUnnamed_19 = 393;
pub const OBJECT_GS: C2RustUnnamed_19 = 392;
pub const OBJECT_HAKA_DOOR: C2RustUnnamed_19 = 391;
pub const OBJECT_GEFF: C2RustUnnamed_19 = 390;
pub const OBJECT_GJ: C2RustUnnamed_19 = 389;
pub const OBJECT_SKB: C2RustUnnamed_19 = 388;
pub const OBJECT_WF: C2RustUnnamed_19 = 387;
pub const OBJECT_MU: C2RustUnnamed_19 = 386;
pub const OBJECT_SPOT01_MATOYAB: C2RustUnnamed_19 = 385;
pub const OBJECT_SPOT01_MATOYA: C2RustUnnamed_19 = 384;
pub const OBJECT_GI_RUPY: C2RustUnnamed_19 = 383;
pub const OBJECT_GANON_ANIME3: C2RustUnnamed_19 = 382;
pub const OBJECT_GANON_ANIME2: C2RustUnnamed_19 = 381;
pub const OBJECT_GANON_ANIME1: C2RustUnnamed_19 = 380;
pub const OBJECT_GI_DEKUPOUCH: C2RustUnnamed_19 = 379;
pub const OBJECT_EFC_DOUGHNUT: C2RustUnnamed_19 = 378;
pub const OBJECT_DEMO_KEKKAI: C2RustUnnamed_19 = 377;
pub const OBJECT_BOWL: C2RustUnnamed_19 = 376;
pub const OBJECT_GI_SOUL: C2RustUnnamed_19 = 375;
pub const OBJECT_GI_GHOST: C2RustUnnamed_19 = 374;
pub const OBJECT_GI_BUTTERFLY: C2RustUnnamed_19 = 373;
pub const OBJECT_GI_INSECT: C2RustUnnamed_19 = 372;
pub const OBJECT_GI_FIRE: C2RustUnnamed_19 = 371;
pub const OBJECT_DNK: C2RustUnnamed_19 = 370;
pub const OBJECT_DNS: C2RustUnnamed_19 = 369;
pub const OBJECT_KIBAKO2: C2RustUnnamed_19 = 368;
pub const OBJECT_SPOT11_OBJ: C2RustUnnamed_19 = 367;
pub const OBJECT_UNSET_16E: C2RustUnnamed_19 = 366;
pub const OBJECT_JYA_DOOR: C2RustUnnamed_19 = 365;
pub const OBJECT_JYA_IRON: C2RustUnnamed_19 = 364;
pub const OBJECT_DOG: C2RustUnnamed_19 = 363;
pub const OBJECT_GR: C2RustUnnamed_19 = 362;
pub const OBJECT_GELDB: C2RustUnnamed_19 = 361;
pub const OBJECT_SHOPNUTS: C2RustUnnamed_19 = 360;
pub const OBJECT_GLA: C2RustUnnamed_19 = 359;
pub const OBJECT_SPOT00_BREAK: C2RustUnnamed_19 = 358;
pub const OBJECT_RS: C2RustUnnamed_19 = 357;
pub const OBJECT_HINTNUTS: C2RustUnnamed_19 = 356;
pub const OBJECT_BOMBIWA: C2RustUnnamed_19 = 355;
pub const OBJECT_SPOT12_OBJ: C2RustUnnamed_19 = 354;
pub const OBJECT_SPOT05_OBJECTS: C2RustUnnamed_19 = 353;
pub const OBJECT_BG: C2RustUnnamed_19 = 352;
pub const OBJECT_BIGOKUTA: C2RustUnnamed_19 = 351;
pub const OBJECT_SSH: C2RustUnnamed_19 = 350;
pub const OBJECT_GI_GODDESS: C2RustUnnamed_19 = 349;
pub const OBJECT_GI_SUTARU: C2RustUnnamed_19 = 348;
pub const OBJECT_FISH: C2RustUnnamed_19 = 347;
pub const OBJECT_EC: C2RustUnnamed_19 = 346;
pub const OBJECT_DS2: C2RustUnnamed_19 = 345;
pub const OBJECT_GI_M_ARROW: C2RustUnnamed_19 = 344;
pub const OBJECT_GI_HOVERBOOTS: C2RustUnnamed_19 = 343;
pub const OBJECT_ZG: C2RustUnnamed_19 = 342;
pub const OBJECT_TS: C2RustUnnamed_19 = 341;
pub const OBJECT_KA: C2RustUnnamed_19 = 340;
pub const OBJECT_GANON2: C2RustUnnamed_19 = 339;
pub const OBJECT_GI_GERUDOMASK: C2RustUnnamed_19 = 338;
pub const OBJECT_GI_ZORAMASK: C2RustUnnamed_19 = 337;
pub const OBJECT_GI_GOLONMASK: C2RustUnnamed_19 = 336;
pub const OBJECT_ZL2_ANIME2: C2RustUnnamed_19 = 335;
pub const OBJECT_ZL2_ANIME1: C2RustUnnamed_19 = 334;
pub const OBJECT_EFC_ERUPC: C2RustUnnamed_19 = 333;
pub const OBJECT_GT: C2RustUnnamed_19 = 332;
pub const OBJECT_DOOR_GERUDO: C2RustUnnamed_19 = 331;
pub const OBJECT_MAG: C2RustUnnamed_19 = 330;
pub const OBJECT_GI_FROG: C2RustUnnamed_19 = 329;
pub const OBJECT_GI_SOLDOUT: C2RustUnnamed_19 = 328;
pub const OBJECT_GI_BRACELET: C2RustUnnamed_19 = 327;
pub const OBJECT_GI_PRESCRIPTION: C2RustUnnamed_19 = 326;
pub const OBJECT_CS: C2RustUnnamed_19 = 325;
pub const OBJECT_JS: C2RustUnnamed_19 = 324;
pub const OBJECT_GI_BROKENSWORD: C2RustUnnamed_19 = 323;
pub const OBJECT_GI_TICKETSTONE: C2RustUnnamed_19 = 322;
pub const OBJECT_GI_MUSHROOM: C2RustUnnamed_19 = 321;
pub const OBJECT_GI_POWDER: C2RustUnnamed_19 = 320;
pub const OBJECT_GI_EYE_LOTION: C2RustUnnamed_19 = 319;
pub const OBJECT_OS: C2RustUnnamed_19 = 318;
pub const OBJECT_FA: C2RustUnnamed_19 = 317;
pub const OBJECT_MM: C2RustUnnamed_19 = 316;
pub const OBJECT_STREAM: C2RustUnnamed_19 = 315;
pub const OBJECT_SIOFUKI: C2RustUnnamed_19 = 314;
pub const OBJECT_GANON_OBJECTS: C2RustUnnamed_19 = 313;
pub const OBJECT_GI_TRUTH_MASK: C2RustUnnamed_19 = 312;
pub const OBJECT_GI_RABIT_MASK: C2RustUnnamed_19 = 311;
pub const OBJECT_GI_SKJ_MASK: C2RustUnnamed_19 = 310;
pub const OBJECT_GI_REDEAD_MASK: C2RustUnnamed_19 = 309;
pub const OBJECT_GI_KI_TAN_MASK: C2RustUnnamed_19 = 308;
pub const OBJECT_FU: C2RustUnnamed_19 = 307;
pub const OBJECT_MK: C2RustUnnamed_19 = 306;
pub const OBJECT_OWL: C2RustUnnamed_19 = 305;
pub const OBJECT_GJYO_OBJECTS: C2RustUnnamed_19 = 304;
pub const OBJECT_KANBAN: C2RustUnnamed_19 = 303;
pub const OBJECT_GI_COIN: C2RustUnnamed_19 = 302;
pub const OBJECT_GI_GLOVES: C2RustUnnamed_19 = 301;
pub const OBJECT_TSUBO: C2RustUnnamed_19 = 300;
pub const OBJECT_KUSA: C2RustUnnamed_19 = 299;
pub const OBJECT_LIGHTSWITCH: C2RustUnnamed_19 = 298;
pub const OBJECT_INGATE: C2RustUnnamed_19 = 297;
pub const OBJECT_HS: C2RustUnnamed_19 = 296;
pub const OBJECT_MS: C2RustUnnamed_19 = 295;
pub const OBJECT_GM: C2RustUnnamed_19 = 294;
pub const OBJECT_BLKOBJ: C2RustUnnamed_19 = 293;
pub const OBJECT_NWC: C2RustUnnamed_19 = 292;
pub const OBJECT_UNSET_123: C2RustUnnamed_19 = 291;
pub const OBJECT_DAIKU: C2RustUnnamed_19 = 290;
pub const OBJECT_TORYO: C2RustUnnamed_19 = 289;
pub const OBJECT_UNSET_120: C2RustUnnamed_19 = 288;
pub const OBJECT_GOROIWA: C2RustUnnamed_19 = 287;
pub const OBJECT_MAMENOKI: C2RustUnnamed_19 = 286;
pub const OBJECT_D_LIFT: C2RustUnnamed_19 = 285;
pub const OBJECT_D_HSBLOCK: C2RustUnnamed_19 = 284;
pub const OBJECT_D_ELEVATOR: C2RustUnnamed_19 = 283;
pub const OBJECT_GND_MAGIC: C2RustUnnamed_19 = 282;
pub const OBJECT_GI_SEED: C2RustUnnamed_19 = 281;
pub const OBJECT_GI_BOOTS_2: C2RustUnnamed_19 = 280;
pub const OBJECT_YABUSAME_POINT: C2RustUnnamed_19 = 279;
pub const OBJECT_GE1: C2RustUnnamed_19 = 278;
pub const OBJECT_BOB: C2RustUnnamed_19 = 277;
pub const OBJECT_FZ: C2RustUnnamed_19 = 276;
pub const OBJECT_SPOT07_OBJECT: C2RustUnnamed_19 = 275;
pub const OBJECT_SPOT03_OBJECT: C2RustUnnamed_19 = 274;
pub const OBJECT_BOJ: C2RustUnnamed_19 = 273;
pub const OBJECT_ANE: C2RustUnnamed_19 = 272;
pub const OBJECT_DS: C2RustUnnamed_19 = 271;
pub const OBJECT_GI_OCARINA_0: C2RustUnnamed_19 = 270;
pub const OBJECT_BBA: C2RustUnnamed_19 = 269;
pub const OBJECT_BJI: C2RustUnnamed_19 = 268;
pub const OBJECT_GI_BOTTLE_LETTER: C2RustUnnamed_19 = 267;
pub const OBJECT_SKJ: C2RustUnnamed_19 = 266;
pub const OBJECT_GI_NIWATORI: C2RustUnnamed_19 = 265;
pub const OBJECT_CNE: C2RustUnnamed_19 = 264;
pub const OBJECT_AHG: C2RustUnnamed_19 = 263;
pub const OBJECT_IK: C2RustUnnamed_19 = 262;
pub const OBJECT_AOB: C2RustUnnamed_19 = 261;
pub const OBJECT_MASTERZOORA: C2RustUnnamed_19 = 260;
pub const OBJECT_MASTERGOLON: C2RustUnnamed_19 = 259;
pub const OBJECT_MASTERKOKIRIHEAD: C2RustUnnamed_19 = 258;
pub const OBJECT_MASTERKOKIRI: C2RustUnnamed_19 = 257;
pub const OBJECT_UMAJUMP: C2RustUnnamed_19 = 256;
pub const OBJECT_KZ: C2RustUnnamed_19 = 255;
pub const OBJECT_ZO: C2RustUnnamed_19 = 254;
pub const OBJECT_KW1: C2RustUnnamed_19 = 253;
pub const OBJECT_KM1: C2RustUnnamed_19 = 252;
pub const OBJECT_MD: C2RustUnnamed_19 = 251;
pub const OBJECT_MD_UNUSED: C2RustUnnamed_19 = 250;
pub const OBJECT_SPOT01_OBJECTS: C2RustUnnamed_19 = 249;
pub const OBJECT_GI_LONGSWORD: C2RustUnnamed_19 = 248;
pub const OBJECT_GI_GRASS: C2RustUnnamed_19 = 247;
pub const OBJECT_GI_HAMMER: C2RustUnnamed_19 = 246;
pub const OBJECT_GI_SAW: C2RustUnnamed_19 = 245;
pub const OBJECT_GI_FISH: C2RustUnnamed_19 = 244;
pub const OBJECT_GI_BEAN: C2RustUnnamed_19 = 243;
pub const OBJECT_GI_CLOTHES: C2RustUnnamed_19 = 242;
pub const OBJECT_JYA_OBJ: C2RustUnnamed_19 = 241;
pub const OBJECT_SPOT15_OBJ: C2RustUnnamed_19 = 240;
pub const OBJECT_GI_LETTER: C2RustUnnamed_19 = 239;
pub const OBJECT_GI_SHIELD_3: C2RustUnnamed_19 = 238;
pub const OBJECT_DEMO_6K: C2RustUnnamed_19 = 237;
pub const OBJECT_ANI: C2RustUnnamed_19 = 236;
pub const OBJECT_GI_LIQUID: C2RustUnnamed_19 = 235;
pub const OBJECT_GI_GLASSES: C2RustUnnamed_19 = 234;
pub const OBJECT_GI_BOW: C2RustUnnamed_19 = 233;
pub const OBJECT_GI_BOOMERANG: C2RustUnnamed_19 = 232;
pub const OBJECT_GI_PACHINKO: C2RustUnnamed_19 = 231;
pub const OBJECT_FR: C2RustUnnamed_19 = 230;
pub const OBJECT_NY: C2RustUnnamed_19 = 229;
pub const OBJECT_UNSET_E4: C2RustUnnamed_19 = 228;
pub const OBJECT_NY_UNUSED: C2RustUnnamed_19 = 227;
pub const OBJECT_SST: C2RustUnnamed_19 = 226;
pub const OBJECT_GANON: C2RustUnnamed_19 = 225;
pub const OBJECT_MA1: C2RustUnnamed_19 = 224;
pub const OBJECT_GI_MILK: C2RustUnnamed_19 = 223;
pub const OBJECT_GI_OCARINA: C2RustUnnamed_19 = 222;
pub const OBJECT_GI_HOOKSHOT: C2RustUnnamed_19 = 221;
pub const OBJECT_GI_SHIELD_2: C2RustUnnamed_19 = 220;
pub const OBJECT_GI_SCALE: C2RustUnnamed_19 = 219;
pub const OBJECT_GI_EGG: C2RustUnnamed_19 = 218;
pub const OBJECT_GI_BOMB_2: C2RustUnnamed_19 = 217;
pub const OBJECT_GI_ARROW: C2RustUnnamed_19 = 216;
pub const OBJECT_GI_GERUDO: C2RustUnnamed_19 = 215;
pub const OBJECT_ANUBICE: C2RustUnnamed_19 = 214;
pub const OBJECT_BXA: C2RustUnnamed_19 = 213;
pub const OBJECT_RR: C2RustUnnamed_19 = 212;
pub const OBJECT_TW: C2RustUnnamed_19 = 211;
pub const OBJECT_HNI: C2RustUnnamed_19 = 210;
pub const OBJECT_GI_PURSE: C2RustUnnamed_19 = 209;
pub const OBJECT_MA2: C2RustUnnamed_19 = 208;
pub const OBJECT_OF1S: C2RustUnnamed_19 = 207;
pub const OBJECT_GI_BOMB_1: C2RustUnnamed_19 = 206;
pub const OBJECT_GI_MAGICPOT: C2RustUnnamed_19 = 205;
pub const OBJECT_DEKUJR: C2RustUnnamed_19 = 204;
pub const OBJECT_GI_SHIELD_1: C2RustUnnamed_19 = 203;
pub const OBJECT_RU2: C2RustUnnamed_19 = 202;
pub const OBJECT_OF1D_MAP: C2RustUnnamed_19 = 201;
pub const OBJECT_GI_MAP: C2RustUnnamed_19 = 200;
pub const OBJECT_GI_STICK: C2RustUnnamed_19 = 199;
pub const OBJECT_GI_BOTTLE: C2RustUnnamed_19 = 198;
pub const OBJECT_OS_ANIME: C2RustUnnamed_19 = 197;
pub const OBJECT_OE4S: C2RustUnnamed_19 = 196;
pub const OBJECT_OE1S: C2RustUnnamed_19 = 195;
pub const OBJECT_SPOT16_OBJ: C2RustUnnamed_19 = 194;
pub const OBJECT_TR: C2RustUnnamed_19 = 193;
pub const OBJECT_IN: C2RustUnnamed_19 = 192;
pub const OBJECT_GI_BOMBPOUCH: C2RustUnnamed_19 = 191;
pub const OBJECT_GI_ARROWCASE: C2RustUnnamed_19 = 190;
pub const OBJECT_GI_HEARTS: C2RustUnnamed_19 = 189;
pub const OBJECT_SA: C2RustUnnamed_19 = 188;
pub const OBJECT_GI_NUTS: C2RustUnnamed_19 = 187;
pub const OBJECT_GI_MEDAL: C2RustUnnamed_19 = 186;
pub const OBJECT_GI_BOSSKEY: C2RustUnnamed_19 = 185;
pub const OBJECT_GI_COMPASS: C2RustUnnamed_19 = 184;
pub const OBJECT_GI_HEART: C2RustUnnamed_19 = 183;
pub const OBJECT_GI_MELODY: C2RustUnnamed_19 = 182;
pub const OBJECT_SB: C2RustUnnamed_19 = 181;
pub const OBJECT_MO: C2RustUnnamed_19 = 180;
pub const OBJECT_NB: C2RustUnnamed_19 = 179;
pub const OBJECT_SHOP_DUNGEN: C2RustUnnamed_19 = 178;
pub const OBJECT_SPOT17_OBJ: C2RustUnnamed_19 = 177;
pub const OBJECT_BDOOR: C2RustUnnamed_19 = 176;
pub const OBJECT_SPOT18_OBJ: C2RustUnnamed_19 = 175;
pub const OBJECT_SPOT09_OBJ: C2RustUnnamed_19 = 174;
pub const OBJECT_GI_JEWEL: C2RustUnnamed_19 = 173;
pub const OBJECT_BROB: C2RustUnnamed_19 = 172;
pub const OBJECT_MIR_RAY: C2RustUnnamed_19 = 171;
pub const OBJECT_GI_KEY: C2RustUnnamed_19 = 170;
pub const OBJECT_DEMO_TRE_LGT: C2RustUnnamed_19 = 169;
pub const OBJECT_EFC_TW: C2RustUnnamed_19 = 168;
pub const OBJECT_RL: C2RustUnnamed_19 = 167;
pub const OBJECT_DH: C2RustUnnamed_19 = 166;
pub const OBJECT_FD2: C2RustUnnamed_19 = 165;
pub const OBJECT_SYOKUDAI: C2RustUnnamed_19 = 164;
pub const OBJECT_RU1: C2RustUnnamed_19 = 163;
pub const OBJECT_HAKA: C2RustUnnamed_19 = 162;
pub const OBJECT_SPOT02_OBJECTS: C2RustUnnamed_19 = 161;
pub const OBJECT_HORSE_LINK_CHILD: C2RustUnnamed_19 = 160;
pub const OBJECT_MEDAL: C2RustUnnamed_19 = 159;
pub const OBJECT_FW: C2RustUnnamed_19 = 158;
pub const OBJECT_DU: C2RustUnnamed_19 = 157;
pub const OBJECT_FD: C2RustUnnamed_19 = 156;
pub const OBJECT_GNDD: C2RustUnnamed_19 = 155;
pub const OBJECT_HEAVY_OBJECT: C2RustUnnamed_19 = 154;
pub const OBJECT_PO_SISTERS: C2RustUnnamed_19 = 153;
pub const OBJECT_RD: C2RustUnnamed_19 = 152;
pub const OBJECT_SD: C2RustUnnamed_19 = 151;
pub const OBJECT_BDAN_OBJECTS: C2RustUnnamed_19 = 150;
pub const OBJECT_TRIFORCE_SPOT: C2RustUnnamed_19 = 149;
pub const OBJECT_LIGHT_RING: C2RustUnnamed_19 = 148;
pub const OBJECT_GOD_LGT: C2RustUnnamed_19 = 147;
pub const OBJECT_EFC_STAR_FIELD: C2RustUnnamed_19 = 146;
pub const OBJECT_EFC_LGT_SHOWER: C2RustUnnamed_19 = 145;
pub const OBJECT_EFC_FLASH: C2RustUnnamed_19 = 144;
pub const OBJECT_EFC_FIRE_BALL: C2RustUnnamed_19 = 143;
pub const OBJECT_EFC_CRYSTAL_LIGHT: C2RustUnnamed_19 = 142;
pub const OBJECT_HAKACH_OBJECTS: C2RustUnnamed_19 = 141;
pub const OBJECT_BV: C2RustUnnamed_19 = 140;
pub const OBJECT_VM: C2RustUnnamed_19 = 139;
pub const OBJECT_XC: C2RustUnnamed_19 = 138;
pub const OBJECT_TK: C2RustUnnamed_19 = 137;
pub const OBJECT_TA: C2RustUnnamed_19 = 136;
pub const OBJECT_IM: C2RustUnnamed_19 = 135;
pub const OBJECT_VASE: C2RustUnnamed_19 = 134;
pub const OBJECT_TRAP: C2RustUnnamed_19 = 133;
pub const OBJECT_UNSET_84: C2RustUnnamed_19 = 132;
pub const OBJECT_UNSET_83: C2RustUnnamed_19 = 131;
pub const OBJECT_PU_BOX: C2RustUnnamed_19 = 130;
pub const OBJECT_LIGHTBOX: C2RustUnnamed_19 = 129;
pub const OBJECT_UNSET_80: C2RustUnnamed_19 = 128;
pub const OBJECT_UNSET_7F: C2RustUnnamed_19 = 127;
pub const OBJECT_UNSET_7E: C2RustUnnamed_19 = 126;
pub const OBJECT_UNSET_7D: C2RustUnnamed_19 = 125;
pub const OBJECT_WOOD02: C2RustUnnamed_19 = 124;
pub const OBJECT_UNSET_7B: C2RustUnnamed_19 = 123;
pub const OBJECT_UNSET_7A: C2RustUnnamed_19 = 122;
pub const OBJECT_UNSET_79: C2RustUnnamed_19 = 121;
pub const OBJECT_UNSET_78: C2RustUnnamed_19 = 120;
pub const OBJECT_BIRD: C2RustUnnamed_19 = 119;
pub const OBJECT_HATA: C2RustUnnamed_19 = 118;
pub const OBJECT_WARP2: C2RustUnnamed_19 = 117;
pub const OBJECT_SPOT08_OBJ: C2RustUnnamed_19 = 116;
pub const OBJECT_MORI_TEX: C2RustUnnamed_19 = 115;
pub const OBJECT_MORI_OBJECTS: C2RustUnnamed_19 = 114;
pub const OBJECT_MORI_HINERI2A: C2RustUnnamed_19 = 113;
pub const OBJECT_MORI_HINERI2: C2RustUnnamed_19 = 112;
pub const OBJECT_MORI_HINERI1A: C2RustUnnamed_19 = 111;
pub const OBJECT_PO_COMPOSER: C2RustUnnamed_19 = 110;
pub const OBJECT_PO_FIELD: C2RustUnnamed_19 = 109;
pub const OBJECT_RELAY_OBJECTS: C2RustUnnamed_19 = 108;
pub const OBJECT_ICE_OBJECTS: C2RustUnnamed_19 = 107;
pub const OBJECT_SPOT06_OBJECTS: C2RustUnnamed_19 = 106;
pub const OBJECT_HAKA_OBJECTS: C2RustUnnamed_19 = 105;
pub const OBJECT_MJIN_OKA: C2RustUnnamed_19 = 104;
pub const OBJECT_MJIN_WIND: C2RustUnnamed_19 = 103;
pub const OBJECT_MJIN_SOUL: C2RustUnnamed_19 = 102;
pub const OBJECT_MJIN_ICE: C2RustUnnamed_19 = 101;
pub const OBJECT_MJIN_FLAME: C2RustUnnamed_19 = 100;
pub const OBJECT_MJIN_DARK: C2RustUnnamed_19 = 99;
pub const OBJECT_MJIN_FLASH: C2RustUnnamed_19 = 98;
pub const OBJECT_MJIN: C2RustUnnamed_19 = 97;
pub const OBJECT_ZL2: C2RustUnnamed_19 = 96;
pub const OBJECT_YUKABYUN: C2RustUnnamed_19 = 95;
pub const OBJECT_TOKI_OBJECTS: C2RustUnnamed_19 = 94;
pub const OBJECT_BB: C2RustUnnamed_19 = 93;
pub const OBJECT_MORI_HINERI1: C2RustUnnamed_19 = 92;
pub const OBJECT_OSSAN: C2RustUnnamed_19 = 91;
pub const OBJECT_FHG: C2RustUnnamed_19 = 90;
pub const OBJECT_MIZU_OBJECTS: C2RustUnnamed_19 = 89;
pub const OBJECT_OA11: C2RustUnnamed_19 = 88;
pub const OBJECT_OA10: C2RustUnnamed_19 = 87;
pub const OBJECT_VALI: C2RustUnnamed_19 = 86;
pub const OBJECT_OE12: C2RustUnnamed_19 = 85;
pub const OBJECT_OE11: C2RustUnnamed_19 = 84;
pub const OBJECT_OE10: C2RustUnnamed_19 = 83;
pub const OBJECT_OE9: C2RustUnnamed_19 = 82;
pub const OBJECT_OE8: C2RustUnnamed_19 = 81;
pub const OBJECT_OE7: C2RustUnnamed_19 = 80;
pub const OBJECT_OE6: C2RustUnnamed_19 = 79;
pub const OBJECT_OE5: C2RustUnnamed_19 = 78;
pub const OBJECT_MENKURI_OBJECTS: C2RustUnnamed_19 = 77;
pub const OBJECT_OE4: C2RustUnnamed_19 = 76;
pub const OBJECT_OE3: C2RustUnnamed_19 = 75;
pub const OBJECT_DEKUNUTS: C2RustUnnamed_19 = 74;
pub const OBJECT_B_HEART: C2RustUnnamed_19 = 73;
pub const OBJECT_WARP1: C2RustUnnamed_19 = 72;
pub const OBJECT_OPENING_DEMO1: C2RustUnnamed_19 = 71;
pub const OBJECT_HORSE_ZELDA: C2RustUnnamed_19 = 70;
pub const OBJECT_OB4: C2RustUnnamed_19 = 69;
pub const OBJECT_OB3: C2RustUnnamed_19 = 68;
pub const OBJECT_OB2: C2RustUnnamed_19 = 67;
pub const OBJECT_OA9: C2RustUnnamed_19 = 66;
pub const OBJECT_OA8: C2RustUnnamed_19 = 65;
pub const OBJECT_JJ: C2RustUnnamed_19 = 64;
pub const OBJECT_OA7: C2RustUnnamed_19 = 63;
pub const OBJECT_OA6: C2RustUnnamed_19 = 62;
pub const OBJECT_OA5: C2RustUnnamed_19 = 61;
pub const OBJECT_OA4: C2RustUnnamed_19 = 60;
pub const OBJECT_OA3: C2RustUnnamed_19 = 59;
pub const OBJECT_UNSET_3A: C2RustUnnamed_19 = 58;
pub const OBJECT_DEKUBABA: C2RustUnnamed_19 = 57;
pub const OBJECT_AM: C2RustUnnamed_19 = 56;
pub const OBJECT_GND: C2RustUnnamed_19 = 55;
pub const OBJECT_YDAN_OBJECTS: C2RustUnnamed_19 = 54;
pub const OBJECT_OE2: C2RustUnnamed_19 = 53;
pub const OBJECT_OE_ANIME: C2RustUnnamed_19 = 52;
pub const OBJECT_OE1: C2RustUnnamed_19 = 51;
pub const OBJECT_SK2: C2RustUnnamed_19 = 50;
pub const OBJECT_BOMBF: C2RustUnnamed_19 = 49;
pub const OBJECT_MB: C2RustUnnamed_19 = 48;
pub const OBJECT_SPOT00_OBJECTS: C2RustUnnamed_19 = 47;
pub const OBJECT_OA2: C2RustUnnamed_19 = 46;
pub const OBJECT_HORSE_GANON: C2RustUnnamed_19 = 45;
pub const OBJECT_HIDAN_OBJECTS: C2RustUnnamed_19 = 44;
pub const OBJECT_DDAN_OBJECTS: C2RustUnnamed_19 = 43;
pub const OBJECT_SPOT04_OBJECTS: C2RustUnnamed_19 = 42;
pub const OBJECT_O_ANIME: C2RustUnnamed_19 = 41;
pub const OBJECT_OB1: C2RustUnnamed_19 = 40;
pub const OBJECT_HORSE_NORMAL: C2RustUnnamed_19 = 39;
pub const OBJECT_EI: C2RustUnnamed_19 = 38;
pub const OBJECT_BW: C2RustUnnamed_19 = 37;
pub const OBJECT_ST: C2RustUnnamed_19 = 36;
pub const OBJECT_OA1: C2RustUnnamed_19 = 35;
pub const OBJECT_TP: C2RustUnnamed_19 = 34;
pub const OBJECT_BL: C2RustUnnamed_19 = 33;
pub const OBJECT_TORCH2: C2RustUnnamed_19 = 32;
pub const OBJECT_DODOJR: C2RustUnnamed_19 = 31;
pub const OBJECT_GOL: C2RustUnnamed_19 = 30;
pub const OBJECT_ZL1: C2RustUnnamed_19 = 29;
pub const OBJECT_GOMA: C2RustUnnamed_19 = 28;
pub const OBJECT_ZF: C2RustUnnamed_19 = 27;
pub const OBJECT_HORSE: C2RustUnnamed_19 = 26;
pub const OBJECT_KINGDODONGO: C2RustUnnamed_19 = 25;
pub const OBJECT_PEEHAT: C2RustUnnamed_19 = 24;
pub const OBJECT_REEBA: C2RustUnnamed_19 = 23;
pub const OBJECT_TITE: C2RustUnnamed_19 = 22;
pub const OBJECT_LINK_CHILD: C2RustUnnamed_19 = 21;
pub const OBJECT_LINK_BOY: C2RustUnnamed_19 = 20;
pub const OBJECT_NIW: C2RustUnnamed_19 = 19;
pub const OBJECT_BUBBLE: C2RustUnnamed_19 = 18;
pub const OBJECT_UNSET_11: C2RustUnnamed_19 = 17;
pub const OBJECT_UNSET_10: C2RustUnnamed_19 = 16;
pub const OBJECT_FIRE: C2RustUnnamed_19 = 15;
pub const OBJECT_BOX: C2RustUnnamed_19 = 14;
pub const OBJECT_FIREFLY: C2RustUnnamed_19 = 13;
pub const OBJECT_DODONGO: C2RustUnnamed_19 = 12;
pub const OBJECT_WALLMASTER: C2RustUnnamed_19 = 11;
pub const OBJECT_DY_OBJ: C2RustUnnamed_19 = 10;
pub const OBJECT_POH: C2RustUnnamed_19 = 9;
pub const OBJECT_CROW: C2RustUnnamed_19 = 8;
pub const OBJECT_OKUTA: C2RustUnnamed_19 = 7;
pub const OBJECT_HUMAN: C2RustUnnamed_19 = 6;
pub const OBJECT_UNSET_5: C2RustUnnamed_19 = 5;
pub const OBJECT_UNSET_4: C2RustUnnamed_19 = 4;
pub const OBJECT_GAMEPLAY_DANGEON_KEEP: C2RustUnnamed_19 = 3;
pub const OBJECT_GAMEPLAY_FIELD_KEEP: C2RustUnnamed_19 = 2;
pub const OBJECT_GAMEPLAY_KEEP: C2RustUnnamed_19 = 1;
pub const OBJECT_INVALID: C2RustUnnamed_19 = 0;
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct InitChainEntry {
    #[bitfield(name = "cont", ty = "u32_0", bits = "0..=0")]
    #[bitfield(name = "type_0", ty = "u32_0", bits = "1..=4")]
    #[bitfield(name = "offset", ty = "u32_0", bits = "5..=15")]
    #[bitfield(name = "value", ty = "s32", bits = "16..=31")]
    pub cont_type_0_offset_value: [u8; 4],
}
pub type C2RustUnnamed_20 = libc::c_uint;
pub const ICHAINTYPE_VEC3S: C2RustUnnamed_20 = 10;
pub const ICHAINTYPE_VEC3F_DIV1000: C2RustUnnamed_20 = 9;
pub const ICHAINTYPE_VEC3F: C2RustUnnamed_20 = 8;
pub const ICHAINTYPE_F32_DIV1000: C2RustUnnamed_20 = 7;
pub const ICHAINTYPE_F32: C2RustUnnamed_20 = 6;
pub const ICHAINTYPE_S32: C2RustUnnamed_20 = 5;
pub const ICHAINTYPE_U32: C2RustUnnamed_20 = 4;
pub const ICHAINTYPE_S16: C2RustUnnamed_20 = 3;
pub const ICHAINTYPE_U16: C2RustUnnamed_20 = 2;
pub const ICHAINTYPE_S8: C2RustUnnamed_20 = 1;
pub const ICHAINTYPE_U8: C2RustUnnamed_20 = 0;
pub type C2RustUnnamed_21 = libc::c_uint;
pub const MSGMODE_PAUSED: C2RustUnnamed_21 = 55;
pub const MSGMODE_TEXT_CLOSING: C2RustUnnamed_21 = 54;
pub const MSGMODE_TEXT_DONE: C2RustUnnamed_21 = 53;
pub const MSGMODE_TEXT_AWAIT_NEXT: C2RustUnnamed_21 = 52;
pub const MSGMODE_FROGS_WAITING: C2RustUnnamed_21 = 51;
pub const MSGMODE_FROGS_PLAYING: C2RustUnnamed_21 = 50;
pub const MSGMODE_FROGS_START: C2RustUnnamed_21 = 49;
pub const MSGMODE_MEMORY_GAME_START_NEXT_ROUND: C2RustUnnamed_21 = 48;
pub const MSGMODE_MEMORY_GAME_ROUND_SUCCESS: C2RustUnnamed_21 = 47;
pub const MSGMODE_MEMORY_GAME_PLAYER_PLAYING: C2RustUnnamed_21 = 46;
pub const MSGMODE_MEMORY_GAME_RIGHT_SKULLKID_WAIT: C2RustUnnamed_21 = 45;
pub const MSGMODE_MEMORY_GAME_RIGHT_SKULLKID_PLAYING: C2RustUnnamed_21 = 44;
pub const MSGMODE_MEMORY_GAME_LEFT_SKULLKID_WAIT: C2RustUnnamed_21 = 43;
pub const MSGMODE_MEMORY_GAME_LEFT_SKULLKID_PLAYING: C2RustUnnamed_21 = 42;
pub const MSGMODE_MEMORY_GAME_START: C2RustUnnamed_21 = 41;
pub const MSGMODE_SCARECROW_PLAYBACK: C2RustUnnamed_21 = 40;
pub const MSGMODE_SCARECROW_RECORDING_DONE: C2RustUnnamed_21 = 39;
pub const MSGMODE_SCARECROW_RECORDING_FAILED: C2RustUnnamed_21 = 38;
pub const MSGMODE_SCARECROW_RECORDING_ONGOING: C2RustUnnamed_21 = 37;
pub const MSGMODE_SCARECROW_RECORDING_START: C2RustUnnamed_21 = 36;
pub const MSGMODE_SCARECROW_LONG_PLAYBACK: C2RustUnnamed_21 = 35;
pub const MSGMODE_SCARECROW_LONG_RECORDING_ONGOING: C2RustUnnamed_21 = 34;
pub const MSGMODE_SCARECROW_LONG_RECORDING_START: C2RustUnnamed_21 = 33;
pub const MSGMODE_UNK_20: C2RustUnnamed_21 = 32;
pub const MSGMODE_OCARINA_AWAIT_INPUT: C2RustUnnamed_21 = 31;
pub const MSGMODE_SONG_PLAYBACK_NOTES_DROP: C2RustUnnamed_21 = 30;
pub const MSGMODE_SONG_PLAYBACK_FAIL: C2RustUnnamed_21 = 29;
pub const MSGMODE_SONG_PLAYBACK_SUCCESS: C2RustUnnamed_21 = 28;
pub const MSGMODE_SONG_PLAYBACK: C2RustUnnamed_21 = 27;
pub const MSGMODE_SONG_DEMONSTRATION_DONE: C2RustUnnamed_21 = 26;
pub const MSGMODE_SONG_DEMONSTRATION: C2RustUnnamed_21 = 25;
pub const MSGMODE_SONG_DEMONSTRATION_SELECT_INSTRUMENT: C2RustUnnamed_21 = 24;
pub const MSGMODE_SONG_PLAYED_ACT: C2RustUnnamed_21 = 23;
pub const MSGMODE_SONG_PLAYED_ACT_BEGIN: C2RustUnnamed_21 = 22;
pub const MSGMODE_DISPLAY_SONG_PLAYED_TEXT: C2RustUnnamed_21 = 21;
pub const MSGMODE_DISPLAY_SONG_PLAYED_TEXT_BEGIN: C2RustUnnamed_21 = 20;
pub const MSGMODE_DISPLAY_SONG_PLAYED: C2RustUnnamed_21 = 19;
pub const MSGMODE_SETUP_DISPLAY_SONG_PLAYED: C2RustUnnamed_21 = 18;
pub const MSGMODE_SONG_PLAYED: C2RustUnnamed_21 = 17;
pub const MSGMODE_OCARINA_NOTES_DROP: C2RustUnnamed_21 = 16;
pub const MSGMODE_OCARINA_FAIL_NO_TEXT: C2RustUnnamed_21 = 15;
pub const MSGMODE_OCARINA_FAIL: C2RustUnnamed_21 = 14;
pub const MSGMODE_OCARINA_CORRECT_PLAYBACK: C2RustUnnamed_21 = 13;
pub const MSGMODE_OCARINA_PLAYING: C2RustUnnamed_21 = 12;
pub const MSGMODE_SONG_PLAYBACK_STARTING: C2RustUnnamed_21 = 11;
pub const MSGMODE_SONG_DEMONSTRATION_STARTING: C2RustUnnamed_21 = 10;
pub const MSGMODE_OCARINA_STARTING: C2RustUnnamed_21 = 9;
pub const MSGMODE_TEXT_DELAYED_BREAK: C2RustUnnamed_21 = 8;
pub const MSGMODE_TEXT_AWAIT_INPUT: C2RustUnnamed_21 = 7;
pub const MSGMODE_TEXT_DISPLAYING: C2RustUnnamed_21 = 6;
pub const MSGMODE_TEXT_CONTINUING: C2RustUnnamed_21 = 5;
pub const MSGMODE_TEXT_NEXT_MSG: C2RustUnnamed_21 = 4;
pub const MSGMODE_TEXT_STARTING: C2RustUnnamed_21 = 3;
pub const MSGMODE_TEXT_BOX_GROWING: C2RustUnnamed_21 = 2;
pub const MSGMODE_TEXT_START: C2RustUnnamed_21 = 1;
pub const MSGMODE_NONE: C2RustUnnamed_21 = 0;
pub type C2RustUnnamed_22 = libc::c_uint;
pub const MTXMODE_APPLY: C2RustUnnamed_22 = 1;
pub const MTXMODE_NEW: C2RustUnnamed_22 = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct MagicFire {
    pub actor: Actor,
    pub collider: ColliderCylinder,
    pub alphaMultiplier: f32_0,
    pub screenTintIntensity: f32_0,
    pub scalingSpeed: f32_0,
    pub action: s16,
    pub screenTintBehaviour: s16,
    pub actionTimer: s16,
    pub screenTintBehaviourTimer: s16,
}
pub const DF_SCREEN_TINT_FINISHED: C2RustUnnamed_24 = 4;
pub const DF_SCREEN_TINT_FADE_OUT: C2RustUnnamed_24 = 3;
pub const DF_SCREEN_TINT_MAINTAIN: C2RustUnnamed_24 = 2;
pub const DF_SCREEN_TINT_FADE_IN: C2RustUnnamed_24 = 1;
pub const DF_SCREEN_TINT_NONE: C2RustUnnamed_24 = 0;
pub const DF_ACTION_EXPAND_QUICKLY: C2RustUnnamed_23 = 3;
pub const DF_ACTION_STOP_EXPANDING: C2RustUnnamed_23 = 2;
pub const DF_ACTION_EXPAND_SLOWLY: C2RustUnnamed_23 = 1;
pub const DF_ACTION_INITIALIZE: C2RustUnnamed_23 = 0;
pub type C2RustUnnamed_23 = libc::c_uint;
pub type C2RustUnnamed_24 = libc::c_uint;
#[no_mangle]
pub static mut Magic_Fire_InitVars: ActorInit =
    unsafe {
        {
            let mut init =
                ActorInit{id: ACTOR_MAGIC_FIRE as libc::c_int as s16,
                          category:
                              ACTORCAT_ITEMACTION as libc::c_int as u8_0,
                          flags:
                              ((1 as libc::c_int) << 4 as libc::c_int |
                                   (1 as libc::c_int) << 25 as libc::c_int) as
                                  u32_0,
                          objectId:
                              OBJECT_GAMEPLAY_KEEP as libc::c_int as s16,
                          instanceSize:
                              ::std::mem::size_of::<MagicFire>() as
                                  libc::c_ulong,
                          init:
                              ::std::mem::transmute::<Option<unsafe extern "C" fn(_:
                                                                                      *mut Actor,
                                                                                  _:
                                                                                      *mut GlobalContext)
                                                                 -> ()>,
                                                      ActorFunc>(Some(MagicFire_Init
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
                                                      ActorFunc>(Some(MagicFire_Destroy
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
                                                      ActorFunc>(Some(MagicFire_Update
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
                                                      ActorFunc>(Some(MagicFire_Draw
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
static mut sTex: [u64_0; 512] =
    [0x144849353c7496b7 as libc::c_longlong as u64_0,
     0xb18e6a462f170702 as libc::c_ulonglong,
     0x7193c4d618cb0ae as libc::c_longlong as u64_0,
     0x896e685931273e21 as libc::c_ulonglong,
     0x204a36355385c9de as libc::c_longlong as u64_0,
     0xb3856f5a492d0f03 as libc::c_ulonglong,
     0x7152c46678baa9d as libc::c_longlong as u64_0,
     0x6d647f856028140b as libc::c_longlong as u64_0,
     0x175b6a4e4a799bb6 as libc::c_longlong as u64_0,
     0xaf946a43250f0403 as libc::c_ulonglong,
     0xc20425b79a9caa8 as libc::c_longlong as u64_0,
     0x6e5b5e522e294d2d as libc::c_longlong as u64_0,
     0x1a4841425b8bd3d8 as libc::c_longlong as u64_0,
     0xae7f61473f290e05 as libc::c_ulonglong,
     0xe213c5a81a0c6ae as libc::c_longlong as u64_0,
     0x7a749c9e61161112 as libc::c_longlong as u64_0,
     0x175c816b5d7fa4b4 as libc::c_longlong as u64_0,
     0xb5a57245200c0206 as libc::c_ulonglong,
     0x142a4b6b94c5dd9f as libc::c_longlong as u64_0,
     0x60454a4727235030 as libc::c_longlong as u64_0,
     0x1946464b628fd1c7 as libc::c_longlong as u64_0,
     0x9c735542301c0906 as libc::c_ulonglong,
     0x162f4a7199bdd9ba as libc::c_longlong as u64_0,
     0x9089b2b87d1d1618 as libc::c_ulonglong,
     0x1a5e90887583a8ad as libc::c_longlong as u64_0,
     0xb2b78550240c030a as libc::c_ulonglong,
     0x1d38597dabdce891 as libc::c_longlong as u64_0,
     0x523637361b18422c as libc::c_longlong as u64_0,
     0x193f44536b8fc2b2 as libc::c_longlong as u64_0,
     0x86624f3c200f070c as libc::c_ulonglong,
     0x203c5b85b0d5e6c2 as libc::c_longlong as u64_0,
     0xa8a3b4b88e25171e as libc::c_ulonglong,
     0x1c5d99a08f8eada3 as libc::c_longlong as u64_0,
     0xacc2955d2f120611 as libc::c_ulonglong,
     0x29476b92c3eeef8f as libc::c_longlong as u64_0,
     0x4d2f2c2a1310342a as libc::c_longlong as u64_0,
     0x1d39435c7892b6a0 as libc::c_longlong as u64_0,
     0x7555513913060914 as libc::c_longlong as u64_0,
     0x2d4b6e9cc9e9ecc4 as libc::c_longlong as u64_0,
     0xbcb4a8a698331621 as libc::c_ulonglong,
     0x215b95aba59db196 as libc::c_longlong as u64_0,
     0x9eb69260391b0c1a as libc::c_ulonglong,
     0x365880aad9f9f4a2 as libc::c_longlong as u64_0,
     0x5b362f2c1a1d3b39 as libc::c_longlong as u64_0,
     0x2f3e4d6b899bb493 as libc::c_longlong as u64_0,
     0x6a535b390b040d1f as libc::c_longlong as u64_0,
     0x3b5c83b4e0f8eabb as libc::c_longlong as u64_0,
     0xc1b59295a7481625 as libc::c_ulonglong,
     0x2a5589aab1acb48c as libc::c_longlong as u64_0,
     0x859174513d251627 as libc::c_ulonglong,
     0x456c97c1ecfef9c0 as libc::c_longlong as u64_0,
     0x8455454342455759 as libc::c_ulonglong,
     0x5157617b97aaae88 as libc::c_longlong as u64_0,
     0x6a636a3a0907152d as libc::c_longlong as u64_0,
     0x4c6f9cccf1ffdca6 as libc::c_longlong as u64_0,
     0xa99d7a8daf5d172b as libc::c_ulonglong,
     0x33517ca0b5bbbc84 as libc::c_longlong as u64_0,
     0x69604834362e2335 as libc::c_longlong as u64_0,
     0x5580aed9f8fffddf as libc::c_longlong as u64_0,
     0xb384696b68616a6e as libc::c_ulonglong,
     0x68767b88a2b4a47d as libc::c_longlong as u64_0,
     0x77837f400d0e203b as libc::c_longlong as u64_0,
     0x5e83b5e1fbffce8d as libc::c_longlong as u64_0,
     0x867b6891b1761c32 as libc::c_ulonglong,
     0x3c537599b5c6bf7e as libc::c_longlong as u64_0,
     0x5036211a2d383548 as libc::c_longlong as u64_0,
     0x6895c4ebfefffef5 as libc::c_longlong as u64_0,
     0xdcad928f81727479 as libc::c_ulonglong,
     0x77919695aab49374 as libc::c_longlong as u64_0,
     0x8aa8964916182c4c as libc::c_ulonglong,
     0x7198cdf2ffffc87e as libc::c_longlong as u64_0,
     0x6a64679caa882838 as libc::c_longlong as u64_0,
     0x485c7898b9cfbb75 as libc::c_longlong as u64_0,
     0x40200d0d29454c5e as libc::c_longlong as u64_0,
     0x7ca8d8f7fffffaf3 as libc::c_longlong as u64_0,
     0xf2c9b0a98d736f7b as libc::c_ulonglong,
     0x81a5ada3b3af806f as libc::c_ulonglong,
     0x9ac6af5d27253d5c as libc::c_ulonglong,
     0x84ace3fcffffd08e as libc::c_ulonglong,
     0x726d7ba49c954142 as libc::c_longlong as u64_0,
     0x5c6c84a1c3d0ab68 as libc::c_longlong as u64_0,
     0x3c200d1030586876 as libc::c_longlong as u64_0,
     0x8fbae2f8fdfff7eb as libc::c_ulonglong,
     0xedccbcb17a5b6681 as libc::c_ulonglong,
     0x89b2c1b4bca67b71 as libc::c_ulonglong,
     0x9dccc68142384c6c as libc::c_ulonglong,
     0x95bbefffffffe1ae as libc::c_ulonglong,
     0x8e818ba4949e625c as libc::c_ulonglong,
     0x778194afcfcb8f59 as libc::c_longlong as u64_0,
     0x43331b1c43728891 as libc::c_longlong as u64_0,
     0xa4c5dce7f3fdf7e5 as libc::c_ulonglong,
     0xdebdb9a86c4f728b as libc::c_ulonglong,
     0x96b9d0c6c29a7e77 as libc::c_ulonglong,
     0x8fb9d7ac674e5d7f as libc::c_ulonglong,
     0xa5bbecfbfefff3ce as libc::c_ulonglong,
     0xb0999599929a706c as libc::c_ulonglong,
     0x8f9ca6c1dfc4774a as libc::c_ulonglong,
     0x4e4e32325d90a6a7 as libc::c_longlong as u64_0,
     0xb3c8cbcde0f5f8e1 as libc::c_ulonglong,
     0xcfb2ac9c6b4b8598 as libc::c_ulonglong,
     0xa8c4dcd7c591827b as libc::c_ulonglong,
     0x7a9fe3d48e667091 as libc::c_longlong as u64_0,
     0xadafd4eaf7fdfae5 as libc::c_ulonglong,
     0xcaaf92868d8e6c71 as libc::c_ulonglong,
     0x98b8bed6edc06943 as libc::c_ulonglong,
     0x5b66474a7fb0b9b5 as libc::c_longlong as u64_0,
     0xbdc7b6afc1e7f6df as libc::c_ulonglong,
     0xc2aaa18e70549ba4 as libc::c_ulonglong,
     0xb5d0e7e6c0857f74 as libc::c_ulonglong,
     0x6384e5f1ab7a81a5 as libc::c_longlong as u64_0,
     0xac99b2cfe6f3f6e4 as libc::c_ulonglong,
     0xc9ac7f6e7f735667 as libc::c_ulonglong,
     0x93d1d7e6f6c96e46 as libc::c_ulonglong,
     0x5e6f565fa1c5c2c2 as libc::c_longlong as u64_0,
     0xc7c3a6959eceefd4 as libc::c_ulonglong,
     0xbaa29783806cadab as libc::c_ulonglong,
     0xbbd6eef1b8797063 as libc::c_ulonglong,
     0x5277def6b78896b8 as libc::c_longlong as u64_0,
     0xa37f8dabcbe5efd8 as libc::c_ulonglong,
     0xc5a7715c6756364b as libc::c_ulonglong,
     0x8ce5e4e9fada8954 as libc::c_ulonglong,
     0x5c675c75bdcec3cd as libc::c_longlong as u64_0,
     0xcebf9d837eb6e3c3 as libc::c_ulonglong,
     0xb69f917e8d8bb4b3 as libc::c_ulonglong,
     0xcae2f5f7af6b594c as libc::c_ulonglong,
     0x4975d5ebb593a9c7 as libc::c_longlong as u64_0,
     0x976d6d88afd7e0c6 as libc::c_ulonglong,
     0xb7a0674f52412739 as libc::c_ulonglong,
     0x8ff2e7d9f3ebaf6e as libc::c_ulonglong,
     0x5c5a5f89cfcbbfd6 as libc::c_longlong as u64_0,
     0xcfb6957666a0d3b0 as libc::c_ulonglong,
     0xb49a878399a8aeac as libc::c_ulonglong,
     0xd4eefafbab624439 as libc::c_ulonglong,
     0x4675caddac9cbbc9 as libc::c_longlong as u64_0,
     0x89605e76a0cfcdb2 as libc::c_ulonglong,
     0xa9955f4747382233 as libc::c_ulonglong,
     0xa2f9e0bedcf3d491 as libc::c_ulonglong,
     0x6053699dd1bebdd9 as libc::c_longlong as u64_0,
     0xc4a183685c94c9a5 as libc::c_ulonglong,
     0xb596768aa5b29493 as libc::c_ulonglong,
     0xd4f8fdfdb1643a31 as libc::c_ulonglong,
     0x477bc7d1a8a9c0bc as libc::c_longlong as u64_0,
     0x7c5b5d7ba5cab89b as libc::c_longlong as u64_0,
     0x9c925f484839203c as libc::c_ulonglong,
     0xc0fdd9a2bae3ebba as libc::c_ulonglong,
     0x755d7fb4cab0b9ce as libc::c_longlong as u64_0,
     0xa87e67575d99c6a6 as libc::c_ulonglong,
     0xb5956380979d6a6b as libc::c_ulonglong,
     0xc8fefffec4744337 as libc::c_ulonglong,
     0x4c7ecacbadb1b3a1 as libc::c_longlong as u64_0,
     0x6f5e6f98b6c0a588 as libc::c_longlong as u64_0,
     0x96976d52513b1d50 as libc::c_ulonglong,
     0xdaffda8c93c1e5db as libc::c_ulonglong,
     0x92719dc0bda7afb5 as libc::c_ulonglong,
     0x825949486ea6c2ab as libc::c_ulonglong,
     0xb48d4f6b7e774042 as libc::c_ulonglong,
     0xb1feffffdb955d4b as libc::c_ulonglong,
     0x5585d0c8b1b09d7f as libc::c_longlong as u64_0,
     0x626890babfb59a7b as libc::c_longlong as u64_0,
     0x959d83605737196a as libc::c_ulonglong,
     0xecffdd7d729cc9ed as libc::c_ulonglong,
     0xb389b7c2b0a2a396 as libc::c_ulonglong,
     0x5f3c2f3f86b0c0b2 as libc::c_longlong as u64_0,
     0xab7f3f4c5e4c2329 as libc::c_ulonglong,
     0x93f8ffffeebd886c as libc::c_ulonglong,
     0x6793d4c4b2a67e62 as libc::c_longlong as u64_0,
     0x5472b2d1bfa39170 as libc::c_longlong as u64_0,
     0x96a7956c562e1a86 as libc::c_ulonglong,
     0xf8ffd66b6383a5ec as libc::c_ulonglong,
     0xce9fc6bca7a29278 as libc::c_ulonglong,
     0x4a2f24439dafb4ae as libc::c_longlong as u64_0,
     0x9369362e3a2d191f as libc::c_ulonglong,
     0x6fe9fffffcdfb899 as libc::c_longlong as u64_0,
     0x88a8cfb9ac94674e as libc::c_ulonglong,
     0x4575cbd8b3928465 as libc::c_longlong as u64_0,
     0x899f9771522421a3 as libc::c_ulonglong,
     0xfaf8b352698886da as libc::c_ulonglong,
     0xdfaec9ada3a58260 as libc::c_ulonglong,
     0x483c2f54ab9f9b96 as libc::c_longlong as u64_0,
     0x74502e212d221e28 as libc::c_longlong as u64_0,
     0x4ac7fffffbeddbc4 as libc::c_longlong as u64_0,
     0xafbcc6aca2805943 as libc::c_ulonglong,
     0x366cd3d0ab8f7a5b as libc::c_longlong as u64_0,
     0x708282694c1d2bb4 as libc::c_longlong as u64_0,
     0xede1833877a175c6 as libc::c_ulonglong,
     0xe9b8bf9f9ea3714f as libc::c_ulonglong,
     0x5458466da986817a as libc::c_longlong as u64_0,
     0x563e281c2d21283f as libc::c_longlong as u64_0,
     0x319dfcfff3dbebe5 as libc::c_longlong as u64_0,
     0xd2cab99f96714c3a as libc::c_ulonglong,
     0x2854c4c7a8917a56 as libc::c_longlong as u64_0,
     0x5763615b481833b4 as libc::c_longlong as u64_0,
     0xcfba512281b876bb as libc::c_ulonglong,
     0xe8bab09193965b3e as libc::c_ulonglong,
     0x5b715d81a1746a5d as libc::c_longlong as u64_0,
     0x4439221b2a1f2d51 as libc::c_longlong as u64_0,
     0x296cf9ffeab9daea as libc::c_longlong as u64_0,
     0xe9d0ab9288643f30 as libc::c_ulonglong,
     0x1c3ba9bd9d897e60 as libc::c_longlong as u64_0,
     0x3f4041504e1a309f as libc::c_longlong as u64_0,
     0xa08b2b137aba84c4 as libc::c_ulonglong,
     0xe4b6a385807d4730 as libc::c_ulonglong,
     0x557665849b78674b as libc::c_longlong as u64_0,
     0x3c392124271a2953 as libc::c_longlong as u64_0,
     0x2c41f3ffe59bb7d0 as libc::c_longlong as u64_0,
     0xe1cd9d8578563425 as libc::c_ulonglong,
     0x142284aa8e7f7f74 as libc::c_longlong as u64_0,
     0x3d293150602d2872 as libc::c_longlong as u64_0,
     0x68601c0b5ba2a0d1 as libc::c_longlong as u64_0,
     0xdbb49e826e633925 as libc::c_ulonglong,
     0x405d557285797452 as libc::c_longlong as u64_0,
     0x3733304836171e41 as libc::c_longlong as u64_0,
     0x332ae9ffe88d91a3 as libc::c_longlong as u64_0,
     0xbfc98f7564482a1b as libc::c_ulonglong,
     0x10105d8f7b6e757f as libc::c_longlong as u64_0,
     0x593a3c6381542e46 as libc::c_longlong as u64_0,
     0x3a441a073776b9dc as libc::c_longlong as u64_0,
     0xd1b5a3886a583926 as libc::c_ulonglong,
     0x2a393b53696e7968 as libc::c_longlong as u64_0,
     0x342b4f8061211229 as libc::c_longlong as u64_0,
     0x341cddfff09d6e6f as libc::c_longlong as u64_0,
     0x96c0836651392112 as libc::c_ulonglong,
     0xc083c6d6b5c6283 as libc::c_longlong as u64_0,
     0x7860587da5894c2b as libc::c_longlong as u64_0,
     0x1d3a1c061a4bcadd as libc::c_longlong as u64_0,
     0xcabdae9679634a31 as libc::c_ulonglong,
     0x211b20384d607e84 as libc::c_longlong as u64_0,
     0x342b7cb79b3e0d1a as libc::c_longlong as u64_0,
     0x2e1ccafff9b45944 as libc::c_longlong as u64_0,
     0x6cb8805a452f1a0c as libc::c_longlong as u64_0,
     0x9042454685b547e as libc::c_longlong as u64_0,
     0x9b8c738fb9b67a28 as libc::c_ulonglong,
     0x123d20050c2ed3cf as libc::c_longlong as u64_0,
     0xc5bfb9a190816642 as libc::c_ulonglong,
     0x210b162b2f497e9e as libc::c_longlong as u64_0,
     0x4649b0e5d4620f19 as libc::c_longlong as u64_0,
     0x2a1faffefeca4c28 as libc::c_longlong as u64_0,
     0x4ca9855a4835190a as libc::c_longlong as u64_0,
     0x70418476d665779 as libc::c_longlong as u64_0,
     0xbaa77e87b0bf9c30 as libc::c_ulonglong,
     0x164821050e20cdc2 as libc::c_longlong as u64_0,
     0xc3b6bca0a49e8252 as libc::c_ulonglong,
     0x260a20312a3e81a8 as libc::c_longlong as u64_0,
     0x707dd5faf884141f as libc::c_longlong as u64_0,
     0x29248bf9ffd8421a as libc::c_longlong as u64_0,
     0x3e99926d624e2c11 as libc::c_longlong as u64_0,
     0x906184774776a89 as libc::c_longlong as u64_0,
     0xb5986b66849d982f as libc::c_ulonglong,
     0x21581f07131bc7bb as libc::c_longlong as u64_0,
     0xbda4b596aeaf985e as libc::c_ulonglong,
     0x240f3b47334791a5 as libc::c_longlong as u64_0,
     0x9cadedfcff9e1821 as libc::c_ulonglong,
     0x272a61ecfed53516 as libc::c_longlong as u64_0,
     0x38929f837d734d22 as libc::c_longlong as u64_0,
     0xc081d4c7d8a84a3 as libc::c_longlong as u64_0,
     0x9c6e463d4e657227 as libc::c_ulonglong,
     0x2e621e08161ac1b9 as libc::c_longlong as u64_0,
     0xb18ea986acb3a764 as libc::c_ulonglong,
     0x1f135a664457a29c as libc::c_longlong as u64_0,
     0xc4d7f8feffb0171e as libc::c_ulonglong,
     0x24283ecbf5c02a19 as libc::c_longlong as u64_0,
     0x3c8ea79594967239 as libc::c_longlong as u64_0,
     0x120c2351839b9fb6 as libc::c_longlong as u64_0,
     0x7c3c231a222e3f19 as libc::c_longlong as u64_0,
     0x3a5e1b09171ab7bb as libc::c_longlong as u64_0,
     0xa47fa17ba9acb36c as libc::c_ulonglong,
     0x201c76885b6cad94 as libc::c_longlong as u64_0,
     0xdfeafbfffcc4161a as libc::c_ulonglong,
     0x1b2324a0cd942420 as libc::c_longlong as u64_0,
     0x4d99aba1a5af8d4c as libc::c_longlong as u64_0,
     0x1d152a5585a2b2c9 as libc::c_longlong as u64_0,
     0x59190c070a0f1814 as libc::c_longlong as u64_0,
     0x3a4c1a0c161aa9be as libc::c_longlong as u64_0,
     0x987b9e7ea8a6bc79 as libc::c_ulonglong,
     0x2b258aa27b8ab498 as libc::c_longlong as u64_0,
     0xe5f0fef6f1d81619 as libc::c_ulonglong,
     0x1318186f9269302a as libc::c_longlong as u64_0,
     0x639daba5a6ac8a54 as libc::c_longlong as u64_0,
     0x2d24315381a1b8d1 as libc::c_longlong as u64_0,
     0x4e0f0508160e071a as libc::c_longlong as u64_0,
     0x2f311b10161a99bf as libc::c_longlong as u64_0,
     0x947ba18ca6a5c789 as libc::c_ulonglong,
     0x3d2b8eb29babbfab as libc::c_longlong as u64_0,
     0xe7f8fde7e5e91819 as libc::c_ulonglong,
     0xd0d18435a534c35 as libc::c_longlong as u64_0,
     0x7798aaaba0957456 as libc::c_longlong as u64_0,
     0x46403e557d9db9d6 as libc::c_longlong as u64_0,
     0x611408163b200420 as libc::c_longlong as u64_0,
     0x201b19161a1f87be as libc::c_longlong as u64_0,
     0x9882a59da6abd49c as libc::c_ulonglong,
     0x513587bbb7c8d0c9 as libc::c_longlong as u64_0,
     0xecfefcdadeec181b as libc::c_ulonglong,
     0xb0816282d4f6e41 as libc::c_longlong as u64_0,
     0x858eadb4987a5d58 as libc::c_ulonglong,
     0x6561505b7c9cb7de as libc::c_longlong as u64_0,
     0x7f1e1433733f061e as libc::c_longlong as u64_0,
     0x1310181a212875ba as libc::c_longlong as u64_0,
     0x9f8ea4a7abbadcab as libc::c_ulonglong,
     0x653e7db6c7dadde2 as libc::c_longlong as u64_0,
     0xf1fffcd9e1e0161a as libc::c_ulonglong,
     0xa0715181e58894d as libc::c_longlong as u64_0,
     0x8984adb98c5d5165 as libc::c_ulonglong,
     0x8a85636684a1afdb as libc::c_ulonglong,
     0x9f2c3066af630b17 as libc::c_ulonglong,
     0x1513161b2f3766b3 as libc::c_longlong as u64_0,
     0x9f9a9fa8b5c4dbb5 as libc::c_ulonglong,
     0x734b72abc5dbe6f5 as libc::c_longlong as u64_0,
     0xf7fffce2eac11118 as libc::c_ulonglong,
     0xb0e161421579157 as libc::c_longlong as u64_0,
     0x868da7b9814f5e89 as libc::c_ulonglong,
     0xb3a5757592a4a4d3 as libc::c_ulonglong,
     0xb64060a0dc851a16 as libc::c_ulonglong,
     0x2b20141a434a5fac as libc::c_longlong as u64_0,
     0x96a497a3c2c6d3b7 as libc::c_ulonglong,
     0x7e596ea1bed7e7f9 as libc::c_longlong as u64_0,
     0xfdfffef1ee980c12 as libc::c_ulonglong,
     0xf1d1c15284a8d64 as libc::c_longlong as u64_0,
     0x829ba2b87e4e77b1 as libc::c_ulonglong,
     0xd4ba8283a2a392c6 as libc::c_ulonglong,
     0xc75899d1f6a13b29 as libc::c_ulonglong,
     0x4e30161e56585aa5 as libc::c_longlong as u64_0,
     0x8ba893a4d0c7c8b5 as libc::c_ulonglong,
     0x83647198b9d3daeb as libc::c_ulonglong,
     0xfefffffef07d1315 as libc::c_ulonglong,
     0x1e332a19303d8271 as libc::c_longlong as u64_0,
     0x81a99ebb815191d3 as libc::c_ulonglong,
     0xedca8c91ac9a7eba as libc::c_ulonglong,
     0xd57dcbf2febb7157 as libc::c_ulonglong,
     0x7a40212a645d5da0 as libc::c_longlong as u64_0,
     0x82ab97afdbccc5b0 as libc::c_ulonglong,
     0x816d7a9abbd4cbd5 as libc::c_ulonglong,
     0xfcffffffef853e34 as libc::c_ulonglong,
     0x3f4c371e3a3a7e7f as libc::c_longlong as u64_0,
     0x80b39ec183559ce5 as libc::c_ulonglong,
     0xf7d1989eb19077bb as libc::c_ulonglong,
     0xe3abedfefed4ab95 as libc::c_ulonglong,
     0xa050364369576c9f as libc::c_ulonglong,
     0x89aea2c3e7dad0ad as libc::c_ulonglong,
     0x7e7488a7c8dcb8b7 as libc::c_longlong as u64_0,
     0xf6fffffff1a77b6c as libc::c_ulonglong,
     0x6d613e22403f878c as libc::c_longlong as u64_0,
     0x81b6a8cd825396da as libc::c_ulonglong,
     0xf1d4a4acb5958bca as libc::c_ulonglong,
     0xf1d2fcffffead8c6 as libc::c_ulonglong,
     0xb86258676a4f73a3 as libc::c_ulonglong,
     0x9ab1b0d9f2eddeab as libc::c_ulonglong,
     0x7b7d95b7d9e39d96 as libc::c_longlong as u64_0,
     0xeffffffef6ceaf9d as libc::c_ulonglong,
     0x9b6b40273f489297 as libc::c_ulonglong,
     0x8abcb7d9825286c3 as libc::c_ulonglong,
     0xe2d3afb6b6a5afdd as libc::c_ulonglong,
     0xf9f0fefffff9eedf as libc::c_ulonglong,
     0xc87d818a714d77a3 as libc::c_ulonglong,
     0xa8b6bfeaf9fbecab as libc::c_ulonglong,
     0x7d87a4c2dfd88177 as libc::c_longlong as u64_0,
     0xe1fefffcf7e7d6c2 as libc::c_ulonglong,
     0xb9683c2d40599e9f as libc::c_ulonglong,
     0x91c1c6e7895878ab as libc::c_ulonglong,
     0xd0ccb7bab4b4d2f2 as libc::c_ulonglong,
     0xfbf3f3fdfffeede0 as libc::c_ulonglong,
     0xca999d9f815c7ca0 as libc::c_ulonglong,
     0xb1bccff7fffff2ae as libc::c_ulonglong,
     0x8796adbdcfbb6359 as libc::c_ulonglong,
     0xc4f6fffcf6e8d8c7 as libc::c_ulonglong,
     0xc35f3c3b4d739da3 as libc::c_ulonglong,
     0x9ec5d6f39763779d as libc::c_ulonglong,
     0xbfc5beb8a7b4eafc as libc::c_ulonglong,
     0xebd7dbf6fffce2d2 as libc::c_ulonglong,
     0xc4a4a4a0886c82a2 as libc::c_ulonglong,
     0xb2c6e0fdfffff2b2 as libc::c_ulonglong,
     0x97a3a9a7a98d433e as libc::c_ulonglong,
     0x9ae2fefcf5dbc5b2 as libc::c_ulonglong,
     0xb752425469828ea0 as libc::c_ulonglong,
     0xaecfe5f9b07d8ba0 as libc::c_ulonglong,
     0xb4c0c7b28d99def4 as libc::c_ulonglong,
     0xcfb2b8e2fcf8dcc9 as libc::c_ulonglong,
     0xb99d948b806b779e as libc::c_ulonglong,
     0xb7d5eefffffff0b9 as libc::c_ulonglong,
     0xa6a69b8677592726 as libc::c_ulonglong,
     0x6dc6fcfef5d2b6a6 as libc::c_longlong as u64_0,
     0xa1454b6f7e827a99 as libc::c_ulonglong,
     0xbddcf2fecea1adae as libc::c_ulonglong,
     0xb3c3d6ac6b6ebddd as libc::c_ulonglong,
     0xae8992cbf3f1dcc4 as libc::c_ulonglong,
     0xa1827269665c6593 as libc::c_ulonglong,
     0xc0e4f9ffffffecbe as libc::c_ulonglong,
     0xada1846449301416 as libc::c_ulonglong,
     0x45a4f9fff8cda99c as libc::c_longlong as u64_0,
     0x9040548a8a756693 as libc::c_ulonglong,
     0xceebfaffe5c6cebc as libc::c_ulonglong,
     0xb5cce7a54b4494be as libc::c_ulonglong,
     0x8d6d77b4e6e4dabc as libc::c_ulonglong,
     0x885f5248403e4d80 as libc::c_ulonglong,
     0xc4effffeffffe7c0 as libc::c_ulonglong,
     0xab926c492b17090d as libc::c_ulonglong,
     0x2c85f3fff5c59c94 as libc::c_longlong as u64_0,
     0x85415b988d626096 as libc::c_ulonglong,
     0xdff9fffff7e5e4c1 as libc::c_ulonglong,
     0xb5d8f4a133246b9e as libc::c_ulonglong,
     0x79626ba2d4d1d1ac as libc::c_longlong as u64_0,
     0x6d473a2d25242e64 as libc::c_longlong as u64_0,
     0xb7f1fffdffffdebb as libc::c_ulonglong,
     0x9f7e5d3b1c0b040b as libc::c_ulonglong,
     0x256feefeefbd928d as libc::c_longlong as u64_0,
     0x8a4d5d968c6268a9 as libc::c_ulonglong,
     0xecfefffffef7eabb as libc::c_ulonglong,
     0xacddfa9724165285 as libc::c_ulonglong,
     0x73656fa0c4b9bd94 as libc::c_longlong as u64_0,
     0x563b2b1c161d294e as libc::c_longlong as u64_0,
     0x9eeafffdfffad2b0 as libc::c_ulonglong,
     0x906f53321506040f as libc::c_ulonglong,
     0x2663e4f8e7b58986 as libc::c_longlong as u64_0,
     0x955e5f8e97737cc4 as libc::c_ulonglong,
     0xf7fffffffff8ddaa as libc::c_ulonglong,
     0x9aceee891b0e4078 as libc::c_ulonglong,
     0x77727ca4b8a2a078 as libc::c_longlong as u64_0,
     0x463624120e22334c as libc::c_longlong as u64_0,
     0x88e2fffefff1bfa0 as libc::c_ulonglong,
     0x81644c2d12050715 as libc::c_ulonglong,
     0x2e61d7ead9ab8582 as libc::c_longlong as u64_0,
     0x9f736486ab8d98e0 as libc::c_ulonglong,
     0xfefdffffffeec896 as libc::c_ulonglong,
     0x81adcd72150b3673 as libc::c_ulonglong,
     0x83858dabaa8c805e as libc::c_ulonglong,
     0x3b33210c0b2a445d as libc::c_longlong as u64_0,
     0x86e0fffffce3ac8d as libc::c_ulonglong,
     0x735d472b14090c1f as libc::c_longlong as u64_0,
     0x3b67cbd5c39f8281 as libc::c_longlong as u64_0,
     0xa2896d86bca8b7f3 as libc::c_ulonglong,
     0xfdfafffffbdcb281 as libc::c_ulonglong,
     0x667f9c540f0b3575 as libc::c_longlong as u64_0,
     0x999b9db19d7b664a as libc::c_ulonglong,
     0x352d1c080b335a7c as libc::c_longlong as u64_0,
     0xa5e7fffff3cf9979 as libc::c_ulonglong,
     0x6558402a1e14162b as libc::c_longlong as u64_0,
     0x4975bdbaaa918184 as libc::c_longlong as u64_0,
     0x9f9b7b90c8bfd3fd as libc::c_ulonglong,
     0xfefafffef1c59a6d as libc::c_ulonglong,
     0x4f5064370a0c357e as libc::c_longlong as u64_0,
     0xb4b1abb28d6c533f as libc::c_ulonglong,
     0x2e2415050c396fa3 as libc::c_longlong as u64_0,
     0xc9f2fffbe4b98565 as libc::c_ulonglong,
     0x57543b2b31272139 as libc::c_longlong as u64_0,
     0x5d83b6a38f7f7c83 as libc::c_longlong as u64_0,
     0x99a88fa0d3d1e8ff as libc::c_ulonglong,
     0xfefafff9dfad855c as libc::c_ulonglong,
     0x3c2d361f08113b88 as libc::c_longlong as u64_0,
     0xc9bdb3ae7c5e4738 as libc::c_ulonglong,
     0x27190d030d3c85c8 as libc::c_longlong as u64_0,
     0xeafcfef3cea17150 as libc::c_ulonglong,
     0x484e372f453a2f4b as libc::c_longlong as u64_0,
     0x6f95b391786a6d7d as libc::c_longlong as u64_0,
     0x93b1a4b3d7dcf6ff as libc::c_ulonglong,
     0xfefcfeedc995714b as libc::c_ulonglong,
     0x2c1818130a1a4396 as libc::c_longlong as u64_0,
     0xd4beb2a66d514235 as libc::c_ulonglong,
     0x1f0e07040f419ee6 as libc::c_longlong as u64_0,
     0xfdfffae3b4885e3f as libc::c_ulonglong,
     0x39463435574a3d5d as libc::c_longlong as u64_0,
     0x82a6af8667555668 as libc::c_ulonglong,
     0x8db2aeb5cfe2fdff as libc::c_ulonglong,
     0xfffff7dab07f603e as libc::c_ulonglong,
     0x1f0d0b0f11264ea0 as libc::c_longlong as u64_0,
     0xd1b5ab9b664d4636 as libc::c_ulonglong,
     0x1608050b1143aff7 as libc::c_longlong as u64_0,
     0xfffff1cd9c724c2f as libc::c_ulonglong,
     0x2b3a353e5f534d71 as libc::c_longlong as u64_0,
     0x98b3a77d5d49404a as libc::c_ulonglong,
     0x85aea5a1bae3ffff as libc::c_ulonglong,
     0xfffce8c3976b4e30 as libc::c_ulonglong,
     0x150707131d385da3 as libc::c_longlong as u64_0,
     0xc3a69d9268585436 as libc::c_ulonglong,
     0xf0905141546bffe as libc::c_longlong as u64_0,
     0xfffde2b685603c22 as libc::c_ulonglong,
     0x1d2d36475e555d85 as libc::c_longlong as u64_0,
     0xabba9c745943332e as libc::c_ulonglong,
     0x7aa78a7f9cdaffff as libc::c_longlong as u64_0,
     0xfff1d5b184583e25 as libc::c_ulonglong,
     0xd04081b2c4a6daa as libc::c_longlong as u64_0,
     0xb4968c8a766e6431 as libc::c_ulonglong,
     0xb0d071c1b4fc8fe as libc::c_longlong as u64_0,
     0xfff4cd9f71502f17 as libc::c_ulonglong,
     0x1221384d58586f9a as libc::c_longlong as u64_0,
     0xbfbc906e54412e19 as libc::c_ulonglong,
     0x6a9664537ccefdff as libc::c_longlong as u64_0,
     0xfce3c1a879492d1b as libc::c_ulonglong,
     0x8030d243c5c7fac as libc::c_longlong as u64_0,
     0xa6887d868c896b26 as libc::c_ulonglong,
     0x90f08202256bbee as libc::c_longlong as u64_0,
     0xedd7ab805d41200d as libc::c_ulonglong,
     0xb19394b4f5a7ba8 as libc::c_longlong as u64_0,
     0xc7ac80664a382c0e as libc::c_ulonglong,
     0x567e413260b2e5f3 as libc::c_longlong as u64_0,
     0xe4c3aa9d6b3a1e12 as libc::c_ulonglong,
     0x5040f28426983a1 as libc::c_longlong as u64_0,
     0x8f766b7f958f5d19 as libc::c_ulonglong,
     0xd2517222960acdb as libc::c_longlong as u64_0,
     0xd5b890694c321606 as libc::c_ulonglong,
     0x6153a494b6589ae as libc::c_longlong as u64_0,
     0xb99d7a5f3f2f2b0e as libc::c_ulonglong,
     0x40652e255296d3ec as libc::c_longlong as u64_0,
     0xd5a897896235180b as libc::c_ulonglong,
     0x307172d4c6f8d9f as libc::c_longlong as u64_0,
     0x7c6b626f7f72410e as libc::c_longlong as u64_0,
     0x11372f29326da1c8 as libc::c_longlong as u64_0,
     0xc3a07c573d240e03 as libc::c_ulonglong,
     0x5153a4953769cb4 as libc::c_longlong as u64_0,
     0xa688745d37293316 as libc::c_ulonglong,
     0x2e542d294e8bcde8 as libc::c_longlong as u64_0,
     0xc697847459321306 as libc::c_ulonglong,
     0x40d2039597d9b9d as libc::c_longlong as u64_0,
     0x72646b746c4d2508 as libc::c_longlong as u64_0];
static mut sMaterialDL: [Gfx; 5] =
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
                                (0xfc as libc::c_int as u32_0 &
                                     (((0x1 as libc::c_int) <<
                                           8 as libc::c_int) -
                                          1 as libc::c_int) as libc::c_uint)
                                    << 24 as libc::c_int |
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
                                               (0 as libc::c_int as u32_0 &
                                                    (((0x1 as libc::c_int) <<
                                                          5 as libc::c_int) -
                                                         1 as libc::c_int) as
                                                        libc::c_uint) <<
                                                   0 as libc::c_int)) &
                                         (((0x1 as libc::c_int) <<
                                               24 as libc::c_int) -
                                              1 as libc::c_int) as
                                             libc::c_uint) <<
                                        0 as libc::c_int,
                            w1:
                                (3 as libc::c_int as u32_0 &
                                     (((0x1 as libc::c_int) <<
                                           4 as libc::c_int) -
                                          1 as libc::c_int) as libc::c_uint)
                                    << 28 as libc::c_int |
                                    (1 as libc::c_int as u32_0 &
                                         (((0x1 as libc::c_int) <<
                                               3 as libc::c_int) -
                                              1 as libc::c_int) as
                                             libc::c_uint) <<
                                        15 as libc::c_int |
                                    (6 as libc::c_int as u32_0 &
                                         (((0x1 as libc::c_int) <<
                                               3 as libc::c_int) -
                                              1 as libc::c_int) as
                                             libc::c_uint) <<
                                        12 as libc::c_int |
                                    (1 as libc::c_int as u32_0 &
                                         (((0x1 as libc::c_int) <<
                                               3 as libc::c_int) -
                                              1 as libc::c_int) as
                                             libc::c_uint) << 9 as libc::c_int
                                    |
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
                                          1 as libc::c_int) as libc::c_uint)
                                    << 24 as libc::c_int |
                                    ((32 as libc::c_int - 3 as libc::c_int -
                                          29 as libc::c_int) as u32_0 &
                                         (((0x1 as libc::c_int) <<
                                               8 as libc::c_int) -
                                              1 as libc::c_int) as
                                             libc::c_uint) << 8 as libc::c_int
                                    |
                                    ((29 as libc::c_int - 1 as libc::c_int) as
                                         u32_0 &
                                         (((0x1 as libc::c_int) <<
                                               8 as libc::c_int) -
                                              1 as libc::c_int) as
                                             libc::c_uint) <<
                                        0 as libc::c_int,
                            w1:
                                ((0 as libc::c_int) << 30 as libc::c_int |
                                     (3 as libc::c_int) << 26 as libc::c_int |
                                     (0 as libc::c_int) << 22 as libc::c_int |
                                     (2 as libc::c_int) << 18 as libc::c_int |
                                     (0x8 as libc::c_int | 0x10 as libc::c_int
                                          | 0x40 as libc::c_int |
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
                                          1 as libc::c_int) as libc::c_uint)
                                    << 24 as libc::c_int |
                                    (!((0x400 as libc::c_int |
                                            0x10000 as libc::c_int |
                                            0x20000 as libc::c_int |
                                            0x40000 as libc::c_int |
                                            0x80000 as libc::c_int) as u32_0)
                                         &
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
                                (0xdf as libc::c_int as u32_0 &
                                     (((0x1 as libc::c_int) <<
                                           8 as libc::c_int) -
                                          1 as libc::c_int) as libc::c_uint)
                                    << 24 as libc::c_int,
                            w1: 0 as libc::c_int as libc::c_uint,};
                 init
             },}];
// Initialized in run_static_initializers
static mut sModelDL: [Gfx; 45] = [Gfx{words: Gwords{w0: 0, w1: 0,},}; 45];
static mut sSphereVtx: [Vtx; 76] =
    [Vtx{v:
             {
                 let mut init =
                     Vtx_t{ob:
                               [-(707 as libc::c_int) as libc::c_short,
                                1732 as libc::c_int as libc::c_short,
                                707 as libc::c_int as libc::c_short],
                           flag: 0 as libc::c_int as libc::c_ushort,
                           tc:
                               [1792 as libc::c_int as libc::c_short,
                                137 as libc::c_int as libc::c_short],
                           cn:
                               [255 as libc::c_int as libc::c_uchar,
                                255 as libc::c_int as libc::c_uchar,
                                255 as libc::c_int as libc::c_uchar,
                                76 as libc::c_int as libc::c_uchar],};
                 init
             },},
     Vtx{v:
             {
                 let mut init =
                     Vtx_t{ob:
                               [0 as libc::c_int as libc::c_short,
                                1732 as libc::c_int as libc::c_short,
                                1000 as libc::c_int as libc::c_short],
                           flag: 0 as libc::c_int as libc::c_ushort,
                           tc:
                               [2048 as libc::c_int as libc::c_short,
                                137 as libc::c_int as libc::c_short],
                           cn:
                               [255 as libc::c_int as libc::c_uchar,
                                255 as libc::c_int as libc::c_uchar,
                                255 as libc::c_int as libc::c_uchar,
                                76 as libc::c_int as libc::c_uchar],};
                 init
             },},
     Vtx{v:
             {
                 let mut init =
                     Vtx_t{ob:
                               [0 as libc::c_int as libc::c_short,
                                2000 as libc::c_int as libc::c_short,
                                0 as libc::c_int as libc::c_short],
                           flag: 0 as libc::c_int as libc::c_ushort,
                           tc:
                               [1920 as libc::c_int as libc::c_short,
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
                               [-(663 as libc::c_int) as libc::c_short,
                                1000 as libc::c_int as libc::c_short,
                                1600 as libc::c_int as libc::c_short],
                           flag: 0 as libc::c_int as libc::c_ushort,
                           tc:
                               [1920 as libc::c_int as libc::c_short,
                                512 as libc::c_int as libc::c_short],
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
                               [663 as libc::c_int as libc::c_short,
                                1000 as libc::c_int as libc::c_short,
                                1600 as libc::c_int as libc::c_short],
                           flag: 0 as libc::c_int as libc::c_ushort,
                           tc:
                               [2176 as libc::c_int as libc::c_short,
                                512 as libc::c_int as libc::c_short],
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
                                2000 as libc::c_int as libc::c_short],
                           flag: 0 as libc::c_int as libc::c_ushort,
                           tc:
                               [2048 as libc::c_int as libc::c_short,
                                1024 as libc::c_int as libc::c_short],
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
                                2000 as libc::c_int as libc::c_short],
                           flag: 0 as libc::c_int as libc::c_ushort,
                           tc:
                               [0 as libc::c_int as libc::c_short,
                                1024 as libc::c_int as libc::c_short],
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
                               [1414 as libc::c_int as libc::c_short,
                                0 as libc::c_int as libc::c_short,
                                1414 as libc::c_int as libc::c_short],
                           flag: 0 as libc::c_int as libc::c_ushort,
                           tc:
                               [256 as libc::c_int as libc::c_short,
                                1024 as libc::c_int as libc::c_short],
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
                               [663 as libc::c_int as libc::c_short,
                                1000 as libc::c_int as libc::c_short,
                                1600 as libc::c_int as libc::c_short],
                           flag: 0 as libc::c_int as libc::c_ushort,
                           tc:
                               [128 as libc::c_int as libc::c_short,
                                512 as libc::c_int as libc::c_short],
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
                               [663 as libc::c_int as libc::c_short,
                                -(1000 as libc::c_int) as libc::c_short,
                                1600 as libc::c_int as libc::c_short],
                           flag: 0 as libc::c_int as libc::c_ushort,
                           tc:
                               [128 as libc::c_int as libc::c_short,
                                1536 as libc::c_int as libc::c_short],
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
                               [1600 as libc::c_int as libc::c_short,
                                -(1000 as libc::c_int) as libc::c_short,
                                663 as libc::c_int as libc::c_short],
                           flag: 0 as libc::c_int as libc::c_ushort,
                           tc:
                               [384 as libc::c_int as libc::c_short,
                                1536 as libc::c_int as libc::c_short],
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
                               [707 as libc::c_int as libc::c_short,
                                -(1732 as libc::c_int) as libc::c_short,
                                707 as libc::c_int as libc::c_short],
                           flag: 0 as libc::c_int as libc::c_ushort,
                           tc:
                               [256 as libc::c_int as libc::c_short,
                                1911 as libc::c_int as libc::c_short],
                           cn:
                               [255 as libc::c_int as libc::c_uchar,
                                255 as libc::c_int as libc::c_uchar,
                                255 as libc::c_int as libc::c_uchar,
                                76 as libc::c_int as libc::c_uchar],};
                 init
             },},
     Vtx{v:
             {
                 let mut init =
                     Vtx_t{ob:
                               [1000 as libc::c_int as libc::c_short,
                                -(1732 as libc::c_int) as libc::c_short,
                                0 as libc::c_int as libc::c_short],
                           flag: 0 as libc::c_int as libc::c_ushort,
                           tc:
                               [512 as libc::c_int as libc::c_short,
                                1911 as libc::c_int as libc::c_short],
                           cn:
                               [255 as libc::c_int as libc::c_uchar,
                                255 as libc::c_int as libc::c_uchar,
                                255 as libc::c_int as libc::c_uchar,
                                76 as libc::c_int as libc::c_uchar],};
                 init
             },},
     Vtx{v:
             {
                 let mut init =
                     Vtx_t{ob:
                               [0 as libc::c_int as libc::c_short,
                                -(2000 as libc::c_int) as libc::c_short,
                                0 as libc::c_int as libc::c_short],
                           flag: 0 as libc::c_int as libc::c_ushort,
                           tc:
                               [384 as libc::c_int as libc::c_short,
                                2048 as libc::c_int as libc::c_short],
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
                               [-(1000 as libc::c_int) as libc::c_short,
                                1732 as libc::c_int as libc::c_short,
                                0 as libc::c_int as libc::c_short],
                           flag: 0 as libc::c_int as libc::c_ushort,
                           tc:
                               [1536 as libc::c_int as libc::c_short,
                                137 as libc::c_int as libc::c_short],
                           cn:
                               [255 as libc::c_int as libc::c_uchar,
                                255 as libc::c_int as libc::c_uchar,
                                255 as libc::c_int as libc::c_uchar,
                                76 as libc::c_int as libc::c_uchar],};
                 init
             },},
     Vtx{v:
             {
                 let mut init =
                     Vtx_t{ob:
                               [0 as libc::c_int as libc::c_short,
                                2000 as libc::c_int as libc::c_short,
                                0 as libc::c_int as libc::c_short],
                           flag: 0 as libc::c_int as libc::c_ushort,
                           tc:
                               [1664 as libc::c_int as libc::c_short,
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
                               [-(1600 as libc::c_int) as libc::c_short,
                                1000 as libc::c_int as libc::c_short,
                                663 as libc::c_int as libc::c_short],
                           flag: 0 as libc::c_int as libc::c_ushort,
                           tc:
                               [1664 as libc::c_int as libc::c_short,
                                512 as libc::c_int as libc::c_short],
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
                               [-(1414 as libc::c_int) as libc::c_short,
                                0 as libc::c_int as libc::c_short,
                                1414 as libc::c_int as libc::c_short],
                           flag: 0 as libc::c_int as libc::c_ushort,
                           tc:
                               [1792 as libc::c_int as libc::c_short,
                                1024 as libc::c_int as libc::c_short],
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
                               [-(663 as libc::c_int) as libc::c_short,
                                -(1000 as libc::c_int) as libc::c_short,
                                1600 as libc::c_int as libc::c_short],
                           flag: 0 as libc::c_int as libc::c_ushort,
                           tc:
                               [1920 as libc::c_int as libc::c_short,
                                1536 as libc::c_int as libc::c_short],
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
                               [663 as libc::c_int as libc::c_short,
                                -(1000 as libc::c_int) as libc::c_short,
                                1600 as libc::c_int as libc::c_short],
                           flag: 0 as libc::c_int as libc::c_ushort,
                           tc:
                               [2176 as libc::c_int as libc::c_short,
                                1536 as libc::c_int as libc::c_short],
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
                                -(1732 as libc::c_int) as libc::c_short,
                                1000 as libc::c_int as libc::c_short],
                           flag: 0 as libc::c_int as libc::c_ushort,
                           tc:
                               [2048 as libc::c_int as libc::c_short,
                                1911 as libc::c_int as libc::c_short],
                           cn:
                               [255 as libc::c_int as libc::c_uchar,
                                255 as libc::c_int as libc::c_uchar,
                                255 as libc::c_int as libc::c_uchar,
                                76 as libc::c_int as libc::c_uchar],};
                 init
             },},
     Vtx{v:
             {
                 let mut init =
                     Vtx_t{ob:
                               [0 as libc::c_int as libc::c_short,
                                -(1732 as libc::c_int) as libc::c_short,
                                1000 as libc::c_int as libc::c_short],
                           flag: 0 as libc::c_int as libc::c_ushort,
                           tc:
                               [0 as libc::c_int as libc::c_short,
                                1911 as libc::c_int as libc::c_short],
                           cn:
                               [255 as libc::c_int as libc::c_uchar,
                                255 as libc::c_int as libc::c_uchar,
                                255 as libc::c_int as libc::c_uchar,
                                76 as libc::c_int as libc::c_uchar],};
                 init
             },},
     Vtx{v:
             {
                 let mut init =
                     Vtx_t{ob:
                               [0 as libc::c_int as libc::c_short,
                                -(2000 as libc::c_int) as libc::c_short,
                                0 as libc::c_int as libc::c_short],
                           flag: 0 as libc::c_int as libc::c_ushort,
                           tc:
                               [128 as libc::c_int as libc::c_short,
                                2048 as libc::c_int as libc::c_short],
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
                               [-(707 as libc::c_int) as libc::c_short,
                                1732 as libc::c_int as libc::c_short,
                                -(707 as libc::c_int) as libc::c_short],
                           flag: 0 as libc::c_int as libc::c_ushort,
                           tc:
                               [1280 as libc::c_int as libc::c_short,
                                137 as libc::c_int as libc::c_short],
                           cn:
                               [255 as libc::c_int as libc::c_uchar,
                                255 as libc::c_int as libc::c_uchar,
                                255 as libc::c_int as libc::c_uchar,
                                76 as libc::c_int as libc::c_uchar],};
                 init
             },},
     Vtx{v:
             {
                 let mut init =
                     Vtx_t{ob:
                               [0 as libc::c_int as libc::c_short,
                                2000 as libc::c_int as libc::c_short,
                                0 as libc::c_int as libc::c_short],
                           flag: 0 as libc::c_int as libc::c_ushort,
                           tc:
                               [1408 as libc::c_int as libc::c_short,
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
                               [-(1600 as libc::c_int) as libc::c_short,
                                1000 as libc::c_int as libc::c_short,
                                -(663 as libc::c_int) as libc::c_short],
                           flag: 0 as libc::c_int as libc::c_ushort,
                           tc:
                               [1408 as libc::c_int as libc::c_short,
                                512 as libc::c_int as libc::c_short],
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
                               [-(2000 as libc::c_int) as libc::c_short,
                                0 as libc::c_int as libc::c_short,
                                0 as libc::c_int as libc::c_short],
                           flag: 0 as libc::c_int as libc::c_ushort,
                           tc:
                               [1536 as libc::c_int as libc::c_short,
                                1024 as libc::c_int as libc::c_short],
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
                               [-(1600 as libc::c_int) as libc::c_short,
                                -(1000 as libc::c_int) as libc::c_short,
                                663 as libc::c_int as libc::c_short],
                           flag: 0 as libc::c_int as libc::c_ushort,
                           tc:
                               [1664 as libc::c_int as libc::c_short,
                                1536 as libc::c_int as libc::c_short],
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
                               [-(707 as libc::c_int) as libc::c_short,
                                -(1732 as libc::c_int) as libc::c_short,
                                707 as libc::c_int as libc::c_short],
                           flag: 0 as libc::c_int as libc::c_ushort,
                           tc:
                               [1792 as libc::c_int as libc::c_short,
                                1911 as libc::c_int as libc::c_short],
                           cn:
                               [255 as libc::c_int as libc::c_uchar,
                                255 as libc::c_int as libc::c_uchar,
                                255 as libc::c_int as libc::c_uchar,
                                76 as libc::c_int as libc::c_uchar],};
                 init
             },},
     Vtx{v:
             {
                 let mut init =
                     Vtx_t{ob:
                               [0 as libc::c_int as libc::c_short,
                                -(2000 as libc::c_int) as libc::c_short,
                                0 as libc::c_int as libc::c_short],
                           flag: 0 as libc::c_int as libc::c_ushort,
                           tc:
                               [1920 as libc::c_int as libc::c_short,
                                2048 as libc::c_int as libc::c_short],
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
                                1732 as libc::c_int as libc::c_short,
                                -(1000 as libc::c_int) as libc::c_short],
                           flag: 0 as libc::c_int as libc::c_ushort,
                           tc:
                               [1024 as libc::c_int as libc::c_short,
                                137 as libc::c_int as libc::c_short],
                           cn:
                               [255 as libc::c_int as libc::c_uchar,
                                255 as libc::c_int as libc::c_uchar,
                                255 as libc::c_int as libc::c_uchar,
                                76 as libc::c_int as libc::c_uchar],};
                 init
             },},
     Vtx{v:
             {
                 let mut init =
                     Vtx_t{ob:
                               [0 as libc::c_int as libc::c_short,
                                2000 as libc::c_int as libc::c_short,
                                0 as libc::c_int as libc::c_short],
                           flag: 0 as libc::c_int as libc::c_ushort,
                           tc:
                               [1152 as libc::c_int as libc::c_short,
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
                               [-(663 as libc::c_int) as libc::c_short,
                                1000 as libc::c_int as libc::c_short,
                                -(1600 as libc::c_int) as libc::c_short],
                           flag: 0 as libc::c_int as libc::c_ushort,
                           tc:
                               [1152 as libc::c_int as libc::c_short,
                                512 as libc::c_int as libc::c_short],
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
                               [-(707 as libc::c_int) as libc::c_short,
                                1732 as libc::c_int as libc::c_short,
                                -(707 as libc::c_int) as libc::c_short],
                           flag: 0 as libc::c_int as libc::c_ushort,
                           tc:
                               [1280 as libc::c_int as libc::c_short,
                                137 as libc::c_int as libc::c_short],
                           cn:
                               [255 as libc::c_int as libc::c_uchar,
                                255 as libc::c_int as libc::c_uchar,
                                255 as libc::c_int as libc::c_uchar,
                                76 as libc::c_int as libc::c_uchar],};
                 init
             },},
     Vtx{v:
             {
                 let mut init =
                     Vtx_t{ob:
                               [0 as libc::c_int as libc::c_short,
                                1732 as libc::c_int as libc::c_short,
                                -(1000 as libc::c_int) as libc::c_short],
                           flag: 0 as libc::c_int as libc::c_ushort,
                           tc:
                               [1024 as libc::c_int as libc::c_short,
                                137 as libc::c_int as libc::c_short],
                           cn:
                               [255 as libc::c_int as libc::c_uchar,
                                255 as libc::c_int as libc::c_uchar,
                                255 as libc::c_int as libc::c_uchar,
                                76 as libc::c_int as libc::c_uchar],};
                 init
             },},
     Vtx{v:
             {
                 let mut init =
                     Vtx_t{ob:
                               [-(1600 as libc::c_int) as libc::c_short,
                                1000 as libc::c_int as libc::c_short,
                                -(663 as libc::c_int) as libc::c_short],
                           flag: 0 as libc::c_int as libc::c_ushort,
                           tc:
                               [1408 as libc::c_int as libc::c_short,
                                512 as libc::c_int as libc::c_short],
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
                               [-(1414 as libc::c_int) as libc::c_short,
                                0 as libc::c_int as libc::c_short,
                                -(1414 as libc::c_int) as libc::c_short],
                           flag: 0 as libc::c_int as libc::c_ushort,
                           tc:
                               [1280 as libc::c_int as libc::c_short,
                                1024 as libc::c_int as libc::c_short],
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
                               [-(2000 as libc::c_int) as libc::c_short,
                                0 as libc::c_int as libc::c_short,
                                0 as libc::c_int as libc::c_short],
                           flag: 0 as libc::c_int as libc::c_ushort,
                           tc:
                               [1536 as libc::c_int as libc::c_short,
                                1024 as libc::c_int as libc::c_short],
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
                               [-(1600 as libc::c_int) as libc::c_short,
                                -(1000 as libc::c_int) as libc::c_short,
                                -(663 as libc::c_int) as libc::c_short],
                           flag: 0 as libc::c_int as libc::c_ushort,
                           tc:
                               [1408 as libc::c_int as libc::c_short,
                                1536 as libc::c_int as libc::c_short],
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
                               [-(1600 as libc::c_int) as libc::c_short,
                                -(1000 as libc::c_int) as libc::c_short,
                                663 as libc::c_int as libc::c_short],
                           flag: 0 as libc::c_int as libc::c_ushort,
                           tc:
                               [1664 as libc::c_int as libc::c_short,
                                1536 as libc::c_int as libc::c_short],
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
                               [-(1000 as libc::c_int) as libc::c_short,
                                -(1732 as libc::c_int) as libc::c_short,
                                0 as libc::c_int as libc::c_short],
                           flag: 0 as libc::c_int as libc::c_ushort,
                           tc:
                               [1536 as libc::c_int as libc::c_short,
                                1911 as libc::c_int as libc::c_short],
                           cn:
                               [255 as libc::c_int as libc::c_uchar,
                                255 as libc::c_int as libc::c_uchar,
                                255 as libc::c_int as libc::c_uchar,
                                76 as libc::c_int as libc::c_uchar],};
                 init
             },},
     Vtx{v:
             {
                 let mut init =
                     Vtx_t{ob:
                               [-(707 as libc::c_int) as libc::c_short,
                                -(1732 as libc::c_int) as libc::c_short,
                                707 as libc::c_int as libc::c_short],
                           flag: 0 as libc::c_int as libc::c_ushort,
                           tc:
                               [1792 as libc::c_int as libc::c_short,
                                1911 as libc::c_int as libc::c_short],
                           cn:
                               [255 as libc::c_int as libc::c_uchar,
                                255 as libc::c_int as libc::c_uchar,
                                255 as libc::c_int as libc::c_uchar,
                                76 as libc::c_int as libc::c_uchar],};
                 init
             },},
     Vtx{v:
             {
                 let mut init =
                     Vtx_t{ob:
                               [0 as libc::c_int as libc::c_short,
                                -(2000 as libc::c_int) as libc::c_short,
                                0 as libc::c_int as libc::c_short],
                           flag: 0 as libc::c_int as libc::c_ushort,
                           tc:
                               [1664 as libc::c_int as libc::c_short,
                                2048 as libc::c_int as libc::c_short],
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
                               [707 as libc::c_int as libc::c_short,
                                1732 as libc::c_int as libc::c_short,
                                -(707 as libc::c_int) as libc::c_short],
                           flag: 0 as libc::c_int as libc::c_ushort,
                           tc:
                               [768 as libc::c_int as libc::c_short,
                                137 as libc::c_int as libc::c_short],
                           cn:
                               [255 as libc::c_int as libc::c_uchar,
                                255 as libc::c_int as libc::c_uchar,
                                255 as libc::c_int as libc::c_uchar,
                                76 as libc::c_int as libc::c_uchar],};
                 init
             },},
     Vtx{v:
             {
                 let mut init =
                     Vtx_t{ob:
                               [0 as libc::c_int as libc::c_short,
                                2000 as libc::c_int as libc::c_short,
                                0 as libc::c_int as libc::c_short],
                           flag: 0 as libc::c_int as libc::c_ushort,
                           tc:
                               [896 as libc::c_int as libc::c_short,
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
                               [663 as libc::c_int as libc::c_short,
                                1000 as libc::c_int as libc::c_short,
                                -(1600 as libc::c_int) as libc::c_short],
                           flag: 0 as libc::c_int as libc::c_ushort,
                           tc:
                               [896 as libc::c_int as libc::c_short,
                                512 as libc::c_int as libc::c_short],
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
                                -(2000 as libc::c_int) as libc::c_short],
                           flag: 0 as libc::c_int as libc::c_ushort,
                           tc:
                               [1024 as libc::c_int as libc::c_short,
                                1024 as libc::c_int as libc::c_short],
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
                               [-(663 as libc::c_int) as libc::c_short,
                                -(1000 as libc::c_int) as libc::c_short,
                                -(1600 as libc::c_int) as libc::c_short],
                           flag: 0 as libc::c_int as libc::c_ushort,
                           tc:
                               [1152 as libc::c_int as libc::c_short,
                                1536 as libc::c_int as libc::c_short],
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
                               [-(707 as libc::c_int) as libc::c_short,
                                -(1732 as libc::c_int) as libc::c_short,
                                -(707 as libc::c_int) as libc::c_short],
                           flag: 0 as libc::c_int as libc::c_ushort,
                           tc:
                               [1280 as libc::c_int as libc::c_short,
                                1911 as libc::c_int as libc::c_short],
                           cn:
                               [255 as libc::c_int as libc::c_uchar,
                                255 as libc::c_int as libc::c_uchar,
                                255 as libc::c_int as libc::c_uchar,
                                76 as libc::c_int as libc::c_uchar],};
                 init
             },},
     Vtx{v:
             {
                 let mut init =
                     Vtx_t{ob:
                               [0 as libc::c_int as libc::c_short,
                                -(2000 as libc::c_int) as libc::c_short,
                                0 as libc::c_int as libc::c_short],
                           flag: 0 as libc::c_int as libc::c_ushort,
                           tc:
                               [1408 as libc::c_int as libc::c_short,
                                2048 as libc::c_int as libc::c_short],
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
                               [1000 as libc::c_int as libc::c_short,
                                1732 as libc::c_int as libc::c_short,
                                0 as libc::c_int as libc::c_short],
                           flag: 0 as libc::c_int as libc::c_ushort,
                           tc:
                               [512 as libc::c_int as libc::c_short,
                                137 as libc::c_int as libc::c_short],
                           cn:
                               [255 as libc::c_int as libc::c_uchar,
                                255 as libc::c_int as libc::c_uchar,
                                255 as libc::c_int as libc::c_uchar,
                                76 as libc::c_int as libc::c_uchar],};
                 init
             },},
     Vtx{v:
             {
                 let mut init =
                     Vtx_t{ob:
                               [0 as libc::c_int as libc::c_short,
                                2000 as libc::c_int as libc::c_short,
                                0 as libc::c_int as libc::c_short],
                           flag: 0 as libc::c_int as libc::c_ushort,
                           tc:
                               [640 as libc::c_int as libc::c_short,
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
                               [1600 as libc::c_int as libc::c_short,
                                1000 as libc::c_int as libc::c_short,
                                -(663 as libc::c_int) as libc::c_short],
                           flag: 0 as libc::c_int as libc::c_ushort,
                           tc:
                               [640 as libc::c_int as libc::c_short,
                                512 as libc::c_int as libc::c_short],
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
                               [1414 as libc::c_int as libc::c_short,
                                0 as libc::c_int as libc::c_short,
                                -(1414 as libc::c_int) as libc::c_short],
                           flag: 0 as libc::c_int as libc::c_ushort,
                           tc:
                               [768 as libc::c_int as libc::c_short,
                                1024 as libc::c_int as libc::c_short],
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
                               [663 as libc::c_int as libc::c_short,
                                -(1000 as libc::c_int) as libc::c_short,
                                -(1600 as libc::c_int) as libc::c_short],
                           flag: 0 as libc::c_int as libc::c_ushort,
                           tc:
                               [896 as libc::c_int as libc::c_short,
                                1536 as libc::c_int as libc::c_short],
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
                                -(1732 as libc::c_int) as libc::c_short,
                                -(1000 as libc::c_int) as libc::c_short],
                           flag: 0 as libc::c_int as libc::c_ushort,
                           tc:
                               [1024 as libc::c_int as libc::c_short,
                                1911 as libc::c_int as libc::c_short],
                           cn:
                               [255 as libc::c_int as libc::c_uchar,
                                255 as libc::c_int as libc::c_uchar,
                                255 as libc::c_int as libc::c_uchar,
                                76 as libc::c_int as libc::c_uchar],};
                 init
             },},
     Vtx{v:
             {
                 let mut init =
                     Vtx_t{ob:
                               [0 as libc::c_int as libc::c_short,
                                -(2000 as libc::c_int) as libc::c_short,
                                0 as libc::c_int as libc::c_short],
                           flag: 0 as libc::c_int as libc::c_ushort,
                           tc:
                               [1152 as libc::c_int as libc::c_short,
                                2048 as libc::c_int as libc::c_short],
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
                               [707 as libc::c_int as libc::c_short,
                                1732 as libc::c_int as libc::c_short,
                                707 as libc::c_int as libc::c_short],
                           flag: 0 as libc::c_int as libc::c_ushort,
                           tc:
                               [256 as libc::c_int as libc::c_short,
                                137 as libc::c_int as libc::c_short],
                           cn:
                               [255 as libc::c_int as libc::c_uchar,
                                255 as libc::c_int as libc::c_uchar,
                                255 as libc::c_int as libc::c_uchar,
                                76 as libc::c_int as libc::c_uchar],};
                 init
             },},
     Vtx{v:
             {
                 let mut init =
                     Vtx_t{ob:
                               [0 as libc::c_int as libc::c_short,
                                2000 as libc::c_int as libc::c_short,
                                0 as libc::c_int as libc::c_short],
                           flag: 0 as libc::c_int as libc::c_ushort,
                           tc:
                               [384 as libc::c_int as libc::c_short,
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
                               [1600 as libc::c_int as libc::c_short,
                                1000 as libc::c_int as libc::c_short,
                                663 as libc::c_int as libc::c_short],
                           flag: 0 as libc::c_int as libc::c_ushort,
                           tc:
                               [384 as libc::c_int as libc::c_short,
                                512 as libc::c_int as libc::c_short],
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
                               [2000 as libc::c_int as libc::c_short,
                                0 as libc::c_int as libc::c_short,
                                0 as libc::c_int as libc::c_short],
                           flag: 0 as libc::c_int as libc::c_ushort,
                           tc:
                               [512 as libc::c_int as libc::c_short,
                                1024 as libc::c_int as libc::c_short],
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
                               [1600 as libc::c_int as libc::c_short,
                                -(1000 as libc::c_int) as libc::c_short,
                                -(663 as libc::c_int) as libc::c_short],
                           flag: 0 as libc::c_int as libc::c_ushort,
                           tc:
                               [640 as libc::c_int as libc::c_short,
                                1536 as libc::c_int as libc::c_short],
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
                               [707 as libc::c_int as libc::c_short,
                                -(1732 as libc::c_int) as libc::c_short,
                                -(707 as libc::c_int) as libc::c_short],
                           flag: 0 as libc::c_int as libc::c_ushort,
                           tc:
                               [768 as libc::c_int as libc::c_short,
                                1911 as libc::c_int as libc::c_short],
                           cn:
                               [255 as libc::c_int as libc::c_uchar,
                                255 as libc::c_int as libc::c_uchar,
                                255 as libc::c_int as libc::c_uchar,
                                76 as libc::c_int as libc::c_uchar],};
                 init
             },},
     Vtx{v:
             {
                 let mut init =
                     Vtx_t{ob:
                               [0 as libc::c_int as libc::c_short,
                                -(2000 as libc::c_int) as libc::c_short,
                                0 as libc::c_int as libc::c_short],
                           flag: 0 as libc::c_int as libc::c_ushort,
                           tc:
                               [896 as libc::c_int as libc::c_short,
                                2048 as libc::c_int as libc::c_short],
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
                                1732 as libc::c_int as libc::c_short,
                                1000 as libc::c_int as libc::c_short],
                           flag: 0 as libc::c_int as libc::c_ushort,
                           tc:
                               [0 as libc::c_int as libc::c_short,
                                137 as libc::c_int as libc::c_short],
                           cn:
                               [255 as libc::c_int as libc::c_uchar,
                                255 as libc::c_int as libc::c_uchar,
                                255 as libc::c_int as libc::c_uchar,
                                76 as libc::c_int as libc::c_uchar],};
                 init
             },},
     Vtx{v:
             {
                 let mut init =
                     Vtx_t{ob:
                               [707 as libc::c_int as libc::c_short,
                                1732 as libc::c_int as libc::c_short,
                                707 as libc::c_int as libc::c_short],
                           flag: 0 as libc::c_int as libc::c_ushort,
                           tc:
                               [256 as libc::c_int as libc::c_short,
                                137 as libc::c_int as libc::c_short],
                           cn:
                               [255 as libc::c_int as libc::c_uchar,
                                255 as libc::c_int as libc::c_uchar,
                                255 as libc::c_int as libc::c_uchar,
                                76 as libc::c_int as libc::c_uchar],};
                 init
             },},
     Vtx{v:
             {
                 let mut init =
                     Vtx_t{ob:
                               [0 as libc::c_int as libc::c_short,
                                2000 as libc::c_int as libc::c_short,
                                0 as libc::c_int as libc::c_short],
                           flag: 0 as libc::c_int as libc::c_ushort,
                           tc:
                               [128 as libc::c_int as libc::c_short,
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
                               [663 as libc::c_int as libc::c_short,
                                1000 as libc::c_int as libc::c_short,
                                1600 as libc::c_int as libc::c_short],
                           flag: 0 as libc::c_int as libc::c_ushort,
                           tc:
                               [128 as libc::c_int as libc::c_short,
                                512 as libc::c_int as libc::c_short],
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
                               [1600 as libc::c_int as libc::c_short,
                                1000 as libc::c_int as libc::c_short,
                                663 as libc::c_int as libc::c_short],
                           flag: 0 as libc::c_int as libc::c_ushort,
                           tc:
                               [384 as libc::c_int as libc::c_short,
                                512 as libc::c_int as libc::c_short],
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
                               [1414 as libc::c_int as libc::c_short,
                                0 as libc::c_int as libc::c_short,
                                1414 as libc::c_int as libc::c_short],
                           flag: 0 as libc::c_int as libc::c_ushort,
                           tc:
                               [256 as libc::c_int as libc::c_short,
                                1024 as libc::c_int as libc::c_short],
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
                               [2000 as libc::c_int as libc::c_short,
                                0 as libc::c_int as libc::c_short,
                                0 as libc::c_int as libc::c_short],
                           flag: 0 as libc::c_int as libc::c_ushort,
                           tc:
                               [512 as libc::c_int as libc::c_short,
                                1024 as libc::c_int as libc::c_short],
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
                               [1600 as libc::c_int as libc::c_short,
                                -(1000 as libc::c_int) as libc::c_short,
                                663 as libc::c_int as libc::c_short],
                           flag: 0 as libc::c_int as libc::c_ushort,
                           tc:
                               [384 as libc::c_int as libc::c_short,
                                1536 as libc::c_int as libc::c_short],
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
                               [1600 as libc::c_int as libc::c_short,
                                -(1000 as libc::c_int) as libc::c_short,
                                -(663 as libc::c_int) as libc::c_short],
                           flag: 0 as libc::c_int as libc::c_ushort,
                           tc:
                               [640 as libc::c_int as libc::c_short,
                                1536 as libc::c_int as libc::c_short],
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
                               [1000 as libc::c_int as libc::c_short,
                                -(1732 as libc::c_int) as libc::c_short,
                                0 as libc::c_int as libc::c_short],
                           flag: 0 as libc::c_int as libc::c_ushort,
                           tc:
                               [512 as libc::c_int as libc::c_short,
                                1911 as libc::c_int as libc::c_short],
                           cn:
                               [255 as libc::c_int as libc::c_uchar,
                                255 as libc::c_int as libc::c_uchar,
                                255 as libc::c_int as libc::c_uchar,
                                76 as libc::c_int as libc::c_uchar],};
                 init
             },},
     Vtx{v:
             {
                 let mut init =
                     Vtx_t{ob:
                               [707 as libc::c_int as libc::c_short,
                                -(1732 as libc::c_int) as libc::c_short,
                                -(707 as libc::c_int) as libc::c_short],
                           flag: 0 as libc::c_int as libc::c_ushort,
                           tc:
                               [768 as libc::c_int as libc::c_short,
                                1911 as libc::c_int as libc::c_short],
                           cn:
                               [255 as libc::c_int as libc::c_uchar,
                                255 as libc::c_int as libc::c_uchar,
                                255 as libc::c_int as libc::c_uchar,
                                76 as libc::c_int as libc::c_uchar],};
                 init
             },},
     Vtx{v:
             {
                 let mut init =
                     Vtx_t{ob:
                               [0 as libc::c_int as libc::c_short,
                                -(2000 as libc::c_int) as libc::c_short,
                                0 as libc::c_int as libc::c_short],
                           flag: 0 as libc::c_int as libc::c_ushort,
                           tc:
                               [640 as libc::c_int as libc::c_short,
                                2048 as libc::c_int as libc::c_short],
                           cn:
                               [255 as libc::c_int as libc::c_uchar,
                                255 as libc::c_int as libc::c_uchar,
                                255 as libc::c_int as libc::c_uchar,
                                0 as libc::c_int as libc::c_uchar],};
                 init
             },}];
static mut sCylinderInit: ColliderCylinderInit =
    {
        let mut init =
            ColliderCylinderInit{base:
                                     {
                                         let mut init =
                                             ColliderInit{colType:
                                                              COLTYPE_NONE as
                                                                  libc::c_int
                                                                  as u8_0,
                                                          atFlags:
                                                              ((1 as
                                                                    libc::c_int)
                                                                   <<
                                                                   0 as
                                                                       libc::c_int
                                                                   |
                                                                   (1 as
                                                                        libc::c_int)
                                                                       <<
                                                                       3 as
                                                                           libc::c_int)
                                                                  as u8_0,
                                                          acFlags:
                                                              0 as libc::c_int
                                                                  as u8_0,
                                                          ocFlags1:
                                                              0 as libc::c_int
                                                                  as u8_0,
                                                          ocFlags2:
                                                              ((1 as
                                                                    libc::c_int)
                                                                   <<
                                                                   4 as
                                                                       libc::c_int)
                                                                  as u8_0,
                                                          shape:
                                                              COLSHAPE_CYLINDER
                                                                  as
                                                                  libc::c_int
                                                                  as u8_0,};
                                         init
                                     },
                                 info:
                                     {
                                         let mut init =
                                             ColliderInfoInit{elemType:
                                                                  ELEMTYPE_UNK0
                                                                      as
                                                                      libc::c_int
                                                                      as u8_0,
                                                              toucher:
                                                                  {
                                                                      let mut init =
                                                                          ColliderTouch{dmgFlags:
                                                                                            0x20000
                                                                                                as
                                                                                                libc::c_int
                                                                                                as
                                                                                                u32_0,
                                                                                        effect:
                                                                                            0
                                                                                                as
                                                                                                libc::c_int
                                                                                                as
                                                                                                u8_0,
                                                                                        damage:
                                                                                            0x1
                                                                                                as
                                                                                                libc::c_int
                                                                                                as
                                                                                                u8_0,};
                                                                      init
                                                                  },
                                                              bumper:
                                                                  {
                                                                      let mut init =
                                                                          ColliderBumpInit{dmgFlags:
                                                                                               0
                                                                                                   as
                                                                                                   libc::c_int
                                                                                                   as
                                                                                                   u32_0,
                                                                                           effect:
                                                                                               0
                                                                                                   as
                                                                                                   libc::c_int
                                                                                                   as
                                                                                                   u8_0,
                                                                                           defense:
                                                                                               0
                                                                                                   as
                                                                                                   libc::c_int
                                                                                                   as
                                                                                                   u8_0,};
                                                                      init
                                                                  },
                                                              toucherFlags:
                                                                  ((1 as
                                                                        libc::c_int)
                                                                       <<
                                                                       0 as
                                                                           libc::c_int
                                                                       |
                                                                       (3 as
                                                                            libc::c_int)
                                                                           <<
                                                                           3
                                                                               as
                                                                               libc::c_int)
                                                                      as u8_0,
                                                              bumperFlags:
                                                                  0 as
                                                                      libc::c_int
                                                                      as u8_0,
                                                              ocElemFlags:
                                                                  0 as
                                                                      libc::c_int
                                                                      as
                                                                      u8_0,};
                                         init
                                     },
                                 dim:
                                     {
                                         let mut init =
                                             Cylinder16{radius:
                                                            9 as libc::c_int
                                                                as s16,
                                                        height:
                                                            9 as libc::c_int
                                                                as s16,
                                                        yShift:
                                                            0 as libc::c_int
                                                                as s16,
                                                        pos:
                                                            {
                                                                let mut init =
                                                                    Vec3s{x:
                                                                              0
                                                                                  as
                                                                                  libc::c_int
                                                                                  as
                                                                                  s16,
                                                                          y:
                                                                              0
                                                                                  as
                                                                                  libc::c_int
                                                                                  as
                                                                                  s16,
                                                                          z:
                                                                              0
                                                                                  as
                                                                                  libc::c_int
                                                                                  as
                                                                                  s16,};
                                                                init
                                                            },};
                                         init
                                     },};
        init
    };
// Initialized in run_static_initializers
static mut sInitChain: [InitChainEntry; 1] =
    [InitChainEntry{cont_type_0_offset_value: [0; 4],}; 1];
static mut sVertexIndices: [u8_0; 60] =
    [3 as libc::c_int as u8_0, 4 as libc::c_int as u8_0,
     5 as libc::c_int as u8_0, 6 as libc::c_int as u8_0,
     7 as libc::c_int as u8_0, 8 as libc::c_int as u8_0,
     9 as libc::c_int as u8_0, 10 as libc::c_int as u8_0,
     16 as libc::c_int as u8_0, 17 as libc::c_int as u8_0,
     18 as libc::c_int as u8_0, 19 as libc::c_int as u8_0,
     25 as libc::c_int as u8_0, 26 as libc::c_int as u8_0,
     27 as libc::c_int as u8_0, 32 as libc::c_int as u8_0,
     35 as libc::c_int as u8_0, 36 as libc::c_int as u8_0,
     37 as libc::c_int as u8_0, 38 as libc::c_int as u8_0,
     39 as libc::c_int as u8_0, 45 as libc::c_int as u8_0,
     46 as libc::c_int as u8_0, 47 as libc::c_int as u8_0,
     52 as libc::c_int as u8_0, 53 as libc::c_int as u8_0,
     54 as libc::c_int as u8_0, 59 as libc::c_int as u8_0,
     60 as libc::c_int as u8_0, 61 as libc::c_int as u8_0,
     67 as libc::c_int as u8_0, 68 as libc::c_int as u8_0,
     69 as libc::c_int as u8_0, 70 as libc::c_int as u8_0,
     71 as libc::c_int as u8_0, 72 as libc::c_int as u8_0,
     0 as libc::c_int as u8_0, 1 as libc::c_int as u8_0,
     11 as libc::c_int as u8_0, 12 as libc::c_int as u8_0,
     14 as libc::c_int as u8_0, 20 as libc::c_int as u8_0,
     21 as libc::c_int as u8_0, 23 as libc::c_int as u8_0,
     28 as libc::c_int as u8_0, 30 as libc::c_int as u8_0,
     33 as libc::c_int as u8_0, 34 as libc::c_int as u8_0,
     40 as libc::c_int as u8_0, 41 as libc::c_int as u8_0,
     43 as libc::c_int as u8_0, 48 as libc::c_int as u8_0,
     50 as libc::c_int as u8_0, 55 as libc::c_int as u8_0,
     57 as libc::c_int as u8_0, 62 as libc::c_int as u8_0,
     64 as libc::c_int as u8_0, 65 as libc::c_int as u8_0,
     73 as libc::c_int as u8_0, 74 as libc::c_int as u8_0];
#[no_mangle]
pub unsafe extern "C" fn MagicFire_Init(mut thisx: *mut Actor,
                                        mut globalCtx: *mut GlobalContext) {
    let mut this: *mut MagicFire = thisx as *mut MagicFire;
    Actor_ProcessInitChain(&mut (*this).actor, sInitChain.as_mut_ptr());
    (*this).action = 0 as libc::c_int as s16;
    (*this).screenTintBehaviour = 0 as libc::c_int as s16;
    (*this).actionTimer = 0 as libc::c_int as s16;
    (*this).alphaMultiplier = -3.0f32;
    Actor_SetScale(&mut (*this).actor, 0.0f32);
    Collider_InitCylinder(globalCtx, &mut (*this).collider);
    Collider_SetCylinder(globalCtx, &mut (*this).collider, &mut (*this).actor,
                         &mut sCylinderInit);
    Collider_UpdateCylinder(&mut (*this).actor, &mut (*this).collider);
    (*this).actor.update =
        Some(MagicFire_UpdateBeforeCast as
                 unsafe extern "C" fn(_: *mut Actor, _: *mut GlobalContext)
                     -> ());
    (*this).actionTimer = 20 as libc::c_int as s16;
    (*this).actor.room = -(1 as libc::c_int) as s8;
}
#[no_mangle]
pub unsafe extern "C" fn MagicFire_Destroy(mut thisx: *mut Actor,
                                           mut globalCtx:
                                               *mut GlobalContext) {
    func_800876C8(globalCtx);
}
#[no_mangle]
pub unsafe extern "C" fn MagicFire_UpdateBeforeCast(mut thisx: *mut Actor,
                                                    mut globalCtx:
                                                        *mut GlobalContext) {
    let mut this: *mut MagicFire = thisx as *mut MagicFire;
    let mut player: *mut Player =
        (*globalCtx).actorCtx.actorLists[ACTORCAT_PLAYER as libc::c_int as
                                             usize].head as *mut Player;
    if (*globalCtx).msgCtx.msgMode as libc::c_int ==
           MSGMODE_OCARINA_CORRECT_PLAYBACK as libc::c_int ||
           (*globalCtx).msgCtx.msgMode as libc::c_int ==
               MSGMODE_SONG_PLAYED as libc::c_int {
        Actor_Kill(&mut (*this).actor);
        return
    }
    if (*this).actionTimer as libc::c_int > 0 as libc::c_int {
        (*this).actionTimer -= 1
    } else {
        (*this).actor.update =
            Some(MagicFire_Update as
                     unsafe extern "C" fn(_: *mut Actor,
                                          _: *mut GlobalContext) -> ());
        func_8002F7DC(&mut (*player).actor, 0x879 as libc::c_int as u16_0);
    }
    (*this).actor.world.pos = (*player).actor.world.pos;
}
#[no_mangle]
pub unsafe extern "C" fn MagicFire_Update(mut thisx: *mut Actor,
                                          mut globalCtx: *mut GlobalContext) {
    let mut this: *mut MagicFire = thisx as *mut MagicFire;
    let mut player: *mut Player =
        (*globalCtx).actorCtx.actorLists[ACTORCAT_PLAYER as libc::c_int as
                                             usize].head as *mut Player;
    let mut pad: s32 = 0;
    (*this).actor.world.pos = (*player).actor.world.pos;
    if (*globalCtx).msgCtx.msgMode as libc::c_int ==
           MSGMODE_OCARINA_CORRECT_PLAYBACK as libc::c_int ||
           (*globalCtx).msgCtx.msgMode as libc::c_int ==
               MSGMODE_SONG_PLAYED as libc::c_int {
        Actor_Kill(&mut (*this).actor);
        return
    }
    if (*this).action as libc::c_int == DF_ACTION_EXPAND_SLOWLY as libc::c_int
       {
        (*this).collider.info.toucher.damage =
            ((*this).actionTimer as libc::c_int + 25 as libc::c_int) as u8_0
    } else if (*this).action as libc::c_int ==
                  DF_ACTION_STOP_EXPANDING as libc::c_int {
        (*this).collider.info.toucher.damage = (*this).actionTimer as u8_0
    }
    Collider_UpdateCylinder(&mut (*this).actor, &mut (*this).collider);
    (*this).collider.dim.radius = ((*this).actor.scale.x * 325.0f32) as s16;
    (*this).collider.dim.height = ((*this).actor.scale.y * 450.0f32) as s16;
    (*this).collider.dim.yShift = ((*this).actor.scale.y * -225.0f32) as s16;
    CollisionCheck_SetAT(globalCtx, &mut (*globalCtx).colChkCtx,
                         &mut (*this).collider.base);
    match (*this).action as libc::c_int {
        0 => {
            (*this).actionTimer = 30 as libc::c_int as s16;
            (*this).actor.scale.z = 0.0f32;
            (*this).actor.scale.y = (*this).actor.scale.z;
            (*this).actor.scale.x = (*this).actor.scale.y;
            (*this).actor.world.rot.z = 0 as libc::c_int as s16;
            (*this).actor.world.rot.y = (*this).actor.world.rot.z;
            (*this).actor.world.rot.x = (*this).actor.world.rot.y;
            (*this).actor.shape.rot.z = 0 as libc::c_int as s16;
            (*this).actor.shape.rot.y = (*this).actor.shape.rot.z;
            (*this).actor.shape.rot.x = (*this).actor.shape.rot.y;
            (*this).alphaMultiplier = 0.0f32;
            (*this).scalingSpeed = 0.08f32;
            (*this).action += 1
        }
        1 => {
            // Fire sphere slowly expands out of player for 30 frames
            Math_StepToF(&mut (*this).alphaMultiplier, 1.0f32,
                         1.0f32 / 30.0f32);
            if (*this).actionTimer as libc::c_int > 0 as libc::c_int {
                Math_SmoothStepToF(&mut (*this).actor.scale.x, 0.4f32,
                                   (*this).scalingSpeed, 0.1f32, 0.001f32);
                (*this).actor.scale.z = (*this).actor.scale.x;
                (*this).actor.scale.y = (*this).actor.scale.z
            } else {
                (*this).actionTimer = 25 as libc::c_int as s16;
                (*this).action += 1
            }
        }
        2 => {
            // Sphere stops expanding and maintains size for 25 frames
            if (*this).actionTimer as libc::c_int <= 0 as libc::c_int {
                (*this).actionTimer = 15 as libc::c_int as s16;
                (*this).action += 1;
                (*this).scalingSpeed = 0.05f32
            }
        }
        3 => {
            // Sphere beings to grow again and quickly expands out until killed
            (*this).alphaMultiplier -= 8.0f32 / 119.00001f32;
            (*this).actor.scale.x += (*this).scalingSpeed;
            (*this).actor.scale.y += (*this).scalingSpeed;
            (*this).actor.scale.z += (*this).scalingSpeed;
            if (*this).alphaMultiplier <= 0.0f32 {
                (*this).action = 0 as libc::c_int as s16;
                Actor_Kill(&mut (*this).actor);
            }
        }
        _ => { }
    }
    match (*this).screenTintBehaviour as libc::c_int {
        0 => {
            if (*this).screenTintBehaviourTimer as libc::c_int <=
                   0 as libc::c_int {
                (*this).screenTintBehaviourTimer = 20 as libc::c_int as s16;
                (*this).screenTintBehaviour =
                    DF_SCREEN_TINT_FADE_IN as libc::c_int as s16
            }
        }
        1 => {
            (*this).screenTintIntensity =
                1.0f32 -
                    (*this).screenTintBehaviourTimer as libc::c_int as
                        libc::c_float / 20.0f32;
            if (*this).screenTintBehaviourTimer as libc::c_int <=
                   0 as libc::c_int {
                (*this).screenTintBehaviourTimer = 45 as libc::c_int as s16;
                (*this).screenTintBehaviour =
                    DF_SCREEN_TINT_MAINTAIN as libc::c_int as s16
            }
        }
        2 => {
            if (*this).screenTintBehaviourTimer as libc::c_int <=
                   0 as libc::c_int {
                (*this).screenTintBehaviourTimer = 5 as libc::c_int as s16;
                (*this).screenTintBehaviour =
                    DF_SCREEN_TINT_FADE_OUT as libc::c_int as s16
            }
        }
        3 => {
            (*this).screenTintIntensity =
                (*this).screenTintBehaviourTimer as libc::c_int as
                    libc::c_float / 5.0f32;
            if (*this).screenTintBehaviourTimer as libc::c_int <=
                   0 as libc::c_int {
                (*this).screenTintBehaviour =
                    DF_SCREEN_TINT_FINISHED as libc::c_int as s16
            }
        }
        _ => { }
    }
    if (*this).actionTimer as libc::c_int > 0 as libc::c_int {
        (*this).actionTimer -= 1
    }
    if (*this).screenTintBehaviourTimer as libc::c_int > 0 as libc::c_int {
        (*this).screenTintBehaviourTimer -= 1
    };
}
#[no_mangle]
pub unsafe extern "C" fn MagicFire_Draw(mut thisx: *mut Actor,
                                        mut globalCtx: *mut GlobalContext) {
    let mut this: *mut MagicFire = thisx as *mut MagicFire;
    let mut pad1: s32 = 0;
    let mut gameplayFrames: u32_0 = (*globalCtx).gameplayFrames;
    let mut pad2: s32 = 0;
    let mut i: s32 = 0;
    let mut alpha: u8_0 = 0;
    if (*this).action as libc::c_int > 0 as libc::c_int {
        let mut __gfxCtx: *mut GraphicsContext = 0 as *mut GraphicsContext;
        let mut dispRefs: [*mut Gfx; 4] = [0 as *mut Gfx; 4];
        __gfxCtx = (*globalCtx).state.gfxCtx;
        Graph_OpenDisps(dispRefs.as_mut_ptr(), (*globalCtx).state.gfxCtx,
                        b"../z_magic_fire.c\x00" as *const u8 as
                            *const libc::c_char, 682 as libc::c_int);
        (*__gfxCtx).polyXlu.p = func_800937C0((*__gfxCtx).polyXlu.p);
        let fresh0 = (*__gfxCtx).polyXlu.p;
        (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
        let mut _g: *mut Gfx = fresh0;
        (*_g).words.w0 =
            (0xfa as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 24 as libc::c_int
                |
                (0 as libc::c_int as u32_0 &
                     (((0x1 as libc::c_int) << 8 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    8 as libc::c_int |
                (0 as libc::c_int as u32_0 &
                     (((0x1 as libc::c_int) << 8 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    0 as libc::c_int;
        (*_g).words.w1 =
            ((60 as libc::c_int as libc::c_float *
                  (*this).screenTintIntensity) as s32 as u8_0 as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 24 as libc::c_int
                |
                ((20 as libc::c_int as libc::c_float *
                      (*this).screenTintIntensity) as s32 as u8_0 as u32_0 &
                     (((0x1 as libc::c_int) << 8 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    16 as libc::c_int |
                ((0 as libc::c_int as libc::c_float *
                      (*this).screenTintIntensity) as s32 as u8_0 as u32_0 &
                     (((0x1 as libc::c_int) << 8 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    8 as libc::c_int |
                ((120 as libc::c_int as libc::c_float *
                      (*this).screenTintIntensity) as s32 as u8_0 as u32_0 &
                     (((0x1 as libc::c_int) << 8 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    0 as libc::c_int;
        let fresh1 = (*__gfxCtx).polyXlu.p;
        (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
        let mut _g_0: *mut Gfx = fresh1;
        (*_g_0).words.w0 =
            (0xe3 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 24 as libc::c_int
                |
                ((32 as libc::c_int - 4 as libc::c_int - 2 as libc::c_int) as
                     u32_0 &
                     (((0x1 as libc::c_int) << 8 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    8 as libc::c_int |
                ((2 as libc::c_int - 1 as libc::c_int) as u32_0 &
                     (((0x1 as libc::c_int) << 8 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    0 as libc::c_int;
        (*_g_0).words.w1 =
            ((3 as libc::c_int) << 4 as libc::c_int) as libc::c_uint;
        let fresh2 = (*__gfxCtx).polyXlu.p;
        (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
        let mut _g_1: *mut Gfx = fresh2;
        (*_g_1).words.w0 =
            (0xe3 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 24 as libc::c_int
                |
                ((32 as libc::c_int - 6 as libc::c_int - 2 as libc::c_int) as
                     u32_0 &
                     (((0x1 as libc::c_int) << 8 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    8 as libc::c_int |
                ((2 as libc::c_int - 1 as libc::c_int) as u32_0 &
                     (((0x1 as libc::c_int) << 8 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    0 as libc::c_int;
        (*_g_1).words.w1 =
            ((3 as libc::c_int) << 6 as libc::c_int) as libc::c_uint;
        let fresh3 = (*__gfxCtx).polyXlu.p;
        (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
        let mut _g_2: *mut Gfx = fresh3;
        (*_g_2).words.w0 =
            (0xf6 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 24 as libc::c_int
                |
                (319 as libc::c_int as u32_0 &
                     (((0x1 as libc::c_int) << 10 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    14 as libc::c_int |
                (239 as libc::c_int as u32_0 &
                     (((0x1 as libc::c_int) << 10 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    2 as libc::c_int;
        (*_g_2).words.w1 =
            (0 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 10 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 14 as libc::c_int
                |
                (0 as libc::c_int as u32_0 &
                     (((0x1 as libc::c_int) << 10 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    2 as libc::c_int;
        func_80093D84((*globalCtx).state.gfxCtx);
        let fresh4 = (*__gfxCtx).polyXlu.p;
        (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
        let mut _g_3: *mut Gfx = fresh4;
        (*_g_3).words.w0 =
            (0xfa as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 24 as libc::c_int
                |
                (0 as libc::c_int as u32_0 &
                     (((0x1 as libc::c_int) << 8 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    8 as libc::c_int |
                (128 as libc::c_int as u32_0 &
                     (((0x1 as libc::c_int) << 8 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    0 as libc::c_int;
        (*_g_3).words.w1 =
            (255 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 24 as libc::c_int
                |
                (200 as libc::c_int as u32_0 &
                     (((0x1 as libc::c_int) << 8 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    16 as libc::c_int |
                (0 as libc::c_int as u32_0 &
                     (((0x1 as libc::c_int) << 8 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    8 as libc::c_int |
                (((*this).alphaMultiplier *
                      255 as libc::c_int as libc::c_float) as u8_0 as u32_0 &
                     (((0x1 as libc::c_int) << 8 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    0 as libc::c_int;
        let fresh5 = (*__gfxCtx).polyXlu.p;
        (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
        let mut _g_4: *mut Gfx = fresh5;
        (*_g_4).words.w0 =
            (0xfb as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 24 as libc::c_int;
        (*_g_4).words.w1 =
            (255 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 24 as libc::c_int
                |
                (0 as libc::c_int as u32_0 &
                     (((0x1 as libc::c_int) << 8 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    16 as libc::c_int |
                (0 as libc::c_int as u32_0 &
                     (((0x1 as libc::c_int) << 8 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    8 as libc::c_int |
                (((*this).alphaMultiplier *
                      255 as libc::c_int as libc::c_float) as u8_0 as u32_0 &
                     (((0x1 as libc::c_int) << 8 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    0 as libc::c_int;
        Matrix_Scale(0.15f32, 0.15f32, 0.15f32,
                     MTXMODE_APPLY as libc::c_int as u8_0);
        let fresh6 = (*__gfxCtx).polyXlu.p;
        (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
        let mut _g_5: *mut Gfx = fresh6;
        (*_g_5).words.w0 =
            (0xda as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 24 as libc::c_int
                |
                ((::std::mem::size_of::<Mtx>() as
                      libc::c_ulong).wrapping_sub(1 as libc::c_int as
                                                      libc::c_uint).wrapping_div(8
                                                                                     as
                                                                                     libc::c_int
                                                                                     as
                                                                                     libc::c_uint)
                     &
                     (((0x1 as libc::c_int) << 5 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    19 as libc::c_int |
                ((0 as libc::c_int / 8 as libc::c_int) as u32_0 &
                     (((0x1 as libc::c_int) << 8 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    8 as libc::c_int |
                (((0 as libc::c_int | 0x2 as libc::c_int | 0 as libc::c_int) ^
                      0x1 as libc::c_int) as u32_0 &
                     (((0x1 as libc::c_int) << 8 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    0 as libc::c_int;
        (*_g_5).words.w1 =
            Matrix_NewMtx((*globalCtx).state.gfxCtx,
                          b"../z_magic_fire.c\x00" as *const u8 as
                              *const libc::c_char as *mut libc::c_char,
                          715 as libc::c_int) as libc::c_uint;
        let fresh7 = (*__gfxCtx).polyXlu.p;
        (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
        let mut _g_6: *mut Gfx = fresh7;
        (*_g_6).words.w0 =
            (0xe7 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 24 as libc::c_int;
        (*_g_6).words.w1 = 0 as libc::c_int as libc::c_uint;
        let fresh8 = (*__gfxCtx).polyXlu.p;
        (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
        let mut _g_7: *mut Gfx = fresh8;
        (*_g_7).words.w0 =
            (0xd7 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 24 as libc::c_int
                |
                (0 as libc::c_int as u32_0 &
                     (((0x1 as libc::c_int) << 8 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    16 as libc::c_int |
                (0 as libc::c_int as u32_0 &
                     (((0x1 as libc::c_int) << 3 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    11 as libc::c_int |
                (0 as libc::c_int as u32_0 &
                     (((0x1 as libc::c_int) << 3 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    8 as libc::c_int |
                (1 as libc::c_int as u32_0 &
                     (((0x1 as libc::c_int) << 7 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    1 as libc::c_int;
        (*_g_7).words.w1 =
            (0xffff as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 16 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 16 as libc::c_int
                |
                (0xffff as libc::c_int as u32_0 &
                     (((0x1 as libc::c_int) << 16 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    0 as libc::c_int;
        let fresh9 = (*__gfxCtx).polyXlu.p;
        (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
        let mut _g_8: *mut Gfx = fresh9;
        (*_g_8).words.w0 =
            (0xe3 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 24 as libc::c_int
                |
                ((32 as libc::c_int - 14 as libc::c_int - 2 as libc::c_int) as
                     u32_0 &
                     (((0x1 as libc::c_int) << 8 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    8 as libc::c_int |
                ((2 as libc::c_int - 1 as libc::c_int) as u32_0 &
                     (((0x1 as libc::c_int) << 8 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    0 as libc::c_int;
        (*_g_8).words.w1 =
            ((0 as libc::c_int) << 14 as libc::c_int) as libc::c_uint;
        let fresh10 = (*__gfxCtx).polyXlu.p;
        (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
        let mut _g_9: *mut Gfx = fresh10;
        (*_g_9).words.w0 =
            (0xfd as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 24 as libc::c_int
                |
                (4 as libc::c_int as u32_0 &
                     (((0x1 as libc::c_int) << 3 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    21 as libc::c_int |
                (2 as libc::c_int as u32_0 &
                     (((0x1 as libc::c_int) << 2 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    19 as libc::c_int |
                ((1 as libc::c_int - 1 as libc::c_int) as u32_0 &
                     (((0x1 as libc::c_int) << 12 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    0 as libc::c_int;
        (*_g_9).words.w1 = sTex.as_mut_ptr() as libc::c_uint;
        let fresh11 = (*__gfxCtx).polyXlu.p;
        (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
        let mut _g_10: *mut Gfx = fresh11;
        (*_g_10).words.w0 =
            (0xf5 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 24 as libc::c_int
                |
                (4 as libc::c_int as u32_0 &
                     (((0x1 as libc::c_int) << 3 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    21 as libc::c_int |
                (2 as libc::c_int as u32_0 &
                     (((0x1 as libc::c_int) << 2 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    19 as libc::c_int |
                (0 as libc::c_int as u32_0 &
                     (((0x1 as libc::c_int) << 9 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    9 as libc::c_int |
                (0 as libc::c_int as u32_0 &
                     (((0x1 as libc::c_int) << 9 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    0 as libc::c_int;
        (*_g_10).words.w1 =
            (7 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 3 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 24 as libc::c_int
                |
                (0 as libc::c_int as u32_0 &
                     (((0x1 as libc::c_int) << 4 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    20 as libc::c_int |
                ((0 as libc::c_int | 0 as libc::c_int) as u32_0 &
                     (((0x1 as libc::c_int) << 2 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    18 as libc::c_int |
                (6 as libc::c_int as u32_0 &
                     (((0x1 as libc::c_int) << 4 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    14 as libc::c_int |
                (0 as libc::c_int as u32_0 &
                     (((0x1 as libc::c_int) << 4 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    10 as libc::c_int |
                ((0 as libc::c_int | 0 as libc::c_int) as u32_0 &
                     (((0x1 as libc::c_int) << 2 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    8 as libc::c_int |
                (6 as libc::c_int as u32_0 &
                     (((0x1 as libc::c_int) << 4 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    4 as libc::c_int |
                (15 as libc::c_int as u32_0 &
                     (((0x1 as libc::c_int) << 4 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    0 as libc::c_int;
        let fresh12 = (*__gfxCtx).polyXlu.p;
        (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
        let mut _g_11: *mut Gfx = fresh12;
        (*_g_11).words.w0 =
            (0xe6 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 24 as libc::c_int;
        (*_g_11).words.w1 = 0 as libc::c_int as libc::c_uint;
        let fresh13 = (*__gfxCtx).polyXlu.p;
        (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
        let mut _g_12: *mut Gfx = fresh13;
        (*_g_12).words.w0 =
            (0xf3 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 24 as libc::c_int
                |
                (0 as libc::c_int as u32_0 &
                     (((0x1 as libc::c_int) << 12 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    12 as libc::c_int |
                (0 as libc::c_int as u32_0 &
                     (((0x1 as libc::c_int) << 12 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    0 as libc::c_int;
        (*_g_12).words.w1 =
            (7 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 3 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 24 as libc::c_int
                |
                ((if ((64 as libc::c_int * 64 as libc::c_int +
                           1 as libc::c_int >> 1 as libc::c_int) -
                          1 as libc::c_int) < 2047 as libc::c_int {
                      (64 as libc::c_int * 64 as libc::c_int +
                           1 as libc::c_int >> 1 as libc::c_int) -
                          1 as libc::c_int
                  } else { 2047 as libc::c_int }) as u32_0 &
                     (((0x1 as libc::c_int) << 12 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    12 as libc::c_int |
                (((((1 as libc::c_int) << 11 as libc::c_int) +
                       (if 1 as libc::c_int >
                               64 as libc::c_int * 1 as libc::c_int /
                                   8 as libc::c_int {
                            1 as libc::c_int
                        } else {
                            (64 as libc::c_int * 1 as libc::c_int) /
                                8 as libc::c_int
                        }) - 1 as libc::c_int) /
                      (if 1 as libc::c_int >
                              64 as libc::c_int * 1 as libc::c_int /
                                  8 as libc::c_int {
                           1 as libc::c_int
                       } else {
                           (64 as libc::c_int * 1 as libc::c_int) /
                               8 as libc::c_int
                       })) as u32_0 &
                     (((0x1 as libc::c_int) << 12 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    0 as libc::c_int;
        let fresh14 = (*__gfxCtx).polyXlu.p;
        (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
        let mut _g_13: *mut Gfx = fresh14;
        (*_g_13).words.w0 =
            (0xe7 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 24 as libc::c_int;
        (*_g_13).words.w1 = 0 as libc::c_int as libc::c_uint;
        let fresh15 = (*__gfxCtx).polyXlu.p;
        (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
        let mut _g_14: *mut Gfx = fresh15;
        (*_g_14).words.w0 =
            (0xf5 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 24 as libc::c_int
                |
                (4 as libc::c_int as u32_0 &
                     (((0x1 as libc::c_int) << 3 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    21 as libc::c_int |
                (1 as libc::c_int as u32_0 &
                     (((0x1 as libc::c_int) << 2 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    19 as libc::c_int |
                ((64 as libc::c_int * 1 as libc::c_int + 7 as libc::c_int >>
                      3 as libc::c_int) as u32_0 &
                     (((0x1 as libc::c_int) << 9 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    9 as libc::c_int |
                (0 as libc::c_int as u32_0 &
                     (((0x1 as libc::c_int) << 9 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    0 as libc::c_int;
        (*_g_14).words.w1 =
            (0 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 3 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 24 as libc::c_int
                |
                (0 as libc::c_int as u32_0 &
                     (((0x1 as libc::c_int) << 4 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    20 as libc::c_int |
                ((0 as libc::c_int | 0 as libc::c_int) as u32_0 &
                     (((0x1 as libc::c_int) << 2 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    18 as libc::c_int |
                (6 as libc::c_int as u32_0 &
                     (((0x1 as libc::c_int) << 4 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    14 as libc::c_int |
                (0 as libc::c_int as u32_0 &
                     (((0x1 as libc::c_int) << 4 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    10 as libc::c_int |
                ((0 as libc::c_int | 0 as libc::c_int) as u32_0 &
                     (((0x1 as libc::c_int) << 2 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    8 as libc::c_int |
                (6 as libc::c_int as u32_0 &
                     (((0x1 as libc::c_int) << 4 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    4 as libc::c_int |
                (15 as libc::c_int as u32_0 &
                     (((0x1 as libc::c_int) << 4 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    0 as libc::c_int;
        let fresh16 = (*__gfxCtx).polyXlu.p;
        (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
        let mut _g_15: *mut Gfx = fresh16;
        (*_g_15).words.w0 =
            (0xf2 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 24 as libc::c_int
                |
                (0 as libc::c_int as u32_0 &
                     (((0x1 as libc::c_int) << 12 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    12 as libc::c_int |
                (0 as libc::c_int as u32_0 &
                     (((0x1 as libc::c_int) << 12 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    0 as libc::c_int;
        (*_g_15).words.w1 =
            (0 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 3 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 24 as libc::c_int
                |
                (((64 as libc::c_int - 1 as libc::c_int) << 2 as libc::c_int)
                     as u32_0 &
                     (((0x1 as libc::c_int) << 12 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    12 as libc::c_int |
                (((64 as libc::c_int - 1 as libc::c_int) << 2 as libc::c_int)
                     as u32_0 &
                     (((0x1 as libc::c_int) << 12 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    0 as libc::c_int;
        let fresh17 = (*__gfxCtx).polyXlu.p;
        (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
        let mut _g_16: *mut Gfx = fresh17;
        (*_g_16).words.w0 =
            (0xf5 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 24 as libc::c_int
                |
                (4 as libc::c_int as u32_0 &
                     (((0x1 as libc::c_int) << 3 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    21 as libc::c_int |
                (1 as libc::c_int as u32_0 &
                     (((0x1 as libc::c_int) << 2 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    19 as libc::c_int |
                (8 as libc::c_int as u32_0 &
                     (((0x1 as libc::c_int) << 9 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    9 as libc::c_int |
                (0 as libc::c_int as u32_0 &
                     (((0x1 as libc::c_int) << 9 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    0 as libc::c_int;
        (*_g_16).words.w1 =
            (1 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 3 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 24 as libc::c_int
                |
                (0 as libc::c_int as u32_0 &
                     (((0x1 as libc::c_int) << 4 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    20 as libc::c_int |
                ((0 as libc::c_int | 0 as libc::c_int) as u32_0 &
                     (((0x1 as libc::c_int) << 2 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    18 as libc::c_int |
                (6 as libc::c_int as u32_0 &
                     (((0x1 as libc::c_int) << 4 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    14 as libc::c_int |
                (14 as libc::c_int as u32_0 &
                     (((0x1 as libc::c_int) << 4 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    10 as libc::c_int |
                ((0 as libc::c_int | 0 as libc::c_int) as u32_0 &
                     (((0x1 as libc::c_int) << 2 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    8 as libc::c_int |
                (6 as libc::c_int as u32_0 &
                     (((0x1 as libc::c_int) << 4 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    4 as libc::c_int |
                (14 as libc::c_int as u32_0 &
                     (((0x1 as libc::c_int) << 4 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    0 as libc::c_int;
        let fresh18 = (*__gfxCtx).polyXlu.p;
        (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
        let mut _g_17: *mut Gfx = fresh18;
        (*_g_17).words.w0 =
            (0xf2 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 24 as libc::c_int
                |
                (0 as libc::c_int as u32_0 &
                     (((0x1 as libc::c_int) << 12 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    12 as libc::c_int |
                (0 as libc::c_int as u32_0 &
                     (((0x1 as libc::c_int) << 12 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    0 as libc::c_int;
        (*_g_17).words.w1 =
            (1 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 3 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 24 as libc::c_int
                |
                (252 as libc::c_int as u32_0 &
                     (((0x1 as libc::c_int) << 12 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    12 as libc::c_int |
                (252 as libc::c_int as u32_0 &
                     (((0x1 as libc::c_int) << 12 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    0 as libc::c_int;
        let fresh19 = (*__gfxCtx).polyXlu.p;
        (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
        let mut _g_18: *mut Gfx = fresh19;
        (*_g_18).words.w0 =
            (0xde as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 24 as libc::c_int
                |
                (0 as libc::c_int as u32_0 &
                     (((0x1 as libc::c_int) << 8 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    16 as libc::c_int |
                (0 as libc::c_int as u32_0 &
                     (((0x1 as libc::c_int) << 16 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    0 as libc::c_int;
        (*_g_18).words.w1 = sMaterialDL.as_mut_ptr() as libc::c_uint;
        let fresh20 = (*__gfxCtx).polyXlu.p;
        (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
        let mut _g_19: *mut Gfx = fresh20;
        (*_g_19).words.w0 =
            (0xde as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 24 as libc::c_int
                |
                (0 as libc::c_int as u32_0 &
                     (((0x1 as libc::c_int) << 8 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    16 as libc::c_int |
                (0 as libc::c_int as u32_0 &
                     (((0x1 as libc::c_int) << 16 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    0 as libc::c_int;
        (*_g_19).words.w1 =
            Gfx_TwoTexScroll((*globalCtx).state.gfxCtx, 0 as libc::c_int,
                             gameplayFrames.wrapping_mul(2 as libc::c_int as
                                                             libc::c_uint).wrapping_rem(512
                                                                                            as
                                                                                            libc::c_int
                                                                                            as
                                                                                            libc::c_uint),
                             (511 as libc::c_int as
                                  libc::c_uint).wrapping_sub(gameplayFrames.wrapping_mul(5
                                                                                             as
                                                                                             libc::c_int
                                                                                             as
                                                                                             libc::c_uint).wrapping_rem(512
                                                                                                                            as
                                                                                                                            libc::c_int
                                                                                                                            as
                                                                                                                            libc::c_uint)),
                             64 as libc::c_int, 64 as libc::c_int,
                             1 as libc::c_int,
                             gameplayFrames.wrapping_mul(2 as libc::c_int as
                                                             libc::c_uint).wrapping_rem(256
                                                                                            as
                                                                                            libc::c_int
                                                                                            as
                                                                                            libc::c_uint),
                             (255 as libc::c_int as
                                  libc::c_uint).wrapping_sub(gameplayFrames.wrapping_mul(20
                                                                                             as
                                                                                             libc::c_int
                                                                                             as
                                                                                             libc::c_uint).wrapping_rem(256
                                                                                                                            as
                                                                                                                            libc::c_int
                                                                                                                            as
                                                                                                                            libc::c_uint)),
                             32 as libc::c_int, 32 as libc::c_int) as
                libc::c_uint;
        let fresh21 = (*__gfxCtx).polyXlu.p;
        (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
        let mut _g_20: *mut Gfx = fresh21;
        (*_g_20).words.w0 =
            (0xde as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 24 as libc::c_int
                |
                (0 as libc::c_int as u32_0 &
                     (((0x1 as libc::c_int) << 8 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    16 as libc::c_int |
                (0 as libc::c_int as u32_0 &
                     (((0x1 as libc::c_int) << 16 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    0 as libc::c_int;
        (*_g_20).words.w1 = sModelDL.as_mut_ptr() as libc::c_uint;
        Graph_CloseDisps(dispRefs.as_mut_ptr(), (*globalCtx).state.gfxCtx,
                         b"../z_magic_fire.c\x00" as *const u8 as
                             *const libc::c_char, 750 as libc::c_int);
        alpha =
            ((*this).alphaMultiplier * 255 as libc::c_int as libc::c_float) as
                s32 as u8_0;
        i = 0 as libc::c_int;
        while i < 36 as libc::c_int {
            sSphereVtx[sVertexIndices[i as usize] as usize].n.a = alpha;
            i += 1
        }
        alpha =
            ((*this).alphaMultiplier * 76 as libc::c_int as libc::c_float) as
                s32 as u8_0;
        i = 36 as libc::c_int;
        while i < 60 as libc::c_int {
            sSphereVtx[sVertexIndices[i as usize] as usize].n.a = alpha;
            i += 1
        }
    };
}
unsafe extern "C" fn run_static_initializers() {
    sModelDL =
        [Gfx{words:
                 {
                     let mut init =
                         Gwords{w0:
                                    (0x1 as libc::c_int as u32_0 &
                                         (((0x1 as libc::c_int) <<
                                               8 as libc::c_int) -
                                              1 as libc::c_int) as
                                             libc::c_uint) <<
                                        24 as libc::c_int |
                                        (32 as libc::c_int as u32_0 &
                                             (((0x1 as libc::c_int) <<
                                                   8 as libc::c_int) -
                                                  1 as libc::c_int) as
                                                 libc::c_uint) <<
                                            12 as libc::c_int |
                                        ((0 as libc::c_int +
                                              32 as libc::c_int) as u32_0 &
                                             (((0x1 as libc::c_int) <<
                                                   7 as libc::c_int) -
                                                  1 as libc::c_int) as
                                                 libc::c_uint) <<
                                            1 as libc::c_int,
                                w1: sSphereVtx.as_mut_ptr() as libc::c_uint,};
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
                                    } else if 0 as libc::c_int ==
                                                  1 as libc::c_int {
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
                                    } else {
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
                                                  ((4 as libc::c_int *
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
                                                  (((4 as libc::c_int *
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
                                              })
                                         }),
                                w1:
                                    if 0 as libc::c_int == 0 as libc::c_int {
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
                                            ((3 as libc::c_int *
                                                  2 as libc::c_int) as u32_0 &
                                                 (((0x1 as libc::c_int) <<
                                                       8 as libc::c_int) -
                                                      1 as libc::c_int) as
                                                     libc::c_uint) <<
                                                0 as libc::c_int
                                    } else if 0 as libc::c_int ==
                                                  1 as libc::c_int {
                                        (((4 as libc::c_int *
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
                                            ((5 as libc::c_int *
                                                  2 as libc::c_int) as u32_0 &
                                                 (((0x1 as libc::c_int) <<
                                                       8 as libc::c_int) -
                                                      1 as libc::c_int) as
                                                     libc::c_uint) <<
                                                0 as libc::c_int
                                    } else {
                                        (((3 as libc::c_int *
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
                                             (((6 as libc::c_int *
                                                    2 as libc::c_int) as u32_0
                                                   &
                                                   (((0x1 as libc::c_int) <<
                                                         8 as libc::c_int) -
                                                        1 as libc::c_int) as
                                                       libc::c_uint) <<
                                                  16 as libc::c_int |
                                                  ((7 as libc::c_int *
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
                                             ((7 as libc::c_int *
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
                                    } else if 0 as libc::c_int ==
                                                  1 as libc::c_int {
                                        (((7 as libc::c_int *
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
                                            ((9 as libc::c_int *
                                                  2 as libc::c_int) as u32_0 &
                                                 (((0x1 as libc::c_int) <<
                                                       8 as libc::c_int) -
                                                      1 as libc::c_int) as
                                                     libc::c_uint) <<
                                                0 as libc::c_int
                                    } else {
                                        (((6 as libc::c_int *
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
                                             (((9 as libc::c_int *
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
                                        (((11 as libc::c_int *
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
                                            ((9 as libc::c_int *
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
                                             ((9 as libc::c_int *
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
                                    } else {
                                        (((9 as libc::c_int *
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
                                             (((11 as libc::c_int *
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
                                                 ((10 as libc::c_int *
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
                                              } else {
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
                                        (((12 as libc::c_int *
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
                                            ((13 as libc::c_int *
                                                  2 as libc::c_int) as u32_0 &
                                                 (((0x1 as libc::c_int) <<
                                                       8 as libc::c_int) -
                                                      1 as libc::c_int) as
                                                     libc::c_uint) <<
                                                0 as libc::c_int
                                    } else if 0 as libc::c_int ==
                                                  1 as libc::c_int {
                                        (((11 as libc::c_int *
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
                                    } else {
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
                                            ((11 as libc::c_int *
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
                                             (((14 as libc::c_int *
                                                    2 as libc::c_int) as u32_0
                                                   &
                                                   (((0x1 as libc::c_int) <<
                                                         8 as libc::c_int) -
                                                        1 as libc::c_int) as
                                                       libc::c_uint) <<
                                                  16 as libc::c_int |
                                                  ((0 as libc::c_int *
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
                                                  (((0 as libc::c_int *
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
                                              })
                                         }),
                                w1:
                                    if 0 as libc::c_int == 0 as libc::c_int {
                                        (((16 as libc::c_int *
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
                                            ((14 as libc::c_int *
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
                                    } else {
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
                                             (((16 as libc::c_int *
                                                    2 as libc::c_int) as u32_0
                                                   &
                                                   (((0x1 as libc::c_int) <<
                                                         8 as libc::c_int) -
                                                        1 as libc::c_int) as
                                                       libc::c_uint) <<
                                                  16 as libc::c_int |
                                                  ((3 as libc::c_int *
                                                        2 as libc::c_int) as
                                                       u32_0 &
                                                       (((0x1 as libc::c_int)
                                                             <<
                                                             8 as libc::c_int)
                                                            -
                                                            1 as libc::c_int)
                                                           as libc::c_uint) <<
                                                      8 as libc::c_int) |
                                                 ((0 as libc::c_int *
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
                                                  (((3 as libc::c_int *
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
                                                  (((0 as libc::c_int *
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
                                              })
                                         }),
                                w1:
                                    if 0 as libc::c_int == 0 as libc::c_int {
                                        (((17 as libc::c_int *
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
                                            ((16 as libc::c_int *
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
                                    } else {
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
                                             (((17 as libc::c_int *
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
                                                 ((3 as libc::c_int *
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
                                                      ((17 as libc::c_int *
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
                                                  (((3 as libc::c_int *
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
                                                       ((17 as libc::c_int *
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
                                        (((18 as libc::c_int *
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
                                            ((17 as libc::c_int *
                                                  2 as libc::c_int) as u32_0 &
                                                 (((0x1 as libc::c_int) <<
                                                       8 as libc::c_int) -
                                                      1 as libc::c_int) as
                                                     libc::c_uint) <<
                                                0 as libc::c_int
                                    } else if 0 as libc::c_int ==
                                                  1 as libc::c_int {
                                        (((5 as libc::c_int *
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
                                            ((18 as libc::c_int *
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
                                             ((18 as libc::c_int *
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
                                             (((18 as libc::c_int *
                                                    2 as libc::c_int) as u32_0
                                                   &
                                                   (((0x1 as libc::c_int) <<
                                                         8 as libc::c_int) -
                                                        1 as libc::c_int) as
                                                       libc::c_uint) <<
                                                  16 as libc::c_int |
                                                  ((19 as libc::c_int *
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
                                                           <<
                                                           8 as libc::c_int) |
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
                                                          << 0 as libc::c_int
                                              })
                                         }),
                                w1:
                                    if 0 as libc::c_int == 0 as libc::c_int {
                                        (((20 as libc::c_int *
                                               2 as libc::c_int) as u32_0 &
                                              (((0x1 as libc::c_int) <<
                                                    8 as libc::c_int) -
                                                   1 as libc::c_int) as
                                                  libc::c_uint) <<
                                             16 as libc::c_int |
                                             ((19 as libc::c_int *
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
                                    } else if 0 as libc::c_int ==
                                                  1 as libc::c_int {
                                        (((19 as libc::c_int *
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
                                    } else {
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
                                            ((19 as libc::c_int *
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
                                             (((21 as libc::c_int *
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
                                                 ((9 as libc::c_int *
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
                                                      ((21 as libc::c_int *
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
                                                  (((9 as libc::c_int *
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
                                                       ((21 as libc::c_int *
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
                                        (((11 as libc::c_int *
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
                                            ((22 as libc::c_int *
                                                  2 as libc::c_int) as u32_0 &
                                                 (((0x1 as libc::c_int) <<
                                                       8 as libc::c_int) -
                                                      1 as libc::c_int) as
                                                     libc::c_uint) <<
                                                0 as libc::c_int
                                    } else if 0 as libc::c_int ==
                                                  1 as libc::c_int {
                                        (((21 as libc::c_int *
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
                                            ((11 as libc::c_int *
                                                  2 as libc::c_int) as u32_0 &
                                                 (((0x1 as libc::c_int) <<
                                                       8 as libc::c_int) -
                                                      1 as libc::c_int) as
                                                     libc::c_uint) <<
                                                0 as libc::c_int
                                    } else {
                                        (((22 as libc::c_int *
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
                                            ((21 as libc::c_int *
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
                                             (((23 as libc::c_int *
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
                                                 ((24 as libc::c_int *
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
                                                       ((24 as libc::c_int *
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
                                                          << 0 as libc::c_int
                                              } else {
                                                  (((24 as libc::c_int *
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
                                        (((25 as libc::c_int *
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
                                            ((23 as libc::c_int *
                                                  2 as libc::c_int) as u32_0 &
                                                 (((0x1 as libc::c_int) <<
                                                       8 as libc::c_int) -
                                                      1 as libc::c_int) as
                                                     libc::c_uint) <<
                                                0 as libc::c_int
                                    } else if 0 as libc::c_int ==
                                                  1 as libc::c_int {
                                        (((14 as libc::c_int *
                                               2 as libc::c_int) as u32_0 &
                                              (((0x1 as libc::c_int) <<
                                                    8 as libc::c_int) -
                                                   1 as libc::c_int) as
                                                  libc::c_uint) <<
                                             16 as libc::c_int |
                                             ((23 as libc::c_int *
                                                   2 as libc::c_int) as u32_0
                                                  &
                                                  (((0x1 as libc::c_int) <<
                                                        8 as libc::c_int) -
                                                       1 as libc::c_int) as
                                                      libc::c_uint) <<
                                                 8 as libc::c_int) |
                                            ((25 as libc::c_int *
                                                  2 as libc::c_int) as u32_0 &
                                                 (((0x1 as libc::c_int) <<
                                                       8 as libc::c_int) -
                                                      1 as libc::c_int) as
                                                     libc::c_uint) <<
                                                0 as libc::c_int
                                    } else {
                                        (((23 as libc::c_int *
                                               2 as libc::c_int) as u32_0 &
                                              (((0x1 as libc::c_int) <<
                                                    8 as libc::c_int) -
                                                   1 as libc::c_int) as
                                                  libc::c_uint) <<
                                             16 as libc::c_int |
                                             ((25 as libc::c_int *
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
                                             (((25 as libc::c_int *
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
                                                      ((25 as libc::c_int *
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
                                                       ((25 as libc::c_int *
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
                                        (((26 as libc::c_int *
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
                                            ((25 as libc::c_int *
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
                                             ((25 as libc::c_int *
                                                   2 as libc::c_int) as u32_0
                                                  &
                                                  (((0x1 as libc::c_int) <<
                                                        8 as libc::c_int) -
                                                       1 as libc::c_int) as
                                                      libc::c_uint) <<
                                                 8 as libc::c_int) |
                                            ((26 as libc::c_int *
                                                  2 as libc::c_int) as u32_0 &
                                                 (((0x1 as libc::c_int) <<
                                                       8 as libc::c_int) -
                                                      1 as libc::c_int) as
                                                     libc::c_uint) <<
                                                0 as libc::c_int
                                    } else {
                                        (((25 as libc::c_int *
                                               2 as libc::c_int) as u32_0 &
                                              (((0x1 as libc::c_int) <<
                                                    8 as libc::c_int) -
                                                   1 as libc::c_int) as
                                                  libc::c_uint) <<
                                             16 as libc::c_int |
                                             ((26 as libc::c_int *
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
                                             (((26 as libc::c_int *
                                                    2 as libc::c_int) as u32_0
                                                   &
                                                   (((0x1 as libc::c_int) <<
                                                         8 as libc::c_int) -
                                                        1 as libc::c_int) as
                                                       libc::c_uint) <<
                                                  16 as libc::c_int |
                                                  ((17 as libc::c_int *
                                                        2 as libc::c_int) as
                                                       u32_0 &
                                                       (((0x1 as libc::c_int)
                                                             <<
                                                             8 as libc::c_int)
                                                            -
                                                            1 as libc::c_int)
                                                           as libc::c_uint) <<
                                                      8 as libc::c_int) |
                                                 ((16 as libc::c_int *
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
                                                  (((17 as libc::c_int *
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
                                                      ((26 as libc::c_int *
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
                                                       ((26 as libc::c_int *
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
                                                      ((17 as libc::c_int *
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
                                        (((27 as libc::c_int *
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
                                            ((26 as libc::c_int *
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
                                             ((26 as libc::c_int *
                                                   2 as libc::c_int) as u32_0
                                                  &
                                                  (((0x1 as libc::c_int) <<
                                                        8 as libc::c_int) -
                                                       1 as libc::c_int) as
                                                      libc::c_uint) <<
                                                 8 as libc::c_int) |
                                            ((27 as libc::c_int *
                                                  2 as libc::c_int) as u32_0 &
                                                 (((0x1 as libc::c_int) <<
                                                       8 as libc::c_int) -
                                                      1 as libc::c_int) as
                                                     libc::c_uint) <<
                                                0 as libc::c_int
                                    } else {
                                        (((26 as libc::c_int *
                                               2 as libc::c_int) as u32_0 &
                                              (((0x1 as libc::c_int) <<
                                                    8 as libc::c_int) -
                                                   1 as libc::c_int) as
                                                  libc::c_uint) <<
                                             16 as libc::c_int |
                                             ((27 as libc::c_int *
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
                                    (0x6 as libc::c_int as u32_0 &
                                         (((0x1 as libc::c_int) <<
                                               8 as libc::c_int) -
                                              1 as libc::c_int) as
                                             libc::c_uint) <<
                                        24 as libc::c_int |
                                        (if 0 as libc::c_int ==
                                                0 as libc::c_int {
                                             (((27 as libc::c_int *
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
                                                 ((17 as libc::c_int *
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
                                                       ((17 as libc::c_int *
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
                                                      ((27 as libc::c_int *
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
                                                  (((17 as libc::c_int *
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
                                                       ((27 as libc::c_int *
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
                                        (((28 as libc::c_int *
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
                                            ((27 as libc::c_int *
                                                  2 as libc::c_int) as u32_0 &
                                                 (((0x1 as libc::c_int) <<
                                                       8 as libc::c_int) -
                                                      1 as libc::c_int) as
                                                     libc::c_uint) <<
                                                0 as libc::c_int
                                    } else if 0 as libc::c_int ==
                                                  1 as libc::c_int {
                                        (((18 as libc::c_int *
                                               2 as libc::c_int) as u32_0 &
                                              (((0x1 as libc::c_int) <<
                                                    8 as libc::c_int) -
                                                   1 as libc::c_int) as
                                                  libc::c_uint) <<
                                             16 as libc::c_int |
                                             ((27 as libc::c_int *
                                                   2 as libc::c_int) as u32_0
                                                  &
                                                  (((0x1 as libc::c_int) <<
                                                        8 as libc::c_int) -
                                                       1 as libc::c_int) as
                                                      libc::c_uint) <<
                                                 8 as libc::c_int) |
                                            ((28 as libc::c_int *
                                                  2 as libc::c_int) as u32_0 &
                                                 (((0x1 as libc::c_int) <<
                                                       8 as libc::c_int) -
                                                      1 as libc::c_int) as
                                                     libc::c_uint) <<
                                                0 as libc::c_int
                                    } else {
                                        (((27 as libc::c_int *
                                               2 as libc::c_int) as u32_0 &
                                              (((0x1 as libc::c_int) <<
                                                    8 as libc::c_int) -
                                                   1 as libc::c_int) as
                                                  libc::c_uint) <<
                                             16 as libc::c_int |
                                             ((28 as libc::c_int *
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
                                             (((28 as libc::c_int *
                                                    2 as libc::c_int) as u32_0
                                                   &
                                                   (((0x1 as libc::c_int) <<
                                                         8 as libc::c_int) -
                                                        1 as libc::c_int) as
                                                       libc::c_uint) <<
                                                  16 as libc::c_int |
                                                  ((20 as libc::c_int *
                                                        2 as libc::c_int) as
                                                       u32_0 &
                                                       (((0x1 as libc::c_int)
                                                             <<
                                                             8 as libc::c_int)
                                                            -
                                                            1 as libc::c_int)
                                                           as libc::c_uint) <<
                                                      8 as libc::c_int) |
                                                 ((18 as libc::c_int *
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
                                                  (((20 as libc::c_int *
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
                                                           <<
                                                           8 as libc::c_int) |
                                                      ((28 as libc::c_int *
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
                                                       ((28 as libc::c_int *
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
                                              })
                                         }),
                                w1:
                                    if 0 as libc::c_int == 0 as libc::c_int {
                                        (((20 as libc::c_int *
                                               2 as libc::c_int) as u32_0 &
                                              (((0x1 as libc::c_int) <<
                                                    8 as libc::c_int) -
                                                   1 as libc::c_int) as
                                                  libc::c_uint) <<
                                             16 as libc::c_int |
                                             ((28 as libc::c_int *
                                                   2 as libc::c_int) as u32_0
                                                  &
                                                  (((0x1 as libc::c_int) <<
                                                        8 as libc::c_int) -
                                                       1 as libc::c_int) as
                                                      libc::c_uint) <<
                                                 8 as libc::c_int) |
                                            ((29 as libc::c_int *
                                                  2 as libc::c_int) as u32_0 &
                                                 (((0x1 as libc::c_int) <<
                                                       8 as libc::c_int) -
                                                      1 as libc::c_int) as
                                                     libc::c_uint) <<
                                                0 as libc::c_int
                                    } else if 0 as libc::c_int ==
                                                  1 as libc::c_int {
                                        (((28 as libc::c_int *
                                               2 as libc::c_int) as u32_0 &
                                              (((0x1 as libc::c_int) <<
                                                    8 as libc::c_int) -
                                                   1 as libc::c_int) as
                                                  libc::c_uint) <<
                                             16 as libc::c_int |
                                             ((29 as libc::c_int *
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
                                    } else {
                                        (((29 as libc::c_int *
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
                                            ((28 as libc::c_int *
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
                                    (0x5 as libc::c_int as u32_0 &
                                         (((0x1 as libc::c_int) <<
                                               8 as libc::c_int) -
                                              1 as libc::c_int) as
                                             libc::c_uint) <<
                                        24 as libc::c_int |
                                        (if 0 as libc::c_int ==
                                                0 as libc::c_int {
                                             (((30 as libc::c_int *
                                                    2 as libc::c_int) as u32_0
                                                   &
                                                   (((0x1 as libc::c_int) <<
                                                         8 as libc::c_int) -
                                                        1 as libc::c_int) as
                                                       libc::c_uint) <<
                                                  16 as libc::c_int |
                                                  ((23 as libc::c_int *
                                                        2 as libc::c_int) as
                                                       u32_0 &
                                                       (((0x1 as libc::c_int)
                                                             <<
                                                             8 as libc::c_int)
                                                            -
                                                            1 as libc::c_int)
                                                           as libc::c_uint) <<
                                                      8 as libc::c_int) |
                                                 ((31 as libc::c_int *
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
                                                       ((31 as libc::c_int *
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
                                                      ((30 as libc::c_int *
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
                                                  (((31 as libc::c_int *
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
                                                       ((30 as libc::c_int *
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
                                                          << 0 as libc::c_int
                                              })
                                         }),
                                w1: 0 as libc::c_int as libc::c_uint,};
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
                                        (32 as libc::c_int as u32_0 &
                                             (((0x1 as libc::c_int) <<
                                                   8 as libc::c_int) -
                                                  1 as libc::c_int) as
                                                 libc::c_uint) <<
                                            12 as libc::c_int |
                                        ((0 as libc::c_int +
                                              32 as libc::c_int) as u32_0 &
                                             (((0x1 as libc::c_int) <<
                                                   7 as libc::c_int) -
                                                  1 as libc::c_int) as
                                                 libc::c_uint) <<
                                            1 as libc::c_int,
                                w1:
                                    &mut *sSphereVtx.as_mut_ptr().offset(32 as
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
                                                  ((3 as libc::c_int *
                                                        2 as libc::c_int) as
                                                       u32_0 &
                                                       (((0x1 as libc::c_int)
                                                             <<
                                                             8 as libc::c_int)
                                                            -
                                                            1 as libc::c_int)
                                                           as libc::c_uint) <<
                                                      8 as libc::c_int) |
                                                 ((0 as libc::c_int *
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
                                                  (((3 as libc::c_int *
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
                                                  (((0 as libc::c_int *
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
                                             ((5 as libc::c_int *
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
                                    } else if 0 as libc::c_int ==
                                                  1 as libc::c_int {
                                        (((5 as libc::c_int *
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
                                    } else {
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
                                            ((5 as libc::c_int *
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
                                             (((6 as libc::c_int *
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
                                                 ((4 as libc::c_int *
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
                                              } else {
                                                  (((4 as libc::c_int *
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
                                        (((6 as libc::c_int *
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
                                            ((6 as libc::c_int *
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
                                             ((6 as libc::c_int *
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
                                             (((8 as libc::c_int *
                                                    2 as libc::c_int) as u32_0
                                                   &
                                                   (((0x1 as libc::c_int) <<
                                                         8 as libc::c_int) -
                                                        1 as libc::c_int) as
                                                       libc::c_uint) <<
                                                  16 as libc::c_int |
                                                  ((7 as libc::c_int *
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
                                              })
                                         }),
                                w1:
                                    if 0 as libc::c_int == 0 as libc::c_int {
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
                                            ((7 as libc::c_int *
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
                                             ((7 as libc::c_int *
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
                                    } else {
                                        (((7 as libc::c_int *
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
                                                 ((10 as libc::c_int *
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
                                        (((11 as libc::c_int *
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
                                            ((12 as libc::c_int *
                                                  2 as libc::c_int) as u32_0 &
                                                 (((0x1 as libc::c_int) <<
                                                       8 as libc::c_int) -
                                                      1 as libc::c_int) as
                                                     libc::c_uint) <<
                                                0 as libc::c_int
                                    } else if 0 as libc::c_int ==
                                                  1 as libc::c_int {
                                        (((2 as libc::c_int *
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
                                            ((11 as libc::c_int *
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
                                             ((11 as libc::c_int *
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
                                        (((13 as libc::c_int *
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
                                            ((13 as libc::c_int *
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
                                             ((13 as libc::c_int *
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
                                             (((14 as libc::c_int *
                                                    2 as libc::c_int) as u32_0
                                                   &
                                                   (((0x1 as libc::c_int) <<
                                                         8 as libc::c_int) -
                                                        1 as libc::c_int) as
                                                       libc::c_uint) <<
                                                  16 as libc::c_int |
                                                  ((0 as libc::c_int *
                                                        2 as libc::c_int) as
                                                       u32_0 &
                                                       (((0x1 as libc::c_int)
                                                             <<
                                                             8 as libc::c_int)
                                                            -
                                                            1 as libc::c_int)
                                                           as libc::c_uint) <<
                                                      8 as libc::c_int) |
                                                 ((13 as libc::c_int *
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
                                                  (((0 as libc::c_int *
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
                                              } else {
                                                  (((13 as libc::c_int *
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
                                             ((4 as libc::c_int *
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
                                    } else if 0 as libc::c_int ==
                                                  1 as libc::c_int {
                                        (((4 as libc::c_int *
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
                                            ((14 as libc::c_int *
                                                  2 as libc::c_int) as u32_0 &
                                                 (((0x1 as libc::c_int) <<
                                                       8 as libc::c_int) -
                                                      1 as libc::c_int) as
                                                     libc::c_uint) <<
                                                0 as libc::c_int
                                    } else {
                                        (((0 as libc::c_int *
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
                                            ((4 as libc::c_int *
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
                                                  ((4 as libc::c_int *
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
                                                  (((4 as libc::c_int *
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
                                             ((6 as libc::c_int *
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
                                        (((6 as libc::c_int *
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
                                            ((15 as libc::c_int *
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
                                             ((15 as libc::c_int *
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
                                        (((16 as libc::c_int *
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
                                            ((6 as libc::c_int *
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
                                             ((6 as libc::c_int *
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
                                    } else {
                                        (((6 as libc::c_int *
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
                                                 ((17 as libc::c_int *
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
                                                       ((17 as libc::c_int *
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
                                                  (((17 as libc::c_int *
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
                                        (((18 as libc::c_int *
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
                                            ((19 as libc::c_int *
                                                  2 as libc::c_int) as u32_0 &
                                                 (((0x1 as libc::c_int) <<
                                                       8 as libc::c_int) -
                                                      1 as libc::c_int) as
                                                     libc::c_uint) <<
                                                0 as libc::c_int
                                    } else if 0 as libc::c_int ==
                                                  1 as libc::c_int {
                                        (((11 as libc::c_int *
                                               2 as libc::c_int) as u32_0 &
                                              (((0x1 as libc::c_int) <<
                                                    8 as libc::c_int) -
                                                   1 as libc::c_int) as
                                                  libc::c_uint) <<
                                             16 as libc::c_int |
                                             ((19 as libc::c_int *
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
                                        (((19 as libc::c_int *
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
                                            ((11 as libc::c_int *
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
                                                 ((18 as libc::c_int *
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
                                        (((20 as libc::c_int *
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
                                            ((11 as libc::c_int *
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
                                             ((11 as libc::c_int *
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
                                    } else {
                                        (((11 as libc::c_int *
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
                                             (((21 as libc::c_int *
                                                    2 as libc::c_int) as u32_0
                                                   &
                                                   (((0x1 as libc::c_int) <<
                                                         8 as libc::c_int) -
                                                        1 as libc::c_int) as
                                                       libc::c_uint) <<
                                                  16 as libc::c_int |
                                                  ((13 as libc::c_int *
                                                        2 as libc::c_int) as
                                                       u32_0 &
                                                       (((0x1 as libc::c_int)
                                                             <<
                                                             8 as libc::c_int)
                                                            -
                                                            1 as libc::c_int)
                                                           as libc::c_uint) <<
                                                      8 as libc::c_int) |
                                                 ((20 as libc::c_int *
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
                                                  (((13 as libc::c_int *
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
                                                      ((21 as libc::c_int *
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
                                                  (((20 as libc::c_int *
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
                                                       ((21 as libc::c_int *
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
                                              })
                                         }),
                                w1:
                                    if 0 as libc::c_int == 0 as libc::c_int {
                                        (((21 as libc::c_int *
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
                                            ((13 as libc::c_int *
                                                  2 as libc::c_int) as u32_0 &
                                                 (((0x1 as libc::c_int) <<
                                                       8 as libc::c_int) -
                                                      1 as libc::c_int) as
                                                     libc::c_uint) <<
                                                0 as libc::c_int
                                    } else if 0 as libc::c_int ==
                                                  1 as libc::c_int {
                                        (((14 as libc::c_int *
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
                                            ((21 as libc::c_int *
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
                                             ((21 as libc::c_int *
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
                                             (((22 as libc::c_int *
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
                                                 ((21 as libc::c_int *
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
                                                       ((21 as libc::c_int *
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
                                              } else {
                                                  (((21 as libc::c_int *
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
                                        (((22 as libc::c_int *
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
                                            ((22 as libc::c_int *
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
                                             ((22 as libc::c_int *
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
                                             (((23 as libc::c_int *
                                                    2 as libc::c_int) as u32_0
                                                   &
                                                   (((0x1 as libc::c_int) <<
                                                         8 as libc::c_int) -
                                                        1 as libc::c_int) as
                                                       libc::c_uint) <<
                                                  16 as libc::c_int |
                                                  ((15 as libc::c_int *
                                                        2 as libc::c_int) as
                                                       u32_0 &
                                                       (((0x1 as libc::c_int)
                                                             <<
                                                             8 as libc::c_int)
                                                            -
                                                            1 as libc::c_int)
                                                           as libc::c_uint) <<
                                                      8 as libc::c_int) |
                                                 ((22 as libc::c_int *
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
                                                           <<
                                                           8 as libc::c_int) |
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
                                                          << 0 as libc::c_int
                                              } else {
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
                                              })
                                         }),
                                w1:
                                    if 0 as libc::c_int == 0 as libc::c_int {
                                        (((23 as libc::c_int *
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
                                            ((15 as libc::c_int *
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
                                             ((15 as libc::c_int *
                                                   2 as libc::c_int) as u32_0
                                                  &
                                                  (((0x1 as libc::c_int) <<
                                                        8 as libc::c_int) -
                                                       1 as libc::c_int) as
                                                      libc::c_uint) <<
                                                 8 as libc::c_int) |
                                            ((23 as libc::c_int *
                                                  2 as libc::c_int) as u32_0 &
                                                 (((0x1 as libc::c_int) <<
                                                       8 as libc::c_int) -
                                                      1 as libc::c_int) as
                                                     libc::c_uint) <<
                                                0 as libc::c_int
                                    } else {
                                        (((15 as libc::c_int *
                                               2 as libc::c_int) as u32_0 &
                                              (((0x1 as libc::c_int) <<
                                                    8 as libc::c_int) -
                                                   1 as libc::c_int) as
                                                  libc::c_uint) <<
                                             16 as libc::c_int |
                                             ((23 as libc::c_int *
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
                                                  ((23 as libc::c_int *
                                                        2 as libc::c_int) as
                                                       u32_0 &
                                                       (((0x1 as libc::c_int)
                                                             <<
                                                             8 as libc::c_int)
                                                            -
                                                            1 as libc::c_int)
                                                           as libc::c_uint) <<
                                                      8 as libc::c_int) |
                                                 ((24 as libc::c_int *
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
                                                       ((24 as libc::c_int *
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
                                                  (((24 as libc::c_int *
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
                                                          << 0 as libc::c_int
                                              })
                                         }),
                                w1:
                                    if 0 as libc::c_int == 0 as libc::c_int {
                                        (((25 as libc::c_int *
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
                                            ((26 as libc::c_int *
                                                  2 as libc::c_int) as u32_0 &
                                                 (((0x1 as libc::c_int) <<
                                                       8 as libc::c_int) -
                                                      1 as libc::c_int) as
                                                     libc::c_uint) <<
                                                0 as libc::c_int
                                    } else if 0 as libc::c_int ==
                                                  1 as libc::c_int {
                                        (((18 as libc::c_int *
                                               2 as libc::c_int) as u32_0 &
                                              (((0x1 as libc::c_int) <<
                                                    8 as libc::c_int) -
                                                   1 as libc::c_int) as
                                                  libc::c_uint) <<
                                             16 as libc::c_int |
                                             ((26 as libc::c_int *
                                                   2 as libc::c_int) as u32_0
                                                  &
                                                  (((0x1 as libc::c_int) <<
                                                        8 as libc::c_int) -
                                                       1 as libc::c_int) as
                                                      libc::c_uint) <<
                                                 8 as libc::c_int) |
                                            ((25 as libc::c_int *
                                                  2 as libc::c_int) as u32_0 &
                                                 (((0x1 as libc::c_int) <<
                                                       8 as libc::c_int) -
                                                      1 as libc::c_int) as
                                                     libc::c_uint) <<
                                                0 as libc::c_int
                                    } else {
                                        (((26 as libc::c_int *
                                               2 as libc::c_int) as u32_0 &
                                              (((0x1 as libc::c_int) <<
                                                    8 as libc::c_int) -
                                                   1 as libc::c_int) as
                                                  libc::c_uint) <<
                                             16 as libc::c_int |
                                             ((25 as libc::c_int *
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
                                             (((27 as libc::c_int *
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
                                                 ((25 as libc::c_int *
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
                                                       ((25 as libc::c_int *
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
                                                      ((27 as libc::c_int *
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
                                                  (((25 as libc::c_int *
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
                                                       ((27 as libc::c_int *
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
                                        (((27 as libc::c_int *
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
                                            ((18 as libc::c_int *
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
                                             ((18 as libc::c_int *
                                                   2 as libc::c_int) as u32_0
                                                  &
                                                  (((0x1 as libc::c_int) <<
                                                        8 as libc::c_int) -
                                                       1 as libc::c_int) as
                                                      libc::c_uint) <<
                                                 8 as libc::c_int) |
                                            ((27 as libc::c_int *
                                                  2 as libc::c_int) as u32_0 &
                                                 (((0x1 as libc::c_int) <<
                                                       8 as libc::c_int) -
                                                      1 as libc::c_int) as
                                                     libc::c_uint) <<
                                                0 as libc::c_int
                                    } else {
                                        (((18 as libc::c_int *
                                               2 as libc::c_int) as u32_0 &
                                              (((0x1 as libc::c_int) <<
                                                    8 as libc::c_int) -
                                                   1 as libc::c_int) as
                                                  libc::c_uint) <<
                                             16 as libc::c_int |
                                             ((27 as libc::c_int *
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
                                             (((28 as libc::c_int *
                                                    2 as libc::c_int) as u32_0
                                                   &
                                                   (((0x1 as libc::c_int) <<
                                                         8 as libc::c_int) -
                                                        1 as libc::c_int) as
                                                       libc::c_uint) <<
                                                  16 as libc::c_int |
                                                  ((20 as libc::c_int *
                                                        2 as libc::c_int) as
                                                       u32_0 &
                                                       (((0x1 as libc::c_int)
                                                             <<
                                                             8 as libc::c_int)
                                                            -
                                                            1 as libc::c_int)
                                                           as libc::c_uint) <<
                                                      8 as libc::c_int) |
                                                 ((27 as libc::c_int *
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
                                                  (((20 as libc::c_int *
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
                                                       ((27 as libc::c_int *
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
                                                      ((28 as libc::c_int *
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
                                                  (((27 as libc::c_int *
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
                                                       ((28 as libc::c_int *
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
                                              })
                                         }),
                                w1:
                                    if 0 as libc::c_int == 0 as libc::c_int {
                                        (((28 as libc::c_int *
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
                                            ((20 as libc::c_int *
                                                  2 as libc::c_int) as u32_0 &
                                                 (((0x1 as libc::c_int) <<
                                                       8 as libc::c_int) -
                                                      1 as libc::c_int) as
                                                     libc::c_uint) <<
                                                0 as libc::c_int
                                    } else if 0 as libc::c_int ==
                                                  1 as libc::c_int {
                                        (((21 as libc::c_int *
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
                                            ((28 as libc::c_int *
                                                  2 as libc::c_int) as u32_0 &
                                                 (((0x1 as libc::c_int) <<
                                                       8 as libc::c_int) -
                                                      1 as libc::c_int) as
                                                     libc::c_uint) <<
                                                0 as libc::c_int
                                    } else {
                                        (((20 as libc::c_int *
                                               2 as libc::c_int) as u32_0 &
                                              (((0x1 as libc::c_int) <<
                                                    8 as libc::c_int) -
                                                   1 as libc::c_int) as
                                                  libc::c_uint) <<
                                             16 as libc::c_int |
                                             ((28 as libc::c_int *
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
                                             (((29 as libc::c_int *
                                                    2 as libc::c_int) as u32_0
                                                   &
                                                   (((0x1 as libc::c_int) <<
                                                         8 as libc::c_int) -
                                                        1 as libc::c_int) as
                                                       libc::c_uint) <<
                                                  16 as libc::c_int |
                                                  ((21 as libc::c_int *
                                                        2 as libc::c_int) as
                                                       u32_0 &
                                                       (((0x1 as libc::c_int)
                                                             <<
                                                             8 as libc::c_int)
                                                            -
                                                            1 as libc::c_int)
                                                           as libc::c_uint) <<
                                                      8 as libc::c_int) |
                                                 ((28 as libc::c_int *
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
                                                  (((21 as libc::c_int *
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
                                                       ((28 as libc::c_int *
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
                                                      ((29 as libc::c_int *
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
                                                  (((28 as libc::c_int *
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
                                                       ((29 as libc::c_int *
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
                                                      ((21 as libc::c_int *
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
                                        (((29 as libc::c_int *
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
                                            ((21 as libc::c_int *
                                                  2 as libc::c_int) as u32_0 &
                                                 (((0x1 as libc::c_int) <<
                                                       8 as libc::c_int) -
                                                      1 as libc::c_int) as
                                                     libc::c_uint) <<
                                                0 as libc::c_int
                                    } else if 0 as libc::c_int ==
                                                  1 as libc::c_int {
                                        (((22 as libc::c_int *
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
                                            ((29 as libc::c_int *
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
                                             ((29 as libc::c_int *
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
                                             (((30 as libc::c_int *
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
                                                 ((29 as libc::c_int *
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
                                                       ((29 as libc::c_int *
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
                                                      ((30 as libc::c_int *
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
                                                  (((29 as libc::c_int *
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
                                                       ((30 as libc::c_int *
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
                                        (((30 as libc::c_int *
                                               2 as libc::c_int) as u32_0 &
                                              (((0x1 as libc::c_int) <<
                                                    8 as libc::c_int) -
                                                   1 as libc::c_int) as
                                                  libc::c_uint) <<
                                             16 as libc::c_int |
                                             ((23 as libc::c_int *
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
                                    } else if 0 as libc::c_int ==
                                                  1 as libc::c_int {
                                        (((23 as libc::c_int *
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
                                            ((30 as libc::c_int *
                                                  2 as libc::c_int) as u32_0 &
                                                 (((0x1 as libc::c_int) <<
                                                       8 as libc::c_int) -
                                                      1 as libc::c_int) as
                                                     libc::c_uint) <<
                                                0 as libc::c_int
                                    } else {
                                        (((22 as libc::c_int *
                                               2 as libc::c_int) as u32_0 &
                                              (((0x1 as libc::c_int) <<
                                                    8 as libc::c_int) -
                                                   1 as libc::c_int) as
                                                  libc::c_uint) <<
                                             16 as libc::c_int |
                                             ((30 as libc::c_int *
                                                   2 as libc::c_int) as u32_0
                                                  &
                                                  (((0x1 as libc::c_int) <<
                                                        8 as libc::c_int) -
                                                       1 as libc::c_int) as
                                                      libc::c_uint) <<
                                                 8 as libc::c_int) |
                                            ((23 as libc::c_int *
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
                                    (0x5 as libc::c_int as u32_0 &
                                         (((0x1 as libc::c_int) <<
                                               8 as libc::c_int) -
                                              1 as libc::c_int) as
                                             libc::c_uint) <<
                                        24 as libc::c_int |
                                        (if 0 as libc::c_int ==
                                                0 as libc::c_int {
                                             (((23 as libc::c_int *
                                                    2 as libc::c_int) as u32_0
                                                   &
                                                   (((0x1 as libc::c_int) <<
                                                         8 as libc::c_int) -
                                                        1 as libc::c_int) as
                                                       libc::c_uint) <<
                                                  16 as libc::c_int |
                                                  ((30 as libc::c_int *
                                                        2 as libc::c_int) as
                                                       u32_0 &
                                                       (((0x1 as libc::c_int)
                                                             <<
                                                             8 as libc::c_int)
                                                            -
                                                            1 as libc::c_int)
                                                           as libc::c_uint) <<
                                                      8 as libc::c_int) |
                                                 ((31 as libc::c_int *
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
                                                  (((30 as libc::c_int *
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
                                                       ((31 as libc::c_int *
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
                                                          << 0 as libc::c_int
                                              } else {
                                                  (((31 as libc::c_int *
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
                                                      ((30 as libc::c_int *
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
                                w1: 0 as libc::c_int as libc::c_uint,};
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
                                        (12 as libc::c_int as u32_0 &
                                             (((0x1 as libc::c_int) <<
                                                   8 as libc::c_int) -
                                                  1 as libc::c_int) as
                                                 libc::c_uint) <<
                                            12 as libc::c_int |
                                        ((0 as libc::c_int +
                                              12 as libc::c_int) as u32_0 &
                                             (((0x1 as libc::c_int) <<
                                                   7 as libc::c_int) -
                                                  1 as libc::c_int) as
                                                 libc::c_uint) <<
                                            1 as libc::c_int,
                                w1:
                                    &mut *sSphereVtx.as_mut_ptr().offset(64 as
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
                                    } else if 0 as libc::c_int ==
                                                  1 as libc::c_int {
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
                                    } else {
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
                                                  ((4 as libc::c_int *
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
                                                  (((4 as libc::c_int *
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
                                              })
                                         }),
                                w1:
                                    if 0 as libc::c_int == 0 as libc::c_int {
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
                                            ((3 as libc::c_int *
                                                  2 as libc::c_int) as u32_0 &
                                                 (((0x1 as libc::c_int) <<
                                                       8 as libc::c_int) -
                                                      1 as libc::c_int) as
                                                     libc::c_uint) <<
                                                0 as libc::c_int
                                    } else if 0 as libc::c_int ==
                                                  1 as libc::c_int {
                                        (((4 as libc::c_int *
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
                                            ((5 as libc::c_int *
                                                  2 as libc::c_int) as u32_0 &
                                                 (((0x1 as libc::c_int) <<
                                                       8 as libc::c_int) -
                                                      1 as libc::c_int) as
                                                     libc::c_uint) <<
                                                0 as libc::c_int
                                    } else {
                                        (((3 as libc::c_int *
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
                                                 ((4 as libc::c_int *
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
                                              } else {
                                                  (((4 as libc::c_int *
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
                                        (((7 as libc::c_int *
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
                                            ((5 as libc::c_int *
                                                  2 as libc::c_int) as u32_0 &
                                                 (((0x1 as libc::c_int) <<
                                                       8 as libc::c_int) -
                                                      1 as libc::c_int) as
                                                     libc::c_uint) <<
                                                0 as libc::c_int
                                    } else if 0 as libc::c_int ==
                                                  1 as libc::c_int {
                                        (((6 as libc::c_int *
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
                                            ((7 as libc::c_int *
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
                                             ((7 as libc::c_int *
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
                                            ((7 as libc::c_int *
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
                                             ((7 as libc::c_int *
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
                                        (((7 as libc::c_int *
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
                                             ((9 as libc::c_int *
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
                                        (((9 as libc::c_int *
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
                                            ((10 as libc::c_int *
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
                                             ((10 as libc::c_int *
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
                                    (0xdf as libc::c_int as u32_0 &
                                         (((0x1 as libc::c_int) <<
                                               8 as libc::c_int) -
                                              1 as libc::c_int) as
                                             libc::c_uint) <<
                                        24 as libc::c_int,
                                w1: 0 as libc::c_int as libc::c_uint,};
                     init
                 },}];
    sInitChain =
        [{
             let mut init = InitChainEntry{cont_type_0_offset_value: [0; 4],};
             init.set_cont(0 as libc::c_int as u32_0);
             init.set_type_0(ICHAINTYPE_VEC3F as libc::c_int as u32_0);
             init.set_offset(&mut (*(0 as *mut Actor)).scale as *mut Vec3f as
                                 size_t as u32_0);
             init.set_value(0 as libc::c_int);
             init
         }]
}
#[used]
#[cfg_attr(target_os = "linux", link_section = ".init_array")]
#[cfg_attr(target_os = "windows", link_section = ".CRT$XIB")]
#[cfg_attr(target_os = "macos", link_section = "__DATA,__mod_init_func")]
static INIT_ARRAY: [unsafe extern "C" fn(); 1] = [run_static_initializers];