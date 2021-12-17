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
    fn EffectSsEnIce_SpawnFlyingVec3s(globalCtx: *mut GlobalContext,
                                      actor: *mut Actor, pos: *mut Vec3s,
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
    fn Actor_Kill(actor: *mut Actor);
    #[no_mangle]
    fn Actor_SetScale(actor: *mut Actor, scale: f32_0);
    #[no_mangle]
    fn Actor_MoveForward(actor: *mut Actor);
    #[no_mangle]
    fn Actor_WorldYawTowardActor(actorA: *mut Actor, actorB: *mut Actor)
     -> s16;
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
    fn BodyBreak_Alloc(bodyBreak: *mut BodyBreak, count: s32,
                       globalCtx: *mut GlobalContext);
    #[no_mangle]
    fn BodyBreak_SetInfo(bodyBreak: *mut BodyBreak, limbIndex: s32,
                         minLimbIndex: s32, maxLimbIndex: s32, count: u32_0,
                         dList: *mut *mut Gfx, objectId: s16);
    #[no_mangle]
    fn BodyBreak_SpawnParts(actor: *mut Actor, bodyBreak: *mut BodyBreak,
                            globalCtx: *mut GlobalContext, type_0: s16)
     -> s32;
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
    fn Actor_SetColorFilter(actor: *mut Actor, colorFlag: s16,
                            colorIntensityMax: s16, xluFlag: s16,
                            duration: s16);
    #[no_mangle]
    fn Actor_FindNearby(globalCtx: *mut GlobalContext, refActor: *mut Actor,
                        actorId: s16, actorCategory: u8_0, range: f32_0)
     -> *mut Actor;
    #[no_mangle]
    fn Actor_ApplyDamage(actor: *mut Actor) -> u8_0;
    #[no_mangle]
    fn Actor_SetDropFlag(actor: *mut Actor, colBody: *mut ColliderInfo,
                         freezeFlag: s32);
    #[no_mangle]
    fn func_80041D4C(colCtx: *mut CollisionContext, poly: *mut CollisionPoly,
                     bgId: s32) -> u32_0;
    #[no_mangle]
    fn func_80041EA4(colCtx: *mut CollisionContext, poly: *mut CollisionPoly,
                     bgId: s32) -> u32_0;
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
    fn SkelAnime_DrawOpa(globalCtx: *mut GlobalContext,
                         skeleton: *mut *mut libc::c_void,
                         jointTable: *mut Vec3s,
                         overrideLimbDraw: OverrideLimbDrawOpa,
                         postLimbDraw: PostLimbDrawOpa,
                         arg: *mut libc::c_void);
    #[no_mangle]
    fn Animation_GetLastFrame(animation: *mut libc::c_void) -> s16;
    #[no_mangle]
    fn SkelAnime_InterpFrameTable(limbCount: s32, dst: *mut Vec3s,
                                  start: *mut Vec3s, target: *mut Vec3s,
                                  weight: f32_0);
    #[no_mangle]
    fn AnimationContext_SetCopyTrue(globalCtx: *mut GlobalContext,
                                    vecCount: s32, dst: *mut Vec3s,
                                    src: *mut Vec3s, copyFlag: *mut u8_0);
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
    fn SkelAnime_CopyFrameTableTrue(skelAnime: *mut SkelAnime,
                                    dst: *mut Vec3s, src: *mut Vec3s,
                                    copyFlag: *mut u8_0);
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
    fn Audio_StopSfxByPosAndId(pos: *mut Vec3f, sfxId: u16_0);
    #[no_mangle]
    fn Rand_ZeroOne() -> f32_0;
    #[no_mangle]
    static mut gStalfosFlinchFromHitBehindAnim: AnimationHeader;
    #[no_mangle]
    static mut gStalfosFallOverBackwardsAnim: AnimationHeader;
    #[no_mangle]
    static mut gStalfosJumpBackwardsAnim: AnimationHeader;
    #[no_mangle]
    static mut gStalfosBlockWithShieldAnim: AnimationHeader;
    #[no_mangle]
    static mut gStalfosFastAdvanceAnim: AnimationHeader;
    #[no_mangle]
    static mut gStalfosMiddleGuardAnim: AnimationHeader;
    #[no_mangle]
    static mut gStalfosSkel: SkeletonHeader;
    #[no_mangle]
    static mut gStalfosSlowAdvanceAnim: AnimationHeader;
    #[no_mangle]
    static mut gStalfosFlinchFromHitFrontAnim: AnimationHeader;
    #[no_mangle]
    static mut gStalfosFallOverForwardsAnim: AnimationHeader;
    #[no_mangle]
    static mut gStalfosJumpAnim: AnimationHeader;
    #[no_mangle]
    static mut gStalfosJumpslashAnim: AnimationHeader;
    #[no_mangle]
    static mut gStalfosDownSlashAnim: AnimationHeader;
    #[no_mangle]
    static mut gStalfosRecoverFromDownSlashAnim: AnimationHeader;
    #[no_mangle]
    static mut gStalfosUpSlashAnim: AnimationHeader;
    #[no_mangle]
    static mut gStalfosLandFromLeapAnim: AnimationHeader;
    #[no_mangle]
    static mut gStalfosSidestepAnim: AnimationHeader;
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
pub struct BodyBreak {
    pub matrices: *mut MtxF,
    pub objectIds: *mut s16,
    pub count: s16,
    pub dLists: *mut *mut Gfx,
    pub val: s32,
    pub prevLimbIndex: s32,
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
pub type C2RustUnnamed_18 = libc::c_uint;
pub const ACTOR_ID_MAX: C2RustUnnamed_18 = 471;
pub const ACTOR_OBJ_WARP2BLOCK: C2RustUnnamed_18 = 470;
pub const ACTOR_BG_JYA_BLOCK: C2RustUnnamed_18 = 469;
pub const ACTOR_EN_MM2: C2RustUnnamed_18 = 468;
pub const ACTOR_EN_ZL4: C2RustUnnamed_18 = 467;
pub const ACTOR_OBJ_HAMISHI: C2RustUnnamed_18 = 466;
pub const ACTOR_OBJ_TIMEBLOCK: C2RustUnnamed_18 = 465;
pub const ACTOR_EN_GE3: C2RustUnnamed_18 = 464;
pub const ACTOR_OBJ_MAKEKINSUTA: C2RustUnnamed_18 = 463;
pub const ACTOR_EN_ZO: C2RustUnnamed_18 = 462;
pub const ACTOR_BG_MENKURI_NISEKABE: C2RustUnnamed_18 = 461;
pub const ACTOR_EN_EG: C2RustUnnamed_18 = 460;
pub const ACTOR_OCEFF_WIPE4: C2RustUnnamed_18 = 459;
pub const ACTOR_EN_KAKASI3: C2RustUnnamed_18 = 458;
pub const ACTOR_EN_KAKASI2: C2RustUnnamed_18 = 457;
pub const ACTOR_BG_ICE_SHUTTER: C2RustUnnamed_18 = 456;
pub const ACTOR_BG_ICE_TURARA: C2RustUnnamed_18 = 455;
pub const ACTOR_EN_COW: C2RustUnnamed_18 = 454;
pub const ACTOR_EN_MA3: C2RustUnnamed_18 = 453;
pub const ACTOR_BG_SPOT18_SHUTTER: C2RustUnnamed_18 = 452;
pub const ACTOR_BG_SPOT18_FUTA: C2RustUnnamed_18 = 451;
pub const ACTOR_BG_SPOT11_OASIS: C2RustUnnamed_18 = 450;
pub const ACTOR_DOOR_KILLER: C2RustUnnamed_18 = 449;
pub const ACTOR_EN_CROW: C2RustUnnamed_18 = 448;
pub const ACTOR_EN_PO_DESERT: C2RustUnnamed_18 = 447;
pub const ACTOR_EN_WALL_TUBO: C2RustUnnamed_18 = 446;
pub const ACTOR_BG_BOWL_WALL: C2RustUnnamed_18 = 445;
pub const ACTOR_EN_DAIKU_KAKARIKO: C2RustUnnamed_18 = 444;
pub const ACTOR_BG_MIZU_SHUTTER: C2RustUnnamed_18 = 443;
pub const ACTOR_BG_MIZU_BWALL: C2RustUnnamed_18 = 442;
pub const ACTOR_EN_GS: C2RustUnnamed_18 = 441;
pub const ACTOR_EN_GB: C2RustUnnamed_18 = 440;
pub const ACTOR_BG_GND_ICEBLOCK: C2RustUnnamed_18 = 439;
pub const ACTOR_BG_GND_NISEKABE: C2RustUnnamed_18 = 438;
pub const ACTOR_BG_GND_SOULMEIRO: C2RustUnnamed_18 = 437;
pub const ACTOR_BG_GND_DARKMEIRO: C2RustUnnamed_18 = 436;
pub const ACTOR_BG_GND_FIREMEIRO: C2RustUnnamed_18 = 435;
pub const ACTOR_DEMO_GEFF: C2RustUnnamed_18 = 434;
pub const ACTOR_DEMO_GJ: C2RustUnnamed_18 = 433;
pub const ACTOR_EN_SKB: C2RustUnnamed_18 = 432;
pub const ACTOR_EN_WF: C2RustUnnamed_18 = 431;
pub const ACTOR_EN_GO2: C2RustUnnamed_18 = 430;
pub const ACTOR_EN_MU: C2RustUnnamed_18 = 429;
pub const ACTOR_EN_TG: C2RustUnnamed_18 = 428;
pub const ACTOR_OBJ_MURE3: C2RustUnnamed_18 = 427;
pub const ACTOR_UNSET_1AA: C2RustUnnamed_18 = 426;
pub const ACTOR_BG_SPOT17_BAKUDANKABE: C2RustUnnamed_18 = 425;
pub const ACTOR_BG_SPOT08_BAKUDANKABE: C2RustUnnamed_18 = 424;
pub const ACTOR_DEMO_KEKKAI: C2RustUnnamed_18 = 423;
pub const ACTOR_EN_HS2: C2RustUnnamed_18 = 422;
pub const ACTOR_BG_BOM_GUARD: C2RustUnnamed_18 = 421;
pub const ACTOR_EN_GUEST: C2RustUnnamed_18 = 420;
pub const ACTOR_EN_DNT_NOMAL: C2RustUnnamed_18 = 419;
pub const ACTOR_EN_DNT_JIJI: C2RustUnnamed_18 = 418;
pub const ACTOR_EN_DNT_DEMO: C2RustUnnamed_18 = 417;
pub const ACTOR_OBJ_KIBAKO2: C2RustUnnamed_18 = 416;
pub const ACTOR_BG_SPOT11_BAKUDANKABE: C2RustUnnamed_18 = 415;
pub const ACTOR_OBJ_COMB: C2RustUnnamed_18 = 414;
pub const ACTOR_BG_SPOT01_OBJECTS2: C2RustUnnamed_18 = 413;
pub const ACTOR_EN_SI: C2RustUnnamed_18 = 412;
pub const ACTOR_EN_DOG: C2RustUnnamed_18 = 411;
pub const ACTOR_EN_NIW_GIRL: C2RustUnnamed_18 = 410;
pub const ACTOR_OCEFF_WIPE3: C2RustUnnamed_18 = 409;
pub const ACTOR_OCEFF_WIPE2: C2RustUnnamed_18 = 408;
pub const ACTOR_EN_GELDB: C2RustUnnamed_18 = 407;
pub const ACTOR_EN_IT: C2RustUnnamed_18 = 406;
pub const ACTOR_EN_SHOPNUTS: C2RustUnnamed_18 = 405;
pub const ACTOR_BG_SPOT00_BREAK: C2RustUnnamed_18 = 404;
pub const ACTOR_EN_NUTSBALL: C2RustUnnamed_18 = 403;
pub const ACTOR_EN_HINTNUTS: C2RustUnnamed_18 = 402;
pub const ACTOR_BG_SPOT12_SAKU: C2RustUnnamed_18 = 401;
pub const ACTOR_BG_SPOT12_GATE: C2RustUnnamed_18 = 400;
pub const ACTOR_BG_JYA_HAHENIRON: C2RustUnnamed_18 = 399;
pub const ACTOR_BG_JYA_1FLIFT: C2RustUnnamed_18 = 398;
pub const ACTOR_BG_SPOT05_SOKO: C2RustUnnamed_18 = 397;
pub const ACTOR_EN_WEIYER: C2RustUnnamed_18 = 396;
pub const ACTOR_OCEFF_STORM: C2RustUnnamed_18 = 395;
pub const ACTOR_OCEFF_WIPE: C2RustUnnamed_18 = 394;
pub const ACTOR_EN_STH: C2RustUnnamed_18 = 393;
pub const ACTOR_EN_SSH: C2RustUnnamed_18 = 392;
pub const ACTOR_OBJ_ROOMTIMER: C2RustUnnamed_18 = 391;
pub const ACTOR_EN_GE2: C2RustUnnamed_18 = 390;
pub const ACTOR_EN_WONDER_TALK2: C2RustUnnamed_18 = 389;
pub const ACTOR_EN_DY_EXTRA: C2RustUnnamed_18 = 388;
pub const ACTOR_SHOT_SUN: C2RustUnnamed_18 = 387;
pub const ACTOR_DEMO_EC: C2RustUnnamed_18 = 386;
pub const ACTOR_EN_TORCH: C2RustUnnamed_18 = 385;
pub const ACTOR_UNSET_180: C2RustUnnamed_18 = 384;
pub const ACTOR_END_TITLE: C2RustUnnamed_18 = 383;
pub const ACTOR_OCEFF_SPOT: C2RustUnnamed_18 = 382;
pub const ACTOR_OBJ_MAKEOSHIHIKI: C2RustUnnamed_18 = 381;
pub const ACTOR_EN_TAKARA_MAN: C2RustUnnamed_18 = 380;
pub const ACTOR_EN_KAKASI: C2RustUnnamed_18 = 379;
pub const ACTOR_BOSS_GANON2: C2RustUnnamed_18 = 378;
pub const ACTOR_EN_ZL3: C2RustUnnamed_18 = 377;
pub const ACTOR_EN_HEISHI4: C2RustUnnamed_18 = 376;
pub const ACTOR_BG_ZG: C2RustUnnamed_18 = 375;
pub const ACTOR_EFC_ERUPC: C2RustUnnamed_18 = 374;
pub const ACTOR_EN_PO_FIELD: C2RustUnnamed_18 = 373;
pub const ACTOR_DEMO_GT: C2RustUnnamed_18 = 372;
pub const ACTOR_ELF_MSG2: C2RustUnnamed_18 = 371;
pub const ACTOR_DOOR_GERUDO: C2RustUnnamed_18 = 370;
pub const ACTOR_EN_MAG: C2RustUnnamed_18 = 369;
pub const ACTOR_EN_OKARINA_EFFECT: C2RustUnnamed_18 = 368;
pub const ACTOR_EN_GANON_MANT: C2RustUnnamed_18 = 367;
pub const ACTOR_EN_HY: C2RustUnnamed_18 = 366;
pub const ACTOR_EN_MD: C2RustUnnamed_18 = 365;
pub const ACTOR_EN_CS: C2RustUnnamed_18 = 364;
pub const ACTOR_EN_JSJUTAN: C2RustUnnamed_18 = 363;
pub const ACTOR_EN_JS: C2RustUnnamed_18 = 362;
pub const ACTOR_BG_JYA_IRONOBJ: C2RustUnnamed_18 = 361;
pub const ACTOR_EN_EX_ITEM: C2RustUnnamed_18 = 360;
pub const ACTOR_EN_ANI: C2RustUnnamed_18 = 359;
pub const ACTOR_BG_SST_FLOOR: C2RustUnnamed_18 = 358;
pub const ACTOR_EN_WEATHER_TAG: C2RustUnnamed_18 = 357;
pub const ACTOR_EN_KZ: C2RustUnnamed_18 = 356;
pub const ACTOR_EN_KO: C2RustUnnamed_18 = 355;
pub const ACTOR_EN_MM: C2RustUnnamed_18 = 354;
pub const ACTOR_UNSET_161: C2RustUnnamed_18 = 353;
pub const ACTOR_EN_STREAM: C2RustUnnamed_18 = 352;
pub const ACTOR_EN_SIOFUKI: C2RustUnnamed_18 = 351;
pub const ACTOR_EN_GANON_ORGAN: C2RustUnnamed_18 = 350;
pub const ACTOR_UNSET_15D: C2RustUnnamed_18 = 349;
pub const ACTOR_BG_SPOT18_BASKET: C2RustUnnamed_18 = 348;
pub const ACTOR_BG_JYA_BOMBIWA: C2RustUnnamed_18 = 347;
pub const ACTOR_BG_JYA_AMISHUTTER: C2RustUnnamed_18 = 346;
pub const ACTOR_BG_JYA_BOMBCHUIWA: C2RustUnnamed_18 = 345;
pub const ACTOR_BG_JYA_BIGMIRROR: C2RustUnnamed_18 = 344;
pub const ACTOR_BG_JYA_LIFT: C2RustUnnamed_18 = 343;
pub const ACTOR_BG_JYA_MEGAMI: C2RustUnnamed_18 = 342;
pub const ACTOR_EN_CHANGER: C2RustUnnamed_18 = 341;
pub const ACTOR_UNSET_154: C2RustUnnamed_18 = 340;
pub const ACTOR_EN_FU: C2RustUnnamed_18 = 339;
pub const ACTOR_EN_GO: C2RustUnnamed_18 = 338;
pub const ACTOR_OBJ_MURE2: C2RustUnnamed_18 = 337;
pub const ACTOR_OBJ_LIGHTSWITCH: C2RustUnnamed_18 = 336;
pub const ACTOR_OBJ_HANA: C2RustUnnamed_18 = 335;
pub const ACTOR_EN_ISHI: C2RustUnnamed_18 = 334;
pub const ACTOR_EN_OWL: C2RustUnnamed_18 = 333;
pub const ACTOR_EN_BOM_BOWL_PIT: C2RustUnnamed_18 = 332;
pub const ACTOR_EN_BOM_BOWL_MAN: C2RustUnnamed_18 = 331;
pub const ACTOR_EN_MK: C2RustUnnamed_18 = 330;
pub const ACTOR_EN_DS: C2RustUnnamed_18 = 329;
pub const ACTOR_BG_GJYO_BRIDGE: C2RustUnnamed_18 = 328;
pub const ACTOR_EN_WONDER_TALK: C2RustUnnamed_18 = 327;
pub const ACTOR_EN_SA: C2RustUnnamed_18 = 326;
pub const ACTOR_BG_SPOT01_IDOSOKO: C2RustUnnamed_18 = 325;
pub const ACTOR_EN_ATTACK_NIW: C2RustUnnamed_18 = 324;
pub const ACTOR_EN_SYATEKI_NIW: C2RustUnnamed_18 = 323;
pub const ACTOR_EN_HEISHI3: C2RustUnnamed_18 = 322;
pub const ACTOR_EN_KANBAN: C2RustUnnamed_18 = 321;
pub const ACTOR_BG_INGATE: C2RustUnnamed_18 = 320;
pub const ACTOR_EN_HS: C2RustUnnamed_18 = 319;
pub const ACTOR_EN_MS: C2RustUnnamed_18 = 318;
pub const ACTOR_EN_GM: C2RustUnnamed_18 = 317;
pub const ACTOR_EN_NIW_LADY: C2RustUnnamed_18 = 316;
pub const ACTOR_EN_CLEAR_TAG: C2RustUnnamed_18 = 315;
pub const ACTOR_EN_SDA: C2RustUnnamed_18 = 314;
pub const ACTOR_OBJ_BLOCKSTOP: C2RustUnnamed_18 = 313;
pub const ACTOR_EN_GE1: C2RustUnnamed_18 = 312;
pub const ACTOR_ITEM_INBOX: C2RustUnnamed_18 = 311;
pub const ACTOR_EN_BLKOBJ: C2RustUnnamed_18 = 310;
pub const ACTOR_EN_NWC: C2RustUnnamed_18 = 309;
pub const ACTOR_UNSET_134: C2RustUnnamed_18 = 308;
pub const ACTOR_EN_DAIKU: C2RustUnnamed_18 = 307;
pub const ACTOR_EN_TORYO: C2RustUnnamed_18 = 306;
pub const ACTOR_EN_EX_RUPPY: C2RustUnnamed_18 = 305;
pub const ACTOR_EN_GOROIWA: C2RustUnnamed_18 = 304;
pub const ACTOR_EN_YABUSAME_MARK: C2RustUnnamed_18 = 303;
pub const ACTOR_EN_OKARINA_TAG: C2RustUnnamed_18 = 302;
pub const ACTOR_OBJ_HSBLOCK: C2RustUnnamed_18 = 301;
pub const ACTOR_OBJ_LIFT: C2RustUnnamed_18 = 300;
pub const ACTOR_OBJ_ELEVATOR: C2RustUnnamed_18 = 299;
pub const ACTOR_OBJ_SWITCH: C2RustUnnamed_18 = 298;
pub const ACTOR_UNSET_129: C2RustUnnamed_18 = 297;
pub const ACTOR_UNSET_128: C2RustUnnamed_18 = 296;
pub const ACTOR_OBJ_BOMBIWA: C2RustUnnamed_18 = 295;
pub const ACTOR_OBJ_BEAN: C2RustUnnamed_18 = 294;
pub const ACTOR_EN_KUSA: C2RustUnnamed_18 = 293;
pub const ACTOR_EN_DIVING_GAME: C2RustUnnamed_18 = 292;
pub const ACTOR_BG_RELAY_OBJECTS: C2RustUnnamed_18 = 291;
pub const ACTOR_EN_PO_RELAY: C2RustUnnamed_18 = 290;
pub const ACTOR_EN_FZ: C2RustUnnamed_18 = 289;
pub const ACTOR_BG_SPOT07_TAKI: C2RustUnnamed_18 = 288;
pub const ACTOR_BG_SPOT03_TAKI: C2RustUnnamed_18 = 287;
pub const ACTOR_OBJ_ICE_POLY: C2RustUnnamed_18 = 286;
pub const ACTOR_EN_TUBO_TRAP: C2RustUnnamed_18 = 285;
pub const ACTOR_EN_HONOTRAP: C2RustUnnamed_18 = 284;
pub const ACTOR_ELF_MSG: C2RustUnnamed_18 = 283;
pub const ACTOR_EN_DNS: C2RustUnnamed_18 = 282;
pub const ACTOR_DEMO_SHD: C2RustUnnamed_18 = 281;
pub const ACTOR_DEMO_EXT: C2RustUnnamed_18 = 280;
pub const ACTOR_EN_G_SWITCH: C2RustUnnamed_18 = 279;
pub const ACTOR_EN_SKJNEEDLE: C2RustUnnamed_18 = 278;
pub const ACTOR_EN_SKJ: C2RustUnnamed_18 = 277;
pub const ACTOR_DEMO_IK: C2RustUnnamed_18 = 276;
pub const ACTOR_EN_IK: C2RustUnnamed_18 = 275;
pub const ACTOR_EN_WONDER_ITEM: C2RustUnnamed_18 = 274;
pub const ACTOR_OBJ_TSUBO: C2RustUnnamed_18 = 273;
pub const ACTOR_OBJ_KIBAKO: C2RustUnnamed_18 = 272;
pub const ACTOR_ITEM_ETCETERA: C2RustUnnamed_18 = 271;
pub const ACTOR_UNSET_10E: C2RustUnnamed_18 = 270;
pub const ACTOR_UNSET_10D: C2RustUnnamed_18 = 269;
pub const ACTOR_ARROW_LIGHT: C2RustUnnamed_18 = 268;
pub const ACTOR_ARROW_ICE: C2RustUnnamed_18 = 267;
pub const ACTOR_ARROW_FIRE: C2RustUnnamed_18 = 266;
pub const ACTOR_UNSET_109: C2RustUnnamed_18 = 265;
pub const ACTOR_BG_UMAJUMP: C2RustUnnamed_18 = 264;
pub const ACTOR_BG_SPOT15_RRBOX: C2RustUnnamed_18 = 263;
pub const ACTOR_BG_GANON_OTYUKA: C2RustUnnamed_18 = 262;
pub const ACTOR_BG_PO_SYOKUDAI: C2RustUnnamed_18 = 261;
pub const ACTOR_BG_SPOT01_IDOMIZU: C2RustUnnamed_18 = 260;
pub const ACTOR_BG_SPOT01_IDOHASHIRA: C2RustUnnamed_18 = 259;
pub const ACTOR_BG_SPOT01_FUSYA: C2RustUnnamed_18 = 258;
pub const ACTOR_EFF_DUST: C2RustUnnamed_18 = 257;
pub const ACTOR_BG_GATE_SHUTTER: C2RustUnnamed_18 = 256;
pub const ACTOR_OBJ_OSHIHIKI: C2RustUnnamed_18 = 255;
pub const ACTOR_FISHING: C2RustUnnamed_18 = 254;
pub const ACTOR_BG_JYA_KANAAMI: C2RustUnnamed_18 = 253;
pub const ACTOR_BG_JYA_COBRA: C2RustUnnamed_18 = 252;
pub const ACTOR_UNSET_FB: C2RustUnnamed_18 = 251;
pub const ACTOR_BG_JYA_ZURERUKABE: C2RustUnnamed_18 = 250;
pub const ACTOR_BG_JYA_GOROIWA: C2RustUnnamed_18 = 249;
pub const ACTOR_BG_SPOT15_SAKU: C2RustUnnamed_18 = 248;
pub const ACTOR_BG_HAKA_GATE: C2RustUnnamed_18 = 247;
pub const ACTOR_EN_ANUBICE_TAG: C2RustUnnamed_18 = 246;
pub const ACTOR_DEMO_6K: C2RustUnnamed_18 = 245;
pub const ACTOR_MAGIC_DARK: C2RustUnnamed_18 = 244;
pub const ACTOR_UNSET_F3: C2RustUnnamed_18 = 243;
pub const ACTOR_UNSET_F2: C2RustUnnamed_18 = 242;
pub const ACTOR_ITEM_OCARINA: C2RustUnnamed_18 = 241;
pub const ACTOR_EN_ICE_HONO: C2RustUnnamed_18 = 240;
pub const ACTOR_BG_ICE_SHELTER: C2RustUnnamed_18 = 239;
pub const ACTOR_ITEM_SHIELD: C2RustUnnamed_18 = 238;
pub const ACTOR_EN_FR: C2RustUnnamed_18 = 237;
pub const ACTOR_EN_NY: C2RustUnnamed_18 = 236;
pub const ACTOR_UNSET_EB: C2RustUnnamed_18 = 235;
pub const ACTOR_UNSET_EA: C2RustUnnamed_18 = 234;
pub const ACTOR_BOSS_SST: C2RustUnnamed_18 = 233;
pub const ACTOR_BOSS_GANON: C2RustUnnamed_18 = 232;
pub const ACTOR_EN_MA1: C2RustUnnamed_18 = 231;
pub const ACTOR_BG_BDAN_SWITCH: C2RustUnnamed_18 = 230;
pub const ACTOR_BG_SPOT16_DOUGHNUT: C2RustUnnamed_18 = 229;
pub const ACTOR_BG_MORI_IDOMIZU: C2RustUnnamed_18 = 228;
pub const ACTOR_BG_MORI_HASHIRA4: C2RustUnnamed_18 = 227;
pub const ACTOR_BG_MORI_HASHIGO: C2RustUnnamed_18 = 226;
pub const ACTOR_EN_ANUBICE_FIRE: C2RustUnnamed_18 = 225;
pub const ACTOR_EN_ANUBICE: C2RustUnnamed_18 = 224;
pub const ACTOR_EN_BX: C2RustUnnamed_18 = 223;
pub const ACTOR_EN_BA: C2RustUnnamed_18 = 222;
pub const ACTOR_EN_RR: C2RustUnnamed_18 = 221;
pub const ACTOR_BOSS_TW: C2RustUnnamed_18 = 220;
pub const ACTOR_EN_HORSE_GAME_CHECK: C2RustUnnamed_18 = 219;
pub const ACTOR_EN_BOM_CHU: C2RustUnnamed_18 = 218;
pub const ACTOR_EN_MA2: C2RustUnnamed_18 = 217;
pub const ACTOR_UNSET_D8: C2RustUnnamed_18 = 216;
pub const ACTOR_BG_HAKA_WATER: C2RustUnnamed_18 = 215;
pub const ACTOR_BG_ICE_OBJECTS: C2RustUnnamed_18 = 214;
pub const ACTOR_BG_SPOT06_OBJECTS: C2RustUnnamed_18 = 213;
pub const ACTOR_BG_MIZU_UZU: C2RustUnnamed_18 = 212;
pub const ACTOR_OBJ_DEKUJR: C2RustUnnamed_18 = 211;
pub const ACTOR_EN_RU2: C2RustUnnamed_18 = 210;
pub const ACTOR_BG_SPOT08_ICEBLOCK: C2RustUnnamed_18 = 209;
pub const ACTOR_BG_BOMBWALL: C2RustUnnamed_18 = 208;
pub const ACTOR_BG_HIDAN_KOWARERUKABE: C2RustUnnamed_18 = 207;
pub const ACTOR_UNSET_CE: C2RustUnnamed_18 = 206;
pub const ACTOR_BG_SPOT16_BOMBSTONE: C2RustUnnamed_18 = 205;
pub const ACTOR_EN_TR: C2RustUnnamed_18 = 204;
pub const ACTOR_EN_IN: C2RustUnnamed_18 = 203;
pub const ACTOR_DEMO_GO: C2RustUnnamed_18 = 202;
pub const ACTOR_DEMO_SA: C2RustUnnamed_18 = 201;
pub const ACTOR_BG_BDAN_OBJECTS: C2RustUnnamed_18 = 200;
pub const ACTOR_EN_KAREBABA: C2RustUnnamed_18 = 199;
pub const ACTOR_EN_BIGOKUTA: C2RustUnnamed_18 = 198;
pub const ACTOR_EN_SB: C2RustUnnamed_18 = 197;
pub const ACTOR_BOSS_MO: C2RustUnnamed_18 = 196;
pub const ACTOR_EN_NB: C2RustUnnamed_18 = 195;
pub const ACTOR_EN_TANA: C2RustUnnamed_18 = 194;
pub const ACTOR_EN_SYATEKI_MAN: C2RustUnnamed_18 = 193;
pub const ACTOR_EN_SYATEKI_ITM: C2RustUnnamed_18 = 192;
pub const ACTOR_BG_SPOT17_FUNEN: C2RustUnnamed_18 = 191;
pub const ACTOR_BG_HAKA_ZOU: C2RustUnnamed_18 = 190;
pub const ACTOR_BG_HAKA_HUTA: C2RustUnnamed_18 = 189;
pub const ACTOR_BG_HAKA_TRAP: C2RustUnnamed_18 = 188;
pub const ACTOR_BG_HAKA_TUBO: C2RustUnnamed_18 = 187;
pub const ACTOR_BOSS_VA: C2RustUnnamed_18 = 186;
pub const ACTOR_BG_SPOT18_OBJ: C2RustUnnamed_18 = 185;
pub const ACTOR_BG_SPOT09_OBJ: C2RustUnnamed_18 = 184;
pub const ACTOR_MIR_RAY: C2RustUnnamed_18 = 183;
pub const ACTOR_EN_BROB: C2RustUnnamed_18 = 182;
pub const ACTOR_EN_FIRE_ROCK: C2RustUnnamed_18 = 181;
pub const ACTOR_EN_ENCOUNT2: C2RustUnnamed_18 = 180;
pub const ACTOR_EN_HEISHI2: C2RustUnnamed_18 = 179;
pub const ACTOR_UNSET_B2: C2RustUnnamed_18 = 178;
pub const ACTOR_BG_HAKA_SGAMI: C2RustUnnamed_18 = 177;
pub const ACTOR_BG_HAKA_SHIP: C2RustUnnamed_18 = 176;
pub const ACTOR_BG_HAKA_MEGANEBG: C2RustUnnamed_18 = 175;
pub const ACTOR_BG_HAKA_MEGANE: C2RustUnnamed_18 = 174;
pub const ACTOR_EN_VB_BALL: C2RustUnnamed_18 = 173;
pub const ACTOR_BG_VB_SIMA: C2RustUnnamed_18 = 172;
pub const ACTOR_EN_FW: C2RustUnnamed_18 = 171;
pub const ACTOR_DEMO_TRE_LGT: C2RustUnnamed_18 = 170;
pub const ACTOR_DEMO_IM: C2RustUnnamed_18 = 169;
pub const ACTOR_DEMO_DU: C2RustUnnamed_18 = 168;
pub const ACTOR_EN_ENCOUNT1: C2RustUnnamed_18 = 167;
pub const ACTOR_EN_RL: C2RustUnnamed_18 = 166;
pub const ACTOR_EN_DHA: C2RustUnnamed_18 = 165;
pub const ACTOR_EN_DH: C2RustUnnamed_18 = 164;
pub const ACTOR_EN_FD_FIRE: C2RustUnnamed_18 = 163;
pub const ACTOR_BOSS_FD2: C2RustUnnamed_18 = 162;
pub const ACTOR_EN_RU1: C2RustUnnamed_18 = 161;
pub const ACTOR_UNSET_A0: C2RustUnnamed_18 = 160;
pub const ACTOR_MAGIC_FIRE: C2RustUnnamed_18 = 159;
pub const ACTOR_MAGIC_WIND: C2RustUnnamed_18 = 158;
pub const ACTOR_BG_HAKA: C2RustUnnamed_18 = 157;
pub const ACTOR_BG_SPOT02_OBJECTS: C2RustUnnamed_18 = 156;
pub const ACTOR_DOOR_ANA: C2RustUnnamed_18 = 155;
pub const ACTOR_EN_HORSE_LINK_CHILD: C2RustUnnamed_18 = 154;
pub const ACTOR_EN_FD: C2RustUnnamed_18 = 153;
pub const ACTOR_EN_DU: C2RustUnnamed_18 = 152;
pub const ACTOR_OBJECT_KANKYO: C2RustUnnamed_18 = 151;
pub const ACTOR_BOSS_FD: C2RustUnnamed_18 = 150;
pub const ACTOR_EN_SW: C2RustUnnamed_18 = 149;
pub const ACTOR_OBJ_MURE: C2RustUnnamed_18 = 148;
pub const ACTOR_BG_PO_EVENT: C2RustUnnamed_18 = 147;
pub const ACTOR_BG_HEAVY_BLOCK: C2RustUnnamed_18 = 146;
pub const ACTOR_EN_PO_SISTERS: C2RustUnnamed_18 = 145;
pub const ACTOR_EN_RD: C2RustUnnamed_18 = 144;
pub const ACTOR_EN_HEISHI1: C2RustUnnamed_18 = 143;
pub const ACTOR_EN_FLOORMAS: C2RustUnnamed_18 = 142;
pub const ACTOR_BG_HIDAN_FWBIG: C2RustUnnamed_18 = 141;
pub const ACTOR_DEMO_KANKYO: C2RustUnnamed_18 = 140;
pub const ACTOR_DEMO_EFFECT: C2RustUnnamed_18 = 139;
pub const ACTOR_EN_VM: C2RustUnnamed_18 = 138;
pub const ACTOR_BG_MORI_RAKKATENJO: C2RustUnnamed_18 = 137;
pub const ACTOR_BG_MORI_KAITENKABE: C2RustUnnamed_18 = 136;
pub const ACTOR_BG_MORI_ELEVATOR: C2RustUnnamed_18 = 135;
pub const ACTOR_BG_MORI_BIGST: C2RustUnnamed_18 = 134;
pub const ACTOR_EN_TK: C2RustUnnamed_18 = 133;
pub const ACTOR_EN_TA: C2RustUnnamed_18 = 132;
pub const ACTOR_UNSET_83: C2RustUnnamed_18 = 131;
pub const ACTOR_EN_VASE: C2RustUnnamed_18 = 130;
pub const ACTOR_EN_AROW_TRAP: C2RustUnnamed_18 = 129;
pub const ACTOR_EN_TRAP: C2RustUnnamed_18 = 128;
pub const ACTOR_UNSET_7F: C2RustUnnamed_18 = 127;
pub const ACTOR_UNSET_7E: C2RustUnnamed_18 = 126;
pub const ACTOR_EN_PU_BOX: C2RustUnnamed_18 = 125;
pub const ACTOR_EN_LIGHTBOX: C2RustUnnamed_18 = 124;
pub const ACTOR_UNSET_7B: C2RustUnnamed_18 = 123;
pub const ACTOR_UNSET_7A: C2RustUnnamed_18 = 122;
pub const ACTOR_UNSET_79: C2RustUnnamed_18 = 121;
pub const ACTOR_UNSET_78: C2RustUnnamed_18 = 120;
pub const ACTOR_EN_WOOD02: C2RustUnnamed_18 = 119;
pub const ACTOR_UNSET_76: C2RustUnnamed_18 = 118;
pub const ACTOR_UNSET_75: C2RustUnnamed_18 = 117;
pub const ACTOR_UNSET_74: C2RustUnnamed_18 = 116;
pub const ACTOR_UNSET_73: C2RustUnnamed_18 = 115;
pub const ACTOR_EN_BIRD: C2RustUnnamed_18 = 114;
pub const ACTOR_BG_HIDAN_HAMSTEP: C2RustUnnamed_18 = 113;
pub const ACTOR_DOOR_TOKI: C2RustUnnamed_18 = 112;
pub const ACTOR_BG_HIDAN_KOUSI: C2RustUnnamed_18 = 111;
pub const ACTOR_BG_MJIN: C2RustUnnamed_18 = 110;
pub const ACTOR_EN_FHG_FIRE: C2RustUnnamed_18 = 109;
pub const ACTOR_BG_TOKI_SWD: C2RustUnnamed_18 = 108;
pub const ACTOR_EN_YUKABYUN: C2RustUnnamed_18 = 107;
pub const ACTOR_BG_TOKI_HIKARI: C2RustUnnamed_18 = 106;
pub const ACTOR_EN_BB: C2RustUnnamed_18 = 105;
pub const ACTOR_BG_MORI_HINERI: C2RustUnnamed_18 = 104;
pub const ACTOR_EN_FHG: C2RustUnnamed_18 = 103;
pub const ACTOR_ARMS_HOOK: C2RustUnnamed_18 = 102;
pub const ACTOR_BG_MIZU_WATER: C2RustUnnamed_18 = 101;
pub const ACTOR_BG_MIZU_MOVEBG: C2RustUnnamed_18 = 100;
pub const ACTOR_EN_VALI: C2RustUnnamed_18 = 99;
pub const ACTOR_BG_MENKURI_EYE: C2RustUnnamed_18 = 98;
pub const ACTOR_BG_MENKURI_KAITEN: C2RustUnnamed_18 = 97;
pub const ACTOR_EN_DEKUNUTS: C2RustUnnamed_18 = 96;
pub const ACTOR_ITEM_B_HEART: C2RustUnnamed_18 = 95;
pub const ACTOR_OBJ_SYOKUDAI: C2RustUnnamed_18 = 94;
pub const ACTOR_DOOR_WARP1: C2RustUnnamed_18 = 93;
pub const ACTOR_BG_DDAN_KD: C2RustUnnamed_18 = 92;
pub const ACTOR_EN_HORSE_ZELDA: C2RustUnnamed_18 = 91;
pub const ACTOR_EN_JJ: C2RustUnnamed_18 = 90;
pub const ACTOR_BG_BREAKWALL: C2RustUnnamed_18 = 89;
pub const ACTOR_BG_DDAN_JD: C2RustUnnamed_18 = 88;
pub const ACTOR_EN_M_THUNDER: C2RustUnnamed_18 = 87;
pub const ACTOR_EN_M_FIRE1: C2RustUnnamed_18 = 86;
pub const ACTOR_EN_DEKUBABA: C2RustUnnamed_18 = 85;
pub const ACTOR_EN_AM: C2RustUnnamed_18 = 84;
pub const ACTOR_UNSET_53: C2RustUnnamed_18 = 83;
pub const ACTOR_BOSS_GANONDROF: C2RustUnnamed_18 = 82;
pub const ACTOR_BG_YDAN_MARUTA: C2RustUnnamed_18 = 81;
pub const ACTOR_BG_YDAN_HASI: C2RustUnnamed_18 = 80;
pub const ACTOR_EN_OE2: C2RustUnnamed_18 = 79;
pub const ACTOR_BG_HIDAN_FSLIFT: C2RustUnnamed_18 = 78;
pub const ACTOR_EN_ZL2: C2RustUnnamed_18 = 77;
pub const ACTOR_EN_BOMBF: C2RustUnnamed_18 = 76;
pub const ACTOR_EN_MB: C2RustUnnamed_18 = 75;
pub const ACTOR_BG_SPOT00_HANEBASI: C2RustUnnamed_18 = 74;
pub const ACTOR_BG_HIDAN_CURTAIN: C2RustUnnamed_18 = 73;
pub const ACTOR_EN_XC: C2RustUnnamed_18 = 72;
pub const ACTOR_BG_HIDAN_SYOKU: C2RustUnnamed_18 = 71;
pub const ACTOR_BG_HIDAN_SIMA: C2RustUnnamed_18 = 70;
pub const ACTOR_BG_HIDAN_SEKIZOU: C2RustUnnamed_18 = 69;
pub const ACTOR_BG_HIDAN_RSEKIZOU: C2RustUnnamed_18 = 68;
pub const ACTOR_BG_HIDAN_ROCK: C2RustUnnamed_18 = 67;
pub const ACTOR_EN_HORSE_GANON: C2RustUnnamed_18 = 66;
pub const ACTOR_BG_HIDAN_HROCK: C2RustUnnamed_18 = 65;
pub const ACTOR_BG_HIDAN_DALM: C2RustUnnamed_18 = 64;
pub const ACTOR_BG_DODOAGO: C2RustUnnamed_18 = 63;
pub const ACTOR_BG_TREEMOUTH: C2RustUnnamed_18 = 62;
pub const ACTOR_EN_OSSAN: C2RustUnnamed_18 = 61;
pub const ACTOR_EN_HORSE_NORMAL: C2RustUnnamed_18 = 60;
pub const ACTOR_EN_RIVER_SOUND: C2RustUnnamed_18 = 59;
pub const ACTOR_EN_EIYER: C2RustUnnamed_18 = 58;
pub const ACTOR_EN_A_OBJ: C2RustUnnamed_18 = 57;
pub const ACTOR_EN_BW: C2RustUnnamed_18 = 56;
pub const ACTOR_EN_ST: C2RustUnnamed_18 = 55;
pub const ACTOR_UNSET_36: C2RustUnnamed_18 = 54;
pub const ACTOR_EN_TP: C2RustUnnamed_18 = 53;
pub const ACTOR_EN_BILI: C2RustUnnamed_18 = 52;
pub const ACTOR_EN_TORCH2: C2RustUnnamed_18 = 51;
pub const ACTOR_EN_BOOM: C2RustUnnamed_18 = 50;
pub const ACTOR_UNSET_31: C2RustUnnamed_18 = 49;
pub const ACTOR_EN_BDFIRE: C2RustUnnamed_18 = 48;
pub const ACTOR_EN_DODOJR: C2RustUnnamed_18 = 47;
pub const ACTOR_DOOR_SHUTTER: C2RustUnnamed_18 = 46;
pub const ACTOR_EN_BUBBLE: C2RustUnnamed_18 = 45;
pub const ACTOR_BG_PUSHBOX: C2RustUnnamed_18 = 44;
pub const ACTOR_EN_GOMA: C2RustUnnamed_18 = 43;
pub const ACTOR_EN_VIEWER: C2RustUnnamed_18 = 42;
pub const ACTOR_EN_ZL1: C2RustUnnamed_18 = 41;
pub const ACTOR_BOSS_GOMA: C2RustUnnamed_18 = 40;
pub const ACTOR_BOSS_DODONGO: C2RustUnnamed_18 = 39;
pub const ACTOR_EN_HATA: C2RustUnnamed_18 = 38;
pub const ACTOR_EN_ZF: C2RustUnnamed_18 = 37;
pub const ACTOR_EN_SCENE_CHANGE: C2RustUnnamed_18 = 36;
pub const ACTOR_EN_HOLL: C2RustUnnamed_18 = 35;
pub const ACTOR_UNSET_22: C2RustUnnamed_18 = 34;
pub const ACTOR_EN_FISH: C2RustUnnamed_18 = 33;
pub const ACTOR_EN_INSECT: C2RustUnnamed_18 = 32;
pub const ACTOR_UNSET_1F: C2RustUnnamed_18 = 31;
pub const ACTOR_EN_BUTTE: C2RustUnnamed_18 = 30;
pub const ACTOR_EN_PEEHAT: C2RustUnnamed_18 = 29;
pub const ACTOR_EN_REEBA: C2RustUnnamed_18 = 28;
pub const ACTOR_EN_TITE: C2RustUnnamed_18 = 27;
pub const ACTOR_UNSET_1A: C2RustUnnamed_18 = 26;
pub const ACTOR_EN_NIW: C2RustUnnamed_18 = 25;
pub const ACTOR_EN_ELF: C2RustUnnamed_18 = 24;
pub const ACTOR_UNSET_17: C2RustUnnamed_18 = 23;
pub const ACTOR_EN_ARROW: C2RustUnnamed_18 = 22;
pub const ACTOR_EN_ITEM00: C2RustUnnamed_18 = 21;
pub const ACTOR_EN_HORSE: C2RustUnnamed_18 = 20;
pub const ACTOR_EN_FIREFLY: C2RustUnnamed_18 = 19;
pub const ACTOR_EN_DODONGO: C2RustUnnamed_18 = 18;
pub const ACTOR_EN_WALLMAS: C2RustUnnamed_18 = 17;
pub const ACTOR_EN_BOM: C2RustUnnamed_18 = 16;
pub const ACTOR_BG_YDAN_SP: C2RustUnnamed_18 = 15;
pub const ACTOR_EN_OKUTA: C2RustUnnamed_18 = 14;
pub const ACTOR_EN_POH: C2RustUnnamed_18 = 13;
pub const ACTOR_BG_HIDAN_FIREWALL: C2RustUnnamed_18 = 12;
pub const ACTOR_BG_DY_YOSEIZO: C2RustUnnamed_18 = 11;
pub const ACTOR_EN_BOX: C2RustUnnamed_18 = 10;
pub const ACTOR_EN_DOOR: C2RustUnnamed_18 = 9;
pub const ACTOR_EN_LIGHT: C2RustUnnamed_18 = 8;
pub const ACTOR_EN_PART: C2RustUnnamed_18 = 7;
pub const ACTOR_UNSET_6: C2RustUnnamed_18 = 6;
pub const ACTOR_UNSET_5: C2RustUnnamed_18 = 5;
pub const ACTOR_EN_GIRLA: C2RustUnnamed_18 = 4;
pub const ACTOR_UNSET_3: C2RustUnnamed_18 = 3;
pub const ACTOR_EN_TEST: C2RustUnnamed_18 = 2;
pub const ACTOR_UNSET_1: C2RustUnnamed_18 = 1;
pub const ACTOR_PLAYER: C2RustUnnamed_18 = 0;
pub type C2RustUnnamed_19 = libc::c_uint;
pub const PLAYER_AP_MAX: C2RustUnnamed_19 = 67;
pub const PLAYER_AP_LENS: C2RustUnnamed_19 = 66;
pub const PLAYER_AP_MASK_TRUTH: C2RustUnnamed_19 = 65;
pub const PLAYER_AP_MASK_GERUDO: C2RustUnnamed_19 = 64;
pub const PLAYER_AP_MASK_ZORA: C2RustUnnamed_19 = 63;
pub const PLAYER_AP_MASK_GORON: C2RustUnnamed_19 = 62;
pub const PLAYER_AP_MASK_BUNNY: C2RustUnnamed_19 = 61;
pub const PLAYER_AP_MASK_SPOOKY: C2RustUnnamed_19 = 60;
pub const PLAYER_AP_MASK_SKULL: C2RustUnnamed_19 = 59;
pub const PLAYER_AP_MASK_KEATON: C2RustUnnamed_19 = 58;
pub const PLAYER_AP_CLAIM_CHECK: C2RustUnnamed_19 = 57;
pub const PLAYER_AP_EYEDROPS: C2RustUnnamed_19 = 56;
pub const PLAYER_AP_FROG: C2RustUnnamed_19 = 55;
pub const PLAYER_AP_PRESCRIPTION: C2RustUnnamed_19 = 54;
pub const PLAYER_AP_SWORD_BROKEN: C2RustUnnamed_19 = 53;
pub const PLAYER_AP_SAW: C2RustUnnamed_19 = 52;
pub const PLAYER_AP_ODD_POTION: C2RustUnnamed_19 = 51;
pub const PLAYER_AP_ODD_MUSHROOM: C2RustUnnamed_19 = 50;
pub const PLAYER_AP_COJIRO: C2RustUnnamed_19 = 49;
pub const PLAYER_AP_POCKET_CUCCO: C2RustUnnamed_19 = 48;
pub const PLAYER_AP_POCKET_EGG: C2RustUnnamed_19 = 47;
pub const PLAYER_AP_BEAN: C2RustUnnamed_19 = 46;
pub const PLAYER_AP_CHICKEN: C2RustUnnamed_19 = 45;
pub const PLAYER_AP_WEIRD_EGG: C2RustUnnamed_19 = 44;
pub const PLAYER_AP_LETTER_ZELDA: C2RustUnnamed_19 = 43;
pub const PLAYER_AP_BOTTLE_FAIRY: C2RustUnnamed_19 = 42;
pub const PLAYER_AP_BOTTLE_MILK_HALF: C2RustUnnamed_19 = 41;
pub const PLAYER_AP_BOTTLE_MILK: C2RustUnnamed_19 = 40;
pub const PLAYER_AP_BOTTLE_POTION_GREEN: C2RustUnnamed_19 = 39;
pub const PLAYER_AP_BOTTLE_POTION_BLUE: C2RustUnnamed_19 = 38;
pub const PLAYER_AP_BOTTLE_POTION_RED: C2RustUnnamed_19 = 37;
pub const PLAYER_AP_BOTTLE_LETTER: C2RustUnnamed_19 = 36;
pub const PLAYER_AP_BOTTLE_BIG_POE: C2RustUnnamed_19 = 35;
pub const PLAYER_AP_BOTTLE_POE: C2RustUnnamed_19 = 34;
pub const PLAYER_AP_BOTTLE_BUG: C2RustUnnamed_19 = 33;
pub const PLAYER_AP_BOTTLE_FIRE: C2RustUnnamed_19 = 32;
pub const PLAYER_AP_BOTTLE_FISH: C2RustUnnamed_19 = 31;
pub const PLAYER_AP_BOTTLE: C2RustUnnamed_19 = 30;
pub const PLAYER_AP_OCARINA_TIME: C2RustUnnamed_19 = 29;
pub const PLAYER_AP_OCARINA_FAIRY: C2RustUnnamed_19 = 28;
pub const PLAYER_AP_NUT: C2RustUnnamed_19 = 27;
pub const PLAYER_AP_DINS_FIRE: C2RustUnnamed_19 = 26;
pub const PLAYER_AP_NAYRUS_LOVE: C2RustUnnamed_19 = 25;
pub const PLAYER_AP_FARORES_WIND: C2RustUnnamed_19 = 24;
pub const PLAYER_AP_MAGIC_SPELL_17: C2RustUnnamed_19 = 23;
pub const PLAYER_AP_MAGIC_SPELL_16: C2RustUnnamed_19 = 22;
pub const PLAYER_AP_MAGIC_SPELL_15: C2RustUnnamed_19 = 21;
pub const PLAYER_AP_BOOMERANG: C2RustUnnamed_19 = 20;
pub const PLAYER_AP_BOMBCHU: C2RustUnnamed_19 = 19;
pub const PLAYER_AP_BOMB: C2RustUnnamed_19 = 18;
pub const PLAYER_AP_LONGSHOT: C2RustUnnamed_19 = 17;
pub const PLAYER_AP_HOOKSHOT: C2RustUnnamed_19 = 16;
pub const PLAYER_AP_SLINGSHOT: C2RustUnnamed_19 = 15;
pub const PLAYER_AP_BOW_0E: C2RustUnnamed_19 = 14;
pub const PLAYER_AP_BOW_0D: C2RustUnnamed_19 = 13;
pub const PLAYER_AP_BOW_0C: C2RustUnnamed_19 = 12;
pub const PLAYER_AP_BOW_LIGHT: C2RustUnnamed_19 = 11;
pub const PLAYER_AP_BOW_ICE: C2RustUnnamed_19 = 10;
pub const PLAYER_AP_BOW_FIRE: C2RustUnnamed_19 = 9;
pub const PLAYER_AP_BOW: C2RustUnnamed_19 = 8;
pub const PLAYER_AP_HAMMER: C2RustUnnamed_19 = 7;
pub const PLAYER_AP_STICK: C2RustUnnamed_19 = 6;
pub const PLAYER_AP_SWORD_BGS: C2RustUnnamed_19 = 5;
pub const PLAYER_AP_SWORD_KOKIRI: C2RustUnnamed_19 = 4;
pub const PLAYER_AP_SWORD_MASTER: C2RustUnnamed_19 = 3;
pub const PLAYER_AP_FISHING_POLE: C2RustUnnamed_19 = 2;
pub const PLAYER_AP_LAST_USED: C2RustUnnamed_19 = 1;
pub const PLAYER_AP_NONE: C2RustUnnamed_19 = 0;
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
pub struct EnTest {
    pub actor: Actor,
    pub bodyPartsPos: [Vec3s; 10],
    pub skelAnime: SkelAnime,
    pub jointTable: [Vec3s; 61],
    pub morphTable: [Vec3s; 61],
    pub upperSkelanime: SkelAnime,
    pub upperJointTable: [Vec3s; 61],
    pub upperMorphTable: [Vec3s; 61],
    pub unk_7C8: u8_0,
    pub actionFunc: EnTestActionFunc,
    pub headRot: Vec3s,
    pub headRotOffset: Vec3s,
    pub unk_7DC: u8_0,
    pub unk_7DD: [libc::c_char; 1],
    pub unk_7DE: u8_0,
    pub iceTimer: s16,
    pub lastDamageEffect: u8_0,
    pub unk_7E4: s32,
    pub timer: s32,
    pub unk_7EC: f32_0,
    pub bodyBreak: BodyBreak,
    pub swordState: s8,
    pub effectIndex: s32,
    pub bodyCollider: ColliderCylinder,
    pub swordCollider: ColliderQuad,
    pub shieldCollider: ColliderCylinder,
}
pub type EnTestActionFunc
    =
    Option<unsafe extern "C" fn(_: *mut EnTest, _: *mut GlobalContext) -> ()>;
pub type C2RustUnnamed_23 = libc::c_uint;
pub const STALFOS_LIMB_MAX: C2RustUnnamed_23 = 61;
pub const STALFOS_LIMB_WAIST: C2RustUnnamed_23 = 60;
pub const STALFOS_LIMB_THIGH_R: C2RustUnnamed_23 = 59;
pub const STALFOS_LIMB_LOWERLEG_R: C2RustUnnamed_23 = 58;
pub const STALFOS_LIMB_FOOT_R: C2RustUnnamed_23 = 57;
pub const STALFOS_LIMB_FOOT_R_ROOT: C2RustUnnamed_23 = 56;
pub const STALFOS_LIMB_ANKLE_R: C2RustUnnamed_23 = 55;
pub const STALFOS_LIMB_ANKLE_R_ROOT: C2RustUnnamed_23 = 54;
pub const STALFOS_LIMB_LOWERLEG_R_ROOT: C2RustUnnamed_23 = 53;
pub const STALFOS_LIMB_THIGH_R_ROOT: C2RustUnnamed_23 = 52;
pub const STALFOS_LIMB_LEG_R_ROOT: C2RustUnnamed_23 = 51;
pub const STALFOS_LIMB_THIGH_L: C2RustUnnamed_23 = 50;
pub const STALFOS_LIMB_LOWERLEG_L: C2RustUnnamed_23 = 49;
pub const STALFOS_LIMB_FOOT_L: C2RustUnnamed_23 = 48;
pub const STALFOS_LIMB_FOOT_L_ROOT: C2RustUnnamed_23 = 47;
pub const STALFOS_LIMB_ANKLE_L: C2RustUnnamed_23 = 46;
pub const STALFOS_LIMB_ANKLE_L_ROOT: C2RustUnnamed_23 = 45;
pub const STALFOS_LIMB_LOWERLEG_L_ROOT: C2RustUnnamed_23 = 44;
pub const STALFOS_LIMB_THIGH_L_ROOT: C2RustUnnamed_23 = 43;
pub const STALFOS_LIMB_LEG_L_ROOT: C2RustUnnamed_23 = 42;
pub const STALFOS_LIMB_LEGS_ROOT: C2RustUnnamed_23 = 41;
pub const STALFOS_LIMB_WAIST_ROOT: C2RustUnnamed_23 = 40;
pub const STALFOS_LIMB_LOWERBODY_ROOT: C2RustUnnamed_23 = 39;
pub const STALFOS_LIMB_CORE_LOWER: C2RustUnnamed_23 = 38;
pub const STALFOS_LIMB_UPPERARM_R: C2RustUnnamed_23 = 37;
pub const STALFOS_LIMB_FOREARM_R: C2RustUnnamed_23 = 36;
pub const STALFOS_LIMB_HAND_R: C2RustUnnamed_23 = 35;
pub const STALFOS_LIMB_SWORD: C2RustUnnamed_23 = 34;
pub const STALFOS_LIMB_HAND_R_ROOT: C2RustUnnamed_23 = 33;
pub const STALFOS_LIMB_FOREARM_R_ROOT: C2RustUnnamed_23 = 32;
pub const STALFOS_LIMB_UPPERARM_R_ROOT: C2RustUnnamed_23 = 31;
pub const STALFOS_LIMB_ARM_R_ROOT: C2RustUnnamed_23 = 30;
pub const STALFOS_LIMB_UPPERARM_L: C2RustUnnamed_23 = 29;
pub const STALFOS_LIMB_FOREARM_L: C2RustUnnamed_23 = 28;
pub const STALFOS_LIMB_SHIELD: C2RustUnnamed_23 = 27;
pub const STALFOS_LIMB_HAND_L: C2RustUnnamed_23 = 26;
pub const STALFOS_LIMB_HAND_L_ROOT: C2RustUnnamed_23 = 25;
pub const STALFOS_LIMB_FOREARM_L_ROOT: C2RustUnnamed_23 = 24;
pub const STALFOS_LIMB_UPPERARM_L_ROOT: C2RustUnnamed_23 = 23;
pub const STALFOS_LIMB_ARM_L_ROOT: C2RustUnnamed_23 = 22;
pub const STALFOS_LIMB_SHOULDER_ARMOR_L: C2RustUnnamed_23 = 21;
pub const STALFOS_LIMB_SHOULDER_ARMOR_L_ROOT: C2RustUnnamed_23 = 20;
pub const STALFOS_LIMB_SHOULDER_L_ROOT: C2RustUnnamed_23 = 19;
pub const STALFOS_LIMB_SHOULDER_ARMOR_R: C2RustUnnamed_23 = 18;
pub const STALFOS_LIMB_SHOULDER_ARMOR_R_ROOT: C2RustUnnamed_23 = 17;
pub const STALFOS_LIMB_SHOULDER_R_ROOT: C2RustUnnamed_23 = 16;
pub const STALFOS_LIMB_CHEST: C2RustUnnamed_23 = 15;
pub const STALFOS_LIMB_CORE_UPPER: C2RustUnnamed_23 = 14;
pub const STALFOS_LIMB_NECK_LOWER: C2RustUnnamed_23 = 13;
pub const STALFOS_LIMB_NECK_UPPER: C2RustUnnamed_23 = 12;
pub const STALFOS_LIMB_HEAD: C2RustUnnamed_23 = 11;
pub const STALFOS_LIMB_JAW: C2RustUnnamed_23 = 10;
pub const STALFOS_LIMB_JAW_ROOT: C2RustUnnamed_23 = 9;
pub const STALFOS_LIMB_8: C2RustUnnamed_23 = 8;
pub const STALFOS_LIMB_7: C2RustUnnamed_23 = 7;
pub const STALFOS_LIMB_HEAD_ROOT: C2RustUnnamed_23 = 6;
pub const STALFOS_LIMB_NECK_ROOT: C2RustUnnamed_23 = 5;
pub const STALFOS_LIMB_CORE_UPPER_ROOT: C2RustUnnamed_23 = 4;
pub const STALFOS_LIMB_CORE_LOWER_ROOT: C2RustUnnamed_23 = 3;
pub const STALFOS_LIMB_UPPERBODY_ROOT: C2RustUnnamed_23 = 2;
pub const STALFOS_LIMB_ROOT: C2RustUnnamed_23 = 1;
pub const STALFOS_LIMB_NONE: C2RustUnnamed_23 = 0;
pub type C2RustUnnamed_24 = libc::c_uint;
pub const STALFOS_TYPE_5: C2RustUnnamed_24 = 5;
pub const STALFOS_TYPE_4: C2RustUnnamed_24 = 4;
pub const STALFOS_TYPE_CEILING: C2RustUnnamed_24 = 3;
pub const STALFOS_TYPE_2: C2RustUnnamed_24 = 2;
pub const STALFOS_TYPE_1: C2RustUnnamed_24 = 1;
pub const STALFOS_TYPE_INVISIBLE: C2RustUnnamed_24 = 0;
pub const STALFOS_DMGEFF_NORMAL: C2RustUnnamed_25 = 0;
pub const STALFOS_DMGEFF_LIGHT: C2RustUnnamed_25 = 14;
pub const STALFOS_DMGEFF_FREEZE: C2RustUnnamed_25 = 15;
pub const STALFOS_DMGEFF_FIREMAGIC: C2RustUnnamed_25 = 6;
pub const STALFOS_DMGEFF_STUN: C2RustUnnamed_25 = 1;
pub const STALFOS_DMGEFF_SLING: C2RustUnnamed_25 = 13;
pub type C2RustUnnamed_25 = libc::c_uint;
static mut sJointCopyFlags: [u8_0; 61] =
    [0 as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
     0 as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
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
     1 as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
     0 as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
     0 as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
     0 as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
     0 as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
     0 as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
     0 as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
     0 as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
     0 as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
     0 as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
     0 as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
     0 as libc::c_int as u8_0];
#[no_mangle]
pub static mut En_Test_InitVars: ActorInit =
    unsafe {
        {
            let mut init =
                ActorInit{id: ACTOR_EN_TEST as libc::c_int as s16,
                          category: ACTORCAT_ENEMY as libc::c_int as u8_0,
                          flags:
                              ((1 as libc::c_int) << 0 as libc::c_int |
                                   (1 as libc::c_int) << 2 as libc::c_int |
                                   (1 as libc::c_int) << 4 as libc::c_int) as
                                  u32_0,
                          objectId: OBJECT_SK2 as libc::c_int as s16,
                          instanceSize:
                              ::std::mem::size_of::<EnTest>() as
                                  libc::c_ulong,
                          init:
                              ::std::mem::transmute::<Option<unsafe extern "C" fn(_:
                                                                                      *mut Actor,
                                                                                  _:
                                                                                      *mut GlobalContext)
                                                                 -> ()>,
                                                      ActorFunc>(Some(EnTest_Init
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
                                                      ActorFunc>(Some(EnTest_Destroy
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
                                                      ActorFunc>(Some(EnTest_Update
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
                                                      ActorFunc>(Some(EnTest_Draw
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
static mut sBodyColliderInit: ColliderCylinderInit =
    {
        let mut init =
            ColliderCylinderInit{base:
                                     {
                                         let mut init =
                                             ColliderInit{colType:
                                                              COLTYPE_HIT5 as
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
                                                            25 as libc::c_int
                                                                as s16,
                                                        height:
                                                            65 as libc::c_int
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
static mut sShieldColliderInit: ColliderCylinderInit =
    {
        let mut init =
            ColliderCylinderInit{base:
                                     {
                                         let mut init =
                                             ColliderInit{colType:
                                                              COLTYPE_METAL as
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
                                                              0 as libc::c_int
                                                                  as u8_0,
                                                          ocFlags2:
                                                              0 as libc::c_int
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
                                                                                               0xffc1ffff
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
                                                                  0 as
                                                                      libc::c_int
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
static mut sSwordColliderInit: ColliderQuadInit =
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
                                                          0 as libc::c_int as
                                                              u8_0,
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
                                                              0 as libc::c_int
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
                                  (STALFOS_DMGEFF_STUN as libc::c_int) <<
                                      4 as libc::c_int) as u8_0,
                             (2 as libc::c_int |
                                  (STALFOS_DMGEFF_NORMAL as libc::c_int) <<
                                      4 as libc::c_int) as u8_0,
                             (1 as libc::c_int |
                                  (STALFOS_DMGEFF_SLING as libc::c_int) <<
                                      4 as libc::c_int) as u8_0,
                             (2 as libc::c_int |
                                  (STALFOS_DMGEFF_NORMAL as libc::c_int) <<
                                      4 as libc::c_int) as u8_0,
                             (0 as libc::c_int |
                                  (STALFOS_DMGEFF_STUN as libc::c_int) <<
                                      4 as libc::c_int) as u8_0,
                             (2 as libc::c_int |
                                  (STALFOS_DMGEFF_NORMAL as libc::c_int) <<
                                      4 as libc::c_int) as u8_0,
                             (2 as libc::c_int |
                                  (STALFOS_DMGEFF_NORMAL as libc::c_int) <<
                                      4 as libc::c_int) as u8_0,
                             (0 as libc::c_int |
                                  (STALFOS_DMGEFF_STUN as libc::c_int) <<
                                      4 as libc::c_int) as u8_0,
                             (1 as libc::c_int |
                                  (STALFOS_DMGEFF_NORMAL as libc::c_int) <<
                                      4 as libc::c_int) as u8_0,
                             (2 as libc::c_int |
                                  (STALFOS_DMGEFF_NORMAL as libc::c_int) <<
                                      4 as libc::c_int) as u8_0,
                             (4 as libc::c_int |
                                  (STALFOS_DMGEFF_NORMAL as libc::c_int) <<
                                      4 as libc::c_int) as u8_0,
                             (2 as libc::c_int |
                                  (STALFOS_DMGEFF_NORMAL as libc::c_int) <<
                                      4 as libc::c_int) as u8_0,
                             (4 as libc::c_int |
                                  (STALFOS_DMGEFF_FREEZE as libc::c_int) <<
                                      4 as libc::c_int) as u8_0,
                             (2 as libc::c_int |
                                  (STALFOS_DMGEFF_LIGHT as libc::c_int) <<
                                      4 as libc::c_int) as u8_0,
                             (2 as libc::c_int |
                                  (STALFOS_DMGEFF_NORMAL as libc::c_int) <<
                                      4 as libc::c_int) as u8_0,
                             (2 as libc::c_int |
                                  (STALFOS_DMGEFF_NORMAL as libc::c_int) <<
                                      4 as libc::c_int) as u8_0,
                             (2 as libc::c_int |
                                  (STALFOS_DMGEFF_NORMAL as libc::c_int) <<
                                      4 as libc::c_int) as u8_0,
                             (0 as libc::c_int |
                                  (STALFOS_DMGEFF_FIREMAGIC as libc::c_int) <<
                                      4 as libc::c_int) as u8_0,
                             (3 as libc::c_int |
                                  (STALFOS_DMGEFF_FREEZE as libc::c_int) <<
                                      4 as libc::c_int) as u8_0,
                             (0 as libc::c_int |
                                  (STALFOS_DMGEFF_LIGHT as libc::c_int) <<
                                      4 as libc::c_int) as u8_0,
                             (0 as libc::c_int |
                                  (STALFOS_DMGEFF_NORMAL as libc::c_int) <<
                                      4 as libc::c_int) as u8_0,
                             (0 as libc::c_int |
                                  (STALFOS_DMGEFF_NORMAL as libc::c_int) <<
                                      4 as libc::c_int) as u8_0,
                             (1 as libc::c_int |
                                  (STALFOS_DMGEFF_NORMAL as libc::c_int) <<
                                      4 as libc::c_int) as u8_0,
                             (4 as libc::c_int |
                                  (STALFOS_DMGEFF_NORMAL as libc::c_int) <<
                                      4 as libc::c_int) as u8_0,
                             (2 as libc::c_int |
                                  (STALFOS_DMGEFF_NORMAL as libc::c_int) <<
                                      4 as libc::c_int) as u8_0,
                             (2 as libc::c_int |
                                  (STALFOS_DMGEFF_NORMAL as libc::c_int) <<
                                      4 as libc::c_int) as u8_0,
                             (8 as libc::c_int |
                                  (STALFOS_DMGEFF_NORMAL as libc::c_int) <<
                                      4 as libc::c_int) as u8_0,
                             (4 as libc::c_int |
                                  (STALFOS_DMGEFF_NORMAL as libc::c_int) <<
                                      4 as libc::c_int) as u8_0,
                             (0 as libc::c_int |
                                  (STALFOS_DMGEFF_NORMAL as libc::c_int) <<
                                      4 as libc::c_int) as u8_0,
                             (0 as libc::c_int |
                                  (STALFOS_DMGEFF_NORMAL as libc::c_int) <<
                                      4 as libc::c_int) as u8_0,
                             (4 as libc::c_int |
                                  (STALFOS_DMGEFF_NORMAL as libc::c_int) <<
                                      4 as libc::c_int) as u8_0,
                             (0 as libc::c_int |
                                  (STALFOS_DMGEFF_NORMAL as libc::c_int) <<
                                      4 as libc::c_int) as u8_0],};
        init
    };
// Initialized in run_static_initializers
static mut sInitChain: [InitChainEntry; 5] =
    [InitChainEntry{cont_type_0_offset_value: [0; 4],}; 5];
#[no_mangle]
pub unsafe extern "C" fn EnTest_SetupAction(mut this: *mut EnTest,
                                            mut actionFunc:
                                                EnTestActionFunc) {
    (*this).actionFunc = actionFunc;
}
#[no_mangle]
pub unsafe extern "C" fn EnTest_Init(mut thisx: *mut Actor,
                                     mut globalCtx: *mut GlobalContext) {
    let mut slashBlure: EffectBlureInit1 =
        EffectBlureInit1{unk_00: [0; 388],
                         p1StartColor: [0; 4],
                         p2StartColor: [0; 4],
                         p1EndColor: [0; 4],
                         p2EndColor: [0; 4],
                         elemDuration: 0,
                         unkFlag: 0,
                         calcMode: 0,};
    let mut this: *mut EnTest = thisx as *mut EnTest;
    Actor_ProcessInitChain(&mut (*this).actor, sInitChain.as_mut_ptr());
    SkelAnime_Init(globalCtx, &mut (*this).skelAnime, &mut gStalfosSkel,
                   &mut gStalfosMiddleGuardAnim,
                   (*this).jointTable.as_mut_ptr(),
                   (*this).morphTable.as_mut_ptr(),
                   STALFOS_LIMB_MAX as libc::c_int);
    SkelAnime_Init(globalCtx, &mut (*this).upperSkelanime, &mut gStalfosSkel,
                   &mut gStalfosMiddleGuardAnim,
                   (*this).upperJointTable.as_mut_ptr(),
                   (*this).upperMorphTable.as_mut_ptr(),
                   STALFOS_LIMB_MAX as libc::c_int);
    ActorShape_Init(&mut (*this).actor.shape, 0.0f32,
                    Some(ActorShadow_DrawFeet as
                             unsafe extern "C" fn(_: *mut Actor,
                                                  _: *mut Lights,
                                                  _: *mut GlobalContext)
                                 -> ()), 90.0f32);
    (*this).actor.colChkInfo.cylRadius = 40 as libc::c_int as s16;
    (*this).actor.colChkInfo.cylHeight = 100 as libc::c_int as s16;
    (*this).actor.focus.pos = (*this).actor.world.pos;
    (*this).actor.focus.pos.y += 45.0f32;
    (*this).actor.colChkInfo.damageTable = &mut sDamageTable;
    Collider_InitCylinder(globalCtx, &mut (*this).bodyCollider);
    Collider_SetCylinder(globalCtx, &mut (*this).bodyCollider,
                         &mut (*this).actor, &mut sBodyColliderInit);
    Collider_InitCylinder(globalCtx, &mut (*this).shieldCollider);
    Collider_SetCylinder(globalCtx, &mut (*this).shieldCollider,
                         &mut (*this).actor, &mut sShieldColliderInit);
    Collider_InitQuad(globalCtx, &mut (*this).swordCollider);
    Collider_SetQuad(globalCtx, &mut (*this).swordCollider,
                     &mut (*this).actor, &mut sSwordColliderInit);
    (*this).actor.colChkInfo.mass = 0xfe as libc::c_int as u8_0;
    (*this).actor.colChkInfo.health = 10 as libc::c_int as u8_0;
    slashBlure.p2EndColor[2 as libc::c_int as usize] =
        255 as libc::c_int as u8_0;
    slashBlure.p2EndColor[1 as libc::c_int as usize] =
        slashBlure.p2EndColor[2 as libc::c_int as usize];
    slashBlure.p2EndColor[0 as libc::c_int as usize] =
        slashBlure.p2EndColor[1 as libc::c_int as usize];
    slashBlure.p1EndColor[2 as libc::c_int as usize] =
        slashBlure.p2EndColor[0 as libc::c_int as usize];
    slashBlure.p1EndColor[1 as libc::c_int as usize] =
        slashBlure.p1EndColor[2 as libc::c_int as usize];
    slashBlure.p1EndColor[0 as libc::c_int as usize] =
        slashBlure.p1EndColor[1 as libc::c_int as usize];
    slashBlure.p2StartColor[2 as libc::c_int as usize] =
        slashBlure.p1EndColor[0 as libc::c_int as usize];
    slashBlure.p2StartColor[1 as libc::c_int as usize] =
        slashBlure.p2StartColor[2 as libc::c_int as usize];
    slashBlure.p2StartColor[0 as libc::c_int as usize] =
        slashBlure.p2StartColor[1 as libc::c_int as usize];
    slashBlure.p1StartColor[3 as libc::c_int as usize] =
        slashBlure.p2StartColor[0 as libc::c_int as usize];
    slashBlure.p1StartColor[2 as libc::c_int as usize] =
        slashBlure.p1StartColor[3 as libc::c_int as usize];
    slashBlure.p1StartColor[1 as libc::c_int as usize] =
        slashBlure.p1StartColor[2 as libc::c_int as usize];
    slashBlure.p1StartColor[0 as libc::c_int as usize] =
        slashBlure.p1StartColor[1 as libc::c_int as usize];
    slashBlure.p1EndColor[3 as libc::c_int as usize] =
        0 as libc::c_int as u8_0;
    slashBlure.p2EndColor[3 as libc::c_int as usize] =
        0 as libc::c_int as u8_0;
    slashBlure.p2StartColor[3 as libc::c_int as usize] =
        64 as libc::c_int as u8_0;
    slashBlure.elemDuration = 4 as libc::c_int;
    slashBlure.unkFlag = 0 as libc::c_int;
    slashBlure.calcMode = 2 as libc::c_int;
    Effect_Add(globalCtx, &mut (*this).effectIndex,
               EFFECT_BLURE1 as libc::c_int, 0 as libc::c_int as u8_0,
               0 as libc::c_int as u8_0,
               &mut slashBlure as *mut EffectBlureInit1 as *mut libc::c_void);
    if (*this).actor.params as libc::c_int !=
           STALFOS_TYPE_CEILING as libc::c_int {
        EnTest_SetupWaitGround(this);
    } else { EnTest_SetupWaitAbove(this); }
    if (*this).actor.params as libc::c_int ==
           STALFOS_TYPE_INVISIBLE as libc::c_int {
        (*this).actor.flags |=
            ((1 as libc::c_int) << 7 as libc::c_int) as libc::c_uint
    };
}
#[no_mangle]
pub unsafe extern "C" fn EnTest_Destroy(mut thisx: *mut Actor,
                                        mut globalCtx: *mut GlobalContext) {
    let mut this: *mut EnTest = thisx as *mut EnTest;
    if (*this).actor.params as libc::c_int != STALFOS_TYPE_2 as libc::c_int &&
           Actor_FindNearby(globalCtx, &mut (*this).actor,
                            ACTOR_EN_TEST as libc::c_int as s16,
                            ACTORCAT_ENEMY as libc::c_int as u8_0,
                            8000.0f32).is_null() {
        func_800F5B58();
    }
    Effect_Delete(globalCtx, (*this).effectIndex);
    Collider_DestroyCylinder(globalCtx, &mut (*this).shieldCollider);
    Collider_DestroyCylinder(globalCtx, &mut (*this).bodyCollider);
    Collider_DestroyQuad(globalCtx, &mut (*this).swordCollider);
}
/* *
 * If EnTest_ChooseAction failed to pick a new action, this function will unconditionally pick
 * a new action as a last resort
 */
#[no_mangle]
pub unsafe extern "C" fn EnTest_ChooseRandomAction(mut this: *mut EnTest,
                                                   mut globalCtx:
                                                       *mut GlobalContext) {
    let mut current_block_3: u64;
    match (Rand_ZeroOne() * 10.0f32) as u32_0 {
        0 | 1 | 5 | 6 => {
            if (*this).actor.xzDistToPlayer < 220.0f32 &&
                   (*this).actor.xzDistToPlayer > 170.0f32 &&
                   Actor_IsFacingPlayer(&mut (*this).actor,
                                        0x71c as libc::c_int as s16) != 0 &&
                   Actor_IsTargeted(globalCtx, &mut (*this).actor) != 0 {
                EnTest_SetupJumpslash(this);
                current_block_3 = 2968425633554183086;
            } else { current_block_3 = 17876106037123091029; }
        }
        8 => { current_block_3 = 17876106037123091029; }
        3 | 4 | 7 => {
            func_808627C4(this, globalCtx);
            current_block_3 = 2968425633554183086;
        }
        2 | 9 | 10 => {
            EnTest_SetupStopAndBlock(this);
            current_block_3 = 2968425633554183086;
        }
        _ => { current_block_3 = 2968425633554183086; }
    }
    match current_block_3 {
        17876106037123091029 =>
        // fallthrough
        {
            EnTest_SetupWalkAndBlock(this);
        }
        _ => { }
    };
}
#[no_mangle]
pub unsafe extern "C" fn EnTest_ChooseAction(mut this: *mut EnTest,
                                             mut globalCtx:
                                                 *mut GlobalContext) {
    let mut pad: s32 = 0;
    let mut player: *mut Player =
        (*globalCtx).actorCtx.actorLists[ACTORCAT_PLAYER as libc::c_int as
                                             usize].head as *mut Player;
    let mut yawDiff: s16 =
        ((*player).actor.shape.rot.y as libc::c_int -
             (*this).actor.shape.rot.y as libc::c_int) as s16;
    yawDiff =
        if yawDiff as libc::c_int >= 0 as libc::c_int {
            yawDiff as libc::c_int
        } else { -(yawDiff as libc::c_int) } as s16;
    if yawDiff as libc::c_int >= 0x61a8 as libc::c_int {
        match (Rand_ZeroOne() * 10.0f32) as u32_0 {
            0 | 3 | 7 => { EnTest_SetupStopAndBlock(this); }
            1 | 5 | 6 | 8 => { func_808627C4(this, globalCtx); }
            2 | 4 | 9 => {
                if (*this).actor.params as libc::c_int !=
                       STALFOS_TYPE_CEILING as libc::c_int {
                    (*this).actor.world.rot.y =
                        (*this).actor.yawTowardsPlayer;
                    EnTest_SetupJumpBack(this);
                }
            }
            _ => { }
        }
    } else if yawDiff as libc::c_int <= 0x3e80 as libc::c_int {
        if (if ((*this).actor.yawTowardsPlayer as libc::c_int -
                    (*this).actor.shape.rot.y as libc::c_int) as s16 as
                   libc::c_int >= 0 as libc::c_int {
                ((*this).actor.yawTowardsPlayer as libc::c_int -
                     (*this).actor.shape.rot.y as libc::c_int) as s16 as
                    libc::c_int
            } else {
                -(((*this).actor.yawTowardsPlayer as libc::c_int -
                       (*this).actor.shape.rot.y as libc::c_int) as s16 as
                      libc::c_int)
            }) > 0x3e80 as libc::c_int {
            if (*globalCtx).gameplayFrames.wrapping_rem(2 as libc::c_int as
                                                            libc::c_uint) !=
                   0 as libc::c_int as libc::c_uint &&
                   (*this).actor.params as libc::c_int !=
                       STALFOS_TYPE_CEILING as libc::c_int {
                (*this).actor.world.rot.y = (*this).actor.yawTowardsPlayer;
                EnTest_SetupJumpBack(this);
            } else if (*this).actor.xzDistToPlayer < 220.0f32 &&
                          (*this).actor.xzDistToPlayer > 170.0f32 {
                if Actor_IsFacingPlayer(&mut (*this).actor,
                                        0x71c as libc::c_int as s16) != 0 &&
                       Actor_IsTargeted(globalCtx, &mut (*this).actor) == 0 {
                    EnTest_SetupJumpslash(this);
                }
            } else { EnTest_SetupWalkAndBlock(this); }
        } else if (*this).actor.xzDistToPlayer < 110.0f32 {
            if Rand_ZeroOne() > 0.2f32 {
                if (*player).stateFlags1 & 0x10 as libc::c_int as libc::c_uint
                       != 0 {
                    if (*this).actor.isTargeted != 0 {
                        EnTest_SetupSlashDown(this);
                    } else { func_808627C4(this, globalCtx); }
                } else { EnTest_SetupSlashDown(this); }
            }
        } else { EnTest_ChooseRandomAction(this, globalCtx); }
    } else { EnTest_ChooseRandomAction(this, globalCtx); };
}
#[no_mangle]
pub unsafe extern "C" fn EnTest_SetupWaitGround(mut this: *mut EnTest) {
    Animation_PlayLoop(&mut (*this).skelAnime, &mut gStalfosMiddleGuardAnim);
    (*this).unk_7C8 = 0 as libc::c_int as u8_0;
    (*this).timer = 15 as libc::c_int;
    (*this).actor.scale.y = 0.0f32;
    (*this).actor.world.pos.y = (*this).actor.home.pos.y - 3.5f32;
    (*this).actor.flags &=
        !((1 as libc::c_int) << 0 as libc::c_int) as libc::c_uint;
    EnTest_SetupAction(this,
                       Some(EnTest_WaitGround as
                                unsafe extern "C" fn(_: *mut EnTest,
                                                     _: *mut GlobalContext)
                                    -> ()));
}
#[no_mangle]
pub unsafe extern "C" fn EnTest_WaitGround(mut this: *mut EnTest,
                                           mut globalCtx:
                                               *mut GlobalContext) {
    SkelAnime_Update(&mut (*this).skelAnime);
    if (*this).timer == 0 as libc::c_int &&
           (if (*this).actor.yDistToPlayer >=
                   0 as libc::c_int as libc::c_float {
                (*this).actor.yDistToPlayer
            } else { -(*this).actor.yDistToPlayer }) < 150.0f32 {
        (*this).unk_7C8 = 3 as libc::c_int as u8_0;
        EnTest_SetupAction(this,
                           Some(EnTest_Rise as
                                    unsafe extern "C" fn(_: *mut EnTest,
                                                         _:
                                                             *mut GlobalContext)
                                        -> ()));
        (*this).actor.world.rot.y = (*this).actor.yawTowardsPlayer;
        (*this).actor.shape.rot.y = (*this).actor.yawTowardsPlayer;
        if (*this).actor.params as libc::c_int !=
               STALFOS_TYPE_2 as libc::c_int {
            func_800F5ACC(0x38 as libc::c_int as u16_0);
        }
    } else {
        if (*this).timer != 0 as libc::c_int { (*this).timer -= 1 }
        (*this).actor.world.pos.y = (*this).actor.home.pos.y - 3.5f32
    };
}
#[no_mangle]
pub unsafe extern "C" fn EnTest_SetupWaitAbove(mut this: *mut EnTest) {
    Animation_PlayLoop(&mut (*this).skelAnime, &mut gStalfosMiddleGuardAnim);
    (*this).unk_7C8 = 0 as libc::c_int as u8_0;
    (*this).actor.world.pos.y = (*this).actor.home.pos.y + 150.0f32;
    Actor_SetScale(&mut (*this).actor, 0.0f32);
    (*this).actor.flags &=
        !((1 as libc::c_int) << 0 as libc::c_int) as libc::c_uint;
    EnTest_SetupAction(this,
                       Some(EnTest_WaitAbove as
                                unsafe extern "C" fn(_: *mut EnTest,
                                                     _: *mut GlobalContext)
                                    -> ()));
}
#[no_mangle]
pub unsafe extern "C" fn EnTest_WaitAbove(mut this: *mut EnTest,
                                          mut globalCtx: *mut GlobalContext) {
    SkelAnime_Update(&mut (*this).skelAnime);
    (*this).actor.world.pos.y = (*this).actor.home.pos.y + 150.0f32;
    if (*this).actor.xzDistToPlayer < 200.0f32 &&
           (if (*this).actor.yDistToPlayer >=
                   0 as libc::c_int as libc::c_float {
                (*this).actor.yDistToPlayer
            } else { -(*this).actor.yDistToPlayer }) < 450.0f32 {
        EnTest_SetupAction(this,
                           Some(EnTest_Fall as
                                    unsafe extern "C" fn(_: *mut EnTest,
                                                         _:
                                                             *mut GlobalContext)
                                        -> ()));
        (*this).actor.flags |=
            ((1 as libc::c_int) << 0 as libc::c_int) as libc::c_uint;
        (*this).actor.world.rot.y = (*this).actor.yawTowardsPlayer;
        (*this).actor.shape.rot.y = (*this).actor.world.rot.y;
        Actor_SetScale(&mut (*this).actor, 0.015f32);
    };
}
#[no_mangle]
pub unsafe extern "C" fn EnTest_SetupIdle(mut this: *mut EnTest) {
    Animation_PlayLoop(&mut (*this).skelAnime, &mut gStalfosMiddleGuardAnim);
    (*this).unk_7C8 = 0xa as libc::c_int as u8_0;
    (*this).timer = (Rand_ZeroOne() * 10.0f32 + 5.0f32) as s32;
    (*this).actor.speedXZ = 0.0f32;
    (*this).actor.world.rot.y = (*this).actor.shape.rot.y;
    EnTest_SetupAction(this,
                       Some(EnTest_Idle as
                                unsafe extern "C" fn(_: *mut EnTest,
                                                     _: *mut GlobalContext)
                                    -> ()));
}
#[no_mangle]
pub unsafe extern "C" fn EnTest_Idle(mut this: *mut EnTest,
                                     mut globalCtx: *mut GlobalContext) {
    let mut player: *mut Player =
        (*globalCtx).actorCtx.actorLists[ACTORCAT_PLAYER as libc::c_int as
                                             usize].head as *mut Player;
    let mut yawDiff: s16 = 0;
    SkelAnime_Update(&mut (*this).skelAnime);
    if EnTest_ReactToProjectile(globalCtx, this) == 0 {
        yawDiff =
            ((*player).actor.shape.rot.y as libc::c_int -
                 (*this).actor.shape.rot.y as libc::c_int) as s16;
        if (*this).actor.xzDistToPlayer < 100.0f32 {
            if (*player).swordState as libc::c_int != 0 as libc::c_int &&
                   (if yawDiff as libc::c_int >= 0 as libc::c_int {
                        yawDiff as libc::c_int
                    } else { -(yawDiff as libc::c_int) }) >=
                       0x1f40 as libc::c_int {
                (*this).actor.world.rot.y = (*this).actor.yawTowardsPlayer;
                (*this).actor.shape.rot.y = (*this).actor.world.rot.y;
                if Rand_ZeroOne() > 0.7f32 &&
                       (*player).swordAnimation as libc::c_int !=
                           0x11 as libc::c_int {
                    EnTest_SetupJumpBack(this);
                } else { func_808627C4(this, globalCtx); }
                return
            }
        }
        if (*this).timer != 0 as libc::c_int {
            (*this).timer -= 1
        } else if Actor_IsFacingPlayer(&mut (*this).actor,
                                       0x1555 as libc::c_int as s16) != 0 {
            if (*this).actor.xzDistToPlayer < 220.0f32 &&
                   (*this).actor.xzDistToPlayer > 160.0f32 &&
                   Rand_ZeroOne() < 0.3f32 {
                if Actor_IsTargeted(globalCtx, &mut (*this).actor) != 0 {
                    EnTest_SetupJumpslash(this);
                } else { func_808627C4(this, globalCtx); }
            } else if Rand_ZeroOne() > 0.3f32 {
                EnTest_SetupWalkAndBlock(this);
            } else { func_808627C4(this, globalCtx); }
        } else if Rand_ZeroOne() > 0.7f32 {
            func_80860BDC(this);
        } else { EnTest_ChooseAction(this, globalCtx); }
    };
}
#[no_mangle]
pub unsafe extern "C" fn EnTest_Fall(mut this: *mut EnTest,
                                     mut globalCtx: *mut GlobalContext) {
    Animation_PlayOnceSetSpeed(&mut (*this).skelAnime,
                               &mut gStalfosLandFromLeapAnim, 0.0f32);
    SkelAnime_Update(&mut (*this).skelAnime);
    if (*this).actor.world.pos.y <= (*this).actor.floorHeight {
        (*this).skelAnime.playSpeed = 1.0f32;
        (*this).unk_7C8 = 0xc as libc::c_int as u8_0;
        (*this).timer = ((*this).unk_7E4 as libc::c_float * 0.15f32) as s32;
        Audio_PlayActorSound2(&mut (*this).actor,
                              0x387a as libc::c_int as u16_0);
        EnTest_SetupAction(this,
                           Some(EnTest_Land as
                                    unsafe extern "C" fn(_: *mut EnTest,
                                                         _:
                                                             *mut GlobalContext)
                                        -> ()));
    };
}
#[no_mangle]
pub unsafe extern "C" fn EnTest_Land(mut this: *mut EnTest,
                                     mut globalCtx: *mut GlobalContext) {
    if SkelAnime_Update(&mut (*this).skelAnime) != 0 {
        EnTest_SetupIdle(this);
        (*this).timer = (Rand_ZeroOne() * 10.0f32 + 5.0f32) as s32
    };
}
#[no_mangle]
pub unsafe extern "C" fn EnTest_SetupWalkAndBlock(mut this: *mut EnTest) {
    Animation_Change(&mut (*this).upperSkelanime,
                     &mut gStalfosBlockWithShieldAnim, 2.0f32, 0.0f32,
                     Animation_GetLastFrame(&mut gStalfosBlockWithShieldAnim
                                                as *mut AnimationHeader as
                                                *mut libc::c_void) as f32_0,
                     2 as libc::c_int as u8_0, 2.0f32);
    Animation_PlayLoop(&mut (*this).skelAnime, &mut gStalfosSlowAdvanceAnim);
    (*this).timer = (Rand_ZeroOne() * 5.0f32) as s16 as s32;
    (*this).unk_7C8 = 0xd as libc::c_int as u8_0;
    (*this).actor.world.rot.y = (*this).actor.shape.rot.y;
    EnTest_SetupAction(this,
                       Some(EnTest_WalkAndBlock as
                                unsafe extern "C" fn(_: *mut EnTest,
                                                     _: *mut GlobalContext)
                                    -> ()));
}
#[no_mangle]
pub unsafe extern "C" fn EnTest_WalkAndBlock(mut this: *mut EnTest,
                                             mut globalCtx:
                                                 *mut GlobalContext) {
    let mut pad: s32 = 0;
    let mut checkDist: f32_0 = 0.0f32;
    let mut pad1: s32 = 0;
    let mut prevFrame: s32 = 0;
    let mut temp_f16: s32 = 0;
    let mut playSpeed: f32_0 = 0.;
    let mut player: *mut Player =
        (*globalCtx).actorCtx.actorLists[ACTORCAT_PLAYER as libc::c_int as
                                             usize].head as *mut Player;
    let mut absPlaySpeed: s32 = 0;
    let mut yawDiff: s16 = 0;
    if EnTest_ReactToProjectile(globalCtx, this) == 0 {
        (*this).timer += 1;
        if Actor_OtherIsTargeted(globalCtx, &mut (*this).actor) != 0 {
            checkDist = 150.0f32
        }
        if (*this).actor.xzDistToPlayer <= 80.0f32 + checkDist {
            Math_SmoothStepToF(&mut (*this).actor.speedXZ, -5.0f32, 1.0f32,
                               0.8f32, 0.0f32);
        } else if (*this).actor.xzDistToPlayer > 110.0f32 + checkDist {
            Math_SmoothStepToF(&mut (*this).actor.speedXZ, 5.0f32, 1.0f32,
                               0.8f32, 0.0f32);
        }
        if (*this).actor.speedXZ >= 5.0f32 {
            (*this).actor.speedXZ = 5.0f32
        } else if (*this).actor.speedXZ < -5.0f32 {
            (*this).actor.speedXZ = -5.0f32
        }
        if (*this).actor.params as libc::c_int ==
               STALFOS_TYPE_CEILING as libc::c_int &&
               Actor_TestFloorInDirection(&mut (*this).actor, globalCtx,
                                          (*this).actor.speedXZ,
                                          (*this).actor.world.rot.y) == 0 {
            (*this).actor.speedXZ *= -1.0f32
        }
        if (if (*this).actor.speedXZ >= 0 as libc::c_int as libc::c_float {
                (*this).actor.speedXZ
            } else { -(*this).actor.speedXZ }) < 3.0f32 {
            Animation_Change(&mut (*this).skelAnime,
                             &mut gStalfosSlowAdvanceAnim, 0.0f32,
                             (*this).skelAnime.curFrame,
                             Animation_GetLastFrame(&mut gStalfosSlowAdvanceAnim
                                                        as
                                                        *mut AnimationHeader
                                                        as *mut libc::c_void)
                                 as f32_0, 0 as libc::c_int as u8_0, -6.0f32);
            playSpeed = (*this).actor.speedXZ * 10.0f32
        } else {
            Animation_Change(&mut (*this).skelAnime,
                             &mut gStalfosFastAdvanceAnim, 0.0f32,
                             (*this).skelAnime.curFrame,
                             Animation_GetLastFrame(&mut gStalfosFastAdvanceAnim
                                                        as
                                                        *mut AnimationHeader
                                                        as *mut libc::c_void)
                                 as f32_0, 0 as libc::c_int as u8_0, -4.0f32);
            playSpeed = (*this).actor.speedXZ * 10.0f32 * 0.02f32
        }
        if (*this).actor.speedXZ >= 0.0f32 {
            if (*this).unk_7DE as libc::c_int == 0 as libc::c_int {
                (*this).unk_7DE = (*this).unk_7DE.wrapping_add(1)
            }
            playSpeed = if playSpeed > 2.5f32 { 2.5f32 } else { playSpeed };
            (*this).skelAnime.playSpeed = playSpeed
        } else {
            playSpeed = if playSpeed < -2.5f32 { -2.5f32 } else { playSpeed };
            (*this).skelAnime.playSpeed = playSpeed
        }
        yawDiff =
            ((*player).actor.shape.rot.y as libc::c_int -
                 (*this).actor.shape.rot.y as libc::c_int) as s16;
        if (*this).actor.xzDistToPlayer < 100.0f32 &&
               (*player).swordState as libc::c_int != 0 as libc::c_int {
            if (if yawDiff as libc::c_int >= 0 as libc::c_int {
                    yawDiff as libc::c_int
                } else { -(yawDiff as libc::c_int) }) >= 0x1f40 as libc::c_int
               {
                (*this).actor.world.rot.y = (*this).actor.yawTowardsPlayer;
                (*this).actor.shape.rot.y = (*this).actor.world.rot.y;
                if Rand_ZeroOne() > 0.7f32 &&
                       (*player).swordAnimation as libc::c_int !=
                           0x11 as libc::c_int {
                    EnTest_SetupJumpBack(this);
                } else { EnTest_SetupStopAndBlock(this); }
                return
            }
        }
        prevFrame = (*this).skelAnime.curFrame as s32;
        SkelAnime_Update(&mut (*this).skelAnime);
        temp_f16 =
            ((*this).skelAnime.curFrame -
                 (if (*this).skelAnime.playSpeed >=
                         0 as libc::c_int as libc::c_float {
                      (*this).skelAnime.playSpeed
                  } else { -(*this).skelAnime.playSpeed })) as s32;
        absPlaySpeed =
            if (*this).skelAnime.playSpeed >=
                   0 as libc::c_int as libc::c_float {
                (*this).skelAnime.playSpeed
            } else { -(*this).skelAnime.playSpeed } as s32;
        if (*this).skelAnime.curFrame as s32 != prevFrame {
            let mut temp_v0_2: s32 = absPlaySpeed + prevFrame;
            if temp_v0_2 > 1 as libc::c_int && temp_f16 <= 0 as libc::c_int ||
                   temp_f16 < 7 as libc::c_int &&
                       temp_v0_2 >= 8 as libc::c_int {
                Audio_PlayActorSound2(&mut (*this).actor,
                                      0x383d as libc::c_int as u16_0);
            }
        }
        if (*this).timer % 32 as libc::c_int == 0 as libc::c_int {
            Audio_PlayActorSound2(&mut (*this).actor,
                                  0x3838 as libc::c_int as u16_0);
            (*this).timer += (Rand_ZeroOne() * 5.0f32) as s16 as libc::c_int
        }
        if (*this).actor.xzDistToPlayer < 220.0f32 &&
               (*this).actor.xzDistToPlayer > 160.0f32 &&
               Actor_IsFacingPlayer(&mut (*this).actor,
                                    0x71c as libc::c_int as s16) != 0 {
            if Actor_IsTargeted(globalCtx, &mut (*this).actor) != 0 {
                if Rand_ZeroOne() < 0.1f32 {
                    EnTest_SetupJumpslash(this);
                    return
                }
            } else if (*player).heldItemActionParam as libc::c_int !=
                          PLAYER_AP_NONE as libc::c_int {
                if (*this).actor.isTargeted != 0 {
                    if (*globalCtx).gameplayFrames.wrapping_rem(2 as
                                                                    libc::c_int
                                                                    as
                                                                    libc::c_uint)
                           != 0 as libc::c_int as libc::c_uint {
                        func_808627C4(this, globalCtx);
                        return
                    }
                    EnTest_ChooseAction(this, globalCtx);
                } else { func_80860EC0(this); }
            }
        }
        if Rand_ZeroOne() < 0.4f32 {
            (*this).actor.world.rot.y = (*this).actor.yawTowardsPlayer;
            (*this).actor.shape.rot.y = (*this).actor.world.rot.y
        }
        if Actor_IsFacingPlayer(&mut (*this).actor,
                                0x11c7 as libc::c_int as s16) == 0 {
            EnTest_SetupIdle(this);
            (*this).timer = (Rand_ZeroOne() * 10.0f32 + 10.0f32) as s32;
            return
        }
        if (*this).actor.xzDistToPlayer < 110.0f32 {
            if Rand_ZeroOne() > 0.2f32 {
                if (*player).stateFlags1 & 0x10 as libc::c_int as libc::c_uint
                       != 0 {
                    if (*this).actor.isTargeted != 0 {
                        EnTest_SetupSlashDown(this);
                    } else { func_808627C4(this, globalCtx); }
                } else { EnTest_SetupSlashDown(this); }
            } else { EnTest_SetupStopAndBlock(this); }
        } else if Rand_ZeroOne() < 0.1f32 { (*this).actor.speedXZ = 5.0f32 }
    };
}
// a variation of sidestep
#[no_mangle]
pub unsafe extern "C" fn func_80860BDC(mut this: *mut EnTest) {
    Animation_PlayLoop(&mut (*this).skelAnime, &mut gStalfosSidestepAnim);
    (*this).unk_7C8 = 0xe as libc::c_int as u8_0;
    EnTest_SetupAction(this,
                       Some(func_80860C24 as
                                unsafe extern "C" fn(_: *mut EnTest,
                                                     _: *mut GlobalContext)
                                    -> ()));
}
// a variation of sidestep
#[no_mangle]
pub unsafe extern "C" fn func_80860C24(mut this: *mut EnTest,
                                       mut globalCtx: *mut GlobalContext) {
    let mut yawDiff: s16 = 0;
    let mut yawChange: s16 = 0;
    let mut playSpeed: f32_0 = 0.;
    let mut prevFrame: s32 = 0;
    let mut temp1: s32 = 0;
    let mut temp2: s32 = 0;
    let mut absPlaySpeed: s32 = 0;
    if EnTest_ReactToProjectile(globalCtx, this) == 0 {
        yawDiff = (*this).actor.yawTowardsPlayer;
        yawDiff =
            (yawDiff as libc::c_int -
                 (*this).actor.shape.rot.y as libc::c_int) as s16;
        if yawDiff as libc::c_int > 0 as libc::c_int {
            yawChange =
                (yawDiff as libc::c_int as libc::c_float / 42.0f32 + 300.0f32)
                    as s16;
            (*this).actor.shape.rot.y =
                ((*this).actor.shape.rot.y as libc::c_int +
                     yawChange as libc::c_int * 2 as libc::c_int) as s16
        } else {
            yawChange =
                (yawDiff as libc::c_int as libc::c_float / 42.0f32 - 300.0f32)
                    as s16;
            (*this).actor.shape.rot.y =
                ((*this).actor.shape.rot.y as libc::c_int +
                     yawChange as libc::c_int * 2 as libc::c_int) as s16
        }
        (*this).actor.world.rot.y = (*this).actor.shape.rot.y;
        if yawDiff as libc::c_int > 0 as libc::c_int {
            playSpeed = yawChange as libc::c_int as libc::c_float * 0.02f32;
            playSpeed = if playSpeed > 1.0f32 { 1.0f32 } else { playSpeed };
            (*this).skelAnime.playSpeed = playSpeed
        } else {
            playSpeed = yawChange as libc::c_int as libc::c_float * 0.02f32;
            playSpeed = if playSpeed < -1.0f32 { -1.0f32 } else { playSpeed };
            (*this).skelAnime.playSpeed = playSpeed
        }
        prevFrame = (*this).skelAnime.curFrame as s32;
        SkelAnime_Update(&mut (*this).skelAnime);
        temp1 =
            ((*this).skelAnime.curFrame -
                 (if (*this).skelAnime.playSpeed >=
                         0 as libc::c_int as libc::c_float {
                      (*this).skelAnime.playSpeed
                  } else { -(*this).skelAnime.playSpeed })) as s32;
        absPlaySpeed =
            if (*this).skelAnime.playSpeed >=
                   0 as libc::c_int as libc::c_float {
                (*this).skelAnime.playSpeed
            } else { -(*this).skelAnime.playSpeed } as s32;
        if (*this).skelAnime.curFrame as s32 != prevFrame {
            temp2 = absPlaySpeed + prevFrame;
            if temp2 > 2 as libc::c_int && temp1 <= 0 as libc::c_int ||
                   temp1 < 7 as libc::c_int && temp2 >= 9 as libc::c_int {
                Audio_PlayActorSound2(&mut (*this).actor,
                                      0x383d as libc::c_int as u16_0);
            }
        }
        if Actor_IsFacingPlayer(&mut (*this).actor,
                                0x71c as libc::c_int as s16) != 0 {
            if Rand_ZeroOne() > 0.8f32 {
                if Rand_ZeroOne() > 0.7f32 {
                    func_80860EC0(this);
                } else { EnTest_ChooseAction(this, globalCtx); }
            } else { EnTest_SetupWalkAndBlock(this); }
        }
    };
}
// a variation of sidestep
#[no_mangle]
pub unsafe extern "C" fn func_80860EC0(mut this: *mut EnTest) {
    Animation_PlayLoop(&mut (*this).skelAnime, &mut gStalfosSidestepAnim);
    (*this).unk_7C8 = 0xf as libc::c_int as u8_0;
    (*this).actor.speedXZ =
        if Rand_ZeroOne() > 0.5f32 { -0.5f32 } else { 0.5f32 };
    (*this).timer = (Rand_ZeroOne() * 15.0f32 + 25.0f32) as s16 as s32;
    (*this).unk_7EC = 0.0f32;
    (*this).actor.world.rot.y = (*this).actor.shape.rot.y;
    EnTest_SetupAction(this,
                       Some(func_80860F84 as
                                unsafe extern "C" fn(_: *mut EnTest,
                                                     _: *mut GlobalContext)
                                    -> ()));
}
// a variation of sidestep
#[no_mangle]
pub unsafe extern "C" fn func_80860F84(mut this: *mut EnTest,
                                       mut globalCtx: *mut GlobalContext) {
    let mut playerYaw180: s16 = 0;
    let mut pad: s32 = 0;
    let mut prevFrame: s32 = 0;
    let mut temp_f16: s32 = 0;
    let mut yawDiff: s16 = 0;
    let mut player: *mut Player =
        (*globalCtx).actorCtx.actorLists[ACTORCAT_PLAYER as libc::c_int as
                                             usize].head as *mut Player;
    let mut checkDist: f32_0 = 0.0f32;
    let mut newYaw: s16 = 0;
    let mut absPlaySpeed: s32 = 0;
    if EnTest_ReactToProjectile(globalCtx, this) == 0 {
        Math_SmoothStepToS(&mut (*this).actor.shape.rot.y,
                           (*this).actor.yawTowardsPlayer,
                           1 as libc::c_int as s16,
                           0xfa0 as libc::c_int as s16,
                           1 as libc::c_int as s16);
        (*this).actor.world.rot.y =
            ((*this).actor.shape.rot.y as libc::c_int + 0x3e80 as libc::c_int)
                as s16;
        playerYaw180 =
            ((*player).actor.shape.rot.y as libc::c_int +
                 0x8000 as libc::c_int) as s16;
        if (*this).actor.speedXZ >= 0.0f32 {
            if (*this).actor.speedXZ < 6.0f32 {
                (*this).actor.speedXZ += 0.5f32
            } else { (*this).actor.speedXZ = 6.0f32 }
        } else if (*this).actor.speedXZ > -6.0f32 {
            (*this).actor.speedXZ -= 0.5f32
        } else { (*this).actor.speedXZ = -6.0f32 }
        if (*this).actor.bgCheckFlags as libc::c_int & 8 as libc::c_int != 0
               ||
               (*this).actor.params as libc::c_int ==
                   STALFOS_TYPE_CEILING as libc::c_int &&
                   Actor_TestFloorInDirection(&mut (*this).actor, globalCtx,
                                              (*this).actor.speedXZ,
                                              (*this).actor.world.rot.y) == 0
           {
            if (*this).actor.bgCheckFlags as libc::c_int & 8 as libc::c_int !=
                   0 {
                if (*this).actor.speedXZ >= 0.0f32 {
                    newYaw =
                        ((*this).actor.shape.rot.y as libc::c_int +
                             0x3fff as libc::c_int) as s16
                } else {
                    newYaw =
                        ((*this).actor.shape.rot.y as libc::c_int -
                             0x3fff as libc::c_int) as s16
                }
                newYaw =
                    ((*this).actor.wallYaw as libc::c_int -
                         newYaw as libc::c_int) as s16
            } else {
                (*this).actor.speedXZ *= -0.8f32;
                newYaw = 0 as libc::c_int as s16
            }
            if (if newYaw as libc::c_int >= 0 as libc::c_int {
                    newYaw as libc::c_int
                } else { -(newYaw as libc::c_int) }) > 0x4000 as libc::c_int {
                (*this).actor.speedXZ *= -0.8f32;
                if (*this).actor.speedXZ < 0.0f32 {
                    (*this).actor.speedXZ -= 0.5f32
                } else { (*this).actor.speedXZ += 0.5f32 }
            }
        }
        if Actor_OtherIsTargeted(globalCtx, &mut (*this).actor) != 0 {
            checkDist = 200.0f32
        }
        if (*this).actor.xzDistToPlayer <= 80.0f32 + checkDist {
            Math_SmoothStepToF(&mut (*this).unk_7EC, -2.5f32, 1.0f32, 0.8f32,
                               0.0f32);
        } else if (*this).actor.xzDistToPlayer > 110.0f32 + checkDist {
            Math_SmoothStepToF(&mut (*this).unk_7EC, 2.5f32, 1.0f32, 0.8f32,
                               0.0f32);
        } else {
            Math_SmoothStepToF(&mut (*this).unk_7EC, 0.0f32, 1.0f32, 6.65f32,
                               0.0f32);
        }
        if (*this).unk_7EC != 0.0f32 {
            (*this).actor.world.pos.x +=
                Math_SinS((*this).actor.shape.rot.y) * (*this).unk_7EC;
            (*this).actor.world.pos.z +=
                Math_CosS((*this).actor.shape.rot.y) * (*this).unk_7EC
        }
        (*this).skelAnime.playSpeed = (*this).actor.speedXZ * 0.5f32;
        prevFrame = (*this).skelAnime.curFrame as s32;
        SkelAnime_Update(&mut (*this).skelAnime);
        temp_f16 =
            ((*this).skelAnime.curFrame -
                 (if (*this).skelAnime.playSpeed >=
                         0 as libc::c_int as libc::c_float {
                      (*this).skelAnime.playSpeed
                  } else { -(*this).skelAnime.playSpeed })) as s32;
        absPlaySpeed =
            if (*this).skelAnime.playSpeed >=
                   0 as libc::c_int as libc::c_float {
                (*this).skelAnime.playSpeed
            } else { -(*this).skelAnime.playSpeed } as s32;
        if (*this).skelAnime.curFrame as s32 != prevFrame {
            let mut temp_v0_2: s32 = absPlaySpeed + prevFrame;
            if temp_v0_2 > 1 as libc::c_int && temp_f16 <= 0 as libc::c_int ||
                   temp_f16 < 7 as libc::c_int &&
                       temp_v0_2 >= 8 as libc::c_int {
                Audio_PlayActorSound2(&mut (*this).actor,
                                      0x383d as libc::c_int as u16_0);
            }
        }
        if (*globalCtx).gameplayFrames & 95 as libc::c_int as libc::c_uint ==
               0 as libc::c_int as libc::c_uint {
            Audio_PlayActorSound2(&mut (*this).actor,
                                  0x3838 as libc::c_int as u16_0);
        }
        yawDiff =
            (playerYaw180 as libc::c_int -
                 (*this).actor.shape.rot.y as libc::c_int) as s16;
        yawDiff =
            if yawDiff as libc::c_int >= 0 as libc::c_int {
                yawDiff as libc::c_int
            } else { -(yawDiff as libc::c_int) } as s16;
        if yawDiff as libc::c_int > 0x6800 as libc::c_int ||
               (*this).timer == 0 as libc::c_int {
            EnTest_ChooseAction(this, globalCtx);
        } else if (*this).timer != 0 as libc::c_int { (*this).timer -= 1 }
    };
}
#[no_mangle]
pub unsafe extern "C" fn EnTest_SetupSlashDown(mut this: *mut EnTest) {
    Animation_PlayOnce(&mut (*this).skelAnime, &mut gStalfosDownSlashAnim);
    Audio_StopSfxByPosAndId(&mut (*this).actor.projectedPos,
                            0x3838 as libc::c_int as u16_0);
    (*this).swordCollider.base.atFlags =
        ((*this).swordCollider.base.atFlags as libc::c_int &
             !((1 as libc::c_int) << 2 as libc::c_int)) as u8_0;
    (*this).unk_7C8 = 0x10 as libc::c_int as u8_0;
    (*this).actor.speedXZ = 0.0f32;
    EnTest_SetupAction(this,
                       Some(EnTest_SlashDown as
                                unsafe extern "C" fn(_: *mut EnTest,
                                                     _: *mut GlobalContext)
                                    -> ()));
    (*this).swordCollider.info.toucher.damage = 16 as libc::c_int as u8_0;
    if (*this).unk_7DE as libc::c_int != 0 as libc::c_int {
        (*this).unk_7DE = 3 as libc::c_int as u8_0
    };
}
#[no_mangle]
pub unsafe extern "C" fn EnTest_SlashDown(mut this: *mut EnTest,
                                          mut globalCtx: *mut GlobalContext) {
    (*this).actor.speedXZ = 0.0f32;
    if ((*this).skelAnime.curFrame as s32) < 4 as libc::c_int {
        Math_SmoothStepToS(&mut (*this).actor.shape.rot.y,
                           (*this).actor.yawTowardsPlayer,
                           1 as libc::c_int as s16,
                           0xbb8 as libc::c_int as s16,
                           0 as libc::c_int as s16);
    }
    if (*this).skelAnime.curFrame as s32 == 7 as libc::c_int {
        Audio_PlayActorSound2(&mut (*this).actor,
                              0x3839 as libc::c_int as u16_0);
    }
    if (*this).skelAnime.curFrame > 7.0f32 &&
           (*this).skelAnime.curFrame < 11.0f32 {
        (*this).swordState = 1 as libc::c_int as s8
    } else { (*this).swordState = 0 as libc::c_int as s8 }
    if SkelAnime_Update(&mut (*this).skelAnime) != 0 {
        if (*globalCtx).gameplayFrames.wrapping_rem(2 as libc::c_int as
                                                        libc::c_uint) !=
               0 as libc::c_int as libc::c_uint {
            EnTest_SetupSlashDownEnd(this);
        } else { EnTest_SetupSlashUp(this); }
    };
}
#[no_mangle]
pub unsafe extern "C" fn EnTest_SetupSlashDownEnd(mut this: *mut EnTest) {
    Animation_PlayOnce(&mut (*this).skelAnime,
                       &mut gStalfosRecoverFromDownSlashAnim);
    (*this).unk_7C8 = 0x12 as libc::c_int as u8_0;
    (*this).actor.speedXZ = 0.0f32;
    EnTest_SetupAction(this,
                       Some(EnTest_SlashDownEnd as
                                unsafe extern "C" fn(_: *mut EnTest,
                                                     _: *mut GlobalContext)
                                    -> ()));
}
#[no_mangle]
pub unsafe extern "C" fn EnTest_SlashDownEnd(mut this: *mut EnTest,
                                             mut globalCtx:
                                                 *mut GlobalContext) {
    let mut player: *mut Player =
        (*globalCtx).actorCtx.actorLists[ACTORCAT_PLAYER as libc::c_int as
                                             usize].head as *mut Player;
    let mut yawDiff: s16 = 0;
    if SkelAnime_Update(&mut (*this).skelAnime) != 0 {
        if (*this).swordCollider.base.atFlags as libc::c_int &
               (1 as libc::c_int) << 1 as libc::c_int != 0 {
            (*this).swordCollider.base.atFlags =
                ((*this).swordCollider.base.atFlags as libc::c_int &
                     !((1 as libc::c_int) << 1 as libc::c_int)) as u8_0;
            if (*this).actor.params as libc::c_int !=
                   STALFOS_TYPE_CEILING as libc::c_int {
                EnTest_SetupJumpBack(this);
                return
            }
        }
        if Rand_ZeroOne() > 0.7f32 {
            EnTest_SetupIdle(this);
            (*this).timer = (Rand_ZeroOne() * 5.0f32 + 5.0f32) as s32;
            return
        }
        (*this).actor.world.rot.y =
            Actor_WorldYawTowardActor(&mut (*this).actor,
                                      &mut (*player).actor);
        if Rand_ZeroOne() > 0.7f32 {
            if (*this).actor.params as libc::c_int !=
                   STALFOS_TYPE_CEILING as libc::c_int {
                EnTest_SetupJumpBack(this);
                return
            }
        }
        yawDiff =
            ((*player).actor.shape.rot.y as libc::c_int -
                 (*this).actor.shape.rot.y as libc::c_int) as s16;
        if (if yawDiff as libc::c_int >= 0 as libc::c_int {
                yawDiff as libc::c_int
            } else { -(yawDiff as libc::c_int) }) <= 0x2710 as libc::c_int {
            yawDiff =
                ((*this).actor.yawTowardsPlayer as libc::c_int -
                     (*this).actor.shape.rot.y as libc::c_int) as s16;
            if (if yawDiff as libc::c_int >= 0 as libc::c_int {
                    yawDiff as libc::c_int
                } else { -(yawDiff as libc::c_int) }) > 0x3e80 as libc::c_int
                   &&
                   (*this).actor.params as libc::c_int !=
                       STALFOS_TYPE_CEILING as libc::c_int {
                (*this).actor.world.rot.y = (*this).actor.yawTowardsPlayer;
                EnTest_SetupJumpBack(this);
            } else if (*player).stateFlags1 &
                          0x10 as libc::c_int as libc::c_uint != 0 {
                if (*this).actor.isTargeted != 0 {
                    EnTest_SetupSlashDown(this);
                } else if (*globalCtx).gameplayFrames.wrapping_rem(2 as
                                                                       libc::c_int
                                                                       as
                                                                       libc::c_uint)
                              != 0 as libc::c_int as libc::c_uint {
                    func_808627C4(this, globalCtx);
                } else { EnTest_SetupJumpBack(this); }
            } else { EnTest_SetupSlashDown(this); }
        } else { func_808627C4(this, globalCtx); }
    };
}
#[no_mangle]
pub unsafe extern "C" fn EnTest_SetupSlashUp(mut this: *mut EnTest) {
    Animation_PlayOnce(&mut (*this).skelAnime, &mut gStalfosUpSlashAnim);
    (*this).swordCollider.base.atFlags =
        ((*this).swordCollider.base.atFlags as libc::c_int &
             !((1 as libc::c_int) << 2 as libc::c_int)) as u8_0;
    (*this).unk_7C8 = 0x11 as libc::c_int as u8_0;
    (*this).swordCollider.info.toucher.damage = 16 as libc::c_int as u8_0;
    (*this).actor.speedXZ = 0.0f32;
    EnTest_SetupAction(this,
                       Some(EnTest_SlashUp as
                                unsafe extern "C" fn(_: *mut EnTest,
                                                     _: *mut GlobalContext)
                                    -> ()));
    if (*this).unk_7DE as libc::c_int != 0 as libc::c_int {
        (*this).unk_7DE = 3 as libc::c_int as u8_0
    };
}
#[no_mangle]
pub unsafe extern "C" fn EnTest_SlashUp(mut this: *mut EnTest,
                                        mut globalCtx: *mut GlobalContext) {
    (*this).actor.speedXZ = 0.0f32;
    if (*this).skelAnime.curFrame as s32 == 2 as libc::c_int {
        Audio_PlayActorSound2(&mut (*this).actor,
                              0x3839 as libc::c_int as u16_0);
    }
    if (*this).skelAnime.curFrame > 1.0f32 &&
           (*this).skelAnime.curFrame < 8.0f32 {
        (*this).swordState = 1 as libc::c_int as s8
    } else { (*this).swordState = 0 as libc::c_int as s8 }
    if SkelAnime_Update(&mut (*this).skelAnime) != 0 {
        EnTest_SetupSlashDown(this);
    };
}
#[no_mangle]
pub unsafe extern "C" fn EnTest_SetupJumpBack(mut this: *mut EnTest) {
    Animation_PlayOnce(&mut (*this).skelAnime,
                       &mut gStalfosJumpBackwardsAnim);
    Audio_PlayActorSound2(&mut (*this).actor, 0x386c as libc::c_int as u16_0);
    (*this).unk_7C8 = 0x14 as libc::c_int as u8_0;
    (*this).timer = 5 as libc::c_int;
    EnTest_SetupAction(this,
                       Some(EnTest_JumpBack as
                                unsafe extern "C" fn(_: *mut EnTest,
                                                     _: *mut GlobalContext)
                                    -> ()));
    if (*this).unk_7DE as libc::c_int != 0 as libc::c_int {
        (*this).unk_7DE = 3 as libc::c_int as u8_0
    }
    if (*this).actor.params as libc::c_int !=
           STALFOS_TYPE_CEILING as libc::c_int {
        (*this).actor.speedXZ = -11.0f32
    } else { (*this).actor.speedXZ = -7.0f32 };
}
#[no_mangle]
pub unsafe extern "C" fn EnTest_JumpBack(mut this: *mut EnTest,
                                         mut globalCtx: *mut GlobalContext) {
    Math_SmoothStepToS(&mut (*this).actor.shape.rot.y,
                       (*this).actor.yawTowardsPlayer,
                       1 as libc::c_int as s16, 0xbb8 as libc::c_int as s16,
                       1 as libc::c_int as s16);
    if (*this).timer == 0 as libc::c_int {
        Audio_PlayActorSound2(&mut (*this).actor,
                              0x3838 as libc::c_int as u16_0);
    } else { (*this).timer -= 1 }
    if SkelAnime_Update(&mut (*this).skelAnime) != 0 {
        if EnTest_ReactToProjectile(globalCtx, this) == 0 {
            if (*this).actor.xzDistToPlayer <= 100.0f32 {
                if Actor_IsFacingPlayer(&mut (*this).actor,
                                        0x1555 as libc::c_int as s16) != 0 {
                    EnTest_SetupSlashDown(this);
                } else {
                    EnTest_SetupIdle(this);
                    (*this).timer = (Rand_ZeroOne() * 5.0f32 + 5.0f32) as s32
                }
            } else if (*this).actor.xzDistToPlayer <= 220.0f32 &&
                          Actor_IsFacingPlayer(&mut (*this).actor,
                                               0xe38 as libc::c_int as s16) !=
                              0 {
                EnTest_SetupJumpslash(this);
            } else {
                EnTest_SetupIdle(this);
                (*this).timer = (Rand_ZeroOne() * 5.0f32 + 5.0f32) as s32
            }
            (*this).actor.flags |=
                ((1 as libc::c_int) << 0 as libc::c_int) as libc::c_uint
        }
    } else if (*this).skelAnime.curFrame ==
                  (*this).skelAnime.endFrame - 4.0f32 {
        Audio_PlayActorSound2(&mut (*this).actor,
                              0x387b as libc::c_int as u16_0);
    };
}
#[no_mangle]
pub unsafe extern "C" fn EnTest_SetupJumpslash(mut this: *mut EnTest) {
    Animation_PlayOnce(&mut (*this).skelAnime, &mut gStalfosJumpAnim);
    Audio_StopSfxByPosAndId(&mut (*this).actor.projectedPos,
                            0x3838 as libc::c_int as u16_0);
    (*this).timer = 0 as libc::c_int;
    (*this).unk_7C8 = 0x17 as libc::c_int as u8_0;
    (*this).actor.velocity.y = 10.0f32;
    (*this).actor.speedXZ = 8.0f32;
    Audio_PlayActorSound2(&mut (*this).actor, 0x386c as libc::c_int as u16_0);
    (*this).actor.world.rot.y = (*this).actor.shape.rot.y;
    (*this).swordCollider.base.atFlags =
        ((*this).swordCollider.base.atFlags as libc::c_int &
             !((1 as libc::c_int) << 2 as libc::c_int)) as u8_0;
    EnTest_SetupAction(this,
                       Some(EnTest_Jumpslash as
                                unsafe extern "C" fn(_: *mut EnTest,
                                                     _: *mut GlobalContext)
                                    -> ()));
    (*this).swordCollider.info.toucher.damage = 32 as libc::c_int as u8_0;
    if (*this).unk_7DE as libc::c_int != 0 as libc::c_int {
        (*this).unk_7DE = 3 as libc::c_int as u8_0
    };
}
#[no_mangle]
pub unsafe extern "C" fn EnTest_Jumpslash(mut this: *mut EnTest,
                                          mut globalCtx: *mut GlobalContext) {
    if SkelAnime_Update(&mut (*this).skelAnime) != 0 {
        if (*this).timer == 0 as libc::c_int {
            Animation_PlayOnce(&mut (*this).skelAnime,
                               &mut gStalfosJumpslashAnim);
            (*this).timer = 1 as libc::c_int;
            (*this).swordState = 1 as libc::c_int as s8;
            Audio_PlayActorSound2(&mut (*this).actor,
                                  0x3839 as libc::c_int as u16_0);
            Audio_PlayActorSound2(&mut (*this).actor,
                                  0x386c as libc::c_int as u16_0);
        } else { (*this).actor.speedXZ = 0.0f32; EnTest_SetupIdle(this); }
    }
    if (*this).timer != 0 as libc::c_int &&
           (*this).skelAnime.curFrame >= 5.0f32 {
        (*this).swordState = 0 as libc::c_int as s8
    }
    if (*this).actor.world.pos.y <= (*this).actor.floorHeight {
        if (*this).actor.speedXZ != 0.0f32 {
            Audio_PlayActorSound2(&mut (*this).actor,
                                  0x387b as libc::c_int as u16_0);
        }
        (*this).actor.world.pos.y = (*this).actor.floorHeight;
        (*this).actor.velocity.y = 0.0f32;
        (*this).actor.speedXZ = 0.0f32
    };
}
#[no_mangle]
pub unsafe extern "C" fn EnTest_SetupJumpUp(mut this: *mut EnTest) {
    Animation_PlayOnce(&mut (*this).skelAnime, &mut gStalfosJumpAnim);
    (*this).timer = 0 as libc::c_int;
    (*this).unk_7C8 = 4 as libc::c_int as u8_0;
    (*this).actor.velocity.y = 14.0f32;
    (*this).actor.speedXZ = 6.0f32;
    Audio_PlayActorSound2(&mut (*this).actor, 0x386c as libc::c_int as u16_0);
    (*this).actor.world.rot.y = (*this).actor.shape.rot.y;
    EnTest_SetupAction(this,
                       Some(EnTest_JumpUp as
                                unsafe extern "C" fn(_: *mut EnTest,
                                                     _: *mut GlobalContext)
                                    -> ()));
}
#[no_mangle]
pub unsafe extern "C" fn EnTest_JumpUp(mut this: *mut EnTest,
                                       mut globalCtx: *mut GlobalContext) {
    Math_SmoothStepToS(&mut (*this).actor.shape.rot.y,
                       (*this).actor.yawTowardsPlayer,
                       1 as libc::c_int as s16, 0xfa0 as libc::c_int as s16,
                       1 as libc::c_int as s16);
    SkelAnime_Update(&mut (*this).skelAnime);
    if (*this).actor.world.pos.y <= (*this).actor.floorHeight {
        Audio_PlayActorSound2(&mut (*this).actor,
                              0x387b as libc::c_int as u16_0);
        (*this).actor.shape.rot.y = (*this).actor.yawTowardsPlayer;
        (*this).actor.world.pos.y = (*this).actor.floorHeight;
        (*this).unk_7E4 = -((*this).actor.velocity.y as s32);
        if (*this).unk_7E4 == 0 as libc::c_int {
            (*this).unk_7E4 = 1 as libc::c_int
        }
        (*this).actor.velocity.y = 0.0f32;
        (*this).actor.speedXZ = 0.0f32;
        (*this).unk_7C8 = 0xc as libc::c_int as u8_0;
        (*this).timer = 4 as libc::c_int;
        Animation_Change(&mut (*this).skelAnime,
                         &mut gStalfosLandFromLeapAnim, 0.0f32, 0.0f32,
                         0.0f32, 2 as libc::c_int as u8_0, 0.0f32);
        EnTest_SetupAction(this,
                           Some(EnTest_Land as
                                    unsafe extern "C" fn(_: *mut EnTest,
                                                         _:
                                                             *mut GlobalContext)
                                        -> ()));
    };
}
#[no_mangle]
pub unsafe extern "C" fn EnTest_SetupStopAndBlock(mut this: *mut EnTest) {
    Animation_Change(&mut (*this).skelAnime, &mut gStalfosBlockWithShieldAnim,
                     2.0f32, 0.0f32,
                     Animation_GetLastFrame(&mut gStalfosBlockWithShieldAnim
                                                as *mut AnimationHeader as
                                                *mut libc::c_void) as f32_0,
                     2 as libc::c_int as u8_0, 2.0f32);
    (*this).unk_7C8 = 0x15 as libc::c_int as u8_0;
    (*this).actor.speedXZ = 0.0f32;
    (*this).timer = (Rand_ZeroOne() * 10.0f32 + 11.0f32) as s32;
    (*this).actor.world.rot.y = (*this).actor.shape.rot.y;
    (*this).unk_7DE = 5 as libc::c_int as u8_0;
    EnTest_SetupAction(this,
                       Some(EnTest_StopAndBlock as
                                unsafe extern "C" fn(_: *mut EnTest,
                                                     _: *mut GlobalContext)
                                    -> ()));
}
#[no_mangle]
pub unsafe extern "C" fn EnTest_StopAndBlock(mut this: *mut EnTest,
                                             mut globalCtx:
                                                 *mut GlobalContext) {
    Math_SmoothStepToF(&mut (*this).actor.speedXZ, 0.0f32, 1.0f32, 0.5f32,
                       0.0f32);
    SkelAnime_Update(&mut (*this).skelAnime);
    if (if ((*this).actor.yawTowardsPlayer as libc::c_int -
                (*this).actor.shape.rot.y as libc::c_int) as s16 as
               libc::c_int >= 0 as libc::c_int {
            ((*this).actor.yawTowardsPlayer as libc::c_int -
                 (*this).actor.shape.rot.y as libc::c_int) as s16 as
                libc::c_int
        } else {
            -(((*this).actor.yawTowardsPlayer as libc::c_int -
                   (*this).actor.shape.rot.y as libc::c_int) as s16 as
                  libc::c_int)
        }) > 0x3e80 as libc::c_int &&
           (*this).actor.params as libc::c_int !=
               STALFOS_TYPE_CEILING as libc::c_int &&
           (*globalCtx).gameplayFrames.wrapping_rem(2 as libc::c_int as
                                                        libc::c_uint) !=
               0 as libc::c_int as libc::c_uint {
        (*this).actor.world.rot.y = (*this).actor.yawTowardsPlayer;
        EnTest_SetupJumpBack(this);
    }
    if (*this).timer == 0 as libc::c_int {
        EnTest_SetupIdleFromBlock(this);
    } else { (*this).timer -= 1 };
}
#[no_mangle]
pub unsafe extern "C" fn EnTest_SetupIdleFromBlock(mut this: *mut EnTest) {
    Animation_MorphToLoop(&mut (*this).skelAnime,
                          &mut gStalfosMiddleGuardAnim, -4.0f32);
    (*this).unk_7C8 = 0x16 as libc::c_int as u8_0;
    EnTest_SetupAction(this,
                       Some(EnTest_IdleFromBlock as
                                unsafe extern "C" fn(_: *mut EnTest,
                                                     _: *mut GlobalContext)
                                    -> ()));
}
#[no_mangle]
pub unsafe extern "C" fn EnTest_IdleFromBlock(mut this: *mut EnTest,
                                              mut globalCtx:
                                                  *mut GlobalContext) {
    Math_SmoothStepToF(&mut (*this).actor.speedXZ, 0.0f32, 1.0f32, 1.5f32,
                       0.0f32);
    SkelAnime_Update(&mut (*this).skelAnime);
    if (*this).skelAnime.morphWeight == 0.0f32 {
        (*this).actor.speedXZ = 0.0f32;
        (*this).unk_7DE = 0 as libc::c_int as u8_0;
        if EnTest_ReactToProjectile(globalCtx, this) == 0 {
            if (*this).actor.xzDistToPlayer < 500.0f32 {
                EnTest_ChooseAction(this, globalCtx);
            } else { func_808627C4(this, globalCtx); }
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn func_80862154(mut this: *mut EnTest) {
    Animation_PlayOnce(&mut (*this).skelAnime,
                       &mut gStalfosFlinchFromHitFrontAnim);
    Audio_PlayActorSound2(&mut (*this).actor, 0x383a as libc::c_int as u16_0);
    (*this).unk_7C8 = 8 as libc::c_int as u8_0;
    (*this).actor.speedXZ = -2.0f32;
    Actor_SetColorFilter(&mut (*this).actor, 0x4000 as libc::c_int as s16,
                         0xff as libc::c_int as s16, 0 as libc::c_int as s16,
                         8 as libc::c_int as s16);
    EnTest_SetupAction(this,
                       Some(func_808621D4 as
                                unsafe extern "C" fn(_: *mut EnTest,
                                                     _: *mut GlobalContext)
                                    -> ()));
}
#[no_mangle]
pub unsafe extern "C" fn func_808621D4(mut this: *mut EnTest,
                                       mut globalCtx: *mut GlobalContext) {
    let mut player: *mut Player =
        (*globalCtx).actorCtx.actorLists[ACTORCAT_PLAYER as libc::c_int as
                                             usize].head as *mut Player;
    Math_SmoothStepToF(&mut (*this).actor.speedXZ, 0.0f32, 1.0f32, 0.1f32,
                       0.0f32);
    if SkelAnime_Update(&mut (*this).skelAnime) != 0 {
        (*this).actor.speedXZ = 0.0f32;
        if (*this).actor.bgCheckFlags as libc::c_int & 8 as libc::c_int != 0
               &&
               ((if ((*this).actor.wallYaw as libc::c_int -
                         (*this).actor.shape.rot.y as libc::c_int) as s16 as
                        libc::c_int >= 0 as libc::c_int {
                     ((*this).actor.wallYaw as libc::c_int -
                          (*this).actor.shape.rot.y as libc::c_int) as s16 as
                         libc::c_int
                 } else {
                     -(((*this).actor.wallYaw as libc::c_int -
                            (*this).actor.shape.rot.y as libc::c_int) as s16
                           as libc::c_int)
                 }) < 0x38a4 as libc::c_int &&
                    (*this).actor.xzDistToPlayer < 80.0f32) {
            EnTest_SetupJumpUp(this);
        } else if EnTest_ReactToProjectile(globalCtx, this) == 0 {
            EnTest_ChooseAction(this, globalCtx);
        } else { return }
    }
    if (*player).swordState as libc::c_int != 0 as libc::c_int {
        if (*this).actor.bgCheckFlags as libc::c_int & 8 as libc::c_int != 0
               &&
               ((if ((*this).actor.wallYaw as libc::c_int -
                         (*this).actor.shape.rot.y as libc::c_int) as s16 as
                        libc::c_int >= 0 as libc::c_int {
                     ((*this).actor.wallYaw as libc::c_int -
                          (*this).actor.shape.rot.y as libc::c_int) as s16 as
                         libc::c_int
                 } else {
                     -(((*this).actor.wallYaw as libc::c_int -
                            (*this).actor.shape.rot.y as libc::c_int) as s16
                           as libc::c_int)
                 }) < 0x38a4 as libc::c_int &&
                    (*this).actor.xzDistToPlayer < 80.0f32) {
            EnTest_SetupJumpUp(this);
        } else if Rand_ZeroOne() > 0.7f32 &&
                      (*this).actor.params as libc::c_int !=
                          STALFOS_TYPE_CEILING as libc::c_int &&
                      (*player).swordAnimation as libc::c_int !=
                          0x11 as libc::c_int {
            EnTest_SetupJumpBack(this);
        } else { EnTest_SetupStopAndBlock(this); }
        (*this).unk_7C8 = 8 as libc::c_int as u8_0
    };
}
#[no_mangle]
pub unsafe extern "C" fn func_80862398(mut this: *mut EnTest) {
    Animation_PlayOnce(&mut (*this).skelAnime,
                       &mut gStalfosFlinchFromHitBehindAnim);
    Audio_PlayActorSound2(&mut (*this).actor, 0x383a as libc::c_int as u16_0);
    (*this).unk_7C8 = 9 as libc::c_int as u8_0;
    (*this).actor.speedXZ = -2.0f32;
    Actor_SetColorFilter(&mut (*this).actor, 0x4000 as libc::c_int as s16,
                         0xff as libc::c_int as s16, 0 as libc::c_int as s16,
                         8 as libc::c_int as s16);
    EnTest_SetupAction(this,
                       Some(func_80862418 as
                                unsafe extern "C" fn(_: *mut EnTest,
                                                     _: *mut GlobalContext)
                                    -> ()));
}
#[no_mangle]
pub unsafe extern "C" fn func_80862418(mut this: *mut EnTest,
                                       mut globalCtx: *mut GlobalContext) {
    let mut player: *mut Player =
        (*globalCtx).actorCtx.actorLists[ACTORCAT_PLAYER as libc::c_int as
                                             usize].head as *mut Player;
    Math_SmoothStepToF(&mut (*this).actor.speedXZ, 0.0f32, 1.0f32, 0.1f32,
                       0.0f32);
    if SkelAnime_Update(&mut (*this).skelAnime) != 0 {
        (*this).actor.speedXZ = 0.0f32;
        if EnTest_ReactToProjectile(globalCtx, this) == 0 {
            EnTest_ChooseAction(this, globalCtx);
        } else { return }
    }
    if (*player).swordState as libc::c_int != 0 as libc::c_int {
        if (*this).actor.bgCheckFlags as libc::c_int & 8 as libc::c_int != 0
               &&
               ((if ((*this).actor.wallYaw as libc::c_int -
                         (*this).actor.shape.rot.y as libc::c_int) as s16 as
                        libc::c_int >= 0 as libc::c_int {
                     ((*this).actor.wallYaw as libc::c_int -
                          (*this).actor.shape.rot.y as libc::c_int) as s16 as
                         libc::c_int
                 } else {
                     -(((*this).actor.wallYaw as libc::c_int -
                            (*this).actor.shape.rot.y as libc::c_int) as s16
                           as libc::c_int)
                 }) < 0x38a4 as libc::c_int &&
                    (*this).actor.xzDistToPlayer < 80.0f32) {
            EnTest_SetupJumpUp(this);
        } else if Rand_ZeroOne() > 0.7f32 &&
                      (*this).actor.params as libc::c_int !=
                          STALFOS_TYPE_CEILING as libc::c_int &&
                      (*player).swordAnimation as libc::c_int !=
                          0x11 as libc::c_int {
            EnTest_SetupJumpBack(this);
        } else { EnTest_SetupStopAndBlock(this); }
        (*this).unk_7C8 = 8 as libc::c_int as u8_0
    };
}
#[no_mangle]
pub unsafe extern "C" fn EnTest_SetupStunned(mut this: *mut EnTest) {
    (*this).unk_7C8 = 0xb as libc::c_int as u8_0;
    (*this).unk_7DE = 0 as libc::c_int as u8_0;
    (*this).swordState = 0 as libc::c_int as s8;
    (*this).skelAnime.playSpeed = 0.0f32;
    (*this).actor.speedXZ = -4.0f32;
    if (*this).lastDamageEffect as libc::c_int ==
           STALFOS_DMGEFF_LIGHT as libc::c_int {
        Actor_SetColorFilter(&mut (*this).actor,
                             -(0x8000 as libc::c_int) as s16,
                             0x78 as libc::c_int as s16,
                             0 as libc::c_int as s16,
                             0x50 as libc::c_int as s16);
    } else {
        Actor_SetColorFilter(&mut (*this).actor, 0 as libc::c_int as s16,
                             0x78 as libc::c_int as s16,
                             0 as libc::c_int as s16,
                             0x50 as libc::c_int as s16);
        if (*this).lastDamageEffect as libc::c_int ==
               STALFOS_DMGEFF_FREEZE as libc::c_int {
            (*this).iceTimer = 36 as libc::c_int as s16
        } else {
            Animation_PlayOnceSetSpeed(&mut (*this).skelAnime,
                                       &mut gStalfosFlinchFromHitFrontAnim,
                                       0.0f32);
        }
    }
    Audio_PlayActorSound2(&mut (*this).actor, 0x389e as libc::c_int as u16_0);
    EnTest_SetupAction(this,
                       Some(EnTest_Stunned as
                                unsafe extern "C" fn(_: *mut EnTest,
                                                     _: *mut GlobalContext)
                                    -> ()));
}
#[no_mangle]
pub unsafe extern "C" fn EnTest_Stunned(mut this: *mut EnTest,
                                        mut globalCtx: *mut GlobalContext) {
    let mut player: *mut Player =
        (*globalCtx).actorCtx.actorLists[ACTORCAT_PLAYER as libc::c_int as
                                             usize].head as *mut Player;
    Math_SmoothStepToF(&mut (*this).actor.speedXZ, 0.0f32, 1.0f32, 1.0f32,
                       0.0f32);
    if (*this).actor.colorFilterTimer as libc::c_int == 0 as libc::c_int {
        if (*this).actor.colChkInfo.health as libc::c_int == 0 as libc::c_int
           {
            func_80862FA8(this, globalCtx);
        } else if (*player).swordState as libc::c_int != 0 as libc::c_int {
            if (*this).actor.bgCheckFlags as libc::c_int & 8 as libc::c_int !=
                   0 &&
                   ((if ((*this).actor.wallYaw as libc::c_int -
                             (*this).actor.shape.rot.y as libc::c_int) as s16
                            as libc::c_int >= 0 as libc::c_int {
                         ((*this).actor.wallYaw as libc::c_int -
                              (*this).actor.shape.rot.y as libc::c_int) as s16
                             as libc::c_int
                     } else {
                         -(((*this).actor.wallYaw as libc::c_int -
                                (*this).actor.shape.rot.y as libc::c_int) as
                               s16 as libc::c_int)
                     }) < 0x38a4 as libc::c_int &&
                        (*this).actor.xzDistToPlayer < 80.0f32) {
                EnTest_SetupJumpUp(this);
            } else if Rand_ZeroOne() > 0.7f32 &&
                          (*player).swordAnimation as libc::c_int !=
                              0x11 as libc::c_int {
                EnTest_SetupJumpBack(this);
            } else { EnTest_SetupStopAndBlock(this); }
            (*this).unk_7C8 = 8 as libc::c_int as u8_0
        } else {
            (*this).actor.speedXZ = 0.0f32;
            if EnTest_ReactToProjectile(globalCtx, this) == 0 {
                EnTest_ChooseAction(this, globalCtx);
            }
        }
    };
}
// a variation of sidestep
#[no_mangle]
pub unsafe extern "C" fn func_808627C4(mut this: *mut EnTest,
                                       mut globalCtx: *mut GlobalContext) {
    if Actor_OtherIsTargeted(globalCtx, &mut (*this).actor) != 0 {
        func_80860EC0(this);
        return
    }
    Animation_MorphToLoop(&mut (*this).skelAnime, &mut gStalfosSidestepAnim,
                          -2.0f32);
    Math_SmoothStepToS(&mut (*this).actor.shape.rot.y,
                       (*this).actor.yawTowardsPlayer,
                       1 as libc::c_int as s16, 0xfa0 as libc::c_int as s16,
                       1 as libc::c_int as s16);
    (*this).actor.speedXZ =
        if (*globalCtx).gameplayFrames.wrapping_rem(2 as libc::c_int as
                                                        libc::c_uint) !=
               0 as libc::c_int as libc::c_uint {
            -4.0f32
        } else { 4.0f32 };
    (*this).actor.world.rot.y =
        ((*this).actor.shape.rot.y as libc::c_int + 0x3fff as libc::c_int) as
            s16;
    (*this).timer = (Rand_ZeroOne() * 20.0f32 + 20.0f32) as s32;
    (*this).unk_7C8 = 0x18 as libc::c_int as u8_0;
    EnTest_SetupAction(this,
                       Some(func_808628C8 as
                                unsafe extern "C" fn(_: *mut EnTest,
                                                     _: *mut GlobalContext)
                                    -> ()));
    (*this).unk_7EC = 0.0f32;
}
// a variation of sidestep
#[no_mangle]
pub unsafe extern "C" fn func_808628C8(mut this: *mut EnTest,
                                       mut globalCtx: *mut GlobalContext) {
    let mut pad: s32 = 0;
    let mut player: *mut Player =
        (*globalCtx).actorCtx.actorLists[ACTORCAT_PLAYER as libc::c_int as
                                             usize].head as *mut Player;
    let mut pad1: s32 = 0;
    let mut prevFrame: s32 = 0;
    let mut temp_f16: s32 = 0;
    let mut pad2: s32 = 0;
    let mut checkDist: f32_0 = 0.0f32;
    let mut newYaw: s16 = 0;
    let mut absPlaySpeed: f32_0 = 0.;
    Math_SmoothStepToS(&mut (*this).actor.shape.rot.y,
                       (*this).actor.yawTowardsPlayer,
                       1 as libc::c_int as s16, 0xfa0 as libc::c_int as s16,
                       1 as libc::c_int as s16);
    if (*this).unk_7DE as libc::c_int == 0 as libc::c_int {
        (*this).unk_7DE = (*this).unk_7DE.wrapping_add(1)
    }
    if (*this).actor.speedXZ >= 0.0f32 {
        if (*this).actor.speedXZ < 6.0f32 {
            (*this).actor.speedXZ += 0.125f32
        } else { (*this).actor.speedXZ = 6.0f32 }
    } else if (*this).actor.speedXZ > -6.0f32 {
        (*this).actor.speedXZ -= 0.125f32
    } else { (*this).actor.speedXZ = -6.0f32 }
    if (*this).actor.bgCheckFlags as libc::c_int & 8 as libc::c_int != 0 ||
           (*this).actor.params as libc::c_int ==
               STALFOS_TYPE_CEILING as libc::c_int &&
               Actor_TestFloorInDirection(&mut (*this).actor, globalCtx,
                                          (*this).actor.speedXZ,
                                          ((*this).actor.shape.rot.y as
                                               libc::c_int +
                                               0x3fff as libc::c_int) as s16)
                   == 0 {
        if (*this).actor.bgCheckFlags as libc::c_int & 8 as libc::c_int != 0 {
            if (*this).actor.speedXZ >= 0.0f32 {
                newYaw =
                    ((*this).actor.shape.rot.y as libc::c_int +
                         0x3fff as libc::c_int) as s16
            } else {
                newYaw =
                    ((*this).actor.shape.rot.y as libc::c_int -
                         0x3fff as libc::c_int) as s16
            }
            newYaw =
                ((*this).actor.wallYaw as libc::c_int - newYaw as libc::c_int)
                    as s16
        } else {
            (*this).actor.speedXZ *= -0.8f32;
            newYaw = 0 as libc::c_int as s16
        }
        if (if newYaw as libc::c_int >= 0 as libc::c_int {
                newYaw as libc::c_int
            } else { -(newYaw as libc::c_int) }) > 0x4000 as libc::c_int {
            (*this).actor.speedXZ *= -0.8f32;
            if (*this).actor.speedXZ < 0.0f32 {
                (*this).actor.speedXZ -= 0.5f32
            } else { (*this).actor.speedXZ += 0.5f32 }
        }
    }
    (*this).actor.world.rot.y =
        ((*this).actor.shape.rot.y as libc::c_int + 0x3fff as libc::c_int) as
            s16;
    if Actor_OtherIsTargeted(globalCtx, &mut (*this).actor) != 0 {
        checkDist = 200.0f32
    }
    if (*this).actor.xzDistToPlayer <= 80.0f32 + checkDist {
        Math_SmoothStepToF(&mut (*this).unk_7EC, -2.5f32, 1.0f32, 0.8f32,
                           0.0f32);
    } else if (*this).actor.xzDistToPlayer > 110.0f32 + checkDist {
        Math_SmoothStepToF(&mut (*this).unk_7EC, 2.5f32, 1.0f32, 0.8f32,
                           0.0f32);
    } else {
        Math_SmoothStepToF(&mut (*this).unk_7EC, 0.0f32, 1.0f32, 6.65f32,
                           0.0f32);
    }
    if (*this).unk_7EC != 0.0f32 {
        (*this).actor.world.pos.x +=
            Math_SinS((*this).actor.shape.rot.y) * (*this).unk_7EC;
        (*this).actor.world.pos.z +=
            Math_CosS((*this).actor.shape.rot.y) * (*this).unk_7EC
    }
    (*this).skelAnime.playSpeed = (*this).actor.speedXZ * 0.5f32;
    prevFrame = (*this).skelAnime.curFrame as s32;
    SkelAnime_Update(&mut (*this).skelAnime);
    temp_f16 =
        ((*this).skelAnime.curFrame -
             (if (*this).skelAnime.playSpeed >=
                     0 as libc::c_int as libc::c_float {
                  (*this).skelAnime.playSpeed
              } else { -(*this).skelAnime.playSpeed })) as s32;
    absPlaySpeed =
        if (*this).skelAnime.playSpeed >= 0 as libc::c_int as libc::c_float {
            (*this).skelAnime.playSpeed
        } else { -(*this).skelAnime.playSpeed };
    if (*this).timer % 32 as libc::c_int == 0 as libc::c_int {
        Audio_PlayActorSound2(&mut (*this).actor,
                              0x3838 as libc::c_int as u16_0);
    }
    if (*this).skelAnime.curFrame as s32 != prevFrame {
        let mut temp_v0_2: s32 = absPlaySpeed as s32 + prevFrame;
        if temp_v0_2 > 1 as libc::c_int && temp_f16 <= 0 as libc::c_int ||
               temp_f16 < 7 as libc::c_int && temp_v0_2 >= 8 as libc::c_int {
            Audio_PlayActorSound2(&mut (*this).actor,
                                  0x383d as libc::c_int as u16_0);
        }
    }
    if (*this).timer == 0 as libc::c_int {
        if Actor_OtherIsTargeted(globalCtx, &mut (*this).actor) != 0 {
            EnTest_SetupIdle(this);
        } else if Actor_IsTargeted(globalCtx, &mut (*this).actor) != 0 {
            if EnTest_ReactToProjectile(globalCtx, this) == 0 {
                EnTest_ChooseAction(this, globalCtx);
            }
        } else if (*player).heldItemActionParam as libc::c_int !=
                      PLAYER_AP_NONE as libc::c_int {
            if (*globalCtx).gameplayFrames.wrapping_rem(2 as libc::c_int as
                                                            libc::c_uint) !=
                   0 as libc::c_int as libc::c_uint {
                EnTest_SetupIdle(this);
            } else { EnTest_SetupWalkAndBlock(this); }
        } else { EnTest_SetupWalkAndBlock(this); }
    } else { (*this).timer -= 1 };
}
#[no_mangle]
pub unsafe extern "C" fn func_80862DBC(mut this: *mut EnTest,
                                       mut globalCtx: *mut GlobalContext) {
    Audio_PlayActorSound2(&mut (*this).actor, 0x383a as libc::c_int as u16_0);
    (*this).unk_7C8 = 2 as libc::c_int as u8_0;
    BodyBreak_Alloc(&mut (*this).bodyBreak, 60 as libc::c_int, globalCtx);
    (*this).actor.home.rot.x = 0 as libc::c_int as s16;
    if (*this).swordState as libc::c_int >= 0 as libc::c_int {
        EffectBlure_AddSpace(Effect_GetByIndex((*this).effectIndex) as
                                 *mut EffectBlure);
        (*this).swordState = -(1 as libc::c_int) as s8
    }
    (*this).actor.flags &=
        !((1 as libc::c_int) << 0 as libc::c_int) as libc::c_uint;
    if (*this).actor.params as libc::c_int == STALFOS_TYPE_5 as libc::c_int {
        Actor_ChangeCategory(globalCtx, &mut (*globalCtx).actorCtx,
                             &mut (*this).actor,
                             ACTORCAT_PROP as libc::c_int as u8_0);
    }
    EnTest_SetupAction(this,
                       Some(func_80862E6C as
                                unsafe extern "C" fn(_: *mut EnTest,
                                                     _: *mut GlobalContext)
                                    -> ()));
}
#[no_mangle]
pub unsafe extern "C" fn func_80862E6C(mut this: *mut EnTest,
                                       mut globalCtx: *mut GlobalContext) {
    if (*this).actor.child.is_null() {
        if (*this).actor.home.rot.x as libc::c_int == 0 as libc::c_int {
            (*this).actor.home.rot.x = (*this).bodyBreak.count
        }
        if BodyBreak_SpawnParts(&mut (*this).actor, &mut (*this).bodyBreak,
                                globalCtx,
                                ((*this).actor.params as libc::c_int +
                                     8 as libc::c_int) as s16) != 0 {
            (*this).actor.child = &mut (*this).actor
        }
    } else if (*this).actor.home.rot.x as libc::c_int == 0 as libc::c_int {
        (*this).actor.colChkInfo.health = 10 as libc::c_int as u8_0;
        if (*this).actor.params as libc::c_int ==
               STALFOS_TYPE_4 as libc::c_int {
            (*this).actor.params = -(1 as libc::c_int) as s16
        } else {
            Actor_ChangeCategory(globalCtx, &mut (*globalCtx).actorCtx,
                                 &mut (*this).actor,
                                 ACTORCAT_ENEMY as libc::c_int as u8_0);
        }
        (*this).actor.child = 0 as *mut Actor;
        (*this).actor.flags |=
            ((1 as libc::c_int) << 0 as libc::c_int) as libc::c_uint;
        EnTest_SetupJumpBack(this);
    } else if (*this).actor.params as libc::c_int ==
                  STALFOS_TYPE_5 as libc::c_int &&
                  Actor_FindNearby(globalCtx, &mut (*this).actor,
                                   ACTOR_EN_TEST as libc::c_int as s16,
                                   ACTORCAT_ENEMY as libc::c_int as u8_0,
                                   8000.0f32).is_null() {
        Item_DropCollectibleRandom(globalCtx, &mut (*this).actor,
                                   &mut (*this).actor.world.pos,
                                   0xd0 as libc::c_int as s16);
        if !(*this).actor.parent.is_null() {
            (*(*this).actor.parent).home.rot.z -= 1
        }
        Actor_Kill(&mut (*this).actor);
    };
}
#[no_mangle]
pub unsafe extern "C" fn func_80862FA8(mut this: *mut EnTest,
                                       mut globalCtx: *mut GlobalContext) {
    Animation_PlayOnce(&mut (*this).skelAnime,
                       &mut gStalfosFallOverBackwardsAnim);
    Audio_PlayActorSound2(&mut (*this).actor, 0x383b as libc::c_int as u16_0);
    (*this).unk_7DE = 0 as libc::c_int as u8_0;
    (*this).actor.flags &=
        !((1 as libc::c_int) << 0 as libc::c_int) as libc::c_uint;
    (*this).actor.colorFilterTimer = 0 as libc::c_int as u8_0;
    (*this).actor.speedXZ = 0.0f32;
    if (*this).actor.params as libc::c_int <=
           STALFOS_TYPE_CEILING as libc::c_int {
        (*this).unk_7C8 = 5 as libc::c_int as u8_0;
        EnTest_SetupAction(this,
                           Some(func_80863044 as
                                    unsafe extern "C" fn(_: *mut EnTest,
                                                         _:
                                                             *mut GlobalContext)
                                        -> ()));
    } else { func_80862DBC(this, globalCtx); };
}
#[no_mangle]
pub unsafe extern "C" fn func_80863044(mut this: *mut EnTest,
                                       mut globalCtx: *mut GlobalContext) {
    if SkelAnime_Update(&mut (*this).skelAnime) != 0 {
        (*this).timer = (Rand_ZeroOne() * 10.0f32 + 10.0f32) as s32;
        (*this).unk_7C8 = 7 as libc::c_int as u8_0;
        EnTest_SetupAction(this,
                           Some(func_808633E8 as
                                    unsafe extern "C" fn(_: *mut EnTest,
                                                         _:
                                                             *mut GlobalContext)
                                        -> ()));
        BodyBreak_Alloc(&mut (*this).bodyBreak, 60 as libc::c_int, globalCtx);
    }
    if (*this).skelAnime.curFrame as s32 == 15 as libc::c_int {
        Audio_PlayActorSound2(&mut (*this).actor,
                              0x387a as libc::c_int as u16_0);
    };
}
#[no_mangle]
pub unsafe extern "C" fn func_808630F0(mut this: *mut EnTest,
                                       mut globalCtx: *mut GlobalContext) {
    Animation_PlayOnce(&mut (*this).skelAnime,
                       &mut gStalfosFallOverForwardsAnim);
    Audio_PlayActorSound2(&mut (*this).actor, 0x383b as libc::c_int as u16_0);
    (*this).unk_7C8 = 6 as libc::c_int as u8_0;
    (*this).actor.colorFilterTimer = 0 as libc::c_int as u8_0;
    (*this).unk_7DE = 0 as libc::c_int as u8_0;
    (*this).actor.speedXZ = 0.0f32;
    if (*this).actor.params as libc::c_int <=
           STALFOS_TYPE_CEILING as libc::c_int {
        (*this).actor.flags &=
            !((1 as libc::c_int) << 0 as libc::c_int) as libc::c_uint;
        EnTest_SetupAction(this,
                           Some(func_8086318C as
                                    unsafe extern "C" fn(_: *mut EnTest,
                                                         _:
                                                             *mut GlobalContext)
                                        -> ()));
    } else { func_80862DBC(this, globalCtx); };
}
#[no_mangle]
pub unsafe extern "C" fn func_8086318C(mut this: *mut EnTest,
                                       mut globalCtx: *mut GlobalContext) {
    if SkelAnime_Update(&mut (*this).skelAnime) != 0 {
        (*this).timer = (Rand_ZeroOne() * 10.0f32 + 10.0f32) as s32;
        (*this).unk_7C8 = 7 as libc::c_int as u8_0;
        EnTest_SetupAction(this,
                           Some(func_808633E8 as
                                    unsafe extern "C" fn(_: *mut EnTest,
                                                         _:
                                                             *mut GlobalContext)
                                        -> ()));
        BodyBreak_Alloc(&mut (*this).bodyBreak, 60 as libc::c_int, globalCtx);
    }
    if (*this).skelAnime.curFrame as s32 == 10 as libc::c_int ||
           (*this).skelAnime.curFrame as s32 == 25 as libc::c_int {
        Audio_PlayActorSound2(&mut (*this).actor,
                              0x387a as libc::c_int as u16_0);
    };
}
#[no_mangle]
pub unsafe extern "C" fn EnTest_SetupRecoil(mut this: *mut EnTest) {
    (*this).swordState = 0 as libc::c_int as s8;
    (*this).skelAnime.moveFlags = 2 as libc::c_int as u8_0;
    (*this).unk_7C8 = 0x13 as libc::c_int as u8_0;
    (*this).skelAnime.playSpeed = -1.0f32;
    (*this).skelAnime.startFrame = (*this).skelAnime.curFrame;
    (*this).skelAnime.endFrame = 0.0f32;
    EnTest_SetupAction(this,
                       Some(EnTest_Recoil as
                                unsafe extern "C" fn(_: *mut EnTest,
                                                     _: *mut GlobalContext)
                                    -> ()));
}
#[no_mangle]
pub unsafe extern "C" fn EnTest_Recoil(mut this: *mut EnTest,
                                       mut globalCtx: *mut GlobalContext) {
    if SkelAnime_Update(&mut (*this).skelAnime) != 0 {
        if Rand_ZeroOne() > 0.7f32 {
            EnTest_SetupIdle(this);
            (*this).timer = (Rand_ZeroOne() * 5.0f32 + 5.0f32) as s32
        } else if (*globalCtx).gameplayFrames.wrapping_rem(2 as libc::c_int as
                                                               libc::c_uint)
                      != 0 as libc::c_int as libc::c_uint &&
                      (*this).actor.params as libc::c_int !=
                          STALFOS_TYPE_CEILING as libc::c_int {
            EnTest_SetupJumpBack(this);
        } else { func_808627C4(this, globalCtx); }
    };
}
#[no_mangle]
pub unsafe extern "C" fn EnTest_Rise(mut this: *mut EnTest,
                                     mut globalCtx: *mut GlobalContext) {
    SkelAnime_Update(&mut (*this).skelAnime);
    if (*this).actor.scale.y < 0.015f32 {
        (*this).actor.scale.y += 0.002f32;
        (*this).actor.world.pos.y = (*this).actor.home.pos.y - 3.5f32
    } else {
        (*this).actor.world.pos.y = (*this).actor.home.pos.y;
        EnTest_SetupJumpBack(this);
    };
}
#[no_mangle]
pub unsafe extern "C" fn func_808633E8(mut this: *mut EnTest,
                                       mut globalCtx: *mut GlobalContext) {
    (*this).actor.params = STALFOS_TYPE_1 as libc::c_int as s16;
    if BodyBreak_SpawnParts(&mut (*this).actor, &mut (*this).bodyBreak,
                            globalCtx, (*this).actor.params) != 0 {
        Item_DropCollectibleRandom(globalCtx, &mut (*this).actor,
                                   &mut (*this).actor.world.pos,
                                   0xd0 as libc::c_int as s16);
        if !(*this).actor.parent.is_null() {
            (*(*this).actor.parent).home.rot.z -= 1
        }
        Actor_Kill(&mut (*this).actor);
    };
}
#[no_mangle]
pub unsafe extern "C" fn EnTest_UpdateHeadRot(mut this: *mut EnTest,
                                              mut globalCtx:
                                                  *mut GlobalContext) {
    let mut lookAngle: s16 = (*this).actor.yawTowardsPlayer;
    lookAngle =
        (lookAngle as libc::c_int -
             ((*this).headRot.y as libc::c_int +
                  (*this).actor.shape.rot.y as libc::c_int) as s16 as
                 libc::c_int) as s16;
    (*this).headRotOffset.y =
        if (lookAngle as libc::c_int) < -(0x7d0 as libc::c_int) {
            -(0x7d0 as libc::c_int)
        } else if lookAngle as libc::c_int > 0x7d0 as libc::c_int {
            0x7d0 as libc::c_int
        } else { lookAngle as libc::c_int } as s16;
    (*this).headRot.y =
        ((*this).headRot.y as libc::c_int +
             (*this).headRotOffset.y as libc::c_int) as s16;
    (*this).headRot.y =
        if ((*this).headRot.y as libc::c_int) < -(0x382f as libc::c_int) {
            -(0x382f as libc::c_int)
        } else if (*this).headRot.y as libc::c_int > 0x382f as libc::c_int {
            0x382f as libc::c_int
        } else { (*this).headRot.y as libc::c_int } as s16;
}
#[no_mangle]
pub unsafe extern "C" fn EnTest_UpdateDamage(mut this: *mut EnTest,
                                             mut globalCtx:
                                                 *mut GlobalContext) {
    let mut player: *mut Player =
        (*globalCtx).actorCtx.actorLists[ACTORCAT_PLAYER as libc::c_int as
                                             usize].head as *mut Player;
    if (*this).shieldCollider.base.acFlags as libc::c_int &
           (1 as libc::c_int) << 7 as libc::c_int != 0 {
        (*this).shieldCollider.base.acFlags =
            ((*this).shieldCollider.base.acFlags as libc::c_int &
                 !((1 as libc::c_int) << 7 as libc::c_int)) as u8_0;
        (*this).bodyCollider.base.acFlags =
            ((*this).bodyCollider.base.acFlags as libc::c_int &
                 !((1 as libc::c_int) << 1 as libc::c_int)) as u8_0;
        if (*this).unk_7C8 as libc::c_int >= 0xa as libc::c_int {
            (*this).actor.speedXZ = -4.0f32
        }
    } else if (*this).bodyCollider.base.acFlags as libc::c_int &
                  (1 as libc::c_int) << 1 as libc::c_int != 0 {
        (*this).bodyCollider.base.acFlags =
            ((*this).bodyCollider.base.acFlags as libc::c_int &
                 !((1 as libc::c_int) << 1 as libc::c_int)) as u8_0;
        if (*this).actor.colChkInfo.damageEffect as libc::c_int !=
               STALFOS_DMGEFF_SLING as libc::c_int &&
               (*this).actor.colChkInfo.damageEffect as libc::c_int !=
                   STALFOS_DMGEFF_FIREMAGIC as libc::c_int {
            (*this).lastDamageEffect = (*this).actor.colChkInfo.damageEffect;
            if (*this).swordState as libc::c_int >= 1 as libc::c_int {
                (*this).swordState = 0 as libc::c_int as s8
            }
            (*this).unk_7DC = (*player).unk_845;
            (*this).actor.world.rot.y = (*this).actor.yawTowardsPlayer;
            Actor_SetDropFlag(&mut (*this).actor,
                              &mut (*this).bodyCollider.info,
                              0 as libc::c_int);
            Audio_StopSfxByPosAndId(&mut (*this).actor.projectedPos,
                                    0x3838 as libc::c_int as u16_0);
            if (*this).actor.colChkInfo.damageEffect as libc::c_int ==
                   STALFOS_DMGEFF_STUN as libc::c_int ||
                   (*this).actor.colChkInfo.damageEffect as libc::c_int ==
                       STALFOS_DMGEFF_FREEZE as libc::c_int ||
                   (*this).actor.colChkInfo.damageEffect as libc::c_int ==
                       STALFOS_DMGEFF_LIGHT as libc::c_int {
                if (*this).unk_7C8 as libc::c_int != 0xb as libc::c_int {
                    Actor_ApplyDamage(&mut (*this).actor);
                    EnTest_SetupStunned(this);
                }
            } else if Actor_IsFacingPlayer(&mut (*this).actor,
                                           0x4000 as libc::c_int as s16) != 0
             {
                if Actor_ApplyDamage(&mut (*this).actor) as libc::c_int ==
                       0 as libc::c_int {
                    Enemy_StartFinishingBlow(globalCtx, &mut (*this).actor);
                    func_80862FA8(this, globalCtx);
                } else { func_80862154(this); }
            } else if Actor_ApplyDamage(&mut (*this).actor) as libc::c_int ==
                          0 as libc::c_int {
                func_808630F0(this, globalCtx);
                Enemy_StartFinishingBlow(globalCtx, &mut (*this).actor);
            } else { func_80862398(this); }
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn EnTest_Update(mut thisx: *mut Actor,
                                       mut globalCtx: *mut GlobalContext) {
    let mut this: *mut EnTest = thisx as *mut EnTest;
    let mut oldWeight: f32_0 = 0.;
    let mut floorProperty: u32_0 = 0;
    let mut pad: s32 = 0;
    EnTest_UpdateDamage(this, globalCtx);
    if (*this).actor.colChkInfo.damageEffect as libc::c_int !=
           STALFOS_DMGEFF_FIREMAGIC as libc::c_int {
        Actor_MoveForward(&mut (*this).actor);
        Actor_UpdateBgCheckInfo(globalCtx, &mut (*this).actor, 75.0f32,
                                30.0f32, 30.0f32, 0x1d as libc::c_int);
        if (*this).actor.params as libc::c_int ==
               STALFOS_TYPE_1 as libc::c_int {
            if (*this).actor.world.pos.y <= (*this).actor.home.pos.y {
                (*this).actor.world.pos.y = (*this).actor.home.pos.y;
                (*this).actor.velocity.y = 0.0f32
            }
            if (*this).actor.floorHeight <= (*this).actor.home.pos.y {
                (*this).actor.floorHeight = (*this).actor.home.pos.y
            }
        } else if (*this).actor.bgCheckFlags as libc::c_int & 2 as libc::c_int
                      != 0 {
            floorProperty =
                func_80041EA4(&mut (*globalCtx).colCtx,
                              (*this).actor.floorPoly,
                              (*this).actor.floorBgId as s32);
            if floorProperty == 5 as libc::c_int as libc::c_uint ||
                   floorProperty == 0xc as libc::c_int as libc::c_uint ||
                   func_80041D4C(&mut (*globalCtx).colCtx,
                                 (*this).actor.floorPoly,
                                 (*this).actor.floorBgId as s32) ==
                       9 as libc::c_int as libc::c_uint {
                Actor_Kill(&mut (*this).actor);
                return
            }
        }
        (*this).actionFunc.expect("non-null function pointer")(this,
                                                               globalCtx);
        let mut current_block_33: u64;
        match (*this).unk_7DE as libc::c_int {
            1 => {
                Animation_Change(&mut (*this).upperSkelanime,
                                 &mut gStalfosBlockWithShieldAnim, 2.0f32,
                                 0.0f32,
                                 Animation_GetLastFrame(&mut gStalfosBlockWithShieldAnim
                                                            as
                                                            *mut AnimationHeader
                                                            as
                                                            *mut libc::c_void)
                                     as f32_0, 2 as libc::c_int as u8_0,
                                 2.0f32);
                AnimationContext_SetCopyTrue(globalCtx,
                                             (*this).skelAnime.limbCount as
                                                 s32,
                                             (*this).skelAnime.jointTable,
                                             (*this).upperSkelanime.jointTable,
                                             sJointCopyFlags.as_mut_ptr());
                (*this).unk_7DE = (*this).unk_7DE.wrapping_add(1);
                current_block_33 = 11048769245176032998;
            }
            2 => {
                SkelAnime_Update(&mut (*this).upperSkelanime);
                SkelAnime_CopyFrameTableTrue(&mut (*this).skelAnime,
                                             (*this).skelAnime.jointTable,
                                             (*this).upperSkelanime.jointTable,
                                             sJointCopyFlags.as_mut_ptr());
                current_block_33 = 11048769245176032998;
            }
            3 => {
                (*this).unk_7DE = (*this).unk_7DE.wrapping_add(1);
                (*this).upperSkelanime.morphWeight = 4.0f32;
                current_block_33 = 16538243685862673438;
            }
            4 => { current_block_33 = 16538243685862673438; }
            0 | _ => { current_block_33 = 11048769245176032998; }
        }
        match current_block_33 {
            16538243685862673438 =>
            // fallthrough
            {
                oldWeight = (*this).upperSkelanime.morphWeight;
                (*this).upperSkelanime.morphWeight -= 1.0f32;
                if (*this).upperSkelanime.morphWeight <= 0.0f32 {
                    (*this).unk_7DE = 0 as libc::c_int as u8_0
                }
                SkelAnime_InterpFrameTable((*this).skelAnime.limbCount as s32,
                                           (*this).upperSkelanime.jointTable,
                                           (*this).upperSkelanime.jointTable,
                                           (*this).skelAnime.jointTable,
                                           1.0f32 -
                                               (*this).upperSkelanime.morphWeight
                                                   / oldWeight);
                SkelAnime_CopyFrameTableTrue(&mut (*this).skelAnime,
                                             (*this).skelAnime.jointTable,
                                             (*this).upperSkelanime.jointTable,
                                             sJointCopyFlags.as_mut_ptr());
            }
            _ => { }
        }
        if (*this).actor.colorFilterTimer as libc::c_int == 0 as libc::c_int
               &&
               (*this).actor.colChkInfo.health as libc::c_int !=
                   0 as libc::c_int {
            if (*this).unk_7C8 as libc::c_int != 0x10 as libc::c_int &&
                   (*this).unk_7C8 as libc::c_int != 0x17 as libc::c_int {
                EnTest_UpdateHeadRot(this, globalCtx);
            } else {
                Math_SmoothStepToS(&mut (*this).headRot.y,
                                   0 as libc::c_int as s16,
                                   1 as libc::c_int as s16,
                                   0x3e8 as libc::c_int as s16,
                                   0 as libc::c_int as s16);
            }
        }
    }
    Collider_UpdateCylinder(&mut (*this).actor, &mut (*this).bodyCollider);
    (*this).actor.focus.pos = (*this).actor.world.pos;
    (*this).actor.focus.pos.y += 45.0f32;
    if (*this).actor.colChkInfo.health as libc::c_int > 0 as libc::c_int ||
           (*this).actor.colorFilterTimer as libc::c_int != 0 as libc::c_int {
        CollisionCheck_SetOC(globalCtx, &mut (*globalCtx).colChkCtx,
                             &mut (*this).bodyCollider.base);
        if (*this).unk_7C8 as libc::c_int >= 0xa as libc::c_int &&
               ((*this).actor.colorFilterTimer as libc::c_int ==
                    0 as libc::c_int ||
                    (*this).actor.colorFilterParams as libc::c_int &
                        0x4000 as libc::c_int == 0) {
            CollisionCheck_SetAC(globalCtx, &mut (*globalCtx).colChkCtx,
                                 &mut (*this).bodyCollider.base);
        }
        if (*this).unk_7DE as libc::c_int != 0 as libc::c_int {
            CollisionCheck_SetAC(globalCtx, &mut (*globalCtx).colChkCtx,
                                 &mut (*this).shieldCollider.base);
        }
    }
    if (*this).swordState as libc::c_int >= 1 as libc::c_int {
        if (*this).swordCollider.base.atFlags as libc::c_int &
               (1 as libc::c_int) << 2 as libc::c_int == 0 {
            CollisionCheck_SetAT(globalCtx, &mut (*globalCtx).colChkCtx,
                                 &mut (*this).swordCollider.base);
        } else {
            (*this).swordCollider.base.atFlags =
                ((*this).swordCollider.base.atFlags as libc::c_int &
                     !((1 as libc::c_int) << 2 as libc::c_int)) as u8_0;
            EnTest_SetupRecoil(this);
        }
    }
    if (*this).actor.params as libc::c_int ==
           STALFOS_TYPE_INVISIBLE as libc::c_int {
        if (*globalCtx).actorCtx.unk_03 as libc::c_int != 0 as libc::c_int {
            (*this).actor.flags |=
                ((1 as libc::c_int) << 0 as libc::c_int |
                     (1 as libc::c_int) << 7 as libc::c_int) as libc::c_uint;
            (*this).actor.shape.shadowDraw =
                Some(ActorShadow_DrawFeet as
                         unsafe extern "C" fn(_: *mut Actor, _: *mut Lights,
                                              _: *mut GlobalContext) -> ())
        } else {
            (*this).actor.flags &=
                !((1 as libc::c_int) << 0 as libc::c_int |
                      (1 as libc::c_int) << 7 as libc::c_int) as libc::c_uint;
            (*this).actor.shape.shadowDraw = None
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn EnTest_OverrideLimbDraw(mut globalCtx:
                                                     *mut GlobalContext,
                                                 mut limbIndex: s32,
                                                 mut dList: *mut *mut Gfx,
                                                 mut pos: *mut Vec3f,
                                                 mut rot: *mut Vec3s,
                                                 mut thisx: *mut libc::c_void)
 -> s32 {
    let mut this: *mut EnTest = thisx as *mut EnTest;
    let mut pad: s32 = 0;
    if limbIndex == STALFOS_LIMB_HEAD_ROOT as libc::c_int {
        (*rot).x =
            ((*rot).x as libc::c_int + (*this).headRot.y as libc::c_int) as
                s16;
        (*rot).y =
            ((*rot).y as libc::c_int - (*this).headRot.x as libc::c_int) as
                s16;
        (*rot).z =
            ((*rot).z as libc::c_int + (*this).headRot.z as libc::c_int) as
                s16
    } else if limbIndex == STALFOS_LIMB_HEAD as libc::c_int {
        let mut __gfxCtx: *mut GraphicsContext = 0 as *mut GraphicsContext;
        let mut dispRefs: [*mut Gfx; 4] = [0 as *mut Gfx; 4];
        __gfxCtx = (*globalCtx).state.gfxCtx;
        Graph_OpenDisps(dispRefs.as_mut_ptr(), (*globalCtx).state.gfxCtx,
                        b"../z_en_test.c\x00" as *const u8 as
                            *const libc::c_char, 3582 as libc::c_int);
        let fresh0 = (*__gfxCtx).polyOpa.p;
        (*__gfxCtx).polyOpa.p = (*__gfxCtx).polyOpa.p.offset(1);
        let mut _g: *mut Gfx = fresh0;
        (*_g).words.w0 =
            (0xe7 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 24 as libc::c_int;
        (*_g).words.w1 = 0 as libc::c_int as libc::c_uint;
        let fresh1 = (*__gfxCtx).polyOpa.p;
        (*__gfxCtx).polyOpa.p = (*__gfxCtx).polyOpa.p.offset(1);
        let mut _g_0: *mut Gfx = fresh1;
        (*_g_0).words.w0 =
            (0xfb as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 24 as libc::c_int;
        (*_g_0).words.w1 =
            ((80 as libc::c_int +
                  (if (Math_SinS((*globalCtx).gameplayFrames.wrapping_mul(2000
                                                                              as
                                                                              libc::c_int
                                                                              as
                                                                              libc::c_uint)
                                     as s16) * 175.0f32) as s16 as libc::c_int
                          >= 0 as libc::c_int {
                       (Math_SinS((*globalCtx).gameplayFrames.wrapping_mul(2000
                                                                               as
                                                                               libc::c_int
                                                                               as
                                                                               libc::c_uint)
                                      as s16) * 175.0f32) as s16 as
                           libc::c_int
                   } else {
                       -((Math_SinS((*globalCtx).gameplayFrames.wrapping_mul(2000
                                                                                 as
                                                                                 libc::c_int
                                                                                 as
                                                                                 libc::c_uint)
                                        as s16) * 175.0f32) as s16 as
                             libc::c_int)
                   })) as u32_0 &
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
                (255 as libc::c_int as u32_0 &
                     (((0x1 as libc::c_int) << 8 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    0 as libc::c_int;
        Graph_CloseDisps(dispRefs.as_mut_ptr(), (*globalCtx).state.gfxCtx,
                         b"../z_en_test.c\x00" as *const u8 as
                             *const libc::c_char, 3587 as libc::c_int);
    }
    if (*this).actor.params as libc::c_int ==
           STALFOS_TYPE_INVISIBLE as libc::c_int &&
           !((*this).actor.flags &
                 ((1 as libc::c_int) << 7 as libc::c_int) as libc::c_uint ==
                 ((1 as libc::c_int) << 7 as libc::c_int) as libc::c_uint) {
        *dList = 0 as *mut Gfx
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn EnTest_PostLimbDraw(mut globalCtx:
                                                 *mut GlobalContext,
                                             mut limbIndex: s32,
                                             mut dList: *mut *mut Gfx,
                                             mut rot: *mut Vec3s,
                                             mut thisx: *mut libc::c_void) {
    static mut unused1: Vec3f =
        {
            let mut init = Vec3f{x: 1100.0f32, y: -700.0f32, z: 0.0f32,};
            init
        };
    static mut D_80864658: Vec3f =
        { let mut init = Vec3f{x: 300.0f32, y: 0.0f32, z: 0.0f32,}; init };
    static mut D_80864664: Vec3f =
        { let mut init = Vec3f{x: 3400.0f32, y: 0.0f32, z: 0.0f32,}; init };
    static mut D_80864670: Vec3f =
        { let mut init = Vec3f{x: 0.0f32, y: 0.0f32, z: 0.0f32,}; init };
    static mut D_8086467C: Vec3f =
        {
            let mut init = Vec3f{x: 7000.0f32, y: 1000.0f32, z: 0.0f32,};
            init
        };
    static mut D_80864688: Vec3f =
        {
            let mut init = Vec3f{x: 3000.0f32, y: -2000.0f32, z: -1000.0f32,};
            init
        };
    static mut D_80864694: Vec3f =
        {
            let mut init = Vec3f{x: 3000.0f32, y: -2000.0f32, z: 1000.0f32,};
            init
        };
    static mut D_808646A0: Vec3f =
        {
            let mut init = Vec3f{x: -1300.0f32, y: 1100.0f32, z: 0.0f32,};
            init
        };
    static mut unused2: Vec3f =
        {
            let mut init = Vec3f{x: -3000.0f32, y: 1900.0f32, z: 800.0f32,};
            init
        };
    static mut unused3: Vec3f =
        {
            let mut init = Vec3f{x: -3000.0f32, y: -1100.0f32, z: 800.0f32,};
            init
        };
    static mut unused4: Vec3f =
        {
            let mut init = Vec3f{x: 1900.0f32, y: 1900.0f32, z: 800.0f32,};
            init
        };
    static mut unused5: Vec3f =
        {
            let mut init = Vec3f{x: -3000.0f32, y: -1100.0f32, z: 800.0f32,};
            init
        };
    static mut unused6: Vec3f =
        {
            let mut init = Vec3f{x: 1900.0f32, y: -1100.0f32, z: 800.0f32,};
            init
        };
    static mut unused7: Vec3f =
        {
            let mut init = Vec3f{x: 1900.0f32, y: 1900.0f32, z: 800.0f32,};
            init
        };
    let mut bodyPart: s32 = -(1 as libc::c_int);
    let mut sp70: Vec3f = Vec3f{x: 0., y: 0., z: 0.,};
    let mut sp64: Vec3f = Vec3f{x: 0., y: 0., z: 0.,};
    let mut this: *mut EnTest = thisx as *mut EnTest;
    let mut pad: s32 = 0;
    let mut sp50: Vec3f = Vec3f{x: 0., y: 0., z: 0.,};
    BodyBreak_SetInfo(&mut (*this).bodyBreak, limbIndex, 0 as libc::c_int,
                      60 as libc::c_int, 60 as libc::c_int as u32_0, dList,
                      -(1 as libc::c_int) as s16);
    if limbIndex == STALFOS_LIMB_SWORD as libc::c_int {
        Matrix_MultVec3f(&mut D_8086467C,
                         &mut *(*this).swordCollider.dim.quad.as_mut_ptr().offset(1
                                                                                      as
                                                                                      libc::c_int
                                                                                      as
                                                                                      isize));
        Matrix_MultVec3f(&mut D_80864688,
                         &mut *(*this).swordCollider.dim.quad.as_mut_ptr().offset(0
                                                                                      as
                                                                                      libc::c_int
                                                                                      as
                                                                                      isize));
        Matrix_MultVec3f(&mut D_80864694,
                         &mut *(*this).swordCollider.dim.quad.as_mut_ptr().offset(3
                                                                                      as
                                                                                      libc::c_int
                                                                                      as
                                                                                      isize));
        Matrix_MultVec3f(&mut D_808646A0,
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
        Matrix_MultVec3f(&mut D_80864664, &mut sp70);
        Matrix_MultVec3f(&mut D_80864670, &mut sp64);
        if (*this).swordState as libc::c_int >= 1 as libc::c_int &&
               ((*this).actor.params as libc::c_int !=
                    STALFOS_TYPE_INVISIBLE as libc::c_int ||
                    (*globalCtx).actorCtx.unk_03 as libc::c_int !=
                        0 as libc::c_int) {
            EffectBlure_AddVertex(Effect_GetByIndex((*this).effectIndex) as
                                      *mut EffectBlure, &mut sp70, &mut sp64);
        } else if (*this).swordState as libc::c_int >= 0 as libc::c_int {
            EffectBlure_AddSpace(Effect_GetByIndex((*this).effectIndex) as
                                     *mut EffectBlure);
            (*this).swordState = -(1 as libc::c_int) as s8
        }
    } else if limbIndex == STALFOS_LIMB_SHIELD as libc::c_int &&
                  (*this).unk_7DE as libc::c_int != 0 as libc::c_int {
        Matrix_MultVec3f(&mut D_80864670, &mut sp64);
        (*this).shieldCollider.dim.pos.x = sp64.x as s16;
        (*this).shieldCollider.dim.pos.y = sp64.y as s16;
        (*this).shieldCollider.dim.pos.z = sp64.z as s16
    } else {
        Actor_SetFeetPos(&mut (*this).actor, limbIndex,
                         STALFOS_LIMB_FOOT_L as libc::c_int, &mut D_80864658,
                         STALFOS_LIMB_ANKLE_R as libc::c_int,
                         &mut D_80864658);
        if limbIndex == STALFOS_LIMB_FOOT_L as libc::c_int ||
               limbIndex == STALFOS_LIMB_ANKLE_R as libc::c_int {
            if (*this).unk_7C8 as libc::c_int == 0x15 as libc::c_int ||
                   (*this).unk_7C8 as libc::c_int == 0x16 as libc::c_int {
                if (*this).actor.speedXZ != 0.0f32 {
                    Matrix_MultVec3f(&mut D_80864658, &mut sp64);
                    Actor_SpawnFloorDustRing(globalCtx, &mut (*this).actor,
                                             &mut sp64, 10.0f32,
                                             1 as libc::c_int, 8.0f32,
                                             0x64 as libc::c_int as s16,
                                             0xf as libc::c_int as s16,
                                             0 as libc::c_int as u8_0);
                }
            }
        }
    }
    if (*this).iceTimer as libc::c_int != 0 as libc::c_int {
        match limbIndex {
            11 => { bodyPart = 0 as libc::c_int }
            15 => { bodyPart = 1 as libc::c_int }
            34 => { bodyPart = 2 as libc::c_int }
            27 => { bodyPart = 3 as libc::c_int }
            37 => { bodyPart = 4 as libc::c_int }
            29 => { bodyPart = 5 as libc::c_int }
            60 => { bodyPart = 6 as libc::c_int }
            48 => { bodyPart = 7 as libc::c_int }
            57 => { bodyPart = 8 as libc::c_int }
            _ => { }
        }
        if bodyPart >= 0 as libc::c_int {
            Matrix_MultVec3f(&mut D_80864670, &mut sp50);
            (*this).bodyPartsPos[bodyPart as usize].x = sp50.x as s16;
            (*this).bodyPartsPos[bodyPart as usize].y = sp50.y as s16;
            (*this).bodyPartsPos[bodyPart as usize].z = sp50.z as s16
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn EnTest_Draw(mut thisx: *mut Actor,
                                     mut globalCtx: *mut GlobalContext) {
    let mut this: *mut EnTest = thisx as *mut EnTest;
    func_80093D18((*globalCtx).state.gfxCtx);
    func_8002EBCC(&mut (*this).actor, globalCtx, 1 as libc::c_int);
    if (*thisx).params as libc::c_int <= STALFOS_TYPE_CEILING as libc::c_int
           || (*thisx).child.is_null() {
        SkelAnime_DrawOpa(globalCtx, (*this).skelAnime.skeleton,
                          (*this).skelAnime.jointTable,
                          Some(EnTest_OverrideLimbDraw as
                                   unsafe extern "C" fn(_: *mut GlobalContext,
                                                        _: s32,
                                                        _: *mut *mut Gfx,
                                                        _: *mut Vec3f,
                                                        _: *mut Vec3s,
                                                        _: *mut libc::c_void)
                                       -> s32),
                          Some(EnTest_PostLimbDraw as
                                   unsafe extern "C" fn(_: *mut GlobalContext,
                                                        _: s32,
                                                        _: *mut *mut Gfx,
                                                        _: *mut Vec3s,
                                                        _: *mut libc::c_void)
                                       -> ()), this as *mut libc::c_void);
    }
    if (*this).iceTimer as libc::c_int != 0 as libc::c_int {
        (*thisx).colorFilterTimer = (*thisx).colorFilterTimer.wrapping_add(1);
        (*this).iceTimer -= 1;
        if (*this).iceTimer as libc::c_int % 4 as libc::c_int ==
               0 as libc::c_int {
            let mut iceIndex: s32 =
                (*this).iceTimer as libc::c_int >> 2 as libc::c_int;
            EffectSsEnIce_SpawnFlyingVec3s(globalCtx, thisx,
                                           &mut *(*this).bodyPartsPos.as_mut_ptr().offset(iceIndex
                                                                                              as
                                                                                              isize),
                                           150 as libc::c_int as s16,
                                           150 as libc::c_int as s16,
                                           150 as libc::c_int as s16,
                                           250 as libc::c_int as s16,
                                           235 as libc::c_int as s16,
                                           245 as libc::c_int as s16,
                                           255 as libc::c_int as s16, 1.5f32);
        }
    };
}
// a variation of sidestep
#[no_mangle]
pub unsafe extern "C" fn func_80864158(mut this: *mut EnTest,
                                       mut xzSpeed: f32_0) {
    Animation_MorphToLoop(&mut (*this).skelAnime, &mut gStalfosSidestepAnim,
                          -2.0f32);
    (*this).actor.speedXZ = xzSpeed;
    (*this).actor.world.rot.y =
        ((*this).actor.shape.rot.y as libc::c_int + 0x3fff as libc::c_int) as
            s16;
    (*this).timer = (Rand_ZeroOne() * 20.0f32 + 15.0f32) as s32;
    (*this).unk_7C8 = 0x18 as libc::c_int as u8_0;
    EnTest_SetupAction(this,
                       Some(func_808628C8 as
                                unsafe extern "C" fn(_: *mut EnTest,
                                                     _: *mut GlobalContext)
                                    -> ()));
}
/* *
 * Check if a projectile actor is within 300 units and react accordingly.
 * Returns true if the projectile test passes and a new action is performed.
 */
#[no_mangle]
pub unsafe extern "C" fn EnTest_ReactToProjectile(mut globalCtx:
                                                      *mut GlobalContext,
                                                  mut this: *mut EnTest)
 -> s32 {
    let mut projectileActor: *mut Actor = 0 as *mut Actor;
    let mut yawToProjectile: s16 = 0;
    let mut wallYawDiff: s16 = 0;
    let mut touchingWall: s16 = 0;
    let mut directionFlag: s16 = 0;
    projectileActor =
        Actor_GetProjectileActor(globalCtx, &mut (*this).actor, 300.0f32);
    if !projectileActor.is_null() {
        yawToProjectile =
            (Actor_WorldYawTowardActor(&mut (*this).actor, projectileActor) as
                 libc::c_int -
                 (*this).actor.shape.rot.y as u16_0 as libc::c_int) as s16;
        if ((*this).actor.bgCheckFlags as libc::c_int & 8 as libc::c_int) as
               u8_0 != 0 {
            wallYawDiff =
                ((*this).actor.wallYaw as u16_0 as libc::c_int -
                     (*this).actor.shape.rot.y as u16_0 as libc::c_int) as
                    s16;
            touchingWall = 1 as libc::c_int as s16
        } else { touchingWall = 0 as libc::c_int as s16 }
        if Math_Vec3f_DistXYZ(&mut (*this).actor.world.pos,
                              &mut (*projectileActor).world.pos) < 200.0f32 {
            if Actor_IsTargeted(globalCtx, &mut (*this).actor) != 0 &&
                   (*projectileActor).id as libc::c_int ==
                       ACTOR_ARMS_HOOK as libc::c_int {
                EnTest_SetupJumpUp(this);
            } else if (if yawToProjectile as libc::c_int >= 0 as libc::c_int {
                           yawToProjectile as libc::c_int
                       } else { -(yawToProjectile as libc::c_int) }) <
                          0x2000 as libc::c_int {
                EnTest_SetupStopAndBlock(this);
            } else if (if yawToProjectile as libc::c_int >= 0 as libc::c_int {
                           yawToProjectile as libc::c_int
                       } else { -(yawToProjectile as libc::c_int) }) <
                          0x6000 as libc::c_int {
                EnTest_SetupJumpBack(this);
            } else { EnTest_SetupJumpUp(this); }
            return 1 as libc::c_int
        }
        if Actor_IsTargeted(globalCtx, &mut (*this).actor) != 0 &&
               (*projectileActor).id as libc::c_int ==
                   ACTOR_ARMS_HOOK as libc::c_int {
            EnTest_SetupJumpUp(this);
            return 1 as libc::c_int
        }
        if (if yawToProjectile as libc::c_int >= 0 as libc::c_int {
                yawToProjectile as libc::c_int
            } else { -(yawToProjectile as libc::c_int) }) <
               0x2000 as libc::c_int ||
               (if yawToProjectile as libc::c_int >= 0 as libc::c_int {
                    yawToProjectile as libc::c_int
                } else { -(yawToProjectile as libc::c_int) }) >
                   0x6000 as libc::c_int {
            directionFlag =
                (*globalCtx).gameplayFrames.wrapping_rem(2 as libc::c_int as
                                                             libc::c_uint) as
                    s16;
            if touchingWall as libc::c_int != 0 &&
                   wallYawDiff as libc::c_int > 0x2000 as libc::c_int &&
                   (wallYawDiff as libc::c_int) < 0x6000 as libc::c_int {
                directionFlag = 0 as libc::c_int as s16
            } else if touchingWall as libc::c_int != 0 &&
                          (wallYawDiff as libc::c_int) <
                              -(0x2000 as libc::c_int) &&
                          wallYawDiff as libc::c_int >
                              -(0x6000 as libc::c_int) {
                directionFlag = 1 as libc::c_int as s16
            }
            if directionFlag != 0 {
                func_80864158(this, 4.0f32);
            } else { func_80864158(this, -4.0f32); }
        } else if (if yawToProjectile as libc::c_int >= 0 as libc::c_int {
                       yawToProjectile as libc::c_int
                   } else { -(yawToProjectile as libc::c_int) }) <
                      0x6000 as libc::c_int {
            directionFlag =
                (*globalCtx).gameplayFrames.wrapping_rem(2 as libc::c_int as
                                                             libc::c_uint) as
                    s16;
            if touchingWall as libc::c_int != 0 &&
                   (if wallYawDiff as libc::c_int >= 0 as libc::c_int {
                        wallYawDiff as libc::c_int
                    } else { -(wallYawDiff as libc::c_int) }) >
                       0x6000 as libc::c_int {
                directionFlag = 0 as libc::c_int as s16
            } else if touchingWall as libc::c_int != 0 &&
                          (if wallYawDiff as libc::c_int >= 0 as libc::c_int {
                               wallYawDiff as libc::c_int
                           } else { -(wallYawDiff as libc::c_int) }) <
                              0x2000 as libc::c_int {
                directionFlag = 1 as libc::c_int as s16
            }
            if directionFlag != 0 {
                EnTest_SetupJumpBack(this);
            } else { EnTest_SetupJumpUp(this); }
        }
        return 1 as libc::c_int
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn run_static_initializers() {
    sInitChain =
        [{
             let mut init = InitChainEntry{cont_type_0_offset_value: [0; 4],};
             init.set_cont(1 as libc::c_int as u32_0);
             init.set_type_0(ICHAINTYPE_S8 as libc::c_int as u32_0);
             init.set_offset(&mut (*(0 as *mut Actor)).naviEnemyId as
                                 *mut u8_0 as size_t as u32_0);
             init.set_value(0x1b as libc::c_int);
             init
         },
         {
             let mut init = InitChainEntry{cont_type_0_offset_value: [0; 4],};
             init.set_cont(1 as libc::c_int as u32_0);
             init.set_type_0(ICHAINTYPE_F32 as libc::c_int as u32_0);
             init.set_offset(&mut (*(0 as *mut Actor)).targetArrowOffset as
                                 *mut f32_0 as size_t as u32_0);
             init.set_value(500 as libc::c_int);
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
             init.set_cont(1 as libc::c_int as u32_0);
             init.set_type_0(ICHAINTYPE_F32 as libc::c_int as u32_0);
             init.set_offset(&mut (*(0 as *mut Actor)).scale.y as *mut f32_0
                                 as size_t as u32_0);
             init.set_value(0 as libc::c_int);
             init
         },
         {
             let mut init = InitChainEntry{cont_type_0_offset_value: [0; 4],};
             init.set_cont(0 as libc::c_int as u32_0);
             init.set_type_0(ICHAINTYPE_F32_DIV1000 as libc::c_int as u32_0);
             init.set_offset(&mut (*(0 as *mut Actor)).gravity as *mut f32_0
                                 as size_t as u32_0);
             init.set_value(-(1500 as libc::c_int));
             init
         }]
}
#[used]
#[cfg_attr(target_os = "linux", link_section = ".init_array")]
#[cfg_attr(target_os = "windows", link_section = ".CRT$XIB")]
#[cfg_attr(target_os = "macos", link_section = "__DATA,__mod_init_func")]
static INIT_ARRAY: [unsafe extern "C" fn(); 1] = [run_static_initializers];