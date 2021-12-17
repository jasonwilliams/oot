#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case,
         non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(const_raw_ptr_to_usize_cast, ptr_wrapping_offset_from,
           register_tool)]
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
    fn LogUtils_HungupThread(name: *const libc::c_char, line: s32);
    #[no_mangle]
    fn Actor_SetObjectDependency(globalCtx: *mut GlobalContext,
                                 actor: *mut Actor);
    #[no_mangle]
    fn func_800434A0(dynaActor: *mut DynaPolyActor);
    #[no_mangle]
    fn Collider_DrawPoly(gfxCtx: *mut GraphicsContext, vA: *mut Vec3f,
                         vB: *mut Vec3f, vC: *mut Vec3f, r: u8_0, g: u8_0,
                         b: u8_0);
    #[no_mangle]
    fn Math_Vec3f_Copy(dest: *mut Vec3f, src: *mut Vec3f);
    #[no_mangle]
    fn Math_Vec3s_ToVec3f(dest: *mut Vec3f, src: *mut Vec3s);
    #[no_mangle]
    fn SkinMatrix_Vec3fMtxFMultXYZ(mf: *mut MtxF, src: *mut Vec3f,
                                   dest: *mut Vec3f);
    #[no_mangle]
    fn SkinMatrix_SetTranslateRotateYXZScale(dest: *mut MtxF, scaleX: f32_0,
                                             scaleY: f32_0, scaleZ: f32_0,
                                             rotX: s16, rotY: s16, rotZ: s16,
                                             translateX: f32_0,
                                             translateY: f32_0,
                                             translateZ: f32_0);
    #[no_mangle]
    fn THA_AllocEndAlign(tha: *mut TwoHeadArena, size: u32_0, mask: u32_0)
     -> *mut libc::c_void;
    #[no_mangle]
    fn GameState_Alloc(gameState: *mut GameState, size: size_t,
                       file: *mut libc::c_char, line: s32)
     -> *mut libc::c_void;
    #[no_mangle]
    fn Math3D_PlaneVsLineSegClosestPoint(planeAA: f32_0, planeAB: f32_0,
                                         planeAC: f32_0, planeADist: f32_0,
                                         planeBA: f32_0, planeBB: f32_0,
                                         planeBC: f32_0, planeBDist: f32_0,
                                         linePointA: *mut Vec3f,
                                         linePointB: *mut Vec3f,
                                         closestPoint: *mut Vec3f) -> s32;
    #[no_mangle]
    fn Math3D_PlaneVsPlaneVsLineClosestPoint(planeAA: f32_0, planeAB: f32_0,
                                             planeAC: f32_0,
                                             planeADist: f32_0,
                                             planeBA: f32_0, planeBB: f32_0,
                                             planeBC: f32_0,
                                             planeBDist: f32_0,
                                             point: *mut Vec3f,
                                             closestPoint: *mut Vec3f) -> s32;
    #[no_mangle]
    fn Math3D_LineSplitRatio(v0: *mut Vec3f, v1: *mut Vec3f, ratio: f32_0,
                             ret: *mut Vec3f);
    #[no_mangle]
    fn Math3D_Vec3fMagnitude(vec: *mut Vec3f) -> f32_0;
    #[no_mangle]
    fn Math3D_Vec3fDistSq(a: *mut Vec3f, b: *mut Vec3f) -> f32_0;
    #[no_mangle]
    fn Math3D_SurfaceNorm(va: *mut Vec3f, vb: *mut Vec3f, vc: *mut Vec3f,
                          normal: *mut Vec3f);
    #[no_mangle]
    fn Math3D_PointRelativeToCubeFaces(point: *mut Vec3f, min: *mut Vec3f,
                                       max: *mut Vec3f) -> s32;
    #[no_mangle]
    fn Math3D_PointRelativeToCubeEdges(point: *mut Vec3f, min: *mut Vec3f,
                                       max: *mut Vec3f) -> s32;
    #[no_mangle]
    fn Math3D_PointRelativeToCubeVertices(point: *mut Vec3f, min: *mut Vec3f,
                                          max: *mut Vec3f) -> s32;
    #[no_mangle]
    fn Math3D_LineVsCube(min: *mut Vec3f, max: *mut Vec3f, a: *mut Vec3f,
                         b: *mut Vec3f) -> s32;
    #[no_mangle]
    fn Math3D_UDistPlaneToPos(nx: f32_0, ny: f32_0, nz: f32_0,
                              originDist: f32_0, p: *mut Vec3f) -> f32_0;
    #[no_mangle]
    fn Math3D_DistPlaneToPos(nx: f32_0, ny: f32_0, nz: f32_0,
                             originDist: f32_0, p: *mut Vec3f) -> f32_0;
    #[no_mangle]
    fn Math3D_TriChkPointParaYIntersectDist(v0: *mut Vec3f, v1: *mut Vec3f,
                                            v2: *mut Vec3f, nx: f32_0,
                                            ny: f32_0, nz: f32_0,
                                            originDist: f32_0, z: f32_0,
                                            x: f32_0, yIntersect: *mut f32_0,
                                            chkDist: f32_0) -> s32;
    #[no_mangle]
    fn Math3D_TriChkPointParaYIntersectInsideTri(v0: *mut Vec3f,
                                                 v1: *mut Vec3f,
                                                 v2: *mut Vec3f, nx: f32_0,
                                                 ny: f32_0, nz: f32_0,
                                                 originDist: f32_0, z: f32_0,
                                                 x: f32_0,
                                                 yIntersect: *mut f32_0,
                                                 chkDist: f32_0) -> s32;
    #[no_mangle]
    fn Math3D_TriChkLineSegParaYIntersect(v0: *mut Vec3f, v1: *mut Vec3f,
                                          v2: *mut Vec3f, nx: f32_0,
                                          ny: f32_0, nz: f32_0,
                                          originDist: f32_0, z: f32_0,
                                          x: f32_0, yIntersect: *mut f32_0,
                                          y0: f32_0, y1: f32_0) -> s32;
    #[no_mangle]
    fn Math3D_TriChkPointParaYDist(v0: *mut Vec3f, v1: *mut Vec3f,
                                   v2: *mut Vec3f, plane: *mut Plane,
                                   z: f32_0, x: f32_0, chkDist: f32_0) -> s32;
    #[no_mangle]
    fn Math3D_TriChkPointParaXIntersect(v0: *mut Vec3f, v1: *mut Vec3f,
                                        v2: *mut Vec3f, nx: f32_0, ny: f32_0,
                                        nz: f32_0, originDist: f32_0,
                                        y: f32_0, z: f32_0,
                                        xIntersect: *mut f32_0) -> s32;
    #[no_mangle]
    fn Math3D_TriChkLineSegParaXIntersect(v0: *mut Vec3f, v1: *mut Vec3f,
                                          v2: *mut Vec3f, nx: f32_0,
                                          ny: f32_0, nz: f32_0,
                                          originDist: f32_0, y: f32_0,
                                          z: f32_0, xIntersect: *mut f32_0,
                                          x0: f32_0, x1: f32_0) -> s32;
    #[no_mangle]
    fn Math3D_TriChkPointParaXDist(v0: *mut Vec3f, v1: *mut Vec3f,
                                   v2: *mut Vec3f, plane: *mut Plane,
                                   y: f32_0, z: f32_0, chkDist: f32_0) -> s32;
    #[no_mangle]
    fn Math3D_TriChkPointParaZIntersect(v0: *mut Vec3f, v1: *mut Vec3f,
                                        v2: *mut Vec3f, nx: f32_0, ny: f32_0,
                                        nz: f32_0, originDist: f32_0,
                                        x: f32_0, y: f32_0,
                                        zIntersect: *mut f32_0) -> s32;
    #[no_mangle]
    fn Math3D_TriChkLineSegParaZIntersect(v0: *mut Vec3f, v1: *mut Vec3f,
                                          v2: *mut Vec3f, nx: f32_0,
                                          ny: f32_0, nz: f32_0,
                                          originDist: f32_0, x: f32_0,
                                          y: f32_0, zIntersect: *mut f32_0,
                                          z0: f32_0, z1: f32_0) -> s32;
    #[no_mangle]
    fn Math3D_TriChkLineSegParaZDist(v0: *mut Vec3f, v1: *mut Vec3f,
                                     v2: *mut Vec3f, plane: *mut Plane,
                                     x: f32_0, y: f32_0, chkDist: f32_0)
     -> s32;
    #[no_mangle]
    fn Math3D_LineVsSph(sphere: *mut Sphere16, line: *mut Linef) -> s32;
    #[no_mangle]
    fn Math3D_TriVsSphIntersect(sphere: *mut Sphere16, tri: *mut TriNorm,
                                intersectPoint: *mut Vec3f) -> s32;
    #[no_mangle]
    fn Math3D_SphVsSph(sphereA: *mut Sphere16, sphereB: *mut Sphere16) -> s32;
    #[no_mangle]
    fn Math3D_XZInSphere(sphere: *mut Sphere16, x: f32_0, z: f32_0) -> s32;
    #[no_mangle]
    fn Math3D_XYInSphere(sphere: *mut Sphere16, x: f32_0, y: f32_0) -> s32;
    #[no_mangle]
    fn Math3D_YZInSphere(sphere: *mut Sphere16, y: f32_0, z: f32_0) -> s32;
    #[no_mangle]
    static mut gGameInfo: *mut GameInfo;
    #[no_mangle]
    static mut gSegments: [u32_0; 16];
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
pub struct DynaPolyActor {
    pub actor: Actor,
    pub bgId: s32,
    pub unk_150: f32_0,
    pub unk_154: f32_0,
    pub unk_158: s16,
    pub unk_15A: u16_0,
    pub unk_15C: u32_0,
    pub unk_160: u8_0,
    pub unk_162: s16,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct DynaRaycast {
    pub globalCtx: *mut GlobalContext,
    pub colCtx: *mut CollisionContext,
    pub xpFlags: u16_0,
    pub resultPoly: *mut *mut CollisionPoly,
    pub yIntersect: f32_0,
    pub pos: *mut Vec3f,
    pub bgId: *mut s32,
    pub actor: *mut Actor,
    pub unk_20: u32_0,
    pub chkDist: f32_0,
    pub dyna: *mut DynaCollisionContext,
    pub ssList: *mut SSList,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct DynaLineTest {
    pub colCtx: *mut CollisionContext,
    pub xpFlags: u16_0,
    pub dyna: *mut DynaCollisionContext,
    pub ssList: *mut SSList,
    pub posA: *mut Vec3f,
    pub posB: *mut Vec3f,
    pub posResult: *mut Vec3f,
    pub resultPoly: *mut *mut CollisionPoly,
    pub chkOneFace: s32,
    pub distSq: *mut f32_0,
    pub chkDist: f32_0,
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
pub const SCENE_ID_MAX: C2RustUnnamed_15 = 110;
pub const SCENE_TESTROOM: C2RustUnnamed_15 = 109;
pub const SCENE_SASATEST: C2RustUnnamed_15 = 108;
pub const SCENE_HAIRAL_NIWA2: C2RustUnnamed_15 = 107;
pub const SCENE_SUTARU: C2RustUnnamed_15 = 106;
pub const SCENE_SYOTES2: C2RustUnnamed_15 = 105;
pub const SCENE_SYOTES: C2RustUnnamed_15 = 104;
pub const SCENE_DEPTH_TEST: C2RustUnnamed_15 = 103;
pub const SCENE_BESITU: C2RustUnnamed_15 = 102;
pub const SCENE_TEST01: C2RustUnnamed_15 = 101;
pub const SCENE_GANON_TOU: C2RustUnnamed_15 = 100;
pub const SCENE_SPOT20: C2RustUnnamed_15 = 99;
pub const SCENE_SPOT18: C2RustUnnamed_15 = 98;
pub const SCENE_SPOT17: C2RustUnnamed_15 = 97;
pub const SCENE_SPOT16: C2RustUnnamed_15 = 96;
pub const SCENE_SPOT15: C2RustUnnamed_15 = 95;
pub const SCENE_SPOT13: C2RustUnnamed_15 = 94;
pub const SCENE_SPOT12: C2RustUnnamed_15 = 93;
pub const SCENE_SPOT11: C2RustUnnamed_15 = 92;
pub const SCENE_SPOT10: C2RustUnnamed_15 = 91;
pub const SCENE_SPOT09: C2RustUnnamed_15 = 90;
pub const SCENE_SPOT08: C2RustUnnamed_15 = 89;
pub const SCENE_SPOT07: C2RustUnnamed_15 = 88;
pub const SCENE_SPOT06: C2RustUnnamed_15 = 87;
pub const SCENE_SPOT05: C2RustUnnamed_15 = 86;
pub const SCENE_SPOT04: C2RustUnnamed_15 = 85;
pub const SCENE_SPOT03: C2RustUnnamed_15 = 84;
pub const SCENE_SPOT02: C2RustUnnamed_15 = 83;
pub const SCENE_SPOT01: C2RustUnnamed_15 = 82;
pub const SCENE_SPOT00: C2RustUnnamed_15 = 81;
pub const SCENE_KINSUTA: C2RustUnnamed_15 = 80;
pub const SCENE_GANON_DEMO: C2RustUnnamed_15 = 79;
pub const SCENE_MAHOUYA: C2RustUnnamed_15 = 78;
pub const SCENE_MIHARIGOYA: C2RustUnnamed_15 = 77;
pub const SCENE_SOUKO: C2RustUnnamed_15 = 76;
pub const SCENE_BOWLING: C2RustUnnamed_15 = 75;
pub const SCENE_NAKANIWA: C2RustUnnamed_15 = 74;
pub const SCENE_TURIBORI: C2RustUnnamed_15 = 73;
pub const SCENE_HAKASITARELAY: C2RustUnnamed_15 = 72;
pub const SCENE_HIRAL_DEMO: C2RustUnnamed_15 = 71;
pub const SCENE_HAIRAL_NIWA_N: C2RustUnnamed_15 = 70;
pub const SCENE_HAIRAL_NIWA: C2RustUnnamed_15 = 69;
pub const SCENE_KENJYANOMA: C2RustUnnamed_15 = 68;
pub const SCENE_TOKINOMA: C2RustUnnamed_15 = 67;
pub const SCENE_SYATEKIJYOU: C2RustUnnamed_15 = 66;
pub const SCENE_HAKAANA_OUKE: C2RustUnnamed_15 = 65;
pub const SCENE_HAKAANA2: C2RustUnnamed_15 = 64;
pub const SCENE_HAKAANA: C2RustUnnamed_15 = 63;
pub const SCENE_KAKUSIANA: C2RustUnnamed_15 = 62;
pub const SCENE_YOUSEI_IZUMI_YOKO: C2RustUnnamed_15 = 61;
pub const SCENE_YOUSEI_IZUMI_TATE: C2RustUnnamed_15 = 60;
pub const SCENE_DAIYOUSEI_IZUMI: C2RustUnnamed_15 = 59;
pub const SCENE_HUT: C2RustUnnamed_15 = 58;
pub const SCENE_TENT: C2RustUnnamed_15 = 57;
pub const SCENE_HYLIA_LABO: C2RustUnnamed_15 = 56;
pub const SCENE_LABO: C2RustUnnamed_15 = 55;
pub const SCENE_MALON_STABLE: C2RustUnnamed_15 = 54;
pub const SCENE_IMPA: C2RustUnnamed_15 = 53;
pub const SCENE_LINK_HOME: C2RustUnnamed_15 = 52;
pub const SCENE_FACE_SHOP: C2RustUnnamed_15 = 51;
pub const SCENE_NIGHT_SHOP: C2RustUnnamed_15 = 50;
pub const SCENE_ALLEY_SHOP: C2RustUnnamed_15 = 49;
pub const SCENE_DRAG: C2RustUnnamed_15 = 48;
pub const SCENE_ZOORA: C2RustUnnamed_15 = 47;
pub const SCENE_GOLON: C2RustUnnamed_15 = 46;
pub const SCENE_KOKIRI_SHOP: C2RustUnnamed_15 = 45;
pub const SCENE_SHOP1: C2RustUnnamed_15 = 44;
pub const SCENE_KAKARIKO3: C2RustUnnamed_15 = 43;
pub const SCENE_KAKARIKO: C2RustUnnamed_15 = 42;
pub const SCENE_KOKIRI_HOME5: C2RustUnnamed_15 = 41;
pub const SCENE_KOKIRI_HOME4: C2RustUnnamed_15 = 40;
pub const SCENE_KOKIRI_HOME3: C2RustUnnamed_15 = 39;
pub const SCENE_KOKIRI_HOME: C2RustUnnamed_15 = 38;
pub const SCENE_SHRINE_R: C2RustUnnamed_15 = 37;
pub const SCENE_SHRINE_N: C2RustUnnamed_15 = 36;
pub const SCENE_SHRINE: C2RustUnnamed_15 = 35;
pub const SCENE_MARKET_RUINS: C2RustUnnamed_15 = 34;
pub const SCENE_MARKET_NIGHT: C2RustUnnamed_15 = 33;
pub const SCENE_MARKET_DAY: C2RustUnnamed_15 = 32;
pub const SCENE_MARKET_ALLEY_N: C2RustUnnamed_15 = 31;
pub const SCENE_MARKET_ALLEY: C2RustUnnamed_15 = 30;
pub const SCENE_ENRUI: C2RustUnnamed_15 = 29;
pub const SCENE_ENTRA_N: C2RustUnnamed_15 = 28;
pub const SCENE_ENTRA: C2RustUnnamed_15 = 27;
pub const SCENE_GANON_FINAL: C2RustUnnamed_15 = 26;
pub const SCENE_GANON_BOSS: C2RustUnnamed_15 = 25;
pub const SCENE_HAKADAN_BS: C2RustUnnamed_15 = 24;
pub const SCENE_JYASINBOSS: C2RustUnnamed_15 = 23;
pub const SCENE_MIZUSIN_BS: C2RustUnnamed_15 = 22;
pub const SCENE_FIRE_BS: C2RustUnnamed_15 = 21;
pub const SCENE_MORIBOSSROOM: C2RustUnnamed_15 = 20;
pub const SCENE_BDAN_BOSS: C2RustUnnamed_15 = 19;
pub const SCENE_DDAN_BOSS: C2RustUnnamed_15 = 18;
pub const SCENE_YDAN_BOSS: C2RustUnnamed_15 = 17;
pub const SCENE_TAKARAYA: C2RustUnnamed_15 = 16;
pub const SCENE_GANONTIKA_SONOGO: C2RustUnnamed_15 = 15;
pub const SCENE_GANON_SONOGO: C2RustUnnamed_15 = 14;
pub const SCENE_GANONTIKA: C2RustUnnamed_15 = 13;
pub const SCENE_GERUDOWAY: C2RustUnnamed_15 = 12;
pub const SCENE_MEN: C2RustUnnamed_15 = 11;
pub const SCENE_GANON: C2RustUnnamed_15 = 10;
pub const SCENE_ICE_DOUKUTO: C2RustUnnamed_15 = 9;
pub const SCENE_HAKADANCH: C2RustUnnamed_15 = 8;
pub const SCENE_HAKADAN: C2RustUnnamed_15 = 7;
pub const SCENE_JYASINZOU: C2RustUnnamed_15 = 6;
pub const SCENE_MIZUSIN: C2RustUnnamed_15 = 5;
pub const SCENE_HIDAN: C2RustUnnamed_15 = 4;
pub const SCENE_BMORI1: C2RustUnnamed_15 = 3;
pub const SCENE_BDAN: C2RustUnnamed_15 = 2;
pub const SCENE_DDAN: C2RustUnnamed_15 = 1;
pub const SCENE_YDAN: C2RustUnnamed_15 = 0;
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
pub struct BgCheckSceneSubdivisionEntry {
    pub sceneId: s16,
    pub subdivAmount: Vec3s,
    pub nodeListMax: s32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct BgCheckSceneMemEntry {
    pub sceneId: s16,
    pub memSize: u32_0,
}
// func_80041DB8, SurfaceType wall properties
#[no_mangle]
pub static mut D_80119D90: [s32; 32] =
    [0 as libc::c_int, 1 as libc::c_int, 3 as libc::c_int, 5 as libc::c_int,
     8 as libc::c_int, 16 as libc::c_int, 32 as libc::c_int,
     64 as libc::c_int, 0 as libc::c_int, 0 as libc::c_int, 0 as libc::c_int,
     0 as libc::c_int, 0 as libc::c_int, 0 as libc::c_int, 0 as libc::c_int,
     0 as libc::c_int, 0 as libc::c_int, 0 as libc::c_int, 0 as libc::c_int,
     0 as libc::c_int, 0 as libc::c_int, 0 as libc::c_int, 0 as libc::c_int,
     0 as libc::c_int, 0 as libc::c_int, 0 as libc::c_int, 0 as libc::c_int,
     0 as libc::c_int, 0 as libc::c_int, 0 as libc::c_int, 0 as libc::c_int,
     0 as libc::c_int];
// SurfaceType_GetSfx
#[no_mangle]
pub static mut D_80119E10: [u16_0; 14] =
    [(0x800 as libc::c_int - 0x800 as libc::c_int) as u16_0,
     (0x801 as libc::c_int - 0x800 as libc::c_int) as u16_0,
     (0x802 as libc::c_int - 0x800 as libc::c_int) as u16_0,
     (0x803 as libc::c_int - 0x800 as libc::c_int) as u16_0,
     (0x804 as libc::c_int - 0x800 as libc::c_int) as u16_0,
     (0x805 as libc::c_int - 0x800 as libc::c_int) as u16_0,
     (0x806 as libc::c_int - 0x800 as libc::c_int) as u16_0,
     (0x807 as libc::c_int - 0x800 as libc::c_int) as u16_0,
     (0x808 as libc::c_int - 0x800 as libc::c_int) as u16_0,
     (0x80b as libc::c_int - 0x800 as libc::c_int) as u16_0,
     (0x80a as libc::c_int - 0x800 as libc::c_int) as u16_0,
     (0x800 as libc::c_int - 0x800 as libc::c_int) as u16_0,
     (0x80f as libc::c_int - 0x800 as libc::c_int) as u16_0,
     (0x809 as libc::c_int - 0x800 as libc::c_int) as u16_0];
/* *
 * original name: T_BGCheck_PosErrorCheck
 */
#[no_mangle]
pub unsafe extern "C" fn BgCheck_PosErrorCheck(mut pos: *mut Vec3f,
                                               mut file: *mut libc::c_char,
                                               mut line: s32) -> s32 {
    if (*pos).x >= 32760.0f32 || (*pos).x <= -32760.0f32 ||
           (*pos).y >= 32760.0f32 || (*pos).y <= -32760.0f32 ||
           (*pos).z >= 32760.0f32 || (*pos).z <= -32760.0f32 {
        osSyncPrintf(b"\x1b[31m\x00" as *const u8 as *const libc::c_char);
        // "Position is invalid."
        osSyncPrintf(b"T_BGCheck_PosErrorCheck():\xe4\xbd\x8d\xe7\xbd\xae\xe3\x81\x8c\xe5\xa6\xa5\xe5\xbd\x93\xe3\x81\xa7\xe3\x81\xaf\xe3\x81\x82\xe3\x82\x8a\xe3\x81\xbe\xe3\x81\x9b\xe3\x82\x93\xe3\x80\x82pos (%f,%f,%f) file:%s line:%d\n\x00"
                         as *const u8 as *const libc::c_char,
                     (*pos).x as libc::c_double, (*pos).y as libc::c_double,
                     (*pos).z as libc::c_double, file, line);
        osSyncPrintf(b"\x1b[m\x00" as *const u8 as *const libc::c_char);
        return 1 as libc::c_int
    }
    return 0 as libc::c_int;
}
/* *
 * Set SSNode
 */
#[no_mangle]
pub unsafe extern "C" fn SSNode_SetValue(mut node: *mut SSNode,
                                         mut polyId: *mut s16,
                                         mut next: u16_0) {
    (*node).polyId = *polyId;
    (*node).next = next;
}
/* *
 * Set SSList to SS_NULL
 */
#[no_mangle]
pub unsafe extern "C" fn SSList_SetNull(mut ssList: *mut SSList) {
    (*ssList).head = 0xffff as libc::c_int as u16_0;
}
/* *
 * Insert `polyId` at the start of the static `ssList` list
 */
#[no_mangle]
pub unsafe extern "C" fn SSNodeList_SetSSListHead(mut nodeList:
                                                      *mut SSNodeList,
                                                  mut ssList: *mut SSList,
                                                  mut polyId: *mut s16) {
    let mut newNodeId: u16_0 = SSNodeList_GetNextNodeIdx(nodeList);
    SSNode_SetValue(&mut *(*nodeList).tbl.offset(newNodeId as isize), polyId,
                    (*ssList).head);
    (*ssList).head = newNodeId;
}
/* *
 * Insert `polyId` at the start of the dyna `ssList` list
 */
#[no_mangle]
pub unsafe extern "C" fn DynaSSNodeList_SetSSListHead(mut nodeList:
                                                          *mut DynaSSNodeList,
                                                      mut ssList: *mut SSList,
                                                      mut polyId: *mut s16) {
    let mut newNodeId: u16_0 = DynaSSNodeList_GetNextNodeIdx(nodeList);
    if newNodeId as libc::c_int != 0xffff as libc::c_int {
    } else {
        __assert(b"new_node != SS_NULL\x00" as *const u8 as
                     *const libc::c_char,
                 b"../z_bgcheck.c\x00" as *const u8 as *const libc::c_char,
                 1776 as libc::c_int);
    };
    SSNode_SetValue(&mut *(*nodeList).tbl.offset(newNodeId as isize), polyId,
                    (*ssList).head);
    (*ssList).head = newNodeId;
}
/* *
 * Initialize DynaSSNodeList
 */
#[no_mangle]
pub unsafe extern "C" fn DynaSSNodeList_Initialize(mut globalCtx:
                                                       *mut GlobalContext,
                                                   mut nodeList:
                                                       *mut DynaSSNodeList) {
    (*nodeList).tbl = 0 as *mut SSNode;
    (*nodeList).count = 0 as libc::c_int;
}
/* *
 * Initialize DynaSSNodeList tbl
 */
#[no_mangle]
pub unsafe extern "C" fn DynaSSNodeList_Alloc(mut globalCtx:
                                                  *mut GlobalContext,
                                              mut nodeList:
                                                  *mut DynaSSNodeList,
                                              mut max: s32) {
    (*nodeList).tbl =
        THA_AllocEndAlign(&mut (*globalCtx).state.tha,
                          (max as
                               libc::c_uint).wrapping_mul(::std::mem::size_of::<SSNode>()
                                                              as
                                                              libc::c_ulong),
                          -(2 as libc::c_int) as u32_0) as *mut SSNode;
    if !(*nodeList).tbl.is_null() {
    } else {
        __assert(b"psst->tbl != NULL\x00" as *const u8 as *const libc::c_char,
                 b"../z_bgcheck.c\x00" as *const u8 as *const libc::c_char,
                 1811 as libc::c_int);
    };
    (*nodeList).max = max;
    (*nodeList).count = 0 as libc::c_int;
}
/* *
 * Reset DynaSSNodeList count
 */
#[no_mangle]
pub unsafe extern "C" fn DynaSSNodeList_ResetCount(mut nodeList:
                                                       *mut DynaSSNodeList) {
    (*nodeList).count = 0 as libc::c_int;
}
/* *
 * Get next available node index in DynaSSNodeList
 * returns SS_NULL if list is full
 */
#[no_mangle]
pub unsafe extern "C" fn DynaSSNodeList_GetNextNodeIdx(mut nodeList:
                                                           *mut DynaSSNodeList)
 -> u16_0 {
    let fresh0 = (*nodeList).count;
    (*nodeList).count = (*nodeList).count + 1;
    let mut idx: u16_0 = fresh0 as u16_0;
    if (*nodeList).max <= idx as libc::c_int {
        return 0xffff as libc::c_int as u16_0
    }
    return idx;
}
/* *
 * original name: T_BGCheck_Vec3sToVec3f
 */
#[no_mangle]
pub unsafe extern "C" fn BgCheck_Vec3sToVec3f(mut src: *mut Vec3s,
                                              mut dst: *mut Vec3f) {
    (*dst).x = (*src).x as f32_0;
    (*dst).y = (*src).y as f32_0;
    (*dst).z = (*src).z as f32_0;
}
/* *
 * original name: T_BGCheck_Vec3fToVec3s
 */
#[no_mangle]
pub unsafe extern "C" fn BgCheck_Vec3fToVec3s(mut dst: *mut Vec3s,
                                              mut src: *mut Vec3f) {
    (*dst).x = (*src).x as s16;
    (*dst).y = (*src).y as s16;
    (*dst).z = (*src).z as s16;
}
/* *
 * Get CollisionPoly's lowest y point
 */
#[no_mangle]
pub unsafe extern "C" fn CollisionPoly_GetMinY(mut poly: *mut CollisionPoly,
                                               mut vtxList: *mut Vec3s)
 -> s16 {
    let mut a: s32 = 0;
    let mut b: s32 = 0;
    let mut c: s32 = 0;
    let mut min: s16 = 0;
    // ! @bug Due to rounding errors, some polys with a slight slope have a y normal of 1.0f/-1.0f. As such, this
    // ! optimization returns the wrong minimum y for a subset of these polys.
    if (*poly).normal.y as libc::c_int ==
           (1.0f32 * 32767.0f32) as s16 as libc::c_int ||
           (*poly).normal.y as libc::c_int ==
               (-1.0f32 * 32767.0f32) as s16 as libc::c_int {
        return (*vtxList.offset(((*poly).c2rust_unnamed.c2rust_unnamed.flags_vIA
                                     as libc::c_int & 0x1fff as libc::c_int)
                                    as isize)).y
    }
    a =
        (*poly).c2rust_unnamed.c2rust_unnamed.flags_vIA as libc::c_int &
            0x1fff as libc::c_int;
    b =
        (*poly).c2rust_unnamed.c2rust_unnamed.flags_vIB as libc::c_int &
            0x1fff as libc::c_int;
    c = (*poly).c2rust_unnamed.c2rust_unnamed.vIC as s32;
    min = (*vtxList.offset(a as isize)).y;
    if min as libc::c_int > (*vtxList.offset(b as isize)).y as libc::c_int {
        min = (*vtxList.offset(b as isize)).y
    }
    if (min as libc::c_int) < (*vtxList.offset(c as isize)).y as libc::c_int {
        return min
    }
    return (*vtxList.offset(c as isize)).y;
}
/* *
 * CollisionPoly get unit normal
 */
#[no_mangle]
pub unsafe extern "C" fn CollisionPoly_GetNormalF(mut poly:
                                                      *mut CollisionPoly,
                                                  mut nx: *mut f32_0,
                                                  mut ny: *mut f32_0,
                                                  mut nz: *mut f32_0) {
    *nx =
        (*poly).normal.x as libc::c_int as libc::c_float *
            (1.0f32 / 32767.0f32);
    *ny =
        (*poly).normal.y as libc::c_int as libc::c_float *
            (1.0f32 / 32767.0f32);
    *nz =
        (*poly).normal.z as libc::c_int as libc::c_float *
            (1.0f32 / 32767.0f32);
}
/* *
 * Compute transform matrix mapping +y (up) to the collision poly's normal
 */
#[no_mangle]
pub unsafe extern "C" fn func_80038A28(mut poly: *mut CollisionPoly,
                                       mut tx: f32_0, mut ty: f32_0,
                                       mut tz: f32_0, mut dest: *mut MtxF) {
    let mut nx: f32_0 = 0.;
    let mut ny: f32_0 = 0.;
    let mut nz: f32_0 = 0.;
    let mut pad: s32 = 0;
    let mut phi_f2: f32_0 = 0.;
    let mut phi_f14: f32_0 = 0.;
    let mut phi_f12: f32_0 = 0.;
    let mut inv_phi_f2: f32_0 = 0.;
    let mut inv_phi_f14: f32_0 = 0.;
    if poly.is_null() { return }
    CollisionPoly_GetNormalF(poly, &mut nx, &mut ny, &mut nz);
    phi_f2 = sqrtf(1.0f32 - nx * nx);
    if !(fabsf(phi_f2) < 0.008f32) {
        inv_phi_f2 = 1.0f32 / phi_f2;
        phi_f14 = ny * inv_phi_f2;
        phi_f12 = -(nz * inv_phi_f2)
    } else {
        phi_f14 = sqrtf(1.0f32 - ny * ny);
        if !(fabsf(phi_f14) < 0.008f32) {
            inv_phi_f14 = 1.0f32 / phi_f14;
            phi_f12 = nx * inv_phi_f14;
            phi_f2 = -(nz * inv_phi_f14)
        } else { phi_f12 = 0.0f32; phi_f2 = 0.0f32 }
    }
    (*dest).c2rust_unnamed.xx = phi_f2;
    (*dest).c2rust_unnamed.yx = -nx * phi_f14;
    (*dest).c2rust_unnamed.zx = nx * phi_f12;
    (*dest).c2rust_unnamed.xy = nx;
    (*dest).c2rust_unnamed.yy = ny;
    (*dest).c2rust_unnamed.zy = nz;
    (*dest).c2rust_unnamed.yz = phi_f12;
    (*dest).c2rust_unnamed.zz = phi_f14;
    (*dest).c2rust_unnamed.wx = 0.0f32;
    (*dest).c2rust_unnamed.wy = 0.0f32;
    (*dest).c2rust_unnamed.xz = 0.0f32;
    (*dest).c2rust_unnamed.wz = 0.0f32;
    (*dest).c2rust_unnamed.xw = tx;
    (*dest).c2rust_unnamed.yw = ty;
    (*dest).c2rust_unnamed.zw = tz;
    (*dest).c2rust_unnamed.ww = 1.0f32;
}
/* *
 * Calculate point distance from plane along normal
 */
#[no_mangle]
pub unsafe extern "C" fn CollisionPoly_GetPointDistanceFromPlane(mut poly:
                                                                     *mut CollisionPoly,
                                                                 mut point:
                                                                     *mut Vec3f)
 -> f32_0 {
    return ((*poly).normal.x as libc::c_int as libc::c_float * (*point).x +
                (*poly).normal.y as libc::c_int as libc::c_float * (*point).y
                +
                (*poly).normal.z as libc::c_int as libc::c_float * (*point).z)
               * (1.0f32 / 32767.0f32) +
               (*poly).dist as libc::c_int as libc::c_float;
}
/* *
 * Get Poly Vertices
 */
#[no_mangle]
pub unsafe extern "C" fn CollisionPoly_GetVertices(mut poly:
                                                       *mut CollisionPoly,
                                                   mut vtxList: *mut Vec3s,
                                                   mut dest: *mut Vec3f) {
    BgCheck_Vec3sToVec3f(&mut *vtxList.offset(((*poly).c2rust_unnamed.c2rust_unnamed.flags_vIA
                                                   as libc::c_int &
                                                   0x1fff as libc::c_int) as
                                                  isize),
                         &mut *dest.offset(0 as libc::c_int as isize));
    BgCheck_Vec3sToVec3f(&mut *vtxList.offset(((*poly).c2rust_unnamed.c2rust_unnamed.flags_vIB
                                                   as libc::c_int &
                                                   0x1fff as libc::c_int) as
                                                  isize),
                         &mut *dest.offset(1 as libc::c_int as isize));
    BgCheck_Vec3sToVec3f(&mut *vtxList.offset((*poly).c2rust_unnamed.c2rust_unnamed.vIC
                                                  as isize),
                         &mut *dest.offset(2 as libc::c_int as isize));
}
/* *
 * Get vertices by bgId
 * original name: T_Polygon_GetVertex_bg_ai
 */
#[no_mangle]
pub unsafe extern "C" fn CollisionPoly_GetVerticesByBgId(mut poly:
                                                             *mut CollisionPoly,
                                                         mut bgId: s32,
                                                         mut colCtx:
                                                             *mut CollisionContext,
                                                         mut dest:
                                                             *mut Vec3f) {
    let mut vtxList: *mut Vec3s = 0 as *mut Vec3s;
    if poly.is_null() || bgId > 50 as libc::c_int || dest.is_null() {
        osSyncPrintf(b"\x1b[41;37m\x00" as *const u8 as *const libc::c_char);
        // "Argument not appropriate. Processing terminated."
        osSyncPrintf(b"T_Polygon_GetVertex_bg_ai(): Error %d %d %d \xe5\xbc\x95\xe6\x95\xb0\xe3\x81\x8c\xe9\x81\xa9\xe5\x88\x87\xe3\x81\xa7\xe3\x81\xaf\xe3\x81\x82\xe3\x82\x8a\xe3\x81\xbe\xe3\x81\x9b\xe3\x82\x93\xe3\x80\x82\xe5\x87\xa6\xe7\x90\x86\xe3\x82\x92\xe7\xb5\x82\xe4\xba\x86\xe3\x81\x97\xe3\x81\xbe\xe3\x81\x99\xe3\x80\x82\n\x00"
                         as *const u8 as *const libc::c_char,
                     (poly == 0 as *mut libc::c_void as *mut CollisionPoly) as
                         libc::c_int,
                     (bgId > 50 as libc::c_int) as libc::c_int,
                     (dest == 0 as *mut libc::c_void as *mut Vec3f) as
                         libc::c_int);
        osSyncPrintf(b"\x1b[m\x00" as *const u8 as *const libc::c_char);
        if !dest.is_null() {
            // ! @bug: dest[2] x and y are not set to 0
            let ref mut fresh1 = (*dest.offset(2 as libc::c_int as isize)).z;
            *fresh1 = 0.0f32;
            let ref mut fresh2 = (*dest.offset(1 as libc::c_int as isize)).z;
            *fresh2 = *fresh1;
            let ref mut fresh3 = (*dest.offset(1 as libc::c_int as isize)).y;
            *fresh3 = *fresh2;
            let ref mut fresh4 = (*dest.offset(1 as libc::c_int as isize)).x;
            *fresh4 = *fresh3;
            let ref mut fresh5 = (*dest.offset(0 as libc::c_int as isize)).z;
            *fresh5 = *fresh4;
            let ref mut fresh6 = (*dest.offset(0 as libc::c_int as isize)).y;
            *fresh6 = *fresh5;
            (*dest.offset(0 as libc::c_int as isize)).x = *fresh6
        }
    } else {
        if bgId == 50 as libc::c_int {
            vtxList = (*(*colCtx).colHeader).vtxList
        } else { vtxList = (*colCtx).dyna.vtxList }
        CollisionPoly_GetVertices(poly, vtxList, dest);
    };
}
/* *
 * Checks if point (`x`,`z`) is within `chkDist` of `poly`, computing `yIntersect` if true
 * Determinant max 300.0f
 */
#[no_mangle]
pub unsafe extern "C" fn CollisionPoly_CheckYIntersectApprox1(mut poly:
                                                                  *mut CollisionPoly,
                                                              mut vtxList:
                                                                  *mut Vec3s,
                                                              mut x: f32_0,
                                                              mut z: f32_0,
                                                              mut yIntersect:
                                                                  *mut f32_0,
                                                              mut chkDist:
                                                                  f32_0)
 -> s32 {
    static mut polyVerts: [Vec3f; 3] = [Vec3f{x: 0., y: 0., z: 0.,}; 3];
    let mut nx: f32_0 = 0.;
    let mut ny: f32_0 = 0.;
    let mut nz: f32_0 = 0.;
    let mut vA: *mut Vec3s = 0 as *mut Vec3s;
    let mut vB: *mut Vec3s = 0 as *mut Vec3s;
    let mut vC: *mut Vec3s = 0 as *mut Vec3s;
    vA =
        &mut *vtxList.offset(((*poly).c2rust_unnamed.c2rust_unnamed.flags_vIA
                                  as libc::c_int & 0x1fff as libc::c_int) as
                                 isize) as *mut Vec3s;
    Math_Vec3s_ToVec3f(&mut *polyVerts.as_mut_ptr().offset(0 as libc::c_int as
                                                               isize), vA);
    vB =
        &mut *vtxList.offset(((*poly).c2rust_unnamed.c2rust_unnamed.flags_vIB
                                  as libc::c_int & 0x1fff as libc::c_int) as
                                 isize) as *mut Vec3s;
    Math_Vec3s_ToVec3f(&mut *polyVerts.as_mut_ptr().offset(1 as libc::c_int as
                                                               isize), vB);
    vC =
        &mut *vtxList.offset((*poly).c2rust_unnamed.c2rust_unnamed.vIC as
                                 isize) as *mut Vec3s;
    Math_Vec3s_ToVec3f(&mut *polyVerts.as_mut_ptr().offset(2 as libc::c_int as
                                                               isize), vC);
    nx =
        (*poly).normal.x as libc::c_int as libc::c_float *
            (1.0f32 / 32767.0f32);
    ny =
        (*poly).normal.y as libc::c_int as libc::c_float *
            (1.0f32 / 32767.0f32);
    nz =
        (*poly).normal.z as libc::c_int as libc::c_float *
            (1.0f32 / 32767.0f32);
    return Math3D_TriChkPointParaYIntersectDist(&mut *polyVerts.as_mut_ptr().offset(0
                                                                                        as
                                                                                        libc::c_int
                                                                                        as
                                                                                        isize),
                                                &mut *polyVerts.as_mut_ptr().offset(1
                                                                                        as
                                                                                        libc::c_int
                                                                                        as
                                                                                        isize),
                                                &mut *polyVerts.as_mut_ptr().offset(2
                                                                                        as
                                                                                        libc::c_int
                                                                                        as
                                                                                        isize),
                                                nx, ny, nz,
                                                (*poly).dist as f32_0, z, x,
                                                yIntersect, chkDist);
}
/* *
 * Checks if point (`x`,`z`) is within `chkDist` of `poly`, computing `yIntersect` if true
 * Determinant max 0.0f (checks if on or within poly)
 */
#[no_mangle]
pub unsafe extern "C" fn CollisionPoly_CheckYIntersect(mut poly:
                                                           *mut CollisionPoly,
                                                       mut vtxList:
                                                           *mut Vec3s,
                                                       mut x: f32_0,
                                                       mut z: f32_0,
                                                       mut yIntersect:
                                                           *mut f32_0,
                                                       mut chkDist: f32_0)
 -> s32 {
    static mut polyVerts: [Vec3f; 3] = [Vec3f{x: 0., y: 0., z: 0.,}; 3];
    let mut nx: f32_0 = 0.;
    let mut ny: f32_0 = 0.;
    let mut nz: f32_0 = 0.;
    CollisionPoly_GetVertices(poly, vtxList, polyVerts.as_mut_ptr());
    CollisionPoly_GetNormalF(poly, &mut nx, &mut ny, &mut nz);
    return Math3D_TriChkPointParaYIntersectInsideTri(&mut *polyVerts.as_mut_ptr().offset(0
                                                                                             as
                                                                                             libc::c_int
                                                                                             as
                                                                                             isize),
                                                     &mut *polyVerts.as_mut_ptr().offset(1
                                                                                             as
                                                                                             libc::c_int
                                                                                             as
                                                                                             isize),
                                                     &mut *polyVerts.as_mut_ptr().offset(2
                                                                                             as
                                                                                             libc::c_int
                                                                                             as
                                                                                             isize),
                                                     nx, ny, nz,
                                                     (*poly).dist as f32_0, z,
                                                     x, yIntersect, chkDist);
}
/* *
 * Checks if point (`x`,`z`) is within 1.0f of `poly`, computing `yIntersect` if true
 * Determinant max 300.0f
 */
#[no_mangle]
pub unsafe extern "C" fn CollisionPoly_CheckYIntersectApprox2(mut poly:
                                                                  *mut CollisionPoly,
                                                              mut vtxList:
                                                                  *mut Vec3s,
                                                              mut x: f32_0,
                                                              mut z: f32_0,
                                                              mut yIntersect:
                                                                  *mut f32_0)
 -> s32 {
    return CollisionPoly_CheckYIntersectApprox1(poly, vtxList, x, z,
                                                yIntersect, 1.0f32);
}
/* *
 * Checks if point (`y`,`z`) is within 1.0f of `poly`, computing `xIntersect` if true
 * Determinant max 300.0f
 */
#[no_mangle]
pub unsafe extern "C" fn CollisionPoly_CheckXIntersectApprox(mut poly:
                                                                 *mut CollisionPoly,
                                                             mut vtxList:
                                                                 *mut Vec3s,
                                                             mut y: f32_0,
                                                             mut z: f32_0,
                                                             mut xIntersect:
                                                                 *mut f32_0)
 -> s32 {
    static mut polyVerts: [Vec3f; 3] = [Vec3f{x: 0., y: 0., z: 0.,}; 3];
    let mut nx: f32_0 = 0.;
    let mut ny: f32_0 = 0.;
    let mut nz: f32_0 = 0.;
    CollisionPoly_GetVertices(poly, vtxList, polyVerts.as_mut_ptr());
    CollisionPoly_GetNormalF(poly, &mut nx, &mut ny, &mut nz);
    return Math3D_TriChkPointParaXIntersect(&mut *polyVerts.as_mut_ptr().offset(0
                                                                                    as
                                                                                    libc::c_int
                                                                                    as
                                                                                    isize),
                                            &mut *polyVerts.as_mut_ptr().offset(1
                                                                                    as
                                                                                    libc::c_int
                                                                                    as
                                                                                    isize),
                                            &mut *polyVerts.as_mut_ptr().offset(2
                                                                                    as
                                                                                    libc::c_int
                                                                                    as
                                                                                    isize),
                                            nx, ny, nz, (*poly).dist as f32_0,
                                            y, z, xIntersect);
}
/* *
 * Checks if point (`x`,`y`) is within 1.0f of `poly`, computing `zIntersect` if true
 * Determinant max 300.0f
 */
#[no_mangle]
pub unsafe extern "C" fn CollisionPoly_CheckZIntersectApprox(mut poly:
                                                                 *mut CollisionPoly,
                                                             mut vtxList:
                                                                 *mut Vec3s,
                                                             mut x: f32_0,
                                                             mut y: f32_0,
                                                             mut zIntersect:
                                                                 *mut f32_0)
 -> s32 {
    static mut polyVerts: [Vec3f; 3] = [Vec3f{x: 0., y: 0., z: 0.,}; 3];
    let mut nx: f32_0 = 0.;
    let mut ny: f32_0 = 0.;
    let mut nz: f32_0 = 0.;
    CollisionPoly_GetVertices(poly, vtxList, polyVerts.as_mut_ptr());
    CollisionPoly_GetNormalF(poly, &mut nx, &mut ny, &mut nz);
    return Math3D_TriChkPointParaZIntersect(&mut *polyVerts.as_mut_ptr().offset(0
                                                                                    as
                                                                                    libc::c_int
                                                                                    as
                                                                                    isize),
                                            &mut *polyVerts.as_mut_ptr().offset(1
                                                                                    as
                                                                                    libc::c_int
                                                                                    as
                                                                                    isize),
                                            &mut *polyVerts.as_mut_ptr().offset(2
                                                                                    as
                                                                                    libc::c_int
                                                                                    as
                                                                                    isize),
                                            nx, ny, nz, (*poly).dist as f32_0,
                                            x, y, zIntersect);
}
/* *
 * Test if travelling from `posA` to `posB` intersects `poly`
 * returns true if an intersection occurs, else false
 * returns `planeIntersect`, which is the point at which the line from `posA` to `posB` crosses `poly`'s plane
 * if `chkOneFace` is true, return false (no intersection) when going through the poly from A to B is done in the
 * normal's direction
 */
#[no_mangle]
pub unsafe extern "C" fn CollisionPoly_LineVsPoly(mut poly:
                                                      *mut CollisionPoly,
                                                  mut vtxList: *mut Vec3s,
                                                  mut posA: *mut Vec3f,
                                                  mut posB: *mut Vec3f,
                                                  mut planeIntersect:
                                                      *mut Vec3f,
                                                  mut chkOneFace: s32,
                                                  mut chkDist: f32_0) -> s32 {
    static mut polyVerts: [Vec3f; 3] = [Vec3f{x: 0., y: 0., z: 0.,}; 3];
    static mut plane: Plane =
        Plane{normal: Vec3f{x: 0., y: 0., z: 0.,}, originDist: 0.,};
    let mut planeDistA: f32_0 = 0.;
    let mut planeDistB: f32_0 = 0.;
    let mut planeDistDelta: f32_0 = 0.;
    plane.originDist = (*poly).dist as f32_0;
    planeDistA =
        ((*poly).normal.x as libc::c_int as libc::c_float * (*posA).x +
             (*poly).normal.y as libc::c_int as libc::c_float * (*posA).y +
             (*poly).normal.z as libc::c_int as libc::c_float * (*posA).z) *
            (1.0f32 / 32767.0f32) + plane.originDist;
    planeDistB =
        ((*poly).normal.x as libc::c_int as libc::c_float * (*posB).x +
             (*poly).normal.y as libc::c_int as libc::c_float * (*posB).y +
             (*poly).normal.z as libc::c_int as libc::c_float * (*posB).z) *
            (1.0f32 / 32767.0f32) + plane.originDist;
    planeDistDelta = planeDistA - planeDistB;
    if planeDistA >= 0.0f32 && planeDistB >= 0.0f32 ||
           planeDistA < 0.0f32 && planeDistB < 0.0f32 ||
           chkOneFace != 0 && planeDistA < 0.0f32 && planeDistB > 0.0f32 ||
           fabsf(planeDistDelta) < 0.008f32 {
        return 0 as libc::c_int
    }
    CollisionPoly_GetNormalF(poly, &mut plane.normal.x, &mut plane.normal.y,
                             &mut plane.normal.z);
    CollisionPoly_GetVertices(poly, vtxList, polyVerts.as_mut_ptr());
    Math3D_LineSplitRatio(posA, posB, planeDistA / planeDistDelta,
                          planeIntersect);
    if fabsf(plane.normal.x) > 0.5f32 &&
           Math3D_TriChkPointParaXDist(&mut *polyVerts.as_mut_ptr().offset(0
                                                                               as
                                                                               libc::c_int
                                                                               as
                                                                               isize),
                                       &mut *polyVerts.as_mut_ptr().offset(1
                                                                               as
                                                                               libc::c_int
                                                                               as
                                                                               isize),
                                       &mut *polyVerts.as_mut_ptr().offset(2
                                                                               as
                                                                               libc::c_int
                                                                               as
                                                                               isize),
                                       &mut plane, (*planeIntersect).y,
                                       (*planeIntersect).z, chkDist) != 0 ||
           fabsf(plane.normal.y) > 0.5f32 &&
               Math3D_TriChkPointParaYDist(&mut *polyVerts.as_mut_ptr().offset(0
                                                                                   as
                                                                                   libc::c_int
                                                                                   as
                                                                                   isize),
                                           &mut *polyVerts.as_mut_ptr().offset(1
                                                                                   as
                                                                                   libc::c_int
                                                                                   as
                                                                                   isize),
                                           &mut *polyVerts.as_mut_ptr().offset(2
                                                                                   as
                                                                                   libc::c_int
                                                                                   as
                                                                                   isize),
                                           &mut plane, (*planeIntersect).z,
                                           (*planeIntersect).x, chkDist) != 0
           ||
           fabsf(plane.normal.z) > 0.5f32 &&
               Math3D_TriChkLineSegParaZDist(&mut *polyVerts.as_mut_ptr().offset(0
                                                                                     as
                                                                                     libc::c_int
                                                                                     as
                                                                                     isize),
                                             &mut *polyVerts.as_mut_ptr().offset(1
                                                                                     as
                                                                                     libc::c_int
                                                                                     as
                                                                                     isize),
                                             &mut *polyVerts.as_mut_ptr().offset(2
                                                                                     as
                                                                                     libc::c_int
                                                                                     as
                                                                                     isize),
                                             &mut plane, (*planeIntersect).x,
                                             (*planeIntersect).y, chkDist) !=
                   0 {
        return 1 as libc::c_int
    }
    return 0 as libc::c_int;
}
/* *
 * Tests if sphere `center` `radius` intersects `poly`
 */
#[no_mangle]
pub unsafe extern "C" fn CollisionPoly_SphVsPoly(mut poly: *mut CollisionPoly,
                                                 mut vtxList: *mut Vec3s,
                                                 mut center: *mut Vec3f,
                                                 mut radius: f32_0) -> s32 {
    static mut sphere: Sphere16 =
        Sphere16{center: Vec3s{x: 0, y: 0, z: 0,}, radius: 0,};
    static mut tri: TriNorm =
        TriNorm{vtx: [Vec3f{x: 0., y: 0., z: 0.,}; 3],
                plane:
                    Plane{normal: Vec3f{x: 0., y: 0., z: 0.,},
                          originDist: 0.,},};
    let mut intersect: Vec3f = Vec3f{x: 0., y: 0., z: 0.,};
    CollisionPoly_GetVertices(poly, vtxList, tri.vtx.as_mut_ptr());
    CollisionPoly_GetNormalF(poly, &mut tri.plane.normal.x,
                             &mut tri.plane.normal.y,
                             &mut tri.plane.normal.z);
    tri.plane.originDist = (*poly).dist as f32_0;
    sphere.center.x = (*center).x as s16;
    sphere.center.y = (*center).y as s16;
    sphere.center.z = (*center).z as s16;
    sphere.radius = radius as s16;
    return Math3D_TriVsSphIntersect(&mut sphere, &mut tri, &mut intersect);
}
/* *
 * Add poly to StaticLookup table
 * Table is sorted by poly's smallest y vertex component
 * `ssList` is the list to append a new poly to
 * `polyList` is the CollisionPoly lookup list
 * `vtxList` is the vertex lookup list
 * `polyId` is the index of the poly in polyList to insert into the lookup table
 */
#[no_mangle]
pub unsafe extern "C" fn StaticLookup_AddPolyToSSList(mut colCtx:
                                                          *mut CollisionContext,
                                                      mut ssList: *mut SSList,
                                                      mut polyList:
                                                          *mut CollisionPoly,
                                                      mut vtxList: *mut Vec3s,
                                                      mut polyId: s16) {
    let mut curNode: *mut SSNode = 0 as *mut SSNode;
    let mut nextNode: *mut SSNode = 0 as *mut SSNode;
    let mut polyYMin: s32 = 0;
    let mut newNodeId: u16_0 = 0;
    let mut curPolyId: s16 = 0;
    // if list is null
    if (*ssList).head as libc::c_int == 0xffff as libc::c_int {
        SSNodeList_SetSSListHead(&mut (*colCtx).polyNodes, ssList,
                                 &mut polyId);
        return
    }
    polyYMin =
        CollisionPoly_GetMinY(&mut *polyList.offset(polyId as isize), vtxList)
            as s32;
    curNode =
        &mut *(*colCtx).polyNodes.tbl.offset((*ssList).head as isize) as
            *mut SSNode;
    curPolyId = (*curNode).polyId;
    // if the poly being inserted has a lower y than the first poly
    if polyYMin <
           (*vtxList.offset(((*polyList.offset(curPolyId as
                                                   isize)).c2rust_unnamed.c2rust_unnamed.flags_vIA
                                 as libc::c_int & 0x1fff as libc::c_int) as
                                isize)).y as libc::c_int &&
           polyYMin <
               (*vtxList.offset(((*polyList.offset(curPolyId as
                                                       isize)).c2rust_unnamed.c2rust_unnamed.flags_vIB
                                     as libc::c_int & 0x1fff as libc::c_int)
                                    as isize)).y as libc::c_int &&
           polyYMin <
               (*vtxList.offset((*polyList.offset(curPolyId as
                                                      isize)).c2rust_unnamed.c2rust_unnamed.vIC
                                    as isize)).y as libc::c_int {
        SSNodeList_SetSSListHead(&mut (*colCtx).polyNodes, ssList,
                                 &mut polyId);
        return
    }
    loop  {
        // if at the end of the list
        if (*curNode).next as libc::c_int == 0xffff as libc::c_int {
            newNodeId = SSNodeList_GetNextNodeIdx(&mut (*colCtx).polyNodes);
            SSNode_SetValue(&mut *(*colCtx).polyNodes.tbl.offset(newNodeId as
                                                                     isize),
                            &mut polyId, 0xffff as libc::c_int as u16_0);
            (*curNode).next = newNodeId;
            return
        }
        nextNode =
            &mut *(*colCtx).polyNodes.tbl.offset((*curNode).next as isize) as
                *mut SSNode;
        curPolyId = (*nextNode).polyId;
        // if the poly being inserted is lower than the next poly
        if polyYMin <
               (*vtxList.offset(((*polyList.offset(curPolyId as
                                                       isize)).c2rust_unnamed.c2rust_unnamed.flags_vIA
                                     as libc::c_int & 0x1fff as libc::c_int)
                                    as isize)).y as libc::c_int &&
               polyYMin <
                   (*vtxList.offset(((*polyList.offset(curPolyId as
                                                           isize)).c2rust_unnamed.c2rust_unnamed.flags_vIB
                                         as libc::c_int &
                                         0x1fff as libc::c_int) as isize)).y
                       as libc::c_int &&
               polyYMin <
                   (*vtxList.offset((*polyList.offset(curPolyId as
                                                          isize)).c2rust_unnamed.c2rust_unnamed.vIC
                                        as isize)).y as libc::c_int {
            newNodeId = SSNodeList_GetNextNodeIdx(&mut (*colCtx).polyNodes);
            SSNode_SetValue(&mut *(*colCtx).polyNodes.tbl.offset(newNodeId as
                                                                     isize),
                            &mut polyId, (*curNode).next);
            (*curNode).next = newNodeId;
            return
        }
        curNode = nextNode
    };
}
/* *
 * Add CollisionPoly to StaticLookup list
 */
#[no_mangle]
pub unsafe extern "C" fn StaticLookup_AddPoly(mut lookup: *mut StaticLookup,
                                              mut colCtx:
                                                  *mut CollisionContext,
                                              mut polyList:
                                                  *mut CollisionPoly,
                                              mut vtxList: *mut Vec3s,
                                              mut index: s16) {
    if (*polyList.offset(index as isize)).normal.y as libc::c_int >
           (0.5f32 * 32767.0f32) as s16 as libc::c_int {
        StaticLookup_AddPolyToSSList(colCtx, &mut (*lookup).floor, polyList,
                                     vtxList, index);
    } else if ((*polyList.offset(index as isize)).normal.y as libc::c_int) <
                  (-0.8f32 * 32767.0f32) as s16 as libc::c_int {
        StaticLookup_AddPolyToSSList(colCtx, &mut (*lookup).ceiling, polyList,
                                     vtxList, index);
    } else {
        StaticLookup_AddPolyToSSList(colCtx, &mut (*lookup).wall, polyList,
                                     vtxList, index);
    };
}
/* *
 * Locates the closest static poly directly underneath `pos`, starting at list `ssList`
 * returns yIntersect of the closest poly, or `yIntersectMin`
 * stores the pointer of the closest poly to `outPoly`
 * if (flags & 1), ignore polys with a normal.y < 0 (from vertical walls to ceilings)
 */
#[no_mangle]
pub unsafe extern "C" fn BgCheck_RaycastFloorStaticList(mut colCtx:
                                                            *mut CollisionContext,
                                                        mut xpFlags: u16_0,
                                                        mut ssList:
                                                            *mut SSList,
                                                        mut outPoly:
                                                            *mut *mut CollisionPoly,
                                                        mut pos: *mut Vec3f,
                                                        mut yIntersectMin:
                                                            f32_0,
                                                        mut chkDist: f32_0,
                                                        mut flags: s32)
 -> f32_0 {
    let mut curNode: *mut SSNode = 0 as *mut SSNode;
    let mut polyId: s32 = 0;
    let mut result: f32_0 = 0.;
    let mut yIntersect: f32_0 = 0.;
    result = yIntersectMin;
    if (*ssList).head as libc::c_int == 0xffff as libc::c_int {
        return result
    }
    curNode =
        &mut *(*colCtx).polyNodes.tbl.offset((*ssList).head as isize) as
            *mut SSNode;
    loop  {
        polyId = (*curNode).polyId as s32;
        if (*(*(*colCtx).colHeader).polyList.offset(polyId as
                                                        isize)).c2rust_unnamed.c2rust_unnamed.flags_vIA
               as libc::c_int &
               (xpFlags as libc::c_int & 7 as libc::c_int) <<
                   13 as libc::c_int != 0 ||
               flags & 1 as libc::c_int != 0 &&
                   ((*(*(*colCtx).colHeader).polyList.offset(polyId as
                                                                 isize)).normal.y
                        as libc::c_int) < 0 as libc::c_int {
            if (*curNode).next as libc::c_int == 0xffff as libc::c_int {
                break ;
            }
            curNode =
                &mut *(*colCtx).polyNodes.tbl.offset((*curNode).next as isize)
                    as *mut SSNode
        } else {
            if (*pos).y <
                   (*(*(*colCtx).colHeader).vtxList.offset(((*(*(*colCtx).colHeader).polyList.offset(polyId
                                                                                                         as
                                                                                                         isize)).c2rust_unnamed.c2rust_unnamed.flags_vIA
                                                                as libc::c_int
                                                                &
                                                                0x1fff as
                                                                    libc::c_int)
                                                               as isize)).y as
                       libc::c_int as libc::c_float &&
                   (*pos).y <
                       (*(*(*colCtx).colHeader).vtxList.offset(((*(*(*colCtx).colHeader).polyList.offset(polyId
                                                                                                             as
                                                                                                             isize)).c2rust_unnamed.c2rust_unnamed.flags_vIB
                                                                    as
                                                                    libc::c_int
                                                                    &
                                                                    0x1fff as
                                                                        libc::c_int)
                                                                   as
                                                                   isize)).y
                           as libc::c_int as libc::c_float &&
                   (*pos).y <
                       (*(*(*colCtx).colHeader).vtxList.offset((*(*(*colCtx).colHeader).polyList.offset(polyId
                                                                                                            as
                                                                                                            isize)).c2rust_unnamed.c2rust_unnamed.vIC
                                                                   as
                                                                   isize)).y
                           as libc::c_int as libc::c_float {
                break ;
            }
            if CollisionPoly_CheckYIntersect(&mut *(*(*colCtx).colHeader).polyList.offset(polyId
                                                                                              as
                                                                                              isize),
                                             (*(*colCtx).colHeader).vtxList,
                                             (*pos).x, (*pos).z,
                                             &mut yIntersect, chkDist) ==
                   1 as libc::c_int {
                // if poly is closer to pos without going over
                if yIntersect < (*pos).y && result < yIntersect {
                    result = yIntersect;
                    *outPoly =
                        &mut *(*(*colCtx).colHeader).polyList.offset(polyId as
                                                                         isize)
                            as *mut CollisionPoly
                }
            }
            if (*curNode).next as libc::c_int == 0xffff as libc::c_int {
                break ;
            }
            curNode =
                &mut *(*colCtx).polyNodes.tbl.offset((*curNode).next as isize)
                    as *mut SSNode
        }
    }
    return result;
}
/* *
 * Locates the closest static poly directly underneath `pos` within `lookup`.
 * returns yIntersect of the closest poly, or `yIntersectMin`
 * stores the pointer of the closest poly to `outPoly`
 */
#[no_mangle]
pub unsafe extern "C" fn BgCheck_RaycastFloorStatic(mut lookup:
                                                        *mut StaticLookup,
                                                    mut colCtx:
                                                        *mut CollisionContext,
                                                    mut xpFlags: u16_0,
                                                    mut poly:
                                                        *mut *mut CollisionPoly,
                                                    mut pos: *mut Vec3f,
                                                    mut arg5: u32_0,
                                                    mut chkDist: f32_0,
                                                    mut yIntersectMin: f32_0)
 -> f32_0 {
    let mut flag: s32 = 0; // skip polys with normal.y < 0
    let mut yIntersect: f32_0 = yIntersectMin;
    if arg5 & 4 as libc::c_int as libc::c_uint != 0 {
        yIntersect =
            BgCheck_RaycastFloorStaticList(colCtx, xpFlags,
                                           &mut (*lookup).floor, poly, pos,
                                           yIntersect, chkDist,
                                           0 as libc::c_int)
    }
    if arg5 & 2 as libc::c_int as libc::c_uint != 0 ||
           arg5 & 8 as libc::c_int as libc::c_uint != 0 {
        flag = 0 as libc::c_int;
        if arg5 & 0x10 as libc::c_int as libc::c_uint != 0 {
            flag = 1 as libc::c_int
        }
        yIntersect =
            BgCheck_RaycastFloorStaticList(colCtx, xpFlags,
                                           &mut (*lookup).wall, poly, pos,
                                           yIntersect, chkDist, flag)
    }
    if arg5 & 1 as libc::c_int as libc::c_uint != 0 {
        flag = 0 as libc::c_int;
        if arg5 & 0x10 as libc::c_int as libc::c_uint != 0 {
            flag = 1 as libc::c_int
        }
        yIntersect =
            BgCheck_RaycastFloorStaticList(colCtx, xpFlags,
                                           &mut (*lookup).ceiling, poly, pos,
                                           yIntersect, chkDist, flag)
    }
    return yIntersect;
}
/* *
 * Compute wall displacement on `posX` and `posZ`
 * sets `wallPolyPtr` to `poly` if `wallPolyPtr` is NULL or not a damage wall
 * returns true if `wallPolyPtr` was changed
 * `invXZlength` is 1 / sqrt( sq(poly.normal.x) + sq(poly.normal.z) )
 */
#[no_mangle]
pub unsafe extern "C" fn BgCheck_ComputeWallDisplacement(mut colCtx:
                                                             *mut CollisionContext,
                                                         mut poly:
                                                             *mut CollisionPoly,
                                                         mut posX: *mut f32_0,
                                                         mut posZ: *mut f32_0,
                                                         mut nx: f32_0,
                                                         mut ny: f32_0,
                                                         mut nz: f32_0,
                                                         mut invXZlength:
                                                             f32_0,
                                                         mut planeDist: f32_0,
                                                         mut radius: f32_0,
                                                         mut wallPolyPtr:
                                                             *mut *mut CollisionPoly)
 -> s32 {
    let mut wallPoly: *mut CollisionPoly = 0 as *mut CollisionPoly;
    let mut surfaceData: u32_0 = 0;
    let mut wallDamage: u32_0 = 0;
    let mut displacement: f32_0 = (radius - planeDist) * invXZlength;
    *posX += displacement * nx;
    *posZ += displacement * nz;
    wallPoly = *wallPolyPtr;
    if wallPoly.is_null() { *wallPolyPtr = poly; return 1 as libc::c_int }
    surfaceData =
        (*(*(*colCtx).colHeader).surfaceTypeList.offset((*wallPoly).type_0 as
                                                            isize)).data[1 as
                                                                             libc::c_int
                                                                             as
                                                                             usize];
    wallDamage =
        if surfaceData & 0x8000000 as libc::c_int as libc::c_uint != 0 {
            1 as libc::c_int
        } else { 0 as libc::c_int } as u32_0;
    if wallDamage == 0 { *wallPolyPtr = poly; return 1 as libc::c_int }
    return 0 as libc::c_int;
}
/* *
 * Performs collision detection on static poly walls within `lookup` on sphere `pos`, `radius`
 * returns true if a collision was detected
 * `outX` `outZ` return the displaced x,z coordinates,
 * `outPoly` returns the pointer to the nearest poly collided with, or NULL
 */
#[no_mangle]
pub unsafe extern "C" fn BgCheck_SphVsStaticWall(mut lookup:
                                                     *mut StaticLookup,
                                                 mut colCtx:
                                                     *mut CollisionContext,
                                                 mut xpFlags: u16_0,
                                                 mut outX: *mut f32_0,
                                                 mut outZ: *mut f32_0,
                                                 mut pos: *mut Vec3f,
                                                 mut radius: f32_0,
                                                 mut outPoly:
                                                     *mut *mut CollisionPoly)
 -> s32 {
    let mut resultPos: Vec3f = Vec3f{x: 0., y: 0., z: 0.,};
    let mut temp_f2: f32_0 = 0.;
    let mut temp_f2_2: f32_0 = 0.;
    let mut planeDist: f32_0 = 0.;
    let mut intersect: f32_0 = 0.;
    let mut result: s32 = 0;
    let mut curPoly: *mut CollisionPoly = 0 as *mut CollisionPoly;
    let mut polyList: *mut CollisionPoly = 0 as *mut CollisionPoly;
    let mut curNode: *mut SSNode = 0 as *mut SSNode;
    let mut invNormalXZ: f32_0 = 0.;
    let mut zTemp: f32_0 = 0.;
    let mut xTemp: f32_0 = 0.;
    let mut polyId: s32 = 0;
    let mut normalXZ: f32_0 = 0.;
    let mut nx: f32_0 = 0.;
    let mut ny: f32_0 = 0.;
    let mut nz: f32_0 = 0.;
    let mut temp_f16: f32_0 = 0.;
    let mut vtxList: *mut Vec3s = 0 as *mut Vec3s;
    let mut pad: u16_0 = 0;
    let mut zMin: f32_0 = 0.;
    let mut zMax: f32_0 = 0.;
    let mut xMin: f32_0 = 0.;
    let mut xMax: f32_0 = 0.;
    result = 0 as libc::c_int;
    if (*lookup).wall.head as libc::c_int == 0xffff as libc::c_int {
        return result
    }
    resultPos = *pos;
    polyList = (*(*colCtx).colHeader).polyList;
    vtxList = (*(*colCtx).colHeader).vtxList;
    curNode =
        &mut *(*colCtx).polyNodes.tbl.offset((*lookup).wall.head as isize) as
            *mut SSNode;
    loop  {
        polyId = (*curNode).polyId as s32;
        curPoly =
            &mut *polyList.offset(polyId as isize) as *mut CollisionPoly;
        if (*pos).y <
               (*vtxList.offset(((*curPoly).c2rust_unnamed.c2rust_unnamed.flags_vIA
                                     as libc::c_int & 0x1fff as libc::c_int)
                                    as isize)).y as libc::c_int as
                   libc::c_float &&
               (*pos).y <
                   (*vtxList.offset(((*curPoly).c2rust_unnamed.c2rust_unnamed.flags_vIB
                                         as libc::c_int &
                                         0x1fff as libc::c_int) as isize)).y
                       as libc::c_int as libc::c_float &&
               (*pos).y <
                   (*vtxList.offset((*curPoly).c2rust_unnamed.c2rust_unnamed.vIC
                                        as isize)).y as libc::c_int as
                       libc::c_float {
            break ;
        }
        nx =
            (*curPoly).normal.x as libc::c_int as libc::c_float *
                (1.0f32 / 32767.0f32);
        ny =
            (*curPoly).normal.y as libc::c_int as libc::c_float *
                (1.0f32 / 32767.0f32);
        nz =
            (*curPoly).normal.z as libc::c_int as libc::c_float *
                (1.0f32 / 32767.0f32);
        normalXZ = sqrtf(nx * nx + nz * nz);
        planeDist =
            Math3D_DistPlaneToPos(nx, ny, nz, (*curPoly).dist as f32_0,
                                  &mut resultPos);
        if radius < fabsf(planeDist) ||
               (*curPoly).c2rust_unnamed.c2rust_unnamed.flags_vIA as
                   libc::c_int &
                   (xpFlags as libc::c_int & 7 as libc::c_int) <<
                       13 as libc::c_int != 0 {
            if (*curNode).next as libc::c_int == 0xffff as libc::c_int {
                break ;
            }
            curNode =
                &mut *(*colCtx).polyNodes.tbl.offset((*curNode).next as isize)
                    as *mut SSNode
        } else {
            if !(fabsf(normalXZ) < 0.008f32) {
            } else {
                __assert(b"!IS_ZERO(ac_size)\x00" as *const u8 as
                             *const libc::c_char,
                         b"../z_bgcheck.c\x00" as *const u8 as
                             *const libc::c_char, 2854 as libc::c_int);
            };
            invNormalXZ = 1.0f32 / normalXZ;
            temp_f16 = fabsf(nz) * invNormalXZ;
            if temp_f16 < 0.4f32 {
                if (*curNode).next as libc::c_int == 0xffff as libc::c_int {
                    break ;
                }
                curNode =
                    &mut *(*colCtx).polyNodes.tbl.offset((*curNode).next as
                                                             isize) as
                        *mut SSNode
            } else {
                // compute curPoly zMin/zMax
                zTemp =
                    (*vtxList.offset(((*curPoly).c2rust_unnamed.c2rust_unnamed.flags_vIA
                                          as libc::c_int &
                                          0x1fff as libc::c_int) as isize)).z
                        as f32_0;
                zMin = zTemp;
                zMax = zMin;
                zTemp =
                    (*vtxList.offset(((*curPoly).c2rust_unnamed.c2rust_unnamed.flags_vIB
                                          as libc::c_int &
                                          0x1fff as libc::c_int) as isize)).z
                        as f32_0;
                if zTemp < zMin {
                    zMin = zTemp
                } else if zMax < zTemp { zMax = zTemp }
                zTemp =
                    (*vtxList.offset((*curPoly).c2rust_unnamed.c2rust_unnamed.vIC
                                         as isize)).z as f32_0;
                if zTemp < zMin {
                    zMin = zTemp
                } else if zTemp > zMax { zMax = zTemp }
                zMin -= radius;
                zMax += radius;
                if resultPos.z < zMin || resultPos.z > zMax {
                    if (*curNode).next as libc::c_int == 0xffff as libc::c_int
                       {
                        break ;
                    }
                    curNode =
                        &mut *(*colCtx).polyNodes.tbl.offset((*curNode).next
                                                                 as isize) as
                            *mut SSNode
                } else {
                    if CollisionPoly_CheckZIntersectApprox(curPoly, vtxList,
                                                           resultPos.x,
                                                           (*pos).y,
                                                           &mut intersect) !=
                           0 {
                        if fabsf(intersect - resultPos.z) <= radius / temp_f16
                           {
                            if (intersect - resultPos.z) * nz <= 4.0f32 {
                                BgCheck_ComputeWallDisplacement(colCtx,
                                                                curPoly,
                                                                &mut resultPos.x,
                                                                &mut resultPos.z,
                                                                nx, ny, nz,
                                                                invNormalXZ,
                                                                planeDist,
                                                                radius,
                                                                outPoly);
                                result = 1 as libc::c_int
                            }
                        }
                    }
                    if (*curNode).next as libc::c_int == 0xffff as libc::c_int
                       {
                        break ;
                    }
                    curNode =
                        &mut *(*colCtx).polyNodes.tbl.offset((*curNode).next
                                                                 as isize) as
                            *mut SSNode
                }
            }
        }
    }
    curNode =
        &mut *(*colCtx).polyNodes.tbl.offset((*lookup).wall.head as isize) as
            *mut SSNode;
    loop  {
        polyId = (*curNode).polyId as s32;
        curPoly =
            &mut *polyList.offset(polyId as isize) as *mut CollisionPoly;
        if (*pos).y <
               (*vtxList.offset(((*curPoly).c2rust_unnamed.c2rust_unnamed.flags_vIA
                                     as libc::c_int & 0x1fff as libc::c_int)
                                    as isize)).y as libc::c_int as
                   libc::c_float &&
               (*pos).y <
                   (*vtxList.offset(((*curPoly).c2rust_unnamed.c2rust_unnamed.flags_vIB
                                         as libc::c_int &
                                         0x1fff as libc::c_int) as isize)).y
                       as libc::c_int as libc::c_float &&
               (*pos).y <
                   (*vtxList.offset((*curPoly).c2rust_unnamed.c2rust_unnamed.vIC
                                        as isize)).y as libc::c_int as
                       libc::c_float {
            break ;
        }
        nx =
            (*curPoly).normal.x as libc::c_int as libc::c_float *
                (1.0f32 / 32767.0f32);
        ny =
            (*curPoly).normal.y as libc::c_int as libc::c_float *
                (1.0f32 / 32767.0f32);
        nz =
            (*curPoly).normal.z as libc::c_int as libc::c_float *
                (1.0f32 / 32767.0f32);
        normalXZ = sqrtf(nx * nx + nz * nz);
        planeDist =
            Math3D_DistPlaneToPos(nx, ny, nz, (*curPoly).dist as f32_0,
                                  &mut resultPos);
        if radius < fabsf(planeDist) ||
               (*curPoly).c2rust_unnamed.c2rust_unnamed.flags_vIA as
                   libc::c_int &
                   (xpFlags as libc::c_int & 7 as libc::c_int) <<
                       13 as libc::c_int != 0 {
            if (*curNode).next as libc::c_int == 0xffff as libc::c_int {
                break ;
            }
            curNode =
                &mut *(*colCtx).polyNodes.tbl.offset((*curNode).next as isize)
                    as *mut SSNode
        } else {
            if !(fabsf(normalXZ) < 0.008f32) {
            } else {
                __assert(b"!IS_ZERO(ac_size)\x00" as *const u8 as
                             *const libc::c_char,
                         b"../z_bgcheck.c\x00" as *const u8 as
                             *const libc::c_char, 2964 as libc::c_int);
            };
            invNormalXZ = 1.0f32 / normalXZ;
            temp_f16 = fabsf(nx) * invNormalXZ;
            if temp_f16 < 0.4f32 {
                if (*curNode).next as libc::c_int == 0xffff as libc::c_int {
                    break ;
                }
                curNode =
                    &mut *(*colCtx).polyNodes.tbl.offset((*curNode).next as
                                                             isize) as
                        *mut SSNode
            } else {
                // compute curPoly xMin/xMax
                xTemp =
                    (*vtxList.offset(((*curPoly).c2rust_unnamed.c2rust_unnamed.flags_vIA
                                          as libc::c_int &
                                          0x1fff as libc::c_int) as isize)).x
                        as f32_0;
                xMin = xTemp;
                xMax = xMin;
                xTemp =
                    (*vtxList.offset(((*curPoly).c2rust_unnamed.c2rust_unnamed.flags_vIB
                                          as libc::c_int &
                                          0x1fff as libc::c_int) as isize)).x
                        as f32_0;
                if xTemp < xMin {
                    xMin = xTemp
                } else if xMax < xTemp { xMax = xTemp }
                xTemp =
                    (*vtxList.offset((*curPoly).c2rust_unnamed.c2rust_unnamed.vIC
                                         as isize)).x as f32_0;
                if xTemp < xMin {
                    xMin = xTemp
                } else if xMax < xTemp { xMax = xTemp }
                xMin -= radius;
                xMax += radius;
                if resultPos.x < xMin || xMax < resultPos.x {
                    if (*curNode).next as libc::c_int == 0xffff as libc::c_int
                       {
                        break ;
                    }
                    curNode =
                        &mut *(*colCtx).polyNodes.tbl.offset((*curNode).next
                                                                 as isize) as
                            *mut SSNode
                } else {
                    if CollisionPoly_CheckXIntersectApprox(curPoly, vtxList,
                                                           (*pos).y,
                                                           resultPos.z,
                                                           &mut intersect) !=
                           0 {
                        if fabsf(intersect - resultPos.x) <= radius / temp_f16
                           {
                            if (intersect - resultPos.x) * nx <= 4.0f32 {
                                BgCheck_ComputeWallDisplacement(colCtx,
                                                                curPoly,
                                                                &mut resultPos.x,
                                                                &mut resultPos.z,
                                                                nx, ny, nz,
                                                                invNormalXZ,
                                                                planeDist,
                                                                radius,
                                                                outPoly);
                                result = 1 as libc::c_int
                            }
                        }
                    }
                    if (*curNode).next as libc::c_int == 0xffff as libc::c_int
                       {
                        break ;
                    }
                    curNode =
                        &mut *(*colCtx).polyNodes.tbl.offset((*curNode).next
                                                                 as isize) as
                            *mut SSNode
                }
            }
        }
    }
    *outX = resultPos.x;
    *outZ = resultPos.z;
    return result;
}
/* *
 * Tests for collision with a static poly ceiling
 * returns true if a collision occurs, else false
 * `outPoly` returns the poly collided with
 * `outY` returns the y coordinate needed to not collide with `outPoly`
 */
#[no_mangle]
pub unsafe extern "C" fn BgCheck_CheckStaticCeiling(mut lookup:
                                                        *mut StaticLookup,
                                                    mut xpFlags: u16_0,
                                                    mut colCtx:
                                                        *mut CollisionContext,
                                                    mut outY: *mut f32_0,
                                                    mut pos: *mut Vec3f,
                                                    mut checkHeight: f32_0,
                                                    mut outPoly:
                                                        *mut *mut CollisionPoly)
 -> s32 {
    let mut result: s32 = 0 as libc::c_int;
    let mut nextId: u16_0 = 0;
    let mut curPoly: *mut CollisionPoly = 0 as *mut CollisionPoly;
    let mut polyList: *mut CollisionPoly = 0 as *mut CollisionPoly;
    let mut ceilingY: f32_0 = 0.;
    let mut vtxList: *mut Vec3s = 0 as *mut Vec3s;
    let mut curNode: *mut SSNode = 0 as *mut SSNode;
    let mut curPolyId: s32 = 0;
    if (*lookup).ceiling.head as libc::c_int == 0xffff as libc::c_int {
        return 0 as libc::c_int
    }
    curNode =
        &mut *(*colCtx).polyNodes.tbl.offset((*lookup).ceiling.head as isize)
            as *mut SSNode;
    polyList = (*(*colCtx).colHeader).polyList;
    vtxList = (*(*colCtx).colHeader).vtxList;
    *outY = (*pos).y;
    loop  {
        curPolyId = (*curNode).polyId as s32;
        if (*(*(*colCtx).colHeader).polyList.offset(curPolyId as
                                                        isize)).c2rust_unnamed.c2rust_unnamed.flags_vIA
               as libc::c_int &
               (xpFlags as libc::c_int & 7 as libc::c_int) <<
                   13 as libc::c_int != 0 {
            if (*curNode).next as libc::c_int == 0xffff as libc::c_int {
                break ;
            }
            curNode =
                &mut *(*colCtx).polyNodes.tbl.offset((*curNode).next as isize)
                    as *mut SSNode
        } else {
            curPoly =
                &mut *polyList.offset(curPolyId as isize) as
                    *mut CollisionPoly;
            if CollisionPoly_CheckYIntersectApprox2(curPoly, vtxList,
                                                    (*pos).x, (*pos).z,
                                                    &mut ceilingY) != 0 {
                let mut intersectDist: f32_0 = ceilingY - *outY;
                let mut ny: f32_0 =
                    (*curPoly).normal.y as libc::c_int as libc::c_float *
                        (1.0f32 / 32767.0f32);
                if intersectDist > 0.0f32 && intersectDist < checkHeight &&
                       intersectDist * ny <= 0 as libc::c_int as libc::c_float
                   {
                    *outY = ceilingY - checkHeight;
                    *outPoly = curPoly;
                    result = 1 as libc::c_int
                }
            }
            if (*curNode).next as libc::c_int == 0xffff as libc::c_int {
                break ;
            }
            curNode =
                &mut *(*colCtx).polyNodes.tbl.offset((*curNode).next as isize)
                    as *mut SSNode
        }
    }
    return result;
}
/* *
 * Tests if line `posA` to `posB` intersects with a static poly in list `ssList`. Uses polyCheckTbl
 * returns true if such a poly exists, else false
 * `outPoly` returns the pointer of the poly intersected
 * `posB` and `outPos` returns the point of intersection with `outPoly`
 * `outDistSq` returns the squared distance from `posA` to the point of intersect
 */
#[no_mangle]
pub unsafe extern "C" fn BgCheck_CheckLineAgainstSSList(mut ssList:
                                                            *mut SSList,
                                                        mut colCtx:
                                                            *mut CollisionContext,
                                                        mut xpFlags1: u16_0,
                                                        mut xpFlags2: u16_0,
                                                        mut posA: *mut Vec3f,
                                                        mut posB: *mut Vec3f,
                                                        mut outPos:
                                                            *mut Vec3f,
                                                        mut outPoly:
                                                            *mut *mut CollisionPoly,
                                                        mut outDistSq:
                                                            *mut f32_0,
                                                        mut chkDist: f32_0,
                                                        mut bccFlags: s32)
 -> s32 {
    let mut curNode: *mut SSNode = 0 as *mut SSNode;
    let mut checkedPoly: *mut u8_0 = 0 as *mut u8_0;
    let mut polyIntersect: Vec3f = Vec3f{x: 0., y: 0., z: 0.,};
    let mut polyList: *mut CollisionPoly = 0 as *mut CollisionPoly;
    let mut curPoly: *mut CollisionPoly = 0 as *mut CollisionPoly;
    let mut result: s32 = 0;
    let mut minY: f32_0 = 0.;
    let mut distSq: f32_0 = 0.;
    let mut polyId: s16 = 0;
    result = 0 as libc::c_int;
    polyList = (*(*colCtx).colHeader).polyList;
    if (*ssList).head as libc::c_int == 0xffff as libc::c_int {
        return result
    }
    curNode =
        &mut *(*colCtx).polyNodes.tbl.offset((*ssList).head as isize) as
            *mut SSNode;
    loop  {
        polyId = (*curNode).polyId;
        checkedPoly =
            &mut *(*colCtx).polyNodes.polyCheckTbl.offset(polyId as isize) as
                *mut u8_0;
        if *checkedPoly as libc::c_int == 1 as libc::c_int ||
               (*polyList.offset(polyId as
                                     isize)).c2rust_unnamed.c2rust_unnamed.flags_vIA
                   as libc::c_int &
                   (xpFlags1 as libc::c_int & 7 as libc::c_int) <<
                       13 as libc::c_int != 0 ||
               !(xpFlags2 as libc::c_int == 0 as libc::c_int ||
                     (*polyList.offset(polyId as
                                           isize)).c2rust_unnamed.c2rust_unnamed.flags_vIA
                         as libc::c_int &
                         (xpFlags2 as libc::c_int & 7 as libc::c_int) <<
                             13 as libc::c_int != 0) {
            if (*curNode).next as libc::c_int == 0xffff as libc::c_int {
                break ;
            }
            curNode =
                &mut *(*colCtx).polyNodes.tbl.offset((*curNode).next as isize)
                    as *mut SSNode
        } else {
            *checkedPoly = 1 as libc::c_int as u8_0;
            curPoly =
                &mut *polyList.offset(polyId as isize) as *mut CollisionPoly;
            minY =
                CollisionPoly_GetMinY(curPoly, (*(*colCtx).colHeader).vtxList)
                    as f32_0;
            if (*posA).y < minY && (*posB).y < minY { break ; }
            if CollisionPoly_LineVsPoly(curPoly,
                                        (*(*colCtx).colHeader).vtxList, posA,
                                        posB, &mut polyIntersect,
                                        (bccFlags &
                                             (1 as libc::c_int) <<
                                                 3 as libc::c_int !=
                                             0 as libc::c_int) as libc::c_int,
                                        chkDist) != 0 {
                distSq = Math3D_Vec3fDistSq(posA, &mut polyIntersect);
                if distSq < *outDistSq {
                    *outDistSq = distSq;
                    *outPos = polyIntersect;
                    *posB = polyIntersect;
                    *outPoly = curPoly;
                    result = 1 as libc::c_int
                }
            }
            if (*curNode).next as libc::c_int == 0xffff as libc::c_int {
                break ;
            }
            curNode =
                &mut *(*colCtx).polyNodes.tbl.offset((*curNode).next as isize)
                    as *mut SSNode
        }
    }
    return result;
}
/* *
 * Tests if line `posA` to `posB` intersects with a static poly in `lookup`. Uses polyCheckTbl
 * returns true if such a poly exists, else false
 * `outPoly` returns the pointer of the poly intersected
 * `posB` and `outPos` returns the point of intersection with `outPoly`
 * `outDistSq` returns the squared distance from `posA` to the point of intersect
 */
#[no_mangle]
pub unsafe extern "C" fn BgCheck_CheckLineInSubdivision(mut lookup:
                                                            *mut StaticLookup,
                                                        mut colCtx:
                                                            *mut CollisionContext,
                                                        mut xpFlags1: u16_0,
                                                        mut xpFlags2: u16_0,
                                                        mut posA: *mut Vec3f,
                                                        mut posB: *mut Vec3f,
                                                        mut outPos:
                                                            *mut Vec3f,
                                                        mut outPoly:
                                                            *mut *mut CollisionPoly,
                                                        mut chkDist: f32_0,
                                                        mut outDistSq:
                                                            *mut f32_0,
                                                        mut bccFlags: u32_0)
 -> s32 {
    let mut result: s32 = 0 as libc::c_int;
    if bccFlags & ((1 as libc::c_int) << 1 as libc::c_int) as libc::c_uint !=
           0 && (*lookup).floor.head as libc::c_int != 0xffff as libc::c_int {
        if BgCheck_CheckLineAgainstSSList(&mut (*lookup).floor, colCtx,
                                          xpFlags1, xpFlags2, posA, posB,
                                          outPos, outPoly, outDistSq, chkDist,
                                          bccFlags as s32) != 0 {
            result = 1 as libc::c_int
        }
    }
    if bccFlags & ((1 as libc::c_int) << 0 as libc::c_int) as libc::c_uint !=
           0 && (*lookup).wall.head as libc::c_int != 0xffff as libc::c_int {
        if BgCheck_CheckLineAgainstSSList(&mut (*lookup).wall, colCtx,
                                          xpFlags1, xpFlags2, posA, posB,
                                          outPos, outPoly, outDistSq, chkDist,
                                          bccFlags as s32) != 0 {
            result = 1 as libc::c_int
        }
    }
    if bccFlags & ((1 as libc::c_int) << 2 as libc::c_int) as libc::c_uint !=
           0 && (*lookup).ceiling.head as libc::c_int != 0xffff as libc::c_int
       {
        if BgCheck_CheckLineAgainstSSList(&mut (*lookup).ceiling, colCtx,
                                          xpFlags1, xpFlags2, posA, posB,
                                          outPos, outPoly, outDistSq, chkDist,
                                          bccFlags as s32) != 0 {
            result = 1 as libc::c_int
        }
    }
    return result;
}
/* *
 * Get first static poly intersecting sphere `center` `radius` from list `node`
 * returns true if any poly intersects the sphere, else returns false
 * `outPoly` returns the pointer of the first poly found that intersects
 */
#[no_mangle]
pub unsafe extern "C" fn BgCheck_SphVsFirstStaticPolyList(mut node:
                                                              *mut SSNode,
                                                          mut xpFlags: u16_0,
                                                          mut colCtx:
                                                              *mut CollisionContext,
                                                          mut center:
                                                              *mut Vec3f,
                                                          mut radius: f32_0,
                                                          mut outPoly:
                                                              *mut *mut CollisionPoly)
 -> s32 {
    let mut polyList: *mut CollisionPoly = (*(*colCtx).colHeader).polyList;
    let mut vtxList: *mut Vec3s = (*(*colCtx).colHeader).vtxList;
    let mut curPoly: *mut CollisionPoly = 0 as *mut CollisionPoly;
    let mut nextId: u16_0 = 0;
    let mut curPolyId: s16 = 0;
    loop  {
        curPolyId = (*node).polyId;
        curPoly =
            &mut *polyList.offset(curPolyId as isize) as *mut CollisionPoly;
        if (*(*(*colCtx).colHeader).polyList.offset(curPolyId as
                                                        isize)).c2rust_unnamed.c2rust_unnamed.flags_vIA
               as libc::c_int &
               (xpFlags as libc::c_int & 7 as libc::c_int) <<
                   13 as libc::c_int != 0 {
            if (*node).next as libc::c_int == 0xffff as libc::c_int {
                break ;
            }
            node =
                &mut *(*colCtx).polyNodes.tbl.offset((*node).next as isize) as
                    *mut SSNode
        } else {
            if (*center).y + radius <
                   (*vtxList.offset(((*curPoly).c2rust_unnamed.c2rust_unnamed.flags_vIA
                                         as libc::c_int &
                                         0x1fff as libc::c_int) as isize)).y
                       as libc::c_int as libc::c_float &&
                   (*center).y + radius <
                       (*vtxList.offset(((*curPoly).c2rust_unnamed.c2rust_unnamed.flags_vIB
                                             as libc::c_int &
                                             0x1fff as libc::c_int) as
                                            isize)).y as libc::c_int as
                           libc::c_float &&
                   (*center).y + radius <
                       (*vtxList.offset((*curPoly).c2rust_unnamed.c2rust_unnamed.vIC
                                            as isize)).y as libc::c_int as
                           libc::c_float {
                break ;
            }
            if CollisionPoly_SphVsPoly(curPoly, vtxList, center, radius) != 0
               {
                *outPoly = curPoly;
                return 1 as libc::c_int
            }
            if (*node).next as libc::c_int == 0xffff as libc::c_int {
                break ;
            }
            node =
                &mut *(*colCtx).polyNodes.tbl.offset((*node).next as isize) as
                    *mut SSNode
        }
    }
    return 0 as libc::c_int;
}
/* *
 * Get first static poly intersecting sphere `center` `radius` within `lookup`
 * returns true if any poly intersects the sphere, else false
 * `outPoly` returns the first poly found that intersects
 */
#[no_mangle]
pub unsafe extern "C" fn BgCheck_SphVsFirstStaticPoly(mut lookup:
                                                          *mut StaticLookup,
                                                      mut xpFlags: u16_0,
                                                      mut colCtx:
                                                          *mut CollisionContext,
                                                      mut center: *mut Vec3f,
                                                      mut radius: f32_0,
                                                      mut outPoly:
                                                          *mut *mut CollisionPoly,
                                                      mut bciFlags: u16_0)
 -> s32 {
    if (*lookup).floor.head as libc::c_int != 0xffff as libc::c_int &&
           bciFlags as libc::c_int & (1 as libc::c_int) << 2 as libc::c_int ==
               0 &&
           BgCheck_SphVsFirstStaticPolyList(&mut *(*colCtx).polyNodes.tbl.offset((*lookup).floor.head
                                                                                     as
                                                                                     isize),
                                            xpFlags, colCtx, center, radius,
                                            outPoly) != 0 {
        return 1 as libc::c_int
    }
    if (*lookup).wall.head as libc::c_int != 0xffff as libc::c_int &&
           bciFlags as libc::c_int & (1 as libc::c_int) << 1 as libc::c_int ==
               0 &&
           BgCheck_SphVsFirstStaticPolyList(&mut *(*colCtx).polyNodes.tbl.offset((*lookup).wall.head
                                                                                     as
                                                                                     isize),
                                            xpFlags, colCtx, center, radius,
                                            outPoly) != 0 {
        return 1 as libc::c_int
    }
    if (*lookup).ceiling.head as libc::c_int != 0xffff as libc::c_int &&
           bciFlags as libc::c_int & (1 as libc::c_int) << 0 as libc::c_int ==
               0 &&
           BgCheck_SphVsFirstStaticPolyList(&mut *(*colCtx).polyNodes.tbl.offset((*lookup).ceiling.head
                                                                                     as
                                                                                     isize),
                                            xpFlags, colCtx, center, radius,
                                            outPoly) != 0 {
        return 1 as libc::c_int
    }
    return 0 as libc::c_int;
}
/* *
 * Get StaticLookup from `pos`
 * Does not return NULL
 */
#[no_mangle]
pub unsafe extern "C" fn BgCheck_GetNearestStaticLookup(mut colCtx:
                                                            *mut CollisionContext,
                                                        mut lookupTbl:
                                                            *mut StaticLookup,
                                                        mut pos: *mut Vec3f)
 -> *mut StaticLookup {
    let mut sector: Vec3i = Vec3i{x: 0, y: 0, z: 0,};
    let mut subdivAmountX: s32 = 0;
    BgCheck_GetStaticLookupIndicesFromPos(colCtx, pos, &mut sector);
    subdivAmountX = (*colCtx).subdivAmount.x;
    return lookupTbl.offset((sector.z * subdivAmountX *
                                 (*colCtx).subdivAmount.y) as
                                isize).offset(sector.x as
                                                  isize).offset((sector.y *
                                                                     subdivAmountX)
                                                                    as isize);
}
/* *
 * Get StaticLookup from `pos`
 * Returns NULL if just outside the mesh bounding box
 */
#[no_mangle]
pub unsafe extern "C" fn BgCheck_GetStaticLookup(mut colCtx:
                                                     *mut CollisionContext,
                                                 mut lookupTbl:
                                                     *mut StaticLookup,
                                                 mut pos: *mut Vec3f)
 -> *mut StaticLookup {
    let mut sector: Vec3i = Vec3i{x: 0, y: 0, z: 0,};
    let mut subdivAmountX: s32 = 0;
    if BgCheck_PosInStaticBoundingBox(colCtx, pos) == 0 {
        return 0 as *mut StaticLookup
    }
    BgCheck_GetStaticLookupIndicesFromPos(colCtx, pos, &mut sector);
    subdivAmountX = (*colCtx).subdivAmount.x;
    return lookupTbl.offset((sector.z * subdivAmountX *
                                 (*colCtx).subdivAmount.y) as
                                isize).offset(sector.x as
                                                  isize).offset((sector.y *
                                                                     subdivAmountX)
                                                                    as isize);
}
/* *
 * Get StaticLookup subdivision indices from `pos`
 * `sector` returns the subdivision x,y,z indices containing or is nearest to `pos`
 */
#[no_mangle]
pub unsafe extern "C" fn BgCheck_GetStaticLookupIndicesFromPos(mut colCtx:
                                                                   *mut CollisionContext,
                                                               mut pos:
                                                                   *mut Vec3f,
                                                               mut sector:
                                                                   *mut Vec3i) {
    (*sector).x =
        (((*pos).x - (*colCtx).minBounds.x) * (*colCtx).subdivLengthInv.x) as
            s32;
    (*sector).y =
        (((*pos).y - (*colCtx).minBounds.y) * (*colCtx).subdivLengthInv.y) as
            s32;
    (*sector).z =
        (((*pos).z - (*colCtx).minBounds.z) * (*colCtx).subdivLengthInv.z) as
            s32;
    if (*sector).x < 0 as libc::c_int {
        (*sector).x = 0 as libc::c_int
    } else if (*sector).x >= (*colCtx).subdivAmount.x {
        (*sector).x = (*colCtx).subdivAmount.x - 1 as libc::c_int
    }
    if (*sector).y < 0 as libc::c_int {
        (*sector).y = 0 as libc::c_int
    } else if (*sector).y >= (*colCtx).subdivAmount.y {
        (*sector).y = (*colCtx).subdivAmount.y - 1 as libc::c_int
    }
    if (*sector).z < 0 as libc::c_int {
        (*sector).z = 0 as libc::c_int
    } else if (*sector).z >= (*colCtx).subdivAmount.z {
        (*sector).z = (*colCtx).subdivAmount.z - 1 as libc::c_int
    };
}
/* *
 * Get negative bias subdivision indices
 * decrements indices if `pos` is within BGCHECK_SUBDIV_OVERLAP units of the negative subdivision boundary
 * `sx`, `sy`, `sz` returns the subdivision x, y, z indices
 */
#[no_mangle]
pub unsafe extern "C" fn BgCheck_GetSubdivisionMinBounds(mut colCtx:
                                                             *mut CollisionContext,
                                                         mut pos: *mut Vec3f,
                                                         mut sx: *mut s32,
                                                         mut sy: *mut s32,
                                                         mut sz: *mut s32) {
    let mut dx: f32_0 = (*pos).x - (*colCtx).minBounds.x;
    let mut dy: f32_0 = (*pos).y - (*colCtx).minBounds.y;
    let mut dz: f32_0 = (*pos).z - (*colCtx).minBounds.z;
    *sx = (dx * (*colCtx).subdivLengthInv.x) as s32;
    *sy = (dy * (*colCtx).subdivLengthInv.y) as s32;
    *sz = (dz * (*colCtx).subdivLengthInv.z) as s32;
    if (dx as s32 % (*colCtx).subdivLength.x as s32) < 50 as libc::c_int &&
           *sx > 0 as libc::c_int {
        *sx -= 1 as libc::c_int
    }
    if (dy as s32 % (*colCtx).subdivLength.y as s32) < 50 as libc::c_int &&
           *sy > 0 as libc::c_int {
        *sy -= 1 as libc::c_int
    }
    if (dz as s32 % (*colCtx).subdivLength.z as s32) < 50 as libc::c_int &&
           *sz > 0 as libc::c_int {
        *sz -= 1 as libc::c_int
    };
}
/* *
 * Get positive bias subdivision indices
 * increments indicies if `pos` is within BGCHECK_SUBDIV_OVERLAP units of the postive subdivision boundary
 * `sx`, `sy`, `sz` returns the subdivision x, y, z indices
 */
#[no_mangle]
pub unsafe extern "C" fn BgCheck_GetSubdivisionMaxBounds(mut colCtx:
                                                             *mut CollisionContext,
                                                         mut pos: *mut Vec3f,
                                                         mut sx: *mut s32,
                                                         mut sy: *mut s32,
                                                         mut sz: *mut s32) {
    let mut dx: f32_0 = (*pos).x - (*colCtx).minBounds.x;
    let mut dy: f32_0 = (*pos).y - (*colCtx).minBounds.y;
    let mut dz: f32_0 = (*pos).z - (*colCtx).minBounds.z;
    *sx = (dx * (*colCtx).subdivLengthInv.x) as s32;
    *sy = (dy * (*colCtx).subdivLengthInv.y) as s32;
    *sz = (dz * (*colCtx).subdivLengthInv.z) as s32;
    if ((*colCtx).subdivLength.x as s32 - 50 as libc::c_int) <
           dx as s32 % (*colCtx).subdivLength.x as s32 &&
           *sx < (*colCtx).subdivAmount.x - 1 as libc::c_int {
        *sx += 1 as libc::c_int
    }
    if ((*colCtx).subdivLength.y as s32 - 50 as libc::c_int) <
           dy as s32 % (*colCtx).subdivLength.y as s32 &&
           *sy < (*colCtx).subdivAmount.y - 1 as libc::c_int {
        *sy += 1 as libc::c_int
    }
    if ((*colCtx).subdivLength.z as s32 - 50 as libc::c_int) <
           dz as s32 % (*colCtx).subdivLength.z as s32 &&
           *sz < (*colCtx).subdivAmount.z - 1 as libc::c_int {
        *sz += 1 as libc::c_int
    };
}
/* *
 * Calculate the subdivision index bounding box for CollisionPoly `polyId`
 * `subdivMinX`, `subdivMinY`, `subdivMinZ` returns the minimum subdivision x, y, z indices
 * `subdivMaxX`, `subdivMaxY`, `subdivMaxZ` returns the maximum subdivision x, y, z indices
 */
#[no_mangle]
pub unsafe extern "C" fn BgCheck_GetPolySubdivisionBounds(mut colCtx:
                                                              *mut CollisionContext,
                                                          mut vtxList:
                                                              *mut Vec3s,
                                                          mut polyList:
                                                              *mut CollisionPoly,
                                                          mut subdivMinX:
                                                              *mut s32,
                                                          mut subdivMinY:
                                                              *mut s32,
                                                          mut subdivMinZ:
                                                              *mut s32,
                                                          mut subdivMaxX:
                                                              *mut s32,
                                                          mut subdivMaxY:
                                                              *mut s32,
                                                          mut subdivMaxZ:
                                                              *mut s32,
                                                          mut polyId: s16) {
    let mut vtxDataTemp: *mut u16_0 = 0 as *mut u16_0;
    let mut minVtx: Vec3f = Vec3f{x: 0., y: 0., z: 0.,};
    let mut maxVtx: Vec3f = Vec3f{x: 0., y: 0., z: 0.,};
    let mut x: f32_0 = 0.;
    let mut y: f32_0 = 0.;
    let mut z: f32_0 = 0.;
    let mut vtx: *mut Vec3s = 0 as *mut Vec3s;
    let mut vtxId: s16 =
        ((*polyList.offset(polyId as
                               isize)).c2rust_unnamed.vtxData[0 as libc::c_int
                                                                  as usize] as
             libc::c_int & 0x1fff as libc::c_int) as s16;
    Math_Vec3s_ToVec3f(&mut maxVtx, &mut *vtxList.offset(vtxId as isize));
    Math_Vec3f_Copy(&mut minVtx, &mut maxVtx);
    vtxDataTemp =
        (*polyList.offset(polyId as
                              isize)).c2rust_unnamed.vtxData.as_mut_ptr().offset(1
                                                                                     as
                                                                                     libc::c_int
                                                                                     as
                                                                                     isize);
    while vtxDataTemp <
              (*polyList.offset(polyId as
                                    isize)).c2rust_unnamed.vtxData.as_mut_ptr().offset(3
                                                                                           as
                                                                                           libc::c_int
                                                                                           as
                                                                                           isize)
          {
        vtxId = (*vtxDataTemp as libc::c_int & 0x1fff as libc::c_int) as s16;
        vtx = &mut *vtxList.offset(vtxId as isize) as *mut Vec3s;
        x = (*vtx).x as f32_0;
        y = (*vtx).y as f32_0;
        z = (*vtx).z as f32_0;
        if minVtx.x > x { minVtx.x = x } else if maxVtx.x < x { maxVtx.x = x }
        if minVtx.y > y { minVtx.y = y } else if maxVtx.y < y { maxVtx.y = y }
        if minVtx.z > z { minVtx.z = z } else if maxVtx.z < z { maxVtx.z = z }
        vtxDataTemp = vtxDataTemp.offset(1)
    }
    BgCheck_GetSubdivisionMinBounds(colCtx, &mut minVtx, subdivMinX,
                                    subdivMinY, subdivMinZ);
    BgCheck_GetSubdivisionMaxBounds(colCtx, &mut maxVtx, subdivMaxX,
                                    subdivMaxY, subdivMaxZ);
}
/* *
 * Test if poly `polyList`[`polyId`] intersects cube `min` `max`
 * returns true if the poly intersects the cube, else false
 */
#[no_mangle]
pub unsafe extern "C" fn BgCheck_PolyIntersectsSubdivision(mut min:
                                                               *mut Vec3f,
                                                           mut max:
                                                               *mut Vec3f,
                                                           mut polyList:
                                                               *mut CollisionPoly,
                                                           mut vtxList:
                                                               *mut Vec3s,
                                                           mut polyId: s16)
 -> s32 {
    let mut intersect: f32_0 = 0.;
    let mut va2: Vec3f = Vec3f{x: 0., y: 0., z: 0.,};
    let mut vb2: Vec3f = Vec3f{x: 0., y: 0., z: 0.,};
    let mut vc2: Vec3f = Vec3f{x: 0., y: 0., z: 0.,};
    let mut poly: *mut CollisionPoly = 0 as *mut CollisionPoly;
    let mut nx: f32_0 = 0.;
    let mut ny: f32_0 = 0.;
    let mut nz: f32_0 = 0.;
    let mut dist: f32_0 = 0.;
    let mut va: Vec3f = Vec3f{x: 0., y: 0., z: 0.,};
    let mut vb: Vec3f = Vec3f{x: 0., y: 0., z: 0.,};
    let mut vc: Vec3f = Vec3f{x: 0., y: 0., z: 0.,};
    let mut flags: [s32; 3] = [0; 3];
    flags[1 as libc::c_int as usize] = 0 as libc::c_int;
    flags[0 as libc::c_int as usize] = flags[1 as libc::c_int as usize];
    poly = &mut *polyList.offset(polyId as isize) as *mut CollisionPoly;
    BgCheck_Vec3sToVec3f(&mut *vtxList.offset(((*poly).c2rust_unnamed.c2rust_unnamed.flags_vIA
                                                   as libc::c_int &
                                                   0x1fff as libc::c_int) as
                                                  isize), &mut va);
    flags[0 as libc::c_int as usize] =
        Math3D_PointRelativeToCubeFaces(&mut va, min, max);
    if flags[0 as libc::c_int as usize] == 0 as libc::c_int {
        return 1 as libc::c_int
    }
    BgCheck_Vec3sToVec3f(&mut *vtxList.offset(((*poly).c2rust_unnamed.c2rust_unnamed.flags_vIB
                                                   as libc::c_int &
                                                   0x1fff as libc::c_int) as
                                                  isize), &mut vb);
    flags[1 as libc::c_int as usize] =
        Math3D_PointRelativeToCubeFaces(&mut vb, min, max);
    if flags[1 as libc::c_int as usize] == 0 as libc::c_int {
        return 1 as libc::c_int
    }
    BgCheck_Vec3sToVec3f(&mut *vtxList.offset((*poly).c2rust_unnamed.c2rust_unnamed.vIC
                                                  as isize), &mut vc);
    flags[2 as libc::c_int as usize] =
        Math3D_PointRelativeToCubeFaces(&mut vc, min, max);
    if flags[2 as libc::c_int as usize] == 0 as libc::c_int {
        return 1 as libc::c_int
    }
    if flags[0 as libc::c_int as usize] & flags[1 as libc::c_int as usize] &
           flags[2 as libc::c_int as usize] != 0 {
        return 0 as libc::c_int
    }
    flags[0 as libc::c_int as usize] |=
        Math3D_PointRelativeToCubeEdges(&mut va, min, max) <<
            8 as libc::c_int;
    flags[1 as libc::c_int as usize] |=
        Math3D_PointRelativeToCubeEdges(&mut vb, min, max) <<
            8 as libc::c_int;
    flags[2 as libc::c_int as usize] |=
        Math3D_PointRelativeToCubeEdges(&mut vc, min, max) <<
            8 as libc::c_int;
    if flags[0 as libc::c_int as usize] & flags[1 as libc::c_int as usize] &
           flags[2 as libc::c_int as usize] != 0 {
        return 0 as libc::c_int
    }
    flags[0 as libc::c_int as usize] |=
        Math3D_PointRelativeToCubeVertices(&mut va, min, max) <<
            0x18 as libc::c_int;
    flags[1 as libc::c_int as usize] |=
        Math3D_PointRelativeToCubeVertices(&mut vb, min, max) <<
            0x18 as libc::c_int;
    flags[2 as libc::c_int as usize] |=
        Math3D_PointRelativeToCubeVertices(&mut vc, min, max) <<
            0x18 as libc::c_int;
    if flags[0 as libc::c_int as usize] & flags[1 as libc::c_int as usize] &
           flags[2 as libc::c_int as usize] != 0 {
        return 0 as libc::c_int
    }
    CollisionPoly_GetNormalF(poly, &mut nx, &mut ny, &mut nz);
    dist = (*poly).dist as f32_0;
    if Math3D_TriChkLineSegParaYIntersect(&mut va, &mut vb, &mut vc, nx, ny,
                                          nz, dist, (*min).z, (*min).x,
                                          &mut intersect, (*min).y, (*max).y)
           != 0 ||
           Math3D_TriChkLineSegParaYIntersect(&mut va, &mut vb, &mut vc, nx,
                                              ny, nz, dist, (*max).z,
                                              (*min).x, &mut intersect,
                                              (*min).y, (*max).y) != 0 ||
           Math3D_TriChkLineSegParaYIntersect(&mut va, &mut vb, &mut vc, nx,
                                              ny, nz, dist, (*min).z,
                                              (*max).x, &mut intersect,
                                              (*min).y, (*max).y) != 0 ||
           Math3D_TriChkLineSegParaYIntersect(&mut va, &mut vb, &mut vc, nx,
                                              ny, nz, dist, (*max).z,
                                              (*max).x, &mut intersect,
                                              (*min).y, (*max).y) != 0 {
        return 1 as libc::c_int
    }
    if Math3D_TriChkLineSegParaZIntersect(&mut va, &mut vb, &mut vc, nx, ny,
                                          nz, dist, (*min).x, (*min).y,
                                          &mut intersect, (*min).z, (*max).z)
           != 0 ||
           Math3D_TriChkLineSegParaZIntersect(&mut va, &mut vb, &mut vc, nx,
                                              ny, nz, dist, (*min).x,
                                              (*max).y, &mut intersect,
                                              (*min).z, (*max).z) != 0 ||
           Math3D_TriChkLineSegParaZIntersect(&mut va, &mut vb, &mut vc, nx,
                                              ny, nz, dist, (*max).x,
                                              (*min).y, &mut intersect,
                                              (*min).z, (*max).z) != 0 ||
           Math3D_TriChkLineSegParaZIntersect(&mut va, &mut vb, &mut vc, nx,
                                              ny, nz, dist, (*max).x,
                                              (*max).y, &mut intersect,
                                              (*min).z, (*max).z) != 0 {
        return 1 as libc::c_int
    }
    if Math3D_TriChkLineSegParaXIntersect(&mut va, &mut vb, &mut vc, nx, ny,
                                          nz, dist, (*min).y, (*min).z,
                                          &mut intersect, (*min).x, (*max).x)
           != 0 ||
           Math3D_TriChkLineSegParaXIntersect(&mut va, &mut vb, &mut vc, nx,
                                              ny, nz, dist, (*min).y,
                                              (*max).z, &mut intersect,
                                              (*min).x, (*max).x) != 0 ||
           Math3D_TriChkLineSegParaXIntersect(&mut va, &mut vb, &mut vc, nx,
                                              ny, nz, dist, (*max).y,
                                              (*min).z, &mut intersect,
                                              (*min).x, (*max).x) != 0 ||
           Math3D_TriChkLineSegParaXIntersect(&mut va, &mut vb, &mut vc, nx,
                                              ny, nz, dist, (*max).y,
                                              (*max).z, &mut intersect,
                                              (*min).x, (*max).x) != 0 {
        return 1 as libc::c_int
    }
    BgCheck_Vec3sToVec3f(&mut *vtxList.offset(((*poly).c2rust_unnamed.c2rust_unnamed.flags_vIA
                                                   as libc::c_int &
                                                   0x1fff as libc::c_int) as
                                                  isize), &mut va2);
    BgCheck_Vec3sToVec3f(&mut *vtxList.offset(((*poly).c2rust_unnamed.c2rust_unnamed.flags_vIB
                                                   as libc::c_int &
                                                   0x1fff as libc::c_int) as
                                                  isize), &mut vb2);
    BgCheck_Vec3sToVec3f(&mut *vtxList.offset((*poly).c2rust_unnamed.c2rust_unnamed.vIC
                                                  as isize), &mut vc2);
    if Math3D_LineVsCube(min, max, &mut va2, &mut vb2) != 0 ||
           Math3D_LineVsCube(min, max, &mut vb2, &mut vc2) != 0 ||
           Math3D_LineVsCube(min, max, &mut vc2, &mut va2) != 0 {
        return 1 as libc::c_int
    }
    return 0 as libc::c_int;
}
/* *
 * Initialize StaticLookup Table
 * returns size of table, in bytes
 */
#[no_mangle]
pub unsafe extern "C" fn BgCheck_InitializeStaticLookup(mut colCtx:
                                                            *mut CollisionContext,
                                                        mut globalCtx:
                                                            *mut GlobalContext,
                                                        mut lookupTbl:
                                                            *mut StaticLookup)
 -> u32_0 {
    let mut vtxList: *mut Vec3s = 0 as *mut Vec3s;
    let mut polyList: *mut CollisionPoly = 0 as *mut CollisionPoly;
    let mut polyMax: s32 = 0;
    let mut polyIdx: s32 = 0;
    let mut sx: s32 = 0;
    let mut sy: s32 = 0;
    let mut sz: s32 = 0;
    // subdivMin indices
    let mut sxMin: s32 = 0;
    let mut syMin: s32 = 0;
    let mut szMin: s32 = 0;
    // subdivMax indices
    let mut sxMax: s32 = 0;
    let mut syMax: s32 = 0;
    let mut szMax: s32 = 0;
    // subdiv min/max bounds for adding a poly
    let mut curSubdivMin: Vec3f = Vec3f{x: 0., y: 0., z: 0.,};
    let mut curSubdivMax: Vec3f = Vec3f{x: 0., y: 0., z: 0.,};
    let mut colHeader: *mut CollisionHeader = (*colCtx).colHeader;
    let mut spA4: *mut StaticLookup = 0 as *mut StaticLookup;
    let mut phi_fp: *mut StaticLookup = 0 as *mut StaticLookup;
    let mut phi_s0: *mut StaticLookup = 0 as *mut StaticLookup;
    let mut sp98: s32 = 0;
    let mut subdivLengthX: f32_0 = 0.;
    let mut subdivLengthY: f32_0 = 0.;
    let mut subdivLengthZ: f32_0 = 0.;
    spA4 = lookupTbl;
    while spA4 <
              lookupTbl.offset(((*colCtx).subdivAmount.x *
                                    (*colCtx).subdivAmount.y *
                                    (*colCtx).subdivAmount.z) as isize) {
        (*spA4).floor.head = 0xffff as libc::c_int as u16_0;
        (*spA4).wall.head = 0xffff as libc::c_int as u16_0;
        (*spA4).ceiling.head = 0xffff as libc::c_int as u16_0;
        spA4 = spA4.offset(1)
    }
    polyMax = (*colHeader).numPolygons as s32;
    vtxList = (*colHeader).vtxList;
    polyList = (*colHeader).polyList;
    sp98 = (*colCtx).subdivAmount.x * (*colCtx).subdivAmount.y;
    subdivLengthX =
        (*colCtx).subdivLength.x +
            (2 as libc::c_int * 50 as libc::c_int) as libc::c_float;
    subdivLengthY =
        (*colCtx).subdivLength.y +
            (2 as libc::c_int * 50 as libc::c_int) as libc::c_float;
    subdivLengthZ =
        (*colCtx).subdivLength.z +
            (2 as libc::c_int * 50 as libc::c_int) as libc::c_float;
    polyIdx = 0 as libc::c_int;
    while polyIdx < polyMax {
        BgCheck_GetPolySubdivisionBounds(colCtx, vtxList, polyList,
                                         &mut sxMin, &mut syMin, &mut szMin,
                                         &mut sxMax, &mut syMax, &mut szMax,
                                         polyIdx as s16);
        spA4 = lookupTbl.offset((szMin * sp98) as isize);
        curSubdivMin.z =
            (*colCtx).subdivLength.z * szMin as libc::c_float +
                (*colCtx).minBounds.z - 50 as libc::c_int as libc::c_float;
        curSubdivMax.z = curSubdivMin.z + subdivLengthZ;
        sz = szMin;
        while sz < szMax + 1 as libc::c_int {
            phi_fp = spA4.offset(((*colCtx).subdivAmount.x * syMin) as isize);
            curSubdivMin.y =
                (*colCtx).subdivLength.y * syMin as libc::c_float +
                    (*colCtx).minBounds.y -
                    50 as libc::c_int as libc::c_float;
            curSubdivMax.y = curSubdivMin.y + subdivLengthY;
            sy = syMin;
            while sy < syMax + 1 as libc::c_int {
                phi_s0 = phi_fp.offset(sxMin as isize);
                curSubdivMin.x =
                    (*colCtx).subdivLength.x * sxMin as libc::c_float +
                        (*colCtx).minBounds.x -
                        50 as libc::c_int as libc::c_float;
                curSubdivMax.x = curSubdivMin.x + subdivLengthX;
                sx = sxMin;
                while sx < sxMax + 1 as libc::c_int {
                    if BgCheck_PolyIntersectsSubdivision(&mut curSubdivMin,
                                                         &mut curSubdivMax,
                                                         polyList, vtxList,
                                                         polyIdx as s16) != 0
                       {
                        StaticLookup_AddPoly(phi_s0, colCtx, polyList,
                                             vtxList, polyIdx as s16);
                    }
                    curSubdivMin.x += (*colCtx).subdivLength.x;
                    curSubdivMax.x += (*colCtx).subdivLength.x;
                    phi_s0 = phi_s0.offset(1);
                    sx += 1
                }
                curSubdivMin.y += (*colCtx).subdivLength.y;
                curSubdivMax.y += (*colCtx).subdivLength.y;
                phi_fp = phi_fp.offset((*colCtx).subdivAmount.x as isize);
                sy += 1
            }
            curSubdivMin.z += (*colCtx).subdivLength.z;
            curSubdivMax.z += (*colCtx).subdivLength.z;
            spA4 = spA4.offset(sp98 as isize);
            sz += 1
        }
        polyIdx += 1
    }
    return ((*colCtx).polyNodes.count as
                libc::c_uint).wrapping_mul(::std::mem::size_of::<SSNode>() as
                                               libc::c_ulong);
}
/* *
 * Is current scene a SPOT scene
 */
#[no_mangle]
pub unsafe extern "C" fn BgCheck_IsSpotScene(mut globalCtx:
                                                 *mut GlobalContext) -> s32 {
    static mut spotScenes: [s16; 19] =
        [SCENE_SPOT00 as libc::c_int as s16,
         SCENE_SPOT01 as libc::c_int as s16,
         SCENE_SPOT02 as libc::c_int as s16,
         SCENE_SPOT03 as libc::c_int as s16,
         SCENE_SPOT04 as libc::c_int as s16,
         SCENE_SPOT05 as libc::c_int as s16,
         SCENE_SPOT06 as libc::c_int as s16,
         SCENE_SPOT07 as libc::c_int as s16,
         SCENE_SPOT08 as libc::c_int as s16,
         SCENE_SPOT09 as libc::c_int as s16,
         SCENE_SPOT10 as libc::c_int as s16,
         SCENE_SPOT11 as libc::c_int as s16,
         SCENE_SPOT12 as libc::c_int as s16,
         SCENE_SPOT13 as libc::c_int as s16,
         SCENE_SPOT15 as libc::c_int as s16,
         SCENE_SPOT16 as libc::c_int as s16,
         SCENE_SPOT17 as libc::c_int as s16,
         SCENE_SPOT18 as libc::c_int as s16,
         SCENE_SPOT20 as libc::c_int as s16];
    let mut i: *mut s16 = 0 as *mut s16;
    i = spotScenes.as_mut_ptr();
    while i <
              spotScenes.as_mut_ptr().offset((::std::mem::size_of::<[s16; 19]>()
                                                  as
                                                  libc::c_ulong).wrapping_div(::std::mem::size_of::<s16>()
                                                                                  as
                                                                                  libc::c_ulong)
                                                 as s32 as isize) {
        if (*globalCtx).sceneNum as libc::c_int == *i as libc::c_int {
            return 1 as libc::c_int
        }
        i = i.offset(1)
    }
    return 0 as libc::c_int;
}
/* *
 * Get custom scene memSize
 */
#[no_mangle]
pub unsafe extern "C" fn BgCheck_TryGetCustomMemsize(mut sceneId: s32,
                                                     mut memSize: *mut u32_0)
 -> s32 {
    static mut sceneMemList: [BgCheckSceneMemEntry; 8] =
        [{
             let mut init =
                 BgCheckSceneMemEntry{sceneId:
                                          SCENE_SPOT00 as libc::c_int as s16,
                                      memSize:
                                          0xb798 as libc::c_int as u32_0,};
             init
         },
         {
             let mut init =
                 BgCheckSceneMemEntry{sceneId:
                                          SCENE_GANON_FINAL as libc::c_int as
                                              s16,
                                      memSize:
                                          0x78c8 as libc::c_int as u32_0,};
             init
         },
         {
             let mut init =
                 BgCheckSceneMemEntry{sceneId:
                                          SCENE_GANON_DEMO as libc::c_int as
                                              s16,
                                      memSize:
                                          0x70c8 as libc::c_int as u32_0,};
             init
         },
         {
             let mut init =
                 BgCheckSceneMemEntry{sceneId:
                                          SCENE_JYASINBOSS as libc::c_int as
                                              s16,
                                      memSize:
                                          0xacc8 as libc::c_int as u32_0,};
             init
         },
         {
             let mut init =
                 BgCheckSceneMemEntry{sceneId:
                                          SCENE_KENJYANOMA as libc::c_int as
                                              s16,
                                      memSize:
                                          0x70c8 as libc::c_int as u32_0,};
             init
         },
         {
             let mut init =
                 BgCheckSceneMemEntry{sceneId:
                                          SCENE_JYASINZOU as libc::c_int as
                                              s16,
                                      memSize:
                                          0x16cc8 as libc::c_int as u32_0,};
             init
         },
         {
             let mut init =
                 BgCheckSceneMemEntry{sceneId:
                                          SCENE_HIDAN as libc::c_int as s16,
                                      memSize:
                                          0x198c8 as libc::c_int as u32_0,};
             init
         },
         {
             let mut init =
                 BgCheckSceneMemEntry{sceneId:
                                          SCENE_GANON_BOSS as libc::c_int as
                                              s16,
                                      memSize:
                                          0x84c8 as libc::c_int as u32_0,};
             init
         }];
    let mut i: s32 = 0;
    i = 0 as libc::c_int;
    while i <
              (::std::mem::size_of::<[BgCheckSceneMemEntry; 8]>() as
                   libc::c_ulong).wrapping_div(::std::mem::size_of::<BgCheckSceneMemEntry>()
                                                   as libc::c_ulong) as s32 {
        if sceneId == sceneMemList[i as usize].sceneId as libc::c_int {
            *memSize = sceneMemList[i as usize].memSize;
            return 1 as libc::c_int
        }
        i += 1
    }
    return 0 as libc::c_int;
}
/* *
 * Compute subdivLength for scene mesh lookup, for a single dimension
 */
#[no_mangle]
pub unsafe extern "C" fn BgCheck_SetSubdivisionDimension(mut min: f32_0,
                                                         mut subdivAmount:
                                                             s32,
                                                         mut max: *mut f32_0,
                                                         mut subdivLength:
                                                             *mut f32_0,
                                                         mut subdivLengthInv:
                                                             *mut f32_0) {
    let mut length: f32_0 = *max - min;
    *subdivLength =
        ((length / subdivAmount as libc::c_float) as s32 + 1 as libc::c_int)
            as f32_0;
    *subdivLength =
        if *subdivLength < 150.0f32 { 150.0f32 } else { *subdivLength };
    *subdivLengthInv = 1.0f32 / *subdivLength;
    *max = *subdivLength * subdivAmount as libc::c_float + min;
}
/* *
 * Allocate CollisionContext
 */
#[no_mangle]
pub unsafe extern "C" fn BgCheck_Allocate(mut colCtx: *mut CollisionContext,
                                          mut globalCtx: *mut GlobalContext,
                                          mut colHeader:
                                              *mut CollisionHeader) {
    static mut sceneSubdivisionList: [BgCheckSceneSubdivisionEntry; 2] =
        [{
             let mut init =
                 BgCheckSceneSubdivisionEntry{sceneId:
                                                  SCENE_HAKADAN as libc::c_int
                                                      as s16,
                                              subdivAmount:
                                                  {
                                                      let mut init =
                                                          Vec3s{x:
                                                                    23 as
                                                                        libc::c_int
                                                                        as
                                                                        s16,
                                                                y:
                                                                    7 as
                                                                        libc::c_int
                                                                        as
                                                                        s16,
                                                                z:
                                                                    14 as
                                                                        libc::c_int
                                                                        as
                                                                        s16,};
                                                      init
                                                  },
                                              nodeListMax:
                                                  -(1 as libc::c_int),};
             init
         },
         {
             let mut init =
                 BgCheckSceneSubdivisionEntry{sceneId:
                                                  SCENE_BMORI1 as libc::c_int
                                                      as s16,
                                              subdivAmount:
                                                  {
                                                      let mut init =
                                                          Vec3s{x:
                                                                    38 as
                                                                        libc::c_int
                                                                        as
                                                                        s16,
                                                                y:
                                                                    1 as
                                                                        libc::c_int
                                                                        as
                                                                        s16,
                                                                z:
                                                                    38 as
                                                                        libc::c_int
                                                                        as
                                                                        s16,};
                                                      init
                                                  },
                                              nodeListMax:
                                                  -(1 as libc::c_int),};
             init
         }];
    let mut tblMax: u32_0 = 0;
    let mut memSize: u32_0 = 0;
    let mut lookupTblMemSize: u32_0 = 0;
    let mut nodeList: *mut SSNodeList = 0 as *mut SSNodeList;
    let mut useCustomSubdivisions: s32 = 0;
    let mut customMemSize: u32_0 = 0;
    let mut customNodeListMax: s32 = 0;
    let mut i: s32 = 0;
    (*colCtx).colHeader = colHeader;
    customNodeListMax = -(1 as libc::c_int);
    // "/*---------------- BGCheck Buffer Memory Size -------------*/\n"
    osSyncPrintf(b"/*---------------- BGCheck \xe3\x83\x90\xe3\x83\x83\xe3\x83\x95\xe3\x82\xa1\xe3\x83\xbc\xe3\x83\xa1\xe3\x83\xa2\xe3\x83\xaa\xe3\x82\xb5\xe3\x82\xa4\xe3\x82\xba -------------*/\n\x00"
                     as *const u8 as *const libc::c_char);
    if (*gGameInfo).data[(6 as libc::c_int * 6 as libc::c_int *
                              16 as libc::c_int + 15 as libc::c_int) as usize]
           as libc::c_int == 0x10 as libc::c_int ||
           (*gGameInfo).data[(6 as libc::c_int * 6 as libc::c_int *
                                  16 as libc::c_int + 15 as libc::c_int) as
                                 usize] as libc::c_int == 0x20 as libc::c_int
           ||
           (*gGameInfo).data[(6 as libc::c_int * 6 as libc::c_int *
                                  16 as libc::c_int + 15 as libc::c_int) as
                                 usize] as libc::c_int == 0x30 as libc::c_int
           ||
           (*gGameInfo).data[(6 as libc::c_int * 6 as libc::c_int *
                                  16 as libc::c_int + 15 as libc::c_int) as
                                 usize] as libc::c_int == 0x40 as libc::c_int
       {
        if (*globalCtx).sceneNum as libc::c_int ==
               SCENE_MALON_STABLE as libc::c_int {
            // "/* BGCheck LonLon Size %dbyte */\n"
            osSyncPrintf(b"/* BGCheck LonLon\xe3\x82\xb5\xe3\x82\xa4\xe3\x82\xba %dbyte */\n\x00"
                             as *const u8 as *const libc::c_char,
                         0x3520 as libc::c_int);
            (*colCtx).memSize = 0x3520 as libc::c_int as u32_0
        } else {
            // "/* BGCheck Mini Size %dbyte */\n"
            osSyncPrintf(b"/* BGCheck \xe3\x83\x9f\xe3\x83\x8b\xe3\x82\xb5\xe3\x82\xa4\xe3\x82\xba %dbyte */\n\x00"
                             as *const u8 as *const libc::c_char,
                         0x4e20 as libc::c_int);
            (*colCtx).memSize = 0x4e20 as libc::c_int as u32_0
        }
        (*colCtx).dyna.polyNodesMax = 500 as libc::c_int;
        (*colCtx).dyna.polyListMax = 256 as libc::c_int;
        (*colCtx).dyna.vtxListMax = 256 as libc::c_int;
        (*colCtx).subdivAmount.x = 2 as libc::c_int;
        (*colCtx).subdivAmount.y = 2 as libc::c_int;
        (*colCtx).subdivAmount.z = 2 as libc::c_int
    } else if BgCheck_IsSpotScene(globalCtx) == 1 as libc::c_int {
        (*colCtx).memSize = 0xf000 as libc::c_int as u32_0;
        // "/* BGCheck Spot Size %dbyte */\n"
        osSyncPrintf(b"/* BGCheck Spot\xe7\x94\xa8\xe3\x82\xb5\xe3\x82\xa4\xe3\x82\xba %dbyte */\n\x00"
                         as *const u8 as *const libc::c_char,
                     0xf000 as libc::c_int);
        (*colCtx).dyna.polyNodesMax = 1000 as libc::c_int;
        (*colCtx).dyna.polyListMax = 512 as libc::c_int;
        (*colCtx).dyna.vtxListMax = 512 as libc::c_int;
        (*colCtx).subdivAmount.x = 16 as libc::c_int;
        (*colCtx).subdivAmount.y = 4 as libc::c_int;
        (*colCtx).subdivAmount.z = 16 as libc::c_int
    } else {
        if BgCheck_TryGetCustomMemsize((*globalCtx).sceneNum as s32,
                                       &mut customMemSize) != 0 {
            (*colCtx).memSize = customMemSize
        } else { (*colCtx).memSize = 0x1cc00 as libc::c_int as u32_0 }
        // "/* BGCheck Normal Size %dbyte  */\n"
        osSyncPrintf(b"/* BGCheck \xe3\x83\x8e\xe3\x83\xbc\xe3\x83\x9e\xe3\x83\xab\xe3\x82\xb5\xe3\x82\xa4\xe3\x82\xba %dbyte  */\n\x00"
                         as *const u8 as *const libc::c_char,
                     (*colCtx).memSize);
        (*colCtx).dyna.polyNodesMax = 1000 as libc::c_int;
        (*colCtx).dyna.polyListMax = 512 as libc::c_int;
        (*colCtx).dyna.vtxListMax = 512 as libc::c_int;
        useCustomSubdivisions = 0 as libc::c_int;
        i = 0 as libc::c_int;
        while i <
                  (::std::mem::size_of::<[BgCheckSceneSubdivisionEntry; 2]>()
                       as
                       libc::c_ulong).wrapping_div(::std::mem::size_of::<BgCheckSceneSubdivisionEntry>()
                                                       as libc::c_ulong) as
                      s32 {
            if (*globalCtx).sceneNum as libc::c_int ==
                   sceneSubdivisionList[i as usize].sceneId as libc::c_int {
                (*colCtx).subdivAmount.x =
                    sceneSubdivisionList[i as usize].subdivAmount.x as s32;
                (*colCtx).subdivAmount.y =
                    sceneSubdivisionList[i as usize].subdivAmount.y as s32;
                (*colCtx).subdivAmount.z =
                    sceneSubdivisionList[i as usize].subdivAmount.z as s32;
                useCustomSubdivisions = 1 as libc::c_int;
                customNodeListMax =
                    sceneSubdivisionList[i as usize].nodeListMax
            }
            i += 1
        }
        if useCustomSubdivisions == 0 as libc::c_int {
            (*colCtx).subdivAmount.x = 16 as libc::c_int;
            (*colCtx).subdivAmount.y = 4 as libc::c_int;
            (*colCtx).subdivAmount.z = 16 as libc::c_int
        }
    }
    (*colCtx).lookupTbl =
        THA_AllocEndAlign(&mut (*globalCtx).state.tha,
                          ((*colCtx).subdivAmount.x as
                               libc::c_uint).wrapping_mul(::std::mem::size_of::<StaticLookup>()
                                                              as
                                                              libc::c_ulong).wrapping_mul((*colCtx).subdivAmount.y
                                                                                              as
                                                                                              libc::c_uint).wrapping_mul((*colCtx).subdivAmount.z
                                                                                                                             as
                                                                                                                             libc::c_uint),
                          !(1 as libc::c_int) as u32_0) as *mut StaticLookup;
    if (*colCtx).lookupTbl.is_null() {
        LogUtils_HungupThread(b"../z_bgcheck.c\x00" as *const u8 as
                                  *const libc::c_char, 4176 as libc::c_int);
    }
    (*colCtx).minBounds.x = (*(*colCtx).colHeader).minBounds.x as f32_0;
    (*colCtx).minBounds.y = (*(*colCtx).colHeader).minBounds.y as f32_0;
    (*colCtx).minBounds.z = (*(*colCtx).colHeader).minBounds.z as f32_0;
    (*colCtx).maxBounds.x = (*(*colCtx).colHeader).maxBounds.x as f32_0;
    (*colCtx).maxBounds.y = (*(*colCtx).colHeader).maxBounds.y as f32_0;
    (*colCtx).maxBounds.z = (*(*colCtx).colHeader).maxBounds.z as f32_0;
    BgCheck_SetSubdivisionDimension((*colCtx).minBounds.x,
                                    (*colCtx).subdivAmount.x,
                                    &mut (*colCtx).maxBounds.x,
                                    &mut (*colCtx).subdivLength.x,
                                    &mut (*colCtx).subdivLengthInv.x);
    BgCheck_SetSubdivisionDimension((*colCtx).minBounds.y,
                                    (*colCtx).subdivAmount.y,
                                    &mut (*colCtx).maxBounds.y,
                                    &mut (*colCtx).subdivLength.y,
                                    &mut (*colCtx).subdivLengthInv.y);
    BgCheck_SetSubdivisionDimension((*colCtx).minBounds.z,
                                    (*colCtx).subdivAmount.z,
                                    &mut (*colCtx).maxBounds.z,
                                    &mut (*colCtx).subdivLength.z,
                                    &mut (*colCtx).subdivLengthInv.z);
    memSize =
        ((*colCtx).subdivAmount.x as
             libc::c_uint).wrapping_mul(::std::mem::size_of::<StaticLookup>()
                                            as
                                            libc::c_ulong).wrapping_mul((*colCtx).subdivAmount.y
                                                                            as
                                                                            libc::c_uint).wrapping_mul((*colCtx).subdivAmount.z
                                                                                                           as
                                                                                                           libc::c_uint).wrapping_add(((*(*colCtx).colHeader).numPolygons
                                                                                                                                           as
                                                                                                                                           libc::c_uint).wrapping_mul(::std::mem::size_of::<u8_0>()
                                                                                                                                                                          as
                                                                                                                                                                          libc::c_ulong)).wrapping_add(((*colCtx).dyna.polyNodesMax
                                                                                                                                                                                                            as
                                                                                                                                                                                                            libc::c_uint).wrapping_mul(::std::mem::size_of::<SSNode>()
                                                                                                                                                                                                                                           as
                                                                                                                                                                                                                                           libc::c_ulong)).wrapping_add(((*colCtx).dyna.polyListMax
                                                                                                                                                                                                                                                                             as
                                                                                                                                                                                                                                                                             libc::c_uint).wrapping_mul(::std::mem::size_of::<CollisionPoly>()
                                                                                                                                                                                                                                                                                                            as
                                                                                                                                                                                                                                                                                                            libc::c_ulong)).wrapping_add(((*colCtx).dyna.vtxListMax
                                                                                                                                                                                                                                                                                                                                              as
                                                                                                                                                                                                                                                                                                                                              libc::c_uint).wrapping_mul(::std::mem::size_of::<Vec3s>()
                                                                                                                                                                                                                                                                                                                                                                             as
                                                                                                                                                                                                                                                                                                                                                                             libc::c_ulong)).wrapping_add(::std::mem::size_of::<CollisionContext>()
                                                                                                                                                                                                                                                                                                                                                                                                              as
                                                                                                                                                                                                                                                                                                                                                                                                              libc::c_ulong);
    if customNodeListMax > 0 as libc::c_int {
        // tblMax is set without checking if customNodeListMax will result in a memory overflow
        // this is a non-issue as long as sceneSubdivisionList.nodeListMax is -1
        tblMax = customNodeListMax as u32_0
    } else {
        if (*colCtx).memSize < memSize {
            LogUtils_HungupThread(b"../z_bgcheck.c\x00" as *const u8 as
                                      *const libc::c_char,
                                  4230 as libc::c_int);
        }
        tblMax =
            (*colCtx).memSize.wrapping_sub(memSize).wrapping_div(::std::mem::size_of::<SSNode>()
                                                                     as
                                                                     libc::c_ulong)
    }
    SSNodeList_Initialize(&mut (*colCtx).polyNodes);
    SSNodeList_Alloc(globalCtx, &mut (*colCtx).polyNodes, tblMax as s32,
                     (*(*colCtx).colHeader).numPolygons as s32);
    lookupTblMemSize =
        BgCheck_InitializeStaticLookup(colCtx, globalCtx,
                                       (*colCtx).lookupTbl);
    osSyncPrintf(b"\x1b[32m\x00" as *const u8 as *const libc::c_char);
    osSyncPrintf(b"/*---\xe7\xb5\x90\xe5\xb1\x80 BG\xe4\xbd\xbf\xe7\x94\xa8\xe3\x82\xb5\xe3\x82\xa4\xe3\x82\xba %dbyte---*/\n\x00"
                     as *const u8 as *const libc::c_char,
                 memSize.wrapping_add(lookupTblMemSize));
    osSyncPrintf(b"\x1b[m\x00" as *const u8 as *const libc::c_char);
    DynaPoly_Init(globalCtx, &mut (*colCtx).dyna);
    DynaPoly_Alloc(globalCtx, &mut (*colCtx).dyna);
}
/* *
 * Get CollisionHeader
 * original name: T_BGCheck_getBGDataInfo
 */
#[no_mangle]
pub unsafe extern "C" fn BgCheck_GetCollisionHeader(mut colCtx:
                                                        *mut CollisionContext,
                                                    mut bgId: s32)
 -> *mut CollisionHeader {
    if bgId == 50 as libc::c_int { return (*colCtx).colHeader }
    if bgId < 0 as libc::c_int || bgId > 50 as libc::c_int {
        return 0 as *mut CollisionHeader
    }
    if (*colCtx).dyna.bgActorFlags[bgId as usize] as libc::c_int &
           1 as libc::c_int == 0 {
        osSyncPrintf(b"\x1b[43;30m\x00" as *const u8 as *const libc::c_char);
        osSyncPrintf(b"T_BGCheck_getBGDataInfo():\xe3\x81\x9d\xe3\x81\xaebg_actor_index\xe3\x81\xaf\xe4\xbd\xbf\xe3\x82\x8f\xe3\x82\x8c\xe3\x81\xa6\xe3\x81\x8a\xe3\x82\x8a\xe3\x81\xbe\xe3\x81\x9b\xe3\x82\x93\xe3\x80\x82index=%d\n\x00"
                         as *const u8 as *const libc::c_char);
        osSyncPrintf(b"\x1b[m\x00" as *const u8 as *const libc::c_char);
        return 0 as *mut CollisionHeader
    }
    return (*colCtx).dyna.bgActors[bgId as usize].colHeader;
}
/* *
 * Test if pos is near collision boundaries
 */
#[no_mangle]
pub unsafe extern "C" fn BgCheck_PosInStaticBoundingBox(mut colCtx:
                                                            *mut CollisionContext,
                                                        mut pos: *mut Vec3f)
 -> s32 {
    if (*pos).x < (*colCtx).minBounds.x - 50 as libc::c_int as libc::c_float
           ||
           ((*colCtx).maxBounds.x + 50 as libc::c_int as libc::c_float) <
               (*pos).x ||
           (*pos).y <
               (*colCtx).minBounds.y - 50 as libc::c_int as libc::c_float ||
           ((*colCtx).maxBounds.y + 50 as libc::c_int as libc::c_float) <
               (*pos).y ||
           (*pos).z <
               (*colCtx).minBounds.z - 50 as libc::c_int as libc::c_float ||
           ((*colCtx).maxBounds.z + 50 as libc::c_int as libc::c_float) <
               (*pos).z {
        return 0 as libc::c_int
    }
    return 1 as libc::c_int;
}
/* *
 * Raycast Toward Floor
 * returns the yIntersect of the nearest poly found directly below `pos`, or BGCHECK_Y_MIN if no floor detected
 * returns the poly found in `outPoly`, and the bgId of the entity in `outBgId`
 */
#[no_mangle]
pub unsafe extern "C" fn BgCheck_RaycastFloorImpl(mut globalCtx:
                                                      *mut GlobalContext,
                                                  mut colCtx:
                                                      *mut CollisionContext,
                                                  mut xpFlags: u16_0,
                                                  mut outPoly:
                                                      *mut *mut CollisionPoly,
                                                  mut outBgId: *mut s32,
                                                  mut pos: *mut Vec3f,
                                                  mut actor: *mut Actor,
                                                  mut arg7: u32_0,
                                                  mut chkDist: f32_0)
 -> f32_0 {
    let mut yIntersectDyna: f32_0 = 0.;
    let mut temp_a0: *mut s32 = 0 as *mut s32;
    let mut lookupTbl: *mut StaticLookup = 0 as *mut StaticLookup;
    let mut checkPos: Vec3f = Vec3f{x: 0., y: 0., z: 0.,};
    let mut lookup: *mut StaticLookup = 0 as *mut StaticLookup;
    let mut dynaRaycast: DynaRaycast =
        DynaRaycast{globalCtx: 0 as *mut GlobalContext,
                    colCtx: 0 as *mut CollisionContext,
                    xpFlags: 0,
                    resultPoly: 0 as *mut *mut CollisionPoly,
                    yIntersect: 0.,
                    pos: 0 as *mut Vec3f,
                    bgId: 0 as *mut s32,
                    actor: 0 as *mut Actor,
                    unk_20: 0,
                    chkDist: 0.,
                    dyna: 0 as *mut DynaCollisionContext,
                    ssList: 0 as *mut SSList,};
    let mut yIntersect: f32_0 = 0.;
    *outBgId = 50 as libc::c_int;
    *outPoly = 0 as *mut CollisionPoly;
    lookupTbl = (*colCtx).lookupTbl;
    yIntersect = -32000.0f32;
    checkPos = *pos;
    while !(checkPos.y < (*colCtx).minBounds.y) {
        if BgCheck_PosErrorCheck(&mut checkPos,
                                 b"../z_bgcheck.c\x00" as *const u8 as
                                     *const libc::c_char as *mut libc::c_char,
                                 4410 as libc::c_int) != 0 {
            if !actor.is_null() {
                osSyncPrintf(b"\xe3\x81\x93\xe3\x81\x84\xe3\x81\xa4,pself_actor->name %d\n\x00"
                                 as *const u8 as *const libc::c_char,
                             (*actor).id as libc::c_int);
            }
        }
        lookup = BgCheck_GetStaticLookup(colCtx, lookupTbl, &mut checkPos);
        if lookup.is_null() {
            checkPos.y -= (*colCtx).subdivLength.y
        } else {
            yIntersect =
                BgCheck_RaycastFloorStatic(lookup, colCtx, xpFlags, outPoly,
                                           pos, arg7, chkDist, -32000.0f32);
            if yIntersect > -32000.0f32 { break ; }
            checkPos.y -= (*colCtx).subdivLength.y
        }
    }
    dynaRaycast.colCtx = colCtx;
    dynaRaycast.xpFlags = xpFlags;
    dynaRaycast.yIntersect = yIntersect;
    dynaRaycast.pos = pos;
    dynaRaycast.actor = actor;
    dynaRaycast.unk_20 = arg7;
    dynaRaycast.chkDist = chkDist;
    dynaRaycast.globalCtx = globalCtx;
    dynaRaycast.resultPoly = outPoly;
    dynaRaycast.bgId = outBgId;
    yIntersectDyna = BgCheck_RaycastFloorDyna(&mut dynaRaycast);
    if yIntersect < yIntersectDyna { yIntersect = yIntersectDyna }
    if yIntersect != -32000.0f32 &&
           func_80041EC8(colCtx, *outPoly, *outBgId) != 0 {
        yIntersect -= 1.0f32
    }
    return yIntersect;
}
/* *
 * Public raycast toward floor
 * returns yIntersect of the poly found, or BGCHECK_Y_MIN if no poly detected
 */
#[no_mangle]
pub unsafe extern "C" fn BgCheck_CameraRaycastFloor1(mut colCtx:
                                                         *mut CollisionContext,
                                                     mut outPoly:
                                                         *mut *mut CollisionPoly,
                                                     mut pos: *mut Vec3f)
 -> f32_0 {
    let mut bgId: s32 = 0;
    return BgCheck_RaycastFloorImpl(0 as *mut GlobalContext, colCtx,
                                    ((1 as libc::c_int) << 0 as libc::c_int)
                                        as u16_0, outPoly, &mut bgId, pos,
                                    0 as *mut Actor,
                                    0x1c as libc::c_int as u32_0, 1.0f32);
}
/* *
 * Public raycast toward floor
 * returns yIntersect of the poly found, or BGCHECK_Y_MIN if no poly detected
 */
#[no_mangle]
pub unsafe extern "C" fn BgCheck_EntityRaycastFloor1(mut colCtx:
                                                         *mut CollisionContext,
                                                     mut outPoly:
                                                         *mut *mut CollisionPoly,
                                                     mut pos: *mut Vec3f)
 -> f32_0 {
    let mut bgId: s32 = 0;
    return BgCheck_RaycastFloorImpl(0 as *mut GlobalContext, colCtx,
                                    ((1 as libc::c_int) << 1 as libc::c_int)
                                        as u16_0, outPoly, &mut bgId, pos,
                                    0 as *mut Actor,
                                    0x1c as libc::c_int as u32_0, 1.0f32);
}
/* *
 * Public raycast toward floor
 * returns yIntersect of the poly found, or BGCHECK_Y_MIN if no poly detected
 */
#[no_mangle]
pub unsafe extern "C" fn BgCheck_EntityRaycastFloor2(mut globalCtx:
                                                         *mut GlobalContext,
                                                     mut colCtx:
                                                         *mut CollisionContext,
                                                     mut outPoly:
                                                         *mut *mut CollisionPoly,
                                                     mut pos: *mut Vec3f)
 -> f32_0 {
    let mut bgId: s32 = 0;
    return BgCheck_RaycastFloorImpl(globalCtx, colCtx,
                                    ((1 as libc::c_int) << 1 as libc::c_int)
                                        as u16_0, outPoly, &mut bgId, pos,
                                    0 as *mut Actor,
                                    0x1c as libc::c_int as u32_0, 1.0f32);
}
/* *
 * Public raycast toward floor
 * returns yIntersect of the poly found, or BGCHECK_Y_MIN if no poly detected
 */
#[no_mangle]
pub unsafe extern "C" fn BgCheck_EntityRaycastFloor3(mut colCtx:
                                                         *mut CollisionContext,
                                                     mut outPoly:
                                                         *mut *mut CollisionPoly,
                                                     mut bgId: *mut s32,
                                                     mut pos: *mut Vec3f)
 -> f32_0 {
    return BgCheck_RaycastFloorImpl(0 as *mut GlobalContext, colCtx,
                                    ((1 as libc::c_int) << 1 as libc::c_int)
                                        as u16_0, outPoly, bgId, pos,
                                    0 as *mut Actor,
                                    0x1c as libc::c_int as u32_0, 1.0f32);
}
/* *
 * Public raycast toward floor
 * returns yIntersect of the poly found, or BGCHECK_Y_MIN if no poly detected
 */
#[no_mangle]
pub unsafe extern "C" fn BgCheck_EntityRaycastFloor4(mut colCtx:
                                                         *mut CollisionContext,
                                                     mut outPoly:
                                                         *mut *mut CollisionPoly,
                                                     mut bgId: *mut s32,
                                                     mut actor: *mut Actor,
                                                     mut pos: *mut Vec3f)
 -> f32_0 {
    return BgCheck_RaycastFloorImpl(0 as *mut GlobalContext, colCtx,
                                    ((1 as libc::c_int) << 1 as libc::c_int)
                                        as u16_0, outPoly, bgId, pos, actor,
                                    0x1c as libc::c_int as u32_0, 1.0f32);
}
/* *
 * Public raycast toward floor
 * returns yIntersect of the poly found, or BGCHECK_Y_MIN if no poly detected
 */
#[no_mangle]
pub unsafe extern "C" fn BgCheck_EntityRaycastFloor5(mut globalCtx:
                                                         *mut GlobalContext,
                                                     mut colCtx:
                                                         *mut CollisionContext,
                                                     mut outPoly:
                                                         *mut *mut CollisionPoly,
                                                     mut bgId: *mut s32,
                                                     mut actor: *mut Actor,
                                                     mut pos: *mut Vec3f)
 -> f32_0 {
    return BgCheck_RaycastFloorImpl(globalCtx, colCtx,
                                    ((1 as libc::c_int) << 1 as libc::c_int)
                                        as u16_0, outPoly, bgId, pos, actor,
                                    0x1c as libc::c_int as u32_0, 1.0f32);
}
/* *
 * Public raycast toward floor
 * returns yIntersect of the poly found, or BGCHECK_Y_MIN if no poly detected
 */
#[no_mangle]
pub unsafe extern "C" fn BgCheck_EntityRaycastFloor6(mut colCtx:
                                                         *mut CollisionContext,
                                                     mut outPoly:
                                                         *mut *mut CollisionPoly,
                                                     mut bgId: *mut s32,
                                                     mut actor: *mut Actor,
                                                     mut pos: *mut Vec3f,
                                                     mut chkDist: f32_0)
 -> f32_0 {
    return BgCheck_RaycastFloorImpl(0 as *mut GlobalContext, colCtx,
                                    ((1 as libc::c_int) << 1 as libc::c_int)
                                        as u16_0, outPoly, bgId, pos, actor,
                                    0x1c as libc::c_int as u32_0, chkDist);
}
/* *
 * Public raycast toward floor
 * returns yIntersect of the poly found, or BGCHECK_Y_MIN if no poly detected
 */
#[no_mangle]
pub unsafe extern "C" fn BgCheck_EntityRaycastFloor7(mut colCtx:
                                                         *mut CollisionContext,
                                                     mut outPoly:
                                                         *mut *mut CollisionPoly,
                                                     mut bgId: *mut s32,
                                                     mut actor: *mut Actor,
                                                     mut pos: *mut Vec3f)
 -> f32_0 {
    return BgCheck_RaycastFloorImpl(0 as *mut GlobalContext, colCtx,
                                    ((1 as libc::c_int) << 1 as libc::c_int)
                                        as u16_0, outPoly, bgId, pos, actor,
                                    0x6 as libc::c_int as u32_0, 1.0f32);
}
/* *
 * Public raycast toward floor
 * returns yIntersect of the poly found, or BGCHECK_Y_MIN if no poly detected
 */
#[no_mangle]
pub unsafe extern "C" fn BgCheck_AnyRaycastFloor1(mut colCtx:
                                                      *mut CollisionContext,
                                                  mut outPoly:
                                                      *mut CollisionPoly,
                                                  mut pos: *mut Vec3f)
 -> f32_0 {
    let mut tempPoly: *mut CollisionPoly = 0 as *mut CollisionPoly;
    let mut result: f32_0 = 0.;
    let mut bgId: s32 = 0;
    result =
        BgCheck_RaycastFloorImpl(0 as *mut GlobalContext, colCtx,
                                 0 as libc::c_int as u16_0, &mut tempPoly,
                                 &mut bgId, pos, 0 as *mut Actor,
                                 0x1c as libc::c_int as u32_0, 1.0f32);
    if !tempPoly.is_null() { *outPoly = *tempPoly }
    return result;
}
/* *
 * Public raycast toward floor
 * returns yIntersect of the poly found, or BGCHECK_Y_MIN if no poly detected
 */
#[no_mangle]
pub unsafe extern "C" fn BgCheck_AnyRaycastFloor2(mut colCtx:
                                                      *mut CollisionContext,
                                                  mut outPoly:
                                                      *mut CollisionPoly,
                                                  mut bgId: *mut s32,
                                                  mut pos: *mut Vec3f)
 -> f32_0 {
    let mut tempPoly: *mut CollisionPoly = 0 as *mut CollisionPoly;
    let mut result: f32_0 =
        BgCheck_RaycastFloorImpl(0 as *mut GlobalContext, colCtx,
                                 0 as libc::c_int as u16_0, &mut tempPoly,
                                 bgId, pos, 0 as *mut Actor,
                                 0x1c as libc::c_int as u32_0, 1.0f32);
    if !tempPoly.is_null() { *outPoly = *tempPoly }
    return result;
}
/* *
 * Public raycast toward floor
 * returns yIntersect of the poly found, or BGCHECK_Y_MIN if no poly detected
 */
#[no_mangle]
pub unsafe extern "C" fn BgCheck_CameraRaycastFloor2(mut colCtx:
                                                         *mut CollisionContext,
                                                     mut outPoly:
                                                         *mut *mut CollisionPoly,
                                                     mut bgId: *mut s32,
                                                     mut pos: *mut Vec3f)
 -> f32_0 {
    return BgCheck_RaycastFloorImpl(0 as *mut GlobalContext, colCtx,
                                    ((1 as libc::c_int) << 0 as libc::c_int)
                                        as u16_0, outPoly, bgId, pos,
                                    0 as *mut Actor,
                                    0x6 as libc::c_int as u32_0, 1.0f32);
}
/* *
 * Public raycast toward floor
 * returns yIntersect of the poly found, or BGCHECK_Y_MIN if no poly detected
 */
#[no_mangle]
pub unsafe extern "C" fn BgCheck_EntityRaycastFloor8(mut colCtx:
                                                         *mut CollisionContext,
                                                     mut outPoly:
                                                         *mut *mut CollisionPoly,
                                                     mut bgId: *mut s32,
                                                     mut actor: *mut Actor,
                                                     mut pos: *mut Vec3f)
 -> f32_0 {
    return BgCheck_RaycastFloorImpl(0 as *mut GlobalContext, colCtx,
                                    ((1 as libc::c_int) << 1 as libc::c_int)
                                        as u16_0, outPoly, bgId, pos, actor,
                                    0x2 as libc::c_int as u32_0, 1.0f32);
}
/* *
 * Public raycast toward floor
 * returns yIntersect of the poly found, or BGCHECK_Y_MIN if no poly detected
 */
#[no_mangle]
pub unsafe extern "C" fn BgCheck_EntityRaycastFloor9(mut colCtx:
                                                         *mut CollisionContext,
                                                     mut outPoly:
                                                         *mut *mut CollisionPoly,
                                                     mut bgId: *mut s32,
                                                     mut pos: *mut Vec3f)
 -> f32_0 {
    return BgCheck_RaycastFloorImpl(0 as *mut GlobalContext, colCtx,
                                    ((1 as libc::c_int) << 1 as libc::c_int)
                                        as u16_0, outPoly, bgId, pos,
                                    0 as *mut Actor,
                                    0x6 as libc::c_int as u32_0, 1.0f32);
}
/* *
 * Tests if moving from `posPrev` to `posNext` will collide with a "wall"
 * `radius` is used to form a sphere for collision detection purposes
 * `checkHeight` is the positive height above posNext to perform certain checks
 * returns true if a collision is detected, else false
 * `outPoly` returns the closest poly detected, while `outBgId` returns the poly owner
 */
#[no_mangle]
pub unsafe extern "C" fn BgCheck_CheckWallImpl(mut colCtx:
                                                   *mut CollisionContext,
                                               mut xpFlags: u16_0,
                                               mut posResult: *mut Vec3f,
                                               mut posNext: *mut Vec3f,
                                               mut posPrev: *mut Vec3f,
                                               mut radius: f32_0,
                                               mut outPoly:
                                                   *mut *mut CollisionPoly,
                                               mut outBgId: *mut s32,
                                               mut actor: *mut Actor,
                                               mut checkHeight: f32_0,
                                               mut argA: u8_0) -> s32 {
    let mut lookupTbl: *mut StaticLookup =
        0 as *mut StaticLookup; // change between posPrev to posNext
    let mut temp_f0: f32_0 = 0.; // unit normal of polygon
    let mut result: s32 = 0;
    let mut poly: *mut CollisionPoly = 0 as *mut CollisionPoly;
    let mut dx: f32_0 = 0.;
    let mut dy: f32_0 = 0.;
    let mut dz: f32_0 = 0.;
    let mut sphCenter: Vec3f = Vec3f{x: 0., y: 0., z: 0.,};
    let mut dynaPolyCollision: s32 = 0;
    let mut posIntersect: Vec3f = Vec3f{x: 0., y: 0., z: 0.,};
    let mut bgId: s32 = 0;
    let mut temp_f0_2: f32_0 = 0.;
    let mut f32temp: f32_0 = 0.;
    let mut nx2: f32_0 = 0.;
    let mut nz2: f32_0 = 0.;
    let mut checkLineNext: Vec3f = Vec3f{x: 0., y: 0., z: 0.,};
    let mut checkLinePrev: Vec3f = Vec3f{x: 0., y: 0., z: 0.,};
    let mut n2XZDist: f32_0 = 0.;
    let mut n3XZDist: f32_0 = 0.;
    let mut nx3: f32_0 = 0.;
    let mut nz3: f32_0 = 0.;
    let mut bccFlags: s32 = 0;
    let mut posIntersect2: Vec3f = Vec3f{x: 0., y: 0., z: 0.,};
    let mut bgId2: s32 = 0;
    let mut nx: f32_0 = 0.;
    let mut ny: f32_0 = 0.;
    let mut nz: f32_0 = 0.;
    result = 0 as libc::c_int;
    *outBgId = 50 as libc::c_int;
    *outPoly = 0 as *mut CollisionPoly;
    lookupTbl = (*colCtx).lookupTbl;
    *posResult = *posNext;
    dx = (*posNext).x - (*posPrev).x;
    dy = (*posNext).y - (*posPrev).y;
    dz = (*posNext).z - (*posPrev).z;
    if BgCheck_PosErrorCheck(posNext,
                             b"../z_bgcheck.c\x00" as *const u8 as
                                 *const libc::c_char as *mut libc::c_char,
                             4831 as libc::c_int) == 1 as libc::c_int ||
           BgCheck_PosErrorCheck(posPrev,
                                 b"../z_bgcheck.c\x00" as *const u8 as
                                     *const libc::c_char as *mut libc::c_char,
                                 4832 as libc::c_int) == 1 as libc::c_int {
        if !actor.is_null() {
            osSyncPrintf(b"\xe3\x81\x93\xe3\x81\x84\xe3\x81\xa4,pself_actor->name %d\n\x00"
                             as *const u8 as *const libc::c_char,
                         (*actor).id as libc::c_int);
        }
    }
    // if there's movement on the xz plane, and argA flag is 0,
    if (dx != 0.0f32 || dz != 0.0f32) &&
           argA as libc::c_int & 1 as libc::c_int == 0 as libc::c_int {
        if checkHeight + dy < 5.0f32 {
            // ! @bug checkHeight is not applied to posPrev/posNext
            result =
                BgCheck_CheckLineImpl(colCtx, xpFlags,
                                      0 as libc::c_int as u16_0, posPrev,
                                      posNext, &mut posIntersect, &mut poly,
                                      &mut bgId, actor, 1.0f32,
                                      (((1 as libc::c_int) << 0 as libc::c_int
                                            |
                                            (1 as libc::c_int) <<
                                                1 as libc::c_int |
                                            (1 as libc::c_int) <<
                                                2 as libc::c_int |
                                            (1 as libc::c_int) <<
                                                3 as libc::c_int |
                                            (1 as libc::c_int) <<
                                                4 as libc::c_int) &
                                           !((1 as libc::c_int) <<
                                                 2 as libc::c_int)) as u32_0);
            if result != 0 {
                ny =
                    (*poly).normal.y as libc::c_int as libc::c_float *
                        (1.0f32 / 32767.0f32);
                // if poly is floor, push result underneath the floor
                if ny > 0.5f32 {
                    (*posResult).x = posIntersect.x;
                    if checkHeight > 1.0f32 {
                        (*posResult).y = posIntersect.y - 1.0f32
                    } else { (*posResult).y = posIntersect.y - checkHeight }
                    (*posResult).z = posIntersect.z
                } else {
                    // poly is wall
                    nx =
                        (*poly).normal.x as libc::c_int as libc::c_float *
                            (1.0f32 / 32767.0f32);
                    nz =
                        (*poly).normal.z as libc::c_int as libc::c_float *
                            (1.0f32 / 32767.0f32);
                    (*posResult).x = radius * nx + posIntersect.x;
                    (*posResult).y = radius * ny + posIntersect.y;
                    (*posResult).z = radius * nz + posIntersect.z
                }
                *outPoly = poly;
                *outBgId = bgId
            }
        } else {
            // if the radius is less than the distance travelled on the xz plane, also test for floor collisions
            bccFlags =
                if radius * radius < dx * dx + dz * dz {
                    ((1 as libc::c_int) << 0 as libc::c_int |
                         (1 as libc::c_int) << 1 as libc::c_int |
                         (1 as libc::c_int) << 2 as libc::c_int |
                         (1 as libc::c_int) << 3 as libc::c_int |
                         (1 as libc::c_int) << 4 as libc::c_int) &
                        !((1 as libc::c_int) << 2 as libc::c_int)
                } else {
                    (((1 as libc::c_int) << 0 as libc::c_int |
                          (1 as libc::c_int) << 1 as libc::c_int |
                          (1 as libc::c_int) << 2 as libc::c_int |
                          (1 as libc::c_int) << 3 as libc::c_int |
                          (1 as libc::c_int) << 4 as libc::c_int) &
                         !((1 as libc::c_int) << 1 as libc::c_int)) &
                        !((1 as libc::c_int) << 2 as libc::c_int)
                };
            // perform a straight line test to see if a line at posNext.y + checkHeight from posPrev.xz to posNext.xz
            // passes through any wall and possibly floor polys
            checkLineNext = *posNext;
            checkLineNext.y += checkHeight;
            checkLinePrev = *posPrev;
            checkLinePrev.y = checkLineNext.y;
            result =
                BgCheck_CheckLineImpl(colCtx, xpFlags,
                                      0 as libc::c_int as u16_0,
                                      &mut checkLinePrev, &mut checkLineNext,
                                      &mut posIntersect, &mut poly, &mut bgId,
                                      actor, 1.0f32, bccFlags as u32_0);
            if result != 0 {
                nx2 =
                    (*poly).normal.x as libc::c_int as libc::c_float *
                        (1.0f32 / 32767.0f32);
                nz2 =
                    (*poly).normal.z as libc::c_int as libc::c_float *
                        (1.0f32 / 32767.0f32);
                n2XZDist = sqrtf(nx2 * nx2 + nz2 * nz2);
                // if poly is not a "flat" floor or "flat" ceiling
                if !(fabsf(n2XZDist) < 0.008f32) {
                    // normalize nx,nz and multiply each by the radius to go back to the other side of the wall
                    f32temp = 1.0f32 / n2XZDist;
                    temp_f0 = radius * f32temp;
                    (*posResult).x = temp_f0 * nx2 + posIntersect.x;
                    (*posResult).z = temp_f0 * nz2 + posIntersect.z;
                    *outPoly = poly;
                    *outBgId = bgId;
                    result = 1 as libc::c_int
                }
            }
        }
    }
    sphCenter = *posResult;
    dynaPolyCollision = 0 as libc::c_int;
    sphCenter.y += checkHeight;
    // test if sphere (sphCenter, radius) collides with a dynamic wall, displacing the x/z coordinates
    if BgCheck_SphVsDynaWall(colCtx, xpFlags, &mut (*posResult).x,
                             &mut (*posResult).z, &mut sphCenter, radius,
                             outPoly, outBgId, actor) != 0 {
        result = 1 as libc::c_int;
        dynaPolyCollision = 1 as libc::c_int;
        sphCenter = *posResult;
        sphCenter.y += checkHeight
    }
    // test if sphere (sphCenter, radius) collides with a static wall, displacing the x/z coordinates
    if BgCheck_PosInStaticBoundingBox(colCtx, posNext) == 1 as libc::c_int &&
           BgCheck_SphVsStaticWall(BgCheck_GetNearestStaticLookup(colCtx,
                                                                  lookupTbl,
                                                                  posResult),
                                   colCtx, xpFlags, &mut (*posResult).x,
                                   &mut (*posResult).z, &mut sphCenter,
                                   radius, outPoly) != 0 {
        *outBgId = 50 as libc::c_int;
        result = 1 as libc::c_int
    }
    // if a collision with a dyna poly was detected
    if dynaPolyCollision == 1 as libc::c_int || *outBgId != 50 as libc::c_int
       {
        if BgCheck_CheckLineImpl(colCtx, xpFlags, 0 as libc::c_int as u16_0,
                                 posPrev, posResult, &mut posIntersect2,
                                 &mut poly, &mut bgId2, actor, 1.0f32,
                                 ((1 as libc::c_int) << 3 as libc::c_int |
                                      (1 as libc::c_int) << 0 as libc::c_int)
                                     as u32_0) != 0 {
            nx3 =
                (*poly).normal.x as libc::c_int as libc::c_float *
                    (1.0f32 / 32767.0f32);
            nz3 =
                (*poly).normal.z as libc::c_int as libc::c_float *
                    (1.0f32 / 32767.0f32);
            n3XZDist = sqrtf(nx3 * nx3 + nz3 * nz3);
            // if poly is not a "flat" floor or "flat" ceiling
            if !(fabsf(n3XZDist) < 0.008f32) {
                // normalize nx,nz and multiply each by the radius to go back to the other side of the wall
                f32temp = 1.0f32 / n3XZDist;
                temp_f0_2 = radius * f32temp;
                (*posResult).x = temp_f0_2 * nx3 + posIntersect2.x;
                (*posResult).z = temp_f0_2 * nz3 + posIntersect2.z;
                *outPoly = poly;
                *outBgId = bgId2;
                result = 1 as libc::c_int
            }
        }
    }
    return result;
}
/* *
 * Public. Tests if moving from `posPrev` to `posNext` will collide with a "wall"
 * `radius` is used to form a sphere for collision detection purposes
 * `checkHeight` is the positive height above posNext to perform certain checks
 * returns true if a collision is detected, else false
 * `outPoly` returns the closest poly detected
 */
#[no_mangle]
pub unsafe extern "C" fn BgCheck_EntitySphVsWall1(mut colCtx:
                                                      *mut CollisionContext,
                                                  mut posResult: *mut Vec3f,
                                                  mut posNext: *mut Vec3f,
                                                  mut posPrev: *mut Vec3f,
                                                  mut radius: f32_0,
                                                  mut outPoly:
                                                      *mut *mut CollisionPoly,
                                                  mut checkHeight: f32_0)
 -> s32 {
    let mut bgId: s32 = 0;
    return BgCheck_CheckWallImpl(colCtx,
                                 ((1 as libc::c_int) << 1 as libc::c_int) as
                                     u16_0, posResult, posNext, posPrev,
                                 radius, outPoly, &mut bgId, 0 as *mut Actor,
                                 checkHeight, 0 as libc::c_int as u8_0);
}
/* *
 * Public. Tests if moving from `posPrev` to `posNext` will collide with a "wall"
 * `radius` is used to form a sphere for collision detection purposes
 * `checkHeight` is the positive height above posNext to perform certain checks
 * returns true if a collision is detected, else false
 * `outPoly` returns the closest poly detected, while `outBgId` returns the poly owner
 */
#[no_mangle]
pub unsafe extern "C" fn BgCheck_EntitySphVsWall2(mut colCtx:
                                                      *mut CollisionContext,
                                                  mut posResult: *mut Vec3f,
                                                  mut posNext: *mut Vec3f,
                                                  mut posPrev: *mut Vec3f,
                                                  mut radius: f32_0,
                                                  mut outPoly:
                                                      *mut *mut CollisionPoly,
                                                  mut outBgId: *mut s32,
                                                  mut checkHeight: f32_0)
 -> s32 {
    return BgCheck_CheckWallImpl(colCtx,
                                 ((1 as libc::c_int) << 1 as libc::c_int) as
                                     u16_0, posResult, posNext, posPrev,
                                 radius, outPoly, outBgId, 0 as *mut Actor,
                                 checkHeight, 0 as libc::c_int as u8_0);
}
/* *
 * Public. Tests if moving from `posPrev` to `posNext` will collide with a "wall"
 * `radius` is used to form a sphere for collision detection purposes
 * `checkHeight` is the positive height above posNext to perform certain checks
 * `actor` is the actor performing the check, allowing it to be skipped
 * returns true if a collision is detected, else false
 * `outPoly` returns the closest poly detected, while `outBgId` returns the poly owner
 */
#[no_mangle]
pub unsafe extern "C" fn BgCheck_EntitySphVsWall3(mut colCtx:
                                                      *mut CollisionContext,
                                                  mut posResult: *mut Vec3f,
                                                  mut posNext: *mut Vec3f,
                                                  mut posPrev: *mut Vec3f,
                                                  mut radius: f32_0,
                                                  mut outPoly:
                                                      *mut *mut CollisionPoly,
                                                  mut outBgId: *mut s32,
                                                  mut actor: *mut Actor,
                                                  mut checkHeight: f32_0)
 -> s32 {
    return BgCheck_CheckWallImpl(colCtx,
                                 ((1 as libc::c_int) << 1 as libc::c_int) as
                                     u16_0, posResult, posNext, posPrev,
                                 radius, outPoly, outBgId, actor, checkHeight,
                                 0 as libc::c_int as u8_0);
}
/* **
 * Public. Tests if moving from `posPrev` to `posNext` will collide with a "wall"
 * Skips a check that occurs only when moving on the xz plane
 * `radius` is used to form a sphere for collision detection purposes
 * `checkHeight` is the positive height above posNext to perform certain checks
 * `actor` is the actor performing the check, allowing it to be skipped
 * returns true if a collision is detected, else false
 * `outPoly` returns the closest poly detected, while `outBgId` returns the poly owner
 */
#[no_mangle]
pub unsafe extern "C" fn BgCheck_EntitySphVsWall4(mut colCtx:
                                                      *mut CollisionContext,
                                                  mut posResult: *mut Vec3f,
                                                  mut posNext: *mut Vec3f,
                                                  mut posPrev: *mut Vec3f,
                                                  mut radius: f32_0,
                                                  mut outPoly:
                                                      *mut *mut CollisionPoly,
                                                  mut outBgId: *mut s32,
                                                  mut actor: *mut Actor,
                                                  mut checkHeight: f32_0)
 -> s32 {
    return BgCheck_CheckWallImpl(colCtx,
                                 ((1 as libc::c_int) << 1 as libc::c_int) as
                                     u16_0, posResult, posNext, posPrev,
                                 radius, outPoly, outBgId, actor, checkHeight,
                                 1 as libc::c_int as u8_0);
}
/* **
 * Tests for collision with a ceiling poly
 * `checkHeight` should be a positive value
 * returns true if a collision occurs, else false
 * `outPoly` returns the poly collided with, while `outBgId` returns the owner of the poly
 * `outY` returns the y coordinate of pos needed to not collide with `outPoly`
 */
#[no_mangle]
pub unsafe extern "C" fn BgCheck_CheckCeilingImpl(mut colCtx:
                                                      *mut CollisionContext,
                                                  mut xpFlags: u16_0,
                                                  mut outY: *mut f32_0,
                                                  mut pos: *mut Vec3f,
                                                  mut checkHeight: f32_0,
                                                  mut outPoly:
                                                      *mut *mut CollisionPoly,
                                                  mut outBgId: *mut s32,
                                                  mut actor: *mut Actor)
 -> s32 {
    let mut lookupTbl: *mut StaticLookup = 0 as *mut StaticLookup;
    let mut lookup: *mut StaticLookup = 0 as *mut StaticLookup;
    let mut result: s32 = 0;
    let mut posTemp: Vec3f = Vec3f{x: 0., y: 0., z: 0.,};
    let mut tempY: f32_0 = 0.;
    *outBgId = 50 as libc::c_int;
    *outY = (*pos).y;
    if BgCheck_PosErrorCheck(pos,
                             b"../z_bgcheck.c\x00" as *const u8 as
                                 *const libc::c_char as *mut libc::c_char,
                             5206 as libc::c_int) == 1 as libc::c_int {
        if !actor.is_null() {
            osSyncPrintf(b"\xe3\x81\x93\xe3\x81\x84\xe3\x81\xa4,pself_actor->name %d\n\x00"
                             as *const u8 as *const libc::c_char,
                         (*actor).id as libc::c_int);
        }
    }
    lookupTbl = (*colCtx).lookupTbl;
    if BgCheck_PosInStaticBoundingBox(colCtx, pos) == 0 {
        return 0 as libc::c_int
    }
    lookup = BgCheck_GetNearestStaticLookup(colCtx, lookupTbl, pos);
    result =
        BgCheck_CheckStaticCeiling(lookup, xpFlags, colCtx, outY, pos,
                                   checkHeight, outPoly);
    posTemp = *pos;
    posTemp.y = *outY;
    tempY = *outY;
    if BgCheck_CheckDynaCeiling(colCtx, xpFlags, &mut tempY, &mut posTemp,
                                checkHeight, outPoly, outBgId, actor) != 0 {
        *outY = tempY;
        result = 1 as libc::c_int
    }
    return result;
}
/* *
 * Tests for collision with any ceiling poly
 * `checkHeight` must be a positive value
 * returns true if a collision occurs, else false
 * `outY` returns the displaced y coordinate needed to not collide with the poly
 */
#[no_mangle]
pub unsafe extern "C" fn BgCheck_AnyCheckCeiling(mut colCtx:
                                                     *mut CollisionContext,
                                                 mut outY: *mut f32_0,
                                                 mut pos: *mut Vec3f,
                                                 mut checkHeight: f32_0)
 -> s32 {
    let mut poly: *mut CollisionPoly = 0 as *mut CollisionPoly;
    let mut bgId: s32 = 0;
    return BgCheck_CheckCeilingImpl(colCtx, 0 as libc::c_int as u16_0, outY,
                                    pos, checkHeight, &mut poly, &mut bgId,
                                    0 as *mut Actor);
}
/* *
 * Tests for collision with any entity solid ceiling poly
 * `checkHeight` must be a positive value
 * returns true if a collision occurs, else false
 * `outY` returns the displaced y coordinate needed to not collide with the poly
 */
#[no_mangle]
pub unsafe extern "C" fn BgCheck_EntityCheckCeiling(mut colCtx:
                                                        *mut CollisionContext,
                                                    mut outY: *mut f32_0,
                                                    mut pos: *mut Vec3f,
                                                    mut checkHeight: f32_0,
                                                    mut outPoly:
                                                        *mut *mut CollisionPoly,
                                                    mut outBgId: *mut s32,
                                                    mut actor: *mut Actor)
 -> s32 {
    return BgCheck_CheckCeilingImpl(colCtx,
                                    ((1 as libc::c_int) << 1 as libc::c_int)
                                        as u16_0, outY, pos, checkHeight,
                                    outPoly, outBgId, actor);
}
/* *
 * Tests if a line from `posA` to `posB` intersects with a poly
 * returns true if it does, else false
 * `posB`? `posResult` returns the point of intersection
 * `outPoly` returns the pointer to the intersected poly, while `outBgId` returns the entity the poly belongs to
 */
#[no_mangle]
pub unsafe extern "C" fn BgCheck_CheckLineImpl(mut colCtx:
                                                   *mut CollisionContext,
                                               mut xpFlags1: u16_0,
                                               mut xpFlags2: u16_0,
                                               mut posA: *mut Vec3f,
                                               mut posB: *mut Vec3f,
                                               mut posResult: *mut Vec3f,
                                               mut outPoly:
                                                   *mut *mut CollisionPoly,
                                               mut outBgId: *mut s32,
                                               mut actor: *mut Actor,
                                               mut chkDist: f32_0,
                                               mut bccFlags: u32_0) -> s32 {
    let mut lookupTbl: *mut StaticLookup = (*colCtx).lookupTbl;
    let mut iLookup: *mut StaticLookup = 0 as *mut StaticLookup;
    let mut subdivMin: [s32; 3] = [0; 3];
    let mut subdivMax: [s32; 3] = [0; 3];
    let mut i: s32 = 0;
    let mut result: s32 = 0;
    let mut distSq: f32_0 = 0.;
    let mut posBTemp: Vec3f = *posB;
    let mut sectorMin: Vec3f = Vec3f{x: 0., y: 0., z: 0.,};
    let mut sectorMax: Vec3f = Vec3f{x: 0., y: 0., z: 0.,};
    let mut k: s32 = 0;
    let mut lookup: *mut StaticLookup = 0 as *mut StaticLookup;
    let mut j: s32 = 0;
    let mut jLookup: *mut StaticLookup = 0 as *mut StaticLookup;
    let mut temp_lo: s32 = 0;
    *outBgId = 50 as libc::c_int;
    if BgCheck_PosErrorCheck(posA,
                             b"../z_bgcheck.c\x00" as *const u8 as
                                 *const libc::c_char as *mut libc::c_char,
                             5334 as libc::c_int) == 1 as libc::c_int ||
           BgCheck_PosErrorCheck(posB,
                                 b"../z_bgcheck.c\x00" as *const u8 as
                                     *const libc::c_char as *mut libc::c_char,
                                 5335 as libc::c_int) == 1 as libc::c_int {
        if !actor.is_null() {
            osSyncPrintf(b"\xe3\x81\x93\xe3\x81\x84\xe3\x81\xa4,pself_actor->name %d\n\x00"
                             as *const u8 as *const libc::c_char,
                         (*actor).id as libc::c_int);
        } else {
            osSyncPrintf(b"pself_actor == NULL\xe3\x81\xa7\xe7\x8a\xaf\xe4\xba\xba\xe4\xb8\x8d\xe6\x98\x8e\n\x00"
                             as *const u8 as *const libc::c_char);
        }
    }
    BgCheck_ResetPolyCheckTbl(&mut (*colCtx).polyNodes,
                              (*(*colCtx).colHeader).numPolygons as s32);
    BgCheck_GetStaticLookupIndicesFromPos(colCtx, posA,
                                          &mut subdivMin as *mut [s32; 3] as
                                              *mut Vec3i);
    BgCheck_GetStaticLookupIndicesFromPos(colCtx, &mut posBTemp,
                                          &mut subdivMax as *mut [s32; 3] as
                                              *mut Vec3i);
    *posResult = *posB;
    result = 0 as libc::c_int;
    distSq = 1.0e38f32;
    *outPoly = 0 as *mut CollisionPoly;
    if subdivMin[0 as libc::c_int as usize] !=
           subdivMax[0 as libc::c_int as usize] ||
           subdivMin[1 as libc::c_int as usize] !=
               subdivMax[1 as libc::c_int as usize] ||
           subdivMin[2 as libc::c_int as usize] !=
               subdivMax[2 as libc::c_int as usize] {
        i = 0 as libc::c_int;
        while i < 3 as libc::c_int {
            if subdivMax[i as usize] < subdivMin[i as usize] {
                j = subdivMax[i as usize];
                subdivMax[i as usize] = subdivMin[i as usize];
                subdivMin[i as usize] = j
            }
            i += 1
        }
        temp_lo = (*colCtx).subdivAmount.x * (*colCtx).subdivAmount.y;
        iLookup =
            lookupTbl.offset((subdivMin[2 as libc::c_int as usize] * temp_lo)
                                 as isize);
        sectorMin.z =
            subdivMin[2 as libc::c_int as usize] as libc::c_float *
                (*colCtx).subdivLength.z + (*colCtx).minBounds.z;
        sectorMax.z = (*colCtx).subdivLength.z + sectorMin.z;
        i = subdivMin[2 as libc::c_int as usize];
        while i < subdivMax[2 as libc::c_int as usize] + 1 as libc::c_int {
            jLookup =
                iLookup.offset((subdivMin[1 as libc::c_int as usize] *
                                    (*colCtx).subdivAmount.x) as isize);
            sectorMin.y =
                subdivMin[1 as libc::c_int as usize] as libc::c_float *
                    (*colCtx).subdivLength.y + (*colCtx).minBounds.y;
            sectorMax.y = (*colCtx).subdivLength.y + sectorMin.y;
            j = subdivMin[1 as libc::c_int as usize];
            while j < subdivMax[1 as libc::c_int as usize] + 1 as libc::c_int
                  {
                lookup =
                    jLookup.offset(subdivMin[0 as libc::c_int as usize] as
                                       isize);
                sectorMin.x =
                    subdivMin[0 as libc::c_int as usize] as libc::c_float *
                        (*colCtx).subdivLength.x + (*colCtx).minBounds.x;
                sectorMax.x = (*colCtx).subdivLength.x + sectorMin.x;
                k = subdivMin[0 as libc::c_int as usize];
                while k <
                          subdivMax[0 as libc::c_int as usize] +
                              1 as libc::c_int {
                    if Math3D_LineVsCube(&mut sectorMin, &mut sectorMax, posA,
                                         &mut posBTemp) == 1 as libc::c_int &&
                           BgCheck_CheckLineInSubdivision(lookup, colCtx,
                                                          xpFlags1, xpFlags2,
                                                          posA, &mut posBTemp,
                                                          posResult, outPoly,
                                                          chkDist,
                                                          &mut distSq,
                                                          bccFlags) != 0 {
                        result = 1 as libc::c_int
                    }
                    lookup = lookup.offset(1);
                    sectorMin.x += (*colCtx).subdivLength.x;
                    sectorMax.x += (*colCtx).subdivLength.x;
                    k += 1
                }
                jLookup = jLookup.offset((*colCtx).subdivAmount.x as isize);
                sectorMin.y += (*colCtx).subdivLength.y;
                sectorMax.y += (*colCtx).subdivLength.y;
                j += 1
            }
            iLookup = iLookup.offset(temp_lo as isize);
            sectorMin.z += (*colCtx).subdivLength.z;
            sectorMax.z += (*colCtx).subdivLength.z;
            i += 1
        }
    } else if BgCheck_PosInStaticBoundingBox(colCtx, posA) == 0 as libc::c_int
     {
        return 0 as libc::c_int
    } else {
        result =
            BgCheck_CheckLineInSubdivision(BgCheck_GetNearestStaticLookup(colCtx,
                                                                          lookupTbl,
                                                                          posA),
                                           colCtx, xpFlags1, xpFlags2, posA,
                                           &mut posBTemp, posResult, outPoly,
                                           chkDist, &mut distSq, bccFlags);
        if result == 1 as libc::c_int {
            distSq = Math3D_Vec3fDistSq(posResult, posA)
        }
    }
    if bccFlags & ((1 as libc::c_int) << 4 as libc::c_int) as libc::c_uint !=
           0 &&
           BgCheck_CheckLineAgainstDyna(colCtx, xpFlags1, posA, &mut posBTemp,
                                        posResult, outPoly, &mut distSq,
                                        outBgId, actor, chkDist,
                                        bccFlags as s32) != 0 {
        result = 1 as libc::c_int
    }
    return result;
}
/* *
 * Get bccFlags
 */
#[no_mangle]
pub unsafe extern "C" fn BgCheck_GetBccFlags(mut chkWall: s32,
                                             mut chkFloor: s32,
                                             mut chkCeil: s32,
                                             mut chkOneFace: s32,
                                             mut chkDyna: s32) -> u32_0 {
    let mut result: u32_0 = 0 as libc::c_int as u32_0;
    if chkWall != 0 {
        result = ((1 as libc::c_int) << 0 as libc::c_int) as u32_0
    }
    if chkFloor != 0 {
        result |= ((1 as libc::c_int) << 1 as libc::c_int) as libc::c_uint
    }
    if chkCeil != 0 {
        result |= ((1 as libc::c_int) << 2 as libc::c_int) as libc::c_uint
    }
    if chkOneFace != 0 {
        result |= ((1 as libc::c_int) << 3 as libc::c_int) as libc::c_uint
    }
    if chkDyna != 0 {
        result |= ((1 as libc::c_int) << 4 as libc::c_int) as libc::c_uint
    }
    return result;
}
/* *
 * Public. Tests if a line from `posA` to `posB` intersects with a poly
 * returns true if it does, else false
 */
#[no_mangle]
pub unsafe extern "C" fn BgCheck_CameraLineTest1(mut colCtx:
                                                     *mut CollisionContext,
                                                 mut posA: *mut Vec3f,
                                                 mut posB: *mut Vec3f,
                                                 mut posResult: *mut Vec3f,
                                                 mut outPoly:
                                                     *mut *mut CollisionPoly,
                                                 mut chkWall: s32,
                                                 mut chkFloor: s32,
                                                 mut chkCeil: s32,
                                                 mut chkOneFace: s32,
                                                 mut bgId: *mut s32) -> s32 {
    return BgCheck_CheckLineImpl(colCtx,
                                 ((1 as libc::c_int) << 0 as libc::c_int) as
                                     u16_0, 0 as libc::c_int as u16_0, posA,
                                 posB, posResult, outPoly, bgId,
                                 0 as *mut Actor, 1.0f32,
                                 BgCheck_GetBccFlags(chkWall, chkFloor,
                                                     chkCeil, chkOneFace,
                                                     1 as libc::c_int));
}
/* *
 * Public. Tests if a line from `posA` to `posB` intersects with a poly
 * returns true if it does, else false
 */
#[no_mangle]
pub unsafe extern "C" fn BgCheck_CameraLineTest2(mut colCtx:
                                                     *mut CollisionContext,
                                                 mut posA: *mut Vec3f,
                                                 mut posB: *mut Vec3f,
                                                 mut posResult: *mut Vec3f,
                                                 mut outPoly:
                                                     *mut *mut CollisionPoly,
                                                 mut chkWall: s32,
                                                 mut chkFloor: s32,
                                                 mut chkCeil: s32,
                                                 mut chkOneFace: s32,
                                                 mut bgId: *mut s32) -> s32 {
    return BgCheck_CheckLineImpl(colCtx, 0 as libc::c_int as u16_0,
                                 ((1 as libc::c_int) << 0 as libc::c_int) as
                                     u16_0, posA, posB, posResult, outPoly,
                                 bgId, 0 as *mut Actor, 1.0f32,
                                 BgCheck_GetBccFlags(chkWall, chkFloor,
                                                     chkCeil, chkOneFace,
                                                     1 as libc::c_int));
}
/* *
 * Public. Tests if a line from `posA` to `posB` intersects with a poly
 * returns true if it does, else false
 */
#[no_mangle]
pub unsafe extern "C" fn BgCheck_EntityLineTest1(mut colCtx:
                                                     *mut CollisionContext,
                                                 mut posA: *mut Vec3f,
                                                 mut posB: *mut Vec3f,
                                                 mut posResult: *mut Vec3f,
                                                 mut outPoly:
                                                     *mut *mut CollisionPoly,
                                                 mut chkWall: s32,
                                                 mut chkFloor: s32,
                                                 mut chkCeil: s32,
                                                 mut chkOneFace: s32,
                                                 mut bgId: *mut s32) -> s32 {
    return BgCheck_CheckLineImpl(colCtx,
                                 ((1 as libc::c_int) << 1 as libc::c_int) as
                                     u16_0, 0 as libc::c_int as u16_0, posA,
                                 posB, posResult, outPoly, bgId,
                                 0 as *mut Actor, 1.0f32,
                                 BgCheck_GetBccFlags(chkWall, chkFloor,
                                                     chkCeil, chkOneFace,
                                                     1 as libc::c_int));
}
/* *
 * Public. Tests if a line from `posA` to `posB` intersects with a poly
 * returns true if it does, else false
 */
#[no_mangle]
pub unsafe extern "C" fn BgCheck_EntityLineTest2(mut colCtx:
                                                     *mut CollisionContext,
                                                 mut posA: *mut Vec3f,
                                                 mut posB: *mut Vec3f,
                                                 mut posResult: *mut Vec3f,
                                                 mut outPoly:
                                                     *mut *mut CollisionPoly,
                                                 mut chkWall: s32,
                                                 mut chkFloor: s32,
                                                 mut chkCeil: s32,
                                                 mut chkOneFace: s32,
                                                 mut bgId: *mut s32,
                                                 mut actor: *mut Actor)
 -> s32 {
    return BgCheck_CheckLineImpl(colCtx,
                                 ((1 as libc::c_int) << 1 as libc::c_int) as
                                     u16_0, 0 as libc::c_int as u16_0, posA,
                                 posB, posResult, outPoly, bgId, actor,
                                 1.0f32,
                                 BgCheck_GetBccFlags(chkWall, chkFloor,
                                                     chkCeil, chkOneFace,
                                                     1 as libc::c_int));
}
/* *
 * Public. Tests if a line from `posA` to `posB` intersects with a poly
 * returns true if it does, else false
 */
#[no_mangle]
pub unsafe extern "C" fn BgCheck_EntityLineTest3(mut colCtx:
                                                     *mut CollisionContext,
                                                 mut posA: *mut Vec3f,
                                                 mut posB: *mut Vec3f,
                                                 mut posResult: *mut Vec3f,
                                                 mut outPoly:
                                                     *mut *mut CollisionPoly,
                                                 mut chkWall: s32,
                                                 mut chkFloor: s32,
                                                 mut chkCeil: s32,
                                                 mut chkOneFace: s32,
                                                 mut bgId: *mut s32,
                                                 mut actor: *mut Actor,
                                                 mut chkDist: f32_0) -> s32 {
    return BgCheck_CheckLineImpl(colCtx,
                                 ((1 as libc::c_int) << 1 as libc::c_int) as
                                     u16_0, 0 as libc::c_int as u16_0, posA,
                                 posB, posResult, outPoly, bgId, actor,
                                 chkDist,
                                 BgCheck_GetBccFlags(chkWall, chkFloor,
                                                     chkCeil, chkOneFace,
                                                     1 as libc::c_int));
}
/* *
 * Public. Tests if a line from `posA` to `posB` intersects with a poly
 * returns true if it does, else false
 */
#[no_mangle]
pub unsafe extern "C" fn BgCheck_ProjectileLineTest(mut colCtx:
                                                        *mut CollisionContext,
                                                    mut posA: *mut Vec3f,
                                                    mut posB: *mut Vec3f,
                                                    mut posResult: *mut Vec3f,
                                                    mut outPoly:
                                                        *mut *mut CollisionPoly,
                                                    mut chkWall: s32,
                                                    mut chkFloor: s32,
                                                    mut chkCeil: s32,
                                                    mut chkOneFace: s32,
                                                    mut bgId: *mut s32)
 -> s32 {
    return BgCheck_CheckLineImpl(colCtx,
                                 ((1 as libc::c_int) << 2 as libc::c_int) as
                                     u16_0, 0 as libc::c_int as u16_0, posA,
                                 posB, posResult, outPoly, bgId,
                                 0 as *mut Actor, 1.0f32,
                                 BgCheck_GetBccFlags(chkWall, chkFloor,
                                                     chkCeil, chkOneFace,
                                                     1 as libc::c_int));
}
/* *
 * Public. Tests if a line from `posA` to `posB` intersects with a poly
 * returns true if it does, else false
 */
#[no_mangle]
pub unsafe extern "C" fn BgCheck_AnyLineTest1(mut colCtx:
                                                  *mut CollisionContext,
                                              mut posA: *mut Vec3f,
                                              mut posB: *mut Vec3f,
                                              mut posResult: *mut Vec3f,
                                              mut outPoly:
                                                  *mut *mut CollisionPoly,
                                              mut chkOneFace: s32) -> s32 {
    return BgCheck_AnyLineTest2(colCtx, posA, posB, posResult, outPoly,
                                1 as libc::c_int, 1 as libc::c_int,
                                1 as libc::c_int, chkOneFace);
}
/* *
 * Public. Tests if a line from `posA` to `posB` intersects with a poly
 * returns true if it does, else false
 */
#[no_mangle]
pub unsafe extern "C" fn BgCheck_AnyLineTest2(mut colCtx:
                                                  *mut CollisionContext,
                                              mut posA: *mut Vec3f,
                                              mut posB: *mut Vec3f,
                                              mut posResult: *mut Vec3f,
                                              mut outPoly:
                                                  *mut *mut CollisionPoly,
                                              mut chkWall: s32,
                                              mut chkFloor: s32,
                                              mut chkCeil: s32,
                                              mut chkOneFace: s32) -> s32 {
    let mut bgId: s32 = 0;
    return BgCheck_CheckLineImpl(colCtx, 0 as libc::c_int as u16_0,
                                 0 as libc::c_int as u16_0, posA, posB,
                                 posResult, outPoly, &mut bgId,
                                 0 as *mut Actor, 1.0f32,
                                 BgCheck_GetBccFlags(chkWall, chkFloor,
                                                     chkCeil, chkOneFace,
                                                     1 as libc::c_int));
}
/* *
 * Public. Tests if a line from `posA` to `posB` intersects with a poly
 * returns true if it does, else false
 */
#[no_mangle]
pub unsafe extern "C" fn BgCheck_AnyLineTest3(mut colCtx:
                                                  *mut CollisionContext,
                                              mut posA: *mut Vec3f,
                                              mut posB: *mut Vec3f,
                                              mut posResult: *mut Vec3f,
                                              mut outPoly:
                                                  *mut *mut CollisionPoly,
                                              mut chkWall: s32,
                                              mut chkFloor: s32,
                                              mut chkCeil: s32,
                                              mut chkOneFace: s32,
                                              mut bgId: *mut s32) -> s32 {
    return BgCheck_CheckLineImpl(colCtx, 0 as libc::c_int as u16_0,
                                 0 as libc::c_int as u16_0, posA, posB,
                                 posResult, outPoly, bgId, 0 as *mut Actor,
                                 1.0f32,
                                 BgCheck_GetBccFlags(chkWall, chkFloor,
                                                     chkCeil, chkOneFace,
                                                     1 as libc::c_int));
}
/* *
 * Get first poly intersecting sphere `center` `radius`
 * ignores `actor` dyna poly
 * returns true if any poly intersects the sphere, else false
 * `outPoly` returns the pointer of the first poly found that intersects
 * `outBgId` returns the bgId of the entity that owns `outPoly`
 */
#[no_mangle]
pub unsafe extern "C" fn BgCheck_SphVsFirstPolyImpl(mut colCtx:
                                                        *mut CollisionContext,
                                                    mut xpFlags: u16_0,
                                                    mut outPoly:
                                                        *mut *mut CollisionPoly,
                                                    mut outBgId: *mut s32,
                                                    mut center: *mut Vec3f,
                                                    mut radius: f32_0,
                                                    mut actor: *mut Actor,
                                                    mut bciFlags: u16_0)
 -> s32 {
    let mut lookup: *mut StaticLookup = 0 as *mut StaticLookup;
    *outBgId = 50 as libc::c_int;
    if BgCheck_PosErrorCheck(center,
                             b"../z_bgcheck.c\x00" as *const u8 as
                                 *const libc::c_char as *mut libc::c_char,
                             5852 as libc::c_int) == 1 as libc::c_int {
        if !actor.is_null() {
            osSyncPrintf(b"\xe3\x81\x93\xe3\x81\x84\xe3\x81\xa4,pself_actor->name %d\n\x00"
                             as *const u8 as *const libc::c_char,
                         (*actor).id as libc::c_int);
        }
    }
    lookup = BgCheck_GetStaticLookup(colCtx, (*colCtx).lookupTbl, center);
    if lookup.is_null() {
        return 0 as libc::c_int
    } else {
        if BgCheck_SphVsFirstStaticPoly(lookup, xpFlags, colCtx, center,
                                        radius, outPoly, bciFlags) != 0 ||
               BgCheck_SphVsFirstDynaPoly(colCtx, xpFlags, outPoly, outBgId,
                                          center, radius, actor, bciFlags) !=
                   0 {
            return 1 as libc::c_int
        }
    }
    return 0 as libc::c_int;
}
/* *
 * Public get first poly intersecting sphere `center` `radius`
 */
#[no_mangle]
pub unsafe extern "C" fn BgCheck_SphVsFirstPoly(mut colCtx:
                                                    *mut CollisionContext,
                                                mut center: *mut Vec3f,
                                                mut radius: f32_0) -> s32 {
    let mut poly: *mut CollisionPoly = 0 as *mut CollisionPoly;
    let mut bgId: s32 = 0;
    return BgCheck_SphVsFirstPolyImpl(colCtx, 0 as libc::c_int as u16_0,
                                      &mut poly, &mut bgId, center, radius,
                                      0 as *mut Actor,
                                      0 as libc::c_int as u16_0);
}
/* *
 * Public get first wall poly intersecting sphere `center` `radius`
 */
#[no_mangle]
pub unsafe extern "C" fn BgCheck_SphVsFirstWall(mut colCtx:
                                                    *mut CollisionContext,
                                                mut center: *mut Vec3f,
                                                mut radius: f32_0) -> s32 {
    let mut poly: *mut CollisionPoly = 0 as *mut CollisionPoly;
    let mut bgId: s32 = 0;
    return BgCheck_SphVsFirstPolyImpl(colCtx, 0 as libc::c_int as u16_0,
                                      &mut poly, &mut bgId, center, radius,
                                      0 as *mut Actor,
                                      ((1 as libc::c_int) << 2 as libc::c_int
                                           |
                                           (1 as libc::c_int) <<
                                               0 as libc::c_int) as u16_0);
}
/* *
 * Init SSNodeList
 */
#[no_mangle]
pub unsafe extern "C" fn SSNodeList_Initialize(mut this: *mut SSNodeList) {
    (*this).max = 0 as libc::c_int as u16_0;
    (*this).count = 0 as libc::c_int as u16_0;
    (*this).tbl = 0 as *mut SSNode;
    (*this).polyCheckTbl = 0 as *mut u8_0;
}
/* *
 * Allocate SSNodeList
 * tblMax is the number of SSNode records to allocate
 * numPolys is the number of polygons defined within the CollisionHeader
 */
#[no_mangle]
pub unsafe extern "C" fn SSNodeList_Alloc(mut globalCtx: *mut GlobalContext,
                                          mut this: *mut SSNodeList,
                                          mut tblMax: s32,
                                          mut numPolys: s32) {
    (*this).max = tblMax as u16_0;
    (*this).count = 0 as libc::c_int as u16_0;
    (*this).tbl =
        THA_AllocEndAlign(&mut (*globalCtx).state.tha,
                          (tblMax as
                               libc::c_uint).wrapping_mul(::std::mem::size_of::<SSNode>()
                                                              as
                                                              libc::c_ulong),
                          -(2 as libc::c_int) as u32_0) as *mut SSNode;
    if !(*this).tbl.is_null() {
    } else {
        __assert(b"this->short_slist_node_tbl != NULL\x00" as *const u8 as
                     *const libc::c_char,
                 b"../z_bgcheck.c\x00" as *const u8 as *const libc::c_char,
                 5975 as libc::c_int);
    };
    (*this).polyCheckTbl =
        GameState_Alloc(&mut (*globalCtx).state, numPolys as size_t,
                        b"../z_bgcheck.c\x00" as *const u8 as
                            *const libc::c_char as *mut libc::c_char,
                        5979 as libc::c_int) as *mut u8_0;
    if !(*this).polyCheckTbl.is_null() {
    } else {
        __assert(b"this->polygon_check != NULL\x00" as *const u8 as
                     *const libc::c_char,
                 b"../z_bgcheck.c\x00" as *const u8 as *const libc::c_char,
                 5981 as libc::c_int);
    };
}
/* *
 * Get next SSNodeList SSNode
 */
#[no_mangle]
pub unsafe extern "C" fn SSNodeList_GetNextNode(mut this: *mut SSNodeList)
 -> *mut SSNode {
    let mut result: *mut SSNode =
        &mut *(*this).tbl.offset((*this).count as isize) as *mut SSNode;
    (*this).count = (*this).count.wrapping_add(1);
    if ((*this).count as libc::c_int) < (*this).max as libc::c_int {
    } else {
        __assert(b"this->short_slist_node_last_index < this->short_slist_node_size\x00"
                     as *const u8 as *const libc::c_char,
                 b"../z_bgcheck.c\x00" as *const u8 as *const libc::c_char,
                 5998 as libc::c_int);
    };
    if !(((*this).count as libc::c_int) < (*this).max as libc::c_int) {
        return 0 as *mut SSNode
    }
    return result;
}
/* *
 * Get next SSNodeList SSNode index
 */
#[no_mangle]
pub unsafe extern "C" fn SSNodeList_GetNextNodeIdx(mut this: *mut SSNodeList)
 -> u16_0 {
    let fresh7 = (*this).count;
    (*this).count = (*this).count.wrapping_add(1);
    let mut new_index: u16_0 = fresh7;
    if (new_index as libc::c_int) < (*this).max as libc::c_int {
    } else {
        __assert(b"new_index < this->short_slist_node_size\x00" as *const u8
                     as *const libc::c_char,
                 b"../z_bgcheck.c\x00" as *const u8 as *const libc::c_char,
                 6021 as libc::c_int);
    };
    return new_index;
}
/* *
 * Initialize ScaleRotPos
 */
#[no_mangle]
pub unsafe extern "C" fn ScaleRotPos_Initialize(mut srp: *mut ScaleRotPos) {
    (*srp).scale.z = 1.0f32;
    (*srp).scale.y = (*srp).scale.z;
    (*srp).scale.x = (*srp).scale.y;
    (*srp).pos.z = 0.0f32;
    (*srp).pos.y = (*srp).pos.z;
    (*srp).pos.x = (*srp).pos.y;
    (*srp).rot.z = 0 as libc::c_int as s16;
    (*srp).rot.y = (*srp).rot.z;
    (*srp).rot.x = (*srp).rot.y;
}
/* *
 * Set ScaleRotPos
 */
#[no_mangle]
pub unsafe extern "C" fn ScaleRotPos_SetValue(mut srp: *mut ScaleRotPos,
                                              mut scale: *mut Vec3f,
                                              mut rot: *mut Vec3s,
                                              mut pos: *mut Vec3f) {
    (*srp).scale = *scale;
    (*srp).rot = *rot;
    (*srp).pos = *pos;
}
/* *
 * ScaleRotPos equality test
 */
#[no_mangle]
pub unsafe extern "C" fn ScaleRotPos_Equals(mut a: *mut ScaleRotPos,
                                            mut b: *mut ScaleRotPos) -> s32 {
    if (*a).scale.x != (*b).scale.x || (*a).scale.y != (*b).scale.y ||
           (*a).scale.z != (*b).scale.z ||
           (*a).rot.x as libc::c_int != (*b).rot.x as libc::c_int ||
           (*a).rot.y as libc::c_int != (*b).rot.y as libc::c_int ||
           (*a).rot.z as libc::c_int != (*b).rot.z as libc::c_int ||
           (*a).pos.x != (*b).pos.x || (*a).pos.y != (*b).pos.y ||
           (*a).pos.z != (*b).pos.z {
        return 0 as libc::c_int
    }
    return 1 as libc::c_int;
}
/* *
 * Reset DynaLookup lists
 */
#[no_mangle]
pub unsafe extern "C" fn DynaLookup_ResetLists(mut dynaLookup:
                                                   *mut DynaLookup) {
    SSList_SetNull(&mut (*dynaLookup).ceiling);
    SSList_SetNull(&mut (*dynaLookup).wall);
    SSList_SetNull(&mut (*dynaLookup).floor);
}
/* *
 * Reset DynaLookup
 */
#[no_mangle]
pub unsafe extern "C" fn DynaLookup_Reset(mut dynaLookup: *mut DynaLookup) {
    (*dynaLookup).polyStartIndex = 0 as libc::c_int as u16_0;
    DynaLookup_ResetLists(dynaLookup);
}
/* *
 * Reset vtxStartIndex
 */
#[no_mangle]
pub unsafe extern "C" fn DynaLookup_ResetVtxStartIndex(mut vtxStartIndex:
                                                           *mut u16_0) {
    *vtxStartIndex = 0 as libc::c_int as u16_0;
}
/* *
 * Initialize BgActor
 */
#[no_mangle]
pub unsafe extern "C" fn BgActor_Initialize(mut globalCtx: *mut GlobalContext,
                                            mut bgActor: *mut BgActor) {
    (*bgActor).actor = 0 as *mut Actor;
    (*bgActor).colHeader = 0 as *mut CollisionHeader;
    ScaleRotPos_Initialize(&mut (*bgActor).prevTransform);
    ScaleRotPos_Initialize(&mut (*bgActor).curTransform);
    DynaLookup_Reset(&mut (*bgActor).dynaLookup);
    DynaLookup_ResetVtxStartIndex(&mut (*bgActor).vtxStartIndex);
    (*bgActor).boundingSphere.center.z = 0 as libc::c_int as s16;
    (*bgActor).boundingSphere.center.y = (*bgActor).boundingSphere.center.z;
    (*bgActor).boundingSphere.center.x = (*bgActor).boundingSphere.center.y;
    (*bgActor).boundingSphere.radius = 0 as libc::c_int as s16;
}
/* *
 * setActor internal
 */
#[no_mangle]
pub unsafe extern "C" fn BgActor_SetActor(mut bgActor: *mut BgActor,
                                          mut actor: *mut Actor,
                                          mut colHeader:
                                              *mut CollisionHeader) {
    (*bgActor).actor = actor;
    (*bgActor).colHeader = colHeader;
    (*bgActor).prevTransform.scale = (*actor).scale;
    (*bgActor).prevTransform.rot = (*actor).shape.rot;
    (*bgActor).prevTransform.rot.x -= 1;
    (*bgActor).prevTransform.pos = (*actor).world.pos;
    (*bgActor).curTransform.scale = (*actor).scale;
    (*bgActor).curTransform.rot = (*actor).shape.rot;
    (*bgActor).curTransform.pos = (*actor).world.pos;
}
/* *
 * Test if the BgActor transform is the same
 */
#[no_mangle]
pub unsafe extern "C" fn BgActor_IsTransformUnchanged(mut bgActor:
                                                          *mut BgActor)
 -> s32 {
    return ScaleRotPos_Equals(&mut (*bgActor).prevTransform,
                              &mut (*bgActor).curTransform);
}
/* *
 * NULL polyList
 */
#[no_mangle]
pub unsafe extern "C" fn DynaPoly_NullPolyList(mut polyList:
                                                   *mut *mut CollisionPoly) {
    *polyList = 0 as *mut CollisionPoly;
}
/* *
 * Allocate dyna.polyList
 */
#[no_mangle]
pub unsafe extern "C" fn DynaPoly_AllocPolyList(mut globalCtx:
                                                    *mut GlobalContext,
                                                mut polyList:
                                                    *mut *mut CollisionPoly,
                                                mut numPolys: s32) {
    *polyList =
        THA_AllocEndAlign(&mut (*globalCtx).state.tha,
                          (numPolys as
                               libc::c_uint).wrapping_mul(::std::mem::size_of::<CollisionPoly>()
                                                              as
                                                              libc::c_ulong),
                          -(2 as libc::c_int) as u32_0) as *mut CollisionPoly;
    if !(*polyList).is_null() {
    } else {
        __assert(b"ptbl->pbuf != NULL\x00" as *const u8 as
                     *const libc::c_char,
                 b"../z_bgcheck.c\x00" as *const u8 as *const libc::c_char,
                 6247 as libc::c_int);
    };
}
/* *
 * NULL vtxList
 */
#[no_mangle]
pub unsafe extern "C" fn DynaPoly_NullVtxList(mut vtxList: *mut *mut Vec3s) {
    *vtxList = 0 as *mut Vec3s;
}
/* *
 * Allocate dyna.vtxList
 */
#[no_mangle]
pub unsafe extern "C" fn DynaPoly_AllocVtxList(mut globalCtx:
                                                   *mut GlobalContext,
                                               mut vtxList: *mut *mut Vec3s,
                                               mut numVtx: s32) {
    *vtxList =
        THA_AllocEndAlign(&mut (*globalCtx).state.tha,
                          (numVtx as
                               libc::c_uint).wrapping_mul(::std::mem::size_of::<Vec3s>()
                                                              as
                                                              libc::c_ulong),
                          -(2 as libc::c_int) as u32_0) as *mut Vec3s;
    if !(*vtxList).is_null() {
    } else {
        __assert(b"ptbl->pbuf != NULL\x00" as *const u8 as
                     *const libc::c_char,
                 b"../z_bgcheck.c\x00" as *const u8 as *const libc::c_char,
                 6277 as libc::c_int);
    };
}
/* *
 * Update BgActor's prevTransform
 */
#[no_mangle]
pub unsafe extern "C" fn DynaPoly_SetBgActorPrevTransform(mut globalCtx:
                                                              *mut GlobalContext,
                                                          mut bgActor:
                                                              *mut BgActor) {
    (*bgActor).prevTransform = (*bgActor).curTransform;
}
/* *
 * Is BgActor Id
 */
#[no_mangle]
pub unsafe extern "C" fn DynaPoly_IsBgIdBgActor(mut bgId: s32) -> s32 {
    if bgId < 0 as libc::c_int || bgId >= 50 as libc::c_int {
        return 0 as libc::c_int
    }
    return 1 as libc::c_int;
}
/* *
 * Init DynaCollisionContext
 */
#[no_mangle]
pub unsafe extern "C" fn DynaPoly_Init(mut globalCtx: *mut GlobalContext,
                                       mut dyna: *mut DynaCollisionContext) {
    (*dyna).bitFlag = ((1 as libc::c_int) << 0 as libc::c_int) as u8_0;
    DynaPoly_NullPolyList(&mut (*dyna).polyList);
    DynaPoly_NullVtxList(&mut (*dyna).vtxList);
    DynaSSNodeList_Initialize(globalCtx, &mut (*dyna).polyNodes);
}
/* *
 * Set DynaCollisionContext
 */
#[no_mangle]
pub unsafe extern "C" fn DynaPoly_Alloc(mut globalCtx: *mut GlobalContext,
                                        mut dyna: *mut DynaCollisionContext) {
    let mut i: s32 = 0;
    i = 0 as libc::c_int;
    while i < 50 as libc::c_int {
        BgActor_Initialize(globalCtx,
                           &mut *(*dyna).bgActors.as_mut_ptr().offset(i as
                                                                          isize));
        (*dyna).bgActorFlags[i as usize] = 0 as libc::c_int as u16_0;
        i += 1
    }
    DynaPoly_NullPolyList(&mut (*dyna).polyList);
    DynaPoly_AllocPolyList(globalCtx, &mut (*dyna).polyList,
                           (*dyna).polyListMax);
    DynaPoly_NullVtxList(&mut (*dyna).vtxList);
    DynaPoly_AllocVtxList(globalCtx, &mut (*dyna).vtxList,
                          (*dyna).vtxListMax);
    DynaSSNodeList_Initialize(globalCtx, &mut (*dyna).polyNodes);
    DynaSSNodeList_Alloc(globalCtx, &mut (*dyna).polyNodes,
                         (*dyna).polyNodesMax);
}
/* *
 * Set BgActor
 * original name: DynaPolyInfo_setActor
 */
#[no_mangle]
pub unsafe extern "C" fn DynaPoly_SetBgActor(mut globalCtx:
                                                 *mut GlobalContext,
                                             mut dyna:
                                                 *mut DynaCollisionContext,
                                             mut actor: *mut Actor,
                                             mut colHeader:
                                                 *mut CollisionHeader)
 -> s32 {
    let mut bgId: s32 = 0;
    let mut foundSlot: s32 = 0 as libc::c_int;
    bgId = 0 as libc::c_int;
    while bgId < 50 as libc::c_int {
        if (*dyna).bgActorFlags[bgId as usize] as libc::c_int &
               1 as libc::c_int == 0 {
            (*dyna).bgActorFlags[bgId as usize] =
                ((*dyna).bgActorFlags[bgId as usize] as libc::c_int |
                     1 as libc::c_int) as u16_0;
            foundSlot = 1 as libc::c_int;
            break ;
        } else { bgId += 1 }
    }
    if foundSlot == 0 as libc::c_int {
        osSyncPrintf(b"\x1b[31m\x00" as *const u8 as *const libc::c_char);
        osSyncPrintf(b"DynaPolyInfo_setActor():\xe3\x83\x80\xe3\x82\xa4\xe3\x83\x8a\xe3\x83\x9f\xe3\x83\x83\xe3\x82\xaf\xe3\x83\x9d\xe3\x83\xaa\xe3\x82\xb4\xe3\x83\xb3 \xe7\xa9\xba\xe3\x81\x8d\xe3\x82\xa4\xe3\x83\xb3\xe3\x83\x87\xe3\x83\x83\xe3\x82\xaf\xe3\x82\xb9\xe3\x81\xaf\xe3\x81\x82\xe3\x82\x8a\xe3\x81\xbe\xe3\x81\x9b\xe3\x82\x93\n\x00"
                         as *const u8 as *const libc::c_char);
        osSyncPrintf(b"\x1b[m\x00" as *const u8 as *const libc::c_char);
        return 50 as libc::c_int
    }
    BgActor_SetActor(&mut *(*dyna).bgActors.as_mut_ptr().offset(bgId as
                                                                    isize),
                     actor, colHeader);
    (*dyna).bitFlag =
        ((*dyna).bitFlag as libc::c_int |
             (1 as libc::c_int) << 0 as libc::c_int) as u8_0;
    (*dyna).bgActorFlags[bgId as usize] =
        ((*dyna).bgActorFlags[bgId as usize] as libc::c_int &
             !(2 as libc::c_int)) as u16_0;
    osSyncPrintf(b"\x1b[32m\x00" as *const u8 as *const libc::c_char);
    osSyncPrintf(b"DynaPolyInfo_setActor():index %d\n\x00" as *const u8 as
                     *const libc::c_char, bgId);
    osSyncPrintf(b"\x1b[m\x00" as *const u8 as *const libc::c_char);
    return bgId;
}
/* *
 * Gets the actor assigned to `bgId`
 * possible orginal name: DynaPolyInfo_getActor
 */
#[no_mangle]
pub unsafe extern "C" fn DynaPoly_GetActor(mut colCtx: *mut CollisionContext,
                                           mut bgId: s32)
 -> *mut DynaPolyActor {
    if DynaPoly_IsBgIdBgActor(bgId) == 0 ||
           (*colCtx).dyna.bgActorFlags[bgId as usize] as libc::c_int &
               1 as libc::c_int == 0 ||
           (*colCtx).dyna.bgActorFlags[bgId as usize] as libc::c_int &
               2 as libc::c_int != 0 {
        return 0 as *mut DynaPolyActor
    }
    return (*colCtx).dyna.bgActors[bgId as usize].actor as *mut DynaPolyActor;
}
#[no_mangle]
pub unsafe extern "C" fn func_8003EBF8(mut globalCtx: *mut GlobalContext,
                                       mut dyna: *mut DynaCollisionContext,
                                       mut bgId: s32) {
    if DynaPoly_IsBgIdBgActor(bgId) != 0 {
        (*dyna).bgActorFlags[bgId as usize] =
            ((*dyna).bgActorFlags[bgId as usize] as libc::c_int |
                 4 as libc::c_int) as u16_0;
        (*dyna).bitFlag =
            ((*dyna).bitFlag as libc::c_int |
                 (1 as libc::c_int) << 0 as libc::c_int) as u8_0
    };
}
#[no_mangle]
pub unsafe extern "C" fn func_8003EC50(mut globalCtx: *mut GlobalContext,
                                       mut dyna: *mut DynaCollisionContext,
                                       mut bgId: s32) {
    if DynaPoly_IsBgIdBgActor(bgId) != 0 {
        (*dyna).bgActorFlags[bgId as usize] =
            ((*dyna).bgActorFlags[bgId as usize] as libc::c_int &
                 !(4 as libc::c_int)) as u16_0;
        (*dyna).bitFlag =
            ((*dyna).bitFlag as libc::c_int |
                 (1 as libc::c_int) << 0 as libc::c_int) as u8_0
    };
}
#[no_mangle]
pub unsafe extern "C" fn func_8003ECA8(mut globalCtx: *mut GlobalContext,
                                       mut dyna: *mut DynaCollisionContext,
                                       mut bgId: s32) {
    if DynaPoly_IsBgIdBgActor(bgId) != 0 {
        (*dyna).bgActorFlags[bgId as usize] =
            ((*dyna).bgActorFlags[bgId as usize] as libc::c_int |
                 8 as libc::c_int) as u16_0;
        (*dyna).bitFlag =
            ((*dyna).bitFlag as libc::c_int |
                 (1 as libc::c_int) << 0 as libc::c_int) as u8_0
    };
}
#[no_mangle]
pub unsafe extern "C" fn func_8003ED00(mut globalCtx: *mut GlobalContext,
                                       mut dyna: *mut DynaCollisionContext,
                                       mut bgId: s32) {
    if DynaPoly_IsBgIdBgActor(bgId) != 0 {
        (*dyna).bgActorFlags[bgId as usize] =
            ((*dyna).bgActorFlags[bgId as usize] as libc::c_int &
                 !(8 as libc::c_int)) as u16_0;
        (*dyna).bitFlag =
            ((*dyna).bitFlag as libc::c_int |
                 (1 as libc::c_int) << 0 as libc::c_int) as u8_0
    };
}
/* *
 * original name: DynaPolyInfo_delReserve
 */
#[no_mangle]
pub unsafe extern "C" fn DynaPoly_DeleteBgActor(mut globalCtx:
                                                    *mut GlobalContext,
                                                mut dyna:
                                                    *mut DynaCollisionContext,
                                                mut bgId: s32) {
    let mut actor: *mut DynaPolyActor = 0 as *mut DynaPolyActor;
    osSyncPrintf(b"\x1b[32m\x00" as *const u8 as *const libc::c_char);
    osSyncPrintf(b"DynaPolyInfo_delReserve():index %d\n\x00" as *const u8 as
                     *const libc::c_char, bgId);
    osSyncPrintf(b"\x1b[m\x00" as *const u8 as *const libc::c_char);
    if DynaPoly_IsBgIdBgActor(bgId) == 0 as libc::c_int {
        if bgId == -(1 as libc::c_int) {
            osSyncPrintf(b"\x1b[32m\x00" as *const u8 as *const libc::c_char);
            // "The index that should have been deleted(? ) was(== -1), processing aborted."
            osSyncPrintf(b"DynaPolyInfo_delReserve():\xe5\x89\x8a\xe9\x99\xa4\xe3\x81\x95\xe3\x82\x8c\xe3\x81\xa6\xe3\x81\x84\xe3\x82\x8b\xe3\x81\xaf\xe3\x81\x9a\xe3\x81\xae(?)\n\xe3\x82\xa4\xe3\x83\xb3\xe3\x83\x87\xe3\x83\x83\xe3\x82\xaf\xe3\x82\xb9(== -1)\xe3\x81\xae\xe3\x81\x9f\xe3\x82\x81,\xe5\x87\xa6\xe7\x90\x86\xe3\x82\x92\xe4\xb8\xad\xe6\xad\xa2\xe3\x81\x97\xe3\x81\xbe\xe3\x81\x99\xe3\x80\x82\n\x00"
                             as *const u8 as *const libc::c_char);
            osSyncPrintf(b"\x1b[m\x00" as *const u8 as *const libc::c_char);
            return
        } else {
            osSyncPrintf(b"\x1b[31m\x00" as *const u8 as *const libc::c_char);
            // "Unable to deallocate index / index unallocated, processing aborted."
            osSyncPrintf(b"DynaPolyInfo_delReserve():\xe7\xa2\xba\xe4\xbf\x9d\xe3\x81\x97\xe3\x81\xa6\xe3\x81\x84\xe3\x81\xaa\xe3\x81\x84\xef\xbc\x8f\xe5\x87\xba\xe6\x9d\xa5\xe3\x81\xaa\xe3\x81\x8b\xe3\x81\xa3\xe3\x81\x9f\xe3\x82\xa4\xe3\x83\xb3\xe3\x83\x87\xe3\x83\x83\xe3\x82\xaf\xe3\x82\xb9\xe3\x81\xae\xe8\xa7\xa3\xe6\x94\xbe\xe3\x81\xae\xe3\x81\x9f\xe3\x82\x81\xe3\x80\x81\xe5\x87\xa6\xe7\x90\x86\xe3\x82\x92\xe4\xb8\xad\xe6\xad\xa2\xe3\x81\x97\xe3\x81\xbe\xe3\x81\x99\xe3\x80\x82index == %d\n\x00"
                             as *const u8 as *const libc::c_char, bgId);
            osSyncPrintf(b"\x1b[m\x00" as *const u8 as *const libc::c_char);
            return
        }
    }
    actor = DynaPoly_GetActor(&mut (*globalCtx).colCtx, bgId);
    if !actor.is_null() {
        (*actor).bgId = -(1 as libc::c_int);
        (*dyna).bgActors[bgId as usize].actor = 0 as *mut Actor;
        (*dyna).bgActorFlags[bgId as usize] =
            ((*dyna).bgActorFlags[bgId as usize] as libc::c_int |
                 2 as libc::c_int) as u16_0
    };
}
#[no_mangle]
pub unsafe extern "C" fn func_8003EE6C(mut globalCtx: *mut GlobalContext,
                                       mut dyna: *mut DynaCollisionContext) {
    (*dyna).bitFlag =
        ((*dyna).bitFlag as libc::c_int |
             (1 as libc::c_int) << 0 as libc::c_int) as u8_0;
}
/* *
 * original name: DynaPolyInfo_expandSRT
 */
#[no_mangle]
pub unsafe extern "C" fn DynaPoly_ExpandSRT(mut globalCtx: *mut GlobalContext,
                                            mut dyna:
                                                *mut DynaCollisionContext,
                                            mut bgId: s32,
                                            mut vtxStartIndex: *mut s32,
                                            mut polyStartIndex: *mut s32) {
    let mut mtx: MtxF = MtxF{mf: [[0.; 4]; 4],};
    let mut actor: *mut Actor = 0 as *mut Actor;
    let mut pad: s32 = 0;
    let mut pad2: s32 = 0;
    let mut numVtxInverse: f32_0 = 0.;
    let mut i: s32 = 0;
    let mut pos: Vec3f = Vec3f{x: 0., y: 0., z: 0.,};
    let mut sphere: *mut Sphere16 = 0 as *mut Sphere16;
    let mut dVtxList: *mut Vec3s = 0 as *mut Vec3s;
    let mut point: *mut Vec3s = 0 as *mut Vec3s;
    let mut newCenterPoint: Vec3f = Vec3f{x: 0., y: 0., z: 0.,};
    let mut newRadiusSq: f32_0 = 0.;
    let mut pbgdata: *mut CollisionHeader = 0 as *mut CollisionHeader;
    let mut newVtx: Vec3f = Vec3f{x: 0., y: 0., z: 0.,};
    let mut vtxA: Vec3f = Vec3f{x: 0., y: 0., z: 0.,};
    let mut vtxB: Vec3f = Vec3f{x: 0., y: 0., z: 0.,};
    let mut vtxC: Vec3f = Vec3f{x: 0., y: 0., z: 0.,};
    let mut newNormal: Vec3f = Vec3f{x: 0., y: 0., z: 0.,};
    pbgdata = (*dyna).bgActors[bgId as usize].colHeader;
    sphere =
        &mut (*(*dyna).bgActors.as_mut_ptr().offset(bgId as
                                                        isize)).boundingSphere;
    actor = (*dyna).bgActors[bgId as usize].actor;
    (*dyna).bgActors[bgId as usize].dynaLookup.polyStartIndex =
        *polyStartIndex as u16_0;
    (*dyna).bgActors[bgId as usize].vtxStartIndex = *vtxStartIndex as u16_0;
    pos = (*actor).world.pos;
    pos.y += (*actor).shape.yOffset * (*actor).scale.y;
    ScaleRotPos_SetValue(&mut (*(*dyna).bgActors.as_mut_ptr().offset(bgId as
                                                                         isize)).curTransform,
                         &mut (*actor).scale, &mut (*actor).shape.rot,
                         &mut pos);
    if (*dyna).bgActorFlags[bgId as usize] as libc::c_int & 4 as libc::c_int
           != 0 {
        return
    }
    if !((*dyna).polyListMax >=
             *polyStartIndex + (*pbgdata).numPolygons as libc::c_int) {
        osSyncPrintf(b"\x1b[31m\x00" as *const u8 as *const libc::c_char);
        // "do not use if %d exceeds %d"
        osSyncPrintf(b"DynaPolyInfo_expandSRT():polygon over %d\xe3\x81\x8c%d\xe3\x82\x92\xe8\xb6\x8a\xe3\x81\x88\xe3\x82\x8b\xe3\x81\xa8\xe3\x83\x80\xe3\x83\xa1\n\x00"
                         as *const u8 as *const libc::c_char,
                     *polyStartIndex + (*pbgdata).numPolygons as libc::c_int,
                     (*dyna).polyListMax);
    }
    if !((*dyna).vtxListMax >=
             *vtxStartIndex + (*pbgdata).numVertices as libc::c_int) {
        osSyncPrintf(b"\x1b[31m\x00" as *const u8 as *const libc::c_char);
        // "do not use if %d exceeds %d"
        osSyncPrintf(b"DynaPolyInfo_expandSRT():vertex over %d\xe3\x81\x8c%d\xe3\x82\x92\xe8\xb6\x8a\xe3\x81\x88\xe3\x82\x8b\xe3\x81\xa8\xe3\x83\x80\xe3\x83\xa1\n\x00"
                         as *const u8 as *const libc::c_char,
                     *vtxStartIndex + (*pbgdata).numVertices as libc::c_int,
                     (*dyna).vtxListMax); // Vtx after mtx transform
    }
    if (*dyna).polyListMax >=
           *polyStartIndex + (*pbgdata).numPolygons as libc::c_int {
    } else {
        __assert(b"pdyna_poly_info->poly_num >= *pstart_poly_index + pbgdata->poly_num\x00"
                     as *const u8 as *const libc::c_char,
                 b"../z_bgcheck.c\x00" as *const u8 as *const libc::c_char,
                 6687 as libc::c_int);
    };
    if (*dyna).vtxListMax >=
           *vtxStartIndex + (*pbgdata).numVertices as libc::c_int {
    } else {
        __assert(b"pdyna_poly_info->vert_num >= *pstart_vert_index + pbgdata->vtx_num\x00"
                     as *const u8 as *const libc::c_char,
                 b"../z_bgcheck.c\x00" as *const u8 as *const libc::c_char,
                 6688 as libc::c_int);
    };
    if (*dyna).bitFlag as libc::c_int & (1 as libc::c_int) << 0 as libc::c_int
           == 0 &&
           BgActor_IsTransformUnchanged(&mut *(*dyna).bgActors.as_mut_ptr().offset(bgId
                                                                                       as
                                                                                       isize))
               == 1 as libc::c_int {
        let mut pi: s32 = 0;
        pi = *polyStartIndex;
        while pi < *polyStartIndex + (*pbgdata).numPolygons as libc::c_int {
            let mut poly: *mut CollisionPoly =
                &mut *(*dyna).polyList.offset(pi as isize) as
                    *mut CollisionPoly;
            let mut normalY: s16 = (*poly).normal.y;
            if normalY as libc::c_int >
                   (0.5f32 * 32767.0f32) as s16 as libc::c_int {
                let mut polyIndex: s16 = pi as s16;
                DynaSSNodeList_SetSSListHead(&mut (*dyna).polyNodes,
                                             &mut (*(*dyna).bgActors.as_mut_ptr().offset(bgId
                                                                                             as
                                                                                             isize)).dynaLookup.floor,
                                             &mut polyIndex);
            } else if (normalY as libc::c_int) <
                          (-0.8f32 * 32767.0f32) as s16 as libc::c_int {
                if (*dyna).bgActorFlags[bgId as usize] as libc::c_int &
                       8 as libc::c_int == 0 {
                    let mut polyIndex_0: s16 = pi as s16;
                    DynaSSNodeList_SetSSListHead(&mut (*dyna).polyNodes,
                                                 &mut (*(*dyna).bgActors.as_mut_ptr().offset(bgId
                                                                                                 as
                                                                                                 isize)).dynaLookup.ceiling,
                                                 &mut polyIndex_0);
                }
            } else {
                let mut polyIndex_1: s16 = pi as s16;
                DynaSSNodeList_SetSSListHead(&mut (*dyna).polyNodes,
                                             &mut (*(*dyna).bgActors.as_mut_ptr().offset(bgId
                                                                                             as
                                                                                             isize)).dynaLookup.wall,
                                             &mut polyIndex_1);
            }
            pi += 1
        }
        *polyStartIndex += (*pbgdata).numPolygons as libc::c_int;
        *vtxStartIndex += (*pbgdata).numVertices as libc::c_int
    } else {
        SkinMatrix_SetTranslateRotateYXZScale(&mut mtx,
                                              (*dyna).bgActors[bgId as
                                                                   usize].curTransform.scale.x,
                                              (*dyna).bgActors[bgId as
                                                                   usize].curTransform.scale.y,
                                              (*dyna).bgActors[bgId as
                                                                   usize].curTransform.scale.z,
                                              (*dyna).bgActors[bgId as
                                                                   usize].curTransform.rot.x,
                                              (*dyna).bgActors[bgId as
                                                                   usize].curTransform.rot.y,
                                              (*dyna).bgActors[bgId as
                                                                   usize].curTransform.rot.z,
                                              (*dyna).bgActors[bgId as
                                                                   usize].curTransform.pos.x,
                                              (*dyna).bgActors[bgId as
                                                                   usize].curTransform.pos.y,
                                              (*dyna).bgActors[bgId as
                                                                   usize].curTransform.pos.z);
        numVtxInverse =
            1.0f32 / (*pbgdata).numVertices as libc::c_int as libc::c_float;
        newCenterPoint.z = 0.0f32;
        newCenterPoint.y = newCenterPoint.z;
        newCenterPoint.x = newCenterPoint.y;
        i = 0 as libc::c_int;
        while i < (*pbgdata).numVertices as libc::c_int {
            let mut vtx: Vec3f = Vec3f{x: 0., y: 0., z: 0.,};
            let mut vtxT: Vec3f = Vec3f{x: 0., y: 0., z: 0.,};
            Math_Vec3s_ToVec3f(&mut vtx,
                               &mut *(*pbgdata).vtxList.offset(i as isize));
            SkinMatrix_Vec3fMtxFMultXYZ(&mut mtx, &mut vtx, &mut vtxT);
            BgCheck_Vec3fToVec3s(&mut *(*dyna).vtxList.offset((*vtxStartIndex
                                                                   + i) as
                                                                  isize),
                                 &mut vtxT);
            if i == 0 as libc::c_int {
                (*dyna).bgActors[bgId as usize].maxY = vtxT.y;
                (*dyna).bgActors[bgId as usize].minY =
                    (*dyna).bgActors[bgId as usize].maxY
            } else if vtxT.y < (*dyna).bgActors[bgId as usize].minY {
                (*dyna).bgActors[bgId as usize].minY = vtxT.y
            } else if (*dyna).bgActors[bgId as usize].maxY < vtxT.y {
                (*dyna).bgActors[bgId as usize].maxY = vtxT.y
            }
            newCenterPoint.x += vtxT.x;
            newCenterPoint.y += vtxT.y;
            newCenterPoint.z += vtxT.z;
            i += 1
        }
        newCenterPoint.x *= numVtxInverse;
        newCenterPoint.y *= numVtxInverse;
        newCenterPoint.z *= numVtxInverse;
        (*sphere).center.x = newCenterPoint.x as s16;
        (*sphere).center.y = newCenterPoint.y as s16;
        (*sphere).center.z = newCenterPoint.z as s16;
        newRadiusSq = -100.0f32;
        i = 0 as libc::c_int;
        while i < (*pbgdata).numVertices as libc::c_int {
            let mut radiusSq: f32_0 = 0.;
            newVtx.x =
                (*(*dyna).vtxList.offset((*vtxStartIndex + i) as isize)).x as
                    f32_0;
            newVtx.y =
                (*(*dyna).vtxList.offset((*vtxStartIndex + i) as isize)).y as
                    f32_0;
            newVtx.z =
                (*(*dyna).vtxList.offset((*vtxStartIndex + i) as isize)).z as
                    f32_0;
            radiusSq = Math3D_Vec3fDistSq(&mut newVtx, &mut newCenterPoint);
            if newRadiusSq < radiusSq { newRadiusSq = radiusSq }
            i += 1
        }
        (*sphere).radius = (sqrtf(newRadiusSq) * 1.1f32) as s16;
        i = 0 as libc::c_int;
        while i < (*pbgdata).numPolygons as libc::c_int {
            let mut newPoly: *mut CollisionPoly =
                &mut *(*dyna).polyList.offset((*polyStartIndex + i) as isize)
                    as *mut CollisionPoly;
            let mut newNormMagnitude: f32_0 = 0.;
            *newPoly = *(*pbgdata).polyList.offset(i as isize);
            // Yeah, this is all kinds of fake, but my God, it matches.
            (*newPoly).c2rust_unnamed.c2rust_unnamed.flags_vIA =
                (((*newPoly).c2rust_unnamed.c2rust_unnamed.flags_vIA as
                      libc::c_int & 0x1fff as libc::c_int) + *vtxStartIndex |
                     (*newPoly).c2rust_unnamed.c2rust_unnamed.flags_vIA as
                         libc::c_int & 0xe000 as libc::c_int) as u16_0;
            (*newPoly).c2rust_unnamed.c2rust_unnamed.flags_vIB =
                (((*newPoly).c2rust_unnamed.c2rust_unnamed.flags_vIB as
                      libc::c_int & 0x1fff as libc::c_int) + *vtxStartIndex |
                     (*newPoly).c2rust_unnamed.c2rust_unnamed.flags_vIB as
                         libc::c_int & 0xe000 as libc::c_int) as u16_0;
            (*newPoly).c2rust_unnamed.c2rust_unnamed.vIC =
                (*vtxStartIndex +
                     (*newPoly).c2rust_unnamed.c2rust_unnamed.vIC as
                         libc::c_int) as u16_0;
            dVtxList = (*dyna).vtxList;
            vtxA.x =
                (*dVtxList.offset(((*newPoly).c2rust_unnamed.c2rust_unnamed.flags_vIA
                                       as libc::c_int & 0x1fff as libc::c_int)
                                      as u32_0 as isize)).x as f32_0;
            vtxA.y =
                (*dVtxList.offset(((*newPoly).c2rust_unnamed.c2rust_unnamed.flags_vIA
                                       as libc::c_int & 0x1fff as libc::c_int)
                                      as u32_0 as isize)).y as f32_0;
            vtxA.z =
                (*dVtxList.offset(((*newPoly).c2rust_unnamed.c2rust_unnamed.flags_vIA
                                       as libc::c_int & 0x1fff as libc::c_int)
                                      as u32_0 as isize)).z as f32_0;
            vtxB.x =
                (*dVtxList.offset(((*newPoly).c2rust_unnamed.c2rust_unnamed.flags_vIB
                                       as libc::c_int & 0x1fff as libc::c_int)
                                      as u32_0 as isize)).x as f32_0;
            vtxB.y =
                (*dVtxList.offset(((*newPoly).c2rust_unnamed.c2rust_unnamed.flags_vIB
                                       as libc::c_int & 0x1fff as libc::c_int)
                                      as u32_0 as isize)).y as f32_0;
            vtxB.z =
                (*dVtxList.offset(((*newPoly).c2rust_unnamed.c2rust_unnamed.flags_vIB
                                       as libc::c_int & 0x1fff as libc::c_int)
                                      as u32_0 as isize)).z as f32_0;
            vtxC.x =
                (*dVtxList.offset((*newPoly).c2rust_unnamed.c2rust_unnamed.vIC
                                      as isize)).x as f32_0;
            vtxC.y =
                (*dVtxList.offset((*newPoly).c2rust_unnamed.c2rust_unnamed.vIC
                                      as isize)).y as f32_0;
            vtxC.z =
                (*dVtxList.offset((*newPoly).c2rust_unnamed.c2rust_unnamed.vIC
                                      as isize)).z as f32_0;
            Math3D_SurfaceNorm(&mut vtxA, &mut vtxB, &mut vtxC,
                               &mut newNormal);
            newNormMagnitude = Math3D_Vec3fMagnitude(&mut newNormal);
            if !(fabsf(newNormMagnitude) < 0.008f32) {
                newNormal.x *= 1.0f32 / newNormMagnitude;
                newNormal.y *= 1.0f32 / newNormMagnitude;
                newNormal.z *= 1.0f32 / newNormMagnitude;
                (*newPoly).normal.x = (newNormal.x * 32767.0f32) as s16;
                (*newPoly).normal.y = (newNormal.y * 32767.0f32) as s16;
                (*newPoly).normal.z = (newNormal.z * 32767.0f32) as s16
            }
            (*newPoly).dist =
                -(newNormal.x *
                      (*dVtxList.offset(((*newPoly).c2rust_unnamed.c2rust_unnamed.flags_vIA
                                             as libc::c_int &
                                             0x1fff as libc::c_int) as u32_0
                                            as isize)).x as libc::c_int as
                          libc::c_float +
                      newNormal.y *
                          (*dVtxList.offset(((*newPoly).c2rust_unnamed.c2rust_unnamed.flags_vIA
                                                 as libc::c_int &
                                                 0x1fff as libc::c_int) as
                                                u32_0 as isize)).y as
                              libc::c_int as libc::c_float +
                      newNormal.z *
                          (*dVtxList.offset(((*newPoly).c2rust_unnamed.c2rust_unnamed.flags_vIA
                                                 as libc::c_int &
                                                 0x1fff as libc::c_int) as
                                                u32_0 as isize)).z as
                              libc::c_int as libc::c_float) as s16;
            if newNormal.y > 0.5f32 {
                let mut polyId: s16 = (*polyStartIndex + i) as s16;
                DynaSSNodeList_SetSSListHead(&mut (*dyna).polyNodes,
                                             &mut (*(*dyna).bgActors.as_mut_ptr().offset(bgId
                                                                                             as
                                                                                             isize)).dynaLookup.floor,
                                             &mut polyId);
            } else if newNormal.y < -0.8f32 {
                let mut polyId_0: s16 = (*polyStartIndex + i) as s16;
                DynaSSNodeList_SetSSListHead(&mut (*dyna).polyNodes,
                                             &mut (*(*dyna).bgActors.as_mut_ptr().offset(bgId
                                                                                             as
                                                                                             isize)).dynaLookup.ceiling,
                                             &mut polyId_0);
            } else {
                let mut polyId_1: s16 = (*polyStartIndex + i) as s16;
                DynaSSNodeList_SetSSListHead(&mut (*dyna).polyNodes,
                                             &mut (*(*dyna).bgActors.as_mut_ptr().offset(bgId
                                                                                             as
                                                                                             isize)).dynaLookup.wall,
                                             &mut polyId_1);
            }
            i += 1
        }
        *polyStartIndex += (*pbgdata).numPolygons as libc::c_int;
        *vtxStartIndex += (*pbgdata).numVertices as libc::c_int
    };
}
#[no_mangle]
pub unsafe extern "C" fn func_8003F8EC(mut globalCtx: *mut GlobalContext,
                                       mut dyna: *mut DynaCollisionContext,
                                       mut actor: *mut Actor) {
    let mut dynaActor: *mut DynaPolyActor = 0 as *mut DynaPolyActor;
    let mut i: s32 = 0;
    i = 0 as libc::c_int;
    while i < 50 as libc::c_int {
        if (*dyna).bgActorFlags[i as usize] as libc::c_int & 1 as libc::c_int
               != 0 {
            dynaActor = DynaPoly_GetActor(&mut (*globalCtx).colCtx, i);
            if !dynaActor.is_null() &&
                   &mut (*dynaActor).actor as *mut Actor == actor {
                func_800434A0(actor as *mut DynaPolyActor);
                return
            }
        }
        i += 1
    };
}
/* *
 * DynaPolyInfo_setup
 */
#[no_mangle]
pub unsafe extern "C" fn DynaPoly_Setup(mut globalCtx: *mut GlobalContext,
                                        mut dyna: *mut DynaCollisionContext) {
    let mut actor: *mut DynaPolyActor = 0 as *mut DynaPolyActor;
    let mut vtxStartIndex: s32 = 0;
    let mut polyStartIndex: s32 = 0;
    let mut i: s32 = 0;
    DynaSSNodeList_ResetCount(&mut (*dyna).polyNodes);
    i = 0 as libc::c_int;
    while i < 50 as libc::c_int {
        DynaLookup_ResetLists(&mut (*(*dyna).bgActors.as_mut_ptr().offset(i as
                                                                              isize)).dynaLookup);
        i += 1
    }
    i = 0 as libc::c_int;
    while i < 50 as libc::c_int {
        if (*dyna).bgActorFlags[i as usize] as libc::c_int & 2 as libc::c_int
               != 0 {
            // Initialize BgActor
            osSyncPrintf(b"\x1b[32m\x00" as *const u8 as *const libc::c_char);
            osSyncPrintf(b"DynaPolyInfo_setup():\xe5\x89\x8a\xe9\x99\xa4 index=%d\n\x00"
                             as *const u8 as *const libc::c_char, i);
            osSyncPrintf(b"\x1b[m\x00" as *const u8 as *const libc::c_char);
            (*dyna).bgActorFlags[i as usize] = 0 as libc::c_int as u16_0;
            BgActor_Initialize(globalCtx,
                               &mut *(*dyna).bgActors.as_mut_ptr().offset(i as
                                                                              isize));
            (*dyna).bitFlag =
                ((*dyna).bitFlag as libc::c_int |
                     (1 as libc::c_int) << 0 as libc::c_int) as u8_0
        }
        if !(*dyna).bgActors[i as usize].actor.is_null() &&
               (*(*dyna).bgActors[i as usize].actor).update.is_none() {
            // Delete BgActor
            osSyncPrintf(b"\x1b[32m\x00" as *const u8 as *const libc::c_char);
            osSyncPrintf(b"DynaPolyInfo_setup():\xe5\x89\x8a\xe9\x99\xa4 index=%d\n\x00"
                             as *const u8 as *const libc::c_char, i);
            osSyncPrintf(b"\x1b[m\x00" as *const u8 as *const libc::c_char);
            actor = DynaPoly_GetActor(&mut (*globalCtx).colCtx, i);
            if actor.is_null() { return }
            (*actor).bgId = -(1 as libc::c_int);
            (*dyna).bgActorFlags[i as usize] = 0 as libc::c_int as u16_0;
            BgActor_Initialize(globalCtx,
                               &mut *(*dyna).bgActors.as_mut_ptr().offset(i as
                                                                              isize));
            (*dyna).bitFlag =
                ((*dyna).bitFlag as libc::c_int |
                     (1 as libc::c_int) << 0 as libc::c_int) as u8_0
        }
        i += 1
    }
    vtxStartIndex = 0 as libc::c_int;
    polyStartIndex = 0 as libc::c_int;
    i = 0 as libc::c_int;
    while i < 50 as libc::c_int {
        if (*dyna).bgActorFlags[i as usize] as libc::c_int & 1 as libc::c_int
               != 0 {
            DynaPoly_ExpandSRT(globalCtx, dyna, i, &mut vtxStartIndex,
                               &mut polyStartIndex);
        }
        i += 1
    }
    (*dyna).bitFlag =
        ((*dyna).bitFlag as libc::c_int &
             !((1 as libc::c_int) << 0 as libc::c_int)) as u8_0;
}
/* *
 * Update all BgActor's previous ScaleRotPos
 */
#[no_mangle]
pub unsafe extern "C" fn DynaPoly_UpdateBgActorTransforms(mut globalCtx:
                                                              *mut GlobalContext,
                                                          mut dyna:
                                                              *mut DynaCollisionContext) {
    let mut i: s32 = 0;
    i = 0 as libc::c_int;
    while i < 50 as libc::c_int {
        if (*dyna).bgActorFlags[i as usize] as libc::c_int & 1 as libc::c_int
               != 0 {
            DynaPoly_SetBgActorPrevTransform(globalCtx,
                                             &mut *(*dyna).bgActors.as_mut_ptr().offset(i
                                                                                            as
                                                                                            isize));
        }
        i += 1
    };
}
/* *
 * Perform dyna poly raycast toward floor on a list of floor, wall, or ceiling polys
 * `listType` specifies the poly list type (e.g. DYNA_RAYCAST_FLOORS)
 */
#[no_mangle]
pub unsafe extern "C" fn BgCheck_RaycastFloorDynaList(mut dynaRaycast:
                                                          *mut DynaRaycast,
                                                      mut listType: u32_0)
 -> f32_0 {
    let mut polyList: *mut CollisionPoly = 0 as *mut CollisionPoly;
    let mut curNode: *mut SSNode = 0 as *mut SSNode;
    let mut result: f32_0 = 0.;
    let mut yIntersect: f32_0 = 0.;
    let mut id: s16 = 0;
    result = (*dynaRaycast).yIntersect;
    if (*(*dynaRaycast).ssList).head as libc::c_int == 0xffff as libc::c_int {
        return result
    }
    polyList = (*(*dynaRaycast).dyna).polyList;
    curNode =
        &mut *(*(*dynaRaycast).dyna).polyNodes.tbl.offset((*(*dynaRaycast).ssList).head
                                                              as isize) as
            *mut SSNode;
    loop  {
        id = (*curNode).polyId;
        if (*polyList.offset(id as
                                 isize)).c2rust_unnamed.c2rust_unnamed.flags_vIA
               as libc::c_int &
               ((*dynaRaycast).xpFlags as libc::c_int & 7 as libc::c_int) <<
                   13 as libc::c_int != 0 {
            if (*curNode).next as libc::c_int == 0xffff as libc::c_int {
                break ;
            }
            curNode =
                &mut *(*(*dynaRaycast).dyna).polyNodes.tbl.offset((*curNode).next
                                                                      as
                                                                      isize)
                    as *mut SSNode
        } else if listType &
                      (2 as libc::c_int | 4 as libc::c_int) as libc::c_uint !=
                      0 &&
                      (*dynaRaycast).unk_20 &
                          0x10 as libc::c_int as libc::c_uint != 0 &&
                      (*polyList.offset(id as isize)).normal.y as libc::c_int
                          as libc::c_float * (1.0f32 / 32767.0f32) < 0.0f32 {
            if (*curNode).next as libc::c_int == 0xffff as libc::c_int {
                break ;
            }
            curNode =
                &mut *(*(*dynaRaycast).dyna).polyNodes.tbl.offset((*curNode).next
                                                                      as
                                                                      isize)
                    as *mut SSNode
        } else {
            if CollisionPoly_CheckYIntersectApprox1(&mut *polyList.offset(id
                                                                              as
                                                                              isize),
                                                    (*(*dynaRaycast).dyna).vtxList,
                                                    (*(*dynaRaycast).pos).x,
                                                    (*(*dynaRaycast).pos).z,
                                                    &mut yIntersect,
                                                    (*dynaRaycast).chkDist) ==
                   1 as libc::c_int && yIntersect < (*(*dynaRaycast).pos).y &&
                   result < yIntersect {
                result = yIntersect;
                *(*dynaRaycast).resultPoly =
                    &mut *(*(*dynaRaycast).dyna).polyList.offset(id as isize)
                        as *mut CollisionPoly
            }
            if (*curNode).next as libc::c_int == 0xffff as libc::c_int {
                break ;
            }
            curNode =
                &mut *(*(*dynaRaycast).dyna).polyNodes.tbl.offset((*curNode).next
                                                                      as
                                                                      isize)
                    as *mut SSNode
        }
    }
    return result;
}
/* *
 * Perform dyna poly raycast toward floor
 * returns the yIntersect of the poly found, or BGCHECK_Y_MIN if no poly is found
 */
#[no_mangle]
pub unsafe extern "C" fn BgCheck_RaycastFloorDyna(mut dynaRaycast:
                                                      *mut DynaRaycast)
 -> f32_0 {
    let mut i: s32 = 0;
    let mut result: f32_0 = 0.;
    let mut intersect2: f32_0 = 0.;
    let mut i2: s32 = 0;
    let mut pauseState: s32 = 0;
    let mut dynaActor: *mut DynaPolyActor = 0 as *mut DynaPolyActor;
    let mut pad: s32 = 0;
    let mut polyVtx: [Vec3f; 3] = [Vec3f{x: 0., y: 0., z: 0.,}; 3];
    let mut polyNorm: Vec3f = Vec3f{x: 0., y: 0., z: 0.,};
    let mut polyIndex: u32_0 = 0;
    let mut polyMin: *mut CollisionPoly = 0 as *mut CollisionPoly;
    let mut srpMtx: MtxF = MtxF{mf: [[0.; 4]; 4],};
    let mut magnitude: f32_0 = 0.;
    let mut vtxList: *mut Vec3s = 0 as *mut Vec3s;
    let mut polyDist: f32_0 = 0.;
    let mut vtx: Vec3f = Vec3f{x: 0., y: 0., z: 0.,};
    let mut intersect: f32_0 = 0.;
    let mut curTransform: *mut ScaleRotPos = 0 as *mut ScaleRotPos;
    let mut poly: *mut CollisionPoly = 0 as *mut CollisionPoly;
    result = -32000.0f32;
    *(*dynaRaycast).bgId = 50 as libc::c_int;
    i = 0 as libc::c_int;
    while i < 50 as libc::c_int {
        if !((*(*dynaRaycast).colCtx).dyna.bgActorFlags[i as usize] as
                 libc::c_int & 1 as libc::c_int == 0) {
            if !((*dynaRaycast).actor ==
                     (*(*dynaRaycast).colCtx).dyna.bgActors[i as usize].actor
                     ||
                     (*(*dynaRaycast).pos).y <
                         (*(*dynaRaycast).colCtx).dyna.bgActors[i as
                                                                    usize].minY
                     ||
                     Math3D_XZInSphere(&mut (*(*(*dynaRaycast).colCtx).dyna.bgActors.as_mut_ptr().offset(i
                                                                                                             as
                                                                                                             isize)).boundingSphere,
                                       (*(*dynaRaycast).pos).x,
                                       (*(*dynaRaycast).pos).z) ==
                         0 as libc::c_int) {
                (*dynaRaycast).dyna = &mut (*(*dynaRaycast).colCtx).dyna;
                if (*dynaRaycast).unk_20 &
                       ((1 as libc::c_int) << 2 as libc::c_int) as
                           libc::c_uint != 0 {
                    (*dynaRaycast).ssList =
                        &mut (*(*(*dynaRaycast).colCtx).dyna.bgActors.as_mut_ptr().offset(i
                                                                                              as
                                                                                              isize)).dynaLookup.floor;
                    intersect2 =
                        BgCheck_RaycastFloorDynaList(dynaRaycast,
                                                     1 as libc::c_int as
                                                         u32_0);
                    if (*dynaRaycast).yIntersect < intersect2 {
                        (*dynaRaycast).yIntersect = intersect2;
                        *(*dynaRaycast).bgId = i;
                        result = intersect2
                    }
                }
                if (*dynaRaycast).unk_20 &
                       ((1 as libc::c_int) << 1 as libc::c_int) as
                           libc::c_uint != 0 ||
                       (*(*dynaRaycast).resultPoly).is_null() &&
                           (*dynaRaycast).unk_20 &
                               8 as libc::c_int as libc::c_uint != 0 {
                    (*dynaRaycast).ssList =
                        &mut (*(*(*dynaRaycast).colCtx).dyna.bgActors.as_mut_ptr().offset(i
                                                                                              as
                                                                                              isize)).dynaLookup.wall;
                    intersect2 =
                        BgCheck_RaycastFloorDynaList(dynaRaycast,
                                                     2 as libc::c_int as
                                                         u32_0);
                    if (*dynaRaycast).yIntersect < intersect2 {
                        (*dynaRaycast).yIntersect = intersect2;
                        *(*dynaRaycast).bgId = i;
                        result = intersect2
                    }
                }
                if (*dynaRaycast).unk_20 &
                       ((1 as libc::c_int) << 0 as libc::c_int) as
                           libc::c_uint != 0 {
                    (*dynaRaycast).ssList =
                        &mut (*(*(*dynaRaycast).colCtx).dyna.bgActors.as_mut_ptr().offset(i
                                                                                              as
                                                                                              isize)).dynaLookup.ceiling;
                    intersect2 =
                        BgCheck_RaycastFloorDynaList(dynaRaycast,
                                                     4 as libc::c_int as
                                                         u32_0);
                    if (*dynaRaycast).yIntersect < intersect2 {
                        (*dynaRaycast).yIntersect = intersect2;
                        *(*dynaRaycast).bgId = i;
                        result = intersect2
                    }
                }
            }
        }
        i += 1
    }
    dynaActor =
        DynaPoly_GetActor((*dynaRaycast).colCtx, *(*dynaRaycast).bgId);
    if result != -32000.0f32 && !dynaActor.is_null() &&
           !(*dynaRaycast).globalCtx.is_null() {
        pauseState =
            ((*(*dynaRaycast).globalCtx).pauseCtx.state as libc::c_int !=
                 0 as libc::c_int) as libc::c_int;
        if pauseState == 0 as libc::c_int {
            pauseState =
                ((*(*dynaRaycast).globalCtx).pauseCtx.debugState as
                     libc::c_int != 0 as libc::c_int) as libc::c_int
        }
        if pauseState == 0 &&
               (*(*dynaRaycast).colCtx).dyna.bgActorFlags[*(*dynaRaycast).bgId
                                                              as usize] as
                   libc::c_int & 2 as libc::c_int != 0 {
            curTransform =
                &mut (*(*(*dynaRaycast).dyna).bgActors.as_mut_ptr().offset(*(*dynaRaycast).bgId
                                                                               as
                                                                               isize)).curTransform;
            polyMin =
                &mut *(*(*dynaRaycast).dyna).polyList.offset((*(*(*dynaRaycast).dyna).bgActors.as_mut_ptr().offset(*(*dynaRaycast).bgId
                                                                                                                       as
                                                                                                                       isize)).dynaLookup.polyStartIndex
                                                                 as isize) as
                    *mut CollisionPoly;
            polyIndex =
                (*(*dynaRaycast).resultPoly).wrapping_offset_from(polyMin) as
                    libc::c_int as u32_0;
            poly =
                &mut *(*(*(*(*dynaRaycast).dyna).bgActors.as_mut_ptr().offset(*(*dynaRaycast).bgId
                                                                                  as
                                                                                  isize)).colHeader).polyList.offset(polyIndex
                                                                                                                         as
                                                                                                                         isize)
                    as *mut CollisionPoly;
            SkinMatrix_SetTranslateRotateYXZScale(&mut srpMtx,
                                                  (*curTransform).scale.x,
                                                  (*curTransform).scale.y,
                                                  (*curTransform).scale.z,
                                                  (*curTransform).rot.x,
                                                  (*curTransform).rot.y,
                                                  (*curTransform).rot.z,
                                                  (*curTransform).pos.x,
                                                  (*curTransform).pos.y,
                                                  (*curTransform).pos.z);
            vtxList =
                (*(*(*dynaRaycast).dyna).bgActors[*(*dynaRaycast).bgId as
                                                      usize].colHeader).vtxList;
            i2 = 0 as libc::c_int;
            while i2 < 3 as libc::c_int {
                Math_Vec3s_ToVec3f(&mut vtx,
                                   &mut *vtxList.offset((*(*poly).c2rust_unnamed.vtxData.as_mut_ptr().offset(i2
                                                                                                                 as
                                                                                                                 isize)
                                                             as libc::c_int &
                                                             0x1fff as
                                                                 libc::c_int)
                                                            as isize));
                SkinMatrix_Vec3fMtxFMultXYZ(&mut srpMtx, &mut vtx,
                                            &mut *polyVtx.as_mut_ptr().offset(i2
                                                                                  as
                                                                                  isize));
                i2 += 1
            }
            Math3D_SurfaceNorm(&mut *polyVtx.as_mut_ptr().offset(0 as
                                                                     libc::c_int
                                                                     as
                                                                     isize),
                               &mut *polyVtx.as_mut_ptr().offset(1 as
                                                                     libc::c_int
                                                                     as
                                                                     isize),
                               &mut *polyVtx.as_mut_ptr().offset(2 as
                                                                     libc::c_int
                                                                     as
                                                                     isize),
                               &mut polyNorm);
            magnitude = Math3D_Vec3fMagnitude(&mut polyNorm);
            if !(fabsf(magnitude) < 0.008f32) {
                polyNorm.x *= 1.0f32 / magnitude;
                polyNorm.y *= 1.0f32 / magnitude;
                polyNorm.z *= 1.0f32 / magnitude;
                polyDist =
                    -(polyNorm.x * polyVtx[0 as libc::c_int as usize].x +
                          polyNorm.y * polyVtx[0 as libc::c_int as usize].y +
                          polyNorm.z * polyVtx[0 as libc::c_int as usize].z);
                if Math3D_TriChkPointParaYIntersectInsideTri(&mut *polyVtx.as_mut_ptr().offset(0
                                                                                                   as
                                                                                                   libc::c_int
                                                                                                   as
                                                                                                   isize),
                                                             &mut *polyVtx.as_mut_ptr().offset(1
                                                                                                   as
                                                                                                   libc::c_int
                                                                                                   as
                                                                                                   isize),
                                                             &mut *polyVtx.as_mut_ptr().offset(2
                                                                                                   as
                                                                                                   libc::c_int
                                                                                                   as
                                                                                                   isize),
                                                             polyNorm.x,
                                                             polyNorm.y,
                                                             polyNorm.z,
                                                             polyDist,
                                                             (*(*dynaRaycast).pos).z,
                                                             (*(*dynaRaycast).pos).x,
                                                             &mut intersect,
                                                             (*dynaRaycast).chkDist)
                       != 0 {
                    if fabsf(intersect - result) < 1.0f32 {
                        result = intersect
                    }
                }
            }
        }
    }
    return result;
}
/* *
 * Performs collision detection on a BgActor's wall polys on sphere `pos`, `radius`
 * returns true if a collision was detected
 * `outX` `outZ` return the displaced x,z coordinates
 * `outPoly` returns the pointer to the nearest poly collided with, or NULL
 * `outBgId` returns `bgId` if the poly SurfaceType's wall damage flag is not set, else ?
 */
#[no_mangle]
pub unsafe extern "C" fn BgCheck_SphVsDynaWallInBgActor(mut colCtx:
                                                            *mut CollisionContext,
                                                        mut xpFlags: u16_0,
                                                        mut dyna:
                                                            *mut DynaCollisionContext,
                                                        mut ssList:
                                                            *mut SSList,
                                                        mut outX: *mut f32_0,
                                                        mut outZ: *mut f32_0,
                                                        mut outPoly:
                                                            *mut *mut CollisionPoly,
                                                        mut outBgId: *mut s32,
                                                        mut pos: *mut Vec3f,
                                                        mut radius: f32_0,
                                                        mut bgId: s32)
 -> s32 {
    let mut temp: f32_0 = 0.;
    let mut intersect: f32_0 = 0.;
    let mut result: s32 = 0 as libc::c_int;
    let mut poly: *mut CollisionPoly = 0 as *mut CollisionPoly;
    let mut curNode: *mut SSNode = 0 as *mut SSNode;
    let mut nx: f32_0 = 0.;
    let mut ny: f32_0 = 0.;
    let mut nz: f32_0 = 0.;
    let mut resultPos: Vec3f = Vec3f{x: 0., y: 0., z: 0.,};
    let mut polyId: s16 = 0;
    let mut zTemp: f32_0 = 0.;
    let mut xTemp: f32_0 = 0.;
    let mut normalXZ: f32_0 = 0.;
    let mut invNormalXZ: f32_0 = 0.;
    let mut planeDist: f32_0 = 0.;
    let mut temp_f18: f32_0 = 0.;
    let mut zIntersectDist: f32_0 = 0.;
    let mut xIntersectDist: f32_0 = 0.;
    let mut zMin: f32_0 = 0.;
    let mut zMax: f32_0 = 0.;
    let mut xMin: f32_0 = 0.;
    let mut xMax: f32_0 = 0.;
    if (*ssList).head as libc::c_int == 0xffff as libc::c_int {
        return result
    }
    resultPos = *pos;
    curNode =
        &mut *(*dyna).polyNodes.tbl.offset((*ssList).head as isize) as
            *mut SSNode;
    loop  {
        polyId = (*curNode).polyId;
        poly =
            &mut *(*dyna).polyList.offset(polyId as isize) as
                *mut CollisionPoly;
        CollisionPoly_GetNormalF(poly, &mut nx, &mut ny, &mut nz);
        normalXZ = sqrtf(nx * nx + nz * nz);
        if !(fabsf(normalXZ) < 0.008f32) {
        } else {
            __assert(b"!IS_ZERO(ac_size)\x00" as *const u8 as
                         *const libc::c_char,
                     b"../z_bgcheck.c\x00" as *const u8 as
                         *const libc::c_char, 7382 as libc::c_int);
        };
        planeDist =
            Math3D_DistPlaneToPos(nx, ny, nz, (*poly).dist as f32_0,
                                  &mut resultPos);
        if radius < fabsf(planeDist) ||
               (*poly).c2rust_unnamed.c2rust_unnamed.flags_vIA as libc::c_int
                   &
                   (xpFlags as libc::c_int & 7 as libc::c_int) <<
                       13 as libc::c_int != 0 {
            if (*curNode).next as libc::c_int == 0xffff as libc::c_int {
                break ;
            }
            curNode =
                &mut *(*dyna).polyNodes.tbl.offset((*curNode).next as isize)
                    as *mut SSNode
        } else {
            invNormalXZ = 1.0f32 / normalXZ;
            temp_f18 = fabsf(nz) * invNormalXZ;
            if temp_f18 < 0.4f32 {
                if (*curNode).next as libc::c_int == 0xffff as libc::c_int {
                    break ;
                }
                curNode =
                    &mut *(*dyna).polyNodes.tbl.offset((*curNode).next as
                                                           isize) as
                        *mut SSNode
            } else {
                // compute poly zMin/zMax
                zTemp =
                    (*(*dyna).vtxList.offset(((*poly).c2rust_unnamed.c2rust_unnamed.flags_vIA
                                                  as libc::c_int &
                                                  0x1fff as libc::c_int) as
                                                 isize)).z as f32_0;
                zMin = zTemp;
                zMax = zMin;
                zTemp =
                    (*(*dyna).vtxList.offset(((*poly).c2rust_unnamed.c2rust_unnamed.flags_vIB
                                                  as libc::c_int &
                                                  0x1fff as libc::c_int) as
                                                 isize)).z as f32_0;
                if zTemp < zMin {
                    zMin = zTemp
                } else if zTemp > zMax { zMax = zTemp }
                zTemp =
                    (*(*dyna).vtxList.offset((*poly).c2rust_unnamed.c2rust_unnamed.vIC
                                                 as isize)).z as f32_0;
                if zTemp < zMin {
                    zMin = zTemp
                } else if zMax < zTemp { zMax = zTemp }
                zMin -= radius;
                zMax += radius;
                if resultPos.z < zMin || zMax < resultPos.z {
                    if (*curNode).next as libc::c_int == 0xffff as libc::c_int
                       {
                        break ;
                    }
                    curNode =
                        &mut *(*dyna).polyNodes.tbl.offset((*curNode).next as
                                                               isize) as
                            *mut SSNode
                } else {
                    if CollisionPoly_CheckZIntersectApprox(poly,
                                                           (*dyna).vtxList,
                                                           resultPos.x,
                                                           (*pos).y,
                                                           &mut intersect) !=
                           0 {
                        if fabsf(intersect - resultPos.z) <= radius / temp_f18
                           {
                            if (intersect - resultPos.z) * nz <= 4.0f32 {
                                if BgCheck_ComputeWallDisplacement(colCtx,
                                                                   poly,
                                                                   &mut resultPos.x,
                                                                   &mut resultPos.z,
                                                                   nx, ny, nz,
                                                                   invNormalXZ,
                                                                   planeDist,
                                                                   radius,
                                                                   outPoly) !=
                                       0 {
                                    *outBgId = bgId
                                }
                                result = 1 as libc::c_int
                            }
                        }
                    }
                    if (*curNode).next as libc::c_int == 0xffff as libc::c_int
                       {
                        break ;
                    }
                    curNode =
                        &mut *(*dyna).polyNodes.tbl.offset((*curNode).next as
                                                               isize) as
                            *mut SSNode
                }
            }
        }
    }
    curNode =
        &mut *(*dyna).polyNodes.tbl.offset((*ssList).head as isize) as
            *mut SSNode;
    loop  {
        polyId = (*curNode).polyId;
        poly =
            &mut *(*dyna).polyList.offset(polyId as isize) as
                *mut CollisionPoly;
        CollisionPoly_GetNormalF(poly, &mut nx, &mut ny, &mut nz);
        normalXZ = sqrtf(nx * nx + nz * nz);
        if !(fabsf(normalXZ) < 0.008f32) {
        } else {
            __assert(b"!IS_ZERO(ac_size)\x00" as *const u8 as
                         *const libc::c_char,
                     b"../z_bgcheck.c\x00" as *const u8 as
                         *const libc::c_char, 7489 as libc::c_int);
        };
        planeDist =
            Math3D_DistPlaneToPos(nx, ny, nz, (*poly).dist as f32_0,
                                  &mut resultPos);
        if radius < fabsf(planeDist) ||
               (*poly).c2rust_unnamed.c2rust_unnamed.flags_vIA as libc::c_int
                   &
                   (xpFlags as libc::c_int & 7 as libc::c_int) <<
                       13 as libc::c_int != 0 {
            if (*curNode).next as libc::c_int == 0xffff as libc::c_int {
                break ;
            }
            curNode =
                &mut *(*dyna).polyNodes.tbl.offset((*curNode).next as isize)
                    as *mut SSNode
        } else {
            invNormalXZ = 1.0f32 / normalXZ;
            temp_f18 = fabsf(nx) * invNormalXZ;
            if temp_f18 < 0.4f32 {
                if (*curNode).next as libc::c_int == 0xffff as libc::c_int {
                    break ;
                }
                curNode =
                    &mut *(*dyna).polyNodes.tbl.offset((*curNode).next as
                                                           isize) as
                        *mut SSNode
            } else {
                // compute poly xMin/xMax
                xTemp =
                    (*(*dyna).vtxList.offset(((*poly).c2rust_unnamed.c2rust_unnamed.flags_vIA
                                                  as libc::c_int &
                                                  0x1fff as libc::c_int) as
                                                 isize)).x as f32_0;
                xMin = xTemp;
                xMax = xMin;
                xTemp =
                    (*(*dyna).vtxList.offset(((*poly).c2rust_unnamed.c2rust_unnamed.flags_vIB
                                                  as libc::c_int &
                                                  0x1fff as libc::c_int) as
                                                 isize)).x as f32_0;
                if xTemp < xMin {
                    xMin = xTemp
                } else if xMax < xTemp { xMax = xTemp }
                xTemp =
                    (*(*dyna).vtxList.offset((*poly).c2rust_unnamed.c2rust_unnamed.vIC
                                                 as isize)).x as f32_0;
                if xTemp < xMin {
                    xMin = xTemp
                } else if xMax < xTemp { xMax = xTemp }
                xMin -= radius;
                xMax += radius;
                if resultPos.x < xMin || xMax < resultPos.x {
                    if (*curNode).next as libc::c_int == 0xffff as libc::c_int
                       {
                        break ;
                    }
                    curNode =
                        &mut *(*dyna).polyNodes.tbl.offset((*curNode).next as
                                                               isize) as
                            *mut SSNode
                } else {
                    if CollisionPoly_CheckXIntersectApprox(poly,
                                                           (*dyna).vtxList,
                                                           (*pos).y,
                                                           resultPos.z,
                                                           &mut intersect) !=
                           0 {
                        xIntersectDist = intersect - resultPos.x;
                        if fabsf(xIntersectDist) <= radius / temp_f18 {
                            if xIntersectDist * nx <= 4.0f32 {
                                if BgCheck_ComputeWallDisplacement(colCtx,
                                                                   poly,
                                                                   &mut resultPos.x,
                                                                   &mut resultPos.z,
                                                                   nx, ny, nz,
                                                                   invNormalXZ,
                                                                   planeDist,
                                                                   radius,
                                                                   outPoly) !=
                                       0 {
                                    *outBgId = bgId
                                }
                                result = 1 as libc::c_int
                            }
                        }
                    }
                    if (*curNode).next as libc::c_int == 0xffff as libc::c_int
                       {
                        break ;
                    }
                    curNode =
                        &mut *(*dyna).polyNodes.tbl.offset((*curNode).next as
                                                               isize) as
                            *mut SSNode
                }
            }
        }
    }
    *outX = resultPos.x;
    *outZ = resultPos.z;
    return result;
}
/* *
 * Performs collision detection on all dyna poly walls using sphere `pos`, `radius`
 * returns true if a collision was detected
 * `outX` `outZ` return the displaced x,z coordinates
 * `outPoly` returns the pointer to the nearest poly collided with, or NULL
 * `outBgId` returns the index of the BgActor that owns `outPoly`
 * If `actor` is not NULL, an BgActor bound to that actor will be ignored
 */
#[no_mangle]
pub unsafe extern "C" fn BgCheck_SphVsDynaWall(mut colCtx:
                                                   *mut CollisionContext,
                                               mut xpFlags: u16_0,
                                               mut outX: *mut f32_0,
                                               mut outZ: *mut f32_0,
                                               mut pos: *mut Vec3f,
                                               mut radius: f32_0,
                                               mut outPoly:
                                                   *mut *mut CollisionPoly,
                                               mut outBgId: *mut s32,
                                               mut actor: *mut Actor) -> s32 {
    let mut resultPos: Vec3f = Vec3f{x: 0., y: 0., z: 0.,};
    let mut result: s32 = 0;
    let mut r: f32_0 = 0.;
    let mut dz: f32_0 = 0.;
    let mut dx: f32_0 = 0.;
    let mut bgActor: *mut BgActor = 0 as *mut BgActor;
    let mut i: s32 = 0;
    result = 0 as libc::c_int;
    resultPos = *pos;
    i = 0 as libc::c_int;
    while i < 50 as libc::c_int {
        if !((*colCtx).dyna.bgActorFlags[i as usize] as libc::c_int &
                 1 as libc::c_int == 0) {
            if !((*(*colCtx).dyna.bgActors.as_mut_ptr().offset(i as
                                                                   isize)).actor
                     == actor) {
                bgActor =
                    &mut *(*colCtx).dyna.bgActors.as_mut_ptr().offset(i as
                                                                          isize)
                        as *mut BgActor;
                if !((*bgActor).minY > resultPos.y ||
                         (*bgActor).maxY < resultPos.y) {
                    (*bgActor).boundingSphere.radius =
                        ((*bgActor).boundingSphere.radius as libc::c_int +
                             radius as s16 as libc::c_int) as s16;
                    r = (*bgActor).boundingSphere.radius as f32_0;
                    dx =
                        (*bgActor).boundingSphere.center.x as libc::c_int as
                            libc::c_float - resultPos.x;
                    dz =
                        (*bgActor).boundingSphere.center.z as libc::c_int as
                            libc::c_float - resultPos.z;
                    if r * r < dx * dx + dz * dz ||
                           Math3D_XYInSphere(&mut (*bgActor).boundingSphere,
                                             resultPos.x, resultPos.y) == 0 &&
                               Math3D_YZInSphere(&mut (*bgActor).boundingSphere,
                                                 resultPos.y, resultPos.z) ==
                                   0 {
                        (*bgActor).boundingSphere.radius =
                            ((*bgActor).boundingSphere.radius as libc::c_int -
                                 radius as s16 as libc::c_int) as s16
                    } else {
                        (*bgActor).boundingSphere.radius =
                            ((*bgActor).boundingSphere.radius as libc::c_int -
                                 radius as s16 as libc::c_int) as s16;
                        if BgCheck_SphVsDynaWallInBgActor(colCtx, xpFlags,
                                                          &mut (*colCtx).dyna,
                                                          &mut (*(*colCtx).dyna.bgActors.as_mut_ptr().offset(i
                                                                                                                 as
                                                                                                                 isize)).dynaLookup.wall,
                                                          outX, outZ, outPoly,
                                                          outBgId,
                                                          &mut resultPos,
                                                          radius, i) != 0 {
                            resultPos.x = *outX;
                            resultPos.z = *outZ;
                            result = 1 as libc::c_int
                        }
                    }
                }
            }
        }
        i += 1
    }
    return result;
}
/* *
 * Tests for collision with a dyna poly ceiling, starting at `ssList`
 * returns true if a collision occurs, else false
 * `outPoly` returns the poly collided with
 * `outY` returns the y coordinate needed to not collide with `outPoly`
 */
#[no_mangle]
pub unsafe extern "C" fn BgCheck_CheckDynaCeilingList(mut colCtx:
                                                          *mut CollisionContext,
                                                      mut xpFlags: u16_0,
                                                      mut dyna:
                                                          *mut DynaCollisionContext,
                                                      mut ssList: *mut SSList,
                                                      mut outY: *mut f32_0,
                                                      mut pos: *mut Vec3f,
                                                      mut checkHeight: f32_0,
                                                      mut outPoly:
                                                          *mut *mut CollisionPoly)
 -> s32 {
    let mut polyId: s16 = 0;
    let mut curNode: *mut SSNode = 0 as *mut SSNode;
    let mut poly: *mut CollisionPoly = 0 as *mut CollisionPoly;
    let mut testPos: Vec3f = Vec3f{x: 0., y: 0., z: 0.,};
    let mut ceilingY: f32_0 = 0.;
    let mut sign: f32_0 = 0.;
    let mut nx: f32_0 = 0.;
    let mut ny: f32_0 = 0.;
    let mut nz: f32_0 = 0.;
    let mut result: s32 = 0 as libc::c_int;
    let mut intersectDist: f32_0 = 0.;
    let mut padding: u16_0 = 0;
    if (*ssList).head as libc::c_int == 0xffff as libc::c_int {
        return 0 as libc::c_int
    }
    curNode =
        &mut *(*dyna).polyNodes.tbl.offset((*ssList).head as isize) as
            *mut SSNode;
    testPos = *pos;
    loop  {
        polyId = (*curNode).polyId;
        poly =
            &mut *(*dyna).polyList.offset(polyId as isize) as
                *mut CollisionPoly;
        if (*poly).c2rust_unnamed.c2rust_unnamed.flags_vIA as libc::c_int &
               (xpFlags as libc::c_int & 7 as libc::c_int) <<
                   13 as libc::c_int != 0 {
            if (*curNode).next as libc::c_int == 0xffff as libc::c_int {
                break ;
            }
            curNode =
                &mut *(*dyna).polyNodes.tbl.offset((*curNode).next as isize)
                    as *mut SSNode
        } else {
            CollisionPoly_GetNormalF(poly, &mut nx, &mut ny, &mut nz);
            if checkHeight <
                   Math3D_UDistPlaneToPos(nx, ny, nz, (*poly).dist as f32_0,
                                          &mut testPos) {
                if (*curNode).next as libc::c_int == 0xffff as libc::c_int {
                    break ;
                }
                curNode =
                    &mut *(*dyna).polyNodes.tbl.offset((*curNode).next as
                                                           isize) as
                        *mut SSNode
            } else {
                if CollisionPoly_CheckYIntersectApprox2(poly, (*dyna).vtxList,
                                                        testPos.x, testPos.z,
                                                        &mut ceilingY) != 0 {
                    intersectDist = ceilingY - testPos.y;
                    if testPos.y < ceilingY && intersectDist < checkHeight &&
                           intersectDist * ny <= 0.0f32 {
                        sign = if 0.0f32 <= ny { 1.0f32 } else { -1.0f32 };
                        testPos.y = sign * checkHeight + ceilingY;
                        result = 1 as libc::c_int;
                        *outPoly = poly
                    }
                }
                if (*curNode).next as libc::c_int == 0xffff as libc::c_int {
                    break ;
                }
                curNode =
                    &mut *(*dyna).polyNodes.tbl.offset((*curNode).next as
                                                           isize) as
                        *mut SSNode
            }
        }
    }
    *outY = testPos.y;
    return result;
}
/* *
 * Tests collision with a dyna poly ceiling
 * returns true if a collision occurs, else false
 * `outPoly` returns the poly collided with, while `outBgId` returns the id of the BgActor that owns the poly
 * `outY` returns the y coordinate needed to not collide with `outPoly`, or `pos`.y + `chkDist` if no collision occurs
 */
#[no_mangle]
pub unsafe extern "C" fn BgCheck_CheckDynaCeiling(mut colCtx:
                                                      *mut CollisionContext,
                                                  mut xpFlags: u16_0,
                                                  mut outY: *mut f32_0,
                                                  mut pos: *mut Vec3f,
                                                  mut chkDist: f32_0,
                                                  mut outPoly:
                                                      *mut *mut CollisionPoly,
                                                  mut outBgId: *mut s32,
                                                  mut actor: *mut Actor)
 -> s32 {
    let mut i: s32 = 0 as libc::c_int;
    let mut result: s32 = 0 as libc::c_int;
    let mut resultY: f32_0 = 0.;
    let mut tempY: f32_0 = chkDist + (*pos).y;
    let mut bgActor: *mut BgActor = 0 as *mut BgActor;
    let mut poly: *mut CollisionPoly = 0 as *mut CollisionPoly;
    resultY = tempY;
    i = 0 as libc::c_int;
    while i < 50 as libc::c_int {
        if !((*colCtx).dyna.bgActorFlags[i as usize] as libc::c_int &
                 1 as libc::c_int == 0) {
            if !(actor == (*colCtx).dyna.bgActors[i as usize].actor) {
                if !(Math3D_XZInSphere(&mut (*(*colCtx).dyna.bgActors.as_mut_ptr().offset(i
                                                                                              as
                                                                                              isize)).boundingSphere,
                                       (*pos).x, (*pos).z) == 0) {
                    if BgCheck_CheckDynaCeilingList(colCtx, xpFlags,
                                                    &mut (*colCtx).dyna,
                                                    &mut (*(*colCtx).dyna.bgActors.as_mut_ptr().offset(i
                                                                                                           as
                                                                                                           isize)).dynaLookup.ceiling,
                                                    &mut tempY, pos, chkDist,
                                                    &mut poly) ==
                           1 as libc::c_int && tempY < resultY {
                        resultY = tempY;
                        *outPoly = poly;
                        *outBgId = i;
                        result = 1 as libc::c_int
                    }
                }
            }
        }
        i += 1
    }
    *outY = resultY;
    return result;
}
/* *
 * Tests if DynaLineTest intersects with a poly
 * returns true if a poly was intersected, else false
 */
#[no_mangle]
pub unsafe extern "C" fn BgCheck_CheckLineAgainstBgActorSSList(mut dynaLineTest:
                                                                   *mut DynaLineTest)
 -> s32 {
    let mut distSq: f32_0 = 0.;
    let mut result: s32 = 0;
    let mut curPoly: *mut CollisionPoly = 0 as *mut CollisionPoly;
    let mut curNode: *mut SSNode = 0 as *mut SSNode;
    let mut polyIntersect: Vec3f = Vec3f{x: 0., y: 0., z: 0.,};
    let mut polyId: s16 = 0;
    if (*(*dynaLineTest).ssList).head as libc::c_int == 0xffff as libc::c_int
       {
        return 0 as libc::c_int
    }
    curNode =
        &mut *(*(*dynaLineTest).dyna).polyNodes.tbl.offset((*(*dynaLineTest).ssList).head
                                                               as isize) as
            *mut SSNode;
    result = 0 as libc::c_int;
    loop  {
        polyId = (*curNode).polyId;
        curPoly =
            &mut *(*(*dynaLineTest).dyna).polyList.offset(polyId as isize) as
                *mut CollisionPoly;
        if (*curPoly).c2rust_unnamed.c2rust_unnamed.flags_vIA as libc::c_int &
               ((*dynaLineTest).xpFlags as libc::c_int & 7 as libc::c_int) <<
                   13 as libc::c_int != 0 {
            if (*curNode).next as libc::c_int == 0xffff as libc::c_int {
                break ;
            }
            curNode =
                &mut *(*(*dynaLineTest).dyna).polyNodes.tbl.offset((*curNode).next
                                                                       as
                                                                       isize)
                    as *mut SSNode
        } else {
            if CollisionPoly_LineVsPoly(curPoly,
                                        (*(*dynaLineTest).dyna).vtxList,
                                        (*dynaLineTest).posA,
                                        (*dynaLineTest).posB,
                                        &mut polyIntersect,
                                        (*dynaLineTest).chkOneFace,
                                        (*dynaLineTest).chkDist) != 0 {
                distSq =
                    Math3D_Vec3fDistSq((*dynaLineTest).posA,
                                       &mut polyIntersect);
                if distSq < *(*dynaLineTest).distSq {
                    *(*dynaLineTest).distSq = distSq;
                    *(*dynaLineTest).posResult = polyIntersect;
                    *(*dynaLineTest).posB = polyIntersect;
                    *(*dynaLineTest).resultPoly = curPoly;
                    result = 1 as libc::c_int
                }
            }
            if (*curNode).next as libc::c_int == 0xffff as libc::c_int {
                break ;
            }
            curNode =
                &mut *(*(*dynaLineTest).dyna).polyNodes.tbl.offset((*curNode).next
                                                                       as
                                                                       isize)
                    as *mut SSNode
        }
    }
    return result;
}
/* *
 * Tests if line `posA` `posB` intersects with a dyna poly within BgActor `bgId`
 * `distSq` is the maximum squared distance to check for a collision
 * returns true if an intersection occurred, else false
 * `posB`? and `posResult` return the point of intersection
 * `outPoly` returns the poly intersected
 * `distSq` returns the squared distance of the intersection
 */
#[no_mangle]
pub unsafe extern "C" fn BgCheck_CheckLineAgainstBgActor(mut colCtx:
                                                             *mut CollisionContext,
                                                         mut xpFlags: u16_0,
                                                         mut posA: *mut Vec3f,
                                                         mut posB: *mut Vec3f,
                                                         mut posResult:
                                                             *mut Vec3f,
                                                         mut outPoly:
                                                             *mut *mut CollisionPoly,
                                                         mut distSq:
                                                             *mut f32_0,
                                                         mut bgId: s32,
                                                         mut chkDist: f32_0,
                                                         mut bccFlags: s32)
 -> s32 {
    let mut result: s32 = 0 as libc::c_int;
    let mut dynaLineTest: DynaLineTest =
        DynaLineTest{colCtx: 0 as *mut CollisionContext,
                     xpFlags: 0,
                     dyna: 0 as *mut DynaCollisionContext,
                     ssList: 0 as *mut SSList,
                     posA: 0 as *mut Vec3f,
                     posB: 0 as *mut Vec3f,
                     posResult: 0 as *mut Vec3f,
                     resultPoly: 0 as *mut *mut CollisionPoly,
                     chkOneFace: 0,
                     distSq: 0 as *mut f32_0,
                     chkDist: 0.,};
    dynaLineTest.colCtx = colCtx;
    dynaLineTest.xpFlags = xpFlags;
    dynaLineTest.dyna = &mut (*colCtx).dyna;
    dynaLineTest.posA = posA;
    dynaLineTest.posB = posB;
    dynaLineTest.posResult = posResult;
    dynaLineTest.resultPoly = outPoly;
    dynaLineTest.chkOneFace =
        (bccFlags & (1 as libc::c_int) << 3 as libc::c_int !=
             0 as libc::c_int) as libc::c_int;
    dynaLineTest.distSq = distSq;
    dynaLineTest.chkDist = chkDist;
    dynaLineTest.ssList =
        &mut (*(*colCtx).dyna.bgActors.as_mut_ptr().offset(bgId as
                                                               isize)).dynaLookup.wall;
    if bccFlags & (1 as libc::c_int) << 0 as libc::c_int != 0 {
        if BgCheck_CheckLineAgainstBgActorSSList(&mut dynaLineTest) != 0 {
            result = 1 as libc::c_int
        }
    }
    dynaLineTest.ssList =
        &mut (*(*colCtx).dyna.bgActors.as_mut_ptr().offset(bgId as
                                                               isize)).dynaLookup.floor;
    if bccFlags & (1 as libc::c_int) << 1 as libc::c_int != 0 {
        if BgCheck_CheckLineAgainstBgActorSSList(&mut dynaLineTest) != 0 {
            result = 1 as libc::c_int
        }
    }
    dynaLineTest.ssList =
        &mut (*(*colCtx).dyna.bgActors.as_mut_ptr().offset(bgId as
                                                               isize)).dynaLookup.ceiling;
    if bccFlags & (1 as libc::c_int) << 2 as libc::c_int != 0 {
        if BgCheck_CheckLineAgainstBgActorSSList(&mut dynaLineTest) != 0 {
            result = 1 as libc::c_int
        }
    }
    return result;
}
/* *
 * Tests if line from `posA` to `posB` passes through a dyna poly.
 * returns true if so, otherwise false
 * `outPoly` returns the pointer of the poly intersected.
 * `outBgId` returns the BgActor index of the poly
 */
#[no_mangle]
pub unsafe extern "C" fn BgCheck_CheckLineAgainstDyna(mut colCtx:
                                                          *mut CollisionContext,
                                                      mut xpFlags: u16_0,
                                                      mut posA: *mut Vec3f,
                                                      mut posB: *mut Vec3f,
                                                      mut posResult:
                                                          *mut Vec3f,
                                                      mut outPoly:
                                                          *mut *mut CollisionPoly,
                                                      mut distSq: *mut f32_0,
                                                      mut outBgId: *mut s32,
                                                      mut actor: *mut Actor,
                                                      mut chkDist: f32_0,
                                                      mut bccFlags: s32)
 -> s32 {
    let mut pad: s32 = 0;
    let mut i: s32 = 0;
    let mut result: s32 = 0 as libc::c_int;
    let mut line: Linef =
        Linef{a: Vec3f{x: 0., y: 0., z: 0.,},
              b: Vec3f{x: 0., y: 0., z: 0.,},};
    let mut ay: f32_0 = 0.;
    let mut by: f32_0 = 0.;
    i = 0 as libc::c_int;
    while i < 50 as libc::c_int {
        if (*colCtx).dyna.bgActorFlags[i as usize] as libc::c_int &
               1 as libc::c_int != 0 {
            if actor != (*colCtx).dyna.bgActors[i as usize].actor {
                ay = (*posA).y;
                by = (*posB).y;
                if !(ay < (*colCtx).dyna.bgActors[i as usize].minY) ||
                       !(by < (*colCtx).dyna.bgActors[i as usize].minY) {
                    if !((*colCtx).dyna.bgActors[i as usize].maxY < ay) ||
                           !((*colCtx).dyna.bgActors[i as usize].maxY < by) {
                        line.a = *posA;
                        line.b = *posB;
                        if Math3D_LineVsSph(&mut (*(*colCtx).dyna.bgActors.as_mut_ptr().offset(i
                                                                                                   as
                                                                                                   isize)).boundingSphere,
                                            &mut line) != 0 as libc::c_int {
                            if BgCheck_CheckLineAgainstBgActor(colCtx,
                                                               xpFlags, posA,
                                                               posB,
                                                               posResult,
                                                               outPoly,
                                                               distSq, i,
                                                               chkDist,
                                                               bccFlags) ==
                                   1 as libc::c_int {
                                *outBgId = i;
                                result = 1 as libc::c_int
                            }
                        }
                    }
                }
            }
        }
        i += 1
    }
    return result;
}
/* *
 * Get first dyna poly intersecting sphere `center` `radius` from list `ssList`
 * returns true if any poly intersects the sphere, else returns false
 * `outPoly` returns the pointer of the first poly found that intersects
 */
#[no_mangle]
pub unsafe extern "C" fn BgCheck_SphVsFirstDynaPolyList(mut colCtx:
                                                            *mut CollisionContext,
                                                        mut xpFlags: u16_0,
                                                        mut outPoly:
                                                            *mut *mut CollisionPoly,
                                                        mut center:
                                                            *mut Vec3f,
                                                        mut radius: f32_0,
                                                        mut ssList:
                                                            *mut SSList)
 -> s32 {
    let mut curPoly: *mut CollisionPoly = 0 as *mut CollisionPoly;
    let mut dyna: *mut DynaCollisionContext = 0 as *mut DynaCollisionContext;
    let mut curNode: *mut SSNode = 0 as *mut SSNode;
    let mut curPolyId: s32 = 0;
    if (*ssList).head as libc::c_int == 0xffff as libc::c_int {
        return 0 as libc::c_int
    }
    dyna = &mut (*colCtx).dyna;
    curNode =
        &mut *(*dyna).polyNodes.tbl.offset((*ssList).head as isize) as
            *mut SSNode;
    loop  {
        curPolyId = (*curNode).polyId as s32;
        curPoly =
            &mut *(*dyna).polyList.offset(curPolyId as isize) as
                *mut CollisionPoly;
        if (*curPoly).c2rust_unnamed.c2rust_unnamed.flags_vIA as libc::c_int &
               (xpFlags as libc::c_int & 7 as libc::c_int) <<
                   13 as libc::c_int != 0 {
            if (*curNode).next as libc::c_int == 0xffff as libc::c_int {
                break ;
            }
            curNode =
                &mut *(*dyna).polyNodes.tbl.offset((*curNode).next as isize)
                    as *mut SSNode
        } else {
            if CollisionPoly_SphVsPoly(curPoly, (*dyna).vtxList, center,
                                       radius) != 0 {
                *outPoly = curPoly;
                return 1 as libc::c_int
            }
            if (*curNode).next as libc::c_int == 0xffff as libc::c_int {
                break ;
            }
            curNode =
                &mut *(*dyna).polyNodes.tbl.offset((*curNode).next as isize)
                    as *mut SSNode
        }
    }
    return 0 as libc::c_int;
}
/* *
 * Get first dyna poly intersecting sphere `center` `radius` from BgActor `bgId`
 * returns true if any poly intersects the sphere, else false
 * `outPoly` returns the pointer of the first poly found that intersects
 */
#[no_mangle]
pub unsafe extern "C" fn BgCheck_SphVsFirstDynaPolyInBgActor(mut colCtx:
                                                                 *mut CollisionContext,
                                                             mut xpFlags:
                                                                 u16_0,
                                                             mut outPoly:
                                                                 *mut *mut CollisionPoly,
                                                             mut center:
                                                                 *mut Vec3f,
                                                             mut radius:
                                                                 f32_0,
                                                             mut bgId: s32,
                                                             mut bciFlags:
                                                                 u16_0)
 -> s32 {
    if bciFlags as libc::c_int & (1 as libc::c_int) << 0 as libc::c_int ==
           0 as libc::c_int {
        if BgCheck_SphVsFirstDynaPolyList(colCtx, xpFlags, outPoly, center,
                                          radius,
                                          &mut (*(*colCtx).dyna.bgActors.as_mut_ptr().offset(bgId
                                                                                                 as
                                                                                                 isize)).dynaLookup.ceiling)
               != 0 {
            return 1 as libc::c_int
        }
    }
    if bciFlags as libc::c_int & (1 as libc::c_int) << 1 as libc::c_int ==
           0 as libc::c_int {
        if BgCheck_SphVsFirstDynaPolyList(colCtx, xpFlags, outPoly, center,
                                          radius,
                                          &mut (*(*colCtx).dyna.bgActors.as_mut_ptr().offset(bgId
                                                                                                 as
                                                                                                 isize)).dynaLookup.wall)
               != 0 {
            return 1 as libc::c_int
        }
    }
    if bciFlags as libc::c_int & (1 as libc::c_int) << 2 as libc::c_int ==
           0 as libc::c_int {
        if BgCheck_SphVsFirstDynaPolyList(colCtx, xpFlags, outPoly, center,
                                          radius,
                                          &mut (*(*colCtx).dyna.bgActors.as_mut_ptr().offset(bgId
                                                                                                 as
                                                                                                 isize)).dynaLookup.floor)
               != 0 {
            return 1 as libc::c_int
        }
    }
    return 0 as libc::c_int;
}
/* *
 * Gets first dyna poly intersecting sphere `center` `radius`
 * returns true if poly detected, else false
 * `outPoly` returns the first intersecting poly, while `outBgId` returns the BgActor index of that poly
 */
#[no_mangle]
pub unsafe extern "C" fn BgCheck_SphVsFirstDynaPoly(mut colCtx:
                                                        *mut CollisionContext,
                                                    mut xpFlags: u16_0,
                                                    mut outPoly:
                                                        *mut *mut CollisionPoly,
                                                    mut outBgId: *mut s32,
                                                    mut center: *mut Vec3f,
                                                    mut radius: f32_0,
                                                    mut actor: *mut Actor,
                                                    mut bciFlags: u16_0)
 -> s32 {
    let mut i: s32 = 0 as libc::c_int;
    let mut testSphere: Sphere16 =
        Sphere16{center: Vec3s{x: 0, y: 0, z: 0,}, radius: 0,};
    i = 0 as libc::c_int;
    while i < 50 as libc::c_int {
        if !((*colCtx).dyna.bgActorFlags[i as usize] as libc::c_int &
                 1 as libc::c_int == 0) {
            if !((*colCtx).dyna.bgActors[i as usize].actor == actor) {
                testSphere.center.x = (*center).x as s16;
                testSphere.center.y = (*center).y as s16;
                testSphere.center.z = (*center).z as s16;
                testSphere.radius = radius as s16;
                if !(Math3D_SphVsSph(&mut testSphere,
                                     &mut (*(*colCtx).dyna.bgActors.as_mut_ptr().offset(i
                                                                                            as
                                                                                            isize)).boundingSphere)
                         == 0) {
                    if BgCheck_SphVsFirstDynaPolyInBgActor(colCtx, xpFlags,
                                                           outPoly, center,
                                                           radius, i,
                                                           bciFlags) != 0 {
                        return 1 as libc::c_int
                    }
                }
            }
        }
        i += 1
    }
    return 0 as libc::c_int;
}
/* *
 * SEGMENTED_TO_VIRTUAL CollisionHeader members
 */
#[no_mangle]
pub unsafe extern "C" fn CollisionHeader_SegmentedToVirtual(mut colHeader:
                                                                *mut CollisionHeader) {
    (*colHeader).vtxList =
        gSegments[(((*colHeader).vtxList as u32_0) << 4 as libc::c_int >>
                       28 as libc::c_int) as
                      usize].wrapping_add((*colHeader).vtxList as u32_0 &
                                              0xffffff as libc::c_int as
                                                  libc::c_uint).wrapping_add(0x80000000
                                                                                 as
                                                                                 libc::c_uint)
            as *mut libc::c_void as *mut Vec3s;
    (*colHeader).polyList =
        gSegments[(((*colHeader).polyList as u32_0) << 4 as libc::c_int >>
                       28 as libc::c_int) as
                      usize].wrapping_add((*colHeader).polyList as u32_0 &
                                              0xffffff as libc::c_int as
                                                  libc::c_uint).wrapping_add(0x80000000
                                                                                 as
                                                                                 libc::c_uint)
            as *mut libc::c_void as *mut CollisionPoly;
    (*colHeader).surfaceTypeList =
        gSegments[(((*colHeader).surfaceTypeList as u32_0) << 4 as libc::c_int
                       >> 28 as libc::c_int) as
                      usize].wrapping_add((*colHeader).surfaceTypeList as
                                              u32_0 &
                                              0xffffff as libc::c_int as
                                                  libc::c_uint).wrapping_add(0x80000000
                                                                                 as
                                                                                 libc::c_uint)
            as *mut libc::c_void as *mut SurfaceType;
    (*colHeader).cameraDataList =
        gSegments[(((*colHeader).cameraDataList as u32_0) << 4 as libc::c_int
                       >> 28 as libc::c_int) as
                      usize].wrapping_add((*colHeader).cameraDataList as u32_0
                                              &
                                              0xffffff as libc::c_int as
                                                  libc::c_uint).wrapping_add(0x80000000
                                                                                 as
                                                                                 libc::c_uint)
            as *mut libc::c_void as *mut CamData;
    (*colHeader).waterBoxes =
        gSegments[(((*colHeader).waterBoxes as u32_0) << 4 as libc::c_int >>
                       28 as libc::c_int) as
                      usize].wrapping_add((*colHeader).waterBoxes as u32_0 &
                                              0xffffff as libc::c_int as
                                                  libc::c_uint).wrapping_add(0x80000000
                                                                                 as
                                                                                 libc::c_uint)
            as *mut libc::c_void as *mut WaterBox;
}
/* *
 * Convert CollisionHeader Segmented to Virtual addressing
 */
#[no_mangle]
pub unsafe extern "C" fn CollisionHeader_GetVirtual(mut colHeader:
                                                        *mut libc::c_void,
                                                    mut dest:
                                                        *mut *mut CollisionHeader) {
    *dest =
        gSegments[((colHeader as u32_0) << 4 as libc::c_int >>
                       28 as libc::c_int) as
                      usize].wrapping_add(colHeader as u32_0 &
                                              0xffffff as libc::c_int as
                                                  libc::c_uint).wrapping_add(0x80000000
                                                                                 as
                                                                                 libc::c_uint)
            as *mut libc::c_void as *mut CollisionHeader;
    CollisionHeader_SegmentedToVirtual(*dest);
}
/* *
 * SEGMENT_TO_VIRTUAL all active BgActor CollisionHeaders
 */
#[no_mangle]
pub unsafe extern "C" fn func_800418D0(mut colCtx: *mut CollisionContext,
                                       mut globalCtx: *mut GlobalContext) {
    let mut dyna: *mut DynaCollisionContext = &mut (*colCtx).dyna;
    let mut i: s32 = 0;
    let mut flag: u16_0 = 0;
    i = 0 as libc::c_int;
    while i < 50 as libc::c_int {
        flag = (*dyna).bgActorFlags[i as usize];
        if flag as libc::c_int & 1 as libc::c_int != 0 &&
               flag as libc::c_int & 2 as libc::c_int == 0 {
            Actor_SetObjectDependency(globalCtx,
                                      (*dyna).bgActors[i as usize].actor);
            CollisionHeader_SegmentedToVirtual((*dyna).bgActors[i as
                                                                    usize].colHeader);
        }
        i += 1
    };
}
/* *
 * Reset SSNodeList polyCheckTbl
 */
#[no_mangle]
pub unsafe extern "C" fn BgCheck_ResetPolyCheckTbl(mut nodeList:
                                                       *mut SSNodeList,
                                                   mut numPolys: s32) {
    let mut t: *mut u8_0 = 0 as *mut u8_0;
    t = (*nodeList).polyCheckTbl;
    while t < (*nodeList).polyCheckTbl.offset(numPolys as isize) {
        *t = 0 as libc::c_int as u8_0;
        t = t.offset(1)
    };
}
/* *
 * Get SurfaceType property set
 */
#[no_mangle]
pub unsafe extern "C" fn SurfaceType_GetData(mut colCtx:
                                                 *mut CollisionContext,
                                             mut poly: *mut CollisionPoly,
                                             mut bgId: s32, mut dataIdx: s32)
 -> u32_0 {
    let mut colHeader: *mut CollisionHeader = 0 as *mut CollisionHeader;
    let mut surfaceTypes: *mut SurfaceType = 0 as *mut SurfaceType;
    colHeader = BgCheck_GetCollisionHeader(colCtx, bgId);
    if colHeader.is_null() || poly.is_null() {
        return 0 as libc::c_int as u32_0
    }
    surfaceTypes = (*colHeader).surfaceTypeList;
    if surfaceTypes ==
           gSegments[0 as libc::c_int as
                         usize].wrapping_add(0x80000000 as libc::c_uint) as
               *mut libc::c_void as *mut SurfaceType {
        return 0 as libc::c_int as u32_0
    }
    return (*surfaceTypes.offset((*poly).type_0 as
                                     isize)).data[dataIdx as usize];
}
/* *
 * SurfaceType return CamData Index
 */
#[no_mangle]
pub unsafe extern "C" fn SurfaceType_GetCamDataIndex(mut colCtx:
                                                         *mut CollisionContext,
                                                     mut poly:
                                                         *mut CollisionPoly,
                                                     mut bgId: s32) -> u32_0 {
    return SurfaceType_GetData(colCtx, poly, bgId, 0 as libc::c_int) &
               0xff as libc::c_int as libc::c_uint;
}
/* *
 * CamData return cameraSType
 */
#[no_mangle]
pub unsafe extern "C" fn func_80041A4C(mut colCtx: *mut CollisionContext,
                                       mut camId: u32_0, mut bgId: s32)
 -> u16_0 {
    let mut result: u16_0 = 0;
    let mut colHeader: *mut CollisionHeader = 0 as *mut CollisionHeader;
    let mut camData: *mut CamData = 0 as *mut CamData;
    colHeader = BgCheck_GetCollisionHeader(colCtx, bgId);
    if colHeader.is_null() { return 0 as libc::c_int as u16_0 }
    camData = (*colHeader).cameraDataList;
    result = (*camData.offset(camId as isize)).cameraSType;
    return result;
}
/* *
 * SurfaceType return cameraSType
 */
#[no_mangle]
pub unsafe extern "C" fn SurfaceType_GetCameraSType(mut colCtx:
                                                        *mut CollisionContext,
                                                    mut poly:
                                                        *mut CollisionPoly,
                                                    mut bgId: s32) -> u16_0 {
    let mut colHeader: *mut CollisionHeader =
        BgCheck_GetCollisionHeader(colCtx, bgId);
    let mut camData: *mut CamData = 0 as *mut CamData;
    let mut surfaceTypes: *mut SurfaceType = 0 as *mut SurfaceType;
    if colHeader.is_null() { return 0 as libc::c_int as u16_0 }
    camData = (*colHeader).cameraDataList;
    if camData ==
           gSegments[0 as libc::c_int as
                         usize].wrapping_add(0x80000000 as libc::c_uint) as
               *mut libc::c_void as *mut CamData {
        return 0 as libc::c_int as u16_0
    }
    surfaceTypes = (*colHeader).surfaceTypeList;
    if surfaceTypes ==
           gSegments[0 as libc::c_int as
                         usize].wrapping_add(0x80000000 as libc::c_uint) as
               *mut libc::c_void as *mut SurfaceType {
        return 0 as libc::c_int as u16_0
    }
    return func_80041A4C(colCtx,
                         SurfaceType_GetCamDataIndex(colCtx, poly, bgId),
                         bgId);
}
/* *
 * CamData Get number of cameras
 */
#[no_mangle]
pub unsafe extern "C" fn func_80041B24(mut colCtx: *mut CollisionContext,
                                       mut camId: u32_0, mut bgId: s32)
 -> u16_0 {
    let mut colHeader: *mut CollisionHeader =
        BgCheck_GetCollisionHeader(colCtx, bgId);
    let mut camData: *mut CamData = 0 as *mut CamData;
    if colHeader.is_null() { return 0 as libc::c_int as u16_0 }
    camData = (*colHeader).cameraDataList;
    if camData ==
           gSegments[0 as libc::c_int as
                         usize].wrapping_add(0x80000000 as libc::c_uint) as
               *mut libc::c_void as *mut CamData {
        return 0 as libc::c_int as u16_0
    }
    return (*camData.offset(camId as isize)).numCameras as u16_0;
}
/* *
 * SurfaceType Get number of cameras
 */
#[no_mangle]
pub unsafe extern "C" fn SurfaceType_GetNumCameras(mut colCtx:
                                                       *mut CollisionContext,
                                                   mut poly:
                                                       *mut CollisionPoly,
                                                   mut bgId: s32) -> u16_0 {
    let mut colHeader: *mut CollisionHeader =
        BgCheck_GetCollisionHeader(colCtx, bgId);
    let mut camData: *mut CamData = 0 as *mut CamData;
    let mut surfaceTypes: *mut SurfaceType = 0 as *mut SurfaceType;
    if colHeader.is_null() { return 0 as libc::c_int as u16_0 }
    camData = (*colHeader).cameraDataList;
    if camData ==
           gSegments[0 as libc::c_int as
                         usize].wrapping_add(0x80000000 as libc::c_uint) as
               *mut libc::c_void as *mut CamData {
        return 0 as libc::c_int as u16_0
    }
    surfaceTypes = (*colHeader).surfaceTypeList;
    if surfaceTypes ==
           gSegments[0 as libc::c_int as
                         usize].wrapping_add(0x80000000 as libc::c_uint) as
               *mut libc::c_void as *mut SurfaceType {
        return 0 as libc::c_int as u16_0
    }
    return func_80041B24(colCtx,
                         SurfaceType_GetCamDataIndex(colCtx, poly, bgId),
                         bgId);
}
/* *
 * CamData Get camPosData
 */
#[no_mangle]
pub unsafe extern "C" fn func_80041C10(mut colCtx: *mut CollisionContext,
                                       mut camId: s32, mut bgId: s32)
 -> *mut Vec3s {
    let mut colHeader: *mut CollisionHeader =
        BgCheck_GetCollisionHeader(colCtx, bgId);
    let mut cameraDataList: *mut CamData = 0 as *mut CamData;
    if colHeader.is_null() { return 0 as *mut Vec3s }
    cameraDataList = (*colHeader).cameraDataList;
    if cameraDataList ==
           gSegments[0 as libc::c_int as
                         usize].wrapping_add(0x80000000 as libc::c_uint) as
               *mut libc::c_void as *mut CamData {
        return 0 as *mut Vec3s
    }
    return gSegments[(((*cameraDataList.offset(camId as isize)).camPosData as
                           u32_0) << 4 as libc::c_int >> 28 as libc::c_int) as
                         usize].wrapping_add((*cameraDataList.offset(camId as
                                                                         isize)).camPosData
                                                 as u32_0 &
                                                 0xffffff as libc::c_int as
                                                     libc::c_uint).wrapping_add(0x80000000
                                                                                    as
                                                                                    libc::c_uint)
               as *mut libc::c_void as *mut Vec3s;
}
/* *
 * SurfaceType Get camPosData
 */
#[no_mangle]
pub unsafe extern "C" fn SurfaceType_GetCamPosData(mut colCtx:
                                                       *mut CollisionContext,
                                                   mut poly:
                                                       *mut CollisionPoly,
                                                   mut bgId: s32)
 -> *mut Vec3s {
    let mut colHeader: *mut CollisionHeader =
        BgCheck_GetCollisionHeader(colCtx, bgId);
    let mut camData: *mut CamData = 0 as *mut CamData;
    let mut surfaceTypes: *mut SurfaceType = 0 as *mut SurfaceType;
    if colHeader.is_null() { return 0 as *mut Vec3s }
    camData = (*colHeader).cameraDataList;
    if camData ==
           gSegments[0 as libc::c_int as
                         usize].wrapping_add(0x80000000 as libc::c_uint) as
               *mut libc::c_void as *mut CamData {
        return 0 as *mut Vec3s
    }
    surfaceTypes = (*colHeader).surfaceTypeList;
    if surfaceTypes ==
           gSegments[0 as libc::c_int as
                         usize].wrapping_add(0x80000000 as libc::c_uint) as
               *mut libc::c_void as *mut SurfaceType {
        return 0 as *mut Vec3s
    }
    return func_80041C10(colCtx,
                         SurfaceType_GetCamDataIndex(colCtx, poly, bgId) as
                             s32, bgId);
}
/* *
 * SurfaceType Get Scene Exit Index
 */
#[no_mangle]
pub unsafe extern "C" fn SurfaceType_GetSceneExitIndex(mut colCtx:
                                                           *mut CollisionContext,
                                                       mut poly:
                                                           *mut CollisionPoly,
                                                       mut bgId: s32)
 -> u32_0 {
    return SurfaceType_GetData(colCtx, poly, bgId, 0 as libc::c_int) >>
               8 as libc::c_int & 0x1f as libc::c_int as libc::c_uint;
}
/* *
 * SurfaceType Get ? Property (& 0x0003 E000)
 */
#[no_mangle]
pub unsafe extern "C" fn func_80041D4C(mut colCtx: *mut CollisionContext,
                                       mut poly: *mut CollisionPoly,
                                       mut bgId: s32) -> u32_0 {
    return SurfaceType_GetData(colCtx, poly, bgId, 0 as libc::c_int) >>
               13 as libc::c_int & 0x1f as libc::c_int as libc::c_uint;
}
/* *
 * SurfaceType Get ? Property (& 0x001C 0000)
 */
#[no_mangle]
pub unsafe extern "C" fn func_80041D70(mut colCtx: *mut CollisionContext,
                                       mut poly: *mut CollisionPoly,
                                       mut bgId: s32) -> u32_0 {
    return SurfaceType_GetData(colCtx, poly, bgId, 0 as libc::c_int) >>
               18 as libc::c_int & 7 as libc::c_int as libc::c_uint;
}
/* *
 * SurfaceType Get Wall Property (Internal)
 */
#[no_mangle]
pub unsafe extern "C" fn func_80041D94(mut colCtx: *mut CollisionContext,
                                       mut poly: *mut CollisionPoly,
                                       mut bgId: s32) -> u32_0 {
    return SurfaceType_GetData(colCtx, poly, bgId, 0 as libc::c_int) >>
               21 as libc::c_int & 0x1f as libc::c_int as libc::c_uint;
}
/* *
 * SurfaceType Get Wall Flags
 */
#[no_mangle]
pub unsafe extern "C" fn func_80041DB8(mut colCtx: *mut CollisionContext,
                                       mut poly: *mut CollisionPoly,
                                       mut bgId: s32) -> s32 {
    return D_80119D90[func_80041D94(colCtx, poly, bgId) as usize];
}
/* *
 * SurfaceType Is Wall Flag (1 << 0) Set
 */
#[no_mangle]
pub unsafe extern "C" fn func_80041DE4(mut colCtx: *mut CollisionContext,
                                       mut poly: *mut CollisionPoly,
                                       mut bgId: s32) -> s32 {
    return if func_80041DB8(colCtx, poly, bgId) & 1 as libc::c_int != 0 {
               1 as libc::c_int
           } else { 0 as libc::c_int };
}
/* *
 * SurfaceType Is Wall Flag (1 << 1) Set
 */
#[no_mangle]
pub unsafe extern "C" fn func_80041E18(mut colCtx: *mut CollisionContext,
                                       mut poly: *mut CollisionPoly,
                                       mut bgId: s32) -> s32 {
    return if func_80041DB8(colCtx, poly, bgId) & 2 as libc::c_int != 0 {
               1 as libc::c_int
           } else { 0 as libc::c_int };
}
/* *
 * SurfaceType Is Wall Flag (1 << 2) Set
 */
#[no_mangle]
pub unsafe extern "C" fn func_80041E4C(mut colCtx: *mut CollisionContext,
                                       mut poly: *mut CollisionPoly,
                                       mut bgId: s32) -> s32 {
    return if func_80041DB8(colCtx, poly, bgId) & 4 as libc::c_int != 0 {
               1 as libc::c_int
           } else { 0 as libc::c_int };
}
/* *
 * unused
 */
#[no_mangle]
pub unsafe extern "C" fn func_80041E80(mut colCtx: *mut CollisionContext,
                                       mut poly: *mut CollisionPoly,
                                       mut bgId: s32) -> u32_0 {
    return SurfaceType_GetData(colCtx, poly, bgId, 0 as libc::c_int) >>
               26 as libc::c_int & 0xf as libc::c_int as libc::c_uint;
}
/* *
 * SurfaceType Get Floor Property
 */
#[no_mangle]
pub unsafe extern "C" fn func_80041EA4(mut colCtx: *mut CollisionContext,
                                       mut poly: *mut CollisionPoly,
                                       mut bgId: s32) -> u32_0 {
    return SurfaceType_GetData(colCtx, poly, bgId, 0 as libc::c_int) >>
               26 as libc::c_int & 0xf as libc::c_int as libc::c_uint;
}
/* *
 * SurfaceType Is Floor Minus 1
 */
#[no_mangle]
pub unsafe extern "C" fn func_80041EC8(mut colCtx: *mut CollisionContext,
                                       mut poly: *mut CollisionPoly,
                                       mut bgId: s32) -> u32_0 {
    return SurfaceType_GetData(colCtx, poly, bgId, 0 as libc::c_int) >>
               30 as libc::c_int & 1 as libc::c_int as libc::c_uint;
}
/* *
 * SurfaceType Is Horse Blocked
 */
#[no_mangle]
pub unsafe extern "C" fn SurfaceType_IsHorseBlocked(mut colCtx:
                                                        *mut CollisionContext,
                                                    mut poly:
                                                        *mut CollisionPoly,
                                                    mut bgId: s32) -> u32_0 {
    return SurfaceType_GetData(colCtx, poly, bgId, 0 as libc::c_int) >>
               31 as libc::c_int & 1 as libc::c_int as libc::c_uint;
}
#[no_mangle]
pub unsafe extern "C" fn func_80041F10(mut colCtx: *mut CollisionContext,
                                       mut poly: *mut CollisionPoly,
                                       mut bgId: s32) -> u32_0 {
    return SurfaceType_GetData(colCtx, poly, bgId, 1 as libc::c_int) &
               0xf as libc::c_int as libc::c_uint;
}
/* *
 * SurfaceType Get Poly Sfx
 */
#[no_mangle]
pub unsafe extern "C" fn SurfaceType_GetSfx(mut colCtx: *mut CollisionContext,
                                            mut poly: *mut CollisionPoly,
                                            mut bgId: s32) -> u16_0 {
    let mut id: s32 = func_80041F10(colCtx, poly, bgId) as s32;
    if id < 0 as libc::c_int || id > 13 as libc::c_int {
        return (0x800 as libc::c_int - 0x800 as libc::c_int) as u16_0
    }
    return D_80119E10[id as usize];
}
/* *
 * SurfaceType get terrain slope surface
 */
#[no_mangle]
pub unsafe extern "C" fn SurfaceType_GetSlope(mut colCtx:
                                                  *mut CollisionContext,
                                              mut poly: *mut CollisionPoly,
                                              mut bgId: s32) -> u32_0 {
    return SurfaceType_GetData(colCtx, poly, bgId, 1 as libc::c_int) >>
               4 as libc::c_int & 3 as libc::c_int as libc::c_uint;
}
/* *
 * SurfaceType get surface lighting setting
 */
#[no_mangle]
pub unsafe extern "C" fn SurfaceType_GetLightSettingIndex(mut colCtx:
                                                              *mut CollisionContext,
                                                          mut poly:
                                                              *mut CollisionPoly,
                                                          mut bgId: s32)
 -> u32_0 {
    return SurfaceType_GetData(colCtx, poly, bgId, 1 as libc::c_int) >>
               6 as libc::c_int & 0x1f as libc::c_int as libc::c_uint;
}
/* *
 * SurfaceType get echo
 */
#[no_mangle]
pub unsafe extern "C" fn SurfaceType_GetEcho(mut colCtx:
                                                 *mut CollisionContext,
                                             mut poly: *mut CollisionPoly,
                                             mut bgId: s32) -> u32_0 {
    return SurfaceType_GetData(colCtx, poly, bgId, 1 as libc::c_int) >>
               11 as libc::c_int & 0x3f as libc::c_int as libc::c_uint;
}
/* *
 * SurfaceType Is Hookshot Surface
 */
#[no_mangle]
pub unsafe extern "C" fn SurfaceType_IsHookshotSurface(mut colCtx:
                                                           *mut CollisionContext,
                                                       mut poly:
                                                           *mut CollisionPoly,
                                                       mut bgId: s32)
 -> u32_0 {
    return SurfaceType_GetData(colCtx, poly, bgId, 1 as libc::c_int) >>
               17 as libc::c_int & 1 as libc::c_int as libc::c_uint;
}
/* *
 * CollisionPoly is ignored by entities
 * Returns true if poly is ignored by entities, else false
 */
#[no_mangle]
pub unsafe extern "C" fn SurfaceType_IsIgnoredByEntities(mut colCtx:
                                                             *mut CollisionContext,
                                                         mut poly:
                                                             *mut CollisionPoly,
                                                         mut bgId: s32)
 -> s32 {
    let mut flags: u32_0 = 0;
    if BgCheck_GetCollisionHeader(colCtx, bgId).is_null() {
        return 1 as libc::c_int
    }
    flags =
        ((*poly).c2rust_unnamed.c2rust_unnamed.flags_vIA as libc::c_int &
             0x4000 as libc::c_int) as u32_0;
    return (flags != 0) as libc::c_int;
}
/* *
 * CollisionPoly is ignored by projectiles
 * Returns true if poly is ignored by projectiles, else false
 */
#[no_mangle]
pub unsafe extern "C" fn SurfaceType_IsIgnoredByProjectiles(mut colCtx:
                                                                *mut CollisionContext,
                                                            mut poly:
                                                                *mut CollisionPoly,
                                                            mut bgId: s32)
 -> s32 {
    let mut flags: u32_0 = 0;
    if BgCheck_GetCollisionHeader(colCtx, bgId).is_null() {
        return 1 as libc::c_int
    }
    flags =
        ((*poly).c2rust_unnamed.c2rust_unnamed.flags_vIA as libc::c_int &
             0x8000 as libc::c_int) as u32_0;
    return (flags != 0) as libc::c_int;
}
/* *
 * CollisionPoly is conveyor enabled
 * Returns true if `poly` is a conveyor surface, else false
 */
#[no_mangle]
pub unsafe extern "C" fn SurfaceType_IsConveyor(mut colCtx:
                                                    *mut CollisionContext,
                                                mut poly: *mut CollisionPoly,
                                                mut bgId: s32) -> s32 {
    let mut flags: u32_0 = 0;
    if BgCheck_GetCollisionHeader(colCtx, bgId).is_null() {
        return 1 as libc::c_int
    }
    flags =
        ((*poly).c2rust_unnamed.c2rust_unnamed.flags_vIB as libc::c_int &
             0x2000 as libc::c_int) as u32_0;
    return (flags != 0) as libc::c_int;
}
/* *
 * SurfaceType Get Conveyor Surface Speed
 */
#[no_mangle]
pub unsafe extern "C" fn SurfaceType_GetConveyorSpeed(mut colCtx:
                                                          *mut CollisionContext,
                                                      mut poly:
                                                          *mut CollisionPoly,
                                                      mut bgId: s32)
 -> u32_0 {
    return SurfaceType_GetData(colCtx, poly, bgId, 1 as libc::c_int) >>
               18 as libc::c_int & 7 as libc::c_int as libc::c_uint;
}
/* *
 * SurfaceType Get Conveyor Direction
 * returns a value between 0-63, representing 360 / 64 degrees of rotation
 */
#[no_mangle]
pub unsafe extern "C" fn SurfaceType_GetConveyorDirection(mut colCtx:
                                                              *mut CollisionContext,
                                                          mut poly:
                                                              *mut CollisionPoly,
                                                          mut bgId: s32)
 -> u32_0 {
    return SurfaceType_GetData(colCtx, poly, bgId, 1 as libc::c_int) >>
               21 as libc::c_int & 0x3f as libc::c_int as libc::c_uint;
}
/* *
 * SurfaceType is Wall Damage
 */
#[no_mangle]
pub unsafe extern "C" fn SurfaceType_IsWallDamage(mut colCtx:
                                                      *mut CollisionContext,
                                                  mut poly:
                                                      *mut CollisionPoly,
                                                  mut bgId: s32) -> u32_0 {
    return if SurfaceType_GetData(colCtx, poly, bgId, 1 as libc::c_int) &
                  0x8000000 as libc::c_int as libc::c_uint != 0 {
               1 as libc::c_int
           } else { 0 as libc::c_int } as u32_0;
}
/* *
 * Zora's Domain WaterBox in King Zora's Room
 */
#[no_mangle]
pub static mut zdWaterBox: WaterBox =
    {
        let mut init =
            WaterBox{xMin: -(348 as libc::c_int) as s16,
                     ySurface: 877 as libc::c_int as s16,
                     zMin: -(1746 as libc::c_int) as s16,
                     xLength: 553 as libc::c_int as s16,
                     zLength: 780 as libc::c_int as s16,
                     properties: 0x2104 as libc::c_int as u32_0,};
        init
    };
/* *
 * WaterBox's effective bounding box
 */
#[no_mangle]
pub static mut zdWaterBoxMinX: f32_0 = -348.0f32;
#[no_mangle]
pub static mut zdWaterBoxMinY: f32_0 = 777.0f32;
#[no_mangle]
pub static mut zdWaterBoxMinZ: f32_0 = -1746.0f32;
#[no_mangle]
pub static mut zdWaterBoxMaxX: f32_0 = 205.0f32;
#[no_mangle]
pub static mut zdWaterBoxMaxY: f32_0 = 977.0f32;
#[no_mangle]
pub static mut zdWaterBoxMaxZ: f32_0 = -967.0f32;
/* *
 * Public. Get the water surface at point (`x`, `ySurface`, `z`). `ySurface` doubles as position y input
 * returns true if point is within the xz boundaries of an active water box, else false
 * `ySurface` returns the water box's surface, while `outWaterBox` returns a pointer to the WaterBox
 */
#[no_mangle]
pub unsafe extern "C" fn WaterBox_GetSurface1(mut globalCtx:
                                                  *mut GlobalContext,
                                              mut colCtx:
                                                  *mut CollisionContext,
                                              mut x: f32_0, mut z: f32_0,
                                              mut ySurface: *mut f32_0,
                                              mut outWaterBox:
                                                  *mut *mut WaterBox) -> s32 {
    if (*globalCtx).sceneNum as libc::c_int == SCENE_SPOT07 as libc::c_int {
        if zdWaterBoxMinX < x && x < zdWaterBoxMaxX &&
               zdWaterBoxMinY < *ySurface && *ySurface < zdWaterBoxMaxY &&
               zdWaterBoxMinZ < z && z < zdWaterBoxMaxZ {
            *outWaterBox = &mut zdWaterBox;
            *ySurface = zdWaterBox.ySurface as f32_0;
            return 1 as libc::c_int
        }
    }
    return WaterBox_GetSurfaceImpl(globalCtx, colCtx, x, z, ySurface,
                                   outWaterBox);
}
/* *
 * Internal. Get the water surface at point (`x`, `ySurface`, `z`). `ySurface` doubles as position y input
 * returns true if point is within the xz boundaries of an active water box, else false
 * `ySurface` returns the water box's surface, while `outWaterBox` returns a pointer to the WaterBox
 */
#[no_mangle]
pub unsafe extern "C" fn WaterBox_GetSurfaceImpl(mut globalCtx:
                                                     *mut GlobalContext,
                                                 mut colCtx:
                                                     *mut CollisionContext,
                                                 mut x: f32_0, mut z: f32_0,
                                                 mut ySurface: *mut f32_0,
                                                 mut outWaterBox:
                                                     *mut *mut WaterBox)
 -> s32 {
    let mut colHeader: *mut CollisionHeader = (*colCtx).colHeader;
    let mut room: u32_0 = 0;
    let mut curWaterBox: *mut WaterBox = 0 as *mut WaterBox;
    if (*colHeader).numWaterBoxes as libc::c_int == 0 as libc::c_int ||
           (*colHeader).waterBoxes ==
               gSegments[0 as libc::c_int as
                             usize].wrapping_add(0x80000000 as libc::c_uint)
                   as *mut libc::c_void as *mut WaterBox {
        return 0 as libc::c_int
    }
    curWaterBox = (*colHeader).waterBoxes;
    while curWaterBox <
              (*colHeader).waterBoxes.offset((*colHeader).numWaterBoxes as
                                                 libc::c_int as isize) {
        room =
            (*curWaterBox).properties >> 13 as libc::c_int &
                0x3f as libc::c_int as libc::c_uint;
        if room == (*globalCtx).roomCtx.curRoom.num as u32_0 ||
               room == 0x3f as libc::c_int as libc::c_uint {
            if (*curWaterBox).properties &
                   0x80000 as libc::c_int as libc::c_uint ==
                   0 as libc::c_int as libc::c_uint {
                if ((*curWaterBox).xMin as libc::c_int as libc::c_float) < x
                       &&
                       x <
                           ((*curWaterBox).xMin as libc::c_int +
                                (*curWaterBox).xLength as libc::c_int) as
                               libc::c_float {
                    if ((*curWaterBox).zMin as libc::c_int as libc::c_float) <
                           z &&
                           z <
                               ((*curWaterBox).zMin as libc::c_int +
                                    (*curWaterBox).zLength as libc::c_int) as
                                   libc::c_float {
                        *outWaterBox = curWaterBox;
                        *ySurface = (*curWaterBox).ySurface as f32_0;
                        return 1 as libc::c_int
                    }
                }
            }
        }
        curWaterBox = curWaterBox.offset(1)
    }
    return 0 as libc::c_int;
}
/* *
 * Gets the first active WaterBox at `pos` where WaterBox.properties & 0x80000 == 0
 * `surfaceChkDist` is the absolute y distance from the water surface to check
 * returns the index of the waterbox found, or -1 if no waterbox is found
 * `outWaterBox` returns the pointer to the waterbox found, or NULL if none is found
 */
#[no_mangle]
pub unsafe extern "C" fn WaterBox_GetSurface2(mut globalCtx:
                                                  *mut GlobalContext,
                                              mut colCtx:
                                                  *mut CollisionContext,
                                              mut pos: *mut Vec3f,
                                              mut surfaceChkDist: f32_0,
                                              mut outWaterBox:
                                                  *mut *mut WaterBox) -> s32 {
    let mut colHeader: *mut CollisionHeader =
        (*colCtx).colHeader; // unused, needed for matching
    let mut room: s32 = 0;
    let mut i: s32 = 0;
    let mut waterBox: *mut WaterBox = 0 as *mut WaterBox;
    let mut waterBoxList: *mut WaterBox = (*colHeader).waterBoxes;
    if (*colHeader).numWaterBoxes as libc::c_int == 0 as libc::c_int ||
           (*colHeader).waterBoxes ==
               gSegments[0 as libc::c_int as
                             usize].wrapping_add(0x80000000 as libc::c_uint)
                   as *mut libc::c_void as *mut WaterBox {
        *outWaterBox = 0 as *mut WaterBox;
        return -(1 as libc::c_int)
    }
    i = 0 as libc::c_int;
    while i < (*colHeader).numWaterBoxes as libc::c_int {
        waterBox =
            &mut *(*colHeader).waterBoxes.offset(i as isize) as *mut WaterBox;
        room =
            ((*waterBox).properties >> 13 as libc::c_int &
                 0x3f as libc::c_int as libc::c_uint) as s32;
        if room == (*globalCtx).roomCtx.curRoom.num as libc::c_int ||
               room == 0x3f as libc::c_int {
            if !((*waterBox).properties &
                     0x80000 as libc::c_int as libc::c_uint != 0) {
                if ((*waterBox).xMin as libc::c_int as libc::c_float) <
                       (*pos).x &&
                       (*pos).x <
                           ((*waterBox).xMin as libc::c_int +
                                (*waterBox).xLength as libc::c_int) as
                               libc::c_float {
                    if ((*waterBox).zMin as libc::c_int as libc::c_float) <
                           (*pos).z &&
                           (*pos).z <
                               ((*waterBox).zMin as libc::c_int +
                                    (*waterBox).zLength as libc::c_int) as
                                   libc::c_float {
                        if (*pos).y - surfaceChkDist <
                               (*waterBox).ySurface as libc::c_int as
                                   libc::c_float &&
                               ((*waterBox).ySurface as libc::c_int as
                                    libc::c_float) < (*pos).y + surfaceChkDist
                           {
                            *outWaterBox = waterBox;
                            return i
                        }
                    }
                }
            }
        }
        i += 1
    }
    *outWaterBox = 0 as *mut WaterBox;
    return -(1 as libc::c_int);
}
/* *
 * WaterBox get CamData index
 */
#[no_mangle]
pub unsafe extern "C" fn WaterBox_GetCamDataIndex(mut colCtx:
                                                      *mut CollisionContext,
                                                  mut waterBox: *mut WaterBox)
 -> u32_0 {
    let mut prop: u32_0 = (*waterBox).properties >> 0 as libc::c_int;
    return prop & 0xff as libc::c_int as libc::c_uint;
}
/* *
 * WaterBox get CamData cameraSType
 */
#[no_mangle]
pub unsafe extern "C" fn WaterBox_GetCameraSType(mut colCtx:
                                                     *mut CollisionContext,
                                                 mut waterBox: *mut WaterBox)
 -> u16_0 {
    let mut camId: s32 = WaterBox_GetCamDataIndex(colCtx, waterBox) as s32;
    let mut camData: *mut CamData = (*(*colCtx).colHeader).cameraDataList;
    if camData ==
           gSegments[0 as libc::c_int as
                         usize].wrapping_add(0x80000000 as libc::c_uint) as
               *mut libc::c_void as *mut CamData {
        return 0 as libc::c_int as u16_0
    }
    return (*(*(*colCtx).colHeader).cameraDataList.offset(camId as
                                                              isize)).cameraSType;
}
/* *
 * WaterBox get lighting settings
 */
#[no_mangle]
pub unsafe extern "C" fn WaterBox_GetLightSettingIndex(mut colCtx:
                                                           *mut CollisionContext,
                                                       mut waterBox:
                                                           *mut WaterBox)
 -> u32_0 {
    let mut prop: u32_0 = (*waterBox).properties >> 8 as libc::c_int;
    return prop & 0x1f as libc::c_int as libc::c_uint;
}
/* *
 * Get the water surface at point (`x`, `ySurface`, `z`). `ySurface` doubles as position y input
 * same as WaterBox_GetSurfaceImpl, but tests if WaterBox properties & 0x80000 != 0
 * returns true if point is within the xz boundaries of an active water box, else false
 * `ySurface` returns the water box's surface, while `outWaterBox` returns a pointer to the WaterBox
 */
#[no_mangle]
pub unsafe extern "C" fn func_800425B0(mut globalCtx: *mut GlobalContext,
                                       mut colCtx: *mut CollisionContext,
                                       mut x: f32_0, mut z: f32_0,
                                       mut ySurface: *mut f32_0,
                                       mut outWaterBox: *mut *mut WaterBox)
 -> s32 {
    let mut colHeader: *mut CollisionHeader = (*colCtx).colHeader;
    let mut room: u32_0 = 0;
    let mut curWaterBox: *mut WaterBox = 0 as *mut WaterBox;
    if (*colHeader).numWaterBoxes as libc::c_int == 0 as libc::c_int ||
           (*colHeader).waterBoxes ==
               gSegments[0 as libc::c_int as
                             usize].wrapping_add(0x80000000 as libc::c_uint)
                   as *mut libc::c_void as *mut WaterBox {
        return 0 as libc::c_int
    }
    curWaterBox = (*colHeader).waterBoxes;
    while curWaterBox <
              (*colHeader).waterBoxes.offset((*colHeader).numWaterBoxes as
                                                 libc::c_int as isize) {
        room =
            (*curWaterBox).properties >> 0xd as libc::c_int &
                0x3f as libc::c_int as libc::c_uint;
        if room == (*globalCtx).roomCtx.curRoom.num as u32_0 ||
               room == 0x3f as libc::c_int as libc::c_uint {
            if (*curWaterBox).properties &
                   0x80000 as libc::c_int as libc::c_uint !=
                   0 as libc::c_int as libc::c_uint {
                if ((*curWaterBox).xMin as libc::c_int as libc::c_float) < x
                       &&
                       x <
                           ((*curWaterBox).xMin as libc::c_int +
                                (*curWaterBox).xLength as libc::c_int) as
                               libc::c_float {
                    if ((*curWaterBox).zMin as libc::c_int as libc::c_float) <
                           z &&
                           z <
                               ((*curWaterBox).zMin as libc::c_int +
                                    (*curWaterBox).zLength as libc::c_int) as
                                   libc::c_float {
                        *outWaterBox = curWaterBox;
                        *ySurface = (*curWaterBox).ySurface as f32_0;
                        return 1 as libc::c_int
                    }
                }
            }
        }
        curWaterBox = curWaterBox.offset(1)
    }
    return 0 as libc::c_int;
}
/* *
 * Gets the `closestPoint` to `point` on the line formed from the intesection of planes `polyA` and `polyB`
 * returns true if the `closestPoint` exists, else returns false
 */
#[no_mangle]
pub unsafe extern "C" fn func_80042708(mut polyA: *mut CollisionPoly,
                                       mut polyB: *mut CollisionPoly,
                                       mut point: *mut Vec3f,
                                       mut closestPoint: *mut Vec3f) -> s32 {
    let mut n1X: f32_0 = 0.;
    let mut n1Y: f32_0 = 0.;
    let mut n1Z: f32_0 = 0.;
    let mut n2X: f32_0 = 0.;
    let mut n2Y: f32_0 = 0.;
    let mut n2Z: f32_0 = 0.;
    CollisionPoly_GetNormalF(polyA, &mut n1X, &mut n1Y, &mut n1Z);
    CollisionPoly_GetNormalF(polyB, &mut n2X, &mut n2Y, &mut n2Z);
    return Math3D_PlaneVsPlaneVsLineClosestPoint(n1X, n1Y, n1Z,
                                                 (*polyA).dist as f32_0, n2X,
                                                 n2Y, n2Z,
                                                 (*polyB).dist as f32_0,
                                                 point, closestPoint);
}
/* *
 * Get the `closestPoint` to line (`pointA`, `pointB`) formed from the intersection of planes `polyA` and `polyB`
 * returns true if the `closestPoint` exists, else returns false
 */
#[no_mangle]
pub unsafe extern "C" fn func_800427B4(mut polyA: *mut CollisionPoly,
                                       mut polyB: *mut CollisionPoly,
                                       mut pointA: *mut Vec3f,
                                       mut pointB: *mut Vec3f,
                                       mut closestPoint: *mut Vec3f) -> s32 {
    let mut n1X: f32_0 = 0.;
    let mut n1Y: f32_0 = 0.;
    let mut n1Z: f32_0 = 0.;
    let mut n2X: f32_0 = 0.;
    let mut n2Y: f32_0 = 0.;
    let mut n2Z: f32_0 = 0.;
    let mut result: s32 = 0;
    CollisionPoly_GetNormalF(polyA, &mut n1X, &mut n1Y, &mut n1Z);
    CollisionPoly_GetNormalF(polyB, &mut n2X, &mut n2Y, &mut n2Z);
    result =
        Math3D_PlaneVsLineSegClosestPoint(n1X, n1Y, n1Z,
                                          (*polyA).dist as f32_0, n2X, n2Y,
                                          n2Z, (*polyB).dist as f32_0, pointA,
                                          pointB, closestPoint);
    return result;
}
/* *
 * Draw a list of dyna polys, specified by `ssList`
 */
#[no_mangle]
pub unsafe extern "C" fn BgCheck_DrawDynaPolyList(mut globalCtx:
                                                      *mut GlobalContext,
                                                  mut colCtx:
                                                      *mut CollisionContext,
                                                  mut dyna:
                                                      *mut DynaCollisionContext,
                                                  mut ssList: *mut SSList,
                                                  mut r: u8_0, mut g: u8_0,
                                                  mut b: u8_0) {
    let mut curPolyId: s16 = 0;
    let mut poly: *mut CollisionPoly = 0 as *mut CollisionPoly;
    let mut curNode: *mut SSNode = 0 as *mut SSNode;
    let mut vA: Vec3f = Vec3f{x: 0., y: 0., z: 0.,};
    let mut vB: Vec3f = Vec3f{x: 0., y: 0., z: 0.,};
    let mut vC: Vec3f = Vec3f{x: 0., y: 0., z: 0.,};
    let mut nx: f32_0 = 0.;
    let mut ny: f32_0 = 0.;
    let mut nz: f32_0 = 0.;
    if (*ssList).head as libc::c_int != 0xffff as libc::c_int {
        curNode =
            &mut *(*dyna).polyNodes.tbl.offset((*ssList).head as isize) as
                *mut SSNode;
        loop  {
            curPolyId = (*curNode).polyId;
            poly =
                &mut *(*dyna).polyList.offset(curPolyId as isize) as
                    *mut CollisionPoly;
            BgCheck_Vec3sToVec3f((*dyna).vtxList.offset(((*poly).c2rust_unnamed.c2rust_unnamed.flags_vIA
                                                             as libc::c_int &
                                                             0x1fff as
                                                                 libc::c_int)
                                                            as isize),
                                 &mut vA);
            BgCheck_Vec3sToVec3f((*dyna).vtxList.offset(((*poly).c2rust_unnamed.c2rust_unnamed.flags_vIB
                                                             as libc::c_int &
                                                             0x1fff as
                                                                 libc::c_int)
                                                            as isize),
                                 &mut vB);
            BgCheck_Vec3sToVec3f((*dyna).vtxList.offset((*poly).c2rust_unnamed.c2rust_unnamed.vIC
                                                            as s32 as isize),
                                 &mut vC);
            if (*gGameInfo).data[(19 as libc::c_int * 6 as libc::c_int *
                                      16 as libc::c_int + 26 as libc::c_int)
                                     as usize] != 0 {
                nx =
                    (*poly).normal.x as libc::c_int as libc::c_float *
                        (1.0f32 / 32767.0f32);
                ny =
                    (*poly).normal.y as libc::c_int as libc::c_float *
                        (1.0f32 / 32767.0f32);
                nz =
                    (*poly).normal.z as libc::c_int as libc::c_float *
                        (1.0f32 / 32767.0f32);
                vA.x +=
                    (*gGameInfo).data[(19 as libc::c_int * 6 as libc::c_int *
                                           16 as libc::c_int +
                                           26 as libc::c_int) as usize] as
                        libc::c_int as libc::c_float * nx;
                vA.y +=
                    (*gGameInfo).data[(19 as libc::c_int * 6 as libc::c_int *
                                           16 as libc::c_int +
                                           26 as libc::c_int) as usize] as
                        libc::c_int as libc::c_float * ny;
                vA.z +=
                    (*gGameInfo).data[(19 as libc::c_int * 6 as libc::c_int *
                                           16 as libc::c_int +
                                           26 as libc::c_int) as usize] as
                        libc::c_int as libc::c_float * nz;
                vB.x +=
                    (*gGameInfo).data[(19 as libc::c_int * 6 as libc::c_int *
                                           16 as libc::c_int +
                                           26 as libc::c_int) as usize] as
                        libc::c_int as libc::c_float * nx;
                vB.y +=
                    (*gGameInfo).data[(19 as libc::c_int * 6 as libc::c_int *
                                           16 as libc::c_int +
                                           26 as libc::c_int) as usize] as
                        libc::c_int as libc::c_float * ny;
                vB.z +=
                    (*gGameInfo).data[(19 as libc::c_int * 6 as libc::c_int *
                                           16 as libc::c_int +
                                           26 as libc::c_int) as usize] as
                        libc::c_int as libc::c_float * nz;
                vC.x +=
                    (*gGameInfo).data[(19 as libc::c_int * 6 as libc::c_int *
                                           16 as libc::c_int +
                                           26 as libc::c_int) as usize] as
                        libc::c_int as libc::c_float * nx;
                vC.y +=
                    (*gGameInfo).data[(19 as libc::c_int * 6 as libc::c_int *
                                           16 as libc::c_int +
                                           26 as libc::c_int) as usize] as
                        libc::c_int as libc::c_float * ny;
                vC.z +=
                    (*gGameInfo).data[(19 as libc::c_int * 6 as libc::c_int *
                                           16 as libc::c_int +
                                           26 as libc::c_int) as usize] as
                        libc::c_int as libc::c_float * nz
            }
            Collider_DrawPoly((*globalCtx).state.gfxCtx, &mut vA, &mut vB,
                              &mut vC, r, g, b);
            if (*curNode).next as libc::c_int == 0xffff as libc::c_int {
                break ;
            }
            curNode =
                &mut *(*dyna).polyNodes.tbl.offset((*curNode).next as isize)
                    as *mut SSNode
        }
    };
}
/* *
 * Draw a BgActor's dyna polys
 * `bgId` is the BgActor index that should be drawn
 */
#[no_mangle]
pub unsafe extern "C" fn BgCheck_DrawBgActor(mut globalCtx:
                                                 *mut GlobalContext,
                                             mut colCtx:
                                                 *mut CollisionContext,
                                             mut bgId: s32) {
    if (*gGameInfo).data[(19 as libc::c_int * 6 as libc::c_int *
                              16 as libc::c_int + 21 as libc::c_int) as usize]
           != 0 {
        BgCheck_DrawDynaPolyList(globalCtx, colCtx, &mut (*colCtx).dyna,
                                 &mut (*(*colCtx).dyna.bgActors.as_mut_ptr().offset(bgId
                                                                                        as
                                                                                        isize)).dynaLookup.ceiling,
                                 255 as libc::c_int as u8_0,
                                 0 as libc::c_int as u8_0,
                                 0 as libc::c_int as u8_0);
    }
    if (*gGameInfo).data[(19 as libc::c_int * 6 as libc::c_int *
                              16 as libc::c_int + 22 as libc::c_int) as usize]
           != 0 {
        BgCheck_DrawDynaPolyList(globalCtx, colCtx, &mut (*colCtx).dyna,
                                 &mut (*(*colCtx).dyna.bgActors.as_mut_ptr().offset(bgId
                                                                                        as
                                                                                        isize)).dynaLookup.wall,
                                 0 as libc::c_int as u8_0,
                                 255 as libc::c_int as u8_0,
                                 0 as libc::c_int as u8_0);
    }
    if (*gGameInfo).data[(19 as libc::c_int * 6 as libc::c_int *
                              16 as libc::c_int + 23 as libc::c_int) as usize]
           != 0 {
        BgCheck_DrawDynaPolyList(globalCtx, colCtx, &mut (*colCtx).dyna,
                                 &mut (*(*colCtx).dyna.bgActors.as_mut_ptr().offset(bgId
                                                                                        as
                                                                                        isize)).dynaLookup.floor,
                                 0 as libc::c_int as u8_0,
                                 0 as libc::c_int as u8_0,
                                 255 as libc::c_int as u8_0);
    };
}
/* *
 * Draw all dyna polys
 */
#[no_mangle]
pub unsafe extern "C" fn BgCheck_DrawDynaCollision(mut globalCtx:
                                                       *mut GlobalContext,
                                                   mut colCtx:
                                                       *mut CollisionContext) {
    let mut bgId: s32 = 0;
    bgId = 0 as libc::c_int;
    while bgId < 50 as libc::c_int {
        if !((*colCtx).dyna.bgActorFlags[bgId as usize] as libc::c_int &
                 1 as libc::c_int == 0) {
            BgCheck_DrawBgActor(globalCtx, colCtx, bgId);
        }
        bgId += 1
    };
}
/* *
 * Draw a static poly
 */
#[no_mangle]
pub unsafe extern "C" fn BgCheck_DrawStaticPoly(mut globalCtx:
                                                    *mut GlobalContext,
                                                mut colCtx:
                                                    *mut CollisionContext,
                                                mut poly: *mut CollisionPoly,
                                                mut r: u8_0, mut g: u8_0,
                                                mut b: u8_0) {
    let mut vA: Vec3f = Vec3f{x: 0., y: 0., z: 0.,};
    let mut vB: Vec3f = Vec3f{x: 0., y: 0., z: 0.,};
    let mut vC: Vec3f = Vec3f{x: 0., y: 0., z: 0.,};
    let mut nx: f32_0 = 0.;
    let mut ny: f32_0 = 0.;
    let mut nz: f32_0 = 0.;
    BgCheck_Vec3sToVec3f((*(*colCtx).colHeader).vtxList.offset(((*poly).c2rust_unnamed.c2rust_unnamed.flags_vIA
                                                                    as
                                                                    libc::c_int
                                                                    &
                                                                    0x1fff as
                                                                        libc::c_int)
                                                                   as isize),
                         &mut vA);
    BgCheck_Vec3sToVec3f((*(*colCtx).colHeader).vtxList.offset(((*poly).c2rust_unnamed.c2rust_unnamed.flags_vIB
                                                                    as
                                                                    libc::c_int
                                                                    &
                                                                    0x1fff as
                                                                        libc::c_int)
                                                                   as isize),
                         &mut vB);
    BgCheck_Vec3sToVec3f((*(*colCtx).colHeader).vtxList.offset((*poly).c2rust_unnamed.c2rust_unnamed.vIC
                                                                   as
                                                                   libc::c_int
                                                                   as isize),
                         &mut vC);
    if (*gGameInfo).data[(19 as libc::c_int * 6 as libc::c_int *
                              16 as libc::c_int + 26 as libc::c_int) as usize]
           as libc::c_int != 0 as libc::c_int {
        nx =
            (*poly).normal.x as libc::c_int as libc::c_float *
                (1.0f32 / 32767.0f32);
        ny =
            (*poly).normal.y as libc::c_int as libc::c_float *
                (1.0f32 / 32767.0f32);
        nz =
            (*poly).normal.z as libc::c_int as libc::c_float *
                (1.0f32 / 32767.0f32);
        vA.x +=
            (*gGameInfo).data[(19 as libc::c_int * 6 as libc::c_int *
                                   16 as libc::c_int + 26 as libc::c_int) as
                                  usize] as libc::c_int as libc::c_float * nx;
        vA.y +=
            (*gGameInfo).data[(19 as libc::c_int * 6 as libc::c_int *
                                   16 as libc::c_int + 26 as libc::c_int) as
                                  usize] as libc::c_int as libc::c_float * ny;
        vA.z +=
            (*gGameInfo).data[(19 as libc::c_int * 6 as libc::c_int *
                                   16 as libc::c_int + 26 as libc::c_int) as
                                  usize] as libc::c_int as libc::c_float * nz;
        vB.x +=
            (*gGameInfo).data[(19 as libc::c_int * 6 as libc::c_int *
                                   16 as libc::c_int + 26 as libc::c_int) as
                                  usize] as libc::c_int as libc::c_float * nx;
        vB.y +=
            (*gGameInfo).data[(19 as libc::c_int * 6 as libc::c_int *
                                   16 as libc::c_int + 26 as libc::c_int) as
                                  usize] as libc::c_int as libc::c_float * ny;
        vB.z +=
            (*gGameInfo).data[(19 as libc::c_int * 6 as libc::c_int *
                                   16 as libc::c_int + 26 as libc::c_int) as
                                  usize] as libc::c_int as libc::c_float * nz;
        vC.x +=
            (*gGameInfo).data[(19 as libc::c_int * 6 as libc::c_int *
                                   16 as libc::c_int + 26 as libc::c_int) as
                                  usize] as libc::c_int as libc::c_float * nx;
        vC.y +=
            (*gGameInfo).data[(19 as libc::c_int * 6 as libc::c_int *
                                   16 as libc::c_int + 26 as libc::c_int) as
                                  usize] as libc::c_int as libc::c_float * ny;
        vC.z +=
            (*gGameInfo).data[(19 as libc::c_int * 6 as libc::c_int *
                                   16 as libc::c_int + 26 as libc::c_int) as
                                  usize] as libc::c_int as libc::c_float * nz
    }
    Collider_DrawPoly((*globalCtx).state.gfxCtx, &mut vA, &mut vB, &mut vC, r,
                      g, b);
}
/* *
 * Draw a list of static polys, specified by `ssList`
 */
#[no_mangle]
pub unsafe extern "C" fn BgCheck_DrawStaticPolyList(mut globalCtx:
                                                        *mut GlobalContext,
                                                    mut colCtx:
                                                        *mut CollisionContext,
                                                    mut ssList: *mut SSList,
                                                    mut r: u8_0, mut g: u8_0,
                                                    mut b: u8_0) {
    let mut curNode: *mut SSNode = 0 as *mut SSNode;
    let mut polyList: *mut CollisionPoly = (*(*colCtx).colHeader).polyList;
    let mut curPolyId: s16 = 0;
    if (*ssList).head as libc::c_int != 0xffff as libc::c_int {
        curNode =
            &mut *(*colCtx).polyNodes.tbl.offset((*ssList).head as isize) as
                *mut SSNode;
        loop  {
            curPolyId = (*curNode).polyId;
            BgCheck_DrawStaticPoly(globalCtx, colCtx,
                                   &mut *polyList.offset(curPolyId as isize),
                                   r, g, b);
            if (*curNode).next as libc::c_int == 0xffff as libc::c_int {
                break ;
            }
            curNode =
                &mut *(*colCtx).polyNodes.tbl.offset((*curNode).next as isize)
                    as *mut SSNode
        }
    };
}
/* *
 * Draw scene collision
 */
#[no_mangle]
pub unsafe extern "C" fn BgCheck_DrawStaticCollision(mut globalCtx:
                                                         *mut GlobalContext,
                                                     mut colCtx:
                                                         *mut CollisionContext) {
    let mut player: *mut Player =
        (*globalCtx).actorCtx.actorLists[ACTORCAT_PLAYER as libc::c_int as
                                             usize].head as *mut Player;
    let mut lookup: *mut StaticLookup =
        BgCheck_GetNearestStaticLookup(colCtx, (*colCtx).lookupTbl,
                                       &mut (*player).actor.world.pos);
    if (*gGameInfo).data[(19 as libc::c_int * 6 as libc::c_int *
                              16 as libc::c_int + 23 as libc::c_int) as usize]
           as libc::c_int != 0 as libc::c_int {
        BgCheck_DrawStaticPolyList(globalCtx, colCtx, &mut (*lookup).floor,
                                   0 as libc::c_int as u8_0,
                                   0 as libc::c_int as u8_0,
                                   255 as libc::c_int as u8_0);
    }
    if (*gGameInfo).data[(19 as libc::c_int * 6 as libc::c_int *
                              16 as libc::c_int + 22 as libc::c_int) as usize]
           as libc::c_int != 0 as libc::c_int {
        BgCheck_DrawStaticPolyList(globalCtx, colCtx, &mut (*lookup).wall,
                                   0 as libc::c_int as u8_0,
                                   255 as libc::c_int as u8_0,
                                   0 as libc::c_int as u8_0);
    }
    if (*gGameInfo).data[(19 as libc::c_int * 6 as libc::c_int *
                              16 as libc::c_int + 21 as libc::c_int) as usize]
           as libc::c_int != 0 as libc::c_int {
        BgCheck_DrawStaticPolyList(globalCtx, colCtx, &mut (*lookup).ceiling,
                                   255 as libc::c_int as u8_0,
                                   0 as libc::c_int as u8_0,
                                   0 as libc::c_int as u8_0);
    };
}
