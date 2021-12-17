#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case,
         non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(const_raw_ptr_to_usize_cast, register_tool)]
extern "C" {
    #[no_mangle]
    fn fabsf(f: f32_0) -> f32_0;
    #[no_mangle]
    fn sqrtf(f: f32_0) -> f32_0;
    #[no_mangle]
    fn __assert(exp: *const libc::c_char, file: *const libc::c_char,
                line: s32);
    #[no_mangle]
    fn osSyncPrintf(fmt: *const libc::c_char, _: ...);
    #[no_mangle]
    fn Effect_Add(globalCtx: *mut GlobalContext, pIndex: *mut s32,
                  type_0: s32, arg3: u8_0, arg4: u8_0,
                  initParams: *mut libc::c_void);
    #[no_mangle]
    fn EffectSsSibuki_SpawnBurst(globalCtx: *mut GlobalContext,
                                 pos: *mut Vec3f);
    #[no_mangle]
    fn EffectSsHitMark_SpawnFixedScale(globalCtx: *mut GlobalContext,
                                       type_0: s32, pos: *mut Vec3f);
    #[no_mangle]
    fn BgCheck_DrawDynaCollision(_: *mut GlobalContext,
                                 _: *mut CollisionContext);
    #[no_mangle]
    fn BgCheck_DrawStaticCollision(_: *mut GlobalContext,
                                   _: *mut CollisionContext);
    #[no_mangle]
    fn DamageTable_Get(index: s32) -> *mut DamageTable;
    #[no_mangle]
    fn Math_Vec3f_Copy(dest: *mut Vec3f, src: *mut Vec3f);
    #[no_mangle]
    fn Math_Vec3s_ToVec3f(dest: *mut Vec3f, src: *mut Vec3s);
    #[no_mangle]
    fn ZeldaArena_MallocDebug(size: u32_0, file: *const libc::c_char,
                              line: s32) -> *mut libc::c_void;
    #[no_mangle]
    fn ZeldaArena_FreeDebug(ptr: *mut libc::c_void, file: *const libc::c_char,
                            line: s32);
    #[no_mangle]
    fn Graph_CloseDisps(dispRefs: *mut *mut Gfx, gfxCtx: *mut GraphicsContext,
                        file: *const libc::c_char, line: s32);
    #[no_mangle]
    fn Math3D_DefPlane(va: *mut Vec3f, vb: *mut Vec3f, vc: *mut Vec3f,
                       nx: *mut f32_0, ny: *mut f32_0, nz: *mut f32_0,
                       originDist: *mut f32_0);
    #[no_mangle]
    fn Graph_Alloc(gfxCtx: *mut GraphicsContext, size: size_t)
     -> *mut libc::c_void;
    #[no_mangle]
    static mut gMtxClear: Mtx;
    #[no_mangle]
    fn Graph_OpenDisps(dispRefs: *mut *mut Gfx, gfxCtx: *mut GraphicsContext,
                       file: *const libc::c_char, line: s32);
    #[no_mangle]
    static mut gGameInfo: *mut GameInfo;
    #[no_mangle]
    fn Math3D_DrawCylinder(globalCtx: *mut GlobalContext,
                           cyl: *mut Cylinder16);
    #[no_mangle]
    fn Math3D_DrawSphere(globalCtx: *mut GlobalContext, sph: *mut Sphere16);
    #[no_mangle]
    fn FrameAdvance_IsEnabled(globalCtx: *mut GlobalContext) -> s32;
    #[no_mangle]
    static mut D_801333E8: s8;
    #[no_mangle]
    static mut D_801333E0: f32_0;
    #[no_mangle]
    fn Audio_PlaySoundGeneral(sfxId: u16_0, pos: *mut Vec3f, token: u8_0,
                              freqScale: *mut f32_0, a4: *mut f32_0,
                              reverbAdd: *mut s8);
    #[no_mangle]
    static mut D_801333D4: Vec3f;
    #[no_mangle]
    fn Math3D_Vec3fDistSq(a: *mut Vec3f, b: *mut Vec3f) -> f32_0;
    #[no_mangle]
    fn Math3D_TriVsTriIntersect(ta: *mut TriNorm, tb: *mut TriNorm,
                                intersect: *mut Vec3f) -> s32;
    #[no_mangle]
    fn Math3D_TriNorm(tri: *mut TriNorm, va: *mut Vec3f, vb: *mut Vec3f,
                      vc: *mut Vec3f);
    #[no_mangle]
    fn Math3D_CylTriVsIntersect(cyl: *mut Cylinder16, tri: *mut TriNorm,
                                intersect: *mut Vec3f) -> s32;
    #[no_mangle]
    fn Math3D_TriVsSphIntersect(sphere: *mut Sphere16, tri: *mut TriNorm,
                                intersectPoint: *mut Vec3f) -> s32;
    #[no_mangle]
    fn Math3D_CylOutsideCylDist(ca: *mut Cylinder16, cb: *mut Cylinder16,
                                deadSpace: *mut f32_0, xzDist: *mut f32_0)
     -> s32;
    #[no_mangle]
    fn Math3D_SphVsCylOverlapCenterDist(sph: *mut Sphere16,
                                        cyl: *mut Cylinder16,
                                        overlapSize: *mut f32_0,
                                        centerDist: *mut f32_0) -> s32;
    #[no_mangle]
    fn Math3D_SphVsSphOverlapCenter(sphereA: *mut Sphere16,
                                    sphereB: *mut Sphere16,
                                    overlapSize: *mut f32_0,
                                    centerDist: *mut f32_0) -> s32;
    #[no_mangle]
    fn Math3D_CylOutsideCyl(ca: *mut Cylinder16, cb: *mut Cylinder16,
                            deadSpace: *mut f32_0) -> s32;
    #[no_mangle]
    fn Math3D_SphVsCylOverlapDist(sph: *mut Sphere16, cyl: *mut Cylinder16,
                                  overlapSize: *mut f32_0) -> s32;
    #[no_mangle]
    fn Math3D_SphVsSphOverlap(sphereA: *mut Sphere16, sphereB: *mut Sphere16,
                              overlapSize: *mut f32_0) -> s32;
    #[no_mangle]
    fn Math3D_CylVsLineSeg(cyl: *mut Cylinder16, linePointA: *mut Vec3f,
                           linePointB: *mut Vec3f, intersectA: *mut Vec3f,
                           intersectB: *mut Vec3f) -> s32;
    #[no_mangle]
    fn Math3D_LineVsSph(sphere: *mut Sphere16, line: *mut Linef) -> s32;
    #[no_mangle]
    fn Matrix_MultVec3f(src: *mut Vec3f, dest: *mut Vec3f);
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
pub struct Plane {
    pub normal: Vec3f,
    pub originDist: f32_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct TriNorm {
    pub vtx: [Vec3f; 3],
    pub plane: Plane,
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
pub struct ColliderInit {
    pub colType: u8_0,
    pub atFlags: u8_0,
    pub acFlags: u8_0,
    pub ocFlags1: u8_0,
    pub ocFlags2: u8_0,
    pub shape: u8_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ColliderInitType1 {
    pub colType: u8_0,
    pub atFlags: u8_0,
    pub acFlags: u8_0,
    pub ocFlags1: u8_0,
    pub shape: u8_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ColliderInitToActor {
    pub actor: *mut Actor,
    pub atFlags: u8_0,
    pub acFlags: u8_0,
    pub ocFlags1: u8_0,
    pub shape: u8_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ColliderBumpInit {
    pub dmgFlags: u32_0,
    pub effect: u8_0,
    pub defense: u8_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ColliderInfoInit {
    pub elemType: u8_0,
    pub toucher: ColliderTouch,
    pub bumper: ColliderBumpInit,
    pub toucherFlags: u8_0,
    pub bumperFlags: u8_0,
    pub ocElemFlags: u8_0,
}
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
pub struct ColliderJntSphElementDimInit {
    pub limb: u8_0,
    pub modelSphere: Sphere16,
    pub scale: s16,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ColliderJntSphElement {
    pub info: ColliderInfo,
    pub dim: ColliderJntSphElementDim,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ColliderJntSphElementInit {
    pub info: ColliderInfoInit,
    pub dim: ColliderJntSphElementDimInit,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ColliderJntSph {
    pub base: Collider,
    pub count: s32,
    pub elements: *mut ColliderJntSphElement,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ColliderJntSphInit {
    pub base: ColliderInit,
    pub count: s32,
    pub elements: *mut ColliderJntSphElementInit,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ColliderJntSphInitType1 {
    pub base: ColliderInitType1,
    pub count: s32,
    pub elements: *mut ColliderJntSphElementInit,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ColliderJntSphInitToActor {
    pub base: ColliderInitToActor,
    pub count: s32,
    pub elements: *mut ColliderJntSphElementInit,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ColliderCylinderInit {
    pub base: ColliderInit,
    pub info: ColliderInfoInit,
    pub dim: Cylinder16,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ColliderCylinderInitType1 {
    pub base: ColliderInitType1,
    pub info: ColliderInfoInit,
    pub dim: Cylinder16,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ColliderCylinderInitToActor {
    pub base: ColliderInitToActor,
    pub info: ColliderInfoInit,
    pub dim: Cylinder16,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ColliderTrisElementDimInit {
    pub vtx: [Vec3f; 3],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ColliderTrisElement {
    pub info: ColliderInfo,
    pub dim: TriNorm,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ColliderTrisElementInit {
    pub info: ColliderInfoInit,
    pub dim: ColliderTrisElementDimInit,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ColliderTris {
    pub base: Collider,
    pub count: s32,
    pub elements: *mut ColliderTrisElement,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ColliderTrisInit {
    pub base: ColliderInit,
    pub count: s32,
    pub elements: *mut ColliderTrisElementInit,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ColliderTrisInitType1 {
    pub base: ColliderInitType1,
    pub count: s32,
    pub elements: *mut ColliderTrisElementInit,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ColliderQuadDimInit {
    pub quad: [Vec3f; 4],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ColliderQuadInit {
    pub base: ColliderInit,
    pub info: ColliderInfoInit,
    pub dim: ColliderQuadDimInit,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ColliderQuadInitType1 {
    pub base: ColliderInitType1,
    pub info: ColliderInfoInit,
    pub dim: ColliderQuadDimInit,
}
pub type C2RustUnnamed_14 = libc::c_uint;
pub const COLTYPE_TREE: C2RustUnnamed_14 = 13;
pub const COLTYPE_HARD: C2RustUnnamed_14 = 12;
pub const COLTYPE_WOOD: C2RustUnnamed_14 = 11;
pub const COLTYPE_NONE: C2RustUnnamed_14 = 10;
pub const COLTYPE_METAL: C2RustUnnamed_14 = 9;
pub const COLTYPE_HIT8: C2RustUnnamed_14 = 8;
pub const COLTYPE_HIT7: C2RustUnnamed_14 = 7;
pub const COLTYPE_HIT6: C2RustUnnamed_14 = 6;
pub const COLTYPE_HIT5: C2RustUnnamed_14 = 5;
pub const COLTYPE_HIT4: C2RustUnnamed_14 = 4;
pub const COLTYPE_HIT3: C2RustUnnamed_14 = 3;
pub const COLTYPE_HIT2: C2RustUnnamed_14 = 2;
pub const COLTYPE_HIT1: C2RustUnnamed_14 = 1;
pub const COLTYPE_HIT0: C2RustUnnamed_14 = 0;
pub type C2RustUnnamed_15 = libc::c_uint;
pub const COLSHAPE_INVALID: C2RustUnnamed_15 = 4;
pub const COLSHAPE_QUAD: C2RustUnnamed_15 = 3;
pub const COLSHAPE_TRIS: C2RustUnnamed_15 = 2;
pub const COLSHAPE_CYLINDER: C2RustUnnamed_15 = 1;
pub const COLSHAPE_JNTSPH: C2RustUnnamed_15 = 0;
pub type C2RustUnnamed_16 = libc::c_uint;
pub const ELEMTYPE_UNK7: C2RustUnnamed_16 = 7;
pub const ELEMTYPE_UNK6: C2RustUnnamed_16 = 6;
pub const ELEMTYPE_UNK5: C2RustUnnamed_16 = 5;
pub const ELEMTYPE_UNK4: C2RustUnnamed_16 = 4;
pub const ELEMTYPE_UNK3: C2RustUnnamed_16 = 3;
pub const ELEMTYPE_UNK2: C2RustUnnamed_16 = 2;
pub const ELEMTYPE_UNK1: C2RustUnnamed_16 = 1;
pub const ELEMTYPE_UNK0: C2RustUnnamed_16 = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CollisionCheckInfoInit {
    pub health: u8_0,
    pub cylRadius: s16,
    pub cylHeight: s16,
    pub mass: u8_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CollisionCheckInfoInit2 {
    pub health: u8_0,
    pub cylRadius: s16,
    pub cylHeight: s16,
    pub cylYShift: s16,
    pub mass: u8_0,
}
pub type C2RustUnnamed_17 = libc::c_uint;
pub const ACTORCAT_CHEST: C2RustUnnamed_17 = 11;
pub const ACTORCAT_DOOR: C2RustUnnamed_17 = 10;
pub const ACTORCAT_BOSS: C2RustUnnamed_17 = 9;
pub const ACTORCAT_MISC: C2RustUnnamed_17 = 8;
pub const ACTORCAT_ITEMACTION: C2RustUnnamed_17 = 7;
pub const ACTORCAT_PROP: C2RustUnnamed_17 = 6;
pub const ACTORCAT_ENEMY: C2RustUnnamed_17 = 5;
pub const ACTORCAT_NPC: C2RustUnnamed_17 = 4;
pub const ACTORCAT_EXPLOSIVE: C2RustUnnamed_17 = 3;
pub const ACTORCAT_PLAYER: C2RustUnnamed_17 = 2;
pub const ACTORCAT_BG: C2RustUnnamed_17 = 1;
pub const ACTORCAT_SWITCH: C2RustUnnamed_17 = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct EffectSparkElement {
    pub velocity: Vec3f,
    pub position: Vec3f,
    pub unkVelocity: Vec3s,
    pub unkPosition: Vec3s,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct EffectSparkInit {
    pub position: Vec3s,
    pub numElements: s32,
    pub elements: [EffectSparkElement; 32],
    pub speed: f32_0,
    pub gravity: f32_0,
    pub uDiv: u32_0,
    pub vDiv: u32_0,
    pub colorStart: [Color_RGBA8; 4],
    pub colorEnd: [Color_RGBA8; 4],
    pub timer: s32,
    pub duration: s32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct EffectShieldParticleInit {
    pub numElements: u8_0,
    pub position: Vec3s,
    pub primColorStart: Color_RGBA8,
    pub envColorStart: Color_RGBA8,
    pub primColorMid: Color_RGBA8,
    pub envColorMid: Color_RGBA8,
    pub primColorEnd: Color_RGBA8,
    pub envColorEnd: Color_RGBA8,
    pub deceleration: f32_0,
    pub maxInitialSpeed: f32_0,
    pub lengthCutoff: f32_0,
    pub duration: u8_0,
    pub lightPoint: LightPoint,
    pub lightDecay: s32,
}
pub type C2RustUnnamed_18 = libc::c_uint;
pub const EFFECT_SHIELD_PARTICLE: C2RustUnnamed_18 = 3;
pub const EFFECT_BLURE2: C2RustUnnamed_18 = 2;
pub const EFFECT_BLURE1: C2RustUnnamed_18 = 1;
pub const EFFECT_SPARK: C2RustUnnamed_18 = 0;
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
pub type ColChkResetFunc
    =
    Option<unsafe extern "C" fn(_: *mut GlobalContext, _: *mut Collider)
               -> s32>;
pub type ColChkApplyFunc
    =
    Option<unsafe extern "C" fn(_: *mut GlobalContext,
                                _: *mut CollisionCheckContext,
                                _: *mut Collider) -> ()>;
pub const EFFECT_HITMARK_WHITE: C2RustUnnamed_19 = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct HitInfo {
    pub blood: u8_0,
    pub effect: u8_0,
}
pub const HIT_WOOD: C2RustUnnamed_21 = 4;
pub const BLOOD_NONE: C2RustUnnamed_20 = 0;
pub const HIT_SOLID: C2RustUnnamed_21 = 3;
pub const HIT_NONE: C2RustUnnamed_21 = 5;
pub const HIT_RED: C2RustUnnamed_21 = 2;
pub const BLOOD_BLUE: C2RustUnnamed_20 = 1;
pub const HIT_WHITE: C2RustUnnamed_21 = 0;
pub const BLOOD_RED: C2RustUnnamed_20 = 4;
pub const BLOOD_GREEN: C2RustUnnamed_20 = 2;
pub const BLOOD_WATER: C2RustUnnamed_20 = 3;
pub const HIT_DUST: C2RustUnnamed_21 = 1;
pub const EFFECT_HITMARK_DUST: C2RustUnnamed_19 = 1;
pub const EFFECT_HITMARK_METAL: C2RustUnnamed_19 = 3;
pub type ColChkBloodFunc
    =
    Option<unsafe extern "C" fn(_: *mut GlobalContext, _: *mut Collider,
                                _: *mut Vec3f) -> ()>;
pub type ColChkVsFunc
    =
    Option<unsafe extern "C" fn(_: *mut GlobalContext,
                                _: *mut CollisionCheckContext,
                                _: *mut Collider, _: *mut Collider) -> ()>;
pub const MASSTYPE_NORMAL: C2RustUnnamed_22 = 2;
pub const MASSTYPE_HEAVY: C2RustUnnamed_22 = 1;
pub const MASSTYPE_IMMOVABLE: C2RustUnnamed_22 = 0;
pub type ColChkLineFunc
    =
    Option<unsafe extern "C" fn(_: *mut GlobalContext,
                                _: *mut CollisionCheckContext,
                                _: *mut Collider, _: *mut Vec3f,
                                _: *mut Vec3f) -> s32>;
pub type C2RustUnnamed_19 = libc::c_uint;
pub const EFFECT_HITMARK_RED: C2RustUnnamed_19 = 2;
pub type C2RustUnnamed_20 = libc::c_uint;
pub const BLOOD_RED2: C2RustUnnamed_20 = 5;
pub type C2RustUnnamed_21 = libc::c_uint;
pub type C2RustUnnamed_22 = libc::c_uint;
/* *
 * Draws a red triangle with vertices vA, vB, and vC.
 */
#[no_mangle]
pub unsafe extern "C" fn Collider_DrawRedPoly(mut gfxCtx:
                                                  *mut GraphicsContext,
                                              mut vA: *mut Vec3f,
                                              mut vB: *mut Vec3f,
                                              mut vC: *mut Vec3f) {
    Collider_DrawPoly(gfxCtx, vA, vB, vC, 255 as libc::c_int as u8_0,
                      0 as libc::c_int as u8_0, 0 as libc::c_int as u8_0);
}
/* *
 * Draws the triangle with vertices vA, vB, and vC and with the specified color.
 */
#[no_mangle]
pub unsafe extern "C" fn Collider_DrawPoly(mut gfxCtx: *mut GraphicsContext,
                                           mut vA: *mut Vec3f,
                                           mut vB: *mut Vec3f,
                                           mut vC: *mut Vec3f, mut r: u8_0,
                                           mut g: u8_0, mut b: u8_0) {
    let mut vtxTbl: *mut Vtx = 0 as *mut Vtx;
    let mut vtx: *mut Vtx = 0 as *mut Vtx;
    let mut nx: f32_0 = 0.;
    let mut ny: f32_0 = 0.;
    let mut nz: f32_0 = 0.;
    let mut originDist: f32_0 = 0.;
    let mut __gfxCtx: *mut GraphicsContext = 0 as *mut GraphicsContext;
    let mut dispRefs: [*mut Gfx; 4] = [0 as *mut Gfx; 4];
    __gfxCtx = gfxCtx;
    Graph_OpenDisps(dispRefs.as_mut_ptr(), gfxCtx,
                    b"../z_collision_check.c\x00" as *const u8 as
                        *const libc::c_char, 713 as libc::c_int);
    let fresh0 = (*__gfxCtx).polyOpa.p;
    (*__gfxCtx).polyOpa.p = (*__gfxCtx).polyOpa.p.offset(1);
    let mut _g: *mut Gfx = fresh0;
    (*_g).words.w0 =
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
    (*_g).words.w1 = &mut gMtxClear as *mut Mtx as libc::c_uint;
    let fresh1 = (*__gfxCtx).polyOpa.p;
    (*__gfxCtx).polyOpa.p = (*__gfxCtx).polyOpa.p.offset(1);
    let mut _g_0: *mut Gfx = fresh1;
    (*_g_0).words.w0 =
        (0xfa as libc::c_int as u32_0 &
             (((0x1 as libc::c_int) << 8 as libc::c_int) - 1 as libc::c_int)
                 as libc::c_uint) << 24 as libc::c_int |
            (0 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 8 as libc::c_int |
            (0xff as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 0 as libc::c_int;
    (*_g_0).words.w1 =
        (r as u32_0 &
             (((0x1 as libc::c_int) << 8 as libc::c_int) - 1 as libc::c_int)
                 as libc::c_uint) << 24 as libc::c_int |
            (g as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 16 as libc::c_int
            |
            (b as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 8 as libc::c_int |
            (50 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 0 as libc::c_int;
    let fresh2 = (*__gfxCtx).polyOpa.p;
    (*__gfxCtx).polyOpa.p = (*__gfxCtx).polyOpa.p.offset(1);
    let mut _g_1: *mut Gfx = fresh2;
    (*_g_1).words.w0 =
        (0xe7 as libc::c_int as u32_0 &
             (((0x1 as libc::c_int) << 8 as libc::c_int) - 1 as libc::c_int)
                 as libc::c_uint) << 24 as libc::c_int;
    (*_g_1).words.w1 = 0 as libc::c_int as libc::c_uint;
    let fresh3 = (*__gfxCtx).polyOpa.p;
    (*__gfxCtx).polyOpa.p = (*__gfxCtx).polyOpa.p.offset(1);
    let mut _g_2: *mut Gfx = fresh3;
    (*_g_2).words.w0 =
        (0xe2 as libc::c_int as u32_0 &
             (((0x1 as libc::c_int) << 8 as libc::c_int) - 1 as libc::c_int)
                 as libc::c_uint) << 24 as libc::c_int |
            ((32 as libc::c_int - 3 as libc::c_int - 29 as libc::c_int) as
                 u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 8 as libc::c_int |
            ((29 as libc::c_int - 1 as libc::c_int) as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 0 as libc::c_int;
    (*_g_2).words.w1 =
        ((3 as libc::c_int) << 30 as libc::c_int |
             (2 as libc::c_int) << 26 as libc::c_int |
             (0 as libc::c_int) << 22 as libc::c_int |
             (0 as libc::c_int) << 18 as libc::c_int |
             (0x8 as libc::c_int | 0x10 as libc::c_int | 0x20 as libc::c_int |
                  0x40 as libc::c_int | 0 as libc::c_int | 0 as libc::c_int |
                  0x2000 as libc::c_int |
                  (0 as libc::c_int) << 28 as libc::c_int |
                  (0 as libc::c_int) << 24 as libc::c_int |
                  (1 as libc::c_int) << 20 as libc::c_int |
                  (1 as libc::c_int) << 16 as libc::c_int)) as libc::c_uint;
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
            (0 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 7 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 1 as libc::c_int;
    (*_g_3).words.w1 =
        (0 as libc::c_int as u32_0 &
             (((0x1 as libc::c_int) << 16 as libc::c_int) - 1 as libc::c_int)
                 as libc::c_uint) << 16 as libc::c_int |
            (0 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 16 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 0 as libc::c_int;
    let fresh5 = (*__gfxCtx).polyOpa.p;
    (*__gfxCtx).polyOpa.p = (*__gfxCtx).polyOpa.p.offset(1);
    let mut _g_4: *mut Gfx = fresh5;
    (*_g_4).words.w0 =
        (0xe7 as libc::c_int as u32_0 &
             (((0x1 as libc::c_int) << 8 as libc::c_int) - 1 as libc::c_int)
                 as libc::c_uint) << 24 as libc::c_int;
    (*_g_4).words.w1 = 0 as libc::c_int as libc::c_uint;
    let fresh6 = (*__gfxCtx).polyOpa.p;
    (*__gfxCtx).polyOpa.p = (*__gfxCtx).polyOpa.p.offset(1);
    let mut _g_5: *mut Gfx = fresh6;
    (*_g_5).words.w0 =
        (0xfc as libc::c_int as u32_0 &
             (((0x1 as libc::c_int) << 8 as libc::c_int) - 1 as libc::c_int)
                 as libc::c_uint) << 24 as libc::c_int |
            (((4 as libc::c_int as u32_0 &
                   (((0x1 as libc::c_int) << 4 as libc::c_int) -
                        1 as libc::c_int) as libc::c_uint) <<
                  20 as libc::c_int |
                  (3 as libc::c_int as u32_0 &
                       (((0x1 as libc::c_int) << 5 as libc::c_int) -
                            1 as libc::c_int) as libc::c_uint) <<
                      15 as libc::c_int |
                  (4 as libc::c_int as u32_0 &
                       (((0x1 as libc::c_int) << 3 as libc::c_int) -
                            1 as libc::c_int) as libc::c_uint) <<
                      12 as libc::c_int |
                  (3 as libc::c_int as u32_0 &
                       (((0x1 as libc::c_int) << 3 as libc::c_int) -
                            1 as libc::c_int) as libc::c_uint) <<
                      9 as libc::c_int |
                  ((31 as libc::c_int as u32_0 &
                        (((0x1 as libc::c_int) << 4 as libc::c_int) -
                             1 as libc::c_int) as libc::c_uint) <<
                       5 as libc::c_int |
                       (31 as libc::c_int as u32_0 &
                            (((0x1 as libc::c_int) << 5 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           0 as libc::c_int)) &
                 (((0x1 as libc::c_int) << 24 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 0 as libc::c_int;
    (*_g_5).words.w1 =
        (31 as libc::c_int as u32_0 &
             (((0x1 as libc::c_int) << 4 as libc::c_int) - 1 as libc::c_int)
                 as libc::c_uint) << 28 as libc::c_int |
            (31 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 3 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 15 as libc::c_int
            |
            (7 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 3 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 12 as libc::c_int
            |
            (7 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 3 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 9 as libc::c_int |
            ((31 as libc::c_int as u32_0 &
                  (((0x1 as libc::c_int) << 4 as libc::c_int) -
                       1 as libc::c_int) as libc::c_uint) << 24 as libc::c_int
                 |
                 (7 as libc::c_int as u32_0 &
                      (((0x1 as libc::c_int) << 3 as libc::c_int) -
                           1 as libc::c_int) as libc::c_uint) <<
                     21 as libc::c_int |
                 (7 as libc::c_int as u32_0 &
                      (((0x1 as libc::c_int) << 3 as libc::c_int) -
                           1 as libc::c_int) as libc::c_uint) <<
                     18 as libc::c_int |
                 (0 as libc::c_int as u32_0 &
                      (((0x1 as libc::c_int) << 3 as libc::c_int) -
                           1 as libc::c_int) as libc::c_uint) <<
                     6 as libc::c_int |
                 (7 as libc::c_int as u32_0 &
                      (((0x1 as libc::c_int) << 3 as libc::c_int) -
                           1 as libc::c_int) as libc::c_uint) <<
                     3 as libc::c_int |
                 (0 as libc::c_int as u32_0 &
                      (((0x1 as libc::c_int) << 3 as libc::c_int) -
                           1 as libc::c_int) as libc::c_uint) <<
                     0 as libc::c_int);
    let fresh7 = (*__gfxCtx).polyOpa.p;
    (*__gfxCtx).polyOpa.p = (*__gfxCtx).polyOpa.p.offset(1);
    let mut _g_6: *mut Gfx = fresh7;
    (*_g_6).words.w0 =
        (0xd9 as libc::c_int as u32_0 &
             (((0x1 as libc::c_int) << 8 as libc::c_int) - 1 as libc::c_int)
                 as libc::c_uint) << 24 as libc::c_int |
            (!(0x600 as libc::c_int as u32_0) &
                 (((0x1 as libc::c_int) << 24 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 0 as libc::c_int;
    (*_g_6).words.w1 = 0 as libc::c_int as u32_0;
    let fresh8 = (*__gfxCtx).polyOpa.p;
    (*__gfxCtx).polyOpa.p = (*__gfxCtx).polyOpa.p.offset(1);
    let mut _g_7: *mut Gfx = fresh8;
    (*_g_7).words.w0 =
        (0xd9 as libc::c_int as u32_0 &
             (((0x1 as libc::c_int) << 8 as libc::c_int) - 1 as libc::c_int)
                 as libc::c_uint) << 24 as libc::c_int |
            (!(0 as libc::c_int as u32_0) &
                 (((0x1 as libc::c_int) << 24 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 0 as libc::c_int;
    (*_g_7).words.w1 = 0x20000 as libc::c_int as u32_0;
    let fresh9 = (*__gfxCtx).polyOpa.p;
    (*__gfxCtx).polyOpa.p = (*__gfxCtx).polyOpa.p.offset(1);
    let mut _g_8: *mut Gfx = fresh9;
    (*_g_8).words.w0 =
        (0xe7 as libc::c_int as u32_0 &
             (((0x1 as libc::c_int) << 8 as libc::c_int) - 1 as libc::c_int)
                 as libc::c_uint) << 24 as libc::c_int;
    (*_g_8).words.w1 = 0 as libc::c_int as libc::c_uint;
    vtxTbl =
        Graph_Alloc(gfxCtx,
                    (3 as libc::c_int as
                         libc::c_uint).wrapping_mul(::std::mem::size_of::<Vtx>()
                                                        as libc::c_ulong) as
                        size_t) as *mut Vtx;
    if !vtxTbl.is_null() {
    } else {
        __assert(b"vtx_tbl != NULL\x00" as *const u8 as *const libc::c_char,
                 b"../z_collision_check.c\x00" as *const u8 as
                     *const libc::c_char, 726 as libc::c_int);
    };
    (*vtxTbl.offset(0 as libc::c_int as
                        isize)).n.ob[0 as libc::c_int as usize] =
        (*vA).x as libc::c_short;
    (*vtxTbl.offset(0 as libc::c_int as
                        isize)).n.ob[1 as libc::c_int as usize] =
        (*vA).y as libc::c_short;
    (*vtxTbl.offset(0 as libc::c_int as
                        isize)).n.ob[2 as libc::c_int as usize] =
        (*vA).z as libc::c_short;
    (*vtxTbl.offset(1 as libc::c_int as
                        isize)).n.ob[0 as libc::c_int as usize] =
        (*vB).x as libc::c_short;
    (*vtxTbl.offset(1 as libc::c_int as
                        isize)).n.ob[1 as libc::c_int as usize] =
        (*vB).y as libc::c_short;
    (*vtxTbl.offset(1 as libc::c_int as
                        isize)).n.ob[2 as libc::c_int as usize] =
        (*vB).z as libc::c_short;
    (*vtxTbl.offset(2 as libc::c_int as
                        isize)).n.ob[0 as libc::c_int as usize] =
        (*vC).x as libc::c_short;
    (*vtxTbl.offset(2 as libc::c_int as
                        isize)).n.ob[1 as libc::c_int as usize] =
        (*vC).y as libc::c_short;
    (*vtxTbl.offset(2 as libc::c_int as
                        isize)).n.ob[2 as libc::c_int as usize] =
        (*vC).z as libc::c_short;
    Math3D_DefPlane(vA, vB, vC, &mut nx, &mut ny, &mut nz, &mut originDist);
    vtx = vtxTbl;
    while vtx < vtxTbl.offset(3 as libc::c_int as isize) {
        (*vtx).n.flag = 0 as libc::c_int as libc::c_ushort;
        (*vtx).n.tc[0 as libc::c_int as usize] =
            0 as libc::c_int as libc::c_short;
        (*vtx).n.tc[1 as libc::c_int as usize] =
            0 as libc::c_int as libc::c_short;
        (*vtx).n.n[0 as libc::c_int as usize] =
            (nx as s32 as u8_0 as libc::c_int & 0xff as libc::c_int) as
                libc::c_schar;
        (*vtx).n.n[1 as libc::c_int as usize] =
            (ny as s32 as u8_0 as libc::c_int & 0xff as libc::c_int) as
                libc::c_schar;
        (*vtx).n.n[2 as libc::c_int as usize] =
            (nz as s32 as u8_0 as libc::c_int & 0xff as libc::c_int) as
                libc::c_schar;
        (*vtx).n.a = 255 as libc::c_int as libc::c_uchar;
        vtx = vtx.offset(1)
    }
    let fresh10 = (*__gfxCtx).polyOpa.p;
    (*__gfxCtx).polyOpa.p = (*__gfxCtx).polyOpa.p.offset(1);
    let mut _g_9: *mut Gfx = fresh10;
    (*_g_9).words.w0 =
        (0x1 as libc::c_int as u32_0 &
             (((0x1 as libc::c_int) << 8 as libc::c_int) - 1 as libc::c_int)
                 as libc::c_uint) << 24 as libc::c_int |
            (3 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 12 as libc::c_int
            |
            ((0 as libc::c_int + 3 as libc::c_int) as u32_0 &
                 (((0x1 as libc::c_int) << 7 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 1 as libc::c_int;
    (*_g_9).words.w1 = vtxTbl as libc::c_uint;
    let fresh11 = (*__gfxCtx).polyOpa.p;
    (*__gfxCtx).polyOpa.p = (*__gfxCtx).polyOpa.p.offset(1);
    let mut _g_10: *mut Gfx = fresh11;
    (*_g_10).words.w0 =
        (0x5 as libc::c_int as u32_0 &
             (((0x1 as libc::c_int) << 8 as libc::c_int) - 1 as libc::c_int)
                 as libc::c_uint) << 24 as libc::c_int |
            (if 0 as libc::c_int == 0 as libc::c_int {
                 (((0 as libc::c_int * 2 as libc::c_int) as u32_0 &
                       (((0x1 as libc::c_int) << 8 as libc::c_int) -
                            1 as libc::c_int) as libc::c_uint) <<
                      16 as libc::c_int |
                      ((1 as libc::c_int * 2 as libc::c_int) as u32_0 &
                           (((0x1 as libc::c_int) << 8 as libc::c_int) -
                                1 as libc::c_int) as libc::c_uint) <<
                          8 as libc::c_int) |
                     ((2 as libc::c_int * 2 as libc::c_int) as u32_0 &
                          (((0x1 as libc::c_int) << 8 as libc::c_int) -
                               1 as libc::c_int) as libc::c_uint) <<
                         0 as libc::c_int
             } else {
                 (if 0 as libc::c_int == 1 as libc::c_int {
                      (((1 as libc::c_int * 2 as libc::c_int) as u32_0 &
                            (((0x1 as libc::c_int) << 8 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           16 as libc::c_int |
                           ((2 as libc::c_int * 2 as libc::c_int) as u32_0 &
                                (((0x1 as libc::c_int) << 8 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               8 as libc::c_int) |
                          ((0 as libc::c_int * 2 as libc::c_int) as u32_0 &
                               (((0x1 as libc::c_int) << 8 as libc::c_int) -
                                    1 as libc::c_int) as libc::c_uint) <<
                              0 as libc::c_int
                  } else {
                      (((2 as libc::c_int * 2 as libc::c_int) as u32_0 &
                            (((0x1 as libc::c_int) << 8 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           16 as libc::c_int |
                           ((0 as libc::c_int * 2 as libc::c_int) as u32_0 &
                                (((0x1 as libc::c_int) << 8 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               8 as libc::c_int) |
                          ((1 as libc::c_int * 2 as libc::c_int) as u32_0 &
                               (((0x1 as libc::c_int) << 8 as libc::c_int) -
                                    1 as libc::c_int) as libc::c_uint) <<
                              0 as libc::c_int
                  })
             });
    (*_g_10).words.w1 = 0 as libc::c_int as libc::c_uint;
    Graph_CloseDisps(dispRefs.as_mut_ptr(), gfxCtx,
                     b"../z_collision_check.c\x00" as *const u8 as
                         *const libc::c_char, 757 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn Collider_InitBase(mut globalCtx: *mut GlobalContext,
                                           mut collider: *mut Collider)
 -> s32 {
    static mut init: Collider =
        {
            let mut init =
                Collider{actor: 0 as *const Actor as *mut Actor,
                         at: 0 as *const Actor as *mut Actor,
                         ac: 0 as *const Actor as *mut Actor,
                         oc: 0 as *const Actor as *mut Actor,
                         atFlags: 0 as libc::c_int as u8_0,
                         acFlags: 0 as libc::c_int as u8_0,
                         ocFlags1: 0 as libc::c_int as u8_0,
                         ocFlags2: 0 as libc::c_int as u8_0,
                         colType: COLTYPE_HIT3 as libc::c_int as u8_0,
                         shape: COLSHAPE_INVALID as libc::c_int as u8_0,};
            init
        };
    *collider = init;
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn Collider_DestroyBase(mut globalCtx:
                                                  *mut GlobalContext,
                                              mut collider: *mut Collider)
 -> s32 {
    return 1 as libc::c_int;
}
/* *
 * Uses default OC2_TYPE_1 and COLTYPE_HIT0
 */
#[no_mangle]
pub unsafe extern "C" fn Collider_SetBaseToActor(mut globalCtx:
                                                     *mut GlobalContext,
                                                 mut collider: *mut Collider,
                                                 mut src:
                                                     *mut ColliderInitToActor)
 -> s32 {
    (*collider).actor = (*src).actor;
    (*collider).atFlags = (*src).atFlags;
    (*collider).acFlags = (*src).acFlags;
    (*collider).ocFlags1 = (*src).ocFlags1;
    (*collider).ocFlags2 = ((1 as libc::c_int) << 4 as libc::c_int) as u8_0;
    (*collider).shape = (*src).shape;
    return 1 as libc::c_int;
}
/* *
 * Uses default OC2_TYPE_1
 */
#[no_mangle]
pub unsafe extern "C" fn Collider_SetBaseType1(mut globalCtx:
                                                   *mut GlobalContext,
                                               mut collider: *mut Collider,
                                               mut actor: *mut Actor,
                                               mut src:
                                                   *mut ColliderInitType1)
 -> s32 {
    (*collider).actor = actor;
    (*collider).colType = (*src).colType;
    (*collider).atFlags = (*src).atFlags;
    (*collider).acFlags = (*src).acFlags;
    (*collider).ocFlags1 = (*src).ocFlags1;
    (*collider).ocFlags2 = ((1 as libc::c_int) << 4 as libc::c_int) as u8_0;
    (*collider).shape = (*src).shape;
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn Collider_SetBase(mut globalCtx: *mut GlobalContext,
                                          mut collider: *mut Collider,
                                          mut actor: *mut Actor,
                                          mut src: *mut ColliderInit) -> s32 {
    (*collider).actor = actor;
    (*collider).colType = (*src).colType;
    (*collider).atFlags = (*src).atFlags;
    (*collider).acFlags = (*src).acFlags;
    (*collider).ocFlags1 = (*src).ocFlags1;
    (*collider).ocFlags2 = (*src).ocFlags2;
    (*collider).shape = (*src).shape;
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn Collider_ResetATBase(mut globalCtx:
                                                  *mut GlobalContext,
                                              mut collider: *mut Collider) {
    (*collider).at = 0 as *mut Actor;
    (*collider).atFlags =
        ((*collider).atFlags as libc::c_int &
             !((1 as libc::c_int) << 1 as libc::c_int |
                   (1 as libc::c_int) << 2 as libc::c_int)) as u8_0;
}
#[no_mangle]
pub unsafe extern "C" fn Collider_ResetACBase(mut globalCtx:
                                                  *mut GlobalContext,
                                              mut collider: *mut Collider) {
    (*collider).ac = 0 as *mut Actor;
    (*collider).acFlags =
        ((*collider).acFlags as libc::c_int &
             !((1 as libc::c_int) << 1 as libc::c_int |
                   (1 as libc::c_int) << 7 as libc::c_int)) as u8_0;
}
#[no_mangle]
pub unsafe extern "C" fn Collider_ResetOCBase(mut globalCtx:
                                                  *mut GlobalContext,
                                              mut collider: *mut Collider) {
    (*collider).oc = 0 as *mut Actor;
    (*collider).ocFlags1 =
        ((*collider).ocFlags1 as libc::c_int &
             !((1 as libc::c_int) << 1 as libc::c_int)) as u8_0;
    (*collider).ocFlags2 =
        ((*collider).ocFlags2 as libc::c_int &
             !((1 as libc::c_int) << 0 as libc::c_int)) as u8_0;
}
#[no_mangle]
pub unsafe extern "C" fn Collider_InitTouch(mut globalCtx: *mut GlobalContext,
                                            mut touch: *mut ColliderTouch)
 -> s32 {
    static mut init: ColliderTouch =
        {
            let mut init =
                ColliderTouch{dmgFlags: 0 as libc::c_int as u32_0,
                              effect: 0 as libc::c_int as u8_0,
                              damage: 0 as libc::c_int as u8_0,};
            init
        };
    *touch = init;
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn Collider_DestroyTouch(mut globalCtx:
                                                   *mut GlobalContext,
                                               mut touch: *mut ColliderTouch)
 -> s32 {
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn Collider_SetTouch(mut globalCtx: *mut GlobalContext,
                                           mut dest: *mut ColliderTouch,
                                           mut src: *mut ColliderTouch)
 -> s32 {
    (*dest).dmgFlags = (*src).dmgFlags;
    (*dest).effect = (*src).effect;
    (*dest).damage = (*src).damage;
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn Collider_ResetATInfo_Unk(mut globalCtx:
                                                      *mut GlobalContext,
                                                  mut info:
                                                      *mut ColliderInfo) {
}
#[no_mangle]
pub unsafe extern "C" fn Collider_InitBump(mut globalCtx: *mut GlobalContext,
                                           mut bump: *mut ColliderBump)
 -> s32 {
    static mut init: ColliderBump =
        {
            let mut init =
                ColliderBump{dmgFlags: 0xffcfffff as libc::c_uint,
                             effect: 0 as libc::c_int as u8_0,
                             defense: 0 as libc::c_int as u8_0,
                             hitPos:
                                 {
                                     let mut init =
                                         Vec3s{x: 0 as libc::c_int as s16,
                                               y: 0 as libc::c_int as s16,
                                               z: 0 as libc::c_int as s16,};
                                     init
                                 },};
            init
        };
    *bump = init;
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn Collider_DestroyBump(mut globalCtx:
                                                  *mut GlobalContext,
                                              mut bump: *mut ColliderBump)
 -> s32 {
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn Collider_SetBump(mut globalCtx: *mut GlobalContext,
                                          mut bump: *mut ColliderBump,
                                          mut init: *mut ColliderBumpInit)
 -> s32 {
    (*bump).dmgFlags = (*init).dmgFlags;
    (*bump).effect = (*init).effect;
    (*bump).defense = (*init).defense;
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn Collider_InitInfo(mut globalCtx: *mut GlobalContext,
                                           mut info: *mut ColliderInfo)
 -> s32 {
    static mut init: ColliderInfo =
        {
            let mut init =
                ColliderInfo{toucher:
                                 {
                                     let mut init =
                                         ColliderTouch{dmgFlags:
                                                           0 as libc::c_int as
                                                               u32_0,
                                                       effect:
                                                           0 as libc::c_int as
                                                               u8_0,
                                                       damage:
                                                           0 as libc::c_int as
                                                               u8_0,};
                                     init
                                 },
                             bumper:
                                 {
                                     let mut init =
                                         ColliderBump{dmgFlags:
                                                          0xffcfffff as
                                                              libc::c_uint,
                                                      effect:
                                                          0 as libc::c_int as
                                                              u8_0,
                                                      defense:
                                                          0 as libc::c_int as
                                                              u8_0,
                                                      hitPos:
                                                          {
                                                              let mut init =
                                                                  Vec3s{x:
                                                                            0
                                                                                as
                                                                                libc::c_int
                                                                                as
                                                                                s16,
                                                                        y:
                                                                            0
                                                                                as
                                                                                libc::c_int
                                                                                as
                                                                                s16,
                                                                        z:
                                                                            0
                                                                                as
                                                                                libc::c_int
                                                                                as
                                                                                s16,};
                                                              init
                                                          },};
                                     init
                                 },
                             elemType: ELEMTYPE_UNK0 as libc::c_int as u8_0,
                             toucherFlags: 0 as libc::c_int as u8_0,
                             bumperFlags: 0 as libc::c_int as u8_0,
                             ocElemFlags: 0 as libc::c_int as u8_0,
                             atHit: 0 as *const Collider as *mut Collider,
                             acHit: 0 as *const Collider as *mut Collider,
                             atHitInfo:
                                 0 as *const ColliderInfo as
                                     *mut ColliderInfo,
                             acHitInfo:
                                 0 as *const ColliderInfo as
                                     *mut ColliderInfo,};
            init
        };
    *info = init;
    Collider_InitTouch(globalCtx, &mut (*info).toucher);
    Collider_InitBump(globalCtx, &mut (*info).bumper);
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn Collider_DestroyInfo(mut globalCtx:
                                                  *mut GlobalContext,
                                              mut info: *mut ColliderInfo)
 -> s32 {
    Collider_DestroyTouch(globalCtx, &mut (*info).toucher);
    Collider_DestroyBump(globalCtx, &mut (*info).bumper);
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn Collider_SetInfo(mut globalCtx: *mut GlobalContext,
                                          mut info: *mut ColliderInfo,
                                          mut infoInit: *mut ColliderInfoInit)
 -> s32 {
    (*info).elemType = (*infoInit).elemType;
    Collider_SetTouch(globalCtx, &mut (*info).toucher,
                      &mut (*infoInit).toucher);
    Collider_SetBump(globalCtx, &mut (*info).bumper, &mut (*infoInit).bumper);
    (*info).toucherFlags = (*infoInit).toucherFlags;
    (*info).bumperFlags = (*infoInit).bumperFlags;
    (*info).ocElemFlags = (*infoInit).ocElemFlags;
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn Collider_ResetATInfo(mut globalCtx:
                                                  *mut GlobalContext,
                                              mut info: *mut ColliderInfo) {
    (*info).atHit = 0 as *mut Collider;
    (*info).atHitInfo = 0 as *mut ColliderInfo;
    (*info).toucherFlags =
        ((*info).toucherFlags as libc::c_int &
             !((1 as libc::c_int) << 1 as libc::c_int)) as u8_0;
    (*info).toucherFlags =
        ((*info).toucherFlags as libc::c_int &
             !((1 as libc::c_int) << 6 as libc::c_int)) as u8_0;
    Collider_ResetATInfo_Unk(globalCtx, info);
}
#[no_mangle]
pub unsafe extern "C" fn Collider_ResetACInfo(mut globalCtx:
                                                  *mut GlobalContext,
                                              mut info: *mut ColliderInfo) {
    (*info).bumper.hitPos.z = 0 as libc::c_int as s16;
    (*info).bumper.hitPos.y = (*info).bumper.hitPos.z;
    (*info).bumper.hitPos.x = (*info).bumper.hitPos.y;
    (*info).bumperFlags =
        ((*info).bumperFlags as libc::c_int &
             !((1 as libc::c_int) << 1 as libc::c_int)) as u8_0;
    (*info).bumperFlags =
        ((*info).bumperFlags as libc::c_int &
             !((1 as libc::c_int) << 7 as libc::c_int)) as u8_0;
    (*info).acHit = 0 as *mut Collider;
    (*info).acHitInfo = 0 as *mut ColliderInfo;
}
#[no_mangle]
pub unsafe extern "C" fn Collider_ResetOCInfo(mut globalCtx:
                                                  *mut GlobalContext,
                                              mut info: *mut ColliderInfo) {
    (*info).ocElemFlags =
        ((*info).ocElemFlags as libc::c_int &
             !((1 as libc::c_int) << 1 as libc::c_int)) as u8_0;
}
#[no_mangle]
pub unsafe extern "C" fn Collider_InitJntSphElementDim(mut globalCtx:
                                                           *mut GlobalContext,
                                                       mut dim:
                                                           *mut ColliderJntSphElementDim)
 -> s32 {
    static mut init: ColliderJntSphElementDim =
        {
            let mut init =
                ColliderJntSphElementDim{modelSphere:
                                             {
                                                 let mut init =
                                                     Sphere16{center:
                                                                  {
                                                                      let mut init =
                                                                          Vec3s{x:
                                                                                    0
                                                                                        as
                                                                                        libc::c_int
                                                                                        as
                                                                                        s16,
                                                                                y:
                                                                                    0
                                                                                        as
                                                                                        libc::c_int
                                                                                        as
                                                                                        s16,
                                                                                z:
                                                                                    0
                                                                                        as
                                                                                        libc::c_int
                                                                                        as
                                                                                        s16,};
                                                                      init
                                                                  },
                                                              radius:
                                                                  0 as
                                                                      libc::c_int
                                                                      as
                                                                      s16,};
                                                 init
                                             },
                                         worldSphere:
                                             {
                                                 let mut init =
                                                     Sphere16{center:
                                                                  {
                                                                      let mut init =
                                                                          Vec3s{x:
                                                                                    0
                                                                                        as
                                                                                        libc::c_int
                                                                                        as
                                                                                        s16,
                                                                                y:
                                                                                    0
                                                                                        as
                                                                                        libc::c_int
                                                                                        as
                                                                                        s16,
                                                                                z:
                                                                                    0
                                                                                        as
                                                                                        libc::c_int
                                                                                        as
                                                                                        s16,};
                                                                      init
                                                                  },
                                                              radius:
                                                                  0 as
                                                                      libc::c_int
                                                                      as
                                                                      s16,};
                                                 init
                                             },
                                         scale: 0.0f32,
                                         limb: 0 as libc::c_int as u8_0,};
            init
        };
    *dim = init;
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn Collider_DestroyJntSphElementDim(mut globalCtx:
                                                              *mut GlobalContext,
                                                          mut element:
                                                              *mut ColliderJntSphElementDim)
 -> s32 {
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn Collider_SetJntSphElementDim(mut globalCtx:
                                                          *mut GlobalContext,
                                                      mut dest:
                                                          *mut ColliderJntSphElementDim,
                                                      mut src:
                                                          *mut ColliderJntSphElementDimInit)
 -> s32 {
    (*dest).limb = (*src).limb;
    (*dest).modelSphere = (*src).modelSphere;
    (*dest).scale = (*src).scale as libc::c_int as libc::c_float * 0.01f32;
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn Collider_InitJntSphElement(mut globalCtx:
                                                        *mut GlobalContext,
                                                    mut element:
                                                        *mut ColliderJntSphElement)
 -> s32 {
    Collider_InitInfo(globalCtx, &mut (*element).info);
    Collider_InitJntSphElementDim(globalCtx, &mut (*element).dim);
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn Collider_DestroyJntSphElement(mut globalCtx:
                                                           *mut GlobalContext,
                                                       mut element:
                                                           *mut ColliderJntSphElement)
 -> s32 {
    Collider_DestroyInfo(globalCtx, &mut (*element).info);
    Collider_DestroyJntSphElementDim(globalCtx, &mut (*element).dim);
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn Collider_SetJntSphElement(mut globalCtx:
                                                       *mut GlobalContext,
                                                   mut dest:
                                                       *mut ColliderJntSphElement,
                                                   mut src:
                                                       *mut ColliderJntSphElementInit)
 -> s32 {
    Collider_SetInfo(globalCtx, &mut (*dest).info, &mut (*src).info);
    Collider_SetJntSphElementDim(globalCtx, &mut (*dest).dim,
                                 &mut (*src).dim);
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn Collider_ResetJntSphElementAT(mut globalCtx:
                                                           *mut GlobalContext,
                                                       mut collider:
                                                           *mut ColliderJntSphElement)
 -> s32 {
    Collider_ResetATInfo(globalCtx, &mut (*collider).info);
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn Collider_ResetJntSphElementAC(mut globalCtx:
                                                           *mut GlobalContext,
                                                       mut collider:
                                                           *mut ColliderJntSphElement)
 -> s32 {
    Collider_ResetACInfo(globalCtx, &mut (*collider).info);
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn Collider_ResetJntSphElementOC(mut globalCtx:
                                                           *mut GlobalContext,
                                                       mut collider:
                                                           *mut ColliderJntSphElement)
 -> s32 {
    Collider_ResetOCInfo(globalCtx, &mut (*collider).info);
    return 1 as libc::c_int;
}
/* *
 * Initializes a ColliderJntSph to default values
 */
#[no_mangle]
pub unsafe extern "C" fn Collider_InitJntSph(mut globalCtx:
                                                 *mut GlobalContext,
                                             mut collider:
                                                 *mut ColliderJntSph) -> s32 {
    Collider_InitBase(globalCtx, &mut (*collider).base);
    (*collider).count = 0 as libc::c_int;
    (*collider).elements = 0 as *mut ColliderJntSphElement;
    return 1 as libc::c_int;
}
/* *
 * Destroys a dynamically allocated ColliderJntSph
 */
#[no_mangle]
pub unsafe extern "C" fn Collider_FreeJntSph(mut globalCtx:
                                                 *mut GlobalContext,
                                             mut collider:
                                                 *mut ColliderJntSph) -> s32 {
    let mut element: *mut ColliderJntSphElement =
        0 as *mut ColliderJntSphElement;
    Collider_DestroyBase(globalCtx, &mut (*collider).base);
    element = (*collider).elements;
    while element < (*collider).elements.offset((*collider).count as isize) {
        Collider_DestroyJntSphElement(globalCtx, element);
        element = element.offset(1)
    }
    (*collider).count = 0 as libc::c_int;
    if !(*collider).elements.is_null() {
        ZeldaArena_FreeDebug((*collider).elements as *mut libc::c_void,
                             b"../z_collision_check.c\x00" as *const u8 as
                                 *const libc::c_char, 1393 as libc::c_int);
    }
    (*collider).elements = 0 as *mut ColliderJntSphElement;
    return 1 as libc::c_int;
}
/* *
 * Destroys a preallocated ColliderJntSph
 */
#[no_mangle]
pub unsafe extern "C" fn Collider_DestroyJntSph(mut globalCtx:
                                                    *mut GlobalContext,
                                                mut collider:
                                                    *mut ColliderJntSph)
 -> s32 {
    let mut element: *mut ColliderJntSphElement =
        0 as *mut ColliderJntSphElement;
    Collider_DestroyBase(globalCtx, &mut (*collider).base);
    element = (*collider).elements;
    while element < (*collider).elements.offset((*collider).count as isize) {
        Collider_DestroyJntSphElement(globalCtx, element);
        element = element.offset(1)
    }
    (*collider).count = 0 as libc::c_int;
    (*collider).elements = 0 as *mut ColliderJntSphElement;
    return 1 as libc::c_int;
}
/* *
 * Sets up the ColliderJntSph using the values in src, sets it to the actor specified in src, and dynamically allocates
 * the element array. Uses default OC2_TYPE_1 and COLTYPE_HIT0. Unused.
 */
#[no_mangle]
pub unsafe extern "C" fn Collider_SetJntSphToActor(mut globalCtx:
                                                       *mut GlobalContext,
                                                   mut dest:
                                                       *mut ColliderJntSph,
                                                   mut src:
                                                       *mut ColliderJntSphInitToActor)
 -> s32 {
    let mut destElem: *mut ColliderJntSphElement =
        0 as *mut ColliderJntSphElement; // "Can not."
    let mut srcElem: *mut ColliderJntSphElementInit =
        0 as *mut ColliderJntSphElementInit;
    Collider_SetBaseToActor(globalCtx, &mut (*dest).base, &mut (*src).base);
    (*dest).count = (*src).count;
    (*dest).elements =
        ZeldaArena_MallocDebug(((*src).count as
                                    libc::c_uint).wrapping_mul(::std::mem::size_of::<ColliderJntSphElement>()
                                                                   as
                                                                   libc::c_ulong),
                               b"../z_collision_check.c\x00" as *const u8 as
                                   *const libc::c_char, 1443 as libc::c_int)
            as *mut ColliderJntSphElement;
    if (*dest).elements.is_null() {
        (*dest).count = 0 as libc::c_int;
        osSyncPrintf(b"\x1b[31m\x00" as *const u8 as *const libc::c_char);
        osSyncPrintf(b"ClObjJntSph_set():zelda_malloc()\xe5\x87\xba\xe6\x9d\xa5\xe3\x81\xbe\xe3\x81\x9b\xe3\x82\x93\xe3\x80\x82\n\x00"
                         as *const u8 as *const libc::c_char);
        osSyncPrintf(b"\x1b[m\x00" as *const u8 as *const libc::c_char);
        return 0 as libc::c_int
    }
    destElem = (*dest).elements;
    srcElem = (*src).elements;
    while destElem < (*dest).elements.offset((*dest).count as isize) {
        Collider_InitJntSphElement(globalCtx, destElem);
        Collider_SetJntSphElement(globalCtx, destElem, srcElem);
        destElem = destElem.offset(1);
        srcElem = srcElem.offset(1)
    }
    return 1 as libc::c_int;
}
/* *
 * Sets up the ColliderJntSph using the values in src and dynamically allocates the element array. Uses default
 * OC2_TYPE_1. Only used by En_Nwc, an unused and unfinished actor.
 */
#[no_mangle]
pub unsafe extern "C" fn Collider_SetJntSphAllocType1(mut globalCtx:
                                                          *mut GlobalContext,
                                                      mut dest:
                                                          *mut ColliderJntSph,
                                                      mut actor: *mut Actor,
                                                      mut src:
                                                          *mut ColliderJntSphInitType1)
 -> s32 {
    let mut destElem: *mut ColliderJntSphElement =
        0 as *mut ColliderJntSphElement; // "Can not."
    let mut srcElem: *mut ColliderJntSphElementInit =
        0 as *mut ColliderJntSphElementInit;
    Collider_SetBaseType1(globalCtx, &mut (*dest).base, actor,
                          &mut (*src).base);
    (*dest).count = (*src).count;
    (*dest).elements =
        ZeldaArena_MallocDebug(((*src).count as
                                    libc::c_uint).wrapping_mul(::std::mem::size_of::<ColliderJntSphElement>()
                                                                   as
                                                                   libc::c_ulong),
                               b"../z_collision_check.c\x00" as *const u8 as
                                   *const libc::c_char, 1490 as libc::c_int)
            as *mut ColliderJntSphElement;
    if (*dest).elements.is_null() {
        (*dest).count = 0 as libc::c_int;
        osSyncPrintf(b"\x1b[31m\x00" as *const u8 as *const libc::c_char);
        osSyncPrintf(b"ClObjJntSph_set3():zelda_malloc_\xe5\x87\xba\xe6\x9d\xa5\xe3\x81\xbe\xe3\x81\x9b\xe3\x82\x93\xe3\x80\x82\n\x00"
                         as *const u8 as *const libc::c_char);
        osSyncPrintf(b"\x1b[m\x00" as *const u8 as *const libc::c_char);
        return 0 as libc::c_int
    }
    destElem = (*dest).elements;
    srcElem = (*src).elements;
    while destElem < (*dest).elements.offset((*dest).count as isize) {
        Collider_InitJntSphElement(globalCtx, destElem);
        Collider_SetJntSphElement(globalCtx, destElem, srcElem);
        destElem = destElem.offset(1);
        srcElem = srcElem.offset(1)
    }
    return 1 as libc::c_int;
}
/* *
 * Sets up the ColliderJntSph using the values in src and dynamically allocates the element array.
 * Unused.
 */
#[no_mangle]
pub unsafe extern "C" fn Collider_SetJntSphAlloc(mut globalCtx:
                                                     *mut GlobalContext,
                                                 mut dest:
                                                     *mut ColliderJntSph,
                                                 mut actor: *mut Actor,
                                                 mut src:
                                                     *mut ColliderJntSphInit)
 -> s32 {
    let mut destElem: *mut ColliderJntSphElement =
        0 as *mut ColliderJntSphElement; // "Can not."
    let mut srcElem: *mut ColliderJntSphElementInit =
        0 as *mut ColliderJntSphElementInit;
    Collider_SetBase(globalCtx, &mut (*dest).base, actor, &mut (*src).base);
    (*dest).count = (*src).count;
    (*dest).elements =
        ZeldaArena_MallocDebug(((*src).count as
                                    libc::c_uint).wrapping_mul(::std::mem::size_of::<ColliderJntSphElement>()
                                                                   as
                                                                   libc::c_ulong),
                               b"../z_collision_check.c\x00" as *const u8 as
                                   *const libc::c_char, 1551 as libc::c_int)
            as *mut ColliderJntSphElement;
    if (*dest).elements.is_null() {
        (*dest).count = 0 as libc::c_int;
        osSyncPrintf(b"\x1b[31m\x00" as *const u8 as *const libc::c_char);
        osSyncPrintf(b"ClObjJntSph_set5():zelda_malloc\xe5\x87\xba\xe6\x9d\xa5\xe3\x81\xbe\xe3\x81\x9b\xe3\x82\x93\n\x00"
                         as *const u8 as *const libc::c_char);
        osSyncPrintf(b"\x1b[m\x00" as *const u8 as *const libc::c_char);
        return 0 as libc::c_int
    }
    destElem = (*dest).elements;
    srcElem = (*src).elements;
    while destElem < (*dest).elements.offset((*dest).count as isize) {
        Collider_InitJntSphElement(globalCtx, destElem);
        Collider_SetJntSphElement(globalCtx, destElem, srcElem);
        destElem = destElem.offset(1);
        srcElem = srcElem.offset(1)
    }
    return 1 as libc::c_int;
}
/* *
 * Sets up the ColliderJntSph using the values in src, placing the element array in elements.
 */
#[no_mangle]
pub unsafe extern "C" fn Collider_SetJntSph(mut globalCtx: *mut GlobalContext,
                                            mut dest: *mut ColliderJntSph,
                                            mut actor: *mut Actor,
                                            mut src: *mut ColliderJntSphInit,
                                            mut elements:
                                                *mut ColliderJntSphElement)
 -> s32 {
    let mut destElem: *mut ColliderJntSphElement =
        0 as *mut ColliderJntSphElement;
    let mut srcElem: *mut ColliderJntSphElementInit =
        0 as *mut ColliderJntSphElementInit;
    Collider_SetBase(globalCtx, &mut (*dest).base, actor, &mut (*src).base);
    (*dest).count = (*src).count;
    (*dest).elements = elements;
    if !(*dest).elements.is_null() {
    } else {
        __assert(b"pclobj_jntsph->elem_tbl != NULL\x00" as *const u8 as
                     *const libc::c_char,
                 b"../z_collision_check.c\x00" as *const u8 as
                     *const libc::c_char, 1603 as libc::c_int);
    };
    destElem = (*dest).elements;
    srcElem = (*src).elements;
    while destElem < (*dest).elements.offset((*dest).count as isize) {
        Collider_InitJntSphElement(globalCtx, destElem);
        Collider_SetJntSphElement(globalCtx, destElem, srcElem);
        destElem = destElem.offset(1);
        srcElem = srcElem.offset(1)
    }
    return 1 as libc::c_int;
}
/* *
 * Resets the collider's AT collision flags.
 */
#[no_mangle]
pub unsafe extern "C" fn Collider_ResetJntSphAT(mut globalCtx:
                                                    *mut GlobalContext,
                                                mut collider: *mut Collider)
 -> s32 {
    let mut element: *mut ColliderJntSphElement =
        0 as *mut ColliderJntSphElement;
    let mut jntSph: *mut ColliderJntSph = collider as *mut ColliderJntSph;
    Collider_ResetATBase(globalCtx, &mut (*jntSph).base);
    element = (*jntSph).elements;
    while element < (*jntSph).elements.offset((*jntSph).count as isize) {
        Collider_ResetJntSphElementAT(globalCtx, element);
        element = element.offset(1)
    }
    return 1 as libc::c_int;
}
/* *
 * Resets the collider's AC collision flags.
 */
#[no_mangle]
pub unsafe extern "C" fn Collider_ResetJntSphAC(mut globalCtx:
                                                    *mut GlobalContext,
                                                mut collider: *mut Collider)
 -> s32 {
    let mut element: *mut ColliderJntSphElement =
        0 as *mut ColliderJntSphElement;
    let mut jntSph: *mut ColliderJntSph = collider as *mut ColliderJntSph;
    Collider_ResetACBase(globalCtx, &mut (*jntSph).base);
    element = (*jntSph).elements;
    while element < (*jntSph).elements.offset((*jntSph).count as isize) {
        Collider_ResetJntSphElementAC(globalCtx, element);
        element = element.offset(1)
    }
    return 1 as libc::c_int;
}
/* *
 * Resets the collider's OC collision flags.
 */
#[no_mangle]
pub unsafe extern "C" fn Collider_ResetJntSphOC(mut globalCtx:
                                                    *mut GlobalContext,
                                                mut collider: *mut Collider)
 -> s32 {
    let mut element: *mut ColliderJntSphElement =
        0 as *mut ColliderJntSphElement;
    let mut jntSph: *mut ColliderJntSph = collider as *mut ColliderJntSph;
    Collider_ResetOCBase(globalCtx, &mut (*jntSph).base);
    element = (*jntSph).elements;
    while element < (*jntSph).elements.offset((*jntSph).count as isize) {
        Collider_ResetJntSphElementOC(globalCtx, element);
        element = element.offset(1)
    }
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn Collider_InitCylinderDim(mut globalCtx:
                                                      *mut GlobalContext,
                                                  mut dim: *mut Cylinder16)
 -> s32 {
    let mut init: Cylinder16 =
        {
            let mut init =
                Cylinder16{radius: 0 as libc::c_int as s16,
                           height: 0 as libc::c_int as s16,
                           yShift: 0 as libc::c_int as s16,
                           pos:
                               {
                                   let mut init =
                                       Vec3s{x: 0 as libc::c_int as s16,
                                             y: 0 as libc::c_int as s16,
                                             z: 0 as libc::c_int as s16,};
                                   init
                               },};
            init
        };
    *dim = init;
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn Collider_DestroyCylinderDim(mut globalCtx:
                                                         *mut GlobalContext,
                                                     mut dim: *mut Cylinder16)
 -> s32 {
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn Collider_SetCylinderDim(mut globalCtx:
                                                     *mut GlobalContext,
                                                 mut dest: *mut Cylinder16,
                                                 mut src: *mut Cylinder16)
 -> s32 {
    *dest = *src;
    return 1 as libc::c_int;
}
/* *
 * Initializes a ColliderCylinder to default values
 */
#[no_mangle]
pub unsafe extern "C" fn Collider_InitCylinder(mut globalCtx:
                                                   *mut GlobalContext,
                                               mut collider:
                                                   *mut ColliderCylinder)
 -> s32 {
    Collider_InitBase(globalCtx, &mut (*collider).base);
    Collider_InitInfo(globalCtx, &mut (*collider).info);
    Collider_InitCylinderDim(globalCtx, &mut (*collider).dim);
    return 1 as libc::c_int;
}
/* *
 * Destroys a ColliderCylinder
 */
#[no_mangle]
pub unsafe extern "C" fn Collider_DestroyCylinder(mut globalCtx:
                                                      *mut GlobalContext,
                                                  mut collider:
                                                      *mut ColliderCylinder)
 -> s32 {
    Collider_DestroyBase(globalCtx, &mut (*collider).base);
    Collider_DestroyInfo(globalCtx, &mut (*collider).info);
    Collider_DestroyCylinderDim(globalCtx, &mut (*collider).dim);
    return 1 as libc::c_int;
}
/* *
 * Sets up the ColliderCylinder using the values in src and sets it to the actor specified in src. Uses default
 * OC2_TYPE_1 and COLTYPE_0. Used only by DekuJr, who sets it to himself anyways.
 */
#[no_mangle]
pub unsafe extern "C" fn Collider_SetCylinderToActor(mut globalCtx:
                                                         *mut GlobalContext,
                                                     mut collider:
                                                         *mut ColliderCylinder,
                                                     mut src:
                                                         *mut ColliderCylinderInitToActor)
 -> s32 {
    Collider_SetBaseToActor(globalCtx, &mut (*collider).base,
                            &mut (*src).base);
    Collider_SetInfo(globalCtx, &mut (*collider).info, &mut (*src).info);
    Collider_SetCylinderDim(globalCtx, &mut (*collider).dim, &mut (*src).dim);
    return 1 as libc::c_int;
}
/* *
 * Sets up the ColliderCylinder using the values in src. Uses default OC2_TYPE_1
 */
#[no_mangle]
pub unsafe extern "C" fn Collider_SetCylinderType1(mut globalCtx:
                                                       *mut GlobalContext,
                                                   mut collider:
                                                       *mut ColliderCylinder,
                                                   mut actor: *mut Actor,
                                                   mut src:
                                                       *mut ColliderCylinderInitType1)
 -> s32 {
    Collider_SetBaseType1(globalCtx, &mut (*collider).base, actor,
                          &mut (*src).base);
    Collider_SetInfo(globalCtx, &mut (*collider).info, &mut (*src).info);
    Collider_SetCylinderDim(globalCtx, &mut (*collider).dim, &mut (*src).dim);
    return 1 as libc::c_int;
}
/* *
 * Sets up the ColliderCylinder using the values in src.
 */
#[no_mangle]
pub unsafe extern "C" fn Collider_SetCylinder(mut globalCtx:
                                                  *mut GlobalContext,
                                              mut collider:
                                                  *mut ColliderCylinder,
                                              mut actor: *mut Actor,
                                              mut src:
                                                  *mut ColliderCylinderInit)
 -> s32 {
    Collider_SetBase(globalCtx, &mut (*collider).base, actor,
                     &mut (*src).base);
    Collider_SetInfo(globalCtx, &mut (*collider).info, &mut (*src).info);
    Collider_SetCylinderDim(globalCtx, &mut (*collider).dim, &mut (*src).dim);
    return 1 as libc::c_int;
}
/* *
 * Resets the collider's AT collision flags.
 */
#[no_mangle]
pub unsafe extern "C" fn Collider_ResetCylinderAT(mut globalCtx:
                                                      *mut GlobalContext,
                                                  mut collider: *mut Collider)
 -> s32 {
    let mut cylinder: *mut ColliderCylinder =
        collider as *mut ColliderCylinder;
    Collider_ResetATBase(globalCtx, &mut (*cylinder).base);
    Collider_ResetATInfo(globalCtx, &mut (*cylinder).info);
    return 1 as libc::c_int;
}
/* *
 * Resets the collider's AC collision flags.
 */
#[no_mangle]
pub unsafe extern "C" fn Collider_ResetCylinderAC(mut globalCtx:
                                                      *mut GlobalContext,
                                                  mut collider: *mut Collider)
 -> s32 {
    let mut cylinder: *mut ColliderCylinder =
        collider as *mut ColliderCylinder;
    Collider_ResetACBase(globalCtx, &mut (*cylinder).base);
    Collider_ResetACInfo(globalCtx, &mut (*cylinder).info);
    return 1 as libc::c_int;
}
/* *
 * Resets the collider's OC collision flags.
 */
#[no_mangle]
pub unsafe extern "C" fn Collider_ResetCylinderOC(mut globalCtx:
                                                      *mut GlobalContext,
                                                  mut collider: *mut Collider)
 -> s32 {
    let mut cylinder: *mut ColliderCylinder =
        collider as *mut ColliderCylinder;
    Collider_ResetOCBase(globalCtx, &mut (*cylinder).base);
    Collider_ResetOCInfo(globalCtx, &mut (*cylinder).info);
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn Collider_InitTrisElementDim(mut globalCtx:
                                                         *mut GlobalContext,
                                                     mut dim: *mut TriNorm)
 -> s32 {
    static mut init: TriNorm =
        {
            let mut init =
                TriNorm{vtx:
                            [{
                                 let mut init =
                                     Vec3f{x: 0.0f32, y: 0.0f32, z: 0.0f32,};
                                 init
                             },
                             {
                                 let mut init =
                                     Vec3f{x: 0.0f32, y: 0.0f32, z: 0.0f32,};
                                 init
                             },
                             {
                                 let mut init =
                                     Vec3f{x: 0.0f32, y: 0.0f32, z: 0.0f32,};
                                 init
                             }],
                        plane:
                            {
                                let mut init =
                                    Plane{normal:
                                              {
                                                  let mut init =
                                                      Vec3f{x: 0.0f32,
                                                            y: 0.0f32,
                                                            z: 0.0f32,};
                                                  init
                                              },
                                          originDist: 0.0f32,};
                                init
                            },};
            init
        };
    *dim = init;
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn Collider_DestroyTrisElementDim(mut globalCtx:
                                                            *mut GlobalContext,
                                                        mut dim: *mut TriNorm)
 -> s32 {
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn Collider_SetTrisElementDim(mut globalCtx:
                                                        *mut GlobalContext,
                                                    mut dest: *mut TriNorm,
                                                    mut src:
                                                        *mut ColliderTrisElementDimInit)
 -> s32 {
    let mut destVtx: *mut Vec3f = 0 as *mut Vec3f;
    let mut srcVtx: *mut Vec3f = 0 as *mut Vec3f;
    let mut nx: f32_0 = 0.;
    let mut ny: f32_0 = 0.;
    let mut nz: f32_0 = 0.;
    let mut originDist: f32_0 = 0.;
    destVtx = (*dest).vtx.as_mut_ptr();
    srcVtx = (*src).vtx.as_mut_ptr();
    while destVtx < (*dest).vtx.as_mut_ptr().offset(3 as libc::c_int as isize)
          {
        *destVtx = *srcVtx;
        destVtx = destVtx.offset(1);
        srcVtx = srcVtx.offset(1)
    }
    Math3D_DefPlane(&mut *(*src).vtx.as_mut_ptr().offset(0 as libc::c_int as
                                                             isize),
                    &mut *(*src).vtx.as_mut_ptr().offset(1 as libc::c_int as
                                                             isize),
                    &mut *(*src).vtx.as_mut_ptr().offset(2 as libc::c_int as
                                                             isize), &mut nx,
                    &mut ny, &mut nz, &mut originDist);
    (*dest).plane.normal.x = nx;
    (*dest).plane.normal.y = ny;
    (*dest).plane.normal.z = nz;
    (*dest).plane.originDist = originDist;
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn Collider_InitTrisElement(mut globalCtx:
                                                      *mut GlobalContext,
                                                  mut collider:
                                                      *mut ColliderTrisElement)
 -> s32 {
    Collider_InitInfo(globalCtx, &mut (*collider).info);
    Collider_InitTrisElementDim(globalCtx, &mut (*collider).dim);
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn Collider_DestroyTrisElement(mut globalCtx:
                                                         *mut GlobalContext,
                                                     mut collider:
                                                         *mut ColliderTrisElement)
 -> s32 {
    Collider_DestroyInfo(globalCtx, &mut (*collider).info);
    Collider_DestroyTrisElementDim(globalCtx, &mut (*collider).dim);
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn Collider_SetTrisElement(mut globalCtx:
                                                     *mut GlobalContext,
                                                 mut dest:
                                                     *mut ColliderTrisElement,
                                                 mut src:
                                                     *mut ColliderTrisElementInit)
 -> s32 {
    Collider_SetInfo(globalCtx, &mut (*dest).info, &mut (*src).info);
    Collider_SetTrisElementDim(globalCtx, &mut (*dest).dim, &mut (*src).dim);
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn Collider_ResetTrisElementAT(mut globalCtx:
                                                         *mut GlobalContext,
                                                     mut element:
                                                         *mut ColliderTrisElement)
 -> s32 {
    Collider_ResetATInfo(globalCtx, &mut (*element).info);
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn Collider_ResetTrisElementAC(mut globalCtx:
                                                         *mut GlobalContext,
                                                     mut element:
                                                         *mut ColliderTrisElement)
 -> s32 {
    Collider_ResetACInfo(globalCtx, &mut (*element).info);
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn Collider_ResetTrisElementOC(mut globalCtx:
                                                         *mut GlobalContext,
                                                     mut element:
                                                         *mut ColliderTrisElement)
 -> s32 {
    Collider_ResetOCInfo(globalCtx, &mut (*element).info);
    return 1 as libc::c_int;
}
/* *
 * Initializes a ColliderTris to default values
 */
#[no_mangle]
pub unsafe extern "C" fn Collider_InitTris(mut globalCtx: *mut GlobalContext,
                                           mut tris: *mut ColliderTris)
 -> s32 {
    Collider_InitBase(globalCtx, &mut (*tris).base);
    (*tris).count = 0 as libc::c_int;
    (*tris).elements = 0 as *mut ColliderTrisElement;
    return 1 as libc::c_int;
}
/* *
 * Destroys a dynamically allocated ColliderTris
 * Unused
 */
#[no_mangle]
pub unsafe extern "C" fn Collider_FreeTris(mut globalCtx: *mut GlobalContext,
                                           mut tris: *mut ColliderTris)
 -> s32 {
    let mut element: *mut ColliderTrisElement = 0 as *mut ColliderTrisElement;
    Collider_DestroyBase(globalCtx, &mut (*tris).base);
    element = (*tris).elements;
    while element < (*tris).elements.offset((*tris).count as isize) {
        Collider_DestroyTrisElement(globalCtx, element);
        element = element.offset(1)
    }
    (*tris).count = 0 as libc::c_int;
    if !(*tris).elements.is_null() {
        ZeldaArena_FreeDebug((*tris).elements as *mut libc::c_void,
                             b"../z_collision_check.c\x00" as *const u8 as
                                 *const libc::c_char, 2099 as libc::c_int);
    }
    (*tris).elements = 0 as *mut ColliderTrisElement;
    return 1 as libc::c_int;
}
/* *
 * Destroys a preallocated ColliderTris
 */
#[no_mangle]
pub unsafe extern "C" fn Collider_DestroyTris(mut globalCtx:
                                                  *mut GlobalContext,
                                              mut tris: *mut ColliderTris)
 -> s32 {
    let mut element: *mut ColliderTrisElement = 0 as *mut ColliderTrisElement;
    Collider_DestroyBase(globalCtx, &mut (*tris).base);
    element = (*tris).elements;
    while element < (*tris).elements.offset((*tris).count as isize) {
        Collider_DestroyTrisElement(globalCtx, element);
        element = element.offset(1)
    }
    (*tris).count = 0 as libc::c_int;
    (*tris).elements = 0 as *mut ColliderTrisElement;
    return 1 as libc::c_int;
}
/* *
 * Sets up the ColliderTris using the values in src and dynamically allocates the element array. Uses default OC2_TYPE_1
 * Unused.
 */
#[no_mangle]
pub unsafe extern "C" fn Collider_SetTrisAllocType1(mut globalCtx:
                                                        *mut GlobalContext,
                                                    mut dest:
                                                        *mut ColliderTris,
                                                    mut actor: *mut Actor,
                                                    mut src:
                                                        *mut ColliderTrisInitType1)
 -> s32 {
    let mut destElem: *mut ColliderTrisElement =
        0 as *mut ColliderTrisElement; // "Can not."
    let mut srcElem: *mut ColliderTrisElementInit =
        0 as *mut ColliderTrisElementInit;
    Collider_SetBaseType1(globalCtx, &mut (*dest).base, actor,
                          &mut (*src).base);
    (*dest).count = (*src).count;
    (*dest).elements =
        ZeldaArena_MallocDebug(((*dest).count as
                                    libc::c_uint).wrapping_mul(::std::mem::size_of::<ColliderTrisElement>()
                                                                   as
                                                                   libc::c_ulong),
                               b"../z_collision_check.c\x00" as *const u8 as
                                   *const libc::c_char, 2156 as libc::c_int)
            as *mut ColliderTrisElement;
    if (*dest).elements.is_null() {
        (*dest).count = 0 as libc::c_int;
        osSyncPrintf(b"\x1b[31m\x00" as *const u8 as *const libc::c_char);
        osSyncPrintf(b"ClObjTris_set3():zelda_malloc()\xe5\x87\xba\xe6\x9d\xa5\xe3\x81\xbe\xe3\x81\x9b\xe3\x82\x93\n\x00"
                         as *const u8 as *const libc::c_char);
        osSyncPrintf(b"\x1b[m\x00" as *const u8 as *const libc::c_char);
        return 0 as libc::c_int
    }
    destElem = (*dest).elements;
    srcElem = (*src).elements;
    while destElem < (*dest).elements.offset((*dest).count as isize) {
        Collider_InitTrisElement(globalCtx, destElem);
        Collider_SetTrisElement(globalCtx, destElem, srcElem);
        destElem = destElem.offset(1);
        srcElem = srcElem.offset(1)
    }
    return 1 as libc::c_int;
}
/* *
 * Sets up the ColliderTris using the values in src and dynamically allocates the element array.
 * Unused
 */
#[no_mangle]
pub unsafe extern "C" fn Collider_SetTrisAlloc(mut globalCtx:
                                                   *mut GlobalContext,
                                               mut dest: *mut ColliderTris,
                                               mut actor: *mut Actor,
                                               mut src: *mut ColliderTrisInit)
 -> s32 {
    let mut destElem: *mut ColliderTrisElement =
        0 as *mut ColliderTrisElement; // "Can not."
    let mut srcElem: *mut ColliderTrisElementInit =
        0 as *mut ColliderTrisElementInit;
    Collider_SetBase(globalCtx, &mut (*dest).base, actor, &mut (*src).base);
    (*dest).count = (*src).count;
    (*dest).elements =
        ZeldaArena_MallocDebug(((*dest).count as
                                    libc::c_uint).wrapping_mul(::std::mem::size_of::<ColliderTrisElement>()
                                                                   as
                                                                   libc::c_ulong),
                               b"../z_collision_check.c\x00" as *const u8 as
                                   *const libc::c_char, 2207 as libc::c_int)
            as *mut ColliderTrisElement;
    if (*dest).elements.is_null() {
        osSyncPrintf(b"\x1b[31m\x00" as *const u8 as *const libc::c_char);
        osSyncPrintf(b"ClObjTris_set5():zelda_malloc\xe5\x87\xba\xe6\x9d\xa5\xe3\x81\xbe\xe3\x81\x9b\xe3\x82\x93\n\x00"
                         as *const u8 as *const libc::c_char);
        osSyncPrintf(b"\x1b[m\x00" as *const u8 as *const libc::c_char);
        (*dest).count = 0 as libc::c_int;
        return 0 as libc::c_int
    }
    destElem = (*dest).elements;
    srcElem = (*src).elements;
    while destElem < (*dest).elements.offset((*dest).count as isize) {
        Collider_InitTrisElement(globalCtx, destElem);
        Collider_SetTrisElement(globalCtx, destElem, srcElem);
        destElem = destElem.offset(1);
        srcElem = srcElem.offset(1)
    }
    return 1 as libc::c_int;
}
/* *
 * Sets up the ColliderTris using the values in src, placing the element array in elements.
 */
#[no_mangle]
pub unsafe extern "C" fn Collider_SetTris(mut globalCtx: *mut GlobalContext,
                                          mut dest: *mut ColliderTris,
                                          mut actor: *mut Actor,
                                          mut src: *mut ColliderTrisInit,
                                          mut elements:
                                              *mut ColliderTrisElement)
 -> s32 {
    let mut destElem: *mut ColliderTrisElement =
        0 as *mut ColliderTrisElement;
    let mut srcElem: *mut ColliderTrisElementInit =
        0 as *mut ColliderTrisElementInit;
    Collider_SetBase(globalCtx, &mut (*dest).base, actor, &mut (*src).base);
    (*dest).count = (*src).count;
    (*dest).elements = elements;
    if !(*dest).elements.is_null() {
    } else {
        __assert(b"pclobj_tris->elem_tbl != NULL\x00" as *const u8 as
                     *const libc::c_char,
                 b"../z_collision_check.c\x00" as *const u8 as
                     *const libc::c_char, 2258 as libc::c_int);
    };
    destElem = (*dest).elements;
    srcElem = (*src).elements;
    while destElem < (*dest).elements.offset((*dest).count as isize) {
        Collider_InitTrisElement(globalCtx, destElem);
        Collider_SetTrisElement(globalCtx, destElem, srcElem);
        destElem = destElem.offset(1);
        srcElem = srcElem.offset(1)
    }
    return 1 as libc::c_int;
}
/* *
 * Resets the collider's AT collision flags.
 */
#[no_mangle]
pub unsafe extern "C" fn Collider_ResetTrisAT(mut globalCtx:
                                                  *mut GlobalContext,
                                              mut collider: *mut Collider)
 -> s32 {
    let mut element: *mut ColliderTrisElement = 0 as *mut ColliderTrisElement;
    let mut tris: *mut ColliderTris = collider as *mut ColliderTris;
    Collider_ResetATBase(globalCtx, &mut (*tris).base);
    element = (*tris).elements;
    while element < (*tris).elements.offset((*tris).count as isize) {
        Collider_ResetTrisElementAT(globalCtx, element);
        element = element.offset(1)
    }
    return 1 as libc::c_int;
}
/* *
 * Resets the collider's AC collision flags.
 */
#[no_mangle]
pub unsafe extern "C" fn Collider_ResetTrisAC(mut globalCtx:
                                                  *mut GlobalContext,
                                              mut collider: *mut Collider)
 -> s32 {
    let mut element: *mut ColliderTrisElement = 0 as *mut ColliderTrisElement;
    let mut tris: *mut ColliderTris = collider as *mut ColliderTris;
    Collider_ResetACBase(globalCtx, &mut (*tris).base);
    element = (*tris).elements;
    while element < (*tris).elements.offset((*tris).count as isize) {
        Collider_ResetTrisElementAC(globalCtx, element);
        element = element.offset(1)
    }
    return 1 as libc::c_int;
}
/* *
 * Resets the collider's OC collision flags.
 */
#[no_mangle]
pub unsafe extern "C" fn Collider_ResetTrisOC(mut globalCtx:
                                                  *mut GlobalContext,
                                              mut collider: *mut Collider)
 -> s32 {
    let mut element: *mut ColliderTrisElement = 0 as *mut ColliderTrisElement;
    let mut tris: *mut ColliderTris = collider as *mut ColliderTris;
    Collider_ResetOCBase(globalCtx, &mut (*tris).base);
    element = (*tris).elements;
    while element < (*tris).elements.offset((*tris).count as isize) {
        Collider_ResetTrisElementOC(globalCtx, element);
        element = element.offset(1)
    }
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn Collider_InitQuadDim(mut globalCtx:
                                                  *mut GlobalContext,
                                              mut dim: *mut ColliderQuadDim)
 -> s32 {
    static mut init: ColliderQuadDim =
        {
            let mut init =
                ColliderQuadDim{quad:
                                    [{
                                         let mut init =
                                             Vec3f{x: 0.0f32,
                                                   y: 0.0f32,
                                                   z: 0.0f32,};
                                         init
                                     },
                                     {
                                         let mut init =
                                             Vec3f{x: 0.0f32,
                                                   y: 0.0f32,
                                                   z: 0.0f32,};
                                         init
                                     },
                                     {
                                         let mut init =
                                             Vec3f{x: 0.0f32,
                                                   y: 0.0f32,
                                                   z: 0.0f32,};
                                         init
                                     },
                                     {
                                         let mut init =
                                             Vec3f{x: 0.0f32,
                                                   y: 0.0f32,
                                                   z: 0.0f32,};
                                         init
                                     }],
                                dcMid:
                                    {
                                        let mut init =
                                            Vec3s{x: 0 as libc::c_int as s16,
                                                  y: 0 as libc::c_int as s16,
                                                  z:
                                                      0 as libc::c_int as
                                                          s16,};
                                        init
                                    },
                                baMid:
                                    {
                                        let mut init =
                                            Vec3s{x: 0 as libc::c_int as s16,
                                                  y: 0 as libc::c_int as s16,
                                                  z:
                                                      0 as libc::c_int as
                                                          s16,};
                                        init
                                    },
                                acDist: 1.0E38f32,};
            init
        };
    *dim = init;
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn Collider_DestroyQuadDim(mut globalCtx:
                                                     *mut GlobalContext,
                                                 mut dim:
                                                     *mut ColliderQuadDim)
 -> s32 {
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn Collider_ResetQuadACDist(mut globalCtx:
                                                      *mut GlobalContext,
                                                  mut dim:
                                                      *mut ColliderQuadDim)
 -> s32 {
    (*dim).acDist = 1.0E38f32;
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn Collider_SetQuadMidpoints(mut dim:
                                                       *mut ColliderQuadDim) {
    (*dim).dcMid.x =
        (((*dim).quad[3 as libc::c_int as usize].x +
              (*dim).quad[2 as libc::c_int as usize].x) * 0.5f32) as s16;
    (*dim).dcMid.y =
        (((*dim).quad[3 as libc::c_int as usize].y +
              (*dim).quad[2 as libc::c_int as usize].y) * 0.5f32) as s16;
    (*dim).dcMid.z =
        (((*dim).quad[3 as libc::c_int as usize].z +
              (*dim).quad[2 as libc::c_int as usize].z) * 0.5f32) as s16;
    (*dim).baMid.x =
        (((*dim).quad[1 as libc::c_int as usize].x +
              (*dim).quad[0 as libc::c_int as usize].x) * 0.5f32) as s16;
    (*dim).baMid.y =
        (((*dim).quad[1 as libc::c_int as usize].y +
              (*dim).quad[0 as libc::c_int as usize].y) * 0.5f32) as s16;
    (*dim).baMid.z =
        (((*dim).quad[1 as libc::c_int as usize].z +
              (*dim).quad[0 as libc::c_int as usize].z) * 0.5f32) as s16;
}
#[no_mangle]
pub unsafe extern "C" fn Collider_SetQuadDim(mut globalCtx:
                                                 *mut GlobalContext,
                                             mut dest: *mut ColliderQuadDim,
                                             mut src:
                                                 *mut ColliderQuadDimInit)
 -> s32 {
    (*dest).quad[0 as libc::c_int as usize] =
        (*src).quad[0 as libc::c_int as usize];
    (*dest).quad[1 as libc::c_int as usize] =
        (*src).quad[1 as libc::c_int as usize];
    (*dest).quad[2 as libc::c_int as usize] =
        (*src).quad[2 as libc::c_int as usize];
    (*dest).quad[3 as libc::c_int as usize] =
        (*src).quad[3 as libc::c_int as usize];
    Collider_SetQuadMidpoints(dest);
    return 1 as libc::c_int;
}
/* *
 * Initializes a ColliderQuad to default values.
 */
#[no_mangle]
pub unsafe extern "C" fn Collider_InitQuad(mut globalCtx: *mut GlobalContext,
                                           mut collider: *mut ColliderQuad)
 -> s32 {
    Collider_InitBase(globalCtx, &mut (*collider).base);
    Collider_InitInfo(globalCtx, &mut (*collider).info);
    Collider_InitQuadDim(globalCtx, &mut (*collider).dim);
    return 1 as libc::c_int;
}
/* *
 * Destroys a ColliderQuad.
 */
#[no_mangle]
pub unsafe extern "C" fn Collider_DestroyQuad(mut globalCtx:
                                                  *mut GlobalContext,
                                              mut collider: *mut ColliderQuad)
 -> s32 {
    Collider_DestroyBase(globalCtx, &mut (*collider).base);
    Collider_DestroyInfo(globalCtx, &mut (*collider).info);
    Collider_DestroyQuadDim(globalCtx, &mut (*collider).dim);
    return 1 as libc::c_int;
}
/* *
 * Sets up the ColliderQuad using the values in src. Uses the default OC2_TYPE_1
 */
#[no_mangle]
pub unsafe extern "C" fn Collider_SetQuadType1(mut globalCtx:
                                                   *mut GlobalContext,
                                               mut collider:
                                                   *mut ColliderQuad,
                                               mut actor: *mut Actor,
                                               mut src:
                                                   *mut ColliderQuadInitType1)
 -> s32 {
    Collider_SetBaseType1(globalCtx, &mut (*collider).base, actor,
                          &mut (*src).base);
    Collider_SetInfo(globalCtx, &mut (*collider).info, &mut (*src).info);
    Collider_SetQuadDim(globalCtx, &mut (*collider).dim, &mut (*src).dim);
    return 1 as libc::c_int;
}
/* *
 * Sets up the ColliderQuad using the values in src.
 */
#[no_mangle]
pub unsafe extern "C" fn Collider_SetQuad(mut globalCtx: *mut GlobalContext,
                                          mut collider: *mut ColliderQuad,
                                          mut actor: *mut Actor,
                                          mut src: *mut ColliderQuadInit)
 -> s32 {
    Collider_SetBase(globalCtx, &mut (*collider).base, actor,
                     &mut (*src).base);
    Collider_SetInfo(globalCtx, &mut (*collider).info, &mut (*src).info);
    Collider_SetQuadDim(globalCtx, &mut (*collider).dim, &mut (*src).dim);
    return 1 as libc::c_int;
}
/* *
 * Resets the collider's AT collision flags.
 */
#[no_mangle]
pub unsafe extern "C" fn Collider_ResetQuadAT(mut globalCtx:
                                                  *mut GlobalContext,
                                              mut collider: *mut Collider)
 -> s32 {
    let mut quad: *mut ColliderQuad = collider as *mut ColliderQuad;
    Collider_ResetATBase(globalCtx, &mut (*quad).base);
    Collider_ResetATInfo(globalCtx, &mut (*quad).info);
    Collider_ResetQuadACDist(globalCtx, &mut (*quad).dim);
    return 1 as libc::c_int;
}
/* *
 * Resets the collider's AC collision flags.
 */
#[no_mangle]
pub unsafe extern "C" fn Collider_ResetQuadAC(mut globalCtx:
                                                  *mut GlobalContext,
                                              mut collider: *mut Collider)
 -> s32 {
    let mut quad: *mut ColliderQuad = collider as *mut ColliderQuad;
    Collider_ResetACBase(globalCtx, &mut (*quad).base);
    Collider_ResetACInfo(globalCtx, &mut (*quad).info);
    return 1 as libc::c_int;
}
/* *
 * Resets the collider's OC collision flags.
 */
#[no_mangle]
pub unsafe extern "C" fn Collider_ResetQuadOC(mut globalCtx:
                                                  *mut GlobalContext,
                                              mut collider: *mut Collider)
 -> s32 {
    let mut quad: *mut ColliderQuad = collider as *mut ColliderQuad;
    Collider_ResetOCBase(globalCtx, &mut (*quad).base);
    Collider_ResetOCInfo(globalCtx, &mut (*quad).info);
    return 1 as libc::c_int;
}
/* *
 * For quad colliders with AT_NEAREST, resets the previous AC collider it hit if the current element is closer,
 * otherwise returns false. Used on player AT colliders to prevent multiple collisions from registering.
 */
#[no_mangle]
pub unsafe extern "C" fn Collider_QuadSetNearestAC(mut globalCtx:
                                                       *mut GlobalContext,
                                                   mut quad:
                                                       *mut ColliderQuad,
                                                   mut hitPos: *mut Vec3f)
 -> s32 {
    let mut acDist: f32_0 = 0.;
    let mut dcMid: Vec3f = Vec3f{x: 0., y: 0., z: 0.,};
    if (*quad).info.toucherFlags as libc::c_int &
           (1 as libc::c_int) << 2 as libc::c_int == 0 {
        return 1 as libc::c_int
    }
    Math_Vec3s_ToVec3f(&mut dcMid, &mut (*quad).dim.dcMid);
    acDist = Math3D_Vec3fDistSq(&mut dcMid, hitPos);
    if acDist < (*quad).dim.acDist {
        (*quad).dim.acDist = acDist;
        if !(*quad).info.atHit.is_null() {
            Collider_ResetACBase(globalCtx, (*quad).info.atHit);
        }
        if !(*quad).info.atHitInfo.is_null() {
            Collider_ResetACInfo(globalCtx, (*quad).info.atHitInfo);
        }
        return 1 as libc::c_int
    }
    return 0 as libc::c_int;
}
/* *
 * Initializes an OcLine to default values
 * OcLines are entirely unused.
 */
#[no_mangle]
pub unsafe extern "C" fn Collider_InitLine(mut globalCtx: *mut GlobalContext,
                                           mut line: *mut OcLine) -> s32 {
    let mut init: Vec3f =
        { let mut init = Vec3f{x: 0.0f32, y: 0.0f32, z: 0.0f32,}; init };
    Math_Vec3f_Copy(&mut (*line).line.a, &mut init);
    Math_Vec3f_Copy(&mut (*line).line.b, &mut init);
    return 1 as libc::c_int;
}
/* *
 * Destroys an OcLine
 * OcLines are entirely unused.
 */
#[no_mangle]
pub unsafe extern "C" fn Collider_DestroyLine(mut globalCtx:
                                                  *mut GlobalContext,
                                              mut line: *mut OcLine) -> s32 {
    return 1 as libc::c_int;
}
/* *
 * Sets up an OcLine with endpoints a and b.
 * OcLines are entirely unused.
 */
#[no_mangle]
pub unsafe extern "C" fn Collider_SetLinePoints(mut GlobalContext:
                                                    *mut GlobalContext,
                                                mut ocLine: *mut OcLine,
                                                mut a: *mut Vec3f,
                                                mut b: *mut Vec3f) -> s32 {
    Math_Vec3f_Copy(&mut (*ocLine).line.a, a);
    Math_Vec3f_Copy(&mut (*ocLine).line.b, b);
    return 1 as libc::c_int;
}
/* *
 * Sets up an OcLine using the values in src.
 * OcLines are entirely unused.
 */
#[no_mangle]
pub unsafe extern "C" fn Collider_SetLine(mut globalCtx: *mut GlobalContext,
                                          mut dest: *mut OcLine,
                                          mut src: *mut OcLine) -> s32 {
    (*dest).ocFlags = (*src).ocFlags;
    Collider_SetLinePoints(globalCtx, dest, &mut (*src).line.a,
                           &mut (*src).line.b);
    return 1 as libc::c_int;
}
/* *
 * Resets the OcLine's collision flags.
 * OcLines are entirely unused.
 */
#[no_mangle]
pub unsafe extern "C" fn Collider_ResetLineOC(mut globalCtx:
                                                  *mut GlobalContext,
                                              mut line: *mut OcLine) -> s32 {
    (*line).ocFlags =
        ((*line).ocFlags as libc::c_int &
             !((1 as libc::c_int) << 0 as libc::c_int)) as u16_0;
    return 1 as libc::c_int;
}
/* *
 * Initializes CollisionCheckContext. Clears all collider arrays, disables SAC, and sets flags for drawing colliders.
 */
#[no_mangle]
pub unsafe extern "C" fn CollisionCheck_InitContext(mut globalCtx:
                                                        *mut GlobalContext,
                                                    mut colChkCtx:
                                                        *mut CollisionCheckContext) {
    (*colChkCtx).sacFlags = 0 as libc::c_int as u16_0;
    CollisionCheck_ClearContext(globalCtx, colChkCtx);
    (*gGameInfo).data[(19 as libc::c_int * 6 as libc::c_int *
                           16 as libc::c_int + 21 as libc::c_int) as usize] =
        1 as libc::c_int as s16;
    (*gGameInfo).data[(19 as libc::c_int * 6 as libc::c_int *
                           16 as libc::c_int + 22 as libc::c_int) as usize] =
        1 as libc::c_int as s16;
    (*gGameInfo).data[(19 as libc::c_int * 6 as libc::c_int *
                           16 as libc::c_int + 23 as libc::c_int) as usize] =
        1 as libc::c_int as s16;
}
#[no_mangle]
pub unsafe extern "C" fn CollisionCheck_DestroyContext(mut globalCtx:
                                                           *mut GlobalContext,
                                                       mut colChkCtx:
                                                           *mut CollisionCheckContext) {
}
/* *
 * Clears all collider lists in CollisionCheckContext when not in SAC mode.
 */
#[no_mangle]
pub unsafe extern "C" fn CollisionCheck_ClearContext(mut globalCtx:
                                                         *mut GlobalContext,
                                                     mut colChkCtx:
                                                         *mut CollisionCheckContext) {
    let mut col: *mut *mut Collider = 0 as *mut *mut Collider;
    let mut line: *mut *mut OcLine = 0 as *mut *mut OcLine;
    if (*colChkCtx).sacFlags as libc::c_int & 1 as libc::c_int == 0 {
        (*colChkCtx).colATCount = 0 as libc::c_int as s16;
        (*colChkCtx).colACCount = 0 as libc::c_int;
        (*colChkCtx).colOCCount = 0 as libc::c_int;
        (*colChkCtx).colLineCount = 0 as libc::c_int;
        col = (*colChkCtx).colAT.as_mut_ptr();
        while col <
                  (*colChkCtx).colAT.as_mut_ptr().offset(50 as libc::c_int as
                                                             isize) {
            *col = 0 as *mut Collider;
            col = col.offset(1)
        }
        col = (*colChkCtx).colAC.as_mut_ptr();
        while col <
                  (*colChkCtx).colAC.as_mut_ptr().offset(60 as libc::c_int as
                                                             isize) {
            *col = 0 as *mut Collider;
            col = col.offset(1)
        }
        col = (*colChkCtx).colOC.as_mut_ptr();
        while col <
                  (*colChkCtx).colOC.as_mut_ptr().offset(50 as libc::c_int as
                                                             isize) {
            *col = 0 as *mut Collider;
            col = col.offset(1)
        }
        line = (*colChkCtx).colLine.as_mut_ptr();
        while line <
                  (*colChkCtx).colLine.as_mut_ptr().offset(3 as libc::c_int as
                                                               isize) {
            *line = 0 as *mut OcLine;
            line = line.offset(1)
        }
    };
}
/* *
 * Enables SAC, an alternate collision check mode that allows direct management of collider lists. Unused.
 */
#[no_mangle]
pub unsafe extern "C" fn CollisionCheck_EnableSAC(mut globalCtx:
                                                      *mut GlobalContext,
                                                  mut colChkCtx:
                                                      *mut CollisionCheckContext) {
    (*colChkCtx).sacFlags =
        ((*colChkCtx).sacFlags as libc::c_int | 1 as libc::c_int) as u16_0;
}
/* *
 * Disables SAC, an alternate collision check mode that allows direct management of collider lists. Unused.
 */
#[no_mangle]
pub unsafe extern "C" fn CollisionCheck_DisableSAC(mut globalCtx:
                                                       *mut GlobalContext,
                                                   mut colChkCtx:
                                                       *mut CollisionCheckContext) {
    (*colChkCtx).sacFlags =
        ((*colChkCtx).sacFlags as libc::c_int & !(1 as libc::c_int)) as u16_0;
}
/* *
 * Draws a collider of any shape.
 * Math3D_DrawSphere and Math3D_DrawCylinder are noops, so JntSph and Cylinder are not drawn.
 */
#[no_mangle]
pub unsafe extern "C" fn Collider_Draw(mut globalCtx: *mut GlobalContext,
                                       mut collider: *mut Collider) {
    let mut jntSph: *mut ColliderJntSph = 0 as *mut ColliderJntSph;
    let mut cylinder: *mut ColliderCylinder = 0 as *mut ColliderCylinder;
    let mut tris: *mut ColliderTris = 0 as *mut ColliderTris;
    let mut quad: *mut ColliderQuad = 0 as *mut ColliderQuad;
    let mut i: s32 = 0;
    if collider.is_null() { return }
    match (*collider).shape as libc::c_int {
        0 => {
            jntSph = collider as *mut ColliderJntSph;
            i = 0 as libc::c_int;
            while i < (*jntSph).count {
                Math3D_DrawSphere(globalCtx,
                                  &mut (*(*jntSph).elements.offset(i as
                                                                       isize)).dim.worldSphere);
                i += 1
            }
        }
        1 => {
            cylinder = collider as *mut ColliderCylinder;
            Math3D_DrawCylinder(globalCtx, &mut (*cylinder).dim);
        }
        2 => {
            tris = collider as *mut ColliderTris;
            i = 0 as libc::c_int;
            while i < (*tris).count {
                Collider_DrawRedPoly((*globalCtx).state.gfxCtx,
                                     &mut *(*(*tris).elements.offset(i as
                                                                         isize)).dim.vtx.as_mut_ptr().offset(0
                                                                                                                 as
                                                                                                                 libc::c_int
                                                                                                                 as
                                                                                                                 isize),
                                     &mut *(*(*tris).elements.offset(i as
                                                                         isize)).dim.vtx.as_mut_ptr().offset(1
                                                                                                                 as
                                                                                                                 libc::c_int
                                                                                                                 as
                                                                                                                 isize),
                                     &mut *(*(*tris).elements.offset(i as
                                                                         isize)).dim.vtx.as_mut_ptr().offset(2
                                                                                                                 as
                                                                                                                 libc::c_int
                                                                                                                 as
                                                                                                                 isize));
                i += 1
            }
        }
        3 => {
            quad = collider as *mut ColliderQuad;
            Collider_DrawRedPoly((*globalCtx).state.gfxCtx,
                                 &mut *(*quad).dim.quad.as_mut_ptr().offset(2
                                                                                as
                                                                                libc::c_int
                                                                                as
                                                                                isize),
                                 &mut *(*quad).dim.quad.as_mut_ptr().offset(3
                                                                                as
                                                                                libc::c_int
                                                                                as
                                                                                isize),
                                 &mut *(*quad).dim.quad.as_mut_ptr().offset(1
                                                                                as
                                                                                libc::c_int
                                                                                as
                                                                                isize));
            Collider_DrawRedPoly((*globalCtx).state.gfxCtx,
                                 &mut *(*quad).dim.quad.as_mut_ptr().offset(1
                                                                                as
                                                                                libc::c_int
                                                                                as
                                                                                isize),
                                 &mut *(*quad).dim.quad.as_mut_ptr().offset(0
                                                                                as
                                                                                libc::c_int
                                                                                as
                                                                                isize),
                                 &mut *(*quad).dim.quad.as_mut_ptr().offset(2
                                                                                as
                                                                                libc::c_int
                                                                                as
                                                                                isize));
        }
        _ => { }
    };
}
/* *
 * Draws collision if AREG(15) and other AREGs are set. AREG(21) draws AT colliders, AREG(22) draws AC colliders,
 * AREG(23) draws OC colliders, AREG(24) draws dynapolys, and AREG(25) draws bg polys
 */
#[no_mangle]
pub unsafe extern "C" fn CollisionCheck_DrawCollision(mut globalCtx:
                                                          *mut GlobalContext,
                                                      mut colChkCtx:
                                                          *mut CollisionCheckContext) {
    let mut collider: *mut Collider = 0 as *mut Collider;
    let mut i: s32 = 0;
    if (*gGameInfo).data[(19 as libc::c_int * 6 as libc::c_int *
                              16 as libc::c_int + 15 as libc::c_int) as usize]
           != 0 {
        if (*gGameInfo).data[(19 as libc::c_int * 6 as libc::c_int *
                                  16 as libc::c_int + 21 as libc::c_int) as
                                 usize] != 0 {
            i = 0 as libc::c_int;
            while i < (*colChkCtx).colATCount as libc::c_int {
                Collider_Draw(globalCtx, (*colChkCtx).colAT[i as usize]);
                i += 1
            }
        }
        if (*gGameInfo).data[(19 as libc::c_int * 6 as libc::c_int *
                                  16 as libc::c_int + 22 as libc::c_int) as
                                 usize] != 0 {
            i = 0 as libc::c_int;
            while i < (*colChkCtx).colACCount {
                Collider_Draw(globalCtx, (*colChkCtx).colAC[i as usize]);
                i += 1
            }
        }
        if (*gGameInfo).data[(19 as libc::c_int * 6 as libc::c_int *
                                  16 as libc::c_int + 23 as libc::c_int) as
                                 usize] != 0 {
            i = 0 as libc::c_int;
            while i < (*colChkCtx).colOCCount {
                collider = (*colChkCtx).colOC[i as usize];
                if (*collider).ocFlags1 as libc::c_int &
                       (1 as libc::c_int) << 0 as libc::c_int != 0 {
                    Collider_Draw(globalCtx, collider);
                }
                i += 1
            }
        }
        if (*gGameInfo).data[(19 as libc::c_int * 6 as libc::c_int *
                                  16 as libc::c_int + 24 as libc::c_int) as
                                 usize] != 0 {
            BgCheck_DrawDynaCollision(globalCtx, &mut (*globalCtx).colCtx);
        }
        if (*gGameInfo).data[(19 as libc::c_int * 6 as libc::c_int *
                                  16 as libc::c_int + 25 as libc::c_int) as
                                 usize] != 0 {
            BgCheck_DrawStaticCollision(globalCtx, &mut (*globalCtx).colCtx);
        }
    };
}
static mut sATResetFuncs: [ColChkResetFunc; 4] =
    unsafe {
        [Some(Collider_ResetJntSphAT as
                  unsafe extern "C" fn(_: *mut GlobalContext,
                                       _: *mut Collider) -> s32),
         Some(Collider_ResetCylinderAT as
                  unsafe extern "C" fn(_: *mut GlobalContext,
                                       _: *mut Collider) -> s32),
         Some(Collider_ResetTrisAT as
                  unsafe extern "C" fn(_: *mut GlobalContext,
                                       _: *mut Collider) -> s32),
         Some(Collider_ResetQuadAT as
                  unsafe extern "C" fn(_: *mut GlobalContext,
                                       _: *mut Collider) -> s32)]
    };
/* *
 * Sets collider as an AT (attack) for the current frame, which will be checked against ACs (attack colliders)
 * The last argument takes a Collider, so pass collider.base rather than the raw collider.
 */
#[no_mangle]
pub unsafe extern "C" fn CollisionCheck_SetAT(mut globalCtx:
                                                  *mut GlobalContext,
                                              mut colChkCtx:
                                                  *mut CollisionCheckContext,
                                              mut collider: *mut Collider)
 -> s32 {
    let mut index: s32 = 0;
    if FrameAdvance_IsEnabled(globalCtx) == 1 as libc::c_int {
        return -(1 as libc::c_int)
    }
    if (*collider).shape as libc::c_int <= COLSHAPE_QUAD as libc::c_int {
    } else {
        __assert(b"pcl_obj->data_type <= CL_DATA_LBL_SWRD\x00" as *const u8 as
                     *const libc::c_char,
                 b"../z_collision_check.c\x00" as *const u8 as
                     *const libc::c_char, 2997 as libc::c_int);
    };
    sATResetFuncs[(*collider).shape as
                      usize].expect("non-null function pointer")(globalCtx,
                                                                 collider);
    if !(*collider).actor.is_null() && (*(*collider).actor).update.is_none() {
        return -(1 as libc::c_int)
    }
    if (*colChkCtx).colATCount as libc::c_int >= 50 as libc::c_int {
        // "Index exceeded and cannot add more"
        osSyncPrintf(b"CollisionCheck_setAT():\xe3\x82\xa4\xe3\x83\xb3\xe3\x83\x87\xe3\x83\x83\xe3\x82\xaf\xe3\x82\xb9\xe3\x81\x8c\xe3\x82\xaa\xe3\x83\xbc\xe3\x83\x90\xe3\x83\xbc\xe3\x81\x97\xe3\x81\xa6\xe8\xbf\xbd\xe5\x8a\xa0\xe4\xb8\x8d\xe8\x83\xbd\n\x00"
                         as *const u8 as *const libc::c_char);
        return -(1 as libc::c_int)
    }
    if (*colChkCtx).sacFlags as libc::c_int & 1 as libc::c_int != 0 {
        return -(1 as libc::c_int)
    }
    index = (*colChkCtx).colATCount as s32;
    let fresh12 = (*colChkCtx).colATCount;
    (*colChkCtx).colATCount = (*colChkCtx).colATCount + 1;
    (*colChkCtx).colAT[fresh12 as usize] = collider;
    return index;
}
/* *
 * Unused. Sets collider as an AT (attack) for the current frame, which will be checked against ACs (attack colliders).
 * If CollisionCheck_SAC is enabled, the collider will be inserted into the list at the specified index, otherwise it
 * will be inserted into the next slot
 */
#[no_mangle]
pub unsafe extern "C" fn CollisionCheck_SetAT_SAC(mut globalCtx:
                                                      *mut GlobalContext,
                                                  mut colChkCtx:
                                                      *mut CollisionCheckContext,
                                                  mut collider: *mut Collider,
                                                  mut index: s32) -> s32 {
    if (*collider).shape as libc::c_int <= COLSHAPE_QUAD as libc::c_int {
    } else {
        __assert(b"pcl_obj->data_type <= CL_DATA_LBL_SWRD\x00" as *const u8 as
                     *const libc::c_char,
                 b"../z_collision_check.c\x00" as *const u8 as
                     *const libc::c_char, 3037 as libc::c_int);
    };
    if FrameAdvance_IsEnabled(globalCtx) == 1 as libc::c_int {
        return -(1 as libc::c_int)
    }
    sATResetFuncs[(*collider).shape as
                      usize].expect("non-null function pointer")(globalCtx,
                                                                 collider);
    if !(*collider).actor.is_null() && (*(*collider).actor).update.is_none() {
        return -(1 as libc::c_int)
    }
    if (*colChkCtx).sacFlags as libc::c_int & 1 as libc::c_int != 0 {
        if !(index < (*colChkCtx).colATCount as libc::c_int) {
            // "You are trying to register a location that is larger than the total number of data."
            osSyncPrintf(b"CollisionCheck_setAT_SAC():\xe5\x85\xa8\xe3\x83\x87\xe3\x83\xbc\xe3\x82\xbf\xe6\x95\xb0\xe3\x82\x88\xe3\x82\x8a\xe5\xa4\xa7\xe3\x81\x8d\xe3\x81\x84\xe3\x81\xa8\xe3\x81\x93\xe3\x82\x8d\xe3\x81\xab\xe7\x99\xbb\xe9\x8c\xb2\xe3\x81\x97\xe3\x82\x88\xe3\x81\x86\xe3\x81\xa8\xe3\x81\x97\xe3\x81\xa6\xe3\x81\x84\xe3\x82\x8b\xe3\x80\x82\n\x00"
                             as *const u8 as *const libc::c_char);
            return -(1 as libc::c_int)
        }
        (*colChkCtx).colAT[index as usize] = collider
    } else {
        if !(((*colChkCtx).colATCount as libc::c_int) < 50 as libc::c_int) {
            // "Index exceeded and cannot add more"
            osSyncPrintf(b"CollisionCheck_setAT():\xe3\x82\xa4\xe3\x83\xb3\xe3\x83\x87\xe3\x83\x83\xe3\x82\xaf\xe3\x82\xb9\xe3\x81\x8c\xe3\x82\xaa\xe3\x83\xbc\xe3\x83\x90\xe3\x83\xbc\xe3\x81\x97\xe3\x81\xa6\xe8\xbf\xbd\xe5\x8a\xa0\xe4\xb8\x8d\xe8\x83\xbd\n\x00"
                             as *const u8 as *const libc::c_char);
            return -(1 as libc::c_int)
        }
        index = (*colChkCtx).colATCount as s32;
        let fresh13 = (*colChkCtx).colATCount;
        (*colChkCtx).colATCount = (*colChkCtx).colATCount + 1;
        (*colChkCtx).colAT[fresh13 as usize] = collider
    }
    return index;
}
static mut sACResetFuncs: [ColChkResetFunc; 4] =
    unsafe {
        [Some(Collider_ResetJntSphAC as
                  unsafe extern "C" fn(_: *mut GlobalContext,
                                       _: *mut Collider) -> s32),
         Some(Collider_ResetCylinderAC as
                  unsafe extern "C" fn(_: *mut GlobalContext,
                                       _: *mut Collider) -> s32),
         Some(Collider_ResetTrisAC as
                  unsafe extern "C" fn(_: *mut GlobalContext,
                                       _: *mut Collider) -> s32),
         Some(Collider_ResetQuadAC as
                  unsafe extern "C" fn(_: *mut GlobalContext,
                                       _: *mut Collider) -> s32)]
    };
/* *
 * Sets collider as an AC (attack collider) for the current frame, allowing it to detect ATs (attacks)
 * The last argument takes a Collider, so pass collider.base rather than the raw collider.
 */
#[no_mangle]
pub unsafe extern "C" fn CollisionCheck_SetAC(mut globalCtx:
                                                  *mut GlobalContext,
                                              mut colChkCtx:
                                                  *mut CollisionCheckContext,
                                              mut collider: *mut Collider)
 -> s32 {
    let mut index: s32 = 0;
    if FrameAdvance_IsEnabled(globalCtx) == 1 as libc::c_int {
        return -(1 as libc::c_int)
    }
    if (*collider).shape as libc::c_int <= COLSHAPE_QUAD as libc::c_int {
    } else {
        __assert(b"pcl_obj->data_type <= CL_DATA_LBL_SWRD\x00" as *const u8 as
                     *const libc::c_char,
                 b"../z_collision_check.c\x00" as *const u8 as
                     *const libc::c_char, 3114 as libc::c_int);
    };
    sACResetFuncs[(*collider).shape as
                      usize].expect("non-null function pointer")(globalCtx,
                                                                 collider);
    if !(*collider).actor.is_null() && (*(*collider).actor).update.is_none() {
        return -(1 as libc::c_int)
    }
    if (*colChkCtx).colACCount >= 60 as libc::c_int {
        // "Index exceeded and cannot add more"
        osSyncPrintf(b"CollisionCheck_setAC():\xe3\x82\xa4\xe3\x83\xb3\xe3\x83\x87\xe3\x83\x83\xe3\x82\xaf\xe3\x82\xb9\xe3\x81\x8c\xe3\x82\xaa\xe3\x83\xbc\xe3\x83\x90\xe3\x81\x97\xe3\x81\xa6\xe8\xbf\xbd\xe5\x8a\xa0\xe4\xb8\x8d\xe8\x83\xbd\n\x00"
                         as *const u8 as *const libc::c_char);
        return -(1 as libc::c_int)
    }
    if (*colChkCtx).sacFlags as libc::c_int & 1 as libc::c_int != 0 {
        return -(1 as libc::c_int)
    }
    index = (*colChkCtx).colACCount;
    let fresh14 = (*colChkCtx).colACCount;
    (*colChkCtx).colACCount = (*colChkCtx).colACCount + 1;
    (*colChkCtx).colAC[fresh14 as usize] = collider;
    return index;
}
/* *
 * Unused. Sets collider as an AC (attack collider) for the current frame, allowing it to detect ATs (attacks).
 * If CollisionCheck_SAC is enabled, the collider will be inserted into the list at the specified index, otherwise it
 * will be inserted into the next slot
 */
#[no_mangle]
pub unsafe extern "C" fn CollisionCheck_SetAC_SAC(mut globalCtx:
                                                      *mut GlobalContext,
                                                  mut colChkCtx:
                                                      *mut CollisionCheckContext,
                                                  mut collider: *mut Collider,
                                                  mut index: s32) -> s32 {
    if (*collider).shape as libc::c_int <= COLSHAPE_QUAD as libc::c_int {
    } else {
        __assert(b"pcl_obj->data_type <= CL_DATA_LBL_SWRD\x00" as *const u8 as
                     *const libc::c_char,
                 b"../z_collision_check.c\x00" as *const u8 as
                     *const libc::c_char, 3153 as libc::c_int);
    };
    if FrameAdvance_IsEnabled(globalCtx) == 1 as libc::c_int {
        return -(1 as libc::c_int)
    }
    sACResetFuncs[(*collider).shape as
                      usize].expect("non-null function pointer")(globalCtx,
                                                                 collider);
    if !(*collider).actor.is_null() && (*(*collider).actor).update.is_none() {
        return -(1 as libc::c_int)
    }
    if (*colChkCtx).sacFlags as libc::c_int & 1 as libc::c_int != 0 {
        if !(index < (*colChkCtx).colACCount) {
            // "You are trying to register a location that is larger than the total number of data."
            osSyncPrintf(b"CollisionCheck_setAC_SAC():\xe5\x85\xa8\xe3\x83\x87\xe3\x83\xbc\xe3\x82\xbf\xe6\x95\xb0\xe3\x82\x88\xe3\x82\x8a\xe5\xa4\xa7\xe3\x81\x8d\xe3\x81\x84\xe3\x81\xa8\xe3\x81\x93\xe3\x82\x8d\xe3\x81\xab\xe7\x99\xbb\xe9\x8c\xb2\xe3\x81\x97\xe3\x82\x88\xe3\x81\x86\xe3\x81\xa8\xe3\x81\x97\xe3\x81\xa6\xe3\x81\x84\xe3\x82\x8b\xe3\x80\x82\n\x00"
                             as *const u8 as *const libc::c_char);
            return -(1 as libc::c_int)
        }
        (*colChkCtx).colAC[index as usize] = collider
    } else {
        if !((*colChkCtx).colACCount < 60 as libc::c_int) {
            // "Index exceeded and cannot add more"
            osSyncPrintf(b"CollisionCheck_setAC():\xe3\x82\xa4\xe3\x83\xb3\xe3\x83\x87\xe3\x83\x83\xe3\x82\xaf\xe3\x82\xb9\xe3\x81\x8c\xe3\x82\xaa\xe3\x83\xbc\xe3\x83\x90\xe3\x81\x97\xe3\x81\xa6\xe8\xbf\xbd\xe5\x8a\xa0\xe4\xb8\x8d\xe8\x83\xbd\n\x00"
                             as *const u8 as *const libc::c_char);
            return -(1 as libc::c_int)
        }
        index = (*colChkCtx).colACCount;
        let fresh15 = (*colChkCtx).colACCount;
        (*colChkCtx).colACCount = (*colChkCtx).colACCount + 1;
        (*colChkCtx).colAC[fresh15 as usize] = collider
    }
    return index;
}
static mut sOCResetFuncs: [ColChkResetFunc; 4] =
    unsafe {
        [Some(Collider_ResetJntSphOC as
                  unsafe extern "C" fn(_: *mut GlobalContext,
                                       _: *mut Collider) -> s32),
         Some(Collider_ResetCylinderOC as
                  unsafe extern "C" fn(_: *mut GlobalContext,
                                       _: *mut Collider) -> s32),
         Some(Collider_ResetTrisOC as
                  unsafe extern "C" fn(_: *mut GlobalContext,
                                       _: *mut Collider) -> s32),
         Some(Collider_ResetQuadOC as
                  unsafe extern "C" fn(_: *mut GlobalContext,
                                       _: *mut Collider) -> s32)]
    };
/* *
 * Sets collider as an OC (object collider) for the current frame, allowing it to detect other OCs
 * The last argument takes a Collider, so pass collider.base rather than the raw collider.
 */
#[no_mangle]
pub unsafe extern "C" fn CollisionCheck_SetOC(mut globalCtx:
                                                  *mut GlobalContext,
                                              mut colChkCtx:
                                                  *mut CollisionCheckContext,
                                              mut collider: *mut Collider)
 -> s32 {
    let mut index: s32 = 0;
    if FrameAdvance_IsEnabled(globalCtx) == 1 as libc::c_int {
        return -(1 as libc::c_int)
    }
    if (*collider).shape as libc::c_int <= COLSHAPE_QUAD as libc::c_int {
    } else {
        __assert(b"pcl_obj->data_type <= CL_DATA_LBL_SWRD\x00" as *const u8 as
                     *const libc::c_char,
                 b"../z_collision_check.c\x00" as *const u8 as
                     *const libc::c_char, 3229 as libc::c_int);
    };
    sOCResetFuncs[(*collider).shape as
                      usize].expect("non-null function pointer")(globalCtx,
                                                                 collider);
    if !(*collider).actor.is_null() && (*(*collider).actor).update.is_none() {
        return -(1 as libc::c_int)
    }
    if (*colChkCtx).colOCCount >= 50 as libc::c_int {
        // "Index exceeded and cannot add more"
        osSyncPrintf(b"CollisionCheck_setOC():\xe3\x82\xa4\xe3\x83\xb3\xe3\x83\x87\xe3\x83\x83\xe3\x82\xaf\xe3\x82\xb9\xe3\x81\x8c\xe3\x82\xaa\xe3\x83\xbc\xe3\x83\x90\xe3\x81\x97\xe3\x81\xa6\xe8\xbf\xbd\xe5\x8a\xa0\xe4\xb8\x8d\xe8\x83\xbd\n\x00"
                         as *const u8 as *const libc::c_char);
        return -(1 as libc::c_int)
    }
    if (*colChkCtx).sacFlags as libc::c_int & 1 as libc::c_int != 0 {
        return -(1 as libc::c_int)
    }
    index = (*colChkCtx).colOCCount;
    let fresh16 = (*colChkCtx).colOCCount;
    (*colChkCtx).colOCCount = (*colChkCtx).colOCCount + 1;
    (*colChkCtx).colOC[fresh16 as usize] = collider;
    return index;
}
/* *
 * Unused. Sets collider as an OC (object collider) for the current frame, allowing it to detect other OCs
 * If CollisionCheck_SAC is enabled, the collider will be inserted into the list at the specified index, otherwise it
 * will be inserted into the next slot
 */
#[no_mangle]
pub unsafe extern "C" fn CollisionCheck_SetOC_SAC(mut globalCtx:
                                                      *mut GlobalContext,
                                                  mut colChkCtx:
                                                      *mut CollisionCheckContext,
                                                  mut collider: *mut Collider,
                                                  mut index: s32) -> s32 {
    if FrameAdvance_IsEnabled(globalCtx) == 1 as libc::c_int {
        return -(1 as libc::c_int)
    }
    if (*collider).shape as libc::c_int <= COLSHAPE_QUAD as libc::c_int {
    } else {
        __assert(b"pcl_obj->data_type <= CL_DATA_LBL_SWRD\x00" as *const u8 as
                     *const libc::c_char,
                 b"../z_collision_check.c\x00" as *const u8 as
                     *const libc::c_char, 3274 as libc::c_int);
    };
    sOCResetFuncs[(*collider).shape as
                      usize].expect("non-null function pointer")(globalCtx,
                                                                 collider);
    if !(*collider).actor.is_null() && (*(*collider).actor).update.is_none() {
        return -(1 as libc::c_int)
    }
    if (*colChkCtx).sacFlags as libc::c_int & 1 as libc::c_int != 0 {
        if !(index < (*colChkCtx).colOCCount) {
            // "You are trying to register a location that is larger than the total number of data."
            osSyncPrintf(b"CollisionCheck_setOC_SAC():\xe5\x85\xa8\xe3\x83\x87\xe3\x83\xbc\xe3\x82\xbf\xe6\x95\xb0\xe3\x82\x88\xe3\x82\x8a\xe5\xa4\xa7\xe3\x81\x8d\xe3\x81\x84\xe3\x81\xa8\xe3\x81\x93\xe3\x82\x8d\xe3\x81\xab\xe7\x99\xbb\xe9\x8c\xb2\xe3\x81\x97\xe3\x82\x88\xe3\x81\x86\xe3\x81\xa8\xe3\x81\x97\xe3\x81\xa6\xe3\x81\x84\xe3\x82\x8b\xe3\x80\x82\n\x00"
                             as *const u8 as *const libc::c_char);
            return -(1 as libc::c_int)
        }
        // ! @bug Should be colOC
        (*colChkCtx).colAT[index as usize] = collider
    } else {
        if !((*colChkCtx).colOCCount < 50 as libc::c_int) {
            // "Index exceeded and cannot add more"
            osSyncPrintf(b"CollisionCheck_setOC():\xe3\x82\xa4\xe3\x83\xb3\xe3\x83\x87\xe3\x83\x83\xe3\x82\xaf\xe3\x82\xb9\xe3\x81\x8c\xe3\x82\xaa\xe3\x83\xbc\xe3\x83\x90\xe3\x81\x97\xe3\x81\xa6\xe8\xbf\xbd\xe5\x8a\xa0\xe4\xb8\x8d\xe8\x83\xbd\n\x00"
                             as *const u8 as *const libc::c_char);
            return -(1 as libc::c_int)
        }
        index = (*colChkCtx).colOCCount;
        let fresh17 = (*colChkCtx).colOCCount;
        (*colChkCtx).colOCCount = (*colChkCtx).colOCCount + 1;
        (*colChkCtx).colOC[fresh17 as usize] = collider
    }
    return index;
}
/* *
 * Sets a line as an OC collider for this frame.
 * OC lines are entirely unused, and do not even have collision check functions.
 */
#[no_mangle]
pub unsafe extern "C" fn CollisionCheck_SetOCLine(mut globalCtx:
                                                      *mut GlobalContext,
                                                  mut colChkCtx:
                                                      *mut CollisionCheckContext,
                                                  mut collider: *mut OcLine)
 -> s32 {
    let mut index: s32 = 0;
    if FrameAdvance_IsEnabled(globalCtx) == 1 as libc::c_int {
        return -(1 as libc::c_int)
    }
    Collider_ResetLineOC(globalCtx, collider);
    if !((*colChkCtx).colLineCount < 3 as libc::c_int) {
        // "Index exceeded and cannot add more"
        osSyncPrintf(b"CollisionCheck_setOCLine():\xe3\x82\xa4\xe3\x83\xb3\xe3\x83\x87\xe3\x83\x83\xe3\x82\xaf\xe3\x82\xb9\xe3\x81\x8c\xe3\x82\xaa\xe3\x83\xbc\xe3\x83\x90\xe3\x81\x97\xe3\x81\xa6\xe8\xbf\xbd\xe5\x8a\xa0\xe4\xb8\x8d\xe8\x83\xbd\n\x00"
                         as *const u8 as *const libc::c_char);
        return -(1 as libc::c_int)
    }
    index = (*colChkCtx).colLineCount;
    let fresh18 = (*colChkCtx).colLineCount;
    (*colChkCtx).colLineCount = (*colChkCtx).colLineCount + 1;
    (*colChkCtx).colLine[fresh18 as usize] = collider;
    return index;
}
/* *
 * Skips AT elements that are off.
 */
#[no_mangle]
pub unsafe extern "C" fn CollisionCheck_SkipTouch(mut info: *mut ColliderInfo)
 -> s32 {
    if (*info).toucherFlags as libc::c_int &
           (1 as libc::c_int) << 0 as libc::c_int == 0 {
        return 1 as libc::c_int
    }
    return 0 as libc::c_int;
}
/* *
 * Skips AC elements that are off.
 */
#[no_mangle]
pub unsafe extern "C" fn CollisionCheck_SkipBump(mut info: *mut ColliderInfo)
 -> s32 {
    if (*info).bumperFlags as libc::c_int &
           (1 as libc::c_int) << 0 as libc::c_int == 0 {
        return 1 as libc::c_int
    }
    return 0 as libc::c_int;
}
/* *
 * If the AT element has no dmgFlags in common with the AC element, no collision happens.
 */
#[no_mangle]
pub unsafe extern "C" fn CollisionCheck_NoSharedFlags(mut atInfo:
                                                          *mut ColliderInfo,
                                                      mut acInfo:
                                                          *mut ColliderInfo)
 -> s32 {
    if (*atInfo).toucher.dmgFlags & (*acInfo).bumper.dmgFlags == 0 {
        return 1 as libc::c_int
    }
    return 0 as libc::c_int;
}
/* *
 * Spawns no blood drops.
 * Used by collider types HIT1, HIT3, HIT5, METAL, NONE, WOOD, HARD, and TREE
 */
#[no_mangle]
pub unsafe extern "C" fn CollisionCheck_NoBlood(mut globalCtx:
                                                    *mut GlobalContext,
                                                mut collider: *mut Collider,
                                                mut v: *mut Vec3f) {
}
/* *
 * Spawns blue blood drops.
 * Used by collider types HIT0 and HIT8.
 */
#[no_mangle]
pub unsafe extern "C" fn CollisionCheck_BlueBlood(mut globalCtx:
                                                      *mut GlobalContext,
                                                  mut collider: *mut Collider,
                                                  mut v: *mut Vec3f) {
    static mut D_8015D8A0: EffectSparkInit =
        EffectSparkInit{position: Vec3s{x: 0, y: 0, z: 0,},
                        numElements: 0,
                        elements:
                            [EffectSparkElement{velocity:
                                                    Vec3f{x: 0.,
                                                          y: 0.,
                                                          z: 0.,},
                                                position:
                                                    Vec3f{x: 0.,
                                                          y: 0.,
                                                          z: 0.,},
                                                unkVelocity:
                                                    Vec3s{x: 0, y: 0, z: 0,},
                                                unkPosition:
                                                    Vec3s{x: 0,
                                                          y: 0,
                                                          z: 0,},}; 32],
                        speed: 0.,
                        gravity: 0.,
                        uDiv: 0,
                        vDiv: 0,
                        colorStart: [Color_RGBA8{r: 0, g: 0, b: 0, a: 0,}; 4],
                        colorEnd: [Color_RGBA8{r: 0, g: 0, b: 0, a: 0,}; 4],
                        timer: 0,
                        duration: 0,};
    let mut effectIndex: s32 = 0;
    D_8015D8A0.position.x = (*v).x as s16;
    D_8015D8A0.position.y = (*v).y as s16;
    D_8015D8A0.position.z = (*v).z as s16;
    D_8015D8A0.uDiv = 5 as libc::c_int as u32_0;
    D_8015D8A0.vDiv = 5 as libc::c_int as u32_0;
    D_8015D8A0.colorStart[0 as libc::c_int as usize].r =
        10 as libc::c_int as u8_0;
    D_8015D8A0.colorStart[0 as libc::c_int as usize].g =
        10 as libc::c_int as u8_0;
    D_8015D8A0.colorStart[0 as libc::c_int as usize].b =
        200 as libc::c_int as u8_0;
    D_8015D8A0.colorStart[0 as libc::c_int as usize].a =
        255 as libc::c_int as u8_0;
    D_8015D8A0.colorStart[1 as libc::c_int as usize].r =
        0 as libc::c_int as u8_0;
    D_8015D8A0.colorStart[1 as libc::c_int as usize].g =
        0 as libc::c_int as u8_0;
    D_8015D8A0.colorStart[1 as libc::c_int as usize].b =
        128 as libc::c_int as u8_0;
    D_8015D8A0.colorStart[1 as libc::c_int as usize].a =
        255 as libc::c_int as u8_0;
    D_8015D8A0.colorStart[2 as libc::c_int as usize].r =
        0 as libc::c_int as u8_0;
    D_8015D8A0.colorStart[2 as libc::c_int as usize].g =
        0 as libc::c_int as u8_0;
    D_8015D8A0.colorStart[2 as libc::c_int as usize].b =
        128 as libc::c_int as u8_0;
    D_8015D8A0.colorStart[2 as libc::c_int as usize].a =
        255 as libc::c_int as u8_0;
    D_8015D8A0.colorStart[3 as libc::c_int as usize].r =
        0 as libc::c_int as u8_0;
    D_8015D8A0.colorStart[3 as libc::c_int as usize].g =
        0 as libc::c_int as u8_0;
    D_8015D8A0.colorStart[3 as libc::c_int as usize].b =
        128 as libc::c_int as u8_0;
    D_8015D8A0.colorStart[3 as libc::c_int as usize].a =
        255 as libc::c_int as u8_0;
    D_8015D8A0.colorEnd[0 as libc::c_int as usize].r =
        0 as libc::c_int as u8_0;
    D_8015D8A0.colorEnd[0 as libc::c_int as usize].g =
        0 as libc::c_int as u8_0;
    D_8015D8A0.colorEnd[0 as libc::c_int as usize].b =
        32 as libc::c_int as u8_0;
    D_8015D8A0.colorEnd[0 as libc::c_int as usize].a =
        0 as libc::c_int as u8_0;
    D_8015D8A0.colorEnd[1 as libc::c_int as usize].r =
        0 as libc::c_int as u8_0;
    D_8015D8A0.colorEnd[1 as libc::c_int as usize].g =
        0 as libc::c_int as u8_0;
    D_8015D8A0.colorEnd[1 as libc::c_int as usize].b =
        32 as libc::c_int as u8_0;
    D_8015D8A0.colorEnd[1 as libc::c_int as usize].a =
        0 as libc::c_int as u8_0;
    D_8015D8A0.colorEnd[2 as libc::c_int as usize].r =
        0 as libc::c_int as u8_0;
    D_8015D8A0.colorEnd[2 as libc::c_int as usize].g =
        0 as libc::c_int as u8_0;
    D_8015D8A0.colorEnd[2 as libc::c_int as usize].b =
        64 as libc::c_int as u8_0;
    D_8015D8A0.colorEnd[2 as libc::c_int as usize].a =
        0 as libc::c_int as u8_0;
    D_8015D8A0.colorEnd[3 as libc::c_int as usize].r =
        0 as libc::c_int as u8_0;
    D_8015D8A0.colorEnd[3 as libc::c_int as usize].g =
        0 as libc::c_int as u8_0;
    D_8015D8A0.colorEnd[3 as libc::c_int as usize].b =
        64 as libc::c_int as u8_0;
    D_8015D8A0.colorEnd[3 as libc::c_int as usize].a =
        0 as libc::c_int as u8_0;
    D_8015D8A0.timer = 0 as libc::c_int;
    D_8015D8A0.duration = 16 as libc::c_int;
    D_8015D8A0.speed = 8.0f32;
    D_8015D8A0.gravity = -1.0f32;
    Effect_Add(globalCtx, &mut effectIndex, EFFECT_SPARK as libc::c_int,
               0 as libc::c_int as u8_0, 1 as libc::c_int as u8_0,
               &mut D_8015D8A0 as *mut EffectSparkInit as *mut libc::c_void);
}
/* *
 * Spawns green blood drops.
 * Used by collider types HIT2 and HIT6. No actor has type HIT2.
 */
#[no_mangle]
pub unsafe extern "C" fn CollisionCheck_GreenBlood(mut globalCtx:
                                                       *mut GlobalContext,
                                                   mut collider:
                                                       *mut Collider,
                                                   mut v: *mut Vec3f) {
    static mut D_8015DD68: EffectSparkInit =
        EffectSparkInit{position: Vec3s{x: 0, y: 0, z: 0,},
                        numElements: 0,
                        elements:
                            [EffectSparkElement{velocity:
                                                    Vec3f{x: 0.,
                                                          y: 0.,
                                                          z: 0.,},
                                                position:
                                                    Vec3f{x: 0.,
                                                          y: 0.,
                                                          z: 0.,},
                                                unkVelocity:
                                                    Vec3s{x: 0, y: 0, z: 0,},
                                                unkPosition:
                                                    Vec3s{x: 0,
                                                          y: 0,
                                                          z: 0,},}; 32],
                        speed: 0.,
                        gravity: 0.,
                        uDiv: 0,
                        vDiv: 0,
                        colorStart: [Color_RGBA8{r: 0, g: 0, b: 0, a: 0,}; 4],
                        colorEnd: [Color_RGBA8{r: 0, g: 0, b: 0, a: 0,}; 4],
                        timer: 0,
                        duration: 0,};
    let mut effectIndex: s32 = 0;
    D_8015DD68.position.x = (*v).x as s16;
    D_8015DD68.position.y = (*v).y as s16;
    D_8015DD68.position.z = (*v).z as s16;
    D_8015DD68.uDiv = 5 as libc::c_int as u32_0;
    D_8015DD68.vDiv = 5 as libc::c_int as u32_0;
    D_8015DD68.colorStart[0 as libc::c_int as usize].r =
        10 as libc::c_int as u8_0;
    D_8015DD68.colorStart[0 as libc::c_int as usize].g =
        200 as libc::c_int as u8_0;
    D_8015DD68.colorStart[0 as libc::c_int as usize].b =
        10 as libc::c_int as u8_0;
    D_8015DD68.colorStart[0 as libc::c_int as usize].a =
        255 as libc::c_int as u8_0;
    D_8015DD68.colorStart[1 as libc::c_int as usize].r =
        0 as libc::c_int as u8_0;
    D_8015DD68.colorStart[1 as libc::c_int as usize].g =
        128 as libc::c_int as u8_0;
    D_8015DD68.colorStart[1 as libc::c_int as usize].b =
        0 as libc::c_int as u8_0;
    D_8015DD68.colorStart[1 as libc::c_int as usize].a =
        255 as libc::c_int as u8_0;
    D_8015DD68.colorStart[2 as libc::c_int as usize].r =
        0 as libc::c_int as u8_0;
    D_8015DD68.colorStart[2 as libc::c_int as usize].g =
        128 as libc::c_int as u8_0;
    D_8015DD68.colorStart[2 as libc::c_int as usize].b =
        0 as libc::c_int as u8_0;
    D_8015DD68.colorStart[2 as libc::c_int as usize].a =
        255 as libc::c_int as u8_0;
    D_8015DD68.colorStart[3 as libc::c_int as usize].r =
        0 as libc::c_int as u8_0;
    D_8015DD68.colorStart[3 as libc::c_int as usize].g =
        128 as libc::c_int as u8_0;
    D_8015DD68.colorStart[3 as libc::c_int as usize].b =
        0 as libc::c_int as u8_0;
    D_8015DD68.colorStart[3 as libc::c_int as usize].a =
        255 as libc::c_int as u8_0;
    D_8015DD68.colorEnd[0 as libc::c_int as usize].r =
        0 as libc::c_int as u8_0;
    D_8015DD68.colorEnd[0 as libc::c_int as usize].g =
        32 as libc::c_int as u8_0;
    D_8015DD68.colorEnd[0 as libc::c_int as usize].b =
        0 as libc::c_int as u8_0;
    D_8015DD68.colorEnd[0 as libc::c_int as usize].a =
        0 as libc::c_int as u8_0;
    D_8015DD68.colorEnd[1 as libc::c_int as usize].r =
        0 as libc::c_int as u8_0;
    D_8015DD68.colorEnd[1 as libc::c_int as usize].g =
        32 as libc::c_int as u8_0;
    D_8015DD68.colorEnd[1 as libc::c_int as usize].b =
        0 as libc::c_int as u8_0;
    D_8015DD68.colorEnd[1 as libc::c_int as usize].a =
        0 as libc::c_int as u8_0;
    D_8015DD68.colorEnd[2 as libc::c_int as usize].r =
        0 as libc::c_int as u8_0;
    D_8015DD68.colorEnd[2 as libc::c_int as usize].g =
        64 as libc::c_int as u8_0;
    D_8015DD68.colorEnd[2 as libc::c_int as usize].b =
        0 as libc::c_int as u8_0;
    D_8015DD68.colorEnd[2 as libc::c_int as usize].a =
        0 as libc::c_int as u8_0;
    D_8015DD68.colorEnd[3 as libc::c_int as usize].r =
        0 as libc::c_int as u8_0;
    D_8015DD68.colorEnd[3 as libc::c_int as usize].g =
        64 as libc::c_int as u8_0;
    D_8015DD68.colorEnd[3 as libc::c_int as usize].b =
        0 as libc::c_int as u8_0;
    D_8015DD68.colorEnd[3 as libc::c_int as usize].a =
        0 as libc::c_int as u8_0;
    D_8015DD68.timer = 0 as libc::c_int;
    D_8015DD68.duration = 16 as libc::c_int;
    D_8015DD68.speed = 8.0f32;
    D_8015DD68.gravity = -1.0f32;
    Effect_Add(globalCtx, &mut effectIndex, EFFECT_SPARK as libc::c_int,
               0 as libc::c_int as u8_0, 1 as libc::c_int as u8_0,
               &mut D_8015DD68 as *mut EffectSparkInit as *mut libc::c_void);
}
/* *
 * Spawns a burst of water.
 * Used by collider type HIT4, which no actor has.
 */
#[no_mangle]
pub unsafe extern "C" fn CollisionCheck_WaterBurst(mut globalCtx:
                                                       *mut GlobalContext,
                                                   mut collider:
                                                       *mut Collider,
                                                   mut pos: *mut Vec3f) {
    EffectSsSibuki_SpawnBurst(globalCtx, pos);
    CollisionCheck_SpawnWaterDroplets(globalCtx, pos);
}
/* *
 * Spawns red blood drops.
 * Used by collider type HIT7, which no actor has.
 */
#[no_mangle]
pub unsafe extern "C" fn CollisionCheck_RedBlood(mut globalCtx:
                                                     *mut GlobalContext,
                                                 mut collider: *mut Collider,
                                                 mut v: *mut Vec3f) {
    CollisionCheck_SpawnRedBlood(globalCtx, v);
}
/* *
 * Spawns red blood drops.
 * Unused.
 */
#[no_mangle]
pub unsafe extern "C" fn CollisionCheck_RedBloodUnused(mut globalCtx:
                                                           *mut GlobalContext,
                                                       mut collider:
                                                           *mut Collider,
                                                       mut v: *mut Vec3f) {
    CollisionCheck_SpawnRedBlood(globalCtx, v);
}
/* *
 * Plays sound effects and displays hitmarks for solid-type AC colliders (METAL, WOOD, HARD, and TREE)
 */
#[no_mangle]
pub unsafe extern "C" fn CollisionCheck_HitSolid(mut globalCtx:
                                                     *mut GlobalContext,
                                                 mut info: *mut ColliderInfo,
                                                 mut collider: *mut Collider,
                                                 mut hitPos: *mut Vec3f) {
    let mut flags: s32 =
        (*info).toucherFlags as libc::c_int &
            (3 as libc::c_int) << 3 as libc::c_int;
    if flags == (0 as libc::c_int) << 3 as libc::c_int &&
           (*collider).colType as libc::c_int != COLTYPE_METAL as libc::c_int
       {
        EffectSsHitMark_SpawnFixedScale(globalCtx,
                                        EFFECT_HITMARK_WHITE as libc::c_int,
                                        hitPos);
        if (*collider).actor.is_null() {
            Audio_PlaySoundGeneral(0x1806 as libc::c_int as u16_0,
                                   &mut D_801333D4, 4 as libc::c_int as u8_0,
                                   &mut D_801333E0, &mut D_801333E0,
                                   &mut D_801333E8);
        } else {
            Audio_PlaySoundGeneral(0x1806 as libc::c_int as u16_0,
                                   &mut (*(*collider).actor).projectedPos,
                                   4 as libc::c_int as u8_0, &mut D_801333E0,
                                   &mut D_801333E0, &mut D_801333E8);
        }
    } else if flags == (0 as libc::c_int) << 3 as libc::c_int {
        // collider->colType == COLTYPE_METAL
        EffectSsHitMark_SpawnFixedScale(globalCtx,
                                        EFFECT_HITMARK_METAL as libc::c_int,
                                        hitPos);
        if (*collider).actor.is_null() {
            CollisionCheck_SpawnShieldParticlesMetal(globalCtx, hitPos);
        } else {
            CollisionCheck_SpawnShieldParticlesMetalSound(globalCtx, hitPos,
                                                          &mut (*(*collider).actor).projectedPos);
        }
    } else if flags == (1 as libc::c_int) << 3 as libc::c_int {
        EffectSsHitMark_SpawnFixedScale(globalCtx,
                                        EFFECT_HITMARK_WHITE as libc::c_int,
                                        hitPos);
        if (*collider).actor.is_null() {
            Audio_PlaySoundGeneral(0x1806 as libc::c_int as u16_0,
                                   &mut D_801333D4, 4 as libc::c_int as u8_0,
                                   &mut D_801333E0, &mut D_801333E0,
                                   &mut D_801333E8);
        } else {
            Audio_PlaySoundGeneral(0x1806 as libc::c_int as u16_0,
                                   &mut (*(*collider).actor).projectedPos,
                                   4 as libc::c_int as u8_0, &mut D_801333E0,
                                   &mut D_801333E0, &mut D_801333E8);
        }
    } else if flags == (2 as libc::c_int) << 3 as libc::c_int {
        EffectSsHitMark_SpawnFixedScale(globalCtx,
                                        EFFECT_HITMARK_DUST as libc::c_int,
                                        hitPos);
        if (*collider).actor.is_null() {
            Audio_PlaySoundGeneral(0x1837 as libc::c_int as u16_0,
                                   &mut D_801333D4, 4 as libc::c_int as u8_0,
                                   &mut D_801333E0, &mut D_801333E0,
                                   &mut D_801333E8);
        } else {
            Audio_PlaySoundGeneral(0x1837 as libc::c_int as u16_0,
                                   &mut (*(*collider).actor).projectedPos,
                                   4 as libc::c_int as u8_0, &mut D_801333E0,
                                   &mut D_801333E0, &mut D_801333E8);
        }
    };
}
/* *
 * Plays a hit sound effect for AT colliders attached to Player based on the AC element's elemType.
 */
#[no_mangle]
pub unsafe extern "C" fn CollisionCheck_SwordHitAudio(mut at: *mut Collider,
                                                      mut acInfo:
                                                          *mut ColliderInfo)
 -> s32 {
    if !(*at).actor.is_null() &&
           (*(*at).actor).category as libc::c_int ==
               ACTORCAT_PLAYER as libc::c_int {
        if (*acInfo).elemType as libc::c_int == ELEMTYPE_UNK0 as libc::c_int {
            Audio_PlaySoundGeneral(0x1811 as libc::c_int as u16_0,
                                   &mut (*(*at).actor).projectedPos,
                                   4 as libc::c_int as u8_0, &mut D_801333E0,
                                   &mut D_801333E0, &mut D_801333E8);
        } else if (*acInfo).elemType as libc::c_int ==
                      ELEMTYPE_UNK1 as libc::c_int {
            Audio_PlaySoundGeneral(0x1824 as libc::c_int as u16_0,
                                   &mut (*(*at).actor).projectedPos,
                                   4 as libc::c_int as u8_0, &mut D_801333E0,
                                   &mut D_801333E0, &mut D_801333E8);
        } else if (*acInfo).elemType as libc::c_int ==
                      ELEMTYPE_UNK2 as libc::c_int {
            Audio_PlaySoundGeneral((0x800 as libc::c_int -
                                        0x800 as libc::c_int) as u16_0,
                                   &mut (*(*at).actor).projectedPos,
                                   4 as libc::c_int as u8_0, &mut D_801333E0,
                                   &mut D_801333E0, &mut D_801333E8);
        } else if (*acInfo).elemType as libc::c_int ==
                      ELEMTYPE_UNK3 as libc::c_int {
            Audio_PlaySoundGeneral((0x800 as libc::c_int -
                                        0x800 as libc::c_int) as u16_0,
                                   &mut (*(*at).actor).projectedPos,
                                   4 as libc::c_int as u8_0, &mut D_801333E0,
                                   &mut D_801333E0, &mut D_801333E8);
        }
    }
    return 1 as libc::c_int;
}
static mut sBloodFuncs: [ColChkBloodFunc; 6] =
    unsafe {
        [Some(CollisionCheck_NoBlood as
                  unsafe extern "C" fn(_: *mut GlobalContext,
                                       _: *mut Collider, _: *mut Vec3f)
                      -> ()),
         Some(CollisionCheck_BlueBlood as
                  unsafe extern "C" fn(_: *mut GlobalContext,
                                       _: *mut Collider, _: *mut Vec3f)
                      -> ()),
         Some(CollisionCheck_GreenBlood as
                  unsafe extern "C" fn(_: *mut GlobalContext,
                                       _: *mut Collider, _: *mut Vec3f)
                      -> ()),
         Some(CollisionCheck_WaterBurst as
                  unsafe extern "C" fn(_: *mut GlobalContext,
                                       _: *mut Collider, _: *mut Vec3f)
                      -> ()),
         Some(CollisionCheck_RedBlood as
                  unsafe extern "C" fn(_: *mut GlobalContext,
                                       _: *mut Collider, _: *mut Vec3f)
                      -> ()),
         Some(CollisionCheck_RedBloodUnused as
                  unsafe extern "C" fn(_: *mut GlobalContext,
                                       _: *mut Collider, _: *mut Vec3f)
                      -> ())]
    };
static mut sHitInfo: [HitInfo; 14] =
    [{
         let mut init =
             HitInfo{blood: BLOOD_BLUE as libc::c_int as u8_0,
                     effect: HIT_WHITE as libc::c_int as u8_0,};
         init
     },
     {
         let mut init =
             HitInfo{blood: BLOOD_NONE as libc::c_int as u8_0,
                     effect: HIT_DUST as libc::c_int as u8_0,};
         init
     },
     {
         let mut init =
             HitInfo{blood: BLOOD_GREEN as libc::c_int as u8_0,
                     effect: HIT_DUST as libc::c_int as u8_0,};
         init
     },
     {
         let mut init =
             HitInfo{blood: BLOOD_NONE as libc::c_int as u8_0,
                     effect: HIT_WHITE as libc::c_int as u8_0,};
         init
     },
     {
         let mut init =
             HitInfo{blood: BLOOD_WATER as libc::c_int as u8_0,
                     effect: HIT_NONE as libc::c_int as u8_0,};
         init
     },
     {
         let mut init =
             HitInfo{blood: BLOOD_NONE as libc::c_int as u8_0,
                     effect: HIT_RED as libc::c_int as u8_0,};
         init
     },
     {
         let mut init =
             HitInfo{blood: BLOOD_GREEN as libc::c_int as u8_0,
                     effect: HIT_WHITE as libc::c_int as u8_0,};
         init
     },
     {
         let mut init =
             HitInfo{blood: BLOOD_RED as libc::c_int as u8_0,
                     effect: HIT_WHITE as libc::c_int as u8_0,};
         init
     },
     {
         let mut init =
             HitInfo{blood: BLOOD_BLUE as libc::c_int as u8_0,
                     effect: HIT_RED as libc::c_int as u8_0,};
         init
     },
     {
         let mut init =
             HitInfo{blood: BLOOD_NONE as libc::c_int as u8_0,
                     effect: HIT_SOLID as libc::c_int as u8_0,};
         init
     },
     {
         let mut init =
             HitInfo{blood: BLOOD_NONE as libc::c_int as u8_0,
                     effect: HIT_NONE as libc::c_int as u8_0,};
         init
     },
     {
         let mut init =
             HitInfo{blood: BLOOD_NONE as libc::c_int as u8_0,
                     effect: HIT_SOLID as libc::c_int as u8_0,};
         init
     },
     {
         let mut init =
             HitInfo{blood: BLOOD_NONE as libc::c_int as u8_0,
                     effect: HIT_SOLID as libc::c_int as u8_0,};
         init
     },
     {
         let mut init =
             HitInfo{blood: BLOOD_NONE as libc::c_int as u8_0,
                     effect: HIT_WOOD as libc::c_int as u8_0,};
         init
     }];
/* *
 * Handles hitmarks, blood, and sound effects for each AC collision, determined by the AC collider's colType
 */
#[no_mangle]
pub unsafe extern "C" fn CollisionCheck_HitEffects(mut globalCtx:
                                                       *mut GlobalContext,
                                                   mut at: *mut Collider,
                                                   mut atInfo:
                                                       *mut ColliderInfo,
                                                   mut ac: *mut Collider,
                                                   mut acInfo:
                                                       *mut ColliderInfo,
                                                   mut hitPos: *mut Vec3f) {
    if (*acInfo).bumperFlags as libc::c_int &
           (1 as libc::c_int) << 6 as libc::c_int != 0 {
        return
    }
    if (*atInfo).toucherFlags as libc::c_int &
           (1 as libc::c_int) << 5 as libc::c_int == 0 &&
           (*atInfo).toucherFlags as libc::c_int &
               (1 as libc::c_int) << 6 as libc::c_int != 0 {
        return
    }
    if !(*ac).actor.is_null() {
        sBloodFuncs[sHitInfo[(*ac).colType as usize].blood as
                        usize].expect("non-null function pointer")(globalCtx,
                                                                   ac,
                                                                   hitPos);
    }
    if !(*ac).actor.is_null() {
        if sHitInfo[(*ac).colType as usize].effect as libc::c_int ==
               HIT_SOLID as libc::c_int {
            CollisionCheck_HitSolid(globalCtx, atInfo, ac, hitPos);
        } else if sHitInfo[(*ac).colType as usize].effect as libc::c_int ==
                      HIT_WOOD as libc::c_int {
            if (*at).actor.is_null() {
                CollisionCheck_SpawnShieldParticles(globalCtx, hitPos);
                Audio_PlaySoundGeneral(0x1837 as libc::c_int as u16_0,
                                       &mut D_801333D4,
                                       4 as libc::c_int as u8_0,
                                       &mut D_801333E0, &mut D_801333E0,
                                       &mut D_801333E8);
            } else {
                CollisionCheck_SpawnShieldParticlesWood(globalCtx, hitPos,
                                                        &mut (*(*at).actor).projectedPos);
            }
        } else if sHitInfo[(*ac).colType as usize].effect as libc::c_int !=
                      HIT_NONE as libc::c_int {
            EffectSsHitMark_SpawnFixedScale(globalCtx,
                                            sHitInfo[(*ac).colType as
                                                         usize].effect as s32,
                                            hitPos);
            if (*acInfo).bumperFlags as libc::c_int &
                   (1 as libc::c_int) << 5 as libc::c_int == 0 {
                CollisionCheck_SwordHitAudio(at, acInfo);
            }
        }
    } else {
        EffectSsHitMark_SpawnFixedScale(globalCtx,
                                        EFFECT_HITMARK_WHITE as libc::c_int,
                                        hitPos);
        if (*ac).actor.is_null() {
            Audio_PlaySoundGeneral(0x1806 as libc::c_int as u16_0,
                                   &mut D_801333D4, 4 as libc::c_int as u8_0,
                                   &mut D_801333E0, &mut D_801333E0,
                                   &mut D_801333E8);
        } else {
            Audio_PlaySoundGeneral(0x1806 as libc::c_int as u16_0,
                                   &mut (*(*ac).actor).projectedPos,
                                   4 as libc::c_int as u8_0, &mut D_801333E0,
                                   &mut D_801333E0, &mut D_801333E8);
        }
    };
}
/* *
 * Sets the flags to indicate an attack bounced off an AC_HARD collider.
 */
#[no_mangle]
pub unsafe extern "C" fn CollisionCheck_SetBounce(mut at: *mut Collider,
                                                  mut ac: *mut Collider) {
    (*at).atFlags =
        ((*at).atFlags as libc::c_int |
             (1 as libc::c_int) << 2 as libc::c_int) as u8_0;
    (*ac).acFlags =
        ((*ac).acFlags as libc::c_int |
             (1 as libc::c_int) << 7 as libc::c_int) as u8_0;
}
/* *
 * Performs the AC collision between the AT element and AC element that collided.
 */
#[no_mangle]
pub unsafe extern "C" fn CollisionCheck_SetATvsAC(mut globalCtx:
                                                      *mut GlobalContext,
                                                  mut at: *mut Collider,
                                                  mut atInfo:
                                                      *mut ColliderInfo,
                                                  mut atPos: *mut Vec3f,
                                                  mut ac: *mut Collider,
                                                  mut acInfo:
                                                      *mut ColliderInfo,
                                                  mut acPos: *mut Vec3f,
                                                  mut hitPos: *mut Vec3f)
 -> s32 {
    if (*ac).acFlags as libc::c_int & (1 as libc::c_int) << 2 as libc::c_int
           != 0 && !(*at).actor.is_null() && !(*ac).actor.is_null() {
        CollisionCheck_SetBounce(at, ac);
    }
    if (*acInfo).bumperFlags as libc::c_int &
           (1 as libc::c_int) << 3 as libc::c_int == 0 {
        (*at).atFlags =
            ((*at).atFlags as libc::c_int |
                 (1 as libc::c_int) << 1 as libc::c_int) as u8_0;
        (*at).at = (*ac).actor;
        (*atInfo).atHit = ac;
        (*atInfo).atHitInfo = acInfo;
        (*atInfo).toucherFlags =
            ((*atInfo).toucherFlags as libc::c_int |
                 (1 as libc::c_int) << 1 as libc::c_int) as u8_0;
        if !(*at).actor.is_null() {
            (*(*at).actor).colChkInfo.atHitEffect = (*acInfo).bumper.effect
        }
    }
    (*ac).acFlags =
        ((*ac).acFlags as libc::c_int |
             (1 as libc::c_int) << 1 as libc::c_int) as u8_0;
    (*ac).ac = (*at).actor;
    (*acInfo).acHit = at;
    (*acInfo).acHitInfo = atInfo;
    (*acInfo).bumperFlags =
        ((*acInfo).bumperFlags as libc::c_int |
             (1 as libc::c_int) << 1 as libc::c_int) as u8_0;
    if !(*ac).actor.is_null() {
        (*(*ac).actor).colChkInfo.acHitEffect = (*atInfo).toucher.effect
    }
    (*acInfo).bumper.hitPos.x = (*hitPos).x as s16;
    (*acInfo).bumper.hitPos.y = (*hitPos).y as s16;
    (*acInfo).bumper.hitPos.z = (*hitPos).z as s16;
    if (*atInfo).toucherFlags as libc::c_int &
           (1 as libc::c_int) << 5 as libc::c_int == 0 &&
           (*ac).colType as libc::c_int != COLTYPE_METAL as libc::c_int &&
           (*ac).colType as libc::c_int != COLTYPE_WOOD as libc::c_int &&
           (*ac).colType as libc::c_int != COLTYPE_HARD as libc::c_int {
        (*acInfo).bumperFlags =
            ((*acInfo).bumperFlags as libc::c_int |
                 (1 as libc::c_int) << 7 as libc::c_int) as u8_0
    } else {
        CollisionCheck_HitEffects(globalCtx, at, atInfo, ac, acInfo, hitPos);
        (*atInfo).toucherFlags =
            ((*atInfo).toucherFlags as libc::c_int |
                 (1 as libc::c_int) << 6 as libc::c_int) as u8_0
    }
    return 1 as libc::c_int;
}
/* *
 * AC overlap check. Calculates the center of each collider element and the point of contact.
 */
#[no_mangle]
pub unsafe extern "C" fn CollisionCheck_AC_JntSphVsJntSph(mut globalCtx:
                                                              *mut GlobalContext,
                                                          mut colChkCtx:
                                                              *mut CollisionCheckContext,
                                                          mut colAT:
                                                              *mut Collider,
                                                          mut colAC:
                                                              *mut Collider) {
    let mut at: *mut ColliderJntSph = colAT as *mut ColliderJntSph;
    let mut atItem: *mut ColliderJntSphElement =
        0 as *mut ColliderJntSphElement;
    let mut ac: *mut ColliderJntSph = colAC as *mut ColliderJntSph;
    let mut acElem: *mut ColliderJntSphElement =
        0 as *mut ColliderJntSphElement;
    let mut overlapSize: f32_0 = 0.;
    let mut centerDist: f32_0 = 0.;
    if (*at).count > 0 as libc::c_int && !(*at).elements.is_null() &&
           (*ac).count > 0 as libc::c_int && !(*ac).elements.is_null() {
        atItem = (*at).elements;
        while atItem < (*at).elements.offset((*at).count as isize) {
            if !(CollisionCheck_SkipTouch(&mut (*atItem).info) ==
                     1 as libc::c_int) {
                acElem = (*ac).elements;
                while acElem < (*ac).elements.offset((*ac).count as isize) {
                    if !(CollisionCheck_SkipBump(&mut (*acElem).info) ==
                             1 as libc::c_int) {
                        if !(CollisionCheck_NoSharedFlags(&mut (*atItem).info,
                                                          &mut (*acElem).info)
                                 == 1 as libc::c_int) {
                            if Math3D_SphVsSphOverlapCenter(&mut (*atItem).dim.worldSphere,
                                                            &mut (*acElem).dim.worldSphere,
                                                            &mut overlapSize,
                                                            &mut centerDist)
                                   == 1 as libc::c_int {
                                let mut acToHit: f32_0 = 0.;
                                let mut hitPos: Vec3f =
                                    Vec3f{x: 0., y: 0., z: 0.,};
                                let mut atPos: Vec3f =
                                    Vec3f{x: 0., y: 0., z: 0.,};
                                let mut acPos: Vec3f =
                                    Vec3f{x: 0., y: 0., z: 0.,};
                                atPos.x =
                                    (*atItem).dim.worldSphere.center.x as
                                        f32_0;
                                atPos.y =
                                    (*atItem).dim.worldSphere.center.y as
                                        f32_0;
                                atPos.z =
                                    (*atItem).dim.worldSphere.center.z as
                                        f32_0;
                                acPos.x =
                                    (*acElem).dim.worldSphere.center.x as
                                        f32_0;
                                acPos.y =
                                    (*acElem).dim.worldSphere.center.y as
                                        f32_0;
                                acPos.z =
                                    (*acElem).dim.worldSphere.center.z as
                                        f32_0;
                                if !(fabsf(centerDist) < 0.008f32) {
                                    acToHit =
                                        (*acElem).dim.worldSphere.radius as
                                            libc::c_int as libc::c_float /
                                            centerDist;
                                    hitPos.x =
                                        (atPos.x - acPos.x) * acToHit +
                                            acPos.x;
                                    hitPos.y =
                                        (atPos.y - acPos.y) * acToHit +
                                            acPos.y;
                                    hitPos.z =
                                        (atPos.z - acPos.z) * acToHit +
                                            acPos.z
                                } else {
                                    Math_Vec3f_Copy(&mut hitPos, &mut atPos);
                                }
                                CollisionCheck_SetATvsAC(globalCtx,
                                                         &mut (*at).base,
                                                         &mut (*atItem).info,
                                                         &mut atPos,
                                                         &mut (*ac).base,
                                                         &mut (*acElem).info,
                                                         &mut acPos,
                                                         &mut hitPos);
                                if (*ac).base.ocFlags2 as libc::c_int &
                                       (1 as libc::c_int) << 6 as libc::c_int
                                       == 0 {
                                    return
                                }
                            }
                        }
                    }
                    acElem = acElem.offset(1)
                }
            }
            atItem = atItem.offset(1)
        }
    };
}
/* *
 * AC overlap check. Calculates the center of each collider element and the point of contact.
 */
#[no_mangle]
pub unsafe extern "C" fn CollisionCheck_AC_JntSphVsCyl(mut globalCtx:
                                                           *mut GlobalContext,
                                                       mut colChkCtx:
                                                           *mut CollisionCheckContext,
                                                       mut colAT:
                                                           *mut Collider,
                                                       mut colAC:
                                                           *mut Collider) {
    let mut at: *mut ColliderJntSph = colAT as *mut ColliderJntSph;
    let mut atItem: *mut ColliderJntSphElement =
        0 as *mut ColliderJntSphElement;
    let mut ac: *mut ColliderCylinder = colAC as *mut ColliderCylinder;
    let mut overlapSize: f32_0 = 0.;
    let mut centerDist: f32_0 = 0.;
    if (*at).count > 0 as libc::c_int && !(*at).elements.is_null() &&
           (*ac).dim.radius as libc::c_int > 0 as libc::c_int &&
           (*ac).dim.height as libc::c_int > 0 as libc::c_int {
        if CollisionCheck_SkipBump(&mut (*ac).info) == 1 as libc::c_int {
            return
        }
        atItem = (*at).elements;
        while atItem < (*at).elements.offset((*at).count as isize) {
            if !(CollisionCheck_SkipTouch(&mut (*atItem).info) ==
                     1 as libc::c_int) {
                if !(CollisionCheck_NoSharedFlags(&mut (*atItem).info,
                                                  &mut (*ac).info) ==
                         1 as libc::c_int) {
                    if Math3D_SphVsCylOverlapCenterDist(&mut (*atItem).dim.worldSphere,
                                                        &mut (*ac).dim,
                                                        &mut overlapSize,
                                                        &mut centerDist) != 0
                       {
                        let mut hitPos: Vec3f = Vec3f{x: 0., y: 0., z: 0.,};
                        let mut atPos: Vec3f = Vec3f{x: 0., y: 0., z: 0.,};
                        let mut acPos: Vec3f = Vec3f{x: 0., y: 0., z: 0.,};
                        let mut acToHit: f32_0 = 0.;
                        atPos.x = (*atItem).dim.worldSphere.center.x as f32_0;
                        atPos.y = (*atItem).dim.worldSphere.center.y as f32_0;
                        atPos.z = (*atItem).dim.worldSphere.center.z as f32_0;
                        acPos.x = (*ac).dim.pos.x as f32_0;
                        acPos.y = (*ac).dim.pos.y as f32_0;
                        acPos.z = (*ac).dim.pos.z as f32_0;
                        if !(fabsf(centerDist) < 0.008f32) {
                            acToHit =
                                (*ac).dim.radius as libc::c_int as
                                    libc::c_float / centerDist;
                            if acToHit <= 1.0f32 {
                                hitPos.x =
                                    (atPos.x - acPos.x) * acToHit + acPos.x;
                                hitPos.y =
                                    (atPos.y - acPos.y) * acToHit + acPos.y;
                                hitPos.z =
                                    (atPos.z - acPos.z) * acToHit + acPos.z
                            } else {
                                Math_Vec3f_Copy(&mut hitPos, &mut atPos);
                            }
                        } else { Math_Vec3f_Copy(&mut hitPos, &mut atPos); }
                        CollisionCheck_SetATvsAC(globalCtx, &mut (*at).base,
                                                 &mut (*atItem).info,
                                                 &mut atPos, &mut (*ac).base,
                                                 &mut (*ac).info, &mut acPos,
                                                 &mut hitPos);
                        return
                    }
                }
            }
            atItem = atItem.offset(1)
        }
    };
}
/* *
 * AC overlap check. Calculates the center of each collider element and the point of contact.
 */
#[no_mangle]
pub unsafe extern "C" fn CollisionCheck_AC_CylVsJntSph(mut globalCtx:
                                                           *mut GlobalContext,
                                                       mut colChkCtx:
                                                           *mut CollisionCheckContext,
                                                       mut colAT:
                                                           *mut Collider,
                                                       mut colAC:
                                                           *mut Collider) {
    let mut at: *mut ColliderCylinder = colAT as *mut ColliderCylinder;
    let mut ac: *mut ColliderJntSph = colAC as *mut ColliderJntSph;
    let mut overlapSize: f32_0 = 0.;
    let mut centerDist: f32_0 = 0.;
    let mut acElem: *mut ColliderJntSphElement =
        0 as *mut ColliderJntSphElement;
    if (*ac).count > 0 as libc::c_int && !(*ac).elements.is_null() &&
           (*at).dim.radius as libc::c_int > 0 as libc::c_int &&
           (*at).dim.height as libc::c_int > 0 as libc::c_int {
        if CollisionCheck_SkipTouch(&mut (*at).info) == 1 as libc::c_int {
            return
        }
        acElem = (*ac).elements;
        while acElem < (*ac).elements.offset((*ac).count as isize) {
            if !(CollisionCheck_SkipBump(&mut (*acElem).info) ==
                     1 as libc::c_int) {
                if !(CollisionCheck_NoSharedFlags(&mut (*at).info,
                                                  &mut (*acElem).info) ==
                         1 as libc::c_int) {
                    if Math3D_SphVsCylOverlapCenterDist(&mut (*acElem).dim.worldSphere,
                                                        &mut (*at).dim,
                                                        &mut overlapSize,
                                                        &mut centerDist) != 0
                       {
                        let mut hitPos: Vec3f = Vec3f{x: 0., y: 0., z: 0.,};
                        let mut atPos: Vec3f = Vec3f{x: 0., y: 0., z: 0.,};
                        let mut acPos: Vec3f = Vec3f{x: 0., y: 0., z: 0.,};
                        let mut acToHit: f32_0 = 0.;
                        atPos.x = (*at).dim.pos.x as f32_0;
                        atPos.y = (*at).dim.pos.y as f32_0;
                        atPos.z = (*at).dim.pos.z as f32_0;
                        acPos.x = (*acElem).dim.worldSphere.center.x as f32_0;
                        acPos.y = (*acElem).dim.worldSphere.center.y as f32_0;
                        acPos.z = (*acElem).dim.worldSphere.center.z as f32_0;
                        if !(fabsf(centerDist) < 0.008f32) {
                            acToHit =
                                (*acElem).dim.worldSphere.radius as
                                    libc::c_int as libc::c_float / centerDist;
                            if acToHit <= 1.0f32 {
                                hitPos.x =
                                    (atPos.x - acPos.x) * acToHit + acPos.x;
                                hitPos.y =
                                    (atPos.y - acPos.y) * acToHit + acPos.y;
                                hitPos.z =
                                    (atPos.z - acPos.z) * acToHit + acPos.z
                            } else {
                                Math_Vec3f_Copy(&mut hitPos, &mut atPos);
                            }
                        } else { Math_Vec3f_Copy(&mut hitPos, &mut atPos); }
                        CollisionCheck_SetATvsAC(globalCtx, &mut (*at).base,
                                                 &mut (*at).info, &mut atPos,
                                                 &mut (*ac).base,
                                                 &mut (*acElem).info,
                                                 &mut acPos, &mut hitPos);
                        if (*ac).base.ocFlags2 as libc::c_int &
                               (1 as libc::c_int) << 6 as libc::c_int == 0 {
                            break ;
                        }
                    }
                }
            }
            acElem = acElem.offset(1)
        }
    };
}
/* *
 * AC overlap check. Calculates the center of each collider element and the point of contact.
 */
#[no_mangle]
pub unsafe extern "C" fn CollisionCheck_AC_JntSphVsTris(mut globalCtx:
                                                            *mut GlobalContext,
                                                        mut colChkCtx:
                                                            *mut CollisionCheckContext,
                                                        mut colAT:
                                                            *mut Collider,
                                                        mut colAC:
                                                            *mut Collider) {
    let mut at: *mut ColliderJntSph = colAT as *mut ColliderJntSph;
    let mut atSph: *mut ColliderJntSphElement =
        0 as *mut ColliderJntSphElement;
    let mut ac: *mut ColliderTris = colAC as *mut ColliderTris;
    let mut acTri: *mut ColliderTrisElement = 0 as *mut ColliderTrisElement;
    let mut hitPos: Vec3f = Vec3f{x: 0., y: 0., z: 0.,};
    if (*at).count > 0 as libc::c_int && !(*at).elements.is_null() &&
           (*ac).count > 0 as libc::c_int && !(*ac).elements.is_null() {
        atSph = (*at).elements;
        while atSph < (*at).elements.offset((*at).count as isize) {
            if !(CollisionCheck_SkipTouch(&mut (*atSph).info) ==
                     1 as libc::c_int) {
                acTri = (*ac).elements;
                while acTri < (*ac).elements.offset((*ac).count as isize) {
                    if !(CollisionCheck_SkipBump(&mut (*acTri).info) ==
                             1 as libc::c_int) {
                        if !(CollisionCheck_NoSharedFlags(&mut (*atSph).info,
                                                          &mut (*acTri).info)
                                 == 1 as libc::c_int) {
                            if Math3D_TriVsSphIntersect(&mut (*atSph).dim.worldSphere,
                                                        &mut (*acTri).dim,
                                                        &mut hitPos) ==
                                   1 as libc::c_int {
                                let mut atPos: Vec3f =
                                    Vec3f{x: 0., y: 0., z: 0.,};
                                let mut acPos: Vec3f =
                                    Vec3f{x: 0., y: 0., z: 0.,};
                                atPos.x =
                                    (*atSph).dim.worldSphere.center.x as
                                        f32_0;
                                atPos.y =
                                    (*atSph).dim.worldSphere.center.y as
                                        f32_0;
                                atPos.z =
                                    (*atSph).dim.worldSphere.center.z as
                                        f32_0;
                                acPos.x =
                                    ((*acTri).dim.vtx[0 as libc::c_int as
                                                          usize].x +
                                         (*acTri).dim.vtx[1 as libc::c_int as
                                                              usize].x +
                                         (*acTri).dim.vtx[2 as libc::c_int as
                                                              usize].x) *
                                        (1.0f32 /
                                             3 as libc::c_int as
                                                 libc::c_float);
                                acPos.y =
                                    ((*acTri).dim.vtx[0 as libc::c_int as
                                                          usize].y +
                                         (*acTri).dim.vtx[1 as libc::c_int as
                                                              usize].y +
                                         (*acTri).dim.vtx[2 as libc::c_int as
                                                              usize].y) *
                                        (1.0f32 /
                                             3 as libc::c_int as
                                                 libc::c_float);
                                acPos.z =
                                    ((*acTri).dim.vtx[0 as libc::c_int as
                                                          usize].z +
                                         (*acTri).dim.vtx[1 as libc::c_int as
                                                              usize].z +
                                         (*acTri).dim.vtx[2 as libc::c_int as
                                                              usize].z) *
                                        (1.0f32 /
                                             3 as libc::c_int as
                                                 libc::c_float);
                                CollisionCheck_SetATvsAC(globalCtx,
                                                         &mut (*at).base,
                                                         &mut (*atSph).info,
                                                         &mut atPos,
                                                         &mut (*ac).base,
                                                         &mut (*acTri).info,
                                                         &mut acPos,
                                                         &mut hitPos);
                                return
                            }
                        }
                    }
                    acTri = acTri.offset(1)
                }
            }
            atSph = atSph.offset(1)
        }
    };
}
/* *
 * AC overlap check. Calculates the center of each collider element and the point of contact.
 */
#[no_mangle]
pub unsafe extern "C" fn CollisionCheck_AC_TrisVsJntSph(mut globalCtx:
                                                            *mut GlobalContext,
                                                        mut colChkCtx:
                                                            *mut CollisionCheckContext,
                                                        mut colAT:
                                                            *mut Collider,
                                                        mut colAC:
                                                            *mut Collider) {
    let mut at: *mut ColliderTris = colAT as *mut ColliderTris;
    let mut atItem: *mut ColliderTrisElement = 0 as *mut ColliderTrisElement;
    let mut ac: *mut ColliderJntSph = colAC as *mut ColliderJntSph;
    let mut acElem: *mut ColliderJntSphElement =
        0 as *mut ColliderJntSphElement;
    let mut hitPos: Vec3f = Vec3f{x: 0., y: 0., z: 0.,};
    if (*ac).count > 0 as libc::c_int && !(*ac).elements.is_null() &&
           (*at).count > 0 as libc::c_int && !(*at).elements.is_null() {
        acElem = (*ac).elements;
        while acElem < (*ac).elements.offset((*ac).count as isize) {
            if !(CollisionCheck_SkipBump(&mut (*acElem).info) ==
                     1 as libc::c_int) {
                atItem = (*at).elements;
                while atItem < (*at).elements.offset((*at).count as isize) {
                    if !(CollisionCheck_SkipTouch(&mut (*atItem).info) ==
                             1 as libc::c_int) {
                        if !(CollisionCheck_NoSharedFlags(&mut (*atItem).info,
                                                          &mut (*acElem).info)
                                 == 1 as libc::c_int) {
                            if Math3D_TriVsSphIntersect(&mut (*acElem).dim.worldSphere,
                                                        &mut (*atItem).dim,
                                                        &mut hitPos) ==
                                   1 as libc::c_int {
                                let mut atPos: Vec3f =
                                    Vec3f{x: 0., y: 0., z: 0.,};
                                let mut acPos: Vec3f =
                                    Vec3f{x: 0., y: 0., z: 0.,};
                                Math_Vec3s_ToVec3f(&mut acPos,
                                                   &mut (*acElem).dim.worldSphere.center);
                                atPos.x =
                                    ((*atItem).dim.vtx[0 as libc::c_int as
                                                           usize].x +
                                         (*atItem).dim.vtx[1 as libc::c_int as
                                                               usize].x +
                                         (*atItem).dim.vtx[2 as libc::c_int as
                                                               usize].x) *
                                        (1.0f32 /
                                             3 as libc::c_int as
                                                 libc::c_float);
                                atPos.y =
                                    ((*atItem).dim.vtx[0 as libc::c_int as
                                                           usize].y +
                                         (*atItem).dim.vtx[1 as libc::c_int as
                                                               usize].y +
                                         (*atItem).dim.vtx[2 as libc::c_int as
                                                               usize].y) *
                                        (1.0f32 /
                                             3 as libc::c_int as
                                                 libc::c_float);
                                atPos.z =
                                    ((*atItem).dim.vtx[0 as libc::c_int as
                                                           usize].z +
                                         (*atItem).dim.vtx[1 as libc::c_int as
                                                               usize].z +
                                         (*atItem).dim.vtx[2 as libc::c_int as
                                                               usize].z) *
                                        (1.0f32 /
                                             3 as libc::c_int as
                                                 libc::c_float);
                                CollisionCheck_SetATvsAC(globalCtx,
                                                         &mut (*at).base,
                                                         &mut (*atItem).info,
                                                         &mut atPos,
                                                         &mut (*ac).base,
                                                         &mut (*acElem).info,
                                                         &mut acPos,
                                                         &mut hitPos);
                                if (*ac).base.ocFlags2 as libc::c_int &
                                       (1 as libc::c_int) << 6 as libc::c_int
                                       == 0 {
                                    return
                                }
                            }
                        }
                    }
                    atItem = atItem.offset(1)
                }
            }
            acElem = acElem.offset(1)
        }
    };
}
/* *
 * AC overlap check. Calculates the center of each collider element and the point of contact.
 */
#[no_mangle]
pub unsafe extern "C" fn CollisionCheck_AC_JntSphVsQuad(mut globalCtx:
                                                            *mut GlobalContext,
                                                        mut colChkCtx:
                                                            *mut CollisionCheckContext,
                                                        mut colAT:
                                                            *mut Collider,
                                                        mut colAC:
                                                            *mut Collider) {
    static mut D_8015E230: TriNorm =
        TriNorm{vtx: [Vec3f{x: 0., y: 0., z: 0.,}; 3],
                plane:
                    Plane{normal: Vec3f{x: 0., y: 0., z: 0.,},
                          originDist: 0.,},};
    static mut D_8015E268: TriNorm =
        TriNorm{vtx: [Vec3f{x: 0., y: 0., z: 0.,}; 3],
                plane:
                    Plane{normal: Vec3f{x: 0., y: 0., z: 0.,},
                          originDist: 0.,},};
    let mut at: *mut ColliderJntSph = colAT as *mut ColliderJntSph;
    let mut ac: *mut ColliderQuad = colAC as *mut ColliderQuad;
    let mut hitPos: Vec3f = Vec3f{x: 0., y: 0., z: 0.,};
    let mut atItem: *mut ColliderJntSphElement =
        0 as *mut ColliderJntSphElement;
    if (*at).count > 0 as libc::c_int && !(*at).elements.is_null() {
        if CollisionCheck_SkipBump(&mut (*ac).info) == 1 as libc::c_int {
            return
        }
        Math3D_TriNorm(&mut D_8015E230,
                       &mut *(*ac).dim.quad.as_mut_ptr().offset(2 as
                                                                    libc::c_int
                                                                    as isize),
                       &mut *(*ac).dim.quad.as_mut_ptr().offset(3 as
                                                                    libc::c_int
                                                                    as isize),
                       &mut *(*ac).dim.quad.as_mut_ptr().offset(1 as
                                                                    libc::c_int
                                                                    as
                                                                    isize));
        Math3D_TriNorm(&mut D_8015E268,
                       &mut *(*ac).dim.quad.as_mut_ptr().offset(1 as
                                                                    libc::c_int
                                                                    as isize),
                       &mut *(*ac).dim.quad.as_mut_ptr().offset(0 as
                                                                    libc::c_int
                                                                    as isize),
                       &mut *(*ac).dim.quad.as_mut_ptr().offset(2 as
                                                                    libc::c_int
                                                                    as
                                                                    isize));
        atItem = (*at).elements;
        while atItem < (*at).elements.offset((*at).count as isize) {
            if !(CollisionCheck_SkipTouch(&mut (*atItem).info) ==
                     1 as libc::c_int) {
                if !(CollisionCheck_NoSharedFlags(&mut (*atItem).info,
                                                  &mut (*ac).info) ==
                         1 as libc::c_int) {
                    if Math3D_TriVsSphIntersect(&mut (*atItem).dim.worldSphere,
                                                &mut D_8015E230, &mut hitPos)
                           == 1 as libc::c_int ||
                           Math3D_TriVsSphIntersect(&mut (*atItem).dim.worldSphere,
                                                    &mut D_8015E268,
                                                    &mut hitPos) ==
                               1 as libc::c_int {
                        let mut atPos: Vec3f = Vec3f{x: 0., y: 0., z: 0.,};
                        let mut acPos: Vec3f = Vec3f{x: 0., y: 0., z: 0.,};
                        Math_Vec3s_ToVec3f(&mut atPos,
                                           &mut (*atItem).dim.worldSphere.center);
                        acPos.x =
                            ((*ac).dim.quad[0 as libc::c_int as usize].x +
                                 ((*ac).dim.quad[1 as libc::c_int as usize].x
                                      +
                                      ((*ac).dim.quad[3 as libc::c_int as
                                                          usize].x +
                                           (*ac).dim.quad[2 as libc::c_int as
                                                              usize].x))) /
                                4.0f32;
                        acPos.y =
                            ((*ac).dim.quad[0 as libc::c_int as usize].y +
                                 ((*ac).dim.quad[1 as libc::c_int as usize].y
                                      +
                                      ((*ac).dim.quad[3 as libc::c_int as
                                                          usize].y +
                                           (*ac).dim.quad[2 as libc::c_int as
                                                              usize].y))) /
                                4.0f32;
                        acPos.z =
                            ((*ac).dim.quad[0 as libc::c_int as usize].z +
                                 ((*ac).dim.quad[1 as libc::c_int as usize].z
                                      +
                                      ((*ac).dim.quad[3 as libc::c_int as
                                                          usize].z +
                                           (*ac).dim.quad[2 as libc::c_int as
                                                              usize].z))) /
                                4.0f32;
                        CollisionCheck_SetATvsAC(globalCtx, &mut (*at).base,
                                                 &mut (*atItem).info,
                                                 &mut atPos, &mut (*ac).base,
                                                 &mut (*ac).info, &mut acPos,
                                                 &mut hitPos);
                        return
                    }
                }
            }
            atItem = atItem.offset(1)
        }
    };
}
/* *
 * AC overlap check. Calculates the center of each collider element and the point of contact.
 */
#[no_mangle]
pub unsafe extern "C" fn CollisionCheck_AC_QuadVsJntSph(mut globalCtx:
                                                            *mut GlobalContext,
                                                        mut colChkCtx:
                                                            *mut CollisionCheckContext,
                                                        mut colAT:
                                                            *mut Collider,
                                                        mut colAC:
                                                            *mut Collider) {
    static mut D_8015E2A0: TriNorm =
        TriNorm{vtx: [Vec3f{x: 0., y: 0., z: 0.,}; 3],
                plane:
                    Plane{normal: Vec3f{x: 0., y: 0., z: 0.,},
                          originDist: 0.,},};
    static mut D_8015E2D8: TriNorm =
        TriNorm{vtx: [Vec3f{x: 0., y: 0., z: 0.,}; 3],
                plane:
                    Plane{normal: Vec3f{x: 0., y: 0., z: 0.,},
                          originDist: 0.,},};
    let mut ac: *mut ColliderJntSph = colAC as *mut ColliderJntSph;
    let mut hitPos: Vec3f = Vec3f{x: 0., y: 0., z: 0.,};
    let mut at: *mut ColliderQuad = colAT as *mut ColliderQuad;
    let mut acElem: *mut ColliderJntSphElement =
        0 as *mut ColliderJntSphElement;
    if (*ac).count > 0 as libc::c_int && !(*ac).elements.is_null() {
        if CollisionCheck_SkipTouch(&mut (*at).info) == 1 as libc::c_int {
            return
        }
        Math3D_TriNorm(&mut D_8015E2A0,
                       &mut *(*at).dim.quad.as_mut_ptr().offset(2 as
                                                                    libc::c_int
                                                                    as isize),
                       &mut *(*at).dim.quad.as_mut_ptr().offset(3 as
                                                                    libc::c_int
                                                                    as isize),
                       &mut *(*at).dim.quad.as_mut_ptr().offset(1 as
                                                                    libc::c_int
                                                                    as
                                                                    isize));
        Math3D_TriNorm(&mut D_8015E2D8,
                       &mut *(*at).dim.quad.as_mut_ptr().offset(2 as
                                                                    libc::c_int
                                                                    as isize),
                       &mut *(*at).dim.quad.as_mut_ptr().offset(1 as
                                                                    libc::c_int
                                                                    as isize),
                       &mut *(*at).dim.quad.as_mut_ptr().offset(0 as
                                                                    libc::c_int
                                                                    as
                                                                    isize));
        acElem = (*ac).elements;
        while acElem < (*ac).elements.offset((*ac).count as isize) {
            if !(CollisionCheck_SkipBump(&mut (*acElem).info) ==
                     1 as libc::c_int) {
                if !(CollisionCheck_NoSharedFlags(&mut (*at).info,
                                                  &mut (*acElem).info) ==
                         1 as libc::c_int) {
                    if Math3D_TriVsSphIntersect(&mut (*acElem).dim.worldSphere,
                                                &mut D_8015E2A0, &mut hitPos)
                           == 1 as libc::c_int ||
                           Math3D_TriVsSphIntersect(&mut (*acElem).dim.worldSphere,
                                                    &mut D_8015E2D8,
                                                    &mut hitPos) ==
                               1 as libc::c_int {
                        if Collider_QuadSetNearestAC(globalCtx, at,
                                                     &mut hitPos) != 0 {
                            let mut atPos: Vec3f =
                                Vec3f{x: 0., y: 0., z: 0.,};
                            let mut acPos: Vec3f =
                                Vec3f{x: 0., y: 0., z: 0.,};
                            acPos.x =
                                (*acElem).dim.worldSphere.center.x as f32_0;
                            acPos.y =
                                (*acElem).dim.worldSphere.center.y as f32_0;
                            acPos.z =
                                (*acElem).dim.worldSphere.center.z as f32_0;
                            atPos.x =
                                ((*at).dim.quad[0 as libc::c_int as usize].x +
                                     ((*at).dim.quad[1 as libc::c_int as
                                                         usize].x +
                                          ((*at).dim.quad[3 as libc::c_int as
                                                              usize].x +
                                               (*at).dim.quad[2 as libc::c_int
                                                                  as
                                                                  usize].x)))
                                    / 4.0f32;
                            atPos.y =
                                ((*at).dim.quad[0 as libc::c_int as usize].y +
                                     ((*at).dim.quad[1 as libc::c_int as
                                                         usize].y +
                                          ((*at).dim.quad[3 as libc::c_int as
                                                              usize].y +
                                               (*at).dim.quad[2 as libc::c_int
                                                                  as
                                                                  usize].y)))
                                    / 4.0f32;
                            atPos.z =
                                ((*at).dim.quad[0 as libc::c_int as usize].z +
                                     ((*at).dim.quad[1 as libc::c_int as
                                                         usize].z +
                                          ((*at).dim.quad[3 as libc::c_int as
                                                              usize].z +
                                               (*at).dim.quad[2 as libc::c_int
                                                                  as
                                                                  usize].z)))
                                    / 4.0f32;
                            CollisionCheck_SetATvsAC(globalCtx,
                                                     &mut (*at).base,
                                                     &mut (*at).info,
                                                     &mut atPos,
                                                     &mut (*ac).base,
                                                     &mut (*acElem).info,
                                                     &mut acPos, &mut hitPos);
                            if (*ac).base.ocFlags2 as libc::c_int &
                                   (1 as libc::c_int) << 6 as libc::c_int == 0
                               {
                                return
                            }
                        }
                    }
                }
            }
            acElem = acElem.offset(1)
        }
    };
}
/* *
 * AC overlap check. Calculates the center of each collider element and the point of contact.
 */
#[no_mangle]
pub unsafe extern "C" fn CollisionCheck_AC_CylVsCyl(mut globalCtx:
                                                        *mut GlobalContext,
                                                    mut colChkCtx:
                                                        *mut CollisionCheckContext,
                                                    mut colAT: *mut Collider,
                                                    mut colAC:
                                                        *mut Collider) {
    let mut at: *mut ColliderCylinder = colAT as *mut ColliderCylinder;
    let mut ac: *mut ColliderCylinder = colAC as *mut ColliderCylinder;
    let mut deadSpace: f32_0 = 0.;
    let mut centerDistXZ: f32_0 = 0.;
    let mut hitPos: Vec3f = Vec3f{x: 0., y: 0., z: 0.,};
    if (*at).dim.radius as libc::c_int > 0 as libc::c_int &&
           (*at).dim.height as libc::c_int > 0 as libc::c_int &&
           (*ac).dim.radius as libc::c_int > 0 as libc::c_int &&
           (*ac).dim.height as libc::c_int > 0 as libc::c_int {
        if CollisionCheck_SkipBump(&mut (*ac).info) == 1 as libc::c_int {
            return
        }
        if CollisionCheck_SkipTouch(&mut (*at).info) == 1 as libc::c_int {
            return
        }
        if CollisionCheck_NoSharedFlags(&mut (*at).info, &mut (*ac).info) ==
               1 as libc::c_int {
            return
        }
        if Math3D_CylOutsideCylDist(&mut (*at).dim, &mut (*ac).dim,
                                    &mut deadSpace, &mut centerDistXZ) ==
               1 as libc::c_int {
            let mut atPos: Vec3f = Vec3f{x: 0., y: 0., z: 0.,};
            let mut acPos: Vec3f = Vec3f{x: 0., y: 0., z: 0.,};
            let mut acToHit: f32_0 = 0.;
            Math_Vec3s_ToVec3f(&mut atPos, &mut (*at).dim.pos);
            Math_Vec3s_ToVec3f(&mut acPos, &mut (*ac).dim.pos);
            if !(fabsf(centerDistXZ) < 0.008f32) {
                acToHit =
                    (*ac).dim.radius as libc::c_int as libc::c_float /
                        centerDistXZ;
                hitPos.y =
                    (*ac).dim.pos.y as f32_0 +
                        (*ac).dim.yShift as libc::c_int as libc::c_float +
                        (*ac).dim.height as libc::c_int as libc::c_float *
                            0.5f32;
                hitPos.x =
                    ((*at).dim.pos.x as f32_0 -
                         (*ac).dim.pos.x as libc::c_int as libc::c_float) *
                        acToHit +
                        (*ac).dim.pos.x as libc::c_int as libc::c_float;
                hitPos.z =
                    ((*at).dim.pos.z as f32_0 -
                         (*ac).dim.pos.z as libc::c_int as libc::c_float) *
                        acToHit +
                        (*ac).dim.pos.z as libc::c_int as libc::c_float
            } else { Math_Vec3s_ToVec3f(&mut hitPos, &mut (*ac).dim.pos); }
            CollisionCheck_SetATvsAC(globalCtx, &mut (*at).base,
                                     &mut (*at).info, &mut atPos,
                                     &mut (*ac).base, &mut (*ac).info,
                                     &mut acPos, &mut hitPos);
        }
    };
}
/* *
 * AC overlap check. Calculates the center of each collider element and the point of contact.
 */
#[no_mangle]
pub unsafe extern "C" fn CollisionCheck_AC_CylVsTris(mut globalCtx:
                                                         *mut GlobalContext,
                                                     mut colChkCtx:
                                                         *mut CollisionCheckContext,
                                                     mut colAT: *mut Collider,
                                                     mut colAC:
                                                         *mut Collider) {
    let mut at: *mut ColliderCylinder = colAT as *mut ColliderCylinder;
    let mut ac: *mut ColliderTris = colAC as *mut ColliderTris;
    let mut acElem: *mut ColliderTrisElement = 0 as *mut ColliderTrisElement;
    let mut hitPos: Vec3f = Vec3f{x: 0., y: 0., z: 0.,};
    if (*at).dim.radius as libc::c_int > 0 as libc::c_int &&
           (*at).dim.height as libc::c_int > 0 as libc::c_int &&
           (*ac).count > 0 as libc::c_int && !(*ac).elements.is_null() {
        if CollisionCheck_SkipTouch(&mut (*at).info) == 1 as libc::c_int {
            return
        }
        acElem = (*ac).elements;
        while acElem < (*ac).elements.offset((*ac).count as isize) {
            if !(CollisionCheck_SkipBump(&mut (*acElem).info) ==
                     1 as libc::c_int) {
                if !(CollisionCheck_NoSharedFlags(&mut (*at).info,
                                                  &mut (*acElem).info) ==
                         1 as libc::c_int) {
                    if Math3D_CylTriVsIntersect(&mut (*at).dim,
                                                &mut (*acElem).dim,
                                                &mut hitPos) ==
                           1 as libc::c_int {
                        let mut atpos: Vec3f = Vec3f{x: 0., y: 0., z: 0.,};
                        let mut acPos: Vec3f = Vec3f{x: 0., y: 0., z: 0.,};
                        Math_Vec3s_ToVec3f(&mut atpos, &mut (*at).dim.pos);
                        acPos.x =
                            ((*acElem).dim.vtx[0 as libc::c_int as usize].x +
                                 (*acElem).dim.vtx[1 as libc::c_int as
                                                       usize].x +
                                 (*acElem).dim.vtx[2 as libc::c_int as
                                                       usize].x) *
                                (1.0f32 / 3 as libc::c_int as libc::c_float);
                        acPos.y =
                            ((*acElem).dim.vtx[0 as libc::c_int as usize].y +
                                 (*acElem).dim.vtx[1 as libc::c_int as
                                                       usize].y +
                                 (*acElem).dim.vtx[2 as libc::c_int as
                                                       usize].y) *
                                (1.0f32 / 3 as libc::c_int as libc::c_float);
                        acPos.z =
                            ((*acElem).dim.vtx[0 as libc::c_int as usize].z +
                                 (*acElem).dim.vtx[1 as libc::c_int as
                                                       usize].z +
                                 (*acElem).dim.vtx[2 as libc::c_int as
                                                       usize].z) *
                                (1.0f32 / 3 as libc::c_int as libc::c_float);
                        CollisionCheck_SetATvsAC(globalCtx, &mut (*at).base,
                                                 &mut (*at).info, &mut atpos,
                                                 &mut (*ac).base,
                                                 &mut (*acElem).info,
                                                 &mut acPos, &mut hitPos);
                        return
                    }
                }
            }
            acElem = acElem.offset(1)
        }
    };
}
/* *
 * AC overlap check. Calculates the center of each collider element and the point of contact.
 */
#[no_mangle]
pub unsafe extern "C" fn CollisionCheck_AC_TrisVsCyl(mut globalCtx:
                                                         *mut GlobalContext,
                                                     mut colChkCtx:
                                                         *mut CollisionCheckContext,
                                                     mut colAT: *mut Collider,
                                                     mut colAC:
                                                         *mut Collider) {
    static mut D_8015E310: Vec3f = Vec3f{x: 0., y: 0., z: 0.,};
    let mut at: *mut ColliderTris = colAT as *mut ColliderTris;
    let mut atItem: *mut ColliderTrisElement = 0 as *mut ColliderTrisElement;
    let mut ac: *mut ColliderCylinder = colAC as *mut ColliderCylinder;
    let mut atPos: Vec3f = Vec3f{x: 0., y: 0., z: 0.,};
    let mut acPos: Vec3f = Vec3f{x: 0., y: 0., z: 0.,};
    if (*ac).dim.radius as libc::c_int > 0 as libc::c_int &&
           (*ac).dim.height as libc::c_int > 0 as libc::c_int &&
           (*at).count > 0 as libc::c_int && !(*at).elements.is_null() {
        if CollisionCheck_SkipBump(&mut (*ac).info) == 1 as libc::c_int {
            return
        }
        atItem = (*at).elements;
        while atItem < (*at).elements.offset((*at).count as isize) {
            if !(CollisionCheck_SkipTouch(&mut (*atItem).info) ==
                     1 as libc::c_int) {
                if !(CollisionCheck_NoSharedFlags(&mut (*atItem).info,
                                                  &mut (*ac).info) ==
                         1 as libc::c_int) {
                    if Math3D_CylTriVsIntersect(&mut (*ac).dim,
                                                &mut (*atItem).dim,
                                                &mut D_8015E310) ==
                           1 as libc::c_int {
                        atPos.x =
                            ((*atItem).dim.vtx[0 as libc::c_int as usize].x +
                                 (*atItem).dim.vtx[1 as libc::c_int as
                                                       usize].x +
                                 (*atItem).dim.vtx[2 as libc::c_int as
                                                       usize].x) *
                                (1.0f32 / 3 as libc::c_int as libc::c_float);
                        atPos.y =
                            ((*atItem).dim.vtx[0 as libc::c_int as usize].y +
                                 (*atItem).dim.vtx[1 as libc::c_int as
                                                       usize].y +
                                 (*atItem).dim.vtx[2 as libc::c_int as
                                                       usize].y) *
                                (1.0f32 / 3 as libc::c_int as libc::c_float);
                        atPos.z =
                            ((*atItem).dim.vtx[0 as libc::c_int as usize].z +
                                 (*atItem).dim.vtx[1 as libc::c_int as
                                                       usize].z +
                                 (*atItem).dim.vtx[2 as libc::c_int as
                                                       usize].z) *
                                (1.0f32 / 3 as libc::c_int as libc::c_float);
                        Math_Vec3s_ToVec3f(&mut acPos, &mut (*ac).dim.pos);
                        CollisionCheck_SetATvsAC(globalCtx, &mut (*at).base,
                                                 &mut (*atItem).info,
                                                 &mut atPos, &mut (*ac).base,
                                                 &mut (*ac).info, &mut acPos,
                                                 &mut D_8015E310);
                        return
                    }
                }
            }
            atItem = atItem.offset(1)
        }
    };
}
/* *
 * AC overlap check. Calculates the center of each collider element and the point of contact.
 */
#[no_mangle]
pub unsafe extern "C" fn CollisionCheck_AC_CylVsQuad(mut globalCtx:
                                                         *mut GlobalContext,
                                                     mut colChkCtx:
                                                         *mut CollisionCheckContext,
                                                     mut colAT: *mut Collider,
                                                     mut colAC:
                                                         *mut Collider) {
    static mut D_8015E320: TriNorm =
        TriNorm{vtx: [Vec3f{x: 0., y: 0., z: 0.,}; 3],
                plane:
                    Plane{normal: Vec3f{x: 0., y: 0., z: 0.,},
                          originDist: 0.,},};
    static mut D_8015E358: TriNorm =
        TriNorm{vtx: [Vec3f{x: 0., y: 0., z: 0.,}; 3],
                plane:
                    Plane{normal: Vec3f{x: 0., y: 0., z: 0.,},
                          originDist: 0.,},};
    static mut D_8015E390: Vec3f = Vec3f{x: 0., y: 0., z: 0.,};
    let mut at: *mut ColliderCylinder = colAT as *mut ColliderCylinder;
    let mut ac: *mut ColliderQuad = colAC as *mut ColliderQuad;
    if (*at).dim.height as libc::c_int > 0 as libc::c_int &&
           (*at).dim.radius as libc::c_int > 0 as libc::c_int {
        if CollisionCheck_SkipTouch(&mut (*at).info) == 1 as libc::c_int ||
               CollisionCheck_SkipBump(&mut (*ac).info) == 1 as libc::c_int {
            return
        }
        if CollisionCheck_NoSharedFlags(&mut (*at).info, &mut (*ac).info) ==
               1 as libc::c_int {
            return
        }
        Math3D_TriNorm(&mut D_8015E320,
                       &mut *(*ac).dim.quad.as_mut_ptr().offset(2 as
                                                                    libc::c_int
                                                                    as isize),
                       &mut *(*ac).dim.quad.as_mut_ptr().offset(3 as
                                                                    libc::c_int
                                                                    as isize),
                       &mut *(*ac).dim.quad.as_mut_ptr().offset(1 as
                                                                    libc::c_int
                                                                    as
                                                                    isize));
        Math3D_TriNorm(&mut D_8015E358,
                       &mut *(*ac).dim.quad.as_mut_ptr().offset(1 as
                                                                    libc::c_int
                                                                    as isize),
                       &mut *(*ac).dim.quad.as_mut_ptr().offset(0 as
                                                                    libc::c_int
                                                                    as isize),
                       &mut *(*ac).dim.quad.as_mut_ptr().offset(2 as
                                                                    libc::c_int
                                                                    as
                                                                    isize));
        if Math3D_CylTriVsIntersect(&mut (*at).dim, &mut D_8015E320,
                                    &mut D_8015E390) == 1 as libc::c_int {
            let mut atPos1: Vec3f = Vec3f{x: 0., y: 0., z: 0.,};
            let mut acPos1: Vec3f = Vec3f{x: 0., y: 0., z: 0.,};
            Math_Vec3s_ToVec3f(&mut atPos1, &mut (*at).dim.pos);
            acPos1.x =
                ((*ac).dim.quad[0 as libc::c_int as usize].x +
                     ((*ac).dim.quad[1 as libc::c_int as usize].x +
                          ((*ac).dim.quad[3 as libc::c_int as usize].x +
                               (*ac).dim.quad[2 as libc::c_int as usize].x)))
                    / 4.0f32;
            acPos1.y =
                ((*ac).dim.quad[0 as libc::c_int as usize].y +
                     ((*ac).dim.quad[1 as libc::c_int as usize].y +
                          ((*ac).dim.quad[3 as libc::c_int as usize].y +
                               (*ac).dim.quad[2 as libc::c_int as usize].y)))
                    / 4.0f32;
            acPos1.z =
                ((*ac).dim.quad[0 as libc::c_int as usize].z +
                     ((*ac).dim.quad[1 as libc::c_int as usize].z +
                          ((*ac).dim.quad[3 as libc::c_int as usize].z +
                               (*ac).dim.quad[2 as libc::c_int as usize].z)))
                    / 4.0f32;
            CollisionCheck_SetATvsAC(globalCtx, &mut (*at).base,
                                     &mut (*at).info, &mut atPos1,
                                     &mut (*ac).base, &mut (*ac).info,
                                     &mut acPos1, &mut D_8015E390);
        } else if Math3D_CylTriVsIntersect(&mut (*at).dim, &mut D_8015E358,
                                           &mut D_8015E390) ==
                      1 as libc::c_int {
            let mut atPos2: Vec3f = Vec3f{x: 0., y: 0., z: 0.,};
            let mut acPos2: Vec3f = Vec3f{x: 0., y: 0., z: 0.,};
            Math_Vec3s_ToVec3f(&mut atPos2, &mut (*at).dim.pos);
            acPos2.x =
                ((*ac).dim.quad[0 as libc::c_int as usize].x +
                     ((*ac).dim.quad[1 as libc::c_int as usize].x +
                          ((*ac).dim.quad[3 as libc::c_int as usize].x +
                               (*ac).dim.quad[2 as libc::c_int as usize].x)))
                    / 4.0f32;
            acPos2.y =
                ((*ac).dim.quad[0 as libc::c_int as usize].y +
                     ((*ac).dim.quad[1 as libc::c_int as usize].y +
                          ((*ac).dim.quad[3 as libc::c_int as usize].y +
                               (*ac).dim.quad[2 as libc::c_int as usize].y)))
                    / 4.0f32;
            acPos2.z =
                ((*ac).dim.quad[0 as libc::c_int as usize].z +
                     ((*ac).dim.quad[1 as libc::c_int as usize].z +
                          ((*ac).dim.quad[3 as libc::c_int as usize].z +
                               (*ac).dim.quad[2 as libc::c_int as usize].z)))
                    / 4.0f32;
            CollisionCheck_SetATvsAC(globalCtx, &mut (*at).base,
                                     &mut (*at).info, &mut atPos2,
                                     &mut (*ac).base, &mut (*ac).info,
                                     &mut acPos2, &mut D_8015E390);
        }
    };
}
/* *
 * AC overlap check. Calculates the center of each collider element and the point of contact.
 */
#[no_mangle]
pub unsafe extern "C" fn CollisionCheck_AC_QuadVsCyl(mut globalCtx:
                                                         *mut GlobalContext,
                                                     mut colChkCtx:
                                                         *mut CollisionCheckContext,
                                                     mut colAT: *mut Collider,
                                                     mut colAC:
                                                         *mut Collider) {
    static mut D_8015E3A0: TriNorm =
        TriNorm{vtx: [Vec3f{x: 0., y: 0., z: 0.,}; 3],
                plane:
                    Plane{normal: Vec3f{x: 0., y: 0., z: 0.,},
                          originDist: 0.,},};
    static mut D_8015E3D8: TriNorm =
        TriNorm{vtx: [Vec3f{x: 0., y: 0., z: 0.,}; 3],
                plane:
                    Plane{normal: Vec3f{x: 0., y: 0., z: 0.,},
                          originDist: 0.,},};
    static mut D_8015E410: Vec3f = Vec3f{x: 0., y: 0., z: 0.,};
    let mut at: *mut ColliderQuad = colAT as *mut ColliderQuad;
    let mut ac: *mut ColliderCylinder = colAC as *mut ColliderCylinder;
    if (*ac).dim.height as libc::c_int > 0 as libc::c_int &&
           (*ac).dim.radius as libc::c_int > 0 as libc::c_int {
        if CollisionCheck_SkipBump(&mut (*ac).info) == 1 as libc::c_int ||
               CollisionCheck_SkipTouch(&mut (*at).info) == 1 as libc::c_int {
            return
        }
        if CollisionCheck_NoSharedFlags(&mut (*at).info, &mut (*ac).info) ==
               1 as libc::c_int {
            return
        }
        Math3D_TriNorm(&mut D_8015E3A0,
                       &mut *(*at).dim.quad.as_mut_ptr().offset(2 as
                                                                    libc::c_int
                                                                    as isize),
                       &mut *(*at).dim.quad.as_mut_ptr().offset(3 as
                                                                    libc::c_int
                                                                    as isize),
                       &mut *(*at).dim.quad.as_mut_ptr().offset(1 as
                                                                    libc::c_int
                                                                    as
                                                                    isize));
        Math3D_TriNorm(&mut D_8015E3D8,
                       &mut *(*at).dim.quad.as_mut_ptr().offset(2 as
                                                                    libc::c_int
                                                                    as isize),
                       &mut *(*at).dim.quad.as_mut_ptr().offset(1 as
                                                                    libc::c_int
                                                                    as isize),
                       &mut *(*at).dim.quad.as_mut_ptr().offset(0 as
                                                                    libc::c_int
                                                                    as
                                                                    isize));
        if Math3D_CylTriVsIntersect(&mut (*ac).dim, &mut D_8015E3A0,
                                    &mut D_8015E410) == 1 as libc::c_int {
            if Collider_QuadSetNearestAC(globalCtx, at, &mut D_8015E410) != 0
               {
                let mut atPos1: Vec3f = Vec3f{x: 0., y: 0., z: 0.,};
                let mut acPos1: Vec3f = Vec3f{x: 0., y: 0., z: 0.,};
                atPos1.x =
                    ((*at).dim.quad[0 as libc::c_int as usize].x +
                         ((*at).dim.quad[1 as libc::c_int as usize].x +
                              ((*at).dim.quad[3 as libc::c_int as usize].x +
                                   (*at).dim.quad[2 as libc::c_int as
                                                      usize].x))) / 4.0f32;
                atPos1.y =
                    ((*at).dim.quad[0 as libc::c_int as usize].y +
                         ((*at).dim.quad[1 as libc::c_int as usize].y +
                              ((*at).dim.quad[3 as libc::c_int as usize].y +
                                   (*at).dim.quad[2 as libc::c_int as
                                                      usize].y))) / 4.0f32;
                atPos1.z =
                    ((*at).dim.quad[0 as libc::c_int as usize].z +
                         ((*at).dim.quad[1 as libc::c_int as usize].z +
                              ((*at).dim.quad[3 as libc::c_int as usize].z +
                                   (*at).dim.quad[2 as libc::c_int as
                                                      usize].z))) / 4.0f32;
                Math_Vec3s_ToVec3f(&mut acPos1, &mut (*ac).dim.pos);
                CollisionCheck_SetATvsAC(globalCtx, &mut (*at).base,
                                         &mut (*at).info, &mut atPos1,
                                         &mut (*ac).base, &mut (*ac).info,
                                         &mut acPos1, &mut D_8015E410);
                return
            }
        }
        if Math3D_CylTriVsIntersect(&mut (*ac).dim, &mut D_8015E3D8,
                                    &mut D_8015E410) == 1 as libc::c_int {
            if Collider_QuadSetNearestAC(globalCtx, at, &mut D_8015E410) != 0
               {
                let mut atPos2: Vec3f = Vec3f{x: 0., y: 0., z: 0.,};
                let mut acPos2: Vec3f = Vec3f{x: 0., y: 0., z: 0.,};
                atPos2.x =
                    ((*at).dim.quad[0 as libc::c_int as usize].x +
                         ((*at).dim.quad[1 as libc::c_int as usize].x +
                              ((*at).dim.quad[3 as libc::c_int as usize].x +
                                   (*at).dim.quad[2 as libc::c_int as
                                                      usize].x))) / 4.0f32;
                atPos2.y =
                    ((*at).dim.quad[0 as libc::c_int as usize].y +
                         ((*at).dim.quad[1 as libc::c_int as usize].y +
                              ((*at).dim.quad[3 as libc::c_int as usize].y +
                                   (*at).dim.quad[2 as libc::c_int as
                                                      usize].y))) / 4.0f32;
                atPos2.z =
                    ((*at).dim.quad[0 as libc::c_int as usize].z +
                         ((*at).dim.quad[1 as libc::c_int as usize].z +
                              ((*at).dim.quad[3 as libc::c_int as usize].z +
                                   (*at).dim.quad[2 as libc::c_int as
                                                      usize].z))) / 4.0f32;
                Math_Vec3s_ToVec3f(&mut acPos2, &mut (*ac).dim.pos);
                CollisionCheck_SetATvsAC(globalCtx, &mut (*at).base,
                                         &mut (*at).info, &mut atPos2,
                                         &mut (*ac).base, &mut (*ac).info,
                                         &mut acPos2, &mut D_8015E410);
            }
        }
    };
}
/* *
 * AC overlap check. Calculates the center of each collider element and the point of contact.
 */
#[no_mangle]
pub unsafe extern "C" fn CollisionCheck_AC_TrisVsTris(mut globalCtx:
                                                          *mut GlobalContext,
                                                      mut colChkCtx:
                                                          *mut CollisionCheckContext,
                                                      mut colAT:
                                                          *mut Collider,
                                                      mut colAC:
                                                          *mut Collider) {
    static mut D_8015E420: Vec3f = Vec3f{x: 0., y: 0., z: 0.,};
    let mut at: *mut ColliderTris = colAT as *mut ColliderTris;
    let mut atItem: *mut ColliderTrisElement = 0 as *mut ColliderTrisElement;
    let mut ac: *mut ColliderTris = colAC as *mut ColliderTris;
    let mut acElem: *mut ColliderTrisElement = 0 as *mut ColliderTrisElement;
    if (*ac).count > 0 as libc::c_int && !(*ac).elements.is_null() &&
           (*at).count > 0 as libc::c_int && !(*at).elements.is_null() {
        acElem = (*ac).elements;
        while acElem < (*ac).elements.offset((*ac).count as isize) {
            if !(CollisionCheck_SkipBump(&mut (*acElem).info) ==
                     1 as libc::c_int) {
                atItem = (*at).elements;
                while atItem < (*at).elements.offset((*at).count as isize) {
                    if !(CollisionCheck_SkipTouch(&mut (*atItem).info) ==
                             1 as libc::c_int) {
                        if !(CollisionCheck_NoSharedFlags(&mut (*atItem).info,
                                                          &mut (*acElem).info)
                                 == 1 as libc::c_int) {
                            if Math3D_TriVsTriIntersect(&mut (*atItem).dim,
                                                        &mut (*acElem).dim,
                                                        &mut D_8015E420) ==
                                   1 as libc::c_int {
                                let mut atPos: Vec3f =
                                    Vec3f{x: 0., y: 0., z: 0.,};
                                let mut acPos: Vec3f =
                                    Vec3f{x: 0., y: 0., z: 0.,};
                                atPos.x =
                                    ((*atItem).dim.vtx[0 as libc::c_int as
                                                           usize].x +
                                         (*atItem).dim.vtx[1 as libc::c_int as
                                                               usize].x +
                                         (*atItem).dim.vtx[2 as libc::c_int as
                                                               usize].x) *
                                        (1.0f32 /
                                             3 as libc::c_int as
                                                 libc::c_float);
                                atPos.y =
                                    ((*atItem).dim.vtx[0 as libc::c_int as
                                                           usize].y +
                                         (*atItem).dim.vtx[1 as libc::c_int as
                                                               usize].y +
                                         (*atItem).dim.vtx[2 as libc::c_int as
                                                               usize].y) *
                                        (1.0f32 /
                                             3 as libc::c_int as
                                                 libc::c_float);
                                atPos.z =
                                    ((*atItem).dim.vtx[0 as libc::c_int as
                                                           usize].z +
                                         (*atItem).dim.vtx[1 as libc::c_int as
                                                               usize].z +
                                         (*atItem).dim.vtx[2 as libc::c_int as
                                                               usize].z) *
                                        (1.0f32 /
                                             3 as libc::c_int as
                                                 libc::c_float);
                                acPos.x =
                                    ((*acElem).dim.vtx[0 as libc::c_int as
                                                           usize].x +
                                         (*acElem).dim.vtx[1 as libc::c_int as
                                                               usize].x +
                                         (*acElem).dim.vtx[2 as libc::c_int as
                                                               usize].x) *
                                        (1.0f32 /
                                             3 as libc::c_int as
                                                 libc::c_float);
                                acPos.y =
                                    ((*acElem).dim.vtx[0 as libc::c_int as
                                                           usize].y +
                                         (*acElem).dim.vtx[1 as libc::c_int as
                                                               usize].y +
                                         (*acElem).dim.vtx[2 as libc::c_int as
                                                               usize].y) *
                                        (1.0f32 /
                                             3 as libc::c_int as
                                                 libc::c_float);
                                acPos.z =
                                    ((*acElem).dim.vtx[0 as libc::c_int as
                                                           usize].z +
                                         (*acElem).dim.vtx[1 as libc::c_int as
                                                               usize].z +
                                         (*acElem).dim.vtx[2 as libc::c_int as
                                                               usize].z) *
                                        (1.0f32 /
                                             3 as libc::c_int as
                                                 libc::c_float);
                                CollisionCheck_SetATvsAC(globalCtx,
                                                         &mut (*at).base,
                                                         &mut (*atItem).info,
                                                         &mut atPos,
                                                         &mut (*ac).base,
                                                         &mut (*acElem).info,
                                                         &mut acPos,
                                                         &mut D_8015E420);
                                return
                            }
                        }
                    }
                    atItem = atItem.offset(1)
                }
            }
            acElem = acElem.offset(1)
        }
    };
}
/* *
 * AC overlap check. Calculates the center of each collider element and the point of contact.
 */
#[no_mangle]
pub unsafe extern "C" fn CollisionCheck_AC_TrisVsQuad(mut globalCtx:
                                                          *mut GlobalContext,
                                                      mut colChkCtx:
                                                          *mut CollisionCheckContext,
                                                      mut colAT:
                                                          *mut Collider,
                                                      mut colAC:
                                                          *mut Collider) {
    static mut D_8015E430: Vec3f = Vec3f{x: 0., y: 0., z: 0.,};
    static mut D_8015E440: TriNorm =
        TriNorm{vtx: [Vec3f{x: 0., y: 0., z: 0.,}; 3],
                plane:
                    Plane{normal: Vec3f{x: 0., y: 0., z: 0.,},
                          originDist: 0.,},};
    static mut D_8015E478: TriNorm =
        TriNorm{vtx: [Vec3f{x: 0., y: 0., z: 0.,}; 3],
                plane:
                    Plane{normal: Vec3f{x: 0., y: 0., z: 0.,},
                          originDist: 0.,},};
    let mut at: *mut ColliderTris = colAT as *mut ColliderTris;
    let mut atItem: *mut ColliderTrisElement = 0 as *mut ColliderTrisElement;
    let mut ac: *mut ColliderQuad = colAC as *mut ColliderQuad;
    if (*at).count > 0 as libc::c_int && !(*at).elements.is_null() {
        if CollisionCheck_SkipBump(&mut (*ac).info) == 1 as libc::c_int {
            return
        }
        Math3D_TriNorm(&mut D_8015E440,
                       &mut *(*ac).dim.quad.as_mut_ptr().offset(2 as
                                                                    libc::c_int
                                                                    as isize),
                       &mut *(*ac).dim.quad.as_mut_ptr().offset(3 as
                                                                    libc::c_int
                                                                    as isize),
                       &mut *(*ac).dim.quad.as_mut_ptr().offset(1 as
                                                                    libc::c_int
                                                                    as
                                                                    isize));
        Math3D_TriNorm(&mut D_8015E478,
                       &mut *(*ac).dim.quad.as_mut_ptr().offset(1 as
                                                                    libc::c_int
                                                                    as isize),
                       &mut *(*ac).dim.quad.as_mut_ptr().offset(0 as
                                                                    libc::c_int
                                                                    as isize),
                       &mut *(*ac).dim.quad.as_mut_ptr().offset(2 as
                                                                    libc::c_int
                                                                    as
                                                                    isize));
        atItem = (*at).elements;
        while atItem < (*at).elements.offset((*at).count as isize) {
            if !(CollisionCheck_SkipTouch(&mut (*atItem).info) ==
                     1 as libc::c_int) {
                if !(CollisionCheck_NoSharedFlags(&mut (*atItem).info,
                                                  &mut (*ac).info) ==
                         1 as libc::c_int) {
                    if Math3D_TriVsTriIntersect(&mut D_8015E440,
                                                &mut (*atItem).dim,
                                                &mut D_8015E430) ==
                           1 as libc::c_int ||
                           Math3D_TriVsTriIntersect(&mut D_8015E478,
                                                    &mut (*atItem).dim,
                                                    &mut D_8015E430) ==
                               1 as libc::c_int {
                        let mut atPos: Vec3f = Vec3f{x: 0., y: 0., z: 0.,};
                        let mut acPos: Vec3f = Vec3f{x: 0., y: 0., z: 0.,};
                        atPos.x =
                            ((*atItem).dim.vtx[0 as libc::c_int as usize].x +
                                 (*atItem).dim.vtx[1 as libc::c_int as
                                                       usize].x +
                                 (*atItem).dim.vtx[2 as libc::c_int as
                                                       usize].x) *
                                (1.0f32 / 3 as libc::c_int as libc::c_float);
                        atPos.y =
                            ((*atItem).dim.vtx[0 as libc::c_int as usize].y +
                                 (*atItem).dim.vtx[1 as libc::c_int as
                                                       usize].y +
                                 (*atItem).dim.vtx[2 as libc::c_int as
                                                       usize].y) *
                                (1.0f32 / 3 as libc::c_int as libc::c_float);
                        atPos.z =
                            ((*atItem).dim.vtx[0 as libc::c_int as usize].z +
                                 (*atItem).dim.vtx[1 as libc::c_int as
                                                       usize].z +
                                 (*atItem).dim.vtx[2 as libc::c_int as
                                                       usize].z) *
                                (1.0f32 / 3 as libc::c_int as libc::c_float);
                        acPos.x =
                            ((*ac).dim.quad[0 as libc::c_int as usize].x +
                                 ((*ac).dim.quad[1 as libc::c_int as usize].x
                                      +
                                      ((*ac).dim.quad[3 as libc::c_int as
                                                          usize].x +
                                           (*ac).dim.quad[2 as libc::c_int as
                                                              usize].x))) /
                                4.0f32;
                        acPos.y =
                            ((*ac).dim.quad[0 as libc::c_int as usize].y +
                                 ((*ac).dim.quad[1 as libc::c_int as usize].y
                                      +
                                      ((*ac).dim.quad[3 as libc::c_int as
                                                          usize].y +
                                           (*ac).dim.quad[2 as libc::c_int as
                                                              usize].y))) /
                                4.0f32;
                        acPos.z =
                            ((*ac).dim.quad[0 as libc::c_int as usize].z +
                                 ((*ac).dim.quad[1 as libc::c_int as usize].z
                                      +
                                      ((*ac).dim.quad[3 as libc::c_int as
                                                          usize].z +
                                           (*ac).dim.quad[2 as libc::c_int as
                                                              usize].z))) /
                                4.0f32;
                        CollisionCheck_SetATvsAC(globalCtx, &mut (*at).base,
                                                 &mut (*atItem).info,
                                                 &mut atPos, &mut (*ac).base,
                                                 &mut (*ac).info, &mut acPos,
                                                 &mut D_8015E430);
                        return
                    }
                }
            }
            atItem = atItem.offset(1)
        }
    };
}
/* *
 * AC overlap check. Calculates the center of each collider element and the point of contact.
 */
#[no_mangle]
pub unsafe extern "C" fn CollisionCheck_AC_QuadVsTris(mut globalCtx:
                                                          *mut GlobalContext,
                                                      mut colChkCtx:
                                                          *mut CollisionCheckContext,
                                                      mut colAT:
                                                          *mut Collider,
                                                      mut colAC:
                                                          *mut Collider) {
    static mut D_8015E4B0: Vec3f = Vec3f{x: 0., y: 0., z: 0.,};
    static mut D_8015E4C0: TriNorm =
        TriNorm{vtx: [Vec3f{x: 0., y: 0., z: 0.,}; 3],
                plane:
                    Plane{normal: Vec3f{x: 0., y: 0., z: 0.,},
                          originDist: 0.,},};
    static mut D_8015E4F8: TriNorm =
        TriNorm{vtx: [Vec3f{x: 0., y: 0., z: 0.,}; 3],
                plane:
                    Plane{normal: Vec3f{x: 0., y: 0., z: 0.,},
                          originDist: 0.,},};
    let mut at: *mut ColliderQuad = colAT as *mut ColliderQuad;
    let mut ac: *mut ColliderTris = colAC as *mut ColliderTris;
    let mut acElem: *mut ColliderTrisElement = 0 as *mut ColliderTrisElement;
    if (*ac).count > 0 as libc::c_int && !(*ac).elements.is_null() {
        if CollisionCheck_SkipTouch(&mut (*at).info) == 1 as libc::c_int {
            return
        }
        Math3D_TriNorm(&mut D_8015E4C0,
                       &mut *(*at).dim.quad.as_mut_ptr().offset(2 as
                                                                    libc::c_int
                                                                    as isize),
                       &mut *(*at).dim.quad.as_mut_ptr().offset(3 as
                                                                    libc::c_int
                                                                    as isize),
                       &mut *(*at).dim.quad.as_mut_ptr().offset(1 as
                                                                    libc::c_int
                                                                    as
                                                                    isize));
        Math3D_TriNorm(&mut D_8015E4F8,
                       &mut *(*at).dim.quad.as_mut_ptr().offset(1 as
                                                                    libc::c_int
                                                                    as isize),
                       &mut *(*at).dim.quad.as_mut_ptr().offset(0 as
                                                                    libc::c_int
                                                                    as isize),
                       &mut *(*at).dim.quad.as_mut_ptr().offset(2 as
                                                                    libc::c_int
                                                                    as
                                                                    isize));
        acElem = (*ac).elements;
        while acElem < (*ac).elements.offset((*ac).count as isize) {
            if !(CollisionCheck_SkipBump(&mut (*acElem).info) ==
                     1 as libc::c_int) {
                if !(CollisionCheck_NoSharedFlags(&mut (*at).info,
                                                  &mut (*acElem).info) ==
                         1 as libc::c_int) {
                    if Math3D_TriVsTriIntersect(&mut D_8015E4C0,
                                                &mut (*acElem).dim,
                                                &mut D_8015E4B0) ==
                           1 as libc::c_int ||
                           Math3D_TriVsTriIntersect(&mut D_8015E4F8,
                                                    &mut (*acElem).dim,
                                                    &mut D_8015E4B0) ==
                               1 as libc::c_int {
                        if Collider_QuadSetNearestAC(globalCtx, at,
                                                     &mut D_8015E4B0) != 0 {
                            let mut atPos: Vec3f =
                                Vec3f{x: 0., y: 0., z: 0.,};
                            let mut acPos: Vec3f =
                                Vec3f{x: 0., y: 0., z: 0.,};
                            acPos.x =
                                ((*acElem).dim.vtx[0 as libc::c_int as
                                                       usize].x +
                                     (*acElem).dim.vtx[1 as libc::c_int as
                                                           usize].x +
                                     (*acElem).dim.vtx[2 as libc::c_int as
                                                           usize].x) *
                                    (1.0f32 /
                                         3 as libc::c_int as libc::c_float);
                            acPos.y =
                                ((*acElem).dim.vtx[0 as libc::c_int as
                                                       usize].y +
                                     (*acElem).dim.vtx[1 as libc::c_int as
                                                           usize].y +
                                     (*acElem).dim.vtx[2 as libc::c_int as
                                                           usize].y) *
                                    (1.0f32 /
                                         3 as libc::c_int as libc::c_float);
                            acPos.z =
                                ((*acElem).dim.vtx[0 as libc::c_int as
                                                       usize].z +
                                     (*acElem).dim.vtx[1 as libc::c_int as
                                                           usize].z +
                                     (*acElem).dim.vtx[2 as libc::c_int as
                                                           usize].z) *
                                    (1.0f32 /
                                         3 as libc::c_int as libc::c_float);
                            atPos.x =
                                ((*at).dim.quad[0 as libc::c_int as usize].x +
                                     ((*at).dim.quad[1 as libc::c_int as
                                                         usize].x +
                                          ((*at).dim.quad[3 as libc::c_int as
                                                              usize].x +
                                               (*at).dim.quad[2 as libc::c_int
                                                                  as
                                                                  usize].x)))
                                    / 4.0f32;
                            atPos.y =
                                ((*at).dim.quad[0 as libc::c_int as usize].y +
                                     ((*at).dim.quad[1 as libc::c_int as
                                                         usize].y +
                                          ((*at).dim.quad[3 as libc::c_int as
                                                              usize].y +
                                               (*at).dim.quad[2 as libc::c_int
                                                                  as
                                                                  usize].y)))
                                    / 4.0f32;
                            atPos.z =
                                ((*at).dim.quad[0 as libc::c_int as usize].z +
                                     ((*at).dim.quad[1 as libc::c_int as
                                                         usize].z +
                                          ((*at).dim.quad[3 as libc::c_int as
                                                              usize].z +
                                               (*at).dim.quad[2 as libc::c_int
                                                                  as
                                                                  usize].z)))
                                    / 4.0f32;
                            CollisionCheck_SetATvsAC(globalCtx,
                                                     &mut (*at).base,
                                                     &mut (*at).info,
                                                     &mut atPos,
                                                     &mut (*ac).base,
                                                     &mut (*acElem).info,
                                                     &mut acPos,
                                                     &mut D_8015E4B0);
                            return
                        }
                    }
                }
            }
            acElem = acElem.offset(1)
        }
    };
}
/* *
 * AC overlap check. Calculates the center of each collider element and the point of contact.
 */
#[no_mangle]
pub unsafe extern "C" fn CollisionCheck_AC_QuadVsQuad(mut globalCtx:
                                                          *mut GlobalContext,
                                                      mut colChkCtx:
                                                          *mut CollisionCheckContext,
                                                      mut colAT:
                                                          *mut Collider,
                                                      mut colAC:
                                                          *mut Collider) {
    static mut D_8015E530: [TriNorm; 2] =
        [TriNorm{vtx: [Vec3f{x: 0., y: 0., z: 0.,}; 3],
                 plane:
                     Plane{normal: Vec3f{x: 0., y: 0., z: 0.,},
                           originDist: 0.,},}; 2];
    static mut D_8015E598: Vec3f = Vec3f{x: 0., y: 0., z: 0.,};
    static mut D_8015E5A8: [TriNorm; 2] =
        [TriNorm{vtx: [Vec3f{x: 0., y: 0., z: 0.,}; 3],
                 plane:
                     Plane{normal: Vec3f{x: 0., y: 0., z: 0.,},
                           originDist: 0.,},}; 2];
    let mut at: *mut ColliderQuad = colAT as *mut ColliderQuad;
    let mut ac: *mut ColliderQuad = colAC as *mut ColliderQuad;
    let mut i: s32 = 0;
    let mut j: s32 = 0;
    if CollisionCheck_SkipTouch(&mut (*at).info) == 1 as libc::c_int {
        return
    }
    if CollisionCheck_SkipBump(&mut (*ac).info) == 1 as libc::c_int { return }
    if CollisionCheck_NoSharedFlags(&mut (*at).info, &mut (*ac).info) ==
           1 as libc::c_int {
        return
    }
    Math3D_TriNorm(&mut *D_8015E5A8.as_mut_ptr().offset(0 as libc::c_int as
                                                            isize),
                   &mut *(*at).dim.quad.as_mut_ptr().offset(2 as libc::c_int
                                                                as isize),
                   &mut *(*at).dim.quad.as_mut_ptr().offset(3 as libc::c_int
                                                                as isize),
                   &mut *(*at).dim.quad.as_mut_ptr().offset(1 as libc::c_int
                                                                as isize));
    Math3D_TriNorm(&mut *D_8015E5A8.as_mut_ptr().offset(1 as libc::c_int as
                                                            isize),
                   &mut *(*at).dim.quad.as_mut_ptr().offset(2 as libc::c_int
                                                                as isize),
                   &mut *(*at).dim.quad.as_mut_ptr().offset(1 as libc::c_int
                                                                as isize),
                   &mut *(*at).dim.quad.as_mut_ptr().offset(0 as libc::c_int
                                                                as isize));
    Math3D_TriNorm(&mut *D_8015E530.as_mut_ptr().offset(0 as libc::c_int as
                                                            isize),
                   &mut *(*ac).dim.quad.as_mut_ptr().offset(2 as libc::c_int
                                                                as isize),
                   &mut *(*ac).dim.quad.as_mut_ptr().offset(3 as libc::c_int
                                                                as isize),
                   &mut *(*ac).dim.quad.as_mut_ptr().offset(1 as libc::c_int
                                                                as isize));
    Math3D_TriNorm(&mut *D_8015E530.as_mut_ptr().offset(1 as libc::c_int as
                                                            isize),
                   &mut *(*ac).dim.quad.as_mut_ptr().offset(2 as libc::c_int
                                                                as isize),
                   &mut *(*ac).dim.quad.as_mut_ptr().offset(1 as libc::c_int
                                                                as isize),
                   &mut *(*ac).dim.quad.as_mut_ptr().offset(0 as libc::c_int
                                                                as isize));
    i = 0 as libc::c_int;
    while i < 2 as libc::c_int {
        j = 0 as libc::c_int;
        while j < 2 as libc::c_int {
            if Math3D_TriVsTriIntersect(&mut *D_8015E5A8.as_mut_ptr().offset(j
                                                                                 as
                                                                                 isize),
                                        &mut *D_8015E530.as_mut_ptr().offset(i
                                                                                 as
                                                                                 isize),
                                        &mut D_8015E598) == 1 as libc::c_int {
                if Collider_QuadSetNearestAC(globalCtx, at, &mut D_8015E598)
                       != 0 {
                    let mut atPos: Vec3f = Vec3f{x: 0., y: 0., z: 0.,};
                    let mut acPos: Vec3f = Vec3f{x: 0., y: 0., z: 0.,};
                    atPos.x =
                        ((*at).dim.quad[0 as libc::c_int as usize].x +
                             ((*at).dim.quad[1 as libc::c_int as usize].x +
                                  ((*at).dim.quad[3 as libc::c_int as usize].x
                                       +
                                       (*at).dim.quad[2 as libc::c_int as
                                                          usize].x))) /
                            4.0f32;
                    atPos.y =
                        ((*at).dim.quad[0 as libc::c_int as usize].y +
                             ((*at).dim.quad[1 as libc::c_int as usize].y +
                                  ((*at).dim.quad[3 as libc::c_int as usize].y
                                       +
                                       (*at).dim.quad[2 as libc::c_int as
                                                          usize].y))) /
                            4.0f32;
                    atPos.z =
                        ((*at).dim.quad[0 as libc::c_int as usize].z +
                             ((*at).dim.quad[1 as libc::c_int as usize].z +
                                  ((*at).dim.quad[3 as libc::c_int as usize].z
                                       +
                                       (*at).dim.quad[2 as libc::c_int as
                                                          usize].z))) /
                            4.0f32;
                    acPos.x =
                        ((*ac).dim.quad[0 as libc::c_int as usize].x +
                             ((*ac).dim.quad[1 as libc::c_int as usize].x +
                                  ((*ac).dim.quad[3 as libc::c_int as usize].x
                                       +
                                       (*ac).dim.quad[2 as libc::c_int as
                                                          usize].x))) /
                            4.0f32;
                    acPos.y =
                        ((*ac).dim.quad[0 as libc::c_int as usize].y +
                             ((*ac).dim.quad[1 as libc::c_int as usize].y +
                                  ((*ac).dim.quad[3 as libc::c_int as usize].y
                                       +
                                       (*ac).dim.quad[2 as libc::c_int as
                                                          usize].y))) /
                            4.0f32;
                    acPos.z =
                        ((*ac).dim.quad[0 as libc::c_int as usize].z +
                             ((*ac).dim.quad[1 as libc::c_int as usize].z +
                                  ((*ac).dim.quad[3 as libc::c_int as usize].z
                                       +
                                       (*ac).dim.quad[2 as libc::c_int as
                                                          usize].z))) /
                            4.0f32;
                    CollisionCheck_SetATvsAC(globalCtx, &mut (*at).base,
                                             &mut (*at).info, &mut atPos,
                                             &mut (*ac).base, &mut (*ac).info,
                                             &mut acPos, &mut D_8015E598);
                    return
                }
            }
            j += 1
        }
        i += 1
    };
}
/* *
 * Sets a ColliderJntSph's hit effects
 */
#[no_mangle]
pub unsafe extern "C" fn CollisionCheck_SetJntSphHitFX(mut globalCtx:
                                                           *mut GlobalContext,
                                                       mut colChkCtx:
                                                           *mut CollisionCheckContext,
                                                       mut collider:
                                                           *mut Collider) {
    let mut jntSph: *mut ColliderJntSph = collider as *mut ColliderJntSph;
    let mut element: *mut ColliderJntSphElement =
        0 as *mut ColliderJntSphElement;
    let mut hitPos: Vec3f = Vec3f{x: 0., y: 0., z: 0.,};
    element = (*jntSph).elements;
    while element < (*jntSph).elements.offset((*jntSph).count as isize) {
        if (*element).info.bumperFlags as libc::c_int &
               (1 as libc::c_int) << 7 as libc::c_int != 0 &&
               !(*element).info.acHitInfo.is_null() &&
               (*(*element).info.acHitInfo).toucherFlags as libc::c_int &
                   (1 as libc::c_int) << 6 as libc::c_int == 0 {
            Math_Vec3s_ToVec3f(&mut hitPos,
                               &mut (*element).info.bumper.hitPos);
            CollisionCheck_HitEffects(globalCtx, (*element).info.acHit,
                                      (*element).info.acHitInfo,
                                      &mut (*jntSph).base,
                                      &mut (*element).info, &mut hitPos);
            (*(*element).info.acHitInfo).toucherFlags =
                ((*(*element).info.acHitInfo).toucherFlags as libc::c_int |
                     (1 as libc::c_int) << 6 as libc::c_int) as u8_0;
            return
        }
        element = element.offset(1)
    };
}
/* *
 * Sets a ColliderCylinder's hit effects
 */
#[no_mangle]
pub unsafe extern "C" fn CollisionCheck_SetCylHitFX(mut globalCtx:
                                                        *mut GlobalContext,
                                                    mut colChkCtx:
                                                        *mut CollisionCheckContext,
                                                    mut collider:
                                                        *mut Collider) {
    let mut cylinder: *mut ColliderCylinder =
        collider as *mut ColliderCylinder;
    let mut hitPos: Vec3f = Vec3f{x: 0., y: 0., z: 0.,};
    if (*cylinder).info.bumperFlags as libc::c_int &
           (1 as libc::c_int) << 7 as libc::c_int != 0 &&
           !(*cylinder).info.acHitInfo.is_null() &&
           (*(*cylinder).info.acHitInfo).toucherFlags as libc::c_int &
               (1 as libc::c_int) << 6 as libc::c_int == 0 {
        Math_Vec3s_ToVec3f(&mut hitPos, &mut (*cylinder).info.bumper.hitPos);
        CollisionCheck_HitEffects(globalCtx, (*cylinder).info.acHit,
                                  (*cylinder).info.acHitInfo,
                                  &mut (*cylinder).base,
                                  &mut (*cylinder).info, &mut hitPos);
        (*(*cylinder).info.acHitInfo).toucherFlags =
            ((*(*cylinder).info.acHitInfo).toucherFlags as libc::c_int |
                 (1 as libc::c_int) << 6 as libc::c_int) as u8_0
    };
}
/* *
 * Sets a ColliderTris's hit effects
 */
#[no_mangle]
pub unsafe extern "C" fn CollisionCheck_SetTrisHitFX(mut globalCtx:
                                                         *mut GlobalContext,
                                                     mut colChkCtx:
                                                         *mut CollisionCheckContext,
                                                     mut collider:
                                                         *mut Collider) {
    let mut tris: *mut ColliderTris = collider as *mut ColliderTris;
    let mut element: *mut ColliderTrisElement = 0 as *mut ColliderTrisElement;
    let mut hitPos: Vec3f = Vec3f{x: 0., y: 0., z: 0.,};
    element = (*tris).elements;
    while element < (*tris).elements.offset((*tris).count as isize) {
        if (*element).info.bumperFlags as libc::c_int &
               (1 as libc::c_int) << 7 as libc::c_int != 0 &&
               !(*element).info.acHitInfo.is_null() &&
               (*(*element).info.acHitInfo).toucherFlags as libc::c_int &
                   (1 as libc::c_int) << 6 as libc::c_int == 0 {
            Math_Vec3s_ToVec3f(&mut hitPos,
                               &mut (*element).info.bumper.hitPos);
            CollisionCheck_HitEffects(globalCtx, (*element).info.acHit,
                                      (*element).info.acHitInfo,
                                      &mut (*tris).base, &mut (*element).info,
                                      &mut hitPos);
            (*(*element).info.acHitInfo).toucherFlags =
                ((*(*element).info.acHitInfo).toucherFlags as libc::c_int |
                     (1 as libc::c_int) << 6 as libc::c_int) as u8_0;
            return
        }
        element = element.offset(1)
    };
}
/* *
 * Sets a ColliderQuad's hit effects
 */
#[no_mangle]
pub unsafe extern "C" fn CollisionCheck_SetQuadHitFX(mut globalCtx:
                                                         *mut GlobalContext,
                                                     mut colChkCtx:
                                                         *mut CollisionCheckContext,
                                                     mut collider:
                                                         *mut Collider) {
    let mut quad: *mut ColliderQuad = collider as *mut ColliderQuad;
    let mut hitPos: Vec3f = Vec3f{x: 0., y: 0., z: 0.,};
    if (*quad).info.bumperFlags as libc::c_int &
           (1 as libc::c_int) << 7 as libc::c_int != 0 &&
           !(*quad).info.acHitInfo.is_null() &&
           (*(*quad).info.acHitInfo).toucherFlags as libc::c_int &
               (1 as libc::c_int) << 6 as libc::c_int == 0 {
        Math_Vec3s_ToVec3f(&mut hitPos, &mut (*quad).info.bumper.hitPos);
        CollisionCheck_HitEffects(globalCtx, (*quad).info.acHit,
                                  (*quad).info.acHitInfo, &mut (*quad).base,
                                  &mut (*quad).info, &mut hitPos);
        (*(*quad).info.acHitInfo).toucherFlags =
            ((*(*quad).info.acHitInfo).toucherFlags as libc::c_int |
                 (1 as libc::c_int) << 6 as libc::c_int) as u8_0
    };
}
static mut sColChkApplyFuncs: [ColChkApplyFunc; 4] =
    unsafe {
        [Some(CollisionCheck_SetJntSphHitFX as
                  unsafe extern "C" fn(_: *mut GlobalContext,
                                       _: *mut CollisionCheckContext,
                                       _: *mut Collider) -> ()),
         Some(CollisionCheck_SetCylHitFX as
                  unsafe extern "C" fn(_: *mut GlobalContext,
                                       _: *mut CollisionCheckContext,
                                       _: *mut Collider) -> ()),
         Some(CollisionCheck_SetTrisHitFX as
                  unsafe extern "C" fn(_: *mut GlobalContext,
                                       _: *mut CollisionCheckContext,
                                       _: *mut Collider) -> ()),
         Some(CollisionCheck_SetQuadHitFX as
                  unsafe extern "C" fn(_: *mut GlobalContext,
                                       _: *mut CollisionCheckContext,
                                       _: *mut Collider) -> ())]
    };
/* *
 * Handles hit effects for each AC collider that had an AC collision. Spawns hitmarks and plays sound effects.
 */
#[no_mangle]
pub unsafe extern "C" fn CollisionCheck_SetHitEffects(mut globalCtx:
                                                          *mut GlobalContext,
                                                      mut colChkCtx:
                                                          *mut CollisionCheckContext) {
    let mut col: *mut *mut Collider = 0 as *mut *mut Collider;
    col = (*colChkCtx).colAC.as_mut_ptr();
    while col <
              (*colChkCtx).colAC.as_mut_ptr().offset((*colChkCtx).colACCount
                                                         as isize) {
        let mut colAC: *mut Collider = *col;
        if !colAC.is_null() &&
               (*colAC).acFlags as libc::c_int &
                   (1 as libc::c_int) << 0 as libc::c_int != 0 {
            if !(!(*colAC).actor.is_null() &&
                     (*(*colAC).actor).update.is_none()) {
                sColChkApplyFuncs[(*colAC).shape as
                                      usize].expect("non-null function pointer")(globalCtx,
                                                                                 colChkCtx,
                                                                                 colAC);
            }
        }
        col = col.offset(1)
    };
}
static mut sACVsFuncs: [[ColChkVsFunc; 4]; 4] =
    unsafe {
        [[Some(CollisionCheck_AC_JntSphVsJntSph as
                   unsafe extern "C" fn(_: *mut GlobalContext,
                                        _: *mut CollisionCheckContext,
                                        _: *mut Collider, _: *mut Collider)
                       -> ()),
          Some(CollisionCheck_AC_JntSphVsCyl as
                   unsafe extern "C" fn(_: *mut GlobalContext,
                                        _: *mut CollisionCheckContext,
                                        _: *mut Collider, _: *mut Collider)
                       -> ()),
          Some(CollisionCheck_AC_JntSphVsTris as
                   unsafe extern "C" fn(_: *mut GlobalContext,
                                        _: *mut CollisionCheckContext,
                                        _: *mut Collider, _: *mut Collider)
                       -> ()),
          Some(CollisionCheck_AC_JntSphVsQuad as
                   unsafe extern "C" fn(_: *mut GlobalContext,
                                        _: *mut CollisionCheckContext,
                                        _: *mut Collider, _: *mut Collider)
                       -> ())],
         [Some(CollisionCheck_AC_CylVsJntSph as
                   unsafe extern "C" fn(_: *mut GlobalContext,
                                        _: *mut CollisionCheckContext,
                                        _: *mut Collider, _: *mut Collider)
                       -> ()),
          Some(CollisionCheck_AC_CylVsCyl as
                   unsafe extern "C" fn(_: *mut GlobalContext,
                                        _: *mut CollisionCheckContext,
                                        _: *mut Collider, _: *mut Collider)
                       -> ()),
          Some(CollisionCheck_AC_CylVsTris as
                   unsafe extern "C" fn(_: *mut GlobalContext,
                                        _: *mut CollisionCheckContext,
                                        _: *mut Collider, _: *mut Collider)
                       -> ()),
          Some(CollisionCheck_AC_CylVsQuad as
                   unsafe extern "C" fn(_: *mut GlobalContext,
                                        _: *mut CollisionCheckContext,
                                        _: *mut Collider, _: *mut Collider)
                       -> ())],
         [Some(CollisionCheck_AC_TrisVsJntSph as
                   unsafe extern "C" fn(_: *mut GlobalContext,
                                        _: *mut CollisionCheckContext,
                                        _: *mut Collider, _: *mut Collider)
                       -> ()),
          Some(CollisionCheck_AC_TrisVsCyl as
                   unsafe extern "C" fn(_: *mut GlobalContext,
                                        _: *mut CollisionCheckContext,
                                        _: *mut Collider, _: *mut Collider)
                       -> ()),
          Some(CollisionCheck_AC_TrisVsTris as
                   unsafe extern "C" fn(_: *mut GlobalContext,
                                        _: *mut CollisionCheckContext,
                                        _: *mut Collider, _: *mut Collider)
                       -> ()),
          Some(CollisionCheck_AC_TrisVsQuad as
                   unsafe extern "C" fn(_: *mut GlobalContext,
                                        _: *mut CollisionCheckContext,
                                        _: *mut Collider, _: *mut Collider)
                       -> ())],
         [Some(CollisionCheck_AC_QuadVsJntSph as
                   unsafe extern "C" fn(_: *mut GlobalContext,
                                        _: *mut CollisionCheckContext,
                                        _: *mut Collider, _: *mut Collider)
                       -> ()),
          Some(CollisionCheck_AC_QuadVsCyl as
                   unsafe extern "C" fn(_: *mut GlobalContext,
                                        _: *mut CollisionCheckContext,
                                        _: *mut Collider, _: *mut Collider)
                       -> ()),
          Some(CollisionCheck_AC_QuadVsTris as
                   unsafe extern "C" fn(_: *mut GlobalContext,
                                        _: *mut CollisionCheckContext,
                                        _: *mut Collider, _: *mut Collider)
                       -> ()),
          Some(CollisionCheck_AC_QuadVsQuad as
                   unsafe extern "C" fn(_: *mut GlobalContext,
                                        _: *mut CollisionCheckContext,
                                        _: *mut Collider, _: *mut Collider)
                       -> ())]]
    };
/* *
 * Iterates through all AC colliders, performing AC collisions with the AT collider.
 */
#[no_mangle]
pub unsafe extern "C" fn CollisionCheck_AC(mut globalCtx: *mut GlobalContext,
                                           mut colChkCtx:
                                               *mut CollisionCheckContext,
                                           mut colAT: *mut Collider) {
    let mut col: *mut *mut Collider = 0 as *mut *mut Collider;
    col = (*colChkCtx).colAC.as_mut_ptr();
    while col <
              (*colChkCtx).colAC.as_mut_ptr().offset((*colChkCtx).colACCount
                                                         as isize) {
        let mut colAC: *mut Collider = *col;
        if !colAC.is_null() &&
               (*colAC).acFlags as libc::c_int &
                   (1 as libc::c_int) << 0 as libc::c_int != 0 {
            if !(!(*colAC).actor.is_null() &&
                     (*(*colAC).actor).update.is_none()) {
                if (*colAC).acFlags as libc::c_int &
                       (*colAT).atFlags as libc::c_int &
                       ((1 as libc::c_int) << 3 as libc::c_int |
                            (1 as libc::c_int) << 4 as libc::c_int |
                            (1 as libc::c_int) << 5 as libc::c_int) != 0 &&
                       colAT != colAC {
                    if !((*colAT).atFlags as libc::c_int &
                             (1 as libc::c_int) << 6 as libc::c_int == 0 &&
                             !(*colAT).actor.is_null() &&
                             (*colAC).actor == (*colAT).actor) {
                        sACVsFuncs[(*colAT).shape as
                                       usize][(*colAC).shape as
                                                  usize].expect("non-null function pointer")(globalCtx,
                                                                                             colChkCtx,
                                                                                             colAT,
                                                                                             colAC);
                    }
                }
            }
        }
        col = col.offset(1)
    };
}
/* *
 * Iterates through all AT colliders, testing them for AC collisions with each AC collider, setting the info regarding
 * the collision for each AC and AT collider that collided. Then spawns hitmarks and plays sound effects for each
 * successful collision. To collide, an AT collider must share a type (AC_TYPE_PLAYER, AC_TYPE_ENEMY, or AC_TYPE_OTHER)
 * with the AC collider and the toucher and bumper elements that overlapped must share a dmgFlag.
 */
#[no_mangle]
pub unsafe extern "C" fn CollisionCheck_AT(mut globalCtx: *mut GlobalContext,
                                           mut colChkCtx:
                                               *mut CollisionCheckContext) {
    let mut col: *mut *mut Collider = 0 as *mut *mut Collider;
    if (*colChkCtx).colATCount as libc::c_int == 0 as libc::c_int ||
           (*colChkCtx).colACCount == 0 as libc::c_int {
        return
    }
    col = (*colChkCtx).colAT.as_mut_ptr();
    while col <
              (*colChkCtx).colAT.as_mut_ptr().offset((*colChkCtx).colATCount
                                                         as libc::c_int as
                                                         isize) {
        let mut colAT: *mut Collider = *col;
        if !colAT.is_null() &&
               (*colAT).atFlags as libc::c_int &
                   (1 as libc::c_int) << 0 as libc::c_int != 0 {
            if !(!(*colAT).actor.is_null() &&
                     (*(*colAT).actor).update.is_none()) {
                CollisionCheck_AC(globalCtx, colChkCtx, colAT);
            }
        }
        col = col.offset(1)
    }
    CollisionCheck_SetHitEffects(globalCtx, colChkCtx);
}
/* *
 * Get mass type. Immobile colliders cannot be pushed, while heavy colliders can only be pushed by heavy and immobile
 * colliders.
 */
#[no_mangle]
pub unsafe extern "C" fn CollisionCheck_GetMassType(mut mass: u8_0) -> s32 {
    if mass as libc::c_int == 0xff as libc::c_int {
        return MASSTYPE_IMMOVABLE as libc::c_int
    }
    if mass as libc::c_int == 0xfe as libc::c_int {
        return MASSTYPE_HEAVY as libc::c_int
    }
    return MASSTYPE_NORMAL as libc::c_int;
}
/* *
 * Sets OC collision flags for OC collider overlaps. If both colliders are attached to actors and can push,
 * also performs an elastic collision where both colliders are moved apart in proportion to their masses.
 */
#[no_mangle]
pub unsafe extern "C" fn CollisionCheck_SetOCvsOC(mut left: *mut Collider,
                                                  mut leftInfo:
                                                      *mut ColliderInfo,
                                                  mut leftPos: *mut Vec3f,
                                                  mut right: *mut Collider,
                                                  mut rightInfo:
                                                      *mut ColliderInfo,
                                                  mut rightPos: *mut Vec3f,
                                                  mut overlap: f32_0) {
    let mut pad: f32_0 = 0.; // leftMassType == MASS_HEAVY | MASS_NORMAL
    let mut leftDispRatio: f32_0 = 0.; // rightMassType == MASS_NORMAL
    let mut rightDispRatio: f32_0 = 0.; // leftMassType == MASS_NORMAL
    let mut xzDist: f32_0 = 0.; // leftMassType == MASS_HEAVY | MASS_IMMOVABLE
    let mut leftMass: f32_0 = 0.;
    let mut rightMass: f32_0 = 0.;
    let mut totalMass: f32_0 = 0.;
    let mut inverseTotalMass: f32_0 = 0.;
    let mut xDelta: f32_0 = 0.;
    let mut zDelta: f32_0 = 0.;
    let mut leftActor: *mut Actor = (*left).actor;
    let mut rightActor: *mut Actor = (*right).actor;
    let mut leftMassType: s32 = 0;
    let mut rightMassType: s32 = 0;
    (*left).ocFlags1 =
        ((*left).ocFlags1 as libc::c_int |
             (1 as libc::c_int) << 1 as libc::c_int) as u8_0;
    (*left).oc = rightActor;
    (*leftInfo).ocElemFlags =
        ((*leftInfo).ocElemFlags as libc::c_int |
             (1 as libc::c_int) << 1 as libc::c_int) as u8_0;
    if (*right).ocFlags2 as libc::c_int &
           (1 as libc::c_int) << 3 as libc::c_int != 0 {
        (*left).ocFlags2 =
            ((*left).ocFlags2 as libc::c_int |
                 (1 as libc::c_int) << 0 as libc::c_int) as u8_0
    }
    (*right).oc = leftActor;
    (*right).ocFlags1 =
        ((*right).ocFlags1 as libc::c_int |
             (1 as libc::c_int) << 1 as libc::c_int) as u8_0;
    (*rightInfo).ocElemFlags =
        ((*rightInfo).ocElemFlags as libc::c_int |
             (1 as libc::c_int) << 1 as libc::c_int) as u8_0;
    if (*left).ocFlags2 as libc::c_int &
           (1 as libc::c_int) << 3 as libc::c_int != 0 {
        (*right).ocFlags2 =
            ((*right).ocFlags2 as libc::c_int |
                 (1 as libc::c_int) << 0 as libc::c_int) as u8_0
    }
    if leftActor.is_null() || rightActor.is_null() ||
           (*left).ocFlags1 as libc::c_int &
               (1 as libc::c_int) << 2 as libc::c_int != 0 ||
           (*right).ocFlags1 as libc::c_int &
               (1 as libc::c_int) << 2 as libc::c_int != 0 {
        return
    }
    rightMassType = CollisionCheck_GetMassType((*leftActor).colChkInfo.mass);
    leftMassType = CollisionCheck_GetMassType((*rightActor).colChkInfo.mass);
    leftMass = (*leftActor).colChkInfo.mass as f32_0;
    rightMass = (*rightActor).colChkInfo.mass as f32_0;
    totalMass = leftMass + rightMass;
    if fabsf(totalMass) < 0.008f32 {
        rightMass = 1.0f32;
        leftMass = rightMass;
        totalMass = 2.0f32
    }
    xDelta = (*rightPos).x - (*leftPos).x;
    zDelta = (*rightPos).z - (*leftPos).z;
    xzDist = sqrtf(xDelta * xDelta + zDelta * zDelta);
    if rightMassType == MASSTYPE_IMMOVABLE as libc::c_int {
        if leftMassType == MASSTYPE_IMMOVABLE as libc::c_int {
            return
        } else {
            leftDispRatio = 0 as libc::c_int as f32_0;
            rightDispRatio = 1 as libc::c_int as f32_0
        }
    } else if rightMassType == MASSTYPE_HEAVY as libc::c_int {
        if leftMassType == MASSTYPE_IMMOVABLE as libc::c_int {
            leftDispRatio = 1 as libc::c_int as f32_0;
            rightDispRatio = 0 as libc::c_int as f32_0
        } else if leftMassType == MASSTYPE_HEAVY as libc::c_int {
            leftDispRatio = 0.5f32;
            rightDispRatio = 0.5f32
        } else {
            leftDispRatio = 0 as libc::c_int as f32_0;
            rightDispRatio = 1 as libc::c_int as f32_0
        }
    } else if leftMassType == MASSTYPE_NORMAL as libc::c_int {
        inverseTotalMass = 1 as libc::c_int as libc::c_float / totalMass;
        leftDispRatio = rightMass * inverseTotalMass;
        rightDispRatio = leftMass * inverseTotalMass
    } else {
        leftDispRatio = 1 as libc::c_int as f32_0;
        rightDispRatio = 0 as libc::c_int as f32_0
    }
    if !(fabsf(xzDist) < 0.008f32) {
        xDelta *= overlap / xzDist;
        zDelta *= overlap / xzDist;
        (*leftActor).colChkInfo.displacement.x += -xDelta * leftDispRatio;
        (*leftActor).colChkInfo.displacement.z += -zDelta * leftDispRatio;
        (*rightActor).colChkInfo.displacement.x += xDelta * rightDispRatio;
        (*rightActor).colChkInfo.displacement.z += zDelta * rightDispRatio
    } else if !(overlap == 0.0f32) {
        (*leftActor).colChkInfo.displacement.x += -overlap * leftDispRatio;
        (*rightActor).colChkInfo.displacement.x += overlap * rightDispRatio
    } else {
        (*leftActor).colChkInfo.displacement.x -= leftDispRatio;
        (*rightActor).colChkInfo.displacement.x += rightDispRatio
    };
}
/* *
 * OC overlap check for two JntSphs
 */
#[no_mangle]
pub unsafe extern "C" fn CollisionCheck_OC_JntSphVsJntSph(mut globalCtx:
                                                              *mut GlobalContext,
                                                          mut colChkCtx:
                                                              *mut CollisionCheckContext,
                                                          mut l:
                                                              *mut Collider,
                                                          mut r:
                                                              *mut Collider) {
    let mut left: *mut ColliderJntSph = l as *mut ColliderJntSph;
    let mut leftElem: *mut ColliderJntSphElement =
        0 as *mut ColliderJntSphElement;
    let mut right: *mut ColliderJntSph = r as *mut ColliderJntSph;
    let mut rightElem: *mut ColliderJntSphElement =
        0 as *mut ColliderJntSphElement;
    let mut overlap: f32_0 = 0.;
    if (*left).count > 0 as libc::c_int && !(*left).elements.is_null() &&
           (*right).count > 0 as libc::c_int && !(*right).elements.is_null() {
        leftElem = (*left).elements;
        while leftElem < (*left).elements.offset((*left).count as isize) {
            if !((*leftElem).info.ocElemFlags as libc::c_int &
                     (1 as libc::c_int) << 0 as libc::c_int == 0) {
                rightElem = (*right).elements;
                while rightElem <
                          (*right).elements.offset((*right).count as isize) {
                    if !((*rightElem).info.ocElemFlags as libc::c_int &
                             (1 as libc::c_int) << 0 as libc::c_int == 0) {
                        if Math3D_SphVsSphOverlap(&mut (*leftElem).dim.worldSphere,
                                                  &mut (*rightElem).dim.worldSphere,
                                                  &mut overlap) ==
                               1 as libc::c_int {
                            let mut leftPos: Vec3f =
                                Vec3f{x: 0., y: 0., z: 0.,};
                            let mut rightPos: Vec3f =
                                Vec3f{x: 0., y: 0., z: 0.,};
                            Math_Vec3s_ToVec3f(&mut leftPos,
                                               &mut (*leftElem).dim.worldSphere.center);
                            Math_Vec3s_ToVec3f(&mut rightPos,
                                               &mut (*rightElem).dim.worldSphere.center);
                            CollisionCheck_SetOCvsOC(&mut (*left).base,
                                                     &mut (*leftElem).info,
                                                     &mut leftPos,
                                                     &mut (*right).base,
                                                     &mut (*rightElem).info,
                                                     &mut rightPos, overlap);
                        }
                    }
                    rightElem = rightElem.offset(1)
                }
            }
            leftElem = leftElem.offset(1)
        }
    };
}
/* *
 * OC overlap check for a JntSph and Cylinder
 */
#[no_mangle]
pub unsafe extern "C" fn CollisionCheck_OC_JntSphVsCyl(mut globalCtx:
                                                           *mut GlobalContext,
                                                       mut colChkCtx:
                                                           *mut CollisionCheckContext,
                                                       mut l: *mut Collider,
                                                       mut r: *mut Collider) {
    let mut left: *mut ColliderJntSph = l as *mut ColliderJntSph;
    let mut leftElem: *mut ColliderJntSphElement =
        0 as *mut ColliderJntSphElement;
    let mut right: *mut ColliderCylinder = r as *mut ColliderCylinder;
    let mut overlap: f32_0 = 0.;
    if (*left).count > 0 as libc::c_int && !(*left).elements.is_null() {
        if (*right).base.ocFlags1 as libc::c_int &
               (1 as libc::c_int) << 0 as libc::c_int != 0 &&
               (*right).info.ocElemFlags as libc::c_int &
                   (1 as libc::c_int) << 0 as libc::c_int != 0 {
            leftElem = (*left).elements;
            while leftElem < (*left).elements.offset((*left).count as isize) {
                if !((*leftElem).info.ocElemFlags as libc::c_int &
                         (1 as libc::c_int) << 0 as libc::c_int == 0) {
                    if Math3D_SphVsCylOverlapDist(&mut (*leftElem).dim.worldSphere,
                                                  &mut (*right).dim,
                                                  &mut overlap) ==
                           1 as libc::c_int {
                        let mut leftPos: Vec3f = Vec3f{x: 0., y: 0., z: 0.,};
                        let mut rightPos: Vec3f = Vec3f{x: 0., y: 0., z: 0.,};
                        Math_Vec3s_ToVec3f(&mut leftPos,
                                           &mut (*leftElem).dim.worldSphere.center);
                        Math_Vec3s_ToVec3f(&mut rightPos,
                                           &mut (*right).dim.pos);
                        CollisionCheck_SetOCvsOC(&mut (*left).base,
                                                 &mut (*leftElem).info,
                                                 &mut leftPos,
                                                 &mut (*right).base,
                                                 &mut (*right).info,
                                                 &mut rightPos, overlap);
                    }
                }
                leftElem = leftElem.offset(1)
            }
        }
    };
}
/* *
 * OC overlap check for a Cylinder and JntSph
 */
#[no_mangle]
pub unsafe extern "C" fn CollisionCheck_OC_CylVsJntSph(mut globalCtx:
                                                           *mut GlobalContext,
                                                       mut colChkCtx:
                                                           *mut CollisionCheckContext,
                                                       mut l: *mut Collider,
                                                       mut r: *mut Collider) {
    CollisionCheck_OC_JntSphVsCyl(globalCtx, colChkCtx, r, l);
}
/* *
 * OC overlap check for two Cylinders
 */
#[no_mangle]
pub unsafe extern "C" fn CollisionCheck_OC_CylVsCyl(mut globalCtx:
                                                        *mut GlobalContext,
                                                    mut colChkCtx:
                                                        *mut CollisionCheckContext,
                                                    mut l: *mut Collider,
                                                    mut r: *mut Collider) {
    let mut left: *mut ColliderCylinder = l as *mut ColliderCylinder;
    let mut right: *mut ColliderCylinder = r as *mut ColliderCylinder;
    let mut deadSpace: f32_0 = 0.;
    if (*left).base.ocFlags1 as libc::c_int &
           (1 as libc::c_int) << 0 as libc::c_int != 0 &&
           (*right).base.ocFlags1 as libc::c_int &
               (1 as libc::c_int) << 0 as libc::c_int != 0 {
        if (*left).info.ocElemFlags as libc::c_int &
               (1 as libc::c_int) << 0 as libc::c_int != 0 &&
               (*right).info.ocElemFlags as libc::c_int &
                   (1 as libc::c_int) << 0 as libc::c_int != 0 {
            if Math3D_CylOutsideCyl(&mut (*left).dim, &mut (*right).dim,
                                    &mut deadSpace) == 1 as libc::c_int {
                let mut leftPos: Vec3f = Vec3f{x: 0., y: 0., z: 0.,};
                let mut rightPos: Vec3f = Vec3f{x: 0., y: 0., z: 0.,};
                Math_Vec3s_ToVec3f(&mut leftPos, &mut (*left).dim.pos);
                Math_Vec3s_ToVec3f(&mut rightPos, &mut (*right).dim.pos);
                CollisionCheck_SetOCvsOC(&mut (*left).base, &mut (*left).info,
                                         &mut leftPos, &mut (*right).base,
                                         &mut (*right).info, &mut rightPos,
                                         deadSpace);
            }
        }
    };
}
/* *
 *  Skip any OC colliders that are off
 */
#[no_mangle]
pub unsafe extern "C" fn CollisionCheck_SkipOC(mut collider: *mut Collider)
 -> s32 {
    if (*collider).ocFlags1 as libc::c_int &
           (1 as libc::c_int) << 0 as libc::c_int == 0 {
        return 1 as libc::c_int
    }
    return 0 as libc::c_int;
}
/* *
 * Checks for OC compatibility. There are three conditions:
 * First, each collider must have an OC flag corresponding to the other's OC type.
 * Second, OC2_UNK1 and OC2_UNK2 can't collide with each other (has something to do with horses?)
 * Third, the colliders can't collide if they belong to the same actor
 */
#[no_mangle]
pub unsafe extern "C" fn CollisionCheck_Incompatible(mut left: *mut Collider,
                                                     mut right: *mut Collider)
 -> s32 {
    if (*left).ocFlags1 as libc::c_int & (*right).ocFlags2 as libc::c_int &
           ((1 as libc::c_int) << 3 as libc::c_int |
                (1 as libc::c_int) << 4 as libc::c_int |
                (1 as libc::c_int) << 5 as libc::c_int) == 0 ||
           (*left).ocFlags2 as libc::c_int & (*right).ocFlags1 as libc::c_int
               &
               ((1 as libc::c_int) << 3 as libc::c_int |
                    (1 as libc::c_int) << 4 as libc::c_int |
                    (1 as libc::c_int) << 5 as libc::c_int) == 0 ||
           (*left).ocFlags2 as libc::c_int &
               (1 as libc::c_int) << 1 as libc::c_int != 0 &&
               (*right).ocFlags2 as libc::c_int &
                   (1 as libc::c_int) << 2 as libc::c_int != 0 ||
           (*right).ocFlags2 as libc::c_int &
               (1 as libc::c_int) << 1 as libc::c_int != 0 &&
               (*left).ocFlags2 as libc::c_int &
                   (1 as libc::c_int) << 2 as libc::c_int != 0 {
        return 1 as libc::c_int
    }
    if (*left).actor == (*right).actor { return 1 as libc::c_int }
    return 0 as libc::c_int;
}
static mut sOCVsFuncs: [[ColChkVsFunc; 4]; 4] =
    unsafe {
        [[Some(CollisionCheck_OC_JntSphVsJntSph as
                   unsafe extern "C" fn(_: *mut GlobalContext,
                                        _: *mut CollisionCheckContext,
                                        _: *mut Collider, _: *mut Collider)
                       -> ()),
          Some(CollisionCheck_OC_JntSphVsCyl as
                   unsafe extern "C" fn(_: *mut GlobalContext,
                                        _: *mut CollisionCheckContext,
                                        _: *mut Collider, _: *mut Collider)
                       -> ()), None, None],
         [Some(CollisionCheck_OC_CylVsJntSph as
                   unsafe extern "C" fn(_: *mut GlobalContext,
                                        _: *mut CollisionCheckContext,
                                        _: *mut Collider, _: *mut Collider)
                       -> ()),
          Some(CollisionCheck_OC_CylVsCyl as
                   unsafe extern "C" fn(_: *mut GlobalContext,
                                        _: *mut CollisionCheckContext,
                                        _: *mut Collider, _: *mut Collider)
                       -> ()), None, None], [None, None, None, None],
         [None, None, None, None]]
    };
/* *
 * Iterates through all OC colliders and collides them with all subsequent OC colliders on the list. During an OC
 * collision, colliders with overlapping elements move away from each other so that their elements no longer overlap.
 * The relative amount each collider is pushed is determined by the collider's mass. Only JntSph and Cylinder colliders
 * can collide, and each collider must have the OC flag corresponding to the other's OC type. Additionally, OC2_UNK1
 * cannot collide with OC2_UNK2, nor can two colliders that share an actor.
 */
#[no_mangle]
pub unsafe extern "C" fn CollisionCheck_OC(mut globalCtx: *mut GlobalContext,
                                           mut colChkCtx:
                                               *mut CollisionCheckContext) {
    let mut left: *mut *mut Collider = 0 as *mut *mut Collider;
    let mut right: *mut *mut Collider = 0 as *mut *mut Collider;
    let mut vsFunc: ColChkVsFunc = None;
    left = (*colChkCtx).colOC.as_mut_ptr();
    while left <
              (*colChkCtx).colOC.as_mut_ptr().offset((*colChkCtx).colOCCount
                                                         as isize) {
        if !((*left).is_null() ||
                 CollisionCheck_SkipOC(*left) == 1 as libc::c_int) {
            right = left.offset(1 as libc::c_int as isize);
            while right <
                      (*colChkCtx).colOC.as_mut_ptr().offset((*colChkCtx).colOCCount
                                                                 as isize) {
                if !((*right).is_null() ||
                         CollisionCheck_SkipOC(*right) == 1 as libc::c_int ||
                         CollisionCheck_Incompatible(*left, *right) ==
                             1 as libc::c_int) {
                    vsFunc =
                        sOCVsFuncs[(**left).shape as
                                       usize][(**right).shape as usize];
                    if vsFunc.is_none() {
                        // "Not compatible"
                        osSyncPrintf(b"CollisionCheck_OC():\xe6\x9c\xaa\xe5\xaf\xbe\xe5\xbf\x9c %d, %d\n\x00"
                                         as *const u8 as *const libc::c_char,
                                     (**left).shape as libc::c_int,
                                     (**right).shape as libc::c_int);
                    } else {
                        vsFunc.expect("non-null function pointer")(globalCtx,
                                                                   colChkCtx,
                                                                   *left,
                                                                   *right);
                    }
                }
                right = right.offset(1)
            }
        }
        left = left.offset(1)
    };
}
/* *
 * Initializes CollisionCheckInfo to default values
 */
#[no_mangle]
pub unsafe extern "C" fn CollisionCheck_InitInfo(mut info:
                                                     *mut CollisionCheckInfo) {
    static mut init: CollisionCheckInfo =
        {
            let mut init =
                CollisionCheckInfo{damageTable:
                                       0 as *const DamageTable as
                                           *mut DamageTable,
                                   displacement:
                                       {
                                           let mut init =
                                               Vec3f{x: 0.0f32,
                                                     y: 0.0f32,
                                                     z: 0.0f32,};
                                           init
                                       },
                                   cylRadius: 10 as libc::c_int as s16,
                                   cylHeight: 10 as libc::c_int as s16,
                                   cylYShift: 0 as libc::c_int as s16,
                                   mass: 50 as libc::c_int as u8_0,
                                   health: 8 as libc::c_int as u8_0,
                                   damage: 0 as libc::c_int as u8_0,
                                   damageEffect: 0 as libc::c_int as u8_0,
                                   atHitEffect: 0 as libc::c_int as u8_0,
                                   acHitEffect: 0 as libc::c_int as u8_0,};
            init
        };
    *info = init;
}
/* *
 * Resets ColisionCheckInfo fields other than DamageTable, mass, and dim.
 */
#[no_mangle]
pub unsafe extern "C" fn CollisionCheck_ResetDamage(mut info:
                                                        *mut CollisionCheckInfo) {
    (*info).damage = 0 as libc::c_int as u8_0;
    (*info).damageEffect = 0 as libc::c_int as u8_0;
    (*info).atHitEffect = 0 as libc::c_int as u8_0;
    (*info).acHitEffect = 0 as libc::c_int as u8_0;
    (*info).displacement.z = 0.0f32;
    (*info).displacement.y = (*info).displacement.z;
    (*info).displacement.x = (*info).displacement.y;
}
/* *
 * Sets up CollisionCheckInfo using the values in init. Does not set a damage table or the unused unk_14.
 * Unused, as all actors that don't set a damage table set their CollisionCheckInfo manually
 */
#[no_mangle]
pub unsafe extern "C" fn CollisionCheck_SetInfoNoDamageTable(mut info:
                                                                 *mut CollisionCheckInfo,
                                                             mut init:
                                                                 *mut CollisionCheckInfoInit) {
    (*info).health = (*init).health;
    (*info).cylRadius = (*init).cylRadius;
    (*info).cylHeight = (*init).cylHeight;
    (*info).mass = (*init).mass;
}
/* *
 * Sets up CollisionCheckInfo using the values in init. Does not set the unused unk_14
 */
#[no_mangle]
pub unsafe extern "C" fn CollisionCheck_SetInfo(mut info:
                                                    *mut CollisionCheckInfo,
                                                mut damageTable:
                                                    *mut DamageTable,
                                                mut init:
                                                    *mut CollisionCheckInfoInit) {
    (*info).health = (*init).health;
    (*info).damageTable = damageTable;
    (*info).cylRadius = (*init).cylRadius;
    (*info).cylHeight = (*init).cylHeight;
    (*info).mass = (*init).mass;
}
/* *
 * Sets up CollisionCheckInfo using the values in init. Sets the unused unk_14
 */
#[no_mangle]
pub unsafe extern "C" fn CollisionCheck_SetInfo2(mut info:
                                                     *mut CollisionCheckInfo,
                                                 mut damageTable:
                                                     *mut DamageTable,
                                                 mut init:
                                                     *mut CollisionCheckInfoInit2) {
    (*info).health = (*init).health;
    (*info).damageTable = damageTable;
    (*info).cylRadius = (*init).cylRadius;
    (*info).cylHeight = (*init).cylHeight;
    (*info).cylYShift = (*init).cylYShift;
    (*info).mass = (*init).mass;
}
/* *
 * Sets up CollisionCheckInfo using the values in Init and a preset damage table. Sets the unused unk_14.
 * Unused, as all actors that use a preset damage table set their CollisionCheckInfo manually.
 */
#[no_mangle]
pub unsafe extern "C" fn CollisionCheck_SetInfoGetDamageTable(mut info:
                                                                  *mut CollisionCheckInfo,
                                                              mut index: s32,
                                                              mut init:
                                                                  *mut CollisionCheckInfoInit2) {
    CollisionCheck_SetInfo2(info, DamageTable_Get(index), init);
}
/* *
 * Apply AC damage effect
 */
#[no_mangle]
pub unsafe extern "C" fn CollisionCheck_ApplyDamage(mut globalCtx:
                                                        *mut GlobalContext,
                                                    mut colChkCtx:
                                                        *mut CollisionCheckContext,
                                                    mut collider:
                                                        *mut Collider,
                                                    mut info:
                                                        *mut ColliderInfo) {
    let mut tbl: *mut DamageTable = 0 as *mut DamageTable;
    let mut damage: f32_0 = 0.;
    if (*collider).actor.is_null() ||
           (*collider).acFlags as libc::c_int &
               (1 as libc::c_int) << 1 as libc::c_int == 0 {
        return
    }
    if (*info).bumperFlags as libc::c_int &
           (1 as libc::c_int) << 1 as libc::c_int == 0 ||
           (*info).bumperFlags as libc::c_int &
               (1 as libc::c_int) << 4 as libc::c_int != 0 {
        return
    }
    if !(*info).acHitInfo.is_null() {
    } else {
        __assert(b"pclobj_elem->ac_hit_elem != NULL\x00" as *const u8 as
                     *const libc::c_char,
                 b"../z_collision_check.c\x00" as *const u8 as
                     *const libc::c_char, 6493 as libc::c_int);
    };
    tbl = (*(*collider).actor).colChkInfo.damageTable;
    if tbl.is_null() {
        damage =
            (*(*info).acHitInfo).toucher.damage as f32_0 -
                (*info).bumper.defense as libc::c_int as libc::c_float;
        if damage < 0 as libc::c_int as libc::c_float {
            damage = 0 as libc::c_int as f32_0
        }
    } else {
        let mut i: s32 = 0;
        let mut flags: u32_0 = (*(*info).acHitInfo).toucher.dmgFlags;
        i = 0 as libc::c_int;
        while i < 0x20 as libc::c_int {
            if flags == 1 as libc::c_int as libc::c_uint { break ; }
            i += 1;
            flags >>= 1 as libc::c_int
        }
        damage =
            ((*tbl).table[i as usize] as libc::c_int & 0xf as libc::c_int) as
                f32_0;
        (*(*collider).actor).colChkInfo.damageEffect =
            ((*tbl).table[i as usize] as libc::c_int >> 4 as libc::c_int &
                 0xf as libc::c_int) as u8_0
    }
    if (*collider).acFlags as libc::c_int &
           (1 as libc::c_int) << 2 as libc::c_int == 0 {
        (*(*collider).actor).colChkInfo.damage =
            ((*(*collider).actor).colChkInfo.damage as libc::c_float + damage)
                as u8_0
    };
}
/* *
 * Apply ColliderJntSph AC damage effect
 */
#[no_mangle]
pub unsafe extern "C" fn CollisionCheck_ApplyDamageJntSph(mut globalCtx:
                                                              *mut GlobalContext,
                                                          mut colChkCtx:
                                                              *mut CollisionCheckContext,
                                                          mut collider:
                                                              *mut Collider) {
    let mut jntSph: *mut ColliderJntSph = collider as *mut ColliderJntSph;
    let mut i: s32 = 0;
    if (*jntSph).count > 0 as libc::c_int && !(*jntSph).elements.is_null() {
        i = 0 as libc::c_int;
        while i < (*jntSph).count {
            CollisionCheck_ApplyDamage(globalCtx, colChkCtx,
                                       &mut (*jntSph).base,
                                       &mut (*(*jntSph).elements.offset(i as
                                                                            isize)).info);
            i += 1
        }
    };
}
/* *
 * Apply ColliderCylinder AC damage effect
 */
#[no_mangle]
pub unsafe extern "C" fn CollisionCheck_ApplyDamageCyl(mut globalCtx:
                                                           *mut GlobalContext,
                                                       mut colChkCtx:
                                                           *mut CollisionCheckContext,
                                                       mut collider:
                                                           *mut Collider) {
    let mut cylinder: *mut ColliderCylinder =
        collider as *mut ColliderCylinder;
    CollisionCheck_ApplyDamage(globalCtx, colChkCtx, &mut (*cylinder).base,
                               &mut (*cylinder).info);
}
/* *
 * Apply ColliderTris AC damage effect
 */
#[no_mangle]
pub unsafe extern "C" fn CollisionCheck_ApplyDamageTris(mut globalCtx:
                                                            *mut GlobalContext,
                                                        mut colChkCtx:
                                                            *mut CollisionCheckContext,
                                                        mut collider:
                                                            *mut Collider) {
    let mut tris: *mut ColliderTris = collider as *mut ColliderTris;
    let mut i: s32 = 0;
    i = 0 as libc::c_int;
    while i < (*tris).count {
        CollisionCheck_ApplyDamage(globalCtx, colChkCtx, collider,
                                   &mut (*(*tris).elements.offset(i as
                                                                      isize)).info);
        i += 1
    };
}
/* *
 *  Apply ColliderQuad AC damage effect
 */
#[no_mangle]
pub unsafe extern "C" fn CollisionCheck_ApplyDamageQuad(mut globalCtx:
                                                            *mut GlobalContext,
                                                        mut colChkCtx:
                                                            *mut CollisionCheckContext,
                                                        mut collider:
                                                            *mut Collider) {
    let mut quad: *mut ColliderQuad = collider as *mut ColliderQuad;
    CollisionCheck_ApplyDamage(globalCtx, colChkCtx, &mut (*quad).base,
                               &mut (*quad).info);
}
static mut sApplyDamageFuncs: [ColChkApplyFunc; 4] =
    unsafe {
        [Some(CollisionCheck_ApplyDamageJntSph as
                  unsafe extern "C" fn(_: *mut GlobalContext,
                                       _: *mut CollisionCheckContext,
                                       _: *mut Collider) -> ()),
         Some(CollisionCheck_ApplyDamageCyl as
                  unsafe extern "C" fn(_: *mut GlobalContext,
                                       _: *mut CollisionCheckContext,
                                       _: *mut Collider) -> ()),
         Some(CollisionCheck_ApplyDamageTris as
                  unsafe extern "C" fn(_: *mut GlobalContext,
                                       _: *mut CollisionCheckContext,
                                       _: *mut Collider) -> ()),
         Some(CollisionCheck_ApplyDamageQuad as
                  unsafe extern "C" fn(_: *mut GlobalContext,
                                       _: *mut CollisionCheckContext,
                                       _: *mut Collider) -> ())]
    };
/* *
 * For all AC colliders, sets any damage effects from collisions with AT colliders to their corresponding actor's
 * CollisionCheckInfo.
 */
#[no_mangle]
pub unsafe extern "C" fn CollisionCheck_Damage(mut globalCtx:
                                                   *mut GlobalContext,
                                               mut colChkCtx:
                                                   *mut CollisionCheckContext) {
    let mut i: s32 = 0;
    i = 0 as libc::c_int;
    while i < (*colChkCtx).colACCount {
        let mut collider: *mut Collider = (*colChkCtx).colAC[i as usize];
        if !collider.is_null() {
            if !((*collider).acFlags as libc::c_int &
                     (1 as libc::c_int) << 6 as libc::c_int != 0) {
                sApplyDamageFuncs[(*collider).shape as
                                      usize].expect("non-null function pointer")(globalCtx,
                                                                                 colChkCtx,
                                                                                 collider);
            }
        }
        i += 1
    };
}
/* *
 * Checks if the line ab intersects any of the ColliderJntSph's elements
 */
#[no_mangle]
pub unsafe extern "C" fn CollisionCheck_LineOC_JntSph(mut globalCtx:
                                                          *mut GlobalContext,
                                                      mut colChkCtx:
                                                          *mut CollisionCheckContext,
                                                      mut collider:
                                                          *mut Collider,
                                                      mut a: *mut Vec3f,
                                                      mut b: *mut Vec3f)
 -> s32 {
    static mut D_8015E610: Linef =
        Linef{a: Vec3f{x: 0., y: 0., z: 0.,},
              b: Vec3f{x: 0., y: 0., z: 0.,},};
    let mut jntSph: *mut ColliderJntSph = collider as *mut ColliderJntSph;
    let mut i: s32 = 0;
    i = 0 as libc::c_int;
    while i < (*jntSph).count {
        let mut element: *mut ColliderJntSphElement =
            &mut *(*jntSph).elements.offset(i as isize) as
                *mut ColliderJntSphElement;
        if !((*element).info.ocElemFlags as libc::c_int &
                 (1 as libc::c_int) << 0 as libc::c_int == 0) {
            D_8015E610.a = *a;
            D_8015E610.b = *b;
            if Math3D_LineVsSph(&mut (*element).dim.worldSphere,
                                &mut D_8015E610) == 1 as libc::c_int {
                return 1 as libc::c_int
            }
        }
        i += 1
    }
    return 0 as libc::c_int;
}
/* *
 * Checks if the line segment ab intersects the ColliderCylinder
 */
#[no_mangle]
pub unsafe extern "C" fn CollisionCheck_LineOC_Cyl(mut globalCtx:
                                                       *mut GlobalContext,
                                                   mut colChkCtx:
                                                       *mut CollisionCheckContext,
                                                   mut collider:
                                                       *mut Collider,
                                                   mut a: *mut Vec3f,
                                                   mut b: *mut Vec3f) -> s32 {
    static mut D_8015E628: Vec3f = Vec3f{x: 0., y: 0., z: 0.,};
    static mut D_8015E638: Vec3f = Vec3f{x: 0., y: 0., z: 0.,};
    let mut cylinder: *mut ColliderCylinder =
        collider as *mut ColliderCylinder;
    if (*cylinder).info.ocElemFlags as libc::c_int &
           (1 as libc::c_int) << 0 as libc::c_int == 0 {
        return 0 as libc::c_int
    }
    if Math3D_CylVsLineSeg(&mut (*cylinder).dim, a, b, &mut D_8015E628,
                           &mut D_8015E638) != 0 as libc::c_int {
        return 1 as libc::c_int
    }
    return 0 as libc::c_int;
}
static mut sOCLineCheckFuncs: [ColChkLineFunc; 4] =
    unsafe {
        [Some(CollisionCheck_LineOC_JntSph as
                  unsafe extern "C" fn(_: *mut GlobalContext,
                                       _: *mut CollisionCheckContext,
                                       _: *mut Collider, _: *mut Vec3f,
                                       _: *mut Vec3f) -> s32),
         Some(CollisionCheck_LineOC_Cyl as
                  unsafe extern "C" fn(_: *mut GlobalContext,
                                       _: *mut CollisionCheckContext,
                                       _: *mut Collider, _: *mut Vec3f,
                                       _: *mut Vec3f) -> s32), None, None]
    };
/* *
 * Checks if the line segment ab intersects any OC colliders, excluding those attached to actors
 * on the exclusion list. Returns true if there are any intersections and false otherwise.
 */
#[no_mangle]
pub unsafe extern "C" fn CollisionCheck_LineOC(mut globalCtx:
                                                   *mut GlobalContext,
                                               mut colChkCtx:
                                                   *mut CollisionCheckContext,
                                               mut a: *mut Vec3f,
                                               mut b: *mut Vec3f,
                                               mut exclusions:
                                                   *mut *mut Actor,
                                               mut numExclusions: s32)
 -> s32 {
    let mut lineCheck: ColChkLineFunc = None;
    let mut col: *mut *mut Collider = 0 as *mut *mut Collider;
    let mut i: s32 = 0;
    let mut exclude: s32 = 0;
    let mut result: s32 = 0 as libc::c_int;
    col = (*colChkCtx).colOC.as_mut_ptr();
    while col <
              (*colChkCtx).colOC.as_mut_ptr().offset((*colChkCtx).colOCCount
                                                         as isize) {
        if !(CollisionCheck_SkipOC(*col) == 1 as libc::c_int) {
            exclude = 0 as libc::c_int;
            i = 0 as libc::c_int;
            while i < numExclusions {
                if (**col).actor == *exclusions.offset(i as isize) {
                    exclude = 1 as libc::c_int;
                    break ;
                } else { i += 1 }
            }
            if !(exclude == 1 as libc::c_int) {
                lineCheck = sOCLineCheckFuncs[(**col).shape as usize];
                if lineCheck.is_none() {
                    // "type %d not supported"
                    osSyncPrintf(b"CollisionCheck_generalLineOcCheck():\xe6\x9c\xaa\xe5\xaf\xbe\xe5\xbf\x9c %d\xe3\x82\xbf\xe3\x82\xa4\xe3\x83\x97\n\x00"
                                     as *const u8 as *const libc::c_char,
                                 (**col).shape as libc::c_int);
                } else {
                    result =
                        lineCheck.expect("non-null function pointer")(globalCtx,
                                                                      colChkCtx,
                                                                      *col, a,
                                                                      b);
                    if result != 0 { break ; }
                }
            }
        }
        col = col.offset(1)
    }
    return result;
}
/* *
 * Checks if the line segment ab intersects any OC colliders. Returns true if there are any intersections and false
 * otherwise. Unused.
 */
#[no_mangle]
pub unsafe extern "C" fn CollisionCheck_LineOCCheckAll(mut globalCtx:
                                                           *mut GlobalContext,
                                                       mut colChkCtx:
                                                           *mut CollisionCheckContext,
                                                       mut a: *mut Vec3f,
                                                       mut b: *mut Vec3f)
 -> s32 {
    return CollisionCheck_LineOC(globalCtx, colChkCtx, a, b,
                                 0 as *mut *mut Actor, 0 as libc::c_int);
}
/* *
 * Checks if the line segment ab intersects any OC colliders, excluding those attached to actors on the exclusion list.
 * Returns true if there are any intersections and false otherwise.
 */
#[no_mangle]
pub unsafe extern "C" fn CollisionCheck_LineOCCheck(mut globalCtx:
                                                        *mut GlobalContext,
                                                    mut colChkCtx:
                                                        *mut CollisionCheckContext,
                                                    mut a: *mut Vec3f,
                                                    mut b: *mut Vec3f,
                                                    mut exclusions:
                                                        *mut *mut Actor,
                                                    mut numExclusions: s32)
 -> s32 {
    return CollisionCheck_LineOC(globalCtx, colChkCtx, a, b, exclusions,
                                 numExclusions);
}
/* *
 * Moves the ColliderCylinder's position to the actor's position
 */
#[no_mangle]
pub unsafe extern "C" fn Collider_UpdateCylinder(mut actor: *mut Actor,
                                                 mut collider:
                                                     *mut ColliderCylinder) {
    (*collider).dim.pos.x = (*actor).world.pos.x as s16;
    (*collider).dim.pos.y = (*actor).world.pos.y as s16;
    (*collider).dim.pos.z = (*actor).world.pos.z as s16;
}
/* *
 * Sets the ColliderCylinder's position
 */
#[no_mangle]
pub unsafe extern "C" fn Collider_SetCylinderPosition(mut collider:
                                                          *mut ColliderCylinder,
                                                      mut pos: *mut Vec3s) {
    (*collider).dim.pos.x = (*pos).x;
    (*collider).dim.pos.y = (*pos).y;
    (*collider).dim.pos.z = (*pos).z;
}
/* *
 * Sets the ColliderQuad's vertices
 */
#[no_mangle]
pub unsafe extern "C" fn Collider_SetQuadVertices(mut collider:
                                                      *mut ColliderQuad,
                                                  mut a: *mut Vec3f,
                                                  mut b: *mut Vec3f,
                                                  mut c: *mut Vec3f,
                                                  mut d: *mut Vec3f) {
    Math_Vec3f_Copy(&mut *(*collider).dim.quad.as_mut_ptr().offset(2 as
                                                                       libc::c_int
                                                                       as
                                                                       isize),
                    c);
    Math_Vec3f_Copy(&mut *(*collider).dim.quad.as_mut_ptr().offset(3 as
                                                                       libc::c_int
                                                                       as
                                                                       isize),
                    d);
    Math_Vec3f_Copy(&mut *(*collider).dim.quad.as_mut_ptr().offset(0 as
                                                                       libc::c_int
                                                                       as
                                                                       isize),
                    a);
    Math_Vec3f_Copy(&mut *(*collider).dim.quad.as_mut_ptr().offset(1 as
                                                                       libc::c_int
                                                                       as
                                                                       isize),
                    b);
    Collider_SetQuadMidpoints(&mut (*collider).dim);
}
/* *
 * Sets the specified ColliderTrisElement's vertices
 */
#[no_mangle]
pub unsafe extern "C" fn Collider_SetTrisVertices(mut collider:
                                                      *mut ColliderTris,
                                                  mut index: s32,
                                                  mut a: *mut Vec3f,
                                                  mut b: *mut Vec3f,
                                                  mut c: *mut Vec3f) {
    let mut element: *mut ColliderTrisElement =
        &mut *(*collider).elements.offset(index as isize) as
            *mut ColliderTrisElement;
    let mut nx: f32_0 = 0.;
    let mut ny: f32_0 = 0.;
    let mut nz: f32_0 = 0.;
    let mut originDist: f32_0 = 0.;
    Math_Vec3f_Copy(&mut *(*element).dim.vtx.as_mut_ptr().offset(0 as
                                                                     libc::c_int
                                                                     as
                                                                     isize),
                    a);
    Math_Vec3f_Copy(&mut *(*element).dim.vtx.as_mut_ptr().offset(1 as
                                                                     libc::c_int
                                                                     as
                                                                     isize),
                    b);
    Math_Vec3f_Copy(&mut *(*element).dim.vtx.as_mut_ptr().offset(2 as
                                                                     libc::c_int
                                                                     as
                                                                     isize),
                    c);
    Math3D_DefPlane(a, b, c, &mut nx, &mut ny, &mut nz, &mut originDist);
    (*element).dim.plane.normal.x = nx;
    (*element).dim.plane.normal.y = ny;
    (*element).dim.plane.normal.z = nz;
    (*element).dim.plane.originDist = originDist;
}
/* *
 * Sets the specified ColliderTrisElement's dim using the values in src
 */
#[no_mangle]
pub unsafe extern "C" fn Collider_SetTrisDim(mut globalCtx:
                                                 *mut GlobalContext,
                                             mut collider: *mut ColliderTris,
                                             mut index: s32,
                                             mut src:
                                                 *mut ColliderTrisElementDimInit) {
    let mut element: *mut ColliderTrisElement =
        &mut *(*collider).elements.offset(index as isize) as
            *mut ColliderTrisElement;
    Collider_SetTrisElementDim(globalCtx, &mut (*element).dim, src);
}
/* *
 * Updates the world spheres for all of the collider's JntSph elements attached to the specified limb
 */
#[no_mangle]
pub unsafe extern "C" fn Collider_UpdateSpheres(mut limb: s32,
                                                mut collider:
                                                    *mut ColliderJntSph) {
    static mut D_8015E648: Vec3f =
        Vec3f{x: 0., y: 0., z: 0.,}; // bss ordering changes here
    static mut D_8015CF00: Vec3f = Vec3f{x: 0., y: 0., z: 0.,};
    let mut i: s32 = 0;
    i = 0 as libc::c_int;
    while i < (*collider).count {
        if limb ==
               (*(*collider).elements.offset(i as isize)).dim.limb as
                   libc::c_int {
            D_8015E648.x =
                (*(*collider).elements.offset(i as
                                                  isize)).dim.modelSphere.center.x
                    as f32_0;
            D_8015E648.y =
                (*(*collider).elements.offset(i as
                                                  isize)).dim.modelSphere.center.y
                    as f32_0;
            D_8015E648.z =
                (*(*collider).elements.offset(i as
                                                  isize)).dim.modelSphere.center.z
                    as f32_0;
            Matrix_MultVec3f(&mut D_8015E648, &mut D_8015CF00);
            (*(*collider).elements.offset(i as
                                              isize)).dim.worldSphere.center.x
                = D_8015CF00.x as s16;
            (*(*collider).elements.offset(i as
                                              isize)).dim.worldSphere.center.y
                = D_8015CF00.y as s16;
            (*(*collider).elements.offset(i as
                                              isize)).dim.worldSphere.center.z
                = D_8015CF00.z as s16;
            (*(*collider).elements.offset(i as isize)).dim.worldSphere.radius
                =
                ((*(*collider).elements.offset(i as
                                                   isize)).dim.modelSphere.radius
                     as libc::c_int as libc::c_float *
                     (*(*collider).elements.offset(i as isize)).dim.scale) as
                    s16
        }
        i += 1
    };
}
/* *
 * Spawns red blood droplets.
 * No actor has a collision type that spawns red blood.
 */
#[no_mangle]
pub unsafe extern "C" fn CollisionCheck_SpawnRedBlood(mut globalCtx:
                                                          *mut GlobalContext,
                                                      mut v: *mut Vec3f) {
    static mut D_8015CF10: EffectSparkInit =
        EffectSparkInit{position: Vec3s{x: 0, y: 0, z: 0,},
                        numElements: 0,
                        elements:
                            [EffectSparkElement{velocity:
                                                    Vec3f{x: 0.,
                                                          y: 0.,
                                                          z: 0.,},
                                                position:
                                                    Vec3f{x: 0.,
                                                          y: 0.,
                                                          z: 0.,},
                                                unkVelocity:
                                                    Vec3s{x: 0, y: 0, z: 0,},
                                                unkPosition:
                                                    Vec3s{x: 0,
                                                          y: 0,
                                                          z: 0,},}; 32],
                        speed: 0.,
                        gravity: 0.,
                        uDiv: 0,
                        vDiv: 0,
                        colorStart: [Color_RGBA8{r: 0, g: 0, b: 0, a: 0,}; 4],
                        colorEnd: [Color_RGBA8{r: 0, g: 0, b: 0, a: 0,}; 4],
                        timer: 0,
                        duration: 0,};
    let mut effectIndex: s32 = 0;
    D_8015CF10.position.x = (*v).x as s16;
    D_8015CF10.position.y = (*v).y as s16;
    D_8015CF10.position.z = (*v).z as s16;
    D_8015CF10.uDiv = 5 as libc::c_int as u32_0;
    D_8015CF10.vDiv = 5 as libc::c_int as u32_0;
    D_8015CF10.colorStart[0 as libc::c_int as usize].r =
        128 as libc::c_int as u8_0;
    D_8015CF10.colorStart[0 as libc::c_int as usize].g =
        0 as libc::c_int as u8_0;
    D_8015CF10.colorStart[0 as libc::c_int as usize].b =
        64 as libc::c_int as u8_0;
    D_8015CF10.colorStart[0 as libc::c_int as usize].a =
        255 as libc::c_int as u8_0;
    D_8015CF10.colorStart[1 as libc::c_int as usize].r =
        128 as libc::c_int as u8_0;
    D_8015CF10.colorStart[1 as libc::c_int as usize].g =
        0 as libc::c_int as u8_0;
    D_8015CF10.colorStart[1 as libc::c_int as usize].b =
        64 as libc::c_int as u8_0;
    D_8015CF10.colorStart[1 as libc::c_int as usize].a =
        255 as libc::c_int as u8_0;
    D_8015CF10.colorStart[2 as libc::c_int as usize].r =
        255 as libc::c_int as u8_0;
    D_8015CF10.colorStart[2 as libc::c_int as usize].g =
        128 as libc::c_int as u8_0;
    D_8015CF10.colorStart[2 as libc::c_int as usize].b =
        0 as libc::c_int as u8_0;
    D_8015CF10.colorStart[2 as libc::c_int as usize].a =
        255 as libc::c_int as u8_0;
    D_8015CF10.colorStart[3 as libc::c_int as usize].r =
        255 as libc::c_int as u8_0;
    D_8015CF10.colorStart[3 as libc::c_int as usize].g =
        128 as libc::c_int as u8_0;
    D_8015CF10.colorStart[3 as libc::c_int as usize].b =
        0 as libc::c_int as u8_0;
    D_8015CF10.colorStart[3 as libc::c_int as usize].a =
        255 as libc::c_int as u8_0;
    D_8015CF10.colorEnd[0 as libc::c_int as usize].r =
        64 as libc::c_int as u8_0;
    D_8015CF10.colorEnd[0 as libc::c_int as usize].g =
        0 as libc::c_int as u8_0;
    D_8015CF10.colorEnd[0 as libc::c_int as usize].b =
        32 as libc::c_int as u8_0;
    D_8015CF10.colorEnd[0 as libc::c_int as usize].a =
        0 as libc::c_int as u8_0;
    D_8015CF10.colorEnd[1 as libc::c_int as usize].r =
        64 as libc::c_int as u8_0;
    D_8015CF10.colorEnd[1 as libc::c_int as usize].g =
        0 as libc::c_int as u8_0;
    D_8015CF10.colorEnd[1 as libc::c_int as usize].b =
        32 as libc::c_int as u8_0;
    D_8015CF10.colorEnd[1 as libc::c_int as usize].a =
        0 as libc::c_int as u8_0;
    D_8015CF10.colorEnd[2 as libc::c_int as usize].r =
        128 as libc::c_int as u8_0;
    D_8015CF10.colorEnd[2 as libc::c_int as usize].g =
        0 as libc::c_int as u8_0;
    D_8015CF10.colorEnd[2 as libc::c_int as usize].b =
        64 as libc::c_int as u8_0;
    D_8015CF10.colorEnd[2 as libc::c_int as usize].a =
        0 as libc::c_int as u8_0;
    D_8015CF10.colorEnd[3 as libc::c_int as usize].r =
        128 as libc::c_int as u8_0;
    D_8015CF10.colorEnd[3 as libc::c_int as usize].g =
        0 as libc::c_int as u8_0;
    D_8015CF10.colorEnd[3 as libc::c_int as usize].b =
        64 as libc::c_int as u8_0;
    D_8015CF10.colorEnd[3 as libc::c_int as usize].a =
        0 as libc::c_int as u8_0;
    D_8015CF10.timer = 0 as libc::c_int;
    D_8015CF10.duration = 16 as libc::c_int;
    D_8015CF10.speed = 8.0f32;
    D_8015CF10.gravity = -1.0f32;
    Effect_Add(globalCtx, &mut effectIndex, EFFECT_SPARK as libc::c_int,
               0 as libc::c_int as u8_0, 1 as libc::c_int as u8_0,
               &mut D_8015CF10 as *mut EffectSparkInit as *mut libc::c_void);
}
/* *
 * Spawns water droplets.
 * No actor has a collision type that spawns water droplets.
 */
#[no_mangle]
pub unsafe extern "C" fn CollisionCheck_SpawnWaterDroplets(mut globalCtx:
                                                               *mut GlobalContext,
                                                           mut v:
                                                               *mut Vec3f) {
    static mut D_8015D3D8: EffectSparkInit =
        EffectSparkInit{position: Vec3s{x: 0, y: 0, z: 0,},
                        numElements: 0,
                        elements:
                            [EffectSparkElement{velocity:
                                                    Vec3f{x: 0.,
                                                          y: 0.,
                                                          z: 0.,},
                                                position:
                                                    Vec3f{x: 0.,
                                                          y: 0.,
                                                          z: 0.,},
                                                unkVelocity:
                                                    Vec3s{x: 0, y: 0, z: 0,},
                                                unkPosition:
                                                    Vec3s{x: 0,
                                                          y: 0,
                                                          z: 0,},}; 32],
                        speed: 0.,
                        gravity: 0.,
                        uDiv: 0,
                        vDiv: 0,
                        colorStart: [Color_RGBA8{r: 0, g: 0, b: 0, a: 0,}; 4],
                        colorEnd: [Color_RGBA8{r: 0, g: 0, b: 0, a: 0,}; 4],
                        timer: 0,
                        duration: 0,};
    let mut effectIndex: s32 = 0;
    D_8015D3D8.position.x = (*v).x as s16;
    D_8015D3D8.position.y = (*v).y as s16;
    D_8015D3D8.position.z = (*v).z as s16;
    D_8015D3D8.uDiv = 5 as libc::c_int as u32_0;
    D_8015D3D8.vDiv = 5 as libc::c_int as u32_0;
    D_8015D3D8.colorStart[0 as libc::c_int as usize].r =
        255 as libc::c_int as u8_0;
    D_8015D3D8.colorStart[0 as libc::c_int as usize].g =
        255 as libc::c_int as u8_0;
    D_8015D3D8.colorStart[0 as libc::c_int as usize].b =
        255 as libc::c_int as u8_0;
    D_8015D3D8.colorStart[0 as libc::c_int as usize].a =
        255 as libc::c_int as u8_0;
    D_8015D3D8.colorStart[1 as libc::c_int as usize].r =
        100 as libc::c_int as u8_0;
    D_8015D3D8.colorStart[1 as libc::c_int as usize].g =
        100 as libc::c_int as u8_0;
    D_8015D3D8.colorStart[1 as libc::c_int as usize].b =
        100 as libc::c_int as u8_0;
    D_8015D3D8.colorStart[1 as libc::c_int as usize].a =
        100 as libc::c_int as u8_0;
    D_8015D3D8.colorStart[2 as libc::c_int as usize].r =
        100 as libc::c_int as u8_0;
    D_8015D3D8.colorStart[2 as libc::c_int as usize].g =
        100 as libc::c_int as u8_0;
    D_8015D3D8.colorStart[2 as libc::c_int as usize].b =
        100 as libc::c_int as u8_0;
    D_8015D3D8.colorStart[2 as libc::c_int as usize].a =
        100 as libc::c_int as u8_0;
    D_8015D3D8.colorStart[3 as libc::c_int as usize].r =
        100 as libc::c_int as u8_0;
    D_8015D3D8.colorStart[3 as libc::c_int as usize].g =
        100 as libc::c_int as u8_0;
    D_8015D3D8.colorStart[3 as libc::c_int as usize].b =
        100 as libc::c_int as u8_0;
    D_8015D3D8.colorStart[3 as libc::c_int as usize].a =
        100 as libc::c_int as u8_0;
    D_8015D3D8.colorEnd[0 as libc::c_int as usize].r =
        50 as libc::c_int as u8_0;
    D_8015D3D8.colorEnd[0 as libc::c_int as usize].g =
        50 as libc::c_int as u8_0;
    D_8015D3D8.colorEnd[0 as libc::c_int as usize].b =
        50 as libc::c_int as u8_0;
    D_8015D3D8.colorEnd[0 as libc::c_int as usize].a =
        50 as libc::c_int as u8_0;
    D_8015D3D8.colorEnd[1 as libc::c_int as usize].r =
        50 as libc::c_int as u8_0;
    D_8015D3D8.colorEnd[1 as libc::c_int as usize].g =
        50 as libc::c_int as u8_0;
    D_8015D3D8.colorEnd[1 as libc::c_int as usize].b =
        50 as libc::c_int as u8_0;
    D_8015D3D8.colorEnd[1 as libc::c_int as usize].a =
        50 as libc::c_int as u8_0;
    D_8015D3D8.colorEnd[2 as libc::c_int as usize].r =
        50 as libc::c_int as u8_0;
    D_8015D3D8.colorEnd[2 as libc::c_int as usize].g =
        50 as libc::c_int as u8_0;
    D_8015D3D8.colorEnd[2 as libc::c_int as usize].b =
        50 as libc::c_int as u8_0;
    D_8015D3D8.colorEnd[2 as libc::c_int as usize].a =
        50 as libc::c_int as u8_0;
    D_8015D3D8.colorEnd[3 as libc::c_int as usize].r =
        0 as libc::c_int as u8_0;
    D_8015D3D8.colorEnd[3 as libc::c_int as usize].g =
        0 as libc::c_int as u8_0;
    D_8015D3D8.colorEnd[3 as libc::c_int as usize].b =
        0 as libc::c_int as u8_0;
    D_8015D3D8.colorEnd[3 as libc::c_int as usize].a =
        0 as libc::c_int as u8_0;
    D_8015D3D8.timer = 0 as libc::c_int;
    D_8015D3D8.duration = 16 as libc::c_int;
    D_8015D3D8.speed = 8.0f32;
    D_8015D3D8.gravity = -1.0f32;
    Effect_Add(globalCtx, &mut effectIndex, EFFECT_SPARK as libc::c_int,
               0 as libc::c_int as u8_0, 1 as libc::c_int as u8_0,
               &mut D_8015D3D8 as *mut EffectSparkInit as *mut libc::c_void);
}
/* *
 * Spawns streaks of light from hits against solid objects
 */
#[no_mangle]
pub unsafe extern "C" fn CollisionCheck_SpawnShieldParticles(mut globalCtx:
                                                                 *mut GlobalContext,
                                                             mut v:
                                                                 *mut Vec3f) {
    static mut initMetal: EffectShieldParticleInit =
        {
            let mut init =
                EffectShieldParticleInit{numElements:
                                             16 as libc::c_int as u8_0,
                                         position:
                                             {
                                                 let mut init =
                                                     Vec3s{x:
                                                               0 as
                                                                   libc::c_int
                                                                   as s16,
                                                           y:
                                                               0 as
                                                                   libc::c_int
                                                                   as s16,
                                                           z:
                                                               0 as
                                                                   libc::c_int
                                                                   as s16,};
                                                 init
                                             },
                                         primColorStart:
                                             {
                                                 let mut init =
                                                     Color_RGBA8{r:
                                                                     0 as
                                                                         libc::c_int
                                                                         as
                                                                         u8_0,
                                                                 g:
                                                                     200 as
                                                                         libc::c_int
                                                                         as
                                                                         u8_0,
                                                                 b:
                                                                     255 as
                                                                         libc::c_int
                                                                         as
                                                                         u8_0,
                                                                 a:
                                                                     255 as
                                                                         libc::c_int
                                                                         as
                                                                         u8_0,};
                                                 init
                                             },
                                         envColorStart:
                                             {
                                                 let mut init =
                                                     Color_RGBA8{r:
                                                                     255 as
                                                                         libc::c_int
                                                                         as
                                                                         u8_0,
                                                                 g:
                                                                     255 as
                                                                         libc::c_int
                                                                         as
                                                                         u8_0,
                                                                 b:
                                                                     255 as
                                                                         libc::c_int
                                                                         as
                                                                         u8_0,
                                                                 a:
                                                                     255 as
                                                                         libc::c_int
                                                                         as
                                                                         u8_0,};
                                                 init
                                             },
                                         primColorMid:
                                             {
                                                 let mut init =
                                                     Color_RGBA8{r:
                                                                     255 as
                                                                         libc::c_int
                                                                         as
                                                                         u8_0,
                                                                 g:
                                                                     255 as
                                                                         libc::c_int
                                                                         as
                                                                         u8_0,
                                                                 b:
                                                                     128 as
                                                                         libc::c_int
                                                                         as
                                                                         u8_0,
                                                                 a:
                                                                     255 as
                                                                         libc::c_int
                                                                         as
                                                                         u8_0,};
                                                 init
                                             },
                                         envColorMid:
                                             {
                                                 let mut init =
                                                     Color_RGBA8{r:
                                                                     255 as
                                                                         libc::c_int
                                                                         as
                                                                         u8_0,
                                                                 g:
                                                                     255 as
                                                                         libc::c_int
                                                                         as
                                                                         u8_0,
                                                                 b:
                                                                     0 as
                                                                         libc::c_int
                                                                         as
                                                                         u8_0,
                                                                 a:
                                                                     255 as
                                                                         libc::c_int
                                                                         as
                                                                         u8_0,};
                                                 init
                                             },
                                         primColorEnd:
                                             {
                                                 let mut init =
                                                     Color_RGBA8{r:
                                                                     255 as
                                                                         libc::c_int
                                                                         as
                                                                         u8_0,
                                                                 g:
                                                                     64 as
                                                                         libc::c_int
                                                                         as
                                                                         u8_0,
                                                                 b:
                                                                     0 as
                                                                         libc::c_int
                                                                         as
                                                                         u8_0,
                                                                 a:
                                                                     200 as
                                                                         libc::c_int
                                                                         as
                                                                         u8_0,};
                                                 init
                                             },
                                         envColorEnd:
                                             {
                                                 let mut init =
                                                     Color_RGBA8{r:
                                                                     255 as
                                                                         libc::c_int
                                                                         as
                                                                         u8_0,
                                                                 g:
                                                                     0 as
                                                                         libc::c_int
                                                                         as
                                                                         u8_0,
                                                                 b:
                                                                     0 as
                                                                         libc::c_int
                                                                         as
                                                                         u8_0,
                                                                 a:
                                                                     255 as
                                                                         libc::c_int
                                                                         as
                                                                         u8_0,};
                                                 init
                                             },
                                         deceleration: 2.1f32,
                                         maxInitialSpeed: 35.0f32,
                                         lengthCutoff: 30.0f32,
                                         duration: 8 as libc::c_int as u8_0,
                                         lightPoint:
                                             {
                                                 let mut init =
                                                     LightPoint{x:
                                                                    0 as
                                                                        libc::c_int
                                                                        as
                                                                        s16,
                                                                y:
                                                                    0 as
                                                                        libc::c_int
                                                                        as
                                                                        s16,
                                                                z:
                                                                    0 as
                                                                        libc::c_int
                                                                        as
                                                                        s16,
                                                                color:
                                                                    [0 as
                                                                         libc::c_int
                                                                         as
                                                                         u8_0,
                                                                     128 as
                                                                         libc::c_int
                                                                         as
                                                                         u8_0,
                                                                     255 as
                                                                         libc::c_int
                                                                         as
                                                                         u8_0],
                                                                drawGlow:
                                                                    0 as
                                                                        libc::c_int
                                                                        as
                                                                        u8_0,
                                                                radius:
                                                                    300 as
                                                                        libc::c_int
                                                                        as
                                                                        s16,};
                                                 init
                                             },
                                         lightDecay: 1 as libc::c_int,};
            init
        };
    let mut effectIndex: s32 = 0;
    initMetal.position.x = (*v).x as s16;
    initMetal.position.y = (*v).y as s16;
    initMetal.position.z = (*v).z as s16;
    initMetal.lightPoint.x = initMetal.position.x;
    initMetal.lightPoint.y = initMetal.position.y;
    initMetal.lightPoint.z = initMetal.position.z;
    Effect_Add(globalCtx, &mut effectIndex,
               EFFECT_SHIELD_PARTICLE as libc::c_int,
               0 as libc::c_int as u8_0, 1 as libc::c_int as u8_0,
               &mut initMetal as *mut EffectShieldParticleInit as
                   *mut libc::c_void);
}
/* *
 * Spawns streaks of light and makes a metallic sound
 */
#[no_mangle]
pub unsafe extern "C" fn CollisionCheck_SpawnShieldParticlesMetal(mut globalCtx:
                                                                      *mut GlobalContext,
                                                                  mut v:
                                                                      *mut Vec3f) {
    CollisionCheck_SpawnShieldParticles(globalCtx, v);
    Audio_PlaySoundGeneral(0x1808 as libc::c_int as u16_0, &mut D_801333D4,
                           4 as libc::c_int as u8_0, &mut D_801333E0,
                           &mut D_801333E0, &mut D_801333E8);
}
/* *
 * Spawns streaks of light and makes a metallic sound at the specified position
 */
#[no_mangle]
pub unsafe extern "C" fn CollisionCheck_SpawnShieldParticlesMetalSound(mut globalCtx:
                                                                           *mut GlobalContext,
                                                                       mut v:
                                                                           *mut Vec3f,
                                                                       mut pos:
                                                                           *mut Vec3f) {
    CollisionCheck_SpawnShieldParticles(globalCtx, v);
    Audio_PlaySoundGeneral(0x1808 as libc::c_int as u16_0, pos,
                           4 as libc::c_int as u8_0, &mut D_801333E0,
                           &mut D_801333E0, &mut D_801333E8);
}
/* *
 * Spawns streaks of light and makes a metallic sound
 */
#[no_mangle]
pub unsafe extern "C" fn CollisionCheck_SpawnShieldParticlesMetal2(mut globalCtx:
                                                                       *mut GlobalContext,
                                                                   mut v:
                                                                       *mut Vec3f) {
    CollisionCheck_SpawnShieldParticlesMetal(globalCtx, v);
}
/* *
 * Spawns streaks of light and makes a wooden sound
 */
#[no_mangle]
pub unsafe extern "C" fn CollisionCheck_SpawnShieldParticlesWood(mut globalCtx:
                                                                     *mut GlobalContext,
                                                                 mut v:
                                                                     *mut Vec3f,
                                                                 mut actorPos:
                                                                     *mut Vec3f) {
    static mut initWood: EffectShieldParticleInit =
        {
            let mut init =
                EffectShieldParticleInit{numElements:
                                             16 as libc::c_int as u8_0,
                                         position:
                                             {
                                                 let mut init =
                                                     Vec3s{x:
                                                               0 as
                                                                   libc::c_int
                                                                   as s16,
                                                           y:
                                                               0 as
                                                                   libc::c_int
                                                                   as s16,
                                                           z:
                                                               0 as
                                                                   libc::c_int
                                                                   as s16,};
                                                 init
                                             },
                                         primColorStart:
                                             {
                                                 let mut init =
                                                     Color_RGBA8{r:
                                                                     0 as
                                                                         libc::c_int
                                                                         as
                                                                         u8_0,
                                                                 g:
                                                                     200 as
                                                                         libc::c_int
                                                                         as
                                                                         u8_0,
                                                                 b:
                                                                     255 as
                                                                         libc::c_int
                                                                         as
                                                                         u8_0,
                                                                 a:
                                                                     255 as
                                                                         libc::c_int
                                                                         as
                                                                         u8_0,};
                                                 init
                                             },
                                         envColorStart:
                                             {
                                                 let mut init =
                                                     Color_RGBA8{r:
                                                                     255 as
                                                                         libc::c_int
                                                                         as
                                                                         u8_0,
                                                                 g:
                                                                     255 as
                                                                         libc::c_int
                                                                         as
                                                                         u8_0,
                                                                 b:
                                                                     255 as
                                                                         libc::c_int
                                                                         as
                                                                         u8_0,
                                                                 a:
                                                                     255 as
                                                                         libc::c_int
                                                                         as
                                                                         u8_0,};
                                                 init
                                             },
                                         primColorMid:
                                             {
                                                 let mut init =
                                                     Color_RGBA8{r:
                                                                     255 as
                                                                         libc::c_int
                                                                         as
                                                                         u8_0,
                                                                 g:
                                                                     255 as
                                                                         libc::c_int
                                                                         as
                                                                         u8_0,
                                                                 b:
                                                                     128 as
                                                                         libc::c_int
                                                                         as
                                                                         u8_0,
                                                                 a:
                                                                     255 as
                                                                         libc::c_int
                                                                         as
                                                                         u8_0,};
                                                 init
                                             },
                                         envColorMid:
                                             {
                                                 let mut init =
                                                     Color_RGBA8{r:
                                                                     255 as
                                                                         libc::c_int
                                                                         as
                                                                         u8_0,
                                                                 g:
                                                                     255 as
                                                                         libc::c_int
                                                                         as
                                                                         u8_0,
                                                                 b:
                                                                     0 as
                                                                         libc::c_int
                                                                         as
                                                                         u8_0,
                                                                 a:
                                                                     255 as
                                                                         libc::c_int
                                                                         as
                                                                         u8_0,};
                                                 init
                                             },
                                         primColorEnd:
                                             {
                                                 let mut init =
                                                     Color_RGBA8{r:
                                                                     255 as
                                                                         libc::c_int
                                                                         as
                                                                         u8_0,
                                                                 g:
                                                                     64 as
                                                                         libc::c_int
                                                                         as
                                                                         u8_0,
                                                                 b:
                                                                     0 as
                                                                         libc::c_int
                                                                         as
                                                                         u8_0,
                                                                 a:
                                                                     200 as
                                                                         libc::c_int
                                                                         as
                                                                         u8_0,};
                                                 init
                                             },
                                         envColorEnd:
                                             {
                                                 let mut init =
                                                     Color_RGBA8{r:
                                                                     255 as
                                                                         libc::c_int
                                                                         as
                                                                         u8_0,
                                                                 g:
                                                                     0 as
                                                                         libc::c_int
                                                                         as
                                                                         u8_0,
                                                                 b:
                                                                     0 as
                                                                         libc::c_int
                                                                         as
                                                                         u8_0,
                                                                 a:
                                                                     255 as
                                                                         libc::c_int
                                                                         as
                                                                         u8_0,};
                                                 init
                                             },
                                         deceleration: 2.1f32,
                                         maxInitialSpeed: 35.0f32,
                                         lengthCutoff: 30.0f32,
                                         duration: 8 as libc::c_int as u8_0,
                                         lightPoint:
                                             {
                                                 let mut init =
                                                     LightPoint{x:
                                                                    0 as
                                                                        libc::c_int
                                                                        as
                                                                        s16,
                                                                y:
                                                                    0 as
                                                                        libc::c_int
                                                                        as
                                                                        s16,
                                                                z:
                                                                    0 as
                                                                        libc::c_int
                                                                        as
                                                                        s16,
                                                                color:
                                                                    [0 as
                                                                         libc::c_int
                                                                         as
                                                                         u8_0,
                                                                     128 as
                                                                         libc::c_int
                                                                         as
                                                                         u8_0,
                                                                     255 as
                                                                         libc::c_int
                                                                         as
                                                                         u8_0],
                                                                drawGlow:
                                                                    0 as
                                                                        libc::c_int
                                                                        as
                                                                        u8_0,
                                                                radius:
                                                                    300 as
                                                                        libc::c_int
                                                                        as
                                                                        s16,};
                                                 init
                                             },
                                         lightDecay: 0 as libc::c_int,};
            init
        };
    let mut effectIndex: s32 = 0;
    initWood.position.x = (*v).x as s16;
    initWood.position.y = (*v).y as s16;
    initWood.position.z = (*v).z as s16;
    initWood.lightPoint.x = initWood.position.x;
    initWood.lightPoint.y = initWood.position.y;
    initWood.lightPoint.z = initWood.position.z;
    Effect_Add(globalCtx, &mut effectIndex,
               EFFECT_SHIELD_PARTICLE as libc::c_int,
               0 as libc::c_int as u8_0, 1 as libc::c_int as u8_0,
               &mut initWood as *mut EffectShieldParticleInit as
                   *mut libc::c_void);
    Audio_PlaySoundGeneral(0x1837 as libc::c_int as u16_0, actorPos,
                           4 as libc::c_int as u8_0, &mut D_801333E0,
                           &mut D_801333E0, &mut D_801333E8);
}
/* *
 * Determines if the line segment connecting itemPos and itemProjPos intersects the side of a cylinder with the given
 * radius, height, and offset at actorPos. Returns 3 if either endpoint is inside the cylinder, otherwise returns the
 * number of points of intersection with the side of the cylinder. The locations of those points are put in out1 and
 * out2, with out1 being closer to itemPos. Line segments that pass through both bases of the cylinder are not detected.
 */
#[no_mangle]
pub unsafe extern "C" fn CollisionCheck_CylSideVsLineSeg(mut radius: f32_0,
                                                         mut height: f32_0,
                                                         mut offset: f32_0,
                                                         mut actorPos:
                                                             *mut Vec3f,
                                                         mut itemPos:
                                                             *mut Vec3f,
                                                         mut itemProjPos:
                                                             *mut Vec3f,
                                                         mut out1: *mut Vec3f,
                                                         mut out2: *mut Vec3f)
 -> s32 {
    let mut actorToItem: Vec3f = Vec3f{x: 0., y: 0., z: 0.,};
    let mut actorToItemProj: Vec3f = Vec3f{x: 0., y: 0., z: 0.,};
    let mut itemStep: Vec3f = Vec3f{x: 0., y: 0., z: 0.,};
    let mut frac1: f32_0 = 0.;
    let mut frac2: f32_0 = 0.;
    let mut intersect2: u32_0 = 0;
    let mut intersect1: u32_0 = 0;
    let mut test1: u32_0 = 0;
    let mut test2: u32_0 = 0;
    let mut radSqDiff: f32_0 = 0.;
    let mut actorDotItemXZ: f32_0 = 0.;
    let mut zero: f32_0 = 0.0f32;
    let mut closeDist: f32_0 = 0.;
    let mut pad1: s32 = 0;
    let mut pad2: s32 = 0;
    actorToItem.x = (*itemPos).x - (*actorPos).x;
    actorToItem.y = (*itemPos).y - (*actorPos).y - offset;
    actorToItem.z = (*itemPos).z - (*actorPos).z;
    actorToItemProj.x = (*itemProjPos).x - (*actorPos).x;
    actorToItemProj.y = (*itemProjPos).y - (*actorPos).y - offset;
    actorToItemProj.z = (*itemProjPos).z - (*actorPos).z;
    itemStep.x = actorToItemProj.x - actorToItem.x;
    itemStep.y = actorToItemProj.y - actorToItem.y;
    itemStep.z = actorToItemProj.z - actorToItem.z;
    if actorToItem.y > 0.0f32 && actorToItem.y < height &&
           sqrtf(actorToItem.x * actorToItem.x +
                     actorToItem.z * actorToItem.z) < radius {
        return 3 as libc::c_int
    }
    if actorToItemProj.y > 0.0f32 && actorToItemProj.y < height &&
           sqrtf(actorToItemProj.x * actorToItemProj.x +
                     actorToItemProj.z * actorToItemProj.z) < radius {
        return 3 as libc::c_int
    }
    radSqDiff =
        actorToItem.x * actorToItem.x + actorToItem.z * actorToItem.z -
            radius * radius;
    if !(fabsf(itemStep.x * itemStep.x + itemStep.z * itemStep.z) < 0.008f32)
       {
        actorDotItemXZ =
            2.0f32 * itemStep.x * actorToItem.x +
                2.0f32 * itemStep.z * actorToItem.z;
        if actorDotItemXZ * actorDotItemXZ <
               4.0f32 * (itemStep.x * itemStep.x + itemStep.z * itemStep.z) *
                   radSqDiff {
            return 0 as libc::c_int
        }
        if actorDotItemXZ * actorDotItemXZ -
               4.0f32 * (itemStep.x * itemStep.x + itemStep.z * itemStep.z) *
                   radSqDiff > zero {
            intersect2 = 1 as libc::c_int as u32_0;
            intersect1 = intersect2
        } else {
            intersect1 = 1 as libc::c_int as u32_0;
            intersect2 = 0 as libc::c_int as u32_0
        }
        closeDist =
            sqrtf(actorDotItemXZ * actorDotItemXZ -
                      4.0f32 *
                          (itemStep.x * itemStep.x + itemStep.z * itemStep.z)
                          * radSqDiff);
        if intersect1 == 1 as libc::c_int as libc::c_uint {
            frac1 =
                (closeDist - actorDotItemXZ) /
                    (2.0f32 *
                         (itemStep.x * itemStep.x + itemStep.z * itemStep.z))
        }
        if intersect2 == 1 as libc::c_int as libc::c_uint {
            frac2 =
                (-actorDotItemXZ - closeDist) /
                    (2.0f32 *
                         (itemStep.x * itemStep.x + itemStep.z * itemStep.z))
        }
    } else if !(fabsf(2.0f32 * itemStep.x * actorToItem.x +
                          2.0f32 * itemStep.z * actorToItem.z) < 0.008f32) {
        intersect1 = 1 as libc::c_int as u32_0;
        intersect2 = 0 as libc::c_int as u32_0;
        frac1 =
            -radSqDiff /
                (2.0f32 * itemStep.x * actorToItem.x +
                     2.0f32 * itemStep.z * actorToItem.z)
    } else {
        if radSqDiff <= 0.0f32 {
            test1 =
                (0.0f32 < actorToItem.y && actorToItem.y < height) as
                    libc::c_int as u32_0;
            test2 =
                (0.0f32 < actorToItemProj.y && actorToItemProj.y < height) as
                    libc::c_int as u32_0;
            if test1 != 0 && test2 != 0 {
                *out1 = actorToItem;
                *out2 = actorToItemProj;
                return 2 as libc::c_int
            }
            if test1 != 0 { *out1 = actorToItem; return 1 as libc::c_int }
            if test2 != 0 { *out1 = actorToItemProj; return 1 as libc::c_int }
        }
        return 0 as libc::c_int
    }
    if intersect2 == 0 as libc::c_int as libc::c_uint {
        if frac1 < 0.0f32 || 1.0f32 < frac1 { return 0 as libc::c_int }
    } else {
        test1 = (frac1 < 0.0f32 || 1.0f32 < frac1) as libc::c_int as u32_0;
        test2 = (frac2 < 0.0f32 || 1.0f32 < frac2) as libc::c_int as u32_0;
        if test1 != 0 && test2 != 0 { return 0 as libc::c_int }
        if test1 != 0 { intersect1 = 0 as libc::c_int as u32_0 }
        if test2 != 0 { intersect2 = 0 as libc::c_int as u32_0 }
    }
    if intersect1 == 1 as libc::c_int as libc::c_uint &&
           (frac1 * itemStep.y + actorToItem.y < 0.0f32 ||
                height < frac1 * itemStep.y + actorToItem.y) {
        intersect1 = 0 as libc::c_int as u32_0
    }
    if intersect2 == 1 as libc::c_int as libc::c_uint &&
           (frac2 * itemStep.y + actorToItem.y < 0.0f32 ||
                height < frac2 * itemStep.y + actorToItem.y) {
        intersect2 = 0 as libc::c_int as u32_0
    }
    if intersect1 == 0 as libc::c_int as libc::c_uint &&
           intersect2 == 0 as libc::c_int as libc::c_uint {
        return 0 as libc::c_int
    } else {
        if intersect1 == 1 as libc::c_int as libc::c_uint &&
               intersect2 == 1 as libc::c_int as libc::c_uint {
            (*out1).x = frac1 * itemStep.x + actorToItem.x + (*actorPos).x;
            (*out1).y = frac1 * itemStep.y + actorToItem.y + (*actorPos).y;
            (*out1).z = frac1 * itemStep.z + actorToItem.z + (*actorPos).z;
            (*out2).x = frac2 * itemStep.x + actorToItem.x + (*actorPos).x;
            (*out2).y = frac2 * itemStep.y + actorToItem.y + (*actorPos).y;
            (*out2).z = frac2 * itemStep.z + actorToItem.z + (*actorPos).z;
            return 2 as libc::c_int
        } else {
            if intersect1 == 1 as libc::c_int as libc::c_uint {
                (*out1).x =
                    frac1 * itemStep.x + actorToItem.x + (*actorPos).x;
                (*out1).y =
                    frac1 * itemStep.y + actorToItem.y + (*actorPos).y;
                (*out1).z =
                    frac1 * itemStep.z + actorToItem.z + (*actorPos).z;
                return 1 as libc::c_int
            } else {
                if intersect2 == 1 as libc::c_int as libc::c_uint {
                    (*out1).x =
                        frac2 * itemStep.x + actorToItem.x + (*actorPos).x;
                    (*out1).y =
                        frac2 * itemStep.y + actorToItem.y + (*actorPos).y;
                    (*out1).z =
                        frac2 * itemStep.z + actorToItem.z + (*actorPos).z;
                    return 1 as libc::c_int
                }
            }
        }
    }
    return 1 as libc::c_int;
}
/* *
 * Gets damage from a sword strike using generic values, and returns 0 if the attack is
 * not sword-type. Used by bosses to require that a sword attack deal the killing blow.
 */
#[no_mangle]
pub unsafe extern "C" fn CollisionCheck_GetSwordDamage(mut dmgFlags: s32)
 -> u8_0 {
    let mut damage: u8_0 = 0 as libc::c_int as u8_0;
    if dmgFlags & 0x400100 as libc::c_int != 0 {
        damage = 1 as libc::c_int as u8_0
    } else if dmgFlags & 0x3000242 as libc::c_int != 0 {
        damage = 2 as libc::c_int as u8_0
    } else if dmgFlags & 0x48800400 as libc::c_int != 0 {
        damage = 4 as libc::c_int as u8_0
    } else if dmgFlags & 0x4000000 as libc::c_int != 0 {
        damage = 8 as libc::c_int as u8_0
    }
    (*gGameInfo).data[(13 as libc::c_int * 6 as libc::c_int *
                           16 as libc::c_int + 7 as libc::c_int) as usize] =
        damage as s16;
    return damage;
}
