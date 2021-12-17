#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case,
         non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(const_raw_ptr_to_usize_cast, const_transmute, register_tool)]
extern "C" {
    #[no_mangle]
    fn Item_DropCollectibleRandom(globalCtx: *mut GlobalContext,
                                  fromActor: *mut Actor, spawnPos: *mut Vec3f,
                                  params: s16);
    #[no_mangle]
    fn EffectBlure_AddVertex(this: *mut EffectBlure, p1: *mut Vec3f,
                             p2: *mut Vec3f);
    #[no_mangle]
    fn EffectBlure_AddSpace(this: *mut EffectBlure);
    #[no_mangle]
    fn Effect_GetByIndex(index: s32) -> *mut libc::c_void;
    #[no_mangle]
    fn Effect_Add(globalCtx: *mut GlobalContext, pIndex: *mut s32,
                  type_0: s32, arg3: u8_0, arg4: u8_0,
                  initParams: *mut libc::c_void);
    #[no_mangle]
    fn Effect_Delete(globalCtx: *mut GlobalContext, index: s32);
    #[no_mangle]
    fn EffectSsEnIce_SpawnFlyingVec3f(globalCtx: *mut GlobalContext,
                                      actor: *mut Actor, pos: *mut Vec3f,
                                      primR: s16, primG: s16, primB: s16,
                                      primA: s16, envR: s16, envG: s16,
                                      envB: s16, scale: f32_0);
    #[no_mangle]
    fn ActorShape_Init(shape: *mut ActorShape, yOffset: f32_0,
                       shadowDraw: ActorShadowFunc, shadowScale: f32_0);
    #[no_mangle]
    fn ActorShadow_DrawFeet(actor: *mut Actor, lights: *mut Lights,
                            globalCtx: *mut GlobalContext);
    #[no_mangle]
    fn Actor_SetFeetPos(actor: *mut Actor, limbIndex: s32, leftFootIndex: s32,
                        leftFootPos: *mut Vec3f, rightFootIndex: s32,
                        rightFootPos: *mut Vec3f);
    #[no_mangle]
    fn Flags_GetSwitch(globalCtx: *mut GlobalContext, flag: s32) -> s32;
    #[no_mangle]
    fn Flags_SetSwitch(globalCtx: *mut GlobalContext, flag: s32);
    #[no_mangle]
    fn Actor_Kill(actor: *mut Actor);
    #[no_mangle]
    fn Actor_MoveForward(actor: *mut Actor);
    #[no_mangle]
    fn Actor_WorldYawTowardActor(actorA: *mut Actor, actorB: *mut Actor)
     -> s16;
    #[no_mangle]
    fn Actor_WorldYawTowardPoint(actor: *mut Actor, refPoint: *mut Vec3f)
     -> s16;
    #[no_mangle]
    fn Actor_WorldDistXYZToActor(actorA: *mut Actor, actorB: *mut Actor)
     -> f32_0;
    #[no_mangle]
    fn Actor_WorldDistXZToPoint(actor: *mut Actor, refPoint: *mut Vec3f)
     -> f32_0;
    #[no_mangle]
    fn Actor_IsFacingPlayer(actor: *mut Actor, angle: s16) -> s32;
    #[no_mangle]
    fn Actor_UpdateBgCheckInfo(globalCtx: *mut GlobalContext,
                               actor: *mut Actor, wallCheckHeight: f32_0,
                               wallCheckRadius: f32_0,
                               ceilingCheckHeight: f32_0, flags: s32);
    #[no_mangle]
    fn func_8002EBCC(actor: *mut Actor, globalCtx: *mut GlobalContext,
                     flag: s32);
    #[no_mangle]
    fn Audio_PlayActorSound2(actor: *mut Actor, sfxId: u16_0);
    #[no_mangle]
    fn Enemy_StartFinishingBlow(globalCtx: *mut GlobalContext,
                                actor: *mut Actor);
    #[no_mangle]
    fn Actor_SpawnFloorDustRing(globalCtx: *mut GlobalContext,
                                actor: *mut Actor, posXZ: *mut Vec3f,
                                radius: f32_0, amountMinusOne: s32,
                                randAccelWeight: f32_0, scale: s16,
                                scaleStep: s16, useLighting: u8_0);
    #[no_mangle]
    fn Actor_GetProjectileActor(globalCtx: *mut GlobalContext,
                                refActor: *mut Actor, radius: f32_0)
     -> *mut Actor;
    #[no_mangle]
    fn Actor_ChangeCategory(globalCtx: *mut GlobalContext,
                            actorCtx: *mut ActorContext, actor: *mut Actor,
                            actorCategory: u8_0);
    #[no_mangle]
    fn Actor_TestFloorInDirection(actor: *mut Actor,
                                  globalCtx: *mut GlobalContext,
                                  distance: f32_0, angle: s16) -> s16;
    #[no_mangle]
    fn Actor_IsTargeted(globalCtx: *mut GlobalContext, actor: *mut Actor)
     -> s32;
    #[no_mangle]
    fn Actor_OtherIsTargeted(globalCtx: *mut GlobalContext, actor: *mut Actor)
     -> s32;
    #[no_mangle]
    fn Rand_CenteredFloat(f: f32_0) -> f32_0;
    #[no_mangle]
    fn Actor_SetColorFilter(actor: *mut Actor, colorFlag: s16,
                            colorIntensityMax: s16, xluFlag: s16,
                            duration: s16);
    #[no_mangle]
    fn Actor_FindNearby(globalCtx: *mut GlobalContext, refActor: *mut Actor,
                        actorId: s16, actorCategory: u8_0, range: f32_0)
     -> *mut Actor;
    #[no_mangle]
    fn func_800354B4(globalCtx: *mut GlobalContext, actor: *mut Actor,
                     range: f32_0, arg3: s16, arg4: s16, arg5: s16) -> s32;
    #[no_mangle]
    fn func_800355B8(globalCtx: *mut GlobalContext, pos: *mut Vec3f);
    #[no_mangle]
    fn Actor_ApplyDamage(actor: *mut Actor) -> u8_0;
    #[no_mangle]
    fn Actor_SetDropFlag(actor: *mut Actor, colBody: *mut ColliderInfo,
                         freezeFlag: s32);
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
    fn Collider_InitQuad(globalCtx: *mut GlobalContext,
                         collider: *mut ColliderQuad) -> s32;
    #[no_mangle]
    fn Collider_DestroyQuad(globalCtx: *mut GlobalContext,
                            collider: *mut ColliderQuad) -> s32;
    #[no_mangle]
    fn Collider_SetQuad(globalCtx: *mut GlobalContext,
                        collider: *mut ColliderQuad, actor: *mut Actor,
                        src: *mut ColliderQuadInit) -> s32;
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
    fn Collider_UpdateCylinder(actor: *mut Actor,
                               collider: *mut ColliderCylinder);
    #[no_mangle]
    fn Collider_SetQuadVertices(collider: *mut ColliderQuad, a: *mut Vec3f,
                                b: *mut Vec3f, c: *mut Vec3f, d: *mut Vec3f);
    #[no_mangle]
    fn Math_CosS(angle: s16) -> f32_0;
    #[no_mangle]
    fn Math_SinS(angle: s16) -> f32_0;
    #[no_mangle]
    fn Math_Vec3f_DistXYZ(a: *mut Vec3f, b: *mut Vec3f) -> f32_0;
    #[no_mangle]
    fn Math_Vec3f_Yaw(a: *mut Vec3f, b: *mut Vec3f) -> s16;
    #[no_mangle]
    fn Actor_ProcessInitChain(actor: *mut Actor,
                              initChain: *mut InitChainEntry);
    #[no_mangle]
    fn Math_SmoothStepToF(pValue: *mut f32_0, target: f32_0, fraction: f32_0,
                          step: f32_0, minStep: f32_0) -> f32_0;
    #[no_mangle]
    fn Math_SmoothStepToS(pValue: *mut s16, target: s16, scale: s16,
                          step: s16, minStep: s16) -> s16;
    #[no_mangle]
    fn func_80093D18(gfxCtx: *mut GraphicsContext);
    #[no_mangle]
    fn func_80093D84(gfxCtx: *mut GraphicsContext);
    #[no_mangle]
    fn Animation_GetLastFrame(animation: *mut libc::c_void) -> s16;
    #[no_mangle]
    fn SkelAnime_Draw(globalCtx: *mut GlobalContext,
                      skeleton: *mut *mut libc::c_void,
                      jointTable: *mut Vec3s,
                      overrideLimbDraw: OverrideLimbDraw,
                      postLimbDraw: PostLimbDraw, arg: *mut libc::c_void,
                      gfx: *mut Gfx) -> *mut Gfx;
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
    fn Animation_PlayOnceSetSpeed(skelAnime: *mut SkelAnime,
                                  animation: *mut AnimationHeader,
                                  playSpeed: f32_0);
    #[no_mangle]
    fn Animation_PlayLoop(skelAnime: *mut SkelAnime,
                          animation: *mut AnimationHeader);
    #[no_mangle]
    fn Animation_MorphToLoop(skelAnime: *mut SkelAnime,
                             animation: *mut AnimationHeader,
                             morphFrames: f32_0);
    #[no_mangle]
    fn Graph_OpenDisps(dispRefs: *mut *mut Gfx, gfxCtx: *mut GraphicsContext,
                       file: *const libc::c_char, line: s32);
    #[no_mangle]
    fn Graph_CloseDisps(dispRefs: *mut *mut Gfx, gfxCtx: *mut GraphicsContext,
                        file: *const libc::c_char, line: s32);
    #[no_mangle]
    fn Matrix_MultVec3f(src: *mut Vec3f, dest: *mut Vec3f);
    #[no_mangle]
    fn func_800F5ACC(bgmID: u16_0);
    #[no_mangle]
    fn func_800F5B58();
    #[no_mangle]
    fn Rand_ZeroOne() -> f32_0;
    #[no_mangle]
    static mut gGameInfo: *mut GameInfo;
    #[no_mangle]
    static mut D_80116280: [Gfx; 0];
    #[no_mangle]
    static mut gZfDinolfosSkel: SkeletonHeader;
    #[no_mangle]
    static mut gZfWalkingAnim: AnimationHeader;
    #[no_mangle]
    static mut gZfLandingAnim: AnimationHeader;
    #[no_mangle]
    static mut gZfJumpingAnim: AnimationHeader;
    #[no_mangle]
    static mut gZfSlashAnim: AnimationHeader;
    #[no_mangle]
    static mut gZfCryingAnim: AnimationHeader;
    #[no_mangle]
    static mut gZfEmptyHandDL: [Gfx; 0];
    #[no_mangle]
    static mut gZfSheathedSwordDL: [Gfx; 0];
    #[no_mangle]
    static mut gZfLizalfosSkel: SkeletonHeader;
    #[no_mangle]
    static mut gZfHopCrouchingAnim: AnimationHeader;
    #[no_mangle]
    static mut gZfHopLeapingAnim: AnimationHeader;
    #[no_mangle]
    static mut gZfHopLandingAnim: AnimationHeader;
    #[no_mangle]
    static mut gZfSheathingSwordAnim: AnimationHeader;
    #[no_mangle]
    static mut gZfDrawingSwordAnim: AnimationHeader;
    #[no_mangle]
    static mut gZfDyingAnim: AnimationHeader;
    #[no_mangle]
    static mut gZfKnockedBackAnim: AnimationHeader;
    #[no_mangle]
    static mut gZfSidesteppingAnim: AnimationHeader;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct EffectBlureElement {
    pub state: s32,
    pub timer: s32,
    pub p1: Vec3s,
    pub p2: Vec3s,
    pub flags: u16_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct EffectBlureInit1 {
    pub unk_00: [libc::c_char; 388],
    pub p1StartColor: [u8_0; 4],
    pub p2StartColor: [u8_0; 4],
    pub p1EndColor: [u8_0; 4],
    pub p2EndColor: [u8_0; 4],
    pub elemDuration: s32,
    pub unkFlag: s32,
    pub calcMode: s32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct EffectBlure {
    pub elements: [EffectBlureElement; 16],
    pub calcMode: s32,
    pub mode4Param: f32_0,
    pub flags: u16_0,
    pub addAngleChange: s16,
    pub addAngle: s16,
    pub p1StartColor: Color_RGBA8,
    pub p2StartColor: Color_RGBA8,
    pub p1EndColor: Color_RGBA8,
    pub p2EndColor: Color_RGBA8,
    pub numElements: u8_0,
    pub elemDuration: u8_0,
    pub unkFlag: u8_0,
    pub drawMode: u8_0,
    pub altPrimColor: Color_RGBA8,
    pub altEnvColor: Color_RGBA8,
}
pub type C2RustUnnamed_21 = libc::c_uint;
pub const EFFECT_SHIELD_PARTICLE: C2RustUnnamed_21 = 3;
pub const EFFECT_BLURE2: C2RustUnnamed_21 = 2;
pub const EFFECT_BLURE1: C2RustUnnamed_21 = 1;
pub const EFFECT_SPARK: C2RustUnnamed_21 = 0;
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
pub struct EnZf {
    pub actor: Actor,
    pub skelAnime: SkelAnime,
    pub jointTable: [Vec3s; 49],
    pub morphTable: [Vec3s; 49],
    pub action: s32,
    pub unk_3E0: s32,
    pub hopAnimIndex: s32,
    pub actionFunc: EnZfActionFunc,
    pub headRot: s16,
    pub headRotTemp: s16,
    pub unk_3F0: s32,
    pub unk_3F4: s16,
    pub iceTimer: s16,
    pub unk_3F8: s16,
    pub swordSheathed: s16,
    pub clearFlag: s16,
    pub curPlatform: s16,
    pub homePlatform: s16,
    pub nextPlatform: s16,
    pub alpha: u8_0,
    pub unk_408: f32_0,
    pub unk_40C: f32_0,
    pub damageEffect: u8_0,
    pub blureIndex: s32,
    pub bodyCollider: ColliderCylinder,
    pub swordCollider: ColliderQuad,
    pub rightFootPos: Vec3f,
    pub leftFootPos: Vec3f,
    pub bodyPartsPos: [Vec3f; 9],
}
pub type EnZfActionFunc
    =
    Option<unsafe extern "C" fn(_: *mut EnZf, _: *mut GlobalContext) -> ()>;
pub type C2RustUnnamed_23 = libc::c_int;
pub const ENZF_TYPE_LIZALFOS_MINIBOSS_B: C2RustUnnamed_23 = 1;
pub const ENZF_TYPE_LIZALFOS_MINIBOSS_A: C2RustUnnamed_23 = 0;
pub const ENZF_TYPE_LIZALFOS_LONE: C2RustUnnamed_23 = -1;
pub const ENZF_TYPE_DINOLFOS: C2RustUnnamed_23 = -2;
pub type C2RustUnnamed_24 = libc::c_uint;
pub const ENZF_ACTION_JUMP_UP: C2RustUnnamed_24 = 22;
pub const ENZF_ACTION_DRAW_SWORD: C2RustUnnamed_24 = 21;
pub const ENZF_ACTION_HOP_AND_TAUNT: C2RustUnnamed_24 = 20;
pub const ENZF_ACTION_HOP_AWAY: C2RustUnnamed_24 = 19;
pub const ENZF_ACTION_SHEATHE_SWORD: C2RustUnnamed_24 = 18;
pub const ENZF_ACTION_DAMAGED: C2RustUnnamed_24 = 16;
pub const ENZF_ACTION_DIE: C2RustUnnamed_24 = 15;
pub const ENZF_ACTION_STUNNED: C2RustUnnamed_24 = 14;
pub const ENZF_ACTION_JUMP_FORWARD: C2RustUnnamed_24 = 13;
pub const ENZF_ACTION_CIRCLE_AROUND_PLAYER: C2RustUnnamed_24 = 12;
pub const ENZF_ACTION_JUMP_BACK: C2RustUnnamed_24 = 11;
pub const ENZF_ACTION_SLASH: C2RustUnnamed_24 = 9;
pub const ENZF_ACTION_RECOIL_FROM_BLOCKED_SLASH: C2RustUnnamed_24 = 8;
pub const ENZF_ACTION_7: C2RustUnnamed_24 = 7;
pub const ENZF_ACTION_6: C2RustUnnamed_24 = 6;
pub const ENZF_ACTION_APPROACH_PLAYER: C2RustUnnamed_24 = 5;
pub const ENZF_ACTION_3: C2RustUnnamed_24 = 3;
pub const ENZF_ACTION_DROP_IN: C2RustUnnamed_24 = 0;
pub type C2RustUnnamed_25 = libc::c_uint;
pub const ENZF_LIMB_MAX: C2RustUnnamed_25 = 49;
pub const ENZF_LIMB_TRUNK: C2RustUnnamed_25 = 48;
pub const ENZF_LIMB_LEFT_THIGH: C2RustUnnamed_25 = 47;
pub const ENZF_LIMB_LEFT_SHIN: C2RustUnnamed_25 = 46;
pub const ENZF_LIMB_LEFT_FOOT: C2RustUnnamed_25 = 45;
pub const ENZF_LIMB_LEFT_FOOT_ROOT: C2RustUnnamed_25 = 44;
pub const ENZF_LIMB_LEFT_SHIN_ROOT: C2RustUnnamed_25 = 43;
pub const ENZF_LIMB_LEFT_THIGH_ROOT: C2RustUnnamed_25 = 42;
pub const ENZF_LIMB_LEFT_LEG_ROOT: C2RustUnnamed_25 = 41;
pub const ENZF_LIMB_RIGHT_THIGH: C2RustUnnamed_25 = 40;
pub const ENZF_LIMB_RIGHT_SHIN: C2RustUnnamed_25 = 39;
pub const ENZF_LIMB_RIGHT_FOOT: C2RustUnnamed_25 = 38;
pub const ENZF_LIMB_RIGHT_FOOT_ROOT: C2RustUnnamed_25 = 37;
pub const ENZF_LIMB_RIGHT_SHIN_ROOT: C2RustUnnamed_25 = 36;
pub const ENZF_LIMB_RIGHT_THIGH_ROOT: C2RustUnnamed_25 = 35;
pub const ENZF_LIMB_RIGHT_LEG_ROOT: C2RustUnnamed_25 = 34;
pub const ENZF_LIMB_SCABBARD: C2RustUnnamed_25 = 33;
pub const ENZF_LIMB_SCABBARD_ROOT: C2RustUnnamed_25 = 32;
pub const ENZF_LIMB_SCABBARD_ROOT_ROOT: C2RustUnnamed_25 = 31;
pub const ENZF_LIMB_TAIL_BASE: C2RustUnnamed_25 = 30;
pub const ENZF_LIMB_TAIL_TIP: C2RustUnnamed_25 = 29;
pub const ENZF_LIMB_TAIL_TIP_ROOT: C2RustUnnamed_25 = 28;
pub const ENZF_LIMB_TAIL_BASE_ROOT: C2RustUnnamed_25 = 27;
pub const ENZF_LIMB_TAIL_ROOT: C2RustUnnamed_25 = 26;
pub const ENZF_LIMB_CHEST_ARMOR: C2RustUnnamed_25 = 25;
pub const ENZF_LIMB_LEFT_UPPER_ARM: C2RustUnnamed_25 = 24;
pub const ENZF_LIMB_LEFT_FOREARM: C2RustUnnamed_25 = 23;
pub const ENZF_LIMB_LEFT_HAND: C2RustUnnamed_25 = 22;
pub const ENZF_LIMB_LEFT_HAND_ROOT: C2RustUnnamed_25 = 21;
pub const ENZF_LIMB_LEFT_FOREARM_ROOT: C2RustUnnamed_25 = 20;
pub const ENZF_LIMB_LEFT_UPPER_ARM_ROOT: C2RustUnnamed_25 = 19;
pub const ENZF_LIMB_LEFT_ARM_ROOT: C2RustUnnamed_25 = 18;
pub const ENZF_LIMB_RIGHT_UPPER_ARM: C2RustUnnamed_25 = 17;
pub const ENZF_LIMB_RIGHT_FOREARM: C2RustUnnamed_25 = 16;
pub const ENZF_LIMB_SWORD: C2RustUnnamed_25 = 15;
pub const ENZF_LIMB_SWORD_ROOT: C2RustUnnamed_25 = 14;
pub const ENZF_LIMB_RIGHT_FOREARM_ROOT: C2RustUnnamed_25 = 13;
pub const ENZF_LIMB_RIGHT_UPPER_ARM_ROOT: C2RustUnnamed_25 = 12;
pub const ENZF_LIMB_RIGHT_ARM_ROOT: C2RustUnnamed_25 = 11;
pub const ENZF_LIMB_NECK: C2RustUnnamed_25 = 10;
pub const ENZF_LIMB_HEAD: C2RustUnnamed_25 = 9;
pub const ENZF_LIMB_JAW: C2RustUnnamed_25 = 8;
pub const ENZF_LIMB_JAW_ROOT: C2RustUnnamed_25 = 7;
pub const ENZF_LIMB_JAW_ROOT_ROOT: C2RustUnnamed_25 = 6;
pub const ENZF_LIMB_HEAD_ROOT: C2RustUnnamed_25 = 5;
pub const ENZF_LIMB_NECK_ROOT: C2RustUnnamed_25 = 4;
pub const ENZF_LIMB_UPPER_BODY_ROOT: C2RustUnnamed_25 = 3;
pub const ENZF_LIMB_BODY_ROOT: C2RustUnnamed_25 = 2;
pub const ENZF_LIMB_ROOT: C2RustUnnamed_25 = 1;
pub const ENZF_LIMB_NONE: C2RustUnnamed_25 = 0;
pub const ENZF_DMGEFF_NONE: C2RustUnnamed_26 = 0;
pub const ENZF_DMGEFF_IMMUNE: C2RustUnnamed_26 = 6;
pub const ENZF_DMGEFF_ICE: C2RustUnnamed_26 = 15;
pub const ENZF_DMGEFF_PROJECTILE: C2RustUnnamed_26 = 13;
pub const ENZF_DMGEFF_STUN: C2RustUnnamed_26 = 1;
pub type C2RustUnnamed_26 = libc::c_uint;
/* *
 * Array of platform positions in Dodongo's Cavern miniboss room.
 *  0 -  7 : Downstairs
 *  8 - 15 : Upstairs inner platforms
 * 16 - 23 : Upstairs outer platforms (including several points on the long thin one)
 */
static mut sPlatformPositions: [Vec3f; 24] =
    [{
         let mut init = Vec3f{x: 3560.0f32, y: 100.0f32, z: -1517.0f32,};
         init
     },
     {
         let mut init = Vec3f{x: 3170.0f32, y: 100.0f32, z: -1767.0f32,};
         init
     },
     {
         let mut init = Vec3f{x: 3165.0f32, y: 100.0f32, z: -2217.0f32,};
         init
     },
     {
         let mut init = Vec3f{x: 3563.0f32, y: 100.0f32, z: -2437.0f32,};
         init
     },
     {
         let mut init = Vec3f{x: 3946.0f32, y: 100.0f32, z: -2217.0f32,};
         init
     },
     {
         let mut init = Vec3f{x: 3942.0f32, y: 100.0f32, z: -1765.0f32,};
         init
     },
     {
         let mut init = Vec3f{x: 2861.0f32, y: 100.0f32, z: -2394.0f32,};
         init
     },
     {
         let mut init = Vec3f{x: 2776.0f32, y: 100.0f32, z: -1987.0f32,};
         init
     },
     {
         let mut init = Vec3f{x: 4527.0f32, y: 531.0f32, z: -1146.0f32,};
         init
     },
     {
         let mut init = Vec3f{x: 4442.0f32, y: 531.0f32, z: -1405.0f32,};
         init
     },
     {
         let mut init = Vec3f{x: 4170.0f32, y: 531.0f32, z: -1395.0f32,};
         init
     },
     {
         let mut init = Vec3f{x: 4030.0f32, y: 531.0f32, z: -1162.0f32,};
         init
     },
     { let mut init = Vec3f{x: 4010.0f32, y: 531.0f32, z: -883.0f32,}; init },
     { let mut init = Vec3f{x: 4270.0f32, y: 531.0f32, z: -810.0f32,}; init },
     { let mut init = Vec3f{x: 4520.0f32, y: 531.0f32, z: -880.0f32,}; init },
     {
         let mut init = Vec3f{x: 4260.0f32, y: 531.0f32, z: -1035.0f32,};
         init
     },
     {
         let mut init = Vec3f{x: 4757.0f32, y: 531.0f32, z: -1146.0f32,};
         init
     },
     { let mut init = Vec3f{x: 3850.0f32, y: 531.0f32, z: -883.0f32,}; init },
     { let mut init = Vec3f{x: 4380.0f32, y: 531.0f32, z: -690.0f32,}; init },
     { let mut init = Vec3f{x: 4197.0f32, y: 531.0f32, z: -646.0f32,}; init },
     {
         let mut init = Vec3f{x: 4070.0f32, y: 531.0f32, z: -1575.0f32,};
         init
     },
     {
         let mut init = Vec3f{x: 3930.0f32, y: 531.0f32, z: -1705.0f32,};
         init
     },
     {
         let mut init = Vec3f{x: 3780.0f32, y: 531.0f32, z: -1835.0f32,};
         init
     },
     {
         let mut init = Vec3f{x: 3560.0f32, y: 531.0f32, z: -1985.0f32,};
         init
     }];
// These seem to relate to the tagging in/out the minibosses do
static mut D_80B4A1B0: s16 = 0 as libc::c_int as s16;
static mut D_80B4A1B4: s16 = 1 as libc::c_int as s16;
#[no_mangle]
pub static mut En_Zf_InitVars: ActorInit =
    unsafe {
        {
            let mut init =
                ActorInit{id: ACTOR_EN_ZF as libc::c_int as s16,
                          category: ACTORCAT_ENEMY as libc::c_int as u8_0,
                          flags:
                              ((1 as libc::c_int) << 0 as libc::c_int |
                                   (1 as libc::c_int) << 2 as libc::c_int |
                                   (1 as libc::c_int) << 4 as libc::c_int) as
                                  u32_0,
                          objectId: OBJECT_ZF as libc::c_int as s16,
                          instanceSize:
                              ::std::mem::size_of::<EnZf>() as libc::c_ulong,
                          init:
                              ::std::mem::transmute::<Option<unsafe extern "C" fn(_:
                                                                                      *mut Actor,
                                                                                  _:
                                                                                      *mut GlobalContext)
                                                                 -> ()>,
                                                      ActorFunc>(Some(EnZf_Init
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
                                                      ActorFunc>(Some(EnZf_Destroy
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
                                                      ActorFunc>(Some(EnZf_Update
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
                                                      ActorFunc>(Some(EnZf_Draw
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
static mut sBodyCylinderInit: ColliderCylinderInit =
    {
        let mut init =
            ColliderCylinderInit{base:
                                     {
                                         let mut init =
                                             ColliderInit{colType:
                                                              COLTYPE_HIT0 as
                                                                  libc::c_int
                                                                  as u8_0,
                                                          atFlags:
                                                              0 as libc::c_int
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
                                                                   (1 as
                                                                        libc::c_int)
                                                                       <<
                                                                       3 as
                                                                           libc::c_int)
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
                                                                  ELEMTYPE_UNK1
                                                                      as
                                                                      libc::c_int
                                                                      as u8_0,
                                                              toucher:
                                                                  {
                                                                      let mut init =
                                                                          ColliderTouch{dmgFlags:
                                                                                            0
                                                                                                as
                                                                                                libc::c_int
                                                                                                as
                                                                                                u32_0,
                                                                                        effect:
                                                                                            0
                                                                                                as
                                                                                                libc::c_int
                                                                                                as
                                                                                                u8_0,
                                                                                        damage:
                                                                                            0
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
                                                                  0 as
                                                                      libc::c_int
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
                                                            70 as libc::c_int
                                                                as s16,
                                                        yShift:
                                                            0 as libc::c_int
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
static mut sSwordQuadInit: ColliderQuadInit =
    {
        let mut init =
            ColliderQuadInit{base:
                                 {
                                     let mut init =
                                         ColliderInit{colType:
                                                          COLTYPE_NONE as
                                                              libc::c_int as
                                                              u8_0,
                                                      atFlags:
                                                          ((1 as libc::c_int)
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
                                                          ((1 as libc::c_int)
                                                               <<
                                                               0 as
                                                                   libc::c_int
                                                               |
                                                               (1 as
                                                                    libc::c_int)
                                                                   <<
                                                                   2 as
                                                                       libc::c_int
                                                               |
                                                               (1 as
                                                                    libc::c_int)
                                                                   <<
                                                                   3 as
                                                                       libc::c_int)
                                                              as u8_0,
                                                      ocFlags1:
                                                          0 as libc::c_int as
                                                              u8_0,
                                                      ocFlags2:
                                                          0 as libc::c_int as
                                                              u8_0,
                                                      shape:
                                                          COLSHAPE_QUAD as
                                                              libc::c_int as
                                                              u8_0,};
                                     init
                                 },
                             info:
                                 {
                                     let mut init =
                                         ColliderInfoInit{elemType:
                                                              ELEMTYPE_UNK0 as
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
                                                                                        0x8
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
                                                                                           0
                                                                                               as
                                                                                               libc::c_int
                                                                                               as
                                                                                               u32_0,
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
                                                                       3 as
                                                                           libc::c_int
                                                                   |
                                                                   (1 as
                                                                        libc::c_int)
                                                                       <<
                                                                       7 as
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
                                                              0 as libc::c_int
                                                                  as u8_0,};
                                     init
                                 },
                             dim:
                                 {
                                     let mut init =
                                         ColliderQuadDimInit{quad:
                                                                 [{
                                                                      let mut init =
                                                                          Vec3f{x:
                                                                                    0.0f32,
                                                                                y:
                                                                                    0.0f32,
                                                                                z:
                                                                                    0.0f32,};
                                                                      init
                                                                  },
                                                                  {
                                                                      let mut init =
                                                                          Vec3f{x:
                                                                                    0.0f32,
                                                                                y:
                                                                                    0.0f32,
                                                                                z:
                                                                                    0.0f32,};
                                                                      init
                                                                  },
                                                                  {
                                                                      let mut init =
                                                                          Vec3f{x:
                                                                                    0.0f32,
                                                                                y:
                                                                                    0.0f32,
                                                                                z:
                                                                                    0.0f32,};
                                                                      init
                                                                  },
                                                                  {
                                                                      let mut init =
                                                                          Vec3f{x:
                                                                                    0.0f32,
                                                                                y:
                                                                                    0.0f32,
                                                                                z:
                                                                                    0.0f32,};
                                                                      init
                                                                  }],};
                                     init
                                 },};
        init
    };
static mut sDamageTable: DamageTable =
    {
        let mut init =
            DamageTable{table:
                            [(0 as libc::c_int |
                                  (ENZF_DMGEFF_STUN as libc::c_int) <<
                                      4 as libc::c_int) as u8_0,
                             (2 as libc::c_int |
                                  (ENZF_DMGEFF_NONE as libc::c_int) <<
                                      4 as libc::c_int) as u8_0,
                             (1 as libc::c_int |
                                  (ENZF_DMGEFF_PROJECTILE as libc::c_int) <<
                                      4 as libc::c_int) as u8_0,
                             (2 as libc::c_int |
                                  (ENZF_DMGEFF_NONE as libc::c_int) <<
                                      4 as libc::c_int) as u8_0,
                             (0 as libc::c_int |
                                  (ENZF_DMGEFF_STUN as libc::c_int) <<
                                      4 as libc::c_int) as u8_0,
                             (2 as libc::c_int |
                                  (ENZF_DMGEFF_NONE as libc::c_int) <<
                                      4 as libc::c_int) as u8_0,
                             (2 as libc::c_int |
                                  (ENZF_DMGEFF_NONE as libc::c_int) <<
                                      4 as libc::c_int) as u8_0,
                             (0 as libc::c_int |
                                  (ENZF_DMGEFF_STUN as libc::c_int) <<
                                      4 as libc::c_int) as u8_0,
                             (1 as libc::c_int |
                                  (ENZF_DMGEFF_NONE as libc::c_int) <<
                                      4 as libc::c_int) as u8_0,
                             (2 as libc::c_int |
                                  (ENZF_DMGEFF_NONE as libc::c_int) <<
                                      4 as libc::c_int) as u8_0,
                             (4 as libc::c_int |
                                  (ENZF_DMGEFF_NONE as libc::c_int) <<
                                      4 as libc::c_int) as u8_0,
                             (2 as libc::c_int |
                                  (ENZF_DMGEFF_PROJECTILE as libc::c_int) <<
                                      4 as libc::c_int) as u8_0,
                             (4 as libc::c_int |
                                  (ENZF_DMGEFF_ICE as libc::c_int) <<
                                      4 as libc::c_int) as u8_0,
                             (2 as libc::c_int |
                                  (ENZF_DMGEFF_PROJECTILE as libc::c_int) <<
                                      4 as libc::c_int) as u8_0,
                             (2 as libc::c_int |
                                  (ENZF_DMGEFF_PROJECTILE as libc::c_int) <<
                                      4 as libc::c_int) as u8_0,
                             (2 as libc::c_int |
                                  (ENZF_DMGEFF_PROJECTILE as libc::c_int) <<
                                      4 as libc::c_int) as u8_0,
                             (2 as libc::c_int |
                                  (ENZF_DMGEFF_PROJECTILE as libc::c_int) <<
                                      4 as libc::c_int) as u8_0,
                             (0 as libc::c_int |
                                  (ENZF_DMGEFF_IMMUNE as libc::c_int) <<
                                      4 as libc::c_int) as u8_0,
                             (3 as libc::c_int |
                                  (ENZF_DMGEFF_ICE as libc::c_int) <<
                                      4 as libc::c_int) as u8_0,
                             (0 as libc::c_int |
                                  (ENZF_DMGEFF_IMMUNE as libc::c_int) <<
                                      4 as libc::c_int) as u8_0,
                             (0 as libc::c_int |
                                  (ENZF_DMGEFF_NONE as libc::c_int) <<
                                      4 as libc::c_int) as u8_0,
                             (0 as libc::c_int |
                                  (ENZF_DMGEFF_NONE as libc::c_int) <<
                                      4 as libc::c_int) as u8_0,
                             (1 as libc::c_int |
                                  (ENZF_DMGEFF_NONE as libc::c_int) <<
                                      4 as libc::c_int) as u8_0,
                             (4 as libc::c_int |
                                  (ENZF_DMGEFF_NONE as libc::c_int) <<
                                      4 as libc::c_int) as u8_0,
                             (2 as libc::c_int |
                                  (ENZF_DMGEFF_NONE as libc::c_int) <<
                                      4 as libc::c_int) as u8_0,
                             (2 as libc::c_int |
                                  (ENZF_DMGEFF_NONE as libc::c_int) <<
                                      4 as libc::c_int) as u8_0,
                             (8 as libc::c_int |
                                  (ENZF_DMGEFF_NONE as libc::c_int) <<
                                      4 as libc::c_int) as u8_0,
                             (4 as libc::c_int |
                                  (ENZF_DMGEFF_NONE as libc::c_int) <<
                                      4 as libc::c_int) as u8_0,
                             (0 as libc::c_int |
                                  (ENZF_DMGEFF_NONE as libc::c_int) <<
                                      4 as libc::c_int) as u8_0,
                             (0 as libc::c_int |
                                  (ENZF_DMGEFF_NONE as libc::c_int) <<
                                      4 as libc::c_int) as u8_0,
                             (4 as libc::c_int |
                                  (ENZF_DMGEFF_NONE as libc::c_int) <<
                                      4 as libc::c_int) as u8_0,
                             (0 as libc::c_int |
                                  (ENZF_DMGEFF_NONE as libc::c_int) <<
                                      4 as libc::c_int) as u8_0],};
        init
    };
// Initialized in run_static_initializers
static mut sInitChain: [InitChainEntry; 3] =
    [InitChainEntry{cont_type_0_offset_value: [0; 4],}; 3];
static mut sHoppingAnims: [*mut AnimationHeader; 3] =
    unsafe {
        [&gZfHopCrouchingAnim as *const AnimationHeader as
             *mut AnimationHeader,
         &gZfHopLeapingAnim as *const AnimationHeader as *mut AnimationHeader,
         &gZfHopLandingAnim as *const AnimationHeader as *mut AnimationHeader]
    };
static mut D_80B4AB30: s32 = 0;
// Set to 0 and incremented in EnZf_HopAway, but not actually used
#[no_mangle]
pub unsafe extern "C" fn EnZf_SetupAction(mut this: *mut EnZf,
                                          mut actionFunc: EnZfActionFunc) {
    (*this).actionFunc = actionFunc;
}
/* *
 * Tests if it will still be on a floor after moving forwards a distance determined by dist, in the shape forward
 * direction. If `dist` is 0, it defaults to a dist depending on speed direction, and params.
 */
#[no_mangle]
pub unsafe extern "C" fn EnZf_PrimaryFloorCheck(mut this: *mut EnZf,
                                                mut globalCtx:
                                                    *mut GlobalContext,
                                                mut dist: f32_0) -> s32 {
    let mut ret: s16 = 0;
    let mut curBgCheckFlags: s16 = 0;
    let mut sin: f32_0 = 0.;
    let mut cos: f32_0 = 0.;
    let mut curPos: Vec3f = Vec3f{x: 0., y: 0., z: 0.,};
    if dist == 0.0f32 {
        dist = if (*this).actor.speedXZ >= 0.0f32 { 1.0f32 } else { -1.0f32 };
        dist =
            if (*this).actor.params as libc::c_int >=
                   ENZF_TYPE_LIZALFOS_MINIBOSS_A as libc::c_int {
                (dist) * 45.0f32
            } else { (dist) * 30.0f32 }
    }
    // Save currents to restore later
    curPos = (*this).actor.world.pos;
    curBgCheckFlags = (*this).actor.bgCheckFlags as s16;
    sin = Math_SinS((*this).actor.world.rot.y) * dist;
    cos = Math_CosS((*this).actor.world.rot.y) * dist;
    (*this).actor.world.pos.x += sin;
    (*this).actor.world.pos.z += cos;
    Actor_UpdateBgCheckInfo(globalCtx, &mut (*this).actor, 0.0f32, 0.0f32,
                            0.0f32, 0x1c as libc::c_int);
    (*this).actor.world.pos = curPos;
    ret =
        ((*this).actor.bgCheckFlags as libc::c_int & 1 as libc::c_int == 0) as
            libc::c_int as s16;
    (*this).actor.bgCheckFlags = curBgCheckFlags as u16_0;
    return ret as s32;
}
/* *
 * Supplementary floor test.
 */
#[no_mangle]
pub unsafe extern "C" fn EnZf_SecondaryFloorCheck(mut this: *mut EnZf,
                                                  mut globalCtx:
                                                      *mut GlobalContext,
                                                  mut dist: f32_0) -> s16 {
    let mut ret: s16 = 0;
    let mut curBgCheckFlags: s16 = 0;
    let mut sin: f32_0 = 0.;
    let mut cos: f32_0 = 0.;
    let mut curPos: Vec3f = Vec3f{x: 0., y: 0., z: 0.,};
    if (*this).actor.speedXZ != 0.0f32 &&
           EnZf_PrimaryFloorCheck(this, globalCtx, (*this).actor.speedXZ) != 0
       {
        return 1 as libc::c_int as s16
    }
    // Save currents to restore later
    curPos = (*this).actor.world.pos;
    curBgCheckFlags = (*this).actor.bgCheckFlags as s16;
    sin = Math_SinS((*this).actor.shape.rot.y) * dist;
    cos = Math_CosS((*this).actor.shape.rot.y) * dist;
    (*this).actor.world.pos.x += sin;
    (*this).actor.world.pos.z += cos;
    Actor_UpdateBgCheckInfo(globalCtx, &mut (*this).actor, 0.0f32, 0.0f32,
                            0.0f32, 0x1c as libc::c_int);
    (*this).actor.world.pos = curPos;
    ret =
        ((*this).actor.bgCheckFlags as libc::c_int & 1 as libc::c_int == 0) as
            libc::c_int as s16;
    (*this).actor.bgCheckFlags = curBgCheckFlags as u16_0;
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn EnZf_Init(mut thisx: *mut Actor,
                                   mut globalCtx: *mut GlobalContext) {
    let mut pad: s32 = 0;
    let mut this: *mut EnZf = thisx as *mut EnZf;
    let mut player: *mut Player =
        (*globalCtx).actorCtx.actorLists[ACTORCAT_PLAYER as libc::c_int as
                                             usize].head as *mut Player;
    let mut blureInit: EffectBlureInit1 =
        EffectBlureInit1{unk_00: [0; 388],
                         p1StartColor: [0; 4],
                         p2StartColor: [0; 4],
                         p1EndColor: [0; 4],
                         p2EndColor: [0; 4],
                         elemDuration: 0,
                         unkFlag: 0,
                         calcMode: 0,};
    let mut posDiff: f32_0 = 0.;
    Actor_ProcessInitChain(thisx, sInitChain.as_mut_ptr());
    (*thisx).targetMode = 3 as libc::c_int as s8;
    (*this).clearFlag =
        (((*thisx).params as libc::c_int & 0xff00 as libc::c_int) >>
             8 as libc::c_int) as s16;
    /* Strip the top byte of params */
    (*thisx).params =
        ((*thisx).params as libc::c_int & 0xff as libc::c_int) as s16;
    /* Return the params to their original value if they were originally negative, i.e. 0xFFFF or 0xFFFE */
    if (*thisx).params as libc::c_int & 0x80 as libc::c_int != 0 {
        (*thisx).params =
            ((*thisx).params as libc::c_int | 0xff00 as libc::c_int) as s16
    } // Lizalfos
    ActorShape_Init(&mut (*thisx).shape, 0.0f32,
                    Some(ActorShadow_DrawFeet as
                             unsafe extern "C" fn(_: *mut Actor,
                                                  _: *mut Lights,
                                                  _: *mut GlobalContext)
                                 -> ()), 90.0f32); // Minibosses
    (*this).unk_3E0 = 0 as libc::c_int;
    (*thisx).colChkInfo.mass = 0xfe as libc::c_int as u8_0;
    (*thisx).colChkInfo.damageTable = &mut sDamageTable;
    blureInit.p2EndColor[2 as libc::c_int as usize] =
        255 as libc::c_int as u8_0;
    blureInit.p2EndColor[1 as libc::c_int as usize] =
        blureInit.p2EndColor[2 as libc::c_int as usize];
    blureInit.p2EndColor[0 as libc::c_int as usize] =
        blureInit.p2EndColor[1 as libc::c_int as usize];
    blureInit.p1EndColor[2 as libc::c_int as usize] =
        blureInit.p2EndColor[0 as libc::c_int as usize];
    blureInit.p1EndColor[1 as libc::c_int as usize] =
        blureInit.p1EndColor[2 as libc::c_int as usize];
    blureInit.p1EndColor[0 as libc::c_int as usize] =
        blureInit.p1EndColor[1 as libc::c_int as usize];
    blureInit.p2StartColor[2 as libc::c_int as usize] =
        blureInit.p1EndColor[0 as libc::c_int as usize];
    blureInit.p2StartColor[1 as libc::c_int as usize] =
        blureInit.p2StartColor[2 as libc::c_int as usize];
    blureInit.p2StartColor[0 as libc::c_int as usize] =
        blureInit.p2StartColor[1 as libc::c_int as usize];
    blureInit.p1StartColor[3 as libc::c_int as usize] =
        blureInit.p2StartColor[0 as libc::c_int as usize];
    blureInit.p1StartColor[2 as libc::c_int as usize] =
        blureInit.p1StartColor[3 as libc::c_int as usize];
    blureInit.p1StartColor[1 as libc::c_int as usize] =
        blureInit.p1StartColor[2 as libc::c_int as usize];
    blureInit.p1StartColor[0 as libc::c_int as usize] =
        blureInit.p1StartColor[1 as libc::c_int as usize];
    blureInit.p2StartColor[3 as libc::c_int as usize] =
        64 as libc::c_int as u8_0;
    blureInit.p2EndColor[3 as libc::c_int as usize] =
        0 as libc::c_int as u8_0;
    blureInit.p1EndColor[3 as libc::c_int as usize] =
        blureInit.p2EndColor[3 as libc::c_int as usize];
    blureInit.elemDuration = 8 as libc::c_int;
    blureInit.unkFlag = 0 as libc::c_int;
    blureInit.calcMode = 2 as libc::c_int;
    Effect_Add(globalCtx, &mut (*this).blureIndex,
               EFFECT_BLURE1 as libc::c_int, 0 as libc::c_int as u8_0,
               0 as libc::c_int as u8_0,
               &mut blureInit as *mut EffectBlureInit1 as *mut libc::c_void);
    Actor_UpdateBgCheckInfo(globalCtx, thisx, 75.0f32, 45.0f32, 45.0f32,
                            0x1d as libc::c_int);
    (*this).alpha = 255 as libc::c_int as u8_0;
    (*thisx).colChkInfo.cylRadius = 40 as libc::c_int as s16;
    (*thisx).colChkInfo.cylHeight = 100 as libc::c_int as s16;
    Collider_InitCylinder(globalCtx, &mut (*this).bodyCollider);
    Collider_SetCylinder(globalCtx, &mut (*this).bodyCollider, thisx,
                         &mut sBodyCylinderInit);
    Collider_InitQuad(globalCtx, &mut (*this).swordCollider);
    Collider_SetQuad(globalCtx, &mut (*this).swordCollider, thisx,
                     &mut sSwordQuadInit);
    if (*thisx).params as libc::c_int == ENZF_TYPE_DINOLFOS as libc::c_int {
        (*thisx).colChkInfo.health = 12 as libc::c_int as u8_0;
        (*thisx).naviEnemyId = 0x10 as libc::c_int as u8_0;
        SkelAnime_Init(globalCtx, &mut (*this).skelAnime,
                       &mut gZfDinolfosSkel, &mut gZfCryingAnim,
                       (*this).jointTable.as_mut_ptr(),
                       (*this).morphTable.as_mut_ptr(),
                       ENZF_LIMB_MAX as libc::c_int);
    } else {
        (*thisx).colChkInfo.health = 6 as libc::c_int as u8_0;
        (*thisx).naviEnemyId = 0xf as libc::c_int as u8_0;
        SkelAnime_Init(globalCtx, &mut (*this).skelAnime,
                       &mut gZfLizalfosSkel, &mut gZfCryingAnim,
                       (*this).jointTable.as_mut_ptr(),
                       (*this).morphTable.as_mut_ptr(),
                       ENZF_LIMB_MAX as libc::c_int);
    }
    if ((*thisx).params as libc::c_int) <
           ENZF_TYPE_LIZALFOS_MINIBOSS_A as libc::c_int {
        // Not minibosses
        (*this).curPlatform = -(1 as libc::c_int) as s16; // Dinolfos
        (*this).homePlatform = (*this).curPlatform;
        D_80B4A1B4 = -(1 as libc::c_int) as s16;
        (*this).hopAnimIndex = 1 as libc::c_int;
        if (*thisx).params as libc::c_int ==
               ENZF_TYPE_LIZALFOS_LONE as libc::c_int {
            EnZf_SetupDropIn(this);
        } else { func_80B45384(this); }
    } else {
        posDiff = (*player).actor.world.pos.y - (*thisx).world.pos.y;
        if (if posDiff >= 0 as libc::c_int as libc::c_float {
                posDiff
            } else { -posDiff }) <= 100.0f32 &&
               Flags_GetSwitch(globalCtx, (*this).clearFlag as s32) == 0 {
            (*this).curPlatform =
                EnZf_FindPlatform(&mut (*thisx).world.pos,
                                  0 as libc::c_int as s16);
            (*this).homePlatform = (*this).curPlatform;
            EnZf_SetupDropIn(this);
            D_80B4A1B4 = 1 as libc::c_int as s16
        } else { Actor_Kill(thisx); }
    };
}
#[no_mangle]
pub unsafe extern "C" fn EnZf_Destroy(mut thisx: *mut Actor,
                                      mut globalCtx: *mut GlobalContext) {
    let mut this: *mut EnZf = thisx as *mut EnZf;
    if (*this).actor.params as libc::c_int >=
           ENZF_TYPE_LIZALFOS_MINIBOSS_A as libc::c_int &&
           Actor_FindNearby(globalCtx, &mut (*this).actor,
                            ACTOR_EN_ZF as libc::c_int as s16,
                            ACTORCAT_ENEMY as libc::c_int as u8_0,
                            10000.0f32).is_null() {
        func_800F5B58();
    }
    Effect_Delete(globalCtx, (*this).blureIndex);
    Collider_DestroyCylinder(globalCtx, &mut (*this).bodyCollider);
    Collider_DestroyQuad(globalCtx, &mut (*this).swordCollider);
}
/* *
 * Finds the index of the platform position in `sPlatformPositions` that is sufficiently close to `pos`.
 * Returns `preferedIndex` if the associated position is close enough.
 */
#[no_mangle]
pub unsafe extern "C" fn EnZf_FindPlatform(mut pos: *mut Vec3f,
                                           mut preferredIndex: s16) -> s16 {
    let mut rangeXZ: f32_0 = 0.;
    let mut i: s16 = 0;
    rangeXZ = 210.0f32;
    // Upstairs has a smaller range
    if (*pos).y >= 420.0f32 {
        rangeXZ = 110.0f32
    } // Will never retreat to the last two
    if preferredIndex as libc::c_int != -(1 as libc::c_int) {
        i = preferredIndex;
        if sPlatformPositions[i as usize].y - 150.0f32 <= (*pos).y &&
               (*pos).y <= sPlatformPositions[i as usize].y + 150.0f32 &&
               sPlatformPositions[i as usize].x - rangeXZ <= (*pos).x &&
               (*pos).x <= sPlatformPositions[i as usize].x + rangeXZ &&
               sPlatformPositions[i as usize].z - rangeXZ <= (*pos).z &&
               (*pos).z <= sPlatformPositions[i as usize].z + rangeXZ {
            return preferredIndex
        }
    }
    i =
        ((::std::mem::size_of::<[Vec3f; 24]>() as
              libc::c_ulong).wrapping_div(::std::mem::size_of::<Vec3f>() as
                                              libc::c_ulong) as s32 -
             1 as libc::c_int) as s16;
    while i as libc::c_int > -(1 as libc::c_int) {
        if sPlatformPositions[i as usize].y - 150.0f32 <= (*pos).y &&
               (*pos).y <= sPlatformPositions[i as usize].y + 150.0f32 &&
               sPlatformPositions[i as usize].x - rangeXZ <= (*pos).x &&
               (*pos).x <= sPlatformPositions[i as usize].x + rangeXZ &&
               sPlatformPositions[i as usize].z - rangeXZ <= (*pos).z &&
               (*pos).z <= sPlatformPositions[i as usize].z + rangeXZ {
            break ;
        }
        i -= 1
    }
    return i;
}
#[no_mangle]
pub unsafe extern "C" fn EnZf_FindNextPlatformAwayFromPlayer(mut pos:
                                                                 *mut Vec3f,
                                                             mut curPlatform:
                                                                 s16,
                                                             mut arg2: s16,
                                                             mut globalCtx:
                                                                 *mut GlobalContext)
 -> s16 {
    let mut distToCurLoopPlatform: f32_0 = 0.;
    let mut platformMinDist: f32_0 = 585.0f32;
    let mut player: *mut Player =
        (*globalCtx).actorCtx.actorLists[ACTORCAT_PLAYER as libc::c_int as
                                             usize].head as *mut Player;
    let mut initialPlatform: s16 = curPlatform;
    let mut playerMaxDist: f32_0 = 400.0f32;
    let mut smallMaxRange: f32_0 = 99998.0f32;
    let mut curLoopPlatform: s16 = 5 as libc::c_int as s16;
    let mut minIndex: s16 = 0 as libc::c_int as s16;
    let mut largeMaxRange: f32_0 = 99999.0f32;
    let mut altNextPlatform: s16 = -(1 as libc::c_int) as s16;
    let mut nextPlatform: s16 = -(1 as libc::c_int) as s16;
    let mut playerPlatform: s16 =
        EnZf_FindPlatform(&mut (*player).actor.world.pos, initialPlatform);
    // Set up search constraints
    // Upstairs
    if (*pos).y > 420.0f32 {
        minIndex = (7 as libc::c_int + 1 as libc::c_int) as s16;
        playerMaxDist = 50.0f32;
        // Upstairs outer
        if initialPlatform as libc::c_int >= 15 as libc::c_int {
            curLoopPlatform =
                ((::std::mem::size_of::<[Vec3f; 24]>() as
                      libc::c_ulong).wrapping_div(::std::mem::size_of::<Vec3f>()
                                                      as libc::c_ulong) as s32
                     - 1 as libc::c_int) as s16; // upstairs inner
            platformMinDist = 400.0f32
        } else {
            curLoopPlatform = (15 as libc::c_int - 1 as libc::c_int) as s16;
            platformMinDist = 380.0f32
        }
    }
    while curLoopPlatform as libc::c_int >= minIndex as libc::c_int {
        if !(curLoopPlatform as libc::c_int == initialPlatform as libc::c_int
                 ||
                 curLoopPlatform as libc::c_int ==
                     playerPlatform as libc::c_int) {
            if !(playerPlatform as libc::c_int == -(1 as libc::c_int) &&
                     Math_Vec3f_DistXYZ(&mut (*player).actor.world.pos,
                                        &mut *sPlatformPositions.as_mut_ptr().offset(curLoopPlatform
                                                                                         as
                                                                                         isize))
                         < playerMaxDist) {
                distToCurLoopPlatform =
                    Math_Vec3f_DistXYZ(pos,
                                       &mut *sPlatformPositions.as_mut_ptr().offset(curLoopPlatform
                                                                                        as
                                                                                        isize));
                if !(platformMinDist < distToCurLoopPlatform) {
                    if distToCurLoopPlatform < smallMaxRange {
                        largeMaxRange = smallMaxRange;
                        altNextPlatform = nextPlatform;
                        smallMaxRange = distToCurLoopPlatform;
                        nextPlatform = curLoopPlatform
                    } else if distToCurLoopPlatform < largeMaxRange {
                        largeMaxRange = distToCurLoopPlatform;
                        altNextPlatform = curLoopPlatform
                    }
                }
            }
        }
        curLoopPlatform -= 1
    }
    // These functions have no side effects, so these two calls do nothing
    Math_Vec3f_DistXYZ(&mut (*player).actor.world.pos,
                       &mut *sPlatformPositions.as_mut_ptr().offset(nextPlatform
                                                                        as
                                                                        isize));
    Math_Vec3f_DistXYZ(&mut (*player).actor.world.pos,
                       &mut *sPlatformPositions.as_mut_ptr().offset(altNextPlatform
                                                                        as
                                                                        isize));
    if altNextPlatform as libc::c_int > 0 as libc::c_int {
        let mut nextPlatformToPlayerYaw: s16 =
            (Math_Vec3f_Yaw(pos,
                            &mut *sPlatformPositions.as_mut_ptr().offset(nextPlatform
                                                                             as
                                                                             isize))
                 as libc::c_int -
                 Math_Vec3f_Yaw(pos, &mut (*player).actor.world.pos) as
                     libc::c_int) as s16;
        if (if nextPlatformToPlayerYaw as libc::c_int >= 0 as libc::c_int {
                nextPlatformToPlayerYaw as libc::c_int
            } else { -(nextPlatformToPlayerYaw as libc::c_int) }) <
               0x36b0 as libc::c_int {
            nextPlatform = altNextPlatform
        }
    }
    if (nextPlatform as libc::c_int) < 0 as libc::c_int {
        nextPlatform = arg2
    }
    return nextPlatform;
}
#[no_mangle]
pub unsafe extern "C" fn EnZf_FindNextPlatformTowardsPlayer(mut pos:
                                                                *mut Vec3f,
                                                            mut curPlatform:
                                                                s16,
                                                            mut arg2: s16,
                                                            mut globalCtx:
                                                                *mut GlobalContext)
 -> s16 {
    let mut curLoopPlatform: s16 = 7 as libc::c_int as s16;
    let mut minIndex: s16 = 0 as libc::c_int as s16;
    let mut player: *mut Player =
        (*globalCtx).actorCtx.actorLists[ACTORCAT_PLAYER as libc::c_int as
                                             usize].head as *mut Player;
    let mut nextPlatform: s16 =
        EnZf_FindPlatform(&mut (*player).actor.world.pos,
                          -(1 as libc::c_int) as s16);
    let mut minRange: f32_0 = 500.0f32;
    let mut smallMaxRange: f32_0 = 99998.0f32;
    let mut largeMaxRange: f32_0 = 99999.0f32;
    let mut phi_s2: s16 = curPlatform;
    let mut phi_s3: s16 = arg2;
    // Upstairs
    if (*pos).y > 200.0f32 {
        curLoopPlatform =
            ((::std::mem::size_of::<[Vec3f; 24]>() as
                  libc::c_ulong).wrapping_div(::std::mem::size_of::<Vec3f>()
                                                  as libc::c_ulong) as s32 -
                 1 as libc::c_int) as s16;
        minIndex = (7 as libc::c_int + 1 as libc::c_int) as s16;
        minRange = 290.0f32
    }
    while curLoopPlatform as libc::c_int >= minIndex as libc::c_int {
        if !(minRange <
                 Math_Vec3f_DistXYZ(pos,
                                    &mut *sPlatformPositions.as_mut_ptr().offset(curLoopPlatform
                                                                                     as
                                                                                     isize)))
           {
            if curLoopPlatform as libc::c_int != nextPlatform as libc::c_int {
                let mut curPlatformDistToPlayer: f32_0 =
                    Math_Vec3f_DistXYZ(&mut (*player).actor.world.pos,
                                       &mut *sPlatformPositions.as_mut_ptr().offset(curLoopPlatform
                                                                                        as
                                                                                        isize));
                if curPlatformDistToPlayer < smallMaxRange {
                    largeMaxRange = smallMaxRange;
                    phi_s3 = phi_s2;
                    smallMaxRange = curPlatformDistToPlayer;
                    phi_s2 = curLoopPlatform
                } else if curPlatformDistToPlayer < largeMaxRange {
                    largeMaxRange = curPlatformDistToPlayer;
                    phi_s3 = curLoopPlatform
                }
            } else { phi_s2 = nextPlatform; break ; }
        }
        curLoopPlatform -= 1
    }
    if phi_s3 as libc::c_int != nextPlatform as libc::c_int {
        nextPlatform = phi_s2
    } else { nextPlatform = phi_s3 }
    return nextPlatform;
}
// Player not targeting this or another EnZf?
#[no_mangle]
pub unsafe extern "C" fn EnZf_CanAttack(mut globalCtx: *mut GlobalContext,
                                        mut this: *mut EnZf) -> s32 {
    let mut targetedActor: *mut Actor = 0 as *mut Actor;
    let mut player: *mut Player =
        (*globalCtx).actorCtx.actorLists[ACTORCAT_PLAYER as libc::c_int as
                                             usize].head as *mut Player;
    if (*this).actor.params as libc::c_int >=
           ENZF_TYPE_LIZALFOS_MINIBOSS_A as libc::c_int {
        // miniboss
        if (*player).stateFlags1 & 0x6000 as libc::c_int as libc::c_uint != 0
           {
            // Hanging or climbing
            return 0 as libc::c_int
        } else { return 1 as libc::c_int }
    } else {
        if Actor_OtherIsTargeted(globalCtx, &mut (*this).actor) == 0 {
            return 1 as libc::c_int
        }
        if (*this).actor.params as libc::c_int ==
               ENZF_TYPE_DINOLFOS as libc::c_int {
            targetedActor = (*player).unk_664;
            if targetedActor.is_null() {
                return 0 as libc::c_int
            } else {
                if (*targetedActor).category as libc::c_int !=
                       ACTORCAT_ENEMY as libc::c_int {
                    return 1 as libc::c_int
                }
                if (*targetedActor).id as libc::c_int !=
                       ACTOR_EN_ZF as libc::c_int {
                    return 0 as libc::c_int
                } else {
                    if (*targetedActor).colorFilterTimer as libc::c_int !=
                           0 as libc::c_int {
                        return 1 as libc::c_int
                    }
                }
            }
        }
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn func_80B44DC4(mut this: *mut EnZf,
                                       mut globalCtx: *mut GlobalContext) {
    let mut angleDiff: s16 =
        ((*this).actor.yawTowardsPlayer as libc::c_int -
             (*this).actor.shape.rot.y as libc::c_int) as s16;
    if (angleDiff as libc::c_int) < 0 as libc::c_int {
        angleDiff = -(angleDiff as libc::c_int) as s16
    }
    if angleDiff as libc::c_int >= 0x1b58 as libc::c_int {
        func_80B483E4(this, globalCtx);
    } else if (*this).actor.xzDistToPlayer <= 100.0f32 &&
                  (*globalCtx).gameplayFrames.wrapping_rem(8 as libc::c_int as
                                                               libc::c_uint)
                      != 0 as libc::c_int as libc::c_uint &&
                  EnZf_CanAttack(globalCtx, this) != 0 {
        EnZf_SetupSlash(this);
    } else { func_80B45384(this); };
}
#[no_mangle]
pub unsafe extern "C" fn EnZf_ChooseAction(mut globalCtx: *mut GlobalContext,
                                           mut this: *mut EnZf) -> s32 {
    let mut angleToWall: s16 = 0;
    let mut explosive: *mut Actor = 0 as *mut Actor;
    angleToWall =
        ((*this).actor.wallYaw as libc::c_int -
             (*this).actor.shape.rot.y as libc::c_int) as s16;
    angleToWall =
        if angleToWall as libc::c_int >= 0 as libc::c_int {
            angleToWall as libc::c_int
        } else { -(angleToWall as libc::c_int) } as s16;
    if func_800354B4(globalCtx, &mut (*this).actor, 100.0f32,
                     0x5dc0 as libc::c_int as s16,
                     0x2aa8 as libc::c_int as s16, (*this).actor.shape.rot.y)
           != 0 {
        (*this).actor.world.rot.y = (*this).actor.yawTowardsPlayer;
        (*this).actor.shape.rot.y = (*this).actor.world.rot.y;
        if (*this).actor.bgCheckFlags as libc::c_int & 8 as libc::c_int != 0
               &&
               (if angleToWall as libc::c_int >= 0 as libc::c_int {
                    angleToWall as libc::c_int
                } else { -(angleToWall as libc::c_int) }) <
                   0x2ee0 as libc::c_int &&
               (*this).actor.xzDistToPlayer < 80.0f32 {
            EnZf_SetupJumpUp(this);
            return 1 as libc::c_int
        } else if (*this).actor.xzDistToPlayer < 90.0f32 &&
                      (*globalCtx).gameplayFrames.wrapping_rem(2 as
                                                                   libc::c_int
                                                                   as
                                                                   libc::c_uint)
                          != 0 as libc::c_int as libc::c_uint {
            EnZf_SetupJumpUp(this);
            return 1 as libc::c_int
        } else { EnZf_SetupJumpBack(this); return 1 as libc::c_int }
    }
    explosive =
        Actor_FindNearby(globalCtx, &mut (*this).actor,
                         -(1 as libc::c_int) as s16,
                         ACTORCAT_EXPLOSIVE as libc::c_int as u8_0, 80.0f32);
    if !explosive.is_null() {
        (*this).actor.world.rot.y = (*this).actor.yawTowardsPlayer;
        (*this).actor.shape.rot.y = (*this).actor.world.rot.y;
        if (*this).actor.bgCheckFlags as libc::c_int & 8 as libc::c_int != 0
               && (angleToWall as libc::c_int) < 0x2ee0 as libc::c_int ||
               (*explosive).id as libc::c_int ==
                   ACTOR_EN_BOM_CHU as libc::c_int {
            if (*explosive).id as libc::c_int ==
                   ACTOR_EN_BOM_CHU as libc::c_int &&
                   Actor_WorldDistXYZToActor(&mut (*this).actor, explosive) <
                       80.0f32 &&
                   (((*this).actor.shape.rot.y as libc::c_int -
                         (*explosive).world.rot.y as libc::c_int +
                         0x8000 as libc::c_int) as s16 as libc::c_int) <
                       0x3e80 as libc::c_int {
                EnZf_SetupJumpUp(this);
                return 1 as libc::c_int
            } else {
                EnZf_SetupCircleAroundPlayer(this, 4.0f32);
                return 1 as libc::c_int
            }
        } else { EnZf_SetupJumpBack(this); return 1 as libc::c_int }
    }
    return 0 as libc::c_int;
}
// Setup functions and action functions
/* *
 * Set position 300 units above ground and invisible, fade in and drop to ground, fully solid when on ground
 */
#[no_mangle]
pub unsafe extern "C" fn EnZf_SetupDropIn(mut this: *mut EnZf) {
    Animation_Change(&mut (*this).skelAnime, &mut gZfJumpingAnim, 0.0f32,
                     9.0f32,
                     Animation_GetLastFrame(&mut gZfJumpingAnim as
                                                *mut AnimationHeader as
                                                *mut libc::c_void) as f32_0,
                     ANIMMODE_LOOP as libc::c_int as u8_0, 0.0f32);
    (*this).actor.world.pos.y = (*this).actor.floorHeight + 300.0f32;
    (*this).actor.shape.shadowAlpha = 0 as libc::c_int as u8_0;
    (*this).alpha = (*this).actor.shape.shadowAlpha;
    (*this).unk_3F0 = 10 as libc::c_int;
    (*this).hopAnimIndex = 1 as libc::c_int;
    (*this).action = ENZF_ACTION_DROP_IN as libc::c_int;
    (*this).actor.bgCheckFlags =
        ((*this).actor.bgCheckFlags as libc::c_int & !(2 as libc::c_int)) as
            u16_0;
    (*this).actor.flags &=
        !((1 as libc::c_int) << 0 as libc::c_int) as libc::c_uint;
    (*this).actor.world.rot.y = (*this).actor.yawTowardsPlayer;
    (*this).actor.shape.rot.y = (*this).actor.world.rot.y;
    EnZf_SetupAction(this,
                     Some(EnZf_DropIn as
                              unsafe extern "C" fn(_: *mut EnZf,
                                                   _: *mut GlobalContext)
                                  -> ()));
}
#[no_mangle]
pub unsafe extern "C" fn EnZf_DropIn(mut this: *mut EnZf,
                                     mut globalCtx: *mut GlobalContext) {
    if (*this).unk_3F0 == 1 as libc::c_int {
        Audio_PlayActorSound2(&mut (*this).actor,
                              0x3829 as libc::c_int as u16_0);
        (*this).actor.flags |=
            ((1 as libc::c_int) << 0 as libc::c_int) as libc::c_uint;
        if (*this).actor.params as libc::c_int ==
               ENZF_TYPE_LIZALFOS_MINIBOSS_A as libc::c_int {
            func_800F5ACC(0x38 as libc::c_int as u16_0);
            // Miniboss theme
        }
    }
    if (*this).unk_3F0 != 0 as libc::c_int {
        if (*this).actor.params as libc::c_int !=
               ENZF_TYPE_LIZALFOS_LONE as libc::c_int {
            (*this).unk_3F0 -= 1
        } else if (*this).actor.xzDistToPlayer <= 160.0f32 {
            (*this).unk_3F0 = 0 as libc::c_int;
            (*this).actor.flags |=
                ((1 as libc::c_int) << 0 as libc::c_int) as libc::c_uint;
            Audio_PlayActorSound2(&mut (*this).actor,
                                  0x3829 as libc::c_int as u16_0);
        }
        (*this).actor.world.pos.y = (*this).actor.floorHeight + 300.0f32
    } else if ((*this).alpha as libc::c_int) < 255 as libc::c_int {
        (*this).alpha =
            ((*this).alpha as libc::c_int +
                 255 as libc::c_int / 5 as libc::c_int) as u8_0
    }
    if (*this).actor.bgCheckFlags as libc::c_int & 3 as libc::c_int != 0 &&
           (*this).hopAnimIndex != 0 as libc::c_int {
        Audio_PlayActorSound2(&mut (*this).actor,
                              0x3828 as libc::c_int as u16_0);
        Animation_Change(&mut (*this).skelAnime, &mut gZfLandingAnim, 1.0f32,
                         0.0f32, 17.0f32,
                         ANIMMODE_ONCE as libc::c_int as u8_0, 0.0f32);
        (*this).hopAnimIndex = 0 as libc::c_int;
        (*this).actor.bgCheckFlags =
            ((*this).actor.bgCheckFlags as libc::c_int & !(2 as libc::c_int))
                as u16_0;
        (*this).actor.world.pos.y = (*this).actor.floorHeight;
        (*this).actor.velocity.y = 0.0f32;
        Actor_SpawnFloorDustRing(globalCtx, &mut (*this).actor,
                                 &mut (*this).leftFootPos, 3.0f32,
                                 2 as libc::c_int, 2.0f32,
                                 0 as libc::c_int as s16,
                                 0 as libc::c_int as s16,
                                 0 as libc::c_int as u8_0);
        Actor_SpawnFloorDustRing(globalCtx, &mut (*this).actor,
                                 &mut (*this).rightFootPos, 3.0f32,
                                 2 as libc::c_int, 2.0f32,
                                 0 as libc::c_int as s16,
                                 0 as libc::c_int as s16,
                                 0 as libc::c_int as u8_0);
    }
    if SkelAnime_Update(&mut (*this).skelAnime) != 0 {
        (*this).alpha = 255 as libc::c_int as u8_0;
        if (*this).actor.params as libc::c_int >
               ENZF_TYPE_LIZALFOS_MINIBOSS_A as libc::c_int {
            // Only miniboss B
            EnZf_SetupSheatheSword(this, globalCtx);
        } else { func_80B45384(this); }
    }
    (*this).actor.shape.shadowAlpha = (*this).alpha;
}
// stop? and choose an action
#[no_mangle]
pub unsafe extern "C" fn func_80B45384(mut this: *mut EnZf) {
    Animation_Change(&mut (*this).skelAnime, &mut gZfCryingAnim, 1.0f32,
                     0.0f32,
                     Animation_GetLastFrame(&mut gZfCryingAnim as
                                                *mut AnimationHeader as
                                                *mut libc::c_void) as f32_0,
                     ANIMMODE_LOOP_INTERP as libc::c_int as u8_0, -4.0f32);
    (*this).action = ENZF_ACTION_3 as libc::c_int;
    (*this).unk_3F0 = (Rand_ZeroOne() * 10.0f32 + 5.0f32) as s32;
    (*this).actor.speedXZ = 0.0f32;
    (*this).actor.world.rot.y = (*this).actor.shape.rot.y;
    EnZf_SetupAction(this,
                     Some(func_80B4543C as
                              unsafe extern "C" fn(_: *mut EnZf,
                                                   _: *mut GlobalContext)
                                  -> ()));
}
#[no_mangle]
pub unsafe extern "C" fn func_80B4543C(mut this: *mut EnZf,
                                       mut globalCtx: *mut GlobalContext) {
    let mut player: *mut Player =
        (*globalCtx).actorCtx.actorLists[ACTORCAT_PLAYER as libc::c_int as
                                             usize].head as *mut Player;
    let mut pad: s32 = 0;
    let mut angleToPlayer: s16 =
        ((*this).actor.yawTowardsPlayer as libc::c_int -
             (*this).headRot as libc::c_int -
             (*this).actor.shape.rot.y as libc::c_int) as s16;
    angleToPlayer =
        if angleToPlayer as libc::c_int >= 0 as libc::c_int {
            angleToPlayer as libc::c_int
        } else { -(angleToPlayer as libc::c_int) } as s16;
    SkelAnime_Update(&mut (*this).skelAnime);
    if EnZf_DodgeRangedEngaging(globalCtx, this) == 0 {
        if (*this).actor.params as libc::c_int ==
               ENZF_TYPE_DINOLFOS as libc::c_int {
            if (*this).unk_3F4 as libc::c_int != 0 as libc::c_int {
                (*this).unk_3F4 -= 1;
                if angleToPlayer as libc::c_int >= 0x1ffe as libc::c_int {
                    return
                }
                (*this).unk_3F4 = 0 as libc::c_int as s16
            } else if EnZf_ChooseAction(globalCtx, this) != 0 { return }
        }
        angleToPlayer =
            ((*player).actor.shape.rot.y as libc::c_int -
                 (*this).actor.shape.rot.y as libc::c_int) as s16;
        angleToPlayer =
            if angleToPlayer as libc::c_int >= 0 as libc::c_int {
                angleToPlayer as libc::c_int
            } else { -(angleToPlayer as libc::c_int) } as s16;
        if (*this).actor.xzDistToPlayer < 100.0f32 &&
               (*player).swordState as libc::c_int != 0 as libc::c_int &&
               angleToPlayer as libc::c_int >= 0x1f40 as libc::c_int {
            (*this).actor.world.rot.y = (*this).actor.yawTowardsPlayer;
            (*this).actor.shape.rot.y = (*this).actor.world.rot.y;
            func_80B483E4(this, globalCtx);
        } else if (*this).unk_3F0 != 0 as libc::c_int {
            (*this).unk_3F0 -= 1
        } else {
            if Actor_IsFacingPlayer(&mut (*this).actor,
                                    (30 as libc::c_int *
                                         0x10000 as libc::c_int /
                                         360 as libc::c_int) as s16) != 0 {
                if (*this).actor.xzDistToPlayer < 200.0f32 &&
                       (*this).actor.xzDistToPlayer > 100.0f32 &&
                       Rand_ZeroOne() < 0.3f32 {
                    if (*this).actor.params as libc::c_int ==
                           ENZF_TYPE_DINOLFOS as libc::c_int {
                        (*this).actor.shape.rot.y =
                            (*this).actor.yawTowardsPlayer;
                        (*this).actor.world.rot.y = (*this).actor.shape.rot.y;
                        EnZf_SetupJumpForward(this);
                    } else { func_80B483E4(this, globalCtx); }
                } else if Rand_ZeroOne() > 0.3f32 {
                    EnZf_SetupApproachPlayer(this, globalCtx);
                } else { func_80B483E4(this, globalCtx); }
            } else { func_80B4604C(this); }
            if (*globalCtx).gameplayFrames &
                   0x5f as libc::c_int as libc::c_uint ==
                   0 as libc::c_int as libc::c_uint {
                Audio_PlayActorSound2(&mut (*this).actor,
                                      0x3829 as libc::c_int as u16_0);
            }
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn EnZf_SetupApproachPlayer(mut this: *mut EnZf,
                                                  mut globalCtx:
                                                      *mut GlobalContext) {
    Animation_MorphToLoop(&mut (*this).skelAnime, &mut gZfWalkingAnim,
                          -4.0f32);
    (*this).action = ENZF_ACTION_APPROACH_PLAYER as libc::c_int;
    if (*this).actor.params as libc::c_int >=
           ENZF_TYPE_LIZALFOS_MINIBOSS_A as libc::c_int {
        // miniboss
        (*this).curPlatform =
            EnZf_FindPlatform(&mut (*this).actor.world.pos,
                              (*this).curPlatform);
        (*this).nextPlatform =
            EnZf_FindNextPlatformTowardsPlayer(&mut (*this).actor.world.pos,
                                               (*this).curPlatform,
                                               (*this).homePlatform,
                                               globalCtx);
        (*this).hopAnimIndex = 0 as libc::c_int
    }
    (*this).actor.speedXZ = 0.0f32;
    EnZf_SetupAction(this,
                     Some(EnZf_ApproachPlayer as
                              unsafe extern "C" fn(_: *mut EnZf,
                                                   _: *mut GlobalContext)
                                  -> ()));
}
#[no_mangle]
pub unsafe extern "C" fn EnZf_ApproachPlayer(mut this: *mut EnZf,
                                             mut globalCtx:
                                                 *mut GlobalContext) {
    let mut sp54: s32 = 0;
    let mut sp50: s32 = 0;
    let mut temp: s32 = 0;
    let mut temp_v1: s16 = 0;
    let mut sp48: s16 = -(1 as libc::c_int) as s16;
    let mut sp44: f32_0 = 350.0f32;
    let mut sp40: f32_0 = 0.0f32;
    let mut player: *mut Player =
        (*globalCtx).actorCtx.actorLists[ACTORCAT_PLAYER as libc::c_int as
                                             usize].head as *mut Player;
    let mut sp30: s32 = 0;
    if (*this).actor.params as libc::c_int >=
           ENZF_TYPE_LIZALFOS_MINIBOSS_A as libc::c_int {
        // miniboss
        sp48 = EnZf_FindPlatform(&mut (*player).actor.world.pos, sp48);
        (*this).curPlatform =
            EnZf_FindPlatform(&mut (*this).actor.world.pos, sp48);
        if (*this).actor.world.pos.y >= 420.0f32 { sp44 = 270.0f32 }
    }
    if EnZf_DodgeRangedEngaging(globalCtx, this) == 0 {
        if sp48 as libc::c_int != (*this).curPlatform as libc::c_int {
            (*this).nextPlatform =
                EnZf_FindNextPlatformTowardsPlayer(&mut (*this).actor.world.pos,
                                                   (*this).curPlatform,
                                                   (*this).homePlatform,
                                                   globalCtx);
            if (sp48 as libc::c_int) < 0 as libc::c_int &&
                   (*this).nextPlatform as libc::c_int ==
                       (*this).curPlatform as libc::c_int {
                sp48 = (*this).curPlatform;
                (*this).actor.world.rot.y = (*this).actor.yawTowardsPlayer;
                (*this).actor.shape.rot.y = (*this).actor.world.rot.y
            } else {
                (*this).actor.yawTowardsPlayer =
                    Actor_WorldYawTowardPoint(&mut (*this).actor,
                                              &mut *sPlatformPositions.as_mut_ptr().offset((*this).nextPlatform
                                                                                               as
                                                                                               isize));
                (*this).actor.shape.rot.y = (*this).actor.yawTowardsPlayer;
                (*this).actor.world.rot.y = (*this).actor.shape.rot.y;
                temp_v1 =
                    ((*this).actor.wallYaw as libc::c_int -
                         (*this).actor.shape.rot.y as libc::c_int) as s16;
                temp_v1 =
                    if temp_v1 as libc::c_int >= 0 as libc::c_int {
                        temp_v1 as libc::c_int
                    } else { -(temp_v1 as libc::c_int) } as s16;
                if (*this).unk_3F8 as libc::c_int != 0 &&
                       (*this).actor.speedXZ > 0.0f32 ||
                       (*this).actor.bgCheckFlags as libc::c_int &
                           8 as libc::c_int != 0 &&
                           temp_v1 as libc::c_int >= 0x5c19 as libc::c_int {
                    if Actor_WorldDistXZToPoint(&mut (*this).actor,
                                                &mut *sPlatformPositions.as_mut_ptr().offset((*this).nextPlatform
                                                                                                 as
                                                                                                 isize))
                           < sp44 &&
                           EnZf_PrimaryFloorCheck(this, globalCtx,
                                                  191.9956f32) == 0 {
                        EnZf_SetupJumpForward(this);
                        if (*this).actor.bgCheckFlags as libc::c_int &
                               8 as libc::c_int != 0 {
                            (*this).actor.velocity.y = 20.0f32
                        }
                        return
                    } else {
                        (*this).actor.world.rot.y =
                            Actor_WorldYawTowardPoint(&mut (*this).actor,
                                                      &mut *sPlatformPositions.as_mut_ptr().offset((*this).curPlatform
                                                                                                       as
                                                                                                       isize))
                    }
                } else {
                    (*this).actor.world.rot.y =
                        (*this).actor.yawTowardsPlayer;
                    (*this).actor.shape.rot.y = (*this).actor.world.rot.y;
                    Math_SmoothStepToF(&mut (*this).actor.speedXZ, 8.0f32,
                                       1.0f32, 1.5f32, 0.0f32);
                }
            }
        }
        if Actor_OtherIsTargeted(globalCtx, &mut (*this).actor) != 0 {
            sp40 = 100.0f32
        }
        if (*this).actor.xzDistToPlayer <= 70.0f32 + sp40 {
            Math_SmoothStepToF(&mut (*this).actor.speedXZ, -8.0f32, 1.0f32,
                               0.5f32, 0.0f32);
        } else {
            Math_SmoothStepToF(&mut (*this).actor.speedXZ, 8.0f32, 1.0f32,
                               0.5f32, 0.0f32);
        }
        (*this).skelAnime.playSpeed = (*this).actor.speedXZ * 1.2f32;
        temp_v1 =
            ((*player).actor.shape.rot.y as libc::c_int -
                 (*this).actor.shape.rot.y as libc::c_int) as s16;
        temp_v1 =
            if temp_v1 as libc::c_int >= 0 as libc::c_int {
                temp_v1 as libc::c_int
            } else { -(temp_v1 as libc::c_int) } as s16;
        if sp48 as libc::c_int == (*this).curPlatform as libc::c_int &&
               (*this).actor.xzDistToPlayer < 150.0f32 &&
               (*player).swordState as libc::c_int != 0 as libc::c_int &&
               temp_v1 as libc::c_int >= 0x1f40 as libc::c_int {
            (*this).actor.world.rot.y = (*this).actor.yawTowardsPlayer;
            (*this).actor.shape.rot.y = (*this).actor.world.rot.y;
            if Rand_ZeroOne() > 0.7f32 {
                func_80B483E4(this, globalCtx);
                return
            }
        }
        sp54 = (*this).skelAnime.curFrame as s32;
        SkelAnime_Update(&mut (*this).skelAnime);
        sp50 =
            ((*this).skelAnime.curFrame -
                 (if (*this).skelAnime.playSpeed >=
                         0 as libc::c_int as libc::c_float {
                      (*this).skelAnime.playSpeed
                  } else { -(*this).skelAnime.playSpeed })) as s32;
        sp30 =
            if (*this).skelAnime.playSpeed >=
                   0 as libc::c_int as libc::c_float {
                (*this).skelAnime.playSpeed
            } else { -(*this).skelAnime.playSpeed } as s32;
        if sp48 as libc::c_int == (*this).curPlatform as libc::c_int {
            if Actor_IsFacingPlayer(&mut (*this).actor,
                                    0x11c7 as libc::c_int as s16) == 0 {
                if Rand_ZeroOne() > 0.5f32 {
                    func_80B462E4(this, globalCtx);
                } else { func_80B45384(this); }
            } else if (*this).actor.xzDistToPlayer < 100.0f32 {
                if Rand_ZeroOne() > 0.05f32 &&
                       EnZf_CanAttack(globalCtx, this) != 0 {
                    EnZf_SetupSlash(this);
                } else if Rand_ZeroOne() > 0.5f32 {
                    func_80B483E4(this, globalCtx);
                } else { func_80B45384(this); }
            } else if (*this).unk_3F8 != 0 {
                func_80B462E4(this, globalCtx);
            } else if Rand_ZeroOne() < 0.1f32 { func_80B45384(this); }
        }
        if (*this).actor.params as libc::c_int ==
               ENZF_TYPE_DINOLFOS as libc::c_int {
            if EnZf_ChooseAction(globalCtx, this) != 0 { return }
            if (*this).actor.xzDistToPlayer < 180.0f32 &&
                   (*this).actor.xzDistToPlayer > 160.0f32 &&
                   Actor_IsFacingPlayer(&mut (*this).actor,
                                        0x71c as libc::c_int as s16) != 0 {
                if Actor_IsTargeted(globalCtx, &mut (*this).actor) != 0 {
                    if Rand_ZeroOne() < 0.1f32 {
                        (*this).actor.shape.rot.y =
                            (*this).actor.yawTowardsPlayer;
                        (*this).actor.world.rot.y = (*this).actor.shape.rot.y;
                        EnZf_SetupJumpForward(this);
                        return
                    }
                } else { func_80B483E4(this, globalCtx); return }
            }
        }
        if (*globalCtx).gameplayFrames & 0x5f as libc::c_int as libc::c_uint
               == 0 as libc::c_int as libc::c_uint {
            Audio_PlayActorSound2(&mut (*this).actor,
                                  0x3829 as libc::c_int as u16_0);
        }
        if sp54 != (*this).skelAnime.curFrame as s32 {
            temp = sp30 + sp54;
            if sp50 < 2 as libc::c_int && temp >= 4 as libc::c_int ||
                   sp50 < 32 as libc::c_int && temp >= 34 as libc::c_int {
                Audio_PlayActorSound2(&mut (*this).actor,
                                      0x382e as libc::c_int as u16_0);
            }
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn EnZf_SetupJumpForward(mut this: *mut EnZf) {
    Animation_Change(&mut (*this).skelAnime, &mut gZfJumpingAnim, 1.0f32,
                     0.0f32, 3.0f32, ANIMMODE_ONCE as libc::c_int as u8_0,
                     -3.0f32);
    (*this).unk_3F0 = 0 as libc::c_int;
    (*this).hopAnimIndex = 1 as libc::c_int;
    (*this).actor.velocity.y = 15.0f32;
    if (*this).actor.params as libc::c_int >=
           ENZF_TYPE_LIZALFOS_MINIBOSS_A as libc::c_int {
        // miniboss
        (*this).actor.speedXZ = 16.0f32
    } else { (*this).actor.speedXZ = 10.0f32 }
    (*this).action = ENZF_ACTION_JUMP_FORWARD as libc::c_int;
    Audio_PlayActorSound2(&mut (*this).actor, 0x382f as libc::c_int as u16_0);
    EnZf_SetupAction(this,
                     Some(EnZf_JumpForward as
                              unsafe extern "C" fn(_: *mut EnZf,
                                                   _: *mut GlobalContext)
                                  -> ()));
}
#[no_mangle]
pub unsafe extern "C" fn EnZf_JumpForward(mut this: *mut EnZf,
                                          mut globalCtx: *mut GlobalContext) {
    if (*this).unk_3F0 != 0 as libc::c_int &&
           (*this).actor.world.pos.y <= (*this).actor.floorHeight {
        (*this).actor.world.pos.y = (*this).actor.floorHeight;
        (*this).hopAnimIndex = 0 as libc::c_int;
        (*this).actor.velocity.y = 0.0f32;
        (*this).actor.speedXZ = 0.0f32
    }
    if SkelAnime_Update(&mut (*this).skelAnime) != 0 {
        if (*this).unk_3F0 == 0 as libc::c_int {
            Animation_Change(&mut (*this).skelAnime, &mut gZfLandingAnim,
                             3.0f32, 0.0f32, 17.0f32,
                             ANIMMODE_ONCE as libc::c_int as u8_0, -3.0f32);
            (*this).unk_3F0 = 10 as libc::c_int;
            Audio_PlayActorSound2(&mut (*this).actor,
                                  0x382f as libc::c_int as u16_0);
        } else {
            (*this).actor.speedXZ = 0.0f32;
            (*this).hopAnimIndex = 0 as libc::c_int;
            EnZf_SetupApproachPlayer(this, globalCtx);
        }
    }
    if (*globalCtx).gameplayFrames & 0x5f as libc::c_int as libc::c_uint ==
           0 as libc::c_int as libc::c_uint {
        Audio_PlayActorSound2(&mut (*this).actor,
                              0x3829 as libc::c_int as u16_0);
    }
    if (*this).actor.params as libc::c_int ==
           ENZF_TYPE_DINOLFOS as libc::c_int &&
           (*this).actor.bgCheckFlags as libc::c_int & 3 as libc::c_int != 0 {
        if EnZf_CanAttack(globalCtx, this) != 0 {
            EnZf_SetupSlash(this);
        } else { func_80B483E4(this, globalCtx); }
    };
}
#[no_mangle]
pub unsafe extern "C" fn func_80B4604C(mut this: *mut EnZf) {
    Animation_MorphToLoop(&mut (*this).skelAnime, &mut gZfWalkingAnim,
                          -4.0f32);
    (*this).action = ENZF_ACTION_6 as libc::c_int;
    EnZf_SetupAction(this,
                     Some(func_80B46098 as
                              unsafe extern "C" fn(_: *mut EnZf,
                                                   _: *mut GlobalContext)
                                  -> ()));
}
#[no_mangle]
pub unsafe extern "C" fn func_80B46098(mut this: *mut EnZf,
                                       mut globalCtx: *mut GlobalContext) {
    let mut pad: s32 = 0;
    let mut phi_f2: f32_0 = 0.;
    let mut player: *mut Player =
        (*globalCtx).actorCtx.actorLists[ACTORCAT_PLAYER as libc::c_int as
                                             usize].head as *mut Player;
    let mut temp_v0: s16 = 0;
    let mut phi_v1: s16 = 0;
    if EnZf_DodgeRangedEngaging(globalCtx, this) == 0 {
        if (*this).actor.params as libc::c_int !=
               ENZF_TYPE_DINOLFOS as libc::c_int ||
               EnZf_ChooseAction(globalCtx, this) == 0 {
            temp_v0 =
                ((*this).actor.yawTowardsPlayer as libc::c_int -
                     (*this).actor.shape.rot.y as libc::c_int) as s16;
            if temp_v0 as libc::c_int > 0 as libc::c_int {
                phi_v1 =
                    (temp_v0 as libc::c_int as libc::c_float * 0.25f32 +
                         2000.0f32) as s16
            } else {
                phi_v1 =
                    (temp_v0 as libc::c_int as libc::c_float * 0.25f32 -
                         2000.0f32) as s16
            }
            (*this).actor.shape.rot.y =
                ((*this).actor.shape.rot.y as libc::c_int +
                     phi_v1 as libc::c_int) as s16;
            (*this).actor.world.rot.y = (*this).actor.shape.rot.y;
            if temp_v0 as libc::c_int > 0 as libc::c_int {
                phi_f2 = phi_v1 as libc::c_int as libc::c_float * 1.5f32;
                if phi_f2 > 2.0f32 { phi_f2 = 2.0f32 }
            } else {
                phi_f2 = phi_v1 as libc::c_int as libc::c_float * 1.5f32;
                if phi_f2 < -2.0f32 { phi_f2 = -2.0f32 }
            }
            (*this).skelAnime.playSpeed = -phi_f2;
            SkelAnime_Update(&mut (*this).skelAnime);
            if (*this).actor.params as libc::c_int >=
                   ENZF_TYPE_LIZALFOS_MINIBOSS_A as libc::c_int {
                // miniboss
                (*this).curPlatform =
                    EnZf_FindPlatform(&mut (*this).actor.world.pos,
                                      (*this).curPlatform);
                if (*this).curPlatform as libc::c_int !=
                       EnZf_FindPlatform(&mut (*player).actor.world.pos,
                                         -(1 as libc::c_int) as s16) as
                           libc::c_int {
                    EnZf_SetupApproachPlayer(this, globalCtx);
                    return
                }
            }
            if Actor_IsFacingPlayer(&mut (*this).actor,
                                    (30 as libc::c_int *
                                         0x10000 as libc::c_int /
                                         360 as libc::c_int) as s16) != 0 {
                if Rand_ZeroOne() > 0.8f32 {
                    func_80B462E4(this, globalCtx);
                } else { EnZf_SetupApproachPlayer(this, globalCtx); }
            }
            if (*globalCtx).gameplayFrames &
                   0x5f as libc::c_int as libc::c_uint ==
                   0 as libc::c_int as libc::c_uint {
                Audio_PlayActorSound2(&mut (*this).actor,
                                      0x3829 as libc::c_int as u16_0);
            }
        }
    };
}
// Conditional setup function
#[no_mangle]
pub unsafe extern "C" fn func_80B462E4(mut this: *mut EnZf,
                                       mut globalCtx: *mut GlobalContext) {
    if ((*this).actor.params as libc::c_int) <
           ENZF_TYPE_LIZALFOS_MINIBOSS_A as libc::c_int ||
           Actor_TestFloorInDirection(&mut (*this).actor, globalCtx, 40.0f32,
                                      ((*this).actor.shape.rot.y as
                                           libc::c_int +
                                           0x3fff as libc::c_int) as s16) as
               libc::c_int != 0 ||
           Actor_TestFloorInDirection(&mut (*this).actor, globalCtx, -40.0f32,
                                      ((*this).actor.shape.rot.y as
                                           libc::c_int +
                                           0x3fff as libc::c_int) as s16) as
               libc::c_int != 0 {
        Animation_PlayLoop(&mut (*this).skelAnime, &mut gZfSidesteppingAnim);
        (*this).actor.speedXZ = Rand_CenteredFloat(12.0f32);
        (*this).actor.world.rot.y = (*this).actor.shape.rot.y;
        (*this).unk_3F0 = (Rand_ZeroOne() * 10.0f32 + 20.0f32) as s32;
        (*this).hopAnimIndex = 0 as libc::c_int;
        (*this).action = ENZF_ACTION_7 as libc::c_int;
        (*this).unk_408 = 0.0f32;
        EnZf_SetupAction(this,
                         Some(func_80B463E4 as
                                  unsafe extern "C" fn(_: *mut EnZf,
                                                       _: *mut GlobalContext)
                                      -> ()));
    } else { EnZf_SetupApproachPlayer(this, globalCtx); };
}
#[no_mangle]
pub unsafe extern "C" fn func_80B463E4(mut this: *mut EnZf,
                                       mut globalCtx: *mut GlobalContext) {
    let mut angleBehindPlayer: s16 = 0;
    let mut phi_v0_3: s16 = 0;
    let mut pad: s32 = 0;
    let mut curKeyFrame: s32 = 0;
    let mut prevKeyFrame: s32 = 0;
    let mut playSpeed: s32 = 0;
    let mut player: *mut Player =
        (*globalCtx).actorCtx.actorLists[ACTORCAT_PLAYER as libc::c_int as
                                             usize].head as *mut Player;
    let mut baseRange: f32_0 = 0.0f32;
    Math_SmoothStepToS(&mut (*this).actor.shape.rot.y,
                       (*this).actor.yawTowardsPlayer,
                       1 as libc::c_int as s16, 4000 as libc::c_int as s16,
                       1 as libc::c_int as s16);
    if EnZf_DodgeRangedEngaging(globalCtx, this) == 0 &&
           ((*this).actor.params as libc::c_int !=
                ENZF_TYPE_DINOLFOS as libc::c_int ||
                EnZf_ChooseAction(globalCtx, this) == 0) {
        (*this).actor.world.rot.y =
            ((*this).actor.shape.rot.y as libc::c_int + 0x3a98 as libc::c_int)
                as s16;
        angleBehindPlayer =
            ((*player).actor.shape.rot.y as libc::c_int +
                 0x8000 as libc::c_int) as s16;
        if Math_SinS((angleBehindPlayer as libc::c_int -
                          (*this).actor.shape.rot.y as libc::c_int) as s16) >=
               0.0f32 {
            (*this).actor.speedXZ -= 0.25f32;
            if (*this).actor.speedXZ < -8.0f32 {
                (*this).actor.speedXZ = -8.0f32
            }
        } else if Math_SinS((angleBehindPlayer as libc::c_int -
                                 (*this).actor.shape.rot.y as libc::c_int) as
                                s16) < 0.0f32 {
            // Superfluous check
            (*this).actor.speedXZ += 0.25f32;
            if (*this).actor.speedXZ > 8.0f32 {
                (*this).actor.speedXZ = 8.0f32
            }
        }
        if (*this).actor.params as libc::c_int >=
               ENZF_TYPE_LIZALFOS_MINIBOSS_A as libc::c_int {
            // miniboss
            if (*this).unk_3F8 != 0 {
                (*this).actor.speedXZ = -(*this).actor.speedXZ
            }
        } else if (*this).actor.bgCheckFlags as libc::c_int & 8 as libc::c_int
                      != 0 ||
                      Actor_TestFloorInDirection(&mut (*this).actor,
                                                 globalCtx,
                                                 (*this).actor.speedXZ,
                                                 ((*this).actor.shape.rot.y as
                                                      libc::c_int +
                                                      0x3fff as libc::c_int)
                                                     as s16) == 0 {
            if (*this).actor.bgCheckFlags as libc::c_int & 8 as libc::c_int !=
                   0 {
                if (*this).actor.speedXZ >= 0.0f32 {
                    phi_v0_3 =
                        ((*this).actor.shape.rot.y as libc::c_int +
                             0x3fff as libc::c_int) as s16
                } else {
                    phi_v0_3 =
                        ((*this).actor.shape.rot.y as libc::c_int -
                             0x3fff as libc::c_int) as s16
                }
                phi_v0_3 =
                    ((*this).actor.wallYaw as libc::c_int -
                         phi_v0_3 as libc::c_int) as s16
            } else {
                (*this).actor.speedXZ *= -0.8f32;
                phi_v0_3 = 0 as libc::c_int as s16
            }
            if (if phi_v0_3 as libc::c_int >= 0 as libc::c_int {
                    phi_v0_3 as libc::c_int
                } else { -(phi_v0_3 as libc::c_int) }) > 0x4000 as libc::c_int
               {
                (*this).actor.speedXZ *= -0.8f32;
                if (*this).actor.speedXZ < 0.0f32 {
                    (*this).actor.speedXZ -= 0.5f32
                } else { (*this).actor.speedXZ += 0.5f32 }
            }
        }
        if Actor_OtherIsTargeted(globalCtx, &mut (*this).actor) != 0 {
            baseRange = 100.0f32
        }
        if (*this).actor.xzDistToPlayer <= 70.0f32 + baseRange {
            Math_SmoothStepToF(&mut (*this).unk_408, -4.0f32, 1.0f32, 1.5f32,
                               0.0f32);
        } else if 90.0f32 + baseRange < (*this).actor.xzDistToPlayer {
            Math_SmoothStepToF(&mut (*this).unk_408, 4.0f32, 1.0f32, 1.5f32,
                               0.0f32);
        } else {
            Math_SmoothStepToF(&mut (*this).unk_408, 0.0f32, 1.0f32, 5.65f32,
                               0.0f32);
        }
        if (*this).unk_408 != 0.0f32 &&
               EnZf_SecondaryFloorCheck(this, globalCtx, (*this).unk_408) == 0
           {
            (*this).actor.world.pos.x +=
                Math_SinS((*this).actor.shape.rot.y) * (*this).unk_408;
            (*this).actor.world.pos.z +=
                Math_CosS((*this).actor.shape.rot.y) * (*this).unk_408
        }
        if (if (*this).actor.speedXZ >= 0 as libc::c_int as libc::c_float {
                (*this).actor.speedXZ
            } else { -(*this).actor.speedXZ }) >=
               (if (*this).unk_408 >= 0 as libc::c_int as libc::c_float {
                    (*this).unk_408
                } else { -(*this).unk_408 }) {
            (*this).skelAnime.playSpeed = (*this).actor.speedXZ * 0.75f32
        } else if (*this).skelAnime.playSpeed < 0.0f32 {
            (*this).skelAnime.playSpeed = (*this).unk_408 * -0.75f32
        } else { (*this).skelAnime.playSpeed = (*this).unk_408 * 0.75f32 }
        curKeyFrame = (*this).skelAnime.curFrame as s32;
        SkelAnime_Update(&mut (*this).skelAnime);
        prevKeyFrame =
            ((*this).skelAnime.curFrame -
                 (if (*this).skelAnime.playSpeed >=
                         0 as libc::c_int as libc::c_float {
                      (*this).skelAnime.playSpeed
                  } else { -(*this).skelAnime.playSpeed })) as s32;
        playSpeed =
            if (*this).skelAnime.playSpeed >=
                   0 as libc::c_int as libc::c_float {
                (*this).skelAnime.playSpeed
            } else { -(*this).skelAnime.playSpeed } as s32;
        if curKeyFrame != (*this).skelAnime.curFrame as s32 {
            let mut nextKeyFrame: s32 = playSpeed + curKeyFrame;
            if prevKeyFrame < 14 as libc::c_int &&
                   nextKeyFrame > 15 as libc::c_int ||
                   prevKeyFrame < 27 as libc::c_int &&
                       nextKeyFrame > 28 as libc::c_int {
                Audio_PlayActorSound2(&mut (*this).actor,
                                      0x382e as libc::c_int as u16_0);
            }
        }
        if (*globalCtx).gameplayFrames & 0x5f as libc::c_int as libc::c_uint
               == 0 as libc::c_int as libc::c_uint {
            Audio_PlayActorSound2(&mut (*this).actor,
                                  0x3829 as libc::c_int as u16_0);
        }
        if Math_CosS((angleBehindPlayer as libc::c_int -
                          (*this).actor.shape.rot.y as libc::c_int) as s16) <
               -0.85f32 || (*this).unk_3F0 == 0 as libc::c_int {
            (*this).actor.world.rot.y = (*this).actor.shape.rot.y;
            if (*this).actor.xzDistToPlayer <= 100.0f32 &&
                   (*globalCtx).gameplayFrames.wrapping_rem(4 as libc::c_int
                                                                as
                                                                libc::c_uint)
                       == 0 as libc::c_int as libc::c_uint &&
                   EnZf_CanAttack(globalCtx, this) != 0 {
                EnZf_SetupSlash(this);
            } else { func_80B45384(this); }
        } else if (*this).unk_3F0 != 0 as libc::c_int { (*this).unk_3F0 -= 1 }
    };
}
#[no_mangle]
pub unsafe extern "C" fn EnZf_SetupSlash(mut this: *mut EnZf) {
    Animation_Change(&mut (*this).skelAnime, &mut gZfSlashAnim, 1.25f32,
                     0.0f32,
                     Animation_GetLastFrame(&mut gZfSlashAnim as
                                                *mut AnimationHeader as
                                                *mut libc::c_void) as f32_0,
                     ANIMMODE_ONCE as libc::c_int as u8_0, -4.0f32);
    if (*this).actor.params as libc::c_int ==
           ENZF_TYPE_DINOLFOS as libc::c_int {
        (*this).skelAnime.playSpeed = 1.75f32
    }
    (*this).swordCollider.base.atFlags =
        ((*this).swordCollider.base.atFlags as libc::c_int &
             !((1 as libc::c_int) << 2 as libc::c_int)) as u8_0;
    (*this).action = ENZF_ACTION_SLASH as libc::c_int;
    Audio_PlayActorSound2(&mut (*this).actor, 0x3829 as libc::c_int as u16_0);
    (*this).actor.speedXZ = 0.0f32;
    EnZf_SetupAction(this,
                     Some(EnZf_Slash as
                              unsafe extern "C" fn(_: *mut EnZf,
                                                   _: *mut GlobalContext)
                                  -> ()));
}
#[no_mangle]
pub unsafe extern "C" fn EnZf_Slash(mut this: *mut EnZf,
                                    mut globalCtx: *mut GlobalContext) {
    let mut player: *mut Player =
        (*globalCtx).actorCtx.actorLists[ACTORCAT_PLAYER as libc::c_int as
                                             usize].head as *mut Player;
    let mut rotDiff: s16 = 0;
    let mut yawDiff: s16 = 0;
    (*this).actor.speedXZ = 0.0f32;
    if (*this).skelAnime.curFrame as s32 == 10 as libc::c_int {
        Audio_PlayActorSound2(&mut (*this).actor,
                              0x382a as libc::c_int as u16_0);
    }
    if SkelAnime_Update(&mut (*this).skelAnime) != 0 {
        EffectBlure_AddSpace(Effect_GetByIndex((*this).blureIndex) as
                                 *mut EffectBlure);
        if (*this).actor.params as libc::c_int ==
               ENZF_TYPE_DINOLFOS as libc::c_int &&
               Actor_IsFacingPlayer(&mut (*this).actor,
                                    5460 as libc::c_int as s16) == 0 {
            func_80B45384(this);
            (*this).unk_3F0 = (Rand_ZeroOne() * 5.0f32 + 5.0f32) as s32;
            (*this).unk_3F4 = (Rand_ZeroOne() * 20.0f32 + 100.0f32) as s16
        } else if Rand_ZeroOne() > 0.7f32 ||
                      (*this).actor.xzDistToPlayer >= 120.0f32 {
            func_80B45384(this);
            (*this).unk_3F0 = (Rand_ZeroOne() * 5.0f32 + 5.0f32) as s32
        } else {
            (*this).actor.world.rot.y = (*this).actor.yawTowardsPlayer;
            if Rand_ZeroOne() > 0.7f32 {
                func_80B483E4(this, globalCtx);
            } else {
                rotDiff =
                    ((*player).actor.shape.rot.y as libc::c_int -
                         (*this).actor.shape.rot.y as libc::c_int) as s16;
                rotDiff =
                    if rotDiff as libc::c_int >= 0 as libc::c_int {
                        rotDiff as libc::c_int
                    } else { -(rotDiff as libc::c_int) } as s16;
                if rotDiff as libc::c_int <= 10000 as libc::c_int {
                    yawDiff =
                        ((*this).actor.yawTowardsPlayer as libc::c_int -
                             (*this).actor.shape.rot.y as libc::c_int) as s16;
                    yawDiff =
                        if yawDiff as libc::c_int >= 0 as libc::c_int {
                            yawDiff as libc::c_int
                        } else { -(yawDiff as libc::c_int) } as s16;
                    if yawDiff as libc::c_int > 16000 as libc::c_int {
                        (*this).actor.world.rot.y =
                            (*this).actor.yawTowardsPlayer;
                        func_80B483E4(this, globalCtx);
                    } else if (*player).stateFlags1 &
                                  0x6010 as libc::c_int as libc::c_uint != 0 {
                        if (*this).actor.isTargeted != 0 {
                            EnZf_SetupSlash(this);
                        } else { func_80B483E4(this, globalCtx); }
                    } else { EnZf_SetupSlash(this); }
                } else { func_80B483E4(this, globalCtx); }
            }
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn EnZf_SetupRecoilFromBlockedSlash(mut this:
                                                              *mut EnZf) {
    let mut frame: f32_0 = (*this).skelAnime.curFrame - 3.0f32;
    Animation_Change(&mut (*this).skelAnime, &mut gZfSlashAnim, -1.0f32,
                     frame, 0.0f32, ANIMMODE_ONCE as libc::c_int as u8_0,
                     0.0f32);
    (*this).action = ENZF_ACTION_RECOIL_FROM_BLOCKED_SLASH as libc::c_int;
    EnZf_SetupAction(this,
                     Some(EnZf_RecoilFromBlockedSlash as
                              unsafe extern "C" fn(_: *mut EnZf,
                                                   _: *mut GlobalContext)
                                  -> ()));
}
#[no_mangle]
pub unsafe extern "C" fn EnZf_RecoilFromBlockedSlash(mut this: *mut EnZf,
                                                     mut globalCtx:
                                                         *mut GlobalContext) {
    if SkelAnime_Update(&mut (*this).skelAnime) != 0 {
        if Rand_ZeroOne() > 0.7f32 {
            func_80B45384(this);
        } else if Rand_ZeroOne() > 0.2f32 &&
                      EnZf_CanAttack(globalCtx, this) != 0 {
            EnZf_SetupSlash(this);
        } else { func_80B483E4(this, globalCtx); }
    };
}
#[no_mangle]
pub unsafe extern "C" fn EnZf_SetupJumpBack(mut this: *mut EnZf) {
    Animation_Change(&mut (*this).skelAnime, &mut gZfJumpingAnim, -1.0f32,
                     3.0f32, 0.0f32, ANIMMODE_ONCE as libc::c_int as u8_0,
                     -3.0f32);
    (*this).unk_3F0 = 0 as libc::c_int;
    (*this).hopAnimIndex = 1 as libc::c_int;
    (*this).action = ENZF_ACTION_JUMP_BACK as libc::c_int;
    (*this).actor.velocity.y = 15.0f32;
    (*this).actor.speedXZ = -15.0f32;
    Audio_PlayActorSound2(&mut (*this).actor, 0x382f as libc::c_int as u16_0);
    EnZf_SetupAction(this,
                     Some(EnZf_JumpBack as
                              unsafe extern "C" fn(_: *mut EnZf,
                                                   _: *mut GlobalContext)
                                  -> ()));
}
#[no_mangle]
pub unsafe extern "C" fn EnZf_JumpBack(mut this: *mut EnZf,
                                       mut globalCtx: *mut GlobalContext) {
    if (*this).unk_3F0 != 0 as libc::c_int &&
           (*this).actor.world.pos.y <= (*this).actor.floorHeight {
        (*this).actor.world.pos.y = (*this).actor.floorHeight;
        (*this).hopAnimIndex = 0 as libc::c_int;
        (*this).actor.velocity.y = 0.0f32;
        (*this).actor.speedXZ = 0.0f32
    }
    if SkelAnime_Update(&mut (*this).skelAnime) != 0 {
        if (*this).unk_3F0 == 0 as libc::c_int {
            Animation_Change(&mut (*this).skelAnime, &mut gZfLandingAnim,
                             3.0f32, 0.0f32, 17.0f32,
                             ANIMMODE_ONCE as libc::c_int as u8_0, -3.0f32);
            (*this).unk_3F0 = 10 as libc::c_int;
            Audio_PlayActorSound2(&mut (*this).actor,
                                  0x382f as libc::c_int as u16_0);
        } else if (*globalCtx).gameplayFrames.wrapping_rem(2 as libc::c_int as
                                                               libc::c_uint)
                      != 0 as libc::c_int as libc::c_uint {
            func_80B483E4(this, globalCtx);
        } else { func_80B45384(this); }
    }
    if (*globalCtx).state.frames & 0x5f as libc::c_int as libc::c_uint ==
           0 as libc::c_int as libc::c_uint {
        Audio_PlayActorSound2(&mut (*this).actor,
                              0x3829 as libc::c_int as u16_0);
    };
}
#[no_mangle]
pub unsafe extern "C" fn EnZf_SetupStunned(mut this: *mut EnZf) {
    if (*this).actor.bgCheckFlags as libc::c_int & 1 as libc::c_int != 0 &&
           ((*this).actor.velocity.y == 0.0f32 ||
                (*this).actor.velocity.y == -4.0f32) {
        (*this).actor.speedXZ = 0.0f32;
        (*this).hopAnimIndex = 0 as libc::c_int
    } else { (*this).hopAnimIndex = 1 as libc::c_int }
    if (*this).damageEffect as libc::c_int == ENZF_DMGEFF_ICE as libc::c_int {
        (*this).iceTimer = 36 as libc::c_int as s16
    } else {
        Animation_PlayOnceSetSpeed(&mut (*this).skelAnime,
                                   &mut gZfKnockedBackAnim, 0.0f32);
    }
    Audio_PlayActorSound2(&mut (*this).actor, 0x389e as libc::c_int as u16_0);
    (*this).action = ENZF_ACTION_STUNNED as libc::c_int;
    EnZf_SetupAction(this,
                     Some(EnZf_Stunned as
                              unsafe extern "C" fn(_: *mut EnZf,
                                                   _: *mut GlobalContext)
                                  -> ()));
}
#[no_mangle]
pub unsafe extern "C" fn EnZf_Stunned(mut this: *mut EnZf,
                                      mut globalCtx: *mut GlobalContext) {
    let mut angleToWall: s16 = 0;
    if (*this).actor.bgCheckFlags as libc::c_int & 2 as libc::c_int != 0 {
        (*this).actor.speedXZ = 0.0f32
    }
    if (*this).actor.bgCheckFlags as libc::c_int & 1 as libc::c_int != 0 {
        if (*this).actor.speedXZ < 0.0f32 { (*this).actor.speedXZ += 0.05f32 }
        (*this).hopAnimIndex = 0 as libc::c_int
    }
    if (*this).actor.colorFilterTimer as libc::c_int == 0 as libc::c_int &&
           (*this).actor.bgCheckFlags as libc::c_int & 1 as libc::c_int != 0 {
        if (*this).actor.colChkInfo.health as libc::c_int == 0 as libc::c_int
           {
            EnZf_SetupDie(this);
        } else if (*this).actor.params as libc::c_int !=
                      ENZF_TYPE_DINOLFOS as libc::c_int ||
                      EnZf_ChooseAction(globalCtx, this) == 0 {
            if D_80B4A1B4 as libc::c_int != -(1 as libc::c_int) {
                func_80B44DC4(this, globalCtx);
            } else {
                angleToWall =
                    ((*this).actor.wallYaw as libc::c_int -
                         (*this).actor.shape.rot.y as libc::c_int) as s16;
                angleToWall =
                    if angleToWall as libc::c_int >= 0 as libc::c_int {
                        angleToWall as libc::c_int
                    } else { -(angleToWall as libc::c_int) } as s16;
                if (*this).actor.params as libc::c_int ==
                       ENZF_TYPE_DINOLFOS as libc::c_int &&
                       (*this).actor.bgCheckFlags as libc::c_int &
                           8 as libc::c_int != 0 &&
                       (if angleToWall as libc::c_int >= 0 as libc::c_int {
                            angleToWall as libc::c_int
                        } else { -(angleToWall as libc::c_int) }) <
                           0x2ee0 as libc::c_int &&
                       (*this).actor.xzDistToPlayer < 90.0f32 {
                    (*this).actor.world.rot.y = (*this).actor.shape.rot.y;
                    EnZf_SetupJumpUp(this);
                } else if EnZf_DodgeRangedEngaging(globalCtx, this) == 0 {
                    if (*this).actor.params as libc::c_int !=
                           ENZF_TYPE_DINOLFOS as libc::c_int {
                        func_80B44DC4(this, globalCtx);
                    } else if (*this).actor.xzDistToPlayer <= 100.0f32 &&
                                  (*globalCtx).gameplayFrames.wrapping_rem(4
                                                                               as
                                                                               libc::c_int
                                                                               as
                                                                               libc::c_uint)
                                      != 0 as libc::c_int as libc::c_uint &&
                                  EnZf_CanAttack(globalCtx, this) != 0 {
                        EnZf_SetupSlash(this);
                    } else { func_80B44DC4(this, globalCtx); }
                }
            }
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn EnZf_SetupSheatheSword(mut this: *mut EnZf,
                                                mut globalCtx:
                                                    *mut GlobalContext) {
    let mut morphFrames: f32_0 = 0.0f32;
    let mut lastFrame: f32_0 =
        Animation_GetLastFrame(&mut gZfSheathingSwordAnim as
                                   *mut AnimationHeader as *mut libc::c_void)
            as f32_0;
    if (*this).action <= ENZF_ACTION_DAMAGED as libc::c_int {
        morphFrames = -4.0f32
    }
    Animation_Change(&mut (*this).skelAnime, &mut gZfSheathingSwordAnim,
                     2.0f32, 0.0f32, lastFrame,
                     ANIMMODE_ONCE as libc::c_int as u8_0, morphFrames);
    (*this).action = ENZF_ACTION_SHEATHE_SWORD as libc::c_int;
    (*this).actor.speedXZ = 0.0f32;
    (*this).curPlatform =
        EnZf_FindPlatform(&mut (*this).actor.world.pos, (*this).curPlatform);
    (*this).nextPlatform =
        EnZf_FindNextPlatformAwayFromPlayer(&mut (*this).actor.world.pos,
                                            (*this).curPlatform,
                                            (*this).homePlatform, globalCtx);
    (*this).actor.world.rot.y = (*this).actor.shape.rot.y;
    EnZf_SetupAction(this,
                     Some(EnZf_SheatheSword as
                              unsafe extern "C" fn(_: *mut EnZf,
                                                   _: *mut GlobalContext)
                                  -> ()));
}
#[no_mangle]
pub unsafe extern "C" fn EnZf_SheatheSword(mut this: *mut EnZf,
                                           mut globalCtx:
                                               *mut GlobalContext) {
    let mut yaw: s16 =
        (Actor_WorldYawTowardPoint(&mut (*this).actor,
                                   &mut *sPlatformPositions.as_mut_ptr().offset((*this).nextPlatform
                                                                                    as
                                                                                    isize))
             as libc::c_int + 0x8000 as libc::c_int) as s16;
    Math_SmoothStepToS(&mut (*this).actor.world.rot.y, yaw,
                       1 as libc::c_int as s16, 1000 as libc::c_int as s16,
                       0 as libc::c_int as s16);
    (*this).actor.shape.rot.y = (*this).actor.world.rot.y;
    if SkelAnime_Update(&mut (*this).skelAnime) != 0 {
        (*this).actor.world.rot.y =
            (yaw as libc::c_int - 0x8000 as libc::c_int) as s16;
        EnZf_SetupHopAway(this, globalCtx);
        (*this).swordSheathed = 1 as libc::c_int as s16
    };
}
#[no_mangle]
pub unsafe extern "C" fn EnZf_SetupHopAndTaunt(mut this: *mut EnZf) {
    (*this).hopAnimIndex = 0 as libc::c_int;
    Animation_MorphToPlayOnce(&mut (*this).skelAnime,
                              sHoppingAnims[0 as libc::c_int as usize],
                              -4.0f32);
    (*this).action = ENZF_ACTION_HOP_AND_TAUNT as libc::c_int;
    (*this).actor.speedXZ = 0.0f32;
    (*this).unk_40C = 0.0f32;
    (*this).unk_408 = 0.0f32;
    EnZf_SetupAction(this,
                     Some(EnZf_HopAndTaunt as
                              unsafe extern "C" fn(_: *mut EnZf,
                                                   _: *mut GlobalContext)
                                  -> ()));
}
#[no_mangle]
pub unsafe extern "C" fn EnZf_HopAndTaunt(mut this: *mut EnZf,
                                          mut globalCtx: *mut GlobalContext) {
    let mut lastFrame: f32_0 = 0.;
    let mut maxDist: f32_0 = 400.0f32;
    Math_SmoothStepToF(&mut (*this).actor.speedXZ, 0.0f32, 1.0f32, 0.5f32,
                       0.0f32);
    Math_SmoothStepToS(&mut (*this).actor.shape.rot.y,
                       ((*this).actor.yawTowardsPlayer as libc::c_int +
                            0x8000 as libc::c_int) as s16,
                       1 as libc::c_int as s16, 4000 as libc::c_int as s16,
                       0 as libc::c_int as s16);
    // Upstairs
    if (*this).actor.world.pos.y >= 420.0f32 { maxDist = 250.0f32 }
    // If player gets too close, run away
    if (*this).actor.xzDistToPlayer < maxDist &&
           (*this).hopAnimIndex != 1 as libc::c_int {
        (*this).actor.shape.rot.y =
            (*this).actor.world.rot.y; // move on to next animation
        EnZf_SetupSheatheSword(this, globalCtx);
    } else {
        if (*this).hopAnimIndex != 1 as libc::c_int {
            EnZf_DodgeRangedWaiting(globalCtx, this);
        }
        if SkelAnime_Update(&mut (*this).skelAnime) != 0 {
            (*this).hopAnimIndex += 1;
            // Loop back to beginning
            if (*this).hopAnimIndex >=
                   (::std::mem::size_of::<[*mut AnimationHeader; 3]>() as
                        libc::c_ulong).wrapping_div(::std::mem::size_of::<*mut AnimationHeader>()
                                                        as libc::c_ulong) as
                       s32 {
                (*this).hopAnimIndex = 0 as libc::c_int
            }
            if (*this).unk_408 != 0.0f32 || (*this).unk_40C != 0.0f32 {
                (*this).hopAnimIndex = 1 as libc::c_int
            }
            lastFrame =
                Animation_GetLastFrame(sHoppingAnims[(*this).hopAnimIndex as
                                                         usize] as
                                           *mut libc::c_void) as f32_0;
            match (*this).hopAnimIndex {
                0 => {
                    (*this).actor.velocity.y = 0.0f32;
                    (*this).actor.world.pos.y = (*this).actor.floorHeight
                }
                1 => {
                    (*this).actor.velocity.y = (*this).unk_40C + 10.0f32;
                    (*this).actor.speedXZ = (*this).unk_408;
                    (*this).unk_408 = 0.0f32;
                    (*this).unk_40C = 0.0f32
                }
                2 => {
                    (*this).actor.world.pos.y = (*this).actor.floorHeight;
                    lastFrame = 3.0f32
                }
                _ => { }
            }
            Animation_Change(&mut (*this).skelAnime,
                             sHoppingAnims[(*this).hopAnimIndex as usize],
                             1.5f32, 0.0f32, lastFrame,
                             ANIMMODE_ONCE as libc::c_int as u8_0, 0.0f32);
        }
        if (*globalCtx).gameplayFrames & 0x5f as libc::c_int as libc::c_uint
               == 0 as libc::c_int as libc::c_uint {
            Audio_PlayActorSound2(&mut (*this).actor,
                                  0x3829 as libc::c_int as u16_0);
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn EnZf_SetupHopAway(mut this: *mut EnZf,
                                           mut globalCtx:
                                               *mut GlobalContext) {
    (*this).hopAnimIndex = 0 as libc::c_int;
    Animation_PlayOnce(&mut (*this).skelAnime,
                       sHoppingAnims[0 as libc::c_int as usize]);
    (*this).action = ENZF_ACTION_HOP_AWAY as libc::c_int;
    (*this).curPlatform =
        EnZf_FindPlatform(&mut (*this).actor.world.pos, (*this).curPlatform);
    (*this).nextPlatform =
        EnZf_FindNextPlatformAwayFromPlayer(&mut (*this).actor.world.pos,
                                            (*this).curPlatform,
                                            (*this).homePlatform, globalCtx);
    EnZf_SetupAction(this,
                     Some(EnZf_HopAway as
                              unsafe extern "C" fn(_: *mut EnZf,
                                                   _: *mut GlobalContext)
                                  -> ()));
}
#[no_mangle]
pub unsafe extern "C" fn EnZf_HopAway(mut this: *mut EnZf,
                                      mut globalCtx: *mut GlobalContext) {
    let mut sp74: f32_0 = 0.;
    let mut sp70: f32_0 = 1.0f32;
    let mut phi_f20: f32_0 = 550.0f32;
    let mut pad: s32 = 0;
    let mut phi_f20_2: f32_0 = 0.;
    let mut phi_f0: f32_0 = 0.;
    let mut pad2: s32 = 0;
    let mut sp5A: s16 = 0;
    let mut sp54: s32 = 0;
    let mut temp_v1_2: s32 = 0;
    let mut phi_v1: s32 = 0;
    sp74 =
        Actor_WorldDistXZToPoint(&mut (*this).actor,
                                 &mut *sPlatformPositions.as_mut_ptr().offset((*this).nextPlatform
                                                                                  as
                                                                                  isize));
    sp54 = (*this).hopAnimIndex;
    if (*globalCtx).gameplayFrames & 0x5f as libc::c_int as libc::c_uint ==
           0 as libc::c_int as libc::c_uint {
        Audio_PlayActorSound2(&mut (*this).actor,
                              0x3829 as libc::c_int as u16_0);
    }
    // Upstairs
    if (*this).actor.world.pos.y >= 420.0f32 { phi_f20 = 280.0f32 }
    sp5A =
        Actor_WorldYawTowardPoint(&mut (*this).actor,
                                  &mut *sPlatformPositions.as_mut_ptr().offset((*this).nextPlatform
                                                                                   as
                                                                                   isize));
    match (*this).hopAnimIndex {
        0 => {
            (*this).actor.world.rot.y = sp5A;
            (*this).actor.shape.rot.y =
                (sp5A as libc::c_int + 0x8000 as libc::c_int) as s16;
            D_80B4AB30 = 0 as libc::c_int;
            (*this).homePlatform = (*this).curPlatform;
            temp_v1_2 =
                (EnZf_PrimaryFloorCheck(this, globalCtx, 107.0f32) == 0) as
                    libc::c_int;
            temp_v1_2 |=
                ((EnZf_PrimaryFloorCheck(this, globalCtx, 220.0f32) == 0) as
                     libc::c_int) << 1 as libc::c_int;
            (*this).hopAnimIndex += 1;
            match temp_v1_2 {
                1 | 3 => {
                    (*this).actor.velocity.y = 12.0f32;
                    if (*this).actor.bgCheckFlags as libc::c_int &
                           8 as libc::c_int != 0 {
                        (*this).actor.velocity.y += 8.0f32
                    }
                    (*this).actor.speedXZ = 8.0f32
                }
                2 => {
                    (*this).actor.velocity.y = 15.0f32;
                    (*this).actor.speedXZ = 20.0f32
                }
                _ => {
                    // 0
                    phi_f20_2 = 107.0f32;
                    phi_f20_2 += 10.0f32;
                    phi_f0 = 8.0f32;
                    phi_f0 += 1.2f32;
                    phi_v1 = 20 as libc::c_int;
                    while phi_v1 >= 0 as libc::c_int {
                        if EnZf_PrimaryFloorCheck(this, globalCtx, phi_f20_2)
                               == 0 {
                            (*this).actor.speedXZ = phi_f0;
                            (*this).actor.velocity.y = 12.0f32;
                            break ;
                        } else {
                            phi_v1 -= 1;
                            phi_f20_2 += 10.0f32;
                            phi_f0 += 1.2f32
                        }
                    }
                    if (*this).actor.speedXZ == 0.0f32 {
                        EnZf_SetupHopAndTaunt(this);
                    }
                }
            }
        }
        1 => {
            if (*this).actor.bgCheckFlags as libc::c_int & 2 as libc::c_int !=
                   0 ||
                   (*this).actor.bgCheckFlags as libc::c_int &
                       1 as libc::c_int != 0 {
                Audio_PlayActorSound2(&mut (*this).actor,
                                      0x3828 as libc::c_int as u16_0);
                (*this).actor.velocity.y = 0.0f32;
                (*this).actor.world.pos.y = (*this).actor.floorHeight;
                (*this).actor.speedXZ = 0.0f32;
                Actor_SpawnFloorDustRing(globalCtx, &mut (*this).actor,
                                         &mut (*this).leftFootPos, 3.0f32,
                                         2 as libc::c_int, 2.0f32,
                                         0 as libc::c_int as s16,
                                         0 as libc::c_int as s16,
                                         0 as libc::c_int as u8_0);
                Actor_SpawnFloorDustRing(globalCtx, &mut (*this).actor,
                                         &mut (*this).rightFootPos, 3.0f32,
                                         2 as libc::c_int, 2.0f32,
                                         0 as libc::c_int as s16,
                                         0 as libc::c_int as s16,
                                         0 as libc::c_int as u8_0);
                if phi_f20 <= (*this).actor.xzDistToPlayer {
                    EnZf_SetupHopAndTaunt(this);
                } else if sp74 < 80.0f32 {
                    (*this).curPlatform =
                        EnZf_FindPlatform(&mut (*this).actor.world.pos,
                                          (*this).curPlatform);
                    (*this).nextPlatform =
                        EnZf_FindNextPlatformAwayFromPlayer(&mut (*this).actor.world.pos,
                                                            (*this).curPlatform,
                                                            (*this).homePlatform,
                                                            globalCtx)
                }
                (*this).hopAnimIndex = 0 as libc::c_int;
                sp70 = 2.0f32
            } else {
                Math_SmoothStepToS(&mut (*this).actor.world.rot.y, sp5A,
                                   1 as libc::c_int as s16,
                                   0xfa0 as libc::c_int as s16,
                                   0 as libc::c_int as s16);
                (*this).actor.shape.rot.y =
                    ((*this).actor.world.rot.y as libc::c_int +
                         0x8000 as libc::c_int) as s16;
                D_80B4AB30 += 1
            }
        }
        2 => {
            if (*this).skelAnime.curFrame == (*this).skelAnime.endFrame {
                (*this).hopAnimIndex = 0 as libc::c_int
            }
        }
        _ => { }
    }
    if sp54 != (*this).hopAnimIndex {
        Animation_PlayOnceSetSpeed(&mut (*this).skelAnime,
                                   sHoppingAnims[(*this).hopAnimIndex as
                                                     usize], sp70);
    }
    SkelAnime_Update(&mut (*this).skelAnime);
}
#[no_mangle]
pub unsafe extern "C" fn EnZf_SetupDrawSword(mut this: *mut EnZf,
                                             mut globalCtx:
                                                 *mut GlobalContext) {
    Animation_PlayOnce(&mut (*this).skelAnime, &mut gZfDrawingSwordAnim);
    (*this).actor.world.rot.y =
        ((*this).actor.world.rot.y as libc::c_int + 0x8000 as libc::c_int) as
            s16;
    (*this).action = ENZF_ACTION_DRAW_SWORD as libc::c_int;
    (*this).actor.speedXZ = 0.0f32;
    (*this).curPlatform =
        EnZf_FindPlatform(&mut (*this).actor.world.pos, (*this).curPlatform);
    (*this).nextPlatform =
        EnZf_FindNextPlatformAwayFromPlayer(&mut (*this).actor.world.pos,
                                            (*this).curPlatform,
                                            (*this).homePlatform, globalCtx);
    EnZf_SetupAction(this,
                     Some(EnZf_DrawSword as
                              unsafe extern "C" fn(_: *mut EnZf,
                                                   _: *mut GlobalContext)
                                  -> ()));
}
#[no_mangle]
pub unsafe extern "C" fn EnZf_DrawSword(mut this: *mut EnZf,
                                        mut globalCtx: *mut GlobalContext) {
    let mut yawTowardsPlayer: s16 = (*this).actor.yawTowardsPlayer;
    if (*this).skelAnime.curFrame >= 26.0f32 {
        Math_SmoothStepToS(&mut (*this).actor.shape.rot.y, yawTowardsPlayer,
                           1 as libc::c_int as s16,
                           6000 as libc::c_int as s16,
                           0 as libc::c_int as s16);
    }
    if SkelAnime_Update(&mut (*this).skelAnime) != 0 {
        Audio_PlayActorSound2(&mut (*this).actor,
                              0x3829 as libc::c_int as u16_0);
        (*this).actor.world.rot.y = yawTowardsPlayer;
        (*this).hopAnimIndex = -(1 as libc::c_int);
        func_80B45384(this);
    }
    if (*this).skelAnime.curFrame == 22.0f32 {
        (*this).swordSheathed = 0 as libc::c_int as s16
    };
}
#[no_mangle]
pub unsafe extern "C" fn EnZf_SetupDamaged(mut this: *mut EnZf) {
    Animation_Change(&mut (*this).skelAnime, &mut gZfKnockedBackAnim, 1.5f32,
                     0.0f32,
                     Animation_GetLastFrame(&mut gZfKnockedBackAnim as
                                                *mut AnimationHeader as
                                                *mut libc::c_void) as f32_0,
                     ANIMMODE_ONCE as libc::c_int as u8_0, -4.0f32);
    if (*this).actor.bgCheckFlags as libc::c_int & 1 as libc::c_int != 0 &&
           ((*this).actor.velocity.y == 0.0f32 ||
                (*this).actor.velocity.y == -4.0f32) {
        (*this).actor.speedXZ = -4.0f32;
        (*this).hopAnimIndex = 0 as libc::c_int
    } else { (*this).hopAnimIndex = 1 as libc::c_int }
    if (*this).actor.params as libc::c_int ==
           ENZF_TYPE_DINOLFOS as libc::c_int {
        (*this).skelAnime.playSpeed = 4.5f32
    }
    if ((*this).actor.params as libc::c_int) <
           ENZF_TYPE_LIZALFOS_MINIBOSS_A as libc::c_int {
        // not miniboss
        (*this).actor.world.rot.y = (*this).actor.yawTowardsPlayer
    }
    Audio_PlayActorSound2(&mut (*this).actor, 0x382b as libc::c_int as u16_0);
    (*this).action = ENZF_ACTION_DAMAGED as libc::c_int;
    EnZf_SetupAction(this,
                     Some(EnZf_Damaged as
                              unsafe extern "C" fn(_: *mut EnZf,
                                                   _: *mut GlobalContext)
                                  -> ()));
}
#[no_mangle]
pub unsafe extern "C" fn EnZf_Damaged(mut this: *mut EnZf,
                                      mut globalCtx: *mut GlobalContext) {
    let mut wallYawDiff: s16 = 0;
    if (*this).actor.bgCheckFlags as libc::c_int & 2 as libc::c_int != 0 {
        (*this).actor.speedXZ = 0.0f32
    }
    if (*this).actor.bgCheckFlags as libc::c_int & 1 as libc::c_int != 0 {
        if (*this).actor.speedXZ < 0.0f32 { (*this).actor.speedXZ += 0.05f32 }
        (*this).hopAnimIndex = 0 as libc::c_int
    }
    Math_SmoothStepToS(&mut (*this).actor.shape.rot.y,
                       (*this).actor.yawTowardsPlayer,
                       1 as libc::c_int as s16, 4500 as libc::c_int as s16,
                       0 as libc::c_int as s16);
    if ((*this).actor.params as libc::c_int !=
            ENZF_TYPE_DINOLFOS as libc::c_int ||
            EnZf_ChooseAction(globalCtx, this) == 0) &&
           SkelAnime_Update(&mut (*this).skelAnime) != 0 &&
           (*this).actor.bgCheckFlags as libc::c_int & 1 as libc::c_int != 0 {
        if D_80B4A1B4 as libc::c_int != -(1 as libc::c_int) {
            if (*this).damageEffect as libc::c_int ==
                   ENZF_DMGEFF_PROJECTILE as libc::c_int {
                D_80B4A1B0 += 1
            } else {
                (*this).actor.world.rot.y = (*this).actor.shape.rot.y;
                if EnZf_PrimaryFloorCheck(this, globalCtx, 135.0f32) == 0 &&
                       (*this).actor.xzDistToPlayer < 90.0f32 {
                    EnZf_SetupJumpUp(this);
                } else if (*this).actor.xzDistToPlayer <= 100.0f32 &&
                              (*globalCtx).gameplayFrames.wrapping_rem(4 as
                                                                           libc::c_int
                                                                           as
                                                                           libc::c_uint)
                                  == 0 as libc::c_int as libc::c_uint {
                    EnZf_SetupSlash(this);
                } else { func_80B44DC4(this, globalCtx); }
            }
        } else {
            wallYawDiff =
                ((*this).actor.wallYaw as libc::c_int -
                     (*this).actor.shape.rot.y as libc::c_int) as s16;
            wallYawDiff =
                if wallYawDiff as libc::c_int >= 0 as libc::c_int {
                    wallYawDiff as libc::c_int
                } else { -(wallYawDiff as libc::c_int) } as s16;
            if (*this).actor.params as libc::c_int ==
                   ENZF_TYPE_DINOLFOS as libc::c_int &&
                   (*this).actor.bgCheckFlags as libc::c_int &
                       8 as libc::c_int != 0 &&
                   (if wallYawDiff as libc::c_int >= 0 as libc::c_int {
                        wallYawDiff as libc::c_int
                    } else { -(wallYawDiff as libc::c_int) }) <
                       12000 as libc::c_int &&
                   (*this).actor.xzDistToPlayer < 90.0f32 {
                EnZf_SetupJumpUp(this);
            } else if EnZf_DodgeRangedEngaging(globalCtx, this) == 0 {
                if (*this).actor.params as libc::c_int !=
                       ENZF_TYPE_DINOLFOS as libc::c_int {
                    (*this).actor.world.rot.y = (*this).actor.shape.rot.y;
                    if EnZf_PrimaryFloorCheck(this, globalCtx, 135.0f32) == 0
                           && (*this).actor.xzDistToPlayer < 90.0f32 {
                        EnZf_SetupJumpUp(this);
                    } else if (*this).actor.xzDistToPlayer <= 100.0f32 &&
                                  (*globalCtx).gameplayFrames.wrapping_rem(4
                                                                               as
                                                                               libc::c_int
                                                                               as
                                                                               libc::c_uint)
                                      == 0 as libc::c_int as libc::c_uint {
                        EnZf_SetupSlash(this);
                    } else { func_80B44DC4(this, globalCtx); }
                } else if (*this).actor.xzDistToPlayer <= 100.0f32 &&
                              (*globalCtx).gameplayFrames.wrapping_rem(4 as
                                                                           libc::c_int
                                                                           as
                                                                           libc::c_uint)
                                  == 0 as libc::c_int as libc::c_uint &&
                              EnZf_CanAttack(globalCtx, this) != 0 {
                    EnZf_SetupSlash(this);
                } else { func_80B44DC4(this, globalCtx); }
            }
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn EnZf_SetupJumpUp(mut this: *mut EnZf) {
    Animation_Change(&mut (*this).skelAnime, &mut gZfJumpingAnim, 1.0f32,
                     0.0f32, 3.0f32, ANIMMODE_ONCE as libc::c_int as u8_0,
                     0.0f32);
    (*this).unk_3F0 = 0 as libc::c_int;
    (*this).hopAnimIndex = 1 as libc::c_int;
    (*this).action = ENZF_ACTION_JUMP_UP as libc::c_int;
    (*this).actor.velocity.y = 22.0f32;
    (*this).actor.speedXZ = 7.5f32;
    Audio_PlayActorSound2(&mut (*this).actor, 0x382f as libc::c_int as u16_0);
    (*this).actor.world.rot.y = (*this).actor.shape.rot.y;
    EnZf_SetupAction(this,
                     Some(EnZf_JumpUp as
                              unsafe extern "C" fn(_: *mut EnZf,
                                                   _: *mut GlobalContext)
                                  -> ()));
}
#[no_mangle]
pub unsafe extern "C" fn EnZf_JumpUp(mut this: *mut EnZf,
                                     mut globalCtx: *mut GlobalContext) {
    Math_SmoothStepToS(&mut (*this).actor.shape.rot.y,
                       (*this).actor.yawTowardsPlayer,
                       1 as libc::c_int as s16, 4000 as libc::c_int as s16,
                       1 as libc::c_int as s16);
    if (*this).actor.velocity.y >= 5.0f32 {
        func_800355B8(globalCtx, &mut (*this).leftFootPos);
        func_800355B8(globalCtx, &mut (*this).rightFootPos);
    }
    if SkelAnime_Update(&mut (*this).skelAnime) != 0 {
        if (*this).unk_3F0 == 0 as libc::c_int {
            Animation_Change(&mut (*this).skelAnime, &mut gZfSlashAnim,
                             3.0f32, 0.0f32, 13.0f32,
                             ANIMMODE_ONCE as libc::c_int as u8_0, -4.0f32);
            (*this).unk_3F0 = 10 as libc::c_int
        } else if (*this).actor.bgCheckFlags as libc::c_int & 3 as libc::c_int
                      != 0 {
            (*this).actor.velocity.y = 0.0f32;
            (*this).actor.shape.rot.y = (*this).actor.yawTowardsPlayer;
            (*this).actor.world.rot.y = (*this).actor.shape.rot.y;
            (*this).actor.speedXZ = 0.0f32;
            (*this).actor.world.pos.y = (*this).actor.floorHeight;
            EnZf_SetupSlash(this);
            Audio_PlayActorSound2(&mut (*this).actor,
                                  0x382a as libc::c_int as u16_0);
            (*this).skelAnime.curFrame = 13.0f32
        }
    };
}
// Conditional setup function
#[no_mangle]
pub unsafe extern "C" fn func_80B483E4(mut this: *mut EnZf,
                                       mut globalCtx: *mut GlobalContext) {
    let mut playerRotY: s16 = 0;
    let mut player: *mut Player = 0 as *mut Player;
    if ((*this).actor.params as libc::c_int) <
           ENZF_TYPE_LIZALFOS_MINIBOSS_A as libc::c_int ||
           Actor_TestFloorInDirection(&mut (*this).actor, globalCtx, 40.0f32,
                                      ((*this).actor.shape.rot.y as
                                           libc::c_int +
                                           0x3fff as libc::c_int) as s16) as
               libc::c_int != 0 ||
           Actor_TestFloorInDirection(&mut (*this).actor, globalCtx, -40.0f32,
                                      ((*this).actor.shape.rot.y as
                                           libc::c_int +
                                           0x3fff as libc::c_int) as s16) as
               libc::c_int != 0 {
        Animation_PlayLoop(&mut (*this).skelAnime, &mut gZfSidesteppingAnim);
        player =
            (*globalCtx).actorCtx.actorLists[ACTORCAT_PLAYER as libc::c_int as
                                                 usize].head as *mut Player;
        Math_SmoothStepToS(&mut (*this).actor.shape.rot.y,
                           (*this).actor.yawTowardsPlayer,
                           1 as libc::c_int as s16,
                           4000 as libc::c_int as s16,
                           1 as libc::c_int as s16);
        playerRotY = (*player).actor.shape.rot.y;
        if Math_SinS((playerRotY as libc::c_int -
                          (*this).actor.shape.rot.y as libc::c_int) as s16) >=
               0.0f32 {
            (*this).actor.speedXZ = -6.0f32
        } else if Math_SinS((playerRotY as libc::c_int -
                                 (*this).actor.shape.rot.y as libc::c_int) as
                                s16) < 0.0f32 {
            // Superfluous check
            (*this).actor.speedXZ = 6.0f32
        }
        (*this).unk_408 = 0.0f32;
        (*this).hopAnimIndex = 0 as libc::c_int;
        (*this).actor.world.rot.y =
            ((*this).actor.shape.rot.y as libc::c_int + 0x3fff as libc::c_int)
                as s16;
        (*this).unk_3F0 = (Rand_ZeroOne() * 10.0f32 + 5.0f32) as s32;
        (*this).action = ENZF_ACTION_CIRCLE_AROUND_PLAYER as libc::c_int;
        EnZf_SetupAction(this,
                         Some(EnZf_CircleAroundPlayer as
                                  unsafe extern "C" fn(_: *mut EnZf,
                                                       _: *mut GlobalContext)
                                      -> ()));
    } else { EnZf_SetupApproachPlayer(this, globalCtx); };
}
#[no_mangle]
pub unsafe extern "C" fn EnZf_CircleAroundPlayer(mut this: *mut EnZf,
                                                 mut globalCtx:
                                                     *mut GlobalContext) {
    let mut playerRot: s16 = 0;
    let mut phi_v0_4: s16 = 0;
    let mut player: *mut Player =
        (*globalCtx).actorCtx.actorLists[ACTORCAT_PLAYER as libc::c_int as
                                             usize].head as *mut Player;
    let mut curKeyFrame: s32 = 0;
    let mut prevKeyFrame: s32 = 0;
    let mut playSpeed: s32 = 0;
    let mut baseRange: f32_0 = 0.0f32;
    Math_SmoothStepToS(&mut (*this).actor.shape.rot.y,
                       (*this).actor.yawTowardsPlayer,
                       1 as libc::c_int as s16, 0xbb8 as libc::c_int as s16,
                       1 as libc::c_int as s16);
    playerRot = (*player).actor.shape.rot.y;
    if (*this).actor.params as libc::c_int >=
           ENZF_TYPE_LIZALFOS_MINIBOSS_A as libc::c_int {
        // miniboss
        if (*this).unk_3F8 != 0 {
            (*this).actor.speedXZ = -(*this).actor.speedXZ
        }
    } else if (*this).actor.bgCheckFlags as libc::c_int & 8 as libc::c_int !=
                  0 ||
                  Actor_TestFloorInDirection(&mut (*this).actor, globalCtx,
                                             (*this).actor.speedXZ,
                                             ((*this).actor.shape.rot.y as
                                                  libc::c_int +
                                                  0x3fff as libc::c_int) as
                                                 s16) == 0 {
        if (*this).actor.bgCheckFlags as libc::c_int & 8 as libc::c_int != 0 {
            if (*this).actor.speedXZ >= 0.0f32 {
                phi_v0_4 =
                    ((*this).actor.shape.rot.y as libc::c_int +
                         0x3fff as libc::c_int) as s16
            } else {
                phi_v0_4 =
                    ((*this).actor.shape.rot.y as libc::c_int -
                         0x3fff as libc::c_int) as s16
            } // Sword tip?
            phi_v0_4 =
                ((*this).actor.wallYaw as libc::c_int -
                     phi_v0_4 as libc::c_int) as s16
        } else {
            (*this).actor.speedXZ *= -0.8f32; // Sword hilt?
            phi_v0_4 = 0 as libc::c_int as s16
        }
        if (if phi_v0_4 as libc::c_int >= 0 as libc::c_int {
                phi_v0_4 as libc::c_int
            } else { -(phi_v0_4 as libc::c_int) }) > 0x4000 as libc::c_int {
            (*this).actor.speedXZ *= -0.8f32;
            if (*this).actor.speedXZ < 0.0f32 {
                (*this).actor.speedXZ -= 0.5f32
            } else { (*this).actor.speedXZ += 0.5f32 }
        }
    }
    if Math_SinS((playerRot as libc::c_int -
                      (*this).actor.shape.rot.y as libc::c_int) as s16) >=
           0.0f32 {
        (*this).actor.speedXZ += 0.125f32
    } else { (*this).actor.speedXZ -= 0.125f32 }
    (*this).actor.world.rot.y =
        ((*this).actor.shape.rot.y as libc::c_int + 0x4000 as libc::c_int) as
            s16;
    if Actor_OtherIsTargeted(globalCtx, &mut (*this).actor) != 0 {
        baseRange = 100.0f32
    }
    if (*this).actor.xzDistToPlayer <= 70.0f32 + baseRange {
        Math_SmoothStepToF(&mut (*this).unk_408, -4.0f32, 1.0f32, 1.5f32,
                           0.0f32);
    } else if 90.0f32 + baseRange < (*this).actor.xzDistToPlayer {
        Math_SmoothStepToF(&mut (*this).unk_408, 4.0f32, 1.0f32, 1.5f32,
                           0.0f32);
    } else {
        Math_SmoothStepToF(&mut (*this).unk_408, 0.0f32, 1.0f32, 5.65f32,
                           0.0f32);
    }
    if (*this).unk_408 != 0.0f32 &&
           EnZf_SecondaryFloorCheck(this, globalCtx, (*this).unk_408) == 0 {
        (*this).actor.world.pos.x +=
            Math_SinS((*this).actor.shape.rot.y) * (*this).unk_408;
        (*this).actor.world.pos.z +=
            Math_CosS((*this).actor.shape.rot.y) * (*this).unk_408
    }
    if (if (*this).actor.speedXZ >= 0 as libc::c_int as libc::c_float {
            (*this).actor.speedXZ
        } else { -(*this).actor.speedXZ }) >=
           (if (*this).unk_408 >= 0 as libc::c_int as libc::c_float {
                (*this).unk_408
            } else { -(*this).unk_408 }) {
        (*this).skelAnime.playSpeed = -(*this).actor.speedXZ * 0.75f32
    } else if (*this).skelAnime.playSpeed < 0.0f32 {
        (*this).skelAnime.playSpeed = (*this).unk_408 * -0.75f32
    } else { (*this).skelAnime.playSpeed = (*this).unk_408 * 0.75f32 }
    curKeyFrame = (*this).skelAnime.curFrame as s32;
    SkelAnime_Update(&mut (*this).skelAnime);
    prevKeyFrame =
        ((*this).skelAnime.curFrame -
             (if (*this).skelAnime.playSpeed >=
                     0 as libc::c_int as libc::c_float {
                  (*this).skelAnime.playSpeed
              } else { -(*this).skelAnime.playSpeed })) as s32;
    playSpeed =
        if (*this).skelAnime.playSpeed >= 0 as libc::c_int as libc::c_float {
            (*this).skelAnime.playSpeed
        } else { -(*this).skelAnime.playSpeed } as s32;
    (*this).curPlatform =
        EnZf_FindPlatform(&mut (*this).actor.world.pos, (*this).curPlatform);
    if EnZf_FindPlatform(&mut (*player).actor.world.pos,
                         -(1 as libc::c_int) as s16) as libc::c_int !=
           (*this).curPlatform as libc::c_int {
        (*this).actor.speedXZ = 0.0f32;
        if (*this).actor.params as libc::c_int >=
               ENZF_TYPE_LIZALFOS_MINIBOSS_A as libc::c_int &&
               D_80B4A1B4 as libc::c_int ==
                   (*this).actor.params as libc::c_int {
            EnZf_SetupHopAndTaunt(this);
        } else { EnZf_SetupApproachPlayer(this, globalCtx); }
    } else if (*this).actor.params as libc::c_int !=
                  ENZF_TYPE_DINOLFOS as libc::c_int ||
                  EnZf_ChooseAction(globalCtx, this) == 0 {
        if (*this).unk_3F0 == 0 as libc::c_int {
            phi_v0_4 =
                ((*player).actor.shape.rot.y as libc::c_int -
                     (*this).actor.shape.rot.y as libc::c_int) as s16;
            phi_v0_4 =
                if phi_v0_4 as libc::c_int >= 0 as libc::c_int {
                    phi_v0_4 as libc::c_int
                } else { -(phi_v0_4 as libc::c_int) } as s16;
            if phi_v0_4 as libc::c_int >= 0x3a98 as libc::c_int {
                if (*this).actor.params as libc::c_int >=
                       ENZF_TYPE_LIZALFOS_MINIBOSS_A as libc::c_int &&
                       D_80B4A1B4 as libc::c_int ==
                           (*this).actor.params as libc::c_int {
                    EnZf_SetupHopAndTaunt(this);
                } else {
                    func_80B45384(this);
                    (*this).unk_3F0 =
                        (Rand_ZeroOne() * 5.0f32 + 1.0f32) as s32
                }
            } else if (*this).actor.params as libc::c_int >=
                          ENZF_TYPE_LIZALFOS_MINIBOSS_A as libc::c_int &&
                          D_80B4A1B4 as libc::c_int ==
                              (*this).actor.params as libc::c_int {
                EnZf_SetupHopAndTaunt(this);
            } else {
                (*this).actor.world.rot.y = (*this).actor.shape.rot.y;
                if (*this).actor.xzDistToPlayer <= 100.0f32 &&
                       (*globalCtx).gameplayFrames.wrapping_rem(4 as
                                                                    libc::c_int
                                                                    as
                                                                    libc::c_uint)
                           == 0 as libc::c_int as libc::c_uint &&
                       EnZf_CanAttack(globalCtx, this) != 0 {
                    EnZf_SetupSlash(this);
                } else if (*this).actor.xzDistToPlayer < 280.0f32 &&
                              (*this).actor.xzDistToPlayer > 240.0f32 &&
                              EnZf_PrimaryFloorCheck(this, globalCtx,
                                                     191.9956f32) == 0 &&
                              (*globalCtx).gameplayFrames.wrapping_rem(2 as
                                                                           libc::c_int
                                                                           as
                                                                           libc::c_uint)
                                  == 0 as libc::c_int as libc::c_uint {
                    EnZf_SetupJumpForward(this);
                } else { EnZf_SetupApproachPlayer(this, globalCtx); }
            }
        } else { (*this).unk_3F0 -= 1 }
        if curKeyFrame != (*this).skelAnime.curFrame as s32 {
            let mut nextKeyFrame: s32 = playSpeed + curKeyFrame;
            if prevKeyFrame < 14 as libc::c_int &&
                   nextKeyFrame >= 16 as libc::c_int ||
                   prevKeyFrame < 27 as libc::c_int &&
                       nextKeyFrame >= 29 as libc::c_int {
                Audio_PlayActorSound2(&mut (*this).actor,
                                      0x382e as libc::c_int as u16_0);
            }
        }
        if (*globalCtx).gameplayFrames & 0x5f as libc::c_int as libc::c_uint
               == 0 as libc::c_int as libc::c_uint {
            Audio_PlayActorSound2(&mut (*this).actor,
                                  0x3829 as libc::c_int as u16_0);
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn EnZf_SetupDie(mut this: *mut EnZf) {
    Animation_Change(&mut (*this).skelAnime, &mut gZfDyingAnim, 1.5f32,
                     0.0f32,
                     Animation_GetLastFrame(&mut gZfDyingAnim as
                                                *mut AnimationHeader as
                                                *mut libc::c_void) as f32_0,
                     ANIMMODE_ONCE as libc::c_int as u8_0, -4.0f32);
    if (*this).actor.bgCheckFlags as libc::c_int & 1 as libc::c_int != 0 &&
           ((*this).actor.velocity.y == 0.0f32 ||
                (*this).actor.velocity.y == -4.0f32) {
        (*this).actor.speedXZ = 0.0f32;
        (*this).hopAnimIndex = 0 as libc::c_int
    } else { (*this).hopAnimIndex = 1 as libc::c_int }
    (*this).action = ENZF_ACTION_DIE as libc::c_int;
    (*this).actor.flags &=
        !((1 as libc::c_int) << 0 as libc::c_int) as libc::c_uint;
    if D_80B4A1B4 as libc::c_int != -(1 as libc::c_int) {
        if !(*this).actor.prev.is_null() {
            (*((*this).actor.prev as *mut EnZf)).unk_3F4 =
                90 as libc::c_int as s16;
            if ((*(*this).actor.prev).colChkInfo.health as libc::c_int) <
                   3 as libc::c_int {
                (*(*this).actor.prev).colChkInfo.health =
                    3 as libc::c_int as u8_0
            }
        } else {
            (*((*this).actor.next as *mut EnZf)).unk_3F4 =
                90 as libc::c_int as s16;
            if ((*(*this).actor.next).colChkInfo.health as libc::c_int) <
                   3 as libc::c_int {
                (*(*this).actor.next).colChkInfo.health =
                    3 as libc::c_int as u8_0
            }
        }
    }
    D_80B4A1B0 = 0 as libc::c_int as s16;
    Audio_PlayActorSound2(&mut (*this).actor, 0x382d as libc::c_int as u16_0);
    EnZf_SetupAction(this,
                     Some(EnZf_Die as
                              unsafe extern "C" fn(_: *mut EnZf,
                                                   _: *mut GlobalContext)
                                  -> ()));
}
#[no_mangle]
pub unsafe extern "C" fn EnZf_Die(mut this: *mut EnZf,
                                  mut globalCtx: *mut GlobalContext) {
    if (*this).actor.bgCheckFlags as libc::c_int & 2 as libc::c_int != 0 {
        (*this).actor.speedXZ = 0.0f32
    }
    if (*this).actor.bgCheckFlags as libc::c_int & 1 as libc::c_int != 0 {
        Math_SmoothStepToF(&mut (*this).actor.speedXZ, 0.0f32, 1.0f32,
                           0.15f32, 0.0f32);
        (*this).hopAnimIndex = 0 as libc::c_int
    }
    if SkelAnime_Update(&mut (*this).skelAnime) != 0 {
        if (*this).actor.category as libc::c_int !=
               ACTORCAT_PROP as libc::c_int {
            if (*this).actor.params as libc::c_int >=
                   ENZF_TYPE_LIZALFOS_MINIBOSS_A as libc::c_int &&
                   D_80B4A1B4 as libc::c_int == -(1 as libc::c_int) {
                Flags_SetSwitch(globalCtx, (*this).clearFlag as s32);
                func_800F5B58();
            } else { D_80B4A1B4 = -(1 as libc::c_int) as s16 }
            Actor_ChangeCategory(globalCtx, &mut (*globalCtx).actorCtx,
                                 &mut (*this).actor,
                                 ACTORCAT_PROP as libc::c_int as u8_0);
        }
        if (*this).alpha as libc::c_int != 0 as libc::c_int {
            (*this).alpha =
                ((*this).alpha as libc::c_int - 5 as libc::c_int) as u8_0;
            (*this).actor.shape.shadowAlpha = (*this).alpha
        } else { Actor_Kill(&mut (*this).actor); }
    } else {
        let mut curFrame: s32 = (*this).skelAnime.curFrame as s32;
        if curFrame == 10 as libc::c_int || curFrame == 18 as libc::c_int {
            Audio_PlayActorSound2(&mut (*this).actor,
                                  0x387a as libc::c_int as u16_0);
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn EnZf_UpdateHeadRotation(mut this: *mut EnZf,
                                                 mut globalCtx:
                                                     *mut GlobalContext) {
    let mut angleTemp: s16 = 0;
    if (*this).actor.params as libc::c_int ==
           ENZF_TYPE_DINOLFOS as libc::c_int &&
           (*this).action == ENZF_ACTION_3 as libc::c_int &&
           (*this).unk_3F4 as libc::c_int != 0 as libc::c_int {
        (*this).headRot =
            (Math_SinS(((*this).unk_3F4 as libc::c_int * 1400 as libc::c_int)
                           as s16) * 0x2aa8 as libc::c_int as libc::c_float)
                as s16
    } else {
        angleTemp = (*this).actor.yawTowardsPlayer;
        angleTemp =
            (angleTemp as libc::c_int -
                 ((*this).headRot as libc::c_int +
                      (*this).actor.shape.rot.y as libc::c_int) as s16 as
                     libc::c_int) as s16;
        (*this).headRotTemp =
            if (angleTemp as libc::c_int) < -(0x7d0 as libc::c_int) {
                -(0x7d0 as libc::c_int)
            } else if angleTemp as libc::c_int > 0x7d0 as libc::c_int {
                0x7d0 as libc::c_int
            } else { angleTemp as libc::c_int } as s16;
        (*this).headRot =
            ((*this).headRot as libc::c_int +
                 (*this).headRotTemp as libc::c_int) as s16;
        (*this).headRot =
            if ((*this).headRot as libc::c_int) < -(0x1cd7 as libc::c_int) {
                -(0x1cd7 as libc::c_int)
            } else if (*this).headRot as libc::c_int > 0x1cd7 as libc::c_int {
                0x1cd7 as libc::c_int
            } else { (*this).headRot as libc::c_int } as s16
    };
}
#[no_mangle]
pub unsafe extern "C" fn EnZf_UpdateDamage(mut this: *mut EnZf,
                                           mut globalCtx:
                                               *mut GlobalContext) {
    let mut pad: s32 = 0;
    let mut dropParams: s16 = 0;
    if (*this).bodyCollider.base.acFlags as libc::c_int &
           (1 as libc::c_int) << 1 as libc::c_int != 0 &&
           (*this).action <= ENZF_ACTION_STUNNED as libc::c_int {
        (*this).bodyCollider.base.acFlags =
            ((*this).bodyCollider.base.acFlags as libc::c_int &
                 !((1 as libc::c_int) << 1 as libc::c_int)) as u8_0;
        if (((*this).actor.params as libc::c_int) <
                ENZF_TYPE_LIZALFOS_MINIBOSS_A as libc::c_int ||
                D_80B4A1B4 as libc::c_int !=
                    (*this).actor.params as libc::c_int) &&
               (*this).actor.colChkInfo.damageEffect as libc::c_int !=
                   ENZF_DMGEFF_IMMUNE as libc::c_int {
            (*this).damageEffect = (*this).actor.colChkInfo.damageEffect;
            Actor_SetDropFlag(&mut (*this).actor,
                              &mut (*this).bodyCollider.info,
                              0 as libc::c_int);
            if (*this).actor.colChkInfo.damageEffect as libc::c_int ==
                   ENZF_DMGEFF_STUN as libc::c_int ||
                   (*this).actor.colChkInfo.damageEffect as libc::c_int ==
                       ENZF_DMGEFF_ICE as libc::c_int {
                if (*this).action != ENZF_ACTION_STUNNED as libc::c_int {
                    Actor_SetColorFilter(&mut (*this).actor,
                                         0 as libc::c_int as s16,
                                         120 as libc::c_int as s16,
                                         0 as libc::c_int as s16,
                                         80 as libc::c_int as s16);
                    Actor_ApplyDamage(&mut (*this).actor);
                    EnZf_SetupStunned(this);
                }
            } else {
                Audio_PlayActorSound2(&mut (*this).actor,
                                      0x3829 as libc::c_int as u16_0);
                Actor_SetColorFilter(&mut (*this).actor,
                                     0x4000 as libc::c_int as s16,
                                     255 as libc::c_int as s16,
                                     0 as libc::c_int as s16,
                                     8 as libc::c_int as s16);
                if Actor_ApplyDamage(&mut (*this).actor) as libc::c_int ==
                       0 as libc::c_int {
                    dropParams = 0x40 as libc::c_int as s16;
                    EnZf_SetupDie(this);
                    if (*this).actor.params as libc::c_int ==
                           ENZF_TYPE_DINOLFOS as libc::c_int {
                        dropParams = 0xe0 as libc::c_int as s16
                    }
                    Item_DropCollectibleRandom(globalCtx, &mut (*this).actor,
                                               &mut (*this).actor.world.pos,
                                               dropParams);
                    Enemy_StartFinishingBlow(globalCtx, &mut (*this).actor);
                } else {
                    if D_80B4A1B4 as libc::c_int != -(1 as libc::c_int) &&
                           (*this).actor.colChkInfo.health as libc::c_int +
                               (*this).actor.colChkInfo.damage as libc::c_int
                               >= 4 as libc::c_int &&
                           ((*this).actor.colChkInfo.health as libc::c_int) <
                               4 as libc::c_int {
                        (*this).damageEffect =
                            ENZF_DMGEFF_PROJECTILE as libc::c_int as u8_0
                    }
                    EnZf_SetupDamaged(this);
                }
            }
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn EnZf_Update(mut thisx: *mut Actor,
                                     mut globalCtx: *mut GlobalContext) {
    let mut pad: s32 = 0;
    let mut this: *mut EnZf = thisx as *mut EnZf;
    let mut pad2: s32 = 0;
    EnZf_UpdateDamage(this, globalCtx);
    if (*this).actor.colChkInfo.damageEffect as libc::c_int !=
           ENZF_DMGEFF_IMMUNE as libc::c_int {
        (*this).unk_3F8 = 0 as libc::c_int as s16;
        if (*this).hopAnimIndex != 1 as libc::c_int &&
               (*this).action != ENZF_ACTION_HOP_AWAY as libc::c_int {
            if (*this).actor.speedXZ != 0.0f32 {
                (*this).unk_3F8 =
                    EnZf_PrimaryFloorCheck(this, globalCtx,
                                           (*this).actor.speedXZ * 1.5f32) as
                        s16
            }
            if (*this).unk_3F8 == 0 {
                (*this).unk_3F8 =
                    EnZf_PrimaryFloorCheck(this, globalCtx, 0.0f32) as s16
            }
        }
        if (*this).unk_3F8 == 0 { Actor_MoveForward(&mut (*this).actor); }
        Actor_UpdateBgCheckInfo(globalCtx, &mut (*this).actor, 25.0f32,
                                30.0f32, 60.0f32, 0x1d as libc::c_int);
        if (*this).actor.bgCheckFlags as libc::c_int & 1 as libc::c_int == 0 {
            (*this).hopAnimIndex = 1 as libc::c_int
        }
        (*this).actionFunc.expect("non-null function pointer")(this,
                                                               globalCtx);
    }
    if (*this).actor.colChkInfo.health as libc::c_int > 0 as libc::c_int {
        if (*this).action != ENZF_ACTION_SLASH as libc::c_int &&
               (*this).action != ENZF_ACTION_STUNNED as libc::c_int {
            EnZf_UpdateHeadRotation(this, globalCtx);
        }
        if D_80B4A1B0 as libc::c_int != 0 as libc::c_int &&
               D_80B4A1B4 as libc::c_int !=
                   (*this).actor.params as libc::c_int {
            EnZf_SetupSheatheSword(this, globalCtx);
            D_80B4A1B4 = (*this).actor.params;
            D_80B4A1B0 = 0 as libc::c_int as s16;
            if !(*this).actor.prev.is_null() {
                (*((*this).actor.prev as *mut EnZf)).unk_3F4 =
                    90 as libc::c_int as s16
            } else {
                (*((*this).actor.next as *mut EnZf)).unk_3F4 =
                    90 as libc::c_int as s16
            }
        }
    }
    if (*this).action >= ENZF_ACTION_DIE as libc::c_int {
        Math_SmoothStepToS(&mut (*this).headRot, 0 as libc::c_int as s16,
                           1 as libc::c_int as s16,
                           2000 as libc::c_int as s16,
                           0 as libc::c_int as s16);
        if (*this).action <= ENZF_ACTION_HOP_AND_TAUNT as libc::c_int {
            if (*this).unk_3F4 as libc::c_int == 1 as libc::c_int &&
                   (*this).actor.bgCheckFlags as libc::c_int &
                       1 as libc::c_int != 0 {
                if (*this).actor.colChkInfo.health as libc::c_int >
                       0 as libc::c_int {
                    EnZf_SetupDrawSword(this, globalCtx);
                }
                (*this).unk_3F4 -= 1
            }
        }
        if (*this).unk_3F4 as libc::c_int >= 2 as libc::c_int {
            (*this).unk_3F4 -= 1
        }
    }
    (*this).actor.focus.pos = (*this).actor.world.pos;
    (*this).actor.focus.pos.y += 40.0f32;
    if (*this).actor.colChkInfo.health as libc::c_int > 0 as libc::c_int &&
           (*this).alpha as libc::c_int == 255 as libc::c_int {
        Collider_UpdateCylinder(&mut (*this).actor,
                                &mut (*this).bodyCollider);
        if (*this).actor.world.pos.y == (*this).actor.floorHeight &&
               (*this).action <= ENZF_ACTION_DAMAGED as libc::c_int {
            CollisionCheck_SetOC(globalCtx, &mut (*globalCtx).colChkCtx,
                                 &mut (*this).bodyCollider.base);
        }
        if ((*this).actor.params as libc::c_int) <
               ENZF_TYPE_LIZALFOS_MINIBOSS_A as libc::c_int ||
               D_80B4A1B4 as libc::c_int !=
                   (*this).actor.params as libc::c_int {
            if (*this).actor.colorFilterTimer as libc::c_int ==
                   0 as libc::c_int ||
                   (*this).actor.colorFilterParams as libc::c_int &
                       0x4000 as libc::c_int == 0 {
                CollisionCheck_SetAC(globalCtx, &mut (*globalCtx).colChkCtx,
                                     &mut (*this).bodyCollider.base);
            }
        }
    }
    if (*this).action == ENZF_ACTION_SLASH as libc::c_int &&
           (*this).skelAnime.curFrame >= 14.0f32 &&
           (*this).skelAnime.curFrame <= 20.0f32 {
        if (*this).swordCollider.base.atFlags as libc::c_int &
               (1 as libc::c_int) << 2 as libc::c_int == 0 &&
               (*this).swordCollider.base.acFlags as libc::c_int &
                   (1 as libc::c_int) << 1 as libc::c_int == 0 {
            CollisionCheck_SetAT(globalCtx, &mut (*globalCtx).colChkCtx,
                                 &mut (*this).swordCollider.base);
        } else {
            (*this).swordCollider.base.atFlags =
                ((*this).swordCollider.base.atFlags as libc::c_int &
                     !((1 as libc::c_int) << 2 as libc::c_int)) as u8_0;
            (*this).swordCollider.base.acFlags =
                ((*this).swordCollider.base.acFlags as libc::c_int &
                     !((1 as libc::c_int) << 1 as libc::c_int)) as u8_0;
            EnZf_SetupRecoilFromBlockedSlash(this);
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn EnZf_OverrideLimbDraw(mut globalCtx:
                                                   *mut GlobalContext,
                                               mut limbIndex: s32,
                                               mut dList: *mut *mut Gfx,
                                               mut pos: *mut Vec3f,
                                               mut rot: *mut Vec3s,
                                               mut thisx: *mut libc::c_void,
                                               mut gfx: *mut *mut Gfx)
 -> s32 {
    let mut this: *mut EnZf = thisx as *mut EnZf;
    match limbIndex {
        5 => {
            (*rot).y =
                ((*rot).y as libc::c_int - (*this).headRot as libc::c_int) as
                    s16
        }
        15 => {
            if (*this).swordSheathed != 0 {
                *dList = gZfEmptyHandDL.as_mut_ptr()
            }
        }
        33 => {
            if (*this).swordSheathed != 0 {
                *dList = gZfSheathedSwordDL.as_mut_ptr()
            }
        }
        _ => { }
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn EnZf_PostLimbDraw(mut globalCtx: *mut GlobalContext,
                                           mut limbIndex: s32,
                                           mut dList: *mut *mut Gfx,
                                           mut rot: *mut Vec3s,
                                           mut thisx: *mut libc::c_void,
                                           mut gfx: *mut *mut Gfx) {
    static mut sUnused: Vec3f =
        {
            let mut init = Vec3f{x: 1100.0f32, y: -700.0f32, z: 0.0f32,};
            init
        };
    static mut footOffset: Vec3f =
        { let mut init = Vec3f{x: 300.0f32, y: 0.0f32, z: 0.0f32,}; init };
    static mut D_80B4A2A4: Vec3f =
        {
            let mut init = Vec3f{x: 300.0f32, y: -1700.0f32, z: 0.0f32,};
            init
        };
    static mut D_80B4A2B0: Vec3f =
        { let mut init = Vec3f{x: -600.0f32, y: 300.0f32, z: 0.0f32,}; init };
    static mut swordQuadOffset1: Vec3f =
        { let mut init = Vec3f{x: 0.0f32, y: 1500.0f32, z: 0.0f32,}; init };
    static mut swordQuadOffset0: Vec3f =
        {
            let mut init = Vec3f{x: -600.0f32, y: -3000.0f32, z: 1000.0f32,};
            init
        };
    static mut swordQuadOffset3: Vec3f =
        {
            let mut init = Vec3f{x: -600.0f32, y: -3000.0f32, z: -1000.0f32,};
            init
        };
    static mut swordQuadOffset2: Vec3f =
        {
            let mut init = Vec3f{x: 1500.0f32, y: -3000.0f32, z: 0.0f32,};
            init
        };
    static mut zeroVec: Vec3f =
        { let mut init = Vec3f{x: 0.0f32, y: 0.0f32, z: 0.0f32,}; init };
    let mut sp54: Vec3f = Vec3f{x: 0., y: 0., z: 0.,};
    let mut sp48: Vec3f = Vec3f{x: 0., y: 0., z: 0.,};
    let mut this: *mut EnZf = thisx as *mut EnZf;
    let mut bodyPart: s32 = -(1 as libc::c_int);
    if limbIndex == ENZF_LIMB_SWORD as libc::c_int {
        Matrix_MultVec3f(&mut swordQuadOffset1,
                         &mut *(*this).swordCollider.dim.quad.as_mut_ptr().offset(1
                                                                                      as
                                                                                      libc::c_int
                                                                                      as
                                                                                      isize));
        Matrix_MultVec3f(&mut swordQuadOffset0,
                         &mut *(*this).swordCollider.dim.quad.as_mut_ptr().offset(0
                                                                                      as
                                                                                      libc::c_int
                                                                                      as
                                                                                      isize));
        Matrix_MultVec3f(&mut swordQuadOffset3,
                         &mut *(*this).swordCollider.dim.quad.as_mut_ptr().offset(3
                                                                                      as
                                                                                      libc::c_int
                                                                                      as
                                                                                      isize));
        Matrix_MultVec3f(&mut swordQuadOffset2,
                         &mut *(*this).swordCollider.dim.quad.as_mut_ptr().offset(2
                                                                                      as
                                                                                      libc::c_int
                                                                                      as
                                                                                      isize));
        Collider_SetQuadVertices(&mut (*this).swordCollider,
                                 &mut *(*this).swordCollider.dim.quad.as_mut_ptr().offset(0
                                                                                              as
                                                                                              libc::c_int
                                                                                              as
                                                                                              isize),
                                 &mut *(*this).swordCollider.dim.quad.as_mut_ptr().offset(1
                                                                                              as
                                                                                              libc::c_int
                                                                                              as
                                                                                              isize),
                                 &mut *(*this).swordCollider.dim.quad.as_mut_ptr().offset(2
                                                                                              as
                                                                                              libc::c_int
                                                                                              as
                                                                                              isize),
                                 &mut *(*this).swordCollider.dim.quad.as_mut_ptr().offset(3
                                                                                              as
                                                                                              libc::c_int
                                                                                              as
                                                                                              isize));
        Matrix_MultVec3f(&mut D_80B4A2A4, &mut sp54);
        Matrix_MultVec3f(&mut D_80B4A2B0, &mut sp48);
        if (*this).action == ENZF_ACTION_SLASH as libc::c_int {
            if (*this).skelAnime.curFrame < 14.0f32 {
                EffectBlure_AddSpace(Effect_GetByIndex((*this).blureIndex) as
                                         *mut EffectBlure);
            } else if (*this).skelAnime.curFrame < 20.0f32 {
                EffectBlure_AddVertex(Effect_GetByIndex((*this).blureIndex) as
                                          *mut EffectBlure, &mut sp54,
                                      &mut sp48);
            }
        }
    } else {
        Actor_SetFeetPos(&mut (*this).actor, limbIndex,
                         ENZF_LIMB_LEFT_FOOT as libc::c_int, &mut footOffset,
                         ENZF_LIMB_RIGHT_FOOT as libc::c_int,
                         &mut footOffset);
    }
    match limbIndex {
        45 => { Matrix_MultVec3f(&mut footOffset, &mut (*this).leftFootPos); }
        38 => {
            Matrix_MultVec3f(&mut footOffset, &mut (*this).rightFootPos);
        }
        _ => { }
    }
    if (*this).iceTimer as libc::c_int != 0 as libc::c_int {
        match limbIndex {
            9 => { bodyPart = 0 as libc::c_int }
            10 => { bodyPart = 1 as libc::c_int }
            25 => { bodyPart = 2 as libc::c_int }
            16 => { bodyPart = 3 as libc::c_int }
            23 => { bodyPart = 4 as libc::c_int }
            48 => { bodyPart = 5 as libc::c_int }
            14 => { bodyPart = 6 as libc::c_int }
            39 => { bodyPart = 7 as libc::c_int }
            43 => { bodyPart = 8 as libc::c_int }
            _ => { }
        }
        if bodyPart >= 0 as libc::c_int {
            Matrix_MultVec3f(&mut zeroVec,
                             &mut *(*this).bodyPartsPos.as_mut_ptr().offset(bodyPart
                                                                                as
                                                                                isize));
        }
    };
}
static mut D_80B4A2F8: [Gfx; 2] =
    [Gfx{words:
             {
                 let mut init =
                     Gwords{w0:
                                (0xd7 as libc::c_int as u32_0 &
                                     (((0x1 as libc::c_int) <<
                                           8 as libc::c_int) -
                                          1 as libc::c_int) as libc::c_uint)
                                    << 24 as libc::c_int |
                                    (0 as libc::c_int as u32_0 &
                                         (((0x1 as libc::c_int) <<
                                               8 as libc::c_int) -
                                              1 as libc::c_int) as
                                             libc::c_uint) <<
                                        16 as libc::c_int |
                                    (0 as libc::c_int as u32_0 &
                                         (((0x1 as libc::c_int) <<
                                               3 as libc::c_int) -
                                              1 as libc::c_int) as
                                             libc::c_uint) <<
                                        11 as libc::c_int |
                                    (0 as libc::c_int as u32_0 &
                                         (((0x1 as libc::c_int) <<
                                               3 as libc::c_int) -
                                              1 as libc::c_int) as
                                             libc::c_uint) << 8 as libc::c_int
                                    |
                                    (1 as libc::c_int as u32_0 &
                                         (((0x1 as libc::c_int) <<
                                               7 as libc::c_int) -
                                              1 as libc::c_int) as
                                             libc::c_uint) <<
                                        1 as libc::c_int,
                            w1:
                                (0xa00 as libc::c_int as u32_0 &
                                     (((0x1 as libc::c_int) <<
                                           16 as libc::c_int) -
                                          1 as libc::c_int) as libc::c_uint)
                                    << 16 as libc::c_int |
                                    (0xa00 as libc::c_int as u32_0 &
                                         (((0x1 as libc::c_int) <<
                                               16 as libc::c_int) -
                                              1 as libc::c_int) as
                                             libc::c_uint) <<
                                        0 as libc::c_int,};
                 init
             },},
     Gfx{words:
             {
                 let mut init =
                     Gwords{w0:
                                (0xdf as libc::c_int as u32_0 &
                                     (((0x1 as libc::c_int) <<
                                           8 as libc::c_int) -
                                          1 as libc::c_int) as libc::c_uint)
                                    << 24 as libc::c_int,
                            w1: 0 as libc::c_int as libc::c_uint,};
                 init
             },}];
#[no_mangle]
pub unsafe extern "C" fn EnZf_Draw(mut thisx: *mut Actor,
                                   mut globalCtx: *mut GlobalContext) {
    let mut pad: s32 = 0;
    let mut this: *mut EnZf = thisx as *mut EnZf;
    // Extra ";" required for matching. Cannot be if (1) {} or the like. Typo?
    let mut __gfxCtx: *mut GraphicsContext =
        0 as *mut GraphicsContext; // fades out when dead
    let mut dispRefs: [*mut Gfx; 4] =
        [0 as *mut Gfx; 4]; // Set to move sideways
    __gfxCtx = (*globalCtx).state.gfxCtx;
    Graph_OpenDisps(dispRefs.as_mut_ptr(), (*globalCtx).state.gfxCtx,
                    b"../z_en_zf.c\x00" as *const u8 as *const libc::c_char,
                    3533 as libc::c_int);
    func_8002EBCC(thisx, globalCtx, 1 as libc::c_int);
    let mut _g: *mut Gfx = D_80B4A2F8.as_mut_ptr();
    (*_g).words.w0 =
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
    (*_g).words.w1 =
        ((*gGameInfo).data[(9 as libc::c_int * 6 as libc::c_int *
                                16 as libc::c_int + 0 as libc::c_int) as
                               usize] as u32_0 &
             (((0x1 as libc::c_int) << 16 as libc::c_int) - 1 as libc::c_int)
                 as libc::c_uint) << 16 as libc::c_int |
            ((*gGameInfo).data[(9 as libc::c_int * 6 as libc::c_int *
                                    16 as libc::c_int + 1 as libc::c_int) as
                                   usize] as u32_0 &
                 (((0x1 as libc::c_int) << 16 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 0 as libc::c_int;
    let fresh0 = (*__gfxCtx).polyOpa.p;
    (*__gfxCtx).polyOpa.p = (*__gfxCtx).polyOpa.p.offset(1);
    let mut _g_0: *mut Gfx = fresh0;
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
    (*_g_0).words.w1 = D_80B4A2F8.as_mut_ptr() as libc::c_uint;
    if (*this).alpha as libc::c_int == 255 as libc::c_int {
        func_80093D18((*globalCtx).state.gfxCtx);
        let fresh1 = (*__gfxCtx).polyOpa.p;
        (*__gfxCtx).polyOpa.p = (*__gfxCtx).polyOpa.p.offset(1);
        let mut _g_1: *mut Gfx = fresh1;
        (*_g_1).words.w0 =
            (0xfb as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 24 as libc::c_int;
        (*_g_1).words.w1 =
            (0 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 24 as libc::c_int
                |
                (0 as libc::c_int as u32_0 &
                     (((0x1 as libc::c_int) << 8 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    16 as libc::c_int |
                (0 as libc::c_int as u32_0 &
                     (((0x1 as libc::c_int) << 8 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    8 as libc::c_int |
                ((*this).alpha as u32_0 &
                     (((0x1 as libc::c_int) << 8 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    0 as libc::c_int;
        let fresh2 = (*__gfxCtx).polyOpa.p;
        (*__gfxCtx).polyOpa.p = (*__gfxCtx).polyOpa.p.offset(1);
        let mut _g_2: *mut Gfx = fresh2;
        (*_g_2).words.w0 =
            (0xdb as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 24 as libc::c_int
                |
                (0x6 as libc::c_int as u32_0 &
                     (((0x1 as libc::c_int) << 8 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    16 as libc::c_int |
                ((0x9 as libc::c_int * 4 as libc::c_int) as u32_0 &
                     (((0x1 as libc::c_int) << 16 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    0 as libc::c_int;
        (*_g_2).words.w1 =
            &mut *D_80116280.as_mut_ptr().offset(2 as libc::c_int as isize) as
                *mut Gfx as libc::c_uint;
        (*__gfxCtx).polyOpa.p =
            SkelAnime_Draw(globalCtx, (*this).skelAnime.skeleton,
                           (*this).skelAnime.jointTable,
                           Some(EnZf_OverrideLimbDraw as
                                    unsafe extern "C" fn(_:
                                                             *mut GlobalContext,
                                                         _: s32,
                                                         _: *mut *mut Gfx,
                                                         _: *mut Vec3f,
                                                         _: *mut Vec3s,
                                                         _: *mut libc::c_void,
                                                         _: *mut *mut Gfx)
                                        -> s32),
                           Some(EnZf_PostLimbDraw as
                                    unsafe extern "C" fn(_:
                                                             *mut GlobalContext,
                                                         _: s32,
                                                         _: *mut *mut Gfx,
                                                         _: *mut Vec3s,
                                                         _: *mut libc::c_void,
                                                         _: *mut *mut Gfx)
                                        -> ()), this as *mut libc::c_void,
                           (*__gfxCtx).polyOpa.p);
        if (*this).iceTimer as libc::c_int != 0 as libc::c_int {
            (*thisx).colorFilterTimer =
                (*thisx).colorFilterTimer.wrapping_add(1);
            (*this).iceTimer -= 1;
            if (*this).iceTimer as libc::c_int % 4 as libc::c_int ==
                   0 as libc::c_int {
                let mut icePosIndex: s32 =
                    (*this).iceTimer as libc::c_int >> 2 as libc::c_int;
                EffectSsEnIce_SpawnFlyingVec3f(globalCtx, thisx,
                                               &mut *(*this).bodyPartsPos.as_mut_ptr().offset(icePosIndex
                                                                                                  as
                                                                                                  isize),
                                               150 as libc::c_int as s16,
                                               150 as libc::c_int as s16,
                                               150 as libc::c_int as s16,
                                               250 as libc::c_int as s16,
                                               235 as libc::c_int as s16,
                                               245 as libc::c_int as s16,
                                               255 as libc::c_int as s16,
                                               1.4f32);
            }
        }
    } else {
        func_80093D84((*globalCtx).state.gfxCtx);
        let fresh3 = (*__gfxCtx).polyXlu.p;
        (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
        let mut _g_3: *mut Gfx = fresh3;
        (*_g_3).words.w0 =
            (0xe7 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 24 as libc::c_int;
        (*_g_3).words.w1 = 0 as libc::c_int as libc::c_uint;
        let fresh4 = (*__gfxCtx).polyXlu.p;
        (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
        let mut _g_4: *mut Gfx = fresh4;
        (*_g_4).words.w0 =
            (0xfb as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 24 as libc::c_int;
        (*_g_4).words.w1 =
            (0 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 24 as libc::c_int
                |
                (0 as libc::c_int as u32_0 &
                     (((0x1 as libc::c_int) << 8 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    16 as libc::c_int |
                (0 as libc::c_int as u32_0 &
                     (((0x1 as libc::c_int) << 8 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    8 as libc::c_int |
                ((*this).alpha as u32_0 &
                     (((0x1 as libc::c_int) << 8 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    0 as libc::c_int;
        let fresh5 = (*__gfxCtx).polyXlu.p;
        (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
        let mut _g_5: *mut Gfx = fresh5;
        (*_g_5).words.w0 =
            (0xdb as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 24 as libc::c_int
                |
                (0x6 as libc::c_int as u32_0 &
                     (((0x1 as libc::c_int) << 8 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    16 as libc::c_int |
                ((0x9 as libc::c_int * 4 as libc::c_int) as u32_0 &
                     (((0x1 as libc::c_int) << 16 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    0 as libc::c_int;
        (*_g_5).words.w1 =
            &mut *D_80116280.as_mut_ptr().offset(0 as libc::c_int as isize) as
                *mut Gfx as libc::c_uint;
        (*__gfxCtx).polyXlu.p =
            SkelAnime_Draw(globalCtx, (*this).skelAnime.skeleton,
                           (*this).skelAnime.jointTable,
                           Some(EnZf_OverrideLimbDraw as
                                    unsafe extern "C" fn(_:
                                                             *mut GlobalContext,
                                                         _: s32,
                                                         _: *mut *mut Gfx,
                                                         _: *mut Vec3f,
                                                         _: *mut Vec3s,
                                                         _: *mut libc::c_void,
                                                         _: *mut *mut Gfx)
                                        -> s32),
                           Some(EnZf_PostLimbDraw as
                                    unsafe extern "C" fn(_:
                                                             *mut GlobalContext,
                                                         _: s32,
                                                         _: *mut *mut Gfx,
                                                         _: *mut Vec3s,
                                                         _: *mut libc::c_void,
                                                         _: *mut *mut Gfx)
                                        -> ()), this as *mut libc::c_void,
                           (*__gfxCtx).polyXlu.p)
    }
    Graph_CloseDisps(dispRefs.as_mut_ptr(), (*globalCtx).state.gfxCtx,
                     b"../z_en_zf.c\x00" as *const u8 as *const libc::c_char,
                     3601 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn EnZf_SetupCircleAroundPlayer(mut this: *mut EnZf,
                                                      mut speed: f32_0) {
    Animation_MorphToLoop(&mut (*this).skelAnime, &mut gZfSidesteppingAnim,
                          -1.0f32);
    (*this).unk_3F0 = (Rand_ZeroOne() * 10.0f32 + 8.0f32) as s32;
    if (*this).actor.params as libc::c_int ==
           ENZF_TYPE_DINOLFOS as libc::c_int {
        (*this).actor.speedXZ = 2.0f32 * speed;
        (*this).unk_3F0 /= 2 as libc::c_int
    } else { (*this).actor.speedXZ = speed }
    (*this).hopAnimIndex = 0 as libc::c_int;
    (*this).actor.world.rot.y =
        ((*this).actor.shape.rot.y as libc::c_int + 0x3fff as libc::c_int) as
            s16;
    (*this).action = ENZF_ACTION_CIRCLE_AROUND_PLAYER as libc::c_int;
    EnZf_SetupAction(this,
                     Some(EnZf_CircleAroundPlayer as
                              unsafe extern "C" fn(_: *mut EnZf,
                                                   _: *mut GlobalContext)
                                  -> ()));
}
#[no_mangle]
pub unsafe extern "C" fn EnZf_DodgeRangedEngaging(mut globalCtx:
                                                      *mut GlobalContext,
                                                  mut this: *mut EnZf)
 -> s32 {
    let mut projectileActor: *mut Actor = 0 as *mut Actor;
    let mut yawToProjectile: s16 = 0;
    let mut phi_t0: s16 = 0;
    let mut phi_v1: s16 = 0;
    projectileActor =
        Actor_GetProjectileActor(globalCtx, &mut (*this).actor, 600.0f32);
    if !projectileActor.is_null() {
        yawToProjectile =
            (Actor_WorldYawTowardActor(&mut (*this).actor, projectileActor) as
                 libc::c_int -
                 (*this).actor.shape.rot.y as u16_0 as s16 as libc::c_int) as
                s16;
        (*this).actor.world.rot.y =
            ((*this).actor.shape.rot.y as libc::c_int + 0x3fff as libc::c_int)
                as s16;
        phi_t0 = 0 as libc::c_int as s16;
        if EnZf_PrimaryFloorCheck(this, globalCtx, -8.0f32) != 0 {
            phi_t0 = 1 as libc::c_int as s16
        }
        if EnZf_PrimaryFloorCheck(this, globalCtx, 8.0f32) != 0 {
            phi_t0 = (phi_t0 as libc::c_int | 2 as libc::c_int) as s16
        }
        (*this).actor.world.rot.y = (*this).actor.shape.rot.y;
        if ((*this).actor.xzDistToPlayer < 90.0f32 ||
                phi_t0 as libc::c_int == 3 as libc::c_int) &&
               EnZf_PrimaryFloorCheck(this, globalCtx, 135.0f32) == 0 ||
               (*projectileActor).id as libc::c_int ==
                   ACTOR_ARMS_HOOK as libc::c_int {
            EnZf_SetupJumpUp(this);
            return 1 as libc::c_int
        }
        (*this).actor.world.rot.y =
            ((*this).actor.shape.rot.y as libc::c_int + 0x3fff as libc::c_int)
                as s16;
        if phi_t0 as libc::c_int == 0 as libc::c_int {
            phi_v1 =
                (*globalCtx).gameplayFrames.wrapping_rem(2 as libc::c_int as
                                                             libc::c_uint) as
                    s16
        } else { phi_v1 = phi_t0 }
        if (if yawToProjectile as libc::c_int >= 0 as libc::c_int {
                yawToProjectile as libc::c_int
            } else { -(yawToProjectile as libc::c_int) }) <
               0x2000 as libc::c_int ||
               (if yawToProjectile as libc::c_int >= 0 as libc::c_int {
                    yawToProjectile as libc::c_int
                } else { -(yawToProjectile as libc::c_int) }) >=
                   0x6000 as libc::c_int {
            if phi_v1 as libc::c_int & 1 as libc::c_int != 0 {
                EnZf_SetupCircleAroundPlayer(this, 8.0f32);
                return 1 as libc::c_int
            }
            EnZf_SetupCircleAroundPlayer(this, -8.0f32);
            return 1 as libc::c_int
        }
        if (if yawToProjectile as libc::c_int >= 0 as libc::c_int {
                yawToProjectile as libc::c_int
            } else { -(yawToProjectile as libc::c_int) }) <
               0x5fff as libc::c_int {
            if phi_v1 as libc::c_int & 1 as libc::c_int != 0 {
                EnZf_SetupCircleAroundPlayer(this, 4.0f32);
                return 1 as libc::c_int
            }
            EnZf_SetupCircleAroundPlayer(this, -4.0f32);
        }
        return 1 as libc::c_int
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn EnZf_DodgeRangedWaiting(mut globalCtx:
                                                     *mut GlobalContext,
                                                 mut this: *mut EnZf) -> s32 {
    let mut projectileActor: *mut Actor = 0 as *mut Actor;
    let mut yawToProjectile: s16 = 0;
    let mut phi_t0: s16 = 0;
    let mut sp1E: s16 = 0;
    let mut sp1C: s16 = 0 as libc::c_int as s16;
    projectileActor =
        Actor_GetProjectileActor(globalCtx, &mut (*this).actor, 600.0f32);
    if !projectileActor.is_null() {
        yawToProjectile =
            (Actor_WorldYawTowardActor(&mut (*this).actor, projectileActor) as
                 libc::c_int -
                 (*this).actor.shape.rot.y as u16_0 as s16 as libc::c_int) as
                s16;
        (*this).actor.world.rot.y =
            ((*this).actor.shape.rot.y as libc::c_int + 0x3fff as libc::c_int)
                as s16;
        phi_t0 = 0 as libc::c_int as s16;
        if EnZf_PrimaryFloorCheck(this, globalCtx, -70.0f32) != 0 {
            phi_t0 = 1 as libc::c_int as s16
        }
        if EnZf_PrimaryFloorCheck(this, globalCtx, 70.0f32) != 0 {
            phi_t0 = (phi_t0 as libc::c_int | 2 as libc::c_int) as s16
        }
        (*this).actor.speedXZ = 0.0f32;
        if (if yawToProjectile as libc::c_int >= 0 as libc::c_int {
                yawToProjectile as libc::c_int
            } else { -(yawToProjectile as libc::c_int) }) <
               0x2000 as libc::c_int ||
               (if yawToProjectile as libc::c_int >= 0 as libc::c_int {
                    yawToProjectile as libc::c_int
                } else { -(yawToProjectile as libc::c_int) }) >=
                   0x6000 as libc::c_int {
            if phi_t0 as libc::c_int == 0 as libc::c_int {
                if (*globalCtx).gameplayFrames.wrapping_rem(2 as libc::c_int
                                                                as
                                                                libc::c_uint)
                       != 0 as libc::c_int as libc::c_uint {
                    sp1E = 6 as libc::c_int as s16
                } else { sp1E = -(6 as libc::c_int) as s16 }
            } else {
                match phi_t0 as libc::c_int {
                    1 => { sp1E = 6 as libc::c_int as s16 }
                    2 => { sp1E = -(6 as libc::c_int) as s16 }
                    3 => {
                        sp1C = 5 as libc::c_int as s16;
                        sp1E = 0 as libc::c_int as s16
                    }
                    _ => { }
                }
            }
        } else if (if yawToProjectile as libc::c_int >= 0 as libc::c_int {
                       yawToProjectile as libc::c_int
                   } else { -(yawToProjectile as libc::c_int) }) <
                      0x5fff as libc::c_int {
            if phi_t0 as libc::c_int == 0 as libc::c_int {
                if (*globalCtx).gameplayFrames.wrapping_rem(2 as libc::c_int
                                                                as
                                                                libc::c_uint)
                       != 0 as libc::c_int as libc::c_uint {
                    sp1E = 6 as libc::c_int as s16
                } else { sp1E = -(6 as libc::c_int) as s16 }
            } else {
                match phi_t0 as libc::c_int {
                    1 => { sp1E = 6 as libc::c_int as s16 }
                    2 => { sp1E = -(6 as libc::c_int) as s16 }
                    3 => {
                        sp1C = 10 as libc::c_int as s16;
                        sp1E = 0 as libc::c_int as s16
                    }
                    _ => { }
                }
            }
        }
        (*this).unk_408 = sp1E as f32_0;
        (*this).unk_40C = sp1C as f32_0;
        return 1 as libc::c_int
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn run_static_initializers() {
    sInitChain =
        [{
             let mut init = InitChainEntry{cont_type_0_offset_value: [0; 4],};
             init.set_cont(1 as libc::c_int as u32_0);
             init.set_type_0(ICHAINTYPE_F32 as libc::c_int as u32_0);
             init.set_offset(&mut (*(0 as *mut Actor)).targetArrowOffset as
                                 *mut f32_0 as size_t as u32_0);
             init.set_value(2000 as libc::c_int);
             init
         },
         {
             let mut init = InitChainEntry{cont_type_0_offset_value: [0; 4],};
             init.set_cont(1 as libc::c_int as u32_0);
             init.set_type_0(ICHAINTYPE_VEC3F_DIV1000 as libc::c_int as
                                 u32_0);
             init.set_offset(&mut (*(0 as *mut Actor)).scale as *mut Vec3f as
                                 size_t as u32_0);
             init.set_value(15 as libc::c_int);
             init
         },
         {
             let mut init = InitChainEntry{cont_type_0_offset_value: [0; 4],};
             init.set_cont(0 as libc::c_int as u32_0);
             init.set_type_0(ICHAINTYPE_F32_DIV1000 as libc::c_int as u32_0);
             init.set_offset(&mut (*(0 as *mut Actor)).gravity as *mut f32_0
                                 as size_t as u32_0);
             init.set_value(-(3500 as libc::c_int));
             init
         }]
}
#[used]
#[cfg_attr(target_os = "linux", link_section = ".init_array")]
#[cfg_attr(target_os = "windows", link_section = ".CRT$XIB")]
#[cfg_attr(target_os = "macos", link_section = "__DATA,__mod_init_func")]
static INIT_ARRAY: [unsafe extern "C" fn(); 1] = [run_static_initializers];
