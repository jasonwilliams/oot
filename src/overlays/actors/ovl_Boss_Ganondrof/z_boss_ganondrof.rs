#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case,
         non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(const_raw_ptr_to_usize_cast, const_transmute, register_tool)]
extern "C" {
    #[no_mangle]
    fn fabsf(f: f32_0) -> f32_0;
    #[no_mangle]
    fn sqrtf(f: f32_0) -> f32_0;
    #[no_mangle]
    fn osSyncPrintf(fmt: *const libc::c_char, _: ...);
    #[no_mangle]
    fn EffectSsHahen_Spawn(globalCtx: *mut GlobalContext, pos: *mut Vec3f,
                           velocity: *mut Vec3f, accel: *mut Vec3f,
                           unused: s16, scale: s16, objId: s16, life: s16,
                           dList: *mut Gfx);
    #[no_mangle]
    fn EffectSsFhgFlash_SpawnLightBall(globalCtx: *mut GlobalContext,
                                       pos: *mut Vec3f, velocity: *mut Vec3f,
                                       accel: *mut Vec3f, scale: s16,
                                       param: u8_0);
    #[no_mangle]
    fn EffectSsFhgFlash_SpawnShock(globalCtx: *mut GlobalContext,
                                   actor: *mut Actor, pos: *mut Vec3f,
                                   scale: s16, param: u8_0);
    #[no_mangle]
    fn EffectSsKFire_Spawn(globalCtx: *mut GlobalContext, pos: *mut Vec3f,
                           velocity: *mut Vec3f, accel: *mut Vec3f,
                           scaleMax: s16, type_0: u8_0);
    #[no_mangle]
    fn ActorShape_Init(shape: *mut ActorShape, yOffset: f32_0,
                       shadowDraw: ActorShadowFunc, shadowScale: f32_0);
    #[no_mangle]
    fn Flags_SetSwitch(globalCtx: *mut GlobalContext, flag: s32);
    #[no_mangle]
    fn Flags_GetClear(globalCtx: *mut GlobalContext, flag: s32) -> s32;
    #[no_mangle]
    fn Flags_SetClear(globalCtx: *mut GlobalContext, flag: s32);
    #[no_mangle]
    fn Actor_Kill(actor: *mut Actor);
    #[no_mangle]
    fn Actor_SetScale(actor: *mut Actor, scale: f32_0);
    #[no_mangle]
    fn func_8002D7EC(actor: *mut Actor);
    #[no_mangle]
    fn Actor_MoveForward(actor: *mut Actor);
    #[no_mangle]
    fn func_8002D908(actor: *mut Actor);
    #[no_mangle]
    fn func_8002DF54(globalCtx: *mut GlobalContext, actor: *mut Actor,
                     arg2: u8_0) -> s32;
    #[no_mangle]
    fn Audio_PlayActorSound2(actor: *mut Actor, sfxId: u16_0);
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
    fn Enemy_StartFinishingBlow(globalCtx: *mut GlobalContext,
                                actor: *mut Actor);
    #[no_mangle]
    fn Rand_ZeroFloat(f: f32_0) -> f32_0;
    #[no_mangle]
    fn Rand_CenteredFloat(f: f32_0) -> f32_0;
    #[no_mangle]
    fn Collider_InitCylinder(globalCtx: *mut GlobalContext,
                             collider: *mut ColliderCylinder) -> s32;
    #[no_mangle]
    fn Collider_DestroyCylinder(globalCtx: *mut GlobalContext,
                                collider: *mut ColliderCylinder) -> s32;
    #[no_mangle]
    fn Collider_SetCylinder(globalCtx: *mut GlobalContext,
                            collider: *mut ColliderCylinder,
                            actor: *mut Actor, src: *mut ColliderCylinderInit)
     -> s32;
    #[no_mangle]
    fn CollisionCheck_SetAT(globalCtx: *mut GlobalContext,
                            colChkCtx: *mut CollisionCheckContext,
                            collider: *mut Collider) -> s32;
    #[no_mangle]
    fn CollisionCheck_SetAC(globalCtx: *mut GlobalContext,
                            colChkCtx: *mut CollisionCheckContext,
                            collider: *mut Collider) -> s32;
    #[no_mangle]
    fn CollisionCheck_SetOC(globalCtx: *mut GlobalContext,
                            colChkCtx: *mut CollisionCheckContext,
                            collider: *mut Collider) -> s32;
    #[no_mangle]
    fn CollisionCheck_GetSwordDamage(dmgFlags: s32) -> u8_0;
    #[no_mangle]
    fn func_80064520(globalCtx: *mut GlobalContext,
                     csCtx: *mut CutsceneContext);
    #[no_mangle]
    fn func_80064534(globalCtx: *mut GlobalContext,
                     csCtx: *mut CutsceneContext);
    #[no_mangle]
    fn Math_CosS(angle: s16) -> f32_0;
    #[no_mangle]
    fn Math_SinS(angle: s16) -> f32_0;
    #[no_mangle]
    fn Actor_ProcessInitChain(actor: *mut Actor,
                              initChain: *mut InitChainEntry);
    #[no_mangle]
    fn Math_ApproachF(pValue: *mut f32_0, target: f32_0, fraction: f32_0,
                      step: f32_0);
    #[no_mangle]
    fn Math_ApproachZeroF(pValue: *mut f32_0, fraction: f32_0, step: f32_0);
    #[no_mangle]
    fn Math_ApproachS(pValue: *mut s16, target: s16, scale: s16, step: s16);
    #[no_mangle]
    fn func_80078914(arg0: *mut Vec3f, sfxId: u16_0);
    #[no_mangle]
    fn Lights_PointNoGlowSetInfo(info: *mut LightInfo, x: s16, y: s16, z: s16,
                                 r: u8_0, g: u8_0, b: u8_0, radius: s16);
    #[no_mangle]
    fn LightContext_InsertLight(globalCtx: *mut GlobalContext,
                                lightCtx: *mut LightContext,
                                info: *mut LightInfo) -> *mut LightNode;
    #[no_mangle]
    fn LightContext_RemoveLight(globalCtx: *mut GlobalContext,
                                lightCtx: *mut LightContext,
                                node: *mut LightNode);
    #[no_mangle]
    fn Gfx_SetFog(gfx: *mut Gfx, r: s32, g: s32, b: s32, a: s32, near: s32,
                  far: s32) -> *mut Gfx;
    #[no_mangle]
    fn func_80093D18(gfxCtx: *mut GraphicsContext);
    #[no_mangle]
    fn SkelAnime_DrawOpa(globalCtx: *mut GlobalContext,
                         skeleton: *mut *mut libc::c_void,
                         jointTable: *mut Vec3s,
                         overrideLimbDraw: OverrideLimbDrawOpa,
                         postLimbDraw: PostLimbDrawOpa,
                         arg: *mut libc::c_void);
    #[no_mangle]
    fn Animation_GetLastFrame(animation: *mut libc::c_void) -> s16;
    #[no_mangle]
    fn SkelAnime_Init(globalCtx: *mut GlobalContext,
                      skelAnime: *mut SkelAnime,
                      skeletonHeaderSeg: *mut SkeletonHeader,
                      animation: *mut AnimationHeader, jointTable: *mut Vec3s,
                      morphTable: *mut Vec3s, limbCount: s32) -> s32;
    #[no_mangle]
    fn SkelAnime_Update(skelAnime: *mut SkelAnime) -> s32;
    #[no_mangle]
    fn Animation_Change(skelAnime: *mut SkelAnime,
                        animation: *mut AnimationHeader, playSpeed: f32_0,
                        startFrame: f32_0, endFrame: f32_0, mode: u8_0,
                        morphFrames: f32_0);
    #[no_mangle]
    fn Animation_PlayOnce(skelAnime: *mut SkelAnime,
                          animation: *mut AnimationHeader);
    #[no_mangle]
    fn Animation_MorphToPlayOnce(skelAnime: *mut SkelAnime,
                                 animation: *mut AnimationHeader,
                                 morphFrames: f32_0);
    #[no_mangle]
    fn Animation_PlayLoop(skelAnime: *mut SkelAnime,
                          animation: *mut AnimationHeader);
    #[no_mangle]
    fn Animation_MorphToLoop(skelAnime: *mut SkelAnime,
                             animation: *mut AnimationHeader,
                             morphFrames: f32_0);
    #[no_mangle]
    fn Animation_OnFrame(skelAnime: *mut SkelAnime, frame: f32_0) -> s32;
    #[no_mangle]
    fn SkelAnime_Free(skelAnime: *mut SkelAnime,
                      globalCtx: *mut GlobalContext);
    #[no_mangle]
    fn Gameplay_SetFog(globalCtx: *mut GlobalContext, gfx: *mut Gfx)
     -> *mut Gfx;
    #[no_mangle]
    fn Gameplay_CreateSubCamera(globalCtx: *mut GlobalContext) -> s16;
    #[no_mangle]
    fn Gameplay_ChangeCameraStatus(globalCtx: *mut GlobalContext, camId: s16,
                                   status: s16) -> s16;
    #[no_mangle]
    fn Gameplay_GetCamera(globalCtx: *mut GlobalContext, camId: s16)
     -> *mut Camera;
    #[no_mangle]
    fn Gameplay_CameraSetAtEye(globalCtx: *mut GlobalContext, camId: s16,
                               at: *mut Vec3f, eye: *mut Vec3f) -> s32;
    #[no_mangle]
    fn func_800C08AC(globalCtx: *mut GlobalContext, camId: s16, arg2: s16);
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
    fn Matrix_RotateX(x: f32_0, mode: u8_0);
    #[no_mangle]
    fn Matrix_RotateY(y: f32_0, mode: u8_0);
    #[no_mangle]
    fn Matrix_RotateZ(z: f32_0, mode: u8_0);
    #[no_mangle]
    fn Matrix_MultVec3f(src: *mut Vec3f, dest: *mut Vec3f);
    #[no_mangle]
    fn Audio_QueueSeqCmd(bgmID: u32_0);
    #[no_mangle]
    fn Math_FAtan2F(y: f32_0, x: f32_0) -> f32_0;
    #[no_mangle]
    fn Rand_ZeroOne() -> f32_0;
    #[no_mangle]
    fn Message_StartTextbox(globalCtx: *mut GlobalContext, textId: u16_0,
                            actor: *mut Actor);
    #[no_mangle]
    static mut gSegments: [u32_0; 16];
    #[no_mangle]
    static mut gPhantomGanonDeathBlowAnim: AnimationHeader;
    #[no_mangle]
    static mut gPhantomGanonLimpAnim: AnimationHeader;
    #[no_mangle]
    static mut gPhantomGanonMaskOnAnim: AnimationHeader;
    #[no_mangle]
    static mut gPhantomGanonScreamAnim: AnimationHeader;
    #[no_mangle]
    static mut gPhantomGanonRidePoseAnim: AnimationHeader;
    #[no_mangle]
    static mut gPhantomGanonLastPoseAnim: AnimationHeader;
    #[no_mangle]
    static mut gPhantomGanonHorseRearingAnim: AnimationHeader;
    #[no_mangle]
    static mut gPhantomGanonRideSpearStrikeAnim: AnimationHeader;
    #[no_mangle]
    static mut gPhantomGanonRideAnim: AnimationHeader;
    #[no_mangle]
    static mut gPhantomGanonEyeTex: [u64_0; 0];
    #[no_mangle]
    static mut gPhantomGanonSmileTex: [u64_0; 0];
    #[no_mangle]
    static mut gPhantomGanonMouthTex: [u64_0; 0];
    #[no_mangle]
    static mut gPhantomGanonFaceDL: [Gfx; 0];
    #[no_mangle]
    static mut gPhantomGanonLimbTex_00A000: [u64_0; 0];
    #[no_mangle]
    static mut gPhantomGanonLimbTex_00A200: [u64_0; 0];
    #[no_mangle]
    static mut gPhantomGanonLimbTex_00A400: [u64_0; 0];
    #[no_mangle]
    static mut gPhantomGanonLimbTex_00A600: [u64_0; 0];
    #[no_mangle]
    static mut gPhantomGanonLimbTex_00A800: [u64_0; 0];
    #[no_mangle]
    static mut gPhantomGanonLimbTex_00A880: [u64_0; 0];
    #[no_mangle]
    static mut gPhantomGanonLimbTex_00AA80: [u64_0; 0];
    #[no_mangle]
    static mut gPhantomGanonLimbTex_00AE80: [u64_0; 0];
    #[no_mangle]
    static mut gPhantomGanonLimbTex_00AF00: [u64_0; 0];
    #[no_mangle]
    static mut gPhantomGanonLimbTex_00AF80: [u64_0; 0];
    #[no_mangle]
    static mut gPhantomGanonLimbTex_00B380: [u64_0; 0];
    #[no_mangle]
    static mut gPhantomGanonLimbTex_00B780: [u64_0; 0];
    #[no_mangle]
    static mut gPhantomGanonLimbTex_00B980: [u64_0; 0];
    #[no_mangle]
    static mut gPhantomGanonLimbTex_00BA80: [u64_0; 0];
    #[no_mangle]
    static mut gPhantomGanonLimbTex_00BC80: [u64_0; 0];
    #[no_mangle]
    static mut gPhantomGanonLimbTex_00BD80: [u64_0; 0];
    #[no_mangle]
    static mut gPhantomGanonLimbTex_00BE80: [u64_0; 0];
    #[no_mangle]
    static mut gPhantomGanonLimbTex_00C080: [u64_0; 0];
    #[no_mangle]
    static mut gPhantomGanonLimbTex_00C180: [u64_0; 0];
    #[no_mangle]
    static mut gPhantomGanonLimbTex_00C200: [u64_0; 0];
    #[no_mangle]
    static mut gPhantomGanonLimbTex_00C400: [u64_0; 0];
    #[no_mangle]
    static mut gPhantomGanonLimbTex_00C480: [u64_0; 0];
    #[no_mangle]
    static mut gPhantomGanonSkel: SkeletonHeader;
    #[no_mangle]
    static mut gPhantomGanonRideSpearRaiseAnim: AnimationHeader;
    #[no_mangle]
    static mut gPhantomGanonRideSpearResetAnim: AnimationHeader;
    #[no_mangle]
    static mut gPhantomGanonThrowAnim: AnimationHeader;
    #[no_mangle]
    static mut gPhantomGanonThrowEndAnim: AnimationHeader;
    #[no_mangle]
    static mut gPhantomGanonGroundDamageAnim: AnimationHeader;
    #[no_mangle]
    static mut gPhantomGanonNeutralAnim: AnimationHeader;
    #[no_mangle]
    static mut gPhantomGanonBlockAnim: AnimationHeader;
    #[no_mangle]
    static mut gPhantomGanonAirDamageAnim: AnimationHeader;
    #[no_mangle]
    static mut gPhantomGanonReturn1Anim: AnimationHeader;
    #[no_mangle]
    static mut gPhantomGanonReturn2Anim: AnimationHeader;
    #[no_mangle]
    static mut gPhantomGanonStunnedAnim: AnimationHeader;
    #[no_mangle]
    static mut gPhantomGanonChargeStartAnim: AnimationHeader;
    #[no_mangle]
    static mut gPhantomGanonChargeAnim: AnimationHeader;
    #[no_mangle]
    static mut gPhantomGanonChargeWindupAnim: AnimationHeader;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SkinAvb {
    pub unk_0: u8_0,
    pub buf: [*mut Vtx; 2],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PSkinAwb {
    pub skeletonHeader: *mut SkeletonHeader,
    pub mtx: MtxF,
    pub avbCount: s32,
    pub avbTbl: *mut SkinAvb,
    pub skelAnime: SkelAnime,
}
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
pub struct ColliderCylinderInit {
    pub base: ColliderInit,
    pub info: ColliderInfoInit,
    pub dim: Cylinder16,
}
pub type C2RustUnnamed_15 = libc::c_uint;
pub const COLTYPE_TREE: C2RustUnnamed_15 = 13;
pub const COLTYPE_HARD: C2RustUnnamed_15 = 12;
pub const COLTYPE_WOOD: C2RustUnnamed_15 = 11;
pub const COLTYPE_NONE: C2RustUnnamed_15 = 10;
pub const COLTYPE_METAL: C2RustUnnamed_15 = 9;
pub const COLTYPE_HIT8: C2RustUnnamed_15 = 8;
pub const COLTYPE_HIT7: C2RustUnnamed_15 = 7;
pub const COLTYPE_HIT6: C2RustUnnamed_15 = 6;
pub const COLTYPE_HIT5: C2RustUnnamed_15 = 5;
pub const COLTYPE_HIT4: C2RustUnnamed_15 = 4;
pub const COLTYPE_HIT3: C2RustUnnamed_15 = 3;
pub const COLTYPE_HIT2: C2RustUnnamed_15 = 2;
pub const COLTYPE_HIT1: C2RustUnnamed_15 = 1;
pub const COLTYPE_HIT0: C2RustUnnamed_15 = 0;
pub type C2RustUnnamed_16 = libc::c_uint;
pub const COLSHAPE_INVALID: C2RustUnnamed_16 = 4;
pub const COLSHAPE_QUAD: C2RustUnnamed_16 = 3;
pub const COLSHAPE_TRIS: C2RustUnnamed_16 = 2;
pub const COLSHAPE_CYLINDER: C2RustUnnamed_16 = 1;
pub const COLSHAPE_JNTSPH: C2RustUnnamed_16 = 0;
pub type C2RustUnnamed_17 = libc::c_uint;
pub const ELEMTYPE_UNK7: C2RustUnnamed_17 = 7;
pub const ELEMTYPE_UNK6: C2RustUnnamed_17 = 6;
pub const ELEMTYPE_UNK5: C2RustUnnamed_17 = 5;
pub const ELEMTYPE_UNK4: C2RustUnnamed_17 = 4;
pub const ELEMTYPE_UNK3: C2RustUnnamed_17 = 3;
pub const ELEMTYPE_UNK2: C2RustUnnamed_17 = 2;
pub const ELEMTYPE_UNK1: C2RustUnnamed_17 = 1;
pub const ELEMTYPE_UNK0: C2RustUnnamed_17 = 0;
pub type C2RustUnnamed_18 = libc::c_uint;
pub const ACTORCAT_CHEST: C2RustUnnamed_18 = 11;
pub const ACTORCAT_DOOR: C2RustUnnamed_18 = 10;
pub const ACTORCAT_BOSS: C2RustUnnamed_18 = 9;
pub const ACTORCAT_MISC: C2RustUnnamed_18 = 8;
pub const ACTORCAT_ITEMACTION: C2RustUnnamed_18 = 7;
pub const ACTORCAT_PROP: C2RustUnnamed_18 = 6;
pub const ACTORCAT_ENEMY: C2RustUnnamed_18 = 5;
pub const ACTORCAT_NPC: C2RustUnnamed_18 = 4;
pub const ACTORCAT_EXPLOSIVE: C2RustUnnamed_18 = 3;
pub const ACTORCAT_PLAYER: C2RustUnnamed_18 = 2;
pub const ACTORCAT_BG: C2RustUnnamed_18 = 1;
pub const ACTORCAT_SWITCH: C2RustUnnamed_18 = 0;
pub type C2RustUnnamed_19 = libc::c_uint;
pub const ACTOR_ID_MAX: C2RustUnnamed_19 = 471;
pub const ACTOR_OBJ_WARP2BLOCK: C2RustUnnamed_19 = 470;
pub const ACTOR_BG_JYA_BLOCK: C2RustUnnamed_19 = 469;
pub const ACTOR_EN_MM2: C2RustUnnamed_19 = 468;
pub const ACTOR_EN_ZL4: C2RustUnnamed_19 = 467;
pub const ACTOR_OBJ_HAMISHI: C2RustUnnamed_19 = 466;
pub const ACTOR_OBJ_TIMEBLOCK: C2RustUnnamed_19 = 465;
pub const ACTOR_EN_GE3: C2RustUnnamed_19 = 464;
pub const ACTOR_OBJ_MAKEKINSUTA: C2RustUnnamed_19 = 463;
pub const ACTOR_EN_ZO: C2RustUnnamed_19 = 462;
pub const ACTOR_BG_MENKURI_NISEKABE: C2RustUnnamed_19 = 461;
pub const ACTOR_EN_EG: C2RustUnnamed_19 = 460;
pub const ACTOR_OCEFF_WIPE4: C2RustUnnamed_19 = 459;
pub const ACTOR_EN_KAKASI3: C2RustUnnamed_19 = 458;
pub const ACTOR_EN_KAKASI2: C2RustUnnamed_19 = 457;
pub const ACTOR_BG_ICE_SHUTTER: C2RustUnnamed_19 = 456;
pub const ACTOR_BG_ICE_TURARA: C2RustUnnamed_19 = 455;
pub const ACTOR_EN_COW: C2RustUnnamed_19 = 454;
pub const ACTOR_EN_MA3: C2RustUnnamed_19 = 453;
pub const ACTOR_BG_SPOT18_SHUTTER: C2RustUnnamed_19 = 452;
pub const ACTOR_BG_SPOT18_FUTA: C2RustUnnamed_19 = 451;
pub const ACTOR_BG_SPOT11_OASIS: C2RustUnnamed_19 = 450;
pub const ACTOR_DOOR_KILLER: C2RustUnnamed_19 = 449;
pub const ACTOR_EN_CROW: C2RustUnnamed_19 = 448;
pub const ACTOR_EN_PO_DESERT: C2RustUnnamed_19 = 447;
pub const ACTOR_EN_WALL_TUBO: C2RustUnnamed_19 = 446;
pub const ACTOR_BG_BOWL_WALL: C2RustUnnamed_19 = 445;
pub const ACTOR_EN_DAIKU_KAKARIKO: C2RustUnnamed_19 = 444;
pub const ACTOR_BG_MIZU_SHUTTER: C2RustUnnamed_19 = 443;
pub const ACTOR_BG_MIZU_BWALL: C2RustUnnamed_19 = 442;
pub const ACTOR_EN_GS: C2RustUnnamed_19 = 441;
pub const ACTOR_EN_GB: C2RustUnnamed_19 = 440;
pub const ACTOR_BG_GND_ICEBLOCK: C2RustUnnamed_19 = 439;
pub const ACTOR_BG_GND_NISEKABE: C2RustUnnamed_19 = 438;
pub const ACTOR_BG_GND_SOULMEIRO: C2RustUnnamed_19 = 437;
pub const ACTOR_BG_GND_DARKMEIRO: C2RustUnnamed_19 = 436;
pub const ACTOR_BG_GND_FIREMEIRO: C2RustUnnamed_19 = 435;
pub const ACTOR_DEMO_GEFF: C2RustUnnamed_19 = 434;
pub const ACTOR_DEMO_GJ: C2RustUnnamed_19 = 433;
pub const ACTOR_EN_SKB: C2RustUnnamed_19 = 432;
pub const ACTOR_EN_WF: C2RustUnnamed_19 = 431;
pub const ACTOR_EN_GO2: C2RustUnnamed_19 = 430;
pub const ACTOR_EN_MU: C2RustUnnamed_19 = 429;
pub const ACTOR_EN_TG: C2RustUnnamed_19 = 428;
pub const ACTOR_OBJ_MURE3: C2RustUnnamed_19 = 427;
pub const ACTOR_UNSET_1AA: C2RustUnnamed_19 = 426;
pub const ACTOR_BG_SPOT17_BAKUDANKABE: C2RustUnnamed_19 = 425;
pub const ACTOR_BG_SPOT08_BAKUDANKABE: C2RustUnnamed_19 = 424;
pub const ACTOR_DEMO_KEKKAI: C2RustUnnamed_19 = 423;
pub const ACTOR_EN_HS2: C2RustUnnamed_19 = 422;
pub const ACTOR_BG_BOM_GUARD: C2RustUnnamed_19 = 421;
pub const ACTOR_EN_GUEST: C2RustUnnamed_19 = 420;
pub const ACTOR_EN_DNT_NOMAL: C2RustUnnamed_19 = 419;
pub const ACTOR_EN_DNT_JIJI: C2RustUnnamed_19 = 418;
pub const ACTOR_EN_DNT_DEMO: C2RustUnnamed_19 = 417;
pub const ACTOR_OBJ_KIBAKO2: C2RustUnnamed_19 = 416;
pub const ACTOR_BG_SPOT11_BAKUDANKABE: C2RustUnnamed_19 = 415;
pub const ACTOR_OBJ_COMB: C2RustUnnamed_19 = 414;
pub const ACTOR_BG_SPOT01_OBJECTS2: C2RustUnnamed_19 = 413;
pub const ACTOR_EN_SI: C2RustUnnamed_19 = 412;
pub const ACTOR_EN_DOG: C2RustUnnamed_19 = 411;
pub const ACTOR_EN_NIW_GIRL: C2RustUnnamed_19 = 410;
pub const ACTOR_OCEFF_WIPE3: C2RustUnnamed_19 = 409;
pub const ACTOR_OCEFF_WIPE2: C2RustUnnamed_19 = 408;
pub const ACTOR_EN_GELDB: C2RustUnnamed_19 = 407;
pub const ACTOR_EN_IT: C2RustUnnamed_19 = 406;
pub const ACTOR_EN_SHOPNUTS: C2RustUnnamed_19 = 405;
pub const ACTOR_BG_SPOT00_BREAK: C2RustUnnamed_19 = 404;
pub const ACTOR_EN_NUTSBALL: C2RustUnnamed_19 = 403;
pub const ACTOR_EN_HINTNUTS: C2RustUnnamed_19 = 402;
pub const ACTOR_BG_SPOT12_SAKU: C2RustUnnamed_19 = 401;
pub const ACTOR_BG_SPOT12_GATE: C2RustUnnamed_19 = 400;
pub const ACTOR_BG_JYA_HAHENIRON: C2RustUnnamed_19 = 399;
pub const ACTOR_BG_JYA_1FLIFT: C2RustUnnamed_19 = 398;
pub const ACTOR_BG_SPOT05_SOKO: C2RustUnnamed_19 = 397;
pub const ACTOR_EN_WEIYER: C2RustUnnamed_19 = 396;
pub const ACTOR_OCEFF_STORM: C2RustUnnamed_19 = 395;
pub const ACTOR_OCEFF_WIPE: C2RustUnnamed_19 = 394;
pub const ACTOR_EN_STH: C2RustUnnamed_19 = 393;
pub const ACTOR_EN_SSH: C2RustUnnamed_19 = 392;
pub const ACTOR_OBJ_ROOMTIMER: C2RustUnnamed_19 = 391;
pub const ACTOR_EN_GE2: C2RustUnnamed_19 = 390;
pub const ACTOR_EN_WONDER_TALK2: C2RustUnnamed_19 = 389;
pub const ACTOR_EN_DY_EXTRA: C2RustUnnamed_19 = 388;
pub const ACTOR_SHOT_SUN: C2RustUnnamed_19 = 387;
pub const ACTOR_DEMO_EC: C2RustUnnamed_19 = 386;
pub const ACTOR_EN_TORCH: C2RustUnnamed_19 = 385;
pub const ACTOR_UNSET_180: C2RustUnnamed_19 = 384;
pub const ACTOR_END_TITLE: C2RustUnnamed_19 = 383;
pub const ACTOR_OCEFF_SPOT: C2RustUnnamed_19 = 382;
pub const ACTOR_OBJ_MAKEOSHIHIKI: C2RustUnnamed_19 = 381;
pub const ACTOR_EN_TAKARA_MAN: C2RustUnnamed_19 = 380;
pub const ACTOR_EN_KAKASI: C2RustUnnamed_19 = 379;
pub const ACTOR_BOSS_GANON2: C2RustUnnamed_19 = 378;
pub const ACTOR_EN_ZL3: C2RustUnnamed_19 = 377;
pub const ACTOR_EN_HEISHI4: C2RustUnnamed_19 = 376;
pub const ACTOR_BG_ZG: C2RustUnnamed_19 = 375;
pub const ACTOR_EFC_ERUPC: C2RustUnnamed_19 = 374;
pub const ACTOR_EN_PO_FIELD: C2RustUnnamed_19 = 373;
pub const ACTOR_DEMO_GT: C2RustUnnamed_19 = 372;
pub const ACTOR_ELF_MSG2: C2RustUnnamed_19 = 371;
pub const ACTOR_DOOR_GERUDO: C2RustUnnamed_19 = 370;
pub const ACTOR_EN_MAG: C2RustUnnamed_19 = 369;
pub const ACTOR_EN_OKARINA_EFFECT: C2RustUnnamed_19 = 368;
pub const ACTOR_EN_GANON_MANT: C2RustUnnamed_19 = 367;
pub const ACTOR_EN_HY: C2RustUnnamed_19 = 366;
pub const ACTOR_EN_MD: C2RustUnnamed_19 = 365;
pub const ACTOR_EN_CS: C2RustUnnamed_19 = 364;
pub const ACTOR_EN_JSJUTAN: C2RustUnnamed_19 = 363;
pub const ACTOR_EN_JS: C2RustUnnamed_19 = 362;
pub const ACTOR_BG_JYA_IRONOBJ: C2RustUnnamed_19 = 361;
pub const ACTOR_EN_EX_ITEM: C2RustUnnamed_19 = 360;
pub const ACTOR_EN_ANI: C2RustUnnamed_19 = 359;
pub const ACTOR_BG_SST_FLOOR: C2RustUnnamed_19 = 358;
pub const ACTOR_EN_WEATHER_TAG: C2RustUnnamed_19 = 357;
pub const ACTOR_EN_KZ: C2RustUnnamed_19 = 356;
pub const ACTOR_EN_KO: C2RustUnnamed_19 = 355;
pub const ACTOR_EN_MM: C2RustUnnamed_19 = 354;
pub const ACTOR_UNSET_161: C2RustUnnamed_19 = 353;
pub const ACTOR_EN_STREAM: C2RustUnnamed_19 = 352;
pub const ACTOR_EN_SIOFUKI: C2RustUnnamed_19 = 351;
pub const ACTOR_EN_GANON_ORGAN: C2RustUnnamed_19 = 350;
pub const ACTOR_UNSET_15D: C2RustUnnamed_19 = 349;
pub const ACTOR_BG_SPOT18_BASKET: C2RustUnnamed_19 = 348;
pub const ACTOR_BG_JYA_BOMBIWA: C2RustUnnamed_19 = 347;
pub const ACTOR_BG_JYA_AMISHUTTER: C2RustUnnamed_19 = 346;
pub const ACTOR_BG_JYA_BOMBCHUIWA: C2RustUnnamed_19 = 345;
pub const ACTOR_BG_JYA_BIGMIRROR: C2RustUnnamed_19 = 344;
pub const ACTOR_BG_JYA_LIFT: C2RustUnnamed_19 = 343;
pub const ACTOR_BG_JYA_MEGAMI: C2RustUnnamed_19 = 342;
pub const ACTOR_EN_CHANGER: C2RustUnnamed_19 = 341;
pub const ACTOR_UNSET_154: C2RustUnnamed_19 = 340;
pub const ACTOR_EN_FU: C2RustUnnamed_19 = 339;
pub const ACTOR_EN_GO: C2RustUnnamed_19 = 338;
pub const ACTOR_OBJ_MURE2: C2RustUnnamed_19 = 337;
pub const ACTOR_OBJ_LIGHTSWITCH: C2RustUnnamed_19 = 336;
pub const ACTOR_OBJ_HANA: C2RustUnnamed_19 = 335;
pub const ACTOR_EN_ISHI: C2RustUnnamed_19 = 334;
pub const ACTOR_EN_OWL: C2RustUnnamed_19 = 333;
pub const ACTOR_EN_BOM_BOWL_PIT: C2RustUnnamed_19 = 332;
pub const ACTOR_EN_BOM_BOWL_MAN: C2RustUnnamed_19 = 331;
pub const ACTOR_EN_MK: C2RustUnnamed_19 = 330;
pub const ACTOR_EN_DS: C2RustUnnamed_19 = 329;
pub const ACTOR_BG_GJYO_BRIDGE: C2RustUnnamed_19 = 328;
pub const ACTOR_EN_WONDER_TALK: C2RustUnnamed_19 = 327;
pub const ACTOR_EN_SA: C2RustUnnamed_19 = 326;
pub const ACTOR_BG_SPOT01_IDOSOKO: C2RustUnnamed_19 = 325;
pub const ACTOR_EN_ATTACK_NIW: C2RustUnnamed_19 = 324;
pub const ACTOR_EN_SYATEKI_NIW: C2RustUnnamed_19 = 323;
pub const ACTOR_EN_HEISHI3: C2RustUnnamed_19 = 322;
pub const ACTOR_EN_KANBAN: C2RustUnnamed_19 = 321;
pub const ACTOR_BG_INGATE: C2RustUnnamed_19 = 320;
pub const ACTOR_EN_HS: C2RustUnnamed_19 = 319;
pub const ACTOR_EN_MS: C2RustUnnamed_19 = 318;
pub const ACTOR_EN_GM: C2RustUnnamed_19 = 317;
pub const ACTOR_EN_NIW_LADY: C2RustUnnamed_19 = 316;
pub const ACTOR_EN_CLEAR_TAG: C2RustUnnamed_19 = 315;
pub const ACTOR_EN_SDA: C2RustUnnamed_19 = 314;
pub const ACTOR_OBJ_BLOCKSTOP: C2RustUnnamed_19 = 313;
pub const ACTOR_EN_GE1: C2RustUnnamed_19 = 312;
pub const ACTOR_ITEM_INBOX: C2RustUnnamed_19 = 311;
pub const ACTOR_EN_BLKOBJ: C2RustUnnamed_19 = 310;
pub const ACTOR_EN_NWC: C2RustUnnamed_19 = 309;
pub const ACTOR_UNSET_134: C2RustUnnamed_19 = 308;
pub const ACTOR_EN_DAIKU: C2RustUnnamed_19 = 307;
pub const ACTOR_EN_TORYO: C2RustUnnamed_19 = 306;
pub const ACTOR_EN_EX_RUPPY: C2RustUnnamed_19 = 305;
pub const ACTOR_EN_GOROIWA: C2RustUnnamed_19 = 304;
pub const ACTOR_EN_YABUSAME_MARK: C2RustUnnamed_19 = 303;
pub const ACTOR_EN_OKARINA_TAG: C2RustUnnamed_19 = 302;
pub const ACTOR_OBJ_HSBLOCK: C2RustUnnamed_19 = 301;
pub const ACTOR_OBJ_LIFT: C2RustUnnamed_19 = 300;
pub const ACTOR_OBJ_ELEVATOR: C2RustUnnamed_19 = 299;
pub const ACTOR_OBJ_SWITCH: C2RustUnnamed_19 = 298;
pub const ACTOR_UNSET_129: C2RustUnnamed_19 = 297;
pub const ACTOR_UNSET_128: C2RustUnnamed_19 = 296;
pub const ACTOR_OBJ_BOMBIWA: C2RustUnnamed_19 = 295;
pub const ACTOR_OBJ_BEAN: C2RustUnnamed_19 = 294;
pub const ACTOR_EN_KUSA: C2RustUnnamed_19 = 293;
pub const ACTOR_EN_DIVING_GAME: C2RustUnnamed_19 = 292;
pub const ACTOR_BG_RELAY_OBJECTS: C2RustUnnamed_19 = 291;
pub const ACTOR_EN_PO_RELAY: C2RustUnnamed_19 = 290;
pub const ACTOR_EN_FZ: C2RustUnnamed_19 = 289;
pub const ACTOR_BG_SPOT07_TAKI: C2RustUnnamed_19 = 288;
pub const ACTOR_BG_SPOT03_TAKI: C2RustUnnamed_19 = 287;
pub const ACTOR_OBJ_ICE_POLY: C2RustUnnamed_19 = 286;
pub const ACTOR_EN_TUBO_TRAP: C2RustUnnamed_19 = 285;
pub const ACTOR_EN_HONOTRAP: C2RustUnnamed_19 = 284;
pub const ACTOR_ELF_MSG: C2RustUnnamed_19 = 283;
pub const ACTOR_EN_DNS: C2RustUnnamed_19 = 282;
pub const ACTOR_DEMO_SHD: C2RustUnnamed_19 = 281;
pub const ACTOR_DEMO_EXT: C2RustUnnamed_19 = 280;
pub const ACTOR_EN_G_SWITCH: C2RustUnnamed_19 = 279;
pub const ACTOR_EN_SKJNEEDLE: C2RustUnnamed_19 = 278;
pub const ACTOR_EN_SKJ: C2RustUnnamed_19 = 277;
pub const ACTOR_DEMO_IK: C2RustUnnamed_19 = 276;
pub const ACTOR_EN_IK: C2RustUnnamed_19 = 275;
pub const ACTOR_EN_WONDER_ITEM: C2RustUnnamed_19 = 274;
pub const ACTOR_OBJ_TSUBO: C2RustUnnamed_19 = 273;
pub const ACTOR_OBJ_KIBAKO: C2RustUnnamed_19 = 272;
pub const ACTOR_ITEM_ETCETERA: C2RustUnnamed_19 = 271;
pub const ACTOR_UNSET_10E: C2RustUnnamed_19 = 270;
pub const ACTOR_UNSET_10D: C2RustUnnamed_19 = 269;
pub const ACTOR_ARROW_LIGHT: C2RustUnnamed_19 = 268;
pub const ACTOR_ARROW_ICE: C2RustUnnamed_19 = 267;
pub const ACTOR_ARROW_FIRE: C2RustUnnamed_19 = 266;
pub const ACTOR_UNSET_109: C2RustUnnamed_19 = 265;
pub const ACTOR_BG_UMAJUMP: C2RustUnnamed_19 = 264;
pub const ACTOR_BG_SPOT15_RRBOX: C2RustUnnamed_19 = 263;
pub const ACTOR_BG_GANON_OTYUKA: C2RustUnnamed_19 = 262;
pub const ACTOR_BG_PO_SYOKUDAI: C2RustUnnamed_19 = 261;
pub const ACTOR_BG_SPOT01_IDOMIZU: C2RustUnnamed_19 = 260;
pub const ACTOR_BG_SPOT01_IDOHASHIRA: C2RustUnnamed_19 = 259;
pub const ACTOR_BG_SPOT01_FUSYA: C2RustUnnamed_19 = 258;
pub const ACTOR_EFF_DUST: C2RustUnnamed_19 = 257;
pub const ACTOR_BG_GATE_SHUTTER: C2RustUnnamed_19 = 256;
pub const ACTOR_OBJ_OSHIHIKI: C2RustUnnamed_19 = 255;
pub const ACTOR_FISHING: C2RustUnnamed_19 = 254;
pub const ACTOR_BG_JYA_KANAAMI: C2RustUnnamed_19 = 253;
pub const ACTOR_BG_JYA_COBRA: C2RustUnnamed_19 = 252;
pub const ACTOR_UNSET_FB: C2RustUnnamed_19 = 251;
pub const ACTOR_BG_JYA_ZURERUKABE: C2RustUnnamed_19 = 250;
pub const ACTOR_BG_JYA_GOROIWA: C2RustUnnamed_19 = 249;
pub const ACTOR_BG_SPOT15_SAKU: C2RustUnnamed_19 = 248;
pub const ACTOR_BG_HAKA_GATE: C2RustUnnamed_19 = 247;
pub const ACTOR_EN_ANUBICE_TAG: C2RustUnnamed_19 = 246;
pub const ACTOR_DEMO_6K: C2RustUnnamed_19 = 245;
pub const ACTOR_MAGIC_DARK: C2RustUnnamed_19 = 244;
pub const ACTOR_UNSET_F3: C2RustUnnamed_19 = 243;
pub const ACTOR_UNSET_F2: C2RustUnnamed_19 = 242;
pub const ACTOR_ITEM_OCARINA: C2RustUnnamed_19 = 241;
pub const ACTOR_EN_ICE_HONO: C2RustUnnamed_19 = 240;
pub const ACTOR_BG_ICE_SHELTER: C2RustUnnamed_19 = 239;
pub const ACTOR_ITEM_SHIELD: C2RustUnnamed_19 = 238;
pub const ACTOR_EN_FR: C2RustUnnamed_19 = 237;
pub const ACTOR_EN_NY: C2RustUnnamed_19 = 236;
pub const ACTOR_UNSET_EB: C2RustUnnamed_19 = 235;
pub const ACTOR_UNSET_EA: C2RustUnnamed_19 = 234;
pub const ACTOR_BOSS_SST: C2RustUnnamed_19 = 233;
pub const ACTOR_BOSS_GANON: C2RustUnnamed_19 = 232;
pub const ACTOR_EN_MA1: C2RustUnnamed_19 = 231;
pub const ACTOR_BG_BDAN_SWITCH: C2RustUnnamed_19 = 230;
pub const ACTOR_BG_SPOT16_DOUGHNUT: C2RustUnnamed_19 = 229;
pub const ACTOR_BG_MORI_IDOMIZU: C2RustUnnamed_19 = 228;
pub const ACTOR_BG_MORI_HASHIRA4: C2RustUnnamed_19 = 227;
pub const ACTOR_BG_MORI_HASHIGO: C2RustUnnamed_19 = 226;
pub const ACTOR_EN_ANUBICE_FIRE: C2RustUnnamed_19 = 225;
pub const ACTOR_EN_ANUBICE: C2RustUnnamed_19 = 224;
pub const ACTOR_EN_BX: C2RustUnnamed_19 = 223;
pub const ACTOR_EN_BA: C2RustUnnamed_19 = 222;
pub const ACTOR_EN_RR: C2RustUnnamed_19 = 221;
pub const ACTOR_BOSS_TW: C2RustUnnamed_19 = 220;
pub const ACTOR_EN_HORSE_GAME_CHECK: C2RustUnnamed_19 = 219;
pub const ACTOR_EN_BOM_CHU: C2RustUnnamed_19 = 218;
pub const ACTOR_EN_MA2: C2RustUnnamed_19 = 217;
pub const ACTOR_UNSET_D8: C2RustUnnamed_19 = 216;
pub const ACTOR_BG_HAKA_WATER: C2RustUnnamed_19 = 215;
pub const ACTOR_BG_ICE_OBJECTS: C2RustUnnamed_19 = 214;
pub const ACTOR_BG_SPOT06_OBJECTS: C2RustUnnamed_19 = 213;
pub const ACTOR_BG_MIZU_UZU: C2RustUnnamed_19 = 212;
pub const ACTOR_OBJ_DEKUJR: C2RustUnnamed_19 = 211;
pub const ACTOR_EN_RU2: C2RustUnnamed_19 = 210;
pub const ACTOR_BG_SPOT08_ICEBLOCK: C2RustUnnamed_19 = 209;
pub const ACTOR_BG_BOMBWALL: C2RustUnnamed_19 = 208;
pub const ACTOR_BG_HIDAN_KOWARERUKABE: C2RustUnnamed_19 = 207;
pub const ACTOR_UNSET_CE: C2RustUnnamed_19 = 206;
pub const ACTOR_BG_SPOT16_BOMBSTONE: C2RustUnnamed_19 = 205;
pub const ACTOR_EN_TR: C2RustUnnamed_19 = 204;
pub const ACTOR_EN_IN: C2RustUnnamed_19 = 203;
pub const ACTOR_DEMO_GO: C2RustUnnamed_19 = 202;
pub const ACTOR_DEMO_SA: C2RustUnnamed_19 = 201;
pub const ACTOR_BG_BDAN_OBJECTS: C2RustUnnamed_19 = 200;
pub const ACTOR_EN_KAREBABA: C2RustUnnamed_19 = 199;
pub const ACTOR_EN_BIGOKUTA: C2RustUnnamed_19 = 198;
pub const ACTOR_EN_SB: C2RustUnnamed_19 = 197;
pub const ACTOR_BOSS_MO: C2RustUnnamed_19 = 196;
pub const ACTOR_EN_NB: C2RustUnnamed_19 = 195;
pub const ACTOR_EN_TANA: C2RustUnnamed_19 = 194;
pub const ACTOR_EN_SYATEKI_MAN: C2RustUnnamed_19 = 193;
pub const ACTOR_EN_SYATEKI_ITM: C2RustUnnamed_19 = 192;
pub const ACTOR_BG_SPOT17_FUNEN: C2RustUnnamed_19 = 191;
pub const ACTOR_BG_HAKA_ZOU: C2RustUnnamed_19 = 190;
pub const ACTOR_BG_HAKA_HUTA: C2RustUnnamed_19 = 189;
pub const ACTOR_BG_HAKA_TRAP: C2RustUnnamed_19 = 188;
pub const ACTOR_BG_HAKA_TUBO: C2RustUnnamed_19 = 187;
pub const ACTOR_BOSS_VA: C2RustUnnamed_19 = 186;
pub const ACTOR_BG_SPOT18_OBJ: C2RustUnnamed_19 = 185;
pub const ACTOR_BG_SPOT09_OBJ: C2RustUnnamed_19 = 184;
pub const ACTOR_MIR_RAY: C2RustUnnamed_19 = 183;
pub const ACTOR_EN_BROB: C2RustUnnamed_19 = 182;
pub const ACTOR_EN_FIRE_ROCK: C2RustUnnamed_19 = 181;
pub const ACTOR_EN_ENCOUNT2: C2RustUnnamed_19 = 180;
pub const ACTOR_EN_HEISHI2: C2RustUnnamed_19 = 179;
pub const ACTOR_UNSET_B2: C2RustUnnamed_19 = 178;
pub const ACTOR_BG_HAKA_SGAMI: C2RustUnnamed_19 = 177;
pub const ACTOR_BG_HAKA_SHIP: C2RustUnnamed_19 = 176;
pub const ACTOR_BG_HAKA_MEGANEBG: C2RustUnnamed_19 = 175;
pub const ACTOR_BG_HAKA_MEGANE: C2RustUnnamed_19 = 174;
pub const ACTOR_EN_VB_BALL: C2RustUnnamed_19 = 173;
pub const ACTOR_BG_VB_SIMA: C2RustUnnamed_19 = 172;
pub const ACTOR_EN_FW: C2RustUnnamed_19 = 171;
pub const ACTOR_DEMO_TRE_LGT: C2RustUnnamed_19 = 170;
pub const ACTOR_DEMO_IM: C2RustUnnamed_19 = 169;
pub const ACTOR_DEMO_DU: C2RustUnnamed_19 = 168;
pub const ACTOR_EN_ENCOUNT1: C2RustUnnamed_19 = 167;
pub const ACTOR_EN_RL: C2RustUnnamed_19 = 166;
pub const ACTOR_EN_DHA: C2RustUnnamed_19 = 165;
pub const ACTOR_EN_DH: C2RustUnnamed_19 = 164;
pub const ACTOR_EN_FD_FIRE: C2RustUnnamed_19 = 163;
pub const ACTOR_BOSS_FD2: C2RustUnnamed_19 = 162;
pub const ACTOR_EN_RU1: C2RustUnnamed_19 = 161;
pub const ACTOR_UNSET_A0: C2RustUnnamed_19 = 160;
pub const ACTOR_MAGIC_FIRE: C2RustUnnamed_19 = 159;
pub const ACTOR_MAGIC_WIND: C2RustUnnamed_19 = 158;
pub const ACTOR_BG_HAKA: C2RustUnnamed_19 = 157;
pub const ACTOR_BG_SPOT02_OBJECTS: C2RustUnnamed_19 = 156;
pub const ACTOR_DOOR_ANA: C2RustUnnamed_19 = 155;
pub const ACTOR_EN_HORSE_LINK_CHILD: C2RustUnnamed_19 = 154;
pub const ACTOR_EN_FD: C2RustUnnamed_19 = 153;
pub const ACTOR_EN_DU: C2RustUnnamed_19 = 152;
pub const ACTOR_OBJECT_KANKYO: C2RustUnnamed_19 = 151;
pub const ACTOR_BOSS_FD: C2RustUnnamed_19 = 150;
pub const ACTOR_EN_SW: C2RustUnnamed_19 = 149;
pub const ACTOR_OBJ_MURE: C2RustUnnamed_19 = 148;
pub const ACTOR_BG_PO_EVENT: C2RustUnnamed_19 = 147;
pub const ACTOR_BG_HEAVY_BLOCK: C2RustUnnamed_19 = 146;
pub const ACTOR_EN_PO_SISTERS: C2RustUnnamed_19 = 145;
pub const ACTOR_EN_RD: C2RustUnnamed_19 = 144;
pub const ACTOR_EN_HEISHI1: C2RustUnnamed_19 = 143;
pub const ACTOR_EN_FLOORMAS: C2RustUnnamed_19 = 142;
pub const ACTOR_BG_HIDAN_FWBIG: C2RustUnnamed_19 = 141;
pub const ACTOR_DEMO_KANKYO: C2RustUnnamed_19 = 140;
pub const ACTOR_DEMO_EFFECT: C2RustUnnamed_19 = 139;
pub const ACTOR_EN_VM: C2RustUnnamed_19 = 138;
pub const ACTOR_BG_MORI_RAKKATENJO: C2RustUnnamed_19 = 137;
pub const ACTOR_BG_MORI_KAITENKABE: C2RustUnnamed_19 = 136;
pub const ACTOR_BG_MORI_ELEVATOR: C2RustUnnamed_19 = 135;
pub const ACTOR_BG_MORI_BIGST: C2RustUnnamed_19 = 134;
pub const ACTOR_EN_TK: C2RustUnnamed_19 = 133;
pub const ACTOR_EN_TA: C2RustUnnamed_19 = 132;
pub const ACTOR_UNSET_83: C2RustUnnamed_19 = 131;
pub const ACTOR_EN_VASE: C2RustUnnamed_19 = 130;
pub const ACTOR_EN_AROW_TRAP: C2RustUnnamed_19 = 129;
pub const ACTOR_EN_TRAP: C2RustUnnamed_19 = 128;
pub const ACTOR_UNSET_7F: C2RustUnnamed_19 = 127;
pub const ACTOR_UNSET_7E: C2RustUnnamed_19 = 126;
pub const ACTOR_EN_PU_BOX: C2RustUnnamed_19 = 125;
pub const ACTOR_EN_LIGHTBOX: C2RustUnnamed_19 = 124;
pub const ACTOR_UNSET_7B: C2RustUnnamed_19 = 123;
pub const ACTOR_UNSET_7A: C2RustUnnamed_19 = 122;
pub const ACTOR_UNSET_79: C2RustUnnamed_19 = 121;
pub const ACTOR_UNSET_78: C2RustUnnamed_19 = 120;
pub const ACTOR_EN_WOOD02: C2RustUnnamed_19 = 119;
pub const ACTOR_UNSET_76: C2RustUnnamed_19 = 118;
pub const ACTOR_UNSET_75: C2RustUnnamed_19 = 117;
pub const ACTOR_UNSET_74: C2RustUnnamed_19 = 116;
pub const ACTOR_UNSET_73: C2RustUnnamed_19 = 115;
pub const ACTOR_EN_BIRD: C2RustUnnamed_19 = 114;
pub const ACTOR_BG_HIDAN_HAMSTEP: C2RustUnnamed_19 = 113;
pub const ACTOR_DOOR_TOKI: C2RustUnnamed_19 = 112;
pub const ACTOR_BG_HIDAN_KOUSI: C2RustUnnamed_19 = 111;
pub const ACTOR_BG_MJIN: C2RustUnnamed_19 = 110;
pub const ACTOR_EN_FHG_FIRE: C2RustUnnamed_19 = 109;
pub const ACTOR_BG_TOKI_SWD: C2RustUnnamed_19 = 108;
pub const ACTOR_EN_YUKABYUN: C2RustUnnamed_19 = 107;
pub const ACTOR_BG_TOKI_HIKARI: C2RustUnnamed_19 = 106;
pub const ACTOR_EN_BB: C2RustUnnamed_19 = 105;
pub const ACTOR_BG_MORI_HINERI: C2RustUnnamed_19 = 104;
pub const ACTOR_EN_FHG: C2RustUnnamed_19 = 103;
pub const ACTOR_ARMS_HOOK: C2RustUnnamed_19 = 102;
pub const ACTOR_BG_MIZU_WATER: C2RustUnnamed_19 = 101;
pub const ACTOR_BG_MIZU_MOVEBG: C2RustUnnamed_19 = 100;
pub const ACTOR_EN_VALI: C2RustUnnamed_19 = 99;
pub const ACTOR_BG_MENKURI_EYE: C2RustUnnamed_19 = 98;
pub const ACTOR_BG_MENKURI_KAITEN: C2RustUnnamed_19 = 97;
pub const ACTOR_EN_DEKUNUTS: C2RustUnnamed_19 = 96;
pub const ACTOR_ITEM_B_HEART: C2RustUnnamed_19 = 95;
pub const ACTOR_OBJ_SYOKUDAI: C2RustUnnamed_19 = 94;
pub const ACTOR_DOOR_WARP1: C2RustUnnamed_19 = 93;
pub const ACTOR_BG_DDAN_KD: C2RustUnnamed_19 = 92;
pub const ACTOR_EN_HORSE_ZELDA: C2RustUnnamed_19 = 91;
pub const ACTOR_EN_JJ: C2RustUnnamed_19 = 90;
pub const ACTOR_BG_BREAKWALL: C2RustUnnamed_19 = 89;
pub const ACTOR_BG_DDAN_JD: C2RustUnnamed_19 = 88;
pub const ACTOR_EN_M_THUNDER: C2RustUnnamed_19 = 87;
pub const ACTOR_EN_M_FIRE1: C2RustUnnamed_19 = 86;
pub const ACTOR_EN_DEKUBABA: C2RustUnnamed_19 = 85;
pub const ACTOR_EN_AM: C2RustUnnamed_19 = 84;
pub const ACTOR_UNSET_53: C2RustUnnamed_19 = 83;
pub const ACTOR_BOSS_GANONDROF: C2RustUnnamed_19 = 82;
pub const ACTOR_BG_YDAN_MARUTA: C2RustUnnamed_19 = 81;
pub const ACTOR_BG_YDAN_HASI: C2RustUnnamed_19 = 80;
pub const ACTOR_EN_OE2: C2RustUnnamed_19 = 79;
pub const ACTOR_BG_HIDAN_FSLIFT: C2RustUnnamed_19 = 78;
pub const ACTOR_EN_ZL2: C2RustUnnamed_19 = 77;
pub const ACTOR_EN_BOMBF: C2RustUnnamed_19 = 76;
pub const ACTOR_EN_MB: C2RustUnnamed_19 = 75;
pub const ACTOR_BG_SPOT00_HANEBASI: C2RustUnnamed_19 = 74;
pub const ACTOR_BG_HIDAN_CURTAIN: C2RustUnnamed_19 = 73;
pub const ACTOR_EN_XC: C2RustUnnamed_19 = 72;
pub const ACTOR_BG_HIDAN_SYOKU: C2RustUnnamed_19 = 71;
pub const ACTOR_BG_HIDAN_SIMA: C2RustUnnamed_19 = 70;
pub const ACTOR_BG_HIDAN_SEKIZOU: C2RustUnnamed_19 = 69;
pub const ACTOR_BG_HIDAN_RSEKIZOU: C2RustUnnamed_19 = 68;
pub const ACTOR_BG_HIDAN_ROCK: C2RustUnnamed_19 = 67;
pub const ACTOR_EN_HORSE_GANON: C2RustUnnamed_19 = 66;
pub const ACTOR_BG_HIDAN_HROCK: C2RustUnnamed_19 = 65;
pub const ACTOR_BG_HIDAN_DALM: C2RustUnnamed_19 = 64;
pub const ACTOR_BG_DODOAGO: C2RustUnnamed_19 = 63;
pub const ACTOR_BG_TREEMOUTH: C2RustUnnamed_19 = 62;
pub const ACTOR_EN_OSSAN: C2RustUnnamed_19 = 61;
pub const ACTOR_EN_HORSE_NORMAL: C2RustUnnamed_19 = 60;
pub const ACTOR_EN_RIVER_SOUND: C2RustUnnamed_19 = 59;
pub const ACTOR_EN_EIYER: C2RustUnnamed_19 = 58;
pub const ACTOR_EN_A_OBJ: C2RustUnnamed_19 = 57;
pub const ACTOR_EN_BW: C2RustUnnamed_19 = 56;
pub const ACTOR_EN_ST: C2RustUnnamed_19 = 55;
pub const ACTOR_UNSET_36: C2RustUnnamed_19 = 54;
pub const ACTOR_EN_TP: C2RustUnnamed_19 = 53;
pub const ACTOR_EN_BILI: C2RustUnnamed_19 = 52;
pub const ACTOR_EN_TORCH2: C2RustUnnamed_19 = 51;
pub const ACTOR_EN_BOOM: C2RustUnnamed_19 = 50;
pub const ACTOR_UNSET_31: C2RustUnnamed_19 = 49;
pub const ACTOR_EN_BDFIRE: C2RustUnnamed_19 = 48;
pub const ACTOR_EN_DODOJR: C2RustUnnamed_19 = 47;
pub const ACTOR_DOOR_SHUTTER: C2RustUnnamed_19 = 46;
pub const ACTOR_EN_BUBBLE: C2RustUnnamed_19 = 45;
pub const ACTOR_BG_PUSHBOX: C2RustUnnamed_19 = 44;
pub const ACTOR_EN_GOMA: C2RustUnnamed_19 = 43;
pub const ACTOR_EN_VIEWER: C2RustUnnamed_19 = 42;
pub const ACTOR_EN_ZL1: C2RustUnnamed_19 = 41;
pub const ACTOR_BOSS_GOMA: C2RustUnnamed_19 = 40;
pub const ACTOR_BOSS_DODONGO: C2RustUnnamed_19 = 39;
pub const ACTOR_EN_HATA: C2RustUnnamed_19 = 38;
pub const ACTOR_EN_ZF: C2RustUnnamed_19 = 37;
pub const ACTOR_EN_SCENE_CHANGE: C2RustUnnamed_19 = 36;
pub const ACTOR_EN_HOLL: C2RustUnnamed_19 = 35;
pub const ACTOR_UNSET_22: C2RustUnnamed_19 = 34;
pub const ACTOR_EN_FISH: C2RustUnnamed_19 = 33;
pub const ACTOR_EN_INSECT: C2RustUnnamed_19 = 32;
pub const ACTOR_UNSET_1F: C2RustUnnamed_19 = 31;
pub const ACTOR_EN_BUTTE: C2RustUnnamed_19 = 30;
pub const ACTOR_EN_PEEHAT: C2RustUnnamed_19 = 29;
pub const ACTOR_EN_REEBA: C2RustUnnamed_19 = 28;
pub const ACTOR_EN_TITE: C2RustUnnamed_19 = 27;
pub const ACTOR_UNSET_1A: C2RustUnnamed_19 = 26;
pub const ACTOR_EN_NIW: C2RustUnnamed_19 = 25;
pub const ACTOR_EN_ELF: C2RustUnnamed_19 = 24;
pub const ACTOR_UNSET_17: C2RustUnnamed_19 = 23;
pub const ACTOR_EN_ARROW: C2RustUnnamed_19 = 22;
pub const ACTOR_EN_ITEM00: C2RustUnnamed_19 = 21;
pub const ACTOR_EN_HORSE: C2RustUnnamed_19 = 20;
pub const ACTOR_EN_FIREFLY: C2RustUnnamed_19 = 19;
pub const ACTOR_EN_DODONGO: C2RustUnnamed_19 = 18;
pub const ACTOR_EN_WALLMAS: C2RustUnnamed_19 = 17;
pub const ACTOR_EN_BOM: C2RustUnnamed_19 = 16;
pub const ACTOR_BG_YDAN_SP: C2RustUnnamed_19 = 15;
pub const ACTOR_EN_OKUTA: C2RustUnnamed_19 = 14;
pub const ACTOR_EN_POH: C2RustUnnamed_19 = 13;
pub const ACTOR_BG_HIDAN_FIREWALL: C2RustUnnamed_19 = 12;
pub const ACTOR_BG_DY_YOSEIZO: C2RustUnnamed_19 = 11;
pub const ACTOR_EN_BOX: C2RustUnnamed_19 = 10;
pub const ACTOR_EN_DOOR: C2RustUnnamed_19 = 9;
pub const ACTOR_EN_LIGHT: C2RustUnnamed_19 = 8;
pub const ACTOR_EN_PART: C2RustUnnamed_19 = 7;
pub const ACTOR_UNSET_6: C2RustUnnamed_19 = 6;
pub const ACTOR_UNSET_5: C2RustUnnamed_19 = 5;
pub const ACTOR_EN_GIRLA: C2RustUnnamed_19 = 4;
pub const ACTOR_UNSET_3: C2RustUnnamed_19 = 3;
pub const ACTOR_EN_TEST: C2RustUnnamed_19 = 2;
pub const ACTOR_UNSET_1: C2RustUnnamed_19 = 1;
pub const ACTOR_PLAYER: C2RustUnnamed_19 = 0;
pub type C2RustUnnamed_20 = libc::c_uint;
pub const OBJECT_ID_MAX: C2RustUnnamed_20 = 402;
pub const OBJECT_ZL4: C2RustUnnamed_20 = 401;
pub const OBJECT_TIMEBLOCK: C2RustUnnamed_20 = 400;
pub const OBJECT_OUKE_HAKA: C2RustUnnamed_20 = 399;
pub const OBJECT_DOOR_KILLER: C2RustUnnamed_20 = 398;
pub const OBJECT_GI_SWORD_1: C2RustUnnamed_20 = 397;
pub const OBJECT_COB: C2RustUnnamed_20 = 396;
pub const OBJECT_COW: C2RustUnnamed_20 = 395;
pub const OBJECT_BWALL: C2RustUnnamed_20 = 394;
pub const OBJECT_PS: C2RustUnnamed_20 = 393;
pub const OBJECT_GS: C2RustUnnamed_20 = 392;
pub const OBJECT_HAKA_DOOR: C2RustUnnamed_20 = 391;
pub const OBJECT_GEFF: C2RustUnnamed_20 = 390;
pub const OBJECT_GJ: C2RustUnnamed_20 = 389;
pub const OBJECT_SKB: C2RustUnnamed_20 = 388;
pub const OBJECT_WF: C2RustUnnamed_20 = 387;
pub const OBJECT_MU: C2RustUnnamed_20 = 386;
pub const OBJECT_SPOT01_MATOYAB: C2RustUnnamed_20 = 385;
pub const OBJECT_SPOT01_MATOYA: C2RustUnnamed_20 = 384;
pub const OBJECT_GI_RUPY: C2RustUnnamed_20 = 383;
pub const OBJECT_GANON_ANIME3: C2RustUnnamed_20 = 382;
pub const OBJECT_GANON_ANIME2: C2RustUnnamed_20 = 381;
pub const OBJECT_GANON_ANIME1: C2RustUnnamed_20 = 380;
pub const OBJECT_GI_DEKUPOUCH: C2RustUnnamed_20 = 379;
pub const OBJECT_EFC_DOUGHNUT: C2RustUnnamed_20 = 378;
pub const OBJECT_DEMO_KEKKAI: C2RustUnnamed_20 = 377;
pub const OBJECT_BOWL: C2RustUnnamed_20 = 376;
pub const OBJECT_GI_SOUL: C2RustUnnamed_20 = 375;
pub const OBJECT_GI_GHOST: C2RustUnnamed_20 = 374;
pub const OBJECT_GI_BUTTERFLY: C2RustUnnamed_20 = 373;
pub const OBJECT_GI_INSECT: C2RustUnnamed_20 = 372;
pub const OBJECT_GI_FIRE: C2RustUnnamed_20 = 371;
pub const OBJECT_DNK: C2RustUnnamed_20 = 370;
pub const OBJECT_DNS: C2RustUnnamed_20 = 369;
pub const OBJECT_KIBAKO2: C2RustUnnamed_20 = 368;
pub const OBJECT_SPOT11_OBJ: C2RustUnnamed_20 = 367;
pub const OBJECT_UNSET_16E: C2RustUnnamed_20 = 366;
pub const OBJECT_JYA_DOOR: C2RustUnnamed_20 = 365;
pub const OBJECT_JYA_IRON: C2RustUnnamed_20 = 364;
pub const OBJECT_DOG: C2RustUnnamed_20 = 363;
pub const OBJECT_GR: C2RustUnnamed_20 = 362;
pub const OBJECT_GELDB: C2RustUnnamed_20 = 361;
pub const OBJECT_SHOPNUTS: C2RustUnnamed_20 = 360;
pub const OBJECT_GLA: C2RustUnnamed_20 = 359;
pub const OBJECT_SPOT00_BREAK: C2RustUnnamed_20 = 358;
pub const OBJECT_RS: C2RustUnnamed_20 = 357;
pub const OBJECT_HINTNUTS: C2RustUnnamed_20 = 356;
pub const OBJECT_BOMBIWA: C2RustUnnamed_20 = 355;
pub const OBJECT_SPOT12_OBJ: C2RustUnnamed_20 = 354;
pub const OBJECT_SPOT05_OBJECTS: C2RustUnnamed_20 = 353;
pub const OBJECT_BG: C2RustUnnamed_20 = 352;
pub const OBJECT_BIGOKUTA: C2RustUnnamed_20 = 351;
pub const OBJECT_SSH: C2RustUnnamed_20 = 350;
pub const OBJECT_GI_GODDESS: C2RustUnnamed_20 = 349;
pub const OBJECT_GI_SUTARU: C2RustUnnamed_20 = 348;
pub const OBJECT_FISH: C2RustUnnamed_20 = 347;
pub const OBJECT_EC: C2RustUnnamed_20 = 346;
pub const OBJECT_DS2: C2RustUnnamed_20 = 345;
pub const OBJECT_GI_M_ARROW: C2RustUnnamed_20 = 344;
pub const OBJECT_GI_HOVERBOOTS: C2RustUnnamed_20 = 343;
pub const OBJECT_ZG: C2RustUnnamed_20 = 342;
pub const OBJECT_TS: C2RustUnnamed_20 = 341;
pub const OBJECT_KA: C2RustUnnamed_20 = 340;
pub const OBJECT_GANON2: C2RustUnnamed_20 = 339;
pub const OBJECT_GI_GERUDOMASK: C2RustUnnamed_20 = 338;
pub const OBJECT_GI_ZORAMASK: C2RustUnnamed_20 = 337;
pub const OBJECT_GI_GOLONMASK: C2RustUnnamed_20 = 336;
pub const OBJECT_ZL2_ANIME2: C2RustUnnamed_20 = 335;
pub const OBJECT_ZL2_ANIME1: C2RustUnnamed_20 = 334;
pub const OBJECT_EFC_ERUPC: C2RustUnnamed_20 = 333;
pub const OBJECT_GT: C2RustUnnamed_20 = 332;
pub const OBJECT_DOOR_GERUDO: C2RustUnnamed_20 = 331;
pub const OBJECT_MAG: C2RustUnnamed_20 = 330;
pub const OBJECT_GI_FROG: C2RustUnnamed_20 = 329;
pub const OBJECT_GI_SOLDOUT: C2RustUnnamed_20 = 328;
pub const OBJECT_GI_BRACELET: C2RustUnnamed_20 = 327;
pub const OBJECT_GI_PRESCRIPTION: C2RustUnnamed_20 = 326;
pub const OBJECT_CS: C2RustUnnamed_20 = 325;
pub const OBJECT_JS: C2RustUnnamed_20 = 324;
pub const OBJECT_GI_BROKENSWORD: C2RustUnnamed_20 = 323;
pub const OBJECT_GI_TICKETSTONE: C2RustUnnamed_20 = 322;
pub const OBJECT_GI_MUSHROOM: C2RustUnnamed_20 = 321;
pub const OBJECT_GI_POWDER: C2RustUnnamed_20 = 320;
pub const OBJECT_GI_EYE_LOTION: C2RustUnnamed_20 = 319;
pub const OBJECT_OS: C2RustUnnamed_20 = 318;
pub const OBJECT_FA: C2RustUnnamed_20 = 317;
pub const OBJECT_MM: C2RustUnnamed_20 = 316;
pub const OBJECT_STREAM: C2RustUnnamed_20 = 315;
pub const OBJECT_SIOFUKI: C2RustUnnamed_20 = 314;
pub const OBJECT_GANON_OBJECTS: C2RustUnnamed_20 = 313;
pub const OBJECT_GI_TRUTH_MASK: C2RustUnnamed_20 = 312;
pub const OBJECT_GI_RABIT_MASK: C2RustUnnamed_20 = 311;
pub const OBJECT_GI_SKJ_MASK: C2RustUnnamed_20 = 310;
pub const OBJECT_GI_REDEAD_MASK: C2RustUnnamed_20 = 309;
pub const OBJECT_GI_KI_TAN_MASK: C2RustUnnamed_20 = 308;
pub const OBJECT_FU: C2RustUnnamed_20 = 307;
pub const OBJECT_MK: C2RustUnnamed_20 = 306;
pub const OBJECT_OWL: C2RustUnnamed_20 = 305;
pub const OBJECT_GJYO_OBJECTS: C2RustUnnamed_20 = 304;
pub const OBJECT_KANBAN: C2RustUnnamed_20 = 303;
pub const OBJECT_GI_COIN: C2RustUnnamed_20 = 302;
pub const OBJECT_GI_GLOVES: C2RustUnnamed_20 = 301;
pub const OBJECT_TSUBO: C2RustUnnamed_20 = 300;
pub const OBJECT_KUSA: C2RustUnnamed_20 = 299;
pub const OBJECT_LIGHTSWITCH: C2RustUnnamed_20 = 298;
pub const OBJECT_INGATE: C2RustUnnamed_20 = 297;
pub const OBJECT_HS: C2RustUnnamed_20 = 296;
pub const OBJECT_MS: C2RustUnnamed_20 = 295;
pub const OBJECT_GM: C2RustUnnamed_20 = 294;
pub const OBJECT_BLKOBJ: C2RustUnnamed_20 = 293;
pub const OBJECT_NWC: C2RustUnnamed_20 = 292;
pub const OBJECT_UNSET_123: C2RustUnnamed_20 = 291;
pub const OBJECT_DAIKU: C2RustUnnamed_20 = 290;
pub const OBJECT_TORYO: C2RustUnnamed_20 = 289;
pub const OBJECT_UNSET_120: C2RustUnnamed_20 = 288;
pub const OBJECT_GOROIWA: C2RustUnnamed_20 = 287;
pub const OBJECT_MAMENOKI: C2RustUnnamed_20 = 286;
pub const OBJECT_D_LIFT: C2RustUnnamed_20 = 285;
pub const OBJECT_D_HSBLOCK: C2RustUnnamed_20 = 284;
pub const OBJECT_D_ELEVATOR: C2RustUnnamed_20 = 283;
pub const OBJECT_GND_MAGIC: C2RustUnnamed_20 = 282;
pub const OBJECT_GI_SEED: C2RustUnnamed_20 = 281;
pub const OBJECT_GI_BOOTS_2: C2RustUnnamed_20 = 280;
pub const OBJECT_YABUSAME_POINT: C2RustUnnamed_20 = 279;
pub const OBJECT_GE1: C2RustUnnamed_20 = 278;
pub const OBJECT_BOB: C2RustUnnamed_20 = 277;
pub const OBJECT_FZ: C2RustUnnamed_20 = 276;
pub const OBJECT_SPOT07_OBJECT: C2RustUnnamed_20 = 275;
pub const OBJECT_SPOT03_OBJECT: C2RustUnnamed_20 = 274;
pub const OBJECT_BOJ: C2RustUnnamed_20 = 273;
pub const OBJECT_ANE: C2RustUnnamed_20 = 272;
pub const OBJECT_DS: C2RustUnnamed_20 = 271;
pub const OBJECT_GI_OCARINA_0: C2RustUnnamed_20 = 270;
pub const OBJECT_BBA: C2RustUnnamed_20 = 269;
pub const OBJECT_BJI: C2RustUnnamed_20 = 268;
pub const OBJECT_GI_BOTTLE_LETTER: C2RustUnnamed_20 = 267;
pub const OBJECT_SKJ: C2RustUnnamed_20 = 266;
pub const OBJECT_GI_NIWATORI: C2RustUnnamed_20 = 265;
pub const OBJECT_CNE: C2RustUnnamed_20 = 264;
pub const OBJECT_AHG: C2RustUnnamed_20 = 263;
pub const OBJECT_IK: C2RustUnnamed_20 = 262;
pub const OBJECT_AOB: C2RustUnnamed_20 = 261;
pub const OBJECT_MASTERZOORA: C2RustUnnamed_20 = 260;
pub const OBJECT_MASTERGOLON: C2RustUnnamed_20 = 259;
pub const OBJECT_MASTERKOKIRIHEAD: C2RustUnnamed_20 = 258;
pub const OBJECT_MASTERKOKIRI: C2RustUnnamed_20 = 257;
pub const OBJECT_UMAJUMP: C2RustUnnamed_20 = 256;
pub const OBJECT_KZ: C2RustUnnamed_20 = 255;
pub const OBJECT_ZO: C2RustUnnamed_20 = 254;
pub const OBJECT_KW1: C2RustUnnamed_20 = 253;
pub const OBJECT_KM1: C2RustUnnamed_20 = 252;
pub const OBJECT_MD: C2RustUnnamed_20 = 251;
pub const OBJECT_MD_UNUSED: C2RustUnnamed_20 = 250;
pub const OBJECT_SPOT01_OBJECTS: C2RustUnnamed_20 = 249;
pub const OBJECT_GI_LONGSWORD: C2RustUnnamed_20 = 248;
pub const OBJECT_GI_GRASS: C2RustUnnamed_20 = 247;
pub const OBJECT_GI_HAMMER: C2RustUnnamed_20 = 246;
pub const OBJECT_GI_SAW: C2RustUnnamed_20 = 245;
pub const OBJECT_GI_FISH: C2RustUnnamed_20 = 244;
pub const OBJECT_GI_BEAN: C2RustUnnamed_20 = 243;
pub const OBJECT_GI_CLOTHES: C2RustUnnamed_20 = 242;
pub const OBJECT_JYA_OBJ: C2RustUnnamed_20 = 241;
pub const OBJECT_SPOT15_OBJ: C2RustUnnamed_20 = 240;
pub const OBJECT_GI_LETTER: C2RustUnnamed_20 = 239;
pub const OBJECT_GI_SHIELD_3: C2RustUnnamed_20 = 238;
pub const OBJECT_DEMO_6K: C2RustUnnamed_20 = 237;
pub const OBJECT_ANI: C2RustUnnamed_20 = 236;
pub const OBJECT_GI_LIQUID: C2RustUnnamed_20 = 235;
pub const OBJECT_GI_GLASSES: C2RustUnnamed_20 = 234;
pub const OBJECT_GI_BOW: C2RustUnnamed_20 = 233;
pub const OBJECT_GI_BOOMERANG: C2RustUnnamed_20 = 232;
pub const OBJECT_GI_PACHINKO: C2RustUnnamed_20 = 231;
pub const OBJECT_FR: C2RustUnnamed_20 = 230;
pub const OBJECT_NY: C2RustUnnamed_20 = 229;
pub const OBJECT_UNSET_E4: C2RustUnnamed_20 = 228;
pub const OBJECT_NY_UNUSED: C2RustUnnamed_20 = 227;
pub const OBJECT_SST: C2RustUnnamed_20 = 226;
pub const OBJECT_GANON: C2RustUnnamed_20 = 225;
pub const OBJECT_MA1: C2RustUnnamed_20 = 224;
pub const OBJECT_GI_MILK: C2RustUnnamed_20 = 223;
pub const OBJECT_GI_OCARINA: C2RustUnnamed_20 = 222;
pub const OBJECT_GI_HOOKSHOT: C2RustUnnamed_20 = 221;
pub const OBJECT_GI_SHIELD_2: C2RustUnnamed_20 = 220;
pub const OBJECT_GI_SCALE: C2RustUnnamed_20 = 219;
pub const OBJECT_GI_EGG: C2RustUnnamed_20 = 218;
pub const OBJECT_GI_BOMB_2: C2RustUnnamed_20 = 217;
pub const OBJECT_GI_ARROW: C2RustUnnamed_20 = 216;
pub const OBJECT_GI_GERUDO: C2RustUnnamed_20 = 215;
pub const OBJECT_ANUBICE: C2RustUnnamed_20 = 214;
pub const OBJECT_BXA: C2RustUnnamed_20 = 213;
pub const OBJECT_RR: C2RustUnnamed_20 = 212;
pub const OBJECT_TW: C2RustUnnamed_20 = 211;
pub const OBJECT_HNI: C2RustUnnamed_20 = 210;
pub const OBJECT_GI_PURSE: C2RustUnnamed_20 = 209;
pub const OBJECT_MA2: C2RustUnnamed_20 = 208;
pub const OBJECT_OF1S: C2RustUnnamed_20 = 207;
pub const OBJECT_GI_BOMB_1: C2RustUnnamed_20 = 206;
pub const OBJECT_GI_MAGICPOT: C2RustUnnamed_20 = 205;
pub const OBJECT_DEKUJR: C2RustUnnamed_20 = 204;
pub const OBJECT_GI_SHIELD_1: C2RustUnnamed_20 = 203;
pub const OBJECT_RU2: C2RustUnnamed_20 = 202;
pub const OBJECT_OF1D_MAP: C2RustUnnamed_20 = 201;
pub const OBJECT_GI_MAP: C2RustUnnamed_20 = 200;
pub const OBJECT_GI_STICK: C2RustUnnamed_20 = 199;
pub const OBJECT_GI_BOTTLE: C2RustUnnamed_20 = 198;
pub const OBJECT_OS_ANIME: C2RustUnnamed_20 = 197;
pub const OBJECT_OE4S: C2RustUnnamed_20 = 196;
pub const OBJECT_OE1S: C2RustUnnamed_20 = 195;
pub const OBJECT_SPOT16_OBJ: C2RustUnnamed_20 = 194;
pub const OBJECT_TR: C2RustUnnamed_20 = 193;
pub const OBJECT_IN: C2RustUnnamed_20 = 192;
pub const OBJECT_GI_BOMBPOUCH: C2RustUnnamed_20 = 191;
pub const OBJECT_GI_ARROWCASE: C2RustUnnamed_20 = 190;
pub const OBJECT_GI_HEARTS: C2RustUnnamed_20 = 189;
pub const OBJECT_SA: C2RustUnnamed_20 = 188;
pub const OBJECT_GI_NUTS: C2RustUnnamed_20 = 187;
pub const OBJECT_GI_MEDAL: C2RustUnnamed_20 = 186;
pub const OBJECT_GI_BOSSKEY: C2RustUnnamed_20 = 185;
pub const OBJECT_GI_COMPASS: C2RustUnnamed_20 = 184;
pub const OBJECT_GI_HEART: C2RustUnnamed_20 = 183;
pub const OBJECT_GI_MELODY: C2RustUnnamed_20 = 182;
pub const OBJECT_SB: C2RustUnnamed_20 = 181;
pub const OBJECT_MO: C2RustUnnamed_20 = 180;
pub const OBJECT_NB: C2RustUnnamed_20 = 179;
pub const OBJECT_SHOP_DUNGEN: C2RustUnnamed_20 = 178;
pub const OBJECT_SPOT17_OBJ: C2RustUnnamed_20 = 177;
pub const OBJECT_BDOOR: C2RustUnnamed_20 = 176;
pub const OBJECT_SPOT18_OBJ: C2RustUnnamed_20 = 175;
pub const OBJECT_SPOT09_OBJ: C2RustUnnamed_20 = 174;
pub const OBJECT_GI_JEWEL: C2RustUnnamed_20 = 173;
pub const OBJECT_BROB: C2RustUnnamed_20 = 172;
pub const OBJECT_MIR_RAY: C2RustUnnamed_20 = 171;
pub const OBJECT_GI_KEY: C2RustUnnamed_20 = 170;
pub const OBJECT_DEMO_TRE_LGT: C2RustUnnamed_20 = 169;
pub const OBJECT_EFC_TW: C2RustUnnamed_20 = 168;
pub const OBJECT_RL: C2RustUnnamed_20 = 167;
pub const OBJECT_DH: C2RustUnnamed_20 = 166;
pub const OBJECT_FD2: C2RustUnnamed_20 = 165;
pub const OBJECT_SYOKUDAI: C2RustUnnamed_20 = 164;
pub const OBJECT_RU1: C2RustUnnamed_20 = 163;
pub const OBJECT_HAKA: C2RustUnnamed_20 = 162;
pub const OBJECT_SPOT02_OBJECTS: C2RustUnnamed_20 = 161;
pub const OBJECT_HORSE_LINK_CHILD: C2RustUnnamed_20 = 160;
pub const OBJECT_MEDAL: C2RustUnnamed_20 = 159;
pub const OBJECT_FW: C2RustUnnamed_20 = 158;
pub const OBJECT_DU: C2RustUnnamed_20 = 157;
pub const OBJECT_FD: C2RustUnnamed_20 = 156;
pub const OBJECT_GNDD: C2RustUnnamed_20 = 155;
pub const OBJECT_HEAVY_OBJECT: C2RustUnnamed_20 = 154;
pub const OBJECT_PO_SISTERS: C2RustUnnamed_20 = 153;
pub const OBJECT_RD: C2RustUnnamed_20 = 152;
pub const OBJECT_SD: C2RustUnnamed_20 = 151;
pub const OBJECT_BDAN_OBJECTS: C2RustUnnamed_20 = 150;
pub const OBJECT_TRIFORCE_SPOT: C2RustUnnamed_20 = 149;
pub const OBJECT_LIGHT_RING: C2RustUnnamed_20 = 148;
pub const OBJECT_GOD_LGT: C2RustUnnamed_20 = 147;
pub const OBJECT_EFC_STAR_FIELD: C2RustUnnamed_20 = 146;
pub const OBJECT_EFC_LGT_SHOWER: C2RustUnnamed_20 = 145;
pub const OBJECT_EFC_FLASH: C2RustUnnamed_20 = 144;
pub const OBJECT_EFC_FIRE_BALL: C2RustUnnamed_20 = 143;
pub const OBJECT_EFC_CRYSTAL_LIGHT: C2RustUnnamed_20 = 142;
pub const OBJECT_HAKACH_OBJECTS: C2RustUnnamed_20 = 141;
pub const OBJECT_BV: C2RustUnnamed_20 = 140;
pub const OBJECT_VM: C2RustUnnamed_20 = 139;
pub const OBJECT_XC: C2RustUnnamed_20 = 138;
pub const OBJECT_TK: C2RustUnnamed_20 = 137;
pub const OBJECT_TA: C2RustUnnamed_20 = 136;
pub const OBJECT_IM: C2RustUnnamed_20 = 135;
pub const OBJECT_VASE: C2RustUnnamed_20 = 134;
pub const OBJECT_TRAP: C2RustUnnamed_20 = 133;
pub const OBJECT_UNSET_84: C2RustUnnamed_20 = 132;
pub const OBJECT_UNSET_83: C2RustUnnamed_20 = 131;
pub const OBJECT_PU_BOX: C2RustUnnamed_20 = 130;
pub const OBJECT_LIGHTBOX: C2RustUnnamed_20 = 129;
pub const OBJECT_UNSET_80: C2RustUnnamed_20 = 128;
pub const OBJECT_UNSET_7F: C2RustUnnamed_20 = 127;
pub const OBJECT_UNSET_7E: C2RustUnnamed_20 = 126;
pub const OBJECT_UNSET_7D: C2RustUnnamed_20 = 125;
pub const OBJECT_WOOD02: C2RustUnnamed_20 = 124;
pub const OBJECT_UNSET_7B: C2RustUnnamed_20 = 123;
pub const OBJECT_UNSET_7A: C2RustUnnamed_20 = 122;
pub const OBJECT_UNSET_79: C2RustUnnamed_20 = 121;
pub const OBJECT_UNSET_78: C2RustUnnamed_20 = 120;
pub const OBJECT_BIRD: C2RustUnnamed_20 = 119;
pub const OBJECT_HATA: C2RustUnnamed_20 = 118;
pub const OBJECT_WARP2: C2RustUnnamed_20 = 117;
pub const OBJECT_SPOT08_OBJ: C2RustUnnamed_20 = 116;
pub const OBJECT_MORI_TEX: C2RustUnnamed_20 = 115;
pub const OBJECT_MORI_OBJECTS: C2RustUnnamed_20 = 114;
pub const OBJECT_MORI_HINERI2A: C2RustUnnamed_20 = 113;
pub const OBJECT_MORI_HINERI2: C2RustUnnamed_20 = 112;
pub const OBJECT_MORI_HINERI1A: C2RustUnnamed_20 = 111;
pub const OBJECT_PO_COMPOSER: C2RustUnnamed_20 = 110;
pub const OBJECT_PO_FIELD: C2RustUnnamed_20 = 109;
pub const OBJECT_RELAY_OBJECTS: C2RustUnnamed_20 = 108;
pub const OBJECT_ICE_OBJECTS: C2RustUnnamed_20 = 107;
pub const OBJECT_SPOT06_OBJECTS: C2RustUnnamed_20 = 106;
pub const OBJECT_HAKA_OBJECTS: C2RustUnnamed_20 = 105;
pub const OBJECT_MJIN_OKA: C2RustUnnamed_20 = 104;
pub const OBJECT_MJIN_WIND: C2RustUnnamed_20 = 103;
pub const OBJECT_MJIN_SOUL: C2RustUnnamed_20 = 102;
pub const OBJECT_MJIN_ICE: C2RustUnnamed_20 = 101;
pub const OBJECT_MJIN_FLAME: C2RustUnnamed_20 = 100;
pub const OBJECT_MJIN_DARK: C2RustUnnamed_20 = 99;
pub const OBJECT_MJIN_FLASH: C2RustUnnamed_20 = 98;
pub const OBJECT_MJIN: C2RustUnnamed_20 = 97;
pub const OBJECT_ZL2: C2RustUnnamed_20 = 96;
pub const OBJECT_YUKABYUN: C2RustUnnamed_20 = 95;
pub const OBJECT_TOKI_OBJECTS: C2RustUnnamed_20 = 94;
pub const OBJECT_BB: C2RustUnnamed_20 = 93;
pub const OBJECT_MORI_HINERI1: C2RustUnnamed_20 = 92;
pub const OBJECT_OSSAN: C2RustUnnamed_20 = 91;
pub const OBJECT_FHG: C2RustUnnamed_20 = 90;
pub const OBJECT_MIZU_OBJECTS: C2RustUnnamed_20 = 89;
pub const OBJECT_OA11: C2RustUnnamed_20 = 88;
pub const OBJECT_OA10: C2RustUnnamed_20 = 87;
pub const OBJECT_VALI: C2RustUnnamed_20 = 86;
pub const OBJECT_OE12: C2RustUnnamed_20 = 85;
pub const OBJECT_OE11: C2RustUnnamed_20 = 84;
pub const OBJECT_OE10: C2RustUnnamed_20 = 83;
pub const OBJECT_OE9: C2RustUnnamed_20 = 82;
pub const OBJECT_OE8: C2RustUnnamed_20 = 81;
pub const OBJECT_OE7: C2RustUnnamed_20 = 80;
pub const OBJECT_OE6: C2RustUnnamed_20 = 79;
pub const OBJECT_OE5: C2RustUnnamed_20 = 78;
pub const OBJECT_MENKURI_OBJECTS: C2RustUnnamed_20 = 77;
pub const OBJECT_OE4: C2RustUnnamed_20 = 76;
pub const OBJECT_OE3: C2RustUnnamed_20 = 75;
pub const OBJECT_DEKUNUTS: C2RustUnnamed_20 = 74;
pub const OBJECT_B_HEART: C2RustUnnamed_20 = 73;
pub const OBJECT_WARP1: C2RustUnnamed_20 = 72;
pub const OBJECT_OPENING_DEMO1: C2RustUnnamed_20 = 71;
pub const OBJECT_HORSE_ZELDA: C2RustUnnamed_20 = 70;
pub const OBJECT_OB4: C2RustUnnamed_20 = 69;
pub const OBJECT_OB3: C2RustUnnamed_20 = 68;
pub const OBJECT_OB2: C2RustUnnamed_20 = 67;
pub const OBJECT_OA9: C2RustUnnamed_20 = 66;
pub const OBJECT_OA8: C2RustUnnamed_20 = 65;
pub const OBJECT_JJ: C2RustUnnamed_20 = 64;
pub const OBJECT_OA7: C2RustUnnamed_20 = 63;
pub const OBJECT_OA6: C2RustUnnamed_20 = 62;
pub const OBJECT_OA5: C2RustUnnamed_20 = 61;
pub const OBJECT_OA4: C2RustUnnamed_20 = 60;
pub const OBJECT_OA3: C2RustUnnamed_20 = 59;
pub const OBJECT_UNSET_3A: C2RustUnnamed_20 = 58;
pub const OBJECT_DEKUBABA: C2RustUnnamed_20 = 57;
pub const OBJECT_AM: C2RustUnnamed_20 = 56;
pub const OBJECT_GND: C2RustUnnamed_20 = 55;
pub const OBJECT_YDAN_OBJECTS: C2RustUnnamed_20 = 54;
pub const OBJECT_OE2: C2RustUnnamed_20 = 53;
pub const OBJECT_OE_ANIME: C2RustUnnamed_20 = 52;
pub const OBJECT_OE1: C2RustUnnamed_20 = 51;
pub const OBJECT_SK2: C2RustUnnamed_20 = 50;
pub const OBJECT_BOMBF: C2RustUnnamed_20 = 49;
pub const OBJECT_MB: C2RustUnnamed_20 = 48;
pub const OBJECT_SPOT00_OBJECTS: C2RustUnnamed_20 = 47;
pub const OBJECT_OA2: C2RustUnnamed_20 = 46;
pub const OBJECT_HORSE_GANON: C2RustUnnamed_20 = 45;
pub const OBJECT_HIDAN_OBJECTS: C2RustUnnamed_20 = 44;
pub const OBJECT_DDAN_OBJECTS: C2RustUnnamed_20 = 43;
pub const OBJECT_SPOT04_OBJECTS: C2RustUnnamed_20 = 42;
pub const OBJECT_O_ANIME: C2RustUnnamed_20 = 41;
pub const OBJECT_OB1: C2RustUnnamed_20 = 40;
pub const OBJECT_HORSE_NORMAL: C2RustUnnamed_20 = 39;
pub const OBJECT_EI: C2RustUnnamed_20 = 38;
pub const OBJECT_BW: C2RustUnnamed_20 = 37;
pub const OBJECT_ST: C2RustUnnamed_20 = 36;
pub const OBJECT_OA1: C2RustUnnamed_20 = 35;
pub const OBJECT_TP: C2RustUnnamed_20 = 34;
pub const OBJECT_BL: C2RustUnnamed_20 = 33;
pub const OBJECT_TORCH2: C2RustUnnamed_20 = 32;
pub const OBJECT_DODOJR: C2RustUnnamed_20 = 31;
pub const OBJECT_GOL: C2RustUnnamed_20 = 30;
pub const OBJECT_ZL1: C2RustUnnamed_20 = 29;
pub const OBJECT_GOMA: C2RustUnnamed_20 = 28;
pub const OBJECT_ZF: C2RustUnnamed_20 = 27;
pub const OBJECT_HORSE: C2RustUnnamed_20 = 26;
pub const OBJECT_KINGDODONGO: C2RustUnnamed_20 = 25;
pub const OBJECT_PEEHAT: C2RustUnnamed_20 = 24;
pub const OBJECT_REEBA: C2RustUnnamed_20 = 23;
pub const OBJECT_TITE: C2RustUnnamed_20 = 22;
pub const OBJECT_LINK_CHILD: C2RustUnnamed_20 = 21;
pub const OBJECT_LINK_BOY: C2RustUnnamed_20 = 20;
pub const OBJECT_NIW: C2RustUnnamed_20 = 19;
pub const OBJECT_BUBBLE: C2RustUnnamed_20 = 18;
pub const OBJECT_UNSET_11: C2RustUnnamed_20 = 17;
pub const OBJECT_UNSET_10: C2RustUnnamed_20 = 16;
pub const OBJECT_FIRE: C2RustUnnamed_20 = 15;
pub const OBJECT_BOX: C2RustUnnamed_20 = 14;
pub const OBJECT_FIREFLY: C2RustUnnamed_20 = 13;
pub const OBJECT_DODONGO: C2RustUnnamed_20 = 12;
pub const OBJECT_WALLMASTER: C2RustUnnamed_20 = 11;
pub const OBJECT_DY_OBJ: C2RustUnnamed_20 = 10;
pub const OBJECT_POH: C2RustUnnamed_20 = 9;
pub const OBJECT_CROW: C2RustUnnamed_20 = 8;
pub const OBJECT_OKUTA: C2RustUnnamed_20 = 7;
pub const OBJECT_HUMAN: C2RustUnnamed_20 = 6;
pub const OBJECT_UNSET_5: C2RustUnnamed_20 = 5;
pub const OBJECT_UNSET_4: C2RustUnnamed_20 = 4;
pub const OBJECT_GAMEPLAY_DANGEON_KEEP: C2RustUnnamed_20 = 3;
pub const OBJECT_GAMEPLAY_FIELD_KEEP: C2RustUnnamed_20 = 2;
pub const OBJECT_GAMEPLAY_KEEP: C2RustUnnamed_20 = 1;
pub const OBJECT_INVALID: C2RustUnnamed_20 = 0;
pub type C2RustUnnamed_21 = libc::c_uint;
pub const SEQ_PLAYER_BGM_SUB: C2RustUnnamed_21 = 3;
pub const SEQ_PLAYER_SFX: C2RustUnnamed_21 = 2;
pub const SEQ_PLAYER_FANFARE: C2RustUnnamed_21 = 1;
pub const SEQ_PLAYER_BGM_MAIN: C2RustUnnamed_21 = 0;
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct InitChainEntry {
    #[bitfield(name = "cont", ty = "u32_0", bits = "0..=0")]
    #[bitfield(name = "type_0", ty = "u32_0", bits = "1..=4")]
    #[bitfield(name = "offset", ty = "u32_0", bits = "5..=15")]
    #[bitfield(name = "value", ty = "s32", bits = "16..=31")]
    pub cont_type_0_offset_value: [u8; 4],
}
pub type C2RustUnnamed_22 = libc::c_uint;
pub const ICHAINTYPE_VEC3S: C2RustUnnamed_22 = 10;
pub const ICHAINTYPE_VEC3F_DIV1000: C2RustUnnamed_22 = 9;
pub const ICHAINTYPE_VEC3F: C2RustUnnamed_22 = 8;
pub const ICHAINTYPE_F32_DIV1000: C2RustUnnamed_22 = 7;
pub const ICHAINTYPE_F32: C2RustUnnamed_22 = 6;
pub const ICHAINTYPE_S32: C2RustUnnamed_22 = 5;
pub const ICHAINTYPE_U32: C2RustUnnamed_22 = 4;
pub const ICHAINTYPE_S16: C2RustUnnamed_22 = 3;
pub const ICHAINTYPE_U16: C2RustUnnamed_22 = 2;
pub const ICHAINTYPE_S8: C2RustUnnamed_22 = 1;
pub const ICHAINTYPE_U8: C2RustUnnamed_22 = 0;
pub type C2RustUnnamed_23 = libc::c_uint;
pub const MTXMODE_APPLY: C2RustUnnamed_23 = 1;
pub const MTXMODE_NEW: C2RustUnnamed_23 = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct BossGanondrof {
    pub actor: Actor,
    pub skelAnime: SkelAnime,
    pub actionFunc: BossGanondrofActionFunc,
    pub work: [s16; 20],
    pub timers: [s16; 5],
    pub killActor: u8_0,
    pub returnCount: u8_0,
    pub shockTimer: u8_0,
    pub flyMode: u8_0,
    pub returnSuccess: u8_0,
    pub fwork: [f32_0; 13],
    pub spearTip: Vec3f,
    pub targetPos: Vec3f,
    pub bodyPartsPos: [Vec3f; 27],
    pub deathCamera: s16,
    pub deathState: s16,
    pub cameraEye: Vec3f,
    pub cameraAt: Vec3f,
    pub cameraEyeVel: Vec3f,
    pub cameraAtVel: Vec3f,
    pub cameraNextEye: Vec3f,
    pub cameraEyeMaxVel: Vec3f,
    pub cameraNextAt: Vec3f,
    pub cameraAtMaxVel: Vec3f,
    pub cameraSpeedMod: f32_0,
    pub cameraAccel: f32_0,
    pub legRotY: f32_0,
    pub legRotZ: f32_0,
    pub legSplitY: f32_0,
    pub armRotY: f32_0,
    pub armRotZ: f32_0,
    pub rideRotZ: [f32_0; 30],
    pub rideRotY: [f32_0; 30],
    pub lightNode: *mut LightNode,
    pub lightInfo: LightInfo,
    pub colliderBody: ColliderCylinder,
    pub colliderSpear: ColliderCylinder,
}
pub type BossGanondrofActionFunc
    =
    Option<unsafe extern "C" fn(_: *mut BossGanondrof, _: *mut GlobalContext)
               -> ()>;
pub type C2RustUnnamed_24 = libc::c_uint;
pub const GND_FLY_CHARGE: C2RustUnnamed_24 = 4;
pub const GND_FLY_RETURN: C2RustUnnamed_24 = 3;
pub const GND_FLY_VOLLEY: C2RustUnnamed_24 = 2;
pub const GND_FLY_NEUTRAL: C2RustUnnamed_24 = 1;
pub const GND_FLY_PAINTING: C2RustUnnamed_24 = 0;
pub type C2RustUnnamed_25 = libc::c_uint;
pub const GND_EYESTATE_BRIGHTEN: C2RustUnnamed_25 = 2;
pub const GND_EYESTATE_FADE: C2RustUnnamed_25 = 1;
pub const GND_EYESTATE_NONE: C2RustUnnamed_25 = 0;
pub type C2RustUnnamed_26 = libc::c_uint;
pub const GND_SHORT_COUNT: C2RustUnnamed_26 = 20;
pub const GND_DEATH_SFX_TIMER: C2RustUnnamed_26 = 17;
pub const GND_DEATH_ENV_TIMER: C2RustUnnamed_26 = 16;
pub const GND_LIMB_DECAY_INDEX: C2RustUnnamed_26 = 15;
pub const GND_BODY_DECAY_FLAG: C2RustUnnamed_26 = 14;
pub const GND_BODY_DECAY_INDEX: C2RustUnnamed_26 = 13;
pub const GND_PARTICLE_ANGLE: C2RustUnnamed_26 = 12;
pub const GND_EYE_STATE: C2RustUnnamed_26 = 11;
pub const GND_MASK_OFF: C2RustUnnamed_26 = 10;
pub const GND_THROW_COUNT: C2RustUnnamed_26 = 9;
pub const GND_THROW_FRAME: C2RustUnnamed_26 = 8;
pub const GND_ACTION_STATE: C2RustUnnamed_26 = 7;
pub const GND_INVINC_TIMER: C2RustUnnamed_26 = 6;
pub const GND_UNKTIMER_2: C2RustUnnamed_26 = 5;
pub const GND_UNKTIMER_1: C2RustUnnamed_26 = 4;
pub const GND_US_3: C2RustUnnamed_26 = 3;
pub const GND_US_2: C2RustUnnamed_26 = 2;
pub const GND_US_1: C2RustUnnamed_26 = 1;
pub const GND_VARIANCE_TIMER: C2RustUnnamed_26 = 0;
pub type C2RustUnnamed_27 = libc::c_uint;
pub const GND_FLOAT_COUNT: C2RustUnnamed_27 = 13;
pub const GND_EYE_ALPHA: C2RustUnnamed_27 = 5;
pub const GND_CAMERA_ANGLE: C2RustUnnamed_27 = 4;
pub const GND_CAMERA_ZOOM: C2RustUnnamed_27 = 3;
pub const GND_EYE_BRIGHTNESS: C2RustUnnamed_27 = 2;
pub const GND_END_FRAME: C2RustUnnamed_27 = 1;
pub const GND_FLOAT_SPEED: C2RustUnnamed_27 = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct EnfHG {
    pub actor: Actor,
    pub bossGndSignal: u8_0,
    pub bossGndInPainting: u8_0,
    pub killActor: u8_0,
    pub fhgFireKillWarp: u8_0,
    pub cameraEye: Vec3f,
    pub cameraAt: Vec3f,
    pub cameraEyeVel: Vec3f,
    pub cameraAtVel: Vec3f,
    pub hoofSfxPos: Vec3f,
    pub inPaintingPos: Vec3f,
    pub inPaintingVelX: f32_0,
    pub inPaintingVelZ: f32_0,
    pub damageSpeedMod: f32_0,
    pub approachRate: f32_0,
    pub cameraSpeedMod: f32_0,
    pub cameraPanZ: f32_0,
    pub unk_1B0: [libc::c_char; 16],
    pub gallopTimer: s16,
    pub curPainting: s16,
    pub targetPainting: s16,
    pub turnTarget: s16,
    pub spawnedWarp: s16,
    pub cutsceneState: s16,
    pub cutsceneCamera: s16,
    pub unk_1CE: [libc::c_char; 6],
    pub timers: [s16; 5],
    pub hitTimer: s16,
    pub turnRot: s16,
    pub unk_1E2: [libc::c_char; 6],
    pub warpColorFilterR: f32_0,
    pub warpColorFilterG: f32_0,
    pub warpColorFilterB: f32_0,
    pub warpColorFilterUnk1: f32_0,
    pub warpColorFilterUnk2: f32_0,
    pub actionFunc: EnfHGActionFunc,
    pub unk_200: [libc::c_char; 4],
    pub skin: PSkinAwb,
}
pub type EnfHGActionFunc
    =
    Option<unsafe extern "C" fn(_: *mut EnfHG, _: *mut GlobalContext) -> ()>;
pub type C2RustUnnamed_28 = libc::c_uint;
pub const FHG_START_FIGHT: C2RustUnnamed_28 = 255;
pub const FHG_FINISH: C2RustUnnamed_28 = 11;
pub const FHG_SPUR: C2RustUnnamed_28 = 10;
pub const FHG_RIDE: C2RustUnnamed_28 = 5;
pub const FHG_RESET: C2RustUnnamed_28 = 4;
pub const FHG_LIGHTNING: C2RustUnnamed_28 = 3;
pub const FHG_REAR: C2RustUnnamed_28 = 2;
pub const FHG_RAISE_SPEAR: C2RustUnnamed_28 = 1;
pub const FHG_NO_SIGNAL: C2RustUnnamed_28 = 0;
pub type C2RustUnnamed_29 = libc::c_uint;
pub const FHGFIRE_LIGHTNING_TRAIL: C2RustUnnamed_29 = 100;
pub const FHGFIRE_ENERGY_BALL: C2RustUnnamed_29 = 50;
pub const FHGFIRE_WARP_DEATH: C2RustUnnamed_29 = 41;
pub const FHGFIRE_WARP_RETREAT: C2RustUnnamed_29 = 40;
pub const FHGFIRE_WARP_EMERGE: C2RustUnnamed_29 = 39;
pub const FHGFIRE_SPEAR_LIGHT: C2RustUnnamed_29 = 38;
pub const FHGFIRE_LIGHTNING_BURST: C2RustUnnamed_29 = 36;
pub const FHGFIRE_LIGHTNING_SHOCK: C2RustUnnamed_29 = 35;
pub const FHGFIRE_LIGHTNING_STRIKE: C2RustUnnamed_29 = 1;
pub type C2RustUnnamed_30 = libc::c_uint;
pub const FHGFIRE_LIGHT_REFLECT: C2RustUnnamed_30 = 2;
pub const FHGFIRE_LIGHT_BLUE: C2RustUnnamed_30 = 1;
pub const FHGFIRE_LIGHT_GREEN: C2RustUnnamed_30 = 0;
pub type C2RustUnnamed_31 = libc::c_uint;
pub const FHGFLASH_LIGHTBALL_WHITE2: C2RustUnnamed_31 = 8;
pub const FHGFLASH_LIGHTBALL_WHITE1: C2RustUnnamed_31 = 7;
pub const FHGFLASH_LIGHTBALL_ORANGE: C2RustUnnamed_31 = 6;
pub const FHGFLASH_LIGHTBALL_PURPLE: C2RustUnnamed_31 = 5;
pub const FHGFLASH_LIGHTBALL_BLUE: C2RustUnnamed_31 = 4;
pub const FHGFLASH_LIGHTBALL_YELLOW: C2RustUnnamed_31 = 3;
pub const FHGFLASH_LIGHTBALL_RED: C2RustUnnamed_31 = 2;
pub const FHGFLASH_LIGHTBALL_LIGHTBLUE: C2RustUnnamed_31 = 1;
pub const FHGFLASH_LIGHTBALL_GREEN: C2RustUnnamed_31 = 0;
pub type C2RustUnnamed_32 = libc::c_uint;
pub const FHGFLASH_SHOCK_PG: C2RustUnnamed_32 = 2;
pub const FHGFLASH_SHOCK_PLAYER: C2RustUnnamed_32 = 1;
pub const FHGFLASH_SHOCK_NO_ACTOR: C2RustUnnamed_32 = 0;
pub type C2RustUnnamed_33 = libc::c_int;
pub const WARP_RED: C2RustUnnamed_33 = 10;
pub const WARP_GREEN: C2RustUnnamed_33 = 9;
pub const WARP_ORANGE: C2RustUnnamed_33 = 8;
pub const WARP_UNK_7: C2RustUnnamed_33 = 7;
pub const WARP_DESTINATION: C2RustUnnamed_33 = 6;
pub const WARP_BLUE_RUTO: C2RustUnnamed_33 = 5;
pub const WARP_YELLOW: C2RustUnnamed_33 = 4;
pub const WARP_PURPLE_CRYSTAL: C2RustUnnamed_33 = 3;
pub const WARP_SAGES: C2RustUnnamed_33 = 2;
pub const WARP_CLEAR_FLAG: C2RustUnnamed_33 = 1;
pub const WARP_DUNGEON_CHILD: C2RustUnnamed_33 = 0;
pub const WARP_DUNGEON_ADULT: C2RustUnnamed_33 = -1;
pub const WARP_BLUE_CRYSTAL: C2RustUnnamed_33 = -2;
pub type C2RustUnnamed_34 = libc::c_uint;
pub const DEATH_FINISH: C2RustUnnamed_34 = 6;
pub const DEATH_DISINTEGRATE: C2RustUnnamed_34 = 5;
pub const DEATH_SCREAM: C2RustUnnamed_34 = 4;
pub const DEATH_WARP: C2RustUnnamed_34 = 3;
pub const DEATH_THROES: C2RustUnnamed_34 = 2;
pub const DEATH_START: C2RustUnnamed_34 = 1;
pub const NOT_DEAD: C2RustUnnamed_34 = 0;
pub type C2RustUnnamed_35 = libc::c_uint;
pub const THROW_SLOW: C2RustUnnamed_35 = 1;
pub const THROW_NORMAL: C2RustUnnamed_35 = 0;
pub type C2RustUnnamed_36 = libc::c_uint;
pub const STUNNED_GROUND: C2RustUnnamed_36 = 1;
pub const STUNNED_FALL: C2RustUnnamed_36 = 0;
pub type C2RustUnnamed_37 = libc::c_uint;
pub const CHARGE_FINISH: C2RustUnnamed_37 = 3;
pub const CHARGE_ATTACK: C2RustUnnamed_37 = 2;
pub const CHARGE_START: C2RustUnnamed_37 = 1;
pub const CHARGE_WINDUP: C2RustUnnamed_37 = 0;
pub type C2RustUnnamed_38 = libc::c_uint;
pub const DEATH_HUNCHED: C2RustUnnamed_38 = 2;
pub const DEATH_LIMP: C2RustUnnamed_38 = 1;
pub const DEATH_SPASM: C2RustUnnamed_38 = 0;
#[no_mangle]
pub static mut Boss_Ganondrof_InitVars: ActorInit =
    unsafe {
        {
            let mut init =
                ActorInit{id: ACTOR_BOSS_GANONDROF as libc::c_int as s16,
                          category: ACTORCAT_BOSS as libc::c_int as u8_0,
                          flags:
                              ((1 as libc::c_int) << 0 as libc::c_int |
                                   (1 as libc::c_int) << 2 as libc::c_int |
                                   (1 as libc::c_int) << 4 as libc::c_int |
                                   (1 as libc::c_int) << 5 as libc::c_int) as
                                  u32_0,
                          objectId: OBJECT_GND as libc::c_int as s16,
                          instanceSize:
                              ::std::mem::size_of::<BossGanondrof>() as
                                  libc::c_ulong,
                          init:
                              ::std::mem::transmute::<Option<unsafe extern "C" fn(_:
                                                                                      *mut Actor,
                                                                                  _:
                                                                                      *mut GlobalContext)
                                                                 -> ()>,
                                                      ActorFunc>(Some(BossGanondrof_Init
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
                                                      ActorFunc>(Some(BossGanondrof_Destroy
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
                                                      ActorFunc>(Some(BossGanondrof_Update
                                                                          as
                                                                          unsafe extern "C" fn(_:
                                                                                                   *mut Actor,
                                                                                               _:
                                                                                                   *mut GlobalContext)
                                                                              ->
                                                                                  ())),
                          draw:
                              ::std::mem::transmute::<Option<unsafe extern "C" fn(_:
                                                                                      *mut Actor,
                                                                                  _:
                                                                                      *mut GlobalContext)
                                                                 -> ()>,
                                                      ActorFunc>(Some(BossGanondrof_Draw
                                                                          as
                                                                          unsafe extern "C" fn(_:
                                                                                                   *mut Actor,
                                                                                               _:
                                                                                                   *mut GlobalContext)
                                                                              ->
                                                                                  ())),};
            init
        }
    };
static mut sCylinderInitBody: ColliderCylinderInit =
    {
        let mut init =
            ColliderCylinderInit{base:
                                     {
                                         let mut init =
                                             ColliderInit{colType:
                                                              COLTYPE_HIT3 as
                                                                  libc::c_int
                                                                  as u8_0,
                                                          atFlags:
                                                              ((1 as
                                                                    libc::c_int)
                                                                   <<
                                                                   0 as
                                                                       libc::c_int
                                                                   |
                                                                   (1 as
                                                                        libc::c_int)
                                                                       <<
                                                                       4 as
                                                                           libc::c_int)
                                                                  as u8_0,
                                                          acFlags:
                                                              ((1 as
                                                                    libc::c_int)
                                                                   <<
                                                                   0 as
                                                                       libc::c_int
                                                                   |
                                                                   (1 as
                                                                        libc::c_int)
                                                                       <<
                                                                       3 as
                                                                           libc::c_int)
                                                                  as u8_0,
                                                          ocFlags1:
                                                              ((1 as
                                                                    libc::c_int)
                                                                   <<
                                                                   0 as
                                                                       libc::c_int
                                                                   |
                                                                   ((1 as
                                                                         libc::c_int)
                                                                        <<
                                                                        3 as
                                                                            libc::c_int
                                                                        |
                                                                        (1 as
                                                                             libc::c_int)
                                                                            <<
                                                                            4
                                                                                as
                                                                                libc::c_int
                                                                        |
                                                                        (1 as
                                                                             libc::c_int)
                                                                            <<
                                                                            5
                                                                                as
                                                                                libc::c_int))
                                                                  as u8_0,
                                                          ocFlags2:
                                                              ((1 as
                                                                    libc::c_int)
                                                                   <<
                                                                   4 as
                                                                       libc::c_int)
                                                                  as u8_0,
                                                          shape:
                                                              COLSHAPE_CYLINDER
                                                                  as
                                                                  libc::c_int
                                                                  as u8_0,};
                                         init
                                     },
                                 info:
                                     {
                                         let mut init =
                                             ColliderInfoInit{elemType:
                                                                  ELEMTYPE_UNK0
                                                                      as
                                                                      libc::c_int
                                                                      as u8_0,
                                                              toucher:
                                                                  {
                                                                      let mut init =
                                                                          ColliderTouch{dmgFlags:
                                                                                            0xffcfffff
                                                                                                as
                                                                                                libc::c_uint,
                                                                                        effect:
                                                                                            0
                                                                                                as
                                                                                                libc::c_int
                                                                                                as
                                                                                                u8_0,
                                                                                        damage:
                                                                                            0x10
                                                                                                as
                                                                                                libc::c_int
                                                                                                as
                                                                                                u8_0,};
                                                                      init
                                                                  },
                                                              bumper:
                                                                  {
                                                                      let mut init =
                                                                          ColliderBumpInit{dmgFlags:
                                                                                               0xffcffffe
                                                                                                   as
                                                                                                   libc::c_uint,
                                                                                           effect:
                                                                                               0
                                                                                                   as
                                                                                                   libc::c_int
                                                                                                   as
                                                                                                   u8_0,
                                                                                           defense:
                                                                                               0
                                                                                                   as
                                                                                                   libc::c_int
                                                                                                   as
                                                                                                   u8_0,};
                                                                      init
                                                                  },
                                                              toucherFlags:
                                                                  ((1 as
                                                                        libc::c_int)
                                                                       <<
                                                                       0 as
                                                                           libc::c_int
                                                                       |
                                                                       (0 as
                                                                            libc::c_int)
                                                                           <<
                                                                           3
                                                                               as
                                                                               libc::c_int)
                                                                      as u8_0,
                                                              bumperFlags:
                                                                  ((1 as
                                                                        libc::c_int)
                                                                       <<
                                                                       0 as
                                                                           libc::c_int
                                                                       |
                                                                       (1 as
                                                                            libc::c_int)
                                                                           <<
                                                                           2
                                                                               as
                                                                               libc::c_int)
                                                                      as u8_0,
                                                              ocElemFlags:
                                                                  ((1 as
                                                                        libc::c_int)
                                                                       <<
                                                                       0 as
                                                                           libc::c_int)
                                                                      as
                                                                      u8_0,};
                                         init
                                     },
                                 dim:
                                     {
                                         let mut init =
                                             Cylinder16{radius:
                                                            30 as libc::c_int
                                                                as s16,
                                                        height:
                                                            90 as libc::c_int
                                                                as s16,
                                                        yShift:
                                                            -(50 as
                                                                  libc::c_int)
                                                                as s16,
                                                        pos:
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
                                     },};
        init
    };
static mut sCylinderInitSpear: ColliderCylinderInit =
    {
        let mut init =
            ColliderCylinderInit{base:
                                     {
                                         let mut init =
                                             ColliderInit{colType:
                                                              COLTYPE_HIT3 as
                                                                  libc::c_int
                                                                  as u8_0,
                                                          atFlags:
                                                              ((1 as
                                                                    libc::c_int)
                                                                   <<
                                                                   0 as
                                                                       libc::c_int
                                                                   |
                                                                   (1 as
                                                                        libc::c_int)
                                                                       <<
                                                                       4 as
                                                                           libc::c_int)
                                                                  as u8_0,
                                                          acFlags:
                                                              ((1 as
                                                                    libc::c_int)
                                                                   <<
                                                                   0 as
                                                                       libc::c_int
                                                                   |
                                                                   (1 as
                                                                        libc::c_int)
                                                                       <<
                                                                       3 as
                                                                           libc::c_int)
                                                                  as u8_0,
                                                          ocFlags1:
                                                              ((1 as
                                                                    libc::c_int)
                                                                   <<
                                                                   0 as
                                                                       libc::c_int
                                                                   |
                                                                   ((1 as
                                                                         libc::c_int)
                                                                        <<
                                                                        3 as
                                                                            libc::c_int
                                                                        |
                                                                        (1 as
                                                                             libc::c_int)
                                                                            <<
                                                                            4
                                                                                as
                                                                                libc::c_int
                                                                        |
                                                                        (1 as
                                                                             libc::c_int)
                                                                            <<
                                                                            5
                                                                                as
                                                                                libc::c_int))
                                                                  as u8_0,
                                                          ocFlags2:
                                                              ((1 as
                                                                    libc::c_int)
                                                                   <<
                                                                   4 as
                                                                       libc::c_int)
                                                                  as u8_0,
                                                          shape:
                                                              COLSHAPE_CYLINDER
                                                                  as
                                                                  libc::c_int
                                                                  as u8_0,};
                                         init
                                     },
                                 info:
                                     {
                                         let mut init =
                                             ColliderInfoInit{elemType:
                                                                  ELEMTYPE_UNK0
                                                                      as
                                                                      libc::c_int
                                                                      as u8_0,
                                                              toucher:
                                                                  {
                                                                      let mut init =
                                                                          ColliderTouch{dmgFlags:
                                                                                            0xffcfffff
                                                                                                as
                                                                                                libc::c_uint,
                                                                                        effect:
                                                                                            0
                                                                                                as
                                                                                                libc::c_int
                                                                                                as
                                                                                                u8_0,
                                                                                        damage:
                                                                                            0x30
                                                                                                as
                                                                                                libc::c_int
                                                                                                as
                                                                                                u8_0,};
                                                                      init
                                                                  },
                                                              bumper:
                                                                  {
                                                                      let mut init =
                                                                          ColliderBumpInit{dmgFlags:
                                                                                               0xffcfffff
                                                                                                   as
                                                                                                   libc::c_uint,
                                                                                           effect:
                                                                                               0
                                                                                                   as
                                                                                                   libc::c_int
                                                                                                   as
                                                                                                   u8_0,
                                                                                           defense:
                                                                                               0
                                                                                                   as
                                                                                                   libc::c_int
                                                                                                   as
                                                                                                   u8_0,};
                                                                      init
                                                                  },
                                                              toucherFlags:
                                                                  ((1 as
                                                                        libc::c_int)
                                                                       <<
                                                                       0 as
                                                                           libc::c_int
                                                                       |
                                                                       (0 as
                                                                            libc::c_int)
                                                                           <<
                                                                           3
                                                                               as
                                                                               libc::c_int)
                                                                      as u8_0,
                                                              bumperFlags:
                                                                  ((1 as
                                                                        libc::c_int)
                                                                       <<
                                                                       0 as
                                                                           libc::c_int)
                                                                      as u8_0,
                                                              ocElemFlags:
                                                                  ((1 as
                                                                        libc::c_int)
                                                                       <<
                                                                       0 as
                                                                           libc::c_int)
                                                                      as
                                                                      u8_0,};
                                         init
                                     },
                                 dim:
                                     {
                                         let mut init =
                                             Cylinder16{radius:
                                                            20 as libc::c_int
                                                                as s16,
                                                        height:
                                                            30 as libc::c_int
                                                                as s16,
                                                        yShift:
                                                            -(20 as
                                                                  libc::c_int)
                                                                as s16,
                                                        pos:
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
                                     },};
        init
    };
// clang-format off
static mut sDecayMaskHigh: [u8_0; 256] =
    [1 as libc::c_int as u8_0, 1 as libc::c_int as u8_0,
     1 as libc::c_int as u8_0, 1 as libc::c_int as u8_0,
     1 as libc::c_int as u8_0, 1 as libc::c_int as u8_0,
     1 as libc::c_int as u8_0, 1 as libc::c_int as u8_0,
     1 as libc::c_int as u8_0, 1 as libc::c_int as u8_0,
     1 as libc::c_int as u8_0, 1 as libc::c_int as u8_0,
     1 as libc::c_int as u8_0, 1 as libc::c_int as u8_0,
     1 as libc::c_int as u8_0, 1 as libc::c_int as u8_0,
     1 as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
     0 as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
     0 as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
     0 as libc::c_int as u8_0, 1 as libc::c_int as u8_0,
     1 as libc::c_int as u8_0, 1 as libc::c_int as u8_0,
     1 as libc::c_int as u8_0, 1 as libc::c_int as u8_0,
     1 as libc::c_int as u8_0, 1 as libc::c_int as u8_0,
     1 as libc::c_int as u8_0, 1 as libc::c_int as u8_0,
     1 as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
     1 as libc::c_int as u8_0, 1 as libc::c_int as u8_0,
     0 as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
     0 as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
     1 as libc::c_int as u8_0, 1 as libc::c_int as u8_0,
     1 as libc::c_int as u8_0, 1 as libc::c_int as u8_0,
     1 as libc::c_int as u8_0, 1 as libc::c_int as u8_0,
     0 as libc::c_int as u8_0, 1 as libc::c_int as u8_0,
     1 as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
     1 as libc::c_int as u8_0, 1 as libc::c_int as u8_0,
     1 as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
     0 as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
     0 as libc::c_int as u8_0, 1 as libc::c_int as u8_0,
     1 as libc::c_int as u8_0, 1 as libc::c_int as u8_0,
     1 as libc::c_int as u8_0, 1 as libc::c_int as u8_0,
     0 as libc::c_int as u8_0, 1 as libc::c_int as u8_0,
     1 as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
     0 as libc::c_int as u8_0, 1 as libc::c_int as u8_0,
     1 as libc::c_int as u8_0, 1 as libc::c_int as u8_0,
     1 as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
     0 as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
     1 as libc::c_int as u8_0, 1 as libc::c_int as u8_0,
     1 as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
     0 as libc::c_int as u8_0, 1 as libc::c_int as u8_0,
     1 as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
     0 as libc::c_int as u8_0, 1 as libc::c_int as u8_0,
     1 as libc::c_int as u8_0, 1 as libc::c_int as u8_0,
     1 as libc::c_int as u8_0, 1 as libc::c_int as u8_0,
     0 as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
     0 as libc::c_int as u8_0, 1 as libc::c_int as u8_0,
     1 as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
     0 as libc::c_int as u8_0, 1 as libc::c_int as u8_0,
     1 as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
     1 as libc::c_int as u8_0, 1 as libc::c_int as u8_0,
     1 as libc::c_int as u8_0, 1 as libc::c_int as u8_0,
     0 as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
     0 as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
     0 as libc::c_int as u8_0, 1 as libc::c_int as u8_0,
     0 as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
     0 as libc::c_int as u8_0, 1 as libc::c_int as u8_0,
     1 as libc::c_int as u8_0, 1 as libc::c_int as u8_0,
     1 as libc::c_int as u8_0, 1 as libc::c_int as u8_0,
     1 as libc::c_int as u8_0, 1 as libc::c_int as u8_0,
     1 as libc::c_int as u8_0, 1 as libc::c_int as u8_0,
     0 as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
     0 as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
     1 as libc::c_int as u8_0, 1 as libc::c_int as u8_0,
     0 as libc::c_int as u8_0, 1 as libc::c_int as u8_0,
     1 as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
     1 as libc::c_int as u8_0, 1 as libc::c_int as u8_0,
     1 as libc::c_int as u8_0, 1 as libc::c_int as u8_0,
     1 as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
     0 as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
     0 as libc::c_int as u8_0, 1 as libc::c_int as u8_0,
     1 as libc::c_int as u8_0, 1 as libc::c_int as u8_0,
     0 as libc::c_int as u8_0, 1 as libc::c_int as u8_0,
     1 as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
     0 as libc::c_int as u8_0, 1 as libc::c_int as u8_0,
     1 as libc::c_int as u8_0, 1 as libc::c_int as u8_0,
     0 as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
     0 as libc::c_int as u8_0, 1 as libc::c_int as u8_0,
     1 as libc::c_int as u8_0, 1 as libc::c_int as u8_0,
     1 as libc::c_int as u8_0, 1 as libc::c_int as u8_0,
     0 as libc::c_int as u8_0, 1 as libc::c_int as u8_0,
     1 as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
     0 as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
     0 as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
     0 as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
     0 as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
     1 as libc::c_int as u8_0, 1 as libc::c_int as u8_0,
     1 as libc::c_int as u8_0, 1 as libc::c_int as u8_0,
     0 as libc::c_int as u8_0, 1 as libc::c_int as u8_0,
     1 as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
     0 as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
     1 as libc::c_int as u8_0, 1 as libc::c_int as u8_0,
     0 as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
     0 as libc::c_int as u8_0, 1 as libc::c_int as u8_0,
     1 as libc::c_int as u8_0, 1 as libc::c_int as u8_0,
     1 as libc::c_int as u8_0, 1 as libc::c_int as u8_0,
     1 as libc::c_int as u8_0, 1 as libc::c_int as u8_0,
     1 as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
     0 as libc::c_int as u8_0, 1 as libc::c_int as u8_0,
     1 as libc::c_int as u8_0, 1 as libc::c_int as u8_0,
     1 as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
     1 as libc::c_int as u8_0, 1 as libc::c_int as u8_0,
     1 as libc::c_int as u8_0, 1 as libc::c_int as u8_0,
     1 as libc::c_int as u8_0, 1 as libc::c_int as u8_0,
     1 as libc::c_int as u8_0, 1 as libc::c_int as u8_0,
     1 as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
     1 as libc::c_int as u8_0, 1 as libc::c_int as u8_0,
     1 as libc::c_int as u8_0, 1 as libc::c_int as u8_0,
     1 as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
     0 as libc::c_int as u8_0, 1 as libc::c_int as u8_0,
     1 as libc::c_int as u8_0, 1 as libc::c_int as u8_0,
     1 as libc::c_int as u8_0, 1 as libc::c_int as u8_0,
     0 as libc::c_int as u8_0, 1 as libc::c_int as u8_0,
     1 as libc::c_int as u8_0, 1 as libc::c_int as u8_0,
     1 as libc::c_int as u8_0, 1 as libc::c_int as u8_0,
     1 as libc::c_int as u8_0, 1 as libc::c_int as u8_0,
     1 as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
     0 as libc::c_int as u8_0, 1 as libc::c_int as u8_0,
     1 as libc::c_int as u8_0, 1 as libc::c_int as u8_0,
     0 as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
     0 as libc::c_int as u8_0, 1 as libc::c_int as u8_0,
     1 as libc::c_int as u8_0, 1 as libc::c_int as u8_0,
     1 as libc::c_int as u8_0, 1 as libc::c_int as u8_0,
     1 as libc::c_int as u8_0, 1 as libc::c_int as u8_0,
     1 as libc::c_int as u8_0, 1 as libc::c_int as u8_0,
     1 as libc::c_int as u8_0, 1 as libc::c_int as u8_0,
     1 as libc::c_int as u8_0, 1 as libc::c_int as u8_0,
     1 as libc::c_int as u8_0, 1 as libc::c_int as u8_0,
     1 as libc::c_int as u8_0, 1 as libc::c_int as u8_0];
static mut sDecayMaskLow: [u8_0; 256] =
    [1 as libc::c_int as u8_0, 1 as libc::c_int as u8_0,
     1 as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
     1 as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
     0 as libc::c_int as u8_0, 1 as libc::c_int as u8_0,
     0 as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
     1 as libc::c_int as u8_0, 1 as libc::c_int as u8_0,
     1 as libc::c_int as u8_0, 1 as libc::c_int as u8_0,
     1 as libc::c_int as u8_0, 1 as libc::c_int as u8_0,
     0 as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
     0 as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
     0 as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
     0 as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
     1 as libc::c_int as u8_0, 1 as libc::c_int as u8_0,
     1 as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
     0 as libc::c_int as u8_0, 1 as libc::c_int as u8_0,
     1 as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
     1 as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
     1 as libc::c_int as u8_0, 1 as libc::c_int as u8_0,
     0 as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
     0 as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
     0 as libc::c_int as u8_0, 1 as libc::c_int as u8_0,
     1 as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
     0 as libc::c_int as u8_0, 1 as libc::c_int as u8_0,
     0 as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
     1 as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
     0 as libc::c_int as u8_0, 1 as libc::c_int as u8_0,
     1 as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
     0 as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
     0 as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
     1 as libc::c_int as u8_0, 1 as libc::c_int as u8_0,
     1 as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
     0 as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
     0 as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
     0 as libc::c_int as u8_0, 1 as libc::c_int as u8_0,
     1 as libc::c_int as u8_0, 1 as libc::c_int as u8_0,
     0 as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
     0 as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
     0 as libc::c_int as u8_0, 1 as libc::c_int as u8_0,
     1 as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
     0 as libc::c_int as u8_0, 1 as libc::c_int as u8_0,
     0 as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
     0 as libc::c_int as u8_0, 1 as libc::c_int as u8_0,
     0 as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
     1 as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
     0 as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
     0 as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
     1 as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
     0 as libc::c_int as u8_0, 1 as libc::c_int as u8_0,
     1 as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
     1 as libc::c_int as u8_0, 1 as libc::c_int as u8_0,
     1 as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
     0 as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
     0 as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
     0 as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
     0 as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
     0 as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
     1 as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
     0 as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
     1 as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
     0 as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
     0 as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
     0 as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
     0 as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
     0 as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
     1 as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
     0 as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
     1 as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
     0 as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
     0 as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
     0 as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
     1 as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
     0 as libc::c_int as u8_0, 1 as libc::c_int as u8_0,
     0 as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
     0 as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
     0 as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
     0 as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
     0 as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
     0 as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
     1 as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
     0 as libc::c_int as u8_0, 1 as libc::c_int as u8_0,
     0 as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
     0 as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
     0 as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
     0 as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
     0 as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
     0 as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
     1 as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
     0 as libc::c_int as u8_0, 1 as libc::c_int as u8_0,
     1 as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
     0 as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
     0 as libc::c_int as u8_0, 1 as libc::c_int as u8_0,
     0 as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
     0 as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
     0 as libc::c_int as u8_0, 1 as libc::c_int as u8_0,
     1 as libc::c_int as u8_0, 1 as libc::c_int as u8_0,
     0 as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
     1 as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
     0 as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
     0 as libc::c_int as u8_0, 1 as libc::c_int as u8_0,
     0 as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
     0 as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
     1 as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
     1 as libc::c_int as u8_0, 1 as libc::c_int as u8_0,
     0 as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
     0 as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
     0 as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
     1 as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
     0 as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
     0 as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
     0 as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
     1 as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
     0 as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
     1 as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
     0 as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
     1 as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
     1 as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
     0 as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
     1 as libc::c_int as u8_0, 1 as libc::c_int as u8_0,
     0 as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
     0 as libc::c_int as u8_0, 1 as libc::c_int as u8_0,
     1 as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
     0 as libc::c_int as u8_0, 1 as libc::c_int as u8_0,
     1 as libc::c_int as u8_0, 1 as libc::c_int as u8_0,
     0 as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
     0 as libc::c_int as u8_0, 1 as libc::c_int as u8_0,
     1 as libc::c_int as u8_0, 1 as libc::c_int as u8_0,
     0 as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
     0 as libc::c_int as u8_0, 1 as libc::c_int as u8_0];
static mut sDecayMaskTotal: [u8_0; 256] =
    [1 as libc::c_int as u8_0, 1 as libc::c_int as u8_0,
     1 as libc::c_int as u8_0, 1 as libc::c_int as u8_0,
     1 as libc::c_int as u8_0, 1 as libc::c_int as u8_0,
     1 as libc::c_int as u8_0, 1 as libc::c_int as u8_0,
     1 as libc::c_int as u8_0, 1 as libc::c_int as u8_0,
     1 as libc::c_int as u8_0, 1 as libc::c_int as u8_0,
     1 as libc::c_int as u8_0, 1 as libc::c_int as u8_0,
     1 as libc::c_int as u8_0, 1 as libc::c_int as u8_0,
     1 as libc::c_int as u8_0, 1 as libc::c_int as u8_0,
     1 as libc::c_int as u8_0, 1 as libc::c_int as u8_0,
     1 as libc::c_int as u8_0, 1 as libc::c_int as u8_0,
     1 as libc::c_int as u8_0, 1 as libc::c_int as u8_0,
     1 as libc::c_int as u8_0, 1 as libc::c_int as u8_0,
     1 as libc::c_int as u8_0, 1 as libc::c_int as u8_0,
     1 as libc::c_int as u8_0, 1 as libc::c_int as u8_0,
     1 as libc::c_int as u8_0, 1 as libc::c_int as u8_0,
     1 as libc::c_int as u8_0, 1 as libc::c_int as u8_0,
     1 as libc::c_int as u8_0, 1 as libc::c_int as u8_0,
     1 as libc::c_int as u8_0, 1 as libc::c_int as u8_0,
     1 as libc::c_int as u8_0, 1 as libc::c_int as u8_0,
     1 as libc::c_int as u8_0, 1 as libc::c_int as u8_0,
     1 as libc::c_int as u8_0, 1 as libc::c_int as u8_0,
     1 as libc::c_int as u8_0, 1 as libc::c_int as u8_0,
     1 as libc::c_int as u8_0, 1 as libc::c_int as u8_0,
     1 as libc::c_int as u8_0, 1 as libc::c_int as u8_0,
     1 as libc::c_int as u8_0, 1 as libc::c_int as u8_0,
     1 as libc::c_int as u8_0, 1 as libc::c_int as u8_0,
     1 as libc::c_int as u8_0, 1 as libc::c_int as u8_0,
     1 as libc::c_int as u8_0, 1 as libc::c_int as u8_0,
     1 as libc::c_int as u8_0, 1 as libc::c_int as u8_0,
     1 as libc::c_int as u8_0, 1 as libc::c_int as u8_0,
     1 as libc::c_int as u8_0, 1 as libc::c_int as u8_0,
     1 as libc::c_int as u8_0, 1 as libc::c_int as u8_0,
     1 as libc::c_int as u8_0, 1 as libc::c_int as u8_0,
     1 as libc::c_int as u8_0, 1 as libc::c_int as u8_0,
     1 as libc::c_int as u8_0, 1 as libc::c_int as u8_0,
     1 as libc::c_int as u8_0, 1 as libc::c_int as u8_0,
     1 as libc::c_int as u8_0, 1 as libc::c_int as u8_0,
     1 as libc::c_int as u8_0, 1 as libc::c_int as u8_0,
     1 as libc::c_int as u8_0, 1 as libc::c_int as u8_0,
     1 as libc::c_int as u8_0, 1 as libc::c_int as u8_0,
     1 as libc::c_int as u8_0, 1 as libc::c_int as u8_0,
     1 as libc::c_int as u8_0, 1 as libc::c_int as u8_0,
     1 as libc::c_int as u8_0, 1 as libc::c_int as u8_0,
     1 as libc::c_int as u8_0, 1 as libc::c_int as u8_0,
     1 as libc::c_int as u8_0, 1 as libc::c_int as u8_0,
     1 as libc::c_int as u8_0, 1 as libc::c_int as u8_0,
     1 as libc::c_int as u8_0, 1 as libc::c_int as u8_0,
     1 as libc::c_int as u8_0, 1 as libc::c_int as u8_0,
     1 as libc::c_int as u8_0, 1 as libc::c_int as u8_0,
     1 as libc::c_int as u8_0, 1 as libc::c_int as u8_0,
     1 as libc::c_int as u8_0, 1 as libc::c_int as u8_0,
     1 as libc::c_int as u8_0, 1 as libc::c_int as u8_0,
     1 as libc::c_int as u8_0, 1 as libc::c_int as u8_0,
     1 as libc::c_int as u8_0, 1 as libc::c_int as u8_0,
     1 as libc::c_int as u8_0, 1 as libc::c_int as u8_0,
     1 as libc::c_int as u8_0, 1 as libc::c_int as u8_0,
     1 as libc::c_int as u8_0, 1 as libc::c_int as u8_0,
     1 as libc::c_int as u8_0, 1 as libc::c_int as u8_0,
     1 as libc::c_int as u8_0, 1 as libc::c_int as u8_0,
     1 as libc::c_int as u8_0, 1 as libc::c_int as u8_0,
     1 as libc::c_int as u8_0, 1 as libc::c_int as u8_0,
     1 as libc::c_int as u8_0, 1 as libc::c_int as u8_0,
     1 as libc::c_int as u8_0, 1 as libc::c_int as u8_0,
     1 as libc::c_int as u8_0, 1 as libc::c_int as u8_0,
     1 as libc::c_int as u8_0, 1 as libc::c_int as u8_0,
     1 as libc::c_int as u8_0, 1 as libc::c_int as u8_0,
     1 as libc::c_int as u8_0, 1 as libc::c_int as u8_0,
     1 as libc::c_int as u8_0, 1 as libc::c_int as u8_0,
     1 as libc::c_int as u8_0, 1 as libc::c_int as u8_0,
     1 as libc::c_int as u8_0, 1 as libc::c_int as u8_0,
     1 as libc::c_int as u8_0, 1 as libc::c_int as u8_0,
     1 as libc::c_int as u8_0, 1 as libc::c_int as u8_0,
     1 as libc::c_int as u8_0, 1 as libc::c_int as u8_0,
     1 as libc::c_int as u8_0, 1 as libc::c_int as u8_0,
     1 as libc::c_int as u8_0, 1 as libc::c_int as u8_0,
     1 as libc::c_int as u8_0, 1 as libc::c_int as u8_0,
     1 as libc::c_int as u8_0, 1 as libc::c_int as u8_0,
     1 as libc::c_int as u8_0, 1 as libc::c_int as u8_0,
     1 as libc::c_int as u8_0, 1 as libc::c_int as u8_0,
     1 as libc::c_int as u8_0, 1 as libc::c_int as u8_0,
     1 as libc::c_int as u8_0, 1 as libc::c_int as u8_0,
     1 as libc::c_int as u8_0, 1 as libc::c_int as u8_0,
     1 as libc::c_int as u8_0, 1 as libc::c_int as u8_0,
     1 as libc::c_int as u8_0, 1 as libc::c_int as u8_0,
     1 as libc::c_int as u8_0, 1 as libc::c_int as u8_0,
     1 as libc::c_int as u8_0, 1 as libc::c_int as u8_0,
     1 as libc::c_int as u8_0, 1 as libc::c_int as u8_0,
     1 as libc::c_int as u8_0, 1 as libc::c_int as u8_0,
     1 as libc::c_int as u8_0, 1 as libc::c_int as u8_0,
     1 as libc::c_int as u8_0, 1 as libc::c_int as u8_0,
     1 as libc::c_int as u8_0, 1 as libc::c_int as u8_0,
     1 as libc::c_int as u8_0, 1 as libc::c_int as u8_0,
     1 as libc::c_int as u8_0, 1 as libc::c_int as u8_0,
     1 as libc::c_int as u8_0, 1 as libc::c_int as u8_0,
     1 as libc::c_int as u8_0, 1 as libc::c_int as u8_0,
     1 as libc::c_int as u8_0, 1 as libc::c_int as u8_0,
     1 as libc::c_int as u8_0, 1 as libc::c_int as u8_0,
     1 as libc::c_int as u8_0, 1 as libc::c_int as u8_0,
     1 as libc::c_int as u8_0, 1 as libc::c_int as u8_0,
     1 as libc::c_int as u8_0, 1 as libc::c_int as u8_0,
     1 as libc::c_int as u8_0, 1 as libc::c_int as u8_0,
     1 as libc::c_int as u8_0, 1 as libc::c_int as u8_0,
     1 as libc::c_int as u8_0, 1 as libc::c_int as u8_0,
     1 as libc::c_int as u8_0, 1 as libc::c_int as u8_0,
     1 as libc::c_int as u8_0, 1 as libc::c_int as u8_0,
     1 as libc::c_int as u8_0, 1 as libc::c_int as u8_0,
     1 as libc::c_int as u8_0, 1 as libc::c_int as u8_0,
     1 as libc::c_int as u8_0, 1 as libc::c_int as u8_0,
     1 as libc::c_int as u8_0, 1 as libc::c_int as u8_0,
     1 as libc::c_int as u8_0, 1 as libc::c_int as u8_0,
     1 as libc::c_int as u8_0, 1 as libc::c_int as u8_0,
     1 as libc::c_int as u8_0, 1 as libc::c_int as u8_0,
     1 as libc::c_int as u8_0, 1 as libc::c_int as u8_0,
     1 as libc::c_int as u8_0, 1 as libc::c_int as u8_0,
     1 as libc::c_int as u8_0, 1 as libc::c_int as u8_0,
     1 as libc::c_int as u8_0, 1 as libc::c_int as u8_0,
     1 as libc::c_int as u8_0, 1 as libc::c_int as u8_0,
     1 as libc::c_int as u8_0, 1 as libc::c_int as u8_0,
     1 as libc::c_int as u8_0, 1 as libc::c_int as u8_0,
     1 as libc::c_int as u8_0, 1 as libc::c_int as u8_0,
     1 as libc::c_int as u8_0, 1 as libc::c_int as u8_0,
     1 as libc::c_int as u8_0, 1 as libc::c_int as u8_0,
     1 as libc::c_int as u8_0, 1 as libc::c_int as u8_0,
     1 as libc::c_int as u8_0, 1 as libc::c_int as u8_0,
     1 as libc::c_int as u8_0, 1 as libc::c_int as u8_0,
     1 as libc::c_int as u8_0, 1 as libc::c_int as u8_0,
     1 as libc::c_int as u8_0, 1 as libc::c_int as u8_0];
// clang-format on
// These are Phantom Ganon's body textures, but I don't know which is which.
static mut sLimbTex_rgba16_8x8: [*mut libc::c_void; 5] =
    unsafe {
        [gPhantomGanonLimbTex_00A800.as_ptr() as *mut _ as *mut libc::c_void,
         gPhantomGanonLimbTex_00AE80.as_ptr() as *mut _ as *mut libc::c_void,
         gPhantomGanonLimbTex_00AF00.as_ptr() as *mut _ as *mut libc::c_void,
         gPhantomGanonLimbTex_00C180.as_ptr() as *mut _ as *mut libc::c_void,
         gPhantomGanonLimbTex_00C400.as_ptr() as *mut _ as *mut libc::c_void]
    };
static mut sLimbTex_rgba16_16x8: [*mut libc::c_void; 5] =
    unsafe {
        [gPhantomGanonLimbTex_00B980.as_ptr() as *mut _ as *mut libc::c_void,
         gPhantomGanonLimbTex_00C480.as_ptr() as *mut _ as *mut libc::c_void,
         gPhantomGanonLimbTex_00BC80.as_ptr() as *mut _ as *mut libc::c_void,
         gPhantomGanonLimbTex_00BD80.as_ptr() as *mut _ as *mut libc::c_void,
         gPhantomGanonLimbTex_00C080.as_ptr() as *mut _ as *mut libc::c_void]
    };
static mut sLimbTex_rgba16_16x16: [*mut libc::c_void; 9] =
    unsafe {
        [gPhantomGanonLimbTex_00C200.as_ptr() as *mut _ as *mut libc::c_void,
         gPhantomGanonLimbTex_00A000.as_ptr() as *mut _ as *mut libc::c_void,
         gPhantomGanonLimbTex_00A200.as_ptr() as *mut _ as *mut libc::c_void,
         gPhantomGanonLimbTex_00A400.as_ptr() as *mut _ as *mut libc::c_void,
         gPhantomGanonLimbTex_00A600.as_ptr() as *mut _ as *mut libc::c_void,
         gPhantomGanonLimbTex_00A880.as_ptr() as *mut _ as *mut libc::c_void,
         gPhantomGanonLimbTex_00B780.as_ptr() as *mut _ as *mut libc::c_void,
         gPhantomGanonLimbTex_00BA80.as_ptr() as *mut _ as *mut libc::c_void,
         gPhantomGanonLimbTex_00BE80.as_ptr() as *mut _ as *mut libc::c_void]
    };
static mut sLimbTex_rgba16_16x32: [*mut libc::c_void; 2] =
    unsafe {
        [gPhantomGanonLimbTex_00AA80.as_ptr() as *mut _ as *mut libc::c_void,
         gPhantomGanonLimbTex_00AF80.as_ptr() as *mut _ as *mut libc::c_void]
    };
static mut sMouthTex_ci8_16x16: [*mut libc::c_void; 2] =
    unsafe {
        [gPhantomGanonMouthTex.as_ptr() as *mut _ as *mut libc::c_void,
         gPhantomGanonSmileTex.as_ptr() as *mut _ as *mut libc::c_void]
    };
// Initialized in run_static_initializers
static mut sInitChain: [InitChainEntry; 4] =
    [InitChainEntry{cont_type_0_offset_value: [0; 4],}; 4];
static mut sAudioVec: Vec3f =
    { let mut init = Vec3f{x: 0.0f32, y: 0.0f32, z: 50.0f32,}; init };
#[no_mangle]
pub unsafe extern "C" fn BossGanondrof_ClearPixels8x8(mut texture: *mut s16,
                                                      mut mask: *mut u8_0,
                                                      mut index: s16) {
    if *mask.offset(index as isize) != 0 {
        *texture.offset((index as libc::c_int / 4 as libc::c_int) as isize) =
            0 as libc::c_int as s16
    };
}
#[no_mangle]
pub unsafe extern "C" fn BossGanondrof_ClearPixels16x8(mut texture: *mut s16,
                                                       mut mask: *mut u8_0,
                                                       mut index: s16) {
    if *mask.offset(index as isize) != 0 {
        *texture.offset((index as libc::c_int / 2 as libc::c_int) as isize) =
            0 as libc::c_int as s16
    };
}
#[no_mangle]
pub unsafe extern "C" fn BossGanondrof_ClearPixels16x16(mut texture: *mut s16,
                                                        mut mask: *mut u8_0,
                                                        mut index: s16) {
    if *mask.offset(index as isize) != 0 {
        *texture.offset(index as isize) = 0 as libc::c_int as s16
    };
}
#[no_mangle]
pub unsafe extern "C" fn BossGanondrof_ClearPixels32x16(mut texture: *mut s16,
                                                        mut mask: *mut u8_0,
                                                        mut index: s16) {
    if *mask.offset(index as isize) != 0 {
        let mut i: s16 =
            ((index as libc::c_int & 0xf as libc::c_int) +
                 ((index as libc::c_int & 0xf0 as libc::c_int) <<
                      1 as libc::c_int)) as s16;
        *texture.offset((i as libc::c_int + 0x10 as libc::c_int) as isize) =
            0 as libc::c_int as s16;
        *texture.offset(i as isize) = 0 as libc::c_int as s16
    };
}
#[no_mangle]
pub unsafe extern "C" fn BossGanondrof_ClearPixels16x32(mut texture: *mut s16,
                                                        mut mask: *mut u8_0,
                                                        mut index: s16) {
    if *mask.offset(index as isize) != 0 {
        let mut i: s16 =
            ((index as libc::c_int & 0xf as libc::c_int) * 2 as libc::c_int +
                 (index as libc::c_int & 0xf0 as libc::c_int) *
                     2 as libc::c_int) as s16;
        *texture.offset((i as libc::c_int + 1 as libc::c_int) as isize) =
            0 as libc::c_int as s16;
        *texture.offset(i as isize) = 0 as libc::c_int as s16
    };
}
#[no_mangle]
pub unsafe extern "C" fn BossGanondrof_ClearPixels(mut mask: *mut u8_0,
                                                   mut index: s16) {
    let mut i: s16 = 0;
    i = 0 as libc::c_int as s16;
    while (i as libc::c_int) < 5 as libc::c_int {
        // ARRAY_COUNT can't be used here because the arrays aren't guaranteed to be the same size.
        BossGanondrof_ClearPixels8x8(gSegments[((sLimbTex_rgba16_8x8[i as
                                                                         usize]
                                                     as u32_0) <<
                                                    4 as libc::c_int >>
                                                    28 as libc::c_int) as
                                                   usize].wrapping_add(sLimbTex_rgba16_8x8[i
                                                                                               as
                                                                                               usize]
                                                                           as
                                                                           u32_0
                                                                           &
                                                                           0xffffff
                                                                               as
                                                                               libc::c_int
                                                                               as
                                                                               libc::c_uint).wrapping_add(0x80000000
                                                                                                              as
                                                                                                              libc::c_uint)
                                         as *mut libc::c_void as *mut s16,
                                     mask,
                                     index); // approaches GND_BOSSROOM_CENTER_Y + 33.0f
        BossGanondrof_ClearPixels16x8(gSegments[((sLimbTex_rgba16_16x8[i as
                                                                           usize]
                                                      as u32_0) <<
                                                     4 as libc::c_int >>
                                                     28 as libc::c_int) as
                                                    usize].wrapping_add(sLimbTex_rgba16_16x8[i
                                                                                                 as
                                                                                                 usize]
                                                                            as
                                                                            u32_0
                                                                            &
                                                                            0xffffff
                                                                                as
                                                                                libc::c_int
                                                                                as
                                                                                libc::c_uint).wrapping_add(0x80000000
                                                                                                               as
                                                                                                               libc::c_uint)
                                          as *mut libc::c_void as *mut s16,
                                      mask,
                                      index); // GND_BOSSROOM_CENTER_Y + 33.0f
        i += 1
    }
    i = 0 as libc::c_int as s16;
    while (i as libc::c_int) <
              (::std::mem::size_of::<[*mut libc::c_void; 9]>() as
                   libc::c_ulong).wrapping_div(::std::mem::size_of::<*mut libc::c_void>()
                                                   as libc::c_ulong) as s32 {
        BossGanondrof_ClearPixels16x16(gSegments[((sLimbTex_rgba16_16x16[i as
                                                                             usize]
                                                       as u32_0) <<
                                                      4 as libc::c_int >>
                                                      28 as libc::c_int) as
                                                     usize].wrapping_add(sLimbTex_rgba16_16x16[i
                                                                                                   as
                                                                                                   usize]
                                                                             as
                                                                             u32_0
                                                                             &
                                                                             0xffffff
                                                                                 as
                                                                                 libc::c_int
                                                                                 as
                                                                                 libc::c_uint).wrapping_add(0x80000000
                                                                                                                as
                                                                                                                libc::c_uint)
                                           as *mut libc::c_void as *mut s16,
                                       mask, index);
        i += 1
    }
    i = 0 as libc::c_int as s16;
    while (i as libc::c_int) <
              (::std::mem::size_of::<[*mut libc::c_void; 2]>() as
                   libc::c_ulong).wrapping_div(::std::mem::size_of::<*mut libc::c_void>()
                                                   as libc::c_ulong) as s32 {
        BossGanondrof_ClearPixels16x32(gSegments[((sLimbTex_rgba16_16x32[i as
                                                                             usize]
                                                       as u32_0) <<
                                                      4 as libc::c_int >>
                                                      28 as libc::c_int) as
                                                     usize].wrapping_add(sLimbTex_rgba16_16x32[i
                                                                                                   as
                                                                                                   usize]
                                                                             as
                                                                             u32_0
                                                                             &
                                                                             0xffffff
                                                                                 as
                                                                                 libc::c_int
                                                                                 as
                                                                                 libc::c_uint).wrapping_add(0x80000000
                                                                                                                as
                                                                                                                libc::c_uint)
                                           as *mut libc::c_void as *mut s16,
                                       mask, index);
        i += 1
    }
    BossGanondrof_ClearPixels32x16(gSegments[((gPhantomGanonLimbTex_00B380.as_mut_ptr()
                                                   as u32_0) <<
                                                  4 as libc::c_int >>
                                                  28 as libc::c_int) as
                                                 usize].wrapping_add(gPhantomGanonLimbTex_00B380.as_mut_ptr()
                                                                         as
                                                                         u32_0
                                                                         &
                                                                         0xffffff
                                                                             as
                                                                             libc::c_int
                                                                             as
                                                                             libc::c_uint).wrapping_add(0x80000000
                                                                                                            as
                                                                                                            libc::c_uint)
                                       as *mut libc::c_void as *mut s16, mask,
                                   index);
    BossGanondrof_ClearPixels16x32(gSegments[((gPhantomGanonEyeTex.as_mut_ptr()
                                                   as u32_0) <<
                                                  4 as libc::c_int >>
                                                  28 as libc::c_int) as
                                                 usize].wrapping_add(gPhantomGanonEyeTex.as_mut_ptr()
                                                                         as
                                                                         u32_0
                                                                         &
                                                                         0xffffff
                                                                             as
                                                                             libc::c_int
                                                                             as
                                                                             libc::c_uint).wrapping_add(0x80000000
                                                                                                            as
                                                                                                            libc::c_uint)
                                       as *mut libc::c_void as *mut s16, mask,
                                   index);
    i = 0 as libc::c_int as s16;
    while (i as libc::c_int) <
              (::std::mem::size_of::<[*mut libc::c_void; 2]>() as
                   libc::c_ulong).wrapping_div(::std::mem::size_of::<*mut libc::c_void>()
                                                   as libc::c_ulong) as s32 {
        BossGanondrof_ClearPixels16x16(gSegments[((sMouthTex_ci8_16x16[i as
                                                                           usize]
                                                       as u32_0) <<
                                                      4 as libc::c_int >>
                                                      28 as libc::c_int) as
                                                     usize].wrapping_add(sMouthTex_ci8_16x16[i
                                                                                                 as
                                                                                                 usize]
                                                                             as
                                                                             u32_0
                                                                             &
                                                                             0xffffff
                                                                                 as
                                                                                 libc::c_int
                                                                                 as
                                                                                 libc::c_uint).wrapping_add(0x80000000
                                                                                                                as
                                                                                                                libc::c_uint)
                                           as *mut libc::c_void as *mut s16,
                                       mask, index);
        i += 1
    };
}
#[no_mangle]
pub unsafe extern "C" fn BossGanondrof_SetColliderPos(mut pos: *mut Vec3f,
                                                      mut collider:
                                                          *mut ColliderCylinder) {
    (*collider).dim.pos.x = (*pos).x as s16;
    (*collider).dim.pos.y = (*pos).y as s16;
    (*collider).dim.pos.z = (*pos).z as s16;
}
#[no_mangle]
pub unsafe extern "C" fn BossGanondrof_Init(mut thisx: *mut Actor,
                                            mut globalCtx:
                                                *mut GlobalContext) {
    let mut pad: s32 = 0;
    let mut this: *mut BossGanondrof = thisx as *mut BossGanondrof;
    Actor_ProcessInitChain(&mut (*this).actor, sInitChain.as_mut_ptr());
    ActorShape_Init(&mut (*this).actor.shape, 0.0f32, None, 0.0f32);
    Actor_SetScale(&mut (*this).actor, 0.01f32);
    SkelAnime_Init(globalCtx, &mut (*this).skelAnime, &mut gPhantomGanonSkel,
                   &mut gPhantomGanonRideAnim, 0 as *mut Vec3s,
                   0 as *mut Vec3s, 0 as libc::c_int);
    if ((*this).actor.params as libc::c_int) < 10 as libc::c_int {
        (*this).actor.params = 1 as libc::c_int as s16;
        (*this).actor.colChkInfo.health = 30 as libc::c_int as u8_0;
        (*this).lightNode =
            LightContext_InsertLight(globalCtx, &mut (*globalCtx).lightCtx,
                                     &mut (*this).lightInfo);
        Lights_PointNoGlowSetInfo(&mut (*this).lightInfo,
                                  (*this).actor.world.pos.x as s16,
                                  (*this).actor.world.pos.y as s16,
                                  (*this).actor.world.pos.z as s16,
                                  255 as libc::c_int as u8_0,
                                  255 as libc::c_int as u8_0,
                                  255 as libc::c_int as u8_0,
                                  255 as libc::c_int as s16);
        BossGanondrof_SetupIntro(this, globalCtx);
    } else { BossGanondrof_SetupPaintings(this); }
    Collider_InitCylinder(globalCtx, &mut (*this).colliderBody);
    Collider_InitCylinder(globalCtx, &mut (*this).colliderSpear);
    Collider_SetCylinder(globalCtx, &mut (*this).colliderBody,
                         &mut (*this).actor, &mut sCylinderInitBody);
    Collider_SetCylinder(globalCtx, &mut (*this).colliderSpear,
                         &mut (*this).actor, &mut sCylinderInitSpear);
    (*this).actor.flags &=
        !((1 as libc::c_int) << 0 as libc::c_int) as libc::c_uint;
    if Flags_GetClear(globalCtx, (*globalCtx).roomCtx.curRoom.num as s32) != 0
       {
        Actor_Kill(&mut (*this).actor);
        Actor_Spawn(&mut (*globalCtx).actorCtx, globalCtx,
                    ACTOR_DOOR_WARP1 as libc::c_int as s16, 14.0f32, -33.0f32,
                    -3315.0f32, 0 as libc::c_int as s16,
                    0 as libc::c_int as s16, 0 as libc::c_int as s16,
                    WARP_DUNGEON_ADULT as libc::c_int as s16);
        Actor_Spawn(&mut (*globalCtx).actorCtx, globalCtx,
                    ACTOR_ITEM_B_HEART as libc::c_int as s16,
                    200.0f32 + 14.0f32, -33.0f32, -3315.0f32,
                    0 as libc::c_int as s16, 0 as libc::c_int as s16,
                    0 as libc::c_int as s16, 0 as libc::c_int as s16);
    } else {
        Actor_SpawnAsChild(&mut (*globalCtx).actorCtx, &mut (*this).actor,
                           globalCtx, ACTOR_EN_FHG as libc::c_int as s16,
                           (*this).actor.world.pos.x,
                           (*this).actor.world.pos.y,
                           (*this).actor.world.pos.z, 0 as libc::c_int as s16,
                           0 as libc::c_int as s16, 0 as libc::c_int as s16,
                           (*this).actor.params);
    };
}
#[no_mangle]
pub unsafe extern "C" fn BossGanondrof_Destroy(mut thisx: *mut Actor,
                                               mut globalCtx:
                                                   *mut GlobalContext) {
    let mut pad: s32 = 0;
    let mut this: *mut BossGanondrof = thisx as *mut BossGanondrof;
    osSyncPrintf(b"DT1\n\x00" as *const u8 as *const libc::c_char);
    SkelAnime_Free(&mut (*this).skelAnime, globalCtx);
    Collider_DestroyCylinder(globalCtx, &mut (*this).colliderBody);
    Collider_DestroyCylinder(globalCtx, &mut (*this).colliderSpear);
    if (*this).actor.params as libc::c_int == 1 as libc::c_int {
        LightContext_RemoveLight(globalCtx, &mut (*globalCtx).lightCtx,
                                 (*this).lightNode);
    }
    osSyncPrintf(b"DT2\n\x00" as *const u8 as *const libc::c_char);
}
#[no_mangle]
pub unsafe extern "C" fn BossGanondrof_SetupIntro(mut this:
                                                      *mut BossGanondrof,
                                                  mut globalCtx:
                                                      *mut GlobalContext) {
    Animation_PlayLoop(&mut (*this).skelAnime,
                       &mut gPhantomGanonRidePoseAnim);
    (*this).actionFunc =
        Some(BossGanondrof_Intro as
                 unsafe extern "C" fn(_: *mut BossGanondrof,
                                      _: *mut GlobalContext) -> ());
    (*this).work[GND_MASK_OFF as libc::c_int as usize] =
        1 as libc::c_int as s16;
}
#[no_mangle]
pub unsafe extern "C" fn BossGanondrof_Intro(mut this: *mut BossGanondrof,
                                             mut globalCtx:
                                                 *mut GlobalContext) {
    let mut i: s16 = 0;
    let mut pad: s32 = 0;
    let mut horse: *mut EnfHG = (*this).actor.child as *mut EnfHG;
    SkelAnime_Update(&mut (*this).skelAnime);
    (*this).actor.world.pos = (*horse).actor.world.pos;
    (*this).actor.world.rot.y = (*horse).actor.world.rot.y;
    (*this).actor.shape.rot.y = (*this).actor.world.rot.y;
    osSyncPrintf(b"SW %d------------------------------------------------\n\x00"
                     as *const u8 as *const libc::c_char,
                 (*horse).bossGndSignal as libc::c_int);
    if (*this).timers[1 as libc::c_int as usize] as libc::c_int !=
           0 as libc::c_int &&
           ((*this).timers[1 as libc::c_int as usize] as libc::c_int) <
               25 as libc::c_int {
        let mut pos: Vec3f = Vec3f{x: 0., y: 0., z: 0.,};
        let mut vel: Vec3f =
            { let mut init = Vec3f{x: 0.0f32, y: 0.0f32, z: 0.0f32,}; init };
        let mut accel: Vec3f =
            { let mut init = Vec3f{x: 0.0f32, y: 0.0f32, z: 0.0f32,}; init };
        pos.x =
            (*this).bodyPartsPos[14 as libc::c_int as usize].x +
                Rand_CenteredFloat(10.0f32);
        pos.y =
            (*this).bodyPartsPos[14 as libc::c_int as usize].y +
                Rand_ZeroFloat(-5.0f32);
        pos.z =
            (*this).bodyPartsPos[14 as libc::c_int as usize].z +
                Rand_CenteredFloat(10.0f32) + 5.0f32;
        accel.y = 0.03f32;
        EffectSsKFire_Spawn(globalCtx, &mut pos, &mut vel, &mut accel,
                            (Rand_ZeroFloat(10.0f32) as s16 as libc::c_int +
                                 5 as libc::c_int) as s16,
                            0 as libc::c_int as u8_0);
    }
    if (*this).timers[1 as libc::c_int as usize] as libc::c_int ==
           20 as libc::c_int {
        (*this).work[GND_MASK_OFF as libc::c_int as usize] =
            0 as libc::c_int as s16
    }
    if (*this).timers[1 as libc::c_int as usize] as libc::c_int ==
           30 as libc::c_int {
        func_80078914(&mut sAudioVec, 0x38a1 as libc::c_int as u16_0);
    }
    if (*horse).bossGndSignal as libc::c_int == FHG_LIGHTNING as libc::c_int {
        Animation_Change(&mut (*this).skelAnime, &mut gPhantomGanonMaskOnAnim,
                         0.5f32, 0.0f32,
                         Animation_GetLastFrame(&mut gPhantomGanonMaskOnAnim
                                                    as *mut AnimationHeader as
                                                    *mut libc::c_void) as
                             f32_0,
                         ANIMMODE_ONCE_INTERP as libc::c_int as u8_0, 0.0f32);
        (*this).timers[1 as libc::c_int as usize] = 40 as libc::c_int as s16
    }
    if (*horse).bossGndSignal as libc::c_int == FHG_REAR as libc::c_int {
        Animation_MorphToPlayOnce(&mut (*this).skelAnime,
                                  &mut gPhantomGanonHorseRearingAnim,
                                  -3.0f32);
    }
    if (*horse).bossGndSignal as libc::c_int == FHG_RIDE as libc::c_int {
        Animation_MorphToLoop(&mut (*this).skelAnime,
                              &mut gPhantomGanonRidePoseAnim, -13.0f32);
    }
    if (*horse).bossGndSignal as libc::c_int == FHG_SPUR as libc::c_int {
        let mut horseTemp: *mut EnfHG = 0 as *mut EnfHG;
        Animation_MorphToPlayOnce(&mut (*this).skelAnime,
                                  &mut gPhantomGanonRideSpearRaiseAnim,
                                  -7.0f32);
        horseTemp = (*this).actor.child as *mut EnfHG;
        Actor_SpawnAsChild(&mut (*globalCtx).actorCtx, &mut (*this).actor,
                           globalCtx, ACTOR_EN_FHG_FIRE as libc::c_int as s16,
                           (*this).spearTip.x, (*this).spearTip.y,
                           (*this).spearTip.z, 50 as libc::c_int as s16,
                           FHGFIRE_LIGHT_GREEN as libc::c_int as s16,
                           0 as libc::c_int as s16,
                           FHGFIRE_SPEAR_LIGHT as libc::c_int as s16);
        (*this).actor.child = &mut (*horseTemp).actor
    }
    if (*horse).bossGndSignal as libc::c_int == FHG_FINISH as libc::c_int {
        Animation_MorphToPlayOnce(&mut (*this).skelAnime,
                                  &mut gPhantomGanonRideSpearResetAnim,
                                  -5.0f32);
    }
    match (*this).work[GND_EYE_STATE as libc::c_int as usize] as libc::c_int {
        1 => {
            (*this).fwork[GND_EYE_ALPHA as libc::c_int as usize] += 40.0f32;
            if (*this).fwork[GND_EYE_ALPHA as libc::c_int as usize] >=
                   255.0f32 {
                (*this).fwork[GND_EYE_ALPHA as libc::c_int as usize] =
                    255.0f32
            }
        }
        2 => {
            (*this).fwork[GND_EYE_BRIGHTNESS as libc::c_int as usize] +=
                20.0f32;
            if (*this).fwork[GND_EYE_BRIGHTNESS as libc::c_int as usize] >
                   255.0f32 {
                (*this).fwork[GND_EYE_BRIGHTNESS as libc::c_int as usize] =
                    255.0f32
            }
        }
        _ => { }
    }
    (*this).armRotY =
        Math_SinS(((*this).work[GND_VARIANCE_TIMER as libc::c_int as usize] as
                       libc::c_int * 0x6e8 as libc::c_int) as s16) *
            0 as libc::c_int as libc::c_float;
    (*this).armRotZ =
        Math_CosS(((*this).work[GND_VARIANCE_TIMER as libc::c_int as usize] as
                       libc::c_int * 0x8dc as libc::c_int) as s16) * 300.0f32;
    i = 0 as libc::c_int as s16;
    while (i as libc::c_int) < 30 as libc::c_int {
        (*this).rideRotY[i as usize] =
            Math_SinS(((*this).work[GND_VARIANCE_TIMER as libc::c_int as
                                        usize] as libc::c_int *
                           (i as libc::c_int * 50 as libc::c_int +
                                0x7b0 as libc::c_int)) as s16) * 100.0f32;
        (*this).rideRotZ[i as usize] =
            Math_CosS(((*this).work[GND_VARIANCE_TIMER as libc::c_int as
                                        usize] as libc::c_int *
                           (i as libc::c_int * 50 as libc::c_int +
                                0x8dc as libc::c_int)) as s16) * 100.0f32;
        i += 1
    }
    if (*horse).bossGndSignal as libc::c_int == FHG_START_FIGHT as libc::c_int
       {
        BossGanondrof_SetupPaintings(this);
        i = 0 as libc::c_int as s16;
        while (i as libc::c_int) < 30 as libc::c_int {
            (*this).rideRotZ[i as usize] = 0.0f32;
            (*this).rideRotY[i as usize] = (*this).rideRotZ[i as usize];
            i += 1
        }
    }
    (*horse).bossGndSignal = FHG_NO_SIGNAL as libc::c_int as u8_0;
}
#[no_mangle]
pub unsafe extern "C" fn BossGanondrof_SetupPaintings(mut this:
                                                          *mut BossGanondrof) {
    Animation_MorphToLoop(&mut (*this).skelAnime, &mut gPhantomGanonRideAnim,
                          -5.0f32);
    (*this).actionFunc =
        Some(BossGanondrof_Paintings as
                 unsafe extern "C" fn(_: *mut BossGanondrof,
                                      _: *mut GlobalContext) -> ());
}
#[no_mangle]
pub unsafe extern "C" fn BossGanondrof_Paintings(mut this: *mut BossGanondrof,
                                                 mut globalCtx:
                                                     *mut GlobalContext) {
    let mut horse: *mut EnfHG = (*this).actor.child as *mut EnfHG;
    osSyncPrintf(b"RUN 1\n\x00" as *const u8 as *const libc::c_char);
    SkelAnime_Update(&mut (*this).skelAnime);
    osSyncPrintf(b"RUN 2\n\x00" as *const u8 as *const libc::c_char);
    if (*horse).bossGndSignal as libc::c_int == FHG_RAISE_SPEAR as libc::c_int
       {
        let mut horseTemp: *mut EnfHG = 0 as *mut EnfHG;
        Animation_MorphToPlayOnce(&mut (*this).skelAnime,
                                  &mut gPhantomGanonRideSpearRaiseAnim,
                                  -2.0f32);
        (*this).actor.flags |=
            ((1 as libc::c_int) << 0 as libc::c_int) as libc::c_uint;
        horseTemp = (*this).actor.child as *mut EnfHG;
        Actor_SpawnAsChild(&mut (*globalCtx).actorCtx, &mut (*this).actor,
                           globalCtx, ACTOR_EN_FHG_FIRE as libc::c_int as s16,
                           (*this).spearTip.x, (*this).spearTip.y,
                           (*this).spearTip.z, 30 as libc::c_int as s16,
                           FHGFIRE_LIGHT_GREEN as libc::c_int as s16,
                           0 as libc::c_int as s16,
                           FHGFIRE_SPEAR_LIGHT as libc::c_int as s16);
        (*this).actor.child = &mut (*horseTemp).actor
    } else if (*horse).bossGndSignal as libc::c_int ==
                  FHG_LIGHTNING as libc::c_int {
        Animation_MorphToPlayOnce(&mut (*this).skelAnime,
                                  &mut gPhantomGanonRideSpearStrikeAnim,
                                  -2.0f32);
    } else if (*horse).bossGndSignal as libc::c_int ==
                  FHG_RESET as libc::c_int {
        Animation_MorphToPlayOnce(&mut (*this).skelAnime,
                                  &mut gPhantomGanonRideSpearResetAnim,
                                  -2.0f32);
    } else if (*horse).bossGndSignal as libc::c_int == FHG_RIDE as libc::c_int
     {
        Animation_MorphToLoop(&mut (*this).skelAnime,
                              &mut gPhantomGanonRideAnim, -2.0f32);
        (*this).actor.flags &=
            !((1 as libc::c_int) << 0 as libc::c_int) as libc::c_uint
    }
    osSyncPrintf(b"RUN 3\n\x00" as *const u8 as *const libc::c_char);
    (*this).actor.world.pos = (*horse).actor.world.pos;
    (*this).actor.world.pos.y = (*horse).actor.world.pos.y;
    (*this).actor.world.rot.y = (*horse).actor.world.rot.y;
    (*this).actor.shape.rot.y = (*this).actor.world.rot.y;
    if (*this).flyMode as libc::c_int != GND_FLY_PAINTING as libc::c_int {
        BossGanondrof_SetupNeutral(this, -20.0f32);
        (*this).timers[0 as libc::c_int as usize] = 100 as libc::c_int as s16;
        (*this).colliderBody.dim.radius = 20 as libc::c_int as s16;
        (*this).colliderBody.dim.height = 60 as libc::c_int as s16;
        (*this).colliderBody.dim.yShift = -(33 as libc::c_int) as s16;
        Audio_PlayActorSound2(&mut (*this).actor,
                              0x38b0 as libc::c_int as u16_0);
        (*this).actor.naviEnemyId = 0x1a as libc::c_int as u8_0
    } else {
        (*horse).bossGndSignal = FHG_NO_SIGNAL as libc::c_int as u8_0;
        (*this).actor.scale.x = (*horse).actor.scale.x / 1.15f32;
        (*this).actor.scale.y = (*horse).actor.scale.y / 1.15f32;
        (*this).actor.scale.z = (*horse).actor.scale.z / 1.15f32;
        osSyncPrintf(b"RUN 4\n\x00" as *const u8 as *const libc::c_char);
    };
}
#[no_mangle]
pub unsafe extern "C" fn BossGanondrof_SetupNeutral(mut this:
                                                        *mut BossGanondrof,
                                                    mut arg1: f32_0) {
    Animation_MorphToLoop(&mut (*this).skelAnime,
                          &mut gPhantomGanonNeutralAnim, arg1);
    (*this).actionFunc =
        Some(BossGanondrof_Neutral as
                 unsafe extern "C" fn(_: *mut BossGanondrof,
                                      _: *mut GlobalContext) -> ());
    (*this).actor.flags |=
        ((1 as libc::c_int) << 0 as libc::c_int) as libc::c_uint;
    (*this).fwork[GND_FLOAT_SPEED as libc::c_int as usize] = 0.0f32;
    (*this).timers[0 as libc::c_int as usize] =
        ((Rand_ZeroOne() * 64.0f32) as s16 as libc::c_int + 30 as libc::c_int)
            as s16;
}
#[no_mangle]
pub unsafe extern "C" fn BossGanondrof_Neutral(mut this: *mut BossGanondrof,
                                               mut globalCtx:
                                                   *mut GlobalContext) {
    let mut targetX: f32_0 = 0.;
    let mut targetY: f32_0 = 0.;
    let mut targetZ: f32_0 = 0.;
    let mut player: *mut Player =
        (*globalCtx).actorCtx.actorLists[ACTORCAT_PLAYER as libc::c_int as
                                             usize].head as *mut Player;
    let mut playerx: *mut Actor = &mut (*player).actor;
    let mut thisx: *mut Actor = &mut (*this).actor;
    let mut rand01: f32_0 = 0.;
    let mut i: s16 = 0;
    SkelAnime_Update(&mut (*this).skelAnime);
    match (*this).flyMode as libc::c_int {
        1 => {
            if (*this).timers[0 as libc::c_int as usize] as libc::c_int ==
                   0 as libc::c_int {
                (*this).timers[0 as libc::c_int as usize] =
                    ((Rand_ZeroOne() * 64.0f32) as s16 as libc::c_int +
                         30 as libc::c_int) as s16;
                rand01 = Rand_ZeroOne();
                if ((*thisx).colChkInfo.health as libc::c_int) <
                       5 as libc::c_int {
                    if rand01 < 0.25f32 {
                        BossGanondrof_SetupThrow(this, globalCtx);
                    } else if rand01 >= 0.8f32 {
                        (*this).flyMode =
                            GND_FLY_CHARGE as libc::c_int as u8_0;
                        (*this).timers[0 as libc::c_int as usize] =
                            60 as libc::c_int as s16;
                        (*this).fwork[GND_FLOAT_SPEED as libc::c_int as usize]
                            = 0.0f32;
                        Audio_PlayActorSound2(thisx,
                                              0x38b0 as libc::c_int as u16_0);
                    } else {
                        (*this).flyMode =
                            GND_FLY_VOLLEY as libc::c_int as u8_0;
                        (*this).timers[0 as libc::c_int as usize] =
                            60 as libc::c_int as s16;
                        (*this).fwork[GND_FLOAT_SPEED as libc::c_int as usize]
                            = 0.0f32;
                        Audio_PlayActorSound2(thisx,
                                              0x38b0 as libc::c_int as u16_0);
                    }
                } else if rand01 < 0.5f32 ||
                              ((*this).work[GND_THROW_COUNT as libc::c_int as
                                                usize] as libc::c_int) <
                                  5 as libc::c_int {
                    BossGanondrof_SetupThrow(this, globalCtx);
                } else {
                    (*this).flyMode = GND_FLY_VOLLEY as libc::c_int as u8_0;
                    (*this).timers[0 as libc::c_int as usize] =
                        60 as libc::c_int as s16;
                    (*this).fwork[GND_FLOAT_SPEED as libc::c_int as usize] =
                        0.0f32;
                    Audio_PlayActorSound2(thisx,
                                          0x38b0 as libc::c_int as u16_0);
                }
            }
            if (*this).timers[1 as libc::c_int as usize] as libc::c_int !=
                   0 as libc::c_int {
                targetX = 14.0f32;
                targetZ = -3315.0f32
            } else {
                targetX =
                    (*playerx).world.pos.x +
                        180.0f32 * Math_SinS((*playerx).shape.rot.y);
                targetZ =
                    (*playerx).world.pos.z +
                        180.0f32 * Math_CosS((*playerx).shape.rot.y);
                if sqrtf((targetX - 14.0f32) * (targetX - 14.0f32) +
                             (targetZ - -3315.0f32) * (targetZ - -3315.0f32))
                       > 280.0f32 {
                    (*this).timers[1 as libc::c_int as usize] =
                        50 as libc::c_int as s16;
                    (*this).fwork[GND_FLOAT_SPEED as libc::c_int as usize] =
                        0.0f32
                }
            }
            targetY = (*playerx).world.pos.y + 100.0f32 + 0.0f32;
            targetX +=
                Math_SinS(((*this).work[GND_VARIANCE_TIMER as libc::c_int as
                                            usize] as libc::c_int *
                               0x500 as libc::c_int) as s16) * 100.0f32;
            targetZ +=
                Math_CosS(((*this).work[GND_VARIANCE_TIMER as libc::c_int as
                                            usize] as libc::c_int *
                               0x700 as libc::c_int) as s16) * 100.0f32
        }
        2 => {
            targetX = 14.0f32 - 14.0f32;
            targetZ = -3315.0f32 + 265.0f32;
            targetY = (*playerx).world.pos.y + 100.0f32 + 100.0f32;
            targetX +=
                Math_SinS(((*this).work[GND_VARIANCE_TIMER as libc::c_int as
                                            usize] as libc::c_int *
                               0x500 as libc::c_int) as s16) * 100.0f32;
            targetZ +=
                Math_CosS(((*this).work[GND_VARIANCE_TIMER as libc::c_int as
                                            usize] as libc::c_int *
                               0x700 as libc::c_int) as s16) * 100.0f32;
            if (*this).timers[0 as libc::c_int as usize] as libc::c_int ==
                   0 as libc::c_int {
                (*this).flyMode = GND_FLY_RETURN as libc::c_int as u8_0;
                (*this).returnSuccess = 0 as libc::c_int as u8_0;
                BossGanondrof_SetupThrow(this, globalCtx);
                (*this).timers[0 as libc::c_int as usize] =
                    80 as libc::c_int as s16
            }
        }
        3 => {
            targetX = 14.0f32 - 14.0f32;
            targetZ = -3315.0f32 + 265.0f32;
            targetY = (*playerx).world.pos.y + 100.0f32 + 100.0f32;
            targetX +=
                Math_SinS(((*this).work[GND_VARIANCE_TIMER as libc::c_int as
                                            usize] as libc::c_int *
                               0x500 as libc::c_int) as s16) * 50.0f32;
            targetZ +=
                Math_CosS(((*this).work[GND_VARIANCE_TIMER as libc::c_int as
                                            usize] as libc::c_int *
                               0x700 as libc::c_int) as s16) * 50.0f32;
            if (*this).returnSuccess != 0 {
                (*this).returnSuccess = 0 as libc::c_int as u8_0;
                BossGanondrof_SetupReturn(this, globalCtx);
                (*this).timers[0 as libc::c_int as usize] =
                    80 as libc::c_int as s16
            }
            if (*this).timers[0 as libc::c_int as usize] as libc::c_int ==
                   0 as libc::c_int {
                (*this).flyMode = GND_FLY_NEUTRAL as libc::c_int as u8_0
            }
        }
        4 => {
            targetX = 14.0f32 - 14.0f32;
            targetZ = -3315.0f32 + 215.0f32;
            targetY = (*playerx).world.pos.y + 100.0f32 + 50.0f32;
            targetX +=
                Math_SinS(((*this).work[GND_VARIANCE_TIMER as libc::c_int as
                                            usize] as libc::c_int *
                               0x500 as libc::c_int) as s16) * 100.0f32;
            targetZ +=
                Math_CosS(((*this).work[GND_VARIANCE_TIMER as libc::c_int as
                                            usize] as libc::c_int *
                               0x700 as libc::c_int) as s16) * 100.0f32;
            if (*this).timers[0 as libc::c_int as usize] as libc::c_int ==
                   0 as libc::c_int {
                BossGanondrof_SetupCharge(this, globalCtx);
            }
        }
        _ => { }
    }
    Math_ApproachF(&mut (*thisx).world.pos.x, targetX, 0.05f32,
                   (*this).fwork[GND_FLOAT_SPEED as libc::c_int as usize]);
    if (*this).timers[2 as libc::c_int as usize] as libc::c_int !=
           0 as libc::c_int {
        Math_ApproachF(&mut (*thisx).world.pos.y, targetY + 100.0f32, 0.1f32,
                       50.0f32);
    } else {
        Math_ApproachF(&mut (*thisx).world.pos.y, targetY, 0.05f32, 10.0f32);
    }
    Math_ApproachF(&mut (*thisx).world.pos.z, targetZ, 0.05f32,
                   (*this).fwork[GND_FLOAT_SPEED as libc::c_int as usize]);
    Math_ApproachF(&mut *(*this).fwork.as_mut_ptr().offset(GND_FLOAT_SPEED as
                                                               libc::c_int as
                                                               isize),
                   50.0f32, 1.0f32, 0.5f32);
    (*thisx).velocity.x = (*thisx).world.pos.x - (*thisx).prevPos.x;
    (*thisx).velocity.z = (*thisx).world.pos.z - (*thisx).prevPos.z;
    (*thisx).world.pos.y +=
        2.0f32 *
            Math_SinS(((*this).work[GND_VARIANCE_TIMER as libc::c_int as
                                        usize] as libc::c_int *
                           1500 as libc::c_int) as s16);
    Math_ApproachS(&mut (*thisx).shape.rot.y, (*thisx).yawTowardsPlayer,
                   5 as libc::c_int as s16, 0xbb8 as libc::c_int as s16);
    if (*this).work[GND_VARIANCE_TIMER as libc::c_int as usize] as libc::c_int
           & 1 as libc::c_int == 0 as libc::c_int {
        let mut pos: Vec3f = Vec3f{x: 0., y: 0., z: 0.,};
        let mut vel: Vec3f =
            { let mut init = Vec3f{x: 0.0f32, y: 0.0f32, z: 0.0f32,}; init };
        let mut accel: Vec3f =
            { let mut init = Vec3f{x: 0.0f32, y: 0.0f32, z: 0.0f32,}; init };
        i = 0 as libc::c_int as s16;
        while (i as libc::c_int) < 3 as libc::c_int {
            pos.x = Rand_CenteredFloat(20.0f32) + (*this).spearTip.x;
            pos.y = Rand_CenteredFloat(20.0f32) + (*this).spearTip.y;
            pos.z = Rand_CenteredFloat(20.0f32) + (*this).spearTip.z;
            accel.y = -0.08f32;
            EffectSsFhgFlash_SpawnLightBall(globalCtx, &mut pos, &mut vel,
                                            &mut accel,
                                            ((Rand_ZeroOne() * 80.0f32) as s16
                                                 as libc::c_int +
                                                 150 as libc::c_int) as s16,
                                            FHGFLASH_LIGHTBALL_GREEN as
                                                libc::c_int as u8_0);
            i += 1
        }
    }
    if (*player).unk_A73 as libc::c_int != 0 as libc::c_int {
        BossGanondrof_SetupBlock(this, globalCtx);
    }
    Audio_PlayActorSound2(thisx,
                          (0x38a4 as libc::c_int - 0x800 as libc::c_int) as
                              u16_0);
}
#[no_mangle]
pub unsafe extern "C" fn BossGanondrof_SetupThrow(mut this:
                                                      *mut BossGanondrof,
                                                  mut globalCtx:
                                                      *mut GlobalContext) {
    let mut horseTemp: *mut EnfHG = 0 as *mut EnfHG;
    let mut lightTime: s16 = 0;
    (*this).fwork[GND_END_FRAME as libc::c_int as usize] =
        Animation_GetLastFrame(&mut gPhantomGanonThrowAnim as
                                   *mut AnimationHeader as *mut libc::c_void)
            as f32_0;
    Animation_MorphToPlayOnce(&mut (*this).skelAnime,
                              &mut gPhantomGanonThrowAnim, -5.0f32);
    (*this).actionFunc =
        Some(BossGanondrof_Throw as
                 unsafe extern "C" fn(_: *mut BossGanondrof,
                                      _: *mut GlobalContext) -> ());
    if Rand_ZeroOne() <= 0.1f32 &&
           (*this).work[GND_THROW_COUNT as libc::c_int as usize] as
               libc::c_int >= 10 as libc::c_int &&
           (*this).flyMode as libc::c_int == GND_FLY_NEUTRAL as libc::c_int {
        (*this).work[GND_ACTION_STATE as libc::c_int as usize] =
            THROW_SLOW as libc::c_int as s16;
        (*this).work[GND_THROW_FRAME as libc::c_int as usize] =
            1000 as libc::c_int as s16;
        lightTime = 32 as libc::c_int as s16
    } else {
        (*this).work[GND_ACTION_STATE as libc::c_int as usize] =
            THROW_NORMAL as libc::c_int as s16;
        (*this).work[GND_THROW_FRAME as libc::c_int as usize] =
            25 as libc::c_int as s16;
        lightTime = 25 as libc::c_int as s16
    }
    horseTemp = (*this).actor.child as *mut EnfHG;
    Actor_SpawnAsChild(&mut (*globalCtx).actorCtx, &mut (*this).actor,
                       globalCtx, ACTOR_EN_FHG_FIRE as libc::c_int as s16,
                       (*this).spearTip.x, (*this).spearTip.y,
                       (*this).spearTip.z, lightTime,
                       FHGFIRE_LIGHT_GREEN as libc::c_int as s16,
                       0 as libc::c_int as s16,
                       FHGFIRE_SPEAR_LIGHT as libc::c_int as s16);
    (*this).actor.child = &mut (*horseTemp).actor;
    (*this).work[GND_THROW_COUNT as libc::c_int as usize] += 1;
    Audio_PlayActorSound2(&mut (*this).actor, 0x38aa as libc::c_int as u16_0);
}
#[no_mangle]
pub unsafe extern "C" fn BossGanondrof_Throw(mut this: *mut BossGanondrof,
                                             mut globalCtx:
                                                 *mut GlobalContext) {
    SkelAnime_Update(&mut (*this).skelAnime);
    osSyncPrintf(b"this->fwork[GND_END_FRAME] = %d\n\x00" as *const u8 as
                     *const libc::c_char,
                 (*this).fwork[GND_END_FRAME as libc::c_int as usize] as s16
                     as libc::c_int);
    osSyncPrintf(b"this->work[GND_SHOT_FRAME] = %d\n\x00" as *const u8 as
                     *const libc::c_char,
                 (*this).work[GND_THROW_FRAME as libc::c_int as usize] as
                     libc::c_int);
    if Animation_OnFrame(&mut (*this).skelAnime,
                         (*this).fwork[GND_END_FRAME as libc::c_int as usize])
           != 0 {
        BossGanondrof_SetupNeutral(this, -6.0f32);
    }
    if (*this).work[GND_ACTION_STATE as libc::c_int as usize] as libc::c_int
           != THROW_NORMAL as libc::c_int &&
           Animation_OnFrame(&mut (*this).skelAnime, 21.0f32) != 0 {
        (*this).fwork[GND_END_FRAME as libc::c_int as usize] =
            Animation_GetLastFrame(&mut gPhantomGanonThrowEndAnim as
                                       *mut AnimationHeader as
                                       *mut libc::c_void) as f32_0;
        Animation_MorphToPlayOnce(&mut (*this).skelAnime,
                                  &mut gPhantomGanonThrowEndAnim, 0.0f32);
        (*this).work[GND_THROW_FRAME as libc::c_int as usize] =
            10 as libc::c_int as s16
    }
    if Animation_OnFrame(&mut (*this).skelAnime,
                         (*this).work[GND_THROW_FRAME as libc::c_int as usize]
                             as f32_0) != 0 {
        if (*this).flyMode as libc::c_int <= GND_FLY_NEUTRAL as libc::c_int {
            Audio_PlayActorSound2(&mut (*this).actor,
                                  0x38a6 as libc::c_int as u16_0);
        } else {
            Audio_PlayActorSound2(&mut (*this).actor,
                                  0x38a5 as libc::c_int as u16_0);
        }
        Audio_PlayActorSound2(&mut (*this).actor,
                              0x38b2 as libc::c_int as u16_0);
    }
    if Animation_OnFrame(&mut (*this).skelAnime,
                         (*this).work[GND_THROW_FRAME as libc::c_int as usize]
                             as f32_0) != 0 {
        let mut horseTemp: *mut EnfHG = (*this).actor.child as *mut EnfHG;
        Actor_SpawnAsChild(&mut (*globalCtx).actorCtx, &mut (*this).actor,
                           globalCtx, ACTOR_EN_FHG_FIRE as libc::c_int as s16,
                           (*this).spearTip.x, (*this).spearTip.y,
                           (*this).spearTip.z,
                           (*this).work[GND_ACTION_STATE as libc::c_int as
                                            usize], 0 as libc::c_int as s16,
                           0 as libc::c_int as s16,
                           FHGFIRE_ENERGY_BALL as libc::c_int as s16);
        (*this).actor.child = &mut (*horseTemp).actor
    }
    Math_ApproachS(&mut (*this).actor.shape.rot.y,
                   (*this).actor.yawTowardsPlayer, 5 as libc::c_int as s16,
                   0x7d0 as libc::c_int as s16);
    (*this).actor.world.pos.x += (*this).actor.velocity.x;
    (*this).actor.world.pos.z += (*this).actor.velocity.z;
    Math_ApproachZeroF(&mut (*this).actor.velocity.x, 1.0f32, 0.5f32);
    Math_ApproachZeroF(&mut (*this).actor.velocity.z, 1.0f32, 0.5f32);
    (*this).actor.world.pos.y +=
        2.0f32 *
            Math_SinS(((*this).work[GND_VARIANCE_TIMER as libc::c_int as
                                        usize] as libc::c_int *
                           1500 as libc::c_int) as s16);
}
#[no_mangle]
pub unsafe extern "C" fn BossGanondrof_SetupReturn(mut this:
                                                       *mut BossGanondrof,
                                                   mut globalCtx:
                                                       *mut GlobalContext) {
    static mut returnAnim: [*mut AnimationHeader; 2] =
        unsafe {
            [&gPhantomGanonReturn1Anim as *const AnimationHeader as
                 *mut AnimationHeader,
             &gPhantomGanonReturn2Anim as *const AnimationHeader as
                 *mut AnimationHeader]
        };
    let mut rand: s16 = (Rand_ZeroOne() * 1.99f32) as s16;
    (*this).fwork[GND_END_FRAME as libc::c_int as usize] =
        Animation_GetLastFrame(returnAnim[rand as usize] as *mut libc::c_void)
            as f32_0;
    Animation_MorphToPlayOnce(&mut (*this).skelAnime,
                              returnAnim[rand as usize], 0.0f32);
    (*this).actionFunc =
        Some(BossGanondrof_Return as
                 unsafe extern "C" fn(_: *mut BossGanondrof,
                                      _: *mut GlobalContext) -> ());
}
#[no_mangle]
pub unsafe extern "C" fn BossGanondrof_Return(mut this: *mut BossGanondrof,
                                              mut globalCtx:
                                                  *mut GlobalContext) {
    SkelAnime_Update(&mut (*this).skelAnime);
    if Animation_OnFrame(&mut (*this).skelAnime, 5.0f32) != 0 {
        Audio_PlayActorSound2(&mut (*this).actor,
                              0x38b2 as libc::c_int as u16_0);
        osSyncPrintf(b"VOISE               2  !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!\n\x00"
                         as *const u8 as *const libc::c_char);
        osSyncPrintf(b"VOISE               2  !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!\n\x00"
                         as *const u8 as *const libc::c_char);
    }
    if Animation_OnFrame(&mut (*this).skelAnime,
                         (*this).fwork[GND_END_FRAME as libc::c_int as usize])
           != 0 {
        BossGanondrof_SetupNeutral(this, 0.0f32);
    }
    (*this).actor.world.pos.x += (*this).actor.velocity.x;
    (*this).actor.world.pos.z += (*this).actor.velocity.z;
    Math_ApproachZeroF(&mut (*this).actor.velocity.x, 1.0f32, 0.5f32);
    Math_ApproachZeroF(&mut (*this).actor.velocity.z, 1.0f32, 0.5f32);
    (*this).actor.world.pos.y +=
        2.0f32 *
            Math_SinS(((*this).work[GND_VARIANCE_TIMER as libc::c_int as
                                        usize] as libc::c_int *
                           1500 as libc::c_int) as s16);
    if (*this).returnSuccess != 0 {
        (*this).returnSuccess = 0 as libc::c_int as u8_0;
        BossGanondrof_SetupReturn(this, globalCtx);
        (*this).timers[0 as libc::c_int as usize] = 80 as libc::c_int as s16
    };
}
#[no_mangle]
pub unsafe extern "C" fn BossGanondrof_SetupStunned(mut this:
                                                        *mut BossGanondrof,
                                                    mut globalCtx:
                                                        *mut GlobalContext) {
    if (*this).actionFunc !=
           Some(BossGanondrof_Stunned as
                    unsafe extern "C" fn(_: *mut BossGanondrof,
                                         _: *mut GlobalContext) -> ()) {
        (*this).fwork[GND_END_FRAME as libc::c_int as usize] =
            Animation_GetLastFrame(&mut gPhantomGanonAirDamageAnim as
                                       *mut AnimationHeader as
                                       *mut libc::c_void) as f32_0;
        Animation_MorphToLoop(&mut (*this).skelAnime,
                              &mut gPhantomGanonAirDamageAnim, 0.0f32);
        (*this).timers[0 as libc::c_int as usize] = 50 as libc::c_int as s16;
        (*this).shockTimer = 60 as libc::c_int as u8_0
    } else {
        (*this).fwork[GND_END_FRAME as libc::c_int as usize] =
            Animation_GetLastFrame(&mut gPhantomGanonGroundDamageAnim as
                                       *mut AnimationHeader as
                                       *mut libc::c_void) as f32_0;
        Animation_MorphToLoop(&mut (*this).skelAnime,
                              &mut gPhantomGanonGroundDamageAnim, 0.0f32);
    }
    (*this).actionFunc =
        Some(BossGanondrof_Stunned as
                 unsafe extern "C" fn(_: *mut BossGanondrof,
                                      _: *mut GlobalContext) -> ());
    (*this).work[GND_ACTION_STATE as libc::c_int as usize] =
        STUNNED_FALL as libc::c_int as s16;
    (*this).actor.velocity.x = 0.0f32;
    (*this).actor.velocity.z = 0.0f32;
}
#[no_mangle]
pub unsafe extern "C" fn BossGanondrof_Stunned(mut this: *mut BossGanondrof,
                                               mut globalCtx:
                                                   *mut GlobalContext) {
    osSyncPrintf(b"DAMAGE   .................................\n\x00" as
                     *const u8 as *const libc::c_char);
    SkelAnime_Update(&mut (*this).skelAnime);
    (*this).actor.gravity = -0.2f32;
    if (*this).actor.world.pos.y <= 5.0f32 {
        if (*this).work[GND_ACTION_STATE as libc::c_int as usize] as
               libc::c_int == STUNNED_FALL as libc::c_int {
            (*this).fwork[GND_END_FRAME as libc::c_int as usize] =
                Animation_GetLastFrame(&mut gPhantomGanonStunnedAnim as
                                           *mut AnimationHeader as
                                           *mut libc::c_void) as f32_0;
            Animation_MorphToLoop(&mut (*this).skelAnime,
                                  &mut gPhantomGanonStunnedAnim, -10.0f32);
            (*this).work[GND_ACTION_STATE as libc::c_int as usize] =
                STUNNED_GROUND as libc::c_int as s16
        }
        (*this).actor.velocity.y = 0.0f32;
        (*this).actor.gravity = 0.0f32;
        if Animation_OnFrame(&mut (*this).skelAnime,
                             (*this).fwork[GND_END_FRAME as libc::c_int as
                                               usize]) != 0 {
            Audio_PlayActorSound2(&mut (*this).actor,
                                  0x38b1 as libc::c_int as u16_0);
        }
        (*this).actor.flags |=
            ((1 as libc::c_int) << 10 as libc::c_int) as libc::c_uint
    }
    osSyncPrintf(b"TIME0 %d ********************************************\n\x00"
                     as *const u8 as *const libc::c_char,
                 (*this).timers[0 as libc::c_int as usize] as libc::c_int);
    if (*this).timers[0 as libc::c_int as usize] as libc::c_int ==
           0 as libc::c_int {
        BossGanondrof_SetupNeutral(this, -5.0f32);
        (*this).timers[0 as libc::c_int as usize] = 30 as libc::c_int as s16;
        (*this).timers[2 as libc::c_int as usize] = 30 as libc::c_int as s16;
        (*this).flyMode = GND_FLY_NEUTRAL as libc::c_int as u8_0;
        (*this).actor.velocity.y = 0.0f32;
        (*this).actor.gravity = 0.0f32
    }
    Actor_MoveForward(&mut (*this).actor);
}
#[no_mangle]
pub unsafe extern "C" fn BossGanondrof_SetupBlock(mut this:
                                                      *mut BossGanondrof,
                                                  mut globalCtx:
                                                      *mut GlobalContext) {
    (*this).fwork[GND_END_FRAME as libc::c_int as usize] =
        Animation_GetLastFrame(&mut gPhantomGanonBlockAnim as
                                   *mut AnimationHeader as *mut libc::c_void)
            as f32_0;
    Animation_MorphToLoop(&mut (*this).skelAnime, &mut gPhantomGanonBlockAnim,
                          -3.0f32);
    (*this).actionFunc =
        Some(BossGanondrof_Block as
                 unsafe extern "C" fn(_: *mut BossGanondrof,
                                      _: *mut GlobalContext) -> ());
    (*this).timers[0 as libc::c_int as usize] = 10 as libc::c_int as s16;
    Audio_PlayActorSound2(&mut (*this).actor, 0x38aa as libc::c_int as u16_0);
}
#[no_mangle]
pub unsafe extern "C" fn BossGanondrof_Block(mut this: *mut BossGanondrof,
                                             mut globalCtx:
                                                 *mut GlobalContext) {
    (*this).colliderBody.base.colType = COLTYPE_METAL as libc::c_int as u8_0;
    SkelAnime_Update(&mut (*this).skelAnime);
    (*this).actor.world.pos.x += (*this).actor.velocity.x;
    (*this).actor.world.pos.z += (*this).actor.velocity.z;
    Math_ApproachZeroF(&mut (*this).actor.velocity.x, 1.0f32, 0.5f32);
    Math_ApproachZeroF(&mut (*this).actor.velocity.z, 1.0f32, 0.5f32);
    (*this).actor.world.pos.y +=
        2.0f32 *
            Math_SinS(((*this).work[GND_VARIANCE_TIMER as libc::c_int as
                                        usize] as libc::c_int *
                           1500 as libc::c_int) as s16);
    if (*this).timers[0 as libc::c_int as usize] as libc::c_int ==
           0 as libc::c_int {
        BossGanondrof_SetupNeutral(this, -5.0f32);
        (*this).timers[0 as libc::c_int as usize] = 10 as libc::c_int as s16;
        (*this).flyMode = GND_FLY_NEUTRAL as libc::c_int as u8_0
    };
}
#[no_mangle]
pub unsafe extern "C" fn BossGanondrof_SetupCharge(mut this:
                                                       *mut BossGanondrof,
                                                   mut globalCtx:
                                                       *mut GlobalContext) {
    (*this).fwork[GND_END_FRAME as libc::c_int as usize] =
        Animation_GetLastFrame(&mut gPhantomGanonChargeWindupAnim as
                                   *mut AnimationHeader as *mut libc::c_void)
            as f32_0;
    Animation_MorphToLoop(&mut (*this).skelAnime,
                          &mut gPhantomGanonChargeWindupAnim, -3.0f32);
    (*this).actionFunc =
        Some(BossGanondrof_Charge as
                 unsafe extern "C" fn(_: *mut BossGanondrof,
                                      _: *mut GlobalContext) -> ());
    (*this).timers[0 as libc::c_int as usize] = 20 as libc::c_int as s16;
    (*this).work[GND_ACTION_STATE as libc::c_int as usize] =
        CHARGE_WINDUP as libc::c_int as s16;
}
#[no_mangle]
pub unsafe extern "C" fn BossGanondrof_Charge(mut this: *mut BossGanondrof,
                                              mut globalCtx:
                                                  *mut GlobalContext) {
    let mut player: *mut Player =
        (*globalCtx).actorCtx.actorLists[ACTORCAT_PLAYER as libc::c_int as
                                             usize].head as *mut Player;
    let mut playerx: *mut Actor = &mut (*player).actor;
    let mut thisx: *mut Actor = &mut (*this).actor;
    let mut dxCenter: f32_0 = (*thisx).world.pos.x - 14.0f32;
    let mut dzCenter: f32_0 = (*thisx).world.pos.z - -3315.0f32;
    (*this).colliderBody.base.colType = COLTYPE_METAL as libc::c_int as u8_0;
    SkelAnime_Update(&mut (*this).skelAnime);
    let mut current_block_60: u64;
    match (*this).work[GND_ACTION_STATE as libc::c_int as usize] as
              libc::c_int {
        0 => {
            if (*this).timers[0 as libc::c_int as usize] as libc::c_int ==
                   218 as libc::c_int {
                Audio_PlayActorSound2(thisx, 0x38aa as libc::c_int as u16_0);
            }
            if (*this).timers[0 as libc::c_int as usize] as libc::c_int ==
                   19 as libc::c_int {
                Audio_PlayActorSound2(thisx, 0x38a9 as libc::c_int as u16_0);
            }
            (*thisx).world.pos.x += (*thisx).velocity.x;
            (*thisx).world.pos.z += (*thisx).velocity.z;
            Math_ApproachZeroF(&mut (*thisx).velocity.x, 1.0f32, 0.5f32);
            Math_ApproachZeroF(&mut (*thisx).velocity.z, 1.0f32, 0.5f32);
            if (*this).timers[0 as libc::c_int as usize] as libc::c_int ==
                   0 as libc::c_int {
                (*this).work[GND_ACTION_STATE as libc::c_int as usize] =
                    CHARGE_START as libc::c_int as s16;
                (*this).timers[0 as libc::c_int as usize] =
                    10 as libc::c_int as s16;
                (*thisx).speedXZ = 0.0f32;
                (*this).fwork[GND_END_FRAME as libc::c_int as usize] =
                    Animation_GetLastFrame(&mut gPhantomGanonChargeStartAnim
                                               as *mut AnimationHeader as
                                               *mut libc::c_void) as f32_0;
                Animation_MorphToPlayOnce(&mut (*this).skelAnime,
                                          &mut gPhantomGanonChargeStartAnim,
                                          0.0f32);
            }
            Math_ApproachS(&mut (*thisx).shape.rot.y,
                           (*thisx).yawTowardsPlayer, 5 as libc::c_int as s16,
                           0x7d0 as libc::c_int as s16);
            current_block_60 = 13303144130133872306;
        }
        1 => {
            if Animation_OnFrame(&mut (*this).skelAnime,
                                 (*this).fwork[GND_END_FRAME as libc::c_int as
                                                   usize]) != 0 {
                (*this).fwork[GND_END_FRAME as libc::c_int as usize] =
                    Animation_GetLastFrame(&mut gPhantomGanonChargeAnim as
                                               *mut AnimationHeader as
                                               *mut libc::c_void) as f32_0;
                Animation_MorphToLoop(&mut (*this).skelAnime,
                                      &mut gPhantomGanonChargeAnim, 0.0f32);
                (*this).work[GND_ACTION_STATE as libc::c_int as usize] =
                    CHARGE_ATTACK as libc::c_int as s16
            }
            current_block_60 = 4640245532892785160;
        }
        2 => { current_block_60 = 4640245532892785160; }
        3 => {
            (*thisx).gravity = 0.2f32;
            Actor_MoveForward(thisx);
            osSyncPrintf(b"YP %f @@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@\n\x00"
                             as *const u8 as *const libc::c_char,
                         (*thisx).world.pos.y as libc::c_double);
            if (*thisx).world.pos.y < 5.0f32 {
                (*thisx).world.pos.y = 5.0f32;
                (*thisx).velocity.y = 0.0f32
            }
            if sqrtf(dxCenter * dxCenter + dzCenter * dzCenter) > 280.0f32 {
                Math_ApproachZeroF(&mut (*thisx).speedXZ, 1.0f32, 2.0f32);
                (*this).timers[0 as libc::c_int as usize] =
                    0 as libc::c_int as s16
            }
            if (*this).timers[0 as libc::c_int as usize] as libc::c_int ==
                   0 as libc::c_int {
                Math_ApproachZeroF(&mut (*thisx).speedXZ, 1.0f32, 2.0f32);
                Math_ApproachZeroF(&mut (*thisx).velocity.y, 1.0f32, 2.0f32);
                Math_ApproachS(&mut (*thisx).shape.rot.y,
                               (*thisx).yawTowardsPlayer,
                               5 as libc::c_int as s16,
                               0x7d0 as libc::c_int as s16);
                if (*thisx).speedXZ <= 0.5f32 &&
                       fabsf((*thisx).velocity.y) <= 0.1f32 {
                    BossGanondrof_SetupNeutral(this, -10.0f32);
                    (*this).timers[0 as libc::c_int as usize] =
                        30 as libc::c_int as s16;
                    (*this).flyMode = GND_FLY_NEUTRAL as libc::c_int as u8_0
                }
            }
            current_block_60 = 13303144130133872306;
        }
        _ => { current_block_60 = 13303144130133872306; }
    }
    match current_block_60 {
        4640245532892785160 => {
            if (*this).timers[0 as libc::c_int as usize] as libc::c_int !=
                   0 as libc::c_int {
                let mut vecToLink: Vec3f = Vec3f{x: 0., y: 0., z: 0.,};
                Math_ApproachS(&mut (*thisx).shape.rot.y,
                               (*thisx).yawTowardsPlayer,
                               5 as libc::c_int as s16,
                               0x7d0 as libc::c_int as s16);
                vecToLink.x = (*playerx).world.pos.x - (*thisx).world.pos.x;
                vecToLink.y =
                    (*playerx).world.pos.y + 40.0f32 - (*thisx).world.pos.y;
                vecToLink.z = (*playerx).world.pos.z - (*thisx).world.pos.z;
                (*thisx).world.rot.y = (*thisx).shape.rot.y;
                (*thisx).world.rot.x =
                    (Math_FAtan2F(vecToLink.y,
                                  sqrtf(vecToLink.x * vecToLink.x +
                                            vecToLink.z * vecToLink.z)) *
                         (0x8000 as libc::c_int as libc::c_float /
                              3.14159265358979323846f32)) as s16
            }
            func_8002D908(thisx);
            func_8002D7EC(thisx);
            Math_ApproachF(&mut (*thisx).speedXZ, 10.0f32, 1.0f32, 0.5f32);
            if sqrtf(dxCenter * dxCenter + dzCenter * dzCenter) > 280.0f32 ||
                   (*thisx).xyzDistToPlayerSq < 100.0f32 * 100.0f32 {
                (*this).work[GND_ACTION_STATE as libc::c_int as usize] =
                    CHARGE_FINISH as libc::c_int as s16;
                (*this).timers[0 as libc::c_int as usize] =
                    20 as libc::c_int as s16
            }
        }
        _ => { }
    }
    if (*thisx).world.pos.y > -33.0f32 + 83.0f32 {
        (*thisx).world.pos.y +=
            2.0f32 *
                Math_SinS(((*this).work[GND_VARIANCE_TIMER as libc::c_int as
                                            usize] as libc::c_int *
                               1500 as libc::c_int) as s16)
    }
    let mut i: s16 = 0;
    let mut pos: Vec3f = Vec3f{x: 0., y: 0., z: 0.,};
    let mut vel: Vec3f =
        { let mut init = Vec3f{x: 0.0f32, y: 0.0f32, z: 0.0f32,}; init };
    let mut accel: Vec3f =
        { let mut init = Vec3f{x: 0.0f32, y: 0.0f32, z: 0.0f32,}; init };
    let mut baseOffset: Vec3f =
        { let mut init = Vec3f{x: 0.0f32, y: 50.0f32, z: 0.0f32,}; init };
    let mut offset: Vec3f = Vec3f{x: 0., y: 0., z: 0.,};
    baseOffset.y = 10.0f32;
    i = 0 as libc::c_int as s16;
    while (i as libc::c_int) < 10 as libc::c_int {
        Matrix_Push();
        Matrix_RotateY((*thisx).shape.rot.y as libc::c_int as libc::c_float /
                           0x8000 as libc::c_int as f32_0 *
                           3.14159265358979323846f32,
                       MTXMODE_NEW as libc::c_int as u8_0);
        Matrix_RotateX((*thisx).shape.rot.x as libc::c_int as libc::c_float /
                           0x8000 as libc::c_int as f32_0 *
                           3.14159265358979323846f32,
                       MTXMODE_APPLY as libc::c_int as u8_0);
        Matrix_RotateZ((*this).work[GND_PARTICLE_ANGLE as libc::c_int as
                                        usize] as libc::c_int as libc::c_float
                           / 0x8000 as libc::c_int as f32_0 *
                           3.14159265358979323846f32,
                       MTXMODE_APPLY as libc::c_int as u8_0);
        Matrix_MultVec3f(&mut baseOffset, &mut offset);
        Matrix_Pop();
        pos.x = (*this).spearTip.x + offset.x;
        pos.y = (*this).spearTip.y + offset.y;
        pos.z = (*this).spearTip.z + offset.z;
        vel.x = offset.x * 500.0f32 / 1000.0f32;
        vel.y = offset.y * 500.0f32 / 1000.0f32;
        vel.z = offset.z * 500.0f32 / 1000.0f32;
        accel.x = offset.x * -50.0f32 / 1000.0f32;
        accel.y = offset.y * -50.0f32 / 1000.0f32;
        accel.z = offset.z * -50.0f32 / 1000.0f32;
        EffectSsFhgFlash_SpawnLightBall(globalCtx, &mut pos, &mut vel,
                                        &mut accel, 150 as libc::c_int as s16,
                                        (i as libc::c_int % 7 as libc::c_int)
                                            as u8_0);
        (*this).work[GND_PARTICLE_ANGLE as libc::c_int as usize] =
            ((*this).work[GND_PARTICLE_ANGLE as libc::c_int as usize] as
                 libc::c_int + 0x1a5c as libc::c_int) as s16;
        i += 1
    }
    if (*this).work[GND_VARIANCE_TIMER as libc::c_int as usize] as libc::c_int
           & 7 as libc::c_int == 0 {
        let mut horse: *mut EnfHG = (*thisx).child as *mut EnfHG;
        Actor_SpawnAsChild(&mut (*globalCtx).actorCtx, thisx, globalCtx,
                           ACTOR_EN_FHG_FIRE as libc::c_int as s16,
                           (*this).spearTip.x, (*this).spearTip.y,
                           (*this).spearTip.z, 8 as libc::c_int as s16,
                           FHGFIRE_LIGHT_BLUE as libc::c_int as s16,
                           0 as libc::c_int as s16,
                           FHGFIRE_SPEAR_LIGHT as libc::c_int as s16);
        (*thisx).child = &mut (*horse).actor
    };
}
#[no_mangle]
pub unsafe extern "C" fn BossGanondrof_SetupDeath(mut this:
                                                      *mut BossGanondrof,
                                                  mut globalCtx:
                                                      *mut GlobalContext) {
    Animation_PlayOnce(&mut (*this).skelAnime,
                       &mut gPhantomGanonDeathBlowAnim);
    (*this).fwork[GND_END_FRAME as libc::c_int as usize] =
        Animation_GetLastFrame(&mut gPhantomGanonDeathBlowAnim as
                                   *mut AnimationHeader as *mut libc::c_void)
            as f32_0;
    (*this).actionFunc =
        Some(BossGanondrof_Death as
                 unsafe extern "C" fn(_: *mut BossGanondrof,
                                      _: *mut GlobalContext) -> ());
    Audio_QueueSeqCmd(((0x1 as libc::c_int) << 28 as libc::c_int |
                           (SEQ_PLAYER_BGM_MAIN as libc::c_int) <<
                               24 as libc::c_int | 0x100ff as libc::c_int) as
                          u32_0);
    Audio_PlayActorSound2(&mut (*this).actor, 0x38af as libc::c_int as u16_0);
    (*this).deathState = DEATH_START as libc::c_int as s16;
    (*this).actor.flags &=
        !((1 as libc::c_int) << 0 as libc::c_int) as libc::c_uint;
    (*this).work[GND_VARIANCE_TIMER as libc::c_int as usize] =
        0 as libc::c_int as s16;
    (*this).shockTimer = 50 as libc::c_int as u8_0;
}
#[no_mangle]
pub unsafe extern "C" fn BossGanondrof_Death(mut this: *mut BossGanondrof,
                                             mut globalCtx:
                                                 *mut GlobalContext) {
    let mut holdCamera: u8_0 = 0 as libc::c_int as u8_0;
    let mut bodyDecayLevel: u8_0 = 0 as libc::c_int as u8_0;
    let mut camX: f32_0 = 0.;
    let mut camZ: f32_0 = 0.;
    let mut pad: f32_0 = 0.;
    let mut player: *mut Player =
        (*globalCtx).actorCtx.actorLists[ACTORCAT_PLAYER as libc::c_int as
                                             usize].head as *mut Player;
    let mut camera: *mut Camera =
        Gameplay_GetCamera(globalCtx, 0 as libc::c_int as s16);
    osSyncPrintf(b"PYP %f\n\x00" as *const u8 as *const libc::c_char,
                 (*player).actor.floorHeight as libc::c_double);
    SkelAnime_Update(&mut (*this).skelAnime);
    (*this).work[GND_DEATH_SFX_TIMER as libc::c_int as usize] += 1;
    if (60 as libc::c_int) <
           (*this).work[GND_DEATH_SFX_TIMER as libc::c_int as usize] as
               libc::c_int &&
           ((*this).work[GND_DEATH_SFX_TIMER as libc::c_int as usize] as
                libc::c_int) < 500 as libc::c_int ||
           (501 as libc::c_int) <
               (*this).work[GND_DEATH_SFX_TIMER as libc::c_int as usize] as
                   libc::c_int &&
               ((*this).work[GND_DEATH_SFX_TIMER as libc::c_int as usize] as
                    libc::c_int) < 620 as libc::c_int {
        Audio_PlayActorSound2(&mut (*this).actor,
                              (0x381c as libc::c_int - 0x800 as libc::c_int)
                                  as u16_0);
    }
    let mut current_block_148: u64;
    match (*this).deathState as libc::c_int {
        1 => {
            func_80064520(globalCtx, &mut (*globalCtx).csCtx);
            func_8002DF54(globalCtx, &mut (*this).actor,
                          1 as libc::c_int as u8_0);
            (*this).deathCamera = Gameplay_CreateSubCamera(globalCtx);
            Gameplay_ChangeCameraStatus(globalCtx, 0 as libc::c_int as s16,
                                        1 as libc::c_int as s16);
            osSyncPrintf(b"7\n\x00" as *const u8 as *const libc::c_char);
            Gameplay_ChangeCameraStatus(globalCtx, (*this).deathCamera,
                                        7 as libc::c_int as s16);
            osSyncPrintf(b"8\n\x00" as *const u8 as *const libc::c_char);
            (*this).deathState = DEATH_THROES as libc::c_int as s16;
            (*player).actor.speedXZ = 0.0f32;
            (*this).timers[0 as libc::c_int as usize] =
                50 as libc::c_int as s16;
            (*this).cameraEye = (*camera).eye;
            (*this).cameraAt = (*camera).at;
            (*this).cameraNextEye.x = (*this).targetPos.x;
            (*this).cameraNextEye.y = -33.0f32 + 83.0f32;
            (*this).cameraNextEye.z =
                (*this).targetPos.z + 100.0f32 +
                    50 as libc::c_int as libc::c_float;
            (*this).cameraNextAt.x = (*this).targetPos.x;
            (*this).cameraNextAt.y = (*this).targetPos.y - 10.0f32;
            (*this).cameraNextAt.z = (*this).targetPos.z;
            (*this).cameraEyeVel.x =
                fabsf((*camera).eye.x - (*this).cameraNextEye.x);
            (*this).cameraEyeVel.y =
                fabsf((*camera).eye.y - (*this).cameraNextEye.y);
            (*this).cameraEyeVel.z =
                fabsf((*camera).eye.z - (*this).cameraNextEye.z);
            (*this).cameraAtVel.x =
                fabsf((*camera).at.x - (*this).cameraNextAt.x);
            (*this).cameraAtVel.y =
                fabsf((*camera).at.y - (*this).cameraNextAt.y);
            (*this).cameraAtVel.z =
                fabsf((*camera).at.z - (*this).cameraNextAt.z);
            (*this).cameraAccel = 0.02f32;
            (*this).cameraEyeMaxVel.z = 0.05f32;
            (*this).cameraEyeMaxVel.y = (*this).cameraEyeMaxVel.z;
            (*this).cameraEyeMaxVel.x = (*this).cameraEyeMaxVel.y;
            (*this).work[GND_ACTION_STATE as libc::c_int as usize] =
                DEATH_SPASM as libc::c_int as s16;
            (*this).timers[0 as libc::c_int as usize] =
                150 as libc::c_int as s16;
            (*this).cameraAtMaxVel.x = 0.2f32;
            (*this).cameraAtMaxVel.y = 0.2f32;
            (*this).cameraAtMaxVel.z = 0.2f32;
            current_block_148 = 6713868179933505918;
        }
        2 => { current_block_148 = 6713868179933505918; }
        3 => {
            if (*this).timers[1 as libc::c_int as usize] as libc::c_int ==
                   1 as libc::c_int {
                let mut horseTemp: *mut EnfHG =
                    (*this).actor.child as *mut EnfHG;
                Actor_SpawnAsChild(&mut (*globalCtx).actorCtx,
                                   &mut (*this).actor, globalCtx,
                                   ACTOR_EN_FHG_FIRE as libc::c_int as s16,
                                   14.0f32, -33.0f32 + 3.0f32, -3315.0f32,
                                   0x4000 as libc::c_int as s16,
                                   0 as libc::c_int as s16,
                                   0 as libc::c_int as s16,
                                   FHGFIRE_WARP_DEATH as libc::c_int as s16);
                (*this).actor.child = &mut (*horseTemp).actor;
                Message_StartTextbox(globalCtx,
                                     0x108e as libc::c_int as u16_0,
                                     0 as *mut Actor);
            }
            (*this).actor.shape.rot.y =
                ((*this).actor.shape.rot.y as libc::c_int -
                     0xc8 as libc::c_int) as s16;
            (*this).actor.world.pos.y +=
                Math_SinS(((*this).work[GND_VARIANCE_TIMER as libc::c_int as
                                            usize] as libc::c_int *
                               1500 as libc::c_int) as s16);
            (*this).fwork[GND_CAMERA_ANGLE as libc::c_int as usize] +=
                0x78 as libc::c_int as libc::c_float;
            camX =
                Math_SinS((*this).fwork[GND_CAMERA_ANGLE as libc::c_int as
                                            usize] as s16) *
                    (*this).fwork[GND_CAMERA_ZOOM as libc::c_int as usize];
            camZ =
                Math_CosS((*this).fwork[GND_CAMERA_ANGLE as libc::c_int as
                                            usize] as s16) *
                    (*this).fwork[GND_CAMERA_ZOOM as libc::c_int as usize];
            (*this).cameraEye.x = 14.0f32 + camX;
            (*this).cameraEye.y = (*this).cameraNextEye.y;
            (*this).cameraEye.z = -3315.0f32 + camZ;
            (*this).cameraAt.x = 14.0f32;
            (*this).cameraAt.y = -33.0f32 + 23.0f32;
            (*this).cameraAt.z = -3315.0f32;
            Math_ApproachF(&mut (*this).cameraNextEye.y, -33.0f32 + 33.0f32,
                           0.05f32, 0.5f32);
            Math_ApproachF(&mut *(*this).fwork.as_mut_ptr().offset(GND_CAMERA_ZOOM
                                                                       as
                                                                       libc::c_int
                                                                       as
                                                                       isize),
                           170.0f32, 0.05f32, 1.0f32);
            Math_ApproachF(&mut (*this).actor.world.pos.x, 14.0f32, 0.05f32,
                           1.5f32);
            Math_ApproachF(&mut (*this).actor.world.pos.y, -33.0f32 + 83.0f32,
                           0.05f32, 1.0f32);
            Math_ApproachF(&mut (*this).actor.world.pos.z, -3315.0f32,
                           0.05f32, 1.5f32);
            if (*this).timers[0 as libc::c_int as usize] as libc::c_int ==
                   0 as libc::c_int {
                (*this).deathState = DEATH_SCREAM as libc::c_int as s16;
                (*this).timers[0 as libc::c_int as usize] =
                    50 as libc::c_int as s16;
                Animation_MorphToLoop(&mut (*this).skelAnime,
                                      &mut gPhantomGanonScreamAnim, -10.0f32);
                (*this).actor.world.pos.x = 14.0f32;
                (*this).actor.world.pos.y = -33.0f32 + 83.0f32;
                (*this).actor.world.pos.z = -3315.0f32;
                (*this).actor.shape.rot.y = 0 as libc::c_int as s16;
                (*this).work[GND_BODY_DECAY_INDEX as libc::c_int as usize] =
                    0 as libc::c_int as s16;
                Audio_PlayActorSound2(&mut (*this).actor,
                                      0x38ac as libc::c_int as u16_0);
            }
            holdCamera = 1 as libc::c_int as u8_0;
            bodyDecayLevel = 1 as libc::c_int as u8_0;
            current_block_148 = 14714495436747744489;
        }
        4 => {
            holdCamera = 1 as libc::c_int as u8_0;
            bodyDecayLevel = 2 as libc::c_int as u8_0;
            (*this).actor.world.pos.y = -33.0f32 + 83.0f32;
            (*this).cameraEye.x = 14.0f32;
            (*this).cameraEye.y = -33.0f32 + 83.0f32;
            (*this).cameraEye.z = -3315.0f32 + 50.0f32;
            (*this).cameraAt.x = 14.0f32;
            (*this).cameraAt.y = -33.0f32 + 103.0f32;
            (*this).cameraAt.z = -3315.0f32;
            if (*this).timers[0 as libc::c_int as usize] as libc::c_int ==
                   0 as libc::c_int {
                (*this).deathState = DEATH_DISINTEGRATE as libc::c_int as s16;
                Animation_MorphToPlayOnce(&mut (*this).skelAnime,
                                          &mut gPhantomGanonLastPoseAnim,
                                          -10.0f32);
                (*this).work[GND_BODY_DECAY_INDEX as libc::c_int as usize] =
                    0 as libc::c_int as s16;
                (*this).timers[0 as libc::c_int as usize] =
                    40 as libc::c_int as s16
            }
            current_block_148 = 14714495436747744489;
        }
        5 => {
            holdCamera = 1 as libc::c_int as u8_0;
            bodyDecayLevel = 3 as libc::c_int as u8_0;
            Math_ApproachZeroF(&mut (*this).cameraEye.y, 0.05f32, 1.0f32);
            Math_ApproachF(&mut (*this).cameraEye.z, -3315.0f32 + 170.0f32,
                           0.05f32, 2.0f32);
            Math_ApproachF(&mut (*this).cameraAt.y, -33.0f32 + 53.0f32,
                           0.05f32, 1.0f32);
            if (*this).timers[0 as libc::c_int as usize] as libc::c_int ==
                   0 as libc::c_int {
                (*this).timers[0 as libc::c_int as usize] =
                    250 as libc::c_int as s16;
                (*this).deathState = DEATH_FINISH as libc::c_int as s16
            }
            current_block_148 = 14714495436747744489;
        }
        6 => {
            holdCamera = 1 as libc::c_int as u8_0;
            bodyDecayLevel = 10 as libc::c_int as u8_0;
            if (*this).timers[0 as libc::c_int as usize] as libc::c_int ==
                   150 as libc::c_int {
                Audio_QueueSeqCmd(((SEQ_PLAYER_BGM_MAIN as libc::c_int) <<
                                       24 as libc::c_int |
                                       0x21 as libc::c_int) as u32_0);
                Actor_Spawn(&mut (*globalCtx).actorCtx, globalCtx,
                            ACTOR_DOOR_WARP1 as libc::c_int as s16, 14.0f32,
                            -33.0f32, -3315.0f32, 0 as libc::c_int as s16,
                            0 as libc::c_int as s16, 0 as libc::c_int as s16,
                            WARP_DUNGEON_ADULT as libc::c_int as s16);
            }
            Math_ApproachZeroF(&mut (*this).cameraEye.y, 0.05f32, 1.0f32);
            Math_ApproachF(&mut (*this).cameraEye.z, -3315.0f32 + 170.0f32,
                           0.05f32, 2.0f32);
            Math_ApproachF(&mut (*this).cameraAt.y, -33.0f32 + 53.0f32,
                           0.05f32, 1.0f32);
            if (*this).timers[0 as libc::c_int as usize] as libc::c_int ==
                   0 as libc::c_int {
                let mut horse: *mut EnfHG = (*this).actor.child as *mut EnfHG;
                (*camera).eye = (*this).cameraEye;
                (*camera).eyeNext = (*this).cameraEye;
                (*camera).at = (*this).cameraAt;
                func_800C08AC(globalCtx, (*this).deathCamera,
                              0 as libc::c_int as s16);
                (*this).deathCamera = 0 as libc::c_int as s16;
                func_80064534(globalCtx, &mut (*globalCtx).csCtx);
                func_8002DF54(globalCtx, &mut (*this).actor,
                              7 as libc::c_int as u8_0);
                Actor_Spawn(&mut (*globalCtx).actorCtx, globalCtx,
                            ACTOR_ITEM_B_HEART as libc::c_int as s16, 14.0f32,
                            -33.0f32, -3315.0f32 + 200.0f32,
                            0 as libc::c_int as s16, 0 as libc::c_int as s16,
                            0 as libc::c_int as s16, 0 as libc::c_int as s16);
                (*this).actor.child = &mut (*horse).actor;
                (*this).killActor = 1 as libc::c_int as u8_0;
                (*horse).killActor = 1 as libc::c_int as u8_0;
                Flags_SetClear(globalCtx,
                               (*globalCtx).roomCtx.curRoom.num as s32);
                Flags_SetSwitch(globalCtx, 0x22 as libc::c_int);
            }
            current_block_148 = 14714495436747744489;
        }
        _ => { current_block_148 = 14714495436747744489; }
    }
    match current_block_148 {
        6713868179933505918 => {
            let mut current_block_47: u64;
            match (*this).work[GND_ACTION_STATE as libc::c_int as usize] as
                      libc::c_int {
                0 => {
                    if Animation_OnFrame(&mut (*this).skelAnime,
                                         (*this).fwork[GND_END_FRAME as
                                                           libc::c_int as
                                                           usize]) != 0 {
                        (*this).fwork[GND_END_FRAME as libc::c_int as usize] =
                            Animation_GetLastFrame(&mut gPhantomGanonAirDamageAnim
                                                       as *mut AnimationHeader
                                                       as *mut libc::c_void)
                                as f32_0;
                        Animation_Change(&mut (*this).skelAnime,
                                         &mut gPhantomGanonAirDamageAnim,
                                         0.5f32, 0.0f32,
                                         (*this).fwork[GND_END_FRAME as
                                                           libc::c_int as
                                                           usize],
                                         ANIMMODE_ONCE_INTERP as libc::c_int
                                             as u8_0, 0.0f32);
                        (*this).work[GND_ACTION_STATE as libc::c_int as usize]
                            = DEATH_LIMP as libc::c_int as s16
                    }
                    current_block_47 = 12497913735442871383;
                }
                1 => {
                    if Animation_OnFrame(&mut (*this).skelAnime,
                                         (*this).fwork[GND_END_FRAME as
                                                           libc::c_int as
                                                           usize]) != 0 {
                        (*this).fwork[GND_END_FRAME as libc::c_int as usize] =
                            Animation_GetLastFrame(&mut gPhantomGanonLimpAnim
                                                       as *mut AnimationHeader
                                                       as *mut libc::c_void)
                                as f32_0;
                        Animation_MorphToLoop(&mut (*this).skelAnime,
                                              &mut gPhantomGanonLimpAnim,
                                              -20.0f32);
                        (*this).work[GND_ACTION_STATE as libc::c_int as usize]
                            = DEATH_HUNCHED as libc::c_int as s16
                    }
                    current_block_47 = 12130795097122128555;
                }
                2 => { current_block_47 = 12130795097122128555; }
                _ => { current_block_47 = 12497913735442871383; }
            }
            match current_block_47 {
                12130795097122128555 => {
                    bodyDecayLevel = 1 as libc::c_int as u8_0
                }
                _ => { }
            }
            Math_ApproachS(&mut (*this).actor.shape.rot.y,
                           ((*this).work[GND_VARIANCE_TIMER as libc::c_int as
                                             usize] as libc::c_int *
                                -(100 as libc::c_int)) as s16,
                           5 as libc::c_int as s16,
                           0xbb8 as libc::c_int as s16);
            Math_ApproachF(&mut (*this).cameraNextEye.z,
                           (*this).targetPos.z + 60.0f32, 0.02f32, 0.5f32);
            Math_ApproachF(&mut (*this).actor.world.pos.y,
                           -33.0f32 + 133.0f32, 0.05f32, 100.0f32);
            (*this).actor.world.pos.y +=
                Math_SinS(((*this).work[GND_VARIANCE_TIMER as libc::c_int as
                                            usize] as libc::c_int *
                               1500 as libc::c_int) as s16);
            (*this).cameraNextAt.x = (*this).targetPos.x;
            (*this).cameraNextAt.y = (*this).targetPos.y - 10.0f32;
            (*this).cameraNextAt.z = (*this).targetPos.z;
            if (*this).timers[0 as libc::c_int as usize] as libc::c_int ==
                   0 as libc::c_int {
                (*this).deathState = DEATH_WARP as libc::c_int as s16;
                (*this).timers[0 as libc::c_int as usize] =
                    350 as libc::c_int as s16;
                (*this).timers[1 as libc::c_int as usize] =
                    50 as libc::c_int as s16;
                (*this).fwork[GND_CAMERA_ZOOM as libc::c_int as usize] =
                    300.0f32;
                (*this).cameraNextEye.y = -33.0f32 + 233.0f32;
                (*player).actor.world.pos.x = 14.0f32 - 200.0f32;
                (*player).actor.world.pos.z = -3315.0f32;
                holdCamera = 1 as libc::c_int as u8_0;
                bodyDecayLevel = 1 as libc::c_int as u8_0
            }
        }
        _ => { }
    }
    if bodyDecayLevel != 0 {
        let mut pos: Vec3f = Vec3f{x: 0., y: 0., z: 0.,};
        let mut vel: Vec3f =
            { let mut init = Vec3f{x: 0.0f32, y: 0.0f32, z: 0.0f32,}; init };
        let mut accelKFire: Vec3f =
            { let mut init = Vec3f{x: 0.0f32, y: 0.0f32, z: 0.0f32,}; init };
        let mut accelHahen: Vec3f =
            { let mut init = Vec3f{x: 0.0f32, y: -0.5f32, z: 0.0f32,}; init };
        let mut limbDecayIndex: s16 = 0;
        let mut i: s16 = 0;
        vel.x = (*this).actor.world.pos.x - (*this).actor.prevPos.x;
        vel.z = (*this).actor.world.pos.z - (*this).actor.prevPos.z;
        if (bodyDecayLevel as libc::c_int) < 10 as libc::c_int {
            if (*this).work[GND_DEATH_ENV_TIMER as libc::c_int as usize] as
                   libc::c_int == 0 as libc::c_int {
                if (*globalCtx).envCtx.unk_BF as libc::c_int ==
                       0 as libc::c_int {
                    (*globalCtx).envCtx.unk_BF = 3 as libc::c_int as u8_0;
                    (*this).work[GND_DEATH_ENV_TIMER as libc::c_int as usize]
                        =
                        (Rand_ZeroFloat(5.0f32) as s16 as libc::c_int as
                             libc::c_float + 4.0f32) as s16;
                    (*globalCtx).envCtx.unk_D6 = 0x28 as libc::c_int as u16_0
                } else {
                    (*globalCtx).envCtx.unk_BF = 0 as libc::c_int as u8_0;
                    (*this).work[GND_DEATH_ENV_TIMER as libc::c_int as usize]
                        =
                        (Rand_ZeroFloat(2.0f32) as s16 as libc::c_int as
                             libc::c_float + 2.0f32) as s16;
                    (*globalCtx).envCtx.unk_D6 = 0x14 as libc::c_int as u16_0
                }
            } else {
                (*this).work[GND_DEATH_ENV_TIMER as libc::c_int as usize] -= 1
            }
            i = 0 as libc::c_int as s16;
            while i as libc::c_int <= 0 as libc::c_int {
                limbDecayIndex =
                    (*this).work[GND_LIMB_DECAY_INDEX as libc::c_int as
                                     usize];
                (*this).work[GND_LIMB_DECAY_INDEX as libc::c_int as usize] +=
                    1;
                (*this).work[GND_LIMB_DECAY_INDEX as libc::c_int as usize] =
                    ((*this).work[GND_LIMB_DECAY_INDEX as libc::c_int as
                                      usize] as libc::c_int %
                         25 as libc::c_int) as s16;
                pos.x =
                    (*this).bodyPartsPos[limbDecayIndex as usize].x +
                        Rand_CenteredFloat(5.0f32);
                pos.y =
                    (*this).bodyPartsPos[limbDecayIndex as usize].y +
                        Rand_CenteredFloat(5.0f32);
                pos.z =
                    (*this).bodyPartsPos[limbDecayIndex as usize].z +
                        Rand_CenteredFloat(5.0f32);
                accelKFire.y = 0.0f32;
                if bodyDecayLevel as libc::c_int == 3 as libc::c_int {
                    accelKFire.y = -0.2f32;
                    accelKFire.x = (14.0f32 - pos.x) * 0.002f32;
                    accelKFire.z = (-3315.0f32 - pos.z) * 0.002f32;
                    accelHahen.x = (14.0f32 - pos.x) * 0.001f32;
                    accelHahen.y = -1.0f32;
                    accelHahen.z = (-3315.0f32 - pos.z) * 0.001f32
                }
                EffectSsKFire_Spawn(globalCtx, &mut pos, &mut vel,
                                    &mut accelKFire,
                                    (Rand_ZeroFloat(20.0f32) as s16 as
                                         libc::c_int + 15 as libc::c_int) as
                                        s16, bodyDecayLevel);
                if Rand_ZeroOne() < 0.5f32 ||
                       bodyDecayLevel as libc::c_int == 3 as libc::c_int {
                    EffectSsHahen_Spawn(globalCtx, &mut pos, &mut vel,
                                        &mut accelHahen,
                                        0 as libc::c_int as s16,
                                        (Rand_ZeroFloat(4.0f32) as s16 as
                                             libc::c_int + 7 as libc::c_int)
                                            as s16,
                                        -(1 as libc::c_int) as s16,
                                        10 as libc::c_int as s16,
                                        0 as *mut Gfx);
                }
                i += 1
            }
        } else {
            (*globalCtx).envCtx.unk_BF = 0 as libc::c_int as u8_0;
            (*globalCtx).envCtx.unk_D6 = 0x14 as libc::c_int as u16_0
        }
        (*this).work[GND_BODY_DECAY_FLAG as libc::c_int as usize] =
            1 as libc::c_int as s16;
        i = 0 as libc::c_int as s16;
        while (i as libc::c_int) < 5 as libc::c_int {
            if bodyDecayLevel as libc::c_int == 1 as libc::c_int {
                BossGanondrof_ClearPixels(sDecayMaskLow.as_mut_ptr(),
                                          (*this).work[GND_BODY_DECAY_INDEX as
                                                           libc::c_int as
                                                           usize]);
            } else if bodyDecayLevel as libc::c_int == 2 as libc::c_int {
                BossGanondrof_ClearPixels(sDecayMaskHigh.as_mut_ptr(),
                                          (*this).work[GND_BODY_DECAY_INDEX as
                                                           libc::c_int as
                                                           usize]);
            } else {
                BossGanondrof_ClearPixels(sDecayMaskTotal.as_mut_ptr(),
                                          (*this).work[GND_BODY_DECAY_INDEX as
                                                           libc::c_int as
                                                           usize]);
            }
            if ((*this).work[GND_BODY_DECAY_INDEX as libc::c_int as usize] as
                    libc::c_int) < 0x100 as libc::c_int {
                (*this).work[GND_BODY_DECAY_INDEX as libc::c_int as usize] +=
                    1
            }
            i += 1
        }
    }
    if (*this).deathCamera as libc::c_int != 0 as libc::c_int {
        if holdCamera == 0 {
            Math_ApproachF(&mut (*this).cameraEye.x, (*this).cameraNextEye.x,
                           (*this).cameraEyeMaxVel.x,
                           (*this).cameraEyeVel.x * (*this).cameraSpeedMod);
            Math_ApproachF(&mut (*this).cameraEye.y, (*this).cameraNextEye.y,
                           (*this).cameraEyeMaxVel.y,
                           (*this).cameraEyeVel.y * (*this).cameraSpeedMod);
            Math_ApproachF(&mut (*this).cameraEye.z, (*this).cameraNextEye.z,
                           (*this).cameraEyeMaxVel.z,
                           (*this).cameraEyeVel.z * (*this).cameraSpeedMod);
            Math_ApproachF(&mut (*this).cameraAt.x, (*this).cameraNextAt.x,
                           (*this).cameraAtMaxVel.x,
                           (*this).cameraAtVel.x * (*this).cameraSpeedMod);
            Math_ApproachF(&mut (*this).cameraAt.y, (*this).cameraNextAt.y,
                           (*this).cameraAtMaxVel.y,
                           (*this).cameraAtVel.y * (*this).cameraSpeedMod);
            Math_ApproachF(&mut (*this).cameraAt.z, (*this).cameraNextAt.z,
                           (*this).cameraAtMaxVel.z,
                           (*this).cameraAtVel.z * (*this).cameraSpeedMod);
            Math_ApproachF(&mut (*this).cameraSpeedMod, 1.0f32, 1.0f32,
                           (*this).cameraAccel);
        }
        Gameplay_CameraSetAtEye(globalCtx, (*this).deathCamera,
                                &mut (*this).cameraAt,
                                &mut (*this).cameraEye);
    };
}
#[no_mangle]
pub unsafe extern "C" fn BossGanondrof_CollisionCheck(mut this:
                                                          *mut BossGanondrof,
                                                      mut globalCtx:
                                                          *mut GlobalContext) {
    let mut acHit: s32 = 0;
    let mut horse: *mut EnfHG = (*this).actor.child as *mut EnfHG;
    let mut hurtbox: *mut ColliderInfo = 0 as *mut ColliderInfo;
    if (*this).work[GND_INVINC_TIMER as libc::c_int as usize] as libc::c_int
           != 0 as libc::c_int {
        (*this).work[GND_INVINC_TIMER as libc::c_int as usize] -= 1;
        (*this).returnCount = 0 as libc::c_int as u8_0;
        (*this).colliderBody.base.acFlags =
            ((*this).colliderBody.base.acFlags as libc::c_int &
                 !((1 as libc::c_int) << 1 as libc::c_int)) as u8_0
    } else {
        acHit =
            (*this).colliderBody.base.acFlags as libc::c_int &
                (1 as libc::c_int) << 1 as libc::c_int;
        if acHit != 0 &&
               (*this).actor.colChkInfo.health as s8 as libc::c_int >
                   0 as libc::c_int ||
               (*this).returnCount as libc::c_int != 0 as libc::c_int {
            if acHit != 0 {
                (*this).colliderBody.base.acFlags =
                    ((*this).colliderBody.base.acFlags as libc::c_int &
                         !((1 as libc::c_int) << 1 as libc::c_int)) as u8_0;
                hurtbox = (*this).colliderBody.info.acHitInfo
            }
            if (*this).flyMode as libc::c_int !=
                   GND_FLY_PAINTING as libc::c_int {
                if acHit != 0 &&
                       (*this).actionFunc !=
                           Some(BossGanondrof_Stunned as
                                    unsafe extern "C" fn(_:
                                                             *mut BossGanondrof,
                                                         _:
                                                             *mut GlobalContext)
                                        -> ()) &&
                       (*hurtbox).toucher.dmgFlags &
                           0x1f8a4 as libc::c_int as libc::c_uint != 0 {
                    Audio_PlayActorSound2(&mut (*this).actor,
                                          (0x800 as libc::c_int -
                                               0x800 as libc::c_int) as
                                              u16_0);
                    osSyncPrintf(b"hit != 0 \n\x00" as *const u8 as
                                     *const libc::c_char);
                } else if (*this).actionFunc !=
                              Some(BossGanondrof_Charge as
                                       unsafe extern "C" fn(_:
                                                                *mut BossGanondrof,
                                                            _:
                                                                *mut GlobalContext)
                                           -> ()) {
                    if (*this).returnCount as libc::c_int == 0 as libc::c_int
                       {
                        let mut dmg: u8_0 = 0;
                        let mut canKill: u8_0 = 0 as libc::c_int as u8_0;
                        let mut dmgFlags: s32 =
                            (*hurtbox).toucher.dmgFlags as s32;
                        if dmgFlags & 0x80 as libc::c_int != 0 { return }
                        dmg = CollisionCheck_GetSwordDamage(dmgFlags);
                        if dmg as libc::c_int == 0 as libc::c_int {
                            dmg = 2 as libc::c_int as u8_0
                        } else { canKill = 1 as libc::c_int as u8_0 };
                        if (*this).actor.colChkInfo.health as s8 as
                               libc::c_int > 2 as libc::c_int ||
                               canKill as libc::c_int != 0 {
                            (*this).actor.colChkInfo.health =
                                ((*this).actor.colChkInfo.health as
                                     libc::c_int - dmg as libc::c_int) as u8_0
                        }
                        if (*this).actor.colChkInfo.health as s8 as
                               libc::c_int <= 0 as libc::c_int {
                            BossGanondrof_SetupDeath(this, globalCtx);
                            Enemy_StartFinishingBlow(globalCtx,
                                                     &mut (*this).actor);
                            return
                        }
                    }
                    BossGanondrof_SetupStunned(this, globalCtx);
                    if (*this).returnCount as libc::c_int >= 2 as libc::c_int
                       {
                        (*this).timers[0 as libc::c_int as usize] =
                            120 as libc::c_int as s16
                    }
                    (*this).work[GND_INVINC_TIMER as libc::c_int as usize] =
                        10 as libc::c_int as s16;
                    (*horse).hitTimer = 20 as libc::c_int as s16;
                    Audio_PlayActorSound2(&mut (*this).actor,
                                          0x38ae as libc::c_int as u16_0);
                } else {
                    Audio_PlayActorSound2(&mut (*this).actor,
                                          (0x800 as libc::c_int -
                                               0x800 as libc::c_int) as
                                              u16_0);
                }
            } else if acHit != 0 &&
                          (*hurtbox).toucher.dmgFlags &
                              0x1f8a4 as libc::c_int as libc::c_uint != 0 {
                (*this).work[GND_INVINC_TIMER as libc::c_int as usize] =
                    10 as libc::c_int as s16;
                (*this).actor.colChkInfo.health =
                    ((*this).actor.colChkInfo.health as libc::c_int -
                         2 as libc::c_int) as u8_0;
                (*horse).hitTimer = 20 as libc::c_int as s16;
                Audio_PlayActorSound2(&mut (*this).actor,
                                      0x38ae as libc::c_int as u16_0);
            }
            (*this).returnCount = 0 as libc::c_int as u8_0
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn BossGanondrof_Update(mut thisx: *mut Actor,
                                              mut globalCtx:
                                                  *mut GlobalContext) {
    let mut cs: f32_0 = 0.;
    let mut sn: f32_0 = 0.;
    let mut legRotTargetY: f32_0 = 0.;
    let mut legRotTargetZ: f32_0 = 0.;
    let mut legSplitTarget: f32_0 = 0.;
    let mut pad2: s32 = 0;
    let mut i: s16 = 0;
    let mut pad: s32 = 0;
    let mut this: *mut BossGanondrof = thisx as *mut BossGanondrof;
    let mut horse: *mut EnfHG = 0 as *mut EnfHG;
    osSyncPrintf(b"MOVE START %d\n\x00" as *const u8 as *const libc::c_char,
                 (*this).actor.params as libc::c_int);
    (*this).actor.flags &=
        !((1 as libc::c_int) << 10 as libc::c_int) as libc::c_uint;
    (*this).colliderBody.base.colType = COLTYPE_HIT3 as libc::c_int as u8_0;
    if (*this).killActor != 0 { Actor_Kill(&mut (*this).actor); return }
    (*this).work[GND_VARIANCE_TIMER as libc::c_int as usize] += 1;
    horse = (*this).actor.child as *mut EnfHG;
    osSyncPrintf(b"MOVE START EEEEEEEEEEEEEEEEEEEEEE%d\n\x00" as *const u8 as
                     *const libc::c_char,
                 (*this).actor.params as libc::c_int);
    (*this).actionFunc.expect("non-null function pointer")(this, globalCtx);
    i = 0 as libc::c_int as s16;
    while (i as libc::c_int) <
              (::std::mem::size_of::<[s16; 5]>() as
                   libc::c_ulong).wrapping_div(::std::mem::size_of::<s16>() as
                                                   libc::c_ulong) as s32 {
        if (*this).timers[i as usize] != 0 { (*this).timers[i as usize] -= 1 }
        i += 1
    }
    if (*this).work[GND_UNKTIMER_1 as libc::c_int as usize] != 0 {
        (*this).work[GND_UNKTIMER_1 as libc::c_int as usize] -= 1
    }
    if (*this).work[GND_UNKTIMER_2 as libc::c_int as usize] != 0 {
        (*this).work[GND_UNKTIMER_2 as libc::c_int as usize] -= 1
    }
    if (*this).actionFunc !=
           Some(BossGanondrof_Death as
                    unsafe extern "C" fn(_: *mut BossGanondrof,
                                         _: *mut GlobalContext) -> ()) {
        BossGanondrof_CollisionCheck(this, globalCtx);
    }
    osSyncPrintf(b"MOVE END\n\x00" as *const u8 as *const libc::c_char);
    BossGanondrof_SetColliderPos(&mut (*this).targetPos,
                                 &mut (*this).colliderBody);
    BossGanondrof_SetColliderPos(&mut (*this).spearTip,
                                 &mut (*this).colliderSpear);
    if (*this).flyMode as libc::c_int == GND_FLY_PAINTING as libc::c_int &&
           (*horse).bossGndInPainting == 0 {
        CollisionCheck_SetAC(globalCtx, &mut (*globalCtx).colChkCtx,
                             &mut (*this).colliderBody.base);
    }
    if (*this).actionFunc ==
           Some(BossGanondrof_Stunned as
                    unsafe extern "C" fn(_: *mut BossGanondrof,
                                         _: *mut GlobalContext) -> ()) &&
           (*this).timers[0 as libc::c_int as usize] as libc::c_int >
               1 as libc::c_int {
        CollisionCheck_SetAC(globalCtx, &mut (*globalCtx).colChkCtx,
                             &mut (*this).colliderBody.base);
        CollisionCheck_SetOC(globalCtx, &mut (*globalCtx).colChkCtx,
                             &mut (*this).colliderBody.base);
    } else if (*this).actionFunc ==
                  Some(BossGanondrof_Block as
                           unsafe extern "C" fn(_: *mut BossGanondrof,
                                                _: *mut GlobalContext) -> ())
     {
        CollisionCheck_SetAC(globalCtx, &mut (*globalCtx).colChkCtx,
                             &mut (*this).colliderBody.base);
    } else if (*this).actionFunc ==
                  Some(BossGanondrof_Charge as
                           unsafe extern "C" fn(_: *mut BossGanondrof,
                                                _: *mut GlobalContext) -> ())
     {
        CollisionCheck_SetAC(globalCtx, &mut (*globalCtx).colChkCtx,
                             &mut (*this).colliderBody.base);
        CollisionCheck_SetAT(globalCtx, &mut (*globalCtx).colChkCtx,
                             &mut (*this).colliderBody.base);
        CollisionCheck_SetAT(globalCtx, &mut (*globalCtx).colChkCtx,
                             &mut (*this).colliderSpear.base);
    }
    (*this).actor.focus.pos = (*this).targetPos;
    sn = Math_SinS(-((*this).actor.shape.rot.y as libc::c_int) as s16);
    cs = Math_CosS(-((*this).actor.shape.rot.y as libc::c_int) as s16);
    legRotTargetY =
        (sn * (*this).actor.velocity.z + cs * (*this).actor.velocity.x) *
            300.0f32;
    legRotTargetZ =
        (-sn * (*this).actor.velocity.x + cs * (*this).actor.velocity.z) *
            300.0f32;
    Math_ApproachF(&mut (*this).legRotY, legRotTargetY, 1.0f32, 600.0f32);
    Math_ApproachF(&mut (*this).legRotZ, legRotTargetZ, 1.0f32, 600.0f32);
    if (*this).flyMode as libc::c_int != GND_FLY_PAINTING as libc::c_int &&
           (*this).actionFunc !=
               Some(BossGanondrof_Stunned as
                        unsafe extern "C" fn(_: *mut BossGanondrof,
                                             _: *mut GlobalContext) -> ()) &&
           (*this).deathState as libc::c_int == NOT_DEAD as libc::c_int {
        legSplitTarget =
            Math_SinS(((*this).work[GND_VARIANCE_TIMER as libc::c_int as
                                        usize] as libc::c_int *
                           0x8dc as libc::c_int) as s16) * -500.0f32 -
                500.0f32
    } else { legSplitTarget = 0.0f32 }
    Math_ApproachF(&mut (*this).legSplitY, legSplitTarget, 1.0f32, 100.0f32);
    if (*this).shockTimer as libc::c_int != 0 as libc::c_int {
        let mut j: s16 = 0;
        (*this).shockTimer = (*this).shockTimer.wrapping_sub(1);
        osSyncPrintf(b"F 1\n\x00" as *const u8 as *const libc::c_char);
        j = 0 as libc::c_int as s16;
        while (j as libc::c_int) < 7 as libc::c_int {
            osSyncPrintf(b"F 15\n\x00" as *const u8 as *const libc::c_char);
            EffectSsFhgFlash_SpawnShock(globalCtx, &mut (*this).actor,
                                        &mut (*this).actor.world.pos,
                                        45 as libc::c_int as s16,
                                        FHGFLASH_SHOCK_PG as libc::c_int as
                                            u8_0);
            j += 1
        }
        osSyncPrintf(b"F 2\n\x00" as *const u8 as *const libc::c_char);
    }
    if (*this).actor.params as libc::c_int == 1 as libc::c_int {
        Lights_PointNoGlowSetInfo(&mut (*this).lightInfo,
                                  (*this).spearTip.x as s16,
                                  (*this).spearTip.y as s16,
                                  (*this).spearTip.z as s16,
                                  255 as libc::c_int as u8_0,
                                  255 as libc::c_int as u8_0,
                                  255 as libc::c_int as u8_0,
                                  200 as libc::c_int as s16);
    };
}
#[no_mangle]
pub unsafe extern "C" fn BossGanondrof_OverrideLimbDraw(mut globalCtx:
                                                            *mut GlobalContext,
                                                        mut limbIndex: s32,
                                                        mut dList:
                                                            *mut *mut Gfx,
                                                        mut pos: *mut Vec3f,
                                                        mut rot: *mut Vec3s,
                                                        mut thisx:
                                                            *mut libc::c_void)
 -> s32 {
    let mut this: *mut BossGanondrof = thisx as *mut BossGanondrof;
    let mut current_block_25: u64;
    match limbIndex {
        15 => {
            if (*this).actionFunc ==
                   Some(BossGanondrof_Intro as
                            unsafe extern "C" fn(_: *mut BossGanondrof,
                                                 _: *mut GlobalContext) -> ())
                   &&
                   (*this).work[GND_MASK_OFF as libc::c_int as usize] as
                       libc::c_int != 0 {
                *dList = gPhantomGanonFaceDL.as_mut_ptr()
            }
            (*rot).y =
                ((*rot).y as libc::c_float +
                     (*this).rideRotY[limbIndex as usize]) as s16;
            (*rot).z =
                ((*rot).z as libc::c_float +
                     (*this).rideRotZ[limbIndex as usize]) as s16;
            current_block_25 = 17500079516916021833;
        }
        19 => {
            (*rot).y =
                ((*rot).y as libc::c_float +
                     ((*this).legRotY + (*this).legSplitY)) as s16;
            (*rot).z = ((*rot).z as libc::c_float + (*this).legRotZ) as s16;
            current_block_25 = 17500079516916021833;
        }
        20 => {
            (*rot).y =
                ((*rot).y as libc::c_float +
                     ((*this).legRotY + (*this).legSplitY)) as s16;
            (*rot).z = ((*rot).z as libc::c_float + (*this).legRotZ) as s16;
            current_block_25 = 17500079516916021833;
        }
        21 => {
            (*rot).y =
                ((*rot).y as libc::c_float +
                     ((*this).legRotY + (*this).legSplitY)) as s16;
            (*rot).z = ((*rot).z as libc::c_float + (*this).legRotZ) as s16;
            current_block_25 = 17500079516916021833;
        }
        23 => {
            (*rot).y =
                ((*rot).y as libc::c_float +
                     ((*this).legRotY - (*this).legSplitY)) as s16;
            (*rot).z = ((*rot).z as libc::c_float + (*this).legRotZ) as s16;
            current_block_25 = 17500079516916021833;
        }
        24 => {
            (*rot).y =
                ((*rot).y as libc::c_float +
                     ((*this).legRotY - (*this).legSplitY)) as s16;
            (*rot).z = ((*rot).z as libc::c_float + (*this).legRotZ) as s16;
            current_block_25 = 17500079516916021833;
        }
        25 => {
            (*rot).y =
                ((*rot).y as libc::c_float +
                     ((*this).legRotY - (*this).legSplitY)) as s16;
            (*rot).z = ((*rot).z as libc::c_float + (*this).legRotZ) as s16;
            current_block_25 = 17500079516916021833;
        }
        5 | 6 | 7 => {
            (*rot).y = ((*rot).y as libc::c_float + (*this).armRotY) as s16;
            (*rot).z = ((*rot).z as libc::c_float + (*this).armRotZ) as s16;
            current_block_25 = 17500079516916021833;
        }
        8 | 9 | 10 => {
            (*rot).y = ((*rot).y as libc::c_float + (*this).armRotY) as s16;
            (*rot).z = ((*rot).z as libc::c_float + (*this).armRotZ) as s16;
            current_block_25 = 17500079516916021833;
        }
        13 => {
            if (*this).deathState as libc::c_int != NOT_DEAD as libc::c_int {
                *dList = 0 as *mut Gfx
            }
            current_block_25 = 11153037792046980663;
        }
        _ => { current_block_25 = 11153037792046980663; }
    }
    match current_block_25 {
        11153037792046980663 => {
            (*rot).y =
                ((*rot).y as libc::c_float +
                     (*this).rideRotY[limbIndex as usize]) as s16;
            (*rot).z =
                ((*rot).z as libc::c_float +
                     (*this).rideRotZ[limbIndex as usize]) as s16
        }
        _ => { }
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn BossGanondrof_PostLimbDraw(mut globalCtx:
                                                        *mut GlobalContext,
                                                    mut limbIndex: s32,
                                                    mut dList: *mut *mut Gfx,
                                                    mut rot: *mut Vec3s,
                                                    mut thisx:
                                                        *mut libc::c_void) {
    static mut zeroVec: Vec3f =
        { let mut init = Vec3f{x: 0.0f32, y: 0.0f32, z: 0.0f32,}; init };
    static mut spearVec: Vec3f =
        { let mut init = Vec3f{x: 0.0f32, y: 0.0f32, z: 6000.0f32,}; init };
    let mut this: *mut BossGanondrof = thisx as *mut BossGanondrof;
    if limbIndex == 14 as libc::c_int {
        Matrix_MultVec3f(&mut zeroVec, &mut (*this).targetPos);
    } else if limbIndex == 13 as libc::c_int {
        Matrix_MultVec3f(&mut spearVec, &mut (*this).spearTip);
    }
    if ((*this).flyMode as libc::c_int != GND_FLY_PAINTING as libc::c_int ||
            (*this).actionFunc ==
                Some(BossGanondrof_Intro as
                         unsafe extern "C" fn(_: *mut BossGanondrof,
                                              _: *mut GlobalContext) -> ()))
           && limbIndex <= 25 as libc::c_int {
        Matrix_MultVec3f(&mut zeroVec,
                         &mut *(*this).bodyPartsPos.as_mut_ptr().offset((limbIndex
                                                                             -
                                                                             1
                                                                                 as
                                                                                 libc::c_int)
                                                                            as
                                                                            isize));
    };
}
#[no_mangle]
pub unsafe extern "C" fn BossGanondrof_GetClearPixelDList(mut gfxCtx:
                                                              *mut GraphicsContext)
 -> *mut Gfx {
    let mut dList: *mut Gfx =
        Graph_Alloc(gfxCtx,
                    (::std::mem::size_of::<Gfx>() as
                         libc::c_ulong).wrapping_mul(4 as libc::c_int as
                                                         libc::c_uint) as
                        size_t) as *mut Gfx;
    let mut dListHead: *mut Gfx = dList;
    let fresh0 = dListHead;
    dListHead = dListHead.offset(1);
    let mut _g: *mut Gfx = fresh0;
    (*_g).words.w0 =
        (0xe7 as libc::c_int as u32_0 &
             (((0x1 as libc::c_int) << 8 as libc::c_int) - 1 as libc::c_int)
                 as libc::c_uint) << 24 as libc::c_int;
    (*_g).words.w1 = 0 as libc::c_int as libc::c_uint;
    let fresh1 = dListHead;
    dListHead = dListHead.offset(1);
    let mut _g_0: *mut Gfx = fresh1;
    (*_g_0).words.w0 =
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
    (*_g_0).words.w1 =
        ((3 as libc::c_int) << 30 as libc::c_int |
             (2 as libc::c_int) << 26 as libc::c_int |
             (0 as libc::c_int) << 22 as libc::c_int |
             (0 as libc::c_int) << 18 as libc::c_int |
             (0x8 as libc::c_int | 0x10 as libc::c_int | 0x20 as libc::c_int |
                  0x40 as libc::c_int | 0 as libc::c_int |
                  0x1000 as libc::c_int | 0x2000 as libc::c_int |
                  0 as libc::c_int | 0 as libc::c_int |
                  (0 as libc::c_int) << 28 as libc::c_int |
                  (0 as libc::c_int) << 24 as libc::c_int |
                  (1 as libc::c_int) << 20 as libc::c_int |
                  (1 as libc::c_int) << 16 as libc::c_int)) as libc::c_uint;
    let fresh2 = dListHead;
    dListHead = dListHead.offset(1);
    let mut _g_1: *mut Gfx = fresh2;
    (*_g_1).words.w0 =
        (0xd9 as libc::c_int as u32_0 &
             (((0x1 as libc::c_int) << 8 as libc::c_int) - 1 as libc::c_int)
                 as libc::c_uint) << 24 as libc::c_int |
            (!(0x400 as libc::c_int as u32_0) &
                 (((0x1 as libc::c_int) << 24 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 0 as libc::c_int;
    (*_g_1).words.w1 = 0 as libc::c_int as u32_0;
    let fresh3 = dListHead;
    dListHead = dListHead.offset(1);
    let mut _g_2: *mut Gfx = fresh3;
    (*_g_2).words.w0 =
        (0xdf as libc::c_int as u32_0 &
             (((0x1 as libc::c_int) << 8 as libc::c_int) - 1 as libc::c_int)
                 as libc::c_uint) << 24 as libc::c_int;
    (*_g_2).words.w1 = 0 as libc::c_int as libc::c_uint;
    return dList;
}
#[no_mangle]
pub unsafe extern "C" fn BossGanondrof_GetNullDList(mut gfxCtx:
                                                        *mut GraphicsContext)
 -> *mut Gfx {
    let mut dList: *mut Gfx =
        Graph_Alloc(gfxCtx,
                    (::std::mem::size_of::<Gfx>() as
                         libc::c_ulong).wrapping_mul(1 as libc::c_int as
                                                         libc::c_uint) as
                        size_t) as *mut Gfx;
    let mut dListHead: *mut Gfx = dList;
    let fresh4 = dListHead;
    dListHead = dListHead.offset(1);
    let mut _g: *mut Gfx = fresh4;
    (*_g).words.w0 =
        (0xdf as libc::c_int as u32_0 &
             (((0x1 as libc::c_int) << 8 as libc::c_int) - 1 as libc::c_int)
                 as libc::c_uint) << 24 as libc::c_int;
    (*_g).words.w1 = 0 as libc::c_int as libc::c_uint;
    return dList;
}
#[no_mangle]
pub unsafe extern "C" fn BossGanondrof_Draw(mut thisx: *mut Actor,
                                            mut globalCtx:
                                                *mut GlobalContext) {
    let mut pad: s32 = 0;
    let mut this: *mut BossGanondrof = thisx as *mut BossGanondrof;
    let mut horse: *mut EnfHG = 0 as *mut EnfHG;
    let mut __gfxCtx: *mut GraphicsContext = 0 as *mut GraphicsContext;
    let mut dispRefs: [*mut Gfx; 4] = [0 as *mut Gfx; 4];
    __gfxCtx = (*globalCtx).state.gfxCtx;
    Graph_OpenDisps(dispRefs.as_mut_ptr(), (*globalCtx).state.gfxCtx,
                    b"../z_boss_ganondrof.c\x00" as *const u8 as
                        *const libc::c_char, 3716 as libc::c_int);
    osSyncPrintf(b"MOVE P = %x\n\x00" as *const u8 as *const libc::c_char,
                 (*this).actor.update);
    osSyncPrintf(b"STOP TIMER = %d ==============\n\x00" as *const u8 as
                     *const libc::c_char,
                 (*this).actor.freezeTimer as libc::c_int);
    horse = (*this).actor.child as *mut EnfHG;
    if (*this).flyMode as libc::c_int == GND_FLY_PAINTING as libc::c_int {
        Matrix_RotateY((*horse).turnRot as libc::c_int as libc::c_float *
                           3.1416f32 / 0x8000 as libc::c_int as f32_0,
                       MTXMODE_APPLY as libc::c_int as u8_0);
    }
    osSyncPrintf(b"YP %f\n\x00" as *const u8 as *const libc::c_char,
                 (*this).actor.world.pos.y as libc::c_double);
    func_80093D18((*globalCtx).state.gfxCtx);
    if (*this).work[GND_INVINC_TIMER as libc::c_int as usize] as libc::c_int &
           4 as libc::c_int != 0 {
        (*__gfxCtx).polyOpa.p =
            Gfx_SetFog((*__gfxCtx).polyOpa.p, 255 as libc::c_int,
                       50 as libc::c_int, 0 as libc::c_int, 0 as libc::c_int,
                       900 as libc::c_int, 1099 as libc::c_int)
    } else {
        (*__gfxCtx).polyOpa.p =
            Gfx_SetFog((*__gfxCtx).polyOpa.p,
                       (*horse).warpColorFilterR as u32_0 as s32,
                       (*horse).warpColorFilterG as u32_0 as s32,
                       (*horse).warpColorFilterB as u32_0 as s32,
                       0 as libc::c_int,
                       (*horse).warpColorFilterUnk1 as s32 +
                           995 as libc::c_int,
                       (*horse).warpColorFilterUnk2 as s32 +
                           1000 as libc::c_int)
    }
    osSyncPrintf(b"DRAW 11\n\x00" as *const u8 as *const libc::c_char);
    osSyncPrintf(b"EYE_COL %d\n\x00" as *const u8 as *const libc::c_char,
                 (*this).fwork[GND_EYE_BRIGHTNESS as libc::c_int as usize] as
                     s16 as libc::c_int);
    let fresh5 = (*__gfxCtx).polyOpa.p;
    (*__gfxCtx).polyOpa.p = (*__gfxCtx).polyOpa.p.offset(1);
    let mut _g: *mut Gfx = fresh5;
    (*_g).words.w0 =
        (0xfb as libc::c_int as u32_0 &
             (((0x1 as libc::c_int) << 8 as libc::c_int) - 1 as libc::c_int)
                 as libc::c_uint) << 24 as libc::c_int;
    (*_g).words.w1 =
        ((*this).fwork[GND_EYE_BRIGHTNESS as libc::c_int as usize] as s16 as
             u32_0 &
             (((0x1 as libc::c_int) << 8 as libc::c_int) - 1 as libc::c_int)
                 as libc::c_uint) << 24 as libc::c_int |
            ((*this).fwork[GND_EYE_BRIGHTNESS as libc::c_int as usize] as s16
                 as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 16 as libc::c_int
            |
            ((*this).fwork[GND_EYE_BRIGHTNESS as libc::c_int as usize] as s16
                 as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 8 as libc::c_int |
            ((*this).fwork[GND_EYE_ALPHA as libc::c_int as usize] as s16 as
                 u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 0 as libc::c_int;
    if (*this).work[GND_BODY_DECAY_FLAG as libc::c_int as usize] != 0 {
        let fresh6 = (*__gfxCtx).polyOpa.p;
        (*__gfxCtx).polyOpa.p = (*__gfxCtx).polyOpa.p.offset(1);
        let mut _g_0: *mut Gfx = fresh6;
        (*_g_0).words.w0 =
            (0xdb as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 24 as libc::c_int
                |
                (0x6 as libc::c_int as u32_0 &
                     (((0x1 as libc::c_int) << 8 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    16 as libc::c_int |
                ((0x8 as libc::c_int * 4 as libc::c_int) as u32_0 &
                     (((0x1 as libc::c_int) << 16 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    0 as libc::c_int;
        (*_g_0).words.w1 =
            BossGanondrof_GetClearPixelDList((*globalCtx).state.gfxCtx) as
                libc::c_uint
    } else {
        let fresh7 = (*__gfxCtx).polyOpa.p;
        (*__gfxCtx).polyOpa.p = (*__gfxCtx).polyOpa.p.offset(1);
        let mut _g_1: *mut Gfx = fresh7;
        (*_g_1).words.w0 =
            (0xdb as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 24 as libc::c_int
                |
                (0x6 as libc::c_int as u32_0 &
                     (((0x1 as libc::c_int) << 8 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    16 as libc::c_int |
                ((0x8 as libc::c_int * 4 as libc::c_int) as u32_0 &
                     (((0x1 as libc::c_int) << 16 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    0 as libc::c_int;
        (*_g_1).words.w1 =
            BossGanondrof_GetNullDList((*globalCtx).state.gfxCtx) as
                libc::c_uint
    }
    SkelAnime_DrawOpa(globalCtx, (*this).skelAnime.skeleton,
                      (*this).skelAnime.jointTable,
                      Some(BossGanondrof_OverrideLimbDraw as
                               unsafe extern "C" fn(_: *mut GlobalContext,
                                                    _: s32, _: *mut *mut Gfx,
                                                    _: *mut Vec3f,
                                                    _: *mut Vec3s,
                                                    _: *mut libc::c_void)
                                   -> s32),
                      Some(BossGanondrof_PostLimbDraw as
                               unsafe extern "C" fn(_: *mut GlobalContext,
                                                    _: s32, _: *mut *mut Gfx,
                                                    _: *mut Vec3s,
                                                    _: *mut libc::c_void)
                                   -> ()), this as *mut libc::c_void);
    osSyncPrintf(b"DRAW 22\n\x00" as *const u8 as *const libc::c_char);
    (*__gfxCtx).polyOpa.p = Gameplay_SetFog(globalCtx, (*__gfxCtx).polyOpa.p);
    Graph_CloseDisps(dispRefs.as_mut_ptr(), (*globalCtx).state.gfxCtx,
                     b"../z_boss_ganondrof.c\x00" as *const u8 as
                         *const libc::c_char, 3814 as libc::c_int);
    osSyncPrintf(b"DRAW END %d\n\x00" as *const u8 as *const libc::c_char,
                 (*this).actor.params as libc::c_int);
}
unsafe extern "C" fn run_static_initializers() {
    sInitChain =
        [{
             let mut init = InitChainEntry{cont_type_0_offset_value: [0; 4],};
             init.set_cont(1 as libc::c_int as u32_0);
             init.set_type_0(ICHAINTYPE_U8 as libc::c_int as u32_0);
             init.set_offset(&mut (*(0 as *mut Actor)).targetMode as *mut s8
                                 as size_t as u32_0);
             init.set_value(5 as libc::c_int);
             init
         },
         {
             let mut init = InitChainEntry{cont_type_0_offset_value: [0; 4],};
             init.set_cont(1 as libc::c_int as u32_0);
             init.set_type_0(ICHAINTYPE_S8 as libc::c_int as u32_0);
             init.set_offset(&mut (*(0 as *mut Actor)).naviEnemyId as
                                 *mut u8_0 as size_t as u32_0);
             init.set_value(0x2b as libc::c_int);
             init
         },
         {
             let mut init = InitChainEntry{cont_type_0_offset_value: [0; 4],};
             init.set_cont(1 as libc::c_int as u32_0);
             init.set_type_0(ICHAINTYPE_F32_DIV1000 as libc::c_int as u32_0);
             init.set_offset(&mut (*(0 as *mut Actor)).gravity as *mut f32_0
                                 as size_t as u32_0);
             init.set_value(0 as libc::c_int);
             init
         },
         {
             let mut init = InitChainEntry{cont_type_0_offset_value: [0; 4],};
             init.set_cont(0 as libc::c_int as u32_0);
             init.set_type_0(ICHAINTYPE_F32 as libc::c_int as u32_0);
             init.set_offset(&mut (*(0 as *mut Actor)).targetArrowOffset as
                                 *mut f32_0 as size_t as u32_0);
             init.set_value(0 as libc::c_int);
             init
         }]
}
#[used]
#[cfg_attr(target_os = "linux", link_section = ".init_array")]
#[cfg_attr(target_os = "windows", link_section = ".CRT$XIB")]
#[cfg_attr(target_os = "macos", link_section = "__DATA,__mod_init_func")]
static INIT_ARRAY: [unsafe extern "C" fn(); 1] = [run_static_initializers];
