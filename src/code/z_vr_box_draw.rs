#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case,
         non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(const_raw_ptr_to_usize_cast, register_tool)]
extern "C" {
    #[no_mangle]
    fn func_800945A0(gfxCtx: *mut GraphicsContext);
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
    fn Matrix_Translate(x: f32_0, y: f32_0, z: f32_0, mode: u8_0);
    #[no_mangle]
    fn Matrix_Scale(x: f32_0, y: f32_0, z: f32_0, mode: u8_0);
    #[no_mangle]
    fn Matrix_RotateX(x: f32_0, mode: u8_0);
    #[no_mangle]
    fn Matrix_RotateY(y: f32_0, mode: u8_0);
    #[no_mangle]
    fn Matrix_RotateZ(z: f32_0, mode: u8_0);
    #[no_mangle]
    fn Matrix_ToMtx(dest: *mut Mtx, file: *mut libc::c_char, line: s32)
     -> *mut Mtx;
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
pub type size_t = libc::c_ulong;
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
pub type C2RustUnnamed_1 = libc::c_uint;
pub const SKYBOX_UNSET_27: C2RustUnnamed_1 = 39;
pub const SKYBOX_HOUSE_ALLEY: C2RustUnnamed_1 = 34;
pub const SKYBOX_HOUSE_SARIA: C2RustUnnamed_1 = 33;
pub const SKYBOX_HOUSE_MIDO: C2RustUnnamed_1 = 32;
pub const SKYBOX_UNSET_1D: C2RustUnnamed_1 = 29;
pub const SKYBOX_TENT: C2RustUnnamed_1 = 28;
pub const SKYBOX_HOUSE_IMPA: C2RustUnnamed_1 = 27;
pub const SKYBOX_HOUSE_RICHARD: C2RustUnnamed_1 = 26;
pub const SKYBOX_BOMBCHU_SHOP: C2RustUnnamed_1 = 24;
pub const SKYBOX_POTION_SHOP_MARKET: C2RustUnnamed_1 = 23;
pub const SKYBOX_POTION_SHOP_KAKARIKO: C2RustUnnamed_1 = 22;
pub const SKYBOX_ZORA_SHOP: C2RustUnnamed_1 = 20;
pub const SKYBOX_GORON_SHOP: C2RustUnnamed_1 = 19;
pub const SKYBOX_KOKIRI_SHOP: C2RustUnnamed_1 = 17;
pub const SKYBOX_HOUSE_KAKARIKO: C2RustUnnamed_1 = 16;
pub const SKYBOX_STABLES: C2RustUnnamed_1 = 15;
pub const SKYBOX_HOUSE_OF_TWINS: C2RustUnnamed_1 = 14;
pub const SKYBOX_HOUSE_KNOW_IT_ALL_BROTHERS: C2RustUnnamed_1 = 12;
pub const SKYBOX_HAPPY_MASK_SHOP: C2RustUnnamed_1 = 11;
pub const SKYBOX_MARKET_CHILD_NIGHT: C2RustUnnamed_1 = 10;
pub const SKYBOX_MARKET_CHILD_DAY: C2RustUnnamed_1 = 9;
pub const SKYBOX_HOUSE_LINK: C2RustUnnamed_1 = 7;
pub const SKYBOX_CUTSCENE_MAP: C2RustUnnamed_1 = 5;
pub const SKYBOX_MARKET_ADULT: C2RustUnnamed_1 = 4;
pub const SKYBOX_OVERCAST_SUNSET: C2RustUnnamed_1 = 3;
pub const SKYBOX_BAZAAR: C2RustUnnamed_1 = 2;
pub const SKYBOX_NORMAL_SKY: C2RustUnnamed_1 = 1;
pub const SKYBOX_NONE: C2RustUnnamed_1 = 0;
pub type C2RustUnnamed_2 = libc::c_uint;
pub const MTXMODE_APPLY: C2RustUnnamed_2 = 1;
pub const MTXMODE_NEW: C2RustUnnamed_2 = 0;
#[no_mangle]
pub static mut sSkyboxDrawMatrix: *mut Mtx = 0 as *const Mtx as *mut Mtx;
#[no_mangle]
pub unsafe extern "C" fn SkyboxDraw_UpdateMatrix(mut skyboxCtx:
                                                     *mut SkyboxContext,
                                                 mut x: f32_0, mut y: f32_0,
                                                 mut z: f32_0) -> *mut Mtx {
    Matrix_Translate(x, y, z, MTXMODE_NEW as libc::c_int as u8_0);
    Matrix_Scale(1.0f32, 1.0f32, 1.0f32,
                 MTXMODE_APPLY as libc::c_int as u8_0);
    Matrix_RotateX((*skyboxCtx).rot.x, MTXMODE_APPLY as libc::c_int as u8_0);
    Matrix_RotateY((*skyboxCtx).rot.y, MTXMODE_APPLY as libc::c_int as u8_0);
    Matrix_RotateZ((*skyboxCtx).rot.z, MTXMODE_APPLY as libc::c_int as u8_0);
    return Matrix_ToMtx(sSkyboxDrawMatrix,
                        b"../z_vr_box_draw.c\x00" as *const u8 as
                            *const libc::c_char as *mut libc::c_char,
                        42 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn SkyboxDraw_Draw(mut skyboxCtx: *mut SkyboxContext,
                                         mut gfxCtx: *mut GraphicsContext,
                                         mut skyboxId: s16, mut blend: s16,
                                         mut x: f32_0, mut y: f32_0,
                                         mut z: f32_0) {
    let mut __gfxCtx: *mut GraphicsContext = 0 as *mut GraphicsContext;
    let mut dispRefs: [*mut Gfx; 4] = [0 as *mut Gfx; 4];
    __gfxCtx = gfxCtx;
    Graph_OpenDisps(dispRefs.as_mut_ptr(), gfxCtx,
                    b"../z_vr_box_draw.c\x00" as *const u8 as
                        *const libc::c_char, 52 as libc::c_int);
    func_800945A0(gfxCtx);
    let fresh0 = (*__gfxCtx).polyOpa.p;
    (*__gfxCtx).polyOpa.p = (*__gfxCtx).polyOpa.p.offset(1);
    let mut _g: *mut Gfx = fresh0;
    (*_g).words.w0 =
        (0xdb as libc::c_int as u32_0 &
             (((0x1 as libc::c_int) << 8 as libc::c_int) - 1 as libc::c_int)
                 as libc::c_uint) << 24 as libc::c_int |
            (0x6 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 16 as libc::c_int
            |
            ((0x7 as libc::c_int * 4 as libc::c_int) as u32_0 &
                 (((0x1 as libc::c_int) << 16 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 0 as libc::c_int;
    (*_g).words.w1 =
        (*skyboxCtx).staticSegments[0 as libc::c_int as usize] as
            libc::c_uint;
    let fresh1 = (*__gfxCtx).polyOpa.p;
    (*__gfxCtx).polyOpa.p = (*__gfxCtx).polyOpa.p.offset(1);
    let mut _g_0: *mut Gfx = fresh1;
    (*_g_0).words.w0 =
        (0xdb as libc::c_int as u32_0 &
             (((0x1 as libc::c_int) << 8 as libc::c_int) - 1 as libc::c_int)
                 as libc::c_uint) << 24 as libc::c_int |
            (0x6 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 16 as libc::c_int
            |
            ((0x8 as libc::c_int * 4 as libc::c_int) as u32_0 &
                 (((0x1 as libc::c_int) << 16 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 0 as libc::c_int;
    (*_g_0).words.w1 =
        (*skyboxCtx).staticSegments[1 as libc::c_int as usize] as
            libc::c_uint;
    let fresh2 = (*__gfxCtx).polyOpa.p;
    (*__gfxCtx).polyOpa.p = (*__gfxCtx).polyOpa.p.offset(1);
    let mut _g_1: *mut Gfx = fresh2;
    (*_g_1).words.w0 =
        (0xdb as libc::c_int as u32_0 &
             (((0x1 as libc::c_int) << 8 as libc::c_int) - 1 as libc::c_int)
                 as libc::c_uint) << 24 as libc::c_int |
            (0x6 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 16 as libc::c_int
            |
            ((0x9 as libc::c_int * 4 as libc::c_int) as u32_0 &
                 (((0x1 as libc::c_int) << 16 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 0 as libc::c_int;
    (*_g_1).words.w1 = (*skyboxCtx).palettes as libc::c_uint;
    let fresh3 = (*__gfxCtx).polyOpa.p;
    (*__gfxCtx).polyOpa.p = (*__gfxCtx).polyOpa.p.offset(1);
    let mut _g_2: *mut Gfx = fresh3;
    (*_g_2).words.w0 =
        (0xfa as libc::c_int as u32_0 &
             (((0x1 as libc::c_int) << 8 as libc::c_int) - 1 as libc::c_int)
                 as libc::c_uint) << 24 as libc::c_int |
            (0 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 8 as libc::c_int |
            (0 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 0 as libc::c_int;
    (*_g_2).words.w1 =
        (0 as libc::c_int as u32_0 &
             (((0x1 as libc::c_int) << 8 as libc::c_int) - 1 as libc::c_int)
                 as libc::c_uint) << 24 as libc::c_int |
            (0 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 16 as libc::c_int
            |
            (0 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 8 as libc::c_int |
            (blend as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 0 as libc::c_int;
    let fresh4 = (*__gfxCtx).polyOpa.p;
    (*__gfxCtx).polyOpa.p = (*__gfxCtx).polyOpa.p.offset(1);
    let mut _g_3: *mut Gfx = fresh4;
    (*_g_3).words.w0 =
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
    (*_g_3).words.w1 =
        (0x8000 as libc::c_int as u32_0 &
             (((0x1 as libc::c_int) << 16 as libc::c_int) - 1 as libc::c_int)
                 as libc::c_uint) << 16 as libc::c_int |
            (0x8000 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 16 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 0 as libc::c_int;
    sSkyboxDrawMatrix =
        Graph_Alloc(gfxCtx,
                    ::std::mem::size_of::<Mtx>() as libc::c_ulong as size_t)
            as *mut Mtx;
    Matrix_Translate(x, y, z, MTXMODE_NEW as libc::c_int as u8_0);
    Matrix_Scale(1.0f32, 1.0f32, 1.0f32,
                 MTXMODE_APPLY as libc::c_int as u8_0);
    Matrix_RotateX((*skyboxCtx).rot.x, MTXMODE_APPLY as libc::c_int as u8_0);
    Matrix_RotateY((*skyboxCtx).rot.y, MTXMODE_APPLY as libc::c_int as u8_0);
    Matrix_RotateZ((*skyboxCtx).rot.z, MTXMODE_APPLY as libc::c_int as u8_0);
    Matrix_ToMtx(sSkyboxDrawMatrix,
                 b"../z_vr_box_draw.c\x00" as *const u8 as *const libc::c_char
                     as *mut libc::c_char, 76 as libc::c_int);
    let fresh5 = (*__gfxCtx).polyOpa.p;
    (*__gfxCtx).polyOpa.p = (*__gfxCtx).polyOpa.p.offset(1);
    let mut _g_4: *mut Gfx = fresh5;
    (*_g_4).words.w0 =
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
    (*_g_4).words.w1 = sSkyboxDrawMatrix as libc::c_uint;
    let fresh6 = (*__gfxCtx).polyOpa.p;
    (*__gfxCtx).polyOpa.p = (*__gfxCtx).polyOpa.p.offset(1);
    let mut _g_5: *mut Gfx = fresh6;
    (*_g_5).words.w0 =
        (0xe3 as libc::c_int as u32_0 &
             (((0x1 as libc::c_int) << 8 as libc::c_int) - 1 as libc::c_int)
                 as libc::c_uint) << 24 as libc::c_int |
            ((32 as libc::c_int - 6 as libc::c_int - 2 as libc::c_int) as
                 u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 8 as libc::c_int |
            ((2 as libc::c_int - 1 as libc::c_int) as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 0 as libc::c_int;
    (*_g_5).words.w1 =
        ((0 as libc::c_int) << 6 as libc::c_int) as libc::c_uint;
    let fresh7 = (*__gfxCtx).polyOpa.p;
    (*__gfxCtx).polyOpa.p = (*__gfxCtx).polyOpa.p.offset(1);
    let mut _g_6: *mut Gfx = fresh7;
    (*_g_6).words.w0 =
        (0xe3 as libc::c_int as u32_0 &
             (((0x1 as libc::c_int) << 8 as libc::c_int) - 1 as libc::c_int)
                 as libc::c_uint) << 24 as libc::c_int |
            ((32 as libc::c_int - 12 as libc::c_int - 2 as libc::c_int) as
                 u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 8 as libc::c_int |
            ((2 as libc::c_int - 1 as libc::c_int) as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 0 as libc::c_int;
    (*_g_6).words.w1 =
        ((2 as libc::c_int) << 12 as libc::c_int) as libc::c_uint;
    let fresh8 = (*__gfxCtx).polyOpa.p;
    (*__gfxCtx).polyOpa.p = (*__gfxCtx).polyOpa.p.offset(1);
    let mut _g_7: *mut Gfx = fresh8;
    (*_g_7).words.w0 =
        (0xfd as libc::c_int as u32_0 &
             (((0x1 as libc::c_int) << 8 as libc::c_int) - 1 as libc::c_int)
                 as libc::c_uint) << 24 as libc::c_int |
            (0 as libc::c_int as u32_0 &
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
    (*_g_7).words.w1 =
        (*(*skyboxCtx).palettes.offset(0 as libc::c_int as
                                           isize)).as_mut_ptr() as
            libc::c_uint;
    let fresh9 = (*__gfxCtx).polyOpa.p;
    (*__gfxCtx).polyOpa.p = (*__gfxCtx).polyOpa.p.offset(1);
    let mut _g_8: *mut Gfx = fresh9;
    (*_g_8).words.w0 =
        (0xe8 as libc::c_int as u32_0 &
             (((0x1 as libc::c_int) << 8 as libc::c_int) - 1 as libc::c_int)
                 as libc::c_uint) << 24 as libc::c_int;
    (*_g_8).words.w1 = 0 as libc::c_int as libc::c_uint;
    let fresh10 = (*__gfxCtx).polyOpa.p;
    (*__gfxCtx).polyOpa.p = (*__gfxCtx).polyOpa.p.offset(1);
    let mut _g_9: *mut Gfx = fresh10;
    (*_g_9).words.w0 =
        (0xf5 as libc::c_int as u32_0 &
             (((0x1 as libc::c_int) << 8 as libc::c_int) - 1 as libc::c_int)
                 as libc::c_uint) << 24 as libc::c_int |
            (0 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 3 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 21 as libc::c_int
            |
            (0 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 2 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 19 as libc::c_int
            |
            (0 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 9 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 9 as libc::c_int |
            (256 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 9 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 0 as libc::c_int;
    (*_g_9).words.w1 =
        (7 as libc::c_int as u32_0 &
             (((0x1 as libc::c_int) << 3 as libc::c_int) - 1 as libc::c_int)
                 as libc::c_uint) << 24 as libc::c_int |
            (0 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 4 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 20 as libc::c_int
            |
            (0 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 2 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 18 as libc::c_int
            |
            (0 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 4 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 14 as libc::c_int
            |
            (0 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 4 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 10 as libc::c_int
            |
            (0 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 2 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 8 as libc::c_int |
            (0 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 4 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 4 as libc::c_int |
            (0 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 4 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 0 as libc::c_int;
    let fresh11 = (*__gfxCtx).polyOpa.p;
    (*__gfxCtx).polyOpa.p = (*__gfxCtx).polyOpa.p.offset(1);
    let mut _g_10: *mut Gfx = fresh11;
    (*_g_10).words.w0 =
        (0xe6 as libc::c_int as u32_0 &
             (((0x1 as libc::c_int) << 8 as libc::c_int) - 1 as libc::c_int)
                 as libc::c_uint) << 24 as libc::c_int;
    (*_g_10).words.w1 = 0 as libc::c_int as libc::c_uint;
    let fresh12 = (*__gfxCtx).polyOpa.p;
    (*__gfxCtx).polyOpa.p = (*__gfxCtx).polyOpa.p.offset(1);
    let mut _g_11: *mut Gfx = fresh12;
    (*_g_11).words.w0 =
        (0xf0 as libc::c_int as u32_0 &
             (((0x1 as libc::c_int) << 8 as libc::c_int) - 1 as libc::c_int)
                 as libc::c_uint) << 24 as libc::c_int;
    (*_g_11).words.w1 =
        (7 as libc::c_int as u32_0 &
             (((0x1 as libc::c_int) << 3 as libc::c_int) - 1 as libc::c_int)
                 as libc::c_uint) << 24 as libc::c_int |
            (255 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 10 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 14 as libc::c_int;
    let fresh13 = (*__gfxCtx).polyOpa.p;
    (*__gfxCtx).polyOpa.p = (*__gfxCtx).polyOpa.p.offset(1);
    let mut _g_12: *mut Gfx = fresh13;
    (*_g_12).words.w0 =
        (0xe7 as libc::c_int as u32_0 &
             (((0x1 as libc::c_int) << 8 as libc::c_int) - 1 as libc::c_int)
                 as libc::c_uint) << 24 as libc::c_int;
    (*_g_12).words.w1 = 0 as libc::c_int as libc::c_uint;
    let fresh14 = (*__gfxCtx).polyOpa.p;
    (*__gfxCtx).polyOpa.p = (*__gfxCtx).polyOpa.p.offset(1);
    let mut _g_13: *mut Gfx = fresh14;
    (*_g_13).words.w0 =
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
    (*_g_13).words.w1 =
        ((2 as libc::c_int) << 14 as libc::c_int) as libc::c_uint;
    let fresh15 = (*__gfxCtx).polyOpa.p;
    (*__gfxCtx).polyOpa.p = (*__gfxCtx).polyOpa.p.offset(1);
    let mut _g_14: *mut Gfx = fresh15;
    (*_g_14).words.w0 =
        (0xe3 as libc::c_int as u32_0 &
             (((0x1 as libc::c_int) << 8 as libc::c_int) - 1 as libc::c_int)
                 as libc::c_uint) << 24 as libc::c_int |
            ((32 as libc::c_int - 9 as libc::c_int - 3 as libc::c_int) as
                 u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 8 as libc::c_int |
            ((3 as libc::c_int - 1 as libc::c_int) as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 0 as libc::c_int;
    (*_g_14).words.w1 =
        ((6 as libc::c_int) << 9 as libc::c_int) as libc::c_uint;
    if (*skyboxCtx).unk_140 != 0 {
        let fresh16 = (*__gfxCtx).polyOpa.p;
        (*__gfxCtx).polyOpa.p = (*__gfxCtx).polyOpa.p.offset(1);
        let mut _g_15: *mut Gfx = fresh16;
        (*_g_15).words.w0 =
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
        (*_g_15).words.w1 =
            (*(*skyboxCtx).dListBuf.offset(0 as libc::c_int as
                                               isize)).as_mut_ptr() as
                libc::c_uint;
        let fresh17 = (*__gfxCtx).polyOpa.p;
        (*__gfxCtx).polyOpa.p = (*__gfxCtx).polyOpa.p.offset(1);
        let mut _g_16: *mut Gfx = fresh17;
        (*_g_16).words.w0 =
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
        (*_g_16).words.w1 =
            (*(*skyboxCtx).dListBuf.offset(1 as libc::c_int as
                                               isize)).as_mut_ptr() as
                libc::c_uint;
        let fresh18 = (*__gfxCtx).polyOpa.p;
        (*__gfxCtx).polyOpa.p = (*__gfxCtx).polyOpa.p.offset(1);
        let mut _g_17: *mut Gfx = fresh18;
        (*_g_17).words.w0 =
            (0xe7 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 24 as libc::c_int;
        (*_g_17).words.w1 = 0 as libc::c_int as libc::c_uint;
        let fresh19 = (*__gfxCtx).polyOpa.p;
        (*__gfxCtx).polyOpa.p = (*__gfxCtx).polyOpa.p.offset(1);
        let mut _g_18: *mut Gfx = fresh19;
        (*_g_18).words.w0 =
            (0xfd as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 24 as libc::c_int
                |
                (0 as libc::c_int as u32_0 &
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
        (*_g_18).words.w1 =
            (*(*skyboxCtx).palettes.offset(1 as libc::c_int as
                                               isize)).as_mut_ptr() as
                libc::c_uint;
        let fresh20 = (*__gfxCtx).polyOpa.p;
        (*__gfxCtx).polyOpa.p = (*__gfxCtx).polyOpa.p.offset(1);
        let mut _g_19: *mut Gfx = fresh20;
        (*_g_19).words.w0 =
            (0xe8 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 24 as libc::c_int;
        (*_g_19).words.w1 = 0 as libc::c_int as libc::c_uint;
        let fresh21 = (*__gfxCtx).polyOpa.p;
        (*__gfxCtx).polyOpa.p = (*__gfxCtx).polyOpa.p.offset(1);
        let mut _g_20: *mut Gfx = fresh21;
        (*_g_20).words.w0 =
            (0xf5 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 24 as libc::c_int
                |
                (0 as libc::c_int as u32_0 &
                     (((0x1 as libc::c_int) << 3 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    21 as libc::c_int |
                (0 as libc::c_int as u32_0 &
                     (((0x1 as libc::c_int) << 2 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    19 as libc::c_int |
                (0 as libc::c_int as u32_0 &
                     (((0x1 as libc::c_int) << 9 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    9 as libc::c_int |
                (256 as libc::c_int as u32_0 &
                     (((0x1 as libc::c_int) << 9 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    0 as libc::c_int;
        (*_g_20).words.w1 =
            (7 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 3 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 24 as libc::c_int
                |
                (0 as libc::c_int as u32_0 &
                     (((0x1 as libc::c_int) << 4 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    20 as libc::c_int |
                (0 as libc::c_int as u32_0 &
                     (((0x1 as libc::c_int) << 2 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    18 as libc::c_int |
                (0 as libc::c_int as u32_0 &
                     (((0x1 as libc::c_int) << 4 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    14 as libc::c_int |
                (0 as libc::c_int as u32_0 &
                     (((0x1 as libc::c_int) << 4 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    10 as libc::c_int |
                (0 as libc::c_int as u32_0 &
                     (((0x1 as libc::c_int) << 2 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    8 as libc::c_int |
                (0 as libc::c_int as u32_0 &
                     (((0x1 as libc::c_int) << 4 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    4 as libc::c_int |
                (0 as libc::c_int as u32_0 &
                     (((0x1 as libc::c_int) << 4 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    0 as libc::c_int;
        let fresh22 = (*__gfxCtx).polyOpa.p;
        (*__gfxCtx).polyOpa.p = (*__gfxCtx).polyOpa.p.offset(1);
        let mut _g_21: *mut Gfx = fresh22;
        (*_g_21).words.w0 =
            (0xe6 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 24 as libc::c_int;
        (*_g_21).words.w1 = 0 as libc::c_int as libc::c_uint;
        let fresh23 = (*__gfxCtx).polyOpa.p;
        (*__gfxCtx).polyOpa.p = (*__gfxCtx).polyOpa.p.offset(1);
        let mut _g_22: *mut Gfx = fresh23;
        (*_g_22).words.w0 =
            (0xf0 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 24 as libc::c_int;
        (*_g_22).words.w1 =
            (7 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 3 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 24 as libc::c_int
                |
                (255 as libc::c_int as u32_0 &
                     (((0x1 as libc::c_int) << 10 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    14 as libc::c_int;
        let fresh24 = (*__gfxCtx).polyOpa.p;
        (*__gfxCtx).polyOpa.p = (*__gfxCtx).polyOpa.p.offset(1);
        let mut _g_23: *mut Gfx = fresh24;
        (*_g_23).words.w0 =
            (0xe7 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 24 as libc::c_int;
        (*_g_23).words.w1 = 0 as libc::c_int as libc::c_uint;
        let fresh25 = (*__gfxCtx).polyOpa.p;
        (*__gfxCtx).polyOpa.p = (*__gfxCtx).polyOpa.p.offset(1);
        let mut _g_24: *mut Gfx = fresh25;
        (*_g_24).words.w0 =
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
        (*_g_24).words.w1 =
            (*(*skyboxCtx).dListBuf.offset(2 as libc::c_int as
                                               isize)).as_mut_ptr() as
                libc::c_uint;
        let fresh26 = (*__gfxCtx).polyOpa.p;
        (*__gfxCtx).polyOpa.p = (*__gfxCtx).polyOpa.p.offset(1);
        let mut _g_25: *mut Gfx = fresh26;
        (*_g_25).words.w0 =
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
        (*_g_25).words.w1 =
            (*(*skyboxCtx).dListBuf.offset(3 as libc::c_int as
                                               isize)).as_mut_ptr() as
                libc::c_uint;
        if skyboxId as libc::c_int != SKYBOX_BAZAAR as libc::c_int {
            if skyboxId as libc::c_int <= SKYBOX_HOUSE_KAKARIKO as libc::c_int
                   ||
                   skyboxId as libc::c_int >
                       SKYBOX_BOMBCHU_SHOP as libc::c_int {
                let fresh27 = (*__gfxCtx).polyOpa.p;
                (*__gfxCtx).polyOpa.p = (*__gfxCtx).polyOpa.p.offset(1);
                let mut _g_26: *mut Gfx = fresh27;
                (*_g_26).words.w0 =
                    (0xe7 as libc::c_int as u32_0 &
                         (((0x1 as libc::c_int) << 8 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        24 as libc::c_int;
                (*_g_26).words.w1 = 0 as libc::c_int as libc::c_uint;
                let fresh28 = (*__gfxCtx).polyOpa.p;
                (*__gfxCtx).polyOpa.p = (*__gfxCtx).polyOpa.p.offset(1);
                let mut _g_27: *mut Gfx = fresh28;
                (*_g_27).words.w0 =
                    (0xfd as libc::c_int as u32_0 &
                         (((0x1 as libc::c_int) << 8 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        24 as libc::c_int |
                        (0 as libc::c_int as u32_0 &
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
                (*_g_27).words.w1 =
                    (*(*skyboxCtx).palettes.offset(2 as libc::c_int as
                                                       isize)).as_mut_ptr() as
                        libc::c_uint;
                let fresh29 = (*__gfxCtx).polyOpa.p;
                (*__gfxCtx).polyOpa.p = (*__gfxCtx).polyOpa.p.offset(1);
                let mut _g_28: *mut Gfx = fresh29;
                (*_g_28).words.w0 =
                    (0xe8 as libc::c_int as u32_0 &
                         (((0x1 as libc::c_int) << 8 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        24 as libc::c_int;
                (*_g_28).words.w1 = 0 as libc::c_int as libc::c_uint;
                let fresh30 = (*__gfxCtx).polyOpa.p;
                (*__gfxCtx).polyOpa.p = (*__gfxCtx).polyOpa.p.offset(1);
                let mut _g_29: *mut Gfx = fresh30;
                (*_g_29).words.w0 =
                    (0xf5 as libc::c_int as u32_0 &
                         (((0x1 as libc::c_int) << 8 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        24 as libc::c_int |
                        (0 as libc::c_int as u32_0 &
                             (((0x1 as libc::c_int) << 3 as libc::c_int) -
                                  1 as libc::c_int) as libc::c_uint) <<
                            21 as libc::c_int |
                        (0 as libc::c_int as u32_0 &
                             (((0x1 as libc::c_int) << 2 as libc::c_int) -
                                  1 as libc::c_int) as libc::c_uint) <<
                            19 as libc::c_int |
                        (0 as libc::c_int as u32_0 &
                             (((0x1 as libc::c_int) << 9 as libc::c_int) -
                                  1 as libc::c_int) as libc::c_uint) <<
                            9 as libc::c_int |
                        (256 as libc::c_int as u32_0 &
                             (((0x1 as libc::c_int) << 9 as libc::c_int) -
                                  1 as libc::c_int) as libc::c_uint) <<
                            0 as libc::c_int;
                (*_g_29).words.w1 =
                    (7 as libc::c_int as u32_0 &
                         (((0x1 as libc::c_int) << 3 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        24 as libc::c_int |
                        (0 as libc::c_int as u32_0 &
                             (((0x1 as libc::c_int) << 4 as libc::c_int) -
                                  1 as libc::c_int) as libc::c_uint) <<
                            20 as libc::c_int |
                        (0 as libc::c_int as u32_0 &
                             (((0x1 as libc::c_int) << 2 as libc::c_int) -
                                  1 as libc::c_int) as libc::c_uint) <<
                            18 as libc::c_int |
                        (0 as libc::c_int as u32_0 &
                             (((0x1 as libc::c_int) << 4 as libc::c_int) -
                                  1 as libc::c_int) as libc::c_uint) <<
                            14 as libc::c_int |
                        (0 as libc::c_int as u32_0 &
                             (((0x1 as libc::c_int) << 4 as libc::c_int) -
                                  1 as libc::c_int) as libc::c_uint) <<
                            10 as libc::c_int |
                        (0 as libc::c_int as u32_0 &
                             (((0x1 as libc::c_int) << 2 as libc::c_int) -
                                  1 as libc::c_int) as libc::c_uint) <<
                            8 as libc::c_int |
                        (0 as libc::c_int as u32_0 &
                             (((0x1 as libc::c_int) << 4 as libc::c_int) -
                                  1 as libc::c_int) as libc::c_uint) <<
                            4 as libc::c_int |
                        (0 as libc::c_int as u32_0 &
                             (((0x1 as libc::c_int) << 4 as libc::c_int) -
                                  1 as libc::c_int) as libc::c_uint) <<
                            0 as libc::c_int;
                let fresh31 = (*__gfxCtx).polyOpa.p;
                (*__gfxCtx).polyOpa.p = (*__gfxCtx).polyOpa.p.offset(1);
                let mut _g_30: *mut Gfx = fresh31;
                (*_g_30).words.w0 =
                    (0xe6 as libc::c_int as u32_0 &
                         (((0x1 as libc::c_int) << 8 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        24 as libc::c_int;
                (*_g_30).words.w1 = 0 as libc::c_int as libc::c_uint;
                let fresh32 = (*__gfxCtx).polyOpa.p;
                (*__gfxCtx).polyOpa.p = (*__gfxCtx).polyOpa.p.offset(1);
                let mut _g_31: *mut Gfx = fresh32;
                (*_g_31).words.w0 =
                    (0xf0 as libc::c_int as u32_0 &
                         (((0x1 as libc::c_int) << 8 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        24 as libc::c_int;
                (*_g_31).words.w1 =
                    (7 as libc::c_int as u32_0 &
                         (((0x1 as libc::c_int) << 3 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        24 as libc::c_int |
                        (255 as libc::c_int as u32_0 &
                             (((0x1 as libc::c_int) << 10 as libc::c_int) -
                                  1 as libc::c_int) as libc::c_uint) <<
                            14 as libc::c_int;
                let fresh33 = (*__gfxCtx).polyOpa.p;
                (*__gfxCtx).polyOpa.p = (*__gfxCtx).polyOpa.p.offset(1);
                let mut _g_32: *mut Gfx = fresh33;
                (*_g_32).words.w0 =
                    (0xe7 as libc::c_int as u32_0 &
                         (((0x1 as libc::c_int) << 8 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        24 as libc::c_int;
                (*_g_32).words.w1 = 0 as libc::c_int as libc::c_uint;
                let fresh34 = (*__gfxCtx).polyOpa.p;
                (*__gfxCtx).polyOpa.p = (*__gfxCtx).polyOpa.p.offset(1);
                let mut _g_33: *mut Gfx = fresh34;
                (*_g_33).words.w0 =
                    (0xde as libc::c_int as u32_0 &
                         (((0x1 as libc::c_int) << 8 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        24 as libc::c_int |
                        (0 as libc::c_int as u32_0 &
                             (((0x1 as libc::c_int) << 8 as libc::c_int) -
                                  1 as libc::c_int) as libc::c_uint) <<
                            16 as libc::c_int |
                        (0 as libc::c_int as u32_0 &
                             (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                  1 as libc::c_int) as libc::c_uint) <<
                            0 as libc::c_int;
                (*_g_33).words.w1 =
                    (*(*skyboxCtx).dListBuf.offset(4 as libc::c_int as
                                                       isize)).as_mut_ptr() as
                        libc::c_uint;
                let fresh35 = (*__gfxCtx).polyOpa.p;
                (*__gfxCtx).polyOpa.p = (*__gfxCtx).polyOpa.p.offset(1);
                let mut _g_34: *mut Gfx = fresh35;
                (*_g_34).words.w0 =
                    (0xde as libc::c_int as u32_0 &
                         (((0x1 as libc::c_int) << 8 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        24 as libc::c_int |
                        (0 as libc::c_int as u32_0 &
                             (((0x1 as libc::c_int) << 8 as libc::c_int) -
                                  1 as libc::c_int) as libc::c_uint) <<
                            16 as libc::c_int |
                        (0 as libc::c_int as u32_0 &
                             (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                  1 as libc::c_int) as libc::c_uint) <<
                            0 as libc::c_int;
                (*_g_34).words.w1 =
                    (*(*skyboxCtx).dListBuf.offset(5 as libc::c_int as
                                                       isize)).as_mut_ptr() as
                        libc::c_uint;
                let fresh36 = (*__gfxCtx).polyOpa.p;
                (*__gfxCtx).polyOpa.p = (*__gfxCtx).polyOpa.p.offset(1);
                let mut _g_35: *mut Gfx = fresh36;
                (*_g_35).words.w0 =
                    (0xe7 as libc::c_int as u32_0 &
                         (((0x1 as libc::c_int) << 8 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        24 as libc::c_int;
                (*_g_35).words.w1 = 0 as libc::c_int as libc::c_uint;
                if (*skyboxCtx).unk_140 as libc::c_int != 2 as libc::c_int {
                    let fresh37 = (*__gfxCtx).polyOpa.p;
                    (*__gfxCtx).polyOpa.p = (*__gfxCtx).polyOpa.p.offset(1);
                    let mut _g_36: *mut Gfx = fresh37;
                    (*_g_36).words.w0 =
                        (0xfd as libc::c_int as u32_0 &
                             (((0x1 as libc::c_int) << 8 as libc::c_int) -
                                  1 as libc::c_int) as libc::c_uint) <<
                            24 as libc::c_int |
                            (0 as libc::c_int as u32_0 &
                                 (((0x1 as libc::c_int) << 3 as libc::c_int) -
                                      1 as libc::c_int) as libc::c_uint) <<
                                21 as libc::c_int |
                            (2 as libc::c_int as u32_0 &
                                 (((0x1 as libc::c_int) << 2 as libc::c_int) -
                                      1 as libc::c_int) as libc::c_uint) <<
                                19 as libc::c_int |
                            ((1 as libc::c_int - 1 as libc::c_int) as u32_0 &
                                 (((0x1 as libc::c_int) << 12 as libc::c_int)
                                      - 1 as libc::c_int) as libc::c_uint) <<
                                0 as libc::c_int;
                    (*_g_36).words.w1 =
                        (*(*skyboxCtx).palettes.offset(3 as libc::c_int as
                                                           isize)).as_mut_ptr()
                            as libc::c_uint;
                    let fresh38 = (*__gfxCtx).polyOpa.p;
                    (*__gfxCtx).polyOpa.p = (*__gfxCtx).polyOpa.p.offset(1);
                    let mut _g_37: *mut Gfx = fresh38;
                    (*_g_37).words.w0 =
                        (0xe8 as libc::c_int as u32_0 &
                             (((0x1 as libc::c_int) << 8 as libc::c_int) -
                                  1 as libc::c_int) as libc::c_uint) <<
                            24 as libc::c_int;
                    (*_g_37).words.w1 = 0 as libc::c_int as libc::c_uint;
                    let fresh39 = (*__gfxCtx).polyOpa.p;
                    (*__gfxCtx).polyOpa.p = (*__gfxCtx).polyOpa.p.offset(1);
                    let mut _g_38: *mut Gfx = fresh39;
                    (*_g_38).words.w0 =
                        (0xf5 as libc::c_int as u32_0 &
                             (((0x1 as libc::c_int) << 8 as libc::c_int) -
                                  1 as libc::c_int) as libc::c_uint) <<
                            24 as libc::c_int |
                            (0 as libc::c_int as u32_0 &
                                 (((0x1 as libc::c_int) << 3 as libc::c_int) -
                                      1 as libc::c_int) as libc::c_uint) <<
                                21 as libc::c_int |
                            (0 as libc::c_int as u32_0 &
                                 (((0x1 as libc::c_int) << 2 as libc::c_int) -
                                      1 as libc::c_int) as libc::c_uint) <<
                                19 as libc::c_int |
                            (0 as libc::c_int as u32_0 &
                                 (((0x1 as libc::c_int) << 9 as libc::c_int) -
                                      1 as libc::c_int) as libc::c_uint) <<
                                9 as libc::c_int |
                            (256 as libc::c_int as u32_0 &
                                 (((0x1 as libc::c_int) << 9 as libc::c_int) -
                                      1 as libc::c_int) as libc::c_uint) <<
                                0 as libc::c_int;
                    (*_g_38).words.w1 =
                        (7 as libc::c_int as u32_0 &
                             (((0x1 as libc::c_int) << 3 as libc::c_int) -
                                  1 as libc::c_int) as libc::c_uint) <<
                            24 as libc::c_int |
                            (0 as libc::c_int as u32_0 &
                                 (((0x1 as libc::c_int) << 4 as libc::c_int) -
                                      1 as libc::c_int) as libc::c_uint) <<
                                20 as libc::c_int |
                            (0 as libc::c_int as u32_0 &
                                 (((0x1 as libc::c_int) << 2 as libc::c_int) -
                                      1 as libc::c_int) as libc::c_uint) <<
                                18 as libc::c_int |
                            (0 as libc::c_int as u32_0 &
                                 (((0x1 as libc::c_int) << 4 as libc::c_int) -
                                      1 as libc::c_int) as libc::c_uint) <<
                                14 as libc::c_int |
                            (0 as libc::c_int as u32_0 &
                                 (((0x1 as libc::c_int) << 4 as libc::c_int) -
                                      1 as libc::c_int) as libc::c_uint) <<
                                10 as libc::c_int |
                            (0 as libc::c_int as u32_0 &
                                 (((0x1 as libc::c_int) << 2 as libc::c_int) -
                                      1 as libc::c_int) as libc::c_uint) <<
                                8 as libc::c_int |
                            (0 as libc::c_int as u32_0 &
                                 (((0x1 as libc::c_int) << 4 as libc::c_int) -
                                      1 as libc::c_int) as libc::c_uint) <<
                                4 as libc::c_int |
                            (0 as libc::c_int as u32_0 &
                                 (((0x1 as libc::c_int) << 4 as libc::c_int) -
                                      1 as libc::c_int) as libc::c_uint) <<
                                0 as libc::c_int;
                    let fresh40 = (*__gfxCtx).polyOpa.p;
                    (*__gfxCtx).polyOpa.p = (*__gfxCtx).polyOpa.p.offset(1);
                    let mut _g_39: *mut Gfx = fresh40;
                    (*_g_39).words.w0 =
                        (0xe6 as libc::c_int as u32_0 &
                             (((0x1 as libc::c_int) << 8 as libc::c_int) -
                                  1 as libc::c_int) as libc::c_uint) <<
                            24 as libc::c_int;
                    (*_g_39).words.w1 = 0 as libc::c_int as libc::c_uint;
                    let fresh41 = (*__gfxCtx).polyOpa.p;
                    (*__gfxCtx).polyOpa.p = (*__gfxCtx).polyOpa.p.offset(1);
                    let mut _g_40: *mut Gfx = fresh41;
                    (*_g_40).words.w0 =
                        (0xf0 as libc::c_int as u32_0 &
                             (((0x1 as libc::c_int) << 8 as libc::c_int) -
                                  1 as libc::c_int) as libc::c_uint) <<
                            24 as libc::c_int;
                    (*_g_40).words.w1 =
                        (7 as libc::c_int as u32_0 &
                             (((0x1 as libc::c_int) << 3 as libc::c_int) -
                                  1 as libc::c_int) as libc::c_uint) <<
                            24 as libc::c_int |
                            (255 as libc::c_int as u32_0 &
                                 (((0x1 as libc::c_int) << 10 as libc::c_int)
                                      - 1 as libc::c_int) as libc::c_uint) <<
                                14 as libc::c_int;
                    let fresh42 = (*__gfxCtx).polyOpa.p;
                    (*__gfxCtx).polyOpa.p = (*__gfxCtx).polyOpa.p.offset(1);
                    let mut _g_41: *mut Gfx = fresh42;
                    (*_g_41).words.w0 =
                        (0xe7 as libc::c_int as u32_0 &
                             (((0x1 as libc::c_int) << 8 as libc::c_int) -
                                  1 as libc::c_int) as libc::c_uint) <<
                            24 as libc::c_int;
                    (*_g_41).words.w1 = 0 as libc::c_int as libc::c_uint;
                    let fresh43 = (*__gfxCtx).polyOpa.p;
                    (*__gfxCtx).polyOpa.p = (*__gfxCtx).polyOpa.p.offset(1);
                    let mut _g_42: *mut Gfx = fresh43;
                    (*_g_42).words.w0 =
                        (0xde as libc::c_int as u32_0 &
                             (((0x1 as libc::c_int) << 8 as libc::c_int) -
                                  1 as libc::c_int) as libc::c_uint) <<
                            24 as libc::c_int |
                            (0 as libc::c_int as u32_0 &
                                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                                      1 as libc::c_int) as libc::c_uint) <<
                                16 as libc::c_int |
                            (0 as libc::c_int as u32_0 &
                                 (((0x1 as libc::c_int) << 16 as libc::c_int)
                                      - 1 as libc::c_int) as libc::c_uint) <<
                                0 as libc::c_int;
                    (*_g_42).words.w1 =
                        (*(*skyboxCtx).dListBuf.offset(6 as libc::c_int as
                                                           isize)).as_mut_ptr()
                            as libc::c_uint;
                    let fresh44 = (*__gfxCtx).polyOpa.p;
                    (*__gfxCtx).polyOpa.p = (*__gfxCtx).polyOpa.p.offset(1);
                    let mut _g_43: *mut Gfx = fresh44;
                    (*_g_43).words.w0 =
                        (0xde as libc::c_int as u32_0 &
                             (((0x1 as libc::c_int) << 8 as libc::c_int) -
                                  1 as libc::c_int) as libc::c_uint) <<
                            24 as libc::c_int |
                            (0 as libc::c_int as u32_0 &
                                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                                      1 as libc::c_int) as libc::c_uint) <<
                                16 as libc::c_int |
                            (0 as libc::c_int as u32_0 &
                                 (((0x1 as libc::c_int) << 16 as libc::c_int)
                                      - 1 as libc::c_int) as libc::c_uint) <<
                                0 as libc::c_int;
                    (*_g_43).words.w1 =
                        (*(*skyboxCtx).dListBuf.offset(7 as libc::c_int as
                                                           isize)).as_mut_ptr()
                            as libc::c_uint
                }
            }
        }
    } else {
        let fresh45 = (*__gfxCtx).polyOpa.p;
        (*__gfxCtx).polyOpa.p = (*__gfxCtx).polyOpa.p.offset(1);
        let mut _g_44: *mut Gfx = fresh45;
        (*_g_44).words.w0 =
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
        (*_g_44).words.w1 =
            (*(*skyboxCtx).dListBuf.offset(0 as libc::c_int as
                                               isize)).as_mut_ptr() as
                libc::c_uint;
        let fresh46 = (*__gfxCtx).polyOpa.p;
        (*__gfxCtx).polyOpa.p = (*__gfxCtx).polyOpa.p.offset(1);
        let mut _g_45: *mut Gfx = fresh46;
        (*_g_45).words.w0 =
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
        (*_g_45).words.w1 =
            (*(*skyboxCtx).dListBuf.offset(2 as libc::c_int as
                                               isize)).as_mut_ptr() as
                libc::c_uint;
        let fresh47 = (*__gfxCtx).polyOpa.p;
        (*__gfxCtx).polyOpa.p = (*__gfxCtx).polyOpa.p.offset(1);
        let mut _g_46: *mut Gfx = fresh47;
        (*_g_46).words.w0 =
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
        (*_g_46).words.w1 =
            (*(*skyboxCtx).dListBuf.offset(4 as libc::c_int as
                                               isize)).as_mut_ptr() as
                libc::c_uint;
        let fresh48 = (*__gfxCtx).polyOpa.p;
        (*__gfxCtx).polyOpa.p = (*__gfxCtx).polyOpa.p.offset(1);
        let mut _g_47: *mut Gfx = fresh48;
        (*_g_47).words.w0 =
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
        (*_g_47).words.w1 =
            (*(*skyboxCtx).dListBuf.offset(6 as libc::c_int as
                                               isize)).as_mut_ptr() as
                libc::c_uint;
        let fresh49 = (*__gfxCtx).polyOpa.p;
        (*__gfxCtx).polyOpa.p = (*__gfxCtx).polyOpa.p.offset(1);
        let mut _g_48: *mut Gfx = fresh49;
        (*_g_48).words.w0 =
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
        (*_g_48).words.w1 =
            (*(*skyboxCtx).dListBuf.offset(8 as libc::c_int as
                                               isize)).as_mut_ptr() as
                libc::c_uint;
        if skyboxId as libc::c_int == SKYBOX_CUTSCENE_MAP as libc::c_int {
            let fresh50 = (*__gfxCtx).polyOpa.p;
            (*__gfxCtx).polyOpa.p = (*__gfxCtx).polyOpa.p.offset(1);
            let mut _g_49: *mut Gfx = fresh50;
            (*_g_49).words.w0 =
                (0xde as libc::c_int as u32_0 &
                     (((0x1 as libc::c_int) << 8 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    24 as libc::c_int |
                    (0 as libc::c_int as u32_0 &
                         (((0x1 as libc::c_int) << 8 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        16 as libc::c_int |
                    (0 as libc::c_int as u32_0 &
                         (((0x1 as libc::c_int) << 16 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        0 as libc::c_int;
            (*_g_49).words.w1 =
                (*(*skyboxCtx).dListBuf.offset(10 as libc::c_int as
                                                   isize)).as_mut_ptr() as
                    libc::c_uint
        }
    }
    let fresh51 = (*__gfxCtx).polyOpa.p;
    (*__gfxCtx).polyOpa.p = (*__gfxCtx).polyOpa.p.offset(1);
    let mut _g_50: *mut Gfx = fresh51;
    (*_g_50).words.w0 =
        (0xe7 as libc::c_int as u32_0 &
             (((0x1 as libc::c_int) << 8 as libc::c_int) - 1 as libc::c_int)
                 as libc::c_uint) << 24 as libc::c_int;
    (*_g_50).words.w1 = 0 as libc::c_int as libc::c_uint;
    Graph_CloseDisps(dispRefs.as_mut_ptr(), gfxCtx,
                     b"../z_vr_box_draw.c\x00" as *const u8 as
                         *const libc::c_char, 125 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn SkyboxDraw_Update(mut skyboxCtx:
                                               *mut SkyboxContext) {
}
