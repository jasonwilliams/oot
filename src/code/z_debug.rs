#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case,
         non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(const_raw_ptr_to_usize_cast, const_transmute, register_tool)]
extern "C" {
    #[no_mangle]
    fn func_800AA000(_: f32_0, _: u8_0, _: u8_0, _: u8_0);
    #[no_mangle]
    fn Graph_OpenDisps(dispRefs: *mut *mut Gfx, gfxCtx: *mut GraphicsContext,
                       file: *const libc::c_char, line: s32);
    #[no_mangle]
    fn Graph_CloseDisps(dispRefs: *mut *mut Gfx, gfxCtx: *mut GraphicsContext,
                        file: *const libc::c_char, line: s32);
    #[no_mangle]
    fn Graph_GfxPlusOne(gfx: *mut Gfx) -> *mut Gfx;
    #[no_mangle]
    fn Graph_BranchDlist(gfx: *mut Gfx, dst: *mut Gfx) -> *mut Gfx;
    #[no_mangle]
    fn GfxPrint_SetColor(this: *mut GfxPrint, r: u32_0, g: u32_0, b: u32_0,
                         a: u32_0);
    #[no_mangle]
    fn GfxPrint_SetPos(this: *mut GfxPrint, x: s32, y: s32);
    #[no_mangle]
    fn GfxPrint_Init(this: *mut GfxPrint);
    #[no_mangle]
    fn GfxPrint_Destroy(this: *mut GfxPrint);
    #[no_mangle]
    fn GfxPrint_Open(this: *mut GfxPrint, dList: *mut Gfx);
    #[no_mangle]
    fn GfxPrint_Close(this: *mut GfxPrint) -> *mut Gfx;
    #[no_mangle]
    fn GfxPrint_Printf(this: *mut GfxPrint, fmt: *const libc::c_char, _: ...)
     -> s32;
    #[no_mangle]
    fn SystemArena_MallocDebug(size: u32_0, file: *const libc::c_char,
                               line: s32) -> *mut libc::c_void;
}
pub type s8 = libc::c_schar;
pub type u8_0 = libc::c_uchar;
pub type s16 = libc::c_short;
pub type u16_0 = libc::c_ushort;
pub type s32 = libc::c_int;
pub type u32_0 = libc::c_uint;
pub type u64_0 = libc::c_ulonglong;
pub type f32_0 = libc::c_float;
pub type OSPri = s32;
pub type OSId = s32;
#[derive(Copy, Clone)]
#[repr(C)]
pub union __OSfp {
    pub f: C2RustUnnamed,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed {
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
pub type PrintCallback
    =
    Option<unsafe extern "C" fn(_: *mut libc::c_void, _: *const libc::c_char,
                                _: u32_0) -> *mut libc::c_void>;
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
pub struct Color_RGBA8 {
    pub r: u8_0,
    pub g: u8_0,
    pub b: u8_0,
    pub a: u8_0,
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
pub struct GfxPrint {
    pub callback: PrintCallback,
    pub dList: *mut Gfx,
    pub posX: u16_0,
    pub posY: u16_0,
    pub baseX: u16_0,
    pub baseY: u8_0,
    pub flags: u8_0,
    pub color: Color_RGBA8_u32,
    pub unk_14: [libc::c_char; 28],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PrintTextBuffer {
    pub x: u8_0,
    pub y: u8_0,
    pub colorId: u8_0,
    pub text: [libc::c_char; 21],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct InputCombo {
    pub push: u16_0,
    pub held: u16_0,
}
#[no_mangle]
pub static mut gGameInfo: *mut GameInfo =
    0 as *const GameInfo as *mut GameInfo;
#[no_mangle]
pub static mut D_8015FA94: s32 = 0;
// no known symbols
#[no_mangle]
pub static mut D_8015FA98: [PrintTextBuffer; 22] =
    [PrintTextBuffer{x: 0, y: 0, colorId: 0, text: [0; 21],}; 22];
#[no_mangle]
pub static mut D_8011E0B0: s16 = 0 as libc::c_int as s16;
// PrintTextBuffer index
#[no_mangle]
pub static mut printTextColors: [Color_RGBA8; 8] =
    [{
         let mut init =
             Color_RGBA8{r: 255 as libc::c_int as u8_0,
                         g: 255 as libc::c_int as u8_0,
                         b: 32 as libc::c_int as u8_0,
                         a: 192 as libc::c_int as u8_0,};
         init
     },
     {
         let mut init =
             Color_RGBA8{r: 255 as libc::c_int as u8_0,
                         g: 150 as libc::c_int as u8_0,
                         b: 128 as libc::c_int as u8_0,
                         a: 192 as libc::c_int as u8_0,};
         init
     },
     {
         let mut init =
             Color_RGBA8{r: 128 as libc::c_int as u8_0,
                         g: 96 as libc::c_int as u8_0,
                         b: 0 as libc::c_int as u8_0,
                         a: 64 as libc::c_int as u8_0,};
         init
     },
     {
         let mut init =
             Color_RGBA8{r: 192 as libc::c_int as u8_0,
                         g: 128 as libc::c_int as u8_0,
                         b: 16 as libc::c_int as u8_0,
                         a: 128 as libc::c_int as u8_0,};
         init
     },
     {
         let mut init =
             Color_RGBA8{r: 255 as libc::c_int as u8_0,
                         g: 192 as libc::c_int as u8_0,
                         b: 32 as libc::c_int as u8_0,
                         a: 128 as libc::c_int as u8_0,};
         init
     },
     {
         let mut init =
             Color_RGBA8{r: 230 as libc::c_int as u8_0,
                         g: 230 as libc::c_int as u8_0,
                         b: 220 as libc::c_int as u8_0,
                         a: 64 as libc::c_int as u8_0,};
         init
     },
     {
         let mut init =
             Color_RGBA8{r: 128 as libc::c_int as u8_0,
                         g: 150 as libc::c_int as u8_0,
                         b: 255 as libc::c_int as u8_0,
                         a: 128 as libc::c_int as u8_0,};
         init
     },
     {
         let mut init =
             Color_RGBA8{r: 128 as libc::c_int as u8_0,
                         g: 255 as libc::c_int as u8_0,
                         b: 32 as libc::c_int as u8_0,
                         a: 128 as libc::c_int as u8_0,};
         init
     }];
#[no_mangle]
pub static mut inputCombos: [InputCombo; 29] =
    [{
         let mut init =
             InputCombo{push: 0x20 as libc::c_int as u16_0,
                        held: 0x8 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             InputCombo{push: 0x20 as libc::c_int as u16_0,
                        held: 0x2 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             InputCombo{push: 0x20 as libc::c_int as u16_0,
                        held: 0x4 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             InputCombo{push: 0x20 as libc::c_int as u16_0,
                        held: 0x8000 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             InputCombo{push: 0x10 as libc::c_int as u16_0,
                        held: 0x4 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             InputCombo{push: 0x20 as libc::c_int as u16_0,
                        held: 0x1 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             InputCombo{push: 0x20 as libc::c_int as u16_0,
                        held: 0x10 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             InputCombo{push: 0x20 as libc::c_int as u16_0,
                        held: 0x200 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             InputCombo{push: 0x20 as libc::c_int as u16_0,
                        held: 0x100 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             InputCombo{push: 0x20 as libc::c_int as u16_0,
                        held: 0x800 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             InputCombo{push: 0x20 as libc::c_int as u16_0,
                        held: 0x4000 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             InputCombo{push: 0x20 as libc::c_int as u16_0,
                        held: 0x2000 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             InputCombo{push: 0x20 as libc::c_int as u16_0,
                        held: 0x400 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             InputCombo{push: 0x10 as libc::c_int as u16_0,
                        held: 0x8000 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             InputCombo{push: 0x10 as libc::c_int as u16_0,
                        held: 0x4000 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             InputCombo{push: 0x10 as libc::c_int as u16_0,
                        held: 0x2000 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             InputCombo{push: 0x10 as libc::c_int as u16_0,
                        held: 0x20 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             InputCombo{push: 0x10 as libc::c_int as u16_0,
                        held: 0x8 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             InputCombo{push: 0x10 as libc::c_int as u16_0,
                        held: 0x1 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             InputCombo{push: 0x10 as libc::c_int as u16_0,
                        held: 0x200 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             InputCombo{push: 0x10 as libc::c_int as u16_0,
                        held: 0x2 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             InputCombo{push: 0x10 as libc::c_int as u16_0,
                        held: 0x1000 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             InputCombo{push: 0x20 as libc::c_int as u16_0,
                        held: 0x1000 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             InputCombo{push: 0x10 as libc::c_int as u16_0,
                        held: 0x100 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             InputCombo{push: 0x10 as libc::c_int as u16_0,
                        held: 0x800 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             InputCombo{push: 0x1000 as libc::c_int as u16_0,
                        held: 0x10 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             InputCombo{push: 0x1000 as libc::c_int as u16_0,
                        held: 0x8000 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             InputCombo{push: 0x1000 as libc::c_int as u16_0,
                        held: 0x4000 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             InputCombo{push: 0x1000 as libc::c_int as u16_0,
                        held: 0x1 as libc::c_int as u16_0,};
         init
     }];
#[no_mangle]
pub static mut regChar: [libc::c_char; 30] =
    unsafe {
        *::std::mem::transmute::<&[u8; 30],
                                 &mut [libc::c_char; 30]>(b" SOPQMYDUIZCNKXcsiWAVHGmnBdkb\x00")
    };
// initialize GameInfo
#[no_mangle]
pub unsafe extern "C" fn func_800636C0() {
    let mut i: s32 = 0;
    gGameInfo =
        SystemArena_MallocDebug(::std::mem::size_of::<GameInfo>() as
                                    libc::c_ulong,
                                b"../z_debug.c\x00" as *const u8 as
                                    *const libc::c_char, 260 as libc::c_int)
            as *mut GameInfo;
    (*gGameInfo).regPage = 0 as libc::c_int;
    (*gGameInfo).regGroup = 0 as libc::c_int;
    (*gGameInfo).regCur = 0 as libc::c_int;
    (*gGameInfo).dpadLast = 0 as libc::c_int;
    (*gGameInfo).repeat = 0 as libc::c_int;
    i = 0 as libc::c_int;
    while i <
              (::std::mem::size_of::<[s16; 2784]>() as
                   libc::c_ulong).wrapping_div(::std::mem::size_of::<s16>() as
                                                   libc::c_ulong) as s32 {
        (*gGameInfo).data[i as usize] = 0 as libc::c_int as s16;
        i += 1
    };
}
// Called when free movement is active.
// 8011D394 to enable camera debugger
#[no_mangle]
pub unsafe extern "C" fn func_8006375C(mut arg0: s32, mut arg1: s32,
                                       mut text: *const libc::c_char) {
}
// Copy Camera Debugger Text
#[no_mangle]
pub unsafe extern "C" fn func_8006376C(mut x: u8_0, mut y: u8_0,
                                       mut colorId: u8_0,
                                       mut text: *const libc::c_char) {
    let mut buf: *mut PrintTextBuffer = 0 as *mut PrintTextBuffer;
    let mut bufText: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut i: s16 = 0;
    buf =
        &mut *D_8015FA98.as_mut_ptr().offset(D_8011E0B0 as isize) as
            *mut PrintTextBuffer;
    if (D_8011E0B0 as libc::c_int) < 0x16 as libc::c_int {
        (*buf).x = x;
        (*buf).y = y;
        (*buf).colorId = colorId;
        i = 0 as libc::c_int as s16;
        bufText = (*buf).text.as_mut_ptr();
        loop  {
            let fresh0 = text;
            text = text.offset(1);
            let fresh1 = bufText;
            bufText = bufText.offset(1);
            *fresh1 = *fresh0;
            if !(*fresh1 != 0) { break ; }
            let fresh2 = i;
            i = i + 1;
            if fresh2 as libc::c_int > 0x14 as libc::c_int { break ; }
        }
        *bufText = '\u{0}' as i32 as libc::c_char;
        D_8011E0B0 += 1
    };
}
// Draw Text
#[no_mangle]
pub unsafe extern "C" fn func_80063828(mut printer: *mut GfxPrint) {
    let mut i: s32 = 0;
    let mut color: *mut Color_RGBA8 = 0 as *mut Color_RGBA8;
    let mut buffer: *mut PrintTextBuffer = 0 as *mut PrintTextBuffer;
    let mut text: *mut libc::c_char = 0 as *mut libc::c_char;
    i = 0 as libc::c_int;
    if D_8011E0B0 as libc::c_int > 0 as libc::c_int {
        loop  {
            buffer =
                &mut *D_8015FA98.as_mut_ptr().offset(i as isize) as
                    *mut PrintTextBuffer;
            text = (*buffer).text.as_mut_ptr();
            color =
                &mut *printTextColors.as_mut_ptr().offset((*buffer).colorId as
                                                              isize) as
                    *mut Color_RGBA8;
            GfxPrint_SetColor(printer, (*color).r as u32_0,
                              (*color).g as u32_0, (*color).b as u32_0,
                              (*color).a as u32_0);
            GfxPrint_SetPos(printer, (*buffer).x as s32, (*buffer).y as s32);
            GfxPrint_Printf(printer,
                            b"%s\x00" as *const u8 as *const libc::c_char,
                            text);
            i += 1 as libc::c_int;
            if !(i < D_8011E0B0 as libc::c_int) { break ; }
        }
    };
}
// Edit REG
#[no_mangle]
pub unsafe extern "C" fn func_8006390C(mut input: *mut Input) {
    let mut dpad: s32 = 0;
    let mut regGroup: s32 = 0;
    let mut increment: s32 = 0;
    let mut input_combo: *mut InputCombo = 0 as *mut InputCombo;
    let mut i: s32 = 0;
    regGroup =
        ((*gGameInfo).regGroup * 6 as libc::c_int + (*gGameInfo).regPage) *
            16 as libc::c_int - 16 as libc::c_int;
    dpad =
        (*input).cur.button as libc::c_int &
            (0x800 as libc::c_int | 0x200 as libc::c_int |
                 0x100 as libc::c_int | 0x400 as libc::c_int);
    if !((*input).cur.button as libc::c_int | !(0x20 as libc::c_int)) ==
           0 as libc::c_int ||
           !((*input).cur.button as libc::c_int | !(0x10 as libc::c_int)) ==
               0 as libc::c_int ||
           !((*input).cur.button as libc::c_int | !(0x1000 as libc::c_int)) ==
               0 as libc::c_int {
        input_combo = inputCombos.as_mut_ptr();
        i = 0 as libc::c_int;
        while i < 29 as libc::c_int {
            if !(!(!((*input_combo).push as libc::c_int) |
                       (*input).cur.button as libc::c_int) != 0 ||
                     !(!((*input_combo).held as libc::c_int) |
                           (*input).press.button as libc::c_int) != 0) {
                break ;
            }
            input_combo = input_combo.offset(1);
            i += 1
        }
        if i < 29 as libc::c_int {
            if i == (*gGameInfo).regGroup {
                (*gGameInfo).regPage =
                    ((*gGameInfo).regPage + 1 as libc::c_int) %
                        (6 as libc::c_int + 1 as libc::c_int);
                return
            }
            (*gGameInfo).regGroup = i;
            (*gGameInfo).regPage = 0 as libc::c_int
        }
    } else {
        match (*gGameInfo).regPage - 1 as libc::c_int {
            0 | 1 | 2 | 3 | 4 | 5 => {
                if dpad == (*gGameInfo).dpadLast {
                    (*gGameInfo).repeat -= 1;
                    if (*gGameInfo).repeat < 0 as libc::c_int {
                        (*gGameInfo).repeat = 1 as libc::c_int
                    } else { dpad ^= (*gGameInfo).dpadLast }
                } else {
                    (*gGameInfo).repeat = 0x10 as libc::c_int;
                    (*gGameInfo).dpadLast = dpad
                }
                increment =
                    if dpad & 0x100 as libc::c_int != 0 as libc::c_int {
                        if !((*input).cur.button as libc::c_int |
                                 !(0x8000 as libc::c_int |
                                       0x4000 as libc::c_int)) ==
                               0 as libc::c_int {
                            1000 as libc::c_int
                        } else if !((*input).cur.button as libc::c_int |
                                        !(0x8000 as libc::c_int)) ==
                                      0 as libc::c_int {
                            100 as libc::c_int
                        } else if !((*input).cur.button as libc::c_int |
                                        !(0x4000 as libc::c_int)) ==
                                      0 as libc::c_int {
                            10 as libc::c_int
                        } else { 1 as libc::c_int }
                    } else if dpad & 0x200 as libc::c_int != 0 as libc::c_int
                     {
                        if !((*input).cur.button as libc::c_int |
                                 !(0x8000 as libc::c_int |
                                       0x4000 as libc::c_int)) ==
                               0 as libc::c_int {
                            -(1000 as libc::c_int)
                        } else if !((*input).cur.button as libc::c_int |
                                        !(0x8000 as libc::c_int)) ==
                                      0 as libc::c_int {
                            -(100 as libc::c_int)
                        } else if !((*input).cur.button as libc::c_int |
                                        !(0x4000 as libc::c_int)) ==
                                      0 as libc::c_int {
                            -(10 as libc::c_int)
                        } else { -(1 as libc::c_int) }
                    } else { 0 as libc::c_int };
                (*gGameInfo).data[((*gGameInfo).regCur + regGroup) as usize] =
                    ((*gGameInfo).data[((*gGameInfo).regCur + regGroup) as
                                           usize] as libc::c_int + increment)
                        as s16;
                if dpad & 0x800 as libc::c_int != 0 as libc::c_int {
                    (*gGameInfo).regCur -= 1;
                    if (*gGameInfo).regCur < 0 as libc::c_int {
                        (*gGameInfo).regCur =
                            16 as libc::c_int - 1 as libc::c_int
                    }
                } else if dpad & 0x400 as libc::c_int != 0 as libc::c_int {
                    (*gGameInfo).regCur += 1;
                    if (*gGameInfo).regCur >= 16 as libc::c_int {
                        (*gGameInfo).regCur = 0 as libc::c_int
                    }
                }
                if (*gGameInfo).data[(17 as libc::c_int * 6 as libc::c_int *
                                          16 as libc::c_int +
                                          0 as libc::c_int) as usize] != 0 {
                    (*gGameInfo).data[(17 as libc::c_int * 6 as libc::c_int *
                                           16 as libc::c_int +
                                           0 as libc::c_int) as usize] =
                        0 as libc::c_int as s16;
                    func_800AA000(0 as libc::c_int as f32_0,
                                  (*gGameInfo).data[(17 as libc::c_int *
                                                         6 as libc::c_int *
                                                         16 as libc::c_int +
                                                         1 as libc::c_int) as
                                                        usize] as u8_0,
                                  (*gGameInfo).data[(17 as libc::c_int *
                                                         6 as libc::c_int *
                                                         16 as libc::c_int +
                                                         2 as libc::c_int) as
                                                        usize] as u8_0,
                                  (*gGameInfo).data[(17 as libc::c_int *
                                                         6 as libc::c_int *
                                                         16 as libc::c_int +
                                                         3 as libc::c_int) as
                                                        usize] as u8_0);
                }
            }
            _ => { }
        }
    };
}
// Draw Memory Viewer
#[no_mangle]
pub unsafe extern "C" fn func_80063C04(mut printer: *mut GfxPrint) {
    let mut i: s32 = 0;
    let mut page: s32 =
        (*gGameInfo).regPage * 16 as libc::c_int - 16 as libc::c_int;
    let mut regGroup: s32 =
        ((*gGameInfo).regGroup * 6 as libc::c_int + (*gGameInfo).regPage) *
            16 as libc::c_int - 16 as libc::c_int;
    let mut pad: s32 = 0;
    let mut name: [libc::c_char; 3] = [0; 3];
    // set up register name string
    name[0 as libc::c_int as usize] =
        'R' as i32 as libc::c_char; // r_group type char
    name[1 as libc::c_int as usize] = regChar[(*gGameInfo).regGroup as usize];
    name[2 as libc::c_int as usize] = '\u{0}' as i32 as libc::c_char;
    GfxPrint_SetColor(printer, 0 as libc::c_int as u32_0,
                      128 as libc::c_int as u32_0,
                      128 as libc::c_int as u32_0,
                      128 as libc::c_int as u32_0);
    i = 0 as libc::c_int;
    while i != 16 as libc::c_int {
        if i == (*gGameInfo).regCur {
            GfxPrint_SetColor(printer, 0 as libc::c_int as u32_0,
                              255 as libc::c_int as u32_0,
                              255 as libc::c_int as u32_0,
                              255 as libc::c_int as u32_0);
        }
        GfxPrint_SetPos(printer, 3 as libc::c_int, i + 5 as libc::c_int);
        GfxPrint_Printf(printer,
                        b"%s%02d%6d\x00" as *const u8 as *const libc::c_char,
                        &mut name as *mut [libc::c_char; 3], page + i,
                        (*gGameInfo).data[(i + regGroup) as usize] as
                            libc::c_int);
        if i == (*gGameInfo).regCur {
            GfxPrint_SetColor(printer, 0 as libc::c_int as u32_0,
                              128 as libc::c_int as u32_0,
                              128 as libc::c_int as u32_0,
                              128 as libc::c_int as u32_0);
        }
        i += 1
    };
}
#[no_mangle]
pub unsafe extern "C" fn func_80063D7C(mut gfxCtx: *mut GraphicsContext) {
    let mut sp7C: *mut Gfx = 0 as *mut Gfx;
    let mut sp78: *mut Gfx = 0 as *mut Gfx;
    let mut printer: GfxPrint =
        GfxPrint{callback: None,
                 dList: 0 as *mut Gfx,
                 posX: 0,
                 posY: 0,
                 baseX: 0,
                 baseY: 0,
                 flags: 0,
                 color:
                     Color_RGBA8_u32{c2rust_unnamed:
                                         C2RustUnnamed_0{r: 0,
                                                         g: 0,
                                                         b: 0,
                                                         a: 0,},},
                 unk_14: [0; 28],};
    let mut tempRet: *mut Gfx = 0 as *mut Gfx;
    let mut __gfxCtx: *mut GraphicsContext = 0 as *mut GraphicsContext;
    let mut dispRefs: [*mut Gfx; 4] = [0 as *mut Gfx; 4];
    __gfxCtx = gfxCtx;
    Graph_OpenDisps(dispRefs.as_mut_ptr(), gfxCtx,
                    b"../z_debug.c\x00" as *const u8 as *const libc::c_char,
                    628 as libc::c_int);
    GfxPrint_Init(&mut printer);
    sp78 = (*__gfxCtx).polyOpa.p;
    tempRet = Graph_GfxPlusOne((*__gfxCtx).polyOpa.p);
    let fresh3 = (*__gfxCtx).overlay.p;
    (*__gfxCtx).overlay.p = (*__gfxCtx).overlay.p.offset(1);
    let mut _g: *mut Gfx = fresh3;
    (*_g).words.w0 =
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
    (*_g).words.w1 = tempRet as libc::c_uint;
    GfxPrint_Open(&mut printer, tempRet);
    if (*gGameInfo).data[(2 as libc::c_int * 6 as libc::c_int *
                              16 as libc::c_int + 0 as libc::c_int) as usize]
           as libc::c_int == 1 as libc::c_int ||
           (*gGameInfo).data[(2 as libc::c_int * 6 as libc::c_int *
                                  16 as libc::c_int + 0 as libc::c_int) as
                                 usize] as libc::c_int == 8 as libc::c_int {
        func_80063828(&mut printer);
    }
    if (*gGameInfo).regPage != 0 as libc::c_int {
        func_80063C04(&mut printer);
    }
    D_8011E0B0 = 0 as libc::c_int as s16;
    sp7C = GfxPrint_Close(&mut printer);
    let fresh4 = sp7C;
    sp7C = sp7C.offset(1);
    let mut _g_0: *mut Gfx = fresh4;
    (*_g_0).words.w0 =
        (0xdf as libc::c_int as u32_0 &
             (((0x1 as libc::c_int) << 8 as libc::c_int) - 1 as libc::c_int)
                 as libc::c_uint) << 24 as libc::c_int;
    (*_g_0).words.w1 = 0 as libc::c_int as libc::c_uint;
    Graph_BranchDlist(sp78, sp7C);
    (*__gfxCtx).polyOpa.p = sp7C;
    Graph_CloseDisps(dispRefs.as_mut_ptr(), gfxCtx,
                     b"../z_debug.c\x00" as *const u8 as *const libc::c_char,
                     664 as libc::c_int);
    GfxPrint_Destroy(&mut printer);
}
