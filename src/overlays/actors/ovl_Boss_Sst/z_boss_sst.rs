#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case,
         non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(const_raw_ptr_to_usize_cast, const_transmute, register_tool)]
extern "C" {
    #[no_mangle]
    fn Item_DropCollectible(globalCtx: *mut GlobalContext,
                            spawnPos: *mut Vec3f, params: s16)
     -> *mut EnItem00;
    #[no_mangle]
    fn EffectSsGSplash_Spawn(globalCtx: *mut GlobalContext, pos: *mut Vec3f,
                             primColor: *mut Color_RGBA8,
                             envColor: *mut Color_RGBA8, type_0: s16,
                             scale: s16);
    #[no_mangle]
    fn ActorShape_Init(shape: *mut ActorShape, yOffset: f32_0,
                       shadowDraw: ActorShadowFunc, shadowScale: f32_0);
    #[no_mangle]
    fn ActorShadow_DrawCircle(actor: *mut Actor, lights: *mut Lights,
                              globalCtx: *mut GlobalContext);
    #[no_mangle]
    fn Flags_SetSwitch(globalCtx: *mut GlobalContext, flag: s32);
    #[no_mangle]
    fn Flags_GetClear(globalCtx: *mut GlobalContext, flag: s32) -> s32;
    #[no_mangle]
    fn Flags_SetClear(globalCtx: *mut GlobalContext, flag: s32);
    #[no_mangle]
    fn TitleCard_InitBossName(globalCtx: *mut GlobalContext,
                              titleCtx: *mut TitleCardContext,
                              texture: *mut libc::c_void, x: s16, y: s16,
                              width: u8_0, height: u8_0);
    #[no_mangle]
    fn Actor_Kill(actor: *mut Actor);
    #[no_mangle]
    fn Actor_SetFocus(actor: *mut Actor, offset: f32_0);
    #[no_mangle]
    fn Actor_WorldYawTowardPoint(actor: *mut Actor, refPoint: *mut Vec3f)
     -> s16;
    #[no_mangle]
    fn Actor_WorldDistXZToPoint(actor: *mut Actor, refPoint: *mut Vec3f)
     -> f32_0;
    #[no_mangle]
    fn func_8002DBD0(actor: *mut Actor, result: *mut Vec3f, arg2: *mut Vec3f);
    #[no_mangle]
    fn func_8002DF54(globalCtx: *mut GlobalContext, actor: *mut Actor,
                     arg2: u8_0) -> s32;
    #[no_mangle]
    fn Actor_UpdateBgCheckInfo(globalCtx: *mut GlobalContext,
                               actor: *mut Actor, wallCheckHeight: f32_0,
                               wallCheckRadius: f32_0,
                               ceilingCheckHeight: f32_0, flags: s32);
    #[no_mangle]
    fn func_8002F71C(globalCtx: *mut GlobalContext, actor: *mut Actor,
                     arg2: f32_0, arg3: s16, arg4: f32_0);
    #[no_mangle]
    fn func_8002F7DC(actor: *mut Actor, sfxId: u16_0);
    #[no_mangle]
    fn Audio_PlayActorSound2(actor: *mut Actor, sfxId: u16_0);
    #[no_mangle]
    fn func_8002F974(actor: *mut Actor, sfxId: u16_0);
    #[no_mangle]
    fn Actor_Spawn(actorCtx: *mut ActorContext, globalCtx: *mut GlobalContext,
                   actorId: s16, posX: f32_0, posY: f32_0, posZ: f32_0,
                   rotX: s16, rotY: s16, rotZ: s16, params: s16)
     -> *mut Actor;
    #[no_mangle]
    fn Enemy_StartFinishingBlow(globalCtx: *mut GlobalContext,
                                actor: *mut Actor);
    #[no_mangle]
    fn Actor_ChangeCategory(globalCtx: *mut GlobalContext,
                            actorCtx: *mut ActorContext, actor: *mut Actor,
                            actorCategory: u8_0);
    #[no_mangle]
    fn Rand_CenteredFloat(f: f32_0) -> f32_0;
    #[no_mangle]
    fn Actor_SetColorFilter(actor: *mut Actor, colorFlag: s16,
                            colorIntensityMax: s16, xluFlag: s16,
                            duration: s16);
    #[no_mangle]
    fn func_8003435C(object: *mut Vec3f, globalCtx: *mut GlobalContext)
     -> *mut Hilite;
    #[no_mangle]
    fn Actor_ApplyDamage(actor: *mut Actor) -> u8_0;
    #[no_mangle]
    fn Collider_InitJntSph(globalCtx: *mut GlobalContext,
                           collider: *mut ColliderJntSph) -> s32;
    #[no_mangle]
    fn Collider_DestroyJntSph(globalCtx: *mut GlobalContext,
                              collider: *mut ColliderJntSph) -> s32;
    #[no_mangle]
    fn Collider_SetJntSph(globalCtx: *mut GlobalContext,
                          dest: *mut ColliderJntSph, actor: *mut Actor,
                          src: *mut ColliderJntSphInit,
                          elements: *mut ColliderJntSphElement) -> s32;
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
    fn CollisionCheck_SetInfo(info: *mut CollisionCheckInfo,
                              damageTable: *mut DamageTable,
                              init: *mut CollisionCheckInfoInit);
    #[no_mangle]
    fn Collider_UpdateCylinder(actor: *mut Actor,
                               collider: *mut ColliderCylinder);
    #[no_mangle]
    fn Collider_UpdateSpheres(limb: s32, collider: *mut ColliderJntSph);
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
    fn Math_ScaledStepToS(pValue: *mut s16, target: s16, step: s16) -> s32;
    #[no_mangle]
    fn Math_StepToS(pValue: *mut s16, target: s16, step: s16) -> s32;
    #[no_mangle]
    fn Math_StepToF(pValue: *mut f32_0, target: f32_0, step: f32_0) -> s32;
    #[no_mangle]
    fn Math_Vec3f_Copy(dest: *mut Vec3f, src: *mut Vec3f);
    #[no_mangle]
    fn Actor_ProcessInitChain(actor: *mut Actor,
                              initChain: *mut InitChainEntry);
    #[no_mangle]
    fn Math_SmoothStepToF(pValue: *mut f32_0, target: f32_0, fraction: f32_0,
                          step: f32_0, minStep: f32_0) -> f32_0;
    #[no_mangle]
    fn Math_ApproachF(pValue: *mut f32_0, target: f32_0, fraction: f32_0,
                      step: f32_0);
    #[no_mangle]
    fn Math_SmoothStepToS(pValue: *mut s16, target: s16, scale: s16,
                          step: s16, minStep: s16) -> s16;
    #[no_mangle]
    fn Math_ApproachS(pValue: *mut s16, target: s16, scale: s16, step: s16);
    #[no_mangle]
    fn func_80078914(arg0: *mut Vec3f, sfxId: u16_0);
    #[no_mangle]
    fn func_80093D18(gfxCtx: *mut GraphicsContext);
    #[no_mangle]
    fn func_80093D84(gfxCtx: *mut GraphicsContext);
    #[no_mangle]
    fn Gfx_TwoTexScroll(gfxCtx: *mut GraphicsContext, tile1: s32, x1: u32_0,
                        y1: u32_0, width1: s32, height1: s32, tile2: s32,
                        x2: u32_0, y2: u32_0, width2: s32, height2: s32)
     -> *mut Gfx;
    #[no_mangle]
    fn SkelAnime_DrawFlexOpa(globalCtx: *mut GlobalContext,
                             skeleton: *mut *mut libc::c_void,
                             jointTable: *mut Vec3s, dListCount: s32,
                             overrideLimbDraw: OverrideLimbDrawOpa,
                             postLimbDraw: PostLimbDrawOpa,
                             arg: *mut libc::c_void);
    #[no_mangle]
    fn Animation_GetLastFrame(animation: *mut libc::c_void) -> s16;
    #[no_mangle]
    fn SkelAnime_DrawFlex(globalCtx: *mut GlobalContext,
                          skeleton: *mut *mut libc::c_void,
                          jointTable: *mut Vec3s, dListCount: s32,
                          overrideLimbDraw: OverrideLimbDraw,
                          postLimbDraw: PostLimbDraw, arg: *mut libc::c_void,
                          gfx: *mut Gfx) -> *mut Gfx;
    #[no_mangle]
    fn SkelAnime_InitFlex(globalCtx: *mut GlobalContext,
                          skelAnime: *mut SkelAnime,
                          skeletonHeaderSeg: *mut FlexSkeletonHeader,
                          animation: *mut AnimationHeader,
                          jointTable: *mut Vec3s, morphTable: *mut Vec3s,
                          limbCount: s32) -> s32;
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
    fn Animation_MorphToLoop(skelAnime: *mut SkelAnime,
                             animation: *mut AnimationHeader,
                             morphFrames: f32_0);
    #[no_mangle]
    fn Animation_OnFrame(skelAnime: *mut SkelAnime, frame: f32_0) -> s32;
    #[no_mangle]
    fn SkinMatrix_Vec3fMtxFMultXYZ(mf: *mut MtxF, src: *mut Vec3f,
                                   dest: *mut Vec3f);
    #[no_mangle]
    fn func_800AA000(_: f32_0, _: u8_0, _: u8_0, _: u8_0);
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
    fn Gameplay_CopyCamera(globalCtx: *mut GlobalContext, camId1: s16,
                           camId2: s16);
    #[no_mangle]
    fn Graph_OpenDisps(dispRefs: *mut *mut Gfx, gfxCtx: *mut GraphicsContext,
                       file: *const libc::c_char, line: s32);
    #[no_mangle]
    fn Graph_CloseDisps(dispRefs: *mut *mut Gfx, gfxCtx: *mut GraphicsContext,
                        file: *const libc::c_char, line: s32);
    #[no_mangle]
    fn Math3D_Vec3fDistSq(a: *mut Vec3f, b: *mut Vec3f) -> f32_0;
    #[no_mangle]
    fn Matrix_Translate(x: f32_0, y: f32_0, z: f32_0, mode: u8_0);
    #[no_mangle]
    fn Matrix_Scale(x: f32_0, y: f32_0, z: f32_0, mode: u8_0);
    #[no_mangle]
    fn Matrix_RotateX(x: f32_0, mode: u8_0);
    #[no_mangle]
    fn Matrix_RotateY(y: f32_0, mode: u8_0);
    #[no_mangle]
    fn Matrix_RotateZYX(x: s16, y: s16, z: s16, mode: u8_0);
    #[no_mangle]
    fn func_800D1694(x: f32_0, y: f32_0, z: f32_0, vec: *mut Vec3s);
    #[no_mangle]
    fn Matrix_NewMtx(gfxCtx: *mut GraphicsContext, file: *mut libc::c_char,
                     line: s32) -> *mut Mtx;
    #[no_mangle]
    fn Matrix_MultVec3f(src: *mut Vec3f, dest: *mut Vec3f);
    #[no_mangle]
    fn Matrix_MultVec3fExt(src: *mut Vec3f, dest: *mut Vec3f, mf: *mut MtxF);
    #[no_mangle]
    fn Audio_StopSfxByPos(_: *mut Vec3f);
    #[no_mangle]
    fn Audio_QueueSeqCmd(bgmID: u32_0);
    #[no_mangle]
    fn Rand_ZeroOne() -> f32_0;
    #[no_mangle]
    fn sinf(_: f32_0) -> f32_0;
    #[no_mangle]
    static mut D_80116280: [Gfx; 0];
    #[no_mangle]
    static mut gSaveContext: SaveContext;
    #[no_mangle]
    static mut gSegments: [u32_0; 16];
    #[no_mangle]
    static mut gBongoLeftHandIdleAnim: AnimationHeader;
    #[no_mangle]
    static mut gBongoLeftHandSkel: FlexSkeletonHeader;
    #[no_mangle]
    static mut gBongoLeftHandFlatPoseAnim: AnimationHeader;
    #[no_mangle]
    static mut gBongoLeftHandOpenPoseAnim: AnimationHeader;
    #[no_mangle]
    static mut gBongoLeftHandFistPoseAnim: AnimationHeader;
    #[no_mangle]
    static mut gBongoLeftHandClenchAnim: AnimationHeader;
    #[no_mangle]
    static mut gBongoLeftHandDamagePoseAnim: AnimationHeader;
    #[no_mangle]
    static mut gBongoLeftHandPushoffPoseAnim: AnimationHeader;
    #[no_mangle]
    static mut gBongoLeftHandHangPoseAnim: AnimationHeader;
    #[no_mangle]
    static mut gBongoRightHandIdleAnim: AnimationHeader;
    #[no_mangle]
    static mut gBongoRightHandSkel: FlexSkeletonHeader;
    #[no_mangle]
    static mut gBongoRightHandFlatPoseAnim: AnimationHeader;
    #[no_mangle]
    static mut gBongoRightHandOpenPoseAnim: AnimationHeader;
    #[no_mangle]
    static mut gBongoRightHandFistPoseAnim: AnimationHeader;
    #[no_mangle]
    static mut gBongoRightHandClenchAnim: AnimationHeader;
    #[no_mangle]
    static mut gBongoRightHandDamagePoseAnim: AnimationHeader;
    #[no_mangle]
    static mut gBongoRightHandPushoffPoseAnim: AnimationHeader;
    #[no_mangle]
    static mut gBongoRightHandHangPoseAnim: AnimationHeader;
    #[no_mangle]
    static mut gBongoHeadStunnedAnim: AnimationHeader;
    #[no_mangle]
    static mut gBongoHeadChargeAnim: AnimationHeader;
    #[no_mangle]
    static mut gBongoHeadKnockoutAnim: AnimationHeader;
    #[no_mangle]
    static mut gBongoHeadEyeCloseAnim: AnimationHeader;
    #[no_mangle]
    static mut gBongoHeadDamagedHandAnim: AnimationHeader;
    #[no_mangle]
    static mut gBongoHeadEyeOpenAnim: AnimationHeader;
    #[no_mangle]
    static mut gBongoHeadDamageAnim: AnimationHeader;
    #[no_mangle]
    static mut gBongoHeadRecoverAnim: AnimationHeader;
    #[no_mangle]
    static mut gBongoHeadEyeCloseIdleAnim: AnimationHeader;
    #[no_mangle]
    static mut gBongoHeadEyeOpenIdleAnim: AnimationHeader;
    #[no_mangle]
    static mut gBongoTitleCardTex: [u64_0; 0];
    #[no_mangle]
    static mut gBongoHeadSkel: FlexSkeletonHeader;
    #[no_mangle]
    static mut gBongoIceCrystalDL: [Gfx; 0];
    #[no_mangle]
    static mut gBongoIceShardDL: [Gfx; 0];
    #[no_mangle]
    static mut gEffFireCircleDL: [Gfx; 0];
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
pub struct Hilite_t {
    pub x1: libc::c_int,
    pub y1: libc::c_int,
    pub x2: libc::c_int,
    pub y2: libc::c_int,
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
#[derive(Copy, Clone)]
#[repr(C)]
pub union Hilite {
    pub h: Hilite_t,
    pub force_structure_alignment: [libc::c_long; 4],
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CollisionCheckInfoInit {
    pub health: u8_0,
    pub cylRadius: s16,
    pub cylHeight: s16,
    pub mass: u8_0,
}
pub type C2RustUnnamed_18 = libc::c_uint;
pub const ITEM00_BOMBS_SPECIAL: C2RustUnnamed_18 = 25;
pub const ITEM00_TUNIC_GORON: C2RustUnnamed_18 = 24;
pub const ITEM00_TUNIC_ZORA: C2RustUnnamed_18 = 23;
pub const ITEM00_SHIELD_HYLIAN: C2RustUnnamed_18 = 22;
pub const ITEM00_SHIELD_DEKU: C2RustUnnamed_18 = 21;
pub const ITEM00_RUPEE_PURPLE: C2RustUnnamed_18 = 20;
pub const ITEM00_RUPEE_ORANGE: C2RustUnnamed_18 = 19;
pub const ITEM00_FLEXIBLE: C2RustUnnamed_18 = 18;
pub const ITEM00_SMALL_KEY: C2RustUnnamed_18 = 17;
pub const ITEM00_SEEDS: C2RustUnnamed_18 = 16;
pub const ITEM00_MAGIC_SMALL: C2RustUnnamed_18 = 15;
pub const ITEM00_MAGIC_LARGE: C2RustUnnamed_18 = 14;
pub const ITEM00_STICK: C2RustUnnamed_18 = 13;
pub const ITEM00_NUTS: C2RustUnnamed_18 = 12;
pub const ITEM00_BOMBS_B: C2RustUnnamed_18 = 11;
pub const ITEM00_ARROWS_LARGE: C2RustUnnamed_18 = 10;
pub const ITEM00_ARROWS_MEDIUM: C2RustUnnamed_18 = 9;
pub const ITEM00_ARROWS_SMALL: C2RustUnnamed_18 = 8;
pub const ITEM00_HEART_CONTAINER: C2RustUnnamed_18 = 7;
pub const ITEM00_HEART_PIECE: C2RustUnnamed_18 = 6;
pub const ITEM00_ARROWS_SINGLE: C2RustUnnamed_18 = 5;
pub const ITEM00_BOMBS_A: C2RustUnnamed_18 = 4;
pub const ITEM00_HEART: C2RustUnnamed_18 = 3;
pub const ITEM00_RUPEE_RED: C2RustUnnamed_18 = 2;
pub const ITEM00_RUPEE_BLUE: C2RustUnnamed_18 = 1;
pub const ITEM00_RUPEE_GREEN: C2RustUnnamed_18 = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct EnItem00 {
    pub actor: Actor,
    pub actionFunc: EnItem00ActionFunc,
    pub collectibleFlag: s16,
    pub getItemId: s16,
    pub unk_154: s16,
    pub unk_156: s16,
    pub unk_158: s16,
    pub unk_15A: s16,
    pub scale: f32_0,
    pub collider: ColliderCylinder,
}
pub type EnItem00ActionFunc
    =
    Option<unsafe extern "C" fn(_: *mut EnItem00, _: *mut GlobalContext)
               -> ()>;
pub type C2RustUnnamed_19 = libc::c_uint;
pub const ACTORCAT_CHEST: C2RustUnnamed_19 = 11;
pub const ACTORCAT_DOOR: C2RustUnnamed_19 = 10;
pub const ACTORCAT_BOSS: C2RustUnnamed_19 = 9;
pub const ACTORCAT_MISC: C2RustUnnamed_19 = 8;
pub const ACTORCAT_ITEMACTION: C2RustUnnamed_19 = 7;
pub const ACTORCAT_PROP: C2RustUnnamed_19 = 6;
pub const ACTORCAT_ENEMY: C2RustUnnamed_19 = 5;
pub const ACTORCAT_NPC: C2RustUnnamed_19 = 4;
pub const ACTORCAT_EXPLOSIVE: C2RustUnnamed_19 = 3;
pub const ACTORCAT_PLAYER: C2RustUnnamed_19 = 2;
pub const ACTORCAT_BG: C2RustUnnamed_19 = 1;
pub const ACTORCAT_SWITCH: C2RustUnnamed_19 = 0;
pub type C2RustUnnamed_20 = libc::c_uint;
pub const ACTOR_ID_MAX: C2RustUnnamed_20 = 471;
pub const ACTOR_OBJ_WARP2BLOCK: C2RustUnnamed_20 = 470;
pub const ACTOR_BG_JYA_BLOCK: C2RustUnnamed_20 = 469;
pub const ACTOR_EN_MM2: C2RustUnnamed_20 = 468;
pub const ACTOR_EN_ZL4: C2RustUnnamed_20 = 467;
pub const ACTOR_OBJ_HAMISHI: C2RustUnnamed_20 = 466;
pub const ACTOR_OBJ_TIMEBLOCK: C2RustUnnamed_20 = 465;
pub const ACTOR_EN_GE3: C2RustUnnamed_20 = 464;
pub const ACTOR_OBJ_MAKEKINSUTA: C2RustUnnamed_20 = 463;
pub const ACTOR_EN_ZO: C2RustUnnamed_20 = 462;
pub const ACTOR_BG_MENKURI_NISEKABE: C2RustUnnamed_20 = 461;
pub const ACTOR_EN_EG: C2RustUnnamed_20 = 460;
pub const ACTOR_OCEFF_WIPE4: C2RustUnnamed_20 = 459;
pub const ACTOR_EN_KAKASI3: C2RustUnnamed_20 = 458;
pub const ACTOR_EN_KAKASI2: C2RustUnnamed_20 = 457;
pub const ACTOR_BG_ICE_SHUTTER: C2RustUnnamed_20 = 456;
pub const ACTOR_BG_ICE_TURARA: C2RustUnnamed_20 = 455;
pub const ACTOR_EN_COW: C2RustUnnamed_20 = 454;
pub const ACTOR_EN_MA3: C2RustUnnamed_20 = 453;
pub const ACTOR_BG_SPOT18_SHUTTER: C2RustUnnamed_20 = 452;
pub const ACTOR_BG_SPOT18_FUTA: C2RustUnnamed_20 = 451;
pub const ACTOR_BG_SPOT11_OASIS: C2RustUnnamed_20 = 450;
pub const ACTOR_DOOR_KILLER: C2RustUnnamed_20 = 449;
pub const ACTOR_EN_CROW: C2RustUnnamed_20 = 448;
pub const ACTOR_EN_PO_DESERT: C2RustUnnamed_20 = 447;
pub const ACTOR_EN_WALL_TUBO: C2RustUnnamed_20 = 446;
pub const ACTOR_BG_BOWL_WALL: C2RustUnnamed_20 = 445;
pub const ACTOR_EN_DAIKU_KAKARIKO: C2RustUnnamed_20 = 444;
pub const ACTOR_BG_MIZU_SHUTTER: C2RustUnnamed_20 = 443;
pub const ACTOR_BG_MIZU_BWALL: C2RustUnnamed_20 = 442;
pub const ACTOR_EN_GS: C2RustUnnamed_20 = 441;
pub const ACTOR_EN_GB: C2RustUnnamed_20 = 440;
pub const ACTOR_BG_GND_ICEBLOCK: C2RustUnnamed_20 = 439;
pub const ACTOR_BG_GND_NISEKABE: C2RustUnnamed_20 = 438;
pub const ACTOR_BG_GND_SOULMEIRO: C2RustUnnamed_20 = 437;
pub const ACTOR_BG_GND_DARKMEIRO: C2RustUnnamed_20 = 436;
pub const ACTOR_BG_GND_FIREMEIRO: C2RustUnnamed_20 = 435;
pub const ACTOR_DEMO_GEFF: C2RustUnnamed_20 = 434;
pub const ACTOR_DEMO_GJ: C2RustUnnamed_20 = 433;
pub const ACTOR_EN_SKB: C2RustUnnamed_20 = 432;
pub const ACTOR_EN_WF: C2RustUnnamed_20 = 431;
pub const ACTOR_EN_GO2: C2RustUnnamed_20 = 430;
pub const ACTOR_EN_MU: C2RustUnnamed_20 = 429;
pub const ACTOR_EN_TG: C2RustUnnamed_20 = 428;
pub const ACTOR_OBJ_MURE3: C2RustUnnamed_20 = 427;
pub const ACTOR_UNSET_1AA: C2RustUnnamed_20 = 426;
pub const ACTOR_BG_SPOT17_BAKUDANKABE: C2RustUnnamed_20 = 425;
pub const ACTOR_BG_SPOT08_BAKUDANKABE: C2RustUnnamed_20 = 424;
pub const ACTOR_DEMO_KEKKAI: C2RustUnnamed_20 = 423;
pub const ACTOR_EN_HS2: C2RustUnnamed_20 = 422;
pub const ACTOR_BG_BOM_GUARD: C2RustUnnamed_20 = 421;
pub const ACTOR_EN_GUEST: C2RustUnnamed_20 = 420;
pub const ACTOR_EN_DNT_NOMAL: C2RustUnnamed_20 = 419;
pub const ACTOR_EN_DNT_JIJI: C2RustUnnamed_20 = 418;
pub const ACTOR_EN_DNT_DEMO: C2RustUnnamed_20 = 417;
pub const ACTOR_OBJ_KIBAKO2: C2RustUnnamed_20 = 416;
pub const ACTOR_BG_SPOT11_BAKUDANKABE: C2RustUnnamed_20 = 415;
pub const ACTOR_OBJ_COMB: C2RustUnnamed_20 = 414;
pub const ACTOR_BG_SPOT01_OBJECTS2: C2RustUnnamed_20 = 413;
pub const ACTOR_EN_SI: C2RustUnnamed_20 = 412;
pub const ACTOR_EN_DOG: C2RustUnnamed_20 = 411;
pub const ACTOR_EN_NIW_GIRL: C2RustUnnamed_20 = 410;
pub const ACTOR_OCEFF_WIPE3: C2RustUnnamed_20 = 409;
pub const ACTOR_OCEFF_WIPE2: C2RustUnnamed_20 = 408;
pub const ACTOR_EN_GELDB: C2RustUnnamed_20 = 407;
pub const ACTOR_EN_IT: C2RustUnnamed_20 = 406;
pub const ACTOR_EN_SHOPNUTS: C2RustUnnamed_20 = 405;
pub const ACTOR_BG_SPOT00_BREAK: C2RustUnnamed_20 = 404;
pub const ACTOR_EN_NUTSBALL: C2RustUnnamed_20 = 403;
pub const ACTOR_EN_HINTNUTS: C2RustUnnamed_20 = 402;
pub const ACTOR_BG_SPOT12_SAKU: C2RustUnnamed_20 = 401;
pub const ACTOR_BG_SPOT12_GATE: C2RustUnnamed_20 = 400;
pub const ACTOR_BG_JYA_HAHENIRON: C2RustUnnamed_20 = 399;
pub const ACTOR_BG_JYA_1FLIFT: C2RustUnnamed_20 = 398;
pub const ACTOR_BG_SPOT05_SOKO: C2RustUnnamed_20 = 397;
pub const ACTOR_EN_WEIYER: C2RustUnnamed_20 = 396;
pub const ACTOR_OCEFF_STORM: C2RustUnnamed_20 = 395;
pub const ACTOR_OCEFF_WIPE: C2RustUnnamed_20 = 394;
pub const ACTOR_EN_STH: C2RustUnnamed_20 = 393;
pub const ACTOR_EN_SSH: C2RustUnnamed_20 = 392;
pub const ACTOR_OBJ_ROOMTIMER: C2RustUnnamed_20 = 391;
pub const ACTOR_EN_GE2: C2RustUnnamed_20 = 390;
pub const ACTOR_EN_WONDER_TALK2: C2RustUnnamed_20 = 389;
pub const ACTOR_EN_DY_EXTRA: C2RustUnnamed_20 = 388;
pub const ACTOR_SHOT_SUN: C2RustUnnamed_20 = 387;
pub const ACTOR_DEMO_EC: C2RustUnnamed_20 = 386;
pub const ACTOR_EN_TORCH: C2RustUnnamed_20 = 385;
pub const ACTOR_UNSET_180: C2RustUnnamed_20 = 384;
pub const ACTOR_END_TITLE: C2RustUnnamed_20 = 383;
pub const ACTOR_OCEFF_SPOT: C2RustUnnamed_20 = 382;
pub const ACTOR_OBJ_MAKEOSHIHIKI: C2RustUnnamed_20 = 381;
pub const ACTOR_EN_TAKARA_MAN: C2RustUnnamed_20 = 380;
pub const ACTOR_EN_KAKASI: C2RustUnnamed_20 = 379;
pub const ACTOR_BOSS_GANON2: C2RustUnnamed_20 = 378;
pub const ACTOR_EN_ZL3: C2RustUnnamed_20 = 377;
pub const ACTOR_EN_HEISHI4: C2RustUnnamed_20 = 376;
pub const ACTOR_BG_ZG: C2RustUnnamed_20 = 375;
pub const ACTOR_EFC_ERUPC: C2RustUnnamed_20 = 374;
pub const ACTOR_EN_PO_FIELD: C2RustUnnamed_20 = 373;
pub const ACTOR_DEMO_GT: C2RustUnnamed_20 = 372;
pub const ACTOR_ELF_MSG2: C2RustUnnamed_20 = 371;
pub const ACTOR_DOOR_GERUDO: C2RustUnnamed_20 = 370;
pub const ACTOR_EN_MAG: C2RustUnnamed_20 = 369;
pub const ACTOR_EN_OKARINA_EFFECT: C2RustUnnamed_20 = 368;
pub const ACTOR_EN_GANON_MANT: C2RustUnnamed_20 = 367;
pub const ACTOR_EN_HY: C2RustUnnamed_20 = 366;
pub const ACTOR_EN_MD: C2RustUnnamed_20 = 365;
pub const ACTOR_EN_CS: C2RustUnnamed_20 = 364;
pub const ACTOR_EN_JSJUTAN: C2RustUnnamed_20 = 363;
pub const ACTOR_EN_JS: C2RustUnnamed_20 = 362;
pub const ACTOR_BG_JYA_IRONOBJ: C2RustUnnamed_20 = 361;
pub const ACTOR_EN_EX_ITEM: C2RustUnnamed_20 = 360;
pub const ACTOR_EN_ANI: C2RustUnnamed_20 = 359;
pub const ACTOR_BG_SST_FLOOR: C2RustUnnamed_20 = 358;
pub const ACTOR_EN_WEATHER_TAG: C2RustUnnamed_20 = 357;
pub const ACTOR_EN_KZ: C2RustUnnamed_20 = 356;
pub const ACTOR_EN_KO: C2RustUnnamed_20 = 355;
pub const ACTOR_EN_MM: C2RustUnnamed_20 = 354;
pub const ACTOR_UNSET_161: C2RustUnnamed_20 = 353;
pub const ACTOR_EN_STREAM: C2RustUnnamed_20 = 352;
pub const ACTOR_EN_SIOFUKI: C2RustUnnamed_20 = 351;
pub const ACTOR_EN_GANON_ORGAN: C2RustUnnamed_20 = 350;
pub const ACTOR_UNSET_15D: C2RustUnnamed_20 = 349;
pub const ACTOR_BG_SPOT18_BASKET: C2RustUnnamed_20 = 348;
pub const ACTOR_BG_JYA_BOMBIWA: C2RustUnnamed_20 = 347;
pub const ACTOR_BG_JYA_AMISHUTTER: C2RustUnnamed_20 = 346;
pub const ACTOR_BG_JYA_BOMBCHUIWA: C2RustUnnamed_20 = 345;
pub const ACTOR_BG_JYA_BIGMIRROR: C2RustUnnamed_20 = 344;
pub const ACTOR_BG_JYA_LIFT: C2RustUnnamed_20 = 343;
pub const ACTOR_BG_JYA_MEGAMI: C2RustUnnamed_20 = 342;
pub const ACTOR_EN_CHANGER: C2RustUnnamed_20 = 341;
pub const ACTOR_UNSET_154: C2RustUnnamed_20 = 340;
pub const ACTOR_EN_FU: C2RustUnnamed_20 = 339;
pub const ACTOR_EN_GO: C2RustUnnamed_20 = 338;
pub const ACTOR_OBJ_MURE2: C2RustUnnamed_20 = 337;
pub const ACTOR_OBJ_LIGHTSWITCH: C2RustUnnamed_20 = 336;
pub const ACTOR_OBJ_HANA: C2RustUnnamed_20 = 335;
pub const ACTOR_EN_ISHI: C2RustUnnamed_20 = 334;
pub const ACTOR_EN_OWL: C2RustUnnamed_20 = 333;
pub const ACTOR_EN_BOM_BOWL_PIT: C2RustUnnamed_20 = 332;
pub const ACTOR_EN_BOM_BOWL_MAN: C2RustUnnamed_20 = 331;
pub const ACTOR_EN_MK: C2RustUnnamed_20 = 330;
pub const ACTOR_EN_DS: C2RustUnnamed_20 = 329;
pub const ACTOR_BG_GJYO_BRIDGE: C2RustUnnamed_20 = 328;
pub const ACTOR_EN_WONDER_TALK: C2RustUnnamed_20 = 327;
pub const ACTOR_EN_SA: C2RustUnnamed_20 = 326;
pub const ACTOR_BG_SPOT01_IDOSOKO: C2RustUnnamed_20 = 325;
pub const ACTOR_EN_ATTACK_NIW: C2RustUnnamed_20 = 324;
pub const ACTOR_EN_SYATEKI_NIW: C2RustUnnamed_20 = 323;
pub const ACTOR_EN_HEISHI3: C2RustUnnamed_20 = 322;
pub const ACTOR_EN_KANBAN: C2RustUnnamed_20 = 321;
pub const ACTOR_BG_INGATE: C2RustUnnamed_20 = 320;
pub const ACTOR_EN_HS: C2RustUnnamed_20 = 319;
pub const ACTOR_EN_MS: C2RustUnnamed_20 = 318;
pub const ACTOR_EN_GM: C2RustUnnamed_20 = 317;
pub const ACTOR_EN_NIW_LADY: C2RustUnnamed_20 = 316;
pub const ACTOR_EN_CLEAR_TAG: C2RustUnnamed_20 = 315;
pub const ACTOR_EN_SDA: C2RustUnnamed_20 = 314;
pub const ACTOR_OBJ_BLOCKSTOP: C2RustUnnamed_20 = 313;
pub const ACTOR_EN_GE1: C2RustUnnamed_20 = 312;
pub const ACTOR_ITEM_INBOX: C2RustUnnamed_20 = 311;
pub const ACTOR_EN_BLKOBJ: C2RustUnnamed_20 = 310;
pub const ACTOR_EN_NWC: C2RustUnnamed_20 = 309;
pub const ACTOR_UNSET_134: C2RustUnnamed_20 = 308;
pub const ACTOR_EN_DAIKU: C2RustUnnamed_20 = 307;
pub const ACTOR_EN_TORYO: C2RustUnnamed_20 = 306;
pub const ACTOR_EN_EX_RUPPY: C2RustUnnamed_20 = 305;
pub const ACTOR_EN_GOROIWA: C2RustUnnamed_20 = 304;
pub const ACTOR_EN_YABUSAME_MARK: C2RustUnnamed_20 = 303;
pub const ACTOR_EN_OKARINA_TAG: C2RustUnnamed_20 = 302;
pub const ACTOR_OBJ_HSBLOCK: C2RustUnnamed_20 = 301;
pub const ACTOR_OBJ_LIFT: C2RustUnnamed_20 = 300;
pub const ACTOR_OBJ_ELEVATOR: C2RustUnnamed_20 = 299;
pub const ACTOR_OBJ_SWITCH: C2RustUnnamed_20 = 298;
pub const ACTOR_UNSET_129: C2RustUnnamed_20 = 297;
pub const ACTOR_UNSET_128: C2RustUnnamed_20 = 296;
pub const ACTOR_OBJ_BOMBIWA: C2RustUnnamed_20 = 295;
pub const ACTOR_OBJ_BEAN: C2RustUnnamed_20 = 294;
pub const ACTOR_EN_KUSA: C2RustUnnamed_20 = 293;
pub const ACTOR_EN_DIVING_GAME: C2RustUnnamed_20 = 292;
pub const ACTOR_BG_RELAY_OBJECTS: C2RustUnnamed_20 = 291;
pub const ACTOR_EN_PO_RELAY: C2RustUnnamed_20 = 290;
pub const ACTOR_EN_FZ: C2RustUnnamed_20 = 289;
pub const ACTOR_BG_SPOT07_TAKI: C2RustUnnamed_20 = 288;
pub const ACTOR_BG_SPOT03_TAKI: C2RustUnnamed_20 = 287;
pub const ACTOR_OBJ_ICE_POLY: C2RustUnnamed_20 = 286;
pub const ACTOR_EN_TUBO_TRAP: C2RustUnnamed_20 = 285;
pub const ACTOR_EN_HONOTRAP: C2RustUnnamed_20 = 284;
pub const ACTOR_ELF_MSG: C2RustUnnamed_20 = 283;
pub const ACTOR_EN_DNS: C2RustUnnamed_20 = 282;
pub const ACTOR_DEMO_SHD: C2RustUnnamed_20 = 281;
pub const ACTOR_DEMO_EXT: C2RustUnnamed_20 = 280;
pub const ACTOR_EN_G_SWITCH: C2RustUnnamed_20 = 279;
pub const ACTOR_EN_SKJNEEDLE: C2RustUnnamed_20 = 278;
pub const ACTOR_EN_SKJ: C2RustUnnamed_20 = 277;
pub const ACTOR_DEMO_IK: C2RustUnnamed_20 = 276;
pub const ACTOR_EN_IK: C2RustUnnamed_20 = 275;
pub const ACTOR_EN_WONDER_ITEM: C2RustUnnamed_20 = 274;
pub const ACTOR_OBJ_TSUBO: C2RustUnnamed_20 = 273;
pub const ACTOR_OBJ_KIBAKO: C2RustUnnamed_20 = 272;
pub const ACTOR_ITEM_ETCETERA: C2RustUnnamed_20 = 271;
pub const ACTOR_UNSET_10E: C2RustUnnamed_20 = 270;
pub const ACTOR_UNSET_10D: C2RustUnnamed_20 = 269;
pub const ACTOR_ARROW_LIGHT: C2RustUnnamed_20 = 268;
pub const ACTOR_ARROW_ICE: C2RustUnnamed_20 = 267;
pub const ACTOR_ARROW_FIRE: C2RustUnnamed_20 = 266;
pub const ACTOR_UNSET_109: C2RustUnnamed_20 = 265;
pub const ACTOR_BG_UMAJUMP: C2RustUnnamed_20 = 264;
pub const ACTOR_BG_SPOT15_RRBOX: C2RustUnnamed_20 = 263;
pub const ACTOR_BG_GANON_OTYUKA: C2RustUnnamed_20 = 262;
pub const ACTOR_BG_PO_SYOKUDAI: C2RustUnnamed_20 = 261;
pub const ACTOR_BG_SPOT01_IDOMIZU: C2RustUnnamed_20 = 260;
pub const ACTOR_BG_SPOT01_IDOHASHIRA: C2RustUnnamed_20 = 259;
pub const ACTOR_BG_SPOT01_FUSYA: C2RustUnnamed_20 = 258;
pub const ACTOR_EFF_DUST: C2RustUnnamed_20 = 257;
pub const ACTOR_BG_GATE_SHUTTER: C2RustUnnamed_20 = 256;
pub const ACTOR_OBJ_OSHIHIKI: C2RustUnnamed_20 = 255;
pub const ACTOR_FISHING: C2RustUnnamed_20 = 254;
pub const ACTOR_BG_JYA_KANAAMI: C2RustUnnamed_20 = 253;
pub const ACTOR_BG_JYA_COBRA: C2RustUnnamed_20 = 252;
pub const ACTOR_UNSET_FB: C2RustUnnamed_20 = 251;
pub const ACTOR_BG_JYA_ZURERUKABE: C2RustUnnamed_20 = 250;
pub const ACTOR_BG_JYA_GOROIWA: C2RustUnnamed_20 = 249;
pub const ACTOR_BG_SPOT15_SAKU: C2RustUnnamed_20 = 248;
pub const ACTOR_BG_HAKA_GATE: C2RustUnnamed_20 = 247;
pub const ACTOR_EN_ANUBICE_TAG: C2RustUnnamed_20 = 246;
pub const ACTOR_DEMO_6K: C2RustUnnamed_20 = 245;
pub const ACTOR_MAGIC_DARK: C2RustUnnamed_20 = 244;
pub const ACTOR_UNSET_F3: C2RustUnnamed_20 = 243;
pub const ACTOR_UNSET_F2: C2RustUnnamed_20 = 242;
pub const ACTOR_ITEM_OCARINA: C2RustUnnamed_20 = 241;
pub const ACTOR_EN_ICE_HONO: C2RustUnnamed_20 = 240;
pub const ACTOR_BG_ICE_SHELTER: C2RustUnnamed_20 = 239;
pub const ACTOR_ITEM_SHIELD: C2RustUnnamed_20 = 238;
pub const ACTOR_EN_FR: C2RustUnnamed_20 = 237;
pub const ACTOR_EN_NY: C2RustUnnamed_20 = 236;
pub const ACTOR_UNSET_EB: C2RustUnnamed_20 = 235;
pub const ACTOR_UNSET_EA: C2RustUnnamed_20 = 234;
pub const ACTOR_BOSS_SST: C2RustUnnamed_20 = 233;
pub const ACTOR_BOSS_GANON: C2RustUnnamed_20 = 232;
pub const ACTOR_EN_MA1: C2RustUnnamed_20 = 231;
pub const ACTOR_BG_BDAN_SWITCH: C2RustUnnamed_20 = 230;
pub const ACTOR_BG_SPOT16_DOUGHNUT: C2RustUnnamed_20 = 229;
pub const ACTOR_BG_MORI_IDOMIZU: C2RustUnnamed_20 = 228;
pub const ACTOR_BG_MORI_HASHIRA4: C2RustUnnamed_20 = 227;
pub const ACTOR_BG_MORI_HASHIGO: C2RustUnnamed_20 = 226;
pub const ACTOR_EN_ANUBICE_FIRE: C2RustUnnamed_20 = 225;
pub const ACTOR_EN_ANUBICE: C2RustUnnamed_20 = 224;
pub const ACTOR_EN_BX: C2RustUnnamed_20 = 223;
pub const ACTOR_EN_BA: C2RustUnnamed_20 = 222;
pub const ACTOR_EN_RR: C2RustUnnamed_20 = 221;
pub const ACTOR_BOSS_TW: C2RustUnnamed_20 = 220;
pub const ACTOR_EN_HORSE_GAME_CHECK: C2RustUnnamed_20 = 219;
pub const ACTOR_EN_BOM_CHU: C2RustUnnamed_20 = 218;
pub const ACTOR_EN_MA2: C2RustUnnamed_20 = 217;
pub const ACTOR_UNSET_D8: C2RustUnnamed_20 = 216;
pub const ACTOR_BG_HAKA_WATER: C2RustUnnamed_20 = 215;
pub const ACTOR_BG_ICE_OBJECTS: C2RustUnnamed_20 = 214;
pub const ACTOR_BG_SPOT06_OBJECTS: C2RustUnnamed_20 = 213;
pub const ACTOR_BG_MIZU_UZU: C2RustUnnamed_20 = 212;
pub const ACTOR_OBJ_DEKUJR: C2RustUnnamed_20 = 211;
pub const ACTOR_EN_RU2: C2RustUnnamed_20 = 210;
pub const ACTOR_BG_SPOT08_ICEBLOCK: C2RustUnnamed_20 = 209;
pub const ACTOR_BG_BOMBWALL: C2RustUnnamed_20 = 208;
pub const ACTOR_BG_HIDAN_KOWARERUKABE: C2RustUnnamed_20 = 207;
pub const ACTOR_UNSET_CE: C2RustUnnamed_20 = 206;
pub const ACTOR_BG_SPOT16_BOMBSTONE: C2RustUnnamed_20 = 205;
pub const ACTOR_EN_TR: C2RustUnnamed_20 = 204;
pub const ACTOR_EN_IN: C2RustUnnamed_20 = 203;
pub const ACTOR_DEMO_GO: C2RustUnnamed_20 = 202;
pub const ACTOR_DEMO_SA: C2RustUnnamed_20 = 201;
pub const ACTOR_BG_BDAN_OBJECTS: C2RustUnnamed_20 = 200;
pub const ACTOR_EN_KAREBABA: C2RustUnnamed_20 = 199;
pub const ACTOR_EN_BIGOKUTA: C2RustUnnamed_20 = 198;
pub const ACTOR_EN_SB: C2RustUnnamed_20 = 197;
pub const ACTOR_BOSS_MO: C2RustUnnamed_20 = 196;
pub const ACTOR_EN_NB: C2RustUnnamed_20 = 195;
pub const ACTOR_EN_TANA: C2RustUnnamed_20 = 194;
pub const ACTOR_EN_SYATEKI_MAN: C2RustUnnamed_20 = 193;
pub const ACTOR_EN_SYATEKI_ITM: C2RustUnnamed_20 = 192;
pub const ACTOR_BG_SPOT17_FUNEN: C2RustUnnamed_20 = 191;
pub const ACTOR_BG_HAKA_ZOU: C2RustUnnamed_20 = 190;
pub const ACTOR_BG_HAKA_HUTA: C2RustUnnamed_20 = 189;
pub const ACTOR_BG_HAKA_TRAP: C2RustUnnamed_20 = 188;
pub const ACTOR_BG_HAKA_TUBO: C2RustUnnamed_20 = 187;
pub const ACTOR_BOSS_VA: C2RustUnnamed_20 = 186;
pub const ACTOR_BG_SPOT18_OBJ: C2RustUnnamed_20 = 185;
pub const ACTOR_BG_SPOT09_OBJ: C2RustUnnamed_20 = 184;
pub const ACTOR_MIR_RAY: C2RustUnnamed_20 = 183;
pub const ACTOR_EN_BROB: C2RustUnnamed_20 = 182;
pub const ACTOR_EN_FIRE_ROCK: C2RustUnnamed_20 = 181;
pub const ACTOR_EN_ENCOUNT2: C2RustUnnamed_20 = 180;
pub const ACTOR_EN_HEISHI2: C2RustUnnamed_20 = 179;
pub const ACTOR_UNSET_B2: C2RustUnnamed_20 = 178;
pub const ACTOR_BG_HAKA_SGAMI: C2RustUnnamed_20 = 177;
pub const ACTOR_BG_HAKA_SHIP: C2RustUnnamed_20 = 176;
pub const ACTOR_BG_HAKA_MEGANEBG: C2RustUnnamed_20 = 175;
pub const ACTOR_BG_HAKA_MEGANE: C2RustUnnamed_20 = 174;
pub const ACTOR_EN_VB_BALL: C2RustUnnamed_20 = 173;
pub const ACTOR_BG_VB_SIMA: C2RustUnnamed_20 = 172;
pub const ACTOR_EN_FW: C2RustUnnamed_20 = 171;
pub const ACTOR_DEMO_TRE_LGT: C2RustUnnamed_20 = 170;
pub const ACTOR_DEMO_IM: C2RustUnnamed_20 = 169;
pub const ACTOR_DEMO_DU: C2RustUnnamed_20 = 168;
pub const ACTOR_EN_ENCOUNT1: C2RustUnnamed_20 = 167;
pub const ACTOR_EN_RL: C2RustUnnamed_20 = 166;
pub const ACTOR_EN_DHA: C2RustUnnamed_20 = 165;
pub const ACTOR_EN_DH: C2RustUnnamed_20 = 164;
pub const ACTOR_EN_FD_FIRE: C2RustUnnamed_20 = 163;
pub const ACTOR_BOSS_FD2: C2RustUnnamed_20 = 162;
pub const ACTOR_EN_RU1: C2RustUnnamed_20 = 161;
pub const ACTOR_UNSET_A0: C2RustUnnamed_20 = 160;
pub const ACTOR_MAGIC_FIRE: C2RustUnnamed_20 = 159;
pub const ACTOR_MAGIC_WIND: C2RustUnnamed_20 = 158;
pub const ACTOR_BG_HAKA: C2RustUnnamed_20 = 157;
pub const ACTOR_BG_SPOT02_OBJECTS: C2RustUnnamed_20 = 156;
pub const ACTOR_DOOR_ANA: C2RustUnnamed_20 = 155;
pub const ACTOR_EN_HORSE_LINK_CHILD: C2RustUnnamed_20 = 154;
pub const ACTOR_EN_FD: C2RustUnnamed_20 = 153;
pub const ACTOR_EN_DU: C2RustUnnamed_20 = 152;
pub const ACTOR_OBJECT_KANKYO: C2RustUnnamed_20 = 151;
pub const ACTOR_BOSS_FD: C2RustUnnamed_20 = 150;
pub const ACTOR_EN_SW: C2RustUnnamed_20 = 149;
pub const ACTOR_OBJ_MURE: C2RustUnnamed_20 = 148;
pub const ACTOR_BG_PO_EVENT: C2RustUnnamed_20 = 147;
pub const ACTOR_BG_HEAVY_BLOCK: C2RustUnnamed_20 = 146;
pub const ACTOR_EN_PO_SISTERS: C2RustUnnamed_20 = 145;
pub const ACTOR_EN_RD: C2RustUnnamed_20 = 144;
pub const ACTOR_EN_HEISHI1: C2RustUnnamed_20 = 143;
pub const ACTOR_EN_FLOORMAS: C2RustUnnamed_20 = 142;
pub const ACTOR_BG_HIDAN_FWBIG: C2RustUnnamed_20 = 141;
pub const ACTOR_DEMO_KANKYO: C2RustUnnamed_20 = 140;
pub const ACTOR_DEMO_EFFECT: C2RustUnnamed_20 = 139;
pub const ACTOR_EN_VM: C2RustUnnamed_20 = 138;
pub const ACTOR_BG_MORI_RAKKATENJO: C2RustUnnamed_20 = 137;
pub const ACTOR_BG_MORI_KAITENKABE: C2RustUnnamed_20 = 136;
pub const ACTOR_BG_MORI_ELEVATOR: C2RustUnnamed_20 = 135;
pub const ACTOR_BG_MORI_BIGST: C2RustUnnamed_20 = 134;
pub const ACTOR_EN_TK: C2RustUnnamed_20 = 133;
pub const ACTOR_EN_TA: C2RustUnnamed_20 = 132;
pub const ACTOR_UNSET_83: C2RustUnnamed_20 = 131;
pub const ACTOR_EN_VASE: C2RustUnnamed_20 = 130;
pub const ACTOR_EN_AROW_TRAP: C2RustUnnamed_20 = 129;
pub const ACTOR_EN_TRAP: C2RustUnnamed_20 = 128;
pub const ACTOR_UNSET_7F: C2RustUnnamed_20 = 127;
pub const ACTOR_UNSET_7E: C2RustUnnamed_20 = 126;
pub const ACTOR_EN_PU_BOX: C2RustUnnamed_20 = 125;
pub const ACTOR_EN_LIGHTBOX: C2RustUnnamed_20 = 124;
pub const ACTOR_UNSET_7B: C2RustUnnamed_20 = 123;
pub const ACTOR_UNSET_7A: C2RustUnnamed_20 = 122;
pub const ACTOR_UNSET_79: C2RustUnnamed_20 = 121;
pub const ACTOR_UNSET_78: C2RustUnnamed_20 = 120;
pub const ACTOR_EN_WOOD02: C2RustUnnamed_20 = 119;
pub const ACTOR_UNSET_76: C2RustUnnamed_20 = 118;
pub const ACTOR_UNSET_75: C2RustUnnamed_20 = 117;
pub const ACTOR_UNSET_74: C2RustUnnamed_20 = 116;
pub const ACTOR_UNSET_73: C2RustUnnamed_20 = 115;
pub const ACTOR_EN_BIRD: C2RustUnnamed_20 = 114;
pub const ACTOR_BG_HIDAN_HAMSTEP: C2RustUnnamed_20 = 113;
pub const ACTOR_DOOR_TOKI: C2RustUnnamed_20 = 112;
pub const ACTOR_BG_HIDAN_KOUSI: C2RustUnnamed_20 = 111;
pub const ACTOR_BG_MJIN: C2RustUnnamed_20 = 110;
pub const ACTOR_EN_FHG_FIRE: C2RustUnnamed_20 = 109;
pub const ACTOR_BG_TOKI_SWD: C2RustUnnamed_20 = 108;
pub const ACTOR_EN_YUKABYUN: C2RustUnnamed_20 = 107;
pub const ACTOR_BG_TOKI_HIKARI: C2RustUnnamed_20 = 106;
pub const ACTOR_EN_BB: C2RustUnnamed_20 = 105;
pub const ACTOR_BG_MORI_HINERI: C2RustUnnamed_20 = 104;
pub const ACTOR_EN_FHG: C2RustUnnamed_20 = 103;
pub const ACTOR_ARMS_HOOK: C2RustUnnamed_20 = 102;
pub const ACTOR_BG_MIZU_WATER: C2RustUnnamed_20 = 101;
pub const ACTOR_BG_MIZU_MOVEBG: C2RustUnnamed_20 = 100;
pub const ACTOR_EN_VALI: C2RustUnnamed_20 = 99;
pub const ACTOR_BG_MENKURI_EYE: C2RustUnnamed_20 = 98;
pub const ACTOR_BG_MENKURI_KAITEN: C2RustUnnamed_20 = 97;
pub const ACTOR_EN_DEKUNUTS: C2RustUnnamed_20 = 96;
pub const ACTOR_ITEM_B_HEART: C2RustUnnamed_20 = 95;
pub const ACTOR_OBJ_SYOKUDAI: C2RustUnnamed_20 = 94;
pub const ACTOR_DOOR_WARP1: C2RustUnnamed_20 = 93;
pub const ACTOR_BG_DDAN_KD: C2RustUnnamed_20 = 92;
pub const ACTOR_EN_HORSE_ZELDA: C2RustUnnamed_20 = 91;
pub const ACTOR_EN_JJ: C2RustUnnamed_20 = 90;
pub const ACTOR_BG_BREAKWALL: C2RustUnnamed_20 = 89;
pub const ACTOR_BG_DDAN_JD: C2RustUnnamed_20 = 88;
pub const ACTOR_EN_M_THUNDER: C2RustUnnamed_20 = 87;
pub const ACTOR_EN_M_FIRE1: C2RustUnnamed_20 = 86;
pub const ACTOR_EN_DEKUBABA: C2RustUnnamed_20 = 85;
pub const ACTOR_EN_AM: C2RustUnnamed_20 = 84;
pub const ACTOR_UNSET_53: C2RustUnnamed_20 = 83;
pub const ACTOR_BOSS_GANONDROF: C2RustUnnamed_20 = 82;
pub const ACTOR_BG_YDAN_MARUTA: C2RustUnnamed_20 = 81;
pub const ACTOR_BG_YDAN_HASI: C2RustUnnamed_20 = 80;
pub const ACTOR_EN_OE2: C2RustUnnamed_20 = 79;
pub const ACTOR_BG_HIDAN_FSLIFT: C2RustUnnamed_20 = 78;
pub const ACTOR_EN_ZL2: C2RustUnnamed_20 = 77;
pub const ACTOR_EN_BOMBF: C2RustUnnamed_20 = 76;
pub const ACTOR_EN_MB: C2RustUnnamed_20 = 75;
pub const ACTOR_BG_SPOT00_HANEBASI: C2RustUnnamed_20 = 74;
pub const ACTOR_BG_HIDAN_CURTAIN: C2RustUnnamed_20 = 73;
pub const ACTOR_EN_XC: C2RustUnnamed_20 = 72;
pub const ACTOR_BG_HIDAN_SYOKU: C2RustUnnamed_20 = 71;
pub const ACTOR_BG_HIDAN_SIMA: C2RustUnnamed_20 = 70;
pub const ACTOR_BG_HIDAN_SEKIZOU: C2RustUnnamed_20 = 69;
pub const ACTOR_BG_HIDAN_RSEKIZOU: C2RustUnnamed_20 = 68;
pub const ACTOR_BG_HIDAN_ROCK: C2RustUnnamed_20 = 67;
pub const ACTOR_EN_HORSE_GANON: C2RustUnnamed_20 = 66;
pub const ACTOR_BG_HIDAN_HROCK: C2RustUnnamed_20 = 65;
pub const ACTOR_BG_HIDAN_DALM: C2RustUnnamed_20 = 64;
pub const ACTOR_BG_DODOAGO: C2RustUnnamed_20 = 63;
pub const ACTOR_BG_TREEMOUTH: C2RustUnnamed_20 = 62;
pub const ACTOR_EN_OSSAN: C2RustUnnamed_20 = 61;
pub const ACTOR_EN_HORSE_NORMAL: C2RustUnnamed_20 = 60;
pub const ACTOR_EN_RIVER_SOUND: C2RustUnnamed_20 = 59;
pub const ACTOR_EN_EIYER: C2RustUnnamed_20 = 58;
pub const ACTOR_EN_A_OBJ: C2RustUnnamed_20 = 57;
pub const ACTOR_EN_BW: C2RustUnnamed_20 = 56;
pub const ACTOR_EN_ST: C2RustUnnamed_20 = 55;
pub const ACTOR_UNSET_36: C2RustUnnamed_20 = 54;
pub const ACTOR_EN_TP: C2RustUnnamed_20 = 53;
pub const ACTOR_EN_BILI: C2RustUnnamed_20 = 52;
pub const ACTOR_EN_TORCH2: C2RustUnnamed_20 = 51;
pub const ACTOR_EN_BOOM: C2RustUnnamed_20 = 50;
pub const ACTOR_UNSET_31: C2RustUnnamed_20 = 49;
pub const ACTOR_EN_BDFIRE: C2RustUnnamed_20 = 48;
pub const ACTOR_EN_DODOJR: C2RustUnnamed_20 = 47;
pub const ACTOR_DOOR_SHUTTER: C2RustUnnamed_20 = 46;
pub const ACTOR_EN_BUBBLE: C2RustUnnamed_20 = 45;
pub const ACTOR_BG_PUSHBOX: C2RustUnnamed_20 = 44;
pub const ACTOR_EN_GOMA: C2RustUnnamed_20 = 43;
pub const ACTOR_EN_VIEWER: C2RustUnnamed_20 = 42;
pub const ACTOR_EN_ZL1: C2RustUnnamed_20 = 41;
pub const ACTOR_BOSS_GOMA: C2RustUnnamed_20 = 40;
pub const ACTOR_BOSS_DODONGO: C2RustUnnamed_20 = 39;
pub const ACTOR_EN_HATA: C2RustUnnamed_20 = 38;
pub const ACTOR_EN_ZF: C2RustUnnamed_20 = 37;
pub const ACTOR_EN_SCENE_CHANGE: C2RustUnnamed_20 = 36;
pub const ACTOR_EN_HOLL: C2RustUnnamed_20 = 35;
pub const ACTOR_UNSET_22: C2RustUnnamed_20 = 34;
pub const ACTOR_EN_FISH: C2RustUnnamed_20 = 33;
pub const ACTOR_EN_INSECT: C2RustUnnamed_20 = 32;
pub const ACTOR_UNSET_1F: C2RustUnnamed_20 = 31;
pub const ACTOR_EN_BUTTE: C2RustUnnamed_20 = 30;
pub const ACTOR_EN_PEEHAT: C2RustUnnamed_20 = 29;
pub const ACTOR_EN_REEBA: C2RustUnnamed_20 = 28;
pub const ACTOR_EN_TITE: C2RustUnnamed_20 = 27;
pub const ACTOR_UNSET_1A: C2RustUnnamed_20 = 26;
pub const ACTOR_EN_NIW: C2RustUnnamed_20 = 25;
pub const ACTOR_EN_ELF: C2RustUnnamed_20 = 24;
pub const ACTOR_UNSET_17: C2RustUnnamed_20 = 23;
pub const ACTOR_EN_ARROW: C2RustUnnamed_20 = 22;
pub const ACTOR_EN_ITEM00: C2RustUnnamed_20 = 21;
pub const ACTOR_EN_HORSE: C2RustUnnamed_20 = 20;
pub const ACTOR_EN_FIREFLY: C2RustUnnamed_20 = 19;
pub const ACTOR_EN_DODONGO: C2RustUnnamed_20 = 18;
pub const ACTOR_EN_WALLMAS: C2RustUnnamed_20 = 17;
pub const ACTOR_EN_BOM: C2RustUnnamed_20 = 16;
pub const ACTOR_BG_YDAN_SP: C2RustUnnamed_20 = 15;
pub const ACTOR_EN_OKUTA: C2RustUnnamed_20 = 14;
pub const ACTOR_EN_POH: C2RustUnnamed_20 = 13;
pub const ACTOR_BG_HIDAN_FIREWALL: C2RustUnnamed_20 = 12;
pub const ACTOR_BG_DY_YOSEIZO: C2RustUnnamed_20 = 11;
pub const ACTOR_EN_BOX: C2RustUnnamed_20 = 10;
pub const ACTOR_EN_DOOR: C2RustUnnamed_20 = 9;
pub const ACTOR_EN_LIGHT: C2RustUnnamed_20 = 8;
pub const ACTOR_EN_PART: C2RustUnnamed_20 = 7;
pub const ACTOR_UNSET_6: C2RustUnnamed_20 = 6;
pub const ACTOR_UNSET_5: C2RustUnnamed_20 = 5;
pub const ACTOR_EN_GIRLA: C2RustUnnamed_20 = 4;
pub const ACTOR_UNSET_3: C2RustUnnamed_20 = 3;
pub const ACTOR_EN_TEST: C2RustUnnamed_20 = 2;
pub const ACTOR_UNSET_1: C2RustUnnamed_20 = 1;
pub const ACTOR_PLAYER: C2RustUnnamed_20 = 0;
pub type C2RustUnnamed_21 = libc::c_uint;
pub const OBJECT_ID_MAX: C2RustUnnamed_21 = 402;
pub const OBJECT_ZL4: C2RustUnnamed_21 = 401;
pub const OBJECT_TIMEBLOCK: C2RustUnnamed_21 = 400;
pub const OBJECT_OUKE_HAKA: C2RustUnnamed_21 = 399;
pub const OBJECT_DOOR_KILLER: C2RustUnnamed_21 = 398;
pub const OBJECT_GI_SWORD_1: C2RustUnnamed_21 = 397;
pub const OBJECT_COB: C2RustUnnamed_21 = 396;
pub const OBJECT_COW: C2RustUnnamed_21 = 395;
pub const OBJECT_BWALL: C2RustUnnamed_21 = 394;
pub const OBJECT_PS: C2RustUnnamed_21 = 393;
pub const OBJECT_GS: C2RustUnnamed_21 = 392;
pub const OBJECT_HAKA_DOOR: C2RustUnnamed_21 = 391;
pub const OBJECT_GEFF: C2RustUnnamed_21 = 390;
pub const OBJECT_GJ: C2RustUnnamed_21 = 389;
pub const OBJECT_SKB: C2RustUnnamed_21 = 388;
pub const OBJECT_WF: C2RustUnnamed_21 = 387;
pub const OBJECT_MU: C2RustUnnamed_21 = 386;
pub const OBJECT_SPOT01_MATOYAB: C2RustUnnamed_21 = 385;
pub const OBJECT_SPOT01_MATOYA: C2RustUnnamed_21 = 384;
pub const OBJECT_GI_RUPY: C2RustUnnamed_21 = 383;
pub const OBJECT_GANON_ANIME3: C2RustUnnamed_21 = 382;
pub const OBJECT_GANON_ANIME2: C2RustUnnamed_21 = 381;
pub const OBJECT_GANON_ANIME1: C2RustUnnamed_21 = 380;
pub const OBJECT_GI_DEKUPOUCH: C2RustUnnamed_21 = 379;
pub const OBJECT_EFC_DOUGHNUT: C2RustUnnamed_21 = 378;
pub const OBJECT_DEMO_KEKKAI: C2RustUnnamed_21 = 377;
pub const OBJECT_BOWL: C2RustUnnamed_21 = 376;
pub const OBJECT_GI_SOUL: C2RustUnnamed_21 = 375;
pub const OBJECT_GI_GHOST: C2RustUnnamed_21 = 374;
pub const OBJECT_GI_BUTTERFLY: C2RustUnnamed_21 = 373;
pub const OBJECT_GI_INSECT: C2RustUnnamed_21 = 372;
pub const OBJECT_GI_FIRE: C2RustUnnamed_21 = 371;
pub const OBJECT_DNK: C2RustUnnamed_21 = 370;
pub const OBJECT_DNS: C2RustUnnamed_21 = 369;
pub const OBJECT_KIBAKO2: C2RustUnnamed_21 = 368;
pub const OBJECT_SPOT11_OBJ: C2RustUnnamed_21 = 367;
pub const OBJECT_UNSET_16E: C2RustUnnamed_21 = 366;
pub const OBJECT_JYA_DOOR: C2RustUnnamed_21 = 365;
pub const OBJECT_JYA_IRON: C2RustUnnamed_21 = 364;
pub const OBJECT_DOG: C2RustUnnamed_21 = 363;
pub const OBJECT_GR: C2RustUnnamed_21 = 362;
pub const OBJECT_GELDB: C2RustUnnamed_21 = 361;
pub const OBJECT_SHOPNUTS: C2RustUnnamed_21 = 360;
pub const OBJECT_GLA: C2RustUnnamed_21 = 359;
pub const OBJECT_SPOT00_BREAK: C2RustUnnamed_21 = 358;
pub const OBJECT_RS: C2RustUnnamed_21 = 357;
pub const OBJECT_HINTNUTS: C2RustUnnamed_21 = 356;
pub const OBJECT_BOMBIWA: C2RustUnnamed_21 = 355;
pub const OBJECT_SPOT12_OBJ: C2RustUnnamed_21 = 354;
pub const OBJECT_SPOT05_OBJECTS: C2RustUnnamed_21 = 353;
pub const OBJECT_BG: C2RustUnnamed_21 = 352;
pub const OBJECT_BIGOKUTA: C2RustUnnamed_21 = 351;
pub const OBJECT_SSH: C2RustUnnamed_21 = 350;
pub const OBJECT_GI_GODDESS: C2RustUnnamed_21 = 349;
pub const OBJECT_GI_SUTARU: C2RustUnnamed_21 = 348;
pub const OBJECT_FISH: C2RustUnnamed_21 = 347;
pub const OBJECT_EC: C2RustUnnamed_21 = 346;
pub const OBJECT_DS2: C2RustUnnamed_21 = 345;
pub const OBJECT_GI_M_ARROW: C2RustUnnamed_21 = 344;
pub const OBJECT_GI_HOVERBOOTS: C2RustUnnamed_21 = 343;
pub const OBJECT_ZG: C2RustUnnamed_21 = 342;
pub const OBJECT_TS: C2RustUnnamed_21 = 341;
pub const OBJECT_KA: C2RustUnnamed_21 = 340;
pub const OBJECT_GANON2: C2RustUnnamed_21 = 339;
pub const OBJECT_GI_GERUDOMASK: C2RustUnnamed_21 = 338;
pub const OBJECT_GI_ZORAMASK: C2RustUnnamed_21 = 337;
pub const OBJECT_GI_GOLONMASK: C2RustUnnamed_21 = 336;
pub const OBJECT_ZL2_ANIME2: C2RustUnnamed_21 = 335;
pub const OBJECT_ZL2_ANIME1: C2RustUnnamed_21 = 334;
pub const OBJECT_EFC_ERUPC: C2RustUnnamed_21 = 333;
pub const OBJECT_GT: C2RustUnnamed_21 = 332;
pub const OBJECT_DOOR_GERUDO: C2RustUnnamed_21 = 331;
pub const OBJECT_MAG: C2RustUnnamed_21 = 330;
pub const OBJECT_GI_FROG: C2RustUnnamed_21 = 329;
pub const OBJECT_GI_SOLDOUT: C2RustUnnamed_21 = 328;
pub const OBJECT_GI_BRACELET: C2RustUnnamed_21 = 327;
pub const OBJECT_GI_PRESCRIPTION: C2RustUnnamed_21 = 326;
pub const OBJECT_CS: C2RustUnnamed_21 = 325;
pub const OBJECT_JS: C2RustUnnamed_21 = 324;
pub const OBJECT_GI_BROKENSWORD: C2RustUnnamed_21 = 323;
pub const OBJECT_GI_TICKETSTONE: C2RustUnnamed_21 = 322;
pub const OBJECT_GI_MUSHROOM: C2RustUnnamed_21 = 321;
pub const OBJECT_GI_POWDER: C2RustUnnamed_21 = 320;
pub const OBJECT_GI_EYE_LOTION: C2RustUnnamed_21 = 319;
pub const OBJECT_OS: C2RustUnnamed_21 = 318;
pub const OBJECT_FA: C2RustUnnamed_21 = 317;
pub const OBJECT_MM: C2RustUnnamed_21 = 316;
pub const OBJECT_STREAM: C2RustUnnamed_21 = 315;
pub const OBJECT_SIOFUKI: C2RustUnnamed_21 = 314;
pub const OBJECT_GANON_OBJECTS: C2RustUnnamed_21 = 313;
pub const OBJECT_GI_TRUTH_MASK: C2RustUnnamed_21 = 312;
pub const OBJECT_GI_RABIT_MASK: C2RustUnnamed_21 = 311;
pub const OBJECT_GI_SKJ_MASK: C2RustUnnamed_21 = 310;
pub const OBJECT_GI_REDEAD_MASK: C2RustUnnamed_21 = 309;
pub const OBJECT_GI_KI_TAN_MASK: C2RustUnnamed_21 = 308;
pub const OBJECT_FU: C2RustUnnamed_21 = 307;
pub const OBJECT_MK: C2RustUnnamed_21 = 306;
pub const OBJECT_OWL: C2RustUnnamed_21 = 305;
pub const OBJECT_GJYO_OBJECTS: C2RustUnnamed_21 = 304;
pub const OBJECT_KANBAN: C2RustUnnamed_21 = 303;
pub const OBJECT_GI_COIN: C2RustUnnamed_21 = 302;
pub const OBJECT_GI_GLOVES: C2RustUnnamed_21 = 301;
pub const OBJECT_TSUBO: C2RustUnnamed_21 = 300;
pub const OBJECT_KUSA: C2RustUnnamed_21 = 299;
pub const OBJECT_LIGHTSWITCH: C2RustUnnamed_21 = 298;
pub const OBJECT_INGATE: C2RustUnnamed_21 = 297;
pub const OBJECT_HS: C2RustUnnamed_21 = 296;
pub const OBJECT_MS: C2RustUnnamed_21 = 295;
pub const OBJECT_GM: C2RustUnnamed_21 = 294;
pub const OBJECT_BLKOBJ: C2RustUnnamed_21 = 293;
pub const OBJECT_NWC: C2RustUnnamed_21 = 292;
pub const OBJECT_UNSET_123: C2RustUnnamed_21 = 291;
pub const OBJECT_DAIKU: C2RustUnnamed_21 = 290;
pub const OBJECT_TORYO: C2RustUnnamed_21 = 289;
pub const OBJECT_UNSET_120: C2RustUnnamed_21 = 288;
pub const OBJECT_GOROIWA: C2RustUnnamed_21 = 287;
pub const OBJECT_MAMENOKI: C2RustUnnamed_21 = 286;
pub const OBJECT_D_LIFT: C2RustUnnamed_21 = 285;
pub const OBJECT_D_HSBLOCK: C2RustUnnamed_21 = 284;
pub const OBJECT_D_ELEVATOR: C2RustUnnamed_21 = 283;
pub const OBJECT_GND_MAGIC: C2RustUnnamed_21 = 282;
pub const OBJECT_GI_SEED: C2RustUnnamed_21 = 281;
pub const OBJECT_GI_BOOTS_2: C2RustUnnamed_21 = 280;
pub const OBJECT_YABUSAME_POINT: C2RustUnnamed_21 = 279;
pub const OBJECT_GE1: C2RustUnnamed_21 = 278;
pub const OBJECT_BOB: C2RustUnnamed_21 = 277;
pub const OBJECT_FZ: C2RustUnnamed_21 = 276;
pub const OBJECT_SPOT07_OBJECT: C2RustUnnamed_21 = 275;
pub const OBJECT_SPOT03_OBJECT: C2RustUnnamed_21 = 274;
pub const OBJECT_BOJ: C2RustUnnamed_21 = 273;
pub const OBJECT_ANE: C2RustUnnamed_21 = 272;
pub const OBJECT_DS: C2RustUnnamed_21 = 271;
pub const OBJECT_GI_OCARINA_0: C2RustUnnamed_21 = 270;
pub const OBJECT_BBA: C2RustUnnamed_21 = 269;
pub const OBJECT_BJI: C2RustUnnamed_21 = 268;
pub const OBJECT_GI_BOTTLE_LETTER: C2RustUnnamed_21 = 267;
pub const OBJECT_SKJ: C2RustUnnamed_21 = 266;
pub const OBJECT_GI_NIWATORI: C2RustUnnamed_21 = 265;
pub const OBJECT_CNE: C2RustUnnamed_21 = 264;
pub const OBJECT_AHG: C2RustUnnamed_21 = 263;
pub const OBJECT_IK: C2RustUnnamed_21 = 262;
pub const OBJECT_AOB: C2RustUnnamed_21 = 261;
pub const OBJECT_MASTERZOORA: C2RustUnnamed_21 = 260;
pub const OBJECT_MASTERGOLON: C2RustUnnamed_21 = 259;
pub const OBJECT_MASTERKOKIRIHEAD: C2RustUnnamed_21 = 258;
pub const OBJECT_MASTERKOKIRI: C2RustUnnamed_21 = 257;
pub const OBJECT_UMAJUMP: C2RustUnnamed_21 = 256;
pub const OBJECT_KZ: C2RustUnnamed_21 = 255;
pub const OBJECT_ZO: C2RustUnnamed_21 = 254;
pub const OBJECT_KW1: C2RustUnnamed_21 = 253;
pub const OBJECT_KM1: C2RustUnnamed_21 = 252;
pub const OBJECT_MD: C2RustUnnamed_21 = 251;
pub const OBJECT_MD_UNUSED: C2RustUnnamed_21 = 250;
pub const OBJECT_SPOT01_OBJECTS: C2RustUnnamed_21 = 249;
pub const OBJECT_GI_LONGSWORD: C2RustUnnamed_21 = 248;
pub const OBJECT_GI_GRASS: C2RustUnnamed_21 = 247;
pub const OBJECT_GI_HAMMER: C2RustUnnamed_21 = 246;
pub const OBJECT_GI_SAW: C2RustUnnamed_21 = 245;
pub const OBJECT_GI_FISH: C2RustUnnamed_21 = 244;
pub const OBJECT_GI_BEAN: C2RustUnnamed_21 = 243;
pub const OBJECT_GI_CLOTHES: C2RustUnnamed_21 = 242;
pub const OBJECT_JYA_OBJ: C2RustUnnamed_21 = 241;
pub const OBJECT_SPOT15_OBJ: C2RustUnnamed_21 = 240;
pub const OBJECT_GI_LETTER: C2RustUnnamed_21 = 239;
pub const OBJECT_GI_SHIELD_3: C2RustUnnamed_21 = 238;
pub const OBJECT_DEMO_6K: C2RustUnnamed_21 = 237;
pub const OBJECT_ANI: C2RustUnnamed_21 = 236;
pub const OBJECT_GI_LIQUID: C2RustUnnamed_21 = 235;
pub const OBJECT_GI_GLASSES: C2RustUnnamed_21 = 234;
pub const OBJECT_GI_BOW: C2RustUnnamed_21 = 233;
pub const OBJECT_GI_BOOMERANG: C2RustUnnamed_21 = 232;
pub const OBJECT_GI_PACHINKO: C2RustUnnamed_21 = 231;
pub const OBJECT_FR: C2RustUnnamed_21 = 230;
pub const OBJECT_NY: C2RustUnnamed_21 = 229;
pub const OBJECT_UNSET_E4: C2RustUnnamed_21 = 228;
pub const OBJECT_NY_UNUSED: C2RustUnnamed_21 = 227;
pub const OBJECT_SST: C2RustUnnamed_21 = 226;
pub const OBJECT_GANON: C2RustUnnamed_21 = 225;
pub const OBJECT_MA1: C2RustUnnamed_21 = 224;
pub const OBJECT_GI_MILK: C2RustUnnamed_21 = 223;
pub const OBJECT_GI_OCARINA: C2RustUnnamed_21 = 222;
pub const OBJECT_GI_HOOKSHOT: C2RustUnnamed_21 = 221;
pub const OBJECT_GI_SHIELD_2: C2RustUnnamed_21 = 220;
pub const OBJECT_GI_SCALE: C2RustUnnamed_21 = 219;
pub const OBJECT_GI_EGG: C2RustUnnamed_21 = 218;
pub const OBJECT_GI_BOMB_2: C2RustUnnamed_21 = 217;
pub const OBJECT_GI_ARROW: C2RustUnnamed_21 = 216;
pub const OBJECT_GI_GERUDO: C2RustUnnamed_21 = 215;
pub const OBJECT_ANUBICE: C2RustUnnamed_21 = 214;
pub const OBJECT_BXA: C2RustUnnamed_21 = 213;
pub const OBJECT_RR: C2RustUnnamed_21 = 212;
pub const OBJECT_TW: C2RustUnnamed_21 = 211;
pub const OBJECT_HNI: C2RustUnnamed_21 = 210;
pub const OBJECT_GI_PURSE: C2RustUnnamed_21 = 209;
pub const OBJECT_MA2: C2RustUnnamed_21 = 208;
pub const OBJECT_OF1S: C2RustUnnamed_21 = 207;
pub const OBJECT_GI_BOMB_1: C2RustUnnamed_21 = 206;
pub const OBJECT_GI_MAGICPOT: C2RustUnnamed_21 = 205;
pub const OBJECT_DEKUJR: C2RustUnnamed_21 = 204;
pub const OBJECT_GI_SHIELD_1: C2RustUnnamed_21 = 203;
pub const OBJECT_RU2: C2RustUnnamed_21 = 202;
pub const OBJECT_OF1D_MAP: C2RustUnnamed_21 = 201;
pub const OBJECT_GI_MAP: C2RustUnnamed_21 = 200;
pub const OBJECT_GI_STICK: C2RustUnnamed_21 = 199;
pub const OBJECT_GI_BOTTLE: C2RustUnnamed_21 = 198;
pub const OBJECT_OS_ANIME: C2RustUnnamed_21 = 197;
pub const OBJECT_OE4S: C2RustUnnamed_21 = 196;
pub const OBJECT_OE1S: C2RustUnnamed_21 = 195;
pub const OBJECT_SPOT16_OBJ: C2RustUnnamed_21 = 194;
pub const OBJECT_TR: C2RustUnnamed_21 = 193;
pub const OBJECT_IN: C2RustUnnamed_21 = 192;
pub const OBJECT_GI_BOMBPOUCH: C2RustUnnamed_21 = 191;
pub const OBJECT_GI_ARROWCASE: C2RustUnnamed_21 = 190;
pub const OBJECT_GI_HEARTS: C2RustUnnamed_21 = 189;
pub const OBJECT_SA: C2RustUnnamed_21 = 188;
pub const OBJECT_GI_NUTS: C2RustUnnamed_21 = 187;
pub const OBJECT_GI_MEDAL: C2RustUnnamed_21 = 186;
pub const OBJECT_GI_BOSSKEY: C2RustUnnamed_21 = 185;
pub const OBJECT_GI_COMPASS: C2RustUnnamed_21 = 184;
pub const OBJECT_GI_HEART: C2RustUnnamed_21 = 183;
pub const OBJECT_GI_MELODY: C2RustUnnamed_21 = 182;
pub const OBJECT_SB: C2RustUnnamed_21 = 181;
pub const OBJECT_MO: C2RustUnnamed_21 = 180;
pub const OBJECT_NB: C2RustUnnamed_21 = 179;
pub const OBJECT_SHOP_DUNGEN: C2RustUnnamed_21 = 178;
pub const OBJECT_SPOT17_OBJ: C2RustUnnamed_21 = 177;
pub const OBJECT_BDOOR: C2RustUnnamed_21 = 176;
pub const OBJECT_SPOT18_OBJ: C2RustUnnamed_21 = 175;
pub const OBJECT_SPOT09_OBJ: C2RustUnnamed_21 = 174;
pub const OBJECT_GI_JEWEL: C2RustUnnamed_21 = 173;
pub const OBJECT_BROB: C2RustUnnamed_21 = 172;
pub const OBJECT_MIR_RAY: C2RustUnnamed_21 = 171;
pub const OBJECT_GI_KEY: C2RustUnnamed_21 = 170;
pub const OBJECT_DEMO_TRE_LGT: C2RustUnnamed_21 = 169;
pub const OBJECT_EFC_TW: C2RustUnnamed_21 = 168;
pub const OBJECT_RL: C2RustUnnamed_21 = 167;
pub const OBJECT_DH: C2RustUnnamed_21 = 166;
pub const OBJECT_FD2: C2RustUnnamed_21 = 165;
pub const OBJECT_SYOKUDAI: C2RustUnnamed_21 = 164;
pub const OBJECT_RU1: C2RustUnnamed_21 = 163;
pub const OBJECT_HAKA: C2RustUnnamed_21 = 162;
pub const OBJECT_SPOT02_OBJECTS: C2RustUnnamed_21 = 161;
pub const OBJECT_HORSE_LINK_CHILD: C2RustUnnamed_21 = 160;
pub const OBJECT_MEDAL: C2RustUnnamed_21 = 159;
pub const OBJECT_FW: C2RustUnnamed_21 = 158;
pub const OBJECT_DU: C2RustUnnamed_21 = 157;
pub const OBJECT_FD: C2RustUnnamed_21 = 156;
pub const OBJECT_GNDD: C2RustUnnamed_21 = 155;
pub const OBJECT_HEAVY_OBJECT: C2RustUnnamed_21 = 154;
pub const OBJECT_PO_SISTERS: C2RustUnnamed_21 = 153;
pub const OBJECT_RD: C2RustUnnamed_21 = 152;
pub const OBJECT_SD: C2RustUnnamed_21 = 151;
pub const OBJECT_BDAN_OBJECTS: C2RustUnnamed_21 = 150;
pub const OBJECT_TRIFORCE_SPOT: C2RustUnnamed_21 = 149;
pub const OBJECT_LIGHT_RING: C2RustUnnamed_21 = 148;
pub const OBJECT_GOD_LGT: C2RustUnnamed_21 = 147;
pub const OBJECT_EFC_STAR_FIELD: C2RustUnnamed_21 = 146;
pub const OBJECT_EFC_LGT_SHOWER: C2RustUnnamed_21 = 145;
pub const OBJECT_EFC_FLASH: C2RustUnnamed_21 = 144;
pub const OBJECT_EFC_FIRE_BALL: C2RustUnnamed_21 = 143;
pub const OBJECT_EFC_CRYSTAL_LIGHT: C2RustUnnamed_21 = 142;
pub const OBJECT_HAKACH_OBJECTS: C2RustUnnamed_21 = 141;
pub const OBJECT_BV: C2RustUnnamed_21 = 140;
pub const OBJECT_VM: C2RustUnnamed_21 = 139;
pub const OBJECT_XC: C2RustUnnamed_21 = 138;
pub const OBJECT_TK: C2RustUnnamed_21 = 137;
pub const OBJECT_TA: C2RustUnnamed_21 = 136;
pub const OBJECT_IM: C2RustUnnamed_21 = 135;
pub const OBJECT_VASE: C2RustUnnamed_21 = 134;
pub const OBJECT_TRAP: C2RustUnnamed_21 = 133;
pub const OBJECT_UNSET_84: C2RustUnnamed_21 = 132;
pub const OBJECT_UNSET_83: C2RustUnnamed_21 = 131;
pub const OBJECT_PU_BOX: C2RustUnnamed_21 = 130;
pub const OBJECT_LIGHTBOX: C2RustUnnamed_21 = 129;
pub const OBJECT_UNSET_80: C2RustUnnamed_21 = 128;
pub const OBJECT_UNSET_7F: C2RustUnnamed_21 = 127;
pub const OBJECT_UNSET_7E: C2RustUnnamed_21 = 126;
pub const OBJECT_UNSET_7D: C2RustUnnamed_21 = 125;
pub const OBJECT_WOOD02: C2RustUnnamed_21 = 124;
pub const OBJECT_UNSET_7B: C2RustUnnamed_21 = 123;
pub const OBJECT_UNSET_7A: C2RustUnnamed_21 = 122;
pub const OBJECT_UNSET_79: C2RustUnnamed_21 = 121;
pub const OBJECT_UNSET_78: C2RustUnnamed_21 = 120;
pub const OBJECT_BIRD: C2RustUnnamed_21 = 119;
pub const OBJECT_HATA: C2RustUnnamed_21 = 118;
pub const OBJECT_WARP2: C2RustUnnamed_21 = 117;
pub const OBJECT_SPOT08_OBJ: C2RustUnnamed_21 = 116;
pub const OBJECT_MORI_TEX: C2RustUnnamed_21 = 115;
pub const OBJECT_MORI_OBJECTS: C2RustUnnamed_21 = 114;
pub const OBJECT_MORI_HINERI2A: C2RustUnnamed_21 = 113;
pub const OBJECT_MORI_HINERI2: C2RustUnnamed_21 = 112;
pub const OBJECT_MORI_HINERI1A: C2RustUnnamed_21 = 111;
pub const OBJECT_PO_COMPOSER: C2RustUnnamed_21 = 110;
pub const OBJECT_PO_FIELD: C2RustUnnamed_21 = 109;
pub const OBJECT_RELAY_OBJECTS: C2RustUnnamed_21 = 108;
pub const OBJECT_ICE_OBJECTS: C2RustUnnamed_21 = 107;
pub const OBJECT_SPOT06_OBJECTS: C2RustUnnamed_21 = 106;
pub const OBJECT_HAKA_OBJECTS: C2RustUnnamed_21 = 105;
pub const OBJECT_MJIN_OKA: C2RustUnnamed_21 = 104;
pub const OBJECT_MJIN_WIND: C2RustUnnamed_21 = 103;
pub const OBJECT_MJIN_SOUL: C2RustUnnamed_21 = 102;
pub const OBJECT_MJIN_ICE: C2RustUnnamed_21 = 101;
pub const OBJECT_MJIN_FLAME: C2RustUnnamed_21 = 100;
pub const OBJECT_MJIN_DARK: C2RustUnnamed_21 = 99;
pub const OBJECT_MJIN_FLASH: C2RustUnnamed_21 = 98;
pub const OBJECT_MJIN: C2RustUnnamed_21 = 97;
pub const OBJECT_ZL2: C2RustUnnamed_21 = 96;
pub const OBJECT_YUKABYUN: C2RustUnnamed_21 = 95;
pub const OBJECT_TOKI_OBJECTS: C2RustUnnamed_21 = 94;
pub const OBJECT_BB: C2RustUnnamed_21 = 93;
pub const OBJECT_MORI_HINERI1: C2RustUnnamed_21 = 92;
pub const OBJECT_OSSAN: C2RustUnnamed_21 = 91;
pub const OBJECT_FHG: C2RustUnnamed_21 = 90;
pub const OBJECT_MIZU_OBJECTS: C2RustUnnamed_21 = 89;
pub const OBJECT_OA11: C2RustUnnamed_21 = 88;
pub const OBJECT_OA10: C2RustUnnamed_21 = 87;
pub const OBJECT_VALI: C2RustUnnamed_21 = 86;
pub const OBJECT_OE12: C2RustUnnamed_21 = 85;
pub const OBJECT_OE11: C2RustUnnamed_21 = 84;
pub const OBJECT_OE10: C2RustUnnamed_21 = 83;
pub const OBJECT_OE9: C2RustUnnamed_21 = 82;
pub const OBJECT_OE8: C2RustUnnamed_21 = 81;
pub const OBJECT_OE7: C2RustUnnamed_21 = 80;
pub const OBJECT_OE6: C2RustUnnamed_21 = 79;
pub const OBJECT_OE5: C2RustUnnamed_21 = 78;
pub const OBJECT_MENKURI_OBJECTS: C2RustUnnamed_21 = 77;
pub const OBJECT_OE4: C2RustUnnamed_21 = 76;
pub const OBJECT_OE3: C2RustUnnamed_21 = 75;
pub const OBJECT_DEKUNUTS: C2RustUnnamed_21 = 74;
pub const OBJECT_B_HEART: C2RustUnnamed_21 = 73;
pub const OBJECT_WARP1: C2RustUnnamed_21 = 72;
pub const OBJECT_OPENING_DEMO1: C2RustUnnamed_21 = 71;
pub const OBJECT_HORSE_ZELDA: C2RustUnnamed_21 = 70;
pub const OBJECT_OB4: C2RustUnnamed_21 = 69;
pub const OBJECT_OB3: C2RustUnnamed_21 = 68;
pub const OBJECT_OB2: C2RustUnnamed_21 = 67;
pub const OBJECT_OA9: C2RustUnnamed_21 = 66;
pub const OBJECT_OA8: C2RustUnnamed_21 = 65;
pub const OBJECT_JJ: C2RustUnnamed_21 = 64;
pub const OBJECT_OA7: C2RustUnnamed_21 = 63;
pub const OBJECT_OA6: C2RustUnnamed_21 = 62;
pub const OBJECT_OA5: C2RustUnnamed_21 = 61;
pub const OBJECT_OA4: C2RustUnnamed_21 = 60;
pub const OBJECT_OA3: C2RustUnnamed_21 = 59;
pub const OBJECT_UNSET_3A: C2RustUnnamed_21 = 58;
pub const OBJECT_DEKUBABA: C2RustUnnamed_21 = 57;
pub const OBJECT_AM: C2RustUnnamed_21 = 56;
pub const OBJECT_GND: C2RustUnnamed_21 = 55;
pub const OBJECT_YDAN_OBJECTS: C2RustUnnamed_21 = 54;
pub const OBJECT_OE2: C2RustUnnamed_21 = 53;
pub const OBJECT_OE_ANIME: C2RustUnnamed_21 = 52;
pub const OBJECT_OE1: C2RustUnnamed_21 = 51;
pub const OBJECT_SK2: C2RustUnnamed_21 = 50;
pub const OBJECT_BOMBF: C2RustUnnamed_21 = 49;
pub const OBJECT_MB: C2RustUnnamed_21 = 48;
pub const OBJECT_SPOT00_OBJECTS: C2RustUnnamed_21 = 47;
pub const OBJECT_OA2: C2RustUnnamed_21 = 46;
pub const OBJECT_HORSE_GANON: C2RustUnnamed_21 = 45;
pub const OBJECT_HIDAN_OBJECTS: C2RustUnnamed_21 = 44;
pub const OBJECT_DDAN_OBJECTS: C2RustUnnamed_21 = 43;
pub const OBJECT_SPOT04_OBJECTS: C2RustUnnamed_21 = 42;
pub const OBJECT_O_ANIME: C2RustUnnamed_21 = 41;
pub const OBJECT_OB1: C2RustUnnamed_21 = 40;
pub const OBJECT_HORSE_NORMAL: C2RustUnnamed_21 = 39;
pub const OBJECT_EI: C2RustUnnamed_21 = 38;
pub const OBJECT_BW: C2RustUnnamed_21 = 37;
pub const OBJECT_ST: C2RustUnnamed_21 = 36;
pub const OBJECT_OA1: C2RustUnnamed_21 = 35;
pub const OBJECT_TP: C2RustUnnamed_21 = 34;
pub const OBJECT_BL: C2RustUnnamed_21 = 33;
pub const OBJECT_TORCH2: C2RustUnnamed_21 = 32;
pub const OBJECT_DODOJR: C2RustUnnamed_21 = 31;
pub const OBJECT_GOL: C2RustUnnamed_21 = 30;
pub const OBJECT_ZL1: C2RustUnnamed_21 = 29;
pub const OBJECT_GOMA: C2RustUnnamed_21 = 28;
pub const OBJECT_ZF: C2RustUnnamed_21 = 27;
pub const OBJECT_HORSE: C2RustUnnamed_21 = 26;
pub const OBJECT_KINGDODONGO: C2RustUnnamed_21 = 25;
pub const OBJECT_PEEHAT: C2RustUnnamed_21 = 24;
pub const OBJECT_REEBA: C2RustUnnamed_21 = 23;
pub const OBJECT_TITE: C2RustUnnamed_21 = 22;
pub const OBJECT_LINK_CHILD: C2RustUnnamed_21 = 21;
pub const OBJECT_LINK_BOY: C2RustUnnamed_21 = 20;
pub const OBJECT_NIW: C2RustUnnamed_21 = 19;
pub const OBJECT_BUBBLE: C2RustUnnamed_21 = 18;
pub const OBJECT_UNSET_11: C2RustUnnamed_21 = 17;
pub const OBJECT_UNSET_10: C2RustUnnamed_21 = 16;
pub const OBJECT_FIRE: C2RustUnnamed_21 = 15;
pub const OBJECT_BOX: C2RustUnnamed_21 = 14;
pub const OBJECT_FIREFLY: C2RustUnnamed_21 = 13;
pub const OBJECT_DODONGO: C2RustUnnamed_21 = 12;
pub const OBJECT_WALLMASTER: C2RustUnnamed_21 = 11;
pub const OBJECT_DY_OBJ: C2RustUnnamed_21 = 10;
pub const OBJECT_POH: C2RustUnnamed_21 = 9;
pub const OBJECT_CROW: C2RustUnnamed_21 = 8;
pub const OBJECT_OKUTA: C2RustUnnamed_21 = 7;
pub const OBJECT_HUMAN: C2RustUnnamed_21 = 6;
pub const OBJECT_UNSET_5: C2RustUnnamed_21 = 5;
pub const OBJECT_UNSET_4: C2RustUnnamed_21 = 4;
pub const OBJECT_GAMEPLAY_DANGEON_KEEP: C2RustUnnamed_21 = 3;
pub const OBJECT_GAMEPLAY_FIELD_KEEP: C2RustUnnamed_21 = 2;
pub const OBJECT_GAMEPLAY_KEEP: C2RustUnnamed_21 = 1;
pub const OBJECT_INVALID: C2RustUnnamed_21 = 0;
pub type C2RustUnnamed_22 = libc::c_uint;
pub const SEQ_PLAYER_BGM_SUB: C2RustUnnamed_22 = 3;
pub const SEQ_PLAYER_SFX: C2RustUnnamed_22 = 2;
pub const SEQ_PLAYER_FANFARE: C2RustUnnamed_22 = 1;
pub const SEQ_PLAYER_BGM_MAIN: C2RustUnnamed_22 = 0;
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct InitChainEntry {
    #[bitfield(name = "cont", ty = "u32_0", bits = "0..=0")]
    #[bitfield(name = "type_0", ty = "u32_0", bits = "1..=4")]
    #[bitfield(name = "offset", ty = "u32_0", bits = "5..=15")]
    #[bitfield(name = "value", ty = "s32", bits = "16..=31")]
    pub cont_type_0_offset_value: [u8; 4],
}
pub type C2RustUnnamed_23 = libc::c_uint;
pub const ICHAINTYPE_VEC3S: C2RustUnnamed_23 = 10;
pub const ICHAINTYPE_VEC3F_DIV1000: C2RustUnnamed_23 = 9;
pub const ICHAINTYPE_VEC3F: C2RustUnnamed_23 = 8;
pub const ICHAINTYPE_F32_DIV1000: C2RustUnnamed_23 = 7;
pub const ICHAINTYPE_F32: C2RustUnnamed_23 = 6;
pub const ICHAINTYPE_S32: C2RustUnnamed_23 = 5;
pub const ICHAINTYPE_U32: C2RustUnnamed_23 = 4;
pub const ICHAINTYPE_S16: C2RustUnnamed_23 = 3;
pub const ICHAINTYPE_U16: C2RustUnnamed_23 = 2;
pub const ICHAINTYPE_S8: C2RustUnnamed_23 = 1;
pub const ICHAINTYPE_U8: C2RustUnnamed_23 = 0;
pub type C2RustUnnamed_24 = libc::c_uint;
pub const MTXMODE_APPLY: C2RustUnnamed_24 = 1;
pub const MTXMODE_NEW: C2RustUnnamed_24 = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct BossSst {
    pub actor: Actor,
    pub skelAnime: SkelAnime,
    pub actionFunc: BossSstActionFunc,
    pub actionVar: s8,
    pub ready: s8,
    pub effectMode: u8_0,
    pub timer: s16,
    pub handAngSpeed: s16,
    pub handMaxSpeed: s16,
    pub handZPosMod: s16,
    pub handYRotMod: s16,
    pub amplitude: s16,
    pub targetYaw: s16,
    pub targetRoll: s16,
    pub jointTable: [Vec3s; 45],
    pub morphTable: [Vec3s; 45],
    pub radius: f32_0,
    pub center: Vec3f,
    pub colliderJntSph: ColliderJntSph,
    pub colliderItems: [ColliderJntSphElement; 11],
    pub colliderCyl: ColliderCylinder,
    pub effects: [BossSstEffect; 18],
    pub trailIndex: s16,
    pub trailCount: s16,
    pub handTrails: [BossSstHandTrail; 7],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct BossSstHandTrail {
    pub world: PosRot,
    pub zPosMod: f32_0,
    pub yRotMod: s16,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct BossSstEffect {
    pub pos: Vec3f,
    pub vel: Vec3f,
    pub rot: Vec3s,
    pub scale: u16_0,
    pub move_0: s16,
    pub status: s16,
    pub alpha: u8_0,
}
pub type BossSstActionFunc
    =
    Option<unsafe extern "C" fn(_: *mut BossSst, _: *mut GlobalContext)
               -> ()>;
pub type C2RustUnnamed_25 = libc::c_int;
pub const BONGO_RIGHT_HAND: C2RustUnnamed_25 = 1;
pub const BONGO_LEFT_HAND: C2RustUnnamed_25 = 0;
pub const BONGO_HEAD: C2RustUnnamed_25 = -1;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct BgSstFloor {
    pub dyna: DynaPolyActor,
    pub drumPhase: s16,
    pub drumAmp: s16,
    pub drumHeight: s16,
}
pub type C2RustUnnamed_26 = libc::c_uint;
pub const BONGOFLOOR_HIT: C2RustUnnamed_26 = 1;
pub const BONGOFLOOR_REST: C2RustUnnamed_26 = 0;
pub type C2RustUnnamed_27 = libc::c_int;
pub const WARP_RED: C2RustUnnamed_27 = 10;
pub const WARP_GREEN: C2RustUnnamed_27 = 9;
pub const WARP_ORANGE: C2RustUnnamed_27 = 8;
pub const WARP_UNK_7: C2RustUnnamed_27 = 7;
pub const WARP_DESTINATION: C2RustUnnamed_27 = 6;
pub const WARP_BLUE_RUTO: C2RustUnnamed_27 = 5;
pub const WARP_YELLOW: C2RustUnnamed_27 = 4;
pub const WARP_PURPLE_CRYSTAL: C2RustUnnamed_27 = 3;
pub const WARP_SAGES: C2RustUnnamed_27 = 2;
pub const WARP_CLEAR_FLAG: C2RustUnnamed_27 = 1;
pub const WARP_DUNGEON_CHILD: C2RustUnnamed_27 = 0;
pub const WARP_DUNGEON_ADULT: C2RustUnnamed_27 = -1;
pub const WARP_BLUE_CRYSTAL: C2RustUnnamed_27 = -2;
pub type C2RustUnnamed_28 = libc::c_uint;
pub const HAND_DEATH: C2RustUnnamed_28 = 11;
pub const HAND_BREAK_ICE: C2RustUnnamed_28 = 10;
pub const HAND_FROZEN: C2RustUnnamed_28 = 9;
pub const HAND_DAMAGED: C2RustUnnamed_28 = 8;
pub const HAND_GRAB: C2RustUnnamed_28 = 7;
pub const HAND_CLAP: C2RustUnnamed_28 = 6;
pub const HAND_PUNCH: C2RustUnnamed_28 = 5;
pub const HAND_SWEEP: C2RustUnnamed_28 = 4;
pub const HAND_SLAM: C2RustUnnamed_28 = 3;
pub const HAND_RETREAT: C2RustUnnamed_28 = 2;
pub const HAND_BEAT: C2RustUnnamed_28 = 1;
pub const HAND_WAIT: C2RustUnnamed_28 = 0;
pub type C2RustUnnamed_29 = libc::c_uint;
pub const BONGO_SHADOW: C2RustUnnamed_29 = 3;
pub const BONGO_SHOCKWAVE: C2RustUnnamed_29 = 2;
pub const BONGO_ICE: C2RustUnnamed_29 = 1;
pub const BONGO_NULL: C2RustUnnamed_29 = 0;
static mut sBodyStaticDList: [Gfx; 3] =
    [Gfx{words:
             {
                 let mut init =
                     Gwords{w0:
                                (0xe7 as libc::c_int as u32_0 &
                                     (((0x1 as libc::c_int) <<
                                           8 as libc::c_int) -
                                          1 as libc::c_int) as libc::c_uint)
                                    << 24 as libc::c_int,
                            w1: 0 as libc::c_int as libc::c_uint,};
                 init
             },},
     Gfx{words:
             {
                 let mut init =
                     Gwords{w0:
                                (0xfc as libc::c_int as u32_0 &
                                     (((0x1 as libc::c_int) <<
                                           8 as libc::c_int) -
                                          1 as libc::c_int) as libc::c_uint)
                                    << 24 as libc::c_int |
                                    (((7 as libc::c_int as u32_0 &
                                           (((0x1 as libc::c_int) <<
                                                 4 as libc::c_int) -
                                                1 as libc::c_int) as
                                               libc::c_uint) <<
                                          20 as libc::c_int |
                                          (3 as libc::c_int as u32_0 &
                                               (((0x1 as libc::c_int) <<
                                                     5 as libc::c_int) -
                                                    1 as libc::c_int) as
                                                   libc::c_uint) <<
                                              15 as libc::c_int |
                                          (7 as libc::c_int as u32_0 &
                                               (((0x1 as libc::c_int) <<
                                                     3 as libc::c_int) -
                                                    1 as libc::c_int) as
                                                   libc::c_uint) <<
                                              12 as libc::c_int |
                                          (7 as libc::c_int as u32_0 &
                                               (((0x1 as libc::c_int) <<
                                                     3 as libc::c_int) -
                                                    1 as libc::c_int) as
                                                   libc::c_uint) <<
                                              9 as libc::c_int |
                                          ((0 as libc::c_int as u32_0 &
                                                (((0x1 as libc::c_int) <<
                                                      4 as libc::c_int) -
                                                     1 as libc::c_int) as
                                                    libc::c_uint) <<
                                               5 as libc::c_int |
                                               (4 as libc::c_int as u32_0 &
                                                    (((0x1 as libc::c_int) <<
                                                          5 as libc::c_int) -
                                                         1 as libc::c_int) as
                                                        libc::c_uint) <<
                                                   0 as libc::c_int)) &
                                         (((0x1 as libc::c_int) <<
                                               24 as libc::c_int) -
                                              1 as libc::c_int) as
                                             libc::c_uint) <<
                                        0 as libc::c_int,
                            w1:
                                (5 as libc::c_int as u32_0 &
                                     (((0x1 as libc::c_int) <<
                                           4 as libc::c_int) -
                                          1 as libc::c_int) as libc::c_uint)
                                    << 28 as libc::c_int |
                                    (5 as libc::c_int as u32_0 &
                                         (((0x1 as libc::c_int) <<
                                               3 as libc::c_int) -
                                              1 as libc::c_int) as
                                             libc::c_uint) <<
                                        15 as libc::c_int |
                                    (7 as libc::c_int as u32_0 &
                                         (((0x1 as libc::c_int) <<
                                               3 as libc::c_int) -
                                              1 as libc::c_int) as
                                             libc::c_uint) <<
                                        12 as libc::c_int |
                                    (6 as libc::c_int as u32_0 &
                                         (((0x1 as libc::c_int) <<
                                               3 as libc::c_int) -
                                              1 as libc::c_int) as
                                             libc::c_uint) << 9 as libc::c_int
                                    |
                                    ((31 as libc::c_int as u32_0 &
                                          (((0x1 as libc::c_int) <<
                                                4 as libc::c_int) -
                                               1 as libc::c_int) as
                                              libc::c_uint) <<
                                         24 as libc::c_int |
                                         (7 as libc::c_int as u32_0 &
                                              (((0x1 as libc::c_int) <<
                                                    3 as libc::c_int) -
                                                   1 as libc::c_int) as
                                                  libc::c_uint) <<
                                             21 as libc::c_int |
                                         (7 as libc::c_int as u32_0 &
                                              (((0x1 as libc::c_int) <<
                                                    3 as libc::c_int) -
                                                   1 as libc::c_int) as
                                                  libc::c_uint) <<
                                             18 as libc::c_int |
                                         (31 as libc::c_int as u32_0 &
                                              (((0x1 as libc::c_int) <<
                                                    3 as libc::c_int) -
                                                   1 as libc::c_int) as
                                                  libc::c_uint) <<
                                             6 as libc::c_int |
                                         (7 as libc::c_int as u32_0 &
                                              (((0x1 as libc::c_int) <<
                                                    3 as libc::c_int) -
                                                   1 as libc::c_int) as
                                                  libc::c_uint) <<
                                             3 as libc::c_int |
                                         (0 as libc::c_int as u32_0 &
                                              (((0x1 as libc::c_int) <<
                                                    3 as libc::c_int) -
                                                   1 as libc::c_int) as
                                                  libc::c_uint) <<
                                             0 as libc::c_int),};
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
static mut sHandTrailDList: [Gfx; 4] =
    [Gfx{words:
             {
                 let mut init =
                     Gwords{w0:
                                (0xe7 as libc::c_int as u32_0 &
                                     (((0x1 as libc::c_int) <<
                                           8 as libc::c_int) -
                                          1 as libc::c_int) as libc::c_uint)
                                    << 24 as libc::c_int,
                            w1: 0 as libc::c_int as libc::c_uint,};
                 init
             },},
     Gfx{words:
             {
                 let mut init =
                     Gwords{w0:
                                (0xfc as libc::c_int as u32_0 &
                                     (((0x1 as libc::c_int) <<
                                           8 as libc::c_int) -
                                          1 as libc::c_int) as libc::c_uint)
                                    << 24 as libc::c_int |
                                    (((31 as libc::c_int as u32_0 &
                                           (((0x1 as libc::c_int) <<
                                                 4 as libc::c_int) -
                                                1 as libc::c_int) as
                                               libc::c_uint) <<
                                          20 as libc::c_int |
                                          (31 as libc::c_int as u32_0 &
                                               (((0x1 as libc::c_int) <<
                                                     5 as libc::c_int) -
                                                    1 as libc::c_int) as
                                                   libc::c_uint) <<
                                              15 as libc::c_int |
                                          (7 as libc::c_int as u32_0 &
                                               (((0x1 as libc::c_int) <<
                                                     3 as libc::c_int) -
                                                    1 as libc::c_int) as
                                                   libc::c_uint) <<
                                              12 as libc::c_int |
                                          (7 as libc::c_int as u32_0 &
                                               (((0x1 as libc::c_int) <<
                                                     3 as libc::c_int) -
                                                    1 as libc::c_int) as
                                                   libc::c_uint) <<
                                              9 as libc::c_int |
                                          ((0 as libc::c_int as u32_0 &
                                                (((0x1 as libc::c_int) <<
                                                      4 as libc::c_int) -
                                                     1 as libc::c_int) as
                                                    libc::c_uint) <<
                                               5 as libc::c_int |
                                               (4 as libc::c_int as u32_0 &
                                                    (((0x1 as libc::c_int) <<
                                                          5 as libc::c_int) -
                                                         1 as libc::c_int) as
                                                        libc::c_uint) <<
                                                   0 as libc::c_int)) &
                                         (((0x1 as libc::c_int) <<
                                               24 as libc::c_int) -
                                              1 as libc::c_int) as
                                             libc::c_uint) <<
                                        0 as libc::c_int,
                            w1:
                                (31 as libc::c_int as u32_0 &
                                     (((0x1 as libc::c_int) <<
                                           4 as libc::c_int) -
                                          1 as libc::c_int) as libc::c_uint)
                                    << 28 as libc::c_int |
                                    (3 as libc::c_int as u32_0 &
                                         (((0x1 as libc::c_int) <<
                                               3 as libc::c_int) -
                                              1 as libc::c_int) as
                                             libc::c_uint) <<
                                        15 as libc::c_int |
                                    (7 as libc::c_int as u32_0 &
                                         (((0x1 as libc::c_int) <<
                                               3 as libc::c_int) -
                                              1 as libc::c_int) as
                                             libc::c_uint) <<
                                        12 as libc::c_int |
                                    (3 as libc::c_int as u32_0 &
                                         (((0x1 as libc::c_int) <<
                                               3 as libc::c_int) -
                                              1 as libc::c_int) as
                                             libc::c_uint) << 9 as libc::c_int
                                    |
                                    ((31 as libc::c_int as u32_0 &
                                          (((0x1 as libc::c_int) <<
                                                4 as libc::c_int) -
                                               1 as libc::c_int) as
                                              libc::c_uint) <<
                                         24 as libc::c_int |
                                         (7 as libc::c_int as u32_0 &
                                              (((0x1 as libc::c_int) <<
                                                    3 as libc::c_int) -
                                                   1 as libc::c_int) as
                                                  libc::c_uint) <<
                                             21 as libc::c_int |
                                         (7 as libc::c_int as u32_0 &
                                              (((0x1 as libc::c_int) <<
                                                    3 as libc::c_int) -
                                                   1 as libc::c_int) as
                                                  libc::c_uint) <<
                                             18 as libc::c_int |
                                         (31 as libc::c_int as u32_0 &
                                              (((0x1 as libc::c_int) <<
                                                    3 as libc::c_int) -
                                                   1 as libc::c_int) as
                                                  libc::c_uint) <<
                                             6 as libc::c_int |
                                         (7 as libc::c_int as u32_0 &
                                              (((0x1 as libc::c_int) <<
                                                    3 as libc::c_int) -
                                                   1 as libc::c_int) as
                                                  libc::c_uint) <<
                                             3 as libc::c_int |
                                         (0 as libc::c_int as u32_0 &
                                              (((0x1 as libc::c_int) <<
                                                    3 as libc::c_int) -
                                                   1 as libc::c_int) as
                                                  libc::c_uint) <<
                                             0 as libc::c_int),};
                 init
             },},
     Gfx{words:
             {
                 let mut init =
                     Gwords{w0:
                                (0xe2 as libc::c_int as u32_0 &
                                     (((0x1 as libc::c_int) <<
                                           8 as libc::c_int) -
                                          1 as libc::c_int) as libc::c_uint)
                                    << 24 as libc::c_int |
                                    ((32 as libc::c_int - 3 as libc::c_int -
                                          29 as libc::c_int) as u32_0 &
                                         (((0x1 as libc::c_int) <<
                                               8 as libc::c_int) -
                                              1 as libc::c_int) as
                                             libc::c_uint) << 8 as libc::c_int
                                    |
                                    ((29 as libc::c_int - 1 as libc::c_int) as
                                         u32_0 &
                                         (((0x1 as libc::c_int) <<
                                               8 as libc::c_int) -
                                              1 as libc::c_int) as
                                             libc::c_uint) <<
                                        0 as libc::c_int,
                            w1:
                                (0x8 as libc::c_int | 0x10 as libc::c_int |
                                     0x20 as libc::c_int | 0x40 as libc::c_int
                                     | 0x80 as libc::c_int |
                                     0x100 as libc::c_int |
                                     0x800 as libc::c_int |
                                     0x4000 as libc::c_int |
                                     (0 as libc::c_int) << 30 as libc::c_int |
                                     (3 as libc::c_int) << 26 as libc::c_int |
                                     (0 as libc::c_int) << 22 as libc::c_int |
                                     (2 as libc::c_int) << 18 as libc::c_int |
                                     (0x8 as libc::c_int | 0x10 as libc::c_int
                                          | 0x20 as libc::c_int |
                                          0x40 as libc::c_int |
                                          0x80 as libc::c_int |
                                          0x100 as libc::c_int |
                                          0x800 as libc::c_int |
                                          0x4000 as libc::c_int |
                                          (0 as libc::c_int) <<
                                              28 as libc::c_int |
                                          (0 as libc::c_int) <<
                                              24 as libc::c_int |
                                          (1 as libc::c_int) <<
                                              20 as libc::c_int |
                                          (0 as libc::c_int) <<
                                              16 as libc::c_int)) as
                                    libc::c_uint,};
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
static mut ovl_Boss_SstVtx_00A3F8: [Vtx; 4] =
    [Vtx{v:
             {
                 let mut init =
                     Vtx_t{ob:
                               [400 as libc::c_int as libc::c_short,
                                400 as libc::c_int as libc::c_short,
                                0 as libc::c_int as libc::c_short],
                           flag: 0 as libc::c_int as libc::c_ushort,
                           tc:
                               [2048 as libc::c_int as libc::c_short,
                                0 as libc::c_int as libc::c_short],
                           cn:
                               [255 as libc::c_int as libc::c_uchar,
                                255 as libc::c_int as libc::c_uchar,
                                255 as libc::c_int as libc::c_uchar,
                                255 as libc::c_int as libc::c_uchar],};
                 init
             },},
     Vtx{v:
             {
                 let mut init =
                     Vtx_t{ob:
                               [-(400 as libc::c_int) as libc::c_short,
                                400 as libc::c_int as libc::c_short,
                                0 as libc::c_int as libc::c_short],
                           flag: 0 as libc::c_int as libc::c_ushort,
                           tc:
                               [0 as libc::c_int as libc::c_short,
                                0 as libc::c_int as libc::c_short],
                           cn:
                               [255 as libc::c_int as libc::c_uchar,
                                255 as libc::c_int as libc::c_uchar,
                                255 as libc::c_int as libc::c_uchar,
                                255 as libc::c_int as libc::c_uchar],};
                 init
             },},
     Vtx{v:
             {
                 let mut init =
                     Vtx_t{ob:
                               [-(400 as libc::c_int) as libc::c_short,
                                -(400 as libc::c_int) as libc::c_short,
                                0 as libc::c_int as libc::c_short],
                           flag: 0 as libc::c_int as libc::c_ushort,
                           tc:
                               [0 as libc::c_int as libc::c_short,
                                6144 as libc::c_int as libc::c_short],
                           cn:
                               [255 as libc::c_int as libc::c_uchar,
                                255 as libc::c_int as libc::c_uchar,
                                255 as libc::c_int as libc::c_uchar,
                                255 as libc::c_int as libc::c_uchar],};
                 init
             },},
     Vtx{v:
             {
                 let mut init =
                     Vtx_t{ob:
                               [400 as libc::c_int as libc::c_short,
                                -(400 as libc::c_int) as libc::c_short,
                                0 as libc::c_int as libc::c_short],
                           flag: 0 as libc::c_int as libc::c_ushort,
                           tc:
                               [2048 as libc::c_int as libc::c_short,
                                6144 as libc::c_int as libc::c_short],
                           cn:
                               [255 as libc::c_int as libc::c_uchar,
                                255 as libc::c_int as libc::c_uchar,
                                255 as libc::c_int as libc::c_uchar,
                                255 as libc::c_int as libc::c_uchar],};
                 init
             },}];
static mut ovl_Boss_SstTex_00A438: [u64_0; 128] =
    [0x404040404040404 as libc::c_longlong as u64_0,
     0x404040404040404 as libc::c_longlong as u64_0,
     0x808080808080808 as libc::c_longlong as u64_0,
     0x808080808080808 as libc::c_longlong as u64_0,
     0xc0c0c0c0c0c0c0c as libc::c_longlong as u64_0,
     0xc0c0c0c0c0c0c0c as libc::c_longlong as u64_0,
     0x1010101010101010 as libc::c_longlong as u64_0,
     0x1010101010101010 as libc::c_longlong as u64_0,
     0x1414141414141414 as libc::c_longlong as u64_0,
     0x1414141414141414 as libc::c_longlong as u64_0,
     0x1818181818181818 as libc::c_longlong as u64_0,
     0x1818181818181818 as libc::c_longlong as u64_0,
     0x1c1c1c1c1c1c1c1c as libc::c_longlong as u64_0,
     0x1c1c1c1c1c1c1c1c as libc::c_longlong as u64_0,
     0x2020202020202020 as libc::c_longlong as u64_0,
     0x2020202020202020 as libc::c_longlong as u64_0,
     0x2424242424242424 as libc::c_longlong as u64_0,
     0x2424242424242424 as libc::c_longlong as u64_0,
     0x2828282828282828 as libc::c_longlong as u64_0,
     0x2828282828282828 as libc::c_longlong as u64_0,
     0x2c2c2c2c2c2c2c2c as libc::c_longlong as u64_0,
     0x2c2c2c2c2c2c2c2c as libc::c_longlong as u64_0,
     0x3030303030303030 as libc::c_longlong as u64_0,
     0x3030303030303030 as libc::c_longlong as u64_0,
     0x3434343434343434 as libc::c_longlong as u64_0,
     0x3434343434343434 as libc::c_longlong as u64_0,
     0x3838383838383838 as libc::c_longlong as u64_0,
     0x3838383838383838 as libc::c_longlong as u64_0,
     0x3c3c3c3c3c3c3c3c as libc::c_longlong as u64_0,
     0x3c3c3c3c3c3c3c3c as libc::c_longlong as u64_0,
     0x4040404040404040 as libc::c_longlong as u64_0,
     0x4040404040404040 as libc::c_longlong as u64_0,
     0x4444444444444444 as libc::c_longlong as u64_0,
     0x4444444444444444 as libc::c_longlong as u64_0,
     0x4848484848484848 as libc::c_longlong as u64_0,
     0x4848484848484848 as libc::c_longlong as u64_0,
     0x4c4c4c4c4c4c4c4c as libc::c_longlong as u64_0,
     0x4c4c4c4c4c4c4c4c as libc::c_longlong as u64_0,
     0x5050505050505050 as libc::c_longlong as u64_0,
     0x5050505050505050 as libc::c_longlong as u64_0,
     0x5454545454545454 as libc::c_longlong as u64_0,
     0x5454545454545454 as libc::c_longlong as u64_0,
     0x5858585858585858 as libc::c_longlong as u64_0,
     0x5858585858585858 as libc::c_longlong as u64_0,
     0x5c5c5c5c5c5c5c5c as libc::c_longlong as u64_0,
     0x5c5c5c5c5c5c5c5c as libc::c_longlong as u64_0,
     0x6060606060606060 as libc::c_longlong as u64_0,
     0x6060606060606060 as libc::c_longlong as u64_0,
     0x6464646464646464 as libc::c_longlong as u64_0,
     0x6464646464646464 as libc::c_longlong as u64_0,
     0x6868686868686868 as libc::c_longlong as u64_0,
     0x6868686868686868 as libc::c_longlong as u64_0,
     0x6c6c6c6c6c6c6c6c as libc::c_longlong as u64_0,
     0x6c6c6c6c6c6c6c6c as libc::c_longlong as u64_0,
     0x7070707070707070 as libc::c_longlong as u64_0,
     0x7070707070707070 as libc::c_longlong as u64_0,
     0x7474747474747474 as libc::c_longlong as u64_0,
     0x7474747474747474 as libc::c_longlong as u64_0,
     0x7878787878787878 as libc::c_longlong as u64_0,
     0x7878787878787878 as libc::c_longlong as u64_0,
     0x7c7c7c7c7c7c7c7c as libc::c_longlong as u64_0,
     0x7c7c7c7c7c7c7c7c as libc::c_longlong as u64_0,
     0x8080808080808080 as libc::c_ulonglong,
     0x8080808080808080 as libc::c_ulonglong,
     0x8484848484848484 as libc::c_ulonglong,
     0x8484848484848484 as libc::c_ulonglong,
     0x8888888888888888 as libc::c_ulonglong,
     0x8888888888888888 as libc::c_ulonglong,
     0x8c8c8c8c8c8c8c8c as libc::c_ulonglong,
     0x8c8c8c8c8c8c8c8c as libc::c_ulonglong,
     0x9090909090909090 as libc::c_ulonglong,
     0x9090909090909090 as libc::c_ulonglong,
     0x9494949494949494 as libc::c_ulonglong,
     0x9494949494949494 as libc::c_ulonglong,
     0x9898989898989898 as libc::c_ulonglong,
     0x9898989898989898 as libc::c_ulonglong,
     0x9c9c9c9c9c9c9c9c as libc::c_ulonglong,
     0x9c9c9c9c9c9c9c9c as libc::c_ulonglong,
     0xa0a0a0a0a0a0a0a0 as libc::c_ulonglong,
     0xa0a0a0a0a0a0a0a0 as libc::c_ulonglong,
     0xa4a4a4a4a4a4a4a4 as libc::c_ulonglong,
     0xa4a4a4a4a4a4a4a4 as libc::c_ulonglong,
     0xa8a8a8a8a8a8a8a8 as libc::c_ulonglong,
     0xa8a8a8a8a8a8a8a8 as libc::c_ulonglong,
     0xacacacacacacacac as libc::c_ulonglong,
     0xacacacacacacacac as libc::c_ulonglong,
     0xb0b0b0b0b0b0b0b0 as libc::c_ulonglong,
     0xb0b0b0b0b0b0b0b0 as libc::c_ulonglong,
     0xb4b4b4b4b4b4b4b4 as libc::c_ulonglong,
     0xb4b4b4b4b4b4b4b4 as libc::c_ulonglong,
     0xb8b8b8b8b8b8b8b8 as libc::c_ulonglong,
     0xb8b8b8b8b8b8b8b8 as libc::c_ulonglong,
     0xbcbcbcbcbcbcbcbc as libc::c_ulonglong,
     0xbcbcbcbcbcbcbcbc as libc::c_ulonglong,
     0xc0c0c0c0c0c0c0c0 as libc::c_ulonglong,
     0xc0c0c0c0c0c0c0c0 as libc::c_ulonglong,
     0xc4c4c4c4c4c4c4c4 as libc::c_ulonglong,
     0xc4c4c4c4c4c4c4c4 as libc::c_ulonglong,
     0xc8c8c8c8c8c8c8c8 as libc::c_ulonglong,
     0xc8c8c8c8c8c8c8c8 as libc::c_ulonglong,
     0xcccccccccccccccc as libc::c_ulonglong,
     0xcccccccccccccccc as libc::c_ulonglong,
     0xd0d0d0d0d0d0d0d0 as libc::c_ulonglong,
     0xd0d0d0d0d0d0d0d0 as libc::c_ulonglong,
     0xd4d4d4d4d4d4d4d4 as libc::c_ulonglong,
     0xd4d4d4d4d4d4d4d4 as libc::c_ulonglong,
     0xd8d8d8d8d8d8d8d8 as libc::c_ulonglong,
     0xd8d8d8d8d8d8d8d8 as libc::c_ulonglong,
     0xdcdcdcdcdcdcdcdc as libc::c_ulonglong,
     0xdcdcdcdcdcdcdcdc as libc::c_ulonglong,
     0xe0e0e0e0e0e0e0e0 as libc::c_ulonglong,
     0xe0e0e0e0e0e0e0e0 as libc::c_ulonglong,
     0xe4e4e4e4e4e4e4e4 as libc::c_ulonglong,
     0xe4e4e4e4e4e4e4e4 as libc::c_ulonglong,
     0xe8e8e8e8e8e8e8e8 as libc::c_ulonglong,
     0xe8e8e8e8e8e8e8e8 as libc::c_ulonglong,
     0xecececececececec as libc::c_ulonglong,
     0xecececececececec as libc::c_ulonglong,
     0xf0f0f0f0f0f0f0f0 as libc::c_ulonglong,
     0xf0f0f0f0f0f0f0f0 as libc::c_ulonglong,
     0xf4f4f4f4f4f4f4f4 as libc::c_ulonglong,
     0xf4f4f4f4f4f4f4f4 as libc::c_ulonglong,
     0xf8f8f8f8f8f8f8f8 as libc::c_ulonglong,
     0xf8f8f8f8f8f8f8f8 as libc::c_ulonglong,
     0xfcfcfcfcfcfcfcfc as libc::c_ulonglong,
     0xfcfcfcfcfcfcfcfc as libc::c_ulonglong,
     0xffffffffffffffff as libc::c_ulonglong,
     0xffffffffffffffff as libc::c_ulonglong];
// Initialized in run_static_initializers
static mut sIntroVanishDList: [Gfx; 17] =
    [Gfx{words: Gwords{w0: 0, w1: 0,},}; 17];
static mut ovl_Boss_SstVtx_00A8C0: [Vtx; 3] =
    [Vtx{v:
             {
                 let mut init =
                     Vtx_t{ob:
                               [-(128 as libc::c_int) as libc::c_short,
                                0 as libc::c_int as libc::c_short,
                                75 as libc::c_int as libc::c_short],
                           flag: 0 as libc::c_int as libc::c_ushort,
                           tc:
                               [-(639 as libc::c_int) as libc::c_short,
                                -(127 as libc::c_int) as libc::c_short],
                           cn:
                               [160 as libc::c_int as libc::c_uchar,
                                151 as libc::c_int as libc::c_uchar,
                                205 as libc::c_int as libc::c_uchar,
                                255 as libc::c_int as libc::c_uchar],};
                 init
             },},
     Vtx{v:
             {
                 let mut init =
                     Vtx_t{ob:
                               [129 as libc::c_int as libc::c_short,
                                0 as libc::c_int as libc::c_short,
                                74 as libc::c_int as libc::c_short],
                           flag: 0 as libc::c_int as libc::c_ushort,
                           tc:
                               [1642 as libc::c_int as libc::c_short,
                                -(165 as libc::c_int) as libc::c_short],
                           cn:
                               [160 as libc::c_int as libc::c_uchar,
                                151 as libc::c_int as libc::c_uchar,
                                205 as libc::c_int as libc::c_uchar,
                                255 as libc::c_int as libc::c_uchar],};
                 init
             },},
     Vtx{v:
             {
                 let mut init =
                     Vtx_t{ob:
                               [0 as libc::c_int as libc::c_short,
                                0 as libc::c_int as libc::c_short,
                                -(148 as libc::c_int) as libc::c_short],
                           flag: 0 as libc::c_int as libc::c_ushort,
                           tc:
                               [534 as libc::c_int as libc::c_short,
                                1830 as libc::c_int as libc::c_short],
                           cn:
                               [160 as libc::c_int as libc::c_uchar,
                                151 as libc::c_int as libc::c_uchar,
                                205 as libc::c_int as libc::c_uchar,
                                255 as libc::c_int as libc::c_uchar],};
                 init
             },}];
static mut ovl_Boss_SstTex_00A8F0: [u64_0; 128] =
    [0 as libc::c_int as u64_0,
     0x10609071215222f as libc::c_longlong as u64_0,
     0x3129200d06030100 as libc::c_longlong as u64_0,
     0 as libc::c_int as u64_0, 0x101 as libc::c_int as u64_0,
     0x7203a46526b8591 as libc::c_longlong as u64_0,
     0x968e744c2c1b0f09 as libc::c_ulonglong, 0 as libc::c_int as u64_0,
     0x3040f as libc::c_int as u64_0,
     0x235793b3c9d8e4e9 as libc::c_longlong as u64_0,
     0xe9e7d2b38b704b29 as libc::c_ulonglong,
     0xf04000000000000 as libc::c_longlong as u64_0,
     0x1061538 as libc::c_int as u64_0,
     0x6ea4ddf2fafdfffd as libc::c_longlong as u64_0,
     0xfdfafaefdecaac7c as libc::c_ulonglong,
     0x4317030100000000 as libc::c_longlong as u64_0,
     0x4184382 as libc::c_int as u64_0,
     0xc6e3f7fdffffffff as libc::c_ulonglong,
     0xfffffbfbf8f2eecc as libc::c_ulonglong,
     0x8e4f1a0403000000 as libc::c_ulonglong,
     0x1175190c4 as libc::c_longlong as u64_0,
     0xf1fbfdffffffffff as libc::c_ulonglong,
     0xfffdfffdfdfdf8ec as libc::c_ulonglong,
     0xcc8e461203000000 as libc::c_ulonglong,
     0x10103074ea7cfe9 as libc::c_longlong as u64_0,
     0xf7fdffffffffffff as libc::c_ulonglong,
     0xfffffdfffffdfff8 as libc::c_ulonglong,
     0xeec9813407000000 as libc::c_ulonglong,
     0x107219ee1e7f4 as libc::c_longlong as u64_0,
     0xfafdfffffffffffd as libc::c_ulonglong,
     0xfffdfffffffffffb as libc::c_ulonglong,
     0xf7e4af5c18010000 as libc::c_ulonglong,
     0x3134cd3eceef5 as libc::c_longlong as u64_0,
     0xfdffffffffffffff as libc::c_ulonglong,
     0xffffffffffffffff as libc::c_ulonglong,
     0xfaf2cd8529060000 as libc::c_ulonglong,
     0x93485e0ebf1fa as libc::c_longlong as u64_0,
     0xfffffffffffffffd as libc::c_ulonglong,
     0xfdfffffffffffdfa as libc::c_ulonglong,
     0xfafae4aa490c0000 as libc::c_ulonglong,
     0x31051ace7f4f8fa as libc::c_longlong as u64_0,
     0xfffffffffffffffd as libc::c_ulonglong,
     0xfdfffffffffffbfa as libc::c_ulonglong,
     0xfaf7f1cd6d1a0300 as libc::c_ulonglong,
     0x3156bc7eff7fbfd as libc::c_longlong as u64_0,
     0xfdfffffffffffffd as libc::c_ulonglong,
     0xfffffffffffdfbfb as libc::c_ulonglong,
     0xfaf7f2de90290400 as libc::c_ulonglong,
     0x32088e0fafbfafd as libc::c_longlong as u64_0,
     0xffffffffffffffff as libc::c_ulonglong,
     0xfffffffffffffdfa as libc::c_ulonglong,
     0xf7f1ece3a8430c01 as libc::c_ulonglong,
     0x6269eeefbfbffff as libc::c_longlong as u64_0,
     0xfdffffffffffffff as libc::c_ulonglong,
     0xfffffffffffffdfa as libc::c_ulonglong,
     0xeee6e9e3b3511001 as libc::c_ulonglong,
     0x41da7f4fffdffff as libc::c_longlong as u64_0,
     0xfffffffffffdffff as libc::c_ulonglong,
     0xfffffffffffdfffb as libc::c_ulonglong,
     0xf5f1ebe3b85f1501 as libc::c_ulonglong,
     0x315a7f8fffdfdff as libc::c_longlong as u64_0,
     0xfffffffffffdffff as libc::c_ulonglong,
     0xfffffffffffffffd as libc::c_ulonglong,
     0xfaf4efe7c06b1a09 as libc::c_ulonglong,
     0x315b8fdfffdfdff as libc::c_longlong as u64_0,
     0xfdffffffffffffff as libc::c_ulonglong,
     0xffffffffffffffff as libc::c_ulonglong,
     0xfaf4eee9cc7a1809 as libc::c_ulonglong,
     0xc18cdfffafbfdfd as libc::c_longlong as u64_0,
     0xffffffffffffffff as libc::c_ulonglong,
     0xfffffffffffdffff as libc::c_ulonglong,
     0xfbf4eeebd27d1509 as libc::c_ulonglong,
     0x90fc9f5e9f7fbfd as libc::c_longlong as u64_0,
     0xffffffffffffffff as libc::c_ulonglong,
     0xfffdfffffffffffb as libc::c_ulonglong,
     0xf7efe9e7c6731707 as libc::c_ulonglong,
     0x406b9ecd7f1fbfd as libc::c_longlong as u64_0,
     0xffffffffffffffff as libc::c_ulonglong,
     0xfffdfffffffffffa as libc::c_ulonglong,
     0xf7f1e9e1b55d0f09 as libc::c_ulonglong,
     0x39ef1daeefafd as libc::c_longlong as u64_0,
     0xfdffffffffffffff as libc::c_ulonglong,
     0xfffdfffdfdfdfbfd as libc::c_ulonglong,
     0xf8f1ecd89845090c as libc::c_ulonglong,
     0x35df2eef5f7f7 as libc::c_longlong as u64_0,
     0xfafdffffffffffff as libc::c_ulonglong,
     0xfffbfbfdfbfbfbfa as libc::c_ulonglong,
     0xf5f1efd2872f040a as libc::c_ulonglong,
     0x120c7ebf5f2ec as libc::c_longlong as u64_0,
     0xf4fdfffffffffdfd as libc::c_ulonglong,
     0xfffffdfbfafafaf4 as libc::c_ulonglong,
     0xf2efeec76d230407 as libc::c_ulonglong,
     0x37dcdf2ece6 as libc::c_longlong as u64_0,
     0xf1f8fbffffffffff as libc::c_ulonglong,
     0xfdfdfbfbf8f7f8f4 as libc::c_ulonglong,
     0xecf2e6a84b130a07 as libc::c_ulonglong,
     0x13a9bebebdd as libc::c_longlong as u64_0,
     0xe3f5fdffffffffff as libc::c_ulonglong,
     0xfffdfdfbf8f8f4ef as libc::c_ulonglong,
     0xebebcd762609090d as libc::c_ulonglong,
     0x1762d7e9dd as libc::c_longlong as u64_0,
     0xdef2fbfffffffdff as libc::c_ulonglong,
     0xfffffdfbf8f8f4f2 as libc::c_ulonglong,
     0xefe3a1400d030309 as libc::c_ulonglong,
     0x32082e0e1 as libc::c_longlong as u64_0,
     0xe7f5fffffffffffd as libc::c_ulonglong,
     0xfffbfdfbfaf8f7f1 as libc::c_ulonglong,
     0xe4be661d03000101 as libc::c_ulonglong,
     0x42b96cc as libc::c_int as u64_0,
     0xd8eefbffffffffff as libc::c_ulonglong,
     0xfffdfbfbfafaf7e6 as libc::c_ulonglong,
     0xc377290600000000 as libc::c_ulonglong, 0x32154 as libc::c_int as u64_0,
     0x79b0d8f1f8ffffff as libc::c_longlong as u64_0,
     0xfdfaf7efe6d3b685 as libc::c_ulonglong,
     0x4b1a040000000000 as libc::c_longlong as u64_0,
     0x30a as libc::c_int as u64_0,
     0x183d73aad5ecf4f7 as libc::c_longlong as u64_0,
     0xf2e1cab39e7d5129 as libc::c_ulonglong,
     0xc01000000000000 as libc::c_longlong as u64_0,
     0 as libc::c_int as u64_0, 0x3040c274e6b73 as libc::c_longlong as u64_0,
     0x62544c433a271706 as libc::c_longlong as u64_0,
     0x100000000000000 as libc::c_longlong as u64_0,
     0 as libc::c_int as u64_0, 0x100060f1b15 as libc::c_longlong as u64_0,
     0x100c120f0c090401 as libc::c_longlong as u64_0,
     0 as libc::c_int as u64_0];
// Initialized in run_static_initializers
static mut sShadowDList: [Gfx; 16] = [Gfx{words: Gwords{w0: 0, w1: 0,},}; 16];
static mut sHead: *mut BossSst = 0 as *const BossSst as *mut BossSst;
static mut sHands: [*mut BossSst; 2] =
    [0 as *const BossSst as *mut BossSst; 2];
static mut sFloor: *mut BgSstFloor =
    0 as *const BgSstFloor as *mut BgSstFloor;
static mut sRoomCenter: Vec3f =
    { let mut init = Vec3f{x: -50.0f32, y: 0.0f32, z: 0.0f32,}; init };
static mut sHandOffsets: [Vec3f; 2] = [Vec3f{x: 0., y: 0., z: 0.,}; 2];
static mut sHandYawOffsets: [s16; 2] = [0; 2];
static mut sCutsceneCamera: s16 = 0;
static mut sCameraAt: Vec3f =
    {
        let mut init =
            Vec3f{x: -50.0f32 + 50.0f32,
                  y: 0.0f32 + 0.0f32,
                  z: 0.0f32 + 0.0f32,};
        init
    };
static mut sCameraEye: Vec3f =
    {
        let mut init =
            Vec3f{x: -50.0f32 + 150.0f32,
                  y: 0.0f32 + 100.0f32,
                  z: 0.0f32 + 0.0f32,};
        init
    };
static mut sCameraAtVel: Vec3f =
    { let mut init = Vec3f{x: 0.0f32, y: 0.0f32, z: 0.0f32,}; init };
static mut sCameraEyeVel: Vec3f =
    { let mut init = Vec3f{x: 0.0f32, y: 0.0f32, z: 0.0f32,}; init };
static mut sCameraAtPoints: [Vec3f; 8] =
    [{
         let mut init =
             Vec3f{x: -50.0f32 - 50.0f32,
                   y: 0.0f32 + 300.0f32,
                   z: 0.0f32 + 0.0f32,};
         init
     },
     {
         let mut init =
             Vec3f{x: -50.0f32 + 150.0f32,
                   y: 0.0f32 + 300.0f32,
                   z: 0.0f32 + 100.0f32,};
         init
     },
     {
         let mut init =
             Vec3f{x: -50.0f32 + 0.0f32,
                   y: 0.0f32 + 600.0f32,
                   z: 0.0f32 + 100.0f32,};
         init
     },
     {
         let mut init =
             Vec3f{x: -50.0f32 + 50.0f32,
                   y: 0.0f32 + 400.0f32,
                   z: 0.0f32 + 200.0f32,};
         init
     },
     {
         let mut init =
             Vec3f{x: -50.0f32 + 50.0f32,
                   y: 0.0f32 + 200.0f32,
                   z: 0.0f32 + 200.0f32,};
         init
     },
     {
         let mut init =
             Vec3f{x: -50.0f32 - 50.0f32,
                   y: 0.0f32 + 0.0f32,
                   z: 0.0f32 + 200.0f32,};
         init
     },
     {
         let mut init =
             Vec3f{x: -50.0f32 - 150.0f32,
                   y: 0.0f32 + 0.0f32,
                   z: 0.0f32 + 100.0f32,};
         init
     },
     {
         let mut init =
             Vec3f{x: -50.0f32 - 60.0f32,
                   y: 0.0f32 + 180.0f32,
                   z: 0.0f32 + 730.0f32,};
         init
     }];
static mut sCameraEyePoints: [Vec3f; 8] =
    [{
         let mut init =
             Vec3f{x: -50.0f32 + 250.0f32,
                   y: 0.0f32 + 800.0f32,
                   z: 0.0f32 + 800.0f32,};
         init
     },
     {
         let mut init =
             Vec3f{x: -50.0f32 - 150.0f32,
                   y: 0.0f32 + 700.0f32,
                   z: 0.0f32 + 1400.0f32,};
         init
     },
     {
         let mut init =
             Vec3f{x: -50.0f32 + 250.0f32,
                   y: 0.0f32 + 100.0f32,
                   z: 0.0f32 + 750.0f32,};
         init
     },
     {
         let mut init =
             Vec3f{x: -50.0f32 + 50.0f32,
                   y: 0.0f32 + 200.0f32,
                   z: 0.0f32 + 900.0f32,};
         init
     },
     {
         let mut init =
             Vec3f{x: -50.0f32 + 50.0f32,
                   y: 0.0f32 + 200.0f32,
                   z: 0.0f32 + 900.0f32,};
         init
     },
     {
         let mut init =
             Vec3f{x: -50.0f32 + 350.0f32,
                   y: 0.0f32 + 400.0f32,
                   z: 0.0f32 + 1200.0f32,};
         init
     },
     {
         let mut init =
             Vec3f{x: -50.0f32 - 50.0f32,
                   y: 0.0f32 + 200.0f32,
                   z: 0.0f32 + 800.0f32,};
         init
     },
     {
         let mut init =
             Vec3f{x: -50.0f32 - 50.0f32,
                   y: 0.0f32 + 200.0f32,
                   z: 0.0f32 + 800.0f32,};
         init
     }];
static mut sZeroVec: Vec3f =
    { let mut init = Vec3f{x: 0.0f32, y: 0.0f32, z: 0.0f32,}; init };
static mut sBodyStatic: u32_0 = 0 as libc::c_int as u32_0;
static mut sBodyColor: Color_RGBA8 =
    {
        let mut init =
            Color_RGBA8{r: 255 as libc::c_int as u8_0,
                        g: 255 as libc::c_int as u8_0,
                        b: 255 as libc::c_int as u8_0,
                        a: 255 as libc::c_int as u8_0,};
        init
    };
static mut sStaticColor: Color_RGBA8 =
    {
        let mut init =
            Color_RGBA8{r: 0 as libc::c_int as u8_0,
                        g: 0 as libc::c_int as u8_0,
                        b: 0 as libc::c_int as u8_0,
                        a: 255 as libc::c_int as u8_0,};
        init
    };
static mut sHandState: [s32; 2] =
    [HAND_WAIT as libc::c_int, HAND_WAIT as libc::c_int];
#[no_mangle]
pub static mut Boss_Sst_InitVars: ActorInit =
    unsafe {
        {
            let mut init =
                ActorInit{id: ACTOR_BOSS_SST as libc::c_int as s16,
                          category: ACTORCAT_BOSS as libc::c_int as u8_0,
                          flags:
                              ((1 as libc::c_int) << 0 as libc::c_int |
                                   (1 as libc::c_int) << 2 as libc::c_int |
                                   (1 as libc::c_int) << 4 as libc::c_int |
                                   (1 as libc::c_int) << 5 as libc::c_int |
                                   (1 as libc::c_int) << 10 as libc::c_int) as
                                  u32_0,
                          objectId: OBJECT_SST as libc::c_int as s16,
                          instanceSize:
                              ::std::mem::size_of::<BossSst>() as
                                  libc::c_ulong,
                          init:
                              ::std::mem::transmute::<Option<unsafe extern "C" fn(_:
                                                                                      *mut Actor,
                                                                                  _:
                                                                                      *mut GlobalContext)
                                                                 -> ()>,
                                                      ActorFunc>(Some(BossSst_Init
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
                                                      ActorFunc>(Some(BossSst_Destroy
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
                                                      ActorFunc>(Some(BossSst_UpdateHand
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
                                                      ActorFunc>(Some(BossSst_DrawHand
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
static mut sJntSphItemsInitHand: [ColliderJntSphElementInit; 11] =
    [{
         let mut init =
             ColliderJntSphElementInit{info:
                                           {
                                               let mut init =
                                                   ColliderInfoInit{elemType:
                                                                        ELEMTYPE_UNK1
                                                                            as
                                                                            libc::c_int
                                                                            as
                                                                            u8_0,
                                                                    toucher:
                                                                        {
                                                                            let mut init =
                                                                                ColliderTouch{dmgFlags:
                                                                                                  0x20000000
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
                                                                        ((1 as
                                                                              libc::c_int)
                                                                             <<
                                                                             0
                                                                                 as
                                                                                 libc::c_int
                                                                             |
                                                                             (0
                                                                                  as
                                                                                  libc::c_int)
                                                                                 <<
                                                                                 3
                                                                                     as
                                                                                     libc::c_int)
                                                                            as
                                                                            u8_0,
                                                                    bumperFlags:
                                                                        ((1 as
                                                                              libc::c_int)
                                                                             <<
                                                                             0
                                                                                 as
                                                                                 libc::c_int)
                                                                            as
                                                                            u8_0,
                                                                    ocElemFlags:
                                                                        ((1 as
                                                                              libc::c_int)
                                                                             <<
                                                                             0
                                                                                 as
                                                                                 libc::c_int)
                                                                            as
                                                                            u8_0,};
                                               init
                                           },
                                       dim:
                                           {
                                               let mut init =
                                                   ColliderJntSphElementDimInit{limb:
                                                                                    2
                                                                                        as
                                                                                        libc::c_int
                                                                                        as
                                                                                        u8_0,
                                                                                modelSphere:
                                                                                    {
                                                                                        let mut init =
                                                                                            Sphere16{center:
                                                                                                         {
                                                                                                             let mut init =
                                                                                                                 Vec3s{x:
                                                                                                                           2000
                                                                                                                               as
                                                                                                                               libc::c_int
                                                                                                                               as
                                                                                                                               s16,
                                                                                                                       y:
                                                                                                                           -(1500
                                                                                                                                 as
                                                                                                                                 libc::c_int)
                                                                                                                               as
                                                                                                                               s16,
                                                                                                                       z:
                                                                                                                           250
                                                                                                                               as
                                                                                                                               libc::c_int
                                                                                                                               as
                                                                                                                               s16,};
                                                                                                             init
                                                                                                         },
                                                                                                     radius:
                                                                                                         65
                                                                                                             as
                                                                                                             libc::c_int
                                                                                                             as
                                                                                                             s16,};
                                                                                        init
                                                                                    },
                                                                                scale:
                                                                                    100
                                                                                        as
                                                                                        libc::c_int
                                                                                        as
                                                                                        s16,};
                                               init
                                           },};
         init
     },
     {
         let mut init =
             ColliderJntSphElementInit{info:
                                           {
                                               let mut init =
                                                   ColliderInfoInit{elemType:
                                                                        ELEMTYPE_UNK1
                                                                            as
                                                                            libc::c_int
                                                                            as
                                                                            u8_0,
                                                                    toucher:
                                                                        {
                                                                            let mut init =
                                                                                ColliderTouch{dmgFlags:
                                                                                                  0x20000000
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
                                                                        ((1 as
                                                                              libc::c_int)
                                                                             <<
                                                                             0
                                                                                 as
                                                                                 libc::c_int
                                                                             |
                                                                             (0
                                                                                  as
                                                                                  libc::c_int)
                                                                                 <<
                                                                                 3
                                                                                     as
                                                                                     libc::c_int)
                                                                            as
                                                                            u8_0,
                                                                    bumperFlags:
                                                                        ((1 as
                                                                              libc::c_int)
                                                                             <<
                                                                             0
                                                                                 as
                                                                                 libc::c_int)
                                                                            as
                                                                            u8_0,
                                                                    ocElemFlags:
                                                                        ((1 as
                                                                              libc::c_int)
                                                                             <<
                                                                             0
                                                                                 as
                                                                                 libc::c_int)
                                                                            as
                                                                            u8_0,};
                                               init
                                           },
                                       dim:
                                           {
                                               let mut init =
                                                   ColliderJntSphElementDimInit{limb:
                                                                                    10
                                                                                        as
                                                                                        libc::c_int
                                                                                        as
                                                                                        u8_0,
                                                                                modelSphere:
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
                                                                                                         22
                                                                                                             as
                                                                                                             libc::c_int
                                                                                                             as
                                                                                                             s16,};
                                                                                        init
                                                                                    },
                                                                                scale:
                                                                                    100
                                                                                        as
                                                                                        libc::c_int
                                                                                        as
                                                                                        s16,};
                                               init
                                           },};
         init
     },
     {
         let mut init =
             ColliderJntSphElementInit{info:
                                           {
                                               let mut init =
                                                   ColliderInfoInit{elemType:
                                                                        ELEMTYPE_UNK1
                                                                            as
                                                                            libc::c_int
                                                                            as
                                                                            u8_0,
                                                                    toucher:
                                                                        {
                                                                            let mut init =
                                                                                ColliderTouch{dmgFlags:
                                                                                                  0x20000000
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
                                                                        ((1 as
                                                                              libc::c_int)
                                                                             <<
                                                                             0
                                                                                 as
                                                                                 libc::c_int
                                                                             |
                                                                             (0
                                                                                  as
                                                                                  libc::c_int)
                                                                                 <<
                                                                                 3
                                                                                     as
                                                                                     libc::c_int)
                                                                            as
                                                                            u8_0,
                                                                    bumperFlags:
                                                                        ((1 as
                                                                              libc::c_int)
                                                                             <<
                                                                             0
                                                                                 as
                                                                                 libc::c_int)
                                                                            as
                                                                            u8_0,
                                                                    ocElemFlags:
                                                                        ((1 as
                                                                              libc::c_int)
                                                                             <<
                                                                             0
                                                                                 as
                                                                                 libc::c_int)
                                                                            as
                                                                            u8_0,};
                                               init
                                           },
                                       dim:
                                           {
                                               let mut init =
                                                   ColliderJntSphElementDimInit{limb:
                                                                                    11
                                                                                        as
                                                                                        libc::c_int
                                                                                        as
                                                                                        u8_0,
                                                                                modelSphere:
                                                                                    {
                                                                                        let mut init =
                                                                                            Sphere16{center:
                                                                                                         {
                                                                                                             let mut init =
                                                                                                                 Vec3s{x:
                                                                                                                           500
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
                                                                                                         22
                                                                                                             as
                                                                                                             libc::c_int
                                                                                                             as
                                                                                                             s16,};
                                                                                        init
                                                                                    },
                                                                                scale:
                                                                                    100
                                                                                        as
                                                                                        libc::c_int
                                                                                        as
                                                                                        s16,};
                                               init
                                           },};
         init
     },
     {
         let mut init =
             ColliderJntSphElementInit{info:
                                           {
                                               let mut init =
                                                   ColliderInfoInit{elemType:
                                                                        ELEMTYPE_UNK1
                                                                            as
                                                                            libc::c_int
                                                                            as
                                                                            u8_0,
                                                                    toucher:
                                                                        {
                                                                            let mut init =
                                                                                ColliderTouch{dmgFlags:
                                                                                                  0x20000000
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
                                                                        ((1 as
                                                                              libc::c_int)
                                                                             <<
                                                                             0
                                                                                 as
                                                                                 libc::c_int
                                                                             |
                                                                             (0
                                                                                  as
                                                                                  libc::c_int)
                                                                                 <<
                                                                                 3
                                                                                     as
                                                                                     libc::c_int)
                                                                            as
                                                                            u8_0,
                                                                    bumperFlags:
                                                                        ((1 as
                                                                              libc::c_int)
                                                                             <<
                                                                             0
                                                                                 as
                                                                                 libc::c_int)
                                                                            as
                                                                            u8_0,
                                                                    ocElemFlags:
                                                                        ((1 as
                                                                              libc::c_int)
                                                                             <<
                                                                             0
                                                                                 as
                                                                                 libc::c_int)
                                                                            as
                                                                            u8_0,};
                                               init
                                           },
                                       dim:
                                           {
                                               let mut init =
                                                   ColliderJntSphElementDimInit{limb:
                                                                                    15
                                                                                        as
                                                                                        libc::c_int
                                                                                        as
                                                                                        u8_0,
                                                                                modelSphere:
                                                                                    {
                                                                                        let mut init =
                                                                                            Sphere16{center:
                                                                                                         {
                                                                                                             let mut init =
                                                                                                                 Vec3s{x:
                                                                                                                           -(250
                                                                                                                                 as
                                                                                                                                 libc::c_int)
                                                                                                                               as
                                                                                                                               s16,
                                                                                                                       y:
                                                                                                                           -(250
                                                                                                                                 as
                                                                                                                                 libc::c_int)
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
                                                                                                         25
                                                                                                             as
                                                                                                             libc::c_int
                                                                                                             as
                                                                                                             s16,};
                                                                                        init
                                                                                    },
                                                                                scale:
                                                                                    100
                                                                                        as
                                                                                        libc::c_int
                                                                                        as
                                                                                        s16,};
                                               init
                                           },};
         init
     },
     {
         let mut init =
             ColliderJntSphElementInit{info:
                                           {
                                               let mut init =
                                                   ColliderInfoInit{elemType:
                                                                        ELEMTYPE_UNK1
                                                                            as
                                                                            libc::c_int
                                                                            as
                                                                            u8_0,
                                                                    toucher:
                                                                        {
                                                                            let mut init =
                                                                                ColliderTouch{dmgFlags:
                                                                                                  0x20000000
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
                                                                        ((1 as
                                                                              libc::c_int)
                                                                             <<
                                                                             0
                                                                                 as
                                                                                 libc::c_int
                                                                             |
                                                                             (0
                                                                                  as
                                                                                  libc::c_int)
                                                                                 <<
                                                                                 3
                                                                                     as
                                                                                     libc::c_int)
                                                                            as
                                                                            u8_0,
                                                                    bumperFlags:
                                                                        ((1 as
                                                                              libc::c_int)
                                                                             <<
                                                                             0
                                                                                 as
                                                                                 libc::c_int)
                                                                            as
                                                                            u8_0,
                                                                    ocElemFlags:
                                                                        ((1 as
                                                                              libc::c_int)
                                                                             <<
                                                                             0
                                                                                 as
                                                                                 libc::c_int)
                                                                            as
                                                                            u8_0,};
                                               init
                                           },
                                       dim:
                                           {
                                               let mut init =
                                                   ColliderJntSphElementDimInit{limb:
                                                                                    16
                                                                                        as
                                                                                        libc::c_int
                                                                                        as
                                                                                        u8_0,
                                                                                modelSphere:
                                                                                    {
                                                                                        let mut init =
                                                                                            Sphere16{center:
                                                                                                         {
                                                                                                             let mut init =
                                                                                                                 Vec3s{x:
                                                                                                                           500
                                                                                                                               as
                                                                                                                               libc::c_int
                                                                                                                               as
                                                                                                                               s16,
                                                                                                                       y:
                                                                                                                           -(250
                                                                                                                                 as
                                                                                                                                 libc::c_int)
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
                                                                                                         25
                                                                                                             as
                                                                                                             libc::c_int
                                                                                                             as
                                                                                                             s16,};
                                                                                        init
                                                                                    },
                                                                                scale:
                                                                                    100
                                                                                        as
                                                                                        libc::c_int
                                                                                        as
                                                                                        s16,};
                                               init
                                           },};
         init
     },
     {
         let mut init =
             ColliderJntSphElementInit{info:
                                           {
                                               let mut init =
                                                   ColliderInfoInit{elemType:
                                                                        ELEMTYPE_UNK1
                                                                            as
                                                                            libc::c_int
                                                                            as
                                                                            u8_0,
                                                                    toucher:
                                                                        {
                                                                            let mut init =
                                                                                ColliderTouch{dmgFlags:
                                                                                                  0x20000000
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
                                                                        ((1 as
                                                                              libc::c_int)
                                                                             <<
                                                                             0
                                                                                 as
                                                                                 libc::c_int
                                                                             |
                                                                             (0
                                                                                  as
                                                                                  libc::c_int)
                                                                                 <<
                                                                                 3
                                                                                     as
                                                                                     libc::c_int)
                                                                            as
                                                                            u8_0,
                                                                    bumperFlags:
                                                                        ((1 as
                                                                              libc::c_int)
                                                                             <<
                                                                             0
                                                                                 as
                                                                                 libc::c_int)
                                                                            as
                                                                            u8_0,
                                                                    ocElemFlags:
                                                                        ((1 as
                                                                              libc::c_int)
                                                                             <<
                                                                             0
                                                                                 as
                                                                                 libc::c_int)
                                                                            as
                                                                            u8_0,};
                                               init
                                           },
                                       dim:
                                           {
                                               let mut init =
                                                   ColliderJntSphElementDimInit{limb:
                                                                                    20
                                                                                        as
                                                                                        libc::c_int
                                                                                        as
                                                                                        u8_0,
                                                                                modelSphere:
                                                                                    {
                                                                                        let mut init =
                                                                                            Sphere16{center:
                                                                                                         {
                                                                                                             let mut init =
                                                                                                                 Vec3s{x:
                                                                                                                           250
                                                                                                                               as
                                                                                                                               libc::c_int
                                                                                                                               as
                                                                                                                               s16,
                                                                                                                       y:
                                                                                                                           -(250
                                                                                                                                 as
                                                                                                                                 libc::c_int)
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
                                                                                                         25
                                                                                                             as
                                                                                                             libc::c_int
                                                                                                             as
                                                                                                             s16,};
                                                                                        init
                                                                                    },
                                                                                scale:
                                                                                    100
                                                                                        as
                                                                                        libc::c_int
                                                                                        as
                                                                                        s16,};
                                               init
                                           },};
         init
     },
     {
         let mut init =
             ColliderJntSphElementInit{info:
                                           {
                                               let mut init =
                                                   ColliderInfoInit{elemType:
                                                                        ELEMTYPE_UNK1
                                                                            as
                                                                            libc::c_int
                                                                            as
                                                                            u8_0,
                                                                    toucher:
                                                                        {
                                                                            let mut init =
                                                                                ColliderTouch{dmgFlags:
                                                                                                  0x20000000
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
                                                                        ((1 as
                                                                              libc::c_int)
                                                                             <<
                                                                             0
                                                                                 as
                                                                                 libc::c_int
                                                                             |
                                                                             (0
                                                                                  as
                                                                                  libc::c_int)
                                                                                 <<
                                                                                 3
                                                                                     as
                                                                                     libc::c_int)
                                                                            as
                                                                            u8_0,
                                                                    bumperFlags:
                                                                        ((1 as
                                                                              libc::c_int)
                                                                             <<
                                                                             0
                                                                                 as
                                                                                 libc::c_int)
                                                                            as
                                                                            u8_0,
                                                                    ocElemFlags:
                                                                        ((1 as
                                                                              libc::c_int)
                                                                             <<
                                                                             0
                                                                                 as
                                                                                 libc::c_int)
                                                                            as
                                                                            u8_0,};
                                               init
                                           },
                                       dim:
                                           {
                                               let mut init =
                                                   ColliderJntSphElementDimInit{limb:
                                                                                    21
                                                                                        as
                                                                                        libc::c_int
                                                                                        as
                                                                                        u8_0,
                                                                                modelSphere:
                                                                                    {
                                                                                        let mut init =
                                                                                            Sphere16{center:
                                                                                                         {
                                                                                                             let mut init =
                                                                                                                 Vec3s{x:
                                                                                                                           500
                                                                                                                               as
                                                                                                                               libc::c_int
                                                                                                                               as
                                                                                                                               s16,
                                                                                                                       y:
                                                                                                                           -(250
                                                                                                                                 as
                                                                                                                                 libc::c_int)
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
                                                                                                         25
                                                                                                             as
                                                                                                             libc::c_int
                                                                                                             as
                                                                                                             s16,};
                                                                                        init
                                                                                    },
                                                                                scale:
                                                                                    100
                                                                                        as
                                                                                        libc::c_int
                                                                                        as
                                                                                        s16,};
                                               init
                                           },};
         init
     },
     {
         let mut init =
             ColliderJntSphElementInit{info:
                                           {
                                               let mut init =
                                                   ColliderInfoInit{elemType:
                                                                        ELEMTYPE_UNK1
                                                                            as
                                                                            libc::c_int
                                                                            as
                                                                            u8_0,
                                                                    toucher:
                                                                        {
                                                                            let mut init =
                                                                                ColliderTouch{dmgFlags:
                                                                                                  0x20000000
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
                                                                        ((1 as
                                                                              libc::c_int)
                                                                             <<
                                                                             0
                                                                                 as
                                                                                 libc::c_int
                                                                             |
                                                                             (0
                                                                                  as
                                                                                  libc::c_int)
                                                                                 <<
                                                                                 3
                                                                                     as
                                                                                     libc::c_int)
                                                                            as
                                                                            u8_0,
                                                                    bumperFlags:
                                                                        ((1 as
                                                                              libc::c_int)
                                                                             <<
                                                                             0
                                                                                 as
                                                                                 libc::c_int)
                                                                            as
                                                                            u8_0,
                                                                    ocElemFlags:
                                                                        ((1 as
                                                                              libc::c_int)
                                                                             <<
                                                                             0
                                                                                 as
                                                                                 libc::c_int)
                                                                            as
                                                                            u8_0,};
                                               init
                                           },
                                       dim:
                                           {
                                               let mut init =
                                                   ColliderJntSphElementDimInit{limb:
                                                                                    25
                                                                                        as
                                                                                        libc::c_int
                                                                                        as
                                                                                        u8_0,
                                                                                modelSphere:
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
                                                                                                         27
                                                                                                             as
                                                                                                             libc::c_int
                                                                                                             as
                                                                                                             s16,};
                                                                                        init
                                                                                    },
                                                                                scale:
                                                                                    100
                                                                                        as
                                                                                        libc::c_int
                                                                                        as
                                                                                        s16,};
                                               init
                                           },};
         init
     },
     {
         let mut init =
             ColliderJntSphElementInit{info:
                                           {
                                               let mut init =
                                                   ColliderInfoInit{elemType:
                                                                        ELEMTYPE_UNK1
                                                                            as
                                                                            libc::c_int
                                                                            as
                                                                            u8_0,
                                                                    toucher:
                                                                        {
                                                                            let mut init =
                                                                                ColliderTouch{dmgFlags:
                                                                                                  0x20000000
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
                                                                        ((1 as
                                                                              libc::c_int)
                                                                             <<
                                                                             0
                                                                                 as
                                                                                 libc::c_int
                                                                             |
                                                                             (0
                                                                                  as
                                                                                  libc::c_int)
                                                                                 <<
                                                                                 3
                                                                                     as
                                                                                     libc::c_int)
                                                                            as
                                                                            u8_0,
                                                                    bumperFlags:
                                                                        ((1 as
                                                                              libc::c_int)
                                                                             <<
                                                                             0
                                                                                 as
                                                                                 libc::c_int)
                                                                            as
                                                                            u8_0,
                                                                    ocElemFlags:
                                                                        ((1 as
                                                                              libc::c_int)
                                                                             <<
                                                                             0
                                                                                 as
                                                                                 libc::c_int)
                                                                            as
                                                                            u8_0,};
                                               init
                                           },
                                       dim:
                                           {
                                               let mut init =
                                                   ColliderJntSphElementDimInit{limb:
                                                                                    26
                                                                                        as
                                                                                        libc::c_int
                                                                                        as
                                                                                        u8_0,
                                                                                modelSphere:
                                                                                    {
                                                                                        let mut init =
                                                                                            Sphere16{center:
                                                                                                         {
                                                                                                             let mut init =
                                                                                                                 Vec3s{x:
                                                                                                                           750
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
                                                                                                         26
                                                                                                             as
                                                                                                             libc::c_int
                                                                                                             as
                                                                                                             s16,};
                                                                                        init
                                                                                    },
                                                                                scale:
                                                                                    100
                                                                                        as
                                                                                        libc::c_int
                                                                                        as
                                                                                        s16,};
                                               init
                                           },};
         init
     },
     {
         let mut init =
             ColliderJntSphElementInit{info:
                                           {
                                               let mut init =
                                                   ColliderInfoInit{elemType:
                                                                        ELEMTYPE_UNK1
                                                                            as
                                                                            libc::c_int
                                                                            as
                                                                            u8_0,
                                                                    toucher:
                                                                        {
                                                                            let mut init =
                                                                                ColliderTouch{dmgFlags:
                                                                                                  0x20000000
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
                                                                        ((1 as
                                                                              libc::c_int)
                                                                             <<
                                                                             0
                                                                                 as
                                                                                 libc::c_int
                                                                             |
                                                                             (0
                                                                                  as
                                                                                  libc::c_int)
                                                                                 <<
                                                                                 3
                                                                                     as
                                                                                     libc::c_int)
                                                                            as
                                                                            u8_0,
                                                                    bumperFlags:
                                                                        ((1 as
                                                                              libc::c_int)
                                                                             <<
                                                                             0
                                                                                 as
                                                                                 libc::c_int)
                                                                            as
                                                                            u8_0,
                                                                    ocElemFlags:
                                                                        ((1 as
                                                                              libc::c_int)
                                                                             <<
                                                                             0
                                                                                 as
                                                                                 libc::c_int)
                                                                            as
                                                                            u8_0,};
                                               init
                                           },
                                       dim:
                                           {
                                               let mut init =
                                                   ColliderJntSphElementDimInit{limb:
                                                                                    5
                                                                                        as
                                                                                        libc::c_int
                                                                                        as
                                                                                        u8_0,
                                                                                modelSphere:
                                                                                    {
                                                                                        let mut init =
                                                                                            Sphere16{center:
                                                                                                         {
                                                                                                             let mut init =
                                                                                                                 Vec3s{x:
                                                                                                                           750
                                                                                                                               as
                                                                                                                               libc::c_int
                                                                                                                               as
                                                                                                                               s16,
                                                                                                                       y:
                                                                                                                           -(150
                                                                                                                                 as
                                                                                                                                 libc::c_int)
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
                                                                                                         21
                                                                                                             as
                                                                                                             libc::c_int
                                                                                                             as
                                                                                                             s16,};
                                                                                        init
                                                                                    },
                                                                                scale:
                                                                                    100
                                                                                        as
                                                                                        libc::c_int
                                                                                        as
                                                                                        s16,};
                                               init
                                           },};
         init
     },
     {
         let mut init =
             ColliderJntSphElementInit{info:
                                           {
                                               let mut init =
                                                   ColliderInfoInit{elemType:
                                                                        ELEMTYPE_UNK1
                                                                            as
                                                                            libc::c_int
                                                                            as
                                                                            u8_0,
                                                                    toucher:
                                                                        {
                                                                            let mut init =
                                                                                ColliderTouch{dmgFlags:
                                                                                                  0x20000000
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
                                                                        ((1 as
                                                                              libc::c_int)
                                                                             <<
                                                                             0
                                                                                 as
                                                                                 libc::c_int
                                                                             |
                                                                             (0
                                                                                  as
                                                                                  libc::c_int)
                                                                                 <<
                                                                                 3
                                                                                     as
                                                                                     libc::c_int)
                                                                            as
                                                                            u8_0,
                                                                    bumperFlags:
                                                                        ((1 as
                                                                              libc::c_int)
                                                                             <<
                                                                             0
                                                                                 as
                                                                                 libc::c_int)
                                                                            as
                                                                            u8_0,
                                                                    ocElemFlags:
                                                                        ((1 as
                                                                              libc::c_int)
                                                                             <<
                                                                             0
                                                                                 as
                                                                                 libc::c_int)
                                                                            as
                                                                            u8_0,};
                                               init
                                           },
                                       dim:
                                           {
                                               let mut init =
                                                   ColliderJntSphElementDimInit{limb:
                                                                                    6
                                                                                        as
                                                                                        libc::c_int
                                                                                        as
                                                                                        u8_0,
                                                                                modelSphere:
                                                                                    {
                                                                                        let mut init =
                                                                                            Sphere16{center:
                                                                                                         {
                                                                                                             let mut init =
                                                                                                                 Vec3s{x:
                                                                                                                           750
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
                                                                                                         20
                                                                                                             as
                                                                                                             libc::c_int
                                                                                                             as
                                                                                                             s16,};
                                                                                        init
                                                                                    },
                                                                                scale:
                                                                                    100
                                                                                        as
                                                                                        libc::c_int
                                                                                        as
                                                                                        s16,};
                                               init
                                           },};
         init
     }];
static mut sJntSphInitHand: ColliderJntSphInit =
    unsafe {
        {
            let mut init =
                ColliderJntSphInit{base:
                                       {
                                           let mut init =
                                               ColliderInit{colType:
                                                                COLTYPE_HIT0
                                                                    as
                                                                    libc::c_int
                                                                    as u8_0,
                                                            atFlags:
                                                                ((1 as
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
                                                                     3 as
                                                                         libc::c_int
                                                                     |
                                                                     (1 as
                                                                          libc::c_int)
                                                                         <<
                                                                         4 as
                                                                             libc::c_int
                                                                     |
                                                                     (1 as
                                                                          libc::c_int)
                                                                         <<
                                                                         5 as
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
                                                                COLSHAPE_JNTSPH
                                                                    as
                                                                    libc::c_int
                                                                    as u8_0,};
                                           init
                                       },
                                   count: 11 as libc::c_int,
                                   elements:
                                       sJntSphItemsInitHand.as_ptr() as
                                           *mut _,};
            init
        }
    };
static mut sJntSphItemsInitHead: [ColliderJntSphElementInit; 11] =
    [{
         let mut init =
             ColliderJntSphElementInit{info:
                                           {
                                               let mut init =
                                                   ColliderInfoInit{elemType:
                                                                        ELEMTYPE_UNK1
                                                                            as
                                                                            libc::c_int
                                                                            as
                                                                            u8_0,
                                                                    toucher:
                                                                        {
                                                                            let mut init =
                                                                                ColliderTouch{dmgFlags:
                                                                                                  0x20000000
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
                                                                             0
                                                                                 as
                                                                                 libc::c_int
                                                                             |
                                                                             (0
                                                                                  as
                                                                                  libc::c_int)
                                                                                 <<
                                                                                 3
                                                                                     as
                                                                                     libc::c_int)
                                                                            as
                                                                            u8_0,
                                                                    bumperFlags:
                                                                        ((1 as
                                                                              libc::c_int)
                                                                             <<
                                                                             0
                                                                                 as
                                                                                 libc::c_int)
                                                                            as
                                                                            u8_0,
                                                                    ocElemFlags:
                                                                        ((1 as
                                                                              libc::c_int)
                                                                             <<
                                                                             0
                                                                                 as
                                                                                 libc::c_int)
                                                                            as
                                                                            u8_0,};
                                               init
                                           },
                                       dim:
                                           {
                                               let mut init =
                                                   ColliderJntSphElementDimInit{limb:
                                                                                    7
                                                                                        as
                                                                                        libc::c_int
                                                                                        as
                                                                                        u8_0,
                                                                                modelSphere:
                                                                                    {
                                                                                        let mut init =
                                                                                            Sphere16{center:
                                                                                                         {
                                                                                                             let mut init =
                                                                                                                 Vec3s{x:
                                                                                                                           1500
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
                                                                                                         70
                                                                                                             as
                                                                                                             libc::c_int
                                                                                                             as
                                                                                                             s16,};
                                                                                        init
                                                                                    },
                                                                                scale:
                                                                                    100
                                                                                        as
                                                                                        libc::c_int
                                                                                        as
                                                                                        s16,};
                                               init
                                           },};
         init
     },
     {
         let mut init =
             ColliderJntSphElementInit{info:
                                           {
                                               let mut init =
                                                   ColliderInfoInit{elemType:
                                                                        ELEMTYPE_UNK1
                                                                            as
                                                                            libc::c_int
                                                                            as
                                                                            u8_0,
                                                                    toucher:
                                                                        {
                                                                            let mut init =
                                                                                ColliderTouch{dmgFlags:
                                                                                                  0x20000000
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
                                                                             0
                                                                                 as
                                                                                 libc::c_int
                                                                             |
                                                                             (0
                                                                                  as
                                                                                  libc::c_int)
                                                                                 <<
                                                                                 3
                                                                                     as
                                                                                     libc::c_int)
                                                                            as
                                                                            u8_0,
                                                                    bumperFlags:
                                                                        ((1 as
                                                                              libc::c_int)
                                                                             <<
                                                                             0
                                                                                 as
                                                                                 libc::c_int)
                                                                            as
                                                                            u8_0,
                                                                    ocElemFlags:
                                                                        ((1 as
                                                                              libc::c_int)
                                                                             <<
                                                                             0
                                                                                 as
                                                                                 libc::c_int)
                                                                            as
                                                                            u8_0,};
                                               init
                                           },
                                       dim:
                                           {
                                               let mut init =
                                                   ColliderJntSphElementDimInit{limb:
                                                                                    6
                                                                                        as
                                                                                        libc::c_int
                                                                                        as
                                                                                        u8_0,
                                                                                modelSphere:
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
                                                                                                         75
                                                                                                             as
                                                                                                             libc::c_int
                                                                                                             as
                                                                                                             s16,};
                                                                                        init
                                                                                    },
                                                                                scale:
                                                                                    100
                                                                                        as
                                                                                        libc::c_int
                                                                                        as
                                                                                        s16,};
                                               init
                                           },};
         init
     },
     {
         let mut init =
             ColliderJntSphElementInit{info:
                                           {
                                               let mut init =
                                                   ColliderInfoInit{elemType:
                                                                        ELEMTYPE_UNK1
                                                                            as
                                                                            libc::c_int
                                                                            as
                                                                            u8_0,
                                                                    toucher:
                                                                        {
                                                                            let mut init =
                                                                                ColliderTouch{dmgFlags:
                                                                                                  0x20000000
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
                                                                             0
                                                                                 as
                                                                                 libc::c_int
                                                                             |
                                                                             (0
                                                                                  as
                                                                                  libc::c_int)
                                                                                 <<
                                                                                 3
                                                                                     as
                                                                                     libc::c_int)
                                                                            as
                                                                            u8_0,
                                                                    bumperFlags:
                                                                        ((1 as
                                                                              libc::c_int)
                                                                             <<
                                                                             0
                                                                                 as
                                                                                 libc::c_int)
                                                                            as
                                                                            u8_0,
                                                                    ocElemFlags:
                                                                        ((1 as
                                                                              libc::c_int)
                                                                             <<
                                                                             0
                                                                                 as
                                                                                 libc::c_int)
                                                                            as
                                                                            u8_0,};
                                               init
                                           },
                                       dim:
                                           {
                                               let mut init =
                                                   ColliderJntSphElementDimInit{limb:
                                                                                    4
                                                                                        as
                                                                                        libc::c_int
                                                                                        as
                                                                                        u8_0,
                                                                                modelSphere:
                                                                                    {
                                                                                        let mut init =
                                                                                            Sphere16{center:
                                                                                                         {
                                                                                                             let mut init =
                                                                                                                 Vec3s{x:
                                                                                                                           5000
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
                                                                                                         120
                                                                                                             as
                                                                                                             libc::c_int
                                                                                                             as
                                                                                                             s16,};
                                                                                        init
                                                                                    },
                                                                                scale:
                                                                                    100
                                                                                        as
                                                                                        libc::c_int
                                                                                        as
                                                                                        s16,};
                                               init
                                           },};
         init
     },
     {
         let mut init =
             ColliderJntSphElementInit{info:
                                           {
                                               let mut init =
                                                   ColliderInfoInit{elemType:
                                                                        ELEMTYPE_UNK1
                                                                            as
                                                                            libc::c_int
                                                                            as
                                                                            u8_0,
                                                                    toucher:
                                                                        {
                                                                            let mut init =
                                                                                ColliderTouch{dmgFlags:
                                                                                                  0x20000000
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
                                                                             0
                                                                                 as
                                                                                 libc::c_int
                                                                             |
                                                                             (0
                                                                                  as
                                                                                  libc::c_int)
                                                                                 <<
                                                                                 3
                                                                                     as
                                                                                     libc::c_int)
                                                                            as
                                                                            u8_0,
                                                                    bumperFlags:
                                                                        ((1 as
                                                                              libc::c_int)
                                                                             <<
                                                                             0
                                                                                 as
                                                                                 libc::c_int)
                                                                            as
                                                                            u8_0,
                                                                    ocElemFlags:
                                                                        ((1 as
                                                                              libc::c_int)
                                                                             <<
                                                                             0
                                                                                 as
                                                                                 libc::c_int)
                                                                            as
                                                                            u8_0,};
                                               init
                                           },
                                       dim:
                                           {
                                               let mut init =
                                                   ColliderJntSphElementDimInit{limb:
                                                                                    3
                                                                                        as
                                                                                        libc::c_int
                                                                                        as
                                                                                        u8_0,
                                                                                modelSphere:
                                                                                    {
                                                                                        let mut init =
                                                                                            Sphere16{center:
                                                                                                         {
                                                                                                             let mut init =
                                                                                                                 Vec3s{x:
                                                                                                                           -(2500
                                                                                                                                 as
                                                                                                                                 libc::c_int)
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
                                                                                                         150
                                                                                                             as
                                                                                                             libc::c_int
                                                                                                             as
                                                                                                             s16,};
                                                                                        init
                                                                                    },
                                                                                scale:
                                                                                    100
                                                                                        as
                                                                                        libc::c_int
                                                                                        as
                                                                                        s16,};
                                               init
                                           },};
         init
     },
     {
         let mut init =
             ColliderJntSphElementInit{info:
                                           {
                                               let mut init =
                                                   ColliderInfoInit{elemType:
                                                                        ELEMTYPE_UNK1
                                                                            as
                                                                            libc::c_int
                                                                            as
                                                                            u8_0,
                                                                    toucher:
                                                                        {
                                                                            let mut init =
                                                                                ColliderTouch{dmgFlags:
                                                                                                  0x20000000
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
                                                                             0
                                                                                 as
                                                                                 libc::c_int
                                                                             |
                                                                             (0
                                                                                  as
                                                                                  libc::c_int)
                                                                                 <<
                                                                                 3
                                                                                     as
                                                                                     libc::c_int)
                                                                            as
                                                                            u8_0,
                                                                    bumperFlags:
                                                                        ((1 as
                                                                              libc::c_int)
                                                                             <<
                                                                             0
                                                                                 as
                                                                                 libc::c_int)
                                                                            as
                                                                            u8_0,
                                                                    ocElemFlags:
                                                                        ((1 as
                                                                              libc::c_int)
                                                                             <<
                                                                             0
                                                                                 as
                                                                                 libc::c_int)
                                                                            as
                                                                            u8_0,};
                                               init
                                           },
                                       dim:
                                           {
                                               let mut init =
                                                   ColliderJntSphElementDimInit{limb:
                                                                                    43
                                                                                        as
                                                                                        libc::c_int
                                                                                        as
                                                                                        u8_0,
                                                                                modelSphere:
                                                                                    {
                                                                                        let mut init =
                                                                                            Sphere16{center:
                                                                                                         {
                                                                                                             let mut init =
                                                                                                                 Vec3s{x:
                                                                                                                           1500
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
                                                                                                         80
                                                                                                             as
                                                                                                             libc::c_int
                                                                                                             as
                                                                                                             s16,};
                                                                                        init
                                                                                    },
                                                                                scale:
                                                                                    100
                                                                                        as
                                                                                        libc::c_int
                                                                                        as
                                                                                        s16,};
                                               init
                                           },};
         init
     },
     {
         let mut init =
             ColliderJntSphElementInit{info:
                                           {
                                               let mut init =
                                                   ColliderInfoInit{elemType:
                                                                        ELEMTYPE_UNK1
                                                                            as
                                                                            libc::c_int
                                                                            as
                                                                            u8_0,
                                                                    toucher:
                                                                        {
                                                                            let mut init =
                                                                                ColliderTouch{dmgFlags:
                                                                                                  0x20000000
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
                                                                             0
                                                                                 as
                                                                                 libc::c_int
                                                                             |
                                                                             (0
                                                                                  as
                                                                                  libc::c_int)
                                                                                 <<
                                                                                 3
                                                                                     as
                                                                                     libc::c_int)
                                                                            as
                                                                            u8_0,
                                                                    bumperFlags:
                                                                        ((1 as
                                                                              libc::c_int)
                                                                             <<
                                                                             0
                                                                                 as
                                                                                 libc::c_int)
                                                                            as
                                                                            u8_0,
                                                                    ocElemFlags:
                                                                        ((1 as
                                                                              libc::c_int)
                                                                             <<
                                                                             0
                                                                                 as
                                                                                 libc::c_int)
                                                                            as
                                                                            u8_0,};
                                               init
                                           },
                                       dim:
                                           {
                                               let mut init =
                                                   ColliderJntSphElementDimInit{limb:
                                                                                    43
                                                                                        as
                                                                                        libc::c_int
                                                                                        as
                                                                                        u8_0,
                                                                                modelSphere:
                                                                                    {
                                                                                        let mut init =
                                                                                            Sphere16{center:
                                                                                                         {
                                                                                                             let mut init =
                                                                                                                 Vec3s{x:
                                                                                                                           7500
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
                                                                                                         70
                                                                                                             as
                                                                                                             libc::c_int
                                                                                                             as
                                                                                                             s16,};
                                                                                        init
                                                                                    },
                                                                                scale:
                                                                                    100
                                                                                        as
                                                                                        libc::c_int
                                                                                        as
                                                                                        s16,};
                                               init
                                           },};
         init
     },
     {
         let mut init =
             ColliderJntSphElementInit{info:
                                           {
                                               let mut init =
                                                   ColliderInfoInit{elemType:
                                                                        ELEMTYPE_UNK1
                                                                            as
                                                                            libc::c_int
                                                                            as
                                                                            u8_0,
                                                                    toucher:
                                                                        {
                                                                            let mut init =
                                                                                ColliderTouch{dmgFlags:
                                                                                                  0x20000000
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
                                                                             0
                                                                                 as
                                                                                 libc::c_int
                                                                             |
                                                                             (0
                                                                                  as
                                                                                  libc::c_int)
                                                                                 <<
                                                                                 3
                                                                                     as
                                                                                     libc::c_int)
                                                                            as
                                                                            u8_0,
                                                                    bumperFlags:
                                                                        ((1 as
                                                                              libc::c_int)
                                                                             <<
                                                                             0
                                                                                 as
                                                                                 libc::c_int)
                                                                            as
                                                                            u8_0,
                                                                    ocElemFlags:
                                                                        ((1 as
                                                                              libc::c_int)
                                                                             <<
                                                                             0
                                                                                 as
                                                                                 libc::c_int)
                                                                            as
                                                                            u8_0,};
                                               init
                                           },
                                       dim:
                                           {
                                               let mut init =
                                                   ColliderJntSphElementDimInit{limb:
                                                                                    44
                                                                                        as
                                                                                        libc::c_int
                                                                                        as
                                                                                        u8_0,
                                                                                modelSphere:
                                                                                    {
                                                                                        let mut init =
                                                                                            Sphere16{center:
                                                                                                         {
                                                                                                             let mut init =
                                                                                                                 Vec3s{x:
                                                                                                                           3000
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
                                                                                                         60
                                                                                                             as
                                                                                                             libc::c_int
                                                                                                             as
                                                                                                             s16,};
                                                                                        init
                                                                                    },
                                                                                scale:
                                                                                    100
                                                                                        as
                                                                                        libc::c_int
                                                                                        as
                                                                                        s16,};
                                               init
                                           },};
         init
     },
     {
         let mut init =
             ColliderJntSphElementInit{info:
                                           {
                                               let mut init =
                                                   ColliderInfoInit{elemType:
                                                                        ELEMTYPE_UNK1
                                                                            as
                                                                            libc::c_int
                                                                            as
                                                                            u8_0,
                                                                    toucher:
                                                                        {
                                                                            let mut init =
                                                                                ColliderTouch{dmgFlags:
                                                                                                  0x20000000
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
                                                                             0
                                                                                 as
                                                                                 libc::c_int
                                                                             |
                                                                             (0
                                                                                  as
                                                                                  libc::c_int)
                                                                                 <<
                                                                                 3
                                                                                     as
                                                                                     libc::c_int)
                                                                            as
                                                                            u8_0,
                                                                    bumperFlags:
                                                                        ((1 as
                                                                              libc::c_int)
                                                                             <<
                                                                             0
                                                                                 as
                                                                                 libc::c_int)
                                                                            as
                                                                            u8_0,
                                                                    ocElemFlags:
                                                                        ((1 as
                                                                              libc::c_int)
                                                                             <<
                                                                             0
                                                                                 as
                                                                                 libc::c_int)
                                                                            as
                                                                            u8_0,};
                                               init
                                           },
                                       dim:
                                           {
                                               let mut init =
                                                   ColliderJntSphElementDimInit{limb:
                                                                                    40
                                                                                        as
                                                                                        libc::c_int
                                                                                        as
                                                                                        u8_0,
                                                                                modelSphere:
                                                                                    {
                                                                                        let mut init =
                                                                                            Sphere16{center:
                                                                                                         {
                                                                                                             let mut init =
                                                                                                                 Vec3s{x:
                                                                                                                           1500
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
                                                                                                         80
                                                                                                             as
                                                                                                             libc::c_int
                                                                                                             as
                                                                                                             s16,};
                                                                                        init
                                                                                    },
                                                                                scale:
                                                                                    100
                                                                                        as
                                                                                        libc::c_int
                                                                                        as
                                                                                        s16,};
                                               init
                                           },};
         init
     },
     {
         let mut init =
             ColliderJntSphElementInit{info:
                                           {
                                               let mut init =
                                                   ColliderInfoInit{elemType:
                                                                        ELEMTYPE_UNK1
                                                                            as
                                                                            libc::c_int
                                                                            as
                                                                            u8_0,
                                                                    toucher:
                                                                        {
                                                                            let mut init =
                                                                                ColliderTouch{dmgFlags:
                                                                                                  0x20000000
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
                                                                             0
                                                                                 as
                                                                                 libc::c_int
                                                                             |
                                                                             (0
                                                                                  as
                                                                                  libc::c_int)
                                                                                 <<
                                                                                 3
                                                                                     as
                                                                                     libc::c_int)
                                                                            as
                                                                            u8_0,
                                                                    bumperFlags:
                                                                        ((1 as
                                                                              libc::c_int)
                                                                             <<
                                                                             0
                                                                                 as
                                                                                 libc::c_int)
                                                                            as
                                                                            u8_0,
                                                                    ocElemFlags:
                                                                        ((1 as
                                                                              libc::c_int)
                                                                             <<
                                                                             0
                                                                                 as
                                                                                 libc::c_int)
                                                                            as
                                                                            u8_0,};
                                               init
                                           },
                                       dim:
                                           {
                                               let mut init =
                                                   ColliderJntSphElementDimInit{limb:
                                                                                    40
                                                                                        as
                                                                                        libc::c_int
                                                                                        as
                                                                                        u8_0,
                                                                                modelSphere:
                                                                                    {
                                                                                        let mut init =
                                                                                            Sphere16{center:
                                                                                                         {
                                                                                                             let mut init =
                                                                                                                 Vec3s{x:
                                                                                                                           7500
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
                                                                                                         70
                                                                                                             as
                                                                                                             libc::c_int
                                                                                                             as
                                                                                                             s16,};
                                                                                        init
                                                                                    },
                                                                                scale:
                                                                                    100
                                                                                        as
                                                                                        libc::c_int
                                                                                        as
                                                                                        s16,};
                                               init
                                           },};
         init
     },
     {
         let mut init =
             ColliderJntSphElementInit{info:
                                           {
                                               let mut init =
                                                   ColliderInfoInit{elemType:
                                                                        ELEMTYPE_UNK1
                                                                            as
                                                                            libc::c_int
                                                                            as
                                                                            u8_0,
                                                                    toucher:
                                                                        {
                                                                            let mut init =
                                                                                ColliderTouch{dmgFlags:
                                                                                                  0x20000000
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
                                                                             0
                                                                                 as
                                                                                 libc::c_int
                                                                             |
                                                                             (0
                                                                                  as
                                                                                  libc::c_int)
                                                                                 <<
                                                                                 3
                                                                                     as
                                                                                     libc::c_int)
                                                                            as
                                                                            u8_0,
                                                                    bumperFlags:
                                                                        ((1 as
                                                                              libc::c_int)
                                                                             <<
                                                                             0
                                                                                 as
                                                                                 libc::c_int)
                                                                            as
                                                                            u8_0,
                                                                    ocElemFlags:
                                                                        ((1 as
                                                                              libc::c_int)
                                                                             <<
                                                                             0
                                                                                 as
                                                                                 libc::c_int)
                                                                            as
                                                                            u8_0,};
                                               init
                                           },
                                       dim:
                                           {
                                               let mut init =
                                                   ColliderJntSphElementDimInit{limb:
                                                                                    41
                                                                                        as
                                                                                        libc::c_int
                                                                                        as
                                                                                        u8_0,
                                                                                modelSphere:
                                                                                    {
                                                                                        let mut init =
                                                                                            Sphere16{center:
                                                                                                         {
                                                                                                             let mut init =
                                                                                                                 Vec3s{x:
                                                                                                                           3000
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
                                                                                                         60
                                                                                                             as
                                                                                                             libc::c_int
                                                                                                             as
                                                                                                             s16,};
                                                                                        init
                                                                                    },
                                                                                scale:
                                                                                    100
                                                                                        as
                                                                                        libc::c_int
                                                                                        as
                                                                                        s16,};
                                               init
                                           },};
         init
     },
     {
         let mut init =
             ColliderJntSphElementInit{info:
                                           {
                                               let mut init =
                                                   ColliderInfoInit{elemType:
                                                                        ELEMTYPE_UNK1
                                                                            as
                                                                            libc::c_int
                                                                            as
                                                                            u8_0,
                                                                    toucher:
                                                                        {
                                                                            let mut init =
                                                                                ColliderTouch{dmgFlags:
                                                                                                  0x20000000
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
                                                                                                     0x80
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
                                                                             0
                                                                                 as
                                                                                 libc::c_int
                                                                             |
                                                                             (0
                                                                                  as
                                                                                  libc::c_int)
                                                                                 <<
                                                                                 3
                                                                                     as
                                                                                     libc::c_int)
                                                                            as
                                                                            u8_0,
                                                                    bumperFlags:
                                                                        0 as
                                                                            libc::c_int
                                                                            as
                                                                            u8_0,
                                                                    ocElemFlags:
                                                                        ((1 as
                                                                              libc::c_int)
                                                                             <<
                                                                             0
                                                                                 as
                                                                                 libc::c_int)
                                                                            as
                                                                            u8_0,};
                                               init
                                           },
                                       dim:
                                           {
                                               let mut init =
                                                   ColliderJntSphElementDimInit{limb:
                                                                                    8
                                                                                        as
                                                                                        libc::c_int
                                                                                        as
                                                                                        u8_0,
                                                                                modelSphere:
                                                                                    {
                                                                                        let mut init =
                                                                                            Sphere16{center:
                                                                                                         {
                                                                                                             let mut init =
                                                                                                                 Vec3s{x:
                                                                                                                           1500
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
                                                                                                         70
                                                                                                             as
                                                                                                             libc::c_int
                                                                                                             as
                                                                                                             s16,};
                                                                                        init
                                                                                    },
                                                                                scale:
                                                                                    100
                                                                                        as
                                                                                        libc::c_int
                                                                                        as
                                                                                        s16,};
                                               init
                                           },};
         init
     }];
static mut sJntSphInitHead: ColliderJntSphInit =
    unsafe {
        {
            let mut init =
                ColliderJntSphInit{base:
                                       {
                                           let mut init =
                                               ColliderInit{colType:
                                                                COLTYPE_HARD
                                                                    as
                                                                    libc::c_int
                                                                    as u8_0,
                                                            atFlags:
                                                                ((1 as
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
                                                                ((1 as
                                                                      libc::c_int)
                                                                     <<
                                                                     3 as
                                                                         libc::c_int
                                                                     |
                                                                     (1 as
                                                                          libc::c_int)
                                                                         <<
                                                                         4 as
                                                                             libc::c_int
                                                                     |
                                                                     (1 as
                                                                          libc::c_int)
                                                                         <<
                                                                         5 as
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
                                                                COLSHAPE_JNTSPH
                                                                    as
                                                                    libc::c_int
                                                                    as u8_0,};
                                           init
                                       },
                                   count: 11 as libc::c_int,
                                   elements:
                                       sJntSphItemsInitHead.as_ptr() as
                                           *mut _,};
            init
        }
    };
static mut sCylinderInitHead: ColliderCylinderInit =
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
                                                              (0 as
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
                                                            85 as libc::c_int
                                                                as s16,
                                                        height:
                                                            100 as libc::c_int
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
static mut sCylinderInitHand: ColliderCylinderInit =
    {
        let mut init =
            ColliderCylinderInit{base:
                                     {
                                         let mut init =
                                             ColliderInit{colType:
                                                              COLTYPE_NONE as
                                                                  libc::c_int
                                                                  as u8_0,
                                                          atFlags:
                                                              ((1 as
                                                                    libc::c_int)
                                                                   <<
                                                                   4 as
                                                                       libc::c_int)
                                                                  as u8_0,
                                                          acFlags:
                                                              0 as libc::c_int
                                                                  as u8_0,
                                                          ocFlags1:
                                                              0 as libc::c_int
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
                                                                                            0x20000000
                                                                                                as
                                                                                                libc::c_int
                                                                                                as
                                                                                                u32_0,
                                                                                        effect:
                                                                                            0x4
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
                                                                       (3 as
                                                                            libc::c_int)
                                                                           <<
                                                                           3
                                                                               as
                                                                               libc::c_int)
                                                                      as u8_0,
                                                              bumperFlags:
                                                                  0 as
                                                                      libc::c_int
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
                                                            85 as libc::c_int
                                                                as s16,
                                                        height:
                                                            1 as libc::c_int
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
static mut sColChkInfoInit: CollisionCheckInfoInit =
    {
        let mut init =
            CollisionCheckInfoInit{health: 36 as libc::c_int as u8_0,
                                   cylRadius: 100 as libc::c_int as s16,
                                   cylHeight: 100 as libc::c_int as s16,
                                   mass: 200 as libc::c_int as u8_0,};
        init
    };
static mut sDamageTable: DamageTable =
    {
        let mut init =
            DamageTable{table:
                            [(0 as libc::c_int |
                                  (0 as libc::c_int) << 4 as libc::c_int) as
                                 u8_0,
                             (2 as libc::c_int |
                                  (0 as libc::c_int) << 4 as libc::c_int) as
                                 u8_0,
                             (1 as libc::c_int |
                                  (0 as libc::c_int) << 4 as libc::c_int) as
                                 u8_0,
                             (2 as libc::c_int |
                                  (0 as libc::c_int) << 4 as libc::c_int) as
                                 u8_0,
                             (0 as libc::c_int |
                                  (0 as libc::c_int) << 4 as libc::c_int) as
                                 u8_0,
                             (2 as libc::c_int |
                                  (0 as libc::c_int) << 4 as libc::c_int) as
                                 u8_0,
                             (2 as libc::c_int |
                                  (0 as libc::c_int) << 4 as libc::c_int) as
                                 u8_0,
                             (2 as libc::c_int |
                                  (0 as libc::c_int) << 4 as libc::c_int) as
                                 u8_0,
                             (1 as libc::c_int |
                                  (0 as libc::c_int) << 4 as libc::c_int) as
                                 u8_0,
                             (2 as libc::c_int |
                                  (0 as libc::c_int) << 4 as libc::c_int) as
                                 u8_0,
                             (4 as libc::c_int |
                                  (0 as libc::c_int) << 4 as libc::c_int) as
                                 u8_0,
                             (2 as libc::c_int |
                                  (0 as libc::c_int) << 4 as libc::c_int) as
                                 u8_0,
                             (4 as libc::c_int |
                                  (0x3 as libc::c_int) << 4 as libc::c_int) as
                                 u8_0,
                             (2 as libc::c_int |
                                  (0 as libc::c_int) << 4 as libc::c_int) as
                                 u8_0,
                             (4 as libc::c_int |
                                  (0 as libc::c_int) << 4 as libc::c_int) as
                                 u8_0,
                             (2 as libc::c_int |
                                  (0 as libc::c_int) << 4 as libc::c_int) as
                                 u8_0,
                             (2 as libc::c_int |
                                  (0 as libc::c_int) << 4 as libc::c_int) as
                                 u8_0,
                             (0 as libc::c_int |
                                  (0 as libc::c_int) << 4 as libc::c_int) as
                                 u8_0,
                             (4 as libc::c_int |
                                  (0x3 as libc::c_int) << 4 as libc::c_int) as
                                 u8_0,
                             (4 as libc::c_int |
                                  (0x4 as libc::c_int) << 4 as libc::c_int) as
                                 u8_0,
                             (0 as libc::c_int |
                                  (0 as libc::c_int) << 4 as libc::c_int) as
                                 u8_0,
                             (0 as libc::c_int |
                                  (0 as libc::c_int) << 4 as libc::c_int) as
                                 u8_0,
                             (1 as libc::c_int |
                                  (0 as libc::c_int) << 4 as libc::c_int) as
                                 u8_0,
                             (4 as libc::c_int |
                                  (0 as libc::c_int) << 4 as libc::c_int) as
                                 u8_0,
                             (2 as libc::c_int |
                                  (0 as libc::c_int) << 4 as libc::c_int) as
                                 u8_0,
                             (2 as libc::c_int |
                                  (0 as libc::c_int) << 4 as libc::c_int) as
                                 u8_0,
                             (8 as libc::c_int |
                                  (0 as libc::c_int) << 4 as libc::c_int) as
                                 u8_0,
                             (4 as libc::c_int |
                                  (0 as libc::c_int) << 4 as libc::c_int) as
                                 u8_0,
                             (0 as libc::c_int |
                                  (0 as libc::c_int) << 4 as libc::c_int) as
                                 u8_0,
                             (0 as libc::c_int |
                                  (0 as libc::c_int) << 4 as libc::c_int) as
                                 u8_0,
                             (4 as libc::c_int |
                                  (0 as libc::c_int) << 4 as libc::c_int) as
                                 u8_0,
                             (0 as libc::c_int |
                                  (0 as libc::c_int) << 4 as libc::c_int) as
                                 u8_0],};
        init
    };
static mut sHandIdleAnims: [*mut AnimationHeader; 2] =
    unsafe {
        [&gBongoLeftHandIdleAnim as *const AnimationHeader as
             *mut AnimationHeader,
         &gBongoRightHandIdleAnim as *const AnimationHeader as
             *mut AnimationHeader]
    };
static mut sHandFlatPoses: [*mut AnimationHeader; 2] =
    unsafe {
        [&gBongoLeftHandFlatPoseAnim as *const AnimationHeader as
             *mut AnimationHeader,
         &gBongoRightHandFlatPoseAnim as *const AnimationHeader as
             *mut AnimationHeader]
    };
static mut sHandOpenPoses: [*mut AnimationHeader; 2] =
    unsafe {
        [&gBongoLeftHandOpenPoseAnim as *const AnimationHeader as
             *mut AnimationHeader,
         &gBongoRightHandOpenPoseAnim as *const AnimationHeader as
             *mut AnimationHeader]
    };
static mut sHandFistPoses: [*mut AnimationHeader; 2] =
    unsafe {
        [&gBongoLeftHandFistPoseAnim as *const AnimationHeader as
             *mut AnimationHeader,
         &gBongoRightHandFistPoseAnim as *const AnimationHeader as
             *mut AnimationHeader]
    };
static mut sHandClenchAnims: [*mut AnimationHeader; 2] =
    unsafe {
        [&gBongoLeftHandClenchAnim as *const AnimationHeader as
             *mut AnimationHeader,
         &gBongoRightHandClenchAnim as *const AnimationHeader as
             *mut AnimationHeader]
    };
static mut sHandDamagePoses: [*mut AnimationHeader; 2] =
    unsafe {
        [&gBongoLeftHandDamagePoseAnim as *const AnimationHeader as
             *mut AnimationHeader,
         &gBongoRightHandDamagePoseAnim as *const AnimationHeader as
             *mut AnimationHeader]
    };
static mut sHandPushoffPoses: [*mut AnimationHeader; 2] =
    unsafe {
        [&gBongoLeftHandPushoffPoseAnim as *const AnimationHeader as
             *mut AnimationHeader,
         &gBongoRightHandPushoffPoseAnim as *const AnimationHeader as
             *mut AnimationHeader]
    };
static mut sHandHangPoses: [*mut AnimationHeader; 2] =
    unsafe {
        [&gBongoLeftHandHangPoseAnim as *const AnimationHeader as
             *mut AnimationHeader,
         &gBongoRightHandHangPoseAnim as *const AnimationHeader as
             *mut AnimationHeader]
    };
// Initialized in run_static_initializers
static mut sInitChain: [InitChainEntry; 3] =
    [InitChainEntry{cont_type_0_offset_value: [0; 4],}; 3];
#[no_mangle]
pub unsafe extern "C" fn BossSst_Init(mut thisx: *mut Actor,
                                      mut globalCtx2: *mut GlobalContext) {
    let mut globalCtx: *mut GlobalContext = globalCtx2; // Sword-type damage
    let mut this: *mut BossSst =
        thisx as *mut BossSst; // randInt == 3 || randInt == 4
    Actor_ProcessInitChain(&mut (*this).actor, sInitChain.as_mut_ptr());
    Collider_InitCylinder(globalCtx, &mut (*this).colliderCyl);
    Collider_InitJntSph(globalCtx, &mut (*this).colliderJntSph);
    CollisionCheck_SetInfo(&mut (*this).actor.colChkInfo, &mut sDamageTable,
                           &mut sColChkInfoInit);
    Flags_SetSwitch(globalCtx, 0x14 as libc::c_int);
    if (*this).actor.params as libc::c_int == BONGO_HEAD as libc::c_int {
        sFloor =
            Actor_Spawn(&mut (*globalCtx).actorCtx, globalCtx,
                        ACTOR_BG_SST_FLOOR as libc::c_int as s16,
                        sRoomCenter.x, sRoomCenter.y, sRoomCenter.z,
                        0 as libc::c_int as s16, 0 as libc::c_int as s16,
                        0 as libc::c_int as s16,
                        BONGOFLOOR_REST as libc::c_int as s16) as
                *mut BgSstFloor;
        SkelAnime_InitFlex(globalCtx, &mut (*this).skelAnime,
                           &mut gBongoHeadSkel,
                           &mut gBongoHeadEyeOpenIdleAnim,
                           (*this).jointTable.as_mut_ptr(),
                           (*this).morphTable.as_mut_ptr(),
                           45 as libc::c_int);
        ActorShape_Init(&mut (*this).actor.shape, 70000.0f32,
                        Some(ActorShadow_DrawCircle as
                                 unsafe extern "C" fn(_: *mut Actor,
                                                      _: *mut Lights,
                                                      _: *mut GlobalContext)
                                     -> ()), 95.0f32);
        Collider_SetJntSph(globalCtx, &mut (*this).colliderJntSph,
                           &mut (*this).actor, &mut sJntSphInitHead,
                           (*this).colliderItems.as_mut_ptr());
        Collider_SetCylinder(globalCtx, &mut (*this).colliderCyl,
                             &mut (*this).actor, &mut sCylinderInitHead);
        sHead = this;
        (*this).actor.world.pos.x = -50.0f32 + 50.0f32;
        (*this).actor.world.pos.y = 0.0f32 + 0.0f32;
        (*this).actor.world.pos.z = 0.0f32 - 650.0f32;
        (*this).actor.home.pos = (*this).actor.world.pos;
        (*this).actor.shape.rot.y = 0 as libc::c_int as s16;
        if Flags_GetClear(globalCtx, (*globalCtx).roomCtx.curRoom.num as s32)
               != 0 {
            Actor_Spawn(&mut (*globalCtx).actorCtx, globalCtx,
                        ACTOR_DOOR_WARP1 as libc::c_int as s16, -50.0f32,
                        0.0f32, 0.0f32 + 400.0f32, 0 as libc::c_int as s16,
                        0 as libc::c_int as s16, 0 as libc::c_int as s16,
                        WARP_DUNGEON_ADULT as libc::c_int as s16);
            Actor_Spawn(&mut (*globalCtx).actorCtx, globalCtx,
                        ACTOR_ITEM_B_HEART as libc::c_int as s16, -50.0f32,
                        0.0f32, 0.0f32 - 200.0f32, 0 as libc::c_int as s16,
                        0 as libc::c_int as s16, 0 as libc::c_int as s16,
                        0 as libc::c_int as s16);
            Actor_Kill(&mut (*this).actor);
        } else {
            sHands[0 as libc::c_int as usize] =
                Actor_Spawn(&mut (*globalCtx).actorCtx, globalCtx,
                            ACTOR_BOSS_SST as libc::c_int as s16,
                            (*this).actor.world.pos.x + 200.0f32,
                            (*this).actor.world.pos.y,
                            (*this).actor.world.pos.z + 400.0f32,
                            0 as libc::c_int as s16,
                            (*this).actor.shape.rot.y,
                            0 as libc::c_int as s16,
                            BONGO_LEFT_HAND as libc::c_int as s16) as
                    *mut BossSst;
            sHands[1 as libc::c_int as usize] =
                Actor_Spawn(&mut (*globalCtx).actorCtx, globalCtx,
                            ACTOR_BOSS_SST as libc::c_int as s16,
                            (*this).actor.world.pos.x + -200.0f32,
                            (*this).actor.world.pos.y,
                            (*this).actor.world.pos.z + 400.0f32,
                            0 as libc::c_int as s16,
                            (*this).actor.shape.rot.y,
                            0 as libc::c_int as s16,
                            BONGO_RIGHT_HAND as libc::c_int as s16) as
                    *mut BossSst;
            (*sHands[0 as libc::c_int as usize]).actor.child =
                &mut (**sHands.as_mut_ptr().offset(1 as libc::c_int as
                                                       isize)).actor;
            (*sHands[1 as libc::c_int as usize]).actor.child =
                &mut (**sHands.as_mut_ptr().offset(0 as libc::c_int as
                                                       isize)).actor;
            (*this).actor.flags &=
                !((1 as libc::c_int) << 0 as libc::c_int) as libc::c_uint;
            (*this).actor.update =
                Some(BossSst_UpdateHead as
                         unsafe extern "C" fn(_: *mut Actor,
                                              _: *mut GlobalContext) -> ());
            (*this).actor.draw =
                Some(BossSst_DrawHead as
                         unsafe extern "C" fn(_: *mut Actor,
                                              _: *mut GlobalContext) -> ());
            (*this).radius = -650.0f32;
            (*this).actor.targetArrowOffset = 4000.0f32;
            BossSst_HeadSetupLurk(this);
            Actor_ChangeCategory(globalCtx, &mut (*globalCtx).actorCtx,
                                 &mut (*this).actor,
                                 ACTORCAT_BOSS as libc::c_int as u8_0);
        }
    } else {
        Collider_SetJntSph(globalCtx, &mut (*this).colliderJntSph,
                           &mut (*this).actor, &mut sJntSphInitHand,
                           (*this).colliderItems.as_mut_ptr());
        Collider_SetCylinder(globalCtx, &mut (*this).colliderCyl,
                             &mut (*this).actor, &mut sCylinderInitHand);
        if (*this).actor.params as libc::c_int ==
               BONGO_LEFT_HAND as libc::c_int {
            SkelAnime_InitFlex(globalCtx, &mut (*this).skelAnime,
                               &mut gBongoLeftHandSkel,
                               &mut gBongoLeftHandIdleAnim,
                               (*this).jointTable.as_mut_ptr(),
                               (*this).morphTable.as_mut_ptr(),
                               27 as libc::c_int);
            (*this).actionVar = -(1 as libc::c_int) as s8;
            let ref mut fresh0 =
                (*(*this).colliderJntSph.elements.offset(0 as libc::c_int as
                                                             isize)).dim.modelSphere.center.z;
            *fresh0 = (*fresh0 as libc::c_int * -(1 as libc::c_int)) as s16
        } else {
            SkelAnime_InitFlex(globalCtx, &mut (*this).skelAnime,
                               &mut gBongoRightHandSkel,
                               &mut gBongoRightHandIdleAnim,
                               (*this).jointTable.as_mut_ptr(),
                               (*this).morphTable.as_mut_ptr(),
                               27 as libc::c_int);
            (*this).actionVar = 1 as libc::c_int as s8
        }
        ActorShape_Init(&mut (*this).actor.shape, 0.0f32,
                        Some(ActorShadow_DrawCircle as
                                 unsafe extern "C" fn(_: *mut Actor,
                                                      _: *mut Lights,
                                                      _: *mut GlobalContext)
                                     -> ()), 95.0f32);
        (*this).handZPosMod = -(3500 as libc::c_int) as s16;
        (*this).actor.targetArrowOffset = 5000.0f32;
        (*this).actor.flags &=
            !((1 as libc::c_int) << 0 as libc::c_int) as libc::c_uint;
        BossSst_HandSetupWait(this);
    };
}
#[no_mangle]
pub unsafe extern "C" fn BossSst_Destroy(mut thisx: *mut Actor,
                                         mut globalCtx: *mut GlobalContext) {
    let mut pad: s32 = 0;
    let mut this: *mut BossSst = thisx as *mut BossSst;
    Collider_DestroyJntSph(globalCtx, &mut (*this).colliderJntSph);
    Collider_DestroyCylinder(globalCtx, &mut (*this).colliderCyl);
    Audio_StopSfxByPos(&mut (*this).center);
}
#[no_mangle]
pub unsafe extern "C" fn BossSst_HeadSetupLurk(mut this: *mut BossSst) {
    (*this).actor.draw = None;
    (*sHands[0 as libc::c_int as usize]).actor.draw = None;
    (*sHands[1 as libc::c_int as usize]).actor.draw = None;
    (*this).actionVar = 0 as libc::c_int as s8;
    (*this).actionFunc =
        Some(BossSst_HeadLurk as
                 unsafe extern "C" fn(_: *mut BossSst, _: *mut GlobalContext)
                     -> ());
}
#[no_mangle]
pub unsafe extern "C" fn BossSst_HeadLurk(mut this: *mut BossSst,
                                          mut globalCtx: *mut GlobalContext) {
    if (*this).actor.yDistToPlayer < 1000.0f32 {
        BossSst_HeadSetupIntro(this, globalCtx);
    };
}
#[no_mangle]
pub unsafe extern "C" fn BossSst_HeadSetupIntro(mut this: *mut BossSst,
                                                mut globalCtx:
                                                    *mut GlobalContext) {
    let mut player: *mut Player =
        (*globalCtx).actorCtx.actorLists[ACTORCAT_PLAYER as libc::c_int as
                                             usize].head as *mut Player;
    (*this).timer = 611 as libc::c_int as s16;
    (*this).ready = 0 as libc::c_int as s8;
    (*player).actor.world.pos.x = sRoomCenter.x;
    (*player).actor.world.pos.y = 0.0f32 + 1000.0f32;
    (*player).actor.world.pos.z = sRoomCenter.z;
    (*player).actor.velocity.y = 0.0f32;
    (*player).linearVelocity = (*player).actor.velocity.y;
    (*player).actor.shape.rot.y = -(0x8000 as libc::c_int) as s16;
    (*player).targetYaw = -(0x8000 as libc::c_int) as s16;
    (*player).currentYaw = -(0x8000 as libc::c_int) as s16;
    (*player).fallStartHeight = 0 as libc::c_int as s16;
    (*player).stateFlags1 |= 0x20 as libc::c_int as libc::c_uint;
    func_80064520(globalCtx, &mut (*globalCtx).csCtx);
    func_8002DF54(globalCtx, &mut (*this).actor, 8 as libc::c_int as u8_0);
    sCutsceneCamera = Gameplay_CreateSubCamera(globalCtx);
    Gameplay_ChangeCameraStatus(globalCtx, 0 as libc::c_int as s16,
                                1 as libc::c_int as s16);
    Gameplay_ChangeCameraStatus(globalCtx, sCutsceneCamera,
                                7 as libc::c_int as s16);
    Math_Vec3f_Copy(&mut sCameraAt, &mut (*player).actor.world.pos);
    if gSaveContext.eventChkInf[7 as libc::c_int as usize] as libc::c_int &
           0x80 as libc::c_int != 0 {
        sCameraEye.z = 0.0f32 - 100.0f32
    }
    Gameplay_CameraSetAtEye(globalCtx, sCutsceneCamera, &mut sCameraAt,
                            &mut sCameraEye);
    Audio_QueueSeqCmd(((0x1 as libc::c_int) << 28 as libc::c_int |
                           (SEQ_PLAYER_BGM_MAIN as libc::c_int) <<
                               24 as libc::c_int | 0x100ff as libc::c_int) as
                          u32_0);
    (*this).actionFunc =
        Some(BossSst_HeadIntro as
                 unsafe extern "C" fn(_: *mut BossSst, _: *mut GlobalContext)
                     -> ());
}
#[no_mangle]
pub unsafe extern "C" fn BossSst_HeadIntro(mut this: *mut BossSst,
                                           mut globalCtx:
                                               *mut GlobalContext) {
    let mut player: *mut Player =
        (*globalCtx).actorCtx.actorLists[ACTORCAT_PLAYER as libc::c_int as
                                             usize].head as *mut Player;
    let mut tempo: s32 = 0;
    let mut introStateTimer: s32 = 0;
    let mut revealStateTimer: s32 = 0;
    if (*this).timer as libc::c_int != 0 as libc::c_int { (*this).timer -= 1 }
    if SkelAnime_Update(&mut (*this).skelAnime) != 0 {
        Animation_MorphToLoop(&mut (*this).skelAnime,
                              &mut gBongoHeadEyeCloseIdleAnim, -3.0f32);
    }
    if (*this).timer as libc::c_int == 0 as libc::c_int {
        (*sHands[1 as libc::c_int as usize]).actor.flags |=
            ((1 as libc::c_int) << 0 as libc::c_int) as libc::c_uint;
        (*sHands[0 as libc::c_int as usize]).actor.flags |=
            ((1 as libc::c_int) << 0 as libc::c_int) as libc::c_uint;
        (*player).stateFlags1 &= !(0x20 as libc::c_int) as libc::c_uint;
        func_80064534(globalCtx, &mut (*globalCtx).csCtx);
        func_8002DF54(globalCtx, &mut (*this).actor,
                      7 as libc::c_int as u8_0);
        sCameraAt.y += 30.0f32;
        sCameraAt.z += 300.0f32;
        Gameplay_CameraSetAtEye(globalCtx, sCutsceneCamera, &mut sCameraAt,
                                &mut sCameraEye);
        Gameplay_CopyCamera(globalCtx, 0 as libc::c_int as s16,
                            sCutsceneCamera);
        Gameplay_ChangeCameraStatus(globalCtx, sCutsceneCamera,
                                    1 as libc::c_int as s16);
        Gameplay_ChangeCameraStatus(globalCtx, 0 as libc::c_int as s16,
                                    7 as libc::c_int as s16);
        Gameplay_ClearCamera(globalCtx, sCutsceneCamera);
        gSaveContext.eventChkInf[7 as libc::c_int as usize] =
            (gSaveContext.eventChkInf[7 as libc::c_int as usize] as
                 libc::c_int | 0x80 as libc::c_int) as u16_0;
        BossSst_HeadSetupNeutral(this);
        (*this).colliderJntSph.base.ocFlags1 =
            ((*this).colliderJntSph.base.ocFlags1 as libc::c_int |
                 (1 as libc::c_int) << 0 as libc::c_int) as u8_0;
        (*sHands[0 as libc::c_int as usize]).colliderJntSph.base.ocFlags1 =
            ((*sHands[0 as libc::c_int as usize]).colliderJntSph.base.ocFlags1
                 as libc::c_int | (1 as libc::c_int) << 0 as libc::c_int) as
                u8_0;
        (*sHands[1 as libc::c_int as usize]).colliderJntSph.base.ocFlags1 =
            ((*sHands[1 as libc::c_int as usize]).colliderJntSph.base.ocFlags1
                 as libc::c_int | (1 as libc::c_int) << 0 as libc::c_int) as
                u8_0;
        (*this).timer = 112 as libc::c_int as s16
    } else if (*this).timer as libc::c_int >= 546 as libc::c_int {
        if (*player).actor.world.pos.y > 100.0f32 {
            (*player).actor.world.pos.x = sRoomCenter.x;
            (*player).actor.world.pos.z = sRoomCenter.z;
            (*player).linearVelocity = 0 as libc::c_int as f32_0;
            (*player).actor.shape.rot.y = -(0x8000 as libc::c_int) as s16;
            (*player).targetYaw = -(0x8000 as libc::c_int) as s16;
            (*player).currentYaw = -(0x8000 as libc::c_int) as s16
        }
        Math_Vec3f_Copy(&mut sCameraAt, &mut (*player).actor.world.pos);
        if (*player).actor.bgCheckFlags as libc::c_int & 2 as libc::c_int != 0
           {
            if (*this).ready == 0 {
                (*sFloor).dyna.actor.params =
                    BONGOFLOOR_HIT as libc::c_int as s16;
                (*this).ready = 1 as libc::c_int as s8;
                func_800AA000((*this).actor.xyzDistToPlayerSq,
                              0xff as libc::c_int as u8_0,
                              0x14 as libc::c_int as u8_0,
                              0x96 as libc::c_int as u8_0);
                Audio_PlayActorSound2(&mut (*sFloor).dyna.actor,
                                      0x3951 as libc::c_int as u16_0);
            } else if gSaveContext.eventChkInf[7 as libc::c_int as usize] as
                          libc::c_int & 0x80 as libc::c_int != 0 {
                (*sHands[1 as libc::c_int as usize]).actor.draw =
                    Some(BossSst_DrawHand as
                             unsafe extern "C" fn(_: *mut Actor,
                                                  _: *mut GlobalContext)
                                 -> ());
                (*sHands[0 as libc::c_int as usize]).actor.draw =
                    Some(BossSst_DrawHand as
                             unsafe extern "C" fn(_: *mut Actor,
                                                  _: *mut GlobalContext)
                                 -> ());
                (*this).actor.draw =
                    Some(BossSst_DrawHead as
                             unsafe extern "C" fn(_: *mut Actor,
                                                  _: *mut GlobalContext)
                                 -> ());
                (*this).timer = 178 as libc::c_int as s16;
                sCameraAt.x = -50.0f32 - 23.0f32;
                sCameraAt.y = 0.0f32 + 0.0f32;
                sCameraAt.z = 0.0f32 + 0.0f32
            } else { (*this).timer = 546 as libc::c_int as s16 }
        }
    } else if (*this).timer as libc::c_int >= 478 as libc::c_int {
        sCameraEye.x += 10.0f32;
        sCameraEye.y += 10.0f32;
        sCameraEye.z -= 10.0f32
    } else if (*this).timer as libc::c_int >= 448 as libc::c_int {
        if (*this).timer as libc::c_int == 460 as libc::c_int {
            (*sHands[1 as libc::c_int as usize]).actor.draw =
                Some(BossSst_DrawHand as
                         unsafe extern "C" fn(_: *mut Actor,
                                              _: *mut GlobalContext) -> ());
            (*sHands[0 as libc::c_int as usize]).actor.draw =
                Some(BossSst_DrawHand as
                         unsafe extern "C" fn(_: *mut Actor,
                                              _: *mut GlobalContext) -> ());
            (*this).actor.draw =
                Some(BossSst_DrawHead as
                         unsafe extern "C" fn(_: *mut Actor,
                                              _: *mut GlobalContext) -> ());
            (*player).actor.world.pos.x = sRoomCenter.x;
            (*player).actor.world.pos.z = sRoomCenter.z;
            BossSst_HandSetupDownbeat(sHands[1 as libc::c_int as usize]);
        }
        if (*this).timer as libc::c_int > 460 as libc::c_int {
            sCameraEye.x -= 40.0f32;
            sCameraEye.y -= 40.0f32;
            sCameraEye.z += 20.0f32
        } else if (*this).timer as libc::c_int == 460 as libc::c_int {
            sCameraAt.x =
                (*sHands[1 as libc::c_int as usize]).actor.home.pos.x +
                    0.0f32;
            sCameraAt.y =
                (*sHands[1 as libc::c_int as usize]).actor.home.pos.y -
                    20.0f32;
            sCameraAt.z =
                (*sHands[1 as libc::c_int as usize]).actor.home.pos.z +
                    10.0f32;
            sCameraEye.x =
                (*sHands[1 as libc::c_int as usize]).actor.home.pos.x +
                    150.0f32;
            sCameraEye.y =
                (*sHands[1 as libc::c_int as usize]).actor.home.pos.y +
                    100.0f32;
            sCameraEye.z =
                (*sHands[1 as libc::c_int as usize]).actor.home.pos.z +
                    80.0f32
        }
    } else {
        if (*this).timer as libc::c_int >= 372 as libc::c_int {
            introStateTimer =
                (*this).timer as libc::c_int - 372 as libc::c_int;
            tempo = 6 as libc::c_int;
            if (*this).timer as libc::c_int == 447 as libc::c_int {
                sCameraAt = (*player).actor.world.pos;
                sCameraEye.x = -50.0f32 - 200.0f32;
                sCameraEye.y = 0.0f32 + 160.0f32;
                sCameraEye.z = 0.0f32 - 190.0f32
            } else if introStateTimer == 11 as libc::c_int {
                sCameraAt.x =
                    (*sHands[1 as libc::c_int as usize]).actor.home.pos.x +
                        30.0f32;
                sCameraAt.y =
                    (*sHands[1 as libc::c_int as usize]).actor.home.pos.y +
                        0.0f32;
                sCameraAt.z =
                    (*sHands[1 as libc::c_int as usize]).actor.home.pos.z +
                        20.0f32;
                sCameraEye.x =
                    (*sHands[1 as libc::c_int as usize]).actor.home.pos.x +
                        100.0f32;
                sCameraEye.y =
                    (*sHands[1 as libc::c_int as usize]).actor.home.pos.y +
                        10.0f32;
                sCameraEye.z =
                    (*sHands[1 as libc::c_int as usize]).actor.home.pos.z -
                        210.0f32
            } else if introStateTimer == 62 as libc::c_int {
                sCameraAt.x =
                    (*sHands[0 as libc::c_int as usize]).actor.home.pos.x +
                        0.0f32;
                sCameraAt.y =
                    (*sHands[0 as libc::c_int as usize]).actor.home.pos.y +
                        50.0f32;
                sCameraAt.z =
                    (*sHands[0 as libc::c_int as usize]).actor.home.pos.z +
                        100.0f32;
                sCameraEye.x =
                    (*sHands[0 as libc::c_int as usize]).actor.home.pos.x +
                        110.0f32;
                sCameraEye.y =
                    (*sHands[0 as libc::c_int as usize]).actor.home.pos.y +
                        180.0f32;
                sCameraEye.z =
                    (*sHands[0 as libc::c_int as usize]).actor.home.pos.z -
                        70.0f32
            }
        } else if (*this).timer as libc::c_int >= 304 as libc::c_int {
            introStateTimer =
                (*this).timer as libc::c_int - 304 as libc::c_int;
            tempo = 5 as libc::c_int;
            if introStateTimer == 11 as libc::c_int {
                sCameraAt.x =
                    (*sHands[1 as libc::c_int as usize]).actor.home.pos.x +
                        40.0f32;
                sCameraAt.y =
                    (*sHands[1 as libc::c_int as usize]).actor.home.pos.y -
                        90.0f32;
                sCameraAt.z =
                    (*sHands[1 as libc::c_int as usize]).actor.home.pos.z -
                        40.0f32;
                sCameraEye.x =
                    (*sHands[1 as libc::c_int as usize]).actor.home.pos.x -
                        20.0f32;
                sCameraEye.y =
                    (*sHands[1 as libc::c_int as usize]).actor.home.pos.y +
                        210.0f32;
                sCameraEye.z =
                    (*sHands[1 as libc::c_int as usize]).actor.home.pos.z +
                        170.0f32
            } else if (*this).timer as libc::c_int == 368 as libc::c_int {
                sCameraAt.x =
                    (*sHands[0 as libc::c_int as usize]).actor.home.pos.x -
                        20.0f32;
                sCameraAt.y =
                    (*sHands[0 as libc::c_int as usize]).actor.home.pos.y +
                        0.0f32;
                sCameraAt.z =
                    (*sHands[0 as libc::c_int as usize]).actor.home.pos.z +
                        0.0f32;
                sCameraEye.x =
                    (*sHands[0 as libc::c_int as usize]).actor.home.pos.x -
                        70.0f32;
                sCameraEye.y =
                    (*sHands[0 as libc::c_int as usize]).actor.home.pos.y +
                        170.0f32;
                sCameraEye.z =
                    (*sHands[0 as libc::c_int as usize]).actor.home.pos.z +
                        150.0f32
            }
        } else if (*this).timer as libc::c_int >= 244 as libc::c_int {
            introStateTimer =
                (*this).timer as libc::c_int - 244 as libc::c_int;
            tempo = 4 as libc::c_int;
            if introStateTimer == 11 as libc::c_int {
                sCameraAt.x =
                    (*sHands[1 as libc::c_int as usize]).actor.home.pos.x +
                        30.0f32;
                sCameraAt.y =
                    (*sHands[1 as libc::c_int as usize]).actor.home.pos.y +
                        70.0f32;
                sCameraAt.z =
                    (*sHands[1 as libc::c_int as usize]).actor.home.pos.z +
                        40.0f32;
                sCameraEye.x =
                    (*sHands[1 as libc::c_int as usize]).actor.home.pos.x +
                        110.0f32;
                sCameraEye.y =
                    (*sHands[1 as libc::c_int as usize]).actor.home.pos.y -
                        140.0f32;
                sCameraEye.z =
                    (*sHands[1 as libc::c_int as usize]).actor.home.pos.z -
                        10.0f32
            } else if (*this).timer as libc::c_int == 300 as libc::c_int {
                sCameraAt.x =
                    (*sHands[0 as libc::c_int as usize]).actor.home.pos.x -
                        20.0f32;
                sCameraAt.y =
                    (*sHands[0 as libc::c_int as usize]).actor.home.pos.y -
                        80.0f32;
                sCameraAt.z =
                    (*sHands[0 as libc::c_int as usize]).actor.home.pos.z +
                        320.0f32;
                sCameraEye.x =
                    (*sHands[0 as libc::c_int as usize]).actor.home.pos.x -
                        130.0f32;
                sCameraEye.y =
                    (*sHands[0 as libc::c_int as usize]).actor.home.pos.y +
                        130.0f32;
                sCameraEye.z =
                    (*sHands[0 as libc::c_int as usize]).actor.home.pos.z -
                        150.0f32
            }
        } else if (*this).timer as libc::c_int >= 192 as libc::c_int {
            introStateTimer =
                (*this).timer as libc::c_int - 192 as libc::c_int;
            tempo = 3 as libc::c_int;
            if (*this).timer as libc::c_int == 240 as libc::c_int {
                sCameraAt.x =
                    (*sHands[0 as libc::c_int as usize]).actor.home.pos.x -
                        190.0f32;
                sCameraAt.y =
                    (*sHands[0 as libc::c_int as usize]).actor.home.pos.y -
                        110.0f32;
                sCameraAt.z =
                    (*sHands[0 as libc::c_int as usize]).actor.home.pos.z +
                        40.0f32;
                sCameraEye.x =
                    (*sHands[0 as libc::c_int as usize]).actor.home.pos.x +
                        120.0f32;
                sCameraEye.y =
                    (*sHands[0 as libc::c_int as usize]).actor.home.pos.y +
                        130.0f32;
                sCameraEye.z =
                    (*sHands[0 as libc::c_int as usize]).actor.home.pos.z +
                        50.0f32
            } else if introStateTimer == 12 as libc::c_int {
                sCameraAt.x = sRoomCenter.x + 50.0f32;
                sCameraAt.y = sRoomCenter.y - 90.0f32;
                sCameraAt.z = sRoomCenter.z - 200.0f32;
                sCameraEye.x = sRoomCenter.x + 50.0f32;
                sCameraEye.y = sRoomCenter.y + 350.0f32;
                sCameraEye.z = sRoomCenter.z + 150.0f32
            }
        } else if (*this).timer as libc::c_int >= 148 as libc::c_int {
            introStateTimer =
                (*this).timer as libc::c_int - 148 as libc::c_int;
            tempo = 2 as libc::c_int
        } else if (*this).timer as libc::c_int >= 112 as libc::c_int {
            introStateTimer =
                (*this).timer as libc::c_int - 112 as libc::c_int;
            tempo = 1 as libc::c_int
        } else {
            introStateTimer =
                (*this).timer as libc::c_int % 28 as libc::c_int;
            tempo = 0 as libc::c_int
        }
        if (*this).timer as libc::c_int <= 198 as libc::c_int {
            revealStateTimer =
                198 as libc::c_int - (*this).timer as libc::c_int;
            if gSaveContext.eventChkInf[7 as libc::c_int as usize] as
                   libc::c_int & 0x80 as libc::c_int != 0 &&
                   revealStateTimer <= 44 as libc::c_int {
                sCameraAt.x += 492.0f32 * 0.01f32;
                sCameraAt.y += 200.0f32 * 0.01f32;
                sCameraEye.x -= 80.0f32 * 0.01f32;
                sCameraEye.y -= 360.0f32 * 0.01f32;
                sCameraEye.z += 1000.0f32 * 0.01f32
            } else if (*this).timer as libc::c_int <= 20 as libc::c_int {
                sCameraAt.y -= 700.0f32 * 0.01f32;
                sCameraAt.z += 900.0f32 * 0.01f32;
                sCameraEye.x += 650.0f32 * 0.01f32;
                sCameraEye.y += 400.0f32 * 0.01f32;
                sCameraEye.z += 1550.0f32 * 0.01f32;
                (*this).actionVar = 1 as libc::c_int as s8;
                (*this).actor.flags |=
                    ((1 as libc::c_int) << 7 as libc::c_int) as libc::c_uint
            } else if revealStateTimer < 40 as libc::c_int {
                sCameraAt.x += 125.0f32 * 0.01f32;
                sCameraAt.y += 350.0f32 * 0.01f32;
                sCameraAt.z += 500.0f32 * 0.01f32;
                sCameraEye.x += 200.0f32 * 0.01f32;
                sCameraEye.y -= 850.0f32 * 0.01f32
            } else if revealStateTimer >= 45 as libc::c_int {
                if revealStateTimer < 85 as libc::c_int {
                    sCameraAt.x -= 250.0f32 * 0.01f32;
                    sCameraAt.y += 425.0f32 * 0.01f32;
                    sCameraAt.z -= 1200.0f32 * 0.01f32;
                    sCameraEye.x -= 650.0f32 * 0.01f32;
                    sCameraEye.y += 125.0f32 * 0.01f32;
                    sCameraEye.z -= 350.0f32 * 0.01f32
                } else if revealStateTimer == 85 as libc::c_int {
                    if gSaveContext.eventChkInf[7 as libc::c_int as usize] as
                           libc::c_int & 0x80 as libc::c_int == 0 {
                        TitleCard_InitBossName(globalCtx,
                                               &mut (*globalCtx).actorCtx.titleCtx,
                                               gSegments[((gBongoTitleCardTex.as_mut_ptr()
                                                               as u32_0) <<
                                                              4 as libc::c_int
                                                              >>
                                                              28 as
                                                                  libc::c_int)
                                                             as
                                                             usize].wrapping_add(gBongoTitleCardTex.as_mut_ptr()
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
                                                   as *mut libc::c_void,
                                               160 as libc::c_int as s16,
                                               180 as libc::c_int as s16,
                                               128 as libc::c_int as u8_0,
                                               40 as libc::c_int as u8_0);
                    }
                    Audio_QueueSeqCmd(((SEQ_PLAYER_BGM_MAIN as libc::c_int) <<
                                           24 as libc::c_int |
                                           0x1b as libc::c_int) as u32_0);
                    Animation_MorphToPlayOnce(&mut (*this).skelAnime,
                                              &mut gBongoHeadEyeCloseAnim,
                                              -5.0f32);
                    BossSst_HeadSfx(this, 0x396b as libc::c_int as u16_0);
                }
            }
        }
        if introStateTimer == 12 as libc::c_int {
            BossSst_HandSetupDownbeat(sHands[1 as libc::c_int as usize]);
        }
        if introStateTimer != 5 as libc::c_int &&
               introStateTimer % (tempo * 2 as libc::c_int + 7 as libc::c_int)
                   == 5 as libc::c_int {
            BossSst_HandSetupOffbeat(sHands[0 as libc::c_int as usize]);
        }
    }
    if (*this).actionFunc !=
           Some(BossSst_HeadNeutral as
                    unsafe extern "C" fn(_: *mut BossSst,
                                         _: *mut GlobalContext) -> ()) {
        Gameplay_CameraSetAtEye(globalCtx, sCutsceneCamera, &mut sCameraAt,
                                &mut sCameraEye);
    };
}
#[no_mangle]
pub unsafe extern "C" fn BossSst_HeadSetupWait(mut this: *mut BossSst) {
    if (*this).skelAnime.animation !=
           &mut gBongoHeadEyeCloseIdleAnim as *mut AnimationHeader as
               *mut libc::c_void {
        Animation_MorphToLoop(&mut (*this).skelAnime,
                              &mut gBongoHeadEyeCloseIdleAnim, -5.0f32);
    }
    (*this).actionFunc =
        Some(BossSst_HeadWait as
                 unsafe extern "C" fn(_: *mut BossSst, _: *mut GlobalContext)
                     -> ());
}
#[no_mangle]
pub unsafe extern "C" fn BossSst_HeadWait(mut this: *mut BossSst,
                                          mut globalCtx: *mut GlobalContext) {
    SkelAnime_Update(&mut (*this).skelAnime);
    if sHandState[(*sHands[0 as libc::c_int as usize]).actor.params as usize]
           == HAND_WAIT as libc::c_int &&
           sHandState[(*sHands[1 as libc::c_int as usize]).actor.params as
                          usize] == HAND_WAIT as libc::c_int {
        BossSst_HeadSetupNeutral(this);
    };
}
#[no_mangle]
pub unsafe extern "C" fn BossSst_HeadSetupNeutral(mut this: *mut BossSst) {
    (*this).timer = 127 as libc::c_int as s16;
    (*this).ready = 0 as libc::c_int as s8;
    (*this).actionFunc =
        Some(BossSst_HeadNeutral as
                 unsafe extern "C" fn(_: *mut BossSst, _: *mut GlobalContext)
                     -> ());
}
#[no_mangle]
pub unsafe extern "C" fn BossSst_HeadNeutral(mut this: *mut BossSst,
                                             mut globalCtx:
                                                 *mut GlobalContext) {
    SkelAnime_Update(&mut (*this).skelAnime);
    if (*this).ready == 0 &&
           (sHandState[(*sHands[0 as libc::c_int as usize]).actor.params as
                           usize] == HAND_BEAT as libc::c_int ||
                sHandState[(*sHands[0 as libc::c_int as usize]).actor.params
                               as usize] == HAND_WAIT as libc::c_int) &&
           (sHandState[(*sHands[1 as libc::c_int as usize]).actor.params as
                           usize] == HAND_BEAT as libc::c_int ||
                sHandState[(*sHands[1 as libc::c_int as usize]).actor.params
                               as usize] == HAND_WAIT as libc::c_int) {
        (*this).ready = 1 as libc::c_int as s8
    }
    if (*this).ready != 0 {
        if (*this).timer as libc::c_int != 0 as libc::c_int {
            (*this).timer -= 1
        }
    }
    if (*this).timer as libc::c_int == 0 as libc::c_int {
        if (*((*globalCtx).actorCtx.actorLists[ACTORCAT_PLAYER as libc::c_int
                                                   as usize].head as
                  *mut Player)).actor.world.pos.y > -50.0f32 &&
               (*((*globalCtx).actorCtx.actorLists[ACTORCAT_PLAYER as
                                                       libc::c_int as
                                                       usize].head as
                      *mut Player)).stateFlags1 &
                   0x6080 as libc::c_int as libc::c_uint == 0 {
            (*sHands[(Rand_ZeroOne() <= 0.5f32) as libc::c_int as
                         usize]).ready = 1 as libc::c_int as s8;
            BossSst_HeadSetupWait(this);
        } else { (*this).timer = 28 as libc::c_int as s16 }
    } else {
        Math_ApproachS(&mut (*this).actor.shape.rot.y,
                       (Actor_WorldYawTowardPoint(&mut (*((*(*globalCtx).actorCtx.actorLists.as_mut_ptr().offset(ACTORCAT_PLAYER
                                                                                                                     as
                                                                                                                     libc::c_int
                                                                                                                     as
                                                                                                                     isize)).head
                                                              as
                                                              *mut Player)).actor,
                                                  &mut sRoomCenter) as
                            libc::c_int + 0x8000 as libc::c_int) as s16,
                       4 as libc::c_int as s16, 0x400 as libc::c_int as s16);
        if (*this).timer as libc::c_int == 28 as libc::c_int ||
               (*this).timer as libc::c_int == 84 as libc::c_int {
            BossSst_HeadSfx(this, 0x398e as libc::c_int as u16_0);
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn BossSst_HeadSetupDamagedHand(mut this: *mut BossSst,
                                                      mut bothHands: s32) {
    if bothHands != 0 {
        Animation_MorphToPlayOnce(&mut (*this).skelAnime,
                                  &mut gBongoHeadEyeOpenAnim, -5.0f32);
    } else {
        Animation_MorphToPlayOnce(&mut (*this).skelAnime,
                                  &mut gBongoHeadDamagedHandAnim, -5.0f32);
    }
    (*this).actionFunc =
        Some(BossSst_HeadDamagedHand as
                 unsafe extern "C" fn(_: *mut BossSst, _: *mut GlobalContext)
                     -> ());
}
#[no_mangle]
pub unsafe extern "C" fn BossSst_HeadDamagedHand(mut this: *mut BossSst,
                                                 mut globalCtx:
                                                     *mut GlobalContext) {
    if SkelAnime_Update(&mut (*this).skelAnime) != 0 {
        if sHandState[(*sHands[0 as libc::c_int as usize]).actor.params as
                          usize] == HAND_DAMAGED as libc::c_int &&
               sHandState[(*sHands[1 as libc::c_int as usize]).actor.params as
                              usize] == HAND_DAMAGED as libc::c_int {
            BossSst_HeadSetupReadyCharge(this);
        } else if sHandState[(*sHands[0 as libc::c_int as usize]).actor.params
                                 as usize] == HAND_FROZEN as libc::c_int ||
                      sHandState[(*sHands[1 as libc::c_int as
                                              usize]).actor.params as usize]
                          == HAND_FROZEN as libc::c_int {
            BossSst_HeadSetupFrozenHand(this);
        } else if (*this).skelAnime.animation ==
                      &mut gBongoHeadEyeOpenAnim as *mut AnimationHeader as
                          *mut libc::c_void {
            BossSst_HeadSetupUnfreezeHand(this);
        } else { BossSst_HeadSetupWait(this); }
    };
}
#[no_mangle]
pub unsafe extern "C" fn BossSst_HeadSetupReadyCharge(mut this:
                                                          *mut BossSst) {
    Animation_MorphToLoop(&mut (*this).skelAnime,
                          &mut gBongoHeadEyeOpenIdleAnim, -5.0f32);
    (*this).actor.speedXZ = 0.0f32;
    (*this).colliderCyl.base.acFlags =
        ((*this).colliderCyl.base.acFlags as libc::c_int |
             (1 as libc::c_int) << 0 as libc::c_int) as u8_0;
    (*this).actionFunc =
        Some(BossSst_HeadReadyCharge as
                 unsafe extern "C" fn(_: *mut BossSst, _: *mut GlobalContext)
                     -> ());
}
#[no_mangle]
pub unsafe extern "C" fn BossSst_HeadReadyCharge(mut this: *mut BossSst,
                                                 mut globalCtx:
                                                     *mut GlobalContext) {
    SkelAnime_Update(&mut (*this).skelAnime);
    if (*sHands[0 as libc::c_int as usize]).ready as libc::c_int != 0 &&
           (*sHands[0 as libc::c_int as usize]).actionFunc ==
               Some(BossSst_HandReadyCharge as
                        unsafe extern "C" fn(_: *mut BossSst,
                                             _: *mut GlobalContext) -> ()) &&
           (*sHands[1 as libc::c_int as usize]).ready as libc::c_int != 0 &&
           (*sHands[1 as libc::c_int as usize]).actionFunc ==
               Some(BossSst_HandReadyCharge as
                        unsafe extern "C" fn(_: *mut BossSst,
                                             _: *mut GlobalContext) -> ()) {
        BossSst_HeadSetupCharge(this);
    } else {
        Math_SmoothStepToS(&mut (*this).actor.shape.rot.y,
                           (*this).actor.yawTowardsPlayer,
                           4 as libc::c_int as s16,
                           0x800 as libc::c_int as s16,
                           0x400 as libc::c_int as s16);
    };
}
#[no_mangle]
pub unsafe extern "C" fn BossSst_HeadSetupCharge(mut this: *mut BossSst) {
    Animation_Change(&mut (*this).skelAnime, &mut gBongoHeadChargeAnim,
                     0.5f32, 0.0f32,
                     Animation_GetLastFrame(&mut gBongoHeadChargeAnim as
                                                *mut AnimationHeader as
                                                *mut libc::c_void) as f32_0,
                     ANIMMODE_ONCE_INTERP as libc::c_int as u8_0, -5.0f32);
    BossSst_HandSetDamage(sHands[0 as libc::c_int as usize],
                          0x20 as libc::c_int);
    BossSst_HandSetDamage(sHands[1 as libc::c_int as usize],
                          0x20 as libc::c_int);
    (*this).colliderJntSph.base.atFlags =
        ((*this).colliderJntSph.base.atFlags as libc::c_int |
             (1 as libc::c_int) << 0 as libc::c_int) as u8_0;
    (*this).actor.speedXZ = 3.0f32;
    (*this).radius = -650.0f32;
    (*this).ready = 0 as libc::c_int as s8;
    (*this).actionFunc =
        Some(BossSst_HeadCharge as
                 unsafe extern "C" fn(_: *mut BossSst, _: *mut GlobalContext)
                     -> ());
}
#[no_mangle]
pub unsafe extern "C" fn BossSst_HeadCharge(mut this: *mut BossSst,
                                            mut globalCtx:
                                                *mut GlobalContext) {
    let mut chargeDist: f32_0 = 0.;
    let mut animFinish: s32 = SkelAnime_Update(&mut (*this).skelAnime);
    if (*this).ready == 0 &&
           Animation_OnFrame(&mut (*this).skelAnime, 6.0f32) != 0 {
        (*this).ready = 1 as libc::c_int as s8;
        (*this).actor.speedXZ = 0.25f32;
        (*this).skelAnime.playSpeed = 0.2f32
    }
    (*this).actor.speedXZ *= 1.25f32;
    (*this).actor.speedXZ =
        if (*this).actor.speedXZ > 45.0f32 {
            45.0f32
        } else { (*this).actor.speedXZ };
    if (*this).ready != 0 {
        if Math_SmoothStepToF(&mut (*this).radius, 650.0f32, 0.4f32,
                              (*this).actor.speedXZ, 1.0f32) < 10.0f32 {
            (*this).radius = 650.0f32;
            BossSst_HeadSetupEndCharge(this);
        } else {
            chargeDist = (650.0f32 - (*this).radius) * 3.0f32;
            if chargeDist > 180.0f32 { chargeDist = 180.0f32 }
            (*this).actor.world.pos.y = (*this).actor.home.pos.y - chargeDist
        }
        if animFinish == 0 {
            sHandOffsets[0 as libc::c_int as usize].z += 5.0f32;
            sHandOffsets[1 as libc::c_int as usize].z += 5.0f32
        }
    } else {
        Math_ApproachF(&mut (*this).radius, -700.0f32, 0.4f32,
                       (*this).actor.speedXZ);
        Math_StepToF(&mut (*this).actor.world.pos.y,
                     (*this).actor.home.pos.y - 180.0f32, 20.0f32);
        sHandOffsets[0 as libc::c_int as usize].y += 5.0f32;
        sHandOffsets[1 as libc::c_int as usize].y += 5.0f32
    }
    if (*this).colliderJntSph.base.atFlags as libc::c_int &
           (1 as libc::c_int) << 1 as libc::c_int != 0 {
        (*this).colliderJntSph.base.atFlags =
            ((*this).colliderJntSph.base.atFlags as libc::c_int &
                 !((1 as libc::c_int) << 0 as libc::c_int |
                       (1 as libc::c_int) << 1 as libc::c_int)) as u8_0;
        (*sHands[0 as libc::c_int as usize]).colliderJntSph.base.atFlags =
            ((*sHands[0 as libc::c_int as usize]).colliderJntSph.base.atFlags
                 as libc::c_int &
                 !((1 as libc::c_int) << 0 as libc::c_int |
                       (1 as libc::c_int) << 1 as libc::c_int)) as u8_0;
        (*sHands[1 as libc::c_int as usize]).colliderJntSph.base.atFlags =
            ((*sHands[1 as libc::c_int as usize]).colliderJntSph.base.atFlags
                 as libc::c_int &
                 !((1 as libc::c_int) << 0 as libc::c_int |
                       (1 as libc::c_int) << 1 as libc::c_int)) as u8_0;
        func_8002F71C(globalCtx, &mut (*this).actor, 10.0f32,
                      (*this).actor.shape.rot.y, 5.0f32);
        func_8002F7DC(&mut (*((*(*globalCtx).actorCtx.actorLists.as_mut_ptr().offset(ACTORCAT_PLAYER
                                                                                         as
                                                                                         libc::c_int
                                                                                         as
                                                                                         isize)).head
                                  as *mut Player)).actor,
                      0x83e as libc::c_int as u16_0);
    };
}
#[no_mangle]
pub unsafe extern "C" fn BossSst_HeadSetupEndCharge(mut this: *mut BossSst) {
    Animation_MorphToLoop(&mut (*this).skelAnime,
                          &mut gBongoHeadEyeCloseIdleAnim, -20.0f32);
    (*this).targetYaw =
        Actor_WorldYawTowardPoint(&mut (*this).actor, &mut sRoomCenter);
    (*this).colliderJntSph.base.atFlags =
        ((*this).colliderJntSph.base.atFlags as libc::c_int &
             !((1 as libc::c_int) << 0 as libc::c_int |
                   (1 as libc::c_int) << 1 as libc::c_int)) as u8_0;
    (*this).colliderCyl.base.acFlags =
        ((*this).colliderCyl.base.acFlags as libc::c_int &
             !((1 as libc::c_int) << 0 as libc::c_int)) as u8_0;
    (*this).radius *= -1.0f32;
    (*this).actionFunc =
        Some(BossSst_HeadEndCharge as
                 unsafe extern "C" fn(_: *mut BossSst, _: *mut GlobalContext)
                     -> ());
}
#[no_mangle]
pub unsafe extern "C" fn BossSst_HeadEndCharge(mut this: *mut BossSst,
                                               mut globalCtx:
                                                   *mut GlobalContext) {
    SkelAnime_Update(&mut (*this).skelAnime);
    if Math_SmoothStepToS(&mut (*this).actor.shape.rot.y, (*this).targetYaw,
                          4 as libc::c_int as s16,
                          0x800 as libc::c_int as s16,
                          0x100 as libc::c_int as s16) as libc::c_int ==
           0 as libc::c_int {
        BossSst_HandSetupRetreat(sHands[0 as libc::c_int as usize]);
        BossSst_HandSetupRetreat(sHands[1 as libc::c_int as usize]);
        BossSst_HeadSetupNeutral(this);
    };
}
#[no_mangle]
pub unsafe extern "C" fn BossSst_HeadSetupFrozenHand(mut this: *mut BossSst) {
    Animation_MorphToLoop(&mut (*this).skelAnime,
                          &mut gBongoHeadEyeOpenIdleAnim, -5.0f32);
    (*this).ready = 0 as libc::c_int as s8;
    (*this).colliderCyl.base.acFlags =
        ((*this).colliderCyl.base.acFlags as libc::c_int |
             (1 as libc::c_int) << 0 as libc::c_int) as u8_0;
    (*this).actionFunc =
        Some(BossSst_HeadFrozenHand as
                 unsafe extern "C" fn(_: *mut BossSst, _: *mut GlobalContext)
                     -> ());
}
#[no_mangle]
pub unsafe extern "C" fn BossSst_HeadFrozenHand(mut this: *mut BossSst,
                                                mut globalCtx:
                                                    *mut GlobalContext) {
    SkelAnime_Update(&mut (*this).skelAnime);
    if (*this).ready != 0 { BossSst_HeadSetupUnfreezeHand(this); };
}
#[no_mangle]
pub unsafe extern "C" fn BossSst_HeadSetupUnfreezeHand(mut this:
                                                           *mut BossSst) {
    Animation_MorphToPlayOnce(&mut (*this).skelAnime,
                              &mut gBongoHeadEyeCloseAnim, -5.0f32);
    (*this).colliderCyl.base.acFlags =
        ((*this).colliderCyl.base.acFlags as libc::c_int &
             !((1 as libc::c_int) << 0 as libc::c_int)) as u8_0;
    (*this).actionFunc =
        Some(BossSst_HeadUnfreezeHand as
                 unsafe extern "C" fn(_: *mut BossSst, _: *mut GlobalContext)
                     -> ());
}
#[no_mangle]
pub unsafe extern "C" fn BossSst_HeadUnfreezeHand(mut this: *mut BossSst,
                                                  mut globalCtx:
                                                      *mut GlobalContext) {
    if SkelAnime_Update(&mut (*this).skelAnime) != 0 {
        BossSst_HeadSetupWait(this);
    };
}
#[no_mangle]
pub unsafe extern "C" fn BossSst_HeadSetupStunned(mut this: *mut BossSst) {
    Animation_MorphToPlayOnce(&mut (*this).skelAnime,
                              &mut gBongoHeadKnockoutAnim, -5.0f32);
    Actor_SetColorFilter(&mut (*this).actor, 0 as libc::c_int as s16,
                         0xff as libc::c_int as s16, 0 as libc::c_int as s16,
                         Animation_GetLastFrame(&mut gBongoHeadKnockoutAnim as
                                                    *mut AnimationHeader as
                                                    *mut libc::c_void));
    (*this).colliderJntSph.base.atFlags =
        ((*this).colliderJntSph.base.atFlags as libc::c_int &
             !((1 as libc::c_int) << 0 as libc::c_int |
                   (1 as libc::c_int) << 1 as libc::c_int)) as u8_0;
    (*this).colliderCyl.base.acFlags =
        ((*this).colliderCyl.base.acFlags as libc::c_int &
             !((1 as libc::c_int) << 0 as libc::c_int)) as u8_0;
    (*this).actionVar = 0 as libc::c_int as s8;
    (*this).actor.flags &=
        !((1 as libc::c_int) << 7 as libc::c_int) as libc::c_uint;
    BossSst_HeadSfx(this, 0x396e as libc::c_int as u16_0);
    (*this).actionFunc =
        Some(BossSst_HeadStunned as
                 unsafe extern "C" fn(_: *mut BossSst, _: *mut GlobalContext)
                     -> ());
}
#[no_mangle]
pub unsafe extern "C" fn BossSst_HeadStunned(mut this: *mut BossSst,
                                             mut globalCtx:
                                                 *mut GlobalContext) {
    let mut bounce: f32_0 = 0.;
    let mut animFinish: s32 = 0;
    let mut currentFrame: f32_0 = 0.;
    Math_StepToF(&mut (*sHandOffsets.as_mut_ptr().offset(0 as libc::c_int as
                                                             isize)).z,
                 600.0f32, 20.0f32);
    Math_StepToF(&mut (*sHandOffsets.as_mut_ptr().offset(1 as libc::c_int as
                                                             isize)).z,
                 600.0f32, 20.0f32);
    Math_StepToF(&mut (*sHandOffsets.as_mut_ptr().offset(0 as libc::c_int as
                                                             isize)).x,
                 200.0f32, 20.0f32);
    Math_StepToF(&mut (*sHandOffsets.as_mut_ptr().offset(1 as libc::c_int as
                                                             isize)).x,
                 -200.0f32, 20.0f32);
    (*this).actor.velocity.y += (*this).actor.gravity;
    animFinish = SkelAnime_Update(&mut (*this).skelAnime);
    currentFrame = (*this).skelAnime.curFrame;
    if currentFrame <= 6.0f32 {
        bounce =
            sinf(3.14159265358979323846f32 /
                     11 as libc::c_int as libc::c_float * currentFrame) *
                100.0f32 + ((*this).actor.home.pos.y - 180.0f32);
        if (*this).actor.world.pos.y < bounce {
            (*this).actor.world.pos.y = bounce
        }
    } else if currentFrame <= 11.0f32 {
        (*this).actor.world.pos.y =
            sinf(3.14159265358979323846f32 /
                     11 as libc::c_int as libc::c_float * currentFrame) *
                170.0f32 + ((*this).actor.home.pos.y - 250.0f32)
    } else {
        (*this).actor.world.pos.y =
            sinf((currentFrame - 11.0f32) *
                     (3.14159265358979323846f32 /
                          5 as libc::c_int as libc::c_float)) * 50.0f32 +
                ((*this).actor.home.pos.y - 250.0f32)
    }
    if animFinish != 0 ||
           Animation_OnFrame(&mut (*this).skelAnime, 11.0f32) != 0 {
        BossSst_HeadSfx(this, 0x3966 as libc::c_int as u16_0);
    }
    if (*this).radius < -500.0f32 {
        Math_SmoothStepToF(&mut (*this).radius, -500.0f32, 1.0f32, 50.0f32,
                           5.0f32);
    } else {
        Math_SmoothStepToF(&mut (*this).actor.speedXZ, 0.0f32, 0.5f32,
                           15.0f32, 3.0f32);
        (*this).radius += (*this).actor.speedXZ
    }
    (*this).radius =
        if (*this).radius > 400.0f32 { 400.0f32 } else { (*this).radius };
    (*this).actor.world.pos.y += (*this).actor.velocity.y;
    if animFinish != 0 { BossSst_HeadSetupVulnerable(this); };
}
#[no_mangle]
pub unsafe extern "C" fn BossSst_HeadSetupVulnerable(mut this: *mut BossSst) {
    Animation_MorphToLoop(&mut (*this).skelAnime, &mut gBongoHeadStunnedAnim,
                          -5.0f32);
    (*this).colliderCyl.base.acFlags =
        ((*this).colliderCyl.base.acFlags as libc::c_int |
             (1 as libc::c_int) << 0 as libc::c_int) as u8_0;
    (*this).colliderCyl.info.bumper.dmgFlags =
        0xfc00702 as libc::c_int as u32_0;
    (*this).actor.speedXZ = 0.0f32;
    let ref mut fresh1 =
        (*(*this).colliderJntSph.elements.offset(10 as libc::c_int as
                                                     isize)).info.bumperFlags;
    *fresh1 =
        (*fresh1 as libc::c_int |
             ((1 as libc::c_int) << 0 as libc::c_int |
                  (1 as libc::c_int) << 2 as libc::c_int)) as u8_0;
    let ref mut fresh2 =
        (*(*this).colliderJntSph.elements.offset(0 as libc::c_int as
                                                     isize)).info.bumperFlags;
    *fresh2 =
        (*fresh2 as libc::c_int & !((1 as libc::c_int) << 0 as libc::c_int))
            as u8_0;
    if (*this).actionFunc !=
           Some(BossSst_HeadDamage as
                    unsafe extern "C" fn(_: *mut BossSst,
                                         _: *mut GlobalContext) -> ()) {
        (*this).timer = 50 as libc::c_int as s16
    }
    (*this).actionFunc =
        Some(BossSst_HeadVulnerable as
                 unsafe extern "C" fn(_: *mut BossSst, _: *mut GlobalContext)
                     -> ());
}
#[no_mangle]
pub unsafe extern "C" fn BossSst_HeadVulnerable(mut this: *mut BossSst,
                                                mut globalCtx:
                                                    *mut GlobalContext) {
    SkelAnime_Update(&mut (*this).skelAnime);
    Math_StepToF(&mut (*sHandOffsets.as_mut_ptr().offset(0 as libc::c_int as
                                                             isize)).z,
                 600.0f32, 20.0f32);
    Math_StepToF(&mut (*sHandOffsets.as_mut_ptr().offset(1 as libc::c_int as
                                                             isize)).z,
                 600.0f32, 20.0f32);
    Math_StepToF(&mut (*sHandOffsets.as_mut_ptr().offset(0 as libc::c_int as
                                                             isize)).x,
                 200.0f32, 20.0f32);
    Math_StepToF(&mut (*sHandOffsets.as_mut_ptr().offset(1 as libc::c_int as
                                                             isize)).x,
                 -200.0f32, 20.0f32);
    if (*this).actor.flags &
           ((1 as libc::c_int) << 13 as libc::c_int) as libc::c_uint ==
           ((1 as libc::c_int) << 13 as libc::c_int) as libc::c_uint {
        (*this).timer =
            ((*this).timer as libc::c_int + 2 as libc::c_int) as s16;
        (*this).timer =
            if (*this).timer as libc::c_int > 50 as libc::c_int {
                50 as libc::c_int
            } else { (*this).timer as libc::c_int } as s16
    } else {
        if (*this).timer as libc::c_int != 0 as libc::c_int {
            (*this).timer -= 1
        }
        if (*this).timer as libc::c_int == 0 as libc::c_int {
            BossSst_HandSetupRecover(sHands[0 as libc::c_int as usize]);
            BossSst_HandSetupRecover(sHands[1 as libc::c_int as usize]);
            BossSst_HeadSetupRecover(this);
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn BossSst_HeadSetupDamage(mut this: *mut BossSst) {
    Animation_MorphToPlayOnce(&mut (*this).skelAnime,
                              &mut gBongoHeadDamageAnim, -3.0f32);
    Actor_SetColorFilter(&mut (*this).actor, 0x4000 as libc::c_int as s16,
                         0xff as libc::c_int as s16, 0 as libc::c_int as s16,
                         Animation_GetLastFrame(&mut gBongoHeadDamageAnim as
                                                    *mut AnimationHeader as
                                                    *mut libc::c_void));
    Actor_SetColorFilter(&mut (**sHands.as_mut_ptr().offset(0 as libc::c_int
                                                                as
                                                                isize)).actor,
                         0x4000 as libc::c_int as s16,
                         0xff as libc::c_int as s16, 0 as libc::c_int as s16,
                         Animation_GetLastFrame(&mut gBongoHeadDamageAnim as
                                                    *mut AnimationHeader as
                                                    *mut libc::c_void));
    Actor_SetColorFilter(&mut (**sHands.as_mut_ptr().offset(1 as libc::c_int
                                                                as
                                                                isize)).actor,
                         0x4000 as libc::c_int as s16,
                         0xff as libc::c_int as s16, 0 as libc::c_int as s16,
                         Animation_GetLastFrame(&mut gBongoHeadDamageAnim as
                                                    *mut AnimationHeader as
                                                    *mut libc::c_void));
    (*this).colliderCyl.base.acFlags =
        ((*this).colliderCyl.base.acFlags as libc::c_int &
             !((1 as libc::c_int) << 0 as libc::c_int)) as u8_0;
    BossSst_HeadSfx(this, 0x3969 as libc::c_int as u16_0);
    (*this).actionFunc =
        Some(BossSst_HeadDamage as
                 unsafe extern "C" fn(_: *mut BossSst, _: *mut GlobalContext)
                     -> ());
}
#[no_mangle]
pub unsafe extern "C" fn BossSst_HeadDamage(mut this: *mut BossSst,
                                            mut globalCtx:
                                                *mut GlobalContext) {
    if (*this).timer as libc::c_int != 0 as libc::c_int { (*this).timer -= 1 }
    if SkelAnime_Update(&mut (*this).skelAnime) != 0 {
        BossSst_HeadSetupVulnerable(this);
    };
}
#[no_mangle]
pub unsafe extern "C" fn BossSst_HeadSetupRecover(mut this: *mut BossSst) {
    Animation_MorphToPlayOnce(&mut (*this).skelAnime,
                              &mut gBongoHeadRecoverAnim, -5.0f32);
    (*this).colliderCyl.base.acFlags =
        ((*this).colliderCyl.base.acFlags as libc::c_int &
             !((1 as libc::c_int) << 0 as libc::c_int)) as u8_0;
    (*this).colliderCyl.info.bumper.dmgFlags = 0xffcfffff as libc::c_uint;
    let ref mut fresh3 =
        (*(*this).colliderJntSph.elements.offset(10 as libc::c_int as
                                                     isize)).info.bumperFlags;
    *fresh3 =
        (*fresh3 as libc::c_int &
             !((1 as libc::c_int) << 0 as libc::c_int |
                   (1 as libc::c_int) << 2 as libc::c_int)) as u8_0;
    let ref mut fresh4 =
        (*(*this).colliderJntSph.elements.offset(0 as libc::c_int as
                                                     isize)).info.bumperFlags;
    *fresh4 =
        (*fresh4 as libc::c_int | (1 as libc::c_int) << 0 as libc::c_int) as
            u8_0;
    (*this).actionVar = 1 as libc::c_int as s8;
    (*this).actor.speedXZ = 5.0f32;
    (*this).actionFunc =
        Some(BossSst_HeadRecover as
                 unsafe extern "C" fn(_: *mut BossSst, _: *mut GlobalContext)
                     -> ());
}
#[no_mangle]
pub unsafe extern "C" fn BossSst_HeadRecover(mut this: *mut BossSst,
                                             mut globalCtx:
                                                 *mut GlobalContext) {
    let mut animFinish: s32 = 0;
    let mut currentFrame: f32_0 = 0.;
    let mut diff: f32_0 = 0.;
    animFinish = SkelAnime_Update(&mut (*this).skelAnime);
    currentFrame = (*this).skelAnime.curFrame;
    if currentFrame < 10.0f32 {
        (*this).actor.world.pos.y += 10.0f32;
        sHandOffsets[0 as libc::c_int as usize].y -= 10.0f32;
        sHandOffsets[1 as libc::c_int as usize].y -= 10.0f32;
        Math_SmoothStepToF(&mut (*this).radius, -750.0f32, 1.0f32,
                           (*this).actor.speedXZ, 2.0f32);
    } else {
        (*this).actor.speedXZ *= 1.25f32;
        (*this).actor.speedXZ =
            if (*this).actor.speedXZ > 50.0f32 {
                50.0f32
            } else { (*this).actor.speedXZ };
        diff =
            Math_SmoothStepToF(&mut (*this).radius, -650.0f32, 1.0f32,
                               (*this).actor.speedXZ, 2.0f32);
        diff +=
            Math_SmoothStepToF(&mut (*this).actor.world.pos.y,
                               (*this).actor.home.pos.y, 0.5f32, 30.0f32,
                               3.0f32)
    }
    if animFinish != 0 && diff < 10.0f32 {
        (*this).actor.world.pos.y = (*this).actor.home.pos.y;
        (*this).radius = -650.0f32;
        BossSst_HandSetupRetreat(sHands[0 as libc::c_int as usize]);
        BossSst_HandSetupRetreat(sHands[1 as libc::c_int as usize]);
        BossSst_HeadSetupNeutral(this);
    };
}
#[no_mangle]
pub unsafe extern "C" fn BossSst_SetCameraTargets(mut cameraSpeedMod: f32_0,
                                                  mut targetIndex: s32) {
    let mut nextAt: *mut Vec3f =
        &mut *sCameraAtPoints.as_mut_ptr().offset(targetIndex as isize) as
            *mut Vec3f;
    let mut nextEye: *mut Vec3f =
        &mut *sCameraEyePoints.as_mut_ptr().offset(targetIndex as isize) as
            *mut Vec3f;
    if targetIndex != 0 as libc::c_int {
        Math_Vec3f_Copy(&mut sCameraAt,
                        &mut *sCameraAtPoints.as_mut_ptr().offset((targetIndex
                                                                       -
                                                                       1 as
                                                                           libc::c_int)
                                                                      as
                                                                      isize));
        Math_Vec3f_Copy(&mut sCameraEye,
                        &mut *sCameraEyePoints.as_mut_ptr().offset((targetIndex
                                                                        -
                                                                        1 as
                                                                            libc::c_int)
                                                                       as
                                                                       isize));
    }
    sCameraAtVel.x = ((*nextAt).x - sCameraAt.x) * cameraSpeedMod;
    sCameraAtVel.y = ((*nextAt).y - sCameraAt.y) * cameraSpeedMod;
    sCameraAtVel.z = ((*nextAt).z - sCameraAt.z) * cameraSpeedMod;
    sCameraEyeVel.x = ((*nextEye).x - sCameraEye.x) * cameraSpeedMod;
    sCameraEyeVel.y = ((*nextEye).y - sCameraEye.y) * cameraSpeedMod;
    sCameraEyeVel.z = ((*nextEye).z - sCameraEye.z) * cameraSpeedMod;
}
#[no_mangle]
pub unsafe extern "C" fn BossSst_UpdateDeathCamera(mut this: *mut BossSst,
                                                   mut globalCtx:
                                                       *mut GlobalContext) {
    let mut cameraAt: Vec3f = Vec3f{x: 0., y: 0., z: 0.,};
    let mut cameraEye: Vec3f = Vec3f{x: 0., y: 0., z: 0.,};
    let mut sn: f32_0 = 0.;
    let mut cs: f32_0 = 0.;
    sCameraAt.x += sCameraAtVel.x;
    sCameraAt.y += sCameraAtVel.y;
    sCameraAt.z += sCameraAtVel.z;
    sCameraEye.x += sCameraEyeVel.x;
    sCameraEye.y += sCameraEyeVel.y;
    sCameraEye.z += sCameraEyeVel.z;
    sn = Math_SinS((*this).actor.shape.rot.y);
    cs = Math_CosS((*this).actor.shape.rot.y);
    cameraAt.x =
        (*this).actor.world.pos.x + sCameraAt.z * sn + sCameraAt.x * cs;
    cameraAt.y = (*this).actor.home.pos.y - 140.0f32 + sCameraAt.y;
    cameraAt.z =
        (*this).actor.world.pos.z + sCameraAt.z * cs - sCameraAt.x * sn;
    cameraEye.x =
        (*this).actor.world.pos.x + sCameraEye.z * sn + sCameraEye.x * cs;
    cameraEye.y = (*this).actor.home.pos.y - 140.0f32 + sCameraEye.y;
    cameraEye.z =
        (*this).actor.world.pos.z + sCameraEye.z * cs - sCameraEye.x * sn;
    Gameplay_CameraSetAtEye(globalCtx, sCutsceneCamera, &mut cameraAt,
                            &mut cameraEye);
}
#[no_mangle]
pub unsafe extern "C" fn BossSst_HeadSetupDeath(mut this: *mut BossSst,
                                                mut globalCtx:
                                                    *mut GlobalContext) {
    let mut player: *mut Player =
        (*globalCtx).actorCtx.actorLists[ACTORCAT_PLAYER as libc::c_int as
                                             usize].head as *mut Player;
    Animation_MorphToLoop(&mut (*this).skelAnime,
                          &mut gBongoHeadEyeOpenIdleAnim, -5.0f32);
    BossSst_HeadSfx(this, 0x396f as libc::c_int as u16_0);
    Actor_SetColorFilter(&mut (*this).actor, 0x4000 as libc::c_int as s16,
                         0xff as libc::c_int as s16, 0 as libc::c_int as s16,
                         60 as libc::c_int as s16);
    Actor_SetColorFilter(&mut (**sHands.as_mut_ptr().offset(0 as libc::c_int
                                                                as
                                                                isize)).actor,
                         0x4000 as libc::c_int as s16,
                         0xff as libc::c_int as s16, 0 as libc::c_int as s16,
                         60 as libc::c_int as s16);
    Actor_SetColorFilter(&mut (**sHands.as_mut_ptr().offset(1 as libc::c_int
                                                                as
                                                                isize)).actor,
                         0x4000 as libc::c_int as s16,
                         0xff as libc::c_int as s16, 0 as libc::c_int as s16,
                         60 as libc::c_int as s16);
    (*this).timer = 60 as libc::c_int as s16;
    (*this).colliderCyl.base.acFlags =
        ((*this).colliderCyl.base.acFlags as libc::c_int &
             !((1 as libc::c_int) << 0 as libc::c_int)) as u8_0;
    (*this).colliderJntSph.base.ocFlags1 =
        ((*this).colliderJntSph.base.ocFlags1 as libc::c_int &
             !((1 as libc::c_int) << 0 as libc::c_int)) as u8_0;
    (*sHands[0 as libc::c_int as usize]).colliderJntSph.base.ocFlags1 =
        ((*sHands[0 as libc::c_int as usize]).colliderJntSph.base.ocFlags1 as
             libc::c_int & !((1 as libc::c_int) << 0 as libc::c_int)) as u8_0;
    (*sHands[1 as libc::c_int as usize]).colliderJntSph.base.ocFlags1 =
        ((*sHands[1 as libc::c_int as usize]).colliderJntSph.base.ocFlags1 as
             libc::c_int & !((1 as libc::c_int) << 0 as libc::c_int)) as u8_0;
    Audio_QueueSeqCmd(((0x1 as libc::c_int) << 28 as libc::c_int |
                           (SEQ_PLAYER_BGM_MAIN as libc::c_int) <<
                               24 as libc::c_int | 0x100ff as libc::c_int) as
                          u32_0);
    sCutsceneCamera = Gameplay_CreateSubCamera(globalCtx);
    Gameplay_ChangeCameraStatus(globalCtx, 0 as libc::c_int as s16,
                                1 as libc::c_int as s16);
    Gameplay_ChangeCameraStatus(globalCtx, sCutsceneCamera,
                                7 as libc::c_int as s16);
    Gameplay_CopyCamera(globalCtx, sCutsceneCamera, 0 as libc::c_int as s16);
    func_8002DF54(globalCtx, &mut (*player).actor, 8 as libc::c_int as u8_0);
    func_80064520(globalCtx, &mut (*globalCtx).csCtx);
    Math_Vec3f_Copy(&mut sCameraEye,
                    &mut (**(*globalCtx).cameraPtrs.as_mut_ptr().offset((*globalCtx).activeCamera
                                                                            as
                                                                            isize)).eye);
    (*this).actionFunc =
        Some(BossSst_HeadDeath as
                 unsafe extern "C" fn(_: *mut BossSst, _: *mut GlobalContext)
                     -> ());
}
#[no_mangle]
pub unsafe extern "C" fn BossSst_HeadDeath(mut this: *mut BossSst,
                                           mut globalCtx:
                                               *mut GlobalContext) {
    SkelAnime_Update(&mut (*this).skelAnime);
    if (*this).timer as libc::c_int != 0 as libc::c_int { (*this).timer -= 1 }
    Math_StepToF(&mut (*this).actor.world.pos.y,
                 (*this).actor.home.pos.y - 140.0f32, 20.0f32);
    if (*this).timer as libc::c_int == 0 as libc::c_int {
        BossSst_HandSetupThrash(sHands[0 as libc::c_int as usize]);
        BossSst_HandSetupThrash(sHands[1 as libc::c_int as usize]);
        BossSst_HeadSetupThrash(this);
    } else if (*this).timer as libc::c_int > 48 as libc::c_int {
        Gameplay_CameraSetAtEye(globalCtx, sCutsceneCamera,
                                &mut (*this).actor.focus.pos,
                                &mut sCameraEye);
        Math_StepToF(&mut (*this).radius, -350.0f32, 10.0f32);
    } else if (*this).timer as libc::c_int == 48 as libc::c_int {
        let mut player: *mut Player =
            (*globalCtx).actorCtx.actorLists[ACTORCAT_PLAYER as libc::c_int as
                                                 usize].head as *mut Player;
        (*player).actor.world.pos.x =
            sRoomCenter.x + 400.0f32 * Math_SinS((*this).actor.shape.rot.y) +
                Math_CosS((*this).actor.shape.rot.y) * -120.0f32;
        (*player).actor.world.pos.z =
            sRoomCenter.z + 400.0f32 * Math_CosS((*this).actor.shape.rot.y) -
                Math_SinS((*this).actor.shape.rot.y) * -120.0f32;
        (*player).actor.shape.rot.y =
            Actor_WorldYawTowardPoint(&mut (*player).actor, &mut sRoomCenter);
        func_8002DBD0(&mut (*this).actor, &mut sCameraEye,
                      &mut (**(*globalCtx).cameraPtrs.as_mut_ptr().offset((*globalCtx).activeCamera
                                                                              as
                                                                              isize)).eye);
        func_8002DBD0(&mut (*this).actor, &mut sCameraAt,
                      &mut (**(*globalCtx).cameraPtrs.as_mut_ptr().offset((*globalCtx).activeCamera
                                                                              as
                                                                              isize)).at);
        (*this).radius = -350.0f32;
        (*this).actor.world.pos.x =
            sRoomCenter.x - Math_SinS((*this).actor.shape.rot.y) * 350.0f32;
        (*this).actor.world.pos.z =
            sRoomCenter.z - Math_CosS((*this).actor.shape.rot.y) * 350.0f32;
        BossSst_SetCameraTargets((1.0f64 /
                                      48 as libc::c_int as libc::c_double) as
                                     f32_0, 0 as libc::c_int);
        BossSst_UpdateDeathCamera(this, globalCtx);
    } else { BossSst_UpdateDeathCamera(this, globalCtx); };
}
#[no_mangle]
pub unsafe extern "C" fn BossSst_HeadSetupThrash(mut this: *mut BossSst) {
    Animation_MorphToPlayOnce(&mut (*this).skelAnime,
                              &mut gBongoHeadEyeOpenIdleAnim, -5.0f32);
    (*this).timer = 160 as libc::c_int as s16;
    (*this).targetYaw = (*this).actor.shape.rot.y;
    BossSst_SetCameraTargets((1.0f64 / 80 as libc::c_int as libc::c_double) as
                                 f32_0, 1 as libc::c_int);
    (*this).actionFunc =
        Some(BossSst_HeadThrash as
                 unsafe extern "C" fn(_: *mut BossSst, _: *mut GlobalContext)
                     -> ());
}
#[no_mangle]
pub unsafe extern "C" fn BossSst_HeadThrash(mut this: *mut BossSst,
                                            mut globalCtx:
                                                *mut GlobalContext) {
    SkelAnime_Update(&mut (*this).skelAnime);
    if (*this).timer as libc::c_int != 0 as libc::c_int { (*this).timer -= 1 }
    if (*this).timer as libc::c_int == 0 as libc::c_int &&
           (*this).actor.shape.rot.y as libc::c_int ==
               (*this).targetYaw as libc::c_int {
        BossSst_HeadSetupDarken(this);
    } else if (*this).timer as libc::c_int >= 80 as libc::c_int {
        BossSst_UpdateDeathCamera(this, globalCtx);
    };
}
#[no_mangle]
pub unsafe extern "C" fn BossSst_HeadSetupDarken(mut this: *mut BossSst) {
    (*this).timer = 160 as libc::c_int as s16;
    BossSst_SetCameraTargets((1.0f64 / 80 as libc::c_int as libc::c_double) as
                                 f32_0, 2 as libc::c_int);
    (*this).actionFunc =
        Some(BossSst_HeadDarken as
                 unsafe extern "C" fn(_: *mut BossSst, _: *mut GlobalContext)
                     -> ());
}
#[no_mangle]
pub unsafe extern "C" fn BossSst_HeadDarken(mut this: *mut BossSst,
                                            mut globalCtx:
                                                *mut GlobalContext) {
    if (*this).timer as libc::c_int != 0 as libc::c_int { (*this).timer -= 1 }
    if (*this).timer as libc::c_int >= 80 as libc::c_int {
        if (*this).timer as libc::c_int == 80 as libc::c_int {
            sBodyStatic = 1 as libc::c_int as u32_0
        }
        BossSst_UpdateDeathCamera(this, globalCtx);
        sBodyColor.b =
            ((*this).timer as libc::c_int * 3 as libc::c_int -
                 240 as libc::c_int) as u8_0;
        sBodyColor.g = sBodyColor.b;
        sBodyColor.r = sBodyColor.g;
        if (*this).timer as libc::c_int == 80 as libc::c_int {
            BossSst_SetCameraTargets((1.0f64 /
                                          80 as libc::c_int as libc::c_double)
                                         as f32_0, 3 as libc::c_int);
        }
    } else {
        sBodyColor.b =
            ((80 as libc::c_int - (*this).timer as libc::c_int) as
                 libc::c_float / 1.0f32) as u8_0;
        sStaticColor.b =
            ((80 as libc::c_int - (*this).timer as libc::c_int) as
                 libc::c_float / 8.0f32) as u8_0;
        sStaticColor.g = sStaticColor.b;
        sStaticColor.r = sStaticColor.g;
        sBodyColor.g = sStaticColor.r;
        sBodyColor.r = sBodyColor.g;
        BossSst_UpdateDeathCamera(this, globalCtx);
        if (*this).timer as libc::c_int == 0 as libc::c_int {
            BossSst_HeadSetupFall(this);
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn BossSst_HeadSetupFall(mut this: *mut BossSst) {
    (*this).actor.speedXZ = 1.0f32;
    Math_Vec3f_Copy(&mut sCameraAt,
                    &mut *sCameraAtPoints.as_mut_ptr().offset(3 as libc::c_int
                                                                  as isize));
    Math_Vec3f_Copy(&mut sCameraEye,
                    &mut *sCameraEyePoints.as_mut_ptr().offset(3 as
                                                                   libc::c_int
                                                                   as isize));
    sCameraAtVel.x = 0.0f32;
    sCameraAtVel.z = 0.0f32;
    sCameraAtVel.y = -50.0f32;
    Math_Vec3f_Copy(&mut sCameraEyeVel, &mut sZeroVec);
    (*this).actionFunc =
        Some(BossSst_HeadFall as
                 unsafe extern "C" fn(_: *mut BossSst, _: *mut GlobalContext)
                     -> ());
}
#[no_mangle]
pub unsafe extern "C" fn BossSst_HeadFall(mut this: *mut BossSst,
                                          mut globalCtx: *mut GlobalContext) {
    (*this).actor.speedXZ *= 1.5f32;
    if Math_StepToF(&mut (*this).actor.world.pos.y,
                    (*this).actor.home.pos.y - 230.0f32,
                    (*this).actor.speedXZ) != 0 {
        BossSst_HeadSetupMelt(this);
    }
    if sCameraAt.y > 200.0f32 { BossSst_UpdateDeathCamera(this, globalCtx); };
}
#[no_mangle]
pub unsafe extern "C" fn BossSst_HeadSetupMelt(mut this: *mut BossSst) {
    BossSst_SpawnHeadShadow(this);
    (*this).timer = 80 as libc::c_int as s16;
    BossSst_SetCameraTargets((1.0f64 / 60 as libc::c_int as libc::c_double) as
                                 f32_0, 5 as libc::c_int);
    (*this).actionFunc =
        Some(BossSst_HeadMelt as
                 unsafe extern "C" fn(_: *mut BossSst, _: *mut GlobalContext)
                     -> ());
}
#[no_mangle]
pub unsafe extern "C" fn BossSst_HeadMelt(mut this: *mut BossSst,
                                          mut globalCtx: *mut GlobalContext) {
    if (*this).timer as libc::c_int != 0 as libc::c_int { (*this).timer -= 1 }
    (*this).actor.scale.y -= 0.00025f32;
    (*this).actor.scale.x += 0.000075f32;
    (*this).actor.scale.z += 0.000075f32;
    (*this).actor.world.pos.y =
        (*this).actor.home.pos.y - 11500.0f32 * (*this).actor.scale.y;
    if (*this).timer as libc::c_int == 0 as libc::c_int {
        BossSst_HeadSetupFinish(this);
    } else if (*this).timer as libc::c_int as libc::c_float >= 20.0f32 {
        BossSst_UpdateDeathCamera(this, globalCtx);
    };
}
#[no_mangle]
pub unsafe extern "C" fn BossSst_HeadSetupFinish(mut this: *mut BossSst) {
    (*this).actor.draw =
        Some(BossSst_DrawEffect as
                 unsafe extern "C" fn(_: *mut Actor, _: *mut GlobalContext)
                     -> ());
    (*this).timer = 40 as libc::c_int as s16;
    Audio_QueueSeqCmd(((SEQ_PLAYER_BGM_MAIN as libc::c_int) <<
                           24 as libc::c_int | 0x21 as libc::c_int) as u32_0);
    BossSst_SetCameraTargets((1.0f64 / 40 as libc::c_int as libc::c_double) as
                                 f32_0, 6 as libc::c_int);
    (*this).actionFunc =
        Some(BossSst_HeadFinish as
                 unsafe extern "C" fn(_: *mut BossSst, _: *mut GlobalContext)
                     -> ());
}
#[no_mangle]
pub unsafe extern "C" fn BossSst_HeadFinish(mut this: *mut BossSst,
                                            mut globalCtx:
                                                *mut GlobalContext) {
    static mut colorIndigo: Color_RGBA8 =
        {
            let mut init =
                Color_RGBA8{r: 80 as libc::c_int as u8_0,
                            g: 80 as libc::c_int as u8_0,
                            b: 150 as libc::c_int as u8_0,
                            a: 255 as libc::c_int as u8_0,};
            init
        };
    static mut colorDarkIndigo: Color_RGBA8 =
        {
            let mut init =
                Color_RGBA8{r: 40 as libc::c_int as u8_0,
                            g: 40 as libc::c_int as u8_0,
                            b: 80 as libc::c_int as u8_0,
                            a: 255 as libc::c_int as u8_0,};
            init
        };
    static mut colorUnused: [Color_RGBA8; 2] =
        [{
             let mut init =
                 Color_RGBA8{r: 0 as libc::c_int as u8_0,
                             g: 0 as libc::c_int as u8_0,
                             b: 0 as libc::c_int as u8_0,
                             a: 255 as libc::c_int as u8_0,};
             init
         },
         {
             let mut init =
                 Color_RGBA8{r: 100 as libc::c_int as u8_0,
                             g: 100 as libc::c_int as u8_0,
                             b: 100 as libc::c_int as u8_0,
                             a: 0 as libc::c_int as u8_0,};
             init
         }];
    let mut spawnPos: Vec3f = Vec3f{x: 0., y: 0., z: 0.,};
    let mut i: s32 = 0;
    (*this).timer -= 1;
    if (*this).effectMode as libc::c_int == BONGO_NULL as libc::c_int {
        if ((*this).timer as libc::c_int) < -(170 as libc::c_int) {
            BossSst_UpdateDeathCamera(this, globalCtx);
            Gameplay_CopyCamera(globalCtx, 0 as libc::c_int as s16,
                                sCutsceneCamera);
            Gameplay_ChangeCameraStatus(globalCtx, sCutsceneCamera,
                                        1 as libc::c_int as s16);
            Gameplay_ChangeCameraStatus(globalCtx, 0 as libc::c_int as s16,
                                        7 as libc::c_int as s16);
            Gameplay_ClearCamera(globalCtx, sCutsceneCamera);
            func_8002DF54(globalCtx,
                          &mut (*((*(*globalCtx).actorCtx.actorLists.as_mut_ptr().offset(ACTORCAT_PLAYER
                                                                                             as
                                                                                             libc::c_int
                                                                                             as
                                                                                             isize)).head
                                      as *mut Player)).actor,
                          7 as libc::c_int as u8_0);
            func_80064534(globalCtx, &mut (*globalCtx).csCtx);
            Actor_Kill(&mut (*this).actor);
            Actor_Kill(&mut (**sHands.as_mut_ptr().offset(0 as libc::c_int as
                                                              isize)).actor);
            Actor_Kill(&mut (**sHands.as_mut_ptr().offset(1 as libc::c_int as
                                                              isize)).actor);
            Flags_SetClear(globalCtx,
                           (*globalCtx).roomCtx.curRoom.num as s32);
        }
    } else if (*this).effects[0 as libc::c_int as usize].alpha as libc::c_int
                  == 0 as libc::c_int {
        Actor_Spawn(&mut (*globalCtx).actorCtx, globalCtx,
                    ACTOR_DOOR_WARP1 as libc::c_int as s16, -50.0f32, 0.0f32,
                    0.0f32, 0 as libc::c_int as s16, 0 as libc::c_int as s16,
                    0 as libc::c_int as s16,
                    WARP_DUNGEON_ADULT as libc::c_int as s16);
        Actor_Spawn(&mut (*globalCtx).actorCtx, globalCtx,
                    ACTOR_ITEM_B_HEART as libc::c_int as s16,
                    Math_SinS((*this).actor.shape.rot.y) * 200.0f32 +
                        -50.0f32, 0.0f32,
                    Math_CosS((*this).actor.shape.rot.y) * 200.0f32 + 0.0f32,
                    0 as libc::c_int as s16, 0 as libc::c_int as s16,
                    0 as libc::c_int as s16, 0 as libc::c_int as s16);
        BossSst_SetCameraTargets(1.0f32, 7 as libc::c_int);
        (*this).effectMode = BONGO_NULL as libc::c_int as u8_0
    } else if (*this).timer as libc::c_int == 0 as libc::c_int {
        (*this).effects[0 as libc::c_int as usize].status =
            0 as libc::c_int as s16;
        (*this).effects[1 as libc::c_int as usize].status =
            -(1 as libc::c_int) as s16;
        (*this).effects[2 as libc::c_int as usize].status =
            -(1 as libc::c_int) as s16
    } else if (*this).timer as libc::c_int > 0 as libc::c_int {
        (*this).effects[0 as libc::c_int as usize].status =
            ((*this).effects[0 as libc::c_int as usize].status as libc::c_int
                 + 5 as libc::c_int) as s16;
        BossSst_UpdateDeathCamera(this, globalCtx);
    }
    colorIndigo.a = (*this).effects[0 as libc::c_int as usize].alpha;
    colorDarkIndigo.a = (*this).effects[0 as libc::c_int as usize].alpha;
    i = 0 as libc::c_int;
    while i < 5 as libc::c_int {
        spawnPos.x = sRoomCenter.x + 0.0f32 + Rand_CenteredFloat(800.0f32);
        spawnPos.y = sRoomCenter.y + -28.0f32 + Rand_ZeroOne() * 5.0f32;
        spawnPos.z = sRoomCenter.z + 0.0f32 + Rand_CenteredFloat(800.0f32);
        EffectSsGSplash_Spawn(globalCtx, &mut spawnPos, &mut colorIndigo,
                              &mut colorDarkIndigo, 0 as libc::c_int as s16,
                              0x3e8 as libc::c_int as s16);
        i += 1
    };
}
#[no_mangle]
pub unsafe extern "C" fn BossSst_HandSetupWait(mut this: *mut BossSst) {
    sHandState[(*this).actor.params as usize] = HAND_WAIT as libc::c_int;
    (*this).colliderJntSph.base.atFlags =
        ((*this).colliderJntSph.base.atFlags as libc::c_int &
             !((1 as libc::c_int) << 0 as libc::c_int |
                   (1 as libc::c_int) << 1 as libc::c_int)) as u8_0;
    Animation_MorphToLoop(&mut (*this).skelAnime,
                          sHandIdleAnims[(*this).actor.params as usize],
                          5.0f32);
    (*this).ready = 0 as libc::c_int as s8;
    (*this).timer = 20 as libc::c_int as s16;
    (*this).actionFunc =
        Some(BossSst_HandWait as
                 unsafe extern "C" fn(_: *mut BossSst, _: *mut GlobalContext)
                     -> ());
}
#[no_mangle]
pub unsafe extern "C" fn BossSst_HandWait(mut this: *mut BossSst,
                                          mut globalCtx: *mut GlobalContext) {
    SkelAnime_Update(&mut (*this).skelAnime);
    Math_StepToF(&mut (*this).actor.world.pos.y, (*this).actor.floorHeight,
                 20.0f32);
    Math_StepToF(&mut (*this).actor.world.pos.x, (*this).actor.home.pos.x,
                 1.0f32);
    Math_StepToF(&mut (*this).actor.world.pos.z, (*this).actor.home.pos.z,
                 1.0f32);
    if sHandState[(*((*this).actor.child as *mut BossSst)).actor.params as
                      usize] == HAND_DAMAGED as libc::c_int {
        let mut player: *mut Player =
            (*globalCtx).actorCtx.actorLists[ACTORCAT_PLAYER as libc::c_int as
                                                 usize].head as *mut Player;
        if (*this).timer as libc::c_int != 0 as libc::c_int {
            (*this).timer -= 1
        }
        if (*this).timer as libc::c_int == 0 as libc::c_int &&
               (*player).actor.world.pos.y > -50.0f32 &&
               (*player).stateFlags1 & 0x6080 as libc::c_int as libc::c_uint
                   == 0 {
            BossSst_HandSelectAttack(this);
        }
    } else if (*sHead).actionFunc ==
                  Some(BossSst_HeadNeutral as
                           unsafe extern "C" fn(_: *mut BossSst,
                                                _: *mut GlobalContext) -> ())
     {
        if (*this).actor.params as libc::c_int ==
               BONGO_RIGHT_HAND as libc::c_int &&
               (*sHead).timer as libc::c_int % 28 as libc::c_int ==
                   12 as libc::c_int {
            BossSst_HandSetupDownbeat(this);
        } else if (*this).actor.params as libc::c_int ==
                      BONGO_LEFT_HAND as libc::c_int &&
                      (*sHead).timer as libc::c_int % 7 as libc::c_int ==
                          5 as libc::c_int &&
                      ((*sHead).timer as libc::c_int) < 112 as libc::c_int {
            BossSst_HandSetupOffbeat(this);
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn BossSst_HandSetupDownbeat(mut this: *mut BossSst) {
    sHandState[(*this).actor.params as usize] = HAND_BEAT as libc::c_int;
    Animation_MorphToPlayOnce(&mut (*this).skelAnime,
                              sHandOpenPoses[(*this).actor.params as usize],
                              5.0f32);
    (*this).actor.shape.rot.x = 0 as libc::c_int as s16;
    (*this).timer = 12 as libc::c_int as s16;
    (*this).actionFunc =
        Some(BossSst_HandDownbeat as
                 unsafe extern "C" fn(_: *mut BossSst, _: *mut GlobalContext)
                     -> ());
}
#[no_mangle]
pub unsafe extern "C" fn BossSst_HandDownbeat(mut this: *mut BossSst,
                                              mut globalCtx:
                                                  *mut GlobalContext) {
    SkelAnime_Update(&mut (*this).skelAnime);
    if sHandState[(*((*this).actor.child as *mut BossSst)).actor.params as
                      usize] == HAND_DAMAGED as libc::c_int {
        BossSst_HandSetupWait(this);
    } else {
        if (*this).timer as libc::c_int != 0 as libc::c_int {
            (*this).timer -= 1
        }
        if (*this).timer as libc::c_int >= 3 as libc::c_int {
            (*this).actor.shape.rot.x =
                ((*this).actor.shape.rot.x as libc::c_int -
                     0x100 as libc::c_int) as s16;
            Math_StepToF(&mut (*this).actor.world.pos.y, 0.0f32 + 180.0f32,
                         20.0f32);
        } else {
            (*this).actor.shape.rot.x =
                ((*this).actor.shape.rot.x as libc::c_int +
                     0x300 as libc::c_int) as s16;
            Math_StepToF(&mut (*this).actor.world.pos.y, 0.0f32 + 0.0f32,
                         60.0f32);
        }
        if (*this).timer as libc::c_int == 0 as libc::c_int {
            (*sFloor).dyna.actor.params =
                BONGOFLOOR_HIT as libc::c_int as s16;
            if (*sHead).actionFunc ==
                   Some(BossSst_HeadWait as
                            unsafe extern "C" fn(_: *mut BossSst,
                                                 _: *mut GlobalContext) -> ())
               {
                if (*this).ready != 0 {
                    BossSst_HandSelectAttack(this);
                } else { BossSst_HandSetupWait(this); }
            } else { BossSst_HandSetupDownbeatEnd(this); }
            func_800AA000((*this).actor.xyzDistToPlayerSq,
                          0xff as libc::c_int as u8_0,
                          0x14 as libc::c_int as u8_0,
                          0x96 as libc::c_int as u8_0);
            Audio_PlayActorSound2(&mut (*this).actor,
                                  0x3951 as libc::c_int as u16_0);
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn BossSst_HandSetupDownbeatEnd(mut this:
                                                          *mut BossSst) {
    (*sFloor).dyna.actor.params = BONGOFLOOR_HIT as libc::c_int as s16;
    Animation_PlayOnce(&mut (*this).skelAnime,
                       sHandFlatPoses[(*this).actor.params as usize]);
    (*this).actionFunc =
        Some(BossSst_HandDownbeatEnd as
                 unsafe extern "C" fn(_: *mut BossSst, _: *mut GlobalContext)
                     -> ());
}
#[no_mangle]
pub unsafe extern "C" fn BossSst_HandDownbeatEnd(mut this: *mut BossSst,
                                                 mut globalCtx:
                                                     *mut GlobalContext) {
    SkelAnime_Update(&mut (*this).skelAnime);
    if sHandState[(*((*this).actor.child as *mut BossSst)).actor.params as
                      usize] == HAND_DAMAGED as libc::c_int {
        BossSst_HandSetupWait(this);
    } else {
        Math_SmoothStepToF(&mut (*this).actor.world.pos.y, 0.0f32 + 40.0f32,
                           0.5f32, 20.0f32, 3.0f32);
        Math_ScaledStepToS(&mut (*this).actor.shape.rot.x,
                           -(0x800 as libc::c_int) as s16,
                           0x100 as libc::c_int as s16);
        Math_StepToF(&mut (*this).actor.world.pos.x, (*this).actor.home.pos.x,
                     1.0f32);
        Math_StepToF(&mut (*this).actor.world.pos.z, (*this).actor.home.pos.z,
                     1.0f32);
        if (*sHead).actionFunc !=
               Some(BossSst_HeadIntro as
                        unsafe extern "C" fn(_: *mut BossSst,
                                             _: *mut GlobalContext) -> ()) &&
               (*sHead).timer as libc::c_int % 28 as libc::c_int ==
                   12 as libc::c_int {
            BossSst_HandSetupDownbeat(this);
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn BossSst_HandSetupOffbeat(mut this: *mut BossSst) {
    sHandState[(*this).actor.params as usize] = HAND_BEAT as libc::c_int;
    Animation_MorphToPlayOnce(&mut (*this).skelAnime,
                              sHandOpenPoses[(*this).actor.params as usize],
                              5.0f32);
    (*this).actor.shape.rot.x = 0 as libc::c_int as s16;
    (*this).timer = 5 as libc::c_int as s16;
    (*this).actionFunc =
        Some(BossSst_HandOffbeat as
                 unsafe extern "C" fn(_: *mut BossSst, _: *mut GlobalContext)
                     -> ());
}
#[no_mangle]
pub unsafe extern "C" fn BossSst_HandOffbeat(mut this: *mut BossSst,
                                             mut globalCtx:
                                                 *mut GlobalContext) {
    SkelAnime_Update(&mut (*this).skelAnime);
    if sHandState[(*((*this).actor.child as *mut BossSst)).actor.params as
                      usize] == HAND_DAMAGED as libc::c_int {
        BossSst_HandSetupWait(this);
    } else {
        if (*this).timer as libc::c_int != 0 as libc::c_int {
            (*this).timer -= 1
        }
        if (*this).timer as libc::c_int != 0 as libc::c_int {
            (*this).actor.shape.rot.x =
                ((*this).actor.shape.rot.x as libc::c_int -
                     0x140 as libc::c_int) as s16;
            Math_StepToF(&mut (*this).actor.world.pos.y, 0.0f32 + 60.0f32,
                         15.0f32);
        } else {
            (*this).actor.shape.rot.x =
                ((*this).actor.shape.rot.x as libc::c_int +
                     0x500 as libc::c_int) as s16;
            Math_StepToF(&mut (*this).actor.world.pos.y, 0.0f32 + 0.0f32,
                         60.0f32);
        }
        if (*this).timer as libc::c_int == 0 as libc::c_int {
            Audio_PlayActorSound2(&mut (*this).actor,
                                  0x3950 as libc::c_int as u16_0);
            BossSst_HandSetupOffbeatEnd(this);
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn BossSst_HandSetupOffbeatEnd(mut this: *mut BossSst) {
    Animation_PlayOnce(&mut (*this).skelAnime,
                       sHandFlatPoses[(*this).actor.params as usize]);
    (*this).actionFunc =
        Some(BossSst_HandOffbeatEnd as
                 unsafe extern "C" fn(_: *mut BossSst, _: *mut GlobalContext)
                     -> ());
}
#[no_mangle]
pub unsafe extern "C" fn BossSst_HandOffbeatEnd(mut this: *mut BossSst,
                                                mut globalCtx:
                                                    *mut GlobalContext) {
    SkelAnime_Update(&mut (*this).skelAnime);
    if sHandState[(*((*this).actor.child as *mut BossSst)).actor.params as
                      usize] == HAND_DAMAGED as libc::c_int {
        BossSst_HandSetupWait(this);
    } else {
        Math_SmoothStepToF(&mut (*this).actor.world.pos.y, 0.0f32 + 40.0f32,
                           0.5f32, 20.0f32, 3.0f32);
        Math_ScaledStepToS(&mut (*this).actor.shape.rot.x,
                           -(0x400 as libc::c_int) as s16,
                           0xa0 as libc::c_int as s16);
        Math_StepToF(&mut (*this).actor.world.pos.x, (*this).actor.home.pos.x,
                     1.0f32);
        Math_StepToF(&mut (*this).actor.world.pos.z, (*this).actor.home.pos.z,
                     1.0f32);
        if (*sHead).actionFunc ==
               Some(BossSst_HeadWait as
                        unsafe extern "C" fn(_: *mut BossSst,
                                             _: *mut GlobalContext) -> ()) {
            if (*this).ready != 0 {
                BossSst_HandSelectAttack(this);
            } else { BossSst_HandSetupWait(this); }
        } else if (*sHead).actionFunc !=
                      Some(BossSst_HeadIntro as
                               unsafe extern "C" fn(_: *mut BossSst,
                                                    _: *mut GlobalContext)
                                   -> ()) &&
                      (*sHead).timer as libc::c_int % 7 as libc::c_int ==
                          5 as libc::c_int &&
                      (*sHead).timer as libc::c_int % 28 as libc::c_int !=
                          5 as libc::c_int {
            BossSst_HandSetupOffbeat(this);
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn BossSst_HandSetupEndSlam(mut this: *mut BossSst) {
    sHandState[(*this).actor.params as usize] = HAND_RETREAT as libc::c_int;
    (*this).colliderJntSph.base.atFlags =
        ((*this).colliderJntSph.base.atFlags as libc::c_int &
             !((1 as libc::c_int) << 0 as libc::c_int |
                   (1 as libc::c_int) << 1 as libc::c_int)) as u8_0;
    Animation_MorphToPlayOnce(&mut (*this).skelAnime,
                              sHandPushoffPoses[(*this).actor.params as
                                                    usize], 6.0f32);
    (*this).actionFunc =
        Some(BossSst_HandEndSlam as
                 unsafe extern "C" fn(_: *mut BossSst, _: *mut GlobalContext)
                     -> ());
}
#[no_mangle]
pub unsafe extern "C" fn BossSst_HandEndSlam(mut this: *mut BossSst,
                                             mut globalCtx:
                                                 *mut GlobalContext) {
    if SkelAnime_Update(&mut (*this).skelAnime) != 0 {
        BossSst_HandSetupRetreat(this);
    };
}
#[no_mangle]
pub unsafe extern "C" fn BossSst_HandSetupRetreat(mut this: *mut BossSst) {
    sHandState[(*this).actor.params as usize] = HAND_RETREAT as libc::c_int;
    Animation_MorphToPlayOnce(&mut (*this).skelAnime,
                              sHandHangPoses[(*this).actor.params as usize],
                              10.0f32);
    (*this).colliderJntSph.base.atFlags =
        ((*this).colliderJntSph.base.atFlags as libc::c_int &
             !((1 as libc::c_int) << 0 as libc::c_int |
                   (1 as libc::c_int) << 1 as libc::c_int)) as u8_0;
    (*this).colliderJntSph.base.acFlags =
        ((*this).colliderJntSph.base.acFlags as libc::c_int |
             (1 as libc::c_int) << 0 as libc::c_int) as u8_0;
    (*this).actor.flags |=
        ((1 as libc::c_int) << 0 as libc::c_int) as libc::c_uint;
    BossSst_HandSetInvulnerable(this, 0 as libc::c_int);
    (*this).timer = 0 as libc::c_int as s16;
    (*this).actionFunc =
        Some(BossSst_HandRetreat as
                 unsafe extern "C" fn(_: *mut BossSst, _: *mut GlobalContext)
                     -> ());
    (*this).actor.speedXZ = 3.0f32;
}
#[no_mangle]
pub unsafe extern "C" fn BossSst_HandRetreat(mut this: *mut BossSst,
                                             mut globalCtx:
                                                 *mut GlobalContext) {
    let mut diff: f32_0 = 0.;
    let mut inPosition: s32 = 0;
    SkelAnime_Update(&mut (*this).skelAnime);
    (*this).actor.speedXZ = (*this).actor.speedXZ * 1.2f32;
    (*this).actor.speedXZ =
        if (*this).actor.speedXZ > 50.0f32 {
            50.0f32
        } else { (*this).actor.speedXZ };
    diff =
        Math_SmoothStepToF(&mut (*this).actor.world.pos.x,
                           (*this).actor.home.pos.x, 0.3f32,
                           (*this).actor.speedXZ, 1.0f32);
    diff +=
        Math_SmoothStepToF(&mut (*this).actor.world.pos.z,
                           (*this).actor.home.pos.z, 0.3f32,
                           (*this).actor.speedXZ, 1.0f32);
    if (*this).timer as libc::c_int != 0 as libc::c_int {
        if (*this).timer as libc::c_int != 0 as libc::c_int {
            (*this).timer -= 1
        }
        (*this).actor.world.pos.y =
            sinf((*this).timer as libc::c_int as libc::c_float *
                     3.14159265358979323846f32 / 16.0f32) * 250.0f32 +
                (*this).actor.home.pos.y;
        if (*this).timer as libc::c_int == 0 as libc::c_int {
            BossSst_HandSetupWait(this);
        } else if (*this).timer as libc::c_int == 4 as libc::c_int {
            Animation_MorphToLoop(&mut (*this).skelAnime,
                                  sHandIdleAnims[(*this).actor.params as
                                                     usize], 4.0f32);
        }
    } else {
        inPosition =
            Math_ScaledStepToS(&mut (*this).actor.shape.rot.y,
                               (*this).actor.home.rot.y,
                               0x200 as libc::c_int as s16);
        inPosition &=
            Math_ScaledStepToS(&mut (*this).actor.shape.rot.z,
                               (*this).actor.home.rot.z,
                               0x200 as libc::c_int as s16);
        inPosition &=
            Math_ScaledStepToS(&mut (*this).handYRotMod,
                               0 as libc::c_int as s16,
                               0x800 as libc::c_int as s16);
        func_8002F974(&mut (*this).actor,
                      (0x3967 as libc::c_int - 0x800 as libc::c_int) as
                          u16_0);
        if Math_SmoothStepToF(&mut (*this).actor.world.pos.y,
                              0.0f32 + 250.0f32, 0.5f32, 70.0f32, 5.0f32) <
               1.0f32 && inPosition != 0 && diff < 10.0f32 {
            (*this).timer = 8 as libc::c_int as s16
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn BossSst_HandSetupReadySlam(mut this: *mut BossSst) {
    sHandState[(*this).actor.params as usize] = HAND_SLAM as libc::c_int;
    (*this).timer = 0 as libc::c_int as s16;
    Animation_MorphToPlayOnce(&mut (*this).skelAnime,
                              sHandOpenPoses[(*this).actor.params as usize],
                              10.0f32);
    (*this).actionFunc =
        Some(BossSst_HandReadySlam as
                 unsafe extern "C" fn(_: *mut BossSst, _: *mut GlobalContext)
                     -> ());
}
#[no_mangle]
pub unsafe extern "C" fn BossSst_HandReadySlam(mut this: *mut BossSst,
                                               mut globalCtx:
                                                   *mut GlobalContext) {
    SkelAnime_Update(&mut (*this).skelAnime);
    if (*this).timer as libc::c_int != 0 as libc::c_int {
        if (*this).timer as libc::c_int != 0 as libc::c_int {
            (*this).timer -= 1
        }
        if (*this).timer as libc::c_int == 0 as libc::c_int {
            BossSst_HandSetupSlam(this);
        }
    } else {
        let mut player: *mut Player =
            (*globalCtx).actorCtx.actorLists[ACTORCAT_PLAYER as libc::c_int as
                                                 usize].head as *mut Player;
        if Math_StepToF(&mut (*this).actor.world.pos.y, 0.0f32 + 300.0f32,
                        30.0f32) != 0 &&
               (*this).actor.xzDistToPlayer < 140.0f32 {
            (*this).timer = 20 as libc::c_int as s16
        }
        Math_ScaledStepToS(&mut (*this).actor.shape.rot.x,
                           -(0x1000 as libc::c_int) as s16,
                           0x100 as libc::c_int as s16);
        Math_ApproachF(&mut (*this).actor.world.pos.x,
                       (*player).actor.world.pos.x, 0.5f32, 40.0f32);
        Math_ApproachF(&mut (*this).actor.world.pos.z,
                       (*player).actor.world.pos.z, 0.5f32, 40.0f32);
        func_8002F974(&mut (*this).actor,
                      (0x3967 as libc::c_int - 0x800 as libc::c_int) as
                          u16_0);
    };
}
#[no_mangle]
pub unsafe extern "C" fn BossSst_HandSetupSlam(mut this: *mut BossSst) {
    sHandState[(*this).actor.params as usize] = HAND_SLAM as libc::c_int;
    (*this).actor.velocity.y = 1.0f32;
    Animation_MorphToPlayOnce(&mut (*this).skelAnime,
                              sHandFlatPoses[(*this).actor.params as usize],
                              10.0f32);
    BossSst_HandSetDamage(this, 0x20 as libc::c_int);
    (*this).ready = 0 as libc::c_int as s8;
    Audio_PlayActorSound2(&mut (*this).actor, 0x3953 as libc::c_int as u16_0);
    (*this).actionFunc =
        Some(BossSst_HandSlam as
                 unsafe extern "C" fn(_: *mut BossSst, _: *mut GlobalContext)
                     -> ());
}
#[no_mangle]
pub unsafe extern "C" fn BossSst_HandSlam(mut this: *mut BossSst,
                                          mut globalCtx: *mut GlobalContext) {
    SkelAnime_Update(&mut (*this).skelAnime);
    Math_StepToS(&mut (*this).handZPosMod, -(0xdac as libc::c_int) as s16,
                 0x1f4 as libc::c_int as s16);
    Math_ScaledStepToS(&mut (*this).actor.shape.rot.x,
                       0 as libc::c_int as s16, 0x1000 as libc::c_int as s16);
    Math_ScaledStepToS(&mut (*this).handYRotMod, 0 as libc::c_int as s16,
                       0x1000 as libc::c_int as s16);
    if (*this).timer as libc::c_int != 0 as libc::c_int {
        if (*this).timer as libc::c_int != 0 as libc::c_int {
            (*this).timer -= 1
        }
        if (*this).timer as libc::c_int == 0 as libc::c_int {
            if (*this).colliderJntSph.base.acFlags as libc::c_int &
                   (1 as libc::c_int) << 0 as libc::c_int != 0 {
                BossSst_HandSetupEndSlam(this);
            } else {
                (*this).colliderJntSph.base.acFlags =
                    ((*this).colliderJntSph.base.acFlags as libc::c_int |
                         (1 as libc::c_int) << 0 as libc::c_int) as u8_0;
                BossSst_HandSetupWait(this);
            }
        }
    } else {
        if (*this).ready != 0 {
            (*this).timer = 30 as libc::c_int as s16;
            (*this).colliderJntSph.base.atFlags =
                ((*this).colliderJntSph.base.atFlags as libc::c_int &
                     !((1 as libc::c_int) << 0 as libc::c_int |
                           (1 as libc::c_int) << 1 as libc::c_int)) as u8_0
        } else {
            (*this).actor.velocity.y *= 1.5f32;
            if Math_StepToF(&mut (*this).actor.world.pos.y,
                            (*this).actor.floorHeight,
                            (*this).actor.velocity.y) != 0 {
                (*this).ready = 1 as libc::c_int as s8;
                Audio_PlayActorSound2(&mut (*this).actor,
                                      0x3950 as libc::c_int as u16_0);
                BossSst_SpawnShockwave(this);
                (*this).colliderCyl.base.atFlags =
                    ((*this).colliderCyl.base.atFlags as libc::c_int |
                         (1 as libc::c_int) << 0 as libc::c_int) as u8_0;
                Collider_UpdateCylinder(&mut (*this).actor,
                                        &mut (*this).colliderCyl);
                (*this).colliderCyl.dim.radius = sCylinderInitHand.dim.radius
            }
        }
        if (*this).colliderJntSph.base.atFlags as libc::c_int &
               (1 as libc::c_int) << 1 as libc::c_int != 0 {
            let mut player: *mut Player =
                (*globalCtx).actorCtx.actorLists[ACTORCAT_PLAYER as
                                                     libc::c_int as
                                                     usize].head as
                    *mut Player;
            (*player).actor.world.pos.x =
                Math_SinS((*this).actor.yawTowardsPlayer) * 100.0f32 +
                    (*this).actor.world.pos.x;
            (*player).actor.world.pos.z =
                Math_CosS((*this).actor.yawTowardsPlayer) * 100.0f32 +
                    (*this).actor.world.pos.z;
            (*this).colliderJntSph.base.atFlags =
                ((*this).colliderJntSph.base.atFlags as libc::c_int &
                     !((1 as libc::c_int) << 0 as libc::c_int |
                           (1 as libc::c_int) << 1 as libc::c_int)) as u8_0;
            func_8002F71C(globalCtx, &mut (*this).actor, 5.0f32,
                          (*this).actor.yawTowardsPlayer, 0.0f32);
        }
        Math_ScaledStepToS(&mut (*this).actor.shape.rot.x,
                           0 as libc::c_int as s16,
                           0x200 as libc::c_int as s16);
    };
}
#[no_mangle]
pub unsafe extern "C" fn BossSst_HandSetupReadySweep(mut this: *mut BossSst) {
    sHandState[(*this).actor.params as usize] = HAND_SWEEP as libc::c_int;
    Animation_MorphToPlayOnce(&mut (*this).skelAnime,
                              sHandOpenPoses[(*this).actor.params as usize],
                              10.0f32);
    (*this).radius =
        Actor_WorldDistXZToPoint(&mut (*this).actor,
                                 &mut (*sHead).actor.world.pos);
    (*this).actor.world.rot.y =
        Actor_WorldYawTowardPoint(&mut (*sHead).actor,
                                  &mut (*this).actor.world.pos);
    (*this).targetYaw =
        ((*this).actor.home.rot.y as libc::c_int +
             (*this).actionVar as libc::c_int * 0x2000 as libc::c_int) as s16;
    (*this).actionFunc =
        Some(BossSst_HandReadySweep as
                 unsafe extern "C" fn(_: *mut BossSst, _: *mut GlobalContext)
                     -> ());
}
#[no_mangle]
pub unsafe extern "C" fn BossSst_HandReadySweep(mut this: *mut BossSst,
                                                mut globalCtx:
                                                    *mut GlobalContext) {
    let mut inPosition: s32 = 0;
    SkelAnime_Update(&mut (*this).skelAnime);
    inPosition =
        Math_StepToF(&mut (*this).actor.world.pos.y, 0.0f32 + 50.0f32,
                     4.0f32);
    inPosition &=
        Math_ScaledStepToS(&mut (*this).actor.shape.rot.y, (*this).targetYaw,
                           0x200 as libc::c_int as s16);
    inPosition &=
        Math_ScaledStepToS(&mut (*this).actor.world.rot.y, (*this).targetYaw,
                           0x400 as libc::c_int as s16);
    inPosition &=
        (Math_SmoothStepToF(&mut (*this).radius,
                            (*sHead).actor.xzDistToPlayer, 0.5f32, 60.0f32,
                            1.0f32) < 10.0f32) as libc::c_int;
    (*this).actor.world.pos.x =
        Math_SinS((*this).actor.world.rot.y) * (*this).radius +
            (*sHead).actor.world.pos.x;
    (*this).actor.world.pos.z =
        Math_CosS((*this).actor.world.rot.y) * (*this).radius +
            (*sHead).actor.world.pos.z;
    if inPosition != 0 {
        BossSst_HandSetupSweep(this);
    } else {
        func_8002F974(&mut (*this).actor,
                      (0x3967 as libc::c_int - 0x800 as libc::c_int) as
                          u16_0);
    };
}
#[no_mangle]
pub unsafe extern "C" fn BossSst_HandSetupSweep(mut this: *mut BossSst) {
    Animation_MorphToPlayOnce(&mut (*this).skelAnime,
                              sHandFlatPoses[(*this).actor.params as usize],
                              5.0f32);
    BossSst_HandSetDamage(this, 0x10 as libc::c_int);
    (*this).targetYaw =
        ((*this).actor.home.rot.y as libc::c_int -
             (*this).actionVar as libc::c_int * 0x2000 as libc::c_int) as s16;
    (*this).handMaxSpeed = 0x300 as libc::c_int as s16;
    (*this).handAngSpeed = 0 as libc::c_int as s16;
    (*this).ready = 0 as libc::c_int as s8;
    Audio_PlayActorSound2(&mut (*this).actor, 0x3953 as libc::c_int as u16_0);
    (*this).actionFunc =
        Some(BossSst_HandSweep as
                 unsafe extern "C" fn(_: *mut BossSst, _: *mut GlobalContext)
                     -> ());
}
#[no_mangle]
pub unsafe extern "C" fn BossSst_HandSweep(mut this: *mut BossSst,
                                           mut globalCtx:
                                               *mut GlobalContext) {
    let mut player: *mut Player =
        (*globalCtx).actorCtx.actorLists[ACTORCAT_PLAYER as libc::c_int as
                                             usize].head as *mut Player;
    let mut newTargetYaw: s16 = 0;
    SkelAnime_Update(&mut (*this).skelAnime);
    (*this).handAngSpeed =
        ((*this).handAngSpeed as libc::c_int + 0x60 as libc::c_int) as s16;
    (*this).handAngSpeed =
        if (*this).handAngSpeed as libc::c_int >
               (*this).handMaxSpeed as libc::c_int {
            (*this).handMaxSpeed as libc::c_int
        } else { (*this).handAngSpeed as libc::c_int } as s16;
    if Math_SmoothStepToS(&mut (*this).actor.shape.rot.y, (*this).targetYaw,
                          4 as libc::c_int as s16, (*this).handAngSpeed,
                          0x10 as libc::c_int as s16) == 0 {
        (*this).colliderJntSph.base.ocFlags1 =
            ((*this).colliderJntSph.base.ocFlags1 as libc::c_int &
                 !((1 as libc::c_int) << 2 as libc::c_int)) as u8_0;
        BossSst_HandSetupRetreat(this);
    } else if (*this).colliderJntSph.base.atFlags as libc::c_int &
                  (1 as libc::c_int) << 1 as libc::c_int != 0 {
        (*this).colliderJntSph.base.atFlags =
            ((*this).colliderJntSph.base.atFlags as libc::c_int &
                 !((1 as libc::c_int) << 0 as libc::c_int |
                       (1 as libc::c_int) << 1 as libc::c_int)) as u8_0;
        (*this).ready = 1 as libc::c_int as s8;
        func_8002F71C(globalCtx, &mut (*this).actor, 5.0f32,
                      ((*this).actor.shape.rot.y as libc::c_int -
                           (*this).actionVar as libc::c_int *
                               0x3800 as libc::c_int) as s16, 0.0f32);
        func_8002F7DC(&mut (*player).actor, 0x83e as libc::c_int as u16_0);
        newTargetYaw =
            ((*this).actor.shape.rot.y as libc::c_int -
                 (*this).actionVar as libc::c_int * 0x1400 as libc::c_int) as
                s16;
        if (newTargetYaw as libc::c_int - (*this).targetYaw as libc::c_int) as
               s16 as libc::c_int * (*this).actionVar as libc::c_int >
               0 as libc::c_int {
            (*this).targetYaw = newTargetYaw
        }
    }
    if (*this).ready == 0 &&
           ((*player).cylinder.dim.height as libc::c_int as libc::c_float >
                40.0f32 || (*player).actor.world.pos.y > 1.0f32) {
        (*this).colliderJntSph.base.atFlags =
            ((*this).colliderJntSph.base.atFlags as libc::c_int |
                 (1 as libc::c_int) << 0 as libc::c_int) as u8_0;
        (*this).colliderJntSph.base.ocFlags1 =
            ((*this).colliderJntSph.base.ocFlags1 as libc::c_int &
                 !((1 as libc::c_int) << 2 as libc::c_int)) as u8_0
    } else {
        (*this).colliderJntSph.base.atFlags =
            ((*this).colliderJntSph.base.atFlags as libc::c_int &
                 !((1 as libc::c_int) << 0 as libc::c_int)) as u8_0;
        (*this).colliderJntSph.base.ocFlags1 =
            ((*this).colliderJntSph.base.ocFlags1 as libc::c_int |
                 (1 as libc::c_int) << 2 as libc::c_int) as u8_0
    }
    (*this).actor.world.pos.x =
        Math_SinS((*this).actor.shape.rot.y) * (*this).radius +
            (*sHead).actor.world.pos.x;
    (*this).actor.world.pos.z =
        Math_CosS((*this).actor.shape.rot.y) * (*this).radius +
            (*sHead).actor.world.pos.z;
}
#[no_mangle]
pub unsafe extern "C" fn BossSst_HandSetupReadyPunch(mut this: *mut BossSst) {
    sHandState[(*this).actor.params as usize] = HAND_PUNCH as libc::c_int;
    Animation_MorphToPlayOnce(&mut (*this).skelAnime,
                              sHandPushoffPoses[(*this).actor.params as
                                                    usize], 10.0f32);
    (*this).actionFunc =
        Some(BossSst_HandReadyPunch as
                 unsafe extern "C" fn(_: *mut BossSst, _: *mut GlobalContext)
                     -> ());
}
#[no_mangle]
pub unsafe extern "C" fn BossSst_HandReadyPunch(mut this: *mut BossSst,
                                                mut globalCtx:
                                                    *mut GlobalContext) {
    let mut inPosition: s32 =
        Math_ScaledStepToS(&mut (*this).actor.shape.rot.y,
                           (*this).actor.yawTowardsPlayer,
                           0x400 as libc::c_int as s16);
    if SkelAnime_Update(&mut (*this).skelAnime) != 0 && inPosition != 0 {
        BossSst_HandSetupPunch(this);
    };
}
#[no_mangle]
pub unsafe extern "C" fn BossSst_HandSetupPunch(mut this: *mut BossSst) {
    (*this).actor.speedXZ = 0.5f32;
    Animation_MorphToPlayOnce(&mut (*this).skelAnime,
                              sHandFistPoses[(*this).actor.params as usize],
                              5.0f32);
    BossSst_HandSetInvulnerable(this, 1 as libc::c_int);
    (*this).targetRoll =
        ((*this).actionVar as libc::c_int * 0x3f00 as libc::c_int) as s16;
    BossSst_HandSetDamage(this, 0x10 as libc::c_int);
    (*this).actionFunc =
        Some(BossSst_HandPunch as
                 unsafe extern "C" fn(_: *mut BossSst, _: *mut GlobalContext)
                     -> ());
}
#[no_mangle]
pub unsafe extern "C" fn BossSst_HandPunch(mut this: *mut BossSst,
                                           mut globalCtx:
                                               *mut GlobalContext) {
    SkelAnime_Update(&mut (*this).skelAnime);
    Math_StepToF(&mut (*this).actor.world.pos.y, 0.0f32 + 80.0f32, 20.0f32);
    if Math_ScaledStepToS(&mut (*this).actor.shape.rot.z, (*this).targetRoll,
                          0x400 as libc::c_int as s16) != 0 {
        (*this).targetRoll =
            ((*this).targetRoll as libc::c_int * -(1 as libc::c_int)) as s16
    }
    (*this).actor.speedXZ *= 1.25f32;
    (*this).actor.speedXZ =
        if (*this).actor.speedXZ > 50.0f32 {
            50.0f32
        } else { (*this).actor.speedXZ };
    (*this).actor.world.pos.x +=
        (*this).actor.speedXZ * Math_SinS((*this).actor.shape.rot.y);
    (*this).actor.world.pos.z +=
        (*this).actor.speedXZ * Math_CosS((*this).actor.shape.rot.y);
    if (*this).actor.bgCheckFlags as libc::c_int & 8 as libc::c_int != 0 {
        BossSst_HandSetupRetreat(this);
    } else if (*this).colliderJntSph.base.atFlags as libc::c_int &
                  (1 as libc::c_int) << 1 as libc::c_int != 0 {
        func_8002F7DC(&mut (*((*(*globalCtx).actorCtx.actorLists.as_mut_ptr().offset(ACTORCAT_PLAYER
                                                                                         as
                                                                                         libc::c_int
                                                                                         as
                                                                                         isize)).head
                                  as *mut Player)).actor,
                      0x83e as libc::c_int as u16_0);
        func_8002F71C(globalCtx, &mut (*this).actor, 10.0f32,
                      (*this).actor.shape.rot.y, 5.0f32);
        BossSst_HandSetupRetreat(this);
    }
    func_8002F974(&mut (*this).actor,
                  (0x3967 as libc::c_int - 0x800 as libc::c_int) as u16_0);
}
#[no_mangle]
pub unsafe extern "C" fn BossSst_HandSetupReadyClap(mut this: *mut BossSst) {
    sHandState[(*this).actor.params as usize] = HAND_CLAP as libc::c_int;
    if sHandState[(*((*this).actor.child as *mut BossSst)).actor.params as
                      usize] != HAND_CLAP as libc::c_int {
        BossSst_HandSetupReadyClap((*this).actor.child as *mut BossSst);
    }
    Animation_MorphToPlayOnce(&mut (*this).skelAnime,
                              sHandOpenPoses[(*this).actor.params as usize],
                              10.0f32);
    (*this).radius =
        Actor_WorldDistXZToPoint(&mut (*this).actor,
                                 &mut (*sHead).actor.world.pos);
    (*this).actor.world.rot.y =
        Actor_WorldYawTowardPoint(&mut (*sHead).actor,
                                  &mut (*this).actor.world.pos);
    (*this).targetYaw =
        ((*this).actor.home.rot.y as libc::c_int -
             (*this).actionVar as libc::c_int * 0x1800 as libc::c_int) as s16;
    (*this).targetRoll =
        ((*this).actionVar as libc::c_int * 0x4000 as libc::c_int) as s16;
    (*this).timer = 0 as libc::c_int as s16;
    (*this).ready = 0 as libc::c_int as s8;
    (*((*this).actor.child as *mut BossSst)).ready = 0 as libc::c_int as s8;
    (*this).actionFunc =
        Some(BossSst_HandReadyClap as
                 unsafe extern "C" fn(_: *mut BossSst, _: *mut GlobalContext)
                     -> ());
}
#[no_mangle]
pub unsafe extern "C" fn BossSst_HandReadyClap(mut this: *mut BossSst,
                                               mut globalCtx:
                                                   *mut GlobalContext) {
    if (*this).timer as libc::c_int != 0 as libc::c_int {
        if (*this).timer as libc::c_int != 0 as libc::c_int {
            (*this).timer -= 1
        }
        if (*this).timer as libc::c_int == 0 as libc::c_int {
            BossSst_HandSetupClap(this);
            BossSst_HandSetupClap((*this).actor.child as *mut BossSst);
            (*((*this).actor.child as *mut BossSst)).radius = (*this).radius
        }
    } else if (*this).ready == 0 {
        (*this).ready = SkelAnime_Update(&mut (*this).skelAnime) as s8;
        (*this).ready =
            ((*this).ready as libc::c_int &
                 Math_ScaledStepToS(&mut (*this).actor.shape.rot.x,
                                    0 as libc::c_int as s16,
                                    0x600 as libc::c_int as s16)) as s8;
        (*this).ready =
            ((*this).ready as libc::c_int &
                 Math_ScaledStepToS(&mut (*this).actor.shape.rot.z,
                                    (*this).targetRoll,
                                    0x600 as libc::c_int as s16)) as s8;
        (*this).ready =
            ((*this).ready as libc::c_int &
                 Math_ScaledStepToS(&mut (*this).actor.shape.rot.y,
                                    (*this).targetYaw,
                                    0x200 as libc::c_int as s16)) as s8;
        (*this).ready =
            ((*this).ready as libc::c_int &
                 Math_ScaledStepToS(&mut (*this).actor.world.rot.y,
                                    (*this).targetYaw,
                                    0x400 as libc::c_int as s16)) as s8;
        (*this).ready =
            ((*this).ready as libc::c_int &
                 (Math_SmoothStepToF(&mut (*this).radius,
                                     (*sHead).actor.xzDistToPlayer, 0.5f32,
                                     50.0f32, 1.0f32) < 10.0f32) as
                     libc::c_int) as s8;
        (*this).ready =
            ((*this).ready as libc::c_int &
                 (Math_SmoothStepToF(&mut (*this).actor.world.pos.y,
                                     0.0f32 + 95.0f32, 0.5f32, 30.0f32,
                                     1.0f32) < 1.0f32) as libc::c_int) as s8;
        (*this).actor.world.pos.x =
            Math_SinS((*this).actor.world.rot.y) * (*this).radius +
                (*sHead).actor.world.pos.x;
        (*this).actor.world.pos.z =
            Math_CosS((*this).actor.world.rot.y) * (*this).radius +
                (*sHead).actor.world.pos.z
    } else if (*((*this).actor.child as *mut BossSst)).ready != 0 {
        (*this).timer = 20 as libc::c_int as s16
    };
}
#[no_mangle]
pub unsafe extern "C" fn BossSst_HandSetupClap(mut this: *mut BossSst) {
    Animation_MorphToPlayOnce(&mut (*this).skelAnime,
                              sHandFlatPoses[(*this).actor.params as usize],
                              3.0f32);
    (*this).timer = 0 as libc::c_int as s16;
    (*this).handMaxSpeed = 0x240 as libc::c_int as s16;
    (*this).handAngSpeed = 0 as libc::c_int as s16;
    (*this).ready = 0 as libc::c_int as s8;
    BossSst_HandSetDamage(this, 0x20 as libc::c_int);
    (*this).actionFunc =
        Some(BossSst_HandClap as
                 unsafe extern "C" fn(_: *mut BossSst, _: *mut GlobalContext)
                     -> ());
}
#[no_mangle]
pub unsafe extern "C" fn BossSst_HandClap(mut this: *mut BossSst,
                                          mut globalCtx: *mut GlobalContext) {
    static mut dropFlag: s32 = 0 as libc::c_int;
    let mut player: *mut Player =
        (*globalCtx).actorCtx.actorLists[ACTORCAT_PLAYER as libc::c_int as
                                             usize].head as *mut Player;
    SkelAnime_Update(&mut (*this).skelAnime);
    if (*this).timer as libc::c_int != 0 as libc::c_int {
        if (*this).timer as libc::c_int != 0 as libc::c_int {
            (*this).timer -= 1
        }
        if (*this).timer as libc::c_int == 0 as libc::c_int {
            if dropFlag != 0 {
                Item_DropCollectible(globalCtx, &mut (*this).actor.world.pos,
                                     if Rand_ZeroOne() < 0.5f32 {
                                         ITEM00_ARROWS_SMALL as libc::c_int
                                     } else {
                                         ITEM00_MAGIC_SMALL as libc::c_int
                                     } as s16);
                dropFlag = 0 as libc::c_int
            }
            BossSst_HandReleasePlayer(this, globalCtx, 1 as libc::c_int);
            BossSst_HandSetupEndClap(this);
        }
    } else {
        if (*this).colliderJntSph.base.atFlags as libc::c_int &
               (1 as libc::c_int) << 1 as libc::c_int != 0 {
            (*this).colliderJntSph.base.atFlags =
                ((*this).colliderJntSph.base.atFlags as libc::c_int &
                     !((1 as libc::c_int) << 0 as libc::c_int |
                           (1 as libc::c_int) << 1 as libc::c_int)) as u8_0;
            let ref mut fresh5 =
                (*((*this).actor.child as
                       *mut BossSst)).colliderJntSph.base.atFlags;
            *fresh5 =
                (*fresh5 as libc::c_int &
                     !((1 as libc::c_int) << 0 as libc::c_int |
                           (1 as libc::c_int) << 1 as libc::c_int)) as u8_0;
            BossSst_HandGrabPlayer(this, globalCtx);
        }
        if (*this).ready != 0 {
            (*this).timer = 30 as libc::c_int as s16;
            (*this).colliderJntSph.base.atFlags =
                ((*this).colliderJntSph.base.atFlags as libc::c_int &
                     !((1 as libc::c_int) << 0 as libc::c_int |
                           (1 as libc::c_int) << 1 as libc::c_int)) as u8_0;
            if (*player).stateFlags2 & 0x80 as libc::c_int as libc::c_uint ==
                   0 {
                dropFlag = 1 as libc::c_int
            }
        } else {
            (*this).handAngSpeed =
                ((*this).handAngSpeed as libc::c_int + 0x40 as libc::c_int) as
                    s16;
            (*this).handAngSpeed =
                if (*this).handAngSpeed as libc::c_int >
                       (*this).handMaxSpeed as libc::c_int {
                    (*this).handMaxSpeed as libc::c_int
                } else { (*this).handAngSpeed as libc::c_int } as s16;
            if Math_ScaledStepToS(&mut (*this).actor.shape.rot.y,
                                  (*this).actor.home.rot.y,
                                  (*this).handAngSpeed) != 0 {
                if (*this).actor.params as libc::c_int ==
                       BONGO_LEFT_HAND as libc::c_int {
                    Audio_PlayActorSound2(&mut (*this).actor,
                                          0x3952 as libc::c_int as u16_0);
                }
                (*this).ready = 1 as libc::c_int as s8
            } else {
                func_8002F974(&mut (*this).actor,
                              (0x3967 as libc::c_int - 0x800 as libc::c_int)
                                  as u16_0);
            }
            (*this).actor.world.pos.x =
                Math_SinS((*this).actor.shape.rot.y) * (*this).radius +
                    (*sHead).actor.world.pos.x;
            (*this).actor.world.pos.z =
                Math_CosS((*this).actor.shape.rot.y) * (*this).radius +
                    (*sHead).actor.world.pos.z
        }
    }
    if (*player).actor.parent == &mut (*this).actor as *mut Actor {
        (*player).unk_850 = 0 as libc::c_int as s16;
        (*player).actor.world.pos = (*this).actor.world.pos
    };
}
#[no_mangle]
pub unsafe extern "C" fn BossSst_HandSetupEndClap(mut this: *mut BossSst) {
    (*this).targetYaw =
        ((*this).actor.home.rot.y as libc::c_int -
             (*this).actionVar as libc::c_int * 0x1000 as libc::c_int) as s16;
    Animation_MorphToPlayOnce(&mut (*this).skelAnime,
                              sHandOpenPoses[(*this).actor.params as usize],
                              10.0f32);
    (*this).colliderJntSph.base.atFlags =
        ((*this).colliderJntSph.base.atFlags as libc::c_int &
             !((1 as libc::c_int) << 0 as libc::c_int |
                   (1 as libc::c_int) << 1 as libc::c_int)) as u8_0;
    (*this).actionFunc =
        Some(BossSst_HandEndClap as
                 unsafe extern "C" fn(_: *mut BossSst, _: *mut GlobalContext)
                     -> ());
}
#[no_mangle]
pub unsafe extern "C" fn BossSst_HandEndClap(mut this: *mut BossSst,
                                             mut globalCtx:
                                                 *mut GlobalContext) {
    SkelAnime_Update(&mut (*this).skelAnime);
    Math_ScaledStepToS(&mut (*this).actor.shape.rot.z,
                       0 as libc::c_int as s16, 0x200 as libc::c_int as s16);
    if Math_ScaledStepToS(&mut (*this).actor.shape.rot.y, (*this).targetYaw,
                          0x100 as libc::c_int as s16) != 0 {
        BossSst_HandSetupRetreat(this);
    }
    (*this).actor.world.pos.x =
        Math_SinS((*this).actor.shape.rot.y) * (*this).radius +
            (*sHead).actor.world.pos.x;
    (*this).actor.world.pos.z =
        Math_CosS((*this).actor.shape.rot.y) * (*this).radius +
            (*sHead).actor.world.pos.z;
}
#[no_mangle]
pub unsafe extern "C" fn BossSst_HandSetupReadyGrab(mut this: *mut BossSst) {
    sHandState[(*this).actor.params as usize] = HAND_GRAB as libc::c_int;
    Animation_MorphToPlayOnce(&mut (*this).skelAnime,
                              sHandOpenPoses[(*this).actor.params as usize],
                              10.0f32);
    (*this).targetYaw =
        ((*this).actionVar as libc::c_int * -(0x5000 as libc::c_int)) as s16;
    (*this).targetRoll =
        ((*this).actionVar as libc::c_int * 0x4000 as libc::c_int) as s16;
    (*this).actionFunc =
        Some(BossSst_HandReadyGrab as
                 unsafe extern "C" fn(_: *mut BossSst, _: *mut GlobalContext)
                     -> ());
}
#[no_mangle]
pub unsafe extern "C" fn BossSst_HandReadyGrab(mut this: *mut BossSst,
                                               mut globalCtx:
                                                   *mut GlobalContext) {
    let mut inPosition: s32 = 0;
    SkelAnime_Update(&mut (*this).skelAnime);
    inPosition =
        (Math_SmoothStepToS(&mut (*this).actor.shape.rot.z,
                            (*this).targetRoll, 4 as libc::c_int as s16,
                            0x800 as libc::c_int as s16,
                            0x100 as libc::c_int as s16) as libc::c_int ==
             0 as libc::c_int) as libc::c_int;
    inPosition &=
        Math_ScaledStepToS(&mut (*this).actor.shape.rot.y,
                           ((*this).actor.yawTowardsPlayer as libc::c_int +
                                (*this).targetYaw as libc::c_int) as s16,
                           0xa00 as libc::c_int as s16);
    Math_ApproachF(&mut (*this).actor.world.pos.y, 0.0f32 + 95.0f32, 0.5f32,
                   20.0f32);
    if inPosition != 0 { BossSst_HandSetupGrab(this); };
}
#[no_mangle]
pub unsafe extern "C" fn BossSst_HandSetupGrab(mut this: *mut BossSst) {
    Animation_MorphToPlayOnce(&mut (*this).skelAnime,
                              sHandFistPoses[(*this).actor.params as usize],
                              5.0f32);
    (*this).actor.world.rot.y =
        ((*this).actor.shape.rot.y as libc::c_int +
             (*this).actionVar as libc::c_int * 0x4000 as libc::c_int) as s16;
    (*this).targetYaw = (*this).actor.world.rot.y;
    (*this).timer = 30 as libc::c_int as s16;
    (*this).actor.speedXZ = 0.5f32;
    BossSst_HandSetDamage(this, 0x20 as libc::c_int);
    (*this).actionFunc =
        Some(BossSst_HandGrab as
                 unsafe extern "C" fn(_: *mut BossSst, _: *mut GlobalContext)
                     -> ());
}
#[no_mangle]
pub unsafe extern "C" fn BossSst_HandGrab(mut this: *mut BossSst,
                                          mut globalCtx: *mut GlobalContext) {
    let mut player: *mut Player =
        (*globalCtx).actorCtx.actorLists[ACTORCAT_PLAYER as libc::c_int as
                                             usize].head as *mut Player;
    if (*this).timer as libc::c_int != 0 as libc::c_int { (*this).timer -= 1 }
    (*this).actor.world.rot.y =
        ((1.0f32 -
              sinf((*this).timer as libc::c_int as libc::c_float *
                       (3.14159265358979323846f32 / 60.0f32))) *
             ((*this).actionVar as libc::c_int * 0x2000 as libc::c_int) as
                 libc::c_float +
             (*this).targetYaw as libc::c_int as libc::c_float) as s16;
    (*this).actor.shape.rot.y =
        ((*this).actor.world.rot.y as libc::c_int -
             (*this).actionVar as libc::c_int * 0x4000 as libc::c_int) as s16;
    if ((*this).timer as libc::c_int) < 5 as libc::c_int {
        Math_SmoothStepToF(&mut (*this).actor.speedXZ, 0.0f32, 0.5f32,
                           25.0f32, 5.0f32);
        if SkelAnime_Update(&mut (*this).skelAnime) != 0 {
            (*this).colliderJntSph.base.atFlags =
                ((*this).colliderJntSph.base.atFlags as libc::c_int &
                     !((1 as libc::c_int) << 0 as libc::c_int |
                           (1 as libc::c_int) << 1 as libc::c_int)) as u8_0;
            (*this).actor.speedXZ = 0.0f32;
            if (*player).stateFlags2 & 0x80 as libc::c_int as libc::c_uint !=
                   0 {
                if Rand_ZeroOne() < 0.5f32 {
                    BossSst_HandSetupCrush(this);
                } else { BossSst_HandSetupSwing(this); }
            } else {
                Item_DropCollectible(globalCtx, &mut (*this).actor.world.pos,
                                     if Rand_ZeroOne() < 0.5f32 {
                                         ITEM00_ARROWS_SMALL as libc::c_int
                                     } else {
                                         ITEM00_MAGIC_SMALL as libc::c_int
                                     } as s16);
                BossSst_HandSetupRetreat(this);
            }
        }
    } else {
        (*this).actor.speedXZ *= 1.26f32;
        (*this).actor.speedXZ =
            if (*this).actor.speedXZ > 70.0f32 {
                70.0f32
            } else { (*this).actor.speedXZ };
        func_8002F974(&mut (*this).actor,
                      (0x3967 as libc::c_int - 0x800 as libc::c_int) as
                          u16_0);
    }
    if (*this).colliderJntSph.base.atFlags as libc::c_int &
           (1 as libc::c_int) << 1 as libc::c_int != 0 {
        (*this).colliderJntSph.base.atFlags =
            ((*this).colliderJntSph.base.atFlags as libc::c_int &
                 !((1 as libc::c_int) << 0 as libc::c_int |
                       (1 as libc::c_int) << 1 as libc::c_int)) as u8_0;
        Audio_PlayActorSound2(&mut (*this).actor,
                              0x3965 as libc::c_int as u16_0);
        BossSst_HandGrabPlayer(this, globalCtx);
        (*this).timer =
            if (*this).timer as libc::c_int > 5 as libc::c_int {
                5 as libc::c_int
            } else { (*this).timer as libc::c_int } as s16
    }
    (*this).actor.world.pos.x +=
        (*this).actor.speedXZ * Math_SinS((*this).actor.world.rot.y);
    (*this).actor.world.pos.z +=
        (*this).actor.speedXZ * Math_CosS((*this).actor.world.rot.y);
    if (*player).stateFlags2 & 0x80 as libc::c_int as libc::c_uint != 0 {
        (*player).unk_850 = 0 as libc::c_int as s16;
        (*player).actor.world.pos = (*this).actor.world.pos;
        (*player).actor.shape.rot.y = (*this).actor.shape.rot.y
    };
}
#[no_mangle]
pub unsafe extern "C" fn BossSst_HandSetupCrush(mut this: *mut BossSst) {
    Animation_MorphToLoop(&mut (*this).skelAnime,
                          sHandClenchAnims[(*this).actor.params as usize],
                          -10.0f32);
    (*this).timer = 20 as libc::c_int as s16;
    (*this).actionFunc =
        Some(BossSst_HandCrush as
                 unsafe extern "C" fn(_: *mut BossSst, _: *mut GlobalContext)
                     -> ());
}
#[no_mangle]
pub unsafe extern "C" fn BossSst_HandCrush(mut this: *mut BossSst,
                                           mut globalCtx:
                                               *mut GlobalContext) {
    let mut player: *mut Player =
        (*globalCtx).actorCtx.actorLists[ACTORCAT_PLAYER as libc::c_int as
                                             usize].head as *mut Player;
    SkelAnime_Update(&mut (*this).skelAnime);
    if (*this).timer as libc::c_int != 0 as libc::c_int { (*this).timer -= 1 }
    if (*player).stateFlags2 & 0x80 as libc::c_int as libc::c_uint == 0 {
        BossSst_HandReleasePlayer(this, globalCtx, 1 as libc::c_int);
        BossSst_HandSetupEndCrush(this);
    } else {
        (*player).actor.world.pos = (*this).actor.world.pos;
        if (*this).timer as libc::c_int == 0 as libc::c_int {
            (*this).timer = 20 as libc::c_int as s16;
            if !(gSaveContext.linkAge == 0 as libc::c_int) {
                func_8002F7DC(&mut (*player).actor,
                              0x6825 as libc::c_int as u16_0);
            } else {
                func_8002F7DC(&mut (*player).actor,
                              0x6805 as libc::c_int as u16_0);
            }
            (*globalCtx).damagePlayer.expect("non-null function pointer")(globalCtx,
                                                                          -(8
                                                                                as
                                                                                libc::c_int));
        }
        if Animation_OnFrame(&mut (*this).skelAnime, 0.0f32) != 0 {
            Audio_PlayActorSound2(&mut (*this).actor,
                                  0x3965 as libc::c_int as u16_0);
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn BossSst_HandSetupEndCrush(mut this: *mut BossSst) {
    Animation_MorphToPlayOnce(&mut (*this).skelAnime,
                              sHandFlatPoses[(*this).actor.params as usize],
                              10.0f32);
    (*this).actionFunc =
        Some(BossSst_HandEndCrush as
                 unsafe extern "C" fn(_: *mut BossSst, _: *mut GlobalContext)
                     -> ());
}
#[no_mangle]
pub unsafe extern "C" fn BossSst_HandEndCrush(mut this: *mut BossSst,
                                              mut globalCtx:
                                                  *mut GlobalContext) {
    if SkelAnime_Update(&mut (*this).skelAnime) != 0 {
        BossSst_HandSetupRetreat(this);
    };
}
#[no_mangle]
pub unsafe extern "C" fn BossSst_HandSetupSwing(mut this: *mut BossSst) {
    (*this).amplitude = -(0x4000 as libc::c_int) as s16;
    (*this).timer = 1 as libc::c_int as s16;
    (*this).center.x =
        (*this).actor.world.pos.x -
            Math_SinS((*this).actor.shape.rot.y) * 200.0f32;
    (*this).center.y = (*this).actor.world.pos.y;
    (*this).center.z =
        (*this).actor.world.pos.z -
            Math_CosS((*this).actor.shape.rot.y) * 200.0f32;
    (*this).actionFunc =
        Some(BossSst_HandSwing as
                 unsafe extern "C" fn(_: *mut BossSst, _: *mut GlobalContext)
                     -> ());
}
#[no_mangle]
pub unsafe extern "C" fn BossSst_HandSwing(mut this: *mut BossSst,
                                           mut globalCtx:
                                               *mut GlobalContext) {
    let mut player: *mut Player =
        (*globalCtx).actorCtx.actorLists[ACTORCAT_PLAYER as libc::c_int as
                                             usize].head as *mut Player;
    let mut offXZ: f32_0 = 0.;
    if Math_ScaledStepToS(&mut (*this).actor.shape.rot.x, (*this).amplitude,
                          ((*this).timer as libc::c_int * 0xe4 as libc::c_int
                               + 0x1c8 as libc::c_int) as s16) != 0 {
        if (*this).amplitude as libc::c_int != 0 as libc::c_int {
            (*this).amplitude = 0 as libc::c_int as s16;
            if (*this).timer as libc::c_int == 4 as libc::c_int {
                Animation_MorphToPlayOnce(&mut (*this).skelAnime,
                                          sHandFlatPoses[(*this).actor.params
                                                             as usize],
                                          4.0f32);
            }
        } else {
            if (*this).timer as libc::c_int == 4 as libc::c_int {
                (*player).actor.shape.rot.x = 0 as libc::c_int as s16;
                (*player).actor.shape.rot.z = 0 as libc::c_int as s16;
                BossSst_HandSetupRetreat(this);
                return
            }
            (*this).amplitude =
                if (*this).timer as libc::c_int == 3 as libc::c_int {
                    -(0x6000 as libc::c_int)
                } else { -(0x4000 as libc::c_int) } as s16;
            (*this).timer += 1
        }
    }
    (*this).actor.world.pos.y =
        Math_CosS(((*this).actor.shape.rot.x as libc::c_int +
                       0x4000 as libc::c_int) as s16) * 200.0f32 +
            (*this).center.y;
    offXZ =
        Math_SinS(((*this).actor.shape.rot.x as libc::c_int +
                       0x4000 as libc::c_int) as s16) * 200.0f32;
    (*this).actor.world.pos.x =
        Math_SinS((*this).actor.shape.rot.y) * offXZ + (*this).center.x;
    (*this).actor.world.pos.z =
        Math_CosS((*this).actor.shape.rot.y) * offXZ + (*this).center.z;
    if (*this).timer as libc::c_int != 4 as libc::c_int {
        (*this).actor.shape.rot.z =
            (((*this).actor.shape.rot.x as libc::c_int +
                  0x4000 as libc::c_int) * (*this).actionVar as libc::c_int)
                as s16
    } else {
        Math_ScaledStepToS(&mut (*this).actor.shape.rot.z,
                           0 as libc::c_int as s16,
                           0x800 as libc::c_int as s16);
    }
    if (*player).stateFlags2 & 0x80 as libc::c_int as libc::c_uint != 0 {
        (*player).unk_850 = 0 as libc::c_int as s16;
        Math_Vec3f_Copy(&mut (*player).actor.world.pos,
                        &mut (*this).actor.world.pos);
        (*player).actor.shape.rot.x = (*this).actor.shape.rot.x;
        (*player).actor.shape.rot.z =
            ((*this).actionVar as libc::c_int * -(0x4000 as libc::c_int) +
                 (*this).actor.shape.rot.z as libc::c_int) as s16
    } else {
        Math_ScaledStepToS(&mut (*player).actor.shape.rot.x,
                           0 as libc::c_int as s16,
                           0x600 as libc::c_int as s16);
        Math_ScaledStepToS(&mut (*player).actor.shape.rot.z,
                           0 as libc::c_int as s16,
                           0x600 as libc::c_int as s16);
        (*player).actor.world.pos.x +=
            20.0f32 * Math_SinS((*this).actor.shape.rot.y);
        (*player).actor.world.pos.z +=
            20.0f32 * Math_CosS((*this).actor.shape.rot.y)
    }
    if (*this).timer as libc::c_int == 4 as libc::c_int &&
           (*this).amplitude as libc::c_int == 0 as libc::c_int &&
           SkelAnime_Update(&mut (*this).skelAnime) != 0 &&
           (*player).stateFlags2 & 0x80 as libc::c_int as libc::c_uint != 0 {
        BossSst_HandReleasePlayer(this, globalCtx, 0 as libc::c_int);
        (*player).actor.world.pos.x +=
            70.0f32 * Math_SinS((*this).actor.shape.rot.y);
        (*player).actor.world.pos.z +=
            70.0f32 * Math_CosS((*this).actor.shape.rot.y);
        func_8002F71C(globalCtx, &mut (*this).actor, 15.0f32,
                      (*this).actor.shape.rot.y, 2.0f32);
        func_8002F7DC(&mut (*player).actor, 0x83e as libc::c_int as u16_0);
    }
    func_8002F974(&mut (*this).actor,
                  (0x3967 as libc::c_int - 0x800 as libc::c_int) as u16_0);
}
#[no_mangle]
pub unsafe extern "C" fn BossSst_HandSetupReel(mut this: *mut BossSst) {
    sHandState[(*this).actor.params as usize] = HAND_DAMAGED as libc::c_int;
    Animation_MorphToPlayOnce(&mut (*this).skelAnime,
                              sHandFlatPoses[(*this).actor.params as usize],
                              4.0f32);
    (*this).timer = 36 as libc::c_int as s16;
    Math_Vec3f_Copy(&mut (*this).center, &mut (*this).actor.world.pos);
    Actor_SetColorFilter(&mut (*this).actor, 0 as libc::c_int as s16,
                         0xff as libc::c_int as s16, 0 as libc::c_int as s16,
                         200 as libc::c_int as s16);
    (*this).actionFunc =
        Some(BossSst_HandReel as
                 unsafe extern "C" fn(_: *mut BossSst, _: *mut GlobalContext)
                     -> ());
}
#[no_mangle]
pub unsafe extern "C" fn BossSst_HandReel(mut this: *mut BossSst,
                                          mut globalCtx: *mut GlobalContext) {
    SkelAnime_Update(&mut (*this).skelAnime);
    if (*this).timer as libc::c_int != 0 as libc::c_int { (*this).timer -= 1 }
    if (*this).timer as libc::c_int % 4 as libc::c_int == 0 {
        if (*this).timer as libc::c_int % 8 as libc::c_int != 0 {
            Animation_MorphToPlayOnce(&mut (*this).skelAnime,
                                      sHandFlatPoses[(*this).actor.params as
                                                         usize], 4.0f32);
        } else {
            Animation_MorphToPlayOnce(&mut (*this).skelAnime,
                                      sHandFistPoses[(*this).actor.params as
                                                         usize], 6.0f32);
        }
    }
    (*this).actor.colorFilterTimer = 200 as libc::c_int as u8_0;
    (*this).actor.world.pos.x += Rand_CenteredFloat(20.0f32);
    (*this).actor.world.pos.y += Rand_CenteredFloat(20.0f32);
    (*this).actor.world.pos.z += Rand_CenteredFloat(20.0f32);
    if (*this).actor.world.pos.y < (*this).actor.floorHeight + 100.0f32 {
        Math_StepToF(&mut (*this).actor.world.pos.y,
                     (*this).actor.floorHeight + 100.0f32, 20.0f32);
    }
    if (*this).timer as libc::c_int == 0 as libc::c_int {
        BossSst_HandSetupReadyShake(this);
    };
}
#[no_mangle]
pub unsafe extern "C" fn BossSst_HandSetupReadyShake(mut this: *mut BossSst) {
    Animation_MorphToPlayOnce(&mut (*this).skelAnime,
                              sHandDamagePoses[(*this).actor.params as usize],
                              8.0f32);
    (*this).actionFunc =
        Some(BossSst_HandReadyShake as
                 unsafe extern "C" fn(_: *mut BossSst, _: *mut GlobalContext)
                     -> ());
}
#[no_mangle]
pub unsafe extern "C" fn BossSst_HandReadyShake(mut this: *mut BossSst,
                                                mut globalCtx:
                                                    *mut GlobalContext) {
    let mut diff: f32_0 = 0.;
    let mut inPosition: s32 = 0;
    diff =
        Math_SmoothStepToF(&mut (*this).actor.world.pos.x,
                           (*this).actor.home.pos.x, 0.5f32, 25.0f32, 1.0f32);
    diff +=
        Math_SmoothStepToF(&mut (*this).actor.world.pos.z,
                           (*this).actor.home.pos.z, 0.5f32, 25.0f32, 1.0f32);
    diff +=
        Math_SmoothStepToF(&mut (*this).actor.world.pos.y,
                           (*this).actor.home.pos.y + 200.0f32, 0.2f32,
                           30.0f32, 1.0f32);
    inPosition =
        Math_ScaledStepToS(&mut (*this).actor.shape.rot.x,
                           0x4000 as libc::c_int as s16,
                           0x400 as libc::c_int as s16);
    inPosition &=
        Math_ScaledStepToS(&mut (*this).actor.shape.rot.z,
                           0 as libc::c_int as s16,
                           0x1000 as libc::c_int as s16);
    inPosition &=
        Math_ScaledStepToS(&mut (*this).actor.shape.rot.y,
                           (*this).actor.home.rot.y,
                           0x800 as libc::c_int as s16);
    inPosition &=
        Math_StepToS(&mut (*this).handZPosMod, -(0x5dc as libc::c_int) as s16,
                     0x1f4 as libc::c_int as s16);
    inPosition &=
        Math_ScaledStepToS(&mut (*this).handYRotMod,
                           ((*this).actionVar as libc::c_int *
                                -(0x2000 as libc::c_int)) as s16,
                           0x800 as libc::c_int as s16);
    (*this).actor.colorFilterTimer = 200 as libc::c_int as u8_0;
    if diff < 30.0f32 && inPosition != 0 {
        BossSst_HandSetupShake(this);
    } else {
        func_8002F974(&mut (*this).actor,
                      (0x3967 as libc::c_int - 0x800 as libc::c_int) as
                          u16_0);
    };
}
#[no_mangle]
pub unsafe extern "C" fn BossSst_HandSetupShake(mut this: *mut BossSst) {
    (*this).timer = 200 as libc::c_int as s16;
    (*this).actionFunc =
        Some(BossSst_HandShake as
                 unsafe extern "C" fn(_: *mut BossSst, _: *mut GlobalContext)
                     -> ());
}
#[no_mangle]
pub unsafe extern "C" fn BossSst_HandShake(mut this: *mut BossSst,
                                           mut globalCtx:
                                               *mut GlobalContext) {
    SkelAnime_Update(&mut (*this).skelAnime);
    if (*this).timer as libc::c_int != 0 as libc::c_int { (*this).timer -= 1 }
    (*this).actor.shape.rot.x =
        (0x4000 as libc::c_int as libc::c_float +
             sinf((*this).timer as libc::c_int as libc::c_float *
                      (3.14159265358979323846f32 /
                           5 as libc::c_int as libc::c_float)) *
                 0x2000 as libc::c_int as libc::c_float) as s16;
    (*this).handYRotMod =
        (((*this).actionVar as libc::c_int * -(0x2000 as libc::c_int)) as
             libc::c_float +
             sinf((*this).timer as libc::c_int as libc::c_float *
                      (3.14159265358979323846f32 /
                           4 as libc::c_int as libc::c_float)) *
                 0x2800 as libc::c_int as libc::c_float) as s16;
    if (*this).timer as libc::c_int % 8 as libc::c_int == 0 {
        Audio_PlayActorSound2(&mut (*this).actor,
                              0x3968 as libc::c_int as u16_0);
    }
    if sHandState[(*((*this).actor.child as *mut BossSst)).actor.params as
                      usize] == HAND_DAMAGED as libc::c_int {
        if (*((*this).actor.child as *mut BossSst)).actionFunc ==
               Some(BossSst_HandShake as
                        unsafe extern "C" fn(_: *mut BossSst,
                                             _: *mut GlobalContext) -> ()) ||
               (*((*this).actor.child as *mut BossSst)).actionFunc ==
                   Some(BossSst_HandReadyCharge as
                            unsafe extern "C" fn(_: *mut BossSst,
                                                 _: *mut GlobalContext) -> ())
           {
            BossSst_HandSetupReadyCharge(this);
        } else if (*this).timer as libc::c_int == 0 as libc::c_int {
            (*this).timer = 80 as libc::c_int as s16
        }
    } else if (*this).timer as libc::c_int == 0 as libc::c_int {
        (*this).actor.flags |=
            ((1 as libc::c_int) << 0 as libc::c_int) as libc::c_uint;
        BossSst_HandSetupSlam(this);
    };
}
#[no_mangle]
pub unsafe extern "C" fn BossSst_HandSetupReadyCharge(mut this:
                                                          *mut BossSst) {
    Animation_MorphToPlayOnce(&mut (*this).skelAnime,
                              sHandFistPoses[(*this).actor.params as usize],
                              10.0f32);
    (*this).ready = 0 as libc::c_int as s8;
    (*this).actionFunc =
        Some(BossSst_HandReadyCharge as
                 unsafe extern "C" fn(_: *mut BossSst, _: *mut GlobalContext)
                     -> ());
}
#[no_mangle]
pub unsafe extern "C" fn BossSst_HandReadyCharge(mut this: *mut BossSst,
                                                 mut globalCtx:
                                                     *mut GlobalContext) {
    if (*this).ready == 0 {
        (*this).ready = SkelAnime_Update(&mut (*this).skelAnime) as s8;
        (*this).ready =
            ((*this).ready as libc::c_int &
                 Math_ScaledStepToS(&mut (*this).actor.shape.rot.x,
                                    0 as libc::c_int as s16,
                                    0x800 as libc::c_int as s16)) as s8;
        (*this).ready =
            ((*this).ready as libc::c_int &
                 Math_ScaledStepToS(&mut (*this).actor.shape.rot.y,
                                    ((*this).actor.home.rot.y as libc::c_int +
                                         (*this).actionVar as libc::c_int *
                                             0x1000 as libc::c_int) as s16,
                                    0x800 as libc::c_int as s16)) as s8;
        (*this).ready =
            ((*this).ready as libc::c_int &
                 Math_ScaledStepToS(&mut (*this).handYRotMod,
                                    0 as libc::c_int as s16,
                                    0x800 as libc::c_int as s16)) as s8;
        (*this).ready =
            ((*this).ready as libc::c_int &
                 Math_ScaledStepToS(&mut (*this).actor.shape.rot.z,
                                    ((*this).actionVar as libc::c_int *
                                         0x2800 as libc::c_int) as s16,
                                    0x800 as libc::c_int as s16)) as s8;
        (*this).ready =
            ((*this).ready as libc::c_int &
                 Math_StepToS(&mut (*this).handZPosMod,
                              -(0xdac as libc::c_int) as s16,
                              0x1f4 as libc::c_int as s16)) as s8;
        if (*this).ready != 0 {
            (*this).actor.colorFilterTimer = 0 as libc::c_int as u8_0
        }
    } else if (*this).colliderJntSph.base.atFlags as libc::c_int &
                  (1 as libc::c_int) << 1 as libc::c_int != 0 {
        (*this).colliderJntSph.base.atFlags =
            ((*this).colliderJntSph.base.atFlags as libc::c_int &
                 !((1 as libc::c_int) << 0 as libc::c_int |
                       (1 as libc::c_int) << 1 as libc::c_int)) as u8_0;
        let ref mut fresh6 =
            (*((*this).actor.child as
                   *mut BossSst)).colliderJntSph.base.atFlags;
        *fresh6 =
            (*fresh6 as libc::c_int &
                 !((1 as libc::c_int) << 0 as libc::c_int |
                       (1 as libc::c_int) << 1 as libc::c_int)) as u8_0;
        (*sHead).colliderJntSph.base.atFlags =
            ((*sHead).colliderJntSph.base.atFlags as libc::c_int &
                 !((1 as libc::c_int) << 0 as libc::c_int |
                       (1 as libc::c_int) << 1 as libc::c_int)) as u8_0;
        func_8002F71C(globalCtx, &mut (*this).actor, 10.0f32,
                      (*this).actor.shape.rot.y, 5.0f32);
        func_8002F7DC(&mut (*((*(*globalCtx).actorCtx.actorLists.as_mut_ptr().offset(ACTORCAT_PLAYER
                                                                                         as
                                                                                         libc::c_int
                                                                                         as
                                                                                         isize)).head
                                  as *mut Player)).actor,
                      0x83e as libc::c_int as u16_0);
    };
}
#[no_mangle]
pub unsafe extern "C" fn BossSst_HandSetupStunned(mut hand: *mut BossSst) {
    Animation_MorphToPlayOnce(&mut (*hand).skelAnime,
                              sHandIdleAnims[(*hand).actor.params as usize],
                              10.0f32);
    if (*hand).actionFunc !=
           Some(BossSst_HandDamage as
                    unsafe extern "C" fn(_: *mut BossSst,
                                         _: *mut GlobalContext) -> ()) {
        (*hand).ready = 0 as libc::c_int as s8
    }
    (*hand).colliderJntSph.base.atFlags =
        ((*hand).colliderJntSph.base.atFlags as libc::c_int &
             !((1 as libc::c_int) << 0 as libc::c_int |
                   (1 as libc::c_int) << 1 as libc::c_int)) as u8_0;
    (*hand).colliderJntSph.base.acFlags =
        ((*hand).colliderJntSph.base.acFlags as libc::c_int |
             (1 as libc::c_int) << 0 as libc::c_int) as u8_0;
    BossSst_HandSetInvulnerable(hand, 1 as libc::c_int);
    Actor_SetColorFilter(&mut (*hand).actor, 0 as libc::c_int as s16,
                         0xff as libc::c_int as s16, 0 as libc::c_int as s16,
                         Animation_GetLastFrame(&mut gBongoHeadKnockoutAnim as
                                                    *mut AnimationHeader as
                                                    *mut libc::c_void));
    (*hand).actionFunc =
        Some(BossSst_HandStunned as
                 unsafe extern "C" fn(_: *mut BossSst, _: *mut GlobalContext)
                     -> ());
}
#[no_mangle]
pub unsafe extern "C" fn BossSst_HandStunned(mut this: *mut BossSst,
                                             mut globalCtx:
                                                 *mut GlobalContext) {
    SkelAnime_Update(&mut (*this).skelAnime);
    Math_ApproachF(&mut (*this).actor.world.pos.z,
                   Math_CosS((*sHead).actor.shape.rot.y) * 200.0f32 +
                       (*this).actor.home.pos.z, 0.5f32, 25.0f32);
    Math_ApproachF(&mut (*this).actor.world.pos.x,
                   Math_SinS((*sHead).actor.shape.rot.y) * 200.0f32 +
                       (*this).actor.home.pos.x, 0.5f32, 25.0f32);
    if (*this).ready == 0 {
        Math_ScaledStepToS(&mut (*this).handYRotMod, 0 as libc::c_int as s16,
                           0x800 as libc::c_int as s16);
        Math_StepToS(&mut (*this).handZPosMod, -(0xdac as libc::c_int) as s16,
                     0x1f4 as libc::c_int as s16);
        Math_ScaledStepToS(&mut (*this).actor.shape.rot.x,
                           (*this).actor.home.rot.x,
                           0x800 as libc::c_int as s16);
        Math_ScaledStepToS(&mut (*this).actor.shape.rot.z,
                           (*this).actor.home.rot.z,
                           0x800 as libc::c_int as s16);
        Math_ScaledStepToS(&mut (*this).actor.shape.rot.y,
                           (*this).actor.home.rot.y,
                           0x800 as libc::c_int as s16);
        if (*sHead).actionFunc ==
               Some(BossSst_HeadVulnerable as
                        unsafe extern "C" fn(_: *mut BossSst,
                                             _: *mut GlobalContext) -> ()) {
            (*this).ready = 1 as libc::c_int as s8;
            Animation_MorphToPlayOnce(&mut (*this).skelAnime,
                                      sHandDamagePoses[(*this).actor.params as
                                                           usize], 10.0f32);
        }
    } else {
        Math_StepToF(&mut (*this).actor.world.pos.y,
                     (*this).actor.floorHeight, 30.0f32);
    };
}
#[no_mangle]
pub unsafe extern "C" fn BossSst_HandSetupDamage(mut hand: *mut BossSst) {
    (*hand).actor.shape.rot.x = 0 as libc::c_int as s16;
    Animation_MorphToPlayOnce(&mut (*hand).skelAnime,
                              sHandOpenPoses[(*hand).actor.params as usize],
                              3.0f32);
    (*hand).timer = 6 as libc::c_int as s16;
    (*hand).actionFunc =
        Some(BossSst_HandDamage as
                 unsafe extern "C" fn(_: *mut BossSst, _: *mut GlobalContext)
                     -> ());
}
#[no_mangle]
pub unsafe extern "C" fn BossSst_HandDamage(mut this: *mut BossSst,
                                            mut globalCtx:
                                                *mut GlobalContext) {
    if (*this).timer as libc::c_int != 0 as libc::c_int { (*this).timer -= 1 }
    SkelAnime_Update(&mut (*this).skelAnime);
    if (*this).timer as libc::c_int >= 2 as libc::c_int {
        (*this).actor.shape.rot.x =
            ((*this).actor.shape.rot.x as libc::c_int - 0x200 as libc::c_int)
                as s16;
        Math_StepToF(&mut (*this).actor.world.pos.y,
                     (*this).actor.floorHeight + 200.0f32, 50.0f32);
    } else {
        (*this).actor.shape.rot.x =
            ((*this).actor.shape.rot.x as libc::c_int + 0x400 as libc::c_int)
                as s16;
        Math_StepToF(&mut (*this).actor.world.pos.y,
                     (*this).actor.floorHeight, 100.0f32);
    }
    if (*this).timer as libc::c_int == 0 as libc::c_int {
        if (*this).actor.floorHeight >= 0.0f32 {
            Audio_PlayActorSound2(&mut (*this).actor,
                                  0x3951 as libc::c_int as u16_0);
        }
        BossSst_HandSetupStunned(this);
    };
}
#[no_mangle]
pub unsafe extern "C" fn BossSst_HandSetupThrash(mut this: *mut BossSst) {
    sHandState[(*this).actor.params as usize] = HAND_DEATH as libc::c_int;
    Animation_MorphToPlayOnce(&mut (*this).skelAnime,
                              sHandOpenPoses[(*this).actor.params as usize],
                              2.0f32);
    (*this).actor.shape.rot.x = 0 as libc::c_int as s16;
    (*this).timer = 160 as libc::c_int as s16;
    if (*this).actor.params as libc::c_int == BONGO_LEFT_HAND as libc::c_int {
        (*this).amplitude = -(0x800 as libc::c_int) as s16
    } else {
        (*this).amplitude = 0 as libc::c_int as s16;
        (*this).actor.shape.rot.x = -(0x800 as libc::c_int) as s16
    }
    (*this).handAngSpeed = 0x180 as libc::c_int as s16;
    (*this).actionFunc =
        Some(BossSst_HandThrash as
                 unsafe extern "C" fn(_: *mut BossSst, _: *mut GlobalContext)
                     -> ());
}
#[no_mangle]
pub unsafe extern "C" fn BossSst_HandThrash(mut this: *mut BossSst,
                                            mut globalCtx:
                                                *mut GlobalContext) {
    if (*this).timer as libc::c_int != 0 as libc::c_int { (*this).timer -= 1 }
    SkelAnime_Update(&mut (*this).skelAnime);
    Math_ApproachF(&mut (*this).actor.world.pos.z,
                   Math_CosS((*sHead).actor.shape.rot.y) * 200.0f32 +
                       (*this).actor.home.pos.z, 0.5f32, 25.0f32);
    Math_ApproachF(&mut (*this).actor.world.pos.x,
                   Math_SinS((*sHead).actor.shape.rot.y) * 200.0f32 +
                       (*this).actor.home.pos.x, 0.5f32, 25.0f32);
    if Math_ScaledStepToS(&mut (*this).actor.shape.rot.x, (*this).amplitude,
                          (*this).handAngSpeed) != 0 {
        if (*this).amplitude as libc::c_int != 0 as libc::c_int {
            (*this).amplitude = 0 as libc::c_int as s16;
            Animation_MorphToPlayOnce(&mut (*this).skelAnime,
                                      sHandFlatPoses[(*this).actor.params as
                                                         usize], 5.0f32);
        } else {
            Audio_PlayActorSound2(&mut (*this).actor,
                                  0x3951 as libc::c_int as u16_0);
            (*this).amplitude = -(0x800 as libc::c_int) as s16;
            Animation_MorphToPlayOnce(&mut (*this).skelAnime,
                                      sHandOpenPoses[(*this).actor.params as
                                                         usize], 5.0f32);
        }
        if ((*this).timer as libc::c_int as libc::c_float) < 80.0f32 {
            (*this).handAngSpeed =
                ((*this).handAngSpeed as libc::c_int - 0x40 as libc::c_int) as
                    s16;
            (*this).handAngSpeed =
                if ((*this).handAngSpeed as libc::c_int) < 0x40 as libc::c_int
                   {
                    0x40 as libc::c_int
                } else { (*this).handAngSpeed as libc::c_int } as s16
        }
    }
    (*this).actor.world.pos.y =
        ((*this).handAngSpeed as libc::c_int as libc::c_float / 256.0f32 +
             0.5f32) * 150.0f32 *
            (-1.0f32 / 0x800 as libc::c_int as libc::c_float) *
            (*this).actor.shape.rot.x as libc::c_int as libc::c_float;
    if (*this).timer as libc::c_int == 0 as libc::c_int {
        BossSst_HandSetupDarken(this);
    };
}
#[no_mangle]
pub unsafe extern "C" fn BossSst_HandSetupDarken(mut this: *mut BossSst) {
    Animation_MorphToPlayOnce(&mut (*this).skelAnime,
                              sHandFlatPoses[(*this).actor.params as usize],
                              5.0f32);
    (*this).actionFunc =
        Some(BossSst_HandDarken as
                 unsafe extern "C" fn(_: *mut BossSst, _: *mut GlobalContext)
                     -> ());
}
#[no_mangle]
pub unsafe extern "C" fn BossSst_HandDarken(mut this: *mut BossSst,
                                            mut globalCtx:
                                                *mut GlobalContext) {
    SkelAnime_Update(&mut (*this).skelAnime);
    Math_ScaledStepToS(&mut (*this).actor.shape.rot.x,
                       -(0x800 as libc::c_int) as s16, (*this).handAngSpeed);
    Math_StepToF(&mut (*this).actor.world.pos.y, 0.0f32 + 90.0f32, 5.0f32);
    if (*sHead).actionFunc ==
           Some(BossSst_HeadFall as
                    unsafe extern "C" fn(_: *mut BossSst,
                                         _: *mut GlobalContext) -> ()) {
        BossSst_HandSetupFall(this);
    };
}
#[no_mangle]
pub unsafe extern "C" fn BossSst_HandSetupFall(mut this: *mut BossSst) {
    Animation_MorphToPlayOnce(&mut (*this).skelAnime,
                              sHandFlatPoses[(*this).actor.params as usize],
                              3.0f32);
    (*this).actionFunc =
        Some(BossSst_HandFall as
                 unsafe extern "C" fn(_: *mut BossSst, _: *mut GlobalContext)
                     -> ());
}
#[no_mangle]
pub unsafe extern "C" fn BossSst_HandFall(mut this: *mut BossSst,
                                          mut globalCtx: *mut GlobalContext) {
    SkelAnime_Update(&mut (*this).skelAnime);
    Math_ScaledStepToS(&mut (*this).actor.shape.rot.x,
                       0 as libc::c_int as s16, 0x400 as libc::c_int as s16);
    (*this).actor.world.pos.y = (*sHead).actor.world.pos.y + 230.0f32;
    if (*sHead).actionFunc ==
           Some(BossSst_HeadMelt as
                    unsafe extern "C" fn(_: *mut BossSst,
                                         _: *mut GlobalContext) -> ()) {
        BossSst_HandSetupMelt(this);
    };
}
#[no_mangle]
pub unsafe extern "C" fn BossSst_HandSetupMelt(mut this: *mut BossSst) {
    BossSst_SpawnHandShadow(this);
    (*this).actor.shape.shadowDraw = None;
    (*this).timer = 80 as libc::c_int as s16;
    (*this).actionFunc =
        Some(BossSst_HandMelt as
                 unsafe extern "C" fn(_: *mut BossSst, _: *mut GlobalContext)
                     -> ());
}
#[no_mangle]
pub unsafe extern "C" fn BossSst_HandMelt(mut this: *mut BossSst,
                                          mut globalCtx: *mut GlobalContext) {
    if (*this).timer as libc::c_int != 0 as libc::c_int { (*this).timer -= 1 }
    (*this).actor.scale.y -= 0.00025f32;
    (*this).actor.scale.x += 0.000025f32;
    (*this).actor.scale.z += 0.000025f32;
    (*this).actor.world.pos.y = 0.0f32 + 0.0f32;
    if (*this).timer as libc::c_int == 0 as libc::c_int {
        BossSst_HandSetupFinish(this);
    };
}
#[no_mangle]
pub unsafe extern "C" fn BossSst_HandSetupFinish(mut this: *mut BossSst) {
    (*this).actor.draw =
        Some(BossSst_DrawEffect as
                 unsafe extern "C" fn(_: *mut Actor, _: *mut GlobalContext)
                     -> ());
    (*this).timer = 20 as libc::c_int as s16;
    (*this).effects[0 as libc::c_int as usize].status =
        0 as libc::c_int as s16;
    (*this).actionFunc =
        Some(BossSst_HandFinish as
                 unsafe extern "C" fn(_: *mut BossSst, _: *mut GlobalContext)
                     -> ());
}
#[no_mangle]
pub unsafe extern "C" fn BossSst_HandFinish(mut this: *mut BossSst,
                                            mut globalCtx:
                                                *mut GlobalContext) {
    if (*this).timer as libc::c_int != 0 as libc::c_int { (*this).timer -= 1 }
    if (*this).timer as libc::c_int == 0 as libc::c_int {
        (*this).effectMode = BONGO_NULL as libc::c_int as u8_0
    };
}
#[no_mangle]
pub unsafe extern "C" fn BossSst_HandSetupRecover(mut this: *mut BossSst) {
    Animation_MorphToPlayOnce(&mut (*this).skelAnime,
                              sHandPushoffPoses[(*this).actor.params as
                                                    usize], 10.0f32);
    (*this).ready = 0 as libc::c_int as s8;
    (*this).actionFunc =
        Some(BossSst_HandRecover as
                 unsafe extern "C" fn(_: *mut BossSst, _: *mut GlobalContext)
                     -> ());
}
#[no_mangle]
pub unsafe extern "C" fn BossSst_HandRecover(mut this: *mut BossSst,
                                             mut globalCtx:
                                                 *mut GlobalContext) {
    Math_SmoothStepToF(&mut (*this).actor.world.pos.y, 0.0f32 + 250.0f32,
                       0.5f32, 70.0f32, 5.0f32);
    if SkelAnime_Update(&mut (*this).skelAnime) != 0 {
        if (*this).ready == 0 {
            Animation_MorphToPlayOnce(&mut (*this).skelAnime,
                                      sHandHangPoses[(*this).actor.params as
                                                         usize], 10.0f32);
            (*this).ready = 1 as libc::c_int as s8
        }
    }
    func_8002F974(&mut (*this).actor,
                  (0x3967 as libc::c_int - 0x800 as libc::c_int) as u16_0);
}
#[no_mangle]
pub unsafe extern "C" fn BossSst_HandSetupFrozen(mut this: *mut BossSst) {
    let mut i: s32 = 0;
    sHandState[(*this).actor.params as usize] = HAND_FROZEN as libc::c_int;
    Math_Vec3f_Copy(&mut (*this).center, &mut (*this).actor.world.pos);
    BossSst_HandSetupReadyBreakIce((*this).actor.child as *mut BossSst);
    (*this).ready = 0 as libc::c_int as s8;
    (*this).effectMode = BONGO_ICE as libc::c_int as u8_0;
    (*this).timer = 35 as libc::c_int as s16;
    i = 0 as libc::c_int;
    while i < 18 as libc::c_int {
        (*this).effects[i as usize].move_0 = 0 as libc::c_int as s16;
        i += 1
    }
    BossSst_SpawnIceCrystal(this, 0 as libc::c_int);
    Actor_SetColorFilter(&mut (*this).actor, 0 as libc::c_int as s16,
                         0xff as libc::c_int as s16, 0 as libc::c_int as s16,
                         0xa as libc::c_int as s16);
    (*this).handAngSpeed = 0 as libc::c_int as s16;
    (*this).actionFunc =
        Some(BossSst_HandFrozen as
                 unsafe extern "C" fn(_: *mut BossSst, _: *mut GlobalContext)
                     -> ());
}
#[no_mangle]
pub unsafe extern "C" fn BossSst_HandFrozen(mut this: *mut BossSst,
                                            mut globalCtx:
                                                *mut GlobalContext) {
    if (*this).timer as libc::c_int != 0 as libc::c_int { (*this).timer -= 1 }
    if (*this).timer as libc::c_int % 2 as libc::c_int != 0 as libc::c_int {
        BossSst_SpawnIceCrystal(this,
                                ((*this).timer as libc::c_int >>
                                     1 as libc::c_int) + 1 as libc::c_int);
    }
    if (*this).ready != 0 {
        BossSst_IceShatter(this);
        BossSst_HandSetupRetreat(this);
        (*sHead).ready = 1 as libc::c_int as s8
    } else {
        (*this).actor.colorFilterTimer = 10 as libc::c_int as u8_0;
        if (*this).handAngSpeed as libc::c_int != 0 as libc::c_int {
            let mut offY: f32_0 =
                Math_SinS((*((*this).actor.child as
                                 *mut BossSst)).actor.shape.rot.x) * 5.0f32;
            let mut offXZ: f32_0 =
                Math_CosS((*((*this).actor.child as
                                 *mut BossSst)).actor.shape.rot.x) * 5.0f32;
            if (*this).handAngSpeed as libc::c_int % 2 as libc::c_int !=
                   0 as libc::c_int {
                offY *= -1.0f32;
                offXZ *= -1.0f32
            }
            (*this).actor.world.pos.x =
                (*this).center.x +
                    Math_CosS((*((*this).actor.child as
                                     *mut BossSst)).actor.shape.rot.y) *
                        offXZ;
            (*this).actor.world.pos.y = (*this).center.y + offY;
            (*this).actor.world.pos.z =
                (*this).center.z +
                    Math_SinS((*((*this).actor.child as
                                     *mut BossSst)).actor.shape.rot.y) *
                        offXZ;
            (*this).handAngSpeed -= 1
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn BossSst_HandSetupReadyBreakIce(mut this:
                                                            *mut BossSst) {
    sHandState[(*this).actor.params as usize] = HAND_BREAK_ICE as libc::c_int;
    Animation_MorphToPlayOnce(&mut (*this).skelAnime,
                              sHandFistPoses[(*this).actor.params as usize],
                              5.0f32);
    (*this).ready = 0 as libc::c_int as s8;
    (*this).actor.colorFilterTimer = 0 as libc::c_int as u8_0;
    if (*this).effectMode as libc::c_int == BONGO_ICE as libc::c_int {
        (*this).effectMode = BONGO_NULL as libc::c_int as u8_0
    }
    (*this).radius =
        Actor_WorldDistXZToPoint(&mut (*this).actor,
                                 &mut (*((*this).actor.child as
                                             *mut BossSst)).center);
    (*this).targetYaw =
        Actor_WorldYawTowardPoint(&mut (*this).actor,
                                  &mut (*((*this).actor.child as
                                              *mut BossSst)).center);
    BossSst_HandSetInvulnerable(this, 1 as libc::c_int);
    (*this).actionFunc =
        Some(BossSst_HandReadyBreakIce as
                 unsafe extern "C" fn(_: *mut BossSst, _: *mut GlobalContext)
                     -> ());
}
#[no_mangle]
pub unsafe extern "C" fn BossSst_HandReadyBreakIce(mut this: *mut BossSst,
                                                   mut globalCtx:
                                                       *mut GlobalContext) {
    let mut inPosition: s32 = 0;
    inPosition =
        Math_ScaledStepToS(&mut (*this).actor.shape.rot.y, (*this).targetYaw,
                           0x400 as libc::c_int as s16);
    inPosition &=
        Math_ScaledStepToS(&mut (*this).actor.shape.rot.x,
                           0x1000 as libc::c_int as s16,
                           0x400 as libc::c_int as s16);
    inPosition &=
        Math_ScaledStepToS(&mut (*this).actor.shape.rot.z,
                           0 as libc::c_int as s16,
                           0x800 as libc::c_int as s16);
    inPosition &=
        Math_ScaledStepToS(&mut (*this).handYRotMod, 0 as libc::c_int as s16,
                           0x400 as libc::c_int as s16);
    inPosition &=
        Math_StepToF(&mut (*this).actor.world.pos.y,
                     (*((*this).actor.child as *mut BossSst)).center.y +
                         200.0f32, 50.0f32);
    inPosition &= Math_StepToF(&mut (*this).radius, 400.0f32, 60.0f32);
    (*this).actor.world.pos.x =
        (*((*this).actor.child as *mut BossSst)).center.x -
            Math_SinS((*this).targetYaw) * (*this).radius;
    (*this).actor.world.pos.z =
        (*((*this).actor.child as *mut BossSst)).center.z -
            Math_CosS((*this).targetYaw) * (*this).radius;
    if SkelAnime_Update(&mut (*this).skelAnime) != 0 && inPosition != 0 {
        BossSst_HandSetupBreakIce(this);
    };
}
#[no_mangle]
pub unsafe extern "C" fn BossSst_HandSetupBreakIce(mut this: *mut BossSst) {
    (*this).timer = 9 as libc::c_int as s16;
    (*this).actionFunc =
        Some(BossSst_HandBreakIce as
                 unsafe extern "C" fn(_: *mut BossSst, _: *mut GlobalContext)
                     -> ());
    (*this).actor.speedXZ = 0.5f32;
}
#[no_mangle]
pub unsafe extern "C" fn BossSst_HandBreakIce(mut this: *mut BossSst,
                                              mut globalCtx:
                                                  *mut GlobalContext) {
    if (*this).timer as libc::c_int % 2 as libc::c_int != 0 as libc::c_int {
        (*this).actor.speedXZ *= 1.5f32;
        (*this).actor.speedXZ =
            if (*this).actor.speedXZ > 60.0f32 {
                60.0f32
            } else { (*this).actor.speedXZ };
        if Math_StepToF(&mut (*this).radius, 100.0f32, (*this).actor.speedXZ)
               != 0 {
            BossSst_SpawnIceShard(this);
            if (*this).timer as libc::c_int != 0 as libc::c_int {
                (*this).timer -= 1
            }
            if (*this).timer as libc::c_int != 0 as libc::c_int {
                Audio_PlayActorSound2(&mut (*this).actor,
                                      0x28cb as libc::c_int as u16_0);
            }
            (*((*this).actor.child as *mut BossSst)).handAngSpeed =
                5 as libc::c_int as s16
        }
    } else {
        (*this).actor.speedXZ *= 0.8f32;
        Math_StepToF(&mut (*this).radius, 500.0f32, (*this).actor.speedXZ);
        if (*this).actor.speedXZ < 2.0f32 {
            if (*this).timer as libc::c_int != 0 as libc::c_int {
                (*this).timer -= 1
            }
        }
    }
    (*this).actor.world.pos.x =
        (*((*this).actor.child as *mut BossSst)).center.x -
            Math_SinS((*this).targetYaw) * (*this).radius;
    (*this).actor.world.pos.z =
        (*((*this).actor.child as *mut BossSst)).center.z -
            Math_CosS((*this).targetYaw) * (*this).radius;
    (*this).actor.world.pos.y =
        (*((*this).actor.child as *mut BossSst)).center.y +
            (*this).radius * 0.4f32;
    if (*this).timer as libc::c_int == 0 as libc::c_int {
        (*((*this).actor.child as *mut BossSst)).ready =
            1 as libc::c_int as s8;
        BossSst_HandSetupRetreat(this);
    }
    func_8002F974(&mut (*this).actor,
                  (0x3967 as libc::c_int - 0x800 as libc::c_int) as u16_0);
}
#[no_mangle]
pub unsafe extern "C" fn BossSst_HandGrabPlayer(mut this: *mut BossSst,
                                                mut globalCtx:
                                                    *mut GlobalContext) {
    let mut player: *mut Player =
        (*globalCtx).actorCtx.actorLists[ACTORCAT_PLAYER as libc::c_int as
                                             usize].head as *mut Player;
    if (*globalCtx).grabPlayer.expect("non-null function pointer")(globalCtx,
                                                                   player) !=
           0 {
        (*player).actor.parent = &mut (*this).actor;
        if (*player).actor.colChkInfo.health as libc::c_int > 0 as libc::c_int
           {
            (*this).colliderJntSph.base.ocFlags1 =
                ((*this).colliderJntSph.base.ocFlags1 as libc::c_int &
                     !((1 as libc::c_int) << 0 as libc::c_int)) as u8_0;
            if sHandState[(*this).actor.params as usize] ==
                   HAND_CLAP as libc::c_int {
                let ref mut fresh7 =
                    (*((*this).actor.child as
                           *mut BossSst)).colliderJntSph.base.ocFlags1;
                *fresh7 =
                    (*fresh7 as libc::c_int &
                         !((1 as libc::c_int) << 0 as libc::c_int)) as u8_0
            }
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn BossSst_HandReleasePlayer(mut this: *mut BossSst,
                                                   mut globalCtx:
                                                       *mut GlobalContext,
                                                   mut dropPlayer: s32) {
    let mut player: *mut Player =
        (*globalCtx).actorCtx.actorLists[ACTORCAT_PLAYER as libc::c_int as
                                             usize].head as *mut Player;
    if (*player).actor.parent == &mut (*this).actor as *mut Actor {
        (*player).actor.parent = 0 as *mut Actor;
        (*player).unk_850 = 100 as libc::c_int as s16;
        (*this).colliderJntSph.base.ocFlags1 =
            ((*this).colliderJntSph.base.ocFlags1 as libc::c_int |
                 (1 as libc::c_int) << 0 as libc::c_int) as u8_0;
        let ref mut fresh8 =
            (*((*this).actor.child as
                   *mut BossSst)).colliderJntSph.base.ocFlags1;
        *fresh8 =
            (*fresh8 as libc::c_int | (1 as libc::c_int) << 0 as libc::c_int)
                as u8_0;
        if dropPlayer != 0 {
            func_8002F71C(globalCtx, &mut (*this).actor, 0.0f32,
                          (*this).actor.shape.rot.y, 0.0f32);
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn BossSst_MoveAround(mut this: *mut BossSst) {
    let mut hand: *mut BossSst = 0 as *mut BossSst;
    let mut vec: *mut Vec3f = 0 as *mut Vec3f;
    let mut sn: f32_0 = 0.;
    let mut cs: f32_0 = 0.;
    let mut i: s32 = 0;
    sn = Math_SinS((*this).actor.shape.rot.y);
    cs = Math_CosS((*this).actor.shape.rot.y);
    if (*this).actionFunc !=
           Some(BossSst_HeadEndCharge as
                    unsafe extern "C" fn(_: *mut BossSst,
                                         _: *mut GlobalContext) -> ()) {
        (*this).actor.world.pos.x = sRoomCenter.x + (*this).radius * sn;
        (*this).actor.world.pos.z = sRoomCenter.z + (*this).radius * cs
    }
    i = 0 as libc::c_int;
    while i < 2 as libc::c_int {
        hand = sHands[i as usize];
        vec =
            &mut *sHandOffsets.as_mut_ptr().offset(i as isize) as *mut Vec3f;
        (*hand).actor.world.pos.x =
            (*this).actor.world.pos.x + (*vec).z * sn + (*vec).x * cs;
        (*hand).actor.world.pos.y = (*this).actor.world.pos.y + (*vec).y;
        (*hand).actor.world.pos.z =
            (*this).actor.world.pos.z + (*vec).z * cs - (*vec).x * sn;
        (*hand).actor.home.pos.x =
            (*this).actor.world.pos.x + 400.0f32 * sn +
                -200.0f32 * (*hand).actionVar as libc::c_int as libc::c_float
                    * cs;
        (*hand).actor.home.pos.y = (*this).actor.world.pos.y;
        (*hand).actor.home.pos.z =
            (*this).actor.world.pos.z + 400.0f32 * cs -
                -200.0f32 * (*hand).actionVar as libc::c_int as libc::c_float
                    * sn;
        (*hand).actor.home.rot.y = (*this).actor.shape.rot.y;
        (*hand).actor.shape.rot.y =
            (sHandYawOffsets[i as usize] as libc::c_int +
                 (*this).actor.shape.rot.y as libc::c_int) as s16;
        if (*hand).actor.world.pos.y < (*hand).actor.floorHeight {
            (*hand).actor.world.pos.y = (*hand).actor.floorHeight
        }
        i += 1
    };
}
#[no_mangle]
pub unsafe extern "C" fn BossSst_HandSelectAttack(mut this: *mut BossSst) {
    let mut rand: f32_0 = Rand_ZeroOne() * 6.0f32;
    let mut randInt: s32 = 0;
    if sHandState[(*((*this).actor.child as *mut BossSst)).actor.params as
                      usize] == HAND_DAMAGED as libc::c_int {
        rand *= 5.0f32 / 6 as libc::c_int as libc::c_float;
        if rand > 4.0f32 { rand = 4.0f32 }
    }
    randInt = rand as s32;
    if randInt == 0 as libc::c_int {
        BossSst_HandSetupReadySlam(this);
    } else if randInt == 1 as libc::c_int {
        BossSst_HandSetupReadySweep(this);
    } else if randInt == 2 as libc::c_int {
        BossSst_HandSetupReadyPunch(this);
    } else if randInt == 5 as libc::c_int {
        BossSst_HandSetupReadyClap(this);
    } else { BossSst_HandSetupReadyGrab(this); };
}
#[no_mangle]
pub unsafe extern "C" fn BossSst_HandSetDamage(mut this: *mut BossSst,
                                               mut damage: s32) {
    let mut i: s32 = 0;
    (*this).colliderJntSph.base.atFlags =
        ((*this).colliderJntSph.base.atFlags as libc::c_int |
             (1 as libc::c_int) << 0 as libc::c_int) as u8_0;
    i = 0 as libc::c_int;
    while i < 11 as libc::c_int {
        (*(*this).colliderJntSph.elements.offset(i as
                                                     isize)).info.toucher.damage
            = damage as u8_0;
        i += 1
    };
}
#[no_mangle]
pub unsafe extern "C" fn BossSst_HandSetInvulnerable(mut this: *mut BossSst,
                                                     mut isInv: s32) {
    (*this).colliderJntSph.base.acFlags =
        ((*this).colliderJntSph.base.acFlags as libc::c_int &
             !((1 as libc::c_int) << 1 as libc::c_int)) as u8_0;
    if isInv != 0 {
        (*this).colliderJntSph.base.colType =
            COLTYPE_HARD as libc::c_int as u8_0;
        (*this).colliderJntSph.base.acFlags =
            ((*this).colliderJntSph.base.acFlags as libc::c_int |
                 (1 as libc::c_int) << 2 as libc::c_int) as u8_0
    } else {
        (*this).colliderJntSph.base.colType =
            COLTYPE_HIT0 as libc::c_int as u8_0;
        (*this).colliderJntSph.base.acFlags =
            ((*this).colliderJntSph.base.acFlags as libc::c_int &
                 !((1 as libc::c_int) << 2 as libc::c_int)) as u8_0
    };
}
#[no_mangle]
pub unsafe extern "C" fn BossSst_HeadSfx(mut this: *mut BossSst,
                                         mut sfxId: u16_0) {
    func_80078914(&mut (*this).center, sfxId);
}
#[no_mangle]
pub unsafe extern "C" fn BossSst_HandCollisionCheck(mut this: *mut BossSst,
                                                    mut globalCtx:
                                                        *mut GlobalContext) {
    if (*this).colliderJntSph.base.acFlags as libc::c_int &
           (1 as libc::c_int) << 1 as libc::c_int != 0 &&
           (*this).colliderJntSph.base.colType as libc::c_int !=
               COLTYPE_HARD as libc::c_int {
        let mut bothHands: s32 = 1 as libc::c_int;
        (*this).colliderJntSph.base.acFlags =
            ((*this).colliderJntSph.base.acFlags as libc::c_int &
                 !((1 as libc::c_int) << 1 as libc::c_int)) as u8_0;
        if (*this).actor.colChkInfo.damageEffect as libc::c_int !=
               0 as libc::c_int ||
               (*this).actor.colChkInfo.damage as libc::c_int !=
                   0 as libc::c_int {
            (*this).colliderJntSph.base.atFlags =
                ((*this).colliderJntSph.base.atFlags as libc::c_int &
                     !((1 as libc::c_int) << 0 as libc::c_int |
                           (1 as libc::c_int) << 1 as libc::c_int)) as u8_0;
            (*this).colliderJntSph.base.acFlags =
                ((*this).colliderJntSph.base.acFlags as libc::c_int &
                     !((1 as libc::c_int) << 0 as libc::c_int)) as u8_0;
            (*this).colliderJntSph.base.ocFlags1 =
                ((*this).colliderJntSph.base.ocFlags1 as libc::c_int &
                     !((1 as libc::c_int) << 2 as libc::c_int)) as u8_0;
            BossSst_HandReleasePlayer(this, globalCtx, 1 as libc::c_int);
            if sHandState[(*((*this).actor.child as
                                 *mut BossSst)).actor.params as usize] ==
                   HAND_CLAP as libc::c_int {
                BossSst_HandReleasePlayer((*this).actor.child as *mut BossSst,
                                          globalCtx, 1 as libc::c_int);
                BossSst_HandSetupRetreat((*this).actor.child as *mut BossSst);
            }
            (*this).actor.flags &=
                !((1 as libc::c_int) << 0 as libc::c_int) as libc::c_uint;
            if (*this).actor.colChkInfo.damageEffect as libc::c_int ==
                   3 as libc::c_int {
                BossSst_HandSetupFrozen(this);
            } else {
                BossSst_HandSetupReel(this);
                if sHandState[(*((*this).actor.child as
                                     *mut BossSst)).actor.params as usize] !=
                       HAND_DAMAGED as libc::c_int {
                    bothHands = 0 as libc::c_int
                }
            }
            BossSst_HeadSetupDamagedHand(sHead, bothHands);
            Item_DropCollectible(globalCtx, &mut (*this).actor.world.pos,
                                 if Rand_ZeroOne() < 0.5f32 {
                                     ITEM00_ARROWS_SMALL as libc::c_int
                                 } else { ITEM00_MAGIC_SMALL as libc::c_int }
                                     as s16);
            Audio_PlayActorSound2(&mut (*this).actor,
                                  0x396a as libc::c_int as u16_0);
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn BossSst_HeadCollisionCheck(mut this: *mut BossSst,
                                                    mut globalCtx:
                                                        *mut GlobalContext) {
    if (*this).colliderCyl.base.acFlags as libc::c_int &
           (1 as libc::c_int) << 1 as libc::c_int != 0 {
        (*this).colliderCyl.base.acFlags =
            ((*this).colliderCyl.base.acFlags as libc::c_int &
                 !((1 as libc::c_int) << 1 as libc::c_int)) as u8_0;
        if (*this).actor.colChkInfo.damageEffect as libc::c_int !=
               0 as libc::c_int ||
               (*this).actor.colChkInfo.damage as libc::c_int !=
                   0 as libc::c_int {
            if (*this).actionFunc ==
                   Some(BossSst_HeadVulnerable as
                            unsafe extern "C" fn(_: *mut BossSst,
                                                 _: *mut GlobalContext) -> ())
               {
                if Actor_ApplyDamage(&mut (*this).actor) as libc::c_int ==
                       0 as libc::c_int {
                    Enemy_StartFinishingBlow(globalCtx, &mut (*this).actor);
                    BossSst_HeadSetupDeath(this, globalCtx);
                } else { BossSst_HeadSetupDamage(this); }
                BossSst_HandSetupDamage(sHands[0 as libc::c_int as usize]);
                BossSst_HandSetupDamage(sHands[1 as libc::c_int as usize]);
            } else {
                BossSst_HeadSetupStunned(this);
                if sHandState[(*sHands[1 as libc::c_int as
                                           usize]).actor.params as usize] ==
                       HAND_FROZEN as libc::c_int {
                    BossSst_IceShatter(sHands[1 as libc::c_int as usize]);
                } else if sHandState[(*sHands[0 as libc::c_int as
                                                  usize]).actor.params as
                                         usize] == HAND_FROZEN as libc::c_int
                 {
                    BossSst_IceShatter(sHands[0 as libc::c_int as usize]);
                }
                BossSst_HandSetupStunned(sHands[1 as libc::c_int as usize]);
                BossSst_HandSetupStunned(sHands[0 as libc::c_int as usize]);
            }
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn BossSst_UpdateHand(mut thisx: *mut Actor,
                                            mut globalCtx:
                                                *mut GlobalContext) {
    let mut pad: s32 = 0;
    let mut this: *mut BossSst = thisx as *mut BossSst;
    let mut trail: *mut BossSstHandTrail = 0 as *mut BossSstHandTrail;
    if (*this).colliderCyl.base.atFlags as libc::c_int &
           (1 as libc::c_int) << 0 as libc::c_int != 0 {
        if ((*this).effects[0 as libc::c_int as usize].move_0 as libc::c_int)
               < 5 as libc::c_int ||
               (*this).actor.xzDistToPlayer <
                   (*this).effects[2 as libc::c_int as usize].scale as
                       libc::c_int as libc::c_float * 0.01f32 *
                       sCylinderInitHand.dim.radius as libc::c_int as
                           libc::c_float ||
               (*this).colliderCyl.base.atFlags as libc::c_int &
                   (1 as libc::c_int) << 1 as libc::c_int != 0 {
            (*this).colliderCyl.base.atFlags =
                ((*this).colliderCyl.base.atFlags as libc::c_int &
                     !((1 as libc::c_int) << 0 as libc::c_int |
                           (1 as libc::c_int) << 1 as libc::c_int)) as u8_0
        } else {
            (*this).colliderCyl.dim.radius =
                ((*this).effects[0 as libc::c_int as usize].scale as
                     libc::c_int as libc::c_float * 0.01f32 *
                     sCylinderInitHand.dim.radius as libc::c_int as
                         libc::c_float) as s16
        }
    }
    BossSst_HandCollisionCheck(this, globalCtx);
    (*this).actionFunc.expect("non-null function pointer")(this, globalCtx);
    Actor_UpdateBgCheckInfo(globalCtx, &mut (*this).actor, 50.0f32, 130.0f32,
                            0.0f32, 5 as libc::c_int);
    Actor_SetFocus(&mut (*this).actor, 0.0f32);
    if (*this).colliderJntSph.base.atFlags as libc::c_int &
           (1 as libc::c_int) << 0 as libc::c_int != 0 {
        CollisionCheck_SetAT(globalCtx, &mut (*globalCtx).colChkCtx,
                             &mut (*this).colliderJntSph.base);
    }
    if (*sHead).actionFunc !=
           Some(BossSst_HeadLurk as
                    unsafe extern "C" fn(_: *mut BossSst,
                                         _: *mut GlobalContext) -> ()) &&
           (*sHead).actionFunc !=
               Some(BossSst_HeadIntro as
                        unsafe extern "C" fn(_: *mut BossSst,
                                             _: *mut GlobalContext) -> ()) &&
           (*this).colliderJntSph.base.acFlags as libc::c_int &
               (1 as libc::c_int) << 0 as libc::c_int != 0 {
        CollisionCheck_SetAC(globalCtx, &mut (*globalCtx).colChkCtx,
                             &mut (*this).colliderJntSph.base);
    }
    if (*this).colliderJntSph.base.ocFlags1 as libc::c_int &
           (1 as libc::c_int) << 0 as libc::c_int != 0 {
        CollisionCheck_SetOC(globalCtx, &mut (*globalCtx).colChkCtx,
                             &mut (*this).colliderJntSph.base);
    }
    if (*this).colliderCyl.base.atFlags as libc::c_int &
           (1 as libc::c_int) << 0 as libc::c_int != 0 {
        CollisionCheck_SetAT(globalCtx, &mut (*globalCtx).colChkCtx,
                             &mut (*this).colliderCyl.base);
    }
    if sHandState[(*this).actor.params as usize] != HAND_DEATH as libc::c_int
           &&
           sHandState[(*this).actor.params as usize] !=
               HAND_WAIT as libc::c_int &&
           sHandState[(*this).actor.params as usize] !=
               HAND_BEAT as libc::c_int &&
           sHandState[(*this).actor.params as usize] !=
               HAND_FROZEN as libc::c_int {
        (*this).trailCount += 1;
        (*this).trailCount =
            if (*this).trailCount as libc::c_int > 7 as libc::c_int {
                7 as libc::c_int
            } else { (*this).trailCount as libc::c_int } as s16
    } else {
        (*this).trailCount -= 1;
        (*this).trailCount =
            if ((*this).trailCount as libc::c_int) < 0 as libc::c_int {
                0 as libc::c_int
            } else { (*this).trailCount as libc::c_int } as s16
    }
    trail =
        &mut *(*this).handTrails.as_mut_ptr().offset((*this).trailIndex as
                                                         isize) as
            *mut BossSstHandTrail;
    Math_Vec3f_Copy(&mut (*trail).world.pos, &mut (*this).actor.world.pos);
    (*trail).world.rot = (*this).actor.shape.rot;
    (*trail).zPosMod = (*this).handZPosMod as f32_0;
    (*trail).yRotMod = (*this).handYRotMod;
    (*this).trailIndex =
        (((*this).trailIndex as libc::c_int + 1 as libc::c_int) %
             7 as libc::c_int) as s16;
    BossSst_UpdateEffect(&mut (*this).actor, globalCtx);
}
#[no_mangle]
pub unsafe extern "C" fn BossSst_UpdateHead(mut thisx: *mut Actor,
                                            mut globalCtx:
                                                *mut GlobalContext) {
    let mut pad: s32 = 0;
    let mut this: *mut BossSst = thisx as *mut BossSst;
    func_8002DBD0(&mut (*this).actor,
                  &mut *sHandOffsets.as_mut_ptr().offset(1 as libc::c_int as
                                                             isize),
                  &mut (**sHands.as_mut_ptr().offset(1 as libc::c_int as
                                                         isize)).actor.world.pos);
    func_8002DBD0(&mut (*this).actor,
                  &mut *sHandOffsets.as_mut_ptr().offset(0 as libc::c_int as
                                                             isize),
                  &mut (**sHands.as_mut_ptr().offset(0 as libc::c_int as
                                                         isize)).actor.world.pos);
    sHandYawOffsets[0 as libc::c_int as usize] =
        ((*sHands[0 as libc::c_int as usize]).actor.shape.rot.y as libc::c_int
             - (*thisx).shape.rot.y as libc::c_int) as s16;
    sHandYawOffsets[1 as libc::c_int as usize] =
        ((*sHands[1 as libc::c_int as usize]).actor.shape.rot.y as libc::c_int
             - (*thisx).shape.rot.y as libc::c_int) as s16;
    BossSst_HeadCollisionCheck(this, globalCtx);
    (*this).actionFunc.expect("non-null function pointer")(this, globalCtx);
    if (*this).actionVar != 0 {
        if (*globalCtx).actorCtx.unk_03 as libc::c_int == 0 as libc::c_int ||
               (*thisx).colorFilterTimer as libc::c_int != 0 as libc::c_int {
            (*this).actor.flags &=
                !((1 as libc::c_int) << 7 as libc::c_int) as libc::c_uint
        } else {
            (*this).actor.flags |=
                ((1 as libc::c_int) << 7 as libc::c_int) as libc::c_uint
        }
    }
    if (*this).colliderJntSph.base.atFlags as libc::c_int &
           (1 as libc::c_int) << 0 as libc::c_int != 0 {
        CollisionCheck_SetAT(globalCtx, &mut (*globalCtx).colChkCtx,
                             &mut (*this).colliderJntSph.base);
    }
    if (*this).actionFunc !=
           Some(BossSst_HeadLurk as
                    unsafe extern "C" fn(_: *mut BossSst,
                                         _: *mut GlobalContext) -> ()) &&
           (*this).actionFunc !=
               Some(BossSst_HeadIntro as
                        unsafe extern "C" fn(_: *mut BossSst,
                                             _: *mut GlobalContext) -> ()) {
        if (*this).colliderCyl.base.acFlags as libc::c_int &
               (1 as libc::c_int) << 0 as libc::c_int != 0 {
            CollisionCheck_SetAC(globalCtx, &mut (*globalCtx).colChkCtx,
                                 &mut (*this).colliderCyl.base);
        }
        CollisionCheck_SetAC(globalCtx, &mut (*globalCtx).colChkCtx,
                             &mut (*this).colliderJntSph.base);
    }
    if (*this).colliderJntSph.base.ocFlags1 as libc::c_int &
           (1 as libc::c_int) << 0 as libc::c_int != 0 {
        CollisionCheck_SetOC(globalCtx, &mut (*globalCtx).colChkCtx,
                             &mut (*this).colliderJntSph.base);
    }
    BossSst_MoveAround(this);
    if ((*this).actionVar == 0 ||
            (*this).actor.flags &
                ((1 as libc::c_int) << 7 as libc::c_int) as libc::c_uint ==
                ((1 as libc::c_int) << 7 as libc::c_int) as libc::c_uint) &&
           ((*this).actionFunc ==
                Some(BossSst_HeadReadyCharge as
                         unsafe extern "C" fn(_: *mut BossSst,
                                              _: *mut GlobalContext) -> ()) ||
                (*this).actionFunc ==
                    Some(BossSst_HeadCharge as
                             unsafe extern "C" fn(_: *mut BossSst,
                                                  _: *mut GlobalContext)
                                 -> ()) ||
                (*this).actionFunc ==
                    Some(BossSst_HeadFrozenHand as
                             unsafe extern "C" fn(_: *mut BossSst,
                                                  _: *mut GlobalContext)
                                 -> ()) ||
                (*this).actionFunc ==
                    Some(BossSst_HeadStunned as
                             unsafe extern "C" fn(_: *mut BossSst,
                                                  _: *mut GlobalContext)
                                 -> ()) ||
                (*this).actionFunc ==
                    Some(BossSst_HeadVulnerable as
                             unsafe extern "C" fn(_: *mut BossSst,
                                                  _: *mut GlobalContext)
                                 -> ()) ||
                (*this).actionFunc ==
                    Some(BossSst_HeadDamage as
                             unsafe extern "C" fn(_: *mut BossSst,
                                                  _: *mut GlobalContext)
                                 -> ())) {
        (*this).actor.flags |=
            ((1 as libc::c_int) << 0 as libc::c_int) as libc::c_uint
    } else {
        (*this).actor.flags &=
            !((1 as libc::c_int) << 0 as libc::c_int) as libc::c_uint
    }
    if (*this).actionFunc ==
           Some(BossSst_HeadCharge as
                    unsafe extern "C" fn(_: *mut BossSst,
                                         _: *mut GlobalContext) -> ()) {
        BossSst_HeadSfx(this,
                        (0x398d as libc::c_int - 0x800 as libc::c_int) as
                            u16_0);
    }
    BossSst_UpdateEffect(&mut (*this).actor, globalCtx);
}
#[no_mangle]
pub unsafe extern "C" fn BossSst_OverrideHandDraw(mut globalCtx:
                                                      *mut GlobalContext,
                                                  mut limbIndex: s32,
                                                  mut dList: *mut *mut Gfx,
                                                  mut pos: *mut Vec3f,
                                                  mut rot: *mut Vec3s,
                                                  mut thisx:
                                                      *mut libc::c_void)
 -> s32 {
    let mut this: *mut BossSst = thisx as *mut BossSst;
    if limbIndex == 1 as libc::c_int {
        (*pos).z += (*this).handZPosMod as libc::c_int as libc::c_float;
        (*rot).y =
            ((*rot).y as libc::c_int + (*this).handYRotMod as libc::c_int) as
                s16
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn BossSst_PostHandDraw(mut globalCtx:
                                                  *mut GlobalContext,
                                              mut limbIndex: s32,
                                              mut dList: *mut *mut Gfx,
                                              mut rot: *mut Vec3s,
                                              mut thisx: *mut libc::c_void) {
    let mut this: *mut BossSst = thisx as *mut BossSst;
    Collider_UpdateSpheres(limbIndex, &mut (*this).colliderJntSph);
}
#[no_mangle]
pub unsafe extern "C" fn BossSst_OverrideHandTrailDraw(mut globalCtx:
                                                           *mut GlobalContext,
                                                       mut limbIndex: s32,
                                                       mut dList:
                                                           *mut *mut Gfx,
                                                       mut pos: *mut Vec3f,
                                                       mut rot: *mut Vec3s,
                                                       mut data:
                                                           *mut libc::c_void,
                                                       mut gfx: *mut *mut Gfx)
 -> s32 {
    let mut trail: *mut BossSstHandTrail = data as *mut BossSstHandTrail;
    if limbIndex == 1 as libc::c_int {
        (*pos).z += (*trail).zPosMod;
        (*rot).y =
            ((*rot).y as libc::c_int + (*trail).yRotMod as libc::c_int) as s16
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn BossSst_DrawHand(mut thisx: *mut Actor,
                                          mut globalCtx: *mut GlobalContext) {
    let mut this: *mut BossSst = thisx as *mut BossSst;
    let mut __gfxCtx: *mut GraphicsContext = 0 as *mut GraphicsContext;
    let mut dispRefs: [*mut Gfx; 4] = [0 as *mut Gfx; 4];
    __gfxCtx = (*globalCtx).state.gfxCtx;
    Graph_OpenDisps(dispRefs.as_mut_ptr(), (*globalCtx).state.gfxCtx,
                    b"../z_boss_sst.c\x00" as *const u8 as
                        *const libc::c_char, 6563 as libc::c_int);
    func_80093D18((*globalCtx).state.gfxCtx);
    let fresh9 = (*__gfxCtx).polyOpa.p;
    (*__gfxCtx).polyOpa.p = (*__gfxCtx).polyOpa.p.offset(1);
    let mut _g: *mut Gfx = fresh9;
    (*_g).words.w0 =
        (0xfa as libc::c_int as u32_0 &
             (((0x1 as libc::c_int) << 8 as libc::c_int) - 1 as libc::c_int)
                 as libc::c_uint) << 24 as libc::c_int |
            (0 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 8 as libc::c_int |
            (0x80 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 0 as libc::c_int;
    (*_g).words.w1 =
        (sBodyColor.r as u32_0 &
             (((0x1 as libc::c_int) << 8 as libc::c_int) - 1 as libc::c_int)
                 as libc::c_uint) << 24 as libc::c_int |
            (sBodyColor.g as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 16 as libc::c_int
            |
            (sBodyColor.b as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 8 as libc::c_int |
            (255 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 0 as libc::c_int;
    if sBodyStatic == 0 {
        let fresh10 = (*__gfxCtx).polyOpa.p;
        (*__gfxCtx).polyOpa.p = (*__gfxCtx).polyOpa.p.offset(1);
        let mut _g_0: *mut Gfx = fresh10;
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
            &mut *D_80116280.as_mut_ptr().offset(2 as libc::c_int as isize) as
                *mut Gfx as libc::c_uint
    } else {
        let fresh11 = (*__gfxCtx).polyOpa.p;
        (*__gfxCtx).polyOpa.p = (*__gfxCtx).polyOpa.p.offset(1);
        let mut _g_1: *mut Gfx = fresh11;
        (*_g_1).words.w0 =
            (0xfb as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 24 as libc::c_int;
        (*_g_1).words.w1 =
            (sStaticColor.r as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 24 as libc::c_int
                |
                (sStaticColor.g as u32_0 &
                     (((0x1 as libc::c_int) << 8 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    16 as libc::c_int |
                (sStaticColor.b as u32_0 &
                     (((0x1 as libc::c_int) << 8 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    8 as libc::c_int |
                (0 as libc::c_int as u32_0 &
                     (((0x1 as libc::c_int) << 8 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    0 as libc::c_int;
        let fresh12 = (*__gfxCtx).polyOpa.p;
        (*__gfxCtx).polyOpa.p = (*__gfxCtx).polyOpa.p.offset(1);
        let mut _g_2: *mut Gfx = fresh12;
        (*_g_2).words.w0 =
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
        (*_g_2).words.w1 = sBodyStaticDList.as_mut_ptr() as libc::c_uint
    }
    SkelAnime_DrawFlexOpa(globalCtx, (*this).skelAnime.skeleton,
                          (*this).skelAnime.jointTable,
                          (*this).skelAnime.dListCount as s32,
                          Some(BossSst_OverrideHandDraw as
                                   unsafe extern "C" fn(_: *mut GlobalContext,
                                                        _: s32,
                                                        _: *mut *mut Gfx,
                                                        _: *mut Vec3f,
                                                        _: *mut Vec3s,
                                                        _: *mut libc::c_void)
                                       -> s32),
                          Some(BossSst_PostHandDraw as
                                   unsafe extern "C" fn(_: *mut GlobalContext,
                                                        _: s32,
                                                        _: *mut *mut Gfx,
                                                        _: *mut Vec3s,
                                                        _: *mut libc::c_void)
                                       -> ()), this as *mut libc::c_void);
    if (*this).trailCount as libc::c_int >= 2 as libc::c_int {
        let mut trail: *mut BossSstHandTrail = 0 as *mut BossSstHandTrail;
        let mut trail2: *mut BossSstHandTrail = 0 as *mut BossSstHandTrail;
        let mut i: s32 = 0;
        let mut idx: s32 = 0;
        let mut end: s32 = 0;
        let mut pad: s32 = 0;
        func_80093D84((*globalCtx).state.gfxCtx);
        end = (*this).trailCount as libc::c_int >> 1 as libc::c_int;
        idx =
            ((*this).trailIndex as libc::c_int + 4 as libc::c_int) %
                7 as libc::c_int;
        trail =
            &mut *(*this).handTrails.as_mut_ptr().offset(idx as isize) as
                *mut BossSstHandTrail;
        trail2 =
            &mut *(*this).handTrails.as_mut_ptr().offset(((idx +
                                                               2 as
                                                                   libc::c_int)
                                                              %
                                                              7 as
                                                                  libc::c_int)
                                                             as isize) as
                *mut BossSstHandTrail;
        i = 0 as libc::c_int;
        while i < end {
            if Math3D_Vec3fDistSq(&mut (*trail2).world.pos,
                                  &mut (*trail).world.pos) > 900.0f32 {
                func_800D1694((*trail).world.pos.x, (*trail).world.pos.y,
                              (*trail).world.pos.z, &mut (*trail).world.rot);
                Matrix_Scale(0.02f32, 0.02f32, 0.02f32,
                             MTXMODE_APPLY as libc::c_int as u8_0);
                let fresh13 = (*__gfxCtx).polyXlu.p;
                (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
                let mut _g_3: *mut Gfx = fresh13;
                (*_g_3).words.w0 =
                    (0xdb as libc::c_int as u32_0 &
                         (((0x1 as libc::c_int) << 8 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        24 as libc::c_int |
                        (0x6 as libc::c_int as u32_0 &
                             (((0x1 as libc::c_int) << 8 as libc::c_int) -
                                  1 as libc::c_int) as libc::c_uint) <<
                            16 as libc::c_int |
                        ((0x8 as libc::c_int * 4 as libc::c_int) as u32_0 &
                             (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                  1 as libc::c_int) as libc::c_uint) <<
                            0 as libc::c_int;
                (*_g_3).words.w1 =
                    sHandTrailDList.as_mut_ptr() as libc::c_uint;
                let fresh14 = (*__gfxCtx).polyXlu.p;
                (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
                let mut _g_4: *mut Gfx = fresh14;
                (*_g_4).words.w0 =
                    (0xfa as libc::c_int as u32_0 &
                         (((0x1 as libc::c_int) << 8 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        24 as libc::c_int |
                        (0 as libc::c_int as u32_0 &
                             (((0x1 as libc::c_int) << 8 as libc::c_int) -
                                  1 as libc::c_int) as libc::c_uint) <<
                            8 as libc::c_int |
                        (0 as libc::c_int as u32_0 &
                             (((0x1 as libc::c_int) << 8 as libc::c_int) -
                                  1 as libc::c_int) as libc::c_uint) <<
                            0 as libc::c_int;
                (*_g_4).words.w1 =
                    (((3 as libc::c_int - i) * 10 as libc::c_int +
                          20 as libc::c_int) as u32_0 &
                         (((0x1 as libc::c_int) << 8 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        24 as libc::c_int |
                        (0 as libc::c_int as u32_0 &
                             (((0x1 as libc::c_int) << 8 as libc::c_int) -
                                  1 as libc::c_int) as libc::c_uint) <<
                            16 as libc::c_int |
                        (((3 as libc::c_int - i) * 20 as libc::c_int +
                              50 as libc::c_int) as u32_0 &
                             (((0x1 as libc::c_int) << 8 as libc::c_int) -
                                  1 as libc::c_int) as libc::c_uint) <<
                            8 as libc::c_int |
                        (((3 as libc::c_int - i) * 30 as libc::c_int +
                              70 as libc::c_int) as u32_0 &
                             (((0x1 as libc::c_int) << 8 as libc::c_int) -
                                  1 as libc::c_int) as libc::c_uint) <<
                            0 as libc::c_int;
                (*__gfxCtx).polyXlu.p =
                    SkelAnime_DrawFlex(globalCtx, (*this).skelAnime.skeleton,
                                       (*this).skelAnime.jointTable,
                                       (*this).skelAnime.dListCount as s32,
                                       Some(BossSst_OverrideHandTrailDraw as
                                                unsafe extern "C" fn(_:
                                                                         *mut GlobalContext,
                                                                     _: s32,
                                                                     _:
                                                                         *mut *mut Gfx,
                                                                     _:
                                                                         *mut Vec3f,
                                                                     _:
                                                                         *mut Vec3s,
                                                                     _:
                                                                         *mut libc::c_void,
                                                                     _:
                                                                         *mut *mut Gfx)
                                                    -> s32), None,
                                       trail as *mut libc::c_void,
                                       (*__gfxCtx).polyXlu.p)
            }
            idx = (idx + 5 as libc::c_int) % 7 as libc::c_int;
            trail2 = trail;
            trail =
                &mut *(*this).handTrails.as_mut_ptr().offset(idx as isize) as
                    *mut BossSstHandTrail;
            i += 1
        }
    }
    Graph_CloseDisps(dispRefs.as_mut_ptr(), (*globalCtx).state.gfxCtx,
                     b"../z_boss_sst.c\x00" as *const u8 as
                         *const libc::c_char, 6654 as libc::c_int);
    BossSst_DrawEffect(&mut (*this).actor, globalCtx);
}
#[no_mangle]
pub unsafe extern "C" fn BossSst_OverrideHeadDraw(mut globalCtx:
                                                      *mut GlobalContext,
                                                  mut limbIndex: s32,
                                                  mut dList: *mut *mut Gfx,
                                                  mut pos: *mut Vec3f,
                                                  mut rot: *mut Vec3s,
                                                  mut thisx:
                                                      *mut libc::c_void,
                                                  mut gfx: *mut *mut Gfx)
 -> s32 {
    let mut this: *mut BossSst = thisx as *mut BossSst;
    let mut shakeAmp: s32 = 0;
    let mut pad: s32 = 0;
    let mut timer12: s32 = 0;
    let mut shakeMod: f32_0 = 0.;
    if !((*this).actor.flags &
             ((1 as libc::c_int) << 7 as libc::c_int) as libc::c_uint ==
             ((1 as libc::c_int) << 7 as libc::c_int) as libc::c_uint) &&
           (*this).actionVar as libc::c_int != 0 {
        *dList = 0 as *mut Gfx
    } else if (*this).actionFunc ==
                  Some(BossSst_HeadThrash as
                           unsafe extern "C" fn(_: *mut BossSst,
                                                _: *mut GlobalContext) -> ())
     {
        // Animation modifications for death cutscene
        shakeAmp =
            (*this).timer as libc::c_int / 10 as libc::c_int +
                1 as libc::c_int;
        if limbIndex == 3 as libc::c_int || limbIndex == 39 as libc::c_int ||
               limbIndex == 42 as libc::c_int {
            shakeMod =
                sinf((*this).timer as libc::c_int as libc::c_float *
                         (3.14159265358979323846f32 /
                              5 as libc::c_int as libc::c_float));
            (*rot).x =
                ((*rot).x as libc::c_float +
                     (0x500 as libc::c_int as libc::c_float * Rand_ZeroOne() +
                          0xa00 as libc::c_int as libc::c_float) /
                         0x10 as libc::c_int as libc::c_float *
                         shakeAmp as libc::c_float * shakeMod) as s16;
            shakeMod =
                sinf(((*this).timer as libc::c_int % 5 as libc::c_int) as
                         libc::c_float *
                         (3.14159265358979323846f32 /
                              5 as libc::c_int as libc::c_float));
            (*rot).z =
                ((*rot).z as libc::c_float -
                     ((0x800 as libc::c_int as libc::c_float * Rand_ZeroOne()
                           + 0x1000 as libc::c_int as libc::c_float) /
                          0x10 as libc::c_int as libc::c_float *
                          shakeAmp as libc::c_float * shakeMod +
                          0x1000 as libc::c_int as libc::c_float)) as s16;
            if limbIndex == 3 as libc::c_int {
                shakeMod =
                    sinf((*this).timer as libc::c_int as libc::c_float *
                             (3.14159265358979323846f32 /
                                  5 as libc::c_int as libc::c_float));
                (*rot).y =
                    ((*rot).y as libc::c_float +
                         (0x500 as libc::c_int as libc::c_float *
                              Rand_ZeroOne() +
                              0xa00 as libc::c_int as libc::c_float) /
                             0x10 as libc::c_int as libc::c_float *
                             shakeAmp as libc::c_float * shakeMod) as s16
            }
        } else if limbIndex == 5 as libc::c_int ||
                      limbIndex == 6 as libc::c_int {
            shakeMod =
                sinf(((*this).timer as libc::c_int % 5 as libc::c_int) as
                         libc::c_float *
                         (3.14159265358979323846f32 /
                              5 as libc::c_int as libc::c_float));
            (*rot).z =
                ((*rot).z as libc::c_float -
                     ((0x280 as libc::c_int as libc::c_float * Rand_ZeroOne()
                           + 0x500 as libc::c_int as libc::c_float) /
                          0x10 as libc::c_int as libc::c_float *
                          shakeAmp as libc::c_float * shakeMod +
                          0x500 as libc::c_int as libc::c_float)) as s16;
            if limbIndex == 5 as libc::c_int {
                shakeMod =
                    sinf((*this).timer as libc::c_int as libc::c_float *
                             (3.14159265358979323846f32 /
                                  5 as libc::c_int as libc::c_float));
                (*rot).x =
                    ((*rot).x as libc::c_float +
                         (0x500 as libc::c_int as libc::c_float *
                              Rand_ZeroOne() +
                              0xa00 as libc::c_int as libc::c_float) /
                             0x10 as libc::c_int as libc::c_float *
                             shakeAmp as libc::c_float * shakeMod) as s16;
                shakeMod =
                    sinf((*this).timer as libc::c_int as libc::c_float *
                             (3.14159265358979323846f32 /
                                  5 as libc::c_int as libc::c_float));
                (*rot).y =
                    ((*rot).y as libc::c_float +
                         (0x500 as libc::c_int as libc::c_float *
                              Rand_ZeroOne() +
                              0xa00 as libc::c_int as libc::c_float) /
                             0x10 as libc::c_int as libc::c_float *
                             shakeAmp as libc::c_float * shakeMod) as s16
            }
        } else if limbIndex == 2 as libc::c_int {
            shakeMod =
                sinf((*this).timer as libc::c_int as libc::c_float *
                         (3.14159265358979323846f32 /
                              5 as libc::c_int as libc::c_float));
            (*rot).x =
                ((*rot).x as libc::c_float +
                     (0x200 as libc::c_int as libc::c_float * Rand_ZeroOne() +
                          0x400 as libc::c_int as libc::c_float) /
                         0x10 as libc::c_int as libc::c_float *
                         shakeAmp as libc::c_float * shakeMod) as s16;
            shakeMod =
                sinf((*this).timer as libc::c_int as libc::c_float *
                         (3.14159265358979323846f32 /
                              5 as libc::c_int as libc::c_float));
            (*rot).y =
                ((*rot).y as libc::c_float +
                     (0x200 as libc::c_int as libc::c_float * Rand_ZeroOne() +
                          0x400 as libc::c_int as libc::c_float) /
                         0x10 as libc::c_int as libc::c_float *
                         shakeAmp as libc::c_float * shakeMod) as s16;
            shakeMod =
                sinf(((*this).timer as libc::c_int % 5 as libc::c_int) as
                         libc::c_float *
                         (3.14159265358979323846f32 /
                              5 as libc::c_int as libc::c_float));
            (*rot).z =
                ((*rot).z as libc::c_float -
                     ((0x100 as libc::c_int as libc::c_float * Rand_ZeroOne()
                           + 0x200 as libc::c_int as libc::c_float) /
                          0x10 as libc::c_int as libc::c_float *
                          shakeAmp as libc::c_float * shakeMod +
                          0x200 as libc::c_int as libc::c_float)) as s16
        }
    } else if (*this).actionFunc ==
                  Some(BossSst_HeadDeath as
                           unsafe extern "C" fn(_: *mut BossSst,
                                                _: *mut GlobalContext) -> ())
     {
        if (*this).timer as libc::c_int > 48 as libc::c_int {
            timer12 = (*this).timer as libc::c_int - 36 as libc::c_int
        } else {
            pad =
                if (*this).timer as libc::c_int > 6 as libc::c_int {
                    6 as libc::c_int
                } else { (*this).timer as libc::c_int };
            timer12 = pad * 2 as libc::c_int
        }
        if limbIndex == 3 as libc::c_int || limbIndex == 39 as libc::c_int ||
               limbIndex == 42 as libc::c_int {
            (*rot).z =
                ((*rot).z as libc::c_float -
                     0x2000 as libc::c_int as libc::c_float *
                         sinf(timer12 as libc::c_float *
                                  (3.14159265358979323846f32 /
                                       24 as libc::c_int as libc::c_float)))
                    as s16
        } else if limbIndex == 5 as libc::c_int ||
                      limbIndex == 6 as libc::c_int {
            (*rot).z =
                ((*rot).z as libc::c_float -
                     0xa00 as libc::c_int as libc::c_float *
                         sinf(timer12 as libc::c_float *
                                  (3.14159265358979323846f32 /
                                       24 as libc::c_int as libc::c_float)))
                    as s16
        } else if limbIndex == 2 as libc::c_int {
            (*rot).z =
                ((*rot).z as libc::c_float -
                     0x400 as libc::c_int as libc::c_float *
                         sinf(timer12 as libc::c_float *
                                  (3.14159265358979323846f32 /
                                       24 as libc::c_int as libc::c_float)))
                    as s16
        }
    } else if (*this).actionFunc ==
                  Some(BossSst_HeadDarken as
                           unsafe extern "C" fn(_: *mut BossSst,
                                                _: *mut GlobalContext) -> ())
                  ||
                  (*this).actionFunc ==
                      Some(BossSst_HeadFall as
                               unsafe extern "C" fn(_: *mut BossSst,
                                                    _: *mut GlobalContext)
                                   -> ()) ||
                  (*this).actionFunc ==
                      Some(BossSst_HeadMelt as
                               unsafe extern "C" fn(_: *mut BossSst,
                                                    _: *mut GlobalContext)
                                   -> ()) {
        if limbIndex == 3 as libc::c_int || limbIndex == 39 as libc::c_int ||
               limbIndex == 42 as libc::c_int {
            (*rot).z =
                ((*rot).z as libc::c_int - 0x1000 as libc::c_int) as s16
        } else if limbIndex == 5 as libc::c_int ||
                      limbIndex == 6 as libc::c_int {
            (*rot).z = ((*rot).z as libc::c_int - 0x500 as libc::c_int) as s16
        } else if limbIndex == 2 as libc::c_int {
            (*rot).z = ((*rot).z as libc::c_int - 0x200 as libc::c_int) as s16
        }
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn BossSst_PostHeadDraw(mut globalCtx:
                                                  *mut GlobalContext,
                                              mut limbIndex: s32,
                                              mut dList: *mut *mut Gfx,
                                              mut rot: *mut Vec3s,
                                              mut thisx: *mut libc::c_void,
                                              mut gfx: *mut *mut Gfx) {
    static mut zeroVec: Vec3f =
        { let mut init = Vec3f{x: 0.0f32, y: 0.0f32, z: 0.0f32,}; init };
    static mut headVec: Vec3f =
        { let mut init = Vec3f{x: 1000.0f32, y: 0.0f32, z: 0.0f32,}; init };
    let mut this: *mut BossSst = thisx as *mut BossSst;
    let mut headPos: Vec3f = Vec3f{x: 0., y: 0., z: 0.,};
    if limbIndex == 8 as libc::c_int {
        Matrix_MultVec3f(&mut zeroVec, &mut (*this).actor.focus.pos);
        Matrix_MultVec3f(&mut headVec, &mut headPos);
        (*this).colliderCyl.dim.pos.x = headPos.x as s16;
        (*this).colliderCyl.dim.pos.y = headPos.y as s16;
        (*this).colliderCyl.dim.pos.z = headPos.z as s16
    }
    Collider_UpdateSpheres(limbIndex, &mut (*this).colliderJntSph);
}
#[no_mangle]
pub unsafe extern "C" fn BossSst_DrawHead(mut thisx: *mut Actor,
                                          mut globalCtx: *mut GlobalContext) {
    let mut pad: s32 = 0;
    let mut this: *mut BossSst = thisx as *mut BossSst;
    let mut __gfxCtx: *mut GraphicsContext = 0 as *mut GraphicsContext;
    let mut dispRefs: [*mut Gfx; 4] = [0 as *mut Gfx; 4];
    __gfxCtx = (*globalCtx).state.gfxCtx;
    Graph_OpenDisps(dispRefs.as_mut_ptr(), (*globalCtx).state.gfxCtx,
                    b"../z_boss_sst.c\x00" as *const u8 as
                        *const libc::c_char, 6810 as libc::c_int);
    if !((*this).actor.flags &
             ((1 as libc::c_int) << 7 as libc::c_int) as libc::c_uint ==
             ((1 as libc::c_int) << 7 as libc::c_int) as libc::c_uint) {
        func_80093D18((*globalCtx).state.gfxCtx);
        let fresh15 = (*__gfxCtx).polyOpa.p;
        (*__gfxCtx).polyOpa.p = (*__gfxCtx).polyOpa.p.offset(1);
        let mut _g: *mut Gfx = fresh15;
        (*_g).words.w0 =
            (0xfa as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 24 as libc::c_int
                |
                (0 as libc::c_int as u32_0 &
                     (((0x1 as libc::c_int) << 8 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    8 as libc::c_int |
                (0x80 as libc::c_int as u32_0 &
                     (((0x1 as libc::c_int) << 8 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    0 as libc::c_int;
        (*_g).words.w1 =
            (sBodyColor.r as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 24 as libc::c_int
                |
                (sBodyColor.g as u32_0 &
                     (((0x1 as libc::c_int) << 8 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    16 as libc::c_int |
                (sBodyColor.b as u32_0 &
                     (((0x1 as libc::c_int) << 8 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    8 as libc::c_int |
                (255 as libc::c_int as u32_0 &
                     (((0x1 as libc::c_int) << 8 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    0 as libc::c_int;
        if sBodyStatic == 0 {
            let fresh16 = (*__gfxCtx).polyOpa.p;
            (*__gfxCtx).polyOpa.p = (*__gfxCtx).polyOpa.p.offset(1);
            let mut _g_0: *mut Gfx = fresh16;
            (*_g_0).words.w0 =
                (0xdb as libc::c_int as u32_0 &
                     (((0x1 as libc::c_int) << 8 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    24 as libc::c_int |
                    (0x6 as libc::c_int as u32_0 &
                         (((0x1 as libc::c_int) << 8 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        16 as libc::c_int |
                    ((0x8 as libc::c_int * 4 as libc::c_int) as u32_0 &
                         (((0x1 as libc::c_int) << 16 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        0 as libc::c_int;
            (*_g_0).words.w1 =
                &mut *D_80116280.as_mut_ptr().offset(2 as libc::c_int as
                                                         isize) as *mut Gfx as
                    libc::c_uint
        } else {
            let fresh17 = (*__gfxCtx).polyOpa.p;
            (*__gfxCtx).polyOpa.p = (*__gfxCtx).polyOpa.p.offset(1);
            let mut _g_1: *mut Gfx = fresh17;
            (*_g_1).words.w0 =
                (0xfb as libc::c_int as u32_0 &
                     (((0x1 as libc::c_int) << 8 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    24 as libc::c_int;
            (*_g_1).words.w1 =
                (sStaticColor.r as u32_0 &
                     (((0x1 as libc::c_int) << 8 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    24 as libc::c_int |
                    (sStaticColor.g as u32_0 &
                         (((0x1 as libc::c_int) << 8 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        16 as libc::c_int |
                    (sStaticColor.b as u32_0 &
                         (((0x1 as libc::c_int) << 8 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        8 as libc::c_int |
                    (0 as libc::c_int as u32_0 &
                         (((0x1 as libc::c_int) << 8 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        0 as libc::c_int;
            let fresh18 = (*__gfxCtx).polyOpa.p;
            (*__gfxCtx).polyOpa.p = (*__gfxCtx).polyOpa.p.offset(1);
            let mut _g_2: *mut Gfx = fresh18;
            (*_g_2).words.w0 =
                (0xdb as libc::c_int as u32_0 &
                     (((0x1 as libc::c_int) << 8 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    24 as libc::c_int |
                    (0x6 as libc::c_int as u32_0 &
                         (((0x1 as libc::c_int) << 8 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        16 as libc::c_int |
                    ((0x8 as libc::c_int * 4 as libc::c_int) as u32_0 &
                         (((0x1 as libc::c_int) << 16 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        0 as libc::c_int;
            (*_g_2).words.w1 = sBodyStaticDList.as_mut_ptr() as libc::c_uint
        }
    } else {
        func_80093D84((*globalCtx).state.gfxCtx);
        let fresh19 = (*__gfxCtx).polyXlu.p;
        (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
        let mut _g_3: *mut Gfx = fresh19;
        (*_g_3).words.w0 =
            (0xfa as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 24 as libc::c_int
                |
                (0 as libc::c_int as u32_0 &
                     (((0x1 as libc::c_int) << 8 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    8 as libc::c_int |
                (0x80 as libc::c_int as u32_0 &
                     (((0x1 as libc::c_int) << 8 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    0 as libc::c_int;
        (*_g_3).words.w1 =
            (255 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 24 as libc::c_int
                |
                (255 as libc::c_int as u32_0 &
                     (((0x1 as libc::c_int) << 8 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    16 as libc::c_int |
                (255 as libc::c_int as u32_0 &
                     (((0x1 as libc::c_int) << 8 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    8 as libc::c_int |
                (255 as libc::c_int as u32_0 &
                     (((0x1 as libc::c_int) << 8 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    0 as libc::c_int;
        let fresh20 = (*__gfxCtx).polyXlu.p;
        (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
        let mut _g_4: *mut Gfx = fresh20;
        (*_g_4).words.w0 =
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
        (*_g_4).words.w1 =
            &mut *D_80116280.as_mut_ptr().offset(2 as libc::c_int as isize) as
                *mut Gfx as libc::c_uint
    }
    if (*this).actionFunc ==
           Some(BossSst_HeadThrash as
                    unsafe extern "C" fn(_: *mut BossSst,
                                         _: *mut GlobalContext) -> ()) {
        let mut randPitch: f32_0 =
            Rand_ZeroOne() *
                (2 as libc::c_int as libc::c_float *
                     3.14159265358979323846f32);
        let mut randYaw: f32_0 =
            Rand_ZeroOne() *
                (2 as libc::c_int as libc::c_float *
                     3.14159265358979323846f32);
        Matrix_RotateY(randYaw, MTXMODE_APPLY as libc::c_int as u8_0);
        Matrix_RotateX(randPitch, MTXMODE_APPLY as libc::c_int as u8_0);
        Matrix_Scale((*this).timer as libc::c_int as libc::c_float *
                         0.000375f32 + 1.0f32,
                     1.0f32 -
                         (*this).timer as libc::c_int as libc::c_float *
                             0.00075f32,
                     (*this).timer as libc::c_int as libc::c_float *
                         0.000375f32 + 1.0f32,
                     MTXMODE_APPLY as libc::c_int as u8_0);
        Matrix_RotateX(-randPitch, MTXMODE_APPLY as libc::c_int as u8_0);
        Matrix_RotateY(-randYaw, MTXMODE_APPLY as libc::c_int as u8_0);
    }
    if !((*this).actor.flags &
             ((1 as libc::c_int) << 7 as libc::c_int) as libc::c_uint ==
             ((1 as libc::c_int) << 7 as libc::c_int) as libc::c_uint) {
        (*__gfxCtx).polyOpa.p =
            SkelAnime_DrawFlex(globalCtx, (*this).skelAnime.skeleton,
                               (*this).skelAnime.jointTable,
                               (*this).skelAnime.dListCount as s32,
                               Some(BossSst_OverrideHeadDraw as
                                        unsafe extern "C" fn(_:
                                                                 *mut GlobalContext,
                                                             _: s32,
                                                             _: *mut *mut Gfx,
                                                             _: *mut Vec3f,
                                                             _: *mut Vec3s,
                                                             _:
                                                                 *mut libc::c_void,
                                                             _: *mut *mut Gfx)
                                            -> s32),
                               Some(BossSst_PostHeadDraw as
                                        unsafe extern "C" fn(_:
                                                                 *mut GlobalContext,
                                                             _: s32,
                                                             _: *mut *mut Gfx,
                                                             _: *mut Vec3s,
                                                             _:
                                                                 *mut libc::c_void,
                                                             _: *mut *mut Gfx)
                                            -> ()), this as *mut libc::c_void,
                               (*__gfxCtx).polyOpa.p)
    } else {
        (*__gfxCtx).polyXlu.p =
            SkelAnime_DrawFlex(globalCtx, (*this).skelAnime.skeleton,
                               (*this).skelAnime.jointTable,
                               (*this).skelAnime.dListCount as s32,
                               Some(BossSst_OverrideHeadDraw as
                                        unsafe extern "C" fn(_:
                                                                 *mut GlobalContext,
                                                             _: s32,
                                                             _: *mut *mut Gfx,
                                                             _: *mut Vec3f,
                                                             _: *mut Vec3s,
                                                             _:
                                                                 *mut libc::c_void,
                                                             _: *mut *mut Gfx)
                                            -> s32),
                               Some(BossSst_PostHeadDraw as
                                        unsafe extern "C" fn(_:
                                                                 *mut GlobalContext,
                                                             _: s32,
                                                             _: *mut *mut Gfx,
                                                             _: *mut Vec3s,
                                                             _:
                                                                 *mut libc::c_void,
                                                             _: *mut *mut Gfx)
                                            -> ()), this as *mut libc::c_void,
                               (*__gfxCtx).polyXlu.p)
    }
    if (*this).actionFunc ==
           Some(BossSst_HeadIntro as
                    unsafe extern "C" fn(_: *mut BossSst,
                                         _: *mut GlobalContext) -> ()) &&
           113 as libc::c_int >= (*this).timer as libc::c_int &&
           (*this).timer as libc::c_int > 20 as libc::c_int {
        let mut yOffset: s32 = 0;
        let mut vanishMaskPos: Vec3f = Vec3f{x: 0., y: 0., z: 0.,};
        let mut vanishMaskOffset: Vec3f = Vec3f{x: 0., y: 0., z: 0.,};
        func_80093D84((*globalCtx).state.gfxCtx);
        let fresh21 = (*__gfxCtx).polyXlu.p;
        (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
        let mut _g_5: *mut Gfx = fresh21;
        (*_g_5).words.w0 =
            (0xfa as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 24 as libc::c_int
                |
                (0 as libc::c_int as u32_0 &
                     (((0x1 as libc::c_int) << 8 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    8 as libc::c_int |
                (0 as libc::c_int as u32_0 &
                     (((0x1 as libc::c_int) << 8 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    0 as libc::c_int;
        (*_g_5).words.w1 =
            (0 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 24 as libc::c_int
                |
                (0 as libc::c_int as u32_0 &
                     (((0x1 as libc::c_int) << 8 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    16 as libc::c_int |
                (18 as libc::c_int as u32_0 &
                     (((0x1 as libc::c_int) << 8 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    8 as libc::c_int |
                (255 as libc::c_int as u32_0 &
                     (((0x1 as libc::c_int) << 8 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    0 as libc::c_int;
        yOffset =
            113 as libc::c_int * 8 as libc::c_int -
                (*this).timer as libc::c_int * 8 as libc::c_int;
        vanishMaskPos.x = -50.0f32 + 85.0f32;
        vanishMaskPos.y = 0.0f32 - 250.0f32 + yOffset as libc::c_float;
        vanishMaskPos.z = 0.0f32 + 190.0f32;
        if vanishMaskPos.y > 450.0f32 { vanishMaskPos.y = 450.0f32 }
        Matrix_MultVec3fExt(&mut vanishMaskPos, &mut vanishMaskOffset,
                            &mut (*globalCtx).billboardMtxF);
        Matrix_Translate((*this).actor.world.pos.x + vanishMaskOffset.x,
                         (*this).actor.world.pos.y + vanishMaskOffset.y,
                         (*this).actor.world.pos.z + vanishMaskOffset.z,
                         MTXMODE_NEW as libc::c_int as u8_0);
        Matrix_Scale(1.0f32, 1.0f32, 1.0f32,
                     MTXMODE_APPLY as libc::c_int as u8_0);
        let fresh22 = (*__gfxCtx).polyXlu.p;
        (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
        let mut _g_6: *mut Gfx = fresh22;
        (*_g_6).words.w0 =
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
        (*_g_6).words.w1 =
            Matrix_NewMtx((*globalCtx).state.gfxCtx,
                          b"../z_boss_sst.c\x00" as *const u8 as
                              *const libc::c_char as *mut libc::c_char,
                          6934 as libc::c_int) as libc::c_uint;
        let fresh23 = (*__gfxCtx).polyXlu.p;
        (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
        let mut _g_7: *mut Gfx = fresh23;
        (*_g_7).words.w0 =
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
        (*_g_7).words.w1 = sIntroVanishDList.as_mut_ptr() as libc::c_uint
    }
    Graph_CloseDisps(dispRefs.as_mut_ptr(), (*globalCtx).state.gfxCtx,
                     b"../z_boss_sst.c\x00" as *const u8 as
                         *const libc::c_char, 6941 as libc::c_int);
    SkinMatrix_Vec3fMtxFMultXYZ(&mut (*globalCtx).viewProjectionMtxF,
                                &mut (*this).actor.focus.pos,
                                &mut (*this).center);
    BossSst_DrawEffect(&mut (*this).actor, globalCtx);
}
#[no_mangle]
pub unsafe extern "C" fn BossSst_SpawnHeadShadow(mut this: *mut BossSst) {
    static mut shadowOffset: [Vec3f; 3] =
        [{ let mut init = Vec3f{x: 0.0f32, y: 0.0f32, z: 340.0f32,}; init },
         {
             let mut init = Vec3f{x: -160.0f32, y: 0.0f32, z: 250.0f32,};
             init
         },
         {
             let mut init = Vec3f{x: 160.0f32, y: 0.0f32, z: 250.0f32,};
             init
         }];
    let mut pad: s32 = 0;
    let mut i: s32 = 0;
    let mut sn: f32_0 = 0.;
    let mut cs: f32_0 = 0.;
    (*this).effectMode = BONGO_SHADOW as libc::c_int as u8_0;
    sn = Math_SinS((*this).actor.shape.rot.y);
    cs = Math_CosS((*this).actor.shape.rot.y);
    i = 0 as libc::c_int;
    while i < 3 as libc::c_int {
        let mut shadow: *mut BossSstEffect =
            &mut *(*this).effects.as_mut_ptr().offset(i as isize) as
                *mut BossSstEffect;
        let mut offset: *mut Vec3f =
            &mut *shadowOffset.as_mut_ptr().offset(i as isize) as *mut Vec3f;
        (*shadow).pos.x =
            (*this).actor.world.pos.x + sn * (*offset).z + cs * (*offset).x;
        (*shadow).pos.y = 0.0f32;
        (*shadow).pos.z =
            (*this).actor.world.pos.z + cs * (*offset).z - sn * (*offset).x;
        (*shadow).scale = 1450 as libc::c_int as u16_0;
        (*shadow).alpha = 254 as libc::c_int as u8_0;
        (*shadow).status = 65 as libc::c_int as s16;
        i += 1
    }
    (*this).effects[3 as libc::c_int as usize].status =
        -(1 as libc::c_int) as s16;
}
#[no_mangle]
pub unsafe extern "C" fn BossSst_SpawnHandShadow(mut this: *mut BossSst) {
    (*this).effectMode = BONGO_SHADOW as libc::c_int as u8_0;
    (*this).effects[0 as libc::c_int as usize].pos.x =
        (*this).actor.world.pos.x +
            Math_CosS((*this).actor.shape.rot.y) * 30.0f32 *
                (*this).actionVar as libc::c_int as libc::c_float;
    (*this).effects[0 as libc::c_int as usize].pos.z =
        (*this).actor.world.pos.z -
            Math_SinS((*this).actor.shape.rot.y) * 30.0f32 *
                (*this).actionVar as libc::c_int as libc::c_float;
    (*this).effects[0 as libc::c_int as usize].pos.y =
        (*this).actor.world.pos.y;
    (*this).effects[0 as libc::c_int as usize].scale =
        2300 as libc::c_int as u16_0;
    (*this).effects[0 as libc::c_int as usize].alpha =
        254 as libc::c_int as u8_0;
    (*this).effects[0 as libc::c_int as usize].status =
        5 as libc::c_int as s16;
    (*this).effects[1 as libc::c_int as usize].status =
        -(1 as libc::c_int) as s16;
}
#[no_mangle]
pub unsafe extern "C" fn BossSst_SpawnShockwave(mut this: *mut BossSst) {
    let mut i: s32 = 0;
    let mut scale: s32 = 120 as libc::c_int;
    let mut alpha: s32 = 250 as libc::c_int;
    Audio_PlayActorSound2(&mut (*this).actor, 0x3964 as libc::c_int as u16_0);
    (*this).effectMode = BONGO_SHOCKWAVE as libc::c_int as u8_0;
    i = 0 as libc::c_int;
    while i < 3 as libc::c_int {
        let mut shockwave: *mut BossSstEffect =
            &mut *(*this).effects.as_mut_ptr().offset(i as isize) as
                *mut BossSstEffect;
        Math_Vec3f_Copy(&mut (*shockwave).pos, &mut (*this).actor.world.pos);
        (*shockwave).move_0 =
            ((i + 9 as libc::c_int) * 2 as libc::c_int) as s16;
        (*shockwave).scale = scale as u16_0;
        (*shockwave).alpha =
            (alpha / (*shockwave).move_0 as libc::c_int) as u8_0;
        scale -= 50 as libc::c_int;
        alpha -= 25 as libc::c_int;
        i += 1
    };
}
#[no_mangle]
pub unsafe extern "C" fn BossSst_SpawnIceCrystal(mut this: *mut BossSst,
                                                 mut index: s32) {
    let mut ice: *mut BossSstEffect =
        &mut *(*this).effects.as_mut_ptr().offset(index as isize) as
            *mut BossSstEffect;
    let mut sphere: *mut Sphere16 = 0 as *mut Sphere16;
    if index < 11 as libc::c_int {
        sphere =
            &mut (*(*this).colliderJntSph.elements.offset(index as
                                                              isize)).dim.worldSphere;
        (*ice).pos.x = (*sphere).center.x as f32_0;
        (*ice).pos.y = (*sphere).center.y as f32_0;
        (*ice).pos.z = (*sphere).center.z as f32_0;
        if index == 0 as libc::c_int {
            (*ice).pos.x -= 25.0f32;
            (*ice).pos.y -= 25.0f32;
            (*ice).pos.z -= 25.0f32
        }
    } else {
        sphere =
            &mut (*(*this).colliderJntSph.elements.offset(0 as libc::c_int as
                                                              isize)).dim.worldSphere;
        (*ice).pos.x =
            (if index - 11 as libc::c_int & 1 as libc::c_int != 0 {
                 1 as libc::c_int
             } else { -(1 as libc::c_int) }) as libc::c_float * 25.0f32 +
                (*sphere).center.x as libc::c_int as libc::c_float;
        (*ice).pos.y =
            (if index - 11 as libc::c_int & 2 as libc::c_int != 0 {
                 1 as libc::c_int
             } else { -(1 as libc::c_int) }) as libc::c_float * 25.0f32 +
                (*sphere).center.y as libc::c_int as libc::c_float;
        (*ice).pos.z =
            (if index - 11 as libc::c_int & 4 as libc::c_int != 0 {
                 1 as libc::c_int
             } else { -(1 as libc::c_int) }) as libc::c_float * 25.0f32 +
                (*sphere).center.z as libc::c_int as libc::c_float
    }
    (*ice).pos.x -= (*this).actor.world.pos.x;
    (*ice).pos.y -= (*this).actor.world.pos.y;
    (*ice).pos.z -= (*this).actor.world.pos.z;
    (*ice).status = 0 as libc::c_int as s16;
    (*ice).rot.x =
        (Rand_ZeroOne() * 0x10000 as libc::c_int as libc::c_float) as s16;
    (*ice).rot.y =
        (Rand_ZeroOne() * 0x10000 as libc::c_int as libc::c_float) as s16;
    (*ice).rot.z =
        (Rand_ZeroOne() * 0x10000 as libc::c_int as libc::c_float) as s16;
    (*ice).alpha = 120 as libc::c_int as u8_0;
    (*ice).move_0 = 1 as libc::c_int as s16;
    (*ice).vel.x = (Rand_ZeroOne() * 0.06f32 + 0.12f32) * (*ice).pos.x;
    (*ice).vel.y = Rand_ZeroOne() * 15.0f32 + 5.0f32;
    (*ice).vel.z = (Rand_ZeroOne() * 0.06f32 + 0.12f32) * (*ice).pos.z;
    (*ice).scale = 4000 as libc::c_int as u16_0;
    if index % 2 as libc::c_int == 0 as libc::c_int {
        Audio_PlayActorSound2(&mut (*this).actor,
                              0x874 as libc::c_int as u16_0);
    };
}
#[no_mangle]
pub unsafe extern "C" fn BossSst_SpawnIceShard(mut this: *mut BossSst) {
    let mut i: s32 = 0;
    let mut spawnPos: Vec3f = Vec3f{x: 0., y: 0., z: 0.,};
    let mut offXZ: f32_0 = 0.;
    (*this).effectMode = BONGO_ICE as libc::c_int as u8_0;
    offXZ = Math_CosS((*this).actor.shape.rot.x) * 50.0f32;
    spawnPos.x =
        Math_CosS((*this).actor.shape.rot.y) * offXZ +
            (*this).actor.world.pos.x;
    spawnPos.y =
        Math_SinS((*this).actor.shape.rot.x) * 50.0f32 +
            (*this).actor.world.pos.y - 10.0f32;
    spawnPos.z =
        Math_SinS((*this).actor.shape.rot.y) * offXZ +
            (*this).actor.world.pos.z;
    i = 0 as libc::c_int;
    while i < 18 as libc::c_int {
        let mut ice: *mut BossSstEffect =
            &mut *(*this).effects.as_mut_ptr().offset(i as isize) as
                *mut BossSstEffect;
        Math_Vec3f_Copy(&mut (*ice).pos, &mut spawnPos);
        (*ice).status = 1 as libc::c_int as s16;
        (*ice).rot.x =
            (Rand_ZeroOne() * 0x10000 as libc::c_int as libc::c_float) as s16;
        (*ice).rot.y =
            (Rand_ZeroOne() * 0x10000 as libc::c_int as libc::c_float) as s16;
        (*ice).rot.z =
            (Rand_ZeroOne() * 0x10000 as libc::c_int as libc::c_float) as s16;
        (*ice).alpha = 120 as libc::c_int as u8_0;
        (*ice).move_0 = 1 as libc::c_int as s16;
        (*ice).vel.x = Rand_CenteredFloat(20.0f32);
        (*ice).vel.y = Rand_ZeroOne() * 10.0f32 + 3.0f32;
        (*ice).vel.z = Rand_CenteredFloat(20.0f32);
        (*ice).scale = (Rand_ZeroOne() * 200.0f32 + 400.0f32) as u16_0;
        i += 1
    };
}
#[no_mangle]
pub unsafe extern "C" fn BossSst_IceShatter(mut this: *mut BossSst) {
    let mut i: s32 = 0;
    (*this).effects[0 as libc::c_int as usize].status =
        1 as libc::c_int as s16;
    Audio_PlayActorSound2(&mut (*this).actor, 0x875 as libc::c_int as u16_0);
    i = 0 as libc::c_int;
    while i < 18 as libc::c_int {
        let mut ice: *mut BossSstEffect =
            &mut *(*this).effects.as_mut_ptr().offset(i as isize) as
                *mut BossSstEffect;
        if (*ice).move_0 != 0 {
            (*ice).pos.x += (*this).actor.world.pos.x;
            (*ice).pos.y += (*this).actor.world.pos.y;
            (*ice).pos.z += (*this).actor.world.pos.z
        }
        i += 1
    };
}
#[no_mangle]
pub unsafe extern "C" fn BossSst_UpdateEffect(mut thisx: *mut Actor,
                                              mut globalCtx:
                                                  *mut GlobalContext) {
    let mut this: *mut BossSst = thisx as *mut BossSst;
    let mut effect: *mut BossSstEffect = 0 as *mut BossSstEffect;
    let mut i: s32 = 0;
    if (*this).effectMode as libc::c_int != BONGO_NULL as libc::c_int {
        if (*this).effectMode as libc::c_int == BONGO_ICE as libc::c_int {
            if (*this).effects[0 as libc::c_int as usize].status != 0 {
                i = 0 as libc::c_int;
                while i < 18 as libc::c_int {
                    effect =
                        &mut *(*this).effects.as_mut_ptr().offset(i as isize)
                            as *mut BossSstEffect;
                    if (*effect).move_0 != 0 {
                        (*effect).pos.x += (*effect).vel.x;
                        (*effect).pos.y += (*effect).vel.y;
                        (*effect).pos.z += (*effect).vel.z;
                        (*effect).alpha =
                            ((*effect).alpha as libc::c_int -
                                 3 as libc::c_int) as u8_0;
                        (*effect).vel.y -= 1.0f32;
                        (*effect).rot.x =
                            ((*effect).rot.x as libc::c_int +
                                 0xd00 as libc::c_int) as s16;
                        (*effect).rot.y =
                            ((*effect).rot.y as libc::c_int +
                                 0x1100 as libc::c_int) as s16;
                        (*effect).rot.z =
                            ((*effect).rot.z as libc::c_int +
                                 0x1500 as libc::c_int) as s16
                    }
                    i += 1
                }
            }
            if (*this).effects[0 as libc::c_int as usize].alpha as libc::c_int
                   == 0 as libc::c_int {
                (*this).effectMode = BONGO_NULL as libc::c_int as u8_0
            }
        } else if (*this).effectMode as libc::c_int ==
                      BONGO_SHOCKWAVE as libc::c_int {
            i = 0 as libc::c_int;
            while i < 3 as libc::c_int {
                let mut effect2: *mut BossSstEffect =
                    &mut *(*this).effects.as_mut_ptr().offset(i as isize) as
                        *mut BossSstEffect;
                let mut scale: s32 =
                    (*effect2).move_0 as libc::c_int * 2 as libc::c_int;
                (*effect2).scale =
                    ((*effect2).scale as libc::c_int +
                         ((if scale > 20 as libc::c_int {
                               20 as libc::c_int
                           } else { scale }) + i)) as u16_0;
                if (*effect2).move_0 as libc::c_int != 0 as libc::c_int {
                    (*effect2).move_0 -= 1
                }
                i += 1
            }
            if (*this).effects[0 as libc::c_int as usize].move_0 as
                   libc::c_int == 0 as libc::c_int {
                (*this).effectMode = BONGO_NULL as libc::c_int as u8_0
            }
        } else if (*this).effectMode as libc::c_int ==
                      BONGO_SHADOW as libc::c_int {
            effect =
                &mut *(*this).effects.as_mut_ptr().offset(0 as libc::c_int as
                                                              isize) as
                    *mut BossSstEffect;
            if (*this).actor.params as libc::c_int ==
                   BONGO_HEAD as libc::c_int {
                SkinMatrix_Vec3fMtxFMultXYZ(&mut (*globalCtx).viewProjectionMtxF,
                                            &mut (*this).actor.focus.pos,
                                            &mut (*this).center);
                BossSst_HeadSfx(this,
                                (0x398c as libc::c_int - 0x800 as libc::c_int)
                                    as u16_0);
            }
            while (*effect).status as libc::c_int != -(1 as libc::c_int) {
                if (*effect).status as libc::c_int == 0 as libc::c_int {
                    (*effect).alpha =
                        ((*effect).alpha as libc::c_int - 2 as libc::c_int) as
                            u8_0
                } else {
                    (*effect).scale =
                        ((*effect).scale as libc::c_int +
                             (*effect).status as libc::c_int) as u16_0
                }
                (*effect).scale =
                    if (*effect).scale as libc::c_int > 10000 as libc::c_int {
                        10000 as libc::c_int
                    } else { (*effect).scale as libc::c_int } as u16_0;
                effect = effect.offset(1)
            }
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn BossSst_DrawEffect(mut thisx: *mut Actor,
                                            mut globalCtx:
                                                *mut GlobalContext) {
    let mut pad: s32 = 0;
    let mut this: *mut BossSst = thisx as *mut BossSst;
    let mut i: s32 = 0;
    let mut effect: *mut BossSstEffect = 0 as *mut BossSstEffect;
    if (*this).effectMode as libc::c_int != BONGO_NULL as libc::c_int {
        let mut __gfxCtx: *mut GraphicsContext = 0 as *mut GraphicsContext;
        let mut dispRefs: [*mut Gfx; 4] = [0 as *mut Gfx; 4];
        __gfxCtx = (*globalCtx).state.gfxCtx;
        Graph_OpenDisps(dispRefs.as_mut_ptr(), (*globalCtx).state.gfxCtx,
                        b"../z_boss_sst.c\x00" as *const u8 as
                            *const libc::c_char, 7302 as libc::c_int);
        func_80093D84((*globalCtx).state.gfxCtx);
        if (*this).effectMode as libc::c_int == BONGO_ICE as libc::c_int {
            let fresh24 = (*__gfxCtx).polyXlu.p;
            (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
            let mut _g: *mut Gfx = fresh24;
            (*_g).words.w0 =
                (0xdb as libc::c_int as u32_0 &
                     (((0x1 as libc::c_int) << 8 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    24 as libc::c_int |
                    (0x6 as libc::c_int as u32_0 &
                         (((0x1 as libc::c_int) << 8 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        16 as libc::c_int |
                    ((0x8 as libc::c_int * 4 as libc::c_int) as u32_0 &
                         (((0x1 as libc::c_int) << 16 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        0 as libc::c_int;
            (*_g).words.w1 =
                Gfx_TwoTexScroll((*globalCtx).state.gfxCtx, 0 as libc::c_int,
                                 0 as libc::c_int as u32_0,
                                 (*globalCtx).gameplayFrames.wrapping_rem(256
                                                                              as
                                                                              libc::c_int
                                                                              as
                                                                              libc::c_uint),
                                 0x20 as libc::c_int, 0x10 as libc::c_int,
                                 1 as libc::c_int, 0 as libc::c_int as u32_0,
                                 (*globalCtx).gameplayFrames.wrapping_mul(2 as
                                                                              libc::c_int
                                                                              as
                                                                              libc::c_uint).wrapping_rem(256
                                                                                                             as
                                                                                                             libc::c_int
                                                                                                             as
                                                                                                             libc::c_uint),
                                 0x40 as libc::c_int, 0x20 as libc::c_int) as
                    libc::c_uint;
            let fresh25 = (*__gfxCtx).polyXlu.p;
            (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
            let mut _g_0: *mut Gfx = fresh25;
            (*_g_0).words.w0 =
                (0xfb as libc::c_int as u32_0 &
                     (((0x1 as libc::c_int) << 8 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    24 as libc::c_int;
            (*_g_0).words.w1 =
                (0 as libc::c_int as u32_0 &
                     (((0x1 as libc::c_int) << 8 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    24 as libc::c_int |
                    (50 as libc::c_int as u32_0 &
                         (((0x1 as libc::c_int) << 8 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        16 as libc::c_int |
                    (100 as libc::c_int as u32_0 &
                         (((0x1 as libc::c_int) << 8 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        8 as libc::c_int |
                    ((*this).effects[0 as libc::c_int as usize].alpha as u32_0
                         &
                         (((0x1 as libc::c_int) << 8 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        0 as libc::c_int;
            let fresh26 = (*__gfxCtx).polyXlu.p;
            (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
            let mut _g_1: *mut Gfx = fresh26;
            (*_g_1).words.w0 =
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
            (*_g_1).words.w1 =
                gBongoIceCrystalDL.as_mut_ptr() as libc::c_uint;
            i = 0 as libc::c_int;
            while i < 18 as libc::c_int {
                effect =
                    &mut *(*this).effects.as_mut_ptr().offset(i as isize) as
                        *mut BossSstEffect;
                if (*effect).move_0 != 0 {
                    func_8003435C(&mut (*effect).pos, globalCtx);
                    if (*this).effects[0 as libc::c_int as usize].status as
                           libc::c_int != 0 as libc::c_int {
                        Matrix_Translate((*effect).pos.x, (*effect).pos.y,
                                         (*effect).pos.z,
                                         MTXMODE_NEW as libc::c_int as u8_0);
                    } else {
                        Matrix_Translate((*effect).pos.x +
                                             (*this).actor.world.pos.x,
                                         (*effect).pos.y +
                                             (*this).actor.world.pos.y,
                                         (*effect).pos.z +
                                             (*this).actor.world.pos.z,
                                         MTXMODE_NEW as libc::c_int as u8_0);
                    }
                    Matrix_RotateZYX((*effect).rot.x, (*effect).rot.y,
                                     (*effect).rot.z,
                                     MTXMODE_APPLY as libc::c_int as u8_0);
                    Matrix_Scale((*effect).scale as libc::c_int as
                                     libc::c_float * 0.001f32,
                                 (*effect).scale as libc::c_int as
                                     libc::c_float * 0.001f32,
                                 (*effect).scale as libc::c_int as
                                     libc::c_float * 0.001f32,
                                 MTXMODE_APPLY as libc::c_int as u8_0);
                    let fresh27 = (*__gfxCtx).polyXlu.p;
                    (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
                    let mut _g_2: *mut Gfx = fresh27;
                    (*_g_2).words.w0 =
                        (0xda as libc::c_int as u32_0 &
                             (((0x1 as libc::c_int) << 8 as libc::c_int) -
                                  1 as libc::c_int) as libc::c_uint) <<
                            24 as libc::c_int |
                            ((::std::mem::size_of::<Mtx>() as
                                  libc::c_ulong).wrapping_sub(1 as libc::c_int
                                                                  as
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
                    (*_g_2).words.w1 =
                        Matrix_NewMtx((*globalCtx).state.gfxCtx,
                                      b"../z_boss_sst.c\x00" as *const u8 as
                                          *const libc::c_char as
                                          *mut libc::c_char,
                                      7350 as libc::c_int) as libc::c_uint;
                    let fresh28 = (*__gfxCtx).polyXlu.p;
                    (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
                    let mut _g_3: *mut Gfx = fresh28;
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
                                 (((0x1 as libc::c_int) << 16 as libc::c_int)
                                      - 1 as libc::c_int) as libc::c_uint) <<
                                0 as libc::c_int;
                    (*_g_3).words.w1 =
                        gBongoIceShardDL.as_mut_ptr() as libc::c_uint
                }
                i += 1
            }
        } else if (*this).effectMode as libc::c_int ==
                      BONGO_SHOCKWAVE as libc::c_int {
            let mut scaleY: f32_0 = 0.005f32;
            let fresh29 = (*__gfxCtx).polyXlu.p;
            (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
            let mut _g_4: *mut Gfx = fresh29;
            (*_g_4).words.w0 =
                (0xe7 as libc::c_int as u32_0 &
                     (((0x1 as libc::c_int) << 8 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    24 as libc::c_int;
            (*_g_4).words.w1 = 0 as libc::c_int as libc::c_uint;
            let fresh30 = (*__gfxCtx).polyXlu.p;
            (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
            let mut _g_5: *mut Gfx = fresh30;
            (*_g_5).words.w0 =
                (0xdb as libc::c_int as u32_0 &
                     (((0x1 as libc::c_int) << 8 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    24 as libc::c_int |
                    (0x6 as libc::c_int as u32_0 &
                         (((0x1 as libc::c_int) << 8 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        16 as libc::c_int |
                    ((0x8 as libc::c_int * 4 as libc::c_int) as u32_0 &
                         (((0x1 as libc::c_int) << 16 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        0 as libc::c_int;
            (*_g_5).words.w1 =
                Gfx_TwoTexScroll((*globalCtx).state.gfxCtx, 0 as libc::c_int,
                                 (*globalCtx).gameplayFrames.wrapping_rem(128
                                                                              as
                                                                              libc::c_int
                                                                              as
                                                                              libc::c_uint),
                                 0 as libc::c_int as u32_0,
                                 0x20 as libc::c_int, 0x40 as libc::c_int,
                                 1 as libc::c_int, 0 as libc::c_int as u32_0,
                                 (*globalCtx).gameplayFrames.wrapping_mul(-(15
                                                                                as
                                                                                libc::c_int)
                                                                              as
                                                                              libc::c_uint).wrapping_rem(256
                                                                                                             as
                                                                                                             libc::c_int
                                                                                                             as
                                                                                                             libc::c_uint),
                                 0x20 as libc::c_int, 0x40 as libc::c_int) as
                    libc::c_uint;
            i = 0 as libc::c_int;
            while i < 3 as libc::c_int {
                effect =
                    &mut *(*this).effects.as_mut_ptr().offset(i as isize) as
                        *mut BossSstEffect;
                if (*effect).move_0 as libc::c_int != 0 as libc::c_int {
                    Matrix_Translate((*effect).pos.x, (*effect).pos.y,
                                     (*effect).pos.z,
                                     MTXMODE_NEW as libc::c_int as u8_0);
                    Matrix_Scale((*effect).scale as libc::c_int as
                                     libc::c_float * 0.001f32, scaleY,
                                 (*effect).scale as libc::c_int as
                                     libc::c_float * 0.001f32,
                                 MTXMODE_APPLY as libc::c_int as u8_0);
                    let fresh31 = (*__gfxCtx).polyXlu.p;
                    (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
                    let mut _g_6: *mut Gfx = fresh31;
                    (*_g_6).words.w0 =
                        (0xe7 as libc::c_int as u32_0 &
                             (((0x1 as libc::c_int) << 8 as libc::c_int) -
                                  1 as libc::c_int) as libc::c_uint) <<
                            24 as libc::c_int;
                    (*_g_6).words.w1 = 0 as libc::c_int as libc::c_uint;
                    let fresh32 = (*__gfxCtx).polyXlu.p;
                    (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
                    let mut _g_7: *mut Gfx = fresh32;
                    (*_g_7).words.w0 =
                        (0xfa as libc::c_int as u32_0 &
                             (((0x1 as libc::c_int) << 8 as libc::c_int) -
                                  1 as libc::c_int) as libc::c_uint) <<
                            24 as libc::c_int |
                            (0x80 as libc::c_int as u32_0 &
                                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                                      1 as libc::c_int) as libc::c_uint) <<
                                8 as libc::c_int |
                            (0x80 as libc::c_int as u32_0 &
                                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                                      1 as libc::c_int) as libc::c_uint) <<
                                0 as libc::c_int;
                    (*_g_7).words.w1 =
                        (30 as libc::c_int as u32_0 &
                             (((0x1 as libc::c_int) << 8 as libc::c_int) -
                                  1 as libc::c_int) as libc::c_uint) <<
                            24 as libc::c_int |
                            (0 as libc::c_int as u32_0 &
                                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                                      1 as libc::c_int) as libc::c_uint) <<
                                16 as libc::c_int |
                            (30 as libc::c_int as u32_0 &
                                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                                      1 as libc::c_int) as libc::c_uint) <<
                                8 as libc::c_int |
                            (((*effect).alpha as libc::c_int *
                                  (*effect).move_0 as libc::c_int) as u32_0 &
                                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                                      1 as libc::c_int) as libc::c_uint) <<
                                0 as libc::c_int;
                    let fresh33 = (*__gfxCtx).polyXlu.p;
                    (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
                    let mut _g_8: *mut Gfx = fresh33;
                    (*_g_8).words.w0 =
                        (0xfb as libc::c_int as u32_0 &
                             (((0x1 as libc::c_int) << 8 as libc::c_int) -
                                  1 as libc::c_int) as libc::c_uint) <<
                            24 as libc::c_int;
                    (*_g_8).words.w1 =
                        (30 as libc::c_int as u32_0 &
                             (((0x1 as libc::c_int) << 8 as libc::c_int) -
                                  1 as libc::c_int) as libc::c_uint) <<
                            24 as libc::c_int |
                            (0 as libc::c_int as u32_0 &
                                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                                      1 as libc::c_int) as libc::c_uint) <<
                                16 as libc::c_int |
                            (30 as libc::c_int as u32_0 &
                                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                                      1 as libc::c_int) as libc::c_uint) <<
                                8 as libc::c_int |
                            (0 as libc::c_int as u32_0 &
                                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                                      1 as libc::c_int) as libc::c_uint) <<
                                0 as libc::c_int;
                    let fresh34 = (*__gfxCtx).polyXlu.p;
                    (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
                    let mut _g_9: *mut Gfx = fresh34;
                    (*_g_9).words.w0 =
                        (0xda as libc::c_int as u32_0 &
                             (((0x1 as libc::c_int) << 8 as libc::c_int) -
                                  1 as libc::c_int) as libc::c_uint) <<
                            24 as libc::c_int |
                            ((::std::mem::size_of::<Mtx>() as
                                  libc::c_ulong).wrapping_sub(1 as libc::c_int
                                                                  as
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
                    (*_g_9).words.w1 =
                        Matrix_NewMtx((*globalCtx).state.gfxCtx,
                                      b"../z_boss_sst.c\x00" as *const u8 as
                                          *const libc::c_char as
                                          *mut libc::c_char,
                                      7396 as libc::c_int) as libc::c_uint;
                    let fresh35 = (*__gfxCtx).polyXlu.p;
                    (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
                    let mut _g_10: *mut Gfx = fresh35;
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
                                 (((0x1 as libc::c_int) << 16 as libc::c_int)
                                      - 1 as libc::c_int) as libc::c_uint) <<
                                0 as libc::c_int;
                    (*_g_10).words.w1 =
                        gEffFireCircleDL.as_mut_ptr() as libc::c_uint
                }
                i += 1;
                scaleY -= 0.001f32
            }
        } else if (*this).effectMode as libc::c_int ==
                      BONGO_SHADOW as libc::c_int {
            let fresh36 = (*__gfxCtx).polyXlu.p;
            (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
            let mut _g_11: *mut Gfx = fresh36;
            (*_g_11).words.w0 =
                (0xfa as libc::c_int as u32_0 &
                     (((0x1 as libc::c_int) << 8 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    24 as libc::c_int |
                    (0 as libc::c_int as u32_0 &
                         (((0x1 as libc::c_int) << 8 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        8 as libc::c_int |
                    (0x80 as libc::c_int as u32_0 &
                         (((0x1 as libc::c_int) << 8 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        0 as libc::c_int;
            (*_g_11).words.w1 =
                (10 as libc::c_int as u32_0 &
                     (((0x1 as libc::c_int) << 8 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    24 as libc::c_int |
                    (10 as libc::c_int as u32_0 &
                         (((0x1 as libc::c_int) << 8 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        16 as libc::c_int |
                    (80 as libc::c_int as u32_0 &
                         (((0x1 as libc::c_int) << 8 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        8 as libc::c_int |
                    (0 as libc::c_int as u32_0 &
                         (((0x1 as libc::c_int) << 8 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        0 as libc::c_int;
            let fresh37 = (*__gfxCtx).polyXlu.p;
            (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
            let mut _g_12: *mut Gfx = fresh37;
            (*_g_12).words.w0 =
                (0xfb as libc::c_int as u32_0 &
                     (((0x1 as libc::c_int) << 8 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    24 as libc::c_int;
            (*_g_12).words.w1 =
                (10 as libc::c_int as u32_0 &
                     (((0x1 as libc::c_int) << 8 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    24 as libc::c_int |
                    (10 as libc::c_int as u32_0 &
                         (((0x1 as libc::c_int) << 8 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        16 as libc::c_int |
                    (10 as libc::c_int as u32_0 &
                         (((0x1 as libc::c_int) << 8 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        8 as libc::c_int |
                    ((*this).effects[0 as libc::c_int as usize].alpha as u32_0
                         &
                         (((0x1 as libc::c_int) << 8 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        0 as libc::c_int;
            effect =
                &mut *(*this).effects.as_mut_ptr().offset(0 as libc::c_int as
                                                              isize) as
                    *mut BossSstEffect;
            while (*effect).status as libc::c_int != -(1 as libc::c_int) {
                Matrix_Translate((*effect).pos.x, (*effect).pos.y,
                                 (*effect).pos.z,
                                 MTXMODE_NEW as libc::c_int as u8_0);
                Matrix_Scale((*effect).scale as libc::c_int as libc::c_float *
                                 0.001f32, 1.0f32,
                             (*effect).scale as libc::c_int as libc::c_float *
                                 0.001f32,
                             MTXMODE_APPLY as libc::c_int as u8_0);
                let fresh38 = (*__gfxCtx).polyXlu.p;
                (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
                let mut _g_13: *mut Gfx = fresh38;
                (*_g_13).words.w0 =
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
                (*_g_13).words.w1 =
                    Matrix_NewMtx((*globalCtx).state.gfxCtx,
                                  b"../z_boss_sst.c\x00" as *const u8 as
                                      *const libc::c_char as
                                      *mut libc::c_char, 7423 as libc::c_int)
                        as libc::c_uint;
                let fresh39 = (*__gfxCtx).polyXlu.p;
                (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
                let mut _g_14: *mut Gfx = fresh39;
                (*_g_14).words.w0 =
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
                (*_g_14).words.w1 = sShadowDList.as_mut_ptr() as libc::c_uint;
                effect = effect.offset(1)
            }
        }
        Graph_CloseDisps(dispRefs.as_mut_ptr(), (*globalCtx).state.gfxCtx,
                         b"../z_boss_sst.c\x00" as *const u8 as
                             *const libc::c_char, 7433 as libc::c_int);
    };
}
unsafe extern "C" fn run_static_initializers() {
    sIntroVanishDList =
        [Gfx{words:
                 {
                     let mut init =
                         Gwords{w0:
                                    (0xe7 as libc::c_int as u32_0 &
                                         (((0x1 as libc::c_int) <<
                                               8 as libc::c_int) -
                                              1 as libc::c_int) as
                                             libc::c_uint) <<
                                        24 as libc::c_int,
                                w1: 0 as libc::c_int as libc::c_uint,};
                     init
                 },},
         Gfx{words:
                 {
                     let mut init =
                         Gwords{w0:
                                    (0xda as libc::c_int as u32_0 &
                                         (((0x1 as libc::c_int) <<
                                               8 as libc::c_int) -
                                              1 as libc::c_int) as
                                             libc::c_uint) <<
                                        24 as libc::c_int |
                                        ((::std::mem::size_of::<Mtx>() as
                                              libc::c_ulong).wrapping_sub(1 as
                                                                              libc::c_int
                                                                              as
                                                                              libc::c_uint).wrapping_div(8
                                                                                                             as
                                                                                                             libc::c_int
                                                                                                             as
                                                                                                             libc::c_uint)
                                             &
                                             (((0x1 as libc::c_int) <<
                                                   5 as libc::c_int) -
                                                  1 as libc::c_int) as
                                                 libc::c_uint) <<
                                            19 as libc::c_int |
                                        ((0 as libc::c_int / 8 as libc::c_int)
                                             as u32_0 &
                                             (((0x1 as libc::c_int) <<
                                                   8 as libc::c_int) -
                                                  1 as libc::c_int) as
                                                 libc::c_uint) <<
                                            8 as libc::c_int |
                                        (((0 as libc::c_int | 0 as libc::c_int
                                               | 0 as libc::c_int) ^
                                              0x1 as libc::c_int) as u32_0 &
                                             (((0x1 as libc::c_int) <<
                                                   8 as libc::c_int) -
                                                  1 as libc::c_int) as
                                                 libc::c_uint) <<
                                            0 as libc::c_int,
                                w1:
                                    0x1000000 as libc::c_int as
                                        libc::c_uint,};
                     init
                 },},
         Gfx{words:
                 {
                     let mut init =
                         Gwords{w0:
                                    (0xd7 as libc::c_int as u32_0 &
                                         (((0x1 as libc::c_int) <<
                                               8 as libc::c_int) -
                                              1 as libc::c_int) as
                                             libc::c_uint) <<
                                        24 as libc::c_int |
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
                                                 libc::c_uint) <<
                                            8 as libc::c_int |
                                        (1 as libc::c_int as u32_0 &
                                             (((0x1 as libc::c_int) <<
                                                   7 as libc::c_int) -
                                                  1 as libc::c_int) as
                                                 libc::c_uint) <<
                                            1 as libc::c_int,
                                w1:
                                    (0xffff as libc::c_int as u32_0 &
                                         (((0x1 as libc::c_int) <<
                                               16 as libc::c_int) -
                                              1 as libc::c_int) as
                                             libc::c_uint) <<
                                        16 as libc::c_int |
                                        (0xffff as libc::c_int as u32_0 &
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
                                    (0xfc as libc::c_int as u32_0 &
                                         (((0x1 as libc::c_int) <<
                                               8 as libc::c_int) -
                                              1 as libc::c_int) as
                                             libc::c_uint) <<
                                        24 as libc::c_int |
                                        (((31 as libc::c_int as u32_0 &
                                               (((0x1 as libc::c_int) <<
                                                     4 as libc::c_int) -
                                                    1 as libc::c_int) as
                                                   libc::c_uint) <<
                                              20 as libc::c_int |
                                              (31 as libc::c_int as u32_0 &
                                                   (((0x1 as libc::c_int) <<
                                                         5 as libc::c_int) -
                                                        1 as libc::c_int) as
                                                       libc::c_uint) <<
                                                  15 as libc::c_int |
                                              (3 as libc::c_int as u32_0 &
                                                   (((0x1 as libc::c_int) <<
                                                         3 as libc::c_int) -
                                                        1 as libc::c_int) as
                                                       libc::c_uint) <<
                                                  12 as libc::c_int |
                                              (1 as libc::c_int as u32_0 &
                                                   (((0x1 as libc::c_int) <<
                                                         3 as libc::c_int) -
                                                        1 as libc::c_int) as
                                                       libc::c_uint) <<
                                                  9 as libc::c_int |
                                              ((31 as libc::c_int as u32_0 &
                                                    (((0x1 as libc::c_int) <<
                                                          4 as libc::c_int) -
                                                         1 as libc::c_int) as
                                                        libc::c_uint) <<
                                                   5 as libc::c_int |
                                                   (31 as libc::c_int as u32_0
                                                        &
                                                        (((0x1 as libc::c_int)
                                                              <<
                                                              5 as
                                                                  libc::c_int)
                                                             -
                                                             1 as libc::c_int)
                                                            as libc::c_uint)
                                                       << 0 as libc::c_int)) &
                                             (((0x1 as libc::c_int) <<
                                                   24 as libc::c_int) -
                                                  1 as libc::c_int) as
                                                 libc::c_uint) <<
                                            0 as libc::c_int,
                                w1:
                                    (31 as libc::c_int as u32_0 &
                                         (((0x1 as libc::c_int) <<
                                               4 as libc::c_int) -
                                              1 as libc::c_int) as
                                             libc::c_uint) <<
                                        28 as libc::c_int |
                                        (31 as libc::c_int as u32_0 &
                                             (((0x1 as libc::c_int) <<
                                                   3 as libc::c_int) -
                                                  1 as libc::c_int) as
                                                 libc::c_uint) <<
                                            15 as libc::c_int |
                                        (7 as libc::c_int as u32_0 &
                                             (((0x1 as libc::c_int) <<
                                                   3 as libc::c_int) -
                                                  1 as libc::c_int) as
                                                 libc::c_uint) <<
                                            12 as libc::c_int |
                                        (7 as libc::c_int as u32_0 &
                                             (((0x1 as libc::c_int) <<
                                                   3 as libc::c_int) -
                                                  1 as libc::c_int) as
                                                 libc::c_uint) <<
                                            9 as libc::c_int |
                                        ((31 as libc::c_int as u32_0 &
                                              (((0x1 as libc::c_int) <<
                                                    4 as libc::c_int) -
                                                   1 as libc::c_int) as
                                                  libc::c_uint) <<
                                             24 as libc::c_int |
                                             (7 as libc::c_int as u32_0 &
                                                  (((0x1 as libc::c_int) <<
                                                        3 as libc::c_int) -
                                                       1 as libc::c_int) as
                                                      libc::c_uint) <<
                                                 21 as libc::c_int |
                                             (7 as libc::c_int as u32_0 &
                                                  (((0x1 as libc::c_int) <<
                                                        3 as libc::c_int) -
                                                       1 as libc::c_int) as
                                                      libc::c_uint) <<
                                                 18 as libc::c_int |
                                             (3 as libc::c_int as u32_0 &
                                                  (((0x1 as libc::c_int) <<
                                                        3 as libc::c_int) -
                                                       1 as libc::c_int) as
                                                      libc::c_uint) <<
                                                 6 as libc::c_int |
                                             (7 as libc::c_int as u32_0 &
                                                  (((0x1 as libc::c_int) <<
                                                        3 as libc::c_int) -
                                                       1 as libc::c_int) as
                                                      libc::c_uint) <<
                                                 3 as libc::c_int |
                                             (0 as libc::c_int as u32_0 &
                                                  (((0x1 as libc::c_int) <<
                                                        3 as libc::c_int) -
                                                       1 as libc::c_int) as
                                                      libc::c_uint) <<
                                                 0 as libc::c_int),};
                     init
                 },},
         Gfx{words:
                 {
                     let mut init =
                         Gwords{w0:
                                    (0xe2 as libc::c_int as u32_0 &
                                         (((0x1 as libc::c_int) <<
                                               8 as libc::c_int) -
                                              1 as libc::c_int) as
                                             libc::c_uint) <<
                                        24 as libc::c_int |
                                        ((32 as libc::c_int - 3 as libc::c_int
                                              - 29 as libc::c_int) as u32_0 &
                                             (((0x1 as libc::c_int) <<
                                                   8 as libc::c_int) -
                                                  1 as libc::c_int) as
                                                 libc::c_uint) <<
                                            8 as libc::c_int |
                                        ((29 as libc::c_int -
                                              1 as libc::c_int) as u32_0 &
                                             (((0x1 as libc::c_int) <<
                                                   8 as libc::c_int) -
                                                  1 as libc::c_int) as
                                                 libc::c_uint) <<
                                            0 as libc::c_int,
                                w1:
                                    (0x10 as libc::c_int | 0x20 as libc::c_int
                                         | 0x40 as libc::c_int |
                                         0x200 as libc::c_int |
                                         0x800 as libc::c_int |
                                         0x4000 as libc::c_int |
                                         (0 as libc::c_int) <<
                                             30 as libc::c_int |
                                         (3 as libc::c_int) <<
                                             26 as libc::c_int |
                                         (0 as libc::c_int) <<
                                             22 as libc::c_int |
                                         (2 as libc::c_int) <<
                                             18 as libc::c_int |
                                         (0x10 as libc::c_int |
                                              0x20 as libc::c_int |
                                              0x40 as libc::c_int |
                                              0x200 as libc::c_int |
                                              0x800 as libc::c_int |
                                              0x4000 as libc::c_int |
                                              (0 as libc::c_int) <<
                                                  28 as libc::c_int |
                                              (0 as libc::c_int) <<
                                                  24 as libc::c_int |
                                              (1 as libc::c_int) <<
                                                  20 as libc::c_int |
                                              (0 as libc::c_int) <<
                                                  16 as libc::c_int)) as
                                        libc::c_uint,};
                     init
                 },},
         Gfx{words:
                 {
                     let mut init =
                         Gwords{w0:
                                    (0xd9 as libc::c_int as u32_0 &
                                         (((0x1 as libc::c_int) <<
                                               8 as libc::c_int) -
                                              1 as libc::c_int) as
                                             libc::c_uint) <<
                                        24 as libc::c_int |
                                        (!((0x400 as libc::c_int |
                                                0x10000 as libc::c_int |
                                                0x20000 as libc::c_int) as
                                               u32_0) &
                                             (((0x1 as libc::c_int) <<
                                                   24 as libc::c_int) -
                                                  1 as libc::c_int) as
                                                 libc::c_uint) <<
                                            0 as libc::c_int,
                                w1: 0 as libc::c_int as u32_0,};
                     init
                 },},
         Gfx{words:
                 {
                     let mut init =
                         Gwords{w0:
                                    (0xfb as libc::c_int as u32_0 &
                                         (((0x1 as libc::c_int) <<
                                               8 as libc::c_int) -
                                              1 as libc::c_int) as
                                             libc::c_uint) <<
                                        24 as libc::c_int,
                                w1:
                                    (0 as libc::c_int as u32_0 &
                                         (((0x1 as libc::c_int) <<
                                               8 as libc::c_int) -
                                              1 as libc::c_int) as
                                             libc::c_uint) <<
                                        24 as libc::c_int |
                                        (0 as libc::c_int as u32_0 &
                                             (((0x1 as libc::c_int) <<
                                                   8 as libc::c_int) -
                                                  1 as libc::c_int) as
                                                 libc::c_uint) <<
                                            16 as libc::c_int |
                                        (0 as libc::c_int as u32_0 &
                                             (((0x1 as libc::c_int) <<
                                                   8 as libc::c_int) -
                                                  1 as libc::c_int) as
                                                 libc::c_uint) <<
                                            8 as libc::c_int |
                                        (0 as libc::c_int as u32_0 &
                                             (((0x1 as libc::c_int) <<
                                                   8 as libc::c_int) -
                                                  1 as libc::c_int) as
                                                 libc::c_uint) <<
                                            0 as libc::c_int,};
                     init
                 },},
         Gfx{words:
                 {
                     let mut init =
                         Gwords{w0:
                                    (0xfd as libc::c_int as u32_0 &
                                         (((0x1 as libc::c_int) <<
                                               8 as libc::c_int) -
                                              1 as libc::c_int) as
                                             libc::c_uint) <<
                                        24 as libc::c_int |
                                        (4 as libc::c_int as u32_0 &
                                             (((0x1 as libc::c_int) <<
                                                   3 as libc::c_int) -
                                                  1 as libc::c_int) as
                                                 libc::c_uint) <<
                                            21 as libc::c_int |
                                        (2 as libc::c_int as u32_0 &
                                             (((0x1 as libc::c_int) <<
                                                   2 as libc::c_int) -
                                                  1 as libc::c_int) as
                                                 libc::c_uint) <<
                                            19 as libc::c_int |
                                        ((1 as libc::c_int - 1 as libc::c_int)
                                             as u32_0 &
                                             (((0x1 as libc::c_int) <<
                                                   12 as libc::c_int) -
                                                  1 as libc::c_int) as
                                                 libc::c_uint) <<
                                            0 as libc::c_int,
                                w1:
                                    ovl_Boss_SstTex_00A438.as_mut_ptr() as
                                        libc::c_uint,};
                     init
                 },},
         Gfx{words:
                 {
                     let mut init =
                         Gwords{w0:
                                    (0xf5 as libc::c_int as u32_0 &
                                         (((0x1 as libc::c_int) <<
                                               8 as libc::c_int) -
                                              1 as libc::c_int) as
                                             libc::c_uint) <<
                                        24 as libc::c_int |
                                        (4 as libc::c_int as u32_0 &
                                             (((0x1 as libc::c_int) <<
                                                   3 as libc::c_int) -
                                                  1 as libc::c_int) as
                                                 libc::c_uint) <<
                                            21 as libc::c_int |
                                        (2 as libc::c_int as u32_0 &
                                             (((0x1 as libc::c_int) <<
                                                   2 as libc::c_int) -
                                                  1 as libc::c_int) as
                                                 libc::c_uint) <<
                                            19 as libc::c_int |
                                        (0 as libc::c_int as u32_0 &
                                             (((0x1 as libc::c_int) <<
                                                   9 as libc::c_int) -
                                                  1 as libc::c_int) as
                                                 libc::c_uint) <<
                                            9 as libc::c_int |
                                        (0 as libc::c_int as u32_0 &
                                             (((0x1 as libc::c_int) <<
                                                   9 as libc::c_int) -
                                                  1 as libc::c_int) as
                                                 libc::c_uint) <<
                                            0 as libc::c_int,
                                w1:
                                    (7 as libc::c_int as u32_0 &
                                         (((0x1 as libc::c_int) <<
                                               3 as libc::c_int) -
                                              1 as libc::c_int) as
                                             libc::c_uint) <<
                                        24 as libc::c_int |
                                        (0 as libc::c_int as u32_0 &
                                             (((0x1 as libc::c_int) <<
                                                   4 as libc::c_int) -
                                                  1 as libc::c_int) as
                                                 libc::c_uint) <<
                                            20 as libc::c_int |
                                        ((0 as libc::c_int |
                                              0x2 as libc::c_int) as u32_0 &
                                             (((0x1 as libc::c_int) <<
                                                   2 as libc::c_int) -
                                                  1 as libc::c_int) as
                                                 libc::c_uint) <<
                                            18 as libc::c_int |
                                        (6 as libc::c_int as u32_0 &
                                             (((0x1 as libc::c_int) <<
                                                   4 as libc::c_int) -
                                                  1 as libc::c_int) as
                                                 libc::c_uint) <<
                                            14 as libc::c_int |
                                        (0 as libc::c_int as u32_0 &
                                             (((0x1 as libc::c_int) <<
                                                   4 as libc::c_int) -
                                                  1 as libc::c_int) as
                                                 libc::c_uint) <<
                                            10 as libc::c_int |
                                        ((0 as libc::c_int | 0 as libc::c_int)
                                             as u32_0 &
                                             (((0x1 as libc::c_int) <<
                                                   2 as libc::c_int) -
                                                  1 as libc::c_int) as
                                                 libc::c_uint) <<
                                            8 as libc::c_int |
                                        (4 as libc::c_int as u32_0 &
                                             (((0x1 as libc::c_int) <<
                                                   4 as libc::c_int) -
                                                  1 as libc::c_int) as
                                                 libc::c_uint) <<
                                            4 as libc::c_int |
                                        (0 as libc::c_int as u32_0 &
                                             (((0x1 as libc::c_int) <<
                                                   4 as libc::c_int) -
                                                  1 as libc::c_int) as
                                                 libc::c_uint) <<
                                            0 as libc::c_int,};
                     init
                 },},
         Gfx{words:
                 {
                     let mut init =
                         Gwords{w0:
                                    (0xe6 as libc::c_int as u32_0 &
                                         (((0x1 as libc::c_int) <<
                                               8 as libc::c_int) -
                                              1 as libc::c_int) as
                                             libc::c_uint) <<
                                        24 as libc::c_int,
                                w1: 0 as libc::c_int as libc::c_uint,};
                     init
                 },},
         Gfx{words:
                 {
                     let mut init =
                         Gwords{w0:
                                    (0xf3 as libc::c_int as u32_0 &
                                         (((0x1 as libc::c_int) <<
                                               8 as libc::c_int) -
                                              1 as libc::c_int) as
                                             libc::c_uint) <<
                                        24 as libc::c_int |
                                        (0 as libc::c_int as u32_0 &
                                             (((0x1 as libc::c_int) <<
                                                   12 as libc::c_int) -
                                                  1 as libc::c_int) as
                                                 libc::c_uint) <<
                                            12 as libc::c_int |
                                        (0 as libc::c_int as u32_0 &
                                             (((0x1 as libc::c_int) <<
                                                   12 as libc::c_int) -
                                                  1 as libc::c_int) as
                                                 libc::c_uint) <<
                                            0 as libc::c_int,
                                w1:
                                    (7 as libc::c_int as u32_0 &
                                         (((0x1 as libc::c_int) <<
                                               3 as libc::c_int) -
                                              1 as libc::c_int) as
                                             libc::c_uint) <<
                                        24 as libc::c_int |
                                        ((if ((16 as libc::c_int *
                                                   64 as libc::c_int +
                                                   1 as libc::c_int >>
                                                   1 as libc::c_int) -
                                                  1 as libc::c_int) <
                                                 2047 as libc::c_int {
                                              (16 as libc::c_int *
                                                   64 as libc::c_int +
                                                   1 as libc::c_int >>
                                                   1 as libc::c_int) -
                                                  1 as libc::c_int
                                          } else { 2047 as libc::c_int }) as
                                             u32_0 &
                                             (((0x1 as libc::c_int) <<
                                                   12 as libc::c_int) -
                                                  1 as libc::c_int) as
                                                 libc::c_uint) <<
                                            12 as libc::c_int |
                                        (((((1 as libc::c_int) <<
                                                11 as libc::c_int) +
                                               (if 1 as libc::c_int >
                                                       16 as libc::c_int *
                                                           1 as libc::c_int /
                                                           8 as libc::c_int {
                                                    1 as libc::c_int
                                                } else {
                                                    (16 as libc::c_int *
                                                         1 as libc::c_int) /
                                                        8 as libc::c_int
                                                }) - 1 as libc::c_int) /
                                              (if 1 as libc::c_int >
                                                      16 as libc::c_int *
                                                          1 as libc::c_int /
                                                          8 as libc::c_int {
                                                   1 as libc::c_int
                                               } else {
                                                   (16 as libc::c_int *
                                                        1 as libc::c_int) /
                                                       8 as libc::c_int
                                               })) as u32_0 &
                                             (((0x1 as libc::c_int) <<
                                                   12 as libc::c_int) -
                                                  1 as libc::c_int) as
                                                 libc::c_uint) <<
                                            0 as libc::c_int,};
                     init
                 },},
         Gfx{words:
                 {
                     let mut init =
                         Gwords{w0:
                                    (0xe7 as libc::c_int as u32_0 &
                                         (((0x1 as libc::c_int) <<
                                               8 as libc::c_int) -
                                              1 as libc::c_int) as
                                             libc::c_uint) <<
                                        24 as libc::c_int,
                                w1: 0 as libc::c_int as libc::c_uint,};
                     init
                 },},
         Gfx{words:
                 {
                     let mut init =
                         Gwords{w0:
                                    (0xf5 as libc::c_int as u32_0 &
                                         (((0x1 as libc::c_int) <<
                                               8 as libc::c_int) -
                                              1 as libc::c_int) as
                                             libc::c_uint) <<
                                        24 as libc::c_int |
                                        (4 as libc::c_int as u32_0 &
                                             (((0x1 as libc::c_int) <<
                                                   3 as libc::c_int) -
                                                  1 as libc::c_int) as
                                                 libc::c_uint) <<
                                            21 as libc::c_int |
                                        (1 as libc::c_int as u32_0 &
                                             (((0x1 as libc::c_int) <<
                                                   2 as libc::c_int) -
                                                  1 as libc::c_int) as
                                                 libc::c_uint) <<
                                            19 as libc::c_int |
                                        ((16 as libc::c_int * 1 as libc::c_int
                                              + 7 as libc::c_int >>
                                              3 as libc::c_int) as u32_0 &
                                             (((0x1 as libc::c_int) <<
                                                   9 as libc::c_int) -
                                                  1 as libc::c_int) as
                                                 libc::c_uint) <<
                                            9 as libc::c_int |
                                        (0 as libc::c_int as u32_0 &
                                             (((0x1 as libc::c_int) <<
                                                   9 as libc::c_int) -
                                                  1 as libc::c_int) as
                                                 libc::c_uint) <<
                                            0 as libc::c_int,
                                w1:
                                    (0 as libc::c_int as u32_0 &
                                         (((0x1 as libc::c_int) <<
                                               3 as libc::c_int) -
                                              1 as libc::c_int) as
                                             libc::c_uint) <<
                                        24 as libc::c_int |
                                        (0 as libc::c_int as u32_0 &
                                             (((0x1 as libc::c_int) <<
                                                   4 as libc::c_int) -
                                                  1 as libc::c_int) as
                                                 libc::c_uint) <<
                                            20 as libc::c_int |
                                        ((0 as libc::c_int |
                                              0x2 as libc::c_int) as u32_0 &
                                             (((0x1 as libc::c_int) <<
                                                   2 as libc::c_int) -
                                                  1 as libc::c_int) as
                                                 libc::c_uint) <<
                                            18 as libc::c_int |
                                        (6 as libc::c_int as u32_0 &
                                             (((0x1 as libc::c_int) <<
                                                   4 as libc::c_int) -
                                                  1 as libc::c_int) as
                                                 libc::c_uint) <<
                                            14 as libc::c_int |
                                        (0 as libc::c_int as u32_0 &
                                             (((0x1 as libc::c_int) <<
                                                   4 as libc::c_int) -
                                                  1 as libc::c_int) as
                                                 libc::c_uint) <<
                                            10 as libc::c_int |
                                        ((0 as libc::c_int | 0 as libc::c_int)
                                             as u32_0 &
                                             (((0x1 as libc::c_int) <<
                                                   2 as libc::c_int) -
                                                  1 as libc::c_int) as
                                                 libc::c_uint) <<
                                            8 as libc::c_int |
                                        (4 as libc::c_int as u32_0 &
                                             (((0x1 as libc::c_int) <<
                                                   4 as libc::c_int) -
                                                  1 as libc::c_int) as
                                                 libc::c_uint) <<
                                            4 as libc::c_int |
                                        (0 as libc::c_int as u32_0 &
                                             (((0x1 as libc::c_int) <<
                                                   4 as libc::c_int) -
                                                  1 as libc::c_int) as
                                                 libc::c_uint) <<
                                            0 as libc::c_int,};
                     init
                 },},
         Gfx{words:
                 {
                     let mut init =
                         Gwords{w0:
                                    (0xf2 as libc::c_int as u32_0 &
                                         (((0x1 as libc::c_int) <<
                                               8 as libc::c_int) -
                                              1 as libc::c_int) as
                                             libc::c_uint) <<
                                        24 as libc::c_int |
                                        (0 as libc::c_int as u32_0 &
                                             (((0x1 as libc::c_int) <<
                                                   12 as libc::c_int) -
                                                  1 as libc::c_int) as
                                                 libc::c_uint) <<
                                            12 as libc::c_int |
                                        (0 as libc::c_int as u32_0 &
                                             (((0x1 as libc::c_int) <<
                                                   12 as libc::c_int) -
                                                  1 as libc::c_int) as
                                                 libc::c_uint) <<
                                            0 as libc::c_int,
                                w1:
                                    (0 as libc::c_int as u32_0 &
                                         (((0x1 as libc::c_int) <<
                                               3 as libc::c_int) -
                                              1 as libc::c_int) as
                                             libc::c_uint) <<
                                        24 as libc::c_int |
                                        (((16 as libc::c_int -
                                               1 as libc::c_int) <<
                                              2 as libc::c_int) as u32_0 &
                                             (((0x1 as libc::c_int) <<
                                                   12 as libc::c_int) -
                                                  1 as libc::c_int) as
                                                 libc::c_uint) <<
                                            12 as libc::c_int |
                                        (((64 as libc::c_int -
                                               1 as libc::c_int) <<
                                              2 as libc::c_int) as u32_0 &
                                             (((0x1 as libc::c_int) <<
                                                   12 as libc::c_int) -
                                                  1 as libc::c_int) as
                                                 libc::c_uint) <<
                                            0 as libc::c_int,};
                     init
                 },},
         Gfx{words:
                 {
                     let mut init =
                         Gwords{w0:
                                    (0x1 as libc::c_int as u32_0 &
                                         (((0x1 as libc::c_int) <<
                                               8 as libc::c_int) -
                                              1 as libc::c_int) as
                                             libc::c_uint) <<
                                        24 as libc::c_int |
                                        (4 as libc::c_int as u32_0 &
                                             (((0x1 as libc::c_int) <<
                                                   8 as libc::c_int) -
                                                  1 as libc::c_int) as
                                                 libc::c_uint) <<
                                            12 as libc::c_int |
                                        ((0 as libc::c_int + 4 as libc::c_int)
                                             as u32_0 &
                                             (((0x1 as libc::c_int) <<
                                                   7 as libc::c_int) -
                                                  1 as libc::c_int) as
                                                 libc::c_uint) <<
                                            1 as libc::c_int,
                                w1:
                                    ovl_Boss_SstVtx_00A3F8.as_mut_ptr() as
                                        libc::c_uint,};
                     init
                 },},
         Gfx{words:
                 {
                     let mut init =
                         Gwords{w0:
                                    (0x6 as libc::c_int as u32_0 &
                                         (((0x1 as libc::c_int) <<
                                               8 as libc::c_int) -
                                              1 as libc::c_int) as
                                             libc::c_uint) <<
                                        24 as libc::c_int |
                                        (if 0 as libc::c_int ==
                                                0 as libc::c_int {
                                             (((0 as libc::c_int *
                                                    2 as libc::c_int) as u32_0
                                                   &
                                                   (((0x1 as libc::c_int) <<
                                                         8 as libc::c_int) -
                                                        1 as libc::c_int) as
                                                       libc::c_uint) <<
                                                  16 as libc::c_int |
                                                  ((1 as libc::c_int *
                                                        2 as libc::c_int) as
                                                       u32_0 &
                                                       (((0x1 as libc::c_int)
                                                             <<
                                                             8 as libc::c_int)
                                                            -
                                                            1 as libc::c_int)
                                                           as libc::c_uint) <<
                                                      8 as libc::c_int) |
                                                 ((2 as libc::c_int *
                                                       2 as libc::c_int) as
                                                      u32_0 &
                                                      (((0x1 as libc::c_int)
                                                            <<
                                                            8 as libc::c_int)
                                                           - 1 as libc::c_int)
                                                          as libc::c_uint) <<
                                                     0 as libc::c_int
                                         } else {
                                             (if 0 as libc::c_int ==
                                                     1 as libc::c_int {
                                                  (((1 as libc::c_int *
                                                         2 as libc::c_int) as
                                                        u32_0 &
                                                        (((0x1 as libc::c_int)
                                                              <<
                                                              8 as
                                                                  libc::c_int)
                                                             -
                                                             1 as libc::c_int)
                                                            as libc::c_uint)
                                                       << 16 as libc::c_int |
                                                       ((2 as libc::c_int *
                                                             2 as libc::c_int)
                                                            as u32_0 &
                                                            (((0x1 as
                                                                   libc::c_int)
                                                                  <<
                                                                  8 as
                                                                      libc::c_int)
                                                                 -
                                                                 1 as
                                                                     libc::c_int)
                                                                as
                                                                libc::c_uint)
                                                           <<
                                                           8 as libc::c_int) |
                                                      ((0 as libc::c_int *
                                                            2 as libc::c_int)
                                                           as u32_0 &
                                                           (((0x1 as
                                                                  libc::c_int)
                                                                 <<
                                                                 8 as
                                                                     libc::c_int)
                                                                -
                                                                1 as
                                                                    libc::c_int)
                                                               as
                                                               libc::c_uint)
                                                          << 0 as libc::c_int
                                              } else {
                                                  (((2 as libc::c_int *
                                                         2 as libc::c_int) as
                                                        u32_0 &
                                                        (((0x1 as libc::c_int)
                                                              <<
                                                              8 as
                                                                  libc::c_int)
                                                             -
                                                             1 as libc::c_int)
                                                            as libc::c_uint)
                                                       << 16 as libc::c_int |
                                                       ((0 as libc::c_int *
                                                             2 as libc::c_int)
                                                            as u32_0 &
                                                            (((0x1 as
                                                                   libc::c_int)
                                                                  <<
                                                                  8 as
                                                                      libc::c_int)
                                                                 -
                                                                 1 as
                                                                     libc::c_int)
                                                                as
                                                                libc::c_uint)
                                                           <<
                                                           8 as libc::c_int) |
                                                      ((1 as libc::c_int *
                                                            2 as libc::c_int)
                                                           as u32_0 &
                                                           (((0x1 as
                                                                  libc::c_int)
                                                                 <<
                                                                 8 as
                                                                     libc::c_int)
                                                                -
                                                                1 as
                                                                    libc::c_int)
                                                               as
                                                               libc::c_uint)
                                                          << 0 as libc::c_int
                                              })
                                         }),
                                w1:
                                    if 0 as libc::c_int == 0 as libc::c_int {
                                        (((0 as libc::c_int *
                                               2 as libc::c_int) as u32_0 &
                                              (((0x1 as libc::c_int) <<
                                                    8 as libc::c_int) -
                                                   1 as libc::c_int) as
                                                  libc::c_uint) <<
                                             16 as libc::c_int |
                                             ((2 as libc::c_int *
                                                   2 as libc::c_int) as u32_0
                                                  &
                                                  (((0x1 as libc::c_int) <<
                                                        8 as libc::c_int) -
                                                       1 as libc::c_int) as
                                                      libc::c_uint) <<
                                                 8 as libc::c_int) |
                                            ((3 as libc::c_int *
                                                  2 as libc::c_int) as u32_0 &
                                                 (((0x1 as libc::c_int) <<
                                                       8 as libc::c_int) -
                                                      1 as libc::c_int) as
                                                     libc::c_uint) <<
                                                0 as libc::c_int
                                    } else if 0 as libc::c_int ==
                                                  1 as libc::c_int {
                                        (((2 as libc::c_int *
                                               2 as libc::c_int) as u32_0 &
                                              (((0x1 as libc::c_int) <<
                                                    8 as libc::c_int) -
                                                   1 as libc::c_int) as
                                                  libc::c_uint) <<
                                             16 as libc::c_int |
                                             ((3 as libc::c_int *
                                                   2 as libc::c_int) as u32_0
                                                  &
                                                  (((0x1 as libc::c_int) <<
                                                        8 as libc::c_int) -
                                                       1 as libc::c_int) as
                                                      libc::c_uint) <<
                                                 8 as libc::c_int) |
                                            ((0 as libc::c_int *
                                                  2 as libc::c_int) as u32_0 &
                                                 (((0x1 as libc::c_int) <<
                                                       8 as libc::c_int) -
                                                      1 as libc::c_int) as
                                                     libc::c_uint) <<
                                                0 as libc::c_int
                                    } else {
                                        (((3 as libc::c_int *
                                               2 as libc::c_int) as u32_0 &
                                              (((0x1 as libc::c_int) <<
                                                    8 as libc::c_int) -
                                                   1 as libc::c_int) as
                                                  libc::c_uint) <<
                                             16 as libc::c_int |
                                             ((0 as libc::c_int *
                                                   2 as libc::c_int) as u32_0
                                                  &
                                                  (((0x1 as libc::c_int) <<
                                                        8 as libc::c_int) -
                                                       1 as libc::c_int) as
                                                      libc::c_uint) <<
                                                 8 as libc::c_int) |
                                            ((2 as libc::c_int *
                                                  2 as libc::c_int) as u32_0 &
                                                 (((0x1 as libc::c_int) <<
                                                       8 as libc::c_int) -
                                                      1 as libc::c_int) as
                                                     libc::c_uint) <<
                                                0 as libc::c_int
                                    },};
                     init
                 },},
         Gfx{words:
                 {
                     let mut init =
                         Gwords{w0:
                                    (0xdf as libc::c_int as u32_0 &
                                         (((0x1 as libc::c_int) <<
                                               8 as libc::c_int) -
                                              1 as libc::c_int) as
                                             libc::c_uint) <<
                                        24 as libc::c_int,
                                w1: 0 as libc::c_int as libc::c_uint,};
                     init
                 },}];
    sShadowDList =
        [Gfx{words:
                 {
                     let mut init =
                         Gwords{w0:
                                    (0xe7 as libc::c_int as u32_0 &
                                         (((0x1 as libc::c_int) <<
                                               8 as libc::c_int) -
                                              1 as libc::c_int) as
                                             libc::c_uint) <<
                                        24 as libc::c_int,
                                w1: 0 as libc::c_int as libc::c_uint,};
                     init
                 },},
         Gfx{words:
                 {
                     let mut init =
                         Gwords{w0:
                                    (0xd7 as libc::c_int as u32_0 &
                                         (((0x1 as libc::c_int) <<
                                               8 as libc::c_int) -
                                              1 as libc::c_int) as
                                             libc::c_uint) <<
                                        24 as libc::c_int |
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
                                                 libc::c_uint) <<
                                            8 as libc::c_int |
                                        (1 as libc::c_int as u32_0 &
                                             (((0x1 as libc::c_int) <<
                                                   7 as libc::c_int) -
                                                  1 as libc::c_int) as
                                                 libc::c_uint) <<
                                            1 as libc::c_int,
                                w1:
                                    (0xffff as libc::c_int as u32_0 &
                                         (((0x1 as libc::c_int) <<
                                               16 as libc::c_int) -
                                              1 as libc::c_int) as
                                             libc::c_uint) <<
                                        16 as libc::c_int |
                                        (0xffff as libc::c_int as u32_0 &
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
                                    (0xfd as libc::c_int as u32_0 &
                                         (((0x1 as libc::c_int) <<
                                               8 as libc::c_int) -
                                              1 as libc::c_int) as
                                             libc::c_uint) <<
                                        24 as libc::c_int |
                                        (4 as libc::c_int as u32_0 &
                                             (((0x1 as libc::c_int) <<
                                                   3 as libc::c_int) -
                                                  1 as libc::c_int) as
                                                 libc::c_uint) <<
                                            21 as libc::c_int |
                                        (2 as libc::c_int as u32_0 &
                                             (((0x1 as libc::c_int) <<
                                                   2 as libc::c_int) -
                                                  1 as libc::c_int) as
                                                 libc::c_uint) <<
                                            19 as libc::c_int |
                                        ((1 as libc::c_int - 1 as libc::c_int)
                                             as u32_0 &
                                             (((0x1 as libc::c_int) <<
                                                   12 as libc::c_int) -
                                                  1 as libc::c_int) as
                                                 libc::c_uint) <<
                                            0 as libc::c_int,
                                w1:
                                    ovl_Boss_SstTex_00A8F0.as_mut_ptr() as
                                        libc::c_uint,};
                     init
                 },},
         Gfx{words:
                 {
                     let mut init =
                         Gwords{w0:
                                    (0xf5 as libc::c_int as u32_0 &
                                         (((0x1 as libc::c_int) <<
                                               8 as libc::c_int) -
                                              1 as libc::c_int) as
                                             libc::c_uint) <<
                                        24 as libc::c_int |
                                        (4 as libc::c_int as u32_0 &
                                             (((0x1 as libc::c_int) <<
                                                   3 as libc::c_int) -
                                                  1 as libc::c_int) as
                                                 libc::c_uint) <<
                                            21 as libc::c_int |
                                        (2 as libc::c_int as u32_0 &
                                             (((0x1 as libc::c_int) <<
                                                   2 as libc::c_int) -
                                                  1 as libc::c_int) as
                                                 libc::c_uint) <<
                                            19 as libc::c_int |
                                        (0 as libc::c_int as u32_0 &
                                             (((0x1 as libc::c_int) <<
                                                   9 as libc::c_int) -
                                                  1 as libc::c_int) as
                                                 libc::c_uint) <<
                                            9 as libc::c_int |
                                        (0 as libc::c_int as u32_0 &
                                             (((0x1 as libc::c_int) <<
                                                   9 as libc::c_int) -
                                                  1 as libc::c_int) as
                                                 libc::c_uint) <<
                                            0 as libc::c_int,
                                w1:
                                    (7 as libc::c_int as u32_0 &
                                         (((0x1 as libc::c_int) <<
                                               3 as libc::c_int) -
                                              1 as libc::c_int) as
                                             libc::c_uint) <<
                                        24 as libc::c_int |
                                        (0 as libc::c_int as u32_0 &
                                             (((0x1 as libc::c_int) <<
                                                   4 as libc::c_int) -
                                                  1 as libc::c_int) as
                                                 libc::c_uint) <<
                                            20 as libc::c_int |
                                        ((0 as libc::c_int |
                                              0x2 as libc::c_int) as u32_0 &
                                             (((0x1 as libc::c_int) <<
                                                   2 as libc::c_int) -
                                                  1 as libc::c_int) as
                                                 libc::c_uint) <<
                                            18 as libc::c_int |
                                        (5 as libc::c_int as u32_0 &
                                             (((0x1 as libc::c_int) <<
                                                   4 as libc::c_int) -
                                                  1 as libc::c_int) as
                                                 libc::c_uint) <<
                                            14 as libc::c_int |
                                        (0 as libc::c_int as u32_0 &
                                             (((0x1 as libc::c_int) <<
                                                   4 as libc::c_int) -
                                                  1 as libc::c_int) as
                                                 libc::c_uint) <<
                                            10 as libc::c_int |
                                        ((0 as libc::c_int |
                                              0x2 as libc::c_int) as u32_0 &
                                             (((0x1 as libc::c_int) <<
                                                   2 as libc::c_int) -
                                                  1 as libc::c_int) as
                                                 libc::c_uint) <<
                                            8 as libc::c_int |
                                        (5 as libc::c_int as u32_0 &
                                             (((0x1 as libc::c_int) <<
                                                   4 as libc::c_int) -
                                                  1 as libc::c_int) as
                                                 libc::c_uint) <<
                                            4 as libc::c_int |
                                        (0 as libc::c_int as u32_0 &
                                             (((0x1 as libc::c_int) <<
                                                   4 as libc::c_int) -
                                                  1 as libc::c_int) as
                                                 libc::c_uint) <<
                                            0 as libc::c_int,};
                     init
                 },},
         Gfx{words:
                 {
                     let mut init =
                         Gwords{w0:
                                    (0xe6 as libc::c_int as u32_0 &
                                         (((0x1 as libc::c_int) <<
                                               8 as libc::c_int) -
                                              1 as libc::c_int) as
                                             libc::c_uint) <<
                                        24 as libc::c_int,
                                w1: 0 as libc::c_int as libc::c_uint,};
                     init
                 },},
         Gfx{words:
                 {
                     let mut init =
                         Gwords{w0:
                                    (0xf3 as libc::c_int as u32_0 &
                                         (((0x1 as libc::c_int) <<
                                               8 as libc::c_int) -
                                              1 as libc::c_int) as
                                             libc::c_uint) <<
                                        24 as libc::c_int |
                                        (0 as libc::c_int as u32_0 &
                                             (((0x1 as libc::c_int) <<
                                                   12 as libc::c_int) -
                                                  1 as libc::c_int) as
                                                 libc::c_uint) <<
                                            12 as libc::c_int |
                                        (0 as libc::c_int as u32_0 &
                                             (((0x1 as libc::c_int) <<
                                                   12 as libc::c_int) -
                                                  1 as libc::c_int) as
                                                 libc::c_uint) <<
                                            0 as libc::c_int,
                                w1:
                                    (7 as libc::c_int as u32_0 &
                                         (((0x1 as libc::c_int) <<
                                               3 as libc::c_int) -
                                              1 as libc::c_int) as
                                             libc::c_uint) <<
                                        24 as libc::c_int |
                                        ((if ((32 as libc::c_int *
                                                   32 as libc::c_int +
                                                   1 as libc::c_int >>
                                                   1 as libc::c_int) -
                                                  1 as libc::c_int) <
                                                 2047 as libc::c_int {
                                              (32 as libc::c_int *
                                                   32 as libc::c_int +
                                                   1 as libc::c_int >>
                                                   1 as libc::c_int) -
                                                  1 as libc::c_int
                                          } else { 2047 as libc::c_int }) as
                                             u32_0 &
                                             (((0x1 as libc::c_int) <<
                                                   12 as libc::c_int) -
                                                  1 as libc::c_int) as
                                                 libc::c_uint) <<
                                            12 as libc::c_int |
                                        (((((1 as libc::c_int) <<
                                                11 as libc::c_int) +
                                               (if 1 as libc::c_int >
                                                       32 as libc::c_int *
                                                           1 as libc::c_int /
                                                           8 as libc::c_int {
                                                    1 as libc::c_int
                                                } else {
                                                    (32 as libc::c_int *
                                                         1 as libc::c_int) /
                                                        8 as libc::c_int
                                                }) - 1 as libc::c_int) /
                                              (if 1 as libc::c_int >
                                                      32 as libc::c_int *
                                                          1 as libc::c_int /
                                                          8 as libc::c_int {
                                                   1 as libc::c_int
                                               } else {
                                                   (32 as libc::c_int *
                                                        1 as libc::c_int) /
                                                       8 as libc::c_int
                                               })) as u32_0 &
                                             (((0x1 as libc::c_int) <<
                                                   12 as libc::c_int) -
                                                  1 as libc::c_int) as
                                                 libc::c_uint) <<
                                            0 as libc::c_int,};
                     init
                 },},
         Gfx{words:
                 {
                     let mut init =
                         Gwords{w0:
                                    (0xe7 as libc::c_int as u32_0 &
                                         (((0x1 as libc::c_int) <<
                                               8 as libc::c_int) -
                                              1 as libc::c_int) as
                                             libc::c_uint) <<
                                        24 as libc::c_int,
                                w1: 0 as libc::c_int as libc::c_uint,};
                     init
                 },},
         Gfx{words:
                 {
                     let mut init =
                         Gwords{w0:
                                    (0xf5 as libc::c_int as u32_0 &
                                         (((0x1 as libc::c_int) <<
                                               8 as libc::c_int) -
                                              1 as libc::c_int) as
                                             libc::c_uint) <<
                                        24 as libc::c_int |
                                        (4 as libc::c_int as u32_0 &
                                             (((0x1 as libc::c_int) <<
                                                   3 as libc::c_int) -
                                                  1 as libc::c_int) as
                                                 libc::c_uint) <<
                                            21 as libc::c_int |
                                        (1 as libc::c_int as u32_0 &
                                             (((0x1 as libc::c_int) <<
                                                   2 as libc::c_int) -
                                                  1 as libc::c_int) as
                                                 libc::c_uint) <<
                                            19 as libc::c_int |
                                        ((32 as libc::c_int * 1 as libc::c_int
                                              + 7 as libc::c_int >>
                                              3 as libc::c_int) as u32_0 &
                                             (((0x1 as libc::c_int) <<
                                                   9 as libc::c_int) -
                                                  1 as libc::c_int) as
                                                 libc::c_uint) <<
                                            9 as libc::c_int |
                                        (0 as libc::c_int as u32_0 &
                                             (((0x1 as libc::c_int) <<
                                                   9 as libc::c_int) -
                                                  1 as libc::c_int) as
                                                 libc::c_uint) <<
                                            0 as libc::c_int,
                                w1:
                                    (0 as libc::c_int as u32_0 &
                                         (((0x1 as libc::c_int) <<
                                               3 as libc::c_int) -
                                              1 as libc::c_int) as
                                             libc::c_uint) <<
                                        24 as libc::c_int |
                                        (0 as libc::c_int as u32_0 &
                                             (((0x1 as libc::c_int) <<
                                                   4 as libc::c_int) -
                                                  1 as libc::c_int) as
                                                 libc::c_uint) <<
                                            20 as libc::c_int |
                                        ((0 as libc::c_int |
                                              0x2 as libc::c_int) as u32_0 &
                                             (((0x1 as libc::c_int) <<
                                                   2 as libc::c_int) -
                                                  1 as libc::c_int) as
                                                 libc::c_uint) <<
                                            18 as libc::c_int |
                                        (5 as libc::c_int as u32_0 &
                                             (((0x1 as libc::c_int) <<
                                                   4 as libc::c_int) -
                                                  1 as libc::c_int) as
                                                 libc::c_uint) <<
                                            14 as libc::c_int |
                                        (0 as libc::c_int as u32_0 &
                                             (((0x1 as libc::c_int) <<
                                                   4 as libc::c_int) -
                                                  1 as libc::c_int) as
                                                 libc::c_uint) <<
                                            10 as libc::c_int |
                                        ((0 as libc::c_int |
                                              0x2 as libc::c_int) as u32_0 &
                                             (((0x1 as libc::c_int) <<
                                                   2 as libc::c_int) -
                                                  1 as libc::c_int) as
                                                 libc::c_uint) <<
                                            8 as libc::c_int |
                                        (5 as libc::c_int as u32_0 &
                                             (((0x1 as libc::c_int) <<
                                                   4 as libc::c_int) -
                                                  1 as libc::c_int) as
                                                 libc::c_uint) <<
                                            4 as libc::c_int |
                                        (0 as libc::c_int as u32_0 &
                                             (((0x1 as libc::c_int) <<
                                                   4 as libc::c_int) -
                                                  1 as libc::c_int) as
                                                 libc::c_uint) <<
                                            0 as libc::c_int,};
                     init
                 },},
         Gfx{words:
                 {
                     let mut init =
                         Gwords{w0:
                                    (0xf2 as libc::c_int as u32_0 &
                                         (((0x1 as libc::c_int) <<
                                               8 as libc::c_int) -
                                              1 as libc::c_int) as
                                             libc::c_uint) <<
                                        24 as libc::c_int |
                                        (0 as libc::c_int as u32_0 &
                                             (((0x1 as libc::c_int) <<
                                                   12 as libc::c_int) -
                                                  1 as libc::c_int) as
                                                 libc::c_uint) <<
                                            12 as libc::c_int |
                                        (0 as libc::c_int as u32_0 &
                                             (((0x1 as libc::c_int) <<
                                                   12 as libc::c_int) -
                                                  1 as libc::c_int) as
                                                 libc::c_uint) <<
                                            0 as libc::c_int,
                                w1:
                                    (0 as libc::c_int as u32_0 &
                                         (((0x1 as libc::c_int) <<
                                               3 as libc::c_int) -
                                              1 as libc::c_int) as
                                             libc::c_uint) <<
                                        24 as libc::c_int |
                                        (((32 as libc::c_int -
                                               1 as libc::c_int) <<
                                              2 as libc::c_int) as u32_0 &
                                             (((0x1 as libc::c_int) <<
                                                   12 as libc::c_int) -
                                                  1 as libc::c_int) as
                                                 libc::c_uint) <<
                                            12 as libc::c_int |
                                        (((32 as libc::c_int -
                                               1 as libc::c_int) <<
                                              2 as libc::c_int) as u32_0 &
                                             (((0x1 as libc::c_int) <<
                                                   12 as libc::c_int) -
                                                  1 as libc::c_int) as
                                                 libc::c_uint) <<
                                            0 as libc::c_int,};
                     init
                 },},
         Gfx{words:
                 {
                     let mut init =
                         Gwords{w0:
                                    (0xfc as libc::c_int as u32_0 &
                                         (((0x1 as libc::c_int) <<
                                               8 as libc::c_int) -
                                              1 as libc::c_int) as
                                             libc::c_uint) <<
                                        24 as libc::c_int |
                                        (((7 as libc::c_int as u32_0 &
                                               (((0x1 as libc::c_int) <<
                                                     4 as libc::c_int) -
                                                    1 as libc::c_int) as
                                                   libc::c_uint) <<
                                              20 as libc::c_int |
                                              (3 as libc::c_int as u32_0 &
                                                   (((0x1 as libc::c_int) <<
                                                         5 as libc::c_int) -
                                                        1 as libc::c_int) as
                                                       libc::c_uint) <<
                                                  15 as libc::c_int |
                                              (1 as libc::c_int as u32_0 &
                                                   (((0x1 as libc::c_int) <<
                                                         3 as libc::c_int) -
                                                        1 as libc::c_int) as
                                                       libc::c_uint) <<
                                                  12 as libc::c_int |
                                              (5 as libc::c_int as u32_0 &
                                                   (((0x1 as libc::c_int) <<
                                                         3 as libc::c_int) -
                                                        1 as libc::c_int) as
                                                       libc::c_uint) <<
                                                  9 as libc::c_int |
                                              ((31 as libc::c_int as u32_0 &
                                                    (((0x1 as libc::c_int) <<
                                                          4 as libc::c_int) -
                                                         1 as libc::c_int) as
                                                        libc::c_uint) <<
                                                   5 as libc::c_int |
                                                   (31 as libc::c_int as u32_0
                                                        &
                                                        (((0x1 as libc::c_int)
                                                              <<
                                                              5 as
                                                                  libc::c_int)
                                                             -
                                                             1 as libc::c_int)
                                                            as libc::c_uint)
                                                       << 0 as libc::c_int)) &
                                             (((0x1 as libc::c_int) <<
                                                   24 as libc::c_int) -
                                                  1 as libc::c_int) as
                                                 libc::c_uint) <<
                                            0 as libc::c_int,
                                w1:
                                    (5 as libc::c_int as u32_0 &
                                         (((0x1 as libc::c_int) <<
                                               4 as libc::c_int) -
                                              1 as libc::c_int) as
                                             libc::c_uint) <<
                                        28 as libc::c_int |
                                        (5 as libc::c_int as u32_0 &
                                             (((0x1 as libc::c_int) <<
                                                   3 as libc::c_int) -
                                                  1 as libc::c_int) as
                                                 libc::c_uint) <<
                                            15 as libc::c_int |
                                        (7 as libc::c_int as u32_0 &
                                             (((0x1 as libc::c_int) <<
                                                   3 as libc::c_int) -
                                                  1 as libc::c_int) as
                                                 libc::c_uint) <<
                                            12 as libc::c_int |
                                        (7 as libc::c_int as u32_0 &
                                             (((0x1 as libc::c_int) <<
                                                   3 as libc::c_int) -
                                                  1 as libc::c_int) as
                                                 libc::c_uint) <<
                                            9 as libc::c_int |
                                        ((31 as libc::c_int as u32_0 &
                                              (((0x1 as libc::c_int) <<
                                                    4 as libc::c_int) -
                                                   1 as libc::c_int) as
                                                  libc::c_uint) <<
                                             24 as libc::c_int |
                                             (7 as libc::c_int as u32_0 &
                                                  (((0x1 as libc::c_int) <<
                                                        3 as libc::c_int) -
                                                       1 as libc::c_int) as
                                                      libc::c_uint) <<
                                                 21 as libc::c_int |
                                             (7 as libc::c_int as u32_0 &
                                                  (((0x1 as libc::c_int) <<
                                                        3 as libc::c_int) -
                                                       1 as libc::c_int) as
                                                      libc::c_uint) <<
                                                 18 as libc::c_int |
                                             (0 as libc::c_int as u32_0 &
                                                  (((0x1 as libc::c_int) <<
                                                        3 as libc::c_int) -
                                                       1 as libc::c_int) as
                                                      libc::c_uint) <<
                                                 6 as libc::c_int |
                                             (7 as libc::c_int as u32_0 &
                                                  (((0x1 as libc::c_int) <<
                                                        3 as libc::c_int) -
                                                       1 as libc::c_int) as
                                                      libc::c_uint) <<
                                                 3 as libc::c_int |
                                             (0 as libc::c_int as u32_0 &
                                                  (((0x1 as libc::c_int) <<
                                                        3 as libc::c_int) -
                                                       1 as libc::c_int) as
                                                      libc::c_uint) <<
                                                 0 as libc::c_int),};
                     init
                 },},
         Gfx{words:
                 {
                     let mut init =
                         Gwords{w0:
                                    (0xe2 as libc::c_int as u32_0 &
                                         (((0x1 as libc::c_int) <<
                                               8 as libc::c_int) -
                                              1 as libc::c_int) as
                                             libc::c_uint) <<
                                        24 as libc::c_int |
                                        ((32 as libc::c_int - 3 as libc::c_int
                                              - 29 as libc::c_int) as u32_0 &
                                             (((0x1 as libc::c_int) <<
                                                   8 as libc::c_int) -
                                                  1 as libc::c_int) as
                                                 libc::c_uint) <<
                                            8 as libc::c_int |
                                        ((29 as libc::c_int -
                                              1 as libc::c_int) as u32_0 &
                                             (((0x1 as libc::c_int) <<
                                                   8 as libc::c_int) -
                                                  1 as libc::c_int) as
                                                 libc::c_uint) <<
                                            0 as libc::c_int,
                                w1:
                                    ((0 as libc::c_int) << 30 as libc::c_int |
                                         (3 as libc::c_int) <<
                                             26 as libc::c_int |
                                         (0 as libc::c_int) <<
                                             22 as libc::c_int |
                                         (2 as libc::c_int) <<
                                             18 as libc::c_int |
                                         (0x8 as libc::c_int |
                                              0x10 as libc::c_int |
                                              0x40 as libc::c_int |
                                              0x100 as libc::c_int |
                                              0x80 as libc::c_int |
                                              0x4000 as libc::c_int |
                                              0xc00 as libc::c_int |
                                              (0 as libc::c_int) <<
                                                  28 as libc::c_int |
                                              (0 as libc::c_int) <<
                                                  24 as libc::c_int |
                                              (1 as libc::c_int) <<
                                                  20 as libc::c_int |
                                              (0 as libc::c_int) <<
                                                  16 as libc::c_int)) as
                                        libc::c_uint,};
                     init
                 },},
         Gfx{words:
                 {
                     let mut init =
                         Gwords{w0:
                                    (0xd9 as libc::c_int as u32_0 &
                                         (((0x1 as libc::c_int) <<
                                               8 as libc::c_int) -
                                              1 as libc::c_int) as
                                             libc::c_uint) <<
                                        24 as libc::c_int |
                                        (!((0x10000 as libc::c_int |
                                                0x40000 as libc::c_int |
                                                0x80000 as libc::c_int) as
                                               u32_0) &
                                             (((0x1 as libc::c_int) <<
                                                   24 as libc::c_int) -
                                                  1 as libc::c_int) as
                                                 libc::c_uint) <<
                                            0 as libc::c_int,
                                w1: 0 as libc::c_int as u32_0,};
                     init
                 },},
         Gfx{words:
                 {
                     let mut init =
                         Gwords{w0:
                                    (0xd9 as libc::c_int as u32_0 &
                                         (((0x1 as libc::c_int) <<
                                               8 as libc::c_int) -
                                              1 as libc::c_int) as
                                             libc::c_uint) <<
                                        24 as libc::c_int |
                                        (!(0 as libc::c_int as u32_0) &
                                             (((0x1 as libc::c_int) <<
                                                   24 as libc::c_int) -
                                                  1 as libc::c_int) as
                                                 libc::c_uint) <<
                                            0 as libc::c_int,
                                w1:
                                    (0x400 as libc::c_int |
                                         0x20000 as libc::c_int) as u32_0,};
                     init
                 },},
         Gfx{words:
                 {
                     let mut init =
                         Gwords{w0:
                                    (0x1 as libc::c_int as u32_0 &
                                         (((0x1 as libc::c_int) <<
                                               8 as libc::c_int) -
                                              1 as libc::c_int) as
                                             libc::c_uint) <<
                                        24 as libc::c_int |
                                        (3 as libc::c_int as u32_0 &
                                             (((0x1 as libc::c_int) <<
                                                   8 as libc::c_int) -
                                                  1 as libc::c_int) as
                                                 libc::c_uint) <<
                                            12 as libc::c_int |
                                        ((0 as libc::c_int + 3 as libc::c_int)
                                             as u32_0 &
                                             (((0x1 as libc::c_int) <<
                                                   7 as libc::c_int) -
                                                  1 as libc::c_int) as
                                                 libc::c_uint) <<
                                            1 as libc::c_int,
                                w1:
                                    ovl_Boss_SstVtx_00A8C0.as_mut_ptr() as
                                        libc::c_uint,};
                     init
                 },},
         Gfx{words:
                 {
                     let mut init =
                         Gwords{w0:
                                    (0x5 as libc::c_int as u32_0 &
                                         (((0x1 as libc::c_int) <<
                                               8 as libc::c_int) -
                                              1 as libc::c_int) as
                                             libc::c_uint) <<
                                        24 as libc::c_int |
                                        (if 0 as libc::c_int ==
                                                0 as libc::c_int {
                                             (((0 as libc::c_int *
                                                    2 as libc::c_int) as u32_0
                                                   &
                                                   (((0x1 as libc::c_int) <<
                                                         8 as libc::c_int) -
                                                        1 as libc::c_int) as
                                                       libc::c_uint) <<
                                                  16 as libc::c_int |
                                                  ((1 as libc::c_int *
                                                        2 as libc::c_int) as
                                                       u32_0 &
                                                       (((0x1 as libc::c_int)
                                                             <<
                                                             8 as libc::c_int)
                                                            -
                                                            1 as libc::c_int)
                                                           as libc::c_uint) <<
                                                      8 as libc::c_int) |
                                                 ((2 as libc::c_int *
                                                       2 as libc::c_int) as
                                                      u32_0 &
                                                      (((0x1 as libc::c_int)
                                                            <<
                                                            8 as libc::c_int)
                                                           - 1 as libc::c_int)
                                                          as libc::c_uint) <<
                                                     0 as libc::c_int
                                         } else {
                                             (if 0 as libc::c_int ==
                                                     1 as libc::c_int {
                                                  (((1 as libc::c_int *
                                                         2 as libc::c_int) as
                                                        u32_0 &
                                                        (((0x1 as libc::c_int)
                                                              <<
                                                              8 as
                                                                  libc::c_int)
                                                             -
                                                             1 as libc::c_int)
                                                            as libc::c_uint)
                                                       << 16 as libc::c_int |
                                                       ((2 as libc::c_int *
                                                             2 as libc::c_int)
                                                            as u32_0 &
                                                            (((0x1 as
                                                                   libc::c_int)
                                                                  <<
                                                                  8 as
                                                                      libc::c_int)
                                                                 -
                                                                 1 as
                                                                     libc::c_int)
                                                                as
                                                                libc::c_uint)
                                                           <<
                                                           8 as libc::c_int) |
                                                      ((0 as libc::c_int *
                                                            2 as libc::c_int)
                                                           as u32_0 &
                                                           (((0x1 as
                                                                  libc::c_int)
                                                                 <<
                                                                 8 as
                                                                     libc::c_int)
                                                                -
                                                                1 as
                                                                    libc::c_int)
                                                               as
                                                               libc::c_uint)
                                                          << 0 as libc::c_int
                                              } else {
                                                  (((2 as libc::c_int *
                                                         2 as libc::c_int) as
                                                        u32_0 &
                                                        (((0x1 as libc::c_int)
                                                              <<
                                                              8 as
                                                                  libc::c_int)
                                                             -
                                                             1 as libc::c_int)
                                                            as libc::c_uint)
                                                       << 16 as libc::c_int |
                                                       ((0 as libc::c_int *
                                                             2 as libc::c_int)
                                                            as u32_0 &
                                                            (((0x1 as
                                                                   libc::c_int)
                                                                  <<
                                                                  8 as
                                                                      libc::c_int)
                                                                 -
                                                                 1 as
                                                                     libc::c_int)
                                                                as
                                                                libc::c_uint)
                                                           <<
                                                           8 as libc::c_int) |
                                                      ((1 as libc::c_int *
                                                            2 as libc::c_int)
                                                           as u32_0 &
                                                           (((0x1 as
                                                                  libc::c_int)
                                                                 <<
                                                                 8 as
                                                                     libc::c_int)
                                                                -
                                                                1 as
                                                                    libc::c_int)
                                                               as
                                                               libc::c_uint)
                                                          << 0 as libc::c_int
                                              })
                                         }),
                                w1: 0 as libc::c_int as libc::c_uint,};
                     init
                 },},
         Gfx{words:
                 {
                     let mut init =
                         Gwords{w0:
                                    (0xdf as libc::c_int as u32_0 &
                                         (((0x1 as libc::c_int) <<
                                               8 as libc::c_int) -
                                              1 as libc::c_int) as
                                             libc::c_uint) <<
                                        24 as libc::c_int,
                                w1: 0 as libc::c_int as libc::c_uint,};
                     init
                 },}];
    sInitChain =
        [{
             let mut init = InitChainEntry{cont_type_0_offset_value: [0; 4],};
             init.set_cont(1 as libc::c_int as u32_0);
             init.set_type_0(ICHAINTYPE_S8 as libc::c_int as u32_0);
             init.set_offset(&mut (*(0 as *mut Actor)).naviEnemyId as
                                 *mut u8_0 as size_t as u32_0);
             init.set_value(0x29 as libc::c_int);
             init
         },
         {
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
             init.set_cont(0 as libc::c_int as u32_0);
             init.set_type_0(ICHAINTYPE_VEC3F_DIV1000 as libc::c_int as
                                 u32_0);
             init.set_offset(&mut (*(0 as *mut Actor)).scale as *mut Vec3f as
                                 size_t as u32_0);
             init.set_value(20 as libc::c_int);
             init
         }]
}
#[used]
#[cfg_attr(target_os = "linux", link_section = ".init_array")]
#[cfg_attr(target_os = "windows", link_section = ".CRT$XIB")]
#[cfg_attr(target_os = "macos", link_section = "__DATA,__mod_init_func")]
static INIT_ARRAY: [unsafe extern "C" fn(); 1] = [run_static_initializers];
