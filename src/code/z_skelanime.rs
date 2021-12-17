#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case,
         non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(const_raw_ptr_to_usize_cast, register_tool)]
extern "C" {
    #[no_mangle]
    fn DmaMgr_SendRequest2(req: *mut DmaRequest, ram: u32_0, vrom: u32_0,
                           size: u32_0, unk5: u32_0, queue: *mut OSMesgQueue,
                           msg: OSMesg, file: *const libc::c_char, line: s32)
     -> s32;
    #[no_mangle]
    fn __assert(exp: *const libc::c_char, file: *const libc::c_char,
                line: s32);
    #[no_mangle]
    fn osSyncPrintf(fmt: *const libc::c_char, _: ...);
    #[no_mangle]
    fn LogUtils_LogThreadId(name: *const libc::c_char, line: s32);
    #[no_mangle]
    fn osRecvMesg(mq: *mut OSMesgQueue, msg: *mut OSMesg, flag: s32) -> s32;
    #[no_mangle]
    fn osCreateMesgQueue(mq: *mut OSMesgQueue, msg: *mut OSMesg, count: s32);
    #[no_mangle]
    fn Math_CosS(angle: s16) -> f32_0;
    #[no_mangle]
    fn Math_SinS(angle: s16) -> f32_0;
    #[no_mangle]
    fn ZeldaArena_MallocDebug(size: u32_0, file: *const libc::c_char,
                              line: s32) -> *mut libc::c_void;
    #[no_mangle]
    fn ZeldaArena_FreeDebug(ptr: *mut libc::c_void, file: *const libc::c_char,
                            line: s32);
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
    fn Matrix_Push();
    #[no_mangle]
    fn Matrix_Pop();
    #[no_mangle]
    fn Matrix_TranslateRotateZYX(translation: *mut Vec3f,
                                 rotation: *mut Vec3s);
    #[no_mangle]
    fn Matrix_ToMtx(dest: *mut Mtx, file: *mut libc::c_char, line: s32)
     -> *mut Mtx;
    #[no_mangle]
    fn Matrix_NewMtx(gfxCtx: *mut GraphicsContext, file: *mut libc::c_char,
                     line: s32) -> *mut Mtx;
    #[no_mangle]
    static mut gSegments: [u32_0; 16];
    #[no_mangle]
    static mut _link_animetionSegmentStart: [u8_0; 0];
    #[no_mangle]
    static mut _link_animetionSegmentRomStart: [u8_0; 0];
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
pub type C2RustUnnamed_14 = libc::c_uint;
pub const ANIMMODE_LOOP_PARTIAL_INTERP: C2RustUnnamed_14 = 5;
pub const ANIMMODE_LOOP_PARTIAL: C2RustUnnamed_14 = 4;
pub const ANIMMODE_ONCE_INTERP: C2RustUnnamed_14 = 3;
pub const ANIMMODE_ONCE: C2RustUnnamed_14 = 2;
pub const ANIMMODE_LOOP_INTERP: C2RustUnnamed_14 = 1;
pub const ANIMMODE_LOOP: C2RustUnnamed_14 = 0;
pub type C2RustUnnamed_15 = libc::c_int;
pub const ANIMTAPER_ACCEL: C2RustUnnamed_15 = 1;
pub const ANIMTAPER_NONE: C2RustUnnamed_15 = 0;
pub const ANIMTAPER_DECEL: C2RustUnnamed_15 = -1;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct StandardLimb {
    pub jointPos: Vec3s,
    pub child: u8_0,
    pub sibling: u8_0,
    pub dList: *mut Gfx,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct LodLimb {
    pub jointPos: Vec3s,
    pub child: u8_0,
    pub sibling: u8_0,
    pub dLists: [*mut Gfx; 2],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct JointIndex {
    pub x: u16_0,
    pub y: u16_0,
    pub z: u16_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AnimationHeader {
    pub common: AnimationHeaderCommon,
    pub frameData: *mut s16,
    pub jointIndices: *mut JointIndex,
    pub staticIndexMax: u16_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct JointKey {
    pub xMax: s16,
    pub x: s16,
    pub yMax: s16,
    pub y: s16,
    pub zMax: s16,
    pub z: s16,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct LegacyAnimationHeader {
    pub frameCount: s16,
    pub limbCount: s16,
    pub frameData: *mut s16,
    pub jointKey: *mut JointKey,
}
pub type OverrideLimbDrawOpa
    =
    Option<unsafe extern "C" fn(_: *mut GlobalContext, _: s32,
                                _: *mut *mut Gfx, _: *mut Vec3f,
                                _: *mut Vec3s, _: *mut libc::c_void) -> s32>;
pub type PostLimbDrawOpa
    =
    Option<unsafe extern "C" fn(_: *mut GlobalContext, _: s32,
                                _: *mut *mut Gfx, _: *mut Vec3s,
                                _: *mut libc::c_void) -> ()>;
pub type OverrideLimbDraw
    =
    Option<unsafe extern "C" fn(_: *mut GlobalContext, _: s32,
                                _: *mut *mut Gfx, _: *mut Vec3f,
                                _: *mut Vec3s, _: *mut libc::c_void,
                                _: *mut *mut Gfx) -> s32>;
pub type PostLimbDraw
    =
    Option<unsafe extern "C" fn(_: *mut GlobalContext, _: s32,
                                _: *mut *mut Gfx, _: *mut Vec3s,
                                _: *mut libc::c_void, _: *mut *mut Gfx)
               -> ()>;
pub type AnimationType = libc::c_uint;
pub const ANIMENTRY_MOVEACTOR: AnimationType = 5;
pub const ANIMENTRY_COPYFALSE: AnimationType = 4;
pub const ANIMENTRY_COPYTRUE: AnimationType = 3;
pub const ANIMENTRY_INTERP: AnimationType = 2;
pub const ANIMENTRY_COPYALL: AnimationType = 1;
pub const ANIMENTRY_LOADFRAME: AnimationType = 0;
pub type AnimationEntryCallback
    =
    Option<unsafe extern "C" fn(_: *mut GlobalContext,
                                _: *mut AnimationEntryData) -> ()>;
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
static mut sDisableAnimQueueFlags: u32_0 = 0 as libc::c_int as u32_0;
static mut sAnimQueueFlags: u32_0 = 0;
/* *
 * Draw a limb of type `LodLimb`
 * Near or far display list is specified via `lod`
 */
#[no_mangle]
pub unsafe extern "C" fn SkelAnime_DrawLimbLod(mut globalCtx:
                                                   *mut GlobalContext,
                                               mut limbIndex: s32,
                                               mut skeleton:
                                                   *mut *mut libc::c_void,
                                               mut jointTable: *mut Vec3s,
                                               mut overrideLimbDraw:
                                                   OverrideLimbDrawOpa,
                                               mut postLimbDraw:
                                                   PostLimbDrawOpa,
                                               mut arg: *mut libc::c_void,
                                               mut lod: s32) {
    let mut limb: *mut LodLimb = 0 as *mut LodLimb;
    let mut dList: *mut Gfx = 0 as *mut Gfx;
    let mut pos: Vec3f = Vec3f{x: 0., y: 0., z: 0.,};
    let mut rot: Vec3s = Vec3s{x: 0, y: 0, z: 0,};
    let mut __gfxCtx: *mut GraphicsContext = 0 as *mut GraphicsContext;
    let mut dispRefs: [*mut Gfx; 4] = [0 as *mut Gfx; 4];
    __gfxCtx = (*globalCtx).state.gfxCtx;
    Graph_OpenDisps(dispRefs.as_mut_ptr(), (*globalCtx).state.gfxCtx,
                    b"../z_skelanime.c\x00" as *const u8 as
                        *const libc::c_char, 773 as libc::c_int);
    Matrix_Push();
    limb =
        gSegments[((*skeleton.offset(limbIndex as isize) as u32_0) <<
                       4 as libc::c_int >> 28 as libc::c_int) as
                      usize].wrapping_add(*skeleton.offset(limbIndex as isize)
                                              as u32_0 &
                                              0xffffff as libc::c_int as
                                                  libc::c_uint).wrapping_add(0x80000000
                                                                                 as
                                                                                 libc::c_uint)
            as *mut libc::c_void as *mut LodLimb;
    limbIndex += 1;
    rot = *jointTable.offset(limbIndex as isize);
    pos.x = (*limb).jointPos.x as f32_0;
    pos.y = (*limb).jointPos.y as f32_0;
    pos.z = (*limb).jointPos.z as f32_0;
    dList = (*limb).dLists[lod as usize];
    if overrideLimbDraw.is_none() ||
           overrideLimbDraw.expect("non-null function pointer")(globalCtx,
                                                                limbIndex,
                                                                &mut dList,
                                                                &mut pos,
                                                                &mut rot, arg)
               == 0 {
        Matrix_TranslateRotateZYX(&mut pos, &mut rot);
        if !dList.is_null() {
            let fresh0 = (*__gfxCtx).polyOpa.p;
            (*__gfxCtx).polyOpa.p = (*__gfxCtx).polyOpa.p.offset(1);
            let mut _g: *mut Gfx = fresh0;
            (*_g).words.w0 =
                (0xda as libc::c_int as u32_0 &
                     (((0x1 as libc::c_int) << 8 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    24 as libc::c_int |
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
                    ((0x2 as libc::c_int ^ 0x1 as libc::c_int) as u32_0 &
                         (((0x1 as libc::c_int) << 8 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        0 as libc::c_int;
            (*_g).words.w1 =
                Matrix_NewMtx((*globalCtx).state.gfxCtx,
                              b"../z_skelanime.c\x00" as *const u8 as
                                  *const libc::c_char as *mut libc::c_char,
                              805 as libc::c_int) as libc::c_uint;
            let fresh1 = (*__gfxCtx).polyOpa.p;
            (*__gfxCtx).polyOpa.p = (*__gfxCtx).polyOpa.p.offset(1);
            let mut _g_0: *mut Gfx = fresh1;
            (*_g_0).words.w0 =
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
            (*_g_0).words.w1 = dList as libc::c_uint
        }
    }
    if postLimbDraw.is_some() {
        postLimbDraw.expect("non-null function pointer")(globalCtx, limbIndex,
                                                         &mut dList, &mut rot,
                                                         arg);
    }
    if (*limb).child as libc::c_int != 0xff as libc::c_int {
        SkelAnime_DrawLimbLod(globalCtx, (*limb).child as s32, skeleton,
                              jointTable, overrideLimbDraw, postLimbDraw, arg,
                              lod);
    }
    Matrix_Pop();
    if (*limb).sibling as libc::c_int != 0xff as libc::c_int {
        SkelAnime_DrawLimbLod(globalCtx, (*limb).sibling as s32, skeleton,
                              jointTable, overrideLimbDraw, postLimbDraw, arg,
                              lod);
    }
    Graph_CloseDisps(dispRefs.as_mut_ptr(), (*globalCtx).state.gfxCtx,
                     b"../z_skelanime.c\x00" as *const u8 as
                         *const libc::c_char, 821 as libc::c_int);
}
/* *
 * Draw all limbs of type `LodLimb` in a given skeleton
 * Near or far display list is specified via `lod`
 */
#[no_mangle]
pub unsafe extern "C" fn SkelAnime_DrawLod(mut globalCtx: *mut GlobalContext,
                                           mut skeleton:
                                               *mut *mut libc::c_void,
                                           mut jointTable: *mut Vec3s,
                                           mut overrideLimbDraw:
                                               OverrideLimbDrawOpa,
                                           mut postLimbDraw: PostLimbDrawOpa,
                                           mut arg: *mut libc::c_void,
                                           mut lod: s32) {
    let mut rootLimb: *mut LodLimb = 0 as *mut LodLimb; // "skel is NULL."
    let mut pad: s32 = 0;
    let mut dList: *mut Gfx = 0 as *mut Gfx;
    let mut pos: Vec3f = Vec3f{x: 0., y: 0., z: 0.,};
    let mut rot: Vec3s = Vec3s{x: 0, y: 0, z: 0,};
    if skeleton.is_null() {
        osSyncPrintf(b"\x1b[31m\x00" as *const u8 as *const libc::c_char);
        osSyncPrintf(b"Si2_Lod_draw():skel\xe3\x81\x8cNULL\xe3\x81\xa7\xe3\x81\x99\xe3\x80\x82\n\x00"
                         as *const u8 as *const libc::c_char);
        osSyncPrintf(b"\x1b[m\x00" as *const u8 as *const libc::c_char);
        return
    }
    let mut __gfxCtx: *mut GraphicsContext = 0 as *mut GraphicsContext;
    let mut dispRefs: [*mut Gfx; 4] = [0 as *mut Gfx; 4];
    __gfxCtx = (*globalCtx).state.gfxCtx;
    Graph_OpenDisps(dispRefs.as_mut_ptr(), (*globalCtx).state.gfxCtx,
                    b"../z_skelanime.c\x00" as *const u8 as
                        *const libc::c_char, 849 as libc::c_int);
    Matrix_Push();
    rootLimb =
        gSegments[((*skeleton.offset(0 as libc::c_int as isize) as u32_0) <<
                       4 as libc::c_int >> 28 as libc::c_int) as
                      usize].wrapping_add(*skeleton.offset(0 as libc::c_int as
                                                               isize) as u32_0
                                              &
                                              0xffffff as libc::c_int as
                                                  libc::c_uint).wrapping_add(0x80000000
                                                                                 as
                                                                                 libc::c_uint)
            as *mut libc::c_void as *mut LodLimb;
    pos.x = (*jointTable.offset(0 as libc::c_int as isize)).x as f32_0;
    pos.y = (*jointTable.offset(0 as libc::c_int as isize)).y as f32_0;
    pos.z = (*jointTable.offset(0 as libc::c_int as isize)).z as f32_0;
    rot = *jointTable.offset(1 as libc::c_int as isize);
    dList = (*rootLimb).dLists[lod as usize];
    if overrideLimbDraw.is_none() ||
           overrideLimbDraw.expect("non-null function pointer")(globalCtx,
                                                                1 as
                                                                    libc::c_int,
                                                                &mut dList,
                                                                &mut pos,
                                                                &mut rot, arg)
               == 0 {
        Matrix_TranslateRotateZYX(&mut pos, &mut rot);
        if !dList.is_null() {
            let fresh2 = (*__gfxCtx).polyOpa.p;
            (*__gfxCtx).polyOpa.p = (*__gfxCtx).polyOpa.p.offset(1);
            let mut _g: *mut Gfx = fresh2;
            (*_g).words.w0 =
                (0xda as libc::c_int as u32_0 &
                     (((0x1 as libc::c_int) << 8 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    24 as libc::c_int |
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
                    ((0x2 as libc::c_int ^ 0x1 as libc::c_int) as u32_0 &
                         (((0x1 as libc::c_int) << 8 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        0 as libc::c_int;
            (*_g).words.w1 =
                Matrix_NewMtx((*globalCtx).state.gfxCtx,
                              b"../z_skelanime.c\x00" as *const u8 as
                                  *const libc::c_char as *mut libc::c_char,
                              881 as libc::c_int) as libc::c_uint;
            let fresh3 = (*__gfxCtx).polyOpa.p;
            (*__gfxCtx).polyOpa.p = (*__gfxCtx).polyOpa.p.offset(1);
            let mut _g_0: *mut Gfx = fresh3;
            (*_g_0).words.w0 =
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
            (*_g_0).words.w1 = dList as libc::c_uint
        }
    }
    if postLimbDraw.is_some() {
        postLimbDraw.expect("non-null function pointer")(globalCtx,
                                                         1 as libc::c_int,
                                                         &mut dList, &mut rot,
                                                         arg);
    }
    if (*rootLimb).child as libc::c_int != 0xff as libc::c_int {
        SkelAnime_DrawLimbLod(globalCtx, (*rootLimb).child as s32, skeleton,
                              jointTable, overrideLimbDraw, postLimbDraw, arg,
                              lod);
    }
    Matrix_Pop();
    Graph_CloseDisps(dispRefs.as_mut_ptr(), (*globalCtx).state.gfxCtx,
                     b"../z_skelanime.c\x00" as *const u8 as
                         *const libc::c_char, 894 as libc::c_int);
}
/* *
 * Draw a limb of type `LodLimb` contained within a flexible skeleton
 * Near or far display list is specified via `lod`
 */
#[no_mangle]
pub unsafe extern "C" fn SkelAnime_DrawFlexLimbLod(mut globalCtx:
                                                       *mut GlobalContext,
                                                   mut limbIndex: s32,
                                                   mut skeleton:
                                                       *mut *mut libc::c_void,
                                                   mut jointTable: *mut Vec3s,
                                                   mut overrideLimbDraw:
                                                       OverrideLimbDrawOpa,
                                                   mut postLimbDraw:
                                                       PostLimbDrawOpa,
                                                   mut arg: *mut libc::c_void,
                                                   mut lod: s32,
                                                   mut mtx: *mut *mut Mtx) {
    let mut limb: *mut LodLimb = 0 as *mut LodLimb;
    let mut newDList: *mut Gfx = 0 as *mut Gfx;
    let mut limbDList: *mut Gfx = 0 as *mut Gfx;
    let mut pos: Vec3f = Vec3f{x: 0., y: 0., z: 0.,};
    let mut rot: Vec3s = Vec3s{x: 0, y: 0, z: 0,};
    Matrix_Push();
    limb =
        gSegments[((*skeleton.offset(limbIndex as isize) as u32_0) <<
                       4 as libc::c_int >> 28 as libc::c_int) as
                      usize].wrapping_add(*skeleton.offset(limbIndex as isize)
                                              as u32_0 &
                                              0xffffff as libc::c_int as
                                                  libc::c_uint).wrapping_add(0x80000000
                                                                                 as
                                                                                 libc::c_uint)
            as *mut libc::c_void as *mut LodLimb;
    limbIndex += 1;
    rot = *jointTable.offset(limbIndex as isize);
    pos.x = (*limb).jointPos.x as f32_0;
    pos.y = (*limb).jointPos.y as f32_0;
    pos.z = (*limb).jointPos.z as f32_0;
    limbDList = (*limb).dLists[lod as usize];
    newDList = limbDList;
    if overrideLimbDraw.is_none() ||
           overrideLimbDraw.expect("non-null function pointer")(globalCtx,
                                                                limbIndex,
                                                                &mut newDList,
                                                                &mut pos,
                                                                &mut rot, arg)
               == 0 {
        Matrix_TranslateRotateZYX(&mut pos, &mut rot);
        if !newDList.is_null() {
            Matrix_ToMtx(*mtx,
                         b"../z_skelanime.c\x00" as *const u8 as
                             *const libc::c_char as *mut libc::c_char,
                         945 as libc::c_int);
            let mut __gfxCtx: *mut GraphicsContext =
                0 as *mut GraphicsContext;
            let mut dispRefs: [*mut Gfx; 4] = [0 as *mut Gfx; 4];
            __gfxCtx = (*globalCtx).state.gfxCtx;
            Graph_OpenDisps(dispRefs.as_mut_ptr(), (*globalCtx).state.gfxCtx,
                            b"../z_skelanime.c\x00" as *const u8 as
                                *const libc::c_char, 946 as libc::c_int);
            let fresh4 = (*__gfxCtx).polyOpa.p;
            (*__gfxCtx).polyOpa.p = (*__gfxCtx).polyOpa.p.offset(1);
            let mut _g: *mut Gfx = fresh4;
            (*_g).words.w0 =
                (0xda as libc::c_int as u32_0 &
                     (((0x1 as libc::c_int) << 8 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    24 as libc::c_int |
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
                    ((0x2 as libc::c_int ^ 0x1 as libc::c_int) as u32_0 &
                         (((0x1 as libc::c_int) << 8 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        0 as libc::c_int;
            (*_g).words.w1 = *mtx as libc::c_uint;
            let fresh5 = (*__gfxCtx).polyOpa.p;
            (*__gfxCtx).polyOpa.p = (*__gfxCtx).polyOpa.p.offset(1);
            let mut _g_0: *mut Gfx = fresh5;
            (*_g_0).words.w0 =
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
            (*_g_0).words.w1 = newDList as libc::c_uint;
            Graph_CloseDisps(dispRefs.as_mut_ptr(), (*globalCtx).state.gfxCtx,
                             b"../z_skelanime.c\x00" as *const u8 as
                                 *const libc::c_char, 949 as libc::c_int);
            *mtx = (*mtx).offset(1)
        } else if !limbDList.is_null() {
            Matrix_ToMtx(*mtx,
                         b"../z_skelanime.c\x00" as *const u8 as
                             *const libc::c_char as *mut libc::c_char,
                         954 as libc::c_int);
            *mtx = (*mtx).offset(1)
        }
    }
    if postLimbDraw.is_some() {
        postLimbDraw.expect("non-null function pointer")(globalCtx, limbIndex,
                                                         &mut limbDList,
                                                         &mut rot, arg);
    }
    if (*limb).child as libc::c_int != 0xff as libc::c_int {
        SkelAnime_DrawFlexLimbLod(globalCtx, (*limb).child as s32, skeleton,
                                  jointTable, overrideLimbDraw, postLimbDraw,
                                  arg, lod, mtx);
    }
    Matrix_Pop();
    if (*limb).sibling as libc::c_int != 0xff as libc::c_int {
        SkelAnime_DrawFlexLimbLod(globalCtx, (*limb).sibling as s32, skeleton,
                                  jointTable, overrideLimbDraw, postLimbDraw,
                                  arg, lod, mtx);
    };
}
/* *
 * Draws all limbs of type `LodLimb` in a given flexible skeleton
 * Limbs in a flexible skeleton have meshes that can stretch to line up with other limbs.
 * An array of matrices is dynamically allocated so each limb can access any transform to ensure its meshes line up.
 */
#[no_mangle]
pub unsafe extern "C" fn SkelAnime_DrawFlexLod(mut globalCtx:
                                                   *mut GlobalContext,
                                               mut skeleton:
                                                   *mut *mut libc::c_void,
                                               mut jointTable: *mut Vec3s,
                                               mut dListCount: s32,
                                               mut overrideLimbDraw:
                                                   OverrideLimbDrawOpa,
                                               mut postLimbDraw:
                                                   PostLimbDrawOpa,
                                               mut arg: *mut libc::c_void,
                                               mut lod: s32) {
    let mut rootLimb: *mut LodLimb = 0 as *mut LodLimb; // "skel is NULL."
    let mut pad: s32 = 0;
    let mut newDList: *mut Gfx = 0 as *mut Gfx;
    let mut limbDList: *mut Gfx = 0 as *mut Gfx;
    let mut pos: Vec3f = Vec3f{x: 0., y: 0., z: 0.,};
    let mut rot: Vec3s = Vec3s{x: 0, y: 0, z: 0,};
    let mut mtx: *mut Mtx =
        Graph_Alloc((*globalCtx).state.gfxCtx,
                    (dListCount as
                         libc::c_uint).wrapping_mul(::std::mem::size_of::<Mtx>()
                                                        as libc::c_ulong) as
                        size_t) as *mut Mtx;
    if skeleton.is_null() {
        osSyncPrintf(b"\x1b[31m\x00" as *const u8 as *const libc::c_char);
        osSyncPrintf(b"Si2_Lod_draw_SV():skel\xe3\x81\x8cNULL\xe3\x81\xa7\xe3\x81\x99\xe3\x80\x82\n\x00"
                         as *const u8 as *const libc::c_char);
        osSyncPrintf(b"\x1b[m\x00" as *const u8 as *const libc::c_char);
        return
    }
    let mut __gfxCtx: *mut GraphicsContext = 0 as *mut GraphicsContext;
    let mut dispRefs: [*mut Gfx; 4] = [0 as *mut Gfx; 4];
    __gfxCtx = (*globalCtx).state.gfxCtx;
    Graph_OpenDisps(dispRefs.as_mut_ptr(), (*globalCtx).state.gfxCtx,
                    b"../z_skelanime.c\x00" as *const u8 as
                        *const libc::c_char, 1000 as libc::c_int);
    let fresh6 = (*__gfxCtx).polyOpa.p;
    (*__gfxCtx).polyOpa.p = (*__gfxCtx).polyOpa.p.offset(1);
    let mut _g: *mut Gfx = fresh6;
    (*_g).words.w0 =
        (0xdb as libc::c_int as u32_0 &
             (((0x1 as libc::c_int) << 8 as libc::c_int) - 1 as libc::c_int)
                 as libc::c_uint) << 24 as libc::c_int |
            (0x6 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 16 as libc::c_int
            |
            ((0xd as libc::c_int * 4 as libc::c_int) as u32_0 &
                 (((0x1 as libc::c_int) << 16 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 0 as libc::c_int;
    (*_g).words.w1 = mtx as libc::c_uint;
    Matrix_Push();
    rootLimb =
        gSegments[((*skeleton.offset(0 as libc::c_int as isize) as u32_0) <<
                       4 as libc::c_int >> 28 as libc::c_int) as
                      usize].wrapping_add(*skeleton.offset(0 as libc::c_int as
                                                               isize) as u32_0
                                              &
                                              0xffffff as libc::c_int as
                                                  libc::c_uint).wrapping_add(0x80000000
                                                                                 as
                                                                                 libc::c_uint)
            as *mut libc::c_void as *mut LodLimb;
    pos.x = (*jointTable.offset(0 as libc::c_int as isize)).x as f32_0;
    pos.y = (*jointTable.offset(0 as libc::c_int as isize)).y as f32_0;
    pos.z = (*jointTable.offset(0 as libc::c_int as isize)).z as f32_0;
    rot = *jointTable.offset(1 as libc::c_int as isize);
    limbDList = (*rootLimb).dLists[lod as usize];
    newDList = limbDList;
    if overrideLimbDraw.is_none() ||
           overrideLimbDraw.expect("non-null function pointer")(globalCtx,
                                                                1 as
                                                                    libc::c_int,
                                                                &mut newDList,
                                                                &mut pos,
                                                                &mut rot, arg)
               == 0 {
        Matrix_TranslateRotateZYX(&mut pos, &mut rot);
        if !newDList.is_null() {
            Matrix_ToMtx(mtx,
                         b"../z_skelanime.c\x00" as *const u8 as
                             *const libc::c_char as *mut libc::c_char,
                         1033 as libc::c_int);
            let fresh7 = (*__gfxCtx).polyOpa.p;
            (*__gfxCtx).polyOpa.p = (*__gfxCtx).polyOpa.p.offset(1);
            let mut _g_0: *mut Gfx = fresh7;
            (*_g_0).words.w0 =
                (0xda as libc::c_int as u32_0 &
                     (((0x1 as libc::c_int) << 8 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    24 as libc::c_int |
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
                    ((0x2 as libc::c_int ^ 0x1 as libc::c_int) as u32_0 &
                         (((0x1 as libc::c_int) << 8 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        0 as libc::c_int;
            (*_g_0).words.w1 = mtx as libc::c_uint;
            let fresh8 = (*__gfxCtx).polyOpa.p;
            (*__gfxCtx).polyOpa.p = (*__gfxCtx).polyOpa.p.offset(1);
            let mut _g_1: *mut Gfx = fresh8;
            (*_g_1).words.w0 =
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
            (*_g_1).words.w1 = newDList as libc::c_uint;
            mtx = mtx.offset(1)
        } else if !limbDList.is_null() {
            Matrix_ToMtx(mtx,
                         b"../z_skelanime.c\x00" as *const u8 as
                             *const libc::c_char as *mut libc::c_char,
                         1040 as libc::c_int);
            mtx = mtx.offset(1)
        }
    }
    if postLimbDraw.is_some() {
        postLimbDraw.expect("non-null function pointer")(globalCtx,
                                                         1 as libc::c_int,
                                                         &mut limbDList,
                                                         &mut rot, arg);
    }
    if (*rootLimb).child as libc::c_int != 0xff as libc::c_int {
        SkelAnime_DrawFlexLimbLod(globalCtx, (*rootLimb).child as s32,
                                  skeleton, jointTable, overrideLimbDraw,
                                  postLimbDraw, arg, lod, &mut mtx);
    }
    Matrix_Pop();
    Graph_CloseDisps(dispRefs.as_mut_ptr(), (*globalCtx).state.gfxCtx,
                     b"../z_skelanime.c\x00" as *const u8 as
                         *const libc::c_char, 1053 as libc::c_int);
}
/* *
 * Draw a limb of type `StandardLimb` to the polyOpa buffer
 */
#[no_mangle]
pub unsafe extern "C" fn SkelAnime_DrawLimbOpa(mut globalCtx:
                                                   *mut GlobalContext,
                                               mut limbIndex: s32,
                                               mut skeleton:
                                                   *mut *mut libc::c_void,
                                               mut jointTable: *mut Vec3s,
                                               mut overrideLimbDraw:
                                                   OverrideLimbDrawOpa,
                                               mut postLimbDraw:
                                                   PostLimbDrawOpa,
                                               mut arg: *mut libc::c_void) {
    let mut limb: *mut StandardLimb = 0 as *mut StandardLimb;
    let mut dList: *mut Gfx = 0 as *mut Gfx;
    let mut pos: Vec3f = Vec3f{x: 0., y: 0., z: 0.,};
    let mut rot: Vec3s = Vec3s{x: 0, y: 0, z: 0,};
    let mut __gfxCtx: *mut GraphicsContext = 0 as *mut GraphicsContext;
    let mut dispRefs: [*mut Gfx; 4] = [0 as *mut Gfx; 4];
    __gfxCtx = (*globalCtx).state.gfxCtx;
    Graph_OpenDisps(dispRefs.as_mut_ptr(), (*globalCtx).state.gfxCtx,
                    b"../z_skelanime.c\x00" as *const u8 as
                        *const libc::c_char, 1076 as libc::c_int);
    Matrix_Push();
    limb =
        gSegments[((*skeleton.offset(limbIndex as isize) as u32_0) <<
                       4 as libc::c_int >> 28 as libc::c_int) as
                      usize].wrapping_add(*skeleton.offset(limbIndex as isize)
                                              as u32_0 &
                                              0xffffff as libc::c_int as
                                                  libc::c_uint).wrapping_add(0x80000000
                                                                                 as
                                                                                 libc::c_uint)
            as *mut libc::c_void as *mut StandardLimb;
    limbIndex += 1;
    rot = *jointTable.offset(limbIndex as isize);
    pos.x = (*limb).jointPos.x as f32_0;
    pos.y = (*limb).jointPos.y as f32_0;
    pos.z = (*limb).jointPos.z as f32_0;
    dList = (*limb).dList;
    if overrideLimbDraw.is_none() ||
           overrideLimbDraw.expect("non-null function pointer")(globalCtx,
                                                                limbIndex,
                                                                &mut dList,
                                                                &mut pos,
                                                                &mut rot, arg)
               == 0 {
        Matrix_TranslateRotateZYX(&mut pos, &mut rot);
        if !dList.is_null() {
            let fresh9 = (*__gfxCtx).polyOpa.p;
            (*__gfxCtx).polyOpa.p = (*__gfxCtx).polyOpa.p.offset(1);
            let mut _g: *mut Gfx = fresh9;
            (*_g).words.w0 =
                (0xda as libc::c_int as u32_0 &
                     (((0x1 as libc::c_int) << 8 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    24 as libc::c_int |
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
                    ((0x2 as libc::c_int ^ 0x1 as libc::c_int) as u32_0 &
                         (((0x1 as libc::c_int) << 8 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        0 as libc::c_int;
            (*_g).words.w1 =
                Matrix_NewMtx((*globalCtx).state.gfxCtx,
                              b"../z_skelanime.c\x00" as *const u8 as
                                  *const libc::c_char as *mut libc::c_char,
                              1103 as libc::c_int) as libc::c_uint;
            let fresh10 = (*__gfxCtx).polyOpa.p;
            (*__gfxCtx).polyOpa.p = (*__gfxCtx).polyOpa.p.offset(1);
            let mut _g_0: *mut Gfx = fresh10;
            (*_g_0).words.w0 =
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
            (*_g_0).words.w1 = dList as libc::c_uint
        }
    }
    if postLimbDraw.is_some() {
        postLimbDraw.expect("non-null function pointer")(globalCtx, limbIndex,
                                                         &mut dList, &mut rot,
                                                         arg);
    }
    if (*limb).child as libc::c_int != 0xff as libc::c_int {
        SkelAnime_DrawLimbOpa(globalCtx, (*limb).child as s32, skeleton,
                              jointTable, overrideLimbDraw, postLimbDraw,
                              arg);
    }
    Matrix_Pop();
    if (*limb).sibling as libc::c_int != 0xff as libc::c_int {
        SkelAnime_DrawLimbOpa(globalCtx, (*limb).sibling as s32, skeleton,
                              jointTable, overrideLimbDraw, postLimbDraw,
                              arg);
    }
    Graph_CloseDisps(dispRefs.as_mut_ptr(), (*globalCtx).state.gfxCtx,
                     b"../z_skelanime.c\x00" as *const u8 as
                         *const libc::c_char, 1121 as libc::c_int);
}
/* *
 * Draw all limbs of type `StandardLimb` in a given skeleton to the polyOpa buffer
 */
#[no_mangle]
pub unsafe extern "C" fn SkelAnime_DrawOpa(mut globalCtx: *mut GlobalContext,
                                           mut skeleton:
                                               *mut *mut libc::c_void,
                                           mut jointTable: *mut Vec3s,
                                           mut overrideLimbDraw:
                                               OverrideLimbDrawOpa,
                                           mut postLimbDraw: PostLimbDrawOpa,
                                           mut arg: *mut libc::c_void) {
    let mut rootLimb: *mut StandardLimb =
        0 as *mut StandardLimb; // "skel is NULL."
    let mut pad: s32 = 0;
    let mut dList: *mut Gfx = 0 as *mut Gfx;
    let mut pos: Vec3f = Vec3f{x: 0., y: 0., z: 0.,};
    let mut rot: Vec3s = Vec3s{x: 0, y: 0, z: 0,};
    if skeleton.is_null() {
        osSyncPrintf(b"\x1b[31m\x00" as *const u8 as *const libc::c_char);
        osSyncPrintf(b"Si2_draw():skel\xe3\x81\x8cNULL\xe3\x81\xa7\xe3\x81\x99\xe3\x80\x82\n\x00"
                         as *const u8 as *const libc::c_char);
        osSyncPrintf(b"\x1b[m\x00" as *const u8 as *const libc::c_char);
        return
    }
    let mut __gfxCtx: *mut GraphicsContext = 0 as *mut GraphicsContext;
    let mut dispRefs: [*mut Gfx; 4] = [0 as *mut Gfx; 4];
    __gfxCtx = (*globalCtx).state.gfxCtx;
    Graph_OpenDisps(dispRefs.as_mut_ptr(), (*globalCtx).state.gfxCtx,
                    b"../z_skelanime.c\x00" as *const u8 as
                        *const libc::c_char, 1148 as libc::c_int);
    Matrix_Push();
    rootLimb =
        gSegments[((*skeleton.offset(0 as libc::c_int as isize) as u32_0) <<
                       4 as libc::c_int >> 28 as libc::c_int) as
                      usize].wrapping_add(*skeleton.offset(0 as libc::c_int as
                                                               isize) as u32_0
                                              &
                                              0xffffff as libc::c_int as
                                                  libc::c_uint).wrapping_add(0x80000000
                                                                                 as
                                                                                 libc::c_uint)
            as *mut libc::c_void as *mut StandardLimb;
    pos.x = (*jointTable.offset(0 as libc::c_int as isize)).x as f32_0;
    pos.y = (*jointTable.offset(0 as libc::c_int as isize)).y as f32_0;
    pos.z = (*jointTable.offset(0 as libc::c_int as isize)).z as f32_0;
    rot = *jointTable.offset(1 as libc::c_int as isize);
    dList = (*rootLimb).dList;
    if overrideLimbDraw.is_none() ||
           overrideLimbDraw.expect("non-null function pointer")(globalCtx,
                                                                1 as
                                                                    libc::c_int,
                                                                &mut dList,
                                                                &mut pos,
                                                                &mut rot, arg)
               == 0 {
        Matrix_TranslateRotateZYX(&mut pos, &mut rot);
        if !dList.is_null() {
            let fresh11 = (*__gfxCtx).polyOpa.p;
            (*__gfxCtx).polyOpa.p = (*__gfxCtx).polyOpa.p.offset(1);
            let mut _g: *mut Gfx = fresh11;
            (*_g).words.w0 =
                (0xda as libc::c_int as u32_0 &
                     (((0x1 as libc::c_int) << 8 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    24 as libc::c_int |
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
                    ((0x2 as libc::c_int ^ 0x1 as libc::c_int) as u32_0 &
                         (((0x1 as libc::c_int) << 8 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        0 as libc::c_int;
            (*_g).words.w1 =
                Matrix_NewMtx((*globalCtx).state.gfxCtx,
                              b"../z_skelanime.c\x00" as *const u8 as
                                  *const libc::c_char as *mut libc::c_char,
                              1176 as libc::c_int) as libc::c_uint;
            let fresh12 = (*__gfxCtx).polyOpa.p;
            (*__gfxCtx).polyOpa.p = (*__gfxCtx).polyOpa.p.offset(1);
            let mut _g_0: *mut Gfx = fresh12;
            (*_g_0).words.w0 =
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
            (*_g_0).words.w1 = dList as libc::c_uint
        }
    }
    if postLimbDraw.is_some() {
        postLimbDraw.expect("non-null function pointer")(globalCtx,
                                                         1 as libc::c_int,
                                                         &mut dList, &mut rot,
                                                         arg);
    }
    if (*rootLimb).child as libc::c_int != 0xff as libc::c_int {
        SkelAnime_DrawLimbOpa(globalCtx, (*rootLimb).child as s32, skeleton,
                              jointTable, overrideLimbDraw, postLimbDraw,
                              arg);
    }
    Matrix_Pop();
    Graph_CloseDisps(dispRefs.as_mut_ptr(), (*globalCtx).state.gfxCtx,
                     b"../z_skelanime.c\x00" as *const u8 as
                         *const libc::c_char, 1190 as libc::c_int);
}
/* *
 * Draw a limb of type `StandardLimb` contained within a flexible skeleton to the polyOpa buffer
 */
#[no_mangle]
pub unsafe extern "C" fn SkelAnime_DrawFlexLimbOpa(mut globalCtx:
                                                       *mut GlobalContext,
                                                   mut limbIndex: s32,
                                                   mut skeleton:
                                                       *mut *mut libc::c_void,
                                                   mut jointTable: *mut Vec3s,
                                                   mut overrideLimbDraw:
                                                       OverrideLimbDrawOpa,
                                                   mut postLimbDraw:
                                                       PostLimbDrawOpa,
                                                   mut arg: *mut libc::c_void,
                                                   mut limbMatricies:
                                                       *mut *mut Mtx) {
    let mut limb: *mut StandardLimb = 0 as *mut StandardLimb;
    let mut newDList: *mut Gfx = 0 as *mut Gfx;
    let mut limbDList: *mut Gfx = 0 as *mut Gfx;
    let mut pos: Vec3f = Vec3f{x: 0., y: 0., z: 0.,};
    let mut rot: Vec3s = Vec3s{x: 0, y: 0, z: 0,};
    let mut __gfxCtx: *mut GraphicsContext = 0 as *mut GraphicsContext;
    let mut dispRefs: [*mut Gfx; 4] = [0 as *mut Gfx; 4];
    __gfxCtx = (*globalCtx).state.gfxCtx;
    Graph_OpenDisps(dispRefs.as_mut_ptr(), (*globalCtx).state.gfxCtx,
                    b"../z_skelanime.c\x00" as *const u8 as
                        *const libc::c_char, 1214 as libc::c_int);
    Matrix_Push();
    limb =
        gSegments[((*skeleton.offset(limbIndex as isize) as u32_0) <<
                       4 as libc::c_int >> 28 as libc::c_int) as
                      usize].wrapping_add(*skeleton.offset(limbIndex as isize)
                                              as u32_0 &
                                              0xffffff as libc::c_int as
                                                  libc::c_uint).wrapping_add(0x80000000
                                                                                 as
                                                                                 libc::c_uint)
            as *mut libc::c_void as *mut StandardLimb;
    limbIndex += 1;
    rot = *jointTable.offset(limbIndex as isize);
    pos.x = (*limb).jointPos.x as f32_0;
    pos.y = (*limb).jointPos.y as f32_0;
    pos.z = (*limb).jointPos.z as f32_0;
    limbDList = (*limb).dList;
    newDList = limbDList;
    if overrideLimbDraw.is_none() ||
           overrideLimbDraw.expect("non-null function pointer")(globalCtx,
                                                                limbIndex,
                                                                &mut newDList,
                                                                &mut pos,
                                                                &mut rot, arg)
               == 0 {
        Matrix_TranslateRotateZYX(&mut pos, &mut rot);
        if !newDList.is_null() {
            Matrix_ToMtx(*limbMatricies,
                         b"../z_skelanime.c\x00" as *const u8 as
                             *const libc::c_char as *mut libc::c_char,
                         1242 as libc::c_int);
            let fresh13 = (*__gfxCtx).polyOpa.p;
            (*__gfxCtx).polyOpa.p = (*__gfxCtx).polyOpa.p.offset(1);
            let mut _g: *mut Gfx = fresh13;
            (*_g).words.w0 =
                (0xda as libc::c_int as u32_0 &
                     (((0x1 as libc::c_int) << 8 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    24 as libc::c_int |
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
                    ((0x2 as libc::c_int ^ 0x1 as libc::c_int) as u32_0 &
                         (((0x1 as libc::c_int) << 8 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        0 as libc::c_int;
            (*_g).words.w1 = *limbMatricies as libc::c_uint;
            let fresh14 = (*__gfxCtx).polyOpa.p;
            (*__gfxCtx).polyOpa.p = (*__gfxCtx).polyOpa.p.offset(1);
            let mut _g_0: *mut Gfx = fresh14;
            (*_g_0).words.w0 =
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
            (*_g_0).words.w1 = newDList as libc::c_uint;
            *limbMatricies = (*limbMatricies).offset(1)
        } else if !limbDList.is_null() {
            Matrix_ToMtx(*limbMatricies,
                         b"../z_skelanime.c\x00" as *const u8 as
                             *const libc::c_char as *mut libc::c_char,
                         1249 as libc::c_int);
            *limbMatricies = (*limbMatricies).offset(1)
        }
    }
    if postLimbDraw.is_some() {
        postLimbDraw.expect("non-null function pointer")(globalCtx, limbIndex,
                                                         &mut limbDList,
                                                         &mut rot, arg);
    }
    if (*limb).child as libc::c_int != 0xff as libc::c_int {
        SkelAnime_DrawFlexLimbOpa(globalCtx, (*limb).child as s32, skeleton,
                                  jointTable, overrideLimbDraw, postLimbDraw,
                                  arg, limbMatricies);
    }
    Matrix_Pop();
    if (*limb).sibling as libc::c_int != 0xff as libc::c_int {
        SkelAnime_DrawFlexLimbOpa(globalCtx, (*limb).sibling as s32, skeleton,
                                  jointTable, overrideLimbDraw, postLimbDraw,
                                  arg, limbMatricies);
    }
    Graph_CloseDisps(dispRefs.as_mut_ptr(), (*globalCtx).state.gfxCtx,
                     b"../z_skelanime.c\x00" as *const u8 as
                         *const libc::c_char, 1265 as libc::c_int);
}
/* *
 * Draw all limbs of type `StandardLimb` in a given flexible skeleton to the polyOpa buffer
 * Limbs in a flexible skeleton have meshes that can stretch to line up with other limbs.
 * An array of matrices is dynamically allocated so each limb can access any transform to ensure its meshes line up.
 */
#[no_mangle]
pub unsafe extern "C" fn SkelAnime_DrawFlexOpa(mut globalCtx:
                                                   *mut GlobalContext,
                                               mut skeleton:
                                                   *mut *mut libc::c_void,
                                               mut jointTable: *mut Vec3s,
                                               mut dListCount: s32,
                                               mut overrideLimbDraw:
                                                   OverrideLimbDrawOpa,
                                               mut postLimbDraw:
                                                   PostLimbDrawOpa,
                                               mut arg: *mut libc::c_void) {
    let mut rootLimb: *mut StandardLimb =
        0 as *mut StandardLimb; // "skel is NULL."
    let mut pad: s32 = 0;
    let mut newDList: *mut Gfx = 0 as *mut Gfx;
    let mut limbDList: *mut Gfx = 0 as *mut Gfx;
    let mut pos: Vec3f = Vec3f{x: 0., y: 0., z: 0.,};
    let mut rot: Vec3s = Vec3s{x: 0, y: 0, z: 0,};
    let mut mtx: *mut Mtx =
        Graph_Alloc((*globalCtx).state.gfxCtx,
                    (dListCount as
                         libc::c_uint).wrapping_mul(::std::mem::size_of::<Mtx>()
                                                        as libc::c_ulong) as
                        size_t) as *mut Mtx;
    if skeleton.is_null() {
        osSyncPrintf(b"\x1b[31m\x00" as *const u8 as *const libc::c_char);
        osSyncPrintf(b"Si2_draw_SV():skel\xe3\x81\x8cNULL\xe3\x81\xa7\xe3\x81\x99\xe3\x80\x82\n\x00"
                         as *const u8 as *const libc::c_char);
        osSyncPrintf(b"\x1b[m\x00" as *const u8 as *const libc::c_char);
        return
    }
    let mut __gfxCtx: *mut GraphicsContext = 0 as *mut GraphicsContext;
    let mut dispRefs: [*mut Gfx; 4] = [0 as *mut Gfx; 4];
    __gfxCtx = (*globalCtx).state.gfxCtx;
    Graph_OpenDisps(dispRefs.as_mut_ptr(), (*globalCtx).state.gfxCtx,
                    b"../z_skelanime.c\x00" as *const u8 as
                        *const libc::c_char, 1294 as libc::c_int);
    let fresh15 = (*__gfxCtx).polyOpa.p;
    (*__gfxCtx).polyOpa.p = (*__gfxCtx).polyOpa.p.offset(1);
    let mut _g: *mut Gfx = fresh15;
    (*_g).words.w0 =
        (0xdb as libc::c_int as u32_0 &
             (((0x1 as libc::c_int) << 8 as libc::c_int) - 1 as libc::c_int)
                 as libc::c_uint) << 24 as libc::c_int |
            (0x6 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 16 as libc::c_int
            |
            ((0xd as libc::c_int * 4 as libc::c_int) as u32_0 &
                 (((0x1 as libc::c_int) << 16 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 0 as libc::c_int;
    (*_g).words.w1 = mtx as libc::c_uint;
    Matrix_Push();
    rootLimb =
        gSegments[((*skeleton.offset(0 as libc::c_int as isize) as u32_0) <<
                       4 as libc::c_int >> 28 as libc::c_int) as
                      usize].wrapping_add(*skeleton.offset(0 as libc::c_int as
                                                               isize) as u32_0
                                              &
                                              0xffffff as libc::c_int as
                                                  libc::c_uint).wrapping_add(0x80000000
                                                                                 as
                                                                                 libc::c_uint)
            as *mut libc::c_void as *mut StandardLimb;
    pos.x = (*jointTable.offset(0 as libc::c_int as isize)).x as f32_0;
    pos.y = (*jointTable.offset(0 as libc::c_int as isize)).y as f32_0;
    pos.z = (*jointTable.offset(0 as libc::c_int as isize)).z as f32_0;
    rot = *jointTable.offset(1 as libc::c_int as isize);
    limbDList = (*rootLimb).dList;
    newDList = limbDList;
    if overrideLimbDraw.is_none() ||
           overrideLimbDraw.expect("non-null function pointer")(globalCtx,
                                                                1 as
                                                                    libc::c_int,
                                                                &mut newDList,
                                                                &mut pos,
                                                                &mut rot, arg)
               == 0 {
        Matrix_TranslateRotateZYX(&mut pos, &mut rot);
        if !newDList.is_null() {
            Matrix_ToMtx(mtx,
                         b"../z_skelanime.c\x00" as *const u8 as
                             *const libc::c_char as *mut libc::c_char,
                         1327 as libc::c_int);
            let fresh16 = (*__gfxCtx).polyOpa.p;
            (*__gfxCtx).polyOpa.p = (*__gfxCtx).polyOpa.p.offset(1);
            let mut _g_0: *mut Gfx = fresh16;
            (*_g_0).words.w0 =
                (0xda as libc::c_int as u32_0 &
                     (((0x1 as libc::c_int) << 8 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    24 as libc::c_int |
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
                    ((0x2 as libc::c_int ^ 0x1 as libc::c_int) as u32_0 &
                         (((0x1 as libc::c_int) << 8 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        0 as libc::c_int;
            (*_g_0).words.w1 = mtx as libc::c_uint;
            let fresh17 = (*__gfxCtx).polyOpa.p;
            (*__gfxCtx).polyOpa.p = (*__gfxCtx).polyOpa.p.offset(1);
            let mut _g_1: *mut Gfx = fresh17;
            (*_g_1).words.w0 =
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
            (*_g_1).words.w1 = newDList as libc::c_uint;
            mtx = mtx.offset(1)
        } else if !limbDList.is_null() {
            Matrix_ToMtx(mtx,
                         b"../z_skelanime.c\x00" as *const u8 as
                             *const libc::c_char as *mut libc::c_char,
                         1334 as libc::c_int);
            mtx = mtx.offset(1)
        }
    }
    if postLimbDraw.is_some() {
        postLimbDraw.expect("non-null function pointer")(globalCtx,
                                                         1 as libc::c_int,
                                                         &mut limbDList,
                                                         &mut rot, arg);
    }
    if (*rootLimb).child as libc::c_int != 0xff as libc::c_int {
        SkelAnime_DrawFlexLimbOpa(globalCtx, (*rootLimb).child as s32,
                                  skeleton, jointTable, overrideLimbDraw,
                                  postLimbDraw, arg, &mut mtx);
    }
    Matrix_Pop();
    Graph_CloseDisps(dispRefs.as_mut_ptr(), (*globalCtx).state.gfxCtx,
                     b"../z_skelanime.c\x00" as *const u8 as
                         *const libc::c_char, 1347 as libc::c_int);
}
/* *
 * Copies frame data from the frame data table, indexed by the joint index table.
 * Indices below limit are copied from that entry in the static frame data table.
 * Indices above limit are offsets to a frame data array indexed by the frame.
 */
#[no_mangle]
pub unsafe extern "C" fn SkelAnime_GetFrameData(mut animation:
                                                    *mut AnimationHeader,
                                                mut frame: s32,
                                                mut limbCount: s32,
                                                mut frameTable: *mut Vec3s) {
    let mut animHeader: *mut AnimationHeader =
        gSegments[((animation as u32_0) << 4 as libc::c_int >>
                       28 as libc::c_int) as
                      usize].wrapping_add(animation as u32_0 &
                                              0xffffff as libc::c_int as
                                                  libc::c_uint).wrapping_add(0x80000000
                                                                                 as
                                                                                 libc::c_uint)
            as *mut libc::c_void as *mut AnimationHeader;
    let mut jointIndices: *mut JointIndex =
        gSegments[(((*animHeader).jointIndices as u32_0) << 4 as libc::c_int
                       >> 28 as libc::c_int) as
                      usize].wrapping_add((*animHeader).jointIndices as u32_0
                                              &
                                              0xffffff as libc::c_int as
                                                  libc::c_uint).wrapping_add(0x80000000
                                                                                 as
                                                                                 libc::c_uint)
            as *mut libc::c_void as *mut JointIndex;
    let mut frameData: *mut s16 =
        gSegments[(((*animHeader).frameData as u32_0) << 4 as libc::c_int >>
                       28 as libc::c_int) as
                      usize].wrapping_add((*animHeader).frameData as u32_0 &
                                              0xffffff as libc::c_int as
                                                  libc::c_uint).wrapping_add(0x80000000
                                                                                 as
                                                                                 libc::c_uint)
            as *mut libc::c_void as *mut s16;
    let mut staticData: *mut s16 =
        &mut *frameData.offset(0 as libc::c_int as isize) as *mut s16;
    let mut dynamicData: *mut s16 =
        &mut *frameData.offset(frame as isize) as *mut s16;
    let mut staticIndexMax: u16_0 = (*animHeader).staticIndexMax;
    let mut i: s32 = 0;
    i = 0 as libc::c_int;
    while i < limbCount {
        if frameTable.is_null() || jointIndices.is_null() ||
               dynamicData.is_null() || staticData.is_null() {
            LogUtils_LogThreadId(b"../z_skelanime.c\x00" as *const u8 as
                                     *const libc::c_char,
                                 1392 as libc::c_int);
            osSyncPrintf(b"out = %08x\n\x00" as *const u8 as
                             *const libc::c_char, frameTable);
            LogUtils_LogThreadId(b"../z_skelanime.c\x00" as *const u8 as
                                     *const libc::c_char,
                                 1393 as libc::c_int);
            osSyncPrintf(b"ref_tbl = %08x\n\x00" as *const u8 as
                             *const libc::c_char, jointIndices);
            LogUtils_LogThreadId(b"../z_skelanime.c\x00" as *const u8 as
                                     *const libc::c_char,
                                 1394 as libc::c_int);
            osSyncPrintf(b"frame_tbl = %08x\n\x00" as *const u8 as
                             *const libc::c_char, dynamicData);
            LogUtils_LogThreadId(b"../z_skelanime.c\x00" as *const u8 as
                                     *const libc::c_char,
                                 1395 as libc::c_int);
            osSyncPrintf(b"tbl = %08x\n\x00" as *const u8 as
                             *const libc::c_char, staticData);
        }
        (*frameTable).x =
            if (*jointIndices).x as libc::c_int >=
                   staticIndexMax as libc::c_int {
                *dynamicData.offset((*jointIndices).x as isize) as libc::c_int
            } else {
                *staticData.offset((*jointIndices).x as isize) as libc::c_int
            } as s16;
        (*frameTable).y =
            if (*jointIndices).y as libc::c_int >=
                   staticIndexMax as libc::c_int {
                *dynamicData.offset((*jointIndices).y as isize) as libc::c_int
            } else {
                *staticData.offset((*jointIndices).y as isize) as libc::c_int
            } as s16;
        (*frameTable).z =
            if (*jointIndices).z as libc::c_int >=
                   staticIndexMax as libc::c_int {
                *dynamicData.offset((*jointIndices).z as isize) as libc::c_int
            } else {
                *staticData.offset((*jointIndices).z as isize) as libc::c_int
            } as s16;
        i += 1;
        frameTable = frameTable.offset(1);
        jointIndices = jointIndices.offset(1)
    };
}
#[no_mangle]
pub unsafe extern "C" fn Animation_GetLength(mut animation: *mut libc::c_void)
 -> s16 {
    let mut common: *mut AnimationHeaderCommon =
        gSegments[((animation as u32_0) << 4 as libc::c_int >>
                       28 as libc::c_int) as
                      usize].wrapping_add(animation as u32_0 &
                                              0xffffff as libc::c_int as
                                                  libc::c_uint).wrapping_add(0x80000000
                                                                                 as
                                                                                 libc::c_uint)
            as *mut libc::c_void as *mut AnimationHeaderCommon;
    return (*common).frameCount;
}
#[no_mangle]
pub unsafe extern "C" fn Animation_GetLastFrame(mut animation:
                                                    *mut libc::c_void)
 -> s16 {
    let mut common: *mut AnimationHeaderCommon =
        gSegments[((animation as u32_0) << 4 as libc::c_int >>
                       28 as libc::c_int) as
                      usize].wrapping_add(animation as u32_0 &
                                              0xffffff as libc::c_int as
                                                  libc::c_uint).wrapping_add(0x80000000
                                                                                 as
                                                                                 libc::c_uint)
            as *mut libc::c_void as *mut AnimationHeaderCommon;
    // Loads an unsigned half for some reason.
    return ((*common).frameCount as u16_0 as libc::c_int - 1 as libc::c_int)
               as s16;
}
/* *
 * Draw a limb of type `StandardLimb` to the specified display buffer
 */
#[no_mangle]
pub unsafe extern "C" fn SkelAnime_DrawLimb(mut globalCtx: *mut GlobalContext,
                                            mut limbIndex: s32,
                                            mut skeleton:
                                                *mut *mut libc::c_void,
                                            mut jointTable: *mut Vec3s,
                                            mut overrideLimbDraw:
                                                OverrideLimbDraw,
                                            mut postLimbDraw: PostLimbDraw,
                                            mut arg: *mut libc::c_void,
                                            mut gfx: *mut Gfx) -> *mut Gfx {
    let mut limb: *mut StandardLimb = 0 as *mut StandardLimb;
    let mut dList: *mut Gfx = 0 as *mut Gfx;
    let mut pos: Vec3f = Vec3f{x: 0., y: 0., z: 0.,};
    let mut rot: Vec3s = Vec3s{x: 0, y: 0, z: 0,};
    Matrix_Push();
    limb =
        gSegments[((*skeleton.offset(limbIndex as isize) as u32_0) <<
                       4 as libc::c_int >> 28 as libc::c_int) as
                      usize].wrapping_add(*skeleton.offset(limbIndex as isize)
                                              as u32_0 &
                                              0xffffff as libc::c_int as
                                                  libc::c_uint).wrapping_add(0x80000000
                                                                                 as
                                                                                 libc::c_uint)
            as *mut libc::c_void as *mut StandardLimb;
    limbIndex += 1;
    rot = *jointTable.offset(limbIndex as isize);
    pos.x = (*limb).jointPos.x as f32_0;
    pos.y = (*limb).jointPos.y as f32_0;
    pos.z = (*limb).jointPos.z as f32_0;
    dList = (*limb).dList;
    if overrideLimbDraw.is_none() ||
           overrideLimbDraw.expect("non-null function pointer")(globalCtx,
                                                                limbIndex,
                                                                &mut dList,
                                                                &mut pos,
                                                                &mut rot, arg,
                                                                &mut gfx) == 0
       {
        Matrix_TranslateRotateZYX(&mut pos, &mut rot);
        if !dList.is_null() {
            let fresh18 = gfx;
            gfx = gfx.offset(1);
            let mut _g: *mut Gfx = fresh18;
            (*_g).words.w0 =
                (0xda as libc::c_int as u32_0 &
                     (((0x1 as libc::c_int) << 8 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    24 as libc::c_int |
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
                    ((0x2 as libc::c_int ^ 0x1 as libc::c_int) as u32_0 &
                         (((0x1 as libc::c_int) << 8 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        0 as libc::c_int;
            (*_g).words.w1 =
                Matrix_NewMtx((*globalCtx).state.gfxCtx,
                              b"../z_skelanime.c\x00" as *const u8 as
                                  *const libc::c_char as *mut libc::c_char,
                              1489 as libc::c_int) as libc::c_uint;
            let fresh19 = gfx;
            gfx = gfx.offset(1);
            let mut _g_0: *mut Gfx = fresh19;
            (*_g_0).words.w0 =
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
            (*_g_0).words.w1 = dList as libc::c_uint
        }
    }
    if postLimbDraw.is_some() {
        postLimbDraw.expect("non-null function pointer")(globalCtx, limbIndex,
                                                         &mut dList, &mut rot,
                                                         arg, &mut gfx);
    }
    if (*limb).child as libc::c_int != 0xff as libc::c_int {
        gfx =
            SkelAnime_DrawLimb(globalCtx, (*limb).child as s32, skeleton,
                               jointTable, overrideLimbDraw, postLimbDraw,
                               arg, gfx)
    }
    Matrix_Pop();
    if (*limb).sibling as libc::c_int != 0xff as libc::c_int {
        gfx =
            SkelAnime_DrawLimb(globalCtx, (*limb).sibling as s32, skeleton,
                               jointTable, overrideLimbDraw, postLimbDraw,
                               arg, gfx)
    }
    return gfx;
}
/* *
 * Draw all limbs of type `StandardLimb` in a given skeleton to the specified display buffer
 */
#[no_mangle]
pub unsafe extern "C" fn SkelAnime_Draw(mut globalCtx: *mut GlobalContext,
                                        mut skeleton: *mut *mut libc::c_void,
                                        mut jointTable: *mut Vec3s,
                                        mut overrideLimbDraw:
                                            OverrideLimbDraw,
                                        mut postLimbDraw: PostLimbDraw,
                                        mut arg: *mut libc::c_void,
                                        mut gfx: *mut Gfx) -> *mut Gfx {
    let mut rootLimb: *mut StandardLimb = 0 as *mut StandardLimb;
    let mut pad: s32 = 0;
    let mut dList: *mut Gfx = 0 as *mut Gfx;
    let mut pos: Vec3f = Vec3f{x: 0., y: 0., z: 0.,};
    let mut rot: Vec3s = Vec3s{x: 0, y: 0, z: 0,};
    if skeleton.is_null() {
        osSyncPrintf(b"\x1b[31m\x00" as *const u8 as *const libc::c_char);
        // "skel is NULL. Returns NULL."
        osSyncPrintf(b"Si2_draw2():skel\xe3\x81\x8cNULL\xe3\x81\xa7\xe3\x81\x99\xe3\x80\x82NULL\xe3\x82\x92\xe8\xbf\x94\xe3\x81\x97\xe3\x81\xbe\xe3\x81\x99\xe3\x80\x82\n\x00"
                         as *const u8 as *const libc::c_char);
        osSyncPrintf(b"\x1b[m\x00" as *const u8 as *const libc::c_char);
        return 0 as *mut Gfx
    }
    Matrix_Push();
    rootLimb =
        gSegments[((*skeleton.offset(0 as libc::c_int as isize) as u32_0) <<
                       4 as libc::c_int >> 28 as libc::c_int) as
                      usize].wrapping_add(*skeleton.offset(0 as libc::c_int as
                                                               isize) as u32_0
                                              &
                                              0xffffff as libc::c_int as
                                                  libc::c_uint).wrapping_add(0x80000000
                                                                                 as
                                                                                 libc::c_uint)
            as *mut libc::c_void as *mut StandardLimb;
    pos.x = (*jointTable.offset(0 as libc::c_int as isize)).x as f32_0;
    pos.y = (*jointTable.offset(0 as libc::c_int as isize)).y as f32_0;
    pos.z = (*jointTable.offset(0 as libc::c_int as isize)).z as f32_0;
    rot = *jointTable.offset(1 as libc::c_int as isize);
    dList = (*rootLimb).dList;
    if overrideLimbDraw.is_none() ||
           overrideLimbDraw.expect("non-null function pointer")(globalCtx,
                                                                1 as
                                                                    libc::c_int,
                                                                &mut dList,
                                                                &mut pos,
                                                                &mut rot, arg,
                                                                &mut gfx) == 0
       {
        Matrix_TranslateRotateZYX(&mut pos, &mut rot);
        if !dList.is_null() {
            let fresh20 = gfx;
            gfx = gfx.offset(1);
            let mut _g: *mut Gfx = fresh20;
            (*_g).words.w0 =
                (0xda as libc::c_int as u32_0 &
                     (((0x1 as libc::c_int) << 8 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    24 as libc::c_int |
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
                    ((0x2 as libc::c_int ^ 0x1 as libc::c_int) as u32_0 &
                         (((0x1 as libc::c_int) << 8 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        0 as libc::c_int;
            (*_g).words.w1 =
                Matrix_NewMtx((*globalCtx).state.gfxCtx,
                              b"../z_skelanime.c\x00" as *const u8 as
                                  *const libc::c_char as *mut libc::c_char,
                              1558 as libc::c_int) as libc::c_uint;
            let fresh21 = gfx;
            gfx = gfx.offset(1);
            let mut _g_0: *mut Gfx = fresh21;
            (*_g_0).words.w0 =
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
            (*_g_0).words.w1 = dList as libc::c_uint
        }
    }
    if postLimbDraw.is_some() {
        postLimbDraw.expect("non-null function pointer")(globalCtx,
                                                         1 as libc::c_int,
                                                         &mut dList, &mut rot,
                                                         arg, &mut gfx);
    }
    if (*rootLimb).child as libc::c_int != 0xff as libc::c_int {
        gfx =
            SkelAnime_DrawLimb(globalCtx, (*rootLimb).child as s32, skeleton,
                               jointTable, overrideLimbDraw, postLimbDraw,
                               arg, gfx)
    }
    Matrix_Pop();
    return gfx;
}
/* *
 * Draw a limb of type `StandardLimb` contained within a flexible skeleton to the specified display buffer
 */
#[no_mangle]
pub unsafe extern "C" fn SkelAnime_DrawFlexLimb(mut globalCtx:
                                                    *mut GlobalContext,
                                                mut limbIndex: s32,
                                                mut skeleton:
                                                    *mut *mut libc::c_void,
                                                mut jointTable: *mut Vec3s,
                                                mut overrideLimbDraw:
                                                    OverrideLimbDraw,
                                                mut postLimbDraw:
                                                    PostLimbDraw,
                                                mut arg: *mut libc::c_void,
                                                mut mtx: *mut *mut Mtx,
                                                mut gfx: *mut Gfx)
 -> *mut Gfx {
    let mut limb: *mut StandardLimb = 0 as *mut StandardLimb;
    let mut newDList: *mut Gfx = 0 as *mut Gfx;
    let mut limbDList: *mut Gfx = 0 as *mut Gfx;
    let mut pos: Vec3f = Vec3f{x: 0., y: 0., z: 0.,};
    let mut rot: Vec3s = Vec3s{x: 0, y: 0, z: 0,};
    Matrix_Push();
    limb =
        gSegments[((*skeleton.offset(limbIndex as isize) as u32_0) <<
                       4 as libc::c_int >> 28 as libc::c_int) as
                      usize].wrapping_add(*skeleton.offset(limbIndex as isize)
                                              as u32_0 &
                                              0xffffff as libc::c_int as
                                                  libc::c_uint).wrapping_add(0x80000000
                                                                                 as
                                                                                 libc::c_uint)
            as *mut libc::c_void as *mut StandardLimb;
    limbIndex += 1;
    rot = *jointTable.offset(limbIndex as isize);
    pos.x = (*limb).jointPos.x as f32_0;
    pos.y = (*limb).jointPos.y as f32_0;
    pos.z = (*limb).jointPos.z as f32_0;
    limbDList = (*limb).dList;
    newDList = limbDList;
    if overrideLimbDraw.is_none() ||
           overrideLimbDraw.expect("non-null function pointer")(globalCtx,
                                                                limbIndex,
                                                                &mut newDList,
                                                                &mut pos,
                                                                &mut rot, arg,
                                                                &mut gfx) == 0
       {
        Matrix_TranslateRotateZYX(&mut pos, &mut rot);
        if !newDList.is_null() {
            Matrix_ToMtx(*mtx,
                         b"../z_skelanime.c\x00" as *const u8 as
                             *const libc::c_char as *mut libc::c_char,
                         1623 as libc::c_int);
            let fresh22 = gfx;
            gfx = gfx.offset(1);
            let mut _g: *mut Gfx = fresh22;
            (*_g).words.w0 =
                (0xda as libc::c_int as u32_0 &
                     (((0x1 as libc::c_int) << 8 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    24 as libc::c_int |
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
                    ((0x2 as libc::c_int ^ 0x1 as libc::c_int) as u32_0 &
                         (((0x1 as libc::c_int) << 8 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        0 as libc::c_int;
            (*_g).words.w1 = *mtx as libc::c_uint;
            let fresh23 = gfx;
            gfx = gfx.offset(1);
            let mut _g_0: *mut Gfx = fresh23;
            (*_g_0).words.w0 =
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
            (*_g_0).words.w1 = newDList as libc::c_uint;
            *mtx = (*mtx).offset(1)
        } else if !limbDList.is_null() {
            Matrix_ToMtx(*mtx,
                         b"../z_skelanime.c\x00" as *const u8 as
                             *const libc::c_char as *mut libc::c_char,
                         1630 as libc::c_int);
            *mtx = (*mtx).offset(1)
        }
    }
    if postLimbDraw.is_some() {
        postLimbDraw.expect("non-null function pointer")(globalCtx, limbIndex,
                                                         &mut limbDList,
                                                         &mut rot, arg,
                                                         &mut gfx);
    }
    if (*limb).child as libc::c_int != 0xff as libc::c_int {
        gfx =
            SkelAnime_DrawFlexLimb(globalCtx, (*limb).child as s32, skeleton,
                                   jointTable, overrideLimbDraw, postLimbDraw,
                                   arg, mtx, gfx)
    }
    Matrix_Pop();
    if (*limb).sibling as libc::c_int != 0xff as libc::c_int {
        gfx =
            SkelAnime_DrawFlexLimb(globalCtx, (*limb).sibling as s32,
                                   skeleton, jointTable, overrideLimbDraw,
                                   postLimbDraw, arg, mtx, gfx)
    }
    return gfx;
}
/* *
 * Draw all limbs of type `StandardLimb` in a given flexible skeleton to the specified display buffer
 * Limbs in a flexible skeleton have meshes that can stretch to line up with other limbs.
 * An array of matrices is dynamically allocated so each limb can access any transform to ensure its meshes line up.
 */
#[no_mangle]
pub unsafe extern "C" fn SkelAnime_DrawFlex(mut globalCtx: *mut GlobalContext,
                                            mut skeleton:
                                                *mut *mut libc::c_void,
                                            mut jointTable: *mut Vec3s,
                                            mut dListCount: s32,
                                            mut overrideLimbDraw:
                                                OverrideLimbDraw,
                                            mut postLimbDraw: PostLimbDraw,
                                            mut arg: *mut libc::c_void,
                                            mut gfx: *mut Gfx) -> *mut Gfx {
    let mut rootLimb: *mut StandardLimb = 0 as *mut StandardLimb;
    let mut pad: s32 = 0;
    let mut newDList: *mut Gfx = 0 as *mut Gfx;
    let mut limbDList: *mut Gfx = 0 as *mut Gfx;
    let mut pos: Vec3f = Vec3f{x: 0., y: 0., z: 0.,};
    let mut rot: Vec3s = Vec3s{x: 0, y: 0, z: 0,};
    let mut mtx: *mut Mtx =
        Graph_Alloc((*globalCtx).state.gfxCtx,
                    (dListCount as
                         libc::c_uint).wrapping_mul(::std::mem::size_of::<Mtx>()
                                                        as libc::c_ulong) as
                        size_t) as *mut Mtx;
    if skeleton.is_null() {
        osSyncPrintf(b"\x1b[31m\x00" as *const u8 as *const libc::c_char);
        // "skel is NULL. Returns NULL."
        osSyncPrintf(b"Si2_draw2_SV():skel\xe3\x81\x8cNULL\xe3\x81\xa7\xe3\x81\x99\xe3\x80\x82NULL\xe3\x82\x92\xe8\xbf\x94\xe3\x81\x97\xe3\x81\xbe\xe3\x81\x99\xe3\x80\x82\n\x00"
                         as *const u8 as *const libc::c_char);
        osSyncPrintf(b"\x1b[m\x00" as *const u8 as *const libc::c_char);
        return 0 as *mut Gfx
    }
    let fresh24 = gfx;
    gfx = gfx.offset(1);
    let mut _g: *mut Gfx = fresh24;
    (*_g).words.w0 =
        (0xdb as libc::c_int as u32_0 &
             (((0x1 as libc::c_int) << 8 as libc::c_int) - 1 as libc::c_int)
                 as libc::c_uint) << 24 as libc::c_int |
            (0x6 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 16 as libc::c_int
            |
            ((0xd as libc::c_int * 4 as libc::c_int) as u32_0 &
                 (((0x1 as libc::c_int) << 16 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 0 as libc::c_int;
    (*_g).words.w1 = mtx as libc::c_uint;
    Matrix_Push();
    rootLimb =
        gSegments[((*skeleton.offset(0 as libc::c_int as isize) as u32_0) <<
                       4 as libc::c_int >> 28 as libc::c_int) as
                      usize].wrapping_add(*skeleton.offset(0 as libc::c_int as
                                                               isize) as u32_0
                                              &
                                              0xffffff as libc::c_int as
                                                  libc::c_uint).wrapping_add(0x80000000
                                                                                 as
                                                                                 libc::c_uint)
            as *mut libc::c_void as *mut StandardLimb;
    pos.x = (*jointTable.offset(0 as libc::c_int as isize)).x as f32_0;
    pos.y = (*jointTable.offset(0 as libc::c_int as isize)).y as f32_0;
    pos.z = (*jointTable.offset(0 as libc::c_int as isize)).z as f32_0;
    rot = *jointTable.offset(1 as libc::c_int as isize);
    limbDList = (*rootLimb).dList;
    newDList = limbDList;
    if overrideLimbDraw.is_none() ||
           overrideLimbDraw.expect("non-null function pointer")(globalCtx,
                                                                1 as
                                                                    libc::c_int,
                                                                &mut newDList,
                                                                &mut pos,
                                                                &mut rot, arg,
                                                                &mut gfx) == 0
       {
        Matrix_TranslateRotateZYX(&mut pos, &mut rot);
        if !newDList.is_null() {
            Matrix_ToMtx(mtx,
                         b"../z_skelanime.c\x00" as *const u8 as
                             *const libc::c_char as *mut libc::c_char,
                         1710 as libc::c_int);
            let fresh25 = gfx;
            gfx = gfx.offset(1);
            let mut _g_0: *mut Gfx = fresh25;
            (*_g_0).words.w0 =
                (0xda as libc::c_int as u32_0 &
                     (((0x1 as libc::c_int) << 8 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    24 as libc::c_int |
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
                    ((0x2 as libc::c_int ^ 0x1 as libc::c_int) as u32_0 &
                         (((0x1 as libc::c_int) << 8 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        0 as libc::c_int;
            (*_g_0).words.w1 = mtx as libc::c_uint;
            let fresh26 = gfx;
            gfx = gfx.offset(1);
            let mut _g_1: *mut Gfx = fresh26;
            (*_g_1).words.w0 =
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
            (*_g_1).words.w1 = newDList as libc::c_uint;
            mtx = mtx.offset(1)
        } else if !limbDList.is_null() {
            Matrix_ToMtx(mtx,
                         b"../z_skelanime.c\x00" as *const u8 as
                             *const libc::c_char as *mut libc::c_char,
                         1717 as libc::c_int);
            mtx = mtx.offset(1)
        }
    }
    if postLimbDraw.is_some() {
        postLimbDraw.expect("non-null function pointer")(globalCtx,
                                                         1 as libc::c_int,
                                                         &mut limbDList,
                                                         &mut rot, arg,
                                                         &mut gfx);
    }
    if (*rootLimb).child as libc::c_int != 0xff as libc::c_int {
        gfx =
            SkelAnime_DrawFlexLimb(globalCtx, (*rootLimb).child as s32,
                                   skeleton, jointTable, overrideLimbDraw,
                                   postLimbDraw, arg, &mut mtx, gfx)
    }
    Matrix_Pop();
    return gfx;
}
/* *
 * Unpacks frame data for the animation at the given frame into frameTable
 * Used by the legacy animation format
 */
#[no_mangle]
pub unsafe extern "C" fn SkelAnime_GetFrameDataLegacy(mut animation:
                                                          *mut LegacyAnimationHeader,
                                                      mut frame: s32,
                                                      mut frameTable:
                                                          *mut Vec3s) -> s32 {
    let mut animHeader: *mut LegacyAnimationHeader =
        gSegments[((animation as u32_0) << 4 as libc::c_int >>
                       28 as libc::c_int) as
                      usize].wrapping_add(animation as u32_0 &
                                              0xffffff as libc::c_int as
                                                  libc::c_uint).wrapping_add(0x80000000
                                                                                 as
                                                                                 libc::c_uint)
            as *mut libc::c_void as *mut LegacyAnimationHeader;
    let mut limbCount: s32 = (*animHeader).limbCount as s32;
    let mut key: *mut JointKey =
        gSegments[(((*animHeader).jointKey as u32_0) << 4 as libc::c_int >>
                       28 as libc::c_int) as
                      usize].wrapping_add((*animHeader).jointKey as u32_0 &
                                              0xffffff as libc::c_int as
                                                  libc::c_uint).wrapping_add(0x80000000
                                                                                 as
                                                                                 libc::c_uint)
            as *mut libc::c_void as *mut JointKey;
    let mut frameData: *mut s16 =
        gSegments[(((*animHeader).frameData as u32_0) << 4 as libc::c_int >>
                       28 as libc::c_int) as
                      usize].wrapping_add((*animHeader).frameData as u32_0 &
                                              0xffffff as libc::c_int as
                                                  libc::c_uint).wrapping_add(0x80000000
                                                                                 as
                                                                                 libc::c_uint)
            as *mut libc::c_void as *mut s16;
    let mut staticData: *mut s16 =
        &mut *frameData.offset(0 as libc::c_int as isize) as *mut s16;
    let mut dynamicData: *mut s16 =
        &mut *frameData.offset(frame as isize) as *mut s16;
    let mut i: s32 = 0;
    (*frameTable).x =
        if frame < (*key).xMax as libc::c_int {
            *dynamicData.offset((*key).x as isize) as libc::c_int
        } else { *staticData.offset((*key).x as isize) as libc::c_int } as
            s16;
    (*frameTable).y =
        if frame < (*key).yMax as libc::c_int {
            *dynamicData.offset((*key).y as isize) as libc::c_int
        } else { *staticData.offset((*key).y as isize) as libc::c_int } as
            s16;
    (*frameTable).z =
        if frame < (*key).zMax as libc::c_int {
            *dynamicData.offset((*key).z as isize) as libc::c_int
        } else { *staticData.offset((*key).z as isize) as libc::c_int } as
            s16;
    frameTable = frameTable.offset(1);
    key = key.offset(1);
    i = 1 as libc::c_int;
    while i <= limbCount {
        (*frameTable).x =
            if frame < (*key).xMax as libc::c_int {
                *dynamicData.offset((*key).x as isize) as libc::c_int
            } else { *staticData.offset((*key).x as isize) as libc::c_int } as
                s16;
        (*frameTable).y =
            if frame < (*key).yMax as libc::c_int {
                *dynamicData.offset((*key).y as isize) as libc::c_int
            } else { *staticData.offset((*key).y as isize) as libc::c_int } as
                s16;
        (*frameTable).z =
            if frame < (*key).zMax as libc::c_int {
                *dynamicData.offset((*key).z as isize) as libc::c_int
            } else { *staticData.offset((*key).z as isize) as libc::c_int } as
                s16;
        i += 1;
        key = key.offset(1);
        frameTable = frameTable.offset(1)
    }
    return limbCount;
}
/* *
 * Used by legacy animation format
 */
#[no_mangle]
pub unsafe extern "C" fn Animation_GetLimbCountLegacy(mut animation:
                                                          *mut LegacyAnimationHeader)
 -> s16 {
    let mut animHeader: *mut LegacyAnimationHeader =
        gSegments[((animation as u32_0) << 4 as libc::c_int >>
                       28 as libc::c_int) as
                      usize].wrapping_add(animation as u32_0 &
                                              0xffffff as libc::c_int as
                                                  libc::c_uint).wrapping_add(0x80000000
                                                                                 as
                                                                                 libc::c_uint)
            as *mut libc::c_void as *mut LegacyAnimationHeader;
    return (*animHeader).limbCount;
}
/* *
 * Used by legacy animation format
 */
#[no_mangle]
pub unsafe extern "C" fn Animation_GetLengthLegacy(mut animation:
                                                       *mut LegacyAnimationHeader)
 -> s16 {
    let mut animHeader: *mut LegacyAnimationHeader =
        gSegments[((animation as u32_0) << 4 as libc::c_int >>
                       28 as libc::c_int) as
                      usize].wrapping_add(animation as u32_0 &
                                              0xffffff as libc::c_int as
                                                  libc::c_uint).wrapping_add(0x80000000
                                                                                 as
                                                                                 libc::c_uint)
            as *mut libc::c_void as *mut LegacyAnimationHeader;
    return (*animHeader).frameCount;
}
/* *
 * Used by legacy animation format
 */
#[no_mangle]
pub unsafe extern "C" fn Animation_GetLastFrameLegacy(mut animation:
                                                          *mut LegacyAnimationHeader)
 -> s16 {
    let mut animHeader: *mut LegacyAnimationHeader =
        gSegments[((animation as u32_0) << 4 as libc::c_int >>
                       28 as libc::c_int) as
                      usize].wrapping_add(animation as u32_0 &
                                              0xffffff as libc::c_int as
                                                  libc::c_uint).wrapping_add(0x80000000
                                                                                 as
                                                                                 libc::c_uint)
            as *mut libc::c_void as *mut LegacyAnimationHeader;
    return ((*animHeader).frameCount as libc::c_int - 1 as libc::c_int) as
               s16;
}
/* *
 * Linearly interpolates the start and target frame tables with the given weight, putting the result in dst
 */
#[no_mangle]
pub unsafe extern "C" fn SkelAnime_InterpFrameTable(mut limbCount: s32,
                                                    mut dst: *mut Vec3s,
                                                    mut start: *mut Vec3s,
                                                    mut target: *mut Vec3s,
                                                    mut weight: f32_0) {
    let mut i: s32 = 0;
    let mut diff: s16 = 0;
    let mut base: s16 = 0;
    if weight < 1.0f32 {
        i = 0 as libc::c_int;
        while i < limbCount {
            base = (*start).x;
            diff = ((*target).x as libc::c_int - base as libc::c_int) as s16;
            (*dst).x =
                ((diff as libc::c_int as libc::c_float * weight) as s16 as
                     libc::c_int + base as libc::c_int) as s16;
            base = (*start).y;
            diff = ((*target).y as libc::c_int - base as libc::c_int) as s16;
            (*dst).y =
                ((diff as libc::c_int as libc::c_float * weight) as s16 as
                     libc::c_int + base as libc::c_int) as s16;
            base = (*start).z;
            diff = ((*target).z as libc::c_int - base as libc::c_int) as s16;
            (*dst).z =
                ((diff as libc::c_int as libc::c_float * weight) as s16 as
                     libc::c_int + base as libc::c_int) as s16;
            i += 1;
            dst = dst.offset(1);
            start = start.offset(1);
            target = target.offset(1)
        }
    } else {
        i = 0 as libc::c_int;
        while i < limbCount {
            (*dst).x = (*target).x;
            (*dst).y = (*target).y;
            (*dst).z = (*target).z;
            i += 1;
            dst = dst.offset(1);
            target = target.offset(1)
        }
    };
}
/* *
 * zeroes out the current request count
 */
#[no_mangle]
pub unsafe extern "C" fn AnimationContext_Reset(mut animationCtx:
                                                    *mut AnimationContext) {
    (*animationCtx).animationCount = 0 as libc::c_int as s16;
}
/* *
 * Shifts the queue flag to the next queue
 */
#[no_mangle]
pub unsafe extern "C" fn AnimationContext_SetNextQueue(mut globalCtx:
                                                           *mut GlobalContext) {
    sAnimQueueFlags <<= 1 as libc::c_int;
}
/* *
 * Disables the current animation queue. Only load and move actor requests will be processed for that queue.
 */
#[no_mangle]
pub unsafe extern "C" fn AnimationContext_DisableQueue(mut globalCtx:
                                                           *mut GlobalContext) {
    sDisableAnimQueueFlags |= sAnimQueueFlags;
}
#[no_mangle]
pub unsafe extern "C" fn AnimationContext_AddEntry(mut animationCtx:
                                                       *mut AnimationContext,
                                                   mut type_0: AnimationType)
 -> *mut AnimationEntry {
    let mut entry: *mut AnimationEntry = 0 as *mut AnimationEntry;
    let mut index: s16 = (*animationCtx).animationCount;
    if index as libc::c_int >= 50 as libc::c_int {
        return 0 as *mut AnimationEntry
    }
    (*animationCtx).animationCount =
        (index as libc::c_int + 1 as libc::c_int) as s16;
    entry =
        &mut *(*animationCtx).entries.as_mut_ptr().offset(index as isize) as
            *mut AnimationEntry;
    (*entry).type_0 = type_0 as u8_0;
    return entry;
}
/* *
 * Requests loading frame data from the Link animation into frameTable
 */
#[no_mangle]
pub unsafe extern "C" fn AnimationContext_SetLoadFrame(mut globalCtx:
                                                           *mut GlobalContext,
                                                       mut animation:
                                                           *mut LinkAnimationHeader,
                                                       mut frame: s32,
                                                       mut limbCount: s32,
                                                       mut frameTable:
                                                           *mut Vec3s) {
    let mut entry: *mut AnimationEntry =
        AnimationContext_AddEntry(&mut (*globalCtx).animationCtx,
                                  ANIMENTRY_LOADFRAME);
    if !entry.is_null() {
        let mut linkAnimHeader: *mut LinkAnimationHeader =
            gSegments[((animation as u32_0) << 4 as libc::c_int >>
                           28 as libc::c_int) as
                          usize].wrapping_add(animation as u32_0 &
                                                  0xffffff as libc::c_int as
                                                      libc::c_uint).wrapping_add(0x80000000
                                                                                     as
                                                                                     libc::c_uint)
                as *mut libc::c_void as *mut LinkAnimationHeader;
        let mut ram: u32_0 = frameTable as u32_0;
        osCreateMesgQueue(&mut (*entry).data.load.msgQueue,
                          &mut (*entry).data.load.msg, 1 as libc::c_int);
        DmaMgr_SendRequest2(&mut (*entry).data.load.req, ram,
                            (_link_animetionSegmentRomStart.as_mut_ptr() as
                                 u32_0).wrapping_add((*linkAnimHeader).segment
                                                         as
                                                         u32_0).wrapping_sub(_link_animetionSegmentStart.as_mut_ptr()
                                                                                 as
                                                                                 u32_0).wrapping_add((::std::mem::size_of::<Vec3s>()
                                                                                                          as
                                                                                                          libc::c_ulong).wrapping_mul(limbCount
                                                                                                                                          as
                                                                                                                                          libc::c_uint).wrapping_add(2
                                                                                                                                                                         as
                                                                                                                                                                         libc::c_int
                                                                                                                                                                         as
                                                                                                                                                                         libc::c_uint).wrapping_mul(frame
                                                                                                                                                                                                        as
                                                                                                                                                                                                        libc::c_uint)),
                            (::std::mem::size_of::<Vec3s>() as
                                 libc::c_ulong).wrapping_mul(limbCount as
                                                                 libc::c_uint).wrapping_add(2
                                                                                                as
                                                                                                libc::c_int
                                                                                                as
                                                                                                libc::c_uint),
                            0 as libc::c_int as u32_0,
                            &mut (*entry).data.load.msgQueue,
                            0 as *mut libc::c_void,
                            b"../z_skelanime.c\x00" as *const u8 as
                                *const libc::c_char, 2004 as libc::c_int);
    };
}
/* *
 * Requests copying all vectors from src frame table into dst frame table
 */
#[no_mangle]
pub unsafe extern "C" fn AnimationContext_SetCopyAll(mut globalCtx:
                                                         *mut GlobalContext,
                                                     mut vecCount: s32,
                                                     mut dst: *mut Vec3s,
                                                     mut src: *mut Vec3s) {
    let mut entry: *mut AnimationEntry =
        AnimationContext_AddEntry(&mut (*globalCtx).animationCtx,
                                  ANIMENTRY_COPYALL);
    if !entry.is_null() {
        (*entry).data.copy.queueFlag = sAnimQueueFlags as u8_0;
        (*entry).data.copy.vecCount = vecCount as u8_0;
        (*entry).data.copy.dst = dst;
        (*entry).data.copy.src = src
    };
}
/* *
 * Requests interpolating between base and mod frame tables with the given weight, placing the result in base
 */
#[no_mangle]
pub unsafe extern "C" fn AnimationContext_SetInterp(mut globalCtx:
                                                        *mut GlobalContext,
                                                    mut vecCount: s32,
                                                    mut base: *mut Vec3s,
                                                    mut mod_0: *mut Vec3s,
                                                    mut weight: f32_0) {
    let mut entry: *mut AnimationEntry =
        AnimationContext_AddEntry(&mut (*globalCtx).animationCtx,
                                  ANIMENTRY_INTERP);
    if !entry.is_null() {
        (*entry).data.interp.queueFlag = sAnimQueueFlags as u8_0;
        (*entry).data.interp.vecCount = vecCount as u8_0;
        (*entry).data.interp.base = base;
        (*entry).data.interp.mod_0 = mod_0;
        (*entry).data.interp.weight = weight
    };
}
/* *
 * Requests copying vectors from src frame table to dst frame table whose copy flag is true
 */
#[no_mangle]
pub unsafe extern "C" fn AnimationContext_SetCopyTrue(mut globalCtx:
                                                          *mut GlobalContext,
                                                      mut vecCount: s32,
                                                      mut dst: *mut Vec3s,
                                                      mut src: *mut Vec3s,
                                                      mut copyFlag:
                                                          *mut u8_0) {
    let mut entry: *mut AnimationEntry =
        AnimationContext_AddEntry(&mut (*globalCtx).animationCtx,
                                  ANIMENTRY_COPYTRUE);
    if !entry.is_null() {
        (*entry).data.copy1.queueFlag = sAnimQueueFlags as u8_0;
        (*entry).data.copy1.vecCount = vecCount as u8_0;
        (*entry).data.copy1.dst = dst;
        (*entry).data.copy1.src = src;
        (*entry).data.copy1.copyFlag = copyFlag
    };
}
/* *
 * Requests copying vectors from src frame table to dst frame table whose copy flag is false
 */
#[no_mangle]
pub unsafe extern "C" fn AnimationContext_SetCopyFalse(mut globalCtx:
                                                           *mut GlobalContext,
                                                       mut vecCount: s32,
                                                       mut dst: *mut Vec3s,
                                                       mut src: *mut Vec3s,
                                                       mut copyFlag:
                                                           *mut u8_0) {
    let mut entry: *mut AnimationEntry =
        AnimationContext_AddEntry(&mut (*globalCtx).animationCtx,
                                  ANIMENTRY_COPYFALSE);
    if !entry.is_null() {
        (*entry).data.copy0.queueFlag = sAnimQueueFlags as u8_0;
        (*entry).data.copy0.vecCount = vecCount as u8_0;
        (*entry).data.copy0.dst = dst;
        (*entry).data.copy0.src = src;
        (*entry).data.copy0.copyFlag = copyFlag
    };
}
/* *
 * Requests moving an actor according to the translation of its root limb
 */
#[no_mangle]
pub unsafe extern "C" fn AnimationContext_SetMoveActor(mut globalCtx:
                                                           *mut GlobalContext,
                                                       mut actor: *mut Actor,
                                                       mut skelAnime:
                                                           *mut SkelAnime,
                                                       mut arg3: f32_0) {
    let mut entry: *mut AnimationEntry =
        AnimationContext_AddEntry(&mut (*globalCtx).animationCtx,
                                  ANIMENTRY_MOVEACTOR);
    if !entry.is_null() {
        (*entry).data.move_0.actor = actor;
        (*entry).data.move_0.skelAnime = skelAnime;
        (*entry).data.move_0.unk_08 = arg3
    };
}
/* *
 * Receives the request for Link's animation frame data
 */
#[no_mangle]
pub unsafe extern "C" fn AnimationContext_LoadFrame(mut globalCtx:
                                                        *mut GlobalContext,
                                                    mut data:
                                                        *mut AnimationEntryData) {
    let mut entry: *mut AnimEntryLoadFrame = &mut (*data).load;
    osRecvMesg(&mut (*entry).msgQueue, 0 as *mut OSMesg, 1 as libc::c_int);
}
/* *
 * If the entry's queue is enabled, copies all vectors from src frame table to dst frame table
 */
#[no_mangle]
pub unsafe extern "C" fn AnimationContext_CopyAll(mut globalCtx:
                                                      *mut GlobalContext,
                                                  mut data:
                                                      *mut AnimationEntryData) {
    let mut entry: *mut AnimEntryCopyAll = &mut (*data).copy;
    if (*entry).queueFlag as libc::c_uint & sDisableAnimQueueFlags == 0 {
        let mut dst: *mut Vec3s = (*entry).dst;
        let mut src: *mut Vec3s = (*entry).src;
        let mut i: s32 = 0;
        i = 0 as libc::c_int;
        while i < (*entry).vecCount as libc::c_int {
            let fresh27 = src;
            src = src.offset(1);
            let fresh28 = dst;
            dst = dst.offset(1);
            *fresh28 = *fresh27;
            i += 1
        }
    };
}
/* *
 * If the entry's queue is enabled, interpolates between the base and mod frame tables, placing the result in base
 */
#[no_mangle]
pub unsafe extern "C" fn AnimationContext_Interp(mut globalCtx:
                                                     *mut GlobalContext,
                                                 mut data:
                                                     *mut AnimationEntryData) {
    let mut entry: *mut AnimEntryInterp = &mut (*data).interp;
    if (*entry).queueFlag as libc::c_uint & sDisableAnimQueueFlags == 0 {
        SkelAnime_InterpFrameTable((*entry).vecCount as s32, (*entry).base,
                                   (*entry).base, (*entry).mod_0,
                                   (*entry).weight);
    };
}
/* *
 * If the entry's queue is enabled, copies all vectors from src frame table to dst frame table whose copy flag is true
 */
#[no_mangle]
pub unsafe extern "C" fn AnimationContext_CopyTrue(mut globalCtx:
                                                       *mut GlobalContext,
                                                   mut data:
                                                       *mut AnimationEntryData) {
    let mut entry: *mut AnimEntryCopyTrue = &mut (*data).copy1;
    if (*entry).queueFlag as libc::c_uint & sDisableAnimQueueFlags == 0 {
        let mut dst: *mut Vec3s = (*entry).dst;
        let mut src: *mut Vec3s = (*entry).src;
        let mut copyFlag: *mut u8_0 = (*entry).copyFlag;
        let mut i: s32 = 0;
        i = 0 as libc::c_int;
        while i < (*entry).vecCount as libc::c_int {
            let fresh29 = copyFlag;
            copyFlag = copyFlag.offset(1);
            if *fresh29 != 0 { *dst = *src }
            i += 1;
            dst = dst.offset(1);
            src = src.offset(1)
        }
    };
}
/* *
 * If the entry's queue is enabled, copies all vectors from src frame table to dst frame table whose copy flag is false
 */
#[no_mangle]
pub unsafe extern "C" fn AnimationContext_CopyFalse(mut globalCtx:
                                                        *mut GlobalContext,
                                                    mut data:
                                                        *mut AnimationEntryData) {
    let mut entry: *mut AnimEntryCopyFalse = &mut (*data).copy0;
    if (*entry).queueFlag as libc::c_uint & sDisableAnimQueueFlags == 0 {
        let mut dst: *mut Vec3s = (*entry).dst;
        let mut src: *mut Vec3s = (*entry).src;
        let mut copyFlag: *mut u8_0 = (*entry).copyFlag;
        let mut i: s32 = 0;
        i = 0 as libc::c_int;
        while i < (*entry).vecCount as libc::c_int {
            let fresh30 = copyFlag;
            copyFlag = copyFlag.offset(1);
            if *fresh30 == 0 { *dst = *src }
            i += 1;
            dst = dst.offset(1);
            src = src.offset(1)
        }
    };
}
/* *
 * Moves an actor according to the translation of its root limb
 */
#[no_mangle]
pub unsafe extern "C" fn AnimationContext_MoveActor(mut globalCtx:
                                                        *mut GlobalContext,
                                                    mut data:
                                                        *mut AnimationEntryData) {
    let mut entry: *mut AnimEntryMoveActor = &mut (*data).move_0;
    let mut actor: *mut Actor = (*entry).actor;
    let mut diff: Vec3f = Vec3f{x: 0., y: 0., z: 0.,};
    SkelAnime_UpdateTranslation((*entry).skelAnime, &mut diff,
                                (*actor).shape.rot.y);
    (*actor).world.pos.x += diff.x * (*actor).scale.x;
    (*actor).world.pos.y += diff.y * (*actor).scale.y * (*entry).unk_08;
    (*actor).world.pos.z += diff.z * (*actor).scale.z;
}
/* *
 * Performs all requests in the animation queue, then resets the queue flags.
 */
#[no_mangle]
pub unsafe extern "C" fn AnimationContext_Update(mut globalCtx:
                                                     *mut GlobalContext,
                                                 mut animationCtx:
                                                     *mut AnimationContext) {
    static mut animFuncs: [AnimationEntryCallback; 6] =
        unsafe {
            [Some(AnimationContext_LoadFrame as
                      unsafe extern "C" fn(_: *mut GlobalContext,
                                           _: *mut AnimationEntryData) -> ()),
             Some(AnimationContext_CopyAll as
                      unsafe extern "C" fn(_: *mut GlobalContext,
                                           _: *mut AnimationEntryData) -> ()),
             Some(AnimationContext_Interp as
                      unsafe extern "C" fn(_: *mut GlobalContext,
                                           _: *mut AnimationEntryData) -> ()),
             Some(AnimationContext_CopyTrue as
                      unsafe extern "C" fn(_: *mut GlobalContext,
                                           _: *mut AnimationEntryData) -> ()),
             Some(AnimationContext_CopyFalse as
                      unsafe extern "C" fn(_: *mut GlobalContext,
                                           _: *mut AnimationEntryData) -> ()),
             Some(AnimationContext_MoveActor as
                      unsafe extern "C" fn(_: *mut GlobalContext,
                                           _: *mut AnimationEntryData) -> ())]
        };
    let mut entry: *mut AnimationEntry = 0 as *mut AnimationEntry;
    entry = (*animationCtx).entries.as_mut_ptr();
    while (*animationCtx).animationCount as libc::c_int != 0 as libc::c_int {
        animFuncs[(*entry).type_0 as
                      usize].expect("non-null function pointer")(globalCtx,
                                                                 &mut (*entry).data);
        entry = entry.offset(1);
        (*animationCtx).animationCount -= 1
    }
    sAnimQueueFlags = 1 as libc::c_int as u32_0;
    sDisableAnimQueueFlags = 0 as libc::c_int as u32_0;
}
/* *
 * Initializes a skeleton to be used with Link animations to a looping animation, dynamically allocating the frame
 * tables if not given.
 */
#[no_mangle]
pub unsafe extern "C" fn SkelAnime_InitLink(mut globalCtx: *mut GlobalContext,
                                            mut skelAnime: *mut SkelAnime,
                                            mut skeletonHeaderSeg:
                                                *mut FlexSkeletonHeader,
                                            mut animation:
                                                *mut LinkAnimationHeader,
                                            mut flags: s32,
                                            mut jointTable: *mut Vec3s,
                                            mut morphTable: *mut Vec3s,
                                            mut limbBufCount: s32) {
    let mut skeletonHeader: *mut FlexSkeletonHeader =
        gSegments[((skeletonHeaderSeg as u32_0) << 4 as libc::c_int >>
                       28 as libc::c_int) as
                      usize].wrapping_add(skeletonHeaderSeg as u32_0 &
                                              0xffffff as libc::c_int as
                                                  libc::c_uint).wrapping_add(0x80000000
                                                                                 as
                                                                                 libc::c_uint)
            as *mut libc::c_void as *mut FlexSkeletonHeader;
    let mut headerJointCount: s32 = (*skeletonHeader).sh.limbCount as s32;
    let mut limbCount: s32 = 0;
    let mut allocSize: size_t = 0;
    (*skelAnime).initFlags = flags as s8;
    limbCount =
        if flags & 2 as libc::c_int != 0 {
            headerJointCount
        } else { 1 as libc::c_int };
    if flags & 1 as libc::c_int != 0 { limbCount += headerJointCount }
    if flags & 4 as libc::c_int != 0 { limbCount += headerJointCount }
    (*skelAnime).limbCount = limbCount as u8_0;
    (*skelAnime).dListCount = (*skeletonHeader).dListCount;
    allocSize =
        (limbCount as
             libc::c_uint).wrapping_mul(::std::mem::size_of::<Vec3s>() as
                                            libc::c_ulong) as size_t;
    (*skelAnime).skeleton =
        gSegments[(((*skeletonHeader).sh.segment as u32_0) << 4 as libc::c_int
                       >> 28 as libc::c_int) as
                      usize].wrapping_add((*skeletonHeader).sh.segment as
                                              u32_0 &
                                              0xffffff as libc::c_int as
                                                  libc::c_uint).wrapping_add(0x80000000
                                                                                 as
                                                                                 libc::c_uint)
            as *mut libc::c_void as *mut *mut libc::c_void;
    if flags & 8 as libc::c_int != 0 {
        allocSize =
            (allocSize as
                 libc::c_ulong).wrapping_add(2 as libc::c_int as
                                                 libc::c_ulong) as size_t as
                size_t
    }
    if jointTable.is_null() {
        (*skelAnime).jointTable =
            ZeldaArena_MallocDebug(allocSize as u32_0,
                                   b"../z_skelanime.c\x00" as *const u8 as
                                       *const libc::c_char,
                                   2364 as libc::c_int) as *mut Vec3s;
        (*skelAnime).morphTable =
            ZeldaArena_MallocDebug(allocSize as u32_0,
                                   b"../z_skelanime.c\x00" as *const u8 as
                                       *const libc::c_char,
                                   2365 as libc::c_int) as *mut Vec3s
    } else {
        if limbBufCount == limbCount {
        } else {
            __assert(b"joint_buff_num == joint_num\x00" as *const u8 as
                         *const libc::c_char,
                     b"../z_skelanime.c\x00" as *const u8 as
                         *const libc::c_char, 2369 as libc::c_int);
        };
        (*skelAnime).jointTable =
            ((jointTable as
                  u32_0).wrapping_add(0xf as libc::c_int as libc::c_uint) &
                 !(0xf as libc::c_int) as libc::c_uint) as *mut Vec3s;
        (*skelAnime).morphTable =
            ((morphTable as
                  u32_0).wrapping_add(0xf as libc::c_int as libc::c_uint) &
                 !(0xf as libc::c_int) as libc::c_uint) as *mut Vec3s
    }
    if (*skelAnime).jointTable.is_null() || (*skelAnime).morphTable.is_null()
       {
        osSyncPrintf(b"\x1b[31m\x00" as *const u8 as *const libc::c_char);
        // "Memory allocation error"
        osSyncPrintf(b"Skeleton_Info_Rom_SV_ct \xe3\x83\xa1\xe3\x83\xa2\xe3\x83\xaa\xe3\x82\xa2\xe3\x83\xad\xe3\x82\xb1\xe3\x83\xbc\xe3\x82\xb7\xe3\x83\xa7\xe3\x83\xb3\xe3\x82\xa8\xe3\x83\xa9\xe3\x83\xbc\n\x00"
                         as *const u8 as *const libc::c_char);
        osSyncPrintf(b"\x1b[m\x00" as *const u8 as *const libc::c_char);
    }
    LinkAnimation_Change(globalCtx, skelAnime, animation, 1.0f32, 0.0f32,
                         0.0f32, ANIMMODE_LOOP as libc::c_int as u8_0,
                         0.0f32);
}
/* *
 * Sets the update function of a SkelAnime that uses Link animations based on its mode
 */
#[no_mangle]
pub unsafe extern "C" fn LinkAnimation_SetUpdateFunction(mut skelAnime:
                                                             *mut SkelAnime) {
    if (*skelAnime).mode as libc::c_int <= ANIMMODE_LOOP_INTERP as libc::c_int
       {
        (*skelAnime).update =
            ::std::mem::transmute::<Option<unsafe extern "C" fn(_:
                                                                    *mut GlobalContext,
                                                                _:
                                                                    *mut SkelAnime)
                                               -> s32>,
                                    Option<unsafe extern "C" fn()
                                               ->
                                                   s32>>(Some(LinkAnimation_Loop
                                                                  as
                                                                  unsafe extern "C" fn(_:
                                                                                           *mut GlobalContext,
                                                                                       _:
                                                                                           *mut SkelAnime)
                                                                      -> s32))
    } else {
        (*skelAnime).update =
            ::std::mem::transmute::<Option<unsafe extern "C" fn(_:
                                                                    *mut GlobalContext,
                                                                _:
                                                                    *mut SkelAnime)
                                               -> s32>,
                                    Option<unsafe extern "C" fn()
                                               ->
                                                   s32>>(Some(LinkAnimation_Once
                                                                  as
                                                                  unsafe extern "C" fn(_:
                                                                                           *mut GlobalContext,
                                                                                       _:
                                                                                           *mut SkelAnime)
                                                                      -> s32))
    }
    (*skelAnime).morphWeight = 0.0f32;
}
/* *
 * Advances the current Link animation and updates all frame tables. If the animation plays once, returns true when it
 * finishes.
 */
#[no_mangle]
pub unsafe extern "C" fn LinkAnimation_Update(mut globalCtx:
                                                  *mut GlobalContext,
                                              mut skelAnime: *mut SkelAnime)
 -> s32 {
    return ::std::mem::transmute::<_,
                                   fn(_: _, _: _)
                                       ->
                                           s32>((*skelAnime).update.expect("non-null function pointer"))(globalCtx,
                                                                                                         skelAnime);
}
/* *
 * Requests an interpolation between the pose in jointTable to the one in morphTable, advancing the morph but not the
 * animation frame
 */
#[no_mangle]
pub unsafe extern "C" fn LinkAnimation_Morph(mut globalCtx:
                                                 *mut GlobalContext,
                                             mut skelAnime: *mut SkelAnime)
 -> s32 {
    let mut prevMorphWeight: f32_0 = (*skelAnime).morphWeight;
    let mut updateRate: f32_0 =
        (*gGameInfo).data[(1 as libc::c_int * 6 as libc::c_int *
                               16 as libc::c_int + 30 as libc::c_int) as
                              usize] as libc::c_int as libc::c_float * 0.5f32;
    (*skelAnime).morphWeight -= (*skelAnime).morphRate * updateRate;
    if (*skelAnime).morphWeight <= 0.0f32 {
        LinkAnimation_SetUpdateFunction(skelAnime);
    }
    AnimationContext_SetInterp(globalCtx, (*skelAnime).limbCount as s32,
                               (*skelAnime).jointTable,
                               (*skelAnime).morphTable,
                               1.0f32 -
                                   (*skelAnime).morphWeight /
                                       prevMorphWeight);
    return 0 as libc::c_int;
}
/* *
 * Requests a load of the next frame of a Link animation, advances the morph, and requests an interpolation between
 * jointTable and morphTable
 */
#[no_mangle]
pub unsafe extern "C" fn LinkAnimation_AnimateFrame(mut globalCtx:
                                                        *mut GlobalContext,
                                                    mut skelAnime:
                                                        *mut SkelAnime) {
    AnimationContext_SetLoadFrame(globalCtx,
                                  (*skelAnime).animation as
                                      *mut LinkAnimationHeader,
                                  (*skelAnime).curFrame as s32,
                                  (*skelAnime).limbCount as s32,
                                  (*skelAnime).jointTable);
    if (*skelAnime).morphWeight != 0 as libc::c_int as libc::c_float {
        let mut updateRate: f32_0 =
            (*gGameInfo).data[(1 as libc::c_int * 6 as libc::c_int *
                                   16 as libc::c_int + 30 as libc::c_int) as
                                  usize] as libc::c_int as libc::c_float *
                0.5f32;
        (*skelAnime).morphWeight -= (*skelAnime).morphRate * updateRate;
        if (*skelAnime).morphWeight <= 0.0f32 {
            (*skelAnime).morphWeight = 0.0f32
        } else {
            AnimationContext_SetInterp(globalCtx,
                                       (*skelAnime).limbCount as s32,
                                       (*skelAnime).jointTable,
                                       (*skelAnime).morphTable,
                                       (*skelAnime).morphWeight);
        }
    };
}
/* *
 * Advances a Link animation that loops over its full length
 */
#[no_mangle]
pub unsafe extern "C" fn LinkAnimation_Loop(mut globalCtx: *mut GlobalContext,
                                            mut skelAnime: *mut SkelAnime)
 -> s32 {
    let mut updateRate: f32_0 =
        (*gGameInfo).data[(1 as libc::c_int * 6 as libc::c_int *
                               16 as libc::c_int + 30 as libc::c_int) as
                              usize] as libc::c_int as libc::c_float * 0.5f32;
    (*skelAnime).curFrame += (*skelAnime).playSpeed * updateRate;
    if (*skelAnime).curFrame < 0.0f32 {
        (*skelAnime).curFrame += (*skelAnime).animLength
    } else if (*skelAnime).animLength <= (*skelAnime).curFrame {
        (*skelAnime).curFrame -= (*skelAnime).animLength
    }
    LinkAnimation_AnimateFrame(globalCtx, skelAnime);
    return 0 as libc::c_int;
}
/* *
 * Advances a Link animation that stops at endFrame and returns true when it is reached.
 */
#[no_mangle]
pub unsafe extern "C" fn LinkAnimation_Once(mut globalCtx: *mut GlobalContext,
                                            mut skelAnime: *mut SkelAnime)
 -> s32 {
    let mut updateRate: f32_0 =
        (*gGameInfo).data[(1 as libc::c_int * 6 as libc::c_int *
                               16 as libc::c_int + 30 as libc::c_int) as
                              usize] as libc::c_int as libc::c_float * 0.5f32;
    if (*skelAnime).curFrame == (*skelAnime).endFrame {
        LinkAnimation_AnimateFrame(globalCtx, skelAnime);
        return 1 as libc::c_int
    }
    (*skelAnime).curFrame += (*skelAnime).playSpeed * updateRate;
    if ((*skelAnime).curFrame - (*skelAnime).endFrame) *
           (*skelAnime).playSpeed > 0.0f32 {
        (*skelAnime).curFrame = (*skelAnime).endFrame
    } else if (*skelAnime).curFrame < 0.0f32 {
        (*skelAnime).curFrame += (*skelAnime).animLength
    } else if (*skelAnime).animLength <= (*skelAnime).curFrame {
        (*skelAnime).curFrame -= (*skelAnime).animLength
    }
    LinkAnimation_AnimateFrame(globalCtx, skelAnime);
    return 0 as libc::c_int;
}
/* *
 * Sets a new morph and resets the morph weight for the current animation.
 */
#[no_mangle]
pub unsafe extern "C" fn Animation_SetMorph(mut globalCtx: *mut GlobalContext,
                                            mut skelAnime: *mut SkelAnime,
                                            mut morphFrames: f32_0) {
    (*skelAnime).morphWeight = 1.0f32;
    (*skelAnime).morphRate = 1.0f32 / morphFrames;
}
/* *
 * General way to set a new Link animation, allowing choice of playback speed, start frame, end frame, play mode, and
 * number of transition frames. Positive morph frames morph from the current pose to the start pose of the new
 * animation, then start the new animation. Negative morph frames start the new animation immediately, modified by the
 * pose immediately before the animation change.
 */
#[no_mangle]
pub unsafe extern "C" fn LinkAnimation_Change(mut globalCtx:
                                                  *mut GlobalContext,
                                              mut skelAnime: *mut SkelAnime,
                                              mut animation:
                                                  *mut LinkAnimationHeader,
                                              mut playSpeed: f32_0,
                                              mut startFrame: f32_0,
                                              mut endFrame: f32_0,
                                              mut mode: u8_0,
                                              mut morphFrames: f32_0) {
    (*skelAnime).mode = mode;
    if morphFrames != 0.0f32 &&
           (animation != (*skelAnime).animation as *mut LinkAnimationHeader ||
                startFrame != (*skelAnime).curFrame) {
        if morphFrames < 0 as libc::c_int as libc::c_float {
            LinkAnimation_SetUpdateFunction(skelAnime);
            SkelAnime_CopyFrameTable(skelAnime, (*skelAnime).morphTable,
                                     (*skelAnime).jointTable);
            morphFrames = -morphFrames
        } else {
            (*skelAnime).update =
                ::std::mem::transmute::<Option<unsafe extern "C" fn(_:
                                                                        *mut GlobalContext,
                                                                    _:
                                                                        *mut SkelAnime)
                                                   -> s32>,
                                        Option<unsafe extern "C" fn()
                                                   ->
                                                       s32>>(Some(LinkAnimation_Morph
                                                                      as
                                                                      unsafe extern "C" fn(_:
                                                                                               *mut GlobalContext,
                                                                                           _:
                                                                                               *mut SkelAnime)
                                                                          ->
                                                                              s32));
            AnimationContext_SetLoadFrame(globalCtx, animation,
                                          startFrame as s32,
                                          (*skelAnime).limbCount as s32,
                                          (*skelAnime).morphTable);
        }
        (*skelAnime).morphWeight = 1.0f32;
        (*skelAnime).morphRate = 1.0f32 / morphFrames
    } else {
        LinkAnimation_SetUpdateFunction(skelAnime);
        AnimationContext_SetLoadFrame(globalCtx, animation, startFrame as s32,
                                      (*skelAnime).limbCount as s32,
                                      (*skelAnime).jointTable);
        (*skelAnime).morphWeight = 0.0f32
    }
    (*skelAnime).animation = animation as *mut libc::c_void;
    (*skelAnime).curFrame = 0.0f32;
    (*skelAnime).startFrame = startFrame;
    (*skelAnime).curFrame = startFrame;
    (*skelAnime).endFrame = endFrame;
    (*skelAnime).animLength =
        Animation_GetLength(animation as *mut libc::c_void) as f32_0;
    (*skelAnime).playSpeed = playSpeed;
}
/* *
 * Immediately changes to a Link animation that plays once at the default speed.
 */
#[no_mangle]
pub unsafe extern "C" fn LinkAnimation_PlayOnce(mut globalCtx:
                                                    *mut GlobalContext,
                                                mut skelAnime: *mut SkelAnime,
                                                mut animation:
                                                    *mut LinkAnimationHeader) {
    LinkAnimation_Change(globalCtx, skelAnime, animation, 1.0f32, 0.0f32,
                         Animation_GetLastFrame(animation as
                                                    *mut libc::c_void) as
                             f32_0, ANIMMODE_ONCE as libc::c_int as u8_0,
                         0.0f32);
}
/* *
 * Immediately changes to a Link animation that plays once at the specified speed.
 */
#[no_mangle]
pub unsafe extern "C" fn LinkAnimation_PlayOnceSetSpeed(mut globalCtx:
                                                            *mut GlobalContext,
                                                        mut skelAnime:
                                                            *mut SkelAnime,
                                                        mut animation:
                                                            *mut LinkAnimationHeader,
                                                        mut playSpeed:
                                                            f32_0) {
    LinkAnimation_Change(globalCtx, skelAnime, animation, playSpeed, 0.0f32,
                         Animation_GetLastFrame(animation as
                                                    *mut libc::c_void) as
                             f32_0, ANIMMODE_ONCE as libc::c_int as u8_0,
                         0.0f32);
}
/* *
 * Immediately changes to a Link animation that loops at the default speed.
 */
#[no_mangle]
pub unsafe extern "C" fn LinkAnimation_PlayLoop(mut globalCtx:
                                                    *mut GlobalContext,
                                                mut skelAnime: *mut SkelAnime,
                                                mut animation:
                                                    *mut LinkAnimationHeader) {
    LinkAnimation_Change(globalCtx, skelAnime, animation, 1.0f32, 0.0f32,
                         Animation_GetLastFrame(animation as
                                                    *mut libc::c_void) as
                             f32_0, ANIMMODE_LOOP as libc::c_int as u8_0,
                         0.0f32);
}
/* *
 * Immediately changes to a Link animation that loops at the specified speed.
 */
#[no_mangle]
pub unsafe extern "C" fn LinkAnimation_PlayLoopSetSpeed(mut globalCtx:
                                                            *mut GlobalContext,
                                                        mut skelAnime:
                                                            *mut SkelAnime,
                                                        mut animation:
                                                            *mut LinkAnimationHeader,
                                                        mut playSpeed:
                                                            f32_0) {
    LinkAnimation_Change(globalCtx, skelAnime, animation, playSpeed, 0.0f32,
                         Animation_GetLastFrame(animation as
                                                    *mut libc::c_void) as
                             f32_0, ANIMMODE_LOOP as libc::c_int as u8_0,
                         0.0f32);
}
/* *
 * Requests copying jointTable to morphTable
 */
#[no_mangle]
pub unsafe extern "C" fn LinkAnimation_CopyJointToMorph(mut globalCtx:
                                                            *mut GlobalContext,
                                                        mut skelAnime:
                                                            *mut SkelAnime) {
    AnimationContext_SetCopyAll(globalCtx, (*skelAnime).limbCount as s32,
                                (*skelAnime).morphTable,
                                (*skelAnime).jointTable);
}
/* *
 * Requests copying morphTable to jointTable
 * unused
 */
#[no_mangle]
pub unsafe extern "C" fn LinkAnimation_CopyMorphToJoint(mut globalCtx:
                                                            *mut GlobalContext,
                                                        mut skelAnime:
                                                            *mut SkelAnime) {
    AnimationContext_SetCopyAll(globalCtx, (*skelAnime).limbCount as s32,
                                (*skelAnime).jointTable,
                                (*skelAnime).morphTable);
}
/* *
 * Requests loading frame data from the Link animation into morphTable
 */
#[no_mangle]
pub unsafe extern "C" fn LinkAnimation_LoadToMorph(mut globalCtx:
                                                       *mut GlobalContext,
                                                   mut skelAnime:
                                                       *mut SkelAnime,
                                                   mut animation:
                                                       *mut LinkAnimationHeader,
                                                   mut frame: f32_0) {
    AnimationContext_SetLoadFrame(globalCtx, animation, frame as s32,
                                  (*skelAnime).limbCount as s32,
                                  (*skelAnime).morphTable);
}
/* *
 * Requests loading frame data from the Link animation into jointTable
 */
#[no_mangle]
pub unsafe extern "C" fn LinkAnimation_LoadToJoint(mut globalCtx:
                                                       *mut GlobalContext,
                                                   mut skelAnime:
                                                       *mut SkelAnime,
                                                   mut animation:
                                                       *mut LinkAnimationHeader,
                                                   mut frame: f32_0) {
    AnimationContext_SetLoadFrame(globalCtx, animation, frame as s32,
                                  (*skelAnime).limbCount as s32,
                                  (*skelAnime).jointTable);
}
/* *
 * Requests interpolating between jointTable and morphTable, placing the result in jointTable
 */
#[no_mangle]
pub unsafe extern "C" fn LinkAnimation_InterpJointMorph(mut globalCtx:
                                                            *mut GlobalContext,
                                                        mut skelAnime:
                                                            *mut SkelAnime,
                                                        mut weight: f32_0) {
    AnimationContext_SetInterp(globalCtx, (*skelAnime).limbCount as s32,
                               (*skelAnime).jointTable,
                               (*skelAnime).morphTable, weight);
}
/* *
 * Requests loading frame data from the Link animations and blending them, placing the result in jointTable
 */
#[no_mangle]
pub unsafe extern "C" fn LinkAnimation_BlendToJoint(mut globalCtx:
                                                        *mut GlobalContext,
                                                    mut skelAnime:
                                                        *mut SkelAnime,
                                                    mut animation1:
                                                        *mut LinkAnimationHeader,
                                                    mut frame1: f32_0,
                                                    mut animation2:
                                                        *mut LinkAnimationHeader,
                                                    mut frame2: f32_0,
                                                    mut blendWeight: f32_0,
                                                    mut blendTable:
                                                        *mut Vec3s) {
    let mut alignedBlendTable: *mut Vec3s = 0 as *mut Vec3s;
    AnimationContext_SetLoadFrame(globalCtx, animation1, frame1 as s32,
                                  (*skelAnime).limbCount as s32,
                                  (*skelAnime).jointTable);
    alignedBlendTable =
        ((blendTable as
              u32_0).wrapping_add(0xf as libc::c_int as libc::c_uint) &
             !(0xf as libc::c_int) as libc::c_uint) as *mut Vec3s;
    AnimationContext_SetLoadFrame(globalCtx, animation2, frame2 as s32,
                                  (*skelAnime).limbCount as s32,
                                  alignedBlendTable);
    AnimationContext_SetInterp(globalCtx, (*skelAnime).limbCount as s32,
                               (*skelAnime).jointTable, alignedBlendTable,
                               blendWeight);
}
/* *
 * Requests loading frame data from the Link animations and blending them, placing the result in morphTable
 */
#[no_mangle]
pub unsafe extern "C" fn LinkAnimation_BlendToMorph(mut globalCtx:
                                                        *mut GlobalContext,
                                                    mut skelAnime:
                                                        *mut SkelAnime,
                                                    mut animation1:
                                                        *mut LinkAnimationHeader,
                                                    mut frame1: f32_0,
                                                    mut animation2:
                                                        *mut LinkAnimationHeader,
                                                    mut frame2: f32_0,
                                                    mut blendWeight: f32_0,
                                                    mut blendTable:
                                                        *mut Vec3s) {
    let mut alignedBlendTable: *mut Vec3s = 0 as *mut Vec3s;
    AnimationContext_SetLoadFrame(globalCtx, animation1, frame1 as s32,
                                  (*skelAnime).limbCount as s32,
                                  (*skelAnime).morphTable);
    alignedBlendTable =
        ((blendTable as
              u32_0).wrapping_add(0xf as libc::c_int as libc::c_uint) &
             !(0xf as libc::c_int) as libc::c_uint) as *mut Vec3s;
    AnimationContext_SetLoadFrame(globalCtx, animation2, frame2 as s32,
                                  (*skelAnime).limbCount as s32,
                                  alignedBlendTable);
    AnimationContext_SetInterp(globalCtx, (*skelAnime).limbCount as s32,
                               (*skelAnime).morphTable, alignedBlendTable,
                               blendWeight);
}
/* *
 * Changes a looping animation to one that stops at the end. Unused
 */
#[no_mangle]
pub unsafe extern "C" fn LinkAnimation_EndLoop(mut skelAnime:
                                                   *mut SkelAnime) {
    (*skelAnime).mode = ANIMMODE_ONCE as libc::c_int as u8_0;
    LinkAnimation_SetUpdateFunction(skelAnime);
}
/* *
 * Checks if the current frame is after frame and the previous frame was before it.
 */
#[no_mangle]
pub unsafe extern "C" fn Animation_OnFrameImpl(mut skelAnime: *mut SkelAnime,
                                               mut frame: f32_0,
                                               mut updateRate: f32_0) -> s32 {
    let mut updateSpeed: f32_0 = (*skelAnime).playSpeed * updateRate;
    let mut prevFrame: f32_0 = (*skelAnime).curFrame - updateSpeed;
    let mut curFrameDiff: f32_0 = 0.;
    let mut prevFrameDiff: f32_0 = 0.;
    if prevFrame < 0.0f32 {
        prevFrame += (*skelAnime).animLength
    } else if prevFrame >= (*skelAnime).animLength {
        prevFrame -= (*skelAnime).animLength
    }
    if frame == 0.0f32 && updateSpeed > 0.0f32 {
        frame = (*skelAnime).animLength
    }
    curFrameDiff = prevFrame + updateSpeed - frame;
    prevFrameDiff = curFrameDiff - updateSpeed;
    if curFrameDiff * updateSpeed >= 0.0f32 &&
           prevFrameDiff * updateSpeed < 0.0f32 {
        return 1 as libc::c_int
    }
    return 0 as libc::c_int;
}
/* *
 * Checks if the current Link animation has reached the specified frame
 */
#[no_mangle]
pub unsafe extern "C" fn LinkAnimation_OnFrame(mut skelAnime: *mut SkelAnime,
                                               mut frame: f32_0) -> s32 {
    let mut updateRate: f32_0 =
        (*gGameInfo).data[(1 as libc::c_int * 6 as libc::c_int *
                               16 as libc::c_int + 30 as libc::c_int) as
                              usize] as libc::c_int as libc::c_float * 0.5f32;
    return Animation_OnFrameImpl(skelAnime, frame, updateRate);
}
/* *
 * Initializes a normal skeleton to a looping animation, dynamically allocating the frame tables if not provided.
 */
#[no_mangle]
pub unsafe extern "C" fn SkelAnime_Init(mut globalCtx: *mut GlobalContext,
                                        mut skelAnime: *mut SkelAnime,
                                        mut skeletonHeaderSeg:
                                            *mut SkeletonHeader,
                                        mut animation: *mut AnimationHeader,
                                        mut jointTable: *mut Vec3s,
                                        mut morphTable: *mut Vec3s,
                                        mut limbCount: s32) -> s32 {
    let mut skeletonHeader: *mut SkeletonHeader =
        gSegments[((skeletonHeaderSeg as u32_0) << 4 as libc::c_int >>
                       28 as libc::c_int) as
                      usize].wrapping_add(skeletonHeaderSeg as u32_0 &
                                              0xffffff as libc::c_int as
                                                  libc::c_uint).wrapping_add(0x80000000
                                                                                 as
                                                                                 libc::c_uint)
            as *mut libc::c_void as
            *mut SkeletonHeader; // "Memory allocation error"
    (*skelAnime).limbCount =
        ((*skeletonHeader).limbCount as libc::c_int + 1 as libc::c_int) as
            u8_0;
    (*skelAnime).skeleton =
        gSegments[(((*skeletonHeader).segment as u32_0) << 4 as libc::c_int >>
                       28 as libc::c_int) as
                      usize].wrapping_add((*skeletonHeader).segment as u32_0 &
                                              0xffffff as libc::c_int as
                                                  libc::c_uint).wrapping_add(0x80000000
                                                                                 as
                                                                                 libc::c_uint)
            as *mut libc::c_void as *mut *mut libc::c_void;
    if jointTable.is_null() {
        (*skelAnime).jointTable =
            ZeldaArena_MallocDebug(((*skelAnime).limbCount as
                                        libc::c_uint).wrapping_mul(::std::mem::size_of::<Vec3s>()
                                                                       as
                                                                       libc::c_ulong),
                                   b"../z_skelanime.c\x00" as *const u8 as
                                       *const libc::c_char,
                                   2968 as libc::c_int) as *mut Vec3s;
        (*skelAnime).morphTable =
            ZeldaArena_MallocDebug(((*skelAnime).limbCount as
                                        libc::c_uint).wrapping_mul(::std::mem::size_of::<Vec3s>()
                                                                       as
                                                                       libc::c_ulong),
                                   b"../z_skelanime.c\x00" as *const u8 as
                                       *const libc::c_char,
                                   2969 as libc::c_int) as *mut Vec3s
    } else {
        if limbCount == (*skelAnime).limbCount as libc::c_int {
        } else {
            __assert(b"joint_buff_num == this->joint_num\x00" as *const u8 as
                         *const libc::c_char,
                     b"../z_skelanime.c\x00" as *const u8 as
                         *const libc::c_char, 2973 as libc::c_int);
        };
        (*skelAnime).jointTable = jointTable;
        (*skelAnime).morphTable = morphTable
    }
    if (*skelAnime).jointTable.is_null() || (*skelAnime).morphTable.is_null()
       {
        osSyncPrintf(b"\x1b[31m\x00" as *const u8 as *const libc::c_char);
        osSyncPrintf(b"Skeleton_Info2_ct \xe3\x83\xa1\xe3\x83\xa2\xe3\x83\xaa\xe3\x82\xa2\xe3\x83\xad\xe3\x82\xb1\xe3\x83\xbc\xe3\x82\xb7\xe3\x83\xa7\xe3\x83\xb3\xe3\x82\xa8\xe3\x83\xa9\xe3\x83\xbc\n\x00"
                         as *const u8 as *const libc::c_char);
        osSyncPrintf(b"\x1b[m\x00" as *const u8 as *const libc::c_char);
    }
    if !animation.is_null() { Animation_PlayLoop(skelAnime, animation); }
    panic!("Reached end of non-void function without returning");
}
/* *
 * Initializes a flex skeleton to a looping animation, dynamically allocating the frame tables if not given.
 */
#[no_mangle]
pub unsafe extern "C" fn SkelAnime_InitFlex(mut globalCtx: *mut GlobalContext,
                                            mut skelAnime: *mut SkelAnime,
                                            mut skeletonHeaderSeg:
                                                *mut FlexSkeletonHeader,
                                            mut animation:
                                                *mut AnimationHeader,
                                            mut jointTable: *mut Vec3s,
                                            mut morphTable: *mut Vec3s,
                                            mut limbCount: s32) -> s32 {
    let mut skeletonHeader: *mut FlexSkeletonHeader =
        gSegments[((skeletonHeaderSeg as u32_0) << 4 as libc::c_int >>
                       28 as libc::c_int) as
                      usize].wrapping_add(skeletonHeaderSeg as u32_0 &
                                              0xffffff as libc::c_int as
                                                  libc::c_uint).wrapping_add(0x80000000
                                                                                 as
                                                                                 libc::c_uint)
            as *mut libc::c_void as *mut FlexSkeletonHeader;
    (*skelAnime).limbCount =
        ((*skeletonHeader).sh.limbCount as libc::c_int + 1 as libc::c_int) as
            u8_0;
    (*skelAnime).dListCount = (*skeletonHeader).dListCount;
    (*skelAnime).skeleton =
        gSegments[(((*skeletonHeader).sh.segment as u32_0) << 4 as libc::c_int
                       >> 28 as libc::c_int) as
                      usize].wrapping_add((*skeletonHeader).sh.segment as
                                              u32_0 &
                                              0xffffff as libc::c_int as
                                                  libc::c_uint).wrapping_add(0x80000000
                                                                                 as
                                                                                 libc::c_uint)
            as *mut libc::c_void as *mut *mut libc::c_void;
    if jointTable.is_null() {
        (*skelAnime).jointTable =
            ZeldaArena_MallocDebug(((*skelAnime).limbCount as
                                        libc::c_uint).wrapping_mul(::std::mem::size_of::<Vec3s>()
                                                                       as
                                                                       libc::c_ulong),
                                   b"../z_skelanime.c\x00" as *const u8 as
                                       *const libc::c_char,
                                   3047 as libc::c_int) as *mut Vec3s;
        (*skelAnime).morphTable =
            ZeldaArena_MallocDebug(((*skelAnime).limbCount as
                                        libc::c_uint).wrapping_mul(::std::mem::size_of::<Vec3s>()
                                                                       as
                                                                       libc::c_ulong),
                                   b"../z_skelanime.c\x00" as *const u8 as
                                       *const libc::c_char,
                                   3048 as libc::c_int) as *mut Vec3s
    } else {
        if limbCount == (*skelAnime).limbCount as libc::c_int {
        } else {
            __assert(b"joint_buff_num == this->joint_num\x00" as *const u8 as
                         *const libc::c_char,
                     b"../z_skelanime.c\x00" as *const u8 as
                         *const libc::c_char, 3052 as libc::c_int);
        };
        (*skelAnime).jointTable = jointTable;
        (*skelAnime).morphTable = morphTable
    }
    if (*skelAnime).jointTable.is_null() || (*skelAnime).morphTable.is_null()
       {
        osSyncPrintf(b"\x1b[31m\x00" as *const u8 as *const libc::c_char);
        // "Memory allocation error"
        osSyncPrintf(b"Skeleton_Info_Rom_SV_ct \xe3\x83\xa1\xe3\x83\xa2\xe3\x83\xaa\xe3\x82\xa2\xe3\x83\xad\xe3\x82\xb1\xe3\x83\xbc\xe3\x82\xb7\xe3\x83\xa7\xe3\x83\xb3\xe3\x82\xa8\xe3\x83\xa9\xe3\x83\xbc\n\x00"
                         as *const u8 as *const libc::c_char);
        osSyncPrintf(b"\x1b[m\x00" as *const u8 as *const libc::c_char);
    }
    if !animation.is_null() { Animation_PlayLoop(skelAnime, animation); }
    panic!("Reached end of non-void function without returning");
}
/* *
 * Initializes a skeleton with SkinLimbs to a looping animation, dynamically allocating the frame tables.
 */
#[no_mangle]
pub unsafe extern "C" fn SkelAnime_InitSkin(mut globalCtx: *mut GlobalContext,
                                            mut skelAnime: *mut SkelAnime,
                                            mut skeletonHeaderSeg:
                                                *mut SkeletonHeader,
                                            mut animation:
                                                *mut AnimationHeader) -> s32 {
    let mut skeletonHeader: *mut SkeletonHeader =
        gSegments[((skeletonHeaderSeg as u32_0) << 4 as libc::c_int >>
                       28 as libc::c_int) as
                      usize].wrapping_add(skeletonHeaderSeg as u32_0 &
                                              0xffffff as libc::c_int as
                                                  libc::c_uint).wrapping_add(0x80000000
                                                                                 as
                                                                                 libc::c_uint)
            as *mut libc::c_void as *mut SkeletonHeader;
    (*skelAnime).limbCount =
        ((*skeletonHeader).limbCount as libc::c_int + 1 as libc::c_int) as
            u8_0;
    (*skelAnime).skeleton =
        gSegments[(((*skeletonHeader).segment as u32_0) << 4 as libc::c_int >>
                       28 as libc::c_int) as
                      usize].wrapping_add((*skeletonHeader).segment as u32_0 &
                                              0xffffff as libc::c_int as
                                                  libc::c_uint).wrapping_add(0x80000000
                                                                                 as
                                                                                 libc::c_uint)
            as *mut libc::c_void as *mut *mut libc::c_void;
    (*skelAnime).jointTable =
        ZeldaArena_MallocDebug(((*skelAnime).limbCount as
                                    libc::c_uint).wrapping_mul(::std::mem::size_of::<Vec3s>()
                                                                   as
                                                                   libc::c_ulong),
                               b"../z_skelanime.c\x00" as *const u8 as
                                   *const libc::c_char, 3120 as libc::c_int)
            as *mut Vec3s;
    (*skelAnime).morphTable =
        ZeldaArena_MallocDebug(((*skelAnime).limbCount as
                                    libc::c_uint).wrapping_mul(::std::mem::size_of::<Vec3s>()
                                                                   as
                                                                   libc::c_ulong),
                               b"../z_skelanime.c\x00" as *const u8 as
                                   *const libc::c_char, 3121 as libc::c_int)
            as *mut Vec3s;
    if (*skelAnime).jointTable.is_null() || (*skelAnime).morphTable.is_null()
       {
        osSyncPrintf(b"\x1b[31m\x00" as *const u8 as *const libc::c_char);
        // "Memory allocation error"
        osSyncPrintf(b"Skeleton_Info2_skin2_ct \xe3\x83\xa1\xe3\x83\xa2\xe3\x83\xaa\xe3\x82\xa2\xe3\x83\xad\xe3\x82\xb1\xe3\x83\xbc\xe3\x82\xb7\xe3\x83\xa7\xe3\x83\xb3\xe3\x82\xa8\xe3\x83\xa9\xe3\x83\xbc\n\x00"
                         as *const u8 as *const libc::c_char);
        osSyncPrintf(b"\x1b[m\x00" as *const u8 as *const libc::c_char);
    }
    if !animation.is_null() { Animation_PlayLoop(skelAnime, animation); }
    panic!("Reached end of non-void function without returning");
}
/* *
 * Sets the SkelAnime's update function based on its current mode.
 */
#[no_mangle]
pub unsafe extern "C" fn SkelAnime_SetUpdate(mut skelAnime: *mut SkelAnime) {
    if (*skelAnime).mode as libc::c_int <= ANIMMODE_LOOP_INTERP as libc::c_int
       {
        (*skelAnime).update =
            ::std::mem::transmute::<Option<unsafe extern "C" fn(_:
                                                                    *mut SkelAnime)
                                               -> s32>,
                                    Option<unsafe extern "C" fn()
                                               ->
                                                   s32>>(Some(SkelAnime_LoopFull
                                                                  as
                                                                  unsafe extern "C" fn(_:
                                                                                           *mut SkelAnime)
                                                                      -> s32))
    } else if (*skelAnime).mode as libc::c_int <=
                  ANIMMODE_ONCE_INTERP as libc::c_int {
        (*skelAnime).update =
            ::std::mem::transmute::<Option<unsafe extern "C" fn(_:
                                                                    *mut SkelAnime)
                                               -> s32>,
                                    Option<unsafe extern "C" fn()
                                               ->
                                                   s32>>(Some(SkelAnime_Once
                                                                  as
                                                                  unsafe extern "C" fn(_:
                                                                                           *mut SkelAnime)
                                                                      -> s32))
    } else {
        (*skelAnime).update =
            ::std::mem::transmute::<Option<unsafe extern "C" fn(_:
                                                                    *mut SkelAnime)
                                               -> s32>,
                                    Option<unsafe extern "C" fn()
                                               ->
                                                   s32>>(Some(SkelAnime_LoopPartial
                                                                  as
                                                                  unsafe extern "C" fn(_:
                                                                                           *mut SkelAnime)
                                                                      -> s32))
    };
}
/* *
 * Advances the current animation and updates all frame tables. If the animation plays once, returns true when it
 * finishes.
 */
#[no_mangle]
pub unsafe extern "C" fn SkelAnime_Update(mut skelAnime: *mut SkelAnime)
 -> s32 {
    return ::std::mem::transmute::<_,
                                   fn(_: _)
                                       ->
                                           s32>((*skelAnime).update.expect("non-null function pointer"))(skelAnime);
}
/* *
 * Morphs from the pose in jointTable to the one in morphTable, advancing the morph but not the animation frame
 */
#[no_mangle]
pub unsafe extern "C" fn SkelAnime_Morph(mut skelAnime: *mut SkelAnime)
 -> s32 {
    let mut prevMorphWeight: f32_0 = (*skelAnime).morphWeight;
    let mut updateRate: f32_0 =
        (*gGameInfo).data[(1 as libc::c_int * 6 as libc::c_int *
                               16 as libc::c_int + 30 as libc::c_int) as
                              usize] as libc::c_int as libc::c_float *
            (1.0f32 / 3.0f32);
    (*skelAnime).morphWeight -= (*skelAnime).morphRate * updateRate;
    if (*skelAnime).morphWeight <= 0.0f32 {
        SkelAnime_SetUpdate(skelAnime);
        (*skelAnime).morphWeight = 0.0f32
    }
    SkelAnime_InterpFrameTable((*skelAnime).limbCount as s32,
                               (*skelAnime).jointTable,
                               (*skelAnime).jointTable,
                               (*skelAnime).morphTable,
                               1.0f32 -
                                   (*skelAnime).morphWeight /
                                       prevMorphWeight);
    return 0 as libc::c_int;
}
/* *
 * Performs a tapered morph from the pose in jointTable to the one in morphTable, advancing the morph but not the
 * animation frame
 */
#[no_mangle]
pub unsafe extern "C" fn SkelAnime_MorphTaper(mut skelAnime: *mut SkelAnime)
 -> s32 {
    let mut prevPhase: s16 =
        ((*skelAnime).morphWeight * 0x4000 as libc::c_int as libc::c_float) as
            s16;
    let mut curPhase: s16 = 0;
    let mut prevWeight: f32_0 = 0.;
    let mut curWeight: f32_0 = 0.;
    let mut updateRate: f32_0 =
        (*gGameInfo).data[(1 as libc::c_int * 6 as libc::c_int *
                               16 as libc::c_int + 30 as libc::c_int) as
                              usize] as libc::c_int as libc::c_float *
            (1.0f32 / 3.0f32);
    (*skelAnime).morphWeight -= (*skelAnime).morphRate * updateRate;
    if (*skelAnime).morphWeight <= 0.0f32 {
        SkelAnime_SetUpdate(skelAnime);
        (*skelAnime).morphWeight = 0.0f32
    }
    curPhase =
        ((*skelAnime).morphWeight * 0x4000 as libc::c_int as libc::c_float) as
            s16;
    if (*skelAnime).taper as libc::c_int <= ANIMTAPER_DECEL as libc::c_int {
        prevWeight = 1.0f32 - Math_CosS(prevPhase);
        curWeight = 1.0f32 - Math_CosS(curPhase)
    } else {
        prevWeight = Math_SinS(prevPhase);
        curWeight = Math_SinS(curPhase)
    }
    if curWeight != 0.0f32 {
        curWeight /= prevWeight
    } else { curWeight = 0.0f32 }
    SkelAnime_InterpFrameTable((*skelAnime).limbCount as s32,
                               (*skelAnime).jointTable,
                               (*skelAnime).jointTable,
                               (*skelAnime).morphTable, 1.0f32 - curWeight);
    return 0 as libc::c_int;
}
/* *
 * Gets frame data for the current frame as modified by morphTable and advances the morph
 */
#[no_mangle]
pub unsafe extern "C" fn SkelAnime_AnimateFrame(mut skelAnime:
                                                    *mut SkelAnime) {
    let mut nextjointTable: [Vec3s; 100] = [Vec3s{x: 0, y: 0, z: 0,}; 100];
    SkelAnime_GetFrameData((*skelAnime).animation as *mut AnimationHeader,
                           (*skelAnime).curFrame as s32,
                           (*skelAnime).limbCount as s32,
                           (*skelAnime).jointTable);
    if (*skelAnime).mode as libc::c_int & 1 as libc::c_int != 0 {
        let mut frame: s32 = (*skelAnime).curFrame as s32;
        let mut partialFrame: f32_0 =
            (*skelAnime).curFrame - frame as libc::c_float;
        frame += 1;
        if frame >= (*skelAnime).animLength as s32 {
            frame = 0 as libc::c_int
        }
        SkelAnime_GetFrameData((*skelAnime).animation as *mut AnimationHeader,
                               frame, (*skelAnime).limbCount as s32,
                               nextjointTable.as_mut_ptr());
        SkelAnime_InterpFrameTable((*skelAnime).limbCount as s32,
                                   (*skelAnime).jointTable,
                                   (*skelAnime).jointTable,
                                   nextjointTable.as_mut_ptr(), partialFrame);
    }
    if (*skelAnime).morphWeight != 0 as libc::c_int as libc::c_float {
        let mut updateRate: f32_0 =
            (*gGameInfo).data[(1 as libc::c_int * 6 as libc::c_int *
                                   16 as libc::c_int + 30 as libc::c_int) as
                                  usize] as libc::c_int as libc::c_float *
                (1.0f32 / 3.0f32);
        (*skelAnime).morphWeight -= (*skelAnime).morphRate * updateRate;
        if (*skelAnime).morphWeight <= 0.0f32 {
            (*skelAnime).morphWeight = 0.0f32
        } else {
            SkelAnime_InterpFrameTable((*skelAnime).limbCount as s32,
                                       (*skelAnime).jointTable,
                                       (*skelAnime).jointTable,
                                       (*skelAnime).morphTable,
                                       (*skelAnime).morphWeight);
        }
    };
}
/* *
 * Advances an animation that loops over its full length and updates the frame tables
 */
#[no_mangle]
pub unsafe extern "C" fn SkelAnime_LoopFull(mut skelAnime: *mut SkelAnime)
 -> s32 {
    let mut updateRate: f32_0 =
        (*gGameInfo).data[(1 as libc::c_int * 6 as libc::c_int *
                               16 as libc::c_int + 30 as libc::c_int) as
                              usize] as libc::c_int as libc::c_float *
            (1.0f32 / 3.0f32);
    (*skelAnime).curFrame += (*skelAnime).playSpeed * updateRate;
    if (*skelAnime).curFrame < 0.0f32 {
        (*skelAnime).curFrame += (*skelAnime).animLength
    } else if (*skelAnime).animLength <= (*skelAnime).curFrame {
        (*skelAnime).curFrame -= (*skelAnime).animLength
    }
    SkelAnime_AnimateFrame(skelAnime);
    return 0 as libc::c_int;
}
/* *
 * Advances an animation that loops over part of its length and updates the frame tables
 */
#[no_mangle]
pub unsafe extern "C" fn SkelAnime_LoopPartial(mut skelAnime: *mut SkelAnime)
 -> s32 {
    let mut updateRate: f32_0 =
        (*gGameInfo).data[(1 as libc::c_int * 6 as libc::c_int *
                               16 as libc::c_int + 30 as libc::c_int) as
                              usize] as libc::c_int as libc::c_float *
            (1.0f32 / 3.0f32);
    (*skelAnime).curFrame += (*skelAnime).playSpeed * updateRate;
    if (*skelAnime).curFrame < (*skelAnime).startFrame {
        (*skelAnime).curFrame =
            (*skelAnime).curFrame - (*skelAnime).startFrame +
                (*skelAnime).endFrame
    } else if (*skelAnime).endFrame <= (*skelAnime).curFrame {
        (*skelAnime).curFrame =
            (*skelAnime).curFrame - (*skelAnime).endFrame +
                (*skelAnime).startFrame
    }
    SkelAnime_AnimateFrame(skelAnime);
    return 0 as libc::c_int;
}
/* *
 * Advances an animation that stops at endFrame and returns true when it is reached.
 */
#[no_mangle]
pub unsafe extern "C" fn SkelAnime_Once(mut skelAnime: *mut SkelAnime)
 -> s32 {
    let mut updateRate: f32_0 =
        (*gGameInfo).data[(1 as libc::c_int * 6 as libc::c_int *
                               16 as libc::c_int + 30 as libc::c_int) as
                              usize] as libc::c_int as libc::c_float *
            (1.0f32 / 3.0f32);
    if (*skelAnime).curFrame == (*skelAnime).endFrame {
        SkelAnime_GetFrameData((*skelAnime).animation as *mut AnimationHeader,
                               (*skelAnime).curFrame as s32,
                               (*skelAnime).limbCount as s32,
                               (*skelAnime).jointTable);
        SkelAnime_AnimateFrame(skelAnime);
        return 1 as libc::c_int
    }
    (*skelAnime).curFrame += (*skelAnime).playSpeed * updateRate;
    if ((*skelAnime).curFrame - (*skelAnime).endFrame) *
           (*skelAnime).playSpeed > 0.0f32 {
        (*skelAnime).curFrame = (*skelAnime).endFrame
    } else if (*skelAnime).curFrame < 0.0f32 {
        (*skelAnime).curFrame += (*skelAnime).animLength
    } else if (*skelAnime).animLength <= (*skelAnime).curFrame {
        (*skelAnime).curFrame -= (*skelAnime).animLength
    }
    SkelAnime_AnimateFrame(skelAnime);
    return 0 as libc::c_int;
}
/* *
 * Fully general way to set a new animation, allowing choice of playback speed, start frame, end frame, play mode,
 * number of transition frames, and tapering of the transition. Positive morph frames morph from the current pose to the
 * start pose of the new animation, then start the new animation. Negative morph frames start the new animation
 * immediately, modified by the pose immediately before the animation change.
 */
#[no_mangle]
pub unsafe extern "C" fn Animation_ChangeImpl(mut skelAnime: *mut SkelAnime,
                                              mut animation:
                                                  *mut AnimationHeader,
                                              mut playSpeed: f32_0,
                                              mut startFrame: f32_0,
                                              mut endFrame: f32_0,
                                              mut mode: u8_0,
                                              mut morphFrames: f32_0,
                                              mut taper: s8) {
    (*skelAnime).mode = mode;
    if morphFrames != 0.0f32 &&
           (animation != (*skelAnime).animation as *mut AnimationHeader ||
                startFrame != (*skelAnime).curFrame) {
        if morphFrames < 0 as libc::c_int as libc::c_float {
            SkelAnime_SetUpdate(skelAnime);
            SkelAnime_CopyFrameTable(skelAnime, (*skelAnime).morphTable,
                                     (*skelAnime).jointTable);
            morphFrames = -morphFrames
        } else {
            if taper as libc::c_int != ANIMTAPER_NONE as libc::c_int {
                (*skelAnime).update =
                    ::std::mem::transmute::<Option<unsafe extern "C" fn(_:
                                                                            *mut SkelAnime)
                                                       -> s32>,
                                            Option<unsafe extern "C" fn()
                                                       ->
                                                           s32>>(Some(SkelAnime_MorphTaper
                                                                          as
                                                                          unsafe extern "C" fn(_:
                                                                                                   *mut SkelAnime)
                                                                              ->
                                                                                  s32));
                (*skelAnime).taper = taper
            } else {
                (*skelAnime).update =
                    ::std::mem::transmute::<Option<unsafe extern "C" fn(_:
                                                                            *mut SkelAnime)
                                                       -> s32>,
                                            Option<unsafe extern "C" fn()
                                                       ->
                                                           s32>>(Some(SkelAnime_Morph
                                                                          as
                                                                          unsafe extern "C" fn(_:
                                                                                                   *mut SkelAnime)
                                                                              ->
                                                                                  s32))
            }
            SkelAnime_GetFrameData(animation, startFrame as s32,
                                   (*skelAnime).limbCount as s32,
                                   (*skelAnime).morphTable);
        }
        (*skelAnime).morphWeight = 1.0f32;
        (*skelAnime).morphRate = 1.0f32 / morphFrames
    } else {
        SkelAnime_SetUpdate(skelAnime);
        SkelAnime_GetFrameData(animation, startFrame as s32,
                               (*skelAnime).limbCount as s32,
                               (*skelAnime).jointTable);
        (*skelAnime).morphWeight = 0.0f32
    }
    (*skelAnime).animation = animation as *mut libc::c_void;
    (*skelAnime).startFrame = startFrame;
    (*skelAnime).endFrame = endFrame;
    (*skelAnime).animLength =
        Animation_GetLength(animation as *mut libc::c_void) as f32_0;
    if (*skelAnime).mode as libc::c_int >=
           ANIMMODE_LOOP_PARTIAL as libc::c_int {
        (*skelAnime).curFrame = 0.0f32
    } else {
        (*skelAnime).curFrame = startFrame;
        if (*skelAnime).mode as libc::c_int <=
               ANIMMODE_LOOP_INTERP as libc::c_int {
            (*skelAnime).endFrame = (*skelAnime).animLength - 1.0f32
        }
    }
    (*skelAnime).playSpeed = playSpeed;
}
/* *
 * General way to set a new animation, allowing choice of playback speed, start frame, end frame, play mode, and number
 * of transition frames. Positive morph frames morph from the current pose to the start pose of the new animation, then
 * start the new animation. Negative morph frames start the new animation immediately, modified by the pose immediately
 * before the animation change.
 */
#[no_mangle]
pub unsafe extern "C" fn Animation_Change(mut skelAnime: *mut SkelAnime,
                                          mut animation: *mut AnimationHeader,
                                          mut playSpeed: f32_0,
                                          mut startFrame: f32_0,
                                          mut endFrame: f32_0, mut mode: u8_0,
                                          mut morphFrames: f32_0) {
    Animation_ChangeImpl(skelAnime, animation, playSpeed, startFrame,
                         endFrame, mode, morphFrames,
                         ANIMTAPER_NONE as libc::c_int as s8);
}
/* *
 * Immediately changes to an animation that plays once at the default speed.
 */
#[no_mangle]
pub unsafe extern "C" fn Animation_PlayOnce(mut skelAnime: *mut SkelAnime,
                                            mut animation:
                                                *mut AnimationHeader) {
    Animation_Change(skelAnime, animation, 1.0f32, 0.0f32,
                     Animation_GetLastFrame(animation as *mut libc::c_void) as
                         f32_0, ANIMMODE_ONCE as libc::c_int as u8_0, 0.0f32);
}
/* *
 * Smoothly transitions to an animation that plays once at the default speed.
 * Positive morph frames morph from the current pose to the start pose of the new animation, then start the new
 * animation. Negative morph frames start the new animation immediately, modified by the pose immediately before the
 * animation change.
 */
#[no_mangle]
pub unsafe extern "C" fn Animation_MorphToPlayOnce(mut skelAnime:
                                                       *mut SkelAnime,
                                                   mut animation:
                                                       *mut AnimationHeader,
                                                   mut morphFrames: f32_0) {
    Animation_Change(skelAnime, animation, 1.0f32, 0.0f32,
                     Animation_GetLastFrame(animation as *mut libc::c_void) as
                         f32_0, ANIMMODE_ONCE as libc::c_int as u8_0,
                     morphFrames);
}
/* *
 * Immediately changes to an animation that plays once at the specified speed.
 */
#[no_mangle]
pub unsafe extern "C" fn Animation_PlayOnceSetSpeed(mut skelAnime:
                                                        *mut SkelAnime,
                                                    mut animation:
                                                        *mut AnimationHeader,
                                                    mut playSpeed: f32_0) {
    Animation_Change(skelAnime, animation, playSpeed, 0.0f32,
                     Animation_GetLastFrame(animation as *mut libc::c_void) as
                         f32_0, ANIMMODE_ONCE as libc::c_int as u8_0, 0.0f32);
}
/* *
 * Immediately changes to an animation that loops at the default.
 */
#[no_mangle]
pub unsafe extern "C" fn Animation_PlayLoop(mut skelAnime: *mut SkelAnime,
                                            mut animation:
                                                *mut AnimationHeader) {
    Animation_Change(skelAnime, animation, 1.0f32, 0.0f32,
                     Animation_GetLastFrame(animation as *mut libc::c_void) as
                         f32_0, ANIMMODE_LOOP as libc::c_int as u8_0, 0.0f32);
}
/* *
 * Smoothly transitions to a looping animation, specifying the number of frames for the transition.
 * Positive morph frames morph from the current pose to the start pose of the new animation, then start the new
 * animation. Negative morph frames start the new animation immediately, modified by the pose immediately before the
 * animation change.
 */
#[no_mangle]
pub unsafe extern "C" fn Animation_MorphToLoop(mut skelAnime: *mut SkelAnime,
                                               mut animation:
                                                   *mut AnimationHeader,
                                               mut morphFrames: f32_0) {
    Animation_Change(skelAnime, animation, 1.0f32, 0.0f32, 0.0f32,
                     ANIMMODE_LOOP as libc::c_int as u8_0, morphFrames);
}
/* *
 * Immediately changes to an animation that loops at the specified speed.
 */
#[no_mangle]
pub unsafe extern "C" fn Animation_PlayLoopSetSpeed(mut skelAnime:
                                                        *mut SkelAnime,
                                                    mut animation:
                                                        *mut AnimationHeader,
                                                    mut playSpeed: f32_0) {
    Animation_Change(skelAnime, animation, playSpeed, 0.0f32,
                     Animation_GetLastFrame(animation as *mut libc::c_void) as
                         f32_0, ANIMMODE_LOOP as libc::c_int as u8_0, 0.0f32);
}
/* *
 * Changes a looping animation to one that stops at the end. Unused
 */
#[no_mangle]
pub unsafe extern "C" fn Animation_EndLoop(mut skelAnime: *mut SkelAnime) {
    (*skelAnime).mode = ANIMMODE_ONCE as libc::c_int as u8_0;
    (*skelAnime).endFrame = (*skelAnime).animLength;
    SkelAnime_SetUpdate(skelAnime);
}
/* *
 * Reverses the current animation.
 */
#[no_mangle]
pub unsafe extern "C" fn Animation_Reverse(mut skelAnime: *mut SkelAnime) {
    let mut startFrame: f32_0 = (*skelAnime).startFrame;
    (*skelAnime).startFrame = (*skelAnime).endFrame;
    (*skelAnime).playSpeed = -(*skelAnime).playSpeed;
    (*skelAnime).endFrame = startFrame;
}
/* *
 * Copies the src frame table to the dst frame table if copyFlag for that limb is true.
 * Used only by En_Test
 */
#[no_mangle]
pub unsafe extern "C" fn SkelAnime_CopyFrameTableTrue(mut skelAnime:
                                                          *mut SkelAnime,
                                                      mut dst: *mut Vec3s,
                                                      mut src: *mut Vec3s,
                                                      mut copyFlag:
                                                          *mut u8_0) {
    let mut i: s32 = 0;
    i = 0 as libc::c_int;
    while i < (*skelAnime).limbCount as libc::c_int {
        let fresh31 = copyFlag;
        copyFlag = copyFlag.offset(1);
        if *fresh31 != 0 { *dst = *src }
        i += 1;
        dst = dst.offset(1);
        src = src.offset(1)
    };
}
/* *
 * Copies the src frame table to the dst frame table if copyFlag for that limb is false.
 * Unused.
 */
#[no_mangle]
pub unsafe extern "C" fn SkelAnime_CopyFrameTableFalse(mut skelAnime:
                                                           *mut SkelAnime,
                                                       mut dst: *mut Vec3s,
                                                       mut src: *mut Vec3s,
                                                       mut copyFlag:
                                                           *mut u8_0) {
    let mut i: s32 = 0;
    i = 0 as libc::c_int;
    while i < (*skelAnime).limbCount as libc::c_int {
        let fresh32 = copyFlag;
        copyFlag = copyFlag.offset(1);
        if *fresh32 == 0 { *dst = *src }
        i += 1;
        dst = dst.offset(1);
        src = src.offset(1)
    };
}
/* *
 * Updates translation of the root limb and calculates `pos`, the difference between
 * the old and new positions of the root limb as rotated by `angle`. Used to allow
 * animations to change an actor's position.
 */
#[no_mangle]
pub unsafe extern "C" fn SkelAnime_UpdateTranslation(mut skelAnime:
                                                         *mut SkelAnime,
                                                     mut diff: *mut Vec3f,
                                                     mut angle: s16) {
    let mut x: f32_0 = 0.;
    let mut z: f32_0 = 0.;
    let mut sin: f32_0 = 0.;
    let mut cos: f32_0 = 0.;
    if (*skelAnime).moveFlags as libc::c_int &
           (1 as libc::c_int) << 4 as libc::c_int != 0 {
        (*diff).z = 0.0f32;
        (*diff).x = (*diff).z
    } else {
        x =
            (*(*skelAnime).jointTable.offset(0 as libc::c_int as isize)).x as
                f32_0;
        z =
            (*(*skelAnime).jointTable.offset(0 as libc::c_int as isize)).z as
                f32_0;
        sin = Math_SinS(angle);
        cos = Math_CosS(angle);
        (*diff).x = x * cos + z * sin;
        (*diff).z = z * cos - x * sin;
        x = (*skelAnime).prevTransl.x as f32_0;
        z = (*skelAnime).prevTransl.z as f32_0;
        sin = Math_SinS((*skelAnime).prevRot);
        cos = Math_CosS((*skelAnime).prevRot);
        (*diff).x -= x * cos + z * sin;
        (*diff).z -= z * cos - x * sin
    }
    (*skelAnime).prevRot = angle;
    (*skelAnime).prevTransl.x =
        (*(*skelAnime).jointTable.offset(0 as libc::c_int as isize)).x;
    (*(*skelAnime).jointTable.offset(0 as libc::c_int as isize)).x =
        (*skelAnime).baseTransl.x;
    (*skelAnime).prevTransl.z =
        (*(*skelAnime).jointTable.offset(0 as libc::c_int as isize)).z;
    (*(*skelAnime).jointTable.offset(0 as libc::c_int as isize)).z =
        (*skelAnime).baseTransl.z;
    if (*skelAnime).moveFlags as libc::c_int &
           (1 as libc::c_int) << 1 as libc::c_int != 0 {
        if (*skelAnime).moveFlags as libc::c_int &
               (1 as libc::c_int) << 4 as libc::c_int != 0 {
            (*diff).y = 0.0f32
        } else {
            (*diff).y =
                ((*(*skelAnime).jointTable.offset(0 as libc::c_int as
                                                      isize)).y as libc::c_int
                     - (*skelAnime).prevTransl.y as libc::c_int) as f32_0
        }
        (*skelAnime).prevTransl.y =
            (*(*skelAnime).jointTable.offset(0 as libc::c_int as isize)).y;
        (*(*skelAnime).jointTable.offset(0 as libc::c_int as isize)).y =
            (*skelAnime).baseTransl.y
    } else {
        (*diff).y = 0.0f32;
        (*skelAnime).prevTransl.y =
            (*(*skelAnime).jointTable.offset(0 as libc::c_int as isize)).y
    }
    (*skelAnime).moveFlags =
        ((*skelAnime).moveFlags as libc::c_int &
             !((1 as libc::c_int) << 4 as libc::c_int)) as u8_0;
}
/* *
 * Checks if the current animation is at the specified frame
 */
#[no_mangle]
pub unsafe extern "C" fn Animation_OnFrame(mut skelAnime: *mut SkelAnime,
                                           mut frame: f32_0) -> s32 {
    return Animation_OnFrameImpl(skelAnime, frame, 1.0f32);
}
/* *
 * Frees the frame tables for a skelAnime with dynamically allocated tables.
 */
#[no_mangle]
pub unsafe extern "C" fn SkelAnime_Free(mut skelAnime: *mut SkelAnime,
                                        mut globalCtx: *mut GlobalContext) {
    if !(*skelAnime).jointTable.is_null() {
        ZeldaArena_FreeDebug((*skelAnime).jointTable as *mut libc::c_void,
                             b"../z_skelanime.c\x00" as *const u8 as
                                 *const libc::c_char, 3729 as libc::c_int);
    } else {
        osSyncPrintf(b"now_joint \xe3\x81\x82\xe3\x81\x8d\xe3\x81\xbe\xe3\x81\xb8\xe3\x82\x93\xef\xbc\x81\xef\xbc\x81\n\x00"
                         as *const u8 as *const libc::c_char);
        // "now_joint is freed! !"
    }
    if !(*skelAnime).morphTable.is_null() {
        ZeldaArena_FreeDebug((*skelAnime).morphTable as *mut libc::c_void,
                             b"../z_skelanime.c\x00" as *const u8 as
                                 *const libc::c_char, 3731 as libc::c_int);
    } else {
        osSyncPrintf(b"morf_joint \xe3\x81\x82\xe3\x81\x8d\xe3\x81\xbe\xe3\x81\xb8\xe3\x82\x93\xef\xbc\x81\xef\xbc\x81\n\x00"
                         as *const u8 as *const libc::c_char);
        // "morf_joint is freed !!"
    };
}
/* *
 * Copies the src frame table to the dst frame table.
 */
#[no_mangle]
pub unsafe extern "C" fn SkelAnime_CopyFrameTable(mut skelAnime:
                                                      *mut SkelAnime,
                                                  mut dst: *mut Vec3s,
                                                  mut src: *mut Vec3s) {
    let mut i: s32 = 0;
    i = 0 as libc::c_int;
    while i < (*skelAnime).limbCount as libc::c_int {
        let fresh33 = src;
        src = src.offset(1);
        let fresh34 = dst;
        dst = dst.offset(1);
        *fresh34 = *fresh33;
        i += 1
    };
}
