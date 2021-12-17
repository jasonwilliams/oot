#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case,
         non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(const_raw_ptr_to_usize_cast, const_transmute, register_tool)]
extern "C" {
    #[no_mangle]
    fn sqrtf(f: f32_0) -> f32_0;
    #[no_mangle]
    fn __assert(exp: *const libc::c_char, file: *const libc::c_char,
                line: s32);
    #[no_mangle]
    fn osSyncPrintf(fmt: *const libc::c_char, _: ...);
    #[no_mangle]
    fn func_8002873C(globalCtx: *mut GlobalContext, pos: *mut Vec3f,
                     velocity: *mut Vec3f, accel: *mut Vec3f, scale: s16,
                     scaleStep: s16, life: s16);
    #[no_mangle]
    fn EffectSsKiraKira_SpawnDispersed(globalCtx: *mut GlobalContext,
                                       pos: *mut Vec3f, velocity: *mut Vec3f,
                                       accel: *mut Vec3f,
                                       primColor: *mut Color_RGBA8,
                                       envColor: *mut Color_RGBA8, scale: s16,
                                       life: s32);
    #[no_mangle]
    fn ActorShape_Init(shape: *mut ActorShape, yOffset: f32_0,
                       shadowDraw: ActorShadowFunc, shadowScale: f32_0);
    #[no_mangle]
    fn Actor_Kill(actor: *mut Actor);
    #[no_mangle]
    fn Actor_SetScale(actor: *mut Actor, scale: f32_0);
    #[no_mangle]
    fn Actor_MoveForward(actor: *mut Actor);
    #[no_mangle]
    fn func_8002EBCC(actor: *mut Actor, globalCtx: *mut GlobalContext,
                     flag: s32);
    #[no_mangle]
    fn func_8002ED80(actor: *mut Actor, globalCtx: *mut GlobalContext,
                     flag: s32);
    #[no_mangle]
    fn Audio_PlayActorSound2(actor: *mut Actor, sfxId: u16_0);
    #[no_mangle]
    fn func_8002F948(actor: *mut Actor, sfxId: u16_0);
    #[no_mangle]
    fn func_8002F974(actor: *mut Actor, sfxId: u16_0);
    #[no_mangle]
    fn Actor_Spawn(actorCtx: *mut ActorContext, globalCtx: *mut GlobalContext,
                   actorId: s16, posX: f32_0, posY: f32_0, posZ: f32_0,
                   rotX: s16, rotY: s16, rotZ: s16, params: s16)
     -> *mut Actor;
    #[no_mangle]
    fn Actor_SpawnAsChild(actorCtx: *mut ActorContext, parent: *mut Actor,
                          globalCtx: *mut GlobalContext, actorId: s16,
                          posX: f32_0, posY: f32_0, posZ: f32_0, rotX: s16,
                          rotY: s16, rotZ: s16, params: s16) -> *mut Actor;
    #[no_mangle]
    fn Actor_ChangeCategory(globalCtx: *mut GlobalContext,
                            actorCtx: *mut ActorContext, actor: *mut Actor,
                            actorCategory: u8_0);
    #[no_mangle]
    fn Rand_CenteredFloat(f: f32_0) -> f32_0;
    #[no_mangle]
    fn GetItem_Draw(globalCtx: *mut GlobalContext, drawId: s16);
    #[no_mangle]
    fn Flags_GetEnv(globalCtx: *mut GlobalContext, flag: s16) -> s32;
    #[no_mangle]
    fn SkelCurve_Clear(skelCurve: *mut SkelAnimeCurve);
    #[no_mangle]
    fn SkelCurve_Init(globalCtx: *mut GlobalContext,
                      skelCurve: *mut SkelAnimeCurve,
                      limbListSeg: *mut SkelCurveLimbList,
                      transUpdIdx: *mut TransformUpdateIndex) -> s32;
    #[no_mangle]
    fn SkelCurve_Destroy(globalCtx: *mut GlobalContext,
                         skelCurve: *mut SkelAnimeCurve);
    #[no_mangle]
    fn SkelCurve_SetAnim(skelCurve: *mut SkelAnimeCurve,
                         transUpdIdx: *mut TransformUpdateIndex, arg2: f32_0,
                         animFinalFrame: f32_0, animCurFrame: f32_0,
                         animSpeed: f32_0);
    #[no_mangle]
    fn SkelCurve_Update(globalCtx: *mut GlobalContext,
                        skelCurve: *mut SkelAnimeCurve) -> s32;
    #[no_mangle]
    fn SkelCurve_Draw(actor: *mut Actor, globalCtx: *mut GlobalContext,
                      skelCurve: *mut SkelAnimeCurve,
                      overrideLimbDraw: OverrideCurveLimbDraw,
                      postLimbDraw: PostCurveLimbDraw, lod: s32,
                      data: *mut libc::c_void);
    #[no_mangle]
    fn Environment_LerpWeight(max: u16_0, min: u16_0, val: u16_0) -> f32_0;
    #[no_mangle]
    fn Math_Vec3f_Yaw(a: *mut Vec3f, b: *mut Vec3f) -> s16;
    #[no_mangle]
    fn Math_SmoothStepToF(pValue: *mut f32_0, target: f32_0, fraction: f32_0,
                          step: f32_0, minStep: f32_0) -> f32_0;
    #[no_mangle]
    fn func_800788CC(sfxId: u16_0);
    #[no_mangle]
    fn func_80078914(arg0: *mut Vec3f, sfxId: u16_0);
    #[no_mangle]
    fn Gfx_CallSetupDL(gfx: *mut Gfx, i: u32_0) -> *mut Gfx;
    #[no_mangle]
    fn func_80093D18(gfxCtx: *mut GraphicsContext);
    #[no_mangle]
    fn func_80093D84(gfxCtx: *mut GraphicsContext);
    #[no_mangle]
    fn Gfx_TexScroll(gfxCtx: *mut GraphicsContext, x: u32_0, y: u32_0,
                     width: s32, height: s32) -> *mut Gfx;
    #[no_mangle]
    fn Gfx_TwoTexScroll(gfxCtx: *mut GraphicsContext, tile1: s32, x1: u32_0,
                        y1: u32_0, width1: s32, height1: s32, tile2: s32,
                        x2: u32_0, y2: u32_0, width2: s32, height2: s32)
     -> *mut Gfx;
    #[no_mangle]
    fn Object_GetIndex(objectCtx: *mut ObjectContext, objectId: s16) -> s32;
    #[no_mangle]
    fn Object_IsLoaded(objectCtx: *mut ObjectContext, bankIndex: s32) -> s32;
    #[no_mangle]
    fn Graph_OpenDisps(dispRefs: *mut *mut Gfx, gfxCtx: *mut GraphicsContext,
                       file: *const libc::c_char, line: s32);
    #[no_mangle]
    fn Graph_CloseDisps(dispRefs: *mut *mut Gfx, gfxCtx: *mut GraphicsContext,
                        file: *const libc::c_char, line: s32);
    #[no_mangle]
    fn Math_Atan2F(x: f32_0, y: f32_0) -> f32_0;
    #[no_mangle]
    fn Matrix_Push();
    #[no_mangle]
    fn Matrix_Pop();
    #[no_mangle]
    fn Matrix_Mult(mf: *mut MtxF, mode: u8_0);
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
    fn Matrix_NewMtx(gfxCtx: *mut GraphicsContext, file: *mut libc::c_char,
                     line: s32) -> *mut Mtx;
    #[no_mangle]
    fn func_800F3F3C(_: u8_0);
    #[no_mangle]
    fn Math_FAtan2F(y: f32_0, x: f32_0) -> f32_0;
    #[no_mangle]
    fn Rand_ZeroOne() -> f32_0;
    #[no_mangle]
    fn sinf(_: f32_0) -> f32_0;
    #[no_mangle]
    fn cosf(_: f32_0) -> f32_0;
    #[no_mangle]
    static mut gSegments: [u32_0; 16];
    #[no_mangle]
    static mut gSaveContext: SaveContext;
    #[no_mangle]
    static mut gEffFlash1DL: [Gfx; 0];
    #[no_mangle]
    static mut gCrystalLightDL: [Gfx; 0];
    #[no_mangle]
    static mut gCreationFireBallDL: [Gfx; 0];
    #[no_mangle]
    static mut gEnliveningLightDL: [Gfx; 0];
    #[no_mangle]
    static mut gGoldenGoddessAuraDL: [Gfx; 0];
    #[no_mangle]
    static mut gGoldenGoddessBodyDL: [Gfx; 0];
    #[no_mangle]
    static mut gGoldenGoddessLightRingDL: [Gfx; 0];
    #[no_mangle]
    static mut gTriforceVtx: [Vtx; 0];
    #[no_mangle]
    static mut gTriforceDL: [Gfx; 0];
    #[no_mangle]
    static mut gTriforceLightColumnDL: [Gfx; 0];
    #[no_mangle]
    static mut gTimeWarpAnim: TransformUpdateIndex;
    #[no_mangle]
    static mut gTimeWarpVtx: [Vtx; 0];
    #[no_mangle]
    static mut gTimeWarpSkel: SkelCurveLimbList;
    #[no_mangle]
    static mut gGiKokiriEmeraldSettingDL: [Gfx; 0];
    #[no_mangle]
    static mut gGiKokiriEmeraldGemDL: [Gfx; 0];
    #[no_mangle]
    static mut gGiGoronRubySettingDL: [Gfx; 0];
    #[no_mangle]
    static mut gGiGoronRubyGemDL: [Gfx; 0];
    #[no_mangle]
    static mut gGiZoraSapphireSettingDL: [Gfx; 0];
    #[no_mangle]
    static mut gGiZoraSapphireGemDL: [Gfx; 0];
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
pub struct TransformData {
    pub unk_00: u16_0,
    pub unk_02: s16,
    pub unk_04: s16,
    pub unk_06: s16,
    pub unk_08: f32_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct TransformUpdateIndex {
    pub refIndex: *mut u8_0,
    pub transformData: *mut TransformData,
    pub copyValues: *mut s16,
    pub unk_0C: s16,
    pub unk_0E: s16,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SkelCurveLimb {
    pub firstChildIdx: u8_0,
    pub nextLimbIdx: u8_0,
    pub dList: [*mut Gfx; 2],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SkelCurveLimbList {
    pub limbs: *mut *mut SkelCurveLimb,
    pub limbCount: u8_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct LimbTransform {
    pub scale: Vec3s,
    pub rot: Vec3s,
    pub pos: Vec3s,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SkelAnimeCurve {
    pub limbCount: u8_0,
    pub limbList: *mut *mut SkelCurveLimb,
    pub transUpdIdx: *mut TransformUpdateIndex,
    pub unk_0C: f32_0,
    pub animFinalFrame: f32_0,
    pub animSpeed: f32_0,
    pub animCurFrame: f32_0,
    pub transforms: *mut LimbTransform,
}
pub type OverrideCurveLimbDraw
    =
    Option<unsafe extern "C" fn(_: *mut GlobalContext, _: *mut SkelAnimeCurve,
                                _: s32, _: *mut libc::c_void) -> s32>;
pub type PostCurveLimbDraw
    =
    Option<unsafe extern "C" fn(_: *mut GlobalContext, _: *mut SkelAnimeCurve,
                                _: s32, _: *mut libc::c_void) -> ()>;
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
pub const ACTOR_ID_MAX: C2RustUnnamed_15 = 471;
pub const ACTOR_OBJ_WARP2BLOCK: C2RustUnnamed_15 = 470;
pub const ACTOR_BG_JYA_BLOCK: C2RustUnnamed_15 = 469;
pub const ACTOR_EN_MM2: C2RustUnnamed_15 = 468;
pub const ACTOR_EN_ZL4: C2RustUnnamed_15 = 467;
pub const ACTOR_OBJ_HAMISHI: C2RustUnnamed_15 = 466;
pub const ACTOR_OBJ_TIMEBLOCK: C2RustUnnamed_15 = 465;
pub const ACTOR_EN_GE3: C2RustUnnamed_15 = 464;
pub const ACTOR_OBJ_MAKEKINSUTA: C2RustUnnamed_15 = 463;
pub const ACTOR_EN_ZO: C2RustUnnamed_15 = 462;
pub const ACTOR_BG_MENKURI_NISEKABE: C2RustUnnamed_15 = 461;
pub const ACTOR_EN_EG: C2RustUnnamed_15 = 460;
pub const ACTOR_OCEFF_WIPE4: C2RustUnnamed_15 = 459;
pub const ACTOR_EN_KAKASI3: C2RustUnnamed_15 = 458;
pub const ACTOR_EN_KAKASI2: C2RustUnnamed_15 = 457;
pub const ACTOR_BG_ICE_SHUTTER: C2RustUnnamed_15 = 456;
pub const ACTOR_BG_ICE_TURARA: C2RustUnnamed_15 = 455;
pub const ACTOR_EN_COW: C2RustUnnamed_15 = 454;
pub const ACTOR_EN_MA3: C2RustUnnamed_15 = 453;
pub const ACTOR_BG_SPOT18_SHUTTER: C2RustUnnamed_15 = 452;
pub const ACTOR_BG_SPOT18_FUTA: C2RustUnnamed_15 = 451;
pub const ACTOR_BG_SPOT11_OASIS: C2RustUnnamed_15 = 450;
pub const ACTOR_DOOR_KILLER: C2RustUnnamed_15 = 449;
pub const ACTOR_EN_CROW: C2RustUnnamed_15 = 448;
pub const ACTOR_EN_PO_DESERT: C2RustUnnamed_15 = 447;
pub const ACTOR_EN_WALL_TUBO: C2RustUnnamed_15 = 446;
pub const ACTOR_BG_BOWL_WALL: C2RustUnnamed_15 = 445;
pub const ACTOR_EN_DAIKU_KAKARIKO: C2RustUnnamed_15 = 444;
pub const ACTOR_BG_MIZU_SHUTTER: C2RustUnnamed_15 = 443;
pub const ACTOR_BG_MIZU_BWALL: C2RustUnnamed_15 = 442;
pub const ACTOR_EN_GS: C2RustUnnamed_15 = 441;
pub const ACTOR_EN_GB: C2RustUnnamed_15 = 440;
pub const ACTOR_BG_GND_ICEBLOCK: C2RustUnnamed_15 = 439;
pub const ACTOR_BG_GND_NISEKABE: C2RustUnnamed_15 = 438;
pub const ACTOR_BG_GND_SOULMEIRO: C2RustUnnamed_15 = 437;
pub const ACTOR_BG_GND_DARKMEIRO: C2RustUnnamed_15 = 436;
pub const ACTOR_BG_GND_FIREMEIRO: C2RustUnnamed_15 = 435;
pub const ACTOR_DEMO_GEFF: C2RustUnnamed_15 = 434;
pub const ACTOR_DEMO_GJ: C2RustUnnamed_15 = 433;
pub const ACTOR_EN_SKB: C2RustUnnamed_15 = 432;
pub const ACTOR_EN_WF: C2RustUnnamed_15 = 431;
pub const ACTOR_EN_GO2: C2RustUnnamed_15 = 430;
pub const ACTOR_EN_MU: C2RustUnnamed_15 = 429;
pub const ACTOR_EN_TG: C2RustUnnamed_15 = 428;
pub const ACTOR_OBJ_MURE3: C2RustUnnamed_15 = 427;
pub const ACTOR_UNSET_1AA: C2RustUnnamed_15 = 426;
pub const ACTOR_BG_SPOT17_BAKUDANKABE: C2RustUnnamed_15 = 425;
pub const ACTOR_BG_SPOT08_BAKUDANKABE: C2RustUnnamed_15 = 424;
pub const ACTOR_DEMO_KEKKAI: C2RustUnnamed_15 = 423;
pub const ACTOR_EN_HS2: C2RustUnnamed_15 = 422;
pub const ACTOR_BG_BOM_GUARD: C2RustUnnamed_15 = 421;
pub const ACTOR_EN_GUEST: C2RustUnnamed_15 = 420;
pub const ACTOR_EN_DNT_NOMAL: C2RustUnnamed_15 = 419;
pub const ACTOR_EN_DNT_JIJI: C2RustUnnamed_15 = 418;
pub const ACTOR_EN_DNT_DEMO: C2RustUnnamed_15 = 417;
pub const ACTOR_OBJ_KIBAKO2: C2RustUnnamed_15 = 416;
pub const ACTOR_BG_SPOT11_BAKUDANKABE: C2RustUnnamed_15 = 415;
pub const ACTOR_OBJ_COMB: C2RustUnnamed_15 = 414;
pub const ACTOR_BG_SPOT01_OBJECTS2: C2RustUnnamed_15 = 413;
pub const ACTOR_EN_SI: C2RustUnnamed_15 = 412;
pub const ACTOR_EN_DOG: C2RustUnnamed_15 = 411;
pub const ACTOR_EN_NIW_GIRL: C2RustUnnamed_15 = 410;
pub const ACTOR_OCEFF_WIPE3: C2RustUnnamed_15 = 409;
pub const ACTOR_OCEFF_WIPE2: C2RustUnnamed_15 = 408;
pub const ACTOR_EN_GELDB: C2RustUnnamed_15 = 407;
pub const ACTOR_EN_IT: C2RustUnnamed_15 = 406;
pub const ACTOR_EN_SHOPNUTS: C2RustUnnamed_15 = 405;
pub const ACTOR_BG_SPOT00_BREAK: C2RustUnnamed_15 = 404;
pub const ACTOR_EN_NUTSBALL: C2RustUnnamed_15 = 403;
pub const ACTOR_EN_HINTNUTS: C2RustUnnamed_15 = 402;
pub const ACTOR_BG_SPOT12_SAKU: C2RustUnnamed_15 = 401;
pub const ACTOR_BG_SPOT12_GATE: C2RustUnnamed_15 = 400;
pub const ACTOR_BG_JYA_HAHENIRON: C2RustUnnamed_15 = 399;
pub const ACTOR_BG_JYA_1FLIFT: C2RustUnnamed_15 = 398;
pub const ACTOR_BG_SPOT05_SOKO: C2RustUnnamed_15 = 397;
pub const ACTOR_EN_WEIYER: C2RustUnnamed_15 = 396;
pub const ACTOR_OCEFF_STORM: C2RustUnnamed_15 = 395;
pub const ACTOR_OCEFF_WIPE: C2RustUnnamed_15 = 394;
pub const ACTOR_EN_STH: C2RustUnnamed_15 = 393;
pub const ACTOR_EN_SSH: C2RustUnnamed_15 = 392;
pub const ACTOR_OBJ_ROOMTIMER: C2RustUnnamed_15 = 391;
pub const ACTOR_EN_GE2: C2RustUnnamed_15 = 390;
pub const ACTOR_EN_WONDER_TALK2: C2RustUnnamed_15 = 389;
pub const ACTOR_EN_DY_EXTRA: C2RustUnnamed_15 = 388;
pub const ACTOR_SHOT_SUN: C2RustUnnamed_15 = 387;
pub const ACTOR_DEMO_EC: C2RustUnnamed_15 = 386;
pub const ACTOR_EN_TORCH: C2RustUnnamed_15 = 385;
pub const ACTOR_UNSET_180: C2RustUnnamed_15 = 384;
pub const ACTOR_END_TITLE: C2RustUnnamed_15 = 383;
pub const ACTOR_OCEFF_SPOT: C2RustUnnamed_15 = 382;
pub const ACTOR_OBJ_MAKEOSHIHIKI: C2RustUnnamed_15 = 381;
pub const ACTOR_EN_TAKARA_MAN: C2RustUnnamed_15 = 380;
pub const ACTOR_EN_KAKASI: C2RustUnnamed_15 = 379;
pub const ACTOR_BOSS_GANON2: C2RustUnnamed_15 = 378;
pub const ACTOR_EN_ZL3: C2RustUnnamed_15 = 377;
pub const ACTOR_EN_HEISHI4: C2RustUnnamed_15 = 376;
pub const ACTOR_BG_ZG: C2RustUnnamed_15 = 375;
pub const ACTOR_EFC_ERUPC: C2RustUnnamed_15 = 374;
pub const ACTOR_EN_PO_FIELD: C2RustUnnamed_15 = 373;
pub const ACTOR_DEMO_GT: C2RustUnnamed_15 = 372;
pub const ACTOR_ELF_MSG2: C2RustUnnamed_15 = 371;
pub const ACTOR_DOOR_GERUDO: C2RustUnnamed_15 = 370;
pub const ACTOR_EN_MAG: C2RustUnnamed_15 = 369;
pub const ACTOR_EN_OKARINA_EFFECT: C2RustUnnamed_15 = 368;
pub const ACTOR_EN_GANON_MANT: C2RustUnnamed_15 = 367;
pub const ACTOR_EN_HY: C2RustUnnamed_15 = 366;
pub const ACTOR_EN_MD: C2RustUnnamed_15 = 365;
pub const ACTOR_EN_CS: C2RustUnnamed_15 = 364;
pub const ACTOR_EN_JSJUTAN: C2RustUnnamed_15 = 363;
pub const ACTOR_EN_JS: C2RustUnnamed_15 = 362;
pub const ACTOR_BG_JYA_IRONOBJ: C2RustUnnamed_15 = 361;
pub const ACTOR_EN_EX_ITEM: C2RustUnnamed_15 = 360;
pub const ACTOR_EN_ANI: C2RustUnnamed_15 = 359;
pub const ACTOR_BG_SST_FLOOR: C2RustUnnamed_15 = 358;
pub const ACTOR_EN_WEATHER_TAG: C2RustUnnamed_15 = 357;
pub const ACTOR_EN_KZ: C2RustUnnamed_15 = 356;
pub const ACTOR_EN_KO: C2RustUnnamed_15 = 355;
pub const ACTOR_EN_MM: C2RustUnnamed_15 = 354;
pub const ACTOR_UNSET_161: C2RustUnnamed_15 = 353;
pub const ACTOR_EN_STREAM: C2RustUnnamed_15 = 352;
pub const ACTOR_EN_SIOFUKI: C2RustUnnamed_15 = 351;
pub const ACTOR_EN_GANON_ORGAN: C2RustUnnamed_15 = 350;
pub const ACTOR_UNSET_15D: C2RustUnnamed_15 = 349;
pub const ACTOR_BG_SPOT18_BASKET: C2RustUnnamed_15 = 348;
pub const ACTOR_BG_JYA_BOMBIWA: C2RustUnnamed_15 = 347;
pub const ACTOR_BG_JYA_AMISHUTTER: C2RustUnnamed_15 = 346;
pub const ACTOR_BG_JYA_BOMBCHUIWA: C2RustUnnamed_15 = 345;
pub const ACTOR_BG_JYA_BIGMIRROR: C2RustUnnamed_15 = 344;
pub const ACTOR_BG_JYA_LIFT: C2RustUnnamed_15 = 343;
pub const ACTOR_BG_JYA_MEGAMI: C2RustUnnamed_15 = 342;
pub const ACTOR_EN_CHANGER: C2RustUnnamed_15 = 341;
pub const ACTOR_UNSET_154: C2RustUnnamed_15 = 340;
pub const ACTOR_EN_FU: C2RustUnnamed_15 = 339;
pub const ACTOR_EN_GO: C2RustUnnamed_15 = 338;
pub const ACTOR_OBJ_MURE2: C2RustUnnamed_15 = 337;
pub const ACTOR_OBJ_LIGHTSWITCH: C2RustUnnamed_15 = 336;
pub const ACTOR_OBJ_HANA: C2RustUnnamed_15 = 335;
pub const ACTOR_EN_ISHI: C2RustUnnamed_15 = 334;
pub const ACTOR_EN_OWL: C2RustUnnamed_15 = 333;
pub const ACTOR_EN_BOM_BOWL_PIT: C2RustUnnamed_15 = 332;
pub const ACTOR_EN_BOM_BOWL_MAN: C2RustUnnamed_15 = 331;
pub const ACTOR_EN_MK: C2RustUnnamed_15 = 330;
pub const ACTOR_EN_DS: C2RustUnnamed_15 = 329;
pub const ACTOR_BG_GJYO_BRIDGE: C2RustUnnamed_15 = 328;
pub const ACTOR_EN_WONDER_TALK: C2RustUnnamed_15 = 327;
pub const ACTOR_EN_SA: C2RustUnnamed_15 = 326;
pub const ACTOR_BG_SPOT01_IDOSOKO: C2RustUnnamed_15 = 325;
pub const ACTOR_EN_ATTACK_NIW: C2RustUnnamed_15 = 324;
pub const ACTOR_EN_SYATEKI_NIW: C2RustUnnamed_15 = 323;
pub const ACTOR_EN_HEISHI3: C2RustUnnamed_15 = 322;
pub const ACTOR_EN_KANBAN: C2RustUnnamed_15 = 321;
pub const ACTOR_BG_INGATE: C2RustUnnamed_15 = 320;
pub const ACTOR_EN_HS: C2RustUnnamed_15 = 319;
pub const ACTOR_EN_MS: C2RustUnnamed_15 = 318;
pub const ACTOR_EN_GM: C2RustUnnamed_15 = 317;
pub const ACTOR_EN_NIW_LADY: C2RustUnnamed_15 = 316;
pub const ACTOR_EN_CLEAR_TAG: C2RustUnnamed_15 = 315;
pub const ACTOR_EN_SDA: C2RustUnnamed_15 = 314;
pub const ACTOR_OBJ_BLOCKSTOP: C2RustUnnamed_15 = 313;
pub const ACTOR_EN_GE1: C2RustUnnamed_15 = 312;
pub const ACTOR_ITEM_INBOX: C2RustUnnamed_15 = 311;
pub const ACTOR_EN_BLKOBJ: C2RustUnnamed_15 = 310;
pub const ACTOR_EN_NWC: C2RustUnnamed_15 = 309;
pub const ACTOR_UNSET_134: C2RustUnnamed_15 = 308;
pub const ACTOR_EN_DAIKU: C2RustUnnamed_15 = 307;
pub const ACTOR_EN_TORYO: C2RustUnnamed_15 = 306;
pub const ACTOR_EN_EX_RUPPY: C2RustUnnamed_15 = 305;
pub const ACTOR_EN_GOROIWA: C2RustUnnamed_15 = 304;
pub const ACTOR_EN_YABUSAME_MARK: C2RustUnnamed_15 = 303;
pub const ACTOR_EN_OKARINA_TAG: C2RustUnnamed_15 = 302;
pub const ACTOR_OBJ_HSBLOCK: C2RustUnnamed_15 = 301;
pub const ACTOR_OBJ_LIFT: C2RustUnnamed_15 = 300;
pub const ACTOR_OBJ_ELEVATOR: C2RustUnnamed_15 = 299;
pub const ACTOR_OBJ_SWITCH: C2RustUnnamed_15 = 298;
pub const ACTOR_UNSET_129: C2RustUnnamed_15 = 297;
pub const ACTOR_UNSET_128: C2RustUnnamed_15 = 296;
pub const ACTOR_OBJ_BOMBIWA: C2RustUnnamed_15 = 295;
pub const ACTOR_OBJ_BEAN: C2RustUnnamed_15 = 294;
pub const ACTOR_EN_KUSA: C2RustUnnamed_15 = 293;
pub const ACTOR_EN_DIVING_GAME: C2RustUnnamed_15 = 292;
pub const ACTOR_BG_RELAY_OBJECTS: C2RustUnnamed_15 = 291;
pub const ACTOR_EN_PO_RELAY: C2RustUnnamed_15 = 290;
pub const ACTOR_EN_FZ: C2RustUnnamed_15 = 289;
pub const ACTOR_BG_SPOT07_TAKI: C2RustUnnamed_15 = 288;
pub const ACTOR_BG_SPOT03_TAKI: C2RustUnnamed_15 = 287;
pub const ACTOR_OBJ_ICE_POLY: C2RustUnnamed_15 = 286;
pub const ACTOR_EN_TUBO_TRAP: C2RustUnnamed_15 = 285;
pub const ACTOR_EN_HONOTRAP: C2RustUnnamed_15 = 284;
pub const ACTOR_ELF_MSG: C2RustUnnamed_15 = 283;
pub const ACTOR_EN_DNS: C2RustUnnamed_15 = 282;
pub const ACTOR_DEMO_SHD: C2RustUnnamed_15 = 281;
pub const ACTOR_DEMO_EXT: C2RustUnnamed_15 = 280;
pub const ACTOR_EN_G_SWITCH: C2RustUnnamed_15 = 279;
pub const ACTOR_EN_SKJNEEDLE: C2RustUnnamed_15 = 278;
pub const ACTOR_EN_SKJ: C2RustUnnamed_15 = 277;
pub const ACTOR_DEMO_IK: C2RustUnnamed_15 = 276;
pub const ACTOR_EN_IK: C2RustUnnamed_15 = 275;
pub const ACTOR_EN_WONDER_ITEM: C2RustUnnamed_15 = 274;
pub const ACTOR_OBJ_TSUBO: C2RustUnnamed_15 = 273;
pub const ACTOR_OBJ_KIBAKO: C2RustUnnamed_15 = 272;
pub const ACTOR_ITEM_ETCETERA: C2RustUnnamed_15 = 271;
pub const ACTOR_UNSET_10E: C2RustUnnamed_15 = 270;
pub const ACTOR_UNSET_10D: C2RustUnnamed_15 = 269;
pub const ACTOR_ARROW_LIGHT: C2RustUnnamed_15 = 268;
pub const ACTOR_ARROW_ICE: C2RustUnnamed_15 = 267;
pub const ACTOR_ARROW_FIRE: C2RustUnnamed_15 = 266;
pub const ACTOR_UNSET_109: C2RustUnnamed_15 = 265;
pub const ACTOR_BG_UMAJUMP: C2RustUnnamed_15 = 264;
pub const ACTOR_BG_SPOT15_RRBOX: C2RustUnnamed_15 = 263;
pub const ACTOR_BG_GANON_OTYUKA: C2RustUnnamed_15 = 262;
pub const ACTOR_BG_PO_SYOKUDAI: C2RustUnnamed_15 = 261;
pub const ACTOR_BG_SPOT01_IDOMIZU: C2RustUnnamed_15 = 260;
pub const ACTOR_BG_SPOT01_IDOHASHIRA: C2RustUnnamed_15 = 259;
pub const ACTOR_BG_SPOT01_FUSYA: C2RustUnnamed_15 = 258;
pub const ACTOR_EFF_DUST: C2RustUnnamed_15 = 257;
pub const ACTOR_BG_GATE_SHUTTER: C2RustUnnamed_15 = 256;
pub const ACTOR_OBJ_OSHIHIKI: C2RustUnnamed_15 = 255;
pub const ACTOR_FISHING: C2RustUnnamed_15 = 254;
pub const ACTOR_BG_JYA_KANAAMI: C2RustUnnamed_15 = 253;
pub const ACTOR_BG_JYA_COBRA: C2RustUnnamed_15 = 252;
pub const ACTOR_UNSET_FB: C2RustUnnamed_15 = 251;
pub const ACTOR_BG_JYA_ZURERUKABE: C2RustUnnamed_15 = 250;
pub const ACTOR_BG_JYA_GOROIWA: C2RustUnnamed_15 = 249;
pub const ACTOR_BG_SPOT15_SAKU: C2RustUnnamed_15 = 248;
pub const ACTOR_BG_HAKA_GATE: C2RustUnnamed_15 = 247;
pub const ACTOR_EN_ANUBICE_TAG: C2RustUnnamed_15 = 246;
pub const ACTOR_DEMO_6K: C2RustUnnamed_15 = 245;
pub const ACTOR_MAGIC_DARK: C2RustUnnamed_15 = 244;
pub const ACTOR_UNSET_F3: C2RustUnnamed_15 = 243;
pub const ACTOR_UNSET_F2: C2RustUnnamed_15 = 242;
pub const ACTOR_ITEM_OCARINA: C2RustUnnamed_15 = 241;
pub const ACTOR_EN_ICE_HONO: C2RustUnnamed_15 = 240;
pub const ACTOR_BG_ICE_SHELTER: C2RustUnnamed_15 = 239;
pub const ACTOR_ITEM_SHIELD: C2RustUnnamed_15 = 238;
pub const ACTOR_EN_FR: C2RustUnnamed_15 = 237;
pub const ACTOR_EN_NY: C2RustUnnamed_15 = 236;
pub const ACTOR_UNSET_EB: C2RustUnnamed_15 = 235;
pub const ACTOR_UNSET_EA: C2RustUnnamed_15 = 234;
pub const ACTOR_BOSS_SST: C2RustUnnamed_15 = 233;
pub const ACTOR_BOSS_GANON: C2RustUnnamed_15 = 232;
pub const ACTOR_EN_MA1: C2RustUnnamed_15 = 231;
pub const ACTOR_BG_BDAN_SWITCH: C2RustUnnamed_15 = 230;
pub const ACTOR_BG_SPOT16_DOUGHNUT: C2RustUnnamed_15 = 229;
pub const ACTOR_BG_MORI_IDOMIZU: C2RustUnnamed_15 = 228;
pub const ACTOR_BG_MORI_HASHIRA4: C2RustUnnamed_15 = 227;
pub const ACTOR_BG_MORI_HASHIGO: C2RustUnnamed_15 = 226;
pub const ACTOR_EN_ANUBICE_FIRE: C2RustUnnamed_15 = 225;
pub const ACTOR_EN_ANUBICE: C2RustUnnamed_15 = 224;
pub const ACTOR_EN_BX: C2RustUnnamed_15 = 223;
pub const ACTOR_EN_BA: C2RustUnnamed_15 = 222;
pub const ACTOR_EN_RR: C2RustUnnamed_15 = 221;
pub const ACTOR_BOSS_TW: C2RustUnnamed_15 = 220;
pub const ACTOR_EN_HORSE_GAME_CHECK: C2RustUnnamed_15 = 219;
pub const ACTOR_EN_BOM_CHU: C2RustUnnamed_15 = 218;
pub const ACTOR_EN_MA2: C2RustUnnamed_15 = 217;
pub const ACTOR_UNSET_D8: C2RustUnnamed_15 = 216;
pub const ACTOR_BG_HAKA_WATER: C2RustUnnamed_15 = 215;
pub const ACTOR_BG_ICE_OBJECTS: C2RustUnnamed_15 = 214;
pub const ACTOR_BG_SPOT06_OBJECTS: C2RustUnnamed_15 = 213;
pub const ACTOR_BG_MIZU_UZU: C2RustUnnamed_15 = 212;
pub const ACTOR_OBJ_DEKUJR: C2RustUnnamed_15 = 211;
pub const ACTOR_EN_RU2: C2RustUnnamed_15 = 210;
pub const ACTOR_BG_SPOT08_ICEBLOCK: C2RustUnnamed_15 = 209;
pub const ACTOR_BG_BOMBWALL: C2RustUnnamed_15 = 208;
pub const ACTOR_BG_HIDAN_KOWARERUKABE: C2RustUnnamed_15 = 207;
pub const ACTOR_UNSET_CE: C2RustUnnamed_15 = 206;
pub const ACTOR_BG_SPOT16_BOMBSTONE: C2RustUnnamed_15 = 205;
pub const ACTOR_EN_TR: C2RustUnnamed_15 = 204;
pub const ACTOR_EN_IN: C2RustUnnamed_15 = 203;
pub const ACTOR_DEMO_GO: C2RustUnnamed_15 = 202;
pub const ACTOR_DEMO_SA: C2RustUnnamed_15 = 201;
pub const ACTOR_BG_BDAN_OBJECTS: C2RustUnnamed_15 = 200;
pub const ACTOR_EN_KAREBABA: C2RustUnnamed_15 = 199;
pub const ACTOR_EN_BIGOKUTA: C2RustUnnamed_15 = 198;
pub const ACTOR_EN_SB: C2RustUnnamed_15 = 197;
pub const ACTOR_BOSS_MO: C2RustUnnamed_15 = 196;
pub const ACTOR_EN_NB: C2RustUnnamed_15 = 195;
pub const ACTOR_EN_TANA: C2RustUnnamed_15 = 194;
pub const ACTOR_EN_SYATEKI_MAN: C2RustUnnamed_15 = 193;
pub const ACTOR_EN_SYATEKI_ITM: C2RustUnnamed_15 = 192;
pub const ACTOR_BG_SPOT17_FUNEN: C2RustUnnamed_15 = 191;
pub const ACTOR_BG_HAKA_ZOU: C2RustUnnamed_15 = 190;
pub const ACTOR_BG_HAKA_HUTA: C2RustUnnamed_15 = 189;
pub const ACTOR_BG_HAKA_TRAP: C2RustUnnamed_15 = 188;
pub const ACTOR_BG_HAKA_TUBO: C2RustUnnamed_15 = 187;
pub const ACTOR_BOSS_VA: C2RustUnnamed_15 = 186;
pub const ACTOR_BG_SPOT18_OBJ: C2RustUnnamed_15 = 185;
pub const ACTOR_BG_SPOT09_OBJ: C2RustUnnamed_15 = 184;
pub const ACTOR_MIR_RAY: C2RustUnnamed_15 = 183;
pub const ACTOR_EN_BROB: C2RustUnnamed_15 = 182;
pub const ACTOR_EN_FIRE_ROCK: C2RustUnnamed_15 = 181;
pub const ACTOR_EN_ENCOUNT2: C2RustUnnamed_15 = 180;
pub const ACTOR_EN_HEISHI2: C2RustUnnamed_15 = 179;
pub const ACTOR_UNSET_B2: C2RustUnnamed_15 = 178;
pub const ACTOR_BG_HAKA_SGAMI: C2RustUnnamed_15 = 177;
pub const ACTOR_BG_HAKA_SHIP: C2RustUnnamed_15 = 176;
pub const ACTOR_BG_HAKA_MEGANEBG: C2RustUnnamed_15 = 175;
pub const ACTOR_BG_HAKA_MEGANE: C2RustUnnamed_15 = 174;
pub const ACTOR_EN_VB_BALL: C2RustUnnamed_15 = 173;
pub const ACTOR_BG_VB_SIMA: C2RustUnnamed_15 = 172;
pub const ACTOR_EN_FW: C2RustUnnamed_15 = 171;
pub const ACTOR_DEMO_TRE_LGT: C2RustUnnamed_15 = 170;
pub const ACTOR_DEMO_IM: C2RustUnnamed_15 = 169;
pub const ACTOR_DEMO_DU: C2RustUnnamed_15 = 168;
pub const ACTOR_EN_ENCOUNT1: C2RustUnnamed_15 = 167;
pub const ACTOR_EN_RL: C2RustUnnamed_15 = 166;
pub const ACTOR_EN_DHA: C2RustUnnamed_15 = 165;
pub const ACTOR_EN_DH: C2RustUnnamed_15 = 164;
pub const ACTOR_EN_FD_FIRE: C2RustUnnamed_15 = 163;
pub const ACTOR_BOSS_FD2: C2RustUnnamed_15 = 162;
pub const ACTOR_EN_RU1: C2RustUnnamed_15 = 161;
pub const ACTOR_UNSET_A0: C2RustUnnamed_15 = 160;
pub const ACTOR_MAGIC_FIRE: C2RustUnnamed_15 = 159;
pub const ACTOR_MAGIC_WIND: C2RustUnnamed_15 = 158;
pub const ACTOR_BG_HAKA: C2RustUnnamed_15 = 157;
pub const ACTOR_BG_SPOT02_OBJECTS: C2RustUnnamed_15 = 156;
pub const ACTOR_DOOR_ANA: C2RustUnnamed_15 = 155;
pub const ACTOR_EN_HORSE_LINK_CHILD: C2RustUnnamed_15 = 154;
pub const ACTOR_EN_FD: C2RustUnnamed_15 = 153;
pub const ACTOR_EN_DU: C2RustUnnamed_15 = 152;
pub const ACTOR_OBJECT_KANKYO: C2RustUnnamed_15 = 151;
pub const ACTOR_BOSS_FD: C2RustUnnamed_15 = 150;
pub const ACTOR_EN_SW: C2RustUnnamed_15 = 149;
pub const ACTOR_OBJ_MURE: C2RustUnnamed_15 = 148;
pub const ACTOR_BG_PO_EVENT: C2RustUnnamed_15 = 147;
pub const ACTOR_BG_HEAVY_BLOCK: C2RustUnnamed_15 = 146;
pub const ACTOR_EN_PO_SISTERS: C2RustUnnamed_15 = 145;
pub const ACTOR_EN_RD: C2RustUnnamed_15 = 144;
pub const ACTOR_EN_HEISHI1: C2RustUnnamed_15 = 143;
pub const ACTOR_EN_FLOORMAS: C2RustUnnamed_15 = 142;
pub const ACTOR_BG_HIDAN_FWBIG: C2RustUnnamed_15 = 141;
pub const ACTOR_DEMO_KANKYO: C2RustUnnamed_15 = 140;
pub const ACTOR_DEMO_EFFECT: C2RustUnnamed_15 = 139;
pub const ACTOR_EN_VM: C2RustUnnamed_15 = 138;
pub const ACTOR_BG_MORI_RAKKATENJO: C2RustUnnamed_15 = 137;
pub const ACTOR_BG_MORI_KAITENKABE: C2RustUnnamed_15 = 136;
pub const ACTOR_BG_MORI_ELEVATOR: C2RustUnnamed_15 = 135;
pub const ACTOR_BG_MORI_BIGST: C2RustUnnamed_15 = 134;
pub const ACTOR_EN_TK: C2RustUnnamed_15 = 133;
pub const ACTOR_EN_TA: C2RustUnnamed_15 = 132;
pub const ACTOR_UNSET_83: C2RustUnnamed_15 = 131;
pub const ACTOR_EN_VASE: C2RustUnnamed_15 = 130;
pub const ACTOR_EN_AROW_TRAP: C2RustUnnamed_15 = 129;
pub const ACTOR_EN_TRAP: C2RustUnnamed_15 = 128;
pub const ACTOR_UNSET_7F: C2RustUnnamed_15 = 127;
pub const ACTOR_UNSET_7E: C2RustUnnamed_15 = 126;
pub const ACTOR_EN_PU_BOX: C2RustUnnamed_15 = 125;
pub const ACTOR_EN_LIGHTBOX: C2RustUnnamed_15 = 124;
pub const ACTOR_UNSET_7B: C2RustUnnamed_15 = 123;
pub const ACTOR_UNSET_7A: C2RustUnnamed_15 = 122;
pub const ACTOR_UNSET_79: C2RustUnnamed_15 = 121;
pub const ACTOR_UNSET_78: C2RustUnnamed_15 = 120;
pub const ACTOR_EN_WOOD02: C2RustUnnamed_15 = 119;
pub const ACTOR_UNSET_76: C2RustUnnamed_15 = 118;
pub const ACTOR_UNSET_75: C2RustUnnamed_15 = 117;
pub const ACTOR_UNSET_74: C2RustUnnamed_15 = 116;
pub const ACTOR_UNSET_73: C2RustUnnamed_15 = 115;
pub const ACTOR_EN_BIRD: C2RustUnnamed_15 = 114;
pub const ACTOR_BG_HIDAN_HAMSTEP: C2RustUnnamed_15 = 113;
pub const ACTOR_DOOR_TOKI: C2RustUnnamed_15 = 112;
pub const ACTOR_BG_HIDAN_KOUSI: C2RustUnnamed_15 = 111;
pub const ACTOR_BG_MJIN: C2RustUnnamed_15 = 110;
pub const ACTOR_EN_FHG_FIRE: C2RustUnnamed_15 = 109;
pub const ACTOR_BG_TOKI_SWD: C2RustUnnamed_15 = 108;
pub const ACTOR_EN_YUKABYUN: C2RustUnnamed_15 = 107;
pub const ACTOR_BG_TOKI_HIKARI: C2RustUnnamed_15 = 106;
pub const ACTOR_EN_BB: C2RustUnnamed_15 = 105;
pub const ACTOR_BG_MORI_HINERI: C2RustUnnamed_15 = 104;
pub const ACTOR_EN_FHG: C2RustUnnamed_15 = 103;
pub const ACTOR_ARMS_HOOK: C2RustUnnamed_15 = 102;
pub const ACTOR_BG_MIZU_WATER: C2RustUnnamed_15 = 101;
pub const ACTOR_BG_MIZU_MOVEBG: C2RustUnnamed_15 = 100;
pub const ACTOR_EN_VALI: C2RustUnnamed_15 = 99;
pub const ACTOR_BG_MENKURI_EYE: C2RustUnnamed_15 = 98;
pub const ACTOR_BG_MENKURI_KAITEN: C2RustUnnamed_15 = 97;
pub const ACTOR_EN_DEKUNUTS: C2RustUnnamed_15 = 96;
pub const ACTOR_ITEM_B_HEART: C2RustUnnamed_15 = 95;
pub const ACTOR_OBJ_SYOKUDAI: C2RustUnnamed_15 = 94;
pub const ACTOR_DOOR_WARP1: C2RustUnnamed_15 = 93;
pub const ACTOR_BG_DDAN_KD: C2RustUnnamed_15 = 92;
pub const ACTOR_EN_HORSE_ZELDA: C2RustUnnamed_15 = 91;
pub const ACTOR_EN_JJ: C2RustUnnamed_15 = 90;
pub const ACTOR_BG_BREAKWALL: C2RustUnnamed_15 = 89;
pub const ACTOR_BG_DDAN_JD: C2RustUnnamed_15 = 88;
pub const ACTOR_EN_M_THUNDER: C2RustUnnamed_15 = 87;
pub const ACTOR_EN_M_FIRE1: C2RustUnnamed_15 = 86;
pub const ACTOR_EN_DEKUBABA: C2RustUnnamed_15 = 85;
pub const ACTOR_EN_AM: C2RustUnnamed_15 = 84;
pub const ACTOR_UNSET_53: C2RustUnnamed_15 = 83;
pub const ACTOR_BOSS_GANONDROF: C2RustUnnamed_15 = 82;
pub const ACTOR_BG_YDAN_MARUTA: C2RustUnnamed_15 = 81;
pub const ACTOR_BG_YDAN_HASI: C2RustUnnamed_15 = 80;
pub const ACTOR_EN_OE2: C2RustUnnamed_15 = 79;
pub const ACTOR_BG_HIDAN_FSLIFT: C2RustUnnamed_15 = 78;
pub const ACTOR_EN_ZL2: C2RustUnnamed_15 = 77;
pub const ACTOR_EN_BOMBF: C2RustUnnamed_15 = 76;
pub const ACTOR_EN_MB: C2RustUnnamed_15 = 75;
pub const ACTOR_BG_SPOT00_HANEBASI: C2RustUnnamed_15 = 74;
pub const ACTOR_BG_HIDAN_CURTAIN: C2RustUnnamed_15 = 73;
pub const ACTOR_EN_XC: C2RustUnnamed_15 = 72;
pub const ACTOR_BG_HIDAN_SYOKU: C2RustUnnamed_15 = 71;
pub const ACTOR_BG_HIDAN_SIMA: C2RustUnnamed_15 = 70;
pub const ACTOR_BG_HIDAN_SEKIZOU: C2RustUnnamed_15 = 69;
pub const ACTOR_BG_HIDAN_RSEKIZOU: C2RustUnnamed_15 = 68;
pub const ACTOR_BG_HIDAN_ROCK: C2RustUnnamed_15 = 67;
pub const ACTOR_EN_HORSE_GANON: C2RustUnnamed_15 = 66;
pub const ACTOR_BG_HIDAN_HROCK: C2RustUnnamed_15 = 65;
pub const ACTOR_BG_HIDAN_DALM: C2RustUnnamed_15 = 64;
pub const ACTOR_BG_DODOAGO: C2RustUnnamed_15 = 63;
pub const ACTOR_BG_TREEMOUTH: C2RustUnnamed_15 = 62;
pub const ACTOR_EN_OSSAN: C2RustUnnamed_15 = 61;
pub const ACTOR_EN_HORSE_NORMAL: C2RustUnnamed_15 = 60;
pub const ACTOR_EN_RIVER_SOUND: C2RustUnnamed_15 = 59;
pub const ACTOR_EN_EIYER: C2RustUnnamed_15 = 58;
pub const ACTOR_EN_A_OBJ: C2RustUnnamed_15 = 57;
pub const ACTOR_EN_BW: C2RustUnnamed_15 = 56;
pub const ACTOR_EN_ST: C2RustUnnamed_15 = 55;
pub const ACTOR_UNSET_36: C2RustUnnamed_15 = 54;
pub const ACTOR_EN_TP: C2RustUnnamed_15 = 53;
pub const ACTOR_EN_BILI: C2RustUnnamed_15 = 52;
pub const ACTOR_EN_TORCH2: C2RustUnnamed_15 = 51;
pub const ACTOR_EN_BOOM: C2RustUnnamed_15 = 50;
pub const ACTOR_UNSET_31: C2RustUnnamed_15 = 49;
pub const ACTOR_EN_BDFIRE: C2RustUnnamed_15 = 48;
pub const ACTOR_EN_DODOJR: C2RustUnnamed_15 = 47;
pub const ACTOR_DOOR_SHUTTER: C2RustUnnamed_15 = 46;
pub const ACTOR_EN_BUBBLE: C2RustUnnamed_15 = 45;
pub const ACTOR_BG_PUSHBOX: C2RustUnnamed_15 = 44;
pub const ACTOR_EN_GOMA: C2RustUnnamed_15 = 43;
pub const ACTOR_EN_VIEWER: C2RustUnnamed_15 = 42;
pub const ACTOR_EN_ZL1: C2RustUnnamed_15 = 41;
pub const ACTOR_BOSS_GOMA: C2RustUnnamed_15 = 40;
pub const ACTOR_BOSS_DODONGO: C2RustUnnamed_15 = 39;
pub const ACTOR_EN_HATA: C2RustUnnamed_15 = 38;
pub const ACTOR_EN_ZF: C2RustUnnamed_15 = 37;
pub const ACTOR_EN_SCENE_CHANGE: C2RustUnnamed_15 = 36;
pub const ACTOR_EN_HOLL: C2RustUnnamed_15 = 35;
pub const ACTOR_UNSET_22: C2RustUnnamed_15 = 34;
pub const ACTOR_EN_FISH: C2RustUnnamed_15 = 33;
pub const ACTOR_EN_INSECT: C2RustUnnamed_15 = 32;
pub const ACTOR_UNSET_1F: C2RustUnnamed_15 = 31;
pub const ACTOR_EN_BUTTE: C2RustUnnamed_15 = 30;
pub const ACTOR_EN_PEEHAT: C2RustUnnamed_15 = 29;
pub const ACTOR_EN_REEBA: C2RustUnnamed_15 = 28;
pub const ACTOR_EN_TITE: C2RustUnnamed_15 = 27;
pub const ACTOR_UNSET_1A: C2RustUnnamed_15 = 26;
pub const ACTOR_EN_NIW: C2RustUnnamed_15 = 25;
pub const ACTOR_EN_ELF: C2RustUnnamed_15 = 24;
pub const ACTOR_UNSET_17: C2RustUnnamed_15 = 23;
pub const ACTOR_EN_ARROW: C2RustUnnamed_15 = 22;
pub const ACTOR_EN_ITEM00: C2RustUnnamed_15 = 21;
pub const ACTOR_EN_HORSE: C2RustUnnamed_15 = 20;
pub const ACTOR_EN_FIREFLY: C2RustUnnamed_15 = 19;
pub const ACTOR_EN_DODONGO: C2RustUnnamed_15 = 18;
pub const ACTOR_EN_WALLMAS: C2RustUnnamed_15 = 17;
pub const ACTOR_EN_BOM: C2RustUnnamed_15 = 16;
pub const ACTOR_BG_YDAN_SP: C2RustUnnamed_15 = 15;
pub const ACTOR_EN_OKUTA: C2RustUnnamed_15 = 14;
pub const ACTOR_EN_POH: C2RustUnnamed_15 = 13;
pub const ACTOR_BG_HIDAN_FIREWALL: C2RustUnnamed_15 = 12;
pub const ACTOR_BG_DY_YOSEIZO: C2RustUnnamed_15 = 11;
pub const ACTOR_EN_BOX: C2RustUnnamed_15 = 10;
pub const ACTOR_EN_DOOR: C2RustUnnamed_15 = 9;
pub const ACTOR_EN_LIGHT: C2RustUnnamed_15 = 8;
pub const ACTOR_EN_PART: C2RustUnnamed_15 = 7;
pub const ACTOR_UNSET_6: C2RustUnnamed_15 = 6;
pub const ACTOR_UNSET_5: C2RustUnnamed_15 = 5;
pub const ACTOR_EN_GIRLA: C2RustUnnamed_15 = 4;
pub const ACTOR_UNSET_3: C2RustUnnamed_15 = 3;
pub const ACTOR_EN_TEST: C2RustUnnamed_15 = 2;
pub const ACTOR_UNSET_1: C2RustUnnamed_15 = 1;
pub const ACTOR_PLAYER: C2RustUnnamed_15 = 0;
pub type C2RustUnnamed_16 = libc::c_uint;
pub const OBJECT_ID_MAX: C2RustUnnamed_16 = 402;
pub const OBJECT_ZL4: C2RustUnnamed_16 = 401;
pub const OBJECT_TIMEBLOCK: C2RustUnnamed_16 = 400;
pub const OBJECT_OUKE_HAKA: C2RustUnnamed_16 = 399;
pub const OBJECT_DOOR_KILLER: C2RustUnnamed_16 = 398;
pub const OBJECT_GI_SWORD_1: C2RustUnnamed_16 = 397;
pub const OBJECT_COB: C2RustUnnamed_16 = 396;
pub const OBJECT_COW: C2RustUnnamed_16 = 395;
pub const OBJECT_BWALL: C2RustUnnamed_16 = 394;
pub const OBJECT_PS: C2RustUnnamed_16 = 393;
pub const OBJECT_GS: C2RustUnnamed_16 = 392;
pub const OBJECT_HAKA_DOOR: C2RustUnnamed_16 = 391;
pub const OBJECT_GEFF: C2RustUnnamed_16 = 390;
pub const OBJECT_GJ: C2RustUnnamed_16 = 389;
pub const OBJECT_SKB: C2RustUnnamed_16 = 388;
pub const OBJECT_WF: C2RustUnnamed_16 = 387;
pub const OBJECT_MU: C2RustUnnamed_16 = 386;
pub const OBJECT_SPOT01_MATOYAB: C2RustUnnamed_16 = 385;
pub const OBJECT_SPOT01_MATOYA: C2RustUnnamed_16 = 384;
pub const OBJECT_GI_RUPY: C2RustUnnamed_16 = 383;
pub const OBJECT_GANON_ANIME3: C2RustUnnamed_16 = 382;
pub const OBJECT_GANON_ANIME2: C2RustUnnamed_16 = 381;
pub const OBJECT_GANON_ANIME1: C2RustUnnamed_16 = 380;
pub const OBJECT_GI_DEKUPOUCH: C2RustUnnamed_16 = 379;
pub const OBJECT_EFC_DOUGHNUT: C2RustUnnamed_16 = 378;
pub const OBJECT_DEMO_KEKKAI: C2RustUnnamed_16 = 377;
pub const OBJECT_BOWL: C2RustUnnamed_16 = 376;
pub const OBJECT_GI_SOUL: C2RustUnnamed_16 = 375;
pub const OBJECT_GI_GHOST: C2RustUnnamed_16 = 374;
pub const OBJECT_GI_BUTTERFLY: C2RustUnnamed_16 = 373;
pub const OBJECT_GI_INSECT: C2RustUnnamed_16 = 372;
pub const OBJECT_GI_FIRE: C2RustUnnamed_16 = 371;
pub const OBJECT_DNK: C2RustUnnamed_16 = 370;
pub const OBJECT_DNS: C2RustUnnamed_16 = 369;
pub const OBJECT_KIBAKO2: C2RustUnnamed_16 = 368;
pub const OBJECT_SPOT11_OBJ: C2RustUnnamed_16 = 367;
pub const OBJECT_UNSET_16E: C2RustUnnamed_16 = 366;
pub const OBJECT_JYA_DOOR: C2RustUnnamed_16 = 365;
pub const OBJECT_JYA_IRON: C2RustUnnamed_16 = 364;
pub const OBJECT_DOG: C2RustUnnamed_16 = 363;
pub const OBJECT_GR: C2RustUnnamed_16 = 362;
pub const OBJECT_GELDB: C2RustUnnamed_16 = 361;
pub const OBJECT_SHOPNUTS: C2RustUnnamed_16 = 360;
pub const OBJECT_GLA: C2RustUnnamed_16 = 359;
pub const OBJECT_SPOT00_BREAK: C2RustUnnamed_16 = 358;
pub const OBJECT_RS: C2RustUnnamed_16 = 357;
pub const OBJECT_HINTNUTS: C2RustUnnamed_16 = 356;
pub const OBJECT_BOMBIWA: C2RustUnnamed_16 = 355;
pub const OBJECT_SPOT12_OBJ: C2RustUnnamed_16 = 354;
pub const OBJECT_SPOT05_OBJECTS: C2RustUnnamed_16 = 353;
pub const OBJECT_BG: C2RustUnnamed_16 = 352;
pub const OBJECT_BIGOKUTA: C2RustUnnamed_16 = 351;
pub const OBJECT_SSH: C2RustUnnamed_16 = 350;
pub const OBJECT_GI_GODDESS: C2RustUnnamed_16 = 349;
pub const OBJECT_GI_SUTARU: C2RustUnnamed_16 = 348;
pub const OBJECT_FISH: C2RustUnnamed_16 = 347;
pub const OBJECT_EC: C2RustUnnamed_16 = 346;
pub const OBJECT_DS2: C2RustUnnamed_16 = 345;
pub const OBJECT_GI_M_ARROW: C2RustUnnamed_16 = 344;
pub const OBJECT_GI_HOVERBOOTS: C2RustUnnamed_16 = 343;
pub const OBJECT_ZG: C2RustUnnamed_16 = 342;
pub const OBJECT_TS: C2RustUnnamed_16 = 341;
pub const OBJECT_KA: C2RustUnnamed_16 = 340;
pub const OBJECT_GANON2: C2RustUnnamed_16 = 339;
pub const OBJECT_GI_GERUDOMASK: C2RustUnnamed_16 = 338;
pub const OBJECT_GI_ZORAMASK: C2RustUnnamed_16 = 337;
pub const OBJECT_GI_GOLONMASK: C2RustUnnamed_16 = 336;
pub const OBJECT_ZL2_ANIME2: C2RustUnnamed_16 = 335;
pub const OBJECT_ZL2_ANIME1: C2RustUnnamed_16 = 334;
pub const OBJECT_EFC_ERUPC: C2RustUnnamed_16 = 333;
pub const OBJECT_GT: C2RustUnnamed_16 = 332;
pub const OBJECT_DOOR_GERUDO: C2RustUnnamed_16 = 331;
pub const OBJECT_MAG: C2RustUnnamed_16 = 330;
pub const OBJECT_GI_FROG: C2RustUnnamed_16 = 329;
pub const OBJECT_GI_SOLDOUT: C2RustUnnamed_16 = 328;
pub const OBJECT_GI_BRACELET: C2RustUnnamed_16 = 327;
pub const OBJECT_GI_PRESCRIPTION: C2RustUnnamed_16 = 326;
pub const OBJECT_CS: C2RustUnnamed_16 = 325;
pub const OBJECT_JS: C2RustUnnamed_16 = 324;
pub const OBJECT_GI_BROKENSWORD: C2RustUnnamed_16 = 323;
pub const OBJECT_GI_TICKETSTONE: C2RustUnnamed_16 = 322;
pub const OBJECT_GI_MUSHROOM: C2RustUnnamed_16 = 321;
pub const OBJECT_GI_POWDER: C2RustUnnamed_16 = 320;
pub const OBJECT_GI_EYE_LOTION: C2RustUnnamed_16 = 319;
pub const OBJECT_OS: C2RustUnnamed_16 = 318;
pub const OBJECT_FA: C2RustUnnamed_16 = 317;
pub const OBJECT_MM: C2RustUnnamed_16 = 316;
pub const OBJECT_STREAM: C2RustUnnamed_16 = 315;
pub const OBJECT_SIOFUKI: C2RustUnnamed_16 = 314;
pub const OBJECT_GANON_OBJECTS: C2RustUnnamed_16 = 313;
pub const OBJECT_GI_TRUTH_MASK: C2RustUnnamed_16 = 312;
pub const OBJECT_GI_RABIT_MASK: C2RustUnnamed_16 = 311;
pub const OBJECT_GI_SKJ_MASK: C2RustUnnamed_16 = 310;
pub const OBJECT_GI_REDEAD_MASK: C2RustUnnamed_16 = 309;
pub const OBJECT_GI_KI_TAN_MASK: C2RustUnnamed_16 = 308;
pub const OBJECT_FU: C2RustUnnamed_16 = 307;
pub const OBJECT_MK: C2RustUnnamed_16 = 306;
pub const OBJECT_OWL: C2RustUnnamed_16 = 305;
pub const OBJECT_GJYO_OBJECTS: C2RustUnnamed_16 = 304;
pub const OBJECT_KANBAN: C2RustUnnamed_16 = 303;
pub const OBJECT_GI_COIN: C2RustUnnamed_16 = 302;
pub const OBJECT_GI_GLOVES: C2RustUnnamed_16 = 301;
pub const OBJECT_TSUBO: C2RustUnnamed_16 = 300;
pub const OBJECT_KUSA: C2RustUnnamed_16 = 299;
pub const OBJECT_LIGHTSWITCH: C2RustUnnamed_16 = 298;
pub const OBJECT_INGATE: C2RustUnnamed_16 = 297;
pub const OBJECT_HS: C2RustUnnamed_16 = 296;
pub const OBJECT_MS: C2RustUnnamed_16 = 295;
pub const OBJECT_GM: C2RustUnnamed_16 = 294;
pub const OBJECT_BLKOBJ: C2RustUnnamed_16 = 293;
pub const OBJECT_NWC: C2RustUnnamed_16 = 292;
pub const OBJECT_UNSET_123: C2RustUnnamed_16 = 291;
pub const OBJECT_DAIKU: C2RustUnnamed_16 = 290;
pub const OBJECT_TORYO: C2RustUnnamed_16 = 289;
pub const OBJECT_UNSET_120: C2RustUnnamed_16 = 288;
pub const OBJECT_GOROIWA: C2RustUnnamed_16 = 287;
pub const OBJECT_MAMENOKI: C2RustUnnamed_16 = 286;
pub const OBJECT_D_LIFT: C2RustUnnamed_16 = 285;
pub const OBJECT_D_HSBLOCK: C2RustUnnamed_16 = 284;
pub const OBJECT_D_ELEVATOR: C2RustUnnamed_16 = 283;
pub const OBJECT_GND_MAGIC: C2RustUnnamed_16 = 282;
pub const OBJECT_GI_SEED: C2RustUnnamed_16 = 281;
pub const OBJECT_GI_BOOTS_2: C2RustUnnamed_16 = 280;
pub const OBJECT_YABUSAME_POINT: C2RustUnnamed_16 = 279;
pub const OBJECT_GE1: C2RustUnnamed_16 = 278;
pub const OBJECT_BOB: C2RustUnnamed_16 = 277;
pub const OBJECT_FZ: C2RustUnnamed_16 = 276;
pub const OBJECT_SPOT07_OBJECT: C2RustUnnamed_16 = 275;
pub const OBJECT_SPOT03_OBJECT: C2RustUnnamed_16 = 274;
pub const OBJECT_BOJ: C2RustUnnamed_16 = 273;
pub const OBJECT_ANE: C2RustUnnamed_16 = 272;
pub const OBJECT_DS: C2RustUnnamed_16 = 271;
pub const OBJECT_GI_OCARINA_0: C2RustUnnamed_16 = 270;
pub const OBJECT_BBA: C2RustUnnamed_16 = 269;
pub const OBJECT_BJI: C2RustUnnamed_16 = 268;
pub const OBJECT_GI_BOTTLE_LETTER: C2RustUnnamed_16 = 267;
pub const OBJECT_SKJ: C2RustUnnamed_16 = 266;
pub const OBJECT_GI_NIWATORI: C2RustUnnamed_16 = 265;
pub const OBJECT_CNE: C2RustUnnamed_16 = 264;
pub const OBJECT_AHG: C2RustUnnamed_16 = 263;
pub const OBJECT_IK: C2RustUnnamed_16 = 262;
pub const OBJECT_AOB: C2RustUnnamed_16 = 261;
pub const OBJECT_MASTERZOORA: C2RustUnnamed_16 = 260;
pub const OBJECT_MASTERGOLON: C2RustUnnamed_16 = 259;
pub const OBJECT_MASTERKOKIRIHEAD: C2RustUnnamed_16 = 258;
pub const OBJECT_MASTERKOKIRI: C2RustUnnamed_16 = 257;
pub const OBJECT_UMAJUMP: C2RustUnnamed_16 = 256;
pub const OBJECT_KZ: C2RustUnnamed_16 = 255;
pub const OBJECT_ZO: C2RustUnnamed_16 = 254;
pub const OBJECT_KW1: C2RustUnnamed_16 = 253;
pub const OBJECT_KM1: C2RustUnnamed_16 = 252;
pub const OBJECT_MD: C2RustUnnamed_16 = 251;
pub const OBJECT_MD_UNUSED: C2RustUnnamed_16 = 250;
pub const OBJECT_SPOT01_OBJECTS: C2RustUnnamed_16 = 249;
pub const OBJECT_GI_LONGSWORD: C2RustUnnamed_16 = 248;
pub const OBJECT_GI_GRASS: C2RustUnnamed_16 = 247;
pub const OBJECT_GI_HAMMER: C2RustUnnamed_16 = 246;
pub const OBJECT_GI_SAW: C2RustUnnamed_16 = 245;
pub const OBJECT_GI_FISH: C2RustUnnamed_16 = 244;
pub const OBJECT_GI_BEAN: C2RustUnnamed_16 = 243;
pub const OBJECT_GI_CLOTHES: C2RustUnnamed_16 = 242;
pub const OBJECT_JYA_OBJ: C2RustUnnamed_16 = 241;
pub const OBJECT_SPOT15_OBJ: C2RustUnnamed_16 = 240;
pub const OBJECT_GI_LETTER: C2RustUnnamed_16 = 239;
pub const OBJECT_GI_SHIELD_3: C2RustUnnamed_16 = 238;
pub const OBJECT_DEMO_6K: C2RustUnnamed_16 = 237;
pub const OBJECT_ANI: C2RustUnnamed_16 = 236;
pub const OBJECT_GI_LIQUID: C2RustUnnamed_16 = 235;
pub const OBJECT_GI_GLASSES: C2RustUnnamed_16 = 234;
pub const OBJECT_GI_BOW: C2RustUnnamed_16 = 233;
pub const OBJECT_GI_BOOMERANG: C2RustUnnamed_16 = 232;
pub const OBJECT_GI_PACHINKO: C2RustUnnamed_16 = 231;
pub const OBJECT_FR: C2RustUnnamed_16 = 230;
pub const OBJECT_NY: C2RustUnnamed_16 = 229;
pub const OBJECT_UNSET_E4: C2RustUnnamed_16 = 228;
pub const OBJECT_NY_UNUSED: C2RustUnnamed_16 = 227;
pub const OBJECT_SST: C2RustUnnamed_16 = 226;
pub const OBJECT_GANON: C2RustUnnamed_16 = 225;
pub const OBJECT_MA1: C2RustUnnamed_16 = 224;
pub const OBJECT_GI_MILK: C2RustUnnamed_16 = 223;
pub const OBJECT_GI_OCARINA: C2RustUnnamed_16 = 222;
pub const OBJECT_GI_HOOKSHOT: C2RustUnnamed_16 = 221;
pub const OBJECT_GI_SHIELD_2: C2RustUnnamed_16 = 220;
pub const OBJECT_GI_SCALE: C2RustUnnamed_16 = 219;
pub const OBJECT_GI_EGG: C2RustUnnamed_16 = 218;
pub const OBJECT_GI_BOMB_2: C2RustUnnamed_16 = 217;
pub const OBJECT_GI_ARROW: C2RustUnnamed_16 = 216;
pub const OBJECT_GI_GERUDO: C2RustUnnamed_16 = 215;
pub const OBJECT_ANUBICE: C2RustUnnamed_16 = 214;
pub const OBJECT_BXA: C2RustUnnamed_16 = 213;
pub const OBJECT_RR: C2RustUnnamed_16 = 212;
pub const OBJECT_TW: C2RustUnnamed_16 = 211;
pub const OBJECT_HNI: C2RustUnnamed_16 = 210;
pub const OBJECT_GI_PURSE: C2RustUnnamed_16 = 209;
pub const OBJECT_MA2: C2RustUnnamed_16 = 208;
pub const OBJECT_OF1S: C2RustUnnamed_16 = 207;
pub const OBJECT_GI_BOMB_1: C2RustUnnamed_16 = 206;
pub const OBJECT_GI_MAGICPOT: C2RustUnnamed_16 = 205;
pub const OBJECT_DEKUJR: C2RustUnnamed_16 = 204;
pub const OBJECT_GI_SHIELD_1: C2RustUnnamed_16 = 203;
pub const OBJECT_RU2: C2RustUnnamed_16 = 202;
pub const OBJECT_OF1D_MAP: C2RustUnnamed_16 = 201;
pub const OBJECT_GI_MAP: C2RustUnnamed_16 = 200;
pub const OBJECT_GI_STICK: C2RustUnnamed_16 = 199;
pub const OBJECT_GI_BOTTLE: C2RustUnnamed_16 = 198;
pub const OBJECT_OS_ANIME: C2RustUnnamed_16 = 197;
pub const OBJECT_OE4S: C2RustUnnamed_16 = 196;
pub const OBJECT_OE1S: C2RustUnnamed_16 = 195;
pub const OBJECT_SPOT16_OBJ: C2RustUnnamed_16 = 194;
pub const OBJECT_TR: C2RustUnnamed_16 = 193;
pub const OBJECT_IN: C2RustUnnamed_16 = 192;
pub const OBJECT_GI_BOMBPOUCH: C2RustUnnamed_16 = 191;
pub const OBJECT_GI_ARROWCASE: C2RustUnnamed_16 = 190;
pub const OBJECT_GI_HEARTS: C2RustUnnamed_16 = 189;
pub const OBJECT_SA: C2RustUnnamed_16 = 188;
pub const OBJECT_GI_NUTS: C2RustUnnamed_16 = 187;
pub const OBJECT_GI_MEDAL: C2RustUnnamed_16 = 186;
pub const OBJECT_GI_BOSSKEY: C2RustUnnamed_16 = 185;
pub const OBJECT_GI_COMPASS: C2RustUnnamed_16 = 184;
pub const OBJECT_GI_HEART: C2RustUnnamed_16 = 183;
pub const OBJECT_GI_MELODY: C2RustUnnamed_16 = 182;
pub const OBJECT_SB: C2RustUnnamed_16 = 181;
pub const OBJECT_MO: C2RustUnnamed_16 = 180;
pub const OBJECT_NB: C2RustUnnamed_16 = 179;
pub const OBJECT_SHOP_DUNGEN: C2RustUnnamed_16 = 178;
pub const OBJECT_SPOT17_OBJ: C2RustUnnamed_16 = 177;
pub const OBJECT_BDOOR: C2RustUnnamed_16 = 176;
pub const OBJECT_SPOT18_OBJ: C2RustUnnamed_16 = 175;
pub const OBJECT_SPOT09_OBJ: C2RustUnnamed_16 = 174;
pub const OBJECT_GI_JEWEL: C2RustUnnamed_16 = 173;
pub const OBJECT_BROB: C2RustUnnamed_16 = 172;
pub const OBJECT_MIR_RAY: C2RustUnnamed_16 = 171;
pub const OBJECT_GI_KEY: C2RustUnnamed_16 = 170;
pub const OBJECT_DEMO_TRE_LGT: C2RustUnnamed_16 = 169;
pub const OBJECT_EFC_TW: C2RustUnnamed_16 = 168;
pub const OBJECT_RL: C2RustUnnamed_16 = 167;
pub const OBJECT_DH: C2RustUnnamed_16 = 166;
pub const OBJECT_FD2: C2RustUnnamed_16 = 165;
pub const OBJECT_SYOKUDAI: C2RustUnnamed_16 = 164;
pub const OBJECT_RU1: C2RustUnnamed_16 = 163;
pub const OBJECT_HAKA: C2RustUnnamed_16 = 162;
pub const OBJECT_SPOT02_OBJECTS: C2RustUnnamed_16 = 161;
pub const OBJECT_HORSE_LINK_CHILD: C2RustUnnamed_16 = 160;
pub const OBJECT_MEDAL: C2RustUnnamed_16 = 159;
pub const OBJECT_FW: C2RustUnnamed_16 = 158;
pub const OBJECT_DU: C2RustUnnamed_16 = 157;
pub const OBJECT_FD: C2RustUnnamed_16 = 156;
pub const OBJECT_GNDD: C2RustUnnamed_16 = 155;
pub const OBJECT_HEAVY_OBJECT: C2RustUnnamed_16 = 154;
pub const OBJECT_PO_SISTERS: C2RustUnnamed_16 = 153;
pub const OBJECT_RD: C2RustUnnamed_16 = 152;
pub const OBJECT_SD: C2RustUnnamed_16 = 151;
pub const OBJECT_BDAN_OBJECTS: C2RustUnnamed_16 = 150;
pub const OBJECT_TRIFORCE_SPOT: C2RustUnnamed_16 = 149;
pub const OBJECT_LIGHT_RING: C2RustUnnamed_16 = 148;
pub const OBJECT_GOD_LGT: C2RustUnnamed_16 = 147;
pub const OBJECT_EFC_STAR_FIELD: C2RustUnnamed_16 = 146;
pub const OBJECT_EFC_LGT_SHOWER: C2RustUnnamed_16 = 145;
pub const OBJECT_EFC_FLASH: C2RustUnnamed_16 = 144;
pub const OBJECT_EFC_FIRE_BALL: C2RustUnnamed_16 = 143;
pub const OBJECT_EFC_CRYSTAL_LIGHT: C2RustUnnamed_16 = 142;
pub const OBJECT_HAKACH_OBJECTS: C2RustUnnamed_16 = 141;
pub const OBJECT_BV: C2RustUnnamed_16 = 140;
pub const OBJECT_VM: C2RustUnnamed_16 = 139;
pub const OBJECT_XC: C2RustUnnamed_16 = 138;
pub const OBJECT_TK: C2RustUnnamed_16 = 137;
pub const OBJECT_TA: C2RustUnnamed_16 = 136;
pub const OBJECT_IM: C2RustUnnamed_16 = 135;
pub const OBJECT_VASE: C2RustUnnamed_16 = 134;
pub const OBJECT_TRAP: C2RustUnnamed_16 = 133;
pub const OBJECT_UNSET_84: C2RustUnnamed_16 = 132;
pub const OBJECT_UNSET_83: C2RustUnnamed_16 = 131;
pub const OBJECT_PU_BOX: C2RustUnnamed_16 = 130;
pub const OBJECT_LIGHTBOX: C2RustUnnamed_16 = 129;
pub const OBJECT_UNSET_80: C2RustUnnamed_16 = 128;
pub const OBJECT_UNSET_7F: C2RustUnnamed_16 = 127;
pub const OBJECT_UNSET_7E: C2RustUnnamed_16 = 126;
pub const OBJECT_UNSET_7D: C2RustUnnamed_16 = 125;
pub const OBJECT_WOOD02: C2RustUnnamed_16 = 124;
pub const OBJECT_UNSET_7B: C2RustUnnamed_16 = 123;
pub const OBJECT_UNSET_7A: C2RustUnnamed_16 = 122;
pub const OBJECT_UNSET_79: C2RustUnnamed_16 = 121;
pub const OBJECT_UNSET_78: C2RustUnnamed_16 = 120;
pub const OBJECT_BIRD: C2RustUnnamed_16 = 119;
pub const OBJECT_HATA: C2RustUnnamed_16 = 118;
pub const OBJECT_WARP2: C2RustUnnamed_16 = 117;
pub const OBJECT_SPOT08_OBJ: C2RustUnnamed_16 = 116;
pub const OBJECT_MORI_TEX: C2RustUnnamed_16 = 115;
pub const OBJECT_MORI_OBJECTS: C2RustUnnamed_16 = 114;
pub const OBJECT_MORI_HINERI2A: C2RustUnnamed_16 = 113;
pub const OBJECT_MORI_HINERI2: C2RustUnnamed_16 = 112;
pub const OBJECT_MORI_HINERI1A: C2RustUnnamed_16 = 111;
pub const OBJECT_PO_COMPOSER: C2RustUnnamed_16 = 110;
pub const OBJECT_PO_FIELD: C2RustUnnamed_16 = 109;
pub const OBJECT_RELAY_OBJECTS: C2RustUnnamed_16 = 108;
pub const OBJECT_ICE_OBJECTS: C2RustUnnamed_16 = 107;
pub const OBJECT_SPOT06_OBJECTS: C2RustUnnamed_16 = 106;
pub const OBJECT_HAKA_OBJECTS: C2RustUnnamed_16 = 105;
pub const OBJECT_MJIN_OKA: C2RustUnnamed_16 = 104;
pub const OBJECT_MJIN_WIND: C2RustUnnamed_16 = 103;
pub const OBJECT_MJIN_SOUL: C2RustUnnamed_16 = 102;
pub const OBJECT_MJIN_ICE: C2RustUnnamed_16 = 101;
pub const OBJECT_MJIN_FLAME: C2RustUnnamed_16 = 100;
pub const OBJECT_MJIN_DARK: C2RustUnnamed_16 = 99;
pub const OBJECT_MJIN_FLASH: C2RustUnnamed_16 = 98;
pub const OBJECT_MJIN: C2RustUnnamed_16 = 97;
pub const OBJECT_ZL2: C2RustUnnamed_16 = 96;
pub const OBJECT_YUKABYUN: C2RustUnnamed_16 = 95;
pub const OBJECT_TOKI_OBJECTS: C2RustUnnamed_16 = 94;
pub const OBJECT_BB: C2RustUnnamed_16 = 93;
pub const OBJECT_MORI_HINERI1: C2RustUnnamed_16 = 92;
pub const OBJECT_OSSAN: C2RustUnnamed_16 = 91;
pub const OBJECT_FHG: C2RustUnnamed_16 = 90;
pub const OBJECT_MIZU_OBJECTS: C2RustUnnamed_16 = 89;
pub const OBJECT_OA11: C2RustUnnamed_16 = 88;
pub const OBJECT_OA10: C2RustUnnamed_16 = 87;
pub const OBJECT_VALI: C2RustUnnamed_16 = 86;
pub const OBJECT_OE12: C2RustUnnamed_16 = 85;
pub const OBJECT_OE11: C2RustUnnamed_16 = 84;
pub const OBJECT_OE10: C2RustUnnamed_16 = 83;
pub const OBJECT_OE9: C2RustUnnamed_16 = 82;
pub const OBJECT_OE8: C2RustUnnamed_16 = 81;
pub const OBJECT_OE7: C2RustUnnamed_16 = 80;
pub const OBJECT_OE6: C2RustUnnamed_16 = 79;
pub const OBJECT_OE5: C2RustUnnamed_16 = 78;
pub const OBJECT_MENKURI_OBJECTS: C2RustUnnamed_16 = 77;
pub const OBJECT_OE4: C2RustUnnamed_16 = 76;
pub const OBJECT_OE3: C2RustUnnamed_16 = 75;
pub const OBJECT_DEKUNUTS: C2RustUnnamed_16 = 74;
pub const OBJECT_B_HEART: C2RustUnnamed_16 = 73;
pub const OBJECT_WARP1: C2RustUnnamed_16 = 72;
pub const OBJECT_OPENING_DEMO1: C2RustUnnamed_16 = 71;
pub const OBJECT_HORSE_ZELDA: C2RustUnnamed_16 = 70;
pub const OBJECT_OB4: C2RustUnnamed_16 = 69;
pub const OBJECT_OB3: C2RustUnnamed_16 = 68;
pub const OBJECT_OB2: C2RustUnnamed_16 = 67;
pub const OBJECT_OA9: C2RustUnnamed_16 = 66;
pub const OBJECT_OA8: C2RustUnnamed_16 = 65;
pub const OBJECT_JJ: C2RustUnnamed_16 = 64;
pub const OBJECT_OA7: C2RustUnnamed_16 = 63;
pub const OBJECT_OA6: C2RustUnnamed_16 = 62;
pub const OBJECT_OA5: C2RustUnnamed_16 = 61;
pub const OBJECT_OA4: C2RustUnnamed_16 = 60;
pub const OBJECT_OA3: C2RustUnnamed_16 = 59;
pub const OBJECT_UNSET_3A: C2RustUnnamed_16 = 58;
pub const OBJECT_DEKUBABA: C2RustUnnamed_16 = 57;
pub const OBJECT_AM: C2RustUnnamed_16 = 56;
pub const OBJECT_GND: C2RustUnnamed_16 = 55;
pub const OBJECT_YDAN_OBJECTS: C2RustUnnamed_16 = 54;
pub const OBJECT_OE2: C2RustUnnamed_16 = 53;
pub const OBJECT_OE_ANIME: C2RustUnnamed_16 = 52;
pub const OBJECT_OE1: C2RustUnnamed_16 = 51;
pub const OBJECT_SK2: C2RustUnnamed_16 = 50;
pub const OBJECT_BOMBF: C2RustUnnamed_16 = 49;
pub const OBJECT_MB: C2RustUnnamed_16 = 48;
pub const OBJECT_SPOT00_OBJECTS: C2RustUnnamed_16 = 47;
pub const OBJECT_OA2: C2RustUnnamed_16 = 46;
pub const OBJECT_HORSE_GANON: C2RustUnnamed_16 = 45;
pub const OBJECT_HIDAN_OBJECTS: C2RustUnnamed_16 = 44;
pub const OBJECT_DDAN_OBJECTS: C2RustUnnamed_16 = 43;
pub const OBJECT_SPOT04_OBJECTS: C2RustUnnamed_16 = 42;
pub const OBJECT_O_ANIME: C2RustUnnamed_16 = 41;
pub const OBJECT_OB1: C2RustUnnamed_16 = 40;
pub const OBJECT_HORSE_NORMAL: C2RustUnnamed_16 = 39;
pub const OBJECT_EI: C2RustUnnamed_16 = 38;
pub const OBJECT_BW: C2RustUnnamed_16 = 37;
pub const OBJECT_ST: C2RustUnnamed_16 = 36;
pub const OBJECT_OA1: C2RustUnnamed_16 = 35;
pub const OBJECT_TP: C2RustUnnamed_16 = 34;
pub const OBJECT_BL: C2RustUnnamed_16 = 33;
pub const OBJECT_TORCH2: C2RustUnnamed_16 = 32;
pub const OBJECT_DODOJR: C2RustUnnamed_16 = 31;
pub const OBJECT_GOL: C2RustUnnamed_16 = 30;
pub const OBJECT_ZL1: C2RustUnnamed_16 = 29;
pub const OBJECT_GOMA: C2RustUnnamed_16 = 28;
pub const OBJECT_ZF: C2RustUnnamed_16 = 27;
pub const OBJECT_HORSE: C2RustUnnamed_16 = 26;
pub const OBJECT_KINGDODONGO: C2RustUnnamed_16 = 25;
pub const OBJECT_PEEHAT: C2RustUnnamed_16 = 24;
pub const OBJECT_REEBA: C2RustUnnamed_16 = 23;
pub const OBJECT_TITE: C2RustUnnamed_16 = 22;
pub const OBJECT_LINK_CHILD: C2RustUnnamed_16 = 21;
pub const OBJECT_LINK_BOY: C2RustUnnamed_16 = 20;
pub const OBJECT_NIW: C2RustUnnamed_16 = 19;
pub const OBJECT_BUBBLE: C2RustUnnamed_16 = 18;
pub const OBJECT_UNSET_11: C2RustUnnamed_16 = 17;
pub const OBJECT_UNSET_10: C2RustUnnamed_16 = 16;
pub const OBJECT_FIRE: C2RustUnnamed_16 = 15;
pub const OBJECT_BOX: C2RustUnnamed_16 = 14;
pub const OBJECT_FIREFLY: C2RustUnnamed_16 = 13;
pub const OBJECT_DODONGO: C2RustUnnamed_16 = 12;
pub const OBJECT_WALLMASTER: C2RustUnnamed_16 = 11;
pub const OBJECT_DY_OBJ: C2RustUnnamed_16 = 10;
pub const OBJECT_POH: C2RustUnnamed_16 = 9;
pub const OBJECT_CROW: C2RustUnnamed_16 = 8;
pub const OBJECT_OKUTA: C2RustUnnamed_16 = 7;
pub const OBJECT_HUMAN: C2RustUnnamed_16 = 6;
pub const OBJECT_UNSET_5: C2RustUnnamed_16 = 5;
pub const OBJECT_UNSET_4: C2RustUnnamed_16 = 4;
pub const OBJECT_GAMEPLAY_DANGEON_KEEP: C2RustUnnamed_16 = 3;
pub const OBJECT_GAMEPLAY_FIELD_KEEP: C2RustUnnamed_16 = 2;
pub const OBJECT_GAMEPLAY_KEEP: C2RustUnnamed_16 = 1;
pub const OBJECT_INVALID: C2RustUnnamed_16 = 0;
pub type C2RustUnnamed_17 = libc::c_uint;
pub const CS_STATE_UNSKIPPABLE_EXEC: C2RustUnnamed_17 = 4;
pub const CS_STATE_UNSKIPPABLE_INIT: C2RustUnnamed_17 = 3;
pub const CS_STATE_SKIPPABLE_EXEC: C2RustUnnamed_17 = 2;
pub const CS_STATE_SKIPPABLE_INIT: C2RustUnnamed_17 = 1;
pub const CS_STATE_IDLE: C2RustUnnamed_17 = 0;
pub type C2RustUnnamed_18 = libc::c_uint;
pub const SCENE_ID_MAX: C2RustUnnamed_18 = 110;
pub const SCENE_TESTROOM: C2RustUnnamed_18 = 109;
pub const SCENE_SASATEST: C2RustUnnamed_18 = 108;
pub const SCENE_HAIRAL_NIWA2: C2RustUnnamed_18 = 107;
pub const SCENE_SUTARU: C2RustUnnamed_18 = 106;
pub const SCENE_SYOTES2: C2RustUnnamed_18 = 105;
pub const SCENE_SYOTES: C2RustUnnamed_18 = 104;
pub const SCENE_DEPTH_TEST: C2RustUnnamed_18 = 103;
pub const SCENE_BESITU: C2RustUnnamed_18 = 102;
pub const SCENE_TEST01: C2RustUnnamed_18 = 101;
pub const SCENE_GANON_TOU: C2RustUnnamed_18 = 100;
pub const SCENE_SPOT20: C2RustUnnamed_18 = 99;
pub const SCENE_SPOT18: C2RustUnnamed_18 = 98;
pub const SCENE_SPOT17: C2RustUnnamed_18 = 97;
pub const SCENE_SPOT16: C2RustUnnamed_18 = 96;
pub const SCENE_SPOT15: C2RustUnnamed_18 = 95;
pub const SCENE_SPOT13: C2RustUnnamed_18 = 94;
pub const SCENE_SPOT12: C2RustUnnamed_18 = 93;
pub const SCENE_SPOT11: C2RustUnnamed_18 = 92;
pub const SCENE_SPOT10: C2RustUnnamed_18 = 91;
pub const SCENE_SPOT09: C2RustUnnamed_18 = 90;
pub const SCENE_SPOT08: C2RustUnnamed_18 = 89;
pub const SCENE_SPOT07: C2RustUnnamed_18 = 88;
pub const SCENE_SPOT06: C2RustUnnamed_18 = 87;
pub const SCENE_SPOT05: C2RustUnnamed_18 = 86;
pub const SCENE_SPOT04: C2RustUnnamed_18 = 85;
pub const SCENE_SPOT03: C2RustUnnamed_18 = 84;
pub const SCENE_SPOT02: C2RustUnnamed_18 = 83;
pub const SCENE_SPOT01: C2RustUnnamed_18 = 82;
pub const SCENE_SPOT00: C2RustUnnamed_18 = 81;
pub const SCENE_KINSUTA: C2RustUnnamed_18 = 80;
pub const SCENE_GANON_DEMO: C2RustUnnamed_18 = 79;
pub const SCENE_MAHOUYA: C2RustUnnamed_18 = 78;
pub const SCENE_MIHARIGOYA: C2RustUnnamed_18 = 77;
pub const SCENE_SOUKO: C2RustUnnamed_18 = 76;
pub const SCENE_BOWLING: C2RustUnnamed_18 = 75;
pub const SCENE_NAKANIWA: C2RustUnnamed_18 = 74;
pub const SCENE_TURIBORI: C2RustUnnamed_18 = 73;
pub const SCENE_HAKASITARELAY: C2RustUnnamed_18 = 72;
pub const SCENE_HIRAL_DEMO: C2RustUnnamed_18 = 71;
pub const SCENE_HAIRAL_NIWA_N: C2RustUnnamed_18 = 70;
pub const SCENE_HAIRAL_NIWA: C2RustUnnamed_18 = 69;
pub const SCENE_KENJYANOMA: C2RustUnnamed_18 = 68;
pub const SCENE_TOKINOMA: C2RustUnnamed_18 = 67;
pub const SCENE_SYATEKIJYOU: C2RustUnnamed_18 = 66;
pub const SCENE_HAKAANA_OUKE: C2RustUnnamed_18 = 65;
pub const SCENE_HAKAANA2: C2RustUnnamed_18 = 64;
pub const SCENE_HAKAANA: C2RustUnnamed_18 = 63;
pub const SCENE_KAKUSIANA: C2RustUnnamed_18 = 62;
pub const SCENE_YOUSEI_IZUMI_YOKO: C2RustUnnamed_18 = 61;
pub const SCENE_YOUSEI_IZUMI_TATE: C2RustUnnamed_18 = 60;
pub const SCENE_DAIYOUSEI_IZUMI: C2RustUnnamed_18 = 59;
pub const SCENE_HUT: C2RustUnnamed_18 = 58;
pub const SCENE_TENT: C2RustUnnamed_18 = 57;
pub const SCENE_HYLIA_LABO: C2RustUnnamed_18 = 56;
pub const SCENE_LABO: C2RustUnnamed_18 = 55;
pub const SCENE_MALON_STABLE: C2RustUnnamed_18 = 54;
pub const SCENE_IMPA: C2RustUnnamed_18 = 53;
pub const SCENE_LINK_HOME: C2RustUnnamed_18 = 52;
pub const SCENE_FACE_SHOP: C2RustUnnamed_18 = 51;
pub const SCENE_NIGHT_SHOP: C2RustUnnamed_18 = 50;
pub const SCENE_ALLEY_SHOP: C2RustUnnamed_18 = 49;
pub const SCENE_DRAG: C2RustUnnamed_18 = 48;
pub const SCENE_ZOORA: C2RustUnnamed_18 = 47;
pub const SCENE_GOLON: C2RustUnnamed_18 = 46;
pub const SCENE_KOKIRI_SHOP: C2RustUnnamed_18 = 45;
pub const SCENE_SHOP1: C2RustUnnamed_18 = 44;
pub const SCENE_KAKARIKO3: C2RustUnnamed_18 = 43;
pub const SCENE_KAKARIKO: C2RustUnnamed_18 = 42;
pub const SCENE_KOKIRI_HOME5: C2RustUnnamed_18 = 41;
pub const SCENE_KOKIRI_HOME4: C2RustUnnamed_18 = 40;
pub const SCENE_KOKIRI_HOME3: C2RustUnnamed_18 = 39;
pub const SCENE_KOKIRI_HOME: C2RustUnnamed_18 = 38;
pub const SCENE_SHRINE_R: C2RustUnnamed_18 = 37;
pub const SCENE_SHRINE_N: C2RustUnnamed_18 = 36;
pub const SCENE_SHRINE: C2RustUnnamed_18 = 35;
pub const SCENE_MARKET_RUINS: C2RustUnnamed_18 = 34;
pub const SCENE_MARKET_NIGHT: C2RustUnnamed_18 = 33;
pub const SCENE_MARKET_DAY: C2RustUnnamed_18 = 32;
pub const SCENE_MARKET_ALLEY_N: C2RustUnnamed_18 = 31;
pub const SCENE_MARKET_ALLEY: C2RustUnnamed_18 = 30;
pub const SCENE_ENRUI: C2RustUnnamed_18 = 29;
pub const SCENE_ENTRA_N: C2RustUnnamed_18 = 28;
pub const SCENE_ENTRA: C2RustUnnamed_18 = 27;
pub const SCENE_GANON_FINAL: C2RustUnnamed_18 = 26;
pub const SCENE_GANON_BOSS: C2RustUnnamed_18 = 25;
pub const SCENE_HAKADAN_BS: C2RustUnnamed_18 = 24;
pub const SCENE_JYASINBOSS: C2RustUnnamed_18 = 23;
pub const SCENE_MIZUSIN_BS: C2RustUnnamed_18 = 22;
pub const SCENE_FIRE_BS: C2RustUnnamed_18 = 21;
pub const SCENE_MORIBOSSROOM: C2RustUnnamed_18 = 20;
pub const SCENE_BDAN_BOSS: C2RustUnnamed_18 = 19;
pub const SCENE_DDAN_BOSS: C2RustUnnamed_18 = 18;
pub const SCENE_YDAN_BOSS: C2RustUnnamed_18 = 17;
pub const SCENE_TAKARAYA: C2RustUnnamed_18 = 16;
pub const SCENE_GANONTIKA_SONOGO: C2RustUnnamed_18 = 15;
pub const SCENE_GANON_SONOGO: C2RustUnnamed_18 = 14;
pub const SCENE_GANONTIKA: C2RustUnnamed_18 = 13;
pub const SCENE_GERUDOWAY: C2RustUnnamed_18 = 12;
pub const SCENE_MEN: C2RustUnnamed_18 = 11;
pub const SCENE_GANON: C2RustUnnamed_18 = 10;
pub const SCENE_ICE_DOUKUTO: C2RustUnnamed_18 = 9;
pub const SCENE_HAKADANCH: C2RustUnnamed_18 = 8;
pub const SCENE_HAKADAN: C2RustUnnamed_18 = 7;
pub const SCENE_JYASINZOU: C2RustUnnamed_18 = 6;
pub const SCENE_MIZUSIN: C2RustUnnamed_18 = 5;
pub const SCENE_HIDAN: C2RustUnnamed_18 = 4;
pub const SCENE_BMORI1: C2RustUnnamed_18 = 3;
pub const SCENE_BDAN: C2RustUnnamed_18 = 2;
pub const SCENE_DDAN: C2RustUnnamed_18 = 1;
pub const SCENE_YDAN: C2RustUnnamed_18 = 0;
pub type C2RustUnnamed_19 = libc::c_uint;
pub const GID_MAX: C2RustUnnamed_19 = 117;
pub const GID_SKULL_TOKEN_2: C2RustUnnamed_19 = 116;
pub const GID_SWORD_KOKIRI: C2RustUnnamed_19 = 115;
pub const GID_BULLET_BAG_50: C2RustUnnamed_19 = 114;
pub const GID_RUPEE_GOLD: C2RustUnnamed_19 = 113;
pub const GID_RUPEE_PURPLE: C2RustUnnamed_19 = 112;
pub const GID_BIG_POE: C2RustUnnamed_19 = 111;
pub const GID_RUPEE_RED: C2RustUnnamed_19 = 110;
pub const GID_RUPEE_BLUE: C2RustUnnamed_19 = 109;
pub const GID_RUPEE_GREEN: C2RustUnnamed_19 = 108;
pub const GID_BULLET_BAG: C2RustUnnamed_19 = 107;
pub const GID_FAIRY: C2RustUnnamed_19 = 106;
pub const GID_POE: C2RustUnnamed_19 = 105;
pub const GID_BUTTERFLY: C2RustUnnamed_19 = 104;
pub const GID_BUG: C2RustUnnamed_19 = 103;
pub const GID_BLUE_FIRE: C2RustUnnamed_19 = 102;
pub const GID_NAYRUS_LOVE: C2RustUnnamed_19 = 101;
pub const GID_FARORES_WIND: C2RustUnnamed_19 = 100;
pub const GID_DINS_FIRE: C2RustUnnamed_19 = 99;
pub const GID_SKULL_TOKEN: C2RustUnnamed_19 = 98;
pub const GID_ARROW_LIGHT: C2RustUnnamed_19 = 97;
pub const GID_ARROW_ICE: C2RustUnnamed_19 = 96;
pub const GID_ARROW_FIRE: C2RustUnnamed_19 = 95;
pub const GID_BOOTS_HOVER: C2RustUnnamed_19 = 94;
pub const GID_COJIRO: C2RustUnnamed_19 = 93;
pub const GID_MASK_GERUDO: C2RustUnnamed_19 = 92;
pub const GID_MASK_ZORA: C2RustUnnamed_19 = 91;
pub const GID_MASK_GORON: C2RustUnnamed_19 = 90;
pub const GID_FROG: C2RustUnnamed_19 = 89;
pub const GID_SOLDOUT: C2RustUnnamed_19 = 88;
pub const GID_BRACELET: C2RustUnnamed_19 = 87;
pub const GID_PRESCRIPTION: C2RustUnnamed_19 = 86;
pub const GID_SWORD_BROKEN: C2RustUnnamed_19 = 85;
pub const GID_CLAIM_CHECK: C2RustUnnamed_19 = 84;
pub const GID_ODD_MUSHROOM: C2RustUnnamed_19 = 83;
pub const GID_ODD_POTION: C2RustUnnamed_19 = 82;
pub const GID_EYEDROPS: C2RustUnnamed_19 = 81;
pub const GID_MASK_TRUTH: C2RustUnnamed_19 = 80;
pub const GID_MASK_BUNNY: C2RustUnnamed_19 = 79;
pub const GID_MASK_SKULL: C2RustUnnamed_19 = 78;
pub const GID_NCOIN_BLUE: C2RustUnnamed_19 = 77;
pub const GID_NCOIN_GREEN: C2RustUnnamed_19 = 76;
pub const GID_NCOIN_RED: C2RustUnnamed_19 = 75;
pub const GID_NCOIN_YELLOW: C2RustUnnamed_19 = 74;
pub const GID_GAUNTLETS_GOLD: C2RustUnnamed_19 = 73;
pub const GID_GAUNTLETS_SILVER: C2RustUnnamed_19 = 72;
pub const GID_SEEDS: C2RustUnnamed_19 = 71;
pub const GID_BOOTS_IRON: C2RustUnnamed_19 = 70;
pub const GID_OCARINA_FAIRY: C2RustUnnamed_19 = 69;
pub const GID_LETTER_RUTO: C2RustUnnamed_19 = 68;
pub const GID_CHICKEN: C2RustUnnamed_19 = 67;
pub const GID_SWORD_BGS: C2RustUnnamed_19 = 66;
pub const GID_GRASS: C2RustUnnamed_19 = 65;
pub const GID_HAMMER: C2RustUnnamed_19 = 64;
pub const GID_SAW: C2RustUnnamed_19 = 63;
pub const GID_FISH: C2RustUnnamed_19 = 62;
pub const GID_BEAN: C2RustUnnamed_19 = 61;
pub const GID_TUNIC_ZORA: C2RustUnnamed_19 = 60;
pub const GID_TUNIC_GORON: C2RustUnnamed_19 = 59;
pub const GID_LETTER_ZELDA: C2RustUnnamed_19 = 58;
pub const GID_SHIELD_MIRROR: C2RustUnnamed_19 = 57;
pub const GID_POTION_BLUE: C2RustUnnamed_19 = 56;
pub const GID_POTION_RED: C2RustUnnamed_19 = 55;
pub const GID_POTION_GREEN: C2RustUnnamed_19 = 54;
pub const GID_LENS: C2RustUnnamed_19 = 53;
pub const GID_BOW: C2RustUnnamed_19 = 52;
pub const GID_BOOMERANG: C2RustUnnamed_19 = 51;
pub const GID_SLINGSHOT: C2RustUnnamed_19 = 50;
pub const GID_MASK_SPOOKY: C2RustUnnamed_19 = 49;
pub const GID_MASK_KEATON: C2RustUnnamed_19 = 48;
pub const GID_MILK: C2RustUnnamed_19 = 47;
pub const GID_OCARINA_TIME: C2RustUnnamed_19 = 46;
pub const GID_LONGSHOT: C2RustUnnamed_19 = 45;
pub const GID_HOOKSHOT: C2RustUnnamed_19 = 44;
pub const GID_SHIELD_HYLIAN: C2RustUnnamed_19 = 43;
pub const GID_SCALE_GOLDEN: C2RustUnnamed_19 = 42;
pub const GID_SCALE_SILVER: C2RustUnnamed_19 = 41;
pub const GID_EGG: C2RustUnnamed_19 = 40;
pub const GID_BOMBCHU: C2RustUnnamed_19 = 39;
pub const GID_ARROWS_LARGE: C2RustUnnamed_19 = 38;
pub const GID_ARROWS_MEDIUM: C2RustUnnamed_19 = 37;
pub const GID_ARROWS_SMALL: C2RustUnnamed_19 = 36;
pub const GID_GERUDO_CARD: C2RustUnnamed_19 = 35;
pub const GID_WALLET_GIANT: C2RustUnnamed_19 = 34;
pub const GID_WALLET_ADULT: C2RustUnnamed_19 = 33;
pub const GID_STONE_OF_AGONY: C2RustUnnamed_19 = 32;
pub const GID_BOMB: C2RustUnnamed_19 = 31;
pub const GID_MAGIC_LARGE: C2RustUnnamed_19 = 30;
pub const GID_MAGIC_SMALL: C2RustUnnamed_19 = 29;
pub const GID_SHIELD_DEKU: C2RustUnnamed_19 = 28;
pub const GID_DUNGEON_MAP: C2RustUnnamed_19 = 27;
pub const GID_STICK: C2RustUnnamed_19 = 26;
pub const GID_BOMB_BAG_40: C2RustUnnamed_19 = 25;
pub const GID_BOMB_BAG_30: C2RustUnnamed_19 = 24;
pub const GID_BOMB_BAG_20: C2RustUnnamed_19 = 23;
pub const GID_QUIVER_50: C2RustUnnamed_19 = 22;
pub const GID_QUIVER_40: C2RustUnnamed_19 = 21;
pub const GID_QUIVER_30: C2RustUnnamed_19 = 20;
pub const GID_HEART_PIECE: C2RustUnnamed_19 = 19;
pub const GID_HEART_CONTAINER: C2RustUnnamed_19 = 18;
pub const GID_NUTS: C2RustUnnamed_19 = 17;
pub const GID_MEDALLION_LIGHT: C2RustUnnamed_19 = 16;
pub const GID_MEDALLION_SHADOW: C2RustUnnamed_19 = 15;
pub const GID_MEDALLION_SPIRIT: C2RustUnnamed_19 = 14;
pub const GID_MEDALLION_WATER: C2RustUnnamed_19 = 13;
pub const GID_MEDALLION_FIRE: C2RustUnnamed_19 = 12;
pub const GID_MEDALLION_FOREST: C2RustUnnamed_19 = 11;
pub const GID_COMPASS: C2RustUnnamed_19 = 10;
pub const GID_KEY_BOSS: C2RustUnnamed_19 = 9;
pub const GID_HEART: C2RustUnnamed_19 = 8;
pub const GID_SONG_PRELUDE: C2RustUnnamed_19 = 7;
pub const GID_SONG_NOCTURNE: C2RustUnnamed_19 = 6;
pub const GID_SONG_REQUIEM: C2RustUnnamed_19 = 5;
pub const GID_SONG_SERENADE: C2RustUnnamed_19 = 4;
pub const GID_SONG_BOLERO: C2RustUnnamed_19 = 3;
pub const GID_SONG_MINUET: C2RustUnnamed_19 = 2;
pub const GID_KEY_SMALL: C2RustUnnamed_19 = 1;
pub const GID_BOTTLE: C2RustUnnamed_19 = 0;
pub type C2RustUnnamed_20 = libc::c_uint;
pub const MTXMODE_APPLY: C2RustUnnamed_20 = 1;
pub const MTXMODE_NEW: C2RustUnnamed_20 = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct DemoEffect {
    pub actor: Actor,
    pub skelCurve: SkelAnimeCurve,
    pub initObjectBankIndex: u8_0,
    pub jewelDisplayList: *mut Gfx,
    pub jewelHolderDisplayList: *mut Gfx,
    pub primXluColor: [u8_0; 3],
    pub envXluColor: [u8_0; 3],
    pub primOpaColor: [u8_0; 3],
    pub envOpaColor: [u8_0; 3],
    pub c2rust_unnamed: C2RustUnnamed_21,
    pub effectFlags: s16,
    pub csActionId: s16,
    pub jewelCsRotation: Vec3s,
    pub initUpdateFunc: DemoEffectFunc,
    pub initDrawFunc: ActorFunc,
    pub updateFunc: DemoEffectFunc,
}
pub type DemoEffectFunc
    =
    Option<unsafe extern "C" fn(_: *mut DemoEffect, _: *mut GlobalContext)
               -> ()>;
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_21 {
    pub fireBall: DemoEffectFireBall,
    pub blueOrb: DemoEffectBlueOrb,
    pub light: DemoEffectLight,
    pub lgtShower: DemoEffectLgtShower,
    pub godLgt: DemoEffectGodLgt,
    pub lightRing: DemoEffectLightRing,
    pub triforceSpot: DemoEffectTriforceSpot,
    pub getItem: DemoEffectGetItem,
    pub timeWarp: DemoEffectTimeWarp,
    pub jewel: DemoEffectJewel,
    pub dust: DemoEffectDust,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct DemoEffectDust {
    pub timer: u8_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct DemoEffectJewel {
    pub type_0: u8_0,
    pub isPositionInit: u8_0,
    pub alpha: u8_0,
    pub timer: s16,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct DemoEffectTimeWarp {
    pub pad: u8_0,
    pub pad2: u8_0,
    pub pad3: u8_0,
    pub shrinkTimer: s16,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct DemoEffectGetItem {
    pub isPositionInit: u8_0,
    pub isLoaded: u8_0,
    pub drawId: u8_0,
    pub rotation: s16,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct DemoEffectTriforceSpot {
    pub triforceSpotOpacity: u8_0,
    pub lightColumnOpacity: u8_0,
    pub crystalLightOpacity: u8_0,
    pub rotation: s16,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct DemoEffectLightRing {
    pub timerIncrement: u8_0,
    pub alpha: u8_0,
    pub pad: u8_0,
    pub timer: s16,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct DemoEffectGodLgt {
    pub type_0: u8_0,
    pub lightRingSpawnDelay: u8_0,
    pub rotation: u8_0,
    pub lightRingSpawnTimer: s16,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct DemoEffectLgtShower {
    pub alpha: u8_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct DemoEffectLight {
    pub alpha: u8_0,
    pub scaleFlag: u8_0,
    pub flicker: u8_0,
    pub rotation: s16,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct DemoEffectBlueOrb {
    pub alpha: u8_0,
    pub scale: u8_0,
    pub pad: u8_0,
    pub rotation: s16,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct DemoEffectFireBall {
    pub timer: u8_0,
}
pub type C2RustUnnamed_22 = libc::c_uint;
pub const DEMO_EFFECT_MAX_TYPE: C2RustUnnamed_22 = 26;
pub const DEMO_EFFECT_TIMEWARP_TIMEBLOCK_SMALL: C2RustUnnamed_22 = 25;
pub const DEMO_EFFECT_TIMEWARP_TIMEBLOCK_LARGE: C2RustUnnamed_22 = 24;
pub const DEMO_EFFECT_LIGHTARROW: C2RustUnnamed_22 = 23;
pub const DEMO_EFFECT_DUST: C2RustUnnamed_22 = 22;
pub const DEMO_EFFECT_JEWEL_ZORA: C2RustUnnamed_22 = 21;
pub const DEMO_EFFECT_JEWEL_GORON: C2RustUnnamed_22 = 20;
pub const DEMO_EFFECT_JEWEL_KOKIRI: C2RustUnnamed_22 = 19;
pub const DEMO_EFFECT_LIGHT: C2RustUnnamed_22 = 18;
pub const DEMO_EFFECT_LIGHTRING_TRIFORCE: C2RustUnnamed_22 = 17;
pub const DEMO_EFFECT_LIGHTRING_SHRINKING: C2RustUnnamed_22 = 16;
pub const DEMO_EFFECT_TIMEWARP_MASTERSWORD: C2RustUnnamed_22 = 15;
pub const DEMO_EFFECT_MEDAL_LIGHT: C2RustUnnamed_22 = 14;
pub const DEMO_EFFECT_MEDAL_SHADOW: C2RustUnnamed_22 = 13;
pub const DEMO_EFFECT_MEDAL_SPIRIT: C2RustUnnamed_22 = 12;
pub const DEMO_EFFECT_MEDAL_FOREST: C2RustUnnamed_22 = 11;
pub const DEMO_EFFECT_MEDAL_WATER: C2RustUnnamed_22 = 10;
pub const DEMO_EFFECT_MEDAL_FIRE: C2RustUnnamed_22 = 9;
pub const DEMO_EFFECT_TRIFORCE_SPOT: C2RustUnnamed_22 = 8;
pub const DEMO_EFFECT_LIGHTRING_EXPANDING: C2RustUnnamed_22 = 7;
pub const DEMO_EFFECT_GOD_LGT_FARORE: C2RustUnnamed_22 = 6;
pub const DEMO_EFFECT_GOD_LGT_NAYRU: C2RustUnnamed_22 = 5;
pub const DEMO_EFFECT_GOD_LGT_DIN: C2RustUnnamed_22 = 4;
pub const DEMO_EFFECT_LGT_SHOWER: C2RustUnnamed_22 = 3;
pub const DEMO_EFFECT_BLUE_ORB: C2RustUnnamed_22 = 2;
pub const DEMO_EFFECT_FIRE_BALL: C2RustUnnamed_22 = 1;
pub const DEMO_EFFECT_CRYSTAL_LIGHT: C2RustUnnamed_22 = 0;
pub type C2RustUnnamed_23 = libc::c_uint;
pub const DEMO_EFFECT_LIGHT_GREEN2: C2RustUnnamed_23 = 6;
pub const DEMO_EFFECT_LIGHT_PURPLE: C2RustUnnamed_23 = 5;
pub const DEMO_EFFECT_LIGHT_YELLOW: C2RustUnnamed_23 = 4;
pub const DEMO_EFFECT_LIGHT_ORANGE: C2RustUnnamed_23 = 3;
pub const DEMO_EFFECT_LIGHT_GREEN: C2RustUnnamed_23 = 2;
pub const DEMO_EFFECT_LIGHT_BLUE: C2RustUnnamed_23 = 1;
pub const DEMO_EFFECT_LIGHT_RED: C2RustUnnamed_23 = 0;
pub type C2RustUnnamed_24 = libc::c_uint;
pub const GOD_LGT_FARORE: C2RustUnnamed_24 = 2;
pub const GOD_LGT_NAYRU: C2RustUnnamed_24 = 1;
pub const GOD_LGT_DIN: C2RustUnnamed_24 = 0;
#[no_mangle]
pub static mut Demo_Effect_InitVars: ActorInit =
    unsafe {
        {
            let mut init =
                ActorInit{id: ACTOR_DEMO_EFFECT as libc::c_int as s16,
                          category: ACTORCAT_BG as libc::c_int as u8_0,
                          flags:
                              ((1 as libc::c_int) << 4 as libc::c_int |
                                   (1 as libc::c_int) << 5 as libc::c_int) as
                                  u32_0,
                          objectId:
                              OBJECT_GAMEPLAY_KEEP as libc::c_int as s16,
                          instanceSize:
                              ::std::mem::size_of::<DemoEffect>() as
                                  libc::c_ulong,
                          init:
                              ::std::mem::transmute::<Option<unsafe extern "C" fn(_:
                                                                                      *mut Actor,
                                                                                  _:
                                                                                      *mut GlobalContext)
                                                                 -> ()>,
                                                      ActorFunc>(Some(DemoEffect_Init
                                                                          as
                                                                          unsafe extern "C" fn(_:
                                                                                                   *mut Actor,
                                                                                               _:
                                                                                                   *mut GlobalContext)
                                                                              ->
                                                                                  ())),
                          destroy:
                              ::std::mem::transmute::<Option<unsafe extern "C" fn(_:
                                                                                      *mut Actor,
                                                                                  _:
                                                                                      *mut GlobalContext)
                                                                 -> ()>,
                                                      ActorFunc>(Some(DemoEffect_Destroy
                                                                          as
                                                                          unsafe extern "C" fn(_:
                                                                                                   *mut Actor,
                                                                                               _:
                                                                                                   *mut GlobalContext)
                                                                              ->
                                                                                  ())),
                          update:
                              ::std::mem::transmute::<Option<unsafe extern "C" fn(_:
                                                                                      *mut Actor,
                                                                                  _:
                                                                                      *mut GlobalContext)
                                                                 -> ()>,
                                                      ActorFunc>(Some(DemoEffect_Update
                                                                          as
                                                                          unsafe extern "C" fn(_:
                                                                                                   *mut Actor,
                                                                                               _:
                                                                                                   *mut GlobalContext)
                                                                              ->
                                                                                  ())),
                          draw: None,};
            init
        }
    };
// This variable assures only one jewel will play SFX
static mut sSfxJewelId: [s16; 1] = [0 as libc::c_int as s16];
// The object used by the effectType
static mut sEffectTypeObjects: [s16; 26] =
    [OBJECT_EFC_CRYSTAL_LIGHT as libc::c_int as s16,
     OBJECT_EFC_FIRE_BALL as libc::c_int as s16,
     OBJECT_GAMEPLAY_KEEP as libc::c_int as s16,
     OBJECT_EFC_LGT_SHOWER as libc::c_int as s16,
     OBJECT_GOD_LGT as libc::c_int as s16,
     OBJECT_GOD_LGT as libc::c_int as s16,
     OBJECT_GOD_LGT as libc::c_int as s16,
     OBJECT_LIGHT_RING as libc::c_int as s16,
     OBJECT_TRIFORCE_SPOT as libc::c_int as s16,
     OBJECT_GI_MEDAL as libc::c_int as s16,
     OBJECT_GI_MEDAL as libc::c_int as s16,
     OBJECT_GI_MEDAL as libc::c_int as s16,
     OBJECT_GI_MEDAL as libc::c_int as s16,
     OBJECT_GI_MEDAL as libc::c_int as s16,
     OBJECT_GI_MEDAL as libc::c_int as s16,
     OBJECT_EFC_TW as libc::c_int as s16,
     OBJECT_LIGHT_RING as libc::c_int as s16,
     OBJECT_LIGHT_RING as libc::c_int as s16,
     OBJECT_GAMEPLAY_KEEP as libc::c_int as s16,
     OBJECT_GI_JEWEL as libc::c_int as s16,
     OBJECT_GI_JEWEL as libc::c_int as s16,
     OBJECT_GI_JEWEL as libc::c_int as s16,
     OBJECT_GI_JEWEL as libc::c_int as s16,
     OBJECT_GI_M_ARROW as libc::c_int as s16,
     OBJECT_EFC_TW as libc::c_int as s16,
     OBJECT_EFC_TW as libc::c_int as s16];
static mut sTimewarpVertexSizeIndices: [u8_0; 21] =
    [1 as libc::c_int as u8_0, 1 as libc::c_int as u8_0,
     2 as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
     1 as libc::c_int as u8_0, 1 as libc::c_int as u8_0,
     2 as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
     1 as libc::c_int as u8_0, 2 as libc::c_int as u8_0,
     0 as libc::c_int as u8_0, 2 as libc::c_int as u8_0,
     1 as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
     1 as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
     2 as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
     2 as libc::c_int as u8_0, 2 as libc::c_int as u8_0,
     0 as libc::c_int as u8_0];
static mut sJewelSparkleColors: [[Color_RGB8; 2]; 5] =
    [[{
          let mut init =
              Color_RGB8{r: 255 as libc::c_int as u8_0,
                         g: 255 as libc::c_int as u8_0,
                         b: 255 as libc::c_int as u8_0,};
          init
      },
      {
          let mut init =
              Color_RGB8{r: 100 as libc::c_int as u8_0,
                         g: 255 as libc::c_int as u8_0,
                         b: 0 as libc::c_int as u8_0,};
          init
      }],
     [{
          let mut init =
              Color_RGB8{r: 255 as libc::c_int as u8_0,
                         g: 255 as libc::c_int as u8_0,
                         b: 255 as libc::c_int as u8_0,};
          init
      },
      {
          let mut init =
              Color_RGB8{r: 200 as libc::c_int as u8_0,
                         g: 0 as libc::c_int as u8_0,
                         b: 150 as libc::c_int as u8_0,};
          init
      }],
     [{
          let mut init =
              Color_RGB8{r: 255 as libc::c_int as u8_0,
                         g: 255 as libc::c_int as u8_0,
                         b: 255 as libc::c_int as u8_0,};
          init
      },
      {
          let mut init =
              Color_RGB8{r: 0 as libc::c_int as u8_0,
                         g: 100 as libc::c_int as u8_0,
                         b: 255 as libc::c_int as u8_0,};
          init
      }],
     [{
          let mut init =
              Color_RGB8{r: 0 as libc::c_int as u8_0,
                         g: 0 as libc::c_int as u8_0,
                         b: 0 as libc::c_int as u8_0,};
          init
      },
      {
          let mut init =
              Color_RGB8{r: 0 as libc::c_int as u8_0,
                         g: 0 as libc::c_int as u8_0,
                         b: 0 as libc::c_int as u8_0,};
          init
      }],
     [{
          let mut init =
              Color_RGB8{r: 223 as libc::c_int as u8_0,
                         g: 0 as libc::c_int as u8_0,
                         b: 0 as libc::c_int as u8_0,};
          init
      },
      {
          let mut init =
              Color_RGB8{r: 0 as libc::c_int as u8_0,
                         g: 0 as libc::c_int as u8_0,
                         b: 0 as libc::c_int as u8_0,};
          init
      }]];
/* *
 * Sets up the update function.
 */
#[no_mangle]
pub unsafe extern "C" fn DemoEffect_SetupUpdate(mut this: *mut DemoEffect,
                                                mut updateFunc:
                                                    DemoEffectFunc) {
    (*this).updateFunc = updateFunc;
}
/* *
 * Gives a number on the range of 0.0f - 1.0f representing current cutscene action completion percentage.
 */
#[no_mangle]
pub unsafe extern "C" fn DemoEffect_InterpolateCsFrames(mut globalCtx:
                                                            *mut GlobalContext,
                                                        mut csActionId: s32)
 -> f32_0 {
    let mut interpolated: f32_0 =
        Environment_LerpWeight((*(*globalCtx).csCtx.npcActions[csActionId as
                                                                   usize]).endFrame,
                               (*(*globalCtx).csCtx.npcActions[csActionId as
                                                                   usize]).startFrame,
                               (*globalCtx).csCtx.frames);
    if interpolated > 1.0f32 { interpolated = 1.0f32 }
    return interpolated;
}
/* *
 * Initializes information for Jewel/Spritual Stone actors.
 */
#[no_mangle]
pub unsafe extern "C" fn DemoEffect_InitJewel(mut globalCtx:
                                                  *mut GlobalContext,
                                              mut this: *mut DemoEffect) {
    (*this).initDrawFunc =
        Some(DemoEffect_DrawJewel as
                 unsafe extern "C" fn(_: *mut Actor, _: *mut GlobalContext)
                     -> ());
    if !(gSaveContext.linkAge == 0 as libc::c_int) {
        (*this).initUpdateFunc =
            Some(DemoEffect_UpdateJewelChild as
                     unsafe extern "C" fn(_: *mut DemoEffect,
                                          _: *mut GlobalContext) -> ())
    } else {
        (*this).initUpdateFunc =
            Some(DemoEffect_UpdateJewelAdult as
                     unsafe extern "C" fn(_: *mut DemoEffect,
                                          _: *mut GlobalContext) -> ())
    }
    if (*globalCtx).sceneNum as libc::c_int == SCENE_TOKINOMA as libc::c_int {
        Actor_SetScale(&mut (*this).actor, 0.35f32);
    } else { Actor_SetScale(&mut (*this).actor, 0.10f32); }
    (*this).csActionId = 1 as libc::c_int as s16;
    (*this).actor.shape.rot.x = 16384 as libc::c_int as s16;
    DemoEffect_InitJewelColor(this);
    (*this).c2rust_unnamed.jewel.alpha = 0 as libc::c_int as u8_0;
    (*this).jewelCsRotation.z = 0 as libc::c_int as s16;
    (*this).jewelCsRotation.y = (*this).jewelCsRotation.z;
    (*this).jewelCsRotation.x = (*this).jewelCsRotation.y;
    sSfxJewelId[0 as libc::c_int as usize] = 0 as libc::c_int as s16;
}
/* *
 * Initializes information for Get Item actors.
 * These are the Medal and Light Arrow actors.
 */
#[no_mangle]
pub unsafe extern "C" fn DemoEffect_InitGetItem(mut this: *mut DemoEffect) {
    (*this).c2rust_unnamed.getItem.isPositionInit = 0 as libc::c_int as u8_0;
    (*this).c2rust_unnamed.getItem.isLoaded = 0 as libc::c_int as u8_0;
    (*this).initDrawFunc =
        Some(DemoEffect_DrawGetItem as
                 unsafe extern "C" fn(_: *mut Actor, _: *mut GlobalContext)
                     -> ());
    (*this).initUpdateFunc =
        Some(DemoEffect_UpdateGetItem as
                 unsafe extern "C" fn(_: *mut DemoEffect,
                                      _: *mut GlobalContext) -> ());
    Actor_SetScale(&mut (*this).actor, 0.25f32);
    (*this).csActionId = 6 as libc::c_int as s16;
}
/* *
 * Main Actor Init function
 */
#[no_mangle]
pub unsafe extern "C" fn DemoEffect_Init(mut thisx: *mut Actor,
                                         mut globalCtx2: *mut GlobalContext) {
    let mut globalCtx: *mut GlobalContext = globalCtx2;
    let mut this: *mut DemoEffect = thisx as *mut DemoEffect;
    let mut effectType: s32 = 0;
    let mut lightEffect: s32 = 0;
    let mut objectIndex: s32 = 0;
    let mut crystalLight: *mut DemoEffect = 0 as *mut DemoEffect;
    let mut lightRing: *mut DemoEffect = 0 as *mut DemoEffect;
    effectType = (*this).actor.params as libc::c_int & 0xff as libc::c_int;
    lightEffect =
        ((*this).actor.params as libc::c_int & 0xf000 as libc::c_int) >>
            12 as libc::c_int;
    osSyncPrintf(b"\x1b[36m no = %d\n\x1b[m\x00" as *const u8 as
                     *const libc::c_char, effectType);
    objectIndex =
        if sEffectTypeObjects[effectType as usize] as libc::c_int ==
               OBJECT_GAMEPLAY_KEEP as libc::c_int {
            0 as libc::c_int
        } else {
            Object_GetIndex(&mut (*globalCtx).objectCtx,
                            sEffectTypeObjects[effectType as usize])
        };
    osSyncPrintf(b"\x1b[36m bank_ID = %d\n\x1b[m\x00" as *const u8 as
                     *const libc::c_char, objectIndex);
    if objectIndex < 0 as libc::c_int {
        if 0 as libc::c_int != 0 {
        } else {
            __assert(b"0\x00" as *const u8 as *const libc::c_char,
                     b"../z_demo_effect.c\x00" as *const u8 as
                         *const libc::c_char, 723 as libc::c_int);
        };
    } else { (*this).initObjectBankIndex = objectIndex as u8_0 }
    (*this).effectFlags = 0 as libc::c_int as s16;
    Actor_SetScale(&mut (*this).actor, 0.2f32);
    let mut current_block_208: u64;
    match effectType {
        0 => {
            (*this).initDrawFunc =
                Some(DemoEffect_DrawCrystalLight as
                         unsafe extern "C" fn(_: *mut Actor,
                                              _: *mut GlobalContext) -> ());
            (*this).initUpdateFunc =
                Some(DemoEffect_UpdateCrystalLight as
                         unsafe extern "C" fn(_: *mut DemoEffect,
                                              _: *mut GlobalContext) -> ());
            current_block_208 = 4491581808830814586;
        }
        1 => {
            (*this).initDrawFunc =
                Some(DemoEffect_DrawFireBall as
                         unsafe extern "C" fn(_: *mut Actor,
                                              _: *mut GlobalContext) -> ());
            (*this).initUpdateFunc =
                Some(DemoEffect_UpdatePositionToParent as
                         unsafe extern "C" fn(_: *mut DemoEffect,
                                              _: *mut GlobalContext) -> ());
            Actor_SetScale(&mut (*this).actor, 0.1f32);
            current_block_208 = 4491581808830814586;
        }
        2 => {
            (*this).initDrawFunc =
                Some(DemoEffect_DrawBlueOrb as
                         unsafe extern "C" fn(_: *mut Actor,
                                              _: *mut GlobalContext) -> ());
            (*this).initUpdateFunc =
                Some(DemoEffect_UpdateBlueOrbGrow as
                         unsafe extern "C" fn(_: *mut DemoEffect,
                                              _: *mut GlobalContext) -> ());
            (*this).c2rust_unnamed.blueOrb.alpha = 255 as libc::c_int as u8_0;
            (*this).c2rust_unnamed.blueOrb.scale = 5 as libc::c_int as u8_0;
            (*this).c2rust_unnamed.blueOrb.rotation = 0 as libc::c_int as s16;
            Actor_SetScale(&mut (*this).actor, 0.05f32);
            (*this).primXluColor[0 as libc::c_int as usize] =
                188 as libc::c_int as u8_0;
            (*this).primXluColor[1 as libc::c_int as usize] =
                255 as libc::c_int as u8_0;
            (*this).primXluColor[2 as libc::c_int as usize] =
                255 as libc::c_int as u8_0;
            (*this).envXluColor[1 as libc::c_int as usize] =
                100 as libc::c_int as u8_0;
            (*this).envXluColor[2 as libc::c_int as usize] =
                255 as libc::c_int as u8_0;
            (*this).envXluColor[0 as libc::c_int as usize] =
                0 as libc::c_int as u8_0;
            current_block_208 = 4491581808830814586;
        }
        18 => {
            (*this).initDrawFunc =
                Some(DemoEffect_DrawLightEffect as
                         unsafe extern "C" fn(_: *mut Actor,
                                              _: *mut GlobalContext) -> ());
            (*this).initUpdateFunc =
                Some(DemoEffect_UpdateLightEffect as
                         unsafe extern "C" fn(_: *mut DemoEffect,
                                              _: *mut GlobalContext) -> ());
            (*this).c2rust_unnamed.light.alpha = 255 as libc::c_int as u8_0;
            (*this).c2rust_unnamed.light.scaleFlag = 0 as libc::c_int as u8_0;
            (*this).c2rust_unnamed.light.flicker = 0 as libc::c_int as u8_0;
            (*this).c2rust_unnamed.light.rotation = 0 as libc::c_int as s16;
            match lightEffect {
                0 => {
                    (*this).primXluColor[0 as libc::c_int as usize] =
                        255 as libc::c_int as u8_0;
                    (*this).primXluColor[1 as libc::c_int as usize] =
                        255 as libc::c_int as u8_0;
                    (*this).primXluColor[2 as libc::c_int as usize] =
                        255 as libc::c_int as u8_0;
                    (*this).envXluColor[1 as libc::c_int as usize] =
                        50 as libc::c_int as u8_0;
                    (*this).envXluColor[0 as libc::c_int as usize] =
                        255 as libc::c_int as u8_0;
                    (*this).envXluColor[2 as libc::c_int as usize] =
                        0 as libc::c_int as u8_0
                }
                1 => {
                    (*this).primXluColor[0 as libc::c_int as usize] =
                        255 as libc::c_int as u8_0;
                    (*this).primXluColor[1 as libc::c_int as usize] =
                        255 as libc::c_int as u8_0;
                    (*this).primXluColor[2 as libc::c_int as usize] =
                        255 as libc::c_int as u8_0;
                    (*this).envXluColor[1 as libc::c_int as usize] =
                        150 as libc::c_int as u8_0;
                    (*this).envXluColor[0 as libc::c_int as usize] =
                        0 as libc::c_int as u8_0;
                    (*this).envXluColor[2 as libc::c_int as usize] =
                        255 as libc::c_int as u8_0
                }
                2 => {
                    (*this).primXluColor[0 as libc::c_int as usize] =
                        255 as libc::c_int as u8_0;
                    (*this).primXluColor[1 as libc::c_int as usize] =
                        255 as libc::c_int as u8_0;
                    (*this).primXluColor[2 as libc::c_int as usize] =
                        255 as libc::c_int as u8_0;
                    (*this).envXluColor[1 as libc::c_int as usize] =
                        200 as libc::c_int as u8_0;
                    (*this).envXluColor[0 as libc::c_int as usize] =
                        0 as libc::c_int as u8_0;
                    (*this).envXluColor[2 as libc::c_int as usize] =
                        0 as libc::c_int as u8_0
                }
                3 => {
                    (*this).primXluColor[0 as libc::c_int as usize] =
                        255 as libc::c_int as u8_0;
                    (*this).primXluColor[1 as libc::c_int as usize] =
                        255 as libc::c_int as u8_0;
                    (*this).primXluColor[2 as libc::c_int as usize] =
                        255 as libc::c_int as u8_0;
                    (*this).envXluColor[1 as libc::c_int as usize] =
                        150 as libc::c_int as u8_0;
                    (*this).envXluColor[0 as libc::c_int as usize] =
                        255 as libc::c_int as u8_0;
                    (*this).envXluColor[2 as libc::c_int as usize] =
                        0 as libc::c_int as u8_0
                }
                4 => {
                    (*this).primXluColor[0 as libc::c_int as usize] =
                        255 as libc::c_int as u8_0;
                    (*this).primXluColor[1 as libc::c_int as usize] =
                        255 as libc::c_int as u8_0;
                    (*this).primXluColor[2 as libc::c_int as usize] =
                        255 as libc::c_int as u8_0;
                    (*this).envXluColor[0 as libc::c_int as usize] =
                        200 as libc::c_int as u8_0;
                    (*this).envXluColor[1 as libc::c_int as usize] =
                        255 as libc::c_int as u8_0;
                    (*this).envXluColor[2 as libc::c_int as usize] =
                        0 as libc::c_int as u8_0
                }
                5 => {
                    (*this).primXluColor[0 as libc::c_int as usize] =
                        255 as libc::c_int as u8_0;
                    (*this).primXluColor[1 as libc::c_int as usize] =
                        255 as libc::c_int as u8_0;
                    (*this).primXluColor[2 as libc::c_int as usize] =
                        255 as libc::c_int as u8_0;
                    // clang-format off
                    (*this).envXluColor[0 as libc::c_int as usize] =
                        200 as libc::c_int as
                            u8_0; // Sameline prevents reordering
                    (*this).envXluColor[1 as libc::c_int as usize] =
                        50 as libc::c_int as u8_0;
                    (*this).envXluColor[2 as libc::c_int as usize] =
                        255 as libc::c_int as u8_0
                }
                6 => {
                    (*this).primXluColor[0 as libc::c_int as usize] =
                        255 as libc::c_int as u8_0;
                    (*this).primXluColor[1 as libc::c_int as usize] =
                        255 as libc::c_int as u8_0;
                    (*this).primXluColor[2 as libc::c_int as usize] =
                        255 as libc::c_int as u8_0;
                    (*this).envXluColor[1 as libc::c_int as usize] =
                        200 as libc::c_int as u8_0;
                    (*this).envXluColor[0 as libc::c_int as usize] =
                        0 as libc::c_int as u8_0;
                    (*this).envXluColor[2 as libc::c_int as usize] =
                        0 as libc::c_int as u8_0
                }
                _ => { }
            }
            (*this).csActionId = 7 as libc::c_int as s16;
            Actor_SetScale(thisx, 0.0f32);
            current_block_208 = 4491581808830814586;
        }
        3 => {
            (*this).c2rust_unnamed.lgtShower.alpha =
                255 as libc::c_int as u8_0;
            (*this).initDrawFunc =
                Some(DemoEffect_DrawLgtShower as
                         unsafe extern "C" fn(_: *mut Actor,
                                              _: *mut GlobalContext) -> ());
            (*this).initUpdateFunc =
                Some(DemoEffect_UpdateLgtShower as
                         unsafe extern "C" fn(_: *mut DemoEffect,
                                              _: *mut GlobalContext) -> ());
            current_block_208 = 4491581808830814586;
        }
        4 => {
            Actor_SetScale(&mut (*this).actor, 0.1f32);
            (*this).initDrawFunc =
                Some(DemoEffect_DrawGodLgt as
                         unsafe extern "C" fn(_: *mut Actor,
                                              _: *mut GlobalContext) -> ());
            (*this).primXluColor[1 as libc::c_int as usize] =
                170 as libc::c_int as u8_0;
            (*this).primXluColor[0 as libc::c_int as usize] =
                255 as libc::c_int as u8_0;
            (*this).primXluColor[2 as libc::c_int as usize] =
                255 as libc::c_int as u8_0;
            (*this).envXluColor[0 as libc::c_int as usize] =
                255 as libc::c_int as u8_0;
            (*this).envXluColor[2 as libc::c_int as usize] =
                255 as libc::c_int as u8_0;
            (*this).envXluColor[1 as libc::c_int as usize] =
                0 as libc::c_int as u8_0;
            (*this).c2rust_unnamed.godLgt.type_0 =
                GOD_LGT_DIN as libc::c_int as u8_0;
            (*this).c2rust_unnamed.godLgt.rotation = 0 as libc::c_int as u8_0;
            (*this).initUpdateFunc =
                Some(DemoEffect_UpdateGodLgtDin as
                         unsafe extern "C" fn(_: *mut DemoEffect,
                                              _: *mut GlobalContext) -> ());
            (*this).csActionId = 0 as libc::c_int as s16;
            current_block_208 = 4491581808830814586;
        }
        5 => {
            if gSaveContext.entranceIndex == 0x13d as libc::c_int {
                Actor_SetScale(&mut (*this).actor, 1.0f32);
            } else { Actor_SetScale(&mut (*this).actor, 0.1f32); }
            (*this).initDrawFunc =
                Some(DemoEffect_DrawGodLgt as
                         unsafe extern "C" fn(_: *mut Actor,
                                              _: *mut GlobalContext) -> ());
            (*this).primXluColor[0 as libc::c_int as usize] =
                170 as libc::c_int as u8_0;
            (*this).primXluColor[1 as libc::c_int as usize] =
                255 as libc::c_int as u8_0;
            (*this).primXluColor[2 as libc::c_int as usize] =
                255 as libc::c_int as u8_0;
            (*this).envXluColor[1 as libc::c_int as usize] =
                40 as libc::c_int as u8_0;
            (*this).envXluColor[2 as libc::c_int as usize] =
                255 as libc::c_int as u8_0;
            (*this).envXluColor[0 as libc::c_int as usize] =
                0 as libc::c_int as u8_0;
            (*this).c2rust_unnamed.godLgt.type_0 =
                GOD_LGT_NAYRU as libc::c_int as u8_0;
            (*this).c2rust_unnamed.godLgt.lightRingSpawnDelay =
                4 as libc::c_int as u8_0;
            (*this).c2rust_unnamed.godLgt.rotation = 0 as libc::c_int as u8_0;
            (*this).c2rust_unnamed.godLgt.lightRingSpawnTimer =
                0 as libc::c_int as s16;
            (*this).initUpdateFunc =
                Some(DemoEffect_UpdateGodLgtNayru as
                         unsafe extern "C" fn(_: *mut DemoEffect,
                                              _: *mut GlobalContext) -> ());
            (*this).csActionId = 1 as libc::c_int as s16;
            current_block_208 = 4491581808830814586;
        }
        6 => {
            if gSaveContext.entranceIndex == 0xee as libc::c_int {
                Actor_SetScale(&mut (*this).actor, 2.4f32);
            } else { Actor_SetScale(&mut (*this).actor, 0.1f32); }
            (*this).initDrawFunc =
                Some(DemoEffect_DrawGodLgt as
                         unsafe extern "C" fn(_: *mut Actor,
                                              _: *mut GlobalContext) -> ());
            (*this).primXluColor[0 as libc::c_int as usize] =
                170 as libc::c_int as u8_0;
            (*this).primXluColor[2 as libc::c_int as usize] =
                170 as libc::c_int as u8_0;
            (*this).primXluColor[1 as libc::c_int as usize] =
                255 as libc::c_int as u8_0;
            (*this).envXluColor[1 as libc::c_int as usize] =
                200 as libc::c_int as u8_0;
            (*this).envXluColor[0 as libc::c_int as usize] =
                0 as libc::c_int as u8_0;
            (*this).envXluColor[2 as libc::c_int as usize] =
                0 as libc::c_int as u8_0;
            (*this).c2rust_unnamed.godLgt.type_0 =
                GOD_LGT_FARORE as libc::c_int as u8_0;
            (*this).c2rust_unnamed.godLgt.rotation = 0 as libc::c_int as u8_0;
            (*this).initUpdateFunc =
                Some(DemoEffect_UpdateGodLgtFarore as
                         unsafe extern "C" fn(_: *mut DemoEffect,
                                              _: *mut GlobalContext) -> ());
            (*this).csActionId = 2 as libc::c_int as s16;
            current_block_208 = 4491581808830814586;
        }
        7 => {
            (*this).initDrawFunc =
                Some(DemoEffect_DrawLightRing as
                         unsafe extern "C" fn(_: *mut Actor,
                                              _: *mut GlobalContext) -> ());
            (*this).initUpdateFunc =
                Some(DemoEffect_UpdateLightRingExpanding as
                         unsafe extern "C" fn(_: *mut DemoEffect,
                                              _: *mut GlobalContext) -> ());
            (*this).c2rust_unnamed.lightRing.timer = 20 as libc::c_int as s16;
            (*this).c2rust_unnamed.lightRing.timerIncrement =
                4 as libc::c_int as u8_0;
            (*this).c2rust_unnamed.lightRing.alpha =
                255 as libc::c_int as u8_0;
            current_block_208 = 4491581808830814586;
        }
        17 => {
            (*this).initDrawFunc =
                Some(DemoEffect_DrawLightRing as
                         unsafe extern "C" fn(_: *mut Actor,
                                              _: *mut GlobalContext) -> ());
            (*this).initUpdateFunc =
                Some(DemoEffect_UpdateLightRingTriforce as
                         unsafe extern "C" fn(_: *mut DemoEffect,
                                              _: *mut GlobalContext) -> ());
            (*this).c2rust_unnamed.lightRing.timer = 20 as libc::c_int as s16;
            (*this).c2rust_unnamed.lightRing.timerIncrement =
                4 as libc::c_int as u8_0;
            (*this).c2rust_unnamed.lightRing.alpha = 0 as libc::c_int as u8_0;
            (*this).csActionId = 4 as libc::c_int as s16;
            current_block_208 = 4491581808830814586;
        }
        16 => {
            (*this).initDrawFunc =
                Some(DemoEffect_DrawLightRing as
                         unsafe extern "C" fn(_: *mut Actor,
                                              _: *mut GlobalContext) -> ());
            (*this).initUpdateFunc =
                Some(DemoEffect_UpdateLightRingShrinking as
                         unsafe extern "C" fn(_: *mut DemoEffect,
                                              _: *mut GlobalContext) -> ());
            (*this).c2rust_unnamed.lightRing.timer =
                351 as libc::c_int as s16;
            (*this).c2rust_unnamed.lightRing.timerIncrement =
                2 as libc::c_int as u8_0;
            (*this).c2rust_unnamed.lightRing.alpha = 0 as libc::c_int as u8_0;
            current_block_208 = 4491581808830814586;
        }
        8 => {
            (*this).initDrawFunc =
                Some(DemoEffect_DrawTriforceSpot as
                         unsafe extern "C" fn(_: *mut Actor,
                                              _: *mut GlobalContext) -> ());
            (*this).initUpdateFunc =
                Some(DemoEffect_UpdateTriforceSpot as
                         unsafe extern "C" fn(_: *mut DemoEffect,
                                              _: *mut GlobalContext) -> ());
            (*this).c2rust_unnamed.triforceSpot.crystalLightOpacity =
                0 as libc::c_int as u8_0;
            (*this).c2rust_unnamed.triforceSpot.lightColumnOpacity =
                0 as libc::c_int as u8_0;
            (*this).c2rust_unnamed.triforceSpot.triforceSpotOpacity =
                0 as libc::c_int as u8_0;
            (*this).c2rust_unnamed.triforceSpot.rotation =
                0 as libc::c_int as s16;
            (*this).primXluColor[0 as libc::c_int as usize] =
                0 as libc::c_int as u8_0;
            (*this).csActionId = 3 as libc::c_int as s16;
            Actor_SetScale(&mut (*this).actor, 0.020f32);
            crystalLight =
                Actor_SpawnAsChild(&mut (*globalCtx).actorCtx,
                                   &mut (*this).actor, globalCtx,
                                   ACTOR_DEMO_EFFECT as libc::c_int as s16,
                                   (*this).actor.world.pos.x,
                                   (*this).actor.world.pos.y,
                                   (*this).actor.world.pos.z,
                                   0 as libc::c_int as s16,
                                   0 as libc::c_int as s16,
                                   0 as libc::c_int as s16,
                                   DEMO_EFFECT_CRYSTAL_LIGHT as libc::c_int as
                                       s16) as *mut DemoEffect;
            if !crystalLight.is_null() {
                Actor_SetScale(&mut (*crystalLight).actor, 0.6f32);
            }
            lightRing =
                Actor_SpawnAsChild(&mut (*globalCtx).actorCtx,
                                   &mut (*crystalLight).actor, globalCtx,
                                   ACTOR_DEMO_EFFECT as libc::c_int as s16,
                                   (*this).actor.world.pos.x,
                                   (*this).actor.world.pos.y,
                                   (*this).actor.world.pos.z,
                                   0 as libc::c_int as s16,
                                   0 as libc::c_int as s16,
                                   0 as libc::c_int as s16,
                                   DEMO_EFFECT_LIGHTRING_TRIFORCE as
                                       libc::c_int as s16) as *mut DemoEffect;
            if !lightRing.is_null() {
                Actor_SetScale(&mut (*lightRing).actor, 0.4f32);
            }
            current_block_208 = 4491581808830814586;
        }
        9 => {
            DemoEffect_InitGetItem(this);
            (*this).c2rust_unnamed.getItem.drawId =
                GID_MEDALLION_FIRE as libc::c_int as u8_0;
            current_block_208 = 4491581808830814586;
        }
        10 => {
            DemoEffect_InitGetItem(this);
            (*this).c2rust_unnamed.getItem.drawId =
                GID_MEDALLION_WATER as libc::c_int as u8_0;
            current_block_208 = 4491581808830814586;
        }
        11 => {
            DemoEffect_InitGetItem(this);
            (*this).c2rust_unnamed.getItem.drawId =
                GID_MEDALLION_FOREST as libc::c_int as u8_0;
            current_block_208 = 4491581808830814586;
        }
        12 => {
            DemoEffect_InitGetItem(this);
            (*this).c2rust_unnamed.getItem.drawId =
                GID_MEDALLION_SPIRIT as libc::c_int as u8_0;
            current_block_208 = 4491581808830814586;
        }
        13 => {
            DemoEffect_InitGetItem(this);
            (*this).c2rust_unnamed.getItem.drawId =
                GID_MEDALLION_SHADOW as libc::c_int as u8_0;
            current_block_208 = 4491581808830814586;
        }
        14 => {
            DemoEffect_InitGetItem(this);
            (*this).c2rust_unnamed.getItem.drawId =
                GID_MEDALLION_LIGHT as libc::c_int as u8_0;
            current_block_208 = 4491581808830814586;
        }
        23 => {
            DemoEffect_InitGetItem(this);
            (*this).c2rust_unnamed.getItem.drawId =
                GID_ARROW_LIGHT as libc::c_int as u8_0;
            current_block_208 = 4491581808830814586;
        }
        24 | 25 => {
            (*this).actor.flags |=
                ((1 as libc::c_int) << 25 as libc::c_int) as libc::c_uint;
            current_block_208 = 16472999995039658896;
        }
        15 => { current_block_208 = 16472999995039658896; }
        19 => {
            (*this).jewelDisplayList = gGiKokiriEmeraldGemDL.as_mut_ptr();
            (*this).jewelHolderDisplayList =
                gGiKokiriEmeraldSettingDL.as_mut_ptr();
            (*this).c2rust_unnamed.jewel.type_0 =
                DEMO_EFFECT_JEWEL_KOKIRI as libc::c_int as u8_0;
            (*this).c2rust_unnamed.jewel.isPositionInit =
                0 as libc::c_int as u8_0;
            DemoEffect_InitJewel(globalCtx, this);
            current_block_208 = 4491581808830814586;
        }
        20 => {
            (*this).jewelDisplayList = gGiGoronRubyGemDL.as_mut_ptr();
            (*this).jewelHolderDisplayList =
                gGiGoronRubySettingDL.as_mut_ptr();
            (*this).c2rust_unnamed.jewel.type_0 =
                DEMO_EFFECT_JEWEL_GORON as libc::c_int as u8_0;
            (*this).c2rust_unnamed.jewel.isPositionInit =
                0 as libc::c_int as u8_0;
            DemoEffect_InitJewel(globalCtx, this);
            current_block_208 = 4491581808830814586;
        }
        21 => {
            (*this).jewelDisplayList = gGiZoraSapphireGemDL.as_mut_ptr();
            (*this).jewelHolderDisplayList =
                gGiZoraSapphireSettingDL.as_mut_ptr();
            (*this).c2rust_unnamed.jewel.type_0 =
                DEMO_EFFECT_JEWEL_ZORA as libc::c_int as u8_0;
            (*this).c2rust_unnamed.jewel.isPositionInit =
                0 as libc::c_int as u8_0;
            DemoEffect_InitJewel(globalCtx, this);
            Actor_ChangeCategory(globalCtx, &mut (*globalCtx).actorCtx,
                                 &mut (*this).actor,
                                 ACTOR_EN_DOOR as libc::c_int as u8_0);
            if (*globalCtx).sceneNum as libc::c_int ==
                   SCENE_BDAN as libc::c_int &&
                   gSaveContext.infTable[20 as libc::c_int as usize] as
                       libc::c_int & 0x20 as libc::c_int != 0 {
                Actor_Kill(&mut (*this).actor);
                return
            }
            current_block_208 = 4491581808830814586;
        }
        22 => {
            (*this).initDrawFunc = None;
            (*this).initUpdateFunc =
                Some(DemoEffect_UpdateDust as
                         unsafe extern "C" fn(_: *mut DemoEffect,
                                              _: *mut GlobalContext) -> ());
            (*this).c2rust_unnamed.dust.timer = 0 as libc::c_int as u8_0;
            (*this).csActionId = 2 as libc::c_int as s16;
            current_block_208 = 4491581808830814586;
        }
        _ => {
            if 0 as libc::c_int != 0 {
            } else {
                __assert(b"0\x00" as *const u8 as *const libc::c_char,
                         b"../z_demo_effect.c\x00" as *const u8 as
                             *const libc::c_char, 1062 as libc::c_int);
            };
            current_block_208 = 4491581808830814586;
        }
    }
    match current_block_208 {
        16472999995039658896 => {
            (*this).initDrawFunc =
                Some(DemoEffect_DrawTimeWarp as
                         unsafe extern "C" fn(_: *mut Actor,
                                              _: *mut GlobalContext) -> ());
            (*this).initUpdateFunc =
                Some(DemoEffect_InitTimeWarp as
                         unsafe extern "C" fn(_: *mut DemoEffect,
                                              _: *mut GlobalContext) -> ());
            (*this).envXluColor[0 as libc::c_int as usize] =
                0 as libc::c_int as u8_0;
            (*this).envXluColor[1 as libc::c_int as usize] =
                100 as libc::c_int as u8_0;
            (*this).envXluColor[2 as libc::c_int as usize] =
                255 as libc::c_int as u8_0;
            SkelCurve_Clear(&mut (*this).skelCurve);
            (*this).c2rust_unnamed.timeWarp.shrinkTimer =
                0 as libc::c_int as s16
        }
        _ => { }
    }
    ActorShape_Init(&mut (*thisx).shape, 0.0f32, None, 0.0f32);
    DemoEffect_SetupUpdate(this,
                           Some(DemoEffect_Wait as
                                    unsafe extern "C" fn(_: *mut DemoEffect,
                                                         _:
                                                             *mut GlobalContext)
                                        -> ()));
}
/* *
 * Main Actor Destroy function
 */
#[no_mangle]
pub unsafe extern "C" fn DemoEffect_Destroy(mut thisx: *mut Actor,
                                            mut globalCtx:
                                                *mut GlobalContext) {
    let mut this: *mut DemoEffect = thisx as *mut DemoEffect;
    let mut effectType: s32 =
        (*this).actor.params as libc::c_int & 0xff as libc::c_int;
    if effectType == DEMO_EFFECT_TIMEWARP_MASTERSWORD as libc::c_int ||
           effectType == DEMO_EFFECT_TIMEWARP_TIMEBLOCK_LARGE as libc::c_int
           ||
           effectType == DEMO_EFFECT_TIMEWARP_TIMEBLOCK_SMALL as libc::c_int {
        SkelCurve_Destroy(globalCtx, &mut (*this).skelCurve);
    };
}
/* *
 * This update function waits until the associate object is loaded.
 * Once the object is loaded, it will copy over the initUpdateFunc/initDrawFunc funcs to be active.
 * They are copied to actor.draw and updateFunc.
 * initUpdateFunc/initDrawFunc are set during initialization and are NOT executed.
 */
#[no_mangle]
pub unsafe extern "C" fn DemoEffect_Wait(mut this: *mut DemoEffect,
                                         mut globalCtx: *mut GlobalContext) {
    if Object_IsLoaded(&mut (*globalCtx).objectCtx,
                       (*this).initObjectBankIndex as s32) != 0 {
        (*this).actor.objBankIndex = (*this).initObjectBankIndex as s8;
        (*this).actor.draw = (*this).initDrawFunc;
        (*this).updateFunc = (*this).initUpdateFunc;
        osSyncPrintf(b"\x1b[36m \xe8\xbb\xa2\xe9\x80\x81\xe7\xb5\x82\xe4\xba\x86 move_wait \x1b[m\x00"
                         as *const u8 as *const libc::c_char);
    };
}
/* *
 * Copies the current Actor's position to the parent Actor's position.
 */
#[no_mangle]
pub unsafe extern "C" fn DemoEffect_UpdatePositionToParent(mut this:
                                                               *mut DemoEffect,
                                                           mut globalCtx:
                                                               *mut GlobalContext) {
    if !(*this).actor.parent.is_null() {
        // Struct copy affects regalloc
        (*this).actor.world.pos.x = (*(*this).actor.parent).world.pos.x;
        (*this).actor.world.pos.y = (*(*this).actor.parent).world.pos.y;
        (*this).actor.world.pos.z = (*(*this).actor.parent).world.pos.z
    };
}
/* *
 * Update function for the Crystal Light actor.
 * The Crystal Light actor is the three beams of light under the Triforce that converge on it.
 * The Crystal Light's position is set to the parent Actor (Triforce) each frame.
 * If the Crystal Light has no parent Actor, then it will raise into the sky.
 */
#[no_mangle]
pub unsafe extern "C" fn DemoEffect_UpdateCrystalLight(mut this:
                                                           *mut DemoEffect,
                                                       mut globalCtx:
                                                           *mut GlobalContext) {
    DemoEffect_UpdatePositionToParent(this, globalCtx);
    (*this).actor.world.pos.y += 14.0f32;
}
/* *
 * Spawns sparkle effects for Medals
 */
#[no_mangle]
pub unsafe extern "C" fn DemoEffect_MedalSparkle(mut this: *mut DemoEffect,
                                                 mut globalCtx:
                                                     *mut GlobalContext,
                                                 mut isSmallSpawner: s32) {
    let mut velocity: Vec3f = Vec3f{x: 0., y: 0., z: 0.,};
    let mut accel: Vec3f = Vec3f{x: 0., y: 0., z: 0.,};
    let mut pos: Vec3f = Vec3f{x: 0., y: 0., z: 0.,};
    let mut primColor: Color_RGBA8 = Color_RGBA8{r: 0, g: 0, b: 0, a: 0,};
    let mut envColor: Color_RGBA8 = Color_RGBA8{r: 0, g: 0, b: 0, a: 0,};
    if isSmallSpawner != 1 as libc::c_int ||
           (*globalCtx).gameplayFrames & 1 as libc::c_int as libc::c_uint ==
               0 as libc::c_int as libc::c_uint {
        primColor.r = 255 as libc::c_int as u8_0;
        primColor.g = 255 as libc::c_int as u8_0;
        primColor.b = 255 as libc::c_int as u8_0;
        envColor.r = 255 as libc::c_int as u8_0;
        envColor.g = 255 as libc::c_int as u8_0;
        envColor.b = 100 as libc::c_int as u8_0;
        primColor.a = 0 as libc::c_int as u8_0;
        velocity.y = 0.0f32;
        accel.x = 0.0f32;
        accel.y = -0.1f32;
        accel.z = 0.0f32;
        if isSmallSpawner != 0 {
            velocity.x = Rand_ZeroOne() - 0.5f32;
            velocity.z = Rand_ZeroOne() - 0.5f32
        } else {
            velocity.x = (Rand_ZeroOne() - 0.5f32) * 2.0f32;
            velocity.z = (Rand_ZeroOne() - 0.5f32) * 2.0f32
        }
        pos.x = Rand_CenteredFloat(10.0f32) + (*this).actor.world.pos.x;
        pos.y = Rand_CenteredFloat(10.0f32) + (*this).actor.world.pos.y;
        pos.z = Rand_CenteredFloat(10.0f32) + (*this).actor.world.pos.z;
        EffectSsKiraKira_SpawnDispersed(globalCtx, &mut pos, &mut velocity,
                                        &mut accel, &mut primColor,
                                        &mut envColor,
                                        1000 as libc::c_int as s16,
                                        16 as libc::c_int);
    };
}
/* *
 * Update function for the GetItem Actors.
 * Medals and Light Arrows.
 * It spawns Medal Sparkle Effects  and scales/moves the Actor based on the current Cutscene Action
 */
#[no_mangle]
pub unsafe extern "C" fn DemoEffect_UpdateGetItem(mut this: *mut DemoEffect,
                                                  mut globalCtx:
                                                      *mut GlobalContext) {
    let mut thisx: *mut Actor = &mut (*this).actor;
    if (*globalCtx).csCtx.state as libc::c_int != CS_STATE_IDLE as libc::c_int
           &&
           !(*globalCtx).csCtx.npcActions[(*this).csActionId as
                                              usize].is_null() {
        if (*this).c2rust_unnamed.getItem.isPositionInit != 0 {
            DemoEffect_MoveGetItem(this, globalCtx, (*this).csActionId as s32,
                                   0.1f32);
        } else {
            DemoEffect_InitPositionFromCsAction(this, globalCtx,
                                                (*this).csActionId as s32);
            (*this).c2rust_unnamed.getItem.isPositionInit =
                1 as libc::c_int as u8_0
        }
        if (*this).c2rust_unnamed.getItem.drawId as libc::c_int !=
               GID_ARROW_LIGHT as libc::c_int {
            (*this).actor.shape.rot.x = 0xe0c0 as libc::c_int as s16
        } else {
            (*this).actor.shape.rot.y =
                ((*this).actor.shape.rot.y as libc::c_int +
                     0x400 as libc::c_int) as s16
        }
        Actor_SetScale(thisx, 0.20f32);
        if gSaveContext.entranceIndex == 0x53 as libc::c_int {
            match (*(*globalCtx).csCtx.npcActions[(*this).csActionId as
                                                      usize]).action as
                      libc::c_int {
                2 => {
                    DemoEffect_MedalSparkle(this, globalCtx,
                                            0 as libc::c_int);
                }
                3 => {
                    DemoEffect_MedalSparkle(this, globalCtx,
                                            1 as libc::c_int);
                }
                _ => { }
            }
        }
        match (*(*globalCtx).csCtx.npcActions[(*this).csActionId as
                                                  usize]).action as
                  libc::c_int {
            2 => {
                if gSaveContext.entranceIndex == 0x53 as libc::c_int {
                    Audio_PlayActorSound2(thisx,
                                          (0x2897 as libc::c_int -
                                               0x800 as libc::c_int) as
                                              u16_0);
                } else {
                    func_800788CC((0x2895 as libc::c_int -
                                       0x800 as libc::c_int) as u16_0);
                }
                if (*this).c2rust_unnamed.getItem.drawId as libc::c_int !=
                       GID_ARROW_LIGHT as libc::c_int {
                    (*this).actor.shape.rot.y =
                        ((*this).actor.shape.rot.y as libc::c_int +
                             0x3e80 as libc::c_int) as s16
                }
                (*this).c2rust_unnamed.getItem.rotation =
                    0x3e80 as libc::c_int as s16
            }
            3 => {
                (*this).c2rust_unnamed.getItem.rotation =
                    ((*this).c2rust_unnamed.getItem.rotation as libc::c_int -
                         (((*this).c2rust_unnamed.getItem.rotation as
                               libc::c_int - 0x3e8 as libc::c_int) as
                              libc::c_float * 0.10f32) as s16 as libc::c_int)
                        as s16;
                if (*this).c2rust_unnamed.getItem.drawId as libc::c_int !=
                       GID_ARROW_LIGHT as libc::c_int {
                    (*this).actor.shape.rot.y =
                        ((*this).actor.shape.rot.y as libc::c_int +
                             (*this).c2rust_unnamed.getItem.rotation as
                                 libc::c_int) as s16
                }
                if gSaveContext.entranceIndex == 0x53 as libc::c_int {
                    Audio_PlayActorSound2(thisx,
                                          (0x2897 as libc::c_int -
                                               0x800 as libc::c_int) as
                                              u16_0);
                } else {
                    func_800788CC((0x2895 as libc::c_int -
                                       0x800 as libc::c_int) as u16_0);
                }
            }
            4 => {
                Audio_PlayActorSound2(thisx,
                                      (0x2895 as libc::c_int -
                                           0x800 as libc::c_int) as u16_0);
            }
            _ => { }
        }
    };
}
/* *
 * Initializes Timewarp Actors.
 * This is an Update Function that is only ran for one frame.
 * Timewarp actors are spawned when Link...
 * 1) Pulls the Master Sword
 * 2) Returns from the Chamber of Sages for the first time
 * 3) Timeblock is cleared with the Song of Time (Large and Small have different versions of Timewarp)
 */
#[no_mangle]
pub unsafe extern "C" fn DemoEffect_InitTimeWarp(mut this: *mut DemoEffect,
                                                 mut globalCtx:
                                                     *mut GlobalContext) {
    let mut effectType: s32 =
        (*this).actor.params as libc::c_int & 0xff as libc::c_int;
    if SkelCurve_Init(globalCtx, &mut (*this).skelCurve, &mut gTimeWarpSkel,
                      &mut gTimeWarpAnim) == 0 {
        if 0 as libc::c_int != 0 {
        } else {
            __assert(b"0\x00" as *const u8 as *const libc::c_char,
                     b"../z_demo_effect.c\x00" as *const u8 as
                         *const libc::c_char, 1283 as libc::c_int);
        };
    }
    if effectType == DEMO_EFFECT_TIMEWARP_TIMEBLOCK_LARGE as libc::c_int ||
           effectType == DEMO_EFFECT_TIMEWARP_TIMEBLOCK_SMALL as libc::c_int {
        SkelCurve_SetAnim(&mut (*this).skelCurve, &mut gTimeWarpAnim, 1.0f32,
                          59.0f32, 1.0f32, 1.7f32);
        SkelCurve_Update(globalCtx, &mut (*this).skelCurve);
        (*this).updateFunc =
            Some(DemoEffect_InitTimeWarpTimeblock as
                     unsafe extern "C" fn(_: *mut DemoEffect,
                                          _: *mut GlobalContext) -> ());
        if effectType == DEMO_EFFECT_TIMEWARP_TIMEBLOCK_LARGE as libc::c_int {
            Actor_SetScale(&mut (*this).actor, 0.14f32);
        } else {
            Actor_SetScale(&mut (*this).actor,
                           84 as libc::c_int as libc::c_float * 0.001f32);
        }
    } else if gSaveContext.sceneSetupIndex == 5 as libc::c_int ||
                  gSaveContext.sceneSetupIndex == 4 as libc::c_int ||
                  gSaveContext.entranceIndex == 0x324 as libc::c_int &&
                      gSaveContext.eventChkInf[12 as libc::c_int as usize] as
                          libc::c_int & 0x200 as libc::c_int == 0 {
        SkelCurve_SetAnim(&mut (*this).skelCurve, &mut gTimeWarpAnim, 1.0f32,
                          59.0f32, 59.0f32, 0.0f32);
        SkelCurve_Update(globalCtx, &mut (*this).skelCurve);
        (*this).updateFunc =
            Some(DemoEffect_UpdateTimeWarpReturnFromChamberOfSages as
                     unsafe extern "C" fn(_: *mut DemoEffect,
                                          _: *mut GlobalContext) -> ());
        osSyncPrintf(b"\x1b[36m \xe7\xb8\xae\xe3\x82\x80\xe3\x83\x90\xe3\x83\xbc\xe3\x82\xb8\xe3\x83\xa7\xe3\x83\xb3 \n\x1b[m\x00"
                         as *const u8 as *const libc::c_char);
    } else {
        SkelCurve_SetAnim(&mut (*this).skelCurve, &mut gTimeWarpAnim, 1.0f32,
                          59.0f32, 1.0f32, 1.0f32);
        SkelCurve_Update(globalCtx, &mut (*this).skelCurve);
        (*this).updateFunc =
            Some(DemoEffect_UpdateTimeWarpPullMasterSword as
                     unsafe extern "C" fn(_: *mut DemoEffect,
                                          _: *mut GlobalContext) -> ());
        osSyncPrintf(b"\x1b[36m \xe9\x80\x9a\xe5\xb8\xb8 \xe3\x83\x90\xe3\x83\xbc\xe3\x82\xb8\xe3\x83\xa7\xe3\x83\xb3 \n\x1b[m\x00"
                         as *const u8 as *const libc::c_char);
    };
}
/* *
 * Update function for the Timewarp Actor that is used when Link pulls the Mastersword
 * It changes the Background Music and updates its SkelCurve animation.
 */
#[no_mangle]
pub unsafe extern "C" fn DemoEffect_UpdateTimeWarpPullMasterSword(mut this:
                                                                      *mut DemoEffect,
                                                                  mut globalCtx:
                                                                      *mut GlobalContext) {
    if Flags_GetEnv(globalCtx, 1 as libc::c_int as s16) != 0 {
        if (*this).effectFlags as libc::c_int & 0x2 as libc::c_int == 0 {
            func_800F3F3C(0 as libc::c_int as u8_0);
            (*this).effectFlags =
                ((*this).effectFlags as libc::c_int | 0x2 as libc::c_int) as
                    s16
        }
        if SkelCurve_Update(globalCtx, &mut (*this).skelCurve) != 0 {
            SkelCurve_SetAnim(&mut (*this).skelCurve, &mut gTimeWarpAnim,
                              1.0f32, 60.0f32, 59.0f32, 0.0f32);
        }
    };
}
/* *
 * Shrinks the Timewarp object vertices.
 * Used by the Chamber of Sages return timewarp and Timeblock clear timewarp.
 */
#[no_mangle]
pub unsafe extern "C" fn DemoEffect_TimewarpShrink(mut size: f32_0) {
    let mut vertices: *mut Vtx = 0 as *mut Vtx;
    let mut i: s32 = 0;
    let mut sizes: [u8_0; 3] = [0; 3];
    // This function uses the data in obj_efc_tw offset 0x0060 to 0x01B0
    vertices =
        gSegments[((gTimeWarpVtx.as_mut_ptr() as u32_0) << 4 as libc::c_int >>
                       28 as libc::c_int) as
                      usize].wrapping_add(gTimeWarpVtx.as_mut_ptr() as u32_0 &
                                              0xffffff as libc::c_int as
                                                  libc::c_uint).wrapping_add(0x80000000
                                                                                 as
                                                                                 libc::c_uint)
            as *mut libc::c_void as *mut Vtx;
    sizes[0 as libc::c_int as usize] = 0 as libc::c_int as u8_0;
    sizes[1 as libc::c_int as usize] = (202.0f32 * size) as s32 as u8_0;
    sizes[2 as libc::c_int as usize] = (255.0f32 * size) as s32 as u8_0;
    i = 0 as libc::c_int;
    while i < 21 as libc::c_int {
        if sTimewarpVertexSizeIndices[i as usize] as libc::c_int !=
               0 as libc::c_int {
            (*vertices.offset(i as isize)).v.cn[3 as libc::c_int as usize] =
                sizes[sTimewarpVertexSizeIndices[i as usize] as usize]
        }
        i += 1
    };
}
/* *
 * Update function for the Timewarp Actor that is used when Link returns from the Chamber of Sages for the first time.
 * It shrinks the timewarp vertices and scales the Actor.
 */
#[no_mangle]
pub unsafe extern "C" fn DemoEffect_UpdateTimeWarpReturnFromChamberOfSages(mut this:
                                                                               *mut DemoEffect,
                                                                           mut globalCtx:
                                                                               *mut GlobalContext) {
    let mut shrinkProgress: f32_0 = 0.;
    (*this).c2rust_unnamed.timeWarp.shrinkTimer += 1;
    if (*this).c2rust_unnamed.timeWarp.shrinkTimer as libc::c_int >
           250 as libc::c_int {
        if gSaveContext.entranceIndex == 0x324 as libc::c_int {
            gSaveContext.eventChkInf[12 as libc::c_int as usize] =
                (gSaveContext.eventChkInf[12 as libc::c_int as usize] as
                     libc::c_int | 0x200 as libc::c_int) as u16_0
        }
        Actor_Kill(&mut (*this).actor);
        return
    }
    if (*this).c2rust_unnamed.timeWarp.shrinkTimer as libc::c_int >
           100 as libc::c_int {
        shrinkProgress =
            (250 as libc::c_int -
                 (*this).c2rust_unnamed.timeWarp.shrinkTimer as libc::c_int)
                as libc::c_float * (1.0f32 / 750.0f32);
        (*this).actor.scale.x = shrinkProgress;
        (*this).actor.scale.z = shrinkProgress;
        DemoEffect_TimewarpShrink(shrinkProgress * 5.0f32);
    }
    func_8002F948(&mut (*this).actor,
                  (0x289d as libc::c_int - 0x800 as libc::c_int) as u16_0);
}
/* *
 * Update function for the Timewarp Actor that is used when a Timeblock is cleared.
 * It shrinks the timewarp vertices and scales the Actor.
 */
#[no_mangle]
pub unsafe extern "C" fn DemoEffect_UpdateTimeWarpTimeblock(mut this:
                                                                *mut DemoEffect,
                                                            mut globalCtx:
                                                                *mut GlobalContext) {
    let mut shrinkProgress: f32_0 = 0.;
    let mut scale: f32_0 = 0.;
    (*this).c2rust_unnamed.timeWarp.shrinkTimer += 1;
    if (*this).c2rust_unnamed.timeWarp.shrinkTimer as libc::c_int <=
           100 as libc::c_int {
        shrinkProgress =
            (100 as libc::c_int -
                 (*this).c2rust_unnamed.timeWarp.shrinkTimer as libc::c_int)
                as libc::c_float * 0.010f32;
        scale = shrinkProgress * 0.14f32;
        if (*this).actor.params as libc::c_int & 0xff as libc::c_int ==
               DEMO_EFFECT_TIMEWARP_TIMEBLOCK_SMALL as libc::c_int {
            scale *= 0.6f32
        }
        (*this).actor.scale.x = scale;
        (*this).actor.scale.z = scale;
        DemoEffect_TimewarpShrink(shrinkProgress);
        func_8002F948(&mut (*this).actor,
                      (0x289d as libc::c_int - 0x800 as libc::c_int) as
                          u16_0);
        return
    }
    DemoEffect_TimewarpShrink(1.0f32);
    Actor_Kill(&mut (*this).actor);
}
/* *
 * Initializes information for the Timewarp Actor used for the Timeblock clear effect.
 * This is an Update Func that is only ran for one frame.
 */
#[no_mangle]
pub unsafe extern "C" fn DemoEffect_InitTimeWarpTimeblock(mut this:
                                                              *mut DemoEffect,
                                                          mut globalCtx:
                                                              *mut GlobalContext) {
    func_8002F948(&mut (*this).actor,
                  (0x289d as libc::c_int - 0x800 as libc::c_int) as u16_0);
    if SkelCurve_Update(globalCtx, &mut (*this).skelCurve) != 0 {
        SkelCurve_SetAnim(&mut (*this).skelCurve, &mut gTimeWarpAnim, 1.0f32,
                          60.0f32, 59.0f32, 0.0f32);
        (*this).updateFunc =
            Some(DemoEffect_UpdateTimeWarpTimeblock as
                     unsafe extern "C" fn(_: *mut DemoEffect,
                                          _: *mut GlobalContext) -> ());
        (*this).c2rust_unnamed.timeWarp.shrinkTimer = 0 as libc::c_int as s16
    };
}
/* *
 * Update function for the Triforce Actor.
 * It rotates and updates the alpha of the Triforce and child actors.
 */
#[no_mangle]
pub unsafe extern "C" fn DemoEffect_UpdateTriforceSpot(mut this:
                                                           *mut DemoEffect,
                                                       mut globalCtx:
                                                           *mut GlobalContext) {
    (*this).c2rust_unnamed.triforceSpot.rotation =
        ((*this).c2rust_unnamed.triforceSpot.rotation as libc::c_int +
             0x3e8 as libc::c_int) as s16;
    if (*globalCtx).csCtx.state as libc::c_int != CS_STATE_IDLE as libc::c_int
           &&
           !(*globalCtx).csCtx.npcActions[(*this).csActionId as
                                              usize].is_null() {
        DemoEffect_MoveToCsEndpoint(this, globalCtx,
                                    (*this).csActionId as s32,
                                    0 as libc::c_int);
        if (*(*globalCtx).csCtx.npcActions[(*this).csActionId as
                                               usize]).action as libc::c_int
               == 2 as libc::c_int {
            if ((*this).primXluColor[0 as libc::c_int as usize] as
                    libc::c_int) < 140 as libc::c_int {
                (*this).primXluColor[0 as libc::c_int as usize] =
                    (*this).primXluColor[0 as libc::c_int as
                                             usize].wrapping_add(1)
            }
            if ((*this).primXluColor[0 as libc::c_int as usize] as
                    libc::c_int) < 30 as libc::c_int {
                (*this).c2rust_unnamed.triforceSpot.triforceSpotOpacity =
                    ((*this).primXluColor[0 as libc::c_int as usize] as s32 as
                         libc::c_float * 8.5f32) as u8_0
            } else {
                (*this).c2rust_unnamed.triforceSpot.triforceSpotOpacity =
                    255 as libc::c_int as u8_0;
                if ((*this).primXluColor[0 as libc::c_int as usize] as
                        libc::c_int) < 60 as libc::c_int {
                    (*this).c2rust_unnamed.triforceSpot.lightColumnOpacity =
                        (((*this).primXluColor[0 as libc::c_int as usize] as
                              s32 - 30 as libc::c_int) as libc::c_float *
                             8.5f32) as u8_0
                } else if (*this).primXluColor[0 as libc::c_int as usize] as
                              libc::c_int <= 140 as libc::c_int {
                    (*this).c2rust_unnamed.triforceSpot.lightColumnOpacity =
                        255 as libc::c_int as u8_0;
                    (*this).c2rust_unnamed.triforceSpot.crystalLightOpacity =
                        (((*this).primXluColor[0 as libc::c_int as usize] as
                              s32 - 60 as libc::c_int) as libc::c_float *
                             3.1875f32) as u8_0
                }
            }
        }
        if gSaveContext.entranceIndex == 0xa0 as libc::c_int &&
               gSaveContext.sceneSetupIndex == 6 as libc::c_int &&
               (*globalCtx).csCtx.frames as libc::c_int == 143 as libc::c_int
           {
            Audio_PlayActorSound2(&mut (*this).actor,
                                  0x1842 as libc::c_int as u16_0);
        }
    };
}
/* *
 * Update function for the LightRing actor that shrinks.
 * This is used in the creation cutscene when Din leaves a fireball that explodes into Death Mountain.
 */
#[no_mangle]
pub unsafe extern "C" fn DemoEffect_UpdateLightRingShrinking(mut this:
                                                                 *mut DemoEffect,
                                                             mut globalCtx:
                                                                 *mut GlobalContext) {
    if ((*this).c2rust_unnamed.lightRing.timer as libc::c_int) <
           (*this).c2rust_unnamed.lightRing.timerIncrement as libc::c_int {
        Actor_Kill(&mut (*this).actor);
        (*this).c2rust_unnamed.lightRing.timer = 0 as libc::c_int as s16
    } else {
        (*this).c2rust_unnamed.lightRing.timer =
            ((*this).c2rust_unnamed.lightRing.timer as libc::c_int -
                 (*this).c2rust_unnamed.lightRing.timerIncrement as
                     libc::c_int) as s16
    }
    if (*this).c2rust_unnamed.lightRing.timer as libc::c_int <=
           255 as libc::c_int {
        if (*this).c2rust_unnamed.lightRing.timer as libc::c_int >=
               225 as libc::c_int {
            (*this).c2rust_unnamed.lightRing.alpha =
                (-((*this).c2rust_unnamed.lightRing.timer as libc::c_int) *
                     8 as libc::c_int + 2048 as libc::c_int) as u8_0
        } else {
            (*this).c2rust_unnamed.lightRing.alpha =
                255 as libc::c_int as u8_0
        }
    }
    if (*this).c2rust_unnamed.lightRing.timer as libc::c_int ==
           255 as libc::c_int {
        func_800F3F3C(5 as libc::c_int as u8_0);
    };
}
/* *
 * Update function for the Lightring Actor that expands.
 * These are spawned by Nayru.
 * These are also used by Din in the creation cutscene when she leaves a fireball that explodes into Death Mountain.
 */
#[no_mangle]
pub unsafe extern "C" fn DemoEffect_UpdateLightRingExpanding(mut this:
                                                                 *mut DemoEffect,
                                                             mut globalCtx:
                                                                 *mut GlobalContext) {
    DemoEffect_UpdatePositionToParent(this, globalCtx);
    (*this).c2rust_unnamed.lightRing.timer =
        ((*this).c2rust_unnamed.lightRing.timer as libc::c_int +
             (*this).c2rust_unnamed.lightRing.timerIncrement as libc::c_int)
            as s16;
    if (*this).c2rust_unnamed.lightRing.timer as libc::c_int >=
           225 as libc::c_int {
        (*this).c2rust_unnamed.lightRing.alpha =
            (-((*this).c2rust_unnamed.lightRing.timer as libc::c_int) *
                 8 as libc::c_int + 2048 as libc::c_int) as u8_0
    }
    if (*this).c2rust_unnamed.lightRing.timer as libc::c_int >
           255 as libc::c_int {
        (*this).c2rust_unnamed.lightRing.timer = 255 as libc::c_int as s16;
        Actor_Kill(&mut (*this).actor);
        (*this).c2rust_unnamed.lightRing.timer = 0 as libc::c_int as s16
    };
}
/* *
 * Update function for the Lightring Actor that expands. This is a special version for the Triforce Actor.
 * This version spawns a blue orb when the cutscene action state is set to 2.
 * Once the Blue Orb Actor is spawned the Update Function is changed to the regular Light Ring Expanding Update Func.
 */
#[no_mangle]
pub unsafe extern "C" fn DemoEffect_UpdateLightRingTriforce(mut this:
                                                                *mut DemoEffect,
                                                            mut globalCtx:
                                                                *mut GlobalContext) {
    let mut blueOrb: *mut DemoEffect = 0 as *mut DemoEffect;
    DemoEffect_UpdatePositionToParent(this, globalCtx);
    if (*globalCtx).csCtx.state as libc::c_int != CS_STATE_IDLE as libc::c_int
       {
        if !(*globalCtx).csCtx.npcActions[(*this).csActionId as
                                              usize].is_null() &&
               (*(*globalCtx).csCtx.npcActions[(*this).csActionId as
                                                   usize]).action as
                   libc::c_int == 2 as libc::c_int {
            blueOrb =
                Actor_Spawn(&mut (*globalCtx).actorCtx, globalCtx,
                            ACTOR_DEMO_EFFECT as libc::c_int as s16,
                            (*this).actor.world.pos.x,
                            (*this).actor.world.pos.y,
                            (*this).actor.world.pos.z,
                            0 as libc::c_int as s16, 0 as libc::c_int as s16,
                            0 as libc::c_int as s16,
                            DEMO_EFFECT_BLUE_ORB as libc::c_int as s16) as
                    *mut DemoEffect;
            if !blueOrb.is_null() {
                Actor_SetScale(&mut (*blueOrb).actor, 0.0f32);
            }
            (*this).updateFunc =
                Some(DemoEffect_UpdateLightRingExpanding as
                         unsafe extern "C" fn(_: *mut DemoEffect,
                                              _: *mut GlobalContext) -> ());
            (*this).c2rust_unnamed.lightRing.alpha =
                255 as libc::c_int as u8_0
        }
    };
}
/* *
 * Update function for the FireBall Actor.
 * This is a special version that is used in the creation cutscene.
 * It moves based on gravity and decrements a timer until zero. Once the timer is zero it will spawn other Actors:
 * A Blue Orb Actor, and a Light Ring Expanding Actor, and a Light Ring Shrinking Actor.
 */
#[no_mangle]
pub unsafe extern "C" fn DemoEffect_UpdateCreationFireball(mut this:
                                                               *mut DemoEffect,
                                                           mut globalCtx:
                                                               *mut GlobalContext) {
    let mut effect: *mut DemoEffect = 0 as *mut DemoEffect;
    Actor_MoveForward(&mut (*this).actor);
    (*this).actor.speedXZ =
        (*this).actor.speedXZ + (*this).actor.gravity * 0.5f32;
    if (*this).c2rust_unnamed.fireBall.timer as libc::c_int !=
           0 as libc::c_int {
        (*this).c2rust_unnamed.fireBall.timer =
            (*this).c2rust_unnamed.fireBall.timer.wrapping_sub(1);
        return
    }
    effect =
        Actor_Spawn(&mut (*globalCtx).actorCtx, globalCtx,
                    ACTOR_DEMO_EFFECT as libc::c_int as s16,
                    (*this).actor.world.pos.x, (*this).actor.world.pos.y,
                    (*this).actor.world.pos.z, 0 as libc::c_int as s16,
                    0 as libc::c_int as s16, 0 as libc::c_int as s16,
                    DEMO_EFFECT_BLUE_ORB as libc::c_int as s16) as
            *mut DemoEffect;
    if !effect.is_null() { Actor_SetScale(&mut (*effect).actor, 0.0f32); }
    effect =
        Actor_Spawn(&mut (*globalCtx).actorCtx, globalCtx,
                    ACTOR_DEMO_EFFECT as libc::c_int as s16,
                    (*this).actor.world.pos.x, (*this).actor.world.pos.y,
                    (*this).actor.world.pos.z, 0 as libc::c_int as s16,
                    0 as libc::c_int as s16, 0 as libc::c_int as s16,
                    DEMO_EFFECT_LIGHTRING_EXPANDING as libc::c_int as s16) as
            *mut DemoEffect;
    if !effect.is_null() { Actor_SetScale(&mut (*effect).actor, 0.1f32); }
    effect =
        Actor_Spawn(&mut (*globalCtx).actorCtx, globalCtx,
                    ACTOR_DEMO_EFFECT as libc::c_int as s16,
                    (*this).actor.world.pos.x, (*this).actor.world.pos.y,
                    (*this).actor.world.pos.z, 0 as libc::c_int as s16,
                    0 as libc::c_int as s16, 0 as libc::c_int as s16,
                    DEMO_EFFECT_LIGHTRING_SHRINKING as libc::c_int as s16) as
            *mut DemoEffect;
    if !effect.is_null() { Actor_SetScale(&mut (*effect).actor, 0.2f32); }
    func_800788CC(0x1842 as libc::c_int as u16_0);
    Actor_Kill(&mut (*this).actor);
}
/* *
 * Initialization function for the FireBall Actor.
 * This is a special version that is used in the creation cutscene.
 * It is an Update Function only executed for one frame. The Update Function is then changed to UpdateCreationFireball.
 */
#[no_mangle]
pub unsafe extern "C" fn DemoEffect_InitCreationFireball(mut this:
                                                             *mut DemoEffect,
                                                         mut globalCtx:
                                                             *mut GlobalContext) {
    let mut parent: *mut Actor = (*this).actor.parent;
    (*this).actor.world.rot.y = (*parent).shape.rot.y;
    (*this).c2rust_unnamed.fireBall.timer = 50 as libc::c_int as u8_0;
    (*this).actor.speedXZ = 1.5f32;
    (*this).actor.minVelocityY = -1.5f32;
    (*this).actor.gravity = -0.03f32;
    (*this).updateFunc =
        Some(DemoEffect_UpdateCreationFireball as
                 unsafe extern "C" fn(_: *mut DemoEffect,
                                      _: *mut GlobalContext) -> ());
}
/* *
 * Update action for the Blue Orb Actor.
 * This Update Function is run while the Blue Orb is Shrinking.
 * The Blue Orb Actor is the blue light sparkle that is in Din's creation cutscene.
 * It's spawned in the middle of the expanding Light Ring.
 * The Blue Orb Actor shrinks after it grows to max size.
 */
#[no_mangle]
pub unsafe extern "C" fn DemoEffect_UpdateBlueOrbShrink(mut this:
                                                            *mut DemoEffect,
                                                        mut globalCtx:
                                                            *mut GlobalContext) {
    (*this).c2rust_unnamed.blueOrb.alpha =
        ((*this).c2rust_unnamed.blueOrb.scale as libc::c_int *
             16 as libc::c_int) as u8_0;
    (*this).c2rust_unnamed.blueOrb.scale =
        (*this).c2rust_unnamed.blueOrb.scale.wrapping_sub(1);
    Actor_SetScale(&mut (*this).actor, (*this).actor.scale.x * 0.9f32);
    if (*this).c2rust_unnamed.blueOrb.scale as libc::c_int == 0 as libc::c_int
       {
        Actor_Kill(&mut (*this).actor);
    };
}
/* *
 * Update action for the Blue Orb Actor.
 * This Update Function is run while the Blue Orb is Growing.
 * The Blue Orb Actor is the blue light sparkle that is in Din's creation cutscene.
 * It's spawned in the middle of the expanding Light Ring.
 * When the scale timer value reaches 0 the Blue Orb's Update Function changes to UpdateBlueOrbShrink.
 */
#[no_mangle]
pub unsafe extern "C" fn DemoEffect_UpdateBlueOrbGrow(mut this:
                                                          *mut DemoEffect,
                                                      mut globalCtx:
                                                          *mut GlobalContext) {
    if !(*this).actor.parent.is_null() {
        // s32 cast necessary to match codegen. Without the explicit cast to u32 the compiler generates complex cast of
        // u8 to float
        Actor_SetScale(&mut (*this).actor,
                       (5.0f32 -
                            (*this).c2rust_unnamed.blueOrb.scale as s32 as
                                libc::c_float) * 0.01f32 * 10.0f32 *
                           (*(*this).actor.parent).scale.x);
    } else {
        Actor_SetScale(&mut (*this).actor,
                       (5.0f32 -
                            (*this).c2rust_unnamed.blueOrb.scale as s32 as
                                libc::c_float) * 0.01f32);
    }
    if (*this).c2rust_unnamed.blueOrb.scale as libc::c_int != 0 as libc::c_int
       {
        (*this).c2rust_unnamed.blueOrb.scale =
            (*this).c2rust_unnamed.blueOrb.scale.wrapping_sub(1)
    } else {
        (*this).c2rust_unnamed.blueOrb.scale = 15 as libc::c_int as u8_0;
        (*this).updateFunc =
            Some(DemoEffect_UpdateBlueOrbShrink as
                     unsafe extern "C" fn(_: *mut DemoEffect,
                                          _: *mut GlobalContext) -> ())
    };
}
/* *
 * Update action for the Light Effect Actor.
 * The Light Effect has various use cases.
 * This function updates the position and scale of the actor based on the current cutscene command.
 */
#[no_mangle]
pub unsafe extern "C" fn DemoEffect_UpdateLightEffect(mut this:
                                                          *mut DemoEffect,
                                                      mut globalCtx:
                                                          *mut GlobalContext) {
    let mut action: u16_0 = 0;
    let mut isLargeSize: s32 = 0;
    isLargeSize =
        ((*this).actor.params as libc::c_int & 0xf00 as libc::c_int) >>
            8 as libc::c_int;
    if (*globalCtx).csCtx.state as libc::c_int != CS_STATE_IDLE as libc::c_int
           &&
           !(*globalCtx).csCtx.npcActions[(*this).csActionId as
                                              usize].is_null() {
        DemoEffect_MoveToCsEndpoint(this, globalCtx,
                                    (*this).csActionId as s32,
                                    0 as libc::c_int);
        match (*(*globalCtx).csCtx.npcActions[(*this).csActionId as
                                                  usize]).action as
                  libc::c_int {
            2 => {
                if ((*this).c2rust_unnamed.light.rotation as libc::c_int) <
                       240 as libc::c_int {
                    if isLargeSize == 0 {
                        if (*this).actor.scale.x < 0.23f32 {
                            (*this).actor.scale.x += 0.001f32;
                            Actor_SetScale(&mut (*this).actor,
                                           (*this).actor.scale.x);
                        }
                    } else if (*this).actor.scale.x < 2.03f32 {
                        (*this).actor.scale.x += 0.05f32;
                        Actor_SetScale(&mut (*this).actor,
                                       (*this).actor.scale.x);
                    }
                }
                (*this).c2rust_unnamed.light.rotation =
                    ((*this).c2rust_unnamed.light.rotation as libc::c_int +
                         6 as libc::c_int) as s16;
                (*this).c2rust_unnamed.light.scaleFlag =
                    ((*this).c2rust_unnamed.light.scaleFlag as libc::c_int +
                         1 as libc::c_int) as u8_0
            }
            3 => {
                Math_SmoothStepToF(&mut (*this).actor.scale.x, 0.0f32, 0.1f32,
                                   0.1f32, 0.005f32);
                Actor_SetScale(&mut (*this).actor, (*this).actor.scale.x);
            }
            _ => { }
        }
        if (*globalCtx).sceneNum as libc::c_int == SCENE_SPOT04 as libc::c_int
               && gSaveContext.sceneSetupIndex == 6 as libc::c_int &&
               (*globalCtx).csCtx.frames as libc::c_int == 197 as libc::c_int
           {
            Audio_PlayActorSound2(&mut (*this).actor,
                                  0x2846 as libc::c_int as u16_0);
        }
        if (*globalCtx).sceneNum as libc::c_int == SCENE_SPOT16 as libc::c_int
               && gSaveContext.sceneSetupIndex == 5 as libc::c_int {
            if DemoEffect_CheckCsAction(this, globalCtx, 1 as libc::c_int) ==
                   0 {
                Audio_PlayActorSound2(&mut (*this).actor,
                                      (0x2847 as libc::c_int -
                                           0x800 as libc::c_int) as u16_0);
            }
            if (*globalCtx).csCtx.frames as libc::c_int == 640 as libc::c_int
               {
                Audio_PlayActorSound2(&mut (*this).actor,
                                      0x2846 as libc::c_int as u16_0);
            }
        }
        if (*globalCtx).sceneNum as libc::c_int == SCENE_SPOT08 as libc::c_int
               && gSaveContext.sceneSetupIndex == 4 as libc::c_int {
            if DemoEffect_CheckCsAction(this, globalCtx, 1 as libc::c_int) ==
                   0 {
                Audio_PlayActorSound2(&mut (*this).actor,
                                      (0x2847 as libc::c_int -
                                           0x800 as libc::c_int) as u16_0);
            }
            if (*globalCtx).csCtx.frames as libc::c_int == 648 as libc::c_int
               {
                Audio_PlayActorSound2(&mut (*this).actor,
                                      0x2846 as libc::c_int as u16_0);
            }
        }
        if (*globalCtx).sceneNum as libc::c_int ==
               SCENE_TOKINOMA as libc::c_int &&
               gSaveContext.sceneSetupIndex == 14 as libc::c_int {
            if (*(*globalCtx).csCtx.npcActions[(*this).csActionId as
                                                   usize]).action as
                   libc::c_int == 2 as libc::c_int {
                Audio_PlayActorSound2(&mut (*this).actor,
                                      (0x2847 as libc::c_int -
                                           0x800 as libc::c_int) as u16_0);
            }
        }
        if (*globalCtx).sceneNum as libc::c_int ==
               SCENE_DAIYOUSEI_IZUMI as libc::c_int ||
               (*globalCtx).sceneNum as libc::c_int ==
                   SCENE_YOUSEI_IZUMI_YOKO as libc::c_int {
            if (*(*globalCtx).csCtx.npcActions[(*this).csActionId as
                                                   usize]).action as
                   libc::c_int == 2 as libc::c_int {
                Audio_PlayActorSound2(&mut (*this).actor,
                                      (0x2847 as libc::c_int -
                                           0x800 as libc::c_int) as u16_0);
            }
        }
    };
}
/* *
 * Update action for the Lgt Shower Actor.
 * The Lgt Shower Actor is the green light effect spawned by Farore in the Kokiri Forst creation cutscene.
 * This function updates the scale and alpha of the Actor.
 */
#[no_mangle]
pub unsafe extern "C" fn DemoEffect_UpdateLgtShower(mut this: *mut DemoEffect,
                                                    mut globalCtx:
                                                        *mut GlobalContext) {
    if (*this).c2rust_unnamed.lgtShower.alpha as libc::c_int >
           3 as libc::c_int {
        (*this).c2rust_unnamed.lgtShower.alpha =
            ((*this).c2rust_unnamed.lgtShower.alpha as libc::c_int -
                 3 as libc::c_int) as u8_0;
        (*this).actor.scale.x *= 1.05f32;
        (*this).actor.scale.y *= 1.05f32;
        (*this).actor.scale.z *= 1.05f32
    } else { Actor_Kill(&mut (*this).actor); };
}
/* *
 * Update action for the God Lgt Din Actor.
 * This is the Goddess Din.
 * This function moves God Lgt Din based on the current cutscene command.
 * This function also spawns a Fireball Actor and sets its update function to the special InitCreationFireball.
 * The spawned Fireball Actor is also scaled to be smaller than regular by this function.
 */
#[no_mangle]
pub unsafe extern "C" fn DemoEffect_UpdateGodLgtDin(mut this: *mut DemoEffect,
                                                    mut globalCtx:
                                                        *mut GlobalContext) {
    let mut fireBall: *mut DemoEffect = 0 as *mut DemoEffect;
    if (*globalCtx).csCtx.state as libc::c_int != CS_STATE_IDLE as libc::c_int
           &&
           !(*globalCtx).csCtx.npcActions[(*this).csActionId as
                                              usize].is_null() {
        DemoEffect_MoveToCsEndpoint(this, globalCtx,
                                    (*this).csActionId as s32,
                                    1 as libc::c_int);
        if (*(*globalCtx).csCtx.npcActions[(*this).csActionId as
                                               usize]).action as libc::c_int
               == 3 as libc::c_int {
            fireBall =
                Actor_SpawnAsChild(&mut (*globalCtx).actorCtx,
                                   &mut (*this).actor, globalCtx,
                                   ACTOR_DEMO_EFFECT as libc::c_int as s16,
                                   (*this).actor.world.pos.x,
                                   (*this).actor.world.pos.y,
                                   (*this).actor.world.pos.z,
                                   0 as libc::c_int as s16,
                                   0 as libc::c_int as s16,
                                   0 as libc::c_int as s16,
                                   DEMO_EFFECT_FIRE_BALL as libc::c_int as
                                       s16) as *mut DemoEffect;
            if !fireBall.is_null() {
                (*fireBall).initUpdateFunc =
                    Some(DemoEffect_InitCreationFireball as
                             unsafe extern "C" fn(_: *mut DemoEffect,
                                                  _: *mut GlobalContext)
                                 -> ());
                Actor_SetScale(&mut (*fireBall).actor, 0.020f32);
            }
        }
        if gSaveContext.entranceIndex == 0xa0 as libc::c_int {
            match gSaveContext.sceneSetupIndex {
                4 => {
                    if (*globalCtx).csCtx.frames as libc::c_int ==
                           288 as libc::c_int {
                        Audio_PlayActorSound2(&mut (*this).actor,
                                              0x1840 as libc::c_int as u16_0);
                    }
                    if (*globalCtx).csCtx.frames as libc::c_int ==
                           635 as libc::c_int {
                        Audio_PlayActorSound2(&mut (*this).actor,
                                              0x1840 as libc::c_int as u16_0);
                    }
                }
                6 => {
                    if (*globalCtx).csCtx.frames as libc::c_int ==
                           55 as libc::c_int {
                        Audio_PlayActorSound2(&mut (*this).actor,
                                              0x1841 as libc::c_int as u16_0);
                    }
                }
                11 => {
                    if (*globalCtx).csCtx.frames as libc::c_int ==
                           350 as libc::c_int {
                        Audio_PlayActorSound2(&mut (*this).actor,
                                              0x1841 as libc::c_int as u16_0);
                    }
                }
                _ => { }
            }
        }
    };
}
/* *
 * Update action for the God Lgt Nayru Actor.
 * This is the Goddess Nayru.
 * This function moves God Lgt Nayure based on the current cutscene command.
 * This function also spawns expanding light rings around Nayru in the creation cutscene
 */
#[no_mangle]
pub unsafe extern "C" fn DemoEffect_UpdateGodLgtNayru(mut this:
                                                          *mut DemoEffect,
                                                      mut globalCtx:
                                                          *mut GlobalContext) {
    let mut lightRing: *mut DemoEffect = 0 as *mut DemoEffect;
    if (*globalCtx).csCtx.state as libc::c_int != CS_STATE_IDLE as libc::c_int
           &&
           !(*globalCtx).csCtx.npcActions[(*this).csActionId as
                                              usize].is_null() {
        DemoEffect_MoveToCsEndpoint(this, globalCtx,
                                    (*this).csActionId as s32,
                                    1 as libc::c_int);
        if (*(*globalCtx).csCtx.npcActions[(*this).csActionId as
                                               usize]).action as libc::c_int
               == 3 as libc::c_int {
            if (*this).c2rust_unnamed.godLgt.lightRingSpawnTimer as
                   libc::c_int != 0 as libc::c_int {
                (*this).c2rust_unnamed.godLgt.lightRingSpawnTimer -= 1
            } else {
                (*this).c2rust_unnamed.godLgt.lightRingSpawnTimer =
                    (*this).c2rust_unnamed.godLgt.lightRingSpawnDelay as s16;
                lightRing =
                    Actor_Spawn(&mut (*globalCtx).actorCtx, globalCtx,
                                ACTOR_DEMO_EFFECT as libc::c_int as s16,
                                (*this).actor.world.pos.x,
                                (*this).actor.world.pos.y,
                                (*this).actor.world.pos.z,
                                ((*this).actor.world.rot.x as libc::c_int +
                                     0x4000 as libc::c_int) as s16,
                                (*this).actor.world.rot.y,
                                (*this).actor.world.rot.z,
                                DEMO_EFFECT_LIGHTRING_EXPANDING as libc::c_int
                                    as s16) as *mut DemoEffect;
                if !lightRing.is_null() {
                    Actor_SetScale(&mut (*lightRing).actor, 1.0f32);
                }
            }
        }
        if gSaveContext.entranceIndex == 0xa0 as libc::c_int {
            match gSaveContext.sceneSetupIndex {
                4 => {
                    if (*globalCtx).csCtx.frames as libc::c_int ==
                           298 as libc::c_int {
                        Audio_PlayActorSound2(&mut (*this).actor,
                                              0x1840 as libc::c_int as u16_0);
                    }
                }
                6 => {
                    if (*globalCtx).csCtx.frames as libc::c_int ==
                           105 as libc::c_int {
                        Audio_PlayActorSound2(&mut (*this).actor,
                                              0x1841 as libc::c_int as u16_0);
                    }
                }
                11 => {
                    if (*globalCtx).csCtx.frames as libc::c_int ==
                           360 as libc::c_int {
                        Audio_PlayActorSound2(&mut (*this).actor,
                                              0x1841 as libc::c_int as u16_0);
                    }
                }
                _ => { }
            }
        }
        if gSaveContext.entranceIndex == 0x13d as libc::c_int &&
               gSaveContext.sceneSetupIndex == 4 as libc::c_int {
            if (*globalCtx).csCtx.frames as libc::c_int == 72 as libc::c_int {
                Audio_PlayActorSound2(&mut (*this).actor,
                                      0x1841 as libc::c_int as u16_0);
            }
            if (*globalCtx).csCtx.frames as libc::c_int == 80 as libc::c_int {
                func_800F3F3C(4 as libc::c_int as u8_0);
            }
        }
    };
}
/* *
 * Update action for the God Lgt Farore Actor.
 * This is the Goddess Farore.
 * This function moves God Lgt Farore based on the current cutscene command.
 * This function also spawns an Lgt Shower Actor during the Kokiri creation cutscene.
 */
#[no_mangle]
pub unsafe extern "C" fn DemoEffect_UpdateGodLgtFarore(mut this:
                                                           *mut DemoEffect,
                                                       mut globalCtx:
                                                           *mut GlobalContext) {
    let mut lgtShower: *mut DemoEffect = 0 as *mut DemoEffect;
    if (*globalCtx).csCtx.state as libc::c_int != CS_STATE_IDLE as libc::c_int
           &&
           !(*globalCtx).csCtx.npcActions[(*this).csActionId as
                                              usize].is_null() {
        DemoEffect_MoveToCsEndpoint(this, globalCtx,
                                    (*this).csActionId as s32,
                                    1 as libc::c_int);
        if (*(*globalCtx).csCtx.npcActions[(*this).csActionId as
                                               usize]).action as libc::c_int
               == 3 as libc::c_int {
            lgtShower =
                Actor_SpawnAsChild(&mut (*globalCtx).actorCtx,
                                   &mut (*this).actor, globalCtx,
                                   ACTOR_DEMO_EFFECT as libc::c_int as s16,
                                   (*this).actor.world.pos.x,
                                   (*this).actor.world.pos.y - 150.0f32,
                                   (*this).actor.world.pos.z,
                                   0 as libc::c_int as s16,
                                   0 as libc::c_int as s16,
                                   0 as libc::c_int as s16,
                                   DEMO_EFFECT_LGT_SHOWER as libc::c_int as
                                       s16) as *mut DemoEffect;
            if !lgtShower.is_null() {
                (*lgtShower).actor.scale.x = 0.23f32;
                (*lgtShower).actor.scale.y = 0.15f32;
                (*lgtShower).actor.scale.z = 0.23f32
            }
            Audio_PlayActorSound2(&mut (*this).actor,
                                  0x1841 as libc::c_int as u16_0);
            func_800F3F3C(3 as libc::c_int as u8_0);
        }
        if gSaveContext.entranceIndex == 0xa0 as libc::c_int {
            match gSaveContext.sceneSetupIndex {
                4 => {
                    if (*globalCtx).csCtx.frames as libc::c_int ==
                           315 as libc::c_int {
                        Audio_PlayActorSound2(&mut (*this).actor,
                                              0x1840 as libc::c_int as u16_0);
                    }
                }
                6 => {
                    if (*globalCtx).csCtx.frames as libc::c_int ==
                           80 as libc::c_int {
                        Audio_PlayActorSound2(&mut (*this).actor,
                                              0x1841 as libc::c_int as u16_0);
                    }
                }
                11 => {
                    if (*globalCtx).csCtx.frames as libc::c_int ==
                           370 as libc::c_int {
                        Audio_PlayActorSound2(&mut (*this).actor,
                                              0x1841 as libc::c_int as u16_0);
                    }
                }
                _ => { }
            }
        }
    };
}
/* *
 * Moves this actor towards the target position with a given speed.
 */
#[no_mangle]
pub unsafe extern "C" fn DemoEffect_MoveTowardTarget(mut targetPos: Vec3f,
                                                     mut this:
                                                         *mut DemoEffect,
                                                     mut speed: f32_0) {
    (*this).actor.world.pos.x +=
        (targetPos.x - (*this).actor.world.pos.x) * speed;
    (*this).actor.world.pos.y +=
        (targetPos.y - (*this).actor.world.pos.y) * speed;
    (*this).actor.world.pos.z +=
        (targetPos.z - (*this).actor.world.pos.z) * speed;
}
/* *
 * Initializes Jewel colors.
 */
#[no_mangle]
pub unsafe extern "C" fn DemoEffect_InitJewelColor(mut this:
                                                       *mut DemoEffect) {
    let mut jewelType: u8_0 = (*this).c2rust_unnamed.jewel.type_0;
    match jewelType as libc::c_int {
        19 => {
            (*this).primXluColor[2 as libc::c_int as usize] =
                160 as libc::c_int as u8_0;
            (*this).primXluColor[0 as libc::c_int as usize] =
                255 as libc::c_int as u8_0;
            (*this).primXluColor[1 as libc::c_int as usize] =
                255 as libc::c_int as u8_0;
            (*this).envXluColor[0 as libc::c_int as usize] =
                0 as libc::c_int as u8_0;
            (*this).envXluColor[1 as libc::c_int as usize] =
                255 as libc::c_int as u8_0;
            (*this).envXluColor[2 as libc::c_int as usize] =
                0 as libc::c_int as u8_0;
            (*this).primOpaColor[2 as libc::c_int as usize] =
                170 as libc::c_int as u8_0;
            (*this).primOpaColor[0 as libc::c_int as usize] =
                255 as libc::c_int as u8_0;
            (*this).primOpaColor[1 as libc::c_int as usize] =
                255 as libc::c_int as u8_0;
            (*this).envOpaColor[1 as libc::c_int as usize] =
                120 as libc::c_int as u8_0;
            (*this).envOpaColor[0 as libc::c_int as usize] =
                150 as libc::c_int as u8_0;
            (*this).envOpaColor[2 as libc::c_int as usize] =
                0 as libc::c_int as u8_0
        }
        20 => {
            (*this).primXluColor[1 as libc::c_int as usize] =
                170 as libc::c_int as u8_0;
            (*this).primXluColor[0 as libc::c_int as usize] =
                255 as libc::c_int as u8_0;
            (*this).primXluColor[2 as libc::c_int as usize] =
                255 as libc::c_int as u8_0;
            (*this).envXluColor[2 as libc::c_int as usize] =
                100 as libc::c_int as u8_0;
            (*this).envXluColor[0 as libc::c_int as usize] =
                255 as libc::c_int as u8_0;
            (*this).envXluColor[1 as libc::c_int as usize] =
                0 as libc::c_int as u8_0;
            (*this).primOpaColor[2 as libc::c_int as usize] =
                170 as libc::c_int as u8_0;
            (*this).primOpaColor[0 as libc::c_int as usize] =
                255 as libc::c_int as u8_0;
            (*this).primOpaColor[1 as libc::c_int as usize] =
                255 as libc::c_int as u8_0;
            (*this).envOpaColor[1 as libc::c_int as usize] =
                120 as libc::c_int as u8_0;
            (*this).envOpaColor[0 as libc::c_int as usize] =
                150 as libc::c_int as u8_0;
            (*this).envOpaColor[2 as libc::c_int as usize] =
                0 as libc::c_int as u8_0
        }
        21 => {
            (*this).primXluColor[0 as libc::c_int as usize] =
                50 as libc::c_int as u8_0;
            (*this).primXluColor[1 as libc::c_int as usize] =
                255 as libc::c_int as u8_0;
            (*this).primXluColor[2 as libc::c_int as usize] =
                255 as libc::c_int as u8_0;
            (*this).envXluColor[2 as libc::c_int as usize] =
                150 as libc::c_int as u8_0;
            (*this).envXluColor[0 as libc::c_int as usize] =
                50 as libc::c_int as u8_0;
            (*this).envXluColor[1 as libc::c_int as usize] =
                0 as libc::c_int as u8_0;
            (*this).primOpaColor[2 as libc::c_int as usize] =
                170 as libc::c_int as u8_0;
            (*this).primOpaColor[0 as libc::c_int as usize] =
                255 as libc::c_int as u8_0;
            (*this).primOpaColor[1 as libc::c_int as usize] =
                255 as libc::c_int as u8_0;
            (*this).envOpaColor[1 as libc::c_int as usize] =
                120 as libc::c_int as u8_0;
            (*this).envOpaColor[0 as libc::c_int as usize] =
                150 as libc::c_int as u8_0;
            (*this).envOpaColor[2 as libc::c_int as usize] =
                0 as libc::c_int as u8_0
        }
        _ => { }
    };
}
/* *
 * Sets the Jewel color based on the alpha variable.
 * This function if a value of less than 1.0f is supplied will drain the color from the Jewels.
 * This effect can be seen in prerelease screenshots.
 */
#[no_mangle]
pub unsafe extern "C" fn DemoEffect_SetJewelColor(mut this: *mut DemoEffect,
                                                  mut alpha: f32_0) {
    DemoEffect_InitJewelColor(this);
    (*this).primXluColor[0 as libc::c_int as usize] =
        ((*this).primXluColor[0 as libc::c_int as usize] as s32 as
             libc::c_float * alpha + 255.0f32 * (1.0f32 - alpha)) as u8_0;
    (*this).primXluColor[1 as libc::c_int as usize] =
        ((*this).primXluColor[1 as libc::c_int as usize] as s32 as
             libc::c_float * alpha + 255.0f32 * (1.0f32 - alpha)) as u8_0;
    (*this).primXluColor[2 as libc::c_int as usize] =
        ((*this).primXluColor[2 as libc::c_int as usize] as s32 as
             libc::c_float * alpha + 255.0f32 * (1.0f32 - alpha)) as u8_0;
    (*this).primOpaColor[0 as libc::c_int as usize] =
        ((*this).primOpaColor[0 as libc::c_int as usize] as s32 as
             libc::c_float * alpha + 255.0f32 * (1.0f32 - alpha)) as u8_0;
    (*this).primOpaColor[1 as libc::c_int as usize] =
        ((*this).primOpaColor[1 as libc::c_int as usize] as s32 as
             libc::c_float * alpha + 255.0f32 * (1.0f32 - alpha)) as u8_0;
    (*this).primOpaColor[2 as libc::c_int as usize] =
        ((*this).primOpaColor[2 as libc::c_int as usize] as s32 as
             libc::c_float * alpha + 255.0f32 * (1.0f32 - alpha)) as u8_0;
    (*this).envXluColor[0 as libc::c_int as usize] =
        ((*this).envXluColor[0 as libc::c_int as usize] as s32 as
             libc::c_float * alpha) as u8_0;
    (*this).envXluColor[1 as libc::c_int as usize] =
        ((*this).envXluColor[1 as libc::c_int as usize] as s32 as
             libc::c_float * alpha) as u8_0;
    (*this).envXluColor[2 as libc::c_int as usize] =
        ((*this).envXluColor[2 as libc::c_int as usize] as s32 as
             libc::c_float * alpha) as u8_0;
    (*this).envOpaColor[0 as libc::c_int as usize] =
        ((*this).envOpaColor[0 as libc::c_int as usize] as s32 as
             libc::c_float * alpha) as u8_0;
    (*this).envOpaColor[1 as libc::c_int as usize] =
        ((*this).envOpaColor[1 as libc::c_int as usize] as s32 as
             libc::c_float * alpha) as u8_0;
    (*this).envOpaColor[2 as libc::c_int as usize] =
        ((*this).envOpaColor[2 as libc::c_int as usize] as s32 as
             libc::c_float * alpha) as u8_0;
}
/* *
 * Moves the Jewel Actor during the activation of the Door of Time cutscene.
 * This is used once the Jewel Actor is done orbiting Link and split up to move into the pedastal slots.
 */
#[no_mangle]
pub unsafe extern "C" fn DemoEffect_MoveJewelSplit(mut world: *mut PosRot,
                                                   mut this:
                                                       *mut DemoEffect) {
    match (*this).c2rust_unnamed.jewel.type_0 as libc::c_int {
        19 => { (*world).pos.x -= 40.0f32 }
        21 => { (*world).pos.x += 40.0f32 }
        20 | _ => { }
    };
}
/* *
 * Moves the Jewel Actor spherically from startPos to endPos.
 * This is used by the Jewel Actor during the Door of Time activation cutscene.
 * This is run when the Jewels merge from Link and begin orbiting him.
 */
#[no_mangle]
pub unsafe extern "C" fn DemoEffect_MoveJewelSpherical(mut degrees: f32_0,
                                                       mut frameDivisor:
                                                           f32_0,
                                                       mut startPos: Vec3f,
                                                       mut endPos: Vec3f,
                                                       mut radius: f32_0,
                                                       mut rotation: Vec3s,
                                                       mut this:
                                                           *mut DemoEffect) {
    let mut pad: s32 = 0;
    let mut pad2: s32 = 0;
    let mut distance: f32_0 = 0.;
    let mut xPos: f32_0 = 0.;
    let mut ySpherical: f32_0 = 0.;
    let mut xzSpherical: f32_0 = 0.;
    distance =
        frameDivisor *
            sqrtf((endPos.x - startPos.x) * (endPos.x - startPos.x) +
                      (endPos.y - startPos.y) * (endPos.y - startPos.y) +
                      (endPos.z - startPos.z) * (endPos.z - startPos.z));
    (*this).actor.world.pos.x =
        radius * cosf(degrees * (3.14159265358979323846f32 / 180.0f32));
    (*this).actor.world.pos.y = distance;
    (*this).actor.world.pos.z =
        radius * sinf(degrees * (3.14159265358979323846f32 / 180.0f32));
    xPos = (*this).actor.world.pos.x;
    ySpherical =
        (*this).actor.world.pos.y *
            cosf(rotation.x as libc::c_int as libc::c_float *
                     (3.14159265358979323846f32 /
                          0x8000 as libc::c_int as libc::c_float)) -
            sinf(rotation.x as libc::c_int as libc::c_float *
                     (3.14159265358979323846f32 /
                          0x8000 as libc::c_int as libc::c_float)) *
                (*this).actor.world.pos.z;
    xzSpherical =
        (*this).actor.world.pos.z *
            cosf(rotation.x as libc::c_int as libc::c_float *
                     (3.14159265358979323846f32 /
                          0x8000 as libc::c_int as libc::c_float)) +
            sinf(rotation.x as libc::c_int as libc::c_float *
                     (3.14159265358979323846f32 /
                          0x8000 as libc::c_int as libc::c_float)) *
                (*this).actor.world.pos.y;
    (*this).actor.world.pos.x =
        xPos *
            cosf(rotation.y as libc::c_int as libc::c_float *
                     (3.14159265358979323846f32 /
                          0x8000 as libc::c_int as libc::c_float)) -
            sinf(rotation.y as libc::c_int as libc::c_float *
                     (3.14159265358979323846f32 /
                          0x8000 as libc::c_int as libc::c_float)) *
                xzSpherical;
    (*this).actor.world.pos.y = ySpherical;
    (*this).actor.world.pos.z =
        xzSpherical *
            cosf(rotation.y as libc::c_int as libc::c_float *
                     (3.14159265358979323846f32 /
                          0x8000 as libc::c_int as libc::c_float)) +
            sinf(rotation.y as libc::c_int as libc::c_float *
                     (3.14159265358979323846f32 /
                          0x8000 as libc::c_int as libc::c_float)) * xPos;
    (*this).actor.world.pos.x += startPos.x;
    (*this).actor.world.pos.y += startPos.y;
    (*this).actor.world.pos.z += startPos.z;
}
/* *
 * Moves the Jewel Actor spherically from startPos to endPos.
 * This is used by the Jewel Actor during the Door of Time activation cutscene.
 * This is run when the Jewels merge from Link and begin orbiting him.
 */
#[no_mangle]
pub unsafe extern "C" fn DemoEffect_MoveJewelActivateDoorOfTime(mut this:
                                                                    *mut DemoEffect,
                                                                mut globalCtx:
                                                                    *mut GlobalContext) {
    let mut startPos: Vec3f = Vec3f{x: 0., y: 0., z: 0.,};
    let mut endPos: Vec3f = Vec3f{x: 0., y: 0., z: 0.,};
    let mut frameDivisor: f32_0 = 0.;
    let mut degrees: f32_0 = 0.;
    let mut radius: f32_0 = 0.;
    let mut csActionId: s32 = 0;
    csActionId = (*this).csActionId as s32;
    startPos.x =
        (*(*globalCtx).csCtx.npcActions[csActionId as usize]).startPos.x as
            f32_0;
    startPos.y =
        (*(*globalCtx).csCtx.npcActions[csActionId as usize]).startPos.y as
            f32_0;
    startPos.z =
        (*(*globalCtx).csCtx.npcActions[csActionId as usize]).startPos.z as
            f32_0;
    endPos.x =
        (*(*globalCtx).csCtx.npcActions[csActionId as usize]).endPos.x as
            f32_0;
    endPos.y =
        (*(*globalCtx).csCtx.npcActions[csActionId as usize]).endPos.y as
            f32_0;
    endPos.z =
        (*(*globalCtx).csCtx.npcActions[csActionId as usize]).endPos.z as
            f32_0;
    frameDivisor = DemoEffect_InterpolateCsFrames(globalCtx, csActionId);
    match (*this).c2rust_unnamed.jewel.type_0 as libc::c_int {
        19 => { degrees = 0.0f32 }
        20 => { degrees = 120.0f32 }
        21 => { degrees = 240.0f32 }
        _ => { }
    }
    radius = 50.0f32 * frameDivisor;
    if radius > 30.0f32 { radius = 30.0f32 }
    if startPos.x != endPos.x || startPos.y != endPos.y ||
           startPos.z != endPos.z {
        (*this).jewelCsRotation.x =
            (Math_Atan2F(endPos.z - startPos.z, -(endPos.x - startPos.x)) *
                 (0x8000 as libc::c_int as libc::c_float /
                      3.14159265358979323846f32)) as s16;
        (*this).jewelCsRotation.y = Math_Vec3f_Yaw(&mut startPos, &mut endPos)
    }
    (*this).jewelCsRotation.z =
        ((*this).jewelCsRotation.z as libc::c_int + 0x400 as libc::c_int) as
            s16;
    degrees +=
        (*this).jewelCsRotation.z as libc::c_int as libc::c_float *
            (360.0f32 / 65536.0f32);
    DemoEffect_MoveJewelSpherical(degrees, frameDivisor, startPos, endPos,
                                  radius, (*this).jewelCsRotation, this);
}
/* *
 * Spawns Sparkle Effects for the Jewel Actor.
 */
#[no_mangle]
pub unsafe extern "C" fn DemoEffect_JewelSparkle(mut this: *mut DemoEffect,
                                                 mut globalCtx:
                                                     *mut GlobalContext,
                                                 mut spawnerCount: s32) {
    let mut velocity: Vec3f = Vec3f{x: 0., y: 0., z: 0.,};
    let mut accel: Vec3f = Vec3f{x: 0., y: 0., z: 0.,};
    let mut primColor: Color_RGBA8 = Color_RGBA8{r: 0, g: 0, b: 0, a: 0,};
    let mut envColor: Color_RGBA8 = Color_RGBA8{r: 0, g: 0, b: 0, a: 0,};
    let mut sparkleColors: *mut Color_RGB8 = 0 as *mut Color_RGB8;
    let mut i: s32 = 0;
    velocity.y = 0.0f32;
    accel.x = 0.0f32;
    accel.y = -0.1f32;
    accel.z = 0.0f32;
    sparkleColors =
        sJewelSparkleColors[((*this).c2rust_unnamed.jewel.type_0 as
                                 libc::c_int -
                                 DEMO_EFFECT_JEWEL_KOKIRI as libc::c_int) as
                                usize].as_mut_ptr();
    primColor.r = (*sparkleColors.offset(0 as libc::c_int as isize)).r;
    primColor.g = (*sparkleColors.offset(0 as libc::c_int as isize)).g;
    primColor.b = (*sparkleColors.offset(0 as libc::c_int as isize)).b;
    envColor.r = (*sparkleColors.offset(1 as libc::c_int as isize)).r;
    envColor.g = (*sparkleColors.offset(1 as libc::c_int as isize)).g;
    envColor.b = (*sparkleColors.offset(1 as libc::c_int as isize)).b;
    primColor.a = 0 as libc::c_int as u8_0;
    i = 0 as libc::c_int;
    while i < spawnerCount {
        velocity.x = (Rand_ZeroOne() - 0.5f32) * 1.5f32;
        velocity.z = (Rand_ZeroOne() - 0.5f32) * 1.5f32;
        EffectSsKiraKira_SpawnDispersed(globalCtx,
                                        &mut (*this).actor.world.pos,
                                        &mut velocity, &mut accel,
                                        &mut primColor, &mut envColor,
                                        3000 as libc::c_int as s16,
                                        16 as libc::c_int);
        i += 1
    };
}
/* *
 * Plays Jewel sound effects.
 * The sSfxJewelId global variable is used to ensure only one Jewel Actor is playing SFX when all are spawned.
 */
#[no_mangle]
pub unsafe extern "C" fn DemoEffect_PlayJewelSfx(mut this: *mut DemoEffect,
                                                 mut globalCtx:
                                                     *mut GlobalContext) {
    if DemoEffect_CheckCsAction(this, globalCtx, 1 as libc::c_int) == 0 {
        if (*this).actor.params as libc::c_int ==
               sSfxJewelId[0 as libc::c_int as usize] as libc::c_int {
            func_8002F974(&mut (*this).actor,
                          (0x286e as libc::c_int - 0x800 as libc::c_int) as
                              u16_0);
        } else if sSfxJewelId[0 as libc::c_int as usize] as libc::c_int ==
                      0 as libc::c_int {
            sSfxJewelId[0 as libc::c_int as usize] = (*this).actor.params;
            func_8002F974(&mut (*this).actor,
                          (0x286e as libc::c_int - 0x800 as libc::c_int) as
                              u16_0);
        }
    };
}
/* *
 * Update Function for the Jewel Actor that is run when Link is an adult.
 * This rotates the Jewel and updates a timer that is used to scroll Jewel textures.
 * There is a call SetJewelColor that does nothing since 1.0f is passed.
 * If a value of less than 1.0f were passed to SetJewelColor, then it would appear to drain the Jewel's color.
 * This can be seen in preprelease screenshots.
 */
#[no_mangle]
pub unsafe extern "C" fn DemoEffect_UpdateJewelAdult(mut this:
                                                         *mut DemoEffect,
                                                     mut globalCtx:
                                                         *mut GlobalContext) {
    (*this).c2rust_unnamed.jewel.timer += 1;
    (*this).actor.shape.rot.y =
        ((*this).actor.shape.rot.y as libc::c_int + 0x400 as libc::c_int) as
            s16;
    DemoEffect_PlayJewelSfx(this, globalCtx);
    DemoEffect_SetJewelColor(this, 1.0f32);
}
/* *
 * Update Function for the Jewel Actor that is run when Link is a child.
 * This rotates the Jewel and updates a timer that is used to scroll Jewel textures.
 * This also updates the Jewel's position based on different cutscenes.
 */
#[no_mangle]
pub unsafe extern "C" fn DemoEffect_UpdateJewelChild(mut this:
                                                         *mut DemoEffect,
                                                     mut globalCtx:
                                                         *mut GlobalContext) {
    let mut hasCmdAction: s32 = 0;
    let mut thisx: *mut Actor = &mut (*this).actor;
    (*this).c2rust_unnamed.jewel.timer += 1;
    if (*globalCtx).csCtx.state as libc::c_int != 0 &&
           !(*globalCtx).csCtx.npcActions[(*this).csActionId as
                                              usize].is_null() {
        match (*(*globalCtx).csCtx.npcActions[(*this).csActionId as
                                                  usize]).action as
                  libc::c_int {
            3 => {
                if gSaveContext.eventChkInf[4 as libc::c_int as usize] as
                       libc::c_int & 0x800 as libc::c_int != 0 {
                    gSaveContext.eventChkInf[4 as libc::c_int as usize] =
                        (gSaveContext.eventChkInf[4 as libc::c_int as usize]
                             as libc::c_int | 0x800 as libc::c_int) as u16_0
                }
                DemoEffect_MoveJewelActivateDoorOfTime(this, globalCtx);
                if (*globalCtx).gameplayFrames &
                       1 as libc::c_int as libc::c_uint ==
                       0 as libc::c_int as libc::c_uint {
                    DemoEffect_JewelSparkle(this, globalCtx,
                                            1 as libc::c_int);
                }
            }
            4 => {
                if (*this).c2rust_unnamed.jewel.isPositionInit != 0 {
                    DemoEffect_MoveToCsEndpoint(this, globalCtx,
                                                (*this).csActionId as s32,
                                                0 as libc::c_int);
                    DemoEffect_MoveJewelSplit(&mut (*thisx).world, this);
                    if (*globalCtx).gameplayFrames &
                           1 as libc::c_int as libc::c_uint ==
                           0 as libc::c_int as libc::c_uint {
                        DemoEffect_JewelSparkle(this, globalCtx,
                                                1 as libc::c_int);
                    }
                } else {
                    DemoEffect_InitPositionFromCsAction(this, globalCtx,
                                                        (*this).csActionId as
                                                            s32);
                    DemoEffect_MoveJewelSplit(&mut (*thisx).world, this);
                    (*this).c2rust_unnamed.jewel.isPositionInit =
                        1 as libc::c_int as u8_0
                }
            }
            6 => { Actor_Kill(thisx); return }
            _ => {
                DemoEffect_MoveToCsEndpoint(this, globalCtx,
                                            (*this).csActionId as s32,
                                            0 as libc::c_int);
                if gSaveContext.entranceIndex == 0x53 as libc::c_int {
                    DemoEffect_MoveJewelSplit(&mut (*thisx).world, this);
                }
            }
        }
    }
    if gSaveContext.entranceIndex == 0x53 as libc::c_int {
        if gSaveContext.eventChkInf[4 as libc::c_int as usize] as libc::c_int
               & 0x800 as libc::c_int == 0 {
            hasCmdAction =
                ((*globalCtx).csCtx.state as libc::c_int != 0 &&
                     !(*globalCtx).csCtx.npcActions[(*this).csActionId as
                                                        usize].is_null()) as
                    libc::c_int;
            if hasCmdAction == 0 {
                (*this).effectFlags =
                    ((*this).effectFlags as libc::c_int | 0x1 as libc::c_int)
                        as s16;
                return
            }
        }
    }
    (*thisx).shape.rot.y =
        ((*thisx).shape.rot.y as libc::c_int + 0x400 as libc::c_int) as s16;
    DemoEffect_PlayJewelSfx(this, globalCtx);
    (*this).effectFlags =
        ((*this).effectFlags as libc::c_int & !(1 as libc::c_int)) as s16;
}
/* *
 * Update Function for the Dust Actor.
 * This is the dust that is spawned in the Temple of Time during the Light Arrows cutscene.
 * This spawns the dust particles and increments a timer
 */
#[no_mangle]
pub unsafe extern "C" fn DemoEffect_UpdateDust(mut this: *mut DemoEffect,
                                               mut globalCtx:
                                                   *mut GlobalContext) {
    let mut pos: Vec3f = Vec3f{x: 0., y: 0., z: 0.,};
    let mut velocity: Vec3f = Vec3f{x: 0., y: 0., z: 0.,};
    let mut accel: Vec3f = Vec3f{x: 0., y: 0., z: 0.,};
    if (*globalCtx).csCtx.state as libc::c_int != CS_STATE_IDLE as libc::c_int
           &&
           !(*globalCtx).csCtx.npcActions[(*this).csActionId as
                                              usize].is_null() &&
           (*(*globalCtx).csCtx.npcActions[(*this).csActionId as
                                               usize]).action as libc::c_int
               == 2 as libc::c_int {
        pos = (*this).actor.world.pos;
        pos.y += 600.0f32;
        pos.x += Rand_CenteredFloat(300.0f32);
        pos.z += 200.0f32 + Rand_CenteredFloat(300.0f32);
        velocity.z = 0.0f32;
        velocity.x = 0.0f32;
        velocity.y = -20.0f32;
        accel.z = 0.0f32;
        accel.x = 0.0f32;
        accel.y = 0.2f32;
        func_8002873C(globalCtx, &mut pos, &mut velocity, &mut accel,
                      300 as libc::c_int as s16, 0 as libc::c_int as s16,
                      30 as libc::c_int as s16);
        (*this).c2rust_unnamed.dust.timer =
            (*this).c2rust_unnamed.dust.timer.wrapping_add(1)
    };
}
/* *
 * This is the main Actor Update Function.
 */
#[no_mangle]
pub unsafe extern "C" fn DemoEffect_Update(mut thisx: *mut Actor,
                                           mut globalCtx:
                                               *mut GlobalContext) {
    let mut this: *mut DemoEffect = thisx as *mut DemoEffect;
    (*this).updateFunc.expect("non-null function pointer")(this, globalCtx);
}
/* *
 * Check if the current cutscene action matches the passed in cutscene action ID.
 */
#[no_mangle]
pub unsafe extern "C" fn DemoEffect_CheckCsAction(mut this: *mut DemoEffect,
                                                  mut globalCtx:
                                                      *mut GlobalContext,
                                                  mut csActionCompareId: s32)
 -> s32 {
    if (*globalCtx).csCtx.state as libc::c_int != CS_STATE_IDLE as libc::c_int
           &&
           !(*globalCtx).csCtx.npcActions[(*this).csActionId as
                                              usize].is_null() &&
           (*(*globalCtx).csCtx.npcActions[(*this).csActionId as
                                               usize]).action as libc::c_int
               == csActionCompareId {
        return 1 as libc::c_int
    }
    return 0 as libc::c_int;
}
/* *
 * Draw function for the Jewel Actor.
 */
#[no_mangle]
pub unsafe extern "C" fn DemoEffect_DrawJewel(mut thisx: *mut Actor,
                                              mut globalCtx2:
                                                  *mut GlobalContext) {
    let mut this: *mut DemoEffect = thisx as *mut DemoEffect;
    let mut globalCtx: *mut GlobalContext = globalCtx2;
    let mut frames: u32_0 = (*this).c2rust_unnamed.jewel.timer as u32_0;
    let mut __gfxCtx: *mut GraphicsContext = 0 as *mut GraphicsContext;
    let mut dispRefs: [*mut Gfx; 4] = [0 as *mut Gfx; 4];
    __gfxCtx = (*globalCtx).state.gfxCtx;
    Graph_OpenDisps(dispRefs.as_mut_ptr(), (*globalCtx).state.gfxCtx,
                    b"../z_demo_effect.c\x00" as *const u8 as
                        *const libc::c_char, 2543 as libc::c_int);
    if DemoEffect_CheckCsAction(this, globalCtx, 1 as libc::c_int) == 0 {
        if (*this).effectFlags as libc::c_int & 0x1 as libc::c_int == 0 {
            match (*this).c2rust_unnamed.jewel.type_0 as libc::c_int {
                19 => {
                    let fresh0 = (*__gfxCtx).polyXlu.p;
                    (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
                    let mut _g: *mut Gfx = fresh0;
                    (*_g).words.w0 =
                        (0xdb as libc::c_int as u32_0 &
                             (((0x1 as libc::c_int) << 8 as libc::c_int) -
                                  1 as libc::c_int) as libc::c_uint) <<
                            24 as libc::c_int |
                            (0x6 as libc::c_int as u32_0 &
                                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                                      1 as libc::c_int) as libc::c_uint) <<
                                16 as libc::c_int |
                            ((9 as libc::c_int * 4 as libc::c_int) as u32_0 &
                                 (((0x1 as libc::c_int) << 16 as libc::c_int)
                                      - 1 as libc::c_int) as libc::c_uint) <<
                                0 as libc::c_int;
                    (*_g).words.w1 =
                        Gfx_TwoTexScroll((*globalCtx).state.gfxCtx,
                                         0 as libc::c_int,
                                         frames.wrapping_mul(4 as libc::c_int
                                                                 as
                                                                 libc::c_uint).wrapping_rem(256
                                                                                                as
                                                                                                libc::c_int
                                                                                                as
                                                                                                libc::c_uint),
                                         (256 as libc::c_int as
                                              libc::c_uint).wrapping_sub(frames.wrapping_mul(2
                                                                                                 as
                                                                                                 libc::c_int
                                                                                                 as
                                                                                                 libc::c_uint).wrapping_rem(256
                                                                                                                                as
                                                                                                                                libc::c_int
                                                                                                                                as
                                                                                                                                libc::c_uint)).wrapping_sub(1
                                                                                                                                                                as
                                                                                                                                                                libc::c_int
                                                                                                                                                                as
                                                                                                                                                                libc::c_uint),
                                         64 as libc::c_int, 64 as libc::c_int,
                                         1 as libc::c_int,
                                         frames.wrapping_mul(2 as libc::c_int
                                                                 as
                                                                 libc::c_uint).wrapping_rem(256
                                                                                                as
                                                                                                libc::c_int
                                                                                                as
                                                                                                libc::c_uint),
                                         (256 as libc::c_int as
                                              libc::c_uint).wrapping_sub(frames.wrapping_rem(256
                                                                                                 as
                                                                                                 libc::c_int
                                                                                                 as
                                                                                                 libc::c_uint)).wrapping_sub(1
                                                                                                                                 as
                                                                                                                                 libc::c_int
                                                                                                                                 as
                                                                                                                                 libc::c_uint),
                                         16 as libc::c_int, 16 as libc::c_int)
                            as libc::c_uint
                }
                20 => {
                    let fresh1 = (*__gfxCtx).polyXlu.p;
                    (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
                    let mut _g_0: *mut Gfx = fresh1;
                    (*_g_0).words.w0 =
                        (0xdb as libc::c_int as u32_0 &
                             (((0x1 as libc::c_int) << 8 as libc::c_int) -
                                  1 as libc::c_int) as libc::c_uint) <<
                            24 as libc::c_int |
                            (0x6 as libc::c_int as u32_0 &
                                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                                      1 as libc::c_int) as libc::c_uint) <<
                                16 as libc::c_int |
                            ((9 as libc::c_int * 4 as libc::c_int) as u32_0 &
                                 (((0x1 as libc::c_int) << 16 as libc::c_int)
                                      - 1 as libc::c_int) as libc::c_uint) <<
                                0 as libc::c_int;
                    (*_g_0).words.w1 =
                        Gfx_TwoTexScroll((*globalCtx).state.gfxCtx,
                                         0 as libc::c_int,
                                         frames.wrapping_mul(4 as libc::c_int
                                                                 as
                                                                 libc::c_uint).wrapping_rem(128
                                                                                                as
                                                                                                libc::c_int
                                                                                                as
                                                                                                libc::c_uint),
                                         (256 as libc::c_int as
                                              libc::c_uint).wrapping_sub(frames.wrapping_mul(2
                                                                                                 as
                                                                                                 libc::c_int
                                                                                                 as
                                                                                                 libc::c_uint).wrapping_rem(256
                                                                                                                                as
                                                                                                                                libc::c_int
                                                                                                                                as
                                                                                                                                libc::c_uint)).wrapping_sub(1
                                                                                                                                                                as
                                                                                                                                                                libc::c_int
                                                                                                                                                                as
                                                                                                                                                                libc::c_uint),
                                         32 as libc::c_int, 64 as libc::c_int,
                                         1 as libc::c_int,
                                         frames.wrapping_mul(2 as libc::c_int
                                                                 as
                                                                 libc::c_uint).wrapping_rem(256
                                                                                                as
                                                                                                libc::c_int
                                                                                                as
                                                                                                libc::c_uint),
                                         (256 as libc::c_int as
                                              libc::c_uint).wrapping_sub(frames.wrapping_rem(256
                                                                                                 as
                                                                                                 libc::c_int
                                                                                                 as
                                                                                                 libc::c_uint)).wrapping_sub(1
                                                                                                                                 as
                                                                                                                                 libc::c_int
                                                                                                                                 as
                                                                                                                                 libc::c_uint),
                                         16 as libc::c_int, 8 as libc::c_int)
                            as libc::c_uint
                }
                21 => {
                    let fresh2 = (*__gfxCtx).polyXlu.p;
                    (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
                    let mut _g_1: *mut Gfx = fresh2;
                    (*_g_1).words.w0 =
                        (0xdb as libc::c_int as u32_0 &
                             (((0x1 as libc::c_int) << 8 as libc::c_int) -
                                  1 as libc::c_int) as libc::c_uint) <<
                            24 as libc::c_int |
                            (0x6 as libc::c_int as u32_0 &
                                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                                      1 as libc::c_int) as libc::c_uint) <<
                                16 as libc::c_int |
                            ((9 as libc::c_int * 4 as libc::c_int) as u32_0 &
                                 (((0x1 as libc::c_int) << 16 as libc::c_int)
                                      - 1 as libc::c_int) as libc::c_uint) <<
                                0 as libc::c_int;
                    (*_g_1).words.w1 =
                        Gfx_TwoTexScroll((*globalCtx).state.gfxCtx,
                                         0 as libc::c_int,
                                         frames.wrapping_mul(4 as libc::c_int
                                                                 as
                                                                 libc::c_uint).wrapping_rem(256
                                                                                                as
                                                                                                libc::c_int
                                                                                                as
                                                                                                libc::c_uint),
                                         (256 as libc::c_int as
                                              libc::c_uint).wrapping_sub(frames.wrapping_mul(2
                                                                                                 as
                                                                                                 libc::c_int
                                                                                                 as
                                                                                                 libc::c_uint).wrapping_rem(256
                                                                                                                                as
                                                                                                                                libc::c_int
                                                                                                                                as
                                                                                                                                libc::c_uint)).wrapping_sub(1
                                                                                                                                                                as
                                                                                                                                                                libc::c_int
                                                                                                                                                                as
                                                                                                                                                                libc::c_uint),
                                         32 as libc::c_int, 32 as libc::c_int,
                                         1 as libc::c_int,
                                         frames.wrapping_mul(2 as libc::c_int
                                                                 as
                                                                 libc::c_uint).wrapping_rem(256
                                                                                                as
                                                                                                libc::c_int
                                                                                                as
                                                                                                libc::c_uint),
                                         (256 as libc::c_int as
                                              libc::c_uint).wrapping_sub(frames.wrapping_rem(256
                                                                                                 as
                                                                                                 libc::c_int
                                                                                                 as
                                                                                                 libc::c_uint)).wrapping_sub(1
                                                                                                                                 as
                                                                                                                                 libc::c_int
                                                                                                                                 as
                                                                                                                                 libc::c_uint),
                                         16 as libc::c_int, 16 as libc::c_int)
                            as libc::c_uint
                }
                _ => { }
            }
            (frames) == 0;
            let fresh3 = (*__gfxCtx).polyOpa.p;
            (*__gfxCtx).polyOpa.p = (*__gfxCtx).polyOpa.p.offset(1);
            let mut _g_2: *mut Gfx = fresh3;
            (*_g_2).words.w0 =
                (0xdb as libc::c_int as u32_0 &
                     (((0x1 as libc::c_int) << 8 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    24 as libc::c_int |
                    (0x6 as libc::c_int as u32_0 &
                         (((0x1 as libc::c_int) << 8 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        16 as libc::c_int |
                    ((8 as libc::c_int * 4 as libc::c_int) as u32_0 &
                         (((0x1 as libc::c_int) << 16 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        0 as libc::c_int;
            (*_g_2).words.w1 =
                Gfx_TexScroll((*globalCtx).state.gfxCtx,
                              frames as u8_0 as u32_0,
                              frames as u8_0 as u32_0, 16 as libc::c_int,
                              16 as libc::c_int) as libc::c_uint;
            let fresh4 = (*__gfxCtx).polyXlu.p;
            (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
            let mut _g_3: *mut Gfx = fresh4;
            (*_g_3).words.w0 =
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
                    (((0 as libc::c_int | 0x2 as libc::c_int |
                           0 as libc::c_int) ^ 0x1 as libc::c_int) as u32_0 &
                         (((0x1 as libc::c_int) << 8 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        0 as libc::c_int;
            (*_g_3).words.w1 =
                Matrix_NewMtx((*globalCtx).state.gfxCtx,
                              b"../z_demo_effect.c\x00" as *const u8 as
                                  *const libc::c_char as *mut libc::c_char,
                              2597 as libc::c_int) as libc::c_uint;
            let fresh5 = (*__gfxCtx).polyOpa.p;
            (*__gfxCtx).polyOpa.p = (*__gfxCtx).polyOpa.p.offset(1);
            let mut _g_4: *mut Gfx = fresh5;
            (*_g_4).words.w0 =
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
                    (((0 as libc::c_int | 0x2 as libc::c_int |
                           0 as libc::c_int) ^ 0x1 as libc::c_int) as u32_0 &
                         (((0x1 as libc::c_int) << 8 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        0 as libc::c_int;
            (*_g_4).words.w1 =
                Matrix_NewMtx((*globalCtx).state.gfxCtx,
                              b"../z_demo_effect.c\x00" as *const u8 as
                                  *const libc::c_char as *mut libc::c_char,
                              2599 as libc::c_int) as libc::c_uint;
            func_80093D84((*globalCtx).state.gfxCtx);
            func_8002ED80(&mut (*this).actor, globalCtx, 0 as libc::c_int);
            let fresh6 = (*__gfxCtx).polyXlu.p;
            (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
            let mut _g_5: *mut Gfx = fresh6;
            (*_g_5).words.w0 =
                (0xfa as libc::c_int as u32_0 &
                     (((0x1 as libc::c_int) << 8 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    24 as libc::c_int |
                    (0 as libc::c_int as u32_0 &
                         (((0x1 as libc::c_int) << 8 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        8 as libc::c_int |
                    (128 as libc::c_int as u32_0 &
                         (((0x1 as libc::c_int) << 8 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        0 as libc::c_int;
            (*_g_5).words.w1 =
                ((*this).primXluColor[0 as libc::c_int as usize] as u32_0 &
                     (((0x1 as libc::c_int) << 8 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    24 as libc::c_int |
                    ((*this).primXluColor[1 as libc::c_int as usize] as u32_0
                         &
                         (((0x1 as libc::c_int) << 8 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        16 as libc::c_int |
                    ((*this).primXluColor[2 as libc::c_int as usize] as u32_0
                         &
                         (((0x1 as libc::c_int) << 8 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        8 as libc::c_int |
                    (255 as libc::c_int as u32_0 &
                         (((0x1 as libc::c_int) << 8 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        0 as libc::c_int;
            let fresh7 = (*__gfxCtx).polyXlu.p;
            (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
            let mut _g_6: *mut Gfx = fresh7;
            (*_g_6).words.w0 =
                (0xfb as libc::c_int as u32_0 &
                     (((0x1 as libc::c_int) << 8 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    24 as libc::c_int;
            (*_g_6).words.w1 =
                ((*this).envXluColor[0 as libc::c_int as usize] as u32_0 &
                     (((0x1 as libc::c_int) << 8 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    24 as libc::c_int |
                    ((*this).envXluColor[1 as libc::c_int as usize] as u32_0 &
                         (((0x1 as libc::c_int) << 8 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        16 as libc::c_int |
                    ((*this).envXluColor[2 as libc::c_int as usize] as u32_0 &
                         (((0x1 as libc::c_int) << 8 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        8 as libc::c_int |
                    (255 as libc::c_int as u32_0 &
                         (((0x1 as libc::c_int) << 8 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        0 as libc::c_int;
            let fresh8 = (*__gfxCtx).polyXlu.p;
            (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
            let mut _g_7: *mut Gfx = fresh8;
            (*_g_7).words.w0 =
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
            (*_g_7).words.w1 = (*this).jewelDisplayList as libc::c_uint;
            func_80093D18((*globalCtx).state.gfxCtx);
            func_8002EBCC(&mut (*this).actor, globalCtx, 0 as libc::c_int);
            let fresh9 = (*__gfxCtx).polyOpa.p;
            (*__gfxCtx).polyOpa.p = (*__gfxCtx).polyOpa.p.offset(1);
            let mut _g_8: *mut Gfx = fresh9;
            (*_g_8).words.w0 =
                (0xfa as libc::c_int as u32_0 &
                     (((0x1 as libc::c_int) << 8 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    24 as libc::c_int |
                    (0 as libc::c_int as u32_0 &
                         (((0x1 as libc::c_int) << 8 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        8 as libc::c_int |
                    (128 as libc::c_int as u32_0 &
                         (((0x1 as libc::c_int) << 8 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        0 as libc::c_int;
            (*_g_8).words.w1 =
                ((*this).primOpaColor[0 as libc::c_int as usize] as u32_0 &
                     (((0x1 as libc::c_int) << 8 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    24 as libc::c_int |
                    ((*this).primOpaColor[1 as libc::c_int as usize] as u32_0
                         &
                         (((0x1 as libc::c_int) << 8 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        16 as libc::c_int |
                    ((*this).primOpaColor[2 as libc::c_int as usize] as u32_0
                         &
                         (((0x1 as libc::c_int) << 8 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        8 as libc::c_int |
                    (255 as libc::c_int as u32_0 &
                         (((0x1 as libc::c_int) << 8 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        0 as libc::c_int;
            let fresh10 = (*__gfxCtx).polyOpa.p;
            (*__gfxCtx).polyOpa.p = (*__gfxCtx).polyOpa.p.offset(1);
            let mut _g_9: *mut Gfx = fresh10;
            (*_g_9).words.w0 =
                (0xfb as libc::c_int as u32_0 &
                     (((0x1 as libc::c_int) << 8 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    24 as libc::c_int;
            (*_g_9).words.w1 =
                ((*this).envOpaColor[0 as libc::c_int as usize] as u32_0 &
                     (((0x1 as libc::c_int) << 8 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    24 as libc::c_int |
                    ((*this).envOpaColor[1 as libc::c_int as usize] as u32_0 &
                         (((0x1 as libc::c_int) << 8 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        16 as libc::c_int |
                    ((*this).envOpaColor[2 as libc::c_int as usize] as u32_0 &
                         (((0x1 as libc::c_int) << 8 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        8 as libc::c_int |
                    (255 as libc::c_int as u32_0 &
                         (((0x1 as libc::c_int) << 8 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        0 as libc::c_int;
            let fresh11 = (*__gfxCtx).polyOpa.p;
            (*__gfxCtx).polyOpa.p = (*__gfxCtx).polyOpa.p.offset(1);
            let mut _g_10: *mut Gfx = fresh11;
            (*_g_10).words.w0 =
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
            (*_g_10).words.w1 = (*this).jewelHolderDisplayList as libc::c_uint
        }
    }
    Graph_CloseDisps(dispRefs.as_mut_ptr(), (*globalCtx).state.gfxCtx,
                     b"../z_demo_effect.c\x00" as *const u8 as
                         *const libc::c_char, 2620 as libc::c_int);
}
/* *
 * Draw function for the Crystal Light Actor.
 */
#[no_mangle]
pub unsafe extern "C" fn DemoEffect_DrawCrystalLight(mut thisx: *mut Actor,
                                                     mut globalCtx:
                                                         *mut GlobalContext) {
    let mut this: *mut DemoEffect = thisx as *mut DemoEffect;
    let mut parent: *mut DemoEffect = (*this).actor.parent as *mut DemoEffect;
    let mut frames: u32_0 =
        (*globalCtx).gameplayFrames & 0xffff as libc::c_int as libc::c_uint;
    let mut __gfxCtx: *mut GraphicsContext = 0 as *mut GraphicsContext;
    let mut dispRefs: [*mut Gfx; 4] = [0 as *mut Gfx; 4];
    __gfxCtx = (*globalCtx).state.gfxCtx;
    Graph_OpenDisps(dispRefs.as_mut_ptr(), (*globalCtx).state.gfxCtx,
                    b"../z_demo_effect.c\x00" as *const u8 as
                        *const libc::c_char, 2634 as libc::c_int);
    if !parent.is_null() {
        let fresh12 = (*__gfxCtx).polyXlu.p;
        (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
        let mut _g: *mut Gfx = fresh12;
        (*_g).words.w0 =
            (0xfa as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 24 as libc::c_int
                |
                (128 as libc::c_int as u32_0 &
                     (((0x1 as libc::c_int) << 8 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    8 as libc::c_int |
                (128 as libc::c_int as u32_0 &
                     (((0x1 as libc::c_int) << 8 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    0 as libc::c_int;
        (*_g).words.w1 =
            (255 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 24 as libc::c_int
                |
                (255 as libc::c_int as u32_0 &
                     (((0x1 as libc::c_int) << 8 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    16 as libc::c_int |
                (170 as libc::c_int as u32_0 &
                     (((0x1 as libc::c_int) << 8 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    8 as libc::c_int |
                ((*parent).c2rust_unnamed.triforceSpot.crystalLightOpacity as
                     u32_0 &
                     (((0x1 as libc::c_int) << 8 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    0 as libc::c_int
    } else {
        let fresh13 = (*__gfxCtx).polyXlu.p;
        (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
        let mut _g_0: *mut Gfx = fresh13;
        (*_g_0).words.w0 =
            (0xfa as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 24 as libc::c_int
                |
                (128 as libc::c_int as u32_0 &
                     (((0x1 as libc::c_int) << 8 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    8 as libc::c_int |
                (128 as libc::c_int as u32_0 &
                     (((0x1 as libc::c_int) << 8 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    0 as libc::c_int;
        (*_g_0).words.w1 =
            (255 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 24 as libc::c_int
                |
                (255 as libc::c_int as u32_0 &
                     (((0x1 as libc::c_int) << 8 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    16 as libc::c_int |
                (170 as libc::c_int as u32_0 &
                     (((0x1 as libc::c_int) << 8 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    8 as libc::c_int |
                (255 as libc::c_int as u32_0 &
                     (((0x1 as libc::c_int) << 8 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    0 as libc::c_int
    }
    func_80093D84((*globalCtx).state.gfxCtx);
    let fresh14 = (*__gfxCtx).polyXlu.p;
    (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
    let mut _g_1: *mut Gfx = fresh14;
    (*_g_1).words.w0 =
        (0xdb as libc::c_int as u32_0 &
             (((0x1 as libc::c_int) << 8 as libc::c_int) - 1 as libc::c_int)
                 as libc::c_uint) << 24 as libc::c_int |
            (0x6 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 16 as libc::c_int
            |
            ((8 as libc::c_int * 4 as libc::c_int) as u32_0 &
                 (((0x1 as libc::c_int) << 16 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 0 as libc::c_int;
    (*_g_1).words.w1 =
        Gfx_TwoTexScroll((*globalCtx).state.gfxCtx, 0 as libc::c_int,
                         frames.wrapping_mul(2 as libc::c_int as
                                                 libc::c_uint).wrapping_rem(512
                                                                                as
                                                                                libc::c_int
                                                                                as
                                                                                libc::c_uint),
                         (512 as libc::c_int as
                              libc::c_uint).wrapping_sub(frames.wrapping_rem(512
                                                                                 as
                                                                                 libc::c_int
                                                                                 as
                                                                                 libc::c_uint)).wrapping_sub(1
                                                                                                                 as
                                                                                                                 libc::c_int
                                                                                                                 as
                                                                                                                 libc::c_uint),
                         128 as libc::c_int, 128 as libc::c_int,
                         1 as libc::c_int,
                         (512 as libc::c_int as
                              libc::c_uint).wrapping_sub(frames.wrapping_mul(2
                                                                                 as
                                                                                 libc::c_int
                                                                                 as
                                                                                 libc::c_uint).wrapping_rem(512
                                                                                                                as
                                                                                                                libc::c_int
                                                                                                                as
                                                                                                                libc::c_uint)).wrapping_sub(1
                                                                                                                                                as
                                                                                                                                                libc::c_int
                                                                                                                                                as
                                                                                                                                                libc::c_uint),
                         0 as libc::c_int as u32_0, 64 as libc::c_int,
                         64 as libc::c_int) as libc::c_uint;
    Matrix_Push();
    Matrix_RotateY(0.0f32, MTXMODE_APPLY as libc::c_int as u8_0);
    Matrix_RotateX((11.0f64 * 3.14159265358979323846f32 as libc::c_double /
                        180.0f64) as f32_0,
                   MTXMODE_APPLY as libc::c_int as u8_0);
    Matrix_Translate(0.0f32, 150.0f32, 0.0f32,
                     MTXMODE_APPLY as libc::c_int as u8_0);
    let fresh15 = (*__gfxCtx).polyXlu.p;
    (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
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
            (((0 as libc::c_int | 0x2 as libc::c_int | 0 as libc::c_int) ^
                  0x1 as libc::c_int) as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 0 as libc::c_int;
    (*_g_2).words.w1 =
        Matrix_NewMtx((*globalCtx).state.gfxCtx,
                      b"../z_demo_effect.c\x00" as *const u8 as
                          *const libc::c_char as *mut libc::c_char,
                      2661 as libc::c_int) as libc::c_uint;
    let fresh16 = (*__gfxCtx).polyXlu.p;
    (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
    let mut _g_3: *mut Gfx = fresh16;
    (*_g_3).words.w0 =
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
    (*_g_3).words.w1 = gCrystalLightDL.as_mut_ptr() as libc::c_uint;
    Matrix_Pop();
    Matrix_Push();
    Matrix_RotateY(2.0f32 * 3.14159265358979323846f32 / 3.0f32,
                   MTXMODE_APPLY as libc::c_int as u8_0);
    Matrix_RotateX((11.0f64 * 3.14159265358979323846f32 as libc::c_double /
                        180.0f64) as f32_0,
                   MTXMODE_APPLY as libc::c_int as u8_0);
    Matrix_Translate(0.0f32, 150.0f32, 0.0f32,
                     MTXMODE_APPLY as libc::c_int as u8_0);
    let fresh17 = (*__gfxCtx).polyXlu.p;
    (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
    let mut _g_4: *mut Gfx = fresh17;
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
    (*_g_4).words.w1 =
        Matrix_NewMtx((*globalCtx).state.gfxCtx,
                      b"../z_demo_effect.c\x00" as *const u8 as
                          *const libc::c_char as *mut libc::c_char,
                      2672 as libc::c_int) as libc::c_uint;
    let fresh18 = (*__gfxCtx).polyXlu.p;
    (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
    let mut _g_5: *mut Gfx = fresh18;
    (*_g_5).words.w0 =
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
    (*_g_5).words.w1 = gCrystalLightDL.as_mut_ptr() as libc::c_uint;
    Matrix_Pop();
    Matrix_Push();
    Matrix_RotateY(4.0f32 * 3.14159265358979323846f32 / 3.0f32,
                   MTXMODE_APPLY as libc::c_int as u8_0);
    Matrix_RotateX((11.0f64 * 3.14159265358979323846f32 as libc::c_double /
                        180.0f64) as f32_0,
                   MTXMODE_APPLY as libc::c_int as u8_0);
    Matrix_Translate(0.0f32, 150.0f32, 0.0f32,
                     MTXMODE_APPLY as libc::c_int as u8_0);
    let fresh19 = (*__gfxCtx).polyXlu.p;
    (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
    let mut _g_6: *mut Gfx = fresh19;
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
            (((0 as libc::c_int | 0x2 as libc::c_int | 0 as libc::c_int) ^
                  0x1 as libc::c_int) as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 0 as libc::c_int;
    (*_g_6).words.w1 =
        Matrix_NewMtx((*globalCtx).state.gfxCtx,
                      b"../z_demo_effect.c\x00" as *const u8 as
                          *const libc::c_char as *mut libc::c_char,
                      2683 as libc::c_int) as libc::c_uint;
    let fresh20 = (*__gfxCtx).polyXlu.p;
    (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
    let mut _g_7: *mut Gfx = fresh20;
    (*_g_7).words.w0 =
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
    (*_g_7).words.w1 = gCrystalLightDL.as_mut_ptr() as libc::c_uint;
    Matrix_Pop();
    Graph_CloseDisps(dispRefs.as_mut_ptr(), (*globalCtx).state.gfxCtx,
                     b"../z_demo_effect.c\x00" as *const u8 as
                         *const libc::c_char, 2688 as libc::c_int);
}
/* *
 * Draw function for the Fire Ball Actor.
 */
#[no_mangle]
pub unsafe extern "C" fn DemoEffect_DrawFireBall(mut thisx: *mut Actor,
                                                 mut globalCtx:
                                                     *mut GlobalContext) {
    let mut this: *mut DemoEffect = thisx as *mut DemoEffect;
    let mut frames: u32_0 = (*globalCtx).gameplayFrames;
    let mut __gfxCtx: *mut GraphicsContext = 0 as *mut GraphicsContext;
    let mut dispRefs: [*mut Gfx; 4] = [0 as *mut Gfx; 4];
    __gfxCtx = (*globalCtx).state.gfxCtx;
    Graph_OpenDisps(dispRefs.as_mut_ptr(), (*globalCtx).state.gfxCtx,
                    b"../z_demo_effect.c\x00" as *const u8 as
                        *const libc::c_char, 2701 as libc::c_int);
    let fresh21 = (*__gfxCtx).polyXlu.p;
    (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
    let mut _g: *mut Gfx = fresh21;
    (*_g).words.w0 =
        (0xfa as libc::c_int as u32_0 &
             (((0x1 as libc::c_int) << 8 as libc::c_int) - 1 as libc::c_int)
                 as libc::c_uint) << 24 as libc::c_int |
            (64 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 8 as libc::c_int |
            (64 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 0 as libc::c_int;
    (*_g).words.w1 =
        (255 as libc::c_int as u32_0 &
             (((0x1 as libc::c_int) << 8 as libc::c_int) - 1 as libc::c_int)
                 as libc::c_uint) << 24 as libc::c_int |
            (200 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 16 as libc::c_int
            |
            (0 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 8 as libc::c_int |
            (255 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 0 as libc::c_int;
    let fresh22 = (*__gfxCtx).polyXlu.p;
    (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
    let mut _g_0: *mut Gfx = fresh22;
    (*_g_0).words.w0 =
        (0xfb as libc::c_int as u32_0 &
             (((0x1 as libc::c_int) << 8 as libc::c_int) - 1 as libc::c_int)
                 as libc::c_uint) << 24 as libc::c_int;
    (*_g_0).words.w1 =
        (255 as libc::c_int as u32_0 &
             (((0x1 as libc::c_int) << 8 as libc::c_int) - 1 as libc::c_int)
                 as libc::c_uint) << 24 as libc::c_int |
            (0 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 16 as libc::c_int
            |
            (0 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 8 as libc::c_int |
            (255 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 0 as libc::c_int;
    func_80093D84((*globalCtx).state.gfxCtx);
    let fresh23 = (*__gfxCtx).polyXlu.p;
    (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
    let mut _g_1: *mut Gfx = fresh23;
    (*_g_1).words.w0 =
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
    (*_g_1).words.w1 =
        Matrix_NewMtx((*globalCtx).state.gfxCtx,
                      b"../z_demo_effect.c\x00" as *const u8 as
                          *const libc::c_char as *mut libc::c_char,
                      2709 as libc::c_int) as libc::c_uint;
    let fresh24 = (*__gfxCtx).polyXlu.p;
    (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
    let mut _g_2: *mut Gfx = fresh24;
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
            (((0 as libc::c_int | 0 as libc::c_int | 0 as libc::c_int) ^
                  0x1 as libc::c_int) as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 0 as libc::c_int;
    (*_g_2).words.w1 = (*globalCtx).billboardMtx as libc::c_uint;
    let fresh25 = (*__gfxCtx).polyXlu.p;
    (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
    let mut _g_3: *mut Gfx = fresh25;
    (*_g_3).words.w0 =
        (0xdb as libc::c_int as u32_0 &
             (((0x1 as libc::c_int) << 8 as libc::c_int) - 1 as libc::c_int)
                 as libc::c_uint) << 24 as libc::c_int |
            (0x6 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 16 as libc::c_int
            |
            ((8 as libc::c_int * 4 as libc::c_int) as u32_0 &
                 (((0x1 as libc::c_int) << 16 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 0 as libc::c_int;
    (*_g_3).words.w1 =
        Gfx_TwoTexScroll((*globalCtx).state.gfxCtx, 0 as libc::c_int,
                         0 as libc::c_int as u32_0, 0 as libc::c_int as u32_0,
                         32 as libc::c_int, 32 as libc::c_int,
                         1 as libc::c_int, 0 as libc::c_int as u32_0,
                         (128 as libc::c_int as
                              libc::c_uint).wrapping_sub(frames.wrapping_mul(20
                                                                                 as
                                                                                 libc::c_int
                                                                                 as
                                                                                 libc::c_uint).wrapping_rem(128
                                                                                                                as
                                                                                                                libc::c_int
                                                                                                                as
                                                                                                                libc::c_uint)).wrapping_sub(1
                                                                                                                                                as
                                                                                                                                                libc::c_int
                                                                                                                                                as
                                                                                                                                                libc::c_uint),
                         32 as libc::c_int, 32 as libc::c_int) as
            libc::c_uint;
    let fresh26 = (*__gfxCtx).polyXlu.p;
    (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
    let mut _g_4: *mut Gfx = fresh26;
    (*_g_4).words.w0 =
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
    (*_g_4).words.w1 = gCreationFireBallDL.as_mut_ptr() as libc::c_uint;
    Graph_CloseDisps(dispRefs.as_mut_ptr(), (*globalCtx).state.gfxCtx,
                     b"../z_demo_effect.c\x00" as *const u8 as
                         *const libc::c_char, 2723 as libc::c_int);
}
/* *
 * Draw function for the God Lgt Actors.
 * This draws either Din, Nayru, or Farore based on the colors set in the DemoEffect struct.
 */
#[no_mangle]
pub unsafe extern "C" fn DemoEffect_DrawGodLgt(mut thisx: *mut Actor,
                                               mut globalCtx:
                                                   *mut GlobalContext) {
    let mut this: *mut DemoEffect = thisx as *mut DemoEffect;
    let mut pad: s32 = 0;
    let mut frames: u32_0 = (*globalCtx).gameplayFrames;
    let mut __gfxCtx: *mut GraphicsContext = 0 as *mut GraphicsContext;
    let mut dispRefs: [*mut Gfx; 4] = [0 as *mut Gfx; 4];
    __gfxCtx = (*globalCtx).state.gfxCtx;
    Graph_OpenDisps(dispRefs.as_mut_ptr(), (*globalCtx).state.gfxCtx,
                    b"../z_demo_effect.c\x00" as *const u8 as
                        *const libc::c_char, 2737 as libc::c_int);
    if DemoEffect_CheckCsAction(this, globalCtx, 2 as libc::c_int) == 0 {
        if gSaveContext.entranceIndex == 0xa0 as libc::c_int {
            if gSaveContext.sceneSetupIndex == 4 as libc::c_int {
                if (*globalCtx).csCtx.frames as libc::c_int <=
                       680 as libc::c_int {
                    func_80078914(&mut (*this).actor.projectedPos,
                                  (0x288b as libc::c_int -
                                       0x800 as libc::c_int) as u16_0);
                }
            } else {
                func_80078914(&mut (*this).actor.projectedPos,
                              (0x288b as libc::c_int - 0x800 as libc::c_int)
                                  as u16_0);
            }
        } else {
            func_80078914(&mut (*this).actor.projectedPos,
                          (0x288b as libc::c_int - 0x800 as libc::c_int) as
                              u16_0);
        }
        let fresh27 = (*__gfxCtx).polyXlu.p;
        (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
        let mut _g: *mut Gfx = fresh27;
        (*_g).words.w0 =
            (0xdb as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 24 as libc::c_int
                |
                (0x6 as libc::c_int as u32_0 &
                     (((0x1 as libc::c_int) << 8 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    16 as libc::c_int |
                ((8 as libc::c_int * 4 as libc::c_int) as u32_0 &
                     (((0x1 as libc::c_int) << 16 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    0 as libc::c_int;
        (*_g).words.w1 =
            Gfx_TwoTexScroll((*globalCtx).state.gfxCtx, 0 as libc::c_int,
                             frames.wrapping_mul(4 as libc::c_int as
                                                     libc::c_uint).wrapping_rem(512
                                                                                    as
                                                                                    libc::c_int
                                                                                    as
                                                                                    libc::c_uint),
                             0 as libc::c_int as u32_0, 128 as libc::c_int,
                             64 as libc::c_int, 1 as libc::c_int,
                             frames.wrapping_mul(2 as libc::c_int as
                                                     libc::c_uint).wrapping_rem(256
                                                                                    as
                                                                                    libc::c_int
                                                                                    as
                                                                                    libc::c_uint),
                             (512 as libc::c_int as
                                  libc::c_uint).wrapping_sub(frames.wrapping_mul(70
                                                                                     as
                                                                                     libc::c_int
                                                                                     as
                                                                                     libc::c_uint).wrapping_rem(512
                                                                                                                    as
                                                                                                                    libc::c_int
                                                                                                                    as
                                                                                                                    libc::c_uint)).wrapping_sub(1
                                                                                                                                                    as
                                                                                                                                                    libc::c_int
                                                                                                                                                    as
                                                                                                                                                    libc::c_uint),
                             64 as libc::c_int, 32 as libc::c_int) as
                libc::c_uint;
        let fresh28 = (*__gfxCtx).polyXlu.p;
        (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
        let mut _g_0: *mut Gfx = fresh28;
        (*_g_0).words.w0 =
            (0xdb as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 24 as libc::c_int
                |
                (0x6 as libc::c_int as u32_0 &
                     (((0x1 as libc::c_int) << 8 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    16 as libc::c_int |
                ((9 as libc::c_int * 4 as libc::c_int) as u32_0 &
                     (((0x1 as libc::c_int) << 16 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    0 as libc::c_int;
        (*_g_0).words.w1 =
            Gfx_TwoTexScroll((*globalCtx).state.gfxCtx, 0 as libc::c_int,
                             0 as libc::c_int as u32_0,
                             0 as libc::c_int as u32_0, 16 as libc::c_int,
                             96 as libc::c_int, 1 as libc::c_int,
                             frames.wrapping_mul(10 as libc::c_int as
                                                     libc::c_uint).wrapping_rem(256
                                                                                    as
                                                                                    libc::c_int
                                                                                    as
                                                                                    libc::c_uint),
                             (256 as libc::c_int as
                                  libc::c_uint).wrapping_sub(frames.wrapping_mul(30
                                                                                     as
                                                                                     libc::c_int
                                                                                     as
                                                                                     libc::c_uint).wrapping_rem(512
                                                                                                                    as
                                                                                                                    libc::c_int
                                                                                                                    as
                                                                                                                    libc::c_uint)).wrapping_sub(1
                                                                                                                                                    as
                                                                                                                                                    libc::c_int
                                                                                                                                                    as
                                                                                                                                                    libc::c_uint),
                             8 as libc::c_int, 32 as libc::c_int) as
                libc::c_uint;
        let fresh29 = (*__gfxCtx).polyXlu.p;
        (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
        let mut _g_1: *mut Gfx = fresh29;
        (*_g_1).words.w0 =
            (0xfa as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 24 as libc::c_int
                |
                (128 as libc::c_int as u32_0 &
                     (((0x1 as libc::c_int) << 8 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    8 as libc::c_int |
                (128 as libc::c_int as u32_0 &
                     (((0x1 as libc::c_int) << 8 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    0 as libc::c_int;
        (*_g_1).words.w1 =
            ((*this).primXluColor[0 as libc::c_int as usize] as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 24 as libc::c_int
                |
                ((*this).primXluColor[1 as libc::c_int as usize] as u32_0 &
                     (((0x1 as libc::c_int) << 8 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    16 as libc::c_int |
                ((*this).primXluColor[2 as libc::c_int as usize] as u32_0 &
                     (((0x1 as libc::c_int) << 8 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    8 as libc::c_int |
                (255 as libc::c_int as u32_0 &
                     (((0x1 as libc::c_int) << 8 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    0 as libc::c_int;
        let fresh30 = (*__gfxCtx).polyXlu.p;
        (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
        let mut _g_2: *mut Gfx = fresh30;
        (*_g_2).words.w0 =
            (0xfb as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 24 as libc::c_int;
        (*_g_2).words.w1 =
            ((*this).envXluColor[0 as libc::c_int as usize] as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 24 as libc::c_int
                |
                ((*this).envXluColor[1 as libc::c_int as usize] as u32_0 &
                     (((0x1 as libc::c_int) << 8 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    16 as libc::c_int |
                ((*this).envXluColor[2 as libc::c_int as usize] as u32_0 &
                     (((0x1 as libc::c_int) << 8 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    8 as libc::c_int |
                (255 as libc::c_int as u32_0 &
                     (((0x1 as libc::c_int) << 8 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    0 as libc::c_int;
        func_80093D84((*globalCtx).state.gfxCtx);
        Matrix_Push();
        let fresh31 = (*__gfxCtx).polyXlu.p;
        (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
        let mut _g_3: *mut Gfx = fresh31;
        (*_g_3).words.w0 =
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
                (((0 as libc::c_int | 0x2 as libc::c_int | 0 as libc::c_int) ^
                      0x1 as libc::c_int) as u32_0 &
                     (((0x1 as libc::c_int) << 8 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    0 as libc::c_int;
        (*_g_3).words.w1 =
            Matrix_NewMtx((*globalCtx).state.gfxCtx,
                          b"../z_demo_effect.c\x00" as *const u8 as
                              *const libc::c_char as *mut libc::c_char,
                          2801 as libc::c_int) as libc::c_uint;
        let fresh32 = (*__gfxCtx).polyXlu.p;
        (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
        let mut _g_4: *mut Gfx = fresh32;
        (*_g_4).words.w0 =
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
        (*_g_4).words.w1 = gGoldenGoddessAuraDL.as_mut_ptr() as libc::c_uint;
        func_80093D18((*globalCtx).state.gfxCtx);
        func_8002EBCC(&mut (*this).actor, globalCtx, 0 as libc::c_int);
        Matrix_Pop();
        (*this).c2rust_unnamed.godLgt.rotation =
            (*this).c2rust_unnamed.godLgt.rotation.wrapping_add(1);
        if (*this).c2rust_unnamed.godLgt.rotation as libc::c_int >
               120 as libc::c_int {
            (*this).c2rust_unnamed.godLgt.rotation = 0 as libc::c_int as u8_0
        }
        Matrix_RotateZ((*this).c2rust_unnamed.godLgt.rotation as s32 as
                           libc::c_float * 3.0f32 *
                           (3.14159265358979323846f32 / 180.0f32),
                       MTXMODE_APPLY as libc::c_int as u8_0);
        Matrix_RotateX(3.14159265358979323846f32 / 2.0f32,
                       MTXMODE_APPLY as libc::c_int as u8_0);
        Matrix_Translate(0.0f32, -140.0f32, 0.0f32,
                         MTXMODE_APPLY as libc::c_int as u8_0);
        Matrix_Scale(0.03f32, 0.03f32, 0.03f32,
                     MTXMODE_APPLY as libc::c_int as u8_0);
        let fresh33 = (*__gfxCtx).polyOpa.p;
        (*__gfxCtx).polyOpa.p = (*__gfxCtx).polyOpa.p.offset(1);
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
                (((0 as libc::c_int | 0x2 as libc::c_int | 0 as libc::c_int) ^
                      0x1 as libc::c_int) as u32_0 &
                     (((0x1 as libc::c_int) << 8 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    0 as libc::c_int;
        (*_g_5).words.w1 =
            Matrix_NewMtx((*globalCtx).state.gfxCtx,
                          b"../z_demo_effect.c\x00" as *const u8 as
                              *const libc::c_char as *mut libc::c_char,
                          2824 as libc::c_int) as libc::c_uint;
        let fresh34 = (*__gfxCtx).polyOpa.p;
        (*__gfxCtx).polyOpa.p = (*__gfxCtx).polyOpa.p.offset(1);
        let mut _g_6: *mut Gfx = fresh34;
        (*_g_6).words.w0 =
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
        (*_g_6).words.w1 = gGoldenGoddessBodyDL.as_mut_ptr() as libc::c_uint
    }
    Graph_CloseDisps(dispRefs.as_mut_ptr(), (*globalCtx).state.gfxCtx,
                     b"../z_demo_effect.c\x00" as *const u8 as
                         *const libc::c_char, 2829 as libc::c_int);
}
/* *
 * Draw function for the Light Effect Actor.
 */
#[no_mangle]
pub unsafe extern "C" fn DemoEffect_DrawLightEffect(mut thisx: *mut Actor,
                                                    mut globalCtx:
                                                        *mut GlobalContext) {
    let mut this: *mut DemoEffect =
        thisx as
            *mut DemoEffect; // necessary to match, should be able to remove after fake matches are fixed
    let mut alpha: *mut u8_0 = 0 as *mut u8_0;
    let mut disp: *mut Gfx = 0 as *mut Gfx;
    let mut __gfxCtx: *mut GraphicsContext = 0 as *mut GraphicsContext;
    let mut dispRefs: [*mut Gfx; 4] = [0 as *mut Gfx; 4];
    __gfxCtx = (*globalCtx).state.gfxCtx;
    Graph_OpenDisps(dispRefs.as_mut_ptr(), (*globalCtx).state.gfxCtx,
                    b"../z_demo_effect.c\x00" as *const u8 as
                        *const libc::c_char, 2842 as libc::c_int);
    if DemoEffect_CheckCsAction(this, globalCtx, 1 as libc::c_int) == 0 {
        if (*this).c2rust_unnamed.light.flicker as libc::c_int ==
               0 as libc::c_int {
            (*this).c2rust_unnamed.light.flicker = 1 as libc::c_int as u8_0
        } else {
            disp = gEffFlash1DL.as_mut_ptr() as u32_0 as *mut Gfx;
            alpha = &mut (*this).c2rust_unnamed.light.alpha;
            func_80093D84((*globalCtx).state.gfxCtx);
            let fresh35 = (*__gfxCtx).polyXlu.p;
            (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
            let mut _g: *mut Gfx = fresh35;
            (*_g).words.w0 =
                (0xfa as libc::c_int as u32_0 &
                     (((0x1 as libc::c_int) << 8 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    24 as libc::c_int |
                    (0 as libc::c_int as u32_0 &
                         (((0x1 as libc::c_int) << 8 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        8 as libc::c_int |
                    (128 as libc::c_int as u32_0 &
                         (((0x1 as libc::c_int) << 8 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        0 as libc::c_int;
            (*_g).words.w1 =
                ((*this).primXluColor[0 as libc::c_int as usize] as u32_0 &
                     (((0x1 as libc::c_int) << 8 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    24 as libc::c_int |
                    ((*this).primXluColor[1 as libc::c_int as usize] as u32_0
                         &
                         (((0x1 as libc::c_int) << 8 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        16 as libc::c_int |
                    ((*this).primXluColor[2 as libc::c_int as usize] as u32_0
                         &
                         (((0x1 as libc::c_int) << 8 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        8 as libc::c_int |
                    (*alpha as u32_0 &
                         (((0x1 as libc::c_int) << 8 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        0 as libc::c_int;
            let fresh36 = (*__gfxCtx).polyXlu.p;
            (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
            let mut _g_0: *mut Gfx = fresh36;
            (*_g_0).words.w0 =
                (0xfb as libc::c_int as u32_0 &
                     (((0x1 as libc::c_int) << 8 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    24 as libc::c_int;
            (*_g_0).words.w1 =
                ((*this).envXluColor[0 as libc::c_int as usize] as u32_0 &
                     (((0x1 as libc::c_int) << 8 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    24 as libc::c_int |
                    ((*this).envXluColor[1 as libc::c_int as usize] as u32_0 &
                         (((0x1 as libc::c_int) << 8 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        16 as libc::c_int |
                    ((*this).envXluColor[2 as libc::c_int as usize] as u32_0 &
                         (((0x1 as libc::c_int) << 8 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        8 as libc::c_int |
                    (255 as libc::c_int as u32_0 &
                         (((0x1 as libc::c_int) << 8 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        0 as libc::c_int;
            Matrix_Scale(((*this).c2rust_unnamed.light.scaleFlag as
                              libc::c_int & 1 as libc::c_int) as libc::c_float
                             * 0.05f32 + 1.0f32,
                         ((*this).c2rust_unnamed.light.scaleFlag as
                              libc::c_int & 1 as libc::c_int) as libc::c_float
                             * 0.05f32 + 1.0f32,
                         ((*this).c2rust_unnamed.light.scaleFlag as
                              libc::c_int & 1 as libc::c_int) as libc::c_float
                             * 0.05f32 + 1.0f32,
                         MTXMODE_APPLY as libc::c_int as u8_0);
            Matrix_Push();
            Matrix_Mult(&mut (*globalCtx).billboardMtxF,
                        MTXMODE_APPLY as libc::c_int as u8_0);
            Matrix_RotateZ((*this).c2rust_unnamed.light.rotation as
                               libc::c_int as libc::c_float *
                               (3.14159265358979323846f32 / 180.0f32),
                           MTXMODE_APPLY as libc::c_int as u8_0);
            let fresh37 = (*__gfxCtx).polyXlu.p;
            (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
            let mut _g_1: *mut Gfx = fresh37;
            (*_g_1).words.w0 =
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
                    (((0 as libc::c_int | 0x2 as libc::c_int |
                           0 as libc::c_int) ^ 0x1 as libc::c_int) as u32_0 &
                         (((0x1 as libc::c_int) << 8 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        0 as libc::c_int;
            (*_g_1).words.w1 =
                Matrix_NewMtx((*globalCtx).state.gfxCtx,
                              b"../z_demo_effect.c\x00" as *const u8 as
                                  *const libc::c_char as *mut libc::c_char,
                              2866 as libc::c_int) as libc::c_uint;
            !disp.is_null();
            let fresh38 = (*__gfxCtx).polyXlu.p;
            (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
            let mut _g_2: *mut Gfx = fresh38;
            (*_g_2).words.w0 =
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
            (*_g_2).words.w1 = disp as libc::c_uint;
            Matrix_Pop();
            Matrix_Mult(&mut (*globalCtx).billboardMtxF,
                        MTXMODE_APPLY as libc::c_int as u8_0);
            Matrix_RotateZ(-((*this).c2rust_unnamed.light.rotation as f32_0) *
                               (3.14159265358979323846f32 / 180.0f32),
                           MTXMODE_APPLY as libc::c_int as u8_0);
            let fresh39 = (*__gfxCtx).polyXlu.p;
            (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
            let mut _g_3: *mut Gfx = fresh39;
            (*_g_3).words.w0 =
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
                    (((0 as libc::c_int | 0x2 as libc::c_int |
                           0 as libc::c_int) ^ 0x1 as libc::c_int) as u32_0 &
                         (((0x1 as libc::c_int) << 8 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        0 as libc::c_int;
            (*_g_3).words.w1 =
                Matrix_NewMtx((*globalCtx).state.gfxCtx,
                              b"../z_demo_effect.c\x00" as *const u8 as
                                  *const libc::c_char as *mut libc::c_char,
                              2874 as libc::c_int) as libc::c_uint;
            let fresh40 = (*__gfxCtx).polyXlu.p;
            (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
            let mut _g_4: *mut Gfx = fresh40;
            (*_g_4).words.w0 =
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
            (*_g_4).words.w1 = disp as libc::c_uint
        }
    }
    Graph_CloseDisps(dispRefs.as_mut_ptr(), (*globalCtx).state.gfxCtx,
                     b"../z_demo_effect.c\x00" as *const u8 as
                         *const libc::c_char, 2881 as libc::c_int);
}
/* *
 * Draw function for the Blue Orb Actor.
 */
#[no_mangle]
pub unsafe extern "C" fn DemoEffect_DrawBlueOrb(mut thisx: *mut Actor,
                                                mut globalCtx:
                                                    *mut GlobalContext) {
    let mut this: *mut DemoEffect = thisx as *mut DemoEffect;
    let mut pad2: s32 = 0;
    let mut __gfxCtx: *mut GraphicsContext = 0 as *mut GraphicsContext;
    let mut dispRefs: [*mut Gfx; 4] = [0 as *mut Gfx; 4];
    __gfxCtx = (*globalCtx).state.gfxCtx;
    Graph_OpenDisps(dispRefs.as_mut_ptr(), (*globalCtx).state.gfxCtx,
                    b"../z_demo_effect.c\x00" as *const u8 as
                        *const libc::c_char, 2892 as libc::c_int);
    let fresh41 = (*__gfxCtx).polyXlu.p;
    (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
    let mut _g: *mut Gfx = fresh41;
    (*_g).words.w0 =
        (0xfa as libc::c_int as u32_0 &
             (((0x1 as libc::c_int) << 8 as libc::c_int) - 1 as libc::c_int)
                 as libc::c_uint) << 24 as libc::c_int |
            (128 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 8 as libc::c_int |
            (128 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 0 as libc::c_int;
    (*_g).words.w1 =
        (188 as libc::c_int as u32_0 &
             (((0x1 as libc::c_int) << 8 as libc::c_int) - 1 as libc::c_int)
                 as libc::c_uint) << 24 as libc::c_int |
            (255 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 16 as libc::c_int
            |
            (255 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 8 as libc::c_int |
            ((*this).c2rust_unnamed.blueOrb.alpha as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 0 as libc::c_int;
    let fresh42 = (*__gfxCtx).polyXlu.p;
    (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
    let mut _g_0: *mut Gfx = fresh42;
    (*_g_0).words.w0 =
        (0xfb as libc::c_int as u32_0 &
             (((0x1 as libc::c_int) << 8 as libc::c_int) - 1 as libc::c_int)
                 as libc::c_uint) << 24 as libc::c_int;
    (*_g_0).words.w1 =
        (0 as libc::c_int as u32_0 &
             (((0x1 as libc::c_int) << 8 as libc::c_int) - 1 as libc::c_int)
                 as libc::c_uint) << 24 as libc::c_int |
            (100 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 16 as libc::c_int
            |
            (255 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 8 as libc::c_int |
            (255 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 0 as libc::c_int;
    func_80093D84((*globalCtx).state.gfxCtx);
    Matrix_Mult(&mut (*globalCtx).billboardMtxF,
                MTXMODE_APPLY as libc::c_int as u8_0);
    Matrix_RotateZ((*this).c2rust_unnamed.blueOrb.rotation as libc::c_int as
                       libc::c_float *
                       (3.14159265358979323846f32 /
                            0x8000 as libc::c_int as libc::c_float),
                   MTXMODE_APPLY as libc::c_int as u8_0);
    let fresh43 = (*__gfxCtx).polyXlu.p;
    (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
    let mut _g_1: *mut Gfx = fresh43;
    (*_g_1).words.w0 =
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
    (*_g_1).words.w1 =
        Matrix_NewMtx((*globalCtx).state.gfxCtx,
                      b"../z_demo_effect.c\x00" as *const u8 as
                          *const libc::c_char as *mut libc::c_char,
                      2901 as libc::c_int) as libc::c_uint;
    (*this).c2rust_unnamed.blueOrb.rotation =
        ((*this).c2rust_unnamed.blueOrb.rotation as libc::c_int +
             0x1f4 as libc::c_int) as s16;
    let fresh44 = (*__gfxCtx).polyXlu.p;
    (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
    let mut _g_2: *mut Gfx = fresh44;
    (*_g_2).words.w0 =
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
    (*_g_2).words.w1 = gEffFlash1DL.as_mut_ptr() as libc::c_uint;
    Graph_CloseDisps(dispRefs.as_mut_ptr(), (*globalCtx).state.gfxCtx,
                     b"../z_demo_effect.c\x00" as *const u8 as
                         *const libc::c_char, 2907 as libc::c_int);
}
/* *
 * Draw function for the Lgt Shower Actor.
 */
#[no_mangle]
pub unsafe extern "C" fn DemoEffect_DrawLgtShower(mut thisx: *mut Actor,
                                                  mut globalCtx:
                                                      *mut GlobalContext) {
    let mut this: *mut DemoEffect = thisx as *mut DemoEffect;
    let mut pad: s32 = 0;
    let mut frames: u32_0 = (*globalCtx).gameplayFrames;
    let mut __gfxCtx: *mut GraphicsContext = 0 as *mut GraphicsContext;
    let mut dispRefs: [*mut Gfx; 4] = [0 as *mut Gfx; 4];
    __gfxCtx = (*globalCtx).state.gfxCtx;
    Graph_OpenDisps(dispRefs.as_mut_ptr(), (*globalCtx).state.gfxCtx,
                    b"../z_demo_effect.c\x00" as *const u8 as
                        *const libc::c_char, 2921 as libc::c_int);
    let fresh45 = (*__gfxCtx).polyXlu.p;
    (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
    let mut _g: *mut Gfx = fresh45;
    (*_g).words.w0 =
        (0xfa as libc::c_int as u32_0 &
             (((0x1 as libc::c_int) << 8 as libc::c_int) - 1 as libc::c_int)
                 as libc::c_uint) << 24 as libc::c_int |
            (64 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 8 as libc::c_int |
            (64 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 0 as libc::c_int;
    (*_g).words.w1 =
        (255 as libc::c_int as u32_0 &
             (((0x1 as libc::c_int) << 8 as libc::c_int) - 1 as libc::c_int)
                 as libc::c_uint) << 24 as libc::c_int |
            (255 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 16 as libc::c_int
            |
            (160 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 8 as libc::c_int |
            ((*this).c2rust_unnamed.lgtShower.alpha as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 0 as libc::c_int;
    let fresh46 = (*__gfxCtx).polyXlu.p;
    (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
    let mut _g_0: *mut Gfx = fresh46;
    (*_g_0).words.w0 =
        (0xfb as libc::c_int as u32_0 &
             (((0x1 as libc::c_int) << 8 as libc::c_int) - 1 as libc::c_int)
                 as libc::c_uint) << 24 as libc::c_int;
    (*_g_0).words.w1 =
        (50 as libc::c_int as u32_0 &
             (((0x1 as libc::c_int) << 8 as libc::c_int) - 1 as libc::c_int)
                 as libc::c_uint) << 24 as libc::c_int |
            (200 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 16 as libc::c_int
            |
            (0 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 8 as libc::c_int |
            (255 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 0 as libc::c_int;
    func_80093D84((*globalCtx).state.gfxCtx);
    let fresh47 = (*__gfxCtx).polyXlu.p;
    (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
    let mut _g_1: *mut Gfx = fresh47;
    (*_g_1).words.w0 =
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
    (*_g_1).words.w1 =
        Matrix_NewMtx((*globalCtx).state.gfxCtx,
                      b"../z_demo_effect.c\x00" as *const u8 as
                          *const libc::c_char as *mut libc::c_char,
                      2927 as libc::c_int) as libc::c_uint;
    let fresh48 = (*__gfxCtx).polyXlu.p;
    (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
    let mut _g_2: *mut Gfx = fresh48;
    (*_g_2).words.w0 =
        (0xdb as libc::c_int as u32_0 &
             (((0x1 as libc::c_int) << 8 as libc::c_int) - 1 as libc::c_int)
                 as libc::c_uint) << 24 as libc::c_int |
            (0x6 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 16 as libc::c_int
            |
            ((8 as libc::c_int * 4 as libc::c_int) as u32_0 &
                 (((0x1 as libc::c_int) << 16 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 0 as libc::c_int;
    (*_g_2).words.w1 =
        Gfx_TwoTexScroll((*globalCtx).state.gfxCtx, 0 as libc::c_int,
                         frames.wrapping_mul(5 as libc::c_int as
                                                 libc::c_uint).wrapping_rem(1024
                                                                                as
                                                                                libc::c_int
                                                                                as
                                                                                libc::c_uint),
                         0 as libc::c_int as u32_0, 256 as libc::c_int,
                         64 as libc::c_int, 1 as libc::c_int,
                         frames.wrapping_mul(10 as libc::c_int as
                                                 libc::c_uint).wrapping_rem(128
                                                                                as
                                                                                libc::c_int
                                                                                as
                                                                                libc::c_uint),
                         (512 as libc::c_int as
                              libc::c_uint).wrapping_sub(frames.wrapping_mul(50
                                                                                 as
                                                                                 libc::c_int
                                                                                 as
                                                                                 libc::c_uint).wrapping_rem(512
                                                                                                                as
                                                                                                                libc::c_int
                                                                                                                as
                                                                                                                libc::c_uint)),
                         32 as libc::c_int, 16 as libc::c_int) as
            libc::c_uint;
    let fresh49 = (*__gfxCtx).polyXlu.p;
    (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
    let mut _g_3: *mut Gfx = fresh49;
    (*_g_3).words.w0 =
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
    (*_g_3).words.w1 = gEnliveningLightDL.as_mut_ptr() as libc::c_uint;
    Graph_CloseDisps(dispRefs.as_mut_ptr(), (*globalCtx).state.gfxCtx,
                     b"../z_demo_effect.c\x00" as *const u8 as
                         *const libc::c_char, 2942 as libc::c_int);
}
/* *
 * Draw function for the Light Ring Actor.
 */
#[no_mangle]
pub unsafe extern "C" fn DemoEffect_DrawLightRing(mut thisx: *mut Actor,
                                                  mut globalCtx2:
                                                      *mut GlobalContext) {
    let mut this: *mut DemoEffect = thisx as *mut DemoEffect;
    let mut globalCtx: *mut GlobalContext = globalCtx2;
    let mut frames: u32_0 = (*this).c2rust_unnamed.lightRing.timer as u32_0;
    let mut __gfxCtx: *mut GraphicsContext = 0 as *mut GraphicsContext;
    let mut dispRefs: [*mut Gfx; 4] = [0 as *mut Gfx; 4];
    __gfxCtx = (*globalCtx).state.gfxCtx;
    Graph_OpenDisps(dispRefs.as_mut_ptr(), (*globalCtx).state.gfxCtx,
                    b"../z_demo_effect.c\x00" as *const u8 as
                        *const libc::c_char, 2956 as libc::c_int);
    func_80093D84((*globalCtx).state.gfxCtx);
    let fresh50 = (*__gfxCtx).polyXlu.p;
    (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
    let mut _g: *mut Gfx = fresh50;
    (*_g).words.w0 =
        (0xfa as libc::c_int as u32_0 &
             (((0x1 as libc::c_int) << 8 as libc::c_int) - 1 as libc::c_int)
                 as libc::c_uint) << 24 as libc::c_int |
            (128 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 8 as libc::c_int |
            (128 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 0 as libc::c_int;
    (*_g).words.w1 =
        (170 as libc::c_int as u32_0 &
             (((0x1 as libc::c_int) << 8 as libc::c_int) - 1 as libc::c_int)
                 as libc::c_uint) << 24 as libc::c_int |
            (255 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 16 as libc::c_int
            |
            (255 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 8 as libc::c_int |
            ((*this).c2rust_unnamed.lightRing.alpha as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 0 as libc::c_int;
    let fresh51 = (*__gfxCtx).polyXlu.p;
    (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
    let mut _g_0: *mut Gfx = fresh51;
    (*_g_0).words.w0 =
        (0xfb as libc::c_int as u32_0 &
             (((0x1 as libc::c_int) << 8 as libc::c_int) - 1 as libc::c_int)
                 as libc::c_uint) << 24 as libc::c_int;
    (*_g_0).words.w1 =
        (0 as libc::c_int as u32_0 &
             (((0x1 as libc::c_int) << 8 as libc::c_int) - 1 as libc::c_int)
                 as libc::c_uint) << 24 as libc::c_int |
            (100 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 16 as libc::c_int
            |
            (255 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 8 as libc::c_int |
            (255 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 0 as libc::c_int;
    let fresh52 = (*__gfxCtx).polyXlu.p;
    (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
    let mut _g_1: *mut Gfx = fresh52;
    (*_g_1).words.w0 =
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
    (*_g_1).words.w1 =
        Matrix_NewMtx((*globalCtx).state.gfxCtx,
                      b"../z_demo_effect.c\x00" as *const u8 as
                          *const libc::c_char as *mut libc::c_char,
                      2963 as libc::c_int) as libc::c_uint;
    let fresh53 = (*__gfxCtx).polyXlu.p;
    (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
    let mut _g_2: *mut Gfx = fresh53;
    (*_g_2).words.w0 =
        (0xdb as libc::c_int as u32_0 &
             (((0x1 as libc::c_int) << 8 as libc::c_int) - 1 as libc::c_int)
                 as libc::c_uint) << 24 as libc::c_int |
            (0x6 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 16 as libc::c_int
            |
            ((8 as libc::c_int * 4 as libc::c_int) as u32_0 &
                 (((0x1 as libc::c_int) << 16 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 0 as libc::c_int;
    (*_g_2).words.w1 =
        Gfx_TwoTexScroll((*globalCtx).state.gfxCtx, 0 as libc::c_int,
                         frames.wrapping_mul(5 as libc::c_int as
                                                 libc::c_uint).wrapping_rem(64
                                                                                as
                                                                                libc::c_int
                                                                                as
                                                                                libc::c_uint),
                         (512 as libc::c_int as
                              libc::c_uint).wrapping_sub(frames.wrapping_mul(2
                                                                                 as
                                                                                 libc::c_int
                                                                                 as
                                                                                 libc::c_uint).wrapping_rem(512
                                                                                                                as
                                                                                                                libc::c_int
                                                                                                                as
                                                                                                                libc::c_uint)).wrapping_sub(1
                                                                                                                                                as
                                                                                                                                                libc::c_int
                                                                                                                                                as
                                                                                                                                                libc::c_uint),
                         16 as libc::c_int, 128 as libc::c_int,
                         1 as libc::c_int, 0 as libc::c_int as u32_0,
                         0 as libc::c_int as u32_0, 8 as libc::c_int,
                         1024 as libc::c_int) as libc::c_uint;
    let fresh54 = (*__gfxCtx).polyXlu.p;
    (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
    let mut _g_3: *mut Gfx = fresh54;
    (*_g_3).words.w0 =
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
    (*_g_3).words.w1 = gGoldenGoddessLightRingDL.as_mut_ptr() as libc::c_uint;
    Graph_CloseDisps(dispRefs.as_mut_ptr(), (*globalCtx).state.gfxCtx,
                     b"../z_demo_effect.c\x00" as *const u8 as
                         *const libc::c_char, 2978 as libc::c_int);
}
/* *
 * Draw function for the Triforce Spot Actor.
 */
#[no_mangle]
pub unsafe extern "C" fn DemoEffect_DrawTriforceSpot(mut thisx: *mut Actor,
                                                     mut globalCtx:
                                                         *mut GlobalContext) {
    let mut this: *mut DemoEffect = thisx as *mut DemoEffect;
    let mut pad: s32 = 0;
    let mut vertices: *mut Vtx =
        gSegments[((gTriforceVtx.as_mut_ptr() as u32_0) << 4 as libc::c_int >>
                       28 as libc::c_int) as
                      usize].wrapping_add(gTriforceVtx.as_mut_ptr() as u32_0 &
                                              0xffffff as libc::c_int as
                                                  libc::c_uint).wrapping_add(0x80000000
                                                                                 as
                                                                                 libc::c_uint)
            as *mut libc::c_void as *mut Vtx;
    let mut frames: u32_0 = (*globalCtx).gameplayFrames;
    let mut __gfxCtx: *mut GraphicsContext = 0 as *mut GraphicsContext;
    let mut dispRefs: [*mut Gfx; 4] = [0 as *mut Gfx; 4];
    __gfxCtx = (*globalCtx).state.gfxCtx;
    Graph_OpenDisps(dispRefs.as_mut_ptr(), (*globalCtx).state.gfxCtx,
                    b"../z_demo_effect.c\x00" as *const u8 as
                        *const libc::c_char, 2994 as libc::c_int);
    if gSaveContext.entranceIndex != 0x400 as libc::c_int ||
           ((*globalCtx).csCtx.frames as libc::c_int) < 885 as libc::c_int {
        func_80093D84((*globalCtx).state.gfxCtx);
        if (*this).c2rust_unnamed.triforceSpot.lightColumnOpacity as
               libc::c_int > 0 as libc::c_int {
            Audio_PlayActorSound2(&mut (*this).actor,
                                  (0x288d as libc::c_int -
                                       0x800 as libc::c_int) as u16_0);
            Matrix_Push();
            Matrix_Scale(1.0f32, 2.4f32, 1.0f32,
                         MTXMODE_APPLY as libc::c_int as u8_0);
            let fresh55 = (*__gfxCtx).polyXlu.p;
            (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
            let mut _g: *mut Gfx = fresh55;
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
                    (((0 as libc::c_int | 0x2 as libc::c_int |
                           0 as libc::c_int) ^ 0x1 as libc::c_int) as u32_0 &
                         (((0x1 as libc::c_int) << 8 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        0 as libc::c_int;
            (*_g).words.w1 =
                Matrix_NewMtx((*globalCtx).state.gfxCtx,
                              b"../z_demo_effect.c\x00" as *const u8 as
                                  *const libc::c_char as *mut libc::c_char,
                              3011 as libc::c_int) as libc::c_uint;
            let fresh56 = (*__gfxCtx).polyXlu.p;
            (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
            let mut _g_0: *mut Gfx = fresh56;
            (*_g_0).words.w0 =
                (0xdb as libc::c_int as u32_0 &
                     (((0x1 as libc::c_int) << 8 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    24 as libc::c_int |
                    (0x6 as libc::c_int as u32_0 &
                         (((0x1 as libc::c_int) << 8 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        16 as libc::c_int |
                    ((9 as libc::c_int * 4 as libc::c_int) as u32_0 &
                         (((0x1 as libc::c_int) << 16 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        0 as libc::c_int;
            (*_g_0).words.w1 =
                Gfx_TwoTexScroll((*globalCtx).state.gfxCtx, 0 as libc::c_int,
                                 0 as libc::c_int as u32_0,
                                 (256 as libc::c_int as
                                      libc::c_uint).wrapping_sub(frames.wrapping_mul(4
                                                                                         as
                                                                                         libc::c_int
                                                                                         as
                                                                                         libc::c_uint).wrapping_rem(256
                                                                                                                        as
                                                                                                                        libc::c_int
                                                                                                                        as
                                                                                                                        libc::c_uint)).wrapping_sub(1
                                                                                                                                                        as
                                                                                                                                                        libc::c_int
                                                                                                                                                        as
                                                                                                                                                        libc::c_uint),
                                 64 as libc::c_int, 64 as libc::c_int,
                                 1 as libc::c_int, 0 as libc::c_int as u32_0,
                                 (256 as libc::c_int as
                                      libc::c_uint).wrapping_sub(frames.wrapping_mul(2
                                                                                         as
                                                                                         libc::c_int
                                                                                         as
                                                                                         libc::c_uint).wrapping_rem(256
                                                                                                                        as
                                                                                                                        libc::c_int
                                                                                                                        as
                                                                                                                        libc::c_uint)).wrapping_sub(1
                                                                                                                                                        as
                                                                                                                                                        libc::c_int
                                                                                                                                                        as
                                                                                                                                                        libc::c_uint),
                                 64 as libc::c_int, 32 as libc::c_int) as
                    libc::c_uint;
            let ref mut fresh57 =
                (*vertices.offset(95 as libc::c_int as isize)).n.a;
            *fresh57 =
                (*this).c2rust_unnamed.triforceSpot.lightColumnOpacity as s8
                    as libc::c_uchar;
            let ref mut fresh58 =
                (*vertices.offset(94 as libc::c_int as isize)).n.a;
            *fresh58 = *fresh57;
            let ref mut fresh59 =
                (*vertices.offset(93 as libc::c_int as isize)).n.a;
            *fresh59 = *fresh58;
            let ref mut fresh60 =
                (*vertices.offset(92 as libc::c_int as isize)).n.a;
            *fresh60 = *fresh59;
            let ref mut fresh61 =
                (*vertices.offset(89 as libc::c_int as isize)).n.a;
            *fresh61 = *fresh60;
            let ref mut fresh62 =
                (*vertices.offset(88 as libc::c_int as isize)).n.a;
            *fresh62 = *fresh61;
            let ref mut fresh63 =
                (*vertices.offset(87 as libc::c_int as isize)).n.a;
            *fresh63 = *fresh62;
            (*vertices.offset(86 as libc::c_int as isize)).n.a = *fresh63;
            let fresh64 = (*__gfxCtx).polyXlu.p;
            (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
            let mut _g_1: *mut Gfx = fresh64;
            (*_g_1).words.w0 =
                (0xfa as libc::c_int as u32_0 &
                     (((0x1 as libc::c_int) << 8 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    24 as libc::c_int |
                    (128 as libc::c_int as u32_0 &
                         (((0x1 as libc::c_int) << 8 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        8 as libc::c_int |
                    (128 as libc::c_int as u32_0 &
                         (((0x1 as libc::c_int) << 8 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        0 as libc::c_int;
            (*_g_1).words.w1 =
                (180 as libc::c_int as u32_0 &
                     (((0x1 as libc::c_int) << 8 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    24 as libc::c_int |
                    (255 as libc::c_int as u32_0 &
                         (((0x1 as libc::c_int) << 8 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        16 as libc::c_int |
                    (255 as libc::c_int as u32_0 &
                         (((0x1 as libc::c_int) << 8 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        8 as libc::c_int |
                    ((*this).c2rust_unnamed.triforceSpot.lightColumnOpacity as
                         u32_0 &
                         (((0x1 as libc::c_int) << 8 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        0 as libc::c_int;
            let fresh65 = (*__gfxCtx).polyXlu.p;
            (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
            let mut _g_2: *mut Gfx = fresh65;
            (*_g_2).words.w0 =
                (0xfb as libc::c_int as u32_0 &
                     (((0x1 as libc::c_int) << 8 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    24 as libc::c_int;
            (*_g_2).words.w1 =
                (0 as libc::c_int as u32_0 &
                     (((0x1 as libc::c_int) << 8 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    24 as libc::c_int |
                    (255 as libc::c_int as u32_0 &
                         (((0x1 as libc::c_int) << 8 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        16 as libc::c_int |
                    (150 as libc::c_int as u32_0 &
                         (((0x1 as libc::c_int) << 8 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        8 as libc::c_int |
                    (255 as libc::c_int as u32_0 &
                         (((0x1 as libc::c_int) << 8 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        0 as libc::c_int;
            let fresh66 = (*__gfxCtx).polyXlu.p;
            (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
            let mut _g_3: *mut Gfx = fresh66;
            (*_g_3).words.w0 =
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
            (*_g_3).words.w1 =
                gTriforceLightColumnDL.as_mut_ptr() as libc::c_uint;
            Matrix_Pop();
        }
        if (*this).c2rust_unnamed.triforceSpot.triforceSpotOpacity as
               libc::c_int != 0 as libc::c_int {
            Audio_PlayActorSound2(&mut (*this).actor,
                                  (0x288c as libc::c_int -
                                       0x800 as libc::c_int) as u16_0);
            let fresh67 = (*__gfxCtx).polyXlu.p;
            (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
            let mut _g_4: *mut Gfx = fresh67;
            (*_g_4).words.w0 =
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
                    (((0 as libc::c_int | 0x2 as libc::c_int |
                           0 as libc::c_int) ^ 0x1 as libc::c_int) as u32_0 &
                         (((0x1 as libc::c_int) << 8 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        0 as libc::c_int;
            (*_g_4).words.w1 =
                Matrix_NewMtx((*globalCtx).state.gfxCtx,
                              b"../z_demo_effect.c\x00" as *const u8 as
                                  *const libc::c_char as *mut libc::c_char,
                              3042 as libc::c_int) as libc::c_uint;
            if ((*this).c2rust_unnamed.triforceSpot.triforceSpotOpacity as
                    libc::c_int) < 250 as libc::c_int {
                func_8002ED80(&mut (*this).actor, globalCtx,
                              0 as libc::c_int);
                func_80093D84((*globalCtx).state.gfxCtx);
                let fresh68 = (*__gfxCtx).polyXlu.p;
                (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
                let mut _g_5: *mut Gfx = fresh68;
                (*_g_5).words.w0 =
                    (0xe2 as libc::c_int as u32_0 &
                         (((0x1 as libc::c_int) << 8 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        24 as libc::c_int |
                        ((32 as libc::c_int - 3 as libc::c_int -
                              29 as libc::c_int) as u32_0 &
                             (((0x1 as libc::c_int) << 8 as libc::c_int) -
                                  1 as libc::c_int) as libc::c_uint) <<
                            8 as libc::c_int |
                        ((29 as libc::c_int - 1 as libc::c_int) as u32_0 &
                             (((0x1 as libc::c_int) << 8 as libc::c_int) -
                                  1 as libc::c_int) as libc::c_uint) <<
                            0 as libc::c_int;
                (*_g_5).words.w1 =
                    ((0 as libc::c_int) << 30 as libc::c_int |
                         (3 as libc::c_int) << 26 as libc::c_int |
                         (0 as libc::c_int) << 22 as libc::c_int |
                         (2 as libc::c_int) << 18 as libc::c_int |
                         (0x8 as libc::c_int | 0x10 as libc::c_int |
                              0x40 as libc::c_int | 0x100 as libc::c_int |
                              0x80 as libc::c_int | 0x4000 as libc::c_int |
                              0x800 as libc::c_int |
                              (0 as libc::c_int) << 28 as libc::c_int |
                              (0 as libc::c_int) << 24 as libc::c_int |
                              (1 as libc::c_int) << 20 as libc::c_int |
                              (0 as libc::c_int) << 16 as libc::c_int)) as
                        libc::c_uint;
                Matrix_RotateY((*this).c2rust_unnamed.triforceSpot.rotation as
                                   libc::c_int as libc::c_float *
                                   (3.14159265358979323846f32 /
                                        0x8000 as libc::c_int as
                                            libc::c_float),
                               MTXMODE_APPLY as libc::c_int as u8_0);
                let fresh69 = (*__gfxCtx).polyXlu.p;
                (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
                let mut _g_6: *mut Gfx = fresh69;
                (*_g_6).words.w0 =
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
                        (((0 as libc::c_int | 0x2 as libc::c_int |
                               0 as libc::c_int) ^ 0x1 as libc::c_int) as
                             u32_0 &
                             (((0x1 as libc::c_int) << 8 as libc::c_int) -
                                  1 as libc::c_int) as libc::c_uint) <<
                            0 as libc::c_int;
                (*_g_6).words.w1 =
                    Matrix_NewMtx((*globalCtx).state.gfxCtx,
                                  b"../z_demo_effect.c\x00" as *const u8 as
                                      *const libc::c_char as
                                      *mut libc::c_char, 3053 as libc::c_int)
                        as libc::c_uint;
                let fresh70 = (*__gfxCtx).polyXlu.p;
                (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
                let mut _g_7: *mut Gfx = fresh70;
                (*_g_7).words.w0 =
                    (0xdb as libc::c_int as u32_0 &
                         (((0x1 as libc::c_int) << 8 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        24 as libc::c_int |
                        (0x6 as libc::c_int as u32_0 &
                             (((0x1 as libc::c_int) << 8 as libc::c_int) -
                                  1 as libc::c_int) as libc::c_uint) <<
                            16 as libc::c_int |
                        ((8 as libc::c_int * 4 as libc::c_int) as u32_0 &
                             (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                  1 as libc::c_int) as libc::c_uint) <<
                            0 as libc::c_int;
                (*_g_7).words.w1 =
                    Gfx_TwoTexScroll((*globalCtx).state.gfxCtx,
                                     0 as libc::c_int,
                                     0 as libc::c_int as u32_0,
                                     0 as libc::c_int as u32_0,
                                     32 as libc::c_int, 16 as libc::c_int,
                                     1 as libc::c_int,
                                     0 as libc::c_int as u32_0,
                                     0 as libc::c_int as u32_0,
                                     16 as libc::c_int, 8 as libc::c_int) as
                        libc::c_uint;
                let fresh71 = (*__gfxCtx).polyXlu.p;
                (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
                let mut _g_8: *mut Gfx = fresh71;
                (*_g_8).words.w0 =
                    (0xfa as libc::c_int as u32_0 &
                         (((0x1 as libc::c_int) << 8 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        24 as libc::c_int |
                        (128 as libc::c_int as u32_0 &
                             (((0x1 as libc::c_int) << 8 as libc::c_int) -
                                  1 as libc::c_int) as libc::c_uint) <<
                            8 as libc::c_int |
                        (128 as libc::c_int as u32_0 &
                             (((0x1 as libc::c_int) << 8 as libc::c_int) -
                                  1 as libc::c_int) as libc::c_uint) <<
                            0 as libc::c_int;
                (*_g_8).words.w1 =
                    (255 as libc::c_int as u32_0 &
                         (((0x1 as libc::c_int) << 8 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        24 as libc::c_int |
                        (255 as libc::c_int as u32_0 &
                             (((0x1 as libc::c_int) << 8 as libc::c_int) -
                                  1 as libc::c_int) as libc::c_uint) <<
                            16 as libc::c_int |
                        (160 as libc::c_int as u32_0 &
                             (((0x1 as libc::c_int) << 8 as libc::c_int) -
                                  1 as libc::c_int) as libc::c_uint) <<
                            8 as libc::c_int |
                        ((*this).c2rust_unnamed.triforceSpot.triforceSpotOpacity
                             as u32_0 &
                             (((0x1 as libc::c_int) << 8 as libc::c_int) -
                                  1 as libc::c_int) as libc::c_uint) <<
                            0 as libc::c_int;
                let fresh72 = (*__gfxCtx).polyXlu.p;
                (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
                let mut _g_9: *mut Gfx = fresh72;
                (*_g_9).words.w0 =
                    (0xfb as libc::c_int as u32_0 &
                         (((0x1 as libc::c_int) << 8 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        24 as libc::c_int;
                (*_g_9).words.w1 =
                    (170 as libc::c_int as u32_0 &
                         (((0x1 as libc::c_int) << 8 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        24 as libc::c_int |
                        (140 as libc::c_int as u32_0 &
                             (((0x1 as libc::c_int) << 8 as libc::c_int) -
                                  1 as libc::c_int) as libc::c_uint) <<
                            16 as libc::c_int |
                        (0 as libc::c_int as u32_0 &
                             (((0x1 as libc::c_int) << 8 as libc::c_int) -
                                  1 as libc::c_int) as libc::c_uint) <<
                            8 as libc::c_int |
                        (255 as libc::c_int as u32_0 &
                             (((0x1 as libc::c_int) << 8 as libc::c_int) -
                                  1 as libc::c_int) as libc::c_uint) <<
                            0 as libc::c_int;
                let fresh73 = (*__gfxCtx).polyXlu.p;
                (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
                let mut _g_10: *mut Gfx = fresh73;
                (*_g_10).words.w0 =
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
                (*_g_10).words.w1 = gTriforceDL.as_mut_ptr() as libc::c_uint
            } else {
                func_8002EBCC(&mut (*this).actor, globalCtx,
                              0 as libc::c_int);
                func_80093D18((*globalCtx).state.gfxCtx);
                let fresh74 = (*__gfxCtx).polyOpa.p;
                (*__gfxCtx).polyOpa.p = (*__gfxCtx).polyOpa.p.offset(1);
                let mut _g_11: *mut Gfx = fresh74;
                (*_g_11).words.w0 =
                    (0xe2 as libc::c_int as u32_0 &
                         (((0x1 as libc::c_int) << 8 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        24 as libc::c_int |
                        ((32 as libc::c_int - 3 as libc::c_int -
                              29 as libc::c_int) as u32_0 &
                             (((0x1 as libc::c_int) << 8 as libc::c_int) -
                                  1 as libc::c_int) as libc::c_uint) <<
                            8 as libc::c_int |
                        ((29 as libc::c_int - 1 as libc::c_int) as u32_0 &
                             (((0x1 as libc::c_int) << 8 as libc::c_int) -
                                  1 as libc::c_int) as libc::c_uint) <<
                            0 as libc::c_int;
                (*_g_11).words.w1 =
                    ((0 as libc::c_int) << 30 as libc::c_int |
                         (3 as libc::c_int) << 26 as libc::c_int |
                         (0 as libc::c_int) << 22 as libc::c_int |
                         (2 as libc::c_int) << 18 as libc::c_int |
                         (0x8 as libc::c_int | 0x10 as libc::c_int |
                              0x20 as libc::c_int | 0x40 as libc::c_int |
                              0 as libc::c_int | 0 as libc::c_int |
                              0x2000 as libc::c_int |
                              (0 as libc::c_int) << 28 as libc::c_int |
                              (0 as libc::c_int) << 24 as libc::c_int |
                              (1 as libc::c_int) << 20 as libc::c_int |
                              (1 as libc::c_int) << 16 as libc::c_int)) as
                        libc::c_uint;
                Matrix_RotateY((*this).c2rust_unnamed.triforceSpot.rotation as
                                   libc::c_int as libc::c_float *
                                   (3.14159265358979323846f32 /
                                        0x8000 as libc::c_int as
                                            libc::c_float),
                               MTXMODE_APPLY as libc::c_int as u8_0);
                let fresh75 = (*__gfxCtx).polyOpa.p;
                (*__gfxCtx).polyOpa.p = (*__gfxCtx).polyOpa.p.offset(1);
                let mut _g_12: *mut Gfx = fresh75;
                (*_g_12).words.w0 =
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
                        (((0 as libc::c_int | 0x2 as libc::c_int |
                               0 as libc::c_int) ^ 0x1 as libc::c_int) as
                             u32_0 &
                             (((0x1 as libc::c_int) << 8 as libc::c_int) -
                                  1 as libc::c_int) as libc::c_uint) <<
                            0 as libc::c_int;
                (*_g_12).words.w1 =
                    Matrix_NewMtx((*globalCtx).state.gfxCtx,
                                  b"../z_demo_effect.c\x00" as *const u8 as
                                      *const libc::c_char as
                                      *mut libc::c_char, 3085 as libc::c_int)
                        as libc::c_uint;
                let fresh76 = (*__gfxCtx).polyOpa.p;
                (*__gfxCtx).polyOpa.p = (*__gfxCtx).polyOpa.p.offset(1);
                let mut _g_13: *mut Gfx = fresh76;
                (*_g_13).words.w0 =
                    (0xdb as libc::c_int as u32_0 &
                         (((0x1 as libc::c_int) << 8 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        24 as libc::c_int |
                        (0x6 as libc::c_int as u32_0 &
                             (((0x1 as libc::c_int) << 8 as libc::c_int) -
                                  1 as libc::c_int) as libc::c_uint) <<
                            16 as libc::c_int |
                        ((8 as libc::c_int * 4 as libc::c_int) as u32_0 &
                             (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                  1 as libc::c_int) as libc::c_uint) <<
                            0 as libc::c_int;
                (*_g_13).words.w1 =
                    Gfx_TwoTexScroll((*globalCtx).state.gfxCtx,
                                     0 as libc::c_int,
                                     0 as libc::c_int as u32_0,
                                     0 as libc::c_int as u32_0,
                                     32 as libc::c_int, 16 as libc::c_int,
                                     1 as libc::c_int,
                                     0 as libc::c_int as u32_0,
                                     0 as libc::c_int as u32_0,
                                     16 as libc::c_int, 8 as libc::c_int) as
                        libc::c_uint;
                let fresh77 = (*__gfxCtx).polyOpa.p;
                (*__gfxCtx).polyOpa.p = (*__gfxCtx).polyOpa.p.offset(1);
                let mut _g_14: *mut Gfx = fresh77;
                (*_g_14).words.w0 =
                    (0xfa as libc::c_int as u32_0 &
                         (((0x1 as libc::c_int) << 8 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        24 as libc::c_int |
                        (128 as libc::c_int as u32_0 &
                             (((0x1 as libc::c_int) << 8 as libc::c_int) -
                                  1 as libc::c_int) as libc::c_uint) <<
                            8 as libc::c_int |
                        (128 as libc::c_int as u32_0 &
                             (((0x1 as libc::c_int) << 8 as libc::c_int) -
                                  1 as libc::c_int) as libc::c_uint) <<
                            0 as libc::c_int;
                (*_g_14).words.w1 =
                    (255 as libc::c_int as u32_0 &
                         (((0x1 as libc::c_int) << 8 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        24 as libc::c_int |
                        (255 as libc::c_int as u32_0 &
                             (((0x1 as libc::c_int) << 8 as libc::c_int) -
                                  1 as libc::c_int) as libc::c_uint) <<
                            16 as libc::c_int |
                        (160 as libc::c_int as u32_0 &
                             (((0x1 as libc::c_int) << 8 as libc::c_int) -
                                  1 as libc::c_int) as libc::c_uint) <<
                            8 as libc::c_int |
                        (255 as libc::c_int as u32_0 &
                             (((0x1 as libc::c_int) << 8 as libc::c_int) -
                                  1 as libc::c_int) as libc::c_uint) <<
                            0 as libc::c_int;
                let fresh78 = (*__gfxCtx).polyOpa.p;
                (*__gfxCtx).polyOpa.p = (*__gfxCtx).polyOpa.p.offset(1);
                let mut _g_15: *mut Gfx = fresh78;
                (*_g_15).words.w0 =
                    (0xfb as libc::c_int as u32_0 &
                         (((0x1 as libc::c_int) << 8 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        24 as libc::c_int;
                (*_g_15).words.w1 =
                    (170 as libc::c_int as u32_0 &
                         (((0x1 as libc::c_int) << 8 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        24 as libc::c_int |
                        (140 as libc::c_int as u32_0 &
                             (((0x1 as libc::c_int) << 8 as libc::c_int) -
                                  1 as libc::c_int) as libc::c_uint) <<
                            16 as libc::c_int |
                        (0 as libc::c_int as u32_0 &
                             (((0x1 as libc::c_int) << 8 as libc::c_int) -
                                  1 as libc::c_int) as libc::c_uint) <<
                            8 as libc::c_int |
                        (255 as libc::c_int as u32_0 &
                             (((0x1 as libc::c_int) << 8 as libc::c_int) -
                                  1 as libc::c_int) as libc::c_uint) <<
                            0 as libc::c_int;
                let fresh79 = (*__gfxCtx).polyOpa.p;
                (*__gfxCtx).polyOpa.p = (*__gfxCtx).polyOpa.p.offset(1);
                let mut _g_16: *mut Gfx = fresh79;
                (*_g_16).words.w0 =
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
                (*_g_16).words.w1 = gTriforceDL.as_mut_ptr() as libc::c_uint
            }
        }
    }
    Graph_CloseDisps(dispRefs.as_mut_ptr(), (*globalCtx).state.gfxCtx,
                     b"../z_demo_effect.c\x00" as *const u8 as
                         *const libc::c_char, 3112 as libc::c_int);
}
/* *
 * Draw function for the Get Item Actors.
 * This is either Medals or Light Arrows based on the drawId.
 */
#[no_mangle]
pub unsafe extern "C" fn DemoEffect_DrawGetItem(mut thisx: *mut Actor,
                                                mut globalCtx:
                                                    *mut GlobalContext) {
    let mut this: *mut DemoEffect = thisx as *mut DemoEffect;
    if DemoEffect_CheckCsAction(this, globalCtx, 1 as libc::c_int) == 0 &&
           DemoEffect_CheckCsAction(this, globalCtx, 4 as libc::c_int) == 0 {
        if (*this).c2rust_unnamed.getItem.isLoaded == 0 {
            (*this).c2rust_unnamed.getItem.isLoaded =
                1 as libc::c_int as u8_0;
            return
        }
        func_8002EBCC(thisx, globalCtx, 0 as libc::c_int);
        func_8002ED80(thisx, globalCtx, 0 as libc::c_int);
        GetItem_Draw(globalCtx, (*this).c2rust_unnamed.getItem.drawId as s16);
    };
}
/* *
 * Callback for the SkelCurve system to draw the animated limbs.
 */
#[no_mangle]
pub unsafe extern "C" fn DemoEffect_DrawTimewarpLimbs(mut globalCtx:
                                                          *mut GlobalContext,
                                                      mut skelCuve:
                                                          *mut SkelAnimeCurve,
                                                      mut limbIndex: s32,
                                                      mut thisx:
                                                          *mut libc::c_void)
 -> s32 {
    let mut pad: s32 = 0;
    let mut this: *mut DemoEffect = thisx as *mut DemoEffect;
    let mut frames: u32_0 = (*globalCtx).gameplayFrames;
    let mut __gfxCtx: *mut GraphicsContext = 0 as *mut GraphicsContext;
    let mut dispRefs: [*mut Gfx; 4] = [0 as *mut Gfx; 4];
    __gfxCtx = (*globalCtx).state.gfxCtx;
    Graph_OpenDisps(dispRefs.as_mut_ptr(), (*globalCtx).state.gfxCtx,
                    b"../z_demo_effect.c\x00" as *const u8 as
                        *const libc::c_char, 3154 as libc::c_int);
    func_80093D84((*globalCtx).state.gfxCtx);
    let fresh80 = (*__gfxCtx).polyXlu.p;
    (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
    let mut _g: *mut Gfx = fresh80;
    (*_g).words.w0 =
        (0xfa as libc::c_int as u32_0 &
             (((0x1 as libc::c_int) << 8 as libc::c_int) - 1 as libc::c_int)
                 as libc::c_uint) << 24 as libc::c_int |
            (0 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 8 as libc::c_int |
            (128 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 0 as libc::c_int;
    (*_g).words.w1 =
        (170 as libc::c_int as u32_0 &
             (((0x1 as libc::c_int) << 8 as libc::c_int) - 1 as libc::c_int)
                 as libc::c_uint) << 24 as libc::c_int |
            (255 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 16 as libc::c_int
            |
            (255 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 8 as libc::c_int |
            (255 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 0 as libc::c_int;
    let fresh81 = (*__gfxCtx).polyXlu.p;
    (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
    let mut _g_0: *mut Gfx = fresh81;
    (*_g_0).words.w0 =
        (0xfb as libc::c_int as u32_0 &
             (((0x1 as libc::c_int) << 8 as libc::c_int) - 1 as libc::c_int)
                 as libc::c_uint) << 24 as libc::c_int;
    (*_g_0).words.w1 =
        ((*this).envXluColor[0 as libc::c_int as usize] as u32_0 &
             (((0x1 as libc::c_int) << 8 as libc::c_int) - 1 as libc::c_int)
                 as libc::c_uint) << 24 as libc::c_int |
            ((*this).envXluColor[1 as libc::c_int as usize] as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 16 as libc::c_int
            |
            ((*this).envXluColor[2 as libc::c_int as usize] as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 8 as libc::c_int |
            (255 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 0 as libc::c_int;
    let fresh82 = (*__gfxCtx).polyXlu.p;
    (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
    let mut _g_1: *mut Gfx = fresh82;
    (*_g_1).words.w0 =
        (0xdb as libc::c_int as u32_0 &
             (((0x1 as libc::c_int) << 8 as libc::c_int) - 1 as libc::c_int)
                 as libc::c_uint) << 24 as libc::c_int |
            (0x6 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 16 as libc::c_int
            |
            ((8 as libc::c_int * 4 as libc::c_int) as u32_0 &
                 (((0x1 as libc::c_int) << 16 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 0 as libc::c_int;
    (*_g_1).words.w1 =
        Gfx_TwoTexScroll((*globalCtx).state.gfxCtx, 0 as libc::c_int,
                         frames.wrapping_mul(6 as libc::c_int as
                                                 libc::c_uint).wrapping_rem(1024
                                                                                as
                                                                                libc::c_int
                                                                                as
                                                                                libc::c_uint),
                         (256 as libc::c_int as
                              libc::c_uint).wrapping_sub(frames.wrapping_mul(16
                                                                                 as
                                                                                 libc::c_int
                                                                                 as
                                                                                 libc::c_uint).wrapping_rem(256
                                                                                                                as
                                                                                                                libc::c_int
                                                                                                                as
                                                                                                                libc::c_uint)).wrapping_sub(1
                                                                                                                                                as
                                                                                                                                                libc::c_int
                                                                                                                                                as
                                                                                                                                                libc::c_uint),
                         256 as libc::c_int, 64 as libc::c_int,
                         1 as libc::c_int,
                         frames.wrapping_mul(4 as libc::c_int as
                                                 libc::c_uint).wrapping_rem(512
                                                                                as
                                                                                libc::c_int
                                                                                as
                                                                                libc::c_uint),
                         (128 as libc::c_int as
                              libc::c_uint).wrapping_sub(frames.wrapping_mul(12
                                                                                 as
                                                                                 libc::c_int
                                                                                 as
                                                                                 libc::c_uint).wrapping_rem(128
                                                                                                                as
                                                                                                                libc::c_int
                                                                                                                as
                                                                                                                libc::c_uint)).wrapping_sub(1
                                                                                                                                                as
                                                                                                                                                libc::c_int
                                                                                                                                                as
                                                                                                                                                libc::c_uint),
                         128 as libc::c_int, 32 as libc::c_int) as
            libc::c_uint;
    Graph_CloseDisps(dispRefs.as_mut_ptr(), (*globalCtx).state.gfxCtx,
                     b"../z_demo_effect.c\x00" as *const u8 as
                         *const libc::c_char, 3172 as libc::c_int);
    if limbIndex == 0 as libc::c_int {
        let mut transform: *mut LimbTransform =
            &mut *(*skelCuve).transforms.offset(0 as libc::c_int as isize) as
                *mut LimbTransform;
        (*transform).scale.y = 1024 as libc::c_int as s16;
        (*transform).scale.x = 1024 as libc::c_int as s16;
        (*transform).scale.z = (*transform).scale.x
    }
    return 1 as libc::c_int;
}
/* *
 * Draw function for the Time Warp Actors.
 */
#[no_mangle]
pub unsafe extern "C" fn DemoEffect_DrawTimeWarp(mut thisx: *mut Actor,
                                                 mut globalCtx:
                                                     *mut GlobalContext) {
    let mut this: *mut DemoEffect = thisx as *mut DemoEffect;
    let mut gfxCtx: *mut GraphicsContext = (*globalCtx).state.gfxCtx;
    let mut effectType: u8_0 =
        ((*this).actor.params as libc::c_int & 0xff as libc::c_int) as u8_0;
    if effectType as libc::c_int ==
           DEMO_EFFECT_TIMEWARP_TIMEBLOCK_LARGE as libc::c_int ||
           effectType as libc::c_int ==
               DEMO_EFFECT_TIMEWARP_TIMEBLOCK_SMALL as libc::c_int ||
           Flags_GetEnv(globalCtx, 1 as libc::c_int as s16) != 0 ||
           gSaveContext.sceneSetupIndex >= 4 as libc::c_int ||
           gSaveContext.entranceIndex == 0x324 as libc::c_int {
        let mut __gfxCtx: *mut GraphicsContext = 0 as *mut GraphicsContext;
        let mut dispRefs: [*mut Gfx; 4] = [0 as *mut Gfx; 4];
        __gfxCtx = gfxCtx;
        Graph_OpenDisps(dispRefs.as_mut_ptr(), gfxCtx,
                        b"../z_demo_effect.c\x00" as *const u8 as
                            *const libc::c_char, 3201 as libc::c_int);
        (*__gfxCtx).polyXlu.p =
            Gfx_CallSetupDL((*__gfxCtx).polyXlu.p,
                            25 as libc::c_int as u32_0);
        Matrix_Scale(2.0f32, 2.0f32, 2.0f32,
                     MTXMODE_APPLY as libc::c_int as u8_0);
        SkelCurve_Draw(thisx, globalCtx, &mut (*this).skelCurve,
                       Some(DemoEffect_DrawTimewarpLimbs as
                                unsafe extern "C" fn(_: *mut GlobalContext,
                                                     _: *mut SkelAnimeCurve,
                                                     _: s32,
                                                     _: *mut libc::c_void)
                                    -> s32), None, 1 as libc::c_int,
                       this as *mut libc::c_void);
        Graph_CloseDisps(dispRefs.as_mut_ptr(), gfxCtx,
                         b"../z_demo_effect.c\x00" as *const u8 as
                             *const libc::c_char, 3216 as libc::c_int);
    };
}
/* *
 * Faces/rotates the Actor towards the current cutscene action end point.
 */
#[no_mangle]
pub unsafe extern "C" fn DemoEffect_FaceToCsEndpoint(mut this:
                                                         *mut DemoEffect,
                                                     mut startPos: Vec3f,
                                                     mut endPos: Vec3f) {
    let mut pad: s32 = 0;
    let mut x: f32_0 = endPos.x - startPos.x;
    let mut z: f32_0 = endPos.z - startPos.z;
    let mut xzDistance: f32_0 = sqrtf(x * x + z * z);
    (*this).actor.shape.rot.y =
        (Math_FAtan2F(x, z) * (32768.0f32 / 3.14159265358979323846f32)) as
            s16;
    (*this).actor.shape.rot.x =
        (Math_FAtan2F(-(endPos.y - startPos.y), xzDistance) *
             (32768.0f32 / 3.14159265358979323846f32)) as s16;
}
/* *
 * Moves the Actor towards the current cutscene action end point.
 * Will only update the Actor's facing/rotation if the shouldUpdateFacing argument is true.
 * The speed is based on the current progress in the cutscene action.
 */
#[no_mangle]
pub unsafe extern "C" fn DemoEffect_MoveToCsEndpoint(mut this:
                                                         *mut DemoEffect,
                                                     mut globalCtx:
                                                         *mut GlobalContext,
                                                     mut csActionId: s32,
                                                     mut shouldUpdateFacing:
                                                         s32) {
    let mut startPos: Vec3f = Vec3f{x: 0., y: 0., z: 0.,};
    let mut endPos: Vec3f = Vec3f{x: 0., y: 0., z: 0.,};
    let mut speed: f32_0 = 0.;
    startPos.x =
        (*(*globalCtx).csCtx.npcActions[csActionId as usize]).startPos.x as
            f32_0;
    startPos.y =
        (*(*globalCtx).csCtx.npcActions[csActionId as usize]).startPos.y as
            f32_0;
    startPos.z =
        (*(*globalCtx).csCtx.npcActions[csActionId as usize]).startPos.z as
            f32_0;
    endPos.x =
        (*(*globalCtx).csCtx.npcActions[csActionId as usize]).endPos.x as
            f32_0;
    endPos.y =
        (*(*globalCtx).csCtx.npcActions[csActionId as usize]).endPos.y as
            f32_0;
    endPos.z =
        (*(*globalCtx).csCtx.npcActions[csActionId as usize]).endPos.z as
            f32_0;
    speed = DemoEffect_InterpolateCsFrames(globalCtx, csActionId);
    (*this).actor.world.pos.x = (endPos.x - startPos.x) * speed + startPos.x;
    (*this).actor.world.pos.y = (endPos.y - startPos.y) * speed + startPos.y;
    (*this).actor.world.pos.z = (endPos.z - startPos.z) * speed + startPos.z;
    if shouldUpdateFacing != 0 {
        DemoEffect_FaceToCsEndpoint(this, startPos, endPos);
    };
}
/* *
 * Moves a GetItem actor towards the current cutscene action's endpoint.
 */
#[no_mangle]
pub unsafe extern "C" fn DemoEffect_MoveGetItem(mut this: *mut DemoEffect,
                                                mut globalCtx:
                                                    *mut GlobalContext,
                                                mut csActionId: s32,
                                                mut speed: f32_0) {
    let mut endPos: Vec3f = Vec3f{x: 0., y: 0., z: 0.,};
    endPos.x =
        (*(*globalCtx).csCtx.npcActions[csActionId as usize]).endPos.x as
            f32_0;
    endPos.y =
        (*(*globalCtx).csCtx.npcActions[csActionId as usize]).endPos.y as
            f32_0;
    endPos.z =
        (*(*globalCtx).csCtx.npcActions[csActionId as usize]).endPos.z as
            f32_0;
    DemoEffect_MoveTowardTarget(endPos, this, speed);
}
/* *
 * Initializes the Actor's position to the current cutscene action's start point.
 */
#[no_mangle]
pub unsafe extern "C" fn DemoEffect_InitPositionFromCsAction(mut this:
                                                                 *mut DemoEffect,
                                                             mut globalCtx:
                                                                 *mut GlobalContext,
                                                             mut csActionIndex:
                                                                 s32) {
    let mut x: f32_0 =
        (*(*globalCtx).csCtx.npcActions[csActionIndex as usize]).startPos.x as
            f32_0;
    let mut y: f32_0 =
        (*(*globalCtx).csCtx.npcActions[csActionIndex as usize]).startPos.y as
            f32_0;
    let mut z: f32_0 =
        (*(*globalCtx).csCtx.npcActions[csActionIndex as usize]).startPos.z as
            f32_0;
    (*this).actor.world.pos.x = x;
    (*this).actor.world.pos.y = y;
    (*this).actor.world.pos.z = z;
}
