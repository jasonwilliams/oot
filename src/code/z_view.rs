#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case,
         non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(const_raw_ptr_to_usize_cast, register_tool)]
extern "C" {
    #[no_mangle]
    fn __assert(exp: *const libc::c_char, file: *const libc::c_char,
                line: s32);
    #[no_mangle]
    fn osSyncPrintf(fmt: *const libc::c_char, _: ...);
    #[no_mangle]
    fn LogUtils_CheckNullPointer(exp: *const libc::c_char,
                                 ptr: *mut libc::c_void,
                                 file: *const libc::c_char, line: s32);
    #[no_mangle]
    fn ShrinkWindow_GetCurrentVal() -> u32_0;
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
    fn Matrix_Put(src: *mut MtxF);
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
    #[no_mangle]
    fn Matrix_MtxToMtxF(src: *mut Mtx, dest: *mut MtxF);
    #[no_mangle]
    fn SystemArena_MallocDebug(size: u32_0, file: *const libc::c_char,
                               line: s32) -> *mut libc::c_void;
    #[no_mangle]
    fn SystemArena_FreeDebug(ptr: *mut libc::c_void,
                             file: *const libc::c_char, line: s32);
    #[no_mangle]
    fn func_80106860(ptr: *mut libc::c_void, val: s32, size: size_t)
     -> *mut libc::c_void;
    #[no_mangle]
    static mut gGameInfo: *mut GameInfo;
    #[no_mangle]
    fn guLookAt(_: *mut Mtx, xEye: f32_0, yEye: f32_0, zEye: f32_0,
                xAt: f32_0, yAt: f32_0, zAt: f32_0, xUp: f32_0, yUp: f32_0,
                zUp: f32_0);
    #[no_mangle]
    fn guPerspective(m: *mut Mtx, perspNorm: *mut u16_0, fovy: f32_0,
                     aspect: f32_0, near: f32_0, far: f32_0, scale: f32_0);
    #[no_mangle]
    static mut gScreenHeight: s32;
    #[no_mangle]
    static mut gScreenWidth: s32;
    #[no_mangle]
    fn guOrtho(_: *mut Mtx, _: f32_0, _: f32_0, _: f32_0, _: f32_0, _: f32_0,
               _: f32_0, _: f32_0);
}
pub type s8 = libc::c_schar;
pub type u8_0 = libc::c_uchar;
pub type s16 = libc::c_short;
pub type u16_0 = libc::c_ushort;
pub type s32 = libc::c_int;
pub type u32_0 = libc::c_uint;
pub type u64_0 = libc::c_ulonglong;
pub type vu32 = u32_0;
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
pub struct GameInfo {
    pub regPage: s32,
    pub regGroup: s32,
    pub regCur: s32,
    pub dpadLast: s32,
    pub repeat: s32,
    pub data: [s16; 2784],
}
pub type C2RustUnnamed_2 = libc::c_uint;
pub const MTXMODE_APPLY: C2RustUnnamed_2 = 1;
pub const MTXMODE_NEW: C2RustUnnamed_2 = 0;
#[no_mangle]
pub static mut D_8012ABF0: vu32 = 1 as libc::c_int as u32_0;
#[no_mangle]
pub unsafe extern "C" fn View_ViewportToVp(mut dest: *mut Vp,
                                           mut src: *mut Viewport) {
    let mut width: s32 = (*src).rightX - (*src).leftX; // memset
    let mut height: s32 = (*src).bottomY - (*src).topY; // "VIEW"
    (*dest).vp.vscale[0 as libc::c_int as usize] =
        (width * 2 as libc::c_int) as
            libc::c_short; // The following is optimized to varX = 0 but affects codegen
    (*dest).vp.vscale[1 as libc::c_int as usize] =
        (height * 2 as libc::c_int) as libc::c_short;
    (*dest).vp.vscale[2 as libc::c_int as usize] =
        0x1ff as libc::c_int as libc::c_short;
    (*dest).vp.vscale[3 as libc::c_int as usize] =
        0 as libc::c_int as libc::c_short;
    (*dest).vp.vtrans[0 as libc::c_int as usize] =
        (((*src).leftX * 2 as libc::c_int + width) * 2 as libc::c_int) as
            libc::c_short;
    (*dest).vp.vtrans[1 as libc::c_int as usize] =
        (((*src).topY * 2 as libc::c_int + height) * 2 as libc::c_int) as
            libc::c_short;
    (*dest).vp.vtrans[2 as libc::c_int as usize] =
        0x1ff as libc::c_int as libc::c_short;
    (*dest).vp.vtrans[3 as libc::c_int as usize] =
        0 as libc::c_int as libc::c_short;
}
#[no_mangle]
pub unsafe extern "C" fn View_New(mut gfxCtx: *mut GraphicsContext)
 -> *mut View {
    let mut view: *mut View =
        SystemArena_MallocDebug(::std::mem::size_of::<View>() as
                                    libc::c_ulong,
                                b"../z_view.c\x00" as *const u8 as
                                    *const libc::c_char, 285 as libc::c_int)
            as *mut View;
    if !view.is_null() {
        func_80106860(view as *mut libc::c_void, 0 as libc::c_int,
                      ::std::mem::size_of::<View>() as libc::c_ulong as
                          size_t);
        View_Init(view, gfxCtx);
    }
    return view;
}
#[no_mangle]
pub unsafe extern "C" fn View_Free(mut view: *mut View) {
    SystemArena_FreeDebug(view as *mut libc::c_void,
                          b"../z_view.c\x00" as *const u8 as
                              *const libc::c_char, 297 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn View_Init(mut view: *mut View,
                                   mut gfxCtx: *mut GraphicsContext) {
    (*view).gfxCtx = gfxCtx;
    (*view).viewport.topY = 0 as libc::c_int;
    (*view).viewport.bottomY = 240 as libc::c_int;
    (*view).viewport.leftX = 0 as libc::c_int;
    (*view).viewport.rightX = 320 as libc::c_int;
    (*view).magic = 0x56494557 as libc::c_int;
    (*view).eye.x = 0.0f32;
    (*view).eye.y = 0.0f32;
    (*view).scale = 1.0f32;
    (*view).fovy = 60.0f32;
    (*view).zNear = 10.0f32;
    (*view).zFar = 12800.0f32;
    (*view).lookAt.x = 0.0f32;
    (*view).up.x = 0.0f32;
    (*view).up.y = 1.0f32;
    (*view).up.z = 0.0f32;
    (*view).eye.z = -1.0f32;
    if D_8012ABF0 != 0 {
        (D_8012ABF0) == 0 as libc::c_int as libc::c_uint;
        osSyncPrintf(b"\nview: initialize ---\n\x00" as *const u8 as
                         *const libc::c_char);
        ::std::ptr::write_volatile(&mut D_8012ABF0 as *mut vu32,
                                   0 as libc::c_int as u32_0)
    }
    (*view).unk_124 = 0 as libc::c_int;
    (*view).flags = 1 as libc::c_int | 2 as libc::c_int | 4 as libc::c_int;
    func_800AA7B8(view);
}
#[no_mangle]
pub unsafe extern "C" fn func_800AA358(mut view: *mut View,
                                       mut eye: *mut Vec3f,
                                       mut lookAt: *mut Vec3f,
                                       mut up: *mut Vec3f) {
    if (*eye).x == (*lookAt).x && (*eye).z == (*lookAt).z {
        (*eye).x += 0.1f32
    }
    (*view).eye = *eye;
    (*view).lookAt = *lookAt;
    (*view).up = *up;
    (*view).flags |= 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn func_800AA3F0(mut view: *mut View,
                                       mut eye: *mut Vec3f,
                                       mut lookAt: *mut Vec3f,
                                       mut up: *mut Vec3f) {
    (*view).eye = *eye;
    (*view).lookAt = *lookAt;
    (*view).up = *up;
}
#[no_mangle]
pub unsafe extern "C" fn View_SetScale(mut view: *mut View,
                                       mut scale: f32_0) {
    (*view).flags |= 4 as libc::c_int;
    (*view).scale = scale;
}
#[no_mangle]
pub unsafe extern "C" fn View_GetScale(mut view: *mut View,
                                       mut scale: *mut f32_0) {
    *scale = (*view).scale;
}
#[no_mangle]
pub unsafe extern "C" fn func_800AA460(mut view: *mut View, mut fovy: f32_0,
                                       mut near: f32_0, mut far: f32_0) {
    (*view).fovy = fovy;
    (*view).zNear = near;
    (*view).zFar = far;
    (*view).flags |= 4 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn func_800AA48C(mut view: *mut View,
                                       mut fovy: *mut f32_0,
                                       mut near: *mut f32_0,
                                       mut far: *mut f32_0) {
    *fovy = (*view).fovy;
    *near = (*view).zNear;
    *far = (*view).zFar;
}
#[no_mangle]
pub unsafe extern "C" fn func_800AA4A8(mut view: *mut View, mut fovy: f32_0,
                                       mut near: f32_0, mut far: f32_0) {
    (*view).fovy = fovy;
    (*view).zNear = near;
    (*view).zFar = far;
    (*view).flags |= 8 as libc::c_int;
    (*view).scale = 1.0f32;
}
#[no_mangle]
pub unsafe extern "C" fn func_800AA4E0(mut view: *mut View,
                                       mut fovy: *mut f32_0,
                                       mut near: *mut f32_0,
                                       mut far: *mut f32_0) {
    *fovy = (*view).fovy;
    *near = (*view).zNear;
    *far = (*view).zFar;
}
#[no_mangle]
pub unsafe extern "C" fn View_SetViewport(mut view: *mut View,
                                          mut viewport: *mut Viewport) {
    (*view).viewport = *viewport;
    (*view).flags |= 2 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn View_GetViewport(mut view: *mut View,
                                          mut viewport: *mut Viewport) {
    *viewport = (*view).viewport;
}
#[no_mangle]
pub unsafe extern "C" fn func_800AA550(mut view: *mut View) {
    let mut varY: s32 = 0;
    let mut varX: s32 = 0;
    let mut pad: s32 = 0;
    let mut ulx: s32 = 0;
    let mut uly: s32 = 0;
    let mut lrx: s32 = 0;
    let mut lry: s32 = 0;
    let mut gfxCtx: *mut GraphicsContext = (*view).gfxCtx;
    varY = ShrinkWindow_GetCurrentVal() as s32;
    varX = -(1 as libc::c_int);
    if varX < 0 as libc::c_int { varX = 0 as libc::c_int }
    if varX > 320 as libc::c_int / 2 as libc::c_int {
        varX = 320 as libc::c_int / 2 as libc::c_int
    }
    if varY < 0 as libc::c_int { varY = 0 as libc::c_int }
    if varY > 240 as libc::c_int / 2 as libc::c_int {
        varY = 240 as libc::c_int / 2 as libc::c_int
    }
    ulx = (*view).viewport.leftX + varX;
    uly = (*view).viewport.topY + varY;
    lrx = (*view).viewport.rightX - varX;
    lry = (*view).viewport.bottomY - varY;
    if ulx >= 0 as libc::c_int {
    } else {
        __assert(b"ulx >= 0\x00" as *const u8 as *const libc::c_char,
                 b"../z_view.c\x00" as *const u8 as *const libc::c_char,
                 454 as libc::c_int);
    };
    if uly >= 0 as libc::c_int {
    } else {
        __assert(b"uly >= 0\x00" as *const u8 as *const libc::c_char,
                 b"../z_view.c\x00" as *const u8 as *const libc::c_char,
                 455 as libc::c_int);
    };
    if lrx <= 320 as libc::c_int {
    } else {
        __assert(b"lrx <= SCREEN_WD\x00" as *const u8 as *const libc::c_char,
                 b"../z_view.c\x00" as *const u8 as *const libc::c_char,
                 456 as libc::c_int);
    };
    if lry <= 240 as libc::c_int {
    } else {
        __assert(b"lry <= SCREEN_HT\x00" as *const u8 as *const libc::c_char,
                 b"../z_view.c\x00" as *const u8 as *const libc::c_char,
                 457 as libc::c_int);
    };
    let mut __gfxCtx: *mut GraphicsContext = 0 as *mut GraphicsContext;
    let mut dispRefs: [*mut Gfx; 4] = [0 as *mut Gfx; 4];
    __gfxCtx = gfxCtx;
    Graph_OpenDisps(dispRefs.as_mut_ptr(), gfxCtx,
                    b"../z_view.c\x00" as *const u8 as *const libc::c_char,
                    459 as libc::c_int);
    let fresh0 = (*__gfxCtx).polyOpa.p;
    (*__gfxCtx).polyOpa.p = (*__gfxCtx).polyOpa.p.offset(1);
    let mut _g: *mut Gfx = fresh0;
    (*_g).words.w0 =
        (0xe7 as libc::c_int as u32_0 &
             (((0x1 as libc::c_int) << 8 as libc::c_int) - 1 as libc::c_int)
                 as libc::c_uint) << 24 as libc::c_int;
    (*_g).words.w1 = 0 as libc::c_int as libc::c_uint;
    let fresh1 = (*__gfxCtx).polyOpa.p;
    (*__gfxCtx).polyOpa.p = (*__gfxCtx).polyOpa.p.offset(1);
    let mut _g_0: *mut Gfx = fresh1;
    (*_g_0).words.w0 =
        (0xed as libc::c_int as u32_0 &
             (((0x1 as libc::c_int) << 8 as libc::c_int) - 1 as libc::c_int)
                 as libc::c_uint) << 24 as libc::c_int |
            ((ulx as libc::c_float * 4.0f32) as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 12 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 12 as libc::c_int
            |
            ((uly as libc::c_float * 4.0f32) as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 12 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 0 as libc::c_int;
    (*_g_0).words.w1 =
        (0 as libc::c_int as u32_0 &
             (((0x1 as libc::c_int) << 2 as libc::c_int) - 1 as libc::c_int)
                 as libc::c_uint) << 24 as libc::c_int |
            ((lrx as libc::c_float * 4.0f32) as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 12 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 12 as libc::c_int
            |
            ((lry as libc::c_float * 4.0f32) as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 12 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 0 as libc::c_int;
    let fresh2 = (*__gfxCtx).polyXlu.p;
    (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
    let mut _g_1: *mut Gfx = fresh2;
    (*_g_1).words.w0 =
        (0xe7 as libc::c_int as u32_0 &
             (((0x1 as libc::c_int) << 8 as libc::c_int) - 1 as libc::c_int)
                 as libc::c_uint) << 24 as libc::c_int;
    (*_g_1).words.w1 = 0 as libc::c_int as libc::c_uint;
    let fresh3 = (*__gfxCtx).polyXlu.p;
    (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
    let mut _g_2: *mut Gfx = fresh3;
    (*_g_2).words.w0 =
        (0xed as libc::c_int as u32_0 &
             (((0x1 as libc::c_int) << 8 as libc::c_int) - 1 as libc::c_int)
                 as libc::c_uint) << 24 as libc::c_int |
            ((ulx as libc::c_float * 4.0f32) as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 12 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 12 as libc::c_int
            |
            ((uly as libc::c_float * 4.0f32) as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 12 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 0 as libc::c_int;
    (*_g_2).words.w1 =
        (0 as libc::c_int as u32_0 &
             (((0x1 as libc::c_int) << 2 as libc::c_int) - 1 as libc::c_int)
                 as libc::c_uint) << 24 as libc::c_int |
            ((lrx as libc::c_float * 4.0f32) as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 12 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 12 as libc::c_int
            |
            ((lry as libc::c_float * 4.0f32) as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 12 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 0 as libc::c_int;
    Graph_CloseDisps(dispRefs.as_mut_ptr(), gfxCtx,
                     b"../z_view.c\x00" as *const u8 as *const libc::c_char,
                     472 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn func_800AA76C(mut view: *mut View, mut x: f32_0,
                                       mut y: f32_0, mut z: f32_0) {
    (*view).unk_E8.x = x;
    (*view).unk_E8.y = y;
    (*view).unk_E8.z = z;
}
#[no_mangle]
pub unsafe extern "C" fn func_800AA78C(mut view: *mut View, mut x: f32_0,
                                       mut y: f32_0, mut z: f32_0) {
    (*view).unk_F4.x = x;
    (*view).unk_F4.y = y;
    (*view).unk_F4.z = z;
}
#[no_mangle]
pub unsafe extern "C" fn func_800AA7AC(mut view: *mut View, mut arg1: f32_0)
 -> s32 {
    (*view).unk_100 = arg1;
    panic!("Reached end of non-void function without returning");
}
#[no_mangle]
pub unsafe extern "C" fn func_800AA7B8(mut view: *mut View) {
    (*view).unk_E8.x = 0.0f32;
    (*view).unk_E8.y = 0.0f32;
    (*view).unk_E8.z = 0.0f32;
    (*view).unk_F4.x = 1.0f32;
    (*view).unk_F4.y = 1.0f32;
    (*view).unk_F4.z = 1.0f32;
    (*view).unk_104 = (*view).unk_E8;
    (*view).unk_110 = (*view).unk_F4;
    (*view).unk_100 = 0.0f32;
}
#[no_mangle]
pub unsafe extern "C" fn func_800AA814(mut view: *mut View) {
    (*view).unk_E8.x = 0.0f32;
    (*view).unk_E8.y = 0.0f32;
    (*view).unk_E8.z = 0.0f32;
    (*view).unk_F4.x = 1.0f32;
    (*view).unk_F4.y = 1.0f32;
    (*view).unk_F4.z = 1.0f32;
    (*view).unk_100 = 1.0f32;
}
#[no_mangle]
pub unsafe extern "C" fn func_800AA840(mut view: *mut View, mut vec1: Vec3f,
                                       mut vec2: Vec3f, mut arg3: f32_0) {
    (*view).unk_E8 = vec1;
    (*view).unk_F4 = vec2;
    (*view).unk_100 = arg3;
}
#[no_mangle]
pub unsafe extern "C" fn func_800AA890(mut view: *mut View, mut mtx: *mut Mtx)
 -> s32 {
    let mut mf: MtxF = MtxF{mf: [[0.; 4]; 4],};
    if (*view).unk_100 == 0.0f32 {
        return 0 as libc::c_int
    } else {
        if (*view).unk_100 == 1.0f32 {
            (*view).unk_104 = (*view).unk_E8;
            (*view).unk_110 = (*view).unk_F4;
            (*view).unk_100 = 0.0f32
        } else {
            (*view).unk_104.x +=
                ((*view).unk_E8.x - (*view).unk_104.x) * (*view).unk_100;
            (*view).unk_104.y +=
                ((*view).unk_E8.y - (*view).unk_104.y) * (*view).unk_100;
            (*view).unk_104.z +=
                ((*view).unk_E8.z - (*view).unk_104.z) * (*view).unk_100;
            (*view).unk_110.x +=
                ((*view).unk_F4.x - (*view).unk_110.x) * (*view).unk_100;
            (*view).unk_110.y +=
                ((*view).unk_F4.y - (*view).unk_110.y) * (*view).unk_100;
            (*view).unk_110.z +=
                ((*view).unk_F4.z - (*view).unk_110.z) * (*view).unk_100
        }
    }
    Matrix_MtxToMtxF(mtx, &mut mf);
    Matrix_Put(&mut mf);
    Matrix_RotateX((*view).unk_104.x, MTXMODE_APPLY as libc::c_int as u8_0);
    Matrix_RotateY((*view).unk_104.y, MTXMODE_APPLY as libc::c_int as u8_0);
    Matrix_RotateZ((*view).unk_104.z, MTXMODE_APPLY as libc::c_int as u8_0);
    Matrix_Scale((*view).unk_110.x, (*view).unk_110.y, (*view).unk_110.z,
                 MTXMODE_APPLY as libc::c_int as u8_0);
    Matrix_RotateZ(-(*view).unk_104.z, MTXMODE_APPLY as libc::c_int as u8_0);
    Matrix_RotateY(-(*view).unk_104.y, MTXMODE_APPLY as libc::c_int as u8_0);
    Matrix_RotateX(-(*view).unk_104.x, MTXMODE_APPLY as libc::c_int as u8_0);
    Matrix_ToMtx(mtx,
                 b"../z_view.c\x00" as *const u8 as *const libc::c_char as
                     *mut libc::c_char, 566 as libc::c_int);
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn func_800AAA50(mut view: *mut View, mut arg1: s32) {
    arg1 = (*view).flags & arg1 | arg1 >> 4 as libc::c_int;
    if arg1 & 8 as libc::c_int != 0 {
        func_800AB0A8(view);
    } else { func_800AAA9C(view); };
}
#[no_mangle]
pub unsafe extern "C" fn func_800AAA9C(mut view: *mut View) -> s32 {
    let mut aspect: f32_0 = 0.;
    let mut width: s32 = 0;
    let mut height: s32 = 0;
    let mut vp: *mut Vp = 0 as *mut Vp;
    let mut projection: *mut Mtx = 0 as *mut Mtx;
    let mut viewing: *mut Mtx = 0 as *mut Mtx;
    let mut gfxCtx: *mut GraphicsContext = (*view).gfxCtx;
    let mut __gfxCtx: *mut GraphicsContext = 0 as *mut GraphicsContext;
    let mut dispRefs: [*mut Gfx; 4] = [0 as *mut Gfx; 4];
    __gfxCtx = gfxCtx;
    Graph_OpenDisps(dispRefs.as_mut_ptr(), gfxCtx,
                    b"../z_view.c\x00" as *const u8 as *const libc::c_char,
                    596 as libc::c_int);
    vp =
        Graph_Alloc(gfxCtx,
                    ::std::mem::size_of::<Vp>() as libc::c_ulong as size_t) as
            *mut Vp;
    LogUtils_CheckNullPointer(b"vp\x00" as *const u8 as *const libc::c_char,
                              vp as *mut libc::c_void,
                              b"../z_view.c\x00" as *const u8 as
                                  *const libc::c_char, 601 as libc::c_int);
    View_ViewportToVp(vp, &mut (*view).viewport);
    (*view).vp = *vp;
    func_800AA550(view);
    let fresh4 = (*__gfxCtx).polyOpa.p;
    (*__gfxCtx).polyOpa.p = (*__gfxCtx).polyOpa.p.offset(1);
    let mut _g: *mut Gfx = fresh4;
    (*_g).words.w0 =
        (0xdc as libc::c_int as u32_0 &
             (((0x1 as libc::c_int) << 8 as libc::c_int) - 1 as libc::c_int)
                 as libc::c_uint) << 24 as libc::c_int |
            ((::std::mem::size_of::<Vp>() as
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
            (8 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 0 as libc::c_int;
    (*_g).words.w1 = vp as libc::c_uint;
    let fresh5 = (*__gfxCtx).polyXlu.p;
    (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
    let mut _g_0: *mut Gfx = fresh5;
    (*_g_0).words.w0 =
        (0xdc as libc::c_int as u32_0 &
             (((0x1 as libc::c_int) << 8 as libc::c_int) - 1 as libc::c_int)
                 as libc::c_uint) << 24 as libc::c_int |
            ((::std::mem::size_of::<Vp>() as
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
            (8 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 0 as libc::c_int;
    (*_g_0).words.w1 = vp as libc::c_uint;
    projection =
        Graph_Alloc(gfxCtx,
                    ::std::mem::size_of::<Mtx>() as libc::c_ulong as size_t)
            as *mut Mtx;
    LogUtils_CheckNullPointer(b"projection\x00" as *const u8 as
                                  *const libc::c_char,
                              projection as *mut libc::c_void,
                              b"../z_view.c\x00" as *const u8 as
                                  *const libc::c_char, 616 as libc::c_int);
    (*view).projectionPtr = projection;
    width = (*view).viewport.rightX - (*view).viewport.leftX;
    height = (*view).viewport.bottomY - (*view).viewport.topY;
    aspect = width as f32_0 / height as f32_0;
    if (*gGameInfo).data[(21 as libc::c_int * 6 as libc::c_int *
                              16 as libc::c_int + 80 as libc::c_int) as usize]
           as libc::c_int == 11 as libc::c_int {
        if (*gGameInfo).data[(21 as libc::c_int * 6 as libc::c_int *
                                  16 as libc::c_int + 94 as libc::c_int) as
                                 usize] as libc::c_int != 11 as libc::c_int {
            (*gGameInfo).data[(21 as libc::c_int * 6 as libc::c_int *
                                   16 as libc::c_int + 94 as libc::c_int) as
                                  usize] = 11 as libc::c_int as s16;
            (*gGameInfo).data[(21 as libc::c_int * 6 as libc::c_int *
                                   16 as libc::c_int + 83 as libc::c_int) as
                                  usize] = 60 as libc::c_int as s16;
            (*gGameInfo).data[(21 as libc::c_int * 6 as libc::c_int *
                                   16 as libc::c_int + 84 as libc::c_int) as
                                  usize] = 13333 as libc::c_int as s16;
            (*gGameInfo).data[(21 as libc::c_int * 6 as libc::c_int *
                                   16 as libc::c_int + 85 as libc::c_int) as
                                  usize] = 10 as libc::c_int as s16;
            (*gGameInfo).data[(21 as libc::c_int * 6 as libc::c_int *
                                   16 as libc::c_int + 86 as libc::c_int) as
                                  usize] = 12800 as libc::c_int as s16;
            (*gGameInfo).data[(21 as libc::c_int * 6 as libc::c_int *
                                   16 as libc::c_int + 87 as libc::c_int) as
                                  usize] = 100 as libc::c_int as s16
        }
        guPerspective(projection, &mut (*view).normal,
                      (*gGameInfo).data[(21 as libc::c_int * 6 as libc::c_int
                                             * 16 as libc::c_int +
                                             83 as libc::c_int) as usize] as
                          f32_0,
                      (*gGameInfo).data[(21 as libc::c_int * 6 as libc::c_int
                                             * 16 as libc::c_int +
                                             84 as libc::c_int) as usize] as
                          libc::c_int as libc::c_float / 10000.0f32,
                      (*gGameInfo).data[(21 as libc::c_int * 6 as libc::c_int
                                             * 16 as libc::c_int +
                                             85 as libc::c_int) as usize] as
                          f32_0,
                      (*gGameInfo).data[(21 as libc::c_int * 6 as libc::c_int
                                             * 16 as libc::c_int +
                                             86 as libc::c_int) as usize] as
                          f32_0,
                      (*gGameInfo).data[(21 as libc::c_int * 6 as libc::c_int
                                             * 16 as libc::c_int +
                                             87 as libc::c_int) as usize] as
                          libc::c_int as libc::c_float / 100.0f32);
    } else {
        guPerspective(projection, &mut (*view).normal, (*view).fovy, aspect,
                      (*view).zNear, (*view).zFar, (*view).scale);
    }
    if (*gGameInfo).data[(4 as libc::c_int * 6 as libc::c_int *
                              16 as libc::c_int + 88 as libc::c_int) as usize]
           as libc::c_int & 1 as libc::c_int != 0 {
        let mut i: s32 = 0;
        let mut mf: MtxF = MtxF{mf: [[0.; 4]; 4],};
        osSyncPrintf(b"fovy %f near %f far %f scale %f aspect %f normal %08x\n\x00"
                         as *const u8 as *const libc::c_char,
                     (*view).fovy as libc::c_double,
                     (*view).zNear as libc::c_double,
                     (*view).zFar as libc::c_double,
                     (*view).scale as libc::c_double,
                     aspect as libc::c_double, (*view).normal as libc::c_int);
        Matrix_MtxToMtxF(projection, &mut mf);
        osSyncPrintf(b"projection\n\x00" as *const u8 as *const libc::c_char);
        i = 0 as libc::c_int;
        while i < 4 as libc::c_int {
            osSyncPrintf(b"\t%f\t%f\t%f\t%f\n\x00" as *const u8 as
                             *const libc::c_char,
                         mf.mf[i as usize][0 as libc::c_int as usize] as
                             libc::c_double,
                         mf.mf[i as usize][1 as libc::c_int as usize] as
                             libc::c_double,
                         mf.mf[i as usize][2 as libc::c_int as usize] as
                             libc::c_double,
                         mf.mf[i as usize][3 as libc::c_int as usize] as
                             libc::c_double);
            i += 1
        }
        osSyncPrintf(b"\n\x00" as *const u8 as *const libc::c_char);
    }
    (*view).projection = *projection;
    func_800AA890(view, projection);
    let fresh6 = (*__gfxCtx).polyOpa.p;
    (*__gfxCtx).polyOpa.p = (*__gfxCtx).polyOpa.p.offset(1);
    let mut _g_1: *mut Gfx = fresh6;
    (*_g_1).words.w0 =
        (0xdb as libc::c_int as u32_0 &
             (((0x1 as libc::c_int) << 8 as libc::c_int) - 1 as libc::c_int)
                 as libc::c_uint) << 24 as libc::c_int |
            (0xe as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 16 as libc::c_int
            |
            (0 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 16 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 0 as libc::c_int;
    (*_g_1).words.w1 = (*view).normal as libc::c_uint;
    let fresh7 = (*__gfxCtx).polyOpa.p;
    (*__gfxCtx).polyOpa.p = (*__gfxCtx).polyOpa.p.offset(1);
    let mut _g_2: *mut Gfx = fresh7;
    (*_g_2).words.w0 =
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
            (((0 as libc::c_int | 0x2 as libc::c_int | 0x4 as libc::c_int) ^
                  0x1 as libc::c_int) as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 0 as libc::c_int;
    (*_g_2).words.w1 = projection as libc::c_uint;
    let fresh8 = (*__gfxCtx).polyXlu.p;
    (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
    let mut _g_3: *mut Gfx = fresh8;
    (*_g_3).words.w0 =
        (0xdb as libc::c_int as u32_0 &
             (((0x1 as libc::c_int) << 8 as libc::c_int) - 1 as libc::c_int)
                 as libc::c_uint) << 24 as libc::c_int |
            (0xe as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 16 as libc::c_int
            |
            (0 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 16 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 0 as libc::c_int;
    (*_g_3).words.w1 = (*view).normal as libc::c_uint;
    let fresh9 = (*__gfxCtx).polyXlu.p;
    (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
    let mut _g_4: *mut Gfx = fresh9;
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
            (((0 as libc::c_int | 0x2 as libc::c_int | 0x4 as libc::c_int) ^
                  0x1 as libc::c_int) as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 0 as libc::c_int;
    (*_g_4).words.w1 = projection as libc::c_uint;
    viewing =
        Graph_Alloc(gfxCtx,
                    ::std::mem::size_of::<Mtx>() as libc::c_ulong as size_t)
            as *mut Mtx;
    LogUtils_CheckNullPointer(b"viewing\x00" as *const u8 as
                                  *const libc::c_char,
                              viewing as *mut libc::c_void,
                              b"../z_view.c\x00" as *const u8 as
                                  *const libc::c_char, 667 as libc::c_int);
    (*view).viewingPtr = viewing;
    if (*view).eye.x == (*view).lookAt.x && (*view).eye.y == (*view).lookAt.y
           && (*view).eye.z == (*view).lookAt.z {
        (*view).eye.x += 1.0f32;
        (*view).eye.y += 1.0f32;
        (*view).eye.z += 1.0f32
    }
    func_800ABE74((*view).eye.x, (*view).eye.y, (*view).eye.z);
    guLookAt(viewing, (*view).eye.x, (*view).eye.y, (*view).eye.z,
             (*view).lookAt.x, (*view).lookAt.y, (*view).lookAt.z,
             (*view).up.x, (*view).up.y, (*view).up.z);
    (*view).viewing = *viewing;
    if (*gGameInfo).data[(4 as libc::c_int * 6 as libc::c_int *
                              16 as libc::c_int + 88 as libc::c_int) as usize]
           as libc::c_int & 2 as libc::c_int != 0 {
        let mut i_0: s32 = 0;
        let mut mf_0: MtxF = MtxF{mf: [[0.; 4]; 4],};
        Matrix_MtxToMtxF((*view).viewingPtr, &mut mf_0);
        osSyncPrintf(b"viewing\n\x00" as *const u8 as *const libc::c_char);
        i_0 = 0 as libc::c_int;
        while i_0 < 4 as libc::c_int {
            osSyncPrintf(b"\t%f\t%f\t%f\t%f\n\x00" as *const u8 as
                             *const libc::c_char,
                         mf_0.mf[i_0 as usize][0 as libc::c_int as usize] as
                             libc::c_double,
                         mf_0.mf[i_0 as usize][1 as libc::c_int as usize] as
                             libc::c_double,
                         mf_0.mf[i_0 as usize][2 as libc::c_int as usize] as
                             libc::c_double,
                         mf_0.mf[i_0 as usize][3 as libc::c_int as usize] as
                             libc::c_double);
            i_0 += 1
        }
        osSyncPrintf(b"\n\x00" as *const u8 as *const libc::c_char);
    }
    let fresh10 = (*__gfxCtx).polyOpa.p;
    (*__gfxCtx).polyOpa.p = (*__gfxCtx).polyOpa.p.offset(1);
    let mut _g_5: *mut Gfx = fresh10;
    (*_g_5).words.w0 =
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
            (((0 as libc::c_int | 0 as libc::c_int | 0x4 as libc::c_int) ^
                  0x1 as libc::c_int) as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 0 as libc::c_int;
    (*_g_5).words.w1 = viewing as libc::c_uint;
    let fresh11 = (*__gfxCtx).polyXlu.p;
    (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
    let mut _g_6: *mut Gfx = fresh11;
    (*_g_6).words.w0 =
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
            (((0 as libc::c_int | 0 as libc::c_int | 0x4 as libc::c_int) ^
                  0x1 as libc::c_int) as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 0 as libc::c_int;
    (*_g_6).words.w1 = viewing as libc::c_uint;
    Graph_CloseDisps(dispRefs.as_mut_ptr(), gfxCtx,
                     b"../z_view.c\x00" as *const u8 as *const libc::c_char,
                     711 as libc::c_int);
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn func_800AB0A8(mut view: *mut View) -> s32 {
    let mut vp: *mut Vp = 0 as *mut Vp;
    let mut projection: *mut Mtx = 0 as *mut Mtx;
    let mut gfxCtx: *mut GraphicsContext = (*view).gfxCtx;
    let mut __gfxCtx: *mut GraphicsContext = 0 as *mut GraphicsContext;
    let mut dispRefs: [*mut Gfx; 4] = [0 as *mut Gfx; 4];
    __gfxCtx = gfxCtx;
    Graph_OpenDisps(dispRefs.as_mut_ptr(), gfxCtx,
                    b"../z_view.c\x00" as *const u8 as *const libc::c_char,
                    726 as libc::c_int);
    vp =
        Graph_Alloc(gfxCtx,
                    ::std::mem::size_of::<Vp>() as libc::c_ulong as size_t) as
            *mut Vp;
    LogUtils_CheckNullPointer(b"vp\x00" as *const u8 as *const libc::c_char,
                              vp as *mut libc::c_void,
                              b"../z_view.c\x00" as *const u8 as
                                  *const libc::c_char, 730 as libc::c_int);
    View_ViewportToVp(vp, &mut (*view).viewport);
    (*view).vp = *vp;
    func_800AA550(view);
    let fresh12 = (*__gfxCtx).polyOpa.p;
    (*__gfxCtx).polyOpa.p = (*__gfxCtx).polyOpa.p.offset(1);
    let mut _g: *mut Gfx = fresh12;
    (*_g).words.w0 =
        (0xdc as libc::c_int as u32_0 &
             (((0x1 as libc::c_int) << 8 as libc::c_int) - 1 as libc::c_int)
                 as libc::c_uint) << 24 as libc::c_int |
            ((::std::mem::size_of::<Vp>() as
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
            (8 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 0 as libc::c_int;
    (*_g).words.w1 = vp as libc::c_uint;
    let fresh13 = (*__gfxCtx).polyXlu.p;
    (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
    let mut _g_0: *mut Gfx = fresh13;
    (*_g_0).words.w0 =
        (0xdc as libc::c_int as u32_0 &
             (((0x1 as libc::c_int) << 8 as libc::c_int) - 1 as libc::c_int)
                 as libc::c_uint) << 24 as libc::c_int |
            ((::std::mem::size_of::<Vp>() as
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
            (8 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 0 as libc::c_int;
    (*_g_0).words.w1 = vp as libc::c_uint;
    let fresh14 = (*__gfxCtx).overlay.p;
    (*__gfxCtx).overlay.p = (*__gfxCtx).overlay.p.offset(1);
    let mut _g_1: *mut Gfx = fresh14;
    (*_g_1).words.w0 =
        (0xdc as libc::c_int as u32_0 &
             (((0x1 as libc::c_int) << 8 as libc::c_int) - 1 as libc::c_int)
                 as libc::c_uint) << 24 as libc::c_int |
            ((::std::mem::size_of::<Vp>() as
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
            (8 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 0 as libc::c_int;
    (*_g_1).words.w1 = vp as libc::c_uint;
    projection =
        Graph_Alloc(gfxCtx,
                    ::std::mem::size_of::<Mtx>() as libc::c_ulong as size_t)
            as *mut Mtx;
    LogUtils_CheckNullPointer(b"projection\x00" as *const u8 as
                                  *const libc::c_char,
                              projection as *mut libc::c_void,
                              b"../z_view.c\x00" as *const u8 as
                                  *const libc::c_char, 744 as libc::c_int);
    (*view).projectionPtr = projection;
    guOrtho(projection, -(gScreenWidth as f32_0) * 0.5f32,
            gScreenWidth as f32_0 * 0.5f32,
            -(gScreenHeight as f32_0) * 0.5f32,
            gScreenHeight as f32_0 * 0.5f32, (*view).zNear, (*view).zFar,
            (*view).scale);
    (*view).projection = *projection;
    let fresh15 = (*__gfxCtx).polyOpa.p;
    (*__gfxCtx).polyOpa.p = (*__gfxCtx).polyOpa.p.offset(1);
    let mut _g_2: *mut Gfx = fresh15;
    (*_g_2).words.w0 =
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
            (((0 as libc::c_int | 0x2 as libc::c_int | 0x4 as libc::c_int) ^
                  0x1 as libc::c_int) as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 0 as libc::c_int;
    (*_g_2).words.w1 = projection as libc::c_uint;
    let fresh16 = (*__gfxCtx).polyXlu.p;
    (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
    let mut _g_3: *mut Gfx = fresh16;
    (*_g_3).words.w0 =
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
            (((0 as libc::c_int | 0x2 as libc::c_int | 0x4 as libc::c_int) ^
                  0x1 as libc::c_int) as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 0 as libc::c_int;
    (*_g_3).words.w1 = projection as libc::c_uint;
    Graph_CloseDisps(dispRefs.as_mut_ptr(), gfxCtx,
                     b"../z_view.c\x00" as *const u8 as *const libc::c_char,
                     762 as libc::c_int);
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn func_800AB2C4(mut view: *mut View) -> s32 {
    let mut vp: *mut Vp = 0 as *mut Vp;
    let mut projection: *mut Mtx = 0 as *mut Mtx;
    let mut gfxCtx: *mut GraphicsContext = 0 as *mut GraphicsContext;
    gfxCtx = (*view).gfxCtx;
    let mut __gfxCtx: *mut GraphicsContext = 0 as *mut GraphicsContext;
    let mut dispRefs: [*mut Gfx; 4] = [0 as *mut Gfx; 4];
    __gfxCtx = gfxCtx;
    Graph_OpenDisps(dispRefs.as_mut_ptr(), gfxCtx,
                    b"../z_view.c\x00" as *const u8 as *const libc::c_char,
                    777 as libc::c_int);
    vp =
        Graph_Alloc(gfxCtx,
                    ::std::mem::size_of::<Vp>() as libc::c_ulong as size_t) as
            *mut Vp;
    LogUtils_CheckNullPointer(b"vp\x00" as *const u8 as *const libc::c_char,
                              vp as *mut libc::c_void,
                              b"../z_view.c\x00" as *const u8 as
                                  *const libc::c_char, 781 as libc::c_int);
    View_ViewportToVp(vp, &mut (*view).viewport);
    (*view).vp = *vp;
    let fresh17 = (*__gfxCtx).overlay.p;
    (*__gfxCtx).overlay.p = (*__gfxCtx).overlay.p.offset(1);
    let mut _g: *mut Gfx = fresh17;
    (*_g).words.w0 =
        (0xe7 as libc::c_int as u32_0 &
             (((0x1 as libc::c_int) << 8 as libc::c_int) - 1 as libc::c_int)
                 as libc::c_uint) << 24 as libc::c_int;
    (*_g).words.w1 = 0 as libc::c_int as libc::c_uint;
    let fresh18 = (*__gfxCtx).overlay.p;
    (*__gfxCtx).overlay.p = (*__gfxCtx).overlay.p.offset(1);
    let mut _g_0: *mut Gfx = fresh18;
    (*_g_0).words.w0 =
        (0xed as libc::c_int as u32_0 &
             (((0x1 as libc::c_int) << 8 as libc::c_int) - 1 as libc::c_int)
                 as libc::c_uint) << 24 as libc::c_int |
            (((*view).viewport.leftX as libc::c_float * 4.0f32) as libc::c_int
                 as u32_0 &
                 (((0x1 as libc::c_int) << 12 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 12 as libc::c_int
            |
            (((*view).viewport.topY as libc::c_float * 4.0f32) as libc::c_int
                 as u32_0 &
                 (((0x1 as libc::c_int) << 12 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 0 as libc::c_int;
    (*_g_0).words.w1 =
        (0 as libc::c_int as u32_0 &
             (((0x1 as libc::c_int) << 2 as libc::c_int) - 1 as libc::c_int)
                 as libc::c_uint) << 24 as libc::c_int |
            (((*view).viewport.rightX as libc::c_float * 4.0f32) as
                 libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 12 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 12 as libc::c_int
            |
            (((*view).viewport.bottomY as libc::c_float * 4.0f32) as
                 libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 12 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 0 as libc::c_int;
    let fresh19 = (*__gfxCtx).overlay.p;
    (*__gfxCtx).overlay.p = (*__gfxCtx).overlay.p.offset(1);
    let mut _g_1: *mut Gfx = fresh19;
    (*_g_1).words.w0 =
        (0xdc as libc::c_int as u32_0 &
             (((0x1 as libc::c_int) << 8 as libc::c_int) - 1 as libc::c_int)
                 as libc::c_uint) << 24 as libc::c_int |
            ((::std::mem::size_of::<Vp>() as
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
            (8 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 0 as libc::c_int;
    (*_g_1).words.w1 = vp as libc::c_uint;
    projection =
        Graph_Alloc(gfxCtx,
                    ::std::mem::size_of::<Mtx>() as libc::c_ulong as size_t)
            as *mut Mtx;
    LogUtils_CheckNullPointer(b"projection\x00" as *const u8 as
                                  *const libc::c_char,
                              projection as *mut libc::c_void,
                              b"../z_view.c\x00" as *const u8 as
                                  *const libc::c_char, 791 as libc::c_int);
    (*view).projectionPtr = projection;
    guOrtho(projection, -(gScreenWidth as f32_0) * 0.5f32,
            gScreenWidth as f32_0 * 0.5f32,
            -(gScreenHeight as f32_0) * 0.5f32,
            gScreenHeight as f32_0 * 0.5f32, (*view).zNear, (*view).zFar,
            (*view).scale);
    (*view).projection = *projection;
    let fresh20 = (*__gfxCtx).overlay.p;
    (*__gfxCtx).overlay.p = (*__gfxCtx).overlay.p.offset(1);
    let mut _g_2: *mut Gfx = fresh20;
    (*_g_2).words.w0 =
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
            (((0 as libc::c_int | 0x2 as libc::c_int | 0x4 as libc::c_int) ^
                  0x1 as libc::c_int) as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 0 as libc::c_int;
    (*_g_2).words.w1 = projection as libc::c_uint;
    Graph_CloseDisps(dispRefs.as_mut_ptr(), gfxCtx,
                     b"../z_view.c\x00" as *const u8 as *const libc::c_char,
                     801 as libc::c_int);
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn func_800AB560(mut view: *mut View) -> s32 {
    let mut pad: [s32; 2] = [0; 2];
    let mut aspect: f32_0 = 0.;
    let mut width: s32 = 0;
    let mut height: s32 = 0;
    let mut vp: *mut Vp = 0 as *mut Vp;
    let mut projection: *mut Mtx = 0 as *mut Mtx;
    let mut viewing: *mut Mtx = 0 as *mut Mtx;
    let mut gfxCtx: *mut GraphicsContext = (*view).gfxCtx;
    let mut __gfxCtx: *mut GraphicsContext = 0 as *mut GraphicsContext;
    let mut dispRefs: [*mut Gfx; 4] = [0 as *mut Gfx; 4];
    __gfxCtx = gfxCtx;
    Graph_OpenDisps(dispRefs.as_mut_ptr(), gfxCtx,
                    b"../z_view.c\x00" as *const u8 as *const libc::c_char,
                    816 as libc::c_int);
    vp =
        Graph_Alloc(gfxCtx,
                    ::std::mem::size_of::<Vp>() as libc::c_ulong as size_t) as
            *mut Vp;
    LogUtils_CheckNullPointer(b"vp\x00" as *const u8 as *const libc::c_char,
                              vp as *mut libc::c_void,
                              b"../z_view.c\x00" as *const u8 as
                                  *const libc::c_char, 821 as libc::c_int);
    View_ViewportToVp(vp, &mut (*view).viewport);
    (*view).vp = *vp;
    let fresh21 = (*__gfxCtx).overlay.p;
    (*__gfxCtx).overlay.p = (*__gfxCtx).overlay.p.offset(1);
    let mut _g: *mut Gfx = fresh21;
    (*_g).words.w0 =
        (0xe7 as libc::c_int as u32_0 &
             (((0x1 as libc::c_int) << 8 as libc::c_int) - 1 as libc::c_int)
                 as libc::c_uint) << 24 as libc::c_int;
    (*_g).words.w1 = 0 as libc::c_int as libc::c_uint;
    let fresh22 = (*__gfxCtx).overlay.p;
    (*__gfxCtx).overlay.p = (*__gfxCtx).overlay.p.offset(1);
    let mut _g_0: *mut Gfx = fresh22;
    (*_g_0).words.w0 =
        (0xed as libc::c_int as u32_0 &
             (((0x1 as libc::c_int) << 8 as libc::c_int) - 1 as libc::c_int)
                 as libc::c_uint) << 24 as libc::c_int |
            (((*view).viewport.leftX as libc::c_float * 4.0f32) as libc::c_int
                 as u32_0 &
                 (((0x1 as libc::c_int) << 12 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 12 as libc::c_int
            |
            (((*view).viewport.topY as libc::c_float * 4.0f32) as libc::c_int
                 as u32_0 &
                 (((0x1 as libc::c_int) << 12 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 0 as libc::c_int;
    (*_g_0).words.w1 =
        (0 as libc::c_int as u32_0 &
             (((0x1 as libc::c_int) << 2 as libc::c_int) - 1 as libc::c_int)
                 as libc::c_uint) << 24 as libc::c_int |
            (((*view).viewport.rightX as libc::c_float * 4.0f32) as
                 libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 12 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 12 as libc::c_int
            |
            (((*view).viewport.bottomY as libc::c_float * 4.0f32) as
                 libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 12 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 0 as libc::c_int;
    let fresh23 = (*__gfxCtx).overlay.p;
    (*__gfxCtx).overlay.p = (*__gfxCtx).overlay.p.offset(1);
    let mut _g_1: *mut Gfx = fresh23;
    (*_g_1).words.w0 =
        (0xdc as libc::c_int as u32_0 &
             (((0x1 as libc::c_int) << 8 as libc::c_int) - 1 as libc::c_int)
                 as libc::c_uint) << 24 as libc::c_int |
            ((::std::mem::size_of::<Vp>() as
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
            (8 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 0 as libc::c_int;
    (*_g_1).words.w1 = vp as libc::c_uint;
    projection =
        Graph_Alloc(gfxCtx,
                    ::std::mem::size_of::<Mtx>() as libc::c_ulong as size_t)
            as *mut Mtx;
    LogUtils_CheckNullPointer(b"projection\x00" as *const u8 as
                                  *const libc::c_char,
                              projection as *mut libc::c_void,
                              b"../z_view.c\x00" as *const u8 as
                                  *const libc::c_char, 833 as libc::c_int);
    (*view).projectionPtr = projection;
    width = (*view).viewport.rightX - (*view).viewport.leftX;
    height = (*view).viewport.bottomY - (*view).viewport.topY;
    aspect = width as f32_0 / height as f32_0;
    guPerspective(projection, &mut (*view).normal, (*view).fovy, aspect,
                  (*view).zNear, (*view).zFar, (*view).scale);
    (*view).projection = *projection;
    let fresh24 = (*__gfxCtx).overlay.p;
    (*__gfxCtx).overlay.p = (*__gfxCtx).overlay.p.offset(1);
    let mut _g_2: *mut Gfx = fresh24;
    (*_g_2).words.w0 =
        (0xdb as libc::c_int as u32_0 &
             (((0x1 as libc::c_int) << 8 as libc::c_int) - 1 as libc::c_int)
                 as libc::c_uint) << 24 as libc::c_int |
            (0xe as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 16 as libc::c_int
            |
            (0 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 16 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 0 as libc::c_int;
    (*_g_2).words.w1 = (*view).normal as libc::c_uint;
    let fresh25 = (*__gfxCtx).overlay.p;
    (*__gfxCtx).overlay.p = (*__gfxCtx).overlay.p.offset(1);
    let mut _g_3: *mut Gfx = fresh25;
    (*_g_3).words.w0 =
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
            (((0 as libc::c_int | 0x2 as libc::c_int | 0x4 as libc::c_int) ^
                  0x1 as libc::c_int) as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 0 as libc::c_int;
    (*_g_3).words.w1 = projection as libc::c_uint;
    viewing =
        Graph_Alloc(gfxCtx,
                    ::std::mem::size_of::<Mtx>() as libc::c_ulong as size_t)
            as *mut Mtx;
    LogUtils_CheckNullPointer(b"viewing\x00" as *const u8 as
                                  *const libc::c_char,
                              viewing as *mut libc::c_void,
                              b"../z_view.c\x00" as *const u8 as
                                  *const libc::c_char, 848 as libc::c_int);
    (*view).viewingPtr = viewing;
    if (*view).eye.x == (*view).lookAt.x && (*view).eye.y == (*view).lookAt.y
           && (*view).eye.z == (*view).lookAt.z {
        (*view).eye.x += 1.0f32;
        (*view).eye.y += 1.0f32;
        (*view).eye.z += 1.0f32
    }
    func_800ABE74((*view).eye.x, (*view).eye.y, (*view).eye.z);
    guLookAt(viewing, (*view).eye.x, (*view).eye.y, (*view).eye.z,
             (*view).lookAt.x, (*view).lookAt.y, (*view).lookAt.z,
             (*view).up.x, (*view).up.y, (*view).up.z);
    (*view).viewing = *viewing;
    let fresh26 = (*__gfxCtx).overlay.p;
    (*__gfxCtx).overlay.p = (*__gfxCtx).overlay.p.offset(1);
    let mut _g_4: *mut Gfx = fresh26;
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
            (((0 as libc::c_int | 0 as libc::c_int | 0x4 as libc::c_int) ^
                  0x1 as libc::c_int) as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 0 as libc::c_int;
    (*_g_4).words.w1 = viewing as libc::c_uint;
    Graph_CloseDisps(dispRefs.as_mut_ptr(), gfxCtx,
                     b"../z_view.c\x00" as *const u8 as *const libc::c_char,
                     871 as libc::c_int);
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn func_800AB944(mut view: *mut View) -> s32 {
    let mut __gfxCtx: *mut GraphicsContext = 0 as *mut GraphicsContext;
    let mut dispRefs: [*mut Gfx; 4] = [0 as *mut Gfx; 4];
    __gfxCtx = (*view).gfxCtx;
    Graph_OpenDisps(dispRefs.as_mut_ptr(), (*view).gfxCtx,
                    b"../z_view.c\x00" as *const u8 as *const libc::c_char,
                    878 as libc::c_int);
    func_800ABE74((*view).eye.x, (*view).eye.y, (*view).eye.z);
    guLookAt((*view).viewingPtr, (*view).eye.x, (*view).eye.y, (*view).eye.z,
             (*view).lookAt.x, (*view).lookAt.y, (*view).lookAt.z,
             (*view).up.x, (*view).up.y, (*view).up.z);
    Graph_CloseDisps(dispRefs.as_mut_ptr(), (*view).gfxCtx,
                     b"../z_view.c\x00" as *const u8 as *const libc::c_char,
                     886 as libc::c_int);
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn func_800AB9EC(mut view: *mut View, mut arg1: s32,
                                       mut gfxp: *mut *mut Gfx) -> s32 {
    let mut gfx: *mut Gfx = *gfxp;
    let mut gfxCtx: *mut GraphicsContext = (*view).gfxCtx;
    let mut width: s32 = 0;
    let mut height: s32 = 0;
    let mut vp: *mut Vp = 0 as *mut Vp;
    let mut projection: *mut Mtx = 0 as *mut Mtx;
    let mut viewing: *mut Mtx = 0 as *mut Mtx;
    arg1 = (*view).flags & arg1 | arg1 >> 4 as libc::c_int;
    if arg1 & 2 as libc::c_int != 0 {
        vp =
            Graph_Alloc(gfxCtx,
                        ::std::mem::size_of::<Vp>() as libc::c_ulong as
                            size_t) as *mut Vp;
        LogUtils_CheckNullPointer(b"vp\x00" as *const u8 as
                                      *const libc::c_char,
                                  vp as *mut libc::c_void,
                                  b"../z_view.c\x00" as *const u8 as
                                      *const libc::c_char,
                                  910 as libc::c_int);
        View_ViewportToVp(vp, &mut (*view).viewport);
        (*view).vp = *vp;
        let fresh27 = gfx;
        gfx = gfx.offset(1);
        let mut _g: *mut Gfx = fresh27;
        (*_g).words.w0 =
            (0xe7 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 24 as libc::c_int;
        (*_g).words.w1 = 0 as libc::c_int as libc::c_uint;
        let fresh28 = gfx;
        gfx = gfx.offset(1);
        let mut _g_0: *mut Gfx = fresh28;
        (*_g_0).words.w0 =
            (0xed as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 24 as libc::c_int
                |
                (((*view).viewport.leftX as libc::c_float * 4.0f32) as
                     libc::c_int as u32_0 &
                     (((0x1 as libc::c_int) << 12 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    12 as libc::c_int |
                (((*view).viewport.topY as libc::c_float * 4.0f32) as
                     libc::c_int as u32_0 &
                     (((0x1 as libc::c_int) << 12 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    0 as libc::c_int;
        (*_g_0).words.w1 =
            (0 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 2 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 24 as libc::c_int
                |
                (((*view).viewport.rightX as libc::c_float * 4.0f32) as
                     libc::c_int as u32_0 &
                     (((0x1 as libc::c_int) << 12 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    12 as libc::c_int |
                (((*view).viewport.bottomY as libc::c_float * 4.0f32) as
                     libc::c_int as u32_0 &
                     (((0x1 as libc::c_int) << 12 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    0 as libc::c_int;
        let fresh29 = gfx;
        gfx = gfx.offset(1);
        let mut _g_1: *mut Gfx = fresh29;
        (*_g_1).words.w0 =
            (0xdc as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 24 as libc::c_int
                |
                ((::std::mem::size_of::<Vp>() as
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
                (8 as libc::c_int as u32_0 &
                     (((0x1 as libc::c_int) << 8 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    0 as libc::c_int;
        (*_g_1).words.w1 = vp as libc::c_uint
    }
    if arg1 & 8 as libc::c_int != 0 {
        projection =
            Graph_Alloc(gfxCtx,
                        ::std::mem::size_of::<Mtx>() as libc::c_ulong as
                            size_t) as *mut Mtx;
        LogUtils_CheckNullPointer(b"projection\x00" as *const u8 as
                                      *const libc::c_char,
                                  projection as *mut libc::c_void,
                                  b"../z_view.c\x00" as *const u8 as
                                      *const libc::c_char,
                                  921 as libc::c_int);
        (*view).projectionPtr = projection;
        guOrtho(projection, -(gScreenWidth as f32_0) * 0.5f32,
                gScreenWidth as f32_0 * 0.5f32,
                -(gScreenHeight as f32_0) * 0.5f32,
                gScreenHeight as f32_0 * 0.5f32, (*view).zNear, (*view).zFar,
                (*view).scale);
        (*view).projection = *projection;
        let fresh30 = gfx;
        gfx = gfx.offset(1);
        let mut _g_2: *mut Gfx = fresh30;
        (*_g_2).words.w0 =
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
                (((0 as libc::c_int | 0x2 as libc::c_int | 0x4 as libc::c_int)
                      ^ 0x1 as libc::c_int) as u32_0 &
                     (((0x1 as libc::c_int) << 8 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    0 as libc::c_int;
        (*_g_2).words.w1 = projection as libc::c_uint
    } else if arg1 & 6 as libc::c_int != 0 {
        projection =
            Graph_Alloc(gfxCtx,
                        ::std::mem::size_of::<Mtx>() as libc::c_ulong as
                            size_t) as *mut Mtx;
        LogUtils_CheckNullPointer(b"projection\x00" as *const u8 as
                                      *const libc::c_char,
                                  projection as *mut libc::c_void,
                                  b"../z_view.c\x00" as *const u8 as
                                      *const libc::c_char,
                                  932 as libc::c_int);
        (*view).projectionPtr = projection;
        width = (*view).viewport.rightX - (*view).viewport.leftX;
        height = (*view).viewport.bottomY - (*view).viewport.topY;
        guPerspective(projection, &mut (*view).normal, (*view).fovy,
                      width as f32_0 / height as f32_0, (*view).zNear,
                      (*view).zFar, (*view).scale);
        (*view).projection = *projection;
        let fresh31 = gfx;
        gfx = gfx.offset(1);
        let mut _g_3: *mut Gfx = fresh31;
        (*_g_3).words.w0 =
            (0xdb as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 24 as libc::c_int
                |
                (0xe as libc::c_int as u32_0 &
                     (((0x1 as libc::c_int) << 8 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    16 as libc::c_int |
                (0 as libc::c_int as u32_0 &
                     (((0x1 as libc::c_int) << 16 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    0 as libc::c_int;
        (*_g_3).words.w1 = (*view).normal as libc::c_uint;
        let fresh32 = gfx;
        gfx = gfx.offset(1);
        let mut _g_4: *mut Gfx = fresh32;
        (*_g_4).words.w0 =
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
                (((0 as libc::c_int | 0x2 as libc::c_int | 0x4 as libc::c_int)
                      ^ 0x1 as libc::c_int) as u32_0 &
                     (((0x1 as libc::c_int) << 8 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    0 as libc::c_int;
        (*_g_4).words.w1 = projection as libc::c_uint
    }
    if arg1 & 1 as libc::c_int != 0 {
        viewing =
            Graph_Alloc(gfxCtx,
                        ::std::mem::size_of::<Mtx>() as libc::c_ulong as
                            size_t) as *mut Mtx;
        LogUtils_CheckNullPointer(b"viewing\x00" as *const u8 as
                                      *const libc::c_char,
                                  viewing as *mut libc::c_void,
                                  b"../z_view.c\x00" as *const u8 as
                                      *const libc::c_char,
                                  948 as libc::c_int);
        (*view).viewingPtr = viewing;
        func_800ABE74((*view).eye.x, (*view).eye.y, (*view).eye.z);
        guLookAt(viewing, (*view).eye.x, (*view).eye.y, (*view).eye.z,
                 (*view).lookAt.x, (*view).lookAt.y, (*view).lookAt.z,
                 (*view).up.x, (*view).up.y, (*view).up.z);
        (*view).viewing = *viewing;
        let fresh33 = gfx;
        gfx = gfx.offset(1);
        let mut _g_5: *mut Gfx = fresh33;
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
                (((0 as libc::c_int | 0 as libc::c_int | 0x4 as libc::c_int) ^
                      0x1 as libc::c_int) as u32_0 &
                     (((0x1 as libc::c_int) << 8 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    0 as libc::c_int;
        (*_g_5).words.w1 = viewing as libc::c_uint
    }
    (*view).flags = 0 as libc::c_int;
    *gfxp = gfx;
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn func_800ABE74(mut eyeX: f32_0, mut eyeY: f32_0,
                                       mut eyeZ: f32_0) -> s32 {
    let mut error: s32 = 0 as libc::c_int;
    if eyeX * eyeX + eyeY * eyeY + eyeZ * eyeZ > 32767.0f32 * 32767.0f32 {
        error = 3 as libc::c_int
    } else {
        let mut absEyeX: f32_0 =
            if eyeX >= 0 as libc::c_int as libc::c_float {
                eyeX
            } else { -eyeX };
        let mut absEyeY: f32_0 =
            if eyeY >= 0 as libc::c_int as libc::c_float {
                eyeY
            } else { -eyeY };
        let mut absEyeZ: f32_0 =
            if eyeZ >= 0 as libc::c_int as libc::c_float {
                eyeZ
            } else { -eyeZ };
        if 18900.0f32 < absEyeX || 18900.0f32 < absEyeY ||
               18900.0f32 < absEyeZ {
            error = 2 as libc::c_int
        } else if 16000.0f32 < absEyeX || 16000.0f32 < absEyeY ||
                      16000.0f32 < absEyeZ {
            error = 1 as libc::c_int
        }
    }
    if error != 0 as libc::c_int {
        osSyncPrintf(b"\x1b[31m\x00" as *const u8 as *const libc::c_char);
        // "Is too large"
        osSyncPrintf(b"eye \xe3\x81\x8c\xe5\xa4\xa7\xe3\x81\x8d\xe3\x81\x99\xe3\x81\x8e\xe3\x81\xbe\xe3\x81\x99 eye=[%8.3f %8.3f %8.3f] error=%d\n\x00"
                         as *const u8 as *const libc::c_char,
                     eyeX as libc::c_double, eyeY as libc::c_double,
                     eyeZ as libc::c_double, error);
        osSyncPrintf(b"\x1b[m\x00" as *const u8 as *const libc::c_char);
    }
    return error;
}
