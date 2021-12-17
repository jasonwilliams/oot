#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case,
         non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(register_tool)]
extern "C" {
    #[no_mangle]
    fn osSyncPrintf(fmt: *const libc::c_char, _: ...);
    #[no_mangle]
    fn func_8002DF38(globalCtx: *mut GlobalContext, actor: *mut Actor,
                     csMode: u8_0) -> s32;
    #[no_mangle]
    fn func_8002DF54(globalCtx: *mut GlobalContext, actor: *mut Actor,
                     arg2: u8_0) -> s32;
    #[no_mangle]
    fn Actor_GetFocus(arg0: *mut PosRot, actor: *mut Actor) -> *mut PosRot;
    #[no_mangle]
    fn Actor_GetWorld(arg0: *mut PosRot, actor: *mut Actor) -> *mut PosRot;
    #[no_mangle]
    fn Actor_GetWorldPosShapeRot(arg0: *mut PosRot, actor: *mut Actor)
     -> *mut PosRot;
    #[no_mangle]
    fn Actor_GetScreenPos(globalCtx: *mut GlobalContext, actor: *mut Actor,
                          x: *mut s16, y: *mut s16);
    #[no_mangle]
    fn BgCheck_EntityRaycastFloor3(colCtx: *mut CollisionContext,
                                   outPoly: *mut *mut CollisionPoly,
                                   bgId: *mut s32, pos: *mut Vec3f) -> f32_0;
    #[no_mangle]
    fn BgCheck_CameraLineTest1(colCtx: *mut CollisionContext,
                               posA: *mut Vec3f, posB: *mut Vec3f,
                               posResult: *mut Vec3f,
                               outPoly: *mut *mut CollisionPoly, chkWall: s32,
                               chkFloor: s32, chkCeil: s32, chkOneFace: s32,
                               bgId: *mut s32) -> s32;
    #[no_mangle]
    fn Camera_ChangeMode(camera: *mut Camera, mode: s16) -> s32;
    #[no_mangle]
    fn func_80078884(sfxId: u16_0);
    #[no_mangle]
    fn OLib_VecSphGeoToVec3f(dest: *mut Vec3f, sph: *mut VecSph)
     -> *mut Vec3f;
    #[no_mangle]
    fn OLib_Vec3fDiffToVecSphGeo(arg0: *mut VecSph, a: *mut Vec3f,
                                 b: *mut Vec3f) -> *mut VecSph;
    #[no_mangle]
    fn Interface_ChangeAlpha(alphaType: u16_0);
    #[no_mangle]
    fn Quake_SetSpeed(idx: s16, value: s16) -> u32_0;
    #[no_mangle]
    fn Quake_SetCountdown(idx: s16, value: s16) -> u32_0;
    #[no_mangle]
    fn Quake_SetQuakeValues(idx: s16, y: s16, x: s16, zoom: s16, rotZ: s16)
     -> u32_0;
    #[no_mangle]
    fn Quake_Add(cam: *mut Camera, callbackIdx: u32_0) -> s16;
    #[no_mangle]
    fn Gameplay_CreateSubCamera(globalCtx: *mut GlobalContext) -> s16;
    #[no_mangle]
    fn Gameplay_ChangeCameraStatus(globalCtx: *mut GlobalContext, camId: s16,
                                   status: s16) -> s16;
    #[no_mangle]
    fn Gameplay_ClearCamera(globalCtx: *mut GlobalContext, camId: s16);
    #[no_mangle]
    fn Gameplay_CameraSetAtEye(globalCtx: *mut GlobalContext, camId: s16,
                               at: *mut Vec3f, eye: *mut Vec3f) -> s32;
    #[no_mangle]
    fn Gameplay_CameraSetFov(globalCtx: *mut GlobalContext, camId: s16,
                             fov: f32_0) -> s32;
    #[no_mangle]
    fn Gameplay_SetCameraRoll(globalCtx: *mut GlobalContext, camId: s16,
                              roll: s16) -> s32;
    #[no_mangle]
    fn Gameplay_CopyCamera(globalCtx: *mut GlobalContext, camId1: s16,
                           camId2: s16);
    #[no_mangle]
    fn func_800C0808(globalCtx: *mut GlobalContext, camId: s16,
                     player: *mut Player, arg3: s16) -> s32;
    #[no_mangle]
    fn Gameplay_CameraChangeSetting(globalCtx: *mut GlobalContext, camId: s16,
                                    arg2: s16) -> s32;
    #[no_mangle]
    fn func_800C0CB8(globalCtx: *mut GlobalContext) -> s32;
    #[no_mangle]
    static mut gSaveContext: SaveContext;
    #[no_mangle]
    fn Math_FAtan2F(y: f32_0, x: f32_0) -> f32_0;
    #[no_mangle]
    fn Rand_ZeroOne() -> f32_0;
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
pub struct ColliderJntSphElementDim {
    pub modelSphere: Sphere16,
    pub worldSphere: Sphere16,
    pub scale: f32_0,
    pub limb: u8_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ColliderJntSphElement {
    pub info: ColliderInfo,
    pub dim: ColliderJntSphElementDim,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ColliderJntSph {
    pub base: Collider,
    pub count: s32,
    pub elements: *mut ColliderJntSphElement,
}
pub type C2RustUnnamed_14 = libc::c_uint;
pub const ACTORCAT_CHEST: C2RustUnnamed_14 = 11;
pub const ACTORCAT_DOOR: C2RustUnnamed_14 = 10;
pub const ACTORCAT_BOSS: C2RustUnnamed_14 = 9;
pub const ACTORCAT_MISC: C2RustUnnamed_14 = 8;
pub const ACTORCAT_ITEMACTION: C2RustUnnamed_14 = 7;
pub const ACTORCAT_PROP: C2RustUnnamed_14 = 6;
pub const ACTORCAT_ENEMY: C2RustUnnamed_14 = 5;
pub const ACTORCAT_NPC: C2RustUnnamed_14 = 4;
pub const ACTORCAT_EXPLOSIVE: C2RustUnnamed_14 = 3;
pub const ACTORCAT_PLAYER: C2RustUnnamed_14 = 2;
pub const ACTORCAT_BG: C2RustUnnamed_14 = 1;
pub const ACTORCAT_SWITCH: C2RustUnnamed_14 = 0;
pub type C2RustUnnamed_15 = libc::c_uint;
pub const CAM_SET_MAX: C2RustUnnamed_15 = 66;
pub const CAM_SET_NORMAL4: C2RustUnnamed_15 = 65;
pub const CAM_SET_PIVOT_FROM_SIDE: C2RustUnnamed_15 = 64;
pub const CAM_SET_DIRECTED_YAW: C2RustUnnamed_15 = 63;
pub const CAM_SET_DUNGEON2: C2RustUnnamed_15 = 62;
pub const CAM_SET_JABU_TENTACLE: C2RustUnnamed_15 = 61;
pub const CAM_SET_CS_C: C2RustUnnamed_15 = 60;
pub const CAM_SET_FISHING: C2RustUnnamed_15 = 59;
pub const CAM_SET_NORMAL2: C2RustUnnamed_15 = 58;
pub const CAM_SET_PIVOT_VERTICAL: C2RustUnnamed_15 = 57;
pub const CAM_SET_TURN_AROUND: C2RustUnnamed_15 = 56;
pub const CAM_SET_FIRE_BIRDS_EYE: C2RustUnnamed_15 = 55;
pub const CAM_SET_MEADOW_UNUSED: C2RustUnnamed_15 = 54;
pub const CAM_SET_MEADOW_BIRDS_EYE: C2RustUnnamed_15 = 53;
pub const CAM_SET_BIG_OCTO: C2RustUnnamed_15 = 52;
pub const CAM_SET_FOREST_DEFEAT_POE: C2RustUnnamed_15 = 51;
pub const CAM_SET_FOREST_UNUSED: C2RustUnnamed_15 = 50;
pub const CAM_SET_FIRE_STAIRCASE: C2RustUnnamed_15 = 49;
pub const CAM_SET_FIRE_PLATFORM: C2RustUnnamed_15 = 48;
pub const CAM_SET_SCENE_TRANSITION: C2RustUnnamed_15 = 47;
pub const CAM_SET_SCENE_UNUSED: C2RustUnnamed_15 = 46;
pub const CAM_SET_BEAN_LOST_WOODS: C2RustUnnamed_15 = 45;
pub const CAM_SET_BEAN_GENERIC: C2RustUnnamed_15 = 44;
pub const CAM_SET_CS_ATTENTION: C2RustUnnamed_15 = 43;
pub const CAM_SET_CS_3: C2RustUnnamed_15 = 42;
pub const CAM_SET_ITEM_UNUSED: C2RustUnnamed_15 = 41;
pub const CAM_SET_SLOW_CHEST_CS: C2RustUnnamed_15 = 40;
pub const CAM_SET_FOREST_BIRDS_EYE: C2RustUnnamed_15 = 39;
pub const CAM_SET_CS_TWISTED_HALLWAY: C2RustUnnamed_15 = 38;
pub const CAM_SET_CS_0: C2RustUnnamed_15 = 37;
pub const CAM_SET_PIVOT_WATER_SURFACE: C2RustUnnamed_15 = 36;
pub const CAM_SET_PIVOT_CORNER: C2RustUnnamed_15 = 35;
pub const CAM_SET_FREE2: C2RustUnnamed_15 = 34;
pub const CAM_SET_FREE0: C2RustUnnamed_15 = 33;
pub const CAM_SET_START1: C2RustUnnamed_15 = 32;
pub const CAM_SET_START0: C2RustUnnamed_15 = 31;
pub const CAM_SET_CRAWLSPACE: C2RustUnnamed_15 = 30;
pub const CAM_SET_DOORC: C2RustUnnamed_15 = 29;
pub const CAM_SET_DOOR0: C2RustUnnamed_15 = 28;
pub const CAM_SET_PREREND_SIDE_SCROLL: C2RustUnnamed_15 = 27;
pub const CAM_SET_PREREND_PIVET: C2RustUnnamed_15 = 26;
pub const CAM_SET_PREREND_FIXED: C2RustUnnamed_15 = 25;
pub const CAM_SET_PIVOT_IN_FRONT: C2RustUnnamed_15 = 24;
pub const CAM_SET_PIVOT_SHOP_BROWSING: C2RustUnnamed_15 = 23;
pub const CAM_SET_PIVOT_CRAWLSPACE: C2RustUnnamed_15 = 22;
pub const CAM_SET_CHU_BOWLING: C2RustUnnamed_15 = 21;
pub const CAM_SET_MARKET_BALCONY: C2RustUnnamed_15 = 20;
pub const CAM_SET_TOWER_UNUSED: C2RustUnnamed_15 = 19;
pub const CAM_SET_TOWER_CLIMB: C2RustUnnamed_15 = 18;
pub const CAM_SET_BOSS_GANON: C2RustUnnamed_15 = 17;
pub const CAM_SET_BOSS_GANONDORF: C2RustUnnamed_15 = 16;
pub const CAM_SET_BOSS_TWINROVA_FLOOR: C2RustUnnamed_15 = 15;
pub const CAM_SET_BOSS_TWINROVA_PLATFORM: C2RustUnnamed_15 = 14;
pub const CAM_SET_BOSS_MORPHA: C2RustUnnamed_15 = 13;
pub const CAM_SET_BOSS_BONGO: C2RustUnnamed_15 = 12;
pub const CAM_SET_BOSS_VOLVAGIA: C2RustUnnamed_15 = 11;
pub const CAM_SET_BOSS_PHANTOM_GANON: C2RustUnnamed_15 = 10;
pub const CAM_SET_BOSS_BARINADE: C2RustUnnamed_15 = 9;
pub const CAM_SET_BOSS_DODONGO: C2RustUnnamed_15 = 8;
pub const CAM_SET_BOSS_GOHMA: C2RustUnnamed_15 = 7;
pub const CAM_SET_HORSE: C2RustUnnamed_15 = 6;
pub const CAM_SET_NORMAL3: C2RustUnnamed_15 = 5;
pub const CAM_SET_DUNGEON1: C2RustUnnamed_15 = 4;
pub const CAM_SET_DUNGEON0: C2RustUnnamed_15 = 3;
pub const CAM_SET_NORMAL1: C2RustUnnamed_15 = 2;
pub const CAM_SET_NORMAL0: C2RustUnnamed_15 = 1;
pub const CAM_SET_NONE: C2RustUnnamed_15 = 0;
pub type C2RustUnnamed_16 = libc::c_uint;
pub const CAM_MODE_MAX: C2RustUnnamed_16 = 21;
pub const CAM_MODE_FOLLOWBOOMERANG: C2RustUnnamed_16 = 20;
pub const CAM_MODE_PUSHPULL: C2RustUnnamed_16 = 19;
pub const CAM_MODE_STILL: C2RustUnnamed_16 = 18;
pub const CAM_MODE_CHARGE: C2RustUnnamed_16 = 17;
pub const CAM_MODE_FREEFALL: C2RustUnnamed_16 = 16;
pub const CAM_MODE_HANGZ: C2RustUnnamed_16 = 15;
pub const CAM_MODE_HANG: C2RustUnnamed_16 = 14;
pub const CAM_MODE_JUMP: C2RustUnnamed_16 = 13;
pub const CAM_MODE_CLIMBZ: C2RustUnnamed_16 = 12;
pub const CAM_MODE_SLINGSHOT: C2RustUnnamed_16 = 11;
pub const CAM_MODE_BOOMERANG: C2RustUnnamed_16 = 10;
pub const CAM_MODE_HOOKSHOT: C2RustUnnamed_16 = 9;
pub const CAM_MODE_BOWARROWZ: C2RustUnnamed_16 = 8;
pub const CAM_MODE_BOWARROW: C2RustUnnamed_16 = 7;
pub const CAM_MODE_FIRSTPERSON: C2RustUnnamed_16 = 6;
pub const CAM_MODE_CLIMB: C2RustUnnamed_16 = 5;
pub const CAM_MODE_BATTLE: C2RustUnnamed_16 = 4;
pub const CAM_MODE_TALK: C2RustUnnamed_16 = 3;
pub const CAM_MODE_FOLLOWTARGET: C2RustUnnamed_16 = 2;
pub const CAM_MODE_TARGET: C2RustUnnamed_16 = 1;
pub const CAM_MODE_NORMAL: C2RustUnnamed_16 = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct OnePointCsFull {
    pub actionFlags: u8_0,
    pub unk_01: u8_0,
    pub initFlags: s16,
    pub timerInit: s16,
    pub rollTargetInit: s16,
    pub fovTargetInit: f32_0,
    pub lerpStepScale: f32_0,
    pub atTargetInit: Vec3f,
    pub eyeTargetInit: Vec3f,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Unique9Anim {
    pub curKeyFrame: *mut OnePointCsFull,
    pub atTarget: Vec3f,
    pub eyeTarget: Vec3f,
    pub playerPos: Vec3f,
    pub fovTarget: f32_0,
    pub atEyeOffsetTarget: VecSph,
    pub rollTarget: s16,
    pub curKeyFrameIdx: s16,
    pub unk_38: s16,
    pub isNewKeyFrame: s16,
    pub keyFrameTimer: s16,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Unique9 {
    pub interfaceFlags: s16,
    pub anim: Unique9Anim,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Unique9OnePointCs {
    pub keyFrameCnt: s32,
    pub keyFrames: *mut OnePointCsFull,
    pub uniq9: Unique9,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct OnePointCsCamera {
    pub atPoints: *mut CutsceneCameraPoint,
    pub eyePoints: *mut CutsceneCameraPoint,
    pub actionParameters: s16,
    pub initTimer: s16,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct EnSw {
    pub actor: Actor,
    pub skelAnime: SkelAnime,
    pub actionFunc: EnSwActionFunc,
    pub collider: ColliderJntSph,
    pub sphs: [ColliderJntSphElement; 1],
    pub unk_1F4: Color_RGBA8,
    pub jointTable: [Vec3s; 30],
    pub morphTable: [Vec3s; 30],
    pub unk_360: u8_0,
    pub unk_364: Vec3f,
    pub unk_370: Vec3f,
    pub unk_37C: Vec3f,
    pub unk_388: s16,
    pub unk_38A: s16,
    pub unk_38C: s16,
    pub unk_38E: s16,
    pub unk_390: s16,
    pub unk_392: s16,
    pub unk_394: s16,
    pub unk_396: [libc::c_char; 66],
    pub unk_3D8: MtxF,
    pub unk_418: [libc::c_char; 8],
    pub unk_420: f32_0,
    pub unk_424: [libc::c_char; 8],
    pub unk_42C: u8_0,
    pub unk_430: *mut CollisionPoly,
    pub unk_434: Vec3f,
    pub unk_440: s16,
    pub unk_442: s16,
    pub unk_444: s16,
    pub unk_446: s16,
    pub unk_448: Vec3f,
    pub unk_454: Vec3f,
    pub unk_460: Vec3f,
    pub unk_46C: Vec3f,
    pub unk_478: Vec3f,
    pub unk_484: Vec3f,
    pub unk_490: [libc::c_char; 72],
}
pub type EnSwActionFunc
    =
    Option<unsafe extern "C" fn(_: *mut EnSw, _: *mut GlobalContext) -> ()>;
static mut sDisableAttention: s16 = 0 as libc::c_int as s16;
static mut sUnused: s16 = -(1 as libc::c_int) as s16;
static mut sPrevFrameCs1100: s32 = -(4096 as libc::c_int);
static mut D_8012013C: [CutsceneCameraPoint; 14] =
    [{
         let mut init =
             CutsceneCameraPoint{continueFlag: 0 as libc::c_int as s8,
                                 cameraRoll: 25 as libc::c_int as s8,
                                 nextPointFrame: 40 as libc::c_int as u16_0,
                                 viewAngle: 70.79991f32,
                                 pos:
                                     {
                                         let mut init =
                                             Vec3s{x:
                                                       -(1814 as libc::c_int)
                                                           as s16,
                                                   y:
                                                       533 as libc::c_int as
                                                           s16,
                                                   z:
                                                       -(1297 as libc::c_int)
                                                           as s16,};
                                         init
                                     },};
         init
     },
     {
         let mut init =
             CutsceneCameraPoint{continueFlag: 0 as libc::c_int as s8,
                                 cameraRoll: 20 as libc::c_int as s8,
                                 nextPointFrame: 40 as libc::c_int as u16_0,
                                 viewAngle: 70.99991f32,
                                 pos:
                                     {
                                         let mut init =
                                             Vec3s{x:
                                                       -(1805 as libc::c_int)
                                                           as s16,
                                                   y:
                                                       434 as libc::c_int as
                                                           s16,
                                                   z:
                                                       -(1293 as libc::c_int)
                                                           as s16,};
                                         init
                                     },};
         init
     },
     {
         let mut init =
             CutsceneCameraPoint{continueFlag: 0 as libc::c_int as s8,
                                 cameraRoll: 10 as libc::c_int as s8,
                                 nextPointFrame: 30 as libc::c_int as u16_0,
                                 viewAngle: 60.0f32,
                                 pos:
                                     {
                                         let mut init =
                                             Vec3s{x:
                                                       -(1794 as libc::c_int)
                                                           as s16,
                                                   y:
                                                       323 as libc::c_int as
                                                           s16,
                                                   z:
                                                       -(1280 as libc::c_int)
                                                           as s16,};
                                         init
                                     },};
         init
     },
     {
         let mut init =
             CutsceneCameraPoint{continueFlag: 0 as libc::c_int as s8,
                                 cameraRoll: 5 as libc::c_int as s8,
                                 nextPointFrame: 25 as libc::c_int as u16_0,
                                 viewAngle: 60.0f32,
                                 pos:
                                     {
                                         let mut init =
                                             Vec3s{x:
                                                       -(1817 as libc::c_int)
                                                           as s16,
                                                   y:
                                                       218 as libc::c_int as
                                                           s16,
                                                   z:
                                                       -(1270 as libc::c_int)
                                                           as s16,};
                                         init
                                     },};
         init
     },
     {
         let mut init =
             CutsceneCameraPoint{continueFlag: 0 as libc::c_int as s8,
                                 cameraRoll: 3 as libc::c_int as s8,
                                 nextPointFrame: 20 as libc::c_int as u16_0,
                                 viewAngle: 60.0f32,
                                 pos:
                                     {
                                         let mut init =
                                             Vec3s{x:
                                                       -(1836 as libc::c_int)
                                                           as s16,
                                                   y:
                                                       168 as libc::c_int as
                                                           s16,
                                                   z:
                                                       -(1243 as libc::c_int)
                                                           as s16,};
                                         init
                                     },};
         init
     },
     {
         let mut init =
             CutsceneCameraPoint{continueFlag: 0 as libc::c_int as s8,
                                 cameraRoll: 0 as libc::c_int as s8,
                                 nextPointFrame: 20 as libc::c_int as u16_0,
                                 viewAngle: 60.0f32,
                                 pos:
                                     {
                                         let mut init =
                                             Vec3s{x:
                                                       -(1905 as libc::c_int)
                                                           as s16,
                                                   y:
                                                       115 as libc::c_int as
                                                           s16,
                                                   z:
                                                       -(1193 as libc::c_int)
                                                           as s16,};
                                         init
                                     },};
         init
     },
     {
         let mut init =
             CutsceneCameraPoint{continueFlag: 0 as libc::c_int as s8,
                                 cameraRoll: 0 as libc::c_int as s8,
                                 nextPointFrame: 30 as libc::c_int as u16_0,
                                 viewAngle: 55.0f32,
                                 pos:
                                     {
                                         let mut init =
                                             Vec3s{x:
                                                       -(1969 as libc::c_int)
                                                           as s16,
                                                   y:
                                                       58 as libc::c_int as
                                                           s16,
                                                   z:
                                                       -(1212 as libc::c_int)
                                                           as s16,};
                                         init
                                     },};
         init
     },
     {
         let mut init =
             CutsceneCameraPoint{continueFlag: 0 as libc::c_int as s8,
                                 cameraRoll: 0 as libc::c_int as s8,
                                 nextPointFrame: 30 as libc::c_int as u16_0,
                                 viewAngle: 55.0f32,
                                 pos:
                                     {
                                         let mut init =
                                             Vec3s{x:
                                                       -(1969 as libc::c_int)
                                                           as s16,
                                                   y:
                                                       31 as libc::c_int as
                                                           s16,
                                                   z:
                                                       -(1164 as libc::c_int)
                                                           as s16,};
                                         init
                                     },};
         init
     },
     {
         let mut init =
             CutsceneCameraPoint{continueFlag: 0 as libc::c_int as s8,
                                 cameraRoll: 0 as libc::c_int as s8,
                                 nextPointFrame: 30 as libc::c_int as u16_0,
                                 viewAngle: 60.0f32,
                                 pos:
                                     {
                                         let mut init =
                                             Vec3s{x:
                                                       -(1969 as libc::c_int)
                                                           as s16,
                                                   y:
                                                       54 as libc::c_int as
                                                           s16,
                                                   z:
                                                       -(1209 as libc::c_int)
                                                           as s16,};
                                         init
                                     },};
         init
     },
     {
         let mut init =
             CutsceneCameraPoint{continueFlag: 0 as libc::c_int as s8,
                                 cameraRoll: 0 as libc::c_int as s8,
                                 nextPointFrame: 30 as libc::c_int as u16_0,
                                 viewAngle: 60.0f32,
                                 pos:
                                     {
                                         let mut init =
                                             Vec3s{x:
                                                       -(1973 as libc::c_int)
                                                           as s16,
                                                   y:
                                                       35 as libc::c_int as
                                                           s16,
                                                   z:
                                                       -(1206 as libc::c_int)
                                                           as s16,};
                                         init
                                     },};
         init
     },
     {
         let mut init =
             CutsceneCameraPoint{continueFlag: 0 as libc::c_int as s8,
                                 cameraRoll: 0 as libc::c_int as s8,
                                 nextPointFrame: 50 as libc::c_int as u16_0,
                                 viewAngle: 60.0f32,
                                 pos:
                                     {
                                         let mut init =
                                             Vec3s{x:
                                                       -(1974 as libc::c_int)
                                                           as s16,
                                                   y:
                                                       12 as libc::c_int as
                                                           s16,
                                                   z:
                                                       -(1179 as libc::c_int)
                                                           as s16,};
                                         init
                                     },};
         init
     },
     {
         let mut init =
             CutsceneCameraPoint{continueFlag: 0 as libc::c_int as s8,
                                 cameraRoll: 0 as libc::c_int as s8,
                                 nextPointFrame: 50 as libc::c_int as u16_0,
                                 viewAngle: 60.0f32,
                                 pos:
                                     {
                                         let mut init =
                                             Vec3s{x:
                                                       -(1974 as libc::c_int)
                                                           as s16,
                                                   y:
                                                       12 as libc::c_int as
                                                           s16,
                                                   z:
                                                       -(1179 as libc::c_int)
                                                           as s16,};
                                         init
                                     },};
         init
     },
     {
         let mut init =
             CutsceneCameraPoint{continueFlag: -(1 as libc::c_int) as s8,
                                 cameraRoll: 0 as libc::c_int as s8,
                                 nextPointFrame: 50 as libc::c_int as u16_0,
                                 viewAngle: 60.0f32,
                                 pos:
                                     {
                                         let mut init =
                                             Vec3s{x:
                                                       -(1974 as libc::c_int)
                                                           as s16,
                                                   y:
                                                       12 as libc::c_int as
                                                           s16,
                                                   z:
                                                       -(1179 as libc::c_int)
                                                           as s16,};
                                         init
                                     },};
         init
     },
     {
         let mut init =
             CutsceneCameraPoint{continueFlag: -(1 as libc::c_int) as s8,
                                 cameraRoll: 0 as libc::c_int as s8,
                                 nextPointFrame: 30 as libc::c_int as u16_0,
                                 viewAngle: 60.0f32,
                                 pos:
                                     {
                                         let mut init =
                                             Vec3s{x:
                                                       -(1974 as libc::c_int)
                                                           as s16,
                                                   y:
                                                       12 as libc::c_int as
                                                           s16,
                                                   z:
                                                       -(1179 as libc::c_int)
                                                           as s16,};
                                         init
                                     },};
         init
     }];
static mut D_8012021C: [CutsceneCameraPoint; 14] =
    [{
         let mut init =
             CutsceneCameraPoint{continueFlag: 0 as libc::c_int as s8,
                                 cameraRoll: 0 as libc::c_int as s8,
                                 nextPointFrame: 0 as libc::c_int as u16_0,
                                 viewAngle: 60.0f32,
                                 pos:
                                     {
                                         let mut init =
                                             Vec3s{x:
                                                       -(1751 as libc::c_int)
                                                           as s16,
                                                   y:
                                                       604 as libc::c_int as
                                                           s16,
                                                   z:
                                                       -(1233 as libc::c_int)
                                                           as s16,};
                                         init
                                     },};
         init
     },
     {
         let mut init =
             CutsceneCameraPoint{continueFlag: 0 as libc::c_int as s8,
                                 cameraRoll: 0 as libc::c_int as s8,
                                 nextPointFrame: 0 as libc::c_int as u16_0,
                                 viewAngle: 60.0f32,
                                 pos:
                                     {
                                         let mut init =
                                             Vec3s{x:
                                                       -(1752 as libc::c_int)
                                                           as s16,
                                                   y:
                                                       516 as libc::c_int as
                                                           s16,
                                                   z:
                                                       -(1233 as libc::c_int)
                                                           as s16,};
                                         init
                                     },};
         init
     },
     {
         let mut init =
             CutsceneCameraPoint{continueFlag: 0 as libc::c_int as s8,
                                 cameraRoll: 0 as libc::c_int as s8,
                                 nextPointFrame: 0 as libc::c_int as u16_0,
                                 viewAngle: 60.0f32,
                                 pos:
                                     {
                                         let mut init =
                                             Vec3s{x:
                                                       -(1751 as libc::c_int)
                                                           as s16,
                                                   y:
                                                       417 as libc::c_int as
                                                           s16,
                                                   z:
                                                       -(1233 as libc::c_int)
                                                           as s16,};
                                         init
                                     },};
         init
     },
     {
         let mut init =
             CutsceneCameraPoint{continueFlag: 0 as libc::c_int as s8,
                                 cameraRoll: 0 as libc::c_int as s8,
                                 nextPointFrame: 0 as libc::c_int as u16_0,
                                 viewAngle: 60.0f32,
                                 pos:
                                     {
                                         let mut init =
                                             Vec3s{x:
                                                       -(1767 as libc::c_int)
                                                           as s16,
                                                   y:
                                                       306 as libc::c_int as
                                                           s16,
                                                   z:
                                                       -(1219 as libc::c_int)
                                                           as s16,};
                                         init
                                     },};
         init
     },
     {
         let mut init =
             CutsceneCameraPoint{continueFlag: 0 as libc::c_int as s8,
                                 cameraRoll: 0 as libc::c_int as s8,
                                 nextPointFrame: 0 as libc::c_int as u16_0,
                                 viewAngle: 60.0f32,
                                 pos:
                                     {
                                         let mut init =
                                             Vec3s{x:
                                                       -(1776 as libc::c_int)
                                                           as s16,
                                                   y:
                                                       257 as libc::c_int as
                                                           s16,
                                                   z:
                                                       -(1205 as libc::c_int)
                                                           as s16,};
                                         init
                                     },};
         init
     },
     {
         let mut init =
             CutsceneCameraPoint{continueFlag: 0 as libc::c_int as s8,
                                 cameraRoll: 0 as libc::c_int as s8,
                                 nextPointFrame: 0 as libc::c_int as u16_0,
                                 viewAngle: 60.0f32,
                                 pos:
                                     {
                                         let mut init =
                                             Vec3s{x:
                                                       -(1881 as libc::c_int)
                                                           as s16,
                                                   y:
                                                       147 as libc::c_int as
                                                           s16,
                                                   z:
                                                       -(1149 as libc::c_int)
                                                           as s16,};
                                         init
                                     },};
         init
     },
     {
         let mut init =
             CutsceneCameraPoint{continueFlag: 0 as libc::c_int as s8,
                                 cameraRoll: 0 as libc::c_int as s8,
                                 nextPointFrame: 0 as libc::c_int as u16_0,
                                 viewAngle: 60.0f32,
                                 pos:
                                     {
                                         let mut init =
                                             Vec3s{x:
                                                       -(1969 as libc::c_int)
                                                           as s16,
                                                   y:
                                                       72 as libc::c_int as
                                                           s16,
                                                   z:
                                                       -(1077 as libc::c_int)
                                                           as s16,};
                                         init
                                     },};
         init
     },
     {
         let mut init =
             CutsceneCameraPoint{continueFlag: 0 as libc::c_int as s8,
                                 cameraRoll: 0 as libc::c_int as s8,
                                 nextPointFrame: 0 as libc::c_int as u16_0,
                                 viewAngle: 60.0f32,
                                 pos:
                                     {
                                         let mut init =
                                             Vec3s{x:
                                                       -(1969 as libc::c_int)
                                                           as s16,
                                                   y: 7 as libc::c_int as s16,
                                                   z:
                                                       -(1048 as libc::c_int)
                                                           as s16,};
                                         init
                                     },};
         init
     },
     {
         let mut init =
             CutsceneCameraPoint{continueFlag: 0 as libc::c_int as s8,
                                 cameraRoll: 0 as libc::c_int as s8,
                                 nextPointFrame: 0 as libc::c_int as u16_0,
                                 viewAngle: 60.0f32,
                                 pos:
                                     {
                                         let mut init =
                                             Vec3s{x:
                                                       -(1969 as libc::c_int)
                                                           as s16,
                                                   y: 1 as libc::c_int as s16,
                                                   z:
                                                       -(1030 as libc::c_int)
                                                           as s16,};
                                         init
                                     },};
         init
     },
     {
         let mut init =
             CutsceneCameraPoint{continueFlag: 0 as libc::c_int as s8,
                                 cameraRoll: 0 as libc::c_int as s8,
                                 nextPointFrame: 0 as libc::c_int as u16_0,
                                 viewAngle: 60.0f32,
                                 pos:
                                     {
                                         let mut init =
                                             Vec3s{x:
                                                       -(1987 as libc::c_int)
                                                           as s16,
                                                   y:
                                                       17 as libc::c_int as
                                                           s16,
                                                   z:
                                                       -(1076 as libc::c_int)
                                                           as s16,};
                                         init
                                     },};
         init
     },
     {
         let mut init =
             CutsceneCameraPoint{continueFlag: 0 as libc::c_int as s8,
                                 cameraRoll: 0 as libc::c_int as s8,
                                 nextPointFrame: 0 as libc::c_int as u16_0,
                                 viewAngle: 60.0f32,
                                 pos:
                                     {
                                         let mut init =
                                             Vec3s{x:
                                                       -(2007 as libc::c_int)
                                                           as s16,
                                                   y:
                                                       10 as libc::c_int as
                                                           s16,
                                                   z:
                                                       -(1004 as libc::c_int)
                                                           as s16,};
                                         init
                                     },};
         init
     },
     {
         let mut init =
             CutsceneCameraPoint{continueFlag: 0 as libc::c_int as s8,
                                 cameraRoll: 0 as libc::c_int as s8,
                                 nextPointFrame: 0 as libc::c_int as u16_0,
                                 viewAngle: 60.0f32,
                                 pos:
                                     {
                                         let mut init =
                                             Vec3s{x:
                                                       -(2007 as libc::c_int)
                                                           as s16,
                                                   y:
                                                       10 as libc::c_int as
                                                           s16,
                                                   z:
                                                       -(1004 as libc::c_int)
                                                           as s16,};
                                         init
                                     },};
         init
     },
     {
         let mut init =
             CutsceneCameraPoint{continueFlag: -(1 as libc::c_int) as s8,
                                 cameraRoll: 0 as libc::c_int as s8,
                                 nextPointFrame: 0 as libc::c_int as u16_0,
                                 viewAngle: 60.0f32,
                                 pos:
                                     {
                                         let mut init =
                                             Vec3s{x:
                                                       -(2007 as libc::c_int)
                                                           as s16,
                                                   y:
                                                       10 as libc::c_int as
                                                           s16,
                                                   z:
                                                       -(1004 as libc::c_int)
                                                           as s16,};
                                         init
                                     },};
         init
     },
     {
         let mut init =
             CutsceneCameraPoint{continueFlag: -(1 as libc::c_int) as s8,
                                 cameraRoll: 0 as libc::c_int as s8,
                                 nextPointFrame: 0 as libc::c_int as u16_0,
                                 viewAngle: 60.0f32,
                                 pos:
                                     {
                                         let mut init =
                                             Vec3s{x:
                                                       -(2007 as libc::c_int)
                                                           as s16,
                                                   y:
                                                       10 as libc::c_int as
                                                           s16,
                                                   z:
                                                       -(1004 as libc::c_int)
                                                           as s16,};
                                         init
                                     },};
         init
     }];
static mut D_801202FC: s16 = 13 as libc::c_int as s16;
static mut D_80120300: s16 = 210 as libc::c_int as s16;
static mut D_80120304: s16 = 0 as libc::c_int as s16;
static mut D_80120308: [CutsceneCameraPoint; 9] =
    [{
         let mut init =
             CutsceneCameraPoint{continueFlag: 0 as libc::c_int as s8,
                                 cameraRoll: 0 as libc::c_int as s8,
                                 nextPointFrame: 10 as libc::c_int as u16_0,
                                 viewAngle: 40.0f32,
                                 pos:
                                     {
                                         let mut init =
                                             Vec3s{x: 0 as libc::c_int as s16,
                                                   y: 4 as libc::c_int as s16,
                                                   z:
                                                       0 as libc::c_int as
                                                           s16,};
                                         init
                                     },};
         init
     },
     {
         let mut init =
             CutsceneCameraPoint{continueFlag: 0 as libc::c_int as s8,
                                 cameraRoll: 0 as libc::c_int as s8,
                                 nextPointFrame: 10 as libc::c_int as u16_0,
                                 viewAngle: 40.000004f32,
                                 pos:
                                     {
                                         let mut init =
                                             Vec3s{x: 0 as libc::c_int as s16,
                                                   y: 4 as libc::c_int as s16,
                                                   z:
                                                       0 as libc::c_int as
                                                           s16,};
                                         init
                                     },};
         init
     },
     {
         let mut init =
             CutsceneCameraPoint{continueFlag: 0 as libc::c_int as s8,
                                 cameraRoll: 0 as libc::c_int as s8,
                                 nextPointFrame: 10 as libc::c_int as u16_0,
                                 viewAngle: 50.0f32,
                                 pos:
                                     {
                                         let mut init =
                                             Vec3s{x: 0 as libc::c_int as s16,
                                                   y: 9 as libc::c_int as s16,
                                                   z:
                                                       0 as libc::c_int as
                                                           s16,};
                                         init
                                     },};
         init
     },
     {
         let mut init =
             CutsceneCameraPoint{continueFlag: 0 as libc::c_int as s8,
                                 cameraRoll: 0 as libc::c_int as s8,
                                 nextPointFrame: 12 as libc::c_int as u16_0,
                                 viewAngle: 55.0f32,
                                 pos:
                                     {
                                         let mut init =
                                             Vec3s{x: 0 as libc::c_int as s16,
                                                   y:
                                                       12 as libc::c_int as
                                                           s16,
                                                   z:
                                                       0 as libc::c_int as
                                                           s16,};
                                         init
                                     },};
         init
     },
     {
         let mut init =
             CutsceneCameraPoint{continueFlag: 0 as libc::c_int as s8,
                                 cameraRoll: 0 as libc::c_int as s8,
                                 nextPointFrame: 15 as libc::c_int as u16_0,
                                 viewAngle: 61.0f32,
                                 pos:
                                     {
                                         let mut init =
                                             Vec3s{x: 0 as libc::c_int as s16,
                                                   y:
                                                       18 as libc::c_int as
                                                           s16,
                                                   z:
                                                       0 as libc::c_int as
                                                           s16,};
                                         init
                                     },};
         init
     },
     {
         let mut init =
             CutsceneCameraPoint{continueFlag: 0 as libc::c_int as s8,
                                 cameraRoll: 0 as libc::c_int as s8,
                                 nextPointFrame: 20 as libc::c_int as u16_0,
                                 viewAngle: 65.0f32,
                                 pos:
                                     {
                                         let mut init =
                                             Vec3s{x: 0 as libc::c_int as s16,
                                                   y:
                                                       29 as libc::c_int as
                                                           s16,
                                                   z:
                                                       0 as libc::c_int as
                                                           s16,};
                                         init
                                     },};
         init
     },
     {
         let mut init =
             CutsceneCameraPoint{continueFlag: 0 as libc::c_int as s8,
                                 cameraRoll: 0 as libc::c_int as s8,
                                 nextPointFrame: 40 as libc::c_int as u16_0,
                                 viewAngle: 60.0f32,
                                 pos:
                                     {
                                         let mut init =
                                             Vec3s{x: 0 as libc::c_int as s16,
                                                   y:
                                                       34 as libc::c_int as
                                                           s16,
                                                   z:
                                                       0 as libc::c_int as
                                                           s16,};
                                         init
                                     },};
         init
     },
     {
         let mut init =
             CutsceneCameraPoint{continueFlag: -(1 as libc::c_int) as s8,
                                 cameraRoll: 0 as libc::c_int as s8,
                                 nextPointFrame: 40 as libc::c_int as u16_0,
                                 viewAngle: 60.0f32,
                                 pos:
                                     {
                                         let mut init =
                                             Vec3s{x: 0 as libc::c_int as s16,
                                                   y:
                                                       34 as libc::c_int as
                                                           s16,
                                                   z:
                                                       0 as libc::c_int as
                                                           s16,};
                                         init
                                     },};
         init
     },
     {
         let mut init =
             CutsceneCameraPoint{continueFlag: -(1 as libc::c_int) as s8,
                                 cameraRoll: 0 as libc::c_int as s8,
                                 nextPointFrame: 10 as libc::c_int as u16_0,
                                 viewAngle: 60.0f32,
                                 pos:
                                     {
                                         let mut init =
                                             Vec3s{x: 0 as libc::c_int as s16,
                                                   y:
                                                       34 as libc::c_int as
                                                           s16,
                                                   z:
                                                       0 as libc::c_int as
                                                           s16,};
                                         init
                                     },};
         init
     }];
static mut D_80120398: [CutsceneCameraPoint; 9] =
    [{
         let mut init =
             CutsceneCameraPoint{continueFlag: 0 as libc::c_int as s8,
                                 cameraRoll: 0 as libc::c_int as s8,
                                 nextPointFrame: 0 as libc::c_int as u16_0,
                                 viewAngle: 60.0f32,
                                 pos:
                                     {
                                         let mut init =
                                             Vec3s{x: 0 as libc::c_int as s16,
                                                   y: 9 as libc::c_int as s16,
                                                   z:
                                                       45 as libc::c_int as
                                                           s16,};
                                         init
                                     },};
         init
     },
     {
         let mut init =
             CutsceneCameraPoint{continueFlag: 0 as libc::c_int as s8,
                                 cameraRoll: 0 as libc::c_int as s8,
                                 nextPointFrame: 0 as libc::c_int as u16_0,
                                 viewAngle: 60.0f32,
                                 pos:
                                     {
                                         let mut init =
                                             Vec3s{x: 0 as libc::c_int as s16,
                                                   y: 8 as libc::c_int as s16,
                                                   z:
                                                       50 as libc::c_int as
                                                           s16,};
                                         init
                                     },};
         init
     },
     {
         let mut init =
             CutsceneCameraPoint{continueFlag: 0 as libc::c_int as s8,
                                 cameraRoll: 0 as libc::c_int as s8,
                                 nextPointFrame: 0 as libc::c_int as u16_0,
                                 viewAngle: 60.0f32,
                                 pos:
                                     {
                                         let mut init =
                                             Vec3s{x: 0 as libc::c_int as s16,
                                                   y:
                                                       17 as libc::c_int as
                                                           s16,
                                                   z:
                                                       58 as libc::c_int as
                                                           s16,};
                                         init
                                     },};
         init
     },
     {
         let mut init =
             CutsceneCameraPoint{continueFlag: 0 as libc::c_int as s8,
                                 cameraRoll: 0 as libc::c_int as s8,
                                 nextPointFrame: 0 as libc::c_int as u16_0,
                                 viewAngle: 60.0f32,
                                 pos:
                                     {
                                         let mut init =
                                             Vec3s{x: 0 as libc::c_int as s16,
                                                   y:
                                                       21 as libc::c_int as
                                                           s16,
                                                   z:
                                                       78 as libc::c_int as
                                                           s16,};
                                         init
                                     },};
         init
     },
     {
         let mut init =
             CutsceneCameraPoint{continueFlag: 0 as libc::c_int as s8,
                                 cameraRoll: 0 as libc::c_int as s8,
                                 nextPointFrame: 0 as libc::c_int as u16_0,
                                 viewAngle: 60.0f32,
                                 pos:
                                     {
                                         let mut init =
                                             Vec3s{x: 0 as libc::c_int as s16,
                                                   y:
                                                       46 as libc::c_int as
                                                           s16,
                                                   z:
                                                       109 as libc::c_int as
                                                           s16,};
                                         init
                                     },};
         init
     },
     {
         let mut init =
             CutsceneCameraPoint{continueFlag: 0 as libc::c_int as s8,
                                 cameraRoll: 0 as libc::c_int as s8,
                                 nextPointFrame: 0 as libc::c_int as u16_0,
                                 viewAngle: 60.0f32,
                                 pos:
                                     {
                                         let mut init =
                                             Vec3s{x: 0 as libc::c_int as s16,
                                                   y:
                                                       58 as libc::c_int as
                                                           s16,
                                                   z:
                                                       118 as libc::c_int as
                                                           s16,};
                                         init
                                     },};
         init
     },
     {
         let mut init =
             CutsceneCameraPoint{continueFlag: 0 as libc::c_int as s8,
                                 cameraRoll: 0 as libc::c_int as s8,
                                 nextPointFrame: 0 as libc::c_int as u16_0,
                                 viewAngle: 60.0f32,
                                 pos:
                                     {
                                         let mut init =
                                             Vec3s{x: 0 as libc::c_int as s16,
                                                   y:
                                                       63 as libc::c_int as
                                                           s16,
                                                   z:
                                                       119 as libc::c_int as
                                                           s16,};
                                         init
                                     },};
         init
     },
     {
         let mut init =
             CutsceneCameraPoint{continueFlag: -(1 as libc::c_int) as s8,
                                 cameraRoll: 0 as libc::c_int as s8,
                                 nextPointFrame: 0 as libc::c_int as u16_0,
                                 viewAngle: 60.0f32,
                                 pos:
                                     {
                                         let mut init =
                                             Vec3s{x: 0 as libc::c_int as s16,
                                                   y:
                                                       62 as libc::c_int as
                                                           s16,
                                                   z:
                                                       119 as libc::c_int as
                                                           s16,};
                                         init
                                     },};
         init
     },
     {
         let mut init =
             CutsceneCameraPoint{continueFlag: -(1 as libc::c_int) as s8,
                                 cameraRoll: 0 as libc::c_int as s8,
                                 nextPointFrame: 0 as libc::c_int as u16_0,
                                 viewAngle: 60.0f32,
                                 pos:
                                     {
                                         let mut init =
                                             Vec3s{x: 0 as libc::c_int as s16,
                                                   y:
                                                       62 as libc::c_int as
                                                           s16,
                                                   z:
                                                       119 as libc::c_int as
                                                           s16,};
                                         init
                                     },};
         init
     }];
static mut D_8012042C: s16 = 90 as libc::c_int as s16;
static mut D_80120430: s16 = 1 as libc::c_int as s16;
static mut D_80120434: [CutsceneCameraPoint; 10] =
    [{
         let mut init =
             CutsceneCameraPoint{continueFlag: 0 as libc::c_int as s8,
                                 cameraRoll: 0 as libc::c_int as s8,
                                 nextPointFrame: 0 as libc::c_int as u16_0,
                                 viewAngle: 60.0f32,
                                 pos:
                                     {
                                         let mut init =
                                             Vec3s{x: 0 as libc::c_int as s16,
                                                   y: 9 as libc::c_int as s16,
                                                   z:
                                                       -(45 as libc::c_int) as
                                                           s16,};
                                         init
                                     },};
         init
     },
     {
         let mut init =
             CutsceneCameraPoint{continueFlag: 0 as libc::c_int as s8,
                                 cameraRoll: 0 as libc::c_int as s8,
                                 nextPointFrame: 0 as libc::c_int as u16_0,
                                 viewAngle: 60.0f32,
                                 pos:
                                     {
                                         let mut init =
                                             Vec3s{x: 0 as libc::c_int as s16,
                                                   y: 9 as libc::c_int as s16,
                                                   z:
                                                       -(45 as libc::c_int) as
                                                           s16,};
                                         init
                                     },};
         init
     },
     {
         let mut init =
             CutsceneCameraPoint{continueFlag: 0 as libc::c_int as s8,
                                 cameraRoll: 0 as libc::c_int as s8,
                                 nextPointFrame: 0 as libc::c_int as u16_0,
                                 viewAngle: 60.0f32,
                                 pos:
                                     {
                                         let mut init =
                                             Vec3s{x: 0 as libc::c_int as s16,
                                                   y: 8 as libc::c_int as s16,
                                                   z:
                                                       -(50 as libc::c_int) as
                                                           s16,};
                                         init
                                     },};
         init
     },
     {
         let mut init =
             CutsceneCameraPoint{continueFlag: 0 as libc::c_int as s8,
                                 cameraRoll: 0 as libc::c_int as s8,
                                 nextPointFrame: 0 as libc::c_int as u16_0,
                                 viewAngle: 60.0f32,
                                 pos:
                                     {
                                         let mut init =
                                             Vec3s{x: 0 as libc::c_int as s16,
                                                   y:
                                                       17 as libc::c_int as
                                                           s16,
                                                   z:
                                                       -(58 as libc::c_int) as
                                                           s16,};
                                         init
                                     },};
         init
     },
     {
         let mut init =
             CutsceneCameraPoint{continueFlag: 0 as libc::c_int as s8,
                                 cameraRoll: 0 as libc::c_int as s8,
                                 nextPointFrame: 0 as libc::c_int as u16_0,
                                 viewAngle: 60.0f32,
                                 pos:
                                     {
                                         let mut init =
                                             Vec3s{x: 0 as libc::c_int as s16,
                                                   y:
                                                       21 as libc::c_int as
                                                           s16,
                                                   z:
                                                       -(78 as libc::c_int) as
                                                           s16,};
                                         init
                                     },};
         init
     },
     {
         let mut init =
             CutsceneCameraPoint{continueFlag: 0 as libc::c_int as s8,
                                 cameraRoll: 0 as libc::c_int as s8,
                                 nextPointFrame: 0 as libc::c_int as u16_0,
                                 viewAngle: 60.0f32,
                                 pos:
                                     {
                                         let mut init =
                                             Vec3s{x: 0 as libc::c_int as s16,
                                                   y:
                                                       46 as libc::c_int as
                                                           s16,
                                                   z:
                                                       -(109 as libc::c_int)
                                                           as s16,};
                                         init
                                     },};
         init
     },
     {
         let mut init =
             CutsceneCameraPoint{continueFlag: 0 as libc::c_int as s8,
                                 cameraRoll: 0 as libc::c_int as s8,
                                 nextPointFrame: 0 as libc::c_int as u16_0,
                                 viewAngle: 60.0f32,
                                 pos:
                                     {
                                         let mut init =
                                             Vec3s{x: 0 as libc::c_int as s16,
                                                   y:
                                                       58 as libc::c_int as
                                                           s16,
                                                   z:
                                                       -(118 as libc::c_int)
                                                           as s16,};
                                         init
                                     },};
         init
     },
     {
         let mut init =
             CutsceneCameraPoint{continueFlag: 0 as libc::c_int as s8,
                                 cameraRoll: 0 as libc::c_int as s8,
                                 nextPointFrame: 0 as libc::c_int as u16_0,
                                 viewAngle: 60.0f32,
                                 pos:
                                     {
                                         let mut init =
                                             Vec3s{x: 0 as libc::c_int as s16,
                                                   y:
                                                       63 as libc::c_int as
                                                           s16,
                                                   z:
                                                       -(119 as libc::c_int)
                                                           as s16,};
                                         init
                                     },};
         init
     },
     {
         let mut init =
             CutsceneCameraPoint{continueFlag: -(1 as libc::c_int) as s8,
                                 cameraRoll: 0 as libc::c_int as s8,
                                 nextPointFrame: 0 as libc::c_int as u16_0,
                                 viewAngle: 60.0f32,
                                 pos:
                                     {
                                         let mut init =
                                             Vec3s{x: 0 as libc::c_int as s16,
                                                   y:
                                                       62 as libc::c_int as
                                                           s16,
                                                   z:
                                                       -(119 as libc::c_int)
                                                           as s16,};
                                         init
                                     },};
         init
     },
     {
         let mut init =
             CutsceneCameraPoint{continueFlag: -(1 as libc::c_int) as s8,
                                 cameraRoll: 0 as libc::c_int as s8,
                                 nextPointFrame: 0 as libc::c_int as u16_0,
                                 viewAngle: 60.0f32,
                                 pos:
                                     {
                                         let mut init =
                                             Vec3s{x: 0 as libc::c_int as s16,
                                                   y:
                                                       62 as libc::c_int as
                                                           s16,
                                                   z:
                                                       -(119 as libc::c_int)
                                                           as s16,};
                                         init
                                     },};
         init
     }];
static mut D_801204D4: [CutsceneCameraPoint; 14] =
    [{
         let mut init =
             CutsceneCameraPoint{continueFlag: 0 as libc::c_int as s8,
                                 cameraRoll: -(15 as libc::c_int) as s8,
                                 nextPointFrame: 40 as libc::c_int as u16_0,
                                 viewAngle: 80.600006f32,
                                 pos:
                                     {
                                         let mut init =
                                             Vec3s{x:
                                                       -(60 as libc::c_int) as
                                                           s16,
                                                   y:
                                                       332 as libc::c_int as
                                                           s16,
                                                   z:
                                                       183 as libc::c_int as
                                                           s16,};
                                         init
                                     },};
         init
     },
     {
         let mut init =
             CutsceneCameraPoint{continueFlag: 0 as libc::c_int as s8,
                                 cameraRoll: -(22 as libc::c_int) as s8,
                                 nextPointFrame: 30 as libc::c_int as u16_0,
                                 viewAngle: 80.600006f32,
                                 pos:
                                     {
                                         let mut init =
                                             Vec3s{x:
                                                       -(60 as libc::c_int) as
                                                           s16,
                                                   y:
                                                       332 as libc::c_int as
                                                           s16,
                                                   z:
                                                       183 as libc::c_int as
                                                           s16,};
                                         init
                                     },};
         init
     },
     {
         let mut init =
             CutsceneCameraPoint{continueFlag: 0 as libc::c_int as s8,
                                 cameraRoll: -(20 as libc::c_int) as s8,
                                 nextPointFrame: 38 as libc::c_int as u16_0,
                                 viewAngle: 80.600006f32,
                                 pos:
                                     {
                                         let mut init =
                                             Vec3s{x:
                                                       -(118 as libc::c_int)
                                                           as s16,
                                                   y:
                                                       344 as libc::c_int as
                                                           s16,
                                                   z:
                                                       41 as libc::c_int as
                                                           s16,};
                                         init
                                     },};
         init
     },
     {
         let mut init =
             CutsceneCameraPoint{continueFlag: 0 as libc::c_int as s8,
                                 cameraRoll: -(18 as libc::c_int) as s8,
                                 nextPointFrame: 32 as libc::c_int as u16_0,
                                 viewAngle: 80.600006f32,
                                 pos:
                                     {
                                         let mut init =
                                             Vec3s{x:
                                                       -(80 as libc::c_int) as
                                                           s16,
                                                   y:
                                                       251 as libc::c_int as
                                                           s16,
                                                   z:
                                                       -(8 as libc::c_int) as
                                                           s16,};
                                         init
                                     },};
         init
     },
     {
         let mut init =
             CutsceneCameraPoint{continueFlag: 0 as libc::c_int as s8,
                                 cameraRoll: -(12 as libc::c_int) as s8,
                                 nextPointFrame: 28 as libc::c_int as u16_0,
                                 viewAngle: 80.600006f32,
                                 pos:
                                     {
                                         let mut init =
                                             Vec3s{x:
                                                       -(64 as libc::c_int) as
                                                           s16,
                                                   y:
                                                       259 as libc::c_int as
                                                           s16,
                                                   z:
                                                       -(28 as libc::c_int) as
                                                           s16,};
                                         init
                                     },};
         init
     },
     {
         let mut init =
             CutsceneCameraPoint{continueFlag: 0 as libc::c_int as s8,
                                 cameraRoll: -(8 as libc::c_int) as s8,
                                 nextPointFrame: 22 as libc::c_int as u16_0,
                                 viewAngle: 80.600006f32,
                                 pos:
                                     {
                                         let mut init =
                                             Vec3s{x:
                                                       -(79 as libc::c_int) as
                                                           s16,
                                                   y:
                                                       200 as libc::c_int as
                                                           s16,
                                                   z:
                                                       -(342 as libc::c_int)
                                                           as s16,};
                                         init
                                     },};
         init
     },
     {
         let mut init =
             CutsceneCameraPoint{continueFlag: 0 as libc::c_int as s8,
                                 cameraRoll: -(5 as libc::c_int) as s8,
                                 nextPointFrame: 10 as libc::c_int as u16_0,
                                 viewAngle: 65.80005f32,
                                 pos:
                                     {
                                         let mut init =
                                             Vec3s{x:
                                                       -(110 as libc::c_int)
                                                           as s16,
                                                   y:
                                                       140 as libc::c_int as
                                                           s16,
                                                   z:
                                                       -(549 as libc::c_int)
                                                           as s16,};
                                         init
                                     },};
         init
     },
     {
         let mut init =
             CutsceneCameraPoint{continueFlag: 0 as libc::c_int as s8,
                                 cameraRoll: -(2 as libc::c_int) as s8,
                                 nextPointFrame: 8 as libc::c_int as u16_0,
                                 viewAngle: 65.2f32,
                                 pos:
                                     {
                                         let mut init =
                                             Vec3s{x:
                                                       -(74 as libc::c_int) as
                                                           s16,
                                                   y:
                                                       109 as libc::c_int as
                                                           s16,
                                                   z:
                                                       -(507 as libc::c_int)
                                                           as s16,};
                                         init
                                     },};
         init
     },
     {
         let mut init =
             CutsceneCameraPoint{continueFlag: 0 as libc::c_int as s8,
                                 cameraRoll: 0 as libc::c_int as s8,
                                 nextPointFrame: 10 as libc::c_int as u16_0,
                                 viewAngle: 65.80002f32,
                                 pos:
                                     {
                                         let mut init =
                                             Vec3s{x:
                                                       -(32 as libc::c_int) as
                                                           s16,
                                                   y:
                                                       78 as libc::c_int as
                                                           s16,
                                                   z:
                                                       -(680 as libc::c_int)
                                                           as s16,};
                                         init
                                     },};
         init
     },
     {
         let mut init =
             CutsceneCameraPoint{continueFlag: 0 as libc::c_int as s8,
                                 cameraRoll: 0 as libc::c_int as s8,
                                 nextPointFrame: 20 as libc::c_int as u16_0,
                                 viewAngle: 85.199936f32,
                                 pos:
                                     {
                                         let mut init =
                                             Vec3s{x:
                                                       25 as libc::c_int as
                                                           s16,
                                                   y:
                                                       127 as libc::c_int as
                                                           s16,
                                                   z:
                                                       -(950 as libc::c_int)
                                                           as s16,};
                                         init
                                     },};
         init
     },
     {
         let mut init =
             CutsceneCameraPoint{continueFlag: 0 as libc::c_int as s8,
                                 cameraRoll: 0 as libc::c_int as s8,
                                 nextPointFrame: 30 as libc::c_int as u16_0,
                                 viewAngle: 85.199936f32,
                                 pos:
                                     {
                                         let mut init =
                                             Vec3s{x:
                                                       25 as libc::c_int as
                                                           s16,
                                                   y:
                                                       127 as libc::c_int as
                                                           s16,
                                                   z:
                                                       -(950 as libc::c_int)
                                                           as s16,};
                                         init
                                     },};
         init
     },
     {
         let mut init =
             CutsceneCameraPoint{continueFlag: 0 as libc::c_int as s8,
                                 cameraRoll: 0 as libc::c_int as s8,
                                 nextPointFrame: 40 as libc::c_int as u16_0,
                                 viewAngle: 85.199936f32,
                                 pos:
                                     {
                                         let mut init =
                                             Vec3s{x:
                                                       25 as libc::c_int as
                                                           s16,
                                                   y:
                                                       127 as libc::c_int as
                                                           s16,
                                                   z:
                                                       -(950 as libc::c_int)
                                                           as s16,};
                                         init
                                     },};
         init
     },
     {
         let mut init =
             CutsceneCameraPoint{continueFlag: -(1 as libc::c_int) as s8,
                                 cameraRoll: 6 as libc::c_int as s8,
                                 nextPointFrame: 30 as libc::c_int as u16_0,
                                 viewAngle: 85.199936f32,
                                 pos:
                                     {
                                         let mut init =
                                             Vec3s{x:
                                                       25 as libc::c_int as
                                                           s16,
                                                   y:
                                                       127 as libc::c_int as
                                                           s16,
                                                   z:
                                                       -(950 as libc::c_int)
                                                           as s16,};
                                         init
                                     },};
         init
     },
     {
         let mut init =
             CutsceneCameraPoint{continueFlag: -(1 as libc::c_int) as s8,
                                 cameraRoll: 0 as libc::c_int as s8,
                                 nextPointFrame: 30 as libc::c_int as u16_0,
                                 viewAngle: 85.199936f32,
                                 pos:
                                     {
                                         let mut init =
                                             Vec3s{x:
                                                       25 as libc::c_int as
                                                           s16,
                                                   y:
                                                       127 as libc::c_int as
                                                           s16,
                                                   z:
                                                       -(950 as libc::c_int)
                                                           as s16,};
                                         init
                                     },};
         init
     }];
static mut D_801205B4: [CutsceneCameraPoint; 14] =
    [{
         let mut init =
             CutsceneCameraPoint{continueFlag: 0 as libc::c_int as s8,
                                 cameraRoll: 0 as libc::c_int as s8,
                                 nextPointFrame: 0 as libc::c_int as u16_0,
                                 viewAngle: 60.0f32,
                                 pos:
                                     {
                                         let mut init =
                                             Vec3s{x:
                                                       -(225 as libc::c_int)
                                                           as s16,
                                                   y:
                                                       785 as libc::c_int as
                                                           s16,
                                                   z:
                                                       -(242 as libc::c_int)
                                                           as s16,};
                                         init
                                     },};
         init
     },
     {
         let mut init =
             CutsceneCameraPoint{continueFlag: 0 as libc::c_int as s8,
                                 cameraRoll: -(21 as libc::c_int) as s8,
                                 nextPointFrame: 0 as libc::c_int as u16_0,
                                 viewAngle: 80.600006f32,
                                 pos:
                                     {
                                         let mut init =
                                             Vec3s{x:
                                                       -(245 as libc::c_int)
                                                           as s16,
                                                   y:
                                                       784 as libc::c_int as
                                                           s16,
                                                   z:
                                                       -(242 as libc::c_int)
                                                           as s16,};
                                         init
                                     },};
         init
     },
     {
         let mut init =
             CutsceneCameraPoint{continueFlag: 0 as libc::c_int as s8,
                                 cameraRoll: -(21 as libc::c_int) as s8,
                                 nextPointFrame: 0 as libc::c_int as u16_0,
                                 viewAngle: 80.600006f32,
                                 pos:
                                     {
                                         let mut init =
                                             Vec3s{x:
                                                       -(288 as libc::c_int)
                                                           as s16,
                                                   y:
                                                       485 as libc::c_int as
                                                           s16,
                                                   z:
                                                       -(379 as libc::c_int)
                                                           as s16,};
                                         init
                                     },};
         init
     },
     {
         let mut init =
             CutsceneCameraPoint{continueFlag: 0 as libc::c_int as s8,
                                 cameraRoll: -(21 as libc::c_int) as s8,
                                 nextPointFrame: 0 as libc::c_int as u16_0,
                                 viewAngle: 80.600006f32,
                                 pos:
                                     {
                                         let mut init =
                                             Vec3s{x:
                                                       -(250 as libc::c_int)
                                                           as s16,
                                                   y:
                                                       244 as libc::c_int as
                                                           s16,
                                                   z:
                                                       -(442 as libc::c_int)
                                                           as s16,};
                                         init
                                     },};
         init
     },
     {
         let mut init =
             CutsceneCameraPoint{continueFlag: 0 as libc::c_int as s8,
                                 cameraRoll: -(21 as libc::c_int) as s8,
                                 nextPointFrame: 0 as libc::c_int as u16_0,
                                 viewAngle: 80.600006f32,
                                 pos:
                                     {
                                         let mut init =
                                             Vec3s{x:
                                                       -(163 as libc::c_int)
                                                           as s16,
                                                   y:
                                                       21 as libc::c_int as
                                                           s16,
                                                   z:
                                                       -(415 as libc::c_int)
                                                           as s16,};
                                         init
                                     },};
         init
     },
     {
         let mut init =
             CutsceneCameraPoint{continueFlag: 0 as libc::c_int as s8,
                                 cameraRoll: -(21 as libc::c_int) as s8,
                                 nextPointFrame: 0 as libc::c_int as u16_0,
                                 viewAngle: 80.600006f32,
                                 pos:
                                     {
                                         let mut init =
                                             Vec3s{x:
                                                       -(98 as libc::c_int) as
                                                           s16,
                                                   y:
                                                       86 as libc::c_int as
                                                           s16,
                                                   z:
                                                       -(520 as libc::c_int)
                                                           as s16,};
                                         init
                                     },};
         init
     },
     {
         let mut init =
             CutsceneCameraPoint{continueFlag: 0 as libc::c_int as s8,
                                 cameraRoll: -(21 as libc::c_int) as s8,
                                 nextPointFrame: 0 as libc::c_int as u16_0,
                                 viewAngle: 80.600006f32,
                                 pos:
                                     {
                                         let mut init =
                                             Vec3s{x:
                                                       -(86 as libc::c_int) as
                                                           s16,
                                                   y:
                                                       31 as libc::c_int as
                                                           s16,
                                                   z:
                                                       -(816 as libc::c_int)
                                                           as s16,};
                                         init
                                     },};
         init
     },
     {
         let mut init =
             CutsceneCameraPoint{continueFlag: 0 as libc::c_int as s8,
                                 cameraRoll: -(21 as libc::c_int) as s8,
                                 nextPointFrame: 0 as libc::c_int as u16_0,
                                 viewAngle: 80.600006f32,
                                 pos:
                                     {
                                         let mut init =
                                             Vec3s{x:
                                                       -(74 as libc::c_int) as
                                                           s16,
                                                   y:
                                                       18 as libc::c_int as
                                                           s16,
                                                   z:
                                                       -(931 as libc::c_int)
                                                           as s16,};
                                         init
                                     },};
         init
     },
     {
         let mut init =
             CutsceneCameraPoint{continueFlag: 0 as libc::c_int as s8,
                                 cameraRoll: 1 as libc::c_int as s8,
                                 nextPointFrame: 0 as libc::c_int as u16_0,
                                 viewAngle: 80.600006f32,
                                 pos:
                                     {
                                         let mut init =
                                             Vec3s{x:
                                                       -(91 as libc::c_int) as
                                                           s16,
                                                   y:
                                                       80 as libc::c_int as
                                                           s16,
                                                   z:
                                                       -(1220 as libc::c_int)
                                                           as s16,};
                                         init
                                     },};
         init
     },
     {
         let mut init =
             CutsceneCameraPoint{continueFlag: 0 as libc::c_int as s8,
                                 cameraRoll: 0 as libc::c_int as s8,
                                 nextPointFrame: 0 as libc::c_int as u16_0,
                                 viewAngle: 85.199936f32,
                                 pos:
                                     {
                                         let mut init =
                                             Vec3s{x:
                                                       14 as libc::c_int as
                                                           s16,
                                                   y:
                                                       153 as libc::c_int as
                                                           s16,
                                                   z:
                                                       -(1340 as libc::c_int)
                                                           as s16,};
                                         init
                                     },};
         init
     },
     {
         let mut init =
             CutsceneCameraPoint{continueFlag: 0 as libc::c_int as s8,
                                 cameraRoll: 0 as libc::c_int as s8,
                                 nextPointFrame: 0 as libc::c_int as u16_0,
                                 viewAngle: 85.199936f32,
                                 pos:
                                     {
                                         let mut init =
                                             Vec3s{x:
                                                       28 as libc::c_int as
                                                           s16,
                                                   y:
                                                       125 as libc::c_int as
                                                           s16,
                                                   z:
                                                       -(1340 as libc::c_int)
                                                           as s16,};
                                         init
                                     },};
         init
     },
     {
         let mut init =
             CutsceneCameraPoint{continueFlag: 0 as libc::c_int as s8,
                                 cameraRoll: 0 as libc::c_int as s8,
                                 nextPointFrame: 0 as libc::c_int as u16_0,
                                 viewAngle: 85.199936f32,
                                 pos:
                                     {
                                         let mut init =
                                             Vec3s{x:
                                                       48 as libc::c_int as
                                                           s16,
                                                   y:
                                                       124 as libc::c_int as
                                                           s16,
                                                   z:
                                                       -(1340 as libc::c_int)
                                                           as s16,};
                                         init
                                     },};
         init
     },
     {
         let mut init =
             CutsceneCameraPoint{continueFlag: -(1 as libc::c_int) as s8,
                                 cameraRoll: 0 as libc::c_int as s8,
                                 nextPointFrame: 0 as libc::c_int as u16_0,
                                 viewAngle: 85.199936f32,
                                 pos:
                                     {
                                         let mut init =
                                             Vec3s{x:
                                                       48 as libc::c_int as
                                                           s16,
                                                   y:
                                                       124 as libc::c_int as
                                                           s16,
                                                   z:
                                                       -(1502 as libc::c_int)
                                                           as s16,};
                                         init
                                     },};
         init
     },
     {
         let mut init =
             CutsceneCameraPoint{continueFlag: -(1 as libc::c_int) as s8,
                                 cameraRoll: 0 as libc::c_int as s8,
                                 nextPointFrame: 0 as libc::c_int as u16_0,
                                 viewAngle: 85.199936f32,
                                 pos:
                                     {
                                         let mut init =
                                             Vec3s{x:
                                                       48 as libc::c_int as
                                                           s16,
                                                   y:
                                                       124 as libc::c_int as
                                                           s16,
                                                   z:
                                                       -(1262 as libc::c_int)
                                                           as s16,};
                                         init
                                     },};
         init
     }];
static mut D_80120694: s16 = 14 as libc::c_int as s16;
static mut D_80120698: s16 = 190 as libc::c_int as s16;
static mut D_8012069C: s16 = 8 as libc::c_int as s16;
static mut D_801206A0: [CutsceneCameraPoint; 12] =
    [{
         let mut init =
             CutsceneCameraPoint{continueFlag: 0 as libc::c_int as s8,
                                 cameraRoll: 6 as libc::c_int as s8,
                                 nextPointFrame: 20 as libc::c_int as u16_0,
                                 viewAngle: 80.0f32,
                                 pos:
                                     {
                                         let mut init =
                                             Vec3s{x:
                                                       -(96 as libc::c_int) as
                                                           s16,
                                                   y:
                                                       40 as libc::c_int as
                                                           s16,
                                                   z:
                                                       170 as libc::c_int as
                                                           s16,};
                                         init
                                     },};
         init
     },
     {
         let mut init =
             CutsceneCameraPoint{continueFlag: 0 as libc::c_int as s8,
                                 cameraRoll: 6 as libc::c_int as s8,
                                 nextPointFrame: 20 as libc::c_int as u16_0,
                                 viewAngle: 80.0f32,
                                 pos:
                                     {
                                         let mut init =
                                             Vec3s{x:
                                                       -(96 as libc::c_int) as
                                                           s16,
                                                   y:
                                                       40 as libc::c_int as
                                                           s16,
                                                   z:
                                                       170 as libc::c_int as
                                                           s16,};
                                         init
                                     },};
         init
     },
     {
         let mut init =
             CutsceneCameraPoint{continueFlag: 0 as libc::c_int as s8,
                                 cameraRoll: 6 as libc::c_int as s8,
                                 nextPointFrame: 20 as libc::c_int as u16_0,
                                 viewAngle: 70.0f32,
                                 pos:
                                     {
                                         let mut init =
                                             Vec3s{x:
                                                       -(70 as libc::c_int) as
                                                           s16,
                                                   y:
                                                       35 as libc::c_int as
                                                           s16,
                                                   z:
                                                       150 as libc::c_int as
                                                           s16,};
                                         init
                                     },};
         init
     },
     {
         let mut init =
             CutsceneCameraPoint{continueFlag: 0 as libc::c_int as s8,
                                 cameraRoll: 5 as libc::c_int as s8,
                                 nextPointFrame: 10 as libc::c_int as u16_0,
                                 viewAngle: 60.0f32,
                                 pos:
                                     {
                                         let mut init =
                                             Vec3s{x:
                                                       -(57 as libc::c_int) as
                                                           s16,
                                                   y:
                                                       34 as libc::c_int as
                                                           s16,
                                                   z:
                                                       133 as libc::c_int as
                                                           s16,};
                                         init
                                     },};
         init
     },
     {
         let mut init =
             CutsceneCameraPoint{continueFlag: 0 as libc::c_int as s8,
                                 cameraRoll: 4 as libc::c_int as s8,
                                 nextPointFrame: 25 as libc::c_int as u16_0,
                                 viewAngle: 65.0f32,
                                 pos:
                                     {
                                         let mut init =
                                             Vec3s{x:
                                                       -(22 as libc::c_int) as
                                                           s16,
                                                   y:
                                                       32 as libc::c_int as
                                                           s16,
                                                   z:
                                                       110 as libc::c_int as
                                                           s16,};
                                         init
                                     },};
         init
     },
     {
         let mut init =
             CutsceneCameraPoint{continueFlag: 0 as libc::c_int as s8,
                                 cameraRoll: 3 as libc::c_int as s8,
                                 nextPointFrame: 12 as libc::c_int as u16_0,
                                 viewAngle: 60.0f32,
                                 pos:
                                     {
                                         let mut init =
                                             Vec3s{x:
                                                       -(9 as libc::c_int) as
                                                           s16,
                                                   y:
                                                       33 as libc::c_int as
                                                           s16,
                                                   z:
                                                       98 as libc::c_int as
                                                           s16,};
                                         init
                                     },};
         init
     },
     {
         let mut init =
             CutsceneCameraPoint{continueFlag: 0 as libc::c_int as s8,
                                 cameraRoll: 3 as libc::c_int as s8,
                                 nextPointFrame: 5 as libc::c_int as u16_0,
                                 viewAngle: 65.0f32,
                                 pos:
                                     {
                                         let mut init =
                                             Vec3s{x:
                                                       -(3 as libc::c_int) as
                                                           s16,
                                                   y:
                                                       29 as libc::c_int as
                                                           s16,
                                                   z:
                                                       87 as libc::c_int as
                                                           s16,};
                                         init
                                     },};
         init
     },
     {
         let mut init =
             CutsceneCameraPoint{continueFlag: 0 as libc::c_int as s8,
                                 cameraRoll: 2 as libc::c_int as s8,
                                 nextPointFrame: 10 as libc::c_int as u16_0,
                                 viewAngle: 65.0f32,
                                 pos:
                                     {
                                         let mut init =
                                             Vec3s{x:
                                                       -(1 as libc::c_int) as
                                                           s16,
                                                   y:
                                                       15 as libc::c_int as
                                                           s16,
                                                   z:
                                                       84 as libc::c_int as
                                                           s16,};
                                         init
                                     },};
         init
     },
     {
         let mut init =
             CutsceneCameraPoint{continueFlag: 0 as libc::c_int as s8,
                                 cameraRoll: 1 as libc::c_int as s8,
                                 nextPointFrame: 200 as libc::c_int as u16_0,
                                 viewAngle: 65.0f32,
                                 pos:
                                     {
                                         let mut init =
                                             Vec3s{x: 0 as libc::c_int as s16,
                                                   y:
                                                       17 as libc::c_int as
                                                           s16,
                                                   z:
                                                       82 as libc::c_int as
                                                           s16,};
                                         init
                                     },};
         init
     },
     {
         let mut init =
             CutsceneCameraPoint{continueFlag: 0 as libc::c_int as s8,
                                 cameraRoll: 1 as libc::c_int as s8,
                                 nextPointFrame: 500 as libc::c_int as u16_0,
                                 viewAngle: 65.0f32,
                                 pos:
                                     {
                                         let mut init =
                                             Vec3s{x: 0 as libc::c_int as s16,
                                                   y:
                                                       18 as libc::c_int as
                                                           s16,
                                                   z:
                                                       82 as libc::c_int as
                                                           s16,};
                                         init
                                     },};
         init
     },
     {
         let mut init =
             CutsceneCameraPoint{continueFlag: -(1 as libc::c_int) as s8,
                                 cameraRoll: 8 as libc::c_int as s8,
                                 nextPointFrame: 50 as libc::c_int as u16_0,
                                 viewAngle: 65.0f32,
                                 pos:
                                     {
                                         let mut init =
                                             Vec3s{x: 0 as libc::c_int as s16,
                                                   y:
                                                       18 as libc::c_int as
                                                           s16,
                                                   z:
                                                       82 as libc::c_int as
                                                           s16,};
                                         init
                                     },};
         init
     },
     {
         let mut init =
             CutsceneCameraPoint{continueFlag: -(1 as libc::c_int) as s8,
                                 cameraRoll: 11 as libc::c_int as s8,
                                 nextPointFrame: 60 as libc::c_int as u16_0,
                                 viewAngle: 65.0f32,
                                 pos:
                                     {
                                         let mut init =
                                             Vec3s{x: 0 as libc::c_int as s16,
                                                   y:
                                                       18 as libc::c_int as
                                                           s16,
                                                   z:
                                                       82 as libc::c_int as
                                                           s16,};
                                         init
                                     },};
         init
     }];
static mut D_80120760: [CutsceneCameraPoint; 12] =
    [{
         let mut init =
             CutsceneCameraPoint{continueFlag: 0 as libc::c_int as s8,
                                 cameraRoll: 6 as libc::c_int as s8,
                                 nextPointFrame: 0 as libc::c_int as u16_0,
                                 viewAngle: 80.0f32,
                                 pos:
                                     {
                                         let mut init =
                                             Vec3s{x:
                                                       -(50 as libc::c_int) as
                                                           s16,
                                                   y:
                                                       10 as libc::c_int as
                                                           s16,
                                                   z:
                                                       180 as libc::c_int as
                                                           s16,};
                                         init
                                     },};
         init
     },
     {
         let mut init =
             CutsceneCameraPoint{continueFlag: 0 as libc::c_int as s8,
                                 cameraRoll: 6 as libc::c_int as s8,
                                 nextPointFrame: 0 as libc::c_int as u16_0,
                                 viewAngle: 80.0f32,
                                 pos:
                                     {
                                         let mut init =
                                             Vec3s{x:
                                                       -(50 as libc::c_int) as
                                                           s16,
                                                   y:
                                                       20 as libc::c_int as
                                                           s16,
                                                   z:
                                                       180 as libc::c_int as
                                                           s16,};
                                         init
                                     },};
         init
     },
     {
         let mut init =
             CutsceneCameraPoint{continueFlag: 0 as libc::c_int as s8,
                                 cameraRoll: 6 as libc::c_int as s8,
                                 nextPointFrame: 0 as libc::c_int as u16_0,
                                 viewAngle: 70.0f32,
                                 pos:
                                     {
                                         let mut init =
                                             Vec3s{x:
                                                       -(40 as libc::c_int) as
                                                           s16,
                                                   y:
                                                       30 as libc::c_int as
                                                           s16,
                                                   z:
                                                       177 as libc::c_int as
                                                           s16,};
                                         init
                                     },};
         init
     },
     {
         let mut init =
             CutsceneCameraPoint{continueFlag: 0 as libc::c_int as s8,
                                 cameraRoll: 5 as libc::c_int as s8,
                                 nextPointFrame: 0 as libc::c_int as u16_0,
                                 viewAngle: 65.0f32,
                                 pos:
                                     {
                                         let mut init =
                                             Vec3s{x: 0 as libc::c_int as s16,
                                                   y:
                                                       35 as libc::c_int as
                                                           s16,
                                                   z:
                                                       172 as libc::c_int as
                                                           s16,};
                                         init
                                     },};
         init
     },
     {
         let mut init =
             CutsceneCameraPoint{continueFlag: 0 as libc::c_int as s8,
                                 cameraRoll: 4 as libc::c_int as s8,
                                 nextPointFrame: 0 as libc::c_int as u16_0,
                                 viewAngle: 65.0f32,
                                 pos:
                                     {
                                         let mut init =
                                             Vec3s{x:
                                                       34 as libc::c_int as
                                                           s16,
                                                   y:
                                                       35 as libc::c_int as
                                                           s16,
                                                   z:
                                                       162 as libc::c_int as
                                                           s16,};
                                         init
                                     },};
         init
     },
     {
         let mut init =
             CutsceneCameraPoint{continueFlag: 0 as libc::c_int as s8,
                                 cameraRoll: 3 as libc::c_int as s8,
                                 nextPointFrame: 0 as libc::c_int as u16_0,
                                 viewAngle: 65.0f32,
                                 pos:
                                     {
                                         let mut init =
                                             Vec3s{x:
                                                       61 as libc::c_int as
                                                           s16,
                                                   y:
                                                       32 as libc::c_int as
                                                           s16,
                                                   z:
                                                       147 as libc::c_int as
                                                           s16,};
                                         init
                                     },};
         init
     },
     {
         let mut init =
             CutsceneCameraPoint{continueFlag: 0 as libc::c_int as s8,
                                 cameraRoll: 3 as libc::c_int as s8,
                                 nextPointFrame: 0 as libc::c_int as u16_0,
                                 viewAngle: 65.0f32,
                                 pos:
                                     {
                                         let mut init =
                                             Vec3s{x:
                                                       72 as libc::c_int as
                                                           s16,
                                                   y:
                                                       30 as libc::c_int as
                                                           s16,
                                                   z:
                                                       128 as libc::c_int as
                                                           s16,};
                                         init
                                     },};
         init
     },
     {
         let mut init =
             CutsceneCameraPoint{continueFlag: 0 as libc::c_int as s8,
                                 cameraRoll: 2 as libc::c_int as s8,
                                 nextPointFrame: 0 as libc::c_int as u16_0,
                                 viewAngle: 65.0f32,
                                 pos:
                                     {
                                         let mut init =
                                             Vec3s{x:
                                                       74 as libc::c_int as
                                                           s16,
                                                   y:
                                                       20 as libc::c_int as
                                                           s16,
                                                   z:
                                                       125 as libc::c_int as
                                                           s16,};
                                         init
                                     },};
         init
     },
     {
         let mut init =
             CutsceneCameraPoint{continueFlag: 0 as libc::c_int as s8,
                                 cameraRoll: 1 as libc::c_int as s8,
                                 nextPointFrame: 0 as libc::c_int as u16_0,
                                 viewAngle: 65.0f32,
                                 pos:
                                     {
                                         let mut init =
                                             Vec3s{x:
                                                       75 as libc::c_int as
                                                           s16,
                                                   y:
                                                       18 as libc::c_int as
                                                           s16,
                                                   z:
                                                       123 as libc::c_int as
                                                           s16,};
                                         init
                                     },};
         init
     },
     {
         let mut init =
             CutsceneCameraPoint{continueFlag: 0 as libc::c_int as s8,
                                 cameraRoll: 1 as libc::c_int as s8,
                                 nextPointFrame: 0 as libc::c_int as u16_0,
                                 viewAngle: 65.0f32,
                                 pos:
                                     {
                                         let mut init =
                                             Vec3s{x:
                                                       75 as libc::c_int as
                                                           s16,
                                                   y:
                                                       10 as libc::c_int as
                                                           s16,
                                                   z:
                                                       123 as libc::c_int as
                                                           s16,};
                                         init
                                     },};
         init
     },
     {
         let mut init =
             CutsceneCameraPoint{continueFlag: -(1 as libc::c_int) as s8,
                                 cameraRoll: 0 as libc::c_int as s8,
                                 nextPointFrame: 0 as libc::c_int as u16_0,
                                 viewAngle: 65.0f32,
                                 pos:
                                     {
                                         let mut init =
                                             Vec3s{x:
                                                       75 as libc::c_int as
                                                           s16,
                                                   y:
                                                       10 as libc::c_int as
                                                           s16,
                                                   z:
                                                       122 as libc::c_int as
                                                           s16,};
                                         init
                                     },};
         init
     },
     {
         let mut init =
             CutsceneCameraPoint{continueFlag: -(1 as libc::c_int) as s8,
                                 cameraRoll: 0 as libc::c_int as s8,
                                 nextPointFrame: 0 as libc::c_int as u16_0,
                                 viewAngle: 65.0f32,
                                 pos:
                                     {
                                         let mut init =
                                             Vec3s{x:
                                                       75 as libc::c_int as
                                                           s16,
                                                   y:
                                                       10 as libc::c_int as
                                                           s16,
                                                   z:
                                                       122 as libc::c_int as
                                                           s16,};
                                         init
                                     },};
         init
     }];
static mut D_80120820: [CutsceneCameraPoint; 12] =
    [{
         let mut init =
             CutsceneCameraPoint{continueFlag: 0 as libc::c_int as s8,
                                 cameraRoll: 6 as libc::c_int as s8,
                                 nextPointFrame: 0 as libc::c_int as u16_0,
                                 viewAngle: 80.0f32,
                                 pos:
                                     {
                                         let mut init =
                                             Vec3s{x:
                                                       85 as libc::c_int as
                                                           s16,
                                                   y: 5 as libc::c_int as s16,
                                                   z:
                                                       170 as libc::c_int as
                                                           s16,};
                                         init
                                     },};
         init
     },
     {
         let mut init =
             CutsceneCameraPoint{continueFlag: 0 as libc::c_int as s8,
                                 cameraRoll: 6 as libc::c_int as s8,
                                 nextPointFrame: 0 as libc::c_int as u16_0,
                                 viewAngle: 80.0f32,
                                 pos:
                                     {
                                         let mut init =
                                             Vec3s{x:
                                                       85 as libc::c_int as
                                                           s16,
                                                   y:
                                                       10 as libc::c_int as
                                                           s16,
                                                   z:
                                                       170 as libc::c_int as
                                                           s16,};
                                         init
                                     },};
         init
     },
     {
         let mut init =
             CutsceneCameraPoint{continueFlag: 0 as libc::c_int as s8,
                                 cameraRoll: 6 as libc::c_int as s8,
                                 nextPointFrame: 0 as libc::c_int as u16_0,
                                 viewAngle: 70.0f32,
                                 pos:
                                     {
                                         let mut init =
                                             Vec3s{x:
                                                       80 as libc::c_int as
                                                           s16,
                                                   y:
                                                       20 as libc::c_int as
                                                           s16,
                                                   z:
                                                       167 as libc::c_int as
                                                           s16,};
                                         init
                                     },};
         init
     },
     {
         let mut init =
             CutsceneCameraPoint{continueFlag: 0 as libc::c_int as s8,
                                 cameraRoll: 5 as libc::c_int as s8,
                                 nextPointFrame: 0 as libc::c_int as u16_0,
                                 viewAngle: 65.0f32,
                                 pos:
                                     {
                                         let mut init =
                                             Vec3s{x:
                                                       74 as libc::c_int as
                                                           s16,
                                                   y:
                                                       25 as libc::c_int as
                                                           s16,
                                                   z:
                                                       165 as libc::c_int as
                                                           s16,};
                                         init
                                     },};
         init
     },
     {
         let mut init =
             CutsceneCameraPoint{continueFlag: 0 as libc::c_int as s8,
                                 cameraRoll: 4 as libc::c_int as s8,
                                 nextPointFrame: 0 as libc::c_int as u16_0,
                                 viewAngle: 65.0f32,
                                 pos:
                                     {
                                         let mut init =
                                             Vec3s{x:
                                                       63 as libc::c_int as
                                                           s16,
                                                   y:
                                                       30 as libc::c_int as
                                                           s16,
                                                   z:
                                                       162 as libc::c_int as
                                                           s16,};
                                         init
                                     },};
         init
     },
     {
         let mut init =
             CutsceneCameraPoint{continueFlag: 0 as libc::c_int as s8,
                                 cameraRoll: 3 as libc::c_int as s8,
                                 nextPointFrame: 0 as libc::c_int as u16_0,
                                 viewAngle: 65.0f32,
                                 pos:
                                     {
                                         let mut init =
                                             Vec3s{x:
                                                       66 as libc::c_int as
                                                           s16,
                                                   y:
                                                       34 as libc::c_int as
                                                           s16,
                                                   z:
                                                       147 as libc::c_int as
                                                           s16,};
                                         init
                                     },};
         init
     },
     {
         let mut init =
             CutsceneCameraPoint{continueFlag: 0 as libc::c_int as s8,
                                 cameraRoll: 3 as libc::c_int as s8,
                                 nextPointFrame: 0 as libc::c_int as u16_0,
                                 viewAngle: 65.0f32,
                                 pos:
                                     {
                                         let mut init =
                                             Vec3s{x:
                                                       72 as libc::c_int as
                                                           s16,
                                                   y:
                                                       34 as libc::c_int as
                                                           s16,
                                                   z:
                                                       128 as libc::c_int as
                                                           s16,};
                                         init
                                     },};
         init
     },
     {
         let mut init =
             CutsceneCameraPoint{continueFlag: 0 as libc::c_int as s8,
                                 cameraRoll: 2 as libc::c_int as s8,
                                 nextPointFrame: 0 as libc::c_int as u16_0,
                                 viewAngle: 65.0f32,
                                 pos:
                                     {
                                         let mut init =
                                             Vec3s{x:
                                                       74 as libc::c_int as
                                                           s16,
                                                   y:
                                                       20 as libc::c_int as
                                                           s16,
                                                   z:
                                                       125 as libc::c_int as
                                                           s16,};
                                         init
                                     },};
         init
     },
     {
         let mut init =
             CutsceneCameraPoint{continueFlag: 0 as libc::c_int as s8,
                                 cameraRoll: 1 as libc::c_int as s8,
                                 nextPointFrame: 0 as libc::c_int as u16_0,
                                 viewAngle: 65.0f32,
                                 pos:
                                     {
                                         let mut init =
                                             Vec3s{x:
                                                       75 as libc::c_int as
                                                           s16,
                                                   y:
                                                       18 as libc::c_int as
                                                           s16,
                                                   z:
                                                       123 as libc::c_int as
                                                           s16,};
                                         init
                                     },};
         init
     },
     {
         let mut init =
             CutsceneCameraPoint{continueFlag: 0 as libc::c_int as s8,
                                 cameraRoll: 1 as libc::c_int as s8,
                                 nextPointFrame: 0 as libc::c_int as u16_0,
                                 viewAngle: 65.0f32,
                                 pos:
                                     {
                                         let mut init =
                                             Vec3s{x:
                                                       75 as libc::c_int as
                                                           s16,
                                                   y:
                                                       10 as libc::c_int as
                                                           s16,
                                                   z:
                                                       123 as libc::c_int as
                                                           s16,};
                                         init
                                     },};
         init
     },
     {
         let mut init =
             CutsceneCameraPoint{continueFlag: -(1 as libc::c_int) as s8,
                                 cameraRoll: 0 as libc::c_int as s8,
                                 nextPointFrame: 0 as libc::c_int as u16_0,
                                 viewAngle: 65.0f32,
                                 pos:
                                     {
                                         let mut init =
                                             Vec3s{x:
                                                       75 as libc::c_int as
                                                           s16,
                                                   y:
                                                       10 as libc::c_int as
                                                           s16,
                                                   z:
                                                       122 as libc::c_int as
                                                           s16,};
                                         init
                                     },};
         init
     },
     {
         let mut init =
             CutsceneCameraPoint{continueFlag: -(1 as libc::c_int) as s8,
                                 cameraRoll: 0 as libc::c_int as s8,
                                 nextPointFrame: 0 as libc::c_int as u16_0,
                                 viewAngle: 65.0f32,
                                 pos:
                                     {
                                         let mut init =
                                             Vec3s{x:
                                                       75 as libc::c_int as
                                                           s16,
                                                   y:
                                                       10 as libc::c_int as
                                                           s16,
                                                   z:
                                                       122 as libc::c_int as
                                                           s16,};
                                         init
                                     },};
         init
     }];
static mut D_801208E4: s16 = 90 as libc::c_int as s16;
static mut D_801208E8: s16 = 8 as libc::c_int as s16;
static mut D_801208EC: [OnePointCsFull; 3] =
    [{
         let mut init =
             OnePointCsFull{actionFlags: 0xf as libc::c_int as u8_0,
                            unk_01: 0x8 as libc::c_int as u8_0,
                            initFlags: 0x101 as libc::c_int as s16,
                            timerInit: 1 as libc::c_int as s16,
                            rollTargetInit: 0 as libc::c_int as s16,
                            fovTargetInit: 60.0f32,
                            lerpStepScale: 1.0f32,
                            atTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: 0.0f32,
                                              y: 0.0f32,
                                              z: 0.0f32,};
                                    init
                                },
                            eyeTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: 0.0f32,
                                              y: 0.0f32,
                                              z: 0.0f32,};
                                    init
                                },};
         init
     },
     {
         let mut init =
             OnePointCsFull{actionFlags: 0x81 as libc::c_int as u8_0,
                            unk_01: 0xff as libc::c_int as u8_0,
                            initFlags: 0x101 as libc::c_int as s16,
                            timerInit: 1 as libc::c_int as s16,
                            rollTargetInit: 0 as libc::c_int as s16,
                            fovTargetInit: 60.0f32,
                            lerpStepScale: 1.0f32,
                            atTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: 0.0f32,
                                              y: -10.0f32,
                                              z: 0.0f32,};
                                    init
                                },
                            eyeTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: 0.0f32,
                                              y: 0.0f32,
                                              z: 150.0f32,};
                                    init
                                },};
         init
     },
     {
         let mut init =
             OnePointCsFull{actionFlags: 0x12 as libc::c_int as u8_0,
                            unk_01: 0xff as libc::c_int as u8_0,
                            initFlags: 0 as libc::c_int as s16,
                            timerInit: 1 as libc::c_int as s16,
                            rollTargetInit: 0 as libc::c_int as s16,
                            fovTargetInit: 60.0f32,
                            lerpStepScale: 1.0f32,
                            atTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: -1.0f32,
                                              y: -1.0f32,
                                              z: -1.0f32,};
                                    init
                                },
                            eyeTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: -1.0f32,
                                              y: -1.0f32,
                                              z: -1.0f32,};
                                    init
                                },};
         init
     }];
static mut D_80120964: [OnePointCsFull; 2] =
    [{
         let mut init =
             OnePointCsFull{actionFlags: 0x8f as libc::c_int as u8_0,
                            unk_01: 0xff as libc::c_int as u8_0,
                            initFlags: 0x101 as libc::c_int as s16,
                            timerInit: 1 as libc::c_int as s16,
                            rollTargetInit: 0 as libc::c_int as s16,
                            fovTargetInit: 60.0f32,
                            lerpStepScale: 1.0f32,
                            atTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: 0.0f32,
                                              y: 0.0f32,
                                              z: 0.0f32,};
                                    init
                                },
                            eyeTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: 0.0f32,
                                              y: 0.0f32,
                                              z: 0.0f32,};
                                    init
                                },};
         init
     },
     {
         let mut init =
             OnePointCsFull{actionFlags: 0x81 as libc::c_int as u8_0,
                            unk_01: 0xff as libc::c_int as u8_0,
                            initFlags: 0xa121 as libc::c_int as s16,
                            timerInit: 1 as libc::c_int as s16,
                            rollTargetInit: 0 as libc::c_int as s16,
                            fovTargetInit: 75.0f32,
                            lerpStepScale: 0.6f32,
                            atTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: 0.0f32,
                                              y: -10.0f32,
                                              z: 0.0f32,};
                                    init
                                },
                            eyeTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: 0.0f32,
                                              y: 0.0f32,
                                              z: 150.0f32,};
                                    init
                                },};
         init
     }];
static mut D_801209B4: [OnePointCsFull; 4] =
    [{
         let mut init =
             OnePointCsFull{actionFlags: 0x8f as libc::c_int as u8_0,
                            unk_01: 0x8 as libc::c_int as u8_0,
                            initFlags: 0x101 as libc::c_int as s16,
                            timerInit: 1 as libc::c_int as s16,
                            rollTargetInit: 0 as libc::c_int as s16,
                            fovTargetInit: 60.0f32,
                            lerpStepScale: 0.9f32,
                            atTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: 0.0f32,
                                              y: 0.0f32,
                                              z: 0.0f32,};
                                    init
                                },
                            eyeTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: 0.0f32,
                                              y: 0.0f32,
                                              z: 0.0f32,};
                                    init
                                },};
         init
     },
     {
         let mut init =
             OnePointCsFull{actionFlags: 0x84 as libc::c_int as u8_0,
                            unk_01: 0x1 as libc::c_int as u8_0,
                            initFlags: 0x100 as libc::c_int as s16,
                            timerInit: 29 as libc::c_int as s16,
                            rollTargetInit: 0 as libc::c_int as s16,
                            fovTargetInit: 45.0f32,
                            lerpStepScale: 0.1f32,
                            atTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: 0.0f32,
                                              y: -10.0f32,
                                              z: 0.0f32,};
                                    init
                                },
                            eyeTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: 0.0f32,
                                              y: 0.0f32,
                                              z: 150.0f32,};
                                    init
                                },};
         init
     },
     {
         let mut init =
             OnePointCsFull{actionFlags: 0x83 as libc::c_int as u8_0,
                            unk_01: 0xff as libc::c_int as u8_0,
                            initFlags: 0 as libc::c_int as s16,
                            timerInit: 10 as libc::c_int as s16,
                            rollTargetInit: 0 as libc::c_int as s16,
                            fovTargetInit: 60.0f32,
                            lerpStepScale: 0.2f32,
                            atTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: 0.0f32,
                                              y: -10.0f32,
                                              z: 0.0f32,};
                                    init
                                },
                            eyeTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: 0.0f32,
                                              y: 0.0f32,
                                              z: 150.0f32,};
                                    init
                                },};
         init
     },
     {
         let mut init =
             OnePointCsFull{actionFlags: 0x12 as libc::c_int as u8_0,
                            unk_01: 0xff as libc::c_int as u8_0,
                            initFlags: 0 as libc::c_int as s16,
                            timerInit: 1 as libc::c_int as s16,
                            rollTargetInit: 0 as libc::c_int as s16,
                            fovTargetInit: 60.0f32,
                            lerpStepScale: 1.0f32,
                            atTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: -1.0f32,
                                              y: -1.0f32,
                                              z: -1.0f32,};
                                    init
                                },
                            eyeTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: -1.0f32,
                                              y: -1.0f32,
                                              z: -1.0f32,};
                                    init
                                },};
         init
     }];
static mut D_80120A54: [OnePointCsFull; 3] =
    [{
         let mut init =
             OnePointCsFull{actionFlags: 0x8f as libc::c_int as u8_0,
                            unk_01: 0xff as libc::c_int as u8_0,
                            initFlags: 0x2525 as libc::c_int as s16,
                            timerInit: 1 as libc::c_int as s16,
                            rollTargetInit: 0 as libc::c_int as s16,
                            fovTargetInit: 75.0f32,
                            lerpStepScale: 0.1f32,
                            atTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: 0.0f32,
                                              y: 20.0f32,
                                              z: -10.0f32,};
                                    init
                                },
                            eyeTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: 0.0f32,
                                              y: 10.0f32,
                                              z: -40.0f32,};
                                    init
                                },};
         init
     },
     {
         let mut init =
             OnePointCsFull{actionFlags: 0x8f as libc::c_int as u8_0,
                            unk_01: 0xff as libc::c_int as u8_0,
                            initFlags: 0 as libc::c_int as s16,
                            timerInit: 9 as libc::c_int as s16,
                            rollTargetInit: 0 as libc::c_int as s16,
                            fovTargetInit: 60.0f32,
                            lerpStepScale: 1.0f32,
                            atTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: -1.0f32,
                                              y: -1.0f32,
                                              z: -1.0f32,};
                                    init
                                },
                            eyeTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: -1.0f32,
                                              y: -1.0f32,
                                              z: -1.0f32,};
                                    init
                                },};
         init
     },
     {
         let mut init =
             OnePointCsFull{actionFlags: 0x8b as libc::c_int as u8_0,
                            unk_01: 0xff as libc::c_int as u8_0,
                            initFlags: 0x22 as libc::c_int as s16,
                            timerInit: 5000 as libc::c_int as s16,
                            rollTargetInit: 0 as libc::c_int as s16,
                            fovTargetInit: 75.0f32,
                            lerpStepScale: 0.005f32,
                            atTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: 0.0f32,
                                              y: 0.0f32,
                                              z: -10.0f32,};
                                    init
                                },
                            eyeTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: -1.0f32,
                                              y: -1.0f32,
                                              z: -1.0f32,};
                                    init
                                },};
         init
     }];
static mut D_80120ACC: [OnePointCsFull; 5] =
    [{
         let mut init =
             OnePointCsFull{actionFlags: 0x8f as libc::c_int as u8_0,
                            unk_01: 0xff as libc::c_int as u8_0,
                            initFlags: 0x442 as libc::c_int as s16,
                            timerInit: 10 as libc::c_int as s16,
                            rollTargetInit: 0 as libc::c_int as s16,
                            fovTargetInit: 40.0f32,
                            lerpStepScale: 1.0f32,
                            atTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: -10.0f32,
                                              y: 45.0f32,
                                              z: 20.0f32,};
                                    init
                                },
                            eyeTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: 20.0f32,
                                              y: 30.0f32,
                                              z: 160.0f32,};
                                    init
                                },};
         init
     },
     {
         let mut init =
             OnePointCsFull{actionFlags: 0x95 as libc::c_int as u8_0,
                            unk_01: 0xff as libc::c_int as u8_0,
                            initFlags: 0 as libc::c_int as s16,
                            timerInit: 1 as libc::c_int as s16,
                            rollTargetInit: 0 as libc::c_int as s16,
                            fovTargetInit: 40.0f32,
                            lerpStepScale: 1.0f32,
                            atTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: -1.0f32,
                                              y: -1.0f32,
                                              z: -1.0f32,};
                                    init
                                },
                            eyeTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: -1.0f32,
                                              y: -1.0f32,
                                              z: -1.0f32,};
                                    init
                                },};
         init
     },
     {
         let mut init =
             OnePointCsFull{actionFlags: 0x8f as libc::c_int as u8_0,
                            unk_01: 0x1 as libc::c_int as u8_0,
                            initFlags: 0x442 as libc::c_int as s16,
                            timerInit: 10 as libc::c_int as s16,
                            rollTargetInit: 0 as libc::c_int as s16,
                            fovTargetInit: 40.0f32,
                            lerpStepScale: 1.0f32,
                            atTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: -10.0f32,
                                              y: 45.0f32,
                                              z: 20.0f32,};
                                    init
                                },
                            eyeTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: 20.0f32,
                                              y: 30.0f32,
                                              z: 160.0f32,};
                                    init
                                },};
         init
     },
     {
         let mut init =
             OnePointCsFull{actionFlags: 0x95 as libc::c_int as u8_0,
                            unk_01: 0xff as libc::c_int as u8_0,
                            initFlags: 0 as libc::c_int as s16,
                            timerInit: 1 as libc::c_int as s16,
                            rollTargetInit: 0 as libc::c_int as s16,
                            fovTargetInit: 40.0f32,
                            lerpStepScale: 1.0f32,
                            atTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: -1.0f32,
                                              y: -1.0f32,
                                              z: -1.0f32,};
                                    init
                                },
                            eyeTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: -1.0f32,
                                              y: -1.0f32,
                                              z: -1.0f32,};
                                    init
                                },};
         init
     },
     {
         let mut init =
             OnePointCsFull{actionFlags: 0x11 as libc::c_int as u8_0,
                            unk_01: 0xff as libc::c_int as u8_0,
                            initFlags: 0 as libc::c_int as s16,
                            timerInit: 1 as libc::c_int as s16,
                            rollTargetInit: 0 as libc::c_int as s16,
                            fovTargetInit: 60.0f32,
                            lerpStepScale: 1.0f32,
                            atTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: -1.0f32,
                                              y: -1.0f32,
                                              z: -1.0f32,};
                                    init
                                },
                            eyeTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: -1.0f32,
                                              y: -1.0f32,
                                              z: -1.0f32,};
                                    init
                                },};
         init
     }];
static mut D_80120B94: [OnePointCsFull; 11] =
    [{
         let mut init =
             OnePointCsFull{actionFlags: 0x8f as libc::c_int as u8_0,
                            unk_01: 0x1 as libc::c_int as u8_0,
                            initFlags: 0x2142 as libc::c_int as s16,
                            timerInit: 1 as libc::c_int as s16,
                            rollTargetInit: 0 as libc::c_int as s16,
                            fovTargetInit: 40.0f32,
                            lerpStepScale: 1.0f32,
                            atTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: 20.0f32,
                                              y: 40.0f32,
                                              z: 20.0f32,};
                                    init
                                },
                            eyeTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: -20.0f32,
                                              y: 0.0f32,
                                              z: -30.0f32,};
                                    init
                                },};
         init
     },
     {
         let mut init =
             OnePointCsFull{actionFlags: 0x84 as libc::c_int as u8_0,
                            unk_01: 0xff as libc::c_int as u8_0,
                            initFlags: 0x404 as libc::c_int as s16,
                            timerInit: 19 as libc::c_int as s16,
                            rollTargetInit: 5 as libc::c_int as s16,
                            fovTargetInit: 70.0f32,
                            lerpStepScale: 0.01f32,
                            atTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: 0.0f32,
                                              y: 30.0f32,
                                              z: 20.0f32,};
                                    init
                                },
                            eyeTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: 120.0f32,
                                              y: 60.0f32,
                                              z: 120.0f32,};
                                    init
                                },};
         init
     },
     {
         let mut init =
             OnePointCsFull{actionFlags: 0x84 as libc::c_int as u8_0,
                            unk_01: 0xff as libc::c_int as u8_0,
                            initFlags: 0x404 as libc::c_int as s16,
                            timerInit: 20 as libc::c_int as s16,
                            rollTargetInit: 0 as libc::c_int as s16,
                            fovTargetInit: 60.0f32,
                            lerpStepScale: 0.01f32,
                            atTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: 0.0f32,
                                              y: 20.0f32,
                                              z: 20.0f32,};
                                    init
                                },
                            eyeTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: 120.0f32,
                                              y: 60.0f32,
                                              z: 120.0f32,};
                                    init
                                },};
         init
     },
     {
         let mut init =
             OnePointCsFull{actionFlags: 0x84 as libc::c_int as u8_0,
                            unk_01: 0xff as libc::c_int as u8_0,
                            initFlags: 0x404 as libc::c_int as s16,
                            timerInit: 40 as libc::c_int as s16,
                            rollTargetInit: -(10 as libc::c_int) as s16,
                            fovTargetInit: 50.0f32,
                            lerpStepScale: 0.02f32,
                            atTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: 0.0f32,
                                              y: 30.0f32,
                                              z: 20.0f32,};
                                    init
                                },
                            eyeTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: 120.0f32,
                                              y: 60.0f32,
                                              z: 120.0f32,};
                                    init
                                },};
         init
     },
     {
         let mut init =
             OnePointCsFull{actionFlags: 0x8f as libc::c_int as u8_0,
                            unk_01: 0xff as libc::c_int as u8_0,
                            initFlags: 0x4141 as libc::c_int as s16,
                            timerInit: 1 as libc::c_int as s16,
                            rollTargetInit: 0 as libc::c_int as s16,
                            fovTargetInit: 40.0f32,
                            lerpStepScale: 1.0f32,
                            atTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: 0.0f32,
                                              y: -10.0f32,
                                              z: 20.0f32,};
                                    init
                                },
                            eyeTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: 0.0f32,
                                              y: 20.0f32,
                                              z: 50.0f32,};
                                    init
                                },};
         init
     },
     {
         let mut init =
             OnePointCsFull{actionFlags: 0x84 as libc::c_int as u8_0,
                            unk_01: 0xff as libc::c_int as u8_0,
                            initFlags: 0x404 as libc::c_int as s16,
                            timerInit: 19 as libc::c_int as s16,
                            rollTargetInit: 0 as libc::c_int as s16,
                            fovTargetInit: 70.0f32,
                            lerpStepScale: 0.01f32,
                            atTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: 0.0f32,
                                              y: 30.0f32,
                                              z: 20.0f32,};
                                    init
                                },
                            eyeTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: 120.0f32,
                                              y: 60.0f32,
                                              z: 120.0f32,};
                                    init
                                },};
         init
     },
     {
         let mut init =
             OnePointCsFull{actionFlags: 0x84 as libc::c_int as u8_0,
                            unk_01: 0xff as libc::c_int as u8_0,
                            initFlags: 0x404 as libc::c_int as s16,
                            timerInit: 40 as libc::c_int as s16,
                            rollTargetInit: 10 as libc::c_int as s16,
                            fovTargetInit: 50.0f32,
                            lerpStepScale: 0.01f32,
                            atTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: 0.0f32,
                                              y: 20.0f32,
                                              z: 20.0f32,};
                                    init
                                },
                            eyeTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: 120.0f32,
                                              y: 60.0f32,
                                              z: 120.0f32,};
                                    init
                                },};
         init
     },
     {
         let mut init =
             OnePointCsFull{actionFlags: 0x84 as libc::c_int as u8_0,
                            unk_01: 0xff as libc::c_int as u8_0,
                            initFlags: 0x404 as libc::c_int as s16,
                            timerInit: 70 as libc::c_int as s16,
                            rollTargetInit: 0 as libc::c_int as s16,
                            fovTargetInit: 60.0f32,
                            lerpStepScale: 0.01f32,
                            atTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: 0.0f32,
                                              y: 30.0f32,
                                              z: 20.0f32,};
                                    init
                                },
                            eyeTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: 120.0f32,
                                              y: 60.0f32,
                                              z: 120.0f32,};
                                    init
                                },};
         init
     },
     {
         let mut init =
             OnePointCsFull{actionFlags: 0x8f as libc::c_int as u8_0,
                            unk_01: 0xff as libc::c_int as u8_0,
                            initFlags: 0x4141 as libc::c_int as s16,
                            timerInit: 1 as libc::c_int as s16,
                            rollTargetInit: 0 as libc::c_int as s16,
                            fovTargetInit: 50.0f32,
                            lerpStepScale: 1.0f32,
                            atTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: 0.0f32,
                                              y: -10.0f32,
                                              z: 0.0f32,};
                                    init
                                },
                            eyeTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: 80.0f32,
                                              y: 20.0f32,
                                              z: 60.0f32,};
                                    init
                                },};
         init
     },
     {
         let mut init =
             OnePointCsFull{actionFlags: 0x8d as libc::c_int as u8_0,
                            unk_01: 0xff as libc::c_int as u8_0,
                            initFlags: 0x4141 as libc::c_int as s16,
                            timerInit: 150 as libc::c_int as s16,
                            rollTargetInit: 0 as libc::c_int as s16,
                            fovTargetInit: 50.0f32,
                            lerpStepScale: 1.0f32,
                            atTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: 0.0f32,
                                              y: 5.0f32,
                                              z: 0.0f32,};
                                    init
                                },
                            eyeTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: 0.0f32,
                                              y: 4.0f32,
                                              z: 120.0f32,};
                                    init
                                },};
         init
     },
     {
         let mut init =
             OnePointCsFull{actionFlags: 0x98 as libc::c_int as u8_0,
                            unk_01: 0xff as libc::c_int as u8_0,
                            initFlags: 0 as libc::c_int as s16,
                            timerInit: 1 as libc::c_int as s16,
                            rollTargetInit: 0 as libc::c_int as s16,
                            fovTargetInit: 50.0f32,
                            lerpStepScale: 1.0f32,
                            atTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: -1.0f32,
                                              y: -1.0f32,
                                              z: -1.0f32,};
                                    init
                                },
                            eyeTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: -1.0f32,
                                              y: -1.0f32,
                                              z: -1.0f32,};
                                    init
                                },};
         init
     }];
static mut D_80120D4C: [OnePointCsFull; 7] =
    [{
         let mut init =
             OnePointCsFull{actionFlags: 0x8f as libc::c_int as u8_0,
                            unk_01: 0x1 as libc::c_int as u8_0,
                            initFlags: 0x2142 as libc::c_int as s16,
                            timerInit: 1 as libc::c_int as s16,
                            rollTargetInit: 0 as libc::c_int as s16,
                            fovTargetInit: 40.0f32,
                            lerpStepScale: 1.0f32,
                            atTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: 20.0f32,
                                              y: 40.0f32,
                                              z: 20.0f32,};
                                    init
                                },
                            eyeTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: -20.0f32,
                                              y: 0.0f32,
                                              z: -30.0f32,};
                                    init
                                },};
         init
     },
     {
         let mut init =
             OnePointCsFull{actionFlags: 0x84 as libc::c_int as u8_0,
                            unk_01: 0xff as libc::c_int as u8_0,
                            initFlags: 0x404 as libc::c_int as s16,
                            timerInit: 19 as libc::c_int as s16,
                            rollTargetInit: 5 as libc::c_int as s16,
                            fovTargetInit: 70.0f32,
                            lerpStepScale: 0.01f32,
                            atTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: 0.0f32,
                                              y: 30.0f32,
                                              z: 20.0f32,};
                                    init
                                },
                            eyeTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: 120.0f32,
                                              y: 60.0f32,
                                              z: 120.0f32,};
                                    init
                                },};
         init
     },
     {
         let mut init =
             OnePointCsFull{actionFlags: 0x84 as libc::c_int as u8_0,
                            unk_01: 0xff as libc::c_int as u8_0,
                            initFlags: 0x404 as libc::c_int as s16,
                            timerInit: 20 as libc::c_int as s16,
                            rollTargetInit: 0 as libc::c_int as s16,
                            fovTargetInit: 60.0f32,
                            lerpStepScale: 0.01f32,
                            atTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: 0.0f32,
                                              y: 20.0f32,
                                              z: 20.0f32,};
                                    init
                                },
                            eyeTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: 120.0f32,
                                              y: 60.0f32,
                                              z: 120.0f32,};
                                    init
                                },};
         init
     },
     {
         let mut init =
             OnePointCsFull{actionFlags: 0x84 as libc::c_int as u8_0,
                            unk_01: 0xff as libc::c_int as u8_0,
                            initFlags: 0x404 as libc::c_int as s16,
                            timerInit: 40 as libc::c_int as s16,
                            rollTargetInit: -(10 as libc::c_int) as s16,
                            fovTargetInit: 50.0f32,
                            lerpStepScale: 0.02f32,
                            atTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: 0.0f32,
                                              y: 30.0f32,
                                              z: 20.0f32,};
                                    init
                                },
                            eyeTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: 120.0f32,
                                              y: 60.0f32,
                                              z: 120.0f32,};
                                    init
                                },};
         init
     },
     {
         let mut init =
             OnePointCsFull{actionFlags: 0x8f as libc::c_int as u8_0,
                            unk_01: 0xff as libc::c_int as u8_0,
                            initFlags: 0x4141 as libc::c_int as s16,
                            timerInit: 1 as libc::c_int as s16,
                            rollTargetInit: 0 as libc::c_int as s16,
                            fovTargetInit: 50.0f32,
                            lerpStepScale: 1.0f32,
                            atTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: 0.0f32,
                                              y: -10.0f32,
                                              z: 0.0f32,};
                                    init
                                },
                            eyeTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: 80.0f32,
                                              y: 20.0f32,
                                              z: 60.0f32,};
                                    init
                                },};
         init
     },
     {
         let mut init =
             OnePointCsFull{actionFlags: 0x8d as libc::c_int as u8_0,
                            unk_01: 0xff as libc::c_int as u8_0,
                            initFlags: 0x4141 as libc::c_int as s16,
                            timerInit: 150 as libc::c_int as s16,
                            rollTargetInit: 0 as libc::c_int as s16,
                            fovTargetInit: 50.0f32,
                            lerpStepScale: 1.0f32,
                            atTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: 0.0f32,
                                              y: 5.0f32,
                                              z: 0.0f32,};
                                    init
                                },
                            eyeTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: 0.0f32,
                                              y: 4.0f32,
                                              z: 120.0f32,};
                                    init
                                },};
         init
     },
     {
         let mut init =
             OnePointCsFull{actionFlags: 0x98 as libc::c_int as u8_0,
                            unk_01: 0xff as libc::c_int as u8_0,
                            initFlags: 0 as libc::c_int as s16,
                            timerInit: 1 as libc::c_int as s16,
                            rollTargetInit: 0 as libc::c_int as s16,
                            fovTargetInit: 50.0f32,
                            lerpStepScale: 1.0f32,
                            atTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: -1.0f32,
                                              y: -1.0f32,
                                              z: -1.0f32,};
                                    init
                                },
                            eyeTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: -1.0f32,
                                              y: -1.0f32,
                                              z: -1.0f32,};
                                    init
                                },};
         init
     }];
static mut D_80120E64: [OnePointCsFull; 8] =
    [{
         let mut init =
             OnePointCsFull{actionFlags: 0x41 as libc::c_int as u8_0,
                            unk_01: 0x1 as libc::c_int as u8_0,
                            initFlags: 0x2142 as libc::c_int as s16,
                            timerInit: 20 as libc::c_int as s16,
                            rollTargetInit: 0 as libc::c_int as s16,
                            fovTargetInit: 50.0f32,
                            lerpStepScale: 1.0f32,
                            atTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: -25.0f32,
                                              y: 20.0f32,
                                              z: 0.0f32,};
                                    init
                                },
                            eyeTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: 0.0f32,
                                              y: 0.0f32,
                                              z: 5.0f32,};
                                    init
                                },};
         init
     },
     {
         let mut init =
             OnePointCsFull{actionFlags: 0x4f as libc::c_int as u8_0,
                            unk_01: 0xff as libc::c_int as u8_0,
                            initFlags: 0 as libc::c_int as s16,
                            timerInit: 80 as libc::c_int as s16,
                            rollTargetInit: 0 as libc::c_int as s16,
                            fovTargetInit: 50.0f32,
                            lerpStepScale: 1.0f32,
                            atTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: -25.0f32,
                                              y: 20.0f32,
                                              z: 0.0f32,};
                                    init
                                },
                            eyeTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: 0.0f32,
                                              y: 0.0f32,
                                              z: 5.0f32,};
                                    init
                                },};
         init
     },
     {
         let mut init =
             OnePointCsFull{actionFlags: 0x4b as libc::c_int as u8_0,
                            unk_01: 0xff as libc::c_int as u8_0,
                            initFlags: 0x4242 as libc::c_int as s16,
                            timerInit: 8 as libc::c_int as s16,
                            rollTargetInit: 0 as libc::c_int as s16,
                            fovTargetInit: 60.0f32,
                            lerpStepScale: 0.1f32,
                            atTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: -25.0f32,
                                              y: 20.0f32,
                                              z: 0.0f32,};
                                    init
                                },
                            eyeTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: 0.0f32,
                                              y: 0.0f32,
                                              z: 5.0f32,};
                                    init
                                },};
         init
     },
     {
         let mut init =
             OnePointCsFull{actionFlags: 0x4b as libc::c_int as u8_0,
                            unk_01: 0xff as libc::c_int as u8_0,
                            initFlags: 0x4242 as libc::c_int as s16,
                            timerInit: 15 as libc::c_int as s16,
                            rollTargetInit: 4 as libc::c_int as s16,
                            fovTargetInit: 55.0f32,
                            lerpStepScale: 0.05f32,
                            atTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: -50.0f32,
                                              y: 20.0f32,
                                              z: 20.0f32,};
                                    init
                                },
                            eyeTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: 0.0f32,
                                              y: 0.0f32,
                                              z: 5.0f32,};
                                    init
                                },};
         init
     },
     {
         let mut init =
             OnePointCsFull{actionFlags: 0x4b as libc::c_int as u8_0,
                            unk_01: 0xff as libc::c_int as u8_0,
                            initFlags: 0x4242 as libc::c_int as s16,
                            timerInit: 15 as libc::c_int as s16,
                            rollTargetInit: -(4 as libc::c_int) as s16,
                            fovTargetInit: 50.0f32,
                            lerpStepScale: 0.05f32,
                            atTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: 0.0f32,
                                              y: 20.0f32,
                                              z: 0.0f32,};
                                    init
                                },
                            eyeTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: 0.0f32,
                                              y: 0.0f32,
                                              z: 5.0f32,};
                                    init
                                },};
         init
     },
     {
         let mut init =
             OnePointCsFull{actionFlags: 0x4b as libc::c_int as u8_0,
                            unk_01: 0xff as libc::c_int as u8_0,
                            initFlags: 0x4242 as libc::c_int as s16,
                            timerInit: 15 as libc::c_int as s16,
                            rollTargetInit: 0 as libc::c_int as s16,
                            fovTargetInit: 50.0f32,
                            lerpStepScale: 0.1f32,
                            atTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: -25.0f32,
                                              y: 20.0f32,
                                              z: 0.0f32,};
                                    init
                                },
                            eyeTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: 0.0f32,
                                              y: 0.0f32,
                                              z: 5.0f32,};
                                    init
                                },};
         init
     },
     {
         let mut init =
             OnePointCsFull{actionFlags: 0x4f as libc::c_int as u8_0,
                            unk_01: 0xff as libc::c_int as u8_0,
                            initFlags: 0 as libc::c_int as s16,
                            timerInit: 40 as libc::c_int as s16,
                            rollTargetInit: 0 as libc::c_int as s16,
                            fovTargetInit: 50.0f32,
                            lerpStepScale: 1.0f32,
                            atTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: -1.0f32,
                                              y: -1.0f32,
                                              z: -1.0f32,};
                                    init
                                },
                            eyeTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: -1.0f32,
                                              y: -1.0f32,
                                              z: -1.0f32,};
                                    init
                                },};
         init
     },
     {
         let mut init =
             OnePointCsFull{actionFlags: 0x12 as libc::c_int as u8_0,
                            unk_01: 0xff as libc::c_int as u8_0,
                            initFlags: 0 as libc::c_int as s16,
                            timerInit: 1 as libc::c_int as s16,
                            rollTargetInit: 0 as libc::c_int as s16,
                            fovTargetInit: 60.0f32,
                            lerpStepScale: 1.0f32,
                            atTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: -1.0f32,
                                              y: -1.0f32,
                                              z: -1.0f32,};
                                    init
                                },
                            eyeTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: -1.0f32,
                                              y: -1.0f32,
                                              z: -1.0f32,};
                                    init
                                },};
         init
     }];
static mut D_80120FA4: [OnePointCsFull; 6] =
    [{
         let mut init =
             OnePointCsFull{actionFlags: 0x8f as libc::c_int as u8_0,
                            unk_01: 0x1 as libc::c_int as u8_0,
                            initFlags: 0x2143 as libc::c_int as s16,
                            timerInit: 30 as libc::c_int as s16,
                            rollTargetInit: 0 as libc::c_int as s16,
                            fovTargetInit: 70.0f32,
                            lerpStepScale: 0.4f32,
                            atTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: 0.0f32,
                                              y: 40.0f32,
                                              z: 50.0f32,};
                                    init
                                },
                            eyeTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: 30.0f32,
                                              y: 10.0f32,
                                              z: -50.0f32,};
                                    init
                                },};
         init
     },
     {
         let mut init =
             OnePointCsFull{actionFlags: 0x95 as libc::c_int as u8_0,
                            unk_01: 0xff as libc::c_int as u8_0,
                            initFlags: 0 as libc::c_int as s16,
                            timerInit: 1 as libc::c_int as s16,
                            rollTargetInit: 0 as libc::c_int as s16,
                            fovTargetInit: 50.0f32,
                            lerpStepScale: 1.0f32,
                            atTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: -1.0f32,
                                              y: -1.0f32,
                                              z: -1.0f32,};
                                    init
                                },
                            eyeTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: -1.0f32,
                                              y: -1.0f32,
                                              z: -1.0f32,};
                                    init
                                },};
         init
     },
     {
         let mut init =
             OnePointCsFull{actionFlags: 0x8f as libc::c_int as u8_0,
                            unk_01: 0xff as libc::c_int as u8_0,
                            initFlags: 0x2222 as libc::c_int as s16,
                            timerInit: 10 as libc::c_int as s16,
                            rollTargetInit: 0 as libc::c_int as s16,
                            fovTargetInit: 42.0f32,
                            lerpStepScale: 1.0f32,
                            atTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: 0.0f32,
                                              y: 40.0f32,
                                              z: 0.0f32,};
                                    init
                                },
                            eyeTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: 0.0f32,
                                              y: 85.0f32,
                                              z: 45.0f32,};
                                    init
                                },};
         init
     },
     {
         let mut init =
             OnePointCsFull{actionFlags: 0x90 as libc::c_int as u8_0,
                            unk_01: 0xff as libc::c_int as u8_0,
                            initFlags: 0 as libc::c_int as s16,
                            timerInit: 1 as libc::c_int as s16,
                            rollTargetInit: 0 as libc::c_int as s16,
                            fovTargetInit: 50.0f32,
                            lerpStepScale: 1.0f32,
                            atTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: -1.0f32,
                                              y: -1.0f32,
                                              z: -1.0f32,};
                                    init
                                },
                            eyeTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: -1.0f32,
                                              y: -1.0f32,
                                              z: -1.0f32,};
                                    init
                                },};
         init
     },
     {
         let mut init =
             OnePointCsFull{actionFlags: 0x81 as libc::c_int as u8_0,
                            unk_01: 0xff as libc::c_int as u8_0,
                            initFlags: 0x2121 as libc::c_int as s16,
                            timerInit: 10 as libc::c_int as s16,
                            rollTargetInit: 0 as libc::c_int as s16,
                            fovTargetInit: 60.0f32,
                            lerpStepScale: 1.0f32,
                            atTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: 0.0f32,
                                              y: 10.0f32,
                                              z: 0.0f32,};
                                    init
                                },
                            eyeTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: 30.0f32,
                                              y: 10.0f32,
                                              z: -80.0f32,};
                                    init
                                },};
         init
     },
     {
         let mut init =
             OnePointCsFull{actionFlags: 0x12 as libc::c_int as u8_0,
                            unk_01: 0xff as libc::c_int as u8_0,
                            initFlags: 0 as libc::c_int as s16,
                            timerInit: 1 as libc::c_int as s16,
                            rollTargetInit: 0 as libc::c_int as s16,
                            fovTargetInit: 60.0f32,
                            lerpStepScale: 1.0f32,
                            atTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: -1.0f32,
                                              y: -1.0f32,
                                              z: -1.0f32,};
                                    init
                                },
                            eyeTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: -1.0f32,
                                              y: -1.0f32,
                                              z: -1.0f32,};
                                    init
                                },};
         init
     }];
static mut D_80121094: [OnePointCsFull; 3] =
    [{
         let mut init =
             OnePointCsFull{actionFlags: 0xf as libc::c_int as u8_0,
                            unk_01: 0x8 as libc::c_int as u8_0,
                            initFlags: 0x2101 as libc::c_int as s16,
                            timerInit: 20 as libc::c_int as s16,
                            rollTargetInit: 0 as libc::c_int as s16,
                            fovTargetInit: 50.0f32,
                            lerpStepScale: 1.0f32,
                            atTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: 3840.0f32,
                                              y: 10.0f32,
                                              z: 950.0f32,};
                                    init
                                },
                            eyeTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: 0.0f32,
                                              y: 0.0f32,
                                              z: 5.0f32,};
                                    init
                                },};
         init
     },
     {
         let mut init =
             OnePointCsFull{actionFlags: 0x1 as libc::c_int as u8_0,
                            unk_01: 0x1 as libc::c_int as u8_0,
                            initFlags: 0x2101 as libc::c_int as s16,
                            timerInit: 50 as libc::c_int as s16,
                            rollTargetInit: 0 as libc::c_int as s16,
                            fovTargetInit: 55.0f32,
                            lerpStepScale: 1.0f32,
                            atTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: 4000.0f32,
                                              y: 50.0f32,
                                              z: 1000.0f32,};
                                    init
                                },
                            eyeTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: 0.0f32,
                                              y: 0.0f32,
                                              z: 5.0f32,};
                                    init
                                },};
         init
     },
     {
         let mut init =
             OnePointCsFull{actionFlags: 0x12 as libc::c_int as u8_0,
                            unk_01: 0xff as libc::c_int as u8_0,
                            initFlags: 0 as libc::c_int as s16,
                            timerInit: 1 as libc::c_int as s16,
                            rollTargetInit: 0 as libc::c_int as s16,
                            fovTargetInit: 60.0f32,
                            lerpStepScale: 1.0f32,
                            atTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: -1.0f32,
                                              y: -1.0f32,
                                              z: -1.0f32,};
                                    init
                                },
                            eyeTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: -1.0f32,
                                              y: -1.0f32,
                                              z: -1.0f32,};
                                    init
                                },};
         init
     }];
static mut D_8012110C: [OnePointCsFull; 3] =
    [{
         let mut init =
             OnePointCsFull{actionFlags: 0x4f as libc::c_int as u8_0,
                            unk_01: 0x5 as libc::c_int as u8_0,
                            initFlags: 0x2142 as libc::c_int as s16,
                            timerInit: 1 as libc::c_int as s16,
                            rollTargetInit: 0 as libc::c_int as s16,
                            fovTargetInit: 50.0f32,
                            lerpStepScale: 1.0f32,
                            atTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: -25.0f32,
                                              y: 20.0f32,
                                              z: 0.0f32,};
                                    init
                                },
                            eyeTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: 0.0f32,
                                              y: 0.0f32,
                                              z: 5.0f32,};
                                    init
                                },};
         init
     },
     {
         let mut init =
             OnePointCsFull{actionFlags: 0x41 as libc::c_int as u8_0,
                            unk_01: 0xff as libc::c_int as u8_0,
                            initFlags: 0x2121 as libc::c_int as s16,
                            timerInit: 10 as libc::c_int as s16,
                            rollTargetInit: 0 as libc::c_int as s16,
                            fovTargetInit: 60.0f32,
                            lerpStepScale: 1.0f32,
                            atTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: 0.0f32,
                                              y: 10.0f32,
                                              z: 0.0f32,};
                                    init
                                },
                            eyeTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: 0.0f32,
                                              y: 10.0f32,
                                              z: -80.0f32,};
                                    init
                                },};
         init
     },
     {
         let mut init =
             OnePointCsFull{actionFlags: 0x12 as libc::c_int as u8_0,
                            unk_01: 0xff as libc::c_int as u8_0,
                            initFlags: 0 as libc::c_int as s16,
                            timerInit: 1 as libc::c_int as s16,
                            rollTargetInit: 0 as libc::c_int as s16,
                            fovTargetInit: 60.0f32,
                            lerpStepScale: 1.0f32,
                            atTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: -1.0f32,
                                              y: -1.0f32,
                                              z: -1.0f32,};
                                    init
                                },
                            eyeTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: -1.0f32,
                                              y: -1.0f32,
                                              z: -1.0f32,};
                                    init
                                },};
         init
     }];
static mut D_80121184: [OnePointCsFull; 2] =
    [{
         let mut init =
             OnePointCsFull{actionFlags: 0x83 as libc::c_int as u8_0,
                            unk_01: 0x1 as libc::c_int as u8_0,
                            initFlags: 0x101 as libc::c_int as s16,
                            timerInit: 40 as libc::c_int as s16,
                            rollTargetInit: 0 as libc::c_int as s16,
                            fovTargetInit: -1.0f32,
                            lerpStepScale: 0.1f32,
                            atTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: 0.0f32,
                                              y: 10.0f32,
                                              z: 0.0f32,};
                                    init
                                },
                            eyeTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: 0.0f32,
                                              y: 0.0f32,
                                              z: 0.0f32,};
                                    init
                                },};
         init
     },
     {
         let mut init =
             OnePointCsFull{actionFlags: 0x12 as libc::c_int as u8_0,
                            unk_01: 0xff as libc::c_int as u8_0,
                            initFlags: 0 as libc::c_int as s16,
                            timerInit: 1 as libc::c_int as s16,
                            rollTargetInit: 0 as libc::c_int as s16,
                            fovTargetInit: 60.0f32,
                            lerpStepScale: 1.0f32,
                            atTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: -1.0f32,
                                              y: -1.0f32,
                                              z: -1.0f32,};
                                    init
                                },
                            eyeTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: -1.0f32,
                                              y: -1.0f32,
                                              z: -1.0f32,};
                                    init
                                },};
         init
     }];
static mut D_801211D4: [OnePointCsFull; 2] =
    [{
         let mut init =
             OnePointCsFull{actionFlags: 0x8f as libc::c_int as u8_0,
                            unk_01: 0x8 as libc::c_int as u8_0,
                            initFlags: 0x101 as libc::c_int as s16,
                            timerInit: 50 as libc::c_int as s16,
                            rollTargetInit: 0 as libc::c_int as s16,
                            fovTargetInit: 60.0f32,
                            lerpStepScale: 1.0f32,
                            atTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: 0.0f32,
                                              y: 10.0f32,
                                              z: 0.0f32,};
                                    init
                                },
                            eyeTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: -10.0f32,
                                              y: 85.0f32,
                                              z: 0.0f32,};
                                    init
                                },};
         init
     },
     {
         let mut init =
             OnePointCsFull{actionFlags: 0x11 as libc::c_int as u8_0,
                            unk_01: 0xff as libc::c_int as u8_0,
                            initFlags: 0 as libc::c_int as s16,
                            timerInit: 1 as libc::c_int as s16,
                            rollTargetInit: 0 as libc::c_int as s16,
                            fovTargetInit: 60.0f32,
                            lerpStepScale: 1.0f32,
                            atTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: -1.0f32,
                                              y: -1.0f32,
                                              z: -1.0f32,};
                                    init
                                },
                            eyeTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: -1.0f32,
                                              y: -1.0f32,
                                              z: -1.0f32,};
                                    init
                                },};
         init
     }];
static mut D_80121224: [OnePointCsFull; 6] =
    [{
         let mut init =
             OnePointCsFull{actionFlags: 0x8f as libc::c_int as u8_0,
                            unk_01: 0x1 as libc::c_int as u8_0,
                            initFlags: 0x4141 as libc::c_int as s16,
                            timerInit: 2 as libc::c_int as s16,
                            rollTargetInit: 0 as libc::c_int as s16,
                            fovTargetInit: 60.0f32,
                            lerpStepScale: 1.0f32,
                            atTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: 0.0f32,
                                              y: 5.0f32,
                                              z: 10.0f32,};
                                    init
                                },
                            eyeTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: 0.0f32,
                                              y: 0.0f32,
                                              z: 45.0f32,};
                                    init
                                },};
         init
     },
     {
         let mut init =
             OnePointCsFull{actionFlags: 0x81 as libc::c_int as u8_0,
                            unk_01: 0xff as libc::c_int as u8_0,
                            initFlags: 0x4141 as libc::c_int as s16,
                            timerInit: 18 as libc::c_int as s16,
                            rollTargetInit: 0 as libc::c_int as s16,
                            fovTargetInit: 45.0f32,
                            lerpStepScale: 1.0f32,
                            atTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: 0.0f32,
                                              y: 5.0f32,
                                              z: 10.0f32,};
                                    init
                                },
                            eyeTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: 0.0f32,
                                              y: -10.0f32,
                                              z: 50.0f32,};
                                    init
                                },};
         init
     },
     {
         let mut init =
             OnePointCsFull{actionFlags: 0x84 as libc::c_int as u8_0,
                            unk_01: 0x34 as libc::c_int as u8_0,
                            initFlags: 0x4104 as libc::c_int as s16,
                            timerInit: 80 as libc::c_int as s16,
                            rollTargetInit: 0 as libc::c_int as s16,
                            fovTargetInit: 70.0f32,
                            lerpStepScale: 0.05f32,
                            atTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: 0.0f32,
                                              y: 0.0f32,
                                              z: 60.0f32,};
                                    init
                                },
                            eyeTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: 0.0f32,
                                              y: 250.0f32,
                                              z: -50.0f32,};
                                    init
                                },};
         init
     },
     {
         let mut init =
             OnePointCsFull{actionFlags: 0x8f as libc::c_int as u8_0,
                            unk_01: 0x1 as libc::c_int as u8_0,
                            initFlags: 0 as libc::c_int as s16,
                            timerInit: 20 as libc::c_int as s16,
                            rollTargetInit: 0 as libc::c_int as s16,
                            fovTargetInit: 70.0f32,
                            lerpStepScale: 1.0f32,
                            atTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: -1.0f32,
                                              y: -1.0f32,
                                              z: -1.0f32,};
                                    init
                                },
                            eyeTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: -1.0f32,
                                              y: -1.0f32,
                                              z: -1.0f32,};
                                    init
                                },};
         init
     },
     {
         let mut init =
             OnePointCsFull{actionFlags: 0x8f as libc::c_int as u8_0,
                            unk_01: 0xff as libc::c_int as u8_0,
                            initFlags: 0x421 as libc::c_int as s16,
                            timerInit: 60 as libc::c_int as s16,
                            rollTargetInit: 0 as libc::c_int as s16,
                            fovTargetInit: 50.0f32,
                            lerpStepScale: 1.0f32,
                            atTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: 0.0f32,
                                              y: -30.0f32,
                                              z: 20.0f32,};
                                    init
                                },
                            eyeTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: 10.0f32,
                                              y: 5.0f32,
                                              z: -50.0f32,};
                                    init
                                },};
         init
     },
     {
         let mut init =
             OnePointCsFull{actionFlags: 0x11 as libc::c_int as u8_0,
                            unk_01: 0xff as libc::c_int as u8_0,
                            initFlags: 0 as libc::c_int as s16,
                            timerInit: 1 as libc::c_int as s16,
                            rollTargetInit: 0 as libc::c_int as s16,
                            fovTargetInit: 60.0f32,
                            lerpStepScale: 1.0f32,
                            atTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: -1.0f32,
                                              y: -1.0f32,
                                              z: -1.0f32,};
                                    init
                                },
                            eyeTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: -1.0f32,
                                              y: -1.0f32,
                                              z: -1.0f32,};
                                    init
                                },};
         init
     }];
static mut D_80121314: [OnePointCsFull; 1] =
    [{
         let mut init =
             OnePointCsFull{actionFlags: 0x8f as libc::c_int as u8_0,
                            unk_01: 0x8 as libc::c_int as u8_0,
                            initFlags: 0x4141 as libc::c_int as s16,
                            timerInit: 1000 as libc::c_int as s16,
                            rollTargetInit: 0 as libc::c_int as s16,
                            fovTargetInit: 75.0f32,
                            lerpStepScale: 0.6f32,
                            atTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: 0.0f32,
                                              y: 0.0f32,
                                              z: 10.0f32,};
                                    init
                                },
                            eyeTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: 0.0f32,
                                              y: 0.0f32,
                                              z: 100.0f32,};
                                    init
                                },};
         init
     }];
static mut D_8012133C: [OnePointCsFull; 3] =
    [{
         let mut init =
             OnePointCsFull{actionFlags: 0x8f as libc::c_int as u8_0,
                            unk_01: 0x1 as libc::c_int as u8_0,
                            initFlags: 0x141 as libc::c_int as s16,
                            timerInit: 40 as libc::c_int as s16,
                            rollTargetInit: 0 as libc::c_int as s16,
                            fovTargetInit: 75.0f32,
                            lerpStepScale: 1.0f32,
                            atTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: 0.0f32,
                                              y: 60.0f32,
                                              z: 0.0f32,};
                                    init
                                },
                            eyeTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: 0.0f32,
                                              y: 0.0f32,
                                              z: 100.0f32,};
                                    init
                                },};
         init
     },
     {
         let mut init =
             OnePointCsFull{actionFlags: 0x83 as libc::c_int as u8_0,
                            unk_01: 0xff as libc::c_int as u8_0,
                            initFlags: 0x2121 as libc::c_int as s16,
                            timerInit: 20 as libc::c_int as s16,
                            rollTargetInit: 0 as libc::c_int as s16,
                            fovTargetInit: 60.0f32,
                            lerpStepScale: 0.2f32,
                            atTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: 0.0f32,
                                              y: -10.0f32,
                                              z: -10.0f32,};
                                    init
                                },
                            eyeTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: 0.0f32,
                                              y: 10.0f32,
                                              z: -100.0f32,};
                                    init
                                },};
         init
     },
     {
         let mut init =
             OnePointCsFull{actionFlags: 0x11 as libc::c_int as u8_0,
                            unk_01: 0xff as libc::c_int as u8_0,
                            initFlags: 0 as libc::c_int as s16,
                            timerInit: 1 as libc::c_int as s16,
                            rollTargetInit: 0 as libc::c_int as s16,
                            fovTargetInit: 60.0f32,
                            lerpStepScale: 1.0f32,
                            atTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: -1.0f32,
                                              y: -1.0f32,
                                              z: -1.0f32,};
                                    init
                                },
                            eyeTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: -1.0f32,
                                              y: -1.0f32,
                                              z: -1.0f32,};
                                    init
                                },};
         init
     }];
static mut D_801213B4: [OnePointCsFull; 5] =
    [{
         let mut init =
             OnePointCsFull{actionFlags: 0x8f as libc::c_int as u8_0,
                            unk_01: 0x8 as libc::c_int as u8_0,
                            initFlags: 0xc2c2 as libc::c_int as s16,
                            timerInit: 40 as libc::c_int as s16,
                            rollTargetInit: 0 as libc::c_int as s16,
                            fovTargetInit: 70.0f32,
                            lerpStepScale: 1.0f32,
                            atTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: 80.0f32,
                                              y: 0.0f32,
                                              z: 20.0f32,};
                                    init
                                },
                            eyeTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: 20.0f32,
                                              y: 0.0f32,
                                              z: 80.0f32,};
                                    init
                                },};
         init
     },
     {
         let mut init =
             OnePointCsFull{actionFlags: 0x8b as libc::c_int as u8_0,
                            unk_01: 0x1 as libc::c_int as u8_0,
                            initFlags: 0xc2c2 as libc::c_int as s16,
                            timerInit: 120 as libc::c_int as s16,
                            rollTargetInit: 0 as libc::c_int as s16,
                            fovTargetInit: 70.0f32,
                            lerpStepScale: 0.1f32,
                            atTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: 80.0f32,
                                              y: 0.0f32,
                                              z: 20.0f32,};
                                    init
                                },
                            eyeTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: 20.0f32,
                                              y: 0.0f32,
                                              z: 80.0f32,};
                                    init
                                },};
         init
     },
     {
         let mut init =
             OnePointCsFull{actionFlags: 0x8f as libc::c_int as u8_0,
                            unk_01: 0x53 as libc::c_int as u8_0,
                            initFlags: 0xc2c2 as libc::c_int as s16,
                            timerInit: 30 as libc::c_int as s16,
                            rollTargetInit: 0 as libc::c_int as s16,
                            fovTargetInit: 50.0f32,
                            lerpStepScale: 1.0f32,
                            atTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: 60.0f32,
                                              y: 0.0f32,
                                              z: 20.0f32,};
                                    init
                                },
                            eyeTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: 60.0f32,
                                              y: 0.0f32,
                                              z: 60.0f32,};
                                    init
                                },};
         init
     },
     {
         let mut init =
             OnePointCsFull{actionFlags: 0x84 as libc::c_int as u8_0,
                            unk_01: 0x45 as libc::c_int as u8_0,
                            initFlags: 0x4222 as libc::c_int as s16,
                            timerInit: 30 as libc::c_int as s16,
                            rollTargetInit: 0 as libc::c_int as s16,
                            fovTargetInit: 60.0f32,
                            lerpStepScale: 0.1f32,
                            atTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: 0.0f32,
                                              y: 50.0f32,
                                              z: 0.0f32,};
                                    init
                                },
                            eyeTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: 5.0f32,
                                              y: 30.0f32,
                                              z: 220.0f32,};
                                    init
                                },};
         init
     },
     {
         let mut init =
             OnePointCsFull{actionFlags: 0x12 as libc::c_int as u8_0,
                            unk_01: 0xff as libc::c_int as u8_0,
                            initFlags: 0 as libc::c_int as s16,
                            timerInit: 1 as libc::c_int as s16,
                            rollTargetInit: 0 as libc::c_int as s16,
                            fovTargetInit: 75.0f32,
                            lerpStepScale: 1.0f32,
                            atTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: -1.0f32,
                                              y: -1.0f32,
                                              z: -1.0f32,};
                                    init
                                },
                            eyeTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: -1.0f32,
                                              y: -1.0f32,
                                              z: -1.0f32,};
                                    init
                                },};
         init
     }];
static mut D_8012147C: [OnePointCsFull; 4] =
    [{
         let mut init =
             OnePointCsFull{actionFlags: 0x8f as libc::c_int as u8_0,
                            unk_01: 0x1 as libc::c_int as u8_0,
                            initFlags: 0x101 as libc::c_int as s16,
                            timerInit: 40 as libc::c_int as s16,
                            rollTargetInit: 0 as libc::c_int as s16,
                            fovTargetInit: 45.0f32,
                            lerpStepScale: 1.0f32,
                            atTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: 820.0f32,
                                              y: 1600.0f32,
                                              z: -400.0f32,};
                                    init
                                },
                            eyeTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: 777.0f32,
                                              y: 1577.0f32,
                                              z: -577.0f32,};
                                    init
                                },};
         init
     },
     {
         let mut init =
             OnePointCsFull{actionFlags: 0x8f as libc::c_int as u8_0,
                            unk_01: 0x1 as libc::c_int as u8_0,
                            initFlags: 0x142 as libc::c_int as s16,
                            timerInit: 1 as libc::c_int as s16,
                            rollTargetInit: 0 as libc::c_int as s16,
                            fovTargetInit: 50.0f32,
                            lerpStepScale: 1.0f32,
                            atTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: -50.0f32,
                                              y: 80.0f32,
                                              z: 0.0f32,};
                                    init
                                },
                            eyeTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: 900.0f32,
                                              y: 1575.0f32,
                                              z: 850.0f32,};
                                    init
                                },};
         init
     },
     {
         let mut init =
             OnePointCsFull{actionFlags: 0x83 as libc::c_int as u8_0,
                            unk_01: 0x8 as libc::c_int as u8_0,
                            initFlags: 0x142 as libc::c_int as s16,
                            timerInit: 89 as libc::c_int as s16,
                            rollTargetInit: -(4 as libc::c_int) as s16,
                            fovTargetInit: 80.0f32,
                            lerpStepScale: 0.07f32,
                            atTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: -50.0f32,
                                              y: 70.0f32,
                                              z: 0.0f32,};
                                    init
                                },
                            eyeTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: 975.0f32,
                                              y: 1575.0f32,
                                              z: 770.0f32,};
                                    init
                                },};
         init
     },
     {
         let mut init =
             OnePointCsFull{actionFlags: 0x11 as libc::c_int as u8_0,
                            unk_01: 0xff as libc::c_int as u8_0,
                            initFlags: 0 as libc::c_int as s16,
                            timerInit: 1 as libc::c_int as s16,
                            rollTargetInit: 0 as libc::c_int as s16,
                            fovTargetInit: 60.0f32,
                            lerpStepScale: 1.0f32,
                            atTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: -1.0f32,
                                              y: -1.0f32,
                                              z: -1.0f32,};
                                    init
                                },
                            eyeTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: -1.0f32,
                                              y: -1.0f32,
                                              z: -1.0f32,};
                                    init
                                },};
         init
     }];
static mut D_8012151C: [OnePointCsFull; 2] =
    [{
         let mut init =
             OnePointCsFull{actionFlags: 0xf as libc::c_int as u8_0,
                            unk_01: 0x1 as libc::c_int as u8_0,
                            initFlags: 0x101 as libc::c_int as s16,
                            timerInit: 29 as libc::c_int as s16,
                            rollTargetInit: 0 as libc::c_int as s16,
                            fovTargetInit: 60.0f32,
                            lerpStepScale: 1.0f32,
                            atTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: -700.0f32,
                                              y: 875.0f32,
                                              z: -100.0f32,};
                                    init
                                },
                            eyeTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: -550.0f32,
                                              y: 920.0f32,
                                              z: -150.0f32,};
                                    init
                                },};
         init
     },
     {
         let mut init =
             OnePointCsFull{actionFlags: 0x12 as libc::c_int as u8_0,
                            unk_01: 0xff as libc::c_int as u8_0,
                            initFlags: 0 as libc::c_int as s16,
                            timerInit: 1 as libc::c_int as s16,
                            rollTargetInit: 0 as libc::c_int as s16,
                            fovTargetInit: 60.0f32,
                            lerpStepScale: 1.0f32,
                            atTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: -1.0f32,
                                              y: -1.0f32,
                                              z: -1.0f32,};
                                    init
                                },
                            eyeTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: -1.0f32,
                                              y: -1.0f32,
                                              z: -1.0f32,};
                                    init
                                },};
         init
     }];
static mut D_8012156C: [OnePointCsFull; 2] =
    [{
         let mut init =
             OnePointCsFull{actionFlags: 0x8f as libc::c_int as u8_0,
                            unk_01: 0x4d as libc::c_int as u8_0,
                            initFlags: 0x4242 as libc::c_int as s16,
                            timerInit: 1 as libc::c_int as s16,
                            rollTargetInit: 0 as libc::c_int as s16,
                            fovTargetInit: 65.0f32,
                            lerpStepScale: 1.0f32,
                            atTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: 60.0f32,
                                              y: 30.0f32,
                                              z: 0.0f32,};
                                    init
                                },
                            eyeTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: 50.0f32,
                                              y: 20.0f32,
                                              z: 150.0f32,};
                                    init
                                },};
         init
     },
     {
         let mut init =
             OnePointCsFull{actionFlags: 0x81 as libc::c_int as u8_0,
                            unk_01: 0xff as libc::c_int as u8_0,
                            initFlags: 0x4242 as libc::c_int as s16,
                            timerInit: -(1 as libc::c_int) as s16,
                            rollTargetInit: 0 as libc::c_int as s16,
                            fovTargetInit: 65.0f32,
                            lerpStepScale: 1.0f32,
                            atTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: -50.0f32,
                                              y: 60.0f32,
                                              z: 0.0f32,};
                                    init
                                },
                            eyeTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: -60.0f32,
                                              y: 40.0f32,
                                              z: 150.0f32,};
                                    init
                                },};
         init
     }];
static mut D_801215BC: [OnePointCsFull; 1] =
    [{
         let mut init =
             OnePointCsFull{actionFlags: 0xf as libc::c_int as u8_0,
                            unk_01: 0xff as libc::c_int as u8_0,
                            initFlags: 0x101 as libc::c_int as s16,
                            timerInit: 5 as libc::c_int as s16,
                            rollTargetInit: 0 as libc::c_int as s16,
                            fovTargetInit: 65.0f32,
                            lerpStepScale: 1.0f32,
                            atTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: -1185.0f32,
                                              y: 655.0f32,
                                              z: 1185.0f32,};
                                    init
                                },
                            eyeTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: -1255.0f32,
                                              y: 735.0f32,
                                              z: 1255.0f32,};
                                    init
                                },};
         init
     }];
static mut D_801215E4: [OnePointCsFull; 10] =
    [{
         let mut init =
             OnePointCsFull{actionFlags: 0xf as libc::c_int as u8_0,
                            unk_01: 0x8 as libc::c_int as u8_0,
                            initFlags: 0x4141 as libc::c_int as s16,
                            timerInit: 20 as libc::c_int as s16,
                            rollTargetInit: 0 as libc::c_int as s16,
                            fovTargetInit: 30.0f32,
                            lerpStepScale: 1.0f32,
                            atTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: 0.0f32,
                                              y: 120.0f32,
                                              z: 0.0f32,};
                                    init
                                },
                            eyeTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: -10.0f32,
                                              y: 140.0f32,
                                              z: -90.0f32,};
                                    init
                                },};
         init
     },
     {
         let mut init =
             OnePointCsFull{actionFlags: 0xf as libc::c_int as u8_0,
                            unk_01: 0x1 as libc::c_int as u8_0,
                            initFlags: 0x101 as libc::c_int as s16,
                            timerInit: 1 as libc::c_int as s16,
                            rollTargetInit: 4 as libc::c_int as s16,
                            fovTargetInit: 75.0f32,
                            lerpStepScale: 1.0f32,
                            atTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: -1360.0f32,
                                              y: -940.0f32,
                                              z: -3343.0f32,};
                                    init
                                },
                            eyeTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: -1060.0f32,
                                              y: -980.0f32,
                                              z: -3325.0f32,};
                                    init
                                },};
         init
     },
     {
         let mut init =
             OnePointCsFull{actionFlags: 0xb as libc::c_int as u8_0,
                            unk_01: 0xff as libc::c_int as u8_0,
                            initFlags: 0x4141 as libc::c_int as s16,
                            timerInit: 129 as libc::c_int as s16,
                            rollTargetInit: 0 as libc::c_int as s16,
                            fovTargetInit: 75.0f32,
                            lerpStepScale: 0.5f32,
                            atTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: 0.0f32,
                                              y: 50.0f32,
                                              z: 0.0f32,};
                                    init
                                },
                            eyeTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: -1.0f32,
                                              y: -1.0f32,
                                              z: -1.0f32,};
                                    init
                                },};
         init
     },
     {
         let mut init =
             OnePointCsFull{actionFlags: 0x3 as libc::c_int as u8_0,
                            unk_01: 0xff as libc::c_int as u8_0,
                            initFlags: 0x303 as libc::c_int as s16,
                            timerInit: 30 as libc::c_int as s16,
                            rollTargetInit: 0 as libc::c_int as s16,
                            fovTargetInit: 70.0f32,
                            lerpStepScale: 0.05f32,
                            atTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: 0.0f32,
                                              y: 80.0f32,
                                              z: 0.0f32,};
                                    init
                                },
                            eyeTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: -10.0f32,
                                              y: 120.0f32,
                                              z: 10.0f32,};
                                    init
                                },};
         init
     },
     {
         let mut init =
             OnePointCsFull{actionFlags: 0xf as libc::c_int as u8_0,
                            unk_01: 0x9 as libc::c_int as u8_0,
                            initFlags: 0x101 as libc::c_int as s16,
                            timerInit: 40 as libc::c_int as s16,
                            rollTargetInit: -(5 as libc::c_int) as s16,
                            fovTargetInit: 70.0f32,
                            lerpStepScale: 1.0f32,
                            atTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: -973.0f32,
                                              y: -924.0f32,
                                              z: -3263.0f32,};
                                    init
                                },
                            eyeTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: -1190.0f32,
                                              y: -1010.0f32,
                                              z: -3365.0f32,};
                                    init
                                },};
         init
     },
     {
         let mut init =
             OnePointCsFull{actionFlags: 0xf as libc::c_int as u8_0,
                            unk_01: 0x1 as libc::c_int as u8_0,
                            initFlags: 0x101 as libc::c_int as s16,
                            timerInit: 1 as libc::c_int as s16,
                            rollTargetInit: 0 as libc::c_int as s16,
                            fovTargetInit: 75.0f32,
                            lerpStepScale: 1.0f32,
                            atTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: -1355.0f32,
                                              y: -700.0f32,
                                              z: -3340.0f32,};
                                    init
                                },
                            eyeTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: -1040.0f32,
                                              y: -940.0f32,
                                              z: -3345.0f32,};
                                    init
                                },};
         init
     },
     {
         let mut init =
             OnePointCsFull{actionFlags: 0x2 as libc::c_int as u8_0,
                            unk_01: 0xff as libc::c_int as u8_0,
                            initFlags: 0x101 as libc::c_int as s16,
                            timerInit: 60 as libc::c_int as s16,
                            rollTargetInit: 0 as libc::c_int as s16,
                            fovTargetInit: 45.0f32,
                            lerpStepScale: 0.8f32,
                            atTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: -1370.0f32,
                                              y: -875.0f32,
                                              z: -3345.0f32,};
                                    init
                                },
                            eyeTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: -1230.0f32,
                                              y: -885.0f32,
                                              z: -3345.0f32,};
                                    init
                                },};
         init
     },
     {
         let mut init =
             OnePointCsFull{actionFlags: 0x1 as libc::c_int as u8_0,
                            unk_01: 0xff as libc::c_int as u8_0,
                            initFlags: 0x101 as libc::c_int as s16,
                            timerInit: 10 as libc::c_int as s16,
                            rollTargetInit: 0 as libc::c_int as s16,
                            fovTargetInit: 70.0f32,
                            lerpStepScale: 1.0f32,
                            atTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: -1370.0f32,
                                              y: -875.0f32,
                                              z: -3345.0f32,};
                                    init
                                },
                            eyeTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: -1210.0f32,
                                              y: -900.0f32,
                                              z: -3420.0f32,};
                                    init
                                },};
         init
     },
     {
         let mut init =
             OnePointCsFull{actionFlags: 0xf as libc::c_int as u8_0,
                            unk_01: 0xff as libc::c_int as u8_0,
                            initFlags: 0 as libc::c_int as s16,
                            timerInit: 20 as libc::c_int as s16,
                            rollTargetInit: 0 as libc::c_int as s16,
                            fovTargetInit: 70.0f32,
                            lerpStepScale: 1.0f32,
                            atTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: -1.0f32,
                                              y: -1.0f32,
                                              z: -1.0f32,};
                                    init
                                },
                            eyeTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: -1.0f32,
                                              y: -1.0f32,
                                              z: -1.0f32,};
                                    init
                                },};
         init
     },
     {
         let mut init =
             OnePointCsFull{actionFlags: 0x11 as libc::c_int as u8_0,
                            unk_01: 0xff as libc::c_int as u8_0,
                            initFlags: 0 as libc::c_int as s16,
                            timerInit: 1 as libc::c_int as s16,
                            rollTargetInit: 0 as libc::c_int as s16,
                            fovTargetInit: 60.0f32,
                            lerpStepScale: 1.0f32,
                            atTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: -1.0f32,
                                              y: -1.0f32,
                                              z: -1.0f32,};
                                    init
                                },
                            eyeTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: -1.0f32,
                                              y: -1.0f32,
                                              z: -1.0f32,};
                                    init
                                },};
         init
     }];
static mut D_80121774: [OnePointCsFull; 4] =
    [{
         let mut init =
             OnePointCsFull{actionFlags: 0xf as libc::c_int as u8_0,
                            unk_01: 0x8 as libc::c_int as u8_0,
                            initFlags: 0x101 as libc::c_int as s16,
                            timerInit: 1 as libc::c_int as s16,
                            rollTargetInit: -(2 as libc::c_int) as s16,
                            fovTargetInit: 75.0f32,
                            lerpStepScale: 1.0f32,
                            atTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: -1340.0f32,
                                              y: -860.0f32,
                                              z: -3345.0f32,};
                                    init
                                },
                            eyeTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: -1415.0f32,
                                              y: -940.0f32,
                                              z: -3520.0f32,};
                                    init
                                },};
         init
     },
     {
         let mut init =
             OnePointCsFull{actionFlags: 0x1 as libc::c_int as u8_0,
                            unk_01: 0x1 as libc::c_int as u8_0,
                            initFlags: 0x142 as libc::c_int as s16,
                            timerInit: 39 as libc::c_int as s16,
                            rollTargetInit: 2 as libc::c_int as s16,
                            fovTargetInit: 70.0f32,
                            lerpStepScale: 1.0f32,
                            atTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: 0.0f32,
                                              y: -20.0f32,
                                              z: 10.0f32,};
                                    init
                                },
                            eyeTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: -1140.0f32,
                                              y: -1010.0f32,
                                              z: -3560.0f32,};
                                    init
                                },};
         init
     },
     {
         let mut init =
             OnePointCsFull{actionFlags: 0x1 as libc::c_int as u8_0,
                            unk_01: 0x5 as libc::c_int as u8_0,
                            initFlags: 0x121 as libc::c_int as s16,
                            timerInit: 20 as libc::c_int as s16,
                            rollTargetInit: 0 as libc::c_int as s16,
                            fovTargetInit: 60.0f32,
                            lerpStepScale: 1.0f32,
                            atTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: 0.0f32,
                                              y: -20.0f32,
                                              z: 20.0f32,};
                                    init
                                },
                            eyeTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: -1220.0f32,
                                              y: -1005.0f32,
                                              z: -3660.0f32,};
                                    init
                                },};
         init
     },
     {
         let mut init =
             OnePointCsFull{actionFlags: 0x12 as libc::c_int as u8_0,
                            unk_01: 0xff as libc::c_int as u8_0,
                            initFlags: 0 as libc::c_int as s16,
                            timerInit: 1 as libc::c_int as s16,
                            rollTargetInit: 0 as libc::c_int as s16,
                            fovTargetInit: 60.0f32,
                            lerpStepScale: 1.0f32,
                            atTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: -1.0f32,
                                              y: -1.0f32,
                                              z: -1.0f32,};
                                    init
                                },
                            eyeTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: -1.0f32,
                                              y: -1.0f32,
                                              z: -1.0f32,};
                                    init
                                },};
         init
     }];
static mut D_80121814: [OnePointCsFull; 4] =
    [{
         let mut init =
             OnePointCsFull{actionFlags: 0xf as libc::c_int as u8_0,
                            unk_01: 0x4c as libc::c_int as u8_0,
                            initFlags: 0x101 as libc::c_int as s16,
                            timerInit: 5 as libc::c_int as s16,
                            rollTargetInit: 0 as libc::c_int as s16,
                            fovTargetInit: 40.0f32,
                            lerpStepScale: 1.0f32,
                            atTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: -1400.0f32,
                                              y: -540.0f32,
                                              z: -3327.0f32,};
                                    init
                                },
                            eyeTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: -1254.0f32,
                                              y: -20.0f32,
                                              z: -3357.0f32,};
                                    init
                                },};
         init
     },
     {
         let mut init =
             OnePointCsFull{actionFlags: 0x1 as libc::c_int as u8_0,
                            unk_01: 0xff as libc::c_int as u8_0,
                            initFlags: 0x101 as libc::c_int as s16,
                            timerInit: 70 as libc::c_int as s16,
                            rollTargetInit: 0 as libc::c_int as s16,
                            fovTargetInit: 75.0f32,
                            lerpStepScale: 0.75f32,
                            atTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: -1327.0f32,
                                              y: 100.0f32,
                                              z: -3342.0f32,};
                                    init
                                },
                            eyeTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: -1320.0f32,
                                              y: 350.0f32,
                                              z: -3540.0f32,};
                                    init
                                },};
         init
     },
     {
         let mut init =
             OnePointCsFull{actionFlags: 0x1 as libc::c_int as u8_0,
                            unk_01: 0xff as libc::c_int as u8_0,
                            initFlags: 0x2121 as libc::c_int as s16,
                            timerInit: 10 as libc::c_int as s16,
                            rollTargetInit: 0 as libc::c_int as s16,
                            fovTargetInit: 60.0f32,
                            lerpStepScale: 0.75f32,
                            atTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: 0.0f32,
                                              y: 10.0f32,
                                              z: 0.0f32,};
                                    init
                                },
                            eyeTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: 0.0f32,
                                              y: 20.0f32,
                                              z: -150.0f32,};
                                    init
                                },};
         init
     },
     {
         let mut init =
             OnePointCsFull{actionFlags: 0x12 as libc::c_int as u8_0,
                            unk_01: 0xff as libc::c_int as u8_0,
                            initFlags: 0 as libc::c_int as s16,
                            timerInit: 1 as libc::c_int as s16,
                            rollTargetInit: 0 as libc::c_int as s16,
                            fovTargetInit: 60.0f32,
                            lerpStepScale: 1.0f32,
                            atTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: -1.0f32,
                                              y: -1.0f32,
                                              z: -1.0f32,};
                                    init
                                },
                            eyeTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: -1.0f32,
                                              y: -1.0f32,
                                              z: -1.0f32,};
                                    init
                                },};
         init
     }];
static mut D_801218B4: [OnePointCsFull; 2] =
    [{
         let mut init =
             OnePointCsFull{actionFlags: 0xf as libc::c_int as u8_0,
                            unk_01: 0xff as libc::c_int as u8_0,
                            initFlags: 0x101 as libc::c_int as s16,
                            timerInit: 60 as libc::c_int as s16,
                            rollTargetInit: 0 as libc::c_int as s16,
                            fovTargetInit: 65.0f32,
                            lerpStepScale: 1.0f32,
                            atTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: 0.0f32,
                                              y: 350.0f32,
                                              z: -1520.0f32,};
                                    init
                                },
                            eyeTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: 0.0f32,
                                              y: 715.0f32,
                                              z: -885.0f32,};
                                    init
                                },};
         init
     },
     {
         let mut init =
             OnePointCsFull{actionFlags: 0x3 as libc::c_int as u8_0,
                            unk_01: 0xff as libc::c_int as u8_0,
                            initFlags: 0x101 as libc::c_int as s16,
                            timerInit: 100 as libc::c_int as s16,
                            rollTargetInit: 0 as libc::c_int as s16,
                            fovTargetInit: 70.0f32,
                            lerpStepScale: 0.02f32,
                            atTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: 0.0f32,
                                              y: 75.0f32,
                                              z: -1335.0f32,};
                                    init
                                },
                            eyeTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: 0.0f32,
                                              y: 20.0f32,
                                              z: -1190.0f32,};
                                    init
                                },};
         init
     }];
static mut D_80121904: [OnePointCsFull; 2] =
    [{
         let mut init =
             OnePointCsFull{actionFlags: 0xf as libc::c_int as u8_0,
                            unk_01: 0xff as libc::c_int as u8_0,
                            initFlags: 0x101 as libc::c_int as s16,
                            timerInit: 50 as libc::c_int as s16,
                            rollTargetInit: 10 as libc::c_int as s16,
                            fovTargetInit: 65.0f32,
                            lerpStepScale: 1.0f32,
                            atTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: 165.0f32,
                                              y: 85.0f32,
                                              z: -920.0f32,};
                                    init
                                },
                            eyeTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: 65.0f32,
                                              y: -30.0f32,
                                              z: -720.0f32,};
                                    init
                                },};
         init
     },
     {
         let mut init =
             OnePointCsFull{actionFlags: 0x11 as libc::c_int as u8_0,
                            unk_01: 0xff as libc::c_int as u8_0,
                            initFlags: 0 as libc::c_int as s16,
                            timerInit: 1 as libc::c_int as s16,
                            rollTargetInit: -(1 as libc::c_int) as s16,
                            fovTargetInit: -1.0f32,
                            lerpStepScale: -1.0f32,
                            atTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: -1.0f32,
                                              y: -1.0f32,
                                              z: -1.0f32,};
                                    init
                                },
                            eyeTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: -1.0f32,
                                              y: -1.0f32,
                                              z: -1.0f32,};
                                    init
                                },};
         init
     }];
static mut D_80121954: [[OnePointCsFull; 2]; 3] =
    [[{
          let mut init =
              OnePointCsFull{actionFlags: 0xf as libc::c_int as u8_0,
                             unk_01: 0xff as libc::c_int as u8_0,
                             initFlags: 0x101 as libc::c_int as s16,
                             timerInit: 20 as libc::c_int as s16,
                             rollTargetInit: 5 as libc::c_int as s16,
                             fovTargetInit: 60.0f32,
                             lerpStepScale: 1.0f32,
                             atTargetInit:
                                 {
                                     let mut init =
                                         Vec3f{x: -700.0f32,
                                               y: 940.0f32,
                                               z: 300.0f32,};
                                     init
                                 },
                             eyeTargetInit:
                                 {
                                     let mut init =
                                         Vec3f{x: -765.0f32,
                                               y: 1000.0f32,
                                               z: 335.0f32,};
                                     init
                                 },};
          init
      },
      {
          let mut init =
              OnePointCsFull{actionFlags: 0x3 as libc::c_int as u8_0,
                             unk_01: 0xff as libc::c_int as u8_0,
                             initFlags: 0x101 as libc::c_int as s16,
                             timerInit: 80 as libc::c_int as s16,
                             rollTargetInit: -(10 as libc::c_int) as s16,
                             fovTargetInit: 70.0f32,
                             lerpStepScale: 0.1f32,
                             atTargetInit:
                                 {
                                     let mut init =
                                         Vec3f{x: -540.0f32,
                                               y: 875.0f32,
                                               z: 245.0f32,};
                                     init
                                 },
                             eyeTargetInit:
                                 {
                                     let mut init =
                                         Vec3f{x: -585.0f32,
                                               y: 900.0f32,
                                               z: 335.0f32,};
                                     init
                                 },};
          init
      }],
     [{
          let mut init =
              OnePointCsFull{actionFlags: 0xf as libc::c_int as u8_0,
                             unk_01: 0xff as libc::c_int as u8_0,
                             initFlags: 0x101 as libc::c_int as s16,
                             timerInit: 40 as libc::c_int as s16,
                             rollTargetInit: -(30 as libc::c_int) as s16,
                             fovTargetInit: 70.0f32,
                             lerpStepScale: 1.0f32,
                             atTargetInit:
                                 {
                                     let mut init =
                                         Vec3f{x: -80.0f32,
                                               y: 115.0f32,
                                               z: -180.0f32,};
                                     init
                                 },
                             eyeTargetInit:
                                 {
                                     let mut init =
                                         Vec3f{x: -5.0f32,
                                               y: 240.0f32,
                                               z: -190.0f32,};
                                     init
                                 },};
          init
      },
      {
          let mut init =
              OnePointCsFull{actionFlags: 0xb as libc::c_int as u8_0,
                             unk_01: 0xff as libc::c_int as u8_0,
                             initFlags: 0x101 as libc::c_int as s16,
                             timerInit: 60 as libc::c_int as s16,
                             rollTargetInit: 20 as libc::c_int as s16,
                             fovTargetInit: 70.0f32,
                             lerpStepScale: 0.1f32,
                             atTargetInit:
                                 {
                                     let mut init =
                                         Vec3f{x: -100.0f32,
                                               y: 350.0f32,
                                               z: -175.0f32,};
                                     init
                                 },
                             eyeTargetInit:
                                 {
                                     let mut init =
                                         Vec3f{x: -5.0f32,
                                               y: 240.0f32,
                                               z: -190.0f32,};
                                     init
                                 },};
          init
      }],
     [{
          let mut init =
              OnePointCsFull{actionFlags: 0x3 as libc::c_int as u8_0,
                             unk_01: 0xff as libc::c_int as u8_0,
                             initFlags: 0x101 as libc::c_int as s16,
                             timerInit: 80 as libc::c_int as s16,
                             rollTargetInit: 5 as libc::c_int as s16,
                             fovTargetInit: 70.0f32,
                             lerpStepScale: 0.2f32,
                             atTargetInit:
                                 {
                                     let mut init =
                                         Vec3f{x: 960.0f32,
                                               y: 900.0f32,
                                               z: 260.0f32,};
                                     init
                                 },
                             eyeTargetInit:
                                 {
                                     let mut init =
                                         Vec3f{x: 970.0f32,
                                               y: 950.0f32,
                                               z: 250.0f32,};
                                     init
                                 },};
          init
      },
      {
          let mut init =
              OnePointCsFull{actionFlags: 0xf as libc::c_int as u8_0,
                             unk_01: 0xff as libc::c_int as u8_0,
                             initFlags: 0x101 as libc::c_int as s16,
                             timerInit: 20 as libc::c_int as s16,
                             rollTargetInit: 5 as libc::c_int as s16,
                             fovTargetInit: 70.0f32,
                             lerpStepScale: 1.0f32,
                             atTargetInit:
                                 {
                                     let mut init =
                                         Vec3f{x: 960.0f32,
                                               y: 900.0f32,
                                               z: 260.0f32,};
                                     init
                                 },
                             eyeTargetInit:
                                 {
                                     let mut init =
                                         Vec3f{x: 970.0f32,
                                               y: 950.0f32,
                                               z: 250.0f32,};
                                     init
                                 },};
          init
      }]];
static mut D_80121A44: [OnePointCsFull; 12] =
    [{
         let mut init =
             OnePointCsFull{actionFlags: 0x4f as libc::c_int as u8_0,
                            unk_01: 0x5 as libc::c_int as u8_0,
                            initFlags: 0x2121 as libc::c_int as s16,
                            timerInit: 10 as libc::c_int as s16,
                            rollTargetInit: 0 as libc::c_int as s16,
                            fovTargetInit: 60.0f32,
                            lerpStepScale: 1.0f32,
                            atTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: 0.0f32,
                                              y: -5.0f32,
                                              z: 0.0f32,};
                                    init
                                },
                            eyeTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: 0.0f32,
                                              y: 0.0f32,
                                              z: -80.0f32,};
                                    init
                                },};
         init
     },
     {
         let mut init =
             OnePointCsFull{actionFlags: 0x42 as libc::c_int as u8_0,
                            unk_01: 0x1 as libc::c_int as u8_0,
                            initFlags: 0x4242 as libc::c_int as s16,
                            timerInit: 30 as libc::c_int as s16,
                            rollTargetInit: 0 as libc::c_int as s16,
                            fovTargetInit: 50.0f32,
                            lerpStepScale: 1.0f32,
                            atTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: 0.0f32,
                                              y: 45.0f32,
                                              z: 0.0f32,};
                                    init
                                },
                            eyeTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: 0.0f32,
                                              y: 45.0f32,
                                              z: 50.0f32,};
                                    init
                                },};
         init
     },
     {
         let mut init =
             OnePointCsFull{actionFlags: 0x55 as libc::c_int as u8_0,
                            unk_01: 0xff as libc::c_int as u8_0,
                            initFlags: 0 as libc::c_int as s16,
                            timerInit: 1 as libc::c_int as s16,
                            rollTargetInit: 0 as libc::c_int as s16,
                            fovTargetInit: 60.0f32,
                            lerpStepScale: 1.0f32,
                            atTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: -1.0f32,
                                              y: -1.0f32,
                                              z: -1.0f32,};
                                    init
                                },
                            eyeTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: -1.0f32,
                                              y: -1.0f32,
                                              z: -1.0f32,};
                                    init
                                },};
         init
     },
     {
         let mut init =
             OnePointCsFull{actionFlags: 0x4f as libc::c_int as u8_0,
                            unk_01: 0x5 as libc::c_int as u8_0,
                            initFlags: 0x2222 as libc::c_int as s16,
                            timerInit: 40 as libc::c_int as s16,
                            rollTargetInit: 5 as libc::c_int as s16,
                            fovTargetInit: 50.0f32,
                            lerpStepScale: 1.0f32,
                            atTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: 0.0f32,
                                              y: 50.0f32,
                                              z: 0.0f32,};
                                    init
                                },
                            eyeTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: 0.0f32,
                                              y: 50.0f32,
                                              z: 50.0f32,};
                                    init
                                },};
         init
     },
     {
         let mut init =
             OnePointCsFull{actionFlags: 0x4f as libc::c_int as u8_0,
                            unk_01: 0x1 as libc::c_int as u8_0,
                            initFlags: 0x4242 as libc::c_int as s16,
                            timerInit: 40 as libc::c_int as s16,
                            rollTargetInit: 5 as libc::c_int as s16,
                            fovTargetInit: 60.0f32,
                            lerpStepScale: 1.0f32,
                            atTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: 30.0f32,
                                              y: 30.0f32,
                                              z: 15.0f32,};
                                    init
                                },
                            eyeTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: 70.0f32,
                                              y: 30.0f32,
                                              z: -40.0f32,};
                                    init
                                },};
         init
     },
     {
         let mut init =
             OnePointCsFull{actionFlags: 0x55 as libc::c_int as u8_0,
                            unk_01: 0xff as libc::c_int as u8_0,
                            initFlags: 0 as libc::c_int as s16,
                            timerInit: 1 as libc::c_int as s16,
                            rollTargetInit: 0 as libc::c_int as s16,
                            fovTargetInit: 60.0f32,
                            lerpStepScale: 1.0f32,
                            atTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: -1.0f32,
                                              y: -1.0f32,
                                              z: -1.0f32,};
                                    init
                                },
                            eyeTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: -1.0f32,
                                              y: -1.0f32,
                                              z: -1.0f32,};
                                    init
                                },};
         init
     },
     {
         let mut init =
             OnePointCsFull{actionFlags: 0x4f as libc::c_int as u8_0,
                            unk_01: 0xff as libc::c_int as u8_0,
                            initFlags: 0x4242 as libc::c_int as s16,
                            timerInit: 30 as libc::c_int as s16,
                            rollTargetInit: -(5 as libc::c_int) as s16,
                            fovTargetInit: 50.0f32,
                            lerpStepScale: 1.0f32,
                            atTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: 20.0f32,
                                              y: 30.0f32,
                                              z: -5.0f32,};
                                    init
                                },
                            eyeTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: 0.0f32,
                                              y: 70.0f32,
                                              z: 70.0f32,};
                                    init
                                },};
         init
     },
     {
         let mut init =
             OnePointCsFull{actionFlags: 0x50 as libc::c_int as u8_0,
                            unk_01: 0xff as libc::c_int as u8_0,
                            initFlags: 0 as libc::c_int as s16,
                            timerInit: 1 as libc::c_int as s16,
                            rollTargetInit: 0 as libc::c_int as s16,
                            fovTargetInit: 60.0f32,
                            lerpStepScale: 1.0f32,
                            atTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: -1.0f32,
                                              y: -1.0f32,
                                              z: -1.0f32,};
                                    init
                                },
                            eyeTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: -1.0f32,
                                              y: -1.0f32,
                                              z: -1.0f32,};
                                    init
                                },};
         init
     },
     {
         let mut init =
             OnePointCsFull{actionFlags: 0x4f as libc::c_int as u8_0,
                            unk_01: 0x1 as libc::c_int as u8_0,
                            initFlags: 0x2242 as libc::c_int as s16,
                            timerInit: 40 as libc::c_int as s16,
                            rollTargetInit: 0 as libc::c_int as s16,
                            fovTargetInit: 45.0f32,
                            lerpStepScale: 1.0f32,
                            atTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: 0.0f32,
                                              y: 30.0f32,
                                              z: 30.0f32,};
                                    init
                                },
                            eyeTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: 25.0f32,
                                              y: 60.0f32,
                                              z: -60.0f32,};
                                    init
                                },};
         init
     },
     {
         let mut init =
             OnePointCsFull{actionFlags: 0x4b as libc::c_int as u8_0,
                            unk_01: 0x1 as libc::c_int as u8_0,
                            initFlags: 0x22c2 as libc::c_int as s16,
                            timerInit: 140 as libc::c_int as s16,
                            rollTargetInit: 0 as libc::c_int as s16,
                            fovTargetInit: 60.0f32,
                            lerpStepScale: 0.04f32,
                            atTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: 0.0f32,
                                              y: 0.0f32,
                                              z: 30.0f32,};
                                    init
                                },
                            eyeTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: 25.0f32,
                                              y: 60.0f32,
                                              z: -60.0f32,};
                                    init
                                },};
         init
     },
     {
         let mut init =
             OnePointCsFull{actionFlags: 0x49 as libc::c_int as u8_0,
                            unk_01: 0xff as libc::c_int as u8_0,
                            initFlags: 0x2222 as libc::c_int as s16,
                            timerInit: 20 as libc::c_int as s16,
                            rollTargetInit: 0 as libc::c_int as s16,
                            fovTargetInit: 60.0f32,
                            lerpStepScale: 0.8f32,
                            atTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: 0.0f32,
                                              y: 50.0f32,
                                              z: 0.0f32,};
                                    init
                                },
                            eyeTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: 0.0f32,
                                              y: 60.0f32,
                                              z: -60.0f32,};
                                    init
                                },};
         init
     },
     {
         let mut init =
             OnePointCsFull{actionFlags: 0x12 as libc::c_int as u8_0,
                            unk_01: 0xff as libc::c_int as u8_0,
                            initFlags: 0 as libc::c_int as s16,
                            timerInit: 1 as libc::c_int as s16,
                            rollTargetInit: 0 as libc::c_int as s16,
                            fovTargetInit: 60.0f32,
                            lerpStepScale: 1.0f32,
                            atTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: -1.0f32,
                                              y: -1.0f32,
                                              z: -1.0f32,};
                                    init
                                },
                            eyeTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: -1.0f32,
                                              y: -1.0f32,
                                              z: -1.0f32,};
                                    init
                                },};
         init
     }];
static mut D_80121C24: [OnePointCsFull; 7] =
    [{
         let mut init =
             OnePointCsFull{actionFlags: 0xf as libc::c_int as u8_0,
                            unk_01: 0x5 as libc::c_int as u8_0,
                            initFlags: 0x101 as libc::c_int as s16,
                            timerInit: 1 as libc::c_int as s16,
                            rollTargetInit: 0 as libc::c_int as s16,
                            fovTargetInit: 60.0f32,
                            lerpStepScale: 1.0f32,
                            atTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: 0.0f32,
                                              y: 0.0f32,
                                              z: 0.0f32,};
                                    init
                                },
                            eyeTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: 0.0f32,
                                              y: 0.0f32,
                                              z: 0.0f32,};
                                    init
                                },};
         init
     },
     {
         let mut init =
             OnePointCsFull{actionFlags: 0x3 as libc::c_int as u8_0,
                            unk_01: 0xff as libc::c_int as u8_0,
                            initFlags: 0x101 as libc::c_int as s16,
                            timerInit: 89 as libc::c_int as s16,
                            rollTargetInit: 0 as libc::c_int as s16,
                            fovTargetInit: 50.0f32,
                            lerpStepScale: 0.4f32,
                            atTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: 125.0f32,
                                              y: 320.0f32,
                                              z: -1500.0f32,};
                                    init
                                },
                            eyeTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: 125.0f32,
                                              y: 500.0f32,
                                              z: -1150.0f32,};
                                    init
                                },};
         init
     },
     {
         let mut init =
             OnePointCsFull{actionFlags: 0xf as libc::c_int as u8_0,
                            unk_01: 0x8 as libc::c_int as u8_0,
                            initFlags: 0x101 as libc::c_int as s16,
                            timerInit: 40 as libc::c_int as s16,
                            rollTargetInit: 4 as libc::c_int as s16,
                            fovTargetInit: 55.0f32,
                            lerpStepScale: 1.0f32,
                            atTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: 0.0f32,
                                              y: 375.0f32,
                                              z: -1440.0f32,};
                                    init
                                },
                            eyeTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: 5.0f32,
                                              y: 365.0f32,
                                              z: -1315.0f32,};
                                    init
                                },};
         init
     },
     {
         let mut init =
             OnePointCsFull{actionFlags: 0xf as libc::c_int as u8_0,
                            unk_01: 0xff as libc::c_int as u8_0,
                            initFlags: 0x101 as libc::c_int as s16,
                            timerInit: 40 as libc::c_int as s16,
                            rollTargetInit: -(4 as libc::c_int) as s16,
                            fovTargetInit: 55.0f32,
                            lerpStepScale: 1.0f32,
                            atTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: 250.0f32,
                                              y: 375.0f32,
                                              z: -1440.0f32,};
                                    init
                                },
                            eyeTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: 235.0f32,
                                              y: 365.0f32,
                                              z: -1315.0f32,};
                                    init
                                },};
         init
     },
     {
         let mut init =
             OnePointCsFull{actionFlags: 0xf as libc::c_int as u8_0,
                            unk_01: 0xff as libc::c_int as u8_0,
                            initFlags: 0x101 as libc::c_int as s16,
                            timerInit: 100 as libc::c_int as s16,
                            rollTargetInit: 0 as libc::c_int as s16,
                            fovTargetInit: 95.0f32,
                            lerpStepScale: 1.0f32,
                            atTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: 125.0f32,
                                              y: 345.0f32,
                                              z: -1500.0f32,};
                                    init
                                },
                            eyeTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: 125.0f32,
                                              y: 255.0f32,
                                              z: -1350.0f32,};
                                    init
                                },};
         init
     },
     {
         let mut init =
             OnePointCsFull{actionFlags: 0x2 as libc::c_int as u8_0,
                            unk_01: 0xff as libc::c_int as u8_0,
                            initFlags: 0x101 as libc::c_int as s16,
                            timerInit: 100 as libc::c_int as s16,
                            rollTargetInit: 0 as libc::c_int as s16,
                            fovTargetInit: 60.0f32,
                            lerpStepScale: 1.0f32,
                            atTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: 125.0f32,
                                              y: 325.0f32,
                                              z: -1500.0f32,};
                                    init
                                },
                            eyeTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: 125.0f32,
                                              y: 480.0f32,
                                              z: -1000.0f32,};
                                    init
                                },};
         init
     },
     {
         let mut init =
             OnePointCsFull{actionFlags: 0x11 as libc::c_int as u8_0,
                            unk_01: 0xff as libc::c_int as u8_0,
                            initFlags: 0 as libc::c_int as s16,
                            timerInit: 1 as libc::c_int as s16,
                            rollTargetInit: 0 as libc::c_int as s16,
                            fovTargetInit: 60.0f32,
                            lerpStepScale: 1.0f32,
                            atTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: -1.0f32,
                                              y: -1.0f32,
                                              z: -1.0f32,};
                                    init
                                },
                            eyeTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: -1.0f32,
                                              y: -1.0f32,
                                              z: -1.0f32,};
                                    init
                                },};
         init
     }];
static mut D_80121D3C: [OnePointCsFull; 3] =
    [{
         let mut init =
             OnePointCsFull{actionFlags: 0xf as libc::c_int as u8_0,
                            unk_01: 0xff as libc::c_int as u8_0,
                            initFlags: 0x101 as libc::c_int as s16,
                            timerInit: 1 as libc::c_int as s16,
                            rollTargetInit: 0 as libc::c_int as s16,
                            fovTargetInit: 60.0f32,
                            lerpStepScale: 1.0f32,
                            atTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: 1023.0f32,
                                              y: 738.0f32,
                                              z: -2628.0f32,};
                                    init
                                },
                            eyeTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: 993.0f32,
                                              y: 770.0f32,
                                              z: -2740.0f32,};
                                    init
                                },};
         init
     },
     {
         let mut init =
             OnePointCsFull{actionFlags: 0x2 as libc::c_int as u8_0,
                            unk_01: 0xff as libc::c_int as u8_0,
                            initFlags: 0x101 as libc::c_int as s16,
                            timerInit: 4 as libc::c_int as s16,
                            rollTargetInit: 0 as libc::c_int as s16,
                            fovTargetInit: 50.0f32,
                            lerpStepScale: 1.0f32,
                            atTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: 1255.0f32,
                                              y: 350.0f32,
                                              z: -1870.0f32,};
                                    init
                                },
                            eyeTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: 1240.0f32,
                                              y: 575.0f32,
                                              z: -2100.0f32,};
                                    init
                                },};
         init
     },
     {
         let mut init =
             OnePointCsFull{actionFlags: 0xf as libc::c_int as u8_0,
                            unk_01: 0xff as libc::c_int as u8_0,
                            initFlags: 0 as libc::c_int as s16,
                            timerInit: -(1 as libc::c_int) as s16,
                            rollTargetInit: 0 as libc::c_int as s16,
                            fovTargetInit: 75.0f32,
                            lerpStepScale: 1.0f32,
                            atTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: -1.0f32,
                                              y: -1.0f32,
                                              z: -1.0f32,};
                                    init
                                },
                            eyeTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: -1.0f32,
                                              y: -1.0f32,
                                              z: -1.0f32,};
                                    init
                                },};
         init
     }];
static mut D_80121DB4: [OnePointCsFull; 9] =
    [{
         let mut init =
             OnePointCsFull{actionFlags: 0xf as libc::c_int as u8_0,
                            unk_01: 0xff as libc::c_int as u8_0,
                            initFlags: 0x101 as libc::c_int as s16,
                            timerInit: 40 as libc::c_int as s16,
                            rollTargetInit: 0 as libc::c_int as s16,
                            fovTargetInit: 70.0f32,
                            lerpStepScale: 1.0f32,
                            atTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: 4290.0f32,
                                              y: -1332.0f32,
                                              z: -1900.0f32,};
                                    init
                                },
                            eyeTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: 4155.0f32,
                                              y: -1360.0f32,
                                              z: -1840.0f32,};
                                    init
                                },};
         init
     },
     {
         let mut init =
             OnePointCsFull{actionFlags: 0x2 as libc::c_int as u8_0,
                            unk_01: 0xff as libc::c_int as u8_0,
                            initFlags: 0x101 as libc::c_int as s16,
                            timerInit: 60 as libc::c_int as s16,
                            rollTargetInit: 0 as libc::c_int as s16,
                            fovTargetInit: 70.0f32,
                            lerpStepScale: 1.0f32,
                            atTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: 4215.0f32,
                                              y: -975.0f32,
                                              z: -2095.0f32,};
                                    init
                                },
                            eyeTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: 4070.0f32,
                                              y: -1000.0f32,
                                              z: -2025.0f32,};
                                    init
                                },};
         init
     },
     {
         let mut init =
             OnePointCsFull{actionFlags: 0xf as libc::c_int as u8_0,
                            unk_01: 0xff as libc::c_int as u8_0,
                            initFlags: 0x101 as libc::c_int as s16,
                            timerInit: 5 as libc::c_int as s16,
                            rollTargetInit: 0 as libc::c_int as s16,
                            fovTargetInit: 70.0f32,
                            lerpStepScale: 1.0f32,
                            atTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: 4215.0f32,
                                              y: -975.0f32,
                                              z: -2095.0f32,};
                                    init
                                },
                            eyeTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: 4070.0f32,
                                              y: -1000.0f32,
                                              z: -2025.0f32,};
                                    init
                                },};
         init
     },
     {
         let mut init =
             OnePointCsFull{actionFlags: 0xf as libc::c_int as u8_0,
                            unk_01: 0xff as libc::c_int as u8_0,
                            initFlags: 0x101 as libc::c_int as s16,
                            timerInit: 80 as libc::c_int as s16,
                            rollTargetInit: 8 as libc::c_int as s16,
                            fovTargetInit: 75.0f32,
                            lerpStepScale: 1.0f32,
                            atTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: 4010.0f32,
                                              y: -1152.0f32,
                                              z: -1728.0f32,};
                                    init
                                },
                            eyeTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: 3997.0f32,
                                              y: -1194.0f32,
                                              z: -1629.0f32,};
                                    init
                                },};
         init
     },
     {
         let mut init =
             OnePointCsFull{actionFlags: 0xf as libc::c_int as u8_0,
                            unk_01: 0x39 as libc::c_int as u8_0,
                            initFlags: 0x2121 as libc::c_int as s16,
                            timerInit: 1 as libc::c_int as s16,
                            rollTargetInit: 8 as libc::c_int as s16,
                            fovTargetInit: 75.0f32,
                            lerpStepScale: 1.0f32,
                            atTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: 20.0f32,
                                              y: 20.0f32,
                                              z: 0.0f32,};
                                    init
                                },
                            eyeTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: 50.0f32,
                                              y: 30.0f32,
                                              z: 200.0f32,};
                                    init
                                },};
         init
     },
     {
         let mut init =
             OnePointCsFull{actionFlags: 0x4 as libc::c_int as u8_0,
                            unk_01: 0xff as libc::c_int as u8_0,
                            initFlags: 0x2121 as libc::c_int as s16,
                            timerInit: 99 as libc::c_int as s16,
                            rollTargetInit: 2 as libc::c_int as s16,
                            fovTargetInit: 70.0f32,
                            lerpStepScale: 0.02f32,
                            atTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: -20.0f32,
                                              y: 0.0f32,
                                              z: 20.0f32,};
                                    init
                                },
                            eyeTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: 300.0f32,
                                              y: 50.0f32,
                                              z: -500.0f32,};
                                    init
                                },};
         init
     },
     {
         let mut init =
             OnePointCsFull{actionFlags: 0x9 as libc::c_int as u8_0,
                            unk_01: 0x38 as libc::c_int as u8_0,
                            initFlags: 0x2121 as libc::c_int as s16,
                            timerInit: 149 as libc::c_int as s16,
                            rollTargetInit: -(20 as libc::c_int) as s16,
                            fovTargetInit: 70.0f32,
                            lerpStepScale: 0.1f32,
                            atTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: 100.0f32,
                                              y: 50.0f32,
                                              z: -100.0f32,};
                                    init
                                },
                            eyeTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: 5000.0f32,
                                              y: 1055.0f32,
                                              z: -2250.0f32,};
                                    init
                                },};
         init
     },
     {
         let mut init =
             OnePointCsFull{actionFlags: 0xf as libc::c_int as u8_0,
                            unk_01: 0xff as libc::c_int as u8_0,
                            initFlags: 0x2121 as libc::c_int as s16,
                            timerInit: 1 as libc::c_int as s16,
                            rollTargetInit: 0 as libc::c_int as s16,
                            fovTargetInit: 60.0f32,
                            lerpStepScale: 1.0f32,
                            atTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: 0.0f32,
                                              y: -20.0f32,
                                              z: 0.0f32,};
                                    init
                                },
                            eyeTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: 0.0f32,
                                              y: 20.0f32,
                                              z: -150.0f32,};
                                    init
                                },};
         init
     },
     {
         let mut init =
             OnePointCsFull{actionFlags: 0x12 as libc::c_int as u8_0,
                            unk_01: 0xff as libc::c_int as u8_0,
                            initFlags: 0 as libc::c_int as s16,
                            timerInit: 1 as libc::c_int as s16,
                            rollTargetInit: -(1 as libc::c_int) as s16,
                            fovTargetInit: -1.0f32,
                            lerpStepScale: -1.0f32,
                            atTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: -1.0f32,
                                              y: -1.0f32,
                                              z: -1.0f32,};
                                    init
                                },
                            eyeTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: -1.0f32,
                                              y: -1.0f32,
                                              z: -1.0f32,};
                                    init
                                },};
         init
     }];
static mut D_80121F1C: [OnePointCsFull; 4] =
    [{
         let mut init =
             OnePointCsFull{actionFlags: 0xf as libc::c_int as u8_0,
                            unk_01: 0x8 as libc::c_int as u8_0,
                            initFlags: 0x101 as libc::c_int as s16,
                            timerInit: 10 as libc::c_int as s16,
                            rollTargetInit: 0 as libc::c_int as s16,
                            fovTargetInit: 60.0f32,
                            lerpStepScale: 1.0f32,
                            atTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: 0.0f32,
                                              y: 0.0f32,
                                              z: 0.0f32,};
                                    init
                                },
                            eyeTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: 0.0f32,
                                              y: 0.0f32,
                                              z: 0.0f32,};
                                    init
                                },};
         init
     },
     {
         let mut init =
             OnePointCsFull{actionFlags: 0x1 as libc::c_int as u8_0,
                            unk_01: 0xff as libc::c_int as u8_0,
                            initFlags: 0x2121 as libc::c_int as s16,
                            timerInit: 10 as libc::c_int as s16,
                            rollTargetInit: 0 as libc::c_int as s16,
                            fovTargetInit: 50.0f32,
                            lerpStepScale: 0.5f32,
                            atTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: 0.0f32,
                                              y: 0.0f32,
                                              z: 0.0f32,};
                                    init
                                },
                            eyeTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: 0.0f32,
                                              y: 0.0f32,
                                              z: 150.0f32,};
                                    init
                                },};
         init
     },
     {
         let mut init =
             OnePointCsFull{actionFlags: 0x1 as libc::c_int as u8_0,
                            unk_01: 0x2 as libc::c_int as u8_0,
                            initFlags: 0x2121 as libc::c_int as s16,
                            timerInit: 23 as libc::c_int as s16,
                            rollTargetInit: 0 as libc::c_int as s16,
                            fovTargetInit: 50.0f32,
                            lerpStepScale: 0.5f32,
                            atTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: 0.0f32,
                                              y: 0.0f32,
                                              z: 0.0f32,};
                                    init
                                },
                            eyeTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: 0.0f32,
                                              y: 0.0f32,
                                              z: 150.0f32,};
                                    init
                                },};
         init
     },
     {
         let mut init =
             OnePointCsFull{actionFlags: 0x11 as libc::c_int as u8_0,
                            unk_01: 0xff as libc::c_int as u8_0,
                            initFlags: 0 as libc::c_int as s16,
                            timerInit: 1 as libc::c_int as s16,
                            rollTargetInit: -(1 as libc::c_int) as s16,
                            fovTargetInit: -1.0f32,
                            lerpStepScale: -1.0f32,
                            atTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: -1.0f32,
                                              y: -1.0f32,
                                              z: -1.0f32,};
                                    init
                                },
                            eyeTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: -1.0f32,
                                              y: -1.0f32,
                                              z: -1.0f32,};
                                    init
                                },};
         init
     }];
static mut D_80121FBC: [OnePointCsFull; 4] =
    [{
         let mut init =
             OnePointCsFull{actionFlags: 0xf as libc::c_int as u8_0,
                            unk_01: 0xff as libc::c_int as u8_0,
                            initFlags: 0x101 as libc::c_int as s16,
                            timerInit: 5 as libc::c_int as s16,
                            rollTargetInit: 0 as libc::c_int as s16,
                            fovTargetInit: 60.0f32,
                            lerpStepScale: 1.0f32,
                            atTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: 0.0f32,
                                              y: 0.0f32,
                                              z: 0.0f32,};
                                    init
                                },
                            eyeTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: 0.0f32,
                                              y: 0.0f32,
                                              z: 0.0f32,};
                                    init
                                },};
         init
     },
     {
         let mut init =
             OnePointCsFull{actionFlags: 0x1 as libc::c_int as u8_0,
                            unk_01: 0xff as libc::c_int as u8_0,
                            initFlags: 0x101 as libc::c_int as s16,
                            timerInit: 10 as libc::c_int as s16,
                            rollTargetInit: 0 as libc::c_int as s16,
                            fovTargetInit: 30.0f32,
                            lerpStepScale: 1.0f32,
                            atTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: -2130.0f32,
                                              y: 2885.0f32,
                                              z: -1055.0f32,};
                                    init
                                },
                            eyeTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: -2085.0f32,
                                              y: 2875.0f32,
                                              z: -1145.0f32,};
                                    init
                                },};
         init
     },
     {
         let mut init =
             OnePointCsFull{actionFlags: 0xf as libc::c_int as u8_0,
                            unk_01: 0xff as libc::c_int as u8_0,
                            initFlags: 0 as libc::c_int as s16,
                            timerInit: 30 as libc::c_int as s16,
                            rollTargetInit: 0 as libc::c_int as s16,
                            fovTargetInit: 60.0f32,
                            lerpStepScale: 1.0f32,
                            atTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: -1.0f32,
                                              y: -1.0f32,
                                              z: -1.0f32,};
                                    init
                                },
                            eyeTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: -1.0f32,
                                              y: -1.0f32,
                                              z: -1.0f32,};
                                    init
                                },};
         init
     },
     {
         let mut init =
             OnePointCsFull{actionFlags: 0x12 as libc::c_int as u8_0,
                            unk_01: 0xff as libc::c_int as u8_0,
                            initFlags: 0 as libc::c_int as s16,
                            timerInit: 1 as libc::c_int as s16,
                            rollTargetInit: -(1 as libc::c_int) as s16,
                            fovTargetInit: -1.0f32,
                            lerpStepScale: -1.0f32,
                            atTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: -1.0f32,
                                              y: -1.0f32,
                                              z: -1.0f32,};
                                    init
                                },
                            eyeTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: -1.0f32,
                                              y: -1.0f32,
                                              z: -1.0f32,};
                                    init
                                },};
         init
     }];
static mut D_8012205C: [OnePointCsFull; 3] =
    [{
         let mut init =
             OnePointCsFull{actionFlags: 0xf as libc::c_int as u8_0,
                            unk_01: 0xff as libc::c_int as u8_0,
                            initFlags: 0x42c2 as libc::c_int as s16,
                            timerInit: 1 as libc::c_int as s16,
                            rollTargetInit: 0 as libc::c_int as s16,
                            fovTargetInit: 50.0f32,
                            lerpStepScale: 1.0f32,
                            atTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: 0.0f32,
                                              y: 220.0f32,
                                              z: 0.0f32,};
                                    init
                                },
                            eyeTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: 0.0f32,
                                              y: 220.0f32,
                                              z: 240.0f32,};
                                    init
                                },};
         init
     },
     {
         let mut init =
             OnePointCsFull{actionFlags: 0xf as libc::c_int as u8_0,
                            unk_01: 0xff as libc::c_int as u8_0,
                            initFlags: 0x80 as libc::c_int as s16,
                            timerInit: 29 as libc::c_int as s16,
                            rollTargetInit: 0 as libc::c_int as s16,
                            fovTargetInit: 50.0f32,
                            lerpStepScale: 1.0f32,
                            atTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: 0.0f32,
                                              y: 220.0f32,
                                              z: 0.0f32,};
                                    init
                                },
                            eyeTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: 0.0f32,
                                              y: 220.0f32,
                                              z: 240.0f32,};
                                    init
                                },};
         init
     },
     {
         let mut init =
             OnePointCsFull{actionFlags: 0x1 as libc::c_int as u8_0,
                            unk_01: 0x1 as libc::c_int as u8_0,
                            initFlags: 0x21a1 as libc::c_int as s16,
                            timerInit: 10 as libc::c_int as s16,
                            rollTargetInit: 0 as libc::c_int as s16,
                            fovTargetInit: 60.0f32,
                            lerpStepScale: 1.0f32,
                            atTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: 0.0f32,
                                              y: -10.0f32,
                                              z: 0.0f32,};
                                    init
                                },
                            eyeTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: 0.0f32,
                                              y: 10.0f32,
                                              z: -200.0f32,};
                                    init
                                },};
         init
     }];
static mut D_801220D4: [OnePointCsFull; 5] =
    [{
         let mut init =
             OnePointCsFull{actionFlags: 0xf as libc::c_int as u8_0,
                            unk_01: 0x1 as libc::c_int as u8_0,
                            initFlags: 0x101 as libc::c_int as s16,
                            timerInit: 5 as libc::c_int as s16,
                            rollTargetInit: 0 as libc::c_int as s16,
                            fovTargetInit: 60.0f32,
                            lerpStepScale: 1.0f32,
                            atTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: 0.0f32,
                                              y: 0.0f32,
                                              z: 0.0f32,};
                                    init
                                },
                            eyeTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: 0.0f32,
                                              y: 0.0f32,
                                              z: 0.0f32,};
                                    init
                                },};
         init
     },
     {
         let mut init =
             OnePointCsFull{actionFlags: 0x1 as libc::c_int as u8_0,
                            unk_01: 0xff as libc::c_int as u8_0,
                            initFlags: 0x4141 as libc::c_int as s16,
                            timerInit: 10 as libc::c_int as s16,
                            rollTargetInit: 5 as libc::c_int as s16,
                            fovTargetInit: 55.0f32,
                            lerpStepScale: 0.75f32,
                            atTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: 400.0f32,
                                              y: -50.0f32,
                                              z: 800.0f32,};
                                    init
                                },
                            eyeTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: 600.0f32,
                                              y: -60.0f32,
                                              z: 800.0f32,};
                                    init
                                },};
         init
     },
     {
         let mut init =
             OnePointCsFull{actionFlags: 0x1 as libc::c_int as u8_0,
                            unk_01: 0xff as libc::c_int as u8_0,
                            initFlags: 0x4141 as libc::c_int as s16,
                            timerInit: 15 as libc::c_int as s16,
                            rollTargetInit: 10 as libc::c_int as s16,
                            fovTargetInit: 40.0f32,
                            lerpStepScale: 0.75f32,
                            atTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: 0.0f32,
                                              y: 0.0f32,
                                              z: 0.0f32,};
                                    init
                                },
                            eyeTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: 0.0f32,
                                              y: 10.0f32,
                                              z: 200.0f32,};
                                    init
                                },};
         init
     },
     {
         let mut init =
             OnePointCsFull{actionFlags: 0xf as libc::c_int as u8_0,
                            unk_01: 0xff as libc::c_int as u8_0,
                            initFlags: 0 as libc::c_int as s16,
                            timerInit: 25 as libc::c_int as s16,
                            rollTargetInit: 0 as libc::c_int as s16,
                            fovTargetInit: 60.0f32,
                            lerpStepScale: 1.0f32,
                            atTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: -1.0f32,
                                              y: -1.0f32,
                                              z: -1.0f32,};
                                    init
                                },
                            eyeTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: -1.0f32,
                                              y: -1.0f32,
                                              z: -1.0f32,};
                                    init
                                },};
         init
     },
     {
         let mut init =
             OnePointCsFull{actionFlags: 0x11 as libc::c_int as u8_0,
                            unk_01: 0xff as libc::c_int as u8_0,
                            initFlags: 0 as libc::c_int as s16,
                            timerInit: 1 as libc::c_int as s16,
                            rollTargetInit: -(1 as libc::c_int) as s16,
                            fovTargetInit: -1.0f32,
                            lerpStepScale: -1.0f32,
                            atTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: -1.0f32,
                                              y: -1.0f32,
                                              z: -1.0f32,};
                                    init
                                },
                            eyeTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: -1.0f32,
                                              y: -1.0f32,
                                              z: -1.0f32,};
                                    init
                                },};
         init
     }];
static mut D_8012219C: [OnePointCsFull; 7] =
    [{
         let mut init =
             OnePointCsFull{actionFlags: 0xf as libc::c_int as u8_0,
                            unk_01: 0xff as libc::c_int as u8_0,
                            initFlags: 0x2121 as libc::c_int as s16,
                            timerInit: 5 as libc::c_int as s16,
                            rollTargetInit: 0 as libc::c_int as s16,
                            fovTargetInit: 60.0f32,
                            lerpStepScale: 1.0f32,
                            atTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: 0.0f32,
                                              y: -5.0f32,
                                              z: 0.0f32,};
                                    init
                                },
                            eyeTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: 0.0f32,
                                              y: 0.0f32,
                                              z: -80.0f32,};
                                    init
                                },};
         init
     },
     {
         let mut init =
             OnePointCsFull{actionFlags: 0x2 as libc::c_int as u8_0,
                            unk_01: 0xff as libc::c_int as u8_0,
                            initFlags: 0x4242 as libc::c_int as s16,
                            timerInit: 15 as libc::c_int as s16,
                            rollTargetInit: 0 as libc::c_int as s16,
                            fovTargetInit: 40.0f32,
                            lerpStepScale: 0.4f32,
                            atTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: 0.0f32,
                                              y: 60.0f32,
                                              z: -20.0f32,};
                                    init
                                },
                            eyeTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: 0.0f32,
                                              y: 60.0f32,
                                              z: 100.0f32,};
                                    init
                                },};
         init
     },
     {
         let mut init =
             OnePointCsFull{actionFlags: 0xf as libc::c_int as u8_0,
                            unk_01: 0xff as libc::c_int as u8_0,
                            initFlags: 0 as libc::c_int as s16,
                            timerInit: 20 as libc::c_int as s16,
                            rollTargetInit: 0 as libc::c_int as s16,
                            fovTargetInit: 40.0f32,
                            lerpStepScale: 1.0f32,
                            atTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: 0.0f32,
                                              y: 60.0f32,
                                              z: -20.0f32,};
                                    init
                                },
                            eyeTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: 0.0f32,
                                              y: 60.0f32,
                                              z: 100.0f32,};
                                    init
                                },};
         init
     },
     {
         let mut init =
             OnePointCsFull{actionFlags: 0x1 as libc::c_int as u8_0,
                            unk_01: 0xff as libc::c_int as u8_0,
                            initFlags: 0x4242 as libc::c_int as s16,
                            timerInit: 20 as libc::c_int as s16,
                            rollTargetInit: 0 as libc::c_int as s16,
                            fovTargetInit: 60.0f32,
                            lerpStepScale: 1.0f32,
                            atTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: 20.0f32,
                                              y: 60.0f32,
                                              z: 20.0f32,};
                                    init
                                },
                            eyeTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: 40.0f32,
                                              y: 60.0f32,
                                              z: -80.0f32,};
                                    init
                                },};
         init
     },
     {
         let mut init =
             OnePointCsFull{actionFlags: 0x10 as libc::c_int as u8_0,
                            unk_01: 0xff as libc::c_int as u8_0,
                            initFlags: 0 as libc::c_int as s16,
                            timerInit: 1 as libc::c_int as s16,
                            rollTargetInit: 0 as libc::c_int as s16,
                            fovTargetInit: 60.0f32,
                            lerpStepScale: 1.0f32,
                            atTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: -1.0f32,
                                              y: -1.0f32,
                                              z: -1.0f32,};
                                    init
                                },
                            eyeTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: -1.0f32,
                                              y: -1.0f32,
                                              z: -1.0f32,};
                                    init
                                },};
         init
     },
     {
         let mut init =
             OnePointCsFull{actionFlags: 0xf as libc::c_int as u8_0,
                            unk_01: 0xff as libc::c_int as u8_0,
                            initFlags: 0 as libc::c_int as s16,
                            timerInit: 90 as libc::c_int as s16,
                            rollTargetInit: 0 as libc::c_int as s16,
                            fovTargetInit: 60.0f32,
                            lerpStepScale: 1.0f32,
                            atTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: -1.0f32,
                                              y: -1.0f32,
                                              z: -1.0f32,};
                                    init
                                },
                            eyeTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: -1.0f32,
                                              y: -1.0f32,
                                              z: -1.0f32,};
                                    init
                                },};
         init
     },
     {
         let mut init =
             OnePointCsFull{actionFlags: 0x11 as libc::c_int as u8_0,
                            unk_01: 0xff as libc::c_int as u8_0,
                            initFlags: 0 as libc::c_int as s16,
                            timerInit: 1 as libc::c_int as s16,
                            rollTargetInit: -(1 as libc::c_int) as s16,
                            fovTargetInit: -1.0f32,
                            lerpStepScale: -1.0f32,
                            atTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: -1.0f32,
                                              y: -1.0f32,
                                              z: -1.0f32,};
                                    init
                                },
                            eyeTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: -1.0f32,
                                              y: -1.0f32,
                                              z: -1.0f32,};
                                    init
                                },};
         init
     }];
static mut D_801222B4: [OnePointCsFull; 5] =
    [{
         let mut init =
             OnePointCsFull{actionFlags: 0xf as libc::c_int as u8_0,
                            unk_01: 0xff as libc::c_int as u8_0,
                            initFlags: 0x101 as libc::c_int as s16,
                            timerInit: 20 as libc::c_int as s16,
                            rollTargetInit: 10 as libc::c_int as s16,
                            fovTargetInit: 45.0f32,
                            lerpStepScale: 1.0f32,
                            atTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: -1200.0f32,
                                              y: 730.0f32,
                                              z: -860.0f32,};
                                    init
                                },
                            eyeTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: -1100.0f32,
                                              y: 500.0f32,
                                              z: -1025.0f32,};
                                    init
                                },};
         init
     },
     {
         let mut init =
             OnePointCsFull{actionFlags: 0xb as libc::c_int as u8_0,
                            unk_01: 0xff as libc::c_int as u8_0,
                            initFlags: 0x101 as libc::c_int as s16,
                            timerInit: 20 as libc::c_int as s16,
                            rollTargetInit: 10 as libc::c_int as s16,
                            fovTargetInit: 45.0f32,
                            lerpStepScale: 0.1f32,
                            atTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: -880.0f32,
                                              y: 480.0f32,
                                              z: -860.0f32,};
                                    init
                                },
                            eyeTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: -1100.0f32,
                                              y: 500.0f32,
                                              z: -1025.0f32,};
                                    init
                                },};
         init
     },
     {
         let mut init =
             OnePointCsFull{actionFlags: 0xb as libc::c_int as u8_0,
                            unk_01: 0x81 as libc::c_int as u8_0,
                            initFlags: 0x101 as libc::c_int as s16,
                            timerInit: 20 as libc::c_int as s16,
                            rollTargetInit: 10 as libc::c_int as s16,
                            fovTargetInit: 45.0f32,
                            lerpStepScale: 0.1f32,
                            atTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: -880.0f32,
                                              y: 500.0f32,
                                              z: -860.0f32,};
                                    init
                                },
                            eyeTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: -1100.0f32,
                                              y: 500.0f32,
                                              z: -1025.0f32,};
                                    init
                                },};
         init
     },
     {
         let mut init =
             OnePointCsFull{actionFlags: 0xb as libc::c_int as u8_0,
                            unk_01: 0x8a as libc::c_int as u8_0,
                            initFlags: 0x101 as libc::c_int as s16,
                            timerInit: 5 as libc::c_int as s16,
                            rollTargetInit: 10 as libc::c_int as s16,
                            fovTargetInit: 45.0f32,
                            lerpStepScale: 0.1f32,
                            atTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: -880.0f32,
                                              y: 500.0f32,
                                              z: -860.0f32,};
                                    init
                                },
                            eyeTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: -1100.0f32,
                                              y: 500.0f32,
                                              z: -1025.0f32,};
                                    init
                                },};
         init
     },
     {
         let mut init =
             OnePointCsFull{actionFlags: 0x12 as libc::c_int as u8_0,
                            unk_01: 0xff as libc::c_int as u8_0,
                            initFlags: 0 as libc::c_int as s16,
                            timerInit: 1 as libc::c_int as s16,
                            rollTargetInit: -(1 as libc::c_int) as s16,
                            fovTargetInit: -1.0f32,
                            lerpStepScale: -1.0f32,
                            atTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: -1.0f32,
                                              y: -1.0f32,
                                              z: -1.0f32,};
                                    init
                                },
                            eyeTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: -1.0f32,
                                              y: -1.0f32,
                                              z: -1.0f32,};
                                    init
                                },};
         init
     }];
static mut D_8012237C: [OnePointCsFull; 2] =
    [{
         let mut init =
             OnePointCsFull{actionFlags: 0xf as libc::c_int as u8_0,
                            unk_01: 0xff as libc::c_int as u8_0,
                            initFlags: 0x101 as libc::c_int as s16,
                            timerInit: 20 as libc::c_int as s16,
                            rollTargetInit: -(2 as libc::c_int) as s16,
                            fovTargetInit: 65.0f32,
                            lerpStepScale: 1.0f32,
                            atTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: -625.0f32,
                                              y: 185.0f32,
                                              z: -685.0f32,};
                                    init
                                },
                            eyeTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: -692.0f32,
                                              y: 226.0f32,
                                              z: -515.0f32,};
                                    init
                                },};
         init
     },
     {
         let mut init =
             OnePointCsFull{actionFlags: 0x11 as libc::c_int as u8_0,
                            unk_01: 0xff as libc::c_int as u8_0,
                            initFlags: 0 as libc::c_int as s16,
                            timerInit: 1 as libc::c_int as s16,
                            rollTargetInit: -(1 as libc::c_int) as s16,
                            fovTargetInit: -1.0f32,
                            lerpStepScale: -1.0f32,
                            atTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: -1.0f32,
                                              y: -1.0f32,
                                              z: -1.0f32,};
                                    init
                                },
                            eyeTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: -1.0f32,
                                              y: -1.0f32,
                                              z: -1.0f32,};
                                    init
                                },};
         init
     }];
static mut D_801223CC: [OnePointCsFull; 6] =
    [{
         let mut init =
             OnePointCsFull{actionFlags: 0xf as libc::c_int as u8_0,
                            unk_01: 0xff as libc::c_int as u8_0,
                            initFlags: 0x101 as libc::c_int as s16,
                            timerInit: 20 as libc::c_int as s16,
                            rollTargetInit: 0 as libc::c_int as s16,
                            fovTargetInit: 55.0f32,
                            lerpStepScale: 1.0f32,
                            atTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: 60.0f32,
                                              y: 1130.0f32,
                                              z: -1430.0f32,};
                                    init
                                },
                            eyeTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: 60.0f32,
                                              y: 1130.0f32,
                                              z: -1190.0f32,};
                                    init
                                },};
         init
     },
     {
         let mut init =
             OnePointCsFull{actionFlags: 0xf as libc::c_int as u8_0,
                            unk_01: 0xff as libc::c_int as u8_0,
                            initFlags: 0x101 as libc::c_int as s16,
                            timerInit: 18 as libc::c_int as s16,
                            rollTargetInit: -(13 as libc::c_int) as s16,
                            fovTargetInit: 68.0f32,
                            lerpStepScale: 1.0f32,
                            atTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: 60.0f32,
                                              y: 1130.0f32,
                                              z: -1445.0f32,};
                                    init
                                },
                            eyeTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: 180.0f32,
                                              y: 1170.0f32,
                                              z: -1240.0f32,};
                                    init
                                },};
         init
     },
     {
         let mut init =
             OnePointCsFull{actionFlags: 0xf as libc::c_int as u8_0,
                            unk_01: 0xff as libc::c_int as u8_0,
                            initFlags: 0x101 as libc::c_int as s16,
                            timerInit: 16 as libc::c_int as s16,
                            rollTargetInit: 18 as libc::c_int as s16,
                            fovTargetInit: 75.0f32,
                            lerpStepScale: 1.0f32,
                            atTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: 42.0f32,
                                              y: 1040.0f32,
                                              z: -1400.0f32,};
                                    init
                                },
                            eyeTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: -20.0f32,
                                              y: 940.0f32,
                                              z: -1280.0f32,};
                                    init
                                },};
         init
     },
     {
         let mut init =
             OnePointCsFull{actionFlags: 0xf as libc::c_int as u8_0,
                            unk_01: 0xff as libc::c_int as u8_0,
                            initFlags: 0x101 as libc::c_int as s16,
                            timerInit: 4 as libc::c_int as s16,
                            rollTargetInit: 0 as libc::c_int as s16,
                            fovTargetInit: 60.0f32,
                            lerpStepScale: 1.0f32,
                            atTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: 60.0f32,
                                              y: 1100.0f32,
                                              z: -1465.0f32,};
                                    init
                                },
                            eyeTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: 60.0f32,
                                              y: 1100.0f32,
                                              z: -1180.0f32,};
                                    init
                                },};
         init
     },
     {
         let mut init =
             OnePointCsFull{actionFlags: 0x2 as libc::c_int as u8_0,
                            unk_01: 0xff as libc::c_int as u8_0,
                            initFlags: 0x101 as libc::c_int as s16,
                            timerInit: 32 as libc::c_int as s16,
                            rollTargetInit: 0 as libc::c_int as s16,
                            fovTargetInit: 70.0f32,
                            lerpStepScale: 1.0f32,
                            atTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: 60.0f32,
                                              y: 1100.0f32,
                                              z: -1030.0f32,};
                                    init
                                },
                            eyeTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: 60.0f32,
                                              y: 1150.0f32,
                                              z: -740.0f32,};
                                    init
                                },};
         init
     },
     {
         let mut init =
             OnePointCsFull{actionFlags: 0x12 as libc::c_int as u8_0,
                            unk_01: 0xff as libc::c_int as u8_0,
                            initFlags: 0 as libc::c_int as s16,
                            timerInit: 1 as libc::c_int as s16,
                            rollTargetInit: -(1 as libc::c_int) as s16,
                            fovTargetInit: -1.0f32,
                            lerpStepScale: -1.0f32,
                            atTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: -1.0f32,
                                              y: -1.0f32,
                                              z: -1.0f32,};
                                    init
                                },
                            eyeTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: -1.0f32,
                                              y: -1.0f32,
                                              z: -1.0f32,};
                                    init
                                },};
         init
     }];
static mut D_801224BC: [OnePointCsFull; 7] =
    [{
         let mut init =
             OnePointCsFull{actionFlags: 0xf as libc::c_int as u8_0,
                            unk_01: 0xff as libc::c_int as u8_0,
                            initFlags: 0x101 as libc::c_int as s16,
                            timerInit: 5 as libc::c_int as s16,
                            rollTargetInit: 0 as libc::c_int as s16,
                            fovTargetInit: 70.0f32,
                            lerpStepScale: 1.0f32,
                            atTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: 60.0f32,
                                              y: 1800.0f32,
                                              z: -920.0f32,};
                                    init
                                },
                            eyeTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: 60.0f32,
                                              y: 1860.0f32,
                                              z: -800.0f32,};
                                    init
                                },};
         init
     },
     {
         let mut init =
             OnePointCsFull{actionFlags: 0x3 as libc::c_int as u8_0,
                            unk_01: 0xff as libc::c_int as u8_0,
                            initFlags: 0x101 as libc::c_int as s16,
                            timerInit: 20 as libc::c_int as s16,
                            rollTargetInit: 0 as libc::c_int as s16,
                            fovTargetInit: 70.0f32,
                            lerpStepScale: 0.1f32,
                            atTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: 60.0f32,
                                              y: 1720.0f32,
                                              z: -920.0f32,};
                                    init
                                },
                            eyeTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: 60.0f32,
                                              y: 1780.0f32,
                                              z: -800.0f32,};
                                    init
                                },};
         init
     },
     {
         let mut init =
             OnePointCsFull{actionFlags: 0x10 as libc::c_int as u8_0,
                            unk_01: 0xff as libc::c_int as u8_0,
                            initFlags: 0 as libc::c_int as s16,
                            timerInit: 1 as libc::c_int as s16,
                            rollTargetInit: 0 as libc::c_int as s16,
                            fovTargetInit: 60.0f32,
                            lerpStepScale: 1.0f32,
                            atTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: -1.0f32,
                                              y: -1.0f32,
                                              z: -1.0f32,};
                                    init
                                },
                            eyeTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: -1.0f32,
                                              y: -1.0f32,
                                              z: -1.0f32,};
                                    init
                                },};
         init
     },
     {
         let mut init =
             OnePointCsFull{actionFlags: 0xf as libc::c_int as u8_0,
                            unk_01: 0xff as libc::c_int as u8_0,
                            initFlags: 0x142 as libc::c_int as s16,
                            timerInit: 1 as libc::c_int as s16,
                            rollTargetInit: 0 as libc::c_int as s16,
                            fovTargetInit: 75.0f32,
                            lerpStepScale: 1.0f32,
                            atTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: 0.0f32,
                                              y: 70.0f32,
                                              z: 0.0f32,};
                                    init
                                },
                            eyeTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: 60.0f32,
                                              y: 990.0f32,
                                              z: -690.0f32,};
                                    init
                                },};
         init
     },
     {
         let mut init =
             OnePointCsFull{actionFlags: 0x3 as libc::c_int as u8_0,
                            unk_01: 0xff as libc::c_int as u8_0,
                            initFlags: 0x142 as libc::c_int as s16,
                            timerInit: 119 as libc::c_int as s16,
                            rollTargetInit: 0 as libc::c_int as s16,
                            fovTargetInit: 75.0f32,
                            lerpStepScale: 0.05f32,
                            atTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: 0.0f32,
                                              y: 70.0f32,
                                              z: 0.0f32,};
                                    init
                                },
                            eyeTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: 60.0f32,
                                              y: 990.0f32,
                                              z: -690.0f32,};
                                    init
                                },};
         init
     },
     {
         let mut init =
             OnePointCsFull{actionFlags: 0x3 as libc::c_int as u8_0,
                            unk_01: 0xff as libc::c_int as u8_0,
                            initFlags: 0x4242 as libc::c_int as s16,
                            timerInit: 20 as libc::c_int as s16,
                            rollTargetInit: 0 as libc::c_int as s16,
                            fovTargetInit: 60.0f32,
                            lerpStepScale: 0.1f32,
                            atTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: 0.0f32,
                                              y: 70.0f32,
                                              z: 0.0f32,};
                                    init
                                },
                            eyeTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: 0.0f32,
                                              y: 100.0f32,
                                              z: 200.0f32,};
                                    init
                                },};
         init
     },
     {
         let mut init =
             OnePointCsFull{actionFlags: 0x12 as libc::c_int as u8_0,
                            unk_01: 0xff as libc::c_int as u8_0,
                            initFlags: 0 as libc::c_int as s16,
                            timerInit: 1 as libc::c_int as s16,
                            rollTargetInit: -(1 as libc::c_int) as s16,
                            fovTargetInit: -1.0f32,
                            lerpStepScale: -1.0f32,
                            atTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: -1.0f32,
                                              y: -1.0f32,
                                              z: -1.0f32,};
                                    init
                                },
                            eyeTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: -1.0f32,
                                              y: -1.0f32,
                                              z: -1.0f32,};
                                    init
                                },};
         init
     }];
static mut D_801225D4: [OnePointCsFull; 5] =
    [{
         let mut init =
             OnePointCsFull{actionFlags: 0xf as libc::c_int as u8_0,
                            unk_01: 0x8 as libc::c_int as u8_0,
                            initFlags: 0x101 as libc::c_int as s16,
                            timerInit: 1 as libc::c_int as s16,
                            rollTargetInit: 0 as libc::c_int as s16,
                            fovTargetInit: 50.0f32,
                            lerpStepScale: 1.0f32,
                            atTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: 4100.0f32,
                                              y: 1200.0f32,
                                              z: -1400.0f32,};
                                    init
                                },
                            eyeTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: 3900.0f32,
                                              y: 1100.0f32,
                                              z: -1400.0f32,};
                                    init
                                },};
         init
     },
     {
         let mut init =
             OnePointCsFull{actionFlags: 0x1 as libc::c_int as u8_0,
                            unk_01: 0x3b as libc::c_int as u8_0,
                            initFlags: 0x101 as libc::c_int as s16,
                            timerInit: 60 as libc::c_int as s16,
                            rollTargetInit: 4 as libc::c_int as s16,
                            fovTargetInit: 50.0f32,
                            lerpStepScale: 0.94f32,
                            atTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: 4100.0f32,
                                              y: 965.0f32,
                                              z: -1385.0f32,};
                                    init
                                },
                            eyeTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: 3790.0f32,
                                              y: 825.0f32,
                                              z: -1325.0f32,};
                                    init
                                },};
         init
     },
     {
         let mut init =
             OnePointCsFull{actionFlags: 0x3 as libc::c_int as u8_0,
                            unk_01: 0xff as libc::c_int as u8_0,
                            initFlags: 0x101 as libc::c_int as s16,
                            timerInit: 90 as libc::c_int as s16,
                            rollTargetInit: -(5 as libc::c_int) as s16,
                            fovTargetInit: 130.0f32,
                            lerpStepScale: 0.02f32,
                            atTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: 4100.0f32,
                                              y: 975.0f32,
                                              z: -1375.0f32,};
                                    init
                                },
                            eyeTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: 3735.0f32,
                                              y: 715.0f32,
                                              z: -1325.0f32,};
                                    init
                                },};
         init
     },
     {
         let mut init =
             OnePointCsFull{actionFlags: 0xf as libc::c_int as u8_0,
                            unk_01: 0x8 as libc::c_int as u8_0,
                            initFlags: 0x2323 as libc::c_int as s16,
                            timerInit: 2 as libc::c_int as s16,
                            rollTargetInit: 0 as libc::c_int as s16,
                            fovTargetInit: 60.0f32,
                            lerpStepScale: 1.0f32,
                            atTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: 0.0f32,
                                              y: 60.0f32,
                                              z: 0.0f32,};
                                    init
                                },
                            eyeTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: -10.0f32,
                                              y: 15.0f32,
                                              z: -200.0f32,};
                                    init
                                },};
         init
     },
     {
         let mut init =
             OnePointCsFull{actionFlags: 0x12 as libc::c_int as u8_0,
                            unk_01: 0xff as libc::c_int as u8_0,
                            initFlags: 0 as libc::c_int as s16,
                            timerInit: 1 as libc::c_int as s16,
                            rollTargetInit: 0 as libc::c_int as s16,
                            fovTargetInit: 60.0f32,
                            lerpStepScale: 1.0f32,
                            atTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: -1.0f32,
                                              y: -1.0f32,
                                              z: -1.0f32,};
                                    init
                                },
                            eyeTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: -1.0f32,
                                              y: -1.0f32,
                                              z: -1.0f32,};
                                    init
                                },};
         init
     }];
static mut D_8012269C: [OnePointCsFull; 3] =
    [{
         let mut init =
             OnePointCsFull{actionFlags: 0xf as libc::c_int as u8_0,
                            unk_01: 0xff as libc::c_int as u8_0,
                            initFlags: 0x101 as libc::c_int as s16,
                            timerInit: 20 as libc::c_int as s16,
                            rollTargetInit: 2 as libc::c_int as s16,
                            fovTargetInit: 45.0f32,
                            lerpStepScale: 1.0f32,
                            atTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: 975.0f32,
                                              y: 225.0f32,
                                              z: -1195.0f32,};
                                    init
                                },
                            eyeTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: 918.0f32,
                                              y: 228.0f32,
                                              z: -1228.0f32,};
                                    init
                                },};
         init
     },
     {
         let mut init =
             OnePointCsFull{actionFlags: 0x10 as libc::c_int as u8_0,
                            unk_01: 0xff as libc::c_int as u8_0,
                            initFlags: 0 as libc::c_int as s16,
                            timerInit: 1 as libc::c_int as s16,
                            rollTargetInit: 0 as libc::c_int as s16,
                            fovTargetInit: 60.0f32,
                            lerpStepScale: 1.0f32,
                            atTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: -1.0f32,
                                              y: -1.0f32,
                                              z: -1.0f32,};
                                    init
                                },
                            eyeTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: -1.0f32,
                                              y: -1.0f32,
                                              z: -1.0f32,};
                                    init
                                },};
         init
     },
     {
         let mut init =
             OnePointCsFull{actionFlags: 0x11 as libc::c_int as u8_0,
                            unk_01: 0xff as libc::c_int as u8_0,
                            initFlags: 0 as libc::c_int as s16,
                            timerInit: 1 as libc::c_int as s16,
                            rollTargetInit: -(1 as libc::c_int) as s16,
                            fovTargetInit: -1.0f32,
                            lerpStepScale: -1.0f32,
                            atTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: -1.0f32,
                                              y: -1.0f32,
                                              z: -1.0f32,};
                                    init
                                },
                            eyeTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: -1.0f32,
                                              y: -1.0f32,
                                              z: -1.0f32,};
                                    init
                                },};
         init
     }];
static mut D_80122714: [OnePointCsFull; 4] =
    [{
         let mut init =
             OnePointCsFull{actionFlags: 0xf as libc::c_int as u8_0,
                            unk_01: 0xff as libc::c_int as u8_0,
                            initFlags: 0x101 as libc::c_int as s16,
                            timerInit: 20 as libc::c_int as s16,
                            rollTargetInit: 0 as libc::c_int as s16,
                            fovTargetInit: 45.0f32,
                            lerpStepScale: 1.0f32,
                            atTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: -915.0f32,
                                              y: -2185.0f32,
                                              z: 6335.0f32,};
                                    init
                                },
                            eyeTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: -915.0f32,
                                              y: -2290.0f32,
                                              z: 6165.0f32,};
                                    init
                                },};
         init
     },
     {
         let mut init =
             OnePointCsFull{actionFlags: 0x2 as libc::c_int as u8_0,
                            unk_01: 0xff as libc::c_int as u8_0,
                            initFlags: 0x101 as libc::c_int as s16,
                            timerInit: -(1 as libc::c_int) as s16,
                            rollTargetInit: 0 as libc::c_int as s16,
                            fovTargetInit: 80.0f32,
                            lerpStepScale: 0.8f32,
                            atTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: -920.0f32,
                                              y: -2270.0f32,
                                              z: 6140.0f32,};
                                    init
                                },
                            eyeTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: -920.0f32,
                                              y: -2280.0f32,
                                              z: 6070.0f32,};
                                    init
                                },};
         init
     },
     {
         let mut init =
             OnePointCsFull{actionFlags: 0x2 as libc::c_int as u8_0,
                            unk_01: 0xff as libc::c_int as u8_0,
                            initFlags: 0x101 as libc::c_int as s16,
                            timerInit: 20 as libc::c_int as s16,
                            rollTargetInit: 0 as libc::c_int as s16,
                            fovTargetInit: 80.0f32,
                            lerpStepScale: 0.9f32,
                            atTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: -920.0f32,
                                              y: -2300.0f32,
                                              z: 6140.0f32,};
                                    init
                                },
                            eyeTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: -920.0f32,
                                              y: -2300.0f32,
                                              z: 6070.0f32,};
                                    init
                                },};
         init
     },
     {
         let mut init =
             OnePointCsFull{actionFlags: 0x11 as libc::c_int as u8_0,
                            unk_01: 0xff as libc::c_int as u8_0,
                            initFlags: 0 as libc::c_int as s16,
                            timerInit: 1 as libc::c_int as s16,
                            rollTargetInit: -(1 as libc::c_int) as s16,
                            fovTargetInit: -1.0f32,
                            lerpStepScale: -1.0f32,
                            atTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: -1.0f32,
                                              y: -1.0f32,
                                              z: -1.0f32,};
                                    init
                                },
                            eyeTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: -1.0f32,
                                              y: -1.0f32,
                                              z: -1.0f32,};
                                    init
                                },};
         init
     }];
static mut D_801227B4: [OnePointCsFull; 6] =
    [{
         let mut init =
             OnePointCsFull{actionFlags: 0xf as libc::c_int as u8_0,
                            unk_01: 0xff as libc::c_int as u8_0,
                            initFlags: 0x101 as libc::c_int as s16,
                            timerInit: 30 as libc::c_int as s16,
                            rollTargetInit: 0 as libc::c_int as s16,
                            fovTargetInit: 60.0f32,
                            lerpStepScale: 1.0f32,
                            atTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: 1400.0f32,
                                              y: 100.0f32,
                                              z: -170.0f32,};
                                    init
                                },
                            eyeTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: 1250.0f32,
                                              y: 100.0f32,
                                              z: -170.0f32,};
                                    init
                                },};
         init
     },
     {
         let mut init =
             OnePointCsFull{actionFlags: 0x3 as libc::c_int as u8_0,
                            unk_01: 0xff as libc::c_int as u8_0,
                            initFlags: 0x4242 as libc::c_int as s16,
                            timerInit: 130 as libc::c_int as s16,
                            rollTargetInit: 0 as libc::c_int as s16,
                            fovTargetInit: 60.0f32,
                            lerpStepScale: 0.2f32,
                            atTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: 0.0f32,
                                              y: -5.0f32,
                                              z: 0.0f32,};
                                    init
                                },
                            eyeTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: -150.0f32,
                                              y: -5.0f32,
                                              z: 0.0f32,};
                                    init
                                },};
         init
     },
     {
         let mut init =
             OnePointCsFull{actionFlags: 0x10 as libc::c_int as u8_0,
                            unk_01: 0xff as libc::c_int as u8_0,
                            initFlags: 0 as libc::c_int as s16,
                            timerInit: 1 as libc::c_int as s16,
                            rollTargetInit: 0 as libc::c_int as s16,
                            fovTargetInit: 60.0f32,
                            lerpStepScale: 1.0f32,
                            atTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: -1.0f32,
                                              y: -1.0f32,
                                              z: -1.0f32,};
                                    init
                                },
                            eyeTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: -1.0f32,
                                              y: -1.0f32,
                                              z: -1.0f32,};
                                    init
                                },};
         init
     },
     {
         let mut init =
             OnePointCsFull{actionFlags: 0x2 as libc::c_int as u8_0,
                            unk_01: 0xff as libc::c_int as u8_0,
                            initFlags: 0x303 as libc::c_int as s16,
                            timerInit: 69 as libc::c_int as s16,
                            rollTargetInit: 0 as libc::c_int as s16,
                            fovTargetInit: 85.0f32,
                            lerpStepScale: 1.0f32,
                            atTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: -40.0f32,
                                              y: 0.0f32,
                                              z: 0.0f32,};
                                    init
                                },
                            eyeTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: -40.0f32,
                                              y: 0.0f32,
                                              z: 0.0f32,};
                                    init
                                },};
         init
     },
     {
         let mut init =
             OnePointCsFull{actionFlags: 0x2 as libc::c_int as u8_0,
                            unk_01: 0xff as libc::c_int as u8_0,
                            initFlags: 0x303 as libc::c_int as s16,
                            timerInit: 20 as libc::c_int as s16,
                            rollTargetInit: 0 as libc::c_int as s16,
                            fovTargetInit: 60.0f32,
                            lerpStepScale: 1.0f32,
                            atTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: 10.0f32,
                                              y: 0.0f32,
                                              z: 0.0f32,};
                                    init
                                },
                            eyeTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: 10.0f32,
                                              y: 0.0f32,
                                              z: 0.0f32,};
                                    init
                                },};
         init
     },
     {
         let mut init =
             OnePointCsFull{actionFlags: 0x11 as libc::c_int as u8_0,
                            unk_01: 0xff as libc::c_int as u8_0,
                            initFlags: 0 as libc::c_int as s16,
                            timerInit: 1 as libc::c_int as s16,
                            rollTargetInit: -(1 as libc::c_int) as s16,
                            fovTargetInit: -1.0f32,
                            lerpStepScale: -1.0f32,
                            atTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: -1.0f32,
                                              y: -1.0f32,
                                              z: -1.0f32,};
                                    init
                                },
                            eyeTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: -1.0f32,
                                              y: -1.0f32,
                                              z: -1.0f32,};
                                    init
                                },};
         init
     }];
static mut D_801228A4: [OnePointCsFull; 5] =
    [{
         let mut init =
             OnePointCsFull{actionFlags: 0xf as libc::c_int as u8_0,
                            unk_01: 0x1 as libc::c_int as u8_0,
                            initFlags: 0x101 as libc::c_int as s16,
                            timerInit: 20 as libc::c_int as s16,
                            rollTargetInit: 5 as libc::c_int as s16,
                            fovTargetInit: 30.0f32,
                            lerpStepScale: 1.0f32,
                            atTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: 800.0f32,
                                              y: -40.0f32,
                                              z: 2170.0f32,};
                                    init
                                },
                            eyeTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: 512.0f32,
                                              y: 142.0f32,
                                              z: 2020.0f32,};
                                    init
                                },};
         init
     },
     {
         let mut init =
             OnePointCsFull{actionFlags: 0x2 as libc::c_int as u8_0,
                            unk_01: 0xff as libc::c_int as u8_0,
                            initFlags: 0x101 as libc::c_int as s16,
                            timerInit: 20 as libc::c_int as s16,
                            rollTargetInit: -(2 as libc::c_int) as s16,
                            fovTargetInit: 70.0f32,
                            lerpStepScale: 0.8f32,
                            atTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: 800.0f32,
                                              y: -40.0f32,
                                              z: 2170.0f32,};
                                    init
                                },
                            eyeTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: 512.0f32,
                                              y: 142.0f32,
                                              z: 2020.0f32,};
                                    init
                                },};
         init
     },
     {
         let mut init =
             OnePointCsFull{actionFlags: 0xf as libc::c_int as u8_0,
                            unk_01: 0x8 as libc::c_int as u8_0,
                            initFlags: 0x101 as libc::c_int as s16,
                            timerInit: 90 as libc::c_int as s16,
                            rollTargetInit: 2 as libc::c_int as s16,
                            fovTargetInit: 62.0f32,
                            lerpStepScale: 1.0f32,
                            atTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: 1140.0f32,
                                              y: 125.0f32,
                                              z: 1920.0f32,};
                                    init
                                },
                            eyeTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: 1255.0f32,
                                              y: 150.0f32,
                                              z: 1785.0f32,};
                                    init
                                },};
         init
     },
     {
         let mut init =
             OnePointCsFull{actionFlags: 0x81 as libc::c_int as u8_0,
                            unk_01: 0xff as libc::c_int as u8_0,
                            initFlags: 0x2121 as libc::c_int as s16,
                            timerInit: 10 as libc::c_int as s16,
                            rollTargetInit: 0 as libc::c_int as s16,
                            fovTargetInit: 60.0f32,
                            lerpStepScale: 1.0f32,
                            atTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: 0.0f32,
                                              y: 10.0f32,
                                              z: 0.0f32,};
                                    init
                                },
                            eyeTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: 30.0f32,
                                              y: 10.0f32,
                                              z: -80.0f32,};
                                    init
                                },};
         init
     },
     {
         let mut init =
             OnePointCsFull{actionFlags: 0x12 as libc::c_int as u8_0,
                            unk_01: 0xff as libc::c_int as u8_0,
                            initFlags: 0 as libc::c_int as s16,
                            timerInit: 1 as libc::c_int as s16,
                            rollTargetInit: 0 as libc::c_int as s16,
                            fovTargetInit: 60.0f32,
                            lerpStepScale: 1.0f32,
                            atTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: -1.0f32,
                                              y: -1.0f32,
                                              z: -1.0f32,};
                                    init
                                },
                            eyeTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: -1.0f32,
                                              y: -1.0f32,
                                              z: -1.0f32,};
                                    init
                                },};
         init
     }];
static mut D_8012296C: [OnePointCsFull; 4] =
    [{
         let mut init =
             OnePointCsFull{actionFlags: 0xf as libc::c_int as u8_0,
                            unk_01: 0xff as libc::c_int as u8_0,
                            initFlags: 0x101 as libc::c_int as s16,
                            timerInit: 20 as libc::c_int as s16,
                            rollTargetInit: -(10 as libc::c_int) as s16,
                            fovTargetInit: 70.0f32,
                            lerpStepScale: 1.0f32,
                            atTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: -930.0f32,
                                              y: 765.0f32,
                                              z: -3075.0f32,};
                                    init
                                },
                            eyeTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: -700.0f32,
                                              y: 700.0f32,
                                              z: -3075.0f32,};
                                    init
                                },};
         init
     },
     {
         let mut init =
             OnePointCsFull{actionFlags: 0x3 as libc::c_int as u8_0,
                            unk_01: 0xff as libc::c_int as u8_0,
                            initFlags: 0x101 as libc::c_int as s16,
                            timerInit: 80 as libc::c_int as s16,
                            rollTargetInit: -(10 as libc::c_int) as s16,
                            fovTargetInit: 70.0f32,
                            lerpStepScale: 0.05f32,
                            atTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: -930.0f32,
                                              y: 205.0f32,
                                              z: -3075.0f32,};
                                    init
                                },
                            eyeTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: -700.0f32,
                                              y: 140.0f32,
                                              z: -3075.0f32,};
                                    init
                                },};
         init
     },
     {
         let mut init =
             OnePointCsFull{actionFlags: 0xf as libc::c_int as u8_0,
                            unk_01: 0xff as libc::c_int as u8_0,
                            initFlags: 0 as libc::c_int as s16,
                            timerInit: 120 as libc::c_int as s16,
                            rollTargetInit: 0 as libc::c_int as s16,
                            fovTargetInit: 70.0f32,
                            lerpStepScale: 1.0f32,
                            atTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: -1.0f32,
                                              y: -1.0f32,
                                              z: -1.0f32,};
                                    init
                                },
                            eyeTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: -1.0f32,
                                              y: -1.0f32,
                                              z: -1.0f32,};
                                    init
                                },};
         init
     },
     {
         let mut init =
             OnePointCsFull{actionFlags: 0x11 as libc::c_int as u8_0,
                            unk_01: 0xff as libc::c_int as u8_0,
                            initFlags: 0 as libc::c_int as s16,
                            timerInit: 1 as libc::c_int as s16,
                            rollTargetInit: 0 as libc::c_int as s16,
                            fovTargetInit: 60.0f32,
                            lerpStepScale: 1.0f32,
                            atTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: -1.0f32,
                                              y: -1.0f32,
                                              z: -1.0f32,};
                                    init
                                },
                            eyeTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: -1.0f32,
                                              y: -1.0f32,
                                              z: -1.0f32,};
                                    init
                                },};
         init
     }];
static mut D_80122A0C: [OnePointCsFull; 2] =
    [{
         let mut init =
             OnePointCsFull{actionFlags: 0xf as libc::c_int as u8_0,
                            unk_01: 0xff as libc::c_int as u8_0,
                            initFlags: 0x101 as libc::c_int as s16,
                            timerInit: 60 as libc::c_int as s16,
                            rollTargetInit: 4 as libc::c_int as s16,
                            fovTargetInit: 50.0f32,
                            lerpStepScale: 1.0f32,
                            atTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: 0.0f32,
                                              y: 400.0f32,
                                              z: -1000.0f32,};
                                    init
                                },
                            eyeTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: -200.0f32,
                                              y: 500.0f32,
                                              z: -850.0f32,};
                                    init
                                },};
         init
     },
     {
         let mut init =
             OnePointCsFull{actionFlags: 0x12 as libc::c_int as u8_0,
                            unk_01: 0xff as libc::c_int as u8_0,
                            initFlags: 0 as libc::c_int as s16,
                            timerInit: 1 as libc::c_int as s16,
                            rollTargetInit: 0 as libc::c_int as s16,
                            fovTargetInit: 50.0f32,
                            lerpStepScale: 1.0f32,
                            atTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: -1.0f32,
                                              y: -1.0f32,
                                              z: -1.0f32,};
                                    init
                                },
                            eyeTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: -1.0f32,
                                              y: -1.0f32,
                                              z: -1.0f32,};
                                    init
                                },};
         init
     }];
static mut D_80122A5C: [OnePointCsFull; 8] =
    [{
         let mut init =
             OnePointCsFull{actionFlags: 0xf as libc::c_int as u8_0,
                            unk_01: 0xff as libc::c_int as u8_0,
                            initFlags: 0x101 as libc::c_int as s16,
                            timerInit: 1 as libc::c_int as s16,
                            rollTargetInit: -(15 as libc::c_int) as s16,
                            fovTargetInit: 70.0f32,
                            lerpStepScale: 1.0f32,
                            atTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: 230.0f32,
                                              y: 3675.0f32,
                                              z: -4230.0f32,};
                                    init
                                },
                            eyeTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: -45.0f32,
                                              y: 3650.0f32,
                                              z: -4415.0f32,};
                                    init
                                },};
         init
     },
     {
         let mut init =
             OnePointCsFull{actionFlags: 0x15 as libc::c_int as u8_0,
                            unk_01: 0xff as libc::c_int as u8_0,
                            initFlags: 0 as libc::c_int as s16,
                            timerInit: 1 as libc::c_int as s16,
                            rollTargetInit: 0 as libc::c_int as s16,
                            fovTargetInit: 60.0f32,
                            lerpStepScale: 1.0f32,
                            atTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: -1.0f32,
                                              y: -1.0f32,
                                              z: -1.0f32,};
                                    init
                                },
                            eyeTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: -1.0f32,
                                              y: -1.0f32,
                                              z: -1.0f32,};
                                    init
                                },};
         init
     },
     {
         let mut init =
             OnePointCsFull{actionFlags: 0xf as libc::c_int as u8_0,
                            unk_01: 0xff as libc::c_int as u8_0,
                            initFlags: 0x101 as libc::c_int as s16,
                            timerInit: 1 as libc::c_int as s16,
                            rollTargetInit: 0 as libc::c_int as s16,
                            fovTargetInit: 60.0f32,
                            lerpStepScale: 1.0f32,
                            atTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: -120.0f32,
                                              y: 2187.0f32,
                                              z: -3286.0f32,};
                                    init
                                },
                            eyeTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: -110.0f32,
                                              y: 2162.0f32,
                                              z: -3262.0f32,};
                                    init
                                },};
         init
     },
     {
         let mut init =
             OnePointCsFull{actionFlags: 0x15 as libc::c_int as u8_0,
                            unk_01: 0xff as libc::c_int as u8_0,
                            initFlags: 0 as libc::c_int as s16,
                            timerInit: 1 as libc::c_int as s16,
                            rollTargetInit: 0 as libc::c_int as s16,
                            fovTargetInit: 60.0f32,
                            lerpStepScale: 1.0f32,
                            atTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: -1.0f32,
                                              y: -1.0f32,
                                              z: -1.0f32,};
                                    init
                                },
                            eyeTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: -1.0f32,
                                              y: -1.0f32,
                                              z: -1.0f32,};
                                    init
                                },};
         init
     },
     {
         let mut init =
             OnePointCsFull{actionFlags: 0xf as libc::c_int as u8_0,
                            unk_01: 0xff as libc::c_int as u8_0,
                            initFlags: 0x101 as libc::c_int as s16,
                            timerInit: 55 as libc::c_int as s16,
                            rollTargetInit: 0 as libc::c_int as s16,
                            fovTargetInit: 60.0f32,
                            lerpStepScale: 1.0f32,
                            atTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: -38.0f32,
                                              y: 1467.0f32,
                                              z: -1102.0f32,};
                                    init
                                },
                            eyeTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: 64.0f32,
                                              y: 1423.0f32,
                                              z: -1188.0f32,};
                                    init
                                },};
         init
     },
     {
         let mut init =
             OnePointCsFull{actionFlags: 0xf as libc::c_int as u8_0,
                            unk_01: 0xff as libc::c_int as u8_0,
                            initFlags: 0x101 as libc::c_int as s16,
                            timerInit: 1 as libc::c_int as s16,
                            rollTargetInit: -(15 as libc::c_int) as s16,
                            fovTargetInit: 70.0f32,
                            lerpStepScale: 1.0f32,
                            atTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: 230.0f32,
                                              y: 3675.0f32,
                                              z: -4230.0f32,};
                                    init
                                },
                            eyeTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: -20.0f32,
                                              y: 3650.0f32,
                                              z: -4400.0f32,};
                                    init
                                },};
         init
     },
     {
         let mut init =
             OnePointCsFull{actionFlags: 0x10 as libc::c_int as u8_0,
                            unk_01: 0xff as libc::c_int as u8_0,
                            initFlags: 0 as libc::c_int as s16,
                            timerInit: 1 as libc::c_int as s16,
                            rollTargetInit: 0 as libc::c_int as s16,
                            fovTargetInit: 60.0f32,
                            lerpStepScale: 1.0f32,
                            atTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: -1.0f32,
                                              y: -1.0f32,
                                              z: -1.0f32,};
                                    init
                                },
                            eyeTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: -1.0f32,
                                              y: -1.0f32,
                                              z: -1.0f32,};
                                    init
                                },};
         init
     },
     {
         let mut init =
             OnePointCsFull{actionFlags: 0x11 as libc::c_int as u8_0,
                            unk_01: 0xff as libc::c_int as u8_0,
                            initFlags: 0 as libc::c_int as s16,
                            timerInit: 1 as libc::c_int as s16,
                            rollTargetInit: 0 as libc::c_int as s16,
                            fovTargetInit: 60.0f32,
                            lerpStepScale: 1.0f32,
                            atTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: -1.0f32,
                                              y: -1.0f32,
                                              z: -1.0f32,};
                                    init
                                },
                            eyeTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: -1.0f32,
                                              y: -1.0f32,
                                              z: -1.0f32,};
                                    init
                                },};
         init
     }];
static mut D_80122B9C: [OnePointCsFull; 3] =
    [{
         let mut init =
             OnePointCsFull{actionFlags: 0xf as libc::c_int as u8_0,
                            unk_01: 0xff as libc::c_int as u8_0,
                            initFlags: 0x101 as libc::c_int as s16,
                            timerInit: 60 as libc::c_int as s16,
                            rollTargetInit: 0 as libc::c_int as s16,
                            fovTargetInit: 65.0f32,
                            lerpStepScale: 1.0f32,
                            atTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: 1095.0f32,
                                              y: 2890.0f32,
                                              z: -2980.0f32,};
                                    init
                                },
                            eyeTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: 1166.0f32,
                                              y: 2695.0f32,
                                              z: -2710.0f32,};
                                    init
                                },};
         init
     },
     {
         let mut init =
             OnePointCsFull{actionFlags: 0xf as libc::c_int as u8_0,
                            unk_01: 0xff as libc::c_int as u8_0,
                            initFlags: 0x101 as libc::c_int as s16,
                            timerInit: 60 as libc::c_int as s16,
                            rollTargetInit: 15 as libc::c_int as s16,
                            fovTargetInit: 65.0f32,
                            lerpStepScale: 1.0f32,
                            atTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: 566.0f32,
                                              y: 4654.0f32,
                                              z: -4550.0f32,};
                                    init
                                },
                            eyeTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: 606.0f32,
                                              y: 5160.0f32,
                                              z: -4740.0f32,};
                                    init
                                },};
         init
     },
     {
         let mut init =
             OnePointCsFull{actionFlags: 0x11 as libc::c_int as u8_0,
                            unk_01: 0xff as libc::c_int as u8_0,
                            initFlags: 0 as libc::c_int as s16,
                            timerInit: 1 as libc::c_int as s16,
                            rollTargetInit: 0 as libc::c_int as s16,
                            fovTargetInit: 60.0f32,
                            lerpStepScale: 1.0f32,
                            atTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: -1.0f32,
                                              y: -1.0f32,
                                              z: -1.0f32,};
                                    init
                                },
                            eyeTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: -1.0f32,
                                              y: -1.0f32,
                                              z: -1.0f32,};
                                    init
                                },};
         init
     }];
static mut D_80122C14: [OnePointCsFull; 1] =
    [{
         let mut init =
             OnePointCsFull{actionFlags: 0xf as libc::c_int as u8_0,
                            unk_01: 0xff as libc::c_int as u8_0,
                            initFlags: 0x101 as libc::c_int as s16,
                            timerInit: 999 as libc::c_int as s16,
                            rollTargetInit: 0 as libc::c_int as s16,
                            fovTargetInit: 85.0f32,
                            lerpStepScale: 1.0f32,
                            atTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: -15.0f32,
                                              y: 185.0f32,
                                              z: 160.0f32,};
                                    init
                                },
                            eyeTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: -15.0f32,
                                              y: 210.0f32,
                                              z: 250.0f32,};
                                    init
                                },};
         init
     }];
static mut D_80122C3C: [OnePointCsFull; 1] =
    [{
         let mut init =
             OnePointCsFull{actionFlags: 0xf as libc::c_int as u8_0,
                            unk_01: 0xff as libc::c_int as u8_0,
                            initFlags: 0x101 as libc::c_int as s16,
                            timerInit: 999 as libc::c_int as s16,
                            rollTargetInit: -(2 as libc::c_int) as s16,
                            fovTargetInit: 70.0f32,
                            lerpStepScale: 1.0f32,
                            atTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: -62.0f32,
                                              y: 60.0f32,
                                              z: -315.0f32,};
                                    init
                                },
                            eyeTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: -115.0f32,
                                              y: 30.0f32,
                                              z: -445.0f32,};
                                    init
                                },};
         init
     }];
static mut D_80122C64: [OnePointCsFull; 1] =
    [{
         let mut init =
             OnePointCsFull{actionFlags: 0xf as libc::c_int as u8_0,
                            unk_01: 0xff as libc::c_int as u8_0,
                            initFlags: 0x101 as libc::c_int as s16,
                            timerInit: 999 as libc::c_int as s16,
                            rollTargetInit: 3 as libc::c_int as s16,
                            fovTargetInit: 70.0f32,
                            lerpStepScale: 1.0f32,
                            atTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: -40.0f32,
                                              y: 80.0f32,
                                              z: 375.0f32,};
                                    init
                                },
                            eyeTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: -85.0f32,
                                              y: 45.0f32,
                                              z: 485.0f32,};
                                    init
                                },};
         init
     }];
static mut D_80122C8C: [OnePointCsFull; 1] =
    [{
         let mut init =
             OnePointCsFull{actionFlags: 0xf as libc::c_int as u8_0,
                            unk_01: 0xff as libc::c_int as u8_0,
                            initFlags: 0x101 as libc::c_int as s16,
                            timerInit: 999 as libc::c_int as s16,
                            rollTargetInit: 5 as libc::c_int as s16,
                            fovTargetInit: 60.0f32,
                            lerpStepScale: 1.0f32,
                            atTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: -70.0f32,
                                              y: 140.0f32,
                                              z: 25.0f32,};
                                    init
                                },
                            eyeTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: 10.0f32,
                                              y: 180.0f32,
                                              z: 195.0f32,};
                                    init
                                },};
         init
     }];
static mut D_80122CB4: [OnePointCsFull; 2] =
    [{
         let mut init =
             OnePointCsFull{actionFlags: 0xf as libc::c_int as u8_0,
                            unk_01: 0xff as libc::c_int as u8_0,
                            initFlags: 0x4242 as libc::c_int as s16,
                            timerInit: 5 as libc::c_int as s16,
                            rollTargetInit: 0 as libc::c_int as s16,
                            fovTargetInit: 60.0f32,
                            lerpStepScale: 1.0f32,
                            atTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: 0.0f32,
                                              y: 0.0f32,
                                              z: 1000.0f32,};
                                    init
                                },
                            eyeTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: 0.0f32,
                                              y: 0.0f32,
                                              z: 1100.0f32,};
                                    init
                                },};
         init
     },
     {
         let mut init =
             OnePointCsFull{actionFlags: 0x2 as libc::c_int as u8_0,
                            unk_01: 0xff as libc::c_int as u8_0,
                            initFlags: 0x4242 as libc::c_int as s16,
                            timerInit: -(1 as libc::c_int) as s16,
                            rollTargetInit: 0 as libc::c_int as s16,
                            fovTargetInit: 60.0f32,
                            lerpStepScale: 1.0f32,
                            atTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: 0.0f32,
                                              y: 0.0f32,
                                              z: -100.0f32,};
                                    init
                                },
                            eyeTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: 0.0f32,
                                              y: 0.0f32,
                                              z: 0.0f32,};
                                    init
                                },};
         init
     }];
static mut D_80122D04: [OnePointCsFull; 2] =
    [{
         let mut init =
             OnePointCsFull{actionFlags: 0xf as libc::c_int as u8_0,
                            unk_01: 0xff as libc::c_int as u8_0,
                            initFlags: 0x4242 as libc::c_int as s16,
                            timerInit: 10 as libc::c_int as s16,
                            rollTargetInit: 0 as libc::c_int as s16,
                            fovTargetInit: 60.0f32,
                            lerpStepScale: 1.0f32,
                            atTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: 0.0f32,
                                              y: 0.0f32,
                                              z: -100.0f32,};
                                    init
                                },
                            eyeTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: 0.0f32,
                                              y: 0.0f32,
                                              z: 0.0f32,};
                                    init
                                },};
         init
     },
     {
         let mut init =
             OnePointCsFull{actionFlags: 0x2 as libc::c_int as u8_0,
                            unk_01: 0xff as libc::c_int as u8_0,
                            initFlags: 0x4242 as libc::c_int as s16,
                            timerInit: -(1 as libc::c_int) as s16,
                            rollTargetInit: 0 as libc::c_int as s16,
                            fovTargetInit: 60.0f32,
                            lerpStepScale: 1.0f32,
                            atTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: 0.0f32,
                                              y: 0.0f32,
                                              z: 1000.0f32,};
                                    init
                                },
                            eyeTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: 0.0f32,
                                              y: 0.0f32,
                                              z: 1100.0f32,};
                                    init
                                },};
         init
     }];
static mut D_80122D54: [OnePointCsFull; 3] =
    [{
         let mut init =
             OnePointCsFull{actionFlags: 0xf as libc::c_int as u8_0,
                            unk_01: 0xff as libc::c_int as u8_0,
                            initFlags: 0x101 as libc::c_int as s16,
                            timerInit: 1 as libc::c_int as s16,
                            rollTargetInit: -(4 as libc::c_int) as s16,
                            fovTargetInit: 50.0f32,
                            lerpStepScale: 1.0f32,
                            atTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: 230.0f32,
                                              y: 65.0f32,
                                              z: 300.0f32,};
                                    init
                                },
                            eyeTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: 50.0f32,
                                              y: 50.0f32,
                                              z: 225.0f32,};
                                    init
                                },};
         init
     },
     {
         let mut init =
             OnePointCsFull{actionFlags: 0x10 as libc::c_int as u8_0,
                            unk_01: 0xff as libc::c_int as u8_0,
                            initFlags: 0 as libc::c_int as s16,
                            timerInit: 1 as libc::c_int as s16,
                            rollTargetInit: 0 as libc::c_int as s16,
                            fovTargetInit: 60.0f32,
                            lerpStepScale: 1.0f32,
                            atTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: -1.0f32,
                                              y: -1.0f32,
                                              z: -1.0f32,};
                                    init
                                },
                            eyeTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: -1.0f32,
                                              y: -1.0f32,
                                              z: -1.0f32,};
                                    init
                                },};
         init
     },
     {
         let mut init =
             OnePointCsFull{actionFlags: 0x11 as libc::c_int as u8_0,
                            unk_01: 0xff as libc::c_int as u8_0,
                            initFlags: 0 as libc::c_int as s16,
                            timerInit: 1 as libc::c_int as s16,
                            rollTargetInit: -(1 as libc::c_int) as s16,
                            fovTargetInit: -1.0f32,
                            lerpStepScale: -1.0f32,
                            atTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: -1.0f32,
                                              y: -1.0f32,
                                              z: -1.0f32,};
                                    init
                                },
                            eyeTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: -1.0f32,
                                              y: -1.0f32,
                                              z: -1.0f32,};
                                    init
                                },};
         init
     }];
static mut D_80122DCC: [OnePointCsFull; 3] =
    [{
         let mut init =
             OnePointCsFull{actionFlags: 0xf as libc::c_int as u8_0,
                            unk_01: 0xff as libc::c_int as u8_0,
                            initFlags: 0x101 as libc::c_int as s16,
                            timerInit: 1 as libc::c_int as s16,
                            rollTargetInit: 0 as libc::c_int as s16,
                            fovTargetInit: 50.0f32,
                            lerpStepScale: 1.0f32,
                            atTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: 0.0f32,
                                              y: 5.0f32,
                                              z: -145.0f32,};
                                    init
                                },
                            eyeTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: 0.0f32,
                                              y: 55.0f32,
                                              z: 55.0f32,};
                                    init
                                },};
         init
     },
     {
         let mut init =
             OnePointCsFull{actionFlags: 0x10 as libc::c_int as u8_0,
                            unk_01: 0xff as libc::c_int as u8_0,
                            initFlags: 0 as libc::c_int as s16,
                            timerInit: 1 as libc::c_int as s16,
                            rollTargetInit: 0 as libc::c_int as s16,
                            fovTargetInit: 60.0f32,
                            lerpStepScale: 1.0f32,
                            atTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: -1.0f32,
                                              y: -1.0f32,
                                              z: -1.0f32,};
                                    init
                                },
                            eyeTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: -1.0f32,
                                              y: -1.0f32,
                                              z: -1.0f32,};
                                    init
                                },};
         init
     },
     {
         let mut init =
             OnePointCsFull{actionFlags: 0x11 as libc::c_int as u8_0,
                            unk_01: 0xff as libc::c_int as u8_0,
                            initFlags: 0 as libc::c_int as s16,
                            timerInit: 1 as libc::c_int as s16,
                            rollTargetInit: -(1 as libc::c_int) as s16,
                            fovTargetInit: -1.0f32,
                            lerpStepScale: -1.0f32,
                            atTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: -1.0f32,
                                              y: -1.0f32,
                                              z: -1.0f32,};
                                    init
                                },
                            eyeTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: -1.0f32,
                                              y: -1.0f32,
                                              z: -1.0f32,};
                                    init
                                },};
         init
     }];
static mut D_80122E44: [[OnePointCsFull; 7]; 2] =
    [[{
          let mut init =
              OnePointCsFull{actionFlags: 0x83 as libc::c_int as u8_0,
                             unk_01: 0xff as libc::c_int as u8_0,
                             initFlags: 0x2222 as libc::c_int as s16,
                             timerInit: 10 as libc::c_int as s16,
                             rollTargetInit: 5 as libc::c_int as s16,
                             fovTargetInit: 90.0f32,
                             lerpStepScale: 0.2f32,
                             atTargetInit:
                                 {
                                     let mut init =
                                         Vec3f{x: 50.0f32,
                                               y: 100.0f32,
                                               z: 140.0f32,};
                                     init
                                 },
                             eyeTargetInit:
                                 {
                                     let mut init =
                                         Vec3f{x: -30.0f32,
                                               y: 10.0f32,
                                               z: -20.0f32,};
                                     init
                                 },};
          init
      },
      {
          let mut init =
              OnePointCsFull{actionFlags: 0x8f as libc::c_int as u8_0,
                             unk_01: 0xff as libc::c_int as u8_0,
                             initFlags: 0 as libc::c_int as s16,
                             timerInit: 20 as libc::c_int as s16,
                             rollTargetInit: 0 as libc::c_int as s16,
                             fovTargetInit: 90.0f32,
                             lerpStepScale: 1.0f32,
                             atTargetInit:
                                 {
                                     let mut init =
                                         Vec3f{x: -1.0f32,
                                               y: -1.0f32,
                                               z: -1.0f32,};
                                     init
                                 },
                             eyeTargetInit:
                                 {
                                     let mut init =
                                         Vec3f{x: -1.0f32,
                                               y: -1.0f32,
                                               z: -1.0f32,};
                                     init
                                 },};
          init
      },
      {
          let mut init =
              OnePointCsFull{actionFlags: 0x3 as libc::c_int as u8_0,
                             unk_01: 0xff as libc::c_int as u8_0,
                             initFlags: 0x4343 as libc::c_int as s16,
                             timerInit: 30 as libc::c_int as s16,
                             rollTargetInit: -(5 as libc::c_int) as s16,
                             fovTargetInit: 50.0f32,
                             lerpStepScale: 0.2f32,
                             atTargetInit:
                                 {
                                     let mut init =
                                         Vec3f{x: -10.0f32,
                                               y: 80.0f32,
                                               z: 10.0f32,};
                                     init
                                 },
                             eyeTargetInit:
                                 {
                                     let mut init =
                                         Vec3f{x: 20.0f32,
                                               y: 20.0f32,
                                               z: 120.0f32,};
                                     init
                                 },};
          init
      },
      {
          let mut init =
              OnePointCsFull{actionFlags: 0x10 as libc::c_int as u8_0,
                             unk_01: 0xff as libc::c_int as u8_0,
                             initFlags: 0 as libc::c_int as s16,
                             timerInit: 1 as libc::c_int as s16,
                             rollTargetInit: -(5 as libc::c_int) as s16,
                             fovTargetInit: 60.0f32,
                             lerpStepScale: 1.0f32,
                             atTargetInit:
                                 {
                                     let mut init =
                                         Vec3f{x: -1.0f32,
                                               y: -1.0f32,
                                               z: -1.0f32,};
                                     init
                                 },
                             eyeTargetInit:
                                 {
                                     let mut init =
                                         Vec3f{x: -1.0f32,
                                               y: -1.0f32,
                                               z: -1.0f32,};
                                     init
                                 },};
          init
      },
      {
          let mut init =
              OnePointCsFull{actionFlags: 0xb as libc::c_int as u8_0,
                             unk_01: 0x1 as libc::c_int as u8_0,
                             initFlags: 0x4343 as libc::c_int as s16,
                             timerInit: 160 as libc::c_int as s16,
                             rollTargetInit: 10 as libc::c_int as s16,
                             fovTargetInit: 80.0f32,
                             lerpStepScale: 0.005f32,
                             atTargetInit:
                                 {
                                     let mut init =
                                         Vec3f{x: -50.0f32,
                                               y: 60.0f32,
                                               z: 0.0f32,};
                                     init
                                 },
                             eyeTargetInit:
                                 {
                                     let mut init =
                                         Vec3f{x: -100.0f32,
                                               y: 20.0f32,
                                               z: 50.0f32,};
                                     init
                                 },};
          init
      },
      {
          let mut init =
              OnePointCsFull{actionFlags: 0x2 as libc::c_int as u8_0,
                             unk_01: 0xff as libc::c_int as u8_0,
                             initFlags: 0x501 as libc::c_int as s16,
                             timerInit: 50 as libc::c_int as s16,
                             rollTargetInit: 0 as libc::c_int as s16,
                             fovTargetInit: 60.0f32,
                             lerpStepScale: 1.0f32,
                             atTargetInit:
                                 {
                                     let mut init =
                                         Vec3f{x: 0.0f32,
                                               y: -10.0f32,
                                               z: 0.0f32,};
                                     init
                                 },
                             eyeTargetInit:
                                 {
                                     let mut init =
                                         Vec3f{x: 0.0f32,
                                               y: 10.0f32,
                                               z: 80.0f32,};
                                     init
                                 },};
          init
      },
      {
          let mut init =
              OnePointCsFull{actionFlags: 0x13 as libc::c_int as u8_0,
                             unk_01: 0xff as libc::c_int as u8_0,
                             initFlags: 0 as libc::c_int as s16,
                             timerInit: 1 as libc::c_int as s16,
                             rollTargetInit: -(1 as libc::c_int) as s16,
                             fovTargetInit: -1.0f32,
                             lerpStepScale: -1.0f32,
                             atTargetInit:
                                 {
                                     let mut init =
                                         Vec3f{x: -1.0f32,
                                               y: -1.0f32,
                                               z: -1.0f32,};
                                     init
                                 },
                             eyeTargetInit:
                                 {
                                     let mut init =
                                         Vec3f{x: -1.0f32,
                                               y: -1.0f32,
                                               z: -1.0f32,};
                                     init
                                 },};
          init
      }],
     [{
          let mut init =
              OnePointCsFull{actionFlags: 0x83 as libc::c_int as u8_0,
                             unk_01: 0xff as libc::c_int as u8_0,
                             initFlags: 0x2222 as libc::c_int as s16,
                             timerInit: 10 as libc::c_int as s16,
                             rollTargetInit: -(5 as libc::c_int) as s16,
                             fovTargetInit: 90.0f32,
                             lerpStepScale: 0.2f32,
                             atTargetInit:
                                 {
                                     let mut init =
                                         Vec3f{x: -50.0f32,
                                               y: 100.0f32,
                                               z: 140.0f32,};
                                     init
                                 },
                             eyeTargetInit:
                                 {
                                     let mut init =
                                         Vec3f{x: 30.0f32,
                                               y: 10.0f32,
                                               z: -20.0f32,};
                                     init
                                 },};
          init
      },
      {
          let mut init =
              OnePointCsFull{actionFlags: 0x8f as libc::c_int as u8_0,
                             unk_01: 0xff as libc::c_int as u8_0,
                             initFlags: 0 as libc::c_int as s16,
                             timerInit: 20 as libc::c_int as s16,
                             rollTargetInit: 0 as libc::c_int as s16,
                             fovTargetInit: 90.0f32,
                             lerpStepScale: 1.0f32,
                             atTargetInit:
                                 {
                                     let mut init =
                                         Vec3f{x: -1.0f32,
                                               y: -1.0f32,
                                               z: -1.0f32,};
                                     init
                                 },
                             eyeTargetInit:
                                 {
                                     let mut init =
                                         Vec3f{x: -1.0f32,
                                               y: -1.0f32,
                                               z: -1.0f32,};
                                     init
                                 },};
          init
      },
      {
          let mut init =
              OnePointCsFull{actionFlags: 0x3 as libc::c_int as u8_0,
                             unk_01: 0xff as libc::c_int as u8_0,
                             initFlags: 0x4343 as libc::c_int as s16,
                             timerInit: 30 as libc::c_int as s16,
                             rollTargetInit: 5 as libc::c_int as s16,
                             fovTargetInit: 50.0f32,
                             lerpStepScale: 0.2f32,
                             atTargetInit:
                                 {
                                     let mut init =
                                         Vec3f{x: 10.0f32,
                                               y: 80.0f32,
                                               z: 10.0f32,};
                                     init
                                 },
                             eyeTargetInit:
                                 {
                                     let mut init =
                                         Vec3f{x: -20.0f32,
                                               y: 20.0f32,
                                               z: 120.0f32,};
                                     init
                                 },};
          init
      },
      {
          let mut init =
              OnePointCsFull{actionFlags: 0x10 as libc::c_int as u8_0,
                             unk_01: 0xff as libc::c_int as u8_0,
                             initFlags: 0 as libc::c_int as s16,
                             timerInit: 1 as libc::c_int as s16,
                             rollTargetInit: 5 as libc::c_int as s16,
                             fovTargetInit: 60.0f32,
                             lerpStepScale: 1.0f32,
                             atTargetInit:
                                 {
                                     let mut init =
                                         Vec3f{x: -1.0f32,
                                               y: -1.0f32,
                                               z: -1.0f32,};
                                     init
                                 },
                             eyeTargetInit:
                                 {
                                     let mut init =
                                         Vec3f{x: -1.0f32,
                                               y: -1.0f32,
                                               z: -1.0f32,};
                                     init
                                 },};
          init
      },
      {
          let mut init =
              OnePointCsFull{actionFlags: 0xb as libc::c_int as u8_0,
                             unk_01: 0x1 as libc::c_int as u8_0,
                             initFlags: 0x4343 as libc::c_int as s16,
                             timerInit: 160 as libc::c_int as s16,
                             rollTargetInit: -(10 as libc::c_int) as s16,
                             fovTargetInit: 80.0f32,
                             lerpStepScale: 0.005f32,
                             atTargetInit:
                                 {
                                     let mut init =
                                         Vec3f{x: 50.0f32,
                                               y: 60.0f32,
                                               z: 0.0f32,};
                                     init
                                 },
                             eyeTargetInit:
                                 {
                                     let mut init =
                                         Vec3f{x: 100.0f32,
                                               y: 20.0f32,
                                               z: 50.0f32,};
                                     init
                                 },};
          init
      },
      {
          let mut init =
              OnePointCsFull{actionFlags: 0x2 as libc::c_int as u8_0,
                             unk_01: 0xff as libc::c_int as u8_0,
                             initFlags: 0x501 as libc::c_int as s16,
                             timerInit: 50 as libc::c_int as s16,
                             rollTargetInit: 0 as libc::c_int as s16,
                             fovTargetInit: 60.0f32,
                             lerpStepScale: 1.0f32,
                             atTargetInit:
                                 {
                                     let mut init =
                                         Vec3f{x: 0.0f32,
                                               y: -10.0f32,
                                               z: 0.0f32,};
                                     init
                                 },
                             eyeTargetInit:
                                 {
                                     let mut init =
                                         Vec3f{x: 0.0f32,
                                               y: 10.0f32,
                                               z: 80.0f32,};
                                     init
                                 },};
          init
      },
      {
          let mut init =
              OnePointCsFull{actionFlags: 0x13 as libc::c_int as u8_0,
                             unk_01: 0xff as libc::c_int as u8_0,
                             initFlags: 0 as libc::c_int as s16,
                             timerInit: 1 as libc::c_int as s16,
                             rollTargetInit: -(1 as libc::c_int) as s16,
                             fovTargetInit: -1.0f32,
                             lerpStepScale: -1.0f32,
                             atTargetInit:
                                 {
                                     let mut init =
                                         Vec3f{x: -1.0f32,
                                               y: -1.0f32,
                                               z: -1.0f32,};
                                     init
                                 },
                             eyeTargetInit:
                                 {
                                     let mut init =
                                         Vec3f{x: -1.0f32,
                                               y: -1.0f32,
                                               z: -1.0f32,};
                                     init
                                 },};
          init
      }]];
static mut D_80123074: [OnePointCsFull; 5] =
    [{
         let mut init =
             OnePointCsFull{actionFlags: 0x8f as libc::c_int as u8_0,
                            unk_01: 0xff as libc::c_int as u8_0,
                            initFlags: 0xa2a2 as libc::c_int as s16,
                            timerInit: 2 as libc::c_int as s16,
                            rollTargetInit: 8 as libc::c_int as s16,
                            fovTargetInit: 70.0f32,
                            lerpStepScale: 1.0f32,
                            atTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: -27.0f32,
                                              y: -96.0f32,
                                              z: 25.0f32,};
                                    init
                                },
                            eyeTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: 37.0f32,
                                              y: -5.0f32,
                                              z: 100.0f32,};
                                    init
                                },};
         init
     },
     {
         let mut init =
             OnePointCsFull{actionFlags: 0x81 as libc::c_int as u8_0,
                            unk_01: 0xff as libc::c_int as u8_0,
                            initFlags: 0xa2a2 as libc::c_int as s16,
                            timerInit: 38 as libc::c_int as s16,
                            rollTargetInit: 4 as libc::c_int as s16,
                            fovTargetInit: 60.0f32,
                            lerpStepScale: 1.0f32,
                            atTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: 64.0f32,
                                              y: -109.0f32,
                                              z: 55.0f32,};
                                    init
                                },
                            eyeTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: 37.0f32,
                                              y: 150.0f32,
                                              z: 155.0f32,};
                                    init
                                },};
         init
     },
     {
         let mut init =
             OnePointCsFull{actionFlags: 0x8f as libc::c_int as u8_0,
                            unk_01: 0xff as libc::c_int as u8_0,
                            initFlags: 0xa2a2 as libc::c_int as s16,
                            timerInit: 2 as libc::c_int as s16,
                            rollTargetInit: 8 as libc::c_int as s16,
                            fovTargetInit: 70.0f32,
                            lerpStepScale: 1.0f32,
                            atTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: 45.0f32,
                                              y: 123.0f32,
                                              z: 45.0f32,};
                                    init
                                },
                            eyeTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: 70.0f32,
                                              y: 5.0f32,
                                              z: 125.0f32,};
                                    init
                                },};
         init
     },
     {
         let mut init =
             OnePointCsFull{actionFlags: 0x81 as libc::c_int as u8_0,
                            unk_01: 0xff as libc::c_int as u8_0,
                            initFlags: 0xa2a2 as libc::c_int as s16,
                            timerInit: 58 as libc::c_int as s16,
                            rollTargetInit: 4 as libc::c_int as s16,
                            fovTargetInit: 60.0f32,
                            lerpStepScale: 0.9f32,
                            atTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: 82.0f32,
                                              y: 95.0f32,
                                              z: 55.0f32,};
                                    init
                                },
                            eyeTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: 25.0f32,
                                              y: -175.0f32,
                                              z: 180.0f32,};
                                    init
                                },};
         init
     },
     {
         let mut init =
             OnePointCsFull{actionFlags: 0x92 as libc::c_int as u8_0,
                            unk_01: 0xff as libc::c_int as u8_0,
                            initFlags: 0 as libc::c_int as s16,
                            timerInit: 1 as libc::c_int as s16,
                            rollTargetInit: 0 as libc::c_int as s16,
                            fovTargetInit: 60.0f32,
                            lerpStepScale: 1.0f32,
                            atTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: -1.0f32,
                                              y: -1.0f32,
                                              z: -1.0f32,};
                                    init
                                },
                            eyeTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: -1.0f32,
                                              y: -1.0f32,
                                              z: -1.0f32,};
                                    init
                                },};
         init
     }];
static mut D_8012313C: [OnePointCsFull; 3] =
    [{
         let mut init =
             OnePointCsFull{actionFlags: 0x8f as libc::c_int as u8_0,
                            unk_01: 0xff as libc::c_int as u8_0,
                            initFlags: 0xa2a2 as libc::c_int as s16,
                            timerInit: 20 as libc::c_int as s16,
                            rollTargetInit: 8 as libc::c_int as s16,
                            fovTargetInit: 70.0f32,
                            lerpStepScale: 1.0f32,
                            atTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: 65.0f32,
                                              y: -150.0f32,
                                              z: 50.0f32,};
                                    init
                                },
                            eyeTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: 30.0f32,
                                              y: 10.0f32,
                                              z: 90.0f32,};
                                    init
                                },};
         init
     },
     {
         let mut init =
             OnePointCsFull{actionFlags: 0x81 as libc::c_int as u8_0,
                            unk_01: 0xff as libc::c_int as u8_0,
                            initFlags: 0xa2a2 as libc::c_int as s16,
                            timerInit: 100 as libc::c_int as s16,
                            rollTargetInit: 0 as libc::c_int as s16,
                            fovTargetInit: 60.0f32,
                            lerpStepScale: 1.0f32,
                            atTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: 70.0f32,
                                              y: -160.0f32,
                                              z: 50.0f32,};
                                    init
                                },
                            eyeTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: 25.0f32,
                                              y: 180.0f32,
                                              z: 180.0f32,};
                                    init
                                },};
         init
     },
     {
         let mut init =
             OnePointCsFull{actionFlags: 0x92 as libc::c_int as u8_0,
                            unk_01: 0xff as libc::c_int as u8_0,
                            initFlags: 0 as libc::c_int as s16,
                            timerInit: 1 as libc::c_int as s16,
                            rollTargetInit: 0 as libc::c_int as s16,
                            fovTargetInit: 60.0f32,
                            lerpStepScale: 1.0f32,
                            atTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: -1.0f32,
                                              y: -1.0f32,
                                              z: -1.0f32,};
                                    init
                                },
                            eyeTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: -1.0f32,
                                              y: -1.0f32,
                                              z: -1.0f32,};
                                    init
                                },};
         init
     }];
static mut D_801231B4: [OnePointCsFull; 4] =
    [{
         let mut init =
             OnePointCsFull{actionFlags: 0x8f as libc::c_int as u8_0,
                            unk_01: 0xc5 as libc::c_int as u8_0,
                            initFlags: 0x4343 as libc::c_int as s16,
                            timerInit: 1 as libc::c_int as s16,
                            rollTargetInit: 0 as libc::c_int as s16,
                            fovTargetInit: 50.0f32,
                            lerpStepScale: 1.0f32,
                            atTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: 0.0f32,
                                              y: 20.0f32,
                                              z: 0.0f32,};
                                    init
                                },
                            eyeTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: 0.0f32,
                                              y: 5.0f32,
                                              z: -1.0f32,};
                                    init
                                },};
         init
     },
     {
         let mut init =
             OnePointCsFull{actionFlags: 0x81 as libc::c_int as u8_0,
                            unk_01: 0xc5 as libc::c_int as u8_0,
                            initFlags: 0x4343 as libc::c_int as s16,
                            timerInit: 48 as libc::c_int as s16,
                            rollTargetInit: 0 as libc::c_int as s16,
                            fovTargetInit: 50.0f32,
                            lerpStepScale: 0.75f32,
                            atTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: 0.0f32,
                                              y: 80.0f32,
                                              z: 0.0f32,};
                                    init
                                },
                            eyeTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: 0.0f32,
                                              y: 15.0f32,
                                              z: -1.0f32,};
                                    init
                                },};
         init
     },
     {
         let mut init =
             OnePointCsFull{actionFlags: 0x8f as libc::c_int as u8_0,
                            unk_01: 0xc5 as libc::c_int as u8_0,
                            initFlags: 0x4343 as libc::c_int as s16,
                            timerInit: 1 as libc::c_int as s16,
                            rollTargetInit: 5 as libc::c_int as s16,
                            fovTargetInit: 45.0f32,
                            lerpStepScale: 1.0f32,
                            atTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: 0.0f32,
                                              y: 0.0f32,
                                              z: 30.0f32,};
                                    init
                                },
                            eyeTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: 30.0f32,
                                              y: 120.0f32,
                                              z: 60.0f32,};
                                    init
                                },};
         init
     },
     {
         let mut init =
             OnePointCsFull{actionFlags: 0x81 as libc::c_int as u8_0,
                            unk_01: 0xc5 as libc::c_int as u8_0,
                            initFlags: 0x4343 as libc::c_int as s16,
                            timerInit: -(1 as libc::c_int) as s16,
                            rollTargetInit: 0 as libc::c_int as s16,
                            fovTargetInit: -1.0f32,
                            lerpStepScale: 1.0f32,
                            atTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: -1.0f32,
                                              y: -1.0f32,
                                              z: -1.0f32,};
                                    init
                                },
                            eyeTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: -1.0f32,
                                              y: -1.0f32,
                                              z: -1.0f32,};
                                    init
                                },};
         init
     }];
static mut D_80123254: [OnePointCsFull; 2] =
    [{
         let mut init =
             OnePointCsFull{actionFlags: 0xf as libc::c_int as u8_0,
                            unk_01: 0xff as libc::c_int as u8_0,
                            initFlags: 0x101 as libc::c_int as s16,
                            timerInit: 1 as libc::c_int as s16,
                            rollTargetInit: 0 as libc::c_int as s16,
                            fovTargetInit: 60.0f32,
                            lerpStepScale: 1.0f32,
                            atTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: 0.0f32,
                                              y: 0.0f32,
                                              z: 0.0f32,};
                                    init
                                },
                            eyeTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: 0.0f32,
                                              y: 0.0f32,
                                              z: 0.0f32,};
                                    init
                                },};
         init
     },
     {
         let mut init =
             OnePointCsFull{actionFlags: 0x3 as libc::c_int as u8_0,
                            unk_01: 0xc5 as libc::c_int as u8_0,
                            initFlags: 0x101 as libc::c_int as s16,
                            timerInit: 49 as libc::c_int as s16,
                            rollTargetInit: 0 as libc::c_int as s16,
                            fovTargetInit: 50.0f32,
                            lerpStepScale: 0.05f32,
                            atTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: 0.0f32,
                                              y: 0.0f32,
                                              z: 0.0f32,};
                                    init
                                },
                            eyeTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: 0.0f32,
                                              y: 0.0f32,
                                              z: 0.0f32,};
                                    init
                                },};
         init
     }];
static mut D_801232A4: [OnePointCsFull; 1] =
    [{
         let mut init =
             OnePointCsFull{actionFlags: 0xf as libc::c_int as u8_0,
                            unk_01: 0x45 as libc::c_int as u8_0,
                            initFlags: 0x101 as libc::c_int as s16,
                            timerInit: 9999 as libc::c_int as s16,
                            rollTargetInit: 0 as libc::c_int as s16,
                            fovTargetInit: 60.0f32,
                            lerpStepScale: 1.0f32,
                            atTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: 0.0f32,
                                              y: 0.0f32,
                                              z: 0.0f32,};
                                    init
                                },
                            eyeTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: 0.0f32,
                                              y: 0.0f32,
                                              z: 0.0f32,};
                                    init
                                },};
         init
     }];
static mut D_801232CC: [OnePointCsFull; 5] =
    [{
         let mut init =
             OnePointCsFull{actionFlags: 0x1 as libc::c_int as u8_0,
                            unk_01: 0xff as libc::c_int as u8_0,
                            initFlags: 0x101 as libc::c_int as s16,
                            timerInit: 45 as libc::c_int as s16,
                            rollTargetInit: -(3 as libc::c_int) as s16,
                            fovTargetInit: 65.0f32,
                            lerpStepScale: 1.0f32,
                            atTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: -52.0f32,
                                              y: 84.0f32,
                                              z: -846.0f32,};
                                    init
                                },
                            eyeTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: -159.0f32,
                                              y: 33.0f32,
                                              z: -729.0f32,};
                                    init
                                },};
         init
     },
     {
         let mut init =
             OnePointCsFull{actionFlags: 0x10 as libc::c_int as u8_0,
                            unk_01: 0xff as libc::c_int as u8_0,
                            initFlags: 0 as libc::c_int as s16,
                            timerInit: 1 as libc::c_int as s16,
                            rollTargetInit: 0 as libc::c_int as s16,
                            fovTargetInit: 60.0f32,
                            lerpStepScale: 1.0f32,
                            atTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: -1.0f32,
                                              y: -1.0f32,
                                              z: -1.0f32,};
                                    init
                                },
                            eyeTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: -1.0f32,
                                              y: -1.0f32,
                                              z: -1.0f32,};
                                    init
                                },};
         init
     },
     {
         let mut init =
             OnePointCsFull{actionFlags: 0xf as libc::c_int as u8_0,
                            unk_01: 0xff as libc::c_int as u8_0,
                            initFlags: 0 as libc::c_int as s16,
                            timerInit: 10 as libc::c_int as s16,
                            rollTargetInit: 0 as libc::c_int as s16,
                            fovTargetInit: 60.0f32,
                            lerpStepScale: 1.0f32,
                            atTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: -1.0f32,
                                              y: -1.0f32,
                                              z: -1.0f32,};
                                    init
                                },
                            eyeTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: -1.0f32,
                                              y: -1.0f32,
                                              z: -1.0f32,};
                                    init
                                },};
         init
     },
     {
         let mut init =
             OnePointCsFull{actionFlags: 0x1 as libc::c_int as u8_0,
                            unk_01: 0xff as libc::c_int as u8_0,
                            initFlags: 0x2121 as libc::c_int as s16,
                            timerInit: 15 as libc::c_int as s16,
                            rollTargetInit: 0 as libc::c_int as s16,
                            fovTargetInit: 60.0f32,
                            lerpStepScale: 1.0f32,
                            atTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: 10.0f32,
                                              y: -5.0f32,
                                              z: 0.0f32,};
                                    init
                                },
                            eyeTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: 0.0f32,
                                              y: 0.0f32,
                                              z: -150.0f32,};
                                    init
                                },};
         init
     },
     {
         let mut init =
             OnePointCsFull{actionFlags: 0x12 as libc::c_int as u8_0,
                            unk_01: 0xff as libc::c_int as u8_0,
                            initFlags: 0 as libc::c_int as s16,
                            timerInit: 1 as libc::c_int as s16,
                            rollTargetInit: 0 as libc::c_int as s16,
                            fovTargetInit: 60.0f32,
                            lerpStepScale: 1.0f32,
                            atTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: -1.0f32,
                                              y: -1.0f32,
                                              z: -1.0f32,};
                                    init
                                },
                            eyeTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: -1.0f32,
                                              y: -1.0f32,
                                              z: -1.0f32,};
                                    init
                                },};
         init
     }];
static mut D_80123394: [OnePointCsFull; 5] =
    [{
         let mut init =
             OnePointCsFull{actionFlags: 0x1 as libc::c_int as u8_0,
                            unk_01: 0xff as libc::c_int as u8_0,
                            initFlags: 0x101 as libc::c_int as s16,
                            timerInit: 45 as libc::c_int as s16,
                            rollTargetInit: 3 as libc::c_int as s16,
                            fovTargetInit: 65.0f32,
                            lerpStepScale: 1.0f32,
                            atTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: -16.0f32,
                                              y: 87.0f32,
                                              z: -829.0f32,};
                                    init
                                },
                            eyeTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: 98.0f32,
                                              y: 24.0f32,
                                              z: -714.0f32,};
                                    init
                                },};
         init
     },
     {
         let mut init =
             OnePointCsFull{actionFlags: 0x10 as libc::c_int as u8_0,
                            unk_01: 0xff as libc::c_int as u8_0,
                            initFlags: 0 as libc::c_int as s16,
                            timerInit: 1 as libc::c_int as s16,
                            rollTargetInit: 0 as libc::c_int as s16,
                            fovTargetInit: 60.0f32,
                            lerpStepScale: 1.0f32,
                            atTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: -1.0f32,
                                              y: -1.0f32,
                                              z: -1.0f32,};
                                    init
                                },
                            eyeTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: -1.0f32,
                                              y: -1.0f32,
                                              z: -1.0f32,};
                                    init
                                },};
         init
     },
     {
         let mut init =
             OnePointCsFull{actionFlags: 0xf as libc::c_int as u8_0,
                            unk_01: 0xff as libc::c_int as u8_0,
                            initFlags: 0 as libc::c_int as s16,
                            timerInit: 10 as libc::c_int as s16,
                            rollTargetInit: 0 as libc::c_int as s16,
                            fovTargetInit: 60.0f32,
                            lerpStepScale: 1.0f32,
                            atTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: -1.0f32,
                                              y: -1.0f32,
                                              z: -1.0f32,};
                                    init
                                },
                            eyeTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: -1.0f32,
                                              y: -1.0f32,
                                              z: -1.0f32,};
                                    init
                                },};
         init
     },
     {
         let mut init =
             OnePointCsFull{actionFlags: 0x1 as libc::c_int as u8_0,
                            unk_01: 0xff as libc::c_int as u8_0,
                            initFlags: 0x2121 as libc::c_int as s16,
                            timerInit: 15 as libc::c_int as s16,
                            rollTargetInit: 0 as libc::c_int as s16,
                            fovTargetInit: 60.0f32,
                            lerpStepScale: 1.0f32,
                            atTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: 10.0f32,
                                              y: -5.0f32,
                                              z: 0.0f32,};
                                    init
                                },
                            eyeTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: 0.0f32,
                                              y: 0.0f32,
                                              z: -150.0f32,};
                                    init
                                },};
         init
     },
     {
         let mut init =
             OnePointCsFull{actionFlags: 0x12 as libc::c_int as u8_0,
                            unk_01: 0xff as libc::c_int as u8_0,
                            initFlags: 0 as libc::c_int as s16,
                            timerInit: 1 as libc::c_int as s16,
                            rollTargetInit: 0 as libc::c_int as s16,
                            fovTargetInit: 60.0f32,
                            lerpStepScale: 1.0f32,
                            atTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: -1.0f32,
                                              y: -1.0f32,
                                              z: -1.0f32,};
                                    init
                                },
                            eyeTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: -1.0f32,
                                              y: -1.0f32,
                                              z: -1.0f32,};
                                    init
                                },};
         init
     }];
static mut D_8012345C: [OnePointCsFull; 4] =
    [{
         let mut init =
             OnePointCsFull{actionFlags: 0x1 as libc::c_int as u8_0,
                            unk_01: 0x1 as libc::c_int as u8_0,
                            initFlags: 0x4242 as libc::c_int as s16,
                            timerInit: 40 as libc::c_int as s16,
                            rollTargetInit: 0 as libc::c_int as s16,
                            fovTargetInit: 40.0f32,
                            lerpStepScale: 1.0f32,
                            atTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: 0.0f32,
                                              y: 50.0f32,
                                              z: -40.0f32,};
                                    init
                                },
                            eyeTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: 0.0f32,
                                              y: 60.0f32,
                                              z: -160.0f32,};
                                    init
                                },};
         init
     },
     {
         let mut init =
             OnePointCsFull{actionFlags: 0x4 as libc::c_int as u8_0,
                            unk_01: 0x4d as libc::c_int as u8_0,
                            initFlags: 0x4242 as libc::c_int as s16,
                            timerInit: 40 as libc::c_int as s16,
                            rollTargetInit: 0 as libc::c_int as s16,
                            fovTargetInit: 60.0f32,
                            lerpStepScale: 0.3f32,
                            atTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: 0.0f32,
                                              y: 90.0f32,
                                              z: -40.0f32,};
                                    init
                                },
                            eyeTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: 0.0f32,
                                              y: 60.0f32,
                                              z: -160.0f32,};
                                    init
                                },};
         init
     },
     {
         let mut init =
             OnePointCsFull{actionFlags: 0x4 as libc::c_int as u8_0,
                            unk_01: 0x1 as libc::c_int as u8_0,
                            initFlags: 0x2121 as libc::c_int as s16,
                            timerInit: 10 as libc::c_int as s16,
                            rollTargetInit: 0 as libc::c_int as s16,
                            fovTargetInit: 60.0f32,
                            lerpStepScale: 0.2f32,
                            atTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: 0.0f32,
                                              y: -10.0f32,
                                              z: 10.0f32,};
                                    init
                                },
                            eyeTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: 0.0f32,
                                              y: 10.0f32,
                                              z: -80.0f32,};
                                    init
                                },};
         init
     },
     {
         let mut init =
             OnePointCsFull{actionFlags: 0x12 as libc::c_int as u8_0,
                            unk_01: 0xff as libc::c_int as u8_0,
                            initFlags: 0 as libc::c_int as s16,
                            timerInit: 1 as libc::c_int as s16,
                            rollTargetInit: 0 as libc::c_int as s16,
                            fovTargetInit: 60.0f32,
                            lerpStepScale: 1.0f32,
                            atTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: -1.0f32,
                                              y: -1.0f32,
                                              z: -1.0f32,};
                                    init
                                },
                            eyeTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: -1.0f32,
                                              y: -1.0f32,
                                              z: -1.0f32,};
                                    init
                                },};
         init
     }];
static mut D_801234FC: [OnePointCsFull; 5] =
    [{
         let mut init =
             OnePointCsFull{actionFlags: 0x1 as libc::c_int as u8_0,
                            unk_01: 0x5 as libc::c_int as u8_0,
                            initFlags: 0x441 as libc::c_int as s16,
                            timerInit: 10 as libc::c_int as s16,
                            rollTargetInit: 0 as libc::c_int as s16,
                            fovTargetInit: 70.0f32,
                            lerpStepScale: 1.0f32,
                            atTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: 0.0f32,
                                              y: -10.0f32,
                                              z: 20.0f32,};
                                    init
                                },
                            eyeTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: 0.0f32,
                                              y: 0.0f32,
                                              z: 120.0f32,};
                                    init
                                },};
         init
     },
     {
         let mut init =
             OnePointCsFull{actionFlags: 0x3 as libc::c_int as u8_0,
                            unk_01: 0xff as libc::c_int as u8_0,
                            initFlags: 0x4141 as libc::c_int as s16,
                            timerInit: 30 as libc::c_int as s16,
                            rollTargetInit: 0 as libc::c_int as s16,
                            fovTargetInit: 50.0f32,
                            lerpStepScale: 0.1f32,
                            atTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: 0.0f32,
                                              y: -10.0f32,
                                              z: 20.0f32,};
                                    init
                                },
                            eyeTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: 0.0f32,
                                              y: 10.0f32,
                                              z: 80.0f32,};
                                    init
                                },};
         init
     },
     {
         let mut init =
             OnePointCsFull{actionFlags: 0x10 as libc::c_int as u8_0,
                            unk_01: 0x1 as libc::c_int as u8_0,
                            initFlags: 0 as libc::c_int as s16,
                            timerInit: 1 as libc::c_int as s16,
                            rollTargetInit: 0 as libc::c_int as s16,
                            fovTargetInit: 60.0f32,
                            lerpStepScale: 1.0f32,
                            atTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: -1.0f32,
                                              y: -1.0f32,
                                              z: -1.0f32,};
                                    init
                                },
                            eyeTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: -1.0f32,
                                              y: -1.0f32,
                                              z: -1.0f32,};
                                    init
                                },};
         init
     },
     {
         let mut init =
             OnePointCsFull{actionFlags: 0x82 as libc::c_int as u8_0,
                            unk_01: 0xff as libc::c_int as u8_0,
                            initFlags: 0x2121 as libc::c_int as s16,
                            timerInit: 10 as libc::c_int as s16,
                            rollTargetInit: 0 as libc::c_int as s16,
                            fovTargetInit: 60.0f32,
                            lerpStepScale: 0.9f32,
                            atTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: 0.0f32,
                                              y: -10.0f32,
                                              z: 0.0f32,};
                                    init
                                },
                            eyeTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: 0.0f32,
                                              y: 10.0f32,
                                              z: -80.0f32,};
                                    init
                                },};
         init
     },
     {
         let mut init =
             OnePointCsFull{actionFlags: 0x12 as libc::c_int as u8_0,
                            unk_01: 0xff as libc::c_int as u8_0,
                            initFlags: 0 as libc::c_int as s16,
                            timerInit: 1 as libc::c_int as s16,
                            rollTargetInit: 0 as libc::c_int as s16,
                            fovTargetInit: 60.0f32,
                            lerpStepScale: 1.0f32,
                            atTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: -1.0f32,
                                              y: -1.0f32,
                                              z: -1.0f32,};
                                    init
                                },
                            eyeTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: -1.0f32,
                                              y: -1.0f32,
                                              z: -1.0f32,};
                                    init
                                },};
         init
     }];
static mut D_801235C4: [OnePointCsFull; 5] =
    [{
         let mut init =
             OnePointCsFull{actionFlags: 0xf as libc::c_int as u8_0,
                            unk_01: 0x1 as libc::c_int as u8_0,
                            initFlags: 0x4141 as libc::c_int as s16,
                            timerInit: 1 as libc::c_int as s16,
                            rollTargetInit: 0 as libc::c_int as s16,
                            fovTargetInit: 50.0f32,
                            lerpStepScale: 1.0f32,
                            atTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: 0.0f32,
                                              y: -10.0f32,
                                              z: 20.0f32,};
                                    init
                                },
                            eyeTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: 0.0f32,
                                              y: 10.0f32,
                                              z: 60.0f32,};
                                    init
                                },};
         init
     },
     {
         let mut init =
             OnePointCsFull{actionFlags: 0x83 as libc::c_int as u8_0,
                            unk_01: 0xff as libc::c_int as u8_0,
                            initFlags: 0x441 as libc::c_int as s16,
                            timerInit: 39 as libc::c_int as s16,
                            rollTargetInit: 0 as libc::c_int as s16,
                            fovTargetInit: 70.0f32,
                            lerpStepScale: 0.1f32,
                            atTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: 0.0f32,
                                              y: -10.0f32,
                                              z: 20.0f32,};
                                    init
                                },
                            eyeTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: 0.0f32,
                                              y: 0.0f32,
                                              z: 100.0f32,};
                                    init
                                },};
         init
     },
     {
         let mut init =
             OnePointCsFull{actionFlags: 0x10 as libc::c_int as u8_0,
                            unk_01: 0xff as libc::c_int as u8_0,
                            initFlags: 0 as libc::c_int as s16,
                            timerInit: 1 as libc::c_int as s16,
                            rollTargetInit: 0 as libc::c_int as s16,
                            fovTargetInit: 60.0f32,
                            lerpStepScale: 1.0f32,
                            atTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: -1.0f32,
                                              y: -1.0f32,
                                              z: -1.0f32,};
                                    init
                                },
                            eyeTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: -1.0f32,
                                              y: -1.0f32,
                                              z: -1.0f32,};
                                    init
                                },};
         init
     },
     {
         let mut init =
             OnePointCsFull{actionFlags: 0x82 as libc::c_int as u8_0,
                            unk_01: 0xff as libc::c_int as u8_0,
                            initFlags: 0x2121 as libc::c_int as s16,
                            timerInit: 15 as libc::c_int as s16,
                            rollTargetInit: 0 as libc::c_int as s16,
                            fovTargetInit: 60.0f32,
                            lerpStepScale: 0.9f32,
                            atTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: 0.0f32,
                                              y: -10.0f32,
                                              z: 0.0f32,};
                                    init
                                },
                            eyeTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: 0.0f32,
                                              y: 10.0f32,
                                              z: -80.0f32,};
                                    init
                                },};
         init
     },
     {
         let mut init =
             OnePointCsFull{actionFlags: 0x12 as libc::c_int as u8_0,
                            unk_01: 0xff as libc::c_int as u8_0,
                            initFlags: 0 as libc::c_int as s16,
                            timerInit: 1 as libc::c_int as s16,
                            rollTargetInit: 0 as libc::c_int as s16,
                            fovTargetInit: 60.0f32,
                            lerpStepScale: 1.0f32,
                            atTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: -1.0f32,
                                              y: -1.0f32,
                                              z: -1.0f32,};
                                    init
                                },
                            eyeTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: -1.0f32,
                                              y: -1.0f32,
                                              z: -1.0f32,};
                                    init
                                },};
         init
     }];
static mut D_8012368C: [OnePointCsFull; 4] =
    [{
         let mut init =
             OnePointCsFull{actionFlags: 0xf as libc::c_int as u8_0,
                            unk_01: 0xff as libc::c_int as u8_0,
                            initFlags: 0x101 as libc::c_int as s16,
                            timerInit: 10 as libc::c_int as s16,
                            rollTargetInit: 0 as libc::c_int as s16,
                            fovTargetInit: 60.0f32,
                            lerpStepScale: 1.0f32,
                            atTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: -1110.0f32,
                                              y: -180.0f32,
                                              z: -840.0f32,};
                                    init
                                },
                            eyeTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: -985.0f32,
                                              y: -220.0f32,
                                              z: -840.0f32,};
                                    init
                                },};
         init
     },
     {
         let mut init =
             OnePointCsFull{actionFlags: 0x2 as libc::c_int as u8_0,
                            unk_01: 0xff as libc::c_int as u8_0,
                            initFlags: 0x101 as libc::c_int as s16,
                            timerInit: 70 as libc::c_int as s16,
                            rollTargetInit: -(45 as libc::c_int) as s16,
                            fovTargetInit: 75.0f32,
                            lerpStepScale: 1.0f32,
                            atTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: -1060.0f32,
                                              y: -160.0f32,
                                              z: -840.0f32,};
                                    init
                                },
                            eyeTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: -1005.0f32,
                                              y: -230.0f32,
                                              z: -840.0f32,};
                                    init
                                },};
         init
     },
     {
         let mut init =
             OnePointCsFull{actionFlags: 0xf as libc::c_int as u8_0,
                            unk_01: 0xff as libc::c_int as u8_0,
                            initFlags: 0 as libc::c_int as s16,
                            timerInit: 10 as libc::c_int as s16,
                            rollTargetInit: -(45 as libc::c_int) as s16,
                            fovTargetInit: 75.0f32,
                            lerpStepScale: 1.0f32,
                            atTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: -1.0f32,
                                              y: -1.0f32,
                                              z: -1.0f32,};
                                    init
                                },
                            eyeTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: -1.0f32,
                                              y: -1.0f32,
                                              z: -1.0f32,};
                                    init
                                },};
         init
     },
     {
         let mut init =
             OnePointCsFull{actionFlags: 0xf as libc::c_int as u8_0,
                            unk_01: 0xff as libc::c_int as u8_0,
                            initFlags: 0x101 as libc::c_int as s16,
                            timerInit: 180 as libc::c_int as s16,
                            rollTargetInit: 9 as libc::c_int as s16,
                            fovTargetInit: 80.0f32,
                            lerpStepScale: 1.0f32,
                            atTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: -1205.0f32,
                                              y: -175.0f32,
                                              z: -840.0f32,};
                                    init
                                },
                            eyeTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: -1305.0f32,
                                              y: -230.0f32,
                                              z: -828.0f32,};
                                    init
                                },};
         init
     }];
static mut D_8012372C: [OnePointCsFull; 4] =
    [{
         let mut init =
             OnePointCsFull{actionFlags: 0xf as libc::c_int as u8_0,
                            unk_01: 0xff as libc::c_int as u8_0,
                            initFlags: 0x142 as libc::c_int as s16,
                            timerInit: 10 as libc::c_int as s16,
                            rollTargetInit: 0 as libc::c_int as s16,
                            fovTargetInit: 70.0f32,
                            lerpStepScale: 1.0f32,
                            atTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: 0.0f32,
                                              y: 80.0f32,
                                              z: 0.0f32,};
                                    init
                                },
                            eyeTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: -1650.0f32,
                                              y: 200.0f32,
                                              z: -2920.0f32,};
                                    init
                                },};
         init
     },
     {
         let mut init =
             OnePointCsFull{actionFlags: 0x2 as libc::c_int as u8_0,
                            unk_01: 0xff as libc::c_int as u8_0,
                            initFlags: 0x142 as libc::c_int as s16,
                            timerInit: 110 as libc::c_int as s16,
                            rollTargetInit: -(2 as libc::c_int) as s16,
                            fovTargetInit: 50.0f32,
                            lerpStepScale: 0.5f32,
                            atTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: 0.0f32,
                                              y: 150.0f32,
                                              z: 0.0f32,};
                                    init
                                },
                            eyeTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: -1320.0f32,
                                              y: 170.0f32,
                                              z: -2900.0f32,};
                                    init
                                },};
         init
     },
     {
         let mut init =
             OnePointCsFull{actionFlags: 0xb as libc::c_int as u8_0,
                            unk_01: 0xff as libc::c_int as u8_0,
                            initFlags: 0x4242 as libc::c_int as s16,
                            timerInit: 100 as libc::c_int as s16,
                            rollTargetInit: 2 as libc::c_int as s16,
                            fovTargetInit: 70.0f32,
                            lerpStepScale: 0.1f32,
                            atTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: 0.0f32,
                                              y: 150.0f32,
                                              z: 50.0f32,};
                                    init
                                },
                            eyeTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: -1.0f32,
                                              y: -1.0f32,
                                              z: -1.0f32,};
                                    init
                                },};
         init
     },
     {
         let mut init =
             OnePointCsFull{actionFlags: 0x3 as libc::c_int as u8_0,
                            unk_01: 0xff as libc::c_int as u8_0,
                            initFlags: 0x4242 as libc::c_int as s16,
                            timerInit: 60 as libc::c_int as s16,
                            rollTargetInit: 2 as libc::c_int as s16,
                            fovTargetInit: 45.0f32,
                            lerpStepScale: 0.01f32,
                            atTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: 0.0f32,
                                              y: 150.0f32,
                                              z: 50.0f32,};
                                    init
                                },
                            eyeTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: 0.0f32,
                                              y: 200.0f32,
                                              z: -80.0f32,};
                                    init
                                },};
         init
     }];
static mut D_801237CC: [OnePointCsFull; 5] =
    [{
         let mut init =
             OnePointCsFull{actionFlags: 0x8f as libc::c_int as u8_0,
                            unk_01: 0xff as libc::c_int as u8_0,
                            initFlags: 0x4242 as libc::c_int as s16,
                            timerInit: 20 as libc::c_int as s16,
                            rollTargetInit: 0 as libc::c_int as s16,
                            fovTargetInit: 50.0f32,
                            lerpStepScale: 1.0f32,
                            atTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: 0.0f32,
                                              y: 50.0f32,
                                              z: -10.0f32,};
                                    init
                                },
                            eyeTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: 0.0f32,
                                              y: 0.0f32,
                                              z: 100.0f32,};
                                    init
                                },};
         init
     },
     {
         let mut init =
             OnePointCsFull{actionFlags: 0xa as libc::c_int as u8_0,
                            unk_01: 0xff as libc::c_int as u8_0,
                            initFlags: 0x101 as libc::c_int as s16,
                            timerInit: 80 as libc::c_int as s16,
                            rollTargetInit: 0 as libc::c_int as s16,
                            fovTargetInit: 75.0f32,
                            lerpStepScale: 1.0f32,
                            atTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: 2900.0f32,
                                              y: 1300.0f32,
                                              z: 530.0f32,};
                                    init
                                },
                            eyeTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: 2800.0f32,
                                              y: 1190.0f32,
                                              z: 540.0f32,};
                                    init
                                },};
         init
     },
     {
         let mut init =
             OnePointCsFull{actionFlags: 0xf as libc::c_int as u8_0,
                            unk_01: 0xff as libc::c_int as u8_0,
                            initFlags: 0 as libc::c_int as s16,
                            timerInit: 10 as libc::c_int as s16,
                            rollTargetInit: 0 as libc::c_int as s16,
                            fovTargetInit: 75.0f32,
                            lerpStepScale: 1.0f32,
                            atTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: -1.0f32,
                                              y: -1.0f32,
                                              z: -1.0f32,};
                                    init
                                },
                            eyeTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: -1.0f32,
                                              y: -1.0f32,
                                              z: -1.0f32,};
                                    init
                                },};
         init
     },
     {
         let mut init =
             OnePointCsFull{actionFlags: 0x2 as libc::c_int as u8_0,
                            unk_01: 0xff as libc::c_int as u8_0,
                            initFlags: 0x101 as libc::c_int as s16,
                            timerInit: 55 as libc::c_int as s16,
                            rollTargetInit: 0 as libc::c_int as s16,
                            fovTargetInit: 75.0f32,
                            lerpStepScale: 1.0f32,
                            atTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: 2900.0f32,
                                              y: 1300.0f32,
                                              z: 530.0f32,};
                                    init
                                },
                            eyeTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: 1500.0f32,
                                              y: 1415.0f32,
                                              z: 650.0f32,};
                                    init
                                },};
         init
     },
     {
         let mut init =
             OnePointCsFull{actionFlags: 0xf as libc::c_int as u8_0,
                            unk_01: 0xff as libc::c_int as u8_0,
                            initFlags: 0 as libc::c_int as s16,
                            timerInit: 100 as libc::c_int as s16,
                            rollTargetInit: -(45 as libc::c_int) as s16,
                            fovTargetInit: 75.0f32,
                            lerpStepScale: 1.0f32,
                            atTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: -1.0f32,
                                              y: -1.0f32,
                                              z: -1.0f32,};
                                    init
                                },
                            eyeTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: -1.0f32,
                                              y: -1.0f32,
                                              z: -1.0f32,};
                                    init
                                },};
         init
     }];
static mut D_80123894: [OnePointCsFull; 3] =
    [{
         let mut init =
             OnePointCsFull{actionFlags: 0xf as libc::c_int as u8_0,
                            unk_01: 0xff as libc::c_int as u8_0,
                            initFlags: 0x101 as libc::c_int as s16,
                            timerInit: 60 as libc::c_int as s16,
                            rollTargetInit: 0 as libc::c_int as s16,
                            fovTargetInit: 60.0f32,
                            lerpStepScale: 1.0f32,
                            atTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: 0.0f32,
                                              y: 0.0f32,
                                              z: 0.0f32,};
                                    init
                                },
                            eyeTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: 0.0f32,
                                              y: 0.0f32,
                                              z: 0.0f32,};
                                    init
                                },};
         init
     },
     {
         let mut init =
             OnePointCsFull{actionFlags: 0xf as libc::c_int as u8_0,
                            unk_01: 0xff as libc::c_int as u8_0,
                            initFlags: 0x4242 as libc::c_int as s16,
                            timerInit: 30 as libc::c_int as s16,
                            rollTargetInit: 0 as libc::c_int as s16,
                            fovTargetInit: 50.0f32,
                            lerpStepScale: 1.0f32,
                            atTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: 0.0f32,
                                              y: 28.0f32,
                                              z: 0.0f32,};
                                    init
                                },
                            eyeTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: 0.0f32,
                                              y: 20.0f32,
                                              z: 40.0f32,};
                                    init
                                },};
         init
     },
     {
         let mut init =
             OnePointCsFull{actionFlags: 0xd as libc::c_int as u8_0,
                            unk_01: 0xff as libc::c_int as u8_0,
                            initFlags: 0 as libc::c_int as s16,
                            timerInit: 120 as libc::c_int as s16,
                            rollTargetInit: 0 as libc::c_int as s16,
                            fovTargetInit: 180.0f32,
                            lerpStepScale: 0.4f32,
                            atTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: 0.0f32,
                                              y: -5.0f32,
                                              z: 0.0f32,};
                                    init
                                },
                            eyeTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: 0.0f32,
                                              y: 2.0f32,
                                              z: 40.0f32,};
                                    init
                                },};
         init
     }];
static mut D_8012390C: [OnePointCsFull; 2] =
    [{
         let mut init =
             OnePointCsFull{actionFlags: 0xf as libc::c_int as u8_0,
                            unk_01: 0xff as libc::c_int as u8_0,
                            initFlags: 0x101 as libc::c_int as s16,
                            timerInit: 30 as libc::c_int as s16,
                            rollTargetInit: 0 as libc::c_int as s16,
                            fovTargetInit: 60.0f32,
                            lerpStepScale: 1.0f32,
                            atTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: 0.0f32,
                                              y: 0.0f32,
                                              z: 0.0f32,};
                                    init
                                },
                            eyeTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: 0.0f32,
                                              y: 0.0f32,
                                              z: 0.0f32,};
                                    init
                                },};
         init
     },
     {
         let mut init =
             OnePointCsFull{actionFlags: 0xf as libc::c_int as u8_0,
                            unk_01: 0xff as libc::c_int as u8_0,
                            initFlags: 0x4242 as libc::c_int as s16,
                            timerInit: 180 as libc::c_int as s16,
                            rollTargetInit: 0 as libc::c_int as s16,
                            fovTargetInit: 60.0f32,
                            lerpStepScale: 1.0f32,
                            atTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: 0.0f32,
                                              y: 78.0f32,
                                              z: 0.0f32,};
                                    init
                                },
                            eyeTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: 0.0f32,
                                              y: 78.0f32,
                                              z: 200.0f32,};
                                    init
                                },};
         init
     }];
static mut D_8012395C: [OnePointCsFull; 3] =
    [{
         let mut init =
             OnePointCsFull{actionFlags: 0xf as libc::c_int as u8_0,
                            unk_01: 0xff as libc::c_int as u8_0,
                            initFlags: 0x101 as libc::c_int as s16,
                            timerInit: 60 as libc::c_int as s16,
                            rollTargetInit: 0 as libc::c_int as s16,
                            fovTargetInit: 60.0f32,
                            lerpStepScale: 1.0f32,
                            atTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: 0.0f32,
                                              y: 0.0f32,
                                              z: 0.0f32,};
                                    init
                                },
                            eyeTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: 0.0f32,
                                              y: 0.0f32,
                                              z: 0.0f32,};
                                    init
                                },};
         init
     },
     {
         let mut init =
             OnePointCsFull{actionFlags: 0xf as libc::c_int as u8_0,
                            unk_01: 0xff as libc::c_int as u8_0,
                            initFlags: 0x4242 as libc::c_int as s16,
                            timerInit: 30 as libc::c_int as s16,
                            rollTargetInit: 0 as libc::c_int as s16,
                            fovTargetInit: 50.0f32,
                            lerpStepScale: 1.0f32,
                            atTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: 0.0f32,
                                              y: 28.0f32,
                                              z: 0.0f32,};
                                    init
                                },
                            eyeTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: 0.0f32,
                                              y: 20.0f32,
                                              z: -45.0f32,};
                                    init
                                },};
         init
     },
     {
         let mut init =
             OnePointCsFull{actionFlags: 0xd as libc::c_int as u8_0,
                            unk_01: 0xff as libc::c_int as u8_0,
                            initFlags: 0 as libc::c_int as s16,
                            timerInit: 120 as libc::c_int as s16,
                            rollTargetInit: 0 as libc::c_int as s16,
                            fovTargetInit: 180.0f32,
                            lerpStepScale: 0.4f32,
                            atTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: 0.0f32,
                                              y: -5.0f32,
                                              z: 0.0f32,};
                                    init
                                },
                            eyeTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: 0.0f32,
                                              y: 2.0f32,
                                              z: 45.0f32,};
                                    init
                                },};
         init
     }];
static mut D_801239D4: [OnePointCsFull; 3] =
    [{
         let mut init =
             OnePointCsFull{actionFlags: 0xf as libc::c_int as u8_0,
                            unk_01: 0xff as libc::c_int as u8_0,
                            initFlags: 0x4242 as libc::c_int as s16,
                            timerInit: 5 as libc::c_int as s16,
                            rollTargetInit: 0 as libc::c_int as s16,
                            fovTargetInit: 60.0f32,
                            lerpStepScale: 1.0f32,
                            atTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: 0.0f32,
                                              y: 20.0f32,
                                              z: 0.0f32,};
                                    init
                                },
                            eyeTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: 0.0f32,
                                              y: 40.0f32,
                                              z: -120.0f32,};
                                    init
                                },};
         init
     },
     {
         let mut init =
             OnePointCsFull{actionFlags: 0x9 as libc::c_int as u8_0,
                            unk_01: 0xff as libc::c_int as u8_0,
                            initFlags: 0x4242 as libc::c_int as s16,
                            timerInit: 0 as libc::c_int as s16,
                            rollTargetInit: 0 as libc::c_int as s16,
                            fovTargetInit: 60.0f32,
                            lerpStepScale: 1.0f32,
                            atTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: 0.0f32,
                                              y: 20.0f32,
                                              z: 0.0f32,};
                                    init
                                },
                            eyeTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: 0.0f32,
                                              y: 0.0f32,
                                              z: 0.0f32,};
                                    init
                                },};
         init
     },
     {
         let mut init =
             OnePointCsFull{actionFlags: 0x12 as libc::c_int as u8_0,
                            unk_01: 0xff as libc::c_int as u8_0,
                            initFlags: 0 as libc::c_int as s16,
                            timerInit: 1 as libc::c_int as s16,
                            rollTargetInit: 0 as libc::c_int as s16,
                            fovTargetInit: 60.0f32,
                            lerpStepScale: 1.0f32,
                            atTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: -1.0f32,
                                              y: -1.0f32,
                                              z: -1.0f32,};
                                    init
                                },
                            eyeTargetInit:
                                {
                                    let mut init =
                                        Vec3f{x: -1.0f32,
                                              y: -1.0f32,
                                              z: -1.0f32,};
                                    init
                                },};
         init
     }];
#[no_mangle]
pub unsafe extern "C" fn OnePointCutscene_AddVecSphToVec3f(mut dst:
                                                               *mut Vec3f,
                                                           mut src:
                                                               *mut Vec3f,
                                                           mut vecSph:
                                                               *mut VecSph) {
    let mut out: Vec3f = Vec3f{x: 0., y: 0., z: 0.,};
    let mut vec: Vec3f = Vec3f{x: 0., y: 0., z: 0.,};
    OLib_VecSphGeoToVec3f(&mut vec, vecSph);
    out.x = (*src).x + vec.x;
    out.y = (*src).y + vec.y;
    out.z = (*src).z + vec.z;
    !dst.is_null();
    *dst = out;
}
#[no_mangle]
pub unsafe extern "C" fn OnePointCutscene_Vec3fYaw(mut vec1: *mut Vec3f,
                                                   mut vec2: *mut Vec3f)
 -> s16 {
    return (Math_FAtan2F((*vec2).x - (*vec1).x, (*vec2).z - (*vec1).z) *
                (180.0f32 / 3.14159265358979323846f32) * 182.04167f32 +
                0.5f32) as s16;
}
#[no_mangle]
pub unsafe extern "C" fn OnePointCutscene_Vec3sToVec3f(mut src: *mut Vec3f,
                                                       mut dst: *mut Vec3s) {
    (*dst).x = (*src).x as s16;
    (*dst).y = (*src).y as s16;
    (*dst).z = (*src).z as s16;
}
#[no_mangle]
pub unsafe extern "C" fn OnePointCutscene_BgCheckLineTest(mut colCtx:
                                                              *mut CollisionContext,
                                                          mut vec1:
                                                              *mut Vec3f,
                                                          mut vec2:
                                                              *mut Vec3f)
 -> s32 {
    let mut posResult: Vec3f = Vec3f{x: 0., y: 0., z: 0.,};
    let mut bgId: s32 = 0;
    let mut outPoly: *mut CollisionPoly = 0 as *mut CollisionPoly;
    return BgCheck_CameraLineTest1(colCtx, vec1, vec2, &mut posResult,
                                   &mut outPoly, 1 as libc::c_int,
                                   1 as libc::c_int, 1 as libc::c_int,
                                   0 as libc::c_int, &mut bgId);
}
#[no_mangle]
pub unsafe extern "C" fn OnePointCutscene_RaycastFloor(mut colCtx:
                                                           *mut CollisionContext,
                                                       mut pos: *mut Vec3f)
 -> f32_0 {
    let mut outPoly: *mut CollisionPoly = 0 as *mut CollisionPoly;
    let mut bgId: s32 = 0;
    return BgCheck_EntityRaycastFloor3(colCtx, &mut outPoly, &mut bgId, pos);
}
#[no_mangle]
pub unsafe extern "C" fn OnePointCutscene_SetCsCamPoints(mut camera:
                                                             *mut Camera,
                                                         mut actionParameters:
                                                             s16,
                                                         mut initTimer: s16,
                                                         mut atPoints:
                                                             *mut CutsceneCameraPoint,
                                                         mut eyePoints:
                                                             *mut CutsceneCameraPoint) {
    let mut onePointCamData: *mut OnePointCsCamera =
        &mut (*camera).paramData as *mut [libc::c_char; 80] as
            *mut OnePointCsCamera;
    (*onePointCamData).atPoints = atPoints;
    (*onePointCamData).eyePoints = eyePoints;
    (*onePointCamData).actionParameters = actionParameters;
    (*onePointCamData).initTimer = initTimer;
}
#[no_mangle]
pub unsafe extern "C" fn OnePointCutscene_SetInfo(mut globalCtx:
                                                      *mut GlobalContext,
                                                  mut camIdx: s16,
                                                  mut csId: s16,
                                                  mut actor: *mut Actor,
                                                  mut timer: s16) -> s32 {
    let mut csCam: *mut Camera = (*globalCtx).cameraPtrs[camIdx as usize];
    let mut childCam: *mut Camera =
        (*globalCtx).cameraPtrs[(*csCam).childCamIdx as usize];
    let mut mainCam: *mut Camera =
        (*globalCtx).cameraPtrs[0 as libc::c_int as usize];
    let mut player: *mut Player = (*mainCam).player;
    let mut spD0: VecSph = VecSph{r: 0., pitch: 0, yaw: 0,};
    let mut i: s32 = 0;
    let mut spC0: Vec3f = Vec3f{x: 0., y: 0., z: 0.,};
    let mut spB4: Vec3f = Vec3f{x: 0., y: 0., z: 0.,};
    let mut spA0: PosRot =
        PosRot{pos: Vec3f{x: 0., y: 0., z: 0.,},
               rot: Vec3s{x: 0, y: 0, z: 0,},};
    let mut sp8C: PosRot =
        PosRot{pos: Vec3f{x: 0., y: 0., z: 0.,},
               rot: Vec3s{x: 0, y: 0, z: 0,},};
    let mut tempRand: f32_0 = 0.;
    let mut csInfo: *mut Unique9OnePointCs =
        (*csCam).paramData.as_mut_ptr() as *mut Unique9OnePointCs;
    match csId as libc::c_int {
        1020 => {
            if (timer as libc::c_int) < 20 as libc::c_int {
                timer = 20 as libc::c_int as s16
            }
            D_801208EC[0 as libc::c_int as usize].atTargetInit =
                (*globalCtx).view.lookAt;
            D_801208EC[0 as libc::c_int as usize].eyeTargetInit =
                (*globalCtx).view.eye;
            D_801208EC[0 as libc::c_int as usize].fovTargetInit =
                (*globalCtx).view.fovy;
            D_801208EC[1 as libc::c_int as usize].atTargetInit =
                (*mainCam).at;
            D_801208EC[1 as libc::c_int as usize].eyeTargetInit =
                (*mainCam).eye;
            D_801208EC[1 as libc::c_int as usize].fovTargetInit =
                (*mainCam).fov;
            D_801208EC[1 as libc::c_int as usize].timerInit =
                (timer as libc::c_int - 1 as libc::c_int) as s16;
            (*csCam).timer = (timer as libc::c_int + 1 as libc::c_int) as s16;
            D_801208EC[1 as libc::c_int as usize].lerpStepScale =
                1.0f32 / (0.5f32 * timer as libc::c_int as libc::c_float);
            (*csInfo).keyFrames = D_801208EC.as_mut_ptr();
            (*csInfo).keyFrameCnt = 3 as libc::c_int;
            func_800C0808(globalCtx, camIdx, player,
                          CAM_SET_CS_C as libc::c_int as s16);
        }
        1030 => {
            D_80120964[0 as libc::c_int as usize].atTargetInit =
                (*globalCtx).view.lookAt;
            D_80120964[0 as libc::c_int as usize].eyeTargetInit =
                (*globalCtx).view.eye;
            D_80120964[0 as libc::c_int as usize].fovTargetInit =
                (*globalCtx).view.fovy;
            OLib_Vec3fDiffToVecSphGeo(&mut spD0, &mut (*mainCam).at,
                                      &mut (*mainCam).eye);
            D_80120964[1 as libc::c_int as usize].eyeTargetInit.y =
                spD0.yaw as f32_0 * (360.0001525f32 / 65535.0f32);
            D_80120964[1 as libc::c_int as usize].timerInit =
                (timer as libc::c_int - 1 as libc::c_int) as s16;
            (*csInfo).keyFrames = D_80120964.as_mut_ptr();
            (*csInfo).keyFrameCnt = 2 as libc::c_int;
            func_800C0808(globalCtx, camIdx, player,
                          CAM_SET_CS_C as libc::c_int as s16);
        }
        5000 => {
            D_801209B4[1 as libc::c_int as usize].atTargetInit =
                (*globalCtx).view.lookAt;
            D_801209B4[0 as libc::c_int as usize].atTargetInit =
                D_801209B4[1 as libc::c_int as usize].atTargetInit;
            D_801209B4[0 as libc::c_int as usize].eyeTargetInit =
                (*globalCtx).view.eye;
            D_801209B4[2 as libc::c_int as usize].fovTargetInit =
                (*globalCtx).view.fovy;
            D_801209B4[0 as libc::c_int as usize].fovTargetInit =
                D_801209B4[2 as libc::c_int as usize].fovTargetInit;
            OLib_Vec3fDiffToVecSphGeo(&mut spD0, &mut (*actor).focus.pos,
                                      &mut (*mainCam).at);
            spD0.r = (*mainCam).dist;
            OnePointCutscene_AddVecSphToVec3f(&mut (*D_801209B4.as_mut_ptr().offset(1
                                                                                        as
                                                                                        libc::c_int
                                                                                        as
                                                                                        isize)).eyeTargetInit,
                                              &mut (*D_801209B4.as_mut_ptr().offset(1
                                                                                        as
                                                                                        libc::c_int
                                                                                        as
                                                                                        isize)).atTargetInit,
                                              &mut spD0);
            D_801209B4[1 as libc::c_int as usize].atTargetInit.y += 20.0f32;
            (*csInfo).keyFrames = D_801209B4.as_mut_ptr();
            (*csInfo).keyFrameCnt = 4 as libc::c_int;
            func_800C0808(globalCtx, camIdx, player,
                          CAM_SET_CS_C as libc::c_int as s16);
        }
        5010 => {
            func_800C0808(globalCtx, camIdx, player,
                          CAM_SET_CS_ATTENTION as libc::c_int as s16);
            Gameplay_CameraSetAtEye(globalCtx, camIdx, &mut (*mainCam).at,
                                    &mut (*mainCam).eye);
            (*csCam).roll = 0 as libc::c_int as s16
        }
        9500 => {
            (*csInfo).keyFrames = D_80120A54.as_mut_ptr();
            (*csInfo).keyFrameCnt = 3 as libc::c_int;
            func_800C0808(globalCtx, camIdx, player,
                          CAM_SET_CS_C as libc::c_int as s16);
        }
        2260 => {
            D_80120ACC[2 as libc::c_int as usize].atTargetInit.x =
                (if (*(*mainCam).globalCtx).state.frames &
                        1 as libc::c_int as libc::c_uint != 0 {
                     -10.0f32
                 } else { 10.0f32 }) + Rand_ZeroOne() * 8.0f32;
            D_80120ACC[0 as libc::c_int as usize].atTargetInit.x =
                D_80120ACC[2 as libc::c_int as usize].atTargetInit.x;
            D_80120ACC[2 as libc::c_int as usize].eyeTargetInit.x =
                (if (*(*mainCam).globalCtx).state.frames &
                        1 as libc::c_int as libc::c_uint != 0 {
                     20.0f32
                 } else { -20.0f32 }) + Rand_ZeroOne() * 5.0f32;
            D_80120ACC[0 as libc::c_int as usize].eyeTargetInit.x =
                D_80120ACC[2 as libc::c_int as usize].eyeTargetInit.x;
            (*csInfo).keyFrames = D_80120ACC.as_mut_ptr();
            (*csInfo).keyFrameCnt = 5 as libc::c_int;
            func_800C0808(globalCtx, camIdx, player,
                          CAM_SET_CS_C as libc::c_int as s16);
        }
        2270 => {
            (*csInfo).keyFrames = D_80120B94.as_mut_ptr();
            (*csInfo).keyFrameCnt = 11 as libc::c_int;
            i = 0 as libc::c_int;
            while i < (*csInfo).keyFrameCnt - 3 as libc::c_int {
                if D_80120B94[i as usize].actionFlags as libc::c_int !=
                       0x8f as libc::c_int {
                    D_80120B94[i as usize].atTargetInit.x =
                        Rand_ZeroOne() * 5.0f32;
                    D_80120B94[i as usize].atTargetInit.z =
                        Rand_ZeroOne() * 30.0f32 + 10.0f32;
                    D_80120B94[i as usize].eyeTargetInit.x =
                        Rand_ZeroOne() * 100.0f32 + 20.0f32;
                    D_80120B94[i as usize].eyeTargetInit.z =
                        Rand_ZeroOne() * 80.0f32 + 50.0f32
                }
                i += 1
            }
            D_80120B94[(camIdx as libc::c_int - 1 as libc::c_int) as
                           usize].eyeTargetInit.y =
                (if (*(*mainCam).globalCtx).state.frames &
                        1 as libc::c_int as libc::c_uint != 0 {
                     3.0f32
                 } else { -3.0f32 }) + Rand_ZeroOne();
            func_800C0808(globalCtx, camIdx, player,
                          CAM_SET_CS_C as libc::c_int as s16);
            i = Quake_Add(csCam, 5 as libc::c_int as u32_0) as s32;
            Quake_SetSpeed(i as s16, 400 as libc::c_int as s16);
            Quake_SetQuakeValues(i as s16, 4 as libc::c_int as s16,
                                 5 as libc::c_int as s16,
                                 40 as libc::c_int as s16,
                                 0x3c as libc::c_int as s16);
            Quake_SetCountdown(i as s16, 1600 as libc::c_int as s16);
        }
        2280 => {
            (*csInfo).keyFrames = D_80120D4C.as_mut_ptr();
            (*csInfo).keyFrameCnt = 7 as libc::c_int;
            i = 0 as libc::c_int;
            while i < (*csInfo).keyFrameCnt - 3 as libc::c_int {
                if D_80120D4C[i as usize].actionFlags as libc::c_int !=
                       0x8f as libc::c_int {
                    D_80120D4C[i as usize].atTargetInit.x =
                        Rand_ZeroOne() * 20.0f32;
                    D_80120D4C[i as usize].atTargetInit.z =
                        Rand_ZeroOne() * 40.0f32 + 10.0f32;
                    D_80120D4C[i as usize].eyeTargetInit.x =
                        Rand_ZeroOne() * 40.0f32 + 60.0f32;
                    D_80120D4C[i as usize].eyeTargetInit.z =
                        Rand_ZeroOne() * 40.0f32 + 80.0f32
                }
                i += 1
            }
            D_80120D4C[(camIdx as libc::c_int - 1 as libc::c_int) as
                           usize].eyeTargetInit.y =
                (if (*(*mainCam).globalCtx).state.frames &
                        1 as libc::c_int as libc::c_uint != 0 {
                     3.0f32
                 } else { -3.0f32 }) + Rand_ZeroOne();
            func_800C0808(globalCtx, camIdx, player,
                          CAM_SET_CS_C as libc::c_int as s16);
            i = Quake_Add(csCam, 5 as libc::c_int as u32_0) as s32;
            Quake_SetSpeed(i as s16, 400 as libc::c_int as s16);
            Quake_SetQuakeValues(i as s16, 2 as libc::c_int as s16,
                                 3 as libc::c_int as s16,
                                 200 as libc::c_int as s16,
                                 0x32 as libc::c_int as s16);
            Quake_SetCountdown(i as s16, 9999 as libc::c_int as s16);
        }
        2220 => {
            (*csInfo).keyFrames = D_80120E64.as_mut_ptr();
            (*csInfo).keyFrameCnt = 8 as libc::c_int;
            func_800C0808(globalCtx, camIdx, player,
                          CAM_SET_CS_C as libc::c_int as s16);
            i = Quake_Add(csCam, 5 as libc::c_int as u32_0) as s32;
            Quake_SetSpeed(i as s16, 400 as libc::c_int as s16);
            Quake_SetQuakeValues(i as s16, 2 as libc::c_int as s16,
                                 2 as libc::c_int as s16,
                                 50 as libc::c_int as s16,
                                 0 as libc::c_int as s16);
            Quake_SetCountdown(i as s16, 280 as libc::c_int as s16);
        }
        2230 => {
            if (*player).actor.world.pos.z < 1000.0f32 {
                D_80120FA4[0 as libc::c_int as usize].eyeTargetInit.x =
                    -D_80120FA4[0 as libc::c_int as usize].eyeTargetInit.x;
                D_80120FA4[2 as libc::c_int as usize].eyeTargetInit.x =
                    -D_80120FA4[2 as libc::c_int as usize].eyeTargetInit.x
            }
            (*csInfo).keyFrames = D_80120FA4.as_mut_ptr();
            (*csInfo).keyFrameCnt = 6 as libc::c_int;
            func_800C0808(globalCtx, camIdx, player,
                          CAM_SET_CS_C as libc::c_int as s16);
        }
        2340 => {
            (*csInfo).keyFrames = D_80121094.as_mut_ptr();
            (*csInfo).keyFrameCnt = 3 as libc::c_int;
            func_800C0808(globalCtx, camIdx, player,
                          CAM_SET_CS_C as libc::c_int as s16);
            i = Quake_Add(csCam, 5 as libc::c_int as u32_0) as s32;
            Quake_SetSpeed(i as s16, 400 as libc::c_int as s16);
            Quake_SetQuakeValues(i as s16, 2 as libc::c_int as s16,
                                 2 as libc::c_int as s16,
                                 50 as libc::c_int as s16,
                                 0 as libc::c_int as s16);
            Quake_SetCountdown(i as s16, 60 as libc::c_int as s16);
        }
        2350 => {
            (*csInfo).keyFrames = D_8012110C.as_mut_ptr();
            (*csInfo).keyFrameCnt = 3 as libc::c_int;
            func_800C0808(globalCtx, camIdx, player,
                          CAM_SET_CS_C as libc::c_int as s16);
        }
        2200 => {
            let mut sp82: s16 = 0;
            let mut sp80: s16 = 0;
            let mut sp7E: s16 = 0;
            let mut sp7C: s16 = 0;
            Actor_GetScreenPos(globalCtx, &mut (*player).actor, &mut sp82,
                               &mut sp7E);
            Actor_GetScreenPos(globalCtx, actor, &mut sp80, &mut sp7C);
            if sp82 as libc::c_int > 0 as libc::c_int &&
                   (sp82 as libc::c_int) < 320 as libc::c_int &&
                   sp7E as libc::c_int > 0 as libc::c_int &&
                   (sp7E as libc::c_int) < 240 as libc::c_int &&
                   sp80 as libc::c_int > 0 as libc::c_int &&
                   (sp80 as libc::c_int) < 320 as libc::c_int &&
                   sp7C as libc::c_int > 0 as libc::c_int &&
                   (sp7C as libc::c_int) < 240 as libc::c_int &&
                   OnePointCutscene_BgCheckLineTest(&mut (*globalCtx).colCtx,
                                                    &mut (*actor).focus.pos,
                                                    &mut (*player).actor.focus.pos)
                       == 0 {
                D_80121184[0 as libc::c_int as usize].atTargetInit.x =
                    ((*globalCtx).view.lookAt.x + (*actor).focus.pos.x) *
                        0.5f32;
                D_80121184[0 as libc::c_int as usize].atTargetInit.y =
                    ((*globalCtx).view.lookAt.y + (*actor).focus.pos.y) *
                        0.5f32;
                D_80121184[0 as libc::c_int as usize].atTargetInit.z =
                    ((*globalCtx).view.lookAt.z + (*actor).focus.pos.z) *
                        0.5f32;
                D_80121184[0 as libc::c_int as usize].eyeTargetInit =
                    (*globalCtx).view.eye;
                D_80121184[0 as libc::c_int as usize].eyeTargetInit.y =
                    (*player).actor.focus.pos.y + 20.0f32;
                D_80121184[0 as libc::c_int as usize].fovTargetInit =
                    (*mainCam).fov * 0.75f32;
                (*csInfo).keyFrames = D_80121184.as_mut_ptr();
                (*csInfo).keyFrameCnt = 2 as libc::c_int
            } else {
                D_801211D4[0 as libc::c_int as usize].atTargetInit.x =
                    (*actor).focus.pos.x;
                D_801211D4[0 as libc::c_int as usize].atTargetInit.y =
                    (*actor).focus.pos.y - 5.0f32;
                D_801211D4[0 as libc::c_int as usize].atTargetInit.z =
                    (*actor).focus.pos.z;
                spC0 = (*(actor as *mut EnSw)).unk_364;
                osSyncPrintf(b"%s(%d): xyz_t: %s (%f %f %f)\n\x00" as
                                 *const u8 as *const libc::c_char,
                             b"../z_onepointdemo.c\x00" as *const u8 as
                                 *const libc::c_char, 1671 as libc::c_int,
                             b"&cp\x00" as *const u8 as *const libc::c_char,
                             spC0.x as libc::c_double,
                             spC0.y as libc::c_double,
                             spC0.z as libc::c_double);
                D_801211D4[0 as libc::c_int as usize].eyeTargetInit.x =
                    (*actor).focus.pos.x + 120.0f32 * spC0.x -
                        Rand_ZeroOne() * 20.0f32;
                D_801211D4[0 as libc::c_int as usize].eyeTargetInit.y =
                    (*actor).focus.pos.y + 120.0f32 * spC0.y + 20.0f32;
                D_801211D4[0 as libc::c_int as usize].eyeTargetInit.z =
                    (*actor).focus.pos.z + 120.0f32 * spC0.z -
                        Rand_ZeroOne() * 20.0f32;
                (*csInfo).keyFrames = D_801211D4.as_mut_ptr();
                (*csInfo).keyFrameCnt = 2 as libc::c_int
            }
            Gameplay_ChangeCameraStatus(globalCtx, 0 as libc::c_int as s16,
                                        3 as libc::c_int as s16);
            func_800C0808(globalCtx, camIdx, player,
                          CAM_SET_CS_C as libc::c_int as s16);
        }
        2290 => {
            let mut rideActor: *mut Actor = (*player).rideActor;
            func_8002DF54(globalCtx, 0 as *mut Actor,
                          8 as libc::c_int as u8_0);
            if !rideActor.is_null() {
                (*rideActor).freezeTimer = 180 as libc::c_int as u16_0
            }
            (*csInfo).keyFrames = D_80121224.as_mut_ptr();
            (*csInfo).keyFrameCnt = 6 as libc::c_int;
            func_800C0808(globalCtx, camIdx, player,
                          CAM_SET_CS_C as libc::c_int as s16);
        }
        5120 => {
            func_8002DF54(globalCtx, 0 as *mut Actor,
                          8 as libc::c_int as u8_0);
            (*csInfo).keyFrames = D_80121314.as_mut_ptr();
            (*csInfo).keyFrameCnt = 1 as libc::c_int;
            func_800C0808(globalCtx, camIdx, player,
                          CAM_SET_CS_C as libc::c_int as s16);
        }
        4510 => {
            D_8012133C[0 as libc::c_int as usize].eyeTargetInit =
                (*actor).world.pos;
            D_8012133C[0 as libc::c_int as usize].eyeTargetInit.y =
                (*player).actor.world.pos.y + 40.0f32;
            func_8002DF54(globalCtx, 0 as *mut Actor,
                          8 as libc::c_int as u8_0);
            (*csInfo).keyFrames = D_8012133C.as_mut_ptr();
            (*csInfo).keyFrameCnt = 3 as libc::c_int;
            func_800C0808(globalCtx, camIdx, player,
                          CAM_SET_CS_C as libc::c_int as s16);
        }
        4500 => {
            Actor_GetFocus(&mut spA0, actor);
            spC0 = spA0.pos;
            spC0.y =
                OnePointCutscene_RaycastFloor(&mut (*globalCtx).colCtx,
                                              &mut spC0) + 40.0f32;
            spD0.r = 150.0f32;
            spD0.yaw = spA0.rot.y;
            spD0.pitch = 0x3e8 as libc::c_int as s16;
            OnePointCutscene_AddVecSphToVec3f(&mut spB4, &mut spC0,
                                              &mut spD0);
            Gameplay_CameraChangeSetting(globalCtx, camIdx,
                                         CAM_SET_FREE2 as libc::c_int as s16);
            Gameplay_CameraSetAtEye(globalCtx, camIdx, &mut spC0, &mut spB4);
            func_8002DF54(globalCtx, 0 as *mut Actor,
                          8 as libc::c_int as u8_0);
            (*csCam).roll = 0 as libc::c_int as s16;
            (*csCam).fov = 50.0f32;
            if (*csCam).childCamIdx as libc::c_int != 0 as libc::c_int {
                OnePointCutscene_EndCutscene(globalCtx, (*csCam).childCamIdx);
            }
        }
        2210 => {
            OLib_Vec3fDiffToVecSphGeo(&mut spD0,
                                      &mut (*player).actor.world.pos,
                                      &mut (*actor).world.pos);
            D_801213B4[2 as libc::c_int as usize].atTargetInit.y =
                spD0.yaw as f32_0 * (360.0001525f32 / 65535.0f32);
            D_801213B4[2 as libc::c_int as usize].eyeTargetInit.y =
                D_801213B4[2 as libc::c_int as usize].atTargetInit.y;
            D_801213B4[1 as libc::c_int as usize].eyeTargetInit.y =
                D_801213B4[2 as libc::c_int as usize].eyeTargetInit.y;
            D_801213B4[0 as libc::c_int as usize].eyeTargetInit.y =
                D_801213B4[1 as libc::c_int as usize].eyeTargetInit.y;
            if Rand_ZeroOne() < 0.0f32 {
                D_801213B4[3 as libc::c_int as usize].eyeTargetInit.x =
                    -D_801213B4[3 as libc::c_int as usize].eyeTargetInit.x
            }
            func_8002DF54(globalCtx, 0 as *mut Actor,
                          8 as libc::c_int as u8_0);
            (*csInfo).keyFrames = D_801213B4.as_mut_ptr();
            (*csInfo).keyFrameCnt = 5 as libc::c_int;
            func_800C0808(globalCtx, camIdx, player,
                          CAM_SET_CS_C as libc::c_int as s16);
        }
        1010 => {
            Gameplay_CameraChangeSetting(globalCtx, camIdx,
                                         CAM_SET_FREE2 as libc::c_int as s16);
            Gameplay_CameraSetAtEye(globalCtx, camIdx, &mut (*childCam).at,
                                    &mut (*childCam).eye);
            Gameplay_CameraSetFov(globalCtx, camIdx, (*childCam).fov);
            Gameplay_SetCameraRoll(globalCtx, camIdx, (*childCam).roll);
        }
        9601 => {
            Gameplay_CameraChangeSetting(globalCtx, camIdx,
                                         CAM_SET_CS_3 as libc::c_int as s16);
            Gameplay_CameraChangeSetting(globalCtx, 0 as libc::c_int as s16,
                                         (*mainCam).prevSetting);
            OnePointCutscene_SetCsCamPoints(csCam,
                                            (D_80120430 as libc::c_int |
                                                 0x1000 as libc::c_int) as
                                                s16, D_8012042C,
                                            D_80120308.as_mut_ptr(),
                                            D_80120398.as_mut_ptr());
        }
        9602 => {
            Gameplay_CameraChangeSetting(globalCtx, camIdx,
                                         CAM_SET_CS_3 as libc::c_int as s16);
            Gameplay_CameraChangeSetting(globalCtx, 0 as libc::c_int as s16,
                                         (*mainCam).prevSetting);
            OnePointCutscene_SetCsCamPoints(csCam,
                                            (D_80120430 as libc::c_int |
                                                 0x1000 as libc::c_int) as
                                                s16, D_8012042C,
                                            D_80120308.as_mut_ptr(),
                                            D_80120434.as_mut_ptr());
        }
        4175 => {
            (*csInfo).keyFrames = D_8012147C.as_mut_ptr();
            (*csInfo).keyFrameCnt = 4 as libc::c_int;
            func_800C0808(globalCtx, camIdx, player,
                          CAM_SET_CS_C as libc::c_int as s16);
        }
        4180 => {
            spC0.x = -1881.0f32;
            spC0.y = 766.0f32;
            spC0.z = -330.0f32;
            spB4.x = -1979.0f32;
            spB4.y = 703.0f32;
            spB4.z = -269.0f32;
            Gameplay_CameraChangeSetting(globalCtx, camIdx,
                                         CAM_SET_FREE2 as libc::c_int as s16);
            Gameplay_CameraSetAtEye(globalCtx, camIdx, &mut spC0, &mut spB4);
            (*csCam).roll = 6 as libc::c_int as s16;
            (*csCam).fov = 75.0f32;
            func_8002DF54(globalCtx, 0 as *mut Actor,
                          8 as libc::c_int as u8_0);
        }
        3040 => {
            func_8002DF54(globalCtx, 0 as *mut Actor,
                          8 as libc::c_int as u8_0);
            D_8012151C[0 as libc::c_int as usize].timerInit =
                (timer as libc::c_int - 1 as libc::c_int) as s16;
            (*csInfo).keyFrames = D_8012151C.as_mut_ptr();
            (*csInfo).keyFrameCnt = 2 as libc::c_int;
            func_800C0808(globalCtx, camIdx, player,
                          CAM_SET_CS_C as libc::c_int as s16);
        }
        3020 => {
            D_8012156C[1 as libc::c_int as usize].timerInit =
                (timer as libc::c_int - 1 as libc::c_int) as s16;
            if (*(*mainCam).globalCtx).state.frames &
                   1 as libc::c_int as libc::c_uint != 0 {
                D_8012156C[0 as libc::c_int as usize].atTargetInit.x =
                    -D_8012156C[0 as libc::c_int as usize].atTargetInit.x;
                D_8012156C[0 as libc::c_int as usize].eyeTargetInit.x =
                    -D_8012156C[0 as libc::c_int as usize].eyeTargetInit.x;
                D_8012156C[1 as libc::c_int as usize].atTargetInit.x =
                    -D_8012156C[1 as libc::c_int as usize].atTargetInit.x;
                D_8012156C[1 as libc::c_int as usize].eyeTargetInit.x =
                    -D_8012156C[1 as libc::c_int as usize].eyeTargetInit.x
            }
            tempRand = Rand_ZeroOne() * 15.0f32;
            D_8012156C[0 as libc::c_int as usize].eyeTargetInit.x += tempRand;
            D_8012156C[1 as libc::c_int as usize].eyeTargetInit.x += tempRand;
            (*csInfo).keyFrames = D_8012156C.as_mut_ptr();
            (*csInfo).keyFrameCnt = 2 as libc::c_int;
            func_800C0808(globalCtx, camIdx, player,
                          CAM_SET_CS_C as libc::c_int as s16);
            func_8002DF54(globalCtx, 0 as *mut Actor,
                          8 as libc::c_int as u8_0);
        }
        3010 => {
            D_801215BC[0 as libc::c_int as usize].timerInit = timer;
            (*csInfo).keyFrames = D_801215BC.as_mut_ptr();
            (*csInfo).keyFrameCnt = 1 as libc::c_int;
            func_800C0808(globalCtx, camIdx, player,
                          CAM_SET_CS_C as libc::c_int as s16);
        }
        3070 => {
            (*csInfo).keyFrames = D_801215E4.as_mut_ptr();
            (*csInfo).keyFrameCnt = 10 as libc::c_int;
            func_800C0808(globalCtx, camIdx, player,
                          CAM_SET_CS_C as libc::c_int as s16);
            i = Quake_Add(csCam, 3 as libc::c_int as u32_0) as s32;
            Quake_SetSpeed(i as s16, 22000 as libc::c_int as s16);
            Quake_SetQuakeValues(i as s16, 2 as libc::c_int as s16,
                                 0 as libc::c_int as s16,
                                 200 as libc::c_int as s16,
                                 0 as libc::c_int as s16);
            Quake_SetCountdown(i as s16, 10 as libc::c_int as s16);
        }
        3080 => {
            (*csInfo).keyFrames = D_80121774.as_mut_ptr();
            (*csInfo).keyFrameCnt = 4 as libc::c_int;
            func_800C0808(globalCtx, camIdx, player,
                          CAM_SET_CS_C as libc::c_int as s16);
        }
        3090 => {
            func_8002DF54(globalCtx, 0 as *mut Actor,
                          8 as libc::c_int as u8_0);
            (*csInfo).keyFrames = D_80121814.as_mut_ptr();
            (*csInfo).keyFrameCnt = 4 as libc::c_int;
            func_800C0808(globalCtx, camIdx, player,
                          CAM_SET_CS_C as libc::c_int as s16);
        }
        3100 => {
            spB4.x = 0.0f32;
            spB4.y = -280.0f32;
            spB4.z = -1400.0f32;
            Actor_GetFocus(&mut spA0, actor);
            spC0 = spA0.pos;
            func_800C0808(globalCtx, camIdx, player,
                          CAM_SET_PIVOT_VERTICAL as libc::c_int as s16);
            Gameplay_CameraSetAtEye(globalCtx, camIdx, &mut spC0, &mut spB4);
            (*csCam).roll = 0 as libc::c_int as s16;
            (*csCam).fov = 70.0f32;
            func_8002DF54(globalCtx, 0 as *mut Actor,
                          8 as libc::c_int as u8_0);
        }
        3380 | 3065 => {
            (*csInfo).keyFrames = D_801218B4.as_mut_ptr();
            (*csInfo).keyFrameCnt = 2 as libc::c_int;
            func_8002DF54(globalCtx, 0 as *mut Actor,
                          8 as libc::c_int as u8_0);
            func_800C0808(globalCtx, camIdx, player,
                          CAM_SET_CS_C as libc::c_int as s16);
            i = Quake_Add(csCam, 1 as libc::c_int as u32_0) as s32;
            Quake_SetSpeed(i as s16, 24000 as libc::c_int as s16);
            Quake_SetQuakeValues(i as s16, 2 as libc::c_int as s16,
                                 0 as libc::c_int as s16,
                                 0 as libc::c_int as s16,
                                 0 as libc::c_int as s16);
            Quake_SetCountdown(i as s16, 160 as libc::c_int as s16);
        }
        3060 => {
            (*csInfo).keyFrames = D_80121904.as_mut_ptr();
            (*csInfo).keyFrameCnt = 2 as libc::c_int;
            func_8002DF54(globalCtx, 0 as *mut Actor,
                          8 as libc::c_int as u8_0);
            func_800C0808(globalCtx, camIdx, player,
                          CAM_SET_CS_C as libc::c_int as s16);
        }
        3050 => {
            Gameplay_CameraChangeSetting(globalCtx, camIdx,
                                         CAM_SET_CS_3 as libc::c_int as s16);
            func_8002DF54(globalCtx, &mut (*player).actor,
                          5 as libc::c_int as u8_0);
            OnePointCutscene_SetCsCamPoints(csCam,
                                            (D_80120304 as libc::c_int |
                                                 0x2000 as libc::c_int) as
                                                s16, D_80120300,
                                            D_8012013C.as_mut_ptr(),
                                            D_8012021C.as_mut_ptr());
            func_80078884(0x4802 as libc::c_int as u16_0);
            OnePointCutscene_Vec3sToVec3f(&mut (*mainCam).at,
                                          &mut (*D_8012013C.as_mut_ptr().offset((D_801202FC
                                                                                     as
                                                                                     libc::c_int
                                                                                     -
                                                                                     2
                                                                                         as
                                                                                         libc::c_int)
                                                                                    as
                                                                                    isize)).pos);
            OnePointCutscene_Vec3sToVec3f(&mut (*mainCam).eye,
                                          &mut (*D_8012021C.as_mut_ptr().offset((D_801202FC
                                                                                     as
                                                                                     libc::c_int
                                                                                     -
                                                                                     2
                                                                                         as
                                                                                         libc::c_int)
                                                                                    as
                                                                                    isize)).pos);
            D_8012013C[(D_801202FC as libc::c_int - 3 as libc::c_int) as
                           usize].pos.x =
                (D_8012013C[(D_801202FC as libc::c_int - 3 as libc::c_int) as
                                usize].pos.x as libc::c_int +
                     (D_8012013C[(D_801202FC as libc::c_int -
                                      2 as libc::c_int) as usize].pos.x as
                          libc::c_int -
                          D_8012013C[(D_801202FC as libc::c_int -
                                          3 as libc::c_int) as usize].pos.x as
                              libc::c_int) / 2 as libc::c_int) as s16;
            D_8012013C[(D_801202FC as libc::c_int - 3 as libc::c_int) as
                           usize].pos.y =
                (D_8012013C[(D_801202FC as libc::c_int - 3 as libc::c_int) as
                                usize].pos.y as libc::c_int +
                     (D_8012013C[(D_801202FC as libc::c_int -
                                      2 as libc::c_int) as usize].pos.y as
                          libc::c_int -
                          D_8012013C[(D_801202FC as libc::c_int -
                                          3 as libc::c_int) as usize].pos.y as
                              libc::c_int) / 2 as libc::c_int) as s16;
            D_8012013C[(D_801202FC as libc::c_int - 3 as libc::c_int) as
                           usize].pos.z =
                (D_8012013C[(D_801202FC as libc::c_int - 3 as libc::c_int) as
                                usize].pos.z as libc::c_int +
                     (D_8012013C[(D_801202FC as libc::c_int -
                                      2 as libc::c_int) as usize].pos.z as
                          libc::c_int -
                          D_8012013C[(D_801202FC as libc::c_int -
                                          3 as libc::c_int) as usize].pos.z as
                              libc::c_int) / 2 as libc::c_int) as s16;
            D_8012021C[(D_801202FC as libc::c_int - 3 as libc::c_int) as
                           usize].pos.x =
                (D_8012021C[(D_801202FC as libc::c_int - 3 as libc::c_int) as
                                usize].pos.x as libc::c_int +
                     (D_8012021C[(D_801202FC as libc::c_int -
                                      2 as libc::c_int) as usize].pos.x as
                          libc::c_int -
                          D_8012021C[(D_801202FC as libc::c_int -
                                          3 as libc::c_int) as usize].pos.x as
                              libc::c_int) / 2 as libc::c_int) as s16;
            D_8012021C[(D_801202FC as libc::c_int - 3 as libc::c_int) as
                           usize].pos.y =
                (D_8012021C[(D_801202FC as libc::c_int - 3 as libc::c_int) as
                                usize].pos.y as libc::c_int +
                     (D_8012021C[(D_801202FC as libc::c_int -
                                      2 as libc::c_int) as usize].pos.y as
                          libc::c_int -
                          D_8012021C[(D_801202FC as libc::c_int -
                                          3 as libc::c_int) as usize].pos.y as
                              libc::c_int) / 2 as libc::c_int) as s16;
            D_8012021C[(D_801202FC as libc::c_int - 3 as libc::c_int) as
                           usize].pos.z =
                (D_8012021C[(D_801202FC as libc::c_int - 3 as libc::c_int) as
                                usize].pos.z as libc::c_int +
                     (D_8012021C[(D_801202FC as libc::c_int -
                                      2 as libc::c_int) as usize].pos.z as
                          libc::c_int -
                          D_8012021C[(D_801202FC as libc::c_int -
                                          3 as libc::c_int) as usize].pos.z as
                              libc::c_int) / 2 as libc::c_int) as s16;
            i = Quake_Add(mainCam, 3 as libc::c_int as u32_0) as s32;
            Quake_SetSpeed(i as s16, 30000 as libc::c_int as s16);
            Quake_SetQuakeValues(i as s16, 2 as libc::c_int as s16,
                                 1 as libc::c_int as s16,
                                 1 as libc::c_int as s16,
                                 0 as libc::c_int as s16);
            Quake_SetCountdown(i as s16, 200 as libc::c_int as s16);
        }
        3120 => {
            (*csInfo).keyFrames =
                D_80121954[-(timer as libc::c_int + 101 as libc::c_int) as
                               usize].as_mut_ptr();
            (*csCam).timer = 100 as libc::c_int as s16;
            (*csCam).unk_14C =
                ((*csCam).unk_14C as libc::c_int | 2 as libc::c_int) as s16;
            (*csInfo).keyFrameCnt = 2 as libc::c_int;
            func_8002DF54(globalCtx, 0 as *mut Actor,
                          8 as libc::c_int as u8_0);
            func_800C0808(globalCtx, camIdx, player,
                          CAM_SET_CS_C as libc::c_int as s16);
        }
        3130 => {
            (*csInfo).keyFrames = D_80121A44.as_mut_ptr();
            (*csInfo).keyFrameCnt = 12 as libc::c_int;
            func_8002DF54(globalCtx, 0 as *mut Actor,
                          8 as libc::c_int as u8_0);
            func_800C0808(globalCtx, camIdx, player,
                          CAM_SET_CS_C as libc::c_int as s16);
            (*csCam).unk_14C =
                ((*csCam).unk_14C as libc::c_int | 2 as libc::c_int) as s16
        }
        3140 => {
            D_80121C24[0 as libc::c_int as usize].atTargetInit =
                (*globalCtx).view.lookAt;
            D_80121C24[0 as libc::c_int as usize].eyeTargetInit =
                (*globalCtx).view.eye;
            D_80121C24[0 as libc::c_int as usize].fovTargetInit =
                (*globalCtx).view.fovy;
            (*csInfo).keyFrames = D_80121C24.as_mut_ptr();
            (*csInfo).keyFrameCnt = 7 as libc::c_int;
            func_800C0808(globalCtx, camIdx, player,
                          CAM_SET_CS_C as libc::c_int as s16);
        }
        3150 => {
            spC0.x = 1890.0f32;
            spC0.y = 886.0f32;
            spC0.z = -1432.0f32;
            spB4.x = 1729.0f32;
            spB4.y = 995.0f32;
            spB4.z = -1405.0f32;
            Gameplay_CameraChangeSetting(globalCtx, camIdx,
                                         CAM_SET_FREE2 as libc::c_int as s16);
            Gameplay_CameraSetAtEye(globalCtx, camIdx, &mut spC0, &mut spB4);
            (*csCam).roll = 0x50 as libc::c_int as s16;
            (*csCam).fov = 55.0f32;
            func_8002DF38(globalCtx, &mut (*player).actor,
                          8 as libc::c_int as u8_0);
        }
        3170 => {
            Actor_GetWorld(&mut spA0, actor);
            spC0 = spA0.pos;
            spD0.pitch = -(0x5dc as libc::c_int) as s16;
            spC0.y += 50.0f32;
            spD0.r = 250.0f32;
            Actor_GetWorld(&mut spA0, &mut (*player).actor);
            spD0.yaw =
                (OnePointCutscene_Vec3fYaw(&mut spC0, &mut spA0.pos) as
                     libc::c_int - 0x7d0 as libc::c_int) as s16;
            OnePointCutscene_AddVecSphToVec3f(&mut spB4, &mut spC0,
                                              &mut spD0);
            Gameplay_CameraChangeSetting(globalCtx, camIdx,
                                         CAM_SET_FREE2 as libc::c_int as s16);
            Gameplay_CameraSetAtEye(globalCtx, camIdx, &mut spC0, &mut spB4);
            Gameplay_CopyCamera(globalCtx, 0 as libc::c_int as s16, camIdx);
            (*csCam).roll = -(1 as libc::c_int) as s16;
            (*csCam).fov = 55.0f32;
            func_8002DF38(globalCtx, actor, 1 as libc::c_int as u8_0);
        }
        3160 => {
            Actor_GetWorld(&mut spA0, actor);
            spC0 = spA0.pos;
            spD0.pitch = 0 as libc::c_int as s16;
            spD0.yaw = spA0.rot.y;
            spD0.r = 150.0f32;
            OnePointCutscene_AddVecSphToVec3f(&mut spB4, &mut spC0,
                                              &mut spD0);
            Gameplay_CameraChangeSetting(globalCtx, camIdx,
                                         CAM_SET_FREE2 as libc::c_int as s16);
            Gameplay_CameraSetAtEye(globalCtx, camIdx, &mut spC0, &mut spB4);
            (*csCam).roll = 0 as libc::c_int as s16;
            (*csCam).fov = 55.0f32;
            func_8002DF38(globalCtx, &mut (*player).actor,
                          8 as libc::c_int as u8_0);
        }
        3180 => {
            Actor_GetWorldPosShapeRot(&mut spA0, actor);
            spC0 = spA0.pos;
            spC0.y += 120.0f32;
            spD0.r = 300.0f32;
            spD0.yaw = spA0.rot.y;
            spD0.pitch = -(0xaf0 as libc::c_int) as s16;
            OnePointCutscene_AddVecSphToVec3f(&mut spB4, &mut spC0,
                                              &mut spD0);
            Gameplay_CameraChangeSetting(globalCtx, camIdx,
                                         CAM_SET_FREE2 as libc::c_int as s16);
            Gameplay_CameraSetAtEye(globalCtx, camIdx, &mut spC0, &mut spB4);
            (*csCam).roll = 0 as libc::c_int as s16;
            (*csCam).fov = 60.0f32;
            func_8002DF38(globalCtx, actor, 1 as libc::c_int as u8_0);
        }
        3190 => {
            Gameplay_CameraChangeSetting(globalCtx, camIdx,
                                         CAM_SET_FOREST_DEFEAT_POE as
                                             libc::c_int as s16);
            Camera_ChangeMode(mainCam, CAM_MODE_NORMAL as libc::c_int as s16);
            func_8002DF38(globalCtx, actor, 0xc as libc::c_int as u8_0);
        }
        3230 => {
            spC0.x = 120.0f32;
            spC0.y = 265.0f32;
            spC0.z = -1570.0f32;
            spB4.x = 80.0f32;
            spB4.y = 445.0f32;
            spB4.z = -1425.0f32;
            Gameplay_CameraChangeSetting(globalCtx, camIdx,
                                         CAM_SET_FREE2 as libc::c_int as s16);
            Gameplay_CameraSetAtEye(globalCtx, camIdx, &mut spC0, &mut spB4);
            (*csCam).roll = 0x1e as libc::c_int as s16;
            (*csCam).fov = 75.0f32;
            func_8002DF38(globalCtx, &mut (*player).actor,
                          8 as libc::c_int as u8_0);
            Actor_GetWorldPosShapeRot(&mut spA0, actor);
            Actor_GetFocus(&mut sp8C, &mut (*player).actor);
            spC0.x = sp8C.pos.x;
            spC0.y = sp8C.pos.y + 70.0f32;
            spC0.z = sp8C.pos.z;
            OLib_Vec3fDiffToVecSphGeo(&mut spD0, &mut spA0.pos,
                                      &mut sp8C.pos);
            spD0.pitch = 0x5dc as libc::c_int as s16;
            spD0.r = 120.0f32;
            OnePointCutscene_AddVecSphToVec3f(&mut spB4, &mut spC0,
                                              &mut spD0);
            Gameplay_CameraSetAtEye(globalCtx, 0 as libc::c_int as s16,
                                    &mut spC0, &mut spB4);
            i = Quake_Add(csCam, 3 as libc::c_int as u32_0) as s32;
            Quake_SetSpeed(i as s16, 22000 as libc::c_int as s16);
            Quake_SetQuakeValues(i as s16, 1 as libc::c_int as s16,
                                 0 as libc::c_int as s16,
                                 0 as libc::c_int as s16,
                                 0 as libc::c_int as s16);
            Quake_SetCountdown(i as s16, 90 as libc::c_int as s16);
        }
        6010 => {
            Actor_GetWorld(&mut spA0, actor);
            spC0 = spA0.pos;
            spD0.pitch = 0 as libc::c_int as s16;
            spC0.y += 70.0f32;
            spD0.yaw =
                (spA0.rot.y as libc::c_int + 0x7fff as libc::c_int) as s16;
            spD0.r = 300.0f32;
            OnePointCutscene_AddVecSphToVec3f(&mut spB4, &mut spC0,
                                              &mut spD0);
            Gameplay_CameraChangeSetting(globalCtx, camIdx,
                                         CAM_SET_FREE2 as libc::c_int as s16);
            Gameplay_CameraSetAtEye(globalCtx, camIdx, &mut spC0, &mut spB4);
            (*csCam).roll = 0 as libc::c_int as s16;
            (*csCam).fov = 45.0f32;
            func_8002DF38(globalCtx, &mut (*player).actor,
                          8 as libc::c_int as u8_0);
        }
        3220 => {
            Actor_GetFocus(&mut spA0, actor);
            spC0 = spA0.pos;
            func_800C0808(globalCtx, camIdx, player,
                          CAM_SET_PIVOT_VERTICAL as libc::c_int as s16);
            Actor_GetWorld(&mut spA0, &mut (*player).actor);
            OLib_Vec3fDiffToVecSphGeo(&mut spD0, &mut spC0, &mut spA0.pos);
            spD0.yaw =
                (spD0.yaw as libc::c_int + 0x3e8 as libc::c_int) as s16;
            spD0.r = 400.0f32;
            OnePointCutscene_AddVecSphToVec3f(&mut spB4, &mut spC0,
                                              &mut spD0);
            spB4.y = spA0.pos.y + 60.0f32;
            Gameplay_CameraSetAtEye(globalCtx, camIdx, &mut spC0, &mut spB4);
            (*csCam).roll = 0 as libc::c_int as s16;
            (*csCam).fov = 75.0f32;
            (*player).currentYaw =
                (spD0.yaw as libc::c_int + 0x7fff as libc::c_int) as s16;
            (*player).actor.world.rot.y = (*player).currentYaw;
            (*player).actor.shape.rot.y = (*player).actor.world.rot.y;
            func_8002DF54(globalCtx, 0 as *mut Actor,
                          8 as libc::c_int as u8_0);
        }
        3240 => {
            D_80121D3C[2 as libc::c_int as usize].timerInit =
                (timer as libc::c_int - 5 as libc::c_int) as s16;
            (*csInfo).keyFrames = D_80121D3C.as_mut_ptr();
            (*csInfo).keyFrameCnt = 3 as libc::c_int;
            func_8002DF54(globalCtx, 0 as *mut Actor,
                          8 as libc::c_int as u8_0);
            func_800C0808(globalCtx, camIdx, player,
                          CAM_SET_CS_C as libc::c_int as s16);
        }
        6001 => {
            Gameplay_CameraChangeSetting(globalCtx, camIdx,
                                         CAM_SET_CS_3 as libc::c_int as s16);
            func_8002DF54(globalCtx, 0 as *mut Actor,
                          8 as libc::c_int as u8_0);
            Actor_GetWorld(&mut spA0, actor);
            if spA0.pos.z > -750.0f32 {
                OnePointCutscene_SetCsCamPoints(csCam, D_801208E8, D_801208E4,
                                                D_801206A0.as_mut_ptr(),
                                                D_80120820.as_mut_ptr());
            } else {
                OnePointCutscene_SetCsCamPoints(csCam, D_801208E8, D_801208E4,
                                                D_801206A0.as_mut_ptr(),
                                                D_80120760.as_mut_ptr());
            }
            i = Quake_Add(csCam, 1 as libc::c_int as u32_0) as s32;
            Quake_SetSpeed(i as s16, 32000 as libc::c_int as s16);
            Quake_SetQuakeValues(i as s16, 0 as libc::c_int as s16,
                                 0 as libc::c_int as s16,
                                 20 as libc::c_int as s16,
                                 0 as libc::c_int as s16);
            Quake_SetCountdown(i as s16,
                               (D_801208E4 as libc::c_int - 10 as libc::c_int)
                                   as s16);
        }
        3400 => {
            Gameplay_CameraChangeSetting(globalCtx, camIdx,
                                         CAM_SET_CS_3 as libc::c_int as s16);
            func_8002DF38(globalCtx, &mut (*player).actor,
                          8 as libc::c_int as u8_0);
            OnePointCutscene_SetCsCamPoints(csCam,
                                            (D_8012069C as libc::c_int |
                                                 0x2000 as libc::c_int) as
                                                s16, D_80120698,
                                            D_801204D4.as_mut_ptr(),
                                            D_801205B4.as_mut_ptr());
            OnePointCutscene_Vec3sToVec3f(&mut (*mainCam).eye,
                                          &mut (*D_801205B4.as_mut_ptr().offset((D_80120694
                                                                                     as
                                                                                     libc::c_int
                                                                                     -
                                                                                     2
                                                                                         as
                                                                                         libc::c_int)
                                                                                    as
                                                                                    isize)).pos);
            OnePointCutscene_Vec3sToVec3f(&mut (*mainCam).at,
                                          &mut (*D_801204D4.as_mut_ptr().offset((D_80120694
                                                                                     as
                                                                                     libc::c_int
                                                                                     -
                                                                                     2
                                                                                         as
                                                                                         libc::c_int)
                                                                                    as
                                                                                    isize)).pos);
            i = Quake_Add(csCam, 1 as libc::c_int as u32_0) as s32;
            Quake_SetSpeed(i as s16, 0x4e20 as libc::c_int as s16);
            Quake_SetQuakeValues(i as s16, 1 as libc::c_int as s16,
                                 0 as libc::c_int as s16,
                                 50 as libc::c_int as s16,
                                 0 as libc::c_int as s16);
            Quake_SetCountdown(i as s16,
                               (D_80120698 as libc::c_int - 20 as libc::c_int)
                                   as s16);
        }
        3390 => {
            (*player).currentYaw = -(0x3fd9 as libc::c_int) as s16;
            (*player).actor.world.rot.y = (*player).currentYaw;
            (*player).actor.shape.rot.y = (*player).actor.world.rot.y;
            (*csInfo).keyFrames = D_80121DB4.as_mut_ptr();
            (*csInfo).keyFrameCnt = 9 as libc::c_int;
            func_8002DF54(globalCtx, 0 as *mut Actor,
                          8 as libc::c_int as u8_0);
            func_800C0808(globalCtx, camIdx, player,
                          CAM_SET_CS_C as libc::c_int as s16);
        }
        3310 => {
            Gameplay_CameraChangeSetting(globalCtx, camIdx,
                                         CAM_SET_FIRE_STAIRCASE as libc::c_int
                                             as s16);
            func_8002DF54(globalCtx, 0 as *mut Actor,
                          8 as libc::c_int as u8_0);
            Gameplay_CopyCamera(globalCtx, camIdx, 0 as libc::c_int as s16);
            i = Quake_Add(csCam, 1 as libc::c_int as u32_0) as s32;
            Quake_SetSpeed(i as s16, 32000 as libc::c_int as s16);
            Quake_SetQuakeValues(i as s16, 2 as libc::c_int as s16,
                                 0 as libc::c_int as s16,
                                 0 as libc::c_int as s16,
                                 0 as libc::c_int as s16);
            Quake_SetCountdown(i as s16, timer);
        }
        3290 => {
            D_80121F1C[0 as libc::c_int as usize].atTargetInit =
                (*globalCtx).view.lookAt;
            D_80121F1C[0 as libc::c_int as usize].eyeTargetInit =
                (*globalCtx).view.eye;
            D_80121F1C[0 as libc::c_int as usize].fovTargetInit =
                (*globalCtx).view.fovy;
            Actor_GetFocus(&mut spA0, actor);
            (*player).currentYaw = spA0.rot.y;
            (*player).actor.world.rot.y = (*player).currentYaw;
            (*player).actor.shape.rot.y = (*player).actor.world.rot.y;
            (*csInfo).keyFrames = D_80121F1C.as_mut_ptr();
            (*csInfo).keyFrameCnt = 4 as libc::c_int;
            func_800C0808(globalCtx, camIdx, player,
                          CAM_SET_CS_C as libc::c_int as s16);
            i = Quake_Add(csCam, 3 as libc::c_int as u32_0) as s32;
            Quake_SetSpeed(i as s16, 12000 as libc::c_int as s16);
            Quake_SetQuakeValues(i as s16, 0 as libc::c_int as s16,
                                 0 as libc::c_int as s16,
                                 1000 as libc::c_int as s16,
                                 0 as libc::c_int as s16);
            Quake_SetCountdown(i as s16, 5 as libc::c_int as s16);
        }
        3340 => {
            D_80121FBC[0 as libc::c_int as usize].atTargetInit =
                (*globalCtx).view.lookAt;
            D_80121FBC[0 as libc::c_int as usize].eyeTargetInit =
                (*globalCtx).view.eye;
            D_80121FBC[0 as libc::c_int as usize].fovTargetInit =
                (*globalCtx).view.fovy;
            (*csInfo).keyFrames = D_80121FBC.as_mut_ptr();
            (*csInfo).keyFrameCnt = 4 as libc::c_int;
            func_8002DF54(globalCtx, 0 as *mut Actor,
                          8 as libc::c_int as u8_0);
            func_800C0808(globalCtx, camIdx, player,
                          CAM_SET_CS_C as libc::c_int as s16);
            i = Quake_Add(csCam, 3 as libc::c_int as u32_0) as s32;
            Quake_SetSpeed(i as s16, 12000 as libc::c_int as s16);
            Quake_SetQuakeValues(i as s16, 0 as libc::c_int as s16,
                                 0 as libc::c_int as s16,
                                 1000 as libc::c_int as s16,
                                 0 as libc::c_int as s16);
            Quake_SetCountdown(i as s16, 5 as libc::c_int as s16);
        }
        3360 => {
            (*csInfo).keyFrames = D_8012205C.as_mut_ptr();
            (*csInfo).keyFrameCnt = 3 as libc::c_int;
            func_8002DF38(globalCtx, &mut (*player).actor,
                          8 as libc::c_int as u8_0);
            func_800C0808(globalCtx, camIdx, player,
                          CAM_SET_CS_C as libc::c_int as s16);
        }
        3350 => {
            D_801220D4[0 as libc::c_int as usize].atTargetInit =
                (*globalCtx).view.lookAt;
            D_801220D4[0 as libc::c_int as usize].eyeTargetInit =
                (*globalCtx).view.eye;
            D_801220D4[0 as libc::c_int as usize].fovTargetInit =
                (*globalCtx).view.fovy;
            if (*actor).world.pos.x > 0.0f32 {
                D_801220D4[1 as libc::c_int as usize].rollTargetInit =
                    -(D_801220D4[1 as libc::c_int as usize].rollTargetInit as
                          libc::c_int) as s16;
                D_801220D4[2 as libc::c_int as usize].rollTargetInit =
                    -(D_801220D4[2 as libc::c_int as usize].rollTargetInit as
                          libc::c_int) as s16;
                D_801220D4[1 as libc::c_int as usize].atTargetInit.x =
                    -D_801220D4[1 as libc::c_int as usize].atTargetInit.x;
                D_801220D4[1 as libc::c_int as usize].atTargetInit.y =
                    50.0f32;
                D_801220D4[1 as libc::c_int as usize].eyeTargetInit.y =
                    80.0f32;
                D_801220D4[1 as libc::c_int as usize].eyeTargetInit.x =
                    -D_801220D4[1 as libc::c_int as usize].eyeTargetInit.x
            }
            func_8002DF54(globalCtx, 0 as *mut Actor,
                          8 as libc::c_int as u8_0);
            (*csInfo).keyFrames = D_801220D4.as_mut_ptr();
            (*csInfo).keyFrameCnt = 5 as libc::c_int;
            func_800C0808(globalCtx, camIdx, player,
                          CAM_SET_CS_C as libc::c_int as s16);
        }
        3330 => {
            (*csInfo).keyFrames = D_8012219C.as_mut_ptr();
            (*csInfo).keyFrameCnt = 7 as libc::c_int;
            func_8002DF38(globalCtx, &mut (*player).actor,
                          8 as libc::c_int as u8_0);
            func_800C0808(globalCtx, camIdx, player,
                          CAM_SET_CS_C as libc::c_int as s16);
        }
        3410 => {
            (*csInfo).keyFrames = D_801222B4.as_mut_ptr();
            (*csInfo).keyFrameCnt = 5 as libc::c_int;
            func_8002DF54(globalCtx, 0 as *mut Actor,
                          8 as libc::c_int as u8_0);
            func_800C0808(globalCtx, camIdx, player,
                          CAM_SET_CS_C as libc::c_int as s16);
            i = Quake_Add(csCam, 1 as libc::c_int as u32_0) as s32;
            Quake_SetSpeed(i as s16, 32000 as libc::c_int as s16);
            Quake_SetQuakeValues(i as s16, 4 as libc::c_int as s16,
                                 0 as libc::c_int as s16,
                                 0 as libc::c_int as s16,
                                 0 as libc::c_int as s16);
            Quake_SetCountdown(i as s16, 20 as libc::c_int as s16);
        }
        3450 => {
            (*csInfo).keyFrames = D_8012237C.as_mut_ptr();
            (*csInfo).keyFrameCnt = 2 as libc::c_int;
            func_8002DF38(globalCtx, &mut (*player).actor,
                          8 as libc::c_int as u8_0);
            func_800C0808(globalCtx, camIdx, player,
                          CAM_SET_CS_C as libc::c_int as s16);
            i = Quake_Add(csCam, 1 as libc::c_int as u32_0) as s32;
            Quake_SetSpeed(i as s16, 32000 as libc::c_int as s16);
            Quake_SetQuakeValues(i as s16, 2 as libc::c_int as s16,
                                 0 as libc::c_int as s16,
                                 0 as libc::c_int as s16,
                                 0 as libc::c_int as s16);
            Quake_SetCountdown(i as s16, 10 as libc::c_int as s16);
        }
        3440 => {
            (*csInfo).keyFrames = D_801223CC.as_mut_ptr();
            (*csInfo).keyFrameCnt = 6 as libc::c_int;
            func_8002DF54(globalCtx, 0 as *mut Actor,
                          8 as libc::c_int as u8_0);
            func_800C0808(globalCtx, camIdx, player,
                          CAM_SET_CS_C as libc::c_int as s16);
            (*player).stateFlags1 |=
                0x20000000 as libc::c_int as libc::c_uint;
            (*player).actor.freezeTimer = 90 as libc::c_int as u16_0;
            i = Quake_Add(csCam, 1 as libc::c_int as u32_0) as s32;
            Quake_SetSpeed(i as s16, 32000 as libc::c_int as s16);
            Quake_SetQuakeValues(i as s16, 2 as libc::c_int as s16,
                                 0 as libc::c_int as s16,
                                 0 as libc::c_int as s16,
                                 0 as libc::c_int as s16);
            Quake_SetCountdown(i as s16, 10 as libc::c_int as s16);
        }
        3430 => {
            (*csInfo).keyFrames = D_801224BC.as_mut_ptr();
            (*csInfo).keyFrameCnt = 7 as libc::c_int;
            func_8002DF54(globalCtx, 0 as *mut Actor,
                          8 as libc::c_int as u8_0);
            func_800C0808(globalCtx, camIdx, player,
                          CAM_SET_CS_C as libc::c_int as s16);
            i = Quake_Add(csCam, 1 as libc::c_int as u32_0) as s32;
            Quake_SetSpeed(i as s16, 32000 as libc::c_int as s16);
            Quake_SetQuakeValues(i as s16, 1 as libc::c_int as s16,
                                 0 as libc::c_int as s16,
                                 10 as libc::c_int as s16,
                                 0 as libc::c_int as s16);
            Quake_SetCountdown(i as s16, 20 as libc::c_int as s16);
        }
        4100 => {
            (*csInfo).keyFrames = D_801225D4.as_mut_ptr();
            (*csInfo).keyFrameCnt = 5 as libc::c_int;
            (*player).currentYaw = 0x3ffc as libc::c_int as s16;
            (*player).actor.world.rot.y = (*player).currentYaw;
            (*player).actor.shape.rot.y = (*player).actor.world.rot.y;
            func_800C0808(globalCtx, camIdx, player,
                          CAM_SET_CS_C as libc::c_int as s16);
            func_8002DF54(globalCtx, 0 as *mut Actor,
                          8 as libc::c_int as u8_0);
        }
        4110 => {
            (*csInfo).keyFrames = D_8012269C.as_mut_ptr();
            (*csInfo).keyFrameCnt = 3 as libc::c_int;
            func_8002DF38(globalCtx, &mut (*player).actor,
                          8 as libc::c_int as u8_0);
            func_800C0808(globalCtx, camIdx, player,
                          CAM_SET_CS_C as libc::c_int as s16);
        }
        4120 => {
            func_8002DF54(globalCtx, 0 as *mut Actor,
                          8 as libc::c_int as u8_0);
            D_80122714[1 as libc::c_int as usize].timerInit =
                80 as libc::c_int as s16;
            (*csInfo).keyFrames = D_80122714.as_mut_ptr();
            (*csInfo).keyFrameCnt = 4 as libc::c_int;
            func_800C0808(globalCtx, camIdx, player,
                          CAM_SET_CS_C as libc::c_int as s16);
        }
        4140 => {
            (*csInfo).keyFrames = D_801227B4.as_mut_ptr();
            (*csInfo).keyFrameCnt = 6 as libc::c_int;
            func_800C0808(globalCtx, camIdx, player,
                          CAM_SET_CS_C as libc::c_int as s16);
            Camera_ChangeMode(mainCam, CAM_MODE_NORMAL as libc::c_int as s16);
        }
        4150 => {
            (*csInfo).keyFrames = D_801228A4.as_mut_ptr();
            (*csInfo).keyFrameCnt = 5 as libc::c_int;
            func_8002DF54(globalCtx, 0 as *mut Actor,
                          8 as libc::c_int as u8_0);
            Camera_ChangeMode(mainCam, CAM_MODE_NORMAL as libc::c_int as s16);
            func_800C0808(globalCtx, camIdx, player,
                          CAM_SET_CS_C as libc::c_int as s16);
        }
        4160 => {
            (*csInfo).keyFrames = D_8012296C.as_mut_ptr();
            (*csInfo).keyFrameCnt = 4 as libc::c_int;
            func_8002DF54(globalCtx, 0 as *mut Actor,
                          8 as libc::c_int as u8_0);
            Camera_ChangeMode(mainCam, CAM_MODE_NORMAL as libc::c_int as s16);
            func_800C0808(globalCtx, camIdx, player,
                          CAM_SET_CS_C as libc::c_int as s16);
        }
        4170 => {
            (*csInfo).keyFrames = D_80122A0C.as_mut_ptr();
            (*csInfo).keyFrameCnt = 2 as libc::c_int;
            func_8002DF54(globalCtx, 0 as *mut Actor,
                          8 as libc::c_int as u8_0);
            Camera_ChangeMode(mainCam, CAM_MODE_NORMAL as libc::c_int as s16);
            func_800C0808(globalCtx, camIdx, player,
                          CAM_SET_CS_C as libc::c_int as s16);
        }
        4190 => {
            (*csInfo).keyFrames = D_80122A5C.as_mut_ptr();
            (*csInfo).keyFrameCnt = 8 as libc::c_int;
            func_8002DF38(globalCtx, &mut (*player).actor,
                          8 as libc::c_int as u8_0);
            Camera_ChangeMode(mainCam, CAM_MODE_NORMAL as libc::c_int as s16);
            func_800C0808(globalCtx, camIdx, player,
                          CAM_SET_CS_C as libc::c_int as s16);
        }
        4200 => {
            (*csInfo).keyFrames = D_80122B9C.as_mut_ptr();
            (*csInfo).keyFrameCnt = 3 as libc::c_int;
            func_8002DF38(globalCtx, &mut (*player).actor,
                          8 as libc::c_int as u8_0);
            Camera_ChangeMode(mainCam, CAM_MODE_NORMAL as libc::c_int as s16);
            func_800C0808(globalCtx, camIdx, player,
                          CAM_SET_CS_C as libc::c_int as s16);
        }
        4210 => {
            (*player).actor.freezeTimer = timer as u16_0;
            (*csInfo).keyFrames = D_80122C14.as_mut_ptr();
            (*csInfo).keyFrameCnt = 1 as libc::c_int;
            func_800C0808(globalCtx, camIdx, player,
                          CAM_SET_CS_C as libc::c_int as s16);
            i = Quake_Add(csCam, 3 as libc::c_int as u32_0) as s32;
            Quake_SetSpeed(i as s16, 12000 as libc::c_int as s16);
            Quake_SetQuakeValues(i as s16, 0 as libc::c_int as s16,
                                 1 as libc::c_int as s16,
                                 100 as libc::c_int as s16,
                                 0 as libc::c_int as s16);
            Quake_SetCountdown(i as s16,
                               (timer as libc::c_int - 80 as libc::c_int) as
                                   s16);
        }
        4220 => {
            (*csInfo).keyFrames =
                if (*player).actor.world.pos.z < -15.0f32 {
                    D_80122C3C.as_mut_ptr()
                } else { D_80122C64.as_mut_ptr() };
            (*csInfo).keyFrameCnt = 1 as libc::c_int;
            func_800C0808(globalCtx, camIdx, player,
                          CAM_SET_CS_C as libc::c_int as s16);
            func_8002DF38(globalCtx, &mut (*player).actor,
                          1 as libc::c_int as u8_0);
            i = Quake_Add(csCam, 3 as libc::c_int as u32_0) as s32;
            Quake_SetSpeed(i as s16, 12000 as libc::c_int as s16);
            Quake_SetQuakeValues(i as s16, 0 as libc::c_int as s16,
                                 1 as libc::c_int as s16,
                                 10 as libc::c_int as s16,
                                 0 as libc::c_int as s16);
            Quake_SetCountdown(i as s16,
                               (timer as libc::c_int - 10 as libc::c_int) as
                                   s16);
        }
        4221 => {
            (*csInfo).keyFrames = D_80122C8C.as_mut_ptr();
            (*csInfo).keyFrameCnt = 1 as libc::c_int;
            func_8002DF54(globalCtx, 0 as *mut Actor,
                          8 as libc::c_int as u8_0);
            func_800C0808(globalCtx, camIdx, player,
                          CAM_SET_CS_C as libc::c_int as s16);
        }
        3260 => {
            func_8002DF54(globalCtx, 0 as *mut Actor,
                          8 as libc::c_int as u8_0);
            D_80122CB4[1 as libc::c_int as usize].timerInit =
                (timer as libc::c_int - 5 as libc::c_int) as s16;
            (*csInfo).keyFrames = D_80122CB4.as_mut_ptr();
            (*csInfo).keyFrameCnt = 2 as libc::c_int;
            func_800C0808(globalCtx, camIdx, player,
                          CAM_SET_CS_C as libc::c_int as s16);
        }
        3261 => {
            func_8002DF54(globalCtx, 0 as *mut Actor,
                          8 as libc::c_int as u8_0);
            D_80122D04[1 as libc::c_int as usize].timerInit =
                (timer as libc::c_int - 10 as libc::c_int) as s16;
            (*csInfo).keyFrames = D_80122D04.as_mut_ptr();
            (*csInfo).keyFrameCnt = 2 as libc::c_int;
            func_800C0808(globalCtx, camIdx, player,
                          CAM_SET_CS_C as libc::c_int as s16);
        }
        8010 => {
            (*csInfo).keyFrames = D_80122D54.as_mut_ptr();
            (*csInfo).keyFrameCnt = 3 as libc::c_int;
            func_800C0808(globalCtx, camIdx, player,
                          CAM_SET_CS_C as libc::c_int as s16);
        }
        8002 => {
            (*csInfo).keyFrames = D_80122DCC.as_mut_ptr();
            (*csInfo).keyFrameCnt = 3 as libc::c_int;
            func_800C0808(globalCtx, camIdx, player,
                          CAM_SET_CS_C as libc::c_int as s16);
        }
        8700 => {
            Actor_GetFocus(&mut spA0, actor);
            Actor_GetFocus(&mut sp8C, &mut (*player).actor);
            D_80122E44[(timer as libc::c_int & 1 as libc::c_int) as
                           usize][0 as libc::c_int as usize].atTargetInit.y =
                (spA0.pos.y - sp8C.pos.y) / 10.0f32 + 90.0f32;
            D_80122E44[(timer as libc::c_int & 1 as libc::c_int) as
                           usize][5 as libc::c_int as usize].atTargetInit =
                (*mainCam).at;
            (*csInfo).keyFrames =
                D_80122E44[(timer as libc::c_int & 1 as libc::c_int) as
                               usize].as_mut_ptr();
            (*csInfo).keyFrameCnt = 7 as libc::c_int;
            func_800C0808(globalCtx, camIdx, player,
                          CAM_SET_CS_C as libc::c_int as s16);
        }
        1100 => {
            let mut tempDiff: s32 =
                (*globalCtx).state.frames.wrapping_sub(sPrevFrameCs1100 as
                                                           libc::c_uint) as
                    s32;
            if tempDiff > 3600 as libc::c_int ||
                   tempDiff < -(3600 as libc::c_int) {
                (*csInfo).keyFrames = D_80123074.as_mut_ptr();
                (*csInfo).keyFrameCnt = 5 as libc::c_int
            } else {
                if (*globalCtx).state.frames &
                       1 as libc::c_int as libc::c_uint != 0 {
                    D_8012313C[0 as libc::c_int as usize].rollTargetInit =
                        -(D_8012313C[0 as libc::c_int as usize].rollTargetInit
                              as libc::c_int) as s16;
                    D_8012313C[0 as libc::c_int as usize].atTargetInit.y =
                        -D_8012313C[0 as libc::c_int as usize].atTargetInit.y;
                    D_8012313C[0 as libc::c_int as usize].eyeTargetInit.y =
                        -D_8012313C[0 as libc::c_int as
                                        usize].eyeTargetInit.y;
                    D_8012313C[1 as libc::c_int as usize].atTargetInit.y =
                        -D_8012313C[1 as libc::c_int as usize].atTargetInit.y
                }
                (*csInfo).keyFrames = D_8012313C.as_mut_ptr();
                (*csInfo).keyFrameCnt = 3 as libc::c_int
            }
            func_800C0808(globalCtx, camIdx, player,
                          CAM_SET_CS_C as libc::c_int as s16);
            sPrevFrameCs1100 = (*globalCtx).state.frames as s32
        }
        9806 => {
            (*csCam).timer = -(99 as libc::c_int) as s16;
            if func_800C0CB8(globalCtx) != 0 {
                func_800C0808(globalCtx, camIdx, player,
                              CAM_SET_TURN_AROUND as libc::c_int as s16);
                (*csCam).data2 = 0xc as libc::c_int as s16
            } else {
                Gameplay_CopyCamera(globalCtx, camIdx,
                                    0 as libc::c_int as s16);
                Gameplay_CameraChangeSetting(globalCtx, camIdx,
                                             CAM_SET_FREE2 as libc::c_int as
                                                 s16);
            }
        }
        9908 => {
            if func_800C0CB8(globalCtx) != 0 {
                D_801231B4[1 as libc::c_int as usize].eyeTargetInit.z =
                    if !(gSaveContext.linkAge == 0 as libc::c_int) {
                        100.0f32
                    } else { 120.0f32 };
                D_801231B4[0 as libc::c_int as usize].eyeTargetInit.z =
                    D_801231B4[1 as libc::c_int as usize].eyeTargetInit.z;
                if (*player).stateFlags1 &
                       0x8000000 as libc::c_int as libc::c_uint != 0 {
                    D_801231B4[2 as libc::c_int as usize].atTargetInit.z =
                        0.0f32
                }
                Actor_GetWorldPosShapeRot(&mut spA0, &mut (*player).actor);
                OLib_Vec3fDiffToVecSphGeo(&mut spD0, &mut spA0.pos,
                                          &mut (*mainCam).at);
                spD0.yaw =
                    (spD0.yaw as libc::c_int - spA0.rot.y as libc::c_int) as
                        s16;
                OLib_VecSphGeoToVec3f(&mut (*D_801231B4.as_mut_ptr().offset(3
                                                                                as
                                                                                libc::c_int
                                                                                as
                                                                                isize)).atTargetInit,
                                      &mut spD0);
                OLib_Vec3fDiffToVecSphGeo(&mut spD0, &mut spA0.pos,
                                          &mut (*mainCam).eye);
                spD0.yaw =
                    (spD0.yaw as libc::c_int - spA0.rot.y as libc::c_int) as
                        s16;
                OLib_VecSphGeoToVec3f(&mut (*D_801231B4.as_mut_ptr().offset(3
                                                                                as
                                                                                libc::c_int
                                                                                as
                                                                                isize)).eyeTargetInit,
                                      &mut spD0);
                D_801231B4[3 as libc::c_int as usize].fovTargetInit =
                    (*mainCam).fov;
                D_801231B4[3 as libc::c_int as usize].timerInit =
                    (timer as libc::c_int - 50 as libc::c_int) as s16;
                (*csInfo).keyFrames = D_801231B4.as_mut_ptr();
                (*csInfo).keyFrameCnt = 4 as libc::c_int;
                func_800C0808(globalCtx, camIdx, player,
                              CAM_SET_CS_C as libc::c_int as s16);
            } else {
                D_80123254[1 as libc::c_int as usize].timerInit =
                    (timer as libc::c_int - 1 as libc::c_int) as s16;
                D_80123254[0 as libc::c_int as usize].fovTargetInit =
                    (*mainCam).fov;
                D_80123254[1 as libc::c_int as usize].atTargetInit =
                    (*mainCam).at;
                D_80123254[0 as libc::c_int as usize].atTargetInit =
                    D_80123254[1 as libc::c_int as usize].atTargetInit;
                D_80123254[1 as libc::c_int as usize].eyeTargetInit =
                    (*mainCam).eye;
                D_80123254[0 as libc::c_int as usize].eyeTargetInit =
                    D_80123254[1 as libc::c_int as usize].eyeTargetInit;
                (*csInfo).keyFrames = D_80123254.as_mut_ptr();
                (*csInfo).keyFrameCnt = 2 as libc::c_int;
                func_800C0808(globalCtx, camIdx, player,
                              CAM_SET_CS_C as libc::c_int as s16);
            }
        }
        1000 => {
            D_801232A4[0 as libc::c_int as usize].atTargetInit =
                (*globalCtx).view.lookAt;
            D_801232A4[0 as libc::c_int as usize].eyeTargetInit =
                (*globalCtx).view.eye;
            D_801232A4[0 as libc::c_int as usize].fovTargetInit =
                (*globalCtx).view.fovy;
            (*csInfo).keyFrames = D_801232A4.as_mut_ptr();
            (*csInfo).keyFrameCnt = 1 as libc::c_int;
            func_800C0808(globalCtx, camIdx, player,
                          CAM_SET_CS_C as libc::c_int as s16);
        }
        8603 => {
            (*csInfo).keyFrames = D_801232CC.as_mut_ptr();
            (*csInfo).keyFrameCnt = 5 as libc::c_int;
            func_800C0808(globalCtx, camIdx, player,
                          CAM_SET_CS_C as libc::c_int as s16);
        }
        8604 => {
            (*csInfo).keyFrames = D_80123394.as_mut_ptr();
            (*csInfo).keyFrameCnt = 5 as libc::c_int;
            func_800C0808(globalCtx, camIdx, player,
                          CAM_SET_CS_C as libc::c_int as s16);
        }
        4000 => {
            (*csInfo).keyFrames = D_8012345C.as_mut_ptr();
            (*csInfo).keyFrameCnt = 4 as libc::c_int;
            func_800C0808(globalCtx, camIdx, player,
                          CAM_SET_CS_C as libc::c_int as s16);
        }
        4010 => {
            (*csInfo).keyFrames = D_801234FC.as_mut_ptr();
            (*csInfo).keyFrameCnt = 5 as libc::c_int;
            func_800C0808(globalCtx, camIdx, player,
                          CAM_SET_CS_C as libc::c_int as s16);
        }
        4011 => {
            (*csInfo).keyFrames = D_801235C4.as_mut_ptr();
            (*csInfo).keyFrameCnt = 5 as libc::c_int;
            func_800C0808(globalCtx, camIdx, player,
                          CAM_SET_CS_C as libc::c_int as s16);
        }
        4020 => {
            (*csInfo).keyFrames = D_8012368C.as_mut_ptr();
            (*csInfo).keyFrameCnt = 4 as libc::c_int;
            func_800C0808(globalCtx, camIdx, player,
                          CAM_SET_CS_C as libc::c_int as s16);
        }
        4021 => {
            (*csInfo).keyFrames = D_8012372C.as_mut_ptr();
            (*csInfo).keyFrameCnt = 4 as libc::c_int;
            func_800C0808(globalCtx, camIdx, player,
                          CAM_SET_CS_C as libc::c_int as s16);
        }
        4022 => {
            (*csCam).timer =
                (D_801237CC[0 as libc::c_int as usize].timerInit as
                     libc::c_int +
                     D_801237CC[3 as libc::c_int as usize].timerInit as
                         libc::c_int +
                     D_801237CC[1 as libc::c_int as usize].timerInit as
                         libc::c_int +
                     D_801237CC[2 as libc::c_int as usize].timerInit as
                         libc::c_int +
                     D_801237CC[4 as libc::c_int as usize].timerInit as
                         libc::c_int) as s16;
            (*csInfo).keyFrames = D_801237CC.as_mut_ptr();
            (*csInfo).keyFrameCnt = 5 as libc::c_int;
            func_800C0808(globalCtx, camIdx, player,
                          CAM_SET_CS_C as libc::c_int as s16);
        }
        9703 => {
            D_80123894[0 as libc::c_int as usize].atTargetInit =
                (*globalCtx).view.lookAt;
            D_80123894[0 as libc::c_int as usize].eyeTargetInit =
                (*globalCtx).view.eye;
            D_80123894[0 as libc::c_int as usize].fovTargetInit =
                (*globalCtx).view.fovy;
            if gSaveContext.linkAge == 0 as libc::c_int {
                D_80123894[1 as libc::c_int as usize].atTargetInit.y =
                    60.0f32;
                D_80123894[1 as libc::c_int as usize].eyeTargetInit.y =
                    52.0f32
            }
            (*csInfo).keyFrames = D_80123894.as_mut_ptr();
            (*csInfo).keyFrameCnt = 3 as libc::c_int;
            func_800C0808(globalCtx, camIdx, player,
                          CAM_SET_CS_C as libc::c_int as s16);
        }
        9704 => {
            D_8012390C[0 as libc::c_int as usize].atTargetInit =
                (*globalCtx).view.lookAt;
            D_8012390C[0 as libc::c_int as usize].eyeTargetInit =
                (*globalCtx).view.eye;
            D_8012390C[0 as libc::c_int as usize].fovTargetInit =
                (*globalCtx).view.fovy;
            (*csInfo).keyFrames = D_8012390C.as_mut_ptr();
            (*csInfo).keyFrameCnt = 2 as libc::c_int;
            func_800C0808(globalCtx, camIdx, player,
                          CAM_SET_CS_C as libc::c_int as s16);
        }
        9705 => {
            D_8012395C[0 as libc::c_int as usize].atTargetInit =
                (*globalCtx).view.lookAt;
            D_8012395C[0 as libc::c_int as usize].eyeTargetInit =
                (*globalCtx).view.eye;
            D_8012395C[0 as libc::c_int as usize].fovTargetInit =
                (*globalCtx).view.fovy;
            (*csInfo).keyFrames = D_8012395C.as_mut_ptr();
            (*csInfo).keyFrameCnt = 3 as libc::c_int;
            func_800C0808(globalCtx, camIdx, player,
                          CAM_SET_CS_C as libc::c_int as s16);
        }
        5110 => {
            D_801239D4[1 as libc::c_int as usize].timerInit =
                10 as libc::c_int as s16;
            (*csInfo).keyFrames = D_801239D4.as_mut_ptr();
            (*csInfo).keyFrameCnt = 3 as libc::c_int;
            func_800C0808(globalCtx, camIdx, actor as *mut Player,
                          CAM_SET_CS_C as libc::c_int as s16);
        }
        _ => {
            osSyncPrintf(b"\x1b[41;37monepointdemo camera: demo number not found !! (%d)\n\x1b[m\x00"
                             as *const u8 as *const libc::c_char,
                         csId as libc::c_int);
        }
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn OnePointCutscene_SetAsChild(mut globalCtx:
                                                         *mut GlobalContext,
                                                     mut newCamIdx: s16,
                                                     mut parentCamIdx: s16)
 -> s16 {
    let mut prevCamIdx: s16 =
        (*(*globalCtx).cameraPtrs[parentCamIdx as usize]).childCamIdx;
    (*(*globalCtx).cameraPtrs[newCamIdx as usize]).parentCamIdx =
        parentCamIdx;
    (*(*globalCtx).cameraPtrs[parentCamIdx as usize]).childCamIdx = newCamIdx;
    return prevCamIdx;
}
/* *
 * Removes a cutscene camera from the list. Returns the parent cam if the removed camera is active, otherwise returns
 * SUBCAM_NONE
 */
#[no_mangle]
pub unsafe extern "C" fn OnePointCutscene_RemoveCamera(mut globalCtx:
                                                           *mut GlobalContext,
                                                       mut camIdx: s16)
 -> s32 {
    let mut camera: *mut Camera = (*globalCtx).cameraPtrs[camIdx as usize];
    let mut nextCamIdx: s32 = 0;
    if (*camera).thisIdx as libc::c_int ==
           (*(*(*camera).globalCtx).cameraPtrs[(*camera).childCamIdx as
                                                   usize]).parentCamIdx as
               libc::c_int {
        (*(*(*camera).globalCtx).cameraPtrs[(*camera).childCamIdx as
                                                usize]).parentCamIdx =
            (*camera).parentCamIdx
    }
    if (*camera).thisIdx as libc::c_int ==
           (*(*(*camera).globalCtx).cameraPtrs[(*camera).parentCamIdx as
                                                   usize]).childCamIdx as
               libc::c_int {
        (*(*(*camera).globalCtx).cameraPtrs[(*camera).parentCamIdx as
                                                usize]).childCamIdx =
            (*camera).childCamIdx
    }
    nextCamIdx =
        if (*globalCtx).activeCamera as libc::c_int == camIdx as libc::c_int {
            (*camera).parentCamIdx as libc::c_int
        } else { -(1 as libc::c_int) };
    (*camera).parentCamIdx = 0 as libc::c_int as s16;
    (*camera).childCamIdx = (*camera).parentCamIdx;
    (*camera).timer = -(1 as libc::c_int) as s16;
    Gameplay_ClearCamera((*camera).globalCtx, (*camera).thisIdx);
    return nextCamIdx;
}
/* *
 * Creates a cutscene subcamera with the specified ID, duration, and targeted actor. The camera is placed into the
 * cutscene queue in front of the specified camera, then all lower priority demos in front of it are removed from the
 * queue.
 */
#[no_mangle]
pub unsafe extern "C" fn OnePointCutscene_Init(mut globalCtx:
                                                   *mut GlobalContext,
                                               mut csId: s16, mut timer: s16,
                                               mut actor: *mut Actor,
                                               mut parentCamIdx: s16) -> s16 {
    let mut temp1: s16 = 0;
    let mut temp2: s16 = 0;
    let mut csCamIdx: s16 = 0;
    let mut csCam: *mut Camera = 0 as *mut Camera;
    if parentCamIdx as libc::c_int == -(1 as libc::c_int) {
        parentCamIdx = (*globalCtx).activeCamera
    }
    csCamIdx = Gameplay_CreateSubCamera(globalCtx);
    if csCamIdx as libc::c_int == -(1 as libc::c_int) {
        osSyncPrintf(b"\x1b[41;37monepoint demo: error: too many cameras ... give up! type=%d\n\x1b[m\x00"
                         as *const u8 as *const libc::c_char,
                     csId as libc::c_int);
        return -(1 as libc::c_int) as s16
    }
    // Inserts the cutscene camera into the cutscene queue in front of parentCam
    temp2 = (*(*globalCtx).cameraPtrs[parentCamIdx as usize]).childCamIdx;
    temp1 = 7 as libc::c_int as s16;
    if temp2 as libc::c_int >= 1 as libc::c_int {
        OnePointCutscene_SetAsChild(globalCtx, temp2, csCamIdx);
        temp1 = 1 as libc::c_int as s16
    } else { Interface_ChangeAlpha(2 as libc::c_int as u16_0); }
    OnePointCutscene_SetAsChild(globalCtx, csCamIdx, parentCamIdx);
    csCam = (*globalCtx).cameraPtrs[csCamIdx as usize];
    (*csCam).timer = timer;
    (*csCam).target = actor;
    (*csCam).at = (*globalCtx).view.lookAt;
    (*csCam).eye = (*globalCtx).view.eye;
    (*csCam).fov = (*globalCtx).view.fovy;
    (*csCam).csId = csId;
    if parentCamIdx as libc::c_int == 0 as libc::c_int {
        Gameplay_ChangeCameraStatus(globalCtx, parentCamIdx,
                                    3 as libc::c_int as s16);
    } else {
        Gameplay_ChangeCameraStatus(globalCtx, parentCamIdx,
                                    1 as libc::c_int as s16);
    }
    OnePointCutscene_SetInfo(globalCtx, csCamIdx, csId, actor, timer);
    Gameplay_ChangeCameraStatus(globalCtx, csCamIdx, temp1);
    // Removes all lower priority cutscenes in front of this cutscene from the queue.
    temp2 = csCamIdx;
    temp1 = (*(*globalCtx).cameraPtrs[csCamIdx as usize]).childCamIdx;
    while temp1 as libc::c_int >= 1 as libc::c_int {
        let mut nextCsId: s16 =
            (*(*globalCtx).cameraPtrs[temp1 as usize]).csId;
        let mut thisCsId: s16 =
            (*(*globalCtx).cameraPtrs[csCamIdx as usize]).csId;
        if (nextCsId as libc::c_int / 100 as libc::c_int) <
               thisCsId as libc::c_int / 100 as libc::c_int {
            osSyncPrintf(b"\x1b[43;30monepointdemo camera[%d]: killed \'coz low priority (%d < %d)\n\x1b[m\x00"
                             as *const u8 as *const libc::c_char,
                         temp1 as libc::c_int, nextCsId as libc::c_int,
                         thisCsId as libc::c_int);
            if (*(*globalCtx).cameraPtrs[temp1 as usize]).csId as libc::c_int
                   != 5010 as libc::c_int {
                temp1 =
                    OnePointCutscene_RemoveCamera(globalCtx, temp1) as s16;
                if temp1 as libc::c_int != -(1 as libc::c_int) {
                    Gameplay_ChangeCameraStatus(globalCtx, temp1,
                                                7 as libc::c_int as s16);
                }
            } else {
                temp2 = temp1;
                OnePointCutscene_EndCutscene(globalCtx, temp1);
            }
        } else { temp2 = temp1 }
        temp1 = (*(*globalCtx).cameraPtrs[temp2 as usize]).childCamIdx
    }
    return csCamIdx;
}
/* *
 *  Ends the cutscene in camIdx by setting its timer to 0. For attention cutscenes, it is set to 5 instead.
 */
#[no_mangle]
pub unsafe extern "C" fn OnePointCutscene_EndCutscene(mut globalCtx:
                                                          *mut GlobalContext,
                                                      mut camIdx: s16)
 -> s16 {
    if camIdx as libc::c_int == -(1 as libc::c_int) {
        camIdx = (*globalCtx).activeCamera
    }
    if !(*globalCtx).cameraPtrs[camIdx as usize].is_null() {
        osSyncPrintf(b"onepointdemo camera[%d]: delete timer=%d next=%d\n\x00"
                         as *const u8 as *const libc::c_char,
                     camIdx as libc::c_int,
                     (*(*globalCtx).cameraPtrs[camIdx as usize]).timer as
                         libc::c_int,
                     (*(*globalCtx).cameraPtrs[camIdx as usize]).parentCamIdx
                         as libc::c_int);
        if (*(*globalCtx).cameraPtrs[camIdx as usize]).csId as libc::c_int ==
               5010 as libc::c_int {
            (*(*globalCtx).cameraPtrs[camIdx as usize]).timer =
                5 as libc::c_int as s16
        } else {
            (*(*globalCtx).cameraPtrs[camIdx as usize]).timer =
                0 as libc::c_int as s16
        }
    }
    return camIdx;
}
/* *
 *  Adds an attention cutscene to the cutscene queue.
 */
#[no_mangle]
pub unsafe extern "C" fn OnePointCutscene_Attention(mut globalCtx:
                                                        *mut GlobalContext,
                                                    mut actor: *mut Actor)
 -> s32 {
    let mut parentCam: *mut Camera = 0 as *mut Camera;
    let mut temp1: s32 = 0;
    let mut temp2: s32 = 0;
    let mut timer: s32 = 0;
    if sDisableAttention != 0 {
        osSyncPrintf(b"\x1b[43;30mactor attention demo camera: canceled by other camera\n\x1b[m\x00"
                         as *const u8 as *const libc::c_char);
        return -(1 as libc::c_int)
    }
    sUnused = -(1 as libc::c_int) as s16;
    parentCam = (*globalCtx).cameraPtrs[0 as libc::c_int as usize];
    if (*parentCam).mode as libc::c_int ==
           CAM_MODE_FOLLOWBOOMERANG as libc::c_int {
        osSyncPrintf(b"\x1b[43;30mactor attention demo camera: change mode BOOKEEPON -> NORMAL\n\x1b[m\x00"
                         as *const u8 as *const libc::c_char);
        Camera_ChangeMode(parentCam, CAM_MODE_NORMAL as libc::c_int as s16);
    }
    // Finds the camera of the first actor attention demo with a lower category actor, or the first non-attention demo
    // after at least one attention demo.
    temp2 = -(1 as libc::c_int);
    while (*parentCam).childCamIdx as libc::c_int != 0 as libc::c_int {
        parentCam =
            (*globalCtx).cameraPtrs[(*parentCam).childCamIdx as usize];
        if parentCam.is_null() { break ; }
        if (*parentCam).setting as libc::c_int !=
               CAM_SET_CS_ATTENTION as libc::c_int {
            if !(temp2 == -(1 as libc::c_int)) { break ; }
        } else {
            temp1 = (*(*parentCam).target).category as s32;
            if (*actor).category as libc::c_int > temp1 { break ; }
            temp2 = temp1
        }
    }
    // Actorcat is only undefined if the actor is in a higher category than all other attention cutscenes. In this case,
    // it goes in the first position of the list. Otherwise, it goes in the index found in the loop.
    temp1 =
        if temp2 == -(1 as libc::c_int) {
            0 as libc::c_int
        } else { (*parentCam).thisIdx as libc::c_int };
    match (*actor).category as libc::c_int {
        0 | 1 | 2 | 6 | 10 => { timer = 30 as libc::c_int }
        4 | 7 | 11 => { timer = 100 as libc::c_int }
        3 | 5 | 8 | 9 | _ => {
            osSyncPrintf(b"\x1b[43;30mactor attention demo camera: %d: unkown part of actor %d\n\x1b[m\x00"
                             as *const u8 as *const libc::c_char,
                         (*globalCtx).state.frames,
                         (*actor).category as libc::c_int);
            timer = 30 as libc::c_int
        }
    }
    osSyncPrintf(b"\x1b[36m%06u:\x1b[m actor attention demo camera: request %d \x00"
                     as *const u8 as *const libc::c_char,
                 (*globalCtx).state.frames, (*actor).category as libc::c_int);
    // If the previous attention cutscene has an actor in the same category, skip this actor.
    if (*actor).category as libc::c_int == temp2 {
        osSyncPrintf(b"\xe2\x86\x92 \x1b[35m\xc3\x97\x1b[m (%d)\n\x00" as
                         *const u8 as *const libc::c_char,
                     (*actor).id as libc::c_int);
        return -(1 as libc::c_int)
    }
    osSyncPrintf(b"\xe2\x86\x92 \x1b[34m\xe2\x97\x8b\x1b[m (%d)\n\x00" as
                     *const u8 as *const libc::c_char,
                 (*actor).id as libc::c_int);
    temp2 =
        OnePointCutscene_Init(globalCtx, 5010 as libc::c_int as s16,
                              timer as s16, actor, temp1 as s16) as s32;
    if temp2 == -(1 as libc::c_int) {
        osSyncPrintf(b"\x1b[41;37mactor attention demo: give up! \n\x1b[m\x00"
                         as *const u8 as *const libc::c_char,
                     (*actor).id as libc::c_int);
        return -(1 as libc::c_int)
    } else {
        let mut data: *mut s32 =
            &mut (**(*globalCtx).cameraPtrs.as_mut_ptr().offset(temp2 as
                                                                    isize)).data1
                as *mut *mut libc::c_void as *mut s32;
        *data = 0x4802 as libc::c_int;
        return temp2
    };
}
/* *
 *  Adds an attention cutscene to the cutscene queue with the specified sound effect
 */
#[no_mangle]
pub unsafe extern "C" fn OnePointCutscene_AttentionSetSfx(mut globalCtx:
                                                              *mut GlobalContext,
                                                          mut actor:
                                                              *mut Actor,
                                                          mut sfxId: s32)
 -> s32 {
    let mut csCamIdx: s32 = OnePointCutscene_Attention(globalCtx, actor);
    if csCamIdx != -(1 as libc::c_int) {
        let mut data: *mut s32 =
            &mut (**(*globalCtx).cameraPtrs.as_mut_ptr().offset(csCamIdx as
                                                                    isize)).data1
                as *mut *mut libc::c_void as *mut s32;
        *data = sfxId
    }
    return csCamIdx;
}
// unused
#[no_mangle]
pub unsafe extern "C" fn OnePointCutscene_EnableAttention() {
    sDisableAttention = 0 as libc::c_int as s16;
}
// unused
#[no_mangle]
pub unsafe extern "C" fn OnePointCutscene_DisableAttention() {
    sDisableAttention = 1 as libc::c_int as s16;
}
#[no_mangle]
pub unsafe extern "C" fn OnePointCutscene_CheckForCategory(mut globalCtx:
                                                               *mut GlobalContext,
                                                           mut category: s32)
 -> s32 {
    let mut parentCam: *mut Camera =
        (*globalCtx).cameraPtrs[0 as libc::c_int as usize];
    while (*parentCam).childCamIdx as libc::c_int != 0 as libc::c_int {
        parentCam =
            (*globalCtx).cameraPtrs[(*parentCam).childCamIdx as usize];
        if parentCam.is_null() ||
               (*parentCam).setting as libc::c_int !=
                   CAM_SET_CS_ATTENTION as libc::c_int {
            break ;
        }
        if category == (*(*parentCam).target).category as libc::c_int {
            return 1 as libc::c_int
        }
    }
    return 0 as libc::c_int;
}
// unused, also empty.
#[no_mangle]
pub unsafe extern "C" fn OnePointCutscene_Noop(mut globalCtx:
                                                   *mut GlobalContext,
                                               mut arg1: s32) {
}
