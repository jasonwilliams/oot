#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case,
         non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(const_raw_ptr_to_usize_cast, const_transmute, register_tool)]
extern "C" {
    #[no_mangle]
    fn DmaMgr_SendRequest1(ram0: *mut libc::c_void, vrom: u32_0, size: u32_0,
                           file: *const libc::c_char, line: s32) -> s32;
    #[no_mangle]
    fn osSyncPrintf(fmt: *const libc::c_char, _: ...);
    #[no_mangle]
    fn func_80094140(gfxCtx: *mut GraphicsContext);
    #[no_mangle]
    fn func_80095248(gfxCtx: *mut GraphicsContext, r: u8_0, g: u8_0, b: u8_0);
    #[no_mangle]
    fn Sram_InitDebugSave();
    #[no_mangle]
    fn View_Init(_: *mut View, _: *mut GraphicsContext);
    #[no_mangle]
    fn View_SetViewport(view: *mut View, viewport: *mut Viewport);
    #[no_mangle]
    fn func_800AAA50(view: *mut View, arg1: s32);
    #[no_mangle]
    fn Gameplay_Init(thisx: *mut GameState);
    #[no_mangle]
    fn GameState_Alloc(gameState: *mut GameState, size: size_t,
                       file: *mut libc::c_char, line: s32)
     -> *mut libc::c_void;
    #[no_mangle]
    fn Graph_OpenDisps(dispRefs: *mut *mut Gfx, gfxCtx: *mut GraphicsContext,
                       file: *const libc::c_char, line: s32);
    #[no_mangle]
    fn Graph_CloseDisps(dispRefs: *mut *mut Gfx, gfxCtx: *mut GraphicsContext,
                        file: *const libc::c_char, line: s32);
    #[no_mangle]
    fn Audio_PlaySoundGeneral(sfxId: u16_0, pos: *mut Vec3f, token: u8_0,
                              freqScale: *mut f32_0, a4: *mut f32_0,
                              reverbAdd: *mut s8);
    #[no_mangle]
    fn Audio_QueueSeqCmd(bgmID: u32_0);
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
    fn Rand_ZeroOne() -> f32_0;
    #[no_mangle]
    fn Title_Init(thisx: *mut GameState);
    #[no_mangle]
    static mut gSaveContext: SaveContext;
    #[no_mangle]
    static mut gGameInfo: *mut GameInfo;
    #[no_mangle]
    static mut gWeatherMode: u8_0;
    #[no_mangle]
    static mut D_801333E8: s8;
    #[no_mangle]
    static mut D_801333E0: f32_0;
    #[no_mangle]
    static mut D_801333D4: Vec3f;
    #[no_mangle]
    static mut _z_select_staticSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut _z_select_staticSegmentRomEnd: [u8_0; 0];
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
pub type PrintCallback
    =
    Option<unsafe extern "C" fn(_: *mut libc::c_void, _: *const libc::c_char,
                                _: u32_0) -> *mut libc::c_void>;
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
pub struct ItemEquips {
    pub buttonItems: [u8_0; 4],
    pub cButtonSlots: [u8_0; 3],
    pub equipment: u16_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Inventory {
    pub items: [u8_0; 24],
    pub ammo: [s8; 16],
    pub equipment: u16_0,
    pub upgrades: u32_0,
    pub questItems: u32_0,
    pub dungeonItems: [u8_0; 20],
    pub dungeonKeys: [s8; 19],
    pub defenseHearts: s8,
    pub gsTokens: s16,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SavedSceneFlags {
    pub chest: u32_0,
    pub swch: u32_0,
    pub clear: u32_0,
    pub collect: u32_0,
    pub unk: u32_0,
    pub rooms: u32_0,
    pub floors: u32_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct HorseData {
    pub scene: s16,
    pub pos: Vec3s,
    pub angle: s16,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct RespawnData {
    pub pos: Vec3f,
    pub yaw: s16,
    pub playerParams: s16,
    pub entranceIndex: s16,
    pub roomIndex: u8_0,
    pub data: s8,
    pub tempSwchFlags: u32_0,
    pub tempCollectFlags: u32_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct FaroresWindData {
    pub pos: Vec3i,
    pub yaw: s32,
    pub playerParams: s32,
    pub entranceIndex: s32,
    pub roomIndex: s32,
    pub set: s32,
    pub tempSwchFlags: s32,
    pub tempCollectFlags: s32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SaveContext {
    pub entranceIndex: s32,
    pub linkAge: s32,
    pub cutsceneIndex: s32,
    pub dayTime: u16_0,
    pub nightFlag: s32,
    pub totalDays: s32,
    pub bgsDayCount: s32,
    pub newf: [libc::c_char; 6],
    pub deaths: u16_0,
    pub playerName: [libc::c_char; 8],
    pub n64ddFlag: s16,
    pub healthCapacity: s16,
    pub health: s16,
    pub magicLevel: s8,
    pub magic: s8,
    pub rupees: s16,
    pub swordHealth: u16_0,
    pub naviTimer: u16_0,
    pub magicAcquired: u8_0,
    pub unk_3B: [libc::c_char; 1],
    pub doubleMagic: u8_0,
    pub doubleDefense: u8_0,
    pub bgsFlag: u8_0,
    pub ocarinaGameRoundNum: u8_0,
    pub childEquips: ItemEquips,
    pub adultEquips: ItemEquips,
    pub unk_54: u32_0,
    pub unk_58: [libc::c_char; 14],
    pub savedSceneNum: s16,
    pub equips: ItemEquips,
    pub inventory: Inventory,
    pub sceneFlags: [SavedSceneFlags; 124],
    pub fw: FaroresWindData,
    pub unk_E8C: [libc::c_char; 16],
    pub gsFlags: [s32; 6],
    pub unk_EB4: [libc::c_char; 4],
    pub highScores: [s32; 7],
    pub eventChkInf: [u16_0; 14],
    pub itemGetInf: [u16_0; 4],
    pub infTable: [u16_0; 30],
    pub unk_F34: [libc::c_char; 4],
    pub worldMapAreaData: u32_0,
    pub unk_F3C: [libc::c_char; 4],
    pub scarecrowCustomSongSet: u8_0,
    pub scarecrowCustomSong: [u8_0; 864],
    pub unk_12A1: [libc::c_char; 36],
    pub scarecrowSpawnSongSet: u8_0,
    pub scarecrowSpawnSong: [u8_0; 128],
    pub unk_1346: [libc::c_char; 2],
    pub horseData: HorseData,
    pub checksum: u16_0,
    pub fileNum: s32,
    pub unk_1358: [libc::c_char; 4],
    pub gameMode: s32,
    pub sceneSetupIndex: s32,
    pub respawnFlag: s32,
    pub respawn: [RespawnData; 3],
    pub entranceSpeed: f32_0,
    pub entranceSound: u16_0,
    pub unk_13C2: [libc::c_char; 1],
    pub unk_13C3: u8_0,
    pub dogParams: s16,
    pub textTriggerFlags: u8_0,
    pub showTitleCard: u8_0,
    pub nayrusLoveTimer: s16,
    pub unk_13CA: [libc::c_char; 2],
    pub rupeeAccumulator: s16,
    pub timer1State: s16,
    pub timer1Value: s16,
    pub timer2State: s16,
    pub timer2Value: s16,
    pub timerX: [s16; 2],
    pub timerY: [s16; 2],
    pub unk_13DE: [libc::c_char; 2],
    pub seqId: u8_0,
    pub natureAmbienceId: u8_0,
    pub buttonStatus: [u8_0; 5],
    pub unk_13E7: u8_0,
    pub unk_13E8: u16_0,
    pub unk_13EA: u16_0,
    pub unk_13EC: u16_0,
    pub unk_13EE: u16_0,
    pub unk_13F0: s16,
    pub unk_13F2: s16,
    pub unk_13F4: s16,
    pub unk_13F6: s16,
    pub unk_13F8: s16,
    pub eventInf: [u16_0; 4],
    pub mapIndex: u16_0,
    pub minigameState: u16_0,
    pub minigameScore: u16_0,
    pub unk_1408: [libc::c_char; 1],
    pub language: u8_0,
    pub audioSetting: u8_0,
    pub unk_140B: [libc::c_char; 1],
    pub zTargetSetting: u8_0,
    pub forcedSeqId: u16_0,
    pub unk_1410: u8_0,
    pub unk_1411: [libc::c_char; 1],
    pub nextCutsceneIndex: u16_0,
    pub cutsceneTrigger: u8_0,
    pub chamberCutsceneNum: u8_0,
    pub nextDayTime: u16_0,
    pub fadeDuration: u8_0,
    pub unk_1419: u8_0,
    pub skyboxTime: u16_0,
    pub dogIsLost: u8_0,
    pub nextTransition: u8_0,
    pub unk_141E: [libc::c_char; 2],
    pub worldMapArea: s16,
    pub sunsSongState: s16,
    pub healthAccumulator: s16,
}
pub type C2RustUnnamed_2 = libc::c_uint;
pub const RESPAWN_MODE_TOP: C2RustUnnamed_2 = 2;
pub const RESPAWN_MODE_RETURN: C2RustUnnamed_2 = 1;
pub const RESPAWN_MODE_DOWN: C2RustUnnamed_2 = 0;
pub type C2RustUnnamed_3 = libc::c_uint;
pub const BTN_DISABLED: C2RustUnnamed_3 = 255;
pub const BTN_ENABLED: C2RustUnnamed_3 = 0;
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
    pub c2rust_unnamed: C2RustUnnamed_4,
    pub rgba: u32_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_4 {
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
    pub c2rust_unnamed: C2RustUnnamed_5,
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
pub union C2RustUnnamed_5 {
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
    pub c2rust_unnamed: C2RustUnnamed_6,
    pub normal: Vec3s,
    pub dist: s16,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_6 {
    pub vtxData: [u16_0; 3],
    pub c2rust_unnamed: C2RustUnnamed_7,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_7 {
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
    pub sides: [C2RustUnnamed_8; 2],
    pub id: s16,
    pub pos: Vec3s,
    pub rotY: s16,
    pub params: s16,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_8 {
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
    pub c2rust_unnamed: C2RustUnnamed_9,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_9 {
    pub single: C2RustUnnamed_11,
    pub multi: C2RustUnnamed_10,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_10 {
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
pub struct C2RustUnnamed_11 {
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
    pub restrictions: C2RustUnnamed_12,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_12 {
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
    pub c2rust_unnamed: C2RustUnnamed_13,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_13 {
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
    pub c2rust_unnamed: C2RustUnnamed_14,
    pub startPos: Vec3i,
    pub endPos: Vec3i,
    pub normal: Vec3i,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_14 {
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
    pub flags: C2RustUnnamed_15,
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
pub struct C2RustUnnamed_15 {
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
pub type C2RustUnnamed_16 = libc::c_uint;
pub const SEQ_PLAYER_BGM_SUB: C2RustUnnamed_16 = 3;
pub const SEQ_PLAYER_SFX: C2RustUnnamed_16 = 2;
pub const SEQ_PLAYER_FANFARE: C2RustUnnamed_16 = 1;
pub const SEQ_PLAYER_BGM_MAIN: C2RustUnnamed_16 = 0;
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
pub struct TitleContext {
    pub state: GameState,
    pub staticSegment: *mut u8_0,
    pub view: View,
    pub sramCtx: SramContext,
    pub unk_1D4: u16_0,
    pub coverAlpha: s16,
    pub addAlpha: s16,
    pub visibleDuration: u16_0,
    pub ult: s16,
    pub uls: s16,
    pub unk_1E0: [libc::c_char; 1],
    pub exit: u8_0,
    pub unk_1E2: [libc::c_char; 6],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SelectContext {
    pub state: GameState,
    pub view: View,
    pub count: s32,
    pub scenes: *mut SceneSelectEntry,
    pub currentScene: s32,
    pub pageDownIndex: s32,
    pub pageDownStops: [s32; 7],
    pub unk_1FC: [libc::c_char; 12],
    pub opt: s32,
    pub topDisplayedScene: s32,
    pub unk_210: [libc::c_char; 12],
    pub verticalInputAccumulator: s32,
    pub verticalInput: s32,
    pub timerUp: s32,
    pub timerDown: s32,
    pub lockUp: s32,
    pub lockDown: s32,
    pub unk_234: s32,
    pub staticSegment: *mut u8_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SceneSelectEntry {
    pub name: *mut libc::c_char,
    pub loadFunc: Option<unsafe extern "C" fn(_: *mut SelectContext, _: s32)
                             -> ()>,
    pub entranceIndex: s32,
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
/*
 * File: z_select.c
 * Overlay: ovl_select
 * Description: Debug Scene Select Menu
 */
#[no_mangle]
pub unsafe extern "C" fn Select_LoadTitle(mut this: *mut SelectContext) {
    (*this).state.running = 0 as libc::c_int as u32_0;
    (*this).state.init =
        Some(Title_Init as unsafe extern "C" fn(_: *mut GameState) -> ());
    (*this).state.size =
        ::std::mem::size_of::<TitleContext>() as libc::c_ulong;
}
#[no_mangle]
pub unsafe extern "C" fn Select_LoadGame(mut this: *mut SelectContext,
                                         mut entranceIndex: s32) {
    osSyncPrintf(b"\x1b[34m\x00" as *const u8 as *const libc::c_char);
    osSyncPrintf(b"\n\n\n\xef\xbc\xa6\xef\xbc\xa9\xef\xbc\xac\xef\xbc\xa5\xef\xbc\xbf\xef\xbc\xae\xef\xbc\xaf\xef\xbc\x9d%x\n\n\n\x00"
                     as *const u8 as *const libc::c_char,
                 gSaveContext.fileNum);
    osSyncPrintf(b"\x1b[m\x00" as *const u8 as *const libc::c_char);
    if gSaveContext.fileNum == 0xff as libc::c_int {
        Sram_InitDebugSave();
        gSaveContext.unk_13F6 = gSaveContext.magic as s16;
        gSaveContext.magic = 0 as libc::c_int as s8;
        gSaveContext.unk_13F4 = 0 as libc::c_int as s16;
        gSaveContext.magicLevel = gSaveContext.magic
    }
    gSaveContext.buttonStatus[4 as libc::c_int as usize] =
        BTN_ENABLED as libc::c_int as u8_0;
    gSaveContext.buttonStatus[3 as libc::c_int as usize] =
        gSaveContext.buttonStatus[4 as libc::c_int as usize];
    gSaveContext.buttonStatus[2 as libc::c_int as usize] =
        gSaveContext.buttonStatus[3 as libc::c_int as usize];
    gSaveContext.buttonStatus[1 as libc::c_int as usize] =
        gSaveContext.buttonStatus[2 as libc::c_int as usize];
    gSaveContext.buttonStatus[0 as libc::c_int as usize] =
        gSaveContext.buttonStatus[1 as libc::c_int as usize];
    gSaveContext.unk_13EC = 0 as libc::c_int as u16_0;
    gSaveContext.unk_13EA = gSaveContext.unk_13EC;
    gSaveContext.unk_13E8 = gSaveContext.unk_13EA;
    gSaveContext.unk_13E7 = gSaveContext.unk_13E8 as u8_0;
    Audio_QueueSeqCmd(((SEQ_PLAYER_BGM_MAIN as libc::c_int) <<
                           24 as libc::c_int | 0x100000ff as libc::c_int) as
                          u32_0);
    gSaveContext.entranceIndex = entranceIndex;
    gSaveContext.respawnFlag = 0 as libc::c_int;
    gSaveContext.respawn[RESPAWN_MODE_DOWN as libc::c_int as
                             usize].entranceIndex =
        -(1 as libc::c_int) as s16;
    gSaveContext.seqId = 0xffff as libc::c_int as u8_0;
    gSaveContext.natureAmbienceId = 0xff as libc::c_int as u8_0;
    gSaveContext.showTitleCard = 1 as libc::c_int as u8_0;
    gWeatherMode = 0 as libc::c_int as u8_0;
    (*this).state.running = 0 as libc::c_int as u32_0;
    (*this).state.init =
        Some(Gameplay_Init as unsafe extern "C" fn(_: *mut GameState) -> ());
    (*this).state.size =
        ::std::mem::size_of::<GlobalContext>() as libc::c_ulong;
}
// "Translation" (Actual name)
static mut sScenes: [SceneSelectEntry; 126] =
    unsafe {
        [{
             let mut init =
                 SceneSelectEntry{name:
                                      b" 1:SPOT00\x00" as *const u8 as
                                          *const libc::c_char as
                                          *mut libc::c_char,
                                  loadFunc:
                                      Some(Select_LoadGame as
                                               unsafe extern "C" fn(_:
                                                                        *mut SelectContext,
                                                                    _: s32)
                                                   -> ()),
                                  entranceIndex: 0xcd as libc::c_int,};
             init
         },
         {
             let mut init =
                 SceneSelectEntry{name:
                                      b" 2:SPOT01\x00" as *const u8 as
                                          *const libc::c_char as
                                          *mut libc::c_char,
                                  loadFunc:
                                      Some(Select_LoadGame as
                                               unsafe extern "C" fn(_:
                                                                        *mut SelectContext,
                                                                    _: s32)
                                                   -> ()),
                                  entranceIndex: 0xdb as libc::c_int,};
             init
         },
         {
             let mut init =
                 SceneSelectEntry{name:
                                      b" 3:SPOT02\x00" as *const u8 as
                                          *const libc::c_char as
                                          *mut libc::c_char,
                                  loadFunc:
                                      Some(Select_LoadGame as
                                               unsafe extern "C" fn(_:
                                                                        *mut SelectContext,
                                                                    _: s32)
                                                   -> ()),
                                  entranceIndex: 0xe4 as libc::c_int,};
             init
         },
         {
             let mut init =
                 SceneSelectEntry{name:
                                      b" 4:SPOT03\x00" as *const u8 as
                                          *const libc::c_char as
                                          *mut libc::c_char,
                                  loadFunc:
                                      Some(Select_LoadGame as
                                               unsafe extern "C" fn(_:
                                                                        *mut SelectContext,
                                                                    _: s32)
                                                   -> ()),
                                  entranceIndex: 0xea as libc::c_int,};
             init
         },
         {
             let mut init =
                 SceneSelectEntry{name:
                                      b" 5:SPOT04\x00" as *const u8 as
                                          *const libc::c_char as
                                          *mut libc::c_char,
                                  loadFunc:
                                      Some(Select_LoadGame as
                                               unsafe extern "C" fn(_:
                                                                        *mut SelectContext,
                                                                    _: s32)
                                                   -> ()),
                                  entranceIndex: 0xee as libc::c_int,};
             init
         },
         {
             let mut init =
                 SceneSelectEntry{name:
                                      b" 6:SPOT05\x00" as *const u8 as
                                          *const libc::c_char as
                                          *mut libc::c_char,
                                  loadFunc:
                                      Some(Select_LoadGame as
                                               unsafe extern "C" fn(_:
                                                                        *mut SelectContext,
                                                                    _: s32)
                                                   -> ()),
                                  entranceIndex: 0xfc as libc::c_int,};
             init
         },
         {
             let mut init =
                 SceneSelectEntry{name:
                                      b" 7:SPOT06\x00" as *const u8 as
                                          *const libc::c_char as
                                          *mut libc::c_char,
                                  loadFunc:
                                      Some(Select_LoadGame as
                                               unsafe extern "C" fn(_:
                                                                        *mut SelectContext,
                                                                    _: s32)
                                                   -> ()),
                                  entranceIndex: 0x102 as libc::c_int,};
             init
         },
         {
             let mut init =
                 SceneSelectEntry{name:
                                      b" 8:SPOT07\x00" as *const u8 as
                                          *const libc::c_char as
                                          *mut libc::c_char,
                                  loadFunc:
                                      Some(Select_LoadGame as
                                               unsafe extern "C" fn(_:
                                                                        *mut SelectContext,
                                                                    _: s32)
                                                   -> ()),
                                  entranceIndex: 0x108 as libc::c_int,};
             init
         },
         {
             let mut init =
                 SceneSelectEntry{name:
                                      b" 9:SPOT08\x00" as *const u8 as
                                          *const libc::c_char as
                                          *mut libc::c_char,
                                  loadFunc:
                                      Some(Select_LoadGame as
                                               unsafe extern "C" fn(_:
                                                                        *mut SelectContext,
                                                                    _: s32)
                                                   -> ()),
                                  entranceIndex: 0x10e as libc::c_int,};
             init
         },
         {
             let mut init =
                 SceneSelectEntry{name:
                                      b"10:SPOT09\x00" as *const u8 as
                                          *const libc::c_char as
                                          *mut libc::c_char,
                                  loadFunc:
                                      Some(Select_LoadGame as
                                               unsafe extern "C" fn(_:
                                                                        *mut SelectContext,
                                                                    _: s32)
                                                   -> ()),
                                  entranceIndex: 0x117 as libc::c_int,};
             init
         },
         {
             let mut init =
                 SceneSelectEntry{name:
                                      b"11:SPOT10\x00" as *const u8 as
                                          *const libc::c_char as
                                          *mut libc::c_char,
                                  loadFunc:
                                      Some(Select_LoadGame as
                                               unsafe extern "C" fn(_:
                                                                        *mut SelectContext,
                                                                    _: s32)
                                                   -> ()),
                                  entranceIndex: 0x11e as libc::c_int,};
             init
         },
         {
             let mut init =
                 SceneSelectEntry{name:
                                      b"12:SPOT11\x00" as *const u8 as
                                          *const libc::c_char as
                                          *mut libc::c_char,
                                  loadFunc:
                                      Some(Select_LoadGame as
                                               unsafe extern "C" fn(_:
                                                                        *mut SelectContext,
                                                                    _: s32)
                                                   -> ()),
                                  entranceIndex: 0x123 as libc::c_int,};
             init
         },
         {
             let mut init =
                 SceneSelectEntry{name:
                                      b"13:SPOT12\x00" as *const u8 as
                                          *const libc::c_char as
                                          *mut libc::c_char,
                                  loadFunc:
                                      Some(Select_LoadGame as
                                               unsafe extern "C" fn(_:
                                                                        *mut SelectContext,
                                                                    _: s32)
                                                   -> ()),
                                  entranceIndex: 0x129 as libc::c_int,};
             init
         },
         {
             let mut init =
                 SceneSelectEntry{name:
                                      b"14:SPOT13\x00" as *const u8 as
                                          *const libc::c_char as
                                          *mut libc::c_char,
                                  loadFunc:
                                      Some(Select_LoadGame as
                                               unsafe extern "C" fn(_:
                                                                        *mut SelectContext,
                                                                    _: s32)
                                                   -> ()),
                                  entranceIndex: 0x130 as libc::c_int,};
             init
         },
         {
             let mut init =
                 SceneSelectEntry{name:
                                      b"15:SPOT15\x00" as *const u8 as
                                          *const libc::c_char as
                                          *mut libc::c_char,
                                  loadFunc:
                                      Some(Select_LoadGame as
                                               unsafe extern "C" fn(_:
                                                                        *mut SelectContext,
                                                                    _: s32)
                                                   -> ()),
                                  entranceIndex: 0x138 as libc::c_int,};
             init
         },
         {
             let mut init =
                 SceneSelectEntry{name:
                                      b"16:SPOT16\x00" as *const u8 as
                                          *const libc::c_char as
                                          *mut libc::c_char,
                                  loadFunc:
                                      Some(Select_LoadGame as
                                               unsafe extern "C" fn(_:
                                                                        *mut SelectContext,
                                                                    _: s32)
                                                   -> ()),
                                  entranceIndex: 0x13d as libc::c_int,};
             init
         },
         {
             let mut init =
                 SceneSelectEntry{name:
                                      b"17:SPOT17\x00" as *const u8 as
                                          *const libc::c_char as
                                          *mut libc::c_char,
                                  loadFunc:
                                      Some(Select_LoadGame as
                                               unsafe extern "C" fn(_:
                                                                        *mut SelectContext,
                                                                    _: s32)
                                                   -> ()),
                                  entranceIndex: 0x147 as libc::c_int,};
             init
         },
         {
             let mut init =
                 SceneSelectEntry{name:
                                      b"18:SPOT18\x00" as *const u8 as
                                          *const libc::c_char as
                                          *mut libc::c_char,
                                  loadFunc:
                                      Some(Select_LoadGame as
                                               unsafe extern "C" fn(_:
                                                                        *mut SelectContext,
                                                                    _: s32)
                                                   -> ()),
                                  entranceIndex: 0x14d as libc::c_int,};
             init
         },
         {
             let mut init =
                 SceneSelectEntry{name:
                                      b"19:SPOT20\x00" as *const u8 as
                                          *const libc::c_char as
                                          *mut libc::c_char,
                                  loadFunc:
                                      Some(Select_LoadGame as
                                               unsafe extern "C" fn(_:
                                                                        *mut SelectContext,
                                                                    _: s32)
                                                   -> ()),
                                  entranceIndex: 0x157 as libc::c_int,};
             init
         },
         {
             let mut init =
                 SceneSelectEntry{name:
                                      b"20:\x8d\xef\xbe\x84\xef\xbd\xb7\xef\xbe\x89\xef\xbe\x8f\x00"
                                          as *const u8 as *const libc::c_char
                                          as *mut libc::c_char,
                                  loadFunc:
                                      Some(Select_LoadGame as
                                               unsafe extern "C" fn(_:
                                                                        *mut SelectContext,
                                                                    _: s32)
                                                   -> ()),
                                  entranceIndex: 0x53 as libc::c_int,};
             init
         },
         {
             let mut init =
                 SceneSelectEntry{name:
                                      b"21:\x8d\xef\xbd\xb9\xef\xbe\x9d\xef\xbd\xbc\xef\xbe\x9e\xef\xbd\xac\xef\xbe\x89\xef\xbe\x8f\x00"
                                          as *const u8 as *const libc::c_char
                                          as *mut libc::c_char,
                                  loadFunc:
                                      Some(Select_LoadGame as
                                               unsafe extern "C" fn(_:
                                                                        *mut SelectContext,
                                                                    _: s32)
                                                   -> ()),
                                  entranceIndex: 0x6b as libc::c_int,};
             init
         },
         {
             let mut init =
                 SceneSelectEntry{name:
                                      b"22:\x8d\xef\xbd\xbc\xef\xbd\xac\xef\xbe\x83\xef\xbd\xb7\xef\xbd\xbc\xef\xbe\x9e\xef\xbd\xae\xef\xbd\xb3\x00"
                                          as *const u8 as *const libc::c_char
                                          as *mut libc::c_char,
                                  loadFunc:
                                      Some(Select_LoadGame as
                                               unsafe extern "C" fn(_:
                                                                        *mut SelectContext,
                                                                    _: s32)
                                                   -> ()),
                                  entranceIndex: 0x3b as libc::c_int,};
             init
         },
         {
             let mut init =
                 SceneSelectEntry{name:
                                      b"23:\x8c\xef\xbe\x8a\xef\xbd\xb2\xef\xbe\x97\xef\xbe\x99\x8d\xef\xbe\x86\xef\xbe\x9c\x8c\xef\xbd\xb9\xef\xbe\x9e\xef\xbd\xb0\xef\xbe\x91\x00"
                                          as *const u8 as *const libc::c_char
                                          as *mut libc::c_char,
                                  loadFunc:
                                      Some(Select_LoadGame as
                                               unsafe extern "C" fn(_:
                                                                        *mut SelectContext,
                                                                    _: s32)
                                                   -> ()),
                                  entranceIndex: 0x7a as libc::c_int,};
             init
         },
         {
             let mut init =
                 SceneSelectEntry{name:
                                      b"24:\x8d\xef\xbe\x8a\xef\xbd\xb6\xef\xbd\xbc\xef\xbe\x80\xef\xbe\x84\xef\xbe\x8b\xef\xbe\x9e\xef\xbd\xba\xef\xbe\x90\xef\xbd\xb1\xef\xbe\x85\x00"
                                          as *const u8 as *const libc::c_char
                                          as *mut libc::c_char,
                                  loadFunc:
                                      Some(Select_LoadGame as
                                               unsafe extern "C" fn(_:
                                                                        *mut SelectContext,
                                                                    _: s32)
                                                   -> ()),
                                  entranceIndex: 0x31c as libc::c_int,};
             init
         },
         {
             let mut init =
                 SceneSelectEntry{name:
                                      b"25:\x8d\xef\xbe\x8a\xef\xbd\xb6\xef\xbd\xbc\xef\xbe\x80\xef\xbe\x84\xef\xbe\x8b\xef\xbe\x9e\xef\xbd\xba\xef\xbe\x90\xef\xbd\xb1\xef\xbe\x85 2\x00"
                                          as *const u8 as *const libc::c_char
                                          as *mut libc::c_char,
                                  loadFunc:
                                      Some(Select_LoadGame as
                                               unsafe extern "C" fn(_:
                                                                        *mut SelectContext,
                                                                    _: s32)
                                                   -> ()),
                                  entranceIndex: 0x4b as libc::c_int,};
             init
         },
         {
             let mut init =
                 SceneSelectEntry{name:
                                      b"26:\x8d\xef\xbd\xb5\xef\xbd\xb3\xef\xbd\xb9 \xef\xbe\x89 \xef\xbe\x8a\xef\xbd\xb6\xef\xbd\xb1\xef\xbe\x85\x00"
                                          as *const u8 as *const libc::c_char
                                          as *mut libc::c_char,
                                  loadFunc:
                                      Some(Select_LoadGame as
                                               unsafe extern "C" fn(_:
                                                                        *mut SelectContext,
                                                                    _: s32)
                                                   -> ()),
                                  entranceIndex: 0x2d as libc::c_int,};
             init
         },
         {
             let mut init =
                 SceneSelectEntry{name:
                                      b"27:\x8d\xef\xbe\x80\xef\xbe\x9e\xef\xbd\xb2\xef\xbe\x96\xef\xbd\xb3\xef\xbd\xbe\xef\xbd\xb2\xef\xbe\x89\xef\xbd\xb2\xef\xbd\xbd\xef\xbe\x9e\xef\xbe\x90\x00"
                                          as *const u8 as *const libc::c_char
                                          as *mut libc::c_char,
                                  loadFunc:
                                      Some(Select_LoadGame as
                                               unsafe extern "C" fn(_:
                                                                        *mut SelectContext,
                                                                    _: s32)
                                                   -> ()),
                                  entranceIndex: 0x315 as libc::c_int,};
             init
         },
         {
             let mut init =
                 SceneSelectEntry{name:
                                      b"28:\x8d\xef\xbe\x84\xef\xbe\x8b\xef\xbe\x9e\xef\xbd\xba\xef\xbe\x90 \xef\xbe\x96\xef\xbd\xb3\xef\xbd\xbe\xef\xbd\xb2 \xef\xbd\xb1\xef\xbe\x85\x00"
                                          as *const u8 as *const libc::c_char
                                          as *mut libc::c_char,
                                  loadFunc:
                                      Some(Select_LoadGame as
                                               unsafe extern "C" fn(_:
                                                                        *mut SelectContext,
                                                                    _: s32)
                                                   -> ()),
                                  entranceIndex: 0x36d as libc::c_int,};
             init
         },
         {
             let mut init =
                 SceneSelectEntry{name:
                                      b"29:\x8d\xef\xbe\x8f\xef\xbe\x8e\xef\xbd\xb3\xef\xbd\xbe\xef\xbd\xb7 \xef\xbe\x96\xef\xbd\xb3\xef\xbd\xbe\xef\xbd\xb2\xef\xbe\x89\xef\xbd\xb2\xef\xbd\xbd\xef\xbe\x9e\xef\xbe\x90\x00"
                                          as *const u8 as *const libc::c_char
                                          as *mut libc::c_char,
                                  loadFunc:
                                      Some(Select_LoadGame as
                                               unsafe extern "C" fn(_:
                                                                        *mut SelectContext,
                                                                    _: s32)
                                                   -> ()),
                                  entranceIndex: 0x371 as libc::c_int,};
             init
         },
         {
             let mut init =
                 SceneSelectEntry{name:
                                      b"30:\x8c\xef\xbd\xb6\xef\xbe\x9e\xef\xbe\x89\xef\xbe\x9d\x8d\xef\xbd\xbb\xef\xbd\xb2\xef\xbd\xbc\xef\xbd\xad\xef\xbd\xb3\xef\xbd\xbe\xef\xbe\x9d\x00"
                                          as *const u8 as *const libc::c_char
                                          as *mut libc::c_char,
                                  loadFunc:
                                      Some(Select_LoadGame as
                                               unsafe extern "C" fn(_:
                                                                        *mut SelectContext,
                                                                    _: s32)
                                                   -> ()),
                                  entranceIndex: 0x43f as libc::c_int,};
             init
         },
         {
             let mut init =
                 SceneSelectEntry{name:
                                      b"31:\x8c\xef\xbe\x8a\xef\xbd\xb2\xef\xbe\x97\xef\xbe\x99\x8d\xef\xbe\x85\xef\xbd\xb6\xef\xbe\x86\xef\xbe\x9c\x00"
                                          as *const u8 as *const libc::c_char
                                          as *mut libc::c_char,
                                  loadFunc:
                                      Some(Select_LoadGame as
                                               unsafe extern "C" fn(_:
                                                                        *mut SelectContext,
                                                                    _: s32)
                                                   -> ()),
                                  entranceIndex: 0x400 as libc::c_int,};
             init
         },
         {
             let mut init =
                 SceneSelectEntry{name:
                                      b"32:\x8d\xef\xbe\x82\xef\xbe\x98\xef\xbe\x8e\xef\xbe\x9e\xef\xbe\x98\x00"
                                          as *const u8 as *const libc::c_char
                                          as *mut libc::c_char,
                                  loadFunc:
                                      Some(Select_LoadGame as
                                               unsafe extern "C" fn(_:
                                                                        *mut SelectContext,
                                                                    _: s32)
                                                   -> ()),
                                  entranceIndex: 0x45f as libc::c_int,};
             init
         },
         {
             let mut init =
                 SceneSelectEntry{name:
                                      b"33:\x8c\xef\xbe\x8e\xef\xbe\x9e\xef\xbe\x91\xef\xbe\x81\xef\xbd\xad\xef\xbd\xb3\xef\xbe\x8e\xef\xbe\x9e\xef\xbd\xb0\xef\xbe\x98\xef\xbe\x9d\xef\xbd\xb8\xef\xbe\x9e\x00"
                                          as *const u8 as *const libc::c_char
                                          as *mut libc::c_char,
                                  loadFunc:
                                      Some(Select_LoadGame as
                                               unsafe extern "C" fn(_:
                                                                        *mut SelectContext,
                                                                    _: s32)
                                                   -> ()),
                                  entranceIndex: 0x507 as libc::c_int,};
             init
         },
         {
             let mut init =
                 SceneSelectEntry{name:
                                      b"34:\x8c\xef\xbe\x9b\xef\xbe\x9d\xef\xbe\x9b\xef\xbe\x9d\x8d\xef\xbe\x8e\xef\xbe\x9e\xef\xbd\xb8\xef\xbd\xbc\xef\xbe\x9e\xef\xbd\xae\xef\xbd\xb3 \xef\xbd\xbf\xef\xbd\xb3\xef\xbd\xba 1\x00"
                                          as *const u8 as *const libc::c_char
                                          as *mut libc::c_char,
                                  loadFunc:
                                      Some(Select_LoadGame as
                                               unsafe extern "C" fn(_:
                                                                        *mut SelectContext,
                                                                    _: s32)
                                                   -> ()),
                                  entranceIndex: 0x4f as libc::c_int,};
             init
         },
         {
             let mut init =
                 SceneSelectEntry{name:
                                      b"35:\x8c\xef\xbe\x9b\xef\xbe\x9d\xef\xbe\x9b\xef\xbe\x9d\x8d\xef\xbe\x8e\xef\xbe\x9e\xef\xbd\xb8\xef\xbd\xbc\xef\xbe\x9e\xef\xbd\xae\xef\xbd\xb3 \xef\xbd\xbf\xef\xbd\xb3\xef\xbd\xba 2\x00"
                                          as *const u8 as *const libc::c_char
                                          as *mut libc::c_char,
                                  loadFunc:
                                      Some(Select_LoadGame as
                                               unsafe extern "C" fn(_:
                                                                        *mut SelectContext,
                                                                    _: s32)
                                                   -> ()),
                                  entranceIndex: 0x5d0 as libc::c_int,};
             init
         },
         {
             let mut init =
                 SceneSelectEntry{name:
                                      b"36:\x8d\xef\xbe\x90\xef\xbe\x8a\xef\xbe\x98 \xef\xbd\xba\xef\xbe\x9e\xef\xbe\x94\x00"
                                          as *const u8 as *const libc::c_char
                                          as *mut libc::c_char,
                                  loadFunc:
                                      Some(Select_LoadGame as
                                               unsafe extern "C" fn(_:
                                                                        *mut SelectContext,
                                                                    _: s32)
                                                   -> ()),
                                  entranceIndex: 0x7e as libc::c_int,};
             init
         },
         {
             let mut init =
                 SceneSelectEntry{name:
                                      b"37:\x8d\xef\xbe\x8f\xef\xbe\x8e\xef\xbd\xb3 \xef\xbe\x89 \xef\xbd\xb8\xef\xbd\xbd\xef\xbe\x98\xef\xbe\x94\x00"
                                          as *const u8 as *const libc::c_char
                                          as *mut libc::c_char,
                                  loadFunc:
                                      Some(Select_LoadGame as
                                               unsafe extern "C" fn(_:
                                                                        *mut SelectContext,
                                                                    _: s32)
                                                   -> ()),
                                  entranceIndex: 0x72 as libc::c_int,};
             init
         },
         {
             let mut init =
                 SceneSelectEntry{name:
                                      b"38:\x8d\xef\xbe\x80\xef\xbd\xb6\xef\xbe\x97\xef\xbe\x8a\xef\xbe\x9e\xef\xbd\xba\xef\xbe\x94\x00"
                                          as *const u8 as *const libc::c_char
                                          as *mut libc::c_char,
                                  loadFunc:
                                      Some(Select_LoadGame as
                                               unsafe extern "C" fn(_:
                                                                        *mut SelectContext,
                                                                    _: s32)
                                                   -> ()),
                                  entranceIndex: 0x63 as libc::c_int,};
             init
         },
         {
             let mut init =
                 SceneSelectEntry{name:
                                      b"39:\x8d\xef\xbd\xb7\xef\xbe\x9d \x8c\xef\xbd\xbd\xef\xbe\x80\xef\xbe\x99\xef\xbe\x81\xef\xbd\xad\xef\xbe\x97 \xef\xbe\x8a\xef\xbd\xb3\xef\xbd\xbd\x00"
                                          as *const u8 as *const libc::c_char
                                          as *mut libc::c_char,
                                  loadFunc:
                                      Some(Select_LoadGame as
                                               unsafe extern "C" fn(_:
                                                                        *mut SelectContext,
                                                                    _: s32)
                                                   -> ()),
                                  entranceIndex: 0x550 as libc::c_int,};
             init
         },
         {
             let mut init =
                 SceneSelectEntry{name:
                                      b"40:\x8d\xef\xbd\xbc\xef\xbe\x9e\xef\xbd\xae\xef\xbd\xb3\xef\xbd\xb6\xef\xbe\x8f\xef\xbe\x81 \xef\xbd\xb2\xef\xbe\x98\xef\xbd\xb8\xef\xbe\x9e\xef\xbe\x81\x00"
                                          as *const u8 as *const libc::c_char
                                          as *mut libc::c_char,
                                  loadFunc:
                                      Some(Select_LoadGame as
                                               unsafe extern "C" fn(_:
                                                                        *mut SelectContext,
                                                                    _: s32)
                                                   -> ()),
                                  entranceIndex: 0x33 as libc::c_int,};
             init
         },
         {
             let mut init =
                 SceneSelectEntry{name:
                                      b"41:\x8d\xef\xbd\xbc\xef\xbe\x9e\xef\xbd\xae\xef\xbd\xb3\xef\xbd\xb6\xef\xbe\x8f\xef\xbe\x81\x00"
                                          as *const u8 as *const libc::c_char
                                          as *mut libc::c_char,
                                  loadFunc:
                                      Some(Select_LoadGame as
                                               unsafe extern "C" fn(_:
                                                                        *mut SelectContext,
                                                                    _: s32)
                                                   -> ()),
                                  entranceIndex: 0xb1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 SceneSelectEntry{name:
                                      b"42:\x8d\xef\xbd\xb3\xef\xbe\x97\xef\xbe\x9b\xef\xbd\xbc\xef\xbe\x9e\x00"
                                          as *const u8 as *const libc::c_char
                                          as *mut libc::c_char,
                                  loadFunc:
                                      Some(Select_LoadGame as
                                               unsafe extern "C" fn(_:
                                                                        *mut SelectContext,
                                                                    _: s32)
                                                   -> ()),
                                  entranceIndex: 0xad as libc::c_int,};
             init
         },
         {
             let mut init =
                 SceneSelectEntry{name:
                                      b"43:\x8d\xef\xbe\x84\xef\xbd\xb7\xef\xbe\x89\xef\xbd\xbc\xef\xbe\x9d\xef\xbe\x83\xef\xbe\x9e\xef\xbe\x9d \xef\xbe\x8f\xef\xbd\xb4\x00"
                                          as *const u8 as *const libc::c_char
                                          as *mut libc::c_char,
                                  loadFunc:
                                      Some(Select_LoadGame as
                                               unsafe extern "C" fn(_:
                                                                        *mut SelectContext,
                                                                    _: s32)
                                                   -> ()),
                                  entranceIndex: 0x171 as libc::c_int,};
             init
         },
         {
             let mut init =
                 SceneSelectEntry{name:
                                      b"44:\x8d\xef\xbe\x98\xef\xbe\x9d\xef\xbd\xb8\xef\xbe\x89\xef\xbd\xb2\xef\xbd\xb4\x00"
                                          as *const u8 as *const libc::c_char
                                          as *mut libc::c_char,
                                  loadFunc:
                                      Some(Select_LoadGame as
                                               unsafe extern "C" fn(_:
                                                                        *mut SelectContext,
                                                                    _: s32)
                                                   -> ()),
                                  entranceIndex: 0xbb as libc::c_int,};
             init
         },
         {
             let mut init =
                 SceneSelectEntry{name:
                                      b"45:\x8c\xef\xbd\xb6\xef\xbd\xb6\xef\xbe\x98\xef\xbd\xba\x8d\xef\xbe\x91\xef\xbe\x97\xef\xbe\x89\xef\xbe\x85\xef\xbd\xb6\xef\xbe\x9e\xef\xbe\x94\x00"
                                          as *const u8 as *const libc::c_char
                                          as *mut libc::c_char,
                                  loadFunc:
                                      Some(Select_LoadGame as
                                               unsafe extern "C" fn(_:
                                                                        *mut SelectContext,
                                                                    _: s32)
                                                   -> ()),
                                  entranceIndex: 0x2fd as libc::c_int,};
             init
         },
         {
             let mut init =
                 SceneSelectEntry{name:
                                      b"46:\x8d\xef\xbd\xb3\xef\xbe\x97\xef\xbe\x9b\xef\xbd\xbc\xef\xbe\x9e\xef\xbe\x89 \xef\xbd\xb2\xef\xbd\xb4\x00"
                                          as *const u8 as *const libc::c_char
                                          as *mut libc::c_char,
                                  loadFunc:
                                      Some(Select_LoadGame as
                                               unsafe extern "C" fn(_:
                                                                        *mut SelectContext,
                                                                    _: s32)
                                                   -> ()),
                                  entranceIndex: 0x43b as libc::c_int,};
             init
         },
         {
             let mut init =
                 SceneSelectEntry{name:
                                      b"47:\x8d\xef\xbd\xba\xef\xbd\xb7\xef\xbe\x98\xef\xbe\x89\xef\xbe\x91\xef\xbe\x97 \xef\xbe\x93\xef\xbe\x89\xef\xbd\xbc\xef\xbe\x98\xef\xbd\xb7\xef\xbd\xae\xef\xbd\xb3\xef\xbe\x80\xef\xbe\x9e\xef\xbd\xb2\xef\xbe\x89\xef\xbd\xb2\xef\xbd\xb4\x00"
                                          as *const u8 as *const libc::c_char
                                          as *mut libc::c_char,
                                  loadFunc:
                                      Some(Select_LoadGame as
                                               unsafe extern "C" fn(_:
                                                                        *mut SelectContext,
                                                                    _: s32)
                                                   -> ()),
                                  entranceIndex: 0xc9 as libc::c_int,};
             init
         },
         {
             let mut init =
                 SceneSelectEntry{name:
                                      b"48:\x8d\xef\xbd\xba\xef\xbd\xb7\xef\xbe\x98\xef\xbe\x89\xef\xbe\x91\xef\xbe\x97 \xef\xbe\x8c\xef\xbe\x80\xef\xbd\xba\xef\xbe\x9e\xef\xbe\x89\xef\xbd\xb2\xef\xbd\xb4\x00"
                                          as *const u8 as *const libc::c_char
                                          as *mut libc::c_char,
                                  loadFunc:
                                      Some(Select_LoadGame as
                                               unsafe extern "C" fn(_:
                                                                        *mut SelectContext,
                                                                    _: s32)
                                                   -> ()),
                                  entranceIndex: 0x9c as libc::c_int,};
             init
         },
         {
             let mut init =
                 SceneSelectEntry{name:
                                      b"49:\x8d\xef\xbd\xba\xef\xbd\xb7\xef\xbe\x98\xef\xbe\x89\xef\xbe\x91\xef\xbe\x97 \x8c\xef\xbe\x90\xef\xbe\x84\xef\xbe\x9e\x8d\xef\xbe\x89\xef\xbd\xb2\xef\xbd\xb4\x00"
                                          as *const u8 as *const libc::c_char
                                          as *mut libc::c_char,
                                  loadFunc:
                                      Some(Select_LoadGame as
                                               unsafe extern "C" fn(_:
                                                                        *mut SelectContext,
                                                                    _: s32)
                                                   -> ()),
                                  entranceIndex: 0x433 as libc::c_int,};
             init
         },
         {
             let mut init =
                 SceneSelectEntry{name:
                                      b"50:\x8d\xef\xbd\xba\xef\xbd\xb7\xef\xbe\x98\xef\xbe\x89\xef\xbe\x91\xef\xbe\x97 \x8c\xef\xbd\xbb\xef\xbe\x98\xef\xbd\xb1\x8d\xef\xbe\x89\xef\xbd\xb2\xef\xbd\xb4\x00"
                                          as *const u8 as *const libc::c_char
                                          as *mut libc::c_char,
                                  loadFunc:
                                      Some(Select_LoadGame as
                                               unsafe extern "C" fn(_:
                                                                        *mut SelectContext,
                                                                    _: s32)
                                                   -> ()),
                                  entranceIndex: 0x437 as libc::c_int,};
             init
         },
         {
             let mut init =
                 SceneSelectEntry{name:
                                      b"51:\x8d\xef\xbd\xb3\xef\xbe\x8f\xef\xbd\xba\xef\xbe\x9e\xef\xbe\x94\x00"
                                          as *const u8 as *const libc::c_char
                                          as *mut libc::c_char,
                                  loadFunc:
                                      Some(Select_LoadGame as
                                               unsafe extern "C" fn(_:
                                                                        *mut SelectContext,
                                                                    _: s32)
                                                   -> ()),
                                  entranceIndex: 0x2f9 as libc::c_int,};
             init
         },
         {
             let mut init =
                 SceneSelectEntry{name:
                                      b"52:\x8d\xef\xbe\x8a\xef\xbd\xb6\xef\xbe\x93\xef\xbe\x98\xef\xbe\x89\xef\xbd\xb2\xef\xbd\xb4\x00"
                                          as *const u8 as *const libc::c_char
                                          as *mut libc::c_char,
                                  loadFunc:
                                      Some(Select_LoadGame as
                                               unsafe extern "C" fn(_:
                                                                        *mut SelectContext,
                                                                    _: s32)
                                                   -> ()),
                                  entranceIndex: 0x30d as libc::c_int,};
             init
         },
         {
             let mut init =
                 SceneSelectEntry{name:
                                      b"53:\x8d\xef\xbd\xb3\xef\xbe\x97\xef\xbe\x9b\xef\xbd\xbc\xef\xbe\x9e \xef\xbd\xb2\xef\xbe\x87\xef\xbd\xb5\xef\xbe\x8a\xef\xbe\x9e\xef\xbd\xbb\xef\xbe\x9d\xef\xbe\x89\xef\xbd\xb2\xef\xbd\xb4\x00"
                                          as *const u8 as *const libc::c_char
                                          as *mut libc::c_char,
                                  loadFunc:
                                      Some(Select_LoadGame as
                                               unsafe extern "C" fn(_:
                                                                        *mut SelectContext,
                                                                    _: s32)
                                                   -> ()),
                                  entranceIndex: 0x398 as libc::c_int,};
             init
         },
         {
             let mut init =
                 SceneSelectEntry{name:
                                      b"54:\x8d\xef\xbd\xb6\xef\xbd\xb6\xef\xbe\x98\xef\xbd\xba\xef\xbe\x91\xef\xbe\x97 \x8c\xef\xbd\xb2\xef\xbe\x9d\xef\xbe\x8a\xef\xbe\x9f\x8d\xef\xbe\x89\xef\xbd\xb2\xef\xbd\xb4\x00"
                                          as *const u8 as *const libc::c_char
                                          as *mut libc::c_char,
                                  loadFunc:
                                      Some(Select_LoadGame as
                                               unsafe extern "C" fn(_:
                                                                        *mut SelectContext,
                                                                    _: s32)
                                                   -> ()),
                                  entranceIndex: 0x39c as libc::c_int,};
             init
         },
         {
             let mut init =
                 SceneSelectEntry{name:
                                      b"55:\x8c\xef\xbe\x8a\xef\xbd\xb2\xef\xbe\x98\xef\xbd\xb1\x8d \xef\xbd\xb9\xef\xbe\x9d\xef\xbd\xb7\xef\xbd\xad\xef\xbd\xb3\xef\xbd\xbc\xef\xbe\x9e\xef\xbd\xae\x00"
                                          as *const u8 as *const libc::c_char
                                          as *mut libc::c_char,
                                  loadFunc:
                                      Some(Select_LoadGame as
                                               unsafe extern "C" fn(_:
                                                                        *mut SelectContext,
                                                                    _: s32)
                                                   -> ()),
                                  entranceIndex: 0x43 as libc::c_int,};
             init
         },
         {
             let mut init =
                 SceneSelectEntry{name:
                                      b"56:\x8c\xef\xbe\x83\xef\xbe\x9d\xef\xbe\x84\x00"
                                          as *const u8 as *const libc::c_char
                                          as *mut libc::c_char,
                                  loadFunc:
                                      Some(Select_LoadGame as
                                               unsafe extern "C" fn(_:
                                                                        *mut SelectContext,
                                                                    _: s32)
                                                   -> ()),
                                  entranceIndex: 0x3a0 as libc::c_int,};
             init
         },
         {
             let mut init =
                 SceneSelectEntry{name:
                                      b"57:\x8d\xef\xbe\x80\xef\xbe\x83\xef\xbe\x89\xef\xbe\x90\xef\xbd\xbe\x00"
                                          as *const u8 as *const libc::c_char
                                          as *mut libc::c_char,
                                  loadFunc:
                                      Some(Select_LoadGame as
                                               unsafe extern "C" fn(_:
                                                                        *mut SelectContext,
                                                                    _: s32)
                                                   -> ()),
                                  entranceIndex: 0xb7 as libc::c_int,};
             init
         },
         {
             let mut init =
                 SceneSelectEntry{name:
                                      b"58:\x8d\xef\xbd\xba\xef\xbd\xb7\xef\xbe\x98\xef\xbd\xbf\xef\xbe\x9e\xef\xbd\xb8\xef\xbe\x89\xef\xbe\x90\xef\xbd\xbe\x00"
                                          as *const u8 as *const libc::c_char
                                          as *mut libc::c_char,
                                  loadFunc:
                                      Some(Select_LoadGame as
                                               unsafe extern "C" fn(_:
                                                                        *mut SelectContext,
                                                                    _: s32)
                                                   -> ()),
                                  entranceIndex: 0xc1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 SceneSelectEntry{name:
                                      b"59:\x8c\xef\xbd\xba\xef\xbe\x9e\xef\xbe\x9b\xef\xbe\x9d\x8d\xef\xbe\x89\xef\xbe\x90\xef\xbd\xbe\x00"
                                          as *const u8 as *const libc::c_char
                                          as *mut libc::c_char,
                                  loadFunc:
                                      Some(Select_LoadGame as
                                               unsafe extern "C" fn(_:
                                                                        *mut SelectContext,
                                                                    _: s32)
                                                   -> ()),
                                  entranceIndex: 0x37c as libc::c_int,};
             init
         },
         {
             let mut init =
                 SceneSelectEntry{name:
                                      b"60:\x8c\xef\xbd\xbf\xef\xbe\x9e\xef\xbd\xb0\xef\xbe\x97\x8d\xef\xbe\x89\xef\xbe\x90\xef\xbd\xbe\x00"
                                          as *const u8 as *const libc::c_char
                                          as *mut libc::c_char,
                                  loadFunc:
                                      Some(Select_LoadGame as
                                               unsafe extern "C" fn(_:
                                                                        *mut SelectContext,
                                                                    _: s32)
                                                   -> ()),
                                  entranceIndex: 0x380 as libc::c_int,};
             init
         },
         {
             let mut init =
                 SceneSelectEntry{name:
                                      b"61:\x8c\xef\xbd\xb6\xef\xbd\xb6\xef\xbe\x98\xef\xbd\xba\x8d\xef\xbe\x91\xef\xbe\x97  \xef\xbd\xb8\xef\xbd\xbd\xef\xbe\x98\xef\xbe\x94\x00"
                                          as *const u8 as *const libc::c_char
                                          as *mut libc::c_char,
                                  loadFunc:
                                      Some(Select_LoadGame as
                                               unsafe extern "C" fn(_:
                                                                        *mut SelectContext,
                                                                    _: s32)
                                                   -> ()),
                                  entranceIndex: 0x384 as libc::c_int,};
             init
         },
         {
             let mut init =
                 SceneSelectEntry{name:
                                      b"62:\x8d\xef\xbd\xbc\xef\xbe\x9e\xef\xbd\xae\xef\xbd\xb3\xef\xbd\xb6\xef\xbe\x8f\xef\xbe\x81 \xef\xbd\xb8\xef\xbd\xbd\xef\xbe\x98\xef\xbe\x94\x00"
                                          as *const u8 as *const libc::c_char
                                          as *mut libc::c_char,
                                  loadFunc:
                                      Some(Select_LoadGame as
                                               unsafe extern "C" fn(_:
                                                                        *mut SelectContext,
                                                                    _: s32)
                                                   -> ()),
                                  entranceIndex: 0x388 as libc::c_int,};
             init
         },
         {
             let mut init =
                 SceneSelectEntry{name:
                                      b"63:\x8d\xef\xbd\xb3\xef\xbe\x97\xef\xbe\x9b\xef\xbd\xbc\xef\xbe\x9e \xef\xbe\x96\xef\xbe\x99\xef\xbe\x89\xef\xbe\x90\xef\xbd\xbe\x00"
                                          as *const u8 as *const libc::c_char
                                          as *mut libc::c_char,
                                  loadFunc:
                                      Some(Select_LoadGame as
                                               unsafe extern "C" fn(_:
                                                                        *mut SelectContext,
                                                                    _: s32)
                                                   -> ()),
                                  entranceIndex: 0x390 as libc::c_int,};
             init
         },
         {
             let mut init =
                 SceneSelectEntry{name:
                                      b"64:\x8d\xef\xbd\xb5\xef\xbe\x92\xef\xbe\x9d\xef\xbe\x94\x00"
                                          as *const u8 as *const libc::c_char
                                          as *mut libc::c_char,
                                  loadFunc:
                                      Some(Select_LoadGame as
                                               unsafe extern "C" fn(_:
                                                                        *mut SelectContext,
                                                                    _: s32)
                                                   -> ()),
                                  entranceIndex: 0x530 as libc::c_int,};
             init
         },
         {
             let mut init =
                 SceneSelectEntry{name:
                                      b"65:\x8c\xef\xbd\xb9\xef\xbe\x9e\xef\xbe\x99\xef\xbe\x84\xef\xbe\x9e\x8d\xef\xbe\x89\xef\xbd\xbc\xef\xbd\xad\xef\xbd\xb3\xef\xbe\x9a\xef\xbe\x9d\xef\xbd\xbc\xef\xbe\x9e\xef\xbd\xae\xef\xbd\xb3\x00"
                                          as *const u8 as *const libc::c_char
                                          as *mut libc::c_char,
                                  loadFunc:
                                      Some(Select_LoadGame as
                                               unsafe extern "C" fn(_:
                                                                        *mut SelectContext,
                                                                    _: s32)
                                                   -> ()),
                                  entranceIndex: 0x8 as libc::c_int,};
             init
         },
         {
             let mut init =
                 SceneSelectEntry{name:
                                      b"66:\x8d\xef\xbe\x96\xef\xbd\xb3\xef\xbd\xbe\xef\xbd\xb2\xef\xbe\x89\xef\xbd\xb7\xef\xbe\x89 \x8c\xef\xbe\x80\xef\xbe\x9e\xef\xbe\x9d\xef\xbd\xbc\xef\xbe\x9e\xef\xbd\xae\xef\xbe\x9d\x00"
                                          as *const u8 as *const libc::c_char
                                          as *mut libc::c_char,
                                  loadFunc:
                                      Some(Select_LoadGame as
                                               unsafe extern "C" fn(_:
                                                                        *mut SelectContext,
                                                                    _: s32)
                                                   -> ()),
                                  entranceIndex: 0 as libc::c_int,};
             init
         },
         {
             let mut init =
                 SceneSelectEntry{name:
                                      b"67:\x8d\xef\xbe\x96\xef\xbd\xb3\xef\xbd\xbe\xef\xbd\xb2\xef\xbe\x89\xef\xbd\xb7\xef\xbe\x89 \x8c\xef\xbe\x80\xef\xbe\x9e\xef\xbe\x9d\xef\xbd\xbc\xef\xbe\x9e\xef\xbd\xae\xef\xbe\x9d \xef\xbe\x8e\xef\xbe\x9e\xef\xbd\xbd\x00"
                                          as *const u8 as *const libc::c_char
                                          as *mut libc::c_char,
                                  loadFunc:
                                      Some(Select_LoadGame as
                                               unsafe extern "C" fn(_:
                                                                        *mut SelectContext,
                                                                    _: s32)
                                                   -> ()),
                                  entranceIndex: 0x40f as libc::c_int,};
             init
         },
         {
             let mut init =
                 SceneSelectEntry{name:
                                      b"68:\x8c\xef\xbe\x84\xef\xbe\x9e\xef\xbe\x84\xef\xbe\x9e\xef\xbe\x9d\xef\xbd\xba\xef\xbe\x9e \xef\xbe\x80\xef\xbe\x9e\xef\xbe\x9d\xef\xbd\xbc\xef\xbe\x9e\xef\xbd\xae\xef\xbe\x9d\x00"
                                          as *const u8 as *const libc::c_char
                                          as *mut libc::c_char,
                                  loadFunc:
                                      Some(Select_LoadGame as
                                               unsafe extern "C" fn(_:
                                                                        *mut SelectContext,
                                                                    _: s32)
                                                   -> ()),
                                  entranceIndex: 0x4 as libc::c_int,};
             init
         },
         {
             let mut init =
                 SceneSelectEntry{name:
                                      b"69:\x8c\xef\xbe\x84\xef\xbe\x9e\xef\xbe\x84\xef\xbe\x9e\xef\xbe\x9d\xef\xbd\xba\xef\xbe\x9e \xef\xbe\x80\xef\xbe\x9e\xef\xbe\x9d\xef\xbd\xbc\xef\xbe\x9e\xef\xbd\xae\xef\xbe\x9d \xef\xbe\x8e\xef\xbe\x9e\xef\xbd\xbd\x00"
                                          as *const u8 as *const libc::c_char
                                          as *mut libc::c_char,
                                  loadFunc:
                                      Some(Select_LoadGame as
                                               unsafe extern "C" fn(_:
                                                                        *mut SelectContext,
                                                                    _: s32)
                                                   -> ()),
                                  entranceIndex: 0x40b as libc::c_int,};
             init
         },
         {
             let mut init =
                 SceneSelectEntry{name:
                                      b"70:\x8d\xef\xbd\xb7\xef\xbd\xae\xef\xbe\x80\xef\xbe\x9e\xef\xbd\xb2\xef\xbd\xb7\xef\xbe\x9e\xef\xbd\xae \x8c\xef\xbe\x80\xef\xbe\x9e\xef\xbe\x9d\xef\xbd\xbc\xef\xbe\x9e\xef\xbd\xae\xef\xbe\x9d\x00"
                                          as *const u8 as *const libc::c_char
                                          as *mut libc::c_char,
                                  loadFunc:
                                      Some(Select_LoadGame as
                                               unsafe extern "C" fn(_:
                                                                        *mut SelectContext,
                                                                    _: s32)
                                                   -> ()),
                                  entranceIndex: 0x28 as libc::c_int,};
             init
         },
         {
             let mut init =
                 SceneSelectEntry{name:
                                      b"71:\x8d\xef\xbd\xb7\xef\xbd\xae\xef\xbe\x80\xef\xbe\x9e\xef\xbd\xb2\xef\xbd\xb7\xef\xbe\x9e\xef\xbd\xae \x8c\xef\xbe\x80\xef\xbe\x9e\xef\xbe\x9d\xef\xbd\xbc\xef\xbe\x9e\xef\xbd\xae\xef\xbe\x9d \xef\xbe\x8e\xef\xbe\x9e\xef\xbd\xbd\x00"
                                          as *const u8 as *const libc::c_char
                                          as *mut libc::c_char,
                                  loadFunc:
                                      Some(Select_LoadGame as
                                               unsafe extern "C" fn(_:
                                                                        *mut SelectContext,
                                                                    _: s32)
                                                   -> ()),
                                  entranceIndex: 0x301 as libc::c_int,};
             init
         },
         {
             let mut init =
                 SceneSelectEntry{name:
                                      b"72:\x8d\xef\xbe\x93\xef\xbe\x98\xef\xbe\x89\xef\xbd\xbc\xef\xbe\x9d\xef\xbe\x83\xef\xbe\x9e\xef\xbe\x9d\x00"
                                          as *const u8 as *const libc::c_char
                                          as *mut libc::c_char,
                                  loadFunc:
                                      Some(Select_LoadGame as
                                               unsafe extern "C" fn(_:
                                                                        *mut SelectContext,
                                                                    _: s32)
                                                   -> ()),
                                  entranceIndex: 0x169 as libc::c_int,};
             init
         },
         {
             let mut init =
                 SceneSelectEntry{name:
                                      b"73:\x8d\xef\xbe\x93\xef\xbe\x98\xef\xbe\x89\xef\xbd\xbc\xef\xbe\x9d\xef\xbe\x83\xef\xbe\x9e\xef\xbe\x9d \x8c\xef\xbe\x8e\xef\xbe\x9e\xef\xbd\xbd\x00"
                                          as *const u8 as *const libc::c_char
                                          as *mut libc::c_char,
                                  loadFunc:
                                      Some(Select_LoadGame as
                                               unsafe extern "C" fn(_:
                                                                        *mut SelectContext,
                                                                    _: s32)
                                                   -> ()),
                                  entranceIndex: 0xc as libc::c_int,};
             init
         },
         {
             let mut init =
                 SceneSelectEntry{name:
                                      b"74:\x8d\xef\xbd\xb2\xef\xbe\x84\xef\xbe\x9e\xef\xbd\xbc\xef\xbe\x80 \x8c\xef\xbe\x80\xef\xbe\x9e\xef\xbe\x9d\xef\xbd\xbc\xef\xbe\x9e\xef\xbd\xae\xef\xbe\x9d\x00"
                                          as *const u8 as *const libc::c_char
                                          as *mut libc::c_char,
                                  loadFunc:
                                      Some(Select_LoadGame as
                                               unsafe extern "C" fn(_:
                                                                        *mut SelectContext,
                                                                    _: s32)
                                                   -> ()),
                                  entranceIndex: 0x98 as libc::c_int,};
             init
         },
         {
             let mut init =
                 SceneSelectEntry{name:
                                      b"75:\x8d\xef\xbe\x8a\xef\xbd\xb6\xef\xbd\xbc\xef\xbe\x80 \x8c\xef\xbe\x80\xef\xbe\x9e\xef\xbe\x9d\xef\xbd\xbc\xef\xbe\x9e\xef\xbd\xae\xef\xbe\x9d\x00"
                                          as *const u8 as *const libc::c_char
                                          as *mut libc::c_char,
                                  loadFunc:
                                      Some(Select_LoadGame as
                                               unsafe extern "C" fn(_:
                                                                        *mut SelectContext,
                                                                    _: s32)
                                                   -> ()),
                                  entranceIndex: 0x37 as libc::c_int,};
             init
         },
         {
             let mut init =
                 SceneSelectEntry{name:
                                      b"76:\x8d\xef\xbe\x8a\xef\xbd\xb6\xef\xbd\xbc\xef\xbe\x80 \x8c\xef\xbe\x80\xef\xbe\x9e\xef\xbe\x9d\xef\xbd\xbc\xef\xbe\x9e\xef\xbd\xae\xef\xbe\x9d \xef\xbe\x8e\xef\xbe\x9e\xef\xbd\xbd\x00"
                                          as *const u8 as *const libc::c_char
                                          as *mut libc::c_char,
                                  loadFunc:
                                      Some(Select_LoadGame as
                                               unsafe extern "C" fn(_:
                                                                        *mut SelectContext,
                                                                    _: s32)
                                                   -> ()),
                                  entranceIndex: 0x413 as libc::c_int,};
             init
         },
         {
             let mut init =
                 SceneSelectEntry{name:
                                      b"77:\x8d\xef\xbe\x8b\xef\xbe\x89\xef\xbd\xbc\xef\xbe\x9d\xef\xbe\x83\xef\xbe\x9e\xef\xbe\x9d\x00"
                                          as *const u8 as *const libc::c_char
                                          as *mut libc::c_char,
                                  loadFunc:
                                      Some(Select_LoadGame as
                                               unsafe extern "C" fn(_:
                                                                        *mut SelectContext,
                                                                    _: s32)
                                                   -> ()),
                                  entranceIndex: 0x165 as libc::c_int,};
             init
         },
         {
             let mut init =
                 SceneSelectEntry{name:
                                      b"78:\x8d\xef\xbe\x8b\xef\xbe\x89\xef\xbd\xbc\xef\xbe\x9d\xef\xbe\x83\xef\xbe\x9e\xef\xbe\x9d \x8c\xef\xbe\x8e\xef\xbe\x9e\xef\xbd\xbd\x00"
                                          as *const u8 as *const libc::c_char
                                          as *mut libc::c_char,
                                  loadFunc:
                                      Some(Select_LoadGame as
                                               unsafe extern "C" fn(_:
                                                                        *mut SelectContext,
                                                                    _: s32)
                                                   -> ()),
                                  entranceIndex: 0x305 as libc::c_int,};
             init
         },
         {
             let mut init =
                 SceneSelectEntry{name:
                                      b"79:\x8d\xef\xbe\x90\xef\xbd\xbd\xef\xbe\x9e\xef\xbe\x89\xef\xbd\xbc\xef\xbe\x9d\xef\xbe\x83\xef\xbe\x9e\xef\xbe\x9d\x00"
                                          as *const u8 as *const libc::c_char
                                          as *mut libc::c_char,
                                  loadFunc:
                                      Some(Select_LoadGame as
                                               unsafe extern "C" fn(_:
                                                                        *mut SelectContext,
                                                                    _: s32)
                                                   -> ()),
                                  entranceIndex: 0x10 as libc::c_int,};
             init
         },
         {
             let mut init =
                 SceneSelectEntry{name:
                                      b"80:\x8d\xef\xbe\x90\xef\xbd\xbd\xef\xbe\x9e\xef\xbe\x89\xef\xbd\xbc\xef\xbe\x9d\xef\xbe\x83\xef\xbe\x9e\xef\xbe\x9d \x8c\xef\xbe\x8e\xef\xbe\x9e\xef\xbd\xbd\x00"
                                          as *const u8 as *const libc::c_char
                                          as *mut libc::c_char,
                                  loadFunc:
                                      Some(Select_LoadGame as
                                               unsafe extern "C" fn(_:
                                                                        *mut SelectContext,
                                                                    _: s32)
                                                   -> ()),
                                  entranceIndex: 0x417 as libc::c_int,};
             init
         },
         {
             let mut init =
                 SceneSelectEntry{name:
                                      b"81:\x8d\xef\xbd\xbc\xef\xbe\x9e\xef\xbd\xac\xef\xbd\xbc\xef\xbe\x9d\xef\xbd\xbf\xef\xbe\x9e\xef\xbd\xb3 \x8c\xef\xbe\x80\xef\xbe\x9e\xef\xbe\x9d\xef\xbd\xbc\xef\xbe\x9e\xef\xbd\xae\xef\xbe\x9d\x00"
                                          as *const u8 as *const libc::c_char
                                          as *mut libc::c_char,
                                  loadFunc:
                                      Some(Select_LoadGame as
                                               unsafe extern "C" fn(_:
                                                                        *mut SelectContext,
                                                                    _: s32)
                                                   -> ()),
                                  entranceIndex: 0x82 as libc::c_int,};
             init
         },
         {
             let mut init =
                 SceneSelectEntry{name:
                                      b"82:\x8d\xef\xbd\xbc\xef\xbe\x9e\xef\xbd\xac\xef\xbd\xbc\xef\xbe\x9d\xef\xbd\xbf\xef\xbe\x9e\xef\xbd\xb3 \x8c\xef\xbe\x80\xef\xbe\x9e\xef\xbe\x9d\xef\xbd\xbc\xef\xbe\x9e\xef\xbd\xae\xef\xbe\x9d \xef\xbd\xb1\xef\xbd\xb2\xef\xbd\xb1\xef\xbe\x9d\xef\xbe\x85\xef\xbd\xaf\xef\xbd\xb8\x00"
                                          as *const u8 as *const libc::c_char
                                          as *mut libc::c_char,
                                  loadFunc:
                                      Some(Select_LoadGame as
                                               unsafe extern "C" fn(_:
                                                                        *mut SelectContext,
                                                                    _: s32)
                                                   -> ()),
                                  entranceIndex: 0x8d as libc::c_int,};
             init
         },
         {
             let mut init =
                 SceneSelectEntry{name:
                                      b"83:\x8d\xef\xbd\xbc\xef\xbe\x9e\xef\xbd\xac\xef\xbd\xbc\xef\xbe\x9d\xef\xbd\xbf\xef\xbe\x9e\xef\xbd\xb3 \x8c\xef\xbe\x80\xef\xbe\x9e\xef\xbe\x9d\xef\xbd\xbc\xef\xbe\x9e\xef\xbd\xae\xef\xbe\x9d \xef\xbe\x8e\xef\xbe\x9e\xef\xbd\xbd\x00"
                                          as *const u8 as *const libc::c_char
                                          as *mut libc::c_char,
                                  loadFunc:
                                      Some(Select_LoadGame as
                                               unsafe extern "C" fn(_:
                                                                        *mut SelectContext,
                                                                    _: s32)
                                                   -> ()),
                                  entranceIndex: 0x5ec as libc::c_int,};
             init
         },
         {
             let mut init =
                 SceneSelectEntry{name:
                                      b"84:\x8c\xef\xbd\xb6\xef\xbe\x9e\xef\xbe\x89\xef\xbe\x9d\x8d\xef\xbe\x89\xef\xbe\x84\xef\xbd\xb3\x00"
                                          as *const u8 as *const libc::c_char
                                          as *mut libc::c_char,
                                  loadFunc:
                                      Some(Select_LoadGame as
                                               unsafe extern "C" fn(_:
                                                                        *mut SelectContext,
                                                                    _: s32)
                                                   -> ()),
                                  entranceIndex: 0x41b as libc::c_int,};
             init
         },
         {
             let mut init =
                 SceneSelectEntry{name:
                                      b"85:\x8c\xef\xbd\xb6\xef\xbe\x9e\xef\xbe\x89\xef\xbe\x9d\x8d\xef\xbe\x89\xef\xbe\x84\xef\xbd\xb3\x8c\xef\xbe\x8e\xef\xbe\x9e\xef\xbd\xbd\x00"
                                          as *const u8 as *const libc::c_char
                                          as *mut libc::c_char,
                                  loadFunc:
                                      Some(Select_LoadGame as
                                               unsafe extern "C" fn(_:
                                                                        *mut SelectContext,
                                                                    _: s32)
                                                   -> ()),
                                  entranceIndex: 0x41f as libc::c_int,};
             init
         },
         {
             let mut init =
                 SceneSelectEntry{name:
                                      b"86:\x8d\xef\xbd\xba\xef\xbd\xb5\xef\xbe\x98\xef\xbe\x89\xef\xbe\x84\xef\xbe\x9e\xef\xbd\xb3\xef\xbd\xb8\xef\xbe\x82\x00"
                                          as *const u8 as *const libc::c_char
                                          as *mut libc::c_char,
                                  loadFunc:
                                      Some(Select_LoadGame as
                                               unsafe extern "C" fn(_:
                                                                        *mut SelectContext,
                                                                    _: s32)
                                                   -> ()),
                                  entranceIndex: 0x88 as libc::c_int,};
             init
         },
         {
             let mut init =
                 SceneSelectEntry{name:
                                      b"87:\x8d\xef\xbe\x8a\xef\xbd\xb6\xef\xbd\xbc\xef\xbe\x80\x8c\xef\xbe\x98\xef\xbe\x9a\xef\xbd\xb0\x00"
                                          as *const u8 as *const libc::c_char
                                          as *mut libc::c_char,
                                  loadFunc:
                                      Some(Select_LoadGame as
                                               unsafe extern "C" fn(_:
                                                                        *mut SelectContext,
                                                                    _: s32)
                                                   -> ()),
                                  entranceIndex: 0x44f as libc::c_int,};
             init
         },
         {
             let mut init =
                 SceneSelectEntry{name:
                                      b"88:\x8c\xef\xbd\xb6\xef\xbe\x9e\xef\xbe\x89\xef\xbe\x9d\x8d\xef\xbe\x81\xef\xbd\xb6 \x8c\xef\xbe\x80\xef\xbe\x9e\xef\xbe\x9d\xef\xbd\xbc\xef\xbe\x9e\xef\xbd\xae\xef\xbe\x9d\x00"
                                          as *const u8 as *const libc::c_char
                                          as *mut libc::c_char,
                                  loadFunc:
                                      Some(Select_LoadGame as
                                               unsafe extern "C" fn(_:
                                                                        *mut SelectContext,
                                                                    _: s32)
                                                   -> ()),
                                  entranceIndex: 0x467 as libc::c_int,};
             init
         },
         {
             let mut init =
                 SceneSelectEntry{name:
                                      b"89:\x8c\xef\xbd\xb6\xef\xbe\x9e\xef\xbe\x89\xef\xbe\x9d\x8d\xef\xbd\xbb\xef\xbd\xb2\xef\xbd\xbc\xef\xbd\xad\xef\xbd\xb3\xef\xbd\xbe\xef\xbe\x9d \x8c\xef\xbe\x83\xef\xbe\x9e\xef\xbe\x93 & \xef\xbe\x8a\xef\xbe\x9e\xef\xbe\x84\xef\xbe\x99\x00"
                                          as *const u8 as *const libc::c_char
                                          as *mut libc::c_char,
                                  loadFunc:
                                      Some(Select_LoadGame as
                                               unsafe extern "C" fn(_:
                                                                        *mut SelectContext,
                                                                    _: s32)
                                                   -> ()),
                                  entranceIndex: 0x517 as libc::c_int,};
             init
         },
         {
             let mut init =
                 SceneSelectEntry{name:
                                      b"90:\x8c\xef\xbd\xb6\xef\xbe\x9e\xef\xbe\x89\xef\xbe\x9d\x8d\xef\xbe\x89\xef\xbe\x84\xef\xbd\xb3 \xef\xbd\xbf\xef\xbe\x89\xef\xbd\xba\xef\xbe\x9e 1\x00"
                                          as *const u8 as *const libc::c_char
                                          as *mut libc::c_char,
                                  loadFunc:
                                      Some(Select_LoadGame as
                                               unsafe extern "C" fn(_:
                                                                        *mut SelectContext,
                                                                    _: s32)
                                                   -> ()),
                                  entranceIndex: 0x179 as libc::c_int,};
             init
         },
         {
             let mut init =
                 SceneSelectEntry{name:
                                      b"91:\x8c\xef\xbd\xb6\xef\xbe\x9e\xef\xbe\x89\xef\xbe\x9d\x8d\xef\xbe\x89\xef\xbe\x84\xef\xbd\xb3 \xef\xbd\xbf\xef\xbe\x89\xef\xbd\xba\xef\xbe\x9e 2\x00"
                                          as *const u8 as *const libc::c_char
                                          as *mut libc::c_char,
                                  loadFunc:
                                      Some(Select_LoadGame as
                                               unsafe extern "C" fn(_:
                                                                        *mut SelectContext,
                                                                    _: s32)
                                                   -> ()),
                                  entranceIndex: 0x1b5 as libc::c_int,};
             init
         },
         {
             let mut init =
                 SceneSelectEntry{name:
                                      b"92:\x8c\xef\xbd\xb6\xef\xbe\x9e\xef\xbe\x89\xef\xbe\x9d\x8d\xef\xbe\x89\xef\xbe\x84\xef\xbd\xb3 \xef\xbd\xbf\xef\xbe\x89\xef\xbd\xba\xef\xbe\x9e 3\x00"
                                          as *const u8 as *const libc::c_char
                                          as *mut libc::c_char,
                                  loadFunc:
                                      Some(Select_LoadGame as
                                               unsafe extern "C" fn(_:
                                                                        *mut SelectContext,
                                                                    _: s32)
                                                   -> ()),
                                  entranceIndex: 0x3dc as libc::c_int,};
             init
         },
         {
             let mut init =
                 SceneSelectEntry{name:
                                      b"93:\x8c\xef\xbd\xb6\xef\xbe\x9e\xef\xbe\x89\xef\xbe\x9d\x8d\xef\xbe\x89\xef\xbe\x84\xef\xbd\xb3 \xef\xbd\xbf\xef\xbe\x89\xef\xbd\xba\xef\xbe\x9e 4\x00"
                                          as *const u8 as *const libc::c_char
                                          as *mut libc::c_char,
                                  loadFunc:
                                      Some(Select_LoadGame as
                                               unsafe extern "C" fn(_:
                                                                        *mut SelectContext,
                                                                    _: s32)
                                                   -> ()),
                                  entranceIndex: 0x3e4 as libc::c_int,};
             init
         },
         {
             let mut init =
                 SceneSelectEntry{name:
                                      b"94:\x8c\xef\xbd\xb6\xef\xbe\x9e\xef\xbe\x89\xef\xbe\x9d\x8d\xef\xbe\x81\xef\xbd\xb6 \xef\xbd\xbf\xef\xbe\x89\xef\xbd\xba\xef\xbe\x9e\x00"
                                          as *const u8 as *const libc::c_char
                                          as *mut libc::c_char,
                                  loadFunc:
                                      Some(Select_LoadGame as
                                               unsafe extern "C" fn(_:
                                                                        *mut SelectContext,
                                                                    _: s32)
                                                   -> ()),
                                  entranceIndex: 0x56c as libc::c_int,};
             init
         },
         {
             let mut init =
                 SceneSelectEntry{name:
                                      b"95:\x8c\xef\xbd\xb9\xef\xbe\x9e\xef\xbe\x99\xef\xbe\x84\xef\xbe\x9e\x8d\xef\xbe\x82\xef\xbd\xb3\xef\xbe\x9b 1-2\x00"
                                          as *const u8 as *const libc::c_char
                                          as *mut libc::c_char,
                                  loadFunc:
                                      Some(Select_LoadGame as
                                               unsafe extern "C" fn(_:
                                                                        *mut SelectContext,
                                                                    _: s32)
                                                   -> ()),
                                  entranceIndex: 0x486 as libc::c_int,};
             init
         },
         {
             let mut init =
                 SceneSelectEntry{name:
                                      b"96:\x8c\xef\xbd\xb9\xef\xbe\x9e\xef\xbe\x99\xef\xbe\x84\xef\xbe\x9e\x8d\xef\xbe\x82\xef\xbd\xb3\xef\xbe\x9b 3-4 9-10\x00"
                                          as *const u8 as *const libc::c_char
                                          as *mut libc::c_char,
                                  loadFunc:
                                      Some(Select_LoadGame as
                                               unsafe extern "C" fn(_:
                                                                        *mut SelectContext,
                                                                    _: s32)
                                                   -> ()),
                                  entranceIndex: 0x48e as libc::c_int,};
             init
         },
         {
             let mut init =
                 SceneSelectEntry{name:
                                      b"97:\x8c\xef\xbd\xb9\xef\xbe\x9e\xef\xbe\x99\xef\xbe\x84\xef\xbe\x9e\x8d\xef\xbe\x82\xef\xbd\xb3\xef\xbe\x9b 5-6\x00"
                                          as *const u8 as *const libc::c_char
                                          as *mut libc::c_char,
                                  loadFunc:
                                      Some(Select_LoadGame as
                                               unsafe extern "C" fn(_:
                                                                        *mut SelectContext,
                                                                    _: s32)
                                                   -> ()),
                                  entranceIndex: 0x496 as libc::c_int,};
             init
         },
         {
             let mut init =
                 SceneSelectEntry{name:
                                      b"98:\x8c\xef\xbd\xb9\xef\xbe\x9e\xef\xbe\x99\xef\xbe\x84\xef\xbe\x9e\x8d\xef\xbe\x82\xef\xbd\xb3\xef\xbe\x9b 7-8\x00"
                                          as *const u8 as *const libc::c_char
                                          as *mut libc::c_char,
                                  loadFunc:
                                      Some(Select_LoadGame as
                                               unsafe extern "C" fn(_:
                                                                        *mut SelectContext,
                                                                    _: s32)
                                                   -> ()),
                                  entranceIndex: 0x49e as libc::c_int,};
             init
         },
         {
             let mut init =
                 SceneSelectEntry{name:
                                      b"99:\x8c\xef\xbd\xb9\xef\xbe\x9e\xef\xbe\x99\xef\xbe\x84\xef\xbe\x9e\x8d\xef\xbe\x82\xef\xbd\xb3\xef\xbe\x9b 11-12\x00"
                                          as *const u8 as *const libc::c_char
                                          as *mut libc::c_char,
                                  loadFunc:
                                      Some(Select_LoadGame as
                                               unsafe extern "C" fn(_:
                                                                        *mut SelectContext,
                                                                    _: s32)
                                                   -> ()),
                                  entranceIndex: 0x4ae as libc::c_int,};
             init
         },
         {
             let mut init =
                 SceneSelectEntry{name:
                                      b"100:\x8c\xef\xbd\xb9\xef\xbe\x9e\xef\xbe\x99\xef\xbe\x84\xef\xbe\x9e\x8d\xef\xbe\x82\xef\xbd\xb3\xef\xbe\x9b 13\x00"
                                          as *const u8 as *const libc::c_char
                                          as *mut libc::c_char,
                                  loadFunc:
                                      Some(Select_LoadGame as
                                               unsafe extern "C" fn(_:
                                                                        *mut SelectContext,
                                                                    _: s32)
                                                   -> ()),
                                  entranceIndex: 0x570 as libc::c_int,};
             init
         },
         {
             let mut init =
                 SceneSelectEntry{name:
                                      b"101:\x8d\xef\xbd\xb6\xef\xbd\xb8\xef\xbd\xbc\xef\xbe\x84\xef\xbe\x8b\xef\xbe\x9e\xef\xbd\xba\xef\xbe\x90\xef\xbd\xb1\xef\xbe\x85 0\x00"
                                          as *const u8 as *const libc::c_char
                                          as *mut libc::c_char,
                                  loadFunc:
                                      Some(Select_LoadGame as
                                               unsafe extern "C" fn(_:
                                                                        *mut SelectContext,
                                                                    _: s32)
                                                   -> ()),
                                  entranceIndex: 0x3f as libc::c_int,};
             init
         },
         {
             let mut init =
                 SceneSelectEntry{name:
                                      b"102:\x8d\xef\xbd\xb6\xef\xbd\xb8\xef\xbd\xbc\xef\xbe\x84\xef\xbe\x8b\xef\xbe\x9e\xef\xbd\xba\xef\xbe\x90\xef\xbd\xb1\xef\xbe\x85 1\x00"
                                          as *const u8 as *const libc::c_char
                                          as *mut libc::c_char,
                                  loadFunc:
                                      Some(Select_LoadGame as
                                               unsafe extern "C" fn(_:
                                                                        *mut SelectContext,
                                                                    _: s32)
                                                   -> ()),
                                  entranceIndex: 0x598 as libc::c_int,};
             init
         },
         {
             let mut init =
                 SceneSelectEntry{name:
                                      b"103:\x8d\xef\xbd\xb6\xef\xbd\xb8\xef\xbd\xbc\xef\xbe\x84\xef\xbe\x8b\xef\xbe\x9e\xef\xbd\xba\xef\xbe\x90\xef\xbd\xb1\xef\xbe\x85 2\x00"
                                          as *const u8 as *const libc::c_char
                                          as *mut libc::c_char,
                                  loadFunc:
                                      Some(Select_LoadGame as
                                               unsafe extern "C" fn(_:
                                                                        *mut SelectContext,
                                                                    _: s32)
                                                   -> ()),
                                  entranceIndex: 0x59c as libc::c_int,};
             init
         },
         {
             let mut init =
                 SceneSelectEntry{name:
                                      b"104:\x8d\xef\xbd\xb6\xef\xbd\xb8\xef\xbd\xbc\xef\xbe\x84\xef\xbe\x8b\xef\xbe\x9e\xef\xbd\xba\xef\xbe\x90\xef\xbd\xb1\xef\xbe\x85 3\x00"
                                          as *const u8 as *const libc::c_char
                                          as *mut libc::c_char,
                                  loadFunc:
                                      Some(Select_LoadGame as
                                               unsafe extern "C" fn(_:
                                                                        *mut SelectContext,
                                                                    _: s32)
                                                   -> ()),
                                  entranceIndex: 0x5a0 as libc::c_int,};
             init
         },
         {
             let mut init =
                 SceneSelectEntry{name:
                                      b"105:\x8d\xef\xbd\xb6\xef\xbd\xb8\xef\xbd\xbc\xef\xbe\x84\xef\xbe\x8b\xef\xbe\x9e\xef\xbd\xba\xef\xbe\x90\xef\xbd\xb1\xef\xbe\x85 4\x00"
                                          as *const u8 as *const libc::c_char
                                          as *mut libc::c_char,
                                  loadFunc:
                                      Some(Select_LoadGame as
                                               unsafe extern "C" fn(_:
                                                                        *mut SelectContext,
                                                                    _: s32)
                                                   -> ()),
                                  entranceIndex: 0x5a4 as libc::c_int,};
             init
         },
         {
             let mut init =
                 SceneSelectEntry{name:
                                      b"106:\x8d\xef\xbd\xb6\xef\xbd\xb8\xef\xbd\xbc\xef\xbe\x84\xef\xbe\x8b\xef\xbe\x9e\xef\xbd\xba\xef\xbe\x90\xef\xbd\xb1\xef\xbe\x85 5\x00"
                                          as *const u8 as *const libc::c_char
                                          as *mut libc::c_char,
                                  loadFunc:
                                      Some(Select_LoadGame as
                                               unsafe extern "C" fn(_:
                                                                        *mut SelectContext,
                                                                    _: s32)
                                                   -> ()),
                                  entranceIndex: 0x5a8 as libc::c_int,};
             init
         },
         {
             let mut init =
                 SceneSelectEntry{name:
                                      b"107:\x8d\xef\xbd\xb6\xef\xbd\xb8\xef\xbd\xbc\xef\xbe\x84\xef\xbe\x8b\xef\xbe\x9e\xef\xbd\xba\xef\xbe\x90\xef\xbd\xb1\xef\xbe\x85 6\x00"
                                          as *const u8 as *const libc::c_char
                                          as *mut libc::c_char,
                                  loadFunc:
                                      Some(Select_LoadGame as
                                               unsafe extern "C" fn(_:
                                                                        *mut SelectContext,
                                                                    _: s32)
                                                   -> ()),
                                  entranceIndex: 0x5ac as libc::c_int,};
             init
         },
         {
             let mut init =
                 SceneSelectEntry{name:
                                      b"108:\x8d\xef\xbd\xb6\xef\xbd\xb8\xef\xbd\xbc\xef\xbe\x84\xef\xbe\x8b\xef\xbe\x9e\xef\xbd\xba\xef\xbe\x90\xef\xbd\xb1\xef\xbe\x85 7\x00"
                                          as *const u8 as *const libc::c_char
                                          as *mut libc::c_char,
                                  loadFunc:
                                      Some(Select_LoadGame as
                                               unsafe extern "C" fn(_:
                                                                        *mut SelectContext,
                                                                    _: s32)
                                                   -> ()),
                                  entranceIndex: 0x5b0 as libc::c_int,};
             init
         },
         {
             let mut init =
                 SceneSelectEntry{name:
                                      b"109:\x8d\xef\xbd\xb6\xef\xbd\xb8\xef\xbd\xbc\xef\xbe\x84\xef\xbe\x8b\xef\xbe\x9e\xef\xbd\xba\xef\xbe\x90\xef\xbd\xb1\xef\xbe\x85 8\x00"
                                          as *const u8 as *const libc::c_char
                                          as *mut libc::c_char,
                                  loadFunc:
                                      Some(Select_LoadGame as
                                               unsafe extern "C" fn(_:
                                                                        *mut SelectContext,
                                                                    _: s32)
                                                   -> ()),
                                  entranceIndex: 0x5b4 as libc::c_int,};
             init
         },
         {
             let mut init =
                 SceneSelectEntry{name:
                                      b"110:\x8d\xef\xbd\xb6\xef\xbd\xb8\xef\xbd\xbc\xef\xbe\x84\xef\xbe\x8b\xef\xbe\x9e\xef\xbd\xba\xef\xbe\x90\xef\xbd\xb1\xef\xbe\x85 9\x00"
                                          as *const u8 as *const libc::c_char
                                          as *mut libc::c_char,
                                  loadFunc:
                                      Some(Select_LoadGame as
                                               unsafe extern "C" fn(_:
                                                                        *mut SelectContext,
                                                                    _: s32)
                                                   -> ()),
                                  entranceIndex: 0x5b8 as libc::c_int,};
             init
         },
         {
             let mut init =
                 SceneSelectEntry{name:
                                      b"111:\x8d\xef\xbd\xb6\xef\xbd\xb8\xef\xbd\xbc\xef\xbe\x84\xef\xbe\x8b\xef\xbe\x9e\xef\xbd\xba\xef\xbe\x90\xef\xbd\xb1\xef\xbe\x85 10\x00"
                                          as *const u8 as *const libc::c_char
                                          as *mut libc::c_char,
                                  loadFunc:
                                      Some(Select_LoadGame as
                                               unsafe extern "C" fn(_:
                                                                        *mut SelectContext,
                                                                    _: s32)
                                                   -> ()),
                                  entranceIndex: 0x5bc as libc::c_int,};
             init
         },
         {
             let mut init =
                 SceneSelectEntry{name:
                                      b"112:\x8d\xef\xbd\xb6\xef\xbd\xb8\xef\xbd\xbc\xef\xbe\x84\xef\xbe\x8b\xef\xbe\x9e\xef\xbd\xba\xef\xbe\x90\xef\xbd\xb1\xef\xbe\x85 11\x00"
                                          as *const u8 as *const libc::c_char
                                          as *mut libc::c_char,
                                  loadFunc:
                                      Some(Select_LoadGame as
                                               unsafe extern "C" fn(_:
                                                                        *mut SelectContext,
                                                                    _: s32)
                                                   -> ()),
                                  entranceIndex: 0x5c0 as libc::c_int,};
             init
         },
         {
             let mut init =
                 SceneSelectEntry{name:
                                      b"113:\x8d\xef\xbd\xb6\xef\xbd\xb8\xef\xbd\xbc\xef\xbe\x84\xef\xbe\x8b\xef\xbe\x9e\xef\xbd\xba\xef\xbe\x90\xef\xbd\xb1\xef\xbe\x85 12\x00"
                                          as *const u8 as *const libc::c_char
                                          as *mut libc::c_char,
                                  loadFunc:
                                      Some(Select_LoadGame as
                                               unsafe extern "C" fn(_:
                                                                        *mut SelectContext,
                                                                    _: s32)
                                                   -> ()),
                                  entranceIndex: 0x5c4 as libc::c_int,};
             init
         },
         {
             let mut init =
                 SceneSelectEntry{name:
                                      b"114:\x8d\xef\xbd\xb6\xef\xbd\xb8\xef\xbd\xbc\xef\xbe\x84\xef\xbe\x8b\xef\xbe\x9e\xef\xbd\xba\xef\xbe\x90\xef\xbd\xb1\xef\xbe\x85 13\x00"
                                          as *const u8 as *const libc::c_char
                                          as *mut libc::c_char,
                                  loadFunc:
                                      Some(Select_LoadGame as
                                               unsafe extern "C" fn(_:
                                                                        *mut SelectContext,
                                                                    _: s32)
                                                   -> ()),
                                  entranceIndex: 0x5fc as libc::c_int,};
             init
         },
         {
             let mut init =
                 SceneSelectEntry{name:
                                      b"115:\x8c\xef\xbe\x8a\xef\xbd\xb2\xef\xbe\x97\xef\xbe\x99 \xef\xbe\x83\xef\xbe\x9e\xef\xbe\x93\x00"
                                          as *const u8 as *const libc::c_char
                                          as *mut libc::c_char,
                                  loadFunc:
                                      Some(Select_LoadGame as
                                               unsafe extern "C" fn(_:
                                                                        *mut SelectContext,
                                                                    _: s32)
                                                   -> ()),
                                  entranceIndex: 0xa0 as libc::c_int,};
             init
         },
         {
             let mut init =
                 SceneSelectEntry{name:
                                      b"116:\x8d\xef\xbe\x8d\xef\xbe\x9e\xef\xbd\xaf\xef\xbd\xbc\xef\xbe\x82 (\xef\xbe\x80\xef\xbd\xb6\xef\xbe\x97\xef\xbe\x8a\xef\xbe\x9e\xef\xbd\xba\x8c\xef\xbe\x9c\xef\xbd\xb0\xef\xbe\x8c\xef\xbe\x9f)\x00"
                                          as *const u8 as *const libc::c_char
                                          as *mut libc::c_char,
                                  loadFunc:
                                      Some(Select_LoadGame as
                                               unsafe extern "C" fn(_:
                                                                        *mut SelectContext,
                                                                    _: s32)
                                                   -> ()),
                                  entranceIndex: 0x520 as libc::c_int,};
             init
         },
         {
             let mut init =
                 SceneSelectEntry{name:
                                      b"117:\x8d\xef\xbd\xbb\xef\xbd\xbb\x8c\xef\xbe\x83\xef\xbd\xbd\xef\xbe\x84\x00"
                                          as *const u8 as *const libc::c_char
                                          as *mut libc::c_char,
                                  loadFunc:
                                      Some(Select_LoadGame as
                                               unsafe extern "C" fn(_:
                                                                        *mut SelectContext,
                                                                    _: s32)
                                                   -> ()),
                                  entranceIndex: 0x18 as libc::c_int,};
             init
         },
         {
             let mut init =
                 SceneSelectEntry{name:
                                      b"118:\x8c\xef\xbe\x83\xef\xbd\xbd\xef\xbe\x84\xef\xbe\x8f\xef\xbd\xaf\xef\xbe\x8c\xef\xbe\x9f\x00"
                                          as *const u8 as *const libc::c_char
                                          as *mut libc::c_char,
                                  loadFunc:
                                      Some(Select_LoadGame as
                                               unsafe extern "C" fn(_:
                                                                        *mut SelectContext,
                                                                    _: s32)
                                                   -> ()),
                                  entranceIndex: 0x94 as libc::c_int,};
             init
         },
         {
             let mut init =
                 SceneSelectEntry{name:
                                      b"119:\x8c\xef\xbe\x83\xef\xbd\xbd\xef\xbe\x84\xef\xbe\x99\xef\xbd\xb0\xef\xbe\x91\x00"
                                          as *const u8 as *const libc::c_char
                                          as *mut libc::c_char,
                                  loadFunc:
                                      Some(Select_LoadGame as
                                               unsafe extern "C" fn(_:
                                                                        *mut SelectContext,
                                                                    _: s32)
                                                   -> ()),
                                  entranceIndex: 0x24 as libc::c_int,};
             init
         },
         {
             let mut init =
                 SceneSelectEntry{name:
                                      b"120:\x8d\xef\xbe\x81\xef\xbd\xad\xef\xbd\xb3\x8c\xef\xbd\xbd\xef\xbe\x80\xef\xbe\x9b\xef\xbe\x8c\xef\xbd\xab\xef\xbd\xbd\x8d\xef\xbe\x8d\xef\xbe\x9e\xef\xbe\x94\x00"
                                          as *const u8 as *const libc::c_char
                                          as *mut libc::c_char,
                                  loadFunc:
                                      Some(Select_LoadGame as
                                               unsafe extern "C" fn(_:
                                                                        *mut SelectContext,
                                                                    _: s32)
                                                   -> ()),
                                  entranceIndex: 0x1c as libc::c_int,};
             init
         },
         {
             let mut init =
                 SceneSelectEntry{name:
                                      b"121:\x8c\xef\xbe\x8e\xef\xbe\x9e\xef\xbd\xbd\xef\xbd\xbd\xef\xbe\x80\xef\xbe\x9b\xef\xbe\x8c\xef\xbd\xab\xef\xbd\xbd\x8d\xef\xbe\x8d\xef\xbe\x9e\xef\xbe\x94\x00"
                                          as *const u8 as *const libc::c_char
                                          as *mut libc::c_char,
                                  loadFunc:
                                      Some(Select_LoadGame as
                                               unsafe extern "C" fn(_:
                                                                        *mut SelectContext,
                                                                    _: s32)
                                                   -> ()),
                                  entranceIndex: 0x20 as libc::c_int,};
             init
         },
         {
             let mut init =
                 SceneSelectEntry{name:
                                      b"122:Sutaru\x00" as *const u8 as
                                          *const libc::c_char as
                                          *mut libc::c_char,
                                  loadFunc:
                                      Some(Select_LoadGame as
                                               unsafe extern "C" fn(_:
                                                                        *mut SelectContext,
                                                                    _: s32)
                                                   -> ()),
                                  entranceIndex: 0x47 as libc::c_int,};
             init
         },
         {
             let mut init =
                 SceneSelectEntry{name:
                                      b"123:jikkenjyou\x00" as *const u8 as
                                          *const libc::c_char as
                                          *mut libc::c_char,
                                  loadFunc:
                                      Some(Select_LoadGame as
                                               unsafe extern "C" fn(_:
                                                                        *mut SelectContext,
                                                                    _: s32)
                                                   -> ()),
                                  entranceIndex: 0x2ea as libc::c_int,};
             init
         },
         {
             let mut init =
                 SceneSelectEntry{name:
                                      b"124:depth\x8c\xef\xbe\x83\xef\xbd\xbd\xef\xbe\x84\x00"
                                          as *const u8 as *const libc::c_char
                                          as *mut libc::c_char,
                                  loadFunc:
                                      Some(Select_LoadGame as
                                               unsafe extern "C" fn(_:
                                                                        *mut SelectContext,
                                                                    _: s32)
                                                   -> ()),
                                  entranceIndex: 0xb6 as libc::c_int,};
             init
         },
         {
             let mut init =
                 SceneSelectEntry{name:
                                      b"125:\x8c\xef\xbe\x8a\xef\xbd\xb2\xef\xbe\x97\xef\xbe\x99\x8d\xef\xbe\x86\xef\xbe\x9c\x8c\xef\xbd\xb9\xef\xbe\x9e\xef\xbd\xb0\xef\xbe\x912\x00"
                                          as *const u8 as *const libc::c_char
                                          as *mut libc::c_char,
                                  loadFunc:
                                      Some(Select_LoadGame as
                                               unsafe extern "C" fn(_:
                                                                        *mut SelectContext,
                                                                    _: s32)
                                                   -> ()),
                                  entranceIndex: 0x76 as libc::c_int,};
             init
         },
         {
             let mut init =
                 SceneSelectEntry{name:
                                      b"title\x00" as *const u8 as
                                          *const libc::c_char as
                                          *mut libc::c_char,
                                  loadFunc:
                                      ::std::mem::transmute::<*mut libc::c_void,
                                                              Option<unsafe extern "C" fn(_:
                                                                                              *mut SelectContext,
                                                                                          _:
                                                                                              s32)
                                                                         ->
                                                                             ()>>(::std::mem::transmute::<Option<unsafe extern "C" fn(_:
                                                                                                                                          *mut SelectContext)
                                                                                                                     ->
                                                                                                                         ()>,
                                                                                                          *mut libc::c_void>(Some(Select_LoadTitle
                                                                                                                                      as
                                                                                                                                      unsafe extern "C" fn(_:
                                                                                                                                                               *mut SelectContext)
                                                                                                                                          ->
                                                                                                                                              ()))),
                                  entranceIndex: 0 as libc::c_int,};
             init
         }]
    };
#[no_mangle]
pub unsafe extern "C" fn Select_UpdateMenu(mut this: *mut SelectContext) {
    let mut input: *mut Input =
        &mut *(*this).state.input.as_mut_ptr().offset(0 as libc::c_int as
                                                          isize) as
            *mut Input;
    let mut pad: s32 = 0;
    let mut selectedScene: *mut SceneSelectEntry = 0 as *mut SceneSelectEntry;
    if (*this).verticalInputAccumulator == 0 as libc::c_int {
        if !((*input).press.button as libc::c_int | !(0x8000 as libc::c_int))
               == 0 as libc::c_int ||
               !((*input).press.button as libc::c_int |
                     !(0x1000 as libc::c_int)) == 0 as libc::c_int {
            selectedScene =
                &mut *(*this).scenes.offset((*this).currentScene as isize) as
                    *mut SceneSelectEntry;
            if (*selectedScene).loadFunc.is_some() {
                (*selectedScene).loadFunc.expect("non-null function pointer")(this,
                                                                              (*selectedScene).entranceIndex);
            }
        }
        if !((*input).press.button as libc::c_int | !(0x4000 as libc::c_int))
               == 0 as libc::c_int {
            if (if !(gSaveContext.linkAge == 0 as libc::c_int) {
                    5 as libc::c_int
                } else { 17 as libc::c_int }) == 17 as libc::c_int {
                gSaveContext.linkAge = 1 as libc::c_int
            } else { gSaveContext.linkAge = 0 as libc::c_int }
        }
        if !((*input).press.button as libc::c_int | !(0x2000 as libc::c_int))
               == 0 as libc::c_int {
            if gSaveContext.cutsceneIndex == 0x8000 as libc::c_int {
                gSaveContext.cutsceneIndex = 0 as libc::c_int
            } else if gSaveContext.cutsceneIndex == 0 as libc::c_int {
                gSaveContext.cutsceneIndex = 0xfff0 as libc::c_int
            } else if gSaveContext.cutsceneIndex == 0xfff0 as libc::c_int {
                gSaveContext.cutsceneIndex = 0xfff1 as libc::c_int
            } else if gSaveContext.cutsceneIndex == 0xfff1 as libc::c_int {
                gSaveContext.cutsceneIndex = 0xfff2 as libc::c_int
            } else if gSaveContext.cutsceneIndex == 0xfff2 as libc::c_int {
                gSaveContext.cutsceneIndex = 0xfff3 as libc::c_int
            } else if gSaveContext.cutsceneIndex == 0xfff3 as libc::c_int {
                gSaveContext.cutsceneIndex = 0xfff4 as libc::c_int
            } else if gSaveContext.cutsceneIndex == 0xfff4 as libc::c_int {
                gSaveContext.cutsceneIndex = 0xfff5 as libc::c_int
            } else if gSaveContext.cutsceneIndex == 0xfff5 as libc::c_int {
                gSaveContext.cutsceneIndex = 0xfff6 as libc::c_int
            } else if gSaveContext.cutsceneIndex == 0xfff6 as libc::c_int {
                gSaveContext.cutsceneIndex = 0xfff7 as libc::c_int
            } else if gSaveContext.cutsceneIndex == 0xfff7 as libc::c_int {
                gSaveContext.cutsceneIndex = 0xfff8 as libc::c_int
            } else if gSaveContext.cutsceneIndex == 0xfff8 as libc::c_int {
                gSaveContext.cutsceneIndex = 0xfff9 as libc::c_int
            } else if gSaveContext.cutsceneIndex == 0xfff9 as libc::c_int {
                gSaveContext.cutsceneIndex = 0xfffa as libc::c_int
            } else if gSaveContext.cutsceneIndex == 0xfffa as libc::c_int {
                gSaveContext.cutsceneIndex = 0x8000 as libc::c_int
            }
        } else if !((*input).press.button as libc::c_int |
                        !(0x10 as libc::c_int)) == 0 as libc::c_int {
            if gSaveContext.cutsceneIndex == 0x8000 as libc::c_int {
                gSaveContext.cutsceneIndex = 0xfffa as libc::c_int
            } else if gSaveContext.cutsceneIndex == 0 as libc::c_int {
                gSaveContext.cutsceneIndex = 0x8000 as libc::c_int
            } else if gSaveContext.cutsceneIndex == 0xfff0 as libc::c_int {
                gSaveContext.cutsceneIndex = 0 as libc::c_int
            } else if gSaveContext.cutsceneIndex == 0xfff1 as libc::c_int {
                gSaveContext.cutsceneIndex = 0xfff0 as libc::c_int
            } else if gSaveContext.cutsceneIndex == 0xfff2 as libc::c_int {
                gSaveContext.cutsceneIndex = 0xfff1 as libc::c_int
            } else if gSaveContext.cutsceneIndex == 0xfff3 as libc::c_int {
                gSaveContext.cutsceneIndex = 0xfff2 as libc::c_int
            } else if gSaveContext.cutsceneIndex == 0xfff4 as libc::c_int {
                gSaveContext.cutsceneIndex = 0xfff3 as libc::c_int
            } else if gSaveContext.cutsceneIndex == 0xfff5 as libc::c_int {
                gSaveContext.cutsceneIndex = 0xfff4 as libc::c_int
            } else if gSaveContext.cutsceneIndex == 0xfff6 as libc::c_int {
                gSaveContext.cutsceneIndex = 0xfff5 as libc::c_int
            } else if gSaveContext.cutsceneIndex == 0xfff7 as libc::c_int {
                gSaveContext.cutsceneIndex = 0xfff6 as libc::c_int
            } else if gSaveContext.cutsceneIndex == 0xfff8 as libc::c_int {
                gSaveContext.cutsceneIndex = 0xfff7 as libc::c_int
            } else if gSaveContext.cutsceneIndex == 0xfff9 as libc::c_int {
                gSaveContext.cutsceneIndex = 0xfff8 as libc::c_int
            } else if gSaveContext.cutsceneIndex == 0xfffa as libc::c_int {
                gSaveContext.cutsceneIndex = 0xfff9 as libc::c_int
            }
        }
        gSaveContext.nightFlag = 0 as libc::c_int;
        if gSaveContext.cutsceneIndex == 0 as libc::c_int {
            gSaveContext.nightFlag = 1 as libc::c_int
        }
        // user can change "opt", but it doesn't do anything
        if !((*input).press.button as libc::c_int | !(0x8 as libc::c_int)) ==
               0 as libc::c_int {
            (*this).opt -= 1
        }
        if !((*input).press.button as libc::c_int | !(0x4 as libc::c_int)) ==
               0 as libc::c_int {
            (*this).opt += 1
        }
        if !((*input).press.button as libc::c_int | !(0x800 as libc::c_int))
               == 0 as libc::c_int {
            if (*this).lockUp == 1 as libc::c_int {
                (*this).timerUp = 0 as libc::c_int
            }
            if (*this).timerUp == 0 as libc::c_int {
                (*this).timerUp = 20 as libc::c_int;
                (*this).lockUp = 1 as libc::c_int;
                Audio_PlaySoundGeneral(0x1800 as libc::c_int as u16_0,
                                       &mut D_801333D4,
                                       4 as libc::c_int as u8_0,
                                       &mut D_801333E0, &mut D_801333E0,
                                       &mut D_801333E8);
                (*this).verticalInput =
                    (*gGameInfo).data[(1 as libc::c_int * 6 as libc::c_int *
                                           16 as libc::c_int +
                                           30 as libc::c_int) as usize] as s32
            }
        }
        if !((*input).cur.button as libc::c_int | !(0x800 as libc::c_int)) ==
               0 as libc::c_int && (*this).timerUp == 0 as libc::c_int {
            Audio_PlaySoundGeneral(0x1800 as libc::c_int as u16_0,
                                   &mut D_801333D4, 4 as libc::c_int as u8_0,
                                   &mut D_801333E0, &mut D_801333E0,
                                   &mut D_801333E8);
            (*this).verticalInput =
                (*gGameInfo).data[(1 as libc::c_int * 6 as libc::c_int *
                                       16 as libc::c_int + 30 as libc::c_int)
                                      as usize] as libc::c_int *
                    3 as libc::c_int
        }
        if !((*input).press.button as libc::c_int | !(0x400 as libc::c_int))
               == 0 as libc::c_int {
            if (*this).lockDown == 1 as libc::c_int {
                (*this).timerDown = 0 as libc::c_int
            }
            if (*this).timerDown == 0 as libc::c_int {
                (*this).timerDown = 20 as libc::c_int;
                (*this).lockDown = 1 as libc::c_int;
                Audio_PlaySoundGeneral(0x1800 as libc::c_int as u16_0,
                                       &mut D_801333D4,
                                       4 as libc::c_int as u8_0,
                                       &mut D_801333E0, &mut D_801333E0,
                                       &mut D_801333E8);
                (*this).verticalInput =
                    -((*gGameInfo).data[(1 as libc::c_int * 6 as libc::c_int *
                                             16 as libc::c_int +
                                             30 as libc::c_int) as usize] as
                          libc::c_int)
            }
        }
        if !((*input).cur.button as libc::c_int | !(0x400 as libc::c_int)) ==
               0 as libc::c_int && (*this).timerDown == 0 as libc::c_int {
            Audio_PlaySoundGeneral(0x1800 as libc::c_int as u16_0,
                                   &mut D_801333D4, 4 as libc::c_int as u8_0,
                                   &mut D_801333E0, &mut D_801333E0,
                                   &mut D_801333E8);
            (*this).verticalInput =
                -((*gGameInfo).data[(1 as libc::c_int * 6 as libc::c_int *
                                         16 as libc::c_int +
                                         30 as libc::c_int) as usize] as
                      libc::c_int) * 3 as libc::c_int
        }
        if !((*input).press.button as libc::c_int | !(0x200 as libc::c_int))
               == 0 as libc::c_int ||
               !((*input).cur.button as libc::c_int | !(0x200 as libc::c_int))
                   == 0 as libc::c_int {
            Audio_PlaySoundGeneral(0x1800 as libc::c_int as u16_0,
                                   &mut D_801333D4, 4 as libc::c_int as u8_0,
                                   &mut D_801333E0, &mut D_801333E0,
                                   &mut D_801333E8);
            (*this).verticalInput =
                (*gGameInfo).data[(1 as libc::c_int * 6 as libc::c_int *
                                       16 as libc::c_int + 30 as libc::c_int)
                                      as usize] as s32
        }
        if !((*input).press.button as libc::c_int | !(0x100 as libc::c_int))
               == 0 as libc::c_int ||
               !((*input).cur.button as libc::c_int | !(0x100 as libc::c_int))
                   == 0 as libc::c_int {
            Audio_PlaySoundGeneral(0x1800 as libc::c_int as u16_0,
                                   &mut D_801333D4, 4 as libc::c_int as u8_0,
                                   &mut D_801333E0, &mut D_801333E0,
                                   &mut D_801333E8);
            (*this).verticalInput =
                -((*gGameInfo).data[(1 as libc::c_int * 6 as libc::c_int *
                                         16 as libc::c_int +
                                         30 as libc::c_int) as usize] as
                      libc::c_int)
        }
    }
    if !((*input).press.button as libc::c_int | !(0x20 as libc::c_int)) ==
           0 as libc::c_int {
        (*this).pageDownIndex += 1;
        (*this).pageDownIndex =
            ((*this).pageDownIndex +
                 (::std::mem::size_of::<[s32; 7]>() as
                      libc::c_ulong).wrapping_div(::std::mem::size_of::<s32>()
                                                      as libc::c_ulong) as
                     s32) %
                (::std::mem::size_of::<[s32; 7]>() as
                     libc::c_ulong).wrapping_div(::std::mem::size_of::<s32>()
                                                     as libc::c_ulong) as s32;
        (*this).topDisplayedScene =
            (*this).pageDownStops[(*this).pageDownIndex as usize];
        (*this).currentScene = (*this).topDisplayedScene
    }
    (*this).verticalInputAccumulator += (*this).verticalInput;
    if (*this).verticalInputAccumulator < -(7 as libc::c_int) {
        (*this).verticalInput = 0 as libc::c_int;
        (*this).verticalInputAccumulator = 0 as libc::c_int;
        (*this).currentScene += 1;
        (*this).currentScene =
            ((*this).currentScene + (*this).count) % (*this).count;
        if (*this).currentScene ==
               ((*this).topDisplayedScene + (*this).count + 19 as libc::c_int)
                   % (*this).count {
            (*this).topDisplayedScene += 1;
            (*this).topDisplayedScene =
                ((*this).topDisplayedScene + (*this).count) % (*this).count
        }
    }
    if (*this).verticalInputAccumulator > 7 as libc::c_int {
        (*this).verticalInput = 0 as libc::c_int;
        (*this).verticalInputAccumulator = 0 as libc::c_int;
        if (*this).currentScene == (*this).topDisplayedScene {
            (*this).topDisplayedScene -= 2 as libc::c_int;
            (*this).topDisplayedScene =
                ((*this).topDisplayedScene + (*this).count) % (*this).count
        }
        (*this).currentScene -= 1;
        (*this).currentScene =
            ((*this).currentScene + (*this).count) % (*this).count;
        if (*this).currentScene ==
               ((*this).topDisplayedScene + (*this).count) % (*this).count {
            (*this).topDisplayedScene -= 1;
            (*this).topDisplayedScene =
                ((*this).topDisplayedScene + (*this).count) % (*this).count
        }
    }
    (*this).currentScene =
        ((*this).currentScene + (*this).count) % (*this).count;
    (*this).topDisplayedScene =
        ((*this).topDisplayedScene + (*this).count) % (*this).count;
    (*gGameInfo).data[(26 as libc::c_int * 6 as libc::c_int *
                           16 as libc::c_int + 80 as libc::c_int) as usize] =
        (*this).currentScene as s16;
    (*gGameInfo).data[(26 as libc::c_int * 6 as libc::c_int *
                           16 as libc::c_int + 81 as libc::c_int) as usize] =
        (*this).topDisplayedScene as s16;
    (*gGameInfo).data[(26 as libc::c_int * 6 as libc::c_int *
                           16 as libc::c_int + 82 as libc::c_int) as usize] =
        (*this).pageDownIndex as s16;
    if (*this).timerUp != 0 as libc::c_int { (*this).timerUp -= 1 }
    if (*this).timerUp == 0 as libc::c_int {
        (*this).lockUp = 0 as libc::c_int
    }
    if (*this).timerDown != 0 as libc::c_int { (*this).timerDown -= 1 }
    if (*this).timerDown == 0 as libc::c_int {
        (*this).lockDown = 0 as libc::c_int
    };
}
#[no_mangle]
pub unsafe extern "C" fn Select_PrintMenu(mut this: *mut SelectContext,
                                          mut printer: *mut GfxPrint) {
    let mut scene: s32 = 0;
    let mut i: s32 = 0;
    let mut name: *mut libc::c_char = 0 as *mut libc::c_char;
    GfxPrint_SetColor(printer, 255 as libc::c_int as u32_0,
                      155 as libc::c_int as u32_0,
                      150 as libc::c_int as u32_0,
                      255 as libc::c_int as u32_0);
    GfxPrint_SetPos(printer, 12 as libc::c_int, 2 as libc::c_int);
    GfxPrint_Printf(printer,
                    b"ZELDA MAP SELECT\x00" as *const u8 as
                        *const libc::c_char);
    GfxPrint_SetColor(printer, 255 as libc::c_int as u32_0,
                      255 as libc::c_int as u32_0,
                      255 as libc::c_int as u32_0,
                      255 as libc::c_int as u32_0);
    i = 0 as libc::c_int;
    while i < 20 as libc::c_int {
        GfxPrint_SetPos(printer, 9 as libc::c_int, i + 4 as libc::c_int);
        scene =
            ((*this).topDisplayedScene + i + (*this).count) % (*this).count;
        if scene == (*this).currentScene {
            GfxPrint_SetColor(printer, 255 as libc::c_int as u32_0,
                              20 as libc::c_int as u32_0,
                              20 as libc::c_int as u32_0,
                              255 as libc::c_int as u32_0);
        } else {
            GfxPrint_SetColor(printer, 200 as libc::c_int as u32_0,
                              200 as libc::c_int as u32_0,
                              55 as libc::c_int as u32_0,
                              255 as libc::c_int as u32_0);
        }
        name = (*(*this).scenes.offset(scene as isize)).name;
        if name.is_null() {
            name =
                b"**Null**\x00" as *const u8 as *const libc::c_char as
                    *mut libc::c_char
        }
        GfxPrint_Printf(printer,
                        b"%s\x00" as *const u8 as *const libc::c_char, name);
        i += 1
    }
    GfxPrint_SetColor(printer, 155 as libc::c_int as u32_0,
                      55 as libc::c_int as u32_0, 150 as libc::c_int as u32_0,
                      255 as libc::c_int as u32_0);
    GfxPrint_SetPos(printer, 20 as libc::c_int, 26 as libc::c_int);
    GfxPrint_Printf(printer,
                    b"OPT=%d\x00" as *const u8 as *const libc::c_char,
                    (*this).opt);
}
static mut sLoadingMessages: [*const libc::c_char; 12] =
    [b"\x8d\xef\xbd\xbc\xef\xbe\x8a\xef\xbe\x9e\xef\xbe\x97\xef\xbd\xb8\xef\xbd\xb5\xef\xbe\x8f\xef\xbe\x81\xef\xbd\xb8\xef\xbe\x80\xef\xbe\x9e\xef\xbd\xbb\xef\xbd\xb2\x00"
         as *const u8 as *const libc::c_char,
     b"\x8d\xef\xbe\x81\xef\xbd\xae\xef\xbd\xaf\xef\xbe\x84 \xef\xbe\x8f\xef\xbd\xaf\xef\xbe\x83\xef\xbe\x88\x00"
         as *const u8 as *const libc::c_char,
     b"\x8c\xef\xbd\xb3\xef\xbd\xaa\xef\xbd\xb2\xef\xbe\x84 \xef\xbd\xb1 \xef\xbe\x93\xef\xbd\xb0\xef\xbe\x92\xef\xbe\x9d\xef\xbe\x84\x00"
         as *const u8 as *const libc::c_char,
     b"\x8c\xef\xbe\x9b\xef\xbd\xb0\xef\xbe\x84\xef\xbe\x9e\x8d\xef\xbe\x81\xef\xbd\xad\xef\xbd\xb3\x00"
         as *const u8 as *const libc::c_char,
     b"\x8d\xef\xbe\x85\xef\xbd\xb3 \xef\xbe\x9c\xef\xbd\xb0\xef\xbd\xb7\xef\xbe\x9d\xef\xbd\xb8\xef\xbe\x9e\x00"
         as *const u8 as *const libc::c_char,
     b"\x8d\xef\xbd\xb2\xef\xbe\x8f \xef\xbe\x82\xef\xbd\xb8\xef\xbd\xaf\xef\xbe\x83\xef\xbe\x8f\xef\xbd\xbd\x00"
         as *const u8 as *const libc::c_char,
     b"\x8d\xef\xbd\xba\xef\xbd\xbc\xef\xbd\xae\xef\xbd\xb3\xef\xbd\xbc\xef\xbe\x9e\xef\xbd\xac\xef\xbe\x85\xef\xbd\xb2\xef\xbe\x96\x00"
         as *const u8 as *const libc::c_char,
     b"\x8c\xef\xbd\xba\xef\xbd\xb0\xef\xbe\x8b\xef\xbd\xb0 \xef\xbe\x8c\xef\xbe\x9e\xef\xbe\x9a\xef\xbd\xb2\xef\xbd\xb8\x00"
         as *const u8 as *const libc::c_char,
     b"\x8cB\xef\xbe\x92\xef\xbe\x9d\xef\xbd\xa6\xef\xbd\xbe\xef\xbd\xaf\xef\xbe\x84\xef\xbd\xbc\xef\xbe\x83\xef\xbd\xb8\xef\xbe\x80\xef\xbe\x9e\xef\xbd\xbb\xef\xbd\xb2\x00"
         as *const u8 as *const libc::c_char,
     b"\x8d\xef\xbd\xbc\xef\xbe\x9e\xef\xbd\xaf\xef\xbe\x84\x8c\xef\xbd\xb6\xef\xbe\x9e\xef\xbe\x8f\xef\xbe\x9d\x8d\xef\xbe\x89\x8c\xef\xbd\xba\x8d\xef\xbe\x83\xef\xbe\x9e\xef\xbd\xb1\xef\xbd\xaf\xef\xbe\x80\x00"
         as *const u8 as *const libc::c_char,
     b"\x8d\xef\xbd\xb2\xef\xbe\x8f\xef\xbd\xbc\xef\xbe\x8a\xef\xbe\x9e\xef\xbe\x97\xef\xbd\xb8\xef\xbd\xb5\xef\xbe\x8f\xef\xbe\x81\xef\xbd\xb8\xef\xbe\x80\xef\xbe\x9e\xef\xbd\xbb\xef\xbd\xb2\x00"
         as *const u8 as *const libc::c_char,
     b"\x8d\xef\xbd\xb1\xef\xbe\x9c\xef\xbe\x83\xef\xbe\x85\xef\xbd\xb2\xef\xbd\xb1\xef\xbe\x9c\xef\xbe\x83\xef\xbe\x85\xef\xbd\xb2\xef\xbd\xa1\xef\xbe\x8b\xef\xbe\x84\xef\xbe\x94\xef\xbd\xbd\xef\xbe\x90\xef\xbe\x8b\xef\xbe\x84\xef\xbe\x94\xef\xbd\xbd\xef\xbe\x90\xef\xbd\xa1\x00"
         as *const u8 as *const libc::c_char];
#[no_mangle]
pub unsafe extern "C" fn Select_PrintLoadingMessage(mut this:
                                                        *mut SelectContext,
                                                    mut printer:
                                                        *mut GfxPrint) {
    let mut randomMsg: s32 = 0;
    GfxPrint_SetPos(printer, 10 as libc::c_int, 15 as libc::c_int);
    GfxPrint_SetColor(printer, 255 as libc::c_int as u32_0,
                      255 as libc::c_int as u32_0,
                      255 as libc::c_int as u32_0,
                      255 as libc::c_int as u32_0);
    randomMsg =
        (Rand_ZeroOne() *
             (::std::mem::size_of::<[*const libc::c_char; 12]>() as
                  libc::c_ulong).wrapping_div(::std::mem::size_of::<*const libc::c_char>()
                                                  as libc::c_ulong) as s32 as
                 libc::c_float) as s32;
    GfxPrint_Printf(printer, b"%s\x00" as *const u8 as *const libc::c_char,
                    sLoadingMessages[randomMsg as usize]);
}
static mut sAgeLabels: [*const libc::c_char; 2] =
    [b"\x8d17(\xef\xbe\x9c\xef\xbd\xb6\xef\xbe\x93\xef\xbe\x89)\x00" as
         *const u8 as *const libc::c_char,
     b"\x8d5(\xef\xbe\x9c\xef\xbd\xb6\xef\xbd\xbd\xef\xbd\xb7\xef\xbe\x9e)\x00"
         as *const u8 as *const libc::c_char];
#[no_mangle]
pub unsafe extern "C" fn Select_PrintAgeSetting(mut this: *mut SelectContext,
                                                mut printer: *mut GfxPrint,
                                                mut age: s32) {
    GfxPrint_SetPos(printer, 4 as libc::c_int, 26 as libc::c_int);
    GfxPrint_SetColor(printer, 255 as libc::c_int as u32_0,
                      255 as libc::c_int as u32_0, 55 as libc::c_int as u32_0,
                      255 as libc::c_int as u32_0);
    GfxPrint_Printf(printer,
                    b"Age:%s\x00" as *const u8 as *const libc::c_char,
                    sAgeLabels[age as usize]);
}
#[no_mangle]
pub unsafe extern "C" fn Select_PrintCutsceneSetting(mut this:
                                                         *mut SelectContext,
                                                     mut printer:
                                                         *mut GfxPrint,
                                                     mut csIndex: u16_0) {
    let mut label: *mut libc::c_char = 0 as *mut libc::c_char;
    GfxPrint_SetPos(printer, 4 as libc::c_int, 25 as libc::c_int);
    GfxPrint_SetColor(printer, 255 as libc::c_int as u32_0,
                      255 as libc::c_int as u32_0, 55 as libc::c_int as u32_0,
                      255 as libc::c_int as u32_0);
    match csIndex as libc::c_int {
        0 => {
            label =
                b"\x8d \xef\xbe\x96\xef\xbe\x99 \x8c\xef\xbd\xba\xef\xbe\x9e\xef\xbe\x9b\xef\xbe\x9d\x00"
                    as *const u8 as *const libc::c_char as *mut libc::c_char;
            gSaveContext.dayTime = 0 as libc::c_int as u16_0
        }
        32768 => {
            // clang-format off
            gSaveContext.dayTime = 0x8000 as libc::c_int as u16_0;
            label =
                b"\x8d\xef\xbd\xb5\xef\xbe\x8b\xef\xbe\x99 \x8c\xef\xbd\xbc\xef\xbe\x9e\xef\xbd\xac\xef\xbe\x97\x00"
                    as *const u8 as *const libc::c_char as *mut libc::c_char
        }
        65520 => {
            // clang-format off
            gSaveContext.dayTime =
                0x8000 as libc::c_int as
                    u16_0; // ASCII BEL character, plays an alert tone
            label =
                b"\xef\xbe\x83\xef\xbe\x9e\xef\xbe\x9300\x00" as *const u8 as
                    *const libc::c_char as *mut libc::c_char
        }
        65521 => {
            label =
                b"\xef\xbe\x83\xef\xbe\x9e\xef\xbe\x9301\x00" as *const u8 as
                    *const libc::c_char as *mut libc::c_char
        }
        65522 => {
            label =
                b"\xef\xbe\x83\xef\xbe\x9e\xef\xbe\x9302\x00" as *const u8 as
                    *const libc::c_char as *mut libc::c_char
        }
        65523 => {
            label =
                b"\xef\xbe\x83\xef\xbe\x9e\xef\xbe\x9303\x00" as *const u8 as
                    *const libc::c_char as *mut libc::c_char
        }
        65524 => {
            label =
                b"\xef\xbe\x83\xef\xbe\x9e\xef\xbe\x9304\x00" as *const u8 as
                    *const libc::c_char as *mut libc::c_char
        }
        65525 => {
            label =
                b"\xef\xbe\x83\xef\xbe\x9e\xef\xbe\x9305\x00" as *const u8 as
                    *const libc::c_char as *mut libc::c_char
        }
        65526 => {
            label =
                b"\xef\xbe\x83\xef\xbe\x9e\xef\xbe\x9306\x00" as *const u8 as
                    *const libc::c_char as *mut libc::c_char
        }
        65527 => {
            label =
                b"\xef\xbe\x83\xef\xbe\x9e\xef\xbe\x9307\x00" as *const u8 as
                    *const libc::c_char as *mut libc::c_char
        }
        65528 => {
            label =
                b"\xef\xbe\x83\xef\xbe\x9e\xef\xbe\x9308\x00" as *const u8 as
                    *const libc::c_char as *mut libc::c_char
        }
        65529 => {
            label =
                b"\xef\xbe\x83\xef\xbe\x9e\xef\xbe\x9309\x00" as *const u8 as
                    *const libc::c_char as *mut libc::c_char
        }
        65530 => {
            label =
                b"\xef\xbe\x83\xef\xbe\x9e\xef\xbe\x930A\x00" as *const u8 as
                    *const libc::c_char as *mut libc::c_char
        }
        _ => { }
    }
    gSaveContext.skyboxTime = gSaveContext.dayTime;
    GfxPrint_Printf(printer,
                    b"Stage:\x8c%s\x00" as *const u8 as *const libc::c_char,
                    label);
}
#[no_mangle]
pub unsafe extern "C" fn Select_DrawMenu(mut this: *mut SelectContext) {
    let mut gfxCtx: *mut GraphicsContext = (*this).state.gfxCtx;
    let mut printer: *mut GfxPrint = 0 as *mut GfxPrint;
    let mut __gfxCtx: *mut GraphicsContext = 0 as *mut GraphicsContext;
    let mut dispRefs: [*mut Gfx; 4] = [0 as *mut Gfx; 4];
    __gfxCtx = gfxCtx;
    Graph_OpenDisps(dispRefs.as_mut_ptr(), gfxCtx,
                    b"../z_select.c\x00" as *const u8 as *const libc::c_char,
                    930 as libc::c_int);
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
            ((0 as libc::c_int * 4 as libc::c_int) as u32_0 &
                 (((0x1 as libc::c_int) << 16 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 0 as libc::c_int;
    (*_g).words.w1 = 0 as *mut libc::c_void as libc::c_uint;
    func_80095248(gfxCtx, 0 as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
                  0 as libc::c_int as u8_0);
    let mut viewport: Viewport =
        Viewport{topY: 0, bottomY: 0, leftX: 0, rightX: 0,};
    viewport.bottomY = 240 as libc::c_int;
    viewport.rightX = 320 as libc::c_int;
    viewport.topY = 0 as libc::c_int;
    viewport.leftX = 0 as libc::c_int;
    View_SetViewport(&mut (*this).view, &mut viewport);
    func_800AAA50(&mut (*this).view, 0xf as libc::c_int);
    func_80094140(gfxCtx);
    let mut fresh1 =
        ::std::vec::from_elem(0,
                              ::std::mem::size_of::<GfxPrint>() as
                                  libc::c_ulong as usize);
    printer = fresh1.as_mut_ptr() as *mut GfxPrint;
    GfxPrint_Init(printer);
    GfxPrint_Open(printer, (*__gfxCtx).polyOpa.p);
    Select_PrintMenu(this, printer);
    Select_PrintAgeSetting(this, printer, gSaveContext.linkAge);
    Select_PrintCutsceneSetting(this, printer,
                                gSaveContext.cutsceneIndex as u16_0);
    (*__gfxCtx).polyOpa.p = GfxPrint_Close(printer);
    GfxPrint_Destroy(printer);
    Graph_CloseDisps(dispRefs.as_mut_ptr(), gfxCtx,
                     b"../z_select.c\x00" as *const u8 as *const libc::c_char,
                     966 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn Select_DrawLoadingScreen(mut this:
                                                      *mut SelectContext) {
    let mut gfxCtx: *mut GraphicsContext = (*this).state.gfxCtx;
    let mut printer: *mut GfxPrint = 0 as *mut GfxPrint;
    let mut __gfxCtx: *mut GraphicsContext = 0 as *mut GraphicsContext;
    let mut dispRefs: [*mut Gfx; 4] = [0 as *mut Gfx; 4];
    __gfxCtx = gfxCtx;
    Graph_OpenDisps(dispRefs.as_mut_ptr(), gfxCtx,
                    b"../z_select.c\x00" as *const u8 as *const libc::c_char,
                    977 as libc::c_int);
    let fresh2 = (*__gfxCtx).polyOpa.p;
    (*__gfxCtx).polyOpa.p = (*__gfxCtx).polyOpa.p.offset(1);
    let mut _g: *mut Gfx = fresh2;
    (*_g).words.w0 =
        (0xdb as libc::c_int as u32_0 &
             (((0x1 as libc::c_int) << 8 as libc::c_int) - 1 as libc::c_int)
                 as libc::c_uint) << 24 as libc::c_int |
            (0x6 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 16 as libc::c_int
            |
            ((0 as libc::c_int * 4 as libc::c_int) as u32_0 &
                 (((0x1 as libc::c_int) << 16 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 0 as libc::c_int;
    (*_g).words.w1 = 0 as *mut libc::c_void as libc::c_uint;
    func_80095248(gfxCtx, 0 as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
                  0 as libc::c_int as u8_0);
    let mut viewport: Viewport =
        Viewport{topY: 0, bottomY: 0, leftX: 0, rightX: 0,};
    viewport.bottomY = 240 as libc::c_int;
    viewport.rightX = 320 as libc::c_int;
    viewport.topY = 0 as libc::c_int;
    viewport.leftX = 0 as libc::c_int;
    View_SetViewport(&mut (*this).view, &mut viewport);
    func_800AAA50(&mut (*this).view, 0xf as libc::c_int);
    func_80094140(gfxCtx);
    let mut fresh3 =
        ::std::vec::from_elem(0,
                              ::std::mem::size_of::<GfxPrint>() as
                                  libc::c_ulong as usize);
    printer = fresh3.as_mut_ptr() as *mut GfxPrint;
    GfxPrint_Init(printer);
    GfxPrint_Open(printer, (*__gfxCtx).polyOpa.p);
    Select_PrintLoadingMessage(this, printer);
    (*__gfxCtx).polyOpa.p = GfxPrint_Close(printer);
    GfxPrint_Destroy(printer);
    Graph_CloseDisps(dispRefs.as_mut_ptr(), gfxCtx,
                     b"../z_select.c\x00" as *const u8 as *const libc::c_char,
                     1006 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn Select_Draw(mut this: *mut SelectContext) {
    let mut gfxCtx: *mut GraphicsContext = (*this).state.gfxCtx;
    let mut __gfxCtx: *mut GraphicsContext = 0 as *mut GraphicsContext;
    let mut dispRefs: [*mut Gfx; 4] = [0 as *mut Gfx; 4];
    __gfxCtx = gfxCtx;
    Graph_OpenDisps(dispRefs.as_mut_ptr(), gfxCtx,
                    b"../z_select.c\x00" as *const u8 as *const libc::c_char,
                    1013 as libc::c_int);
    let fresh4 = (*__gfxCtx).polyOpa.p;
    (*__gfxCtx).polyOpa.p = (*__gfxCtx).polyOpa.p.offset(1);
    let mut _g: *mut Gfx = fresh4;
    (*_g).words.w0 =
        (0xdb as libc::c_int as u32_0 &
             (((0x1 as libc::c_int) << 8 as libc::c_int) - 1 as libc::c_int)
                 as libc::c_uint) << 24 as libc::c_int |
            (0x6 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 16 as libc::c_int
            |
            ((0 as libc::c_int * 4 as libc::c_int) as u32_0 &
                 (((0x1 as libc::c_int) << 16 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 0 as libc::c_int;
    (*_g).words.w1 = 0 as *mut libc::c_void as libc::c_uint;
    func_80095248(gfxCtx, 0 as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
                  0 as libc::c_int as u8_0);
    let mut viewport: Viewport =
        Viewport{topY: 0, bottomY: 0, leftX: 0, rightX: 0,};
    viewport.bottomY = 240 as libc::c_int;
    viewport.rightX = 320 as libc::c_int;
    viewport.topY = 0 as libc::c_int;
    viewport.leftX = 0 as libc::c_int;
    View_SetViewport(&mut (*this).view, &mut viewport);
    func_800AAA50(&mut (*this).view, 0xf as libc::c_int);
    if (*this).state.running == 0 {
        Select_DrawLoadingScreen(this);
    } else { Select_DrawMenu(this); }
    Graph_CloseDisps(dispRefs.as_mut_ptr(), gfxCtx,
                     b"../z_select.c\x00" as *const u8 as *const libc::c_char,
                     1037 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn Select_Main(mut thisx: *mut GameState) {
    let mut this: *mut SelectContext = thisx as *mut SelectContext;
    Select_UpdateMenu(this);
    Select_Draw(this);
}
#[no_mangle]
pub unsafe extern "C" fn Select_Destroy(mut thisx: *mut GameState) {
    osSyncPrintf(b"%c\x00" as *const u8 as *const libc::c_char,
                 '\u{7}' as i32);
    // "view_cleanup will hang, so it won't be called"
    osSyncPrintf(b"*** view_cleanup\xe3\x81\xaf\xe3\x83\x8f\xe3\x83\xb3\xe3\x82\xb0\xe3\x82\xa2\xe3\x83\x83\xe3\x83\x97\xe3\x81\x99\xe3\x82\x8b\xe3\x81\xae\xe3\x81\xa7\xe3\x80\x81\xe5\x91\xbc\xe3\x81\xb0\xe3\x81\xaa\xe3\x81\x84 ***\n\x00"
                     as *const u8 as *const libc::c_char); // Hyrule Field
}
#[no_mangle]
pub unsafe extern "C" fn Select_Init(mut thisx: *mut GameState) {
    let mut this: *mut SelectContext =
        thisx as *mut SelectContext; // Temple Of Time
    let mut size: u32_0 = 0; // Treasure Chest Game
    let mut pad: s32 = 0; // Gravekeeper's Hut
    (*this).state.main =
        Some(Select_Main as
                 unsafe extern "C" fn(_: *mut GameState) -> ()); // Zora Shop
    (*this).state.destroy =
        Some(Select_Destroy as
                 unsafe extern "C" fn(_: *mut GameState)
                     -> ()); // Bottom of the Well
    (*this).scenes = sScenes.as_mut_ptr(); // Escaping Ganon's Tower 3
    (*this).topDisplayedScene = 0 as libc::c_int;
    (*this).currentScene = 0 as libc::c_int;
    (*this).pageDownStops[0 as libc::c_int as usize] = 0 as libc::c_int;
    (*this).pageDownStops[1 as libc::c_int as usize] = 19 as libc::c_int;
    (*this).pageDownStops[2 as libc::c_int as usize] = 37 as libc::c_int;
    (*this).pageDownStops[3 as libc::c_int as usize] = 51 as libc::c_int;
    (*this).pageDownStops[4 as libc::c_int as usize] = 59 as libc::c_int;
    (*this).pageDownStops[5 as libc::c_int as usize] = 73 as libc::c_int;
    (*this).pageDownStops[6 as libc::c_int as usize] = 91 as libc::c_int;
    (*this).pageDownIndex = 0 as libc::c_int;
    (*this).opt = 0 as libc::c_int;
    (*this).count =
        (::std::mem::size_of::<[SceneSelectEntry; 126]>() as
             libc::c_ulong).wrapping_div(::std::mem::size_of::<SceneSelectEntry>()
                                             as libc::c_ulong) as s32;
    View_Init(&mut (*this).view, (*this).state.gfxCtx);
    (*this).view.flags = 0x8 as libc::c_int | 0x2 as libc::c_int;
    (*this).verticalInputAccumulator = 0 as libc::c_int;
    (*this).verticalInput = 0 as libc::c_int;
    (*this).timerUp = 0 as libc::c_int;
    (*this).timerDown = 0 as libc::c_int;
    (*this).lockUp = 0 as libc::c_int;
    (*this).lockDown = 0 as libc::c_int;
    (*this).unk_234 = 0 as libc::c_int;
    size =
        (_z_select_staticSegmentRomEnd.as_mut_ptr() as
             u32_0).wrapping_sub(_z_select_staticSegmentRomStart.as_mut_ptr()
                                     as u32_0);
    if (*gGameInfo).data[(26 as libc::c_int * 6 as libc::c_int *
                              16 as libc::c_int + 80 as libc::c_int) as usize]
           as libc::c_int >= 0 as libc::c_int &&
           ((*gGameInfo).data[(26 as libc::c_int * 6 as libc::c_int *
                                   16 as libc::c_int + 80 as libc::c_int) as
                                  usize] as libc::c_int) < (*this).count {
        (*this).currentScene =
            (*gGameInfo).data[(26 as libc::c_int * 6 as libc::c_int *
                                   16 as libc::c_int + 80 as libc::c_int) as
                                  usize] as s32;
        (*this).topDisplayedScene =
            (*gGameInfo).data[(26 as libc::c_int * 6 as libc::c_int *
                                   16 as libc::c_int + 81 as libc::c_int) as
                                  usize] as s32;
        (*this).pageDownIndex =
            (*gGameInfo).data[(26 as libc::c_int * 6 as libc::c_int *
                                   16 as libc::c_int + 82 as libc::c_int) as
                                  usize] as s32
    }
    (*gGameInfo).data[(1 as libc::c_int * 6 as libc::c_int * 16 as libc::c_int
                           + 30 as libc::c_int) as usize] =
        1 as libc::c_int as s16;
    (*this).staticSegment =
        GameState_Alloc(&mut (*this).state, size as size_t,
                        b"../z_select.c\x00" as *const u8 as
                            *const libc::c_char as *mut libc::c_char,
                        1114 as libc::c_int) as *mut u8_0;
    DmaMgr_SendRequest1((*this).staticSegment as *mut libc::c_void,
                        _z_select_staticSegmentRomStart.as_mut_ptr() as u32_0,
                        size,
                        b"../z_select.c\x00" as *const u8 as
                            *const libc::c_char, 1115 as libc::c_int);
    gSaveContext.cutsceneIndex = 0x8000 as libc::c_int;
    gSaveContext.linkAge = 1 as libc::c_int;
}
