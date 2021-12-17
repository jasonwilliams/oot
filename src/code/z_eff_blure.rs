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
    fn __assert(exp: *const libc::c_char, file: *const libc::c_char,
                line: s32);
    #[no_mangle]
    fn osSyncPrintf(fmt: *const libc::c_char, _: ...);
    #[no_mangle]
    fn func_80027E34(arg0: s16, arg1: s16, arg2: f32_0) -> s16;
    #[no_mangle]
    fn func_80027E84(arg0: u8_0, arg1: u8_0, arg2: f32_0) -> u8_0;
    #[no_mangle]
    fn Math_Vec3s_ToVec3f(dest: *mut Vec3f, src: *mut Vec3s);
    #[no_mangle]
    fn Math_Vec3f_Diff(a: *mut Vec3f, b: *mut Vec3f, dest: *mut Vec3f);
    #[no_mangle]
    fn Math_Vec3s_DiffToVec3f(dest: *mut Vec3f, a: *mut Vec3s, b: *mut Vec3s);
    #[no_mangle]
    fn Math_Vec3f_Scale(vec: *mut Vec3f, scaleF: f32_0);
    #[no_mangle]
    fn Color_RGBA8_Copy(dst: *mut Color_RGBA8, src: *mut Color_RGBA8);
    #[no_mangle]
    fn Gfx_CallSetupDL(gfx: *mut Gfx, i: u32_0) -> *mut Gfx;
    #[no_mangle]
    fn func_800942F0(gfxCtx: *mut GraphicsContext);
    #[no_mangle]
    fn SkinMatrix_Vec3fMtxFMultXYZ(mf: *mut MtxF, src: *mut Vec3f,
                                   dest: *mut Vec3f);
    #[no_mangle]
    fn SkinMatrix_MtxFMtxFMult(mfA: *mut MtxF, mfB: *mut MtxF,
                               dest: *mut MtxF);
    #[no_mangle]
    fn SkinMatrix_SetScale(mf: *mut MtxF, x: f32_0, y: f32_0, z: f32_0);
    #[no_mangle]
    fn SkinMatrix_SetTranslate(mf: *mut MtxF, x: f32_0, y: f32_0, z: f32_0);
    #[no_mangle]
    fn SkinMatrix_MtxFToNewMtx(gfxCtx: *mut GraphicsContext, src: *mut MtxF)
     -> *mut Mtx;
    #[no_mangle]
    fn func_800A7EC0(mf: *mut MtxF, a: s16, x: f32_0, y: f32_0, z: f32_0);
    #[no_mangle]
    fn Graph_Alloc(gfxCtx: *mut GraphicsContext, size: size_t)
     -> *mut libc::c_void;
    #[no_mangle]
    fn Graph_OpenDisps(dispRefs: *mut *mut Gfx, gfxCtx: *mut GraphicsContext,
                       file: *const libc::c_char, line: s32);
    #[no_mangle]
    fn Graph_CloseDisps(dispRefs: *mut *mut Gfx, gfxCtx: *mut GraphicsContext,
                        file: *const libc::c_char, line: s32);
    #[no_mangle]
    fn Math3D_CosOut(a: *mut Vec3f, b: *mut Vec3f, dst: *mut f32_0) -> s32;
    #[no_mangle]
    fn Math3D_Vec3fMagnitude(vec: *mut Vec3f) -> f32_0;
    #[no_mangle]
    static mut gMtxClear: Mtx;
    #[no_mangle]
    fn Math_FNearbyIntF(x: f32_0) -> f32_0;
    #[no_mangle]
    static mut gUnknownEffBlureTex: [u64_0; 0];
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
pub struct Color_RGBA8 {
    pub r: u8_0,
    pub g: u8_0,
    pub b: u8_0,
    pub a: u8_0,
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
pub struct EffectBlureElement {
    pub state: s32,
    pub timer: s32,
    pub p1: Vec3s,
    pub p2: Vec3s,
    pub flags: u16_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct EffectBlureInit1 {
    pub unk_00: [libc::c_char; 388],
    pub p1StartColor: [u8_0; 4],
    pub p2StartColor: [u8_0; 4],
    pub p1EndColor: [u8_0; 4],
    pub p2EndColor: [u8_0; 4],
    pub elemDuration: s32,
    pub unkFlag: s32,
    pub calcMode: s32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct EffectBlureInit2 {
    pub calcMode: s32,
    pub flags: u16_0,
    pub addAngleChange: s16,
    pub p1StartColor: [u8_0; 4],
    pub p2StartColor: [u8_0; 4],
    pub p1EndColor: [u8_0; 4],
    pub p2EndColor: [u8_0; 4],
    pub elemDuration: u8_0,
    pub unkFlag: u8_0,
    pub drawMode: u8_0,
    pub mode4Param: u8_0,
    pub altPrimColor: Color_RGBA8,
    pub altEnvColor: Color_RGBA8,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct EffectBlure {
    pub elements: [EffectBlureElement; 16],
    pub calcMode: s32,
    pub mode4Param: f32_0,
    pub flags: u16_0,
    pub addAngleChange: s16,
    pub addAngle: s16,
    pub p1StartColor: Color_RGBA8,
    pub p2StartColor: Color_RGBA8,
    pub p1EndColor: Color_RGBA8,
    pub p2EndColor: Color_RGBA8,
    pub numElements: u8_0,
    pub elemDuration: u8_0,
    pub unkFlag: u8_0,
    pub drawMode: u8_0,
    pub altPrimColor: Color_RGBA8,
    pub altEnvColor: Color_RGBA8,
}
#[no_mangle]
pub unsafe extern "C" fn EffectBlure_AddVertex(mut this: *mut EffectBlure,
                                               mut p1: *mut Vec3f,
                                               mut p2: *mut Vec3f) {
    let mut elem: *mut EffectBlureElement = 0 as *mut EffectBlureElement;
    let mut numElements: s32 = 0;
    let mut sp16C: Vec3f = Vec3f{x: 0., y: 0., z: 0.,};
    let mut sp160: Vec3f = Vec3f{x: 0., y: 0., z: 0.,};
    let mut sp154: Vec3f = Vec3f{x: 0., y: 0., z: 0.,};
    let mut scale: f32_0 = 0.;
    let mut sp110: MtxF = MtxF{mf: [[0.; 4]; 4],};
    let mut spD0: MtxF = MtxF{mf: [[0.; 4]; 4],};
    let mut sp90: MtxF = MtxF{mf: [[0.; 4]; 4],};
    let mut sp50: MtxF = MtxF{mf: [[0.; 4]; 4],};
    let mut sp44: Vec3f = Vec3f{x: 0., y: 0., z: 0.,};
    let mut sp38: Vec3f = Vec3f{x: 0., y: 0., z: 0.,};
    if !this.is_null() {
        numElements = (*this).numElements as s32;
        if numElements >= 16 as libc::c_int {
            // "Blure vertex addition processing: Table over %d"
            osSyncPrintf(b"\xe3\x83\x96\xe3\x83\xa9\xe2\x94\x80\xe9\xa0\x82\xe7\x82\xb9\xe8\xbf\xbd\xe5\x8a\xa0\xe5\x87\xa6\xe7\x90\x86:\xe3\x83\x86\xe3\x83\xbc\xe3\x83\x96\xe3\x83\xab\xe3\x82\xaa\xe3\x83\xbc\xe3\x83\x90\xe3\x83\xbc %d\n\x00"
                             as *const u8 as *const libc::c_char,
                         numElements);
            return
        }
        elem =
            &mut *(*this).elements.as_mut_ptr().offset(numElements as isize)
                as *mut EffectBlureElement;
        (*elem).state = 1 as libc::c_int;
        if (*this).flags as libc::c_int & 2 as libc::c_int == 0 {
            (*elem).p1.x = (*p1).x as s16;
            (*elem).p1.y = (*p1).y as s16;
            (*elem).p1.z = (*p1).z as s16;
            (*elem).p2.x = (*p2).x as s16;
            (*elem).p2.y = (*p2).y as s16;
            (*elem).p2.z = (*p2).z as s16
        } else {
            sp16C.x =
                ((*elem.offset(-(1 as libc::c_int as isize))).p2.x as f32_0 +
                     (*elem.offset(-(1 as libc::c_int as isize))).p1.x as
                         f32_0) * 0.5f32;
            sp16C.y =
                ((*elem.offset(-(1 as libc::c_int as isize))).p2.y as f32_0 +
                     (*elem.offset(-(1 as libc::c_int as isize))).p1.y as
                         f32_0) * 0.5f32;
            sp16C.z =
                ((*elem.offset(-(1 as libc::c_int as isize))).p2.z as f32_0 +
                     (*elem.offset(-(1 as libc::c_int as isize))).p1.z as
                         f32_0) * 0.5f32;
            sp160.x = ((*p1).x + (*p2).x) * 0.5f32;
            sp160.y = ((*p1).y + (*p2).y) * 0.5f32;
            sp160.z = ((*p1).z + (*p2).z) * 0.5f32;
            Math_Vec3f_Diff(&mut sp160, &mut sp16C, &mut sp154);
            scale = Math3D_Vec3fMagnitude(&mut sp154);
            if !(fabsf(scale) < 0.008f32) {
                scale = 1.0f32 / scale;
                Math_Vec3f_Scale(&mut sp154, scale);
                SkinMatrix_SetTranslate(&mut sp110, sp160.x, sp160.y,
                                        sp160.z);
                func_800A7EC0(&mut spD0, (*this).addAngle, sp154.x, sp154.y,
                              sp154.z);
                SkinMatrix_MtxFMtxFMult(&mut sp110, &mut spD0, &mut sp90);
                SkinMatrix_SetTranslate(&mut sp110, -sp160.x, -sp160.y,
                                        -sp160.z);
                SkinMatrix_MtxFMtxFMult(&mut sp90, &mut sp110, &mut sp50);
                SkinMatrix_Vec3fMtxFMultXYZ(&mut sp50, p1, &mut sp38);
                SkinMatrix_Vec3fMtxFMultXYZ(&mut sp50, p2, &mut sp44);
                (*elem).p1.x = sp38.x as s16;
                (*elem).p1.y = sp38.y as s16;
                (*elem).p1.z = sp38.z as s16;
                (*elem).p2.x = sp44.x as s16;
                (*elem).p2.y = sp44.y as s16;
                (*elem).p2.z = sp44.z as s16
            }
        }
        (*elem).timer = 1 as libc::c_int;
        (*this).numElements = (*this).numElements.wrapping_add(1)
    };
}
#[no_mangle]
pub unsafe extern "C" fn EffectBlure_AddSpace(mut this: *mut EffectBlure) {
    let mut elem: *mut EffectBlureElement = 0 as *mut EffectBlureElement;
    let mut numElements: s32 = 0;
    if !this.is_null() {
        numElements = (*this).numElements as s32;
        if numElements >= 16 as libc::c_int {
            // "Blure space addition processing: Table over %d"
            osSyncPrintf(b"\xe3\x83\x96\xe3\x83\xa9\xe2\x94\x80\xe7\xa9\xba\xe7\x99\xbd\xe8\xbf\xbd\xe5\x8a\xa0\xe5\x87\xa6\xe7\x90\x86:\xe3\x83\x86\xe3\x83\xbc\xe3\x83\x96\xe3\x83\xab\xe3\x82\xaa\xe3\x83\xbc\xe3\x83\x90\xe3\x83\xbc %d\n\x00"
                             as *const u8 as *const libc::c_char,
                         numElements);
            return
        }
        elem =
            &mut *(*this).elements.as_mut_ptr().offset(numElements as isize)
                as *mut EffectBlureElement;
        (*elem).state = 0 as libc::c_int;
        (*elem).timer = 1 as libc::c_int;
        (*this).numElements = (*this).numElements.wrapping_add(1)
    };
}
#[no_mangle]
pub unsafe extern "C" fn EffectBlure_InitElements(mut this:
                                                      *mut EffectBlure) {
    let mut elem: *mut EffectBlureElement = 0 as *mut EffectBlureElement;
    let mut i: s32 = 0;
    (*this).numElements = 0 as libc::c_int as u8_0;
    i = 0 as libc::c_int;
    while i < 16 as libc::c_int {
        elem =
            &mut *(*this).elements.as_mut_ptr().offset(i as isize) as
                *mut EffectBlureElement;
        (*elem).state = 2 as libc::c_int;
        (*elem).p1.x = 0 as libc::c_int as s16;
        (*elem).p1.y = 0 as libc::c_int as s16;
        (*elem).p1.z = 0 as libc::c_int as s16;
        (*elem).p2.x = 0 as libc::c_int as s16;
        (*elem).p2.y = 0 as libc::c_int as s16;
        (*elem).p2.z = 0 as libc::c_int as s16;
        (*elem).timer = 0 as libc::c_int;
        (*elem).flags = 0 as libc::c_int as u16_0;
        i += 1
    };
}
#[no_mangle]
pub unsafe extern "C" fn EffectBlure_Init1(mut thisx: *mut libc::c_void,
                                           mut initParamsx:
                                               *mut libc::c_void) {
    let mut this: *mut EffectBlure = thisx as *mut EffectBlure;
    let mut initParams: *mut EffectBlureInit1 =
        initParamsx as *mut EffectBlureInit1;
    if !this.is_null() && !initParams.is_null() {
        EffectBlure_InitElements(this);
        (*this).p1StartColor.r =
            (*initParams).p1StartColor[0 as libc::c_int as usize];
        (*this).p2StartColor.r =
            (*initParams).p2StartColor[0 as libc::c_int as usize];
        (*this).p1EndColor.r =
            (*initParams).p1EndColor[0 as libc::c_int as usize];
        (*this).p2EndColor.r =
            (*initParams).p2EndColor[0 as libc::c_int as usize];
        (*this).p1StartColor.g =
            (*initParams).p1StartColor[1 as libc::c_int as usize];
        (*this).p2StartColor.g =
            (*initParams).p2StartColor[1 as libc::c_int as usize];
        (*this).p1EndColor.g =
            (*initParams).p1EndColor[1 as libc::c_int as usize];
        (*this).p2EndColor.g =
            (*initParams).p2EndColor[1 as libc::c_int as usize];
        (*this).p1StartColor.b =
            (*initParams).p1StartColor[2 as libc::c_int as usize];
        (*this).p2StartColor.b =
            (*initParams).p2StartColor[2 as libc::c_int as usize];
        (*this).p1EndColor.b =
            (*initParams).p1EndColor[2 as libc::c_int as usize];
        (*this).p2EndColor.b =
            (*initParams).p2EndColor[2 as libc::c_int as usize];
        (*this).p1StartColor.a =
            (*initParams).p1StartColor[3 as libc::c_int as usize];
        (*this).p2StartColor.a =
            (*initParams).p2StartColor[3 as libc::c_int as usize];
        (*this).p1EndColor.a =
            (*initParams).p1EndColor[3 as libc::c_int as usize];
        (*this).p2EndColor.a =
            (*initParams).p2EndColor[3 as libc::c_int as usize];
        (*this).elemDuration = (*initParams).elemDuration as u8_0;
        (*this).unkFlag = (*initParams).unkFlag as u8_0;
        (*this).calcMode = (*initParams).calcMode;
        (*this).flags = 0 as libc::c_int as u16_0;
        (*this).addAngleChange = 0 as libc::c_int as s16;
        (*this).addAngle = 0 as libc::c_int as s16;
        (*this).drawMode = 0 as libc::c_int as u8_0;
        (*this).altPrimColor.r = 0 as libc::c_int as u8_0;
        (*this).altPrimColor.g = 0 as libc::c_int as u8_0;
        (*this).altPrimColor.b = 0 as libc::c_int as u8_0;
        (*this).altPrimColor.a = 0 as libc::c_int as u8_0;
        (*this).altEnvColor.r = 0 as libc::c_int as u8_0;
        (*this).altEnvColor.g = 0 as libc::c_int as u8_0;
        (*this).altEnvColor.b = 0 as libc::c_int as u8_0;
        (*this).altEnvColor.a = 0 as libc::c_int as u8_0;
        (*this).mode4Param = 1.0f32
    };
}
#[no_mangle]
pub unsafe extern "C" fn EffectBlure_Init2(mut thisx: *mut libc::c_void,
                                           mut initParamsx:
                                               *mut libc::c_void) {
    let mut this: *mut EffectBlure = thisx as *mut EffectBlure;
    let mut initParams: *mut EffectBlureInit2 =
        initParamsx as *mut EffectBlureInit2;
    if !this.is_null() && !initParams.is_null() {
        EffectBlure_InitElements(this);
        (*this).p1StartColor.r =
            (*initParams).p1StartColor[0 as libc::c_int as usize];
        (*this).p2StartColor.r =
            (*initParams).p2StartColor[0 as libc::c_int as usize];
        (*this).p1EndColor.r =
            (*initParams).p1EndColor[0 as libc::c_int as usize];
        (*this).p2EndColor.r =
            (*initParams).p2EndColor[0 as libc::c_int as usize];
        (*this).p1StartColor.g =
            (*initParams).p1StartColor[1 as libc::c_int as usize];
        (*this).p2StartColor.g =
            (*initParams).p2StartColor[1 as libc::c_int as usize];
        (*this).p1EndColor.g =
            (*initParams).p1EndColor[1 as libc::c_int as usize];
        (*this).p2EndColor.g =
            (*initParams).p2EndColor[1 as libc::c_int as usize];
        (*this).p1StartColor.b =
            (*initParams).p1StartColor[2 as libc::c_int as usize];
        (*this).p2StartColor.b =
            (*initParams).p2StartColor[2 as libc::c_int as usize];
        (*this).p1EndColor.b =
            (*initParams).p1EndColor[2 as libc::c_int as usize];
        (*this).p2EndColor.b =
            (*initParams).p2EndColor[2 as libc::c_int as usize];
        (*this).p1StartColor.a =
            (*initParams).p1StartColor[3 as libc::c_int as usize];
        (*this).p2StartColor.a =
            (*initParams).p2StartColor[3 as libc::c_int as usize];
        (*this).p1EndColor.a =
            (*initParams).p1EndColor[3 as libc::c_int as usize];
        (*this).p2EndColor.a =
            (*initParams).p2EndColor[3 as libc::c_int as usize];
        (*this).elemDuration = (*initParams).elemDuration;
        (*this).unkFlag = (*initParams).unkFlag;
        (*this).calcMode = (*initParams).calcMode;
        (*this).flags = (*initParams).flags;
        (*this).drawMode = (*initParams).drawMode;
        (*this).addAngleChange = (*initParams).addAngleChange;
        (*this).addAngle = 0 as libc::c_int as s16;
        (*this).mode4Param = (*initParams).mode4Param as f32_0;
        (*this).altPrimColor = (*initParams).altPrimColor;
        (*this).altEnvColor = (*initParams).altEnvColor
    };
}
#[no_mangle]
pub unsafe extern "C" fn EffectBlure_Destroy(mut thisx: *mut libc::c_void) { }
#[no_mangle]
pub unsafe extern "C" fn EffectBlure_Update(mut thisx: *mut libc::c_void)
 -> s32 {
    let mut this: *mut EffectBlure = thisx as *mut EffectBlure;
    let mut i: s32 = 0;
    if this.is_null() { return 0 as libc::c_int }
    if (*this).numElements as libc::c_int == 0 as libc::c_int {
        return 0 as libc::c_int
    }
    while (*this).elements[0 as libc::c_int as usize].state ==
              0 as libc::c_int {
        i = 0 as libc::c_int;
        while i < 15 as libc::c_int {
            (*this).elements[i as usize] =
                (*this).elements[(i + 1 as libc::c_int) as usize];
            i += 1
        }
        (*this).elements[i as usize].state = 2 as libc::c_int;
        (*this).elements[i as usize].p1.x = 0 as libc::c_int as s16;
        (*this).elements[i as usize].p1.y = 0 as libc::c_int as s16;
        (*this).elements[i as usize].p1.z = 0 as libc::c_int as s16;
        (*this).elements[i as usize].p2.x = 0 as libc::c_int as s16;
        (*this).elements[i as usize].p2.y = 0 as libc::c_int as s16;
        (*this).elements[i as usize].p2.z = 0 as libc::c_int as s16;
        (*this).elements[i as usize].flags = 0 as libc::c_int as u16_0;
        (*this).elements[i as usize].timer = 0 as libc::c_int;
        (*this).numElements = (*this).numElements.wrapping_sub(1);
        if (*this).numElements as libc::c_int <= 0 as libc::c_int {
            (*this).numElements = 0 as libc::c_int as u8_0;
            return 0 as libc::c_int
        }
    }
    if (*this).elements[0 as libc::c_int as usize].state == 2 as libc::c_int {
        return 0 as libc::c_int
    }
    i = 0 as libc::c_int;
    while i < (*this).numElements as libc::c_int {
        (*this).elements[i as usize].timer += 1;
        i += 1
    }
    if ((*this).elemDuration as libc::c_int) <
           (*this).elements[0 as libc::c_int as usize].timer {
        i = 0 as libc::c_int;
        while i < 15 as libc::c_int {
            (*this).elements[i as usize] =
                (*this).elements[(i + 1 as libc::c_int) as usize];
            i += 1
        }
        (*this).elements[i as usize].state = 2 as libc::c_int;
        (*this).elements[i as usize].p1.x = 0 as libc::c_int as s16;
        (*this).elements[i as usize].p1.y = 0 as libc::c_int as s16;
        (*this).elements[i as usize].p1.z = 0 as libc::c_int as s16;
        (*this).elements[i as usize].p2.x = 0 as libc::c_int as s16;
        (*this).elements[i as usize].p2.y = 0 as libc::c_int as s16;
        (*this).elements[i as usize].p2.z = 0 as libc::c_int as s16;
        (*this).elements[i as usize].flags = 0 as libc::c_int as u16_0;
        (*this).elements[i as usize].timer = 0 as libc::c_int;
        (*this).numElements = (*this).numElements.wrapping_sub(1);
        if (*this).numElements as libc::c_int <= 0 as libc::c_int {
            (*this).numElements = 0 as libc::c_int as u8_0;
            return 0 as libc::c_int
        }
        return 0 as libc::c_int
    }
    (*this).addAngle =
        ((*this).addAngle as libc::c_int +
             (*this).addAngleChange as libc::c_int) as s16;
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn EffectBlure_UpdateFlags(mut elem:
                                                     *mut EffectBlureElement) {
    let mut sp64: Vec3f = Vec3f{x: 0., y: 0., z: 0.,};
    let mut sp58: Vec3f = Vec3f{x: 0., y: 0., z: 0.,};
    let mut sp4C: Vec3f = Vec3f{x: 0., y: 0., z: 0.,};
    let mut sp40: Vec3f = Vec3f{x: 0., y: 0., z: 0.,};
    if (*elem.offset(-(1 as libc::c_int as isize))).state == 0 as libc::c_int
           ||
           (*elem.offset(1 as libc::c_int as isize)).state == 0 as libc::c_int
       {
        (*elem).flags =
            ((*elem).flags as libc::c_int & !(3 as libc::c_int)) as u16_0;
        (*elem).flags =
            ((*elem).flags as libc::c_int | 2 as libc::c_int) as u16_0
    } else {
        let mut prev: *mut EffectBlureElement =
            elem.offset(-(1 as libc::c_int as isize));
        let mut next: *mut EffectBlureElement =
            elem.offset(1 as libc::c_int as isize);
        let mut sp34: f32_0 = 0.;
        let mut sp30: f32_0 = 0.;
        let mut sp2C: f32_0 = 0.;
        // Necessary to match
        Math_Vec3s_DiffToVec3f(&mut sp64, &mut (*elem).p1, &mut (*prev).p1);
        Math_Vec3s_DiffToVec3f(&mut sp58, &mut (*elem).p2, &mut (*prev).p2);
        Math_Vec3s_DiffToVec3f(&mut sp4C, &mut (*next).p1, &mut (*elem).p1);
        Math_Vec3s_DiffToVec3f(&mut sp40, &mut (*next).p2, &mut (*elem).p2);
        if Math3D_CosOut(&mut sp64, &mut sp4C, &mut sp34) != 0 ||
               Math3D_CosOut(&mut sp58, &mut sp40, &mut sp30) != 0 ||
               Math3D_CosOut(&mut sp4C, &mut sp40, &mut sp2C) != 0 {
            (*elem).flags =
                ((*elem).flags as libc::c_int & !(3 as libc::c_int)) as u16_0;
            (*elem).flags =
                ((*elem).flags as libc::c_int | 0 as libc::c_int) as u16_0
        } else if sp34 <= -0.5f32 || sp30 <= -0.5f32 || sp2C <= 0.7071f32 {
            // cos(45 degrees)
            (*elem).flags =
                ((*elem).flags as libc::c_int & !(3 as libc::c_int)) as u16_0;
            (*elem).flags =
                ((*elem).flags as libc::c_int | 0 as libc::c_int) as u16_0
        } else {
            (*elem).flags =
                ((*elem).flags as libc::c_int & !(3 as libc::c_int)) as u16_0;
            (*elem).flags =
                ((*elem).flags as libc::c_int | 1 as libc::c_int) as u16_0
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn EffectBlure_GetComputedValues(mut this:
                                                           *mut EffectBlure,
                                                       mut index: s32,
                                                       mut ratio: f32_0,
                                                       mut vec1: *mut Vec3s,
                                                       mut vec2: *mut Vec3s,
                                                       mut color1:
                                                           *mut Color_RGBA8,
                                                       mut color2:
                                                           *mut Color_RGBA8) {
    let mut sp30: Vec3s = Vec3s{x: 0, y: 0, z: 0,};
    let mut mode4Param: f32_0 = 0.;
    let mut elem: *mut EffectBlureElement =
        &mut *(*this).elements.as_mut_ptr().offset(index as isize) as
            *mut EffectBlureElement;
    match (*this).calcMode {
        1 => {
            (*vec1).x = func_80027E34((*elem).p1.x, (*elem).p2.x, ratio);
            (*vec1).y = func_80027E34((*elem).p1.y, (*elem).p2.y, ratio);
            (*vec1).z = func_80027E34((*elem).p1.z, (*elem).p2.z, ratio);
            (*vec2).x = (*elem).p2.x;
            (*vec2).y = (*elem).p2.y;
            (*vec2).z = (*elem).p2.z
        }
        2 => {
            (*vec1).x = (*elem).p1.x;
            (*vec1).y = (*elem).p1.y;
            (*vec1).z = (*elem).p1.z;
            (*vec2).x = func_80027E34((*elem).p2.x, (*elem).p1.x, ratio);
            (*vec2).y = func_80027E34((*elem).p2.y, (*elem).p1.y, ratio);
            (*vec2).z = func_80027E34((*elem).p2.z, (*elem).p1.z, ratio)
        }
        3 => {
            ratio *= 0.5f32;
            (*vec1).x = func_80027E34((*elem).p1.x, (*elem).p2.x, ratio);
            (*vec1).y = func_80027E34((*elem).p1.y, (*elem).p2.y, ratio);
            (*vec1).z = func_80027E34((*elem).p1.z, (*elem).p2.z, ratio);
            (*vec2).x = func_80027E34((*elem).p2.x, (*elem).p1.x, ratio);
            (*vec2).y = func_80027E34((*elem).p2.y, (*elem).p1.y, ratio);
            (*vec2).z = func_80027E34((*elem).p2.z, (*elem).p1.z, ratio);
            ratio *= 2.0f32
        }
        4 => {
            sp30.x =
                ((*elem).p1.x as libc::c_int - (*elem).p2.x as libc::c_int) as
                    s16;
            sp30.y =
                ((*elem).p1.y as libc::c_int - (*elem).p2.y as libc::c_int) as
                    s16;
            sp30.z =
                ((*elem).p1.z as libc::c_int - (*elem).p2.z as libc::c_int) as
                    s16;
            mode4Param = (*this).mode4Param - 1.0f32;
            (*vec1).x =
                (sp30.x as libc::c_int as libc::c_float * 0.5f32 * mode4Param
                     * ratio + (*elem).p1.x as libc::c_int as libc::c_float)
                    as s16;
            (*vec1).y =
                (sp30.y as libc::c_int as libc::c_float * 0.5f32 * mode4Param
                     * ratio + (*elem).p1.y as libc::c_int as libc::c_float)
                    as s16;
            // Necessary to match
            (*vec1).z =
                (sp30.z as libc::c_int as libc::c_float * 0.5f32 * mode4Param
                     * ratio + (*elem).p1.z as libc::c_int as libc::c_float)
                    as
                    s16; // Optimized out but seems necessary to match stack usage
            (*vec2).x =
                (-(sp30.x as libc::c_int as libc::c_float * 0.5f32 *
                       mode4Param * ratio) +
                     (*elem).p2.x as libc::c_int as libc::c_float) as s16;
            (*vec2).y =
                (-(sp30.y as libc::c_int as libc::c_float * 0.5f32 *
                       mode4Param * ratio) +
                     (*elem).p2.y as libc::c_int as libc::c_float) as s16;
            (*vec2).z =
                (-(sp30.z as libc::c_int as libc::c_float * 0.5f32 *
                       mode4Param * ratio) +
                     (*elem).p2.z as libc::c_int as libc::c_float) as s16
        }
        0 | _ => {
            (*vec1).x = (*elem).p1.x;
            (*vec1).y = (*elem).p1.y;
            (*vec1).z = (*elem).p1.z;
            (*vec2).x = (*elem).p2.x;
            (*vec2).y = (*elem).p2.y;
            (*vec2).z = (*elem).p2.z
        }
    }
    sp30 = sp30;
    if (*this).flags as libc::c_int & 0x10 as libc::c_int != 0 {
        (*color1).a = 255 as libc::c_int as u8_0;
        (*color1).b = (*color1).a;
        (*color1).g = (*color1).b;
        (*color1).r = (*color1).g;
        (*color2).a = 255 as libc::c_int as u8_0;
        (*color2).b = (*color2).a;
        (*color2).g = (*color2).b;
        (*color2).r = (*color2).g
    } else {
        (*color1).r =
            func_80027E84((*this).p1StartColor.r, (*this).p1EndColor.r,
                          ratio);
        (*color1).g =
            func_80027E84((*this).p1StartColor.g, (*this).p1EndColor.g,
                          ratio);
        (*color1).b =
            func_80027E84((*this).p1StartColor.b, (*this).p1EndColor.b,
                          ratio);
        (*color1).a =
            func_80027E84((*this).p1StartColor.a, (*this).p1EndColor.a,
                          ratio);
        (*color2).r =
            func_80027E84((*this).p2StartColor.r, (*this).p2EndColor.r,
                          ratio);
        (*color2).g =
            func_80027E84((*this).p2StartColor.g, (*this).p2EndColor.g,
                          ratio);
        (*color2).b =
            func_80027E84((*this).p2StartColor.b, (*this).p2EndColor.b,
                          ratio);
        (*color2).a =
            func_80027E84((*this).p2StartColor.a, (*this).p2EndColor.a, ratio)
    };
}
#[no_mangle]
pub unsafe extern "C" fn EffectBlure_SetupSmooth(mut this: *mut EffectBlure,
                                                 mut gfxCtx:
                                                     *mut GraphicsContext) {
    let mut __gfxCtx: *mut GraphicsContext = 0 as *mut GraphicsContext;
    let mut dispRefs: [*mut Gfx; 4] = [0 as *mut Gfx; 4];
    __gfxCtx = gfxCtx;
    Graph_OpenDisps(dispRefs.as_mut_ptr(), gfxCtx,
                    b"../z_eff_blure.c\x00" as *const u8 as
                        *const libc::c_char, 809 as libc::c_int);
    (*__gfxCtx).polyXlu.p =
        Gfx_CallSetupDL((*__gfxCtx).polyXlu.p, 0x26 as libc::c_int as u32_0);
    Graph_CloseDisps(dispRefs.as_mut_ptr(), gfxCtx,
                     b"../z_eff_blure.c\x00" as *const u8 as
                         *const libc::c_char, 813 as libc::c_int);
}
// original name: "SQ_NoInterpolate_disp"
#[no_mangle]
pub unsafe extern "C" fn EffectBlure_DrawElemNoInterpolation(mut this:
                                                                 *mut EffectBlure,
                                                             mut elem:
                                                                 *mut EffectBlureElement,
                                                             mut index: s32,
                                                             mut gfxCtx:
                                                                 *mut GraphicsContext) {
    static mut baseVtx: Vtx_t =
        {
            let mut init =
                Vtx_t{ob:
                          [0 as libc::c_int as libc::c_short,
                           0 as libc::c_int as libc::c_short,
                           0 as libc::c_int as libc::c_short],
                      flag: 0 as libc::c_int as libc::c_ushort,
                      tc:
                          [0 as libc::c_int as libc::c_short,
                           0 as libc::c_int as libc::c_short],
                      cn:
                          [255 as libc::c_int as libc::c_uchar,
                           255 as libc::c_int as libc::c_uchar,
                           255 as libc::c_int as libc::c_uchar,
                           255 as libc::c_int as libc::c_uchar],};
            init
        };
    let mut vtx: *mut Vtx = 0 as *mut Vtx;
    let mut sp8C: Vec3s = Vec3s{x: 0, y: 0, z: 0,};
    let mut sp84: Vec3s = Vec3s{x: 0, y: 0, z: 0,};
    let mut ratio: f32_0 = 0.;
    let mut sp7C: Color_RGBA8 = Color_RGBA8{r: 0, g: 0, b: 0, a: 0,};
    let mut sp78: Color_RGBA8 = Color_RGBA8{r: 0, g: 0, b: 0, a: 0,};
    let mut sp6C: Vec3f = Vec3f{x: 0., y: 0., z: 0.,};
    let mut sp60: Vec3f = Vec3f{x: 0., y: 0., z: 0.,};
    let mut sp54: Vec3f = Vec3f{x: 0., y: 0., z: 0.,};
    let mut __gfxCtx: *mut GraphicsContext = 0 as *mut GraphicsContext;
    let mut dispRefs: [*mut Gfx; 4] = [0 as *mut Gfx; 4];
    __gfxCtx = gfxCtx;
    Graph_OpenDisps(dispRefs.as_mut_ptr(), gfxCtx,
                    b"../z_eff_blure.c\x00" as *const u8 as
                        *const libc::c_char, 838 as libc::c_int);
    Math_Vec3s_ToVec3f(&mut sp6C,
                       &mut (*(*this).elements.as_mut_ptr().offset(0 as
                                                                       libc::c_int
                                                                       as
                                                                       isize)).p2);
    vtx =
        Graph_Alloc(gfxCtx,
                    ::std::mem::size_of::<[Vtx; 4]>() as libc::c_ulong as
                        size_t) as *mut Vtx;
    if vtx.is_null() {
        // "Vertices cannot be secured."
        osSyncPrintf(b"z_eff_blure.c::SQ_NoInterpolate_disp() \xe9\xa0\x82\xe7\x82\xb9\xe7\xa2\xba\xe4\xbf\x9d\xe3\x81\xa7\xe3\x81\x8d\xe3\x81\x9a\xe3\x80\x82\n\x00"
                         as *const u8 as *const libc::c_char);
    } else {
        (*vtx.offset(0 as libc::c_int as isize)).v = baseVtx;
        (*vtx.offset(1 as libc::c_int as isize)).v = baseVtx;
        (*vtx.offset(2 as libc::c_int as isize)).v = baseVtx;
        (*vtx.offset(3 as libc::c_int as isize)).v = baseVtx;
        ratio = (*elem).timer as f32_0 / (*this).elemDuration as f32_0;
        EffectBlure_GetComputedValues(this, index, ratio, &mut sp8C,
                                      &mut sp84, &mut sp7C, &mut sp78);
        sp60.x = sp84.x as f32_0;
        sp60.y = sp84.y as f32_0;
        sp60.z = sp84.z as f32_0;
        Math_Vec3f_Diff(&mut sp60, &mut sp6C, &mut sp54);
        Math_Vec3f_Scale(&mut sp54, 10.0f32);
        (*vtx.offset(0 as libc::c_int as
                         isize)).v.ob[0 as libc::c_int as usize] =
            sp54.x as libc::c_short;
        (*vtx.offset(0 as libc::c_int as
                         isize)).v.ob[1 as libc::c_int as usize] =
            sp54.y as libc::c_short;
        (*vtx.offset(0 as libc::c_int as
                         isize)).v.ob[2 as libc::c_int as usize] =
            sp54.z as libc::c_short;
        (*vtx.offset(0 as libc::c_int as
                         isize)).v.cn[0 as libc::c_int as usize] = sp78.r;
        (*vtx.offset(0 as libc::c_int as
                         isize)).v.cn[1 as libc::c_int as usize] = sp78.g;
        (*vtx.offset(0 as libc::c_int as
                         isize)).v.cn[2 as libc::c_int as usize] = sp78.b;
        (*vtx.offset(0 as libc::c_int as
                         isize)).v.cn[3 as libc::c_int as usize] = sp78.a;
        // Necessary to match
        sp60.x = sp8C.x as f32_0;
        sp60.y = sp8C.y as f32_0;
        sp60.z = sp8C.z as f32_0;
        Math_Vec3f_Diff(&mut sp60, &mut sp6C, &mut sp54);
        Math_Vec3f_Scale(&mut sp54, 10.0f32);
        (*vtx.offset(1 as libc::c_int as
                         isize)).v.ob[0 as libc::c_int as usize] =
            sp54.x as libc::c_short;
        (*vtx.offset(1 as libc::c_int as
                         isize)).v.ob[1 as libc::c_int as usize] =
            sp54.y as libc::c_short;
        (*vtx.offset(1 as libc::c_int as
                         isize)).v.ob[2 as libc::c_int as usize] =
            sp54.z as libc::c_short;
        (*vtx.offset(1 as libc::c_int as
                         isize)).v.cn[0 as libc::c_int as usize] = sp7C.r;
        (*vtx.offset(1 as libc::c_int as
                         isize)).v.cn[1 as libc::c_int as usize] = sp7C.g;
        (*vtx.offset(1 as libc::c_int as
                         isize)).v.cn[2 as libc::c_int as usize] = sp7C.b;
        (*vtx.offset(1 as libc::c_int as
                         isize)).v.cn[3 as libc::c_int as usize] = sp7C.a;
        ratio =
            (*elem.offset(1 as libc::c_int as isize)).timer as f32_0 /
                (*this).elemDuration as f32_0;
        EffectBlure_GetComputedValues(this, index + 1 as libc::c_int, ratio,
                                      &mut sp8C, &mut sp84, &mut sp7C,
                                      &mut sp78);
        sp60.x = sp8C.x as f32_0;
        sp60.y = sp8C.y as f32_0;
        sp60.z = sp8C.z as f32_0;
        Math_Vec3f_Diff(&mut sp60, &mut sp6C, &mut sp54);
        Math_Vec3f_Scale(&mut sp54, 10.0f32);
        (*vtx.offset(2 as libc::c_int as
                         isize)).v.ob[0 as libc::c_int as usize] =
            sp54.x as libc::c_short;
        (*vtx.offset(2 as libc::c_int as
                         isize)).v.ob[1 as libc::c_int as usize] =
            sp54.y as libc::c_short;
        (*vtx.offset(2 as libc::c_int as
                         isize)).v.ob[2 as libc::c_int as usize] =
            sp54.z as libc::c_short;
        (*vtx.offset(2 as libc::c_int as
                         isize)).v.cn[0 as libc::c_int as usize] = sp7C.r;
        (*vtx.offset(2 as libc::c_int as
                         isize)).v.cn[1 as libc::c_int as usize] = sp7C.g;
        (*vtx.offset(2 as libc::c_int as
                         isize)).v.cn[2 as libc::c_int as usize] = sp7C.b;
        (*vtx.offset(2 as libc::c_int as
                         isize)).v.cn[3 as libc::c_int as usize] = sp7C.a;
        // Necessary to match
        sp60.x = sp84.x as f32_0;
        sp60.y = sp84.y as f32_0;
        sp60.z = sp84.z as f32_0;
        Math_Vec3f_Diff(&mut sp60, &mut sp6C, &mut sp54);
        Math_Vec3f_Scale(&mut sp54, 10.0f32);
        (*vtx.offset(3 as libc::c_int as
                         isize)).v.ob[0 as libc::c_int as usize] =
            sp54.x as libc::c_short;
        (*vtx.offset(3 as libc::c_int as
                         isize)).v.ob[1 as libc::c_int as usize] =
            sp54.y as libc::c_short;
        (*vtx.offset(3 as libc::c_int as
                         isize)).v.ob[2 as libc::c_int as usize] =
            sp54.z as libc::c_short;
        (*vtx.offset(3 as libc::c_int as
                         isize)).v.cn[0 as libc::c_int as usize] = sp78.r;
        (*vtx.offset(3 as libc::c_int as
                         isize)).v.cn[1 as libc::c_int as usize] = sp78.g;
        (*vtx.offset(3 as libc::c_int as
                         isize)).v.cn[2 as libc::c_int as usize] = sp78.b;
        (*vtx.offset(3 as libc::c_int as
                         isize)).v.cn[3 as libc::c_int as usize] = sp78.a;
        let fresh0 = (*__gfxCtx).polyXlu.p;
        (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
        let mut _g: *mut Gfx = fresh0;
        (*_g).words.w0 =
            (0x1 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 24 as libc::c_int
                |
                (4 as libc::c_int as u32_0 &
                     (((0x1 as libc::c_int) << 8 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    12 as libc::c_int |
                ((0 as libc::c_int + 4 as libc::c_int) as u32_0 &
                     (((0x1 as libc::c_int) << 7 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    1 as libc::c_int;
        (*_g).words.w1 = vtx as libc::c_uint;
        let fresh1 = (*__gfxCtx).polyXlu.p;
        (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
        let mut _g_0: *mut Gfx = fresh1;
        (*_g_0).words.w0 =
            (0x6 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 24 as libc::c_int
                |
                (if 0 as libc::c_int == 0 as libc::c_int {
                     (((0 as libc::c_int * 2 as libc::c_int) as u32_0 &
                           (((0x1 as libc::c_int) << 8 as libc::c_int) -
                                1 as libc::c_int) as libc::c_uint) <<
                          16 as libc::c_int |
                          ((1 as libc::c_int * 2 as libc::c_int) as u32_0 &
                               (((0x1 as libc::c_int) << 8 as libc::c_int) -
                                    1 as libc::c_int) as libc::c_uint) <<
                              8 as libc::c_int) |
                         ((2 as libc::c_int * 2 as libc::c_int) as u32_0 &
                              (((0x1 as libc::c_int) << 8 as libc::c_int) -
                                   1 as libc::c_int) as libc::c_uint) <<
                             0 as libc::c_int
                 } else {
                     (if 0 as libc::c_int == 1 as libc::c_int {
                          (((1 as libc::c_int * 2 as libc::c_int) as u32_0 &
                                (((0x1 as libc::c_int) << 8 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               16 as libc::c_int |
                               ((2 as libc::c_int * 2 as libc::c_int) as u32_0
                                    &
                                    (((0x1 as libc::c_int) <<
                                          8 as libc::c_int) -
                                         1 as libc::c_int) as libc::c_uint) <<
                                   8 as libc::c_int) |
                              ((0 as libc::c_int * 2 as libc::c_int) as u32_0
                                   &
                                   (((0x1 as libc::c_int) << 8 as libc::c_int)
                                        - 1 as libc::c_int) as libc::c_uint)
                                  << 0 as libc::c_int
                      } else {
                          (((2 as libc::c_int * 2 as libc::c_int) as u32_0 &
                                (((0x1 as libc::c_int) << 8 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               16 as libc::c_int |
                               ((0 as libc::c_int * 2 as libc::c_int) as u32_0
                                    &
                                    (((0x1 as libc::c_int) <<
                                          8 as libc::c_int) -
                                         1 as libc::c_int) as libc::c_uint) <<
                                   8 as libc::c_int) |
                              ((1 as libc::c_int * 2 as libc::c_int) as u32_0
                                   &
                                   (((0x1 as libc::c_int) << 8 as libc::c_int)
                                        - 1 as libc::c_int) as libc::c_uint)
                                  << 0 as libc::c_int
                      })
                 });
        (*_g_0).words.w1 =
            if 0 as libc::c_int == 0 as libc::c_int {
                (((0 as libc::c_int * 2 as libc::c_int) as u32_0 &
                      (((0x1 as libc::c_int) << 8 as libc::c_int) -
                           1 as libc::c_int) as libc::c_uint) <<
                     16 as libc::c_int |
                     ((2 as libc::c_int * 2 as libc::c_int) as u32_0 &
                          (((0x1 as libc::c_int) << 8 as libc::c_int) -
                               1 as libc::c_int) as libc::c_uint) <<
                         8 as libc::c_int) |
                    ((3 as libc::c_int * 2 as libc::c_int) as u32_0 &
                         (((0x1 as libc::c_int) << 8 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        0 as libc::c_int
            } else if 0 as libc::c_int == 1 as libc::c_int {
                (((2 as libc::c_int * 2 as libc::c_int) as u32_0 &
                      (((0x1 as libc::c_int) << 8 as libc::c_int) -
                           1 as libc::c_int) as libc::c_uint) <<
                     16 as libc::c_int |
                     ((3 as libc::c_int * 2 as libc::c_int) as u32_0 &
                          (((0x1 as libc::c_int) << 8 as libc::c_int) -
                               1 as libc::c_int) as libc::c_uint) <<
                         8 as libc::c_int) |
                    ((0 as libc::c_int * 2 as libc::c_int) as u32_0 &
                         (((0x1 as libc::c_int) << 8 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        0 as libc::c_int
            } else {
                (((3 as libc::c_int * 2 as libc::c_int) as u32_0 &
                      (((0x1 as libc::c_int) << 8 as libc::c_int) -
                           1 as libc::c_int) as libc::c_uint) <<
                     16 as libc::c_int |
                     ((0 as libc::c_int * 2 as libc::c_int) as u32_0 &
                          (((0x1 as libc::c_int) << 8 as libc::c_int) -
                               1 as libc::c_int) as libc::c_uint) <<
                         8 as libc::c_int) |
                    ((2 as libc::c_int * 2 as libc::c_int) as u32_0 &
                         (((0x1 as libc::c_int) << 8 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        0 as libc::c_int
            }
    }
    Graph_CloseDisps(dispRefs.as_mut_ptr(), gfxCtx,
                     b"../z_eff_blure.c\x00" as *const u8 as
                         *const libc::c_char, 932 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn EffectBlure_DrawElemHermiteInterpolation(mut this:
                                                                      *mut EffectBlure,
                                                                  mut elem:
                                                                      *mut EffectBlureElement,
                                                                  mut index:
                                                                      s32,
                                                                  mut gfxCtx:
                                                                      *mut GraphicsContext) {
    static mut baseVtx: Vtx_t =
        {
            let mut init =
                Vtx_t{ob:
                          [0 as libc::c_int as libc::c_short,
                           0 as libc::c_int as libc::c_short,
                           0 as libc::c_int as libc::c_short],
                      flag: 0 as libc::c_int as libc::c_ushort,
                      tc:
                          [0 as libc::c_int as libc::c_short,
                           0 as libc::c_int as libc::c_short],
                      cn:
                          [0xff as libc::c_int as libc::c_uchar,
                           0xff as libc::c_int as libc::c_uchar,
                           0xff as libc::c_int as libc::c_uchar,
                           0xff as libc::c_int as libc::c_uchar],};
            init
        };
    let mut vtx: *mut Vtx = 0 as *mut Vtx;
    let mut sp1EC: Vec3s = Vec3s{x: 0, y: 0, z: 0,};
    let mut sp1E4: Vec3s = Vec3s{x: 0, y: 0, z: 0,};
    let mut ratio: f32_0 = 0.;
    let mut sp1DC: Color_RGBA8 = Color_RGBA8{r: 0, g: 0, b: 0, a: 0,};
    let mut sp1D8: Color_RGBA8 = Color_RGBA8{r: 0, g: 0, b: 0, a: 0,};
    let mut sp1CC: Vec3f = Vec3f{x: 0., y: 0., z: 0.,};
    let mut sp1C0: Vec3f = Vec3f{x: 0., y: 0., z: 0.,};
    let mut sp1B4: Vec3f = Vec3f{x: 0., y: 0., z: 0.,};
    let mut sp1A8: Vec3f = Vec3f{x: 0., y: 0., z: 0.,};
    let mut sp1A4: Color_RGBA8 = Color_RGBA8{r: 0, g: 0, b: 0, a: 0,};
    let mut sp1A0: Color_RGBA8 = Color_RGBA8{r: 0, g: 0, b: 0, a: 0,};
    let mut sp19C: Color_RGBA8 = Color_RGBA8{r: 0, g: 0, b: 0, a: 0,};
    let mut sp198: Color_RGBA8 = Color_RGBA8{r: 0, g: 0, b: 0, a: 0,};
    let mut sp18C: Vec3f = Vec3f{x: 0., y: 0., z: 0.,};
    let mut sp180: Vec3f = Vec3f{x: 0., y: 0., z: 0.,};
    let mut sp174: Vec3f = Vec3f{x: 0., y: 0., z: 0.,};
    let mut sp168: Vec3f = Vec3f{x: 0., y: 0., z: 0.,};
    let mut i: s32 = 0;
    let mut sp158: Vec3f = Vec3f{x: 0., y: 0., z: 0.,};
    let mut sp14C: Vec3f = Vec3f{x: 0., y: 0., z: 0.,};
    let mut sp148: Color_RGBA8 = Color_RGBA8{r: 0, g: 0, b: 0, a: 0,};
    let mut sp144: Color_RGBA8 = Color_RGBA8{r: 0, g: 0, b: 0, a: 0,};
    let mut sp138: Vec3f = Vec3f{x: 0., y: 0., z: 0.,};
    let mut __gfxCtx: *mut GraphicsContext = 0 as *mut GraphicsContext;
    let mut dispRefs: [*mut Gfx; 4] = [0 as *mut Gfx; 4];
    __gfxCtx = gfxCtx;
    Graph_OpenDisps(dispRefs.as_mut_ptr(), gfxCtx,
                    b"../z_eff_blure.c\x00" as *const u8 as
                        *const libc::c_char, 971 as libc::c_int);
    Math_Vec3s_ToVec3f(&mut sp138,
                       &mut (*(*this).elements.as_mut_ptr().offset(0 as
                                                                       libc::c_int
                                                                       as
                                                                       isize)).p2);
    ratio = (*elem).timer as f32_0 / (*this).elemDuration as f32_0;
    EffectBlure_GetComputedValues(this, index, ratio, &mut sp1EC, &mut sp1E4,
                                  &mut sp1A4, &mut sp1A0);
    Math_Vec3s_ToVec3f(&mut sp1CC, &mut sp1EC);
    Math_Vec3s_ToVec3f(&mut sp1C0, &mut sp1E4);
    ratio =
        (*elem.offset(1 as libc::c_int as isize)).timer as f32_0 /
            (*this).elemDuration as f32_0;
    EffectBlure_GetComputedValues(this, index + 1 as libc::c_int, ratio,
                                  &mut sp1EC, &mut sp1E4, &mut sp19C,
                                  &mut sp198);
    Math_Vec3s_ToVec3f(&mut sp18C, &mut sp1EC);
    Math_Vec3s_ToVec3f(&mut sp180, &mut sp1E4);
    if (*elem).flags as libc::c_int & 3 as libc::c_int == 2 as libc::c_int {
        Math_Vec3f_Diff(&mut sp18C, &mut sp1CC, &mut sp1B4);
        Math_Vec3f_Diff(&mut sp180, &mut sp1C0, &mut sp1A8);
    } else {
        let mut sp118: Vec3f = Vec3f{x: 0., y: 0., z: 0.,};
        let mut sp10C: Vec3f = Vec3f{x: 0., y: 0., z: 0.,};
        if index - 1 as libc::c_int >= 0 as libc::c_int {
        } else {
            __assert(b"index - 1 >= 0\x00" as *const u8 as
                         *const libc::c_char,
                     b"../z_eff_blure.c\x00" as *const u8 as
                         *const libc::c_char, 1005 as libc::c_int);
        };
        ratio =
            (*elem.offset(-(1 as libc::c_int as isize))).timer as f32_0 /
                (*this).elemDuration as f32_0;
        EffectBlure_GetComputedValues(this, index - 1 as libc::c_int, ratio,
                                      &mut sp1EC, &mut sp1E4, &mut sp1DC,
                                      &mut sp1D8);
        Math_Vec3s_ToVec3f(&mut sp118, &mut sp1EC);
        Math_Vec3s_ToVec3f(&mut sp10C, &mut sp1E4);
        Math_Vec3f_Diff(&mut sp18C, &mut sp118, &mut sp1B4);
        Math_Vec3f_Diff(&mut sp180, &mut sp10C, &mut sp1A8);
    }
    Math_Vec3f_Scale(&mut sp1B4, 0.5f32);
    Math_Vec3f_Scale(&mut sp1A8, 0.5f32);
    if (*elem.offset(1 as libc::c_int as isize)).flags as libc::c_int &
           3 as libc::c_int == 2 as libc::c_int {
        Math_Vec3f_Diff(&mut sp18C, &mut sp1CC, &mut sp174);
        Math_Vec3f_Diff(&mut sp180, &mut sp1C0, &mut sp168);
    } else {
        let mut sp100: Vec3f = Vec3f{x: 0., y: 0., z: 0.,};
        let mut spF4: Vec3f = Vec3f{x: 0., y: 0., z: 0.,};
        if (index + 2 as libc::c_int) < (*this).numElements as libc::c_int {
        } else {
            __assert(b"index + 2 < this2->now_edge_num\x00" as *const u8 as
                         *const libc::c_char,
                     b"../z_eff_blure.c\x00" as *const u8 as
                         *const libc::c_char, 1032 as libc::c_int);
        };
        ratio =
            (*elem.offset(2 as libc::c_int as isize)).timer as f32_0 /
                (*this).elemDuration as f32_0;
        EffectBlure_GetComputedValues(this, index + 2 as libc::c_int, ratio,
                                      &mut sp1EC, &mut sp1E4, &mut sp1DC,
                                      &mut sp1D8);
        Math_Vec3s_ToVec3f(&mut sp100, &mut sp1EC);
        Math_Vec3s_ToVec3f(&mut spF4, &mut sp1E4);
        Math_Vec3f_Diff(&mut sp100, &mut sp1CC, &mut sp174);
        Math_Vec3f_Diff(&mut spF4, &mut sp1C0, &mut sp168);
    }
    Math_Vec3f_Scale(&mut sp174, 0.5f32);
    Math_Vec3f_Scale(&mut sp168, 0.5f32);
    vtx =
        Graph_Alloc(gfxCtx,
                    ::std::mem::size_of::<[Vtx; 16]>() as libc::c_ulong as
                        size_t) as *mut Vtx;
    if vtx.is_null() {
        // "Vertices cannot be secured."
        osSyncPrintf(b"z_eff_blure.c::SQ_HermiteInterpolate_disp() \xe9\xa0\x82\xe7\x82\xb9\xe7\xa2\xba\xe4\xbf\x9d\xe3\x81\xa7\xe3\x81\x8d\xe3\x81\x9a\xe3\x80\x82\n\x00"
                         as *const u8 as *const libc::c_char); // t
    } else {
        Math_Vec3f_Diff(&mut sp1CC, &mut sp138, &mut sp158); // t^2
        Math_Vec3f_Scale(&mut sp158, 10.0f32); // t^3
        Math_Vec3f_Diff(&mut sp1C0, &mut sp138, &mut sp14C); // t^3 - t^2
        Math_Vec3f_Scale(&mut sp14C, 10.0f32); // t^3 - 2t^2 + t
        Color_RGBA8_Copy(&mut sp148, &mut sp1A4); // 2t^3 - 3t^2 + 1
        Color_RGBA8_Copy(&mut sp144, &mut sp1A0); // 3t^2 - 2t^3
        (*vtx.offset(0 as libc::c_int as isize)).v = baseVtx;
        (*vtx.offset(1 as libc::c_int as isize)).v = baseVtx;
        (*vtx.offset(0 as libc::c_int as
                         isize)).v.ob[0 as libc::c_int as usize] =
            Math_FNearbyIntF(sp158.x) as libc::c_short;
        (*vtx.offset(0 as libc::c_int as
                         isize)).v.ob[1 as libc::c_int as usize] =
            Math_FNearbyIntF(sp158.y) as libc::c_short;
        (*vtx.offset(0 as libc::c_int as
                         isize)).v.ob[2 as libc::c_int as usize] =
            Math_FNearbyIntF(sp158.z) as libc::c_short;
        (*vtx.offset(0 as libc::c_int as
                         isize)).v.cn[0 as libc::c_int as usize] = sp148.r;
        (*vtx.offset(0 as libc::c_int as
                         isize)).v.cn[1 as libc::c_int as usize] = sp148.g;
        (*vtx.offset(0 as libc::c_int as
                         isize)).v.cn[2 as libc::c_int as usize] = sp148.b;
        (*vtx.offset(0 as libc::c_int as
                         isize)).v.cn[3 as libc::c_int as usize] = sp148.a;
        (*vtx.offset(1 as libc::c_int as
                         isize)).v.ob[0 as libc::c_int as usize] =
            Math_FNearbyIntF(sp14C.x) as libc::c_short;
        (*vtx.offset(1 as libc::c_int as
                         isize)).v.ob[1 as libc::c_int as usize] =
            Math_FNearbyIntF(sp14C.y) as libc::c_short;
        (*vtx.offset(1 as libc::c_int as
                         isize)).v.ob[2 as libc::c_int as usize] =
            Math_FNearbyIntF(sp14C.z) as libc::c_short;
        (*vtx.offset(1 as libc::c_int as
                         isize)).v.cn[0 as libc::c_int as usize] = sp144.r;
        (*vtx.offset(1 as libc::c_int as
                         isize)).v.cn[1 as libc::c_int as usize] = sp144.g;
        (*vtx.offset(1 as libc::c_int as
                         isize)).v.cn[2 as libc::c_int as usize] = sp144.b;
        (*vtx.offset(1 as libc::c_int as
                         isize)).v.cn[3 as libc::c_int as usize] = sp144.a;
        i = 1 as libc::c_int;
        while i < 8 as libc::c_int {
            let mut j1: s32 = 2 as libc::c_int * i;
            let mut j2: s32 = 2 as libc::c_int * i + 1 as libc::c_int;
            let mut spE0: Vec3f = Vec3f{x: 0., y: 0., z: 0.,};
            let mut temp_f28: f32_0 = i as libc::c_float / 7.0f32;
            let mut temp_f0: f32_0 = temp_f28 * temp_f28;
            let mut temp_f2: f32_0 = temp_f0 * temp_f28;
            let mut temp_f20: f32_0 = temp_f2 - temp_f0;
            let mut temp_f22: f32_0 = temp_f2 - 2.0f32 * temp_f0 + temp_f28;
            let mut temp_f24: f32_0 =
                2.0f32 * temp_f2 - temp_f0 * 3.0f32 + 1.0f32;
            let mut temp_f26: f32_0 = temp_f0 * 3.0f32 - 2.0f32 * temp_f2;
            let mut pad1: s32 = 0;
            let mut pad2: s32 = 0;
            // p = (2t^3 - 3t^2 + 1)p0 + (3t^2 - 2t^3)p1 + (t^3 - 2t^2 + t)m0 + (t^3 - t^2)m1
            spE0.x =
                temp_f24 * sp1CC.x + temp_f26 * sp18C.x + temp_f22 * sp1B4.x +
                    temp_f20 * sp174.x;
            spE0.y =
                temp_f24 * sp1CC.y + temp_f26 * sp18C.y + temp_f22 * sp1B4.y +
                    temp_f20 * sp174.y;
            spE0.z =
                temp_f24 * sp1CC.z + temp_f26 * sp18C.z + temp_f22 * sp1B4.z +
                    temp_f20 * sp174.z;
            Math_Vec3f_Diff(&mut spE0, &mut sp138, &mut sp158);
            Math_Vec3f_Scale(&mut sp158, 10.0f32);
            spE0.x =
                temp_f24 * sp1C0.x + temp_f26 * sp180.x + temp_f22 * sp1A8.x +
                    temp_f20 * sp168.x;
            spE0.y =
                temp_f24 * sp1C0.y + temp_f26 * sp180.y + temp_f22 * sp1A8.y +
                    temp_f20 * sp168.y;
            spE0.z =
                temp_f24 * sp1C0.z + temp_f26 * sp180.z + temp_f22 * sp1A8.z +
                    temp_f20 * sp168.z;
            Math_Vec3f_Diff(&mut spE0, &mut sp138, &mut sp14C);
            Math_Vec3f_Scale(&mut sp14C, 10.0f32);
            (*vtx.offset(j1 as isize)).v = baseVtx;
            (*vtx.offset(j2 as isize)).v = baseVtx;
            (*vtx.offset(j1 as isize)).v.ob[0 as libc::c_int as usize] =
                Math_FNearbyIntF(sp158.x) as libc::c_short;
            (*vtx.offset(j1 as isize)).v.ob[1 as libc::c_int as usize] =
                Math_FNearbyIntF(sp158.y) as libc::c_short;
            (*vtx.offset(j1 as isize)).v.ob[2 as libc::c_int as usize] =
                Math_FNearbyIntF(sp158.z) as libc::c_short;
            (*vtx.offset(j1 as isize)).v.cn[0 as libc::c_int as usize] =
                func_80027E84(sp1A4.r, sp19C.r, temp_f28);
            (*vtx.offset(j1 as isize)).v.cn[1 as libc::c_int as usize] =
                func_80027E84(sp1A4.g, sp19C.g, temp_f28);
            (*vtx.offset(j1 as isize)).v.cn[2 as libc::c_int as usize] =
                func_80027E84(sp1A4.b, sp19C.b, temp_f28);
            (*vtx.offset(j1 as isize)).v.cn[3 as libc::c_int as usize] =
                func_80027E84(sp1A4.a, sp19C.a, temp_f28);
            (*vtx.offset(j2 as isize)).v.ob[0 as libc::c_int as usize] =
                Math_FNearbyIntF(sp14C.x) as libc::c_short;
            (*vtx.offset(j2 as isize)).v.ob[1 as libc::c_int as usize] =
                Math_FNearbyIntF(sp14C.y) as libc::c_short;
            (*vtx.offset(j2 as isize)).v.ob[2 as libc::c_int as usize] =
                Math_FNearbyIntF(sp14C.z) as libc::c_short;
            (*vtx.offset(j2 as isize)).v.cn[0 as libc::c_int as usize] =
                func_80027E84(sp1A0.r, sp198.r, temp_f28);
            (*vtx.offset(j2 as isize)).v.cn[1 as libc::c_int as usize] =
                func_80027E84(sp1A0.g, sp198.g, temp_f28);
            (*vtx.offset(j2 as isize)).v.cn[2 as libc::c_int as usize] =
                func_80027E84(sp1A0.b, sp198.b, temp_f28);
            (*vtx.offset(j2 as isize)).v.cn[3 as libc::c_int as usize] =
                func_80027E84(sp1A0.a, sp198.a, temp_f28);
            i += 1
        }
        let fresh2 = (*__gfxCtx).polyXlu.p;
        (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
        let mut _g: *mut Gfx = fresh2;
        (*_g).words.w0 =
            (0x1 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 24 as libc::c_int
                |
                (16 as libc::c_int as u32_0 &
                     (((0x1 as libc::c_int) << 8 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    12 as libc::c_int |
                ((0 as libc::c_int + 16 as libc::c_int) as u32_0 &
                     (((0x1 as libc::c_int) << 7 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    1 as libc::c_int;
        (*_g).words.w1 = vtx as libc::c_uint;
        let fresh3 = (*__gfxCtx).polyXlu.p;
        (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
        let mut _g_0: *mut Gfx = fresh3;
        (*_g_0).words.w0 =
            (0x6 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 24 as libc::c_int
                |
                (if 0 as libc::c_int == 0 as libc::c_int {
                     (((0 as libc::c_int * 2 as libc::c_int) as u32_0 &
                           (((0x1 as libc::c_int) << 8 as libc::c_int) -
                                1 as libc::c_int) as libc::c_uint) <<
                          16 as libc::c_int |
                          ((1 as libc::c_int * 2 as libc::c_int) as u32_0 &
                               (((0x1 as libc::c_int) << 8 as libc::c_int) -
                                    1 as libc::c_int) as libc::c_uint) <<
                              8 as libc::c_int) |
                         ((3 as libc::c_int * 2 as libc::c_int) as u32_0 &
                              (((0x1 as libc::c_int) << 8 as libc::c_int) -
                                   1 as libc::c_int) as libc::c_uint) <<
                             0 as libc::c_int
                 } else {
                     (if 0 as libc::c_int == 1 as libc::c_int {
                          (((1 as libc::c_int * 2 as libc::c_int) as u32_0 &
                                (((0x1 as libc::c_int) << 8 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               16 as libc::c_int |
                               ((3 as libc::c_int * 2 as libc::c_int) as u32_0
                                    &
                                    (((0x1 as libc::c_int) <<
                                          8 as libc::c_int) -
                                         1 as libc::c_int) as libc::c_uint) <<
                                   8 as libc::c_int) |
                              ((0 as libc::c_int * 2 as libc::c_int) as u32_0
                                   &
                                   (((0x1 as libc::c_int) << 8 as libc::c_int)
                                        - 1 as libc::c_int) as libc::c_uint)
                                  << 0 as libc::c_int
                      } else {
                          (((3 as libc::c_int * 2 as libc::c_int) as u32_0 &
                                (((0x1 as libc::c_int) << 8 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               16 as libc::c_int |
                               ((0 as libc::c_int * 2 as libc::c_int) as u32_0
                                    &
                                    (((0x1 as libc::c_int) <<
                                          8 as libc::c_int) -
                                         1 as libc::c_int) as libc::c_uint) <<
                                   8 as libc::c_int) |
                              ((1 as libc::c_int * 2 as libc::c_int) as u32_0
                                   &
                                   (((0x1 as libc::c_int) << 8 as libc::c_int)
                                        - 1 as libc::c_int) as libc::c_uint)
                                  << 0 as libc::c_int
                      })
                 });
        (*_g_0).words.w1 =
            if 0 as libc::c_int == 0 as libc::c_int {
                (((0 as libc::c_int * 2 as libc::c_int) as u32_0 &
                      (((0x1 as libc::c_int) << 8 as libc::c_int) -
                           1 as libc::c_int) as libc::c_uint) <<
                     16 as libc::c_int |
                     ((3 as libc::c_int * 2 as libc::c_int) as u32_0 &
                          (((0x1 as libc::c_int) << 8 as libc::c_int) -
                               1 as libc::c_int) as libc::c_uint) <<
                         8 as libc::c_int) |
                    ((2 as libc::c_int * 2 as libc::c_int) as u32_0 &
                         (((0x1 as libc::c_int) << 8 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        0 as libc::c_int
            } else if 0 as libc::c_int == 1 as libc::c_int {
                (((3 as libc::c_int * 2 as libc::c_int) as u32_0 &
                      (((0x1 as libc::c_int) << 8 as libc::c_int) -
                           1 as libc::c_int) as libc::c_uint) <<
                     16 as libc::c_int |
                     ((2 as libc::c_int * 2 as libc::c_int) as u32_0 &
                          (((0x1 as libc::c_int) << 8 as libc::c_int) -
                               1 as libc::c_int) as libc::c_uint) <<
                         8 as libc::c_int) |
                    ((0 as libc::c_int * 2 as libc::c_int) as u32_0 &
                         (((0x1 as libc::c_int) << 8 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        0 as libc::c_int
            } else {
                (((2 as libc::c_int * 2 as libc::c_int) as u32_0 &
                      (((0x1 as libc::c_int) << 8 as libc::c_int) -
                           1 as libc::c_int) as libc::c_uint) <<
                     16 as libc::c_int |
                     ((0 as libc::c_int * 2 as libc::c_int) as u32_0 &
                          (((0x1 as libc::c_int) << 8 as libc::c_int) -
                               1 as libc::c_int) as libc::c_uint) <<
                         8 as libc::c_int) |
                    ((3 as libc::c_int * 2 as libc::c_int) as u32_0 &
                         (((0x1 as libc::c_int) << 8 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        0 as libc::c_int
            };
        let fresh4 = (*__gfxCtx).polyXlu.p;
        (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
        let mut _g_1: *mut Gfx = fresh4;
        (*_g_1).words.w0 =
            (0x6 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 24 as libc::c_int
                |
                (if 0 as libc::c_int == 0 as libc::c_int {
                     (((2 as libc::c_int * 2 as libc::c_int) as u32_0 &
                           (((0x1 as libc::c_int) << 8 as libc::c_int) -
                                1 as libc::c_int) as libc::c_uint) <<
                          16 as libc::c_int |
                          ((3 as libc::c_int * 2 as libc::c_int) as u32_0 &
                               (((0x1 as libc::c_int) << 8 as libc::c_int) -
                                    1 as libc::c_int) as libc::c_uint) <<
                              8 as libc::c_int) |
                         ((5 as libc::c_int * 2 as libc::c_int) as u32_0 &
                              (((0x1 as libc::c_int) << 8 as libc::c_int) -
                                   1 as libc::c_int) as libc::c_uint) <<
                             0 as libc::c_int
                 } else {
                     (if 0 as libc::c_int == 1 as libc::c_int {
                          (((3 as libc::c_int * 2 as libc::c_int) as u32_0 &
                                (((0x1 as libc::c_int) << 8 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               16 as libc::c_int |
                               ((5 as libc::c_int * 2 as libc::c_int) as u32_0
                                    &
                                    (((0x1 as libc::c_int) <<
                                          8 as libc::c_int) -
                                         1 as libc::c_int) as libc::c_uint) <<
                                   8 as libc::c_int) |
                              ((2 as libc::c_int * 2 as libc::c_int) as u32_0
                                   &
                                   (((0x1 as libc::c_int) << 8 as libc::c_int)
                                        - 1 as libc::c_int) as libc::c_uint)
                                  << 0 as libc::c_int
                      } else {
                          (((5 as libc::c_int * 2 as libc::c_int) as u32_0 &
                                (((0x1 as libc::c_int) << 8 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               16 as libc::c_int |
                               ((2 as libc::c_int * 2 as libc::c_int) as u32_0
                                    &
                                    (((0x1 as libc::c_int) <<
                                          8 as libc::c_int) -
                                         1 as libc::c_int) as libc::c_uint) <<
                                   8 as libc::c_int) |
                              ((3 as libc::c_int * 2 as libc::c_int) as u32_0
                                   &
                                   (((0x1 as libc::c_int) << 8 as libc::c_int)
                                        - 1 as libc::c_int) as libc::c_uint)
                                  << 0 as libc::c_int
                      })
                 });
        (*_g_1).words.w1 =
            if 0 as libc::c_int == 0 as libc::c_int {
                (((2 as libc::c_int * 2 as libc::c_int) as u32_0 &
                      (((0x1 as libc::c_int) << 8 as libc::c_int) -
                           1 as libc::c_int) as libc::c_uint) <<
                     16 as libc::c_int |
                     ((5 as libc::c_int * 2 as libc::c_int) as u32_0 &
                          (((0x1 as libc::c_int) << 8 as libc::c_int) -
                               1 as libc::c_int) as libc::c_uint) <<
                         8 as libc::c_int) |
                    ((4 as libc::c_int * 2 as libc::c_int) as u32_0 &
                         (((0x1 as libc::c_int) << 8 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        0 as libc::c_int
            } else if 0 as libc::c_int == 1 as libc::c_int {
                (((5 as libc::c_int * 2 as libc::c_int) as u32_0 &
                      (((0x1 as libc::c_int) << 8 as libc::c_int) -
                           1 as libc::c_int) as libc::c_uint) <<
                     16 as libc::c_int |
                     ((4 as libc::c_int * 2 as libc::c_int) as u32_0 &
                          (((0x1 as libc::c_int) << 8 as libc::c_int) -
                               1 as libc::c_int) as libc::c_uint) <<
                         8 as libc::c_int) |
                    ((2 as libc::c_int * 2 as libc::c_int) as u32_0 &
                         (((0x1 as libc::c_int) << 8 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        0 as libc::c_int
            } else {
                (((4 as libc::c_int * 2 as libc::c_int) as u32_0 &
                      (((0x1 as libc::c_int) << 8 as libc::c_int) -
                           1 as libc::c_int) as libc::c_uint) <<
                     16 as libc::c_int |
                     ((2 as libc::c_int * 2 as libc::c_int) as u32_0 &
                          (((0x1 as libc::c_int) << 8 as libc::c_int) -
                               1 as libc::c_int) as libc::c_uint) <<
                         8 as libc::c_int) |
                    ((5 as libc::c_int * 2 as libc::c_int) as u32_0 &
                         (((0x1 as libc::c_int) << 8 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        0 as libc::c_int
            };
        let fresh5 = (*__gfxCtx).polyXlu.p;
        (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
        let mut _g_2: *mut Gfx = fresh5;
        (*_g_2).words.w0 =
            (0x6 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 24 as libc::c_int
                |
                (if 0 as libc::c_int == 0 as libc::c_int {
                     (((4 as libc::c_int * 2 as libc::c_int) as u32_0 &
                           (((0x1 as libc::c_int) << 8 as libc::c_int) -
                                1 as libc::c_int) as libc::c_uint) <<
                          16 as libc::c_int |
                          ((5 as libc::c_int * 2 as libc::c_int) as u32_0 &
                               (((0x1 as libc::c_int) << 8 as libc::c_int) -
                                    1 as libc::c_int) as libc::c_uint) <<
                              8 as libc::c_int) |
                         ((7 as libc::c_int * 2 as libc::c_int) as u32_0 &
                              (((0x1 as libc::c_int) << 8 as libc::c_int) -
                                   1 as libc::c_int) as libc::c_uint) <<
                             0 as libc::c_int
                 } else {
                     (if 0 as libc::c_int == 1 as libc::c_int {
                          (((5 as libc::c_int * 2 as libc::c_int) as u32_0 &
                                (((0x1 as libc::c_int) << 8 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               16 as libc::c_int |
                               ((7 as libc::c_int * 2 as libc::c_int) as u32_0
                                    &
                                    (((0x1 as libc::c_int) <<
                                          8 as libc::c_int) -
                                         1 as libc::c_int) as libc::c_uint) <<
                                   8 as libc::c_int) |
                              ((4 as libc::c_int * 2 as libc::c_int) as u32_0
                                   &
                                   (((0x1 as libc::c_int) << 8 as libc::c_int)
                                        - 1 as libc::c_int) as libc::c_uint)
                                  << 0 as libc::c_int
                      } else {
                          (((7 as libc::c_int * 2 as libc::c_int) as u32_0 &
                                (((0x1 as libc::c_int) << 8 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               16 as libc::c_int |
                               ((4 as libc::c_int * 2 as libc::c_int) as u32_0
                                    &
                                    (((0x1 as libc::c_int) <<
                                          8 as libc::c_int) -
                                         1 as libc::c_int) as libc::c_uint) <<
                                   8 as libc::c_int) |
                              ((5 as libc::c_int * 2 as libc::c_int) as u32_0
                                   &
                                   (((0x1 as libc::c_int) << 8 as libc::c_int)
                                        - 1 as libc::c_int) as libc::c_uint)
                                  << 0 as libc::c_int
                      })
                 });
        (*_g_2).words.w1 =
            if 0 as libc::c_int == 0 as libc::c_int {
                (((4 as libc::c_int * 2 as libc::c_int) as u32_0 &
                      (((0x1 as libc::c_int) << 8 as libc::c_int) -
                           1 as libc::c_int) as libc::c_uint) <<
                     16 as libc::c_int |
                     ((7 as libc::c_int * 2 as libc::c_int) as u32_0 &
                          (((0x1 as libc::c_int) << 8 as libc::c_int) -
                               1 as libc::c_int) as libc::c_uint) <<
                         8 as libc::c_int) |
                    ((6 as libc::c_int * 2 as libc::c_int) as u32_0 &
                         (((0x1 as libc::c_int) << 8 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        0 as libc::c_int
            } else if 0 as libc::c_int == 1 as libc::c_int {
                (((7 as libc::c_int * 2 as libc::c_int) as u32_0 &
                      (((0x1 as libc::c_int) << 8 as libc::c_int) -
                           1 as libc::c_int) as libc::c_uint) <<
                     16 as libc::c_int |
                     ((6 as libc::c_int * 2 as libc::c_int) as u32_0 &
                          (((0x1 as libc::c_int) << 8 as libc::c_int) -
                               1 as libc::c_int) as libc::c_uint) <<
                         8 as libc::c_int) |
                    ((4 as libc::c_int * 2 as libc::c_int) as u32_0 &
                         (((0x1 as libc::c_int) << 8 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        0 as libc::c_int
            } else {
                (((6 as libc::c_int * 2 as libc::c_int) as u32_0 &
                      (((0x1 as libc::c_int) << 8 as libc::c_int) -
                           1 as libc::c_int) as libc::c_uint) <<
                     16 as libc::c_int |
                     ((4 as libc::c_int * 2 as libc::c_int) as u32_0 &
                          (((0x1 as libc::c_int) << 8 as libc::c_int) -
                               1 as libc::c_int) as libc::c_uint) <<
                         8 as libc::c_int) |
                    ((7 as libc::c_int * 2 as libc::c_int) as u32_0 &
                         (((0x1 as libc::c_int) << 8 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        0 as libc::c_int
            };
        let fresh6 = (*__gfxCtx).polyXlu.p;
        (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
        let mut _g_3: *mut Gfx = fresh6;
        (*_g_3).words.w0 =
            (0x6 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 24 as libc::c_int
                |
                (if 0 as libc::c_int == 0 as libc::c_int {
                     (((6 as libc::c_int * 2 as libc::c_int) as u32_0 &
                           (((0x1 as libc::c_int) << 8 as libc::c_int) -
                                1 as libc::c_int) as libc::c_uint) <<
                          16 as libc::c_int |
                          ((7 as libc::c_int * 2 as libc::c_int) as u32_0 &
                               (((0x1 as libc::c_int) << 8 as libc::c_int) -
                                    1 as libc::c_int) as libc::c_uint) <<
                              8 as libc::c_int) |
                         ((9 as libc::c_int * 2 as libc::c_int) as u32_0 &
                              (((0x1 as libc::c_int) << 8 as libc::c_int) -
                                   1 as libc::c_int) as libc::c_uint) <<
                             0 as libc::c_int
                 } else {
                     (if 0 as libc::c_int == 1 as libc::c_int {
                          (((7 as libc::c_int * 2 as libc::c_int) as u32_0 &
                                (((0x1 as libc::c_int) << 8 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               16 as libc::c_int |
                               ((9 as libc::c_int * 2 as libc::c_int) as u32_0
                                    &
                                    (((0x1 as libc::c_int) <<
                                          8 as libc::c_int) -
                                         1 as libc::c_int) as libc::c_uint) <<
                                   8 as libc::c_int) |
                              ((6 as libc::c_int * 2 as libc::c_int) as u32_0
                                   &
                                   (((0x1 as libc::c_int) << 8 as libc::c_int)
                                        - 1 as libc::c_int) as libc::c_uint)
                                  << 0 as libc::c_int
                      } else {
                          (((9 as libc::c_int * 2 as libc::c_int) as u32_0 &
                                (((0x1 as libc::c_int) << 8 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               16 as libc::c_int |
                               ((6 as libc::c_int * 2 as libc::c_int) as u32_0
                                    &
                                    (((0x1 as libc::c_int) <<
                                          8 as libc::c_int) -
                                         1 as libc::c_int) as libc::c_uint) <<
                                   8 as libc::c_int) |
                              ((7 as libc::c_int * 2 as libc::c_int) as u32_0
                                   &
                                   (((0x1 as libc::c_int) << 8 as libc::c_int)
                                        - 1 as libc::c_int) as libc::c_uint)
                                  << 0 as libc::c_int
                      })
                 });
        (*_g_3).words.w1 =
            if 0 as libc::c_int == 0 as libc::c_int {
                (((6 as libc::c_int * 2 as libc::c_int) as u32_0 &
                      (((0x1 as libc::c_int) << 8 as libc::c_int) -
                           1 as libc::c_int) as libc::c_uint) <<
                     16 as libc::c_int |
                     ((9 as libc::c_int * 2 as libc::c_int) as u32_0 &
                          (((0x1 as libc::c_int) << 8 as libc::c_int) -
                               1 as libc::c_int) as libc::c_uint) <<
                         8 as libc::c_int) |
                    ((8 as libc::c_int * 2 as libc::c_int) as u32_0 &
                         (((0x1 as libc::c_int) << 8 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        0 as libc::c_int
            } else if 0 as libc::c_int == 1 as libc::c_int {
                (((9 as libc::c_int * 2 as libc::c_int) as u32_0 &
                      (((0x1 as libc::c_int) << 8 as libc::c_int) -
                           1 as libc::c_int) as libc::c_uint) <<
                     16 as libc::c_int |
                     ((8 as libc::c_int * 2 as libc::c_int) as u32_0 &
                          (((0x1 as libc::c_int) << 8 as libc::c_int) -
                               1 as libc::c_int) as libc::c_uint) <<
                         8 as libc::c_int) |
                    ((6 as libc::c_int * 2 as libc::c_int) as u32_0 &
                         (((0x1 as libc::c_int) << 8 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        0 as libc::c_int
            } else {
                (((8 as libc::c_int * 2 as libc::c_int) as u32_0 &
                      (((0x1 as libc::c_int) << 8 as libc::c_int) -
                           1 as libc::c_int) as libc::c_uint) <<
                     16 as libc::c_int |
                     ((6 as libc::c_int * 2 as libc::c_int) as u32_0 &
                          (((0x1 as libc::c_int) << 8 as libc::c_int) -
                               1 as libc::c_int) as libc::c_uint) <<
                         8 as libc::c_int) |
                    ((9 as libc::c_int * 2 as libc::c_int) as u32_0 &
                         (((0x1 as libc::c_int) << 8 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        0 as libc::c_int
            };
        let fresh7 = (*__gfxCtx).polyXlu.p;
        (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
        let mut _g_4: *mut Gfx = fresh7;
        (*_g_4).words.w0 =
            (0x6 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 24 as libc::c_int
                |
                (if 0 as libc::c_int == 0 as libc::c_int {
                     (((8 as libc::c_int * 2 as libc::c_int) as u32_0 &
                           (((0x1 as libc::c_int) << 8 as libc::c_int) -
                                1 as libc::c_int) as libc::c_uint) <<
                          16 as libc::c_int |
                          ((9 as libc::c_int * 2 as libc::c_int) as u32_0 &
                               (((0x1 as libc::c_int) << 8 as libc::c_int) -
                                    1 as libc::c_int) as libc::c_uint) <<
                              8 as libc::c_int) |
                         ((11 as libc::c_int * 2 as libc::c_int) as u32_0 &
                              (((0x1 as libc::c_int) << 8 as libc::c_int) -
                                   1 as libc::c_int) as libc::c_uint) <<
                             0 as libc::c_int
                 } else {
                     (if 0 as libc::c_int == 1 as libc::c_int {
                          (((9 as libc::c_int * 2 as libc::c_int) as u32_0 &
                                (((0x1 as libc::c_int) << 8 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               16 as libc::c_int |
                               ((11 as libc::c_int * 2 as libc::c_int) as
                                    u32_0 &
                                    (((0x1 as libc::c_int) <<
                                          8 as libc::c_int) -
                                         1 as libc::c_int) as libc::c_uint) <<
                                   8 as libc::c_int) |
                              ((8 as libc::c_int * 2 as libc::c_int) as u32_0
                                   &
                                   (((0x1 as libc::c_int) << 8 as libc::c_int)
                                        - 1 as libc::c_int) as libc::c_uint)
                                  << 0 as libc::c_int
                      } else {
                          (((11 as libc::c_int * 2 as libc::c_int) as u32_0 &
                                (((0x1 as libc::c_int) << 8 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               16 as libc::c_int |
                               ((8 as libc::c_int * 2 as libc::c_int) as u32_0
                                    &
                                    (((0x1 as libc::c_int) <<
                                          8 as libc::c_int) -
                                         1 as libc::c_int) as libc::c_uint) <<
                                   8 as libc::c_int) |
                              ((9 as libc::c_int * 2 as libc::c_int) as u32_0
                                   &
                                   (((0x1 as libc::c_int) << 8 as libc::c_int)
                                        - 1 as libc::c_int) as libc::c_uint)
                                  << 0 as libc::c_int
                      })
                 });
        (*_g_4).words.w1 =
            if 0 as libc::c_int == 0 as libc::c_int {
                (((8 as libc::c_int * 2 as libc::c_int) as u32_0 &
                      (((0x1 as libc::c_int) << 8 as libc::c_int) -
                           1 as libc::c_int) as libc::c_uint) <<
                     16 as libc::c_int |
                     ((11 as libc::c_int * 2 as libc::c_int) as u32_0 &
                          (((0x1 as libc::c_int) << 8 as libc::c_int) -
                               1 as libc::c_int) as libc::c_uint) <<
                         8 as libc::c_int) |
                    ((10 as libc::c_int * 2 as libc::c_int) as u32_0 &
                         (((0x1 as libc::c_int) << 8 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        0 as libc::c_int
            } else if 0 as libc::c_int == 1 as libc::c_int {
                (((11 as libc::c_int * 2 as libc::c_int) as u32_0 &
                      (((0x1 as libc::c_int) << 8 as libc::c_int) -
                           1 as libc::c_int) as libc::c_uint) <<
                     16 as libc::c_int |
                     ((10 as libc::c_int * 2 as libc::c_int) as u32_0 &
                          (((0x1 as libc::c_int) << 8 as libc::c_int) -
                               1 as libc::c_int) as libc::c_uint) <<
                         8 as libc::c_int) |
                    ((8 as libc::c_int * 2 as libc::c_int) as u32_0 &
                         (((0x1 as libc::c_int) << 8 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        0 as libc::c_int
            } else {
                (((10 as libc::c_int * 2 as libc::c_int) as u32_0 &
                      (((0x1 as libc::c_int) << 8 as libc::c_int) -
                           1 as libc::c_int) as libc::c_uint) <<
                     16 as libc::c_int |
                     ((8 as libc::c_int * 2 as libc::c_int) as u32_0 &
                          (((0x1 as libc::c_int) << 8 as libc::c_int) -
                               1 as libc::c_int) as libc::c_uint) <<
                         8 as libc::c_int) |
                    ((11 as libc::c_int * 2 as libc::c_int) as u32_0 &
                         (((0x1 as libc::c_int) << 8 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        0 as libc::c_int
            };
        let fresh8 = (*__gfxCtx).polyXlu.p;
        (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
        let mut _g_5: *mut Gfx = fresh8;
        (*_g_5).words.w0 =
            (0x6 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 24 as libc::c_int
                |
                (if 0 as libc::c_int == 0 as libc::c_int {
                     (((10 as libc::c_int * 2 as libc::c_int) as u32_0 &
                           (((0x1 as libc::c_int) << 8 as libc::c_int) -
                                1 as libc::c_int) as libc::c_uint) <<
                          16 as libc::c_int |
                          ((11 as libc::c_int * 2 as libc::c_int) as u32_0 &
                               (((0x1 as libc::c_int) << 8 as libc::c_int) -
                                    1 as libc::c_int) as libc::c_uint) <<
                              8 as libc::c_int) |
                         ((13 as libc::c_int * 2 as libc::c_int) as u32_0 &
                              (((0x1 as libc::c_int) << 8 as libc::c_int) -
                                   1 as libc::c_int) as libc::c_uint) <<
                             0 as libc::c_int
                 } else {
                     (if 0 as libc::c_int == 1 as libc::c_int {
                          (((11 as libc::c_int * 2 as libc::c_int) as u32_0 &
                                (((0x1 as libc::c_int) << 8 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               16 as libc::c_int |
                               ((13 as libc::c_int * 2 as libc::c_int) as
                                    u32_0 &
                                    (((0x1 as libc::c_int) <<
                                          8 as libc::c_int) -
                                         1 as libc::c_int) as libc::c_uint) <<
                                   8 as libc::c_int) |
                              ((10 as libc::c_int * 2 as libc::c_int) as u32_0
                                   &
                                   (((0x1 as libc::c_int) << 8 as libc::c_int)
                                        - 1 as libc::c_int) as libc::c_uint)
                                  << 0 as libc::c_int
                      } else {
                          (((13 as libc::c_int * 2 as libc::c_int) as u32_0 &
                                (((0x1 as libc::c_int) << 8 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               16 as libc::c_int |
                               ((10 as libc::c_int * 2 as libc::c_int) as
                                    u32_0 &
                                    (((0x1 as libc::c_int) <<
                                          8 as libc::c_int) -
                                         1 as libc::c_int) as libc::c_uint) <<
                                   8 as libc::c_int) |
                              ((11 as libc::c_int * 2 as libc::c_int) as u32_0
                                   &
                                   (((0x1 as libc::c_int) << 8 as libc::c_int)
                                        - 1 as libc::c_int) as libc::c_uint)
                                  << 0 as libc::c_int
                      })
                 });
        (*_g_5).words.w1 =
            if 0 as libc::c_int == 0 as libc::c_int {
                (((10 as libc::c_int * 2 as libc::c_int) as u32_0 &
                      (((0x1 as libc::c_int) << 8 as libc::c_int) -
                           1 as libc::c_int) as libc::c_uint) <<
                     16 as libc::c_int |
                     ((13 as libc::c_int * 2 as libc::c_int) as u32_0 &
                          (((0x1 as libc::c_int) << 8 as libc::c_int) -
                               1 as libc::c_int) as libc::c_uint) <<
                         8 as libc::c_int) |
                    ((12 as libc::c_int * 2 as libc::c_int) as u32_0 &
                         (((0x1 as libc::c_int) << 8 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        0 as libc::c_int
            } else if 0 as libc::c_int == 1 as libc::c_int {
                (((13 as libc::c_int * 2 as libc::c_int) as u32_0 &
                      (((0x1 as libc::c_int) << 8 as libc::c_int) -
                           1 as libc::c_int) as libc::c_uint) <<
                     16 as libc::c_int |
                     ((12 as libc::c_int * 2 as libc::c_int) as u32_0 &
                          (((0x1 as libc::c_int) << 8 as libc::c_int) -
                               1 as libc::c_int) as libc::c_uint) <<
                         8 as libc::c_int) |
                    ((10 as libc::c_int * 2 as libc::c_int) as u32_0 &
                         (((0x1 as libc::c_int) << 8 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        0 as libc::c_int
            } else {
                (((12 as libc::c_int * 2 as libc::c_int) as u32_0 &
                      (((0x1 as libc::c_int) << 8 as libc::c_int) -
                           1 as libc::c_int) as libc::c_uint) <<
                     16 as libc::c_int |
                     ((10 as libc::c_int * 2 as libc::c_int) as u32_0 &
                          (((0x1 as libc::c_int) << 8 as libc::c_int) -
                               1 as libc::c_int) as libc::c_uint) <<
                         8 as libc::c_int) |
                    ((13 as libc::c_int * 2 as libc::c_int) as u32_0 &
                         (((0x1 as libc::c_int) << 8 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        0 as libc::c_int
            };
        let fresh9 = (*__gfxCtx).polyXlu.p;
        (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
        let mut _g_6: *mut Gfx = fresh9;
        (*_g_6).words.w0 =
            (0x6 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 24 as libc::c_int
                |
                (if 0 as libc::c_int == 0 as libc::c_int {
                     (((12 as libc::c_int * 2 as libc::c_int) as u32_0 &
                           (((0x1 as libc::c_int) << 8 as libc::c_int) -
                                1 as libc::c_int) as libc::c_uint) <<
                          16 as libc::c_int |
                          ((13 as libc::c_int * 2 as libc::c_int) as u32_0 &
                               (((0x1 as libc::c_int) << 8 as libc::c_int) -
                                    1 as libc::c_int) as libc::c_uint) <<
                              8 as libc::c_int) |
                         ((15 as libc::c_int * 2 as libc::c_int) as u32_0 &
                              (((0x1 as libc::c_int) << 8 as libc::c_int) -
                                   1 as libc::c_int) as libc::c_uint) <<
                             0 as libc::c_int
                 } else {
                     (if 0 as libc::c_int == 1 as libc::c_int {
                          (((13 as libc::c_int * 2 as libc::c_int) as u32_0 &
                                (((0x1 as libc::c_int) << 8 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               16 as libc::c_int |
                               ((15 as libc::c_int * 2 as libc::c_int) as
                                    u32_0 &
                                    (((0x1 as libc::c_int) <<
                                          8 as libc::c_int) -
                                         1 as libc::c_int) as libc::c_uint) <<
                                   8 as libc::c_int) |
                              ((12 as libc::c_int * 2 as libc::c_int) as u32_0
                                   &
                                   (((0x1 as libc::c_int) << 8 as libc::c_int)
                                        - 1 as libc::c_int) as libc::c_uint)
                                  << 0 as libc::c_int
                      } else {
                          (((15 as libc::c_int * 2 as libc::c_int) as u32_0 &
                                (((0x1 as libc::c_int) << 8 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               16 as libc::c_int |
                               ((12 as libc::c_int * 2 as libc::c_int) as
                                    u32_0 &
                                    (((0x1 as libc::c_int) <<
                                          8 as libc::c_int) -
                                         1 as libc::c_int) as libc::c_uint) <<
                                   8 as libc::c_int) |
                              ((13 as libc::c_int * 2 as libc::c_int) as u32_0
                                   &
                                   (((0x1 as libc::c_int) << 8 as libc::c_int)
                                        - 1 as libc::c_int) as libc::c_uint)
                                  << 0 as libc::c_int
                      })
                 });
        (*_g_6).words.w1 =
            if 0 as libc::c_int == 0 as libc::c_int {
                (((12 as libc::c_int * 2 as libc::c_int) as u32_0 &
                      (((0x1 as libc::c_int) << 8 as libc::c_int) -
                           1 as libc::c_int) as libc::c_uint) <<
                     16 as libc::c_int |
                     ((15 as libc::c_int * 2 as libc::c_int) as u32_0 &
                          (((0x1 as libc::c_int) << 8 as libc::c_int) -
                               1 as libc::c_int) as libc::c_uint) <<
                         8 as libc::c_int) |
                    ((14 as libc::c_int * 2 as libc::c_int) as u32_0 &
                         (((0x1 as libc::c_int) << 8 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        0 as libc::c_int
            } else if 0 as libc::c_int == 1 as libc::c_int {
                (((15 as libc::c_int * 2 as libc::c_int) as u32_0 &
                      (((0x1 as libc::c_int) << 8 as libc::c_int) -
                           1 as libc::c_int) as libc::c_uint) <<
                     16 as libc::c_int |
                     ((14 as libc::c_int * 2 as libc::c_int) as u32_0 &
                          (((0x1 as libc::c_int) << 8 as libc::c_int) -
                               1 as libc::c_int) as libc::c_uint) <<
                         8 as libc::c_int) |
                    ((12 as libc::c_int * 2 as libc::c_int) as u32_0 &
                         (((0x1 as libc::c_int) << 8 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        0 as libc::c_int
            } else {
                (((14 as libc::c_int * 2 as libc::c_int) as u32_0 &
                      (((0x1 as libc::c_int) << 8 as libc::c_int) -
                           1 as libc::c_int) as libc::c_uint) <<
                     16 as libc::c_int |
                     ((12 as libc::c_int * 2 as libc::c_int) as u32_0 &
                          (((0x1 as libc::c_int) << 8 as libc::c_int) -
                               1 as libc::c_int) as libc::c_uint) <<
                         8 as libc::c_int) |
                    ((15 as libc::c_int * 2 as libc::c_int) as u32_0 &
                         (((0x1 as libc::c_int) << 8 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        0 as libc::c_int
            }
    }
    Graph_CloseDisps(dispRefs.as_mut_ptr(), gfxCtx,
                     b"../z_eff_blure.c\x00" as *const u8 as
                         *const libc::c_char, 1184 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn EffectBlure_DrawSmooth(mut this2: *mut EffectBlure,
                                                mut gfxCtx:
                                                    *mut GraphicsContext) {
    let mut this: *mut EffectBlure = this2;
    let mut elem: *mut EffectBlureElement = 0 as *mut EffectBlureElement;
    let mut i: s32 = 0;
    let mut spDC: MtxF = MtxF{mf: [[0.; 4]; 4],};
    let mut sp9C: MtxF = MtxF{mf: [[0.; 4]; 4],};
    let mut sp5C: MtxF = MtxF{mf: [[0.; 4]; 4],};
    let mut mtx: *mut Mtx = 0 as *mut Mtx;
    let mut __gfxCtx: *mut GraphicsContext = 0 as *mut GraphicsContext;
    let mut dispRefs: [*mut Gfx; 4] = [0 as *mut Gfx; 4];
    __gfxCtx = gfxCtx;
    Graph_OpenDisps(dispRefs.as_mut_ptr(), gfxCtx,
                    b"../z_eff_blure.c\x00" as *const u8 as
                        *const libc::c_char, 1201 as libc::c_int);
    if ((*this).numElements as libc::c_int) < 2 as libc::c_int { return }
    (*this).elements[0 as libc::c_int as usize].flags =
        ((*this).elements[0 as libc::c_int as usize].flags as libc::c_int &
             !(3 as libc::c_int)) as u16_0;
    (*this).elements[0 as libc::c_int as usize].flags =
        ((*this).elements[0 as libc::c_int as usize].flags as libc::c_int |
             2 as libc::c_int) as u16_0;
    elem =
        &mut *(*this).elements.as_mut_ptr().offset(1 as libc::c_int as isize)
            as *mut EffectBlureElement;
    while elem <
              (*this).elements.as_mut_ptr().offset((*this).numElements as
                                                       libc::c_int as
                                                       isize).offset(-(1 as
                                                                           libc::c_int
                                                                           as
                                                                           isize))
          {
        EffectBlure_UpdateFlags(elem);
        elem = elem.offset(1)
    }
    (*this).elements[((*this).numElements as libc::c_int - 1 as libc::c_int)
                         as usize].flags =
        ((*this).elements[((*this).numElements as libc::c_int -
                               1 as libc::c_int) as usize].flags as
             libc::c_int & !(3 as libc::c_int)) as u16_0;
    (*this).elements[((*this).numElements as libc::c_int - 1 as libc::c_int)
                         as usize].flags =
        ((*this).elements[((*this).numElements as libc::c_int -
                               1 as libc::c_int) as usize].flags as
             libc::c_int | 2 as libc::c_int) as u16_0;
    EffectBlure_SetupSmooth(this, gfxCtx);
    SkinMatrix_SetTranslate(&mut spDC,
                            (*this).elements[0 as libc::c_int as usize].p2.x
                                as f32_0,
                            (*this).elements[0 as libc::c_int as usize].p2.y
                                as f32_0,
                            (*this).elements[0 as libc::c_int as usize].p2.z
                                as f32_0);
    SkinMatrix_SetScale(&mut sp9C, 0.1f32, 0.1f32, 0.1f32);
    SkinMatrix_MtxFMtxFMult(&mut spDC, &mut sp9C, &mut sp5C);
    mtx = SkinMatrix_MtxFToNewMtx(gfxCtx, &mut sp5C);
    if mtx.is_null() { return }
    let fresh10 = (*__gfxCtx).polyXlu.p;
    (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
    let mut _g: *mut Gfx = fresh10;
    (*_g).words.w0 =
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
    (*_g).words.w1 = mtx as libc::c_uint;
    i = 0 as libc::c_int;
    elem =
        &mut *(*this).elements.as_mut_ptr().offset(0 as libc::c_int as isize)
            as *mut EffectBlureElement;
    while elem <
              (*this).elements.as_mut_ptr().offset((*this).numElements as
                                                       libc::c_int as
                                                       isize).offset(-(1 as
                                                                           libc::c_int
                                                                           as
                                                                           isize))
          {
        if !((*elem).state == 0 as libc::c_int ||
                 (*elem.offset(1 as libc::c_int as isize)).state ==
                     0 as libc::c_int) {
            if (*elem).flags as libc::c_int & 3 as libc::c_int ==
                   0 as libc::c_int &&
                   (*elem.offset(1 as libc::c_int as isize)).flags as
                       libc::c_int & 3 as libc::c_int == 0 as libc::c_int ||
                   (*elem).flags as libc::c_int & 3 as libc::c_int ==
                       2 as libc::c_int &&
                       (*elem.offset(1 as libc::c_int as isize)).flags as
                           libc::c_int & 3 as libc::c_int == 0 as libc::c_int
                   ||
                   (*elem).flags as libc::c_int & 3 as libc::c_int ==
                       0 as libc::c_int &&
                       (*elem.offset(1 as libc::c_int as isize)).flags as
                           libc::c_int & 3 as libc::c_int == 2 as libc::c_int
                   ||
                   (*elem).flags as libc::c_int & 3 as libc::c_int ==
                       2 as libc::c_int &&
                       (*elem.offset(1 as libc::c_int as isize)).flags as
                           libc::c_int & 3 as libc::c_int == 2 as libc::c_int
               {
                EffectBlure_DrawElemNoInterpolation(this, elem, i, gfxCtx);
            } else {
                EffectBlure_DrawElemHermiteInterpolation(this, elem, i,
                                                         gfxCtx);
            }
        }
        i += 1;
        elem = elem.offset(1)
    }
    Graph_CloseDisps(dispRefs.as_mut_ptr(), gfxCtx,
                     b"../z_eff_blure.c\x00" as *const u8 as
                         *const libc::c_char, 1263 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn EffectBlure_SetupSimple(mut gfxCtx:
                                                     *mut GraphicsContext,
                                                 mut this: *mut EffectBlure,
                                                 mut vtx: *mut Vtx) {
    let mut __gfxCtx: *mut GraphicsContext = 0 as *mut GraphicsContext;
    let mut dispRefs: [*mut Gfx; 4] = [0 as *mut Gfx; 4];
    __gfxCtx = gfxCtx;
    Graph_OpenDisps(dispRefs.as_mut_ptr(), gfxCtx,
                    b"../z_eff_blure.c\x00" as *const u8 as
                        *const libc::c_char, 1280 as libc::c_int);
    (*__gfxCtx).polyXlu.p =
        Gfx_CallSetupDL((*__gfxCtx).polyXlu.p, 0x26 as libc::c_int as u32_0);
    Graph_CloseDisps(dispRefs.as_mut_ptr(), gfxCtx,
                     b"../z_eff_blure.c\x00" as *const u8 as
                         *const libc::c_char, 1285 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn EffectBlure_SetupSimpleAlt(mut gfxCtx:
                                                        *mut GraphicsContext,
                                                    mut this:
                                                        *mut EffectBlure,
                                                    mut vtx: *mut Vtx) {
    let mut __gfxCtx: *mut GraphicsContext = 0 as *mut GraphicsContext;
    let mut dispRefs: [*mut Gfx; 4] = [0 as *mut Gfx; 4];
    __gfxCtx = gfxCtx;
    Graph_OpenDisps(dispRefs.as_mut_ptr(), gfxCtx,
                    b"../z_eff_blure.c\x00" as *const u8 as
                        *const libc::c_char, 1294 as libc::c_int);
    let fresh11 = (*__gfxCtx).polyXlu.p;
    (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
    let mut _g: *mut Gfx = fresh11;
    (*_g).words.w0 =
        (0xe7 as libc::c_int as u32_0 &
             (((0x1 as libc::c_int) << 8 as libc::c_int) - 1 as libc::c_int)
                 as libc::c_uint) << 24 as libc::c_int;
    (*_g).words.w1 = 0 as libc::c_int as libc::c_uint;
    (*__gfxCtx).polyXlu.p =
        Gfx_CallSetupDL((*__gfxCtx).polyXlu.p, 0x26 as libc::c_int as u32_0);
    let fresh12 = (*__gfxCtx).polyXlu.p;
    (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
    let mut _g_0: *mut Gfx = fresh12;
    (*_g_0).words.w0 =
        (0xe3 as libc::c_int as u32_0 &
             (((0x1 as libc::c_int) << 8 as libc::c_int) - 1 as libc::c_int)
                 as libc::c_uint) << 24 as libc::c_int |
            ((32 as libc::c_int - 20 as libc::c_int - 2 as libc::c_int) as
                 u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 8 as libc::c_int |
            ((2 as libc::c_int - 1 as libc::c_int) as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 0 as libc::c_int;
    (*_g_0).words.w1 =
        ((1 as libc::c_int) << 20 as libc::c_int) as libc::c_uint;
    let fresh13 = (*__gfxCtx).polyXlu.p;
    (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
    let mut _g_1: *mut Gfx = fresh13;
    (*_g_1).words.w0 =
        (0xe3 as libc::c_int as u32_0 &
             (((0x1 as libc::c_int) << 8 as libc::c_int) - 1 as libc::c_int)
                 as libc::c_uint) << 24 as libc::c_int |
            ((32 as libc::c_int - 14 as libc::c_int - 2 as libc::c_int) as
                 u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 8 as libc::c_int |
            ((2 as libc::c_int - 1 as libc::c_int) as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 0 as libc::c_int;
    (*_g_1).words.w1 =
        ((0 as libc::c_int) << 14 as libc::c_int) as libc::c_uint;
    let fresh14 = (*__gfxCtx).polyXlu.p;
    (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
    let mut _g_2: *mut Gfx = fresh14;
    (*_g_2).words.w0 =
        (0xd7 as libc::c_int as u32_0 &
             (((0x1 as libc::c_int) << 8 as libc::c_int) - 1 as libc::c_int)
                 as libc::c_uint) << 24 as libc::c_int |
            (0 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 16 as libc::c_int
            |
            (0 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 3 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 11 as libc::c_int
            |
            (0 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 3 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 8 as libc::c_int |
            (1 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 7 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 1 as libc::c_int;
    (*_g_2).words.w1 =
        (0xffff as libc::c_int as u32_0 &
             (((0x1 as libc::c_int) << 16 as libc::c_int) - 1 as libc::c_int)
                 as libc::c_uint) << 16 as libc::c_int |
            (0xffff as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 16 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 0 as libc::c_int;
    let fresh15 = (*__gfxCtx).polyXlu.p;
    (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
    let mut _g_3: *mut Gfx = fresh15;
    (*_g_3).words.w0 =
        (0xfd as libc::c_int as u32_0 &
             (((0x1 as libc::c_int) << 8 as libc::c_int) - 1 as libc::c_int)
                 as libc::c_uint) << 24 as libc::c_int |
            (4 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 3 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 21 as libc::c_int
            |
            (2 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 2 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 19 as libc::c_int
            |
            ((1 as libc::c_int - 1 as libc::c_int) as u32_0 &
                 (((0x1 as libc::c_int) << 12 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 0 as libc::c_int;
    (*_g_3).words.w1 = gUnknownEffBlureTex.as_mut_ptr() as libc::c_uint;
    let fresh16 = (*__gfxCtx).polyXlu.p;
    (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
    let mut _g_4: *mut Gfx = fresh16;
    (*_g_4).words.w0 =
        (0xf5 as libc::c_int as u32_0 &
             (((0x1 as libc::c_int) << 8 as libc::c_int) - 1 as libc::c_int)
                 as libc::c_uint) << 24 as libc::c_int |
            (4 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 3 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 21 as libc::c_int
            |
            (2 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 2 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 19 as libc::c_int
            |
            (0 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 9 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 9 as libc::c_int |
            (0 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 9 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 0 as libc::c_int;
    (*_g_4).words.w1 =
        (7 as libc::c_int as u32_0 &
             (((0x1 as libc::c_int) << 3 as libc::c_int) - 1 as libc::c_int)
                 as libc::c_uint) << 24 as libc::c_int |
            (0 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 4 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 20 as libc::c_int
            |
            ((0 as libc::c_int | 0 as libc::c_int) as u32_0 &
                 (((0x1 as libc::c_int) << 2 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 18 as libc::c_int
            |
            (5 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 4 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 14 as libc::c_int
            |
            (0 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 4 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 10 as libc::c_int
            |
            ((0 as libc::c_int | 0x2 as libc::c_int) as u32_0 &
                 (((0x1 as libc::c_int) << 2 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 8 as libc::c_int |
            (6 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 4 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 4 as libc::c_int |
            (0 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 4 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 0 as libc::c_int;
    let fresh17 = (*__gfxCtx).polyXlu.p;
    (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
    let mut _g_5: *mut Gfx = fresh17;
    (*_g_5).words.w0 =
        (0xe6 as libc::c_int as u32_0 &
             (((0x1 as libc::c_int) << 8 as libc::c_int) - 1 as libc::c_int)
                 as libc::c_uint) << 24 as libc::c_int;
    (*_g_5).words.w1 = 0 as libc::c_int as libc::c_uint;
    let fresh18 = (*__gfxCtx).polyXlu.p;
    (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
    let mut _g_6: *mut Gfx = fresh18;
    (*_g_6).words.w0 =
        (0xf3 as libc::c_int as u32_0 &
             (((0x1 as libc::c_int) << 8 as libc::c_int) - 1 as libc::c_int)
                 as libc::c_uint) << 24 as libc::c_int |
            (0 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 12 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 12 as libc::c_int
            |
            (0 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 12 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 0 as libc::c_int;
    (*_g_6).words.w1 =
        (7 as libc::c_int as u32_0 &
             (((0x1 as libc::c_int) << 3 as libc::c_int) - 1 as libc::c_int)
                 as libc::c_uint) << 24 as libc::c_int |
            ((if ((64 as libc::c_int * 32 as libc::c_int + 1 as libc::c_int >>
                       1 as libc::c_int) - 1 as libc::c_int) <
                     2047 as libc::c_int {
                  (64 as libc::c_int * 32 as libc::c_int + 1 as libc::c_int >>
                       1 as libc::c_int) - 1 as libc::c_int
              } else { 2047 as libc::c_int }) as u32_0 &
                 (((0x1 as libc::c_int) << 12 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 12 as libc::c_int
            |
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
                      1 as libc::c_int) as libc::c_uint) << 0 as libc::c_int;
    let fresh19 = (*__gfxCtx).polyXlu.p;
    (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
    let mut _g_7: *mut Gfx = fresh19;
    (*_g_7).words.w0 =
        (0xe7 as libc::c_int as u32_0 &
             (((0x1 as libc::c_int) << 8 as libc::c_int) - 1 as libc::c_int)
                 as libc::c_uint) << 24 as libc::c_int;
    (*_g_7).words.w1 = 0 as libc::c_int as libc::c_uint;
    let fresh20 = (*__gfxCtx).polyXlu.p;
    (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
    let mut _g_8: *mut Gfx = fresh20;
    (*_g_8).words.w0 =
        (0xf5 as libc::c_int as u32_0 &
             (((0x1 as libc::c_int) << 8 as libc::c_int) - 1 as libc::c_int)
                 as libc::c_uint) << 24 as libc::c_int |
            (4 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 3 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 21 as libc::c_int
            |
            (1 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 2 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 19 as libc::c_int
            |
            ((64 as libc::c_int * 1 as libc::c_int + 7 as libc::c_int >>
                  3 as libc::c_int) as u32_0 &
                 (((0x1 as libc::c_int) << 9 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 9 as libc::c_int |
            (0 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 9 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 0 as libc::c_int;
    (*_g_8).words.w1 =
        (0 as libc::c_int as u32_0 &
             (((0x1 as libc::c_int) << 3 as libc::c_int) - 1 as libc::c_int)
                 as libc::c_uint) << 24 as libc::c_int |
            (0 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 4 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 20 as libc::c_int
            |
            ((0 as libc::c_int | 0 as libc::c_int) as u32_0 &
                 (((0x1 as libc::c_int) << 2 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 18 as libc::c_int
            |
            (5 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 4 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 14 as libc::c_int
            |
            (0 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 4 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 10 as libc::c_int
            |
            ((0 as libc::c_int | 0x2 as libc::c_int) as u32_0 &
                 (((0x1 as libc::c_int) << 2 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 8 as libc::c_int |
            (6 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 4 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 4 as libc::c_int |
            (0 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 4 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 0 as libc::c_int;
    let fresh21 = (*__gfxCtx).polyXlu.p;
    (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
    let mut _g_9: *mut Gfx = fresh21;
    (*_g_9).words.w0 =
        (0xf2 as libc::c_int as u32_0 &
             (((0x1 as libc::c_int) << 8 as libc::c_int) - 1 as libc::c_int)
                 as libc::c_uint) << 24 as libc::c_int |
            (0 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 12 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 12 as libc::c_int
            |
            (0 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 12 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 0 as libc::c_int;
    (*_g_9).words.w1 =
        (0 as libc::c_int as u32_0 &
             (((0x1 as libc::c_int) << 3 as libc::c_int) - 1 as libc::c_int)
                 as libc::c_uint) << 24 as libc::c_int |
            (((64 as libc::c_int - 1 as libc::c_int) << 2 as libc::c_int) as
                 u32_0 &
                 (((0x1 as libc::c_int) << 12 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 12 as libc::c_int
            |
            (((32 as libc::c_int - 1 as libc::c_int) << 2 as libc::c_int) as
                 u32_0 &
                 (((0x1 as libc::c_int) << 12 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 0 as libc::c_int;
    let fresh22 = (*__gfxCtx).polyXlu.p;
    (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
    let mut _g_10: *mut Gfx = fresh22;
    (*_g_10).words.w0 =
        (0xfc as libc::c_int as u32_0 &
             (((0x1 as libc::c_int) << 8 as libc::c_int) - 1 as libc::c_int)
                 as libc::c_uint) << 24 as libc::c_int |
            (((1 as libc::c_int as u32_0 &
                   (((0x1 as libc::c_int) << 4 as libc::c_int) -
                        1 as libc::c_int) as libc::c_uint) <<
                  20 as libc::c_int |
                  (14 as libc::c_int as u32_0 &
                       (((0x1 as libc::c_int) << 5 as libc::c_int) -
                            1 as libc::c_int) as libc::c_uint) <<
                      15 as libc::c_int |
                  (1 as libc::c_int as u32_0 &
                       (((0x1 as libc::c_int) << 3 as libc::c_int) -
                            1 as libc::c_int) as libc::c_uint) <<
                      12 as libc::c_int |
                  (3 as libc::c_int as u32_0 &
                       (((0x1 as libc::c_int) << 3 as libc::c_int) -
                            1 as libc::c_int) as libc::c_uint) <<
                      9 as libc::c_int |
                  ((3 as libc::c_int as u32_0 &
                        (((0x1 as libc::c_int) << 4 as libc::c_int) -
                             1 as libc::c_int) as libc::c_uint) <<
                       5 as libc::c_int |
                       (0 as libc::c_int as u32_0 &
                            (((0x1 as libc::c_int) << 5 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           0 as libc::c_int)) &
                 (((0x1 as libc::c_int) << 24 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 0 as libc::c_int;
    (*_g_10).words.w1 =
        (3 as libc::c_int as u32_0 &
             (((0x1 as libc::c_int) << 4 as libc::c_int) - 1 as libc::c_int)
                 as libc::c_uint) << 28 as libc::c_int |
            (1 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 3 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 15 as libc::c_int
            |
            (7 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 3 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 12 as libc::c_int
            |
            (7 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 3 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 9 as libc::c_int |
            ((5 as libc::c_int as u32_0 &
                  (((0x1 as libc::c_int) << 4 as libc::c_int) -
                       1 as libc::c_int) as libc::c_uint) << 24 as libc::c_int
                 |
                 (7 as libc::c_int as u32_0 &
                      (((0x1 as libc::c_int) << 3 as libc::c_int) -
                           1 as libc::c_int) as libc::c_uint) <<
                     21 as libc::c_int |
                 (7 as libc::c_int as u32_0 &
                      (((0x1 as libc::c_int) << 3 as libc::c_int) -
                           1 as libc::c_int) as libc::c_uint) <<
                     18 as libc::c_int |
                 (5 as libc::c_int as u32_0 &
                      (((0x1 as libc::c_int) << 3 as libc::c_int) -
                           1 as libc::c_int) as libc::c_uint) <<
                     6 as libc::c_int |
                 (7 as libc::c_int as u32_0 &
                      (((0x1 as libc::c_int) << 3 as libc::c_int) -
                           1 as libc::c_int) as libc::c_uint) <<
                     3 as libc::c_int |
                 (0 as libc::c_int as u32_0 &
                      (((0x1 as libc::c_int) << 3 as libc::c_int) -
                           1 as libc::c_int) as libc::c_uint) <<
                     0 as libc::c_int);
    let fresh23 = (*__gfxCtx).polyXlu.p;
    (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
    let mut _g_11: *mut Gfx = fresh23;
    (*_g_11).words.w0 =
        (0xe2 as libc::c_int as u32_0 &
             (((0x1 as libc::c_int) << 8 as libc::c_int) - 1 as libc::c_int)
                 as libc::c_uint) << 24 as libc::c_int |
            ((32 as libc::c_int - 3 as libc::c_int - 29 as libc::c_int) as
                 u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 8 as libc::c_int |
            ((29 as libc::c_int - 1 as libc::c_int) as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 0 as libc::c_int;
    (*_g_11).words.w1 =
        ((0 as libc::c_int) << 30 as libc::c_int |
             (3 as libc::c_int) << 26 as libc::c_int |
             (0 as libc::c_int) << 22 as libc::c_int |
             (2 as libc::c_int) << 18 as libc::c_int |
             (0x10 as libc::c_int | 0x40 as libc::c_int | 0x300 as libc::c_int
                  | 0x4000 as libc::c_int | 0x800 as libc::c_int |
                  (0 as libc::c_int) << 28 as libc::c_int |
                  (0 as libc::c_int) << 24 as libc::c_int |
                  (1 as libc::c_int) << 20 as libc::c_int |
                  (0 as libc::c_int) << 16 as libc::c_int)) as libc::c_uint;
    let fresh24 = (*__gfxCtx).polyXlu.p;
    (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
    let mut _g_12: *mut Gfx = fresh24;
    (*_g_12).words.w0 =
        (0xd9 as libc::c_int as u32_0 &
             (((0x1 as libc::c_int) << 8 as libc::c_int) - 1 as libc::c_int)
                 as libc::c_uint) << 24 as libc::c_int |
            (!((0x10000 as libc::c_int | 0x20000 as libc::c_int |
                    0x40000 as libc::c_int | 0x80000 as libc::c_int) as u32_0)
                 &
                 (((0x1 as libc::c_int) << 24 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 0 as libc::c_int;
    (*_g_12).words.w1 = 0 as libc::c_int as u32_0;
    let fresh25 = (*__gfxCtx).polyXlu.p;
    (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
    let mut _g_13: *mut Gfx = fresh25;
    (*_g_13).words.w0 =
        (0xd9 as libc::c_int as u32_0 &
             (((0x1 as libc::c_int) << 8 as libc::c_int) - 1 as libc::c_int)
                 as libc::c_uint) << 24 as libc::c_int |
            (!(0 as libc::c_int as u32_0) &
                 (((0x1 as libc::c_int) << 24 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 0 as libc::c_int;
    (*_g_13).words.w1 =
        (0x1 as libc::c_int | 0x4 as libc::c_int | 0x200000 as libc::c_int) as
            u32_0;
    let fresh26 = (*__gfxCtx).polyXlu.p;
    (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
    let mut _g_14: *mut Gfx = fresh26;
    (*_g_14).words.w0 =
        (0xe7 as libc::c_int as u32_0 &
             (((0x1 as libc::c_int) << 8 as libc::c_int) - 1 as libc::c_int)
                 as libc::c_uint) << 24 as libc::c_int;
    (*_g_14).words.w1 = 0 as libc::c_int as libc::c_uint;
    let fresh27 = (*__gfxCtx).polyXlu.p;
    (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
    let mut _g_15: *mut Gfx = fresh27;
    (*_g_15).words.w0 =
        (0xfb as libc::c_int as u32_0 &
             (((0x1 as libc::c_int) << 8 as libc::c_int) - 1 as libc::c_int)
                 as libc::c_uint) << 24 as libc::c_int;
    (*_g_15).words.w1 =
        ((*this).altEnvColor.r as u32_0 &
             (((0x1 as libc::c_int) << 8 as libc::c_int) - 1 as libc::c_int)
                 as libc::c_uint) << 24 as libc::c_int |
            ((*this).altEnvColor.g as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 16 as libc::c_int
            |
            ((*this).altEnvColor.b as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 8 as libc::c_int |
            ((*this).altEnvColor.a as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 0 as libc::c_int;
    Graph_CloseDisps(dispRefs.as_mut_ptr(), gfxCtx,
                     b"../z_eff_blure.c\x00" as *const u8 as
                         *const libc::c_char, 1329 as libc::c_int);
}
#[no_mangle]
pub static mut sSetupHandlers:
           [Option<unsafe extern "C" fn(_: *mut GraphicsContext,
                                        _: *mut EffectBlure, _: *mut Vtx)
                       -> ()>; 2] =
    unsafe {
        [Some(EffectBlure_SetupSimple as
                  unsafe extern "C" fn(_: *mut GraphicsContext,
                                       _: *mut EffectBlure, _: *mut Vtx)
                      -> ()),
         Some(EffectBlure_SetupSimpleAlt as
                  unsafe extern "C" fn(_: *mut GraphicsContext,
                                       _: *mut EffectBlure, _: *mut Vtx)
                      -> ())]
    };
#[no_mangle]
pub static mut D_80115788: s32 = 0 as libc::c_int;
// unused
// original name: "EffectBlureInfo2_disp_makeDisplayList"
#[no_mangle]
pub unsafe extern "C" fn EffectBlure_DrawSimpleVertices(mut gfxCtx:
                                                            *mut GraphicsContext,
                                                        mut this:
                                                            *mut EffectBlure,
                                                        mut vtx: *mut Vtx) {
    let mut mtx: *mut Mtx = 0 as *mut Mtx;
    let mut __gfxCtx: *mut GraphicsContext = 0 as *mut GraphicsContext;
    let mut dispRefs: [*mut Gfx; 4] = [0 as *mut Gfx; 4];
    __gfxCtx = gfxCtx;
    Graph_OpenDisps(dispRefs.as_mut_ptr(), gfxCtx,
                    b"../z_eff_blure.c\x00" as *const u8 as
                        *const libc::c_char, 1356 as libc::c_int);
    sSetupHandlers[(*this).drawMode as
                       usize].expect("non-null function pointer")(gfxCtx,
                                                                  this, vtx);
    let fresh28 = (*__gfxCtx).polyXlu.p;
    (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
    let mut _g: *mut Gfx = fresh28;
    (*_g).words.w0 =
        (0xe7 as libc::c_int as u32_0 &
             (((0x1 as libc::c_int) << 8 as libc::c_int) - 1 as libc::c_int)
                 as libc::c_uint) << 24 as libc::c_int;
    (*_g).words.w1 = 0 as libc::c_int as libc::c_uint;
    let mut sp1B0: Vec3f = Vec3f{x: 0., y: 0., z: 0.,};
    let mut sp1A4: Vec3f = Vec3f{x: 0., y: 0., z: 0.,};
    let mut sp198: Vec3f = Vec3f{x: 0., y: 0., z: 0.,};
    let mut alphaRatio: f32_0 = 0.;
    let mut sp154: MtxF = MtxF{mf: [[0.; 4]; 4],};
    let mut sp114: MtxF = MtxF{mf: [[0.; 4]; 4],};
    let mut spD4: MtxF = MtxF{mf: [[0.; 4]; 4],};
    let mut sp94: MtxF = MtxF{mf: [[0.; 4]; 4],};
    let mut scale: f32_0 = 0.;
    let mut i: s32 = 0;
    let mut j: s32 = 0;
    j = 0 as libc::c_int;
    i = 0 as libc::c_int;
    while i < (*this).numElements as libc::c_int - 1 as libc::c_int {
        if (*this).drawMode as libc::c_int == 1 as libc::c_int {
            alphaRatio =
                (*this).elements[i as usize].timer as f32_0 /
                    (*this).elemDuration as f32_0;
            let fresh29 = (*__gfxCtx).polyXlu.p;
            (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
            let mut _g_0: *mut Gfx = fresh29;
            (*_g_0).words.w0 =
                (0xfa as libc::c_int as u32_0 &
                     (((0x1 as libc::c_int) << 8 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    24 as libc::c_int |
                    (0 as libc::c_int as u32_0 &
                         (((0x1 as libc::c_int) << 8 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        8 as libc::c_int |
                    (0x80 as libc::c_int as u32_0 &
                         (((0x1 as libc::c_int) << 8 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        0 as libc::c_int;
            (*_g_0).words.w1 =
                ((*this).altPrimColor.r as u32_0 &
                     (((0x1 as libc::c_int) << 8 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    24 as libc::c_int |
                    ((*this).altPrimColor.g as u32_0 &
                         (((0x1 as libc::c_int) << 8 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        16 as libc::c_int |
                    ((*this).altPrimColor.b as u32_0 &
                         (((0x1 as libc::c_int) << 8 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        8 as libc::c_int |
                    (((*this).altPrimColor.a as libc::c_int as libc::c_float *
                          (1.0f32 - alphaRatio)) as u32_0 &
                         (((0x1 as libc::c_int) << 8 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        0 as libc::c_int;
            let fresh30 = (*__gfxCtx).polyXlu.p;
            (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
            let mut _g_1: *mut Gfx = fresh30;
            (*_g_1).words.w0 =
                (0xe7 as libc::c_int as u32_0 &
                     (((0x1 as libc::c_int) << 8 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    24 as libc::c_int;
            (*_g_1).words.w1 = 0 as libc::c_int as libc::c_uint
        }
        // Necessary to match
        let fresh31 = (*__gfxCtx).polyXlu.p;
        (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
        let mut _g_2: *mut Gfx = fresh31;
        (*_g_2).words.w0 =
            (0x1 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 24 as libc::c_int
                |
                (4 as libc::c_int as u32_0 &
                     (((0x1 as libc::c_int) << 8 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    12 as libc::c_int |
                ((0 as libc::c_int + 4 as libc::c_int) as u32_0 &
                     (((0x1 as libc::c_int) << 7 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    1 as libc::c_int;
        (*_g_2).words.w1 =
            &mut *vtx.offset(j as isize) as *mut Vtx as libc::c_uint;
        let fresh32 = (*__gfxCtx).polyXlu.p;
        (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
        let mut _g_3: *mut Gfx = fresh32;
        (*_g_3).words.w0 =
            (0x6 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 24 as libc::c_int
                |
                (if 0 as libc::c_int == 0 as libc::c_int {
                     (((0 as libc::c_int * 2 as libc::c_int) as u32_0 &
                           (((0x1 as libc::c_int) << 8 as libc::c_int) -
                                1 as libc::c_int) as libc::c_uint) <<
                          16 as libc::c_int |
                          ((1 as libc::c_int * 2 as libc::c_int) as u32_0 &
                               (((0x1 as libc::c_int) << 8 as libc::c_int) -
                                    1 as libc::c_int) as libc::c_uint) <<
                              8 as libc::c_int) |
                         ((3 as libc::c_int * 2 as libc::c_int) as u32_0 &
                              (((0x1 as libc::c_int) << 8 as libc::c_int) -
                                   1 as libc::c_int) as libc::c_uint) <<
                             0 as libc::c_int
                 } else {
                     (if 0 as libc::c_int == 1 as libc::c_int {
                          (((1 as libc::c_int * 2 as libc::c_int) as u32_0 &
                                (((0x1 as libc::c_int) << 8 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               16 as libc::c_int |
                               ((3 as libc::c_int * 2 as libc::c_int) as u32_0
                                    &
                                    (((0x1 as libc::c_int) <<
                                          8 as libc::c_int) -
                                         1 as libc::c_int) as libc::c_uint) <<
                                   8 as libc::c_int) |
                              ((0 as libc::c_int * 2 as libc::c_int) as u32_0
                                   &
                                   (((0x1 as libc::c_int) << 8 as libc::c_int)
                                        - 1 as libc::c_int) as libc::c_uint)
                                  << 0 as libc::c_int
                      } else {
                          (((3 as libc::c_int * 2 as libc::c_int) as u32_0 &
                                (((0x1 as libc::c_int) << 8 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               16 as libc::c_int |
                               ((0 as libc::c_int * 2 as libc::c_int) as u32_0
                                    &
                                    (((0x1 as libc::c_int) <<
                                          8 as libc::c_int) -
                                         1 as libc::c_int) as libc::c_uint) <<
                                   8 as libc::c_int) |
                              ((1 as libc::c_int * 2 as libc::c_int) as u32_0
                                   &
                                   (((0x1 as libc::c_int) << 8 as libc::c_int)
                                        - 1 as libc::c_int) as libc::c_uint)
                                  << 0 as libc::c_int
                      })
                 });
        (*_g_3).words.w1 =
            if 0 as libc::c_int == 0 as libc::c_int {
                (((0 as libc::c_int * 2 as libc::c_int) as u32_0 &
                      (((0x1 as libc::c_int) << 8 as libc::c_int) -
                           1 as libc::c_int) as libc::c_uint) <<
                     16 as libc::c_int |
                     ((3 as libc::c_int * 2 as libc::c_int) as u32_0 &
                          (((0x1 as libc::c_int) << 8 as libc::c_int) -
                               1 as libc::c_int) as libc::c_uint) <<
                         8 as libc::c_int) |
                    ((2 as libc::c_int * 2 as libc::c_int) as u32_0 &
                         (((0x1 as libc::c_int) << 8 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        0 as libc::c_int
            } else if 0 as libc::c_int == 1 as libc::c_int {
                (((3 as libc::c_int * 2 as libc::c_int) as u32_0 &
                      (((0x1 as libc::c_int) << 8 as libc::c_int) -
                           1 as libc::c_int) as libc::c_uint) <<
                     16 as libc::c_int |
                     ((2 as libc::c_int * 2 as libc::c_int) as u32_0 &
                          (((0x1 as libc::c_int) << 8 as libc::c_int) -
                               1 as libc::c_int) as libc::c_uint) <<
                         8 as libc::c_int) |
                    ((0 as libc::c_int * 2 as libc::c_int) as u32_0 &
                         (((0x1 as libc::c_int) << 8 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        0 as libc::c_int
            } else {
                (((2 as libc::c_int * 2 as libc::c_int) as u32_0 &
                      (((0x1 as libc::c_int) << 8 as libc::c_int) -
                           1 as libc::c_int) as libc::c_uint) <<
                     16 as libc::c_int |
                     ((0 as libc::c_int * 2 as libc::c_int) as u32_0 &
                          (((0x1 as libc::c_int) << 8 as libc::c_int) -
                               1 as libc::c_int) as libc::c_uint) <<
                         8 as libc::c_int) |
                    ((3 as libc::c_int * 2 as libc::c_int) as u32_0 &
                         (((0x1 as libc::c_int) << 8 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        0 as libc::c_int
            };
        if (*this).flags as libc::c_int & 4 as libc::c_int != 0 {
            sp1B0.x =
                ((*vtx.offset((4 as libc::c_int * i + 0 as libc::c_int) as
                                  isize)).v.ob[0 as libc::c_int as usize] as
                     f32_0 +
                     (*vtx.offset((4 as libc::c_int * i + 1 as libc::c_int) as
                                      isize)).v.ob[0 as libc::c_int as usize]
                         as f32_0) * 0.5f32;
            sp1B0.y =
                ((*vtx.offset((4 as libc::c_int * i + 0 as libc::c_int) as
                                  isize)).v.ob[1 as libc::c_int as usize] as
                     f32_0 +
                     (*vtx.offset((4 as libc::c_int * i + 1 as libc::c_int) as
                                      isize)).v.ob[1 as libc::c_int as usize]
                         as f32_0) * 0.5f32;
            sp1B0.z =
                ((*vtx.offset((4 as libc::c_int * i + 0 as libc::c_int) as
                                  isize)).v.ob[2 as libc::c_int as usize] as
                     f32_0 +
                     (*vtx.offset((4 as libc::c_int * i + 1 as libc::c_int) as
                                      isize)).v.ob[2 as libc::c_int as usize]
                         as f32_0) * 0.5f32;
            sp1A4.x =
                ((*vtx.offset((4 as libc::c_int * i + 2 as libc::c_int) as
                                  isize)).v.ob[0 as libc::c_int as usize] as
                     f32_0 +
                     (*vtx.offset((4 as libc::c_int * i + 3 as libc::c_int) as
                                      isize)).v.ob[0 as libc::c_int as usize]
                         as f32_0) * 0.5f32;
            sp1A4.y =
                ((*vtx.offset((4 as libc::c_int * i + 2 as libc::c_int) as
                                  isize)).v.ob[1 as libc::c_int as usize] as
                     f32_0 +
                     (*vtx.offset((4 as libc::c_int * i + 3 as libc::c_int) as
                                      isize)).v.ob[1 as libc::c_int as usize]
                         as f32_0) * 0.5f32;
            sp1A4.z =
                ((*vtx.offset((4 as libc::c_int * i + 2 as libc::c_int) as
                                  isize)).v.ob[2 as libc::c_int as usize] as
                     f32_0 +
                     (*vtx.offset((4 as libc::c_int * i + 3 as libc::c_int) as
                                      isize)).v.ob[2 as libc::c_int as usize]
                         as f32_0) * 0.5f32;
            Math_Vec3f_Diff(&mut sp1A4, &mut sp1B0, &mut sp198);
            scale =
                sqrtf(sp198.x * sp198.x + sp198.y * sp198.y +
                          sp198.z * sp198.z);
            if fabsf(scale) > 0.0005f32 {
                scale = 1.0f32 / scale;
                Math_Vec3f_Scale(&mut sp198, scale);
                SkinMatrix_SetTranslate(&mut sp154, sp1B0.x, sp1B0.y,
                                        sp1B0.z);
                func_800A7EC0(&mut sp114, 0x3fff as libc::c_int as s16,
                              sp198.x, sp198.y, sp198.z);
                SkinMatrix_MtxFMtxFMult(&mut sp154, &mut sp114, &mut spD4);
                SkinMatrix_SetTranslate(&mut sp154, -sp1B0.x, -sp1B0.y,
                                        -sp1B0.z);
                SkinMatrix_MtxFMtxFMult(&mut spD4, &mut sp154, &mut sp94);
                mtx = SkinMatrix_MtxFToNewMtx(gfxCtx, &mut sp94);
                if mtx.is_null() {
                    // "Forced termination because a matrix cannot be taken"
                    osSyncPrintf(b"EffectBlureInfo2_disp_makeDisplayList()\xe3\x83\x9e\xe3\x83\x88\xe3\x83\xaa\xe3\x83\x83\xe3\x82\xaf\xe3\x82\xb9\xe5\x8f\x96\xe3\x82\x8c\xe3\x81\xaa\xe3\x81\x84\xe3\x81\xae\xe3\x81\xa7,\xe5\xbc\xb7\xe5\x88\xb6\xe7\xb5\x82\xe4\xba\x86\n\x00"
                                     as *const u8 as *const libc::c_char);
                    break ;
                } else {
                    let fresh33 = (*__gfxCtx).polyXlu.p;
                    (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
                    let mut _g_4: *mut Gfx = fresh33;
                    (*_g_4).words.w0 =
                        (0xda as libc::c_int as u32_0 &
                             (((0x1 as libc::c_int) << 8 as libc::c_int) -
                                  1 as libc::c_int) as libc::c_uint) <<
                            24 as libc::c_int |
                            ((::std::mem::size_of::<Mtx>() as
                                  libc::c_ulong).wrapping_sub(1 as libc::c_int
                                                                  as
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
                            (((0 as libc::c_int | 0x2 as libc::c_int |
                                   0 as libc::c_int) ^ 0x1 as libc::c_int) as
                                 u32_0 &
                                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                                      1 as libc::c_int) as libc::c_uint) <<
                                0 as libc::c_int;
                    (*_g_4).words.w1 = mtx as libc::c_uint;
                    let fresh34 = (*__gfxCtx).polyXlu.p;
                    (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
                    let mut _g_5: *mut Gfx = fresh34;
                    (*_g_5).words.w0 =
                        (0x1 as libc::c_int as u32_0 &
                             (((0x1 as libc::c_int) << 8 as libc::c_int) -
                                  1 as libc::c_int) as libc::c_uint) <<
                            24 as libc::c_int |
                            (4 as libc::c_int as u32_0 &
                                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                                      1 as libc::c_int) as libc::c_uint) <<
                                12 as libc::c_int |
                            ((0 as libc::c_int + 4 as libc::c_int) as u32_0 &
                                 (((0x1 as libc::c_int) << 7 as libc::c_int) -
                                      1 as libc::c_int) as libc::c_uint) <<
                                1 as libc::c_int;
                    (*_g_5).words.w1 =
                        &mut *vtx.offset(j as isize) as *mut Vtx as
                            libc::c_uint;
                    let fresh35 = (*__gfxCtx).polyXlu.p;
                    (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
                    let mut _g_6: *mut Gfx = fresh35;
                    (*_g_6).words.w0 =
                        (0x6 as libc::c_int as u32_0 &
                             (((0x1 as libc::c_int) << 8 as libc::c_int) -
                                  1 as libc::c_int) as libc::c_uint) <<
                            24 as libc::c_int |
                            (if 0 as libc::c_int == 0 as libc::c_int {
                                 (((0 as libc::c_int * 2 as libc::c_int) as
                                       u32_0 &
                                       (((0x1 as libc::c_int) <<
                                             8 as libc::c_int) -
                                            1 as libc::c_int) as libc::c_uint)
                                      << 16 as libc::c_int |
                                      ((1 as libc::c_int * 2 as libc::c_int)
                                           as u32_0 &
                                           (((0x1 as libc::c_int) <<
                                                 8 as libc::c_int) -
                                                1 as libc::c_int) as
                                               libc::c_uint) <<
                                          8 as libc::c_int) |
                                     ((3 as libc::c_int * 2 as libc::c_int) as
                                          u32_0 &
                                          (((0x1 as libc::c_int) <<
                                                8 as libc::c_int) -
                                               1 as libc::c_int) as
                                              libc::c_uint) <<
                                         0 as libc::c_int
                             } else {
                                 (if 0 as libc::c_int == 1 as libc::c_int {
                                      (((1 as libc::c_int * 2 as libc::c_int)
                                            as u32_0 &
                                            (((0x1 as libc::c_int) <<
                                                  8 as libc::c_int) -
                                                 1 as libc::c_int) as
                                                libc::c_uint) <<
                                           16 as libc::c_int |
                                           ((3 as libc::c_int *
                                                 2 as libc::c_int) as u32_0 &
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
                                      (((3 as libc::c_int * 2 as libc::c_int)
                                            as u32_0 &
                                            (((0x1 as libc::c_int) <<
                                                  8 as libc::c_int) -
                                                 1 as libc::c_int) as
                                                libc::c_uint) <<
                                           16 as libc::c_int |
                                           ((0 as libc::c_int *
                                                 2 as libc::c_int) as u32_0 &
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
                                  })
                             });
                    (*_g_6).words.w1 =
                        if 0 as libc::c_int == 0 as libc::c_int {
                            (((0 as libc::c_int * 2 as libc::c_int) as u32_0 &
                                  (((0x1 as libc::c_int) << 8 as libc::c_int)
                                       - 1 as libc::c_int) as libc::c_uint) <<
                                 16 as libc::c_int |
                                 ((3 as libc::c_int * 2 as libc::c_int) as
                                      u32_0 &
                                      (((0x1 as libc::c_int) <<
                                            8 as libc::c_int) -
                                           1 as libc::c_int) as libc::c_uint)
                                     << 8 as libc::c_int) |
                                ((2 as libc::c_int * 2 as libc::c_int) as
                                     u32_0 &
                                     (((0x1 as libc::c_int) <<
                                           8 as libc::c_int) -
                                          1 as libc::c_int) as libc::c_uint)
                                    << 0 as libc::c_int
                        } else if 0 as libc::c_int == 1 as libc::c_int {
                            (((3 as libc::c_int * 2 as libc::c_int) as u32_0 &
                                  (((0x1 as libc::c_int) << 8 as libc::c_int)
                                       - 1 as libc::c_int) as libc::c_uint) <<
                                 16 as libc::c_int |
                                 ((2 as libc::c_int * 2 as libc::c_int) as
                                      u32_0 &
                                      (((0x1 as libc::c_int) <<
                                            8 as libc::c_int) -
                                           1 as libc::c_int) as libc::c_uint)
                                     << 8 as libc::c_int) |
                                ((0 as libc::c_int * 2 as libc::c_int) as
                                     u32_0 &
                                     (((0x1 as libc::c_int) <<
                                           8 as libc::c_int) -
                                          1 as libc::c_int) as libc::c_uint)
                                    << 0 as libc::c_int
                        } else {
                            (((2 as libc::c_int * 2 as libc::c_int) as u32_0 &
                                  (((0x1 as libc::c_int) << 8 as libc::c_int)
                                       - 1 as libc::c_int) as libc::c_uint) <<
                                 16 as libc::c_int |
                                 ((0 as libc::c_int * 2 as libc::c_int) as
                                      u32_0 &
                                      (((0x1 as libc::c_int) <<
                                            8 as libc::c_int) -
                                           1 as libc::c_int) as libc::c_uint)
                                     << 8 as libc::c_int) |
                                ((3 as libc::c_int * 2 as libc::c_int) as
                                     u32_0 &
                                     (((0x1 as libc::c_int) <<
                                           8 as libc::c_int) -
                                          1 as libc::c_int) as libc::c_uint)
                                    << 0 as libc::c_int
                        };
                    let fresh36 = (*__gfxCtx).polyXlu.p;
                    (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
                    let mut _g_7: *mut Gfx = fresh36;
                    (*_g_7).words.w0 =
                        (0xda as libc::c_int as u32_0 &
                             (((0x1 as libc::c_int) << 8 as libc::c_int) -
                                  1 as libc::c_int) as libc::c_uint) <<
                            24 as libc::c_int |
                            ((::std::mem::size_of::<Mtx>() as
                                  libc::c_ulong).wrapping_sub(1 as libc::c_int
                                                                  as
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
                            (((0 as libc::c_int | 0x2 as libc::c_int |
                                   0 as libc::c_int) ^ 0x1 as libc::c_int) as
                                 u32_0 &
                                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                                      1 as libc::c_int) as libc::c_uint) <<
                                0 as libc::c_int;
                    (*_g_7).words.w1 =
                        &mut gMtxClear as *mut Mtx as libc::c_uint
                }
            }
        }
        j += 4 as libc::c_int;
        i += 1
    }
    Graph_CloseDisps(dispRefs.as_mut_ptr(), gfxCtx,
                     b"../z_eff_blure.c\x00" as *const u8 as
                         *const libc::c_char, 1452 as libc::c_int);
}
#[no_mangle]
pub static mut D_8011578C: [Vtx_t; 4] =
    [{
         let mut init =
             Vtx_t{ob:
                       [0 as libc::c_int as libc::c_short,
                        0 as libc::c_int as libc::c_short,
                        0 as libc::c_int as libc::c_short],
                   flag: 0 as libc::c_int as libc::c_ushort,
                   tc:
                       [0 as libc::c_int as libc::c_short,
                        1024 as libc::c_int as libc::c_short],
                   cn:
                       [0xff as libc::c_int as libc::c_uchar,
                        0xff as libc::c_int as libc::c_uchar,
                        0xff as libc::c_int as libc::c_uchar,
                        0xff as libc::c_int as libc::c_uchar],};
         init
     },
     {
         let mut init =
             Vtx_t{ob:
                       [0 as libc::c_int as libc::c_short,
                        0 as libc::c_int as libc::c_short,
                        0 as libc::c_int as libc::c_short],
                   flag: 0 as libc::c_int as libc::c_ushort,
                   tc:
                       [0 as libc::c_int as libc::c_short,
                        0 as libc::c_int as libc::c_short],
                   cn:
                       [0xff as libc::c_int as libc::c_uchar,
                        0xff as libc::c_int as libc::c_uchar,
                        0xff as libc::c_int as libc::c_uchar,
                        0xff as libc::c_int as libc::c_uchar],};
         init
     },
     {
         let mut init =
             Vtx_t{ob:
                       [0 as libc::c_int as libc::c_short,
                        0 as libc::c_int as libc::c_short,
                        0 as libc::c_int as libc::c_short],
                   flag: 0 as libc::c_int as libc::c_ushort,
                   tc:
                       [2048 as libc::c_int as libc::c_short,
                        1024 as libc::c_int as libc::c_short],
                   cn:
                       [0xff as libc::c_int as libc::c_uchar,
                        0xff as libc::c_int as libc::c_uchar,
                        0xff as libc::c_int as libc::c_uchar,
                        0xff as libc::c_int as libc::c_uchar],};
         init
     },
     {
         let mut init =
             Vtx_t{ob:
                       [0 as libc::c_int as libc::c_short,
                        0 as libc::c_int as libc::c_short,
                        0 as libc::c_int as libc::c_short],
                   flag: 0 as libc::c_int as libc::c_ushort,
                   tc:
                       [2048 as libc::c_int as libc::c_short,
                        0 as libc::c_int as libc::c_short],
                   cn:
                       [0xff as libc::c_int as libc::c_uchar,
                        0xff as libc::c_int as libc::c_uchar,
                        0xff as libc::c_int as libc::c_uchar,
                        0xff as libc::c_int as libc::c_uchar],};
         init
     }];
#[no_mangle]
pub static mut D_801157CC: [Vtx_t; 4] =
    [{
         let mut init =
             Vtx_t{ob:
                       [0 as libc::c_int as libc::c_short,
                        0 as libc::c_int as libc::c_short,
                        0 as libc::c_int as libc::c_short],
                   flag: 0 as libc::c_int as libc::c_ushort,
                   tc:
                       [2048 as libc::c_int as libc::c_short,
                        1024 as libc::c_int as libc::c_short],
                   cn:
                       [0xff as libc::c_int as libc::c_uchar,
                        0xff as libc::c_int as libc::c_uchar,
                        0xff as libc::c_int as libc::c_uchar,
                        0xff as libc::c_int as libc::c_uchar],};
         init
     },
     {
         let mut init =
             Vtx_t{ob:
                       [0 as libc::c_int as libc::c_short,
                        0 as libc::c_int as libc::c_short,
                        0 as libc::c_int as libc::c_short],
                   flag: 0 as libc::c_int as libc::c_ushort,
                   tc:
                       [2048 as libc::c_int as libc::c_short,
                        0 as libc::c_int as libc::c_short],
                   cn:
                       [0xff as libc::c_int as libc::c_uchar,
                        0xff as libc::c_int as libc::c_uchar,
                        0xff as libc::c_int as libc::c_uchar,
                        0xff as libc::c_int as libc::c_uchar],};
         init
     },
     {
         let mut init =
             Vtx_t{ob:
                       [0 as libc::c_int as libc::c_short,
                        0 as libc::c_int as libc::c_short,
                        0 as libc::c_int as libc::c_short],
                   flag: 0 as libc::c_int as libc::c_ushort,
                   tc:
                       [2048 as libc::c_int as libc::c_short,
                        1024 as libc::c_int as libc::c_short],
                   cn:
                       [0xff as libc::c_int as libc::c_uchar,
                        0xff as libc::c_int as libc::c_uchar,
                        0xff as libc::c_int as libc::c_uchar,
                        0xff as libc::c_int as libc::c_uchar],};
         init
     },
     {
         let mut init =
             Vtx_t{ob:
                       [0 as libc::c_int as libc::c_short,
                        0 as libc::c_int as libc::c_short,
                        0 as libc::c_int as libc::c_short],
                   flag: 0 as libc::c_int as libc::c_ushort,
                   tc:
                       [2048 as libc::c_int as libc::c_short,
                        0 as libc::c_int as libc::c_short],
                   cn:
                       [0xff as libc::c_int as libc::c_uchar,
                        0xff as libc::c_int as libc::c_uchar,
                        0xff as libc::c_int as libc::c_uchar,
                        0xff as libc::c_int as libc::c_uchar],};
         init
     }];
#[no_mangle]
pub unsafe extern "C" fn EffectBlure_DrawSimple(mut this2: *mut EffectBlure,
                                                mut gfxCtx:
                                                    *mut GraphicsContext) {
    let mut this: *mut EffectBlure = this2;
    let mut vtx: *mut Vtx = 0 as *mut Vtx;
    let mut vtxIter: *mut Vtx = 0 as *mut Vtx;
    let mut elem: *mut EffectBlureElement = 0 as *mut EffectBlureElement;
    let mut vtxCount: s32 = 0;
    let mut i: s32 = 0;
    let mut j: s32 = 0;
    let mut sp74: Vec3s = Vec3s{x: 0, y: 0, z: 0,};
    let mut sp6C: Vec3s = Vec3s{x: 0, y: 0, z: 0,};
    let mut ratio: f32_0 = 0.;
    let mut sp64: Color_RGBA8 = Color_RGBA8{r: 0, g: 0, b: 0, a: 0,};
    let mut sp60: Color_RGBA8 = Color_RGBA8{r: 0, g: 0, b: 0, a: 0,};
    if (*this).numElements as libc::c_int >= 2 as libc::c_int {
        vtxCount = (*this).numElements as libc::c_int * 4 as libc::c_int;
        vtx =
            Graph_Alloc(gfxCtx,
                        (vtxCount as
                             libc::c_uint).wrapping_mul(::std::mem::size_of::<Vtx>()
                                                            as libc::c_ulong)
                            as size_t) as *mut Vtx;
        if vtx.is_null() {
            // "Vertices cannot be secured. Forced termination"
            osSyncPrintf(b"\xe3\x83\x96\xe3\x83\xa9\xe2\x94\x80\xe8\xa1\xa8\xe7\xa4\xba:\xe9\xa0\x82\xe7\x82\xb9\xe7\xa2\xba\xe4\xbf\x9d\xe3\x81\xa7\xe3\x81\x8d\xe3\x81\x9a\xe3\x80\x82\xe5\xbc\xb7\xe5\x88\xb6\xe7\xb5\x82\xe4\xba\x86\n\x00"
                             as *const u8 as *const libc::c_char);
            return
        }
        vtxIter = vtx;
        i = 0 as libc::c_int;
        while i < 4 as libc::c_int {
            (*vtxIter).v = D_8011578C[i as usize];
            vtxIter = vtxIter.offset(1);
            i += 1
        }
        if (*this).numElements as libc::c_int >= 2 as libc::c_int {
            elem = (*this).elements.as_mut_ptr();
            while elem <
                      (*this).elements.as_mut_ptr().offset((*this).numElements
                                                               as libc::c_int
                                                               as
                                                               isize).offset(-(2
                                                                                   as
                                                                                   libc::c_int
                                                                                   as
                                                                                   isize))
                  {
                i = 0 as libc::c_int;
                while i < 4 as libc::c_int {
                    (*vtxIter).v = D_801157CC[i as usize];
                    vtxIter = vtxIter.offset(1);
                    i += 1
                }
                elem = elem.offset(1)
            }
        }
        i = 0 as libc::c_int;
        while i < (*this).numElements as libc::c_int {
            elem =
                &mut *(*this).elements.as_mut_ptr().offset(i as isize) as
                    *mut EffectBlureElement;
            ratio = (*elem).timer as f32_0 / (*this).elemDuration as f32_0;
            EffectBlure_GetComputedValues(this, i, ratio, &mut sp74,
                                          &mut sp6C, &mut sp64, &mut sp60);
            j = i * 4 as libc::c_int - 2 as libc::c_int;
            if j >= 0 as libc::c_int {
                (*vtx.offset(j as isize)).v.ob[0 as libc::c_int as usize] =
                    sp74.x;
                (*vtx.offset(j as isize)).v.ob[1 as libc::c_int as usize] =
                    sp74.y;
                (*vtx.offset(j as isize)).v.ob[2 as libc::c_int as usize] =
                    sp74.z;
                (*vtx.offset(j as isize)).v.cn[0 as libc::c_int as usize] =
                    sp64.r;
                (*vtx.offset(j as isize)).v.cn[1 as libc::c_int as usize] =
                    sp64.g;
                (*vtx.offset(j as isize)).v.cn[2 as libc::c_int as usize] =
                    sp64.b;
                (*vtx.offset(j as isize)).v.cn[3 as libc::c_int as usize] =
                    sp64.a
            }
            j += 1;
            if j >= 0 as libc::c_int {
                (*vtx.offset(j as isize)).v.ob[0 as libc::c_int as usize] =
                    sp6C.x;
                (*vtx.offset(j as isize)).v.ob[1 as libc::c_int as usize] =
                    sp6C.y;
                (*vtx.offset(j as isize)).v.ob[2 as libc::c_int as usize] =
                    sp6C.z;
                (*vtx.offset(j as isize)).v.cn[0 as libc::c_int as usize] =
                    sp60.r;
                (*vtx.offset(j as isize)).v.cn[1 as libc::c_int as usize] =
                    sp60.g;
                (*vtx.offset(j as isize)).v.cn[2 as libc::c_int as usize] =
                    sp60.b;
                (*vtx.offset(j as isize)).v.cn[3 as libc::c_int as usize] =
                    sp60.a
            }
            j += 1;
            if vtxCount >= j {
                (*vtx.offset(j as isize)).v.ob[0 as libc::c_int as usize] =
                    sp74.x;
                (*vtx.offset(j as isize)).v.ob[1 as libc::c_int as usize] =
                    sp74.y;
                (*vtx.offset(j as isize)).v.ob[2 as libc::c_int as usize] =
                    sp74.z;
                (*vtx.offset(j as isize)).v.cn[0 as libc::c_int as usize] =
                    sp64.r;
                (*vtx.offset(j as isize)).v.cn[1 as libc::c_int as usize] =
                    sp64.g;
                (*vtx.offset(j as isize)).v.cn[2 as libc::c_int as usize] =
                    sp64.b;
                (*vtx.offset(j as isize)).v.cn[3 as libc::c_int as usize] =
                    sp64.a
            }
            j += 1;
            if vtxCount >= j {
                (*vtx.offset(j as isize)).v.ob[0 as libc::c_int as usize] =
                    sp6C.x;
                (*vtx.offset(j as isize)).v.ob[1 as libc::c_int as usize] =
                    sp6C.y;
                (*vtx.offset(j as isize)).v.ob[2 as libc::c_int as usize] =
                    sp6C.z;
                (*vtx.offset(j as isize)).v.cn[0 as libc::c_int as usize] =
                    sp60.r;
                (*vtx.offset(j as isize)).v.cn[1 as libc::c_int as usize] =
                    sp60.g;
                (*vtx.offset(j as isize)).v.cn[2 as libc::c_int as usize] =
                    sp60.b;
                (*vtx.offset(j as isize)).v.cn[3 as libc::c_int as usize] =
                    sp60.a
            }
            i += 1
        }
        EffectBlure_DrawSimpleVertices(gfxCtx, this, vtx);
    };
}
#[no_mangle]
pub unsafe extern "C" fn EffectBlure_Draw(mut thisx: *mut libc::c_void,
                                          mut gfxCtx: *mut GraphicsContext) {
    let mut this: *mut EffectBlure = thisx as *mut EffectBlure;
    let mut vtx: *mut Vtx = 0 as *mut Vtx;
    let mut elem: *mut EffectBlureElement = 0 as *mut EffectBlureElement;
    let mut i: s32 = 0;
    let mut j: s32 = 0;
    let mut phi_t2: s32 = 0;
    let mut __gfxCtx: *mut GraphicsContext = 0 as *mut GraphicsContext;
    let mut dispRefs: [*mut Gfx; 4] = [0 as *mut Gfx; 4];
    __gfxCtx = gfxCtx;
    Graph_OpenDisps(dispRefs.as_mut_ptr(), gfxCtx,
                    b"../z_eff_blure.c\x00" as *const u8 as
                        *const libc::c_char, 1596 as libc::c_int);
    let fresh37 = (*__gfxCtx).polyXlu.p;
    (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
    let mut _g: *mut Gfx = fresh37;
    (*_g).words.w0 =
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
    (*_g).words.w1 = &mut gMtxClear as *mut Mtx as libc::c_uint;
    if (*this).numElements as libc::c_int != 0 as libc::c_int {
        if (*this).flags as libc::c_int == 0 as libc::c_int {
            func_800942F0(gfxCtx);
            let fresh38 = (*__gfxCtx).polyXlu.p;
            (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
            let mut _g_0: *mut Gfx = fresh38;
            (*_g_0).words.w0 =
                (0xe7 as libc::c_int as u32_0 &
                     (((0x1 as libc::c_int) << 8 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    24 as libc::c_int;
            (*_g_0).words.w1 = 0 as libc::c_int as libc::c_uint;
            vtx =
                Graph_Alloc(gfxCtx,
                            ::std::mem::size_of::<[Vtx; 32]>() as
                                libc::c_ulong as size_t) as *mut Vtx;
            if vtx.is_null() {
                // "Blure display: Vertex table could not be secured"
                osSyncPrintf(b"\xe3\x83\x96\xe3\x83\xa9\xe2\x94\x80\xe8\xa1\xa8\xe7\xa4\xba:\xe9\xa0\x82\xe7\x82\xb9\xe3\x83\x86\xe3\x83\xbc\xe3\x83\x96\xe3\x83\xab\xe7\xa2\xba\xe4\xbf\x9d\xe3\x81\xa7\xe3\x81\x8d\xe3\x81\x9a\n\x00"
                                 as *const u8 as *const libc::c_char);
            } else {
                j = 0 as libc::c_int;
                i = 0 as libc::c_int;
                while i < (*this).numElements as libc::c_int {
                    elem =
                        &mut *(*this).elements.as_mut_ptr().offset(i as isize)
                            as *mut EffectBlureElement;
                    if (*elem).state == 1 as libc::c_int {
                        let mut ratio: f32_0 =
                            (*elem).timer as f32_0 /
                                (*this).elemDuration as f32_0;
                        match (*this).calcMode {
                            1 => {
                                (*vtx.offset(j as
                                                 isize)).v.ob[0 as libc::c_int
                                                                  as usize] =
                                    func_80027E34((*elem).p1.x, (*elem).p2.x,
                                                  ratio);
                                (*vtx.offset(j as
                                                 isize)).v.ob[1 as libc::c_int
                                                                  as usize] =
                                    func_80027E34((*elem).p1.y, (*elem).p2.y,
                                                  ratio);
                                (*vtx.offset(j as
                                                 isize)).v.ob[2 as libc::c_int
                                                                  as usize] =
                                    func_80027E34((*elem).p1.z, (*elem).p2.z,
                                                  ratio);
                                (*vtx.offset((j + 1 as libc::c_int) as
                                                 isize)).v.ob[0 as libc::c_int
                                                                  as usize] =
                                    (*elem).p2.x;
                                (*vtx.offset((j + 1 as libc::c_int) as
                                                 isize)).v.ob[1 as libc::c_int
                                                                  as usize] =
                                    (*elem).p2.y;
                                (*vtx.offset((j + 1 as libc::c_int) as
                                                 isize)).v.ob[2 as libc::c_int
                                                                  as usize] =
                                    (*elem).p2.z
                            }
                            2 => {
                                (*vtx.offset(j as
                                                 isize)).v.ob[0 as libc::c_int
                                                                  as usize] =
                                    (*elem).p1.x;
                                (*vtx.offset(j as
                                                 isize)).v.ob[1 as libc::c_int
                                                                  as usize] =
                                    (*elem).p1.y;
                                (*vtx.offset(j as
                                                 isize)).v.ob[2 as libc::c_int
                                                                  as usize] =
                                    (*elem).p1.z;
                                (*vtx.offset((j + 1 as libc::c_int) as
                                                 isize)).v.ob[0 as libc::c_int
                                                                  as usize] =
                                    func_80027E34((*elem).p2.x, (*elem).p1.x,
                                                  ratio);
                                (*vtx.offset((j + 1 as libc::c_int) as
                                                 isize)).v.ob[1 as libc::c_int
                                                                  as usize] =
                                    func_80027E34((*elem).p2.y, (*elem).p1.y,
                                                  ratio);
                                (*vtx.offset((j + 1 as libc::c_int) as
                                                 isize)).v.ob[2 as libc::c_int
                                                                  as usize] =
                                    func_80027E34((*elem).p2.z, (*elem).p1.z,
                                                  ratio)
                            }
                            3 => {
                                ratio = ratio * 0.5f32;
                                (*vtx.offset(j as
                                                 isize)).v.ob[0 as libc::c_int
                                                                  as usize] =
                                    func_80027E34((*elem).p1.x, (*elem).p2.x,
                                                  ratio);
                                (*vtx.offset(j as
                                                 isize)).v.ob[1 as libc::c_int
                                                                  as usize] =
                                    func_80027E34((*elem).p1.y, (*elem).p2.y,
                                                  ratio);
                                (*vtx.offset(j as
                                                 isize)).v.ob[2 as libc::c_int
                                                                  as usize] =
                                    func_80027E34((*elem).p1.z, (*elem).p2.z,
                                                  ratio);
                                (*vtx.offset((j + 1 as libc::c_int) as
                                                 isize)).v.ob[0 as libc::c_int
                                                                  as usize] =
                                    func_80027E34((*elem).p2.x, (*elem).p1.x,
                                                  ratio);
                                (*vtx.offset((j + 1 as libc::c_int) as
                                                 isize)).v.ob[1 as libc::c_int
                                                                  as usize] =
                                    func_80027E34((*elem).p2.y, (*elem).p1.y,
                                                  ratio);
                                (*vtx.offset((j + 1 as libc::c_int) as
                                                 isize)).v.ob[2 as libc::c_int
                                                                  as usize] =
                                    func_80027E34((*elem).p2.z, (*elem).p1.z,
                                                  ratio);
                                ratio = ratio + ratio
                            }
                            0 | _ => {
                                (*vtx.offset(j as
                                                 isize)).v.ob[0 as libc::c_int
                                                                  as usize] =
                                    (*elem).p1.x;
                                (*vtx.offset(j as
                                                 isize)).v.ob[1 as libc::c_int
                                                                  as usize] =
                                    (*elem).p1.y;
                                (*vtx.offset(j as
                                                 isize)).v.ob[2 as libc::c_int
                                                                  as usize] =
                                    (*elem).p1.z;
                                (*vtx.offset((j + 1 as libc::c_int) as
                                                 isize)).v.ob[0 as libc::c_int
                                                                  as usize] =
                                    (*elem).p2.x;
                                (*vtx.offset((j + 1 as libc::c_int) as
                                                 isize)).v.ob[1 as libc::c_int
                                                                  as usize] =
                                    (*elem).p2.y;
                                (*vtx.offset((j + 1 as libc::c_int) as
                                                 isize)).v.ob[2 as libc::c_int
                                                                  as usize] =
                                    (*elem).p2.z
                            }
                        }
                        (*vtx.offset(j as isize)).v.flag =
                            0 as libc::c_int as libc::c_ushort;
                        (*vtx.offset(j as
                                         isize)).v.tc[0 as libc::c_int as
                                                          usize] =
                            0 as libc::c_int as libc::c_short;
                        (*vtx.offset(j as
                                         isize)).v.tc[1 as libc::c_int as
                                                          usize] =
                            0 as libc::c_int as libc::c_short;
                        (*vtx.offset(j as
                                         isize)).v.cn[0 as libc::c_int as
                                                          usize] =
                            func_80027E84((*this).p1StartColor.r,
                                          (*this).p1EndColor.r, ratio);
                        (*vtx.offset(j as
                                         isize)).v.cn[1 as libc::c_int as
                                                          usize] =
                            func_80027E84((*this).p1StartColor.g,
                                          (*this).p1EndColor.g, ratio);
                        (*vtx.offset(j as
                                         isize)).v.cn[2 as libc::c_int as
                                                          usize] =
                            func_80027E84((*this).p1StartColor.b,
                                          (*this).p1EndColor.b, ratio);
                        (*vtx.offset(j as
                                         isize)).v.cn[3 as libc::c_int as
                                                          usize] =
                            func_80027E84((*this).p1StartColor.a,
                                          (*this).p1EndColor.a, ratio);
                        j += 1;
                        (*vtx.offset(j as isize)).v.flag =
                            0 as libc::c_int as libc::c_ushort;
                        (*vtx.offset(j as
                                         isize)).v.tc[0 as libc::c_int as
                                                          usize] =
                            0 as libc::c_int as libc::c_short;
                        (*vtx.offset(j as
                                         isize)).v.tc[1 as libc::c_int as
                                                          usize] =
                            0 as libc::c_int as libc::c_short;
                        (*vtx.offset(j as
                                         isize)).v.cn[0 as libc::c_int as
                                                          usize] =
                            func_80027E84((*this).p2StartColor.r,
                                          (*this).p2EndColor.r, ratio);
                        (*vtx.offset(j as
                                         isize)).v.cn[1 as libc::c_int as
                                                          usize] =
                            func_80027E84((*this).p2StartColor.g,
                                          (*this).p2EndColor.g, ratio);
                        (*vtx.offset(j as
                                         isize)).v.cn[2 as libc::c_int as
                                                          usize] =
                            func_80027E84((*this).p2StartColor.b,
                                          (*this).p2EndColor.b, ratio);
                        (*vtx.offset(j as
                                         isize)).v.cn[3 as libc::c_int as
                                                          usize] =
                            func_80027E84((*this).p2StartColor.a,
                                          (*this).p2EndColor.a, ratio);
                        j += 1
                    }
                    i += 1
                }
                j = 0 as libc::c_int;
                let fresh39 = (*__gfxCtx).polyXlu.p;
                (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
                let mut _g_1: *mut Gfx = fresh39;
                (*_g_1).words.w0 =
                    (0x1 as libc::c_int as u32_0 &
                         (((0x1 as libc::c_int) << 8 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        24 as libc::c_int |
                        (32 as libc::c_int as u32_0 &
                             (((0x1 as libc::c_int) << 8 as libc::c_int) -
                                  1 as libc::c_int) as libc::c_uint) <<
                            12 as libc::c_int |
                        ((0 as libc::c_int + 32 as libc::c_int) as u32_0 &
                             (((0x1 as libc::c_int) << 7 as libc::c_int) -
                                  1 as libc::c_int) as libc::c_uint) <<
                            1 as libc::c_int;
                (*_g_1).words.w1 = vtx as libc::c_uint;
                phi_t2 = 0 as libc::c_int;
                i = 0 as libc::c_int;
                while i < (*this).numElements as libc::c_int {
                    elem =
                        &mut *(*this).elements.as_mut_ptr().offset(i as isize)
                            as *mut EffectBlureElement;
                    if (*elem).state == 0 as libc::c_int {
                        phi_t2 = 0 as libc::c_int
                    } else {
                        if phi_t2 == 0 as libc::c_int {
                            phi_t2 = 1 as libc::c_int
                        } else {
                            let fresh40 = (*__gfxCtx).polyXlu.p;
                            (*__gfxCtx).polyXlu.p =
                                (*__gfxCtx).polyXlu.p.offset(1);
                            let mut _g_2: *mut Gfx = fresh40;
                            (*_g_2).words.w0 =
                                (0x7 as libc::c_int as u32_0 &
                                     (((0x1 as libc::c_int) <<
                                           8 as libc::c_int) -
                                          1 as libc::c_int) as libc::c_uint)
                                    << 24 as libc::c_int |
                                    (if 0 as libc::c_int == 0 as libc::c_int {
                                         ((((j - 2 as libc::c_int) *
                                                2 as libc::c_int) as u32_0 &
                                               (((0x1 as libc::c_int) <<
                                                     8 as libc::c_int) -
                                                    1 as libc::c_int) as
                                                   libc::c_uint) <<
                                              16 as libc::c_int |
                                              (((j - 1 as libc::c_int) *
                                                    2 as libc::c_int) as u32_0
                                                   &
                                                   (((0x1 as libc::c_int) <<
                                                         8 as libc::c_int) -
                                                        1 as libc::c_int) as
                                                       libc::c_uint) <<
                                                  8 as libc::c_int) |
                                             (((j + 1 as libc::c_int) *
                                                   2 as libc::c_int) as u32_0
                                                  &
                                                  (((0x1 as libc::c_int) <<
                                                        8 as libc::c_int) -
                                                       1 as libc::c_int) as
                                                      libc::c_uint) <<
                                                 0 as libc::c_int
                                     } else {
                                         (if 0 as libc::c_int ==
                                                 1 as libc::c_int {
                                              ((((j - 1 as libc::c_int) *
                                                     2 as libc::c_int) as
                                                    u32_0 &
                                                    (((0x1 as libc::c_int) <<
                                                          8 as libc::c_int) -
                                                         1 as libc::c_int) as
                                                        libc::c_uint) <<
                                                   16 as libc::c_int |
                                                   (((j + 1 as libc::c_int) *
                                                         2 as libc::c_int) as
                                                        u32_0 &
                                                        (((0x1 as libc::c_int)
                                                              <<
                                                              8 as
                                                                  libc::c_int)
                                                             -
                                                             1 as libc::c_int)
                                                            as libc::c_uint)
                                                       << 8 as libc::c_int) |
                                                  ((j * 2 as libc::c_int) as
                                                       u32_0 &
                                                       (((0x1 as libc::c_int)
                                                             <<
                                                             8 as libc::c_int)
                                                            -
                                                            1 as libc::c_int)
                                                           as libc::c_uint) <<
                                                      0 as libc::c_int
                                          } else {
                                              (if 0 as libc::c_int ==
                                                      2 as libc::c_int {
                                                   ((((j + 1 as libc::c_int) *
                                                          2 as libc::c_int) as
                                                         u32_0 &
                                                         (((0x1 as
                                                                libc::c_int)
                                                               <<
                                                               8 as
                                                                   libc::c_int)
                                                              -
                                                              1 as
                                                                  libc::c_int)
                                                             as libc::c_uint)
                                                        << 16 as libc::c_int |
                                                        ((j *
                                                              2 as
                                                                  libc::c_int)
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
                                                            8 as libc::c_int)
                                                       |
                                                       (((j -
                                                              2 as
                                                                  libc::c_int)
                                                             *
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
                                                   (((j * 2 as libc::c_int) as
                                                         u32_0 &
                                                         (((0x1 as
                                                                libc::c_int)
                                                               <<
                                                               8 as
                                                                   libc::c_int)
                                                              -
                                                              1 as
                                                                  libc::c_int)
                                                             as libc::c_uint)
                                                        << 16 as libc::c_int |
                                                        (((j -
                                                               2 as
                                                                   libc::c_int)
                                                              *
                                                              2 as
                                                                  libc::c_int)
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
                                                            8 as libc::c_int)
                                                       |
                                                       (((j -
                                                              1 as
                                                                  libc::c_int)
                                                             *
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
                                          })
                                     });
                            (*_g_2).words.w1 =
                                if 0 as libc::c_int == 0 as libc::c_int {
                                    ((((j - 2 as libc::c_int) *
                                           2 as libc::c_int) as u32_0 &
                                          (((0x1 as libc::c_int) <<
                                                8 as libc::c_int) -
                                               1 as libc::c_int) as
                                              libc::c_uint) <<
                                         16 as libc::c_int |
                                         (((j + 1 as libc::c_int) *
                                               2 as libc::c_int) as u32_0 &
                                              (((0x1 as libc::c_int) <<
                                                    8 as libc::c_int) -
                                                   1 as libc::c_int) as
                                                  libc::c_uint) <<
                                             8 as libc::c_int) |
                                        ((j * 2 as libc::c_int) as u32_0 &
                                             (((0x1 as libc::c_int) <<
                                                   8 as libc::c_int) -
                                                  1 as libc::c_int) as
                                                 libc::c_uint) <<
                                            0 as libc::c_int
                                } else if 0 as libc::c_int == 1 as libc::c_int
                                 {
                                    ((((j - 1 as libc::c_int) *
                                           2 as libc::c_int) as u32_0 &
                                          (((0x1 as libc::c_int) <<
                                                8 as libc::c_int) -
                                               1 as libc::c_int) as
                                              libc::c_uint) <<
                                         16 as libc::c_int |
                                         ((j * 2 as libc::c_int) as u32_0 &
                                              (((0x1 as libc::c_int) <<
                                                    8 as libc::c_int) -
                                                   1 as libc::c_int) as
                                                  libc::c_uint) <<
                                             8 as libc::c_int) |
                                        (((j - 2 as libc::c_int) *
                                              2 as libc::c_int) as u32_0 &
                                             (((0x1 as libc::c_int) <<
                                                   8 as libc::c_int) -
                                                  1 as libc::c_int) as
                                                 libc::c_uint) <<
                                            0 as libc::c_int
                                } else if 0 as libc::c_int == 2 as libc::c_int
                                 {
                                    ((((j + 1 as libc::c_int) *
                                           2 as libc::c_int) as u32_0 &
                                          (((0x1 as libc::c_int) <<
                                                8 as libc::c_int) -
                                               1 as libc::c_int) as
                                              libc::c_uint) <<
                                         16 as libc::c_int |
                                         (((j - 2 as libc::c_int) *
                                               2 as libc::c_int) as u32_0 &
                                              (((0x1 as libc::c_int) <<
                                                    8 as libc::c_int) -
                                                   1 as libc::c_int) as
                                                  libc::c_uint) <<
                                             8 as libc::c_int) |
                                        (((j - 1 as libc::c_int) *
                                              2 as libc::c_int) as u32_0 &
                                             (((0x1 as libc::c_int) <<
                                                   8 as libc::c_int) -
                                                  1 as libc::c_int) as
                                                 libc::c_uint) <<
                                            0 as libc::c_int
                                } else {
                                    (((j * 2 as libc::c_int) as u32_0 &
                                          (((0x1 as libc::c_int) <<
                                                8 as libc::c_int) -
                                               1 as libc::c_int) as
                                              libc::c_uint) <<
                                         16 as libc::c_int |
                                         (((j - 1 as libc::c_int) *
                                               2 as libc::c_int) as u32_0 &
                                              (((0x1 as libc::c_int) <<
                                                    8 as libc::c_int) -
                                                   1 as libc::c_int) as
                                                  libc::c_uint) <<
                                             8 as libc::c_int) |
                                        (((j + 1 as libc::c_int) *
                                              2 as libc::c_int) as u32_0 &
                                             (((0x1 as libc::c_int) <<
                                                   8 as libc::c_int) -
                                                  1 as libc::c_int) as
                                                 libc::c_uint) <<
                                            0 as libc::c_int
                                };
                            // Necessary to match
                            if (*this).unkFlag as libc::c_int ==
                                   1 as libc::c_int {
                                phi_t2 = 0 as libc::c_int
                            }
                        }
                        j += 2 as libc::c_int
                    }
                    i += 1
                }
            }
        } else if ((*this).drawMode as libc::c_int) < 2 as libc::c_int {
            EffectBlure_DrawSimple(this, gfxCtx);
        } else { EffectBlure_DrawSmooth(this, gfxCtx); }
    }
    Graph_CloseDisps(dispRefs.as_mut_ptr(), gfxCtx,
                     b"../z_eff_blure.c\x00" as *const u8 as
                         *const libc::c_char, 1823 as libc::c_int);
}
