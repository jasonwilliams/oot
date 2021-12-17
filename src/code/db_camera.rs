#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case,
         non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(const_raw_ptr_to_usize_cast, const_transmute, register_tool)]
extern "C" {
    #[no_mangle]
    fn osSyncPrintf(fmt: *const libc::c_char, _: ...);
    #[no_mangle]
    fn func_8006376C(x: u8_0, y: u8_0, colorId: u8_0,
                     text: *const libc::c_char);
    #[no_mangle]
    fn DebugDisplay_AddObject(posX: f32_0, posY: f32_0, posZ: f32_0,
                              rotX: s16, rotY: s16, rotZ: s16, scaleX: f32_0,
                              scaleY: f32_0, scaleZ: f32_0, red: u8_0,
                              green: u8_0, blue: u8_0, alpha: u8_0,
                              type_0: s16, gfxCtx: *mut GraphicsContext)
     -> *mut DebugDispObject;
    #[no_mangle]
    fn Math_CosS(angle: s16) -> f32_0;
    #[no_mangle]
    fn Math_SinS(angle: s16) -> f32_0;
    #[no_mangle]
    fn OLib_Vec3fDist(a: *mut Vec3f, b: *mut Vec3f) -> f32_0;
    #[no_mangle]
    fn OLib_VecSphGeoToVec3f(dest: *mut Vec3f, sph: *mut VecSph)
     -> *mut Vec3f;
    #[no_mangle]
    fn OLib_Vec3fToVecSphGeo(arg0: *mut VecSph, arg1: *mut Vec3f)
     -> *mut VecSph;
    #[no_mangle]
    fn OLib_Vec3fDiffToVecSphGeo(arg0: *mut VecSph, a: *mut Vec3f,
                                 b: *mut Vec3f) -> *mut VecSph;
    #[no_mangle]
    fn Interface_ChangeAlpha(alphaType: u16_0);
    #[no_mangle]
    fn ShrinkWindow_SetVal(value: s32);
    #[no_mangle]
    fn func_800BB2B4(pos: *mut Vec3f, roll: *mut f32_0, fov: *mut f32_0,
                     point: *mut CutsceneCameraPoint, keyframe: *mut s16,
                     curFrame: *mut f32_0) -> s32;
    #[no_mangle]
    fn Mempak_Init(controllerNb: s32) -> s32;
    #[no_mangle]
    fn Mempak_GetFreeBytes(controllerNb: s32) -> s32;
    #[no_mangle]
    fn Mempak_FindFile(controllerNb: s32, start: libc::c_char,
                       end: libc::c_char) -> s32;
    #[no_mangle]
    fn Mempak_Write(controllerNb: s32, idx: libc::c_char,
                    buffer: *mut libc::c_void, offset: s32, size: s32) -> s32;
    #[no_mangle]
    fn Mempak_Read(controllerNb: s32, idx: libc::c_char,
                   buffer: *mut libc::c_void, offset: s32, size: s32) -> s32;
    #[no_mangle]
    fn Mempak_Alloc(controllerNb: s32, idx: *mut libc::c_char, size: s32)
     -> s32;
    #[no_mangle]
    fn Mempak_DeleteFile(controllerNb: s32, idx: libc::c_char) -> s32;
    #[no_mangle]
    fn Mempak_GetFileSize(controllerNb: s32, idx: libc::c_char) -> s32;
    #[no_mangle]
    fn DebugArena_MallocDebug(size: u32_0, file: *const libc::c_char,
                              line: s32) -> *mut libc::c_void;
    #[no_mangle]
    fn DebugArena_FreeDebug(ptr: *mut libc::c_void, file: *const libc::c_char,
                            line: s32);
    #[no_mangle]
    static mut D_801333E8: s8;
    #[no_mangle]
    static mut D_801333E0: f32_0;
    #[no_mangle]
    static mut D_801333D4: Vec3f;
    #[no_mangle]
    fn Audio_PlaySoundGeneral(sfxId: u16_0, pos: *mut Vec3f, token: u8_0,
                              freqScale: *mut f32_0, a4: *mut f32_0,
                              reverbAdd: *mut s8);
    #[no_mangle]
    static mut gGameInfo: *mut GameInfo;
    #[no_mangle]
    static mut gSaveContext: SaveContext;
    #[no_mangle]
    static mut D_8015FCC8: u8_0;
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
pub struct VecSph {
    pub r: f32_0,
    pub pitch: s16,
    pub yaw: s16,
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Color_RGB8 {
    pub r: u8_0,
    pub g: u8_0,
    pub b: u8_0,
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
pub struct DbCameraSub {
    pub mode: s16,
    pub nFrames: s16,
    pub nPoints: s16,
    pub unkIdx: s16,
    pub unk_08: s16,
    pub unk_0A: s16,
    pub unk_0C: s32,
    pub unk_10: [libc::c_char; 20],
    pub position: [CutsceneCameraPoint; 129],
    pub lookAt: [CutsceneCameraPoint; 129],
    pub demoCtrlMenu: s16,
    pub demoCtrlActionIdx: s16,
    pub demoCtrlToggleSwitch: s16,
    pub unk_104A: Vec3s,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct DbCamera {
    pub unk_00: s32,
    pub at: Vec3f,
    pub eye: Vec3f,
    pub unk_1C: Vec3f,
    pub unk_28: [libc::c_char; 12],
    pub unk_34: s32,
    pub unk_38: s32,
    pub unk_3C: s32,
    pub unk_40: s32,
    pub unk_44: s32,
    pub fov: f32_0,
    pub roll: s16,
    pub unk_4E: [libc::c_char; 2],
    pub rollDegrees: f32_0,
    pub unk_54: Vec3f,
    pub unk_60: Vec3f,
    pub unk_6C: Vec3f,
    pub unk_78: s16,
    pub unk_7A: s16,
    pub sub: DbCameraSub,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct DbCameraCut {
    pub letter: libc::c_char,
    pub unk_01: u8_0,
    pub mode: s16,
    pub position: *mut CutsceneCameraPoint,
    pub lookAt: *mut CutsceneCameraPoint,
    pub nFrames: s16,
    pub nPoints: s16,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct DbCameraAnim {
    pub curFrame: f32_0,
    pub unk_04: f32_0,
    pub keyframe: s16,
    pub unk_0A: s16,
    pub unk_0C: s16,
    pub positionPos: Vec3f,
    pub lookAtPos: Vec3f,
    pub roll: f32_0,
    pub fov: f32_0,
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
pub struct DebugDispObject {
    pub pos: Vec3f,
    pub rot: Vec3s,
    pub scale: Vec3f,
    pub color: Color_RGBA8,
    pub type_0: s16,
    pub next: *mut DebugDispObject,
}
static mut sGlobalCtx: *mut GlobalContext =
    0 as *const GlobalContext as *mut GlobalContext;
// TODO: cleanup these arrays and UB access
#[no_mangle]
pub static mut D_8012CEE0: [*mut libc::c_char; 1] =
    [b"\x8c\xef\xbd\xb7-\xef\xbe\x8c\xef\xbe\x9a-\xef\xbe\x91\x8d\xef\xbd\xb6\xef\xbe\x9e\x00"
         as *const u8 as *const libc::c_char as *mut libc::c_char];
#[no_mangle]
pub static mut D_8012CEE4: *mut libc::c_char =
    b"\x8d\xef\xbe\x80\xef\xbe\x98\xef\xbe\x8f\xef\xbd\xbe\xef\xbe\x9d\xef\xbd\xa1\x00"
        as *const u8 as *const libc::c_char as *mut libc::c_char;
#[no_mangle]
pub static mut D_8012CEE8: *mut libc::c_char =
    b"\x8d\xef\xbd\xbb\xef\xbd\xb2\xef\xbd\xbe\xef\xbd\xb2\xef\xbe\x83\xef\xbe\x9e\xef\xbd\xb7\xef\xbe\x8f\xef\xbd\xbe\xef\xbe\x9d\x00"
        as *const u8 as *const libc::c_char as *mut libc::c_char;
#[no_mangle]
pub static mut D_8012CEEC: *mut libc::c_char =
    b"\x8d\xef\xbd\xbb\xef\xbd\xb2\xef\xbd\xbe\xef\xbd\xb2\xef\xbd\xbc\xef\xbd\xad\xef\xbd\xb3\xef\xbe\x98\xef\xbd\xae\xef\xbd\xb3\x00"
        as *const u8 as *const libc::c_char as *mut libc::c_char;
#[no_mangle]
pub static mut D_8012CEF0: *mut libc::c_char =
    b"\x8d\xef\xbd\xbb\xef\xbd\xb2\xef\xbd\xbe\xef\xbd\xb2\xef\xbe\x81\xef\xbd\xad\xef\xbd\xb3!\x00"
        as *const u8 as *const libc::c_char as *mut libc::c_char;
#[no_mangle]
pub static mut D_8012CEF4: *mut libc::c_char =
    b"DEMO CAMERA TOOL\x00" as *const u8 as *const libc::c_char as
        *mut libc::c_char;
#[no_mangle]
pub static mut D_8012CEF8: [*mut libc::c_char; 3] =
    [b"\x8d\xef\xbe\x93\xef\xbd\xb3\xef\xbe\x8a\xef\xbd\xb2\xef\xbe\x98\xef\xbe\x8f\xef\xbd\xbe\xef\xbe\x9d\x00"
         as *const u8 as *const libc::c_char as *mut libc::c_char,
     b"\x8d\xef\xbe\x84\xef\xbd\xb3\xef\xbe\x9b\xef\xbd\xb8   \xef\xbe\x83\xef\xbe\x9d\xef\xbe\x92\x00"
         as *const u8 as *const libc::c_char as *mut libc::c_char,
     b"\x8d\xef\xbe\x8d\xef\xbe\x9d\xef\xbd\xba\xef\xbd\xb3   /  \x00" as
         *const u8 as *const libc::c_char as *mut libc::c_char];
#[no_mangle]
pub static mut D_8012CF04: *mut libc::c_char =
    b">            >\x00" as *const u8 as *const libc::c_char as
        *mut libc::c_char;
#[no_mangle]
pub static mut D_8012CF08: *mut libc::c_char =
    b"<            <\x00" as *const u8 as *const libc::c_char as
        *mut libc::c_char;
#[no_mangle]
pub static mut D_8012CF0C: *mut libc::c_char =
    b"<          >\x00" as *const u8 as *const libc::c_char as
        *mut libc::c_char;
#[no_mangle]
pub static mut D_8012CF10: *mut libc::c_char =
    b"\x8c*\xef\xbe\x8c\xef\xbe\x9f\xef\xbe\x9a\xef\xbd\xb2\xef\xbe\x94-*\x00"
        as *const u8 as *const libc::c_char as *mut libc::c_char;
#[no_mangle]
pub static mut D_8012CF14: *mut libc::c_char =
    b"\x8cE\xef\xbe\x93-\xef\xbe\x84\xef\xbe\x9e\x8d \xef\xbd\xbf\xef\xbd\xb3\xef\xbe\x80\xef\xbd\xb2\x00"
        as *const u8 as *const libc::c_char as *mut libc::c_char;
#[no_mangle]
pub static mut D_8012CF18: [*mut libc::c_char; 3] =
    [b"\x8cE\xef\xbe\x93-\xef\xbe\x84\xef\xbe\x9e\x8d\xef\xbd\xbe\xef\xbe\x9e\xef\xbd\xaf\xef\xbe\x80\xef\xbd\xb2\x00"
         as *const u8 as *const libc::c_char as *mut libc::c_char,
     b"\x8d\xef\xbd\xb6\xef\xbe\x9e\xef\xbe\x92\xef\xbe\x9d\x8c   \xef\xbe\x83\xef\xbe\x9e\xef\xbe\x93\x00"
         as *const u8 as *const libc::c_char as *mut libc::c_char,
     b"\x8d\xef\xbd\xb6\xef\xbe\x9e\xef\xbe\x92\xef\xbe\x9d   \xef\xbe\x8c\xef\xbe\x82\xef\xbd\xb3\x00"
         as *const u8 as *const libc::c_char as *mut libc::c_char];
#[no_mangle]
pub static mut D_8012CF24: [*mut libc::c_char; 3] =
    [b"\x8dP\xef\xbd\xbc\xef\xbe\x9e\xef\xbd\xb6\xef\xbe\x9d  MAX\x00" as
         *const u8 as *const libc::c_char as *mut libc::c_char,
     b"\x8c\xef\xbe\x98\xef\xbe\x9d\xef\xbd\xb8\x8d    \xef\xbd\xb7\xef\xbd\xb5\xef\xbd\xb8\x00"
         as *const u8 as *const libc::c_char as *mut libc::c_char,
     b"\x8c\xef\xbe\x98\xef\xbe\x9d\xef\xbd\xb8\x8d     \xef\xbe\x91\xef\xbd\xbc\x00"
         as *const u8 as *const libc::c_char as *mut libc::c_char];
#[no_mangle]
pub static mut D_8012CF30: *mut libc::c_char =
    b"\x8d*\xef\xbe\x90\xef\xbe\x83\xef\xbe\x99\xef\xbd\xb2\xef\xbe\x81*\x00"
        as *const u8 as *const libc::c_char as *mut libc::c_char;
#[no_mangle]
pub static mut D_8012CF34: *mut libc::c_char =
    b"\x8c*\xef\xbd\xb6\xef\xbe\x92\xef\xbe\x97\x8d\xef\xbd\xb2\xef\xbe\x81*\x00"
        as *const u8 as *const libc::c_char as *mut libc::c_char;
#[no_mangle]
pub static mut D_8012CF38: *mut libc::c_char =
    b"DEBUG CAMERA\x00" as *const u8 as *const libc::c_char as
        *mut libc::c_char;
#[no_mangle]
pub static mut D_8012CF3C: *mut libc::c_char =
    b"\x8c\xef\xbd\xbe\xef\xbe\x9d\xef\xbe\x80-/\xef\xbe\x9b\xef\xbd\xaf\xef\xbd\xb8\x00"
        as *const u8 as *const libc::c_char as *mut libc::c_char;
#[no_mangle]
pub static mut D_8012CF40: *mut libc::c_char =
    b"\x8c\xef\xbd\xbe\xef\xbe\x9d\xef\xbe\x80-/\xef\xbe\x8c\xef\xbe\x98-\x00"
        as *const u8 as *const libc::c_char as *mut libc::c_char;
#[no_mangle]
pub static mut D_8012CF44: *mut libc::c_char =
    b"DEMO CONTROL\x00" as *const u8 as *const libc::c_char as
        *mut libc::c_char;
#[no_mangle]
pub static mut D_8012CF48: *mut libc::c_char =
    b"\x8c\xef\xbe\x92\xef\xbe\x93\xef\xbe\x98\x8d\xef\xbd\xb6\xef\xbe\x9e\xef\xbe\x80\xef\xbe\x98\xef\xbe\x8f\xef\xbd\xbe\xef\xbe\x9d\x00"
        as *const u8 as *const libc::c_char as *mut libc::c_char;
#[no_mangle]
pub static mut D_8012CF4C: *mut libc::c_char =
    b"p\x00" as *const u8 as *const libc::c_char as *mut libc::c_char;
#[no_mangle]
pub static mut D_8012CF50: [*mut libc::c_char; 4] =
    [b"e\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
     b"s\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
     b"l\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
     b"c\x00" as *const u8 as *const libc::c_char as *mut libc::c_char];
#[no_mangle]
pub static mut D_8012CF60: [*mut libc::c_char; 1] =
    [b"\x8c\xef\xbe\x92\xef\xbe\x93\xef\xbe\x98\xef\xbe\x8a\xef\xbe\x9f\xef\xbd\xaf\xef\xbd\xb8\x00"
         as *const u8 as *const libc::c_char as *mut libc::c_char];
// "Mempak"
#[no_mangle]
pub static mut D_8012CF64: *mut libc::c_char =
    b"\x8c\xef\xbd\xbe\xef\xbd\xb0\xef\xbe\x8c\xef\xbe\x9e\x00" as *const u8
        as *const libc::c_char as *mut libc::c_char;
// "Save"
#[no_mangle]
pub static mut D_8012CF68: *mut libc::c_char =
    b"\x8c\xef\xbe\x9b\xef\xbd\xb0\xef\xbe\x84\xef\xbe\x9e\x00" as *const u8
        as *const libc::c_char as *mut libc::c_char;
// "Load"
#[no_mangle]
pub static mut D_8012CF6C: *mut libc::c_char =
    b"\x8c\xef\xbd\xb8\xef\xbe\x98\xef\xbd\xb1-\x00" as *const u8 as
        *const libc::c_char as *mut libc::c_char;
// "Clear"
#[no_mangle]
pub static mut D_8012CF70: *mut libc::c_char =
    b"\x8d\xef\xbd\xa6\xef\xbe\x87\xef\xbd\xb6\xef\xbe\x85\xef\xbd\xb2\xef\xbe\x83\xef\xbe\x9e\xef\xbe\x88\x00"
        as *const u8 as *const libc::c_char as *mut libc::c_char;
#[no_mangle]
pub static mut D_8012CF74: *mut libc::c_char =
    b"FREE      BYTE\x00" as *const u8 as *const libc::c_char as
        *mut libc::c_char;
#[no_mangle]
pub static mut D_8012CF78: *mut libc::c_char =
    b"NEED      BYTE\x00" as *const u8 as *const libc::c_char as
        *mut libc::c_char;
#[no_mangle]
pub static mut D_8012CF7C: *mut libc::c_char =
    b"\x8c*\xef\xbe\x92\xef\xbe\x93\xef\xbe\x98-\xef\xbe\x8a\xef\xbe\x9f\xef\xbd\xaf\xef\xbd\xb8*\x00"
        as *const u8 as *const libc::c_char as *mut libc::c_char;
#[no_mangle]
pub static mut D_8012CF80: *mut libc::c_char =
    b"\x8d\xef\xbd\xa6\xef\xbe\x90\xef\xbe\x82\xef\xbd\xb9\xef\xbe\x97\xef\xbe\x9a\xef\xbe\x8f\xef\xbd\xbe\xef\xbe\x9d\x00"
        as *const u8 as *const libc::c_char as *mut libc::c_char;
#[no_mangle]
pub static mut D_8012CF84: *mut libc::c_char =
    b"\x8c\xef\xbe\x8c\xef\xbd\xa7\xef\xbd\xb2\xef\xbe\x99 \x8d\xef\xbd\xa6\x00"
        as *const u8 as *const libc::c_char as *mut libc::c_char;
#[no_mangle]
pub static mut D_8012CF88: [*mut libc::c_char; 3] =
    [b"\x8d\xef\xbd\xbc\xef\xbe\x83\xef\xbe\x93\xef\xbd\xb2\xef\xbd\xb2\xef\xbe\x83\xef\xbe\x9e\xef\xbd\xbd\xef\xbd\xb6?\x00"
         as *const u8 as *const libc::c_char as *mut libc::c_char,
     b"\x8d\xef\xbd\xb9\xef\xbe\x9e\xef\xbe\x9d\xef\xbd\xbb\xef\xbe\x9e\xef\xbd\xb2\xef\xbe\x8d\xef\xbe\x9d\xef\xbd\xbc\xef\xbd\xad\xef\xbd\xb3\xef\xbe\x81\xef\xbd\xad\xef\xbd\xb3\xef\xbe\x89\x00"
         as *const u8 as *const libc::c_char as *mut libc::c_char,
     b"\x8c\xef\xbe\x8c\xef\xbd\xa7\xef\xbd\xb2\xef\xbe\x99\x8d\xef\xbe\x8a\xef\xbe\x8a\xef\xbd\xb7\xef\xbd\xbb\xef\xbe\x9a\xef\xbe\x8f\xef\xbd\xbd\x00"
         as *const u8 as *const libc::c_char as *mut libc::c_char];
#[no_mangle]
pub static mut D_8012CF94: *mut libc::c_char =
    b"\x8d\xef\xbe\x8a\xef\xbd\xb2\x00" as *const u8 as *const libc::c_char as
        *mut libc::c_char;
#[no_mangle]
pub static mut D_8012CF98: *mut libc::c_char =
    b"\x8d\xef\xbd\xb2\xef\xbd\xb2\xef\xbd\xb4\x00" as *const u8 as
        *const libc::c_char as *mut libc::c_char;
#[no_mangle]
pub static mut D_8012CF9C: [*mut libc::c_char; 2] =
    [b"\x8d\xef\xbd\xbc\xef\xbe\x83\xef\xbd\xb2\xef\xbe\x8f\xef\xbd\xbd\x00"
         as *const u8 as *const libc::c_char as *mut libc::c_char,
     b"\x8d\xef\xbd\xb3\xef\xbe\x9c\xef\xbd\xb6\xef\xbe\x9e\xef\xbd\xb7\x00"
         as *const u8 as *const libc::c_char as *mut libc::c_char];
#[no_mangle]
pub static mut D_8012CFA4: *mut libc::c_char =
    b"\x8d\xef\xbd\xbc\xef\xbe\x8f\xef\xbd\xbc\xef\xbe\x80\x00" as *const u8
        as *const libc::c_char as *mut libc::c_char;
#[no_mangle]
pub static mut D_8012CFA8: *mut libc::c_char =
    b"USE       BYTE\x00" as *const u8 as *const libc::c_char as
        *mut libc::c_char;
#[no_mangle]
pub static mut D_8012CFAC: *mut libc::c_char =
    b"\x8d\xef\xbe\x86\xef\xbd\xbc\xef\xbd\xaf\xef\xbe\x8a\xef\xbe\x9f\xef\xbd\xb2\x00"
        as *const u8 as *const libc::c_char as *mut libc::c_char;
#[no_mangle]
pub static mut D_8012CFB0: *mut libc::c_char =
    b"\x8cE\xef\xbe\x93-\xef\xbe\x84\xef\xbe\x9e\x8d  \xef\xbd\xba\xef\xbe\x83\xef\xbd\xb2\x00"
        as *const u8 as *const libc::c_char as *mut libc::c_char;
#[no_mangle]
pub static mut D_8012CFB4: [libc::c_char; 19] =
    unsafe {
        *::std::mem::transmute::<&[u8; 19],
                                 &mut [libc::c_char; 19]>(b"\x8c\xef\xbe\x8c\xef\xbe\x9a-\xef\xbe\x91       \x00")
    };
#[no_mangle]
pub static mut D_8012CFC4: [libc::c_char; 13] =
    unsafe {
        *::std::mem::transmute::<&[u8; 13],
                                 &mut [libc::c_char; 13]>(b"\x8c\xef\xbd\xb7-   /   \x00")
    };
#[no_mangle]
pub static mut D_8012CFD0: [libc::c_char; 25] =
    unsafe {
        *::std::mem::transmute::<&[u8; 25],
                                 &mut [libc::c_char; 25]>(b"\x8d(\xef\xbd\xbe\xef\xbe\x9d\xef\xbe\x80-\xef\xbe\x83\xef\xbe\x9d)\x00\x00\x00\x00\x00\x00")
    };
#[no_mangle]
pub static mut D_8012CFE4: [libc::c_char; 25] =
    unsafe {
        *::std::mem::transmute::<&[u8; 25],
                                 &mut [libc::c_char; 25]>(b"\x8d(\xef\xbd\xb9\xef\xbe\x9e\xef\xbe\x9d\xef\xbe\x83\xef\xbe\x9d)\x00\x00\x00\x00\x00\x00\x00")
    };
#[no_mangle]
pub static mut D_8012CFF8: [libc::c_char; 25] =
    unsafe {
        *::std::mem::transmute::<&[u8; 25],
                                 &mut [libc::c_char; 25]>(b"\x8c(\xef\xbe\x8c\xef\xbe\x9f\xef\xbe\x9a\xef\xbd\xb2\xef\xbe\x94-)\x00\x00\x00\x00\x00\x00")
    };
#[no_mangle]
pub static mut D_8012D00C: [libc::c_char; 25] =
    unsafe {
        *::std::mem::transmute::<&[u8; 25],
                                 &mut [libc::c_char; 25]>(b"\x8d(\xef\xbd\xb2\xef\xbe\x81\xef\xbd\xb1\xef\xbe\x9c\xef\xbd\xbe)\x00\x00\x00\x00\x00\x00\x00")
    };
#[no_mangle]
pub static mut D_8012D020: [libc::c_char; 24] =
    unsafe {
        *::std::mem::transmute::<&[u8; 24],
                                 &mut [libc::c_char; 24]>(b"\x8d(\xef\xbd\xbe\xef\xbd\xaf\xef\xbe\x83\xef\xbd\xb2)\x00\x00\x00\x00\x00\x00\x00\x00\x00")
    };
#[no_mangle]
pub static mut D_8012D034: [libc::c_char; 25] =
    unsafe {
        *::std::mem::transmute::<&[u8; 25],
                                 &mut [libc::c_char; 25]>(b"\x8d(\xef\xbd\xb7\xef\xbd\xac\xef\xbd\xaf\xef\xbd\xb6\xef\xbe\x9d)\x00\x00\x00\x00\x00\x00\x00")
    };
#[no_mangle]
pub static mut D_8012D048: [libc::c_char; 25] =
    unsafe {
        *::std::mem::transmute::<&[u8; 25],
                                 &mut [libc::c_char; 25]>(b"\x8c\xef\xbe\x8e\xef\xbe\x9f\xef\xbd\xb2\xef\xbe\x9d\xef\xbe\x84No.  \x00\x00\x00\x00")
    };
#[no_mangle]
pub static mut D_8012D05C: [libc::c_char; 21] =
    unsafe {
        *::std::mem::transmute::<&[u8; 21],
                                 &mut [libc::c_char; 21]>(b"\x8d\xef\xbd\xb6\xef\xbe\x9e\xef\xbd\xb6\xef\xbd\xb8    \x00\x00\x00\x00")
    };
#[no_mangle]
pub static mut D_8012D070: [libc::c_char; 20] =
    unsafe {
        *::std::mem::transmute::<&[u8; 20],
                                 &mut [libc::c_char; 20]>(b"\x8cN\xef\xbe\x8c\xef\xbe\x9a-\xef\xbe\x91   \x00\x00\x00\x00\x00")
    };
#[no_mangle]
pub static mut D_8012D084: [libc::c_char; 21] =
    unsafe {
        *::std::mem::transmute::<&[u8; 21],
                                 &mut [libc::c_char; 21]>(b"\x8dZ\xef\xbd\xb6\xef\xbd\xb2\xef\xbe\x83\xef\xbe\x9d   \x00\x00\x00\x00")
    };
#[no_mangle]
pub static mut D_8012D098: [libc::c_char; 20] =
    unsafe {
        *::std::mem::transmute::<&[u8; 20],
                                 &mut [libc::c_char; 20]>(b"\x8c\xef\xbe\x93-\xef\xbe\x84\xef\xbe\x9e    \x00\x00\x00\x00\x00")
    };
#[no_mangle]
pub static mut D_8012D0AC: [libc::c_char; 25] =
    unsafe {
        *::std::mem::transmute::<&[u8; 25],
                                 &mut [libc::c_char; 25]>(b"  R\x8d\xef\xbe\x81\xef\xbd\xad\xef\xbd\xb3\xef\xbd\xbc\xef\xbe\x9d  \x00\x00\x00\x00")
    };
#[no_mangle]
pub static mut D_8012D0C0: [libc::c_char; 24] =
    unsafe {
        *::std::mem::transmute::<&[u8; 24],
                                 &mut [libc::c_char; 24]>(b"\x8dP\xef\xbd\xbc\xef\xbe\x9e\xef\xbd\xb6\xef\xbe\x9d       \x00\x00\x00")
    };
#[no_mangle]
pub static mut D_8012D0D4: [libc::c_char; 19] =
    unsafe {
        *::std::mem::transmute::<&[u8; 19],
                                 &mut [libc::c_char; 19]>(b"\x8d\xef\xbd\xb7\xef\xbd\xae\xef\xbe\x98       \x00\x00")
    };
#[no_mangle]
pub static mut D_8012D0E4: [libc::c_char; 24] =
    unsafe {
        *::std::mem::transmute::<&[u8; 24],
                                 &mut [libc::c_char; 24]>(b"\x8dX\xef\xbd\xb6\xef\xbd\xb2\xef\xbe\x83\xef\xbe\x9d       \x00\x00\x00")
    };
#[no_mangle]
pub static mut D_8012D0F8: [libc::c_char; 24] =
    unsafe {
        *::std::mem::transmute::<&[u8; 24],
                                 &mut [libc::c_char; 24]>(b"\x8dY\xef\xbd\xb6\xef\xbd\xb2\xef\xbe\x83\xef\xbe\x9d       \x00\x00\x00")
    };
static mut sDbCamPtr: *mut DbCamera = 0 as *const DbCamera as *mut DbCamera;
static mut D_8016110C: s16 = 0;
static mut sDbCamAnim: DbCameraAnim =
    DbCameraAnim{curFrame: 0.,
                 unk_04: 0.,
                 keyframe: 0,
                 unk_0A: 0,
                 unk_0C: 0,
                 positionPos: Vec3f{x: 0., y: 0., z: 0.,},
                 lookAtPos: Vec3f{x: 0., y: 0., z: 0.,},
                 roll: 0.,
                 fov: 0.,};
#[no_mangle]
pub unsafe extern "C" fn DbCamera_AddVecSph(mut out: *mut Vec3f,
                                            mut in_0: *mut Vec3f,
                                            mut sph: *mut VecSph)
 -> *mut Vec3f {
    let mut ret: Vec3f = Vec3f{x: 0., y: 0., z: 0.,};
    let mut vec: Vec3f = Vec3f{x: 0., y: 0., z: 0.,};
    OLib_VecSphGeoToVec3f(&mut vec, sph);
    ret.x = (*in_0).x + vec.x;
    ret.y = (*in_0).y + vec.y;
    ret.z = (*in_0).z + vec.z;
    *out = ret;
    return out;
}
#[no_mangle]
pub unsafe extern "C" fn DbCamera_CalcUpFromPitchYawRoll(mut dest: *mut Vec3f,
                                                         mut pitch: s16,
                                                         mut yaw: s16,
                                                         mut roll: s16)
 -> *mut Vec3f {
    let mut sinPitch: f32_0 = 0.;
    let mut cosPitch: f32_0 = 0.;
    let mut sinYaw: f32_0 = 0.;
    let mut cosYaw: f32_0 = 0.;
    let mut sinNegRoll: f32_0 = 0.;
    let mut cosNegRoll: f32_0 = 0.;
    let mut spA4: Vec3f = Vec3f{x: 0., y: 0., z: 0.,};
    let mut sp54: f32_0 = 0.;
    let mut sp4C: f32_0 = 0.;
    let mut cosPitchCosYawSinRoll: f32_0 = 0.;
    let mut negSinPitch: f32_0 = 0.;
    let mut temp_f10_2: f32_0 = 0.;
    let mut cosPitchcosYaw: f32_0 = 0.;
    let mut temp_f14: f32_0 = 0.;
    let mut negSinPitchSinYaw: f32_0 = 0.;
    let mut negSinPitchCosYaw: f32_0 = 0.;
    let mut cosPitchSinYaw: f32_0 = 0.;
    let mut temp_f4_2: f32_0 = 0.;
    let mut temp_f6: f32_0 = 0.;
    let mut temp_f8: f32_0 = 0.;
    let mut temp_f8_2: f32_0 = 0.;
    let mut temp_f8_3: f32_0 = 0.;
    sinPitch = Math_SinS(pitch);
    cosPitch = Math_CosS(pitch);
    sinYaw = Math_SinS(yaw);
    cosYaw = Math_CosS(yaw);
    sinNegRoll = Math_SinS(-(roll as libc::c_int) as s16);
    cosNegRoll = Math_CosS(-(roll as libc::c_int) as s16);
    negSinPitch = -sinPitch;
    negSinPitchSinYaw = negSinPitch * sinYaw;
    negSinPitchCosYaw = negSinPitch * cosYaw;
    temp_f14 = 1.0f32 - cosNegRoll;
    cosPitchSinYaw = cosPitch * sinYaw;
    sp54 = cosPitchSinYaw * cosPitchSinYaw;
    sp4C = cosPitchSinYaw * sinPitch * temp_f14;
    cosPitchcosYaw = cosPitch * cosYaw;
    temp_f4_2 = (1.0f32 - sp54) * cosNegRoll + sp54;
    cosPitchCosYawSinRoll = cosPitchcosYaw * sinNegRoll;
    temp_f6 = cosPitchcosYaw * cosPitchSinYaw * temp_f14;
    temp_f10_2 = sinPitch * sinNegRoll;
    spA4.x =
        negSinPitchSinYaw * temp_f4_2 +
            cosPitch * (sp4C - cosPitchCosYawSinRoll) +
            negSinPitchCosYaw * (temp_f6 + temp_f10_2);
    sp54 = sinPitch * sinPitch;
    temp_f4_2 = sinPitch * cosPitchcosYaw * temp_f14;
    temp_f8_3 = cosPitchSinYaw * sinNegRoll;
    temp_f8 = sp4C + cosPitchCosYawSinRoll;
    spA4.y =
        negSinPitchSinYaw * temp_f8 +
            cosPitch * ((1.0f32 - sp54) * cosNegRoll + sp54) +
            negSinPitchCosYaw * (temp_f4_2 - temp_f8_3);
    temp_f8_2 = temp_f6 - temp_f10_2;
    spA4.z =
        negSinPitchSinYaw * temp_f8_2 + cosPitch * (temp_f4_2 + temp_f8_3) +
            negSinPitchCosYaw *
                ((1.0f32 - cosPitchcosYaw * cosPitchcosYaw) * cosNegRoll +
                     cosPitchcosYaw * cosPitchcosYaw);
    *dest = spA4;
    return dest;
}
#[no_mangle]
pub unsafe extern "C" fn DbCamera_SetTextValue(mut value: s16,
                                               mut str: *mut libc::c_char,
                                               mut endIdx: u8_0)
 -> *mut libc::c_char {
    let mut strIter: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut sign: libc::c_char = 0;
    strIter =
        str.offset(endIdx as s32 as
                       isize).offset(-(1 as libc::c_int as isize));
    *str.offset(endIdx as isize) = '\u{0}' as i32 as libc::c_char;
    if value as libc::c_int >= 0 as libc::c_int {
        sign = ' ' as i32 as libc::c_char
    } else {
        sign = '-' as i32 as libc::c_char;
        value = -(value as libc::c_int) as s16
    }
    loop  {
        // clang-format off
        let fresh0 = strIter;
        strIter = strIter.offset(-1);
        *fresh0 =
            (value as libc::c_int % 10 as libc::c_int + '0' as i32) as
                libc::c_char;
        value = (value as libc::c_int / 10 as libc::c_int) as s16;
        if !(value as libc::c_int != 0 as libc::c_int) { break ; }
        // clang-format on
    } // bool
    if sign as libc::c_int == '-' as i32 {
        let fresh1 = strIter; // bool
        strIter = strIter.offset(-1);
        *fresh1 = sign
    }
    while strIter >= str {
        let fresh2 = str;
        str = str.offset(1);
        *fresh2 = ' ' as i32 as libc::c_char
    }
    return strIter;
}
#[no_mangle]
pub unsafe extern "C" fn DbCamera_Vec3SToF(mut in_0: *mut Vec3s,
                                           mut out: *mut Vec3f) {
    (*out).x = (*in_0).x as f32_0;
    (*out).y = (*in_0).y as f32_0;
    (*out).z = (*in_0).z as f32_0;
}
#[no_mangle]
pub unsafe extern "C" fn DbCamera_Vec3FToS(mut in_0: *mut Vec3f,
                                           mut out: *mut Vec3s) {
    (*out).x = (*in_0).x as s16;
    (*out).y = (*in_0).y as s16;
    (*out).z = (*in_0).z as s16;
}
#[no_mangle]
pub unsafe extern "C" fn DbCamera_CopyVec3f(mut in_0: *mut Vec3f,
                                            mut out: *mut Vec3f) {
    (*out).x = (*in_0).x;
    (*out).y = (*in_0).y;
    (*out).z = (*in_0).z;
}
#[no_mangle]
pub unsafe extern "C" fn DbCamera_Vec3SToF2(mut in_0: *mut Vec3s,
                                            mut out: *mut Vec3f) {
    (*out).x = (*in_0).x as f32_0;
    (*out).y = (*in_0).y as f32_0;
    (*out).z = (*in_0).z as f32_0;
}
#[no_mangle]
pub unsafe extern "C" fn func_800B3F94(mut posRot: *mut PosRot,
                                       mut vec: *mut Vec3f,
                                       mut out: *mut Vec3s) {
    let mut sph: VecSph = VecSph{r: 0., pitch: 0, yaw: 0,};
    let mut tempVec: Vec3f = Vec3f{x: 0., y: 0., z: 0.,};
    OLib_Vec3fDiffToVecSphGeo(&mut sph, &mut (*posRot).pos, vec);
    sph.yaw =
        (sph.yaw as libc::c_int - (*posRot).rot.y as libc::c_int) as s16;
    OLib_VecSphGeoToVec3f(&mut tempVec, &mut sph);
    DbCamera_Vec3FToS(&mut tempVec, out);
}
#[no_mangle]
pub unsafe extern "C" fn func_800B3FF4(mut posRot: *mut PosRot,
                                       mut vec: *mut Vec3f,
                                       mut out: *mut Vec3f) {
    let mut sph: VecSph = VecSph{r: 0., pitch: 0, yaw: 0,};
    let mut tempVec: Vec3f = Vec3f{x: 0., y: 0., z: 0.,};
    DbCamera_CopyVec3f(vec, &mut tempVec);
    OLib_Vec3fToVecSphGeo(&mut sph, &mut tempVec);
    sph.yaw =
        (sph.yaw as libc::c_int + (*posRot).rot.y as libc::c_int) as s16;
    DbCamera_AddVecSph(out, &mut (*posRot).pos, &mut sph);
}
#[no_mangle]
pub unsafe extern "C" fn func_800B404C(mut posRot: *mut PosRot,
                                       mut vec: *mut Vec3s,
                                       mut out: *mut Vec3f) {
    let mut tempVec: Vec3f = Vec3f{x: 0., y: 0., z: 0.,};
    DbCamera_Vec3SToF(vec, &mut tempVec);
    func_800B3FF4(posRot, &mut tempVec, out);
}
#[no_mangle]
pub unsafe extern "C" fn func_800B4088(mut dbCamera: *mut DbCamera,
                                       mut cam: *mut Camera) -> s32 {
    let mut position: *mut CutsceneCameraPoint =
        0 as *mut CutsceneCameraPoint;
    let mut lookAt: *mut CutsceneCameraPoint = 0 as *mut CutsceneCameraPoint;
    let mut i: s32 = 0;
    position =
        &mut *(*dbCamera).sub.position.as_mut_ptr().offset((*dbCamera).sub.unkIdx
                                                               as isize) as
            *mut CutsceneCameraPoint;
    lookAt =
        &mut *(*dbCamera).sub.lookAt.as_mut_ptr().offset((*dbCamera).sub.unkIdx
                                                             as isize) as
            *mut CutsceneCameraPoint;
    (*position).continueFlag = -(1 as libc::c_int) as s8;
    (*lookAt).continueFlag = (*position).continueFlag;
    (*position).nextPointFrame = 0 as libc::c_int as u16_0;
    (*lookAt).nextPointFrame = 30 as libc::c_int as u16_0;
    (*position).cameraRoll =
        ((*dbCamera).roll as libc::c_int as libc::c_float *
             (360.0f32 / 256.0f32)) as s8;
    (*lookAt).cameraRoll = (*position).cameraRoll;
    (*position).viewAngle = (*dbCamera).fov;
    (*lookAt).viewAngle = (*position).viewAngle;
    if (*dbCamera).sub.mode as libc::c_int != 1 as libc::c_int {
        DbCamera_Vec3FToS(&mut (*dbCamera).eye, &mut (*position).pos);
        DbCamera_Vec3FToS(&mut (*dbCamera).at, &mut (*lookAt).pos);
    } else {
        func_800B3F94(&mut (*cam).playerPosRot, &mut (*dbCamera).at,
                      &mut (*lookAt).pos);
        func_800B3F94(&mut (*cam).playerPosRot, &mut (*dbCamera).eye,
                      &mut (*position).pos);
    }
    i = 0 as libc::c_int;
    while i < (*dbCamera).sub.nPoints as libc::c_int - 2 as libc::c_int {
        (*dbCamera).sub.lookAt[i as usize].continueFlag =
            0 as libc::c_int as s8;
        (*dbCamera).sub.position[i as usize].continueFlag =
            (*dbCamera).sub.lookAt[i as usize].continueFlag;
        i += 1
    }
    (*dbCamera).sub.lookAt[i as usize].continueFlag =
        -(1 as libc::c_int) as s8;
    (*dbCamera).sub.position[i as usize].continueFlag =
        (*dbCamera).sub.lookAt[i as usize].continueFlag;
    return (*dbCamera).sub.unkIdx as s32;
}
#[no_mangle]
pub unsafe extern "C" fn func_800B41DC(mut dbCamera: *mut DbCamera,
                                       mut idx: s16,
                                       mut cameraPtr: *mut Camera) -> s16 {
    let mut position: *mut CutsceneCameraPoint =
        &mut *(*dbCamera).sub.position.as_mut_ptr().offset(idx as isize) as
            *mut CutsceneCameraPoint;
    let mut lookAt: *mut CutsceneCameraPoint =
        &mut *(*dbCamera).sub.lookAt.as_mut_ptr().offset(idx as isize) as
            *mut CutsceneCameraPoint;
    if (*dbCamera).sub.mode as libc::c_int != 1 as libc::c_int {
        DbCamera_Vec3SToF2(&mut (*position).pos, &mut (*dbCamera).eye);
        DbCamera_Vec3SToF2(&mut (*lookAt).pos, &mut (*dbCamera).at);
    } else {
        func_800B404C(&mut (*cameraPtr).playerPosRot, &mut (*lookAt).pos,
                      &mut (*dbCamera).at);
        func_800B404C(&mut (*cameraPtr).playerPosRot, &mut (*position).pos,
                      &mut (*dbCamera).eye);
    }
    (*dbCamera).roll = (*lookAt).cameraRoll as s16;
    (*dbCamera).rollDegrees =
        (*dbCamera).roll as libc::c_int as libc::c_float *
            (360.0f32 / 256.0f32);
    (*dbCamera).fov = (*lookAt).viewAngle;
    return idx;
}
#[no_mangle]
pub unsafe extern "C" fn func_800B42C0(mut dbCamera: *mut DbCamera,
                                       mut cameraPtr: *mut Camera) -> s32 {
    let mut position: *mut CutsceneCameraPoint =
        &mut *(*dbCamera).sub.position.as_mut_ptr().offset((*dbCamera).sub.unkIdx
                                                               as isize) as
            *mut CutsceneCameraPoint;
    let mut lookAt: *mut CutsceneCameraPoint =
        &mut *(*dbCamera).sub.lookAt.as_mut_ptr().offset((*dbCamera).sub.unkIdx
                                                             as isize) as
            *mut CutsceneCameraPoint;
    (*lookAt).continueFlag = 0 as libc::c_int as s8;
    (*position).continueFlag = (*lookAt).continueFlag;
    if (*dbCamera).sub.mode as libc::c_int != 1 as libc::c_int {
        DbCamera_Vec3FToS(&mut (*dbCamera).eye, &mut (*position).pos);
        DbCamera_Vec3FToS(&mut (*dbCamera).at, &mut (*lookAt).pos);
    } else {
        func_800B3F94(&mut (*cameraPtr).playerPosRot, &mut (*dbCamera).at,
                      &mut (*lookAt).pos);
        func_800B3F94(&mut (*cameraPtr).playerPosRot, &mut (*dbCamera).eye,
                      &mut (*position).pos);
    }
    return (*dbCamera).sub.unkIdx as s32;
}
#[no_mangle]
pub unsafe extern "C" fn func_800B4370(mut dbCamera: *mut DbCamera,
                                       mut idx: s16, mut cam: *mut Camera)
 -> s32 {
    let mut lookAt: *mut CutsceneCameraPoint =
        &mut *(*dbCamera).sub.lookAt.as_mut_ptr().offset(idx as isize) as
            *mut CutsceneCameraPoint;
    let mut position: *mut CutsceneCameraPoint =
        &mut *(*dbCamera).sub.position.as_mut_ptr().offset(idx as isize) as
            *mut CutsceneCameraPoint;
    let mut sph: VecSph = VecSph{r: 0., pitch: 0, yaw: 0,};
    let mut at: Vec3f = Vec3f{x: 0., y: 0., z: 0.,};
    if (*dbCamera).sub.mode as libc::c_int != 1 as libc::c_int {
        if (*dbCamera).sub.unk_0C != 0 {
            DbCamera_Vec3SToF2(&mut (*position).pos, &mut (*dbCamera).at);
        } else {
            DbCamera_Vec3SToF2(&mut (*lookAt).pos, &mut (*dbCamera).at);
        }
    } else {
        if (*dbCamera).sub.unk_0C != 0 {
            func_800B404C(&mut (*cam).playerPosRot, &mut (*position).pos,
                          &mut at);
        } else {
            func_800B404C(&mut (*cam).playerPosRot, &mut (*lookAt).pos,
                          &mut at);
        }
        (*dbCamera).at = at
    }
    sph.pitch = 0x2000 as libc::c_int as s16;
    sph.yaw = (sph.yaw as libc::c_int - 0x7fff as libc::c_int) as s16;
    sph.r = 250.0f32;
    DbCamera_AddVecSph(&mut (*dbCamera).eye, &mut (*dbCamera).at, &mut sph);
    (*dbCamera).roll = (*lookAt).cameraRoll as s16;
    (*dbCamera).rollDegrees =
        (*dbCamera).roll as libc::c_int as libc::c_float *
            (360.0f32 / 256.0f32);
    (*dbCamera).fov = (*lookAt).viewAngle;
    return idx as s32;
}
#[no_mangle]
pub unsafe extern "C" fn func_800B44E0(mut dbCamera: *mut DbCamera,
                                       mut cam: *mut Camera) {
    let mut i: s32 = 0;
    if !((*sGlobalCtx).state.input[2 as libc::c_int as usize].press.button as
             libc::c_int | !(0x1 as libc::c_int)) == 0 as libc::c_int {
        sDbCamAnim.keyframe = 0 as libc::c_int as s16;
        sDbCamAnim.unk_0A = 1 as libc::c_int as s16;
        sDbCamAnim.curFrame = 0.0f32;
        sDbCamAnim.unk_04 = 0 as libc::c_int as f32_0;
        i = 0 as libc::c_int;
        while i < (*dbCamera).sub.nPoints as libc::c_int - 2 as libc::c_int {
            (*dbCamera).sub.lookAt[i as usize].continueFlag =
                0 as libc::c_int as s8;
            (*dbCamera).sub.position[i as usize].continueFlag =
                (*dbCamera).sub.lookAt[i as usize].continueFlag;
            i += 1
        }
        (*dbCamera).sub.lookAt[i as usize].continueFlag =
            -(1 as libc::c_int) as s8;
        (*dbCamera).sub.position[i as usize].continueFlag =
            (*dbCamera).sub.lookAt[i as usize].continueFlag
    }
    if ((*dbCamera).sub.nPoints as libc::c_int) < 6 as libc::c_int {
        if sDbCamAnim.unk_0A as libc::c_int != 0 as libc::c_int {
            Audio_PlaySoundGeneral(0x4806 as libc::c_int as u16_0,
                                   &mut D_801333D4, 4 as libc::c_int as u8_0,
                                   &mut D_801333E0, &mut D_801333E0,
                                   &mut D_801333E8);
            sDbCamAnim.unk_0A = 0 as libc::c_int as s16
        }
        func_8006376C(0x11 as libc::c_int as u8_0,
                      0x17 as libc::c_int as u8_0, 3 as libc::c_int as u8_0,
                      D_8012CEE0[0 as libc::c_int as usize]);
        func_8006376C(0x12 as libc::c_int as u8_0,
                      0x18 as libc::c_int as u8_0, 3 as libc::c_int as u8_0,
                      D_8012CEE4);
        func_8006376C(0x10 as libc::c_int as u8_0,
                      0x1a as libc::c_int as u8_0, 1 as libc::c_int as u8_0,
                      D_8012CEE8);
        return
    }
    if func_800BB2B4(&mut sDbCamAnim.positionPos, &mut sDbCamAnim.roll,
                     &mut sDbCamAnim.fov,
                     (*dbCamera).sub.position.as_mut_ptr(),
                     &mut sDbCamAnim.keyframe, &mut sDbCamAnim.curFrame) == 0
           &&
           func_800BB2B4(&mut sDbCamAnim.lookAtPos, &mut sDbCamAnim.roll,
                         &mut sDbCamAnim.fov,
                         (*dbCamera).sub.lookAt.as_mut_ptr(),
                         &mut sDbCamAnim.keyframe, &mut sDbCamAnim.curFrame)
               == 0 && sDbCamAnim.unk_0A as libc::c_int == 1 as libc::c_int {
        Audio_PlaySoundGeneral(0x480b as libc::c_int as u16_0,
                               &mut D_801333D4, 4 as libc::c_int as u8_0,
                               &mut D_801333E0, &mut D_801333E0,
                               &mut D_801333E8);
        sDbCamAnim.unk_04 += 1.;
        if (*dbCamera).sub.nFrames as libc::c_int > 0 as libc::c_int &&
               ((*dbCamera).sub.nFrames as libc::c_int as libc::c_float) <
                   sDbCamAnim.unk_04 {
            sDbCamAnim.unk_0A = 0 as libc::c_int as s16;
            func_8006376C(0xf as libc::c_int as u8_0,
                          0x1a as libc::c_int as u8_0,
                          1 as libc::c_int as u8_0, D_8012CEEC);
        }
        if (*dbCamera).sub.mode as libc::c_int != 1 as libc::c_int {
            DbCamera_CopyVec3f(&mut sDbCamAnim.positionPos,
                               &mut (*dbCamera).eye);
            DbCamera_CopyVec3f(&mut sDbCamAnim.lookAtPos,
                               &mut (*dbCamera).at);
        } else {
            func_800B3FF4(&mut (*cam).playerPosRot, &mut sDbCamAnim.lookAtPos,
                          &mut (*dbCamera).at);
            func_800B3FF4(&mut (*cam).playerPosRot,
                          &mut sDbCamAnim.positionPos, &mut (*dbCamera).eye);
        }
        (*dbCamera).fov = sDbCamAnim.fov;
        (*dbCamera).roll = sDbCamAnim.roll as s16;
        (*dbCamera).rollDegrees = sDbCamAnim.roll * (360.0f32 / 256.0f32);
        DbCamera_SetTextValue(sDbCamAnim.unk_04 as s16,
                              &mut *D_8012CFB4.as_mut_ptr().offset(8 as
                                                                       libc::c_int
                                                                       as
                                                                       isize),
                              4 as libc::c_int as u8_0);
        func_8006376C(0x10 as libc::c_int as u8_0,
                      0x17 as libc::c_int as u8_0, 3 as libc::c_int as u8_0,
                      D_8012CFB4.as_mut_ptr());
        D_8012CFC4[5 as libc::c_int as usize] =
            ((sDbCamAnim.keyframe as libc::c_int + 1 as libc::c_int) /
                 10 as libc::c_int + '0' as i32) as libc::c_char;
        D_8012CFC4[6 as libc::c_int as usize] =
            ((sDbCamAnim.keyframe as libc::c_int + 1 as libc::c_int) %
                 10 as libc::c_int + '0' as i32) as libc::c_char;
        D_8012CFC4[8 as libc::c_int as usize] =
            (((*dbCamera).sub.nPoints as libc::c_int - 5 as libc::c_int) /
                 10 as libc::c_int + '0' as i32) as libc::c_char;
        D_8012CFC4[9 as libc::c_int as usize] =
            (((*dbCamera).sub.nPoints as libc::c_int - 5 as libc::c_int) %
                 10 as libc::c_int + '0' as i32) as libc::c_char;
        func_8006376C(0x10 as libc::c_int as u8_0,
                      0x18 as libc::c_int as u8_0, 3 as libc::c_int as u8_0,
                      D_8012CFC4.as_mut_ptr());
        func_8006376C(0x10 as libc::c_int as u8_0,
                      0x1a as libc::c_int as u8_0, 1 as libc::c_int as u8_0,
                      D_8012CEF0);
        return
    }
    sDbCamAnim.unk_0A = 0 as libc::c_int as s16;
    func_8006376C(0xf as libc::c_int as u8_0, 0x1a as libc::c_int as u8_0,
                  1 as libc::c_int as u8_0, D_8012CEEC);
}
#[no_mangle]
pub unsafe extern "C" fn DbCamera_PrintPoints(mut name: *const libc::c_char,
                                              mut count: s16,
                                              mut points:
                                                  *mut CutsceneCameraPoint) {
    let mut i: s32 = 0;
    osSyncPrintf(b"@@@static SplinedatZ  %s[] = {\n\x00" as *const u8 as
                     *const libc::c_char, name);
    i = 0 as libc::c_int;
    while i < count as libc::c_int {
        osSyncPrintf(b"@@@    /* key frame %2d */ {\n\x00" as *const u8 as
                         *const libc::c_char, i);
        osSyncPrintf(b"@@@    /*     code     */ %d,\n\x00" as *const u8 as
                         *const libc::c_char,
                     (*points.offset(i as isize)).continueFlag as
                         libc::c_int);
        osSyncPrintf(b"@@@    /*     z        */ %d,\n\x00" as *const u8 as
                         *const libc::c_char,
                     (*points.offset(i as isize)).cameraRoll as libc::c_int);
        osSyncPrintf(b"@@@    /*     T        */ %d,\n\x00" as *const u8 as
                         *const libc::c_char,
                     (*points.offset(i as isize)).nextPointFrame as
                         libc::c_int);
        osSyncPrintf(b"@@@    /*     zoom     */ %f,\n\x00" as *const u8 as
                         *const libc::c_char,
                     (*points.offset(i as isize)).viewAngle as
                         libc::c_double);
        osSyncPrintf(b"@@@    /*     pos      */ { %d, %d, %d }\n\x00" as
                         *const u8 as *const libc::c_char,
                     (*points.offset(i as isize)).pos.x as libc::c_int,
                     (*points.offset(i as isize)).pos.y as libc::c_int,
                     (*points.offset(i as isize)).pos.z as libc::c_int);
        osSyncPrintf(b"@@@    },\n\x00" as *const u8 as *const libc::c_char);
        i += 1
    }
    osSyncPrintf(b"@@@};\n@@@\n\x00" as *const u8 as *const libc::c_char);
}
#[no_mangle]
pub unsafe extern "C" fn DbCamera_PrintF32Bytes(mut value: f32_0) {
    let mut b: f32_0 = value;
    let mut a: *mut libc::c_char = &mut b as *mut f32_0 as *mut libc::c_char;
    osSyncPrintf(b"\n@@@%d,%d,%d,%d,\x00" as *const u8 as *const libc::c_char,
                 *a.offset(0 as libc::c_int as isize) as libc::c_int,
                 *a.offset(1 as libc::c_int as isize) as libc::c_int,
                 *a.offset(2 as libc::c_int as isize) as libc::c_int,
                 *a.offset(3 as libc::c_int as isize) as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn DbCamera_PrintU16Bytes(mut value: u16_0) {
    let mut pad: u16_0 = 0;
    let mut b: u16_0 = value;
    let mut a: *mut libc::c_char = &mut b as *mut u16_0 as *mut libc::c_char;
    osSyncPrintf(b"\n@@@%d,%d,\x00" as *const u8 as *const libc::c_char,
                 *a.offset(0 as libc::c_int as isize) as libc::c_int,
                 *a.offset(1 as libc::c_int as isize) as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn DbCamera_PrintS16Bytes(mut value: s16) {
    let mut pad: u16_0 = 0;
    let mut b: s16 = value;
    let mut a: *mut libc::c_char = &mut b as *mut s16 as *mut libc::c_char;
    osSyncPrintf(b"\n@@@%d,%d,\x00" as *const u8 as *const libc::c_char,
                 *a.offset(0 as libc::c_int as isize) as libc::c_int,
                 *a.offset(1 as libc::c_int as isize) as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn DbCamera_PrintCutBytes(mut cut: *mut DbCameraCut) {
    let mut point: *mut CutsceneCameraPoint = 0 as *mut CutsceneCameraPoint;
    let mut points: *mut CutsceneCameraPoint = 0 as *mut CutsceneCameraPoint;
    let mut i: s32 = 0;
    points = (*cut).lookAt;
    osSyncPrintf(b"\n@@@ 0,0,0,2,\t/* Look Camera\t*/\x00" as *const u8 as
                     *const libc::c_char);
    osSyncPrintf(b"\n@@@ 0,1,\t/* dousa\t*/\x00" as *const u8 as
                     *const libc::c_char);
    osSyncPrintf(b"\n@@@ 0,0,\t/* Start Flame\t*/\x00" as *const u8 as
                     *const libc::c_char);
    DbCamera_PrintU16Bytes((*cut).nFrames as u16_0);
    osSyncPrintf(b"\t/*  End   Flame\t*/\x00" as *const u8 as
                     *const libc::c_char);
    osSyncPrintf(b"\n@@@0,0,\t/*  Dammy\t*/\n@@@ \x00" as *const u8 as
                     *const libc::c_char);
    i = 0 as libc::c_int;
    while i < (*cut).nPoints as libc::c_int {
        point = points.offset(i as isize);
        osSyncPrintf(b"\n@@@    %d, /*     code     */\x00" as *const u8 as
                         *const libc::c_char,
                     (*point).continueFlag as libc::c_int);
        osSyncPrintf(b"\n@@@    %d,  /*     z        */\x00" as *const u8 as
                         *const libc::c_char,
                     (*point).cameraRoll as libc::c_int);
        DbCamera_PrintU16Bytes((*point).nextPointFrame);
        osSyncPrintf(b"\t/*  sokudo\t*/\x00" as *const u8 as
                         *const libc::c_char);
        DbCamera_PrintF32Bytes((*point).viewAngle);
        osSyncPrintf(b"\t/*  zoom\t*/\x00" as *const u8 as
                         *const libc::c_char);
        DbCamera_PrintS16Bytes((*point).pos.x);
        osSyncPrintf(b"\t/*  x pos\t*/\x00" as *const u8 as
                         *const libc::c_char);
        DbCamera_PrintS16Bytes((*point).pos.y);
        osSyncPrintf(b"\t/*  y pos\t*/\x00" as *const u8 as
                         *const libc::c_char);
        DbCamera_PrintS16Bytes((*point).pos.z);
        osSyncPrintf(b"\t/*  z pos\t*/\n\x00" as *const u8 as
                         *const libc::c_char);
        osSyncPrintf(b"\n@@@0,0,\t/*  Dammy\t*/\n@@@ \x00" as *const u8 as
                         *const libc::c_char);
        i += 1
    }
    points = (*cut).position;
    osSyncPrintf(b"\n@@@ 0,0,0,1,\t/* Position Camera */\x00" as *const u8 as
                     *const libc::c_char);
    osSyncPrintf(b"\n@@@ 0,1,\t/* dousa\t*/\x00" as *const u8 as
                     *const libc::c_char);
    osSyncPrintf(b"\n@@@ 0,0,\t/* Start Flame\t*/\x00" as *const u8 as
                     *const libc::c_char);
    DbCamera_PrintU16Bytes((*cut).nFrames as u16_0);
    osSyncPrintf(b"\t/*  End   Flame\t*/\x00" as *const u8 as
                     *const libc::c_char);
    osSyncPrintf(b"\n@@@0,0,\t/*  Dammy\t*/\n@@@ \x00" as *const u8 as
                     *const libc::c_char);
    i = 0 as libc::c_int;
    while i < (*cut).nPoints as libc::c_int {
        point = points.offset(i as isize);
        osSyncPrintf(b"\n@@@    %d, /*     code     */\x00" as *const u8 as
                         *const libc::c_char,
                     (*point).continueFlag as libc::c_int);
        osSyncPrintf(b"\n@@@    %d, /*     z        */\x00" as *const u8 as
                         *const libc::c_char,
                     (*point).cameraRoll as libc::c_int);
        DbCamera_PrintU16Bytes((*point).nextPointFrame);
        osSyncPrintf(b"\t/*  sokudo\t*/\x00" as *const u8 as
                         *const libc::c_char);
        DbCamera_PrintF32Bytes((*point).viewAngle);
        osSyncPrintf(b"\t/*  zoom\t*/\x00" as *const u8 as
                         *const libc::c_char);
        DbCamera_PrintS16Bytes((*point).pos.x);
        osSyncPrintf(b"\t/*  x pos\t*/\x00" as *const u8 as
                         *const libc::c_char);
        DbCamera_PrintS16Bytes((*point).pos.y);
        osSyncPrintf(b"\t/*  y pos\t*/\x00" as *const u8 as
                         *const libc::c_char);
        DbCamera_PrintS16Bytes((*point).pos.z);
        osSyncPrintf(b"\t/*  z pos\t*/\x00" as *const u8 as
                         *const libc::c_char);
        osSyncPrintf(b"\n@@@0,0,\t/*  Dammy\t*/\n@@@ \x00" as *const u8 as
                         *const libc::c_char);
        i += 1
    };
}
#[no_mangle]
pub unsafe extern "C" fn DbCamera_Init(mut dbCamera: *mut DbCamera,
                                       mut cameraPtr: *mut Camera) {
    (*dbCamera).sub.unk_104A.z = 0 as libc::c_int as s16;
    (*dbCamera).unk_44 = 0 as libc::c_int;
    (*dbCamera).unk_00 = 0 as libc::c_int;
    (*dbCamera).unk_34 = 0 as libc::c_int;
    (*dbCamera).unk_3C = 0 as libc::c_int;
    (*dbCamera).unk_38 = -(1 as libc::c_int);
    (*dbCamera).unk_40 = -(1 as libc::c_int);
    (*dbCamera).roll = 0 as libc::c_int as s16;
    (*dbCamera).sub.unk_104A.y = (*dbCamera).sub.unk_104A.z;
    (*dbCamera).sub.unk_104A.x = (*dbCamera).sub.unk_104A.z;
    (*dbCamera).fov = 0.0f32;
    (*dbCamera).rollDegrees = 0.0f32;
    sGlobalCtx = (*cameraPtr).globalCtx;
    (*dbCamera).sub.mode = 0 as libc::c_int as s16;
    (*dbCamera).sub.nFrames = -(1 as libc::c_int) as s16;
    (*dbCamera).sub.nPoints = 1 as libc::c_int as s16;
    (*dbCamera).sub.unkIdx = 0 as libc::c_int as s16;
    (*dbCamera).sub.unk_08 = 0 as libc::c_int as s16;
    (*dbCamera).sub.unk_0A = 0 as libc::c_int as s16;
    (*dbCamera).unk_78 = 0 as libc::c_int as s16;
    (*dbCamera).unk_7A = 0 as libc::c_int as s16;
    (*dbCamera).sub.demoCtrlMenu =
        (0 as libc::c_int * 100 as libc::c_int + 0 as libc::c_int) as s16;
    (*dbCamera).sub.demoCtrlActionIdx = 0 as libc::c_int as s16;
    (*dbCamera).sub.demoCtrlToggleSwitch = 0 as libc::c_int as s16;
    (*dbCamera).unk_6C.x = 0 as libc::c_int as f32_0;
    (*dbCamera).unk_6C.y = 0 as libc::c_int as f32_0;
    (*dbCamera).unk_6C.z = 0 as libc::c_int as f32_0;
}
#[no_mangle]
pub unsafe extern "C" fn DbgCamera_Enable(mut dbCamera: *mut DbCamera,
                                          mut cam: *mut Camera) {
    (*dbCamera).at = (*cam).at;
    (*dbCamera).eye = (*cam).eye;
    (*dbCamera).unk_1C = (*cam).up;
    (*dbCamera).fov = (*cam).fov;
    (*dbCamera).roll = 0 as libc::c_int as s16;
    (*dbCamera).sub.nPoints = 1 as libc::c_int as s16;
    (*dbCamera).sub.unkIdx = 0 as libc::c_int as s16;
    (*dbCamera).sub.unk_08 = 0 as libc::c_int as s16;
    (*dbCamera).sub.unk_0A = 1 as libc::c_int as s16;
    (*dbCamera).sub.unk_0C = 1 as libc::c_int;
    (*dbCamera).unk_78 = 0 as libc::c_int as s16;
    (*dbCamera).unk_7A = 0 as libc::c_int as s16;
    (*dbCamera).rollDegrees = 0.0f32;
    func_800B4088(dbCamera, cam);
}
#[no_mangle]
pub unsafe extern "C" fn DbCamera_Update(mut dbCamera: *mut DbCamera,
                                         mut cam: *mut Camera) {
    static mut D_8012D10C: s32 = 100 as libc::c_int;
    static mut D_8012D110: s32 = 0 as libc::c_int;
    static mut D_80161140: s32 = 0;
    static mut D_80161144: s32 = 0;
    let mut sp124: *mut Vec3f = 0 as *mut Vec3f;
    let mut temp_f0_5: f32_0 = 0.;
    let mut yaw: s16 = 0;
    let mut new_var2: f32_0 = 0.;
    let mut temp_f2: f32_0 = 0.;
    let mut pitch: s16 = 0;
    let mut sp111: libc::c_char = 0;
    let mut sp110: libc::c_char = 0;
    let mut temp_f2_2: f32_0 = 0.;
    let mut sp104: VecSph = VecSph{r: 0., pitch: 0, yaw: 0,};
    let mut spFC: VecSph = VecSph{r: 0., pitch: 0, yaw: 0,};
    let mut spF4: VecSph = VecSph{r: 0., pitch: 0, yaw: 0,};
    let mut temp_s6: *mut PosRot = 0 as *mut PosRot;
    let mut eye: *mut Vec3f = 0 as *mut Vec3f;
    let mut at: *mut Vec3f = 0 as *mut Vec3f;
    let mut phi_s0: *mut Vec3f = 0 as *mut Vec3f;
    let mut spD8: Vec3f = Vec3f{x: 0., y: 0., z: 0.,};
    let mut pad: s32 = 0;
    let mut sp90: *mut Vec3f = 0 as *mut Vec3f;
    let mut sp80: *mut Vec3f = 0 as *mut Vec3f;
    let mut sp7C: *mut Vec3f = 0 as *mut Vec3f;
    let mut i: s32 = 0;
    let mut spB8: Vec3f = Vec3f{x: 0., y: 0., z: 0.,};
    let mut spAC: Vec3f = Vec3f{x: 0., y: 0., z: 0.,};
    let mut spAA: s16 = 0;
    let mut spA0: VecSph = VecSph{r: 0., pitch: 0, yaw: 0,};
    sp90 = &mut (*dbCamera).unk_54;
    temp_s6 = &mut (*cam).playerPosRot;
    at = &mut (*cam).at;
    eye = &mut (*cam).eye;
    *sp90 = (*temp_s6).pos;
    (*dbCamera).unk_60 = (*cam).at;
    sp80 = &mut (*dbCamera).eye;
    sp7C = &mut (*dbCamera).at;
    if !((*sGlobalCtx).state.input[2 as libc::c_int as usize].press.button as
             libc::c_int | !(0x2000 as libc::c_int)) == 0 as libc::c_int {
        (*dbCamera).unk_00 += 1;
        (*dbCamera).unk_00 %= 3 as libc::c_int;
        (*dbCamera).unk_38 = 1 as libc::c_int;
        (*dbCamera).unk_44 = 0 as libc::c_int;
        (*dbCamera).unk_40 = -(1 as libc::c_int);
        (*dbCamera).sub.demoCtrlActionIdx = 0 as libc::c_int as s16;
        sDbCamAnim.unk_0A = 0 as libc::c_int as s16;
        Audio_PlaySoundGeneral(0x4830 as libc::c_int as u16_0,
                               &mut D_801333D4, 4 as libc::c_int as u8_0,
                               &mut D_801333E0, &mut D_801333E0,
                               &mut D_801333E8);
    } else if (*dbCamera).unk_38 == -(1 as libc::c_int) {
        (*dbCamera).unk_38 = 1 as libc::c_int
    } else { (*dbCamera).unk_38 = 0 as libc::c_int }
    match (*dbCamera).unk_00 {
        0 => {
            match (*dbCamera).unk_78 as libc::c_int {
                0 => {
                    sp124 = &mut (*dbCamera).unk_60;
                    D_80161144 = 0 as libc::c_int;
                    D_80161140 = 0 as libc::c_int
                }
                1 => {
                    sp124 = &mut (*dbCamera).unk_6C;
                    D_80161144 = 0 as libc::c_int;
                    D_80161140 = 0 as libc::c_int
                }
                2 => {
                    sp124 = &mut (*dbCamera).unk_54;
                    D_80161144 = 0 as libc::c_int;
                    D_80161140 = 1 as libc::c_int
                }
                _ => { }
            }
        }
        1 => {
            match (*dbCamera).sub.unk_08 as libc::c_int {
                0 => {
                    D_80161144 = (*dbCamera).sub.unk_0C;
                    if D_80161144 != 0 { sp124 = sp80 } else { sp124 = sp7C }
                    D_80161140 = 0 as libc::c_int
                }
                1 => {
                    D_80161144 = (*dbCamera).sub.unk_0C;
                    if D_80161144 != 0 { sp124 = sp80 } else { sp124 = sp7C }
                    D_80161140 = 0 as libc::c_int
                }
                2 => {
                    D_80161144 = 0 as libc::c_int;
                    D_80161140 = 1 as libc::c_int;
                    sp124 = sp7C
                }
                _ => { }
            }
        }
        2 => { DbCamera_UpdateDemoControl(dbCamera, cam); return }
        _ => { }
    }
    phi_s0 = sp124;
    if D_80161144 == 0 {
        OLib_Vec3fDiffToVecSphGeo(&mut sp104, sp7C, sp80);
    } else { OLib_Vec3fDiffToVecSphGeo(&mut sp104, sp80, sp7C); }
    if (*dbCamera).unk_44 > 100 as libc::c_int {
        (*dbCamera).unk_44 = 100 as libc::c_int
    }
    new_var2 = (*dbCamera).unk_44 as libc::c_float * 0.15f32 + 0.2f32;
    temp_f2 = new_var2 * (sp104.r / 100.0f32);
    if (*dbCamera).unk_38 != 0 as libc::c_int || (*dbCamera).unk_3C != 0 {
        if D_80161144 != 0 { *sp80 = *phi_s0 } else { *sp7C = *phi_s0 }
        (*dbCamera).unk_3C = D_80161140;
        if !((*sGlobalCtx).state.input[2 as libc::c_int as usize].cur.button
                 as libc::c_int |
                 !(0x4000 as libc::c_int | 0x20 as libc::c_int)) ==
               0 as libc::c_int {
            sp104.r += temp_f2;
            if sp104.r > 30000.0f32 { sp104.r = 30000.0f32 }
            if (*dbCamera).unk_40 == 7 as libc::c_int {
                (*dbCamera).unk_44 += 1
            } else { (*dbCamera).unk_44 = 0 as libc::c_int }
            (*dbCamera).unk_40 = 7 as libc::c_int
        } else if !((*sGlobalCtx).state.input[2 as libc::c_int as
                                                  usize].cur.button as
                        libc::c_int | !(0x4000 as libc::c_int)) ==
                      0 as libc::c_int {
            spFC = sp104;
            spFC.r = temp_f2;
            if D_80161144 == 0 {
                spFC.yaw = sp104.yaw;
                DbCamera_AddVecSph(sp7C, sp7C, &mut spFC);
            } else {
                spFC.pitch = -(spFC.pitch as libc::c_int) as s16;
                spFC.yaw =
                    (sp104.yaw as libc::c_int - 0x7fff as libc::c_int) as s16;
                DbCamera_AddVecSph(sp80, sp80, &mut spFC);
            }
            if (*dbCamera).unk_40 == 0xb as libc::c_int {
                (*dbCamera).unk_44 += 1
            } else { (*dbCamera).unk_44 = 0 as libc::c_int }
            (*dbCamera).unk_40 = 0xb as libc::c_int
        } else if !((*sGlobalCtx).state.input[2 as libc::c_int as
                                                  usize].cur.button as
                        libc::c_int |
                        !(0x8000 as libc::c_int | 0x20 as libc::c_int)) ==
                      0 as libc::c_int {
            sp104.r -= temp_f2;
            if sp104.r < 10.0f32 { sp104.r = 10.0f32 }
            if (*dbCamera).unk_40 == 8 as libc::c_int {
                (*dbCamera).unk_44 += 1
            } else { (*dbCamera).unk_44 = 0 as libc::c_int }
            (*dbCamera).unk_40 = 8 as libc::c_int
        } else if !((*sGlobalCtx).state.input[2 as libc::c_int as
                                                  usize].cur.button as
                        libc::c_int | !(0x8000 as libc::c_int)) ==
                      0 as libc::c_int {
            spFC = sp104;
            spFC.r = -temp_f2;
            if D_80161144 == 0 {
                spFC.yaw = sp104.yaw;
                DbCamera_AddVecSph(sp7C, sp7C, &mut spFC);
            } else {
                spFC.pitch = -(spFC.pitch as libc::c_int) as s16;
                spFC.yaw =
                    (sp104.yaw as libc::c_int - 0x7fff as libc::c_int) as s16;
                DbCamera_AddVecSph(sp80, sp80, &mut spFC);
            }
            if (*dbCamera).unk_40 == 0xc as libc::c_int {
                (*dbCamera).unk_44 += 1
            } else { (*dbCamera).unk_44 = 0 as libc::c_int }
            (*dbCamera).unk_40 = 0xc as libc::c_int
        } else {
            (*dbCamera).unk_44 = 0 as libc::c_int;
            (*dbCamera).unk_40 = -(1 as libc::c_int)
        }
    } else if !((*sGlobalCtx).state.input[2 as libc::c_int as
                                              usize].cur.button as libc::c_int
                    | !(0x400 as libc::c_int | 0x20 as libc::c_int)) ==
                  0 as libc::c_int {
        spFC = sp104;
        spFC.r = temp_f2;
        spFC.pitch = 0 as libc::c_int as s16;
        if D_80161144 == 0 {
            spFC.yaw = sp104.yaw;
            DbCamera_AddVecSph(sp7C, sp7C, &mut spFC);
        } else {
            spFC.yaw =
                (sp104.yaw as libc::c_int - 0x7fff as libc::c_int) as s16;
            DbCamera_AddVecSph(sp80, sp80, &mut spFC);
        }
        if (*dbCamera).unk_40 == 1 as libc::c_int {
            (*dbCamera).unk_44 += 1
        } else { (*dbCamera).unk_44 = 0 as libc::c_int }
        (*dbCamera).unk_40 = 1 as libc::c_int
    } else if !((*sGlobalCtx).state.input[2 as libc::c_int as
                                              usize].cur.button as libc::c_int
                    | !(0x800 as libc::c_int | 0x20 as libc::c_int)) ==
                  0 as libc::c_int {
        spFC = sp104;
        spFC.r = -temp_f2;
        spFC.pitch = 0 as libc::c_int as s16;
        if D_80161144 == 0 {
            spFC.yaw = sp104.yaw;
            DbCamera_AddVecSph(sp7C, sp7C, &mut spFC);
        } else {
            spFC.yaw =
                (sp104.yaw as libc::c_int - 0x7fff as libc::c_int) as s16;
            DbCamera_AddVecSph(sp80, sp80, &mut spFC);
        }
        if (*dbCamera).unk_40 == 2 as libc::c_int {
            (*dbCamera).unk_44 += 1
        } else { (*dbCamera).unk_44 = 0 as libc::c_int }
        (*dbCamera).unk_40 = 2 as libc::c_int
    } else if !((*sGlobalCtx).state.input[2 as libc::c_int as
                                              usize].cur.button as libc::c_int
                    | !(0x800 as libc::c_int)) == 0 as libc::c_int {
        spFC = sp104;
        spFC.r = temp_f2;
        spFC.pitch = 0x3fff as libc::c_int as s16;
        spFC.yaw = sp104.yaw;
        if D_80161144 == 0 {
            DbCamera_AddVecSph(sp7C, sp7C, &mut spFC);
        } else { DbCamera_AddVecSph(sp80, sp80, &mut spFC); }
        if (*dbCamera).unk_40 == 3 as libc::c_int {
            (*dbCamera).unk_44 += 1
        } else { (*dbCamera).unk_44 = 0 as libc::c_int }
        (*dbCamera).unk_40 = 3 as libc::c_int
    } else if !((*sGlobalCtx).state.input[2 as libc::c_int as
                                              usize].cur.button as libc::c_int
                    | !(0x400 as libc::c_int)) == 0 as libc::c_int {
        spFC = sp104;
        spFC.r = temp_f2;
        spFC.pitch = -(0x3fff as libc::c_int) as s16;
        spFC.yaw = sp104.yaw;
        if D_80161144 == 0 {
            DbCamera_AddVecSph(sp7C, sp7C, &mut spFC);
        } else { DbCamera_AddVecSph(sp80, sp80, &mut spFC); }
        if (*dbCamera).unk_40 == 4 as libc::c_int {
            (*dbCamera).unk_44 += 1
        } else { (*dbCamera).unk_44 = 0 as libc::c_int }
        (*dbCamera).unk_40 = 4 as libc::c_int
    } else if !((*sGlobalCtx).state.input[2 as libc::c_int as
                                              usize].cur.button as libc::c_int
                    | !(0x100 as libc::c_int | 0x20 as libc::c_int)) ==
                  0 as libc::c_int ||
                  !((*sGlobalCtx).state.input[2 as libc::c_int as
                                                  usize].cur.button as
                        libc::c_int | !(0x100 as libc::c_int)) ==
                      0 as libc::c_int {
        spFC = sp104;
        spFC.r = temp_f2;
        spFC.pitch = 0 as libc::c_int as s16;
        if D_80161144 == 0 {
            spFC.yaw =
                (sp104.yaw as libc::c_int + 0x3fff as libc::c_int) as s16;
            DbCamera_AddVecSph(sp7C, sp7C, &mut spFC);
        } else {
            spFC.yaw =
                (sp104.yaw as libc::c_int - 0x3fff as libc::c_int) as s16;
            DbCamera_AddVecSph(sp80, sp80, &mut spFC);
        }
        if (*dbCamera).unk_40 == 5 as libc::c_int {
            (*dbCamera).unk_44 += 1
        } else { (*dbCamera).unk_44 = 0 as libc::c_int }
        (*dbCamera).unk_40 = 5 as libc::c_int
    } else if !((*sGlobalCtx).state.input[2 as libc::c_int as
                                              usize].cur.button as libc::c_int
                    | !(0x200 as libc::c_int | 0x20 as libc::c_int)) ==
                  0 as libc::c_int ||
                  !((*sGlobalCtx).state.input[2 as libc::c_int as
                                                  usize].cur.button as
                        libc::c_int | !(0x200 as libc::c_int)) ==
                      0 as libc::c_int {
        spFC = sp104;
        spFC.r = temp_f2;
        spFC.pitch = 0 as libc::c_int as s16;
        if D_80161144 == 0 {
            spFC.yaw =
                (sp104.yaw as libc::c_int - 0x3fff as libc::c_int) as s16;
            DbCamera_AddVecSph(sp7C, sp7C, &mut spFC);
        } else {
            spFC.yaw =
                (sp104.yaw as libc::c_int + 0x3fff as libc::c_int) as s16;
            DbCamera_AddVecSph(sp80, sp80, &mut spFC);
        }
        if (*dbCamera).unk_40 == 6 as libc::c_int {
            (*dbCamera).unk_44 += 1
        } else { (*dbCamera).unk_44 = 0 as libc::c_int }
        (*dbCamera).unk_40 = 6 as libc::c_int
    } else if !((*sGlobalCtx).state.input[2 as libc::c_int as
                                              usize].cur.button as libc::c_int
                    | !(0x4000 as libc::c_int | 0x20 as libc::c_int)) ==
                  0 as libc::c_int {
        sp104.r = sp104.r + temp_f2;
        if sp104.r > 30000.0f32 { sp104.r = 30000.0f32 }
        if (*dbCamera).unk_40 == 7 as libc::c_int {
            (*dbCamera).unk_44 += 1
        } else { (*dbCamera).unk_44 = 0 as libc::c_int }
        (*dbCamera).unk_40 = 7 as libc::c_int
    } else if !((*sGlobalCtx).state.input[2 as libc::c_int as
                                              usize].cur.button as libc::c_int
                    | !(0x4000 as libc::c_int)) == 0 as libc::c_int {
        spFC = sp104;
        spFC.r = temp_f2;
        if D_80161144 == 0 {
            spFC.yaw = sp104.yaw;
            DbCamera_AddVecSph(sp7C, sp7C, &mut spFC);
        } else {
            spFC.pitch = -(spFC.pitch as libc::c_int) as s16;
            spFC.yaw =
                (sp104.yaw as libc::c_int - 0x7fff as libc::c_int) as s16;
            DbCamera_AddVecSph(sp80, sp80, &mut spFC);
        }
        if (*dbCamera).unk_40 == 0xb as libc::c_int {
            (*dbCamera).unk_44 += 1
        } else { (*dbCamera).unk_44 = 0 as libc::c_int }
        (*dbCamera).unk_40 = 0xb as libc::c_int
    } else if !((*sGlobalCtx).state.input[2 as libc::c_int as
                                              usize].cur.button as libc::c_int
                    | !(0x8000 as libc::c_int | 0x20 as libc::c_int)) ==
                  0 as libc::c_int {
        sp104.r -= temp_f2;
        if sp104.r < 10.0f32 { sp104.r = 10.0f32 }
        if (*dbCamera).unk_40 == 8 as libc::c_int {
            (*dbCamera).unk_44 += 1
        } else { (*dbCamera).unk_44 = 0 as libc::c_int }
        (*dbCamera).unk_40 = 8 as libc::c_int
    } else if !((*sGlobalCtx).state.input[2 as libc::c_int as
                                              usize].cur.button as libc::c_int
                    | !(0x8000 as libc::c_int)) == 0 as libc::c_int {
        spFC = sp104;
        spFC.r = -temp_f2;
        if D_80161144 == 0 {
            spFC.yaw = sp104.yaw;
            DbCamera_AddVecSph(sp7C, sp7C, &mut spFC);
        } else {
            spFC.pitch = -(spFC.pitch as libc::c_int) as s16;
            spFC.yaw =
                (sp104.yaw as libc::c_int - 0x7fff as libc::c_int) as s16;
            DbCamera_AddVecSph(sp80, sp80, &mut spFC);
        }
        if (*dbCamera).unk_40 == 0xc as libc::c_int {
            (*dbCamera).unk_44 += 1
        } else { (*dbCamera).unk_44 = 0 as libc::c_int }
        (*dbCamera).unk_40 = 0xc as libc::c_int
    } else {
        (*dbCamera).unk_44 = 0 as libc::c_int;
        (*dbCamera).unk_40 = -(1 as libc::c_int)
    }
    if !((*sGlobalCtx).state.input[2 as libc::c_int as usize].cur.button as
             libc::c_int | !(0x10 as libc::c_int)) == 0 as libc::c_int {
        if (*dbCamera).unk_00 == 0 as libc::c_int {
            (*dbCamera).sub.unk_104A = (*cam).inputDir;
            *sp7C = (*cam).at;
            *sp80 = (*cam).eye;
            (*dbCamera).unk_1C.x = 0.0f32;
            (*dbCamera).unk_1C.z = 0.0f32;
            (*dbCamera).unk_1C.y = 1.0f32
        } else if (*dbCamera).sub.unk_08 as libc::c_int == 2 as libc::c_int {
            Audio_PlaySoundGeneral(0x4809 as libc::c_int as u16_0,
                                   &mut D_801333D4, 4 as libc::c_int as u8_0,
                                   &mut D_801333E0, &mut D_801333E0,
                                   &mut D_801333E8);
            (*dbCamera).sub.unk_08 = 0 as libc::c_int as s16;
            func_800B41DC(dbCamera, (*dbCamera).sub.unkIdx, cam);
        } else if !((*sGlobalCtx).state.input[2 as libc::c_int as
                                                  usize].press.button as
                        libc::c_int | !(0x10 as libc::c_int)) ==
                      0 as libc::c_int &&
                      !((*sGlobalCtx).state.input[2 as libc::c_int as
                                                      usize].cur.button as
                            libc::c_int | !(0x20 as libc::c_int)) ==
                          0 as libc::c_int {
            Audio_PlaySoundGeneral(0x480a as libc::c_int as u16_0,
                                   &mut D_801333D4, 4 as libc::c_int as u8_0,
                                   &mut D_801333E0, &mut D_801333E0,
                                   &mut D_801333E8);
            (*dbCamera).sub.nPoints =
                ((*dbCamera).sub.unkIdx as libc::c_int + 1 as libc::c_int) as
                    s16;
            func_800B4088(dbCamera, cam);
        } else if !((*sGlobalCtx).state.input[2 as libc::c_int as
                                                  usize].press.button as
                        libc::c_int | !(0x10 as libc::c_int)) ==
                      0 as libc::c_int {
            if (*dbCamera).sub.unkIdx as libc::c_int == 0x80 as libc::c_int {
                Audio_PlaySoundGeneral(0x4806 as libc::c_int as u16_0,
                                       &mut D_801333D4,
                                       4 as libc::c_int as u8_0,
                                       &mut D_801333E0, &mut D_801333E0,
                                       &mut D_801333E8);
            } else {
                Audio_PlaySoundGeneral(0x1802 as libc::c_int as u16_0,
                                       &mut D_801333D4,
                                       4 as libc::c_int as u8_0,
                                       &mut D_801333E0, &mut D_801333E0,
                                       &mut D_801333E8);
                func_800B42C0(dbCamera, cam);
                if (*dbCamera).sub.unkIdx as libc::c_int ==
                       (*dbCamera).sub.nPoints as libc::c_int -
                           1 as libc::c_int {
                    (*dbCamera).sub.unkIdx += 1;
                    (*dbCamera).sub.nPoints += 1;
                    func_800B4088(dbCamera, cam);
                }
            }
        }
    } else {
        temp_f0_5 =
            (*sGlobalCtx).state.input[2 as libc::c_int as usize].rel.stick_y
                as f32_0;
        temp_f2_2 =
            (*sGlobalCtx).state.input[2 as libc::c_int as usize].rel.stick_x
                as f32_0;
        pitch =
            (temp_f0_5 * temp_f0_5 / 600.0f32 * 0.8f32 * 182.04167f32 +
                 0.5f32) as s16;
        yaw =
            (temp_f2_2 * temp_f2_2 / 600.0f32 * 0.8f32 * 182.04167f32 +
                 0.5f32) as s16;
        if D_80161144 == 0 {
            sp104.pitch =
                (sp104.pitch as libc::c_int +
                     if temp_f0_5 >= 0.0f32 {
                         pitch as libc::c_int
                     } else { -(pitch as libc::c_int) } as s16 as libc::c_int)
                    as s16;
            sp104.yaw =
                (sp104.yaw as libc::c_int +
                     if temp_f2_2 >= 0.0f32 {
                         yaw as libc::c_int
                     } else { -(yaw as libc::c_int) } as s16 as libc::c_int)
                    as s16;
            DbCamera_AddVecSph(sp80, sp7C, &mut sp104);
            (*dbCamera).sub.unk_104A.x = -(sp104.pitch as libc::c_int) as s16;
            (*dbCamera).sub.unk_104A.y =
                (sp104.yaw as libc::c_int - 0x7fff as libc::c_int) as s16
        } else {
            sp104.pitch =
                (sp104.pitch as libc::c_int +
                     if temp_f0_5 >= 0.0f32 {
                         -(pitch as libc::c_int)
                     } else { pitch as libc::c_int } as s16 as libc::c_int) as
                    s16;
            sp104.yaw =
                (sp104.yaw as libc::c_int +
                     if temp_f2_2 >= 0.0f32 {
                         -(yaw as libc::c_int)
                     } else { yaw as libc::c_int } as s16 as libc::c_int) as
                    s16;
            DbCamera_AddVecSph(sp7C, sp80, &mut sp104);
            (*dbCamera).sub.unk_104A.x = sp104.pitch;
            (*dbCamera).sub.unk_104A.y = sp104.yaw
        }
        OLib_Vec3fDiffToVecSphGeo(&mut spF4, sp80, sp7C);
        DbCamera_CalcUpFromPitchYawRoll(&mut (*dbCamera).unk_1C, spF4.pitch,
                                        spF4.yaw,
                                        ((*dbCamera).rollDegrees *
                                             182.04167f32 + 0.5f32) as s16);
        if (*dbCamera).unk_00 == 1 as libc::c_int {
            if !((*sGlobalCtx).state.input[2 as libc::c_int as
                                               usize].cur.button as
                     libc::c_int | !(0x1 as libc::c_int)) == 0 as libc::c_int
               {
                (*cam).inputDir = (*dbCamera).sub.unk_104A;
                new_var2 = OLib_Vec3fDist(&mut (*cam).at, &mut (*cam).eye);
                (*cam).at = *sp7C;
                spFC = sp104;
                spFC.r = new_var2;
                DbCamera_AddVecSph(&mut (*cam).eye, &mut (*cam).at,
                                   &mut spFC);
            }
        }
    }
    if (*dbCamera).unk_00 == 1 as libc::c_int {
        (*gGameInfo).data[(2 as libc::c_int * 6 as libc::c_int *
                               16 as libc::c_int + 0 as libc::c_int) as usize]
            = 8 as libc::c_int as s16;
        func_8006376C(0xc as libc::c_int as u8_0, 5 as libc::c_int as u8_0,
                      0 as libc::c_int as u8_0, D_8012CEF4);
        if !((*sGlobalCtx).state.input[2 as libc::c_int as usize].cur.button
                 as libc::c_int | !(0x1 as libc::c_int)) == 0 as libc::c_int
               &&
               !(!((*sGlobalCtx).state.input[2 as libc::c_int as
                                                 usize].cur.button as
                       libc::c_int | !(0x20 as libc::c_int)) ==
                     0 as libc::c_int) {
            func_800B44E0(dbCamera, cam);
        } else {
            if !((*sGlobalCtx).state.input[2 as libc::c_int as
                                               usize].press.button as
                     libc::c_int | !(0x1 as libc::c_int)) == 0 as libc::c_int
                   &&
                   !((*sGlobalCtx).state.input[2 as libc::c_int as
                                                   usize].cur.button as
                         libc::c_int | !(0x20 as libc::c_int)) ==
                       0 as libc::c_int {
                Audio_PlaySoundGeneral(0x4803 as libc::c_int as u16_0,
                                       &mut D_801333D4,
                                       4 as libc::c_int as u8_0,
                                       &mut D_801333E0, &mut D_801333E0,
                                       &mut D_801333E8);
                osSyncPrintf(b"@@@\n@@@\n@@@/* *** spline point data ** start here *** */\n@@@\n\x00"
                                 as *const u8 as *const libc::c_char);
                DbCamera_PrintPoints(b"Lookat\x00" as *const u8 as
                                         *const libc::c_char,
                                     (*dbCamera).sub.nPoints,
                                     (*dbCamera).sub.lookAt.as_mut_ptr());
                DbCamera_PrintPoints(b"Position\x00" as *const u8 as
                                         *const libc::c_char,
                                     (*dbCamera).sub.nPoints,
                                     (*dbCamera).sub.position.as_mut_ptr());
                osSyncPrintf(b"@@@static short  nPoints = %d;\n@@@\n\x00" as
                                 *const u8 as *const libc::c_char,
                             (*dbCamera).sub.nPoints as libc::c_int);
                osSyncPrintf(b"@@@static short  nFrames = %d;\n@@@\n\x00" as
                                 *const u8 as *const libc::c_char,
                             (*dbCamera).sub.nFrames as libc::c_int);
                osSyncPrintf(b"@@@static short  Mode = %d;\n@@@\n\x00" as
                                 *const u8 as *const libc::c_char,
                             (*dbCamera).sub.mode as libc::c_int);
                osSyncPrintf(b"@@@\n@@@\n@@@/* *** spline point data ** finish! *** */\n@@@\n\x00"
                                 as *const u8 as *const libc::c_char);
            } else if !((*sGlobalCtx).state.input[2 as libc::c_int as
                                                      usize].press.button as
                            libc::c_int | !(0x2 as libc::c_int)) ==
                          0 as libc::c_int {
                Audio_PlaySoundGeneral(0x4809 as libc::c_int as u16_0,
                                       &mut D_801333D4,
                                       4 as libc::c_int as u8_0,
                                       &mut D_801333E0, &mut D_801333E0,
                                       &mut D_801333E8);
                (*dbCamera).sub.unk_08 =
                    (((*dbCamera).sub.unk_08 as libc::c_int +
                          1 as libc::c_int) % 3 as libc::c_int) as s16
            }
            if !((*sGlobalCtx).state.input[2 as libc::c_int as
                                               usize].press.button as
                     libc::c_int | !(0x8 as libc::c_int)) == 0 as libc::c_int
                   &&
                   !((*sGlobalCtx).state.input[2 as libc::c_int as
                                                   usize].cur.button as
                         libc::c_int | !(0x20 as libc::c_int)) ==
                       0 as libc::c_int {
                Audio_PlaySoundGeneral(0x1800 as libc::c_int as u16_0,
                                       &mut D_801333D4,
                                       4 as libc::c_int as u8_0,
                                       &mut D_801333E0, &mut D_801333E0,
                                       &mut D_801333E8);
                if (*dbCamera).sub.unkIdx as libc::c_int > 0 as libc::c_int {
                    (*dbCamera).sub.unkIdx -= 1
                } else {
                    (*dbCamera).sub.unkIdx =
                        ((*dbCamera).sub.nPoints as libc::c_int -
                             1 as libc::c_int) as s16
                }
            } else if !((*sGlobalCtx).state.input[2 as libc::c_int as
                                                      usize].press.button as
                            libc::c_int | !(0x8 as libc::c_int)) ==
                          0 as libc::c_int {
                Audio_PlaySoundGeneral(0x1800 as libc::c_int as u16_0,
                                       &mut D_801333D4,
                                       4 as libc::c_int as u8_0,
                                       &mut D_801333E0, &mut D_801333E0,
                                       &mut D_801333E8);
                if (*dbCamera).sub.unkIdx as libc::c_int > 0 as libc::c_int {
                    (*dbCamera).sub.unkIdx -= 1
                } else {
                    (*dbCamera).sub.unkIdx =
                        ((*dbCamera).sub.nPoints as libc::c_int -
                             1 as libc::c_int) as s16
                }
                if (*dbCamera).sub.unk_08 as libc::c_int == 2 as libc::c_int
                       &&
                       (*dbCamera).sub.unkIdx as libc::c_int !=
                           (*dbCamera).sub.nPoints as libc::c_int -
                               1 as libc::c_int {
                    func_800B4370(dbCamera, (*dbCamera).sub.unkIdx, cam);
                    (*dbCamera).roll = 0 as libc::c_int as s16;
                    (*dbCamera).fov = 60.0f32;
                    (*dbCamera).rollDegrees = 0 as libc::c_int as f32_0
                } else {
                    func_800B41DC(dbCamera, (*dbCamera).sub.unkIdx, cam);
                    (*dbCamera).fov =
                        (*dbCamera).sub.lookAt[(*dbCamera).sub.unkIdx as
                                                   usize].viewAngle;
                    (*dbCamera).roll =
                        (*dbCamera).sub.lookAt[(*dbCamera).sub.unkIdx as
                                                   usize].cameraRoll as s16;
                    (*dbCamera).rollDegrees =
                        (*dbCamera).roll as libc::c_int as libc::c_float *
                            1.40625f32
                }
            }
            if !((*sGlobalCtx).state.input[2 as libc::c_int as
                                               usize].cur.button as
                     libc::c_int | !(0x20 as libc::c_int)) == 0 as libc::c_int
                   &&
                   !((*sGlobalCtx).state.input[2 as libc::c_int as
                                                   usize].press.button as
                         libc::c_int | !(0x4 as libc::c_int)) ==
                       0 as libc::c_int {
                Audio_PlaySoundGeneral(0x1800 as libc::c_int as u16_0,
                                       &mut D_801333D4,
                                       4 as libc::c_int as u8_0,
                                       &mut D_801333E0, &mut D_801333E0,
                                       &mut D_801333E8);
                if ((*dbCamera).sub.unkIdx as libc::c_int) <
                       (*dbCamera).sub.nPoints as libc::c_int -
                           1 as libc::c_int {
                    (*dbCamera).sub.unkIdx += 1
                } else { (*dbCamera).sub.unkIdx = 0 as libc::c_int as s16 }
            } else if !((*sGlobalCtx).state.input[2 as libc::c_int as
                                                      usize].press.button as
                            libc::c_int | !(0x4 as libc::c_int)) ==
                          0 as libc::c_int {
                Audio_PlaySoundGeneral(0x1800 as libc::c_int as u16_0,
                                       &mut D_801333D4,
                                       4 as libc::c_int as u8_0,
                                       &mut D_801333E0, &mut D_801333E0,
                                       &mut D_801333E8);
                if ((*dbCamera).sub.unkIdx as libc::c_int) <
                       (*dbCamera).sub.nPoints as libc::c_int -
                           1 as libc::c_int {
                    (*dbCamera).sub.unkIdx += 1
                } else { (*dbCamera).sub.unkIdx = 0 as libc::c_int as s16 }
                if (*dbCamera).sub.unk_08 as libc::c_int == 2 as libc::c_int
                       &&
                       (*dbCamera).sub.unkIdx as libc::c_int !=
                           (*dbCamera).sub.nPoints as libc::c_int -
                               1 as libc::c_int {
                    func_800B4370(dbCamera, (*dbCamera).sub.unkIdx, cam);
                    (*dbCamera).roll = 0 as libc::c_int as s16;
                    (*dbCamera).fov = 60.0f32;
                    (*dbCamera).rollDegrees = 0 as libc::c_int as f32_0
                } else {
                    func_800B41DC(dbCamera, (*dbCamera).sub.unkIdx, cam);
                    (*dbCamera).fov =
                        (*dbCamera).sub.lookAt[(*dbCamera).sub.unkIdx as
                                                   usize].viewAngle;
                    (*dbCamera).roll =
                        (*dbCamera).sub.lookAt[(*dbCamera).sub.unkIdx as
                                                   usize].cameraRoll as s16;
                    (*dbCamera).rollDegrees =
                        (*dbCamera).roll as libc::c_int as libc::c_float *
                            1.40625f32
                }
            }
            func_8006376C(0xa as libc::c_int as u8_0,
                          6 as libc::c_int as u8_0,
                          if (*dbCamera).sub.unk_08 as libc::c_int ==
                                 0 as libc::c_int {
                              7 as libc::c_int
                          } else { 4 as libc::c_int } as u8_0,
                          D_8012D00C.as_mut_ptr());
            func_8006376C(0x11 as libc::c_int as u8_0,
                          6 as libc::c_int as u8_0,
                          if (*dbCamera).sub.unk_08 as libc::c_int ==
                                 1 as libc::c_int {
                              7 as libc::c_int
                          } else { 4 as libc::c_int } as u8_0,
                          D_8012D020.as_mut_ptr());
            func_8006376C(0x17 as libc::c_int as u8_0,
                          6 as libc::c_int as u8_0,
                          if (*dbCamera).sub.unk_08 as libc::c_int ==
                                 2 as libc::c_int {
                              7 as libc::c_int
                          } else { 4 as libc::c_int } as u8_0,
                          D_8012D034.as_mut_ptr());
            if (*dbCamera).sub.unkIdx as libc::c_int == 0x80 as libc::c_int {
                func_8006376C(0x10 as libc::c_int as u8_0,
                              0x1a as libc::c_int as u8_0,
                              1 as libc::c_int as u8_0,
                              D_8012CEF8[0 as libc::c_int as usize]);
            } else if (*dbCamera).sub.unkIdx as libc::c_int ==
                          (*dbCamera).sub.nPoints as libc::c_int -
                              1 as libc::c_int {
                *D_8012CEE0[7 as libc::c_int as
                                usize].offset(10 as libc::c_int as isize) =
                    ((*dbCamera).sub.nPoints as libc::c_int /
                         10 as libc::c_int + '0' as i32) as libc::c_char;
                *D_8012CEE0[7 as libc::c_int as
                                usize].offset(11 as libc::c_int as isize) =
                    ((*dbCamera).sub.nPoints as libc::c_int %
                         10 as libc::c_int + '0' as i32) as libc::c_char;
                func_8006376C(0xf as libc::c_int as u8_0,
                              0x1a as libc::c_int as u8_0,
                              1 as libc::c_int as u8_0,
                              D_8012CEE0[7 as libc::c_int as usize]);
            } else {
                *D_8012CEE0[8 as libc::c_int as
                                usize].offset(10 as libc::c_int as isize) =
                    (((*dbCamera).sub.unkIdx as libc::c_int +
                          1 as libc::c_int) / 10 as libc::c_int + '0' as i32)
                        as libc::c_char;
                *D_8012CEE0[8 as libc::c_int as
                                usize].offset(11 as libc::c_int as isize) =
                    (((*dbCamera).sub.unkIdx as libc::c_int +
                          1 as libc::c_int) % 10 as libc::c_int + '0' as i32)
                        as libc::c_char;
                *D_8012CEE0[8 as libc::c_int as
                                usize].offset(13 as libc::c_int as isize) =
                    (((*dbCamera).sub.nPoints as libc::c_int -
                          1 as libc::c_int) / 10 as libc::c_int + '0' as i32)
                        as libc::c_char;
                *D_8012CEE0[8 as libc::c_int as
                                usize].offset(14 as libc::c_int as isize) =
                    (((*dbCamera).sub.nPoints as libc::c_int -
                          1 as libc::c_int) % 10 as libc::c_int + '0' as i32)
                        as libc::c_char;
                func_8006376C(0xf as libc::c_int as u8_0,
                              0x1a as libc::c_int as u8_0,
                              1 as libc::c_int as u8_0,
                              D_8012CEE0[8 as libc::c_int as usize]);
            }
            match (*dbCamera).sub.unk_08 as libc::c_int {
                2 => { (*dbCamera).unk_3C = 0 as libc::c_int }
                0 => {
                    (*dbCamera).unk_3C = 0 as libc::c_int;
                    if (*dbCamera).sub.mode as libc::c_int != 1 as libc::c_int
                       {
                        func_8006376C(0xd as libc::c_int as u8_0,
                                      0x18 as libc::c_int as u8_0,
                                      3 as libc::c_int as u8_0,
                                      if D_80161144 == 0 {
                                          D_8012CF04
                                      } else { D_8012CF08 });
                        DbCamera_SetTextValue((sp104.pitch as libc::c_int as
                                                   libc::c_float *
                                                   0.00549325f32) as s16,
                                              &mut *D_8012D0E4.as_mut_ptr().offset(11
                                                                                       as
                                                                                       libc::c_int
                                                                                       as
                                                                                       isize),
                                              4 as libc::c_int as u8_0);
                        func_8006376C(0xf as libc::c_int as u8_0,
                                      0x17 as libc::c_int as u8_0,
                                      3 as libc::c_int as u8_0,
                                      D_8012D0E4.as_mut_ptr());
                        DbCamera_SetTextValue((sp104.yaw as libc::c_int as
                                                   libc::c_float *
                                                   0.00549325f32) as s16,
                                              &mut *D_8012D0F8.as_mut_ptr().offset(11
                                                                                       as
                                                                                       libc::c_int
                                                                                       as
                                                                                       isize),
                                              4 as libc::c_int as u8_0);
                        func_8006376C(0xf as libc::c_int as u8_0,
                                      0x18 as libc::c_int as u8_0,
                                      3 as libc::c_int as u8_0,
                                      D_8012D0F8.as_mut_ptr());
                        DbCamera_SetTextValue(sp104.r as s16,
                                              &mut *D_8012D0D4.as_mut_ptr().offset(8
                                                                                       as
                                                                                       libc::c_int
                                                                                       as
                                                                                       isize),
                                              6 as libc::c_int as u8_0);
                        func_8006376C(0xf as libc::c_int as u8_0,
                                      0x19 as libc::c_int as u8_0,
                                      3 as libc::c_int as u8_0,
                                      D_8012D0D4.as_mut_ptr());
                    } else {
                        func_8006376C(0xe as libc::c_int as u8_0,
                                      0x18 as libc::c_int as u8_0,
                                      3 as libc::c_int as u8_0, D_8012CF0C);
                        func_8006376C(0x10 as libc::c_int as u8_0,
                                      0x16 as libc::c_int as u8_0,
                                      3 as libc::c_int as u8_0, D_8012CF10);
                        sp110 = 'X' as i32 as libc::c_char;
                        DbCamera_SetTextValue((*temp_s6).pos.x as s16,
                                              &mut sp111,
                                              7 as libc::c_int as u8_0);
                        func_8006376C(0x10 as libc::c_int as u8_0,
                                      0x17 as libc::c_int as u8_0,
                                      3 as libc::c_int as u8_0, &mut sp110);
                        sp110 = 'Y' as i32 as libc::c_char;
                        DbCamera_SetTextValue((*temp_s6).pos.y as s16,
                                              &mut sp111,
                                              7 as libc::c_int as u8_0);
                        func_8006376C(0x10 as libc::c_int as u8_0,
                                      0x18 as libc::c_int as u8_0,
                                      3 as libc::c_int as u8_0, &mut sp110);
                        sp110 = 'Z' as i32 as libc::c_char;
                        DbCamera_SetTextValue((*temp_s6).pos.z as s16,
                                              &mut sp111,
                                              7 as libc::c_int as u8_0);
                        func_8006376C(0x10 as libc::c_int as u8_0,
                                      0x19 as libc::c_int as u8_0,
                                      3 as libc::c_int as u8_0, &mut sp110);
                    }
                }
                1 => {
                    (*dbCamera).unk_3C = 1 as libc::c_int;
                    if !((*sGlobalCtx).state.input[2 as libc::c_int as
                                                       usize].press.button as
                             libc::c_int | !(0x800 as libc::c_int)) ==
                           0 as libc::c_int {
                        Audio_PlaySoundGeneral(0x480c as libc::c_int as u16_0,
                                               &mut D_801333D4,
                                               4 as libc::c_int as u8_0,
                                               &mut D_801333E0,
                                               &mut D_801333E0,
                                               &mut D_801333E8);
                        if (*dbCamera).sub.unk_0A as libc::c_int ==
                               0 as libc::c_int {
                            (*dbCamera).sub.unk_0A = 5 as libc::c_int as s16
                        } else { (*dbCamera).sub.unk_0A -= 1 }
                    }
                    if !((*sGlobalCtx).state.input[2 as libc::c_int as
                                                       usize].press.button as
                             libc::c_int | !(0x400 as libc::c_int)) ==
                           0 as libc::c_int {
                        Audio_PlaySoundGeneral(0x480c as libc::c_int as u16_0,
                                               &mut D_801333D4,
                                               4 as libc::c_int as u8_0,
                                               &mut D_801333E0,
                                               &mut D_801333E0,
                                               &mut D_801333E8);
                        if (*dbCamera).sub.unk_0A as libc::c_int ==
                               5 as libc::c_int {
                            (*dbCamera).sub.unk_0A = 0 as libc::c_int as s16
                        } else { (*dbCamera).sub.unk_0A += 1 }
                    }
                    if !((*sGlobalCtx).state.input[2 as libc::c_int as
                                                       usize].press.button as
                             libc::c_int | !(0x200 as libc::c_int)) ==
                           0 as libc::c_int {
                        Audio_PlaySoundGeneral(0x480c as libc::c_int as u16_0,
                                               &mut D_801333D4,
                                               4 as libc::c_int as u8_0,
                                               &mut D_801333E0,
                                               &mut D_801333E0,
                                               &mut D_801333E8);
                        match (*dbCamera).sub.unk_0A as libc::c_int {
                            1 => {
                                if !((*sGlobalCtx).state.input[2 as
                                                                   libc::c_int
                                                                   as
                                                                   usize].cur.button
                                         as libc::c_int |
                                         !(0x20 as libc::c_int)) ==
                                       0 as libc::c_int {
                                    (*dbCamera).sub.lookAt[(*dbCamera).sub.unkIdx
                                                               as
                                                               usize].nextPointFrame
                                        =
                                        ((*dbCamera).sub.lookAt[(*dbCamera).sub.unkIdx
                                                                    as
                                                                    usize].nextPointFrame
                                             as libc::c_int -
                                             5 as libc::c_int) as u16_0
                                } else {
                                    (*dbCamera).sub.lookAt[(*dbCamera).sub.unkIdx
                                                               as
                                                               usize].nextPointFrame
                                        =
                                        (*dbCamera).sub.lookAt[(*dbCamera).sub.unkIdx
                                                                   as
                                                                   usize].nextPointFrame.wrapping_sub(1)
                                }
                            }
                            3 => {
                                (*dbCamera).sub.mode -= 1;
                                if (*dbCamera).sub.mode as libc::c_int ==
                                       -(1 as libc::c_int) {
                                    (*dbCamera).sub.mode =
                                        2 as libc::c_int as s16
                                }
                                if (*dbCamera).sub.mode as libc::c_int ==
                                       1 as libc::c_int {
                                    (*dbCamera).unk_78 =
                                        2 as libc::c_int as s16;
                                    i = 0 as libc::c_int;
                                    while i <
                                              (*dbCamera).sub.nPoints as
                                                  libc::c_int {
                                        DbCamera_Vec3SToF2(&mut (*(*dbCamera).sub.lookAt.as_mut_ptr().offset(i
                                                                                                                 as
                                                                                                                 isize)).pos,
                                                           &mut spD8);
                                        func_800B3F94(temp_s6, &mut spD8,
                                                      &mut (*(*dbCamera).sub.lookAt.as_mut_ptr().offset(i
                                                                                                            as
                                                                                                            isize)).pos);
                                        DbCamera_Vec3SToF2(&mut (*(*dbCamera).sub.position.as_mut_ptr().offset(i
                                                                                                                   as
                                                                                                                   isize)).pos,
                                                           &mut spD8);
                                        func_800B3F94(temp_s6, &mut spD8,
                                                      &mut (*(*dbCamera).sub.position.as_mut_ptr().offset(i
                                                                                                              as
                                                                                                              isize)).pos);
                                        i += 1
                                    }
                                } else {
                                    i = 0 as libc::c_int;
                                    while i <
                                              (*dbCamera).sub.nPoints as
                                                  libc::c_int {
                                        func_800B404C(temp_s6,
                                                      &mut (*(*dbCamera).sub.lookAt.as_mut_ptr().offset(i
                                                                                                            as
                                                                                                            isize)).pos,
                                                      &mut spD8);
                                        DbCamera_Vec3FToS(&mut spD8,
                                                          &mut (*(*dbCamera).sub.lookAt.as_mut_ptr().offset(i
                                                                                                                as
                                                                                                                isize)).pos);
                                        func_800B404C(temp_s6,
                                                      &mut (*(*dbCamera).sub.position.as_mut_ptr().offset(i
                                                                                                              as
                                                                                                              isize)).pos,
                                                      &mut spD8);
                                        DbCamera_Vec3FToS(&mut spD8,
                                                          &mut (*(*dbCamera).sub.position.as_mut_ptr().offset(i
                                                                                                                  as
                                                                                                                  isize)).pos);
                                        i += 1
                                    }
                                }
                            }
                            4 => { (*dbCamera).sub.unk_0C = 0 as libc::c_int }
                            2 => {
                                if !((*sGlobalCtx).state.input[2 as
                                                                   libc::c_int
                                                                   as
                                                                   usize].cur.button
                                         as libc::c_int |
                                         !(0x20 as libc::c_int)) ==
                                       0 as libc::c_int {
                                    (*dbCamera).sub.lookAt[(*dbCamera).sub.unkIdx
                                                               as
                                                               usize].cameraRoll
                                        =
                                        ((*dbCamera).sub.lookAt[(*dbCamera).sub.unkIdx
                                                                    as
                                                                    usize].cameraRoll
                                             as libc::c_int -
                                             5 as libc::c_int) as s8;
                                    (*dbCamera).roll =
                                        (*dbCamera).sub.lookAt[(*dbCamera).sub.unkIdx
                                                                   as
                                                                   usize].cameraRoll
                                            as s16
                                } else {
                                    (*dbCamera).sub.lookAt[(*dbCamera).sub.unkIdx
                                                               as
                                                               usize].cameraRoll
                                        -= 1;
                                    (*dbCamera).roll =
                                        (*dbCamera).sub.lookAt[(*dbCamera).sub.unkIdx
                                                                   as
                                                                   usize].cameraRoll
                                            as s16
                                }
                                (*dbCamera).rollDegrees =
                                    (*dbCamera).roll as libc::c_int as
                                        libc::c_float * 1.40625f32
                            }
                            _ => { }
                        }
                    }
                    if !((*sGlobalCtx).state.input[2 as libc::c_int as
                                                       usize].cur.button as
                             libc::c_int | !(0x200 as libc::c_int)) ==
                           0 as libc::c_int {
                        let fresh3 = D_8012D10C;
                        D_8012D10C = D_8012D10C + 1;
                        if fresh3 % 5 as libc::c_int == 0 as libc::c_int {
                            Audio_PlaySoundGeneral(0x480c as libc::c_int as
                                                       u16_0, &mut D_801333D4,
                                                   4 as libc::c_int as u8_0,
                                                   &mut D_801333E0,
                                                   &mut D_801333E0,
                                                   &mut D_801333E8);
                        }
                        match (*dbCamera).sub.unk_0A as libc::c_int {
                            0 => {
                                if !((*sGlobalCtx).state.input[2 as
                                                                   libc::c_int
                                                                   as
                                                                   usize].cur.button
                                         as libc::c_int |
                                         !(0x20 as libc::c_int)) ==
                                       0 as libc::c_int {
                                    (*dbCamera).sub.lookAt[(*dbCamera).sub.unkIdx
                                                               as
                                                               usize].viewAngle
                                        -= 1.0f32;
                                    (*dbCamera).fov =
                                        (*dbCamera).sub.lookAt[(*dbCamera).sub.unkIdx
                                                                   as
                                                                   usize].viewAngle
                                } else {
                                    (*dbCamera).sub.lookAt[(*dbCamera).sub.unkIdx
                                                               as
                                                               usize].viewAngle
                                        -= 0.2f32;
                                    (*dbCamera).fov =
                                        (*dbCamera).sub.lookAt[(*dbCamera).sub.unkIdx
                                                                   as
                                                                   usize].viewAngle
                                }
                            }
                            5 => {
                                if !((*sGlobalCtx).state.input[2 as
                                                                   libc::c_int
                                                                   as
                                                                   usize].cur.button
                                         as libc::c_int |
                                         !(0x20 as libc::c_int)) ==
                                       0 as libc::c_int {
                                    (*dbCamera).sub.nFrames =
                                        ((*dbCamera).sub.nFrames as
                                             libc::c_int - 10 as libc::c_int)
                                            as s16
                                } else { (*dbCamera).sub.nFrames -= 1 }
                                if ((*dbCamera).sub.nFrames as libc::c_int) <
                                       -(1 as libc::c_int) {
                                    if ((*dbCamera).sub.nPoints as
                                            libc::c_int) < 5 as libc::c_int {
                                        (*dbCamera).sub.nFrames =
                                            -(1 as libc::c_int) as s16
                                    } else {
                                        (*dbCamera).sub.nFrames =
                                            0 as libc::c_int as s16;
                                        i = 0 as libc::c_int;
                                        while i <
                                                  (*dbCamera).sub.nPoints as
                                                      libc::c_int {
                                            (*dbCamera).sub.nFrames =
                                                ((*dbCamera).sub.nFrames as
                                                     libc::c_int +
                                                     (*dbCamera).sub.lookAt[i
                                                                                as
                                                                                usize].nextPointFrame
                                                         as libc::c_int) as
                                                    s16;
                                            i += 1
                                        }
                                        i =
                                            (*dbCamera).sub.nFrames as
                                                libc::c_int /
                                                (*dbCamera).sub.nPoints as
                                                    libc::c_int;
                                        (*dbCamera).sub.nFrames =
                                            ((*dbCamera).sub.nFrames as
                                                 libc::c_int -
                                                 (i * 5 as libc::c_int) as s16
                                                     as libc::c_int) as s16
                                    }
                                }
                            }
                            _ => { }
                        }
                    }
                    if !((*sGlobalCtx).state.input[2 as libc::c_int as
                                                       usize].press.button as
                             libc::c_int | !(0x100 as libc::c_int)) ==
                           0 as libc::c_int {
                        Audio_PlaySoundGeneral(0x480c as libc::c_int as u16_0,
                                               &mut D_801333D4,
                                               4 as libc::c_int as u8_0,
                                               &mut D_801333E0,
                                               &mut D_801333E0,
                                               &mut D_801333E8);
                        let mut current_block_607: u64;
                        match (*dbCamera).sub.unk_0A as libc::c_int {
                            1 => {
                                if !((*sGlobalCtx).state.input[2 as
                                                                   libc::c_int
                                                                   as
                                                                   usize].cur.button
                                         as libc::c_int |
                                         !(0x20 as libc::c_int)) ==
                                       0 as libc::c_int {
                                    (*dbCamera).sub.lookAt[(*dbCamera).sub.unkIdx
                                                               as
                                                               usize].nextPointFrame
                                        =
                                        ((*dbCamera).sub.lookAt[(*dbCamera).sub.unkIdx
                                                                    as
                                                                    usize].nextPointFrame
                                             as libc::c_int +
                                             5 as libc::c_int) as u16_0
                                } else {
                                    (*dbCamera).sub.lookAt[(*dbCamera).sub.unkIdx
                                                               as
                                                               usize].nextPointFrame
                                        =
                                        (*dbCamera).sub.lookAt[(*dbCamera).sub.unkIdx
                                                                   as
                                                                   usize].nextPointFrame.wrapping_add(1)
                                }
                                current_block_607 = 9898910953550671049;
                            }
                            3 => {
                                (*dbCamera).sub.mode += 1;
                                if (*dbCamera).sub.mode as libc::c_int ==
                                       3 as libc::c_int {
                                    (*dbCamera).sub.mode =
                                        0 as libc::c_int as s16
                                }
                                if (*dbCamera).sub.mode as libc::c_int ==
                                       1 as libc::c_int {
                                    (*dbCamera).unk_78 =
                                        2 as libc::c_int as s16;
                                    i = 0 as libc::c_int;
                                    while i <
                                              (*dbCamera).sub.nPoints as
                                                  libc::c_int {
                                        DbCamera_Vec3SToF2(&mut (*(*dbCamera).sub.lookAt.as_mut_ptr().offset(i
                                                                                                                 as
                                                                                                                 isize)).pos,
                                                           &mut spD8);
                                        func_800B3F94(temp_s6, &mut spD8,
                                                      &mut (*(*dbCamera).sub.lookAt.as_mut_ptr().offset(i
                                                                                                            as
                                                                                                            isize)).pos);
                                        DbCamera_Vec3SToF2(&mut (*(*dbCamera).sub.position.as_mut_ptr().offset(i
                                                                                                                   as
                                                                                                                   isize)).pos,
                                                           &mut spD8);
                                        func_800B3F94(temp_s6, &mut spD8,
                                                      &mut (*(*dbCamera).sub.position.as_mut_ptr().offset(i
                                                                                                              as
                                                                                                              isize)).pos);
                                        i += 1
                                    }
                                } else {
                                    i = 0 as libc::c_int;
                                    while i <
                                              (*dbCamera).sub.nPoints as
                                                  libc::c_int {
                                        func_800B404C(temp_s6,
                                                      &mut (*(*dbCamera).sub.lookAt.as_mut_ptr().offset(i
                                                                                                            as
                                                                                                            isize)).pos,
                                                      &mut spD8);
                                        DbCamera_Vec3FToS(&mut spD8,
                                                          &mut (*(*dbCamera).sub.lookAt.as_mut_ptr().offset(i
                                                                                                                as
                                                                                                                isize)).pos);
                                        func_800B404C(temp_s6,
                                                      &mut (*(*dbCamera).sub.position.as_mut_ptr().offset(i
                                                                                                              as
                                                                                                              isize)).pos,
                                                      &mut spD8);
                                        DbCamera_Vec3FToS(&mut spD8,
                                                          &mut (*(*dbCamera).sub.position.as_mut_ptr().offset(i
                                                                                                                  as
                                                                                                                  isize)).pos);
                                        i += 1
                                    }
                                }
                                current_block_607 = 788615945127772514;
                            }
                            4 => { current_block_607 = 788615945127772514; }
                            2 => {
                                if !((*sGlobalCtx).state.input[2 as
                                                                   libc::c_int
                                                                   as
                                                                   usize].cur.button
                                         as libc::c_int |
                                         !(0x20 as libc::c_int)) ==
                                       0 as libc::c_int {
                                    (*dbCamera).sub.lookAt[(*dbCamera).sub.unkIdx
                                                               as
                                                               usize].cameraRoll
                                        =
                                        ((*dbCamera).sub.lookAt[(*dbCamera).sub.unkIdx
                                                                    as
                                                                    usize].cameraRoll
                                             as libc::c_int +
                                             5 as libc::c_int) as s8;
                                    (*dbCamera).roll =
                                        (*dbCamera).sub.lookAt[(*dbCamera).sub.unkIdx
                                                                   as
                                                                   usize].cameraRoll
                                            as s16
                                } else {
                                    (*dbCamera).sub.lookAt[(*dbCamera).sub.unkIdx
                                                               as
                                                               usize].cameraRoll
                                        += 1;
                                    (*dbCamera).roll =
                                        (*dbCamera).sub.lookAt[(*dbCamera).sub.unkIdx
                                                                   as
                                                                   usize].cameraRoll
                                            as s16
                                }
                                (*dbCamera).rollDegrees =
                                    (*dbCamera).roll as libc::c_int as
                                        libc::c_float * 1.40625f32;
                                current_block_607 = 9898910953550671049;
                            }
                            _ => { current_block_607 = 9898910953550671049; }
                        }
                        match current_block_607 {
                            788615945127772514 => {
                                (*dbCamera).sub.unk_0C = 1 as libc::c_int
                            }
                            _ => { }
                        }
                    }
                    if !((*sGlobalCtx).state.input[2 as libc::c_int as
                                                       usize].cur.button as
                             libc::c_int | !(0x100 as libc::c_int)) ==
                           0 as libc::c_int {
                        let fresh4 = D_8012D10C;
                        D_8012D10C = D_8012D10C + 1;
                        if fresh4 % 5 as libc::c_int == 0 as libc::c_int {
                            Audio_PlaySoundGeneral(0x480c as libc::c_int as
                                                       u16_0, &mut D_801333D4,
                                                   4 as libc::c_int as u8_0,
                                                   &mut D_801333E0,
                                                   &mut D_801333E0,
                                                   &mut D_801333E8);
                        }
                        match (*dbCamera).sub.unk_0A as libc::c_int {
                            0 => {
                                if !((*sGlobalCtx).state.input[2 as
                                                                   libc::c_int
                                                                   as
                                                                   usize].cur.button
                                         as libc::c_int |
                                         !(0x20 as libc::c_int)) ==
                                       0 as libc::c_int {
                                    (*dbCamera).sub.lookAt[(*dbCamera).sub.unkIdx
                                                               as
                                                               usize].viewAngle
                                        += 1.0f32;
                                    (*dbCamera).fov =
                                        (*dbCamera).sub.lookAt[(*dbCamera).sub.unkIdx
                                                                   as
                                                                   usize].viewAngle
                                } else {
                                    (*dbCamera).sub.lookAt[(*dbCamera).sub.unkIdx
                                                               as
                                                               usize].viewAngle
                                        += 0.2f32;
                                    (*dbCamera).fov =
                                        (*dbCamera).sub.lookAt[(*dbCamera).sub.unkIdx
                                                                   as
                                                                   usize].viewAngle
                                }
                            }
                            5 => {
                                if !((*sGlobalCtx).state.input[2 as
                                                                   libc::c_int
                                                                   as
                                                                   usize].cur.button
                                         as libc::c_int |
                                         !(0x20 as libc::c_int)) ==
                                       0 as libc::c_int {
                                    (*dbCamera).sub.nFrames =
                                        ((*dbCamera).sub.nFrames as
                                             libc::c_int + 10 as libc::c_int)
                                            as s16
                                } else { (*dbCamera).sub.nFrames += 1 }
                                !(&mut (*dbCamera).at as
                                      *mut Vec3f).is_null();
                            }
                            _ => { }
                        }
                    }
                    DbCamera_SetTextValue((*dbCamera).sub.lookAt[(*dbCamera).sub.unkIdx
                                                                     as
                                                                     usize].viewAngle
                                              as s16,
                                          &mut *D_8012D05C.as_mut_ptr().offset(10
                                                                                   as
                                                                                   libc::c_int
                                                                                   as
                                                                                   isize),
                                          3 as libc::c_int as u8_0);
                    func_8006376C(0x10 as libc::c_int as u8_0,
                                  0x14 as libc::c_int as u8_0,
                                  if (*dbCamera).sub.unk_0A as libc::c_int ==
                                         0 as libc::c_int {
                                      7 as libc::c_int
                                  } else { 4 as libc::c_int } as u8_0,
                                  D_8012D05C.as_mut_ptr());
                    DbCamera_SetTextValue((*dbCamera).sub.lookAt[(*dbCamera).sub.unkIdx
                                                                     as
                                                                     usize].nextPointFrame
                                              as s16,
                                          &mut *D_8012D070.as_mut_ptr().offset(9
                                                                                   as
                                                                                   libc::c_int
                                                                                   as
                                                                                   isize),
                                          3 as libc::c_int as u8_0);
                    func_8006376C(0x10 as libc::c_int as u8_0,
                                  0x15 as libc::c_int as u8_0,
                                  if (*dbCamera).sub.unk_0A as libc::c_int ==
                                         1 as libc::c_int {
                                      7 as libc::c_int
                                  } else { 4 as libc::c_int } as u8_0,
                                  D_8012D070.as_mut_ptr());
                    DbCamera_SetTextValue((*dbCamera).sub.lookAt[(*dbCamera).sub.unkIdx
                                                                     as
                                                                     usize].cameraRoll
                                              as s16,
                                          &mut *D_8012D084.as_mut_ptr().offset(10
                                                                                   as
                                                                                   libc::c_int
                                                                                   as
                                                                                   isize),
                                          3 as libc::c_int as u8_0);
                    func_8006376C(0x10 as libc::c_int as u8_0,
                                  0x16 as libc::c_int as u8_0,
                                  if (*dbCamera).sub.unk_0A as libc::c_int ==
                                         2 as libc::c_int {
                                      7 as libc::c_int
                                  } else { 4 as libc::c_int } as u8_0,
                                  D_8012D084.as_mut_ptr());
                    func_8006376C(0xf as libc::c_int as u8_0,
                                  0x17 as libc::c_int as u8_0,
                                  if (*dbCamera).sub.unk_0A as libc::c_int ==
                                         3 as libc::c_int {
                                      7 as libc::c_int
                                  } else { 4 as libc::c_int } as u8_0,
                                  if (*dbCamera).sub.mode as libc::c_int ==
                                         1 as libc::c_int {
                                      D_8012CF14
                                  } else if (*dbCamera).sub.mode as
                                                libc::c_int ==
                                                0 as libc::c_int {
                                      *D_8012CF18.as_mut_ptr()
                                  } else { D_8012CFB0 });
                    if (*dbCamera).sub.unk_0C != 0 {
                        D_8012D05C[80 as libc::c_int as usize] =
                            '>' as i32 as libc::c_char
                    } else {
                        D_8012D05C[80 as libc::c_int as usize] =
                            '<' as i32 as libc::c_char
                    }
                    D_8012D05C[81 as libc::c_int as usize] =
                        ' ' as i32 as libc::c_char;
                    D_8012D05C[94 as libc::c_int as usize] =
                        ' ' as i32 as libc::c_char;
                    if (*dbCamera).sub.unk_0C != 0 {
                        D_8012D05C[95 as libc::c_int as usize] =
                            '>' as i32 as libc::c_char
                    } else {
                        D_8012D05C[95 as libc::c_int as usize] =
                            '<' as i32 as libc::c_char
                    }
                    D_8012D05C[96 as libc::c_int as usize] =
                        '\u{0}' as i32 as libc::c_char;
                    func_8006376C(0xf as libc::c_int as u8_0,
                                  0x18 as libc::c_int as u8_0,
                                  if (*dbCamera).sub.unk_0A as libc::c_int ==
                                         4 as libc::c_int {
                                      7 as libc::c_int
                                  } else { 4 as libc::c_int } as u8_0,
                                  D_8012D0AC.as_mut_ptr());
                    DbCamera_SetTextValue((*dbCamera).sub.nFrames,
                                          &mut *D_8012D0C0.as_mut_ptr().offset(10
                                                                                   as
                                                                                   libc::c_int
                                                                                   as
                                                                                   isize),
                                          5 as libc::c_int as u8_0);
                    func_8006376C(0xf as libc::c_int as u8_0,
                                  0x19 as libc::c_int as u8_0,
                                  if (*dbCamera).sub.unk_0A as libc::c_int ==
                                         5 as libc::c_int {
                                      7 as libc::c_int
                                  } else { 4 as libc::c_int } as u8_0,
                                  if (*dbCamera).sub.nFrames as libc::c_int ==
                                         -(1 as libc::c_int) {
                                      *D_8012CF24.as_mut_ptr()
                                  } else { D_8012D0C0.as_mut_ptr() });
                }
                _ => { }
            }
            if (*dbCamera).sub.mode as libc::c_int != 1 as libc::c_int {
                func_8006376C(3 as libc::c_int as u8_0,
                              0x16 as libc::c_int as u8_0,
                              if (*dbCamera).sub.unk_08 as libc::c_int ==
                                     1 as libc::c_int &&
                                     (*dbCamera).sub.unk_0A as libc::c_int ==
                                         4 as libc::c_int && D_80161144 == 0 {
                                  7 as libc::c_int
                              } else if D_80161144 == 0 {
                                  4 as libc::c_int
                              } else { 3 as libc::c_int } as u8_0,
                              D_8012CF30);
                sp110 = 'X' as i32 as libc::c_char;
                DbCamera_SetTextValue((*dbCamera).at.x as s16, &mut sp111,
                                      6 as libc::c_int as u8_0);
                func_8006376C(3 as libc::c_int as u8_0,
                              0x17 as libc::c_int as u8_0,
                              2 as libc::c_int as u8_0, &mut sp110);
                sp110 = 'Y' as i32 as libc::c_char;
                DbCamera_SetTextValue((*dbCamera).at.y as s16, &mut sp111,
                                      6 as libc::c_int as u8_0);
                func_8006376C(3 as libc::c_int as u8_0,
                              0x18 as libc::c_int as u8_0,
                              2 as libc::c_int as u8_0, &mut sp110);
                sp110 = 'Z' as i32 as libc::c_char;
                DbCamera_SetTextValue((*dbCamera).at.z as s16, &mut sp111,
                                      6 as libc::c_int as u8_0);
                func_8006376C(3 as libc::c_int as u8_0,
                              0x19 as libc::c_int as u8_0,
                              2 as libc::c_int as u8_0, &mut sp110);
                func_8006376C(0x1e as libc::c_int as u8_0,
                              0x16 as libc::c_int as u8_0,
                              if (*dbCamera).sub.unk_08 as libc::c_int ==
                                     1 as libc::c_int &&
                                     (*dbCamera).sub.unk_0A as libc::c_int ==
                                         4 as libc::c_int && D_80161144 != 0 {
                                  7 as libc::c_int
                              } else if D_80161144 != 0 {
                                  4 as libc::c_int
                              } else { 3 as libc::c_int } as u8_0,
                              D_8012CF34);
                sp110 = 'X' as i32 as libc::c_char;
                DbCamera_SetTextValue((*dbCamera).eye.x as s16, &mut sp111,
                                      6 as libc::c_int as u8_0);
                func_8006376C(0x1e as libc::c_int as u8_0,
                              0x17 as libc::c_int as u8_0,
                              2 as libc::c_int as u8_0, &mut sp110);
                sp110 = 'Y' as i32 as libc::c_char;
                DbCamera_SetTextValue((*dbCamera).eye.y as s16, &mut sp111,
                                      6 as libc::c_int as u8_0);
                func_8006376C(0x1e as libc::c_int as u8_0,
                              0x18 as libc::c_int as u8_0,
                              2 as libc::c_int as u8_0, &mut sp110);
                sp110 = 'Z' as i32 as libc::c_char;
                DbCamera_SetTextValue((*dbCamera).eye.z as s16, &mut sp111,
                                      6 as libc::c_int as u8_0);
                func_8006376C(0x1e as libc::c_int as u8_0,
                              0x19 as libc::c_int as u8_0,
                              2 as libc::c_int as u8_0, &mut sp110);
            } else {
                !D_8012CEE0[0 as libc::c_int as usize].is_null();
                OLib_Vec3fDiffToVecSphGeo(&mut spFC, sp90, sp7C);
                spFC.yaw =
                    (spFC.yaw as libc::c_int -
                         (*cam).playerPosRot.rot.y as libc::c_int) as s16;
                func_8006376C(3 as libc::c_int as u8_0,
                              0x16 as libc::c_int as u8_0,
                              if (*dbCamera).sub.unk_08 as libc::c_int ==
                                     1 as libc::c_int &&
                                     (*dbCamera).sub.unk_0A as libc::c_int ==
                                         4 as libc::c_int && D_80161144 == 0 {
                                  7 as libc::c_int
                              } else if D_80161144 == 0 {
                                  4 as libc::c_int
                              } else { 3 as libc::c_int } as u8_0,
                              D_8012CF30);
                DbCamera_SetTextValue((spFC.pitch as libc::c_int as
                                           libc::c_float * 0.00549325f32) as
                                          s16,
                                      &mut *D_8012D0E4.as_mut_ptr().offset(10
                                                                               as
                                                                               libc::c_int
                                                                               as
                                                                               isize),
                                      4 as libc::c_int as u8_0);
                func_8006376C(3 as libc::c_int as u8_0,
                              0x17 as libc::c_int as u8_0,
                              3 as libc::c_int as u8_0,
                              D_8012D0E4.as_mut_ptr());
                DbCamera_SetTextValue((spFC.yaw as libc::c_int as
                                           libc::c_float * 0.00549325f32) as
                                          s16,
                                      &mut *D_8012D0F8.as_mut_ptr().offset(10
                                                                               as
                                                                               libc::c_int
                                                                               as
                                                                               isize),
                                      4 as libc::c_int as u8_0);
                func_8006376C(3 as libc::c_int as u8_0,
                              0x18 as libc::c_int as u8_0,
                              3 as libc::c_int as u8_0,
                              D_8012D0F8.as_mut_ptr());
                DbCamera_SetTextValue(spFC.r as s16,
                                      &mut *D_8012D0D4.as_mut_ptr().offset(7
                                                                               as
                                                                               libc::c_int
                                                                               as
                                                                               isize),
                                      6 as libc::c_int as u8_0);
                func_8006376C(3 as libc::c_int as u8_0,
                              0x19 as libc::c_int as u8_0,
                              3 as libc::c_int as u8_0,
                              D_8012D0D4.as_mut_ptr());
                OLib_Vec3fDiffToVecSphGeo(&mut spFC, sp90, sp80);
                spFC.yaw =
                    (spFC.yaw as libc::c_int -
                         (*cam).playerPosRot.rot.y as libc::c_int) as s16;
                func_8006376C(0x1e as libc::c_int as u8_0,
                              0x16 as libc::c_int as u8_0,
                              if (*dbCamera).sub.unk_08 as libc::c_int ==
                                     1 as libc::c_int &&
                                     (*dbCamera).sub.unk_0A as libc::c_int ==
                                         4 as libc::c_int && D_80161144 != 0 {
                                  7 as libc::c_int
                              } else if D_80161144 != 0 {
                                  4 as libc::c_int
                              } else { 3 as libc::c_int } as u8_0,
                              D_8012CF34);
                DbCamera_SetTextValue((spFC.pitch as libc::c_int as
                                           libc::c_float * 0.00549325f32) as
                                          s16,
                                      &mut *D_8012D0E4.as_mut_ptr().offset(10
                                                                               as
                                                                               libc::c_int
                                                                               as
                                                                               isize),
                                      4 as libc::c_int as u8_0);
                func_8006376C(0x1c as libc::c_int as u8_0,
                              0x17 as libc::c_int as u8_0,
                              3 as libc::c_int as u8_0,
                              D_8012D0E4.as_mut_ptr());
                DbCamera_SetTextValue((spFC.yaw as libc::c_int as
                                           libc::c_float * 0.00549325f32) as
                                          s16,
                                      &mut *D_8012D0F8.as_mut_ptr().offset(10
                                                                               as
                                                                               libc::c_int
                                                                               as
                                                                               isize),
                                      4 as libc::c_int as u8_0);
                func_8006376C(0x1c as libc::c_int as u8_0,
                              0x18 as libc::c_int as u8_0,
                              3 as libc::c_int as u8_0,
                              D_8012D0F8.as_mut_ptr());
                DbCamera_SetTextValue(spFC.r as s16,
                                      &mut *D_8012D0D4.as_mut_ptr().offset(7
                                                                               as
                                                                               libc::c_int
                                                                               as
                                                                               isize),
                                      6 as libc::c_int as u8_0);
                func_8006376C(0x1c as libc::c_int as u8_0,
                              0x19 as libc::c_int as u8_0,
                              3 as libc::c_int as u8_0,
                              D_8012D0D4.as_mut_ptr());
            }
            DebugDisplay_AddObject((*dbCamera).at.x,
                                   (*dbCamera).at.y + 1.0f32,
                                   (*dbCamera).at.z, 0 as libc::c_int as s16,
                                   0 as libc::c_int as s16,
                                   0 as libc::c_int as s16, 0.02f32, 2.0f32,
                                   0.02f32, 0xff as libc::c_int as u8_0,
                                   0xff as libc::c_int as u8_0,
                                   0x7f as libc::c_int as u8_0,
                                   0x40 as libc::c_int as u8_0,
                                   0 as libc::c_int as s16,
                                   (*(*cam).globalCtx).view.gfxCtx);
            DebugDisplay_AddObject((*dbCamera).at.x,
                                   (*dbCamera).at.y + 1.0f32,
                                   (*dbCamera).at.z, 0 as libc::c_int as s16,
                                   0 as libc::c_int as s16,
                                   0 as libc::c_int as s16, 2.0f32, 0.02f32,
                                   0.02f32, 0x7f as libc::c_int as u8_0,
                                   0xff as libc::c_int as u8_0,
                                   0xff as libc::c_int as u8_0,
                                   0x40 as libc::c_int as u8_0,
                                   0 as libc::c_int as s16,
                                   (*(*cam).globalCtx).view.gfxCtx);
            DebugDisplay_AddObject((*dbCamera).at.x,
                                   (*dbCamera).at.y + 1.0f32,
                                   (*dbCamera).at.z, 0 as libc::c_int as s16,
                                   0 as libc::c_int as s16,
                                   0 as libc::c_int as s16, 0.02f32, 0.02f32,
                                   2.0f32, 0xff as libc::c_int as u8_0,
                                   0x7f as libc::c_int as u8_0,
                                   0xff as libc::c_int as u8_0,
                                   0x40 as libc::c_int as u8_0,
                                   0 as libc::c_int as s16,
                                   (*(*cam).globalCtx).view.gfxCtx);
            if (*dbCamera).sub.unk_08 as libc::c_int == 2 as libc::c_int {
                i = 0 as libc::c_int;
                while i <
                          (*dbCamera).sub.nPoints as libc::c_int -
                              1 as libc::c_int {
                    if (*dbCamera).sub.mode as libc::c_int != 1 as libc::c_int
                       {
                        DbCamera_Vec3SToF2(&mut (*(*dbCamera).sub.position.as_mut_ptr().offset(i
                                                                                                   as
                                                                                                   isize)).pos,
                                           &mut spAC);
                        DbCamera_Vec3SToF2(&mut (*(*dbCamera).sub.lookAt.as_mut_ptr().offset(i
                                                                                                 as
                                                                                                 isize)).pos,
                                           &mut spB8);
                    } else {
                        func_800B404C(temp_s6,
                                      &mut (*(*dbCamera).sub.lookAt.as_mut_ptr().offset(i
                                                                                            as
                                                                                            isize)).pos,
                                      &mut spB8);
                        func_800B404C(temp_s6,
                                      &mut (*(*dbCamera).sub.position.as_mut_ptr().offset(i
                                                                                              as
                                                                                              isize)).pos,
                                      &mut spAC);
                    }
                    OLib_Vec3fDiffToVecSphGeo(&mut spFC, &mut spAC,
                                              &mut spB8);
                    spAA =
                        ((*dbCamera).sub.lookAt[i as usize].cameraRoll as
                             libc::c_int * 0xb6 as libc::c_int) as s16;
                    if i == (*dbCamera).sub.unkIdx as libc::c_int {
                        DebugDisplay_AddObject(spAC.x, spAC.y, spAC.z,
                                               (spFC.pitch as libc::c_int *
                                                    -(1 as libc::c_int)) as
                                                   s16, spFC.yaw, spAA,
                                               0.5f32, 0.5f32, 0.5f32,
                                               0x7f as libc::c_int as u8_0,
                                               0xff as libc::c_int as u8_0,
                                               0x7f as libc::c_int as u8_0,
                                               0x80 as libc::c_int as u8_0,
                                               5 as libc::c_int as s16,
                                               (*(*cam).globalCtx).view.gfxCtx);
                        DebugDisplay_AddObject(spB8.x, spB8.y, spB8.z,
                                               (spFC.pitch as libc::c_int *
                                                    -(1 as libc::c_int)) as
                                                   s16, spFC.yaw, spAA,
                                               1.5f32, 2.0f32, 1.0f32,
                                               0x7f as libc::c_int as u8_0,
                                               0xff as libc::c_int as u8_0,
                                               0x7f as libc::c_int as u8_0,
                                               0x80 as libc::c_int as u8_0,
                                               4 as libc::c_int as s16,
                                               (*(*cam).globalCtx).view.gfxCtx);
                    } else {
                        DebugDisplay_AddObject(spAC.x, spAC.y, spAC.z,
                                               (spFC.pitch as libc::c_int *
                                                    -(1 as libc::c_int)) as
                                                   s16, spFC.yaw, spAA,
                                               0.5f32, 0.5f32, 0.5f32,
                                               0xff as libc::c_int as u8_0,
                                               0x7f as libc::c_int as u8_0,
                                               0x7f as libc::c_int as u8_0,
                                               0x80 as libc::c_int as u8_0,
                                               5 as libc::c_int as s16,
                                               (*(*cam).globalCtx).view.gfxCtx);
                        DebugDisplay_AddObject(spB8.x, spB8.y, spB8.z,
                                               (spFC.pitch as libc::c_int *
                                                    -(1 as libc::c_int)) as
                                                   s16, spFC.yaw, spAA,
                                               1.5f32, 2.0f32, 1.0f32,
                                               0xff as libc::c_int as u8_0,
                                               0x7f as libc::c_int as u8_0,
                                               0x7f as libc::c_int as u8_0,
                                               0x80 as libc::c_int as u8_0,
                                               4 as libc::c_int as s16,
                                               (*(*cam).globalCtx).view.gfxCtx);
                    }
                    i += 1
                }
            }
        }
    } else {
        (*gGameInfo).data[(2 as libc::c_int * 6 as libc::c_int *
                               16 as libc::c_int + 0 as libc::c_int) as usize]
            = 8 as libc::c_int as s16;
        (*dbCamera).roll = 0 as libc::c_int as s16;
        (*dbCamera).fov = 60.0f32;
        (*dbCamera).rollDegrees =
            (*dbCamera).roll as libc::c_int as libc::c_float * 1.40625f32;
        if !((*sGlobalCtx).state.input[2 as libc::c_int as usize].press.button
                 as libc::c_int | !(0x2 as libc::c_int)) == 0 as libc::c_int {
            Audio_PlaySoundGeneral(0x4809 as libc::c_int as u16_0,
                                   &mut D_801333D4, 4 as libc::c_int as u8_0,
                                   &mut D_801333E0, &mut D_801333E0,
                                   &mut D_801333E8);
            (*dbCamera).unk_78 =
                (((*dbCamera).unk_78 as libc::c_int + 1 as libc::c_int) %
                     3 as libc::c_int) as s16;
            (*dbCamera).unk_38 = -(1 as libc::c_int)
        }
        func_8006376C(0xe as libc::c_int as u8_0, 5 as libc::c_int as u8_0,
                      0 as libc::c_int as u8_0, D_8012CF38);
        func_8006376C(9 as libc::c_int as u8_0, 6 as libc::c_int as u8_0,
                      if (*dbCamera).unk_78 as libc::c_int == 0 as libc::c_int
                         {
                          7 as libc::c_int
                      } else { 4 as libc::c_int } as u8_0,
                      D_8012CFD0.as_mut_ptr());
        func_8006376C(0x11 as libc::c_int as u8_0, 6 as libc::c_int as u8_0,
                      if (*dbCamera).unk_78 as libc::c_int == 1 as libc::c_int
                         {
                          7 as libc::c_int
                      } else { 4 as libc::c_int } as u8_0,
                      D_8012CFE4.as_mut_ptr());
        func_8006376C(0x18 as libc::c_int as u8_0, 6 as libc::c_int as u8_0,
                      if (*dbCamera).unk_78 as libc::c_int == 2 as libc::c_int
                         {
                          7 as libc::c_int
                      } else { 4 as libc::c_int } as u8_0,
                      D_8012CFF8.as_mut_ptr());
        func_8006376C(3 as libc::c_int as u8_0, 0x16 as libc::c_int as u8_0,
                      if D_80161144 != 0 {
                          3 as libc::c_int
                      } else { 4 as libc::c_int } as u8_0, D_8012CF30);
        sp110 = 'X' as i32 as libc::c_char;
        DbCamera_SetTextValue((*dbCamera).at.x as s16, &mut sp111,
                              6 as libc::c_int as u8_0);
        func_8006376C(3 as libc::c_int as u8_0, 0x17 as libc::c_int as u8_0,
                      2 as libc::c_int as u8_0, &mut sp110);
        sp110 = 'Y' as i32 as libc::c_char;
        DbCamera_SetTextValue((*dbCamera).at.y as s16, &mut sp111,
                              6 as libc::c_int as u8_0);
        func_8006376C(3 as libc::c_int as u8_0, 0x18 as libc::c_int as u8_0,
                      2 as libc::c_int as u8_0, &mut sp110);
        sp110 = 'Z' as i32 as libc::c_char;
        DbCamera_SetTextValue((*dbCamera).at.z as s16, &mut sp111,
                              6 as libc::c_int as u8_0);
        func_8006376C(3 as libc::c_int as u8_0, 0x19 as libc::c_int as u8_0,
                      2 as libc::c_int as u8_0, &mut sp110);
        func_8006376C(0x1e as libc::c_int as u8_0,
                      0x16 as libc::c_int as u8_0,
                      if D_80161144 != 0 {
                          4 as libc::c_int
                      } else { 3 as libc::c_int } as u8_0, D_8012CF34);
        sp110 = 'X' as i32 as libc::c_char;
        DbCamera_SetTextValue((*dbCamera).eye.x as s16, &mut sp111,
                              6 as libc::c_int as u8_0);
        func_8006376C(0x1e as libc::c_int as u8_0,
                      0x17 as libc::c_int as u8_0, 2 as libc::c_int as u8_0,
                      &mut sp110);
        sp110 = 'Y' as i32 as libc::c_char;
        DbCamera_SetTextValue((*dbCamera).eye.y as s16, &mut sp111,
                              6 as libc::c_int as u8_0);
        func_8006376C(0x1e as libc::c_int as u8_0,
                      0x18 as libc::c_int as u8_0, 2 as libc::c_int as u8_0,
                      &mut sp110);
        sp110 = 'Z' as i32 as libc::c_char;
        DbCamera_SetTextValue((*dbCamera).eye.z as s16, &mut sp111,
                              6 as libc::c_int as u8_0);
        func_8006376C(0x1e as libc::c_int as u8_0,
                      0x19 as libc::c_int as u8_0, 2 as libc::c_int as u8_0,
                      &mut sp110);
        func_8006376C(0xd as libc::c_int as u8_0, 0x18 as libc::c_int as u8_0,
                      3 as libc::c_int as u8_0,
                      if D_80161144 == 0 { D_8012CF04 } else { D_8012CF08 });
        DbCamera_SetTextValue((sp104.pitch as libc::c_int as libc::c_float *
                                   0.00549325f32) as s16,
                              &mut *D_8012D0E4.as_mut_ptr().offset(11 as
                                                                       libc::c_int
                                                                       as
                                                                       isize),
                              4 as libc::c_int as u8_0);
        func_8006376C(0xf as libc::c_int as u8_0, 0x17 as libc::c_int as u8_0,
                      3 as libc::c_int as u8_0, D_8012D0E4.as_mut_ptr());
        DbCamera_SetTextValue((sp104.yaw as libc::c_int as libc::c_float *
                                   0.00549325f32) as s16,
                              &mut *D_8012D0F8.as_mut_ptr().offset(11 as
                                                                       libc::c_int
                                                                       as
                                                                       isize),
                              4 as libc::c_int as u8_0);
        func_8006376C(0xf as libc::c_int as u8_0, 0x18 as libc::c_int as u8_0,
                      3 as libc::c_int as u8_0, D_8012D0F8.as_mut_ptr());
        DbCamera_SetTextValue(sp104.r as s16,
                              &mut *D_8012D0D4.as_mut_ptr().offset(8 as
                                                                       libc::c_int
                                                                       as
                                                                       isize),
                              6 as libc::c_int as u8_0);
        func_8006376C(0xf as libc::c_int as u8_0, 0x19 as libc::c_int as u8_0,
                      3 as libc::c_int as u8_0, D_8012D0D4.as_mut_ptr());
        if (*dbCamera).unk_3C != 0 {
            func_8006376C(0x10 as libc::c_int as u8_0,
                          0x1a as libc::c_int as u8_0,
                          1 as libc::c_int as u8_0, D_8012CF3C);
        } else {
            func_8006376C(0x10 as libc::c_int as u8_0,
                          0x1a as libc::c_int as u8_0,
                          1 as libc::c_int as u8_0, D_8012CF40);
        }
        D_8012D110 += 1;
        D_8012D110 %= 50 as libc::c_int;
        OLib_Vec3fDiffToVecSphGeo(&mut spA0, &mut (*cam).eye, &mut (*cam).at);
        DebugDisplay_AddObject((*dbCamera).at.x, (*dbCamera).at.y + 1.0f32,
                               (*dbCamera).at.z, 0 as libc::c_int as s16,
                               0 as libc::c_int as s16,
                               0 as libc::c_int as s16, 0.02f32, 2.0f32,
                               0.02f32, 0xff as libc::c_int as u8_0,
                               0xff as libc::c_int as u8_0,
                               0x7f as libc::c_int as u8_0,
                               0x2d as libc::c_int as u8_0,
                               0 as libc::c_int as s16,
                               (*(*cam).globalCtx).view.gfxCtx);
        DebugDisplay_AddObject((*dbCamera).at.x, (*dbCamera).at.y + 1.0f32,
                               (*dbCamera).at.z, 0 as libc::c_int as s16,
                               0 as libc::c_int as s16,
                               0 as libc::c_int as s16, 2.0f32, 0.02f32,
                               0.02f32, 0x7f as libc::c_int as u8_0,
                               0xff as libc::c_int as u8_0,
                               0xff as libc::c_int as u8_0,
                               0x2d as libc::c_int as u8_0,
                               0 as libc::c_int as s16,
                               (*(*cam).globalCtx).view.gfxCtx);
        DebugDisplay_AddObject((*dbCamera).at.x, (*dbCamera).at.y + 1.0f32,
                               (*dbCamera).at.z, 0 as libc::c_int as s16,
                               0 as libc::c_int as s16,
                               0 as libc::c_int as s16, 0.02f32, 0.02f32,
                               2.0f32, 0xff as libc::c_int as u8_0,
                               0x7f as libc::c_int as u8_0,
                               0xff as libc::c_int as u8_0,
                               0x2d as libc::c_int as u8_0,
                               0 as libc::c_int as s16,
                               (*(*cam).globalCtx).view.gfxCtx);
        DebugDisplay_AddObject((*cam).eye.x, (*cam).eye.y, (*cam).eye.z,
                               (spA0.pitch as libc::c_int *
                                    -(1 as libc::c_int)) as s16, spA0.yaw,
                               0 as libc::c_int as s16, 0.5f32, 0.5f32,
                               0.5f32, 0xff as libc::c_int as u8_0,
                               0x7f as libc::c_int as u8_0,
                               0x7f as libc::c_int as u8_0,
                               0x80 as libc::c_int as u8_0,
                               5 as libc::c_int as s16,
                               (*(*cam).globalCtx).view.gfxCtx);
        DebugDisplay_AddObject((*cam).at.x, (*cam).at.y, (*cam).at.z,
                               (spA0.pitch as libc::c_int *
                                    -(1 as libc::c_int)) as s16, spA0.yaw,
                               0 as libc::c_int as s16, 1.5f32, 2.0f32,
                               1.0f32, 0xff as libc::c_int as u8_0,
                               0x7f as libc::c_int as u8_0,
                               0x7f as libc::c_int as u8_0,
                               0x80 as libc::c_int as u8_0,
                               4 as libc::c_int as s16,
                               (*(*cam).globalCtx).view.gfxCtx);
        OLib_Vec3fDiffToVecSphGeo(&mut spA0, &mut (*cam).eyeNext,
                                  &mut (*cam).at);
        DebugDisplay_AddObject((*cam).eyeNext.x, (*cam).eyeNext.y,
                               (*cam).eyeNext.z,
                               (spA0.pitch as libc::c_int *
                                    -(1 as libc::c_int)) as s16, spA0.yaw,
                               0 as libc::c_int as s16, 0.5f32, 0.5f32,
                               0.5f32, 0xff as libc::c_int as u8_0,
                               0xc0 as libc::c_int as u8_0,
                               0x7f as libc::c_int as u8_0,
                               0x50 as libc::c_int as u8_0,
                               5 as libc::c_int as s16,
                               (*(*cam).globalCtx).view.gfxCtx);
    };
}
static mut sCurFileIdx: s16 = 0;
static mut sLastFileIdx: s16 = 0;
// holds the file index of the slot to move
// is the size correct? todo: add ALIGN32 for sizeof in Mempak functions, replace 0xF with sizeof()
static mut sDbCameraCuts: [DbCameraCut; 16] =
    [DbCameraCut{letter: 0,
                 unk_01: 0,
                 mode: 0,
                 position:
                     0 as *const CutsceneCameraPoint as
                         *mut CutsceneCameraPoint,
                 lookAt:
                     0 as *const CutsceneCameraPoint as
                         *mut CutsceneCameraPoint,
                 nFrames: 0,
                 nPoints: 0,}; 16];
static mut D_80161250: [libc::c_char; 128] = [0; 128];
static mut sLetters: [libc::c_char; 26] = [0; 26];
static mut D_801612EA: libc::c_char = 0;
static mut sAllocSize: s32 = 0;
#[no_mangle]
pub unsafe extern "C" fn DbCamera_GetFirstAvailableLetter() -> s32 {
    let mut i: s32 = 0;
    i = 0 as libc::c_int;
    while i <
              (::std::mem::size_of::<[libc::c_char; 26]>() as
                   libc::c_ulong).wrapping_div(::std::mem::size_of::<libc::c_char>()
                                                   as libc::c_ulong) as s32 {
        match sLetters[i as usize] as libc::c_int {
            79 => { }
            _ => { return 'A' as i32 + i }
        }
        i += 1
    }
    return '?' as i32;
}
#[no_mangle]
pub unsafe extern "C" fn DbCamera_InitCut(mut idx: s32,
                                          mut sub: *mut DbCameraSub)
 -> libc::c_char {
    let mut i: s32 = 0;
    sDbCameraCuts[idx as usize].unk_01 = 0x61 as libc::c_int as u8_0;
    sDbCameraCuts[idx as usize].letter =
        DbCamera_GetFirstAvailableLetter() as libc::c_char;
    D_80161250[(0x3f as libc::c_int +
                    sDbCameraCuts[idx as usize].letter as libc::c_int) as
                   usize] = 'O' as i32 as libc::c_char;
    i =
        ((*sub).nPoints as
             libc::c_uint).wrapping_mul(::std::mem::size_of::<CutsceneCameraPoint>()
                                            as libc::c_ulong) as s32;
    sDbCameraCuts[idx as usize].lookAt =
        DebugArena_MallocDebug(i as u32_0,
                               b"../db_camera.c\x00" as *const u8 as
                                   *const libc::c_char, 2748 as libc::c_int)
            as *mut CutsceneCameraPoint;
    if sDbCameraCuts[idx as usize].lookAt.is_null() {
        // "Debug camera memory allocation failure"
        osSyncPrintf(b"%s: %d: \xe3\x83\x87\xe3\x83\x90\xe3\x83\x83\xe3\x82\xb0\xe3\x82\xab\xe3\x83\xa1\xe3\x83\xa9 \xe3\x83\xa1\xe3\x83\xa2\xe3\x83\xaa\xe7\xa2\xba\xe4\xbf\x9d\xe5\xa4\xb1\xe6\x95\x97\xef\xbc\x81\xef\xbc\x81\n\x00"
                         as *const u8 as *const libc::c_char,
                     b"../db_camera.c\x00" as *const u8 as
                         *const libc::c_char, 2751 as libc::c_int);
        return '?' as i32 as libc::c_char
    }
    sDbCameraCuts[idx as usize].position =
        DebugArena_MallocDebug(i as u32_0,
                               b"../db_camera.c\x00" as *const u8 as
                                   *const libc::c_char, 2754 as libc::c_int)
            as *mut CutsceneCameraPoint;
    if sDbCameraCuts[idx as usize].position.is_null() {
        // "Debug camera memory allocation failure"
        osSyncPrintf(b"%s: %d: \xe3\x83\x87\xe3\x83\x90\xe3\x83\x83\xe3\x82\xb0\xe3\x82\xab\xe3\x83\xa1\xe3\x83\xa9 \xe3\x83\xa1\xe3\x83\xa2\xe3\x83\xaa\xe7\xa2\xba\xe4\xbf\x9d\xe5\xa4\xb1\xe6\x95\x97\xef\xbc\x81\xef\xbc\x81\n\x00"
                         as *const u8 as *const libc::c_char,
                     b"../db_camera.c\x00" as *const u8 as
                         *const libc::c_char, 2757 as libc::c_int);
        DebugArena_FreeDebug(sDbCameraCuts[idx as usize].lookAt as
                                 *mut libc::c_void,
                             b"../db_camera.c\x00" as *const u8 as
                                 *const libc::c_char, 2758 as libc::c_int);
        sDbCameraCuts[idx as usize].lookAt = 0 as *mut CutsceneCameraPoint;
        return '?' as i32 as libc::c_char
    }
    sDbCameraCuts[idx as usize].mode = (*sub).mode;
    sDbCameraCuts[idx as usize].nFrames = (*sub).nFrames;
    sDbCameraCuts[idx as usize].nPoints = (*sub).nPoints;
    i = 0 as libc::c_int;
    while i < (*sub).nPoints as libc::c_int {
        *sDbCameraCuts[idx as usize].lookAt.offset(i as isize) =
            (*sub).lookAt[i as usize];
        *sDbCameraCuts[idx as usize].position.offset(i as isize) =
            (*sub).position[i as usize];
        i += 1
    }
    return sDbCameraCuts[idx as usize].letter;
}
#[no_mangle]
pub unsafe extern "C" fn DbCamera_ResetCut(mut idx: s32,
                                           mut shouldFree: s32) {
    if sDbCameraCuts[idx as usize].letter as libc::c_int != '?' as i32 {
        D_80161250[(0x3f as libc::c_int +
                        sDbCameraCuts[idx as usize].letter as libc::c_int) as
                       usize] = 'X' as i32 as libc::c_char
    }
    if shouldFree != 0 {
        DebugArena_FreeDebug(sDbCameraCuts[idx as usize].lookAt as
                                 *mut libc::c_void,
                             b"../db_camera.c\x00" as *const u8 as
                                 *const libc::c_char, 2784 as libc::c_int);
        DebugArena_FreeDebug(sDbCameraCuts[idx as usize].position as
                                 *mut libc::c_void,
                             b"../db_camera.c\x00" as *const u8 as
                                 *const libc::c_char, 2785 as libc::c_int);
    }
    sDbCameraCuts[idx as usize].letter = '?' as i32 as libc::c_char;
    sDbCameraCuts[idx as usize].lookAt = 0 as *mut CutsceneCameraPoint;
    sDbCameraCuts[idx as usize].position = 0 as *mut CutsceneCameraPoint;
    sDbCameraCuts[idx as usize].mode = 0 as libc::c_int as s16;
    sDbCameraCuts[idx as usize].nFrames = 0 as libc::c_int as s16;
    sDbCameraCuts[idx as usize].nPoints = 0 as libc::c_int as s16;
}
#[no_mangle]
pub unsafe extern "C" fn DbCamera_CalcMempakAllocSize() -> s32 {
    let mut i: s32 = 0;
    sAllocSize = 0 as libc::c_int;
    i = 0 as libc::c_int;
    while i <
              (::std::mem::size_of::<[DbCameraCut; 16]>() as
                   libc::c_ulong).wrapping_div(::std::mem::size_of::<DbCameraCut>()
                                                   as libc::c_ulong) as s32 -
                  1 as libc::c_int {
        if sDbCameraCuts[i as usize].letter as libc::c_int != '?' as i32 {
            sAllocSize =
                (sAllocSize as
                     libc::c_uint).wrapping_add(((sDbCameraCuts[i as
                                                                    usize].nPoints
                                                      as
                                                      libc::c_uint).wrapping_mul(::std::mem::size_of::<CutsceneCameraPoint>()
                                                                                     as
                                                                                     libc::c_ulong).wrapping_add(0x1f
                                                                                                                     as
                                                                                                                     libc::c_int
                                                                                                                     as
                                                                                                                     libc::c_uint)
                                                     &
                                                     !(0x1f as libc::c_int) as
                                                         libc::c_uint).wrapping_mul(2
                                                                                        as
                                                                                        libc::c_int
                                                                                        as
                                                                                        libc::c_uint))
                    as s32 as s32
        }
        i += 1
    }
    sAllocSize += 0x100 as libc::c_int;
    sAllocSize = sAllocSize + 0xff as libc::c_int & !(0xff as libc::c_int);
    return sAllocSize;
}
#[no_mangle]
pub unsafe extern "C" fn DbCamera_GetMempakAllocSize() -> s32 {
    return sAllocSize;
}
#[no_mangle]
pub unsafe extern "C" fn DbCamera_LoadCallback(mut c: *mut libc::c_char)
 -> s32 {
    let mut i: s32 = 0;
    let mut size: s32 = 0;
    let mut off: s32 = 0;
    i = 0 as libc::c_int;
    while i <
              (::std::mem::size_of::<[DbCameraCut; 16]>() as
                   libc::c_ulong).wrapping_div(::std::mem::size_of::<DbCameraCut>()
                                                   as libc::c_ulong) as s32 -
                  1 as libc::c_int {
        if sDbCameraCuts[i as usize].letter as libc::c_int != '?' as i32 {
            DbCamera_ResetCut(i, 1 as libc::c_int);
            sLetters[i as usize] = 'X' as i32 as libc::c_char
        }
        i += 1
    }
    if Mempak_Read(2 as libc::c_int, *c,
                   sDbCameraCuts.as_mut_ptr() as *mut libc::c_void,
                   0 as libc::c_int,
                   ::std::mem::size_of::<[DbCameraCut; 16]>() as libc::c_ulong
                       as s32) == 0 {
        return 0 as libc::c_int
    }
    off = ::std::mem::size_of::<[DbCameraCut; 16]>() as libc::c_ulong as s32;
    i = 0 as libc::c_int;
    while i <
              (::std::mem::size_of::<[DbCameraCut; 16]>() as
                   libc::c_ulong).wrapping_div(::std::mem::size_of::<DbCameraCut>()
                                                   as libc::c_ulong) as s32 -
                  1 as libc::c_int {
        if sDbCameraCuts[i as usize].letter as libc::c_int != '?' as i32 {
            size =
                (sDbCameraCuts[i as usize].nPoints as
                     libc::c_uint).wrapping_mul(::std::mem::size_of::<CutsceneCameraPoint>()
                                                    as libc::c_ulong) as s32;
            sDbCameraCuts[i as usize].lookAt =
                DebugArena_MallocDebug((size + 0x1f as libc::c_int &
                                            !(0x1f as libc::c_int)) as u32_0,
                                       b"../db_camera.c\x00" as *const u8 as
                                           *const libc::c_char,
                                       2844 as libc::c_int) as
                    *mut CutsceneCameraPoint;
            if sDbCameraCuts[i as usize].lookAt.is_null() {
                // "Debug camera memory allocation failure"
                osSyncPrintf(b"%s: %d: \xe3\x83\x87\xe3\x83\x90\xe3\x83\x83\xe3\x82\xb0\xe3\x82\xab\xe3\x83\xa1\xe3\x83\xa9 \xe3\x83\xa1\xe3\x83\xa2\xe3\x83\xaa\xe7\xa2\xba\xe4\xbf\x9d\xe5\xa4\xb1\xe6\x95\x97\xef\xbc\x81\xef\xbc\x81\n\x00"
                                 as *const u8 as *const libc::c_char,
                             b"../db_camera.c\x00" as *const u8 as
                                 *const libc::c_char, 2847 as libc::c_int);
                return 0 as libc::c_int
            }
            if Mempak_Read(2 as libc::c_int, *c,
                           sDbCameraCuts[i as usize].lookAt as
                               *mut libc::c_void, off,
                           size + 0x1f as libc::c_int &
                               !(0x1f as libc::c_int)) == 0 {
                return 0 as libc::c_int
            }
            off += size + 0x1f as libc::c_int & !(0x1f as libc::c_int);
            sDbCameraCuts[i as usize].position =
                DebugArena_MallocDebug((size + 0x1f as libc::c_int &
                                            !(0x1f as libc::c_int)) as u32_0,
                                       b"../db_camera.c\x00" as *const u8 as
                                           *const libc::c_char,
                                       2855 as libc::c_int) as
                    *mut CutsceneCameraPoint;
            if sDbCameraCuts[i as usize].position.is_null() {
                // "Debug camera memory allocation failure"
                osSyncPrintf(b"%s: %d: \xe3\x83\x87\xe3\x83\x90\xe3\x83\x83\xe3\x82\xb0\xe3\x82\xab\xe3\x83\xa1\xe3\x83\xa9 \xe3\x83\xa1\xe3\x83\xa2\xe3\x83\xaa\xe7\xa2\xba\xe4\xbf\x9d\xe5\xa4\xb1\xe6\x95\x97\xef\xbc\x81\xef\xbc\x81\n\x00"
                                 as *const u8 as *const libc::c_char,
                             b"../db_camera.c\x00" as *const u8 as
                                 *const libc::c_char,
                             2858 as libc::c_int); // DEMO CONTROL
                return 0 as libc::c_int
            } // todo: find something better
            if Mempak_Read(2 as libc::c_int, *c,
                           sDbCameraCuts[i as usize].position as
                               *mut libc::c_void, off,
                           size + 0x1f as libc::c_int &
                               !(0x1f as libc::c_int)) == 0 {
                return 0 as libc::c_int
            } // cursor
            off +=
                size + 0x1f as libc::c_int &
                    !(0x1f as libc::c_int); // NEED      BYTE
            D_80161250[(0x3f as libc::c_int +
                            sDbCameraCuts[i as usize].letter as libc::c_int)
                           as usize] = 'O' as i32 as libc::c_char
        } // FREE      BYTE
        i += 1
    } // current selection
    return 1 as libc::c_int; // useless
}
#[no_mangle]
pub unsafe extern "C" fn DbCamera_SaveCallback(mut c: *mut libc::c_char)
 -> s32 {
    let mut pad: [s32; 2] = [0; 2];
    let mut ret: s32 = 0;
    let mut freeSize: u32_0 = 0;
    let mut off: s32 = 0;
    let mut size: s32 = 0;
    let mut i: s32 = 0;
    ret = Mempak_GetFileSize(2 as libc::c_int, *c);
    freeSize = Mempak_GetFreeBytes(2 as libc::c_int) as u32_0;
    if (sAllocSize as u32_0) < freeSize.wrapping_add(ret as libc::c_uint) {
        if Mempak_Alloc(2 as libc::c_int, c, sAllocSize) == 0 {
            return 0 as libc::c_int
        }
        if Mempak_Write(2 as libc::c_int, *c,
                        sDbCameraCuts.as_mut_ptr() as *mut libc::c_void,
                        0 as libc::c_int,
                        ::std::mem::size_of::<[DbCameraCut; 16]>() as
                            libc::c_ulong as s32) == 0 {
            Mempak_DeleteFile(2 as libc::c_int, *c);
            return 0 as libc::c_int
        }
        off =
            ::std::mem::size_of::<[DbCameraCut; 16]>() as libc::c_ulong as
                s32;
        i = 0 as libc::c_int;
        while i <
                  (::std::mem::size_of::<[DbCameraCut; 16]>() as
                       libc::c_ulong).wrapping_div(::std::mem::size_of::<DbCameraCut>()
                                                       as libc::c_ulong) as
                      s32 - 1 as libc::c_int {
            if sDbCameraCuts[i as usize].letter as libc::c_int != '?' as i32 {
                size =
                    (sDbCameraCuts[i as usize].nPoints as
                         libc::c_uint).wrapping_mul(::std::mem::size_of::<CutsceneCameraPoint>()
                                                        as libc::c_ulong) as
                        s32;
                ret =
                    Mempak_Write(2 as libc::c_int, *c,
                                 sDbCameraCuts[i as usize].lookAt as
                                     *mut libc::c_void, off,
                                 size + 0x1f as libc::c_int &
                                     !(0x1f as libc::c_int));
                if ret == 0 { break ; }
                off += size + 0x1f as libc::c_int & !(0x1f as libc::c_int);
                ret =
                    Mempak_Write(2 as libc::c_int, *c,
                                 sDbCameraCuts[i as usize].position as
                                     *mut libc::c_void, off,
                                 size + 0x1f as libc::c_int &
                                     !(0x1f as libc::c_int));
                if ret == 0 { break ; }
                off += size + 0x1f as libc::c_int & !(0x1f as libc::c_int)
            }
            ret = 1 as libc::c_int;
            i += 1
        }
        if ret != 0 {
            return *c as s32
        } else {
            Mempak_DeleteFile(2 as libc::c_int, *c);
            return 0 as libc::c_int
        }
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn DbCamera_ClearCallback(mut c: *mut libc::c_char)
 -> s32 {
    return Mempak_DeleteFile(2 as libc::c_int, *c);
}
#[no_mangle]
pub unsafe extern "C" fn DbCamera_DrawSlotLetters(mut str: *mut libc::c_char,
                                                  mut y: s16, mut x: s16,
                                                  mut colorId: s32) {
    let mut i: s32 = 0;
    i = 0 as libc::c_int;
    while i <
              (::std::mem::size_of::<[DbCameraCut; 16]>() as
                   libc::c_ulong).wrapping_div(::std::mem::size_of::<DbCameraCut>()
                                                   as libc::c_ulong) as s32 -
                  1 as libc::c_int {
        *str.offset((i * 2 as libc::c_int + 1 as libc::c_int) as isize) =
            sDbCameraCuts[i as usize].letter;
        *str.offset((i * 2 as libc::c_int + 0 as libc::c_int) as isize) =
            '-' as i32 as libc::c_char;
        i += 1
    }
    let ref mut fresh5 =
        *str.offset((i * 2 as libc::c_int + 1 as libc::c_int) as isize);
    *fresh5 = '\u{0}' as i32 as libc::c_char;
    *str.offset(0x14 as libc::c_int as isize) = *fresh5;
    func_8006376C(x as u8_0, y as u8_0, colorId as u8_0, str);
    let ref mut fresh6 =
        *str.offset((i * 2 as libc::c_int + 0 as libc::c_int) as isize);
    *fresh6 = '-' as i32 as libc::c_char;
    *str.offset(0x14 as libc::c_int as isize) = *fresh6;
    func_8006376C((x as libc::c_int + 0x14 as libc::c_int) as u8_0, y as u8_0,
                  colorId as u8_0, str.offset(0x14 as libc::c_int as isize));
}
#[no_mangle]
pub unsafe extern "C" fn DbCamera_PrintAllCuts(mut cam: *mut Camera) {
    let mut i: s32 = 0;
    Audio_PlaySoundGeneral(0x4803 as libc::c_int as u16_0, &mut D_801333D4,
                           4 as libc::c_int as u8_0, &mut D_801333E0,
                           &mut D_801333E0, &mut D_801333E8);
    osSyncPrintf(b"@@@\n@@@\n@@@/* ****** spline point data ** start here ***** */\n@@@\n\x00"
                     as *const u8 as *const libc::c_char);
    i = 0 as libc::c_int;
    while i <
              (::std::mem::size_of::<[DbCameraCut; 16]>() as
                   libc::c_ulong).wrapping_div(::std::mem::size_of::<DbCameraCut>()
                                                   as libc::c_ulong) as s32 -
                  1 as libc::c_int {
        let mut cut: *mut DbCameraCut =
            &mut *sDbCameraCuts.as_mut_ptr().offset(i as isize) as
                *mut DbCameraCut;
        if (*cut).nPoints as libc::c_int != 0 as libc::c_int {
            if i != 0 as libc::c_int {
                osSyncPrintf(b"@@@\n@@@/* ** %d ** */\n@@@\n\x00" as *const u8
                                 as *const libc::c_char, i);
            }
            DbCamera_PrintPoints(b"Lookat\x00" as *const u8 as
                                     *const libc::c_char, (*cut).nPoints,
                                 (*cut).lookAt);
            DbCamera_PrintPoints(b"Position\x00" as *const u8 as
                                     *const libc::c_char, (*cut).nPoints,
                                 (*cut).position);
            osSyncPrintf(b"@@@static short  nPoints = %d;\n@@@\n\x00" as
                             *const u8 as *const libc::c_char,
                         (*cut).nPoints as libc::c_int);
            osSyncPrintf(b"@@@static short  nFrames = %d;\n@@@\n\x00" as
                             *const u8 as *const libc::c_char,
                         (*cut).nFrames as libc::c_int);
            osSyncPrintf(b"@@@static short  Mode = %d;\n@@@\n\x00" as
                             *const u8 as *const libc::c_char,
                         (*cut).mode as libc::c_int);
        }
        i += 1
    }
    osSyncPrintf(b"@@@\n@@@\n@@@/* ****** spline point data ** finish! ***** */\n@@@\n\x00"
                     as *const u8 as *const libc::c_char);
}
#[no_mangle]
pub static mut D_8012D114: [libc::c_char; 21] =
    unsafe {
        *::std::mem::transmute::<&[u8; 21],
                                 &mut [libc::c_char; 21]>(b"\x8c\xef\xbe\x8c\xef\xbe\x9a-\xef\xbe\x91         \x00")
    };
#[no_mangle]
pub static mut D_8012D128: [libc::c_char; 21] =
    unsafe {
        *::std::mem::transmute::<&[u8; 21],
                                 &mut [libc::c_char; 21]>(b"\x8c\xef\xbe\x84-\xef\xbe\x80\xef\xbe\x99         \x00")
    };
#[no_mangle]
pub static mut D_8012D13C: [libc::c_char; 15] =
    unsafe {
        *::std::mem::transmute::<&[u8; 15],
                                 &mut [libc::c_char; 15]>(b"\x8c\xef\xbd\xb7-     /   \x00")
    };
#[no_mangle]
pub unsafe extern "C" fn func_800B91B0(mut cam: *mut Camera,
                                       mut dbCamera: *mut DbCamera) -> s32 {
    let mut pointCount: s32 = 0;
    let mut curPoint: s32 = 0;
    while sDbCameraCuts[D_8016110C as usize].letter as libc::c_int ==
              '?' as i32 {
        D_8016110C += 1;
        if D_8016110C as libc::c_int >=
               (::std::mem::size_of::<[DbCameraCut; 16]>() as
                    libc::c_ulong).wrapping_div(::std::mem::size_of::<DbCameraCut>()
                                                    as libc::c_ulong) as s32 -
                   1 as libc::c_int {
            sDbCamAnim.curFrame = 0.0f32;
            sDbCamAnim.unk_04 = 0 as libc::c_int as f32_0;
            sDbCamAnim.keyframe = 0 as libc::c_int as s16;
            sDbCamAnim.unk_0A = 0 as libc::c_int as s16;
            D_8016110C = 0 as libc::c_int as s16;
            return 0 as libc::c_int
        }
    }
    if func_800BB2B4(&mut sDbCamAnim.positionPos, &mut sDbCamAnim.roll,
                     &mut sDbCamAnim.fov,
                     sDbCameraCuts[D_8016110C as usize].position,
                     &mut sDbCamAnim.keyframe, &mut sDbCamAnim.curFrame) == 0
           &&
           func_800BB2B4(&mut sDbCamAnim.lookAtPos, &mut sDbCamAnim.roll,
                         &mut sDbCamAnim.fov,
                         sDbCameraCuts[D_8016110C as usize].lookAt,
                         &mut sDbCamAnim.keyframe, &mut sDbCamAnim.curFrame)
               == 0 {
        D_8012D13C[7 as libc::c_int as usize] =
            ((sDbCamAnim.keyframe as libc::c_int + 1 as libc::c_int) /
                 10 as libc::c_int + '0' as i32) as libc::c_char;
        D_8012D13C[8 as libc::c_int as usize] =
            ((sDbCamAnim.keyframe as libc::c_int + 1 as libc::c_int) %
                 10 as libc::c_int + '0' as i32) as libc::c_char;
        D_8012D13C[10 as libc::c_int as usize] =
            ((sDbCameraCuts[D_8016110C as usize].nPoints as libc::c_int -
                  5 as libc::c_int) / 10 as libc::c_int + '0' as i32) as
                libc::c_char;
        D_8012D13C[11 as libc::c_int as usize] =
            ((sDbCameraCuts[D_8016110C as usize].nPoints as libc::c_int -
                  5 as libc::c_int) % 10 as libc::c_int + '0' as i32) as
                libc::c_char;
        DbCamera_SetTextValue(sDbCamAnim.unk_04 as s16,
                              &mut *D_8012D114.as_mut_ptr().offset(10 as
                                                                       libc::c_int
                                                                       as
                                                                       isize),
                              4 as libc::c_int as u8_0);
        func_8006376C(0xf as libc::c_int as u8_0, 0x16 as libc::c_int as u8_0,
                      3 as libc::c_int as u8_0, D_8012D114.as_mut_ptr());
        DbCamera_SetTextValue(sDbCamAnim.unk_0C,
                              &mut *D_8012D128.as_mut_ptr().offset(10 as
                                                                       libc::c_int
                                                                       as
                                                                       isize),
                              4 as libc::c_int as u8_0);
        func_8006376C(0xf as libc::c_int as u8_0, 0x17 as libc::c_int as u8_0,
                      3 as libc::c_int as u8_0, D_8012D128.as_mut_ptr());
        func_8006376C(0xf as libc::c_int as u8_0, 0x18 as libc::c_int as u8_0,
                      3 as libc::c_int as u8_0, D_8012D13C.as_mut_ptr());
        func_8006376C(0x10 as libc::c_int as u8_0,
                      0x1a as libc::c_int as u8_0, 1 as libc::c_int as u8_0,
                      D_8012CEF0);
        sDbCamAnim.unk_04 += 1.;
        sDbCamAnim.unk_0C += 1;
        if sDbCameraCuts[D_8016110C as usize].nFrames as libc::c_int >
               0 as libc::c_int &&
               (sDbCameraCuts[D_8016110C as usize].nFrames as libc::c_int as
                    libc::c_float) < sDbCamAnim.unk_04 {
            D_8016110C += 1;
            sDbCamAnim.curFrame = 0.0f32;
            sDbCamAnim.unk_04 = 0 as libc::c_int as f32_0;
            sDbCamAnim.keyframe = 0 as libc::c_int as s16;
            return D_8016110C as libc::c_int | 0x8000 as libc::c_int
        }
        if sDbCameraCuts[D_8016110C as usize].mode as libc::c_int !=
               1 as libc::c_int {
            DbCamera_CopyVec3f(&mut sDbCamAnim.positionPos,
                               &mut (*dbCamera).eye);
            DbCamera_CopyVec3f(&mut sDbCamAnim.lookAtPos,
                               &mut (*dbCamera).at);
        } else {
            func_800B3FF4(&mut (*cam).playerPosRot, &mut sDbCamAnim.lookAtPos,
                          &mut (*dbCamera).at);
            func_800B3FF4(&mut (*cam).playerPosRot,
                          &mut sDbCamAnim.positionPos, &mut (*dbCamera).eye);
        }
        (*dbCamera).fov = sDbCamAnim.fov;
        (*dbCamera).roll = sDbCamAnim.roll as s16;
        (*dbCamera).rollDegrees = sDbCamAnim.roll * (360.0f32 / 256.0f32)
    } else {
        D_8016110C += 1;
        sDbCamAnim.keyframe = 0 as libc::c_int as s16;
        sDbCamAnim.curFrame = 0.0f32;
        sDbCamAnim.unk_04 = 0 as libc::c_int as f32_0;
        if D_8016110C as libc::c_int ==
               (::std::mem::size_of::<[DbCameraCut; 16]>() as
                    libc::c_ulong).wrapping_div(::std::mem::size_of::<DbCameraCut>()
                                                    as libc::c_ulong) as s32 -
                   1 as libc::c_int {
            D_8016110C = 0 as libc::c_int as s16;
            sDbCamAnim.curFrame = 0.0f32;
            sDbCamAnim.keyframe = 0 as libc::c_int as s16;
            sDbCamAnim.unk_0A = 0 as libc::c_int as s16;
            return -(1 as libc::c_int)
        }
    }
    return D_8016110C as libc::c_int | 0x8000 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn DbCamera_Reset(mut cam: *mut Camera,
                                        mut dbCam: *mut DbCamera) {
    let mut i: s32 = 0;
    D_801612EA = '*' as i32 as libc::c_char;
    i = 0 as libc::c_int;
    while i <
              (::std::mem::size_of::<[libc::c_char; 26]>() as
                   libc::c_ulong).wrapping_div(::std::mem::size_of::<libc::c_char>()
                                                   as libc::c_ulong) as s32 {
        sLetters[i as usize] = 'X' as i32 as libc::c_char;
        i += 1
    }
    i = 0 as libc::c_int;
    while i < 0xf as libc::c_int {
        DbCamera_ResetCut(i, 0 as libc::c_int);
        i += 1
    }
    sDbCamPtr = dbCam;
    D_8016110C = 0 as libc::c_int as s16;
    sCurFileIdx = 0 as libc::c_int as s16;
    sLastFileIdx = -(1 as libc::c_int) as s16;
    sDbCamAnim.unk_0A = 0 as libc::c_int as s16;
}
#[no_mangle]
pub unsafe extern "C" fn DbCamera_UpdateDemoControl(mut dbCamera:
                                                        *mut DbCamera,
                                                    mut cam: *mut Camera)
 -> s32 {
    let mut current_block: u64;
    static mut sMempakFiles: s32 = 0;
    static mut sDbCameraColors: [u32_0; 6] =
        [4 as libc::c_int as u32_0, 4 as libc::c_int as u32_0,
         4 as libc::c_int as u32_0, 7 as libc::c_int as u32_0,
         4 as libc::c_int as u32_0, 4 as libc::c_int as u32_0];
    static mut sMempakFilesize: s32 = 0 as libc::c_int;
    let mut i: s32 = 0;
    let mut idx1: s32 = 0;
    let mut idx2: s32 = 0;
    let mut idx3: s16 = 0;
    let mut sp74: [libc::c_char; 38] = [0; 38];
    let mut sp64: DbCameraCut =
        DbCameraCut{letter: 0,
                    unk_01: 0,
                    mode: 0,
                    position:
                        0 as *const CutsceneCameraPoint as
                            *mut CutsceneCameraPoint,
                    lookAt:
                        0 as *const CutsceneCameraPoint as
                            *mut CutsceneCameraPoint,
                    nFrames: 0,
                    nPoints: 0,};
    let mut sp5C: VecSph = VecSph{r: 0., pitch: 0, yaw: 0,};
    let mut callbacks:
            [Option<unsafe extern "C" fn(_: *mut libc::c_char) -> s32>; 3] =
        [Some(DbCamera_SaveCallback as
                  unsafe extern "C" fn(_: *mut libc::c_char) -> s32),
         Some(DbCamera_LoadCallback as
                  unsafe extern "C" fn(_: *mut libc::c_char) -> s32),
         Some(DbCamera_ClearCallback as
                  unsafe extern "C" fn(_: *mut libc::c_char) -> s32)];
    func_8006376C(0xe as libc::c_int as u8_0, 5 as libc::c_int as u8_0,
                  0 as libc::c_int as u8_0, D_8012CF44);
    idx1 = sCurFileIdx as libc::c_int >> 1 as libc::c_int;
    idx2 = sLastFileIdx as libc::c_int >> 1 as libc::c_int;
    match (*dbCamera).sub.demoCtrlActionIdx as libc::c_int {
        1 | 2 | 3 => {
            match (*dbCamera).sub.demoCtrlMenu as libc::c_int {
                100 | 200 | 300 => {
                    if (1 as libc::c_int) << sCurFileIdx as libc::c_int &
                           sMempakFiles != 0 {
                        if !((*sGlobalCtx).state.input[2 as libc::c_int as
                                                           usize].press.button
                                 as libc::c_int | !(0x200 as libc::c_int)) ==
                               0 as libc::c_int ||
                               !((*sGlobalCtx).state.input[2 as libc::c_int as
                                                               usize].press.button
                                     as libc::c_int | !(0x100 as libc::c_int))
                                   == 0 as libc::c_int {
                            Audio_PlaySoundGeneral(0x4809 as libc::c_int as
                                                       u16_0, &mut D_801333D4,
                                                   4 as libc::c_int as u8_0,
                                                   &mut D_801333E0,
                                                   &mut D_801333E0,
                                                   &mut D_801333E8);
                            (*dbCamera).sub.demoCtrlToggleSwitch =
                                ((*dbCamera).sub.demoCtrlToggleSwitch as
                                     libc::c_int ^ 1 as libc::c_int) as s16
                        }
                        *D_8012CEE0[41 as libc::c_int as
                                        usize].offset(9 as libc::c_int as
                                                          isize) =
                            (sCurFileIdx as libc::c_int + 'A' as i32) as
                                libc::c_char;
                        func_8006376C(0xa as libc::c_int as u8_0,
                                      7 as libc::c_int as u8_0,
                                      5 as libc::c_int as u8_0,
                                      D_8012CEE0[41 as libc::c_int as usize]);
                        func_8006376C(0x10 as libc::c_int as u8_0,
                                      7 as libc::c_int as u8_0,
                                      5 as libc::c_int as u8_0,
                                      D_8012CF60[(*dbCamera).sub.demoCtrlActionIdx
                                                     as usize]);
                        func_8006376C(0x14 as libc::c_int as u8_0,
                                      7 as libc::c_int as u8_0,
                                      5 as libc::c_int as u8_0,
                                      D_8012CF88[0 as libc::c_int as usize]);
                        func_8006376C(0x11 as libc::c_int as u8_0,
                                      8 as libc::c_int as u8_0,
                                      if (*dbCamera).sub.demoCtrlToggleSwitch
                                             as libc::c_int != 0 {
                                          4 as libc::c_int
                                      } else { 7 as libc::c_int } as u8_0,
                                      D_8012CF94);
                        func_8006376C(0x15 as libc::c_int as u8_0,
                                      8 as libc::c_int as u8_0,
                                      if (*dbCamera).sub.demoCtrlToggleSwitch
                                             as libc::c_int != 0 {
                                          7 as libc::c_int
                                      } else { 4 as libc::c_int } as u8_0,
                                      D_8012CF98);
                        if !((*sGlobalCtx).state.input[2 as libc::c_int as
                                                           usize].press.button
                                 as libc::c_int | !(0x8000 as libc::c_int)) ==
                               0 as libc::c_int {
                            if (*dbCamera).sub.demoCtrlToggleSwitch as
                                   libc::c_int == 0 as libc::c_int {
                                Audio_PlaySoundGeneral(0x4808 as libc::c_int
                                                           as u16_0,
                                                       &mut D_801333D4,
                                                       4 as libc::c_int as
                                                           u8_0,
                                                       &mut D_801333E0,
                                                       &mut D_801333E0,
                                                       &mut D_801333E8);
                                (*dbCamera).sub.demoCtrlMenu += 1
                            } else {
                                Audio_PlaySoundGeneral(0x480a as libc::c_int
                                                           as u16_0,
                                                       &mut D_801333D4,
                                                       4 as libc::c_int as
                                                           u8_0,
                                                       &mut D_801333E0,
                                                       &mut D_801333E0,
                                                       &mut D_801333E8);
                                (*dbCamera).sub.demoCtrlMenu =
                                    0 as libc::c_int as s16
                            }
                        }
                    } else if (*dbCamera).sub.demoCtrlMenu as libc::c_int ==
                                  100 as libc::c_int {
                        (*dbCamera).sub.demoCtrlMenu += 1
                    } else {
                        (*dbCamera).sub.demoCtrlToggleSwitch =
                            ((*dbCamera).sub.demoCtrlToggleSwitch as
                                 libc::c_int ^ 1 as libc::c_int) as s16;
                        *D_8012CF84.offset(9 as libc::c_int as isize) =
                            (sCurFileIdx as libc::c_int + 'A' as i32) as
                                libc::c_char;
                        func_8006376C(0xd as libc::c_int as u8_0,
                                      7 as libc::c_int as u8_0,
                                      5 as libc::c_int as u8_0,
                                      D_8012CF88[-(1 as libc::c_int) as
                                                     usize]);
                        func_8006376C(0x12 as libc::c_int as u8_0,
                                      7 as libc::c_int as u8_0,
                                      5 as libc::c_int as u8_0, D_8012CF80);
                        func_8006376C(0xd as libc::c_int as u8_0,
                                      9 as libc::c_int as u8_0,
                                      if (*dbCamera).sub.demoCtrlToggleSwitch
                                             as libc::c_int != 0 {
                                          1 as libc::c_int
                                      } else { 6 as libc::c_int } as u8_0,
                                      b"PRESS B BUTTON\x00" as *const u8 as
                                          *const libc::c_char);
                    }
                    if !((*sGlobalCtx).state.input[2 as libc::c_int as
                                                       usize].press.button as
                             libc::c_int | !(0x4000 as libc::c_int)) ==
                           0 as libc::c_int {
                        Audio_PlaySoundGeneral(0x480a as libc::c_int as u16_0,
                                               &mut D_801333D4,
                                               4 as libc::c_int as u8_0,
                                               &mut D_801333E0,
                                               &mut D_801333E0,
                                               &mut D_801333E8);
                        (*dbCamera).sub.demoCtrlMenu =
                            0 as libc::c_int as s16;
                        return 1 as libc::c_int
                    }
                    current_block = 8433100169055355706;
                }
                101 | 201 | 301 => {
                    *D_8012CEE0[41 as libc::c_int as
                                    usize].offset(9 as libc::c_int as isize) =
                        (sCurFileIdx as libc::c_int + 'A' as i32) as
                            libc::c_char;
                    func_8006376C(0xc as libc::c_int as u8_0,
                                  7 as libc::c_int as u8_0,
                                  5 as libc::c_int as u8_0,
                                  D_8012CEE0[41 as libc::c_int as usize]);
                    func_8006376C(0x12 as libc::c_int as u8_0,
                                  7 as libc::c_int as u8_0,
                                  5 as libc::c_int as u8_0,
                                  D_8012CF60[(*dbCamera).sub.demoCtrlActionIdx
                                                 as usize]);
                    func_8006376C(0x16 as libc::c_int as u8_0,
                                  7 as libc::c_int as u8_0,
                                  5 as libc::c_int as u8_0,
                                  D_8012CF9C[0 as libc::c_int as usize]);
                    if callbacks[((*dbCamera).sub.demoCtrlActionIdx as
                                      libc::c_int - 1 as libc::c_int) as
                                     usize].expect("non-null function pointer")(&mut *D_8012CF84.offset(9
                                                                                                            as
                                                                                                            libc::c_int
                                                                                                            as
                                                                                                            isize))
                           != 0 {
                        (*dbCamera).sub.demoCtrlMenu += 1;
                        return 1 as libc::c_int
                    } else {
                        (*dbCamera).sub.demoCtrlMenu =
                            ((*dbCamera).sub.demoCtrlMenu as libc::c_int +
                                 8 as libc::c_int) as s16;
                        return 1 as libc::c_int
                    }
                }
                102 | 202 | 302 => {
                    (*dbCamera).sub.demoCtrlToggleSwitch =
                        ((*dbCamera).sub.demoCtrlToggleSwitch as libc::c_int ^
                             1 as libc::c_int) as s16;
                    *D_8012CEE0[41 as libc::c_int as
                                    usize].offset(9 as libc::c_int as isize) =
                        (sCurFileIdx as libc::c_int + 'A' as i32) as
                            libc::c_char;
                    func_8006376C(0xd as libc::c_int as u8_0,
                                  7 as libc::c_int as u8_0,
                                  5 as libc::c_int as u8_0,
                                  D_8012CEE0[41 as libc::c_int as usize]);
                    func_8006376C(0x13 as libc::c_int as u8_0,
                                  7 as libc::c_int as u8_0,
                                  5 as libc::c_int as u8_0,
                                  D_8012CF60[((*dbCamera).sub.demoCtrlMenu as
                                                  libc::c_int /
                                                  100 as libc::c_int) as
                                                 usize]);
                    func_8006376C(0x17 as libc::c_int as u8_0,
                                  7 as libc::c_int as u8_0,
                                  5 as libc::c_int as u8_0, D_8012CFA4);
                    func_8006376C(0xd as libc::c_int as u8_0,
                                  9 as libc::c_int as u8_0,
                                  if (*dbCamera).sub.demoCtrlToggleSwitch as
                                         libc::c_int != 0 as libc::c_int {
                                      1 as libc::c_int
                                  } else { 6 as libc::c_int } as u8_0,
                                  b"PRESS B BUTTON\x00" as *const u8 as
                                      *const libc::c_char);
                    if !((*sGlobalCtx).state.input[2 as libc::c_int as
                                                       usize].press.button as
                             libc::c_int | !(0x8000 as libc::c_int)) ==
                           0 as libc::c_int ||
                           !((*sGlobalCtx).state.input[2 as libc::c_int as
                                                           usize].press.button
                                 as libc::c_int | !(0x4000 as libc::c_int)) ==
                               0 as libc::c_int {
                        Audio_PlaySoundGeneral(0x4808 as libc::c_int as u16_0,
                                               &mut D_801333D4,
                                               4 as libc::c_int as u8_0,
                                               &mut D_801333E0,
                                               &mut D_801333E0,
                                               &mut D_801333E8);
                        if (*dbCamera).sub.demoCtrlMenu as libc::c_int ==
                               2 as libc::c_int * 100 as libc::c_int +
                                   2 as libc::c_int {
                            (*dbCamera).sub.demoCtrlActionIdx =
                                0 as libc::c_int as s16
                        }
                        (*dbCamera).sub.demoCtrlMenu =
                            (0 as libc::c_int * 100 as libc::c_int +
                                 0 as libc::c_int) as s16;
                        return 1 as libc::c_int
                    }
                    current_block = 8433100169055355706;
                }
                109 | 209 | 309 => {
                    (*dbCamera).sub.demoCtrlToggleSwitch =
                        ((*dbCamera).sub.demoCtrlToggleSwitch as libc::c_int ^
                             1 as libc::c_int) as s16;
                    *D_8012CEE0[41 as libc::c_int as
                                    usize].offset(9 as libc::c_int as isize) =
                        (sCurFileIdx as libc::c_int + 'A' as i32) as
                            libc::c_char;
                    func_8006376C(0xd as libc::c_int as u8_0,
                                  7 as libc::c_int as u8_0,
                                  5 as libc::c_int as u8_0,
                                  D_8012CEE0[((*dbCamera).sub.demoCtrlMenu as
                                                  libc::c_int /
                                                  100 as libc::c_int +
                                                  32 as libc::c_int) as
                                                 usize]);
                    func_8006376C(0x11 as libc::c_int as u8_0,
                                  7 as libc::c_int as u8_0,
                                  5 as libc::c_int as u8_0, D_8012CFAC);
                    func_8006376C(0x17 as libc::c_int as u8_0,
                                  7 as libc::c_int as u8_0,
                                  5 as libc::c_int as u8_0, D_8012CFA4);
                    func_8006376C(0xd as libc::c_int as u8_0,
                                  9 as libc::c_int as u8_0,
                                  if (*dbCamera).sub.demoCtrlToggleSwitch as
                                         libc::c_int != 0 as libc::c_int {
                                      1 as libc::c_int
                                  } else { 6 as libc::c_int } as u8_0,
                                  b"PRESS B BUTTON\x00" as *const u8 as
                                      *const libc::c_char);
                    if !((*sGlobalCtx).state.input[2 as libc::c_int as
                                                       usize].press.button as
                             libc::c_int | !(0x8000 as libc::c_int)) ==
                           0 as libc::c_int ||
                           !((*sGlobalCtx).state.input[2 as libc::c_int as
                                                           usize].press.button
                                 as libc::c_int | !(0x4000 as libc::c_int)) ==
                               0 as libc::c_int {
                        Audio_PlaySoundGeneral(0x4808 as libc::c_int as u16_0,
                                               &mut D_801333D4,
                                               4 as libc::c_int as u8_0,
                                               &mut D_801333E0,
                                               &mut D_801333E0,
                                               &mut D_801333E8);
                        (*dbCamera).sub.demoCtrlMenu =
                            ((*dbCamera).sub.demoCtrlMenu as libc::c_int -
                                 9 as libc::c_int) as s16
                    }
                    current_block = 8433100169055355706;
                }
                1 => { current_block = 1886879122319939353; }
                _ => {
                    if Mempak_Init(2 as libc::c_int) != 0 {
                        sMempakFiles =
                            Mempak_FindFile(2 as libc::c_int,
                                            'A' as i32 as libc::c_char,
                                            'E' as i32 as libc::c_char);
                        (*dbCamera).sub.demoCtrlMenu =
                            (0 as libc::c_int * 100 as libc::c_int +
                                 1 as libc::c_int) as s16;
                        DbCamera_CalcMempakAllocSize();
                        if (1 as libc::c_int) << sCurFileIdx as libc::c_int &
                               sMempakFiles != 0 {
                            sMempakFilesize =
                                Mempak_GetFileSize(2 as libc::c_int,
                                                   (sCurFileIdx as libc::c_int
                                                        + 'A' as i32) as
                                                       libc::c_char);
                            (*dbCamera).sub.demoCtrlActionIdx =
                                2 as libc::c_int as s16
                        } else {
                            sMempakFilesize = 0 as libc::c_int;
                            (*dbCamera).sub.demoCtrlActionIdx =
                                1 as libc::c_int as s16
                        }
                    } else {
                        func_8006376C(0xc as libc::c_int as u8_0,
                                      0x1a as libc::c_int as u8_0,
                                      4 as libc::c_int as u8_0,
                                      D_8012CF60[0 as libc::c_int as usize]);
                        func_8006376C(0x13 as libc::c_int as u8_0,
                                      0x1a as libc::c_int as u8_0,
                                      4 as libc::c_int as u8_0, D_8012CF80);
                        if !((*sGlobalCtx).state.input[2 as libc::c_int as
                                                           usize].press.button
                                 as libc::c_int | !(0x4000 as libc::c_int)) ==
                               0 as libc::c_int ||
                               !((*sGlobalCtx).state.input[2 as libc::c_int as
                                                               usize].press.button
                                     as libc::c_int | !(0x800 as libc::c_int))
                                   == 0 as libc::c_int ||
                               !((*sGlobalCtx).state.input[2 as libc::c_int as
                                                               usize].press.button
                                     as libc::c_int | !(0x400 as libc::c_int))
                                   == 0 as libc::c_int {
                            Audio_PlaySoundGeneral(0x480a as libc::c_int as
                                                       u16_0, &mut D_801333D4,
                                                   4 as libc::c_int as u8_0,
                                                   &mut D_801333E0,
                                                   &mut D_801333E0,
                                                   &mut D_801333E8);
                            (*dbCamera).sub.demoCtrlActionIdx =
                                0 as libc::c_int as s16
                        }
                        return 2 as libc::c_int
                    }
                    current_block = 1886879122319939353;
                }
            }
            match current_block {
                1886879122319939353 => {
                    idx2 = 1 as libc::c_int;
                    i = 0 as libc::c_int;
                    while i < 5 as libc::c_int {
                        sp74[(i * 2 as libc::c_int + 1 as libc::c_int) as
                                 usize] =
                            if sMempakFiles & idx2 != 0 {
                                (i) + 'A' as i32
                            } else { '?' as i32 } as libc::c_char;
                        sp74[(i * 2 as libc::c_int + 0 as libc::c_int) as
                                 usize] = '-' as i32 as libc::c_char;
                        idx2 <<= 1 as libc::c_int;
                        i += 1
                    }
                    sp74[(i * 2 as libc::c_int + 0 as libc::c_int) as usize] =
                        '-' as i32 as libc::c_char;
                    sp74[(i * 2 as libc::c_int + 1 as libc::c_int) as usize] =
                        '\u{0}' as i32 as libc::c_char;
                    if !((*sGlobalCtx).state.input[2 as libc::c_int as
                                                       usize].press.button as
                             libc::c_int | !(0x100 as libc::c_int)) ==
                           0 as libc::c_int {
                        Audio_PlaySoundGeneral(0x4809 as libc::c_int as u16_0,
                                               &mut D_801333D4,
                                               4 as libc::c_int as u8_0,
                                               &mut D_801333E0,
                                               &mut D_801333E0,
                                               &mut D_801333E8);
                        if sCurFileIdx as libc::c_int >= 4 as libc::c_int {
                            sCurFileIdx = 0 as libc::c_int as s16
                        } else { sCurFileIdx += 1 }
                        if (1 as libc::c_int) << sCurFileIdx as libc::c_int &
                               sMempakFiles != 0 {
                            sMempakFilesize =
                                Mempak_GetFileSize(2 as libc::c_int,
                                                   (sCurFileIdx as libc::c_int
                                                        + 'A' as i32) as
                                                       libc::c_char);
                            (*dbCamera).sub.demoCtrlActionIdx =
                                2 as libc::c_int as s16
                        } else {
                            sMempakFilesize = 0 as libc::c_int;
                            (*dbCamera).sub.demoCtrlActionIdx =
                                1 as libc::c_int as s16
                        }
                    }
                    if !((*sGlobalCtx).state.input[2 as libc::c_int as
                                                       usize].press.button as
                             libc::c_int | !(0x200 as libc::c_int)) ==
                           0 as libc::c_int {
                        Audio_PlaySoundGeneral(0x4809 as libc::c_int as u16_0,
                                               &mut D_801333D4,
                                               4 as libc::c_int as u8_0,
                                               &mut D_801333E0,
                                               &mut D_801333E0,
                                               &mut D_801333E8);
                        if sCurFileIdx as libc::c_int <= 0 as libc::c_int {
                            sCurFileIdx = 4 as libc::c_int as s16
                        } else { sCurFileIdx -= 1 }
                        if (1 as libc::c_int) << sCurFileIdx as libc::c_int &
                               sMempakFiles != 0 {
                            sMempakFilesize =
                                Mempak_GetFileSize(2 as libc::c_int,
                                                   (sCurFileIdx as libc::c_int
                                                        + 'A' as i32) as
                                                       libc::c_char);
                            (*dbCamera).sub.demoCtrlActionIdx =
                                2 as libc::c_int as s16
                        } else {
                            sMempakFilesize = 0 as libc::c_int;
                            (*dbCamera).sub.demoCtrlActionIdx =
                                1 as libc::c_int as s16
                        }
                    }
                    idx3 = (*dbCamera).sub.demoCtrlActionIdx;
                    func_8006376C(0xe as libc::c_int as u8_0,
                                  7 as libc::c_int as u8_0,
                                  5 as libc::c_int as u8_0,
                                  D_8012CF50[idx3 as usize]);
                    func_8006376C(0xf as libc::c_int as u8_0,
                                  7 as libc::c_int as u8_0,
                                  4 as libc::c_int as u8_0,
                                  sp74.as_mut_ptr());
                    func_8006376C((sCurFileIdx as libc::c_int *
                                       2 as libc::c_int + 0x10 as libc::c_int)
                                      as u8_0, 7 as libc::c_int as u8_0,
                                  7 as libc::c_int as u8_0,
                                  b"_\x00" as *const u8 as
                                      *const libc::c_char);
                    DbCamera_SetTextValue(DbCamera_GetMempakAllocSize() as
                                              s16, sp74.as_mut_ptr(),
                                          6 as libc::c_int as u8_0);
                    func_8006376C(0xd as libc::c_int as u8_0,
                                  9 as libc::c_int as u8_0,
                                  6 as libc::c_int as u8_0, D_8012CF78);
                    func_8006376C(0x11 as libc::c_int as u8_0,
                                  9 as libc::c_int as u8_0,
                                  4 as libc::c_int as u8_0,
                                  sp74.as_mut_ptr());
                    DbCamera_SetTextValue(Mempak_GetFreeBytes(2 as
                                                                  libc::c_int)
                                              as s16, sp74.as_mut_ptr(),
                                          6 as libc::c_int as u8_0);
                    func_8006376C(0xd as libc::c_int as u8_0,
                                  0xa as libc::c_int as u8_0,
                                  6 as libc::c_int as u8_0, D_8012CF74);
                    func_8006376C(0x11 as libc::c_int as u8_0,
                                  0xa as libc::c_int as u8_0,
                                  4 as libc::c_int as u8_0,
                                  sp74.as_mut_ptr());
                    if sMempakFilesize != 0 as libc::c_int {
                        DbCamera_SetTextValue(sMempakFilesize as s16,
                                              sp74.as_mut_ptr(),
                                              6 as libc::c_int as u8_0);
                        func_8006376C(0xd as libc::c_int as u8_0,
                                      0xb as libc::c_int as u8_0,
                                      7 as libc::c_int as u8_0, D_8012CFA8);
                        func_8006376C(0x11 as libc::c_int as u8_0,
                                      0xb as libc::c_int as u8_0,
                                      4 as libc::c_int as u8_0,
                                      sp74.as_mut_ptr());
                    }
                    idx1 =
                        (*dbCamera).sub.demoCtrlActionIdx as libc::c_int +
                            2 as libc::c_int;
                    func_8006376C(0xf as libc::c_int as u8_0,
                                  0x16 as libc::c_int as u8_0,
                                  1 as libc::c_int as u8_0, D_8012CF7C);
                    func_8006376C(0x12 as libc::c_int as u8_0,
                                  0x17 as libc::c_int as u8_0,
                                  sDbCameraColors[idx1 as usize] as u8_0,
                                  D_8012CF64);
                    func_8006376C(0x12 as libc::c_int as u8_0,
                                  0x18 as libc::c_int as u8_0,
                                  sDbCameraColors[(idx1 - 1 as libc::c_int) as
                                                      usize] as u8_0,
                                  D_8012CF68);
                    func_8006376C(0x12 as libc::c_int as u8_0,
                                  0x19 as libc::c_int as u8_0,
                                  sDbCameraColors[(idx1 - 2 as libc::c_int) as
                                                      usize] as u8_0,
                                  D_8012CF6C);
                    func_8006376C(0xe as libc::c_int as u8_0,
                                  ((*dbCamera).sub.demoCtrlActionIdx as
                                       libc::c_int + 0x16 as libc::c_int) as
                                      u8_0, 7 as libc::c_int as u8_0,
                                  D_8012CF0C);
                    func_8006376C(0xd as libc::c_int as u8_0,
                                  0x1a as libc::c_int as u8_0,
                                  5 as libc::c_int as u8_0,
                                  D_8012CF60[0 as libc::c_int as usize]);
                    func_8006376C(0x14 as libc::c_int as u8_0,
                                  0x1a as libc::c_int as u8_0,
                                  5 as libc::c_int as u8_0, D_8012CF70);
                    if !((*sGlobalCtx).state.input[2 as libc::c_int as
                                                       usize].press.button as
                             libc::c_int | !(0x800 as libc::c_int)) ==
                           0 as libc::c_int {
                        Audio_PlaySoundGeneral(0x4809 as libc::c_int as u16_0,
                                               &mut D_801333D4,
                                               4 as libc::c_int as u8_0,
                                               &mut D_801333E0,
                                               &mut D_801333E0,
                                               &mut D_801333E8);
                        (*dbCamera).sub.demoCtrlActionIdx =
                            (((*dbCamera).sub.demoCtrlActionIdx as libc::c_int
                                  - 1 as libc::c_int) as
                                 libc::c_uint).wrapping_rem(4 as libc::c_uint)
                                as s16
                    }
                    if !((*sGlobalCtx).state.input[2 as libc::c_int as
                                                       usize].press.button as
                             libc::c_int | !(0x400 as libc::c_int)) ==
                           0 as libc::c_int {
                        Audio_PlaySoundGeneral(0x4809 as libc::c_int as u16_0,
                                               &mut D_801333D4,
                                               4 as libc::c_int as u8_0,
                                               &mut D_801333E0,
                                               &mut D_801333E0,
                                               &mut D_801333E8);
                        (*dbCamera).sub.demoCtrlActionIdx =
                            (((*dbCamera).sub.demoCtrlActionIdx as libc::c_int
                                  + 1 as libc::c_int) as
                                 libc::c_uint).wrapping_rem(4 as libc::c_uint)
                                as s16
                    }
                    if !((*sGlobalCtx).state.input[2 as libc::c_int as
                                                       usize].press.button as
                             libc::c_int | !(0x8000 as libc::c_int)) ==
                           0 as libc::c_int {
                        Audio_PlaySoundGeneral(0x4808 as libc::c_int as u16_0,
                                               &mut D_801333D4,
                                               4 as libc::c_int as u8_0,
                                               &mut D_801333E0,
                                               &mut D_801333E0,
                                               &mut D_801333E8);
                        (*dbCamera).sub.demoCtrlToggleSwitch =
                            0 as libc::c_int as s16;
                        (*dbCamera).sub.demoCtrlMenu =
                            ((*dbCamera).sub.demoCtrlActionIdx as libc::c_int
                                 * 100 as libc::c_int + 0 as libc::c_int) as
                                s16
                    }
                    if !((*sGlobalCtx).state.input[2 as libc::c_int as
                                                       usize].press.button as
                             libc::c_int | !(0x4000 as libc::c_int)) ==
                           0 as libc::c_int {
                        Audio_PlaySoundGeneral(0x480a as libc::c_int as u16_0,
                                               &mut D_801333D4,
                                               4 as libc::c_int as u8_0,
                                               &mut D_801333E0,
                                               &mut D_801333E0,
                                               &mut D_801333E8);
                        (*dbCamera).sub.demoCtrlActionIdx =
                            0 as libc::c_int as s16;
                        return 1 as libc::c_int
                    }
                }
                _ => { }
            }
            return 1 as libc::c_int
        }
        _ => {
            if !((*sGlobalCtx).state.input[2 as libc::c_int as
                                               usize].press.button as
                     libc::c_int | !(0x800 as libc::c_int)) ==
                   0 as libc::c_int {
                Audio_PlaySoundGeneral(0x4809 as libc::c_int as u16_0,
                                       &mut D_801333D4,
                                       4 as libc::c_int as u8_0,
                                       &mut D_801333E0, &mut D_801333E0,
                                       &mut D_801333E8);
                (*dbCamera).sub.demoCtrlMenu =
                    (0 as libc::c_int * 100 as libc::c_int + 0 as libc::c_int)
                        as s16;
                (*dbCamera).sub.demoCtrlActionIdx =
                    (((*dbCamera).sub.demoCtrlActionIdx as libc::c_int -
                          1 as libc::c_int) as
                         libc::c_uint).wrapping_rem(4 as libc::c_uint) as s16;
                sCurFileIdx = 0 as libc::c_int as s16
            }
            if !((*sGlobalCtx).state.input[2 as libc::c_int as
                                               usize].press.button as
                     libc::c_int | !(0x400 as libc::c_int)) ==
                   0 as libc::c_int {
                Audio_PlaySoundGeneral(0x4809 as libc::c_int as u16_0,
                                       &mut D_801333D4,
                                       4 as libc::c_int as u8_0,
                                       &mut D_801333E0, &mut D_801333E0,
                                       &mut D_801333E8);
                (*dbCamera).sub.demoCtrlMenu =
                    (0 as libc::c_int * 100 as libc::c_int + 0 as libc::c_int)
                        as s16;
                (*dbCamera).sub.demoCtrlActionIdx =
                    (((*dbCamera).sub.demoCtrlActionIdx as libc::c_int +
                          1 as libc::c_int) as
                         libc::c_uint).wrapping_rem(4 as libc::c_uint) as s16;
                sCurFileIdx = 0 as libc::c_int as s16
            }
            DbCamera_DrawSlotLetters(sp74.as_mut_ptr(),
                                     7 as libc::c_int as s16,
                                     5 as libc::c_int as s16,
                                     4 as libc::c_int);
            if sDbCamAnim.unk_0A as libc::c_int != 0 as libc::c_int {
                func_8006376C(4 as libc::c_int as u8_0,
                              7 as libc::c_int as u8_0,
                              5 as libc::c_int as u8_0, D_8012CF4C);
                func_8006376C((D_8016110C as libc::c_int * 2 as libc::c_int +
                                   6 as libc::c_int) as u8_0,
                              7 as libc::c_int as u8_0,
                              7 as libc::c_int as u8_0,
                              b">\x00" as *const u8 as *const libc::c_char);
                if !((*sGlobalCtx).state.input[2 as libc::c_int as
                                                   usize].press.button as
                         libc::c_int | !(0x8 as libc::c_int)) ==
                       0 as libc::c_int {
                    if D_8016110C as libc::c_int > 0 as libc::c_int {
                        D_8016110C -= 1
                    }
                    sDbCamAnim.curFrame = 0.0f32;
                    sDbCamAnim.keyframe = 0 as libc::c_int as s16;
                    sDbCamAnim.unk_04 = 0 as libc::c_int as f32_0
                } else if !((*sGlobalCtx).state.input[2 as libc::c_int as
                                                          usize].press.button
                                as libc::c_int | !(0x4 as libc::c_int)) ==
                              0 as libc::c_int {
                    if (D_8016110C as libc::c_int) < 14 as libc::c_int {
                        D_8016110C += 1
                    }
                    sDbCamAnim.curFrame = 0.0f32;
                    sDbCamAnim.keyframe = 0 as libc::c_int as s16;
                    sDbCamAnim.unk_04 = 0 as libc::c_int as f32_0
                } else if !((*sGlobalCtx).state.input[2 as libc::c_int as
                                                          usize].press.button
                                as libc::c_int | !(0x2 as libc::c_int)) ==
                              0 as libc::c_int {
                    sDbCamAnim.unk_0A = 0 as libc::c_int as s16;
                    Interface_ChangeAlpha(2 as libc::c_int as u16_0);
                    ShrinkWindow_SetVal(0 as libc::c_int);
                    D_8016110C = 0 as libc::c_int as s16;
                    return 2 as libc::c_int
                }
                if func_800B91B0(cam, dbCamera) == 0 as libc::c_int {
                    Interface_ChangeAlpha(2 as libc::c_int as u16_0);
                    ShrinkWindow_SetVal(0 as libc::c_int);
                    Audio_PlaySoundGeneral(0x4803 as libc::c_int as u16_0,
                                           &mut D_801333D4,
                                           4 as libc::c_int as u8_0,
                                           &mut D_801333E0, &mut D_801333E0,
                                           &mut D_801333E8);
                }
                OLib_Vec3fDiffToVecSphGeo(&mut sp5C, &mut (*dbCamera).eye,
                                          &mut (*dbCamera).at);
                DbCamera_CalcUpFromPitchYawRoll(&mut (*dbCamera).unk_1C,
                                                sp5C.pitch, sp5C.yaw,
                                                ((*dbCamera).rollDegrees *
                                                     182.04167f32 + 0.5f32) as
                                                    s16);
                return 2 as libc::c_int
            }
            if !((*sGlobalCtx).state.input[1 as libc::c_int as
                                               usize].press.button as
                     libc::c_int | !(0x1 as libc::c_int)) == 0 as libc::c_int
               {
                D_8015FCC8 = 0 as libc::c_int as u8_0;
                gSaveContext.cutsceneIndex = 0xfffd as libc::c_int;
                gSaveContext.cutsceneTrigger = 1 as libc::c_int as u8_0;
                sDbCamAnim.curFrame = 0.0f32;
                sDbCamAnim.keyframe = 0 as libc::c_int as s16;
                sDbCamAnim.unk_04 = 0 as libc::c_int as f32_0;
                sDbCamAnim.unk_0A = 1 as libc::c_int as s16;
                sDbCamAnim.unk_0C = 0 as libc::c_int as s16;
                D_8016110C = 0 as libc::c_int as s16;
                Audio_PlaySoundGeneral(0x480b as libc::c_int as u16_0,
                                       &mut D_801333D4,
                                       4 as libc::c_int as u8_0,
                                       &mut D_801333E0, &mut D_801333E0,
                                       &mut D_801333E8);
            }
            if !((*sGlobalCtx).state.input[2 as libc::c_int as
                                               usize].press.button as
                     libc::c_int | !(0x20 as libc::c_int)) == 0 as libc::c_int
               {
                if sp74[sCurFileIdx as usize] as libc::c_int == '?' as i32 {
                    sLastFileIdx = -(1 as libc::c_int) as s16;
                    D_801612EA = '*' as i32 as libc::c_char
                } else {
                    sLastFileIdx = sCurFileIdx;
                    D_801612EA = sDbCameraCuts[idx1 as usize].letter
                }
            } else if !(!((*sGlobalCtx).state.input[2 as libc::c_int as
                                                        usize].cur.button as
                              libc::c_int | !(0x20 as libc::c_int)) ==
                            0 as libc::c_int) {
                if sLastFileIdx as libc::c_int != -(1 as libc::c_int) {
                    match sp74[sCurFileIdx as usize] as libc::c_int {
                        63 => {
                            Audio_PlaySoundGeneral(0x4808 as libc::c_int as
                                                       u16_0, &mut D_801333D4,
                                                   4 as libc::c_int as u8_0,
                                                   &mut D_801333E0,
                                                   &mut D_801333E0,
                                                   &mut D_801333E8);
                            sDbCameraCuts[idx1 as usize] =
                                sDbCameraCuts[idx2 as usize];
                            sp74[sCurFileIdx as usize] =
                                '?' as i32 as libc::c_char;
                            DbCamera_ResetCut(idx2, 0 as libc::c_int);
                        }
                        45 => {
                            Audio_PlaySoundGeneral(0x4808 as libc::c_int as
                                                       u16_0, &mut D_801333D4,
                                                   4 as libc::c_int as u8_0,
                                                   &mut D_801333E0,
                                                   &mut D_801333E0,
                                                   &mut D_801333E8);
                            sp64 = sDbCameraCuts[idx2 as usize];
                            if (sLastFileIdx as libc::c_int) <
                                   sCurFileIdx as libc::c_int {
                                // rotate right
                                i = idx2;
                                while i < idx1 - 1 as libc::c_int &&
                                          i <
                                              (::std::mem::size_of::<[DbCameraCut; 16]>()
                                                   as
                                                   libc::c_ulong).wrapping_div(::std::mem::size_of::<DbCameraCut>()
                                                                                   as
                                                                                   libc::c_ulong)
                                                  as s32 - 1 as libc::c_int {
                                    sDbCameraCuts[i as usize] =
                                        sDbCameraCuts[(i + 1 as libc::c_int)
                                                          as usize];
                                    i += 1
                                }
                                sDbCameraCuts[(idx1 - 1 as libc::c_int) as
                                                  usize] = sp64
                            } else if (sCurFileIdx as libc::c_int) <
                                          sLastFileIdx as libc::c_int {
                                // rotate left
                                i = idx2;
                                while idx1 < i && i > 0 as libc::c_int {
                                    sDbCameraCuts[i as usize] =
                                        sDbCameraCuts[(i - 1 as libc::c_int)
                                                          as usize];
                                    i -= 1
                                }
                                sDbCameraCuts[idx1 as usize] = sp64
                            }
                            i = 0 as libc::c_int;
                            while i <
                                      (::std::mem::size_of::<[DbCameraCut; 16]>()
                                           as
                                           libc::c_ulong).wrapping_div(::std::mem::size_of::<DbCameraCut>()
                                                                           as
                                                                           libc::c_ulong)
                                          as s32 - 1 as libc::c_int {
                                sp74[(i * 2 as libc::c_int + 1 as libc::c_int)
                                         as usize] =
                                    sDbCameraCuts[i as usize].letter;
                                i += 1
                            }
                        }
                        _ => {
                            Audio_PlaySoundGeneral(0x4806 as libc::c_int as
                                                       u16_0, &mut D_801333D4,
                                                   4 as libc::c_int as u8_0,
                                                   &mut D_801333E0,
                                                   &mut D_801333E0,
                                                   &mut D_801333E8);
                        }
                    }
                }
                sLastFileIdx = -(1 as libc::c_int) as s16
            }
            if !((*sGlobalCtx).state.input[2 as libc::c_int as
                                               usize].press.button as
                     libc::c_int | !(0x8000 as libc::c_int)) ==
                   0 as libc::c_int {
                if sp74[sCurFileIdx as usize] as libc::c_int == '?' as i32 {
                    Audio_PlaySoundGeneral(0x4808 as libc::c_int as u16_0,
                                           &mut D_801333D4,
                                           4 as libc::c_int as u8_0,
                                           &mut D_801333E0, &mut D_801333E0,
                                           &mut D_801333E8);
                    sp74[sCurFileIdx as usize] =
                        DbCamera_InitCut(idx1, &mut (*dbCamera).sub);
                    if sp74[sCurFileIdx as usize] as libc::c_int == '?' as i32
                       {
                        func_8006376C(0xf as libc::c_int as u8_0,
                                      0x18 as libc::c_int as u8_0,
                                      7 as libc::c_int as u8_0, D_8012CF48);
                    }
                }
            }
            if !((*sGlobalCtx).state.input[2 as libc::c_int as
                                               usize].press.button as
                     libc::c_int | !(0x4000 as libc::c_int)) ==
                   0 as libc::c_int {
                if sp74[sCurFileIdx as usize] as libc::c_int != '?' as i32 &&
                       sp74[sCurFileIdx as usize] as libc::c_int != '-' as i32
                   {
                    Audio_PlaySoundGeneral(0x480a as libc::c_int as u16_0,
                                           &mut D_801333D4,
                                           4 as libc::c_int as u8_0,
                                           &mut D_801333E0, &mut D_801333E0,
                                           &mut D_801333E8);
                    sp74[sCurFileIdx as usize] = '?' as i32 as libc::c_char;
                    DbCamera_ResetCut(idx1, 1 as libc::c_int);
                }
            }
            if !((*sGlobalCtx).state.input[2 as libc::c_int as
                                               usize].press.button as
                     libc::c_int | !(0x10 as libc::c_int)) == 0 as libc::c_int
               {
                if sp74[sCurFileIdx as usize] as libc::c_int != '?' as i32 &&
                       sp74[sCurFileIdx as usize] as libc::c_int != '-' as i32
                   {
                    Audio_PlaySoundGeneral(0x4808 as libc::c_int as u16_0,
                                           &mut D_801333D4,
                                           4 as libc::c_int as u8_0,
                                           &mut D_801333E0, &mut D_801333E0,
                                           &mut D_801333E8);
                    i = 0 as libc::c_int;
                    while i <
                              sDbCameraCuts[idx1 as usize].nPoints as
                                  libc::c_int {
                        (*dbCamera).sub.lookAt[i as usize] =
                            *sDbCameraCuts[idx1 as
                                               usize].lookAt.offset(i as
                                                                        isize);
                        i += 1
                    }
                    // why use another loop for that...
                    i = 0 as libc::c_int;
                    while i <
                              sDbCameraCuts[idx1 as usize].nPoints as
                                  libc::c_int {
                        (*dbCamera).sub.position[i as usize] =
                            *sDbCameraCuts[idx1 as
                                               usize].position.offset(i as
                                                                          isize);
                        i += 1
                    }
                    (*dbCamera).sub.mode = sDbCameraCuts[idx1 as usize].mode;
                    (*dbCamera).sub.nFrames =
                        sDbCameraCuts[idx1 as usize].nFrames;
                    (*dbCamera).sub.nPoints =
                        sDbCameraCuts[idx1 as usize].nPoints;
                    (*dbCamera).sub.unkIdx = 0 as libc::c_int as s16;
                    func_800B41DC(dbCamera, (*dbCamera).sub.unkIdx, cam);
                    sp74[sCurFileIdx as usize] = '?' as i32 as libc::c_char;
                    DbCamera_ResetCut(idx1, 1 as libc::c_int);
                    (*dbCamera).unk_00 = 1 as libc::c_int
                }
            }
            if !((*sGlobalCtx).state.input[2 as libc::c_int as
                                               usize].press.button as
                     libc::c_int | !(0x100 as libc::c_int)) ==
                   0 as libc::c_int {
                Audio_PlaySoundGeneral(0x4809 as libc::c_int as u16_0,
                                       &mut D_801333D4,
                                       4 as libc::c_int as u8_0,
                                       &mut D_801333E0, &mut D_801333E0,
                                       &mut D_801333E8);
                if sCurFileIdx as libc::c_int == 0x1e as libc::c_int {
                    sCurFileIdx = 0 as libc::c_int as s16
                } else { sCurFileIdx += 1 }
            }
            if !((*sGlobalCtx).state.input[2 as libc::c_int as
                                               usize].press.button as
                     libc::c_int | !(0x200 as libc::c_int)) ==
                   0 as libc::c_int {
                Audio_PlaySoundGeneral(0x4809 as libc::c_int as u16_0,
                                       &mut D_801333D4,
                                       4 as libc::c_int as u8_0,
                                       &mut D_801333E0, &mut D_801333E0,
                                       &mut D_801333E8);
                sCurFileIdx =
                    if sCurFileIdx as libc::c_int == 0 as libc::c_int {
                        0x1e as libc::c_int
                    } else { (sCurFileIdx as libc::c_int) - 1 as libc::c_int }
                        as s16
            }
            if !((*sGlobalCtx).state.input[2 as libc::c_int as
                                               usize].cur.button as
                     libc::c_int | !(0x20 as libc::c_int)) == 0 as libc::c_int
                   &&
                   !((*sGlobalCtx).state.input[2 as libc::c_int as
                                                   usize].press.button as
                         libc::c_int | !(0x1 as libc::c_int)) ==
                       0 as libc::c_int {
                i = 0 as libc::c_int;
                while i <
                          (::std::mem::size_of::<[DbCameraCut; 16]>() as
                               libc::c_ulong).wrapping_div(::std::mem::size_of::<DbCameraCut>()
                                                               as
                                                               libc::c_ulong)
                              as s32 - 1 as libc::c_int {
                    osSyncPrintf(b"###%2d:(%c) (%d %d) %d %d %d\n\x00" as
                                     *const u8 as *const libc::c_char, i,
                                 sDbCameraCuts[i as usize].letter as
                                     libc::c_int,
                                 sDbCameraCuts[i as usize].position,
                                 sDbCameraCuts[i as usize].lookAt,
                                 sDbCameraCuts[i as usize].nFrames as
                                     libc::c_int,
                                 sDbCameraCuts[i as usize].nPoints as
                                     libc::c_int,
                                 sDbCameraCuts[i as usize].mode as
                                     libc::c_int);
                    i += 1
                }
                DbCamera_PrintAllCuts(cam);
            } else if !((*sGlobalCtx).state.input[2 as libc::c_int as
                                                      usize].cur.button as
                            libc::c_int | !(0x20 as libc::c_int)) ==
                          0 as libc::c_int &&
                          !((*sGlobalCtx).state.input[2 as libc::c_int as
                                                          usize].press.button
                                as libc::c_int | !(0x2 as libc::c_int)) ==
                              0 as libc::c_int {
                Audio_PlaySoundGeneral(0x4803 as libc::c_int as u16_0,
                                       &mut D_801333D4,
                                       4 as libc::c_int as u8_0,
                                       &mut D_801333E0, &mut D_801333E0,
                                       &mut D_801333E8);
                i = 0 as libc::c_int;
                while i <
                          (::std::mem::size_of::<[DbCameraCut; 16]>() as
                               libc::c_ulong).wrapping_div(::std::mem::size_of::<DbCameraCut>()
                                                               as
                                                               libc::c_ulong)
                              as s32 - 1 as libc::c_int {
                    if sDbCameraCuts[i as usize].nPoints as libc::c_int !=
                           0 as libc::c_int {
                        osSyncPrintf(b"\n@@@ /* CUT [%d]\t*/\x00" as *const u8
                                         as *const libc::c_char, i);
                        DbCamera_PrintCutBytes(&mut *sDbCameraCuts.as_mut_ptr().offset(i
                                                                                           as
                                                                                           isize));
                    }
                    i += 1
                }
            } else if !((*sGlobalCtx).state.input[2 as libc::c_int as
                                                      usize].press.button as
                            libc::c_int | !(0x1 as libc::c_int)) ==
                          0 as libc::c_int {
                sDbCamAnim.curFrame = 0.0f32;
                sDbCamAnim.keyframe = 0 as libc::c_int as s16;
                sDbCamAnim.unk_04 = 0.0f32;
                sDbCamAnim.unk_0A = 1 as libc::c_int as s16;
                sDbCamAnim.unk_0C = 0 as libc::c_int as s16;
                Interface_ChangeAlpha(50 as libc::c_int as u16_0);
                ShrinkWindow_SetVal(0x20 as libc::c_int);
                D_8016110C = 0 as libc::c_int as s16;
                Audio_PlaySoundGeneral(0x480b as libc::c_int as u16_0,
                                       &mut D_801333D4,
                                       4 as libc::c_int as u8_0,
                                       &mut D_801333E0, &mut D_801333E0,
                                       &mut D_801333E8);
            }
            func_8006376C(4 as libc::c_int as u8_0, 7 as libc::c_int as u8_0,
                          5 as libc::c_int as u8_0,
                          D_8012CF50[0 as libc::c_int as usize]);
            sp74[1 as libc::c_int as usize] =
                0 as libc::c_int as libc::c_char;
            if sLastFileIdx as libc::c_int != -(1 as libc::c_int) {
                sp74[0 as libc::c_int as usize] = D_801612EA;
                func_8006376C((sLastFileIdx as libc::c_int + 5 as libc::c_int)
                                  as u8_0, 7 as libc::c_int as u8_0,
                              2 as libc::c_int as u8_0, sp74.as_mut_ptr());
            } else {
                sp74[0 as libc::c_int as usize] = '_' as i32 as libc::c_char
            }
            func_8006376C((sCurFileIdx as libc::c_int + 5 as libc::c_int) as
                              u8_0, 7 as libc::c_int as u8_0,
                          7 as libc::c_int as u8_0, sp74.as_mut_ptr());
            return 1 as libc::c_int
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn func_800BB03C(mut cam: *mut Camera) {
    func_800B91B0(cam, sDbCamPtr);
}
#[no_mangle]
pub unsafe extern "C" fn func_800BB060() {
    sDbCamAnim.unk_0A = 0 as libc::c_int as s16;
}
#[no_mangle]
pub unsafe extern "C" fn func_800BB06C() -> s32 {
    return ((*sDbCamPtr).unk_00 == 2 as libc::c_int &&
                sDbCamAnim.unk_0A as libc::c_int != 0 as libc::c_int) as
               libc::c_int;
}
