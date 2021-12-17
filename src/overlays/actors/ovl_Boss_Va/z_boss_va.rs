#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case,
         non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(const_raw_ptr_to_usize_cast, const_transmute, register_tool)]
extern "C" {
    #[no_mangle]
    fn func_80026860(globalCtx: *mut GlobalContext, color: *mut Color_RGBA8,
                     arg2: s16, arg3: s16);
    #[no_mangle]
    fn func_80026A6C(globalCtx: *mut GlobalContext);
    #[no_mangle]
    fn EffectSsDtBubble_SpawnColorProfile(globalCtx: *mut GlobalContext,
                                          pos: *mut Vec3f,
                                          velocity: *mut Vec3f,
                                          accel: *mut Vec3f, scale: s16,
                                          life: s16, colorProfile: s16,
                                          randXZ: s16);
    #[no_mangle]
    fn EffectSsDeadSound_SpawnStationary(globalCtx: *mut GlobalContext,
                                         pos: *mut Vec3f, sfxId: u16_0,
                                         lowerPriority: s16, repeatMode: s16,
                                         life: s32);
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
    fn Actor_SetScale(actor: *mut Actor, scale: f32_0);
    #[no_mangle]
    fn Actor_MoveForward(actor: *mut Actor);
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
    fn func_80033C30(arg0: *mut Vec3f, arg1: *mut Vec3f, alpha: u8_0,
                     globalCtx: *mut GlobalContext);
    #[no_mangle]
    fn Rand_ZeroFloat(f: f32_0) -> f32_0;
    #[no_mangle]
    fn Rand_CenteredFloat(f: f32_0) -> f32_0;
    #[no_mangle]
    fn Actor_SetColorFilter(actor: *mut Actor, colorFlag: s16,
                            colorIntensityMax: s16, xluFlag: s16,
                            duration: s16);
    #[no_mangle]
    fn func_80035844(arg0: *mut Vec3f, arg1: *mut Vec3f, arg2: *mut Vec3s,
                     arg3: s32);
    #[no_mangle]
    fn BgCheck_EntityRaycastFloor1(colCtx: *mut CollisionContext,
                                   outPoly: *mut *mut CollisionPoly,
                                   pos: *mut Vec3f) -> f32_0;
    #[no_mangle]
    fn Camera_AddQuake(camera: *mut Camera, arg1: s32, y: s16, countdown: s32)
     -> s32;
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
    fn Collider_InitQuad(globalCtx: *mut GlobalContext,
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
    fn Collider_UpdateSpheres(limb: s32, collider: *mut ColliderJntSph);
    #[no_mangle]
    fn func_80064520(globalCtx: *mut GlobalContext,
                     csCtx: *mut CutsceneContext);
    #[no_mangle]
    fn func_80064534(globalCtx: *mut GlobalContext,
                     csCtx: *mut CutsceneContext);
    #[no_mangle]
    fn Lib_MemSet(dest: *mut u8_0, size: size_t, val: u8_0);
    #[no_mangle]
    fn Math_CosS(angle: s16) -> f32_0;
    #[no_mangle]
    fn Math_SinS(angle: s16) -> f32_0;
    #[no_mangle]
    fn Math_ScaledStepToS(pValue: *mut s16, target: s16, step: s16) -> s32;
    #[no_mangle]
    fn Rand_S16Offset(base: s16, range: s16) -> s16;
    #[no_mangle]
    fn Math_Vec3f_DistXYZ(a: *mut Vec3f, b: *mut Vec3f) -> f32_0;
    #[no_mangle]
    fn Math_Vec3f_DistXZ(a: *mut Vec3f, b: *mut Vec3f) -> f32_0;
    #[no_mangle]
    fn Math_Vec3f_Yaw(a: *mut Vec3f, b: *mut Vec3f) -> s16;
    #[no_mangle]
    fn Math_Vec3f_Pitch(a: *mut Vec3f, b: *mut Vec3f) -> s16;
    #[no_mangle]
    fn Math_SmoothStepToF(pValue: *mut f32_0, target: f32_0, fraction: f32_0,
                          step: f32_0, minStep: f32_0) -> f32_0;
    #[no_mangle]
    fn Math_SmoothStepToS(pValue: *mut s16, target: s16, scale: s16,
                          step: s16, minStep: s16) -> s16;
    #[no_mangle]
    fn func_80093C14(gfxCtx: *mut GraphicsContext);
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
    fn SkelAnime_DrawOpa(globalCtx: *mut GlobalContext,
                         skeleton: *mut *mut libc::c_void,
                         jointTable: *mut Vec3s,
                         overrideLimbDraw: OverrideLimbDrawOpa,
                         postLimbDraw: PostLimbDrawOpa,
                         arg: *mut libc::c_void);
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
    fn SkelAnime_Init(globalCtx: *mut GlobalContext,
                      skelAnime: *mut SkelAnime,
                      skeletonHeaderSeg: *mut SkeletonHeader,
                      animation: *mut AnimationHeader, jointTable: *mut Vec3s,
                      morphTable: *mut Vec3s, limbCount: s32) -> s32;
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
    fn SkelAnime_Free(skelAnime: *mut SkelAnime,
                      globalCtx: *mut GlobalContext);
    #[no_mangle]
    fn Gameplay_CreateSubCamera(globalCtx: *mut GlobalContext) -> s16;
    #[no_mangle]
    fn Gameplay_ChangeCameraStatus(globalCtx: *mut GlobalContext, camId: s16,
                                   status: s16) -> s16;
    #[no_mangle]
    fn Gameplay_ClearCamera(globalCtx: *mut GlobalContext, camId: s16);
    #[no_mangle]
    fn Gameplay_GetCamera(globalCtx: *mut GlobalContext, camId: s16)
     -> *mut Camera;
    #[no_mangle]
    fn Gameplay_CameraSetAtEye(globalCtx: *mut GlobalContext, camId: s16,
                               at: *mut Vec3f, eye: *mut Vec3f) -> s32;
    #[no_mangle]
    fn Graph_OpenDisps(dispRefs: *mut *mut Gfx, gfxCtx: *mut GraphicsContext,
                       file: *const libc::c_char, line: s32);
    #[no_mangle]
    fn Graph_CloseDisps(dispRefs: *mut *mut Gfx, gfxCtx: *mut GraphicsContext,
                        file: *const libc::c_char, line: s32);
    #[no_mangle]
    fn Math_SinF(angle: f32_0) -> f32_0;
    #[no_mangle]
    fn Matrix_Push();
    #[no_mangle]
    fn Matrix_Pop();
    #[no_mangle]
    fn Matrix_Get(dest: *mut MtxF);
    #[no_mangle]
    fn Matrix_Put(src: *mut MtxF);
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
    fn Matrix_RotateZYX(x: s16, y: s16, z: s16, mode: u8_0);
    #[no_mangle]
    fn Matrix_NewMtx(gfxCtx: *mut GraphicsContext, file: *mut libc::c_char,
                     line: s32) -> *mut Mtx;
    #[no_mangle]
    fn Matrix_MultVec3f(src: *mut Vec3f, dest: *mut Vec3f);
    #[no_mangle]
    fn func_800D1FD4(mf: *mut MtxF);
    #[no_mangle]
    fn Matrix_MtxFToZYXRotS(mf: *mut MtxF, rotDest: *mut Vec3s, flag: s32);
    #[no_mangle]
    fn func_800F436C(pos: *mut Vec3f, sfxId: u16_0, arg2: f32_0);
    #[no_mangle]
    fn Audio_PlaySoundGeneral(sfxId: u16_0, pos: *mut Vec3f, token: u8_0,
                              freqScale: *mut f32_0, a4: *mut f32_0,
                              reverbAdd: *mut s8);
    #[no_mangle]
    fn Audio_QueueSeqCmd(bgmID: u32_0);
    #[no_mangle]
    fn Rand_ZeroOne() -> f32_0;
    #[no_mangle]
    static mut gSegments: [u32_0; 16];
    #[no_mangle]
    static mut gGameInfo: *mut GameInfo;
    #[no_mangle]
    static mut gSaveContext: SaveContext;
    #[no_mangle]
    static mut D_801333E8: s8;
    #[no_mangle]
    static mut D_801333E0: f32_0;
    #[no_mangle]
    static mut gBarinadeBariAnim: AnimationHeader;
    #[no_mangle]
    static mut gBarinadeDL_000FA0: [Gfx; 0];
    #[no_mangle]
    static mut gBarinadeTitleCardTex: [u64_0; 0];
    #[no_mangle]
    static mut gBarinadeBariSkel: SkeletonHeader;
    #[no_mangle]
    static mut gBarinadeBodyAnim: AnimationHeader;
    #[no_mangle]
    static mut gBarinadeDL_008BB8: [Gfx; 0];
    #[no_mangle]
    static mut gBarinadeDL_008D70: [Gfx; 0];
    #[no_mangle]
    static mut gBarinadeDL_008F08: [Gfx; 0];
    #[no_mangle]
    static mut gBarinadeDL_008F70: [Gfx; 0];
    #[no_mangle]
    static mut gBarinadeDL_009430: [Gfx; 0];
    #[no_mangle]
    static mut gBarinadeDL_009468: [Gfx; 0];
    #[no_mangle]
    static mut gBarinadeSparkBall1Tex: [u64_0; 0];
    #[no_mangle]
    static mut gBarinadeSparkBall2Tex: [u64_0; 0];
    #[no_mangle]
    static mut gBarinadeSparkBall3Tex: [u64_0; 0];
    #[no_mangle]
    static mut gBarinadeSparkBall4Tex: [u64_0; 0];
    #[no_mangle]
    static mut gBarinadeSparkBall5Tex: [u64_0; 0];
    #[no_mangle]
    static mut gBarinadeSparkBall6Tex: [u64_0; 0];
    #[no_mangle]
    static mut gBarinadeSparkBall7Tex: [u64_0; 0];
    #[no_mangle]
    static mut gBarinadeSparkBall8Tex: [u64_0; 0];
    #[no_mangle]
    static mut gBarinadeDL_011738: [Gfx; 0];
    #[no_mangle]
    static mut gBarinadeDL_011768: [Gfx; 0];
    #[no_mangle]
    static mut gBarinadeDL_0128B8: [Gfx; 0];
    #[no_mangle]
    static mut gBarinadeDL_012948: [Gfx; 0];
    #[no_mangle]
    static mut gBarinadeDL_012BA0: [Gfx; 0];
    #[no_mangle]
    static mut gBarinadeDL_012C50: [Gfx; 0];
    #[no_mangle]
    static mut gBarinadeDL_0135B0: [Gfx; 0];
    #[no_mangle]
    static mut gBarinadeDL_013638: [Gfx; 0];
    #[no_mangle]
    static mut gBarinadeDL_0156A0: [Gfx; 0];
    #[no_mangle]
    static mut gBarinadeDL_015710: [Gfx; 0];
    #[no_mangle]
    static mut gBarinadeBodySkel: SkeletonHeader;
    #[no_mangle]
    static mut gBarinadeSupportDamage1Anim: AnimationHeader;
    #[no_mangle]
    static mut gBarinadeSupportDamage2Anim: AnimationHeader;
    #[no_mangle]
    static mut gBarinadeSupportAttachedAnim: AnimationHeader;
    #[no_mangle]
    static mut gBarinadeSupportSkel: FlexSkeletonHeader;
    #[no_mangle]
    static mut gBarinadeSupportCutAnim: AnimationHeader;
    #[no_mangle]
    static mut gBarinadeSupportDetachedAnim: AnimationHeader;
    #[no_mangle]
    static mut gBarinadeCutSupportSkel: FlexSkeletonHeader;
    #[no_mangle]
    static mut gBarinadeStumpAnim: AnimationHeader;
    #[no_mangle]
    static mut gBarinadeStumpSkel: FlexSkeletonHeader;
    #[no_mangle]
    static mut gBarinadeZapperDamage1Anim: AnimationHeader;
    #[no_mangle]
    static mut gBarinadeZapperDamage2Anim: AnimationHeader;
    #[no_mangle]
    static mut gBarinadeZapperIdleAnim: AnimationHeader;
    #[no_mangle]
    static mut gBarinadeZapperSkel: FlexSkeletonHeader;
    #[no_mangle]
    static mut gBarinadeDoorPiece1DL: [Gfx; 0];
    #[no_mangle]
    static mut gBarinadeDoorPiece2DL: [Gfx; 0];
    #[no_mangle]
    static mut gBarinadeDoorPiece3DL: [Gfx; 0];
    #[no_mangle]
    static mut gBarinadeDoorPiece4DL: [Gfx; 0];
    #[no_mangle]
    static mut gBarinadeDoorPiece5DL: [Gfx; 0];
    #[no_mangle]
    static mut gBarinadeDoorPiece6DL: [Gfx; 0];
    #[no_mangle]
    static mut gBarinadeDoorPiece7DL: [Gfx; 0];
    #[no_mangle]
    static mut gBarinadeDoorPiece8DL: [Gfx; 0];
    #[no_mangle]
    static mut gEffBubble1Tex: [u64_0; 0];
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
pub type C2RustUnnamed_21 = libc::c_uint;
pub const SEQ_PLAYER_BGM_SUB: C2RustUnnamed_21 = 3;
pub const SEQ_PLAYER_SFX: C2RustUnnamed_21 = 2;
pub const SEQ_PLAYER_FANFARE: C2RustUnnamed_21 = 1;
pub const SEQ_PLAYER_BGM_MAIN: C2RustUnnamed_21 = 0;
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
pub type C2RustUnnamed_22 = libc::c_uint;
pub const MTXMODE_APPLY: C2RustUnnamed_22 = 1;
pub const MTXMODE_NEW: C2RustUnnamed_22 = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct BossVa {
    pub actor: Actor,
    pub skelAnime: SkelAnime,
    pub actionFunc: BossVaActionFunc,
    pub onCeiling: u8_0,
    pub burst: u8_0,
    pub invincibilityTimer: s8,
    pub isDead: u8_0,
    pub timer: s32,
    pub timer2: s16,
    pub unk_1A0: f32_0,
    pub unk_1A4: f32_0,
    pub unk_1A8: f32_0,
    pub unk_1AC: s16,
    pub bodyGlow: s16,
    pub unk_1B0: s16,
    pub armTip: Vec3f,
    pub zapNeckPos: Vec3f,
    pub zapHeadPos: Vec3f,
    pub unk_1D8: Vec3f,
    pub unk_1E4: s16,
    pub unk_1E6: s16,
    pub unk_1E8: s16,
    pub unk_1EA: s16,
    pub unk_1EC: s16,
    pub unk_1EE: s16,
    pub unk_1F0: s16,
    pub unk_1F2: s16,
    pub unk_1F4: s16,
    pub headRot: Vec3s,
    pub effectPos: [Vec3f; 10],
    pub unk_274: Vec3f,
    pub unk_280: Vec3f,
    pub colliderBody: ColliderCylinder,
    pub colliderSph: ColliderJntSph,
    pub elements: [ColliderJntSphElement; 1],
    pub colliderLightning: ColliderQuad,
}
pub type BossVaActionFunc
    =
    Option<unsafe extern "C" fn(_: *mut BossVa, _: *mut GlobalContext) -> ()>;
pub type C2RustUnnamed_23 = libc::c_int;
pub const BOSSVA_DOOR: C2RustUnnamed_23 = 19;
pub const BOSSVA_STUMP_3: C2RustUnnamed_23 = 18;
pub const BOSSVA_STUMP_2: C2RustUnnamed_23 = 17;
pub const BOSSVA_STUMP_1: C2RustUnnamed_23 = 16;
pub const BOSSVA_BARI_LOWER_5: C2RustUnnamed_23 = 15;
pub const BOSSVA_BARI_LOWER_4: C2RustUnnamed_23 = 14;
pub const BOSSVA_BARI_LOWER_3: C2RustUnnamed_23 = 13;
pub const BOSSVA_BARI_LOWER_2: C2RustUnnamed_23 = 12;
pub const BOSSVA_BARI_LOWER_1: C2RustUnnamed_23 = 11;
pub const BOSSVA_BARI_UPPER_5: C2RustUnnamed_23 = 10;
pub const BOSSVA_BARI_UPPER_4: C2RustUnnamed_23 = 9;
pub const BOSSVA_BARI_UPPER_3: C2RustUnnamed_23 = 8;
pub const BOSSVA_BARI_UPPER_2: C2RustUnnamed_23 = 7;
pub const BOSSVA_BARI_UPPER_1: C2RustUnnamed_23 = 6;
pub const BOSSVA_ZAPPER_3: C2RustUnnamed_23 = 5;
pub const BOSSVA_ZAPPER_2: C2RustUnnamed_23 = 4;
pub const BOSSVA_ZAPPER_1: C2RustUnnamed_23 = 3;
pub const BOSSVA_SUPPORT_3: C2RustUnnamed_23 = 2;
pub const BOSSVA_SUPPORT_2: C2RustUnnamed_23 = 1;
pub const BOSSVA_SUPPORT_1: C2RustUnnamed_23 = 0;
pub const BOSSVA_BODY: C2RustUnnamed_23 = -1;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct EnBoom {
    pub actor: Actor,
    pub collider: ColliderQuad,
    pub moveTo: *mut Actor,
    pub grabbed: *mut Actor,
    pub returnTimer: u8_0,
    pub activeTimer: u8_0,
    pub effectIndex: s32,
    pub boomerangInfo: WeaponInfo,
    pub actionFunc: EnBoomActionFunc,
}
pub type EnBoomActionFunc
    =
    Option<unsafe extern "C" fn(_: *mut EnBoom, _: *mut GlobalContext) -> ()>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct BossVaEffect {
    pub pos: Vec3f,
    pub velocity: Vec3f,
    pub accel: Vec3f,
    pub type_0: u8_0,
    pub timer: u16_0,
    pub mode: s16,
    pub rot: Vec3s,
    pub primColor: [s16; 4],
    pub envColor: [s16; 4],
    pub scale: f32_0,
    pub scaleMod: f32_0,
    pub offset: Vec3f,
    pub parent: *mut BossVa,
}
pub type C2RustUnnamed_24 = libc::c_uint;
pub const VA_GORE: C2RustUnnamed_24 = 8;
pub const VA_TUMOR: C2RustUnnamed_24 = 7;
pub const VA_BLOOD: C2RustUnnamed_24 = 6;
pub const VA_ZAP_CHARGE: C2RustUnnamed_24 = 5;
pub const VA_SPARK_BALL: C2RustUnnamed_24 = 4;
pub const VA_SMALL_SPARK: C2RustUnnamed_24 = 3;
pub const VA_BLAST_SPARK: C2RustUnnamed_24 = 2;
pub const VA_LARGE_SPARK: C2RustUnnamed_24 = 1;
pub const VA_NONE: C2RustUnnamed_24 = 0;
pub type C2RustUnnamed_25 = libc::c_uint;
pub const SPARK_LINK: C2RustUnnamed_25 = 6;
pub const SPARK_BODY: C2RustUnnamed_25 = 5;
pub const SPARK_UNUSED: C2RustUnnamed_25 = 4;
pub const SPARK_BLAST: C2RustUnnamed_25 = 3;
pub const SPARK_BARI: C2RustUnnamed_25 = 2;
pub const SPARK_TETHER: C2RustUnnamed_25 = 1;
pub type C2RustUnnamed_26 = libc::c_uint;
pub const BLOOD_SPOT: C2RustUnnamed_26 = 2;
pub const BLOOD_SPLATTER: C2RustUnnamed_26 = 1;
pub const BLOOD_DROPLET: C2RustUnnamed_26 = 0;
pub type C2RustUnnamed_27 = libc::c_uint;
pub const TUMOR_ARM: C2RustUnnamed_27 = 2;
pub const TUMOR_BODY: C2RustUnnamed_27 = 1;
pub const TUMOR_UNUSED: C2RustUnnamed_27 = 0;
pub type C2RustUnnamed_28 = libc::c_uint;
pub const GORE_FADING: C2RustUnnamed_28 = 2;
pub const GORE_FLOOR: C2RustUnnamed_28 = 1;
pub const GORE_PERMANENT: C2RustUnnamed_28 = 0;
pub type C2RustUnnamed_29 = libc::c_int;
pub const DEATH_FINISH: C2RustUnnamed_29 = 24;
pub const DEATH_MUSIC: C2RustUnnamed_29 = 23;
pub const DEATH_CORE_BURST: C2RustUnnamed_29 = 22;
pub const DEATH_CORE_DEAD: C2RustUnnamed_29 = 21;
pub const DEATH_CORE_TUMORS: C2RustUnnamed_29 = 20;
pub const DEATH_SHELL_BURST: C2RustUnnamed_29 = 19;
pub const DEATH_ZAPPER_3: C2RustUnnamed_29 = 18;
pub const DEATH_ZAPPER_2: C2RustUnnamed_29 = 17;
pub const DEATH_ZAPPER_1: C2RustUnnamed_29 = 16;
pub const DEATH_BODY_TUMORS: C2RustUnnamed_29 = 15;
pub const DEATH_START: C2RustUnnamed_29 = 14;
pub const BOSSVA_BATTLE: C2RustUnnamed_29 = 13;
pub const INTRO_FINISH: C2RustUnnamed_29 = 12;
pub const INTRO_BRIGHTEN: C2RustUnnamed_29 = 11;
pub const INTRO_TITLE: C2RustUnnamed_29 = 10;
pub const INTRO_ATTACH_BARI: C2RustUnnamed_29 = 9;
pub const INTRO_CALL_BARI: C2RustUnnamed_29 = 8;
pub const INTRO_UNUSED_CALL_BARI: C2RustUnnamed_29 = 7;
pub const INTRO_LOOK_SUPPORT: C2RustUnnamed_29 = 6;
pub const INTRO_BODY_SOUND: C2RustUnnamed_29 = 5;
pub const INTRO_SUPPORT_CAMERA: C2RustUnnamed_29 = 4;
pub const INTRO_REVERSE_CAMERA: C2RustUnnamed_29 = 3;
pub const INTRO_LOOK_BARI: C2RustUnnamed_29 = 2;
pub const INTRO_SPAWN_BARI: C2RustUnnamed_29 = 1;
pub const INTRO_CRACKLE: C2RustUnnamed_29 = 0;
pub const INTRO_DOOR_SHUT: C2RustUnnamed_29 = -1;
pub const INTRO_CLOSE_DOOR: C2RustUnnamed_29 = -2;
pub const INTRO_LOOK_DOOR: C2RustUnnamed_29 = -3;
pub const INTRO_START: C2RustUnnamed_29 = -4;
pub const INTRO_UNUSED_START: C2RustUnnamed_29 = -5;
#[no_mangle]
pub static mut Boss_Va_InitVars: ActorInit =
    unsafe {
        {
            let mut init =
                ActorInit{id: ACTOR_BOSS_VA as libc::c_int as s16,
                          category: ACTORCAT_BOSS as libc::c_int as u8_0,
                          flags:
                              ((1 as libc::c_int) << 0 as libc::c_int |
                                   (1 as libc::c_int) << 2 as libc::c_int |
                                   (1 as libc::c_int) << 4 as libc::c_int |
                                   (1 as libc::c_int) << 5 as libc::c_int) as
                                  u32_0,
                          objectId: OBJECT_BV as libc::c_int as s16,
                          instanceSize:
                              ::std::mem::size_of::<BossVa>() as
                                  libc::c_ulong,
                          init:
                              ::std::mem::transmute::<Option<unsafe extern "C" fn(_:
                                                                                      *mut Actor,
                                                                                  _:
                                                                                      *mut GlobalContext)
                                                                 -> ()>,
                                                      ActorFunc>(Some(BossVa_Init
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
                                                      ActorFunc>(Some(BossVa_Destroy
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
                                                      ActorFunc>(Some(BossVa_Update
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
                                                      ActorFunc>(Some(BossVa_Draw
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
static mut sCylinderInit: ColliderCylinderInit =
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
                                                                   5 as
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
                                                                                            0xffcfffef
                                                                                                as
                                                                                                libc::c_uint,
                                                                                        effect:
                                                                                            0x3
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
                                                                                               0x10
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
                                                            85 as libc::c_int
                                                                as s16,
                                                        height:
                                                            120 as libc::c_int
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
static mut sJntSphElementsInitSupport: [ColliderJntSphElementInit; 1] =
    [{
         let mut init =
             ColliderJntSphElementInit{info:
                                           {
                                               let mut init =
                                                   ColliderInfoInit{elemType:
                                                                        ELEMTYPE_UNK0
                                                                            as
                                                                            libc::c_int
                                                                            as
                                                                            u8_0,
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
                                                                                                     0x10
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
                                                                        0 as
                                                                            libc::c_int
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
                                                                        0 as
                                                                            libc::c_int
                                                                            as
                                                                            u8_0,};
                                               init
                                           },
                                       dim:
                                           {
                                               let mut init =
                                                   ColliderJntSphElementDimInit{limb:
                                                                                    0
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
     }];
// Initialized in run_static_initializers
static mut sJntSphInitSupport: ColliderJntSphInit =
    ColliderJntSphInit{base:
                           ColliderInit{colType: 0,
                                        atFlags: 0,
                                        acFlags: 0,
                                        ocFlags1: 0,
                                        ocFlags2: 0,
                                        shape: 0,},
                       count: 0,
                       elements:
                           0 as *const ColliderJntSphElementInit as
                               *mut ColliderJntSphElementInit,};
static mut sJntSphElementsInitBari: [ColliderJntSphElementInit; 1] =
    [{
         let mut init =
             ColliderJntSphElementInit{info:
                                           {
                                               let mut init =
                                                   ColliderInfoInit{elemType:
                                                                        ELEMTYPE_UNK0
                                                                            as
                                                                            libc::c_int
                                                                            as
                                                                            u8_0,
                                                                    toucher:
                                                                        {
                                                                            let mut init =
                                                                                ColliderTouch{dmgFlags:
                                                                                                  0xffcfffff
                                                                                                      as
                                                                                                      libc::c_uint,
                                                                                              effect:
                                                                                                  0x3
                                                                                                      as
                                                                                                      libc::c_int
                                                                                                      as
                                                                                                      u8_0,
                                                                                              damage:
                                                                                                  0x4
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
                                                                        0 as
                                                                            libc::c_int
                                                                            as
                                                                            u8_0,};
                                               init
                                           },
                                       dim:
                                           {
                                               let mut init =
                                                   ColliderJntSphElementDimInit{limb:
                                                                                    0
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
                                                                                                         30
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
// Initialized in run_static_initializers
static mut sJntSphInitBari: ColliderJntSphInit =
    ColliderJntSphInit{base:
                           ColliderInit{colType: 0,
                                        atFlags: 0,
                                        acFlags: 0,
                                        ocFlags1: 0,
                                        ocFlags2: 0,
                                        shape: 0,},
                       count: 0,
                       elements:
                           0 as *const ColliderJntSphElementInit as
                               *mut ColliderJntSphElementInit,};
static mut sQuadInit: ColliderQuadInit =
    {
        let mut init =
            ColliderQuadInit{base:
                                 {
                                     let mut init =
                                         ColliderInit{colType:
                                                          COLTYPE_METAL as
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
                                                              u8_0,}; // ! params could be WARP_DUNGEON_CHILD however this can also spawn Ru1
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
                                                                                        0x20000000
                                                                                            as
                                                                                            libc::c_int
                                                                                            as
                                                                                            u32_0,
                                                                                    effect:
                                                                                        0x3
                                                                                            as
                                                                                            libc::c_int
                                                                                            as
                                                                                            u8_0,
                                                                                    damage:
                                                                                        0x4
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
                                                                                           0x10
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
static mut sInitPosOffsets: [Vec3f; 26] =
    [{ let mut init = Vec3f{x: 0.0f32, y: 175.35f32, z: 0.0f32,}; init },
     { let mut init = Vec3f{x: 0.0f32, y: 175.35f32, z: 0.0f32,}; init },
     { let mut init = Vec3f{x: 0.0f32, y: 175.35f32, z: 0.0f32,}; init },
     { let mut init = Vec3f{x: 120.0f32, y: 103.425f32, z: -67.0f32,}; init },
     { let mut init = Vec3f{x: 0.0f32, y: 103.425f32, z: 140.0f32,}; init },
     {
         let mut init = Vec3f{x: -120.0f32, y: 103.425f32, z: -70.0f32,};
         init
     }, { let mut init = Vec3f{x: -2.0f32, y: 16.0f32, z: 50.0f32,}; init },
     { let mut init = Vec3f{x: 48.0f32, y: 16.0f32, z: 15.0f32,}; init },
     { let mut init = Vec3f{x: 25.0f32, y: 16.0f32, z: -36.0f32,}; init },
     { let mut init = Vec3f{x: -29.0f32, y: 16.0f32, z: -36.0f32,}; init },
     { let mut init = Vec3f{x: -63.0f32, y: 16.0f32, z: 22.0f32,}; init },
     { let mut init = Vec3f{x: 0.0f32, y: -10.0f32, z: -64.0f32,}; init },
     { let mut init = Vec3f{x: 63.0f32, y: -10.0f32, z: -22.0f32,}; init },
     { let mut init = Vec3f{x: 35.0f32, y: -10.0f32, z: 46.0f32,}; init },
     { let mut init = Vec3f{x: -36.0f32, y: -10.0f32, z: 46.0f32,}; init },
     { let mut init = Vec3f{x: -49.0f32, y: -10.0f32, z: -17.0f32,}; init },
     { let mut init = Vec3f{x: 0.0f32, y: 160.0f32, z: 370.0f32,}; init },
     { let mut init = Vec3f{x: 65.0f32, y: 35.0f32, z: 370.0f32,}; init },
     { let mut init = Vec3f{x: 80.0f32, y: 70.0f32, z: -130.0f32,}; init },
     { let mut init = Vec3f{x: -160.0f32, y: 100.0f32, z: -130.0f32,}; init },
     { let mut init = Vec3f{x: -150.0f32, y: 130.0f32, z: 0.0f32,}; init },
     { let mut init = Vec3f{x: 230.0f32, y: 0.0f32, z: 0.0f32,}; init },
     { let mut init = Vec3f{x: 60.0f32, y: 140.0f32, z: 0.0f32,}; init },
     { let mut init = Vec3f{x: 0.0f32, y: 40.0f32, z: 270.0f32,}; init },
     { let mut init = Vec3f{x: -100.0f32, y: 10.0f32, z: 200.0f32,}; init },
     { let mut init = Vec3f{x: -90.0f32, y: 70.0f32, z: -310.0f32,}; init }];
static mut sInitRot: [Vec3s; 16] =
    [{
         let mut init =
             Vec3s{x: 0x1ffe as libc::c_int as s16,
                   y: 0 as libc::c_int as s16,
                   z: 0 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             Vec3s{x: 0x1ffe as libc::c_int as s16,
                   y: 0x5550 as libc::c_int as s16,
                   z: 0 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             Vec3s{x: 0x1ffe as libc::c_int as s16,
                   y: 0xaab0 as libc::c_int as s16,
                   z: 0 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             Vec3s{x: 0xd558 as libc::c_int as s16,
                   y: 0x5550 as libc::c_int as s16,
                   z: 0 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             Vec3s{x: 0xd558 as libc::c_int as s16,
                   y: 0 as libc::c_int as s16,
                   z: 0 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             Vec3s{x: 0xd558 as libc::c_int as s16,
                   y: 0xaab0 as libc::c_int as s16,
                   z: 0 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             Vec3s{x: 0x2aa8 as libc::c_int as s16,
                   y: 0xfccc as libc::c_int as s16,
                   z: 0 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             Vec3s{x: 0x2aa8 as libc::c_int as s16,
                   y: 0x3330 as libc::c_int as s16,
                   z: 0 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             Vec3s{x: 0x2aa8 as libc::c_int as s16,
                   y: 0x6660 as libc::c_int as s16,
                   z: 0 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             Vec3s{x: 0x2aa8 as libc::c_int as s16,
                   y: 0x99a0 as libc::c_int as s16,
                   z: 0 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             Vec3s{x: 0x2aa8 as libc::c_int as s16,
                   y: 0xccd0 as libc::c_int as s16,
                   z: 0 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             Vec3s{x: 0x4c98 as libc::c_int as s16,
                   y: 0x81d0 as libc::c_int as s16,
                   z: 0 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             Vec3s{x: 0x4c98 as libc::c_int as s16,
                   y: 0x4f70 as libc::c_int as s16,
                   z: 0 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             Vec3s{x: 0x4c98 as libc::c_int as s16,
                   y: 0x1758 as libc::c_int as s16,
                   z: 0 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             Vec3s{x: 0x4c98 as libc::c_int as s16,
                   y: 0xe8a8 as libc::c_int as s16,
                   z: 0 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             Vec3s{x: 0x4c98 as libc::c_int as s16,
                   y: 0xb648 as libc::c_int as s16,
                   z: 0 as libc::c_int as s16,};
         init
     }];
static mut sWarpPos: [Vec3f; 3] =
    [{ let mut init = Vec3f{x: 10.0f32, y: 0.0f32, z: 30.0f32,}; init },
     { let mut init = Vec3f{x: 260.0f32, y: 0.0f32, z: -470.0f32,}; init },
     { let mut init = Vec3f{x: -240.0f32, y: 0.0f32, z: -470.0f32,}; init }];
static mut sDamageTable: [DamageTable; 1] =
    [{
         let mut init =
             DamageTable{table:
                             [(0 as libc::c_int |
                                   (0x1 as libc::c_int) << 4 as libc::c_int)
                                  as u8_0,
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
                                   (0x1 as libc::c_int) << 4 as libc::c_int)
                                  as u8_0,
                              (2 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (2 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (0 as libc::c_int |
                                   (0x1 as libc::c_int) << 4 as libc::c_int)
                                  as u8_0,
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
                              (2 as libc::c_int |
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
                              (2 as libc::c_int |
                                   (0 as libc::c_int) << 4 as libc::c_int) as
                                  u8_0,
                              (0 as libc::c_int |
                                   (0xe as libc::c_int) << 4 as libc::c_int)
                                  as u8_0,
                              (0 as libc::c_int |
                                   (0x6 as libc::c_int) << 4 as libc::c_int)
                                  as u8_0,
                              (0 as libc::c_int |
                                   (0xd as libc::c_int) << 4 as libc::c_int)
                                  as u8_0,
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
     }];
static mut sZeroVec: Vec3f =
    { let mut init = Vec3f{x: 0.0f32, y: 0.0f32, z: 0.0f32,}; init };
static mut sKillBari: u8_0 = 0 as libc::c_int as u8_0;
static mut sBodyBari: [u8_0; 10] =
    [0 as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
     0 as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
     0 as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
     0 as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
     0 as libc::c_int as u8_0, 0 as libc::c_int as u8_0];
static mut sCsCamera: s16 = 0 as libc::c_int as s16;
static mut sVaEffects: [BossVaEffect; 400] =
    [BossVaEffect{pos: Vec3f{x: 0., y: 0., z: 0.,},
                  velocity: Vec3f{x: 0., y: 0., z: 0.,},
                  accel: Vec3f{x: 0., y: 0., z: 0.,},
                  type_0: 0,
                  timer: 0,
                  mode: 0,
                  rot: Vec3s{x: 0, y: 0, z: 0,},
                  primColor: [0; 4],
                  envColor: [0; 4],
                  scale: 0.,
                  scaleMod: 0.,
                  offset: Vec3f{x: 0., y: 0., z: 0.,},
                  parent: 0 as *const BossVa as *mut BossVa,}; 400];
static mut sBodyState: u8_0 = 0;
static mut sFightPhase: u8_0 = 0;
static mut sCsState: s8 = 0;
static mut sCameraEye: Vec3f = Vec3f{x: 0., y: 0., z: 0.,};
static mut sCameraAt: Vec3f = Vec3f{x: 0., y: 0., z: 0.,};
static mut sCameraNextEye: Vec3f = Vec3f{x: 0., y: 0., z: 0.,};
static mut sCameraNextAt: Vec3f = Vec3f{x: 0., y: 0., z: 0.,};
static mut sCameraEyeMaxVel: Vec3f = Vec3f{x: 0., y: 0., z: 0.,};
static mut sCameraAtMaxVel: Vec3f = Vec3f{x: 0., y: 0., z: 0.,};
static mut sDoorState: s16 = 0;
static mut sPhase3StopMoving: u8_0 = 0;
static mut sZapperRot: Vec3s = Vec3s{x: 0, y: 0, z: 0,};
static mut sPhase2Timer: u16_0 = 0;
static mut sPhase4HP: s8 = 0;
#[no_mangle]
pub unsafe extern "C" fn BossVa_SetupAction(mut this: *mut BossVa,
                                            mut func: BossVaActionFunc) {
    (*this).actionFunc = func;
}
#[no_mangle]
pub unsafe extern "C" fn BossVa_AttachToBody(mut this: *mut BossVa) {
    let mut vaBody: *mut BossVa = (*this).actor.parent as *mut BossVa;
    Matrix_Translate((*vaBody).actor.world.pos.x, (*vaBody).actor.world.pos.y,
                     (*vaBody).actor.world.pos.z,
                     MTXMODE_NEW as libc::c_int as u8_0);
    Matrix_RotateZYX((*vaBody).actor.shape.rot.x, 0 as libc::c_int as s16,
                     (*vaBody).actor.shape.rot.z,
                     MTXMODE_APPLY as libc::c_int as u8_0);
    Matrix_MultVec3f(&mut *sInitPosOffsets.as_mut_ptr().offset((*this).actor.params
                                                                   as isize),
                     &mut (*this).actor.world.pos);
    match (*this).actor.params as libc::c_int {
        0 | 1 | 2 => {
            if (*this).onCeiling == 0 {
                (*this).actor.shape.rot.x =
                    (sInitRot[(*this).actor.params as usize].x as libc::c_int
                         + (*vaBody).actor.shape.rot.x as libc::c_int) as s16;
                (*this).actor.shape.rot.y =
                    sInitRot[(*this).actor.params as usize].y;
                (*this).actor.shape.rot.z =
                    (sInitRot[(*this).actor.params as usize].z as libc::c_int
                         + (*vaBody).actor.shape.rot.z as libc::c_int) as s16
            }
        }
        3 | 4 | 5 => {
            (*this).actor.shape.rot.y =
                sInitRot[(*this).actor.params as usize].y;
            (*this).actor.shape.rot.x =
                (sInitRot[(*this).actor.params as usize].x as libc::c_int +
                     (Math_CosS(sInitRot[(*this).actor.params as usize].y) *
                          (*vaBody).actor.shape.rot.x as libc::c_int as
                              libc::c_float) as s16 as libc::c_int -
                     (Math_SinS(sInitRot[(*this).actor.params as usize].y) *
                          (*vaBody).actor.shape.rot.z as libc::c_int as
                              libc::c_float) as s16 as libc::c_int) as s16;
            (*this).actor.shape.rot.z =
                ((Math_CosS(sInitRot[(*this).actor.params as usize].y) *
                      (*vaBody).actor.shape.rot.z as libc::c_int as
                          libc::c_float) as s16 as libc::c_int +
                     (sInitRot[(*this).actor.params as usize].z as libc::c_int
                          +
                          (Math_SinS(sInitRot[(*this).actor.params as
                                                  usize].y) *
                               (*vaBody).actor.shape.rot.x as libc::c_int as
                                   libc::c_float) as s16 as libc::c_int)) as
                    s16
        }
        _ => { }
    }
    (*this).actor.world.rot = (*this).actor.shape.rot;
    (*this).actor.shape.yOffset =
        (*((*this).actor.parent as *mut BossVa)).actor.shape.yOffset;
}
#[no_mangle]
pub unsafe extern "C" fn BossVa_BloodDroplets(mut globalCtx:
                                                  *mut GlobalContext,
                                              mut pos: *mut Vec3f,
                                              mut phase: s16, mut yaw: s16) {
    let mut i: s32 = 0;
    let mut spawnPos: Vec3f = Vec3f{x: 0., y: 0., z: 0.,};
    i = 2 as libc::c_int;
    while i > 0 as libc::c_int {
        spawnPos.x = Rand_CenteredFloat(10.0f32) + (*pos).x;
        spawnPos.y = (*pos).y - Rand_ZeroOne() * 15.0f32;
        spawnPos.z = Rand_CenteredFloat(10.0f32) + (*pos).z;
        BossVa_SpawnBloodDroplets(globalCtx, sVaEffects.as_mut_ptr(),
                                  &mut spawnPos, 65 as libc::c_int as s16,
                                  phase, yaw);
        i -= 1
    };
}
#[no_mangle]
pub unsafe extern "C" fn BossVa_BloodSplatter(mut globalCtx:
                                                  *mut GlobalContext,
                                              mut src: *mut BossVaEffect,
                                              mut yaw: s16, mut scale: s16,
                                              mut count: s32) {
    let mut i: s32 = 0;
    let mut pos: Vec3f = Vec3f{x: 0., y: 0., z: 0.,};
    i = count;
    while i > 0 as libc::c_int {
        pos.x = Rand_CenteredFloat(10.0f32) + (*src).pos.x;
        pos.y = (*src).pos.y - Rand_ZeroOne() * 15.0f32;
        pos.z = Rand_CenteredFloat(10.0f32) + (*src).pos.z;
        BossVa_SpawnBloodSplatter(globalCtx, sVaEffects.as_mut_ptr(),
                                  &mut pos,
                                  (Rand_CenteredFloat(0x6590 as libc::c_int as
                                                          f32_0) as s16 as
                                       libc::c_int + yaw as libc::c_int) as
                                      s16, scale);
        i -= 1
    };
}
#[no_mangle]
pub unsafe extern "C" fn BossVa_Gore(mut globalCtx: *mut GlobalContext,
                                     mut src: *mut BossVaEffect, mut yaw: s16,
                                     mut scale: s16) {
    let mut i: s32 = 0;
    let mut pos: Vec3f = Vec3f{x: 0., y: 0., z: 0.,};
    i =
        if sCsState as libc::c_int <= DEATH_SHELL_BURST as libc::c_int {
            2 as libc::c_int
        } else { 1 as libc::c_int };
    while i > 0 as libc::c_int {
        pos.x = Rand_CenteredFloat(10.0f32) + (*src).pos.x;
        pos.y = Rand_CenteredFloat(10.0f32) + (*src).pos.y;
        pos.z = Rand_CenteredFloat(10.0f32) + (*src).pos.z;
        BossVa_SpawnGore(globalCtx, sVaEffects.as_mut_ptr(), &mut pos,
                         (Rand_CenteredFloat(0x6590 as libc::c_int as f32_0)
                              as s16 as libc::c_int + yaw as libc::c_int) as
                             s16, scale);
        i -= 1
    };
}
#[no_mangle]
pub unsafe extern "C" fn BossVa_Spark(mut globalCtx: *mut GlobalContext,
                                      mut this: *mut BossVa, mut count: s32,
                                      mut scale: s16, mut xzSpread: f32_0,
                                      mut ySpread: f32_0, mut mode: u8_0,
                                      mut range: f32_0, mut fixed: u8_0) {
    let mut i: s32 = 0;
    let mut index: s16 = 0;
    let mut offset: Vec3f = Vec3f{x: 0., y: 0., z: 0.,};
    i = count;
    while i > 0 as libc::c_int {
        if fixed == 0 {
            index = (Rand_ZeroOne() * (range - 0.6f32)) as s16
        } else { index = (range - 0.6f32) as s16 }
        offset.x =
            Rand_CenteredFloat(xzSpread) + (*this).effectPos[index as usize].x
                - (*this).actor.world.pos.x;
        offset.y =
            Rand_CenteredFloat(ySpread) + (*this).effectPos[index as usize].y
                - (*this).actor.world.pos.y;
        offset.z =
            Rand_CenteredFloat(xzSpread) + (*this).effectPos[index as usize].z
                - (*this).actor.world.pos.z;
        BossVa_SpawnSpark(globalCtx, sVaEffects.as_mut_ptr(), this,
                          &mut offset, scale, mode);
        i -= 1
    };
}
#[no_mangle]
pub unsafe extern "C" fn BossVa_Tumor(mut globalCtx: *mut GlobalContext,
                                      mut this: *mut BossVa, mut count: s32,
                                      mut scale: s16, mut xzSpread: f32_0,
                                      mut ySpread: f32_0, mut mode: u8_0,
                                      mut range: f32_0, mut fixed: u8_0) {
    let mut index: s16 = 0;
    let mut i: s32 = 0;
    let mut offset: Vec3f = Vec3f{x: 0., y: 0., z: 0.,};
    i = count;
    while i > 0 as libc::c_int {
        if fixed == 0 {
            index = (Rand_ZeroOne() * (range - 0.6f32)) as s16
        } else { index = (range - 0.6f32) as s16 }
        offset.x =
            Rand_CenteredFloat(xzSpread) + (*this).effectPos[index as usize].x
                - (*this).actor.world.pos.x;
        offset.y =
            Rand_CenteredFloat(ySpread) + (*this).effectPos[index as usize].y
                - (*this).actor.world.pos.y;
        offset.z =
            Rand_CenteredFloat(xzSpread) + (*this).effectPos[index as usize].z
                - (*this).actor.world.pos.z;
        BossVa_SpawnTumor(globalCtx, sVaEffects.as_mut_ptr(), this,
                          &mut offset, scale, mode);
        i -= 1
    };
}
#[no_mangle]
pub unsafe extern "C" fn BossVa_SetSparkEnv(mut globalCtx:
                                                *mut GlobalContext) {
    (*globalCtx).envCtx.adjAmbientColor[0 as libc::c_int as usize] =
        0xa as libc::c_int as s16;
    (*globalCtx).envCtx.adjAmbientColor[1 as libc::c_int as usize] =
        0xa as libc::c_int as s16;
    (*globalCtx).envCtx.adjAmbientColor[2 as libc::c_int as usize] =
        0xa as libc::c_int as s16;
    (*globalCtx).envCtx.adjLight1Color[0 as libc::c_int as usize] =
        0x73 as libc::c_int as s16;
    (*globalCtx).envCtx.adjLight1Color[1 as libc::c_int as usize] =
        0x41 as libc::c_int as s16;
    (*globalCtx).envCtx.adjLight1Color[2 as libc::c_int as usize] =
        0x64 as libc::c_int as s16;
    (*globalCtx).envCtx.adjFogColor[0 as libc::c_int as usize] =
        0x78 as libc::c_int as s16;
    (*globalCtx).envCtx.adjFogColor[1 as libc::c_int as usize] =
        0x78 as libc::c_int as s16;
    (*globalCtx).envCtx.adjFogColor[2 as libc::c_int as usize] =
        0x46 as libc::c_int as s16;
}
#[no_mangle]
pub unsafe extern "C" fn BossVa_SetDeathEnv(mut globalCtx:
                                                *mut GlobalContext) {
    (*globalCtx).envCtx.adjFogColor[0 as libc::c_int as usize] =
        0xdc as libc::c_int as s16;
    (*globalCtx).envCtx.adjFogColor[1 as libc::c_int as usize] =
        0xdc as libc::c_int as s16;
    (*globalCtx).envCtx.adjFogColor[2 as libc::c_int as usize] =
        0x96 as libc::c_int as s16;
    (*globalCtx).envCtx.adjFogNear = -(0x3e8 as libc::c_int) as s16;
    (*globalCtx).envCtx.adjFogFar = -(0x384 as libc::c_int) as s16;
    (*globalCtx).envCtx.adjAmbientColor[0 as libc::c_int as usize] =
        0xc8 as libc::c_int as s16;
    (*globalCtx).envCtx.adjAmbientColor[1 as libc::c_int as usize] =
        0xc8 as libc::c_int as s16;
    (*globalCtx).envCtx.adjAmbientColor[2 as libc::c_int as usize] =
        0xc8 as libc::c_int as s16;
    (*globalCtx).envCtx.adjLight1Color[0 as libc::c_int as usize] =
        0xd7 as libc::c_int as s16;
    (*globalCtx).envCtx.adjLight1Color[1 as libc::c_int as usize] =
        0xa5 as libc::c_int as s16;
    (*globalCtx).envCtx.adjLight1Color[2 as libc::c_int as usize] =
        0xc8 as libc::c_int as s16;
    (*globalCtx).envCtx.screenFillColor[0 as libc::c_int as usize] =
        0xdc as libc::c_int as u8_0;
    (*globalCtx).envCtx.screenFillColor[1 as libc::c_int as usize] =
        0xdc as libc::c_int as u8_0;
    (*globalCtx).envCtx.screenFillColor[2 as libc::c_int as usize] =
        0x96 as libc::c_int as u8_0;
    (*globalCtx).envCtx.screenFillColor[3 as libc::c_int as usize] =
        0x64 as libc::c_int as u8_0;
}
#[no_mangle]
pub unsafe extern "C" fn BossVa_FindBoomerang(mut globalCtx:
                                                  *mut GlobalContext)
 -> *mut EnBoom {
    let mut actorIt: *mut Actor =
        (*globalCtx).actorCtx.actorLists[ACTORCAT_MISC as libc::c_int as
                                             usize].head;
    while !actorIt.is_null() {
        if (*actorIt).id as libc::c_int != ACTOR_EN_BOOM as libc::c_int {
            actorIt = (*actorIt).next
        } else { return actorIt as *mut EnBoom }
    }
    return 0 as *mut EnBoom;
}
#[no_mangle]
pub unsafe extern "C" fn BossVa_KillBari(mut this: *mut BossVa,
                                         mut globalCtx: *mut GlobalContext) {
    let mut i: s32 = 0;
    let mut scale: s16 = 0;
    let mut pos: Vec3f = Vec3f{x: 0., y: 0., z: 0.,};
    let mut velocity: Vec3f =
        { let mut init = Vec3f{x: 0.0f32, y: 0.0f32, z: 0.0f32,}; init };
    let mut accel: Vec3f =
        { let mut init = Vec3f{x: 0.0f32, y: 0.0f32, z: 0.0f32,}; init };
    i = 7 as libc::c_int;
    while i >= 0 as libc::c_int {
        pos.x = Rand_CenteredFloat(60.0f32) + (*this).actor.world.pos.x;
        pos.y =
            Rand_CenteredFloat(50.0f32) +
                ((*this).actor.world.pos.y +
                     (*this).actor.shape.yOffset * (*this).actor.scale.y);
        pos.z = Rand_CenteredFloat(60.0f32) + (*this).actor.world.pos.z;
        velocity.y = Rand_ZeroOne() + 1.0f32;
        scale =
            Rand_S16Offset(80 as libc::c_int as s16,
                           100 as libc::c_int as s16);
        if Rand_ZeroOne() < 0.7f32 {
            EffectSsDtBubble_SpawnColorProfile(globalCtx, &mut pos,
                                               &mut velocity, &mut accel,
                                               scale,
                                               25 as libc::c_int as s16,
                                               2 as libc::c_int as s16,
                                               1 as libc::c_int as s16);
        } else {
            EffectSsDtBubble_SpawnColorProfile(globalCtx, &mut pos,
                                               &mut velocity, &mut accel,
                                               scale,
                                               25 as libc::c_int as s16,
                                               0 as libc::c_int as s16,
                                               1 as libc::c_int as s16);
        }
        i -= 1
    }
    sFightPhase = sFightPhase.wrapping_add(1);
    BossVa_SetupBariDeath(this);
}
#[no_mangle]
pub unsafe extern "C" fn BossVa_Init(mut thisx: *mut Actor,
                                     mut globalCtx2: *mut GlobalContext) {
    let mut globalCtx: *mut GlobalContext = globalCtx2;
    let mut this: *mut BossVa = thisx as *mut BossVa;
    let mut i: s32 = 0;
    let mut warpId: s16 = 0;
    Actor_SetScale(&mut (*this).actor, 0.1f32);
    (*this).actor.targetMode = 5 as libc::c_int as s8;
    (*this).actor.colChkInfo.mass = 0xff as libc::c_int as u8_0;
    match (*this).actor.params as libc::c_int {
        -1 => {
            SkelAnime_Init(globalCtx, &mut (*this).skelAnime,
                           &mut gBarinadeBodySkel, &mut gBarinadeBodyAnim,
                           0 as *mut Vec3s, 0 as *mut Vec3s,
                           0 as libc::c_int);
            (*this).actor.flags |=
                ((1 as libc::c_int) << 24 as libc::c_int) as libc::c_uint
        }
        0 | 1 | 2 => {
            SkelAnime_InitFlex(globalCtx, &mut (*this).skelAnime,
                               &mut gBarinadeSupportSkel,
                               &mut gBarinadeSupportAttachedAnim,
                               0 as *mut Vec3s, 0 as *mut Vec3s,
                               0 as libc::c_int);
        }
        3 | 4 | 5 => {
            SkelAnime_InitFlex(globalCtx, &mut (*this).skelAnime,
                               &mut gBarinadeZapperSkel,
                               &mut gBarinadeZapperIdleAnim, 0 as *mut Vec3s,
                               0 as *mut Vec3s, 0 as libc::c_int);
        }
        16 | 17 | 18 => {
            SkelAnime_InitFlex(globalCtx, &mut (*this).skelAnime,
                               &mut gBarinadeStumpSkel,
                               &mut gBarinadeStumpAnim, 0 as *mut Vec3s,
                               0 as *mut Vec3s, 0 as libc::c_int);
        }
        19 => { }
        _ => {
            (*this).actor.flags |=
                ((1 as libc::c_int) << 24 as libc::c_int) as libc::c_uint;
            SkelAnime_Init(globalCtx, &mut (*this).skelAnime,
                           &mut gBarinadeBariSkel, &mut gBarinadeBariAnim,
                           0 as *mut Vec3s, 0 as *mut Vec3s,
                           0 as libc::c_int);
            (*this).actor.shape.yOffset = 400.0f32
        }
    }
    (*this).actor.focus.pos = (*this).actor.world.pos;
    (*this).onCeiling = 0 as libc::c_int as u8_0;
    (*this).actor.naviEnemyId = 0x14 as libc::c_int as u8_0;
    match (*this).actor.params as libc::c_int {
        -1 => {
            Actor_SpawnAsChild(&mut (*globalCtx).actorCtx, &mut (*this).actor,
                               globalCtx, ACTOR_BOSS_VA as libc::c_int as s16,
                               0.0f32, 80.0f32, 400.0f32,
                               0 as libc::c_int as s16,
                               0 as libc::c_int as s16,
                               0 as libc::c_int as s16,
                               BOSSVA_DOOR as libc::c_int as s16);
            if Flags_GetClear(globalCtx,
                              (*globalCtx).roomCtx.curRoom.num as s32) != 0 {
                warpId = ACTOR_EN_RU1 as libc::c_int as s16;
                if gSaveContext.eventChkInf[3 as libc::c_int as usize] as
                       libc::c_int & 0x80 as libc::c_int != 0 {
                    warpId = ACTOR_DOOR_WARP1 as libc::c_int as s16
                }
                Actor_Spawn(&mut (*globalCtx).actorCtx, globalCtx, warpId,
                            (*this).actor.world.pos.x,
                            (*this).actor.world.pos.y,
                            (*this).actor.world.pos.z,
                            0 as libc::c_int as s16, 0 as libc::c_int as s16,
                            0 as libc::c_int as s16, 0 as libc::c_int as s16);
                Actor_Spawn(&mut (*globalCtx).actorCtx, globalCtx,
                            ACTOR_ITEM_B_HEART as libc::c_int as s16,
                            (*this).actor.world.pos.x + 160.0f32,
                            (*this).actor.world.pos.y,
                            (*this).actor.world.pos.z,
                            0 as libc::c_int as s16, 0 as libc::c_int as s16,
                            0 as libc::c_int as s16, 0 as libc::c_int as s16);
                sDoorState = 100 as libc::c_int as s16;
                Actor_Kill(&mut (*this).actor);
            } else {
                (*this).actor.colChkInfo.damageTable =
                    sDamageTable.as_mut_ptr();
                sPhase2Timer = 0xffff as libc::c_int as u16_0;
                if gSaveContext.eventChkInf[7 as libc::c_int as usize] as
                       libc::c_int & 0x40 as libc::c_int != 0 {
                    sCsState = INTRO_CALL_BARI as libc::c_int as s8;
                    sDoorState = 100 as libc::c_int as s16;
                    func_8002DF54(globalCtx, &mut (*this).actor,
                                  1 as libc::c_int as u8_0);
                    (*globalCtx).envCtx.screenFillColor[0 as libc::c_int as
                                                            usize] =
                        0xdc as libc::c_int as u8_0;
                    (*globalCtx).envCtx.screenFillColor[1 as libc::c_int as
                                                            usize] =
                        0xdc as libc::c_int as u8_0;
                    (*globalCtx).envCtx.screenFillColor[2 as libc::c_int as
                                                            usize] =
                        0xbe as libc::c_int as u8_0;
                    (*globalCtx).envCtx.screenFillColor[3 as libc::c_int as
                                                            usize] =
                        0xd2 as libc::c_int as u8_0;
                    func_80064520(globalCtx, &mut (*globalCtx).csCtx);
                    sCsCamera = Gameplay_CreateSubCamera(globalCtx);
                    Gameplay_ChangeCameraStatus(globalCtx,
                                                0 as libc::c_int as s16,
                                                1 as libc::c_int as s16);
                    Gameplay_ChangeCameraStatus(globalCtx, sCsCamera,
                                                7 as libc::c_int as s16);
                    sCameraEye.x = 140.0f32;
                    sCameraNextEye.x = sCameraEye.x;
                    sCameraEye.y = 205.0f32;
                    sCameraNextEye.y = sCameraEye.y;
                    sCameraEye.z = -20.0f32;
                    sCameraNextEye.z = sCameraEye.z;
                    sCameraAt.x = 10.0f32;
                    sCameraNextAt.x = sCameraAt.x;
                    sCameraAt.y = 50.0f32;
                    sCameraNextAt.y = sCameraAt.y;
                    sCameraAt.z = -220.0f32;
                    sCameraNextAt.z = sCameraAt.z;
                    Gameplay_CameraSetAtEye(globalCtx, sCsCamera,
                                            &mut sCameraAt, &mut sCameraEye);
                    (*this).timer = 20 as libc::c_int;
                    i = BOSSVA_BARI_LOWER_5 as libc::c_int;
                    while i >= BOSSVA_BARI_UPPER_1 as libc::c_int {
                        Actor_SpawnAsChild(&mut (*globalCtx).actorCtx,
                                           &mut (*this).actor, globalCtx,
                                           ACTOR_BOSS_VA as libc::c_int as
                                               s16,
                                           sInitPosOffsets[i as usize].x +
                                               (*this).actor.world.pos.x,
                                           sInitPosOffsets[i as usize].y +
                                               (*this).actor.world.pos.y,
                                           sInitPosOffsets[i as usize].z +
                                               (*this).actor.world.pos.z,
                                           (sInitRot[i as usize].x as
                                                libc::c_int +
                                                (*this).actor.world.rot.x as
                                                    libc::c_int) as s16,
                                           (sInitRot[i as usize].y as
                                                libc::c_int +
                                                (*this).actor.world.rot.y as
                                                    libc::c_int) as s16,
                                           (sInitRot[i as usize].z as
                                                libc::c_int +
                                                (*this).actor.world.rot.z as
                                                    libc::c_int) as s16,
                                           i as s16);
                        i -= 1
                    }
                    sCameraEyeMaxVel = sZeroVec;
                    sCameraAtMaxVel = sCameraEyeMaxVel
                } else {
                    sCsState = INTRO_START as libc::c_int as s8;
                    sDoorState = 5 as libc::c_int as s16
                }
                (*this).zapHeadPos.x = 1.0f32;
                Collider_InitCylinder(globalCtx, &mut (*this).colliderBody);
                Collider_SetCylinder(globalCtx, &mut (*this).colliderBody,
                                     &mut (*this).actor, &mut sCylinderInit);
                i = BOSSVA_ZAPPER_3 as libc::c_int;
                while i >= BOSSVA_SUPPORT_1 as libc::c_int {
                    Actor_SpawnAsChild(&mut (*globalCtx).actorCtx,
                                       &mut (*this).actor, globalCtx,
                                       ACTOR_BOSS_VA as libc::c_int as s16,
                                       sInitPosOffsets[i as usize].x +
                                           (*this).actor.world.pos.x,
                                       sInitPosOffsets[i as usize].y +
                                           (*this).actor.world.pos.y,
                                       sInitPosOffsets[i as usize].z +
                                           (*this).actor.world.pos.z,
                                       (sInitRot[i as usize].x as libc::c_int
                                            +
                                            (*this).actor.world.rot.x as
                                                libc::c_int) as s16,
                                       (sInitRot[i as usize].y as libc::c_int
                                            +
                                            (*this).actor.world.rot.y as
                                                libc::c_int) as s16,
                                       (sInitRot[i as usize].z as libc::c_int
                                            +
                                            (*this).actor.world.rot.z as
                                                libc::c_int) as s16,
                                       i as s16);
                    i -= 1
                }
                Lib_MemSet(sVaEffects.as_mut_ptr() as *mut u8_0,
                           ((::std::mem::size_of::<[BossVaEffect; 400]>() as
                                 libc::c_ulong).wrapping_div(::std::mem::size_of::<BossVaEffect>()
                                                                 as
                                                                 libc::c_ulong)
                                as s32 as
                                libc::c_uint).wrapping_mul(::std::mem::size_of::<BossVaEffect>()
                                                               as
                                                               libc::c_ulong)
                               as size_t, 0 as libc::c_int as u8_0);
                if (sCsState as libc::c_int) < BOSSVA_BATTLE as libc::c_int {
                    BossVa_SetupIntro(this);
                } else { BossVa_SetupBodyPhase1(this); }
            }
        }
        0 | 1 | 2 => {
            Collider_InitJntSph(globalCtx, &mut (*this).colliderSph);
            Collider_SetJntSph(globalCtx, &mut (*this).colliderSph,
                               &mut (*this).actor, &mut sJntSphInitSupport,
                               (*this).elements.as_mut_ptr());
            if (sCsState as libc::c_int) < BOSSVA_BATTLE as libc::c_int {
                BossVa_SetupSupportIntro(this, globalCtx);
            } else { BossVa_SetupSupportAttached(this, globalCtx); }
            (*this).onCeiling = (*this).onCeiling.wrapping_add(1)
        }
        3 | 4 | 5 => {
            Collider_InitQuad(globalCtx, &mut (*this).colliderLightning);
            Collider_SetQuad(globalCtx, &mut (*this).colliderLightning,
                             &mut (*this).actor, &mut sQuadInit);
            if (sCsState as libc::c_int) < BOSSVA_BATTLE as libc::c_int {
                BossVa_SetupZapperIntro(this, globalCtx);
            } else { BossVa_SetupZapperAttack(this, globalCtx); }
        }
        16 | 17 | 18 => { BossVa_SetupStump(this, globalCtx); }
        19 => { BossVa_SetupDoor(this, globalCtx); }
        _ => {
            Collider_InitJntSph(globalCtx, &mut (*this).colliderSph);
            Collider_SetJntSph(globalCtx, &mut (*this).colliderSph,
                               &mut (*this).actor, &mut sJntSphInitBari,
                               (*this).elements.as_mut_ptr());
            Collider_InitQuad(globalCtx, &mut (*this).colliderLightning);
            Collider_SetQuad(globalCtx, &mut (*this).colliderLightning,
                             &mut (*this).actor, &mut sQuadInit);
            (*this).unk_1D8.x = 1.0f32;
            (*this).unk_1D8.y = 1.0f32;
            if (sCsState as libc::c_int) < BOSSVA_BATTLE as libc::c_int {
                BossVa_SetupBariIntro(this, globalCtx);
            } else if sFightPhase as libc::c_int >= 9 as libc::c_int {
                BossVa_SetupBariPhase3Attack(this, globalCtx);
            } else { BossVa_SetupBariPhase2Attack(this, globalCtx); }
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn BossVa_Destroy(mut thisx: *mut Actor,
                                        mut globalCtx: *mut GlobalContext) {
    let mut this: *mut BossVa = thisx as *mut BossVa;
    SkelAnime_Free(&mut (*this).skelAnime, globalCtx);
    Collider_DestroyJntSph(globalCtx, &mut (*this).colliderSph);
    Collider_DestroyCylinder(globalCtx, &mut (*this).colliderBody);
}
#[no_mangle]
pub unsafe extern "C" fn BossVa_SetupIntro(mut this: *mut BossVa) {
    let mut lastFrame: f32_0 =
        Animation_GetLastFrame(&mut gBarinadeBodyAnim as *mut AnimationHeader
                                   as *mut libc::c_void) as f32_0;
    Animation_Change(&mut (*this).skelAnime, &mut gBarinadeBodyAnim, 1.0f32,
                     lastFrame, lastFrame,
                     ANIMMODE_ONCE as libc::c_int as u8_0, 0.0f32);
    (*this).actor.shape.yOffset = -450.0f32;
    (*this).actor.flags &=
        !((1 as libc::c_int) << 0 as libc::c_int) as libc::c_uint;
    BossVa_SetupAction(this,
                       Some(BossVa_BodyIntro as
                                unsafe extern "C" fn(_: *mut BossVa,
                                                     _: *mut GlobalContext)
                                    -> ()));
}
#[no_mangle]
pub unsafe extern "C" fn BossVa_BodyIntro(mut this: *mut BossVa,
                                          mut globalCtx: *mut GlobalContext) {
    let mut i: s32 = 0;
    let mut player: *mut Player =
        (*globalCtx).actorCtx.actorLists[ACTORCAT_PLAYER as libc::c_int as
                                             usize].head as *mut Player;
    (*this).unk_1AC =
        ((*this).unk_1AC as libc::c_int + 0xc31 as libc::c_int) as s16;
    (*this).unk_1A0 = Math_CosS((*this).unk_1AC) * 0.1f32 + 1.0f32;
    (*this).unk_1A4 = Math_SinS((*this).unk_1AC) * 0.05f32 + 1.0f32;
    match sCsState as libc::c_int {
        -5 => {
            (*this).timer -= 1;
            if (*this).timer == 0 as libc::c_int {
                sCsState = INTRO_CLOSE_DOOR as libc::c_int as s8;
                (*this).timer = 10 as libc::c_int
            }
        }
        -4 => {
            (*globalCtx).envCtx.screenFillColor[0 as libc::c_int as usize] =
                0xdc as libc::c_int as u8_0;
            (*globalCtx).envCtx.screenFillColor[1 as libc::c_int as usize] =
                0xdc as libc::c_int as u8_0;
            (*globalCtx).envCtx.screenFillColor[2 as libc::c_int as usize] =
                0xbe as libc::c_int as u8_0;
            (*globalCtx).envCtx.screenFillColor[3 as libc::c_int as usize] =
                0xd2 as libc::c_int as u8_0;
            func_8002DF54(globalCtx, &mut (*this).actor,
                          8 as libc::c_int as u8_0);
            (*player).actor.shape.rot.y = 0x7fff as libc::c_int as s16;
            (*player).actor.world.rot.y = (*player).actor.shape.rot.y;
            sCsState += 1
        }
        -3 => {
            func_80064520(globalCtx, &mut (*globalCtx).csCtx);
            if sCsCamera as libc::c_int == 0 as libc::c_int {
                sCsCamera = Gameplay_CreateSubCamera(globalCtx)
            }
            Gameplay_ChangeCameraStatus(globalCtx, 0 as libc::c_int as s16,
                                        1 as libc::c_int as s16);
            Gameplay_ChangeCameraStatus(globalCtx, sCsCamera,
                                        7 as libc::c_int as s16);
            sCameraEye.x = 13.0f32;
            sCameraNextEye.x = sCameraEye.x;
            sCameraEye.y = 124.0f32;
            sCameraNextEye.y = sCameraEye.y;
            sCameraEye.z = 167.0f32;
            sCameraNextEye.z = sCameraEye.z;
            sCameraAt.x = (*player).actor.world.pos.x;
            sCameraNextAt.x = sCameraAt.x;
            sCameraAt.y = (*player).actor.world.pos.y;
            sCameraNextAt.y = sCameraAt.y;
            sCameraAt.z = (*player).actor.world.pos.z;
            sCameraNextAt.z = sCameraAt.z;
            sCameraEyeMaxVel = sZeroVec;
            sCameraAtMaxVel = sCameraEyeMaxVel;
            (*this).timer = 10 as libc::c_int;
            sCsState += 1
        }
        -2 => {
            (*this).timer -= 1;
            if (*this).timer == 0 as libc::c_int {
                func_8002DF54(globalCtx, &mut (*this).actor,
                              2 as libc::c_int as u8_0);
                sCsState += 1;
                (*this).timer = 30 as libc::c_int
            }
        }
        -1 => {
            (*this).timer -= 1;
            if (*this).timer == 0 as libc::c_int { sCsState += 1 }
            if Rand_ZeroOne() < 0.1f32 {
                Audio_PlayActorSound2(&mut (*this).actor,
                                      (0x3943 as libc::c_int -
                                           0x800 as libc::c_int) as u16_0);
            }
        }
        0 => {
            func_8002DF54(globalCtx, &mut (*this).actor,
                          1 as libc::c_int as u8_0);
            sCsState += 1
        }
        1 => {
            func_80064520(globalCtx, &mut (*globalCtx).csCtx);
            if sCsCamera as libc::c_int == 0 as libc::c_int {
                sCsCamera = Gameplay_CreateSubCamera(globalCtx)
            }
            Gameplay_ChangeCameraStatus(globalCtx, 0 as libc::c_int as s16,
                                        1 as libc::c_int as s16);
            Gameplay_ChangeCameraStatus(globalCtx, sCsCamera,
                                        7 as libc::c_int as s16);
            sCameraEye.x = 13.0f32;
            sCameraNextEye.x = sCameraEye.x;
            sCameraEye.y = 124.0f32;
            sCameraNextEye.y = sCameraEye.y;
            sCameraEye.z = 167.0f32;
            sCameraNextEye.z = sCameraEye.z;
            sCameraAt.x = (*player).actor.world.pos.x;
            sCameraNextAt.x = sCameraAt.x;
            sCameraAt.y = (*player).actor.world.pos.y;
            sCameraNextAt.y = sCameraAt.y;
            sCameraAt.z = (*player).actor.world.pos.z;
            sCameraNextAt.z = sCameraAt.z;
            sCameraEyeMaxVel = sZeroVec;
            sCameraAtMaxVel = sCameraEyeMaxVel;
            i = BOSSVA_BARI_LOWER_5 as libc::c_int;
            while i >= BOSSVA_BARI_UPPER_1 as libc::c_int {
                Actor_SpawnAsChild(&mut (*globalCtx).actorCtx,
                                   &mut (*this).actor, globalCtx,
                                   ACTOR_BOSS_VA as libc::c_int as s16,
                                   sInitPosOffsets[i as usize].x +
                                       (*this).actor.world.pos.x,
                                   sInitPosOffsets[i as usize].y +
                                       (*this).actor.world.pos.y,
                                   sInitPosOffsets[i as usize].z +
                                       (*this).actor.world.pos.z,
                                   (sInitRot[i as usize].x as libc::c_int +
                                        (*this).actor.world.rot.x as
                                            libc::c_int) as s16,
                                   (sInitRot[i as usize].y as libc::c_int +
                                        (*this).actor.world.rot.y as
                                            libc::c_int) as s16,
                                   (sInitRot[i as usize].z as libc::c_int +
                                        (*this).actor.world.rot.z as
                                            libc::c_int) as s16, i as s16);
                i -= 1
            }
            (*this).timer = 90 as libc::c_int;
            sCsState += 1
        }
        3 => {
            sCameraNextEye.x = -92.0f32;
            sCameraNextEye.y = 22.0f32;
            sCameraNextEye.z = 360.0f32;
            sCameraNextAt.x = 63.0f32;
            sCameraNextAt.y = 104.0f32;
            sCameraNextAt.z = 248.0f32;
            Math_SmoothStepToF(&mut sCameraEyeMaxVel.x, 7.0f32, 0.3f32,
                               0.7f32, 0.05f32);
            sCameraEyeMaxVel.z = sCameraEyeMaxVel.x;
            sCameraEyeMaxVel.y = sCameraEyeMaxVel.z;
            sCameraAtMaxVel = sCameraEyeMaxVel;
            (*this).timer -= 1;
            if (*this).timer == 0 as libc::c_int {
                sCsState += 1;
                (*this).timer = 60 as libc::c_int
            }
        }
        4 => {
            sCameraEye.x = 140.0f32;
            sCameraNextEye.x = sCameraEye.x;
            sCameraEye.y = 205.0f32;
            sCameraNextEye.y = sCameraEye.y;
            sCameraEye.z = -20.0f32;
            sCameraNextEye.z = sCameraEye.z;
            sCameraAt.x = 10.0f32;
            sCameraNextAt.x = sCameraAt.x;
            sCameraAt.y = 247.0f32;
            sCameraNextAt.y = sCameraAt.y;
            sCameraAt.z = -220.0f32;
            sCameraNextAt.z = sCameraAt.z;
            sCsState += 1;
            (*this).timer = 1 as libc::c_int
        }
        5 => {
            sCameraNextAt.x = 10.0f32;
            sCameraNextAt.y = 247.0f32;
            sCameraNextAt.z = -220.0f32;
            Math_SmoothStepToF(&mut sCameraEyeMaxVel.x, 7.0f32, 0.3f32,
                               0.7f32, 0.05f32);
            sCameraEyeMaxVel.z = sCameraEyeMaxVel.x;
            sCameraEyeMaxVel.y = sCameraEyeMaxVel.z;
            sCameraAtMaxVel = sCameraEyeMaxVel;
            (*this).timer -= 1;
            if (*this).timer == 0 as libc::c_int {
                sCsState += 1;
                (*this).timer = 40 as libc::c_int
            }
        }
        6 => {
            (*this).timer -= 1;
            if (*this).timer == 0 as libc::c_int {
                sCameraNextAt.x = 10.0f32;
                sCameraNextAt.y = 50.0f32;
                sCameraNextAt.z = -220.0f32;
                sCameraEyeMaxVel = sZeroVec;
                sCameraAtMaxVel = sCameraEyeMaxVel;
                sCsState += 1;
                sCsState += 1;
                (*this).timer = 20 as libc::c_int
            }
        }
        8 => {
            Math_SmoothStepToF(&mut sCameraEyeMaxVel.x, 14.0f32, 0.3f32,
                               1.0f32, 0.25f32);
            sCameraEyeMaxVel.y = sCameraEyeMaxVel.x * 0.7f32;
            sCameraEyeMaxVel.z = sCameraEyeMaxVel.x;
            sCameraAtMaxVel = sCameraEyeMaxVel;
            sCameraAtMaxVel.z = sCameraAtMaxVel.z * 1.75f32;
            (*this).timer -= 1;
            if (*this).timer == 0 as libc::c_int {
                sCsState += 1;
                (*this).timer = 7500 as libc::c_int;
                (*this).unk_1F2 = 0 as libc::c_int as s16
            }
        }
        9 => {
            i = 10 as libc::c_int;
            while i >= 1 as libc::c_int {
                if sBodyBari[(i - 1 as libc::c_int) as usize] != 0 {
                    if sBodyBari[(i - 1 as libc::c_int) as usize] as
                           libc::c_int == 1 as libc::c_int {
                        Audio_PlayActorSound2(&mut (*this).actor,
                                              0x3941 as libc::c_int as u16_0);
                        BossVa_SetSparkEnv(globalCtx);
                        if (*this).onCeiling as libc::c_int ==
                               0 as libc::c_int {
                            (*this).onCeiling = 2 as libc::c_int as u8_0
                            // Not used by body
                        }
                    } else if sBodyBari[(i - 1 as libc::c_int) as usize] as
                                  libc::c_int == 2 as libc::c_int {
                        BossVa_Spark(globalCtx, this, 6 as libc::c_int,
                                     140 as libc::c_int as s16, 50.0f32,
                                     30.0f32,
                                     SPARK_BARI as libc::c_int as u8_0,
                                     i as f32_0,
                                     1 as libc::c_int as
                                         u8_0); // Not used by body
                    }
                    if sBodyBari[(i - 1 as libc::c_int) as usize] as
                           libc::c_int <= 2 as libc::c_int {
                        sBodyBari[(i - 1 as libc::c_int) as usize] =
                            sBodyBari[(i - 1 as libc::c_int) as
                                          usize].wrapping_add(1)
                    }
                }
                i -= 1
            }
            Math_SmoothStepToS(&mut (*this).unk_1F2,
                               0x280 as libc::c_int as s16,
                               1 as libc::c_int as s16,
                               0x32 as libc::c_int as s16,
                               0 as libc::c_int as s16);
            Math_SmoothStepToF(&mut sCameraEyeMaxVel.x, 14.0f32, 0.3f32,
                               1.0f32, 0.25f32);
            sCameraEyeMaxVel.z = sCameraEyeMaxVel.x;
            sCameraAtMaxVel = sCameraEyeMaxVel;
            if (*this).timer >= 45000 as libc::c_int {
                (*globalCtx).envCtx.unk_BF = 1 as libc::c_int as u8_0;
                func_8002DF54(globalCtx, &mut (*this).actor,
                              8 as libc::c_int as u8_0);
            } else if (*this).timer >= 35000 as libc::c_int {
                Audio_QueueSeqCmd(((SEQ_PLAYER_BGM_MAIN as libc::c_int) <<
                                       24 as libc::c_int |
                                       0x1b as libc::c_int) as u32_0);
            }
            (*this).timer += (*this).unk_1F2 as libc::c_int;
            if (*this).timer >= 65536 as libc::c_int {
                sCameraAtMaxVel.y = 9.8f32;
                sCameraEyeMaxVel.y = sCameraAtMaxVel.y;
                sCsState += 1;
                sCameraNextEye.x = 10.0f32;
                sCameraNextEye.z = 0.0f32;
                sCameraNextAt.x = 10.0f32;
                sCameraNextAt.y = 140.0f32;
                sCameraNextAt.z = -200.0f32;
                if gSaveContext.eventChkInf[7 as libc::c_int as usize] as
                       libc::c_int & 0x40 as libc::c_int == 0 {
                    TitleCard_InitBossName(globalCtx,
                                           &mut (*globalCtx).actorCtx.titleCtx,
                                           gSegments[((gBarinadeTitleCardTex.as_mut_ptr()
                                                           as u32_0) <<
                                                          4 as libc::c_int >>
                                                          28 as libc::c_int)
                                                         as
                                                         usize].wrapping_add(gBarinadeTitleCardTex.as_mut_ptr()
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
                                           0xa0 as libc::c_int as s16,
                                           0xb4 as libc::c_int as s16,
                                           0x80 as libc::c_int as u8_0,
                                           0x28 as libc::c_int as u8_0);
                }
                if Rand_ZeroOne() < 0.1f32 {
                    Audio_PlayActorSound2(&mut (*this).actor,
                                          (0x3943 as libc::c_int -
                                               0x800 as libc::c_int) as
                                              u16_0);
                }
                (*this).timer = 40 as libc::c_int
            } else {
                sCameraEyeMaxVel.y = 1.6f32;
                sCameraNextEye.y = 5.0f32;
                sCameraNextEye.x = Math_SinS((*this).timer as s16) * 200.0f32;
                sCameraNextEye.z =
                    Math_CosS((*this).timer as s16) * 200.0f32 + -200.0f32
            }
        }
        10 => {
            BossVa_Spark(globalCtx, this, 3 as libc::c_int,
                         140 as libc::c_int as s16, 50.0f32, 30.0f32,
                         SPARK_BARI as libc::c_int as u8_0, 10.0f32,
                         0 as libc::c_int as u8_0);
            (*this).timer -= 1;
            if (*this).timer == 0 as libc::c_int {
                sCsState += 1;
                (*this).timer = 45 as libc::c_int
            }
        }
        11 => {
            BossVa_Spark(globalCtx, this, 3 as libc::c_int,
                         140 as libc::c_int as s16, 50.0f32, 30.0f32,
                         SPARK_BARI as libc::c_int as u8_0, 10.0f32,
                         0 as libc::c_int as u8_0);
            (*this).timer -= 1;
            if (*this).timer == 0 as libc::c_int {
                sCsState += 1;
                (*this).timer = 11 as libc::c_int
            }
        }
        12 => {
            (*this).timer -= 1;
            if (*this).timer == 0 as libc::c_int {
                Gameplay_ClearCamera(globalCtx, sCsCamera);
                sCsCamera = 0 as libc::c_int as s16;
                func_80064534(globalCtx, &mut (*globalCtx).csCtx);
                Gameplay_ChangeCameraStatus(globalCtx,
                                            0 as libc::c_int as s16,
                                            7 as libc::c_int as s16);
                func_8002DF54(globalCtx, &mut (*this).actor,
                              7 as libc::c_int as u8_0);
                sCsState += 1;
                gSaveContext.eventChkInf[7 as libc::c_int as usize] =
                    (gSaveContext.eventChkInf[7 as libc::c_int as usize] as
                         libc::c_int | 0x40 as libc::c_int) as u16_0;
                (*player).actor.world.rot.y =
                    ((*this).actor.yawTowardsPlayer as libc::c_int +
                         0x8000 as libc::c_int) as s16;
                (*player).actor.shape.rot.y = (*player).actor.world.rot.y
            }
        }
        13 => { BossVa_SetupBodyPhase1(this); }
        _ => { }
    }
    if sCsState as libc::c_int >= INTRO_BODY_SOUND as libc::c_int {
        func_800F436C(&mut (*this).actor.projectedPos,
                      (0x393c as libc::c_int - 0x800 as libc::c_int) as u16_0,
                      1.0f32);
        if sCsState as libc::c_int >= INTRO_CALL_BARI as libc::c_int &&
               (*globalCtx).gameplayFrames.wrapping_rem(4 as libc::c_int as
                                                            libc::c_uint) ==
                   0 as libc::c_int as libc::c_uint {
            BossVa_Spark(globalCtx, this, 1 as libc::c_int,
                         100 as libc::c_int as s16, 50.0f32, 10.0f32,
                         SPARK_BODY as libc::c_int as u8_0, 10.0f32,
                         0 as libc::c_int as u8_0);
        }
    }
    (*this).unk_1B0 =
        ((*this).unk_1B0 as libc::c_int + 0xce4 as libc::c_int) as s16;
    (*this).bodyGlow =
        ((Math_SinS((*this).unk_1B0) * 50.0f32) as s16 as libc::c_int +
             150 as libc::c_int) as s16;
    if sCsCamera as libc::c_int != 0 as libc::c_int &&
           sCsState as libc::c_int <= INTRO_TITLE as libc::c_int {
        Math_SmoothStepToF(&mut sCameraEye.x, sCameraNextEye.x, 0.3f32,
                           sCameraEyeMaxVel.x, 0.075f32);
        Math_SmoothStepToF(&mut sCameraEye.y, sCameraNextEye.y, 0.3f32,
                           sCameraEyeMaxVel.y, 0.075f32);
        Math_SmoothStepToF(&mut sCameraEye.z, sCameraNextEye.z, 0.3f32,
                           sCameraEyeMaxVel.z, 0.075f32);
        Math_SmoothStepToF(&mut sCameraAt.x, sCameraNextAt.x, 0.3f32,
                           sCameraAtMaxVel.x, 0.075f32);
        Math_SmoothStepToF(&mut sCameraAt.y, sCameraNextAt.y, 0.3f32,
                           sCameraAtMaxVel.y, 0.075f32);
        Math_SmoothStepToF(&mut sCameraAt.z, sCameraNextAt.z, 0.3f32,
                           sCameraAtMaxVel.z, 0.075f32);
        Gameplay_CameraSetAtEye(globalCtx, sCsCamera, &mut sCameraAt,
                                &mut sCameraEye);
    };
}
#[no_mangle]
pub unsafe extern "C" fn BossVa_SetupBodyPhase1(mut this: *mut BossVa) {
    let mut lastFrame: f32_0 =
        Animation_GetLastFrame(&mut gBarinadeBodyAnim as *mut AnimationHeader
                                   as *mut libc::c_void) as f32_0;
    Animation_Change(&mut (*this).skelAnime, &mut gBarinadeBodyAnim, 1.0f32,
                     lastFrame, lastFrame,
                     ANIMMODE_ONCE as libc::c_int as u8_0, 0.0f32);
    (*this).actor.shape.yOffset = -450.0f32;
    (*this).actor.flags &=
        !((1 as libc::c_int) << 0 as libc::c_int) as libc::c_uint;
    (*this).timer = 25 as libc::c_int;
    sBodyState = 0x80 as libc::c_int as u8_0;
    BossVa_SetupAction(this,
                       Some(BossVa_BodyPhase1 as
                                unsafe extern "C" fn(_: *mut BossVa,
                                                     _: *mut GlobalContext)
                                    -> ()));
}
#[no_mangle]
pub unsafe extern "C" fn BossVa_BodyPhase1(mut this: *mut BossVa,
                                           mut globalCtx:
                                               *mut GlobalContext) {
    let mut player: *mut Player =
        (*globalCtx).actorCtx.actorLists[ACTORCAT_PLAYER as libc::c_int as
                                             usize].head as *mut Player;
    (*this).unk_1B0 =
        ((*this).unk_1B0 as libc::c_int + 0xce4 as libc::c_int) as s16;
    (*this).bodyGlow =
        ((Math_SinS((*this).unk_1B0) * 50.0f32) as s16 as libc::c_int +
             150 as libc::c_int) as s16;
    if (*this).timer != 0 as libc::c_int {
        (*this).timer -= 1;
        if (*this).timer == 0 as libc::c_int {
            sBodyState =
                (sBodyState as libc::c_int &
                     !(0x80 as libc::c_int) as u8_0 as libc::c_int) as u8_0
        }
    }
    if (*this).colliderBody.base.atFlags as libc::c_int &
           (1 as libc::c_int) << 1 as libc::c_int != 0 {
        (*this).colliderBody.base.atFlags =
            ((*this).colliderBody.base.atFlags as libc::c_int &
                 !((1 as libc::c_int) << 1 as libc::c_int)) as u8_0;
        if (*this).colliderBody.base.at == &mut (*player).actor as *mut Actor
           {
            func_8002F71C(globalCtx, &mut (*this).actor, 8.0f32,
                          (*this).actor.yawTowardsPlayer, 8.0f32);
        }
    }
    if sBodyState as libc::c_int & 0x7f as libc::c_int != 0 {
        (*this).skelAnime.curFrame = 0.0f32;
        Actor_SetColorFilter(&mut (*this).actor, 0 as libc::c_int as s16,
                             255 as libc::c_int as s16,
                             0 as libc::c_int as s16,
                             12 as libc::c_int as s16);
        Audio_PlayActorSound2(&mut (*this).actor,
                              0x393d as libc::c_int as u16_0);
    }
    if SkelAnime_Update(&mut (*this).skelAnime) != 0 &&
           sFightPhase as libc::c_int >= 3 as libc::c_int {
        BossVa_SetupBodyPhase2(this, globalCtx);
    }
    Math_SmoothStepToS(&mut (*this).actor.shape.rot.x,
                       (*this).actor.world.rot.x, 1 as libc::c_int as s16,
                       0xc8 as libc::c_int as s16, 0 as libc::c_int as s16);
    Math_SmoothStepToS(&mut (*this).actor.shape.rot.z,
                       (*this).actor.world.rot.z, 1 as libc::c_int as s16,
                       0xc8 as libc::c_int as s16, 0 as libc::c_int as s16);
    (*this).unk_1AC =
        ((*this).unk_1AC as libc::c_int + 0xc31 as libc::c_int) as s16;
    (*this).unk_1A0 = Math_CosS((*this).unk_1AC) * 0.1f32 + 1.0f32;
    (*this).unk_1A4 = Math_SinS((*this).unk_1AC) * 0.05f32 + 1.0f32;
    if (*globalCtx).gameplayFrames.wrapping_rem(4 as libc::c_int as
                                                    libc::c_uint) ==
           0 as libc::c_int as libc::c_uint {
        BossVa_Spark(globalCtx, this, 1 as libc::c_int,
                     100 as libc::c_int as s16, 50.0f32, 10.0f32,
                     SPARK_BARI as libc::c_int as u8_0, 10.0f32,
                     0 as libc::c_int as u8_0);
    }
    if Rand_ZeroOne() < 0.1f32 {
        Audio_PlayActorSound2(&mut (*this).actor,
                              (0x3943 as libc::c_int - 0x800 as libc::c_int)
                                  as u16_0);
    }
    Collider_UpdateCylinder(&mut (*this).actor, &mut (*this).colliderBody);
    CollisionCheck_SetOC(globalCtx, &mut (*globalCtx).colChkCtx,
                         &mut (*this).colliderBody.base);
    CollisionCheck_SetAT(globalCtx, &mut (*globalCtx).colChkCtx,
                         &mut (*this).colliderBody.base);
    func_800F436C(&mut (*this).actor.projectedPos,
                  (0x393c as libc::c_int - 0x800 as libc::c_int) as u16_0,
                  1.0f32);
}
#[no_mangle]
pub unsafe extern "C" fn BossVa_SetupBodyPhase2(mut this: *mut BossVa,
                                                mut globalCtx:
                                                    *mut GlobalContext) {
    let mut i: s32 = 0;
    sFightPhase = sFightPhase.wrapping_add(1);
    i = BOSSVA_BARI_UPPER_5 as libc::c_int;
    while i >= BOSSVA_BARI_UPPER_1 as libc::c_int {
        Actor_SpawnAsChild(&mut (*globalCtx).actorCtx, &mut (*this).actor,
                           globalCtx, ACTOR_BOSS_VA as libc::c_int as s16,
                           sInitPosOffsets[i as usize].x +
                               (*this).actor.world.pos.x,
                           sInitPosOffsets[i as usize].y +
                               (*this).actor.world.pos.y,
                           sInitPosOffsets[i as usize].z +
                               (*this).actor.world.pos.z,
                           (sInitRot[i as usize].x as libc::c_int +
                                (*this).actor.world.rot.x as libc::c_int) as
                               s16,
                           (sInitRot[i as usize].y as libc::c_int +
                                (*this).actor.world.rot.y as libc::c_int) as
                               s16,
                           (sInitRot[i as usize].z as libc::c_int +
                                (*this).actor.world.rot.z as libc::c_int) as
                               s16, i as s16);
        i -= 1
    }
    (*this).invincibilityTimer = 0 as libc::c_int as s8;
    (*this).actor.flags |=
        ((1 as libc::c_int) << 0 as libc::c_int) as libc::c_uint;
    BossVa_SetupAction(this,
                       Some(BossVa_BodyPhase2 as
                                unsafe extern "C" fn(_: *mut BossVa,
                                                     _: *mut GlobalContext)
                                    -> ()));
}
#[no_mangle]
pub unsafe extern "C" fn BossVa_BodyPhase2(mut this: *mut BossVa,
                                           mut globalCtx:
                                               *mut GlobalContext) {
    let mut player: *mut Player =
        (*globalCtx).actorCtx.actorLists[ACTORCAT_PLAYER as libc::c_int as
                                             usize].head as *mut Player;
    let mut sp48: Vec3f = Vec3f{x: 0., y: 0., z: 0.,};
    if (*this).actor.colorFilterTimer as libc::c_int == 0 as libc::c_int {
        sPhase2Timer = sPhase2Timer.wrapping_add(1);
        if (*this).invincibilityTimer as libc::c_int != 0 as libc::c_int &&
               (*this).actor.colorFilterParams as libc::c_int &
                   0x4000 as libc::c_int != 0 {
            Actor_SetColorFilter(&mut (*this).actor, 0 as libc::c_int as s16,
                                 255 as libc::c_int as s16,
                                 0 as libc::c_int as s16,
                                 160 as libc::c_int as s16);
            (*this).actor.colorFilterTimer =
                (*this).invincibilityTimer as u8_0
        } else {
            (*this).colliderBody.info.bumper.dmgFlags =
                0x10 as libc::c_int as u32_0
        }
    }
    if (*this).colliderBody.base.acFlags as libc::c_int &
           (1 as libc::c_int) << 1 as libc::c_int != 0 {
        (*this).colliderBody.base.acFlags =
            ((*this).colliderBody.base.acFlags as libc::c_int &
                 !((1 as libc::c_int) << 1 as libc::c_int)) as u8_0;
        if (*(*this).colliderBody.base.ac).id as libc::c_int ==
               ACTOR_EN_BOOM as libc::c_int {
            sPhase2Timer =
                (sPhase2Timer as libc::c_int & 0xfe00 as libc::c_int) as
                    u16_0;
            Actor_SetColorFilter(&mut (*this).actor, 0 as libc::c_int as s16,
                                 255 as libc::c_int as s16,
                                 0 as libc::c_int as s16,
                                 160 as libc::c_int as s16);
            (*this).colliderBody.info.bumper.dmgFlags =
                0xfc00712 as libc::c_int as u32_0
        } else {
            sKillBari = sKillBari.wrapping_add(1);
            if (*this).actor.colorFilterTimer as libc::c_int !=
                   0 as libc::c_int &&
                   (*this).actor.colorFilterParams as libc::c_int &
                       0x4000 as libc::c_int == 0 {
                (*this).invincibilityTimer =
                    ((*this).actor.colorFilterTimer as libc::c_int -
                         5 as libc::c_int) as s8;
                if (*this).invincibilityTimer as libc::c_int >
                       160 as libc::c_int {
                    (*this).invincibilityTimer = 0 as libc::c_int as s8
                }
            }
            Actor_SetColorFilter(&mut (*this).actor,
                                 0x4000 as libc::c_int as s16,
                                 255 as libc::c_int as s16,
                                 0 as libc::c_int as s16,
                                 12 as libc::c_int as s16);
        }
        Audio_PlayActorSound2(&mut (*this).actor,
                              0x393e as libc::c_int as u16_0);
    }
    if (*this).colliderBody.base.atFlags as libc::c_int &
           (1 as libc::c_int) << 1 as libc::c_int != 0 {
        (*this).colliderBody.base.atFlags =
            ((*this).colliderBody.base.atFlags as libc::c_int &
                 !((1 as libc::c_int) << 1 as libc::c_int)) as u8_0;
        sPhase2Timer =
            (sPhase2Timer as libc::c_int + 0x18 as libc::c_int &
                 0xfff0 as libc::c_int) as u16_0;
        if (*this).colliderBody.base.at == &mut (*player).actor as *mut Actor
           {
            func_8002F71C(globalCtx, &mut (*this).actor, 8.0f32,
                          (*this).actor.yawTowardsPlayer, 8.0f32);
            Audio_PlayActorSound2(&mut (*player).actor,
                                  0x83e as libc::c_int as u16_0);
        }
    }
    if sPhase2Timer as libc::c_int > 10 as libc::c_int &&
           sPhase2Timer as libc::c_int & 7 as libc::c_int == 0 &&
           (*this).actor.speedXZ == 1.0f32 {
        sp48 = (*this).actor.world.pos;
        sp48.y +=
            310.0f32 + (*this).actor.shape.yOffset * (*this).actor.scale.y;
        sp48.x += -10.0f32;
        sp48.z += 220.0f32;
        BossVa_SpawnSparkBall(globalCtx, sVaEffects.as_mut_ptr(), this,
                              &mut sp48, 4 as libc::c_int as s16,
                              0 as libc::c_int as u8_0);
    }
    if Rand_ZeroOne() < 0.1f32 {
        Audio_PlayActorSound2(&mut (*this).actor,
                              (0x3943 as libc::c_int - 0x800 as libc::c_int)
                                  as u16_0);
    }
    Math_SmoothStepToS(&mut (*this).actor.shape.rot.x,
                       (*this).actor.world.rot.x, 1 as libc::c_int as s16,
                       0xc8 as libc::c_int as s16, 0 as libc::c_int as s16);
    Math_SmoothStepToS(&mut (*this).actor.shape.rot.z,
                       (*this).actor.world.rot.z, 1 as libc::c_int as s16,
                       0xc8 as libc::c_int as s16, 0 as libc::c_int as s16);
    Math_SmoothStepToF(&mut (*this).actor.shape.yOffset, -1000.0f32, 1.0f32,
                       20.0f32, 0.0f32);
    if sPhase2Timer as libc::c_int & 0x100 as libc::c_int == 0 {
        (*this).actor.flags |=
            ((1 as libc::c_int) << 0 as libc::c_int) as libc::c_uint;
        (*this).actor.speedXZ = 1.0f32
    } else {
        (*this).actor.flags &=
            !((1 as libc::c_int) << 0 as libc::c_int) as libc::c_uint;
        (*this).actor.speedXZ = 0.0f32
    }
    if SkelAnime_Update(&mut (*this).skelAnime) != 0 &&
           sFightPhase as libc::c_int >= 9 as libc::c_int {
        BossVa_SetupBodyPhase3(this);
    }
    (*this).unk_1AC =
        ((*this).unk_1AC as libc::c_int + 0xc31 as libc::c_int) as s16;
    (*this).unk_1A0 = Math_CosS((*this).unk_1AC) * 0.1f32 + 1.0f32;
    (*this).unk_1A4 = Math_SinS((*this).unk_1AC) * 0.05f32 + 1.0f32;
    if (*globalCtx).gameplayFrames.wrapping_rem(4 as libc::c_int as
                                                    libc::c_uint) ==
           0 as libc::c_int as libc::c_uint {
        BossVa_Spark(globalCtx, this, 1 as libc::c_int,
                     100 as libc::c_int as s16, 50.0f32, 10.0f32,
                     SPARK_BODY as libc::c_int as u8_0, 10.0f32,
                     0 as libc::c_int as u8_0);
    }
    (*this).actor.focus.pos = (*this).actor.world.pos;
    (*this).actor.focus.pos.y += 45.0f32;
    Collider_UpdateCylinder(&mut (*this).actor, &mut (*this).colliderBody);
    CollisionCheck_SetOC(globalCtx, &mut (*globalCtx).colChkCtx,
                         &mut (*this).colliderBody.base);
    if (*this).actor.colorFilterTimer as libc::c_int == 0 as libc::c_int {
        CollisionCheck_SetAT(globalCtx, &mut (*globalCtx).colChkCtx,
                             &mut (*this).colliderBody.base);
    }
    if (*this).actor.colorFilterTimer as libc::c_int == 0 as libc::c_int ||
           (*this).actor.colorFilterParams as libc::c_int &
               0x4000 as libc::c_int == 0 {
        CollisionCheck_SetAC(globalCtx, &mut (*globalCtx).colChkCtx,
                             &mut (*this).colliderBody.base);
    }
    func_800F436C(&mut (*this).actor.projectedPos,
                  (0x393c as libc::c_int - 0x800 as libc::c_int) as u16_0,
                  (*this).headRot.y as libc::c_int as libc::c_float *
                      0.00025f32 + 1.0f32);
}
#[no_mangle]
pub unsafe extern "C" fn BossVa_SetupBodyPhase3(mut this: *mut BossVa) {
    (*this).colliderBody.info.bumper.dmgFlags = 0x10 as libc::c_int as u32_0;
    (*this).actor.speedXZ = 0.0f32;
    sPhase3StopMoving = 0 as libc::c_int as u8_0;
    BossVa_SetupAction(this,
                       Some(BossVa_BodyPhase3 as
                                unsafe extern "C" fn(_: *mut BossVa,
                                                     _: *mut GlobalContext)
                                    -> ()));
}
#[no_mangle]
pub unsafe extern "C" fn BossVa_BodyPhase3(mut this: *mut BossVa,
                                           mut globalCtx:
                                               *mut GlobalContext) {
    let mut pad: s32 = 0;
    let mut player: *mut Player =
        (*globalCtx).actorCtx.actorLists[ACTORCAT_PLAYER as libc::c_int as
                                             usize].head as *mut Player;
    let mut i: s32 = 0;
    let mut sp62: s16 = 0;
    sp62 =
        Math_Vec3f_Yaw(&mut (*this).actor.world.pos,
                       &mut (*this).actor.home.pos);
    (*this).unk_1B0 =
        ((*this).unk_1B0 as libc::c_int + 0xce4 as libc::c_int) as s16;
    (*this).bodyGlow =
        ((Math_SinS((*this).unk_1B0) * 50.0f32) as s16 as libc::c_int +
             150 as libc::c_int) as s16;
    if (*this).colliderBody.base.atFlags as libc::c_int &
           (1 as libc::c_int) << 1 as libc::c_int != 0 {
        (*this).colliderBody.base.atFlags =
            ((*this).colliderBody.base.atFlags as libc::c_int &
                 !((1 as libc::c_int) << 1 as libc::c_int)) as u8_0;
        if (*this).colliderBody.base.at == &mut (*player).actor as *mut Actor
           {
            func_8002F71C(globalCtx, &mut (*this).actor, 8.0f32,
                          (*this).actor.yawTowardsPlayer, 8.0f32);
            (*this).actor.world.rot.y =
                ((*this).actor.world.rot.y as libc::c_int +
                     (Rand_CenteredFloat(0x2ee0 as libc::c_int as f32_0) as
                          s16 as libc::c_int + 0x8000 as libc::c_int)) as s16;
            Audio_PlayActorSound2(&mut (*player).actor,
                                  0x83e as libc::c_int as u16_0);
        }
    }
    if (*this).colliderBody.base.acFlags as libc::c_int &
           (1 as libc::c_int) << 1 as libc::c_int != 0 {
        (*this).skelAnime.curFrame = 0.0f32;
        Actor_SetColorFilter(&mut (*this).actor, 0 as libc::c_int as s16,
                             255 as libc::c_int as s16,
                             0 as libc::c_int as s16,
                             12 as libc::c_int as s16);
        Audio_PlayActorSound2(&mut (*this).actor,
                              0x393e as libc::c_int as u16_0);
        sBodyState = 1 as libc::c_int as u8_0;
        (*this).timer = 131 as libc::c_int;
        (*this).actor.flags &=
            !((1 as libc::c_int) << 0 as libc::c_int) as libc::c_uint
    } else {
        sBodyState = 0 as libc::c_int as u8_0;
        if (*this).timer == 0 as libc::c_int {
            if Math_SmoothStepToS(&mut (*this).headRot.y,
                                  0xfa0 as libc::c_int as s16,
                                  1 as libc::c_int as s16,
                                  0x12c as libc::c_int as s16,
                                  0 as libc::c_int as s16) as libc::c_int ==
                   0 as libc::c_int {
                if (*this).actor.speedXZ == 0.0f32 {
                    (*this).actor.world.rot.y = (*this).actor.yawTowardsPlayer
                }
                Math_SmoothStepToF(&mut (*this).actor.speedXZ, 3.0f32, 1.0f32,
                                   0.15f32, 0.0f32);
            }
            (*this).actor.flags |=
                ((1 as libc::c_int) << 0 as libc::c_int) as libc::c_uint
        } else {
            (*this).timer -= 1;
            if (*this).timer < 35 as libc::c_int {
                sBodyState = 0x80 as libc::c_int as u8_0
            }
            Math_SmoothStepToS(&mut (*this).headRot.y,
                               0 as libc::c_int as s16,
                               1 as libc::c_int as s16,
                               0x12c as libc::c_int as s16,
                               0 as libc::c_int as s16);
            Math_SmoothStepToF(&mut (*this).actor.speedXZ, 0.0f32, 1.0f32,
                               0.2f32, 0.0f32);
            Math_SmoothStepToF(&mut (*this).actor.shape.yOffset, -1420.0f32,
                               1.0f32, 30.0f32, 0.0f32);
        }
    }
    if Math_Vec3f_DistXZ(&mut (*this).actor.world.pos,
                         &mut (*this).actor.home.pos) >= 400.0f32 {
        Math_SmoothStepToS(&mut (*this).actor.world.rot.y, sp62,
                           1 as libc::c_int as s16,
                           0x3e8 as libc::c_int as s16,
                           0 as libc::c_int as s16);
    } else if (*player).invincibilityTimer as libc::c_int != 0 as libc::c_int
     {
        Math_SmoothStepToS(&mut (*this).actor.world.rot.y,
                           ((*this).actor.yawTowardsPlayer as libc::c_int +
                                0x8000 as libc::c_int) as s16,
                           1 as libc::c_int as s16,
                           0x12c as libc::c_int as s16,
                           0 as libc::c_int as s16);
    } else if (*globalCtx).gameplayFrames &
                  0x80 as libc::c_int as libc::c_uint ==
                  0 as libc::c_int as libc::c_uint {
        Math_SmoothStepToS(&mut (*this).actor.world.rot.y,
                           (*this).actor.yawTowardsPlayer,
                           1 as libc::c_int as s16,
                           0x12c as libc::c_int as s16,
                           0 as libc::c_int as s16);
    } else {
        Math_SmoothStepToS(&mut (*this).actor.world.rot.y, sp62,
                           1 as libc::c_int as s16,
                           0x258 as libc::c_int as s16,
                           0 as libc::c_int as s16);
    }
    if sPhase3StopMoving != 0 { (*this).actor.speedXZ = 0.0f32 }
    Actor_MoveForward(&mut (*this).actor);
    if SkelAnime_Update(&mut (*this).skelAnime) != 0 &&
           sFightPhase as libc::c_int >= 15 as libc::c_int {
        BossVa_SetupBodyPhase4(this, globalCtx);
    }
    (*this).actor.shape.rot.y =
        ((*this).actor.shape.rot.y as libc::c_int +
             (*this).headRot.y as libc::c_int) as s16;
    if sFightPhase as libc::c_int == 9 as libc::c_int {
        Math_SmoothStepToF(&mut (*this).actor.shape.yOffset, -450.0f32,
                           1.0f32, 15.0f32, 0.0f32);
    } else {
        Math_SmoothStepToF(&mut (*this).actor.shape.yOffset, -810.0f32,
                           1.0f32, 15.0f32, 0.0f32);
    }
    if (*this).actor.shape.yOffset >= -500.0f32 &&
           sFightPhase as libc::c_int == 9 as libc::c_int {
        i = BOSSVA_BARI_LOWER_5 as libc::c_int;
        while i >= BOSSVA_BARI_LOWER_1 as libc::c_int {
            Actor_SpawnAsChild(&mut (*globalCtx).actorCtx, &mut (*this).actor,
                               globalCtx, ACTOR_BOSS_VA as libc::c_int as s16,
                               sInitPosOffsets[i as usize].x +
                                   (*this).actor.world.pos.x,
                               sInitPosOffsets[i as usize].y +
                                   (*this).actor.world.pos.y,
                               sInitPosOffsets[i as usize].z +
                                   (*this).actor.world.pos.z,
                               (sInitRot[i as usize].x as libc::c_int +
                                    (*this).actor.world.rot.x as libc::c_int)
                                   as s16,
                               (sInitRot[i as usize].y as libc::c_int +
                                    (*this).actor.world.rot.y as libc::c_int)
                                   as s16,
                               (sInitRot[i as usize].z as libc::c_int +
                                    (*this).actor.world.rot.z as libc::c_int)
                                   as s16, i as s16);
            i -= 1
        }
        sFightPhase = sFightPhase.wrapping_add(1)
    }
    (*this).unk_1AC =
        ((*this).unk_1AC as libc::c_int + 0xc31 as libc::c_int) as s16;
    (*this).unk_1A0 = Math_CosS((*this).unk_1AC) * 0.1f32 + 1.0f32;
    (*this).unk_1A4 = Math_SinS((*this).unk_1AC) * 0.05f32 + 1.0f32;
    if (*globalCtx).gameplayFrames.wrapping_rem(4 as libc::c_int as
                                                    libc::c_uint) ==
           0 as libc::c_int as libc::c_uint {
        BossVa_Spark(globalCtx, this, 1 as libc::c_int,
                     0x64 as libc::c_int as s16, 50.0f32, 10.0f32,
                     SPARK_BODY as libc::c_int as u8_0, 10.0f32,
                     0 as libc::c_int as u8_0);
    }
    (*this).actor.focus.pos = (*this).actor.world.pos;
    (*this).actor.focus.pos.y += 20.0f32;
    if Rand_ZeroOne() < 0.1f32 {
        Audio_PlayActorSound2(&mut (*this).actor,
                              (0x3943 as libc::c_int - 0x800 as libc::c_int)
                                  as u16_0);
    }
    Collider_UpdateCylinder(&mut (*this).actor, &mut (*this).colliderBody);
    CollisionCheck_SetOC(globalCtx, &mut (*globalCtx).colChkCtx,
                         &mut (*this).colliderBody.base);
    CollisionCheck_SetAT(globalCtx, &mut (*globalCtx).colChkCtx,
                         &mut (*this).colliderBody.base);
    if (*this).timer == 0 as libc::c_int {
        CollisionCheck_SetAC(globalCtx, &mut (*globalCtx).colChkCtx,
                             &mut (*this).colliderBody.base);
    }
    func_800F436C(&mut (*this).actor.projectedPos,
                  (0x393c as libc::c_int - 0x800 as libc::c_int) as u16_0,
                  (*this).headRot.y as libc::c_int as libc::c_float *
                      0.00025f32 + 1.0f32);
}
#[no_mangle]
pub unsafe extern "C" fn BossVa_SetupBodyPhase4(mut this: *mut BossVa,
                                                mut globalCtx:
                                                    *mut GlobalContext) {
    (*this).unk_1AC = 0 as libc::c_int as s16;
    (*this).actor.flags |=
        ((1 as libc::c_int) << 0 as libc::c_int) as libc::c_uint;
    (*this).headRot.y = (*this).unk_1AC;
    (*this).actor.world.rot.y = (*this).actor.yawTowardsPlayer;
    (*this).timer2 =
        ((Rand_ZeroOne() * 150.0f32) as s16 as libc::c_int +
             300 as libc::c_int) as s16;
    sBodyState = 1 as libc::c_int as u8_0;
    sPhase4HP = 4 as libc::c_int as s8;
    if (*this).actor.shape.yOffset != 0.0f32 {
        (*this).timer = -(30 as libc::c_int)
    }
    (*this).colliderBody.dim.radius = 55 as libc::c_int as s16;
    BossVa_SetupAction(this,
                       Some(BossVa_BodyPhase4 as
                                unsafe extern "C" fn(_: *mut BossVa,
                                                     _: *mut GlobalContext)
                                    -> ()));
}
#[no_mangle]
pub unsafe extern "C" fn BossVa_BodyPhase4(mut this: *mut BossVa,
                                           mut globalCtx:
                                               *mut GlobalContext) {
    let mut player: *mut Player =
        (*globalCtx).actorCtx.actorLists[ACTORCAT_PLAYER as libc::c_int as
                                             usize].head as *mut Player;
    let mut tmpf1: f32_0 = 0.;
    let mut boomerang: *mut EnBoom = 0 as *mut EnBoom;
    (*this).unk_1B0 =
        ((*this).unk_1B0 as libc::c_int +
             ((sFightPhase as libc::c_int - 15 as libc::c_int +
                   1 as libc::c_int) as libc::c_float * 1000.0f32) as s16 as
                 libc::c_int + 0xce4 as libc::c_int) as s16;
    (*this).bodyGlow =
        ((Math_SinS((*this).unk_1B0) * 50.0f32) as s16 as libc::c_int +
             150 as libc::c_int) as s16;
    if (*this).colliderBody.base.atFlags as libc::c_int &
           (1 as libc::c_int) << 1 as libc::c_int != 0 {
        (*this).colliderBody.base.atFlags =
            ((*this).colliderBody.base.atFlags as libc::c_int &
                 !((1 as libc::c_int) << 1 as libc::c_int)) as u8_0;
        if (*this).colliderBody.base.at == &mut (*player).actor as *mut Actor
           {
            func_8002F71C(globalCtx, &mut (*this).actor, 8.0f32,
                          (*this).actor.yawTowardsPlayer, 8.0f32);
            (*this).actor.world.rot.y =
                ((*this).actor.world.rot.y as libc::c_int +
                     (Rand_CenteredFloat(0x2ee0 as libc::c_int as f32_0) as
                          s16 as libc::c_int + 0x8000 as libc::c_int)) as s16;
            Audio_PlayActorSound2(&mut (*player).actor,
                                  0x83e as libc::c_int as u16_0);
        }
    }
    if Rand_ZeroOne() < 0.1f32 {
        Audio_PlayActorSound2(&mut (*this).actor,
                              (0x3943 as libc::c_int - 0x800 as libc::c_int)
                                  as u16_0);
    }
    if (*this).colliderBody.base.acFlags as libc::c_int &
           (1 as libc::c_int) << 1 as libc::c_int != 0 {
        (*this).colliderBody.base.acFlags =
            ((*this).colliderBody.base.acFlags as libc::c_int &
                 !((1 as libc::c_int) << 1 as libc::c_int)) as u8_0;
        (*this).skelAnime.curFrame = 0.0f32;
        if (*this).timer >= 0 as libc::c_int {
            if (*this).invincibilityTimer as libc::c_int == 0 as libc::c_int {
                (*this).invincibilityTimer = 8 as libc::c_int as s8;
                if (*this).actor.colChkInfo.damageEffect as libc::c_int !=
                       1 as libc::c_int {
                    (*this).actor.world.rot.y =
                        (*this).actor.yawTowardsPlayer;
                    Audio_PlayActorSound2(&mut (*this).actor,
                                          0x393d as libc::c_int as u16_0);
                    Actor_SetColorFilter(&mut (*this).actor,
                                         0x4000 as libc::c_int as s16,
                                         255 as libc::c_int as s16,
                                         0 as libc::c_int as s16,
                                         12 as libc::c_int as s16);
                    sPhase4HP =
                        (sPhase4HP as libc::c_int -
                             (*this).actor.colChkInfo.damage as libc::c_int)
                            as s8;
                    if sPhase4HP as libc::c_int <= 0 as libc::c_int {
                        (*this).timer = 0 as libc::c_int;
                        sFightPhase = sFightPhase.wrapping_add(1);
                        sPhase4HP =
                            (sPhase4HP as libc::c_int + 3 as libc::c_int) as
                                s8;
                        if sFightPhase as libc::c_int >= 18 as libc::c_int {
                            BossVa_SetupBodyDeath(this, globalCtx);
                            Enemy_StartFinishingBlow(globalCtx,
                                                     &mut (*this).actor);
                            return
                        }
                        (*this).actor.speedXZ = -10.0f32;
                        (*this).timer =
                            -(170 as libc::c_int) -
                                (Rand_ZeroOne() * 150.0f32) as s16 as
                                    libc::c_int
                    }
                } else {
                    (*this).timer =
                        Rand_CenteredFloat(40.0f32) as s16 as libc::c_int +
                            160 as libc::c_int;
                    (*this).headRot.y = 0 as libc::c_int as s16;
                    (*this).actor.speedXZ = 0.0f32;
                    Actor_SetColorFilter(&mut (*this).actor,
                                         0 as libc::c_int as s16,
                                         125 as libc::c_int as s16,
                                         0 as libc::c_int as s16,
                                         255 as libc::c_int as s16);
                    Audio_PlayActorSound2(&mut (*this).actor,
                                          0x393e as libc::c_int as u16_0);
                }
            }
        } else if (*(*this).colliderBody.base.ac).id as libc::c_int ==
                      ACTOR_EN_BOOM as libc::c_int {
            boomerang = (*this).colliderBody.base.ac as *mut EnBoom;
            (*boomerang).returnTimer = 0 as libc::c_int as u8_0;
            (*boomerang).moveTo = &mut (*player).actor;
            (*boomerang).actor.world.rot.y =
                (*boomerang).actor.yawTowardsPlayer;
            Audio_PlayActorSound2(&mut (*this).actor,
                                  0x1808 as libc::c_int as u16_0);
        }
    } else if (*this).timer2 as libc::c_int == 0 as libc::c_int &&
                  (*this).actor.shape.yOffset == 0.0f32 {
        (*this).timer =
            -(220 as libc::c_int) -
                (Rand_ZeroOne() * 200.0f32) as s16 as libc::c_int
    } else if (*this).timer2 as libc::c_int != 0 as libc::c_int {
        (*this).timer2 -= 1
    }
    SkelAnime_Update(&mut (*this).skelAnime);
    if (*this).timer == 0 as libc::c_int {
        Math_SmoothStepToF(&mut (*this).actor.shape.yOffset, 0.0f32, 1.0f32,
                           (sFightPhase as libc::c_int - 15 as libc::c_int +
                                1 as libc::c_int) as libc::c_float * 5.0f32 +
                               10.0f32, 0.0f32);
        if Math_SmoothStepToS(&mut (*this).headRot.y,
                              (((sFightPhase as libc::c_int -
                                     15 as libc::c_int + 1 as libc::c_int) as
                                    libc::c_float * 500.0f32) as s16 as
                                   libc::c_int + 0xfa0 as libc::c_int) as s16,
                              1 as libc::c_int as s16,
                              0x12c as libc::c_int as s16,
                              0 as libc::c_int as s16) as libc::c_int ==
               0 as libc::c_int {
            if (*this).actor.speedXZ == 0.0f32 {
                (*this).actor.colorFilterTimer = 0 as libc::c_int as u8_0;
                (*this).actor.world.rot.y = (*this).actor.yawTowardsPlayer;
                (*this).timer2 =
                    ((Rand_ZeroOne() * 150.0f32) as s16 as libc::c_int +
                         300 as libc::c_int) as s16
            }
            Math_SmoothStepToF(&mut (*this).actor.speedXZ,
                               (sFightPhase as libc::c_int - 15 as libc::c_int
                                    + 1 as libc::c_int) as libc::c_float *
                                   1.5f32 + 4.0f32, 1.0f32, 0.25f32, 0.0f32);
        }
        (*this).colliderBody.info.bumper.dmgFlags =
            0x10 as libc::c_int as u32_0
    } else {
        Math_SmoothStepToS(&mut (*this).headRot.y, 0 as libc::c_int as s16,
                           1 as libc::c_int as s16,
                           0x96 as libc::c_int as s16,
                           0 as libc::c_int as s16);
        if (*this).timer > 0 as libc::c_int {
            if (*player).stateFlags1 &
                   0x4000000 as libc::c_int as libc::c_uint != 0 &&
                   (*this).timer > 35 as libc::c_int {
                (*this).timer = 35 as libc::c_int
            }
            Math_SmoothStepToF(&mut (*this).actor.shape.yOffset, -480.0f32,
                               1.0f32, 30.0f32, 0.0f32);
            (*this).colliderBody.info.bumper.dmgFlags =
                0xfc00712 as libc::c_int as u32_0;
            (*this).timer -= 1
        } else {
            if (*player).stateFlags1 &
                   0x4000000 as libc::c_int as libc::c_uint != 0 &&
                   (*this).timer < -(60 as libc::c_int) {
                (*this).timer = -(59 as libc::c_int)
            }
            if (*globalCtx).gameplayFrames.wrapping_rem(4 as libc::c_int as
                                                            libc::c_uint) ==
                   0 as libc::c_int as libc::c_uint {
                BossVa_Spark(globalCtx, this, 2 as libc::c_int,
                             0x64 as libc::c_int as s16, 220.0f32, 5.0f32,
                             SPARK_BODY as libc::c_int as u8_0, 12.0f32,
                             1 as libc::c_int as u8_0);
            }
            if (*this).timer < -(30 as libc::c_int) {
                if (*this).actor.speedXZ > 0.0f32 {
                    Math_SmoothStepToF(&mut (*this).actor.speedXZ, 0.0f32,
                                       1.0f32, 0.5f32, 0.0f32);
                }
                Math_SmoothStepToF(&mut (*this).actor.shape.yOffset,
                                   -1400.0f32, 1.0f32, 60.0f32, 0.0f32);
            } else {
                if (*this).actor.speedXZ == 0.0f32 {
                    (*this).actor.world.rot.y =
                        ((*this).actor.yawTowardsPlayer as libc::c_int +
                             0x8000 as libc::c_int) as s16;
                    (*this).timer2 =
                        ((Rand_ZeroOne() * 150.0f32) as s16 as libc::c_int +
                             330 as libc::c_int) as s16
                }
                Math_SmoothStepToS(&mut (*this).headRot.y,
                                   0xfa0 as libc::c_int as s16,
                                   1 as libc::c_int as s16,
                                   0x1f4 as libc::c_int as s16,
                                   0 as libc::c_int as s16);
                tmpf1 =
                    (sFightPhase as libc::c_int - 15 as libc::c_int +
                         1 as libc::c_int) as f32_0;
                Math_SmoothStepToF(&mut (*this).actor.speedXZ,
                                   tmpf1 + tmpf1 + 4.0f32, 1.0f32, 0.25f32,
                                   0.0f32);
                Math_SmoothStepToF(&mut (*this).actor.shape.yOffset, 0.0f32,
                                   1.0f32, 20.0f32, 0.0f32);
            }
            (*this).timer += 1
        }
    }
    (*this).actor.shape.rot.y =
        ((*this).actor.shape.rot.y as libc::c_int +
             (*this).headRot.y as libc::c_int) as s16;
    if (*this).actor.speedXZ < 0.0f32 {
        Math_SmoothStepToF(&mut (*this).actor.speedXZ, 0.0f32, 1.0f32, 0.5f32,
                           0.0f32);
    }
    (*this).unk_1AC =
        ((*this).unk_1AC as libc::c_int + 0xc31 as libc::c_int) as s16;
    (*this).unk_1A0 = Math_CosS((*this).unk_1AC) * 0.1f32 + 1.0f32;
    (*this).unk_1A4 = Math_SinS((*this).unk_1AC) * 0.05f32 + 1.0f32;
    if (*this).actor.bgCheckFlags as libc::c_int & 8 as libc::c_int != 0 {
        (*this).actor.bgCheckFlags =
            ((*this).actor.bgCheckFlags as libc::c_int & !(8 as libc::c_int))
                as u16_0;
        (*this).actor.world.rot.y =
            (Rand_CenteredFloat((30 as libc::c_int *
                                     (0x10000 as libc::c_int /
                                          360 as libc::c_int)) as f32_0) as
                 s16 as libc::c_int + (*this).actor.wallYaw as libc::c_int) as
                s16
    }
    if sFightPhase as libc::c_int <= 15 as libc::c_int {
        if Math_Vec3f_DistXZ(&mut (*this).actor.world.pos,
                             &mut (*this).actor.home.pos) >= 400.0f32 {
            Math_SmoothStepToS(&mut (*this).actor.world.rot.y,
                               Math_Vec3f_Yaw(&mut (*this).actor.world.pos,
                                              &mut (*this).actor.home.pos),
                               1 as libc::c_int as s16,
                               0x5dc as libc::c_int as s16,
                               0 as libc::c_int as s16);
        } else if (*player).invincibilityTimer as libc::c_int !=
                      0 as libc::c_int {
            Math_SmoothStepToS(&mut (*this).actor.world.rot.y,
                               ((*this).actor.yawTowardsPlayer as libc::c_int
                                    + 0x8000 as libc::c_int) as s16,
                               1 as libc::c_int as s16,
                               0x12c as libc::c_int as s16,
                               0 as libc::c_int as s16);
        } else if (*globalCtx).gameplayFrames &
                      0x80 as libc::c_int as libc::c_uint ==
                      0 as libc::c_int as libc::c_uint {
            Math_SmoothStepToS(&mut (*this).actor.world.rot.y,
                               (*this).actor.yawTowardsPlayer,
                               1 as libc::c_int as s16,
                               (((sFightPhase as libc::c_int -
                                      15 as libc::c_int + 1 as libc::c_int) as
                                     libc::c_float * 100.0f32) as s16 as
                                    libc::c_int + 0x64 as libc::c_int) as s16,
                               0 as libc::c_int as s16);
        }
    }
    Actor_MoveForward(&mut (*this).actor);
    (*this).actor.focus.pos = (*this).actor.world.pos;
    (*this).actor.focus.pos.y += 60.0f32;
    if (*globalCtx).gameplayFrames.wrapping_rem(2 as libc::c_int as
                                                    libc::c_uint) ==
           0 as libc::c_int as libc::c_uint &&
           (*this).timer == 0 as libc::c_int {
        BossVa_Spark(globalCtx, this, 2 as libc::c_int,
                     125 as libc::c_int as s16, 40.0f32, 10.0f32,
                     SPARK_BODY as libc::c_int as u8_0, 10.0f32,
                     0 as libc::c_int as u8_0);
        BossVa_Spark(globalCtx, this, 1 as libc::c_int,
                     100 as libc::c_int as s16, 15.0f32, 10.0f32,
                     SPARK_BARI as libc::c_int as u8_0, 11.0f32,
                     1 as libc::c_int as u8_0);
    }
    Actor_UpdateBgCheckInfo(globalCtx, &mut (*this).actor, 30.0f32, 70.0f32,
                            0.0f32, 1 as libc::c_int);
    Collider_UpdateCylinder(&mut (*this).actor, &mut (*this).colliderBody);
    CollisionCheck_SetOC(globalCtx, &mut (*globalCtx).colChkCtx,
                         &mut (*this).colliderBody.base);
    if (*this).invincibilityTimer as libc::c_int == 0 as libc::c_int {
        CollisionCheck_SetAC(globalCtx, &mut (*globalCtx).colChkCtx,
                             &mut (*this).colliderBody.base);
    }
    if (*this).headRot.y as libc::c_int > 0x3e8 as libc::c_int ||
           (*this).actor.shape.yOffset < -1200.0f32 {
        CollisionCheck_SetAT(globalCtx, &mut (*globalCtx).colChkCtx,
                             &mut (*this).colliderBody.base);
    }
    func_800F436C(&mut (*this).actor.projectedPos,
                  (0x393c as libc::c_int - 0x800 as libc::c_int) as u16_0,
                  (*this).headRot.y as libc::c_int as libc::c_float *
                      0.00025f32 + 1.0f32);
    if (*this).invincibilityTimer as libc::c_int != 0 as libc::c_int {
        (*this).invincibilityTimer -= 1;
        sBodyState =
            (sBodyState as libc::c_int & 0x80 as libc::c_int |
                 2 as libc::c_int) as u8_0
    } else {
        sBodyState =
            (sBodyState as libc::c_int & 0x80 as libc::c_int |
                 1 as libc::c_int) as u8_0
    };
}
#[no_mangle]
pub unsafe extern "C" fn BossVa_SetupBodyDeath(mut this: *mut BossVa,
                                               mut globalCtx:
                                                   *mut GlobalContext) {
    func_800F436C(&mut (*this).actor.projectedPos,
                  (0x393c as libc::c_int - 0x800 as libc::c_int) as u16_0,
                  1.0f32);
    (*this).actor.flags &=
        !((1 as libc::c_int) << 0 as libc::c_int |
              (1 as libc::c_int) << 2 as libc::c_int) as libc::c_uint;
    Audio_QueueSeqCmd(((0x1 as libc::c_int) << 28 as libc::c_int |
                           (SEQ_PLAYER_BGM_MAIN as libc::c_int) <<
                               24 as libc::c_int | 0x100ff as libc::c_int) as
                          u32_0);
    (*this).headRot.x = 0xc31 as libc::c_int as s16;
    sCsState = DEATH_START as libc::c_int as s8;
    (*this).actor.speedXZ = 0.0f32;
    (*this).unk_1A8 = 0.0f32;
    Flags_SetClear(globalCtx, (*globalCtx).roomCtx.curRoom.num as s32);
    BossVa_SetupAction(this,
                       Some(BossVa_BodyDeath as
                                unsafe extern "C" fn(_: *mut BossVa,
                                                     _: *mut GlobalContext)
                                    -> ()));
}
#[no_mangle]
pub unsafe extern "C" fn BossVa_BodyDeath(mut this: *mut BossVa,
                                          mut globalCtx: *mut GlobalContext) {
    let mut i: s32 = 0;
    let mut camera: *mut Camera =
        Gameplay_GetCamera(globalCtx, 0 as libc::c_int as s16);
    let mut sp7C: s32 = 0;
    let mut player: *mut Player =
        (*globalCtx).actorCtx.actorLists[ACTORCAT_PLAYER as libc::c_int as
                                             usize].head as *mut Player;
    let mut tmp16: s16 = 0;
    let mut current_block_91: u64;
    match sCsState as libc::c_int {
        14 => {
            func_8002DF54(globalCtx, &mut (*this).actor,
                          1 as libc::c_int as u8_0);
            func_80064520(globalCtx, &mut (*globalCtx).csCtx);
            sCsCamera = Gameplay_CreateSubCamera(globalCtx);
            Gameplay_ChangeCameraStatus(globalCtx, 0 as libc::c_int as s16,
                                        1 as libc::c_int as s16);
            Gameplay_ChangeCameraStatus(globalCtx, sCsCamera,
                                        7 as libc::c_int as s16);
            sCameraNextAt.x = (*this).actor.world.pos.x;
            sCameraNextAt.y = (*this).actor.world.pos.y;
            sCameraNextAt.z = (*this).actor.world.pos.z;
            sCameraAt = (*camera).at;
            sCameraEye = (*camera).eye;
            sCameraNextEye = sCameraEye;
            sCameraNextEye.y = 40.0f32;
            sCameraNextAt.y = 140.0f32;
            sCameraEyeMaxVel = sZeroVec;
            sCameraAtMaxVel = sCameraEyeMaxVel;
            (*this).unk_1AC =
                (Math_Vec3f_Yaw(&mut sCameraEye, &mut sCameraNextAt) as
                     libc::c_int - 0x100 as libc::c_int) as s16;
            (*this).unk_1B0 = 15 as libc::c_int as s16;
            (*globalCtx).envCtx.screenFillColor[2 as libc::c_int as usize] =
                0xff as libc::c_int as u8_0;
            (*globalCtx).envCtx.screenFillColor[1 as libc::c_int as usize] =
                (*globalCtx).envCtx.screenFillColor[2 as libc::c_int as
                                                        usize];
            (*globalCtx).envCtx.screenFillColor[0 as libc::c_int as usize] =
                (*globalCtx).envCtx.screenFillColor[1 as libc::c_int as
                                                        usize];
            (*globalCtx).envCtx.screenFillColor[3 as libc::c_int as usize] =
                0 as libc::c_int as u8_0;
            (*globalCtx).envCtx.fillScreen = 1 as libc::c_int as u8_0;
            sCsState += 1;
            current_block_91 = 7302789546274060726;
        }
        15 => { current_block_91 = 7302789546274060726; }
        21 => {
            (*this).unk_1AC =
                ((*this).unk_1AC as libc::c_int + 0x1862 as libc::c_int) as
                    s16;
            (*this).unk_1A0 = Math_CosS((*this).unk_1AC) * 0.12f32 + 1.0f32;
            (*this).unk_1A4 = Math_SinS((*this).unk_1AC) * 0.075f32 + 1.0f32;
            if (*this).isDead == 0 {
                (*this).burst = (*this).burst.wrapping_add(1);
                (*this).isDead = (*this).isDead.wrapping_add(1);
                (*this).timer = 30 as libc::c_int;
                sCsState += 1;
                EffectSsDeadSound_SpawnStationary(globalCtx,
                                                  &mut (*this).actor.projectedPos,
                                                  0x3940 as libc::c_int as
                                                      u16_0,
                                                  1 as libc::c_int as s16,
                                                  1 as libc::c_int as s16,
                                                  0x28 as libc::c_int);
                (*this).onCeiling = 2 as libc::c_int as u8_0;
                BossVa_SetDeathEnv(globalCtx);
                func_8002DF54(globalCtx, &mut (*this).actor,
                              8 as libc::c_int as u8_0);
            }
            current_block_91 = 14329534724295951598;
        }
        22 => {
            if (*this).timer == 13 as libc::c_int {
                Audio_QueueSeqCmd(((SEQ_PLAYER_BGM_MAIN as libc::c_int) <<
                                       24 as libc::c_int |
                                       0x21 as libc::c_int) as u32_0);
            }
            (*this).timer -= 1;
            if (*this).timer == 0 as libc::c_int {
                sCameraNextAt.x = (*this).actor.world.pos.x;
                sCameraNextAt.y = (*this).actor.world.pos.y + 30.0f32;
                sCameraNextAt.z = (*this).actor.world.pos.z;
                sCameraNextEye.x =
                    Math_SinS((*player).actor.shape.rot.y) * -130.0f32 +
                        (*player).actor.world.pos.x;
                sCameraNextEye.z =
                    Math_CosS((*player).actor.shape.rot.y) * -130.0f32 +
                        (*player).actor.world.pos.z;
                sCameraNextEye.y = (*player).actor.world.pos.y + 55.0f32;
                sCameraEyeMaxVel = sZeroVec;
                sCameraAtMaxVel = sCameraEyeMaxVel;
                sCsState += 1;
                (*this).timer = 133 as libc::c_int
            }
            current_block_91 = 14329534724295951598;
        }
        23 => {
            Math_SmoothStepToF(&mut sCameraEyeMaxVel.x, 1.5f32, 0.3f32,
                               0.05f32, 0.015f32);
            sCameraEyeMaxVel.z = sCameraEyeMaxVel.x;
            sCameraEyeMaxVel.y = sCameraEyeMaxVel.z;
            sCameraAtMaxVel = sCameraEyeMaxVel;
            (*this).timer -= 1;
            if (*this).timer == 0 as libc::c_int {
                Gameplay_ClearCamera(globalCtx, sCsCamera);
                sCsCamera = 0 as libc::c_int as s16;
                func_80064534(globalCtx, &mut (*globalCtx).csCtx);
                Gameplay_ChangeCameraStatus(globalCtx,
                                            0 as libc::c_int as s16,
                                            7 as libc::c_int as s16);
                (*camera).eye = sCameraEye;
                (*camera).eyeNext = (*camera).eye;
                (*camera).at = sCameraAt;
                func_8002DF54(globalCtx, &mut (*this).actor,
                              7 as libc::c_int as u8_0);
                sCsState += 1;
                Actor_Spawn(&mut (*globalCtx).actorCtx, globalCtx,
                            ACTOR_ITEM_B_HEART as libc::c_int as s16,
                            (*this).actor.world.pos.x,
                            (*this).actor.world.pos.y,
                            (*this).actor.world.pos.z,
                            0 as libc::c_int as s16, 0 as libc::c_int as s16,
                            0 as libc::c_int as s16, 0 as libc::c_int as s16);
                i = 2 as libc::c_int;
                sp7C = 2 as libc::c_int;
                while i > 0 as libc::c_int {
                    if Math_Vec3f_DistXYZ(&mut *sWarpPos.as_mut_ptr().offset(i
                                                                                 as
                                                                                 isize),
                                          &mut (*player).actor.world.pos) <
                           Math_Vec3f_DistXYZ(&mut *sWarpPos.as_mut_ptr().offset((i
                                                                                      -
                                                                                      1
                                                                                          as
                                                                                          libc::c_int)
                                                                                     as
                                                                                     isize),
                                              &mut (*player).actor.world.pos)
                       {
                        sp7C = i - 1 as libc::c_int
                    }
                    i -= 1
                }
                Actor_Spawn(&mut (*globalCtx).actorCtx, globalCtx,
                            ACTOR_EN_RU1 as libc::c_int as s16,
                            sWarpPos[sp7C as usize].x,
                            sWarpPos[sp7C as usize].y,
                            sWarpPos[sp7C as usize].z,
                            0 as libc::c_int as s16, 0 as libc::c_int as s16,
                            0 as libc::c_int as s16, 0 as libc::c_int as s16);
            }
            current_block_91 = 15883151696356780458;
        }
        24 => { current_block_91 = 15883151696356780458; }
        _ => { current_block_91 = 14329534724295951598; }
    }
    match current_block_91 {
        15883151696356780458 => {
            Rand_CenteredFloat(0.5f32);
            (*globalCtx).envCtx.fillScreen = 0 as libc::c_int as u8_0
        }
        7302789546274060726 => {
            (*this).unk_1AC =
                ((*this).unk_1AC as libc::c_int + 0x100 as libc::c_int) as
                    s16;
            sCameraNextEye.x =
                Math_SinS((*this).unk_1AC) * (160.0f32 + (*this).unk_1A8) +
                    sCameraNextAt.x;
            sCameraNextEye.z =
                Math_CosS((*this).unk_1AC) * (160.0f32 + (*this).unk_1A8) +
                    sCameraNextAt.z;
            Math_SmoothStepToF(&mut sCameraEyeMaxVel.x, 16.0f32, 0.4f32,
                               1.5f32, 0.5f32);
            sCameraEyeMaxVel.z = sCameraEyeMaxVel.x;
            sCameraEyeMaxVel.y = sCameraEyeMaxVel.x * 0.5f32;
            sCameraAtMaxVel = sCameraEyeMaxVel;
            tmp16 =
                (Rand_CenteredFloat(0.5f32) +
                     (sCameraEyeMaxVel.x * 0.5f32 + 0.6f32)) as s16;
            if (*globalCtx).gameplayFrames.wrapping_rem(4 as libc::c_int as
                                                            libc::c_uint) ==
                   0 as libc::c_int as libc::c_uint &&
                   (*this).unk_1B0 as libc::c_int != 0 as libc::c_int {
                i = 6 as libc::c_int;
                while i > 1 as libc::c_int {
                    BossVa_Tumor(globalCtx, this, 1 as libc::c_int, tmp16,
                                 0.0f32, 0.0f32,
                                 TUMOR_BODY as libc::c_int as u8_0,
                                 i as f32_0, 1 as libc::c_int as u8_0);
                    i -= 1
                }
                BossVa_Tumor(globalCtx, this, 1 as libc::c_int, tmp16, 0.0f32,
                             0.0f32, TUMOR_BODY as libc::c_int as u8_0,
                             11.0f32, 1 as libc::c_int as u8_0);
                (*this).unk_1B0 -= 1
            }
            if (*this).unk_1B0 as libc::c_int == 0 as libc::c_int {
                sCsState += 1;
                sCameraEyeMaxVel = sZeroVec;
                sCameraAtMaxVel = sCameraEyeMaxVel
            }
        }
        _ => { }
    }
    if sCsCamera as libc::c_int != 0 as libc::c_int {
        Math_SmoothStepToF(&mut sCameraEye.x, sCameraNextEye.x, 0.3f32,
                           sCameraEyeMaxVel.x, 0.15f32);
        Math_SmoothStepToF(&mut sCameraEye.y, sCameraNextEye.y, 0.3f32,
                           sCameraEyeMaxVel.y, 0.15f32);
        Math_SmoothStepToF(&mut sCameraEye.z, sCameraNextEye.z, 0.3f32,
                           sCameraEyeMaxVel.z, 0.15f32);
        Math_SmoothStepToF(&mut sCameraAt.x, sCameraNextAt.x, 0.3f32,
                           sCameraAtMaxVel.x, 0.15f32);
        Math_SmoothStepToF(&mut sCameraAt.y, sCameraNextAt.y, 0.3f32,
                           sCameraAtMaxVel.y, 0.15f32);
        Math_SmoothStepToF(&mut sCameraAt.z, sCameraNextAt.z, 0.3f32,
                           sCameraAtMaxVel.z, 0.15f32);
        Gameplay_CameraSetAtEye(globalCtx, sCsCamera, &mut sCameraAt,
                                &mut sCameraEye);
    }
    SkelAnime_Update(&mut (*this).skelAnime);
    Math_SmoothStepToF(&mut (*this).actor.shape.yOffset, -480.0f32, 1.0f32,
                       30.0f32, 0.0f32);
    Math_SmoothStepToS(&mut (*this).headRot.y, 0 as libc::c_int as s16,
                       1 as libc::c_int as s16, 0xc8 as libc::c_int as s16,
                       0 as libc::c_int as s16);
    Math_SmoothStepToS(&mut (*this).headRot.x, 0 as libc::c_int as s16,
                       1 as libc::c_int as s16, 0xc8 as libc::c_int as s16,
                       0 as libc::c_int as s16);
    Math_SmoothStepToS(&mut (*this).bodyGlow, 200 as libc::c_int as s16,
                       1 as libc::c_int as s16, 10 as libc::c_int as s16,
                       0 as libc::c_int as s16);
    if (*globalCtx).envCtx.screenFillColor[3 as libc::c_int as usize] as
           libc::c_int != 0 as libc::c_int {
        (*globalCtx).envCtx.screenFillColor[3 as libc::c_int as usize] =
            ((*globalCtx).envCtx.screenFillColor[3 as libc::c_int as usize] as
                 libc::c_int - 50 as libc::c_int) as u8_0
    }
    Math_SmoothStepToF(&mut (*this).actor.speedXZ, 0.0f32, 1.0f32, 0.5f32,
                       0.0f32);
    (*this).actor.shape.rot.y =
        ((*this).actor.shape.rot.y as libc::c_int +
             (*this).headRot.y as libc::c_int) as s16;
    (*this).unk_1AC =
        ((*this).unk_1AC as libc::c_int + (*this).headRot.x as libc::c_int) as
            s16;
    (*this).unk_1A0 = Math_CosS((*this).unk_1AC) * 0.1f32 + 1.0f32;
    (*this).unk_1A4 = Math_SinS((*this).unk_1AC) * 0.05f32 + 1.0f32;
}
#[no_mangle]
pub unsafe extern "C" fn BossVa_SetupSupportIntro(mut this: *mut BossVa,
                                                  mut globalCtx:
                                                      *mut GlobalContext) {
    Animation_Change(&mut (*this).skelAnime,
                     &mut gBarinadeSupportAttachedAnim, 0.0f32, 0.0f32,
                     Animation_GetLastFrame(&mut gBarinadeSupportAttachedAnim
                                                as *mut AnimationHeader as
                                                *mut libc::c_void) as f32_0,
                     ANIMMODE_LOOP_INTERP as libc::c_int as u8_0, 0.0f32);
    (*this).timer = 0 as libc::c_int;
    BossVa_SetupAction(this,
                       Some(BossVa_SupportIntro as
                                unsafe extern "C" fn(_: *mut BossVa,
                                                     _: *mut GlobalContext)
                                    -> ()));
}
#[no_mangle]
pub unsafe extern "C" fn BossVa_SupportIntro(mut this: *mut BossVa,
                                             mut globalCtx:
                                                 *mut GlobalContext) {
    BossVa_AttachToBody(this);
    if sCsState as libc::c_int == BOSSVA_BATTLE as libc::c_int {
        BossVa_SetupSupportAttached(this, globalCtx);
    } else if sCsState as libc::c_int >= INTRO_REVERSE_CAMERA as libc::c_int {
        (*this).timer += 1;
        if (*this).timer % 2 as libc::c_int == 0 as libc::c_int {
            BossVa_Spark(globalCtx, this, 2 as libc::c_int,
                         90 as libc::c_int as s16, 5.0f32, 0.0f32,
                         SPARK_BODY as libc::c_int as u8_0,
                         ((((*this).timer & 0x20 as libc::c_int) >>
                               5 as libc::c_int) + 1 as libc::c_int) as f32_0,
                         1 as libc::c_int as u8_0);
        }
        SkelAnime_Update(&mut (*this).skelAnime);
        Math_SmoothStepToF(&mut (*this).skelAnime.playSpeed, 1.0f32, 1.0f32,
                           0.05f32, 0.0f32);
        if Rand_ZeroOne() < 0.1f32 {
            Audio_PlayActorSound2(&mut (*this).actor,
                                  (0x3943 as libc::c_int -
                                       0x800 as libc::c_int) as u16_0);
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn BossVa_SetupSupportAttached(mut this: *mut BossVa,
                                                     mut globalCtx:
                                                         *mut GlobalContext) {
    Animation_Change(&mut (*this).skelAnime,
                     &mut gBarinadeSupportAttachedAnim, 1.0f32, 0.0f32,
                     Animation_GetLastFrame(&mut gBarinadeSupportAttachedAnim
                                                as *mut AnimationHeader as
                                                *mut libc::c_void) as f32_0,
                     ANIMMODE_LOOP as libc::c_int as u8_0, 0.0f32);
    (*this).timer = (*this).actor.params as libc::c_int * 10 as libc::c_int;
    BossVa_SetupAction(this,
                       Some(BossVa_SupportAttached as
                                unsafe extern "C" fn(_: *mut BossVa,
                                                     _: *mut GlobalContext)
                                    -> ()));
}
#[no_mangle]
pub unsafe extern "C" fn BossVa_SupportAttached(mut this: *mut BossVa,
                                                mut globalCtx:
                                                    *mut GlobalContext) {
    (*this).timer += 1;
    if sBodyState as libc::c_int & 0x7f as libc::c_int != 0 {
        Actor_SetColorFilter(&mut (*this).actor, 0 as libc::c_int as s16,
                             255 as libc::c_int as s16,
                             0 as libc::c_int as s16,
                             12 as libc::c_int as s16);
        if Rand_ZeroOne() > 0.5f32 {
            Animation_Change(&mut (*this).skelAnime,
                             &mut gBarinadeSupportDamage1Anim, 1.0f32, 0.0f32,
                             Animation_GetLastFrame(&mut gBarinadeSupportDamage1Anim
                                                        as
                                                        *mut AnimationHeader
                                                        as *mut libc::c_void)
                                 as f32_0,
                             ANIMMODE_ONCE as libc::c_int as u8_0, 0.0f32);
        } else {
            Animation_Change(&mut (*this).skelAnime,
                             &mut gBarinadeSupportDamage2Anim, 1.0f32, 0.0f32,
                             Animation_GetLastFrame(&mut gBarinadeSupportDamage2Anim
                                                        as
                                                        *mut AnimationHeader
                                                        as *mut libc::c_void)
                                 as f32_0,
                             ANIMMODE_ONCE as libc::c_int as u8_0, 0.0f32);
        }
    }
    if SkelAnime_Update(&mut (*this).skelAnime) != 0 {
        Animation_Change(&mut (*this).skelAnime,
                         &mut gBarinadeSupportAttachedAnim, 1.0f32, 0.0f32,
                         Animation_GetLastFrame(&mut gBarinadeSupportAttachedAnim
                                                    as *mut AnimationHeader as
                                                    *mut libc::c_void) as
                             f32_0, ANIMMODE_LOOP as libc::c_int as u8_0,
                         0.0f32);
    }
    BossVa_AttachToBody(this);
    if Rand_ZeroOne() < 0.1f32 {
        Audio_PlayActorSound2(&mut (*this).actor,
                              (0x3943 as libc::c_int - 0x800 as libc::c_int)
                                  as u16_0);
    }
    if (*this).colliderSph.base.acFlags as libc::c_int &
           (1 as libc::c_int) << 1 as libc::c_int != 0 {
        BossVa_SetupSupportCut(this, globalCtx);
    } else {
        if (*this).actor.colorFilterTimer as libc::c_int == 0 as libc::c_int {
            CollisionCheck_SetAC(globalCtx, &mut (*globalCtx).colChkCtx,
                                 &mut (*this).colliderSph.base);
        }
        if (*this).timer % 2 as libc::c_int == 0 as libc::c_int {
            BossVa_Spark(globalCtx, this, 1 as libc::c_int,
                         100 as libc::c_int as s16, 5.0f32, 0.0f32,
                         SPARK_BODY as libc::c_int as u8_0,
                         ((((*this).timer & 0x20 as libc::c_int) >>
                               5 as libc::c_int) + 1 as libc::c_int) as f32_0,
                         1 as libc::c_int as u8_0);
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn BossVa_SetupSupportCut(mut this: *mut BossVa,
                                                mut globalCtx:
                                                    *mut GlobalContext) {
    let mut stumpParams: s32 =
        (*this).actor.params as libc::c_int + BOSSVA_STUMP_1 as libc::c_int;
    sBodyState = sBodyState.wrapping_add(1);
    sFightPhase = sFightPhase.wrapping_add(1);
    Actor_Spawn(&mut (*globalCtx).actorCtx, globalCtx,
                ACTOR_BOSS_VA as libc::c_int as s16, (*this).armTip.x,
                (*this).armTip.y + 20.0f32, (*this).armTip.z,
                0 as libc::c_int as s16, (*this).actor.shape.rot.y,
                0 as libc::c_int as s16, stumpParams as s16);
    Camera_AddQuake(&mut (*globalCtx).mainCamera, 2 as libc::c_int,
                    11 as libc::c_int as s16, 8 as libc::c_int);
    (*this).burst = 0 as libc::c_int as u8_0;
    (*this).timer2 = 0 as libc::c_int as s16;
    BossVa_SetupAction(this,
                       Some(BossVa_SupportCut as
                                unsafe extern "C" fn(_: *mut BossVa,
                                                     _: *mut GlobalContext)
                                    -> ()));
}
#[no_mangle]
pub unsafe extern "C" fn BossVa_SupportCut(mut this: *mut BossVa,
                                           mut globalCtx:
                                               *mut GlobalContext) {
    let mut vaBody: *mut BossVa = (*this).actor.parent as *mut BossVa;
    let mut lastFrame: f32_0 = 0.;
    BossVa_AttachToBody(this);
    if (*this).onCeiling != 0 {
        lastFrame =
            Animation_GetLastFrame(&mut gBarinadeSupportCutAnim as
                                       *mut AnimationHeader as
                                       *mut libc::c_void) as f32_0;
        (*this).onCeiling = 0 as libc::c_int as u8_0;
        (*this).timer = (Rand_ZeroOne() * 10.0f32) as s32 + 5 as libc::c_int;
        SkelAnime_Free(&mut (*this).skelAnime, globalCtx);
        SkelAnime_InitFlex(globalCtx, &mut (*this).skelAnime,
                           &mut gBarinadeCutSupportSkel,
                           &mut gBarinadeSupportCutAnim, 0 as *mut Vec3s,
                           0 as *mut Vec3s, 0 as libc::c_int);
        Animation_Change(&mut (*this).skelAnime, &mut gBarinadeSupportCutAnim,
                         1.0f32, 0.0f32, lastFrame,
                         ANIMMODE_ONCE as libc::c_int as u8_0, 0.0f32);
        sBodyState = 0 as libc::c_int as u8_0;
        let ref mut fresh0 =
            (*((*this).actor.parent as *mut BossVa)).actor.shape.yOffset;
        *fresh0 -= 60.0f32;
        match (*this).actor.params as libc::c_int {
            0 => {
                let ref mut fresh1 =
                    (*((*this).actor.parent as
                           *mut BossVa)).actor.world.rot.x;
                *fresh1 =
                    (*fresh1 as libc::c_int + 0x4b0 as libc::c_int) as s16
            }
            1 => {
                let ref mut fresh2 =
                    (*((*this).actor.parent as
                           *mut BossVa)).actor.world.rot.x;
                *fresh2 =
                    (*fresh2 as libc::c_int - 0x258 as libc::c_int) as s16;
                let ref mut fresh3 =
                    (*((*this).actor.parent as
                           *mut BossVa)).actor.world.rot.z;
                *fresh3 =
                    (*fresh3 as libc::c_int - 0x4e2 as libc::c_int) as s16
            }
            2 => {
                let ref mut fresh4 =
                    (*((*this).actor.parent as
                           *mut BossVa)).actor.world.rot.x;
                *fresh4 =
                    (*fresh4 as libc::c_int - 0x258 as libc::c_int) as s16;
                let ref mut fresh5 =
                    (*((*this).actor.parent as
                           *mut BossVa)).actor.world.rot.z;
                *fresh5 =
                    (*fresh5 as libc::c_int + 0x4e2 as libc::c_int) as s16
            }
            _ => { }
        }
    }
    Math_SmoothStepToS(&mut (*this).headRot.x,
                       ((*vaBody).headRot.y as libc::c_int *
                            -(3 as libc::c_int)) as s16,
                       1 as libc::c_int as s16, 0x4b0 as libc::c_int as s16,
                       0 as libc::c_int as s16);
    if SkelAnime_Update(&mut (*this).skelAnime) != 0 {
        lastFrame =
            Animation_GetLastFrame(&mut gBarinadeSupportDetachedAnim as
                                       *mut AnimationHeader as
                                       *mut libc::c_void) as f32_0;
        Animation_Change(&mut (*this).skelAnime,
                         &mut gBarinadeSupportDetachedAnim, 1.0f32, 0.0f32,
                         lastFrame,
                         ANIMMODE_LOOP_INTERP as libc::c_int as u8_0, 0.0f32);
        (*this).actor.flags &=
            !((1 as libc::c_int) << 0 as libc::c_int) as libc::c_uint
    }
    if (*this).timer == 0 as libc::c_int &&
           (sCsState as libc::c_int) < DEATH_START as libc::c_int {
        (*this).timer = (Rand_ZeroOne() * 10.0f32) as s32 + 10 as libc::c_int;
        BossVa_BloodDroplets(globalCtx, &mut (*this).armTip,
                             (*this).headRot.x, (*this).actor.shape.rot.y);
    }
    if sCsState as libc::c_int >= DEATH_START as libc::c_int {
        Math_SmoothStepToF(&mut (*this).skelAnime.playSpeed, 0.0f32, 0.3f32,
                           0.25f32, 0.125f32);
    }
    let mut current_block_54: u64;
    match sCsState as libc::c_int {
        19 => {
            sCameraEye = sCameraNextEye;
            sCameraAt = sCameraNextAt;
            Math_SmoothStepToF(&mut sCameraEye.x, sCameraNextAt.x, 1.0f32,
                               10.0f32, 0.0f32);
            Math_SmoothStepToF(&mut sCameraEye.z, sCameraNextAt.z, 1.0f32,
                               10.0f32, 0.0f32);
            sCameraEye.y += 20.0f32;
            sCsState += 1;
            current_block_54 = 5693692818851092359;
        }
        20 | 21 | 22 => { current_block_54 = 5693692818851092359; }
        _ => { current_block_54 = 17233182392562552756; }
    }
    match current_block_54 {
        5693692818851092359 => {
            if (*this).burst == 0 {
                if (*globalCtx).gameplayFrames.wrapping_rem(2 as libc::c_int
                                                                as
                                                                libc::c_uint)
                       != 0 as libc::c_int as libc::c_uint {
                    BossVa_Tumor(globalCtx, this, 1 as libc::c_int,
                                 (Rand_CenteredFloat(5.0f32) as s16 as
                                      libc::c_int + 6 as libc::c_int) as s16,
                                 7.0f32, 5.0f32,
                                 TUMOR_ARM as libc::c_int as u8_0,
                                 (((*this).timer2 as libc::c_int >>
                                       3 as libc::c_int) + 1 as libc::c_int)
                                     as f32_0, 1 as libc::c_int as u8_0);
                }
                (*this).timer2 += 1;
                if (*this).timer2 as libc::c_int >= 32 as libc::c_int {
                    (*this).burst = (*this).burst.wrapping_add(1);
                    (*this).isDead = 1 as libc::c_int as u8_0;
                    Audio_PlayActorSound2(&mut (*this).actor,
                                          0x3945 as libc::c_int as u16_0);
                    if (*this).actor.params as libc::c_int ==
                           BOSSVA_SUPPORT_3 as libc::c_int {
                        sCsState += 1
                    }
                }
            } else {
                (*this).timer2 -= 1;
                if (*this).timer2 as libc::c_int == 0 as libc::c_int {
                    Actor_Kill(&mut (*this).actor);
                }
            }
        }
        _ => { }
    }
    (*this).timer -= 1;
}
#[no_mangle]
pub unsafe extern "C" fn BossVa_SetupStump(mut this: *mut BossVa,
                                           mut globalCtx:
                                               *mut GlobalContext) {
    Animation_Change(&mut (*this).skelAnime, &mut gBarinadeStumpAnim, 1.0f32,
                     0.0f32,
                     Animation_GetLastFrame(&mut gBarinadeStumpAnim as
                                                *mut AnimationHeader as
                                                *mut libc::c_void) as f32_0,
                     ANIMMODE_ONCE as libc::c_int as u8_0, 0.0f32);
    (*this).actor.flags &=
        !((1 as libc::c_int) << 0 as libc::c_int) as libc::c_uint;
    BossVa_SetupAction(this,
                       Some(BossVa_Stump as
                                unsafe extern "C" fn(_: *mut BossVa,
                                                     _: *mut GlobalContext)
                                    -> ()));
}
#[no_mangle]
pub unsafe extern "C" fn BossVa_Stump(mut this: *mut BossVa,
                                      mut globalCtx: *mut GlobalContext) {
    if SkelAnime_Update(&mut (*this).skelAnime) != 0 &&
           Rand_ZeroOne() < 0.3f32 {
        (*this).skelAnime.curFrame -= Rand_ZeroOne() * 3.0f32
    }
    if sCsState as libc::c_int >= DEATH_START as libc::c_int {
        Actor_Kill(&mut (*this).actor);
    };
}
#[no_mangle]
pub unsafe extern "C" fn BossVa_SetupZapperIntro(mut this: *mut BossVa,
                                                 mut globalCtx:
                                                     *mut GlobalContext) {
    let mut lastFrame: f32_0 =
        Animation_GetLastFrame(&mut gBarinadeZapperIdleAnim as
                                   *mut AnimationHeader as *mut libc::c_void)
            as f32_0;
    Animation_Change(&mut (*this).skelAnime, &mut gBarinadeZapperIdleAnim,
                     1.0f32, lastFrame - 1.0f32, lastFrame,
                     ANIMMODE_LOOP_INTERP as libc::c_int as u8_0, -6.0f32);
    (*this).actor.flags &=
        !((1 as libc::c_int) << 0 as libc::c_int) as libc::c_uint;
    BossVa_SetupAction(this,
                       Some(BossVa_ZapperIntro as
                                unsafe extern "C" fn(_: *mut BossVa,
                                                     _: *mut GlobalContext)
                                    -> ()));
}
#[no_mangle]
pub unsafe extern "C" fn BossVa_ZapperIntro(mut this: *mut BossVa,
                                            mut globalCtx:
                                                *mut GlobalContext) {
    BossVa_AttachToBody(this);
    match sCsState as libc::c_int {
        10 | 11 | 12 => { SkelAnime_Update(&mut (*this).skelAnime); }
        13 => { BossVa_SetupZapperAttack(this, globalCtx); }
        _ => { }
    }
    Math_SmoothStepToS(&mut (*this).unk_1F2,
                       ((*this).actor.shape.rot.y as libc::c_int -
                            (*this).actor.shape.rot.x as libc::c_int) as s16,
                       1 as libc::c_int as s16, 0x2ee as libc::c_int as s16,
                       0 as libc::c_int as s16);
    Math_SmoothStepToS(&mut (*this).unk_1F0,
                       (*(*this).skelAnime.jointTable.offset(7 as libc::c_int
                                                                 as isize)).z,
                       1 as libc::c_int as s16, 0x2ee as libc::c_int as s16,
                       0 as libc::c_int as s16);
}
#[no_mangle]
pub unsafe extern "C" fn BossVa_SetupZapperAttack(mut this: *mut BossVa,
                                                  mut globalCtx:
                                                      *mut GlobalContext) {
    let mut lastFrame: f32_0 =
        Animation_GetLastFrame(&mut gBarinadeZapperIdleAnim as
                                   *mut AnimationHeader as *mut libc::c_void)
            as f32_0;
    Animation_Change(&mut (*this).skelAnime, &mut gBarinadeZapperIdleAnim,
                     1.0f32, lastFrame - 1.0f32, lastFrame,
                     ANIMMODE_LOOP_INTERP as libc::c_int as u8_0, -6.0f32);
    (*this).actor.flags &=
        !((1 as libc::c_int) << 0 as libc::c_int) as libc::c_uint;
    BossVa_SetupAction(this,
                       Some(BossVa_ZapperAttack as
                                unsafe extern "C" fn(_: *mut BossVa,
                                                     _: *mut GlobalContext)
                                    -> ()));
}
#[no_mangle]
pub unsafe extern "C" fn BossVa_ZapperAttack(mut this: *mut BossVa,
                                             mut globalCtx:
                                                 *mut GlobalContext) {
    let mut player: *mut Player =
        (*globalCtx).actorCtx.actorLists[ACTORCAT_PLAYER as libc::c_int as
                                             usize].head as *mut Player;
    let mut boomerang: *mut EnBoom = 0 as *mut EnBoom;
    let mut boomTarget: *mut Actor = 0 as *mut Actor;
    let mut yaw: s16 = 0;
    let mut sp98: s16 = 0;
    let mut sp96: s16 = 0;
    let mut sp94: s16 = 0;
    let mut tmp17: s16 = 0;
    let mut sp90: s16 = 0x1f4 as libc::c_int as s16;
    let mut sp8E: s16 = 0;
    let mut sp88: u32_0 = 0;
    let mut sp7C: Vec3f = Vec3f{x: 0., y: 0., z: 0.,};
    let mut pad3: s32 = 0;
    let mut sp74: f32_0 = 0.;
    let mut i: s32 = 0;
    let mut sp6E: s16 = 0;
    let mut sp6C: s16 = 0;
    let mut sp68: f32_0 = 0.;
    let mut sp64: f32_0 = 0.;
    let mut sp60: f32_0 = 0.;
    let mut sp5C: f32_0 = 0.;
    let mut sp5A: s16 = 0;
    let mut sp58: s16 = 0;
    let mut sp56: s16 = 0;
    let mut sp54: s16 = 0;
    let mut sp50: f32_0 = 0.;
    boomerang = BossVa_FindBoomerang(globalCtx);
    if boomerang.is_null() || (*boomerang).moveTo.is_null() ||
           (*boomerang).moveTo == &mut (*player).actor as *mut Actor {
        sp7C = (*player).actor.world.pos;
        sp7C.y += 10.0f32;
        sp8E = 0x3e80 as libc::c_int as s16
    } else {
        sp74 =
            (*gGameInfo).data[(1 as libc::c_int * 6 as libc::c_int *
                                   16 as libc::c_int + 30 as libc::c_int) as
                                  usize] as libc::c_int as libc::c_float *
                0.5f32;
        sp8E = 0x4650 as libc::c_int as s16;
        boomTarget = (*boomerang).moveTo;
        sp7C = (*boomerang).actor.world.pos;
        sp6C = (*boomerang).actor.world.rot.y;
        sp56 = (*boomerang).actor.world.rot.x;
        i = (*boomerang).returnTimer as s32;
        while i >= 3 as libc::c_int {
            sp6E = Math_Vec3f_Yaw(&mut sp7C, &mut (*boomTarget).focus.pos);
            sp5A = (sp6C as libc::c_int - sp6E as libc::c_int) as s16;
            sp58 = Math_Vec3f_Pitch(&mut sp7C, &mut (*boomTarget).focus.pos);
            sp54 = (sp56 as libc::c_int - sp58 as libc::c_int) as s16;
            sp50 =
                (200.0f32 -
                     Math_Vec3f_DistXYZ(&mut sp7C,
                                        &mut (*boomTarget).focus.pos)) *
                    0.005f32;
            if sp50 < 0.12f32 { sp50 = 0.12f32 }
            if (sp5A as libc::c_int) < 0 as libc::c_int {
                sp5A = -(sp5A as libc::c_int) as s16
            }
            if (sp54 as libc::c_int) < 0 as libc::c_int {
                sp54 = -(sp54 as libc::c_int) as s16
            }
            Math_ScaledStepToS(&mut sp6C, sp6E,
                               (sp5A as libc::c_int as libc::c_float * sp50)
                                   as s16);
            Math_ScaledStepToS(&mut sp56, sp58,
                               (sp54 as libc::c_int as libc::c_float * sp50)
                                   as s16);
            sp68 = -Math_SinS(sp56) * 12.0f32;
            sp5C = Math_CosS(sp56) * 12.0f32;
            sp64 = Math_SinS(sp6C) * sp5C;
            sp60 = Math_CosS(sp6C);
            sp7C.x += sp64 * sp74;
            sp7C.y += sp68 * sp74;
            sp7C.z += sp60 * sp5C * sp74;
            i -= 1
        }
        sp90 = 0x3e80 as libc::c_int as s16
    }
    SkelAnime_Update(&mut (*this).skelAnime);
    BossVa_AttachToBody(this);
    if sFightPhase as libc::c_int >= 15 as libc::c_int {
        BossVa_SetupZapperEnraged(this, globalCtx);
        return
    }
    if sBodyState as libc::c_int & 0x7f as libc::c_int != 0 {
        BossVa_SetupZapperDamaged(this, globalCtx);
        return
    }
    if (sFightPhase as libc::c_int) < 15 as libc::c_int &&
           (*((*this).actor.parent as *mut BossVa)).actor.speedXZ != 0.0f32 {
        BossVa_SetupZapperHold(this, globalCtx);
        return
    }
    sp98 = Math_Vec3f_Yaw(&mut sp7C, &mut (*this).armTip);
    tmp17 =
        (sp98 as libc::c_int - (*this).actor.shape.rot.y as libc::c_int) as
            s16;
    if (sp8E as libc::c_int >=
            (if tmp17 as libc::c_int >= 0 as libc::c_int {
                 tmp17 as libc::c_int
             } else { -(tmp17 as libc::c_int) }) ||
            (*this).burst as libc::c_int != 0) &&
           sBodyState as libc::c_int & 0x80 as libc::c_int == 0 &&
           (*player).stateFlags1 & 0x4000000 as libc::c_int as libc::c_uint ==
               0 {
        if (*this).burst == 0 {
            sp94 =
                (sp98 as libc::c_int -
                     (*this).actor.shape.rot.y as libc::c_int) as s16;
            if (if sp94 as libc::c_int >= 0 as libc::c_int {
                    sp94 as libc::c_int
                } else { -(sp94 as libc::c_int) }) > 0x1770 as libc::c_int {
                sp94 =
                    if sp94 as libc::c_int > 0 as libc::c_int {
                        0x1770 as libc::c_int
                    } else { -(0x1770 as libc::c_int) } as s16
            }
            tmp17 =
                Math_SmoothStepToS(&mut (*this).unk_1E6, sp94,
                                   1 as libc::c_int as s16,
                                   0x6d6 as libc::c_int as s16,
                                   0 as libc::c_int as s16);
            sp88 =
                if tmp17 as libc::c_int >= 0 as libc::c_int {
                    tmp17 as libc::c_int
                } else { -(tmp17 as libc::c_int) } as u32_0;
            sp94 = (sp98 as libc::c_int - sp94 as libc::c_int) as s16;
            if (if sp94 as libc::c_int >= 0 as libc::c_int {
                    sp94 as libc::c_int
                } else { -(sp94 as libc::c_int) }) > 0x1770 as libc::c_int {
                sp94 =
                    if sp94 as libc::c_int > 0 as libc::c_int {
                        0x1770 as libc::c_int
                    } else { -(0x1770 as libc::c_int) } as s16
            }
            tmp17 =
                Math_SmoothStepToS(&mut (*this).unk_1EC, sp94,
                                   1 as libc::c_int as s16,
                                   0x6d6 as libc::c_int as s16,
                                   0 as libc::c_int as s16);
            sp88 =
                (sp88 as
                     libc::c_uint).wrapping_add(if tmp17 as libc::c_int >=
                                                       0 as libc::c_int {
                                                    tmp17 as libc::c_int
                                                } else {
                                                    -(tmp17 as libc::c_int)
                                                } as libc::c_uint) as u32_0 as
                    u32_0;
            yaw = Math_Vec3f_Yaw(&mut (*this).zapHeadPos, &mut sp7C);
            tmp17 =
                Math_SmoothStepToS(&mut (*this).unk_1F2,
                                   (yaw as libc::c_int -
                                        0x4000 as libc::c_int) as s16,
                                   1 as libc::c_int as s16,
                                   0x9c4 as libc::c_int as s16,
                                   0 as libc::c_int as s16);
            sp88 =
                (sp88 as
                     libc::c_uint).wrapping_add(if tmp17 as libc::c_int >=
                                                       0 as libc::c_int {
                                                    tmp17 as libc::c_int
                                                } else {
                                                    -(tmp17 as libc::c_int)
                                                } as libc::c_uint) as u32_0 as
                    u32_0;
            sp96 =
                ((*this).actor.shape.rot.x as libc::c_int +
                     (*(*this).skelAnime.jointTable.offset(1 as libc::c_int as
                                                               isize)).z as
                         libc::c_int +
                     (*(*this).skelAnime.jointTable.offset(2 as libc::c_int as
                                                               isize)).z as
                         libc::c_int +
                     (*(*this).skelAnime.jointTable.offset(3 as libc::c_int as
                                                               isize)).z as
                         libc::c_int +
                     (*(*this).skelAnime.jointTable.offset(4 as libc::c_int as
                                                               isize)).z as
                         libc::c_int +
                     (*(*this).skelAnime.jointTable.offset(5 as libc::c_int as
                                                               isize)).z as
                         libc::c_int) as s16;
            yaw = Math_Vec3f_Pitch(&mut sp7C, &mut (*this).zapNeckPos);
            tmp17 =
                Math_SmoothStepToS(&mut (*this).unk_1EA,
                                   (yaw as libc::c_int - sp96 as libc::c_int)
                                       as s16, 1 as libc::c_int as s16,
                                   0xfa0 as libc::c_int as s16,
                                   0 as libc::c_int as s16);
            sp88 =
                (sp88 as
                     libc::c_uint).wrapping_add(if tmp17 as libc::c_int >=
                                                       0 as libc::c_int {
                                                    tmp17 as libc::c_int
                                                } else {
                                                    -(tmp17 as libc::c_int)
                                                } as libc::c_uint) as u32_0 as
                    u32_0;
            yaw = Math_Vec3f_Pitch(&mut (*this).zapHeadPos, &mut sp7C);
            tmp17 =
                Math_SmoothStepToS(&mut (*this).unk_1F0,
                                   -(yaw as libc::c_int) as s16,
                                   1 as libc::c_int as s16,
                                   0xfa0 as libc::c_int as s16,
                                   0 as libc::c_int as s16);
            sp88 =
                (sp88 as
                     libc::c_uint).wrapping_add(if tmp17 as libc::c_int >=
                                                       0 as libc::c_int {
                                                    tmp17 as libc::c_int
                                                } else {
                                                    -(tmp17 as libc::c_int)
                                                } as libc::c_uint) as u32_0 as
                    u32_0;
            (*this).skelAnime.playSpeed = 0.0f32;
            if Math_SmoothStepToF(&mut (*this).skelAnime.curFrame, 0.0f32,
                                  1.0f32, 2.0f32, 0.0f32) == 0.0f32 {
                if sp88 < sp90 as libc::c_uint {
                    (*this).timer2 = 0 as libc::c_int as s16;
                    (*this).burst = (*this).burst.wrapping_add(1);
                    (*this).unk_1D8 = sp7C
                }
                if Rand_ZeroOne() < 0.1f32 {
                    Audio_PlayActorSound2(&mut (*this).actor,
                                          (0x3943 as libc::c_int -
                                               0x800 as libc::c_int) as
                                              u16_0);
                }
            }
        }
    } else {
        if (*this).burst as libc::c_int != 0 ||
               ((*this).timer2 as libc::c_int) < 0 as libc::c_int {
            if (*this).colliderLightning.base.atFlags as libc::c_int &
                   (1 as libc::c_int) << 1 as libc::c_int != 0 {
                if (*this).timer2 as libc::c_int > 0 as libc::c_int {
                    Audio_PlayActorSound2(&mut (*this).actor,
                                          0x3946 as libc::c_int as u16_0);
                    BossVa_SetSparkEnv(globalCtx);
                    (*this).timer2 = -(1 as libc::c_int) as s16;
                    (*((*this).actor.parent as *mut BossVa)).onCeiling =
                        6 as libc::c_int as u8_0
                    // not used by body
                }
            } else if (*this).timer2 as libc::c_int > 0 as libc::c_int {
                (*this).timer2 = 0 as libc::c_int as s16
            }
            if ((*this).timer2 as libc::c_int) < 0 as libc::c_int &&
                   (*player).stateFlags1 &
                       0x4000000 as libc::c_int as libc::c_uint != 0 {
                BossVa_Spark(globalCtx, this, 1 as libc::c_int,
                             30 as libc::c_int as s16, 0.0f32, 0.0f32,
                             SPARK_LINK as libc::c_int as u8_0, 0.0f32,
                             1 as libc::c_int as u8_0);
            }
        }
        Math_SmoothStepToS(&mut (*this).unk_1E6, 0 as libc::c_int as s16,
                           1 as libc::c_int as s16,
                           0x6d6 as libc::c_int as s16,
                           0 as libc::c_int as s16);
        Math_SmoothStepToS(&mut (*this).unk_1EC, 0 as libc::c_int as s16,
                           1 as libc::c_int as s16,
                           0x6d6 as libc::c_int as s16,
                           0 as libc::c_int as s16);
        Math_SmoothStepToS(&mut (*this).unk_1EA, 0 as libc::c_int as s16,
                           1 as libc::c_int as s16,
                           0x6d6 as libc::c_int as s16,
                           0 as libc::c_int as s16);
        Math_SmoothStepToS(&mut (*this).unk_1F2,
                           ((*this).actor.shape.rot.y as libc::c_int -
                                (*this).actor.shape.rot.x as libc::c_int) as
                               s16, 1 as libc::c_int as s16,
                           0x6d6 as libc::c_int as s16,
                           0 as libc::c_int as s16);
        Math_SmoothStepToS(&mut (*this).unk_1F0,
                           (*(*this).skelAnime.jointTable.offset(7 as
                                                                     libc::c_int
                                                                     as
                                                                     isize)).z,
                           1 as libc::c_int as s16,
                           0x6d6 as libc::c_int as s16,
                           0 as libc::c_int as s16);
        Math_SmoothStepToF(&mut (*this).skelAnime.playSpeed, 1.0f32, 1.0f32,
                           0.05f32, 0.0f32);
        (*this).burst = 0 as libc::c_int as u8_0
    }
    if (*this).burst as libc::c_int != 0 &&
           (*this).burst as libc::c_int != 2 as libc::c_int {
        // burst can never be 2
        if (*this).timer2 as libc::c_int >= 32 as libc::c_int {
            if (*this).timer2 as libc::c_int == 32 as libc::c_int {
                Audio_PlayActorSound2(&mut (*this).actor,
                                      0x3942 as libc::c_int as u16_0);
            }
            BossVa_Spark(globalCtx, this, 2 as libc::c_int,
                         110 as libc::c_int as s16, 15.0f32, 15.0f32,
                         SPARK_BLAST as libc::c_int as u8_0, 5.0f32,
                         1 as libc::c_int as u8_0);
            BossVa_Spark(globalCtx, this, 2 as libc::c_int,
                         110 as libc::c_int as s16, 15.0f32, 15.0f32,
                         SPARK_BLAST as libc::c_int as u8_0, 6.0f32,
                         1 as libc::c_int as u8_0);
            BossVa_Spark(globalCtx, this, 2 as libc::c_int,
                         110 as libc::c_int as s16, 15.0f32, 15.0f32,
                         SPARK_BLAST as libc::c_int as u8_0, 7.0f32,
                         1 as libc::c_int as u8_0);
            CollisionCheck_SetAT(globalCtx, &mut (*globalCtx).colChkCtx,
                                 &mut (*this).colliderLightning.base);
            CollisionCheck_SetAC(globalCtx, &mut (*globalCtx).colChkCtx,
                                 &mut (*this).colliderLightning.base);
        } else {
            BossVa_Spark(globalCtx, this, 2 as libc::c_int,
                         50 as libc::c_int as s16, 15.0f32, 0.0f32,
                         SPARK_BODY as libc::c_int as u8_0,
                         (((*this).timer2 as libc::c_int >> 3 as libc::c_int)
                              + 1 as libc::c_int) as f32_0,
                         1 as libc::c_int as u8_0);
            if (*this).timer2 as libc::c_int == 30 as libc::c_int {
                BossVa_SetSparkEnv(globalCtx);
            }
            if (*this).timer2 as libc::c_int == 20 as libc::c_int {
                let mut sp44: Vec3f = (*this).zapHeadPos;
                BossVa_SpawnZapperCharge(globalCtx, sVaEffects.as_mut_ptr(),
                                         this, &mut sp44,
                                         &mut (*this).headRot,
                                         100 as libc::c_int as s16,
                                         0 as libc::c_int as u8_0);
            }
        }
        (*this).timer2 += 1;
        if (*this).timer2 as libc::c_int >= 40 as libc::c_int {
            (*this).burst = 0 as libc::c_int as u8_0
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn BossVa_SetupZapperDamaged(mut this: *mut BossVa,
                                                   mut globalCtx:
                                                       *mut GlobalContext) {
    if Rand_ZeroOne() > 0.5f32 {
        Animation_Change(&mut (*this).skelAnime,
                         &mut gBarinadeZapperDamage1Anim, 0.5f32, 0.0f32,
                         Animation_GetLastFrame(&mut gBarinadeZapperDamage1Anim
                                                    as *mut AnimationHeader as
                                                    *mut libc::c_void) as
                             f32_0,
                         ANIMMODE_ONCE_INTERP as libc::c_int as u8_0, 4.0f32);
    } else {
        Animation_Change(&mut (*this).skelAnime,
                         &mut gBarinadeZapperDamage2Anim, 0.5f32, 0.0f32,
                         Animation_GetLastFrame(&mut gBarinadeZapperDamage2Anim
                                                    as *mut AnimationHeader as
                                                    *mut libc::c_void) as
                             f32_0,
                         ANIMMODE_ONCE_INTERP as libc::c_int as u8_0, 4.0f32);
    }
    Actor_SetColorFilter(&mut (*this).actor, 0 as libc::c_int as s16,
                         255 as libc::c_int as s16, 0 as libc::c_int as s16,
                         12 as libc::c_int as s16);
    (*this).burst = 0 as libc::c_int as u8_0;
    BossVa_SetupAction(this,
                       Some(BossVa_ZapperDamaged as
                                unsafe extern "C" fn(_: *mut BossVa,
                                                     _: *mut GlobalContext)
                                    -> ()));
}
#[no_mangle]
pub unsafe extern "C" fn BossVa_ZapperDamaged(mut this: *mut BossVa,
                                              mut globalCtx:
                                                  *mut GlobalContext) {
    BossVa_AttachToBody(this);
    Math_SmoothStepToS(&mut (*this).unk_1E6, 0 as libc::c_int as s16,
                       1 as libc::c_int as s16, 0xfa0 as libc::c_int as s16,
                       0 as libc::c_int as s16);
    Math_SmoothStepToS(&mut (*this).unk_1E4, 0 as libc::c_int as s16,
                       1 as libc::c_int as s16, 0xfa0 as libc::c_int as s16,
                       0 as libc::c_int as s16);
    Math_SmoothStepToS(&mut (*this).unk_1EC, 0 as libc::c_int as s16,
                       1 as libc::c_int as s16, 0xfa0 as libc::c_int as s16,
                       0 as libc::c_int as s16);
    Math_SmoothStepToS(&mut (*this).unk_1EA, 0 as libc::c_int as s16,
                       1 as libc::c_int as s16, 0xfa0 as libc::c_int as s16,
                       0 as libc::c_int as s16);
    Math_SmoothStepToS(&mut (*this).unk_1F2,
                       ((*this).actor.shape.rot.y as libc::c_int -
                            (*this).actor.shape.rot.x as libc::c_int) as s16,
                       1 as libc::c_int as s16, 0x2ee as libc::c_int as s16,
                       0 as libc::c_int as s16);
    Math_SmoothStepToS(&mut (*this).unk_1F0,
                       (*(*this).skelAnime.jointTable.offset(7 as libc::c_int
                                                                 as isize)).z,
                       1 as libc::c_int as s16, 0x2ee as libc::c_int as s16,
                       0 as libc::c_int as s16);
    if SkelAnime_Update(&mut (*this).skelAnime) != 0 {
        if sFightPhase as libc::c_int >= 15 as libc::c_int {
            BossVa_SetupZapperEnraged(this, globalCtx);
        } else { BossVa_SetupZapperAttack(this, globalCtx); }
    };
}
#[no_mangle]
pub unsafe extern "C" fn BossVa_SetupZapperDeath(mut this: *mut BossVa,
                                                 mut globalCtx:
                                                     *mut GlobalContext) {
    let mut lastFrame: f32_0 =
        Animation_GetLastFrame(&mut gBarinadeZapperIdleAnim as
                                   *mut AnimationHeader as *mut libc::c_void)
            as f32_0;
    Animation_Change(&mut (*this).skelAnime, &mut gBarinadeZapperIdleAnim,
                     Rand_ZeroOne() + 0.25f32, Rand_ZeroOne() * 3.0f32,
                     lastFrame, ANIMMODE_LOOP_INTERP as libc::c_int as u8_0,
                     -6.0f32);
    (*this).burst = 0 as libc::c_int as u8_0;
    (*this).timer2 =
        ((*this).actor.params as libc::c_int * -(6 as libc::c_int) +
             18 as libc::c_int) as s16;
    (*this).unk_1B0 = 0 as libc::c_int as s16;
    BossVa_SetupAction(this,
                       Some(BossVa_ZapperDeath as
                                unsafe extern "C" fn(_: *mut BossVa,
                                                     _: *mut GlobalContext)
                                    -> ()));
}
#[no_mangle]
pub unsafe extern "C" fn BossVa_ZapperDeath(mut this: *mut BossVa,
                                            mut globalCtx:
                                                *mut GlobalContext) {
    let mut sp3C: f32_0 = 55.0f32;
    let mut tmpf1: f32_0 = 0.;
    let mut tmpf2: f32_0 = 0.;
    BossVa_AttachToBody(this);
    if (*globalCtx).gameplayFrames.wrapping_rem(32 as libc::c_int as
                                                    libc::c_uint) ==
           0 as libc::c_int as libc::c_uint &&
           sCsState as libc::c_int <= DEATH_BODY_TUMORS as libc::c_int {
        (*this).unk_1E8 =
            Rand_CenteredFloat(0x4000 as libc::c_int as f32_0) as s16;
        (*this).unk_1EE =
            Rand_CenteredFloat(0x4000 as libc::c_int as f32_0) as s16;
        (*this).unk_1F4 =
            (Rand_CenteredFloat(0x4000 as libc::c_int as f32_0) as s16 as
                 libc::c_int + (*this).actor.shape.rot.y as libc::c_int -
                 (*this).actor.shape.rot.x as libc::c_int) as s16
    } else {
        Math_SmoothStepToF(&mut (*this).skelAnime.playSpeed, 0.0f32, 1.0f32,
                           0.025f32, 0.0f32);
    }
    SkelAnime_Update(&mut (*this).skelAnime);
    Math_SmoothStepToS(&mut (*this).unk_1E6, (*this).unk_1E8,
                       1 as libc::c_int as s16,
                       (Rand_CenteredFloat(500.0f32) as s16 as libc::c_int +
                            0x1f4 as libc::c_int) as s16,
                       0 as libc::c_int as s16);
    Math_SmoothStepToS(&mut (*this).unk_1E4, 0 as libc::c_int as s16,
                       1 as libc::c_int as s16, 0x1f4 as libc::c_int as s16,
                       0 as libc::c_int as s16);
    Math_SmoothStepToS(&mut (*this).unk_1EC, (*this).unk_1EE,
                       1 as libc::c_int as s16,
                       (Rand_CenteredFloat(500.0f32) as s16 as libc::c_int +
                            0x1f4 as libc::c_int) as s16,
                       0 as libc::c_int as s16);
    Math_SmoothStepToS(&mut (*this).unk_1EA, 0 as libc::c_int as s16,
                       1 as libc::c_int as s16, 0x1f4 as libc::c_int as s16,
                       0 as libc::c_int as s16);
    Math_SmoothStepToS(&mut (*this).unk_1F2, (*this).unk_1F4,
                       1 as libc::c_int as s16,
                       (Rand_CenteredFloat(500.0f32) as s16 as libc::c_int +
                            0x1f4 as libc::c_int) as s16,
                       0 as libc::c_int as s16);
    let mut current_block_54: u64;
    match sCsState as libc::c_int {
        17 => { sp3C = -55.0f32; current_block_54 = 15360263109566822851; }
        16 | 18 => { current_block_54 = 15360263109566822851; }
        _ => { current_block_54 = 13321564401369230990; }
    }
    match current_block_54 {
        15360263109566822851 => {
            if (*this).burst == 0 {
                if (*this).actor.params as libc::c_int ==
                       BOSSVA_ZAPPER_1 as libc::c_int &&
                       ((*this).timer2 as libc::c_int) < 16 as libc::c_int ||
                       (*this).actor.params as libc::c_int ==
                           BOSSVA_ZAPPER_2 as libc::c_int &&
                           ((*this).timer2 as libc::c_int) < 24 as libc::c_int
                       ||
                       (*this).actor.params as libc::c_int ==
                           BOSSVA_ZAPPER_3 as libc::c_int {
                    if (*this).timer2 as libc::c_int % 2 as libc::c_int ==
                           0 as libc::c_int &&
                           (*this).timer2 as libc::c_int >= 0 as libc::c_int {
                        if ((*this).timer2 as libc::c_int) < 8 as libc::c_int
                           {
                            BossVa_Tumor(globalCtx, this, 1 as libc::c_int,
                                         (Rand_CenteredFloat(5.0f32) as s16 as
                                              libc::c_int +
                                              0xd as libc::c_int) as s16,
                                         0.0f32, 0.0f32,
                                         TUMOR_ARM as libc::c_int as u8_0,
                                         0.6f32, 1 as libc::c_int as u8_0);
                        } else {
                            BossVa_Tumor(globalCtx, this, 1 as libc::c_int,
                                         (Rand_CenteredFloat(5.0f32) as s16 as
                                              libc::c_int + 6 as libc::c_int)
                                             as s16, 0.0f32, 7.0f32,
                                         TUMOR_ARM as libc::c_int as u8_0,
                                         (((*this).timer2 as libc::c_int >>
                                               3 as libc::c_int) +
                                              1 as libc::c_int) as f32_0,
                                         1 as libc::c_int as u8_0);
                        }
                        BossVa_Spark(globalCtx, this, 2 as libc::c_int,
                                     50 as libc::c_int as s16, 15.0f32,
                                     0.0f32,
                                     SPARK_BODY as libc::c_int as u8_0,
                                     (((*this).timer2 as libc::c_int >>
                                           3 as libc::c_int) +
                                          1 as libc::c_int) as f32_0,
                                     1 as libc::c_int as u8_0);
                    }
                    (*this).timer2 += 1;
                    if (*this).timer2 as libc::c_int >= 32 as libc::c_int {
                        (*this).burst = (*this).burst.wrapping_add(1);
                        (*this).isDead = 1 as libc::c_int as u8_0;
                        BossVa_SetDeathEnv(globalCtx);
                        Audio_PlayActorSound2(&mut (*this).actor,
                                              0x3945 as libc::c_int as u16_0);
                    }
                } else {
                    (*this).burst = (*this).burst.wrapping_add(1);
                    (*this).isDead = 1 as libc::c_int as u8_0;
                    (*this).timer2 = 32 as libc::c_int as s16;
                    sCsState += 1
                }
                if (*this).actor.params as libc::c_int -
                       BOSSVA_ZAPPER_1 as libc::c_int +
                       DEATH_ZAPPER_1 as libc::c_int ==
                       sCsState as libc::c_int {
                    sCameraAt.x = (*this).zapNeckPos.x;
                    sCameraAt.y = (*this).zapNeckPos.y;
                    sCameraEye.y = sCameraAt.y;
                    sCameraAt.z = (*this).zapNeckPos.z;
                    sCameraEye.x =
                        Math_CosS(-((*this).actor.shape.rot.y as libc::c_int +
                                        (*this).unk_1B0 as libc::c_int) as
                                      s16) * sp3C + (*this).zapNeckPos.x;
                    sCameraEye.z =
                        Math_SinS(-((*this).actor.shape.rot.y as libc::c_int +
                                        (*this).unk_1B0 as libc::c_int) as
                                      s16) * sp3C + (*this).zapNeckPos.z;
                    (*this).unk_1B0 =
                        ((*this).unk_1B0 as libc::c_int +
                             0x15e as libc::c_int) as s16
                }
            } else {
                (*this).timer2 -= 1;
                if (*this).timer2 as libc::c_int == 0 as libc::c_int {
                    if (*this).actor.params as libc::c_int ==
                           BOSSVA_ZAPPER_3 as libc::c_int {
                        sCsState += 1
                    }
                    Actor_Kill(&mut (*this).actor);
                }
            }
        }
        _ => { }
    };
}
#[no_mangle]
pub unsafe extern "C" fn BossVa_SetupZapperEnraged(mut this: *mut BossVa,
                                                   mut globalCtx:
                                                       *mut GlobalContext) {
    let mut lastFrame: f32_0 =
        Animation_GetLastFrame(&mut gBarinadeZapperIdleAnim as
                                   *mut AnimationHeader as *mut libc::c_void)
            as f32_0;
    Animation_Change(&mut (*this).skelAnime, &mut gBarinadeZapperIdleAnim,
                     1.0f32, lastFrame - 1.0f32, lastFrame,
                     ANIMMODE_LOOP_INTERP as libc::c_int as u8_0, -6.0f32);
    (*this).burst = 0 as libc::c_int as u8_0;
    BossVa_SetupAction(this,
                       Some(BossVa_ZapperEnraged as
                                unsafe extern "C" fn(_: *mut BossVa,
                                                     _: *mut GlobalContext)
                                    -> ()));
}
#[no_mangle]
pub unsafe extern "C" fn BossVa_ZapperEnraged(mut this: *mut BossVa,
                                              mut globalCtx:
                                                  *mut GlobalContext) {
    let mut player: *mut Player =
        (*globalCtx).actorCtx.actorLists[ACTORCAT_PLAYER as libc::c_int as
                                             usize].head as *mut Player;
    let mut pad: s32 = 0;
    let mut tmp16: s16 = 0;
    let mut sp6C: s16 = 0;
    let mut sp6A: s16 = 0;
    let mut sp68: s16 = 0;
    let mut yaw: s16 = 0;
    let mut sp60: u32_0 = 0;
    let mut sp54: Vec3f = (*player).actor.world.pos;
    sp54.y += 10.0f32;
    SkelAnime_Update(&mut (*this).skelAnime);
    BossVa_AttachToBody(this);
    if sFightPhase as libc::c_int >= 18 as libc::c_int {
        BossVa_SetupZapperDeath(this, globalCtx);
        return
    }
    if sBodyState as libc::c_int & 0x7e as libc::c_int != 0 {
        BossVa_SetupZapperDamaged(this, globalCtx);
        return
    }
    sp54.y = (sp54.y as libc::c_double + 25.0f64) as f32_0;
    sp6C = Math_Vec3f_Yaw(&mut sp54, &mut (*this).armTip);
    tmp16 =
        (sp6C as libc::c_int - (*this).actor.shape.rot.y as libc::c_int) as
            s16;
    if ((if tmp16 as libc::c_int >= 0 as libc::c_int {
             tmp16 as libc::c_int
         } else { -(tmp16 as libc::c_int) }) <= 0x4650 as libc::c_int ||
            (*this).burst as libc::c_int != 0) &&
           sBodyState as libc::c_int & 0x80 as libc::c_int == 0 &&
           (*player).stateFlags1 & 0x4000000 as libc::c_int as libc::c_uint ==
               0 {
        if (*this).burst == 0 {
            sp68 =
                (sp6C as libc::c_int -
                     (*this).actor.shape.rot.y as libc::c_int) as s16;
            if (if sp68 as libc::c_int >= 0 as libc::c_int {
                    sp68 as libc::c_int
                } else { -(sp68 as libc::c_int) }) > 0x1770 as libc::c_int {
                sp68 =
                    if sp68 as libc::c_int > 0 as libc::c_int {
                        0x1770 as libc::c_int
                    } else { -(0x1770 as libc::c_int) } as s16
            }
            tmp16 =
                Math_SmoothStepToS(&mut (*this).unk_1E6, sp68,
                                   1 as libc::c_int as s16,
                                   0xdac as libc::c_int as s16,
                                   0 as libc::c_int as s16);
            sp60 =
                if tmp16 as libc::c_int >= 0 as libc::c_int {
                    tmp16 as libc::c_int
                } else { -(tmp16 as libc::c_int) } as u32_0;
            sp68 = (sp6C as libc::c_int - sp68 as libc::c_int) as s16;
            if (if sp68 as libc::c_int >= 0 as libc::c_int {
                    sp68 as libc::c_int
                } else { -(sp68 as libc::c_int) }) > 0x1770 as libc::c_int {
                sp68 =
                    if sp68 as libc::c_int > 0 as libc::c_int {
                        0x1770 as libc::c_int
                    } else { -(0x1770 as libc::c_int) } as s16
            }
            tmp16 =
                Math_SmoothStepToS(&mut (*this).unk_1EC, sp68,
                                   1 as libc::c_int as s16,
                                   0xdac as libc::c_int as s16,
                                   0 as libc::c_int as s16);
            sp60 =
                (sp60 as
                     libc::c_uint).wrapping_add(if tmp16 as libc::c_int >=
                                                       0 as libc::c_int {
                                                    tmp16 as libc::c_int
                                                } else {
                                                    -(tmp16 as libc::c_int)
                                                } as libc::c_uint) as u32_0 as
                    u32_0;
            yaw = Math_Vec3f_Yaw(&mut (*this).zapHeadPos, &mut sp54);
            tmp16 =
                Math_SmoothStepToS(&mut (*this).unk_1F2,
                                   (yaw as libc::c_int -
                                        0x4000 as libc::c_int) as s16,
                                   1 as libc::c_int as s16,
                                   0xea6 as libc::c_int as s16,
                                   0 as libc::c_int as s16);
            sp60 =
                (sp60 as
                     libc::c_uint).wrapping_add(if tmp16 as libc::c_int >=
                                                       0 as libc::c_int {
                                                    tmp16 as libc::c_int
                                                } else {
                                                    -(tmp16 as libc::c_int)
                                                } as libc::c_uint) as u32_0 as
                    u32_0;
            sp6A =
                ((*this).actor.shape.rot.x as libc::c_int +
                     (*(*this).skelAnime.jointTable.offset(1 as libc::c_int as
                                                               isize)).x as
                         libc::c_int +
                     (*(*this).skelAnime.jointTable.offset(2 as libc::c_int as
                                                               isize)).x as
                         libc::c_int +
                     (*(*this).skelAnime.jointTable.offset(3 as libc::c_int as
                                                               isize)).x as
                         libc::c_int +
                     (*(*this).skelAnime.jointTable.offset(4 as libc::c_int as
                                                               isize)).x as
                         libc::c_int +
                     (*(*this).skelAnime.jointTable.offset(5 as libc::c_int as
                                                               isize)).x as
                         libc::c_int) as s16;
            yaw = Math_Vec3f_Pitch(&mut sp54, &mut (*this).zapNeckPos);
            tmp16 =
                Math_SmoothStepToS(&mut (*this).unk_1EA,
                                   (yaw as libc::c_int - sp6A as libc::c_int)
                                       as s16, 1 as libc::c_int as s16,
                                   0x1b58 as libc::c_int as s16,
                                   0 as libc::c_int as s16);
            sp60 =
                (sp60 as
                     libc::c_uint).wrapping_add(if tmp16 as libc::c_int >=
                                                       0 as libc::c_int {
                                                    tmp16 as libc::c_int
                                                } else {
                                                    -(tmp16 as libc::c_int)
                                                } as libc::c_uint) as u32_0 as
                    u32_0;
            yaw = Math_Vec3f_Pitch(&mut (*this).zapHeadPos, &mut sp54);
            tmp16 =
                Math_SmoothStepToS(&mut (*this).unk_1F0,
                                   -(yaw as libc::c_int) as s16,
                                   1 as libc::c_int as s16,
                                   0x1b58 as libc::c_int as s16,
                                   0 as libc::c_int as s16);
            sp60 =
                (sp60 as
                     libc::c_uint).wrapping_add(if tmp16 as libc::c_int >=
                                                       0 as libc::c_int {
                                                    tmp16 as libc::c_int
                                                } else {
                                                    -(tmp16 as libc::c_int)
                                                } as libc::c_uint) as u32_0 as
                    u32_0;
            (*this).skelAnime.playSpeed = 0.0f32;
            if Math_SmoothStepToF(&mut (*this).skelAnime.curFrame, 0.0f32,
                                  1.0f32, 3.0f32, 0.0f32) == 0.0f32 &&
                   sp60 < 0x258 as libc::c_int as libc::c_uint {
                (*this).timer2 = 0 as libc::c_int as s16;
                (*this).burst = (*this).burst.wrapping_add(1);
                (*this).unk_1D8 = sp54;
                if Rand_ZeroOne() < 0.1f32 {
                    Audio_PlayActorSound2(&mut (*this).actor,
                                          (0x3943 as libc::c_int -
                                               0x800 as libc::c_int) as
                                              u16_0);
                }
            }
        }
    } else {
        if (*this).burst as libc::c_int != 0 ||
               ((*this).timer2 as libc::c_int) < 0 as libc::c_int {
            if (*this).colliderLightning.base.atFlags as libc::c_int &
                   (1 as libc::c_int) << 1 as libc::c_int != 0 {
                if (*this).timer2 as libc::c_int > 0 as libc::c_int {
                    Audio_PlayActorSound2(&mut (*this).actor,
                                          0x3946 as libc::c_int as u16_0);
                    BossVa_SetSparkEnv(globalCtx);
                    (*this).timer2 = -(1 as libc::c_int) as s16;
                    (*((*this).actor.parent as *mut BossVa)).onCeiling =
                        6 as libc::c_int as u8_0
                    // not used by body
                }
            } else if (*this).timer2 as libc::c_int > 0 as libc::c_int {
                (*this).timer2 = 0 as libc::c_int as s16
            }
            if ((*this).timer2 as libc::c_int) < 0 as libc::c_int &&
                   (*player).stateFlags1 &
                       0x4000000 as libc::c_int as libc::c_uint != 0 {
                BossVa_Spark(globalCtx, this, 1 as libc::c_int,
                             30 as libc::c_int as s16, 0.0f32,
                             0 as libc::c_int as f32_0,
                             SPARK_LINK as libc::c_int as u8_0, 0.0f32,
                             1 as libc::c_int as u8_0);
            }
        }
        Math_SmoothStepToS(&mut (*this).unk_1E6, 0 as libc::c_int as s16,
                           1 as libc::c_int as s16,
                           0xea6 as libc::c_int as s16,
                           0 as libc::c_int as s16);
        Math_SmoothStepToS(&mut (*this).unk_1EC, 0 as libc::c_int as s16,
                           1 as libc::c_int as s16,
                           0xea6 as libc::c_int as s16,
                           0 as libc::c_int as s16);
        Math_SmoothStepToS(&mut (*this).unk_1EA, 0 as libc::c_int as s16,
                           1 as libc::c_int as s16,
                           0xea6 as libc::c_int as s16,
                           0 as libc::c_int as s16);
        Math_SmoothStepToS(&mut (*this).unk_1F2,
                           ((*this).actor.shape.rot.y as libc::c_int -
                                (*this).actor.shape.rot.x as libc::c_int) as
                               s16, 1 as libc::c_int as s16,
                           0xea6 as libc::c_int as s16,
                           0 as libc::c_int as s16);
        Math_SmoothStepToS(&mut (*this).unk_1F0,
                           (*(*this).skelAnime.jointTable.offset(7 as
                                                                     libc::c_int
                                                                     as
                                                                     isize)).z,
                           1 as libc::c_int as s16,
                           0xea6 as libc::c_int as s16,
                           0 as libc::c_int as s16);
        Math_SmoothStepToF(&mut (*this).skelAnime.playSpeed, 1.0f32, 1.0f32,
                           0.05f32, 0.0f32);
        (*this).burst = 0 as libc::c_int as u8_0
    }
    if (*this).burst as libc::c_int != 0 &&
           (*this).burst as libc::c_int != 2 as libc::c_int {
        // burst can never be 2
        if (*this).timer2 as libc::c_int >= 16 as libc::c_int {
            if (*this).timer2 as libc::c_int == 18 as libc::c_int {
                Audio_PlayActorSound2(&mut (*this).actor,
                                      0x3942 as libc::c_int as u16_0);
            }
            BossVa_Spark(globalCtx, this, 2 as libc::c_int,
                         110 as libc::c_int as s16, 15.0f32, 15.0f32,
                         SPARK_BLAST as libc::c_int as u8_0, 5.0f32,
                         1 as libc::c_int as u8_0);
            BossVa_Spark(globalCtx, this, 2 as libc::c_int,
                         110 as libc::c_int as s16, 15.0f32, 15.0f32,
                         SPARK_BLAST as libc::c_int as u8_0, 6.0f32,
                         1 as libc::c_int as u8_0);
            BossVa_Spark(globalCtx, this, 2 as libc::c_int,
                         110 as libc::c_int as s16, 15.0f32, 15.0f32,
                         SPARK_BLAST as libc::c_int as u8_0, 7.0f32,
                         1 as libc::c_int as u8_0);
            CollisionCheck_SetAT(globalCtx, &mut (*globalCtx).colChkCtx,
                                 &mut (*this).colliderLightning.base);
            CollisionCheck_SetAC(globalCtx, &mut (*globalCtx).colChkCtx,
                                 &mut (*this).colliderLightning.base);
        } else {
            BossVa_Spark(globalCtx, this, 2 as libc::c_int,
                         50 as libc::c_int as s16, 15.0f32, 0.0f32,
                         SPARK_BODY as libc::c_int as u8_0,
                         (((*this).timer2 as libc::c_int >> 1 as libc::c_int)
                              + 1 as libc::c_int) as f32_0,
                         1 as libc::c_int as u8_0);
            if (*this).timer2 as libc::c_int == 14 as libc::c_int {
                BossVa_SetSparkEnv(globalCtx);
            }
            if (*this).timer2 as libc::c_int == 4 as libc::c_int {
                let mut sp48: Vec3f = (*this).zapHeadPos;
                BossVa_SpawnZapperCharge(globalCtx, sVaEffects.as_mut_ptr(),
                                         this, &mut sp48,
                                         &mut (*this).headRot,
                                         100 as libc::c_int as s16,
                                         0 as libc::c_int as u8_0);
            }
        }
        (*this).timer2 += 1;
        if (*this).timer2 as libc::c_int >= 24 as libc::c_int {
            (*this).burst = 0 as libc::c_int as u8_0
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn BossVa_SetupZapperHold(mut this: *mut BossVa,
                                                mut globalCtx:
                                                    *mut GlobalContext) {
    Animation_Change(&mut (*this).skelAnime, &mut gBarinadeZapperDamage2Anim,
                     0.0f32, 0.0f32,
                     Animation_GetLastFrame(&mut gBarinadeZapperDamage2Anim as
                                                *mut AnimationHeader as
                                                *mut libc::c_void) as f32_0,
                     ANIMMODE_ONCE_INTERP as libc::c_int as u8_0, -6.0f32);
    (*this).burst = 0 as libc::c_int as u8_0;
    BossVa_SetupAction(this,
                       Some(BossVa_ZapperHold as
                                unsafe extern "C" fn(_: *mut BossVa,
                                                     _: *mut GlobalContext)
                                    -> ()));
}
#[no_mangle]
pub unsafe extern "C" fn BossVa_ZapperHold(mut this: *mut BossVa,
                                           mut globalCtx:
                                               *mut GlobalContext) {
    SkelAnime_Update(&mut (*this).skelAnime);
    BossVa_AttachToBody(this);
    Math_SmoothStepToS(&mut (*this).unk_1E6, 0 as libc::c_int as s16,
                       1 as libc::c_int as s16, 0x1770 as libc::c_int as s16,
                       0 as libc::c_int as s16);
    Math_SmoothStepToS(&mut (*this).unk_1E4, 0 as libc::c_int as s16,
                       1 as libc::c_int as s16, 0x1770 as libc::c_int as s16,
                       0 as libc::c_int as s16);
    Math_SmoothStepToS(&mut (*this).unk_1EC, 0 as libc::c_int as s16,
                       1 as libc::c_int as s16, 0x1770 as libc::c_int as s16,
                       0 as libc::c_int as s16);
    Math_SmoothStepToS(&mut (*this).unk_1EA, 0 as libc::c_int as s16,
                       1 as libc::c_int as s16, 0x1770 as libc::c_int as s16,
                       0 as libc::c_int as s16);
    Math_SmoothStepToS(&mut (*this).unk_1F2,
                       ((*this).actor.shape.rot.y as libc::c_int -
                            0x4000 as libc::c_int) as s16,
                       1 as libc::c_int as s16, 0x2710 as libc::c_int as s16,
                       0 as libc::c_int as s16);
    Math_SmoothStepToS(&mut (*this).unk_1F0,
                       ((*(*this).skelAnime.jointTable.offset(7 as libc::c_int
                                                                  as isize)).z
                            as libc::c_int - 0x1388 as libc::c_int) as s16,
                       1 as libc::c_int as s16, 0x1770 as libc::c_int as s16,
                       0 as libc::c_int as s16);
    if (*((*this).actor.parent as *mut BossVa)).actor.speedXZ == 0.0f32 {
        BossVa_SetupZapperAttack(this, globalCtx);
    };
}
#[no_mangle]
pub unsafe extern "C" fn BossVa_SetupBariIntro(mut this: *mut BossVa,
                                               mut globalCtx:
                                                   *mut GlobalContext) {
    Animation_Change(&mut (*this).skelAnime, &mut gBarinadeBariAnim, 1.0f32,
                     0.0f32,
                     Animation_GetLastFrame(&mut gBarinadeBariAnim as
                                                *mut AnimationHeader as
                                                *mut libc::c_void) as f32_0,
                     ANIMMODE_LOOP as libc::c_int as u8_0, 0.0f32);
    (*this).unk_1A0 = 60.0f32;
    (*this).unk_1A4 = Rand_ZeroOne() * 360.0f32;
    (*this).timer2 = 64 as libc::c_int as s16;
    (*this).unk_1F0 = 120 as libc::c_int as s16;
    (*this).unk_1A8 = 0.0f32;
    (*this).actor.world.pos.x =
        sInitPosOffsets[((*this).actor.params as libc::c_int +
                             10 as libc::c_int) as usize].x +
            (*this).actor.home.pos.x;
    (*this).actor.world.pos.y =
        sInitPosOffsets[((*this).actor.params as libc::c_int +
                             10 as libc::c_int) as usize].y +
            (*this).actor.home.pos.y;
    (*this).actor.world.pos.z =
        sInitPosOffsets[((*this).actor.params as libc::c_int +
                             10 as libc::c_int) as usize].z +
            (*this).actor.home.pos.z;
    (*this).timer = 45 as libc::c_int;
    (*this).actor.flags &=
        !((1 as libc::c_int) << 0 as libc::c_int) as libc::c_uint;
    BossVa_SetupAction(this,
                       Some(BossVa_BariIntro as
                                unsafe extern "C" fn(_: *mut BossVa,
                                                     _: *mut GlobalContext)
                                    -> ()));
}
#[no_mangle]
pub unsafe extern "C" fn BossVa_BariIntro(mut this: *mut BossVa,
                                          mut globalCtx: *mut GlobalContext) {
    let mut sp54: Vec3f = (*this).actor.home.pos;
    let mut sp50: f32_0 = 40.0f32;
    let mut sp4E: s16 = 0;
    let mut tmp: s16 = 0;
    if (*this).actor.home.pos.y >= 0.0f32 { sp54.y += 25.0f32 }
    (*this).unk_1A4 += Rand_ZeroOne() * 0.25f32;
    let mut current_block_68: u64;
    match sCsState as libc::c_int {
        2 => {
            if (*this).actor.params as libc::c_int ==
                   BOSSVA_BARI_UPPER_1 as libc::c_int {
                func_8002DF54(globalCtx, &mut (*this).actor,
                              1 as libc::c_int as u8_0);
                if Math_SmoothStepToF(&mut (*this).actor.world.pos.y, 60.0f32,
                                      0.3f32, 1.0f32, 0.15f32) == 0.0f32 {
                    (*this).timer -= 1;
                    if (*this).timer == 0 as libc::c_int { sCsState += 1 }
                }
            }
            (*this).actor.shape.rot.x = 0 as libc::c_int as s16;
            current_block_68 = 10512632378975961025;
        }
        3 | 4 | 5 | 6 => {
            if (*this).actor.params as libc::c_int !=
                   BOSSVA_BARI_UPPER_1 as libc::c_int {
                Math_SmoothStepToF(&mut (*this).actor.world.pos.y,
                                   sInitPosOffsets[((*this).actor.params as
                                                        libc::c_int +
                                                        10 as libc::c_int) as
                                                       usize].y +
                                       (*this).actor.home.pos.y, 0.3f32,
                                   1.0f32, 0.15f32);
                (*this).actor.world.pos.x +=
                    Math_SinF((*this).unk_1A4 * 0.25f32) * 0.5f32
            } else {
                Math_SmoothStepToF(&mut (*this).actor.world.pos.y, 60.0f32,
                                   0.3f32, 1.0f32, 0.15f32);
            }
            (*this).actor.world.pos.y +=
                Math_SinF((*this).unk_1A4) *
                    (2.0f32 - Math_SinF((*this).unk_1A4));
            current_block_68 = 10512632378975961025;
        }
        8 | 9 => {
            if (*this).timer2 as libc::c_int > 15 as libc::c_int &&
                   (*this).timer < 0 as libc::c_int {
                Math_SmoothStepToF(&mut (*this).actor.world.pos.x, sp54.x,
                                   1.0f32, 6.5f32, 0.0f32);
                Math_SmoothStepToF(&mut (*this).actor.world.pos.y, sp54.y,
                                   1.0f32, 6.5f32, 0.0f32);
                Math_SmoothStepToF(&mut (*this).actor.world.pos.z, sp54.z,
                                   1.0f32, 6.5f32, 0.0f32);
                sp50 =
                    Math_Vec3f_DistXYZ(&mut sp54,
                                       &mut (*this).actor.world.pos);
                if sp50 <= 60.0f32 {
                    tmp =
                        Math_SmoothStepToS(&mut (*this).actor.shape.rot.x,
                                           (*this).actor.home.rot.x,
                                           1 as libc::c_int as s16,
                                           0x7d0 as libc::c_int as s16,
                                           0 as libc::c_int as s16);
                    sp4E =
                        if tmp as libc::c_int >= 0 as libc::c_int {
                            tmp as libc::c_int
                        } else { -(tmp as libc::c_int) } as s16;
                    tmp =
                        Math_SmoothStepToS(&mut (*this).actor.shape.rot.y,
                                           (*this).actor.home.rot.y,
                                           1 as libc::c_int as s16,
                                           0x7d0 as libc::c_int as s16,
                                           0 as libc::c_int as s16);
                    sp4E =
                        (sp4E as libc::c_int +
                             if tmp as libc::c_int >= 0 as libc::c_int {
                                 tmp as libc::c_int
                             } else { -(tmp as libc::c_int) }) as s16;
                    if sp50 == 0.0f32 &&
                           sp4E as libc::c_int == 0 as libc::c_int {
                        if (*this).isDead == 0 {
                            if (*this).actor.params as libc::c_int >=
                                   BOSSVA_BARI_LOWER_1 as libc::c_int {
                                if (*this).actor.params as libc::c_int ==
                                       BOSSVA_BARI_LOWER_1 as libc::c_int {
                                    sBodyBari[0 as libc::c_int as usize] =
                                        sBodyBari[0 as libc::c_int as
                                                      usize].wrapping_add(1)
                                } else {
                                    sBodyBari[((*this).actor.params as
                                                   libc::c_int -
                                                   BOSSVA_BARI_UPPER_1 as
                                                       libc::c_int) as usize]
                                        =
                                        sBodyBari[((*this).actor.params as
                                                       libc::c_int -
                                                       BOSSVA_BARI_UPPER_1 as
                                                           libc::c_int) as
                                                      usize].wrapping_add(1)
                                }
                            } else {
                                sBodyBari[((*this).actor.params as libc::c_int
                                               -
                                               BOSSVA_BARI_UPPER_1 as
                                                   libc::c_int +
                                               1 as libc::c_int) as usize] =
                                    sBodyBari[((*this).actor.params as
                                                   libc::c_int -
                                                   BOSSVA_BARI_UPPER_1 as
                                                       libc::c_int +
                                                   1 as libc::c_int) as
                                                  usize].wrapping_add(1)
                            }
                            (*this).timer = -(30 as libc::c_int);
                            (*this).isDead = (*this).isDead.wrapping_add(1)
                        } else {
                            (*this).timer += 1;
                            if (*this).timer == 0 as libc::c_int {
                                Actor_Kill(&mut (*this).actor);
                            }
                        }
                        return
                    }
                }
            }
            current_block_68 = 8269604632731669067;
        }
        7 => { current_block_68 = 8269604632731669067; }
        13 => {
            (*this).timer += 1;
            if (*this).timer == 0 as libc::c_int {
                Actor_Kill(&mut (*this).actor);
            }
            return
        }
        10 | 11 | 12 | _ => { current_block_68 = 10512632378975961025; }
    }
    match current_block_68 {
        8269604632731669067 => {
            (*this).timer -= 1;
            if (*this).timer == 0 as libc::c_int {
                (*this).timer2 = 0 as libc::c_int as s16
            } else {
                func_80035844(&mut (*((*this).actor.parent as
                                          *mut BossVa)).actor.world.pos,
                              &mut (*this).actor.world.pos,
                              &mut (*this).actor.world.rot, 0 as libc::c_int);
                (*this).unk_1A0 =
                    Math_Vec3f_DistXYZ(&mut (*((*this).actor.parent as
                                                   *mut BossVa)).actor.world.pos,
                                       &mut (*this).actor.world.pos);
                if sp50 > 30.0f32 {
                    BossVa_Spark(globalCtx, this, 1 as libc::c_int,
                                 80 as libc::c_int as s16, 15.0f32, 0.0f32,
                                 SPARK_BARI as libc::c_int as u8_0, 1.0f32,
                                 1 as libc::c_int as u8_0);
                }
            }
        }
        _ => { }
    }
    if (*globalCtx).gameplayFrames.wrapping_rem(4 as libc::c_int as
                                                    libc::c_uint) ==
           0 as libc::c_int as libc::c_uint &&
           (sCsState as libc::c_int) < INTRO_ATTACH_BARI as libc::c_int {
        BossVa_Spark(globalCtx, this, 1 as libc::c_int,
                     70 as libc::c_int as s16, 25.0f32, 20.0f32,
                     SPARK_BARI as libc::c_int as u8_0, 2.0f32,
                     1 as libc::c_int as u8_0);
    }
    if Rand_ZeroOne() < 0.1f32 {
        Audio_PlayActorSound2(&mut (*this).actor,
                              (0x3943 as libc::c_int - 0x800 as libc::c_int)
                                  as u16_0);
    };
}
#[no_mangle]
pub unsafe extern "C" fn BossVa_SetupBariPhase3Attack(mut this: *mut BossVa,
                                                      mut globalCtx:
                                                          *mut GlobalContext) {
    Animation_Change(&mut (*this).skelAnime, &mut gBarinadeBariAnim, 1.0f32,
                     0.0f32,
                     Animation_GetLastFrame(&mut gBarinadeBariAnim as
                                                *mut AnimationHeader as
                                                *mut libc::c_void) as f32_0,
                     ANIMMODE_LOOP as libc::c_int as u8_0, 0.0f32);
    (*this).timer2 = 0x80 as libc::c_int as s16;
    (*this).unk_1F0 = 0x78 as libc::c_int as s16;
    (*this).unk_1A0 = 60.0f32;
    (*this).unk_1A8 = 0.0f32;
    (*this).actor.flags &=
        !((1 as libc::c_int) << 0 as libc::c_int) as libc::c_uint;
    BossVa_SetupAction(this,
                       Some(BossVa_BariPhase3Attack as
                                unsafe extern "C" fn(_: *mut BossVa,
                                                     _: *mut GlobalContext)
                                    -> ()));
}
#[no_mangle]
pub unsafe extern "C" fn BossVa_BariPhase3Attack(mut this: *mut BossVa,
                                                 mut globalCtx:
                                                     *mut GlobalContext) {
    let mut player: *mut Player =
        (*globalCtx).actorCtx.actorLists[ACTORCAT_PLAYER as libc::c_int as
                                             usize].head as *mut Player;
    let mut boomerang: *mut EnBoom = 0 as *mut EnBoom;
    let mut sp54: Vec3f = (*((*this).actor.parent as *mut BossVa)).unk_1D8;
    let mut sp52: s16 = 0;
    let mut pad: s32 = 0;
    (*this).unk_1A4 += Rand_ZeroOne() * 0.5f32;
    sp52 = ((*this).timer2 as libc::c_int & 0x1ff as libc::c_int) as s16;
    if (*globalCtx).gameplayFrames.wrapping_rem(128 as libc::c_int as
                                                    libc::c_uint) ==
           0 as libc::c_int as libc::c_uint {
        (*this).headRot.x =
            ((Rand_ZeroOne() * 100.0f32) as s16 as libc::c_int +
                 100 as libc::c_int) as s16
    }
    Math_SmoothStepToS(&mut (*this).headRot.z, (*this).headRot.x,
                       1 as libc::c_int as s16, 0x1e as libc::c_int as s16,
                       0 as libc::c_int as s16);
    (*this).headRot.y =
        ((*this).headRot.y as libc::c_int + (*this).headRot.z as libc::c_int)
            as s16;
    if (*this).colliderLightning.base.atFlags as libc::c_int &
           (1 as libc::c_int) << 1 as libc::c_int != 0 ||
           (*this).colliderSph.base.atFlags as libc::c_int &
               (1 as libc::c_int) << 1 as libc::c_int != 0 {
        if (*this).colliderLightning.base.at ==
               &mut (*player).actor as *mut Actor ||
               (*this).colliderSph.base.at ==
                   &mut (*player).actor as *mut Actor {
            func_8002F71C(globalCtx, &mut (*this).actor, 8.0f32,
                          (*((*this).actor.parent as
                                 *mut BossVa)).actor.yawTowardsPlayer,
                          8.0f32);
            Audio_PlayActorSound2(&mut (*player).actor,
                                  0x83e as libc::c_int as u16_0);
            (*this).colliderSph.base.at = 0 as *mut Actor;
            (*this).colliderLightning.base.at = 0 as *mut Actor
        }
        (*this).colliderLightning.base.atFlags =
            ((*this).colliderLightning.base.atFlags as libc::c_int &
                 !((1 as libc::c_int) << 1 as libc::c_int)) as u8_0;
        (*this).colliderSph.base.atFlags =
            ((*this).colliderSph.base.atFlags as libc::c_int &
                 !((1 as libc::c_int) << 1 as libc::c_int)) as u8_0
    }
    if (*this).colliderSph.base.acFlags as libc::c_int &
           (1 as libc::c_int) << 1 as libc::c_int != 0 {
        (*this).colliderSph.base.acFlags =
            ((*this).colliderSph.base.acFlags as libc::c_int &
                 !((1 as libc::c_int) << 1 as libc::c_int)) as u8_0;
        if (*(*this).colliderSph.base.ac).id as libc::c_int ==
               ACTOR_EN_BOOM as libc::c_int &&
               sp52 as libc::c_int >= 128 as libc::c_int {
            boomerang = (*this).colliderSph.base.ac as *mut EnBoom;
            (*boomerang).returnTimer = 0 as libc::c_int as u8_0;
            (*boomerang).moveTo = &mut (*player).actor;
            (*boomerang).actor.world.rot.y =
                (*boomerang).actor.yawTowardsPlayer;
            Audio_PlayActorSound2(&mut (*this).actor,
                                  0x1808 as libc::c_int as u16_0);
        }
    }
    (*this).actor.world.pos.x =
        Math_SinS((*this).actor.world.rot.y) * (*this).unk_1A0 + sp54.x;
    (*this).actor.world.pos.z =
        Math_CosS((*this).actor.world.rot.y) * (*this).unk_1A0 + sp54.z;
    Math_SmoothStepToF(&mut (*this).actor.world.pos.y, 4.0f32, 1.0f32, 2.0f32,
                       0.0f32);
    (*this).actor.world.pos.y += 2.0f32 * Math_SinF((*this).unk_1A4);
    (*this).actor.world.rot.x =
        Math_Vec3f_Pitch(&mut sp54, &mut (*this).actor.world.pos);
    Math_SmoothStepToF(&mut (*this).unk_1A0, 160.0f32, 1.0f32, 2.0f32,
                       0.0f32);
    Math_SmoothStepToS(&mut (*this).actor.shape.rot.x,
                       0 as libc::c_int as s16, 1 as libc::c_int as s16,
                       0x5dc as libc::c_int as s16, 0 as libc::c_int as s16);
    if (*this).timer2 as libc::c_int & 0x200 as libc::c_int == 0 {
        (*this).unk_1AC = 0xbb8 as libc::c_int as s16
    } else { (*this).unk_1AC = -(0xbb8 as libc::c_int) as s16 }
    if sp52 as libc::c_int >= 128 as libc::c_int {
        BossVa_Spark(globalCtx, this, 1 as libc::c_int,
                     75 as libc::c_int as s16, 15.0f32, 7.0f32,
                     SPARK_TETHER as libc::c_int as u8_0, 1.0f32,
                     1 as libc::c_int as u8_0);
        CollisionCheck_SetAC(globalCtx, &mut (*globalCtx).colChkCtx,
                             &mut (*this).colliderSph.base);
        sPhase3StopMoving = 0 as libc::c_int as u8_0
    } else { sPhase3StopMoving = 1 as libc::c_int as u8_0 }
    CollisionCheck_SetAT(globalCtx, &mut (*globalCtx).colChkCtx,
                         &mut (*this).colliderLightning.base);
    CollisionCheck_SetAT(globalCtx, &mut (*globalCtx).colChkCtx,
                         &mut (*this).colliderSph.base);
    if (*globalCtx).gameplayFrames.wrapping_rem(4 as libc::c_int as
                                                    libc::c_uint) ==
           0 as libc::c_int as libc::c_uint {
        Math_SmoothStepToS(&mut (*this).unk_1F0, 0x78 as libc::c_int as s16,
                           1 as libc::c_int as s16, 0xa as libc::c_int as s16,
                           0 as libc::c_int as s16);
    }
    if Rand_ZeroOne() < 0.1f32 {
        Audio_PlayActorSound2(&mut (*this).actor,
                              (0x3943 as libc::c_int - 0x800 as libc::c_int)
                                  as u16_0);
    }
    (*this).actor.world.rot.y =
        ((*this).actor.world.rot.y as libc::c_int +
             (*this).unk_1AC as libc::c_int) as s16;
    if sBodyState as libc::c_int & 0x7f as libc::c_int != 0 {
        BossVa_SetupBariPhase3Stunned(this, globalCtx);
    };
}
#[no_mangle]
pub unsafe extern "C" fn BossVa_SetupBariPhase2Attack(mut this: *mut BossVa,
                                                      mut globalCtx:
                                                          *mut GlobalContext) {
    Animation_Change(&mut (*this).skelAnime, &mut gBarinadeBariAnim, 1.0f32,
                     0.0f32,
                     Animation_GetLastFrame(&mut gBarinadeBariAnim as
                                                *mut AnimationHeader as
                                                *mut libc::c_void) as f32_0,
                     ANIMMODE_LOOP as libc::c_int as u8_0, 0.0f32);
    (*this).timer2 = 0x40 as libc::c_int as s16;
    (*this).unk_1F0 = 0x78 as libc::c_int as s16;
    (*this).unk_1A0 = 60.0f32;
    (*this).unk_1A8 = 0.0f32;
    (*this).actor.flags &=
        !((1 as libc::c_int) << 0 as libc::c_int) as libc::c_uint;
    BossVa_SetupAction(this,
                       Some(BossVa_BariPhase2Attack as
                                unsafe extern "C" fn(_: *mut BossVa,
                                                     _: *mut GlobalContext)
                                    -> ()));
}
#[no_mangle]
pub unsafe extern "C" fn BossVa_BariPhase2Attack(mut this: *mut BossVa,
                                                 mut globalCtx:
                                                     *mut GlobalContext) {
    let mut player: *mut Player =
        (*globalCtx).actorCtx.actorLists[ACTORCAT_PLAYER as libc::c_int as
                                             usize].head as *mut Player;
    let mut boomerang: *mut EnBoom = 0 as *mut EnBoom;
    let mut sp54: Vec3f = (*((*this).actor.parent as *mut BossVa)).unk_1D8;
    let mut sp52: s16 = 0;
    let mut sp50: s16 = 0;
    let mut sp4C: f32_0 = 0.;
    let mut pad: s32 = 0;
    (*this).unk_1A4 += Rand_ZeroOne() * 0.5f32;
    sp52 = ((*this).timer2 as libc::c_int & 0x1ff as libc::c_int) as s16;
    if (*globalCtx).gameplayFrames.wrapping_rem(128 as libc::c_int as
                                                    libc::c_uint) ==
           0 as libc::c_int as libc::c_uint {
        (*this).headRot.x =
            ((Rand_ZeroOne() * 100.0f32) as s16 as libc::c_int +
                 100 as libc::c_int) as s16
    }
    sp50 =
        (sFightPhase as libc::c_int * 70 as libc::c_int - 280 as libc::c_int)
            as s16;
    Math_SmoothStepToS(&mut (*this).headRot.z, (*this).headRot.x,
                       1 as libc::c_int as s16, 0x1e as libc::c_int as s16,
                       0 as libc::c_int as s16);
    (*this).headRot.y =
        ((*this).headRot.y as libc::c_int + (*this).headRot.z as libc::c_int)
            as s16;
    if sKillBari as libc::c_int != 0 as libc::c_int {
        sKillBari = sKillBari.wrapping_sub(1);
        BossVa_KillBari(this, globalCtx);
        return
    }
    if (*this).colliderLightning.base.atFlags as libc::c_int &
           (1 as libc::c_int) << 1 as libc::c_int != 0 ||
           (*this).colliderSph.base.atFlags as libc::c_int &
               (1 as libc::c_int) << 1 as libc::c_int != 0 {
        if (*this).colliderLightning.base.at ==
               &mut (*player).actor as *mut Actor ||
               (*this).colliderSph.base.at ==
                   &mut (*player).actor as *mut Actor {
            func_8002F71C(globalCtx, &mut (*this).actor, 8.0f32,
                          (*((*this).actor.parent as
                                 *mut BossVa)).actor.yawTowardsPlayer,
                          8.0f32);
            Audio_PlayActorSound2(&mut (*player).actor,
                                  0x83e as libc::c_int as u16_0);
            (*this).colliderSph.base.at = 0 as *mut Actor;
            (*this).colliderLightning.base.at = 0 as *mut Actor
        }
        (*this).colliderLightning.base.atFlags =
            ((*this).colliderLightning.base.atFlags as libc::c_int &
                 !((1 as libc::c_int) << 1 as libc::c_int)) as u8_0;
        (*this).colliderSph.base.atFlags =
            ((*this).colliderSph.base.atFlags as libc::c_int &
                 !((1 as libc::c_int) << 1 as libc::c_int)) as u8_0
    }
    Math_SmoothStepToF(&mut (*this).actor.world.pos.y, 4.0f32, 1.0f32, 2.0f32,
                       0.0f32);
    (*this).actor.world.rot.x =
        Math_Vec3f_Pitch(&mut sp54, &mut (*this).actor.world.pos);
    if (*globalCtx).gameplayFrames.wrapping_rem(8 as libc::c_int as
                                                    libc::c_uint) ==
           0 as libc::c_int as libc::c_uint {
        Math_SmoothStepToS(&mut (*this).unk_1F0, 0x28 as libc::c_int as s16,
                           1 as libc::c_int as s16, 0xa as libc::c_int as s16,
                           0 as libc::c_int as s16);
        BossVa_Spark(globalCtx, this, 1 as libc::c_int, (*this).unk_1F0,
                     25.0f32, 20.0f32, 2 as libc::c_int as u8_0, 2.0f32,
                     1 as libc::c_int as u8_0);
    }
    if sPhase2Timer as libc::c_int & 0x100 as libc::c_int == 0 &&
           (*((*this).actor.parent as *mut BossVa)).actor.colorFilterTimer as
               libc::c_int == 0 as libc::c_int {
        sp4C = 200.0f32;
        BossVa_Spark(globalCtx, this, 1 as libc::c_int,
                     125 as libc::c_int as s16, 15.0f32, 7.0f32,
                     SPARK_TETHER as libc::c_int as u8_0, 1.0f32,
                     1 as libc::c_int as u8_0);
        (*this).actor.flags &=
            !((1 as libc::c_int) << 0 as libc::c_int) as libc::c_uint;
        if (*this).actor.params as libc::c_int & 1 as libc::c_int != 0 {
            sp4C = -200.0f32
        }
        Math_SmoothStepToF(&mut (*this).unk_1A0,
                           Math_SinS((sPhase2Timer as libc::c_int *
                                          0x190 as libc::c_int) as s16) * sp4C
                               + 320.0f32, 1.0f32, 10.0f32, 0.0f32);
        Math_SmoothStepToS(&mut (*this).unk_1AC,
                           (sp50 as libc::c_int + 0x1f4 as libc::c_int) as
                               s16, 1 as libc::c_int as s16,
                           0x3c as libc::c_int as s16,
                           0 as libc::c_int as s16);
        (*this).actor.world.pos.y += 2.0f32 * Math_SinF((*this).unk_1A4);
        if (*this).colliderSph.base.acFlags as libc::c_int &
               (1 as libc::c_int) << 1 as libc::c_int != 0 {
            (*this).colliderSph.base.acFlags =
                ((*this).colliderSph.base.acFlags as libc::c_int &
                     !((1 as libc::c_int) << 1 as libc::c_int)) as u8_0;
            if (*(*this).colliderSph.base.ac).id as libc::c_int ==
                   ACTOR_EN_BOOM as libc::c_int &&
                   sp52 as libc::c_int >= 64 as libc::c_int {
                boomerang = (*this).colliderSph.base.ac as *mut EnBoom;
                (*boomerang).returnTimer = 0 as libc::c_int as u8_0;
                (*boomerang).moveTo = &mut (*player).actor;
                (*boomerang).actor.world.rot.y =
                    (*boomerang).actor.yawTowardsPlayer;
                Audio_PlayActorSound2(&mut (*this).actor,
                                      0x1808 as libc::c_int as u16_0);
            }
        }
        CollisionCheck_SetAT(globalCtx, &mut (*globalCtx).colChkCtx,
                             &mut (*this).colliderLightning.base);
        CollisionCheck_SetAT(globalCtx, &mut (*globalCtx).colChkCtx,
                             &mut (*this).colliderSph.base);
    } else {
        (*this).actor.flags |=
            ((1 as libc::c_int) << 0 as libc::c_int) as libc::c_uint;
        Math_SmoothStepToS(&mut (*this).unk_1AC,
                           (sp50 as libc::c_int + 150 as libc::c_int) as s16,
                           1 as libc::c_int as s16,
                           0x3c as libc::c_int as s16,
                           0 as libc::c_int as s16);
        if (*((*this).actor.parent as *mut BossVa)).actor.colorFilterTimer as
               libc::c_int == 0 as libc::c_int {
            Math_SmoothStepToF(&mut (*this).unk_1A0, 180.0f32, 1.0f32, 1.5f32,
                               0.0f32);
        } else {
            (*this).unk_1AC = 0 as libc::c_int as s16;
            if (*this).actor.colorFilterTimer as libc::c_int ==
                   0 as libc::c_int {
                Actor_SetColorFilter(&mut (*this).actor,
                                     0 as libc::c_int as s16,
                                     255 as libc::c_int as s16,
                                     0x2000 as libc::c_int as s16,
                                     (*((*this).actor.parent as
                                            *mut BossVa)).actor.colorFilterTimer
                                         as s16);
            }
        }
        (*this).actor.world.pos.y += Math_SinF((*this).unk_1A4) * 4.0f32;
        if (*this).colliderSph.base.acFlags as libc::c_int &
               (1 as libc::c_int) << 1 as libc::c_int != 0 {
            BossVa_KillBari(this, globalCtx);
        }
    }
    Math_SmoothStepToS(&mut (*this).actor.shape.rot.x,
                       0 as libc::c_int as s16, 1 as libc::c_int as s16,
                       0x5dc as libc::c_int as s16, 0 as libc::c_int as s16);
    CollisionCheck_SetAC(globalCtx, &mut (*globalCtx).colChkCtx,
                         &mut (*this).colliderSph.base);
    if (*globalCtx).gameplayFrames.wrapping_rem(4 as libc::c_int as
                                                    libc::c_uint) ==
           0 as libc::c_int as libc::c_uint {
        Math_SmoothStepToS(&mut (*this).unk_1F0, 0x78 as libc::c_int as s16,
                           1 as libc::c_int as s16, 0xa as libc::c_int as s16,
                           0 as libc::c_int as s16);
    }
    if Rand_ZeroOne() < 0.1f32 {
        Audio_PlayActorSound2(&mut (*this).actor,
                              (0x3943 as libc::c_int - 0x800 as libc::c_int)
                                  as u16_0);
    }
    if (*((*this).actor.parent as *mut BossVa)).actor.colorFilterTimer as
           libc::c_int == 0 as libc::c_int {
        if (*this).timer2 as libc::c_int & 0x400 as libc::c_int == 0 {
            (*this).actor.world.rot.y =
                ((*this).actor.world.rot.y as libc::c_int +
                     (*this).unk_1AC as libc::c_int) as s16
        } else {
            (*this).actor.world.rot.y =
                ((*this).actor.world.rot.y as libc::c_int -
                     (*this).unk_1AC as libc::c_int) as s16
        }
        (*this).actor.world.pos.x =
            Math_SinS((*this).actor.world.rot.y) * (*this).unk_1A0 + sp54.x;
        (*this).actor.world.pos.z =
            Math_CosS((*this).actor.world.rot.y) * (*this).unk_1A0 + sp54.z
    };
}
#[no_mangle]
pub unsafe extern "C" fn BossVa_SetupBariPhase3Stunned(mut this: *mut BossVa,
                                                       mut globalCtx:
                                                           *mut GlobalContext) {
    (*this).actor.flags |=
        ((1 as libc::c_int) << 0 as libc::c_int) as libc::c_uint;
    (*this).timer = (*((*this).actor.parent as *mut BossVa)).timer;
    Actor_SetColorFilter(&mut (*this).actor, 0 as libc::c_int as s16,
                         255 as libc::c_int as s16,
                         0x2000 as libc::c_int as s16, (*this).timer as s16);
    BossVa_SetupAction(this,
                       Some(BossVa_BariPhase3Stunned as
                                unsafe extern "C" fn(_: *mut BossVa,
                                                     _: *mut GlobalContext)
                                    -> ()));
}
#[no_mangle]
pub unsafe extern "C" fn BossVa_BariPhase3Stunned(mut this: *mut BossVa,
                                                  mut globalCtx:
                                                      *mut GlobalContext) {
    let mut sp44_pad: s32 = 0;
    let mut sp40: Vec3f = (*((*this).actor.parent as *mut BossVa)).unk_1D8;
    (*this).actor.world.rot.x =
        Math_Vec3f_Pitch(&mut (*((*this).actor.parent as
                                     *mut BossVa)).actor.world.pos,
                         &mut (*this).actor.world.pos);
    if (*this).colliderSph.base.acFlags as libc::c_int &
           (1 as libc::c_int) << 1 as libc::c_int != 0 {
        BossVa_KillBari(this, globalCtx);
        return
    }
    (*this).unk_1A4 += Rand_ZeroOne() * 0.5f32;
    Math_SmoothStepToF(&mut (*this).actor.world.pos.y, 4.0f32, 1.0f32, 2.0f32,
                       0.0f32);
    (*this).actor.world.pos.y += Math_SinF((*this).unk_1A4) * 3.0f32;
    CollisionCheck_SetAC(globalCtx, &mut (*globalCtx).colChkCtx,
                         &mut (*this).colliderSph.base);
    if (*globalCtx).gameplayFrames.wrapping_rem(4 as libc::c_int as
                                                    libc::c_uint) ==
           0 as libc::c_int as libc::c_uint {
        Math_SmoothStepToS(&mut (*this).unk_1F0, 0x28 as libc::c_int as s16,
                           1 as libc::c_int as s16, 0xa as libc::c_int as s16,
                           0 as libc::c_int as s16);
        BossVa_Spark(globalCtx, this, 1 as libc::c_int, (*this).unk_1F0,
                     25.0f32, 20.0f32, SPARK_BARI as libc::c_int as u8_0,
                     2.0f32, 1 as libc::c_int as u8_0);
    }
    (*this).timer -= 1;
    (*this).actor.world.rot.x =
        Math_Vec3f_Pitch(&mut sp40, &mut (*this).actor.world.pos);
    if (*this).timer <= 0 as libc::c_int {
        if (*this).timer == 0 as libc::c_int {
            (*this).timer2 = 0 as libc::c_int as s16
        } else {
            BossVa_Spark(globalCtx, this, 1 as libc::c_int,
                         85 as libc::c_int as s16, 15.0f32, 0.0f32,
                         SPARK_TETHER as libc::c_int as u8_0, 1.0f32,
                         1 as libc::c_int as u8_0);
            if (*this).timer2 as libc::c_int >= 0x10 as libc::c_int {
                (*this).actor.flags &=
                    !((1 as libc::c_int) << 0 as libc::c_int) as libc::c_uint;
                (*this).timer2 = 0x80 as libc::c_int as s16;
                BossVa_SetupAction(this,
                                   Some(BossVa_BariPhase3Attack as
                                            unsafe extern "C" fn(_:
                                                                     *mut BossVa,
                                                                 _:
                                                                     *mut GlobalContext)
                                                -> ()));
            }
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn BossVa_SetupBariDeath(mut this: *mut BossVa) {
    (*this).actor.flags &=
        !((1 as libc::c_int) << 0 as libc::c_int) as libc::c_uint;
    (*this).timer = 30 as libc::c_int;
    Audio_PlayActorSound2(&mut (*this).actor, 0x3944 as libc::c_int as u16_0);
    (*this).isDead = (*this).isDead.wrapping_add(1);
    BossVa_SetupAction(this,
                       Some(BossVa_BariDeath as
                                unsafe extern "C" fn(_: *mut BossVa,
                                                     _: *mut GlobalContext)
                                    -> ()));
}
#[no_mangle]
pub unsafe extern "C" fn BossVa_BariDeath(mut this: *mut BossVa,
                                          mut globalCtx: *mut GlobalContext) {
    (*this).timer -= 1;
    if (*this).timer == 0 as libc::c_int { Actor_Kill(&mut (*this).actor); };
}
#[no_mangle]
pub unsafe extern "C" fn BossVa_SetupDoor(mut this: *mut BossVa,
                                          mut globalCtx: *mut GlobalContext) {
    if sCsState as libc::c_int >= INTRO_SPAWN_BARI as libc::c_int {
        sDoorState = 100 as libc::c_int as s16
    }
    (*this).actor.flags &=
        !((1 as libc::c_int) << 0 as libc::c_int) as libc::c_uint;
    BossVa_SetupAction(this,
                       Some(BossVa_Door as
                                unsafe extern "C" fn(_: *mut BossVa,
                                                     _: *mut GlobalContext)
                                    -> ()));
}
#[no_mangle]
pub unsafe extern "C" fn BossVa_Door(mut this: *mut BossVa,
                                     mut globalCtx: *mut GlobalContext) {
    if sDoorState as libc::c_int == 29 as libc::c_int {
        Audio_PlayActorSound2(&mut (*this).actor,
                              0x2865 as libc::c_int as u16_0);
    }
    if sCsState as libc::c_int <= INTRO_DOOR_SHUT as libc::c_int {
        if (sDoorState as libc::c_int) < 100 as libc::c_int {
            sDoorState = (sDoorState as libc::c_int + 8 as libc::c_int) as s16
        } else { sDoorState = 100 as libc::c_int as s16 }
    };
}
#[no_mangle]
pub unsafe extern "C" fn BossVa_Update(mut thisx: *mut Actor,
                                       mut globalCtx2: *mut GlobalContext) {
    let mut globalCtx: *mut GlobalContext = globalCtx2;
    let mut this: *mut BossVa = thisx as *mut BossVa;
    let mut boomerang: *mut EnBoom = 0 as *mut EnBoom;
    let mut i: s32 = 0;
    (*this).actionFunc.expect("non-null function pointer")(this, globalCtx);
    match (*this).actor.params as libc::c_int {
        -1 => {
            if (*this).colliderBody.base.acFlags as libc::c_int &
                   (1 as libc::c_int) << 1 as libc::c_int != 0 {
                (*this).colliderBody.base.acFlags =
                    ((*this).colliderBody.base.acFlags as libc::c_int &
                         !((1 as libc::c_int) << 1 as libc::c_int)) as u8_0;
                if (*(*this).colliderBody.base.ac).id as libc::c_int ==
                       ACTOR_EN_BOOM as libc::c_int {
                    boomerang = (*this).colliderBody.base.ac as *mut EnBoom;
                    (*boomerang).returnTimer = 0 as libc::c_int as u8_0
                }
            }
            BossVa_UpdateEffects(globalCtx);
            i = 2 as libc::c_int;
            while i >= 0 as libc::c_int {
                if (*globalCtx).envCtx.adjAmbientColor[i as usize] as
                       libc::c_int - 1 as libc::c_int > 0 as libc::c_int {
                    (*globalCtx).envCtx.adjAmbientColor[i as usize] =
                        ((*globalCtx).envCtx.adjAmbientColor[i as usize] as
                             libc::c_int - 1 as libc::c_int) as s16
                } else {
                    (*globalCtx).envCtx.adjAmbientColor[i as usize] =
                        0 as libc::c_int as s16
                }
                if (*globalCtx).envCtx.adjLight1Color[i as usize] as
                       libc::c_int - 10 as libc::c_int > 0 as libc::c_int {
                    (*globalCtx).envCtx.adjLight1Color[i as usize] =
                        ((*globalCtx).envCtx.adjLight1Color[i as usize] as
                             libc::c_int - 10 as libc::c_int) as s16
                } else {
                    (*globalCtx).envCtx.adjLight1Color[i as usize] =
                        0 as libc::c_int as s16
                }
                if (*globalCtx).envCtx.adjFogColor[i as usize] as libc::c_int
                       - 10 as libc::c_int > 0 as libc::c_int {
                    (*globalCtx).envCtx.adjFogColor[i as usize] =
                        ((*globalCtx).envCtx.adjFogColor[i as usize] as
                             libc::c_int - 10 as libc::c_int) as s16
                } else {
                    (*globalCtx).envCtx.adjFogColor[i as usize] =
                        0 as libc::c_int as s16
                }
                i -= 1
            }
            if (*this).onCeiling as libc::c_int > 0 as libc::c_int {
                (*this).onCeiling = (*this).onCeiling.wrapping_sub(1)
                // not used by body
            }
        }
        0 | 1 | 2 | 3 | 4 | 5 | 19 => { }
        _ => {
            (*this).timer2 +=
                1; // This stack slot is almost certainly actually globalCtx2, but can't make it match
            (*this).actor.focus.pos = (*this).actor.world.pos;
            (*this).actor.focus.pos.y += 45.0f32;
            (*this).unk_1D8.y =
                Math_CosS(((*this).timer2 as libc::c_int *
                               0xfa4 as libc::c_int) as s16) * 0.24f32 +
                    0.76f32;
            (*this).unk_1D8.x =
                Math_SinS(((*this).timer2 as libc::c_int *
                               0xfa4 as libc::c_int) as s16) * 0.2f32 + 1.0f32
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn BossVa_BodyOverrideLimbDraw(mut globalCtx:
                                                         *mut GlobalContext,
                                                     mut limbIndex: s32,
                                                     mut dList: *mut *mut Gfx,
                                                     mut pos: *mut Vec3f,
                                                     mut rot: *mut Vec3s,
                                                     mut thisx:
                                                         *mut libc::c_void)
 -> s32 {
    let mut this: *mut BossVa = thisx as *mut BossVa;
    let mut pad: s32 = 0;
    let mut __gfxCtx: *mut GraphicsContext = 0 as *mut GraphicsContext;
    let mut dispRefs: [*mut Gfx; 4] = [0 as *mut Gfx; 4];
    __gfxCtx = (*globalCtx).state.gfxCtx;
    Graph_OpenDisps(dispRefs.as_mut_ptr(), (*globalCtx).state.gfxCtx,
                    b"../z_boss_va.c\x00" as *const u8 as *const libc::c_char,
                    4156 as libc::c_int);
    if limbIndex == 20 as libc::c_int {
        let fresh6 = (*__gfxCtx).polyOpa.p;
        (*__gfxCtx).polyOpa.p = (*__gfxCtx).polyOpa.p.offset(1);
        let mut _g: *mut Gfx = fresh6;
        (*_g).words.w0 =
            (0xe7 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 24 as libc::c_int;
        (*_g).words.w1 = 0 as libc::c_int as libc::c_uint;
        let fresh7 = (*__gfxCtx).polyOpa.p;
        (*__gfxCtx).polyOpa.p = (*__gfxCtx).polyOpa.p.offset(1);
        let mut _g_0: *mut Gfx = fresh7;
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
            Gfx_TwoTexScroll((*globalCtx).state.gfxCtx, 0 as libc::c_int,
                             0 as libc::c_int as u32_0,
                             0 as libc::c_int as u32_0, 8 as libc::c_int,
                             16 as libc::c_int, 1 as libc::c_int,
                             0 as libc::c_int as u32_0,
                             (*globalCtx).gameplayFrames.wrapping_mul(-(2 as
                                                                            libc::c_int)
                                                                          as
                                                                          libc::c_uint).wrapping_rem(64
                                                                                                         as
                                                                                                         libc::c_int
                                                                                                         as
                                                                                                         libc::c_uint),
                             16 as libc::c_int, 16 as libc::c_int) as
                libc::c_uint;
        let fresh8 = (*__gfxCtx).polyOpa.p;
        (*__gfxCtx).polyOpa.p = (*__gfxCtx).polyOpa.p.offset(1);
        let mut _g_1: *mut Gfx = fresh8;
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
                ((*this).bodyGlow as u32_0 &
                     (((0x1 as libc::c_int) << 8 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    0 as libc::c_int;
        Matrix_RotateX(-3.14159265358979323846f32 /
                           2 as libc::c_int as libc::c_float,
                       MTXMODE_APPLY as libc::c_int as u8_0);
    } else if limbIndex >= 10 as libc::c_int && limbIndex < 20 as libc::c_int
     {
        (*rot).x = ((*rot).x as libc::c_int - 0x4000 as libc::c_int) as s16;
        *dList = 0 as *mut Gfx
    } else if limbIndex == 6 as libc::c_int {
        Matrix_Scale((*this).unk_1A4, (*this).unk_1A4, (*this).unk_1A4,
                     MTXMODE_APPLY as libc::c_int as u8_0);
    } else if limbIndex == 61 as libc::c_int {
        Matrix_Scale((*this).unk_1A0, (*this).unk_1A0, (*this).unk_1A0,
                     MTXMODE_APPLY as libc::c_int as u8_0);
    } else if limbIndex == 7 as libc::c_int {
        (*rot).x = ((*rot).x as libc::c_int - 0xccc as libc::c_int) as s16
    }
    Graph_CloseDisps(dispRefs.as_mut_ptr(), (*globalCtx).state.gfxCtx,
                     b"../z_boss_va.c\x00" as *const u8 as
                         *const libc::c_char, 4183 as libc::c_int);
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn BossVa_BodyPostLimbDraw(mut globalCtx:
                                                     *mut GlobalContext,
                                                 mut limbIndex: s32,
                                                 mut dList: *mut *mut Gfx,
                                                 mut rot: *mut Vec3s,
                                                 mut thisx:
                                                     *mut libc::c_void) {
    let mut this: *mut BossVa = thisx as *mut BossVa;
    let mut sp78: Vec3f =
        { let mut init = Vec3f{x: 0.0f32, y: 0.0f32, z: 0.0f32,}; init };
    let mut pad: s32 = 0;
    let mut __gfxCtx: *mut GraphicsContext = 0 as *mut GraphicsContext;
    let mut dispRefs: [*mut Gfx; 4] = [0 as *mut Gfx; 4];
    __gfxCtx = (*globalCtx).state.gfxCtx;
    Graph_OpenDisps(dispRefs.as_mut_ptr(), (*globalCtx).state.gfxCtx,
                    b"../z_boss_va.c\x00" as *const u8 as *const libc::c_char,
                    4192 as libc::c_int);
    if limbIndex == 6 as libc::c_int {
        if (sFightPhase as libc::c_int) < 9 as libc::c_int {
            sp78.x = -1000.0f32
        } else { sp78.x = 200.0f32 }
        Matrix_MultVec3f(&mut sp78, &mut (*this).unk_1D8);
    } else if limbIndex >= 10 as libc::c_int && limbIndex < 20 as libc::c_int
                  &&
                  sBodyBari[(limbIndex - 10 as libc::c_int) as usize] as
                      libc::c_int != 0 as libc::c_int {
        if (limbIndex >= 16 as libc::c_int || limbIndex == 10 as libc::c_int)
               && sFightPhase as libc::c_int <= 9 as libc::c_int {
            let fresh9 = (*__gfxCtx).polyXlu.p;
            (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
            let mut _g: *mut Gfx = fresh9;
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
                              b"../z_boss_va.c\x00" as *const u8 as
                                  *const libc::c_char as *mut libc::c_char,
                              4208 as libc::c_int) as libc::c_uint;
            let fresh10 = (*__gfxCtx).polyXlu.p;
            (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
            let mut _g_0: *mut Gfx = fresh10;
            (*_g_0).words.w0 =
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
            (*_g_0).words.w1 = gBarinadeDL_008BB8.as_mut_ptr() as libc::c_uint
        } else if limbIndex >= 11 as libc::c_int &&
                      sFightPhase as libc::c_int <= 3 as libc::c_int {
            let fresh11 = (*__gfxCtx).polyXlu.p;
            (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
            let mut _g_1: *mut Gfx = fresh11;
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
                              b"../z_boss_va.c\x00" as *const u8 as
                                  *const libc::c_char as *mut libc::c_char,
                              4212 as libc::c_int) as libc::c_uint;
            let fresh12 = (*__gfxCtx).polyXlu.p;
            (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
            let mut _g_2: *mut Gfx = fresh12;
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
            (*_g_2).words.w1 = gBarinadeDL_008BB8.as_mut_ptr() as libc::c_uint
        }
        if sCsState as libc::c_int >= DEATH_START as libc::c_int {
            sp78.x = Rand_CenteredFloat(530.0f32);
            sp78.y = Rand_CenteredFloat(530.0f32);
            sp78.z = -60.0f32
        }
        Matrix_MultVec3f(&mut sp78,
                         &mut *(*this).effectPos.as_mut_ptr().offset((limbIndex
                                                                          -
                                                                          10
                                                                              as
                                                                              libc::c_int)
                                                                         as
                                                                         isize));
    } else if limbIndex == 25 as libc::c_int {
        let fresh13 = (*__gfxCtx).polyXlu.p;
        (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
        let mut _g_3: *mut Gfx = fresh13;
        (*_g_3).words.w0 =
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
        (*_g_3).words.w1 =
            Gfx_TwoTexScroll((*globalCtx).state.gfxCtx, 0 as libc::c_int,
                             0 as libc::c_int as u32_0,
                             (*globalCtx).gameplayFrames.wrapping_mul(10 as
                                                                          libc::c_int
                                                                          as
                                                                          libc::c_uint).wrapping_rem(128
                                                                                                         as
                                                                                                         libc::c_int
                                                                                                         as
                                                                                                         libc::c_uint),
                             16 as libc::c_int, 32 as libc::c_int,
                             1 as libc::c_int, 0 as libc::c_int as u32_0,
                             (*globalCtx).gameplayFrames.wrapping_mul(5 as
                                                                          libc::c_int
                                                                          as
                                                                          libc::c_uint).wrapping_rem(128
                                                                                                         as
                                                                                                         libc::c_int
                                                                                                         as
                                                                                                         libc::c_uint),
                             16 as libc::c_int, 32 as libc::c_int) as
                libc::c_uint;
        let fresh14 = (*__gfxCtx).polyXlu.p;
        (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
        let mut _g_4: *mut Gfx = fresh14;
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
                (((0 as libc::c_int | 0x2 as libc::c_int | 0 as libc::c_int) ^
                      0x1 as libc::c_int) as u32_0 &
                     (((0x1 as libc::c_int) << 8 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    0 as libc::c_int;
        (*_g_4).words.w1 =
            Matrix_NewMtx((*globalCtx).state.gfxCtx,
                          b"../z_boss_va.c\x00" as *const u8 as
                              *const libc::c_char as *mut libc::c_char,
                          4232 as libc::c_int) as libc::c_uint;
        let fresh15 = (*__gfxCtx).polyXlu.p;
        (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
        let mut _g_5: *mut Gfx = fresh15;
        (*_g_5).words.w0 =
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
        (*_g_5).words.w1 = gBarinadeDL_008D70.as_mut_ptr() as libc::c_uint
    } else if !(*dList).is_null() && limbIndex >= 29 as libc::c_int &&
                  limbIndex < 56 as libc::c_int {
        let fresh16 = (*__gfxCtx).polyXlu.p;
        (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
        let mut _g_6: *mut Gfx = fresh16;
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
                          b"../z_boss_va.c\x00" as *const u8 as
                              *const libc::c_char as *mut libc::c_char,
                          4236 as libc::c_int) as libc::c_uint;
        let fresh17 = (*__gfxCtx).polyXlu.p;
        (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
        let mut _g_7: *mut Gfx = fresh17;
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
        (*_g_7).words.w1 = *dList as libc::c_uint
    } else if limbIndex == 24 as libc::c_int &&
                  (sCsState as libc::c_int) < DEATH_START as libc::c_int {
        sp78.x = (*this).actor.shape.yOffset + 450.0f32 + -140.0f32;
        Matrix_MultVec3f(&mut sp78, &mut (*this).unk_280);
        sp78.x = 200.0f32;
        Matrix_MultVec3f(&mut sp78, &mut (*this).unk_274);
    }
    if limbIndex == 7 as libc::c_int &&
           sCsState as libc::c_int >= DEATH_START as libc::c_int {
        sp78.x = Rand_CenteredFloat(320.0f32) + -250.0f32;
        sp78.y = Rand_CenteredFloat(320.0f32);
        sp78.z = Rand_CenteredFloat(320.0f32);
        if sp78.y < 0.0f32 { sp78.y -= 150.0f32 } else { sp78.y += 150.0f32 }
        if sp78.z < 0.0f32 { sp78.z -= 150.0f32 } else { sp78.z += 150.0f32 }
        Matrix_MultVec3f(&mut sp78, &mut (*this).unk_274);
    }
    Graph_CloseDisps(dispRefs.as_mut_ptr(), (*globalCtx).state.gfxCtx,
                     b"../z_boss_va.c\x00" as *const u8 as
                         *const libc::c_char, 4264 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn BossVa_SupportOverrideLimbDraw(mut globalCtx:
                                                            *mut GlobalContext,
                                                        mut limbIndex: s32,
                                                        mut dList:
                                                            *mut *mut Gfx,
                                                        mut pos: *mut Vec3f,
                                                        mut rot: *mut Vec3s,
                                                        mut thisx:
                                                            *mut libc::c_void)
 -> s32 {
    let mut this: *mut BossVa = thisx as *mut BossVa;
    if (*this).onCeiling == 0 && limbIndex == 4 as libc::c_int {
        (*rot).z =
            ((*rot).z as libc::c_int + (*this).headRot.x as libc::c_int) as
                s16
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn BossVa_SupportPostLimbDraw(mut globalCtx:
                                                        *mut GlobalContext,
                                                    mut limbIndex: s32,
                                                    mut dList: *mut *mut Gfx,
                                                    mut rot: *mut Vec3s,
                                                    mut thisx:
                                                        *mut libc::c_void) {
    let mut this: *mut BossVa = thisx as *mut BossVa;
    let mut sp20: Vec3f =
        { let mut init = Vec3f{x: 0.0f32, y: 0.0f32, z: 0.0f32,}; init };
    let mut pad: s32 = 0;
    if (*this).onCeiling != 0 {
        match limbIndex {
            4 => {
                Matrix_MultVec3f(&mut sZeroVec, &mut (*this).actor.focus.pos);
                Collider_UpdateSpheres(0 as libc::c_int,
                                       &mut (*this).colliderSph);
            }
            7 => {
                Matrix_MultVec3f(&mut sZeroVec, &mut (*this).armTip);
                sp20.x =
                    (((*this).timer & 0x1f as libc::c_int) >>
                         1 as libc::c_int) as libc::c_float * -40.0f32;
                sp20.y =
                    (((*this).timer & 0x1f as libc::c_int) >>
                         1 as libc::c_int) as libc::c_float * -7.0f32;
                Matrix_MultVec3f(&mut sp20,
                                 &mut *(*this).effectPos.as_mut_ptr().offset(0
                                                                                 as
                                                                                 libc::c_int
                                                                                 as
                                                                                 isize));
            }
            9 => {
                sp20.x =
                    (((*this).timer & 0x1f as libc::c_int) >>
                         1 as libc::c_int) as libc::c_float * -60.0f32;
                sp20.y =
                    (((*this).timer & 0x1f as libc::c_int) >>
                         1 as libc::c_int) as libc::c_float * -45.0f32;
                Matrix_MultVec3f(&mut sp20,
                                 &mut *(*this).effectPos.as_mut_ptr().offset(1
                                                                                 as
                                                                                 libc::c_int
                                                                                 as
                                                                                 isize));
            }
            _ => { }
        }
    } else {
        match limbIndex {
            5 => { Matrix_MultVec3f(&mut sZeroVec, &mut (*this).armTip); }
            8 => {
                sp20.x =
                    ((*this).timer2 as libc::c_int & 7 as libc::c_int) as
                        libc::c_float * 90.0f32;
                Matrix_MultVec3f(&mut sp20,
                                 &mut *(*this).effectPos.as_mut_ptr().offset(2
                                                                                 as
                                                                                 libc::c_int
                                                                                 as
                                                                                 isize));
            }
            9 => {
                sp20.x =
                    ((*this).timer2 as libc::c_int & 7 as libc::c_int) as
                        libc::c_float * 50.0f32;
                Matrix_MultVec3f(&mut sp20,
                                 &mut *(*this).effectPos.as_mut_ptr().offset(1
                                                                                 as
                                                                                 libc::c_int
                                                                                 as
                                                                                 isize));
            }
            10 => {
                sp20.x =
                    ((*this).timer2 as libc::c_int & 7 as libc::c_int) as
                        libc::c_float * 46.0f32;
                Matrix_MultVec3f(&mut sp20,
                                 &mut *(*this).effectPos.as_mut_ptr().offset(0
                                                                                 as
                                                                                 libc::c_int
                                                                                 as
                                                                                 isize));
            }
            _ => { }
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn BossVa_ZapperOverrideLimbDraw(mut globalCtx:
                                                           *mut GlobalContext,
                                                       mut limbIndex: s32,
                                                       mut dList:
                                                           *mut *mut Gfx,
                                                       mut pos: *mut Vec3f,
                                                       mut rot: *mut Vec3s,
                                                       mut thisx:
                                                           *mut libc::c_void)
 -> s32 {
    let mut this: *mut BossVa = thisx as *mut BossVa;
    let mut zapperMtx: MtxF = MtxF{mf: [[0.; 4]; 4],};
    match limbIndex {
        4 => {
            (*rot).y =
                ((*rot).y as libc::c_int + (*this).unk_1E6 as libc::c_int) as
                    s16;
            (*rot).z =
                ((*rot).z as libc::c_int + (*this).unk_1E4 as libc::c_int) as
                    s16
        }
        5 => {
            (*rot).y =
                ((*rot).y as libc::c_int + (*this).unk_1EC as libc::c_int) as
                    s16;
            (*rot).z =
                ((*rot).z as libc::c_int + (*this).unk_1EA as libc::c_int) as
                    s16
        }
        7 => {
            Matrix_Translate((*pos).x, (*pos).y, (*pos).z,
                             MTXMODE_APPLY as libc::c_int as u8_0);
            Matrix_Get(&mut zapperMtx);
            Matrix_MtxFToZYXRotS(&mut zapperMtx, &mut sZapperRot,
                                 0 as libc::c_int);
            Matrix_RotateX(-(sZapperRot.x as libc::c_int) as libc::c_float *
                               (3.14159265358979323846f32 /
                                    0x8000 as libc::c_int as libc::c_float),
                           MTXMODE_APPLY as libc::c_int as u8_0);
            Matrix_RotateY(-(sZapperRot.y as libc::c_int) as libc::c_float *
                               (3.14159265358979323846f32 /
                                    0x8000 as libc::c_int as libc::c_float),
                           MTXMODE_APPLY as libc::c_int as u8_0);
            Matrix_RotateZ(-(sZapperRot.z as libc::c_int) as libc::c_float *
                               (3.14159265358979323846f32 /
                                    0x8000 as libc::c_int as libc::c_float),
                           MTXMODE_APPLY as libc::c_int as u8_0);
            Matrix_RotateY((*this).unk_1F2 as libc::c_int as libc::c_float *
                               (3.14159265358979323846f32 /
                                    0x8000 as libc::c_int as libc::c_float),
                           MTXMODE_APPLY as libc::c_int as u8_0);
            Matrix_RotateZ((*this).unk_1F0 as libc::c_int as libc::c_float *
                               (3.14159265358979323846f32 /
                                    0x8000 as libc::c_int as libc::c_float),
                           MTXMODE_APPLY as libc::c_int as u8_0);
            (*pos).z = 0.0f32;
            (*pos).y = (*pos).z;
            (*pos).x = (*pos).y;
            (*rot).z = 0 as libc::c_int as s16;
            (*rot).y = (*rot).z;
            (*rot).x = (*rot).y
        }
        _ => { }
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn BossVa_ZapperPostLimbDraw(mut globalCtx:
                                                       *mut GlobalContext,
                                                   mut limbIndex: s32,
                                                   mut dList: *mut *mut Gfx,
                                                   mut rot: *mut Vec3s,
                                                   mut thisx:
                                                       *mut libc::c_void) {
    let mut this: *mut BossVa = thisx as *mut BossVa;
    let mut sp70: Vec3f =
        { let mut init = Vec3f{x: 0.0f32, y: 0.0f32, z: 0.0f32,}; init };
    let mut sp64: Vec3f =
        { let mut init = Vec3f{x: 15.0f32, y: 0.0f32, z: 0.0f32,}; init };
    let mut sp58: Vec3f =
        { let mut init = Vec3f{x: -15.0f32, y: 0.0f32, z: 0.0f32,}; init };
    let mut sp4C: Vec3f =
        { let mut init = Vec3f{x: 15.0f32, y: 0.0f32, z: 0.0f32,}; init };
    let mut sp40: Vec3f =
        { let mut init = Vec3f{x: -15.0f32, y: 0.0f32, z: 0.0f32,}; init };
    let mut sp3E: s16 = 0;
    let mut sp3C: s16 = 0;
    match limbIndex {
        3 => {
            sp70.x =
                ((*this).timer2 as libc::c_int & 7 as libc::c_int) as
                    libc::c_float * 30.0f32;
            Matrix_MultVec3f(&mut sp70,
                             &mut *(*this).effectPos.as_mut_ptr().offset(0 as
                                                                             libc::c_int
                                                                             as
                                                                             isize));
        }
        4 => {
            Matrix_MultVec3f(&mut sZeroVec, &mut (*this).armTip);
            sp70.x =
                ((*this).timer2 as libc::c_int & 7 as libc::c_int) as
                    libc::c_float * 30.0f32;
            Matrix_MultVec3f(&mut sp70,
                             &mut *(*this).effectPos.as_mut_ptr().offset(1 as
                                                                             libc::c_int
                                                                             as
                                                                             isize));
        }
        5 => {
            Matrix_MultVec3f(&mut sZeroVec, &mut (*this).zapNeckPos);
            sp70.x =
                ((*this).timer2 as libc::c_int & 7 as libc::c_int) as
                    libc::c_float * 46.0f32;
            Matrix_MultVec3f(&mut sp70,
                             &mut *(*this).effectPos.as_mut_ptr().offset(2 as
                                                                             libc::c_int
                                                                             as
                                                                             isize));
        }
        7 => {
            Matrix_MultVec3f(&mut sZeroVec, &mut (*this).zapHeadPos);
            sp70.x =
                ((*this).timer2 as libc::c_int & 7 as libc::c_int) as
                    libc::c_float * 46.0f32;
            Matrix_MultVec3f(&mut sp70,
                             &mut *(*this).effectPos.as_mut_ptr().offset(3 as
                                                                             libc::c_int
                                                                             as
                                                                             isize));
            sp70.x = 20.0f32;
            Matrix_MultVec3f(&mut sp70,
                             &mut *(*this).effectPos.as_mut_ptr().offset(9 as
                                                                             libc::c_int
                                                                             as
                                                                             isize));
            func_80035844(&mut *(*this).effectPos.as_mut_ptr().offset(9 as
                                                                          libc::c_int
                                                                          as
                                                                          isize),
                          &mut (*this).unk_1D8, &mut (*this).headRot,
                          0 as libc::c_int);
            sp3E = (*this).headRot.x;
            sp3C = (*this).headRot.y;
            Matrix_Push();
            Matrix_Translate((*this).effectPos[9 as libc::c_int as usize].x,
                             (*this).effectPos[9 as libc::c_int as usize].y,
                             (*this).effectPos[9 as libc::c_int as usize].z,
                             MTXMODE_NEW as libc::c_int as u8_0);
            Matrix_RotateZYX(sp3E, sp3C, 0 as libc::c_int as s16,
                             MTXMODE_APPLY as libc::c_int as u8_0);
            sp70.x = 0.0f32;
            if sFightPhase as libc::c_int >= 15 as libc::c_int {
                sp70.z =
                    ((*this).timer2 as libc::c_int - 16 as libc::c_int &
                         7 as libc::c_int) as libc::c_float * 120.0f32
            } else {
                sp70.z =
                    ((*this).timer2 as libc::c_int - 32 as libc::c_int &
                         0xf as libc::c_int) as libc::c_float * 80.0f32
            }
            sp70.z += 40.0f32;
            sp40.z = sp70.z;
            sp4C.z = sp40.z;
            sp70.z += 50.0f32;
            Matrix_MultVec3f(&mut sp70,
                             &mut *(*this).effectPos.as_mut_ptr().offset(4 as
                                                                             libc::c_int
                                                                             as
                                                                             isize));
            if sFightPhase as libc::c_int >= 15 as libc::c_int {
                sp70.z -= 33.0f32;
                if sp70.z < 0.0f32 { sp70.z = 0.0f32 }
                Matrix_MultVec3f(&mut sp70,
                                 &mut *(*this).effectPos.as_mut_ptr().offset(6
                                                                                 as
                                                                                 libc::c_int
                                                                                 as
                                                                                 isize));
                sp70.z -= 33.0f32;
                if sp70.z < 0.0f32 { sp70.z = 0.0f32 }
            } else {
                sp70.z -= 22.0f32;
                if sp70.z < 0.0f32 { sp70.z = 0.0f32 }
                Matrix_MultVec3f(&mut sp70,
                                 &mut *(*this).effectPos.as_mut_ptr().offset(6
                                                                                 as
                                                                                 libc::c_int
                                                                                 as
                                                                                 isize));
                sp70.z -= 22.0f32;
                if sp70.z < 0.0f32 { sp70.z = 0.0f32 }
            }
            Matrix_MultVec3f(&mut sp70,
                             &mut *(*this).effectPos.as_mut_ptr().offset(5 as
                                                                             libc::c_int
                                                                             as
                                                                             isize));
            Matrix_MultVec3f(&mut sp64,
                             &mut *(*this).colliderLightning.dim.quad.as_mut_ptr().offset(1
                                                                                              as
                                                                                              libc::c_int
                                                                                              as
                                                                                              isize));
            Matrix_MultVec3f(&mut sp58,
                             &mut *(*this).colliderLightning.dim.quad.as_mut_ptr().offset(0
                                                                                              as
                                                                                              libc::c_int
                                                                                              as
                                                                                              isize));
            Matrix_MultVec3f(&mut sp4C,
                             &mut *(*this).colliderLightning.dim.quad.as_mut_ptr().offset(3
                                                                                              as
                                                                                              libc::c_int
                                                                                              as
                                                                                              isize));
            Matrix_MultVec3f(&mut sp40,
                             &mut *(*this).colliderLightning.dim.quad.as_mut_ptr().offset(2
                                                                                              as
                                                                                              libc::c_int
                                                                                              as
                                                                                              isize));
            Collider_SetQuadVertices(&mut (*this).colliderLightning,
                                     &mut *(*this).colliderLightning.dim.quad.as_mut_ptr().offset(0
                                                                                                      as
                                                                                                      libc::c_int
                                                                                                      as
                                                                                                      isize),
                                     &mut *(*this).colliderLightning.dim.quad.as_mut_ptr().offset(1
                                                                                                      as
                                                                                                      libc::c_int
                                                                                                      as
                                                                                                      isize),
                                     &mut *(*this).colliderLightning.dim.quad.as_mut_ptr().offset(2
                                                                                                      as
                                                                                                      libc::c_int
                                                                                                      as
                                                                                                      isize),
                                     &mut *(*this).colliderLightning.dim.quad.as_mut_ptr().offset(3
                                                                                                      as
                                                                                                      libc::c_int
                                                                                                      as
                                                                                                      isize));
            Matrix_Pop();
        }
        _ => { }
    };
}
#[no_mangle]
pub unsafe extern "C" fn BossVa_BariOverrideLimbDraw(mut globalCtx:
                                                         *mut GlobalContext,
                                                     mut limbIndex: s32,
                                                     mut dList: *mut *mut Gfx,
                                                     mut pos: *mut Vec3f,
                                                     mut rot: *mut Vec3s,
                                                     mut thisx:
                                                         *mut libc::c_void)
 -> s32 {
    let mut this: *mut BossVa = thisx as *mut BossVa;
    match limbIndex {
        2 => { *dList = 0 as *mut Gfx }
        3 => {
            Matrix_Scale((*this).unk_1D8.x, 1.0f32, (*this).unk_1D8.x,
                         MTXMODE_APPLY as libc::c_int as u8_0);
        }
        4 => {
            Matrix_Scale(1.0f32, (*this).unk_1D8.y, 1.0f32,
                         MTXMODE_APPLY as libc::c_int as u8_0);
        }
        _ => { }
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn BossVa_BariPostLimbDraw(mut globalCtx:
                                                     *mut GlobalContext,
                                                 mut limbIndex: s32,
                                                 mut dList: *mut *mut Gfx,
                                                 mut rot: *mut Vec3s,
                                                 mut thisx:
                                                     *mut libc::c_void) {
    let mut this: *mut BossVa = thisx as *mut BossVa;
    let mut __gfxCtx: *mut GraphicsContext = 0 as *mut GraphicsContext;
    let mut dispRefs: [*mut Gfx; 4] = [0 as *mut Gfx; 4];
    __gfxCtx = (*globalCtx).state.gfxCtx;
    Graph_OpenDisps(dispRefs.as_mut_ptr(), (*globalCtx).state.gfxCtx,
                    b"../z_boss_va.c\x00" as *const u8 as *const libc::c_char,
                    4494 as libc::c_int);
    if limbIndex == 2 as libc::c_int {
        let fresh18 = (*__gfxCtx).polyXlu.p;
        (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
        let mut _g: *mut Gfx = fresh18;
        (*_g).words.w0 =
            (0xdb as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 24 as libc::c_int
                |
                (0x6 as libc::c_int as u32_0 &
                     (((0x1 as libc::c_int) << 8 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    16 as libc::c_int |
                ((0xa as libc::c_int * 4 as libc::c_int) as u32_0 &
                     (((0x1 as libc::c_int) << 16 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    0 as libc::c_int;
        (*_g).words.w1 =
            Gfx_TwoTexScroll((*globalCtx).state.gfxCtx, 0 as libc::c_int,
                             0 as libc::c_int as u32_0,
                             (*globalCtx).gameplayFrames.wrapping_mul(10 as
                                                                          libc::c_int
                                                                          as
                                                                          libc::c_uint).wrapping_rem(32
                                                                                                         as
                                                                                                         libc::c_int
                                                                                                         as
                                                                                                         libc::c_uint),
                             16 as libc::c_int, 32 as libc::c_int,
                             1 as libc::c_int, 0 as libc::c_int as u32_0,
                             (*globalCtx).gameplayFrames.wrapping_mul(-(5 as
                                                                            libc::c_int)
                                                                          as
                                                                          libc::c_uint).wrapping_rem(32
                                                                                                         as
                                                                                                         libc::c_int
                                                                                                         as
                                                                                                         libc::c_uint),
                             16 as libc::c_int, 32 as libc::c_int) as
                libc::c_uint;
        let fresh19 = (*__gfxCtx).polyXlu.p;
        (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
        let mut _g_0: *mut Gfx = fresh19;
        (*_g_0).words.w0 =
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
        (*_g_0).words.w1 =
            Matrix_NewMtx((*globalCtx).state.gfxCtx,
                          b"../z_boss_va.c\x00" as *const u8 as
                              *const libc::c_char as *mut libc::c_char,
                          4508 as libc::c_int) as libc::c_uint;
        let fresh20 = (*__gfxCtx).polyXlu.p;
        (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
        let mut _g_1: *mut Gfx = fresh20;
        (*_g_1).words.w0 =
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
        (*_g_1).words.w1 = gBarinadeDL_000FA0.as_mut_ptr() as libc::c_uint
    } else if limbIndex == 3 as libc::c_int || limbIndex == 4 as libc::c_int {
        let fresh21 = (*__gfxCtx).polyXlu.p;
        (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
        let mut _g_2: *mut Gfx = fresh21;
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
                (((0 as libc::c_int | 0x2 as libc::c_int | 0 as libc::c_int) ^
                      0x1 as libc::c_int) as u32_0 &
                     (((0x1 as libc::c_int) << 8 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    0 as libc::c_int;
        (*_g_2).words.w1 =
            Matrix_NewMtx((*globalCtx).state.gfxCtx,
                          b"../z_boss_va.c\x00" as *const u8 as
                              *const libc::c_char as *mut libc::c_char,
                          4512 as libc::c_int) as libc::c_uint;
        let fresh22 = (*__gfxCtx).polyXlu.p;
        (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
        let mut _g_3: *mut Gfx = fresh22;
        (*_g_3).words.w0 =
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
        (*_g_3).words.w1 = *dList as libc::c_uint
    }
    Graph_CloseDisps(dispRefs.as_mut_ptr(), (*globalCtx).state.gfxCtx,
                     b"../z_boss_va.c\x00" as *const u8 as
                         *const libc::c_char, 4517 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn BossVa_Draw(mut thisx: *mut Actor,
                                     mut globalCtx: *mut GlobalContext) {
    let mut paramsPtr: *mut s16 = 0 as *mut s16;
    let mut this: *mut BossVa = thisx as *mut BossVa;
    let mut spBC: Vec3f = Vec3f{x: 0., y: 0., z: 0.,};
    let mut spB0: Vec3f =
        { let mut init = Vec3f{x: 0.0f32, y: 45.0f32, z: 0.0f32,}; init };
    let mut spA4: Vec3f =
        { let mut init = Vec3f{x: 0.4f32, y: 0.4f32, z: 0.4f32,}; init };
    let mut sp98: Vec3f =
        { let mut init = Vec3f{x: 15.0f32, y: 40.0f32, z: 0.0f32,}; init };
    let mut sp8C: Vec3f =
        { let mut init = Vec3f{x: -15.0f32, y: 40.0f32, z: 0.0f32,}; init };
    let mut sp80: Vec3f =
        { let mut init = Vec3f{x: 15.0f32, y: 40.0f32, z: 0.0f32,}; init };
    let mut sp74: Vec3f =
        { let mut init = Vec3f{x: -15.0f32, y: 40.0f32, z: 0.0f32,}; init };
    let mut unused: Color_RGBA8 =
        {
            let mut init =
                Color_RGBA8{r: 250 as libc::c_int as u8_0,
                            g: 250 as libc::c_int as u8_0,
                            b: 230 as libc::c_int as u8_0,
                            a: 200 as libc::c_int as u8_0,};
            init
        };
    let mut __gfxCtx: *mut GraphicsContext = 0 as *mut GraphicsContext;
    let mut dispRefs: [*mut Gfx; 4] = [0 as *mut Gfx; 4];
    __gfxCtx = (*globalCtx).state.gfxCtx;
    Graph_OpenDisps(dispRefs.as_mut_ptr(), (*globalCtx).state.gfxCtx,
                    b"../z_boss_va.c\x00" as *const u8 as *const libc::c_char,
                    4542 as libc::c_int);
    func_80093D18((*globalCtx).state.gfxCtx);
    paramsPtr = &mut (*this).actor.params;
    func_80093D84((*globalCtx).state.gfxCtx);
    match (*this).actor.params as libc::c_int {
        -1 => {
            if (*globalCtx).envCtx.adjFogNear as libc::c_int !=
                   0 as libc::c_int {
                (*globalCtx).envCtx.adjFogNear =
                    ((*globalCtx).envCtx.adjFogNear as libc::c_int +
                         0x15e as libc::c_int) as s16;
                if (*globalCtx).envCtx.adjFogNear as libc::c_int >
                       0 as libc::c_int {
                    (*globalCtx).envCtx.adjFogNear = 0 as libc::c_int as s16
                }
            }
            if (*globalCtx).envCtx.adjFogFar as libc::c_int !=
                   0 as libc::c_int {
                (*globalCtx).envCtx.adjFogFar =
                    ((*globalCtx).envCtx.adjFogFar as libc::c_int +
                         0x15e as libc::c_int) as s16;
                if (*globalCtx).envCtx.adjFogFar as libc::c_int >
                       0 as libc::c_int {
                    (*globalCtx).envCtx.adjFogFar = 0 as libc::c_int as s16
                }
            }
            if (*this).isDead == 0 {
                let fresh23 = (*__gfxCtx).polyOpa.p;
                (*__gfxCtx).polyOpa.p = (*__gfxCtx).polyOpa.p.offset(1);
                let mut _g: *mut Gfx = fresh23;
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
                    Gfx_TwoTexScroll((*globalCtx).state.gfxCtx,
                                     0 as libc::c_int,
                                     0 as libc::c_int as u32_0,
                                     0 as libc::c_int as u32_0,
                                     8 as libc::c_int, 16 as libc::c_int,
                                     1 as libc::c_int,
                                     0 as libc::c_int as u32_0,
                                     (*globalCtx).gameplayFrames.wrapping_mul(-(10
                                                                                    as
                                                                                    libc::c_int)
                                                                                  as
                                                                                  libc::c_uint).wrapping_rem(16
                                                                                                                 as
                                                                                                                 libc::c_int
                                                                                                                 as
                                                                                                                 libc::c_uint),
                                     16 as libc::c_int, 16 as libc::c_int) as
                        libc::c_uint;
                let fresh24 = (*__gfxCtx).polyOpa.p;
                (*__gfxCtx).polyOpa.p = (*__gfxCtx).polyOpa.p.offset(1);
                let mut _g_0: *mut Gfx = fresh24;
                (*_g_0).words.w0 =
                    (0xdb as libc::c_int as u32_0 &
                         (((0x1 as libc::c_int) << 8 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        24 as libc::c_int |
                        (0x6 as libc::c_int as u32_0 &
                             (((0x1 as libc::c_int) << 8 as libc::c_int) -
                                  1 as libc::c_int) as libc::c_uint) <<
                            16 as libc::c_int |
                        ((0x9 as libc::c_int * 4 as libc::c_int) as u32_0 &
                             (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                  1 as libc::c_int) as libc::c_uint) <<
                            0 as libc::c_int;
                (*_g_0).words.w1 =
                    Gfx_TwoTexScroll((*globalCtx).state.gfxCtx,
                                     0 as libc::c_int,
                                     0 as libc::c_int as u32_0,
                                     (*globalCtx).gameplayFrames.wrapping_mul(-(10
                                                                                    as
                                                                                    libc::c_int)
                                                                                  as
                                                                                  libc::c_uint).wrapping_rem(32
                                                                                                                 as
                                                                                                                 libc::c_int
                                                                                                                 as
                                                                                                                 libc::c_uint),
                                     16 as libc::c_int, 0x20 as libc::c_int,
                                     1 as libc::c_int,
                                     0 as libc::c_int as u32_0,
                                     (*globalCtx).gameplayFrames.wrapping_mul(-(5
                                                                                    as
                                                                                    libc::c_int)
                                                                                  as
                                                                                  libc::c_uint).wrapping_rem(32
                                                                                                                 as
                                                                                                                 libc::c_int
                                                                                                                 as
                                                                                                                 libc::c_uint),
                                     16 as libc::c_int, 32 as libc::c_int) as
                        libc::c_uint;
                SkelAnime_DrawOpa(globalCtx, (*this).skelAnime.skeleton,
                                  (*this).skelAnime.jointTable,
                                  Some(BossVa_BodyOverrideLimbDraw as
                                           unsafe extern "C" fn(_:
                                                                    *mut GlobalContext,
                                                                _: s32,
                                                                _:
                                                                    *mut *mut Gfx,
                                                                _: *mut Vec3f,
                                                                _: *mut Vec3s,
                                                                _:
                                                                    *mut libc::c_void)
                                               -> s32),
                                  Some(BossVa_BodyPostLimbDraw as
                                           unsafe extern "C" fn(_:
                                                                    *mut GlobalContext,
                                                                _: s32,
                                                                _:
                                                                    *mut *mut Gfx,
                                                                _: *mut Vec3s,
                                                                _:
                                                                    *mut libc::c_void)
                                               -> ()),
                                  this as *mut libc::c_void);
            }
        }
        0 | 1 | 2 => {
            if (*this).isDead == 0 {
                SkelAnime_DrawFlexOpa(globalCtx, (*this).skelAnime.skeleton,
                                      (*this).skelAnime.jointTable,
                                      (*this).skelAnime.dListCount as s32,
                                      Some(BossVa_SupportOverrideLimbDraw as
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
                                                                        *mut libc::c_void)
                                                   -> s32),
                                      Some(BossVa_SupportPostLimbDraw as
                                               unsafe extern "C" fn(_:
                                                                        *mut GlobalContext,
                                                                    _: s32,
                                                                    _:
                                                                        *mut *mut Gfx,
                                                                    _:
                                                                        *mut Vec3s,
                                                                    _:
                                                                        *mut libc::c_void)
                                                   -> ()),
                                      this as *mut libc::c_void);
            }
        }
        3 | 4 | 5 => {
            if (*this).isDead == 0 {
                SkelAnime_DrawFlexOpa(globalCtx, (*this).skelAnime.skeleton,
                                      (*this).skelAnime.jointTable,
                                      (*this).skelAnime.dListCount as s32,
                                      Some(BossVa_ZapperOverrideLimbDraw as
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
                                                                        *mut libc::c_void)
                                                   -> s32),
                                      Some(BossVa_ZapperPostLimbDraw as
                                               unsafe extern "C" fn(_:
                                                                        *mut GlobalContext,
                                                                    _: s32,
                                                                    _:
                                                                        *mut *mut Gfx,
                                                                    _:
                                                                        *mut Vec3s,
                                                                    _:
                                                                        *mut libc::c_void)
                                                   -> ()),
                                      this as *mut libc::c_void);
            }
        }
        16 | 17 | 18 => {
            SkelAnime_DrawFlexOpa(globalCtx, (*this).skelAnime.skeleton,
                                  (*this).skelAnime.jointTable,
                                  (*this).skelAnime.dListCount as s32, None,
                                  None, 0 as *mut libc::c_void);
        }
        19 => { }
        _ => {
            if (*this).isDead == 0 {
                SkelAnime_DrawOpa(globalCtx, (*this).skelAnime.skeleton,
                                  (*this).skelAnime.jointTable,
                                  Some(BossVa_BariOverrideLimbDraw as
                                           unsafe extern "C" fn(_:
                                                                    *mut GlobalContext,
                                                                _: s32,
                                                                _:
                                                                    *mut *mut Gfx,
                                                                _: *mut Vec3f,
                                                                _: *mut Vec3s,
                                                                _:
                                                                    *mut libc::c_void)
                                               -> s32),
                                  Some(BossVa_BariPostLimbDraw as
                                           unsafe extern "C" fn(_:
                                                                    *mut GlobalContext,
                                                                _: s32,
                                                                _:
                                                                    *mut *mut Gfx,
                                                                _: *mut Vec3s,
                                                                _:
                                                                    *mut libc::c_void)
                                               -> ()),
                                  this as *mut libc::c_void);
                Collider_UpdateSpheres(0 as libc::c_int,
                                       &mut (*this).colliderSph);
                if (sCsState as libc::c_int) < BOSSVA_BATTLE as libc::c_int {
                    spBC =
                        (*((*this).actor.parent as
                               *mut BossVa)).actor.world.pos
                } else {
                    spBC = (*((*this).actor.parent as *mut BossVa)).unk_1D8
                }
                Matrix_MultVec3f(&mut sZeroVec,
                                 &mut *(*this).effectPos.as_mut_ptr().offset(1
                                                                                 as
                                                                                 libc::c_int
                                                                                 as
                                                                                 isize));
                Matrix_Push();
                Matrix_Translate(spBC.x, spBC.y, spBC.z,
                                 MTXMODE_NEW as libc::c_int as u8_0);
                Matrix_RotateZYX((*this).actor.world.rot.x,
                                 (*this).actor.world.rot.y,
                                 0 as libc::c_int as s16,
                                 MTXMODE_APPLY as libc::c_int as u8_0);
                sp74.z = (*this).unk_1A0;
                sp80.z = sp74.z;
                spB0.z =
                    ((*this).timer2 as libc::c_int & 0xf as libc::c_int) as
                        libc::c_float * ((*this).unk_1A0 * 0.0625f32);
                Matrix_MultVec3f(&mut spB0,
                                 &mut *(*this).effectPos.as_mut_ptr().offset(0
                                                                                 as
                                                                                 libc::c_int
                                                                                 as
                                                                                 isize));
                Matrix_MultVec3f(&mut sp98,
                                 &mut *(*this).colliderLightning.dim.quad.as_mut_ptr().offset(1
                                                                                                  as
                                                                                                  libc::c_int
                                                                                                  as
                                                                                                  isize));
                Matrix_MultVec3f(&mut sp8C,
                                 &mut *(*this).colliderLightning.dim.quad.as_mut_ptr().offset(0
                                                                                                  as
                                                                                                  libc::c_int
                                                                                                  as
                                                                                                  isize));
                Matrix_MultVec3f(&mut sp80,
                                 &mut *(*this).colliderLightning.dim.quad.as_mut_ptr().offset(3
                                                                                                  as
                                                                                                  libc::c_int
                                                                                                  as
                                                                                                  isize));
                Matrix_MultVec3f(&mut sp74,
                                 &mut *(*this).colliderLightning.dim.quad.as_mut_ptr().offset(2
                                                                                                  as
                                                                                                  libc::c_int
                                                                                                  as
                                                                                                  isize));
                Collider_SetQuadVertices(&mut (*this).colliderLightning,
                                         &mut *(*this).colliderLightning.dim.quad.as_mut_ptr().offset(0
                                                                                                          as
                                                                                                          libc::c_int
                                                                                                          as
                                                                                                          isize),
                                         &mut *(*this).colliderLightning.dim.quad.as_mut_ptr().offset(1
                                                                                                          as
                                                                                                          libc::c_int
                                                                                                          as
                                                                                                          isize),
                                         &mut *(*this).colliderLightning.dim.quad.as_mut_ptr().offset(2
                                                                                                          as
                                                                                                          libc::c_int
                                                                                                          as
                                                                                                          isize),
                                         &mut *(*this).colliderLightning.dim.quad.as_mut_ptr().offset(3
                                                                                                          as
                                                                                                          libc::c_int
                                                                                                          as
                                                                                                          isize));
                Matrix_Pop();
                spBC = (*this).actor.world.pos;
                spBC.y += 9.0f32;
                if (*this).actor.colorFilterTimer as libc::c_int !=
                       0 as libc::c_int {
                    func_80026A6C(globalCtx);
                }
                func_80033C30(&mut spBC, &mut spA4,
                              0xff as libc::c_int as u8_0, globalCtx);
                if (*this).actor.colorFilterTimer as libc::c_int !=
                       0 as libc::c_int {
                    let mut blue: Color_RGBA8 =
                        {
                            let mut init =
                                Color_RGBA8{r: 0 as libc::c_int as u8_0,
                                            g: 0 as libc::c_int as u8_0,
                                            b: 255 as libc::c_int as u8_0,
                                            a: 255 as libc::c_int as u8_0,};
                            init
                        };
                    func_80026860(globalCtx, &mut blue,
                                  (*this).actor.colorFilterTimer as s16,
                                  ((*this).actor.colorFilterParams as
                                       libc::c_int & 0xff as libc::c_int) as
                                      s16);
                }
            }
        }
    }
    if *paramsPtr as libc::c_int == BOSSVA_BODY as libc::c_int {
        BossVa_DrawEffects(sVaEffects.as_mut_ptr(), globalCtx);
    } else if *paramsPtr as libc::c_int == BOSSVA_DOOR as libc::c_int {
        BossVa_DrawDoor(globalCtx, sDoorState);
    }
    Graph_CloseDisps(dispRefs.as_mut_ptr(), (*globalCtx).state.gfxCtx,
                     b"../z_boss_va.c\x00" as *const u8 as
                         *const libc::c_char, 4673 as libc::c_int);
}
// Unreferenced? Possibly a color
#[no_mangle]
pub unsafe extern "C" fn BossVa_UpdateEffects(mut globalCtx:
                                                  *mut GlobalContext) {
    let mut effect: *mut BossVaEffect = sVaEffects.as_mut_ptr();
    let mut player: *mut Player =
        (*globalCtx).actorCtx.actorLists[ACTORCAT_PLAYER as libc::c_int as
                                             usize].head as *mut Player;
    let mut spB6: s16 = 0;
    let mut i: s16 = 0;
    let mut floorY: f32_0 = 0.;
    let mut padAC: s32 = 0;
    let mut pitch: s16 = 0;
    let mut yaw: s16 = 0;
    let mut refActor2: *mut BossVa = 0 as *mut BossVa;
    let mut refActor: *mut BossVa = 0 as *mut BossVa;
    let mut sp94: Vec3f = Vec3f{x: 0., y: 0., z: 0.,};
    let mut sp90: *mut CollisionPoly = 0 as *mut CollisionPoly;
    let mut pad8C: f32_0 = 0.;
    let mut sp80: Vec3f = Vec3f{x: 0., y: 0., z: 0.,};
    let mut sp7C: *mut CollisionPoly = 0 as *mut CollisionPoly;
    let mut pad78: f32_0 = 0.;
    let mut pad74: f32_0 = 0.;
    i = 0 as libc::c_int as s16;
    while (i as libc::c_int) <
              (::std::mem::size_of::<[BossVaEffect; 400]>() as
                   libc::c_ulong).wrapping_div(::std::mem::size_of::<BossVaEffect>()
                                                   as libc::c_ulong) as s32 {
        if (*effect).type_0 as libc::c_int != VA_NONE as libc::c_int {
            (*effect).timer = (*effect).timer.wrapping_sub(1);
            (*effect).pos.x += (*effect).velocity.x;
            (*effect).pos.y += (*effect).velocity.y;
            (*effect).pos.z += (*effect).velocity.z;
            (*effect).velocity.x += (*effect).accel.x;
            (*effect).velocity.y += (*effect).accel.y;
            (*effect).velocity.z += (*effect).accel.z;
            if (*effect).type_0 as libc::c_int ==
                   VA_LARGE_SPARK as libc::c_int ||
                   (*effect).type_0 as libc::c_int ==
                       VA_SMALL_SPARK as libc::c_int {
                refActor = (*effect).parent;
                (*effect).rot.z =
                    ((*effect).rot.z as libc::c_int +
                         ((Rand_ZeroOne() *
                               0x4e20 as libc::c_int as libc::c_float) as s16
                              as libc::c_int + 0x2000 as libc::c_int)) as s16;
                (*effect).rot.y =
                    ((*effect).rot.y as libc::c_int +
                         ((Rand_ZeroOne() *
                               0x2710 as libc::c_int as libc::c_float) as s16
                              as libc::c_int + 0x2000 as libc::c_int)) as s16;
                if (*effect).mode as libc::c_int ==
                       SPARK_TETHER as libc::c_int ||
                       (*effect).mode as libc::c_int ==
                           SPARK_UNUSED as libc::c_int {
                    pitch =
                        ((*effect).rot.x as libc::c_int -
                             Math_Vec3f_Pitch(&mut (*refActor).actor.world.pos,
                                              &mut (*((*refActor).actor.parent
                                                          as
                                                          *mut BossVa)).unk_1D8)
                                 as libc::c_int) as s16;
                    pad8C = Math_SinS((*refActor).actor.world.rot.y);
                    (*effect).pos.x =
                        (*refActor).actor.world.pos.x -
                            (*effect).offset.x * pad8C;
                    pad74 = Math_CosS((*refActor).actor.world.rot.y);
                    (*effect).pos.z =
                        (*refActor).actor.world.pos.z -
                            (*effect).offset.x * pad74;
                    pad78 = Math_CosS(-(pitch as libc::c_int) as s16);
                    (*effect).pos.y =
                        (*effect).offset.y * pad78 +
                            (*refActor).actor.world.pos.y
                } else if (*effect).mode as libc::c_int ==
                              SPARK_BARI as libc::c_int ||
                              (*effect).mode as libc::c_int ==
                                  SPARK_BODY as libc::c_int {
                    (*effect).pos.x =
                        (*effect).offset.x + (*refActor).actor.world.pos.x;
                    (*effect).pos.y =
                        (*effect).offset.y + (*refActor).actor.world.pos.y;
                    (*effect).pos.z =
                        (*effect).offset.z + (*refActor).actor.world.pos.z
                } else {
                    spB6 =
                        Rand_ZeroFloat((::std::mem::size_of::<[Vec3f; 18]>()
                                            as
                                            libc::c_ulong).wrapping_div(::std::mem::size_of::<Vec3f>()
                                                                            as
                                                                            libc::c_ulong)
                                           as s32 as libc::c_float - 0.1f32)
                            as s16;
                    (*effect).pos.x =
                        (*player).bodyPartsPos[spB6 as usize].x +
                            Rand_CenteredFloat(10.0f32);
                    (*effect).pos.y =
                        (*player).bodyPartsPos[spB6 as usize].y +
                            Rand_CenteredFloat(15.0f32);
                    (*effect).pos.z =
                        (*player).bodyPartsPos[spB6 as usize].z +
                            Rand_CenteredFloat(10.0f32)
                }
                if ((*effect).timer as libc::c_int) < 100 as libc::c_int {
                    (*effect).primColor[3 as libc::c_int as usize] =
                        ((*effect).primColor[3 as libc::c_int as usize] as
                             libc::c_int - 50 as libc::c_int) as s16;
                    if ((*effect).primColor[3 as libc::c_int as usize] as
                            libc::c_int) < 0 as libc::c_int {
                        (*effect).primColor[3 as libc::c_int as usize] =
                            0 as libc::c_int as s16;
                        (*effect).timer = 0 as libc::c_int as u16_0;
                        (*effect).type_0 = VA_NONE as libc::c_int as u8_0
                    }
                }
            }
            if (*effect).type_0 as libc::c_int ==
                   VA_BLAST_SPARK as libc::c_int {
                (*effect).rot.z =
                    ((*effect).rot.z as libc::c_int +
                         ((Rand_ZeroOne() *
                               0x4e20 as libc::c_int as libc::c_float) as s16
                              as libc::c_int + 0x4000 as libc::c_int)) as s16;
                if ((*effect).timer as libc::c_int) < 100 as libc::c_int {
                    (*effect).primColor[3 as libc::c_int as usize] =
                        ((*effect).primColor[3 as libc::c_int as usize] as
                             libc::c_int - 50 as libc::c_int) as s16;
                    if ((*effect).primColor[3 as libc::c_int as usize] as
                            libc::c_int) < 0 as libc::c_int {
                        (*effect).primColor[3 as libc::c_int as usize] =
                            0 as libc::c_int as s16;
                        (*effect).timer = 0 as libc::c_int as u16_0;
                        (*effect).type_0 = VA_NONE as libc::c_int as u8_0
                    }
                }
            }
            if (*effect).type_0 as libc::c_int == VA_SPARK_BALL as libc::c_int
               {
                refActor2 = (*effect).parent;
                (*effect).rot.z =
                    ((*effect).rot.z as libc::c_int +
                         ((Rand_ZeroOne() *
                               0x2710 as libc::c_int as libc::c_float) as s16
                              as libc::c_int + 0x24a8 as libc::c_int)) as s16;
                (*effect).pos.x =
                    (*effect).offset.x + (*refActor2).actor.world.pos.x;
                (*effect).pos.y =
                    (*refActor2).actor.world.pos.y + 310.0f32 +
                        (*refActor2).actor.shape.yOffset *
                            (*refActor2).actor.scale.y;
                (*effect).pos.z =
                    (*effect).offset.z + (*refActor2).actor.world.pos.z;
                (*effect).mode =
                    ((*effect).mode as libc::c_int + 1 as libc::c_int &
                         7 as libc::c_int) as s16;
                if ((*effect).timer as libc::c_int) < 100 as libc::c_int {
                    (*effect).primColor[3 as libc::c_int as usize] =
                        ((*effect).primColor[3 as libc::c_int as usize] as
                             libc::c_int - 50 as libc::c_int) as s16;
                    if ((*effect).primColor[3 as libc::c_int as usize] as
                            libc::c_int) < 0 as libc::c_int {
                        (*effect).primColor[3 as libc::c_int as usize] =
                            0 as libc::c_int as s16;
                        (*effect).timer = 0 as libc::c_int as u16_0;
                        (*effect).type_0 = VA_NONE as libc::c_int as u8_0
                    }
                }
            }
            if (*effect).type_0 as libc::c_int == VA_ZAP_CHARGE as libc::c_int
               {
                (*effect).mode =
                    ((*effect).mode as libc::c_int + 1 as libc::c_int &
                         7 as libc::c_int) as s16;
                (*effect).primColor[3 as libc::c_int as usize] =
                    ((*effect).primColor[3 as libc::c_int as usize] as
                         libc::c_int - 20 as libc::c_int) as s16;
                if (*effect).primColor[3 as libc::c_int as usize] as
                       libc::c_int <= 0 as libc::c_int {
                    (*effect).primColor[3 as libc::c_int as usize] =
                        0 as libc::c_int as s16;
                    (*effect).timer = 0 as libc::c_int as u16_0;
                    (*effect).type_0 = VA_NONE as libc::c_int as u8_0
                }
            }
            if (*effect).type_0 as libc::c_int == VA_BLOOD as libc::c_int {
                if ((*effect).mode as libc::c_int) < BLOOD_SPOT as libc::c_int
                   {
                    sp94 = (*effect).pos;
                    sp94.y -= (*effect).velocity.y + 4.0f32;
                    floorY =
                        BgCheck_EntityRaycastFloor1(&mut (*globalCtx).colCtx,
                                                    &mut sp90, &mut sp94);
                    if !sp90.is_null() && (*effect).pos.y <= floorY {
                        (*effect).mode = BLOOD_SPOT as libc::c_int as s16;
                        (*effect).pos.y = floorY + 1.0f32;
                        if sCsState as libc::c_int <=
                               DEATH_SHELL_BURST as libc::c_int {
                            (*effect).timer = 80 as libc::c_int as u16_0
                        } else {
                            (*effect).timer = 60000 as libc::c_int as u16_0
                        }
                        (*effect).velocity = sZeroVec;
                        (*effect).accel = (*effect).velocity
                    }
                    if (*effect).timer == 0 {
                        (*effect).type_0 = VA_NONE as libc::c_int as u8_0
                    }
                } else if ((*effect).timer as libc::c_int) < 20 as libc::c_int
                 {
                    (*effect).envColor[3 as libc::c_int as usize] =
                        ((*effect).timer as libc::c_int * 5 as libc::c_int) as
                            s16;
                    (*effect).primColor[3 as libc::c_int as usize] =
                        ((*effect).timer as libc::c_int * 10 as libc::c_int)
                            as s16
                } else if (*effect).timer as libc::c_int >
                              50000 as libc::c_int {
                    (*effect).timer = (*effect).timer.wrapping_add(1)
                }
                if (*effect).timer == 0 {
                    (*effect).type_0 = VA_NONE as libc::c_int as u8_0
                }
            }
            if (*effect).type_0 as libc::c_int == VA_GORE as libc::c_int {
                if (*effect).mode as libc::c_int ==
                       GORE_PERMANENT as libc::c_int {
                    sp80 = (*effect).pos;
                    sp80.y -= (*effect).velocity.y + 4.0f32;
                    (*effect).rot.x =
                        ((*effect).rot.x as libc::c_int +
                             0x1770 as libc::c_int) as s16;
                    floorY =
                        BgCheck_EntityRaycastFloor1(&mut (*globalCtx).colCtx,
                                                    &mut sp7C, &mut sp80);
                    if !sp7C.is_null() && (*effect).pos.y <= floorY {
                        (*effect).mode = GORE_FLOOR as libc::c_int as s16;
                        (*effect).timer = 30 as libc::c_int as u16_0;
                        (*effect).pos.y = floorY + 1.0f32;
                        (*effect).velocity = sZeroVec;
                        (*effect).accel = (*effect).velocity;
                        (*effect).rot.x = -(0x4000 as libc::c_int) as s16
                    }
                    if (*effect).timer == 0 {
                        (*effect).type_0 = VA_NONE as libc::c_int as u8_0
                    }
                } else if (*effect).mode as libc::c_int ==
                              GORE_FADING as libc::c_int {
                    if (*effect).timer as libc::c_int == 0 as libc::c_int {
                        (*effect).type_0 = VA_NONE as libc::c_int as u8_0
                    }
                } else {
                    Math_SmoothStepToF(&mut (*effect).scaleMod, 0.075f32,
                                       1.0f32, 0.005f32, 0.0f32);
                    Math_SmoothStepToF(&mut (*effect).offset.y, 0.0f32,
                                       0.6f32, 0.005f32, 0.0013f32);
                    if (*globalCtx).gameplayFrames.wrapping_rem(4 as
                                                                    libc::c_int
                                                                    as
                                                                    libc::c_uint)
                           == 0 as libc::c_int as libc::c_uint {
                        Math_SmoothStepToS(&mut *(*effect).primColor.as_mut_ptr().offset(0
                                                                                             as
                                                                                             libc::c_int
                                                                                             as
                                                                                             isize),
                                           95 as libc::c_int as s16,
                                           1 as libc::c_int as s16,
                                           1 as libc::c_int as s16,
                                           0 as libc::c_int as s16);
                    }
                }
                (*effect).offset.x += (*effect).offset.y
            }
            if (*effect).type_0 as libc::c_int == VA_TUMOR as libc::c_int {
                refActor = (*effect).parent;
                (*effect).rot.z =
                    ((*effect).rot.z as libc::c_int + 0x157c as libc::c_int)
                        as s16;
                (*effect).envColor[3 as libc::c_int as usize] =
                    ((Math_SinS((*effect).rot.z) * 50.0f32) as s16 as
                         libc::c_int + 80 as libc::c_int) as s16;
                Math_SmoothStepToF(&mut (*effect).scale, (*effect).scaleMod,
                                   1.0f32, 0.01f32, 0.005f32);
                (*effect).pos.x =
                    (*effect).offset.x + (*refActor).actor.world.pos.x;
                (*effect).pos.y =
                    (*effect).offset.y + (*refActor).actor.world.pos.y;
                (*effect).pos.z =
                    (*effect).offset.z + (*refActor).actor.world.pos.z;
                match (*effect).mode as libc::c_int {
                    0 => {
                        if (*effect).timer as libc::c_int == 0 as libc::c_int
                           {
                            yaw =
                                Math_Vec3f_Yaw(&mut (*refActor).actor.world.pos,
                                               &mut (*effect).pos);
                            (*effect).type_0 = VA_NONE as libc::c_int as u8_0;
                            BossVa_BloodSplatter(globalCtx, effect, yaw,
                                                 ((*effect).scale * 4500.0f32)
                                                     as s16,
                                                 1 as libc::c_int);
                            BossVa_Gore(globalCtx, effect, yaw,
                                        ((*effect).scale * 1.2f32) as s16);
                        }
                    }
                    1 | 2 => {
                        if (*refActor).burst != 0 {
                            (*effect).type_0 = VA_NONE as libc::c_int as u8_0;
                            yaw =
                                Math_Vec3f_Yaw(&mut (*refActor).actor.world.pos,
                                               &mut (*effect).pos);
                            BossVa_BloodSplatter(globalCtx, effect, yaw,
                                                 ((*effect).scale * 4500.0f32)
                                                     as s16,
                                                 1 as libc::c_int);
                            BossVa_Gore(globalCtx, effect, yaw,
                                        ((*effect).scale * 1.2f32) as s16);
                        }
                    }
                    _ => { }
                }
            }
        }
        i += 1;
        effect = effect.offset(1)
    };
}
#[no_mangle]
pub unsafe extern "C" fn BossVa_DrawEffects(mut effect: *mut BossVaEffect,
                                            mut globalCtx:
                                                *mut GlobalContext) {
    static mut sSparkBallTex: [*mut libc::c_void; 8] =
        unsafe {
            [gBarinadeSparkBall1Tex.as_ptr() as *mut _ as *mut libc::c_void,
             gBarinadeSparkBall2Tex.as_ptr() as *mut _ as *mut libc::c_void,
             gBarinadeSparkBall3Tex.as_ptr() as *mut _ as *mut libc::c_void,
             gBarinadeSparkBall4Tex.as_ptr() as *mut _ as *mut libc::c_void,
             gBarinadeSparkBall5Tex.as_ptr() as *mut _ as *mut libc::c_void,
             gBarinadeSparkBall6Tex.as_ptr() as *mut _ as *mut libc::c_void,
             gBarinadeSparkBall7Tex.as_ptr() as *mut _ as *mut libc::c_void,
             gBarinadeSparkBall8Tex.as_ptr() as *mut _ as *mut libc::c_void]
        };
    let mut i: s16 = 0;
    let mut gfxCtx: *mut GraphicsContext = (*globalCtx).state.gfxCtx;
    let mut flag: u8_0 = 0 as libc::c_int as u8_0;
    let mut effectHead: *mut BossVaEffect = effect;
    let mut camera: *mut Camera = Gameplay_GetCamera(globalCtx, sCsCamera);
    let mut __gfxCtx: *mut GraphicsContext = 0 as *mut GraphicsContext;
    let mut dispRefs: [*mut Gfx; 4] = [0 as *mut Gfx; 4];
    __gfxCtx = gfxCtx;
    Graph_OpenDisps(dispRefs.as_mut_ptr(), gfxCtx,
                    b"../z_boss_va.c\x00" as *const u8 as *const libc::c_char,
                    4953 as libc::c_int);
    i = 0 as libc::c_int as s16;
    while (i as libc::c_int) <
              (::std::mem::size_of::<[BossVaEffect; 400]>() as
                   libc::c_ulong).wrapping_div(::std::mem::size_of::<BossVaEffect>()
                                                   as libc::c_ulong) as s32 {
        if (*effect).type_0 as libc::c_int == VA_LARGE_SPARK as libc::c_int {
            if flag == 0 {
                func_80093D84((*globalCtx).state.gfxCtx);
                let fresh25 = (*__gfxCtx).polyXlu.p;
                (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
                let mut _g: *mut Gfx = fresh25;
                (*_g).words.w0 =
                    (0xfb as libc::c_int as u32_0 &
                         (((0x1 as libc::c_int) << 8 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        24 as libc::c_int;
                (*_g).words.w1 =
                    (130 as libc::c_int as u32_0 &
                         (((0x1 as libc::c_int) << 8 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        24 as libc::c_int |
                        (130 as libc::c_int as u32_0 &
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
                let fresh26 = (*__gfxCtx).polyXlu.p;
                (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
                let mut _g_0: *mut Gfx = fresh26;
                (*_g_0).words.w0 =
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
                (*_g_0).words.w1 =
                    gBarinadeDL_0156A0.as_mut_ptr() as libc::c_uint;
                flag = flag.wrapping_add(1)
            }
            let fresh27 = (*__gfxCtx).polyXlu.p;
            (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
            let mut _g_1: *mut Gfx = fresh27;
            (*_g_1).words.w0 =
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
            (*_g_1).words.w1 =
                (230 as libc::c_int as u32_0 &
                     (((0x1 as libc::c_int) << 8 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    24 as libc::c_int |
                    (230 as libc::c_int as u32_0 &
                         (((0x1 as libc::c_int) << 8 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        16 as libc::c_int |
                    (230 as libc::c_int as u32_0 &
                         (((0x1 as libc::c_int) << 8 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        8 as libc::c_int |
                    ((*effect).primColor[3 as libc::c_int as usize] as u32_0 &
                         (((0x1 as libc::c_int) << 8 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        0 as libc::c_int;
            Matrix_Translate((*effect).pos.x, (*effect).pos.y,
                             (*effect).pos.z,
                             MTXMODE_NEW as libc::c_int as u8_0);
            func_800D1FD4(&mut (*globalCtx).billboardMtxF);
            Matrix_RotateZ((*effect).rot.z as libc::c_int as libc::c_float /
                               0x8000 as libc::c_int as f32_0 * 3.1416f32,
                           MTXMODE_APPLY as libc::c_int as u8_0);
            Matrix_Scale((*effect).scale * 0.0185f32,
                         (*effect).scale * 0.0185f32, 1.0f32,
                         MTXMODE_APPLY as libc::c_int as u8_0);
            let fresh28 = (*__gfxCtx).polyXlu.p;
            (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
            let mut _g_2: *mut Gfx = fresh28;
            (*_g_2).words.w0 =
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
            (*_g_2).words.w1 =
                Matrix_NewMtx(gfxCtx,
                              b"../z_boss_va.c\x00" as *const u8 as
                                  *const libc::c_char as *mut libc::c_char,
                              4976 as libc::c_int) as libc::c_uint;
            let fresh29 = (*__gfxCtx).polyXlu.p;
            (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
            let mut _g_3: *mut Gfx = fresh29;
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
            (*_g_3).words.w1 = gBarinadeDL_015710.as_mut_ptr() as libc::c_uint
        }
        i += 1;
        effect = effect.offset(1)
    }
    effect = effectHead;
    i = 0 as libc::c_int as s16;
    flag = 0 as libc::c_int as u8_0;
    while (i as libc::c_int) <
              (::std::mem::size_of::<[BossVaEffect; 400]>() as
                   libc::c_ulong).wrapping_div(::std::mem::size_of::<BossVaEffect>()
                                                   as libc::c_ulong) as s32 {
        if (*effect).type_0 as libc::c_int == VA_SPARK_BALL as libc::c_int {
            if flag == 0 {
                func_80093D84((*globalCtx).state.gfxCtx);
                let fresh30 = (*__gfxCtx).polyXlu.p;
                (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
                let mut _g_4: *mut Gfx = fresh30;
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
                (*_g_4).words.w1 =
                    gBarinadeDL_011738.as_mut_ptr() as libc::c_uint;
                flag = flag.wrapping_add(1)
            }
            Matrix_Translate((*effect).pos.x, (*effect).pos.y,
                             (*effect).pos.z,
                             MTXMODE_NEW as libc::c_int as u8_0);
            func_800D1FD4(&mut (*globalCtx).billboardMtxF);
            Matrix_Scale((*effect).scale, (*effect).scale, (*effect).scale,
                         MTXMODE_APPLY as libc::c_int as u8_0);
            Matrix_RotateZ((*effect).rot.z as libc::c_int as libc::c_float /
                               0x8000 as libc::c_int as f32_0 * 3.1416f32,
                           MTXMODE_APPLY as libc::c_int as u8_0);
            let fresh31 = (*__gfxCtx).polyXlu.p;
            (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
            let mut _g_5: *mut Gfx = fresh31;
            (*_g_5).words.w0 =
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
            (*_g_5).words.w1 =
                Matrix_NewMtx(gfxCtx,
                              b"../z_boss_va.c\x00" as *const u8 as
                                  *const libc::c_char as *mut libc::c_char,
                              5002 as libc::c_int) as libc::c_uint;
            let fresh32 = (*__gfxCtx).polyXlu.p;
            (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
            let mut _g_6: *mut Gfx = fresh32;
            (*_g_6).words.w0 =
                (0xe7 as libc::c_int as u32_0 &
                     (((0x1 as libc::c_int) << 8 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    24 as libc::c_int;
            (*_g_6).words.w1 = 0 as libc::c_int as libc::c_uint;
            let fresh33 = (*__gfxCtx).polyXlu.p;
            (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
            let mut _g_7: *mut Gfx = fresh33;
            (*_g_7).words.w0 =
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
            (*_g_7).words.w1 =
                gSegments[((sSparkBallTex[(*effect).mode as usize] as u32_0)
                               << 4 as libc::c_int >> 28 as libc::c_int) as
                              usize].wrapping_add(sSparkBallTex[(*effect).mode
                                                                    as usize]
                                                      as u32_0 &
                                                      0xffffff as libc::c_int
                                                          as
                                                          libc::c_uint).wrapping_add(0x80000000
                                                                                         as
                                                                                         libc::c_uint)
                    as *mut libc::c_void as libc::c_uint;
            let fresh34 = (*__gfxCtx).polyXlu.p;
            (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
            let mut _g_8: *mut Gfx = fresh34;
            (*_g_8).words.w0 =
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
            (*_g_8).words.w1 =
                ((*effect).primColor[0 as libc::c_int as usize] as u32_0 &
                     (((0x1 as libc::c_int) << 8 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    24 as libc::c_int |
                    ((*effect).primColor[1 as libc::c_int as usize] as u32_0 &
                         (((0x1 as libc::c_int) << 8 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        16 as libc::c_int |
                    ((*effect).primColor[2 as libc::c_int as usize] as u32_0 &
                         (((0x1 as libc::c_int) << 8 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        8 as libc::c_int |
                    ((*effect).primColor[3 as libc::c_int as usize] as u32_0 &
                         (((0x1 as libc::c_int) << 8 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        0 as libc::c_int;
            let fresh35 = (*__gfxCtx).polyXlu.p;
            (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
            let mut _g_9: *mut Gfx = fresh35;
            (*_g_9).words.w0 =
                (0xe7 as libc::c_int as u32_0 &
                     (((0x1 as libc::c_int) << 8 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    24 as libc::c_int;
            (*_g_9).words.w1 = 0 as libc::c_int as libc::c_uint;
            let fresh36 = (*__gfxCtx).polyXlu.p;
            (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
            let mut _g_10: *mut Gfx = fresh36;
            (*_g_10).words.w0 =
                (0xfb as libc::c_int as u32_0 &
                     (((0x1 as libc::c_int) << 8 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    24 as libc::c_int;
            (*_g_10).words.w1 =
                ((*effect).envColor[0 as libc::c_int as usize] as u32_0 &
                     (((0x1 as libc::c_int) << 8 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    24 as libc::c_int |
                    ((*effect).envColor[1 as libc::c_int as usize] as u32_0 &
                         (((0x1 as libc::c_int) << 8 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        16 as libc::c_int |
                    ((*effect).envColor[2 as libc::c_int as usize] as u32_0 &
                         (((0x1 as libc::c_int) << 8 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        8 as libc::c_int |
                    ((*effect).envColor[3 as libc::c_int as usize] as u32_0 &
                         (((0x1 as libc::c_int) << 8 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        0 as libc::c_int;
            let fresh37 = (*__gfxCtx).polyXlu.p;
            (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
            let mut _g_11: *mut Gfx = fresh37;
            (*_g_11).words.w0 =
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
            (*_g_11).words.w1 =
                gBarinadeDL_011768.as_mut_ptr() as libc::c_uint
        }
        i += 1;
        effect = effect.offset(1)
    }
    effect = effectHead;
    i = 0 as libc::c_int as s16;
    flag = 0 as libc::c_int as u8_0;
    while (i as libc::c_int) <
              (::std::mem::size_of::<[BossVaEffect; 400]>() as
                   libc::c_ulong).wrapping_div(::std::mem::size_of::<BossVaEffect>()
                                                   as libc::c_ulong) as s32 {
        if (*effect).type_0 as libc::c_int == VA_BLOOD as libc::c_int {
            if flag == 0 {
                func_80093D84((*globalCtx).state.gfxCtx);
                let fresh38 = (*__gfxCtx).polyXlu.p;
                (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
                let mut _g_12: *mut Gfx = fresh38;
                (*_g_12).words.w0 =
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
                (*_g_12).words.w1 =
                    gBarinadeDL_009430.as_mut_ptr() as libc::c_uint;
                let fresh39 = (*__gfxCtx).polyXlu.p;
                (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
                let mut _g_13: *mut Gfx = fresh39;
                (*_g_13).words.w0 =
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
                (*_g_13).words.w1 =
                    gSegments[((gEffBubble1Tex.as_mut_ptr() as u32_0) <<
                                   4 as libc::c_int >> 28 as libc::c_int) as
                                  usize].wrapping_add(gEffBubble1Tex.as_mut_ptr()
                                                          as u32_0 &
                                                          0xffffff as
                                                              libc::c_int as
                                                              libc::c_uint).wrapping_add(0x80000000
                                                                                             as
                                                                                             libc::c_uint)
                        as *mut libc::c_void as libc::c_uint;
                flag = flag.wrapping_add(1)
            }
            let fresh40 = (*__gfxCtx).polyXlu.p;
            (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
            let mut _g_14: *mut Gfx = fresh40;
            (*_g_14).words.w0 =
                (0xe7 as libc::c_int as u32_0 &
                     (((0x1 as libc::c_int) << 8 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    24 as libc::c_int;
            (*_g_14).words.w1 = 0 as libc::c_int as libc::c_uint;
            let fresh41 = (*__gfxCtx).polyXlu.p;
            (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
            let mut _g_15: *mut Gfx = fresh41;
            (*_g_15).words.w0 =
                (0xfb as libc::c_int as u32_0 &
                     (((0x1 as libc::c_int) << 8 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    24 as libc::c_int;
            (*_g_15).words.w1 =
                (0 as libc::c_int as u32_0 &
                     (((0x1 as libc::c_int) << 8 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    24 as libc::c_int |
                    (100 as libc::c_int as u32_0 &
                         (((0x1 as libc::c_int) << 8 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        16 as libc::c_int |
                    (0 as libc::c_int as u32_0 &
                         (((0x1 as libc::c_int) << 8 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        8 as libc::c_int |
                    ((*effect).envColor[3 as libc::c_int as usize] as u32_0 &
                         (((0x1 as libc::c_int) << 8 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        0 as libc::c_int;
            let fresh42 = (*__gfxCtx).polyXlu.p;
            (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
            let mut _g_16: *mut Gfx = fresh42;
            (*_g_16).words.w0 =
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
            (*_g_16).words.w1 =
                (0 as libc::c_int as u32_0 &
                     (((0x1 as libc::c_int) << 8 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    24 as libc::c_int |
                    (150 as libc::c_int as u32_0 &
                         (((0x1 as libc::c_int) << 8 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        16 as libc::c_int |
                    (0 as libc::c_int as u32_0 &
                         (((0x1 as libc::c_int) << 8 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        8 as libc::c_int |
                    ((*effect).primColor[3 as libc::c_int as usize] as u32_0 &
                         (((0x1 as libc::c_int) << 8 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        0 as libc::c_int;
            Matrix_Translate((*effect).pos.x, (*effect).pos.y,
                             (*effect).pos.z,
                             MTXMODE_NEW as libc::c_int as u8_0);
            if (*effect).mode as libc::c_int == BLOOD_SPOT as libc::c_int {
                Matrix_RotateX(3.14159265358979323846f32 /
                                   2 as libc::c_int as libc::c_float,
                               MTXMODE_APPLY as libc::c_int as u8_0);
            } else { func_800D1FD4(&mut (*globalCtx).billboardMtxF); }
            Matrix_Scale((*effect).scale, (*effect).scale, 1.0f32,
                         MTXMODE_APPLY as libc::c_int as u8_0);
            let fresh43 = (*__gfxCtx).polyXlu.p;
            (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
            let mut _g_17: *mut Gfx = fresh43;
            (*_g_17).words.w0 =
                (0xe7 as libc::c_int as u32_0 &
                     (((0x1 as libc::c_int) << 8 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    24 as libc::c_int;
            (*_g_17).words.w1 = 0 as libc::c_int as libc::c_uint;
            let fresh44 = (*__gfxCtx).polyXlu.p;
            (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
            let mut _g_18: *mut Gfx = fresh44;
            (*_g_18).words.w0 =
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
            (*_g_18).words.w1 =
                Matrix_NewMtx(gfxCtx,
                              b"../z_boss_va.c\x00" as *const u8 as
                                  *const libc::c_char as *mut libc::c_char,
                              5052 as libc::c_int) as libc::c_uint;
            let fresh45 = (*__gfxCtx).polyXlu.p;
            (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
            let mut _g_19: *mut Gfx = fresh45;
            (*_g_19).words.w0 =
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
            (*_g_19).words.w1 =
                gBarinadeDL_009468.as_mut_ptr() as libc::c_uint
        }
        i += 1;
        effect = effect.offset(1)
    }
    effect = effectHead;
    i = 0 as libc::c_int as s16;
    flag = 0 as libc::c_int as u8_0;
    while (i as libc::c_int) <
              (::std::mem::size_of::<[BossVaEffect; 400]>() as
                   libc::c_ulong).wrapping_div(::std::mem::size_of::<BossVaEffect>()
                                                   as libc::c_ulong) as s32 {
        if (*effect).type_0 as libc::c_int == VA_TUMOR as libc::c_int {
            let mut parent: *mut BossVa = (*effect).parent;
            if flag == 0 {
                func_80093D18((*globalCtx).state.gfxCtx);
                let fresh46 = (*__gfxCtx).polyOpa.p;
                (*__gfxCtx).polyOpa.p = (*__gfxCtx).polyOpa.p.offset(1);
                let mut _g_20: *mut Gfx = fresh46;
                (*_g_20).words.w0 =
                    (0xfb as libc::c_int as u32_0 &
                         (((0x1 as libc::c_int) << 8 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        24 as libc::c_int;
                (*_g_20).words.w1 =
                    (0 as libc::c_int as u32_0 &
                         (((0x1 as libc::c_int) << 8 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        24 as libc::c_int |
                        (0 as libc::c_int as u32_0 &
                             (((0x1 as libc::c_int) << 8 as libc::c_int) -
                                  1 as libc::c_int) as libc::c_uint) <<
                            16 as libc::c_int |
                        (0 as libc::c_int as u32_0 &
                             (((0x1 as libc::c_int) << 8 as libc::c_int) -
                                  1 as libc::c_int) as libc::c_uint) <<
                            8 as libc::c_int |
                        ((*effect).envColor[3 as libc::c_int as usize] as
                             u32_0 &
                             (((0x1 as libc::c_int) << 8 as libc::c_int) -
                                  1 as libc::c_int) as libc::c_uint) <<
                            0 as libc::c_int;
                let fresh47 = (*__gfxCtx).polyOpa.p;
                (*__gfxCtx).polyOpa.p = (*__gfxCtx).polyOpa.p.offset(1);
                let mut _g_21: *mut Gfx = fresh47;
                (*_g_21).words.w0 =
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
                (*_g_21).words.w1 =
                    gBarinadeDL_0128B8.as_mut_ptr() as libc::c_uint;
                flag = flag.wrapping_add(1)
            }
            if (*effect).mode as libc::c_int != TUMOR_BODY as libc::c_int ||
                   Math_Vec3f_DistXZ(&mut (*camera).eye, &mut (*effect).pos) -
                       Math_Vec3f_DistXZ(&mut (*camera).eye,
                                         &mut (*parent).actor.world.pos) <
                       10.0f32 {
                Matrix_Translate((*effect).pos.x, (*effect).pos.y,
                                 (*effect).pos.z,
                                 MTXMODE_NEW as libc::c_int as u8_0);
                Matrix_Scale((*effect).scale, (*effect).scale,
                             (*effect).scale,
                             MTXMODE_APPLY as libc::c_int as u8_0);
                let fresh48 = (*__gfxCtx).polyOpa.p;
                (*__gfxCtx).polyOpa.p = (*__gfxCtx).polyOpa.p.offset(1);
                let mut _g_22: *mut Gfx = fresh48;
                (*_g_22).words.w0 =
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
                (*_g_22).words.w1 =
                    Matrix_NewMtx(gfxCtx,
                                  b"../z_boss_va.c\x00" as *const u8 as
                                      *const libc::c_char as
                                      *mut libc::c_char, 5080 as libc::c_int)
                        as libc::c_uint;
                let fresh49 = (*__gfxCtx).polyOpa.p;
                (*__gfxCtx).polyOpa.p = (*__gfxCtx).polyOpa.p.offset(1);
                let mut _g_23: *mut Gfx = fresh49;
                (*_g_23).words.w0 =
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
                (*_g_23).words.w1 =
                    gBarinadeDL_012948.as_mut_ptr() as libc::c_uint
            }
        }
        i += 1;
        effect = effect.offset(1)
    }
    effect = effectHead;
    i = 0 as libc::c_int as s16;
    flag = 0 as libc::c_int as u8_0;
    while (i as libc::c_int) <
              (::std::mem::size_of::<[BossVaEffect; 400]>() as
                   libc::c_ulong).wrapping_div(::std::mem::size_of::<BossVaEffect>()
                                                   as libc::c_ulong) as s32 {
        if (*effect).type_0 as libc::c_int == VA_GORE as libc::c_int {
            if flag == 0 {
                func_80093D18((*globalCtx).state.gfxCtx);
                let fresh50 = (*__gfxCtx).polyOpa.p;
                (*__gfxCtx).polyOpa.p = (*__gfxCtx).polyOpa.p.offset(1);
                let mut _g_24: *mut Gfx = fresh50;
                (*_g_24).words.w0 =
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
                (*_g_24).words.w1 =
                    gBarinadeDL_012BA0.as_mut_ptr() as libc::c_uint;
                flag = flag.wrapping_add(1)
            }
            let fresh51 = (*__gfxCtx).polyOpa.p;
            (*__gfxCtx).polyOpa.p = (*__gfxCtx).polyOpa.p.offset(1);
            let mut _g_25: *mut Gfx = fresh51;
            (*_g_25).words.w0 =
                (0xe7 as libc::c_int as u32_0 &
                     (((0x1 as libc::c_int) << 8 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    24 as libc::c_int;
            (*_g_25).words.w1 = 0 as libc::c_int as libc::c_uint;
            let fresh52 = (*__gfxCtx).polyOpa.p;
            (*__gfxCtx).polyOpa.p = (*__gfxCtx).polyOpa.p.offset(1);
            let mut _g_26: *mut Gfx = fresh52;
            (*_g_26).words.w0 =
                (0xfb as libc::c_int as u32_0 &
                     (((0x1 as libc::c_int) << 8 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    24 as libc::c_int;
            (*_g_26).words.w1 =
                (255 as libc::c_int as u32_0 &
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
                    ((*effect).primColor[3 as libc::c_int as usize] as u32_0 &
                         (((0x1 as libc::c_int) << 8 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        0 as libc::c_int;
            let fresh53 = (*__gfxCtx).polyOpa.p;
            (*__gfxCtx).polyOpa.p = (*__gfxCtx).polyOpa.p.offset(1);
            let mut _g_27: *mut Gfx = fresh53;
            (*_g_27).words.w0 =
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
            (*_g_27).words.w1 =
                ((*effect).primColor[0 as libc::c_int as usize] as u32_0 &
                     (((0x1 as libc::c_int) << 8 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    24 as libc::c_int |
                    ((*effect).primColor[1 as libc::c_int as usize] as u32_0 &
                         (((0x1 as libc::c_int) << 8 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        16 as libc::c_int |
                    ((*effect).primColor[2 as libc::c_int as usize] as u32_0 &
                         (((0x1 as libc::c_int) << 8 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        8 as libc::c_int |
                    ((*effect).primColor[3 as libc::c_int as usize] as u32_0 &
                         (((0x1 as libc::c_int) << 8 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        0 as libc::c_int;
            Matrix_Translate((*effect).pos.x, (*effect).pos.y,
                             (*effect).pos.z,
                             MTXMODE_NEW as libc::c_int as u8_0);
            Matrix_RotateZYX((*effect).rot.x, (*effect).rot.y,
                             0 as libc::c_int as s16,
                             MTXMODE_APPLY as libc::c_int as u8_0);
            Matrix_Scale((*effect).scale, (*effect).scale, (*effect).scale,
                         MTXMODE_APPLY as libc::c_int as u8_0);
            Matrix_RotateX((*effect).offset.x * 0.115f32,
                           MTXMODE_APPLY as libc::c_int as u8_0);
            Matrix_RotateY((*effect).offset.x * 0.13f32,
                           MTXMODE_APPLY as libc::c_int as u8_0);
            Matrix_RotateZ((*effect).offset.x * 0.1f32,
                           MTXMODE_APPLY as libc::c_int as u8_0);
            Matrix_Scale(1.0f32 - (*effect).scaleMod,
                         (*effect).scaleMod + 1.0f32,
                         1.0f32 - (*effect).scaleMod,
                         MTXMODE_APPLY as libc::c_int as u8_0);
            Matrix_RotateZ(-((*effect).offset.x * 0.1f32),
                           MTXMODE_APPLY as libc::c_int as u8_0);
            Matrix_RotateY(-((*effect).offset.x * 0.13f32),
                           MTXMODE_APPLY as libc::c_int as u8_0);
            Matrix_RotateX(-((*effect).offset.x * 0.115f32),
                           MTXMODE_APPLY as libc::c_int as u8_0);
            let fresh54 = (*__gfxCtx).polyOpa.p;
            (*__gfxCtx).polyOpa.p = (*__gfxCtx).polyOpa.p.offset(1);
            let mut _g_28: *mut Gfx = fresh54;
            (*_g_28).words.w0 =
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
            (*_g_28).words.w1 =
                Matrix_NewMtx(gfxCtx,
                              b"../z_boss_va.c\x00" as *const u8 as
                                  *const libc::c_char as *mut libc::c_char,
                              5124 as libc::c_int) as libc::c_uint;
            let fresh55 = (*__gfxCtx).polyOpa.p;
            (*__gfxCtx).polyOpa.p = (*__gfxCtx).polyOpa.p.offset(1);
            let mut _g_29: *mut Gfx = fresh55;
            (*_g_29).words.w0 =
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
            (*_g_29).words.w1 =
                gBarinadeDL_012C50.as_mut_ptr() as libc::c_uint
        }
        i += 1;
        effect = effect.offset(1)
    }
    effect = effectHead;
    i = 0 as libc::c_int as s16;
    flag = 0 as libc::c_int as u8_0;
    while (i as libc::c_int) <
              (::std::mem::size_of::<[BossVaEffect; 400]>() as
                   libc::c_ulong).wrapping_div(::std::mem::size_of::<BossVaEffect>()
                                                   as libc::c_ulong) as s32 {
        if (*effect).type_0 as libc::c_int == VA_ZAP_CHARGE as libc::c_int {
            if flag == 0 {
                func_80093D84((*globalCtx).state.gfxCtx);
                let fresh56 = (*__gfxCtx).polyXlu.p;
                (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
                let mut _g_30: *mut Gfx = fresh56;
                (*_g_30).words.w0 =
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
                (*_g_30).words.w1 =
                    gBarinadeDL_0135B0.as_mut_ptr() as libc::c_uint;
                flag = flag.wrapping_add(1)
            }
            let fresh57 = (*__gfxCtx).polyXlu.p;
            (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
            let mut _g_31: *mut Gfx = fresh57;
            (*_g_31).words.w0 =
                (0xe7 as libc::c_int as u32_0 &
                     (((0x1 as libc::c_int) << 8 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    24 as libc::c_int;
            (*_g_31).words.w1 = 0 as libc::c_int as libc::c_uint;
            let fresh58 = (*__gfxCtx).polyXlu.p;
            (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
            let mut _g_32: *mut Gfx = fresh58;
            (*_g_32).words.w0 =
                (0xfb as libc::c_int as u32_0 &
                     (((0x1 as libc::c_int) << 8 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    24 as libc::c_int;
            (*_g_32).words.w1 =
                (255 as libc::c_int as u32_0 &
                     (((0x1 as libc::c_int) << 8 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    24 as libc::c_int |
                    (255 as libc::c_int as u32_0 &
                         (((0x1 as libc::c_int) << 8 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        16 as libc::c_int |
                    (50 as libc::c_int as u32_0 &
                         (((0x1 as libc::c_int) << 8 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        8 as libc::c_int |
                    ((*effect).primColor[3 as libc::c_int as usize] as u32_0 &
                         (((0x1 as libc::c_int) << 8 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        0 as libc::c_int;
            let fresh59 = (*__gfxCtx).polyXlu.p;
            (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
            let mut _g_33: *mut Gfx = fresh59;
            (*_g_33).words.w0 =
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
            (*_g_33).words.w1 =
                (255 as libc::c_int as u32_0 &
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
                    ((*effect).primColor[3 as libc::c_int as usize] as u32_0 &
                         (((0x1 as libc::c_int) << 8 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        0 as libc::c_int;
            Matrix_Translate((*effect).pos.x, (*effect).pos.y,
                             (*effect).pos.z,
                             MTXMODE_NEW as libc::c_int as u8_0);
            Matrix_RotateZYX((*effect).rot.x, (*effect).rot.y,
                             0 as libc::c_int as s16,
                             MTXMODE_APPLY as libc::c_int as u8_0);
            Matrix_Scale((*effect).scale, (*effect).scale, (*effect).scale,
                         MTXMODE_APPLY as libc::c_int as u8_0);
            let fresh60 = (*__gfxCtx).polyXlu.p;
            (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
            let mut _g_34: *mut Gfx = fresh60;
            (*_g_34).words.w0 =
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
            (*_g_34).words.w1 =
                Matrix_NewMtx(gfxCtx,
                              b"../z_boss_va.c\x00" as *const u8 as
                                  *const libc::c_char as *mut libc::c_char,
                              5152 as libc::c_int) as libc::c_uint;
            let fresh61 = (*__gfxCtx).polyXlu.p;
            (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
            let mut _g_35: *mut Gfx = fresh61;
            (*_g_35).words.w0 =
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
            (*_g_35).words.w1 =
                gBarinadeDL_013638.as_mut_ptr() as libc::c_uint
        }
        i += 1;
        effect = effect.offset(1)
    }
    effect = effectHead;
    i = 0 as libc::c_int as s16;
    flag = 0 as libc::c_int as u8_0;
    while (i as libc::c_int) <
              (::std::mem::size_of::<[BossVaEffect; 400]>() as
                   libc::c_ulong).wrapping_div(::std::mem::size_of::<BossVaEffect>()
                                                   as libc::c_ulong) as s32 {
        if (*effect).type_0 as libc::c_int == VA_BLAST_SPARK as libc::c_int {
            if flag == 0 {
                func_80093C14((*globalCtx).state.gfxCtx);
                let fresh62 = (*__gfxCtx).polyXlu.p;
                (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
                let mut _g_36: *mut Gfx = fresh62;
                (*_g_36).words.w0 =
                    (0xfb as libc::c_int as u32_0 &
                         (((0x1 as libc::c_int) << 8 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        24 as libc::c_int;
                (*_g_36).words.w1 =
                    (130 as libc::c_int as u32_0 &
                         (((0x1 as libc::c_int) << 8 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        24 as libc::c_int |
                        (130 as libc::c_int as u32_0 &
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
                let fresh63 = (*__gfxCtx).polyXlu.p;
                (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
                let mut _g_37: *mut Gfx = fresh63;
                (*_g_37).words.w0 =
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
                (*_g_37).words.w1 =
                    gBarinadeDL_0156A0.as_mut_ptr() as libc::c_uint;
                flag = flag.wrapping_add(1)
            }
            let fresh64 = (*__gfxCtx).polyXlu.p;
            (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
            let mut _g_38: *mut Gfx = fresh64;
            (*_g_38).words.w0 =
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
            (*_g_38).words.w1 =
                (230 as libc::c_int as u32_0 &
                     (((0x1 as libc::c_int) << 8 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    24 as libc::c_int |
                    (230 as libc::c_int as u32_0 &
                         (((0x1 as libc::c_int) << 8 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        16 as libc::c_int |
                    (230 as libc::c_int as u32_0 &
                         (((0x1 as libc::c_int) << 8 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        8 as libc::c_int |
                    ((*effect).primColor[3 as libc::c_int as usize] as u32_0 &
                         (((0x1 as libc::c_int) << 8 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        0 as libc::c_int;
            Matrix_Translate((*effect).pos.x, (*effect).pos.y,
                             (*effect).pos.z,
                             MTXMODE_NEW as libc::c_int as u8_0);
            func_800D1FD4(&mut (*globalCtx).billboardMtxF);
            Matrix_RotateZ((*effect).rot.z as libc::c_int as libc::c_float /
                               0x8000 as libc::c_int as f32_0 * 3.1416f32,
                           MTXMODE_APPLY as libc::c_int as u8_0);
            Matrix_Scale((*effect).scale * 0.02f32, (*effect).scale * 0.02f32,
                         1.0f32, MTXMODE_APPLY as libc::c_int as u8_0);
            let fresh65 = (*__gfxCtx).polyXlu.p;
            (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
            let mut _g_39: *mut Gfx = fresh65;
            (*_g_39).words.w0 =
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
            (*_g_39).words.w1 =
                Matrix_NewMtx(gfxCtx,
                              b"../z_boss_va.c\x00" as *const u8 as
                                  *const libc::c_char as *mut libc::c_char,
                              5180 as libc::c_int) as libc::c_uint;
            let fresh66 = (*__gfxCtx).polyXlu.p;
            (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
            let mut _g_40: *mut Gfx = fresh66;
            (*_g_40).words.w0 =
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
            (*_g_40).words.w1 =
                gBarinadeDL_015710.as_mut_ptr() as libc::c_uint
        }
        i += 1;
        effect = effect.offset(1)
    }
    effect = effectHead;
    i = 0 as libc::c_int as s16;
    flag = 0 as libc::c_int as u8_0;
    while (i as libc::c_int) <
              (::std::mem::size_of::<[BossVaEffect; 400]>() as
                   libc::c_ulong).wrapping_div(::std::mem::size_of::<BossVaEffect>()
                                                   as libc::c_ulong) as s32 {
        if (*effect).type_0 as libc::c_int == VA_SMALL_SPARK as libc::c_int {
            if flag == 0 {
                func_80093D84((*globalCtx).state.gfxCtx);
                let fresh67 = (*__gfxCtx).polyXlu.p;
                (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
                let mut _g_41: *mut Gfx = fresh67;
                (*_g_41).words.w0 =
                    (0xfb as libc::c_int as u32_0 &
                         (((0x1 as libc::c_int) << 8 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        24 as libc::c_int;
                (*_g_41).words.w1 =
                    (255 as libc::c_int as u32_0 &
                         (((0x1 as libc::c_int) << 8 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        24 as libc::c_int |
                        (255 as libc::c_int as u32_0 &
                             (((0x1 as libc::c_int) << 8 as libc::c_int) -
                                  1 as libc::c_int) as libc::c_uint) <<
                            16 as libc::c_int |
                        (100 as libc::c_int as u32_0 &
                             (((0x1 as libc::c_int) << 8 as libc::c_int) -
                                  1 as libc::c_int) as libc::c_uint) <<
                            8 as libc::c_int |
                        (0 as libc::c_int as u32_0 &
                             (((0x1 as libc::c_int) << 8 as libc::c_int) -
                                  1 as libc::c_int) as libc::c_uint) <<
                            0 as libc::c_int;
                let fresh68 = (*__gfxCtx).polyXlu.p;
                (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
                let mut _g_42: *mut Gfx = fresh68;
                (*_g_42).words.w0 =
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
                (*_g_42).words.w1 =
                    gBarinadeDL_008F08.as_mut_ptr() as libc::c_uint;
                flag = flag.wrapping_add(1)
            }
            let fresh69 = (*__gfxCtx).polyXlu.p;
            (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
            let mut _g_43: *mut Gfx = fresh69;
            (*_g_43).words.w0 =
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
            (*_g_43).words.w1 =
                (255 as libc::c_int as u32_0 &
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
                    ((*effect).primColor[3 as libc::c_int as usize] as u32_0 &
                         (((0x1 as libc::c_int) << 8 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        0 as libc::c_int;
            Matrix_Translate((*effect).pos.x, (*effect).pos.y,
                             (*effect).pos.z,
                             MTXMODE_NEW as libc::c_int as u8_0);
            Matrix_RotateZ((*effect).rot.z as libc::c_int as libc::c_float /
                               0x8000 as libc::c_int as f32_0 * 3.1416f32,
                           MTXMODE_APPLY as libc::c_int as u8_0);
            Matrix_RotateY((*effect).rot.y as libc::c_int as libc::c_float /
                               0x8000 as libc::c_int as f32_0 * 3.1416f32,
                           MTXMODE_APPLY as libc::c_int as u8_0);
            Matrix_Scale((*effect).scale, (*effect).scale, 1.0f32,
                         MTXMODE_APPLY as libc::c_int as u8_0);
            let fresh70 = (*__gfxCtx).polyXlu.p;
            (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
            let mut _g_44: *mut Gfx = fresh70;
            (*_g_44).words.w0 =
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
            (*_g_44).words.w1 =
                Matrix_NewMtx(gfxCtx,
                              b"../z_boss_va.c\x00" as *const u8 as
                                  *const libc::c_char as *mut libc::c_char,
                              5208 as libc::c_int) as libc::c_uint;
            let fresh71 = (*__gfxCtx).polyXlu.p;
            (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
            let mut _g_45: *mut Gfx = fresh71;
            (*_g_45).words.w0 =
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
            (*_g_45).words.w1 =
                gBarinadeDL_008F70.as_mut_ptr() as libc::c_uint
        }
        i += 1;
        effect = effect.offset(1)
    }
    Graph_CloseDisps(dispRefs.as_mut_ptr(), gfxCtx,
                     b"../z_boss_va.c\x00" as *const u8 as
                         *const libc::c_char, 5215 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn BossVa_SpawnSpark(mut globalCtx: *mut GlobalContext,
                                           mut effect: *mut BossVaEffect,
                                           mut this: *mut BossVa,
                                           mut offset: *mut Vec3f,
                                           mut scale: s16, mut mode: u8_0) {
    let mut player: *mut Player =
        (*globalCtx).actorCtx.actorLists[ACTORCAT_PLAYER as libc::c_int as
                                             usize].head as *mut Player;
    let mut index: s16 = 0;
    let mut pos: Vec3f =
        { let mut init = Vec3f{x: 0.0f32, y: -1000.0f32, z: 0.0f32,}; init };
    let mut tempVec: Vec3f = Vec3f{x: 0., y: 0., z: 0.,};
    let mut i: s16 = 0;
    i = 0 as libc::c_int as s16;
    while (i as libc::c_int) <
              (::std::mem::size_of::<[BossVaEffect; 400]>() as
                   libc::c_ulong).wrapping_div(::std::mem::size_of::<BossVaEffect>()
                                                   as libc::c_ulong) as s32 {
        if (*effect).type_0 as libc::c_int == VA_NONE as libc::c_int {
            (*effect).type_0 = VA_LARGE_SPARK as libc::c_int as u8_0;
            (*effect).parent = this;
            (*effect).pos = pos;
            (*effect).timer =
                ((Rand_ZeroOne() * 10.0f32) as s16 as libc::c_int +
                     111 as libc::c_int) as u16_0;
            (*effect).accel = sZeroVec;
            (*effect).velocity = (*effect).accel;
            (*effect).mode = mode as s16;
            let mut current_block_25: u64;
            match mode as libc::c_int {
                4 => {
                    (*effect).type_0 = VA_SMALL_SPARK as libc::c_int as u8_0;
                    current_block_25 = 9995318210332142376;
                }
                1 => { current_block_25 = 9995318210332142376; }
                5 => {
                    (*effect).type_0 = VA_SMALL_SPARK as libc::c_int as u8_0;
                    current_block_25 = 11181320600178091067;
                }
                2 => { current_block_25 = 11181320600178091067; }
                3 => {
                    (*effect).type_0 = VA_BLAST_SPARK as libc::c_int as u8_0;
                    (*effect).pos.x = (*offset).x + (*this).actor.world.pos.x;
                    (*effect).pos.y = (*offset).y + (*this).actor.world.pos.y;
                    (*effect).pos.z = (*offset).z + (*this).actor.world.pos.z;
                    (*effect).timer = 111 as libc::c_int as u16_0;
                    current_block_25 = 11459959175219260272;
                }
                6 => {
                    (*effect).type_0 = VA_SMALL_SPARK as libc::c_int as u8_0;
                    index =
                        Rand_ZeroFloat((::std::mem::size_of::<[Vec3f; 18]>()
                                            as
                                            libc::c_ulong).wrapping_div(::std::mem::size_of::<Vec3f>()
                                                                            as
                                                                            libc::c_ulong)
                                           as s32 as libc::c_float - 0.1f32)
                            as s16;
                    (*effect).pos.x =
                        (*player).bodyPartsPos[index as usize].x +
                            Rand_CenteredFloat(10.0f32);
                    (*effect).pos.y =
                        (*player).bodyPartsPos[index as usize].y +
                            Rand_CenteredFloat(15.0f32);
                    (*effect).pos.z =
                        (*player).bodyPartsPos[index as usize].z +
                            Rand_CenteredFloat(10.0f32);
                    current_block_25 = 11459959175219260272;
                }
                _ => { current_block_25 = 11459959175219260272; }
            }
            match current_block_25 {
                11181320600178091067 => {
                    (*effect).offset.x = (*offset).x;
                    (*effect).offset.z = (*offset).z
                }
                9995318210332142376 => {
                    tempVec = *offset;
                    tempVec.x += (*this).actor.world.pos.x;
                    tempVec.z += (*this).actor.world.pos.z;
                    (*effect).offset.x =
                        Math_Vec3f_DistXZ(&mut (*this).actor.world.pos,
                                          &mut tempVec);
                    (*effect).rot.x =
                        Math_Vec3f_Pitch(&mut (*this).actor.world.pos,
                                         &mut (*((*this).actor.parent as
                                                     *mut BossVa)).unk_1D8)
                }
                _ => { }
            }
            (*effect).offset.y = (*offset).y;
            (*effect).scale =
                (Rand_ZeroFloat(scale as f32_0) +
                     scale as libc::c_int as libc::c_float) * 0.01f32;
            (*effect).primColor[3 as libc::c_int as usize] =
                255 as libc::c_int as s16;
            break ;
        } else { i += 1; effect = effect.offset(1) }
    };
}
#[no_mangle]
pub unsafe extern "C" fn BossVa_SpawnSparkBall(mut globalCtx:
                                                   *mut GlobalContext,
                                               mut effect: *mut BossVaEffect,
                                               mut this: *mut BossVa,
                                               mut offset: *mut Vec3f,
                                               mut scale: s16,
                                               mut mode: u8_0) {
    let mut pos: Vec3f =
        { let mut init = Vec3f{x: 0.0f32, y: -1000.0f32, z: 0.0f32,}; init };
    let mut i: s16 = 0;
    i = 0 as libc::c_int as s16;
    while (i as libc::c_int) <
              (::std::mem::size_of::<[BossVaEffect; 400]>() as
                   libc::c_ulong).wrapping_div(::std::mem::size_of::<BossVaEffect>()
                                                   as libc::c_ulong) as s32 {
        if (*effect).type_0 as libc::c_int == VA_NONE as libc::c_int {
            (*effect).type_0 = VA_SPARK_BALL as libc::c_int as u8_0;
            (*effect).parent = this;
            (*effect).pos = pos;
            (*effect).accel = sZeroVec;
            (*effect).velocity = (*effect).accel;
            (*effect).mode = 0 as libc::c_int as s16;
            (*effect).offset.x = (*offset).x;
            (*effect).offset.z = (*offset).z;
            (*effect).offset.y = (*offset).y;
            (*effect).timer =
                ((Rand_ZeroOne() * 10.0f32) as s16 as libc::c_int +
                     111 as libc::c_int) as u16_0;
            (*effect).primColor[3 as libc::c_int as usize] =
                230 as libc::c_int as s16;
            (*effect).primColor[2 as libc::c_int as usize] =
                (*effect).primColor[3 as libc::c_int as usize];
            (*effect).primColor[1 as libc::c_int as usize] =
                (*effect).primColor[2 as libc::c_int as usize];
            (*effect).primColor[0 as libc::c_int as usize] =
                (*effect).primColor[1 as libc::c_int as usize];
            (*effect).envColor[0 as libc::c_int as usize] =
                0 as libc::c_int as s16;
            (*effect).envColor[1 as libc::c_int as usize] =
                100 as libc::c_int as s16;
            (*effect).envColor[2 as libc::c_int as usize] =
                220 as libc::c_int as s16;
            (*effect).envColor[3 as libc::c_int as usize] =
                160 as libc::c_int as s16;
            (*effect).scale =
                (Rand_ZeroFloat(scale as f32_0) +
                     scale as libc::c_int as libc::c_float) * 0.01f32;
            return
        }
        i += 1;
        effect = effect.offset(1)
    };
}
#[no_mangle]
pub unsafe extern "C" fn BossVa_SpawnBloodDroplets(mut globalCtx:
                                                       *mut GlobalContext,
                                                   mut effect:
                                                       *mut BossVaEffect,
                                                   mut pos: *mut Vec3f,
                                                   mut scale: s16,
                                                   mut phase: s16,
                                                   mut yaw: s16) {
    let mut i: s32 = 0;
    let mut accel: Vec3f =
        { let mut init = Vec3f{x: 0.0f32, y: 0.0f32, z: 0.0f32,}; init };
    let mut velocity: Vec3f =
        { let mut init = Vec3f{x: 0.0f32, y: 0.0f32, z: 0.0f32,}; init };
    let mut xzVel: f32_0 = 0.;
    i = 0 as libc::c_int;
    while i <
              (::std::mem::size_of::<[BossVaEffect; 400]>() as
                   libc::c_ulong).wrapping_div(::std::mem::size_of::<BossVaEffect>()
                                                   as libc::c_ulong) as s32 {
        if (*effect).type_0 as libc::c_int == VA_NONE as libc::c_int {
            (*effect).type_0 = VA_BLOOD as libc::c_int as u8_0;
            (*effect).pos = *pos;
            (*effect).mode = BLOOD_DROPLET as libc::c_int as s16;
            xzVel = Math_SinS(phase) * 6.0f32;
            velocity.x = Rand_CenteredFloat(1.0f32) + -Math_SinS(yaw) * xzVel;
            velocity.z = Rand_CenteredFloat(1.0f32) + -Math_CosS(yaw) * xzVel;
            (*effect).velocity = velocity;
            accel.y = Rand_CenteredFloat(0.3f32) - 1.0f32;
            (*effect).accel = accel;
            (*effect).timer = 20 as libc::c_int as u16_0;
            (*effect).envColor[3 as libc::c_int as usize] =
                100 as libc::c_int as s16;
            (*effect).primColor[3 as libc::c_int as usize] =
                200 as libc::c_int as s16;
            (*effect).scale =
                (Rand_ZeroFloat(scale as f32_0) +
                     scale as libc::c_int as libc::c_float) * 0.01f32;
            break ;
        } else { i += 1; effect = effect.offset(1) }
    };
}
#[no_mangle]
pub unsafe extern "C" fn BossVa_SpawnBloodSplatter(mut globalCtx:
                                                       *mut GlobalContext,
                                                   mut effect:
                                                       *mut BossVaEffect,
                                                   mut pos: *mut Vec3f,
                                                   mut yaw: s16,
                                                   mut scale: s16) {
    let mut i: s32 = 0;
    let mut xzVel: f32_0 = 0.;
    let mut accel: Vec3f =
        { let mut init = Vec3f{x: 0.0f32, y: 0.0f32, z: 0.0f32,}; init };
    let mut velocity: Vec3f = Vec3f{x: 0., y: 0., z: 0.,};
    i = 0 as libc::c_int;
    while i <
              (::std::mem::size_of::<[BossVaEffect; 400]>() as
                   libc::c_ulong).wrapping_div(::std::mem::size_of::<BossVaEffect>()
                                                   as libc::c_ulong) as s32 {
        if (*effect).type_0 as libc::c_int == VA_NONE as libc::c_int {
            (*effect).type_0 = VA_BLOOD as libc::c_int as u8_0;
            (*effect).pos = *pos;
            (*effect).mode = BLOOD_SPLATTER as libc::c_int as s16;
            xzVel = Rand_ZeroOne() * 7.0f32;
            velocity.x = Math_SinS(yaw) * xzVel;
            velocity.y = Rand_CenteredFloat(4.0f32) + 4.0f32;
            velocity.z = Math_CosS(yaw) * xzVel;
            (*effect).velocity = velocity;
            accel.y = Rand_CenteredFloat(0.3f32) - 1.0f32;
            (*effect).accel = accel;
            if sCsState as libc::c_int <= DEATH_SHELL_BURST as libc::c_int {
                (*effect).timer = 20 as libc::c_int as u16_0
            } else { (*effect).timer = 60 as libc::c_int as u16_0 }
            (*effect).envColor[3 as libc::c_int as usize] =
                100 as libc::c_int as s16;
            (*effect).primColor[3 as libc::c_int as usize] =
                200 as libc::c_int as s16;
            (*effect).scale = scale as libc::c_int as libc::c_float * 0.01f32;
            break ;
        } else { i += 1; effect = effect.offset(1) }
    };
}
#[no_mangle]
pub unsafe extern "C" fn BossVa_SpawnTumor(mut globalCtx: *mut GlobalContext,
                                           mut effect: *mut BossVaEffect,
                                           mut this: *mut BossVa,
                                           mut offset: *mut Vec3f,
                                           mut scale: s16, mut mode: u8_0) {
    let mut pos: Vec3f =
        { let mut init = Vec3f{x: 0.0f32, y: -1000.0f32, z: 0.0f32,}; init };
    let mut i: s16 = 0;
    i = 0 as libc::c_int as s16;
    while (i as libc::c_int) <
              (::std::mem::size_of::<[BossVaEffect; 400]>() as
                   libc::c_ulong).wrapping_div(::std::mem::size_of::<BossVaEffect>()
                                                   as libc::c_ulong) as s32 {
        if (*effect).type_0 as libc::c_int == VA_NONE as libc::c_int {
            (*effect).type_0 = VA_TUMOR as libc::c_int as u8_0;
            (*effect).parent = this;
            (*effect).pos = pos;
            (*effect).accel = sZeroVec;
            (*effect).velocity = (*effect).accel;
            (*effect).mode = mode as s16;
            (*effect).rot.z = 0 as libc::c_int as s16;
            (*effect).offset.x = (*offset).x;
            (*effect).offset.z = (*offset).z;
            (*effect).offset.y = (*offset).y;
            (*effect).timer =
                ((Rand_ZeroOne() * 10.0f32) as s16 as libc::c_int +
                     10 as libc::c_int) as u16_0;
            (*effect).envColor[3 as libc::c_int as usize] =
                100 as libc::c_int as s16;
            (*effect).scaleMod =
                scale as libc::c_int as libc::c_float * 0.01f32;
            (*effect).scale = 0.0f32;
            if i as libc::c_int % 4 as libc::c_int == 0 as libc::c_int ||
                   mode as libc::c_int == 2 as libc::c_int {
                Audio_PlaySoundGeneral(0x393f as libc::c_int as u16_0,
                                       &mut (*effect).pos,
                                       4 as libc::c_int as u8_0,
                                       &mut D_801333E0, &mut D_801333E0,
                                       &mut D_801333E8);
            }
            break ;
        } else { i += 1; effect = effect.offset(1) }
    };
}
#[no_mangle]
pub unsafe extern "C" fn BossVa_SpawnGore(mut globalCtx: *mut GlobalContext,
                                          mut effect: *mut BossVaEffect,
                                          mut pos: *mut Vec3f, mut yaw: s16,
                                          mut scale: s16) {
    let mut i: s32 = 0;
    let mut xzVel: f32_0 = 0.;
    let mut accel: Vec3f =
        { let mut init = Vec3f{x: 0.0f32, y: 0.0f32, z: 0.0f32,}; init };
    let mut velocity: Vec3f = Vec3f{x: 0., y: 0., z: 0.,};
    i = 0 as libc::c_int;
    while i <
              (::std::mem::size_of::<[BossVaEffect; 400]>() as
                   libc::c_ulong).wrapping_div(::std::mem::size_of::<BossVaEffect>()
                                                   as libc::c_ulong) as s32 {
        if (*effect).type_0 as libc::c_int == VA_NONE as libc::c_int {
            (*effect).type_0 = VA_GORE as libc::c_int as u8_0;
            (*effect).pos = *pos;
            (*effect).scaleMod = 0.0f32;
            xzVel = Rand_ZeroOne() * 4.0f32 + 4.0f32;
            velocity.x = Math_SinS(yaw) * xzVel;
            velocity.y = Rand_CenteredFloat(8.0f32);
            velocity.z = Math_CosS(yaw) * xzVel;
            (*effect).velocity = velocity;
            accel.y = Rand_CenteredFloat(0.3f32) - 1.0f32;
            (*effect).accel = accel;
            (*effect).timer = 20 as libc::c_int as u16_0;
            if sCsState as libc::c_int <= DEATH_SHELL_BURST as libc::c_int {
                (*effect).mode = GORE_FADING as libc::c_int as s16
            } else { (*effect).mode = GORE_PERMANENT as libc::c_int as s16 }
            (*effect).envColor[3 as libc::c_int as usize] =
                255 as libc::c_int as s16;
            (*effect).envColor[2 as libc::c_int as usize] =
                (*effect).envColor[3 as libc::c_int as usize];
            (*effect).envColor[1 as libc::c_int as usize] =
                (*effect).envColor[2 as libc::c_int as usize];
            (*effect).envColor[0 as libc::c_int as usize] =
                (*effect).envColor[1 as libc::c_int as usize];
            (*effect).primColor[3 as libc::c_int as usize] =
                (*effect).envColor[0 as libc::c_int as usize];
            (*effect).primColor[0 as libc::c_int as usize] =
                155 as libc::c_int as s16;
            (*effect).primColor[2 as libc::c_int as usize] =
                55 as libc::c_int as s16;
            (*effect).primColor[1 as libc::c_int as usize] =
                (*effect).primColor[2 as libc::c_int as usize];
            (*effect).rot.x =
                Rand_CenteredFloat(0x10000 as libc::c_int as f32_0) as s16;
            (*effect).rot.y =
                Rand_CenteredFloat(0x10000 as libc::c_int as f32_0) as s16;
            (*effect).scale =
                (Rand_ZeroFloat(scale as f32_0) +
                     scale as libc::c_int as libc::c_float) * 0.01f32;
            (*effect).offset.y = Rand_ZeroOne() * 0.25f32 + 0.9f32;
            break ;
        } else { i += 1; effect = effect.offset(1) }
    };
}
#[no_mangle]
pub unsafe extern "C" fn BossVa_SpawnZapperCharge(mut globalCtx:
                                                      *mut GlobalContext,
                                                  mut effect:
                                                      *mut BossVaEffect,
                                                  mut this: *mut BossVa,
                                                  mut pos: *mut Vec3f,
                                                  mut rot: *mut Vec3s,
                                                  mut scale: s16,
                                                  mut mode: u8_0) {
    let mut unused: Vec3f =
        { let mut init = Vec3f{x: 0.0f32, y: -1000.0f32, z: 0.0f32,}; init };
    let mut i: s16 = 0;
    i = 0 as libc::c_int as s16;
    while (i as libc::c_int) <
              (::std::mem::size_of::<[BossVaEffect; 400]>() as
                   libc::c_ulong).wrapping_div(::std::mem::size_of::<BossVaEffect>()
                                                   as libc::c_ulong) as s32 {
        if (*effect).type_0 as libc::c_int == VA_NONE as libc::c_int {
            (*effect).type_0 = VA_ZAP_CHARGE as libc::c_int as u8_0;
            (*effect).parent = this;
            (*effect).pos = *pos;
            (*effect).accel = sZeroVec;
            (*effect).velocity = (*effect).accel;
            (*effect).mode = mode as s16;
            (*effect).rot.x =
                ((*rot).x as libc::c_int + 0x4000 as libc::c_int) as s16;
            (*effect).rot.y = (*rot).y;
            (*effect).timer =
                ((Rand_ZeroOne() * 10.0f32) as s16 as libc::c_int +
                     10 as libc::c_int) as u16_0;
            (*effect).primColor[3 as libc::c_int as usize] =
                240 as libc::c_int as s16;
            (*effect).scale = scale as libc::c_int as libc::c_float * 0.01f32;
            break ;
        } else { i += 1; effect = effect.offset(1) }
    };
}
#[no_mangle]
pub unsafe extern "C" fn BossVa_DrawDoor(mut globalCtx: *mut GlobalContext,
                                         mut scale: s16) {
    static mut doorPieceDispList: [*mut Gfx; 8] =
        unsafe {
            [gBarinadeDoorPiece1DL.as_ptr() as *mut _,
             gBarinadeDoorPiece2DL.as_ptr() as *mut _,
             gBarinadeDoorPiece3DL.as_ptr() as *mut _,
             gBarinadeDoorPiece4DL.as_ptr() as *mut _,
             gBarinadeDoorPiece5DL.as_ptr() as *mut _,
             gBarinadeDoorPiece6DL.as_ptr() as *mut _,
             gBarinadeDoorPiece7DL.as_ptr() as *mut _,
             gBarinadeDoorPiece8DL.as_ptr() as *mut _]
        };
    static mut doorPieceLength: [s16; 8] =
        [836 as libc::c_int as s16, 900 as libc::c_int as s16,
         836 as libc::c_int as s16, 1016 as libc::c_int as s16,
         800 as libc::c_int as s16, 1016 as libc::c_int as s16,
         836 as libc::c_int as s16, 900 as libc::c_int as s16];
    let mut doorMtx: MtxF = MtxF{mf: [[0.; 4]; 4],};
    let mut yScale: f32_0 = 0.;
    let mut segAngle: f32_0 = 0.0f32;
    let mut i: s32 = 0;
    let mut __gfxCtx: *mut GraphicsContext = 0 as *mut GraphicsContext;
    let mut dispRefs: [*mut Gfx; 4] = [0 as *mut Gfx; 4];
    __gfxCtx = (*globalCtx).state.gfxCtx;
    Graph_OpenDisps(dispRefs.as_mut_ptr(), (*globalCtx).state.gfxCtx,
                    b"../z_boss_va.c\x00" as *const u8 as *const libc::c_char,
                    5600 as libc::c_int);
    Matrix_Translate(0.0f32, 80.0f32, 400.0f32,
                     MTXMODE_NEW as libc::c_int as u8_0);
    Matrix_RotateY(3.14159265358979323846f32,
                   MTXMODE_APPLY as libc::c_int as u8_0);
    yScale = scale as libc::c_int as libc::c_float * 0.01f32 * 0.1f32;
    Matrix_Scale(0.1f32, yScale, 0.1f32,
                 MTXMODE_APPLY as libc::c_int as u8_0);
    if yScale != 0.0f32 { yScale = 0.1f32 / yScale } else { yScale = 0.0f32 }
    Matrix_Get(&mut doorMtx);
    i = 0 as libc::c_int;
    while i < 8 as libc::c_int {
        Matrix_Put(&mut doorMtx);
        Matrix_RotateZ(segAngle, MTXMODE_APPLY as libc::c_int as u8_0);
        Matrix_Translate(0.0f32,
                         doorPieceLength[i as usize] as libc::c_int as
                             libc::c_float * yScale, 0.0f32,
                         MTXMODE_APPLY as libc::c_int as u8_0);
        let fresh72 = (*__gfxCtx).polyOpa.p;
        (*__gfxCtx).polyOpa.p = (*__gfxCtx).polyOpa.p.offset(1);
        let mut _g: *mut Gfx = fresh72;
        (*_g).words.w0 =
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
        (*_g).words.w1 =
            Matrix_NewMtx((*globalCtx).state.gfxCtx,
                          b"../z_boss_va.c\x00" as *const u8 as
                              *const libc::c_char as *mut libc::c_char,
                          5621 as libc::c_int) as libc::c_uint;
        let fresh73 = (*__gfxCtx).polyOpa.p;
        (*__gfxCtx).polyOpa.p = (*__gfxCtx).polyOpa.p.offset(1);
        let mut _g_0: *mut Gfx = fresh73;
        (*_g_0).words.w0 =
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
        (*_g_0).words.w1 = doorPieceDispList[i as usize] as libc::c_uint;
        i += 1;
        segAngle -=
            3.14159265358979323846f32 / 4 as libc::c_int as libc::c_float
    }
    Graph_CloseDisps(dispRefs.as_mut_ptr(), (*globalCtx).state.gfxCtx,
                     b"../z_boss_va.c\x00" as *const u8 as
                         *const libc::c_char, 5629 as libc::c_int);
}
unsafe extern "C" fn run_static_initializers() {
    sJntSphInitSupport =
        {
            let mut init =
                ColliderJntSphInit{base:
                                       {
                                           let mut init =
                                               ColliderInit{colType:
                                                                COLTYPE_HIT6
                                                                    as
                                                                    libc::c_int
                                                                    as u8_0,
                                                            atFlags:
                                                                0 as
                                                                    libc::c_int
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
                                                                0 as
                                                                    libc::c_int
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
                                   count:
                                       (::std::mem::size_of::<[ColliderJntSphElementInit; 1]>()
                                            as
                                            libc::c_ulong).wrapping_div(::std::mem::size_of::<ColliderJntSphElementInit>()
                                                                            as
                                                                            libc::c_ulong)
                                           as s32,
                                   elements:
                                       sJntSphElementsInitSupport.as_mut_ptr(),};
            init
        };
    sJntSphInitBari =
        {
            let mut init =
                ColliderJntSphInit{base:
                                       {
                                           let mut init =
                                               ColliderInit{colType:
                                                                COLTYPE_NONE
                                                                    as
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
                                                                0 as
                                                                    libc::c_int
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
                                   count:
                                       (::std::mem::size_of::<[ColliderJntSphElementInit; 1]>()
                                            as
                                            libc::c_ulong).wrapping_div(::std::mem::size_of::<ColliderJntSphElementInit>()
                                                                            as
                                                                            libc::c_ulong)
                                           as s32,
                                   elements:
                                       sJntSphElementsInitBari.as_mut_ptr(),};
            init
        }
}
#[used]
#[cfg_attr(target_os = "linux", link_section = ".init_array")]
#[cfg_attr(target_os = "windows", link_section = ".CRT$XIB")]
#[cfg_attr(target_os = "macos", link_section = "__DATA,__mod_init_func")]
static INIT_ARRAY: [unsafe extern "C" fn(); 1] = [run_static_initializers];
