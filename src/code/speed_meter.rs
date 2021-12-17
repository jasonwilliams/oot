#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case,
         non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(const_raw_ptr_to_usize_cast, register_tool)]
extern "C" {
    #[no_mangle]
    fn osSyncPrintf(fmt: *const libc::c_char, _: ...);
    #[no_mangle]
    fn LogUtils_CheckNullPointer(exp: *const libc::c_char,
                                 ptr: *mut libc::c_void,
                                 file: *const libc::c_char, line: s32);
    #[no_mangle]
    fn LogUtils_LogThreadId(name: *const libc::c_char, line: s32);
    #[no_mangle]
    fn ZeldaArena_GetSizes(outMaxFree: *mut u32_0, outFree: *mut u32_0,
                           outAlloc: *mut u32_0);
    #[no_mangle]
    fn ZeldaArena_IsInitalized() -> u8_0;
    #[no_mangle]
    fn View_Init(_: *mut View, _: *mut GraphicsContext);
    #[no_mangle]
    fn View_SetViewport(view: *mut View, viewport: *mut Viewport);
    #[no_mangle]
    fn func_800AB9EC(view: *mut View, arg1: s32, p: *mut *mut Gfx) -> s32;
    #[no_mangle]
    fn THGA_GetSize(thga: *mut TwoHeadGfxArena) -> s32;
    #[no_mangle]
    fn THA_GetSize(tha: *mut TwoHeadArena) -> s32;
    #[no_mangle]
    fn Graph_OpenDisps(dispRefs: *mut *mut Gfx, gfxCtx: *mut GraphicsContext,
                       file: *const libc::c_char, line: s32);
    #[no_mangle]
    fn Graph_CloseDisps(dispRefs: *mut *mut Gfx, gfxCtx: *mut GraphicsContext,
                        file: *const libc::c_char, line: s32);
    #[no_mangle]
    fn SystemArena_GetSizes(outMaxFree: *mut u32_0, outFree: *mut u32_0,
                            outAlloc: *mut u32_0);
    #[no_mangle]
    static mut gIrqMgrRetraceTime: OSTime;
    #[no_mangle]
    static mut gGameInfo: *mut GameInfo;
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
pub type OSPri = s32;
pub type OSId = s32;
#[derive(Copy, Clone)]
#[repr(C)]
pub union __OSfp {
    pub f: C2RustUnnamed_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_0 {
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
pub type OSTime = u64_0;
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
pub struct Input {
    pub cur: OSContPad,
    pub prev: OSContPad,
    pub press: OSContPad,
    pub rel: OSContPad,
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SpeedMeter {
    pub unk_00: [libc::c_char; 24],
    pub unk_18: s32,
    pub y: s32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SpeedMeterAllocEntry {
    pub maxval: s32,
    pub val: s32,
    pub backColor: u16_0,
    pub foreColor: u16_0,
    pub ulx: s32,
    pub lrx: s32,
    pub uly: s32,
    pub lry: s32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SpeedMeterTimeEntry {
    pub time: *mut OSTime,
    pub x: u8_0,
    pub y: u8_0,
    pub color: u16_0,
}
#[no_mangle]
pub static mut D_8016A520: OSTime = 0;
#[no_mangle]
pub static mut D_8016A528: OSTime = 0;
#[no_mangle]
pub static mut D_8016A530: OSTime = 0;
#[no_mangle]
pub static mut D_8016A538: OSTime = 0;
#[no_mangle]
pub static mut D_8016A540: OSTime = 0;
#[no_mangle]
pub static mut D_8016A548: OSTime = 0;
#[no_mangle]
pub static mut D_8016A550: OSTime = 0;
#[no_mangle]
pub static mut D_8016A558: OSTime = 0;
#[no_mangle]
pub static mut gRSPAudioTotalTime: OSTime = 0;
#[no_mangle]
pub static mut gRSPGFXTotalTime: OSTime = 0;
#[no_mangle]
pub static mut gRSPOtherTotalTime: OSTime = 0;
#[no_mangle]
pub static mut D_8016A578: OSTime = 0;
#[no_mangle]
pub static mut gRDPTotalTime: OSTime = 0;
#[no_mangle]
pub static mut sSpeedMeterTimeEntryPtr: *mut SpeedMeterTimeEntry =
    0 as *const SpeedMeterTimeEntry as *mut SpeedMeterTimeEntry;
#[no_mangle]
pub static mut sSpeedMeterTimeEntryArray: [SpeedMeterTimeEntry; 6] =
    unsafe {
        [{
             let mut init =
                 SpeedMeterTimeEntry{time:
                                         &D_8016A520 as *const OSTime as
                                             *mut OSTime,
                                     x: 0 as libc::c_int as u8_0,
                                     y: 0 as libc::c_int as u8_0,
                                     color:
                                         ((255 as libc::c_int) <<
                                              8 as libc::c_int &
                                              0xf800 as libc::c_int |
                                              (0 as libc::c_int) <<
                                                  3 as libc::c_int &
                                                  0x7c0 as libc::c_int |
                                              0 as libc::c_int >>
                                                  2 as libc::c_int &
                                                  0x3e as libc::c_int |
                                              1 as libc::c_int &
                                                  0x1 as libc::c_int) as
                                             u16_0,};
             init
         },
         {
             let mut init =
                 SpeedMeterTimeEntry{time:
                                         &D_8016A528 as *const OSTime as
                                             *mut OSTime,
                                     x: 0 as libc::c_int as u8_0,
                                     y: 2 as libc::c_int as u8_0,
                                     color:
                                         ((255 as libc::c_int) <<
                                              8 as libc::c_int &
                                              0xf800 as libc::c_int |
                                              (255 as libc::c_int) <<
                                                  3 as libc::c_int &
                                                  0x7c0 as libc::c_int |
                                              0 as libc::c_int >>
                                                  2 as libc::c_int &
                                                  0x3e as libc::c_int |
                                              1 as libc::c_int &
                                                  0x1 as libc::c_int) as
                                             u16_0,};
             init
         },
         {
             let mut init =
                 SpeedMeterTimeEntry{time:
                                         &D_8016A530 as *const OSTime as
                                             *mut OSTime,
                                     x: 0 as libc::c_int as u8_0,
                                     y: 4 as libc::c_int as u8_0,
                                     color:
                                         ((0 as libc::c_int) <<
                                              8 as libc::c_int &
                                              0xf800 as libc::c_int |
                                              (0 as libc::c_int) <<
                                                  3 as libc::c_int &
                                                  0x7c0 as libc::c_int |
                                              255 as libc::c_int >>
                                                  2 as libc::c_int &
                                                  0x3e as libc::c_int |
                                              1 as libc::c_int &
                                                  0x1 as libc::c_int) as
                                             u16_0,};
             init
         },
         {
             let mut init =
                 SpeedMeterTimeEntry{time:
                                         &D_8016A538 as *const OSTime as
                                             *mut OSTime,
                                     x: 0 as libc::c_int as u8_0,
                                     y: 6 as libc::c_int as u8_0,
                                     color:
                                         ((255 as libc::c_int) <<
                                              8 as libc::c_int &
                                              0xf800 as libc::c_int |
                                              (128 as libc::c_int) <<
                                                  3 as libc::c_int &
                                                  0x7c0 as libc::c_int |
                                              128 as libc::c_int >>
                                                  2 as libc::c_int &
                                                  0x3e as libc::c_int |
                                              1 as libc::c_int &
                                                  0x1 as libc::c_int) as
                                             u16_0,};
             init
         },
         {
             let mut init =
                 SpeedMeterTimeEntry{time:
                                         &D_8016A540 as *const OSTime as
                                             *mut OSTime,
                                     x: 0 as libc::c_int as u8_0,
                                     y: 8 as libc::c_int as u8_0,
                                     color:
                                         ((0 as libc::c_int) <<
                                              8 as libc::c_int &
                                              0xf800 as libc::c_int |
                                              (255 as libc::c_int) <<
                                                  3 as libc::c_int &
                                                  0x7c0 as libc::c_int |
                                              0 as libc::c_int >>
                                                  2 as libc::c_int &
                                                  0x3e as libc::c_int |
                                              1 as libc::c_int &
                                                  0x1 as libc::c_int) as
                                             u16_0,};
             init
         },
         {
             let mut init =
                 SpeedMeterTimeEntry{time:
                                         &D_8016A548 as *const OSTime as
                                             *mut OSTime,
                                     x: 0 as libc::c_int as u8_0,
                                     y: 10 as libc::c_int as u8_0,
                                     color:
                                         ((255 as libc::c_int) <<
                                              8 as libc::c_int &
                                              0xf800 as libc::c_int |
                                              (0 as libc::c_int) <<
                                                  3 as libc::c_int &
                                                  0x7c0 as libc::c_int |
                                              255 as libc::c_int >>
                                                  2 as libc::c_int &
                                                  0x3e as libc::c_int |
                                              1 as libc::c_int &
                                                  0x1 as libc::c_int) as
                                             u16_0,};
             init
         }]
    };
#[no_mangle]
pub unsafe extern "C" fn SpeedMeter_InitImpl(mut this: *mut SpeedMeter,
                                             mut arg1: u32_0, mut y: u32_0) {
    LogUtils_CheckNullPointer(b"this\x00" as *const u8 as *const libc::c_char,
                              this as *mut libc::c_void,
                              b"../speed_meter.c\x00" as *const u8 as
                                  *const libc::c_char, 181 as libc::c_int);
    (*this).unk_18 = arg1 as s32;
    (*this).y = y as s32;
}
#[no_mangle]
pub unsafe extern "C" fn SpeedMeter_Init(mut this: *mut SpeedMeter) {
    SpeedMeter_InitImpl(this, 32 as libc::c_int as u32_0,
                        22 as libc::c_int as u32_0);
}
#[no_mangle]
pub unsafe extern "C" fn SpeedMeter_Destroy(mut this: *mut SpeedMeter) { }
#[no_mangle]
pub unsafe extern "C" fn SpeedMeter_DrawTimeEntries(mut this: *mut SpeedMeter,
                                                    mut gfxCtx:
                                                        *mut GraphicsContext) {
    let mut pad: [s32; 2] = [0; 2];
    let mut baseX: u32_0 = 32 as libc::c_int as u32_0;
    let mut temp: s32 = 0;
    let mut i: s32 = 0;
    let mut uly: s32 = 0;
    let mut lry: s32 = 0;
    let mut view: View =
        View{magic: 0,
             gfxCtx: 0 as *mut GraphicsContext,
             viewport: Viewport{topY: 0, bottomY: 0, leftX: 0, rightX: 0,},
             fovy: 0.,
             zNear: 0.,
             zFar: 0.,
             scale: 0.,
             eye: Vec3f{x: 0., y: 0., z: 0.,},
             lookAt: Vec3f{x: 0., y: 0., z: 0.,},
             up: Vec3f{x: 0., y: 0., z: 0.,},
             vp: Vp{vp: Vp_t{vscale: [0; 4], vtrans: [0; 4],},},
             projection: Mtx{m: [[0; 4]; 4],},
             viewing: Mtx{m: [[0; 4]; 4],},
             projectionPtr: 0 as *mut Mtx,
             viewingPtr: 0 as *mut Mtx,
             unk_E8: Vec3f{x: 0., y: 0., z: 0.,},
             unk_F4: Vec3f{x: 0., y: 0., z: 0.,},
             unk_100: 0.,
             unk_104: Vec3f{x: 0., y: 0., z: 0.,},
             unk_110: Vec3f{x: 0., y: 0., z: 0.,},
             normal: 0,
             flags: 0,
             unk_124: 0,};
    let mut pad2: [u32_0; 3] = [0; 3];
    let mut gfx: *mut Gfx = 0 as *mut Gfx;
    uly = (*this).y;
    lry = (*this).y + 2 as libc::c_int;
    let mut __gfxCtx: *mut GraphicsContext = 0 as *mut GraphicsContext;
    let mut dispRefs: [*mut Gfx; 4] = [0 as *mut Gfx; 4];
    __gfxCtx = gfxCtx;
    Graph_OpenDisps(dispRefs.as_mut_ptr(), gfxCtx,
                    b"../speed_meter.c\x00" as *const u8 as
                        *const libc::c_char, 225 as libc::c_int);
    /* ! @bug if gIrqMgrRetraceTime is 0, CLOSE_DISPS will never be reached */
    if gIrqMgrRetraceTime == 0 as libc::c_int as libc::c_ulonglong { return }
    sSpeedMeterTimeEntryPtr =
        &mut *sSpeedMeterTimeEntryArray.as_mut_ptr().offset(0 as libc::c_int
                                                                as isize) as
            *mut SpeedMeterTimeEntry;
    i = 0 as libc::c_int;
    while i <
              (::std::mem::size_of::<[SpeedMeterTimeEntry; 6]>() as
                   libc::c_ulong).wrapping_div(::std::mem::size_of::<SpeedMeterTimeEntry>()
                                                   as libc::c_ulong) as s32 {
        temp =
            (*(*sSpeedMeterTimeEntryPtr).time as f64_0 /
                 gIrqMgrRetraceTime as libc::c_double * 64.0f64) as s32;
        (*sSpeedMeterTimeEntryPtr).x =
            (temp as libc::c_uint).wrapping_add(baseX) as u8_0;
        sSpeedMeterTimeEntryPtr = sSpeedMeterTimeEntryPtr.offset(1);
        i += 1
    }
    View_Init(&mut view, gfxCtx);
    view.flags = 0xa as libc::c_int;
    let mut viewport: Viewport =
        Viewport{topY: 0, bottomY: 0, leftX: 0, rightX: 0,};
    viewport.bottomY = 240 as libc::c_int;
    viewport.rightX = 320 as libc::c_int;
    viewport.topY = 0 as libc::c_int;
    viewport.leftX = 0 as libc::c_int;
    View_SetViewport(&mut view, &mut viewport);
    gfx = (*__gfxCtx).overlay.p;
    func_800AB9EC(&mut view, 0xf as libc::c_int, &mut gfx);
    let fresh0 = gfx;
    gfx = gfx.offset(1);
    let mut _g: *mut Gfx = fresh0;
    (*_g).words.w0 =
        (0xe7 as libc::c_int as u32_0 &
             (((0x1 as libc::c_int) << 8 as libc::c_int) - 1 as libc::c_int)
                 as libc::c_uint) << 24 as libc::c_int;
    (*_g).words.w1 = 0 as libc::c_int as libc::c_uint;
    let fresh1 = gfx;
    gfx = gfx.offset(1);
    let mut _g_0: *mut Gfx = fresh1;
    (*_g_0).words.w0 =
        (0xef as libc::c_int as u32_0 &
             (((0x1 as libc::c_int) << 8 as libc::c_int) - 1 as libc::c_int)
                 as libc::c_uint) << 24 as libc::c_int |
            (((0 as libc::c_int) << 4 as libc::c_int |
                  (0 as libc::c_int) << 6 as libc::c_int |
                  (0 as libc::c_int) << 8 as libc::c_int |
                  (0 as libc::c_int) << 9 as libc::c_int |
                  (0 as libc::c_int) << 12 as libc::c_int |
                  (0 as libc::c_int) << 14 as libc::c_int |
                  (0 as libc::c_int) << 16 as libc::c_int |
                  (0 as libc::c_int) << 17 as libc::c_int |
                  (0 as libc::c_int) << 19 as libc::c_int |
                  (3 as libc::c_int) << 20 as libc::c_int |
                  (0 as libc::c_int) << 23 as libc::c_int) as u32_0 &
                 (((0x1 as libc::c_int) << 24 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 0 as libc::c_int;
    (*_g_0).words.w1 =
        ((0 as libc::c_int) << 0 as libc::c_int |
             (0 as libc::c_int) << 2 as libc::c_int |
             (0 as libc::c_int) << 30 as libc::c_int |
             (0 as libc::c_int) << 26 as libc::c_int |
             (0 as libc::c_int) << 22 as libc::c_int |
             (0 as libc::c_int) << 18 as libc::c_int |
             (0 as libc::c_int) << 28 as libc::c_int |
             (0 as libc::c_int) << 24 as libc::c_int |
             (0 as libc::c_int) << 20 as libc::c_int |
             (0 as libc::c_int) << 16 as libc::c_int) as libc::c_uint;
    let fresh2 = gfx;
    gfx = gfx.offset(1);
    let mut _g_1: *mut Gfx = fresh2;
    (*_g_1).words.w0 =
        (0xe7 as libc::c_int as u32_0 &
             (((0x1 as libc::c_int) << 8 as libc::c_int) - 1 as libc::c_int)
                 as libc::c_uint) << 24 as libc::c_int;
    (*_g_1).words.w1 = 0 as libc::c_int as libc::c_uint;
    let fresh3 = gfx;
    gfx = gfx.offset(1);
    let mut _g_2: *mut Gfx = fresh3;
    (*_g_2).words.w0 =
        (0xf7 as libc::c_int as u32_0 &
             (((0x1 as libc::c_int) << 8 as libc::c_int) - 1 as libc::c_int)
                 as libc::c_uint) << 24 as libc::c_int;
    (*_g_2).words.w1 =
        (((0 as libc::c_int) << 8 as libc::c_int & 0xf800 as libc::c_int |
              (0 as libc::c_int) << 3 as libc::c_int & 0x7c0 as libc::c_int |
              255 as libc::c_int >> 2 as libc::c_int & 0x3e as libc::c_int |
              1 as libc::c_int & 0x1 as libc::c_int) << 16 as libc::c_int |
             ((0 as libc::c_int) << 8 as libc::c_int & 0xf800 as libc::c_int |
                  (0 as libc::c_int) << 3 as libc::c_int &
                      0x7c0 as libc::c_int |
                  255 as libc::c_int >> 2 as libc::c_int & 0x3e as libc::c_int
                  | 1 as libc::c_int & 0x1 as libc::c_int)) as libc::c_uint;
    let fresh4 = gfx;
    gfx = gfx.offset(1);
    let mut _g_3: *mut Gfx = fresh4;
    (*_g_3).words.w0 =
        (0xf6 as libc::c_int as u32_0 &
             (((0x1 as libc::c_int) << 8 as libc::c_int) - 1 as libc::c_int)
                 as libc::c_uint) << 24 as libc::c_int |
            (baseX.wrapping_add((64 as libc::c_int * 1 as libc::c_int) as
                                    libc::c_uint) &
                 (((0x1 as libc::c_int) << 10 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 14 as libc::c_int
            |
            (lry as u32_0 &
                 (((0x1 as libc::c_int) << 10 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 2 as libc::c_int;
    (*_g_3).words.w1 =
        (baseX.wrapping_add((64 as libc::c_int * 0 as libc::c_int) as
                                libc::c_uint) &
             (((0x1 as libc::c_int) << 10 as libc::c_int) - 1 as libc::c_int)
                 as libc::c_uint) << 14 as libc::c_int |
            (uly as u32_0 &
                 (((0x1 as libc::c_int) << 10 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 2 as libc::c_int;
    let fresh5 = gfx;
    gfx = gfx.offset(1);
    let mut _g_4: *mut Gfx = fresh5;
    (*_g_4).words.w0 =
        (0xe7 as libc::c_int as u32_0 &
             (((0x1 as libc::c_int) << 8 as libc::c_int) - 1 as libc::c_int)
                 as libc::c_uint) << 24 as libc::c_int;
    (*_g_4).words.w1 = 0 as libc::c_int as libc::c_uint;
    let fresh6 = gfx;
    gfx = gfx.offset(1);
    let mut _g_5: *mut Gfx = fresh6;
    (*_g_5).words.w0 =
        (0xe7 as libc::c_int as u32_0 &
             (((0x1 as libc::c_int) << 8 as libc::c_int) - 1 as libc::c_int)
                 as libc::c_uint) << 24 as libc::c_int;
    (*_g_5).words.w1 = 0 as libc::c_int as libc::c_uint;
    let fresh7 = gfx;
    gfx = gfx.offset(1);
    let mut _g_6: *mut Gfx = fresh7;
    (*_g_6).words.w0 =
        (0xf7 as libc::c_int as u32_0 &
             (((0x1 as libc::c_int) << 8 as libc::c_int) - 1 as libc::c_int)
                 as libc::c_uint) << 24 as libc::c_int;
    (*_g_6).words.w1 =
        (((0 as libc::c_int) << 8 as libc::c_int & 0xf800 as libc::c_int |
              (255 as libc::c_int) << 3 as libc::c_int & 0x7c0 as libc::c_int
              | 0 as libc::c_int >> 2 as libc::c_int & 0x3e as libc::c_int |
              1 as libc::c_int & 0x1 as libc::c_int) << 16 as libc::c_int |
             ((0 as libc::c_int) << 8 as libc::c_int & 0xf800 as libc::c_int |
                  (255 as libc::c_int) << 3 as libc::c_int &
                      0x7c0 as libc::c_int |
                  0 as libc::c_int >> 2 as libc::c_int & 0x3e as libc::c_int |
                  1 as libc::c_int & 0x1 as libc::c_int)) as libc::c_uint;
    let fresh8 = gfx;
    gfx = gfx.offset(1);
    let mut _g_7: *mut Gfx = fresh8;
    (*_g_7).words.w0 =
        (0xf6 as libc::c_int as u32_0 &
             (((0x1 as libc::c_int) << 8 as libc::c_int) - 1 as libc::c_int)
                 as libc::c_uint) << 24 as libc::c_int |
            (baseX.wrapping_add((64 as libc::c_int * 2 as libc::c_int) as
                                    libc::c_uint) &
                 (((0x1 as libc::c_int) << 10 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 14 as libc::c_int
            |
            (lry as u32_0 &
                 (((0x1 as libc::c_int) << 10 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 2 as libc::c_int;
    (*_g_7).words.w1 =
        (baseX.wrapping_add((64 as libc::c_int * 1 as libc::c_int) as
                                libc::c_uint) &
             (((0x1 as libc::c_int) << 10 as libc::c_int) - 1 as libc::c_int)
                 as libc::c_uint) << 14 as libc::c_int |
            (uly as u32_0 &
                 (((0x1 as libc::c_int) << 10 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 2 as libc::c_int;
    let fresh9 = gfx;
    gfx = gfx.offset(1);
    let mut _g_8: *mut Gfx = fresh9;
    (*_g_8).words.w0 =
        (0xe7 as libc::c_int as u32_0 &
             (((0x1 as libc::c_int) << 8 as libc::c_int) - 1 as libc::c_int)
                 as libc::c_uint) << 24 as libc::c_int;
    (*_g_8).words.w1 = 0 as libc::c_int as libc::c_uint;
    let fresh10 = gfx;
    gfx = gfx.offset(1);
    let mut _g_9: *mut Gfx = fresh10;
    (*_g_9).words.w0 =
        (0xe7 as libc::c_int as u32_0 &
             (((0x1 as libc::c_int) << 8 as libc::c_int) - 1 as libc::c_int)
                 as libc::c_uint) << 24 as libc::c_int;
    (*_g_9).words.w1 = 0 as libc::c_int as libc::c_uint;
    let fresh11 = gfx;
    gfx = gfx.offset(1);
    let mut _g_10: *mut Gfx = fresh11;
    (*_g_10).words.w0 =
        (0xf7 as libc::c_int as u32_0 &
             (((0x1 as libc::c_int) << 8 as libc::c_int) - 1 as libc::c_int)
                 as libc::c_uint) << 24 as libc::c_int;
    (*_g_10).words.w1 =
        (((255 as libc::c_int) << 8 as libc::c_int & 0xf800 as libc::c_int |
              (0 as libc::c_int) << 3 as libc::c_int & 0x7c0 as libc::c_int |
              0 as libc::c_int >> 2 as libc::c_int & 0x3e as libc::c_int |
              1 as libc::c_int & 0x1 as libc::c_int) << 16 as libc::c_int |
             ((255 as libc::c_int) << 8 as libc::c_int & 0xf800 as libc::c_int
                  |
                  (0 as libc::c_int) << 3 as libc::c_int &
                      0x7c0 as libc::c_int |
                  0 as libc::c_int >> 2 as libc::c_int & 0x3e as libc::c_int |
                  1 as libc::c_int & 0x1 as libc::c_int)) as libc::c_uint;
    let fresh12 = gfx;
    gfx = gfx.offset(1);
    let mut _g_11: *mut Gfx = fresh12;
    (*_g_11).words.w0 =
        (0xf6 as libc::c_int as u32_0 &
             (((0x1 as libc::c_int) << 8 as libc::c_int) - 1 as libc::c_int)
                 as libc::c_uint) << 24 as libc::c_int |
            (baseX.wrapping_add((64 as libc::c_int * 3 as libc::c_int) as
                                    libc::c_uint) &
                 (((0x1 as libc::c_int) << 10 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 14 as libc::c_int
            |
            (lry as u32_0 &
                 (((0x1 as libc::c_int) << 10 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 2 as libc::c_int;
    (*_g_11).words.w1 =
        (baseX.wrapping_add((64 as libc::c_int * 2 as libc::c_int) as
                                libc::c_uint) &
             (((0x1 as libc::c_int) << 10 as libc::c_int) - 1 as libc::c_int)
                 as libc::c_uint) << 14 as libc::c_int |
            (uly as u32_0 &
                 (((0x1 as libc::c_int) << 10 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 2 as libc::c_int;
    let fresh13 = gfx;
    gfx = gfx.offset(1);
    let mut _g_12: *mut Gfx = fresh13;
    (*_g_12).words.w0 =
        (0xe7 as libc::c_int as u32_0 &
             (((0x1 as libc::c_int) << 8 as libc::c_int) - 1 as libc::c_int)
                 as libc::c_uint) << 24 as libc::c_int;
    (*_g_12).words.w1 = 0 as libc::c_int as libc::c_uint;
    let fresh14 = gfx;
    gfx = gfx.offset(1);
    let mut _g_13: *mut Gfx = fresh14;
    (*_g_13).words.w0 =
        (0xe7 as libc::c_int as u32_0 &
             (((0x1 as libc::c_int) << 8 as libc::c_int) - 1 as libc::c_int)
                 as libc::c_uint) << 24 as libc::c_int;
    (*_g_13).words.w1 = 0 as libc::c_int as libc::c_uint;
    let fresh15 = gfx;
    gfx = gfx.offset(1);
    let mut _g_14: *mut Gfx = fresh15;
    (*_g_14).words.w0 =
        (0xf7 as libc::c_int as u32_0 &
             (((0x1 as libc::c_int) << 8 as libc::c_int) - 1 as libc::c_int)
                 as libc::c_uint) << 24 as libc::c_int;
    (*_g_14).words.w1 =
        (((255 as libc::c_int) << 8 as libc::c_int & 0xf800 as libc::c_int |
              (0 as libc::c_int) << 3 as libc::c_int & 0x7c0 as libc::c_int |
              255 as libc::c_int >> 2 as libc::c_int & 0x3e as libc::c_int |
              1 as libc::c_int & 0x1 as libc::c_int) << 16 as libc::c_int |
             ((255 as libc::c_int) << 8 as libc::c_int & 0xf800 as libc::c_int
                  |
                  (0 as libc::c_int) << 3 as libc::c_int &
                      0x7c0 as libc::c_int |
                  255 as libc::c_int >> 2 as libc::c_int & 0x3e as libc::c_int
                  | 1 as libc::c_int & 0x1 as libc::c_int)) as libc::c_uint;
    let fresh16 = gfx;
    gfx = gfx.offset(1);
    let mut _g_15: *mut Gfx = fresh16;
    (*_g_15).words.w0 =
        (0xf6 as libc::c_int as u32_0 &
             (((0x1 as libc::c_int) << 8 as libc::c_int) - 1 as libc::c_int)
                 as libc::c_uint) << 24 as libc::c_int |
            (baseX.wrapping_add((64 as libc::c_int * 4 as libc::c_int) as
                                    libc::c_uint) &
                 (((0x1 as libc::c_int) << 10 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 14 as libc::c_int
            |
            (lry as u32_0 &
                 (((0x1 as libc::c_int) << 10 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 2 as libc::c_int;
    (*_g_15).words.w1 =
        (baseX.wrapping_add((64 as libc::c_int * 3 as libc::c_int) as
                                libc::c_uint) &
             (((0x1 as libc::c_int) << 10 as libc::c_int) - 1 as libc::c_int)
                 as libc::c_uint) << 14 as libc::c_int |
            (uly as u32_0 &
                 (((0x1 as libc::c_int) << 10 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 2 as libc::c_int;
    let fresh17 = gfx;
    gfx = gfx.offset(1);
    let mut _g_16: *mut Gfx = fresh17;
    (*_g_16).words.w0 =
        (0xe7 as libc::c_int as u32_0 &
             (((0x1 as libc::c_int) << 8 as libc::c_int) - 1 as libc::c_int)
                 as libc::c_uint) << 24 as libc::c_int;
    (*_g_16).words.w1 = 0 as libc::c_int as libc::c_uint;
    sSpeedMeterTimeEntryPtr =
        &mut *sSpeedMeterTimeEntryArray.as_mut_ptr().offset(0 as libc::c_int
                                                                as isize) as
            *mut SpeedMeterTimeEntry;
    i = 0 as libc::c_int;
    while i <
              (::std::mem::size_of::<[SpeedMeterTimeEntry; 6]>() as
                   libc::c_ulong).wrapping_div(::std::mem::size_of::<SpeedMeterTimeEntry>()
                                                   as libc::c_ulong) as s32 {
        let fresh18 = gfx;
        gfx = gfx.offset(1);
        let mut _g_17: *mut Gfx = fresh18;
        (*_g_17).words.w0 =
            (0xe7 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 24 as libc::c_int;
        (*_g_17).words.w1 = 0 as libc::c_int as libc::c_uint;
        let fresh19 = gfx;
        gfx = gfx.offset(1);
        let mut _g_18: *mut Gfx = fresh19;
        (*_g_18).words.w0 =
            (0xf7 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 24 as libc::c_int;
        (*_g_18).words.w1 =
            (((*sSpeedMeterTimeEntryPtr).color as libc::c_int) <<
                 16 as libc::c_int |
                 (*sSpeedMeterTimeEntryPtr).color as libc::c_int) as
                libc::c_uint;
        let fresh20 = gfx;
        gfx = gfx.offset(1);
        let mut _g_19: *mut Gfx = fresh20;
        (*_g_19).words.w0 =
            (0xf6 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 24 as libc::c_int
                |
                ((*sSpeedMeterTimeEntryPtr).x as u32_0 &
                     (((0x1 as libc::c_int) << 10 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    14 as libc::c_int |
                ((lry + (*sSpeedMeterTimeEntryPtr).y as libc::c_int +
                      1 as libc::c_int) as u32_0 &
                     (((0x1 as libc::c_int) << 10 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    2 as libc::c_int;
        (*_g_19).words.w1 =
            (baseX &
                 (((0x1 as libc::c_int) << 10 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 14 as libc::c_int
                |
                ((lry + (*sSpeedMeterTimeEntryPtr).y as libc::c_int) as u32_0
                     &
                     (((0x1 as libc::c_int) << 10 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    2 as libc::c_int;
        let fresh21 = gfx;
        gfx = gfx.offset(1);
        let mut _g_20: *mut Gfx = fresh21;
        (*_g_20).words.w0 =
            (0xe7 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 24 as libc::c_int;
        (*_g_20).words.w1 = 0 as libc::c_int as libc::c_uint;
        sSpeedMeterTimeEntryPtr = sSpeedMeterTimeEntryPtr.offset(1);
        i += 1
    }
    let fresh22 = gfx;
    gfx = gfx.offset(1);
    let mut _g_21: *mut Gfx = fresh22;
    (*_g_21).words.w0 =
        (0xe7 as libc::c_int as u32_0 &
             (((0x1 as libc::c_int) << 8 as libc::c_int) - 1 as libc::c_int)
                 as libc::c_uint) << 24 as libc::c_int;
    (*_g_21).words.w1 = 0 as libc::c_int as libc::c_uint;
    (*__gfxCtx).overlay.p = gfx;
    Graph_CloseDisps(dispRefs.as_mut_ptr(), gfxCtx,
                     b"../speed_meter.c\x00" as *const u8 as
                         *const libc::c_char, 276 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn SpeedMeter_InitAllocEntry(mut this:
                                                       *mut SpeedMeterAllocEntry,
                                                   mut maxval: u32_0,
                                                   mut val: u32_0,
                                                   mut backColor: u16_0,
                                                   mut foreColor: u16_0,
                                                   mut ulx: u32_0,
                                                   mut lrx: u32_0,
                                                   mut uly: u32_0,
                                                   mut lry: u32_0) {
    (*this).maxval = maxval as s32;
    (*this).val = val as s32;
    (*this).backColor = backColor;
    (*this).foreColor = foreColor;
    (*this).ulx = ulx as s32;
    (*this).lrx = lrx as s32;
    (*this).uly = uly as s32;
    (*this).lry = lry as s32;
}
#[no_mangle]
pub unsafe extern "C" fn SpeedMeter_DrawAllocEntry(mut this:
                                                       *mut SpeedMeterAllocEntry,
                                                   mut gfxCtx:
                                                       *mut GraphicsContext) {
    let mut usedOff: s32 = 0;
    let mut view: View =
        View{magic: 0,
             gfxCtx: 0 as *mut GraphicsContext,
             viewport: Viewport{topY: 0, bottomY: 0, leftX: 0, rightX: 0,},
             fovy: 0.,
             zNear: 0.,
             zFar: 0.,
             scale: 0.,
             eye: Vec3f{x: 0., y: 0., z: 0.,},
             lookAt: Vec3f{x: 0., y: 0., z: 0.,},
             up: Vec3f{x: 0., y: 0., z: 0.,},
             vp: Vp{vp: Vp_t{vscale: [0; 4], vtrans: [0; 4],},},
             projection: Mtx{m: [[0; 4]; 4],},
             viewing: Mtx{m: [[0; 4]; 4],},
             projectionPtr: 0 as *mut Mtx,
             viewingPtr: 0 as *mut Mtx,
             unk_E8: Vec3f{x: 0., y: 0., z: 0.,},
             unk_F4: Vec3f{x: 0., y: 0., z: 0.,},
             unk_100: 0.,
             unk_104: Vec3f{x: 0., y: 0., z: 0.,},
             unk_110: Vec3f{x: 0., y: 0., z: 0.,},
             normal: 0,
             flags: 0,
             unk_124: 0,};
    let mut gfx: *mut Gfx = 0 as *mut Gfx;
    if (*this).maxval == 0 as libc::c_int {
        osSyncPrintf(b"\x1b[31m\x00" as *const u8 as *const libc::c_char);
        LogUtils_LogThreadId(b"../speed_meter.c\x00" as *const u8 as
                                 *const libc::c_char, 313 as libc::c_int);
        osSyncPrintf(b"this->maxval = %d\n\x00" as *const u8 as
                         *const libc::c_char, (*this).maxval);
        osSyncPrintf(b"\x1b[m\x00" as *const u8 as *const libc::c_char);
    } else {
        let mut __gfxCtx: *mut GraphicsContext = 0 as *mut GraphicsContext;
        let mut dispRefs: [*mut Gfx; 4] = [0 as *mut Gfx; 4];
        __gfxCtx = gfxCtx;
        Graph_OpenDisps(dispRefs.as_mut_ptr(), gfxCtx,
                        b"../speed_meter.c\x00" as *const u8 as
                            *const libc::c_char, 318 as libc::c_int);
        View_Init(&mut view, gfxCtx);
        view.flags = 0xa as libc::c_int;
        let mut viewport: Viewport =
            Viewport{topY: 0, bottomY: 0, leftX: 0, rightX: 0,};
        viewport.bottomY = 240 as libc::c_int;
        viewport.rightX = 320 as libc::c_int;
        viewport.topY = 0 as libc::c_int;
        viewport.leftX = 0 as libc::c_int;
        View_SetViewport(&mut view, &mut viewport);
        gfx = (*__gfxCtx).overlay.p;
        func_800AB9EC(&mut view, 0xf as libc::c_int, &mut gfx);
        let fresh23 = gfx;
        gfx = gfx.offset(1);
        let mut _g: *mut Gfx = fresh23;
        (*_g).words.w0 =
            (0xe7 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 24 as libc::c_int;
        (*_g).words.w1 = 0 as libc::c_int as libc::c_uint;
        let fresh24 = gfx;
        gfx = gfx.offset(1);
        let mut _g_0: *mut Gfx = fresh24;
        (*_g_0).words.w0 =
            (0xef as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 24 as libc::c_int
                |
                (((0 as libc::c_int) << 4 as libc::c_int |
                      (0 as libc::c_int) << 6 as libc::c_int |
                      (0 as libc::c_int) << 8 as libc::c_int |
                      (0 as libc::c_int) << 9 as libc::c_int |
                      (0 as libc::c_int) << 12 as libc::c_int |
                      (0 as libc::c_int) << 14 as libc::c_int |
                      (0 as libc::c_int) << 16 as libc::c_int |
                      (0 as libc::c_int) << 17 as libc::c_int |
                      (0 as libc::c_int) << 19 as libc::c_int |
                      (3 as libc::c_int) << 20 as libc::c_int |
                      (0 as libc::c_int) << 23 as libc::c_int) as u32_0 &
                     (((0x1 as libc::c_int) << 24 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    0 as libc::c_int;
        (*_g_0).words.w1 =
            ((0 as libc::c_int) << 0 as libc::c_int |
                 (0 as libc::c_int) << 2 as libc::c_int |
                 (0 as libc::c_int) << 30 as libc::c_int |
                 (0 as libc::c_int) << 26 as libc::c_int |
                 (0 as libc::c_int) << 22 as libc::c_int |
                 (0 as libc::c_int) << 18 as libc::c_int |
                 (0 as libc::c_int) << 28 as libc::c_int |
                 (0 as libc::c_int) << 24 as libc::c_int |
                 (0 as libc::c_int) << 20 as libc::c_int |
                 (0 as libc::c_int) << 16 as libc::c_int) as libc::c_uint;
        usedOff =
            ((*this).lrx - (*this).ulx) * (*this).val / (*this).maxval +
                (*this).ulx;
        let fresh25 = gfx;
        gfx = gfx.offset(1);
        let mut _g_1: *mut Gfx = fresh25;
        (*_g_1).words.w0 =
            (0xe7 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 24 as libc::c_int;
        (*_g_1).words.w1 = 0 as libc::c_int as libc::c_uint;
        let fresh26 = gfx;
        gfx = gfx.offset(1);
        let mut _g_2: *mut Gfx = fresh26;
        (*_g_2).words.w0 =
            (0xf7 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 24 as libc::c_int;
        (*_g_2).words.w1 =
            (((*this).backColor as libc::c_int) << 16 as libc::c_int |
                 (*this).backColor as libc::c_int) as libc::c_uint;
        let fresh27 = gfx;
        gfx = gfx.offset(1);
        let mut _g_3: *mut Gfx = fresh27;
        (*_g_3).words.w0 =
            (0xf6 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 24 as libc::c_int
                |
                ((*this).lrx as u32_0 &
                     (((0x1 as libc::c_int) << 10 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    14 as libc::c_int |
                ((*this).lry as u32_0 &
                     (((0x1 as libc::c_int) << 10 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    2 as libc::c_int;
        (*_g_3).words.w1 =
            (usedOff as u32_0 &
                 (((0x1 as libc::c_int) << 10 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 14 as libc::c_int
                |
                ((*this).uly as u32_0 &
                     (((0x1 as libc::c_int) << 10 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    2 as libc::c_int;
        let fresh28 = gfx;
        gfx = gfx.offset(1);
        let mut _g_4: *mut Gfx = fresh28;
        (*_g_4).words.w0 =
            (0xe7 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 24 as libc::c_int;
        (*_g_4).words.w1 = 0 as libc::c_int as libc::c_uint;
        let fresh29 = gfx;
        gfx = gfx.offset(1);
        let mut _g_5: *mut Gfx = fresh29;
        (*_g_5).words.w0 =
            (0xe7 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 24 as libc::c_int;
        (*_g_5).words.w1 = 0 as libc::c_int as libc::c_uint;
        let fresh30 = gfx;
        gfx = gfx.offset(1);
        let mut _g_6: *mut Gfx = fresh30;
        (*_g_6).words.w0 =
            (0xf7 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 24 as libc::c_int;
        (*_g_6).words.w1 =
            (((*this).foreColor as libc::c_int) << 16 as libc::c_int |
                 (*this).foreColor as libc::c_int) as libc::c_uint;
        let fresh31 = gfx;
        gfx = gfx.offset(1);
        let mut _g_7: *mut Gfx = fresh31;
        (*_g_7).words.w0 =
            (0xf6 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 24 as libc::c_int
                |
                (usedOff as u32_0 &
                     (((0x1 as libc::c_int) << 10 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    14 as libc::c_int |
                ((*this).lry as u32_0 &
                     (((0x1 as libc::c_int) << 10 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    2 as libc::c_int;
        (*_g_7).words.w1 =
            ((*this).ulx as u32_0 &
                 (((0x1 as libc::c_int) << 10 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 14 as libc::c_int
                |
                ((*this).uly as u32_0 &
                     (((0x1 as libc::c_int) << 10 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    2 as libc::c_int;
        let fresh32 = gfx;
        gfx = gfx.offset(1);
        let mut _g_8: *mut Gfx = fresh32;
        (*_g_8).words.w0 =
            (0xe7 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 24 as libc::c_int;
        (*_g_8).words.w1 = 0 as libc::c_int as libc::c_uint;
        let fresh33 = gfx;
        gfx = gfx.offset(1);
        let mut _g_9: *mut Gfx = fresh33;
        (*_g_9).words.w0 =
            (0xe7 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 24 as libc::c_int;
        (*_g_9).words.w1 = 0 as libc::c_int as libc::c_uint;
        (*__gfxCtx).overlay.p = gfx;
        Graph_CloseDisps(dispRefs.as_mut_ptr(), gfxCtx,
                         b"../speed_meter.c\x00" as *const u8 as
                             *const libc::c_char, 339 as libc::c_int);
    };
}
#[no_mangle]
pub unsafe extern "C" fn SpeedMeter_DrawAllocEntries(mut meter:
                                                         *mut SpeedMeter,
                                                     mut gfxCtx:
                                                         *mut GraphicsContext,
                                                     mut state:
                                                         *mut GameState) {
    let mut pad: [s32; 2] = [0; 2];
    let mut ulx: u32_0 = 30 as libc::c_int as u32_0;
    let mut lrx: u32_0 = 290 as libc::c_int as u32_0;
    let mut entry: SpeedMeterAllocEntry =
        SpeedMeterAllocEntry{maxval: 0,
                             val: 0,
                             backColor: 0,
                             foreColor: 0,
                             ulx: 0,
                             lrx: 0,
                             uly: 0,
                             lry: 0,};
    let mut temp: u32_0 = 0;
    let mut y: s32 = 0;
    let mut thga: *mut TwoHeadGfxArena = 0 as *mut TwoHeadGfxArena;
    let mut zeldaFreeMax: u32_0 = 0;
    let mut zeldaFree: u32_0 = 0;
    let mut zeldaAlloc: u32_0 = 0;
    let mut sysFreeMax: s32 = 0;
    let mut sysFree: s32 = 0;
    let mut sysAlloc: s32 = 0;
    y = 212 as libc::c_int;
    if (*gGameInfo).data[(1 as libc::c_int * 6 as libc::c_int *
                              16 as libc::c_int + 0 as libc::c_int) as usize]
           as libc::c_int > 2 as libc::c_int {
        if ZeldaArena_IsInitalized() != 0 {
            ZeldaArena_GetSizes(&mut zeldaFreeMax, &mut zeldaFree,
                                &mut zeldaAlloc);
            SpeedMeter_InitAllocEntry(&mut entry,
                                      zeldaFree.wrapping_add(zeldaAlloc),
                                      zeldaAlloc,
                                      ((0 as libc::c_int) << 8 as libc::c_int
                                           & 0xf800 as libc::c_int |
                                           (0 as libc::c_int) <<
                                               3 as libc::c_int &
                                               0x7c0 as libc::c_int |
                                           255 as libc::c_int >>
                                               2 as libc::c_int &
                                               0x3e as libc::c_int |
                                           1 as libc::c_int &
                                               0x1 as libc::c_int) as u16_0,
                                      ((255 as libc::c_int) <<
                                           8 as libc::c_int &
                                           0xf800 as libc::c_int |
                                           (255 as libc::c_int) <<
                                               3 as libc::c_int &
                                               0x7c0 as libc::c_int |
                                           255 as libc::c_int >>
                                               2 as libc::c_int &
                                               0x3e as libc::c_int |
                                           1 as libc::c_int &
                                               0x1 as libc::c_int) as u16_0,
                                      ulx, lrx, y as u32_0,
                                      (y + 1 as libc::c_int) as u32_0);
            SpeedMeter_DrawAllocEntry(&mut entry, gfxCtx);
            y += 1;
            y += 1
        }
    }
    if (*gGameInfo).data[(1 as libc::c_int * 6 as libc::c_int *
                              16 as libc::c_int + 0 as libc::c_int) as usize]
           as libc::c_int > 1 as libc::c_int {
        SystemArena_GetSizes(&mut sysFreeMax as *mut s32 as *mut u32_0,
                             &mut sysFree as *mut s32 as *mut u32_0,
                             &mut sysAlloc as *mut s32 as *mut u32_0);
        SpeedMeter_InitAllocEntry(&mut entry,
                                  ((sysFree + sysAlloc) as
                                       libc::c_uint).wrapping_sub((*state).tha.size),
                                  (sysAlloc as
                                       libc::c_uint).wrapping_sub((*state).tha.size),
                                  ((0 as libc::c_int) << 8 as libc::c_int &
                                       0xf800 as libc::c_int |
                                       (0 as libc::c_int) << 3 as libc::c_int
                                           & 0x7c0 as libc::c_int |
                                       255 as libc::c_int >> 2 as libc::c_int
                                           & 0x3e as libc::c_int |
                                       1 as libc::c_int & 0x1 as libc::c_int)
                                      as u16_0,
                                  ((255 as libc::c_int) << 8 as libc::c_int &
                                       0xf800 as libc::c_int |
                                       (128 as libc::c_int) <<
                                           3 as libc::c_int &
                                           0x7c0 as libc::c_int |
                                       128 as libc::c_int >> 2 as libc::c_int
                                           & 0x3e as libc::c_int |
                                       1 as libc::c_int & 0x1 as libc::c_int)
                                      as u16_0, ulx, lrx, y as u32_0,
                                  y as u32_0);
        SpeedMeter_DrawAllocEntry(&mut entry, gfxCtx);
        y += 1
    }
    thga = &mut (*state).tha as *mut TwoHeadArena as *mut TwoHeadGfxArena;
    SpeedMeter_InitAllocEntry(&mut entry, (*thga).size,
                              (*thga).size.wrapping_sub(THA_GetSize(thga as
                                                                        *mut TwoHeadArena)
                                                            as libc::c_uint),
                              ((0 as libc::c_int) << 8 as libc::c_int &
                                   0xf800 as libc::c_int |
                                   (0 as libc::c_int) << 3 as libc::c_int &
                                       0x7c0 as libc::c_int |
                                   255 as libc::c_int >> 2 as libc::c_int &
                                       0x3e as libc::c_int |
                                   1 as libc::c_int & 0x1 as libc::c_int) as
                                  u16_0,
                              ((0 as libc::c_int) << 8 as libc::c_int &
                                   0xf800 as libc::c_int |
                                   (255 as libc::c_int) << 3 as libc::c_int &
                                       0x7c0 as libc::c_int |
                                   0 as libc::c_int >> 2 as libc::c_int &
                                       0x3e as libc::c_int |
                                   1 as libc::c_int & 0x1 as libc::c_int) as
                                  u16_0, ulx, lrx, y as u32_0, y as u32_0);
    SpeedMeter_DrawAllocEntry(&mut entry, gfxCtx);
    y += 1;
    thga = &mut (*gfxCtx).polyOpa;
    SpeedMeter_InitAllocEntry(&mut entry, (*thga).size,
                              (*thga).size.wrapping_sub(THGA_GetSize(thga) as
                                                            libc::c_uint),
                              ((0 as libc::c_int) << 8 as libc::c_int &
                                   0xf800 as libc::c_int |
                                   (0 as libc::c_int) << 3 as libc::c_int &
                                       0x7c0 as libc::c_int |
                                   255 as libc::c_int >> 2 as libc::c_int &
                                       0x3e as libc::c_int |
                                   1 as libc::c_int & 0x1 as libc::c_int) as
                                  u16_0,
                              ((255 as libc::c_int) << 8 as libc::c_int &
                                   0xf800 as libc::c_int |
                                   (0 as libc::c_int) << 3 as libc::c_int &
                                       0x7c0 as libc::c_int |
                                   255 as libc::c_int >> 2 as libc::c_int &
                                       0x3e as libc::c_int |
                                   1 as libc::c_int & 0x1 as libc::c_int) as
                                  u16_0, ulx, lrx, y as u32_0, y as u32_0);
    SpeedMeter_DrawAllocEntry(&mut entry, gfxCtx);
    y += 1;
    thga = &mut (*gfxCtx).polyXlu;
    SpeedMeter_InitAllocEntry(&mut entry, (*thga).size,
                              (*thga).size.wrapping_sub(THGA_GetSize(thga) as
                                                            libc::c_uint),
                              ((0 as libc::c_int) << 8 as libc::c_int &
                                   0xf800 as libc::c_int |
                                   (0 as libc::c_int) << 3 as libc::c_int &
                                       0x7c0 as libc::c_int |
                                   255 as libc::c_int >> 2 as libc::c_int &
                                       0x3e as libc::c_int |
                                   1 as libc::c_int & 0x1 as libc::c_int) as
                                  u16_0,
                              ((255 as libc::c_int) << 8 as libc::c_int &
                                   0xf800 as libc::c_int |
                                   (255 as libc::c_int) << 3 as libc::c_int &
                                       0x7c0 as libc::c_int |
                                   0 as libc::c_int >> 2 as libc::c_int &
                                       0x3e as libc::c_int |
                                   1 as libc::c_int & 0x1 as libc::c_int) as
                                  u16_0, ulx, lrx, y as u32_0, y as u32_0);
    SpeedMeter_DrawAllocEntry(&mut entry, gfxCtx);
    y += 1;
    thga = &mut (*gfxCtx).overlay;
    SpeedMeter_InitAllocEntry(&mut entry, (*thga).size,
                              (*thga).size.wrapping_sub(THGA_GetSize(thga) as
                                                            libc::c_uint),
                              ((0 as libc::c_int) << 8 as libc::c_int &
                                   0xf800 as libc::c_int |
                                   (0 as libc::c_int) << 3 as libc::c_int &
                                       0x7c0 as libc::c_int |
                                   255 as libc::c_int >> 2 as libc::c_int &
                                       0x3e as libc::c_int |
                                   1 as libc::c_int & 0x1 as libc::c_int) as
                                  u16_0,
                              ((255 as libc::c_int) << 8 as libc::c_int &
                                   0xf800 as libc::c_int |
                                   (0 as libc::c_int) << 3 as libc::c_int &
                                       0x7c0 as libc::c_int |
                                   0 as libc::c_int >> 2 as libc::c_int &
                                       0x3e as libc::c_int |
                                   1 as libc::c_int & 0x1 as libc::c_int) as
                                  u16_0, ulx, lrx, y as u32_0, y as u32_0);
    SpeedMeter_DrawAllocEntry(&mut entry, gfxCtx);
    y += 1;
}
