#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case,
         non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(register_tool)]
extern "C" {
    #[no_mangle]
    fn fabsf(f: f32_0) -> f32_0;
    #[no_mangle]
    fn osSyncPrintf(fmt: *const libc::c_char, _: ...);
    #[no_mangle]
    fn Math_CosS(angle: s16) -> f32_0;
    #[no_mangle]
    fn Math_SinS(angle: s16) -> f32_0;
    #[no_mangle]
    fn Graph_Alloc(gfxCtx: *mut GraphicsContext, size: size_t)
     -> *mut libc::c_void;
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
// clang-format off
#[no_mangle]
pub static mut sMtxFClear: MtxF =
    MtxF{mf:
             [[1.0f32, 0.0f32, 0.0f32, 0.0f32],
              [0.0f32, 1.0f32, 0.0f32, 0.0f32],
              [0.0f32, 0.0f32, 1.0f32, 0.0f32],
              [0.0f32, 0.0f32, 0.0f32, 1.0f32]],};
// clang-format on
/* *
 * Multiplies the matrix mf by a 4 components column vector [ src , 1 ] and writes the resulting 4 components to xyzDest
 * and wDest.
 *
 * \f[ \begin{bmatrix} \texttt{xyzDest} \\ \texttt{wDest} \\ \end{bmatrix}
 *      = [\texttt{mf}] \cdot
 *        \begin{bmatrix} \texttt{src} \\ 1 \\ \end{bmatrix}
 * \f]
 */
#[no_mangle]
pub unsafe extern "C" fn SkinMatrix_Vec3fMtxFMultXYZW(mut mf: *mut MtxF,
                                                      mut src: *mut Vec3f,
                                                      mut xyzDest: *mut Vec3f,
                                                      mut wDest: *mut f32_0) {
    (*xyzDest).x =
        (*mf).c2rust_unnamed.xw +
            ((*src).x * (*mf).c2rust_unnamed.xx +
                 (*src).y * (*mf).c2rust_unnamed.xy +
                 (*src).z * (*mf).c2rust_unnamed.xz);
    (*xyzDest).y =
        (*mf).c2rust_unnamed.yw +
            ((*src).x * (*mf).c2rust_unnamed.yx +
                 (*src).y * (*mf).c2rust_unnamed.yy +
                 (*src).z * (*mf).c2rust_unnamed.yz);
    (*xyzDest).z =
        (*mf).c2rust_unnamed.zw +
            ((*src).x * (*mf).c2rust_unnamed.zx +
                 (*src).y * (*mf).c2rust_unnamed.zy +
                 (*src).z * (*mf).c2rust_unnamed.zz);
    *wDest =
        (*mf).c2rust_unnamed.ww +
            ((*src).x * (*mf).c2rust_unnamed.wx +
                 (*src).y * (*mf).c2rust_unnamed.wy +
                 (*src).z * (*mf).c2rust_unnamed.wz);
}
/* *
 * Multiplies the matrix mf by a 4 components column vector [ src , 1 ] and writes the resulting xyz components to dest.
 *
 * \f[ \begin{bmatrix} \texttt{dest} \\ - \\ \end{bmatrix}
 *      = [\texttt{mf}] \cdot
 *        \begin{bmatrix} \texttt{src} \\ 1 \\ \end{bmatrix}
 * \f]
 */
#[no_mangle]
pub unsafe extern "C" fn SkinMatrix_Vec3fMtxFMultXYZ(mut mf: *mut MtxF,
                                                     mut src: *mut Vec3f,
                                                     mut dest: *mut Vec3f) {
    let mut mx: f32_0 = (*mf).c2rust_unnamed.xx;
    let mut my: f32_0 = (*mf).c2rust_unnamed.xy;
    let mut mz: f32_0 = (*mf).c2rust_unnamed.xz;
    let mut mw: f32_0 = (*mf).c2rust_unnamed.xw;
    (*dest).x = mw + ((*src).x * mx + (*src).y * my + (*src).z * mz);
    mx = (*mf).c2rust_unnamed.yx;
    my = (*mf).c2rust_unnamed.yy;
    mz = (*mf).c2rust_unnamed.yz;
    mw = (*mf).c2rust_unnamed.yw;
    (*dest).y = mw + ((*src).x * mx + (*src).y * my + (*src).z * mz);
    mx = (*mf).c2rust_unnamed.zx;
    my = (*mf).c2rust_unnamed.zy;
    mz = (*mf).c2rust_unnamed.zz;
    mw = (*mf).c2rust_unnamed.zw;
    (*dest).z = mw + ((*src).x * mx + (*src).y * my + (*src).z * mz);
}
/* *
 * Matrix multiplication, dest = mfA * mfB.
 * mfB and dest should not be the same matrix.
 */
#[no_mangle]
pub unsafe extern "C" fn SkinMatrix_MtxFMtxFMult(mut mfA: *mut MtxF,
                                                 mut mfB: *mut MtxF,
                                                 mut dest: *mut MtxF) {
    let mut cx: f32_0 = 0.;
    let mut cy: f32_0 = 0.;
    let mut cz: f32_0 = 0.;
    let mut cw: f32_0 = 0.;
    //---ROW1---
    let mut rx: f32_0 = (*mfA).c2rust_unnamed.xx;
    let mut ry: f32_0 = (*mfA).c2rust_unnamed.xy;
    let mut rz: f32_0 = (*mfA).c2rust_unnamed.xz;
    let mut rw: f32_0 = (*mfA).c2rust_unnamed.xw;
    //--------
    cx = (*mfB).c2rust_unnamed.xx;
    cy = (*mfB).c2rust_unnamed.yx;
    cz = (*mfB).c2rust_unnamed.zx;
    cw = (*mfB).c2rust_unnamed.wx;
    (*dest).c2rust_unnamed.xx = rx * cx + ry * cy + rz * cz + rw * cw;
    cx = (*mfB).c2rust_unnamed.xy;
    cy = (*mfB).c2rust_unnamed.yy;
    cz = (*mfB).c2rust_unnamed.zy;
    cw = (*mfB).c2rust_unnamed.wy;
    (*dest).c2rust_unnamed.xy = rx * cx + ry * cy + rz * cz + rw * cw;
    cx = (*mfB).c2rust_unnamed.xz;
    cy = (*mfB).c2rust_unnamed.yz;
    cz = (*mfB).c2rust_unnamed.zz;
    cw = (*mfB).c2rust_unnamed.wz;
    (*dest).c2rust_unnamed.xz = rx * cx + ry * cy + rz * cz + rw * cw;
    cx = (*mfB).c2rust_unnamed.xw;
    cy = (*mfB).c2rust_unnamed.yw;
    cz = (*mfB).c2rust_unnamed.zw;
    cw = (*mfB).c2rust_unnamed.ww;
    (*dest).c2rust_unnamed.xw = rx * cx + ry * cy + rz * cz + rw * cw;
    //---ROW2---
    rx = (*mfA).c2rust_unnamed.yx;
    ry = (*mfA).c2rust_unnamed.yy;
    rz = (*mfA).c2rust_unnamed.yz;
    rw = (*mfA).c2rust_unnamed.yw;
    //--------
    cx = (*mfB).c2rust_unnamed.xx;
    cy = (*mfB).c2rust_unnamed.yx;
    cz = (*mfB).c2rust_unnamed.zx;
    cw = (*mfB).c2rust_unnamed.wx;
    (*dest).c2rust_unnamed.yx = rx * cx + ry * cy + rz * cz + rw * cw;
    cx = (*mfB).c2rust_unnamed.xy;
    cy = (*mfB).c2rust_unnamed.yy;
    cz = (*mfB).c2rust_unnamed.zy;
    cw = (*mfB).c2rust_unnamed.wy;
    (*dest).c2rust_unnamed.yy = rx * cx + ry * cy + rz * cz + rw * cw;
    cx = (*mfB).c2rust_unnamed.xz;
    cy = (*mfB).c2rust_unnamed.yz;
    cz = (*mfB).c2rust_unnamed.zz;
    cw = (*mfB).c2rust_unnamed.wz;
    (*dest).c2rust_unnamed.yz = rx * cx + ry * cy + rz * cz + rw * cw;
    cx = (*mfB).c2rust_unnamed.xw;
    cy = (*mfB).c2rust_unnamed.yw;
    cz = (*mfB).c2rust_unnamed.zw;
    cw = (*mfB).c2rust_unnamed.ww;
    (*dest).c2rust_unnamed.yw = rx * cx + ry * cy + rz * cz + rw * cw;
    //---ROW3---
    rx = (*mfA).c2rust_unnamed.zx;
    ry = (*mfA).c2rust_unnamed.zy;
    rz = (*mfA).c2rust_unnamed.zz;
    rw = (*mfA).c2rust_unnamed.zw;
    //--------
    cx = (*mfB).c2rust_unnamed.xx;
    cy = (*mfB).c2rust_unnamed.yx;
    cz = (*mfB).c2rust_unnamed.zx;
    cw = (*mfB).c2rust_unnamed.wx;
    (*dest).c2rust_unnamed.zx = rx * cx + ry * cy + rz * cz + rw * cw;
    cx = (*mfB).c2rust_unnamed.xy;
    cy = (*mfB).c2rust_unnamed.yy;
    cz = (*mfB).c2rust_unnamed.zy;
    cw = (*mfB).c2rust_unnamed.wy;
    (*dest).c2rust_unnamed.zy = rx * cx + ry * cy + rz * cz + rw * cw;
    cx = (*mfB).c2rust_unnamed.xz;
    cy = (*mfB).c2rust_unnamed.yz;
    cz = (*mfB).c2rust_unnamed.zz;
    cw = (*mfB).c2rust_unnamed.wz;
    (*dest).c2rust_unnamed.zz = rx * cx + ry * cy + rz * cz + rw * cw;
    cx = (*mfB).c2rust_unnamed.xw;
    cy = (*mfB).c2rust_unnamed.yw;
    cz = (*mfB).c2rust_unnamed.zw;
    cw = (*mfB).c2rust_unnamed.ww;
    (*dest).c2rust_unnamed.zw = rx * cx + ry * cy + rz * cz + rw * cw;
    //---ROW4---
    rx = (*mfA).c2rust_unnamed.wx;
    ry = (*mfA).c2rust_unnamed.wy;
    rz = (*mfA).c2rust_unnamed.wz;
    rw = (*mfA).c2rust_unnamed.ww;
    //--------
    cx = (*mfB).c2rust_unnamed.xx;
    cy = (*mfB).c2rust_unnamed.yx;
    cz = (*mfB).c2rust_unnamed.zx;
    cw = (*mfB).c2rust_unnamed.wx;
    (*dest).c2rust_unnamed.wx = rx * cx + ry * cy + rz * cz + rw * cw;
    cx = (*mfB).c2rust_unnamed.xy;
    cy = (*mfB).c2rust_unnamed.yy;
    cz = (*mfB).c2rust_unnamed.zy;
    cw = (*mfB).c2rust_unnamed.wy;
    (*dest).c2rust_unnamed.wy = rx * cx + ry * cy + rz * cz + rw * cw;
    cx = (*mfB).c2rust_unnamed.xz;
    cy = (*mfB).c2rust_unnamed.yz;
    cz = (*mfB).c2rust_unnamed.zz;
    cw = (*mfB).c2rust_unnamed.wz;
    (*dest).c2rust_unnamed.wz = rx * cx + ry * cy + rz * cz + rw * cw;
    cx = (*mfB).c2rust_unnamed.xw;
    cy = (*mfB).c2rust_unnamed.yw;
    cz = (*mfB).c2rust_unnamed.zw;
    cw = (*mfB).c2rust_unnamed.ww;
    (*dest).c2rust_unnamed.ww = rx * cx + ry * cy + rz * cz + rw * cw;
}
/* *
 * "Clear" in this file means the identity matrix.
 */
#[no_mangle]
pub unsafe extern "C" fn SkinMatrix_GetClear(mut mfp: *mut *mut MtxF) {
    *mfp = &mut sMtxFClear;
}
#[no_mangle]
pub unsafe extern "C" fn SkinMatrix_Clear(mut mf: *mut MtxF) {
    (*mf).c2rust_unnamed.xx = 1.0f32;
    (*mf).c2rust_unnamed.yy = 1.0f32;
    (*mf).c2rust_unnamed.zz = 1.0f32;
    (*mf).c2rust_unnamed.ww = 1.0f32;
    (*mf).c2rust_unnamed.yx = 0.0f32;
    (*mf).c2rust_unnamed.zx = 0.0f32;
    (*mf).c2rust_unnamed.wx = 0.0f32;
    (*mf).c2rust_unnamed.xy = 0.0f32;
    (*mf).c2rust_unnamed.zy = 0.0f32;
    (*mf).c2rust_unnamed.wy = 0.0f32;
    (*mf).c2rust_unnamed.xz = 0.0f32;
    (*mf).c2rust_unnamed.yz = 0.0f32;
    (*mf).c2rust_unnamed.wz = 0.0f32;
    (*mf).c2rust_unnamed.xw = 0.0f32;
    (*mf).c2rust_unnamed.yw = 0.0f32;
    (*mf).c2rust_unnamed.zw = 0.0f32;
}
#[no_mangle]
pub unsafe extern "C" fn SkinMatrix_MtxFCopy(mut src: *mut MtxF,
                                             mut dest: *mut MtxF) {
    (*dest).c2rust_unnamed.xx = (*src).c2rust_unnamed.xx;
    (*dest).c2rust_unnamed.yx = (*src).c2rust_unnamed.yx;
    (*dest).c2rust_unnamed.zx = (*src).c2rust_unnamed.zx;
    (*dest).c2rust_unnamed.wx = (*src).c2rust_unnamed.wx;
    (*dest).c2rust_unnamed.xy = (*src).c2rust_unnamed.xy;
    (*dest).c2rust_unnamed.yy = (*src).c2rust_unnamed.yy;
    (*dest).c2rust_unnamed.zy = (*src).c2rust_unnamed.zy;
    (*dest).c2rust_unnamed.wy = (*src).c2rust_unnamed.wy;
    (*dest).c2rust_unnamed.xz = (*src).c2rust_unnamed.xz;
    (*dest).c2rust_unnamed.yz = (*src).c2rust_unnamed.yz;
    (*dest).c2rust_unnamed.zz = (*src).c2rust_unnamed.zz;
    (*dest).c2rust_unnamed.wz = (*src).c2rust_unnamed.wz;
    (*dest).c2rust_unnamed.xw = (*src).c2rust_unnamed.xw;
    (*dest).c2rust_unnamed.yw = (*src).c2rust_unnamed.yw;
    (*dest).c2rust_unnamed.zw = (*src).c2rust_unnamed.zw;
    (*dest).c2rust_unnamed.ww = (*src).c2rust_unnamed.ww;
}
/* *
 * Inverts a matrix using the Gauss-Jordan method.
 * returns 0 if successfully inverted
 * returns 2 if matrix non-invertible (0 determinant)
 */
#[no_mangle]
pub unsafe extern "C" fn SkinMatrix_Invert(mut src: *mut MtxF,
                                           mut dest: *mut MtxF) -> s32 {
    let mut mfCopy: MtxF = MtxF{mf: [[0.; 4]; 4],};
    let mut i: s32 = 0;
    let mut pad: s32 = 0;
    let mut temp2: f32_0 = 0.;
    let mut temp1: f32_0 = 0.;
    let mut thisCol: s32 = 0;
    let mut thisRow: s32 = 0;
    SkinMatrix_MtxFCopy(src, &mut mfCopy);
    SkinMatrix_Clear(dest);
    thisCol = 0 as libc::c_int;
    while thisCol < 4 as libc::c_int {
        thisRow = thisCol;
        while thisRow < 4 as libc::c_int &&
                  fabsf(mfCopy.mf[thisCol as usize][thisRow as usize]) <
                      0.0005f32 {
            thisRow += 1
        }
        if thisRow == 4 as libc::c_int {
            // Reaching row = 4 means the column is either all 0 or a duplicate column.
            // Therefore src is a singular matrix (0 determinant).
            osSyncPrintf(b"\x1b[43;30m\x00" as *const u8 as
                             *const libc::c_char);
            osSyncPrintf(b"Skin_Matrix_InverseMatrix():\xe9\x80\x86\xe8\xa1\x8c\xe5\x88\x97\xe3\x81\xa4\xe3\x81\x8f\xe3\x82\x8c\xe3\x81\xbe\xe3\x81\x9b\xe3\x82\x93\n\x00"
                             as *const u8 as *const libc::c_char);
            osSyncPrintf(b"\x1b[m\x00" as *const u8 as *const libc::c_char);
            return 2 as libc::c_int
        }
        if thisRow != thisCol {
            // Diagonal element mf[thisCol][thisCol] is zero.
            // Swap the rows thisCol and thisRow.
            i = 0 as libc::c_int;
            while i < 4 as libc::c_int {
                temp1 = mfCopy.mf[i as usize][thisRow as usize];
                mfCopy.mf[i as usize][thisRow as usize] =
                    mfCopy.mf[i as usize][thisCol as usize];
                mfCopy.mf[i as usize][thisCol as usize] = temp1;
                temp2 = (*dest).mf[i as usize][thisRow as usize];
                (*dest).mf[i as usize][thisRow as usize] =
                    (*dest).mf[i as usize][thisCol as usize];
                (*dest).mf[i as usize][thisCol as usize] = temp2;
                i += 1
            }
        }
        // Scale this whole row such that the diagonal element is 1.
        temp1 = mfCopy.mf[thisCol as usize][thisCol as usize];
        i = 0 as libc::c_int;
        while i < 4 as libc::c_int {
            mfCopy.mf[i as usize][thisCol as usize] /= temp1;
            (*dest).mf[i as usize][thisCol as usize] /= temp1;
            i += 1
        }
        thisRow = 0 as libc::c_int;
        while thisRow < 4 as libc::c_int {
            if thisRow != thisCol {
                temp1 = mfCopy.mf[thisCol as usize][thisRow as usize];
                i = 0 as libc::c_int;
                while i < 4 as libc::c_int {
                    mfCopy.mf[i as usize][thisRow as usize] -=
                        mfCopy.mf[i as usize][thisCol as usize] * temp1;
                    (*dest).mf[i as usize][thisRow as usize] -=
                        (*dest).mf[i as usize][thisCol as usize] * temp1;
                    i += 1
                }
            }
            thisRow += 1
        }
        thisCol += 1
    }
    return 0 as libc::c_int;
}
/* *
 * Produces a matrix which scales x,y,z components of vectors or x,y,z rows of matrices (when applied on LHS)
 */
#[no_mangle]
pub unsafe extern "C" fn SkinMatrix_SetScale(mut mf: *mut MtxF, mut x: f32_0,
                                             mut y: f32_0, mut z: f32_0) {
    (*mf).c2rust_unnamed.yx = 0.0f32;
    (*mf).c2rust_unnamed.zx = 0.0f32;
    (*mf).c2rust_unnamed.wx = 0.0f32;
    (*mf).c2rust_unnamed.xy = 0.0f32;
    (*mf).c2rust_unnamed.zy = 0.0f32;
    (*mf).c2rust_unnamed.wy = 0.0f32;
    (*mf).c2rust_unnamed.xz = 0.0f32;
    (*mf).c2rust_unnamed.yz = 0.0f32;
    (*mf).c2rust_unnamed.wz = 0.0f32;
    (*mf).c2rust_unnamed.xw = 0.0f32;
    (*mf).c2rust_unnamed.yw = 0.0f32;
    (*mf).c2rust_unnamed.zw = 0.0f32;
    (*mf).c2rust_unnamed.ww = 1.0f32;
    (*mf).c2rust_unnamed.xx = x;
    (*mf).c2rust_unnamed.yy = y;
    (*mf).c2rust_unnamed.zz = z;
}
/* *
 * Produces a rotation matrix using ZYX Tait-Bryan angles.
 */
#[no_mangle]
pub unsafe extern "C" fn SkinMatrix_SetRotateZYX(mut mf: *mut MtxF,
                                                 mut x: s16, mut y: s16,
                                                 mut z: s16) {
    let mut cos: f32_0 = 0.; // required to match
    let mut sinZ: f32_0 = Math_SinS(z);
    let mut cosZ: f32_0 = Math_CosS(z);
    let mut xy: f32_0 = 0.;
    let mut sin: f32_0 = 0.;
    let mut xz: f32_0 = 0.;
    let mut yy: f32_0 = 0.;
    let mut yz: f32_0 = 0.;
    (*mf).c2rust_unnamed.yy = cosZ;
    (*mf).c2rust_unnamed.xy = -sinZ;
    (*mf).c2rust_unnamed.wz = 0 as libc::c_int as libc::c_float;
    (*mf).c2rust_unnamed.wy = (*mf).c2rust_unnamed.wz;
    (*mf).c2rust_unnamed.wx = (*mf).c2rust_unnamed.wy;
    (*mf).c2rust_unnamed.zw = 0 as libc::c_int as libc::c_float;
    (*mf).c2rust_unnamed.yw = (*mf).c2rust_unnamed.zw;
    (*mf).c2rust_unnamed.xw = (*mf).c2rust_unnamed.yw;
    (*mf).c2rust_unnamed.ww = 1 as libc::c_int as libc::c_float;
    if y as libc::c_int != 0 as libc::c_int {
        sin = Math_SinS(y);
        cos = Math_CosS(y);
        (*mf).c2rust_unnamed.xx = cosZ * cos;
        (*mf).c2rust_unnamed.xz = cosZ * sin;
        (*mf).c2rust_unnamed.yx = sinZ * cos;
        (*mf).c2rust_unnamed.yz = sinZ * sin;
        (*mf).c2rust_unnamed.zx = -sin;
        (*mf).c2rust_unnamed.zz = cos
    } else {
        (*mf).c2rust_unnamed.xx = cosZ;
        xz = sinZ;
        (*mf).c2rust_unnamed.yx = sinZ;
        (*mf).c2rust_unnamed.yz = 0 as libc::c_int as libc::c_float;
        (*mf).c2rust_unnamed.xz = (*mf).c2rust_unnamed.yz;
        (*mf).c2rust_unnamed.zx = (*mf).c2rust_unnamed.xz;
        (*mf).c2rust_unnamed.zz = 1 as libc::c_int as libc::c_float
    }
    if x as libc::c_int != 0 as libc::c_int {
        sin = Math_SinS(x);
        cos = Math_CosS(x);
        xy = (*mf).c2rust_unnamed.xy;
        xz = (*mf).c2rust_unnamed.xz;
        (*mf).c2rust_unnamed.xy = xy * cos + xz * sin;
        (*mf).c2rust_unnamed.xz = xz * cos - xy * sin;
        yz = (*mf).c2rust_unnamed.yz;
        yy = (*mf).c2rust_unnamed.yy;
        (*mf).c2rust_unnamed.yy = yy * cos + yz * sin;
        (*mf).c2rust_unnamed.yz = yz * cos - yy * sin;
        (cos) != 0.;
        (*mf).c2rust_unnamed.zy = (*mf).c2rust_unnamed.zz * sin;
        (*mf).c2rust_unnamed.zz = (*mf).c2rust_unnamed.zz * cos
    } else { (*mf).c2rust_unnamed.zy = 0 as libc::c_int as libc::c_float };
}
/* *
 * Produces a rotation matrix using YXZ Tait-Bryan angles.
 */
#[no_mangle]
pub unsafe extern "C" fn SkinMatrix_SetRotateYXZ(mut mf: *mut MtxF,
                                                 mut x: s16, mut y: s16,
                                                 mut z: s16) {
    let mut cos: f32_0 = 0.; // required to match
    let mut sinY: f32_0 = Math_SinS(y);
    let mut cosY: f32_0 = Math_CosS(y);
    let mut zx: f32_0 = 0.;
    let mut sin: f32_0 = 0.;
    let mut zy: f32_0 = 0.;
    let mut xx: f32_0 = 0.;
    let mut xy: f32_0 = 0.;
    (*mf).c2rust_unnamed.xx = cosY;
    (*mf).c2rust_unnamed.zx = -sinY;
    (*mf).c2rust_unnamed.wz = 0 as libc::c_int as libc::c_float;
    (*mf).c2rust_unnamed.wy = 0 as libc::c_int as libc::c_float;
    (*mf).c2rust_unnamed.wx = 0 as libc::c_int as libc::c_float;
    (*mf).c2rust_unnamed.zw = 0 as libc::c_int as libc::c_float;
    (*mf).c2rust_unnamed.yw = 0 as libc::c_int as libc::c_float;
    (*mf).c2rust_unnamed.xw = 0 as libc::c_int as libc::c_float;
    (*mf).c2rust_unnamed.ww = 1 as libc::c_int as libc::c_float;
    if x as libc::c_int != 0 as libc::c_int {
        sin = Math_SinS(x);
        cos = Math_CosS(x);
        (*mf).c2rust_unnamed.zz = cosY * cos;
        (*mf).c2rust_unnamed.zy = cosY * sin;
        (*mf).c2rust_unnamed.xz = sinY * cos;
        (*mf).c2rust_unnamed.xy = sinY * sin;
        (*mf).c2rust_unnamed.yz = -sin;
        (*mf).c2rust_unnamed.yy = cos
    } else {
        (*mf).c2rust_unnamed.zz = cosY;
        xy = sinY;
        (*mf).c2rust_unnamed.xz = sinY;
        (*mf).c2rust_unnamed.yz = 0 as libc::c_int as libc::c_float;
        (*mf).c2rust_unnamed.zy = (*mf).c2rust_unnamed.yz;
        (*mf).c2rust_unnamed.xy = (*mf).c2rust_unnamed.zy;
        (*mf).c2rust_unnamed.yy = 1 as libc::c_int as libc::c_float
    }
    if z as libc::c_int != 0 as libc::c_int {
        sin = Math_SinS(z);
        cos = Math_CosS(z);
        xx = (*mf).c2rust_unnamed.xx;
        xy = (*mf).c2rust_unnamed.xy;
        (*mf).c2rust_unnamed.xx = xx * cos + xy * sin;
        (*mf).c2rust_unnamed.xy = xy * cos - xx * sin;
        zy = (*mf).c2rust_unnamed.zy;
        zx = (*mf).c2rust_unnamed.zx;
        (*mf).c2rust_unnamed.zx = zx * cos + zy * sin;
        (*mf).c2rust_unnamed.zy = zy * cos - zx * sin;
        (cos) != 0.;
        (*mf).c2rust_unnamed.yx = (*mf).c2rust_unnamed.yy * sin;
        (*mf).c2rust_unnamed.yy = (*mf).c2rust_unnamed.yy * cos
    } else { (*mf).c2rust_unnamed.yx = 0 as libc::c_int as libc::c_float };
}
/* *
 * Produces a matrix which translates a vector by amounts in the x, y and z directions
 */
#[no_mangle]
pub unsafe extern "C" fn SkinMatrix_SetTranslate(mut mf: *mut MtxF,
                                                 mut x: f32_0, mut y: f32_0,
                                                 mut z: f32_0) {
    (*mf).c2rust_unnamed.yx = 0.0f32;
    (*mf).c2rust_unnamed.zx = 0.0f32;
    (*mf).c2rust_unnamed.wx = 0.0f32;
    (*mf).c2rust_unnamed.xy = 0.0f32;
    (*mf).c2rust_unnamed.zy = 0.0f32;
    (*mf).c2rust_unnamed.wy = 0.0f32;
    (*mf).c2rust_unnamed.xz = 0.0f32;
    (*mf).c2rust_unnamed.yz = 0.0f32;
    (*mf).c2rust_unnamed.wz = 0.0f32;
    (*mf).c2rust_unnamed.xx = 1.0f32;
    (*mf).c2rust_unnamed.yy = 1.0f32;
    (*mf).c2rust_unnamed.zz = 1.0f32;
    (*mf).c2rust_unnamed.ww = 1.0f32;
    (*mf).c2rust_unnamed.xw = x;
    (*mf).c2rust_unnamed.yw = y;
    (*mf).c2rust_unnamed.zw = z;
}
/* *
 * Produces a matrix which scales, then rotates (using ZYX Tait-Bryan angles), then translates.
 */
#[no_mangle]
pub unsafe extern "C" fn SkinMatrix_SetTranslateRotateZYXScale(mut dest:
                                                                   *mut MtxF,
                                                               mut scaleX:
                                                                   f32_0,
                                                               mut scaleY:
                                                                   f32_0,
                                                               mut scaleZ:
                                                                   f32_0,
                                                               mut rotX: s16,
                                                               mut rotY: s16,
                                                               mut rotZ: s16,
                                                               mut translateX:
                                                                   f32_0,
                                                               mut translateY:
                                                                   f32_0,
                                                               mut translateZ:
                                                                   f32_0) {
    let mut mft1: MtxF = MtxF{mf: [[0.; 4]; 4],};
    let mut mft2: MtxF = MtxF{mf: [[0.; 4]; 4],};
    SkinMatrix_SetTranslate(dest, translateX, translateY, translateZ);
    SkinMatrix_SetRotateZYX(&mut mft1, rotX, rotY, rotZ);
    SkinMatrix_MtxFMtxFMult(dest, &mut mft1, &mut mft2);
    SkinMatrix_SetScale(&mut mft1, scaleX, scaleY, scaleZ);
    SkinMatrix_MtxFMtxFMult(&mut mft2, &mut mft1, dest);
}
/* *
 * Produces a matrix which scales, then rotates (using YXZ Tait-Bryan angles), then translates.
 */
#[no_mangle]
pub unsafe extern "C" fn SkinMatrix_SetTranslateRotateYXZScale(mut dest:
                                                                   *mut MtxF,
                                                               mut scaleX:
                                                                   f32_0,
                                                               mut scaleY:
                                                                   f32_0,
                                                               mut scaleZ:
                                                                   f32_0,
                                                               mut rotX: s16,
                                                               mut rotY: s16,
                                                               mut rotZ: s16,
                                                               mut translateX:
                                                                   f32_0,
                                                               mut translateY:
                                                                   f32_0,
                                                               mut translateZ:
                                                                   f32_0) {
    let mut mft1: MtxF = MtxF{mf: [[0.; 4]; 4],};
    let mut mft2: MtxF = MtxF{mf: [[0.; 4]; 4],};
    SkinMatrix_SetTranslate(dest, translateX, translateY, translateZ);
    SkinMatrix_SetRotateYXZ(&mut mft1, rotX, rotY, rotZ);
    SkinMatrix_MtxFMtxFMult(dest, &mut mft1, &mut mft2);
    SkinMatrix_SetScale(&mut mft1, scaleX, scaleY, scaleZ);
    SkinMatrix_MtxFMtxFMult(&mut mft2, &mut mft1, dest);
}
/* *
 * Produces a matrix which rotates (using ZYX Tait-Bryan angles), then translates.
 */
#[no_mangle]
pub unsafe extern "C" fn SkinMatrix_SetTranslateRotateZYX(mut dest: *mut MtxF,
                                                          mut rotX: s16,
                                                          mut rotY: s16,
                                                          mut rotZ: s16,
                                                          mut translateX:
                                                              f32_0,
                                                          mut translateY:
                                                              f32_0,
                                                          mut translateZ:
                                                              f32_0) {
    let mut rotation: MtxF = MtxF{mf: [[0.; 4]; 4],};
    let mut translation: MtxF = MtxF{mf: [[0.; 4]; 4],};
    SkinMatrix_SetTranslate(&mut translation, translateX, translateY,
                            translateZ);
    SkinMatrix_SetRotateZYX(&mut rotation, rotX, rotY, rotZ);
    SkinMatrix_MtxFMtxFMult(&mut translation, &mut rotation, dest);
}
#[no_mangle]
pub unsafe extern "C" fn SkinMatrix_Vec3fToVec3s(mut src: *mut Vec3f,
                                                 mut dest: *mut Vec3s) {
    (*dest).x = (*src).x as s16;
    (*dest).y = (*src).y as s16;
    (*dest).z = (*src).z as s16;
}
#[no_mangle]
pub unsafe extern "C" fn SkinMatrix_Vec3sToVec3f(mut src: *mut Vec3s,
                                                 mut dest: *mut Vec3f) {
    (*dest).x = (*src).x as f32_0;
    (*dest).y = (*src).y as f32_0;
    (*dest).z = (*src).z as f32_0;
}
#[no_mangle]
pub unsafe extern "C" fn SkinMatrix_MtxFToMtx(mut src: *mut MtxF,
                                              mut dest: *mut Mtx) {
    let mut temp: s32 = 0;
    let mut m1: *mut u16_0 =
        &mut *(*(*dest).m.as_mut_ptr().offset(0 as libc::c_int as
                                                  isize)).as_mut_ptr().offset(0
                                                                                  as
                                                                                  libc::c_int
                                                                                  as
                                                                                  isize)
            as *mut libc::c_long as *mut u16_0;
    let mut m2: *mut u16_0 =
        &mut *(*(*dest).m.as_mut_ptr().offset(2 as libc::c_int as
                                                  isize)).as_mut_ptr().offset(0
                                                                                  as
                                                                                  libc::c_int
                                                                                  as
                                                                                  isize)
            as *mut libc::c_long as *mut u16_0;
    temp =
        ((*src).c2rust_unnamed.xx * 0x10000 as libc::c_int as libc::c_float)
            as s32;
    *m1.offset(0 as libc::c_int as isize) =
        (temp >> 0x10 as libc::c_int) as u16_0;
    *m1.offset((16 as libc::c_int + 0 as libc::c_int) as isize) =
        (temp & 0xffff as libc::c_int) as u16_0;
    temp =
        ((*src).c2rust_unnamed.yx * 0x10000 as libc::c_int as libc::c_float)
            as s32;
    *m1.offset(1 as libc::c_int as isize) =
        (temp >> 0x10 as libc::c_int) as u16_0;
    *m1.offset((16 as libc::c_int + 1 as libc::c_int) as isize) =
        (temp & 0xffff as libc::c_int) as u16_0;
    temp =
        ((*src).c2rust_unnamed.zx * 0x10000 as libc::c_int as libc::c_float)
            as s32;
    *m1.offset(2 as libc::c_int as isize) =
        (temp >> 0x10 as libc::c_int) as u16_0;
    *m1.offset((16 as libc::c_int + 2 as libc::c_int) as isize) =
        (temp & 0xffff as libc::c_int) as u16_0;
    temp =
        ((*src).c2rust_unnamed.wx * 0x10000 as libc::c_int as libc::c_float)
            as s32;
    *m1.offset(3 as libc::c_int as isize) =
        (temp >> 0x10 as libc::c_int) as u16_0;
    *m1.offset((16 as libc::c_int + 3 as libc::c_int) as isize) =
        (temp & 0xffff as libc::c_int) as u16_0;
    temp =
        ((*src).c2rust_unnamed.xy * 0x10000 as libc::c_int as libc::c_float)
            as s32;
    *m1.offset(4 as libc::c_int as isize) =
        (temp >> 0x10 as libc::c_int) as u16_0;
    *m1.offset((16 as libc::c_int + 4 as libc::c_int) as isize) =
        (temp & 0xffff as libc::c_int) as u16_0;
    temp =
        ((*src).c2rust_unnamed.yy * 0x10000 as libc::c_int as libc::c_float)
            as s32;
    *m1.offset(5 as libc::c_int as isize) =
        (temp >> 0x10 as libc::c_int) as u16_0;
    *m1.offset((16 as libc::c_int + 5 as libc::c_int) as isize) =
        (temp & 0xffff as libc::c_int) as u16_0;
    temp =
        ((*src).c2rust_unnamed.zy * 0x10000 as libc::c_int as libc::c_float)
            as s32;
    *m1.offset(6 as libc::c_int as isize) =
        (temp >> 0x10 as libc::c_int) as u16_0;
    *m1.offset((16 as libc::c_int + 6 as libc::c_int) as isize) =
        (temp & 0xffff as libc::c_int) as u16_0;
    temp =
        ((*src).c2rust_unnamed.wy * 0x10000 as libc::c_int as libc::c_float)
            as s32;
    *m1.offset(7 as libc::c_int as isize) =
        (temp >> 0x10 as libc::c_int) as u16_0;
    *m1.offset((16 as libc::c_int + 7 as libc::c_int) as isize) =
        (temp & 0xffff as libc::c_int) as u16_0;
    temp =
        ((*src).c2rust_unnamed.xz * 0x10000 as libc::c_int as libc::c_float)
            as s32;
    *m1.offset(8 as libc::c_int as isize) =
        (temp >> 0x10 as libc::c_int) as u16_0;
    *m1.offset((16 as libc::c_int + 8 as libc::c_int) as isize) =
        (temp & 0xffff as libc::c_int) as u16_0;
    temp =
        ((*src).c2rust_unnamed.yz * 0x10000 as libc::c_int as libc::c_float)
            as s32;
    *m1.offset(9 as libc::c_int as isize) =
        (temp >> 0x10 as libc::c_int) as u16_0;
    *m2.offset(9 as libc::c_int as isize) =
        (temp & 0xffff as libc::c_int) as u16_0;
    temp =
        ((*src).c2rust_unnamed.zz * 0x10000 as libc::c_int as libc::c_float)
            as s32;
    *m1.offset(10 as libc::c_int as isize) =
        (temp >> 0x10 as libc::c_int) as u16_0;
    *m2.offset(10 as libc::c_int as isize) =
        (temp & 0xffff as libc::c_int) as u16_0;
    temp =
        ((*src).c2rust_unnamed.wz * 0x10000 as libc::c_int as libc::c_float)
            as s32;
    *m1.offset(11 as libc::c_int as isize) =
        (temp >> 0x10 as libc::c_int) as u16_0;
    *m2.offset(11 as libc::c_int as isize) =
        (temp & 0xffff as libc::c_int) as u16_0;
    temp =
        ((*src).c2rust_unnamed.xw * 0x10000 as libc::c_int as libc::c_float)
            as s32;
    *m1.offset(12 as libc::c_int as isize) =
        (temp >> 0x10 as libc::c_int) as u16_0;
    *m2.offset(12 as libc::c_int as isize) =
        (temp & 0xffff as libc::c_int) as u16_0;
    temp =
        ((*src).c2rust_unnamed.yw * 0x10000 as libc::c_int as libc::c_float)
            as s32;
    *m1.offset(13 as libc::c_int as isize) =
        (temp >> 0x10 as libc::c_int) as u16_0;
    *m2.offset(13 as libc::c_int as isize) =
        (temp & 0xffff as libc::c_int) as u16_0;
    temp =
        ((*src).c2rust_unnamed.zw * 0x10000 as libc::c_int as libc::c_float)
            as s32;
    *m1.offset(14 as libc::c_int as isize) =
        (temp >> 0x10 as libc::c_int) as u16_0;
    *m2.offset(14 as libc::c_int as isize) =
        (temp & 0xffff as libc::c_int) as u16_0;
    temp =
        ((*src).c2rust_unnamed.ww * 0x10000 as libc::c_int as libc::c_float)
            as s32;
    *m1.offset(15 as libc::c_int as isize) =
        (temp >> 0x10 as libc::c_int) as u16_0;
    *m2.offset(15 as libc::c_int as isize) =
        (temp & 0xffff as libc::c_int) as u16_0;
}
#[no_mangle]
pub unsafe extern "C" fn SkinMatrix_MtxFToNewMtx(mut gfxCtx:
                                                     *mut GraphicsContext,
                                                 mut src: *mut MtxF)
 -> *mut Mtx {
    let mut mtx: *mut Mtx =
        Graph_Alloc(gfxCtx,
                    ::std::mem::size_of::<Mtx>() as libc::c_ulong as size_t)
            as *mut Mtx;
    if mtx.is_null() {
        osSyncPrintf(b"Skin_Matrix_to_Mtx_new() \xe7\xa2\xba\xe4\xbf\x9d\xe5\xa4\xb1\xe6\x95\x97:NULL\xe3\x82\x92\xe8\xbf\x94\xe3\x81\x97\xe3\x81\xa6\xe7\xb5\x82\xe4\xba\x86\n\x00"
                         as *const u8 as *const libc::c_char, mtx);
        return 0 as *mut Mtx
    }
    SkinMatrix_MtxFToMtx(src, mtx);
    return mtx;
}
/* *
 * Produces a matrix which rotates vectors by angle a around a unit vector with components (x,y,z)
 */
#[no_mangle]
pub unsafe extern "C" fn func_800A7EC0(mut mf: *mut MtxF, mut a: s16,
                                       mut x: f32_0, mut y: f32_0,
                                       mut z: f32_0) {
    let mut sinA: f32_0 = 0.;
    let mut cosA: f32_0 = 0.;
    let mut xx: f32_0 = 0.;
    let mut yy: f32_0 = 0.;
    let mut zz: f32_0 = 0.;
    let mut xy: f32_0 = 0.;
    let mut yz: f32_0 = 0.;
    let mut xz: f32_0 = 0.;
    let mut pad: f32_0 = 0.;
    sinA = Math_SinS(a);
    cosA = Math_CosS(a);
    xx = x * x;
    yy = y * y;
    zz = z * z;
    xy = x * y;
    yz = y * z;
    xz = x * z;
    (*mf).c2rust_unnamed.xx = (1.0f32 - xx) * cosA + xx;
    (*mf).c2rust_unnamed.yx = (1.0f32 - cosA) * xy + z * sinA;
    (*mf).c2rust_unnamed.zx = (1.0f32 - cosA) * xz - y * sinA;
    (*mf).c2rust_unnamed.wx = 0.0f32;
    (*mf).c2rust_unnamed.xy = (1.0f32 - cosA) * xy - z * sinA;
    (*mf).c2rust_unnamed.yy = (1.0f32 - yy) * cosA + yy;
    (*mf).c2rust_unnamed.zy = (1.0f32 - cosA) * yz + x * sinA;
    (*mf).c2rust_unnamed.wy = 0.0f32;
    (*mf).c2rust_unnamed.xz = (1.0f32 - cosA) * xz + y * sinA;
    (*mf).c2rust_unnamed.yz = (1.0f32 - cosA) * yz - x * sinA;
    (*mf).c2rust_unnamed.zz = (1.0f32 - zz) * cosA + zz;
    (*mf).c2rust_unnamed.wz = 0.0f32;
    (*mf).c2rust_unnamed.zw = 0.0f32;
    (*mf).c2rust_unnamed.yw = (*mf).c2rust_unnamed.zw;
    (*mf).c2rust_unnamed.xw = (*mf).c2rust_unnamed.yw;
    (*mf).c2rust_unnamed.ww = 1.0f32;
}
#[no_mangle]
pub unsafe extern "C" fn func_800A8030(mut mf: *mut MtxF,
                                       mut arg1: *mut f32_0) {
    let mut n: f32_0 = 0.;
    let mut xNorm: f32_0 = 0.;
    let mut yNorm: f32_0 = 0.;
    let mut zNorm: f32_0 = 0.;
    let mut wxNorm: f32_0 = 0.;
    let mut wyNorm: f32_0 = 0.;
    let mut wzNorm: f32_0 = 0.;
    let mut xxNorm: f32_0 = 0.;
    let mut xyNorm: f32_0 = 0.;
    let mut xzNorm: f32_0 = 0.;
    let mut yyNorm: f32_0 = 0.;
    let mut yzNorm: f32_0 = 0.;
    let mut zzNorm: f32_0 = 0.;
    n =
        2.0f32 /
            (*arg1.offset(3 as libc::c_int as isize) *
                 *arg1.offset(3 as libc::c_int as isize) +
                 (*arg1.offset(2 as libc::c_int as isize) *
                      *arg1.offset(2 as libc::c_int as isize) +
                      (*arg1.offset(1 as libc::c_int as isize) *
                           *arg1.offset(1 as libc::c_int as isize) +
                           *arg1.offset(0 as libc::c_int as isize) *
                               *arg1.offset(0 as libc::c_int as isize))));
    xNorm = *arg1.offset(0 as libc::c_int as isize) * n;
    yNorm = *arg1.offset(1 as libc::c_int as isize) * n;
    zNorm = *arg1.offset(2 as libc::c_int as isize) * n;
    wxNorm = *arg1.offset(3 as libc::c_int as isize) * xNorm;
    wyNorm = *arg1.offset(3 as libc::c_int as isize) * yNorm;
    wzNorm = *arg1.offset(3 as libc::c_int as isize) * zNorm;
    xxNorm = *arg1.offset(0 as libc::c_int as isize) * xNorm;
    xyNorm = *arg1.offset(0 as libc::c_int as isize) * yNorm;
    xzNorm = *arg1.offset(0 as libc::c_int as isize) * zNorm;
    yyNorm = *arg1.offset(1 as libc::c_int as isize) * yNorm;
    yzNorm = *arg1.offset(1 as libc::c_int as isize) * zNorm;
    zzNorm = *arg1.offset(2 as libc::c_int as isize) * zNorm;
    (*mf).c2rust_unnamed.xx = 1.0f32 - (yyNorm + zzNorm);
    (*mf).c2rust_unnamed.yx = xyNorm + wzNorm;
    (*mf).c2rust_unnamed.zx = xzNorm - wyNorm;
    (*mf).c2rust_unnamed.wx = 0.0f32;
    (*mf).c2rust_unnamed.xy = xyNorm - wzNorm;
    (*mf).c2rust_unnamed.yy = 1.0f32 - (xxNorm + zzNorm);
    (*mf).c2rust_unnamed.zy = yzNorm + wxNorm;
    (*mf).c2rust_unnamed.wy = 0.0f32;
    (*mf).c2rust_unnamed.xz = yzNorm + wyNorm;
    (*mf).c2rust_unnamed.yz = yzNorm - wxNorm;
    (*mf).c2rust_unnamed.zz = 1.0f32 - (xxNorm + yyNorm);
    (*mf).c2rust_unnamed.wz = 0.0f32;
    (*mf).c2rust_unnamed.xw = 0.0f32;
    (*mf).c2rust_unnamed.yw = 0.0f32;
    (*mf).c2rust_unnamed.ww = 1.0f32;
    (*mf).c2rust_unnamed.zw = 0.0f32;
}
