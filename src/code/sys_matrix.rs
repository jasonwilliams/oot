#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case,
         non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(const_raw_ptr_to_usize_cast, register_tool)]
extern "C" {
    #[no_mangle]
    fn sqrtf(f: f32_0) -> f32_0;
    #[no_mangle]
    fn __assert(exp: *const libc::c_char, file: *const libc::c_char,
                line: s32);
    #[no_mangle]
    fn osSyncPrintf(fmt: *const libc::c_char, _: ...);
    #[no_mangle]
    fn Math_CosS(angle: s16) -> f32_0;
    #[no_mangle]
    fn Math_SinS(angle: s16) -> f32_0;
    #[no_mangle]
    fn SkinMatrix_MtxFMtxFMult(mfA: *mut MtxF, mfB: *mut MtxF,
                               dest: *mut MtxF);
    #[no_mangle]
    fn SkinMatrix_SetScale(mf: *mut MtxF, x: f32_0, y: f32_0, z: f32_0);
    #[no_mangle]
    fn SkinMatrix_SetRotateZYX(mf: *mut MtxF, x: s16, y: s16, z: s16);
    #[no_mangle]
    fn SkinMatrix_SetTranslate(mf: *mut MtxF, x: f32_0, y: f32_0, z: f32_0);
    #[no_mangle]
    fn GameState_Alloc(gameState: *mut GameState, size: size_t,
                       file: *mut libc::c_char, line: s32)
     -> *mut libc::c_void;
    #[no_mangle]
    fn Graph_Alloc(gfxCtx: *mut GraphicsContext, size: size_t)
     -> *mut libc::c_void;
    #[no_mangle]
    fn Fault_AddHungupAndCrash(_: *const libc::c_char, _: u32_0);
    #[no_mangle]
    fn Math_FAtan2F(y: f32_0, x: f32_0) -> f32_0;
    #[no_mangle]
    fn cosf(_: f32_0) -> f32_0;
    #[no_mangle]
    fn sinf(_: f32_0) -> f32_0;
    #[no_mangle]
    fn guMtxF2L(m1: *mut MtxF, m2: *mut Mtx);
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
pub struct Input {
    pub cur: OSContPad,
    pub prev: OSContPad,
    pub press: OSContPad,
    pub rel: OSContPad,
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
pub type C2RustUnnamed_2 = libc::c_uint;
pub const MTXMODE_APPLY: C2RustUnnamed_2 = 1;
pub const MTXMODE_NEW: C2RustUnnamed_2 = 0;
// clang-format off
#[no_mangle]
pub static mut gMtxClear: Mtx =
    Mtx{m:
            [[65536 as libc::c_int as libc::c_long,
              0 as libc::c_int as libc::c_long,
              1 as libc::c_int as libc::c_long,
              0 as libc::c_int as libc::c_long],
             [0 as libc::c_int as libc::c_long,
              65536 as libc::c_int as libc::c_long,
              0 as libc::c_int as libc::c_long,
              1 as libc::c_int as libc::c_long],
             [0 as libc::c_int as libc::c_long,
              0 as libc::c_int as libc::c_long,
              0 as libc::c_int as libc::c_long,
              0 as libc::c_int as libc::c_long],
             [0 as libc::c_int as libc::c_long,
              0 as libc::c_int as libc::c_long,
              0 as libc::c_int as libc::c_long,
              0 as libc::c_int as libc::c_long]],};
#[no_mangle]
pub static mut gMtxFClear: MtxF =
    MtxF{mf:
             [[1.0f32, 0.0f32, 0.0f32, 0.0f32],
              [0.0f32, 1.0f32, 0.0f32, 0.0f32],
              [0.0f32, 0.0f32, 1.0f32, 0.0f32],
              [0.0f32, 0.0f32, 0.0f32, 1.0f32]],};
// clang-format on
#[no_mangle]
pub static mut sMatrixStack: *mut MtxF = 0 as *const MtxF as *mut MtxF;
// "Matrix_stack"
#[no_mangle]
pub static mut sCurrentMatrix: *mut MtxF = 0 as *const MtxF as *mut MtxF;
// "Matrix_now"
#[no_mangle]
pub unsafe extern "C" fn Matrix_Init(mut gameState: *mut GameState) {
    sCurrentMatrix =
        GameState_Alloc(gameState,
                        (20 as libc::c_int as
                             libc::c_uint).wrapping_mul(::std::mem::size_of::<MtxF>()
                                                            as libc::c_ulong)
                            as size_t,
                        b"../sys_matrix.c\x00" as *const u8 as
                            *const libc::c_char as *mut libc::c_char,
                        153 as libc::c_int) as *mut MtxF;
    sMatrixStack = sCurrentMatrix;
}
#[no_mangle]
pub unsafe extern "C" fn Matrix_Push() {
    Matrix_MtxFCopy(sCurrentMatrix.offset(1 as libc::c_int as isize),
                    sCurrentMatrix);
    sCurrentMatrix = sCurrentMatrix.offset(1);
}
#[no_mangle]
pub unsafe extern "C" fn Matrix_Pop() {
    sCurrentMatrix = sCurrentMatrix.offset(-1);
    if sCurrentMatrix >= sMatrixStack {
    } else {
        __assert(b"Matrix_now >= Matrix_stack\x00" as *const u8 as
                     *const libc::c_char,
                 b"../sys_matrix.c\x00" as *const u8 as *const libc::c_char,
                 176 as libc::c_int);
    };
}
#[no_mangle]
pub unsafe extern "C" fn Matrix_Get(mut dest: *mut MtxF) {
    Matrix_MtxFCopy(dest, sCurrentMatrix);
}
#[no_mangle]
pub unsafe extern "C" fn Matrix_Put(mut src: *mut MtxF) {
    Matrix_MtxFCopy(sCurrentMatrix, src);
}
#[no_mangle]
pub unsafe extern "C" fn Matrix_GetCurrent() -> *mut MtxF {
    return sCurrentMatrix;
}
#[no_mangle]
pub unsafe extern "C" fn Matrix_Mult(mut mf: *mut MtxF, mut mode: u8_0) {
    let mut cmf: *mut MtxF = Matrix_GetCurrent();
    if mode as libc::c_int == MTXMODE_APPLY as libc::c_int {
        SkinMatrix_MtxFMtxFMult(cmf, mf, cmf);
    } else { Matrix_MtxFCopy(sCurrentMatrix, mf); };
}
#[no_mangle]
pub unsafe extern "C" fn Matrix_Translate(mut x: f32_0, mut y: f32_0,
                                          mut z: f32_0, mut mode: u8_0) {
    let mut cmf: *mut MtxF = sCurrentMatrix;
    let mut tx: f32_0 = 0.;
    let mut ty: f32_0 = 0.;
    if mode as libc::c_int == MTXMODE_APPLY as libc::c_int {
        tx = (*cmf).c2rust_unnamed.xx;
        ty = (*cmf).c2rust_unnamed.xy;
        (*cmf).c2rust_unnamed.xw +=
            tx * x + ty * y + (*cmf).c2rust_unnamed.xz * z;
        tx = (*cmf).c2rust_unnamed.yx;
        ty = (*cmf).c2rust_unnamed.yy;
        (*cmf).c2rust_unnamed.yw +=
            tx * x + ty * y + (*cmf).c2rust_unnamed.yz * z;
        tx = (*cmf).c2rust_unnamed.zx;
        ty = (*cmf).c2rust_unnamed.zy;
        (*cmf).c2rust_unnamed.zw +=
            tx * x + ty * y + (*cmf).c2rust_unnamed.zz * z;
        tx = (*cmf).c2rust_unnamed.wx;
        ty = (*cmf).c2rust_unnamed.wy;
        (*cmf).c2rust_unnamed.ww +=
            tx * x + ty * y + (*cmf).c2rust_unnamed.wz * z
    } else { SkinMatrix_SetTranslate(cmf, x, y, z); };
}
#[no_mangle]
pub unsafe extern "C" fn Matrix_Scale(mut x: f32_0, mut y: f32_0,
                                      mut z: f32_0, mut mode: u8_0) {
    let mut cmf: *mut MtxF = sCurrentMatrix;
    if mode as libc::c_int == MTXMODE_APPLY as libc::c_int {
        (*cmf).c2rust_unnamed.xx *= x;
        (*cmf).c2rust_unnamed.yx *= x;
        (*cmf).c2rust_unnamed.zx *= x;
        (*cmf).c2rust_unnamed.xy *= y;
        (*cmf).c2rust_unnamed.yy *= y;
        (*cmf).c2rust_unnamed.zy *= y;
        (*cmf).c2rust_unnamed.xz *= z;
        (*cmf).c2rust_unnamed.yz *= z;
        (*cmf).c2rust_unnamed.zz *= z;
        (*cmf).c2rust_unnamed.wx *= x;
        (*cmf).c2rust_unnamed.wy *= y;
        (*cmf).c2rust_unnamed.wz *= z
    } else { SkinMatrix_SetScale(cmf, x, y, z); };
}
#[no_mangle]
pub unsafe extern "C" fn Matrix_RotateX(mut x: f32_0, mut mode: u8_0) {
    let mut cmf: *mut MtxF = 0 as *mut MtxF;
    let mut sin: f32_0 = 0.;
    let mut cos: f32_0 = 0.;
    let mut temp1: f32_0 = 0.;
    let mut temp2: f32_0 = 0.;
    if mode as libc::c_int == MTXMODE_APPLY as libc::c_int {
        if x != 0 as libc::c_int as libc::c_float {
            cmf = sCurrentMatrix;
            sin = sinf(x);
            cos = cosf(x);
            temp1 = (*cmf).c2rust_unnamed.xy;
            temp2 = (*cmf).c2rust_unnamed.xz;
            (*cmf).c2rust_unnamed.xy = temp1 * cos + temp2 * sin;
            (*cmf).c2rust_unnamed.xz = temp2 * cos - temp1 * sin;
            temp1 = (*cmf).c2rust_unnamed.yy;
            temp2 = (*cmf).c2rust_unnamed.yz;
            (*cmf).c2rust_unnamed.yy = temp1 * cos + temp2 * sin;
            (*cmf).c2rust_unnamed.yz = temp2 * cos - temp1 * sin;
            temp1 = (*cmf).c2rust_unnamed.zy;
            temp2 = (*cmf).c2rust_unnamed.zz;
            (*cmf).c2rust_unnamed.zy = temp1 * cos + temp2 * sin;
            (*cmf).c2rust_unnamed.zz = temp2 * cos - temp1 * sin;
            temp1 = (*cmf).c2rust_unnamed.wy;
            temp2 = (*cmf).c2rust_unnamed.wz;
            (*cmf).c2rust_unnamed.wy = temp1 * cos + temp2 * sin;
            (*cmf).c2rust_unnamed.wz = temp2 * cos - temp1 * sin
        }
    } else {
        cmf = sCurrentMatrix;
        if x != 0 as libc::c_int as libc::c_float {
            sin = sinf(x);
            cos = cosf(x)
        } else { sin = 0.0f32; cos = 1.0f32 }
        (*cmf).c2rust_unnamed.yx = 0.0f32;
        (*cmf).c2rust_unnamed.zx = 0.0f32;
        (*cmf).c2rust_unnamed.wx = 0.0f32;
        (*cmf).c2rust_unnamed.xy = 0.0f32;
        (*cmf).c2rust_unnamed.wy = 0.0f32;
        (*cmf).c2rust_unnamed.xz = 0.0f32;
        (*cmf).c2rust_unnamed.wz = 0.0f32;
        (*cmf).c2rust_unnamed.xw = 0.0f32;
        (*cmf).c2rust_unnamed.yw = 0.0f32;
        (*cmf).c2rust_unnamed.zw = 0.0f32;
        (*cmf).c2rust_unnamed.xx = 1.0f32;
        (*cmf).c2rust_unnamed.ww = 1.0f32;
        (*cmf).c2rust_unnamed.yy = cos;
        (*cmf).c2rust_unnamed.zz = cos;
        (*cmf).c2rust_unnamed.zy = sin;
        (*cmf).c2rust_unnamed.yz = -sin
    };
}
#[no_mangle]
pub unsafe extern "C" fn Matrix_RotateY(mut y: f32_0, mut mode: u8_0) {
    let mut cmf: *mut MtxF = 0 as *mut MtxF;
    let mut sin: f32_0 = 0.;
    let mut cos: f32_0 = 0.;
    let mut temp1: f32_0 = 0.;
    let mut temp2: f32_0 = 0.;
    if mode as libc::c_int == MTXMODE_APPLY as libc::c_int {
        if y != 0 as libc::c_int as libc::c_float {
            cmf = sCurrentMatrix;
            sin = sinf(y);
            cos = cosf(y);
            temp1 = (*cmf).c2rust_unnamed.xx;
            temp2 = (*cmf).c2rust_unnamed.xz;
            (*cmf).c2rust_unnamed.xx = temp1 * cos - temp2 * sin;
            (*cmf).c2rust_unnamed.xz = temp1 * sin + temp2 * cos;
            temp1 = (*cmf).c2rust_unnamed.yx;
            temp2 = (*cmf).c2rust_unnamed.yz;
            (*cmf).c2rust_unnamed.yx = temp1 * cos - temp2 * sin;
            (*cmf).c2rust_unnamed.yz = temp1 * sin + temp2 * cos;
            temp1 = (*cmf).c2rust_unnamed.zx;
            temp2 = (*cmf).c2rust_unnamed.zz;
            (*cmf).c2rust_unnamed.zx = temp1 * cos - temp2 * sin;
            (*cmf).c2rust_unnamed.zz = temp1 * sin + temp2 * cos;
            temp1 = (*cmf).c2rust_unnamed.wx;
            temp2 = (*cmf).c2rust_unnamed.wz;
            (*cmf).c2rust_unnamed.wx = temp1 * cos - temp2 * sin;
            (*cmf).c2rust_unnamed.wz = temp1 * sin + temp2 * cos
        }
    } else {
        cmf = sCurrentMatrix;
        if y != 0 as libc::c_int as libc::c_float {
            sin = sinf(y);
            cos = cosf(y)
        } else { sin = 0.0f32; cos = 1.0f32 }
        (*cmf).c2rust_unnamed.yx = 0.0f32;
        (*cmf).c2rust_unnamed.wx = 0.0f32;
        (*cmf).c2rust_unnamed.xy = 0.0f32;
        (*cmf).c2rust_unnamed.zy = 0.0f32;
        (*cmf).c2rust_unnamed.wy = 0.0f32;
        (*cmf).c2rust_unnamed.yz = 0.0f32;
        (*cmf).c2rust_unnamed.wz = 0.0f32;
        (*cmf).c2rust_unnamed.xw = 0.0f32;
        (*cmf).c2rust_unnamed.yw = 0.0f32;
        (*cmf).c2rust_unnamed.zw = 0.0f32;
        (*cmf).c2rust_unnamed.yy = 1.0f32;
        (*cmf).c2rust_unnamed.ww = 1.0f32;
        (*cmf).c2rust_unnamed.xx = cos;
        (*cmf).c2rust_unnamed.zz = cos;
        (*cmf).c2rust_unnamed.zx = -sin;
        (*cmf).c2rust_unnamed.xz = sin
    };
}
#[no_mangle]
pub unsafe extern "C" fn Matrix_RotateZ(mut z: f32_0, mut mode: u8_0) {
    let mut cmf: *mut MtxF = 0 as *mut MtxF;
    let mut sin: f32_0 = 0.;
    let mut cos: f32_0 = 0.;
    let mut temp1: f32_0 = 0.;
    let mut temp2: f32_0 = 0.;
    if mode as libc::c_int == MTXMODE_APPLY as libc::c_int {
        if z != 0 as libc::c_int as libc::c_float {
            cmf = sCurrentMatrix;
            sin = sinf(z);
            cos = cosf(z);
            temp1 = (*cmf).c2rust_unnamed.xx;
            temp2 = (*cmf).c2rust_unnamed.xy;
            (*cmf).c2rust_unnamed.xx = temp1 * cos + temp2 * sin;
            (*cmf).c2rust_unnamed.xy = temp2 * cos - temp1 * sin;
            temp1 = (*cmf).c2rust_unnamed.yx;
            temp2 = (*cmf).c2rust_unnamed.yy;
            (*cmf).c2rust_unnamed.yx = temp1 * cos + temp2 * sin;
            (*cmf).c2rust_unnamed.yy = temp2 * cos - temp1 * sin;
            temp1 = (*cmf).c2rust_unnamed.zx;
            temp2 = (*cmf).c2rust_unnamed.zy;
            (*cmf).c2rust_unnamed.zx = temp1 * cos + temp2 * sin;
            (*cmf).c2rust_unnamed.zy = temp2 * cos - temp1 * sin;
            temp1 = (*cmf).c2rust_unnamed.wx;
            temp2 = (*cmf).c2rust_unnamed.wy;
            (*cmf).c2rust_unnamed.wx = temp1 * cos + temp2 * sin;
            (*cmf).c2rust_unnamed.wy = temp2 * cos - temp1 * sin
        }
    } else {
        cmf = sCurrentMatrix;
        if z != 0 as libc::c_int as libc::c_float {
            sin = sinf(z);
            cos = cosf(z)
        } else { sin = 0.0f32; cos = 1.0f32 }
        (*cmf).c2rust_unnamed.zx = 0.0f32;
        (*cmf).c2rust_unnamed.wx = 0.0f32;
        (*cmf).c2rust_unnamed.zy = 0.0f32;
        (*cmf).c2rust_unnamed.wy = 0.0f32;
        (*cmf).c2rust_unnamed.xz = 0.0f32;
        (*cmf).c2rust_unnamed.yz = 0.0f32;
        (*cmf).c2rust_unnamed.wz = 0.0f32;
        (*cmf).c2rust_unnamed.xw = 0.0f32;
        (*cmf).c2rust_unnamed.yw = 0.0f32;
        (*cmf).c2rust_unnamed.zw = 0.0f32;
        (*cmf).c2rust_unnamed.zz = 1.0f32;
        (*cmf).c2rust_unnamed.ww = 1.0f32;
        (*cmf).c2rust_unnamed.xx = cos;
        (*cmf).c2rust_unnamed.yy = cos;
        (*cmf).c2rust_unnamed.yx = sin;
        (*cmf).c2rust_unnamed.xy = -sin
    };
}
/* *
 * Rotate using ZYX Tait-Bryan angles.
 * This means a (column) vector is first rotated around X, then around Y, then around Z, then (if `mode` is
 * `MTXMODE_APPLY`) gets transformed according to whatever the matrix was before adding the ZYX rotation.
 * Original Name: Matrix_RotateXYZ, changed to reflect rotation order.
 */
#[no_mangle]
pub unsafe extern "C" fn Matrix_RotateZYX(mut x: s16, mut y: s16, mut z: s16,
                                          mut mode: u8_0) {
    let mut cmf: *mut MtxF = sCurrentMatrix;
    let mut temp1: f32_0 = 0.;
    let mut temp2: f32_0 = 0.;
    let mut sin: f32_0 = 0.;
    let mut cos: f32_0 = 0.;
    if mode as libc::c_int == MTXMODE_APPLY as libc::c_int {
        sin = Math_SinS(z);
        cos = Math_CosS(z);
        temp1 = (*cmf).c2rust_unnamed.xx;
        temp2 = (*cmf).c2rust_unnamed.xy;
        (*cmf).c2rust_unnamed.xx = temp1 * cos + temp2 * sin;
        (*cmf).c2rust_unnamed.xy = temp2 * cos - temp1 * sin;
        temp1 = (*cmf).c2rust_unnamed.yx;
        temp2 = (*cmf).c2rust_unnamed.yy;
        (*cmf).c2rust_unnamed.yx = temp1 * cos + temp2 * sin;
        (*cmf).c2rust_unnamed.yy = temp2 * cos - temp1 * sin;
        temp1 = (*cmf).c2rust_unnamed.zx;
        temp2 = (*cmf).c2rust_unnamed.zy;
        (*cmf).c2rust_unnamed.zx = temp1 * cos + temp2 * sin;
        (*cmf).c2rust_unnamed.zy = temp2 * cos - temp1 * sin;
        temp1 = (*cmf).c2rust_unnamed.wx;
        temp2 = (*cmf).c2rust_unnamed.wy;
        (*cmf).c2rust_unnamed.wx = temp1 * cos + temp2 * sin;
        (*cmf).c2rust_unnamed.wy = temp2 * cos - temp1 * sin;
        if y as libc::c_int != 0 as libc::c_int {
            sin = Math_SinS(y);
            cos = Math_CosS(y);
            temp1 = (*cmf).c2rust_unnamed.xx;
            temp2 = (*cmf).c2rust_unnamed.xz;
            (*cmf).c2rust_unnamed.xx = temp1 * cos - temp2 * sin;
            (*cmf).c2rust_unnamed.xz = temp1 * sin + temp2 * cos;
            temp1 = (*cmf).c2rust_unnamed.yx;
            temp2 = (*cmf).c2rust_unnamed.yz;
            (*cmf).c2rust_unnamed.yx = temp1 * cos - temp2 * sin;
            (*cmf).c2rust_unnamed.yz = temp1 * sin + temp2 * cos;
            temp1 = (*cmf).c2rust_unnamed.zx;
            temp2 = (*cmf).c2rust_unnamed.zz;
            (*cmf).c2rust_unnamed.zx = temp1 * cos - temp2 * sin;
            (*cmf).c2rust_unnamed.zz = temp1 * sin + temp2 * cos;
            temp1 = (*cmf).c2rust_unnamed.wx;
            temp2 = (*cmf).c2rust_unnamed.wz;
            (*cmf).c2rust_unnamed.wx = temp1 * cos - temp2 * sin;
            (*cmf).c2rust_unnamed.wz = temp1 * sin + temp2 * cos
        }
        if x as libc::c_int != 0 as libc::c_int {
            sin = Math_SinS(x);
            cos = Math_CosS(x);
            temp1 = (*cmf).c2rust_unnamed.xy;
            temp2 = (*cmf).c2rust_unnamed.xz;
            (*cmf).c2rust_unnamed.xy = temp1 * cos + temp2 * sin;
            (*cmf).c2rust_unnamed.xz = temp2 * cos - temp1 * sin;
            temp1 = (*cmf).c2rust_unnamed.yy;
            temp2 = (*cmf).c2rust_unnamed.yz;
            (*cmf).c2rust_unnamed.yy = temp1 * cos + temp2 * sin;
            (*cmf).c2rust_unnamed.yz = temp2 * cos - temp1 * sin;
            temp1 = (*cmf).c2rust_unnamed.zy;
            temp2 = (*cmf).c2rust_unnamed.zz;
            (*cmf).c2rust_unnamed.zy = temp1 * cos + temp2 * sin;
            (*cmf).c2rust_unnamed.zz = temp2 * cos - temp1 * sin;
            temp1 = (*cmf).c2rust_unnamed.wy;
            temp2 = (*cmf).c2rust_unnamed.wz;
            (*cmf).c2rust_unnamed.wy = temp1 * cos + temp2 * sin;
            (*cmf).c2rust_unnamed.wz = temp2 * cos - temp1 * sin
        }
    } else { SkinMatrix_SetRotateZYX(cmf, x, y, z); };
}
/* *
 * Translate and rotate using ZYX Tait-Bryan angles.
 * This means a (column) vector is first rotated around X, then around Y, then around Z, then translated, then gets
 * transformed according to whatever the matrix was previously.
 */
#[no_mangle]
pub unsafe extern "C" fn Matrix_TranslateRotateZYX(mut translation:
                                                       *mut Vec3f,
                                                   mut rotation: *mut Vec3s) {
    let mut cmf: *mut MtxF = sCurrentMatrix;
    let mut sin: f32_0 = Math_SinS((*rotation).z);
    let mut cos: f32_0 = Math_CosS((*rotation).z);
    let mut temp1: f32_0 = 0.;
    let mut temp2: f32_0 = 0.;
    temp1 = (*cmf).c2rust_unnamed.xx;
    temp2 = (*cmf).c2rust_unnamed.xy;
    (*cmf).c2rust_unnamed.xw +=
        temp1 * (*translation).x + temp2 * (*translation).y +
            (*cmf).c2rust_unnamed.xz * (*translation).z;
    (*cmf).c2rust_unnamed.xx = temp1 * cos + temp2 * sin;
    (*cmf).c2rust_unnamed.xy = temp2 * cos - temp1 * sin;
    temp1 = (*cmf).c2rust_unnamed.yx;
    temp2 = (*cmf).c2rust_unnamed.yy;
    (*cmf).c2rust_unnamed.yw +=
        temp1 * (*translation).x + temp2 * (*translation).y +
            (*cmf).c2rust_unnamed.yz * (*translation).z;
    (*cmf).c2rust_unnamed.yx = temp1 * cos + temp2 * sin;
    (*cmf).c2rust_unnamed.yy = temp2 * cos - temp1 * sin;
    temp1 = (*cmf).c2rust_unnamed.zx;
    temp2 = (*cmf).c2rust_unnamed.zy;
    (*cmf).c2rust_unnamed.zw +=
        temp1 * (*translation).x + temp2 * (*translation).y +
            (*cmf).c2rust_unnamed.zz * (*translation).z;
    (*cmf).c2rust_unnamed.zx = temp1 * cos + temp2 * sin;
    (*cmf).c2rust_unnamed.zy = temp2 * cos - temp1 * sin;
    temp1 = (*cmf).c2rust_unnamed.wx;
    temp2 = (*cmf).c2rust_unnamed.wy;
    (*cmf).c2rust_unnamed.ww +=
        temp1 * (*translation).x + temp2 * (*translation).y +
            (*cmf).c2rust_unnamed.wz * (*translation).z;
    (*cmf).c2rust_unnamed.wx = temp1 * cos + temp2 * sin;
    (*cmf).c2rust_unnamed.wy = temp2 * cos - temp1 * sin;
    if (*rotation).y as libc::c_int != 0 as libc::c_int {
        sin = Math_SinS((*rotation).y);
        cos = Math_CosS((*rotation).y);
        temp1 = (*cmf).c2rust_unnamed.xx;
        temp2 = (*cmf).c2rust_unnamed.xz;
        (*cmf).c2rust_unnamed.xx = temp1 * cos - temp2 * sin;
        (*cmf).c2rust_unnamed.xz = temp1 * sin + temp2 * cos;
        temp1 = (*cmf).c2rust_unnamed.yx;
        temp2 = (*cmf).c2rust_unnamed.yz;
        (*cmf).c2rust_unnamed.yx = temp1 * cos - temp2 * sin;
        (*cmf).c2rust_unnamed.yz = temp1 * sin + temp2 * cos;
        temp1 = (*cmf).c2rust_unnamed.zx;
        temp2 = (*cmf).c2rust_unnamed.zz;
        (*cmf).c2rust_unnamed.zx = temp1 * cos - temp2 * sin;
        (*cmf).c2rust_unnamed.zz = temp1 * sin + temp2 * cos;
        temp1 = (*cmf).c2rust_unnamed.wx;
        temp2 = (*cmf).c2rust_unnamed.wz;
        (*cmf).c2rust_unnamed.wx = temp1 * cos - temp2 * sin;
        (*cmf).c2rust_unnamed.wz = temp1 * sin + temp2 * cos
    }
    if (*rotation).x as libc::c_int != 0 as libc::c_int {
        sin = Math_SinS((*rotation).x);
        cos = Math_CosS((*rotation).x);
        temp1 = (*cmf).c2rust_unnamed.xy;
        temp2 = (*cmf).c2rust_unnamed.xz;
        (*cmf).c2rust_unnamed.xy = temp1 * cos + temp2 * sin;
        (*cmf).c2rust_unnamed.xz = temp2 * cos - temp1 * sin;
        temp1 = (*cmf).c2rust_unnamed.yy;
        temp2 = (*cmf).c2rust_unnamed.yz;
        (*cmf).c2rust_unnamed.yy = temp1 * cos + temp2 * sin;
        (*cmf).c2rust_unnamed.yz = temp2 * cos - temp1 * sin;
        temp1 = (*cmf).c2rust_unnamed.zy;
        temp2 = (*cmf).c2rust_unnamed.zz;
        (*cmf).c2rust_unnamed.zy = temp1 * cos + temp2 * sin;
        (*cmf).c2rust_unnamed.zz = temp2 * cos - temp1 * sin;
        temp1 = (*cmf).c2rust_unnamed.wy;
        temp2 = (*cmf).c2rust_unnamed.wz;
        (*cmf).c2rust_unnamed.wy = temp1 * cos + temp2 * sin;
        (*cmf).c2rust_unnamed.wz = temp2 * cos - temp1 * sin
    };
}
#[no_mangle]
pub unsafe extern "C" fn func_800D1694(mut x: f32_0, mut y: f32_0,
                                       mut z: f32_0, mut vec: *mut Vec3s) {
    let mut cmf: *mut MtxF = sCurrentMatrix;
    let mut sp30: f32_0 = Math_SinS((*vec).y);
    let mut sp2C: f32_0 = Math_CosS((*vec).y);
    let mut sp28: f32_0 = 0.;
    let mut sp24: f32_0 = 0.;
    (*cmf).c2rust_unnamed.xx = sp2C;
    (*cmf).c2rust_unnamed.zx = -sp30;
    (*cmf).c2rust_unnamed.xw = x;
    (*cmf).c2rust_unnamed.yw = y;
    (*cmf).c2rust_unnamed.zw = z;
    (*cmf).c2rust_unnamed.wx = 0.0f32;
    (*cmf).c2rust_unnamed.wy = 0.0f32;
    (*cmf).c2rust_unnamed.wz = 0.0f32;
    (*cmf).c2rust_unnamed.ww = 1.0f32;
    if (*vec).x as libc::c_int != 0 as libc::c_int {
        sp24 = Math_SinS((*vec).x);
        sp28 = Math_CosS((*vec).x);
        (*cmf).c2rust_unnamed.zz = sp2C * sp28;
        (*cmf).c2rust_unnamed.zy = sp2C * sp24;
        (*cmf).c2rust_unnamed.xz = sp30 * sp28;
        (*cmf).c2rust_unnamed.xy = sp30 * sp24;
        (*cmf).c2rust_unnamed.yz = -sp24;
        (*cmf).c2rust_unnamed.yy = sp28
    } else {
        (*cmf).c2rust_unnamed.zz = sp2C;
        (*cmf).c2rust_unnamed.xz = sp30;
        (*cmf).c2rust_unnamed.yz = 0.0f32;
        (*cmf).c2rust_unnamed.zy = 0.0f32;
        (*cmf).c2rust_unnamed.xy = 0.0f32;
        (*cmf).c2rust_unnamed.yy = 1.0f32
    }
    if (*vec).z as libc::c_int != 0 as libc::c_int {
        sp24 = Math_SinS((*vec).z);
        sp28 = Math_CosS((*vec).z);
        sp30 = (*cmf).c2rust_unnamed.xx;
        sp2C = (*cmf).c2rust_unnamed.xy;
        (*cmf).c2rust_unnamed.xx = sp30 * sp28 + sp2C * sp24;
        (*cmf).c2rust_unnamed.xy = sp2C * sp28 - sp30 * sp24;
        sp30 = (*cmf).c2rust_unnamed.zx;
        sp2C = (*cmf).c2rust_unnamed.zy;
        (*cmf).c2rust_unnamed.zx = sp30 * sp28 + sp2C * sp24;
        (*cmf).c2rust_unnamed.zy = sp2C * sp28 - sp30 * sp24;
        sp2C = (*cmf).c2rust_unnamed.yy;
        (*cmf).c2rust_unnamed.yx = sp2C * sp24;
        (*cmf).c2rust_unnamed.yy = sp2C * sp28
    } else { (*cmf).c2rust_unnamed.yx = 0.0f32 };
}
#[no_mangle]
pub unsafe extern "C" fn Matrix_MtxFToMtx(mut src: *mut MtxF,
                                          mut dest: *mut Mtx) -> *mut Mtx {
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
    return dest;
}
#[no_mangle]
pub unsafe extern "C" fn Matrix_ToMtx(mut dest: *mut Mtx,
                                      mut file: *mut libc::c_char,
                                      mut line: s32) -> *mut Mtx {
    return Matrix_MtxFToMtx(Matrix_CheckFloats(sCurrentMatrix, file, line),
                            dest);
}
#[no_mangle]
pub unsafe extern "C" fn Matrix_NewMtx(mut gfxCtx: *mut GraphicsContext,
                                       mut file: *mut libc::c_char,
                                       mut line: s32) -> *mut Mtx {
    return Matrix_ToMtx(Graph_Alloc(gfxCtx,
                                    ::std::mem::size_of::<Mtx>() as
                                        libc::c_ulong as size_t) as *mut Mtx,
                        file, line);
}
#[no_mangle]
pub unsafe extern "C" fn Matrix_MtxFToNewMtx(mut src: *mut MtxF,
                                             mut gfxCtx: *mut GraphicsContext)
 -> *mut Mtx {
    return Matrix_MtxFToMtx(src,
                            Graph_Alloc(gfxCtx,
                                        ::std::mem::size_of::<Mtx>() as
                                            libc::c_ulong as size_t) as
                                *mut Mtx);
}
#[no_mangle]
pub unsafe extern "C" fn Matrix_MultVec3f(mut src: *mut Vec3f,
                                          mut dest: *mut Vec3f) {
    let mut cmf: *mut MtxF = sCurrentMatrix;
    (*dest).x =
        (*cmf).c2rust_unnamed.xw +
            ((*cmf).c2rust_unnamed.xx * (*src).x +
                 (*cmf).c2rust_unnamed.xy * (*src).y +
                 (*cmf).c2rust_unnamed.xz * (*src).z);
    (*dest).y =
        (*cmf).c2rust_unnamed.yw +
            ((*cmf).c2rust_unnamed.yx * (*src).x +
                 (*cmf).c2rust_unnamed.yy * (*src).y +
                 (*cmf).c2rust_unnamed.yz * (*src).z);
    (*dest).z =
        (*cmf).c2rust_unnamed.zw +
            ((*cmf).c2rust_unnamed.zx * (*src).x +
                 (*cmf).c2rust_unnamed.zy * (*src).y +
                 (*cmf).c2rust_unnamed.zz * (*src).z);
}
#[no_mangle]
pub unsafe extern "C" fn Matrix_MtxFCopy(mut dest: *mut MtxF,
                                         mut src: *mut MtxF) {
    (*dest).c2rust_unnamed.xx = (*src).c2rust_unnamed.xx;
    (*dest).c2rust_unnamed.yx = (*src).c2rust_unnamed.yx;
    (*dest).c2rust_unnamed.zx = (*src).c2rust_unnamed.zx;
    (*dest).c2rust_unnamed.wx = (*src).c2rust_unnamed.wx;
    (*dest).c2rust_unnamed.xy = (*src).c2rust_unnamed.xy;
    (*dest).c2rust_unnamed.yy = (*src).c2rust_unnamed.yy;
    (*dest).c2rust_unnamed.zy = (*src).c2rust_unnamed.zy;
    (*dest).c2rust_unnamed.wy = (*src).c2rust_unnamed.wy;
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
    (*dest).c2rust_unnamed.xz = (*src).c2rust_unnamed.xz;
    (*dest).c2rust_unnamed.yz = (*src).c2rust_unnamed.yz;
    (*dest).c2rust_unnamed.zz = (*src).c2rust_unnamed.zz;
    (*dest).c2rust_unnamed.wz = (*src).c2rust_unnamed.wz;
    (*dest).c2rust_unnamed.xw = (*src).c2rust_unnamed.xw;
    (*dest).c2rust_unnamed.yw = (*src).c2rust_unnamed.yw;
    (*dest).c2rust_unnamed.zw = (*src).c2rust_unnamed.zw;
    (*dest).c2rust_unnamed.ww = (*src).c2rust_unnamed.ww;
}
#[no_mangle]
pub unsafe extern "C" fn Matrix_MtxToMtxF(mut src: *mut Mtx,
                                          mut dest: *mut MtxF) {
    let mut m1: *mut u16_0 =
        &mut *(*(*src).m.as_mut_ptr().offset(0 as libc::c_int as
                                                 isize)).as_mut_ptr().offset(0
                                                                                 as
                                                                                 libc::c_int
                                                                                 as
                                                                                 isize)
            as *mut libc::c_long as *mut u16_0;
    let mut m2: *mut u16_0 =
        &mut *(*(*src).m.as_mut_ptr().offset(2 as libc::c_int as
                                                 isize)).as_mut_ptr().offset(0
                                                                                 as
                                                                                 libc::c_int
                                                                                 as
                                                                                 isize)
            as *mut libc::c_long as *mut u16_0;
    (*dest).c2rust_unnamed.xx =
        ((*m1.offset(0 as libc::c_int as isize) as libc::c_int) <<
             0x10 as libc::c_int |
             *m2.offset(0 as libc::c_int as isize) as libc::c_int) as
            libc::c_float * (1 as libc::c_int as libc::c_float / 65536.0f32);
    (*dest).c2rust_unnamed.yx =
        ((*m1.offset(1 as libc::c_int as isize) as libc::c_int) <<
             0x10 as libc::c_int |
             *m2.offset(1 as libc::c_int as isize) as libc::c_int) as
            libc::c_float * (1 as libc::c_int as libc::c_float / 65536.0f32);
    (*dest).c2rust_unnamed.zx =
        ((*m1.offset(2 as libc::c_int as isize) as libc::c_int) <<
             0x10 as libc::c_int |
             *m2.offset(2 as libc::c_int as isize) as libc::c_int) as
            libc::c_float * (1 as libc::c_int as libc::c_float / 65536.0f32);
    (*dest).c2rust_unnamed.wx =
        ((*m1.offset(3 as libc::c_int as isize) as libc::c_int) <<
             0x10 as libc::c_int |
             *m2.offset(3 as libc::c_int as isize) as libc::c_int) as
            libc::c_float * (1 as libc::c_int as libc::c_float / 65536.0f32);
    (*dest).c2rust_unnamed.xy =
        ((*m1.offset(4 as libc::c_int as isize) as libc::c_int) <<
             0x10 as libc::c_int |
             *m2.offset(4 as libc::c_int as isize) as libc::c_int) as
            libc::c_float * (1 as libc::c_int as libc::c_float / 65536.0f32);
    (*dest).c2rust_unnamed.yy =
        ((*m1.offset(5 as libc::c_int as isize) as libc::c_int) <<
             0x10 as libc::c_int |
             *m2.offset(5 as libc::c_int as isize) as libc::c_int) as
            libc::c_float * (1 as libc::c_int as libc::c_float / 65536.0f32);
    (*dest).c2rust_unnamed.zy =
        ((*m1.offset(6 as libc::c_int as isize) as libc::c_int) <<
             0x10 as libc::c_int |
             *m2.offset(6 as libc::c_int as isize) as libc::c_int) as
            libc::c_float * (1 as libc::c_int as libc::c_float / 65536.0f32);
    (*dest).c2rust_unnamed.wy =
        ((*m1.offset(7 as libc::c_int as isize) as libc::c_int) <<
             0x10 as libc::c_int |
             *m2.offset(7 as libc::c_int as isize) as libc::c_int) as
            libc::c_float * (1 as libc::c_int as libc::c_float / 65536.0f32);
    (*dest).c2rust_unnamed.xz =
        ((*m1.offset(8 as libc::c_int as isize) as libc::c_int) <<
             0x10 as libc::c_int |
             *m2.offset(8 as libc::c_int as isize) as libc::c_int) as
            libc::c_float * (1 as libc::c_int as libc::c_float / 65536.0f32);
    (*dest).c2rust_unnamed.yz =
        ((*m1.offset(9 as libc::c_int as isize) as libc::c_int) <<
             0x10 as libc::c_int |
             *m2.offset(9 as libc::c_int as isize) as libc::c_int) as
            libc::c_float * (1 as libc::c_int as libc::c_float / 65536.0f32);
    (*dest).c2rust_unnamed.zz =
        ((*m1.offset(10 as libc::c_int as isize) as libc::c_int) <<
             0x10 as libc::c_int |
             *m2.offset(10 as libc::c_int as isize) as libc::c_int) as
            libc::c_float * (1 as libc::c_int as libc::c_float / 65536.0f32);
    (*dest).c2rust_unnamed.wz =
        ((*m1.offset(11 as libc::c_int as isize) as libc::c_int) <<
             0x10 as libc::c_int |
             *m2.offset(11 as libc::c_int as isize) as libc::c_int) as
            libc::c_float * (1 as libc::c_int as libc::c_float / 65536.0f32);
    (*dest).c2rust_unnamed.xw =
        ((*m1.offset(12 as libc::c_int as isize) as libc::c_int) <<
             0x10 as libc::c_int |
             *m2.offset(12 as libc::c_int as isize) as libc::c_int) as
            libc::c_float * (1 as libc::c_int as libc::c_float / 65536.0f32);
    (*dest).c2rust_unnamed.yw =
        ((*m1.offset(13 as libc::c_int as isize) as libc::c_int) <<
             0x10 as libc::c_int |
             *m2.offset(13 as libc::c_int as isize) as libc::c_int) as
            libc::c_float * (1 as libc::c_int as libc::c_float / 65536.0f32);
    (*dest).c2rust_unnamed.zw =
        ((*m1.offset(14 as libc::c_int as isize) as libc::c_int) <<
             0x10 as libc::c_int |
             *m2.offset(14 as libc::c_int as isize) as libc::c_int) as
            libc::c_float * (1 as libc::c_int as libc::c_float / 65536.0f32);
    (*dest).c2rust_unnamed.ww =
        ((*m1.offset(15 as libc::c_int as isize) as libc::c_int) <<
             0x10 as libc::c_int |
             *m2.offset(15 as libc::c_int as isize) as libc::c_int) as
            libc::c_float * (1 as libc::c_int as libc::c_float / 65536.0f32);
}
#[no_mangle]
pub unsafe extern "C" fn Matrix_MultVec3fExt(mut src: *mut Vec3f,
                                             mut dest: *mut Vec3f,
                                             mut mf: *mut MtxF) {
    (*dest).x =
        (*mf).c2rust_unnamed.xw +
            ((*mf).c2rust_unnamed.xx * (*src).x +
                 (*mf).c2rust_unnamed.xy * (*src).y +
                 (*mf).c2rust_unnamed.xz * (*src).z);
    (*dest).y =
        (*mf).c2rust_unnamed.yw +
            ((*mf).c2rust_unnamed.yx * (*src).x +
                 (*mf).c2rust_unnamed.yy * (*src).y +
                 (*mf).c2rust_unnamed.yz * (*src).z);
    (*dest).z =
        (*mf).c2rust_unnamed.zw +
            ((*mf).c2rust_unnamed.zx * (*src).x +
                 (*mf).c2rust_unnamed.zy * (*src).y +
                 (*mf).c2rust_unnamed.zz * (*src).z);
}
#[no_mangle]
pub unsafe extern "C" fn Matrix_Transpose(mut mf: *mut MtxF) {
    let mut temp: f32_0 = 0.;
    temp = (*mf).c2rust_unnamed.yx;
    (*mf).c2rust_unnamed.yx = (*mf).c2rust_unnamed.xy;
    (*mf).c2rust_unnamed.xy = temp;
    temp = (*mf).c2rust_unnamed.zx;
    (*mf).c2rust_unnamed.zx = (*mf).c2rust_unnamed.xz;
    (*mf).c2rust_unnamed.xz = temp;
    temp = (*mf).c2rust_unnamed.zy;
    (*mf).c2rust_unnamed.zy = (*mf).c2rust_unnamed.yz;
    (*mf).c2rust_unnamed.yz = temp;
}
#[no_mangle]
pub unsafe extern "C" fn func_800D1FD4(mut mf: *mut MtxF) {
    let mut cmf: *mut MtxF = sCurrentMatrix;
    let mut temp: f32_0 = 0.;
    let mut temp2: f32_0 = 0.;
    let mut temp3: f32_0 = 0.;
    temp = (*cmf).c2rust_unnamed.xx;
    temp *= temp;
    temp2 = (*cmf).c2rust_unnamed.yx;
    temp += temp2 * temp2;
    temp2 = (*cmf).c2rust_unnamed.zx;
    temp += temp2 * temp2;
    temp3 = sqrtf(temp);
    (*cmf).c2rust_unnamed.xx = (*mf).c2rust_unnamed.xx * temp3;
    (*cmf).c2rust_unnamed.yx = (*mf).c2rust_unnamed.yx * temp3;
    (*cmf).c2rust_unnamed.zx = (*mf).c2rust_unnamed.zx * temp3;
    temp = (*cmf).c2rust_unnamed.xy;
    temp *= temp;
    temp2 = (*cmf).c2rust_unnamed.yy;
    temp += temp2 * temp2;
    temp2 = (*cmf).c2rust_unnamed.zy;
    temp += temp2 * temp2;
    temp3 = sqrtf(temp);
    (*cmf).c2rust_unnamed.xy = (*mf).c2rust_unnamed.xy * temp3;
    (*cmf).c2rust_unnamed.yy = (*mf).c2rust_unnamed.yy * temp3;
    (*cmf).c2rust_unnamed.zy = (*mf).c2rust_unnamed.zy * temp3;
    temp = (*cmf).c2rust_unnamed.xz;
    temp *= temp;
    temp2 = (*cmf).c2rust_unnamed.yz;
    temp += temp2 * temp2;
    temp2 = (*cmf).c2rust_unnamed.zz;
    temp += temp2 * temp2;
    temp3 = sqrtf(temp);
    (*cmf).c2rust_unnamed.xz = (*mf).c2rust_unnamed.xz * temp3;
    (*cmf).c2rust_unnamed.yz = (*mf).c2rust_unnamed.yz * temp3;
    (*cmf).c2rust_unnamed.zz = (*mf).c2rust_unnamed.zz * temp3;
}
/* *
 * Gets the rotation the specified matrix represents, using Tait-Bryan YXZ angles.
 * The flag value doesn't matter for a rotation matrix. Not 0 does extra calculation.
 */
#[no_mangle]
pub unsafe extern "C" fn Matrix_MtxFToYXZRotS(mut mf: *mut MtxF,
                                              mut rotDest: *mut Vec3s,
                                              mut flag: s32) {
    let mut temp: f32_0 = 0.;
    let mut temp2: f32_0 = 0.;
    let mut temp3: f32_0 = 0.;
    let mut temp4: f32_0 = 0.;
    temp = (*mf).c2rust_unnamed.xz;
    temp *= temp;
    temp += (*mf).c2rust_unnamed.zz * (*mf).c2rust_unnamed.zz;
    (*rotDest).x =
        (Math_FAtan2F(-(*mf).c2rust_unnamed.yz, sqrtf(temp)) *
             (0x8000 as libc::c_int as libc::c_float /
                  3.14159265358979323846f32)) as s16;
    if (*rotDest).x as libc::c_int == 0x4000 as libc::c_int ||
           (*rotDest).x as libc::c_int == -(0x4000 as libc::c_int) {
        (*rotDest).z = 0 as libc::c_int as s16;
        (*rotDest).y =
            (Math_FAtan2F(-(*mf).c2rust_unnamed.zx, (*mf).c2rust_unnamed.xx) *
                 (0x8000 as libc::c_int as libc::c_float /
                      3.14159265358979323846f32)) as s16
    } else {
        (*rotDest).y =
            (Math_FAtan2F((*mf).c2rust_unnamed.xz, (*mf).c2rust_unnamed.zz) *
                 (0x8000 as libc::c_int as libc::c_float /
                      3.14159265358979323846f32)) as s16;
        if flag == 0 {
            (*rotDest).z =
                (Math_FAtan2F((*mf).c2rust_unnamed.yx,
                              (*mf).c2rust_unnamed.yy) *
                     (0x8000 as libc::c_int as libc::c_float /
                          3.14159265358979323846f32)) as s16
        } else {
            temp = (*mf).c2rust_unnamed.xx;
            temp2 = (*mf).c2rust_unnamed.zx;
            temp3 = (*mf).c2rust_unnamed.zy;
            temp *= temp;
            temp += temp2 * temp2;
            temp2 = (*mf).c2rust_unnamed.yx;
            temp += temp2 * temp2;
            /* temp = xx^2+zx^2+yx^2 == 1 for a rotation matrix */
            temp = sqrtf(temp);
            temp = temp2 / temp;
            temp2 = (*mf).c2rust_unnamed.xy;
            temp2 *= temp2;
            temp2 += temp3 * temp3;
            temp3 = (*mf).c2rust_unnamed.yy;
            temp2 += temp3 * temp3;
            /* temp2 = xy^2+zy^2+yy^2 == 1 for a rotation matrix */
            temp2 = sqrtf(temp2);
            temp2 = temp3 / temp2;
            /* for a rotation matrix, temp == yx and temp2 == yy
             * which is the same as in the !flag branch */
            (*rotDest).z =
                (Math_FAtan2F(temp, temp2) *
                     (0x8000 as libc::c_int as libc::c_float /
                          3.14159265358979323846f32)) as s16
        }
    };
}
/* *
 * Gets the rotation the specified matrix represents, using Tait-Bryan ZYX angles.
 * The flag value doesn't matter for a rotation matrix. Not 0 does extra calculation.
 */
#[no_mangle]
pub unsafe extern "C" fn Matrix_MtxFToZYXRotS(mut mf: *mut MtxF,
                                              mut rotDest: *mut Vec3s,
                                              mut flag: s32) {
    let mut temp: f32_0 = 0.;
    let mut temp2: f32_0 = 0.;
    let mut temp3: f32_0 = 0.;
    let mut temp4: f32_0 = 0.;
    temp = (*mf).c2rust_unnamed.xx;
    temp *= temp;
    temp += (*mf).c2rust_unnamed.yx * (*mf).c2rust_unnamed.yx;
    (*rotDest).y =
        (Math_FAtan2F(-(*mf).c2rust_unnamed.zx, sqrtf(temp)) *
             (0x8000 as libc::c_int as libc::c_float /
                  3.14159265358979323846f32)) as s16;
    if (*rotDest).y as libc::c_int == 0x4000 as libc::c_int ||
           (*rotDest).y as libc::c_int == -(0x4000 as libc::c_int) {
        (*rotDest).x = 0 as libc::c_int as s16;
        (*rotDest).z =
            (Math_FAtan2F(-(*mf).c2rust_unnamed.xy, (*mf).c2rust_unnamed.yy) *
                 (0x8000 as libc::c_int as libc::c_float /
                      3.14159265358979323846f32)) as s16
    } else {
        (*rotDest).z =
            (Math_FAtan2F((*mf).c2rust_unnamed.yx, (*mf).c2rust_unnamed.xx) *
                 (0x8000 as libc::c_int as libc::c_float /
                      3.14159265358979323846f32)) as s16;
        if flag == 0 {
            (*rotDest).x =
                (Math_FAtan2F((*mf).c2rust_unnamed.zy,
                              (*mf).c2rust_unnamed.zz) *
                     (0x8000 as libc::c_int as libc::c_float /
                          3.14159265358979323846f32)) as s16
        } else {
            // see Matrix_MtxFToYXZRotS
            temp = (*mf).c2rust_unnamed.xy;
            temp2 = (*mf).c2rust_unnamed.yy;
            temp3 = (*mf).c2rust_unnamed.yz;
            temp *= temp;
            temp += temp2 * temp2;
            temp2 = (*mf).c2rust_unnamed.zy;
            temp += temp2 * temp2;
            temp = sqrtf(temp);
            temp = temp2 / temp;
            temp2 = (*mf).c2rust_unnamed.xz;
            temp2 *= temp2;
            temp2 += temp3 * temp3;
            temp3 = (*mf).c2rust_unnamed.zz;
            temp2 += temp3 * temp3;
            temp2 = sqrtf(temp2);
            temp2 = temp3 / temp2;
            (*rotDest).x =
                (Math_FAtan2F(temp, temp2) *
                     (0x8000 as libc::c_int as libc::c_float /
                          3.14159265358979323846f32)) as s16
        }
    };
}
/*
 * Rotate the matrix by `f` radians around a unit vector `vec`.
 * NB: vec is assumed to be a unit vector.
 */
#[no_mangle]
pub unsafe extern "C" fn Matrix_RotateAxis(mut f: f32_0, mut vec: *mut Vec3f,
                                           mut mode: u8_0) {
    let mut cmf: *mut MtxF = 0 as *mut MtxF;
    let mut sin: f32_0 = 0.;
    let mut cos: f32_0 = 0.;
    let mut rCos: f32_0 = 0.;
    let mut vrs: f32_0 = 0.;
    let mut temp1: f32_0 = 0.;
    let mut temp2: f32_0 = 0.;
    let mut temp3: f32_0 = 0.;
    let mut temp4: f32_0 = 0.;
    let mut temp5: f32_0 = 0.;
    if mode as libc::c_int == MTXMODE_APPLY as libc::c_int {
        if f != 0 as libc::c_int as libc::c_float {
            cmf = sCurrentMatrix;
            sin = sinf(f);
            cos = cosf(f);
            temp2 = (*cmf).c2rust_unnamed.xy;
            temp3 = (*cmf).c2rust_unnamed.xz;
            temp1 = (*cmf).c2rust_unnamed.xx;
            temp4 =
                ((*vec).x * temp1 + (*vec).y * temp2 + (*vec).z * temp3) *
                    (1.0f32 - cos);
            (*cmf).c2rust_unnamed.xx =
                temp1 * cos + (*vec).x * temp4 +
                    sin * (temp2 * (*vec).z - temp3 * (*vec).y);
            (*cmf).c2rust_unnamed.xy =
                temp2 * cos + (*vec).y * temp4 +
                    sin * (temp3 * (*vec).x - temp1 * (*vec).z);
            (*cmf).c2rust_unnamed.xz =
                temp3 * cos + (*vec).z * temp4 +
                    sin * (temp1 * (*vec).y - temp2 * (*vec).x);
            temp1 = (*cmf).c2rust_unnamed.yx;
            temp2 = (*cmf).c2rust_unnamed.yy;
            temp3 = (*cmf).c2rust_unnamed.yz;
            temp4 =
                ((*vec).x * temp1 + (*vec).y * temp2 + (*vec).z * temp3) *
                    (1.0f32 - cos);
            (*cmf).c2rust_unnamed.yx =
                temp1 * cos + (*vec).x * temp4 +
                    sin * (temp2 * (*vec).z - temp3 * (*vec).y);
            (*cmf).c2rust_unnamed.yy =
                temp2 * cos + (*vec).y * temp4 +
                    sin * (temp3 * (*vec).x - temp1 * (*vec).z);
            (*cmf).c2rust_unnamed.yz =
                temp3 * cos + (*vec).z * temp4 +
                    sin * (temp1 * (*vec).y - temp2 * (*vec).x);
            temp1 = (*cmf).c2rust_unnamed.zx;
            temp2 = (*cmf).c2rust_unnamed.zy;
            temp3 = (*cmf).c2rust_unnamed.zz;
            temp4 =
                ((*vec).x * temp1 + (*vec).y * temp2 + (*vec).z * temp3) *
                    (1.0f32 - cos);
            (*cmf).c2rust_unnamed.zx =
                temp1 * cos + (*vec).x * temp4 +
                    sin * (temp2 * (*vec).z - temp3 * (*vec).y);
            (*cmf).c2rust_unnamed.zy =
                temp2 * cos + (*vec).y * temp4 +
                    sin * (temp3 * (*vec).x - temp1 * (*vec).z);
            (*cmf).c2rust_unnamed.zz =
                temp3 * cos + (*vec).z * temp4 +
                    sin * (temp1 * (*vec).y - temp2 * (*vec).x)
        }
    } else {
        cmf = sCurrentMatrix;
        if f != 0 as libc::c_int as libc::c_float {
            sin = sinf(f);
            cos = cosf(f);
            rCos = 1.0f32 - cos;
            (*cmf).c2rust_unnamed.xx = (*vec).x * (*vec).x * rCos + cos;
            (*cmf).c2rust_unnamed.yy = (*vec).y * (*vec).y * rCos + cos;
            (*cmf).c2rust_unnamed.zz = (*vec).z * (*vec).z * rCos + cos;
            temp2 = (*vec).x * rCos * (*vec).y;
            temp3 = (*vec).z * sin;
            (*cmf).c2rust_unnamed.yx = temp2 + temp3;
            (*cmf).c2rust_unnamed.xy = temp2 - temp3;
            temp2 = (*vec).x * rCos * (*vec).z;
            temp3 = (*vec).y * sin;
            (*cmf).c2rust_unnamed.zx = temp2 - temp3;
            (*cmf).c2rust_unnamed.xz = temp2 + temp3;
            temp2 = (*vec).y * rCos * (*vec).z;
            temp3 = (*vec).x * sin;
            (*cmf).c2rust_unnamed.zy = temp2 + temp3;
            (*cmf).c2rust_unnamed.yz = temp2 - temp3;
            (*cmf).c2rust_unnamed.zw = 0.0f32;
            (*cmf).c2rust_unnamed.yw = (*cmf).c2rust_unnamed.zw;
            (*cmf).c2rust_unnamed.xw = (*cmf).c2rust_unnamed.yw;
            (*cmf).c2rust_unnamed.wz = (*cmf).c2rust_unnamed.xw;
            (*cmf).c2rust_unnamed.wy = (*cmf).c2rust_unnamed.wz;
            (*cmf).c2rust_unnamed.wx = (*cmf).c2rust_unnamed.wy;
            (*cmf).c2rust_unnamed.ww = 1.0f32
        } else {
            (*cmf).c2rust_unnamed.yx = 0.0f32;
            (*cmf).c2rust_unnamed.zx = 0.0f32;
            (*cmf).c2rust_unnamed.wx = 0.0f32;
            (*cmf).c2rust_unnamed.xy = 0.0f32;
            (*cmf).c2rust_unnamed.zy = 0.0f32;
            (*cmf).c2rust_unnamed.wy = 0.0f32;
            (*cmf).c2rust_unnamed.xz = 0.0f32;
            (*cmf).c2rust_unnamed.yz = 0.0f32;
            (*cmf).c2rust_unnamed.wz = 0.0f32;
            (*cmf).c2rust_unnamed.xw = 0.0f32;
            (*cmf).c2rust_unnamed.yw = 0.0f32;
            (*cmf).c2rust_unnamed.zw = 0.0f32;
            (*cmf).c2rust_unnamed.xx = 1.0f32;
            (*cmf).c2rust_unnamed.yy = 1.0f32;
            (*cmf).c2rust_unnamed.zz = 1.0f32;
            (*cmf).c2rust_unnamed.ww = 1.0f32
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn Matrix_CheckFloats(mut mf: *mut MtxF,
                                            mut file: *mut libc::c_char,
                                            mut line: s32) -> *mut MtxF {
    let mut i: s32 = 0;
    let mut j: s32 = 0;
    i = 0 as libc::c_int;
    while i < 4 as libc::c_int {
        j = 0 as libc::c_int;
        while j < 4 as libc::c_int {
            if !(-32768.0f32 <= (*mf).mf[i as usize][j as usize]) ||
                   !((*mf).mf[i as usize][j as usize] < 32768.0f32) {
                osSyncPrintf(b"%s %d: [%s] =\n/ %12.6f %12.6f %12.6f %12.6f \\\n| %12.6f %12.6f %12.6f %12.6f |\n| %12.6f %12.6f %12.6f %12.6f |\n\\ %12.6f %12.6f %12.6f %12.6f /\n\x00"
                                 as *const u8 as *const libc::c_char, file,
                             line,
                             b"mf\x00" as *const u8 as *const libc::c_char,
                             (*mf).c2rust_unnamed.xx as libc::c_double,
                             (*mf).c2rust_unnamed.xy as libc::c_double,
                             (*mf).c2rust_unnamed.xz as libc::c_double,
                             (*mf).c2rust_unnamed.xw as libc::c_double,
                             (*mf).c2rust_unnamed.yx as libc::c_double,
                             (*mf).c2rust_unnamed.yy as libc::c_double,
                             (*mf).c2rust_unnamed.yz as libc::c_double,
                             (*mf).c2rust_unnamed.yw as libc::c_double,
                             (*mf).c2rust_unnamed.zx as libc::c_double,
                             (*mf).c2rust_unnamed.zy as libc::c_double,
                             (*mf).c2rust_unnamed.zz as libc::c_double,
                             (*mf).c2rust_unnamed.zw as libc::c_double,
                             (*mf).c2rust_unnamed.wx as libc::c_double,
                             (*mf).c2rust_unnamed.wy as libc::c_double,
                             (*mf).c2rust_unnamed.wz as libc::c_double,
                             (*mf).c2rust_unnamed.ww as libc::c_double);
                Fault_AddHungupAndCrash(file, line as u32_0);
            }
            j += 1
        }
        i += 1
    }
    return mf;
}
#[no_mangle]
pub unsafe extern "C" fn func_800D2A34(mut mf: *mut MtxF, mut arg1: f32_0,
                                       mut arg2: f32_0, mut arg3: f32_0,
                                       mut arg4: f32_0) {
    (*mf).c2rust_unnamed.yx = 0.0f32;
    (*mf).c2rust_unnamed.zx = 0.0f32;
    (*mf).c2rust_unnamed.wx = 0.0f32;
    (*mf).c2rust_unnamed.xy = 0.0f32;
    (*mf).c2rust_unnamed.zy = 0.0f32;
    (*mf).c2rust_unnamed.wy = 0.0f32;
    (*mf).c2rust_unnamed.xz = 0.0f32;
    (*mf).c2rust_unnamed.yz = 0.0f32;
    (*mf).c2rust_unnamed.wz = 0.0f32;
    (*mf).c2rust_unnamed.xx = arg1;
    (*mf).c2rust_unnamed.yy = arg1;
    (*mf).c2rust_unnamed.zz = arg1;
    (*mf).c2rust_unnamed.xw = arg2;
    (*mf).c2rust_unnamed.yw = arg3;
    (*mf).c2rust_unnamed.zw = arg4;
    (*mf).c2rust_unnamed.ww = 1.0f32;
}
#[no_mangle]
pub unsafe extern "C" fn func_800D2A98(mut mtx: *mut Mtx, mut arg1: f32_0,
                                       mut arg2: f32_0, mut arg3: f32_0,
                                       mut arg4: f32_0) {
    let mut mf: MtxF = MtxF{mf: [[0.; 4]; 4],};
    func_800D2A34(&mut mf, arg1, arg2, arg3, arg4);
    guMtxF2L(&mut mf, mtx);
}
#[no_mangle]
pub unsafe extern "C" fn func_800D2AE4(mut mtx: *mut Mtx, mut arg1: f32_0,
                                       mut arg2: f32_0, mut arg3: f32_0,
                                       mut arg4: f32_0) {
    let mut m1: *mut u16_0 =
        &mut *(*(*mtx).m.as_mut_ptr().offset(0 as libc::c_int as
                                                 isize)).as_mut_ptr().offset(0
                                                                                 as
                                                                                 libc::c_int
                                                                                 as
                                                                                 isize)
            as *mut libc::c_long as *mut u16_0;
    let mut m2: *mut u16_0 =
        &mut *(*(*mtx).m.as_mut_ptr().offset(2 as libc::c_int as
                                                 isize)).as_mut_ptr().offset(0
                                                                                 as
                                                                                 libc::c_int
                                                                                 as
                                                                                 isize)
            as *mut libc::c_long as *mut u16_0;
    let mut temp: u32_0 = 0;
    temp = (arg1 * 65536.0f32) as s32 as u32_0;
    *m2.offset(0 as libc::c_int as isize) =
        (temp & 0xffff as libc::c_int as libc::c_uint) as u16_0;
    *m1.offset(0 as libc::c_int as isize) =
        (temp >> 16 as libc::c_int & 0xffff as libc::c_int as libc::c_uint) as
            u16_0;
    temp = (arg1 * 65536.0f32) as s32 as u32_0;
    *m1.offset(5 as libc::c_int as isize) =
        (temp >> 16 as libc::c_int & 0xffff as libc::c_int as libc::c_uint) as
            u16_0;
    *m2.offset(5 as libc::c_int as isize) =
        (temp & 0xffff as libc::c_int as libc::c_uint) as u16_0;
    temp = (arg1 * 65536.0f32) as s32 as u32_0;
    *m1.offset(10 as libc::c_int as isize) =
        (temp >> 16 as libc::c_int & 0xffff as libc::c_int as libc::c_uint) as
            u16_0;
    *m2.offset(10 as libc::c_int as isize) =
        (temp & 0xffff as libc::c_int as libc::c_uint) as u16_0;
    temp = (arg2 * 65536.0f32) as s32 as u32_0;
    *m1.offset(12 as libc::c_int as isize) =
        (temp >> 16 as libc::c_int & 0xffff as libc::c_int as libc::c_uint) as
            u16_0;
    *m2.offset(12 as libc::c_int as isize) =
        (temp & 0xffff as libc::c_int as libc::c_uint) as u16_0;
    temp = (arg3 * 65536.0f32) as s32 as u32_0;
    *m1.offset(13 as libc::c_int as isize) =
        (temp >> 16 as libc::c_int & 0xffff as libc::c_int as libc::c_uint) as
            u16_0;
    *m2.offset(13 as libc::c_int as isize) =
        (temp & 0xffff as libc::c_int as libc::c_uint) as u16_0;
    temp = (arg4 * 65536.0f32) as s32 as u32_0;
    *m1.offset(14 as libc::c_int as isize) =
        (temp >> 16 as libc::c_int & 0xffff as libc::c_int as libc::c_uint) as
            u16_0;
    *m2.offset(14 as libc::c_int as isize) =
        (temp & 0xffff as libc::c_int as libc::c_uint) as u16_0;
    *m1.offset(1 as libc::c_int as isize) = 0 as libc::c_int as u16_0;
    *m1.offset(2 as libc::c_int as isize) = 0 as libc::c_int as u16_0;
    *m1.offset(3 as libc::c_int as isize) = 0 as libc::c_int as u16_0;
    *m1.offset(4 as libc::c_int as isize) = 0 as libc::c_int as u16_0;
    *m1.offset(6 as libc::c_int as isize) = 0 as libc::c_int as u16_0;
    *m1.offset(7 as libc::c_int as isize) = 0 as libc::c_int as u16_0;
    *m1.offset(8 as libc::c_int as isize) = 0 as libc::c_int as u16_0;
    *m1.offset(9 as libc::c_int as isize) = 0 as libc::c_int as u16_0;
    *m1.offset(11 as libc::c_int as isize) = 0 as libc::c_int as u16_0;
    *m1.offset(15 as libc::c_int as isize) = 1 as libc::c_int as u16_0;
    *m2.offset(1 as libc::c_int as isize) = 0 as libc::c_int as u16_0;
    *m2.offset(2 as libc::c_int as isize) = 0 as libc::c_int as u16_0;
    *m2.offset(3 as libc::c_int as isize) = 0 as libc::c_int as u16_0;
    *m2.offset(4 as libc::c_int as isize) = 0 as libc::c_int as u16_0;
    *m2.offset(6 as libc::c_int as isize) = 0 as libc::c_int as u16_0;
    *m2.offset(7 as libc::c_int as isize) = 0 as libc::c_int as u16_0;
    *m2.offset(8 as libc::c_int as isize) = 0 as libc::c_int as u16_0;
    *m2.offset(9 as libc::c_int as isize) = 0 as libc::c_int as u16_0;
    *m2.offset(11 as libc::c_int as isize) = 0 as libc::c_int as u16_0;
    *m2.offset(15 as libc::c_int as isize) = 0 as libc::c_int as u16_0;
}
#[no_mangle]
pub unsafe extern "C" fn func_800D2BD0(mut mtx: *mut Mtx, mut arg1: f32_0,
                                       mut arg2: f32_0, mut arg3: f32_0,
                                       mut arg4: f32_0, mut arg5: f32_0,
                                       mut arg6: f32_0) {
    let mut m1: *mut u16_0 =
        &mut *(*(*mtx).m.as_mut_ptr().offset(0 as libc::c_int as
                                                 isize)).as_mut_ptr().offset(0
                                                                                 as
                                                                                 libc::c_int
                                                                                 as
                                                                                 isize)
            as *mut libc::c_long as *mut u16_0;
    let mut m2: *mut u16_0 =
        &mut *(*(*mtx).m.as_mut_ptr().offset(2 as libc::c_int as
                                                 isize)).as_mut_ptr().offset(0
                                                                                 as
                                                                                 libc::c_int
                                                                                 as
                                                                                 isize)
            as *mut libc::c_long as *mut u16_0;
    let mut temp: u32_0 = 0;
    temp = (arg1 * 65536.0f32) as s32 as u32_0;
    *m1.offset(0 as libc::c_int as isize) =
        (temp >> 16 as libc::c_int & 0xffff as libc::c_int as libc::c_uint) as
            u16_0;
    *m2.offset(0 as libc::c_int as isize) =
        (temp & 0xffff as libc::c_int as libc::c_uint) as u16_0;
    temp = (arg2 * 65536.0f32) as s32 as u32_0;
    *m1.offset(5 as libc::c_int as isize) =
        (temp >> 16 as libc::c_int & 0xffff as libc::c_int as libc::c_uint) as
            u16_0;
    *m2.offset(5 as libc::c_int as isize) =
        (temp & 0xffff as libc::c_int as libc::c_uint) as u16_0;
    temp = (arg3 * 65536.0f32) as s32 as u32_0;
    *m1.offset(10 as libc::c_int as isize) =
        (temp >> 16 as libc::c_int & 0xffff as libc::c_int as libc::c_uint) as
            u16_0;
    *m2.offset(10 as libc::c_int as isize) =
        (temp & 0xffff as libc::c_int as libc::c_uint) as u16_0;
    temp = (arg4 * 65536.0f32) as s32 as u32_0;
    *m1.offset(12 as libc::c_int as isize) =
        (temp >> 16 as libc::c_int & 0xffff as libc::c_int as libc::c_uint) as
            u16_0;
    *m2.offset(12 as libc::c_int as isize) =
        (temp & 0xffff as libc::c_int as libc::c_uint) as u16_0;
    temp = (arg5 * 65536.0f32) as s32 as u32_0;
    *m1.offset(13 as libc::c_int as isize) =
        (temp >> 16 as libc::c_int & 0xffff as libc::c_int as libc::c_uint) as
            u16_0;
    *m2.offset(13 as libc::c_int as isize) =
        (temp & 0xffff as libc::c_int as libc::c_uint) as u16_0;
    temp = (arg6 * 65536.0f32) as s32 as u32_0;
    *m1.offset(14 as libc::c_int as isize) =
        (temp >> 16 as libc::c_int & 0xffff as libc::c_int as libc::c_uint) as
            u16_0;
    *m2.offset(14 as libc::c_int as isize) =
        (temp & 0xffff as libc::c_int as libc::c_uint) as u16_0;
    *m1.offset(1 as libc::c_int as isize) = 0 as libc::c_int as u16_0;
    *m1.offset(2 as libc::c_int as isize) = 0 as libc::c_int as u16_0;
    *m1.offset(3 as libc::c_int as isize) = 0 as libc::c_int as u16_0;
    *m1.offset(4 as libc::c_int as isize) = 0 as libc::c_int as u16_0;
    *m1.offset(6 as libc::c_int as isize) = 0 as libc::c_int as u16_0;
    *m1.offset(7 as libc::c_int as isize) = 0 as libc::c_int as u16_0;
    *m1.offset(8 as libc::c_int as isize) = 0 as libc::c_int as u16_0;
    *m1.offset(9 as libc::c_int as isize) = 0 as libc::c_int as u16_0;
    *m1.offset(11 as libc::c_int as isize) = 0 as libc::c_int as u16_0;
    *m1.offset(15 as libc::c_int as isize) = 1 as libc::c_int as u16_0;
    *m2.offset(1 as libc::c_int as isize) = 0 as libc::c_int as u16_0;
    *m2.offset(2 as libc::c_int as isize) = 0 as libc::c_int as u16_0;
    *m2.offset(3 as libc::c_int as isize) = 0 as libc::c_int as u16_0;
    *m2.offset(4 as libc::c_int as isize) = 0 as libc::c_int as u16_0;
    *m2.offset(6 as libc::c_int as isize) = 0 as libc::c_int as u16_0;
    *m2.offset(7 as libc::c_int as isize) = 0 as libc::c_int as u16_0;
    *m2.offset(8 as libc::c_int as isize) = 0 as libc::c_int as u16_0;
    *m2.offset(9 as libc::c_int as isize) = 0 as libc::c_int as u16_0;
    *m2.offset(11 as libc::c_int as isize) = 0 as libc::c_int as u16_0;
    *m2.offset(15 as libc::c_int as isize) = 0 as libc::c_int as u16_0;
}
#[no_mangle]
pub unsafe extern "C" fn func_800D2CEC(mut mtx: *mut Mtx, mut arg1: f32_0,
                                       mut arg2: f32_0, mut arg3: f32_0,
                                       mut arg4: f32_0, mut arg5: f32_0,
                                       mut arg6: f32_0) {
    let mut m: *mut Mtx_t = &mut (*mtx).m;
    let mut m1: *mut u16_0 =
        (*m)[0 as libc::c_int as usize].as_mut_ptr() as *mut u16_0;
    let mut m2: *mut u16_0 =
        (*m)[2 as libc::c_int as usize].as_mut_ptr() as *mut u16_0;
    let mut temp: u32_0 = 0;
    (*m)[0 as libc::c_int as usize][1 as libc::c_int as usize] =
        0 as libc::c_int as libc::c_long;
    (*m)[2 as libc::c_int as usize][1 as libc::c_int as usize] =
        0 as libc::c_int as libc::c_long;
    (*m)[0 as libc::c_int as usize][3 as libc::c_int as usize] =
        0 as libc::c_int as libc::c_long;
    (*m)[2 as libc::c_int as usize][3 as libc::c_int as usize] =
        0 as libc::c_int as libc::c_long;
    (*m)[0 as libc::c_int as usize][4 as libc::c_int as usize] =
        0 as libc::c_int as libc::c_long;
    temp = (arg1 * 65536.0f32) as s32 as u32_0;
    (*m)[0 as libc::c_int as usize][0 as libc::c_int as usize] =
        temp as libc::c_long;
    *m1.offset(1 as libc::c_int as isize) = 0 as libc::c_int as u16_0;
    (*m)[2 as libc::c_int as usize][0 as libc::c_int as usize] =
        (temp << 16 as libc::c_int) as libc::c_long;
    temp = (arg2 * 65536.0f32) as s32 as u32_0;
    (*m)[0 as libc::c_int as usize][2 as libc::c_int as usize] =
        (temp >> 16 as libc::c_int) as libc::c_long;
    (*m)[2 as libc::c_int as usize][2 as libc::c_int as usize] =
        (temp & 0xffff as libc::c_int as libc::c_uint) as libc::c_long;
    temp = (arg3 * 65536.0f32) as s32 as u32_0;
    (*m)[1 as libc::c_int as usize][1 as libc::c_int as usize] =
        temp as libc::c_long;
    *m1.offset(11 as libc::c_int as isize) = 0 as libc::c_int as u16_0;
    (*m)[3 as libc::c_int as usize][1 as libc::c_int as usize] =
        (temp << 16 as libc::c_int) as libc::c_long;
    (*m)[2 as libc::c_int as usize][4 as libc::c_int as usize] =
        0 as libc::c_int as libc::c_long;
    temp = (arg4 * 65536.0f32) as s32 as u32_0;
    *m1.offset(12 as libc::c_int as isize) =
        (temp >> 16 as libc::c_int & 0xffff as libc::c_int as libc::c_uint) as
            u16_0;
    *m2.offset(12 as libc::c_int as isize) =
        (temp & 0xffff as libc::c_int as libc::c_uint) as u16_0;
    temp = (arg5 * 65536.0f32) as s32 as u32_0;
    *m1.offset(13 as libc::c_int as isize) =
        (temp >> 16 as libc::c_int & 0xffff as libc::c_int as libc::c_uint) as
            u16_0;
    *m2.offset(13 as libc::c_int as isize) =
        (temp & 0xffff as libc::c_int as libc::c_uint) as u16_0;
    temp = (arg6 * 65536.0f32) as s32 as u32_0;
    *m1.offset(14 as libc::c_int as isize) =
        (temp >> 16 as libc::c_int & 0xffff as libc::c_int as libc::c_uint) as
            u16_0;
    *m1.offset(15 as libc::c_int as isize) = 1 as libc::c_int as u16_0;
    (*m)[3 as libc::c_int as usize][3 as libc::c_int as usize] =
        (temp << 16 as libc::c_int) as libc::c_long;
}
